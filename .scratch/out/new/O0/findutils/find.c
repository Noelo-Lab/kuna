// Function: _DT_INIT @ 0x6000
void _DT_INIT(void) // return-dupe
{
  if (!dat_48fa8)
    return;
  (*dat_48fa8)();
}

// Function: sub_6020 @ 0x6020
void sub_6020(void)
{
  (*dat_48b00)(); // jump-as-call
}

// Function: free @ 0x6970
void free(void *a0)
{
  (*dat_48fc0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x6980
void __cxa_finalize(void)
{
  (*dat_48fd0)(); // jump-as-call
}

// Function: ctime @ 0x6990
char * ctime(void *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48b08)(); // jump-as-call
  return v1;
}

// Function: setmntent @ 0x69a0
void * setmntent(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48b10)(); // jump-as-call
  return v1;
}

// Function: chdir @ 0x69b0
int chdir(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_48b18)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x69c0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48b20)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x69d0
void strtoumax(void)
{
  (*dat_48b28)(); // jump-as-call
}

// Function: endmntent @ 0x69e0
int endmntent(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_48b30)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x69f0
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_48b38)(); // jump-as-call
  return v1;
}

// Function: mktime @ 0x6a00
long mktime(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_48b40)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x6a10
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48b48)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x6a20
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48b50)(); // jump-as-call
  return v1;
}

// Function: getgrnam @ 0x6a30
group * getgrnam(char *a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_48b58)(); // jump-as-call
  return v1;
}

// Function: snprintf @ 0x6a40
int snprintf(char *a0,unsigned long a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_48b60)(); // jump-as-call
  return v1;
}

// Function: close @ 0x6a50
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48b68)(); // jump-as-call
  return v1;
}

// Function: hasmntopt @ 0x6a60
char * hasmntopt(void *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48b70)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x6a70
void abort(void)
{
  (*dat_48b78)(); // jump-as-call
}

// Function: memchr @ 0x6a80
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48b80)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x6a90
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_48b88)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x6aa0
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_48b90)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x6ab0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48b98)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x6ac0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48ba0)(); // jump-as-call
  return v1;
}

// Function: uname @ 0x6ad0
int uname(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_48ba8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x6ae0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48bb0)(); // jump-as-call
  return v1;
}

// Function: __isoc99_sscanf @ 0x6af0
int __isoc99_sscanf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_48bb8)(); // jump-as-call
  return v1;
}

// Function: toupper @ 0x6b00
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48bc0)(); // jump-as-call
  return v1;
}

// Function: execvp @ 0x6b10
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_48bc8)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x6b20
void exit(int a0)
{
  (*dat_48bd0)(); // jump-as-call
}

// Function: __assert_fail @ 0x6b30
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_48bd8)(); // jump-as-call
}

// Function: fstat @ 0x6b40
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_48be0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x6b50
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48be8)(); // jump-as-call
  return v1;
}

// Function: gettimeofday @ 0x6b60
int gettimeofday(timeval *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_48bf0)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x6b70
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_48bf8)(); // jump-as-call
  return v1;
}

// Function: strpbrk @ 0x6b80
char * strpbrk(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48c00)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x6b90
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48c08)(); // jump-as-call
  return v1;
}

// Function: getmntent @ 0x6ba0
void * getmntent(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48c10)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x6bb0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_48c18)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x6bc0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48c20)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x6bd0
void freecon(char *a0)
{
  (*dat_48c28)(); // jump-as-call
}

// Function: fopen @ 0x6be0
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_48c30)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_minor @ 0x6bf0
void gnu_dev_minor(void)
{
  (*dat_48c38)(); // jump-as-call
}

// Function: fchdir @ 0x6c00
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48c40)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x6c10
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48c48)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x6c20
void _exit(int a0)
{
  (*dat_48c50)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x6c30
void __cxa_atexit(void)
{
  (*dat_48c58)(); // jump-as-call
}

// Function: sysconf @ 0x6c40
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_48c60)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x6c50
int fgetfilecon(int a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_48c68)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x6c60
void tzset(void)
{
  (*dat_48c70)(); // jump-as-call
}

// Function: getpwuid @ 0x6c70
passwd * getpwuid(unsigned int a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_48c78)(); // jump-as-call
  return v1;
}

// Function: lsetfilecon @ 0x6c80
int lsetfilecon(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_48c80)(); // jump-as-call
  return v1;
}

// Function: re_compile_pattern @ 0x6c90
char * re_compile_pattern(char *a0,unsigned long a1,re_pattern_buffer *a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48c88)(); // jump-as-call
  return v1;
}

// Function: vfprintf @ 0x6ca0
int vfprintf(FILE *a0,char *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_48c90)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x6cb0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48c98)(); // jump-as-call
  return v1;
}

// Function: is_selinux_enabled @ 0x6cc0
int is_selinux_enabled(void)
{
  int v1; // eax
  
  v1 = (*dat_48ca0)(); // jump-as-call
  return v1;
}

// Function: fputc @ 0x6cd0
int fputc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_48ca8)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x6ce0
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_48cb0)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x6cf0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48cb8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x6d00
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48cc0)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x6d10
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_48cc8)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x6d20
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48cd0)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x6d30
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48cd8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x6d40
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48ce0)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x6d50
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_48ce8)(); // jump-as-call
  return v1;
}

// Function: bsearch @ 0x6d60
void bsearch(void)
{
  (*dat_48cf0)(); // jump-as-call
}

// Function: sprintf @ 0x6d70
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_48cf8)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x6d80
FILE * fdopen(int a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_48d00)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x6d90
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48d08)(); // jump-as-call
  return v1;
}

// Function: strtok_r @ 0x6da0
char * strtok_r(char *a0,char *a1,char **a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48d10)(); // jump-as-call
  return v1;
}

// Function: timegm @ 0x6db0
long timegm(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_48d18)(); // jump-as-call
  return v1;
}

// Function: poll @ 0x6dc0
void poll(void)
{
  (*dat_48d20)(); // jump-as-call
}

// Function: gmtime_r @ 0x6dd0
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_48d28)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x6de0
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48d30)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x6df0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48d38)(); // jump-as-call
  return v1;
}

// Function: abs @ 0x6e00
int abs(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48d40)(); // jump-as-call
  return v1;
}

// Function: strcat @ 0x6e10
char * strcat(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48d48)(); // jump-as-call
  return v1;
}

// Function: memrchr @ 0x6e20
void * memrchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48d50)(); // jump-as-call
  return v1;
}

// Function: fputs @ 0x6e30
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_48d58)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x6e40
void lseek(void)
{
  (*dat_48d60)(); // jump-as-call
}

// Function: strtol @ 0x6e50
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_48d68)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x6e60
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_48d70)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x6e70
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_48d78)(); // jump-as-call
  return v1;
}

// Function: endpwent @ 0x6e80
void endpwent(void)
{
  (*dat_48d80)(); // jump-as-call
}

// Function: putc @ 0x6e90
int putc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_48d88)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x6ea0
void re_set_syntax(void)
{
  (*dat_48d90)(); // jump-as-call
}

// Function: signal @ 0x6eb0
void signal(void)
{
  (*dat_48d98)(); // jump-as-call
}

// Function: strspn @ 0x6ec0
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48da0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x6ed0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48da8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x6ee0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48db0)(); // jump-as-call
  return v1;
}

// Function: waitpid @ 0x6ef0
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_48db8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x6f00
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48dc0)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x6f10
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_48dc8)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x6f20
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48dd0)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x6f30
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_48dd8)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x6f40
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_48de0)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x6f50
void qsort(void)
{
  (*dat_48de8)(); // jump-as-call
}

// Function: strdup @ 0x6f60
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48df0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x6f70
void __stack_chk_fail(void)
{
  (*dat_48df8)(); // jump-as-call
}

// Function: strcmp @ 0x6f80
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_48e00)(); // jump-as-call
  return v1;
}

// Function: tolower @ 0x6f90
int tolower(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48e08)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x6fa0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_48e10)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x6fb0
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48e18)(); // jump-as-call
  return v1;
}

// Function: getgrgid @ 0x6fc0
group * getgrgid(unsigned int a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_48e20)(); // jump-as-call
  return v1;
}

// Function: getline @ 0x6fd0
long getline(char **a0,void *a1,FILE *a2)
{
  long v1; // rax
  
  v1 = (*dat_48e28)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x6fe0
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_48e30)(); // jump-as-call
  return v1;
}

// Function: strcpy @ 0x6ff0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48e38)(); // jump-as-call
  return v1;
}

// Function: setfilecon @ 0x7000
int setfilecon(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_48e40)(); // jump-as-call
  return v1;
}

// Function: endgrent @ 0x7010
void endgrent(void)
{
  (*dat_48e48)(); // jump-as-call
}

// Function: rpmatch @ 0x7020
int rpmatch(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_48e50)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x7030
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_48e58)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x7040
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48e60)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x7050
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48e68)(); // jump-as-call
  return v1;
}

// Function: faccessat @ 0x7060
int faccessat(int a0,char *a1,int a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_48e70)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x7070
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48e78)(); // jump-as-call
  return v1;
}

// Function: strncpy @ 0x7080
char * strncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48e80)(); // jump-as-call
  return v1;
}

// Function: getfilecon @ 0x7090
int getfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_48e88)(); // jump-as-call
  return v1;
}

// Function: getrlimit @ 0x70a0
void getrlimit(void)
{
  (*dat_48e90)(); // jump-as-call
}

// Function: difftime @ 0x70b0
void difftime(void)
{
  (*dat_48e98)(); // jump-as-call
}

// Function: localtime_r @ 0x70c0
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_48ea0)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_major @ 0x70d0
void gnu_dev_major(void)
{
  (*dat_48ea8)(); // jump-as-call
}

// Function: fseeko @ 0x70e0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_48eb0)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_makedev @ 0x70f0
void gnu_dev_makedev(void)
{
  (*dat_48eb8)(); // jump-as-call
}

// Function: unsetenv @ 0x7100
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_48ec0)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x7110
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_48ec8)(); // jump-as-call
  return v1;
}

// Function: access @ 0x7120
int access(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_48ed0)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x7130
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_48ed8)(); // jump-as-call
  return v1;
}

// Function: modf @ 0x7140
void modf(void)
{
  (*dat_48ee0)(); // jump-as-call
}

// Function: unlinkat @ 0x7150
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_48ee8)(); // jump-as-call
  return v1;
}

// Function: fwrite @ 0x7160
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48ef0)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x7170
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48ef8)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x7180
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_48f00)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x7190
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48f08)(); // jump-as-call
  return v1;
}

// Function: error @ 0x71a0
void error(int a0,int a1,char *a2,...)
{
  (*dat_48f10)(); // jump-as-call
}

// Function: fprintf @ 0x71b0
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_48f18)(); // jump-as-call
  return v1;
}

// Function: localtime @ 0x71c0
tm * localtime(void *a0)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_48f20)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x71d0
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_48f28)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x71e0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48f30)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x71f0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_48f38)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x7200
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_48f40)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x7210
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_48f48)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x7220
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_48f50)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x7230
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_48f58)(); // jump-as-call
  return v1;
}

// Function: open @ 0x7240
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_48f60)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x7250
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_48f68)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x7260
passwd * getpwnam(char *a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_48f70)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x7270
int lgetfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_48f78)(); // jump-as-call
  return v1;
}

// Function: re_match @ 0x7280
int re_match(re_pattern_buffer *a0,char *a1,int a2,int a3,void *a4)
{
  int v1; // eax
  
  v1 = (*dat_48f80)(); // jump-as-call
  return v1;
}

// Function: strndup @ 0x7290
char * strndup(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_48f88)(); // jump-as-call
  return v1;
}

// Function: time @ 0x72a0
void time(void)
{
  (*dat_48f90)(); // jump-as-call
}

// Function: fflush @ 0x72b0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_48f98)(); // jump-as-call
  return v1;
}

// Function: fdopendir @ 0x72c0
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_48fa0)(); // jump-as-call
  return v1;
}

// Function: sub_72d0 @ 0x72d0
void sub_72d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_48fe0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_7300 @ 0x7300
void sub_7300(void)
{
  return;
}

// Function: _FINI_0 @ 0x7370
void _FINI_0(void)
{
  if (!dat_49910) {
    if (dat_48fd0)
      __cxa_finalize(dat_49008);
    sub_7300();
    dat_49910 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x73b0
void _INIT_0(void)
{
  return;
}

// Function: sub_73b9 @ 0x73b9
void sub_73b9(void)
{
  if ((dat_49010 & 0x200) && (0 <= dat_49018)) {
    close(dat_49018);
    dat_49018 = -1;
  }
}

// Function: sub_73f4 @ 0x73f4
void sub_73f4(int a0)
{
  if (dat_49010 & 0x200) {
    if ((a0 != -100) && (a0 <= -1))
      __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir"); // no-return
    dat_499d8 = a0;
    if (dat_49018 <= -1) {
      if (a0 != -100) { // branch-flip
        if (0 <= a0) // branch-flip
          dat_49018 = sub_1d639(a0);
        else if ((dat_49018 <= -1) && (a0 <= -1))
          __assert_fail("curr_fd >= 0 || dir_fd >= 0","ftsfind.c",0x7d,"inside_dir"); // no-return
      }
      else {
        dat_49018 = -100;
      }
    }
  }
}

// Function: sub_74c2 @ 0x74c2
char * sub_74c2(int a0) // return-dupe x14
{
  switch(a0) {
    default:
      sprintf((char *)0x49918,"[%d]",a0);
      return (char *)0x49918;
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

// Function: sub_75b5 @ 0x75b5
void sub_75b5(struct_3 *a0,void *a1,void *a2) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_75b5
{
  struct_2 *v1; // rax
  
  dat_499c4 = ((*(short *)((long)a1 + 0x68) != 10) && (*(short *)((long)a1 + 0x68) != 0xb));
  dat_499d0 = *(unsigned long *)((long)a1 + 0x30);
  dat_499d8 = a0->field_0x2c;
  v1 = (struct_2 *)sub_d7ff();
  sub_10873(*(char **)((long)a1 + 0x38),a2,v1);
  if (!dat_499e0)
    return;
  sub_1fc0f(a0,a1,4);
}

// Function: sub_7664 @ 0x7664
char * sub_7664(unsigned int a0,char *a1,long a2,unsigned int a3)
{
  char v1;
  char *v2; // rax
  
  if (a2) { // branch-flip
    v2 = &a1[a2];
    v1 = *v2;
    v2 = &a1[a2];
    *v2 = '\0';
    v2 = (unsigned long)sub_2f861(a0,a3,a1);
    a1[a2] = v1;
  }
  else {
    v2 = (unsigned long)sub_2f861(a0,a3,"");
  }
  return v2;
}

// Function: sub_76ea @ 0x76ea
void sub_76ea(struct_4 *a0) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  
  if ((a0->field_0x88 & 0xf000) == 0xa000) {
    v1 = (char *)sub_1096a(0,a0->field_0x38);
    error(0,0,gettext("Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points."),v1);
    return;
  }
  v1 = (char *)sub_7664(1,*(char **)(a0->field_0x0 + 0x38),*(long *)(a0->field_0x0 + 0x48),dat_499ac);
  v2 = (char *)sub_1096a(0,a0->field_0x38);
  error(0,0,gettext("File system loop detected; %s is part of the same file system loop as %s."),v2,v1);
}

// Function: sub_77d0 @ 0x77d0
unsigned int sub_77d0(unsigned long a0) // return-dupe
{
  int v1; // eax
  char v2 [152];
  
  v1 = (*dat_49990)(a0,v2);
  if ((v1) && (*__errno_location() == 0x28))
    return 1;
  return 0;
}

// Function: sub_7852 @ 0x7852
void sub_7852(struct_3 *a0,void *a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_7852
{
  unsigned long v1;
  unsigned int v10; // stack - 0xe4
  int v11; // stack - 0xe0
  int v12; // stack - 0xdc
  unsigned long v13; // stack - 0xd0
  unsigned long v14; // stack - 0xc8
  unsigned long v15; // stack - 0xc0
  unsigned long v16; // stack - 0xb8
  unsigned long v17; // stack - 0xb0
  unsigned long v18; // stack - 0xa8
  unsigned long v19; // stack - 0xa0
  int v2;
  unsigned long v20; // stack - 0x98
  unsigned long v21; // stack - 0x90
  unsigned long v22; // stack - 0x88
  unsigned long v23; // stack - 0x80
  unsigned long v24; // stack - 0x78
  unsigned long v25; // stack - 0x70
  unsigned long v26; // stack - 0x68
  unsigned long v27; // stack - 0x60
  unsigned long v28; // stack - 0x58
  unsigned long v29; // stack - 0x50
  unsigned char v3;
  unsigned char v4;
  char *v5;
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  unsigned long v9; // stack - 0xd8
  
  if (dat_49980 & 4) {
    v5 = (char *)sub_2f861(2,dat_499ac,*(char **)((long)a1 + 0x30));
    v6 = (char *)sub_2f861(1,dat_499ac,*(char **)((long)a1 + 0x38));
    v2 = dat_49014;
    v1 = *(unsigned long *)((long)a1 + 0x58);
    v7 = (char *)sub_74c2(*(unsigned short *)((long)a1 + 0x68));
    v8 = (char *)sub_2f861(0,dat_499ac,*(char **)((long)a1 + 0x38));
    fprintf(stderr,"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",v8,v7,(int)v1,v2,v6,v5);
  }
  if (*(short *)((long)a1 + 0x68) != 6) { // branch-flip
    if (((long)dat_49014 < *(long *)((long)a1 + 0x58)) || (!*(long *)((long)a1 + 0x58)))
      sub_73b9();
  }
  else {
    sub_73b9();
  }
  sub_73f4(a0->field_0x2c);
  dat_49014 = (unsigned int)*(unsigned long *)((long)a1 + 0x58);
  v13 = *(unsigned long *)((long)a1 + 0x78);
  if (*(short *)((long)a1 + 0x68) == 7) {
    sub_10a13(*(unsigned int *)((long)a1 + 0x40),*(char **)((long)a1 + 0x38));
    return;
  }
  if (*(short *)((long)a1 + 0x68) != 4) { // branch-flip
    if (*(short *)((long)a1 + 0x68) == 2) {
      sub_76ea(a1);
      dat_499e4 = 1;
      return;
    }
    if (*(short *)((long)a1 + 0x68) != 0xd) { // branch-flip
      if (*(short *)((long)a1 + 0x68) == 10) {
        if (!*(long *)((long)a1 + 0x58)) {
          sub_10a13(*(unsigned int *)((long)a1 + 0x40),*(char **)((long)a1 + 0x38));
          return;
        }
        v1 = *(unsigned long *)((long)a1 + 0x30);
        if (sub_77d0(v1)) {
          sub_10a13(0x28,*(char **)((long)a1 + 0x38));
          return;
        }
        sub_10a13(*(unsigned int *)((long)a1 + 0x40),*(char **)((long)a1 + 0x38));
      }
    }
    else {
      v1 = *(unsigned long *)((long)a1 + 0x30);
      if (sub_77d0(v1)) {
        sub_10a13(0x28,*(char **)((long)a1 + 0x38));
        return;
      }
    }
  }
  else {
    sub_10a13(*(unsigned int *)((long)a1 + 0x40),*(char **)((long)a1 + 0x38));
    if (dat_49940)
      return;
  }
  if ((*(short *)((long)a1 + 0x68) != 0xb) && (*(short *)((long)a1 + 0x68) != 10)) {
    dat_499c4 = 1;
    dat_499c5 = 1;
    v9 = *(unsigned long *)((long)a1 + 0x70);
    v13 = *(unsigned long *)((long)a1 + 0x78);
    v14 = *(unsigned long *)((long)a1 + 0x80);
    v15 = *(unsigned long *)((long)a1 + 0x88);
    v16 = *(unsigned long *)((long)a1 + 0x90);
    v17 = *(unsigned long *)((long)a1 + 0x98);
    v18 = *(unsigned long *)((long)a1 + 0xa0);
    v19 = *(unsigned long *)((long)a1 + 0xa8);
    v20 = *(unsigned long *)((long)a1 + 0xb0);
    v21 = *(unsigned long *)((long)a1 + 0xb8);
    v22 = *(unsigned long *)((long)a1 + 0xc0);
    v23 = *(unsigned long *)((long)a1 + 200);
    v24 = *(unsigned long *)((long)a1 + 0xd0);
    v25 = *(unsigned long *)((long)a1 + 0xd8);
    v26 = *(unsigned long *)((long)a1 + 0xe0);
    v27 = *(unsigned long *)((long)a1 + 0xe8);
    v29 = *(unsigned long *)((long)a1 + 0xf8);
    v28 = *(unsigned long *)((long)a1 + 0xf0);
    v10 = (unsigned int)v15;
    dat_499c8 = (unsigned int)v15;
    if (!(unsigned int)v15) {
      v5 = (char *)sub_2f861(0,dat_499ac,*(char **)((long)a1 + 0x38));
      error(0,0,gettext("WARNING: file %s appears to have mode 0000"),v5);
    }
  }
  else {
    if (dat_499c4 == 1)
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting"); // no-return
    if ((*(short *)((long)a1 + 0x68) != 0xb) && (dat_499c8))
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,"consider_visiting"); // no-return
    v10 = dat_499c8;
  }
  dat_499c0 = (unsigned int)*(unsigned long *)((long)a1 + 0x58);
  if ((v10) && (v5 = *(char **)((long)a1 + 0x38), sub_fe38(&v10,v5,(char *)((long)a1 + 0x100),&v9,0) != '\x01'))
    return;
  v11 = 0;
  v12 = !(((v10 & 0xf000) != 0x4000) && (((*(short *)((long)a1 + 0x68) != 1 && (*(short *)((long)a1 + 0x68) != 6)) && (*(short *)((long)a1 + 0x68) != 2))));
  if ((v12) && (*(short *)((long)a1 + 0x68) == 0xb)) {
    sub_1fc0f(a0,a1,1);
    return;
  }
  if ((0 <= dat_49944) && (((long)dat_49944 <= *(long *)((long)a1 + 0x58) && (sub_1fc0f(a0,a1,4), (long)dat_49944 < *(long *)((long)a1 + 0x58)))))
    v11 = 1;
  v4 = dat_499c5;
  v3 = dat_499c4;
  if ((*(short *)((long)a1 + 0x68) != 1) || (dat_49940 == '\x01')) {
    if ((*(short *)((long)a1 + 0x68) != 6) || (!dat_49940)) {
      if (*(long *)((long)a1 + 0x58) < (long)dat_49948)
        v11 = 1;
    }
    else {
      v11 = 1;
    }
  }
  else {
    v11 = 1;
  }
  if (dat_49980 & 4) {
    v5 = (char *)sub_74c2(*(unsigned short *)((long)a1 + 0x68));
    v6 = (char *)sub_2f861(0,dat_499ac,*(char **)((long)a1 + 0x38));
    fprintf(stderr,"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",v6,v5,v12,v11,(unsigned int)v3,(unsigned int)v4);
  }
  if (!v11)
    sub_75b5(a0,a1,&v9);
  if (*(short *)((long)a1 + 0x68) != 6)
    return;
  dat_499e0 = 0;
}

// Function: sub_803a @ 0x803a
unsigned long sub_803a(char *a0) // return-dupe
{
  struct_28 *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  void *v4; // rax
  char *v5; // stack - 0x38
  int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x30
  
  dat_499dc = (unsigned int)strlen(a0);
  sub_73f4(0xffffff9c);
  v7 = 0;
  if (dat_49988 != 2) { // branch-flip
    if (dat_49988 <= 2) {
      if (dat_49988) { // branch-flip
        if (dat_49988 == 1)
          dat_49010 |= 3;
      }
      else {
        dat_49010 |= 0x10;
      }
    }
  }
  else {
    dat_49010 |= 0x11;
  }
  if (dat_4994d)
    dat_49010 |= 0x40;
  v5 = a0;
  v1 = (struct_28 *)sub_1ec55(&v5,dat_49010,0);
  if (!v1) {
    v2 = sub_1096a(0,a0);
    v3 = gettext("cannot search %s");
    error(0,*__errno_location(),v3,v2);
    dat_499e4 = 1;
    return 1;
  }
  v6 = -0x80000000;
  while( true ) {
    *__errno_location() = 0;
    v4 = (void *)sub_1f599(v1);
    if (!v4) break;
    if ((dat_499e8) && (v6 != (int)*(unsigned long *)((long)v4 + 0x58)))
      sub_f6cb();
    v6 = (int)*(unsigned long *)((long)v4 + 0x58);
    dat_499f8 = 0;
    dat_499c4 = 0;
    dat_499c5 = *(int *)((long)v4 + 0x88) != 0;
    if ((bool)dat_499c5)
      dat_499c8 = *(unsigned int *)((long)v4 + 0x88);
    else {
      dat_499c8 = 0;
    }
    sub_7852(v1,v4);
  }
  if (!*__errno_location()) {
    if (!sub_1f0df(v1))
      return 1;
    v3 = gettext("failed to restore working directory after searching %s");
    error(0,*__errno_location(),v3,a0);
    dat_499e4 = 1;
    return 0;
  }
  v3 = (char *)sub_1096a(0,a0);
  error(0,*__errno_location(),"failed to read file names from file system at or below %s",v3);
  dat_499e4 = 1;
  return 0;
}

// Function: sub_82e7 @ 0x82e7
unsigned long sub_82e7(int a0,unsigned long *a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_82e7
{
  char *v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  stat v12; // stack - 0x148
  stat v13; // stack - 0xb8
  unsigned short v14; // stack - 0x22
  unsigned int v15; // stack - 0x178
  bool v16; // stack - 0x17a
  int v17; // stack - 0x174
  FILE *v18; // stack - 0x170
  char *v19; // stack - 0x168
  char *v2;
  struct_45 *v20; // stack - 0x160
  char *v21; // stack - 0x158
  unsigned long v22; // stack - 0x150
  bool v3; // al
  char v4; // al
  int v5; // eax
  char *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v3 = ((1 <= a0) && (v1 = (char *)*a1, sub_ff96(v1,1) != '\x01'));
  v18 = NULL;
  v19 = NULL;
  if (dat_499b0) { // branch-flip
    if (v3) {
      v6 = (char *)sub_1096a(0,(char *)*a1);
      error(0,0,gettext("extra operand %s"),v6);
      error(1,0,"%s",gettext("file operands cannot be combined with -files0-from"));
      return v7;
    }
    if (strcmp(dat_499b0,"-")) { // branch-flip
      v19 = (char *)sub_1096a(0,dat_499b0);
      v18 = fopen(dat_499b0,"r");
      if (!v18) {
        v6 = gettext("cannot open %s for reading");
        error(1,*__errno_location(),v6,v19);
        return v9;
      }
      v17 = fileno(v18);
      if (v17 <= -1)
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints"); // no-return
      if ((((dat_499b8) && (!fstat(v17,&v12))) && (!fstat(0,&v13))) && ((v12._8_8_ == v13._8_8_ && (v12._0_8_ == v13._0_8_)))) {
        error(1,0,"%s: %s\n",gettext("option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir"),v19);
        return v10;
      }
      sub_1d5ba(v17,1);
    }
    else {
      if (dat_499b8) {
        error(1,0,"%s\n",gettext("option -files0-from reading from standard input cannot be combined with -ok, -okdir"));
        return v8;
      }
      v6 = gettext("(standard input)");
      v19 = (char *)sub_1096a(0,v6);
      v18 = stdin;
    }
    v20 = (struct_45 *)sub_1d0ff(v18);
  }
  else {
    if (!v3) {
      v14 = 0x2e;
      return sub_803a(&v14);
    }
    v20 = (struct_45 *)sub_1d0aa(a1);
  }
  if (!v20)
    sub_321fa(); // no-return
  v16 = 1;
  do {
    while( true ) {
      v6 = (char *)sub_1d16c(v20,&v15);
      v21 = v6;
      if (!v6) {
        switch(v15) {
          default:
            __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,"process_all_startpoints"); // no-return
          case 2:
            goto label_8852;
          case 3:
            sub_321fa(); // no-return
          case 4:
            v6 = gettext("%s: read error");
            error(0,*__errno_location(),v6,v19);
            dat_499e4 = 1;
            v16 = 0;
          
        }
        goto label_8852;
      }
      if (*v6) break;
      if (dat_499b0) { // branch-flip
        v22 = sub_1d257(v20);
        error(0,0,"%s:%lu: %s",v19,v22,gettext("invalid zero-length file name"));
      }
      else {
        error(0,2,"%s",(char *)sub_1096a(0,v6));
      }
      dat_499e4 = 1;
      v16 = 0;
    }
    v2 = v6;
    if ((!dat_499b0) && (v2 = v21, sub_ff96(v6,1))) goto label_8852;
    v21 = v2;
    dat_499dc = (unsigned int)strlen(v21);
    v4 = sub_803a(v21);
  } while (v4 == '\x01');
  v16 = 0;
label_8852:
  sub_1d295(v20);
  if (((v16) && (dat_499b0)) && ((ferror(v18) || (v5 = sub_1dad5(v18), v5)))) {
    error(1,0,gettext("error reading %s"),v19);
    return v11;
  }
  return (unsigned long)v16;
}

// Function: main @ 0x88ed
int main(int argc,char **argv,char **envp)
{
  int v1; // eax
  int v2; // eax
  int v3; // eax
  char *v4; // rax
  void *v5; // rax
  
  if (*argv) // branch-flip
    sub_2dd35(*argv);
  else {
    sub_2dd35("find");
  }
  sub_f78b();
  dat_499f8 = 0;
  dat_499e4 = 0;
  dat_499e8 = 0;
  dat_499d8 = 0xffffff9c;
  if (sub_1b5f4())
    sub_1b20a();
  dat_499f0 = sub_10be9("w");
  if (!dat_499f0) {
    v4 = gettext("Failed to initialize shared-file hash table");
    error(1,*__errno_location(),v4);
    return v1;
  }
  sub_106bd((struct_64 *)0x49940);
  setlocale(6,"");
  bindtextdomain("findutils","/usr/local/share/locale");
  textdomain("findutils");
  if (sub_37150(sub_1d697)) {
    v4 = gettext("The atexit library function failed");
    error(1,*__errno_location(),v4);
    return v2;
  }
  v3 = sub_103e8(argc,argv);
  if (dat_49980 & 2)
    dat_49990 = sub_fd39;
  if (dat_49980 & 0x80)
    fprintf(stderr,"cur_day_start = %s",ctime((void *)0x49968));
  v5 = (void *)sub_dd20(argc,argv,v3);
  if (sub_82e7(argc - v3,&argv[v3])) {
    sub_ae53(v5);
    sub_fa38();
  }
  return dat_499e4;
}

// Function: sub_8b12 @ 0x8b12
unsigned long sub_8b12(unsigned int *a0)
{
  *a0 = dat_49010;
  return 1;
}

// Function: sub_8b31 @ 0x8b31
double sub_8b31(unsigned long a0,long a1,unsigned long a2,long a3)
{
  double v1; // xmm0_qa
  
  difftime(a0,a2);
  return v1 + dat_389e8 * (double)(a1 - a3);
}

// Function: sub_8ba9 @ 0x8ba9
unsigned long sub_8ba9(long a0,long a1,long a2,long a3) // return-dupe x2
{
  double v1; // xmm0_qa
  
  if ((a0 == a2) && (a1 == a3))
    return 0;
  sub_8b31(a0,a1,a2,a3);
  if (v1 < 0.0)
    return 0xffffffff;
  return 1;
}

// Function: sub_8c32 @ 0x8c32
unsigned long sub_8c32(long a0,long a1,void *a2,int a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_8c32
{
  unsigned long v1;
  unsigned int v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  v2 = *(unsigned int *)((long)a2 + 0x3c);
  if (v2 != 2) { // branch-flip
    if (2 < v2)
      __assert_fail("0","pred.c",0x8a,"pred_timewindow"); // no-return, return-dupe
    if (v2) { // branch-flip
      if (v2 != 1) {
        __assert_fail("0","pred.c",0x8a,"pred_timewindow");
      }
      v2 = sub_8ba9(a0,a1,*(long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
      v2 >>= 0x1f;
      v3 = (unsigned long)v2;
    }
    else {
      v3 = sub_8ba9(a0,a1,*(long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
      v1 = v3 >> 8;
      v3 = CONCAT71((undefined7)v1,0 < (int)v2);
    }
  }
  else {
    sub_8b31(a0,a1,*(unsigned long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
    v2 = !((v4 <= 0.0) || ((double)a3 < v4));
    v3 = (unsigned long)v2;
  }
  return v3;
}

// Function: sub_8d51 @ 0x8d51
void sub_8d51(unsigned long a0,void *a1,void *a2)
{
  char v1 [16];
  
  v1 = sub_30a4c(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x3c);
}

// Function: sub_8d9c @ 0x8d9c
unsigned long sub_8d9c(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  if ((*(long *)((long)a2 + 0x110)) && (v1 = *(struct_2 **)((long)a2 + 0x110), !sub_10873(a0,a1,v1)))
    return 0;
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_10873(a0,a1,v1);
}

// Function: sub_8e0d @ 0x8e0d
unsigned long sub_8e0d(unsigned long a0,void *a1,void *a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer"); // no-return
  v2 = sub_30a4c(a1);
  v1 = sub_8ba9(SUB168(v2,0),SUB168(v2,8),*(long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}

// Function: sub_8e96 @ 0x8e96
void sub_8e96(unsigned long a0,void *a1,void *a2)
{
  char v1 [16];
  
  v1 = sub_30a4c(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x15180);
}

// Function: sub_8ee1 @ 0x8ee1
unsigned long sub_8ee1(void)
{
  return 1;
}

// Function: sub_8efc @ 0x8efc
void sub_8efc(unsigned long a0,void *a1,void *a2)
{
  char v1 [16];
  
  v1 = sub_30a66(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x3c);
}

// Function: sub_8f47 @ 0x8f47
unsigned long sub_8f47(unsigned long a0,void *a1,void *a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer"); // no-return
  v2 = sub_30a66(a1);
  v1 = sub_8ba9(SUB168(v2,0),SUB168(v2,8),*(long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}

// Function: sub_8fd0 @ 0x8fd0
void sub_8fd0(char *a0,void *a1,void *a2)
{
  if (*(long *)((long)a2 + 0x110))
    sub_10873(a0,a1,*(struct_2 **)((long)a2 + 0x110));
  sub_10873(a0,a1,*(struct_2 **)((long)a2 + 0x118));
}

// Function: sub_9036 @ 0x9036
void sub_9036(unsigned long a0,void *a1,void *a2)
{
  char v1 [16];
  
  v1 = sub_30a66(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x15180);
}

// Function: sub_9081 @ 0x9081
unsigned long sub_9081(int a0)
{
  int v1; // eax
  
  v1 = unlinkat(dat_499d8,dat_499d0,a0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_90b1 @ 0x90b1
unsigned long sub_90b1(char *a0,void *a1) // return-dupe x3
{
  unsigned long v1; // rax
  char *v2; // rax
  int v3; // stack - 0x1c
  
  if (!strcmp(dat_499d0,"."))
    return 1;
  v3 = 0;
  if ((dat_499c4) && ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x4000))
    v3 = 0x200;
  if (sub_9081(v3))
    return 1;
  if ((*__errno_location() == 2) && (dat_4994e)) {
    *__errno_location() = 0;
    return 1;
  }
  if ((*__errno_location() == 0x15) && ((!v3 && (sub_9081(0x200)))))
    return 1;
  v1 = sub_1096a(0,a0);
  v2 = gettext("cannot delete %s");
  error(0,*__errno_location(),v2,v1);
  dat_499e4 = 1;
  return 0;
}

// Function: sub_91f4 @ 0x91f4
unsigned long sub_91f4(char *a0,void *a1) // early-return x5
{
  int v1; // eax
  char *v2; // rax
  DIR *v3; // rax
  unsigned long v4; // rax
  unsigned char v5; // stack - 0x2d
  dirent *v6; // stack - 0x28
  
  if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0x4000) { // branch-flip
    if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0x8000)
      return 0;
    v4 = CONCAT71((undefined7)((unsigned long)*(long *)((long)a1 + 0x30) >> 8),*(long *)((long)a1 + 0x30) == 0);
  }
  else {
    v5 = 1;
    *__errno_location() = 0;
    v1 = sub_2704e(dat_499d8,dat_499d0,0x90900);
    if (v1 <= -1) {
      v2 = (char *)sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      return 0;
    }
    v3 = fdopendir(v1);
    if (!v3) {
      v2 = (char *)sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      close(v1);
      return 0;
    }
    *__errno_location() = 0;
    v6 = readdir(v3);
    while (v6) {
      if ((v6->field_0x13 != '.') || ((v6->field_0x14 && ((v6->field_0x14 != '.' || (v6->field_0x15)))))) {
        v5 = 0;
        break;
      }
      v6 = readdir(v3);
    }
    if (*__errno_location()) {
      v2 = (char *)sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      closedir(v3);
      return 0;
    }
    if (closedir(v3)) {
      v2 = (char *)sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      return 0;
    }
    v4 = (unsigned long)v5;
  }
  return v4;
}

// Function: sub_9466 @ 0x9466
void sub_9466(char *a0,unsigned long a1,struct_19 *a2)
{
  sub_b300(a0,a1,a2);
}

// Function: sub_9497 @ 0x9497
void sub_9497(unsigned long a0,unsigned long a1,struct_19 *a2)
{
  sub_b300(dat_499d0,a1,a2);
}

// Function: sub_94cb @ 0x94cb
unsigned long sub_94cb(void)
{
  return 0;
}

// Function: sub_94e6 @ 0x94e6
unsigned long sub_94e6(unsigned char *a0,void *a1,void *a2)
{
  sub_1b7b3(a0,dat_499d8,dat_499d0,a1,dat_49958,dat_4997c,(unsigned char)*(char *)((long)a2 + 0x28),*(FILE **)((long)a2 + 0x40));
  return 1;
}

// Function: sub_954f @ 0x954f
unsigned long sub_954f(char *a0,unsigned long a1,void *a2)
{
  sub_1c643(*(FILE **)((long)a2 + 0x40),*(unsigned int **)((long)a2 + 0x58),(unsigned char)*(char *)((long)a2 + 0x50),"%s\n",a0);
  return 1;
}

// Function: sub_959f @ 0x959f
unsigned long sub_959f(char *a0,unsigned long a1,void *a2)
{
  FILE *v1;
  
  v1 = *(FILE **)((long)a2 + 0x40);
  fputs(a0,v1);
  putc(0,v1);
  return 1;
}

// Function: sub_95ee @ 0x95ee
bool sub_95ee(unsigned long a0,long *a1,void *a2)
{
  char *v1;
  char *v2; // rax
  
  v1 = *(char **)((long)a2 + 0x38);
  v2 = (char *)sub_13a18(a1,a0);
  return strcmp(v2,v1) == 0;
}

// Function: sub_9643 @ 0x9643
unsigned long sub_9643(unsigned long a0,void *a1,void *a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)((long)a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && ((unsigned long)*(unsigned int *)((long)a1 + 0x20) < *(unsigned long *)((long)a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)((long)a2 + 0x40) < (unsigned long)*(unsigned int *)((long)a1 + 0x20))
        return 1;
    }
  }
  else if ((unsigned long)*(unsigned int *)((long)a1 + 0x20) == *(unsigned long *)((long)a2 + 0x40))
    return 1;
  return 0;
}

// Function: sub_96d8 @ 0x96d8
bool sub_96d8(unsigned long a0,void *a1,void *a2)
{
  return *(int *)((long)a2 + 0x38) == *(int *)((long)a1 + 0x20);
}

// Function: sub_970c @ 0x970c
void sub_970c(char *a0,struct_67 *a1,struct_68 *a2)
{
  sub_9975(a0,a1,a2,1);
}

// Function: sub_973f @ 0x973f
bool sub_973f(char *a0,char *a1,int a2)
{
  int v1; // eax
  char *v2; // rax
  
  v2 = (char *)sub_1d85c(a0);
  sub_1da6f(v2);
  v1 = fnmatch(a1,v2,a2);
  free(v2);
  return v1 == 0;
}

// Function: sub_97a2 @ 0x97a2
void sub_97a2(char *a0,unsigned long a1,void *a2)
{
  sub_973f(a0,*(char **)((long)a2 + 0x38),0x10);
}

// Function: sub_97d8 @ 0x97d8
unsigned long sub_97d8(unsigned long a0,void *a1,void *a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)((long)a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && (*(unsigned long *)((long)a1 + 8) < *(unsigned long *)((long)a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)((long)a2 + 0x40) < *(unsigned long *)((long)a1 + 8))
        return 1;
    }
  }
  else if (*(long *)((long)a1 + 8) == *(long *)((long)a2 + 0x40))
    return 1;
  return 0;
}

// Function: sub_986a @ 0x986a
bool sub_986a(char *a0,unsigned long a1,void *a2)
{
  char *v1;
  
  v1 = *(char **)((long)a2 + 0x38);
  return fnmatch(v1,a0,0x10) == 0;
}

// Function: sub_98b0 @ 0x98b0
unsigned long sub_98b0(unsigned long a0,void *a1,void *a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)((long)a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && (*(unsigned long *)((long)a1 + 0x10) < *(unsigned long *)((long)a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)((long)a2 + 0x40) < *(unsigned long *)((long)a1 + 0x10))
        return 1;
    }
  }
  else if (*(long *)((long)a1 + 0x10) == *(long *)((long)a2 + 0x40))
    return 1;
  return 0;
}

// Function: sub_9942 @ 0x9942
void sub_9942(char *a0,struct_67 *a1,struct_68 *a2)
{
  sub_9975(a0,a1,a2,0);
}

// Function: sub_9975 @ 0x9975
char sub_9975(char *a0,struct_67 *a1,struct_68 *a2,bool a3) // return-dupe, ternary
{
  char *v1;
  char *v2; // rax
  int v3; // edx
  char v4; // stack - 0x11
  
  v4 = 0;
  if ((a1->field_0x18 & 0xf000) != 0xa000)
    return 0;
  v2 = (char *)sub_1d06d(dat_499d8,dat_499d0);
  if (v2) { // branch-flip
    v3 = (a3) ? 0x10 : 0; // branch-flip
    v1 = a2->field_0x38;
    if (!fnmatch(v1,v2,v3))
      v4 = 1;
  }
  else {
    sub_10a13(*__errno_location(),a0);
    dat_499e4 = 1;
  }
  free(v2);
  return v4;
}

// Function: sub_9a33 @ 0x9a33
void sub_9a33(unsigned char *a0,void *a1,void *a2)
{
  sub_94e6(a0,a1,a2);
}

// Function: sub_9a64 @ 0x9a64
void sub_9a64(unsigned long a0,void *a1,void *a2)
{
  char v1 [16];
  
  v1 = sub_30a80(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x3c);
}

// Function: sub_9aaf @ 0x9aaf
void sub_9aaf(unsigned long a0,void *a1,void *a2)
{
  char v1 [16];
  
  v1 = sub_30a80(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x15180);
}

// Function: sub_9afa @ 0x9afa
void sub_9afa(char *a0,unsigned long a1,void *a2)
{
  sub_973f(a0,*(char **)((long)a2 + 0x38),0);
}

// Function: sub_9b30 @ 0x9b30
bool sub_9b30(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_10873(a0,a1,v1) == '\0';
}

// Function: sub_9b79 @ 0x9b79
unsigned long sub_9b79(unsigned long a0,void *a1,void *a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer"); // no-return
  v2 = sub_30a80(a1);
  v1 = sub_8ba9(SUB168(v2,0),SUB168(v2,8),*(long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}

// Function: sub_9c02 @ 0x9c02
unsigned long sub_9c02(char *a0,void *a1,void *a2) // early-return x2
{
  bool v1;
  unsigned long v2; // rax
  char v3 [16];
  long v4; // stack - 0x28
  long v5; // stack - 0x20
  
  v1 = 0;
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY"); // no-return
  switch(*(unsigned int *)((long)a2 + 0x38)) {
    case 0:
      v3 = sub_30a4c(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      break;
    case 1:
      v3 = sub_30a9a(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      if (v5 <= -1) {
        v2 = sub_1096a(0,a0);
        error(0,0,gettext("WARNING: cannot determine birth time of file %s"),v2);
        return 0;
      }
      break;
    case 2:
      v3 = sub_30a66(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      break;
    case 3:
      v3 = sub_30a80(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      break;
    case 4:
      if (*(int *)((long)a2 + 0x38) != 4)
        return 0;
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY"); // no-return
    
  }
  if (v1) {
    v2 = sub_8ba9(v4,v5,*(long *)((long)a2 + 0x40),*(long *)((long)a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v2 >> 8),0 < (int)v2);
  }
  __assert_fail("collected","pred.c",0x29b,"pred_newerXY"); // no-return
}

// Function: sub_9dd5 @ 0x9dd5
unsigned long sub_9dd5(unsigned long a0,void *a1)
{
  group *v1; // rax
  
  v1 = getgrgid(*(unsigned int *)((long)a1 + 0x20));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}

// Function: sub_9e03 @ 0x9e03
unsigned long sub_9e03(unsigned long a0,void *a1)
{
  passwd *v1; // rax
  
  v1 = getpwuid(*(unsigned int *)((long)a1 + 0x1c));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}

// Function: sub_9e31 @ 0x9e31
void sub_9e31(char *a0,char *a1)
{
  char *v1; // rax
  
  sub_1e1a8(stdout);
  v1 = gettext("< %s ... %s > ? ");
  if (fprintf(stderr,v1,a0,a1) <= -1) {
    v1 = gettext("Failed to write prompt for -ok");
    error(1,*__errno_location(),v1);
    return;
  }
  sub_1e1a8(stderr);
  sub_32d7c();
}

// Function: sub_9ece @ 0x9ece
unsigned long sub_9ece(char *a0,unsigned long a1,struct_19 *a2) // early-return
{
  char *v1;
  
  v1 = *(char **)a2->field_0xe8;
  if (sub_9e31(v1,a0))
    return sub_b300(a0,a1,a2);
  return 0;
}

// Function: sub_9f27 @ 0x9f27
unsigned long sub_9f27(char *a0,unsigned long a1,void *a2) // early-return
{
  char *v1;
  unsigned long v2; // rax
  
  v1 = (char *)**(unsigned long **)((long)a2 + 0xe8);
  if (sub_9e31(v1,a0)) {
    v2 = sub_b300(dat_499d0,a1,a2);
    return v2;
  }
  return 0;
}

// Function: sub_9f83 @ 0x9f83
unsigned long sub_9f83(void)
{
  return 1;
}

// Function: sub_9f9e @ 0x9f9e
unsigned long sub_9f9e(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  if ((*(long *)((long)a2 + 0x110)) && (v1 = *(struct_2 **)((long)a2 + 0x110), sub_10873(a0,a1,v1) == '\x01'))
    return 1;
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_10873(a0,a1,v1);
}

// Function: sub_a012 @ 0xa012
bool sub_a012(char *a0,unsigned long a1,void *a2)
{
  char *v1;
  
  v1 = *(char **)((long)a2 + 0x38);
  return fnmatch(v1,a0,0) == 0;
}

// Function: sub_a058 @ 0xa058
unsigned int sub_a058(unsigned long a0,void *a1,void *a2) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4; // eax
  
  v1 = *(unsigned int *)((long)a1 + 0x18);
  v4 = v1 & 0xf000;
  v2 = *(unsigned int *)((long)a2 + ((long)(int)(unsigned int)(v4 == 0x4000) + 0xc) * 4 + 0xc);
  v3 = *(unsigned int *)((long)a2 + 0x38);
  if (v3 != 2) { // branch-flip
    if (2 < v3)
      abort(); // no-return, return-dupe
    if (v3) { // branch-flip
      if (v3 != 1) {
        abort();
      }
      if (!v2) {
        v4 = 1;
        return v4;
      }
      v4 = v1 & v2;
      v4 = CONCAT31((undefined3)(v4 >> 8),v4 != 0);
    }
    else {
      v4 = v1 & v2;
      v4 = CONCAT31((undefined3)(v4 >> 8),v2 == v4);
    }
  }
  else {
    v4 = v1 & 0xfff;
    v4 = CONCAT31((undefined3)(v4 >> 8),v2 == v4);
  }
  return v4;
}

// Function: sub_a0fa @ 0xa0fa
unsigned long sub_a0fa(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_499d8,dat_499d0,1,0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_a137 @ 0xa137
unsigned long sub_a137(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_499d8,dat_499d0,4,0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_a174 @ 0xa174
unsigned long sub_a174(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_499d8,dat_499d0,2,0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_a1b1 @ 0xa1b1
unsigned long sub_a1b1(char *a0,unsigned long a1,void *a2)
{
  sub_1c643(*(FILE **)((long)a2 + 0x40),*(unsigned int **)((long)a2 + 0x58),(unsigned char)*(char *)((long)a2 + 0x50),"%s\n",a0);
  return 1;
}

// Function: sub_a201 @ 0xa201
void sub_a201(char *a0,unsigned long a1,void *a2)
{
  sub_959f(a0,a1,a2);
}

// Function: sub_a232 @ 0xa232
unsigned long sub_a232(unsigned long a0,void *a1)
{
  if (dat_49940) {
    if (!dat_499c4)
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune"); // no-return
    if ((a1) && ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x4000))
      dat_499e0 = 1;
  }
  return 1;
}

// Function: sub_a2b0 @ 0xa2b0
void sub_a2b0(void)
{
  sub_fa38();
  exit(dat_499e4); // no-return
}

// Function: sub_a2da @ 0xa2da
bool sub_a2da(char *a0,unsigned long a1,void *a2)
{
  re_pattern_buffer *v1;
  int v2; // eax
  
  v2 = (int)strlen(a0);
  v1 = *(re_pattern_buffer **)((long)a2 + 0x38);
  return v2 == re_match(v1,a0,v2,0,NULL);
}

// Function: sub_a336 @ 0xa336
unsigned long sub_a336(unsigned long a0,void *a1,void *a2) // return-dupe x3
{
  unsigned int v1;
  unsigned long v2; // rax
  
  v2 = (unsigned long)(*(long *)((long)a1 + 0x30) % (long)*(int *)((long)a2 + 0x3c) != 0) + *(long *)((long)a1 + 0x30) / (long)*(int *)((long)a2 + 0x3c);
  v1 = *(unsigned int *)((long)a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && (v2 < *(unsigned long *)((long)a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)((long)a2 + 0x40) < v2)
        return 1;
    }
  }
  else if (v2 == *(unsigned long *)((long)a2 + 0x40))
    return 1;
  return 0;
}

// Function: sub_a3f7 @ 0xa3f7
unsigned int sub_a3f7(char *a0,struct_24 *a1,void *a2) // return-dupe x2, return-dupe
{
  int v1; // eax
  
  if ((a1->field_0x8) && (a1->field_0x8 != *(long *)((long)a2 + 0x38)))
    return 0;
  v1 = sub_f27d(a0,dat_499d0,a1);
  if (v1)
    return 0;
  if ((a1->field_0x8 == *(long *)((long)a2 + 0x38)) && (a1->field_0x0 == *(long *)((long)a2 + 0x40)))
    return 1;
  return 0;
}

// Function: sub_a497 @ 0xa497
unsigned long sub_a497(void)
{
  return 1;
}

// Function: sub_a4b2 @ 0xa4b2
unsigned long sub_a4b2(unsigned long a0,void *a1,long a2) // return-dupe x2
{
  unsigned int v1; // stack - 0x10
  unsigned int v2; // stack - 0xc
  
  v2 = 7;
  if (!dat_499c5)
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type"); // no-return
  if (!dat_499c8)
    return 0;
  if (dat_499c4) // branch-flip
    v1 = *(unsigned int *)((long)a1 + 0x18);
  else {
    v1 = dat_499c8;
  }
  switch(v1 & 0xf000) { // branch-flip
    case 0x1000:
      v2 = 5;
      break;
    case 0x2000:
      v2 = 1;
      break;
    case 0x4000:
      v2 = 2;
      break;
    case 0x6000:
      v2 = 0;
      break;
    case 0x8000:
      v2 = 3;
      break;
    case 0xa000:
      v2 = 4;
      break;
    case 0xc000:
      v2 = 6;
    
  }
  if ((v2 != 7) && (*(char *)(a2 + 0x38 + (unsigned long)v2)))
    return 1;
  return 0;
}

// Function: sub_a600 @ 0xa600
unsigned long sub_a600(unsigned long a0,void *a1,void *a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)((long)a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && ((unsigned long)*(unsigned int *)((long)a1 + 0x1c) < *(unsigned long *)((long)a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)((long)a2 + 0x40) < (unsigned long)*(unsigned int *)((long)a1 + 0x1c))
        return 1;
    }
  }
  else if ((unsigned long)*(unsigned int *)((long)a1 + 0x1c) == *(unsigned long *)((long)a2 + 0x40))
    return 1;
  return 0;
}

// Function: sub_a695 @ 0xa695
unsigned long sub_a695(unsigned long a0,void *a1,void *a2) // early-return
{
  char v1 [16];
  char v2 [16];
  long v3; // stack - 0x38
  long v4; // stack - 0x30
  
  v1 = sub_30a4c(a1);
  v2 = sub_30a66(a1);
  if ((int)sub_8ba9(SUB168(v1,0),SUB168(v1,8),SUB168(v2,0),SUB168(v2,8)) <= -1)
    return 0;
  v3 = SUB168(v2,0) - SUB168(v1,0);
  v4 = SUB168(v2,8) - SUB168(v1,8);
  if (v4 <= -1) {
    v4 += 1000000000;
    v3 -= 1;
  }
  return sub_8c32(v3,v4,a2,0x15180);
}

// Function: sub_a75a @ 0xa75a
bool sub_a75a(unsigned long a0,void *a1,void *a2)
{
  return *(int *)((long)a2 + 0x38) == *(int *)((long)a1 + 0x1c);
}

// Function: sub_a78e @ 0xa78e
int * sub_a78e(char *a0,void *a1,long a2) // early-return, ternary
{
  char *v1;
  char v2; // al
  int v3; // eax
  int *v4; // rax
  char v5 [152];
  void *v6; // stack - 0xc0
  
  v2 = sub_fe03();
  v6 = (v2) ? sub_fcc0 : sub_fc21; // branch-flip
  sub_f26e(v5);
  v3 = (*v6)(dat_499d0,v5);
  if (v3) { // branch-flip
    v2 = sub_fe03();
    if ((!v2) || (v4 = __errno_location(), *v4 != 2)) {
      v4 = (char *)sub_1096a(0,a0);
      v1 = (char *)v4;
      v4 = __errno_location();
      error(0,*v4,"%s",v1);
      v4 = NULL;
      dat_499e4 = 1;
      return v4;
    }
    v4 = (unsigned long)sub_a4b2(a0,a1,a2);
  }
  else {
    v4 = (unsigned long)sub_a4b2(a0,v5,a2);
  }
  return v4;
}

// Function: sub_a8d8 @ 0xa8d8
unsigned long sub_a8d8(char *a0,unsigned long a1,void *a2) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  char *v5; // stack - 0x28
  
  v1 = (*dat_499a0)(dat_499d8,dat_499d0,&v5);
  if (v1 <= -1) {
    v2 = sub_1096a(0,a0);
    v3 = gettext("getfilecon failed: %s");
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  v1 = fnmatch(*(char **)((long)a2 + 0x38),v5,0);
  freecon(v5);
  return CONCAT71((undefined7)((unsigned long)v4 >> 8),v1 == 0);
}

// Function: sub_a9cd @ 0xa9cd
char * sub_a9cd(char *a0,char *a1) // early-return
{
  int v1;
  int v2; // stack - 0xc
  
  if (!a0)
    return NULL;
  strcpy(a1,a0);
  v2 = (int)strlen(a1);
  do {
    v1 = v2;
    v2 = v1 + -1;
    if (v2 <= -1) break;
  } while ((a1[v2] == ' ') || (a1[v2] == '\t'));
  a1[v1] = '\0';
  return a1;
}

// Function: sub_aa62 @ 0xaa62
void sub_aa62(FILE *a0,void *a1)
{
  char *v1;
  char v2 [264];
  void *v3; // stack - 0x120
  
  for (v3 = a1; v3; v3 = *(void **)((long)v3 + 0x108)) {
    v1 = *(char **)((long)v3 + 8);
    fprintf(a0,"[%s] ",(char *)sub_a9cd(v1,v2));
  }
  fprintf(a0,"\n");
}

// Function: sub_ab30 @ 0xab30
void sub_ab30(FILE *a0,struct_23 *a1) // return-dupe x3
{
  bool v1;
  
  v1 = 0;
  if (!a1)
    return;
  if (((a1->field_0x0 == sub_9f9e) || (a1->field_0x0 == sub_8d9c)) && (!a1->field_0x110)) {
    sub_ab30(a0,a1->field_0x118);
    return;
  }
  if ((a1->field_0x110) || (a1->field_0x118))
    v1 = 1;
  if (v1)
    fprintf(a0,"%s"," ( ");
  sub_ac38(a0,a1);
  if (!v1)
    return;
  fprintf(a0,"%s"," ) ");
}

// Function: sub_ac38 @ 0xac38
void sub_ac38(FILE *a0,void *a1) // return-dupe, ternary x3
{
  char *v1; // rax
  char *v2; // rcx
  char *v3; // rdx
  
  if (!a1)
    return;
  sub_ab30(a0,*(struct_23 **)((long)a1 + 0x110));
  v2 = (*(char *)((long)a1 + 0x1c)) ? "[need inum] " : ""; // branch-flip
  v3 = (*(char *)((long)a1 + 0x1b)) ? "[need type] " : ""; // branch-flip
  v1 = (*(char *)((long)a1 + 0x1a)) ? "[call stat] " : ""; // branch-flip
  fprintf(a0,"%s%s%s",v1,v3,v2);
  sub_c2dc(a0,a1);
  fprintf(a0," [est success rate %.4g] ",(double)*(float *)((long)a1 + 0x24));
  if (dat_49980 & 0x40) {
    fprintf(a0,"[real success rate %lu/%lu",*(unsigned long *)((long)a1 + 0x128),*(unsigned long *)((long)a1 + 0x120));
    if (*(long *)((long)a1 + 0x120)) // branch-flip
      fprintf(a0,"=%.4g] ",(double)*(unsigned long *)((long)a1 + 0x128) / (double)*(unsigned long *)((long)a1 + 0x120));
    else {
      fprintf(a0,"=_] ");
    }
  }
  sub_ab30(a0,*(struct_23 **)((long)a1 + 0x118));
}

// Function: sub_ae53 @ 0xae53
void sub_ae53(void *a0) // return-dupe
{
  if (!(dat_49980 & 0x40))
    return;
  fprintf(stderr,"Predicate success rates after completion:\n");
  sub_ac38(stderr,a0);
  fprintf(stderr,"\n");
}

// Function: sub_aec7 @ 0xaec7
void sub_aec7(long *a0)
{
  long *v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (!v1)
      return;
    if (!*v1)
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check"); // no-return
    if (!v1[0x26])
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check"); // no-return
    if ((*(long *)(v1[0x26] + 0x18)) && (*(long *)(v1[0x26] + 0x18) != *v1)) break;
    switch(*(unsigned int *)v1[0x26]) {
      case 0:
      case 2:
        if (!*(int *)v1[0x26])
          __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check"); // no-return
        if (*(int *)v1[0x26] == 2)
          __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,"pred_sanity_check"); // no-return
        break;
      case 1:
      case 3:
      case 4:
      case 5:
        if (*(char *)((long)v1 + 0x19) == '\x01')
          __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check"); // no-return
        if ((char)v1[3] == '\x01')
          __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check"); // no-return
        break;
      case 6:
        if (!(char)v1[3])
          __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check"); // no-return
        if ((((void *)*v1 != sub_a232) && ((void *)*v1 != sub_a2b0)) && (!*(char *)((long)v1 + 0x19)))
          __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check"); // no-return
      
    }
    v1 = (long *)v1[0x21];
  }
  __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check"); // no-return
}

// Function: sub_b184 @ 0xb184
bool sub_b184(struct_5 *a0,unsigned int a1,char *a2)
{
  unsigned int *v1;
  bool v2; // sf
  
  a0->field_0xc0 = sub_31bc2(0x10);
  *(unsigned long *)(a0->field_0xc0 + 8) = 0;
  v1 = (unsigned int *)a0->field_0xc0;
  *v1 = sub_2704e(a1,a2,0);
  v2 = 0 <= *(int *)a0->field_0xc0;
  if (v2)
    sub_1d5ba(*(unsigned int *)a0->field_0xc0,1);
  return v2;
}

// Function: sub_b22b @ 0xb22b
char * sub_b22b(struct_6 *a0) // early-return
{
  char *v1;
  unsigned char v2; // al
  char *v3; // rax
  
  if (a0->field_0x98) {
    v3 = (char *)0x1;
    return v3;
  }
  if (!a0->field_0x98) {
    v3 = strchr(dat_499d0,0x2f);
    if (v3) { // branch-flip
      v3 = (char *)sub_1d9d4(dat_499d0);
      v1 = v3;
      v2 = sub_b184(a0,dat_499d8,v3);
      free(v1);
      v3 = (unsigned long)(unsigned long)v2;
    }
    else {
      v3 = (unsigned long)sub_b184(a0,dat_499d8,".");
    }
    return v3;
  }
  __assert_fail("!execp->state.todo","exec.c",0x47,"record_exec_dir"); // no-return
}

// Function: sub_b300 @ 0xb300
unsigned long sub_b300(char *a0,unsigned long a1,struct_19 *a2) // ternary
{
  char v1; // al
  char *v10; // stack - 0x38
  char *v11; // stack - 0x30
  long v12; // stack - 0x28
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned char v7; // stack - 0x46
  int v8; // stack - 0x44
  char *v9; // stack - 0x40
  
  v9 = NULL;
  v1 = sub_10933(a2->field_0x0);
  if (v1) { // branch-flip
    if (sub_b22b(&a2->field_0x38) != '\x01') {
      v2 = sub_1096a(0,a0);
      v3 = gettext("Failed to save working directory in order to run a command on %s");
      error(1,*__errno_location(),v3,v2);
      return v4;
    }
    v9 = (char *)sub_1d85c(dat_499d0);
    v10 = v9;
    if (*v9 != '/') { // branch-flip
      v11 = "./";
      v12 = 2;
    }
    else {
      v11 = NULL;
      v12 = 0;
    }
  }
  else {
    if (a2->field_0xf8 != dat_49a00)
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec"); // no-return
    v11 = NULL;
    v12 = 0;
    v10 = a0;
  }
  if (a2->field_0x38) { // branch-flip
    sub_1a1b8((struct_13 *)&a2->field_0x39[7],(struct_12 *)&a2->field_0x39[0x5f],v10,strlen(v10) + 1,v11,v12,0);
    if (a2->field_0xd0)
      dat_499e8 = 1;
    v7 = 1;
  }
  else {
    for (v8 = 0; v8 < a2->field_0xf0; v8 = v8 + 1) {
      v5 = strlen(v10);
      v6 = strlen(*(char **)((long)v8 * 8 + a2->field_0xe8));
      sub_19af7(&a2->field_0x39[7],(struct_12 *)&a2->field_0x39[0x5f],*(char **)((long)v8 * 8 + a2->field_0xe8),v6,v11,v12,v10,v5,0);
    }
    sub_19f5a(&a2->field_0x39[7],(struct_14 *)&a2->field_0x39[0x5f]);
    if (a2->field_0x100 & 0x7f) // branch-flip
      v7 = 0;
    else {
      v7 = ((int)a2->field_0x100 >> 8 & 0xffU) ? 0 : 1; // branch-flip
    }
    if (v1)
      sub_2fe52(a2->field_0xf8);
  }
  if (v9) {
    if (!v1)
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec"); // no-return
    free(v9);
  }
  return (unsigned long)v7;
}

// Function: sub_b615 @ 0xb615
char sub_b615(bool a0,struct_21 *a1) // return-dupe
{
  char *v1;
  char *v2; // rax
  char *v3; // rbx
  unsigned long v4; // stack - 0x3a
  char v5; // stack - 0x3b
  unsigned short v6; // stack - 0x32
  
  v5 = 1;
  if (a0) {
    v4 = 0x6c756e2f7665642f;
    v6 = 0x6c;
    if (0 <= close(0)) { // branch-flip
      if ((int)sub_1e034(&v4,0) <= -1) {
        v1 = (char *)sub_1096a(0,&v4);
        error(0,*__errno_location(),"%s",v1);
      }
    }
    else {
      v1 = gettext("Cannot close standard input");
      error(0,*__errno_location(),v1);
      v5 = 0;
    }
  }
  if (!sub_2fe17(a1))
    return v5;
  if ((0 <= a1->field_0x0) || (!a1->field_0x8))
    v1 = "";
  else {
    v1 = a1->field_0x8;
  }
  if ((0 <= a1->field_0x0) || (!a1->field_0x8))
    v3 = "";
  else {
    v3 = ": ";
  }
  v2 = gettext("Failed to change directory%s%s");
  error(0,*__errno_location(),v2,v3,v1);
  return 0;
}

// Function: sub_b7ae @ 0xb7ae
unsigned long sub_b7ae(unsigned long a0,struct_69 *a1,unsigned long a2,char **a3) // return-dupe x2
{
  char v1;
  struct_21 *v2;
  int v3; // eax
  unsigned int v4;
  char *v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  int v8; // stack - 0x2c
  
  if (dat_49980 & 0x20) {
    fprintf(stderr,"DebugExec: launching process (argc=%lu):",a1->field_0x60 - 1);
    for (v8 = 0; (unsigned long)(long)v8 < (unsigned long)(a1->field_0x60 - 1U); v8 = v8 + 1) {
      v5 = *(char **)((long)v8 * 8 + a1->field_0x68);
      fprintf(stderr," %s",(char *)sub_1096a(0,v5));
    }
    fprintf(stderr,"\n");
  }
  sub_1e1a8(stdout);
  sub_1e1a8(stderr);
  if (dat_4901c) {
    dat_4901c = 0;
    signal(0x11,0);
  }
  v3 = fork();
  if (v3 == -1) {
    v5 = gettext("cannot fork");
    error(1,*__errno_location(),v5);
    return v6;
  }
  if (!v3) {
    if (!a1->field_0xc0)
      __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch"); // no-return
    v2 = (struct_21 *)a1->field_0xc0;
    v1 = a1->field_0xbc;
    if (sub_b615((unsigned char)v1,v2) != '\x01')
      _exit(1); // no-return
    if (sub_1b5f4())
      sub_1b61e();
    if (sub_1a94d(a3)) // branch-flip
      *__errno_location() = 7;
    else {
      execvp(*a3,a3);
    }
    v5 = (char *)sub_1096a(0,*a3);
    error(0,*__errno_location(),"%s",v5);
    _exit(1); // no-return
  }
  while (waitpid(v3,&a1->field_0xc8,0) == -1) {
    if (*__errno_location() != 4) {
      v7 = sub_1096a(0,*a3);
      v5 = gettext("error waiting for %s");
      error(0,*__errno_location(),v5,v7);
      dat_499e4 = 1;
      return 0;
    }
  }
  if ((char)(((unsigned char)a1->field_0xc8 & 0x7f) + 1) >> 1 <= '\0') {
    v4 = (int)a1->field_0xc8 >> 8 & 0xff;
    if (dat_49980 & 0x20)
      fprintf(stderr,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)v3,v4);
    if (v4) {
      if (!a1->field_0x0)
        return 1;
      dat_499e4 = 1;
      return 1;
    }
    return 1;
  }
  v4 = a1->field_0xc8;
  v5 = (char *)sub_2f861(0,dat_499ac,*a3);
  error(0,0,gettext("%s terminated by signal %d"),v5,v4 & 0x7f);
  if (!a1->field_0x0)
    return 1;
  dat_499e4 = 1;
  return 1;
}

// Function: sub_bb9d @ 0xbb9d
unsigned long sub_bb9d(char *a0,bool a1) // early-return
{
  char *v1;
  int v2; // eax
  int v3; // stack - 0x14
  unsigned long v4; // stack - 0x10
  
  v3 = 0;
  if (a1)
    v3 = 0x10;
  if (dat_49a28) {
    v4 = 0;
    while( true ) {
      if (dat_49a28 <= v4)
        return 0;
      v1 = *(char **)(v4 * 8 + dat_49a20);
      if (!fnmatch(a0,v1,v3)) break;
      v4 += 1;
    }
    return 1;
  }
  v2 = fnmatch(a0,".",v3);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
}

// Function: sub_bc44 @ 0xbc44
char * sub_bc44(long *a0,short a1,void *a2)
{
  long v1;
  unsigned long v2;
  char *v3; // rax
  long v4; // stack - 0x28
  
  v1 = *a0;
  if (!*a0) {
    v3 = gettext("invalid expression");
    error(1,0,v3);
    return v3;
  }
  switch(*(unsigned int *)(*a0 + 0x10)) {
    case 0:
      v3 = gettext("invalid expression");
      error(1,0,v3);
      goto label_bd00;
    case 2:
      v4 = *a0;
      *a0 = *(long *)(*a0 + 0x108);
      v3 = (unsigned long)sub_bc44(a0,4,v4);
      *(char **)(v4 + 0x118) = v3;
      break;
    case 3:
label_bd00:
      v2 = *(unsigned long *)(v1 + 8);
      v3 = gettext("invalid expression; you have used a binary operator \'%s\' with nothing before it.");
      error(1,0,v3,v2);
      goto label_bd31;
    case 4:
      if ((!*(long *)(*a0 + 0x108)) || (*(char *)(*(long *)(*a0 + 0x108) + 0x29))) {
        v2 = *(unsigned long *)(v1 + 8);
        v3 = gettext("invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'");
        error(1,0,v3,v2);
      }
      v1 = *a0;
      *a0 = *(long *)(*a0 + 0x108);
      if (*(int *)(*a0 + 0x10) == 5) {
        if (*(char *)(v1 + 0x29)) {
          v2 = *(unsigned long *)(*a0 + 8);
          v3 = gettext("invalid expression: expected expression before closing parentheses \'%s\'.");
          error(1,0,v3,v2);
        }
        v3 = gettext("invalid expression; empty parentheses are not allowed.");
        error(1,0,v3);
      }
      v3 = (long)sub_bc44(a0,0,v1);
      v4 = (long)v3;
      if ((!*a0) || (*(int *)(*a0 + 0x10) != 5)) {
        v3 = gettext("invalid expression; I was expecting to find a \')\' somewhere but did not see one.");
        error(1,0,v3);
      }
      *a0 = *(long *)(*a0 + 0x108);
      break;
    case 5:
label_bd31:
      if (!a2) {
        v2 = *(unsigned long *)(v1 + 8);
        v3 = gettext("invalid expression: expected expression before closing parentheses \'%s\'.");
        error(1,0,v3,v2);
      }
      if (((*(int *)((long)a2 + 0x10) == 2) || (*(int *)((long)a2 + 0x10) == 3)) && (*(char *)(v1 + 0x29) != '\x01')) {
        v2 = *(unsigned long *)((long)a2 + 8);
        v3 = gettext("expected an expression between \'%s\' and \')\'");
        error(1,0,v3,v2);
      }
      if (*(char *)(*a0 + 0x29)) {
        v2 = *(unsigned long *)((long)a2 + 8);
        v3 = gettext("expected an expression after \'%s\'");
        error(1,0,v3,v2);
      }
      v3 = gettext("invalid expression; you have too many \')\'");
      error(1,0,v3);
      goto label_be27;
    case 1:
label_be27:
      v4 = *a0;
      *a0 = *(long *)(*a0 + 0x108);
      break;
    default:
      v3 = gettext("oops -- invalid expression type!");
      error(1,0,v3);
      return v3;
    
  }
  if (*a0) { // branch-flip
    if ((int)a1 < *(int *)(*a0 + 0x14)) {
      v3 = (long)sub_c08f(a0,v4,(int)a1);
      v4 = (long)v3;
      if (!v3) {
        v3 = gettext("invalid expression");
        error(1,0,v3);
        return v3;
      }
    }
    v3 = (char *)v4;
  }
  else {
    v3 = (char *)v4;
  }
  return v3;
}

// Function: sub_c08f @ 0xc08f
long sub_c08f(long *a0,long a1,short a2) // early-return
{
  unsigned int v1;
  unsigned int v2;
  int v3;
  long v4;
  long v5; // rax
  long v6; // rax
  long v7; // stack - 0x28
  
  if ((!*a0) || (*(int *)(*a0 + 0x10) == 5))
    return 0;
  v7 = a1;
  while( true ) {
    if (((!*a0) || (*(int *)(*a0 + 0x14) <= (int)a2)) || (v1 = *(unsigned int *)(*a0 + 0x10), v1 == 5))
      return v7;
    if (5 < v1) break;
    if (v1 == 4) {
      error(1,0,gettext("invalid expression")); // return-dupe
      return v5;
    }
    if (4 < v1) break;
    if (v1 <= 2) {
      error(1,0,gettext("invalid expression"));
      return v5;
    }
    if (v1 != 3) break;
    v4 = *a0;
    *(long *)(*a0 + 0x110) = v7;
    v7 = *a0;
    *a0 = *(long *)(*a0 + 0x108);
    v2 = *(unsigned int *)(v7 + 0x14);
    *(unsigned long *)(v7 + 0x118) = sub_bc44(a0,(int)(short)v2,v4);
  }
  v3 = *(int *)(*a0 + 0x10);
  error(1,0,gettext("oops -- invalid expression type (%d)!"),v3);
  return v6;
}

// Function: sub_c209 @ 0xc209
unsigned int sub_c209(struct_48 *a0) // early-return x3
{
  if ((((a0->field_0x0 == sub_9afa) || (a0->field_0x0 == sub_a012)) || (a0->field_0x0 == sub_97a2)) || (a0->field_0x0 == sub_986a))
    return 1;
  if (!dat_499a8)
    return 0;
  if (((a0->field_0x0 != sub_8d9c) && (a0->field_0x0 != sub_9b30)) && ((a0->field_0x0 != sub_8fd0 && (a0->field_0x0 != sub_9f9e))))
    return CONCAT31((undefined3)((unsigned int)a0->field_0x20 >> 8),a0->field_0x20 == 0);
  return 0;
}

// Function: sub_c2dc @ 0xc2dc
void sub_c2dc(FILE *a0,void *a1) // return-dupe
{
  if (*(long *)((long)a1 + 0x30)) {
    fprintf(a0,"%s %s",*(unsigned long *)((long)a1 + 8),*(unsigned long *)((long)a1 + 0x30));
    return;
  }
  fprintf(a0,"%s",*(unsigned long *)((long)a1 + 8));
}

// Function: sub_c34d @ 0xc34d
void sub_c34d(struct_31 *a0)
{
  a0->field_0x8 = 0;
  a0->field_0x0 = a0->field_0x8;
}

// Function: sub_c377 @ 0xc377
void sub_c377(struct_31 *a0,struct_49 *a1,unsigned long *a2) // return-dupe
{
  *a2 = a1->field_0x110;
  a1->field_0x110 = a0->field_0x0;
  a0->field_0x0 = a1;
  if (a0->field_0x8)
    return;
  a0->field_0x8 = a0->field_0x0;
}

// Function: sub_c3e1 @ 0xc3e1
unsigned long sub_c3e1(void *a0,void *a1,bool a2) // return-dupe x6
{
  if (*(int *)((long)a0 + 0x20) != *(int *)((long)a1 + 0x20)) {
    if (*(unsigned int *)((long)a0 + 0x20) < *(unsigned int *)((long)a1 + 0x20))
      return 0xffffffff;
    return 1;
  }
  if (*(float *)((long)a0 + 0x24) == *(float *)((long)a1 + 0x24))
    return 0;
  if (!a2) {
    if (*(float *)((long)a0 + 0x24) < *(float *)((long)a1 + 0x24))
      return 1;
    return 0xffffffff;
  }
  if (*(float *)((long)a0 + 0x24) < *(float *)((long)a1 + 0x24))
    return 0xffffffff;
  return 1;
}

// Function: sub_c49b @ 0xc49b
void sub_c49b(struct_31 *a0,unsigned long *a1) // return-dupe
{
  int v1;
  long v2;
  void *v3;
  void *v4;
  long v5; // stack - 0x28
  long v6; // stack - 0x38
  long v7; // stack - 0x20
  
  if (!a0->field_0x0)
    return;
  if (dat_49980 & 8) {
    fprintf(stderr,"%s:\n","predlist before merge sort");
    sub_ea9f(stderr,(void *)a0->field_0x0,2);
  }
  sub_d838(a0->field_0x0);
  sub_c34d(&v5);
  while (a0->field_0x0) {
    v2 = a0->field_0x0;
    a0->field_0x0 = *(unsigned long *)(a0->field_0x0 + 0x110);
    *(unsigned long *)(v2 + 0x110) = 0;
    v6 = v5;
    while ((v6 && (v1 = *(int *)(v6 + 0x14), v3 = *(void **)(v2 + 0x118), v4 = *(void **)(v6 + 0x118), (int)sub_c3e1(v4,v3,v1 != 2) <= -1))) {
      v6 = *(long *)(v6 + 0x110);
    }
    if (v6) { // branch-flip
      *(unsigned long *)(v2 + 0x110) = *(unsigned long *)(v6 + 0x110);
      if (!*(long *)(v2 + 0x110))
        v7 = v2;
      *(long *)(v6 + 0x110) = v2;
    }
    else {
      *(long *)(v2 + 0x110) = v5;
      v5 = v2;
      if (!v7)
        v7 = v2;
    }
  }
  if (dat_49980 & 8) {
    fprintf(stderr,"%s:\n","predlist after merge sort");
    sub_ea9f(stderr,v5,2);
  }
  sub_d838(v5);
  sub_d3fd(v5,v7,a1);
  sub_c34d(a0);
}

// Function: sub_c6e0 @ 0xc6e0
void sub_c6e0(long a0,int a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  int v1; // stack - 0xc
  
  dat_49a38 = sub_c49b;
  sub_c49b(a2,a4);
  (*dat_49a38)(a3,a4);
  for (v1 = 0; v1 < a1; v1 = v1 + 1) {
    (*dat_49a38)((long)v1 * 0x10 + a0,a4);
  }
}

// Function: sub_c77a @ 0xc77a
unsigned int sub_c77a(void *a0) // return-dupe
{
  unsigned long v1;
  
  if (!a0)
    return 0;
  if (((!*(char *)((long)a0 + 0x18)) && (v1 = *(unsigned long *)((long)a0 + 0x110), !sub_c77a(v1))) && (v1 = *(unsigned long *)((long)a0 + 0x118), !sub_c77a(v1)))
    return 0;
  return 1;
}

// Function: sub_c7e3 @ 0xc7e3
unsigned int sub_c7e3(void *a0) // early-return
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned int v3; // stack - 0x14
  
  if (a0) {
    v2 = sub_c7e3(*(unsigned long *)((long)a0 + 0x110));
    v1 = v2;
    v2 = sub_c7e3(*(unsigned long *)((long)a0 + 0x118));
    if (v1 <= v2)
      v1 = v2;
    v2 = v1;
    v3 = v2;
    if (v2 < *(unsigned int *)((long)a0 + 0x20))
      v3 = *(unsigned int *)((long)a0 + 0x20);
    v2 = v3;
    return v2;
  }
  v2 = 0;
  return v2;
}

// Function: sub_c856 @ 0xc856
void sub_c856(struct_47 *a0)
{
  long v1;
  
  v1 = *(long *)(a0->field_0x110 + 0x118);
  *(long *)(a0->field_0x110 + 0x118) = a0->field_0x118;
  a0->field_0x118 = v1;
}

// Function: sub_c8a7 @ 0xc8a7
unsigned long sub_c8a7(struct_47 *a0) // return-dupe
{
  float v1;
  float v2;
  void *v3;
  bool v4;
  long *v5; // rax
  long *v6; // rax
  int v7; // stack - 0x30
  int v8; // stack - 0x2c
  char *v9; // stack - 0x20
  
  v9 = NULL;
  if (a0->field_0x10 != 3)
    v9 = "Not a binary operation";
  if ((!v9) && ((!a0->field_0x110 || (!a0->field_0x118))))
    v9 = "Doesn\'t have two arms";
  if ((!v9) && (!*(long *)(a0->field_0x110 + 0x118)))
    v9 = "Left arm has no child on RHS";
  v5 = &a0->field_0x118;
  v6 = (long *)(a0->field_0x110 + 0x118);
  if ((!v9) && (v3 = (void *)*v6, sub_c77a(v3)))
    v9 = "Left subtree has side-effects";
  if ((!v9) && (v3 = (void *)*v5, sub_c77a(v3)))
    v9 = "Right subtree has side-effects";
  if (!v9) {
    v7 = sub_c7e3((void *)*v6);
    v8 = sub_c7e3((void *)*v5);
    if (v7 < v8)
      v9 = "efficient as-is";
  }
  if (!v9) {
    if (v7 != v8) // branch-flip
      v4 = 1;
    else {
      v1 = *(float *)(*v6 + 0x24);
      v2 = *(float *)(*v5 + 0x24);
      if (dat_49980 & 8)
        fprintf(stderr,"Success rates: l=%f, r=%f\n",(double)v1,(double)v2);
      if (a0->field_0x0 != sub_9f9e) { // branch-flip
        if (a0->field_0x0 != sub_8d9c) { // branch-flip
          v4 = 0;
          v9 = "Not \'AND\' or \'OR\'";
        }
        else {
          v4 = v1 < v2;
          if (v2 <= v1)
            v9 = "Operation is AND; right success rate <= left";
        }
      }
      else {
        v4 = v2 < v1;
        if (v1 <= v2)
          v9 = "Operation is OR; right success rate >= left";
      }
    }
    if (v4) {
      if (dat_49980 & 8) {
        fprintf(stderr,"Performing arm swap on:\n");
        sub_ea9f(stderr,a0,0);
      }
      sub_c856(a0);
      return 1;
    }
  }
  if (!(dat_49980 & 8))
    return 0;
  fprintf(stderr,"Not an arm swap candidate (%s):\n",v9);
  sub_ea9f(stderr,a0,0);
  return 0;
}

// Function: sub_cb9f @ 0xcb9f
char sub_cb9f(struct_47 *a0) // return-dupe
{
  long v1;
  bool v2;
  
  if (a0) {
    do {
      v2 = 0;
      if (((sub_c8a7(a0)) || (v1 = a0->field_0x110, sub_cb9f(v1))) || (v1 = a0->field_0x118, sub_cb9f(v1)))
        v2 = 1;
    } while (v2);
    return 0;
  }
  return 0;
}

// Function: sub_cc0f @ 0xcc0f
unsigned long sub_cc0f(long *a0) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_cc0f
{
  unsigned int v1;
  unsigned long v10; // stack - 0xe8
  unsigned long v11; // stack - 0xf8
  unsigned char v12; // stack - 0x12a
  int v13; // stack - 0x128
  unsigned int v14; // stack - 0x124
  int v15; // stack - 0x120
  long v16; // stack - 0x118
  long *v17; // stack - 0x110
  unsigned long v18; // stack - 0xf0
  unsigned long v19; // stack - 0xe0
  unsigned int v2;
  void *v3;
  unsigned long v4;
  long v5;
  long v6;
  bool v7; // al
  unsigned long v8; // rax
  char v9 [184];
  
  v11 = 0;
  v18 = 0;
  v10 = 0;
  v19 = 0;
  v12 = 0;
  if ((!a0) || (!*a0))
    return 0;
  for (v13 = 0; v13 <= 10; v13 = v13 + 1) {
    sub_c34d(&v9[(long)v13 * 0x10]);
  }
  v14 = 3;
  v17 = a0;
  for (v16 = *a0; *(long *)(v16 + 0x110); v16 = *(long *)(v16 + 0x110)) {
    v17 = (long *)(v16 + 0x110);
    v14 = *(unsigned int *)(v16 + 0x14);
  }
  if (*(int *)(v16 + 0x10) != 3)
    sub_d2d0(v16,v14,v17);
  if (dat_49980 & 9) {
    fprintf(stderr,"Normalized Eval Tree:\n");
    sub_ea9f(stderr,(void *)*a0,0);
  }
  v15 = 0;
  v17 = a0;
  if ((*a0) && (*(int *)(*a0 + 0x10) == 3))
    v15 = *(int *)(*a0 + 0x14);
label_d216:
  do {
    v16 = *v17;
    if (!v16) {
      sub_c6e0(v9,0xb,&v10,&v11,v17);
      return (unsigned long)v12;
    }
    if ((*(int *)(v16 + 0x10) == 3) && (v15 != *(int *)(v16 + 0x14)))
      v16 = sub_d2d0(v16,v15,v17);
    v1 = *(unsigned int *)(*(long *)(v16 + 0x118) + 0x10);
    v3 = (void *)**(long **)(v16 + 0x118);
    if (v1 == 3) {
      v5 = *(long *)(v16 + 0x118);
      *(char *)(v5 + 0x18) = sub_cc0f(v16 + 0x118);
      goto label_d1ab;
    }
    if (4 <= v1) {
      error(1,0,gettext("oops -- invalid expression type!")); // return-dupe
      return v8;
    }
    if (2 <= v1) {
      if (v1 != 2) {
        error(1,0,gettext("oops -- invalid expression type!"));
        return v8;
      }
      v5 = *(long *)(v16 + 0x118);
      v6 = *(long *)(v16 + 0x118);
      *(char *)(v6 + 0x18) = sub_cc0f(v5 + 0x118);
label_d1ab:
      if (*(char *)(*(long *)(v16 + 0x118) + 0x18)) {
        sub_c6e0(v9,0xb,&v10,&v11,v17);
        v12 = 1;
      }
      v17 = (long *)(v16 + 0x110);
      goto label_d216;
    }
    if ((v15 == 1) || (*(char *)(*(long *)(v16 + 0x118) + 0x18) == '\x01')) goto label_d1ab;
    v4 = *(unsigned long *)(v16 + 0x118);
    if (sub_c209(v4)) { // branch-flip
      if (dat_49980 & 8) {
        fprintf(stderr,"-O%d: promoting cheap predicate ",(unsigned int)dat_499a8);
        sub_c2dc(stderr,*(void **)(v16 + 0x118));
        fprintf(stderr," into name_list\n");
      }
      sub_c377(&v10,v16,v17);
    }
    else if (v3 != sub_a2da) { // branch-flip
      v7 = (((2 <= dat_499a8) && (((*(int *)(*(long *)(v16 + 0x118) + 0x20) == 2 || (*(int *)(*(long *)(v16 + 0x118) + 0x20) == 1)) && (*(char *)(*(long *)(v16 + 0x118) + 0x1a) != '\x01')))) || (3 <= dat_499a8));
      if (!v7) goto label_d1ab;
      if (dat_49980 & 8) {
        fprintf(stderr,"-O%d: categorising predicate ",(unsigned int)dat_499a8);
        sub_c2dc(stderr,*(void **)(v16 + 0x118));
        v2 = *(unsigned int *)(*(long *)(v16 + 0x118) + 0x20);
        fprintf(stderr," by cost (%s)\n",(char *)sub_e95d(v2));
      }
      sub_c377(&v9[(unsigned long)*(unsigned int *)(*(long *)(v16 + 0x118) + 0x20) * 0x10],v16,v17);
    }
    else {
      sub_c377(&v11,v16,v17);
    }
  } while( true );
}

// Function: sub_d290 @ 0xd290
void sub_d290(void)
{
  return;
}

// Function: sub_d2d0 @ 0xd2d0
struct_51 * sub_d2d0(unsigned long a0,unsigned int a1,unsigned long *a2)
{
  struct_51 *v1; // rax
  unsigned int v2; // xmm0_da
  unsigned int v3; // xmm0_da
  
  v1 = (struct_51 *)sub_3205a(0x138);
  v1->field_0x10 = 3;
  v1->field_0x14 = a1;
  v1->field_0x20 = 0;
  if (a1 != 3) { // branch-flip
    if (4 <= a1) goto label_d3dd;
    if (a1 != 1) { // branch-flip
      if (a1 == 2) {
        v1->field_0x0 = sub_9f9e;
        v1->field_0x8 = "-o";
        sub_d290();
        v1->field_0x24 = v2;
      }
    }
    else {
      v1->field_0x0 = sub_8fd0;
      v1->field_0x8 = ",";
      v1->field_0x24 = dat_39684;
    }
  }
  else {
    v1->field_0x0 = sub_8d9c;
    v1->field_0x8 = "-a";
    sub_d290();
    v1->field_0x24 = v3;
  }
label_d3dd:
  v1->field_0x118 = a0;
  *a2 = v1;
  return v1;
}

// Function: sub_d3fd @ 0xd3fd
void sub_d3fd(unsigned long a0,struct_49 *a1,unsigned long *a2)
{
  a1->field_0x110 = *a2;
  *a2 = a0;
}

// Function: sub_d431 @ 0xd431
unsigned long sub_d431(long a0,unsigned long a1,long a2,void *a3)
{
  unsigned long v1; // stack - 0x18
  
  v1 = 1;
  while( true ) {
    if (a1 <= v1)
      return 1;
    if ((int)(*a3)(v1 * a2 + a0,(v1 - 1) * a2 + a0) <= -1) break;
    if (1 <= (int)(*a3)((v1 - 1) * a2 + a0,v1 * a2 + a0))
      __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted"); // no-return
    v1 += 1;
  }
  return 0;
}

// Function: sub_d52e @ 0xd52e
void sub_d52e(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x20
  
  v2 = *a0;
  v1 = *a1;
  memcmp(&v2,&v1,8);
}

// Function: sub_d5a5 @ 0xd5a5
char * sub_d5a5(struct_54 *a0) // ternary x2
{
  char *v1;
  char *v2; // rax
  void *v3 [3]; // stack - 0x38
  unsigned int v4; // stack - 0x48
  unsigned int v5; // stack - 0x44
  
  if (a0->field_0x1a) // branch-flip
    v4 = 3;
  else if (a0->field_0x1c) // branch-flip
    v4 = 1;
  else {
    v4 = (a0->field_0x1b) ? 2 : 0; // branch-flip
  }
  if ((a0->field_0x0 != sub_9466) && (a0->field_0x0 != sub_9497)) {
    if (a0->field_0x0 != sub_1376d) { // branch-flip
      if (!dat_49a30) {
        qsort(0x49020,0x3b,0x10,sub_d52e);
        if (sub_d431(0x49020,0x3b,0x10,sub_d52e) != '\x01') {
          error(1,0,"failed to sort the costlookup array");
          return v2;
        }
        dat_49a30 = 1;
      }
      v3[0] = a0->field_0x0;
      v2 = (long)bsearch(v3,0x49020,0x3b,0x10,sub_d52e);
      if (v2) // branch-flip
        v5 = *(unsigned int *)((long)v2 + 8);
      else {
        v1 = a0->field_0x8;
        v2 = gettext("warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug");
        error(0,0,v2,v1);
        v5 = 10;
      }
    }
    else {
      v5 = a0->field_0x20;
    }
  }
  else {
    v5 = (a0->field_0x38) ? 7 : 8; // branch-flip
  }
  if (v4 >= v5)
    v5 = v4;
  v2 = (unsigned long)(unsigned long)v5;
  return v2;
}

// Function: sub_d7ac @ 0xd7ac
void sub_d7ac(void *a0) // return-dupe
{
  if (!a0)
    return;
  sub_d7ac(*(unsigned long *)((long)a0 + 0x118));
  sub_d7ac(*(unsigned long *)((long)a0 + 0x110));
  *(unsigned int *)((long)a0 + 0x20) = sub_d5a5(a0);
}

// Function: sub_d7ff @ 0xd7ff
unsigned long sub_d7ff(void)
{
  return dat_49a10;
}

// Function: sub_d810 @ 0xd810
unsigned int sub_d810(void *a0)
{
  unsigned int v1;
  
  if (a0) // branch-flip
    v1 = *(unsigned int *)((long)a0 + 0x24);
  else {
    v1 = dat_39684;
  }
  return v1;
}

// Function: sub_d838 @ 0xd838
void sub_d838(struct_50 *a0) // return-dupe x4
{
  unsigned int v1; // xmm0_da
  
  if (!a0)
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates"); // no-return
  if (a0->field_0x118)
    sub_d838(a0->field_0x118);
  if (a0->field_0x110)
    sub_d838(a0->field_0x110);
  if (a0->field_0x10 == 5)
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates"); // no-return
  if (a0->field_0x10 == 4)
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates"); // no-return
  switch(a0->field_0x10) {
    case 0:
      goto label_d958;
    case 1:
      if (a0->field_0x118)
        __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates"); // no-return
      if (!a0->field_0x110)
        return;
      __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates"); // no-return
    case 2:
      if (a0->field_0x0 != sub_9b30)
        __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates"); // no-return
      if (!a0->field_0x110) {
        *(float *)a0->field_0x24 = dat_39684 - *(float *)(a0->field_0x118 + 0x24);
        return;
      }
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates"); // no-return
    case 3:
      if (a0->field_0x0 != sub_8d9c) { // branch-flip
        if (a0->field_0x0 != sub_8fd0) {
          if (a0->field_0x0 != sub_9f9e)
            __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates"); // no-return
          sub_d810(a0->field_0x118);
          sub_d810(a0->field_0x110);
        }
      }
      else {
        sub_d810(a0->field_0x118);
        sub_d810(a0->field_0x110);
      }
      sub_d290();
      *(unsigned int *)a0->field_0x24 = v1;
      return;
    case 4:
    case 5:
      *(float *)a0->field_0x24 = dat_39684;
      return;
    default:
      __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates"); // no-return
    
  }
label_d958:
  if (a0->field_0x118)
    __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates"); // no-return
  if (!a0->field_0x110)
    return;
  __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates"); // no-return
}

// Function: sub_dc43 @ 0xdc43
void sub_dc43(void *a0,bool a1) // return-dupe
{
  if ((a1) && (*(int *)((long)a0 + 0x10) != 3))
    __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization"); // no-return
  if (*(long *)((long)a0 + 0x110)) {
    if (*(int *)(*(long *)((long)a0 + 0x110) + 0x10) != 3)
      __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization"); // no-return
    sub_dc43(*(unsigned long *)((long)a0 + 0x110),0);
  }
  if (!*(long *)((long)a0 + 0x118))
    return;
  sub_dc43(*(unsigned long *)((long)a0 + 0x118),0);
}

// Function: sub_dd20 @ 0xdd20
struct_58 * sub_dd20(int a0,long a1,int a2)
{
  char *v1;
  int v10; // stack - 0x54
  int *v11; // stack - 0x48
  int *v12; // stack - 0x40
  struct_58 *v13; // stack - 0x38
  char *v14; // stack - 0x30
  int *v15; // stack - 0x28
  long *v2;
  int *v3;
  char v4; // al
  int v5; // eax
  struct_58 *v6; // rax
  int v7; // stack - 0x5c
  int v8; // stack - 0x58
  long *v9; // stack - 0x50
  
  dat_49a08 = NULL;
  v6 = (long)(long)a2;
  v6 = (long)(a1 + (long)v6 * 8);
  dat_49a20 = (long)v6;
  v7 = a0;
  v8 = a2;
  while ((v8 < v7 && (v6 = (long)(long)v8, v6 = (unsigned long *)(a1 + (long)v6 * 8), v4 = sub_ff96(*(char **)v6,1), v4 != '\x01'))) {
    v6 = (long)(dat_49a28 + 1);
    dat_49a28 = (long)v6;
    v5 = v8 + 1;
    v8 = v5;
  }
  v6 = (int *)sub_1437a("(");
  v11 = (int *)v6;
  v6 = (int *)sub_1437a(")");
  v12 = (int *)v6;
  v6 = (struct_58 *)sub_1437a("print");
  v13 = v6;
  if (!v11)
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree"); // no-return
  if (!v12)
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree"); // no-return
  if (!v6)
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree"); // no-return
  sub_168b2(v11,a1,&v7);
  dat_49a18[1] = (long)"(";
  *(char *)((long)dat_49a08 + 0x29) = 1;
  sub_14262(a1,v7,dat_49a18,dat_49a08);
  sub_aec7(dat_49a18);
  while( true ) {
    if (v7 <= v8) {
      sub_14287(a1,v7,dat_49a18,dat_49a08);
      v2 = dat_49a08;
      if (dat_49a08[0x21]) { // branch-flip
        v4 = sub_ff58((void *)dat_49a08[0x21]);
        if (v4 != '\x01') { // branch-flip
          v9 = dat_49a08;
          dat_49a08 = (long *)dat_49a08[0x21];
          sub_aec7(dat_49a08);
          free(v9);
        }
        else {
          sub_14833(v12,a1,&v7);
          dat_49a18[1] = (long)")";
          *(char *)((long)dat_49a18 + 0x29) = 1;
          sub_aec7(dat_49a18);
          sub_16eb3(v13,a1,&v7);
          dat_49a18[1] = (long)"-print";
          *(char *)((long)dat_49a18 + 0x29) = 1;
          sub_aec7(dat_49a18);
          sub_aec7(dat_49a08);
        }
      }
      else {
        v9 = dat_49a08;
        dat_49a08 = (long *)dat_49a08[0x21];
        dat_49a18 = dat_49a08;
        free(v2);
        sub_16eb3(v13,a1,&v7);
        dat_49a18[1] = (long)"-print";
        sub_aec7(dat_49a18);
        sub_aec7(dat_49a08);
      }
      if (dat_49980 & 9) {
        fprintf(stderr,"Predicate List:\n");
        sub_aa62(stderr,dat_49a08);
      }
      sub_13f89(dat_49a08);
      sub_aec7(dat_49a08);
      v9 = dat_49a08;
      v6 = (unsigned long)sub_bc44(&v9,0,NULL);
      dat_49a10 = v6;
      sub_d838(v6);
      if (!v9) {
        if (dat_49980 & 9) {
          fprintf(stderr,"Eval Tree:\n");
          sub_ea9f(stderr,dat_49a10,0);
        }
        sub_d7ac(dat_49a10);
        sub_cc0f((long *)0x49a10);
        sub_dc43(dat_49a10,1);
        sub_cb9f(dat_49a10);
        sub_dc43(dat_49a10,1);
        if (dat_49980 & 9) {
          fprintf(stderr,"Optimized Eval Tree:\n");
          sub_ea9f(stderr,dat_49a10,0);
          fprintf(stderr,"Optimized command line:\n");
          sub_ac38(stderr,dat_49a10);
          fprintf(stderr,"\n");
        }
        v6 = (struct_58 *)dat_49a10;
        return v6;
      }
      if ((void *)*v9 == sub_8ee1) {
        v6 = (char *)gettext("you have too many \')\'");
        error(1,0,(char *)v6);
        return v6;
      }
      if (!v9[1]) {
        v6 = (char *)gettext("unexpected extra predicate");
        error(1,0,(char *)v6);
        return v6;
      }
      v1 = (char *)v9[1];
      v6 = (char *)gettext("unexpected extra predicate \'%s\'");
      error(1,0,(char *)v6,v1);
      return v6;
    }
    dat_499f8 = 0;
    v6 = (long)(long)v8;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v4 = sub_ff96(*(char **)v6,0);
    if (v4 != '\x01') {
      v6 = (long)(long)v8;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v1 = *(char **)v6;
      v6 = (char *)gettext("paths must precede expression: `%s\'");
      error(0,0,(char *)v6,v1);
      v6 = (long)(long)v8;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v5 = access(*(char **)v6,0);
      if (!v5) {
        v1 = (char *)dat_49a18[1];
        v6 = (char *)gettext("possible unquoted pattern after predicate `%s\'?");
        error(0,0,(char *)v6,v1);
      }
      exit(1); // no-return
    }
    v6 = (long)(long)v8;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v14 = *(char **)v6;
    v6 = (int *)sub_1437a(v14);
    v3 = (int *)v6;
    v15 = (int *)v6;
    if (!v6) {
      v6 = (char *)gettext("unknown predicate `%s\'");
      error(1,0,(char *)v6,v14);
      return v6;
    }
    if (*(int *)v6 != 4) {
      v5 = v8 + 1;
      v8 = v5;
    }
    v10 = v8;
    v4 = (**(void **)&v3[4])(v3,a1,&v8);
    if (v4 != '\x01') break;
    dat_49a18[1] = (long)v14;
    if (v10 != v8) { // branch-flip
      v6 = (long)(long)v10;
      dat_49a18[6] = *(long *)((long)v6 * 8 + a1);
    }
    else {
      dat_49a18[6] = 0;
    }
    sub_aec7(dat_49a18);
    sub_aec7(dat_49a08);
  }
  v6 = (long)(long)v8;
  v6 = (long *)(a1 + (long)v6 * 8);
  if (!*(long *)v6) {
    v6 = (char *)gettext("missing argument to `%s\'");
    error(1,0,(char *)v6,v14);
    return v6;
  }
  if ((*v15 == 4) && (v10 == v8)) {
    v6 = (char *)gettext("invalid predicate `%s\'");
    error(1,0,(char *)v6,v14);
    return v6;
  }
  v6 = (long)(long)v8;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v1 = *(char **)v6;
  v6 = (char *)gettext("invalid argument `%s\' to `%s\'");
  error(1,0,(char *)v6,v1,v14);
  return v6;
}

// Function: sub_e60c @ 0xe60c
void sub_e60c(void *a0)
{
  *(unsigned long *)((long)a0 + 0x128) = 0;
  *(unsigned long *)((long)a0 + 0x120) = *(unsigned long *)((long)a0 + 0x128);
}

// Function: sub_e644 @ 0xe644
long sub_e644(int *a0)
{
  long v1; // rax
  
  v1 = sub_e67d(a0);
  if (v1)
    *(unsigned long *)(v1 + 0x30) = 0;
  return v1;
}

// Function: sub_e67d @ 0xe67d
long sub_e67d(int *a0)
{
  long v1; // rax
  
  if (!*a0)
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred"); // no-return
  if (*a0 == 2)
    __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred"); // no-return
  v1 = sub_3205a(0x138);
  if (dat_49a08) // branch-flip
    *(long *)(dat_49a18 + 0x108) = v1;
  else {
    dat_49a08 = v1;
  }
  *(int **)(v1 + 0x130) = a0;
  *(unsigned int *)(v1 + 0x10) = 0;
  *(unsigned int *)(v1 + 0x14) = 0;
  *(char *)(v1 + 0x1a) = 1;
  *(char *)(v1 + 0x1b) = 1;
  *(unsigned int *)(v1 + 0x20) = 10;
  *(char **)(v1 + 0x30) = "ThisShouldBeSetToSomethingElse";
  *(char *)(v1 + 0x28) = dat_4994f;
  *(unsigned int *)(v1 + 0x24) = dat_39684;
  dat_49a18 = v1;
  sub_e60c(v1);
  return dat_49a18;
}

// Function: sub_e7dd @ 0xe7dd
long sub_e7dd(int *a0,unsigned long a1)
{
  unsigned int v1;
  long v2; // rax
  unsigned long *v3; // rax
  long v4; // rax
  
  if (!dat_49a40)
    dat_49a40 = sub_1437a("and");
  if (!dat_49a40)
    __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op"); // no-return
  if (!dat_49a18) goto label_e92c;
  v1 = *(unsigned int *)(dat_49a18 + 0x10);
  if (v1 != 5) {
    if (5 < v1) goto label_e92c;
    if (!v1) {
      error(1,0,gettext("oops -- invalid default insertion of and!"));
      return v2;
    }
    if (v1 != 1) goto label_e92c;
  }
  v3 = (unsigned long *)sub_e644(dat_49a40);
  *v3 = sub_8d9c;
  v3[1] = "-a";
  *(unsigned int *)&v3[2] = 3;
  *(unsigned int *)((long)v3 + 0x14) = 3;
  *(char *)((long)v3 + 0x1a) = 0;
  *(char *)((long)v3 + 0x1b) = 0;
  *(char *)((long)v3 + 0x1c) = 0;
  v3[6] = 0;
  v3[7] = 0;
  *(char *)&v3[3] = 0;
  *(char *)((long)v3 + 0x19) = 0;
label_e92c:
  v4 = sub_e67d(a0);
  *(unsigned long *)(v4 + 0x30) = a1;
  *(int **)(v4 + 0x130) = a0;
  return v4;
}

// Function: sub_e95d @ 0xe95d
char * sub_e95d(int a0)
{
  unsigned int v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (0xb <= v1)
      return "unknown";
    if (a0 == *(int *)((unsigned long)v1 * 0x10 + 300000)) break;
    v1 += 1;
  }
  return *(char **)((unsigned long)v1 * 0x10 + 0x493e8);
}

// Function: sub_e9bd @ 0xe9bd
unsigned long sub_e9bd(short a0)
{
  int v1; // stack - 0xc
  
  v1 = 0;
  while ((*(short *)((long)v1 * 0x10 + 0x49520) != -1 && (a0 != *(short *)((long)v1 * 0x10 + 0x49520)))) {
    v1 += 1;
  }
  return *(unsigned long *)((long)v1 * 0x10 + 0x49528);
}

// Function: sub_ea2e @ 0xea2e
unsigned long sub_ea2e(short a0)
{
  int v1; // stack - 0xc
  
  v1 = 0;
  while ((*(short *)((long)v1 * 0x10 + 0x494a0) != -1 && (a0 != *(short *)((long)v1 * 0x10 + 0x494a0)))) {
    v1 += 1;
  }
  return *(unsigned long *)((long)v1 * 0x10 + 0x494a8);
}

// Function: sub_ea9f @ 0xea9f
void sub_ea9f(FILE *a0,void *a1,int a2) // return-dupe x3, ternary x3
{
  float v1;
  unsigned int v2;
  char *v3; // rax
  bool v4;
  int v5; // stack - 0x20
  
  if (!a1)
    return;
  for (v5 = 0; v5 < a2; v5 = v5 + 1) {
    fprintf(a0,"    ");
  }
  fprintf(a0,"pred=[");
  sub_c2dc(a0,a1);
  v3 = (char *)sub_ea2e((int)(short)*(unsigned int *)((long)a1 + 0x14));
  v2 = *(unsigned int *)((long)a1 + 0x10);
  fprintf(a0,"] type=%s prec=%s",(char *)sub_e9bd((int)(short)v2),v3);
  v3 = (*(char *)((long)a1 + 0x18)) ? "" : "no "; // branch-flip
  v1 = *(float *)((long)a1 + 0x24);
  v2 = *(unsigned int *)((long)a1 + 0x20);
  fprintf(a0," cost=%s est_success_rate=%#.4g %sside effects ",(char *)sub_e95d(v2),(double)v1,v3);
  if (((*(char *)((long)a1 + 0x1a)) || (*(char *)((long)a1 + 0x1b))) || (*(char *)((long)a1 + 0x1c))) {
    fprintf(a0,"Needs ");
    v4 = *(char *)((long)a1 + 0x1a) != '\0';
    if (v4)
      fprintf(a0,"stat");
    if (*(char *)((long)a1 + 0x1c)) {
      v3 = (v4) ? "," : "";
      fprintf(a0,"%sinode",v3);
      v4 = 1;
    }
    if (*(char *)((long)a1 + 0x1b)) {
      v3 = (v4) ? "," : "";
      fprintf(a0,"%stype",v3);
    }
  }
  fprintf(a0,"\n");
  for (v5 = 0; v5 < a2; v5 = v5 + 1) {
    fprintf(a0,"    ");
  }
  if ((!*(long *)((long)a1 + 0x110)) && (!*(long *)((long)a1 + 0x118))) {
    fprintf(a0,"no children.\n");
    return;
  }
  if (*(long *)((long)a1 + 0x110)) { // branch-flip
    fprintf(a0,"left:\n");
    sub_ea9f(a0,*(unsigned long *)((long)a1 + 0x110),a2 + 1);
  }
  else {
    fprintf(a0,"no left.\n");
  }
  for (v5 = 0; v5 < a2; v5 = v5 + 1) {
    fprintf(a0,"    ");
  }
  if (*(long *)((long)a1 + 0x118)) {
    fprintf(a0,"right:\n");
    sub_ea9f(a0,*(unsigned long *)((long)a1 + 0x118),a2 + 1);
    return;
  }
  fprintf(a0,"no right.\n");
}

// Function: sub_ee5a @ 0xee5a
struct_56 * sub_ee5a(struct_57 *a0,unsigned long a1,unsigned long a2)
{
  struct_56 *v1; // rax
  
  v1 = (struct_56 *)sub_e7dd(a0,a2);
  v1->field_0x0 = a1;
  v1->field_0x8 = a0->field_0x8;
  v1->field_0x38 = 0;
  v1->field_0x10 = 1;
  v1->field_0x14 = 0;
  return v1;
}

// Function: sub_eecc @ 0xeecc
void sub_eecc(struct_58 *a0,unsigned long a1)
{
  if (!a0->field_0x18)
    __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary"); // no-return
  sub_ee5a(a0,a0->field_0x18,a1);
}

// Function: sub_ef32 @ 0xef32
void sub_ef32(struct_58 *a0)
{
  sub_eecc(a0,0);
}

// Function: sub_ef55 @ 0xef55
void sub_ef55(int a0) // return-dupe, ternary
{
  FILE *v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x20
  
  v1 = stdout;
  fputs(gettext("Valid arguments for -D:\n"),v1);
  if (a0) {
    for (v3 = 0; v3 <= 8; v3 = v3 + 1) {
      fprintf(stdout,"%-10s %s\n",*(char **)(v3 * 0x18 + 0x495a0),*(char **)(v3 * 0x18 + 0x495b0));
    }
    return;
  }
  for (v3 = 0; v3 <= 8; v3 = v3 + 1) {
    v2 = (v3) ? ", " : ""; // branch-flip
    fprintf(stdout,"%s%s",v2,*(char **)(v3 * 0x18 + 0x495a0));
  }
}

// Function: sub_f071 @ 0xf071
void sub_f071(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_4a230;
  if (a0) {
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
    exit(a0); // no-return
  }
  fprintf(stdout,gettext("Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n"),v2);
  v1 = stdout;
  fputs(gettext("\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n"),v1);
  v1 = stdout;
  fputs(gettext("\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n"),v1);
  v1 = stdout;
  fputs(gettext("\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n"),v1);
  v1 = stdout;
  fputs(gettext("\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n"),v1);
  v1 = stdout;
  fputs(gettext("\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n"),v1);
  v1 = stdout;
  fputs(gettext("\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n"),v1);
  v1 = stdout;
  fputs(gettext("\nOther common options:\n"),v1);
  v1 = stdout;
  fputs(gettext("      --help                   display this help and exit\n"),v1);
  v1 = stdout;
  fputs(gettext("      --version                output version information and exit\n\n"),v1);
  sub_ef55(0);
  v1 = stdout;
  fputs(gettext("\nUse \'-D help\' for a description of the options, or see find(1)\n\n"),v1);
  sub_1d008(stdout,dat_4a230);
  exit(0); // no-return
}

// Function: sub_f26e @ 0xf26e
void sub_f26e(void)
{
  return;
}

// Function: sub_f27d @ 0xf27d
unsigned long sub_f27d(char *a0,char *a1,struct_1 *a2) // return-dupe
{
  int v1; // eax
  char *v2; // rax
  
  if (dat_499c4 != '\x01') {
    sub_f26e(a2);
    v1 = (*dat_49990)(a1,a2);
    if (v1) {
      if ((dat_4994e == '\x01') && (*__errno_location() == 2))
        return 0xffffffff;
      sub_10a13(*__errno_location(),a0);
      return 0xffffffff;
    }
    if (!a2->field_0x18) {
      v2 = (char *)sub_2f861(0,dat_499ac,a1);
      error(0,0,gettext("WARNING: file %s appears to have mode 0000"),v2);
      dat_499e4 = 1;
    }
  }
  dat_499c8 = a2->field_0x18;
  dat_499c5 = 1;
  dat_499c4 = 1;
  return 0;
}

// Function: sub_f388 @ 0xf388
unsigned long sub_f388(char *a0,void *a1,struct_2 *a2) // return-dupe
{
  bool v1;
  int v2; // eax
  
  v1 = 0;
  if ((a2->field_0x1a) && (dat_499c4 != '\x01'))
    v1 = 1;
  else if ((a2->field_0x1b) && (dat_499c5 != '\x01'))
    v1 = 1;
  else if (a2->field_0x1c) {
    if (*(long *)((long)a1 + 8)) { // branch-flip
      if ((dat_499c5 != '\x01') || ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x4000))
        v1 = 1;
    }
    else {
      v1 = 1;
    }
  }
  if ((v1) && (v2 = sub_f27d(a0,dat_499d0,a1), v2))
    return 0xffffffff;
  return 0;
}

// Function: sub_f45a @ 0xf45a
unsigned long sub_f45a(void) // return-dupe
{
  int v1; // eax
  char v2 [130];
  char v3 [262];
  float v4; // xmm0_da
  
  if (uname(v2))
    return 1;
  v4 = (float)(double)(*dat_48fb8)(v3);
  v1 = strcmp("Linux",v2);
  if (v1) {
    v1 = strcmp("FreeBSD",v2);
    if (v1)
      return 1;
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),(float)s_3a453._1_4_ <= v4);
  }
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),dat_3a450 <= v4);
}

// Function: sub_f540 @ 0xf540
unsigned long sub_f540(long a0)
{
  sub_19f5a((void *)(a0 + 8),(struct_14 *)(a0 + 0x60));
  return 0;
}

// Function: sub_f57a @ 0xf57a
void sub_f57a(struct_22 *a0) // return-dupe
{
  sub_1a9dd(a0->field_0xc0,sub_f540,a0);
  if (a0->field_0xc0 == dat_49a00)
    return;
  sub_2fe52(a0->field_0xc0);
  free(a0->field_0xc0);
  a0->field_0xc0 = NULL;
}

// Function: sub_f5fa @ 0xf5fa
void sub_f5fa(struct_23 *a0) // return-dupe
{
  if (!a0)
    return;
  if (dat_499e8) {
    sub_f5fa(a0->field_0x110);
    if ((((a0->field_0x0 == sub_9497) || (a0->field_0x0 == sub_9f27)) && (a0->field_0x38)) && (a0->field_0xd0))
      sub_f57a(&a0->field_0x38);
    sub_f5fa(a0->field_0x118);
    return;
  }
  __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs"); // no-return
}

// Function: sub_f6cb @ 0xf6cb
void sub_f6cb(void) // return-dupe
{
  if (!dat_499e8)
    return;
  sub_f5fa((struct_23 *)sub_d7ff());
  dat_499e8 = 0;
}

// Function: sub_f6f5 @ 0xf6f5
void sub_f6f5(struct_23 *a0) // return-dupe
{
  if (!a0)
    return;
  sub_f6f5(a0->field_0x110);
  if (((a0->field_0x0 == sub_9466) && (a0->field_0x38)) && (a0->field_0xd0))
    sub_19f5a(&a0->field_0x39[7],(struct_14 *)&a0->field_0x39[0x5f]);
  sub_f6f5(a0->field_0x118);
}

// Function: sub_f78b @ 0xf78b
void sub_f78b(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rbx
  unsigned long v4;
  
  dat_49a00 = (int *)sub_31bc2(0x10);
  v1 = sub_2fd99(dat_49a00);
  if (v1) {
    if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
      v4 = 0x398cd;
    else {
      v4 = *(unsigned long *)&dat_49a00[2];
    }
    if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
      v3 = "";
    else {
      v3 = ": ";
    }
    v2 = gettext("Failed to save initial working directory%s%s");
    error(1,*__errno_location(),v2,v3,v4);
    return;
  }
}

// Function: sub_f861 @ 0xf861
void sub_f861(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rbx
  unsigned long v4;
  
  v1 = sub_2fe17(dat_49a00);
  if (!v1) {
    sub_2fe52(dat_49a00);
    free(dat_49a00);
    dat_49a00 = NULL;
    return;
  }
  if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
    v4 = 0x398cd;
  else {
    v4 = *(unsigned long *)&dat_49a00[2];
  }
  if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
    v3 = "";
  else {
    v3 = ": ";
  }
  v2 = gettext("Failed to restore initial working directory%s%s");
  error(0,*__errno_location(),v2,v3,v4);
  _exit(1); // no-return
}

// Function: sub_f959 @ 0xf959
void sub_f959(void *a0,void *a1) // return-dupe
{
  if (*(long *)((long)a0 + 0x110))
    sub_f959(*(unsigned long *)((long)a0 + 0x110),a1);
  (*a1)(a0);
  if (!*(long *)((long)a0 + 0x118))
    return;
  sub_f959(*(unsigned long *)((long)a0 + 0x118),a1);
}

// Function: sub_f9d1 @ 0xf9d1
void sub_f9d1(struct_61 *a0) // return-dupe
{
  if ((((a0->field_0x0 != sub_954f) && (a0->field_0x0 != sub_1376d)) && (a0->field_0x0 != sub_94e6)) && (a0->field_0x0 != sub_959f))
    return;
  a0->field_0x40 = 0;
}

// Function: sub_fa38 @ 0xfa38
void sub_fa38(void) // return-dupe
{
  int v1; // eax
  void *v2; // rax
  
  v2 = (void *)sub_d7ff();
  if (v2) {
    sub_f959(v2,sub_f6f5);
    sub_f6cb();
  }
  sub_10cac(dat_499f0);
  if (v2)
    sub_f959(v2,sub_f9d1);
  sub_f861();
  if (sub_1b5f4()) {
    sub_1b61e();
    sub_1b5c4();
  }
  v1 = sub_1e1a8(stdout);
  if (v1 != -1)
    return;
  sub_10a77(*__errno_location(),"standard output");
}

// Function: sub_fae2 @ 0xfae2
int * sub_fae2(char *a0,stat *a1,unsigned int a2)
{
  int v1; // eax
  int *v2; // rax
  
  v2 = __errno_location();
  if ((*v2 != 2) && (*v2 != 0x14))
    v2 = (unsigned long)(unsigned long)a2;
  else {
    if (dat_49980 & 2)
      fprintf(stderr,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
    v1 = fstatat(dat_499d8,a0,a1,0x100);
  }
  return v2;
}

// Function: sub_fb5c @ 0xfb5c
unsigned long sub_fb5c(char *a0,stat *a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_499d8 != -100) && (dat_499d8 <= -1))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat"); // no-return
  sub_f26e(a1);
  if (dat_499c0) // branch-flip
    v1 = fstatat(dat_499d8,a0,a1,0x100);
  else {
    v1 = fstatat(dat_499d8,a0,a1,0);
    if (!v1) {
      v2 = 0;
      return v2;
    }
    v2 = sub_fae2(a0,a1,v1);
  }
  return v2;
}

// Function: sub_fc21 @ 0xfc21
unsigned long sub_fc21(char *a0,stat *a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_499d8 != -100) && (dat_499d8 <= -1))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat"); // no-return
  sub_f26e(a1);
  v1 = fstatat(dat_499d8,a0,a1,0);
  if (!v1)
    return 0;
  v2 = sub_fae2(a0,a1,v1);
  return v2;
}

// Function: sub_fcc0 @ 0xfcc0
void sub_fcc0(char *a0,stat *a1)
{
  if ((dat_499d8 <= -1) && (dat_499d8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  sub_f26e(a1);
  fstatat(dat_499d8,a0,a1,0x100);
}

// Function: sub_fd39 @ 0xfd39
void sub_fd39(char *a0,stat *a1) // return-dupe x2
{
  dat_49a48 += 1;
  fprintf(stderr,"debug_stat (%s)\n",a0);
  if (dat_49988 == 2) {
    sub_fb5c(a0,a1);
    return;
  }
  if (dat_49988 <= 2) {
    if (!dat_49988) {
      sub_fcc0(a0,a1);
      return;
    }
    if (dat_49988 == 1) {
      sub_fc21(a0,a1);
      return;
    }
  }
  __assert_fail("0","util.c",0x2a5,"debug_stat"); // no-return
}

// Function: sub_fe03 @ 0xfe03
unsigned int sub_fe03(void) // early-return x2
{
  if (dat_49988 == 1)
    return 1;
  if (dat_49988 != 2)
    return 0;
  return CONCAT31((undefined3)((unsigned int)dat_499c0 >> 8),dat_499c0 == 0);
}

// Function: sub_fe38 @ 0xfe38
unsigned long sub_fe38(unsigned int *a0,char *a1,char *a2,struct_1 *a3,bool a4) // return-dupe x3
{
  if (*a0) {
    if (((*a0 & 0xf000) == 0xa000) && (sub_fe03())) {
      if (!sub_f27d(a1,a2,a3)) {
        dat_499c8 = a3->field_0x18;
        *a0 = dat_499c8;
        dat_499c5 = 1;
        return 1;
      }
      return 0;
    }
    dat_499c8 = *a0;
    a3->field_0x18 = dat_499c8;
    dat_499c5 = 1;
    return 1;
  }
  if (a4) {
    dat_499c4 = 0;
    dat_499c5 = 0;
    dat_499c8 = 0;
    return 1;
  }
  if (!sub_f27d(a1,a2,a3)) {
    *a0 = a3->field_0x18;
    dat_499c5 = 1;
    dat_499c8 = *a0;
    return 1;
  }
  return 0;
}

// Function: sub_ff58 @ 0xff58
unsigned long sub_ff58(void *a0)
{
  void *v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (!v1)
      return 1;
    if (*(char *)((long)v1 + 0x19)) break;
    v1 = *(void **)((long)v1 + 0x108);
  }
  return 0;
}

// Function: sub_ff96 @ 0xff96
bool sub_ff96(char *a0,bool a1) // early-return x6
{
  switch(*a0) {
    case 0x21:
    case 0x28:
      goto label_10027;
    default:
      return 0;
    case 0x29:
    case 0x2c:
      if (!a0[1])
        return a1 == 0;
      return 0;
    case 0x2d:
      if (!a0[1])
        return 0;
      return 1;
    
  }
label_10027:
  if (!a0[1])
    return 1;
  return 0;
}

// Function: sub_1004b @ 0x1004b
void sub_1004b(char *a0)
{
  bool v1;
  char *v2;
  char *v3; // stack - 0x40
  char v4 [2]; // stack - 0x22
  char *v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  
  v3 = NULL;
  v4[0] = ',';
  v4[1] = '\0';
  v1 = 1;
  v5 = strtok_r(a0,v4,&v3);
  do {
    if (!v5) {
      if (v1) {
        error(0,0,gettext("Empty argument to the -D option."));
        sub_f071(1); // no-return
      }
      if (!(dat_49980 & 0x10))
        return;
      sub_ef55(1);
      exit(0); // no-return
    }
    v1 = 0;
    for (v6 = 0; v6 <= 8; v6 = v6 + 1) {
      v2 = *(char **)(v6 * 0x18 + 0x495a0);
      if (!strcmp(v2,v5)) {
        dat_49980 = (long)*(int *)(v6 * 0x18 + 0x495a8) | dat_49980;
        break;
      }
    }
    if (9 <= v6) {
      v2 = (char *)sub_2f861(0,dat_499ac,a0);
      error(0,0,gettext("Ignoring unrecognised debug flag %s"),v2);
    }
    v5 = strtok_r(NULL,v4,&v3);
  } while( true );
}

// Function: sub_101fd @ 0x101fd
void sub_101fd(char *a0)
{
  int v1;
  char *v2; // rax
  char *v3; // stack - 0x30
  unsigned long v4; // stack - 0x28
  
  if (!*a0) {
    error(1,0,gettext("The -O option must be immediately followed by a decimal integer"));
    return;
  }
  if (!(*(unsigned short *)((unsigned long)(unsigned char)*a0 * 2 + *(long *)__ctype_b_loc()) & 0x800)) {
    error(1,0,gettext("Please specify a decimal number immediately after -O"));
    return;
  }
  v1 = *__errno_location();
  *__errno_location() = 0;
  v4 = strtoul(a0,&v3,10);
  if ((!v4) && (a0 == v3)) {
    error(1,0,gettext("Please specify a decimal number immediately after -O"));
    return;
  }
  if (*v3) {
    error(1,0,gettext("Invalid optimisation level %s"),a0);
    return;
  }
  if ((v4 == 0xffffffffffffffff) && (*__errno_location())) {
    v2 = gettext("Invalid optimisation level %s");
    error(1,*__errno_location(),v2,a0);
    return;
  }
  if (v4 <= 0xffff) {
    dat_499a8 = (unsigned short)v4;
    *__errno_location() = v1;
    return;
  }
  error(1,0,gettext("Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate."),v4);
}

// Function: sub_103e8 @ 0x103e8
int sub_103e8(int a0,long a1)
{
  char *v1;
  int v2; // stack - 0x10
  
  v2 = 1;
  do {
    if (a0 <= v2)
      return v2;
    v1 = *(char **)(a1 + (long)v2 * 8);
    if (strcmp("-H",v1)) { // branch-flip
      v1 = *(char **)(a1 + (long)v2 * 8);
      if (strcmp("-L",v1)) { // branch-flip
        v1 = *(char **)(a1 + (long)v2 * 8);
        if (strcmp("-P",v1)) { // branch-flip
          v1 = *(char **)(a1 + (long)v2 * 8);
          if (!strcmp("--",v1))
            return v2 + 1;
          v1 = *(char **)(a1 + (long)v2 * 8);
          if (strcmp("-D",v1)) { // branch-flip
            v1 = *(char **)(a1 + (long)v2 * 8);
            if (strncmp("-O",v1,2))
              return v2;
            sub_101fd((char *)(*(long *)(a1 + (long)v2 * 8) + 2));
          }
          else {
            if (a0 <= v2 + 1) {
              error(0,0,gettext("Missing argument after the -D option."));
              sub_f071(1); // no-return
            }
            sub_1004b(*(char **)(a1 + ((long)v2 + 1) * 8));
            v2 += 1;
          }
        }
        else {
          sub_141b0(0);
        }
      }
      else {
        sub_141b0(1);
      }
    }
    else {
      sub_141b0(2);
    }
    v2 += 1;
  } while( true );
}

// Function: sub_10601 @ 0x10601
undefined16 sub_10601(void)
{
  long v1;
  char v2 [16];
  timeval v3; // stack - 0x28
  
  if (gettimeofday(&v3,NULL)) { // branch-flip
    v3._0_8_ = time(0);
    if (v3._0_8_ == -1)
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now"); // no-return
    v1 = 0;
  }
  else {
    v1 = v3._8_8_ * 1000;
  }
  v2._8_8_ = v1;
  v2._0_8_ = v3._0_8_;
  return v2._0_16_;
}

// Function: sub_106bd @ 0x106bd
void sub_106bd(struct_64 *a0)
{
  char v1 [16];
  
  if (getenv("POSIXLY_CORRECT")) // branch-flip
    a0->field_0x11 = 1;
  else {
    a0->field_0x11 = 0;
  }
  a0->field_0x58 = sub_f45a();
  a0->field_0x5c = 0;
  if (isatty(0)) { // branch-flip
    a0->field_0x10 = 1;
    a0->field_0xf = 0;
  }
  else {
    a0->field_0x10 = 0;
    a0->field_0xf = 0;
  }
  if (a0->field_0x11)
    a0->field_0x10 = 0;
  a0->field_0x0 = 1;
  a0->field_0x1 = 0;
  a0->field_0x8 = 0xffffffff;
  a0->field_0x4 = a0->field_0x8;
  v1 = sub_10601();
  a0->field_0x18 = SUB168(v1,0);
  a0->field_0x20 = SUB168(v1,8);
  a0->field_0x28 = a0->field_0x18 + -0x15180;
  a0->field_0x30 = a0->field_0x20;
  a0->field_0x38 = 0;
  a0->field_0xd = 0;
  a0->field_0xe = 0;
  if (a0->field_0x11) // branch-flip
    a0->field_0x3c = 0x200;
  else {
    a0->field_0x3c = 0x400;
  }
  a0->field_0x40 = 0;
  a0->field_0x68 = 2;
  if (getenv("FIND_BLOCK_SIZE")) {
    error(1,0,gettext("The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable"));
    return;
  }
  a0->field_0xc = 0;
  sub_141b0(0);
  a0->field_0x6c = 8;
  a0->field_0x70 = 0;
  a0->field_0x78 = 0;
}

// Function: sub_10873 @ 0x10873
unsigned long sub_10873(char *a0,void *a1,struct_2 *a2) // return-dupe x2
{
  void *v1;
  
  a2->field_0x120 = a2->field_0x120 + 1;
  if ((((a2->field_0x1a) || (a2->field_0x1b)) || (a2->field_0x1c)) && (sub_f388(a0,a1,a2)))
    return 0;
  v1 = a2->field_0x0;
  if ((*v1)(a0,a1,a2)) {
    a2->field_0x128 = a2->field_0x128 + 1;
    return 1;
  }
  return 0;
}

// Function: sub_10933 @ 0x10933
unsigned int sub_10933(void *a0) // return-dupe
{
  if ((a0 != sub_9497) && (a0 != sub_9f27))
    return 0;
  return 1;
}

// Function: sub_1096a @ 0x1096a
void sub_1096a(unsigned int a0,char *a1)
{
  sub_2f861(a0,dat_499ac,a1);
}

// Function: sub_10995 @ 0x10995
void sub_10995(int a0,int a1,char a2,char *a3) // return-dupe
{
  if ((a2 != '\x01') || (dat_499f8 != '\x01')) {
    error(a0,a1,"%s",(char *)sub_1096a(0,a3));
    dat_499e4 = 1;
  }
  if (!a2)
    return;
  dat_499f8 = 1;
}

// Function: sub_10a13 @ 0x10a13
void sub_10a13(unsigned int a0,char *a1)
{
  sub_10995(0,a0,1,a1);
}

// Function: sub_10a44 @ 0x10a44
void sub_10a44(unsigned int a0,char *a1)
{
  sub_10995(1,a0,1,a1);
  abort(); // no-return
}

// Function: sub_10a77 @ 0x10a77
void sub_10a77(unsigned int a0,char *a1)
{
  sub_10995(0,a0,0,a1);
}

// Function: sub_10aa8 @ 0x10aa8
void sub_10aa8(unsigned int a0,char *a1)
{
  dat_499f8 = 0;
  sub_10995(1,a0,0,a1);
  abort(); // no-return
}

// Function: sub_10ae2 @ 0x10ae2
unsigned int sub_10ae2(struct_24 *a0,struct_24 *a1) // return-dupe
{
  if ((a0->field_0x8 == a1->field_0x8) && (a0->field_0x0 == a1->field_0x0))
    return 1;
  return 0;
}

// Function: sub_10b3b @ 0x10b3b
void sub_10b3b(void *a0)
{
  FILE *v1;
  char *v2;
  
  if (*(long *)((long)a0 + 0x18)) {
    v1 = *(FILE **)((long)a0 + 0x18);
    if (sub_1dad5(v1)) {
      v2 = *(char **)((long)a0 + 0x10);
      sub_10aa8(*__errno_location(),v2); // no-return
    }
  }
  free(*(void **)((long)a0 + 0x10));
  free(a0);
}

// Function: sub_10bb1 @ 0x10bb1
unsigned long sub_10bb1(struct_8 *a0,unsigned long a1)
{
  return (a0->field_0x8 ^ a0->field_0x0) % a1;
}

// Function: sub_10be9 @ 0x10be9
struct_24 * sub_10be9(char *a0) // return-dupe x2
{
  struct_24 *v1; // rax
  
  v1 = malloc(0x10);
  if (!v1)
    return NULL;
  v1->field_0x0 = (long)strdup(a0);
  if (v1->field_0x0) {
    v1->field_0x8 = sub_21bb1(0xb,NULL,sub_10bb1,sub_10ae2,sub_10b3b);
    if (!v1->field_0x8) {
      free((void *)v1->field_0x0);
      free(v1);
      return NULL;
    }
    return v1;
  }
  free(v1);
  return NULL;
}

// Function: sub_10cac @ 0x10cac
void sub_10cac(struct_62 *a0)
{
  free(a0->field_0x0);
  sub_21e31(a0->field_0x8);
}

// Function: sub_10ce6 @ 0x10ce6
unsigned long sub_10ce6(struct_59 *a0,char *a1) // early-return x5
{
  char *v1;
  struct_32 *v2;
  int v3;
  unsigned long *v4; // rax
  long v5; // rax
  unsigned long v6;
  stat v7; // stack - 0xa8
  
  v4 = malloc(0x20);
  if (!v4)
    return 0;
  v4[2] = strdup(a1);
  if (!v4[2]) {
    free(v4);
    return 0;
  }
  v1 = a0->field_0x0;
  v4[3] = sub_1e49e(a1,v1);
  if (!v4[3]) {
    sub_10b3b(v4);
    return 0;
  }
  v3 = fileno((FILE *)v4[3]);
  if (0 <= v3) {
    sub_1d5ba(v3,1);
    if (fstat(v3,&v7) <= -1) {
      sub_10b3b(v4);
      return 0;
    }
    *v4 = v7._0_8_;
    v4[1] = v7._8_8_;
    v5 = sub_214fc(a0->field_0x8,v4);
    if (v5) { // branch-flip
      sub_10b3b(v4);
      v6 = *(unsigned long *)(v5 + 0x18);
    }
    else {
      v2 = a0->field_0x8;
      if (!sub_228f2(v2,v4)) {
        v3 = *__errno_location();
        sub_10b3b(v4);
        *__errno_location() = v3;
        return 0;
      }
      v6 = v4[3];
    }
    return v6;
  }
  __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen"); // no-return
}

// Function: sub_10f4b @ 0x10f4b
long sub_10f4b(long *a0,char *a1,int a2,int a3,char a4,char a5,void *a6)
{
  long v1;
  char *v2;
  unsigned int v3; // stack - 0x24
  char *v4; // stack - 0x20
  
  v3 = 0;
  if (a4 == '{')
    __assert_fail("format_char != \'{\'","print.c",0x4e,"make_segment"); // no-return
  if (a4 == '[')
    __assert_fail("format_char != \'[\'","print.c",0x4f,"make_segment"); // no-return
  if (a4 == '(')
    __assert_fail("format_char != \'(\'","print.c",0x50,"make_segment"); // no-return
  *a0 = sub_31bc2(0x20);
  *(int *)*a0 = a3;
  *(char *)(*a0 + 4) = a4;
  *(char *)(*a0 + 5) = a5;
  *(unsigned long *)(*a0 + 0x18) = 0;
  *(int *)(*a0 + 0x10) = a2;
  v1 = *a0;
  *(unsigned long *)(v1 + 8) = sub_31bc2((long)a2 + 2);
  v2 = *(char **)(v1 + 8);
  strncpy(v2,a1,(long)a2);
  v2 = &v2[a2];
  if ((a3) && (a3 != 1)) {
    if (a3 != 2)
      __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment"); // no-return
    v4 = v2;
    switch(a4) {
      case 0x25:
        v4 = &v2[1];
        *v2 = '%';
        break;
      case 0x41:
      case 0x42:
      case 0x43:
      case 0x46:
      case 0x4d:
      case 0x54:
      case 0x61:
      case 99:
      case 0x67:
      case 0x73:
      case 0x74:
      case 0x75:
        *(char *)((long)a6 + 0x1a) = 1;
        v3 = 3;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x44:
      case 0x47:
      case 0x55:
      case 0x62:
      case 0x6b:
      case 0x6e:
        *(char *)((long)a6 + 0x1a) = 1;
        v3 = 3;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x48:
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x50:
      case 0x66:
      case 0x68:
      case 0x70:
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x53:
        *(char *)((long)a6 + 0x1a) = 1;
        v3 = 3;
        v4 = &v2[1];
        *v2 = 'g';
        break;
      case 0x59:
        *(char *)((long)a6 + 0x1a) = 1;
        v3 = 2;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x5a:
        v3 = 5;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 100:
        v4 = &v2[1];
        *v2 = 'd';
        break;
      case 0x69:
        *(char *)((long)a6 + 0x1c) = 1;
        v3 = 1;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x6c:
        *(char *)((long)a6 + 0x1a) = 1;
        v3 = 4;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x6d:
        v4 = &v2[1];
        *v2 = 'o';
        *(char *)((long)a6 + 0x1a) = 1;
        v3 = 3;
        break;
      case 0x79:
        *(char *)((long)a6 + 0x1b) = 1;
        v3 = 2;
        v4 = &v2[1];
        *v2 = 's';
      
    }
    *v4 = '\0';
    if (*(unsigned int *)((long)a6 + 0x20) < v3)
      *(unsigned int *)((long)a6 + 0x20) = v3;
    v1 = *a0;
  }
  else {
    if (a4)
      __assert_fail("0 == format_char","print.c",0x61,"make_segment"); // no-return
    if (a5)
      __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment"); // no-return
    *v2 = '\0';
    v1 = *a0;
  }
  return v1 + 0x18;
}

// Function: sub_1133c @ 0x1133c
unsigned int sub_1133c(char a0) // return-dupe
{
  if (('0' <= a0) && (a0 <= '7'))
    return 1;
  return 0;
}

// Function: sub_11366 @ 0x11366
int sub_11366(char *a0,long *a1)
{
  char v1;
  int v2;
  int v3; // r12d
  long v4; // stack - 0x20
  
  v4 = 0;
  v2 = 0;
  for (v3 = 0; v3 <= 2; v3 = v3 + 1) {
    v1 = a0[v4];
    if (!sub_1133c((int)v1)) break;
    v2 = (int)a0[v4] + v2 * 8 + -0x30;
    v4 += 1;
  }
  *a1 = v4 + -1;
  return v2;
}

// Function: sub_113f2 @ 0x113f2
char sub_113f2(char a0) // return-dupe x8
{
  switch(a0) {
    case 0x5c:
      return 0x5c;
    default:
      return 0;
    case 0x61:
      return 7;
    case 0x62:
      return 8;
    case 0x66:
      return 0xc;
    case 0x6e:
      return 10;
    case 0x72:
      return 0xd;
    case 0x74:
      return 9;
    case 0x76:
      return 0xb;
    
  }
}

// Function: sub_11467 @ 0x11467
long sub_11467(char *a0)
{
  char v1;
  long v2; // stack - 0x10
  
  v2 = 0;
  do {
    v2 += 1;
    if (!a0[v2]) break;
    v1 = a0[v2];
  } while (strchr("-+ #",(int)v1));
  while (*(unsigned short *)((unsigned long)(unsigned char)a0[v2] * 2 + *(long *)__ctype_b_loc()) & 0x800) {
    v2 += 1;
  }
  if (a0[v2] == '.') {
    do {
      v2 += 1;
    } while (*(unsigned short *)((unsigned long)(unsigned char)a0[v2] * 2 + *(long *)__ctype_b_loc()) & 0x800);
  }
  return v2;
}

// Function: sub_11545 @ 0x11545
unsigned long sub_11545(char a0) // return-dupe x2
{
  if (strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)a0))
    return 1;
  if (strchr("ABCT",(int)a0))
    return 2;
  return 0;
}

// Function: sub_1159f @ 0x1159f
unsigned long sub_1159f(unsigned long *a0,struct_57 *a1,char *a2) // return-dupe, ternary
{
  char v1;
  char *v10; // stack - 0x50
  char *v11; // stack - 0x48
  long *v12; // stack - 0x40
  long v13; // stack - 0x38
  long v14; // stack - 0x30
  void *v15; // stack - 0x28
  unsigned long v2;
  int v3;
  int v4;
  char v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  char *v8; // rdx
  long v9; // stack - 0x58
  
  v10 = a2;
  v15 = (void *)sub_ee5a(a1,sub_1376d,a2);
  *(char *)((long)v15 + 0x19) = 1;
  *(char *)((long)v15 + 0x18) = *(char *)((long)v15 + 0x19);
  v2 = a0[1];
  *(unsigned long *)((long)v15 + 0x38) = *a0;
  *(unsigned long *)((long)v15 + 0x40) = v2;
  v2 = a0[3];
  *(unsigned long *)((long)v15 + 0x48) = a0[2];
  *(unsigned long *)((long)v15 + 0x50) = v2;
  *(unsigned long *)((long)v15 + 0x58) = a0[4];
  *(char *)((long)v15 + 0x1b) = 0;
  *(char *)((long)v15 + 0x1a) = 0;
  *(unsigned int *)((long)v15 + 0x20) = 0;
  v12 = (long *)((long)v15 + 0x38);
  *v12 = 0;
  v11 = v10;
  do {
    if (!*v11) {
      if (v11 <= v10)
        return 1;
      sub_10f4b(v12,v10,(int)v11 - (int)v10,0,0,0,v15);
      return 1;
    }
    if ((*v11 == '\\') && (v11[1] == 'c')) {
      sub_10f4b(v12,v10,(int)v11 - (int)v10,1,0,0,v15);
      if (!*(char *)((long)v15 + 0x1a))
        return 1;
      if (3 <= *(unsigned int *)((long)v15 + 0x20))
        return 1;
      *(unsigned int *)((long)v15 + 0x20) = 3;
      return 1;
    }
    if (*v11 != '\\') { // branch-flip
      if (*v11 == '%') {
        if (!v11[1]) {
          error(1,0,gettext("error: %s at end of format string"),v11);
          return v6;
        }
        v14 = (v11[1] != '%') ? sub_11467(v11) : 1; // branch-flip
        v11 = &v11[v14];
        v14 = sub_11545((int)*v11);
        if ((v14) && (v11[v14 + -1])) {
          if (v14 != 2) // branch-flip
            v5 = '\0';
          else {
            v5 = v11[1];
          }
          v1 = *v11;
          v4 = (int)v10;
          v3 = (int)v11;
          v12 = (long *)sub_10f4b(v12,v10,v3 - v4,2,(int)v1,(int)v5,v15);
          v11 = &v11[v14 + -1];
        }
        else {
          v5 = *v11;
          if (strchr("{[(",(int)v5)) {
            v5 = *v11;
            error(1,0,gettext("error: the format directive `%%%c\' is reserved for future use"),v5);
            return v7;
          }
          if ((v14 != 2) || (v11[1])) {
            v5 = *v11;
            error(0,0,gettext("warning: unrecognized format directive `%%%c\'"),v5);
          }
          else {
            v5 = *v11;
            error(0,0,gettext("warning: format directive `%%%c\' should be followed by another character"),v5);
          }
          v3 = (int)v11;
          v4 = (int)v10;
          v12 = (long *)sub_10f4b(v12,v10,(v3 + 1) - v4,0,0,0,v15);
        }
        v10 = &v11[1];
      }
    }
    else {
      v13 = 1;
      if (v11[1]) { // branch-flip
        v5 = v11[1];
        if (sub_1133c((int)v5)) { // branch-flip
          v9 = 0;
          v8 = &v11[v13];
          *v11 = sub_11366(v8,&v9);
          v13 += v9;
        }
        else {
          v5 = sub_113f2((int)v11[v13]);
          if (!v5) {
            v5 = v11[v13];
            error(0,0,gettext("warning: unrecognized escape `\\%c\'"),v5);
            v11 = &v11[v13];
            goto label_11aa1;
          }
          *v11 = v5;
        }
      }
      else {
        error(0,0,gettext("warning: escape `\\\' followed by nothing at all"));
        v13 -= 1;
      }
      v4 = (int)v10;
      v3 = (int)v11;
      v12 = (long *)sub_10f4b(v12,v10,(v3 - v4) + 1,0,0,0,v15);
      v10 = &v11[v13 + 1];
      v11 = &v11[v13];
    }
label_11aa1:
    v11 = &v11[1];
  } while( true );
}

// Function: sub_11b14 @ 0x11b14
unsigned long sub_11b14(char *a0,char *a1,long *a2,long *a3) // return-dupe
{
  bool v1;
  long v2; // stack - 0x10
  
  v1 = 0;
  for (v2 = 0; (a0[v2] && (a1[v2])); v2 = v2 + 1) {
    if (a0[v2] != a1[v2]) {
      if ((!(*(unsigned short *)((unsigned long)(unsigned char)a0[v2] * 2 + *(long *)__ctype_b_loc()) & 0x800)) || (!(*(unsigned short *)((unsigned long)(unsigned char)a1[v2] * 2 + *(long *)__ctype_b_loc()) & 0x800)))
        return 0;
      if (v1) {
        if (v2 - *a2 != *a3)
          return 0;
        *a3 = *a3 + 1;
      }
      else {
        *a2 = v2;
        *a3 = 1;
        v1 = 1;
      }
    }
  }
  if ((!a0[v2]) && (!a1[v2]))
    return 1;
  return 0;
}

// Function: sub_11c89 @ 0x11c89
long sub_11c89(char *a0,tm *a1,void *a2,unsigned long a3) // ternary
{
  char v1; // al
  unsigned long v10; // stack - 0x60
  tm v2; // stack - 0x58
  long v3; // stack - 0x90
  long v4; // stack - 0x98
  char *v5; // stack - 0x88
  unsigned long v6; // stack - 0x80
  unsigned long v7; // stack - 0x78
  char *v8; // stack - 0x70
  long v9; // stack - 0x68
  
  v5 = NULL;
  v5 = (char *)sub_31bc2(strlen(a0) + 2);
  *v5 = '_';
  memcpy(&v5[1],a0,strlen(a0) + 1);
  v2._8_8_ = *(unsigned long *)&a1->field_0x8;
  v2._16_8_ = *(unsigned long *)&a1->field_0x10;
  v2._24_8_ = *(unsigned long *)&a1->field_0x18;
  v2._32_8_ = *(unsigned long *)&a1->field_0x20;
  v2._40_8_ = *(unsigned long *)&a1->field_0x28;
  v2._48_8_ = *(unsigned long *)&a1->field_0x30;
  v2._0_4_ = SUB84(*(unsigned long *)a1,0);
  v2._4_4_ = SUB84((unsigned long)*(unsigned long *)a1 >> 0x20,0); // branch-flip
  v2._0_4_ = (0xb <= v2._0_4_) ? v2._0_4_ + -0xb : v2._0_4_ + 0xb;
  if (!dat_49a60) {
    dat_49a68 = 1;
    dat_49a60 = (char *)sub_31bc2(1);
  }
  while( true ) {
    v6 = strftime(dat_49a60,dat_49a68,v5,a1);
    if ((v6) && (v6 < dat_49a68)) break;
    dat_49a60 = (char *)sub_31dbf(dat_49a60,(unsigned long *)0x49a68,1);
  }
  v4 = 0;
  v3 = 0;
  v7 = a3 + v6 + 1;
  dat_49a60 = (long)sub_31c2c(dat_49a60,v7);
  dat_49a68 = v7;
  v8 = (char *)sub_31bc2(v7);
  strftime(v8,dat_49a68,v5,&v2);
  v1 = sub_11b14(dat_49a60,v8,&v4,&v3);
  if ((v1) && ((v3 == 2 && (!(*(unsigned short *)((unsigned long)*(unsigned char *)((long)dat_49a60 + v3 + v4) * 2 + *(long *)__ctype_b_loc()) & 0x800))))) {
    v9 = v3 + v4;
    v10 = (v6 - v9) + 1;
    if (v7 != v10 + v9 + a3)
      __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,"do_time_format"); // no-return
    memmove((void *)((long)dat_49a60 + a3 + v9),(void *)((long)dat_49a60 + v9),v10);
    memcpy((void *)((long)dat_49a60 + v3 + v4),a2,a3);
  }
  free(v5);
  free(v8);
  return (long)dat_49a60 + 1;
}

// Function: sub_1201f @ 0x1201f
char * sub_1201f(long a0,long a1,int a2) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1201f
{
  long v1;
  char *v10; // stack - 0x80
  long v11; // stack - 0x78
  unsigned long v12; // stack - 0x70
  char *v13; // stack - 0x68
  unsigned long v14; // stack - 0x60
  char v15; // stack - 0x53
  char v16; // stack - 0x52
  char v2; // stack - 0x54
  char v3 [40]; // stack - 0x48
  long v4; // stack - 0xa8
  long v5; // stack - 0xa0
  int v6; // stack - 0x98
  int v7; // stack - 0x94
  char *v8; // stack - 0x90
  tm *v9; // stack - 0x88
  
  v6 = 0;
  v7 = 0;
  v4 = a0;
  v5 = a1;
  if (a2 != 0x2b) { // branch-flip
    v2 = '%';
    v15 = (char)a2;
    v16 = 0;
    v7 = !((0x19 <= (unsigned int)(a2 - 0x40U)) || (!(0x1180001UL >> ((unsigned char)(a2 - 0x40U) & 0x3f) & 1)));
  }
  else {
    strcpy(&v2,"%Y-%m-%d+%T");
    v7 = 1;
  }
  if (v7) { // branch-flip
    v6 = snprintf(v3,0x20,".%09ld0",v5);
    if (0x20 <= v6)
      __assert_fail("charsprinted < NS_BUF_LEN","print.c",0x299,"format_date"); // no-return
  }
  else {
    v6 = 0;
    v3[0] = '\0';
  }
  if (a2 != 0x40) {
    v9 = localtime(&v4);
    if ((v9) && (v10 = (char *)sub_11c89(&v2,v9,v3,(long)v6), v10))
      return v10;
  }
  v11 = v4;
  if (0 <= v4) // branch-flip
    v1 = v4;
  else {
    v1 = -v4;
  }
  v8 = (char *)sub_22eb1(v1,(char *)0x49a81,0,1,1);
  if ((char *)0x49a81 <= v8) {
    if ((char *)0x49d97 <= v8)
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date"); // no-return
    if (v4 <= -1) {
      v8 = &v8[-1];
      *v8 = '-';
    }
    if (v7) {
      v12 = strlen(v8);
      v13 = &v8[v12 - 0x49a80];
      if ((char *)0x317 <= v13)
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date"); // no-return
      v14 = 0x316 - (long)v13;
      if (v14 <= strlen(v3))
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)v6,v14,v3);
      if (v14 <= strlen(v3))
        __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date"); // no-return
      strcat(v8,v3);
    }
    return v8;
  }
  __assert_fail("p > buf","print.c",0x2b9,"format_date"); // no-return
}

// Function: sub_123c8 @ 0x123c8
unsigned long sub_123c8(unsigned long a0,long a1) // early-return
{
  unsigned long v1; // stack - 0x28
  long v2; // stack - 0x20
  int v3; // stack - 0x14
  tm *v4; // stack - 0x10
  
  v1 = a0;
  v2 = a1;
  v4 = localtime(&v1);
  if (!v4)
    return sub_1201f(v1,v2,0x40);
  if (*(int *)&v4->field_0x18 <= -1)
    __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format"); // no-return
  if (7 <= *(int *)&v4->field_0x18)
    __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format"); // no-return
  if (*(int *)&v4->field_0x10 <= -1)
    __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format"); // no-return
  if (0xc <= *(int *)&v4->field_0x10)
    __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format"); // no-return
  if (*(int *)&v4->field_0x8 <= -1)
    __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format"); // no-return
  if (0x18 <= *(int *)&v4->field_0x8)
    __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format"); // no-return
  if (*(int *)&v4->field_0x4 <= 0x3b) {
    if (0x3e <= *(int *)v4)
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format"); // no-return
    v3 = snprintf((char *)0x49da0,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",*(char **)((long)*(int *)&v4->field_0x18 * 8 + 0x49680),*(char **)((long)*(int *)&v4->field_0x10 * 8 + 0x496c0),*(int *)&v4->field_0xc,*(int *)&v4->field_0x8,*(int *)&v4->field_0x4,*(int *)v4,v2,*(int *)&v4->field_0x14 + 0x76c);
    if (v3 <= 0x3ff)
      return 0x49da0;
    __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format"); // no-return
  }
  __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format"); // no-return
}

// Function: sub_12693 @ 0x12693
double sub_12693(void *a0)
{
  double v1;
  
  if (*(long *)((long)a0 + 0x30)) // branch-flip
    v1 = (dat_3acc0 * (double)*(long *)((long)a0 + 0x40)) / (double)*(long *)((long)a0 + 0x30);
  else if (*(long *)((long)a0 + 0x40)) { // branch-flip
    if (0 <= *(long *)((long)a0 + 0x40)) // branch-flip
      v1 = dat_3acb8;
    else {
      v1 = dat_3acb0;
    }
  }
  else {
    v1 = dat_3aca8;
  }
  return v1;
}

// Function: sub_1272c @ 0x1272c
void sub_1272c(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,void *a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  FILE *v1;
  unsigned long v10; // stack - 0xb0
  unsigned long v11; // stack - 0xa8
  unsigned long v12; // stack - 0xa0
  unsigned long v13; // stack - 0x98
  unsigned long v14; // stack - 0x88
  unsigned long v15; // stack - 0x78
  unsigned long v16; // stack - 0x68
  unsigned long v17; // stack - 0x58
  unsigned long v18; // stack - 0x48
  unsigned long v19; // stack - 0x38
  char *v2;
  unsigned long v20; // stack - 0x28
  char v3; // al
  char v4 [16];
  unsigned int v5; // stack - 0xe8
  unsigned int v6; // stack - 0xe4
  char *v7; // stack - 0xe0
  char *v8; // stack - 0xd8
  unsigned long v9; // stack - 0xb8
  
  if (v3) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v5 = 0x10;
  v6 = 0x30;
  v7 = &Stack0000000000000008;
  v8 = v4;
  v1 = *(FILE **)((long)a8 + 8);
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  if (0 <= vfprintf(v1,a9,&v5))
    return;
  v2 = *(char **)((long)a8 + 0x10);
  sub_10a77(*__errno_location(),v2);
}

// Function: sub_12839 @ 0x12839
void sub_12839(void *a0,char *a1,char *a2) // return-dupe
{
  char v1;
  unsigned int *v2;
  FILE *v3;
  char *v4;
  
  v1 = *(char *)((long)a0 + 0x18);
  v2 = *(unsigned int **)((long)a0 + 0x20);
  v3 = *(FILE **)((long)a0 + 8);
  if (0 <= (int)sub_1c643(v3,v2,(unsigned char)v1,a1,a2))
    return;
  v4 = *(char **)((long)a0 + 0x10);
  sub_10a77(*__errno_location(),v4);
}

// Function: sub_128a9 @ 0x128a9
void sub_128a9(void *a0,unsigned long a1,unsigned long a2,void *a3) // return-dupe
{
  FILE *v1;
  char *v2;
  
  v1 = *(FILE **)((long)a3 + 8);
  if (a2 <= fwrite(a0,a1,a2,v1))
    return;
  v2 = *(char **)((long)a3 + 0x10);
  sub_10a77(*__errno_location(),v2);
}

// Function: sub_12910 @ 0x12910
void sub_12910(void *a0) // return-dupe
{
  FILE *v1;
  char *v2;
  
  v1 = *(FILE **)((long)a0 + 8);
  if (!sub_1e1a8(v1))
    return;
  v2 = *(char **)((long)a0 + 0x10);
  sub_10a77(*__errno_location(),v2);
}

// Function: sub_12955 @ 0x12955
char * sub_12955(int a0) // return-dupe x7
{
  if (a0 == 0x8000)
    return "f";
  if (a0 == 0x4000)
    return "d";
  if (a0 == 0xa000)
    return "l";
  if (a0 == 0xc000)
    return "s";
  if (a0 == 0x6000)
    return "b";
  if (a0 == 0x2000)
    return "c";
  if (a0 == 0x1000)
    return "p";
  return "U";
}

// Function: sub_129e7 @ 0x129e7
void sub_129e7(void *a0,struct_53 *a1,char *a2,struct_52 *a3)
{
  unsigned int v1;
  char *v10; // stack - 0x380
  void *v11; // stack - 0x378
  passwd *v12; // stack - 0x370
  char *v13; // stack - 0x368
  group *v14; // stack - 0x360
  void *v15; // stack - 0x358
  void *v16; // stack - 0x350
  char v17; // stack - 0x2ae
  int v2; // eax
  char *v3; // rax
  unsigned long v4; // rax
  char v5 [10];
  stat v6; // stack - 0x348
  char *v7; // stack - 0x390
  char v8 [16];
  char *v9; // stack - 0x388
  
  v1 = a1->field_0x0;
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if (v1 == 1) {
          sub_128a9(a1->field_0x8,1,(long)(int)a1->field_0x10,a0);
          sub_12910(a0);
        }
      }
      else {
        sub_128a9(a1->field_0x8,1,(long)(int)a1->field_0x10,a0);
      }
    }
  }
  else {
    switch(a1->field_0x4) {
      case 0:
        error(1,0,gettext("error: %s at end of format string"),"%");
        break;
      case 0x25:
        sub_128a9(a1->field_0x8,1,(long)(int)a1->field_0x10,a0);
        break;
      case 0x44:
        v4 = sub_22eb1(a3->field_0x0,v5,0,1,1);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x46:
        v4 = sub_13a18(a3,a2);
        sub_12839(a0,a1->field_0x8,v4);
        break;
      case 0x48:
        v16 = (void *)sub_31bc2((long)(dat_499dc + 1));
        memcpy(v16,a2,(long)dat_499dc);
        *(char *)((long)v16 + (long)dat_499dc) = 0;
        sub_1272c(a0,a1->field_0x8,v16);
        free(v16);
        break;
      case 0x4d:
        sub_1e472(a3,v5);
        v17 = 0;
        sub_1272c(a0,a1->field_0x8,v5);
        break;
      case 0x50:
        if (1 <= dat_499c0) { // branch-flip
          v9 = &a2[dat_499dc];
          if (*v9 == '/')
            v9 = &v9[1];
        }
        else {
          v9 = "";
        }
        sub_12839(a0,a1->field_0x8,v9);
        break;
      case 0x53:
        sub_12693(a3);
        sub_1272c(a0,a1->field_0x8);
        break;
      case 0x59:
        if ((a3->field_0x18 & 0xf000) != 0xa000) { // branch-flip
          v4 = sub_12955(a3->field_0x18 & 0xf000);
          sub_1272c(a0,a1->field_0x8,v4);
        }
        else if (fstatat(dat_499d8,dat_499d0,&v6,0)) { // branch-flip
          if ((*__errno_location() != 2) && (*__errno_location() != 0x14)) {
            if (*__errno_location() != 0x28) { // branch-flip
              sub_1272c(a0,a1->field_0x8,"?");
              v4 = sub_1096a(0,a2);
              error(0,*__errno_location(),"%s",v4);
            }
            else {
              sub_1272c(a0,a1->field_0x8,"L");
            }
          }
          else {
            sub_1272c(a0,a1->field_0x8,"N");
          }
        }
        else {
          v4 = sub_12955(v6._24_4_ & 0xf000);
          sub_1272c(a0,a1->field_0x8,v4);
        }
        break;
      case 0x5a:
        v2 = (*dat_499a0)(dat_499d8,dat_499d0,&v7);
        if (0 <= v2) { // branch-flip
          sub_1272c(a0,a1->field_0x8,v7);
          freecon(v7);
        }
        else {
          sub_1272c(a0,a1->field_0x8,0x3aa45);
          v4 = sub_1096a(0,a2);
          v3 = gettext("getfilecon failed: %s");
          error(0,*__errno_location(),v3,v4);
          dat_499e4 = 1;
        }
        break;
      case 0x61:
        v8 = sub_30a4c(a3);
        v4 = sub_123c8(SUB168(v8,0),SUB168(v8,8));
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x62:
        v4 = sub_22eb1(a3->field_0x40,v5,0,0x200,0x200);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 99:
        v8 = sub_30a66(a3);
        v4 = sub_123c8(SUB168(v8,0),SUB168(v8,8));
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 100:
        sub_1272c(a0,a1->field_0x8,dat_499c0);
        break;
      case 0x66:
        v15 = (void *)sub_1d85c(a2);
        sub_12839(a0,a1->field_0x8,v15);
        free(v15);
        break;
      case 0x67:
        v14 = getgrgid(a3->field_0x20);
        if (v14) {
          *(char *)((long)(int)a1->field_0x10 + (long)a1->field_0x8) = 0x73;
          sub_1272c(a0,a1->field_0x8,*(unsigned long *)v14);
          return;
        }
        v4 = sub_22eb1(a3->field_0x20,v5,0,1,1); // return-dupe
        sub_1272c(a0,a1->field_0x8,v4);
        return;
      case 0x47:
        v4 = sub_22eb1(a3->field_0x20,v5,0,1,1);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x68:
        v3 = (char *)sub_321c9(a2);
        v13 = v3;
        for (v10 = &v13[strlen(v3) - 1]; (v13 <= v10 && (*v10 == '/')); v10 = &v10[-1]) {
        }
        if ((v13 < v10) && (v10[1] == '/'))
          v10[1] = '\0';
        v10 = strrchr(v13,0x2f);
        if (v10) { // branch-flip
          *v10 = '\0';
          sub_12839(a0,a1->field_0x8,v13);
        }
        else {
          sub_12839(a0,a1->field_0x8,".");
        }
        free(v13);
        break;
      case 0x69:
        v4 = sub_22eb1(a3->field_0x8,v5,0,1,1);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x6b:
        v4 = sub_22eb1(a3->field_0x40,v5,0,0x200,0x400);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x6c:
        v11 = NULL;
        if ((a3->field_0x18 & 0xf000) == 0xa000) {
          v11 = (void *)sub_1d06d(dat_499d8,dat_499d0);
          if (!v11) {
            sub_10a13(*__errno_location(),a2);
            dat_499e4 = 1;
          }
        }
        if (v11) // branch-flip
          sub_12839(a0,a1->field_0x8,v11);
        else {
          sub_12839(a0,a1->field_0x8,0x3aa45);
        }
        free(v11);
        break;
      case 0x6d:
        sub_1272c(a0,a1->field_0x8,a3->field_0x18 & 0xfff);
        break;
      case 0x6e:
        v4 = sub_22eb1(a3->field_0x10,v5,0,1,1);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x70:
        sub_12839(a0,a1->field_0x8,a2);
        break;
      case 0x73:
        v4 = sub_22eb1(a3->field_0x30,v5,0,1,1);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x74:
        v8 = sub_30a80(a3);
        v4 = sub_123c8(SUB168(v8,0),SUB168(v8,8));
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x75:
        v12 = getpwuid(a3->field_0x1c);
        if (v12) {
          *(char *)((long)(int)a1->field_0x10 + (long)a1->field_0x8) = 0x73;
          sub_1272c(a0,a1->field_0x8,*(unsigned long *)v12);
          return;
        }
        v4 = sub_22eb1(a3->field_0x1c,v5,0,1,1); // return-dupe
        sub_1272c(a0,a1->field_0x8,v4);
        return;
      case 0x55:
        v4 = sub_22eb1(a3->field_0x1c,v5,0,1,1);
        sub_1272c(a0,a1->field_0x8,v4);
        break;
      case 0x79:
        v4 = sub_12955(a3->field_0x18 & 0xf000);
        sub_1272c(a0,a1->field_0x8,v4);
      
    }
  }
  return;
}

// Function: sub_1376d @ 0x1376d
unsigned long sub_1376d(char *a0,struct_52 *a1,void *a2)
{
  char v1;
  unsigned char v2;
  unsigned long *v3; // rax
  unsigned long v4; // rax
  char v5 [16];
  int *v6; // stack - 0x28
  long v7; // stack - 0x18
  long v8; // stack - 0x10
  
  v3 = (unsigned long *)((long)a2 + 0x38);
  v6 = (int *)*v3;
  do {
    if (!v6)
      return 1;
    if ((*v6 != 2) || (!*(char *)((long)v6 + 5)))
      sub_129e7(v3,v6,a0,a1);
    else {
      v1 = (char)v6[1];
      if (v1 != 'T') { // branch-flip
        if ('T' < v1)
          __assert_fail("0","print.c",0x518,"pred_fprintf"); // no-return, return-dupe
        if (v1 != 'C') { // branch-flip
          if ('C' < v1) {
            __assert_fail("0","print.c",0x518,"pred_fprintf");
          }
          if (v1 != 'A') { // branch-flip
            if (v1 != 'B') {
              __assert_fail("0","print.c",0x518,"pred_fprintf");
            }
            v5 = sub_30a9a(a1);
            v8 = SUB168(v5,8);
            v7 = SUB168(v5,0);
            if (*(char *)((long)v6 + 5) != '@') // branch-flip
              v2 = (unsigned char)~SUB161(v5,0xf) >> 7;
            else {
              v2 = 1;
            }
          }
          else {
            v5 = sub_30a4c(a1);
            v8 = SUB168(v5,8);
            v7 = SUB168(v5,0);
            v2 = 1;
          }
        }
        else {
          v5 = sub_30a66(a1);
          v8 = SUB168(v5,8);
          v7 = SUB168(v5,0);
          v2 = 1;
        }
      }
      else {
        v5 = sub_30a80(a1);
        v8 = SUB168(v5,8);
        v7 = SUB168(v5,0);
        v2 = 1;
      }
      if (v2) { // branch-flip
        v4 = sub_1201f(v7,v8,(int)*(char *)((long)v6 + 5));
        sub_1272c(v3,*(unsigned long *)&v6[2],v4);
      }
      else {
        sub_1272c(v3,*(unsigned long *)&v6[2],0x3aa45);
      }
    }
    v6 = *(int **)&v6[6];
  } while( true );
}

// Function: sub_1395f @ 0x1395f
void sub_1395f(void *a0)
{
  void *v1;
  void *v2; // stack - 0x20
  
  v2 = a0;
  while (v2) {
    v1 = *(void **)((long)v2 + 0x30);
    sub_26f67(v2);
    v2 = v1;
  }
}

// Function: sub_1399c @ 0x1399c
long sub_1399c(char a0)
{
  if (((dat_4a1a0) && (dat_4a1a8 != '\x01')) && (a0)) {
    sub_1395f(dat_4a1a0);
    dat_4a1a0 = 0;
  }
  if (!dat_4a1a0) {
    dat_4a1a0 = sub_262d1((unsigned char)a0);
    dat_4a1a8 = a0;
  }
  return dat_4a1a0;
}

// Function: sub_13a18 @ 0x13a18
void * sub_13a18(long *a0,unsigned long a1)
{
  if (dat_4a1b0) {
    if ((dat_4a1b8) && (*a0 == dat_4a1c0))
      return dat_4a1b0;
    free(dat_4a1b0);
  }
  dat_4a1c0 = *a0;
  dat_4a1b0 = (void *)sub_13be0(a0,a1,(void *)0x4a1b8);
  return dat_4a1b0;
}

// Function: sub_13aa6 @ 0x13aa6
unsigned long sub_13aa6(char *a0) // return-dupe x2
{
  char *v1;
  long v2; // stack - 0x18
  
  if (!strcmp("afs",a0))
    return 1;
  v2 = sub_1399c(0);
  if (v2) {
    while( true ) {
      if (!v2)
        return 0;
      v1 = *(char **)(v2 + 0x18);
      if (!strcmp(a0,v1)) break;
      v2 = *(long *)(v2 + 0x30);
    }
    return 1;
  }
  return 1;
}

// Function: sub_13b3b @ 0x13b3b
unsigned long sub_13b3b(void *a0) // return-dupe x2
{
  int v1; // eax
  unsigned long v2 [19]; // stack - 0xa8
  
  if (*(long *)((long)a0 + 0x20) != -1)
    return 0;
  sub_f26e(v2);
  v1 = (*dat_49990)(*(unsigned long *)((long)a0 + 8),v2);
  if (!v1) {
    *(unsigned long *)((long)a0 + 0x20) = v2[0];
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_13be0 @ 0x13be0
long sub_13be0(long *a0,unsigned long a1,void *a2)
{
  char *v1;
  long v2; // rax
  long v3; // stack - 0x28
  long v4; // stack - 0x20
  long v5; // stack - 0x18
  
  v4 = 0;
  v3 = sub_1399c(1);
  if (v3) {
    v5 = 0;
    for (; v3; v3 = *(long *)(v3 + 0x30)) {
      v1 = *(char **)(v3 + 0x18);
      if (((strcmp(v1,"ignore")) && (!sub_13b3b(v3))) && (*(long *)(v3 + 0x20) == *a0))
        v4 = v3;
    }
    if (v4) {
      v1 = *(char **)(v4 + 0x18);
      v5 = sub_321c9(v1);
    }
    *(bool *)a2 = v5 != 0;
    if (!v5)
      v5 = sub_321c9(gettext("unknown"));
    return v5;
  }
  error(1,0,gettext("Cannot read mounted file system list"));
  return v2;
}

// Function: sub_13cfc @ 0x13cfc
void * sub_13cfc(long *a0)
{
  int v1; // eax
  unsigned long v2; // stack - 0x40
  long v3; // stack - 0x38
  long v4; // stack - 0x30
  void *v5; // stack - 0x28
  long v6; // stack - 0x20
  void *v7; // stack - 0x18
  
  v2 = 0;
  v3 = 0;
  v5 = NULL;
  v6 = sub_262d1(0);
  for (v4 = v6; v4; v4 = *(long *)(v4 + 0x30)) {
    v7 = (void *)sub_1ac29(v5,(v3 + 1) * 8,&v2);
    if (v7) { // branch-flip
      v5 = v7;
      v1 = sub_13b3b(v4);
      if (!v1) {
        *(unsigned long *)(v3 * 8 + (long)v5) = *(unsigned long *)(v4 + 0x20);
        v3 += 1;
      }
    }
    else {
      free(v5);
      v5 = NULL;
    }
  }
  sub_1395f(v6);
  if (v5)
    *a0 = v3;
  return v5;
}

// Function: sub_13e1c @ 0x13e1c
int * sub_13e1c(unsigned int a0,char *a1,char **a2,unsigned int a3)
{
  int *v1; // rax
  
  v1 = __errno_location();
  if ((*v1 != 2) && (*v1 != 0x14))
    v1 = (unsigned long)(unsigned long)a3;
  else {
    if (dat_49980 & 2)
      fprintf(stderr,"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n",a1);
    v1 = (unsigned long)sub_3011a(a0,a1,a2);
  }
  return v1;
}

// Function: sub_13e94 @ 0x13e94
unsigned long sub_13e94(unsigned int a0,char *a1,char **a2) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_499c0) // branch-flip
    v2 = sub_3011a(a0,a1,a2);
  else {
    v1 = sub_2fe8c(a0,a1,a2);
    if (!v1) {
      v2 = 0;
      return v2;
    }
    v2 = sub_13e1c(a0,a1,a2,v1);
  }
  return v2;
}

// Function: sub_13f08 @ 0x13f08
unsigned long sub_13f08(unsigned int a0,char *a1,char **a2) // early-return
{
  int v1; // eax
  
  v1 = sub_2fe8c(a0,a1,a2);
  if (!v1)
    return 0;
  return sub_13e1c(a0,a1,a2,v1);
}

// Function: sub_13f5b @ 0x13f5b
void sub_13f5b(unsigned int a0,char *a1,char **a2)
{
  sub_3011a(a0,a1,a2);
}

// Function: sub_13f89 @ 0x13f89
void sub_13f89(long *a0)
{
  bool v1;
  bool v2;
  long *v3; // stack - 0x20
  
  v2 = 0;
  v1 = 0;
  for (v3 = a0; v3; v3 = (long *)v3[0x21]) {
    if ((void *)*v3 != sub_90b1) { // branch-flip
      if ((void *)*v3 == sub_a232)
        v1 = 1;
    }
    else {
      v2 = 1;
    }
  }
  if (((v1) && (v2)) && (dat_49941 != '\x01')) {
    error(1,0,gettext("The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option."));
    return;
  }
}

// Function: sub_14033 @ 0x14033
long sub_14033(void)
{
  int v1; // stack - 0xc
  
  if (!dat_4a1d0) {
    v1 = 0;
    while (*(long *)((long)v1 * 0x20 + 0x47de8)) {
      if (*(int *)((long)v1 * 0x20 + 0x47de0) == 1) {
        dat_4a1d0 = (long)v1 * 0x20 + 0x47de0;
        return dat_4a1d0;
      }
      v1 += 1;
    }
  }
  return dat_4a1d0;
}

// Function: sub_140b3 @ 0x140b3
unsigned long sub_140b3(void *a0,char a1,struct_70 *a2) // early-return x3
{
  char v1 [16];
  
  if (a1 == 'm') {
    v1 = sub_30a80(a0);
    a2->field_0x0 = SUB168(v1,0);
    a2->field_0x8 = SUB168(v1,8);
    return 1;
  }
  if (a1 <= 'm') {
    if (a1 == 'c') {
      v1 = sub_30a66(a0);
      a2->field_0x0 = SUB168(v1,0);
      a2->field_0x8 = SUB168(v1,8);
      return 1;
    }
    if (a1 <= 'c') {
      if (a1 == 'B') {
        v1 = sub_30a9a(a0);
        a2->field_0x0 = SUB168(v1,0);
        a2->field_0x8 = SUB168(v1,8);
        return ~a2->field_0x8 >> 0x3f;
      }
      if (a1 == 'a') {
        v1 = sub_30a4c(a0);
        a2->field_0x0 = SUB168(v1,0);
        a2->field_0x8 = SUB168(v1,8);
        return 1;
      }
    }
  }
  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime"); // no-return
}

// Function: sub_141b0 @ 0x141b0
void sub_141b0(unsigned int a0) // return-dupe
{
  switch(a0) { // branch-flip
    case 0:
      dat_49990 = sub_fcc0;
      dat_499a0 = sub_13f5b;
      break;
    case 1:
      dat_49990 = sub_fc21;
      dat_499a0 = sub_13f08;
      dat_4994c = 1;
      break;
    case 2:
      dat_49990 = sub_fb5c;
      dat_499a0 = sub_13e94;
      dat_4994c = 1;
    
  }
  dat_49988 = a0;
  if (!(dat_49980 & 2))
    return;
  dat_49990 = sub_fd39;
}

// Function: sub_14262 @ 0x14262
void sub_14262(void)
{
  dat_4a1c8 = 0;
}

// Function: sub_14287 @ 0x14287
void sub_14287(void)
{
  return;
}

// Function: sub_142a1 @ 0x142a1
char sub_142a1(void) // early-return
{
  if (dat_49951)
    return 0;
  return dat_49950;
}

// Function: sub_142c4 @ 0x142c4
int * sub_142c4(char *a0,int *a1)
{
  char *v1;
  char v2; // al
  
  if (*a1 != 2) {
    if (*a1 == 1)
      return NULL;
    if (*a1) { // branch-flip
      if (!dat_4a1c8)
        dat_4a1c8 = a0;
    }
    else if ((dat_4a1c8) && (v2 = sub_142a1(), v1 = dat_4a1c8, v2))
      error(0,0,gettext("warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments."),a0,v1,a0);
  }
  return a1;
}

// Function: sub_1437a @ 0x1437a
unsigned long sub_1437a(char *a0)
{
  char *v1;
  char *v2; // stack - 0x20
  int v3; // stack - 0x14
  
  if ((!strncmp("-newer",a0,6)) && (strlen(a0) == 8))
    return sub_142c4(a0,(int *)0x47dc0);
  v2 = a0;
  if (*a0 == '-')
    v2 = &a0[1];
  v3 = 0;
  while( true ) {
    if (!*(long *)((long)v3 * 0x20 + 0x47de8))
      return 0;
    v1 = *(char **)((long)v3 * 0x20 + 0x47de8);
    if (!strcmp(v1,v2)) break;
    v3 += 1;
  }
  return sub_142c4(a0,(int *)((long)v3 * 0x20 + 0x47de0));
}

// Function: sub_1446f @ 0x1446f
unsigned int sub_1446f(float a0)
{
  unsigned int v1;
  
  if (dat_3c028 <= a0) {
    v1 = dat_3c034;
    if ((dat_3c030 > a0) || (a0 > dat_3c038)) {
      return v1; // return-dupe
    }
    v1 = dat_3c03c;
  }
  else {
    v1 = dat_3c02c;
  }
  return v1;
}

// Function: sub_144ce @ 0x144ce
float sub_144ce(long a0)
{
  a0 = dat_49968 - a0;
  sub_1446f();
  return (float)(int)(a0 / 0x15180);
}

// Function: sub_14528 @ 0x14528
unsigned long sub_14528(long a0,int *a1,unsigned long *a2) // return-dupe
{
  if ((a0) && (*(long *)(a0 + (long)*a1 * 8))) {
    *a2 = *(unsigned long *)(a0 + (long)*a1 * 8);
    *a1 = *a1 + 1;
    return 1;
  }
  *a2 = 0;
  return 0;
}

// Function: sub_145ab @ 0x145ab
char sub_145ab(long a0,int *a1,unsigned long *a2)
{
  char v1; // al
  unsigned long v2; // stack - 0x18
  
  v1 = sub_14528(a0,a1,&v2);
  *a2 = v2;
  return v1;
}

// Function: sub_14611 @ 0x14611
unsigned long sub_14611(long a0,int *a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  int v1; // eax
  unsigned long v2; // stack - 0x28
  
  if (!sub_145ab(a0,a1,&v2)) {
    *a3 = 0;
    return 0;
  }
  *a3 = v2;
  v1 = (*dat_49990)(v2,a2);
  if (!v1)
    return 1;
  sub_10a44(*__errno_location(),v2); // no-return
}

// Function: sub_146c4 @ 0x146c4
unsigned long sub_146c4(int *a0)
{
  struct_55 *v1; // rax
  
  v1 = (struct_55 *)sub_e644(a0);
  v1->field_0x0 = sub_8d9c;
  v1->field_0x10 = 3;
  v1->field_0x14 = 3;
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  return 1;
}

// Function: sub_1472e @ 0x1472e
bool sub_1472e(struct_58 *a0,long a1,int *a2)
{
  long v1; // rax
  char v2 [88];
  unsigned long v3; // stack - 0xc8
  bool v4; // zf
  unsigned int v5; // xmm0_da
  char v6 [16];
  long v7; // stack - 0xc0
  long v8; // stack - 0x60
  
  sub_f26e(v2);
  v4 = sub_14611(a1,a2,v2,&v3) != '\0';
  if (v4) {
    v1 = sub_eecc(a0,v3);
    v7 = v1;
    *(unsigned int *)(v1 + 0x38) = 0;
    v6 = sub_30a80(v2);
    *(long *)(v1 + 0x40) = SUB168(v6,0);
    *(long *)(v1 + 0x48) = SUB168(v6,8);
    *(unsigned int *)(v7 + 0x3c) = 0;
    sub_144ce(v8);
    *(unsigned int *)(v7 + 0x24) = v5;
  }
  return v4;
}

// Function: sub_14833 @ 0x14833
unsigned long sub_14833(int *a0)
{
  struct_55 *v1; // rax
  
  v1 = (struct_55 *)sub_e644(a0);
  v1->field_0x0 = sub_8ee1;
  v1->field_0x10 = 5;
  v1->field_0x14 = 0;
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  return 1;
}

// Function: sub_1489d @ 0x1489d
bool sub_1489d(struct_58 *a0,long a1,int *a2)
{
  long v1; // rax
  char v2 [88];
  unsigned long v3; // stack - 0xc8
  bool v4; // zf
  unsigned int v5; // xmm0_da
  char v6 [16];
  long v7; // stack - 0xc0
  long v8; // stack - 0x60
  
  sub_f26e(v2);
  v4 = sub_14611(a1,a2,v2,&v3) != '\0';
  if (v4) {
    v1 = sub_eecc(a0,v3);
    v7 = v1;
    *(unsigned int *)(v1 + 0x38) = 2;
    v6 = sub_30a80(v2);
    *(long *)(v1 + 0x40) = SUB168(v6,0);
    *(long *)(v1 + 0x48) = SUB168(v6,8);
    *(unsigned int *)(v7 + 0x3c) = 0;
    sub_144ce(v8);
    *(unsigned int *)(v7 + 0x24) = v5;
  }
  return v4;
}

// Function: sub_149a2 @ 0x149a2
unsigned long sub_149a2(int *a0)
{
  struct_71 *v1; // rax
  
  v1 = (struct_71 *)sub_e644(a0);
  v1->field_0x0 = sub_8fd0;
  v1->field_0x10 = 3;
  v1->field_0x14 = 1;
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  v1->field_0x24 = dat_3c030;
  return 1;
}

// Function: sub_14a1d @ 0x14a1d
tm * sub_14a1d(void) // return-dupe
{
  tm *v1;
  int v2; // eax
  tm *v3; // rax
  int v4;
  
  if (dat_49978 == '\x01') {
    v3 = (tm *)0x1;
    return v3;
  }
  v3 = (long)(dat_49968 + 0x15180);
  dat_49970 = 0;
  dat_49968 = (long)v3;
  v3 = localtime((void *)0x49968);
  v1 = v3;
  if (v3) { // branch-flip
    v4 = *(int *)v3;
    v2 = *(int *)&v3->field_0x4 * 0x3c;
    v4 += v2;
    v2 = *(int *)&v1->field_0x8 * 0xe10;
    v2 += v4;
    v3 = (long)(long)v2;
  }
  else {
    v3 = (long)(dat_49968 % 0x15180);
  }
  dat_49968 -= (long)v3;
  v3 = (tm *)0x1;
  dat_49978 = 1;
  return v3;
}

// Function: sub_14afc @ 0x14afc
unsigned long sub_14afc(struct_58 *a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  dat_49940 = 0;
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}

// Function: sub_14b71 @ 0x14b71
void sub_14b71(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_49940 = 0;
  dat_49941 = 1;
  sub_17bae(a0,a1,a2);
}

// Function: sub_14bb0 @ 0x14bb0
void sub_14bb0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  if (sub_142a1())
    error(0,0,gettext("warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature."));
  sub_14b71(a0,a1,a2);
}

// Function: sub_14c10 @ 0x14c10
unsigned long sub_14c10(struct_58 *a0)
{
  *(unsigned int *)(sub_ef32(a0) + 0x24) = dat_3c02c;
  return 1;
}

// Function: sub_14c50 @ 0x14c50
void sub_14c50(struct_74 *a0,long a1,int *a2)
{
  sub_1893a("-exec",a0,a1,a2);
}

// Function: sub_14c88 @ 0x14c88
void sub_14c88(struct_74 *a0,long a1,int *a2)
{
  sub_1893a("-execdir",a0,a1,a2);
}

// Function: sub_14cc0 @ 0x14cc0
unsigned long sub_14cc0(void)
{
  long v1; // rax
  
  v1 = sub_ef32((struct_58 *)sub_1437a("false"));
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(char *)(v1 + 0x19) = 0;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(unsigned int *)(v1 + 0x24) = 0;
  return 1;
}

// Function: sub_14d31 @ 0x14d31
void sub_14d31(void)
{
  sub_14cc0();
}

// Function: sub_14d50 @ 0x14d50
bool sub_14d50(unsigned long a0,long a1,int *a2)
{
  unsigned long v1; // stack - 0x18
  bool v2; // zf
  
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2)
    dat_499b0 = v1;
  return v2;
}

// Function: sub_14dbf @ 0x14dbf
unsigned long sub_14dbf(struct_58 *a0,char *a1)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  if (a1) // branch-flip
    sub_19964(a1,(struct_60 *)(v1 + 0x38));
  else {
    sub_19a85((struct_60 *)(v1 + 0x38));
  }
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}

// Function: sub_14e42 @ 0x14e42
unsigned long sub_14e42(struct_58 *a0,long a1,int *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_145ab(a1,a2,&v1))
    return 0;
  if (!sub_14dbf(a0,v1)) {
    *a2 = *a2 + -1;
    return 0;
  }
  return 1;
}

// Function: sub_14ecc @ 0x14ecc
void sub_14ecc(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_141b0(1);
  sub_17bae(a0,a1,a2);
}

// Function: sub_14f07 @ 0x14f07
bool sub_14f07(struct_58 *a0,long a1,int *a2)
{
  unsigned long v1; // stack - 0x20
  bool v2; // zf
  long v3; // stack - 0x18
  
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2) {
    v3 = sub_eecc(a0,v1);
    sub_19964(v1,(struct_60 *)(v3 + 0x38));
    *(char *)(v3 + 0x19) = 1;
    *(char *)(v3 + 0x18) = *(char *)(v3 + 0x19);
    *(char *)(v3 + 0x1b) = 0;
    *(char *)(v3 + 0x1a) = *(char *)(v3 + 0x1b);
    *(unsigned int *)(v3 + 0x24) = dat_3c030;
  }
  return v2;
}

// Function: sub_14fd8 @ 0x14fd8
unsigned long sub_14fd8(struct_58 *a0,char *a1)
{
  long v1; // rax
  
  v1 = sub_eecc(a0,a1);
  if (a1) // branch-flip
    sub_19964(a1,(struct_60 *)(v1 + 0x38));
  else {
    sub_19a85((struct_60 *)(v1 + 0x38));
  }
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}

// Function: sub_15079 @ 0x15079
unsigned long sub_15079(struct_58 *a0,long a1,int *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_145ab(a1,a2,&v1))
    return 0;
  if (!sub_14fd8(a0,v1)) {
    *a2 = *a2 + -1;
    return 0;
  }
  return 1;
}

// Function: sub_15103 @ 0x15103
unsigned int sub_15103(char *a0)
{
  char *v1; // rax
  stat v2;
  unsigned int v3;
  
  if (stat("/",&v2)) // branch-flip
    v3 = dat_3c030;
  else {
    v1 = (char *)sub_13a18(&v2,"/");
    if (strcmp(a0,v1)) // branch-flip
      v3 = dat_3c034;
    else {
      v3 = dat_3c040;
    }
  }
  return v3;
}

// Function: sub_151c2 @ 0x151c2
unsigned long sub_151c2(struct_58 *a0,long a1,int *a2) // early-return x2
{
  char v1; // al
  unsigned long v2; // rax
  char *v3; // stack - 0x20
  unsigned int v4; // xmm0_da
  long v5; // stack - 0x18
  
  if (!sub_145ab(a1,a2,&v3))
    return 0;
  if ((2 <= dat_499a8) && (v1 = sub_13aa6(v3), !v1)) {
    if (dat_49980 & 8)
      fprintf(stderr,"-fstype %s can never succeed, substituting -false\n",v3);
    v2 = sub_14cc0();
    return v2;
  }
  v5 = sub_eecc(a0,v3);
  *(char **)(v5 + 0x38) = v3;
  sub_15103(v3);
  *(unsigned int *)(v5 + 0x24) = v4;
  return 1;
}

// Function: sub_152b9 @ 0x152b9
unsigned long sub_152b9(struct_58 *a0,long a1,int *a2) // return-dupe
{
  long v1; // rax
  unsigned int v2;
  
  v1 = sub_197bf(a1,a2,a0);
  if (v1) {
    if (100 <= *(unsigned long *)(v1 + 0x40)) // branch-flip
      v2 = dat_3c048;
    else {
      v2 = dat_3c044;
    }
    *(unsigned int *)(v1 + 0x24) = v2;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}

// Function: sub_15339 @ 0x15339
unsigned long sub_15339(struct_58 *a0,long a1,int *a2) // return-dupe
{
  int v1; // eax
  unsigned int v10; // stack - 0x44
  group *v11; // stack - 0x30
  char *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  char *v8; // stack - 0x38
  unsigned int v9;
  
  if (!sub_145ab(a1,a2,&v8))
    return 0;
  v11 = getgrnam(v8);
  endgrent();
  if (v11) // branch-flip
    v10 = *(unsigned int *)&v11->field_0x10;
  else {
    v1 = (int)strspn(v8,"0123456789");
    if (!v1) {
      if (*v8) {
        v2 = (char *)sub_2f861(0,dat_499ac,v8);
        error(1,0,gettext("%s is not the name of an existing group"),v2);
        return v5;
      }
      error(1,0,gettext("argument to -group is empty, but should be a group name"));
      return v6;
    }
    if (v8[v1]) {
      v2 = (char *)sub_2f861(1,dat_499ac,&v8[v1]);
      v3 = (char *)sub_2f861(0,dat_499ac,v8);
      error(1,0,gettext("%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s"),v3,v2);
      return v4;
    }
    v10 = sub_1cd12(v8,dat_499ac);
  }
  v7 = sub_eecc(a0,v8);
  *(unsigned int *)(v7 + 0x38) = v10;
  if (100 <= *(unsigned long *)(v7 + 0x40)) // branch-flip
    v9 = dat_3c048;
  else {
    v9 = dat_3c044;
  }
  *(unsigned int *)(v7 + 0x24) = v9;
  return 1;
}

// Function: sub_15554 @ 0x15554
void sub_15554(void)
{
  sub_f071(0); // no-return
}

// Function: sub_15576 @ 0x15576
unsigned int sub_15576(char *a0,int a1)
{
  unsigned int v1;
  
  if (strpbrk(a0,"*?[")) // branch-flip
    v1 = dat_3c04c; // return-dupe
  else {
    if (a1) {
      if (strpbrk(a0,".")) {
        v1 = dat_3c04c;
        return v1;
      }
    }
    v1 = dat_3c028;
  }
  return v1;
}

// Function: sub_155d9 @ 0x155d9
bool sub_155d9(struct_58 *a0,long a1,int *a2)
{
  unsigned long v1; // stack - 0x20
  bool v2; // zf
  long v3; // stack - 0x18
  
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2) {
    v3 = sub_eecc(a0,v1);
    *(unsigned long *)(v3 + 0x38) = v1;
    *(float *)(v3 + 0x24) = (float)sub_15576(v1,0) * dat_3c028;
  }
  return v2;
}

// Function: sub_15686 @ 0x15686
unsigned long sub_15686(void)
{
  unsigned long v1; // rax
  
  if (dat_4a1d8 == 1)
    return (unsigned long)dat_4a1d8; // return-dupe
  if (!fnmatch("foo","foo",0)) {
    if (fnmatch("Foo","foo",0)) {
      if (!fnmatch("Foo","foo",0x10)) {
        dat_4a1d8 = 1;
        return (unsigned long)dat_4a1d8;
      }
    }
  }
  error(1,0,gettext("sanity check of the fnmatch() library function failed."));
  return v1;
}

// Function: sub_1573c @ 0x1573c
unsigned long sub_1573c(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  if (sub_142a1()) {
    if (strchr(a2,0x2f)) {
      v1 = sub_1096a(2,a1);
      v2 = sub_1096a(1,"/");
      v3 = sub_1096a(0,a0);
      error(0,0,gettext("warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?"),v3,v2,v1);
    }
  }
  return 1;
}

// Function: sub_157fa @ 0x157fa
unsigned long sub_157fa(struct_58 *a0,long a1,int *a2) // return-dupe
{
  char v1; // al
  unsigned long v2; // stack - 0x20
  unsigned int v3; // xmm0_da
  long v4; // stack - 0x18
  
  sub_15686();
  if ((sub_145ab(a1,a2,&v2)) && (v1 = sub_1573c("-iname","-iwholename",v2), v1)) {
    v4 = sub_eecc(a0,v2);
    *(char *)(v4 + 0x1b) = 0;
    *(char *)(v4 + 0x1a) = *(char *)(v4 + 0x1b);
    *(unsigned long *)(v4 + 0x38) = v2;
    sub_15576(v2,0);
    *(unsigned int *)(v4 + 0x24) = v3;
    return 1;
  }
  return 0;
}

// Function: sub_158e1 @ 0x158e1
bool sub_158e1(struct_58 *a0,long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_197bf(a1,a2,a0);
  if (!v1)
    *a2 = *a2 + -1;
  else {
    *(unsigned int *)(v1 + 0x24) = dat_3c050;
    *(char *)(v1 + 0x1c) = 1;
    *(char *)(v1 + 0x1a) = 0;
    *(char *)(v1 + 0x1b) = 0;
  }
  return v1 != 0;
}

// Function: sub_15961 @ 0x15961
void sub_15961(struct_57 *a0,long a1,int *a2)
{
  sub_1723e(a1,a2,a0,dat_4999c | 0x400000);
}

// Function: sub_1599c @ 0x1599c
unsigned long sub_1599c(struct_58 *a0,long a1,int *a2) // return-dupe, return-dupe x2
{
  long v1; // rax
  
  v1 = sub_197bf(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(long *)(v1 + 0x40) == 1) {
    *(unsigned int *)(v1 + 0x24) = dat_3c044;
    return 1;
  }
  if (*(long *)(v1 + 0x40) == 2) {
    *(unsigned int *)(v1 + 0x24) = dat_3c02c;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = dat_3c054;
  return 1;
}

// Function: sub_15a46 @ 0x15a46
bool sub_15a46(struct_58 *a0,long a1,int *a2)
{
  unsigned long v1; // stack - 0x20
  bool v2; // zf
  long v3; // stack - 0x18
  
  sub_15686();
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2) {
    v3 = sub_eecc(a0,v1);
    *(unsigned long *)(v3 + 0x38) = v1;
    *(float *)(v3 + 0x24) = (float)sub_15576(v1,0) * dat_3c028;
  }
  return v2;
}

// Function: sub_15af8 @ 0x15af8
void sub_15af8(struct_58 *a0)
{
  sub_14dbf(a0,NULL);
}

// Function: sub_15b23 @ 0x15b23
unsigned long sub_15b23(unsigned long a0,long a1,int *a2,int *a3)
{
  int v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  char *v4; // stack - 0x30
  char *v5; // stack - 0x28
  
  v5 = *(char **)(a1 + (long)*a2 * 8 + -8);
  if (!sub_145ab(a1,a2,&v4))
    return 0;
  v1 = (int)strspn(v4,"0123456789");
  if ((1 <= v1) && (!v4[v1])) {
    v1 = sub_1cd12(v4,dat_499ac);
    *a3 = v1;
    if (0 <= *a3)
      return sub_17bae(a0,a1,a2);
  }
  v2 = (char *)sub_2f861(0,dat_499ac,v4);
  error(1,0,gettext("Expected a positive decimal integer argument to %s, but got %s"),v5,v2);
  return v3;
}

// Function: sub_15c63 @ 0x15c63
void sub_15c63(unsigned long a0,long a1,int *a2)
{
  sub_15b23(a0,a1,a2,(int *)0x49944);
}

// Function: sub_15c98 @ 0x15c98
void sub_15c98(unsigned long a0,long a1,int *a2)
{
  sub_15b23(a0,a1,a2,(int *)0x49948);
}

// Function: sub_15ccd @ 0x15ccd
unsigned long sub_15ccd(struct_58 *a0,long a1,int *a2,unsigned int a3) // return-dupe
{
  int v1;
  long v10; // stack - 0x20
  unsigned long v11; // stack - 0x18
  char v2; // al
  unsigned long v3; // stack - 0x48
  unsigned int v4; // stack - 0x28
  unsigned int v5; // xmm0_da
  long v6; // stack - 0x40
  long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned int v9; // stack - 0x24
  
  v1 = *a2;
  if (!sub_145ab(a1,a2,&v3))
    return 0;
  v8 = dat_49970;
  v7 = dat_49968 + 0x15180;
  v4 = a3;
  v2 = sub_1916a(v3,&v4,v7,dat_49970,"arithmetic overflow while converting %s minutes to a number of seconds");
  if (!v2) {
    *a2 = v1;
    return 0;
  }
  v6 = sub_eecc(a0,v3);
  *(unsigned long *)(v6 + 0x38) = CONCAT44(v9,v4);
  *(long *)(v6 + 0x40) = v10;
  *(unsigned long *)(v6 + 0x48) = v11;
  sub_144ce(v10);
  *(unsigned int *)(v6 + 0x24) = v5;
  return 1;
}

// Function: sub_15df0 @ 0x15df0
void sub_15df0(struct_58 *a0,long a1,int *a2)
{
  sub_15ccd(a0,a1,a2,0);
}

// Function: sub_15e23 @ 0x15e23
void sub_15e23(struct_58 *a0,long a1,int *a2)
{
  sub_15ccd(a0,a1,a2,2);
}

// Function: sub_15e56 @ 0x15e56
void sub_15e56(struct_58 *a0,long a1,int *a2)
{
  sub_15ccd(a0,a1,a2,3);
}

// Function: sub_15e89 @ 0x15e89
unsigned long sub_15e89(struct_58 *a0,long a1,int *a2) // return-dupe
{
  int v1;
  char v2; // al
  unsigned long v3; // stack - 0x20
  unsigned int v4; // xmm0_da
  long v5; // stack - 0x18
  
  v1 = *a2;
  if (!sub_145ab(a1,a2,&v3))
    return 0;
  sub_15686();
  v2 = sub_1573c("-name","-wholename",v3);
  if (!v2) {
    *a2 = v1;
    return 0;
  }
  v5 = sub_eecc(a0,v3);
  *(char *)(v5 + 0x1b) = 0;
  *(char *)(v5 + 0x1a) = *(char *)(v5 + 0x1b);
  *(unsigned long *)(v5 + 0x38) = v3;
  sub_15576(v3,0);
  *(unsigned int *)(v5 + 0x24) = v4;
  return 1;
}

// Function: sub_15f82 @ 0x15f82
unsigned long sub_15f82(int *a0)
{
  struct_55 *v1; // rax
  
  v1 = (struct_55 *)sub_e7dd(a0,0);
  v1->field_0x0 = sub_9b30;
  v1->field_0x10 = 2;
  v1->field_0x14 = 4;
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  return 1;
}

// Function: sub_15ff1 @ 0x15ff1
bool sub_15ff1(struct_58 *a0,long a1,int *a2)
{
  long v1; // rax
  char v2 [88];
  unsigned long v3; // stack - 0xc8
  bool v4; // zf
  unsigned int v5; // xmm0_da
  char v6 [16];
  long v7; // stack - 0xc0
  long v8; // stack - 0x60
  
  sub_f26e(v2);
  v4 = sub_14611(a1,a2,v2,&v3) != '\0';
  if (v4) {
    v1 = sub_eecc(a0,v3);
    v7 = v1;
    v6 = sub_30a80(v2);
    *(long *)(v1 + 0x40) = SUB168(v6,0);
    *(long *)(v1 + 0x48) = SUB168(v6,8);
    *(unsigned int *)(v7 + 0x38) = 3;
    *(unsigned int *)(v7 + 0x3c) = 0;
    sub_144ce(v8);
    *(unsigned int *)(v7 + 0x24) = v5;
  }
  return v4;
}

// Function: sub_160f6 @ 0x160f6
long * sub_160f6(struct_58 *a0,long a1,int *a2) // early-return x4
{
  char v1;
  char v2;
  char *v3;
  long *v4;
  int v5; // eax
  long *v6; // rax
  char v7 [6]; // stack - 0x26
  char v8 [146];
  unsigned int v9; // xmm0_da
  
  if ((!a1) || (v6 = (long)(long)*a2, v6 = (long *)(a1 + (long)v6 * 8), !*v6)) {
    v6 = NULL;
    return v6;
  }
  v6 = (long)(long)*a2;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v6 = (unsigned long)strlen((char *)*v6);
  if (v6 != (long *)0x8) {
    v6 = NULL;
    return v6;
  }
  builtin_strncpy(v7,"aBcmt",6);
  v6 = (long)(long)*a2;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v5 = strncmp("-newer",(char *)*v6,6);
  if (v5)
    __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY"); // no-return
  v6 = (long)(long)*a2;
  v6 = (long *)(a1 + (long)v6 * 8);
  v1 = *(char *)(*v6 + 6);
  v6 = (long)(long)*a2;
  v6 = (long *)(a1 + (long)v6 * 8);
  v2 = *(char *)(*v6 + 7);
  if ((v1 == 'B') || (v2 == 'B')) {
    v6 = (char *)gettext("This system does not provide a way to find the birth time of a file.");
    error(0,0,(char *)v6);
    v6 = NULL;
    return v6;
  }
  if (((v1 == 't') || (v6 = (char *)strchr(v7,(int)v1), !v6)) || (v6 = (char *)strchr(v7,(int)v2), !v6)) {
    v6 = NULL;
    return v6;
  }
  v5 = *a2 + 1;
  v6 = (long)(long)v5;
  v6 = (long *)(a1 + (long)v6 * 8);
  if (!*v6) {
    v6 = (long)(long)*a2;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v6 = (char *)sub_2f861(0,dat_499ac,(char *)*v6);
    v3 = (char *)v6;
    v6 = (char *)gettext("The %s test needs an argument");
    error(1,0,(char *)v6,v3);
    return v6;
  }
  *a2 = *a2 + 1;
  v6 = (long)(long)*a2;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v6 = (long *)sub_eecc(a0,*v6);
  v4 = v6;
  if (v1 != 'm') { // branch-flip
    if ('m' < v1) {
label_16427:
      v6 = (char *)strchr(v7,(int)v1);
      if (!v6)
        __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY"); // no-return
      __assert_fail("0","parser.c",0x64c,"parse_newerXY"); // no-return
    }
    if (v1 != 'c') { // branch-flip
      if ('c' < v1) goto label_16427;
      if (v1 != 'B') { // branch-flip
        if (v1 != 'a') goto label_16427;
        *(unsigned int *)&v6[7] = 0;
      }
      else {
        *(unsigned int *)&v6[7] = 1;
      }
    }
    else {
      *(unsigned int *)&v6[7] = 2;
    }
  }
  else {
    *(unsigned int *)&v6[7] = 3;
  }
  if (v2 != 't') { // branch-flip
    sub_f26e(v8);
    v6 = (long)(long)*a2;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v5 = (*dat_49990)(*v6,v8);
    if (v5) {
      v6 = (long)(long)*a2;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v3 = (char *)*v6;
      v6 = (int *)__errno_location();
      sub_10a44(*(int *)v6,v3); // no-return
    }
    v5 = sub_140b3(v8,(int)v2,(struct_70 *)&v4[8]);
    if (!v5) {
      v6 = (long)(long)*a2;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v6 = (char *)sub_1096a(0,(char *)*v6);
      v3 = (char *)v6;
      v6 = (char *)gettext("Cannot obtain birth time of file %s");
      error(1,0,(char *)v6,v3);
      return v6;
    }
  }
  else {
    v6 = (long)(long)*a2;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v3 = (char *)*v6;
    if (sub_2dcb0((struct_76 *)&v4[8],v3,(long *)0x49958) != '\x01') {
      v6 = (long)(long)*a2;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v6 = (char *)sub_2f861(0,dat_499ac,(char *)*v6);
      v3 = (char *)v6;
      v6 = (char *)gettext("I cannot figure out how to interpret %s as a date or time");
      error(1,0,(char *)v6,v3);
      return v6;
    }
  }
  *(unsigned int *)((long)v4 + 0x3c) = 0;
  sub_144ce(v4[8]);
  *(unsigned int *)((long)v4 + 0x24) = v9;
  *a2 = *a2 + 1;
  if (!*v4)
    __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY"); // no-return
  if ((void *)*v4 == sub_9c02) {
    if (*(char *)((long)v4 + 0x1a)) {
      v6 = (long *)0x1;
      return v6;
    }
    __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY"); // no-return
  }
  __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"); // no-return
}

// Function: sub_1674d @ 0x1674d
void sub_1674d(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994c = 1;
  sub_17bae(a0,a1,a2);
}

// Function: sub_16785 @ 0x16785
unsigned long sub_16785(struct_58 *a0)
{
  *(unsigned int *)(sub_eecc(a0,0) + 0x24) = dat_3c060;
  return 1;
}

// Function: sub_167ca @ 0x167ca
unsigned long sub_167ca(struct_58 *a0)
{
  *(unsigned int *)(sub_ef32(a0) + 0x24) = dat_3c054;
  return 1;
}

// Function: sub_1680a @ 0x1680a
void sub_1680a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_49950 = 0;
  sub_17bae(a0,a1,a2);
}

// Function: sub_16842 @ 0x16842
void sub_16842(struct_74 *a0,long a1,int *a2)
{
  sub_1893a("-ok",a0,a1,a2);
}

// Function: sub_1687a @ 0x1687a
void sub_1687a(struct_74 *a0,long a1,int *a2)
{
  sub_1893a("-okdir",a0,a1,a2);
}

// Function: sub_168b2 @ 0x168b2
unsigned long sub_168b2(int *a0)
{
  struct_55 *v1; // rax
  
  v1 = (struct_55 *)sub_e7dd(a0,0);
  v1->field_0x0 = sub_9f83;
  v1->field_0x10 = 4;
  v1->field_0x14 = 0;
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  return 1;
}

// Function: sub_16921 @ 0x16921
unsigned long sub_16921(int *a0)
{
  struct_55 *v1; // rax
  
  v1 = (struct_55 *)sub_e644(a0);
  v1->field_0x0 = sub_9f9e;
  v1->field_0x10 = 3;
  v1->field_0x14 = 2;
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  return 1;
}

// Function: sub_1698b @ 0x1698b
unsigned long sub_1698b(char *a0,char a1) // return-dupe x2
{
  char *v1; // rax
  
  v1 = strrchr(a0,0x2f);
  if ((v1) && (!v1[1])) {
    if (sub_bb9d(a0,(unsigned char)a1))
      return 1;
    return 0;
  }
  return 1;
}

// Function: sub_169f6 @ 0x169f6
unsigned long sub_169f6(struct_57 *a0,long a1,int *a2,char *a3,void *a4) // return-dupe
{
  char v1; // al
  char *v2; // stack - 0x30
  unsigned int v3; // xmm0_da
  long v4; // stack - 0x28
  
  sub_15686();
  if (sub_145ab(a1,a2,&v2)) {
    v4 = sub_ee5a(a0,a4,v2);
    *(char *)(v4 + 0x1b) = 0;
    *(char *)(v4 + 0x1a) = *(char *)(v4 + 0x1b);
    *(char **)(v4 + 0x38) = v2;
    v3 = sub_15576(v2,0);
    *(unsigned int *)(v4 + 0x24) = v3;
    if ((dat_49951 != '\x01') && (v1 = sub_1698b(v2,a4 == sub_986a), v1 != '\x01')) {
      error(0,0,gettext("warning: -%s %s will not match anything because it ends with /."),a3,v2);
      *(unsigned int *)(v4 + 0x24) = dat_3c064;
    }
    return 1;
  }
  return 0;
}

// Function: sub_16b4b @ 0x16b4b
void sub_16b4b(struct_57 *a0,long a1,int *a2)
{
  sub_169f6(a0,a1,a2,"path",sub_a012);
}

// Function: sub_16b8a @ 0x16b8a
void sub_16b8a(struct_57 *a0,long a1,int *a2)
{
  sub_169f6(a0,a1,a2,"wholename",sub_a012);
}

// Function: sub_16bc9 @ 0x16bc9
void sub_16bc9(struct_57 *a0,long a1,int *a2)
{
  sub_169f6(a0,a1,a2,"ipath",sub_986a);
}

// Function: sub_16c08 @ 0x16c08
void sub_16c08(struct_57 *a0,long a1,int *a2)
{
  sub_169f6(a0,a1,a2,"iwholename",sub_986a);
}

// Function: sub_16c47 @ 0x16c47
unsigned long sub_16c47(struct_58 *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v10; // stack - 0x24
  unsigned long v2; // rax
  char *v3; // stack - 0x40
  int v4; // stack - 0x28
  unsigned int v5; // stack - 0x4c
  int v6; // stack - 0x48
  unsigned int v7; // stack - 0x44
  char *v8; // stack - 0x38
  long v9; // stack - 0x30
  
  if (sub_145ab(a1,a2,&v3) != '\x01')
    return 0;
  if (*v3 != '-') { // branch-flip
    if (*v3 != '/') { // branch-flip
      v6 = 0;
      v7 = 2;
      v5 = dat_3c02c;
    }
    else {
      v6 = 1;
      v7 = 1;
      v5 = dat_3c034;
    }
  }
  else {
    v6 = 1;
    v7 = 0;
    v5 = dat_3c048;
  }
  v8 = (char *)sub_25a35(&v3[v6]);
  if ((v8) && (((*v3 != '+' || (v3[1] <= '/')) || ('8' <= v3[1])))) {
    v4 = sub_25f2e(0,0,0,v8,NULL);
    v10 = sub_25f2e(0,1,0,v8,NULL);
    free(v8);
    v1 = v3;
    if (((*v3 == '/') && (!v4)) && (!v10)) {
      error(0,0,gettext("warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files."),v1);
      v7 = 0;
      v5 = dat_3c068;
    }
    v9 = sub_eecc(a0,v3);
    *(unsigned int *)(v9 + 0x24) = v5;
    *(unsigned int *)(v9 + 0x38) = v7;
    memcpy((void *)(v9 + 0x3c),&v4,8);
    return 1;
  }
  v1 = (char *)sub_2f861(0,dat_499ac,v3);
  error(1,0,gettext("invalid mode %s"),v1);
  return v2;
}

// Function: sub_16eb3 @ 0x16eb3
unsigned long sub_16eb3(struct_58 *a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  sub_19a85((struct_60 *)(v1 + 0x38));
  return 1;
}

// Function: sub_16f20 @ 0x16f20
void sub_16f20(struct_58 *a0)
{
  sub_14fd8(a0,NULL);
}

// Function: sub_16f4b @ 0x16f4b
unsigned long sub_16f4b(struct_57 *a0,long a1,int *a2) // return-dupe x2
{
  int v1;
  unsigned long v2; // stack - 0x40
  char v3 [40];
  
  v1 = *a2;
  if (!sub_14528(a1,a2,&v2))
    return 0;
  sub_19a85(v3);
  if (sub_1159f(v3,a0,v2))
    return 1;
  *a2 = v1;
  return 0;
}

// Function: sub_16fef @ 0x16fef
unsigned long sub_16fef(struct_57 *a0,long a1,int *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  char v3 [40];
  int v4; // stack - 0x4c
  
  v4 = *a2;
  if ((sub_145ab(a1,a2,&v1)) && (sub_14528(a1,a2,&v2))) {
    sub_19964(v1,v3);
    v4 = *a2;
    if (sub_1159f(v3,a0,v2))
      return 1;
  }
  *a2 = v4;
  return 0;
}

// Function: sub_170b7 @ 0x170b7
unsigned long sub_170b7(struct_58 *a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  if (dat_49940 != '\x01') {
    *(char *)(v1 + 0x1b) = 0;
    *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  }
  *(char *)(v1 + 0x18) = 1;
  *(char *)(v1 + 0x19) = 0;
  return 1;
}

// Function: sub_1711b @ 0x1711b
unsigned long sub_1711b(struct_58 *a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(char *)(v1 + 0x18) = 1;
  *(char *)(v1 + 0x19) = 0;
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}

// Function: sub_17182 @ 0x17182
unsigned long sub_17182(unsigned long a0,long a1,int *a2) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x18
  
  if (sub_145ab(a1,a2,&v2)) {
    dat_4999c = sub_1ca8f(v2);
    v1 = sub_17bae(a0,a1,a2);
    return v1;
  }
  return 0;
}

// Function: sub_1720a @ 0x1720a
void sub_1720a(struct_57 *a0,long a1,int *a2)
{
  sub_1723e(a1,a2,a0,dat_4999c);
}

// Function: sub_1723e @ 0x1723e
unsigned long sub_1723e(long a0,int *a1,struct_57 *a2,int a3) // return-dupe
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3; // stack - 0x40
  unsigned int v4; // xmm0_da
  long v5; // stack - 0x38
  re_pattern_buffer *v6; // stack - 0x30
  char *v7; // stack - 0x28
  
  if (!sub_145ab(a0,a1,&v3))
    return 0;
  v5 = sub_ee5a(a2,sub_a2da,v3);
  *(char *)(v5 + 0x1b) = 0;
  *(char *)(v5 + 0x1a) = *(char *)(v5 + 0x1b);
  v6 = (re_pattern_buffer *)sub_31bc2(0x40);
  *(re_pattern_buffer **)(v5 + 0x38) = v6;
  *(unsigned long *)&v6->field_0x8 = 100;
  v1 = *(unsigned long *)&v6->field_0x8;
  *(unsigned long *)v6 = sub_31bc2(v1);
  *(unsigned long *)&v6->field_0x20 = 0;
  re_set_syntax((long)a3);
  *(long *)&v6->field_0x18 = (long)a3;
  *(unsigned long *)&v6->field_0x28 = 0;
  v1 = strlen(v3);
  v7 = re_compile_pattern(v3,v1,v6);
  if (!v7) {
    sub_15576(v3,1);
    *(unsigned int *)(v5 + 0x24) = v4;
    return 1;
  }
  error(1,0,gettext("failed to compile regular expression \'%s\': %s"),v3,v7);
  return v2;
}

// Function: sub_173d3 @ 0x173d3
unsigned long sub_173d3(struct_58 *a0,long a1,int *a2) // return-dupe
{
  char v1;
  char v10; // stack - 0x4d
  int v11; // stack - 0x48
  int v12; // stack - 0x44
  char *v13; // stack - 0x38
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  int v6; // stack - 0x4c
  long v7; // stack - 0x40
  char v8 [2]; // stack - 0x22
  undefined4 v9;
  
  v11 = 0x200;
  if ((!a1) || (!*(long *)(a1 + (long)*a2 * 8)))
    return 0;
  v13 = *(char **)(a1 + (long)*a2 * 8);
  v12 = (int)strlen(v13);
  if (!v12) {
    error(1,0,gettext("invalid null argument to -size"));
    return v2;
  }
  v10 = v13[(long)v12 + -1];
  if (v10 != 'w') { // branch-flip
    if ('w' < v10) {
label_175d2:
      v1 = *(char *)((long)v12 + -1 + *(long *)(a1 + (long)*a2 * 8));
      error(1,0,gettext("invalid -size type `%c\'"),v1);
      return v3;
    }
    if (v10 != 'k') { // branch-flip
      if ('k' < v10) goto label_175d2;
      if (v10 != 'c') { // branch-flip
        if ('c' < v10) goto label_175d2;
        if (v10 != 'b') { // branch-flip
          if ('b' < v10) goto label_175d2;
          if (v10 != 'M') { // branch-flip
            if ('M' < v10) goto label_175d2;
            if (':' <= v10) { // branch-flip
              if (v10 != 'G') goto label_175d2;
              v11 = 0x40000000;
              v13[(long)v12 + -1] = '\0';
            }
            else {
              if (v10 <= '/') goto label_175d2;
              v10 = '\0';
            }
          }
          else {
            v11 = 0x100000;
            v13[(long)v12 + -1] = '\0';
          }
        }
        else {
          v11 = 0x200;
          v13[(long)v12 + -1] = '\0';
        }
      }
      else {
        v11 = 1;
        v13[(long)v12 + -1] = '\0';
      }
    }
    else {
      v11 = 0x400;
      v13[(long)v12 + -1] = '\0';
    }
  }
  else {
    v11 = 2;
    v13[(long)v12 + -1] = '\0';
  }
  v1 = sub_19722(v13,&v7,&v6);
  if (v1 == '\x01') {
    v5 = sub_eecc(a0,v13);
    *(int *)(v5 + 0x38) = v6;
    *(int *)(v5 + 0x3c) = v11;
    *(long *)(v5 + 0x40) = v7;
    *(char *)(v5 + 0x1a) = 1;
    *(char *)(v5 + 0x1b) = 0;
    if (v6) { // branch-flip
      if (v6 != 1) // branch-flip
        *(unsigned int *)(v5 + 0x24) = dat_3c02c;
      else {
        if (0x5001 <= (unsigned long)(v7 * v11)) // branch-flip
          v9 = s_3c06b._1_4_;
        else {
          v9 = dat_3c028;
        }
        *(unsigned int *)(v5 + 0x24) = v9;
      }
    }
    else {
      if (0x5001 <= (unsigned long)(v7 * v11)) // branch-flip
        v9 = dat_3c028;
      else {
        v9 = s_3c06b._1_4_;
      }
      *(unsigned int *)(v5 + 0x24) = v9;
    }
    *a2 = *a2 + 1;
    return 1;
  }
  v8[0] = v10;
  v8[1] = 0;
  error(1,0,gettext("Invalid argument `%s%s\' to -size"),v13,v8);
  return v4;
}

// Function: sub_1777e @ 0x1777e
unsigned long sub_1777e(struct_58 *a0,long a1,int *a2) // return-dupe, ternary
{
  int v1; // eax
  undefined8 v10; // stack - 0x130
  undefined8 v11; // stack - 0x128
  undefined8 v12; // stack - 0x120
  undefined8 v13; // stack - 0x118
  undefined8 v14; // stack - 0x110
  undefined8 v15; // stack - 0x108
  undefined8 v16; // stack - 0x100
  undefined8 v17; // stack - 0xf8
  undefined8 v18; // stack - 0xf0
  undefined8 v19; // stack - 0xe8
  long v2; // rax
  undefined8 v20; // stack - 0xe0
  undefined8 v21; // stack - 0xd8
  undefined8 v22; // stack - 0xd0
  undefined8 v23; // stack - 0xc8
  undefined8 v24; // stack - 0xc0
  long v3; // stack - 0x148
  unsigned long v4 [2]; // stack - 0x158
  stat v5; // stack - 0xb8
  int v6; // stack - 0x160
  unsigned int v7; // stack - 0x15c
  long v8; // stack - 0x140
  undefined8 v9; // stack - 0x138
  
  sub_f26e(&v3);
  if (sub_14611(a1,a2,&v3,v4) != '\x01')
    return 0;
  sub_f26e(&v5);
  v7 = 0;
  if (dat_49988) // branch-flip
    v6 = -1;
  else if (dat_49998) { // branch-flip
    v7 = 0x20000;
    v6 = -1;
  }
  else {
    v6 = ((v10 & 0xf000) != 0xa000) ? -1 : -2; // branch-flip
  }
  if (v6 == -3)
    __assert_fail("fd != -3","parser.c",0x8bf,"parse_samefile"); // no-return
  if (v6 == -1) {
    v6 = sub_1b433(v4[0],v7);
    if (0 <= v6) {
      if (fstat(v6,&v5))
        sub_10a44(*__errno_location(),v4[0]); // no-return
      v1 = (*dat_49990)(v4[0],&v3);
      if (v1)
        sub_10a44(*__errno_location(),v4[0]); // no-return
      if ((dat_49988) || (dat_49998 == '\x01')) {
        v3 = v5._0_8_;
        v8 = v5._8_8_;
        v9 = v5._16_8_;
        v10 = v5._24_8_;
        v11 = v5._32_8_;
        v12 = v5._40_8_;
        v13 = v5._48_8_;
        v14 = v5._56_8_;
        v15 = v5._64_8_;
        v16 = v5._72_8_;
        v17 = v5._80_8_;
        v18 = v5._88_8_;
        v19 = v5._96_8_;
        v20 = v5._104_8_;
        v21 = v5._112_8_;
        v22 = v5._120_8_;
        v23 = v5._128_8_;
        v24 = v5._136_8_;
      }
      else if ((v10 & 0xf000) != 0xa000) { // branch-flip
        if ((v3 != v5._0_8_) || (v8 != v5._8_8_)) {
          close(v6);
          v6 = -1;
        }
      }
      else {
        close(v6);
        v6 = -1;
      }
    }
  }
  v2 = sub_eecc(a0,v4[0]);
  *(long *)(v2 + 0x38) = v8;
  *(long *)(v2 + 0x40) = v3;
  *(int *)(v2 + 0x48) = v6;
  *(char *)(v2 + 0x1b) = 0;
  *(char *)(v2 + 0x1a) = 1;
  *(unsigned int *)(v2 + 0x24) = dat_3c02c;
  return 1;
}

// Function: sub_17b57 @ 0x17b57
unsigned long sub_17b57(struct_58 *a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}

// Function: sub_17bae @ 0x17bae
void sub_17bae(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_17b57((struct_58 *)sub_14033(),a1,a2);
}

// Function: sub_17be3 @ 0x17be3
unsigned long sub_17be3(struct_58 *a0) // return-dupe
{
  struct_78 *v1; // rax
  
  v1 = (struct_78 *)sub_ef32(a0);
  v1->field_0x1b = 0;
  v1->field_0x1a = v1->field_0x1b;
  v1->field_0x19 = 0;
  v1->field_0x18 = v1->field_0x19;
  if (v1->field_0x0 == sub_a0fa) {
    v1->field_0x24 = dat_3c048;
    return 1;
  }
  v1->field_0x24 = s_3c06b._1_4_;
  return 1;
}

// Function: sub_17c77 @ 0x17c77
void sub_17c77(struct_57 *a0,long a1,int *a2)
{
  sub_183bf(a1,a2,a0,sub_a4b2);
}

// Function: sub_17cac @ 0x17cac
unsigned long sub_17cac(struct_58 *a0,long a1,int *a2) // return-dupe
{
  long v1; // rax
  unsigned int v2;
  
  v1 = sub_197bf(a1,a2,a0);
  if (v1) {
    if (100 <= *(unsigned long *)(v1 + 0x40)) // branch-flip
      v2 = dat_3c048;
    else {
      v2 = dat_3c044;
    }
    *(unsigned int *)(v1 + 0x24) = v2;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}

// Function: sub_17d2c @ 0x17d2c
char * sub_17d2c(struct_58 *a0,long a1,int *a2) // return-dupe
{
  char v1; // al
  unsigned long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  char *v2; // rax
  unsigned long v3; // stack - 0x60
  unsigned long v4; // stack - 0x38
  unsigned int v5; // xmm0_da
  char *v6; // stack - 0x58
  long v7; // stack - 0x50
  unsigned long v8; // stack - 0x48
  unsigned long v9; // stack - 0x40
  
  v6 = "arithmetic overflow while converting %s days to a number of seconds";
  v1 = sub_145ab(a1,a2,&v3);
  if (!v1) {
    v2 = NULL;
    return v2;
  }
  v8 = 0;
  v9 = 0;
  v1 = sub_1916a(v3,&v4,0,0,v6);
  if (v1) {
    v2 = (long)sub_eecc(a0,v3);
    v7 = (long)v2;
    *(unsigned long *)((long)v2 + 0x38) = v4;
    *(unsigned long *)((long)v2 + 0x40) = v10;
    *(unsigned long *)((long)v2 + 0x48) = v11;
    sub_1446f();
    *(unsigned int *)(v7 + 0x24) = v5;
    v2 = (char *)0x1;
    return v2;
  }
  v2 = gettext("Invalid argument %s to -used");
  error(1,0,v2,v3);
  return v2;
}

// Function: sub_17e8d @ 0x17e8d
unsigned long sub_17e8d(struct_58 *a0,long a1,int *a2) // return-dupe
{
  char *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  long v4; // rax
  char *v5; // stack - 0x40
  unsigned int v6;
  unsigned int v7; // stack - 0x44
  passwd *v8; // stack - 0x38
  unsigned long v9; // stack - 0x30
  
  if (!sub_145ab(a1,a2,&v5))
    return 0;
  v8 = getpwnam(v5);
  endpwent();
  if (v8) // branch-flip
    v7 = *(unsigned int *)&v8->field_0x10;
  else {
    v9 = strspn(v5,"0123456789");
    if ((!v9) || (v5[v9])) {
      if (*v5) {
        v1 = (char *)sub_2f861(0,dat_499ac,v5);
        error(1,0,gettext("%s is not the name of a known user"),v1);
        return v2;
      }
      error(1,0,gettext("The argument to -user should not be empty"));
      return v3;
    }
    v7 = sub_1cd12(v5,dat_499ac);
  }
  v4 = sub_eecc(a0,v5);
  *(unsigned int *)(v4 + 0x38) = v7;
  if (100 <= *(unsigned int *)(v4 + 0x38)) // branch-flip
    v6 = dat_3c048;
  else {
    v6 = dat_3c044;
  }
  *(unsigned int *)(v4 + 0x24) = v6;
  return 1;
}

// Function: sub_1802c @ 0x1802c
void sub_1802c(void) // ternary
{
  char *v1; // rax
  unsigned int v2; // stack - 0x14
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  sub_1b6b6("find");
  printf(gettext("Features enabled: "));
  printf("D_TYPE ");
  v1 = (dat_49998) ? "enabled" : "disabled"; // branch-flip
  printf("O_NOFOLLOW(%s) ",v1);
  printf("LEAF_OPTIMISATION ");
  if (1 <= is_selinux_enabled())
    printf("SELINUX ");
  v2 = 0;
  if (sub_8b12(&v2)) {
    printf("FTS(");
    if (v2 & 0x200)
      printf("FTS_CWDFD");
    printf(") ");
  }
  printf("CBO(level=%d) ",(unsigned long)dat_499a8);
  printf("\n");
  exit(0); // no-return
}

// Function: sub_181cd @ 0x181cd
long * sub_181cd(struct_58 *a0,long a1,int *a2) // return-dupe
{
  long v1;
  long *v2; // rax
  
  if ((a1) && (v2 = (long)(long)*a2, v2 = (long *)(a1 + (long)v2 * 8), *v2)) {
    if (1 <= is_selinux_enabled()) {
      v2 = (long)sub_eecc(a0,0);
      v1 = (long)v2;
      *(unsigned int *)((long)v2 + 0x24) = dat_3c02c;
      *(char *)((long)v2 + 0x1a) = 0;
      v2 = (long)(long)*a2;
      v2 = (unsigned long *)(a1 + (long)v2 * 8);
      *(long *)(v1 + 0x38) = *v2;
      *a2 = *a2 + 1;
      v2 = (long *)0x1;
      return v2;
    }
    v2 = (char *)gettext("invalid predicate -context: SELinux is not enabled.");
    error(1,0,(char *)v2);
    return v2;
  }
  v2 = NULL;
  return v2;
}

// Function: sub_182aa @ 0x182aa
void sub_182aa(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994d = 1;
  sub_17bae(a0,a1,a2);
}

// Function: sub_182e2 @ 0x182e2
void sub_182e2(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994e = 1;
  sub_17bae(a0,a1,a2);
}

// Function: sub_1831a @ 0x1831a
void sub_1831a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994e = 0;
  sub_17bae(a0,a1,a2);
}

// Function: sub_18352 @ 0x18352
void sub_18352(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_49950 = 1;
  sub_17bae(a0,a1,a2);
}

// Function: sub_1838a @ 0x1838a
void sub_1838a(struct_57 *a0,long a1,int *a2)
{
  sub_183bf(a1,a2,a0,sub_a78e);
}

// Function: sub_183bf @ 0x183bf
char * sub_183bf(long a0,int *a1,struct_57 *a2,void *a3) // return-dupe, ternary
{
  char v1;
  char *v2;
  char *v3; // rax
  char *v4; // stack - 0x38
  unsigned int v5; // stack - 0x40
  float v6; // stack - 0x3c
  char *v7; // stack - 0x30
  long v8; // stack - 0x28
  
  v3 = (a3 != sub_a78e) ? "-type" : "-xtype"; // branch-flip
  v7 = v3;
  if (sub_145ab(a0,a1,&v4) != '\x01') {
    v3 = NULL;
    return v3;
  }
  if (!*v4) {
    v3 = gettext("Arguments to %s should contain at least one letter");
    error(1,0,v3,v7);
    return v3;
  }
  v3 = (long)sub_ee5a(a2,a3,v4);
  v8 = (long)v3;
  *(unsigned int *)((long)v3 + 0x24) = 0;
  if (a3 != sub_a78e) { // branch-flip
    *(char *)((long)v3 + 0x1a) = 0;
    *(char *)((long)v3 + 0x1b) = 1;
  }
  else {
    *(char *)((long)v3 + 0x1a) = 1;
    *(char *)((long)v3 + 0x1b) = 0;
  }
  do {
    do {
      v2 = v4;
      if (!*v4) {
        v3 = (char *)0x1;
        return v3;
      }
      switch(*v4) {
        case 0x44:
          v1 = *v4;
          v3 = gettext("%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.");
          error(1,0,v3,v7,(unsigned long)(unsigned int)(int)v1);
          v1 = *v4; // return-dupe
          v3 = gettext("Unknown argument to %s: %c");
          error(1,0,v3,v7,v1);
          return v3;
        default:
          v1 = *v4;
          v3 = gettext("Unknown argument to %s: %c");
          error(1,0,v3,v7,v1);
          return v3;
        case 0x62:
          v5 = 0;
          v6 = dat_3c078;
          break;
        case 99:
          v5 = 1;
          v6 = dat_3c07c;
          break;
        case 100:
          v5 = 2;
          v6 = dat_3c080;
          break;
        case 0x66:
          v5 = 3;
          v6 = dat_3c084;
          break;
        case 0x6c:
          v5 = 4;
          v6 = dat_3c088;
          break;
        case 0x70:
          v5 = 5;
          v6 = dat_3c08c;
          break;
        case 0x73:
          v5 = 6;
          v6 = dat_3c090;
        
      }
      v3 = (unsigned long)(unsigned long)v5;
      if (*(char *)(v8 + 0x38 + (long)v3)) {
        v1 = *v4;
        v3 = gettext("Duplicate file type \'%c\' in the argument list to %s.");
        error(1,0,v3,v1,v7);
        return v3;
      }
      *(float *)(v8 + 0x24) = *(float *)(v8 + 0x24) + v6;
      v3 = (unsigned long)(unsigned long)v5;
      *(char *)(v8 + 0x38 + (long)v3) = 1;
      v3 = &v4[1];
      v4 = v3;
    } while (!*v3);
    if (*v3 != ',') {
      v3 = gettext("Must separate multiple arguments to %s using: \',\'");
      error(1,0,v3,v7);
      return v3;
    }
    v3 = &v2[2];
    v4 = v3;
  } while (*v3);
  v3 = gettext("Last file type in list argument to %s is missing, i.e., list is ending on: \',\'");
  error(1,0,v3,v7);
  return v3;
}

// Function: sub_1876f @ 0x1876f
unsigned long sub_1876f(FILE *a0) // early-return
{
  int v1; // eax
  
  v1 = fileno(a0);
  if (v1 == -1)
    return 0;
  v1 = isatty(v1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 != 0);
}

// Function: sub_187ac @ 0x187ac
void sub_187ac(unsigned long a0) // return-dupe
{
  char v1; // al
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x40
  long v5; // stack - 0x48
  char *v6; // stack - 0x38
  char *v7; // stack - 0x30
  char *v8; // stack - 0x28
  
  v6 = getenv("PATH");
  v7 = ":";
  if (!v6)
    return;
  sub_1cf56(v6,":",1,&v5,&v4);
  do {
    if ((!v4) || ((v4 == 1 && (v6[v5] == '.')))) {
      error(1,0,gettext("The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)"),a0);
      return;
    }
    if (v6[v5] != '/') {
      v2 = strndup(&v6[v5],v4);
      v8 = v2;
      if (!v2)
        v2 = &v6[v5];
      v3 = sub_1096a(0,v2);
      error(1,0,gettext("The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH"),v3,a0);
      return;
    }
    v1 = sub_1cf56(v6,v7,0,&v5,&v4);
  } while (v1);
}

// Function: sub_1893a @ 0x1893a
unsigned long sub_1893a(unsigned long a0,struct_74 *a1,long a2,int *a3) // early-return x2, ternary
{
  int v1;
  char *v10; // rax
  char *v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  char v14 [24];
  int v15; // stack - 0x90
  int v16; // stack - 0x8c
  int v17; // stack - 0x84
  char *v18; // stack - 0x78
  char *v19; // stack - 0x70
  void *v2;
  bool v3;
  bool v4;
  int v5;
  void *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  char *v9;
  
  v2 = a1->field_0x18;
  if ((!a2) || (!*(long *)(a2 + (long)*a3 * 8)))
    return 0;
  v6 = (void *)sub_ee5a(a1,v2,"(some -exec* arguments)");
  *(char *)((long)v6 + 0x19) = 1;
  *(char *)((long)v6 + 0x18) = *(char *)((long)v6 + 0x19);
  *(char *)((long)v6 + 0x1a) = 0;
  *(char *)((long)v6 + 0x1b) = *(char *)((long)v6 + 0x1a);
  *(unsigned long *)((long)v6 + 0xf8) = 0;
  if ((v2 != sub_9f27) && (v2 != sub_9ece)) {
    v3 = 1;
    *(char *)((long)v6 + 0xf4) = 0;
  }
  else {
    v3 = 0;
    dat_499b8 = 1;
    *(char *)((long)v6 + 0xf4) = 1;
  }
  if ((v2 != sub_9497) && (v2 != sub_9f27)) {
    if (!dat_49a00)
      __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok"); // no-return
    *(long *)((long)v6 + 0xf8) = dat_49a00;
  }
  else {
    *(unsigned long *)((long)v6 + 0xf8) = 0;
    dat_4994e = 0;
    sub_187ac(a0);
  }
  *(char *)((long)v6 + 0x38) = 0;
  v1 = *a3;
  v4 = 0;
  v17 = 0;
  v18 = NULL;
  v15 = v1;
  while( true ) {
    if ((!*(long *)(a2 + (long)v15 * 8)) || ((**(char **)(a2 + (long)v15 * 8) == ';' && (!*(char *)(*(long *)(a2 + (long)v15 * 8) + 1))))) goto label_18cc6;
    if ((v3) && (((**(char **)(a2 + (long)v15 * 8) == '+' && (!*(char *)(*(long *)(a2 + (long)v15 * 8) + 1))) && (v4)))) break;
    v4 = 0;
    v9 = *(char **)(a2 + (long)v15 * 8);
    if (sub_24cc3(v9,"{}")) {
      v4 = 1;
      v18 = *(char **)(a2 + (long)v15 * 8);
      v17 += 1;
      if ((!v15) && ((v2 == sub_9497 || (v2 == sub_9f27)))) {
        error(1,0,gettext("You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem."));
        return v7;
      }
    }
    v15 += 1;
  }
  *(char *)((long)v6 + 0x38) = 1;
label_18cc6:
  if ((v15 == v1) || (!*(long *)(a2 + (long)v15 * 8))) {
    *a3 = v15;
    free(v6);
    return 0;
  }
  if (*(char *)((long)v6 + 0x38)) {
    v19 = (v2 != sub_9497) ? "" : "dir"; // branch-flip
    if (2 <= v17) {
      error(1,0,gettext("Only one instance of {} is supported with -exec%s ... +"),v19);
      return v8;
    }
    if (strlen(v18) != 2) {
      if (0x14 <= (unsigned int)snprintf(v14,0x13,"-exec%s ... {} +",v19))
        __assert_fail("needed <= MsgBufSize","parser.c",0xbc6,"insert_exec_ok"); // no-return
      v9 = (char *)sub_2f861(2,dat_499ac,v18);
      v10 = (char *)sub_2f861(1,dat_499ac,"{}");
      v11 = (char *)sub_2f861(0,dat_499ac,v14);
      error(1,0,gettext("In %s the %s must appear by itself, but you specified %s"),v11,v10,v9);
      return v12;
    }
  }
  v5 = sub_1a58c((struct_72 *)((long)v6 + 0x40),0x800);
  if ((v5) && ((unsigned int)(v5 - 1U) <= 1)) {
    error(1,0,gettext("The environment is too large for exec()."));
    return v13;
  }
  sub_1a6e3((void *)((long)v6 + 0x40));
  *(void **)((long)v6 + 0x80) = sub_b7ae;
  if (*(char *)((long)v6 + 0x38)) { // branch-flip
    *(unsigned long *)((long)v6 + 0xe8) = 0;
    *(unsigned long *)((long)v6 + 0x70) = 0;
    *(unsigned long *)((long)v6 + 0x68) = 0;
    *(unsigned long *)((long)v6 + 0x88) = 0;
    *(unsigned long *)((long)v6 + 0x90) = 0;
    *(long *)((long)v6 + 0x78) = (long)((v15 - v1) + -1);
    sub_1a742((struct_58 *)((long)v6 + 0x40),(struct_73 *)((long)v6 + 0x98),(long)v6 + 0x38);
    for (v16 = v1; v16 < v15 + -1; v16 = v16 + 1) {
      v9 = *(char **)(a2 + (long)v16 * 8);
      sub_1a1b8((struct_13 *)((long)v6 + 0x40),(struct_12 *)((long)v6 + 0x98),*(char **)(a2 + (long)v16 * 8),strlen(v9) + 1,NULL,0,1);
    }
  }
  else {
    *(int *)((long)v6 + 0xf0) = v15 - v1;
    *(char **)((long)v6 + 0x70) = "{}";
    v9 = *(char **)((long)v6 + 0x70);
    *(unsigned long *)((long)v6 + 0x68) = strlen(v9);
    *(unsigned long *)((long)v6 + 0x88) = 0;
    *(unsigned long *)((long)v6 + 0x90) = 0;
    v5 = *(int *)((long)v6 + 0xf0);
    *(unsigned long *)((long)v6 + 0xe8) = sub_31bc2((long)v5 << 3);
    sub_1a742((struct_58 *)((long)v6 + 0x40),(struct_73 *)((long)v6 + 0x98),(long)v6 + 0x38);
    for (v16 = 0; v16 < *(int *)((long)v6 + 0xf0); v16 = v16 + 1) {
      *(unsigned long *)(*(long *)((long)v6 + 0xe8) + (long)v16 * 8) = *(unsigned long *)((long)(v1 + v16) * 8 + a2);
    }
  }
  if (*(long *)(a2 + (long)v15 * 8)) // branch-flip
    *a3 = v15 + 1;
  else {
    *a3 = v15;
  }
  return 1;
}

// Function: sub_1916a @ 0x1916a
unsigned long sub_1916a(unsigned long a0,void *a1,long a2,long a3,char *a4) // return-dupe x2, return-dupe
{
  unsigned int v1;
  char v2; // al
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x30
  char v5 [8];
  double v6; // stack - 0x20
  double v7; // xmm0_qa
  double v8; // stack - 0x18
  
  v4 = a0;
  v2 = sub_196ab(&v4,(unsigned int *)((long)a1 + 4));
  if (!v2) {
    v3 = 0;
    return v3;
  }
  v1 = *(unsigned int *)((long)a1 + 4);
  if ((v1 != 2) && (v1 <= 2)) {
    if (v1) { // branch-flip
      if (v1 == 1)
        *(unsigned int *)((long)a1 + 4) = 0;
    }
    else {
      *(unsigned int *)((long)a1 + 4) = 1;
    }
  }
  v2 = sub_3223a(v4,NULL,v5,dat_48fe8);
  if (!v2) {
    v3 = 0;
    return v3;
  }
  modf(&v6);
  v8 = dat_3c098 * v7;
  if ((double)dat_3c000 <= v8)
    __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp"); // no-return
  *(long *)((long)a1 + 8) = (long)((double)a2 - v6);
  v2 = v6 < 0.0 != a2 < *(long *)((long)a1 + 8);
  if ((bool)v2) {
    error(1,0,a4,v4);
    return v3;
  }
  *(long *)((long)a1 + 0x10) = (long)((double)a3 - v8);
  if (v8 <= (double)a3) {
    v3 = 1;
    return v3;
  }
  *(long *)((long)a1 + 0x10) = *(long *)((long)a1 + 0x10) + dat_3c000;
  *(long *)((long)a1 + 8) = *(long *)((long)a1 + 8) + -1;
  v3 = 1;
  return v3;
}

// Function: sub_1938f @ 0x1938f
char * sub_1938f(struct_58 *a0,long a1,int *a2) // return-dupe x2, ternary x2
{
  char v1; // al
  char *v10; // stack - 0x58
  unsigned long v11; // stack - 0x50
  long v12; // stack - 0x48
  long v13; // stack - 0x40
  long v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  int v16; // stack - 0x24
  long v17; // stack - 0x20
  unsigned long v18; // stack - 0x18
  char *v2; // rax
  char *v3; // rdx
  unsigned long v4; // stack - 0x68
  int v5; // stack - 0x70
  unsigned int v6; // stack - 0x28
  unsigned long v7; // stack - 0x60
  unsigned int v8; // xmm0_da
  int v9; // stack - 0x6c
  
  v2 = gettext("arithmetic overflow while converting %s days to a number of seconds");
  v10 = v2;
  v9 = *a2;
  v1 = sub_145ab(a1,a2,&v4);
  if (v1 != '\x01') {
    v2 = NULL;
    return v2;
  }
  v11 = v4;
  v14 = dat_49968;
  v15 = dat_49970;
  v1 = sub_196ab(&v4,&v5);
  if ((v1) && (v5 == 1)) {
    v2 = (long)(v14 + 0x1517f);
    v12 = (long)v2;
    v2 = (long)(v14 + 0x1517f);
    v14 = (long)v2;
  }
  v4 = v11;
  v1 = sub_1916a(v11,&v6,v14,v15,v10);
  if (v1 != '\x01') {
    *a2 = v9;
    v2 = NULL;
    return v2;
  }
  v2 = (long)sub_eecc(a0,v11);
  v13 = (long)v2;
  *(unsigned long *)((long)v2 + 0x38) = CONCAT44(v16,v6);
  *(long *)((long)v2 + 0x40) = v17;
  *(unsigned long *)((long)v2 + 0x48) = v18;
  sub_144ce(v17);
  *(unsigned int *)(v13 + 0x24) = v8;
  if (!(dat_49980 & 1)) {
    v2 = (char *)0x1;
    return v2;
  }
  fprintf(stderr,"inserting %s\n",*(char **)(v13 + 8));
  if (v16) { // branch-flip
    if (v16 != 1) { // branch-flip
      v3 = (v16 != 2) ? " ?" : ">="; // branch-flip
    }
    else {
      v3 = " <";
    }
  }
  else {
    v3 = " >";
  }
  if (v16) { // branch-flip
    if (v16 != 1) { // branch-flip
      v2 = (v16 != 2) ? "?" : "eq"; // branch-flip
    }
    else {
      v2 = "lt";
    }
  }
  else {
    v2 = "gt";
  }
  fprintf(stderr,"    type: %s    %s  ",v2,v3);
  v7 = *(unsigned long *)(v13 + 0x40);
  v2 = ctime(&v7);
  fprintf(stderr,"%lu %s",*(unsigned long *)(v13 + 0x40),v2);
  if (v16 != 2) {
    v2 = (char *)0x1;
    return v2;
  }
  v2 = (unsigned long)(*(long *)(v13 + 0x40) + 0x15180);
  v7 = (unsigned long)v2;
  v2 = ctime(&v7);
  fprintf(stderr,"                 <  %lu %s",v7,v2);
  v2 = (char *)0x1;
  return v2;
}

// Function: sub_196ab @ 0x196ab
unsigned long sub_196ab(long *a0,unsigned int *a1) // return-dupe x2
{
  if (*(char *)*a0 == '+') {
    *a1 = 0;
    *a0 = *a0 + 1;
    return 1;
  }
  if (*(char *)*a0 != '-') {
    *a1 = 2;
    return 1;
  }
  *a1 = 1;
  *a0 = *a0 + 1;
  return 1;
}

// Function: sub_19722 @ 0x19722
unsigned long sub_19722(long a0,unsigned long *a1,unsigned int *a2) // early-return x2
{
  unsigned long v1; // rax
  long v2; // stack - 0x20
  char v3 [8];
  
  if (!a0)
    return 0;
  v2 = a0;
  if ((a2) && (sub_196ab(&v2,a2) != '\x01'))
    return 0;
  v1 = sub_3291e(v2,v3,10,a1,"");
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}

// Function: sub_197bf @ 0x197bf
char * sub_197bf(long a0,int *a1,struct_58 *a2) // early-return, ternary x2
{
  char v1; // al
  char *v2; // rax
  char *v3; // rdx
  unsigned long v4; // stack - 0x28
  int v5; // stack - 0x2c
  unsigned long v6; // stack - 0x20
  long v7; // stack - 0x18
  
  v1 = sub_145ab(a0,a1,&v4);
  if ((v1) && (v1 = sub_19722(v4,&v6,&v5), v1)) {
    v2 = (long)sub_eecc(a2,v4);
    v7 = (long)v2;
    *(int *)((long)v2 + 0x38) = v5;
    *(unsigned long *)((long)v2 + 0x40) = v6;
    if (dat_49980 & 1) {
      fprintf(stderr,"inserting %s\n",*(char **)((long)v2 + 8));
      if (v5) { // branch-flip
        if (v5 != 1) { // branch-flip
          v3 = (v5 != 2) ? " ?" : " ="; // branch-flip
        }
        else {
          v3 = " <";
        }
      }
      else {
        v3 = " >";
      }
      if (v5) { // branch-flip
        if (v5 != 1) { // branch-flip
          v2 = (v5 != 2) ? "?" : "eq"; // branch-flip
        }
        else {
          v2 = "lt";
        }
      }
      else {
        v2 = "gt";
      }
      fprintf(stderr,"    type: %s    %s  ",v2,v3);
      fprintf(stderr,"%lu\n",*(unsigned long *)(v7 + 0x40));
    }
    v2 = (char *)v7;
    return v2;
  }
  v2 = NULL;
  return v2;
}

// Function: sub_19964 @ 0x19964
void sub_19964(char *a0,struct_60 *a1)
{
  FILE *v1;
  
  a1->field_0x0 = 0;
  a1->field_0x20 = sub_2de20(NULL);
  if (strcmp(a0,"/dev/stderr")) { // branch-flip
    if (strcmp(a0,"/dev/stdout")) { // branch-flip
      a1->field_0x8 = (FILE *)sub_10ce6(dat_499f0,a0);
      a1->field_0x10 = a0;
      if (!a1->field_0x8)
        sub_10aa8(*__errno_location(),a0); // no-return
    }
    else {
      a1->field_0x8 = stdout;
      a1->field_0x10 = gettext("standard output");
    }
  }
  else {
    a1->field_0x8 = stderr;
    a1->field_0x10 = gettext("standard error");
  }
  v1 = a1->field_0x8;
  a1->field_0x18 = sub_1876f(v1);
}

// Function: sub_19a85 @ 0x19a85
void sub_19a85(struct_60 *a0)
{
  sub_19964("/dev/stdout",a0);
}

// Function: sub_19aae @ 0x19aae
void sub_19aae(struct_13 *a0,struct_12 *a1)
{
  sub_1a1b8(a0,a1,dat_49720,0,NULL,0,0);
}

// Function: sub_19af7 @ 0x19af7
void sub_19af7(void *a0,struct_12 *a1,char *a2,unsigned long a3,char *a4,long a5,char *a6,long a7,unsigned int a8)
{
  long v1; // rax
  unsigned long v2; // stack - 0x48
  char *v3; // stack - 0x40
  char *v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  v5 = *(long *)((long)a0 + 0x18) - 1;
  if (!dat_4a1e0)
    dat_4a1e0 = (char *)sub_31bc2(*(long *)((long)a0 + 0x18) + 1);
  v4 = dat_4a1e0;
  v2 = a3;
  v3 = a2;
  do {
    v1 = sub_24cc3(v3,*(char **)((long)a0 + 0x30));
    if (v1) // branch-flip
      v6 = v1 - (long)v3;
    else {
      v6 = v2;
    }
    if (v5 <= v6) break;
    v5 -= v6;
    strncpy(v4,v3,v6);
    v4 = &v4[v6];
    v3 = &v3[v6];
    v2 -= v6;
    if (v1) {
      if (v5 <= (unsigned long)(a5 + a7)) break;
      v5 -= a5 + a7;
      if (a4) {
        strcpy(v4,a4);
        v4 = &v4[a5];
      }
      strcpy(v4,a6);
      v4 = &v4[a7];
      v3 = &v3[*(long *)((long)a0 + 0x28)];
      v2 -= *(long *)((long)a0 + 0x28);
    }
  } while (*v3);
  if (!*v3) {
    *v4 = '\0';
    sub_1a1b8(a0,a1,dat_4a1e0,&v4[1 - (long)dat_4a1e0],NULL,0,a8);
    return;
  }
  error(1,0,gettext("command too long"));
}

// Function: sub_19cee @ 0x19cee
unsigned long sub_19cee(struct_7 *a0,void *a1,bool a2,unsigned long a3) // ternary x2
{
  unsigned long v1; // stack - 0x38
  
  if (a2) { // branch-flip
    if (*(unsigned long *)((long)a1 + 0x40) < a3)
      *(unsigned long *)((long)a1 + 0x40) = a3;
  }
  else if ((a3 < *(unsigned long *)((long)a1 + 0x48)) || (!*(long *)((long)a1 + 0x48)))
    *(unsigned long *)((long)a1 + 0x48) = a3;
  if ((*(long *)((long)a1 + 0x40)) && (*(unsigned long *)((long)a1 + 0x40) < *(unsigned long *)((long)a1 + 0x48))) {
    v1 = (unsigned long)(*(long *)((long)a1 + 0x48) - *(long *)((long)a1 + 0x40)) >> 1;
    if (a2) { // branch-flip
      v1 = (v1) ? a3 + v1 : a3 + 1; // branch-flip
    }
    else {
      v1 = (v1) ? a3 - v1 : a3 - 1; // branch-flip
    }
  }
  else if (a2) { // branch-flip
    v1 = a3;
    if (a3 != 0xffffffffffffffff)
      v1 = a3 + 1;
  }
  else {
    v1 = a3 >> 1;
  }
  if ((a0->field_0x38) && (v1 <= a0->field_0x38 + 1U))
    v1 = a0->field_0x38 + 1;
  if (!v1)
    v1 = 1;
  return v1;
}

// Function: sub_19e2b @ 0x19e2b
unsigned long sub_19e2b(struct_9 *a0,struct_8 *a1,long a2,unsigned long a3,long a4)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v3 = 0;
  v4 = 0;
  while (v4 < a0->field_0x38) {
    v1 = v4 + 1;
    v2 = v3 + 1;
    *(unsigned long *)(v3 * 8 + a2) = *(unsigned long *)(a1->field_0x8 + v4 * 8);
    v3 = v2;
    v4 = v1;
  }
  v4 += a4;
  while ((v4 < a1->field_0x0 && (v3 < a3))) {
    v1 = v4 + 1;
    v2 = v3 + 1;
    *(unsigned long *)(v3 * 8 + a2) = *(unsigned long *)(a1->field_0x8 + v4 * 8);
    v3 = v2;
    v4 = v1;
  }
  if (v3 < a0->field_0x38)
    __assert_fail("dst_pos >= ctl->initial_argc","buildcmd.c",0xf2,"copy_args"); // no-return
  *(unsigned long *)(a2 + v3 * 8) = 0;
  return v3;
}

// Function: sub_19f5a @ 0x19f5a
void sub_19f5a(void *a0,struct_14 *a1)
{
  void *v1;
  unsigned long v2;
  void *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  sub_19aae(a0,a1);
  if (!a1->field_0x0)
    __assert_fail("state->cmd_argc > 0","buildcmd.c",0x105,"bc_do_exec"); // no-return
  if (*(long *)(a1->field_0x0 * 8 + -8 + a1->field_0x8))
    __assert_fail("state->cmd_argv[state->cmd_argc-1] == NULL","buildcmd.c",0x106,"bc_do_exec"); // no-return
  v3 = (void *)sub_31bc2((a1->field_0x0 + 1) * 8);
  v6 = 0;
  v5 = a1->field_0x0;
  do {
    v4 = sub_19e2b(a0,a1,v3,v5,v6);
    v1 = *(void **)((long)a0 + 0x40);
    v2 = a1->field_0x30;
    if ((*v1)(a0,v2,v4 & 0xffffffff,v3)) { // branch-flip
      v5 = sub_19cee(a0,a1,1,v5);
      v6 += v4 - *(long *)((long)a0 + 0x38);
    }
    else {
      if (v5 <= *(long *)((long)a0 + 0x38) + 1U) {
        error(1,0,gettext("can\'t call exec() due to argument size restrictions"));
        return;
      }
      v5 = sub_19cee(a0,a1,0,v5);
    }
  } while (v6 + 1U < a1->field_0x0 - *(long *)((long)a0 + 0x38));
  free(v3);
  sub_1a840(a0,a1);
}

// Function: sub_1a152 @ 0x1a152
bool sub_1a152(int a0,void *a1,long *a2) // early-return
{
  if (((!a0) && (*(long *)((long)a1 + 0x50))) && (*a2 - *(long *)((long)a1 + 0x38) == *(long *)((long)a1 + 0x50)))
    return 1;
  return *a2 == *(long *)((long)a1 + 0x20);
}

// Function: sub_1a1b8 @ 0x1a1b8
void sub_1a1b8(struct_13 *a0,struct_12 *a1,char *a2,long a3,char *a4,long a5,int a6) // return-dupe
{
  unsigned long v1;
  void *v2;
  int v3; // eax
  bool v4; // zf
  
  v4 = a2 != dat_49720;
  if (!a2)
    __assert_fail("arg != NULL","buildcmd.c",0x155,"bc_push_arg"); // no-return
  if (v4) {
    if (a0->field_0x18 < a1->field_0x20 + a3 + a5) {
      if ((a6) || (a1->field_0x0 == a0->field_0x38)) {
        error(1,0,gettext("cannot fit single argument within argument list size limit"));
        return;
      }
      if ((a0->field_0x30) || ((a0->field_0x0 && ((a0->field_0x48 || (a0->field_0x50)))))) {
        error(1,0,gettext("argument list too long"));
        return;
      }
      sub_19f5a(a0,a1);
    }
    v3 = sub_1a152(a6,a0,a1);
    if (v3)
      sub_19f5a(a0,a1);
  }
  if (!a6)
    a1->field_0x38 = 1;
  if (a1->field_0x10 <= a1->field_0x0) {
    if (a1->field_0x8) { // branch-flip
      a1->field_0x10 = a1->field_0x10 * 2;
      v1 = a1->field_0x10;
      v2 = (void *)a1->field_0x8;
      a1->field_0x8 = sub_31c2c(v2,v1 * 8);
    }
    else {
      a1->field_0x10 = 0x40;
      v1 = a1->field_0x10;
      a1->field_0x8 = sub_31bc2(v1 << 3);
    }
  }
  if (v4) {
    v1 = a1->field_0x0;
    a1->field_0x0 = v1 + 1;
    *(unsigned long *)(v1 * 8 + a1->field_0x8) = a1->field_0x18 + a1->field_0x20;
    if (a4) {
      strcpy((char *)(a1->field_0x18 + a1->field_0x20),a4);
      a1->field_0x20 = a1->field_0x20 + a5;
    }
    strcpy((char *)(a1->field_0x18 + a1->field_0x20),a2);
    a1->field_0x20 = a1->field_0x20 + a3;
    v3 = sub_1a152(a6,a0,a1);
    if (v3)
      sub_19f5a(a0,a1);
  }
  else {
    v1 = a1->field_0x0;
    a1->field_0x0 = v1 + 1;
    *(unsigned long *)(v1 * 8 + a1->field_0x8) = 0;
  }
  if (!a6)
    return;
  a1->field_0x28 = a1->field_0x20;
}

// Function: sub_1a4e9 @ 0x1a4e9
long sub_1a4e9(void) // early-return
{
  long v1; // rax
  
  v1 = sysconf(0);
  if (1 <= v1)
    return v1;
  return 0x7fffffffffffffff;
}

// Function: sub_1a51c @ 0x1a51c
unsigned long sub_1a51c(void)
{
  return 0;
}

// Function: sub_1a53a @ 0x1a53a
long sub_1a53a(void)
{
  long *v1;
  char *v2;
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v3 = 0;
  v4 = environ;
  while (*v4) {
    v1 = &v4[1];
    v2 = (char *)*v4;
    v3 += strlen(v2) + 1;
    v4 = v1;
  }
  return v3;
}

// Function: sub_1a58c @ 0x1a58c
unsigned long sub_1a58c(struct_72 *a0,long a1) // return-dupe x2
{
  unsigned long v1; // rax
  
  v1 = sub_1a53a();
  a0->field_0x10 = 0x1000;
  a0->field_0x8 = sub_1a4e9();
  a0->field_0x0 = 0;
  if ((unsigned long)a0->field_0x8 < v1)
    return 1;
  if ((unsigned long)a0->field_0x8 <= a1 + v1)
    return 2;
  a0->field_0x8 = a0->field_0x8 - v1;
  a0->field_0x8 = a0->field_0x8 - a1;
  a0->field_0x20 = ((unsigned long)a0->field_0x8 >> 3) - 2;
  if (a0->field_0x20) {
    a0->field_0x28 = 0;
    a0->field_0x30 = 0;
    a0->field_0x38 = 0;
    a0->field_0x40 = sub_1a51c;
    a0->field_0x48 = 0;
    a0->field_0x50 = 0;
    a0->field_0x18 = a0->field_0x8;
    return 0;
  }
  __assert_fail("ctl->max_arg_count > 0","buildcmd.c",0x206,"bc_init_controlinfo"); // no-return
}

// Function: sub_1a6e3 @ 0x1a6e3
void sub_1a6e3(void *a0) // return-dupe x2
{
  if (*(unsigned long *)((long)a0 + 8) <= 0x1ffff) {
    *(unsigned long *)((long)a0 + 0x18) = *(unsigned long *)((long)a0 + 8);
    return;
  }
  if (0x20001 <= *(unsigned long *)((long)a0 + 0x10)) {
    *(unsigned long *)((long)a0 + 0x18) = *(unsigned long *)((long)a0 + 0x10);
    return;
  }
  *(unsigned long *)((long)a0 + 0x18) = 0x20000;
}

// Function: sub_1a742 @ 0x1a742
void sub_1a742(struct_58 *a0,struct_73 *a1,unsigned long a2)
{
  long v1;
  
  a1->field_0x0 = 0;
  a1->field_0x20 = 0;
  a1->field_0x8 = 0;
  a1->field_0x10 = 0;
  a1->field_0x40 = 0;
  a1->field_0x48 = 0;
  if (0x7ffffffffffff800 <= (unsigned long)a0->field_0x18)
    __assert_fail("ctl->arg_max <= (LONG_MAX - 2048L)","buildcmd.c",0x23c,"bc_init_state"); // no-return
  v1 = a0->field_0x18;
  a1->field_0x18 = sub_31bc2(v1 + 1);
  a1->field_0x28 = 0;
  a1->field_0x20 = a1->field_0x28;
  a1->field_0x38 = 0;
  a1->field_0x3c = 0xffffffff;
  a1->field_0x30 = a2;
}

// Function: sub_1a840 @ 0x1a840
void sub_1a840(struct_11 *a0,struct_10 *a1)
{
  a1->field_0x0 = a0->field_0x38;
  a1->field_0x20 = a1->field_0x28;
  a1->field_0x38 = 0;
  a1->field_0x3c = 0xffffffff;
}

// Function: sub_1a888 @ 0x1a888
unsigned char * sub_1a888(char *a0,unsigned long a1) // return-dupe
{
  char *v1;
  int v2; // eax
  unsigned char *v3; // rax
  unsigned long v4; // stack - 0x30
  char v5 [8];
  unsigned char *v6; // stack - 0x28
  
  v3 = (unsigned char *)getenv(a0);
  if (!v3) {
    v3 = NULL;
    return v3;
  }
  v6 = v3;
  v2 = sub_323e9(v3,v5,10,&v4,NULL);
  if (!v2) {
    if (a1 <= v4) {
      v3 = NULL;
      return v3;
    }
    v3 = (unsigned char *)0x1;
    return v3;
  }
  v3 = (char *)gettext("Environment variable %s is not set to a valid decimal number");
  v1 = (char *)v3;
  v3 = (int *)__errno_location();
  error(1,*(int *)v3,v1,a0);
  return v3;
}

// Function: sub_1a94d @ 0x1a94d
unsigned int sub_1a94d(long *a0)
{
  char *v1;
  long *v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v4 = 0;
  v3 = 0;
  for (v2 = a0; *v2; v2 = &v2[1]) {
    v4 += 1;
    v1 = (char *)*v2;
    v3 += strlen(v1);
  }
  if ((!sub_1a888("__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT",v4)) && (!sub_1a888("__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT",v3)))
    return 0;
  return 1;
}

// Function: sub_1a9dd @ 0x1a9dd
int sub_1a9dd(struct_21 *a0,void *a1,struct_22 *a2)
{
  int v1; // eax
  int v2;
  char v3 [24];
  
  if (sub_2fd99(v3))
    sub_26fd4(*__errno_location()); // no-return
  if (!sub_2fe17(a0)) {
    v1 = (*a1)(a2);
    if (0 <= v1) // branch-flip
      v2 = 0;
    else {
      v2 = *__errno_location();
    }
    if (!sub_2fe17(v3)) {
      sub_2fe52(v3);
      if (v2)
        *__errno_location() = v2;
      return v1;
    }
    sub_27011(*__errno_location()); // no-return
  }
  sub_27011(*__errno_location()); // no-return
}

// Function: sub_1aacc @ 0x1aacc
int * sub_1aacc(int a0,void *a1,unsigned long a2) // early-return
{
  int v1;
  unsigned int v2;
  int v3;
  int *v4; // rax
  char v5 [24];
  
  if (a0 != -100) { // branch-flip
    v3 = sub_2fd99(v5);
    if (v3) {
      v4 = __errno_location();
      sub_26fd4(*v4); // no-return
    }
    v3 = fchdir(a0);
    if (v3) {
      v4 = __errno_location();
      v1 = *v4;
      sub_2fe52(v5);
      v4 = __errno_location();
      *v4 = v1;
      v4 = (int *)0xffffffff;
      return v4;
    }
    v3 = (*a1)(a2);
    v2 = v3;
    if (0 <= v3) // branch-flip
      v3 = 0;
    else {
      v4 = __errno_location();
      v3 = *v4;
    }
    v1 = v3;
    v3 = sub_2fe17(v5);
    if (v3) {
      v4 = __errno_location();
      sub_27011(*v4); // no-return
    }
    sub_2fe52(v5);
    if (v1) {
      v4 = __errno_location();
      *v4 = v1;
    }
    v4 = (unsigned long)(unsigned long)v2;
  }
  else {
    v4 = (unsigned long)(*a1)(a2);
  }
  return v4;
}

// Function: sub_1abd7 @ 0x1abd7
unsigned long sub_1abd7(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  if (a0) // branch-flip
    v1 = a0;
  else {
    v1 = 0x10;
  }
  while( true ) {
    if (a1 <= v1)
      return v1;
    if (v1 * 2 < v1) break;
    v1 <<= 1;
  }
  return a1;
}

// Function: sub_1ac29 @ 0x1ac29
void * sub_1ac29(void *a0,unsigned long a1,unsigned long *a2)
{
  unsigned long v1; // rax
  int v2; // stack - 0x1c
  void *v3; // stack - 0x18
  
  v2 = *__errno_location();
  if (!a1)
    __assert_fail("wanted > 0u","extendbuf.c",0x47,"extendbuf"); // no-return
  v1 = sub_1abd7(*a2,a1);
  if (*a2) { // branch-flip
    if (v1 != *a2) { // branch-flip
      *a2 = v1;
      v3 = realloc(a0,v1);
      if (!v3)
        v2 = *__errno_location();
    }
    else {
      v3 = a0;
    }
  }
  else {
    if (a0)
      __assert_fail("NULL == existing","extendbuf.c",0x4f,"extendbuf"); // no-return
    *a2 = v1;
    v3 = malloc(v1);
  }
  if (v3)
    *__errno_location() = v2;
  return v3;
}

// Function: sub_1ad4d @ 0x1ad4d
long sub_1ad4d(void *a0,unsigned long a1,unsigned long *a2)
{
  long v1; // rax
  
  v1 = sub_1ac29(a0,a1,a2);
  if (!v1) {
    free(a0);
    sub_321fa(); // no-return
  }
  return v1;
}

// Function: sub_1ad9e @ 0x1ad9e
int sub_1ad9e(void) // return-dupe
{
  bool v1;
  int v2; // eax
  DIR *v3; // rax
  dirent *v4; // rax
  char *v5; // rax
  int v6; // stack - 0x3c
  
  v6 = -1;
  v3 = (DIR *)sub_1d78f("/proc/self/fd");
  if (!v3)
    return -1;
  v1 = 0;
  while( true ) {
    *__errno_location() = 0;
    v4 = readdir(v3);
    if (!v4) break;
    if ((v4->field_0x13 != '.') || ((v4->field_0x14 && ((v4->field_0x14 != '.' || (v4->field_0x15)))))) {
      v2 = sub_1cd12(&v4->field_0x13,0);
      if (v6 < v2)
        v6 = v2;
      v1 = 1;
    }
  }
  if (*__errno_location()) {
    v5 = (char *)sub_2f861(0,8,"/proc/self/fd");
    error(0,*__errno_location(),"%s",v5);
    v1 = 0;
  }
  closedir(v3);
  if (!v1)
    return -1;
  return v6;
}

// Function: sub_1aee1 @ 0x1aee1
long sub_1aee1(void)
{
  int v1; // eax
  long v2; // rax
  long v3 [3]; // stack - 0x28
  long v4; // stack - 0x30
  
  v1 = sub_1ad9e();
  v2 = (long)v1;
  if (v2 < 0) {
    v2 = sysconf(4);
    v4 = v2;
    if (v2 == -1)
      v4 = 0x14;
    v1 = getrlimit(7,v3);
    v2 = v4;
    if ((v1) || (v3[0] == -1)) {
      return v2;
    }
    v2 = v3[0];
  }
  return v2;
}

// Function: sub_1af76 @ 0x1af76
int sub_1af76(int a0,int a1,void *a2,unsigned long a3)
{
  int v1; // eax
  long v2;
  long v3;
  int v4; // stack - 0x22c
  int v5; // stack - 0x228
  int v6; // stack - 0x224
  int v7; // stack - 0x220
  
  v4 = a0;
  while( true ) {
    if (a1 <= v4)
      return 0;
    v6 = a1 - v4;
    if (0x41 <= v6)
      v6 = 0x40;
    for (v5 = 0; v5 < v6; v5 = v5 + 1) {
      *(unsigned short *)&(&v2)[(long)v5 * 2] = 5;
      *(unsigned short *)((long)&v2 + ((long)v5 * 4 + 1) * 2) = 0;
      (&v2)[(long)v5 * 2 + -1] = v4 + v5;
    }
    if (poll(&v3,(long)v6,0) == -1) break;
    for (v7 = 0; v7 < v6; v7 = v7 + 1) {
      if ((*(short *)((long)&v2 + ((long)v7 * 4 + 1) * 2) != 0x20) && (v1 = (*a2)((&v2)[(long)v7 * 2 + -1],a3), v1))
        return v1;
    }
    v4 += v6;
  }
  return -1;
}

// Function: sub_1b128 @ 0x1b128
unsigned int sub_1b128(unsigned int a0)
{
  return sub_1db8e(a0,1) & 1;
}

// Function: sub_1b156 @ 0x1b156
unsigned long sub_1b156(unsigned int a0,struct_65 *a1) // return-dupe x2
{
  void *v1; // rax
  
  if (sub_1b128(a0))
    return 0;
  v1 = (void *)sub_1ac29(a1->field_0x0,(a1->field_0x8 + 1) * 4,&a1[1]);
  if (v1) {
    a1->field_0x0 = v1;
    *(unsigned int *)((long)a1->field_0x0 + a1->field_0x8 * 4) = a0;
    a1->field_0x8 = a1->field_0x8 + 1;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_1b20a @ 0x1b20a
void sub_1b20a(void)
{
  unsigned long v1; // stack - 0x28
  int v2; // stack - 0x2c
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v2 = sub_1aee1();
  v1 = 0;
  v4 = 0;
  v3 = 0;
  if (v2 != 0x7fffffff)
    v2 += 1;
  sub_1af76(0,v2,sub_1b156,&v1);
  dat_4a1e8 = v1;
  dat_4a1f0 = v3;
}

// Function: sub_1b29c @ 0x1b29c
unsigned long sub_1b29c(int a0,struct_63 *a1) // return-dupe x2
{
  if (sub_1b128(a0))
    return 0;
  do {
    if (a1->field_0x8 <= (unsigned long)a1->field_0x10) {
label_1b33a:
      a1->field_0x18 = a0;
      return 0xffffffff;
    }
    if (a0 <= *(int *)(a1->field_0x10 * 4 + a1->field_0x0)) {
      if (a0 == *(int *)(a1->field_0x10 * 4 + a1->field_0x0))
        return 0;
      goto label_1b33a;
    }
    a1->field_0x10 = a1->field_0x10 + 1;
  } while( true );
}

// Function: sub_1b352 @ 0x1b352
unsigned int sub_1b352(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x38
  int v2; // stack - 0x3c
  unsigned long v3; // stack - 0x30
  unsigned long v4; // stack - 0x28
  unsigned int v5; // stack - 0x20
  
  v2 = sub_1aee1();
  if (v2 != 0x7fffffff)
    v2 += 1;
  v4 = 0;
  v5 = 0xffffffff;
  v1 = a0;
  v3 = a1;
  sub_1af76(0,v2,sub_1b29c,&v1);
  return v5;
}

// Function: sub_1b3df @ 0x1b3df
bool sub_1b3df(void)
{
  int v1; // eax
  bool v2; // stack - 0xd
  
  v2 = 0;
  v1 = sub_1e034("/",0x80000);
  if (0 <= v1) {
    v2 = sub_1b128(v1) != 0;
    close(v1);
  }
  return v2;
}

// Function: sub_1b433 @ 0x1b433
int sub_1b433(char *a0,unsigned int a1)
{
  int v1; // eax
  unsigned int v2; // stack - 0xe0
  unsigned int v3; // stack - 0xa8
  
  v2 = 0;
  if (a1 & 0x40)
    v2 = v3;
  if (dat_4a1f8 != '\x01') {
    dat_4a1f9 = sub_1b3df();
    dat_4a1f8 = '\x01';
  }
  v1 = sub_1e034(a0,a1 | 0x80000,v2);
  if ((0 <= v1) && (dat_4a1f9 != '\x01'))
    sub_1d5ba(v1,1);
  return v1;
}

// Function: sub_1b5c4 @ 0x1b5c4
void sub_1b5c4(void)
{
  free(dat_4a1e8);
  dat_4a1e8 = NULL;
  dat_4a1f0 = 0;
}

// Function: sub_1b5f4 @ 0x1b5f4
bool sub_1b5f4(void)
{
  return getenv("GNU_FINDUTILS_FD_LEAK_CHECK") != NULL;
}

// Function: sub_1b61e @ 0x1b61e
void sub_1b61e(void)
{
  int v1; // eax
  
  v1 = sub_1b352(dat_4a1e8,dat_4a1f0);
  if (v1 <= -1)
    return;
  error(0,0,gettext("File descriptor %d will leak; please report this as a bug, remembering to include a detailed description of the simplest way to reproduce this problem."),v1);
  __assert_fail("no_leaks","fdleak.c",0x18c,"complain_about_leaky_fds"); // no-return
}

// Function: sub_1b6b6 @ 0x1b6b6
void sub_1b6b6(char *a0)
{
  char *v1; // rax
  char *v2; // rax
  
  sub_1e1a8(stderr);
  v1 = gettext("Kevin Dalley");
  v2 = gettext("James Youngman");
  sub_319e2(stdout,a0,"GNU findutils",dat_49728,gettext("Eric B. Decker"),v2,v1,0);
}

// Function: sub_1b74e @ 0x1b74e
unsigned long sub_1b74e(FILE *a0,unsigned long a1,int *a2) // return-dupe
{
  int v1; // eax
  
  v1 = fprintf(a0,"%*lu",*a2,a1);
  if (v1 <= -1)
    return 0;
  if (v1 <= *a2)
    return 1;
  *a2 = v1;
  return 1;
}

// Function: sub_1b7b3 @ 0x1b7b3
void sub_1b7b3(unsigned char *a0,unsigned int a1,unsigned long a2,void *a3,long a4,int a5,unsigned int a6,FILE *a7)
{
  unsigned long v1;
  char v10 [12];
  char v11 [664];
  char v12 [256];
  char *v13;
  FILE **v14; // rsp
  FILE **v15;
  long v16; // stack - 0x450
  void *v17; // stack - 0x448
  unsigned long v18; // stack - 0x440
  int v19; // stack - 0x438
  long v2;
  unsigned int v20; // stack - 0x434
  unsigned char *v21; // stack - 0x430
  char v22; // stack - 0x425
  unsigned int v23; // stack - 0x424
  int v24; // stack - 0x420
  int v25; // stack - 0x41c
  int v26; // stack - 0x418
  int v27; // stack - 0x414
  int v28; // stack - 0x410
  int v29; // stack - 0x40c
  char v3; // al
  char *v30; // stack - 0x408
  unsigned long v31; // stack - 0x400
  char *v32; // stack - 0x3f8
  char *v33; // stack - 0x3f0
  tm *v34; // stack - 0x3e8
  char *v35; // stack - 0x3e0
  char *v36; // stack - 0x3d8
  unsigned char *v37; // stack - 0x3d0
  unsigned long v4;
  unsigned char *v5; // rax
  char *v6; // rax
  int v7;
  unsigned long v8; // rdx
  FILE *v9; // stack - 0x458
  
  v14 = &v9;
  v9 = a7;
  v22 = '\x01';
  v23 = 0;
  v16 = a4;
  v17 = a3;
  v18 = a2;
  v19 = a5;
  v20 = a1;
  v21 = a0;
  sub_1e2a6(*(unsigned int *)((long)a3 + 0x18),v10);
  v4 = *(unsigned long *)((long)v17 + 8);
  v24 = fprintf(v9,"%*s",dat_49730,(char *)sub_22eb1(v4,v11,0,1,1));
  if (0 <= v24) { // branch-flip
    if (dat_49730 < v24)
      dat_49730 = v24;
  }
  else {
    v22 = '\0';
    v23 = 100;
  }
  if (v22) {
    if (putc(0x20,v9) == -1) {
      v22 = '\0';
      v23 = 0x96;
    }
    v8 = (unsigned long)v19;
    v4 = *(unsigned long *)((long)v17 + 0x40);
    v24 = fprintf(v9,"%*s",dat_49734,(char *)sub_22eb1(v4,v11,0,0x200,v8));
    if (0 <= v24) { // branch-flip
      if (dat_49734 < v24)
        dat_49734 = v24;
    }
    else {
      v22 = '\0';
      v23 = 200;
    }
  }
  if ((v22) && (putc(0x20,v9) == -1)) {
    v22 = '\0';
    v23 = 0xfa;
  }
  if ((v22) && (fputs(v10,v9) <= -1)) {
    v22 = '\0';
    v23 = 0x113;
  }
  if (v22) {
    v24 = fprintf(v9,"%*lu",dat_49738,*(unsigned long *)((long)v17 + 0x10));
    if (0 <= v24) { // branch-flip
      if (dat_49738 < v24)
        dat_49738 = v24;
    }
    else {
      v22 = '\0';
      v23 = 300;
    }
  }
  if (v22) {
    if (putc(0x20,v9) == -1) {
      v22 = '\0';
      v23 = 0xfa;
    }
    v32 = (char *)sub_23cf8(*(unsigned int *)((long)v17 + 0x1c));
    if (v32) { // branch-flip
      v25 = sub_25431(v32,0);
      if (dat_4973c < v25)
        dat_4973c = v25;
      v22 = -((char)~(unsigned char)((unsigned int)fprintf(v9,"%-*s ",dat_4973c,v32) >> 0x18) >> 7);
      if (v22 != '\x01')
        v23 = 400;
    }
    else {
      v24 = fprintf(v9,"%-8lu ",(unsigned long)*(unsigned int *)((long)v17 + 0x1c));
      if (dat_4973c < v24)
        dat_4973c = v24;
      v22 = 0 < v24;
      if (!(bool)v22)
        v23 = 0x1c2;
    }
  }
  if (v22) {
    v33 = (char *)sub_23f4a(*(unsigned int *)((long)v17 + 0x20));
    if (v33) { // branch-flip
      v26 = sub_25431(v33,0);
      if (dat_49740 < v26)
        dat_49740 = v26;
      v22 = -((char)~(unsigned char)((unsigned int)fprintf(v9,"%-*s ",dat_49740,v33) >> 0x18) >> 7);
      if (v22 != '\x01')
        v23 = 500;
    }
    else {
      v24 = fprintf(v9,"%-*lu",dat_49740,(unsigned long)*(unsigned int *)((long)v17 + 0x20));
      if (dat_49740 < v24)
        dat_49740 = v24;
      v22 = -((char)~(unsigned char)((unsigned int)v24 >> 0x18) >> 7);
      if (v22) { // branch-flip
        if (putc(0x20,v9) == -1) {
          v22 = '\0';
          v23 = 0x20d;
        }
      }
      else {
        v23 = 0x226;
      }
    }
  }
  if (v22) {
    if (((*(unsigned int *)((long)v17 + 0x18) & 0xf000) != 0x2000) && ((*(unsigned int *)((long)v17 + 0x18) & 0xf000) != 0x6000)) {
      if (0 <= v19) // branch-flip
        v27 = 1;
      else {
        v27 = v19;
      }
      v8 = (unsigned long)v27;
      v4 = *(unsigned long *)((long)v17 + 0x30);
      v24 = fprintf(v9,"%*s",dat_4974c,(char *)sub_22eb1(v4,v11,0,1,v8));
      if (0 <= v24) { // branch-flip
        if (dat_4974c < v24)
          dat_4974c = v24;
      }
      else {
        v22 = '\0';
        v23 = 800;
      }
    }
    else {
      v1 = *(unsigned long *)((long)v17 + 0x28);
      v3 = sub_1b74e(v9,gnu_dev_major(v1),(int *)0x49744);
      if (v3 != '\x01') {
        v22 = '\0';
        v23 = 600;
      }
      if ((v22) && (fprintf(v9,", ") <= -1)) {
        v22 = '\0';
        v23 = 0x271;
      }
      if (v22) {
        v1 = *(unsigned long *)((long)v17 + 0x28);
        v3 = sub_1b74e(v9,gnu_dev_minor(v1),(int *)0x49748);
        if (v3 != '\x01') {
          v22 = '\0';
          v23 = 0x28a;
        }
      }
    }
  }
  if ((v22) && (putc(0x20,v9) == -1)) {
    v22 = '\0';
    v23 = 0x352;
  }
  v15 = &v9;
  if (v22) {
    v34 = localtime((void *)((long)v17 + 0x58));
    if (v34) { // branch-flip
      v30 = v12;
      v31 = 0x100;
      if ((v16 + -0xed4e00 <= *(long *)((long)v17 + 0x58)) && (*(long *)((long)v17 + 0x58) <= v16 + 0xe10))
        v36 = "%b %e %H:%M";
      else {
        v36 = "%b %e  %Y";
      }
      while( true ) {
        *(unsigned long *)((long)v14 + -8) = 0x1c077;
        if (strftime(v30,v31,v36,v34)) break;
        v31 *= 2;
        v4 = ((v31 + 0x17) / 0x10) * 0x10;
        for (v13 = (char *)v14; v13 != (char *)((long)v14 + -(v4 & 0xfffffffffffff000)); v13 = &v13[-0x1000]) {
          *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
        }
        v2 = -(unsigned long)((unsigned int)v4 & 0xfff);
        v14 = (char *)&v13[v2];
        if (v4 & 0xfff)
          *(unsigned long *)&v13[((unsigned long)((unsigned int)v4 & 0xfff) - 8) + v2] = *(unsigned long *)&v13[((unsigned long)((unsigned int)v4 & 0xfff) - 8) + v2];
        v30 = (char *)((unsigned long)&v13[v2 + 0xf] & 0xfffffffffffffff0);
      }
      *(unsigned long *)((long)v14 + -8) = 0x1c0a5;
      v15 = v14;
      if (fprintf(v9,"%s ",v30) <= -1) {
        v22 = '\0';
        v23 = 900;
      }
    }
    else {
      v28 = 0xc;
      if (0 <= *(long *)((long)v17 + 0x58)) { // branch-flip
        v6 = (char *)sub_22eb1(*(unsigned long *)((long)v17 + 0x58),v11,0,1,1);
        v15 = &v9;
        if (fprintf(v9,"%*s ",v28,v6) <= -1) {
          v22 = '\0';
          v23 = 0x44c;
          v15 = &v9;
        }
      }
      else {
        v6 = (char *)sub_22eb1(-*(long *)((long)v17 + 0x58),v11,0,1,1);
        v7 = v28;
        v35 = v6;
        v29 = v7 - (int)strlen(v6);
        v7 = 0;
        if (0 <= v29)
          v7 = v29;
        v15 = &v9;
        if (fprintf(v9,"%*s%s ",v7,"-",v35) <= -1) {
          v22 = '\0';
          v23 = 1000;
          v15 = &v9;
        }
      }
    }
  }
  if (v22) {
    *(unsigned long *)((long)v15 + -8) = 0x1c217;
    v22 = sub_1c605(v21,v9,a6);
    if (v22 != '\x01')
      v23 = 0x4b0;
  }
  if (v22) {
    if ((*(unsigned int *)((long)v17 + 0x18) & 0xf000) == 0xa000) {
      *(unsigned long *)((long)v15 + -8) = 0x1c273;
      v5 = (unsigned char *)sub_1d06d(v20,v18);
      v37 = v5;
      if (v5) { // branch-flip
        *(unsigned long *)((long)v15 + -8) = 0x1c29d;
        if (fputs(" -> ",v9) <= -1) {
          v22 = '\0';
          v23 = 0x514;
        }
        if (v22) {
          *(unsigned long *)((long)v15 + -8) = 0x1c2d7;
          v22 = sub_1c605(v37,v9,a6);
          if (v22 != '\x01')
            v23 = 0x546;
        }
      }
      else {
        *(unsigned long *)((long)v15 + -8) = 0x1c2fc;
        v7 = *__errno_location();
        *(unsigned long *)((long)v15 + -8) = 0x1c320;
        error(0,v7,"%s",(char *)v21);
      }
      *(unsigned long *)((long)v15 + -8) = 0x1c32f;
      free(v37);
    }
    if (v22) {
      *(unsigned long *)((long)v15 + -8) = 0x1c34c;
      if (putc(10,v9) == -1) {
        v22 = '\0';
        v23 = 0x578;
      }
    }
  }
  if (v22 != '\x01') {
    *(unsigned long *)((long)v15 + -8) = 0x1c38d;
    v6 = gettext("Failed to write output (at stage %d)");
    *(unsigned long *)((long)v15 + -8) = 0x1c395;
    v7 = *__errno_location();
    *(unsigned long *)((long)v15 + -8) = 0x1c3b3;
    error(1,v7,v6,(unsigned long)v23);
    return;
  }
}

// Function: sub_1c3ce @ 0x1c3ce
unsigned int sub_1c3ce(char *a0,FILE *a1)
{
  return (unsigned int)~fprintf(a1,"%s",a0) >> 0x1f;
}

// Function: sub_1c408 @ 0x1c408
unsigned long sub_1c408(unsigned char *a0,FILE *a1) // return-dupe x2
{
  unsigned char *v1;
  unsigned char v2;
  unsigned int v3; // eax
  int v4; // stack - 0x1c
  
  do {
    v1 = &a0[1];
    v2 = *a0;
    v3 = (unsigned int)v2;
    if (!v2)
      return 1;
    if (0x23 <= v3) { // branch-flip
      if (v3 == 0x5c) {
        v4 = fprintf(a1,"\\\\");
        goto label_1c5d6;
      }
label_1c585:
      if ((0x21 <= v2) && (v2 <= 0x7e)) {
        if (putc(v3,a1) == -1)
          return 0;
        v4 = 1;
      }
      else {
        v4 = fprintf(a1,"\\%03o",v3);
      }
    }
    else {
      if (v3 < 8) goto label_1c585;
      switch(v3) {
        case 8:
          v4 = fprintf(a1,"\\b");
          break;
        case 9:
          v4 = fprintf(a1,"\\t");
          break;
        case 10:
          v4 = fprintf(a1,"\\n");
          break;
        default:
          goto label_1c585;
        case 0xc:
          v4 = fprintf(a1,"\\f");
          break;
        case 0xd:
          v4 = fprintf(a1,"\\r");
          break;
        case 0x20:
          v4 = fprintf(a1,"\\ ");
          break;
        case 0x22:
          v4 = fprintf(a1,"\\\"");
        
      }
    }
label_1c5d6:
    a0 = v1;
    if (v4 <= -1)
      return 0;
  } while( true );
}

// Function: sub_1c605 @ 0x1c605
void sub_1c605(unsigned char *a0,FILE *a1,int a2) // return-dupe
{
  if (a2) {
    sub_1c3ce(a0,a1);
    return;
  }
  sub_1c408(a0,a1);
}

// Function: sub_1c643 @ 0x1c643
int sub_1c643(FILE *a0,unsigned int *a1,bool a2,char *a3,char *a4)
{
  unsigned long v1; // rax
  char v2 [8200];
  int v3; // stack - 0x202c
  char *v4; // stack - 0x2028
  
  if (a2) { // branch-flip
    v1 = sub_2f235(v2,0x2000,a4,0xffffffffffffffff,a1);
    if (0x2000 <= v1) { // branch-flip
      v4 = (char *)sub_31bc2(v1 + 1);
      sub_2f235(v4,v1 + 1,a4,0xffffffffffffffff,a1);
    }
    else {
      v4 = v2;
    }
    v4[sub_1c8a3(v4,v1)] = 0;
    v3 = fprintf(a0,a3,v4);
    if (v4 != v2)
      free(v4);
  }
  else {
    v3 = fprintf(a0,a3,a4);
  }
  return v3;
}

// Function: sub_1c815 @ 0x1c815
char sub_1c815(char a0)
{
  return a0;
}

// Function: sub_1c824 @ 0x1c824
long sub_1c824(char *a0,long a1)
{
  char v1;
  long v2;
  char *v3; // stack - 0x28
  
  for (v3 = a0; v3 < &a0[a1]; v3 = &v3[1]) {
    v2 = *(long *)__ctype_b_loc();
    v1 = *v3;
    if (!(*(unsigned short *)((unsigned long)(unsigned char)sub_1c815((int)v1) * 2 + v2) & 0x4000))
      *v3 = '?';
  }
  return a1;
}

// Function: sub_1c8a3 @ 0x1c8a3
long sub_1c8a3(char *a0,long a1)
{
  char *v1;
  long v10; // stack - 0x28
  char *v11; // stack - 0x20
  char *v2;
  int v3; // eax
  long v4; // rax
  mbstate_t v5;
  int v6; // stack - 0x40
  int v7; // stack - 0x3c
  char *v8; // stack - 0x38
  char *v9; // stack - 0x30
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v11 = &a0[a1];
    v8 = a0;
    v9 = a0;
label_1ca5b:
    if (v8 < v11) {
      v3 = (int)*v8;
      if (0x7f <= v3) { // branch-flip
label_1c961:
        memset(&v5,0,8);
        do {
          v10 = sub_2419c(&v6,v8,(long)v11 - (long)v8,&v5);
          if (v10 == -1) {
            v8 = &v8[1];
            v1 = &v9[1];
            *v9 = '?';
            v9 = v1;
            break;
          }
          if (v10 == -2) {
            v8 = v11;
            v1 = &v9[1];
            *v9 = '?';
            v9 = v1;
            break;
          }
          if (!v10)
            v10 = 1;
          v7 = wcwidth(v6);
          if (0 <= v7) { // branch-flip
            while (v10) {
              v1 = &v8[1];
              v2 = &v9[1];
              *v9 = *v8;
              v10 -= 1;
              v8 = v1;
              v9 = v2;
            }
          }
          else {
            v8 = &v8[v10];
            v1 = &v9[1];
            *v9 = '?';
            v9 = v1;
          }
        } while (!mbsinit(&v5));
      }
      else {
        if (v3 <= 0x60) {
          if (0x60 <= v3) goto label_1c961;
          if (v3 <= 0x40) {
            if (0x24 <= v3) { // branch-flip
              if (0x1b <= (unsigned int)(v3 - 0x25U)) goto label_1c961;
            }
            else if (v3 <= 0x1f) goto label_1c961;
          }
        }
        v1 = &v8[1];
        v2 = &v9[1];
        *v9 = *v8;
        v8 = v1;
        v9 = v2;
      }
      goto label_1ca5b;
    }
    v4 = (long)v9 - (long)a0;
  }
  else {
    v4 = sub_1c824(a0,a1);
  }
  return v4;
}

// Function: sub_1ca8f @ 0x1ca8f
unsigned long sub_1ca8f(char *a0)
{
  char *v1;
  char *v2; // rax
  unsigned long v3; // rax
  unsigned int v4; // stack - 0x34
  long v5; // stack - 0x30
  char *v6; // stack - 0x28
  
  v5 = 0;
  v4 = 0;
  while( true ) {
    if (0xd <= v4) {
      v1 = (char *)sub_31bc2(v5 + 1);
      v6 = v1;
      for (v4 = 0; v4 <= 0xc; v4 = v4 + 1) {
        if (v4) {
          strcpy(v6,", ");
          v6 = &v6[2];
        }
        v2 = (char *)sub_2fd76(*(char **)((unsigned long)v4 * 0x10 + 0x49760));
        v6 = &v6[sprintf(v6,"%s",v2)];
      }
      v2 = (char *)sub_2fd76(a0);
      error(1,0,gettext("Unknown regular expression type %s; valid types are %s."),v2,v1);
      return v3;
    }
    v1 = *(char **)((unsigned long)v4 * 0x10 + 0x49760);
    if (!strcmp(v1,a0)) break;
    v1 = (char *)sub_2fd76(*(char **)((unsigned long)v4 * 0x10 + 0x49760));
    v5 += strlen(v1) + 2;
    v4 += 1;
  }
  return (unsigned long)*(unsigned int *)((unsigned long)v4 * 0x10 + 0x4976c);
}

// Function: sub_1cbff @ 0x1cbff
unsigned long sub_1cbff(unsigned int a0) // early-return
{
  if (a0 <= 0xc)
    return *(unsigned long *)((unsigned long)a0 * 0x10 + 0x49760);
  return 0;
}

// Function: sub_1cc2e @ 0x1cc2e
unsigned int sub_1cc2e(unsigned int a0) // early-return
{
  if (a0 <= 0xc)
    return *(unsigned int *)((unsigned long)a0 * 0x10 + 0x4976c);
  return 0xffffffff;
}

// Function: sub_1cc5c @ 0x1cc5c
unsigned int sub_1cc5c(unsigned int a0) // early-return
{
  if (a0 <= 0xc)
    return *(unsigned int *)((unsigned long)a0 * 0x10 + 0x49768);
  return 0;
}

// Function: sub_1cc8a @ 0x1cc8a
unsigned int sub_1cc8a(unsigned int a0,unsigned int a1) // return-dupe
{
  unsigned int v1; // stack - 0x10
  
  if (0xd <= a0)
    return 0xffffffff;
  v1 = 0;
  while( true ) {
    if (a0 <= v1)
      return 0xffffffff;
    if ((*(unsigned int *)((unsigned long)v1 * 0x10 + 0x49768) & a1) && (*(int *)((unsigned long)a0 * 0x10 + 0x4976c) == *(int *)((unsigned long)v1 * 0x10 + 0x4976c))) break;
    v1 += 1;
  }
  return v1;
}

// Function: sub_1cd12 @ 0x1cd12
long sub_1cd12(char *a0,unsigned int a1)
{
  long v1; // rax
  long v10; // stack - 0x38
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  long v7; // rax
  long v8; // rax
  char *v9; // stack - 0x40
  
  *__errno_location() = 0;
  v10 = strtol(a0,&v9,10);
  if ((v10 == 0x7fffffffffffffff) || (v10 == -0x8000000000000000)) {
    if (*__errno_location() != 0x22) {
      error(1,*__errno_location(),"%s",a0);
      return v2;
    }
    error(1,*__errno_location(),"%s",a0);
    return v1;
  }
  if ((v10 <= 0x7fffffff) && (-0x80000001 < v10)) {
    if (*v9) {
      v4 = sub_2f861(1,a1,a0);
      v5 = sub_2f861(0,a1,v9);
      v6 = gettext("Unexpected suffix %s on %s");
      error(1,*__errno_location(),v6,v5,v4);
      return v7;
    }
    if (a0 != v9)
      return v10;
    v4 = sub_2f861(0,a1,a0);
    v6 = gettext("Expected an integer: %s");
    error(1,*__errno_location(),v6,v4);
    return v8;
  }
  *__errno_location() = 0x22;
  error(1,*__errno_location(),"%s",a0);
  return v3;
}

// Function: sub_1cf01 @ 0x1cf01
unsigned long sub_1cf01(char *a0,char *a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (*a1) { // branch-flip
    v1 = strpbrk(a0,a1);
    if (!v1) {
      v2 = strlen(a0); // return-dupe
      return v2;
    }
    v2 = (long)v1 - (long)a0;
  }
  else {
    v2 = strlen(a0);
  }
  return v2;
}

// Function: sub_1cf56 @ 0x1cf56
unsigned long sub_1cf56(char *a0,char *a1,bool a2,long *a3,long *a4)
{
  long v1;
  
  if (a2) { // branch-flip
    *a3 = 0;
    *a4 = 0;
  }
  else {
    *a3 = *a3 + *a4;
    if (!a0[*a3])
      return 0;
    *a3 = *a3 + 1;
  }
  v1 = *a3;
  *a4 = sub_1cf01(&a0[v1],a1);
  return 1;
}

// Function: sub_1d008 @ 0x1d008
void sub_1d008(FILE *a0,char *a1)
{
  fprintf(a0,gettext("Please see also the documentation at %s.\nYou can report (and track progress on fixing) bugs in the \"%s\"\nprogram via the %s bug-reporting page at\n%s or, if\nyou have no web access, by sending email to <%s>.\n"),"http://www.gnu.org/software/findutils/",a1,"GNU findutils","https://savannah.gnu.org/bugs/?group=findutils","bug-findutils@gnu.org");
}

// Function: sub_1d06d @ 0x1d06d
void sub_1d06d(unsigned int a0,unsigned long a1)
{
  sub_1d51d(a0,a1,NULL,0,NULL,dat_48fd8);
}

// Function: sub_1d0aa @ 0x1d0aa
struct_43 * sub_1d0aa(unsigned long a0) // early-return
{
  struct_43 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return NULL;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
  return v1;
}

// Function: sub_1d0ff @ 0x1d0ff
unsigned long * sub_1d0ff(unsigned long a0) // early-return
{
  unsigned long *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return NULL;
  *v1 = a0;
  v1[2] = 0;
  v1[3] = 0;
  v1[1] = 0;
  v1[4] = 0;
  return v1;
}

// Function: sub_1d16c @ 0x1d16c
long sub_1d16c(struct_44 *a0,unsigned int *a1) // early-return x2, ternary
{
  FILE *v1;
  long *v2;
  long v3; // rax
  unsigned int v4; // edx
  
  if (a0->field_0x0) { // branch-flip
    v3 = getdelim((char **)&a0->field_0x10,a0->field_0x18,0,(FILE *)a0->field_0x0);
    if (v3 <= -1) {
      v1 = (FILE *)a0->field_0x0;
      v4 = (feof(v1)) ? 2 : 4; // branch-flip
      *a1 = v4;
      v3 = 0;
      return v3;
    }
    *a1 = 1;
    a0->field_0x8 = a0->field_0x8 + 1;
    v3 = a0->field_0x10;
  }
  else {
    if (!*a0->field_0x28) {
      *a1 = 2;
      v3 = 0;
      return v3;
    }
    *a1 = 1;
    v2 = a0->field_0x28;
    a0->field_0x28 = &v2[1];
    v3 = *v2;
  }
  return v3;
}

// Function: sub_1d257 @ 0x1d257
long sub_1d257(struct_45 *a0)
{
  long v1;
  
  if (a0->field_0x0) // branch-flip
    v1 = a0->field_0x8;
  else {
    v1 = a0->field_0x28 - a0->field_0x20 >> 3;
  }
  return v1;
}

// Function: sub_1d295 @ 0x1d295
void sub_1d295(struct_46 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0);
}

// Function: sub_1d2d0 @ 0x1d2d0
void * sub_1d2d0(unsigned int a0,unsigned long a1,void *a2,unsigned long a3,unsigned long *a4,void *a5,void *a6) // early-return x3, return-dupe
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
    v6 = (unsigned long *)0x488c0;
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
label_1d4ed:
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
        if (!v5) goto label_1d4ed;
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

// Function: sub_1d51d @ 0x1d51d
void sub_1d51d(unsigned int a0,unsigned long a1,void *a2,unsigned long a3,unsigned long *a4,void *a5)
{
  char v1 [1032];
  
  sub_1d2d0(a0,a1,a2,a3,a4,a5,v1);
}

// Function: sub_1d5ba @ 0x1d5ba
unsigned long sub_1d5ba(unsigned int a0,bool a1) // return-dupe, ternary
{
  int v1; // eax
  unsigned int v2; // eax
  
  v1 = sub_1db8e(a0,1,0);
  if (0 <= v1) {
    v2 = (a1) ? v1 | 1 : v1 & 0xfffffffe; // branch-flip
    if ((v1 == v2) || (sub_1db8e(a0,2,v2) != -1))
      return 0;
  }
  return 0xffffffff;
}

// Function: sub_1d639 @ 0x1d639
void sub_1d639(unsigned int a0)
{
  sub_1db8e(a0,0x406,0);
}

// Function: sub_1d663 @ 0x1d663
void sub_1d663(unsigned long a0)
{
  dat_4a200 = a0;
}

// Function: sub_1d67d @ 0x1d67d
void sub_1d67d(char a0)
{
  dat_4a208 = a0;
}

// Function: sub_1d697 @ 0x1d697
void sub_1d697(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_33ec6(stdout);
  if (v1) {
    if (dat_4a208 == '\x01') {
      if (*__errno_location() == 0x20) goto label_1d768;
    }
    v2 = gettext("write error");
    if (dat_4a200) { // branch-flip
      v3 = (char *)sub_2fa70(dat_4a200);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_49830); // no-return
  }
label_1d768:
  v1 = sub_33ec6(stderr);
  if (v1)
    _exit(dat_49830); // no-return
}

// Function: sub_1d78f @ 0x1d78f
DIR * sub_1d78f(char *a0)
{
  int v1; // eax
  int v2; // stack - 0x24
  DIR *v3; // stack - 0x18
  DIR *v4; // stack - 0x10
  
  v3 = opendir(a0);
  if (v3) {
    v1 = dirfd(v3);
    if ((0 <= v1) && (v1 <= 2)) {
      v1 = sub_1db8e(v1,0x406,3);
      if (0 <= v1) { // branch-flip
        v4 = fdopendir(v1);
        v2 = *__errno_location();
        if (!v4)
          close(v1);
      }
      else {
        v2 = *__errno_location();
        v4 = NULL;
      }
      closedir(v3);
      *__errno_location() = v2;
      v3 = v4;
    }
  }
  return v3;
}

// Function: sub_1d85c @ 0x1d85c
void * sub_1d85c(char *a0)
{
  long v1; // rax
  void *v2; // rax
  char *v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v3 = (char *)sub_33495(a0);
  if (*v3) { // branch-flip
    v1 = sub_33502(v3);
    v4 = v1 + (unsigned long)(v3[v1] == '/');
  }
  else {
    v4 = sub_33502(a0);
    v3 = a0;
  }
  v2 = (void *)sub_31be8(v4 + 1);
  memcpy(v2,v3,v4);
  *(char *)((long)v2 + v4) = 0;
  return v2;
}

// Function: sub_1d94f @ 0x1d94f
unsigned long sub_1d94f(char *a0)
{
  char v1;
  unsigned long v2; // stack - 0x18
  
  v1 = *a0;
  for (v2 = sub_33495(a0) - (long)a0; ((v1 == '/') < v2 && (a0[v2 - 1] == '/')); v2 = v2 - 1) {
  }
  return v2;
}

// Function: sub_1d9d4 @ 0x1d9d4
char * sub_1d9d4(char *a0) // early-return
{
  char *v1; // rax
  unsigned long v2; // stack - 0x18
  
  v2 = sub_1d94f(a0);
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

// Function: sub_1da6f @ 0x1da6f
bool sub_1da6f(char *a0)
{
  char v1;
  long v2; // rax
  char *v3; // stack - 0x18
  
  v3 = (char *)sub_33495(a0);
  if (!*v3)
    v3 = a0;
  v2 = sub_33502(v3);
  v1 = v3[v2];
  v3[v2] = '\0';
  return v1 != '\0';
}

// Function: sub_1dad5 @ 0x1dad5
unsigned long sub_1dad5(FILE *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_1db63;
  }
  if (sub_1e1a8(a0))
    v2 = *__errno_location();
label_1db63:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_1db8e @ 0x1db8e
int sub_1db8e(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_1df00(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_1df31(a0,(unsigned int)v2);
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

// Function: sub_1df00 @ 0x1df00
int sub_1df00(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_1df31 @ 0x1df31
int sub_1df31(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_4a20c) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_4a20c = 1;
    else {
      v3 = sub_1df00(a0,a1);
      if (0 <= v3)
        dat_4a20c = -1;
    }
  }
  else {
    v3 = sub_1df00(a0,a1);
  }
  if ((0 <= v3) && (dat_4a20c == -1)) {
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

// Function: sub_1e034 @ 0x1e034
void sub_1e034(char *a0,unsigned int a1)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa8
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = v2;
  sub_3121c(open(a0,a1,(unsigned long)v1));
}

// Function: sub_1e170 @ 0x1e170
void sub_1e170(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_1e588(a0,0,1);
}

// Function: sub_1e1a8 @ 0x1e1a8
void sub_1e1a8(FILE *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_1e170(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_1e1f7 @ 0x1e1f7
unsigned long sub_1e1f7(unsigned int a0) // return-dupe x7
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

// Function: sub_1e2a6 @ 0x1e2a6
void sub_1e2a6(unsigned int a0,char *a1) // ternary x12
{
  char v1; // al
  
  *a1 = sub_1e1f7(a0);
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

// Function: sub_1e472 @ 0x1e472
void sub_1e472(void *a0,char *a1)
{
  sub_1e2a6(*(unsigned int *)((long)a0 + 0x18),a1);
}

// Function: sub_1e49e @ 0x1e49e
FILE * sub_1e49e(char *a0,char *a1)
{
  int v1;
  int v2;
  FILE *v3; // rax
  FILE *v4; // stack - 0x10
  
  v4 = fopen(a0,a1);
  if (((!v4) || (v2 = fileno(v4), v2 <= -1)) || (3 <= v2))
    return v4;
  v2 = sub_311f2(v2);
  if (v2 <= -1) {
    v2 = *__errno_location();
    sub_1dad5(v4);
    *__errno_location() = v2;
    return NULL;
  }
  if ((!sub_1dad5(v4)) && (v3 = fdopen(v2,a1), v3))
    return v3;
  v1 = *__errno_location();
  close(v2);
  *__errno_location() = v1;
  return NULL;
}

// Function: sub_1e588 @ 0x1e588
unsigned long sub_1e588(FILE *a0,long a1,int a2) // early-return x2
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

// Function: sub_1e646 @ 0x1e646
unsigned int sub_1e646(struct_24 *a0,struct_24 *a1) // return-dupe
{
  if ((a0->field_0x8 == a1->field_0x8) && (a0->field_0x0 == a1->field_0x0))
    return 1;
  return 0;
}

// Function: sub_1e69f @ 0x1e69f
unsigned long sub_1e69f(void *a0,unsigned long a1)
{
  return *(unsigned long *)((long)a0 + 8) % a1;
}

// Function: sub_1e6cd @ 0x1e6cd
unsigned long sub_1e6cd(void *a0) // return-dupe x2, return-dupe
{
  if (!(*(unsigned int *)((long)a0 + 0x48) & 0x102)) {
    *(void **)((long)a0 + 0x58) = malloc(0x20);
    if (*(long *)((long)a0 + 0x58)) {
      sub_33f75(*(void **)((long)a0 + 0x58));
      return 1;
    }
    return 0;
  }
  *(unsigned long *)((long)a0 + 0x58) = sub_21bb1(0x1f,NULL,sub_1e69f,sub_1e646,dat_48fc0);
  if (*(long *)((long)a0 + 0x58))
    return 1;
  return 0;
}

// Function: sub_1e779 @ 0x1e779
unsigned long sub_1e779(void *a0,struct_37 *a1) // return-dupe x3
{
  struct_36 *v1;
  struct_38 *v2; // rax
  struct_38 *v3; // rax
  
  if (!(*(unsigned int *)((long)a0 + 0x48) & 0x102)) {
    v1 = *(struct_36 **)((long)a0 + 0x58);
    if (!sub_33f9b(v1,&a1->field_0x70))
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
  v3 = (struct_38 *)sub_228f2(*(void **)((long)a0 + 0x58),v2);
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

// Function: sub_1e88b @ 0x1e88b
void sub_1e88b(void *a0,void *a1)
{
  long v1;
  void *v2; // rax
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  
  if (*(unsigned int *)((long)a0 + 0x48) & 0x102) { // branch-flip
    v3 = *(long *)((long)a1 + 0x70);
    v4 = *(unsigned long *)((long)a1 + 0x78);
    v2 = (void *)sub_22962(*(void **)((long)a0 + 0x58),&v3);
    if (!v2)
      abort(); // no-return
    free(v2);
  }
  else {
    v1 = *(long *)((long)a1 + 8);
    if ((v1) && (0 <= *(long *)(v1 + 0x58))) {
      if (!*(long *)(*(long *)((long)a0 + 0x58) + 0x10))
        abort(); // no-return
      if ((**(long **)((long)a0 + 0x58) == *(long *)((long)a1 + 0x78)) && (*(long *)(*(long *)((long)a0 + 0x58) + 8) == *(long *)((long)a1 + 0x70))) {
        *(unsigned long *)(*(long *)((long)a0 + 0x58) + 8) = *(unsigned long *)(v1 + 0x70);
        **(unsigned long **)((long)a0 + 0x58) = *(unsigned long *)(v1 + 0x78);
      }
    }
  }
}

// Function: sub_1e9bd @ 0x1e9bd
void sub_1e9bd(void *a0) // return-dupe
{
  if (!(*(unsigned int *)((long)a0 + 0x48) & 0x102)) {
    free(*(void **)((long)a0 + 0x58));
    return;
  }
  if (*(long *)((long)a0 + 0x58)) {
    sub_21e31(*(struct_27 **)((long)a0 + 0x58));
    return;
  }
}

// Function: sub_1ea0f @ 0x1ea0f
void sub_1ea0f(void *a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_23bd0(a0) == '\x01') break;
    v1 = sub_23c72(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_1ea57 @ 0x1ea57
void sub_1ea57(void *a0,bool a1) // ternary
{
  unsigned long v1; // rdx
  
  if (*(short *)((long)a0 + 0x68) != 0xb)
    abort(); // no-return
  v1 = (a1) ? 2 : 1; // branch-flip
  *(unsigned long *)((long)a0 + 0xa0) = v1;
}

// Function: sub_1ea9f @ 0x1ea9f
void sub_1ea9f(void *a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)((long)a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_23be6((void *)((long)a0 + 0x60),v1);
    if (0 <= v1)
      close(v1);
  }
  else if ((!(*(unsigned int *)((long)a0 + 0x48) & 4)) && (0 <= v1))
    close(v1);
  *(int *)((long)a0 + 0x2c) = a1;
}

// Function: sub_1eb2f @ 0x1eb2f
unsigned int sub_1eb2f(void *a0)
{
  unsigned int v1;
  int v2;
  
  if (!(*(unsigned int *)((long)a0 + 0x48) & 4)) {
    if (*(unsigned int *)((long)a0 + 0x48) & 0x200) { // branch-flip
      if (*(unsigned int *)((long)a0 + 0x48) & 0x200) // branch-flip
        v1 = 0xffffff9c;
      else {
        v1 = *(unsigned int *)((long)a0 + 0x28);
      }
      sub_1ea9f(a0,v1,1);
    }
    else {
      if (*(unsigned int *)((long)a0 + 0x48) & 0x200) // branch-flip
        v2 = -100;
      else {
        v2 = *(int *)((long)a0 + 0x28);
      }
      if (fchdir(v2)) {
        v1 = 1;
        sub_1ea0f((void *)((long)a0 + 0x60)); // return-dupe
        return v1;
      }
    }
  }
  v1 = 0;
  sub_1ea0f((void *)((long)a0 + 0x60));
  return v1;
}

// Function: sub_1ebdd @ 0x1ebdd
unsigned int sub_1ebdd(void *a0,char *a1)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  
  v1 = (*(unsigned int *)((long)a0 + 0x48) & 0x10) << 0xd | 0x90900;
  if (*(unsigned int *)((long)a0 + 0x48) & 0x200) // branch-flip
    v2 = sub_2704e(*(unsigned int *)((long)a0 + 0x2c),a1,v1);
  else {
    v2 = sub_1e034(a1,v1);
  }
  return v2;
}

// Function: sub_1ec55 @ 0x1ec55
long * sub_1ec55(long *a0,unsigned int a1,long a2) // early-return x5
{
  bool v1; // al
  long *v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  void *v5;
  long *v6; // stack - 0x70
  void *v7; // stack - 0x60
  void *v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  v7 = NULL;
  v8 = NULL;
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
  v3 = sub_20ecf(a0);
  if (v3 <= 0xfff)
    v3 = 0x1000;
  if (sub_20ceb(v2,v3) != '\x01') {
    free(v2); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v7 = (void *)sub_20bf4(v2,(void *)0x3c7d4,0);
    if (v7) {
      *(unsigned long *)((long)v7 + 0x58) = 0xffffffffffffffff;
      goto label_1edb4;
    }
  }
  else {
label_1edb4:
    if ((a2) && (!(*(unsigned int *)&v2[9] & 0x400)))
      v1 = 0;
    else {
      v1 = 1;
    }
    v5 = NULL;
    v3 = 0;
    for (v6 = a0; *v6; v6 = &v6[1]) {
      v9 = strlen((char *)*v6);
      if (((!(a1 & 0x800)) && (3 <= v9)) && (*(char *)(*v6 + (v9 - 1)) == '/')) {
        for (; (2 <= v9 && (*(char *)(*v6 + (v9 - 2)) == '/')); v9 = v9 - 1) {
        }
      }
      v4 = (void *)sub_20bf4(v2,(void *)*v6,v9);
      if (!v4) goto label_1efde;
      *(unsigned long *)((long)v4 + 0x58) = 0;
      *(void **)((long)v4 + 8) = v7;
      *(long *)((long)v4 + 0x30) = (long)v4 + 0x100;
      if ((v1) && (v5)) {
        *(unsigned short *)((long)v4 + 0x68) = 0xb;
        sub_1ea57(v4,1);
      }
      else {
        *(unsigned short *)((long)v4 + 0x68) = sub_208ad(v2,v4,0);
      }
      if (a2) { // branch-flip
        *(void **)((long)v4 + 0x10) = v5;
        v5 = v4;
      }
      else {
        *(unsigned long *)((long)v4 + 0x10) = 0;
        if (v5) { // branch-flip
          *(void **)((long)v8 + 0x10) = v4;
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
      v5 = (void *)sub_20a86(v2,v5,v3);
    *v2 = sub_20bf4(v2,(void *)0x3c7d4,0);
    if (*v2) {
      *(void **)(*v2 + 0x10) = v5;
      *(unsigned short *)(*v2 + 0x68) = 9;
      *(unsigned long *)(*v2 + 0x58) = 1;
      if (sub_1e6cd(v2) == '\x01') {
        if ((!(*(unsigned int *)&v2[9] & 4)) && (!(*(unsigned int *)&v2[9] & 0x200))) {
          *(unsigned int *)&v2[5] = sub_1ebdd(v2,".");
          if ((int)v2[5] <= -1)
            *(unsigned int *)&v2[9] = *(unsigned int *)&v2[9] | 4;
        }
        sub_23b73(&v2[0xc],0xffffffff);
        return v2;
      }
    }
label_1efde:
    sub_20caa(v5);
    free(v7);
  }
  free((void *)v2[4]);
  free(v2);
  return NULL;
}

// Function: sub_1f020 @ 0x1f020
void sub_1f020(struct_42 *a0,void *a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  *(unsigned long *)((long)a1 + 0x48) = *(unsigned long *)((long)a1 + 0x60);
  memmove((void *)a0->field_0x20,(void *)((long)a1 + 0x100),*(long *)((long)a1 + 0x48) + 1);
  v1 = strrchr((char *)((long)a1 + 0x100),0x2f);
  if ((v1) && ((v1 != (char *)((long)a1 + 0x100) || (v1[1])))) {
    v2 = strlen(&v1[1]);
    memmove((void *)((long)a1 + 0x100),&v1[1],v2 + 1);
    *(unsigned long *)((long)a1 + 0x60) = v2;
  }
  *(long *)((long)a1 + 0x38) = a0->field_0x20;
  *(unsigned long *)((long)a1 + 0x30) = *(unsigned long *)((long)a1 + 0x38);
}

// Function: sub_1f0df @ 0x1f0df
unsigned long sub_1f0df(struct_28 *a0) // return-dupe, ternary
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
    sub_20caa((void *)a0->field_0x8);
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
  sub_1ea0f(&a0[1].field_0x8);
  if (a0->field_0x50)
    sub_21e31((struct_27 *)a0->field_0x50);
  sub_1e9bd(a0);
  free(a0);
  if (v4) {
    *__errno_location() = v4;
    return 0xffffffff;
  }
  return 0;
}

// Function: sub_1f26c @ 0x1f26c
unsigned long sub_1f26c(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_1f2a1 @ 0x1f2a1
unsigned long sub_1f2a1(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_1f2d7 @ 0x1f2d7
unsigned long sub_1f2d7(void *a0,int a1) // early-return x2
{
  long v1;
  struct_31 *v2; // rax
  struct_31 *v3; // rax
  unsigned long v4 [2]; // stack - 0x98
  statfs v5; // stack - 0x88
  struct_32 *v6; // stack - 0xb8
  
  v1 = *(long *)((long)a0 + 0x50);
  v6 = *(struct_32 **)(v1 + 0x50);
  if (!(*(unsigned int *)(v1 + 0x48) & 0x200))
    return 0;
  if (!v6) {
    *(unsigned long *)(v1 + 0x50) = sub_21bb1(0xd,NULL,sub_1f26c,sub_1f2a1,dat_48fc0);
    v6 = *(struct_32 **)(v1 + 0x50);
  }
  if (v6) {
    v4[0] = *(unsigned long *)((long)a0 + 0x70);
    v1 = sub_214fc(v6,v4);
    if (v1)
      return *(unsigned long *)(v1 + 8);
  }
  if ((0 <= a1) && (!fstatfs(a1,&v5))) {
    if ((v6) && (v2 = malloc(0x10), v2)) {
      v2->field_0x0 = *(unsigned long *)((long)a0 + 0x70);
      v2->field_0x8 = v5._0_8_;
      v3 = (struct_31 *)sub_228f2(v6,v2);
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

// Function: sub_1f4d0 @ 0x1f4d0
unsigned long sub_1f4d0(void *a0,unsigned int a1) // early-return
{
  long v1; // rax
  
  v1 = sub_1f2d7(a0,a1);
  if ((v1 != 0xff534d42) && ((0xff534d43 <= v1 || ((v1 != 0x6969 && (v1 != 0x1021994))))))
    return 1;
  return 0;
}

// Function: sub_1f526 @ 0x1f526
unsigned long sub_1f526(void *a0,unsigned int a1) // early-return
{
  long v1; // rax
  
  v1 = sub_1f2d7(a0,a1);
  if ((v1 != 0xff534d42) && ((0xff534d43 <= v1 || ((v1 != 0x5346414f && ((0x53464150 <= v1 || ((v1 != 0x9fa0 && ((0x9fa1 <= v1 || ((v1 && (v1 != 0x6969))))))))))))))
    return 1;
  return 0;
}

// Function: sub_1f599 @ 0x1f599
void * sub_1f599(struct_42 *a0) // early-return x7, ternary
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
    *(unsigned short *)((long)v6 + 0x68) = sub_208ad(a0,v6,0);
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
          v6 = (void *)sub_1fee9(a0,3);
          if (!v6) {
            if (a0->field_0x48 & 0x2000)
              return NULL;
label_1fa69:
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
                      sub_1ea9f(a0,*(unsigned int *)((long)v6 + 0x44),1);
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
                else if ((!(*(unsigned short *)((long)v6 + 0x6a) & 1)) && (v8 = *(void **)((long)v6 + 8), sub_20f29(a0,v8,0xffffffff,".."))) {
                  *(int *)((long)v6 + 0x40) = *__errno_location();
                  a0->field_0x48 = a0->field_0x48 | 0x2000;
                }
              }
              else if (sub_1eb2f(a0)) {
                *(int *)((long)v6 + 0x40) = *__errno_location();
                a0->field_0x48 = a0->field_0x48 | 0x2000;
              }
              if (*(short *)((long)v6 + 0x68) != 2) {
                v5 = (*(int *)((long)v6 + 0x40)) ? 7 : 6; // branch-flip
                *(unsigned short *)((long)v6 + 0x68) = v5;
                if (!*(int *)((long)v6 + 0x40))
                  sub_1e88b(a0,v6);
              }
              if (!(a0->field_0x48 & 0x2000))
                return v6;
              return NULL;
            }
            abort(); // no-return
          }
          free(v8);
          goto label_1f973;
        }
        v6 = *(void **)((long)v8 + 0x10);
        if (!v6) goto label_1fa69;
        a0->field_0x0 = (long)v6;
        free(v8);
        if (!*(long *)((long)v6 + 0x58)) {
          if (sub_1eb2f(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          sub_1e9bd(a0);
          sub_1f020(a0,v6);
          sub_1e6cd(a0);
          goto label_1f9d7;
        }
        v8 = v6;
      } while (*(short *)((long)v6 + 0x6c) == 4);
      if (*(short *)((long)v6 + 0x6c) == 2) {
        *(unsigned short *)((long)v6 + 0x68) = sub_208ad(a0,v6,1);
        if ((*(short *)((long)v6 + 0x68) == 1) && (!(a0->field_0x48 & 4))) {
          *(unsigned int *)((long)v6 + 0x44) = sub_1ebdd(a0,".");
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
          sub_20caa((void *)a0->field_0x8);
          a0->field_0x8 = 0;
        }
        *(unsigned short *)((long)v6 + 0x68) = 6;
        sub_1e88b(a0,v6);
        return v6;
      }
      if ((a0->field_0x8) && (a0->field_0x48 & 0x1000)) {
        a0->field_0x48 = a0->field_0x48 & 0xffffefff;
        sub_20caa((void *)a0->field_0x8);
        a0->field_0x8 = 0;
      }
      if (a0->field_0x8) { // branch-flip
        v4 = *(char **)((long)v6 + 0x30);
        if (sub_20f29(a0,v6,0xffffffff,v4)) {
          *(int *)((long)v6 + 0x40) = *__errno_location();
          *(unsigned short *)((long)v6 + 0x6a) = *(unsigned short *)((long)v6 + 0x6a) | 1;
          for (v7 = a0->field_0x8; v7; v7 = *(long *)(v7 + 0x10)) {
            *(unsigned long *)(v7 + 0x30) = *(unsigned long *)(*(long *)(v7 + 8) + 0x30);
          }
        }
      }
      else {
        a0->field_0x8 = sub_1fee9(a0,3);
        if (!a0->field_0x8) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)((long)v6 + 0x40)) && (*(short *)((long)v6 + 0x68) != 4))
              *(unsigned short *)((long)v6 + 0x68) = 7;
            sub_1e88b(a0,v6);
            return v6;
          }
          return NULL;
        }
      }
      v6 = (void *)a0->field_0x8;
      a0->field_0x8 = 0;
    }
label_1f973:
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
    *(unsigned short *)((long)v6 + 0x68) = sub_208ad(a0,v6,1);
    if ((*(short *)((long)v6 + 0x68) == 1) && (!(a0->field_0x48 & 4))) {
      *(unsigned int *)((long)v6 + 0x44) = sub_1ebdd(a0,".");
      if (0 <= *(int *)((long)v6 + 0x44)) // branch-flip
        *(unsigned short *)((long)v6 + 0x6a) = *(unsigned short *)((long)v6 + 0x6a) | 2;
      else {
        *(int *)((long)v6 + 0x40) = *__errno_location();
        *(unsigned short *)((long)v6 + 0x68) = 7;
      }
    }
  }
label_1f9d7:
  a0->field_0x0 = (long)v6;
  if (*(short *)((long)v6 + 0x68) == 0xb) {
    if (*(long *)((long)v6 + 0xa0) != 2) { // branch-flip
      if (*(long *)((long)v6 + 0xa0) != 1)
        abort(); // no-return
    }
    else {
      *(unsigned short *)((long)v6 + 0x68) = sub_208ad(a0,v6,0);
    }
  }
  if (*(short *)((long)v6 + 0x68) == 1) {
    if (!*(long *)((long)v6 + 0x58))
      a0->field_0x18 = *(long *)((long)v6 + 0x70);
    if (sub_1e779(a0,v6) != '\x01') {
      *__errno_location() = 0xc;
      return NULL;
    }
  }
  return v6;
}

// Function: sub_1fc0f @ 0x1fc0f
unsigned long sub_1fc0f(unsigned long a0,void *a1,int a2) // return-dupe
{
  if ((((a2) && (a2 != 1)) && (a2 != 2)) && ((a2 != 3 && (a2 != 4)))) {
    *__errno_location() = 0x16;
    return 1;
  }
  *(short *)((long)a1 + 0x6c) = (short)a2;
  return 0;
}

// Function: sub_1fc6a @ 0x1fc6a
int * sub_1fc6a(struct_42 *a0,int a1) // early-return x4
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
      sub_20caa((void *)a0->field_0x8);
    if (a1 != 0x1000) // branch-flip
      v6 = 1;
    else {
      v4 = a0->field_0x48 | 0x1000;
      a0->field_0x48 = v4;
      v6 = 2;
    }
    if (((*(long *)(v2 + 0x58)) || (**(char **)(v2 + 0x30) == '/')) || (v4 = a0->field_0x48 & 4, v4)) {
      v5 = (long)sub_1fee9(a0,v6);
      a0->field_0x8 = (long)v5;
      v5 = (long)a0->field_0x8;
    }
    else {
      v4 = sub_1ebdd(a0,".");
      v3 = v4;
      if (v4 <= -1) {
        a0->field_0x8 = 0;
        v5 = NULL;
        return v5;
      }
      v5 = (long)sub_1fee9(a0,v6);
      a0->field_0x8 = (long)v5;
      v4 = a0->field_0x48 & 0x200;
      if (v4) // branch-flip
        sub_1ea9f(a0,v3,1);
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

// Function: sub_1fe09 @ 0x1fe09
int sub_1fe09(long *a0,long *a1)
{
  return (unsigned int)(*(unsigned long *)(*a1 + 0x78) < *(unsigned long *)(*a0 + 0x78)) - (unsigned int)(*(unsigned long *)(*a0 + 0x78) < *(unsigned long *)(*a1 + 0x78));
}

// Function: sub_1fe5d @ 0x1fe5d
void sub_1fe5d(struct_1 *a0,unsigned int a1)
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
  a0->field_0x18 = v1;
}

// Function: sub_1fee9 @ 0x1fee9
void * sub_1fee9(struct_41 *a0,int a1) // early-return, return-dupe x2, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_1fee9
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
  void *v21; // stack - 0x78
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
  v15 = *(long *)((long)v21 + 0x18) != 0;
  v16 = 0;
  if (v15) {
    v22 = *(DIR **)((long)v21 + 0x18);
    v11 = dirfd(v22);
    if (v11 <= -1) {
      closedir(*(DIR **)((long)v21 + 0x18));
      *(unsigned long *)((long)v21 + 0x18) = 0;
      if (a1 != 3)
        return NULL;
      *(unsigned short *)((long)v21 + 0x68) = 4;
      *(int *)((long)v21 + 0x40) = *__errno_location();
      return NULL;
    }
  }
  else {
    if ((a0->field_0x48 & 0x10) && ((!(a0->field_0x48 & 1) || (*(long *)((long)v21 + 0x58)))))
      v12 = 0x20000;
    else {
      v12 = 0;
    }
    if ((a0->field_0x48 & 4) || (!(a0->field_0x48 & 0x200)))
      v6 = 0xffffff9c;
    else {
      v6 = a0->field_0x2c;
    }
    v12 = sub_2718c(v6,*(char **)((long)v21 + 0x30),v12,&v11);
    *(unsigned long *)((long)v21 + 0x18) = v12;
    if (!*(long *)((long)v21 + 0x18)) {
      if (a1 != 3)
        return NULL;
      *(unsigned short *)((long)v21 + 0x68) = 4;
      *(int *)((long)v21 + 0x40) = *__errno_location();
      return NULL;
    }
    if (*(short *)((long)v21 + 0x68) != 0xb) { // branch-flip
      if (a0->field_0x48 & 0x100) {
        sub_1e88b(a0,v21);
        sub_208ad(a0,v21,0);
        v2 = sub_1e779(a0,v21);
        if (v2 != '\x01') {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      v5 = sub_208ad(a0,v21,0);
      *(unsigned short *)((long)v21 + 0x68) = v5;
    }
  }
  v23 = (a0->field_0x40) ? 0xffffffffffffffff : 100000; // branch-flip
  if (v15)
    v1 = 1;
  else {
    v3 = ((a1 != 2) && ((((!(a0->field_0x48 & 8) || (!(a0->field_0x48 & 0x10))) || (a0->field_0x48 & 0x20)) || ((*(long *)((long)v21 + 0x80) != 2 || (v7 = sub_1f526(v21,v11), !v7))))));
    if ((v3) || (v1 = 0, a1 == 3)) {
      if (a0->field_0x48 & 0x200)
        v11 = sub_1db8e(v11,0x406,3);
      if ((0 <= v11) && (v7 = sub_20f29(a0,v21,v11,NULL), !v7))
        v1 = 1;
      else {
        if ((v3) && (a1 == 3))
          *(int *)((long)v21 + 0x40) = *__errno_location();
        *(unsigned short *)((long)v21 + 0x6a) = *(unsigned short *)((long)v21 + 0x6a) | 1;
        v1 = 0;
        closedir(*(DIR **)((long)v21 + 0x18));
        *(unsigned long *)((long)v21 + 0x18) = 0;
        if ((a0->field_0x48 & 0x200) && (0 <= v11))
          close(v11);
        *(unsigned long *)((long)v21 + 0x18) = 0;
      }
    }
  }
  if (*(char *)(*(long *)((long)v21 + 0x48) + -1 + *(long *)((long)v21 + 0x38)) != '/') // branch-flip
    v8 = *(long *)((long)v21 + 0x48);
  else {
    v8 = *(long *)((long)v21 + 0x48) + -1;
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
  v25 = *(long *)((long)v21 + 0x58) + 1;
  v3 = 0;
  v13 = NULL;
  v18 = NULL;
  v14 = 0;
  do {
    do {
      if (!*(long *)((long)v21 + 0x18)) {
label_206f9:
        if (*(long *)((long)v21 + 0x18)) {
          closedir(*(DIR **)((long)v21 + 0x18));
          *(unsigned long *)((long)v21 + 0x18) = 0;
        }
        goto label_20728;
      }
      *__errno_location() = 0;
      v26 = readdir(*(DIR **)((long)v21 + 0x18));
      if (!v26) {
        if (*__errno_location()) {
          *(int *)((long)v21 + 0x40) = *__errno_location();
          if ((v15) || (v14))
            v5 = 7;
          else {
            v5 = 4;
          }
          *(unsigned short *)((long)v21 + 0x68) = v5;
        }
        goto label_206f9;
      }
    } while (((!(a0->field_0x48 & 0x20)) && (v26->field_0x13 == '.')) && ((!v26->field_0x14 || ((v26->field_0x14 == '.' && (!v26->field_0x15))))));
    v27 = strlen(&v26->field_0x13);
    v10 = (void *)sub_20bf4(a0,&v26->field_0x13,v27);
    if (!v10) {
label_2044a:
      v17 = *__errno_location();
      free(v10);
      sub_20caa(v13);
      closedir(*(DIR **)((long)v21 + 0x18));
      *(unsigned long *)((long)v21 + 0x18) = 0;
      *(unsigned short *)((long)v21 + 0x68) = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      *__errno_location() = v17;
      return NULL;
    }
    if (v19 <= v27) {
      v28 = a0->field_0x20;
      v8 = v24 + v27;
      if (sub_20ceb(a0,v8 + 1) != '\x01') goto label_2044a;
      if ((v28 != a0->field_0x20) && (v3 = 1, a0->field_0x48 & 4))
        v20 = (char *)(v24 + a0->field_0x20);
      v19 = a0->field_0x30 - v24;
    }
    v29 = v27 + v24;
    if (v29 < v24) {
      free(v10);
      sub_20caa(v13);
      closedir(*(DIR **)((long)v21 + 0x18));
      *(unsigned long *)((long)v21 + 0x18) = 0;
      *(unsigned short *)((long)v21 + 0x68) = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      *__errno_location() = 0x24;
      return NULL;
    }
    *(long *)((long)v10 + 0x58) = v25;
    *(void **)((long)v10 + 8) = a0->field_0x0;
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
      *(unsigned short *)((long)v10 + 0x68) = sub_208ad(a0,v10,0);
    else {
      v4 = (((a0->field_0x48 & 8) && ((v26->field_0x12 && (v26->field_0x12 != '\x04')))) && ((a0->field_0x48 & 0x10 || (v26->field_0x12 != '\n'))));
      *(unsigned short *)((long)v10 + 0x68) = 0xb;
      sub_1fe5d((struct_1 *)((long)v10 + 0x70),(unsigned char)v26->field_0x12);
      sub_1ea57(v10,!v4);
    }
    *(unsigned long *)((long)v10 + 0x10) = 0;
    if (v13) // branch-flip
      *(void **)((long)v18 + 0x10) = v10;
    else {
      v13 = v10;
    }
    v18 = v10;
    if ((v14 == 10000) && (!a0->field_0x40))
      v16 = sub_1f4d0(v21,v11);
    v14 += 1;
  } while (v14 < v23);
label_20728:
  if (v3)
    sub_20dbc(a0,v13);
  if (a0->field_0x48 & 4) {
    if ((v24 == a0->field_0x30) || (!v14))
      v20 = &v20[-1];
    *v20 = 0;
  }
  if (((!v15) && (v1)) && ((a1 == 1 || (!v14)))) {
    if (*(long *)((long)v21 + 0x58)) // branch-flip
      v7 = sub_20f29(a0,*(void **)((long)v21 + 8),0xffffffff,"..");
    else {
      v7 = sub_1eb2f(a0);
    }
    if (v7) {
      *(unsigned short *)((long)v21 + 0x68) = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      sub_20caa(v13);
      return NULL;
    }
  }
  if (v14) {
    if (v16) {
      a0->field_0x40 = (long)sub_1fe09;
      v13 = (void *)sub_20a86(a0,v13,v14);
      a0->field_0x40 = 0;
    }
    if ((a0->field_0x40) && (2 <= v14))
      v13 = (void *)sub_20a86(a0,v13,v14);
    return v13;
  }
  if (((a1 == 3) && (*(short *)((long)v21 + 0x68) != 4)) && (*(short *)((long)v21 + 0x68) != 7))
    *(unsigned short *)((long)v21 + 0x68) = 6;
  sub_20caa(v13);
  return NULL;
}

// Function: sub_208ad @ 0x208ad
unsigned long sub_208ad(void *a0,void *a1,bool a2)
{
  stat *v1;
  int v2;
  char *v3;
  bool v4; // stack - 0x34
  
  v1 = (stat *)((long)a1 + 0x70);
  v4 = a2;
  if ((!*(long *)((long)a1 + 0x58)) && (*(unsigned int *)((long)a0 + 0x48) & 1))
    v4 = 1;
  if ((*(unsigned int *)((long)a0 + 0x48) & 2) || (v4)) {
    v3 = *(char **)((long)a1 + 0x30);
    if (!stat(v3,v1)) goto label_209a5;
    if ((*__errno_location() == 2) && (v3 = *(char **)((long)a1 + 0x30), !lstat(v3,v1))) {
      *__errno_location() = 0;
      return 0xd;
    }
    *(int *)((long)a1 + 0x40) = *__errno_location();
  }
  else {
    v3 = *(char **)((long)a1 + 0x30);
    v2 = *(int *)((long)a0 + 0x2c);
    if (!fstatat(v2,v3,v1,0x100)) {
label_209a5:
      if ((*(unsigned int *)((long)a1 + 0x88) & 0xf000) == 0x4000) {
        if ((*(char *)((long)a1 + 0x100) == '.') && ((!*(char *)((long)a1 + 0x101) || ((*(char *)((long)a1 + 0x101) == '.' && (!*(char *)((long)a1 + 0x102))))))) {
          if (!*(long *)((long)a1 + 0x58))
            return 1;
          return 5;
        }
        return 1;
      }
      if ((*(unsigned int *)((long)a1 + 0x88) & 0xf000) == 0xa000)
        return 0xc;
      if ((*(unsigned int *)((long)a1 + 0x88) & 0xf000) == 0x8000)
        return 8;
      return 3;
    }
    *(int *)((long)a1 + 0x40) = *__errno_location();
  }
  memset(v1,0,0x90);
  return 10;
}

// Function: sub_20a41 @ 0x20a41
void sub_20a41(long *a0,unsigned long a1)
{
  (**(void **)(*(long *)(*a0 + 0x50) + 0x40))(a0,a1);
}

// Function: sub_20a86 @ 0x20a86
void * sub_20a86(void *a0,void *a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  
  v2 = *(unsigned long *)((long)a0 + 0x40);
  if (*(unsigned long *)((long)a0 + 0x38) < a2) {
    *(unsigned long *)((long)a0 + 0x38) = a2 + 0x28;
    if ((0x2000000000000000 <= *(unsigned long *)((long)a0 + 0x38)) || (v3 = realloc(*(void **)((long)a0 + 0x10),*(long *)((long)a0 + 0x38) * 8), !v3)) {
      free(*(void **)((long)a0 + 0x10));
      *(unsigned long *)((long)a0 + 0x10) = 0;
      *(unsigned long *)((long)a0 + 0x38) = 0;
      return a1;
    }
    *(void **)((long)a0 + 0x10) = v3;
  }
  v4 = *(unsigned long **)((long)a0 + 0x10);
  while (a1) {
    v1 = &v4[1];
    *v4 = a1;
    a1 = *(void **)((long)a1 + 0x10);
    v4 = v1;
  }
  qsort(*(unsigned long *)((long)a0 + 0x10),a2,8,v2);
  v5 = *(long **)((long)a0 + 0x10);
  v3 = (void *)*v5;
  while (a2 = a2 - 1, a2) {
    *(long *)(*v5 + 0x10) = v5[1];
    v5 = &v5[1];
  }
  *(unsigned long *)(*v5 + 0x10) = 0;
  return v3;
}

// Function: sub_20bf4 @ 0x20bf4
void * sub_20bf4(struct_26 *a0,void *a1,unsigned long a2) // early-return
{
  void *v1; // rax
  
  v1 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (!v1)
    return NULL;
  memcpy((void *)((long)v1 + 0x100),a1,a2);
  *(char *)((long)v1 + a2 + 0x100) = 0;
  *(unsigned long *)((long)v1 + 0x60) = a2;
  *(struct_26 **)((long)v1 + 0x50) = a0;
  *(unsigned long *)((long)v1 + 0x38) = a0->field_0x20;
  *(unsigned int *)((long)v1 + 0x40) = 0;
  *(unsigned long *)((long)v1 + 0x18) = 0;
  *(unsigned short *)((long)v1 + 0x6a) = 0;
  *(unsigned short *)((long)v1 + 0x6c) = 3;
  *(unsigned long *)((long)v1 + 0x20) = 0;
  *(unsigned long *)((long)v1 + 0x28) = 0;
  return v1;
}

// Function: sub_20caa @ 0x20caa
void sub_20caa(void *a0)
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

// Function: sub_20ceb @ 0x20ceb
unsigned long sub_20ceb(void *a0,long a1) // return-dupe x2
{
  unsigned long v1; // rax
  void *v2; // rax
  
  v1 = a1 + *(long *)((long)a0 + 0x30) + 0x100;
  if (v1 < *(unsigned long *)((long)a0 + 0x30)) {
    free(*(void **)((long)a0 + 0x20));
    *(unsigned long *)((long)a0 + 0x20) = 0;
    *__errno_location() = 0x24;
    return 0;
  }
  *(unsigned long *)((long)a0 + 0x30) = v1;
  v2 = realloc(*(void **)((long)a0 + 0x20),*(unsigned long *)((long)a0 + 0x30));
  if (!v2) {
    free(*(void **)((long)a0 + 0x20));
    *(unsigned long *)((long)a0 + 0x20) = 0;
    return 0;
  }
  *(void **)((long)a0 + 0x20) = v2;
  return 1;
}

// Function: sub_20dbc @ 0x20dbc
void sub_20dbc(void *a0,void *a1) // ternary
{
  long v1;
  void *v2; // stack - 0x18
  
  v1 = *(long *)((long)a0 + 0x20);
  for (v2 = *(void **)((long)a0 + 8); v2; v2 = *(void **)((long)v2 + 0x10)) {
    if (*(long *)((long)v2 + 0x30) != (long)v2 + 0x100)
      *(long *)((long)v2 + 0x30) = (*(long *)((long)v2 + 0x30) - *(long *)((long)v2 + 0x38)) + v1;
    *(long *)((long)v2 + 0x38) = v1;
  }
  v2 = a1;
  while (0 <= *(long *)((long)v2 + 0x58)) {
    if (*(long *)((long)v2 + 0x30) != (long)v2 + 0x100)
      *(long *)((long)v2 + 0x30) = (*(long *)((long)v2 + 0x30) - *(long *)((long)v2 + 0x38)) + v1;
    *(long *)((long)v2 + 0x38) = v1;
    v2 = (*(long *)((long)v2 + 0x10)) ? *(void **)((long)v2 + 0x10) : *(void **)((long)v2 + 8); // branch-flip
  }
}

// Function: sub_20ecf @ 0x20ecf
long sub_20ecf(long *a0)
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

// Function: sub_20f29 @ 0x20f29
int sub_20f29(struct_40 *a0,void *a1,int a2,char *a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_20f29
{
  bool v1; // al
  int v2;
  stat v3; // stack - 0xa8
  char *v4; // stack - 0xe8
  int v5; // stack - 0xdc
  int v6; // stack - 0xb8
  int v7; // stack - 0xb4
  
  v1 = ((a3) && (!strcmp(a3,"..")));
  if (a0->field_0x48 & 4) {
    if ((a0->field_0x48 & 0x200) && (0 <= a2))
      close(a2);
    return 0;
  }
  v4 = a3;
  v5 = a2;
  if ((((a2 <= -1) && (v1)) && (a0->field_0x48 & 0x200)) && ((sub_23bd0(&a0[1].field_0x0[0x14]) != '\x01' && (v2 = sub_23c72(&a0[1].field_0x0[0x14]), 0 <= v2)))) {
    v5 = v2;
    v4 = NULL;
  }
  v7 = v5;
  if (v5 <= -1) {
    v7 = sub_1ebdd(a0,v4);
    if (v7 <= -1)
      return -1;
  }
  if ((a0->field_0x48 & 2) || ((v4 && (!strcmp(v4,".."))))) {
    if (fstat(v7,&v3)) {
      v6 = -1;
      goto label_211ac;
    }
    if ((*(long *)((long)a1 + 0x70) != v3._0_8_) || (*(long *)((long)a1 + 0x78) != v3._8_8_)) {
      *__errno_location() = 2;
      v6 = -1;
      goto label_211ac;
    }
  }
  if (a0->field_0x48 & 0x200) {
    sub_1ea9f(a0,v7,!v1);
    return 0;
  }
  v6 = fchdir(v7);
label_211ac:
  if (v5 <= -1) {
    v2 = *__errno_location();
    close(v7);
    *__errno_location() = v2;
  }
  return v6;
}

// Function: sub_211f8 @ 0x211f8
unsigned long sub_211f8(struct_38 *a0)
{
  return a0->field_0x10;
}

// Function: sub_2120e @ 0x2120e
unsigned long sub_2120e(struct_58 *a0)
{
  return a0->field_0x18;
}

// Function: sub_21224 @ 0x21224
unsigned long sub_21224(struct_26 *a0)
{
  return a0->field_0x20;
}

// Function: sub_2123a @ 0x2123a
unsigned long sub_2123a(struct_80 *a0)
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

// Function: sub_212bc @ 0x212bc
unsigned long sub_212bc(struct_80 *a0) // return-dupe
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

// Function: sub_2135a @ 0x2135a
void sub_2135a(struct_26 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_21224(a0);
  v2 = sub_211f8(a0);
  v3 = sub_2120e(a0);
  v4 = sub_2123a(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_3c898 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_214a1 @ 0x214a1
long sub_214a1(struct_30 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}

// Function: sub_214fc @ 0x214fc
long sub_214fc(struct_32 *a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_214a1(a0,a1);
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

// Function: sub_21593 @ 0x21593
long sub_21593(struct_80 *a0) // early-return
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

// Function: sub_215f3 @ 0x215f3
long sub_215f3(void *a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_214a1(a0,a1);
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

// Function: sub_21691 @ 0x21691
unsigned long sub_21691(struct_80 *a0,long a1,unsigned long a2)
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

// Function: sub_21731 @ 0x21731
long sub_21731(struct_80 *a0,void *a1,unsigned long a2)
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

// Function: sub_217c9 @ 0x217c9
unsigned long sub_217c9(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_21823 @ 0x21823
unsigned long sub_21823(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_21892 @ 0x21892
unsigned long sub_21892(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_21823(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_218dd @ 0x218dd
void sub_218dd(struct_81 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_3c8a0;
  a0->field_0x8 = dat_3c8a4;
  a0->field_0xc = dat_3c8a8;
  a0->field_0x10 = 0;
}

// Function: sub_21933 @ 0x21933
unsigned long sub_21933(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_33606(a0,3) % a1;
}

// Function: sub_2196e @ 0x2196e
unsigned long sub_2196e(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_2198b @ 0x2198b
unsigned long sub_2198b(struct_25 *a0) // return-dupe x2
{
  float *v1;
  
  v1 = a0->field_0x28;
  if (v1 == (float *)0x3c810)
    return 1;
  if ((((dat_3c8ac < v1[2]) && (v1[2] < dat_3c8a0 - dat_3c8ac)) && (dat_3c8ac + dat_3c8a0 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_3c8ac < v1[1])) && ((v1[1] <= dat_3c8a0 && (*v1 + dat_3c8ac < v1[2]))))))
    return 1;
  a0->field_0x28 = (float *)0x3c810;
  return 0;
}

// Function: sub_21a98 @ 0x21a98
unsigned long sub_21a98(unsigned long a0,void *a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)((long)a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)((long)a1 + 8);
    if (dat_3c8b0 <= v2)
      return 0;
    if (dat_3c8b4 <= v2)
      v3 = (long)(v2 - dat_3c8b4) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_21892(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_21bb1 @ 0x21bb1
long * sub_21bb1(unsigned long a0,void *a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  void *v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_21933;
  v3 = a3;
  if (!a3)
    v3 = sub_2196e;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = (void *)0x3c810;
  v2[5] = (long)v5;
  if (sub_2198b(v2) == '\x01') {
    v2[2] = sub_21a98(a0,v5);
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

// Function: sub_21d26 @ 0x21d26
void sub_21d26(struct_82 *a0)
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

// Function: sub_21e31 @ 0x21e31
void sub_21e31(struct_27 *a0)
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

// Function: sub_21f60 @ 0x21f60
void * sub_21f60(struct_29 *a0)
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

// Function: sub_21faf @ 0x21faf
void sub_21faf(struct_29 *a0,struct_31 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = a0->field_0x48;
  a0->field_0x48 = a1;
}

// Function: sub_21fe9 @ 0x21fe9
long sub_21fe9(struct_32 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_31 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_214a1(a0,a1);
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
      v3 = (struct_31 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_21faf(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_31 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_21faf(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_22182 @ 0x22182
unsigned long sub_22182(struct_34 *a0,struct_80 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_24 *v3; // rax
  struct_24 *v4; // rax
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
        v3 = (struct_24 *)sub_214a1(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_21faf(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_24 *)sub_214a1(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_24 *)sub_21f60(a0);
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

// Function: sub_22356 @ 0x22356
unsigned long sub_22356(struct_35 *a0,unsigned long a1) // return-dupe x4
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
  
  v3 = sub_21a98(a1,a0->field_0x28);
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
  if (sub_22182(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_22182(a0,&v1,1) == '\x01') && (sub_22182(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_22581 @ 0x22581
unsigned long sub_22581(void *a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_21fe9(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(*(long *)((long)a0 + 0x28) + 8) < (float)*(unsigned long *)((long)a0 + 0x18)) && (sub_2198b(a0), (float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(*(long *)((long)a0 + 0x28) + 8) < (float)*(unsigned long *)((long)a0 + 0x18))) {
    v6 = *(long *)((long)a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)((long)a0 + 0x10)); // branch-flip
    if (dat_3c8b0 <= v4)
      return 0xffffffff;
    v1 = (dat_3c8b4 <= v4) ? (long)(v4 - dat_3c8b4) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_22356(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_21fe9(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_21f60(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_228f2 @ 0x228f2
unsigned long sub_228f2(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_22581(a0,a1,&v3);
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

// Function: sub_22962 @ 0x22962
long sub_22962(void *a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_21fe9(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + -1, (float)*(unsigned long *)((long)a0 + 0x18) < (float)*(unsigned long *)((long)a0 + 0x10) * **(float **)((long)a0 + 0x28))) && (sub_2198b(a0), (float)*(unsigned long *)((long)a0 + 0x18) < (float)*(unsigned long *)((long)a0 + 0x10) * **(float **)((long)a0 + 0x28))) {
    v6 = *(long *)((long)a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_3c8b4 <= v3) ? (long)(v3 - dat_3c8b4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)((long)a0 + 0x10));
      v7 = (dat_3c8b4 <= v3) ? (long)(v3 - dat_3c8b4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_22356(a0,v7);
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

// Function: sub_22c68 @ 0x22c68
void sub_22c68(void *a0,long a1)
{
  sub_22962(a0,a1);
}

// Function: sub_22c91 @ 0x22c91
void sub_22c91(void)
{
  return;
}

// Function: sub_22d7a @ 0x22d7a
void * sub_22d7a(void *a0,unsigned long a1,unsigned char *a2,char *a3)
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

// Function: sub_22eb1 @ 0x22eb1
char * sub_22eb1(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_22eb1
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
label_23400:
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
      goto label_237bf;
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
    goto label_23400;
  }
  v11 = (long double)(long)a4;
  if ((long)a4 <= -1)
    v11 = dat_3c950 + v11;
  v12 = (long double)(long)a0;
  if ((long)a0 <= -1)
    v12 = dat_3c950 + v12;
  v13 = (long double)(long)a3;
  if ((long)a3 <= -1)
    v13 = dat_3c950 + v13;
  if (a2 & 0x10) { // branch-flip
    v27 = (long double)1;
    v15 = 0;
    do {
      v27 = v27 * (long double)v5;
      v15 += 1;
      if ((v13 / v11) * v12 < v27 * (long double)v5) break;
    } while (v15 < 8);
    sub_22c91(v4);
    sprintf(a1,"%.1Lf");
    v24 = strlen(a1);
    v25 = v22 + 1;
    if ((v25 + (unsigned long)((a2 & 0x20) == 0) + 1 < v24) || ((a2 & 8 && (a1[v24 - 1] == '0')))) {
      sub_22c91(v4);
      sprintf(a1,"%.0Lf");
      v24 = strlen(a1);
      v25 = 0;
    }
  }
  else {
    sub_22c91(v4);
    sprintf(a1,"%.0Lf");
    v24 = strlen(a1);
    v25 = 0;
  }
  v18 = &v9[-v24];
  memmove(v18,a1,v24);
  v20 = &v18[v24 - v25];
label_237bf:
  if (a2 & 4)
    v18 = (char *)sub_22d7a(v18,(long)v20 - (long)v18,v1,v23);
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
        v10 = *(char *)((long)v15 + 0x3c8c0);
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

// Function: sub_2394b @ 0x2394b
unsigned long sub_2394b(void) // return-dupe
{
  if (getenv("POSIXLY_CORRECT"))
    return 0x200;
  return 0x400;
}

// Function: sub_23975 @ 0x23975
int sub_23975(unsigned char *a0,unsigned long *a1,unsigned int *a2)
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
        *a1 = sub_2394b();
        goto label_23af9;
      }
    }
  }
  if (*v4 == 0x27) {
    v5 = 4;
    v4 = &v4[1];
  }
  v2 = sub_32e49(v4,0x47440,0x3c8f0,4);
  if (0 <= v2) { // branch-flip
    v5 |= *(unsigned int *)((long)v2 * 4 + 0x3c8f0);
    *a1 = 1;
  }
  else {
    v2 = sub_3291e(v4,&v3,0,a1,"eEgGkKmMpPtTyYzZ0");
    if (v2) {
      *a2 = 0;
      return v2;
    }
    while( true ) {
      if (('0' <= (char)*v4) && ((char)*v4 <= '9')) goto label_23af9;
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
label_23af9:
  *a2 = v5;
  return 0;
}

// Function: sub_23b1d @ 0x23b1d
unsigned int sub_23b1d(unsigned char *a0,unsigned int *a1,long *a2)
{
  unsigned int v1; // stack - 0xc
  
  v1 = sub_23975(a0,a2,a1);
  if (!*a2) {
    *a2 = sub_2394b();
    v1 = 4;
  }
  return v1;
}

// Function: sub_23b73 @ 0x23b73
void sub_23b73(void *a0,unsigned int a1)
{
  int v1; // stack - 0xc
  
  *(char *)((long)a0 + 0x1c) = 1;
  *(unsigned int *)((long)a0 + 0x14) = 0;
  *(unsigned int *)((long)a0 + 0x18) = 0;
  for (v1 = 0; v1 <= 3; v1 = v1 + 1) {
    *(unsigned int *)((long)a0 + (long)v1 * 4) = a1;
  }
  *(unsigned int *)((long)a0 + 0x10) = a1;
}

// Function: sub_23bd0 @ 0x23bd0
char sub_23bd0(void *a0)
{
  return *(char *)((long)a0 + 0x1c);
}

// Function: sub_23be6 @ 0x23be6
unsigned int sub_23be6(void *a0,unsigned int a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  
  v2 = (unsigned int)(*(unsigned char *)((long)a0 + 0x1c) ^ 1) + *(int *)((long)a0 + 0x14) & 3;
  v1 = *(unsigned int *)((long)a0 + (unsigned long)v2 * 4);
  *(unsigned int *)((long)a0 + (unsigned long)v2 * 4) = a1;
  *(unsigned int *)((long)a0 + 0x14) = v2;
  if (v2 == *(unsigned int *)((long)a0 + 0x18))
    *(unsigned int *)((long)a0 + 0x18) = (unsigned int)(*(unsigned char *)((long)a0 + 0x1c) ^ 1) + *(int *)((long)a0 + 0x18) & 3;
  *(char *)((long)a0 + 0x1c) = 0;
  return v1;
}

// Function: sub_23c72 @ 0x23c72
unsigned int sub_23c72(void *a0)
{
  unsigned int v1;
  
  if (sub_23bd0(a0))
    abort(); // no-return
  v1 = *(unsigned int *)((long)a0 + (unsigned long)*(unsigned int *)((long)a0 + 0x14) * 4);
  *(unsigned int *)((long)a0 + (unsigned long)*(unsigned int *)((long)a0 + 0x14) * 4) = *(unsigned int *)((long)a0 + 0x10);
  if (*(int *)((long)a0 + 0x14) != *(int *)((long)a0 + 0x18)) // branch-flip
    *(unsigned int *)((long)a0 + 0x14) = *(int *)((long)a0 + 0x14) + 3U & 3;
  else {
    *(char *)((long)a0 + 0x1c) = 1;
  }
  return v1;
}

// Function: sub_23cf8 @ 0x23cf8
unsigned int * sub_23cf8(unsigned int a0) // early-return
{
  passwd *v1; // rax
  char *v2;
  unsigned int *v3; // stack - 0x28
  unsigned int *v4; // stack - 0x20
  
  v4 = NULL;
  v3 = dat_4a210;
  do {
    if (!v3) {
label_23d44:
      if (!v4) {
        v1 = getpwuid(a0);
        if (v1) // branch-flip
          v2 = *(char **)v1;
        else {
          v2 = "";
        }
        v4 = (unsigned int *)sub_31bc2(strlen(v2) + 0x18 & 0xfffffffffffffff8);
        *v4 = a0;
        strcpy((char *)&v4[4],v2);
        *(unsigned int **)&v4[2] = dat_4a210;
        dat_4a210 = v4;
      }
      if ((char)v4[4])
        return &v4[4];
      return NULL;
    }
    if (a0 == *v3) {
      v4 = v3;
      goto label_23d44;
    }
    v3 = *(unsigned int **)&v3[2];
  } while( true );
}

// Function: sub_23def @ 0x23def
unsigned int * sub_23def(char *a0) // early-return
{
  passwd *v1; // rax
  unsigned int *v2; // rax
  unsigned int *v3; // stack - 0x18
  
  v3 = dat_4a210;
  while( true ) {
    if (!v3) {
      v3 = dat_4a218;
      while( true ) {
        if (!v3) {
          v1 = getpwnam(a0);
          v2 = (unsigned int *)sub_31bc2(strlen(a0) + 0x18 & 0xfffffffffffffff8);
          strcpy((char *)&v2[4],a0);
          if (v1) {
            *v2 = *(unsigned int *)&v1->field_0x10;
            *(unsigned int **)&v2[2] = dat_4a210;
            dat_4a210 = v2;
            return v2;
          }
          *(unsigned int **)&v2[2] = dat_4a218;
          dat_4a218 = v2;
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

// Function: sub_23f4a @ 0x23f4a
unsigned int * sub_23f4a(unsigned int a0) // early-return
{
  group *v1; // rax
  char *v2;
  unsigned int *v3; // stack - 0x28
  unsigned int *v4; // stack - 0x20
  
  v4 = NULL;
  v3 = dat_4a220;
  do {
    if (!v3) {
label_23f96:
      if (!v4) {
        v1 = getgrgid(a0);
        if (v1) // branch-flip
          v2 = *(char **)v1;
        else {
          v2 = "";
        }
        v4 = (unsigned int *)sub_31bc2(strlen(v2) + 0x18 & 0xfffffffffffffff8);
        *v4 = a0;
        strcpy((char *)&v4[4],v2);
        *(unsigned int **)&v4[2] = dat_4a220;
        dat_4a220 = v4;
      }
      if ((char)v4[4])
        return &v4[4];
      return NULL;
    }
    if (a0 == *v3) {
      v4 = v3;
      goto label_23f96;
    }
    v3 = *(unsigned int **)&v3[2];
  } while( true );
}

// Function: sub_24041 @ 0x24041
unsigned int * sub_24041(char *a0) // early-return
{
  group *v1; // rax
  unsigned int *v2; // rax
  unsigned int *v3; // stack - 0x18
  
  v3 = dat_4a220;
  while( true ) {
    if (!v3) {
      v3 = dat_4a228;
      while( true ) {
        if (!v3) {
          v1 = getgrnam(a0);
          v2 = (unsigned int *)sub_31bc2(strlen(a0) + 0x18 & 0xfffffffffffffff8);
          strcpy((char *)&v2[4],a0);
          if (v1) {
            *v2 = *(unsigned int *)&v1->field_0x10;
            *(unsigned int **)&v2[2] = dat_4a220;
            dat_4a220 = v2;
            return v2;
          }
          *(unsigned int **)&v2[2] = dat_4a228;
          dat_4a228 = v2;
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

// Function: sub_2419c @ 0x2419c
unsigned long sub_2419c(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_340eb(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_24246 @ 0x24246
unsigned long sub_24246(char *a0,long a1,unsigned long a2,unsigned long *a3) // return-dupe
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
      v2 = sub_342c4(a2 << 3);
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
        goto label_2443b;
      }
    }
    v11 += 1;
    *(unsigned long *)(v10 * 8 + v2) = v10 - v11;
label_2443b:
  }
  *v3 = 0;
  v12 = 0;
  v13 = v8;
  v14 = v8;
  do {
    while( true ) {
      if (!*v14) goto label_24506;
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
label_24506:
  *(unsigned long *)((long)v5 + -8) = 0x24512;
  v16 = v2;
  sub_343d1(v2);
  return 1;
}

// Function: sub_2452d @ 0x2452d
unsigned long sub_2452d(void *a0,void *a1,long *a2)
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
  v22 = sub_34552(a1);
  if (0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0) && !SUB168(ZEXT816(0x38) * ZEXT816(v22),8)) { // branch-flip
    if (0xfa1 <= v22 * 0x38) { // branch-flip
      v3 = sub_342c4(v22 * 0x38);
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
  *(unsigned long *)&v12[-8] = 0x24725;
  v23 = v3;
  v24 = v3;
  v25 = v4;
  v26 = v4;
  memset(v6,0,8);
  while( true ) {
    v33 = 0;
    *(unsigned long *)&v12[-8] = 0x24786;
    sub_256ef(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) break;
    *(unsigned long *)&v12[-8] = 0x2475f;
    sub_34467((struct_16 *)(v24 + v17 * 0x30),&v7);
    v7 = (void *)(v34 + (long)v7);
    v17 += 1;
  }
  *(unsigned long *)(v26 + 8) = 1;
  v19 = 0;
  v18 = 2;
label_24991:
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
          *(unsigned long *)&v12[-8] = 0x248e7;
          if (!memcmp(v1,v2,v3)) {
            v13 = 1;
            goto label_248fa;
          }
        }
        v13 = 0;
      }
label_248fa:
      if (v13) {
        v19 += 1;
        *(unsigned long *)(v18 * 8 + v26) = v18 - v19;
        goto label_24989;
      }
      if (!v19) goto label_2493c;
      v19 -= *(long *)(v26 + v19 * 8);
    } while( true );
  }
  *v14 = 0;
  v20 = 0;
  v29 = v16;
  v10[0] = 0;
  *(unsigned long *)&v12[-8] = 0x249ed;
  memset(v9,0,8);
  v28 = 0;
  v7 = v16;
  v8[0] = 0;
  *(unsigned long *)&v12[-8] = 0x24a1d;
  memset(v6,0,8);
  v33 = 0;
  do {
    *(unsigned long *)&v12[-8] = 0x24c73;
    sub_256ef(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) goto label_24c99;
    if ((*(char *)(v24 + v20 * 0x30 + 0x10)) && (v35))
      v13 = *(int *)(v24 + v20 * 0x30 + 0x14) == v36;
    else {
      if (*(long *)(v24 + v20 * 0x30 + 8) == v34) {
        v3 = *(unsigned long *)(v24 + v20 * 0x30 + 8);
        v2 = *(void **)(v24 + v20 * 0x30);
        *(unsigned long *)&v12[-8] = 0x24b0e;
        if (!memcmp(v2,v7,v3)) {
          v13 = 1;
          goto label_24b21;
        }
      }
      v13 = 0;
    }
label_24b21:
    if (v13) {
      v20 += 1;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
      if (v20 == v22) {
        *v14 = (long)v29;
label_24c99:
        *(unsigned long *)&v12[-8] = 0x24ca8;
        sub_343d1(v23);
        return 1;
      }
    }
    else if (v20) { // branch-flip
      v21 = *(long *)(v26 + v20 * 8);
      v20 -= v21;
      for (; v21; v21 = v21 + -1) {
        *(unsigned long *)&v12[-8] = 0x24bb7;
        sub_256ef(v10);
        if ((v31 != '\x01') || (v32))
          v13 = 1;
        else {
          v13 = 0;
        }
        if (!v13) {
          *(unsigned long *)&v12[-8] = 0x24bde; // no-return
          abort();
        }
        v29 = (void *)(v30 + (long)v29);
        v28 = 0;
      }
    }
    else {
      *(unsigned long *)&v12[-8] = 0x24c17;
      sub_256ef(v10);
      if ((v31 != '\x01') || (v32))
        v13 = 1;
      else {
        v13 = 0;
      }
      if (!v13) {
        *(unsigned long *)&v12[-8] = 0x24c3e; // no-return
        abort();
      }
      v29 = (void *)(v30 + (long)v29);
      v28 = 0;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
    }
  } while( true );
label_2493c:
  *(unsigned long *)(v18 * 8 + v26) = v18;
label_24989:
  v18 += 1;
  goto label_24991;
}

// Function: sub_24cc3 @ 0x24cc3
char * sub_24cc3(char *a0,char *a1) // early-return, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_24cc3
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
    sub_256ef(v7);
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
        sub_256ef(v10);
        if ((v41 != '\x01') || (v42))
          v16 = 1;
        else {
          v16 = 0;
        }
        if (!v16)
          return NULL;
        if (((v2) && (10 <= v18)) && (v18 * 5 <= v19)) {
          for (v21 = v19 - v20; v21; v21 = v21 + -1) {
            sub_256ef(v11);
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
          sub_256ef(v11);
          if ((v36 != '\x01') || (v37))
            v16 = 1;
          else {
            v16 = 0;
          }
          if (!v16) {
            if (sub_2452d(a0,a1,&v12))
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
          sub_256ef(v15);
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
            sub_256ef(v15);
            if ((v51 != '\x01') || (v52))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16)
              return v39;
            sub_256ef(v13);
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
          if (sub_24246(v17,a1,v5,&v12))
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

// Function: sub_25431 @ 0x25431
void sub_25431(char *a0,unsigned int a1)
{
  sub_25467(a0,strlen(a0),a1);
}

// Function: sub_25467 @ 0x25467
int sub_25467(char *a0,long a1,unsigned int a2)
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
label_25620:
    if (v8 < v10) {
      v3 = (int)*v8;
      if (0x7f <= v3) { // branch-flip
label_254ff:
        memset(&v4,0,8);
        do {
          v9 = sub_2419c(&v5,v8,(long)v10 - (long)v8,&v4);
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
            if (v3 > 0x40) goto label_254f1;
            if (0x24 <= v3) { // branch-flip
              if (0x1b > (unsigned int)(v3 - 0x25U)) goto label_254f1;
            }
            else if (v3 > 0x1f) goto label_254f1;
          }
          goto label_254ff;
        }
label_254f1:
        v8 = &v8[1];
        v6 += 1;
      }
      goto label_25620;
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

// Function: sub_256ef @ 0x256ef
void sub_256ef(struct_15 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_3450d((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_258f5;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_30ad4(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_2419c(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
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
label_258f5:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_25906 @ 0x25906
void sub_25906(void *a0,long a1)
{
  *(long *)((long)a0 + 0x10) = *(long *)((long)a0 + 0x10) + a1;
}

// Function: sub_25930 @ 0x25930
void sub_25930(struct_83 *a0,struct_83 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_34467((struct_16 *)&a0[1].field_0x1[2],(struct_17 *)&a1[1].field_0x1[2]);
}

// Function: sub_259c5 @ 0x259c5
unsigned int sub_259c5(unsigned int a0)
{
  return a0;
}

// Function: sub_259d5 @ 0x259d5
struct_77 * sub_259d5(unsigned int a0,unsigned int a1)
{
  struct_77 *v1; // rax
  
  v1 = (struct_77 *)sub_31bc2(0x20);
  v1->field_0x0 = 0x3d;
  v1->field_0x1 = 1;
  v1->field_0x4 = 0xfff;
  v1->field_0x8 = a0;
  v1->field_0xc = a1;
  v1->field_0x11 = 0;
  return v1;
}

// Function: sub_25a35 @ 0x25a35
void * sub_25a35(char *a0) // early-return x3, ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_25a35
{
  long v1;
  unsigned int v10; // stack - 0x4c
  unsigned int v11; // stack - 0x48
  unsigned int v12; // stack - 0x44
  unsigned int v13; // stack - 0x40
  unsigned int v14; // stack - 0x3c
  long v15; // stack - 0x30
  char *v16; // stack - 0x28
  unsigned long v17; // stack - 0x20
  char v2;
  char v3;
  unsigned int v4; // eax
  unsigned int v5; // eax
  int v6; // eax
  void *v7; // rax
  char *v8;
  char v9; // stack - 0x4e
  
  v15 = 0;
  if (('0' <= *a0) && (*a0 <= '7')) {
    v10 = 0;
    v16 = a0;
    do {
      v8 = &v16[1];
      v10 = ((int)*v16 + v10 * 8) - 0x30;
      if (0x1000 <= v10)
        return NULL;
    } while (('0' <= *v8) && (v16 = v8, *v8 <= '7'));
    if (*v8)
      return NULL;
    v4 = sub_259c5(v10);
    v5 = (5 <= (long)v8 - (long)a0) ? 0xfff : v4 & 0xc00 | 0x3ff; // branch-flip
    return (void *)sub_259d5(v4,v5);
  }
  v17 = 1;
  for (v16 = a0; *v16; v16 = &v16[1]) {
    v6 = !(((*v16 != '=') && (*v16 != '+')) && (*v16 != '-'));
    v17 += (long)v6;
  }
  v7 = (void *)sub_31d3b(v17,0x10);
  v16 = a0;
label_25b94:
  v11 = 0;
  do {
    v2 = *v16;
    if (v2 != 'u') { // branch-flip
      if ('u' < v2) {
        free(v7); // return-dupe
        return NULL;
      }
      if (v2 != 'o') { // branch-flip
        if ('o' < v2) {
          free(v7);
          return NULL;
        }
        if (v2 != 'g') { // branch-flip
          if ('g' < v2) {
            free(v7);
            return NULL;
          }
          if (v2 != 'a') goto label_25bd4;
          v11 = 0xfff;
        }
        else {
          v11 |= 0x438;
        }
      }
      else {
        v11 |= 0x207;
      }
    }
    else {
      v11 |= 0x9c0;
    }
    v16 = &v16[1];
  } while( true );
label_25bd4:
  if (('b' <= v2) || ((v2 != '=' && (('>' <= v2 || ((v2 != '+' && (v2 != '-')))))))) {
    free(v7);
    return NULL;
  }
  do {
    v8 = &v16[1];
    v2 = *v16;
    v13 = 0;
    v9 = '\x03';
    v3 = *v8;
    if (v3 != 'u') { // branch-flip
      if ('u' < v3) {
label_25d55:
        v12 = 0;
        v9 = '\x01';
        v16 = v8;
        do {
          switch(*v16) {
            case 0x58:
              v9 = '\x02';
              break;
            default:
              goto label_25dcc;
            case 0x72:
              v12 |= 0x124;
              break;
            case 0x73:
              v12 |= 0xc00;
              break;
            case 0x74:
              v12 |= 0x200;
              break;
            case 0x77:
              v12 |= 0x92;
              break;
            case 0x78:
              v12 |= 0x49;
            
          }
          v16 = &v16[1];
        } while( true );
      }
      if (v3 != 'o') { // branch-flip
        if ('o' < v3) goto label_25d55;
        if ('8' <= v3) { // branch-flip
          if (v3 != 'g') goto label_25d55;
          v12 = 0x38;
          v16 = &v16[2];
        }
        else {
          if (v3 <= '/') goto label_25d55;
          v14 = 0;
          v16 = v8;
          do {
            v8 = &v16[1];
            v14 = ((int)*v16 + v14 * 8) - 0x30;
            if (0x1000 <= v14) {
              free(v7);
              return NULL;
            }
            v16 = v8;
          } while (('0' <= *v8) && (*v8 <= '7'));
          if ((v11) || ((*v8 && (*v8 != ',')))) {
            free(v7);
            return NULL;
          }
          v13 = 0xfff;
          v11 = 0xfff;
          v12 = sub_259c5(v14);
          v9 = '\x01';
        }
      }
      else {
        v12 = 7;
        v16 = &v16[2];
      }
    }
    else {
      v12 = 0x1c0;
      v16 = &v16[2];
    }
label_25dcc:
    v1 = v15 + 1;
    v8 = (char *)((long)v7 + v15 * 0x10);
    *v8 = v2;
    v8[1] = v9;
    *(unsigned int *)&v8[4] = v11;
    *(unsigned int *)&v8[8] = v12;
    if (v13) // branch-flip
      v12 = v13;
    else if (v11)
      v12 = v11 & v12;
    *(unsigned int *)&v8[0xc] = v12;
    v15 = v1;
  } while (((*v16 == '=') || (*v16 == '+')) || (*v16 == '-'));
  if (*v16 != ',') {
    if (!*v16) {
      *(char *)((long)v7 + v1 * 0x10 + 1) = 0;
      return v7;
    }
    free(v7);
    return NULL;
  }
  v16 = &v16[1];
  goto label_25b94;
}

// Function: sub_25ebd @ 0x25ebd
unsigned long sub_25ebd(char *a0) // early-return
{
  stat v1; // stack - 0xa8
  
  if (stat(a0,&v1))
    return 0;
  return sub_259d5(v1._24_4_,0xfff);
}

// Function: sub_25f2e @ 0x25f2e
unsigned int sub_25f2e(unsigned int a0,bool a1,unsigned int a2,char *a3,unsigned int *a4) // ternary x5
{
  char v1;
  unsigned int v10; // stack - 0x18
  unsigned int v2; // eax
  unsigned int v3; // eax
  unsigned int v4;
  unsigned int v5; // edx
  unsigned int v6;
  char *v7; // stack - 0x40
  unsigned int v8; // stack - 0x20
  unsigned int v9; // stack - 0x1c
  
  v8 = a0 & 0xfff;
  v9 = 0;
  for (v7 = a3; v7[1]; v7 = &v7[0x10]) {
    v4 = *(unsigned int *)&v7[4];
    v5 = (a1) ? 0xc00 : 0; // branch-flip
    v10 = *(unsigned int *)&v7[8];
    v1 = v7[1];
    if (v1 != '\x03') { // branch-flip
      if ((('\x04' <= v1) || (v1 == '\x01')) || ((v1 != '\x02' || (!a1 && !(v8 & 0x49))))) goto label_26026;
      v10 |= 0x49;
    }
    else {
      v10 &= v8;
      v6 = (v10 & 0x124) ? 0x124 : 0; // branch-flip
      v2 = (v10 & 0x92) ? 0x92 : 0; // branch-flip
      v3 = (v10 & 0x49) ? 0x49 : 0; // branch-flip
      v10 |= v3 | v6 | v2;
    }
label_26026:
    if (v4) // branch-flip
      v6 = v4;
    else {
      v6 = ~a2;
    }
    v10 &= v6 & ~(~*(unsigned int *)&v7[0xc] & v5);
    v1 = *v7;
    if (v1 != '=') { // branch-flip
      if (v1 <= '=') {
        if (v1 != '+') { // branch-flip
          if (v1 == '-') {
            v9 |= v10;
            v8 &= ~v10;
          }
        }
        else {
          v9 |= v10;
          v8 |= v10;
        }
      }
    }
    else {
      v4 = (v4) ? ~v4 : 0; // branch-flip
      v4 |= ~*(unsigned int *)&v7[0xc] & v5;
      v9 |= ~v4 & 0xfff;
      v8 = v8 & v4 | v10;
    }
  }
  if (a4)
    *a4 = v9;
  return v8;
}

// Function: sub_260da @ 0x260da
unsigned long sub_260da(void)
{
  return 0xffffffffffffffff;
}

// Function: sub_260ef @ 0x260ef
void sub_260ef(char *a0)
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  v3 = 0;
  v1 = strlen(a0) + 1;
  for (v2 = 0; v2 < v1; v2 = v2 + 1) {
    if ((((a0[v2] != '\\') || (v1 <= v2 + 4)) || ((a0[v2 + 1] <= '/' || ('4' <= a0[v2 + 1])))) || (((a0[v2 + 2] <= '/' || ('8' <= a0[v2 + 2])) || ((a0[v2 + 3] <= '/' || ('8' <= a0[v2 + 3]))))))
      a0[v3] = a0[v2];
    else {
      a0[v3] = a0[v2 + 3] + (a0[v2 + 2] + '\xd0' + (a0[v2 + 1] + '\xd0') * '\b') * '\b' + '\xd0';
      v2 += 3;
    }
    v3 += 1;
  }
}

// Function: sub_26298 @ 0x26298
char * sub_26298(char *a0)
{
  char *v1; // rax
  
  v1 = strchr(a0,0x20);
  if (v1)
    *v1 = '\0';
  return v1;
}

// Function: sub_262d1 @ 0x262d1
long * sub_262d1(void) // warn: iteboolean: re-rolled 4 0/1 select diamond(s) to a boolean assignment in sub_262d1
{
  char *v1;
  int v10; // stack - 0xa0
  unsigned long v11; // stack - 0x78
  char *v12; // stack - 0x80
  int v13; // stack - 0x94
  int v14; // stack - 0x90
  int v15; // stack - 0x8c
  long **v16; // stack - 0x70
  char *v17; // stack - 0x68
  FILE *v18; // stack - 0x60
  char *v19; // stack - 0x58
  unsigned char v2; // al
  unsigned long *v20; // stack - 0x50
  long *v21; // stack - 0x48
  char *v22; // stack - 0x40
  long v23; // stack - 0x38
  char *v24; // stack - 0x30
  char *v25; // stack - 0x28
  char *v26; // stack - 0x20
  char *v27; // stack - 0x18
  char v3; // al
  int v4; // eax
  long v5; // rax
  char *v6; // rax
  long *v7; // stack - 0x88
  int v8; // stack - 0x98
  int v9; // stack - 0x9c
  
  v16 = &v7;
  v17 = "/proc/self/mountinfo";
  v18 = fopen("/proc/self/mountinfo","re");
  if (v18) { // branch-flip
    v12 = NULL;
    v11 = 0;
    while (getline(&v12,&v11,v18) != -1) {
      v14 = __isoc99_sscanf(v12,"%*u %*u %u:%u %n",&v10,&v9,&v8);
      if ((v14 == 2) || (v14 == 3)) {
        v22 = &v12[v8];
        v23 = sub_26298(v22);
        if (v23) {
          v24 = (char *)(v23 + 1);
          v23 = sub_26298(v24);
          if (v23) {
            v25 = strstr((char *)(v23 + 1)," - ");
            if (v25) {
              v26 = &v25[3];
              v23 = sub_26298(v26);
              if (v23) {
                v27 = (char *)(v23 + 1);
                if (sub_26298(v27)) {
                  sub_260ef(v27);
                  sub_260ef(v24);
                  sub_260ef(v22);
                  sub_260ef(v26);
                  v21 = (long *)sub_31bc2(0x38);
                  v5 = sub_321c9(v27);
                  *v21 = v5;
                  v5 = sub_321c9(v24);
                  v21[1] = v5;
                  v5 = sub_321c9(v22);
                  v21[2] = v5;
                  v5 = sub_321c9(v26);
                  v21[3] = v5;
                  *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] | 4;
                  v5 = gnu_dev_makedev(v10,v9);
                  v21[4] = v5;
                  v1 = (char *)v21[3];
                  v2 = !((((((((strcmp(v1,"autofs")) && (v1 = (char *)v21[3], strcmp(v1,"proc"))) && (v1 = (char *)v21[3], strcmp(v1,"subfs"))) && ((v1 = (char *)v21[3], strcmp(v1,"debugfs") && (v1 = (char *)v21[3], strcmp(v1,"devpts"))))) && ((v1 = (char *)v21[3], strcmp(v1,"fusectl") && ((v1 = (char *)v21[3], strcmp(v1,"fuse.portal") && (v1 = (char *)v21[3], strcmp(v1,"mqueue"))))))) && (v1 = (char *)v21[3], strcmp(v1,"rpc_pipefs"))) && (((v1 = (char *)v21[3], strcmp(v1,"sysfs") && (v1 = (char *)v21[3], strcmp(v1,"devfs"))) && (v1 = (char *)v21[3], strcmp(v1,"kernfs"))))) && ((v1 = (char *)v21[3], strcmp(v1,"ignore") && (v1 = (char *)v21[3], strcmp(v1,"none")))));
                  *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] & 0xfe | v2;
                  v1 = (char *)*v21;
                  v3 = (((((strchr(v1,0x3a)) || (((*(char *)*v21 == '/' && (*(char *)(*v21 + 1) == '/')) && ((v1 = (char *)v21[3], !strcmp(v1,"smbfs") || ((v1 = (char *)v21[3], !strcmp(v1,"smb3") || (v1 = (char *)v21[3], !strcmp(v1,"cifs"))))))))) || (v1 = (char *)v21[3], !strcmp(v1,"acfs"))) || (((v1 = (char *)v21[3], !strcmp(v1,"afs") || (v1 = (char *)v21[3], !strcmp(v1,"coda"))) || (v1 = (char *)v21[3], !strcmp(v1,"auristorfs"))))) || ((((v1 = (char *)v21[3], !strcmp(v1,"fhgfs") || (v1 = (char *)v21[3], !strcmp(v1,"gpfs"))) || ((v1 = (char *)v21[3], !strcmp(v1,"ibrix") || ((v1 = (char *)v21[3], !strcmp(v1,"ocfs2") || (v1 = (char *)v21[3], !strcmp(v1,"vxfs"))))))) || (v1 = (char *)*v21, !strcmp("-hosts",v1)))));
                  *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] & 0xfd | v3 * '\x02';
                  *v16 = v21;
                  v16 = (long **)&v21[6];
                }
              }
            }
          }
        }
      }
    }
    free(v12);
    if (ferror_unlocked(v18)) { // branch-flip
      v13 = *__errno_location();
      sub_1dad5(v18);
      *__errno_location() = v13;
    }
    else {
      v4 = sub_1dad5(v18);
      if (v4 != -1) goto label_26eea;
    }
  }
  else {
    v19 = "/etc/mtab";
    v18 = setmntent("/etc/mtab","r");
    if (!v18)
      return NULL;
    while( true ) {
      v20 = getmntent(v18);
      if (!v20) break;
      v6 = hasmntopt(v20,"bind");
      v21 = (long *)sub_31bc2(0x38);
      v1 = (char *)*v20;
      *v21 = sub_321c9(v1);
      v5 = sub_321c9((char *)v20[1]);
      v21[1] = v5;
      v21[2] = 0;
      v5 = sub_321c9((char *)v20[2]);
      v21[3] = v5;
      *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] | 4;
      v1 = (char *)v21[3];
      v2 = !(((((((strcmp(v1,"autofs")) && (v1 = (char *)v21[3], strcmp(v1,"proc"))) && (v1 = (char *)v21[3], strcmp(v1,"subfs"))) && ((v1 = (char *)v21[3], strcmp(v1,"debugfs") && (v1 = (char *)v21[3], strcmp(v1,"devpts"))))) && (v1 = (char *)v21[3], strcmp(v1,"fusectl"))) && (((v1 = (char *)v21[3], strcmp(v1,"fuse.portal") && (v1 = (char *)v21[3], strcmp(v1,"mqueue"))) && ((v1 = (char *)v21[3], strcmp(v1,"rpc_pipefs") && (((v1 = (char *)v21[3], strcmp(v1,"sysfs") && (v1 = (char *)v21[3], strcmp(v1,"devfs"))) && (v1 = (char *)v21[3], strcmp(v1,"kernfs"))))))))) && ((v1 = (char *)v21[3], strcmp(v1,"ignore") && ((v1 = (char *)v21[3], strcmp(v1,"none") || (v6))))));
      *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] & 0xfe | v2;
      v1 = (char *)*v21;
      v3 = (((strchr(v1,0x3a)) || ((((*(char *)*v21 == '/' && (*(char *)(*v21 + 1) == '/')) && ((v1 = (char *)v21[3], !strcmp(v1,"smbfs") || ((v1 = (char *)v21[3], !strcmp(v1,"smb3") || (v1 = (char *)v21[3], !strcmp(v1,"cifs"))))))) || (v1 = (char *)v21[3], !strcmp(v1,"acfs"))))) || (((((v1 = (char *)v21[3], !strcmp(v1,"afs") || (v1 = (char *)v21[3], !strcmp(v1,"coda"))) || (v1 = (char *)v21[3], !strcmp(v1,"auristorfs"))) || ((v1 = (char *)v21[3], !strcmp(v1,"fhgfs") || (v1 = (char *)v21[3], !strcmp(v1,"gpfs"))))) || (((v1 = (char *)v21[3], !strcmp(v1,"ibrix") || ((v1 = (char *)v21[3], !strcmp(v1,"ocfs2") || (v1 = (char *)v21[3], !strcmp(v1,"vxfs"))))) || (v1 = (char *)*v21, !strcmp("-hosts",v1)))))));
      *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] & 0xfd | v3 * '\x02';
      v5 = sub_260da(v20[3]);
      v21[4] = v5;
      *v16 = v21;
      v16 = (long **)&v21[6];
    }
    if (endmntent(v18)) {
label_26eea:
      *v16 = NULL;
      return v7;
    }
  }
  v15 = *__errno_location();
  *v16 = NULL;
  while (v7) {
    v21 = (long *)v7[6];
    sub_26f67(v7);
    v7 = v21;
  }
  *__errno_location() = v15;
  return NULL;
}

// Function: sub_26f67 @ 0x26f67
void sub_26f67(struct_66 *a0)
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0->field_0x10);
  if (a0->field_0x28 & 4)
    free(a0->field_0x18);
  free(a0);
}

// Function: sub_26fd4 @ 0x26fd4
void sub_26fd4(int a0)
{
  error(dat_49830,a0,gettext("unable to record current working directory"));
  abort(); // no-return
}

// Function: sub_27011 @ 0x27011
void sub_27011(int a0)
{
  error(dat_49830,a0,gettext("failed to return to initial working directory"));
  abort(); // no-return
}

// Function: sub_2704e @ 0x2704e
void sub_2704e(int a0,char *a1,unsigned int a2)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa0
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = v2;
  sub_3121c(openat(a0,a1,a2,(unsigned long)v1));
}

// Function: sub_2718c @ 0x2718c
DIR * sub_2718c(unsigned int a0,char *a1,unsigned int a2,int *a3) // early-return
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  
  v2 = sub_2704e(a0,a1,a2 | 0x90900);
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

// Function: sub_2721e @ 0x2721e
unsigned long sub_2721e(void)
{
  return 0;
}

// Function: sub_27231 @ 0x27231
char sub_27231(char a0)
{
  return a0;
}

// Function: sub_27244 @ 0x27244
void sub_27244(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  fputs("date: ",stderr);
  v3 = 8;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  vfprintf(stderr,a8,&v3);
}

// Function: sub_27333 @ 0x27333
unsigned long sub_27333(void)
{
  return 0;
}

// Function: sub_27346 @ 0x27346
void sub_27346(void *a0) // return-dupe x2
{
  unsigned long v1; // stack + 0x8
  long v2; // stack + 0x10
  long v3; // stack + 0x18
  
  if ((((*(long *)((long)a0 + 0xa8)) && (!*(long *)((long)a0 + 0x30))) && (*(char *)((long)a0 + 0xa1) != '\x01')) && ((*(long *)((long)a0 + 200) || (3 <= v3)))) {
    *(char *)((long)a0 + 0xd8) = 1;
    *(unsigned long *)((long)a0 + 0x20) = v1;
    *(long *)((long)a0 + 0x28) = v2;
    *(long *)((long)a0 + 0x30) = v3;
    return;
  }
  if (5 <= v3) {
    *(long *)((long)a0 + 0xa8) = *(long *)((long)a0 + 0xa8) + 1;
    *(long *)((long)a0 + 0x40) = v2 % 100;
    *(long *)((long)a0 + 0x38) = (v2 / 100) % 100;
    *(long *)((long)a0 + 0x28) = v2 / 10000;
    *(long *)((long)a0 + 0x30) = v3 + -4;
    return;
  }
  *(long *)((long)a0 + 200) = *(long *)((long)a0 + 200) + 1;
  if (3 <= v3) { // branch-flip
    *(long *)((long)a0 + 0x48) = v2 / 100;
    *(long *)((long)a0 + 0x50) = v2 % 100;
  }
  else {
    *(long *)((long)a0 + 0x48) = v2;
    *(unsigned long *)((long)a0 + 0x50) = 0;
  }
  *(unsigned long *)((long)a0 + 0x58) = 0;
  *(unsigned long *)((long)a0 + 0x60) = 0;
  *(unsigned int *)((long)a0 + 0x1c) = 2;
}

// Function: sub_275d8 @ 0x275d8
bool sub_275d8(void *a0,int a1)
{
  int v1;
  long v10; // stack + 0x10
  long v11; // stack + 0x18
  long v12; // stack + 0x20
  long v13; // stack + 0x28
  long v14; // stack + 0x30
  int v15; // stack + 0x38
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  long v7;
  bool v8;
  long v9; // stack + 0x8
  
  if (0 <= a1) { // branch-flip
    v1 = *(int *)((long)a0 + 0x98);
    *(int *)((long)a0 + 0x98) = v15 + v1;
    v2 = *(long *)((long)a0 + 0x90);
    *(long *)((long)a0 + 0x90) = v14 + v2;
    v3 = *(long *)((long)a0 + 0x88);
    *(long *)((long)a0 + 0x88) = v13 + v3;
    v4 = *(long *)((long)a0 + 0x80);
    *(long *)((long)a0 + 0x80) = v12 + v4;
    v5 = *(long *)((long)a0 + 0x78);
    *(long *)((long)a0 + 0x78) = v11 + v5;
    v6 = *(long *)((long)a0 + 0x70);
    *(long *)((long)a0 + 0x70) = v10 + v6;
    v7 = *(long *)((long)a0 + 0x68);
    *(long *)((long)a0 + 0x68) = v9 + v7;
    v8 = SCARRY8(v9,v7) || (((((SCARRY4(v15,v1) || SCARRY8(v14,v2)) || SCARRY8(v13,v3)) || SCARRY8(v12,v4)) || SCARRY8(v11,v5)) || SCARRY8(v10,v6));
  }
  else {
    v1 = *(int *)((long)a0 + 0x98);
    *(int *)((long)a0 + 0x98) = v1 - v15;
    v2 = *(long *)((long)a0 + 0x90);
    *(long *)((long)a0 + 0x90) = v2 - v14;
    v3 = *(long *)((long)a0 + 0x88);
    *(long *)((long)a0 + 0x88) = v3 - v13;
    v4 = *(long *)((long)a0 + 0x80);
    *(long *)((long)a0 + 0x80) = v4 - v12;
    v5 = *(long *)((long)a0 + 0x78);
    *(long *)((long)a0 + 0x78) = v5 - v11;
    v6 = *(long *)((long)a0 + 0x70);
    *(long *)((long)a0 + 0x70) = v6 - v10;
    v7 = *(long *)((long)a0 + 0x68);
    *(long *)((long)a0 + 0x68) = v7 - v9;
    v8 = SBORROW8(v7,v9) || (((((SBORROW4(v1,v15) || SBORROW8(v2,v14)) || SBORROW8(v3,v13)) || SBORROW8(v4,v12)) || SBORROW8(v5,v11)) || SBORROW8(v6,v10));
  }
  if (!v8)
    *(char *)((long)a0 + 0xa1) = 1;
  return !v8;
}

// Function: sub_278cd @ 0x278cd
void sub_278cd(void *a0,unsigned long a1,unsigned long a2,unsigned long a3,int a4)
{
  *(unsigned long *)((long)a0 + 0x48) = a1;
  *(unsigned long *)((long)a0 + 0x50) = a2;
  *(unsigned long *)((long)a0 + 0x58) = a3;
  *(long *)((long)a0 + 0x60) = (long)a4;
}

// Function: sub_2791e @ 0x2791e
char * sub_2791e(void *a0,char *a1,int a2)
{
  int v1; // stack - 0xc
  
  if (*(char *)((long)a0 + 0xdf)) { // branch-flip
    if ((-1 <= *(long *)((long)a0 + 8)) && (*(long *)((long)a0 + 8) <= 0xc))
      v1 = snprintf(a1,(long)a2,"%s",&"last"[(*(long *)((long)a0 + 8) + 1) * 0xb]);
    else {
      v1 = snprintf(a1,(long)a2,"%ld",*(long *)((long)a0 + 8));
    }
  }
  else {
    *a1 = '\0';
    v1 = 0;
  }
  if ((((0 <= *(int *)((long)a0 + 0x10)) && (*(int *)((long)a0 + 0x10) <= 6)) && (0 <= v1)) && (v1 < a2))
    snprintf(&a1[v1],(long)(a2 - v1),&" %s"[(int)(unsigned int)(v1 == 0)],&"Sun"[(long)*(int *)((long)a0 + 0x10) * 4]);
  return a1;
}

// Function: sub_27a68 @ 0x27a68
char * sub_27a68(int a0,char *a1) // ternary
{
  char *v1;
  int v2;
  char v3; // al
  int v4; // eax
  char *v5; // stack - 0x10
  
  v3 = (0 <= a0) ? '+' : '-'; // branch-flip
  v4 = abs(a0 / 0xe10);
  v1 = &a1[sprintf(a1,"%c%02d",v3,v4)];
  v4 = abs(a0 % 0xe10);
  if (v4) {
    v2 = v4 / 0x3c;
    v4 %= 0x3c;
    *v1 = ':';
    v1[1] = (char)(v2 / 10) + '0';
    v5 = &v1[3];
    v1[2] = (char)v2 + (char)(v2 / 10) * '\xf6' + '0';
    if (v4) {
      *v5 = ':';
      v1[4] = (char)(v4 / 10) + '0';
      v5 = &v1[6];
      v1[5] = (char)v4 + (char)(v4 / 10) * '\xf6' + '0';
    }
    *v5 = '\0';
  }
  return a1;
}

// Function: sub_27c77 @ 0x27c77
void sub_27c77(unsigned long a0,void *a1) // return-dupe, ternary
{
  int v1;
  unsigned int v2;
  long v3;
  char *v4; // rax
  char v5 [104];
  bool v6; // stack - 0xad
  
  v6 = 0;
  if (sub_27333(a1) != '\x01')
    return;
  sub_27244(gettext("parsed %s part: "),a0);
  if ((*(long *)((long)a1 + 0xa8)) && (*(char *)((long)a1 + 0xd9) != '\x01')) {
    fprintf(stderr,"(Y-M-D) %04ld-%02ld-%02ld",*(long *)((long)a1 + 0x28),*(long *)((long)a1 + 0x38),*(long *)((long)a1 + 0x40));
    *(char *)((long)a1 + 0xd9) = 1;
    v6 = 1;
  }
  if (*(char *)((long)a1 + 0xd8) != *(char *)((long)a1 + 0xde)) {
    if (v6)
      fputc(0x20,stderr);
    v3 = *(long *)((long)a1 + 0x28);
    fprintf(stderr,gettext("year: %04ld"),v3);
    *(char *)((long)a1 + 0xde) = *(char *)((long)a1 + 0xd8);
    v6 = 1;
  }
  if ((*(long *)((long)a1 + 200)) && (*(char *)((long)a1 + 0xdc) != '\x01')) {
    fprintf(stderr,&" %02ld:%02ld:%02ld"[(int)(unsigned int)(v6 ^ 1)],*(unsigned long *)((long)a1 + 0x48),*(unsigned long *)((long)a1 + 0x50),*(unsigned long *)((long)a1 + 0x58));
    if (*(long *)((long)a1 + 0x60))
      fprintf(stderr,".%09d",(int)*(unsigned long *)((long)a1 + 0x60));
    if (*(int *)((long)a1 + 0x1c) == 1)
      fputs("pm",stderr);
    *(char *)((long)a1 + 0xdc) = 1;
    v6 = 1;
  }
  if ((*(long *)((long)a1 + 0xb0)) && (*(char *)((long)a1 + 0xda) != '\x01')) {
    if (v6)
      fputc(0x20,stderr);
    v1 = *(int *)((long)a1 + 0x10);
    v3 = *(long *)((long)a1 + 8);
    v4 = (char *)sub_2791e(a1,v5,100);
    fprintf(stderr,gettext("%s (day ordinal=%ld number=%d)"),v4,v3,v1);
    *(char *)((long)a1 + 0xda) = 1;
    v6 = 1;
  }
  if ((*(long *)((long)a1 + 0xb8)) && (*(char *)((long)a1 + 0xdb) != '\x01')) {
    v4 = (*(long *)((long)a1 + 0xc0)) ? " DST" : ""; // branch-flip
    fprintf(stderr,&" isdst=%d%s"[(int)(unsigned int)(v6 ^ 1)],(unsigned long)*(unsigned int *)((long)a1 + 0x14),v4);
    *(char *)((long)a1 + 0xdb) = 1;
    v6 = 1;
  }
  if ((*(long *)((long)a1 + 0xd0)) && (*(char *)((long)a1 + 0xdd) != '\x01')) {
    v2 = *(unsigned int *)((long)a1 + 0x18);
    fprintf(stderr,&" UTC%s"[(int)(unsigned int)(v6 ^ 1)],sub_27a68(v2,v5));
    *(char *)((long)a1 + 0xdd) = 1;
    v6 = 1;
  }
  if (*(char *)((long)a1 + 0xa0)) {
    v3 = *(long *)((long)a1 + 0x58);
    if (v6)
      fputc(0x20,stderr);
    fprintf(stderr,gettext("number of seconds: %ld"),v3);
  }
  fputc(10,stderr);
}

// Function: sub_281ae @ 0x281ae
unsigned char sub_281ae(unsigned char a0,long a1,char *a2) // early-return
{
  if (!a1)
    return a0;
  fprintf(stderr,&" %+ld %s"[(int)(unsigned int)(a0 ^ 1)],a1,a2);
  return 1;
}

// Function: sub_2820e @ 0x2820e
void sub_2820e(unsigned long a0,void *a1) // return-dupe x2
{
  FILE *v1;
  char v2; // al
  
  if (sub_27333(a1) != '\x01')
    return;
  sub_27244(gettext("parsed %s part: "),a0);
  v1 = stderr;
  if ((((!*(long *)((long)a1 + 0x68)) && (!*(long *)((long)a1 + 0x70))) && (!*(long *)((long)a1 + 0x78))) && (((!*(long *)((long)a1 + 0x80) && (!*(long *)((long)a1 + 0x88))) && ((!*(long *)((long)a1 + 0x90) && (!*(int *)((long)a1 + 0x98))))))) {
    fputs(gettext("today/this/now\n"),v1);
    return;
  }
  v2 = sub_281ae(0,*(long *)((long)a1 + 0x68),"year(s)");
  v2 = sub_281ae((unsigned char)v2,*(long *)((long)a1 + 0x70),"month(s)");
  v2 = sub_281ae((unsigned char)v2,*(long *)((long)a1 + 0x78),"day(s)");
  v2 = sub_281ae((unsigned char)v2,*(long *)((long)a1 + 0x80),"hour(s)");
  v2 = sub_281ae((unsigned char)v2,*(long *)((long)a1 + 0x88),"minutes");
  v2 = sub_281ae((unsigned char)v2,*(long *)((long)a1 + 0x90),"seconds");
  sub_281ae((unsigned char)v2,(long)*(int *)((long)a1 + 0x98),"nanoseconds");
  fputc(10,stderr);
}

// Function: sub_283f7 @ 0x283f7
void sub_283f7(void)
{
  return;
}

// Function: sub_28423 @ 0x28423
unsigned int sub_28423(long *a0) // return-dupe
{
  unsigned long v1;
  long v10; // rax
  char v11 [24];
  unsigned long v12 [140];
  unsigned long v13; // stack - 0x518
  int v14; // stack - 0x598
  int v15; // stack - 0x590
  int v16; // stack - 0x58c
  int v17; // stack - 0x588
  unsigned int v18; // stack - 0x584
  int v19; // stack - 0x580
  char *v2;
  int v20; // stack - 0x57c
  long v21; // stack - 0x570
  char *v22; // stack - 0x568
  char *v23; // stack - 0x560
  unsigned long *v24; // stack - 0x558
  unsigned long *v25; // stack - 0x550
  unsigned long v26; // stack - 0x510
  unsigned long v27; // stack - 0x508
  unsigned long v28; // stack - 0x500
  unsigned long v29; // stack - 0x4f8
  char v3;
  unsigned long v30; // stack - 0x4f0
  unsigned long v31; // stack - 0x4e8
  unsigned long v32; // stack - 0x4d8
  unsigned long v33; // stack - 0x4d0
  unsigned long v34; // stack - 0x4c8
  unsigned long v35; // stack - 0x4c0
  unsigned long v36; // stack - 0x4b8
  unsigned long v37; // stack - 0x4b0
  unsigned long v38; // stack - 0x4a8
  int v4;
  int v5; // eax
  long v6;
  unsigned long v7;
  char *v8; // rax
  unsigned long *v9;
  
  v15 = 0;
  v16 = 0;
  v21 = 0x14;
  v22 = v11;
  v24 = v12;
  v14 = -2;
  v23 = v22;
  v25 = v24;
  do {
    v2 = v22;
    v20 = 0;
    *v23 = (char)v15;
    if (&v22[v21 + -1] <= v23) {
      v6 = (long)v23 - (long)v22;
      v7 = v6 + 1;
      if (v21 <= 0x13) {
        v21 <<= 1;
        if (0x15 <= v21)
          v21 = 0x14;
        v8 = malloc(v21 * 0x39 + 0x37);
        if (v8) {
          memcpy(v8,v22,v7);
          v22 = v8;
          v9 = (unsigned long *)&v8[((v21 + 0x37) / 0x38) * 0x38];
          memcpy(v9,v24,v7 * 0x38);
          if (v2 != v11)
            free(v2);
          v23 = &v8[v6];
          v25 = &v9[v7 * 7 + -7];
          v24 = v9;
          if (v23 < &v8[v21 + -1]) goto label_28738;
label_2a80b:
          v18 = 1;
label_2a83f:
          if (v14 != -2) {
            if ((0 <= v14) && (v14 <= 0x115))
              v4 = (int)*(char *)((long)v14 + 0x3cce0);
            else {
              v4 = 2;
            }
            sub_283f7("Cleanup: discarding lookahead",v4,&v13,a0);
          }
          v25 = &v25[(long)v20 * -7];
          for (v23 = &v23[-(long)v20]; v23 != v22; v23 = &v23[-1]) {
            sub_283f7("Cleanup: popping",(int)*(char *)((long)(int)*v23 + 0x3d040),v25,a0);
            v25 = &v25[-7];
          }
          if (v22 != v11) {
            free(v22);
            return v18;
          }
          return v18;
        }
      }
      sub_2b67b(a0,"memory exhausted");
      v18 = 2;
      goto label_2a83f;
    }
label_28738:
    if (v15 == 0xc) {
      v18 = 0;
      goto label_2a83f;
    }
    v3 = *(char *)((long)v15 + 0x3ce00);
    if (v3 != -0x5d) {
      if (v14 == -2)
        v14 = sub_2b1ac(&v13,a0);
      if (0 < v14) {
        if (v14 != 0x100) {
          if ((0 <= v14) && (v14 <= 0x115))
            v19 = (int)*(char *)((long)v14 + 0x3cce0);
          else {
            v19 = 2;
          }
          goto label_28813;
        }
        v14 = 0x101;
        goto label_2a67d;
      }
      v14 = 0;
      v19 = 0;
label_28813:
      v4 = v3 + v19;
      if (((v4 < 0) || (0x70 < v4)) || (v19 != *(char *)((long)v4 + 0x3cfc0))) goto label_28915;
      v15 = (int)*(char *)((long)v4 + 0x3cf40);
      if (v15 <= 0) {
        v17 = -v15;
        goto label_2893f;
      }
      if (v16)
        v16 -= 1;
      v9 = &v25[7];
      *v9 = v13;
      v25[8] = v26;
      v25[9] = v27;
      v25[10] = v28;
      v25[0xb] = v29;
      v25[0xc] = v30;
      v25[0xd] = v31;
      v14 = -2;
      v25 = v9;
      goto label_284c7;
    }
label_28915:
    v17 = (int)*(char *)((long)v15 + 0x3ce80);
    if (!v17) {
      if (v14 != -2) { // branch-flip
        if ((0 <= v14) && (v14 <= 0x115))
          v4 = (int)*(char *)((long)v14 + 0x3cce0);
        else {
          v4 = 2;
        }
      }
      else {
        v4 = -2;
      }
      if (!v16)
        sub_2b67b(a0,"syntax error");
      if (v16 == 3) {
        if (1 <= v14) { // branch-flip
          sub_283f7("Error: discarding",v4,&v13,a0);
          v14 = -2;
        }
        else if (!v14) goto label_2a80b;
      }
label_2a67d:
      v16 = 3;
      while( true ) {
        v20 = 0;
        if (((*(char *)((long)v15 + 0x3ce00) != -0x5d) && (v4 = *(char *)((long)v15 + 0x3ce00) + 1, 0 <= v4)) && ((v4 <= 0x70 && ((*(char *)((long)v4 + 0x3cfc0) == '\x01' && (v3 = *(char *)((long)v4 + 0x3cf40), 1 <= v3)))))) break;
        if (v23 == v22) goto label_2a80b;
        sub_283f7("Error: popping",(int)*(char *)((long)v15 + 0x3d040),v25,a0);
        v25 = &v25[-7];
        v23 = &v23[-1];
        v15 = (int)*v23;
      }
      v9 = &v25[7];
      *v9 = v13;
      v25[8] = v26;
      v25[9] = v27;
      v25[10] = v28;
      v25[0xb] = v29;
      v25[0xc] = v30;
      v25[0xd] = v31;
      v15 = (int)v3;
      v25 = v9;
      goto label_284c7;
    }
label_2893f:
    v20 = (int)*(char *)((long)v17 + 0x3d120);
    v9 = &v25[(long)(1 - v20) * 7];
    v32 = *v9;
    v33 = v9[1];
    v34 = v9[2];
    v35 = v9[3];
    v36 = v9[4];
    v37 = v9[5];
    v38 = v9[6];
    switch(v17) {
      case 4:
        v7 = v25[1];
        a0[0xb] = *v25;
        a0[0xc] = v7;
        *(char *)&a0[0x14] = 1;
        sub_27c77(gettext("number of seconds"),a0);
        break;
      default:
        break;
      case 7:
        a0[0x19] = a0[0x19] + 1;
        a0[0x15] = a0[0x15] + 1;
        sub_27c77(gettext("datetime"),a0);
        break;
      case 8:
        a0[0x19] = a0[0x19] + 1;
        sub_27c77(gettext("time"),a0);
        break;
      case 9:
        a0[0x17] = a0[0x17] + 1;
        sub_27c77(gettext("local_zone"),a0);
        break;
      case 10:
        a0[0x1a] = a0[0x1a] + 1;
        sub_27c77(gettext("zone"),a0);
        break;
      case 0xb:
        a0[0x15] = a0[0x15] + 1;
        sub_27c77(gettext("date"),a0);
        break;
      case 0xc:
        a0[0x16] = a0[0x16] + 1;
        sub_27c77(gettext("day"),a0);
        break;
      case 0xd:
        sub_2820e(gettext("relative"),a0);
        break;
      case 0xe:
        sub_27c77(gettext("number"),a0);
        break;
      case 0xf:
        sub_2820e(gettext("hybrid"),a0);
        break;
      case 0x12:
        sub_278cd(a0,v25[-6],0,0,0);
        *(int *)((long)a0 + 0x1c) = (int)*v25;
        break;
      case 0x13:
        sub_278cd(a0,v25[-0x14],v25[-6],0,0);
        *(int *)((long)a0 + 0x1c) = (int)*v25;
        break;
      case 0x14:
        sub_278cd(a0,v25[-0x22],v25[-0x14],v25[-7],v25[-6] & 0xffffffff);
        *(int *)((long)a0 + 0x1c) = (int)*v25;
        break;
      case 0x16:
        sub_278cd(a0,v25[-6],0,0,0);
        *(unsigned int *)((long)a0 + 0x1c) = 2;
        break;
      case 0x17:
        sub_278cd(a0,v25[-0x14],v25[-6],0,0);
        *(unsigned int *)((long)a0 + 0x1c) = 2;
        break;
      case 0x18:
        sub_278cd(a0,v25[-0x22],v25[-0x14],v25[-7],v25[-6] & 0xffffffff);
        *(unsigned int *)((long)a0 + 0x1c) = 2;
        break;
      case 0x1b:
        a0[0x1a] = a0[0x1a] + 1;
        v3 = sub_2a972(a0,*v25);
        goto label_2a426;
      case 0x1c:
        *(int *)((long)a0 + 0x14) = (int)*v25;
        break;
      case 0x1d:
        *(unsigned int *)((long)a0 + 0x14) = 1;
        a0[0x18] = a0[0x18] + 1;
        break;
      case 0x1e:
        *(int *)&a0[3] = (int)*v25;
        break;
      case 0x1f:
        *(unsigned int *)&a0[3] = 0xffff9d90;
        break;
      case 0x20:
        *(int *)&a0[3] = (int)v25[-7];
        if (sub_275d8(a0,1) != '\x01') goto label_2a80b;
        sub_2820e(gettext("relative"),a0);
        break;
      case 0x21:
        *(unsigned int *)&a0[3] = 0xffff9d90;
        if (sub_275d8(a0,1) != '\x01') goto label_2a80b;
        sub_2820e(gettext("relative"),a0);
        break;
      case 0x22:
        v7 = *v25;
        if ((sub_2a972(a0,v7) != '\x01') || (v6 = a0[3], v7 = v25[-0xe], v10 = (long)(int)v6 + v7, v4 = (int)v10, *(int *)&a0[3] = v4, v10 != v4 || SCARRY8((long)(int)v6,v7))) goto label_2a80b;
        break;
      case 0x23:
        *(int *)&a0[3] = (int)*v25 + 0xe10;
        break;
      case 0x24:
        *(int *)&a0[3] = (int)v25[-7] + 0xe10;
        break;
      case 0x25:
        a0[1] = 0;
        *(int *)&a0[2] = (int)*v25;
        break;
      case 0x26:
        a0[1] = 0;
        *(int *)&a0[2] = (int)v25[-7];
        break;
      case 0x27:
        a0[1] = v25[-7];
        *(int *)&a0[2] = (int)*v25;
        *(char *)((long)a0 + 0xdf) = 1;
        break;
      case 0x28:
        a0[1] = v25[-6];
        *(int *)&a0[2] = (int)*v25;
        *(char *)((long)a0 + 0xdf) = 1;
        break;
      case 0x29:
        a0[7] = v25[-0xd];
        a0[8] = v25[1];
        break;
      case 0x2a:
        if (4 <= (long)v25[-0x1a]) { // branch-flip
          if (sub_27333(a0)) {
            v7 = v25[-0x1a];
            v1 = v25[-0x1b];
            sub_27244(gettext("warning: value %ld has %ld digits. Assuming YYYY/MM/DD\n"),v1,v7);
          }
          v7 = v25[-0x1b];
          a0[4] = v25[-0x1c];
          a0[5] = v7;
          a0[6] = v25[-0x1a];
          a0[7] = v25[-0xd];
          a0[8] = v25[1];
        }
        else {
          if (sub_27333(a0)) {
            v7 = v25[-0x1b];
            sub_27244(gettext("warning: value %ld has less than 4 digits. Assuming MM/DD/YY[YY]\n"),v7);
          }
          a0[7] = v25[-0x1b];
          a0[8] = v25[-0xd];
          v7 = v25[1];
          a0[4] = *v25;
          a0[5] = v7;
          a0[6] = v25[2];
        }
        break;
      case 0x2b:
        a0[8] = v25[-0xd];
        a0[7] = v25[-7];
        v7 = v25[1];
        a0[5] = -v7;
        if (SBORROW8(0,v7)) goto label_2a80b;
        a0[6] = v25[2];
        break;
      case 0x2c:
        a0[7] = v25[-0xe];
        v7 = v25[-6];
        a0[8] = -v7;
        if ((SBORROW8(0,v7)) || (v7 = v25[1], a0[5] = -v7, SBORROW8(0,v7))) goto label_2a80b;
        a0[6] = v25[2];
        break;
      case 0x2d:
        a0[7] = v25[-7];
        a0[8] = v25[1];
        break;
      case 0x2e:
        a0[7] = v25[-0x15];
        a0[8] = v25[-0xd];
        v7 = v25[1];
        a0[4] = *v25;
        a0[5] = v7;
        a0[6] = v25[2];
        break;
      case 0x2f:
        a0[8] = v25[-6];
        a0[7] = *v25;
        break;
      case 0x30:
        a0[8] = v25[-0xd];
        a0[7] = v25[-7];
        v7 = v25[1];
        a0[4] = *v25;
        a0[5] = v7;
        a0[6] = v25[2];
        break;
      case 0x32:
        v7 = v25[-0xd];
        a0[4] = v25[-0xe];
        a0[5] = v7;
        a0[6] = v25[-0xc];
        v7 = v25[-6];
        a0[7] = -v7;
        if ((SBORROW8(0,v7)) || (v7 = v25[1], a0[8] = -v7, SBORROW8(0,v7))) goto label_2a80b;
        break;
      case 0x33:
        v3 = sub_275d8(a0,*v25 & 0xffffffff);
        goto label_2a426;
      case 0x34:
        v3 = sub_275d8(a0,1);
        goto label_2a426;
      case 0x35:
        v3 = sub_275d8(a0,1);
        goto label_2a426;
      case 0x36:
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v32 = v25[-7];
        break;
      case 0x37:
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v32 = v25[-6];
        break;
      case 0x38:
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v32 = 1;
        break;
      case 0x39:
        v32 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v33 = v25[-7];
        break;
      case 0x3a:
        v32 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v33 = v25[-6];
        break;
      case 0x3b:
        v32 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v33 = 1;
        break;
      case 0x3c:
        v32 = 0;
        v33 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v34 = *v25 * v25[-7];
        if (SEXT816((long)v34) != SEXT816((long)*v25) * SEXT816((long)v25[-7])) goto label_2a80b;
        break;
      case 0x3d:
        v32 = 0;
        v33 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v34 = *v25 * v25[-6];
        if (SEXT816((long)v34) != SEXT816((long)*v25) * SEXT816((long)v25[-6])) goto label_2a80b;
        break;
      case 0x3e:
        v32 = 0;
        v33 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v34 = *v25;
        break;
      case 0x3f:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v35 = v25[-7];
        break;
      case 0x40:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v35 = v25[-6];
        break;
      case 0x41:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v35 = 1;
        break;
      case 0x42:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v37 = 0;
        v38 = 0;
        v36 = v25[-7];
        break;
      case 0x43:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v37 = 0;
        v38 = 0;
        v36 = v25[-6];
        break;
      case 0x44:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v37 = 0;
        v38 = 0;
        v36 = 1;
        break;
      case 0x45:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v38 = 0;
        v37 = v25[-7];
        break;
      case 0x46:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v38 = 0;
        v37 = v25[-6];
        break;
      case 0x47:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = v25[-7];
        v38 = v25[-6] & 0xffffffff;
        break;
      case 0x48:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = v25[-7];
        v38 = v25[-6] & 0xffffffff;
        break;
      case 0x49:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v38 = 0;
        v37 = 1;
        break;
      case 0x4b:
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v32 = v25[-6];
        break;
      case 0x4c:
        v32 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v33 = v25[-6];
        break;
      case 0x4d:
        v32 = 0;
        v33 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v34 = *v25 * v25[-6];
        if (SEXT816((long)v34) != SEXT816((long)*v25) * SEXT816((long)v25[-6])) goto label_2a80b;
        break;
      case 0x4e:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v35 = v25[-6];
        break;
      case 0x4f:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v37 = 0;
        v38 = 0;
        v36 = v25[-6];
        break;
      case 0x50:
        v32 = 0;
        v33 = 0;
        v34 = 0;
        v35 = 0;
        v36 = 0;
        v38 = 0;
        v37 = v25[-6];
        break;
      case 0x51:
        v32 = 0;
        v33 = 0;
        v35 = 0;
        v36 = 0;
        v37 = 0;
        v38 = 0;
        v34 = *v25;
        break;
      case 0x55:
        v7 = v25[1];
        if (sub_2721e(v7)) goto label_2a80b;
        v32 = v25[1];
        v33 = 0;
        break;
      case 0x57:
        v7 = v25[1];
        if (sub_2721e(v7)) goto label_2a80b;
        v32 = v25[1];
        v33 = 0;
        break;
      case 0x58:
        sub_27346(a0);
        break;
      case 0x59:
        sub_27346(a0);
        v3 = sub_275d8(a0,1);
label_2a426:
        if (v3 != '\x01') goto label_2a80b;
        break;
      case 0x5a:
        v32 = 0xffffffffffffffff;
        break;
      case 0x5b:
        v32 = v25[1];
      
    }
    v23 = &v23[-(long)v20];
    v9 = &v25[(long)v20 * -7 + 7];
    *v9 = v32;
    v25[(long)v20 * -7 + 8] = v33;
    v25[(long)v20 * -7 + 9] = v34;
    v25[(long)v20 * -7 + 10] = v35;
    v25[(long)v20 * -7 + 0xb] = v36;
    v25[(long)v20 * -7 + 0xc] = v37;
    v25[(long)v20 * -7 + 0xd] = v38;
    v4 = *(char *)((long)v17 + 0x3d0c0) + -0x1c;
    v5 = (int)*v23 + (int)*(char *)((long)v4 + 0x3cf00);
    if (((0 <= v5) && (v5 <= 0x70)) && (*(char *)((long)v5 + 0x3cfc0) == *v23))
      v3 = *(char *)((long)v5 + 0x3cf40);
    else {
      v3 = *(char *)((long)v4 + 0x3cf20);
    }
    v15 = (int)v3;
    v25 = v9;
label_284c7:
    v23 = &v23[1];
  } while( true );
}

// Function: sub_2a972 @ 0x2a972
unsigned long sub_2a972(void *a0,long a1) // return-dupe
{
  long v1; // rax
  bool v2;
  bool v3; // stack + 0x8
  long v4; // stack + 0x10
  long v5; // stack + 0x18
  long v6; // stack - 0x18
  
  v2 = 0;
  if ((v5 <= 2) && (a1 <= -1))
    v4 *= 100;
  if (0 <= a1) { // branch-flip
    v1 = v4 * 0x3c;
    if (v3) { // branch-flip
      v2 = SBORROW8(v1,a1);
      a1 = -a1;
    }
    else {
      v2 = SCARRY8(a1,v1);
    }
    v6 = v1 + a1;
    v2 = v2 || SEXT816(v1) != SEXT816(v4) * SEXT816(0x3c);
  }
  else {
    v6 = (v4 / 100) * 0x3c + v4 % 100;
  }
  if (((!v2) && (-0x5a0 <= v6)) && (v6 <= 0x5a0)) {
    *(int *)((long)a0 + 0x18) = (int)v6 * 0x3c;
    return 1;
  }
  return 0;
}

// Function: sub_2ab3b @ 0x2ab3b
unsigned long sub_2ab3b(unsigned long a0,int a1) // early-return x5
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v2 = a0;
  if (a1) { // branch-flip
    if (a1 != 1) { // branch-flip
      if (((long)a0 < 0) || (0x17 < (long)a0)) {
        v2 = 0xffffffff;
        return v2;
      }
    }
    else {
      if (((long)a0 <= 0) || (0xb < (long)a0)) {
        if (a0 != 0xc) {
          v2 = 0xffffffff;
          return v2;
        }
        v2 = 0xc;
        return v2;
      }
      v1 = (int)a0;
      v1 += 0xc;
      v2 = (unsigned long)v1;
    }
  }
  else if (((long)a0 <= 0) || (0xc <= (long)a0)) {
    if (a0 != 0xc) {
      v2 = 0xffffffff;
      return v2;
    }
    v2 = 0;
    return v2;
  }
  return v2;
}

// Function: sub_2abc6 @ 0x2abc6
char * sub_2abc6(int a0,char *a1)
{
  unsigned int v1; // eax
  
  v1 = abs(a0 % 100);
  sprintf(a1,&"-%02d%02d"[(int)(unsigned int)(-0x76c <= a0)],(unsigned long)(unsigned int)abs(a0 / 100 + 0x13),(unsigned long)v1);
  return a1;
}

// Function: sub_2ac65 @ 0x2ac65
char * sub_2ac65(bool a0,int *a1) // return-dupe, ternary
{
  bool v1;
  int v2; // eax
  char *v3; // rax
  long v4; // stack + 0x10
  long v5; // stack + 0x18
  long v6; // stack - 0x20
  
  v6 = v4;
  if ((0 <= v4) && (v5 == 2)) {
    v3 = (0x45 <= v4) ? (char *)0x76c : (char *)0x7d0; // branch-flip
    v6 = v4 + (long)v3;
    if (a0) {
      v3 = gettext("warning: adjusting year value %ld to %ld\n");
      sub_27244(v3,v4,v6);
    }
  }
  if (0 <= v6) { // branch-flip
    v3 = (long)(v6 + -0x76c);
    v1 = v3 != (char *)(long)v2 || SBORROW8(v6,0x76c);
    *a1 = v2;
  }
  else {
    v3 = (long)(0xfffffffffffff894 - v6);
    v1 = v3 != (char *)(long)v2 || SBORROW8(-0x76c,v6);
    *a1 = v2;
  }
  if (!v1) {
    v3 = (char *)0x1;
    return v3;
  }
  if (!a0) {
    v3 = NULL;
    return v3;
  }
  v3 = gettext("error: out-of-range year %ld\n");
  sub_27244(v3,v6);
  v3 = NULL;
  return v3;
}

// Function: sub_2ad8a @ 0x2ad8a
long * sub_2ad8a(void *a0,char *a1)
{
  char *v1;
  long *v2; // stack - 0x10
  
  for (v2 = (long *)0x47880; *v2; v2 = &v2[2]) {
    v1 = (char *)*v2;
    if (!strcmp(a1,v1))
      return v2;
  }
  for (v2 = (long *)((long)a0 + 0xe0); *v2; v2 = &v2[2]) {
    v1 = (char *)*v2;
    if (!strcmp(a1,v1))
      return v2;
  }
  v2 = (long *)0x478c0;
  while( true ) {
    if (!*v2)
      return NULL;
    v1 = (char *)*v2;
    if (!strcmp(a1,v1)) break;
    v2 = &v2[2];
  }
  return v2;
}

// Function: sub_2ae65 @ 0x2ae65
long * sub_2ae65(void *a0,char *a1) // early-return x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_2ae65
{
  char *v1;
  char v2; // al
  bool v3;
  int v4;
  unsigned long v5; // rax
  long *v6; // rax
  char *v7; // stack - 0x28
  char *v8; // stack - 0x20
  long *v9; // stack - 0x18
  
  for (v7 = a1; *v7; v7 = &v7[1]) {
    v2 = sub_27231((int)*v7);
    *v7 = sub_33991((unsigned char)v2);
  }
  for (v9 = (long *)0x47460; *v9; v9 = &v9[2]) {
    v1 = (char *)*v9;
    if (!strcmp(a1,v1))
      return v9;
  }
  v5 = strlen(a1);
  v3 = !((v5 != 3) && ((v5 != 4 || (a1[3] != '.'))));
  for (v9 = (long *)0x474c0; *v9; v9 = &v9[2]) {
    if (v3)
      v4 = strncmp(a1,(char *)*v9,3);
    else {
      v4 = strcmp(a1,(char *)*v9);
    }
    if (!v4)
      return v9;
  }
  v6 = (long *)sub_2ad8a(a0,a1);
  if (v6)
    return v6;
  if (!strcmp(a1,"DST"))
    return (long *)0x474b0;
  for (v9 = (long *)0x47660; *v9; v9 = &v9[2]) {
    v1 = (char *)*v9;
    if (!strcmp(a1,v1))
      return v9;
  }
  if (a1[v5 - 1] == 'S') {
    a1[v5 - 1] = '\0';
    for (v9 = (long *)0x47660; *v9; v9 = &v9[2]) {
      v1 = (char *)*v9;
      if (!strcmp(a1,v1))
        return v9;
    }
    a1[v5 - 1] = 'S';
  }
  for (v9 = (long *)0x47720; *v9; v9 = &v9[2]) {
    v1 = (char *)*v9;
    if (!strcmp(a1,v1))
      return v9;
  }
  if (v5 == 1) {
    for (v9 = (long *)0x47bc0; *v9; v9 = &v9[2]) {
      if (*a1 == *(char *)*v9)
        return v9;
    }
  }
  v3 = 0;
  v7 = a1;
  v8 = a1;
  while (*v7 = *v8, *v7) {
    if (*v8 != '.') // branch-flip
      v7 = &v7[1];
    else {
      v3 = 1;
    }
    v8 = &v8[1];
  }
  if ((v3) && (v6 = (long *)sub_2ad8a(a0,a1), v6))
    return v6;
  return NULL;
}

// Function: sub_2b1ac @ 0x2b1ac
unsigned int sub_2b1ac(unsigned char *a0,long *a1) // return-dupe x3, return-dupe, ternary x2
{
  unsigned char *v1;
  int v10; // stack - 0x64
  int v11; // stack - 0x60
  int v12; // stack - 0x5c
  long v13; // stack - 0x58
  long v14; // stack - 0x50
  unsigned char *v15; // stack - 0x48
  unsigned char *v16; // stack - 0x40
  long v17; // stack - 0x38
  unsigned char v2;
  char v3;
  char *v4;
  int v5; // eax
  long v6; // rax
  unsigned char v7 [19];
  unsigned char v8 [5];
  unsigned char v9; // stack - 0x65
  
  do {
    while( true ) {
      while( true ) {
        v9 = *(unsigned char *)*a1;
        if (!sub_338d5(v9)) break;
        *a1 = *a1 + 1;
      }
      if (((sub_337fb(v9)) || (v9 == 0x2d)) || (v9 == 0x2b)) break;
      if (sub_33753(v9)) {
        v16 = v7;
        do {
          if (v16 < v8) {
            v1 = &v16[1];
            *v16 = v9;
            v16 = v1;
          }
          *a1 = *a1 + 1;
          v9 = *(unsigned char *)*a1;
        } while ((sub_33753(v9)) || (v9 == 0x2e));
        *v16 = 0;
        v6 = sub_2ae65(a1,v7);
        if (!v6) {
          if (sub_27333(a1)) {
            sub_27244(gettext("error: unknown word \'%s\'\n"),v7);
            return 0x3f;
          }
          return 0x3f;
        }
        *(long *)a0 = (long)*(int *)(v6 + 0xc);
        return *(unsigned int *)(v6 + 8);
      }
      if (v9 != 0x28) {
        v4 = (char *)*a1;
        *a1 = (long)&v4[1];
        v3 = *v4;
        return (unsigned int)(unsigned char)sub_27231((int)v3);
      }
      v17 = 0;
      do {
        v4 = (char *)*a1;
        *a1 = (long)&v4[1];
        v3 = *v4;
        if (!v3)
          return 0;
        if (v3 != '(') { // branch-flip
          if (v3 == ')')
            v17 -= 1;
        }
        else {
          v17 += 1;
        }
      } while (v17);
    }
    v15 = (unsigned char *)*a1;
    if ((v9 != 0x2d) && (v9 != 0x2b)) {
      v10 = 0;
      break;
    }
    v10 = (v9 != 0x2d) ? 1 : -1; // branch-flip
    do {
      v15 = &v15[1];
      *a1 = (long)v15;
      v9 = *(unsigned char *)*a1;
    } while (sub_338d5(v9));
  } while (sub_337fb(v9) != '\x01');
  v13 = 0;
  v1 = v15;
  do {
    v15 = v1;
    v6 = v13 * 10;
    if (SEXT816(v6) != SEXT816(v13) * SEXT816(10))
      return 0x3f;
    v5 = (0 <= v10) ? (unsigned int)v9 - 0x30 : 0x30 - (unsigned int)v9; // branch-flip
    v13 = v5 + v6;
    if (SCARRY8((long)v5,v6))
      return 0x3f;
    v1 = &v15[1];
    v9 = *v1;
  } while (sub_337fb(v9));
  if (((v9 != 0x2e) && (v9 != 0x2c)) || (v2 = v15[2], !sub_337fb((int)(char)v2))) {
    *a0 = (unsigned char)((unsigned int)v10 >> 0x1f);
    *(long *)&a0[8] = v13;
    *(long *)&a0[0x10] = (long)v1 - *a1;
    *a1 = (long)v1;
    if (v10)
      return 0x112;
    return 0x113;
  }
  v1 = &v15[2];
  v15 = &v15[3];
  v12 = (char)*v1 + -0x30;
  for (v11 = 2; v11 <= 9; v11 = v11 + 1) {
    v12 *= 10;
    v2 = *v15;
    if (sub_337fb((int)(char)v2)) {
      v1 = &v15[1];
      v12 += (char)*v15 + -0x30;
      v15 = v1;
    }
  }
  if (v10 <= -1) {
    while (v2 = *v15, sub_337fb((int)(char)v2)) {
      if (*v15 != 0x30) {
        v12 += 1;
        break;
      }
      v15 = &v15[1];
    }
  }
  while (v2 = *v15, sub_337fb((int)(char)v2)) {
    v15 = &v15[1];
  }
  v14 = v13;
  if ((v10 <= -1) && (v12)) {
    v14 = v13 + -1;
    if (SBORROW8(v13,1))
      return 0x3f;
    v12 = 1000000000 - v12;
  }
  *(long *)a0 = v14;
  *(long *)&a0[8] = (long)v12;
  *a1 = (long)v15;
  if (v10)
    return 0x114;
  return 0x115;
}

// Function: sub_2b67b @ 0x2b67b
unsigned long sub_2b67b(void)
{
  return 0;
}

// Function: sub_2b692 @ 0x2b692
unsigned int sub_2b692(unsigned int *a0,unsigned int *a1) // early-return
{
  unsigned int v1; // eax
  
  if ((int)a1[6] <= -1)
    return 0;
  v1 = a1[5] ^ a0[5] | *a0 ^ *a1 | a1[1] ^ a0[1] | a1[2] ^ a0[2] | a1[3] ^ a0[3] | a1[4] ^ a0[4];
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_2b725 @ 0x2b725
char * sub_2b725(tm *a0,void *a1,char *a2,int a3)
{
  int v1; // eax
  char v2 [40];
  int v3; // stack - 0x40
  
  v1 = sub_34722(a2,(long)a3,"(Y-M-D) %Y-%m-%d %H:%M:%S",a0,0,0);
  if (((a1) && (v1 < a3)) && (*(long *)((long)a1 + 0xd0))) {
    v3 = *(int *)((long)a1 + 0x18);
    if (((*(long *)((long)a1 + 0xb8)) && (!*(long *)((long)a1 + 0xd0))) && (1 <= *(int *)((long)a1 + 0x14)))
      v3 += 0xe10;
    snprintf(&a2[v1],(long)(a3 - v1)," TZ=%s",(char *)sub_27a68(v3,v2));
  }
  return a2;
}

// Function: sub_2b83c @ 0x2b83c
char * sub_2b83c(void *a0,char *a1,int a2)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  char v4 [13];
  
  v1 = *(unsigned int *)((long)a0 + 0xc);
  v2 = *(int *)((long)a0 + 0x10);
  v3 = *(unsigned int *)((long)a0 + 0x14);
  snprintf(a1,(long)a2,"(Y-M-D) %s-%02d-%02d",sub_2abc6(v3,v4),(unsigned long)(v2 + 1),(unsigned long)v1);
  return a1;
}

// Function: sub_2b8d7 @ 0x2b8d7
char * sub_2b8d7(int *a0,char *a1,int a2)
{
  snprintf(a1,(long)a2,"%02d:%02d:%02d",a0[2],a0[1],*a0);
  return a1;
}

// Function: sub_2b92e @ 0x2b92e
void sub_2b92e(tm *a0,tm *a1,void *a2,bool a3) // return-dupe, ternary x6, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_2b92e
{
  int v1;
  char *v10; // rdi
  char *v11; // r8
  bool v12; // zf
  bool v13; // zf
  bool v14; // zf
  bool v15; // zf
  bool v16; // zf
  unsigned int v17; // stack - 0x8c
  int v2;
  bool v3; // al
  unsigned long v4; // rax
  char *v5; // rax
  char *v6; // rcx
  char *v7; // rdx
  char v8 [104];
  char *v9; // rax
  
  v1 = *(int *)a0;
  v2 = *(int *)a1;
  v12 = *(int *)&a0->field_0x4 != *(int *)&a1->field_0x4;
  v13 = *(int *)&a0->field_0x8 == *(int *)&a1->field_0x8;
  v14 = *(int *)&a0->field_0xc != *(int *)&a1->field_0xc;
  v15 = *(int *)&a0->field_0x10 != *(int *)&a1->field_0x10;
  v16 = *(int *)&a0->field_0x14 != *(int *)&a1->field_0x14;
  v3 = !(((((v1 != v2) || (v12)) || (v13)) || ((v14 || (v15)))) || (v16));
  if (sub_27333(a2) != '\x01')
    return;
  sub_27244(gettext("error: invalid date/time value:\n"));
  v4 = sub_2b725(a0,a2,v8,100);
  sub_27244(gettext("    user provided time: \'%s\'\n"),v4);
  v4 = sub_2b725(a1,a2,v8,100);
  sub_27244(gettext("       normalized time: \'%s\'\n"),v4);
  v9 = (v1 != v2) ? "--" : "";
  v6 = (v12) ? "--" : "";
  v7 = (v13) ? "" : "--";
  v11 = (v14) ? "--" : "";
  v10 = (v15) ? "--" : "";
  v5 = (v16) ? "----" : "";
  v17 = snprintf(v8,100,"                                 %4s %2s %2s %2s %2s %2s",v5,v10,v11,v7,v6,v9);
  if (0 <= (int)v17) {
    if (100 <= v17)
      v17 = 99;
    while ((1 <= (int)v17 && (v8[(int)(v17 - 1)] == ' '))) {
      v17 -= 1;
    }
    v8[(int)v17] = '\0';
  }
  sub_27244("%s\n",v8);
  sub_27244(gettext("     possible reasons:\n"));
  if (v3)
    sub_27244(gettext("       non-existing due to daylight-saving time;\n"));
  if ((v14) && (v15))
    sub_27244(gettext("       invalid day/month combination;\n"));
  sub_27244(gettext("       numeric values overflow;\n"));
  if (a3) // branch-flip
    v9 = gettext("incorrect timezone");
  else {
    v9 = gettext("missing timezone");
  }
  sub_27244("       %s\n",v9);
}

// Function: sub_2bd5c @ 0x2bd5c
char sub_2bd5c(struct_76 *a0,char *a1,long *a2,unsigned long a3,long *a4,char *a5) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_2bd5c
{
  char v1;
  long *v10; // rax
  long v100; // stack - 0x1e8
  char v101; // stack - 0x1e0
  char v102; // stack - 0x1df
  char v103; // stack - 0x1de
  char v104; // stack - 0x1dd
  char v105; // stack - 0x1dc
  char v106; // stack - 0x1db
  char v107; // stack - 0x1da
  char v108; // stack - 0x1d9
  char *v109; // stack - 0x1d8
  long v11;
  unsigned int v110; // stack - 0x1d0
  int v111; // stack - 0x1cc
  char *v112; // stack - 0x1c8
  unsigned int v113; // stack - 0x1c0
  int v114; // stack - 0x1bc
  unsigned long v115; // stack - 0x1b8
  char v116; // stack - 0x97
  char v117; // stack - 0x96
  long v12;
  long v13;
  unsigned long v14; // rax
  long v15 [2];
  char v16 [112];
  int v17; // stack - 0x378
  unsigned long v18 [4]; // stack - 0x2f8
  tm v19; // stack - 0x338
  char v2; // al
  char *v20; // stack - 0x2b8
  char v21 [32];
  int v22; // stack - 0x424
  char v23 [112];
  unsigned int v24; // stack - 0x438
  unsigned int v25; // stack - 0x3f8
  char v26 [101];
  char v27; // stack - 0x98
  char v28 [13];
  bool v29; // zf
  int v3;
  char *v30; // stack - 0x558
  long *v31; // stack - 0x540
  char *v32; // stack - 0x538
  char v33; // stack - 0x523
  char v34; // stack - 0x522
  int v35; // stack - 0x51c
  char *v36; // stack - 0x4e0
  unsigned long v37; // stack - 0x4d8
  long *v38; // stack - 0x4d0
  unsigned long v39; // stack - 0x4c8
  int v4; // eax
  char *v40; // stack - 0x4c0
  char *v41; // stack - 0x4b8
  char *v42; // stack - 0x4b0
  unsigned int v43; // stack - 0x434
  int v44; // stack - 0x430
  int v45; // stack - 0x42c
  int v46; // stack - 0x428
  int v47; // stack - 0x420
  int v48; // stack - 0x418
  long v49; // stack - 0x410
  int v5;
  unsigned int v50; // stack - 0x3f4
  int v51; // stack - 0x3f0
  int v52; // stack - 0x3ec
  int v53; // stack - 0x3e8
  int v54; // stack - 0x3e4
  int v55; // stack - 0x3d8
  long v56; // stack - 0x3b8
  long v57; // stack - 0x3b0
  long v58; // stack - 0x3a8
  long v59; // stack - 0x3a0
  unsigned long v6;
  long v60; // stack - 0x398
  long v61; // stack - 0x390
  unsigned long v62; // stack - 0x388
  int v63; // stack - 0x374
  int v64; // stack - 0x370
  int v65; // stack - 0x36c
  int v66; // stack - 0x368
  int v67; // stack - 0x364
  int v68; // stack - 0x358
  char *v69; // stack - 0x348
  char *v7; // rax
  int v70; // stack - 0x2d8
  unsigned long v71; // stack - 0x2d0
  char *v72; // stack - 0x2c8
  long v73; // stack - 0x2b0
  int v74; // stack - 0x2a8
  int v75; // stack - 0x2a4
  int v76; // stack - 0x2a0
  int v77; // stack - 0x29c
  long v78; // stack - 0x290
  unsigned long v79; // stack - 0x288
  undefined8 v8;
  long v80; // stack - 0x280
  long v81; // stack - 0x278
  unsigned long v82; // stack - 0x270
  long v83; // stack - 0x268
  long v84; // stack - 0x260
  long v85; // stack - 0x258
  long v86; // stack - 0x250
  long v87; // stack - 0x248
  long v88; // stack - 0x240
  long v89; // stack - 0x238
  char *v9;
  long v90; // stack - 0x230
  long v91; // stack - 0x228
  unsigned long v92; // stack - 0x220
  char v93; // stack - 0x218
  char v94; // stack - 0x217
  unsigned long v95; // stack - 0x210
  unsigned long v96; // stack - 0x208
  long v97; // stack - 0x200
  unsigned long v98; // stack - 0x1f8
  unsigned long v99; // stack - 0x1f0
  
  v33 = 0;
  v6 = strlen(a1);
  v36 = NULL;
  v31 = a2;
  if (!a2) {
    sub_34082(v15);
    v31 = v15;
  }
  v11 = *v31;
  v12 = v31[1];
  v32 = a1;
  while (v1 = *v32, sub_338d5((unsigned char)v1)) {
    v32 = &v32[1];
  }
  v56 = 0;
  v57 = 0;
  v58 = 0;
  v59 = 0;
  v60 = 0;
  v61 = 0;
  v62 = 0;
  v30 = a5;
  v38 = a4;
  if (!strncmp(v32,"TZ=\"",4)) {
    v39 = 1;
    v40 = &v32[4];
label_2c0a2:
    if (*v40) {
      if (*v40 != '\\') {
        if (*v40 != '\"') goto label_2c092;
        v41 = v16;
        if (0x65 <= (long)v39) {
          v36 = malloc(v39);
          if (!v36) goto label_2dc5a;
          v41 = v36;
        }
        v42 = v41;
        v40 = &v32[4];
        while (*v40 != '\"') {
          v1 = *v40;
          v9 = &v42[1];
          *v42 = v40[v1 == '\\'];
          v40 = &(&v40[v1 == '\\'])[1];
          v42 = v9;
        }
        *v42 = '\0';
        v10 = (long *)sub_30b5c(v41);
        if (!v10) goto label_2dc5a;
        v38 = v10;
        v30 = v41;
        do {
          v32 = &v40[1];
          v1 = *v32;
          v40 = v32;
        } while (sub_338d5((unsigned char)v1));
        goto label_2c0b4;
      }
      v40 = &v40[1];
      if ((*v40 == '\\') || (*v40 == '\"')) goto label_2c092;
    }
    goto label_2c0b4;
  }
label_2c0b4:
  if (!sub_30fdc(v38,v31,&v17)) goto label_2dc5a;
  if (!*v32)
    v32 = "0";
  v20 = v32;
  v78 = (long)v67 + 0x76c;
  v79 = 0;
  v80 = (long)(v66 + 1);
  v81 = (long)v65;
  v82 = (unsigned long)v64;
  v83 = (long)v63;
  v84 = (long)v17;
  v85 = (long)(int)v12;
  v48 = v68;
  v77 = 2;
  v86 = v56;
  v87 = v57;
  v88 = v58;
  v89 = v59;
  v90 = v60;
  v91 = v61;
  v92 = v62;
  v93 = '\0';
  v94 = '\0';
  v95 = 0;
  v96 = 0;
  v99 = 0;
  v97 = 0;
  v98 = 0;
  v100 = 0;
  v101 = 0;
  v102 = 0;
  v103 = 0;
  v105 = 0;
  v104 = 0;
  v106 = 0;
  v107 = 0;
  v108 = 0;
  v109 = v69;
  v110 = 0x10d;
  v111 = v68;
  v112 = NULL;
  v35 = 1;
  while( true ) {
    if ((4 <= v35) || (v19._0_8_ = v11 + v35 * 0x76a700, SCARRY8(v11,(long)(v35 * 0x76a700)))) goto label_2c3cc;
    if ((sub_30fdc(v38,&v19,v18)) && ((v72 && (v70 != v111)))) break;
    v35 += 1;
  }
  v112 = v72;
  v113 = 0x10d;
  v114 = v70;
  v115 = 0;
label_2c3cc:
  if (((v109) && (v112)) && (!strcmp(v109,v112))) {
    v111 = -1;
    v112 = NULL;
  }
  if (sub_28423(&v20)) {
    v1 = sub_27333(&v20);
    v9 = v20;
    if (v1) {
      if (&a1[v6] <= v20) // branch-flip
        v7 = gettext("error: parsing failed\n");
      else {
        v7 = gettext("error: parsing failed, stopped at \'%s\'\n");
      }
      sub_27244(v7,v9);
    }
    goto label_2dc5a;
  }
  if (sub_27333(&v20)) {
    sub_27244(gettext("input timezone: "));
    if (v93) // branch-flip
      fprintf(stderr,gettext("\'@timespec\' - always UTC"));
    else if (v100) // branch-flip
      fprintf(stderr,gettext("parsed date/time string"));
    else if (v30) { // branch-flip
      if (v38 != a4) // branch-flip
        fprintf(stderr,gettext("TZ=\"%s\" in date string"),v30);
      else if (strcmp(v30,"UTC0")) // branch-flip
        fprintf(stderr,gettext("TZ=\"%s\" environment value"),v30);
      else {
        fprintf(stderr,gettext("TZ=\"UTC0\" environment value or -u"));
      }
    }
    else {
      fprintf(stderr,gettext("system default"));
    }
    if (((v97) && (!v100)) && (1 <= v75))
      fprintf(stderr,", dst");
    if (v100) {
      v8 = sub_27a68(v76,v21);
      fprintf(stderr," (%s)",v8);
    }
    fputc(10,stderr);
  }
  if (v93) { // branch-flip
    a0->field_0x0 = v84;
    a0->field_0x8 = v85;
  }
  else {
    if (2 <= (long)(v100 + v97 | v99 | v95 | v96 | v98)) {
      if (sub_27333(&v20)) {
        if (2 <= (long)v99)
          sub_27244("error: seen multiple time parts\n");
        if (2 <= (long)v95)
          sub_27244("error: seen multiple date parts\n");
        if (2 <= (long)v96)
          sub_27244("error: seen multiple days parts\n");
        if (2 <= (long)v98)
          sub_27244("error: seen multiple daylight-saving parts\n");
        if (2 <= v100 + v97)
          sub_27244("error: seen multiple time-zone parts\n");
      }
      goto label_2dc5a;
    }
    v2 = sub_27333(&v20);
    if (((sub_2ac65((unsigned char)v2,&v22) != '\x01') || (v46 = (int)(v80 + -1), v80 + -1 != (long)v46 || SCARRY8(v80,-1))) || (v45 = (int)v81, v81 != v45)) {
      if (sub_27333(&v20))
        sub_27244(gettext("error: year, month, or day overflow\n"));
      goto label_2dc5a;
    }
    if ((v99) || (((v94 && (!v95)) && (!v96)))) {
      v44 = sub_2ab3b(v82,v77);
      if (v44 <= -1) {
        if (v77) { // branch-flip
          v9 = (v77 != 1) ? "" : "pm"; // branch-flip
        }
        else {
          v9 = "am";
        }
        v1 = sub_27333(&v20);
        v6 = v82;
        if (v1)
          sub_27244(gettext("error: invalid hour %ld%s\n"),v6,v9);
        goto label_2dc5a;
      }
      v43 = (unsigned int)v83;
      v24 = (unsigned int)v84;
      if (sub_27333(&v20)) {
        v8 = sub_2b8d7(&v24,v23,100);
        if (v99) // branch-flip
          v9 = gettext("using specified time as starting value: \'%s\'\n");
        else {
          v9 = gettext("using current time as starting value: \'%s\'\n");
        }
        sub_27244(v9,v8);
      }
    }
    else {
      v24 = 0;
      v43 = 0;
      v44 = 0;
      v85 = 0;
      if (sub_27333(&v20))
        sub_27244("warning: using midnight as starting time: 00:00:00\n");
    }
    if (v99 || (v95 || v96))
      v48 = -1;
    if (v97)
      v48 = v75;
    v25 = v24;
    v50 = v43;
    v51 = v44;
    v52 = v45;
    v53 = v46;
    v54 = v22;
    v55 = v48;
    v47 = -1;
    v37 = sub_3108c(v38,&v24);
    if (sub_2b692(&v25,&v24) != '\x01') {
      v34 = '\0';
      v29 = v100 != 0;
      if (v29) {
        v117 = 0x58;
        v116 = 0x58;
        v27 = 0x58;
        sub_27a68(v76,v26);
        v10 = (long *)sub_30b5c(&v27);
        if (!v10) {
          if (sub_27333(&v20))
            sub_27244(gettext("error: tzalloc (\"%s\") failed\n"),&v27);
          goto label_2dc5a;
        }
        v24 = v25;
        v43 = v50;
        v44 = v51;
        v45 = v52;
        v46 = v53;
        v22 = v54;
        v48 = v55;
        v47 = -1;
        v37 = sub_3108c(v10,&v24);
        v34 = sub_2b692(&v25,&v24);
        sub_30db5(v10);
      }
      if (v34 != '\x01') {
        sub_2b92e(&v25,&v24,&v20,v29);
        goto label_2dc5a;
      }
    }
    if ((v96) && (!v95)) {
      v5 = ((1 <= v73) && (v47 != v74));
      v18[0] = (v73 - v5) * 7;
      if (((SEXT816((long)v18[0]) != SEXT816(v73 - v5) * SEXT816(7)) || (v11 = (long)(((v74 - v47) + 7) % 7), v29 = SCARRY8(v11,v18[0]), v18[0] = v11 + v18[0], v29)) || (v11 = (long)v45, v12 = v11 + v18[0], v45 = (int)v12, v12 != v45 || SCARRY8(v11,v18[0])))
        v37 = 0xffffffffffffffff;
      else {
        v48 = -1;
        v37 = sub_3108c(v38,&v24);
      }
      if (v37 == 0xffffffffffffffff) {
        if (sub_27333(&v20)) {
          v8 = sub_2b725(&v24,&v20,v23,100);
          v14 = sub_2791e(&v20,&v27,100);
          sub_27244(gettext("error: day \'%s\' (day ordinal=%ld number=%d) resulted in an invalid date: \'%s\'\n"),v14,v73,v74,v8);
        }
        goto label_2dc5a;
      }
      if (sub_27333(&v20)) {
        v8 = sub_2b725(&v24,&v20,v23,100);
        v14 = sub_2791e(&v20,&v27,100);
        sub_27244(gettext("new start date: \'%s\' is \'%s\'\n"),v14,v8);
      }
    }
    if (sub_27333(&v20)) {
      if ((!v95) && (!v96)) {
        v8 = sub_2b83c(&v24,v23,100);
        sub_27244(gettext("using current date as starting value: \'%s\'\n"),v8);
      }
      if ((v96) && (v95)) {
        v8 = sub_2791e(&v20,&v27,100);
        sub_27244(gettext("warning: day (%s) ignored when explicit dates are given\n"),v8);
      }
      v8 = sub_2b725(&v24,&v20,v23,100);
      sub_27244(gettext("starting date/time: \'%s\'\n"),v8);
    }
    if (v88 || (v86 || v87)) {
      if (sub_27333(&v20)) {
        if (((v86) || (v87)) && (v45 != 0xf))
          sub_27244(gettext("warning: when adding relative months/years, it is recommended to specify the 15th of the months\n"));
        if ((v88) && (v44 != 0xc))
          sub_27244(gettext("warning: when adding relative days, it is recommended to specify noon\n"));
      }
      v11 = v22 + v86;
      v5 = (int)v11;
      if (v11 == v5 && !SCARRY8((long)v22,v86)) {
        v11 = v46 + v87;
        v3 = (int)v11;
        v19._0_4_ = v3;
        if (v11 == v3 && !SCARRY8((long)v46,v87)) {
          v11 = v45 + v88;
          v4 = (int)v11;
          v18[0] = CONCAT44(v18[0]._4_4_,v4);
          if (v11 == v4 && !SCARRY8((long)v45,v88)) {
            v44 = v51;
            v43 = v50;
            v24 = v25;
            v48 = v55;
            v45 = v4;
            v46 = v3;
            v22 = v5;
            v37 = sub_3108c(v38,&v24);
            if (v37 == 0xffffffffffffffff) {
              if (sub_27333(&v20)) {
                v8 = sub_2b725(&v24,&v20,v23,100);
                sub_27244(gettext("error: adding relative date resulted in an invalid date: \'%s\'\n"),v8);
              }
              goto label_2dc5a;
            }
            v1 = sub_27333(&v20);
            v13 = v88;
            v12 = v87;
            v11 = v86;
            if (v1) {
              sub_27244(gettext("after date adjustment (%+ld years, %+ld months, %+ld days),\n"),v11,v12,v13);
              v8 = sub_2b725(&v24,&v20,v23,100);
              sub_27244(gettext("    new date/time = \'%s\'\n"),v8);
              if ((v55 != -1) && (v48 != v55))
                sub_27244(gettext("warning: daylight saving time changed after date adjustment\n"));
              if ((!v88) && ((v45 != (int)v18[0] || ((!v87 && (v46 != v19._0_4_)))))) {
                sub_27244(gettext("warning: month/year adjustment resulted in shifted dates:\n"));
                v6 = v18[0];
                v3 = v19._0_4_ + 1;
                v8 = sub_2abc6(v5,v28);
                sub_27244(gettext("     adjusted Y M D: %s %02d %02d\n"),v8,v3,v6 & 0xffffffff);
                v3 = v45;
                v5 = v46 + 1;
                v8 = sub_2abc6(v22,v28);
                sub_27244(gettext("   normalized Y M D: %s %02d %02d\n"),v8,v5,v3);
              }
            }
            goto label_2d50c;
          }
        }
      }
      if (sub_27333(&v20))
        sub_27244(gettext("error: %s:%d\n"),"parse-datetime.y",0x865);
      goto label_2dc5a;
    }
label_2d50c:
    if (v100) {
      v19._0_8_ = v76 - v49;
      v29 = SBORROW8(v37,v19._0_8_);
      v37 -= v19._0_8_;
      v18[0] = v37;
      if (v29 || SBORROW8((long)v76,v49)) {
        if (sub_27333(&v20))
          sub_27244(gettext("error: timezone %d caused time_t overflow\n"),v76);
        goto label_2dc5a;
      }
    }
    if (sub_27333(&v20)) {
      v8 = sub_2b725(&v24,&v20,v23,100);
      sub_27244(gettext("\'%s\' = %ld epoch-seconds\n"),v8,v37);
    }
    v11 = (v85 + (int)v92) % 1000000000 + 1000000000;
    v5 = (int)v11 + (int)(v11 / 1000000000) * -1000000000;
    v11 = v89 * 0xe10;
    if (((((SEXT816(v11) != SEXT816(v89) * SEXT416(0xe10)) || (SCARRY8(v37,v11))) || (v12 = v90 * 0x3c, SEXT816(v12) != SEXT816(v90) * SEXT816(0x3c))) || ((v13 = v12 + v37 + v11, SCARRY8(v12,v37 + v11) || (v11 = v91 + v13, SCARRY8(v91,v13))))) || (v12 = (long)(int)(((v85 + (int)v92) - (long)v5) / 1000000000), v19._0_8_ = v12 + v11, SCARRY8(v12,v11))) {
      if (sub_27333(&v20))
        sub_27244(gettext("error: adding relative time caused an overflow\n"));
      goto label_2dc5a;
    }
    a0->field_0x0 = v19._0_8_;
    a0->field_0x8 = (long)v5;
    v1 = sub_27333(&v20);
    v6 = v92;
    v13 = v91;
    v12 = v90;
    v11 = v89;
    if ((v1) && ((int)v92 || ((v89 || v90) || v91))) {
      sub_27244(gettext("after time adjustment (%+ld hours, %+ld minutes, %+ld seconds, %+d ns),\n"),v11,v12,v13,v6 & 0xffffffff);
      v8 = v19._0_8_;
      sub_27244(gettext("    new time = %ld epoch-seconds\n"),v8);
      if ((v48 != -1) && ((sub_30fdc(v38,a0,v18) && (v48 != v70))))
        sub_27244(gettext("warning: daylight saving time changed after time adjustment\n"));
    }
  }
  if (sub_27333(&v20)) {
    if (v30) { // branch-flip
      if (strcmp(v30,"UTC0")) // branch-flip
        sub_27244(gettext("timezone: TZ=\"%s\" environment value\n"),v30);
      else {
        sub_27244(gettext("timezone: Universal Time\n"));
      }
    }
    else {
      sub_27244(gettext("timezone: system default\n"));
    }
    v11 = a0->field_0x0;
    v6 = a0->field_0x8;
    sub_27244(gettext("final: %ld.%09d (epoch-seconds)\n"),v11,v6 & 0xffffffff);
    if (gmtime_r(a0,&v19)) {
      v8 = sub_2b725(&v19,NULL,v23,100);
      sub_27244(gettext("final: %s (UTC)\n"),v8);
    }
    if (sub_30fdc(v38,a0,v18)) {
      v8 = sub_27a68(v71 & 0xffffffff,v21);
      v14 = sub_2b725(v18,NULL,v23,100);
      sub_27244(gettext("final: %s (UTC%s)\n"),v14,v8);
    }
  }
  v33 = 1;
label_2dc5a:
  if (v38 != a4)
    sub_30db5(v38);
  free(v36);
  return v33;
label_2c092:
  v40 = &v40[1];
  v39 += 1;
  goto label_2c0a2;
}

// Function: sub_2dcb0 @ 0x2dcb0
char sub_2dcb0(struct_76 *a0,char *a1,long *a2) // early-return
{
  char v1; // al
  char *v2; // rax
  long *v3; // rax
  
  v2 = getenv("TZ");
  v3 = (long *)sub_30b5c(v2);
  if (!v3)
    return 0;
  v1 = sub_2bd5c(a0,a1,a2,0,v3,v2);
  sub_30db5(v3);
  return v1;
}

// Function: sub_2dd35 @ 0x2dd35
void sub_2dd35(char *a0)
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
        program_invocation_short_name = &v1[3];
        v2 = program_invocation_short_name;
      }
    }
  }
  dat_4a230 = v2;
  __progname_full = v2;
}

// Function: sub_2de20 @ 0x2de20
unsigned long sub_2de20(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x4a240;
  v2 = sub_32102(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_2de6f @ 0x2de6f
unsigned int sub_2de6f(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x4a240;
  return *a0;
}

// Function: sub_2de93 @ 0x2de93
void sub_2de93(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x4a240;
  *a0 = a1;
}

// Function: sub_2debe @ 0x2debe
unsigned int sub_2debe(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x4a240;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_2df4e @ 0x2df4e
unsigned int sub_2df4e(void *a0,unsigned int a1)
{
  unsigned int v1;
  void *v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = (void *)0x4a240;
  v1 = *(unsigned int *)((long)v2 + 4);
  *(unsigned int *)((long)v2 + 4) = a1;
  return v1;
}

// Function: sub_2df88 @ 0x2df88
void sub_2df88(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x4a240;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_2dfea @ 0x2dfea
unsigned long * sub_2dfea(unsigned long *a0,unsigned int a1)
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

// Function: sub_2e0af @ 0x2e0af
char * sub_2e0af(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_3427c();
  if (!sub_339b4(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_339b4(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x3e2be;
  return (char *)0x3e2c2;
}

// Function: sub_2e174 @ 0x2e174
unsigned long sub_2e174(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_2e22e:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_2e382:
      v14 = 1;
      goto label_2e386;
    case 2:
label_2e395:
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
      goto label_2e382;
    case 4:
label_2e386:
      if (!v14)
        v2 = 1;
      goto label_2e395;
    case 5:
label_2e270:
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
      goto label_2e270;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_2e0af("`",v19);
        v17 = (char *)sub_2e0af("\'",v19);
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
label_2f034:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_2f06c;
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
      if (v14) goto label_2f1b0;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_2e953:
    if (!v24) {
label_2e95e:
      v16 = 1;
label_2e962:
      if (v19 != 2) goto label_2ee79;
label_2e73b:
      if (v14) goto label_2f1b0;
    }
    goto label_2ee79;
  }
  if (0x7e < v22) {
label_2ea3f:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_2eab9:
      v31 = sub_2419c(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_2eb67;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_2ec16;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_2ec16:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_2ee79;
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
        if (v14) goto label_2f1b0;
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
    goto label_2ef9f;
  }
  if (v22 == 0x7d) {
label_2e921:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_2ee79;
    goto label_2e953;
  }
  if (0x7d < v22) goto label_2ea3f;
  if (v22 == 0x7c) goto label_2e962;
  if (0x7c < v22) goto label_2ea3f;
  if (v22 == 0x7b) goto label_2e921;
  if (0x7b < v22) goto label_2ea3f;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_2ea3f;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_2ea36;
    if (v11 & 0xa4000000) goto label_2e962;
    if (!(v11 & 0x8000000)) goto label_2ea3f;
    if (v19 == 2) {
      if (!v14) goto label_2ef9f;
      goto label_2f1b0;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_2e8f7;
    goto label_2ef9f;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_2ee79;
        goto label_2f02f;
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
        goto label_2ee79;
      }
      goto label_2f1b0;
    default:
      goto label_2ea3f;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_2e8f7;
    case 10:
      v23 = 0x6e;
      goto label_2e8f7;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_2e8f7:
      if ((v19 == 2) && (v14)) goto label_2f1b0;
      break;
    case 0x20:
      goto label_2e95e;
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
      goto label_2e962;
    case 0x23:
      goto label_2e953;
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
label_2ea36:
      v16 = 1;
      goto label_2ee79;
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
          goto label_2ee79;
        }
        goto label_2f1b0;
      }
      goto label_2ee79;
    case 0x3f:
      if (v19 == 2) goto label_2e73b;
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
          goto label_2ee79;
        }
        goto label_2f1b0;
      }
      goto label_2ee79;
    
  }
  if (v2) {
    v22 = v23;
label_2eee4:
    if (v14) goto label_2f1b0;
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
label_2ee79:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_2eee4;
  }
label_2ef9f:
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
label_2f02f:
  v24 += 1;
  goto label_2f034;
label_2eb67:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_2f1b0;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_2ec16;
  goto label_2eab9;
label_2f06c:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_2f1b0:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_2e174(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_2e174(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_2e22e;
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

// Function: sub_2f235 @ 0x2f235
unsigned long sub_2f235(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x4a240;
  v1 = *__errno_location();
  v2 = sub_2e174(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_2f2dd @ 0x2f2dd
void sub_2f2dd(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_2f313(a0,a1,NULL,a2);
}

// Function: sub_2f313 @ 0x2f313
long sub_2f313(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x4a240;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_2e174(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_31c0e(v3 + 1U);
  sub_2e174(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_2f43f @ 0x2f43f
void sub_2f43f(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_498b0;
  for (v2 = 1; v2 < dat_49840; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x4a280) {
    free(*(void **)((long)v1 + 8));
    dat_498a0 = 0x100;
    dat_498a8 = 0x4a280;
  }
  if (v1 == (void *)0x498a0) {
    dat_49840 = 1;
    return;
  }
  free(v1);
  dat_49840 = 1;
  dat_498b0 = (void *)0x498a0;
}

// Function: sub_2f4fe @ 0x2f4fe
void * sub_2f4fe(int a0,char *a1,unsigned long a2,struct_0 *a3)
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
  v6 = dat_498b0;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_49840 <= a0) {
      v5 = dat_498b0 == (unsigned long *)0x498a0;
      v4 = (long)dat_49840;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_498b0;
      }
      dat_498b0 = (unsigned long *)sub_31e9e(v3,&v4,(long)((a0 - dat_49840) + 1),0x7fffffff,0x10);
      v6 = dat_498b0;
      if (v5) {
        *dat_498b0 = dat_498a0;
        dat_498b0[1] = dat_498a8;
      }
      memset(&dat_498b0[(long)dat_49840 * 2],0,(v4 - dat_49840) * 0x10);
      dat_49840 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_2e174(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x4a280)
        free(v7);
      v7 = (void *)sub_31c0e(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_2e174(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_2f7ad @ 0x2f7ad
void sub_2f7ad(unsigned int a0,char *a1)
{
  sub_2f4fe(a0,a1,0xffffffffffffffff,(struct_0 *)0x4a240);
}

// Function: sub_2f7e1 @ 0x2f7e1
void sub_2f7e1(unsigned int a0,char *a1,unsigned long a2)
{
  sub_2f4fe(a0,a1,a2,(struct_0 *)0x4a240);
}

// Function: sub_2f813 @ 0x2f813
void sub_2f813(char *a0)
{
  sub_2f7ad(0,a0);
}

// Function: sub_2f836 @ 0x2f836
void sub_2f836(char *a0,unsigned long a1)
{
  sub_2f7e1(0,a0,a1);
}

// Function: sub_2f861 @ 0x2f861
void sub_2f861(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_2dfea(v1,a1);
  sub_2f4fe(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_2f8c9 @ 0x2f8c9
void sub_2f8c9(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_2dfea(v1,a1);
  sub_2f4fe(a0,a2,a3,v1);
}

// Function: sub_2f92f @ 0x2f92f
void sub_2f92f(unsigned int a0,char *a1)
{
  sub_2f861(0,a0,a1);
}

// Function: sub_2f957 @ 0x2f957
void sub_2f957(unsigned int a0,char *a1,unsigned long a2)
{
  sub_2f8c9(0,a0,a1,a2);
}

// Function: sub_2f987 @ 0x2f987
void sub_2f987(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_4a240;
  v2 = dat_4a248;
  v3 = dat_4a250;
  v4 = dat_4a258;
  v5 = dat_4a260;
  v6 = dat_4a268;
  v7 = dat_4a270;
  sub_2debe(&v1,(int)a2,1);
  sub_2f4fe(0,a0,a1,&v1);
}

// Function: sub_2fa42 @ 0x2fa42
void sub_2fa42(char *a0,char a1)
{
  sub_2f987(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_2fa70 @ 0x2fa70
void sub_2fa70(char *a0)
{
  sub_2fa42(a0,0x3a);
}

// Function: sub_2fa93 @ 0x2fa93
void sub_2fa93(char *a0,unsigned long a1)
{
  sub_2f987(a0,a1,0x3a);
}

// Function: sub_2fac1 @ 0x2fac1
void sub_2fac1(unsigned int a0,unsigned int a1,char *a2)
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
  sub_2dfea(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_2debe(&v2,0x3a,1);
  sub_2f4fe(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_2fb83 @ 0x2fb83
void sub_2fb83(unsigned int a0,long a1,long a2,char *a3)
{
  sub_2fbbd(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_2fbbd @ 0x2fbbd
void sub_2fbbd(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_4a240;
  v2 = dat_4a248;
  v3 = dat_4a250;
  v4 = dat_4a258;
  v5 = dat_4a260;
  v6 = dat_4a268;
  v7 = dat_4a270;
  sub_2df88(&v1,a1,a2);
  sub_2f4fe(a0,a3,a4,&v1);
}

// Function: sub_2fc7b @ 0x2fc7b
void sub_2fc7b(long a0,long a1,char *a2)
{
  sub_2fb83(0,a0,a1,a2);
}

// Function: sub_2fcae @ 0x2fcae
void sub_2fcae(long a0,long a1,char *a2,unsigned long a3)
{
  sub_2fbbd(0,a0,a1,a2,a3);
}

// Function: sub_2fcec @ 0x2fcec
void sub_2fcec(unsigned int a0,char *a1,unsigned long a2)
{
  sub_2f4fe(a0,a1,a2,(struct_0 *)0x49860);
}

// Function: sub_2fd1e @ 0x2fd1e
void sub_2fd1e(char *a0,unsigned long a1)
{
  sub_2fcec(0,a0,a1);
}

// Function: sub_2fd49 @ 0x2fd49
void sub_2fd49(unsigned int a0,char *a1)
{
  sub_2fcec(a0,a1,0xffffffffffffffff);
}

// Function: sub_2fd76 @ 0x2fd76
void sub_2fd76(char *a0)
{
  sub_2fd49(0,a0);
}

// Function: sub_2fd99 @ 0x2fd99
unsigned long sub_2fd99(struct_20 *a0) // return-dupe x2
{
  a0->field_0x8 = 0;
  a0->field_0x0 = sub_1e034(".",0x80000);
  if (0 <= a0->field_0x0)
    return 0;
  a0->field_0x8 = getcwd(NULL,0);
  if (a0->field_0x8)
    return 0;
  return 0xffffffff;
}

// Function: sub_2fe17 @ 0x2fe17
void sub_2fe17(struct_21 *a0) // return-dupe
{
  if (0 <= a0->field_0x0) {
    fchdir(a0->field_0x0);
    return;
  }
  sub_33b60(a0->field_0x8);
}

// Function: sub_2fe52 @ 0x2fe52
void sub_2fe52(struct_18 *a0)
{
  if (0 <= a0->field_0x0)
    close(a0->field_0x0);
  free(a0->field_0x8);
}

// Function: sub_2fe8c @ 0x2fe8c
unsigned long sub_2fe8c(int a0,char *a1,char **a2) // early-return x2
{
  unsigned int v1; // eax
  int v2;
  char *v3; // rax
  char v4 [4040];
  int v5 [4]; // stack - 0xfe8
  
  if ((a0 == -100) || (*a1 == '/'))
    return sub_30943(a1,a2);
  v3 = (char *)sub_33321(v4,a0,a1);
  if (v3) {
    v1 = sub_30943(v3,a2);
    v2 = *__errno_location();
    if (v3 != v4)
      free(v3);
    if (v1 != 0xffffffff)
      return (unsigned long)v1;
    if (((((v2 != 0x14) && (v2 != 2)) && (v2 != 1)) && ((v2 != 0xd && (v2 != 0x26)))) && (v2 != 0x5f)) {
      *__errno_location() = v2;
      return 0xffffffff;
    }
  }
  if (sub_2fd99(v5))
    sub_26fd4(*__errno_location()); // no-return
  if ((0 <= a0) && (a0 == v5[0])) {
    sub_2fe52(v5);
    *__errno_location() = 9;
    return 0xffffffff;
  }
  if (fchdir(a0)) {
    v2 = *__errno_location();
    sub_2fe52(v5);
    *__errno_location() = v2;
    return 0xffffffff;
  }
  v1 = sub_30943(a1,a2);
  if (v1 != 0xffffffff) // branch-flip
    v2 = 0;
  else {
    v2 = *__errno_location();
  }
  if (!sub_2fe17(v5)) {
    sub_2fe52(v5);
    if (v2)
      *__errno_location() = v2;
    return (unsigned long)v1;
  }
  sub_27011(*__errno_location()); // no-return
}

// Function: sub_3011a @ 0x3011a
unsigned long sub_3011a(int a0,char *a1,char **a2) // early-return x2
{
  unsigned int v1; // eax
  int v2;
  char *v3; // rax
  char v4 [4040];
  int v5 [4]; // stack - 0xfe8
  
  if ((a0 == -100) || (*a1 == '/'))
    return sub_30980(a1,a2);
  v3 = (char *)sub_33321(v4,a0,a1);
  if (v3) {
    v1 = sub_30980(v3,a2);
    v2 = *__errno_location();
    if (v3 != v4)
      free(v3);
    if (v1 != 0xffffffff)
      return (unsigned long)v1;
    if (((((v2 != 0x14) && (v2 != 2)) && (v2 != 1)) && ((v2 != 0xd && (v2 != 0x26)))) && (v2 != 0x5f)) {
      *__errno_location() = v2;
      return 0xffffffff;
    }
  }
  if (sub_2fd99(v5))
    sub_26fd4(*__errno_location()); // no-return
  if ((0 <= a0) && (a0 == v5[0])) {
    sub_2fe52(v5);
    *__errno_location() = 9;
    return 0xffffffff;
  }
  if (fchdir(a0)) {
    v2 = *__errno_location();
    sub_2fe52(v5);
    *__errno_location() = v2;
    return 0xffffffff;
  }
  v1 = sub_30980(a1,a2);
  if (v1 != 0xffffffff) // branch-flip
    v2 = 0;
  else {
    v2 = *__errno_location();
  }
  if (!sub_2fe17(v5)) {
    sub_2fe52(v5);
    if (v2)
      *__errno_location() = v2;
    return (unsigned long)v1;
  }
  sub_27011(*__errno_location()); // no-return
}

// Function: sub_303a8 @ 0x303a8
unsigned long sub_303a8(int a0,char *a1,char *a2) // early-return x2
{
  unsigned int v1; // eax
  int v2;
  char *v3; // rax
  char v4 [4040];
  int v5 [4]; // stack - 0xfe8
  
  if ((a0 == -100) || (*a1 == '/'))
    return CONCAT44(dat_4,setfilecon(a1,a2));
  v3 = (char *)sub_33321(v4,a0,a1);
  if (v3) {
    v1 = setfilecon(v3,a2);
    v2 = *__errno_location();
    if (v3 != v4)
      free(v3);
    if (v1 != 0xffffffff)
      return (unsigned long)v1;
    if (((((v2 != 0x14) && (v2 != 2)) && (v2 != 1)) && ((v2 != 0xd && (v2 != 0x26)))) && (v2 != 0x5f)) {
      *__errno_location() = v2;
      return 0xffffffff;
    }
  }
  if (sub_2fd99(v5))
    sub_26fd4(*__errno_location()); // no-return
  if ((0 <= a0) && (a0 == v5[0])) {
    sub_2fe52(v5);
    *__errno_location() = 9;
    return 0xffffffff;
  }
  if (fchdir(a0)) {
    v2 = *__errno_location();
    sub_2fe52(v5);
    *__errno_location() = v2;
    return 0xffffffff;
  }
  v1 = setfilecon(a1,a2);
  if (v1 != 0xffffffff) // branch-flip
    v2 = 0;
  else {
    v2 = *__errno_location();
  }
  if (!sub_2fe17(v5)) {
    sub_2fe52(v5);
    if (v2)
      *__errno_location() = v2;
    return (unsigned long)v1;
  }
  sub_27011(*__errno_location()); // no-return
}

// Function: sub_30636 @ 0x30636
unsigned long sub_30636(int a0,char *a1,char *a2) // early-return x2
{
  unsigned int v1; // eax
  int v2;
  char *v3; // rax
  char v4 [4040];
  int v5 [4]; // stack - 0xfe8
  
  if ((a0 == -100) || (*a1 == '/'))
    return CONCAT44(dat_4,lsetfilecon(a1,a2));
  v3 = (char *)sub_33321(v4,a0,a1);
  if (v3) {
    v1 = lsetfilecon(v3,a2);
    v2 = *__errno_location();
    if (v3 != v4)
      free(v3);
    if (v1 != 0xffffffff)
      return (unsigned long)v1;
    if (((((v2 != 0x14) && (v2 != 2)) && (v2 != 1)) && ((v2 != 0xd && (v2 != 0x26)))) && (v2 != 0x5f)) {
      *__errno_location() = v2;
      return 0xffffffff;
    }
  }
  if (sub_2fd99(v5))
    sub_26fd4(*__errno_location()); // no-return
  if ((0 <= a0) && (a0 == v5[0])) {
    sub_2fe52(v5);
    *__errno_location() = 9;
    return 0xffffffff;
  }
  if (fchdir(a0)) {
    v2 = *__errno_location();
    sub_2fe52(v5);
    *__errno_location() = v2;
    return 0xffffffff;
  }
  v1 = lsetfilecon(a1,a2);
  if (v1 != 0xffffffff) // branch-flip
    v2 = 0;
  else {
    v2 = *__errno_location();
  }
  if (!sub_2fe17(v5)) {
    sub_2fe52(v5);
    if (v2)
      *__errno_location() = v2;
    return (unsigned long)v1;
  }
  sub_27011(*__errno_location()); // no-return
}

// Function: sub_308c4 @ 0x308c4
int sub_308c4(int a0,char **a1) // early-return x2
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

// Function: sub_30943 @ 0x30943
void sub_30943(char *a0,char **a1)
{
  sub_308c4(getfilecon(a0,a1),a1);
}

// Function: sub_30980 @ 0x30980
void sub_30980(char *a0,char **a1)
{
  sub_308c4(lgetfilecon(a0,a1),a1);
}

// Function: sub_309bd @ 0x309bd
void sub_309bd(int a0,char **a1)
{
  sub_308c4(fgetfilecon(a0,a1),a1);
}

// Function: sub_309f7 @ 0x309f7
unsigned long sub_309f7(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x50);
}

// Function: sub_30a0d @ 0x30a0d
unsigned long sub_30a0d(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x70);
}

// Function: sub_30a23 @ 0x30a23
unsigned long sub_30a23(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x60);
}

// Function: sub_30a39 @ 0x30a39
unsigned long sub_30a39(void)
{
  return 0;
}

// Function: sub_30a4c @ 0x30a4c
undefined16 sub_30a4c(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x48);
}

// Function: sub_30a66 @ 0x30a66
undefined16 sub_30a66(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x68);
}

// Function: sub_30a80 @ 0x30a80
undefined16 sub_30a80(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x58);
}

// Function: sub_30a9a @ 0x30a9a
undefined16 sub_30a9a(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_30ac0 @ 0x30ac0
unsigned int sub_30ac0(unsigned int a0)
{
  return a0;
}

// Function: sub_30ad4 @ 0x30ad4
unsigned long sub_30ad4(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_30b1c @ 0x30b1c
void sub_30b1c(void *a0,void *a1,unsigned long a2)
{
  memcpy(a0,a1,a2);
  *(char *)(a2 + (long)a0) = 0;
}

// Function: sub_30b5c @ 0x30b5c
unsigned long * sub_30b5c(char *a0) // ternary
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
      sub_30b1c((void *)((long)v3 + 9),a0,v1);
  }
  return v3;
}

// Function: sub_30c11 @ 0x30c11
unsigned long sub_30c11(long *a0,struct_75 *a1) // return-dupe
{
  struct_75 *v1;
  long v2;
  long v3; // rax
  long *v4; // stack - 0x30
  char *v5; // stack - 0x20
  
  v5 = "";
  v1 = (struct_75 *)a1->field_0x30;
  if ((!v1) || ((a1 <= v1 && (v1 < &a1[1]))))
    return 1;
  if (v1->field_0x0[0]) {
    v5 = (char *)((long)a0 + 9);
    v4 = a0;
    while (strcmp(v5,(char *)v1)) {
      if ((!*v5) && ((v5 != (char *)((long)v4 + 9) || (!(char)v4[1])))) {
        v3 = strlen((char *)v1) + 1;
        if ((long)v4 + (0x80U - (long)v5) <= v3) { // branch-flip
          *v4 = sub_30b5c(v1);
          v2 = *v4;
          if (!v2)
            return 0;
          *(char *)(v2 + 8) = 0;
          v5 = (char *)(v2 + 9);
        }
        else {
          sub_30b1c(v5,v1,v3);
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

// Function: sub_30db5 @ 0x30db5
void sub_30db5(unsigned long *a0) // return-dupe
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

// Function: sub_30dfb @ 0x30dfb
void sub_30dfb(void)
{
  getenv("TZ");
}

// Function: sub_30e14 @ 0x30e14
void sub_30e14(char *a0) // return-dupe
{
  if (a0) {
    setenv("TZ",a0,1);
    return;
  }
  unsetenv("TZ");
}

// Function: sub_30e59 @ 0x30e59
bool sub_30e59(void *a0) // ternary
{
  char *v1; // rax
  bool v2; // zf
  
  v1 = (*(char *)((long)a0 + 8)) ? (char *)((long)a0 + 9) : NULL; // branch-flip
  v2 = sub_30e14(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_30ea3 @ 0x30ea3
unsigned long * sub_30ea3(void *a0) // early-return x2
{
  int v1;
  char *v2; // rax
  unsigned long *v3; // rax
  bool v4;
  
  v2 = (char *)sub_30dfb();
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
  v3 = (unsigned long *)sub_30b5c(v2);
  if ((v3) && (sub_30e59(a0) != '\x01')) {
    v1 = *__errno_location();
    sub_30db5(v3);
    *__errno_location() = v1;
    return NULL;
  }
  return v3;
}

// Function: sub_30f70 @ 0x30f70
char sub_30f70(unsigned long *a0) // early-return
{
  char v1; // al
  int v2; // stack - 0xc
  
  if (a0 == (unsigned long *)0x1)
    return '\x01';
  v2 = *__errno_location();
  v1 = sub_30e59(a0);
  if (v1 != '\x01')
    v2 = *__errno_location();
  sub_30db5(a0);
  *__errno_location() = v2;
  return v1;
}

// Function: sub_30fdc @ 0x30fdc
struct_75 * sub_30fdc(long *a0,void *a1,struct_75 *a2) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_30fdc
{
  bool v1; // al
  unsigned long *v2; // rax
  
  if (!a0)
    return (struct_75 *)gmtime_r(a1,(tm *)a2);
  v2 = (unsigned long *)sub_30ea3(a0);
  if (v2) {
    v1 = ((localtime_r(a1,(tm *)a2)) && (sub_30c11(a0,a2)));
    if ((sub_30f70(v2)) && (v1))
      return a2;
  }
  return NULL;
}

// Function: sub_3108c @ 0x3108c
long sub_3108c(long *a0,tm *a1) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_3108c
{
  bool v1; // al
  unsigned long *v2; // rax
  long v3; // rax
  tm v4; // stack - 0x58
  
  if (!a0)
    return timegm(a1);
  v2 = (unsigned long *)sub_30ea3(a0);
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
    v1 = ((0 <= (int)v4._28_4_) && (sub_30c11(a0,&v4)));
    if ((sub_30f70(v2)) && (v1)) {
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

// Function: sub_311f2 @ 0x311f2
void sub_311f2(unsigned int a0)
{
  sub_1db8e(a0,0,3);
}

// Function: sub_3121c @ 0x3121c
int sub_3121c(int a0)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_311f2(a0);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}

// Function: sub_3126d @ 0x3126d
void sub_3126d(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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
  fputs("\n",a0);
  fprintf(a0,gettext("License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n"),"https://gnu.org/licenses/gpl.html");
  fputs("\n",a0);
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

// Function: sub_31869 @ 0x31869
void sub_31869(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_3126d(a0,a1,a2,a3,a4,v1);
}

// Function: sub_318dc @ 0x318dc
void sub_318dc(FILE *a0,char *a1,char *a2,char *a3,struct_79 *a4)
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
  sub_3126d(a0,a1,a2,a3,v2,v3);
}

// Function: sub_319e2 @ 0x319e2
void sub_319e2(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_318dc(a8,a9,a10,a11,&v3);
}

// Function: sub_31ac6 @ 0x31ac6
void sub_31ac6(void)
{
  fputs("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-findutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU findutils","http://www.gnu.org/software/findutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_31b6f @ 0x31b6f
void sub_31b6f(void *a0,unsigned long a1,unsigned long a2)
{
  sub_31ca8(a0,a1,a2);
}

// Function: sub_31ba0 @ 0x31ba0
long sub_31ba0(long a0)
{
  if (!a0)
    sub_321fa(); // no-return
  return a0;
}

// Function: sub_31bc2 @ 0x31bc2
void sub_31bc2(unsigned long a0)
{
  sub_31ba0(malloc(a0));
}

// Function: sub_31be8 @ 0x31be8
void sub_31be8(unsigned long a0)
{
  sub_31ba0(sub_341af(a0));
}

// Function: sub_31c0e @ 0x31c0e
void sub_31c0e(unsigned long a0)
{
  sub_31bc2(a0);
}

// Function: sub_31c2c @ 0x31c2c
void * sub_31c2c(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_321fa(); // no-return
  return v1;
}

// Function: sub_31c77 @ 0x31c77
void sub_31c77(void *a0,unsigned long a1)
{
  sub_31ba0(sub_341cd(a0,a1));
}

// Function: sub_31ca8 @ 0x31ca8
void * sub_31ca8(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_321fa(); // no-return
  return v1;
}

// Function: sub_31d02 @ 0x31d02
void sub_31d02(void *a0,unsigned long a1,unsigned long a2)
{
  sub_31ba0(sub_3422d(a0,a1,a2));
}

// Function: sub_31d3b @ 0x31d3b
void sub_31d3b(unsigned long a0,unsigned long a1)
{
  sub_31ca8(NULL,a0,a1);
}

// Function: sub_31d66 @ 0x31d66
void sub_31d66(unsigned long a0,unsigned long a1)
{
  sub_31d02(NULL,a0,a1);
}

// Function: sub_31d91 @ 0x31d91
void sub_31d91(void *a0,unsigned long *a1)
{
  sub_31dbf(a0,a1,1);
}

// Function: sub_31dbf @ 0x31dbf
unsigned long sub_31dbf(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_321fa(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_31ca8(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_31e9e @ 0x31e9e
unsigned long sub_31e9e(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_321fa(); // no-return
  v3 = sub_31c2c(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_3205a @ 0x3205a
void sub_3205a(unsigned long a0)
{
  sub_320a0(a0,1);
}

// Function: sub_3207d @ 0x3207d
void sub_3207d(unsigned long a0)
{
  sub_320d1(a0,1);
}

// Function: sub_320a0 @ 0x320a0
void sub_320a0(unsigned long a0,unsigned long a1)
{
  sub_31ba0(calloc(a0,a1));
}

// Function: sub_320d1 @ 0x320d1
void sub_320d1(unsigned long a0,unsigned long a1)
{
  sub_31ba0(sub_34204(a0,a1));
}

// Function: sub_32102 @ 0x32102
void sub_32102(void *a0,unsigned long a1)
{
  memcpy((void *)sub_31bc2(a1),a0,a1);
}

// Function: sub_3213a @ 0x3213a
void sub_3213a(void *a0,unsigned long a1)
{
  memcpy((void *)sub_31be8(a1),a0,a1);
}

// Function: sub_3217a @ 0x3217a
void sub_3217a(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_31be8(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_321c9 @ 0x321c9
void sub_321c9(char *a0)
{
  sub_32102(a0,strlen(a0) + 1);
}

// Function: sub_321fa @ 0x321fa
void sub_321fa(void)
{
  error(dat_49830,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_3223a @ 0x3223a
char sub_3223a(char *a0,unsigned long *a1,double *a2,void *a3)
{
  char *v1; // stack - 0x20
  double v2; // xmm0_qa
  char v3; // stack - 0x21
  double v4; // stack - 0x18
  
  v3 = 1;
  *__errno_location() = 0;
  (*a3)(a0,&v1);
  v4 = v2;
  if ((a0 != v1) && ((a1 || (!*v1)))) {
    if ((v2 == 0.0) || (*__errno_location() != 0x22)) goto label_322d9;
    v3 = 0;
  }
  else {
    v3 = 0;
  }
label_322d9:
  if (a1)
    *a1 = v1;
  *a2 = v4;
  return v3;
}

// Function: sub_32312 @ 0x32312
bool sub_32312(unsigned long *a0,int a1)
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

// Function: sub_323a4 @ 0x323a4
unsigned int sub_323a4(unsigned long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_32312(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_323e9 @ 0x323e9
unsigned int sub_323e9(unsigned char *a0,char **a1,int a2,unsigned long *a3,char *a4) // early-return x2
{
  char v1;
  char **v10; // stack - 0x18
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoul"); // no-return
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
  v3 = strtoul((char *)a0,v10,a2);
  if (a0 != (unsigned char *)*v10) { // branch-flip
    if (*__errno_location()) {
      if (*__errno_location() != 0x22)
        return 4;
      v5 = 1;
    }
  }
  else {
    if (((!a4) || (!**v10)) || (v1 = **v10, !strchr(a4,(int)v1)))
      return 4;
    v3 = 1;
  }
  if (!a4) {
    *a3 = v3;
    return v5;
  }
  if (**v10) {
    v6 = 0x400;
    v7 = 1;
    v1 = **v10;
    if (!strchr(a4,(int)v1)) {
      *a3 = v3;
      return v5 | 2;
    }
    if ((((unsigned int)((int)**v10 - 0x45U) <= 0x2f) && (0x814400308945U >> ((unsigned char)((int)**v10 - 0x45U) & 0x3f) & 1)) && (strchr(a4,0x30))) {
      v1 = (*v10)[1];
      if (v1 != 'i') { // branch-flip
        if ((v1 <= 'i') && ((v1 == 'B' || (v1 == 'D')))) {
          v6 = 1000;
          v7 = 2;
        }
      }
      else if ((*v10)[2] == 'B')
        v7 = 3;
    }
    switch(**v10) {
      case 0x42:
        v8 = sub_32312(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_323a4(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_323a4(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_323a4(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_323a4(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_323a4(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_323a4(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_323a4(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_323a4(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_32312(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_32312(&v3,2);
      
    }
    v5 |= v8;
    *v10 = &(*v10)[v7];
    if (**v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_32847 @ 0x32847
bool sub_32847(unsigned long *a0,int a1)
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

// Function: sub_328d9 @ 0x328d9
unsigned int sub_328d9(unsigned long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_32847(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_3291e @ 0x3291e
unsigned int sub_3291e(unsigned char *a0,long *a1,int a2,unsigned long *a3,char *a4) // early-return x2
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoumax"); // no-return
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
        v8 = sub_32847(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_328d9(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_328d9(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_328d9(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_328d9(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_328d9(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_328d9(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_328d9(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_328d9(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_32847(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_32847(&v3,2);
      
    }
    v5 |= v8;
    *v10 = *v10 + (long)v7;
    if (*(char *)*v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_32d7c @ 0x32d7c
bool sub_32d7c(void)
{
  unsigned long v1; // stack - 0x20
  char *v2; // stack - 0x28
  bool v3; // stack - 0x29
  long v4; // stack - 0x18
  
  v2 = NULL;
  v1 = 0;
  v4 = getline(&v2,&v1,stdin);
  if (1 <= v4) { // branch-flip
    if (v2[v4 + -1] == '\n')
      v2[v4 + -1] = '\0';
    v3 = 0 < rpmatch(v2);
  }
  else {
    v3 = 0;
  }
  free(v2);
  return v3;
}

// Function: sub_32e34 @ 0x32e34
void sub_32e34(void)
{
  exit(dat_49830); // no-return
}

// Function: sub_32e49 @ 0x32e49
long sub_32e49(char *a0,long a1,long a2,unsigned long a3) // early-return
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

// Function: sub_32f7c @ 0x32f7c
long sub_32f7c(char *a0,long a1)
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

// Function: sub_32ff2 @ 0x32ff2
void sub_32ff2(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = gettext("ambiguous argument %s for %s");
  else {
    v1 = gettext("invalid argument %s for %s");
  }
  v2 = sub_2fd49(1,a0);
  error(0,0,v1,sub_2f861(0,8,a1),v2);
}

// Function: sub_3308b @ 0x3308b
void sub_3308b(long a0,long a1,unsigned long a2)
{
  char *v1;
  FILE *v2;
  long v3; // stack - 0x28
  void *v4; // stack - 0x20
  
  v2 = stderr;
  v4 = NULL;
  fputs(gettext("Valid arguments are:"),v2);
  v3 = 0;
  do {
    if (!*(long *)(a0 + v3 * 8)) {
      putc(10,stderr);
      return;
    }
    if (v3) { // branch-flip
      if (memcmp(v4,(void *)(a2 * v3 + a1),a2)) goto label_3310c;
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,", %s",(char *)sub_2fd76(v1));
    }
    else {
label_3310c:
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,"\n  - %s",(char *)sub_2fd76(v1));
      v4 = (void *)(a1 + a2 * v3);
    }
    v3 += 1;
  } while( true );
}

// Function: sub_331e2 @ 0x331e2
long sub_331e2(char *a0,char *a1,long a2,long a3,unsigned long a4,void *a5,unsigned int a6) // early-return, ternary
{
  long v1; // stack - 0x10
  
  v1 = ((char)a6) ? sub_32e49(a1,a2,a3,a4) : sub_32f7c(a1,a2); // branch-flip
  if (0 <= v1)
    return v1;
  sub_32ff2(a0,a1,v1);
  sub_3308b(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_33291 @ 0x33291
unsigned long sub_33291(void *a0,long a1,long a2,unsigned long a3)
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

// Function: sub_33321 @ 0x33321
char * sub_33321(char *a0,int a1,char *a2) // early-return, ternary
{
  int v1; // eax
  unsigned long v2; // rax
  char v3 [40];
  char *v4; // stack - 0x48
  
  if (*a2) { // branch-flip
    if (!dat_4a380) {
      v1 = open("/proc/self/fd",0x90900);
      if (0 <= v1) { // branch-flip
        sprintf(v3,"/proc/self/fd/%d/../fd",v1);
        dat_4a380 = (access(v3,0)) ? -1 : 1; // branch-flip
        close(v1);
      }
      else {
        dat_4a380 = -1;
      }
    }
    if (dat_4a380 <= -1)
      return NULL;
    v2 = strlen(a2) + 0x1b;
    v4 = a0;
    if (0xfc1 <= v2) {
      v4 = malloc(v2);
      if (!v4)
        return NULL;
    }
    strcpy(&v4[sprintf(v4,"/proc/self/fd/%d/",a1)],a2);
    a0 = v4;
  }
  else {
    *a0 = '\0';
  }
  return a0;
}

// Function: sub_33495 @ 0x33495
char * sub_33495(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_334ec;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_334ec:
  }
  return v2;
}

// Function: sub_33502 @ 0x33502
unsigned long sub_33502(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}

// Function: sub_33555 @ 0x33555
unsigned long sub_33555(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_33575 @ 0x33575
unsigned long sub_33575(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_33595 @ 0x33595
unsigned int sub_33595(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_335b1 @ 0x335b1
unsigned int sub_335b1(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_335cd @ 0x335cd
unsigned long sub_335cd(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_33606 @ 0x33606
unsigned long sub_33606(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_3363f @ 0x3363f
unsigned int sub_3363f(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_33675 @ 0x33675
unsigned int sub_33675(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_336ab @ 0x336ab
unsigned int sub_336ab(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_336e0 @ 0x336e0
unsigned int sub_336e0(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_33715 @ 0x33715
unsigned long sub_33715(int a0) // return-dupe
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

// Function: sub_33753 @ 0x33753
unsigned long sub_33753(int a0) // return-dupe
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

// Function: sub_33785 @ 0x33785
bool sub_33785(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_337a6 @ 0x337a6
unsigned int sub_337a6(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_337ce @ 0x337ce
unsigned long sub_337ce(int a0) // return-dupe
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

// Function: sub_337fb @ 0x337fb
bool sub_337fb(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_3381f @ 0x3381f
bool sub_3381f(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_33843 @ 0x33843
bool sub_33843(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_33867 @ 0x33867
bool sub_33867(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_3388b @ 0x3388b
unsigned long sub_3388b(int a0) // return-dupe, return-dupe x3
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

// Function: sub_338d5 @ 0x338d5
unsigned long sub_338d5(int a0) // return-dupe
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

// Function: sub_33902 @ 0x33902
bool sub_33902(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_33926 @ 0x33926
unsigned long sub_33926(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_3396e @ 0x3396e
int sub_3396e(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_33991 @ 0x33991
int sub_33991(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_339b4 @ 0x339b4
int sub_339b4(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_3396e((unsigned char)*a0);
    v2 = sub_3396e((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_33a2e @ 0x33a2e
void sub_33a2e(unsigned int *a0)
{
  *a0 = 0xffffff9c;
}

// Function: sub_33a47 @ 0x33a47
void sub_33a47(int *a0)
{
  fchdir(*a0);
}

// Function: sub_33a66 @ 0x33a66
void sub_33a66(int *a0)
{
  int v1;
  
  if (0 <= *a0) {
    v1 = *a0;
    if (close(v1))
      __assert_fail("! close_fail","chdir-long.c",0x40,"cdb_free"); // no-return
  }
}

// Function: sub_33acb @ 0x33acb
unsigned long sub_33acb(int *a0,char *a1) // return-dupe
{
  int v1; // eax
  
  v1 = openat(*a0,a1,0x10900);
  if (v1 <= -1)
    return 0xffffffff;
  sub_33a66(a0);
  *a0 = v1;
  return 0;
}

// Function: sub_33b29 @ 0x33b29
char * sub_33b29(char *a0)
{
  return &a0[strspn(a0,"/")];
}

// Function: sub_33b60 @ 0x33b60
int sub_33b60(char *a0)
{
  char *v1; // rax
  unsigned long v10; // stack - 0x28
  char *v11; // stack - 0x20
  char *v12; // stack - 0x18
  char v2 [4];
  char *v3; // stack - 0x60
  int v4; // stack - 0x48
  int v5; // stack - 0x44
  int v6; // stack - 0x40
  int v7; // stack - 0x3c
  unsigned long v8; // stack - 0x38
  char *v9; // stack - 0x30
  
  v4 = chdir(a0);
  if ((!v4) || (*__errno_location() != 0x24))
    return v4;
  v8 = strlen(a0);
  v9 = &a0[v8];
  sub_33a2e(v2);
  if (!v8)
    __assert_fail("0 < len","chdir-long.c",0x7e,"chdir_long"); // no-return
  if (v8 <= 0xfff)
    __assert_fail("4096 <= len","chdir-long.c",0x7f,"chdir_long"); // no-return
  v10 = strspn(a0,"/");
  if (v10 != 2) { // branch-flip
    v3 = a0;
    if (v10) {
      if (sub_33acb(v2,"/")) goto label_33e8b;
      v3 = &a0[v10];
    }
  }
  else {
    v11 = memchr(&a0[3],0x2f,(long)v9 - (long)&a0[3]);
    if (!v11) {
      *__errno_location() = 0x24;
      return -1;
    }
    *v11 = 0;
    v5 = sub_33acb(v2,a0);
    *v11 = 0x2f;
    if (v5) goto label_33e8b;
    v1 = &v11[1];
    v3 = (char *)sub_33b29(v1);
  }
  if (*v3 == '/')
    __assert_fail("*dir != \'/\'","chdir-long.c",0xa2,"chdir_long"); // no-return
  if (v9 < v3)
    __assert_fail("dir <= dir_end","chdir-long.c",0xa3,"chdir_long"); // no-return
  while (0xfff < (long)v9 - (long)v3) {
    v12 = memrchr(v3,0x2f,0x1000);
    if (!v12) {
      *__errno_location() = 0x24;
      return -1;
    }
    *v12 = 0;
    if (0x1000 <= (long)v12 - (long)v3)
      __assert_fail("slash - dir < 4096","chdir-long.c",0xb3,"chdir_long"); // no-return
    v6 = sub_33acb(v2,v3);
    *v12 = 0x2f;
    if (v6) goto label_33e8b;
    v1 = &v12[1];
    v3 = (char *)sub_33b29(v1);
  }
  if (((v9 <= v3) || (!sub_33acb(v2,v3))) && (!sub_33a47(v2))) {
    sub_33a66(v2);
    return 0;
  }
label_33e8b:
  v7 = *__errno_location();
  sub_33a66(v2);
  *__errno_location() = v7;
  return -1;
}

// Function: sub_33ec6 @ 0x33ec6
unsigned long sub_33ec6(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror(a0);
  v3 = sub_1dad5(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_33f55 @ 0x33f55
unsigned long sub_33f55(unsigned long a0)
{
  a0 = a0 - 1 & a0;
  return CONCAT71((undefined7)(a0 >> 8),a0 == 0);
}

// Function: sub_33f75 @ 0x33f75
void sub_33f75(void *a0)
{
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned int *)((long)a0 + 0x18) = 0x95f616;
}

// Function: sub_33f9b @ 0x33f9b
unsigned long sub_33f9b(struct_36 *a0,struct_24 *a1) // return-dupe
{
  unsigned long v1;
  
  if (a0->field_0x18 != 0x95f616)
    __assert_fail("state->magic == 9827862","cycle-check.c",0x3c,"cycle_check"); // no-return
  if (((a0->field_0x10) && (a1->field_0x8 == a0->field_0x0)) && (a1->field_0x0 == a0->field_0x8))
    return 1;
  a0->field_0x10 = a0->field_0x10 + 1;
  v1 = a0->field_0x10;
  if (!sub_33f55(v1))
    return 0;
  if (a0->field_0x10) {
    a0->field_0x8 = a1->field_0x0;
    a0->field_0x0 = a1->field_0x8;
    return 0;
  }
  return 1;
}

// Function: sub_34082 @ 0x34082
void sub_34082(timespec *a0)
{
  clock_gettime(0,a0);
}

// Function: sub_340a6 @ 0x340a6
undefined16 sub_340a6(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_34082(v1);
  return v1._0_16_;
}

// Function: sub_340eb @ 0x340eb
unsigned int sub_340eb(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_37102(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_34195 @ 0x34195
unsigned long sub_34195(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_341af @ 0x341af
void sub_341af(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_341cd @ 0x341cd
void sub_341cd(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_34204 @ 0x34204
void sub_34204(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_3422d @ 0x3422d
void sub_3422d(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_3427c @ 0x3427c
char * sub_3427c(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_342c4 @ 0x342c4
long sub_342c4(unsigned long a0) // early-return
{
  void *v1; // rax
  long v2; // rax
  long v3; // rax
  
  if ((a0 <= a0 + 0x8000000000000020) && (v1 = malloc(a0 + 0x20), v1)) {
    v2 = (((long)v1 + 0x10U & 0xffffffffffffffe0) - (long)v1) + 0x10;
    v3 = (long)v1 + v2;
    *(char *)(v3 + -1) = (char)v2;
    return v3;
  }
  return 0;
}

// Function: sub_343d1 @ 0x343d1
void sub_343d1(void *a0) // return-dupe
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1)));
}

// Function: sub_3442c @ 0x3442c
unsigned int sub_3442c(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_34467 @ 0x34467
void sub_34467(struct_16 *a0,struct_17 *a1) // return-dupe
{
  if ((struct_17 *)a1->field_0x0 != &a1[1]) // branch-flip
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

// Function: sub_3450d @ 0x3450d
bool sub_3450d(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0x3eb60) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_34552 @ 0x34552
char * sub_34552(char *a0)
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
      sub_256ef(v4);
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

// Function: sub_34622 @ 0x34622
long sub_34622(long a0,long a1,long a2)
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

// Function: sub_3467a @ 0x3467a
long sub_3467a(long a0,long a1,long a2)
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

// Function: sub_346d2 @ 0x346d2
int sub_346d2(int a0,int a1)
{
  return (a0 - ((a0 - a1) + 0x17e) % 7) + 3;
}

// Function: sub_34722 @ 0x34722
void sub_34722(char *a0,long a1,char *a2,tm *a3,unsigned long a4,unsigned int a5)
{
  char v1; // stack - 0x11
  
  v1 = 0;
  sub_347a5(a0,a1,a2,a3,0,0,0xffffffffffffffff,&v1,a4,a5);
}

// Function: sub_347a5 @ 0x347a5
long sub_347a5(char *a0,long a1,char *a2,tm *a3,char a4,int a5,int a6,unsigned long a7,unsigned long a8,int a9) // early-return x13, ternary x6, warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_347a5
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
      goto label_36fb5;
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
label_36db3:
        v42 = &v42[-1];
        break;
      case 0x25:
        if (v6 != &v42[-1]) goto label_36db3;
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
        goto label_36fb5;
      case 0x3a:
        for (v45 = 1; v42[v45] == ':'; v45 = v45 + 1) {
        }
        if (v42[v45] == 'z') {
          v42 = &v42[v45];
          goto label_36b84;
        }
        break;
      case 0x41:
        if (!v8) {
          if (v7)
            v24 = 1;
          v3 = 0;
          goto label_3501f;
        }
        break;
      case 0x42:
        if (v8 != 'E') {
          if (v7)
            v24 = 1;
          v3 = 0;
          goto label_3501f;
        }
        break;
      case 0x43:
        if (v8 == 'E') goto label_3501f;
        v23 = *(int *)&a3->field_0x14 < -0x76c;
        v29 = (int)((unsigned int)((*(unsigned int *)&a3->field_0x14 >> 0x1f & (unsigned int)!(bool)v23) != 0) * -99 + *(int *)&a3->field_0x14) / 100 + 0x13;
        v27 = 2;
label_353d1:
        if (!v26)
          v26 = a5;
        if (v26 != 0x2b) { // branch-flip
label_3541e:
          v7 = 0;
        }
        else {
          v10 = (v27 != 2) ? 9999 : 99; // branch-flip
          if ((v29 <= v10) && (a6 <= v27)) goto label_3541e;
          v7 = 1;
        }
        goto label_3546a;
      case 0x44:
        if (!v8) {
          v43 = "%m/%d/%y";
          goto label_34e0b;
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
        goto label_34e15;
      case 0x47:
      case 0x56:
      case 0x67:
        if (v8 != 'E') {
          v9 = (0 <= *(int *)&a3->field_0x14) ? -100 : 300; // branch-flip
          v10 = v9 + *(int *)&a3->field_0x14;
          v36 = 0;
          v37 = sub_346d2(*(unsigned int *)&a3->field_0x1c,*(unsigned int *)&a3->field_0x18);
          if (0 <= v37) { // branch-flip
            v9 = !((v10 & 3) || ((!((int)v10 % 100) && ((int)v10 % 400))));
            v9 = sub_346d2(*(int *)&a3->field_0x1c - (v9 + 0x16d),*(unsigned int *)&a3->field_0x18);
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
            v37 = sub_346d2(v11 + 0x16d + v9,v2);
          }
          if (*v42 != 'G') { // branch-flip
            if (*v42 != 'g') {
              v27 = 2;
              v28 = v37 / 7 + 1;
              goto label_35448;
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
          goto label_353d1;
        }
        break;
      case 0x48:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)&a3->field_0x8;
          goto label_35448;
        }
        break;
      case 0x49:
        if (v8 != 'E') {
          v27 = 2;
          v28 = v25;
          goto label_35448;
        }
        break;
      case 0x4d:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)&a3->field_0x4;
          goto label_35448;
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
            sub_3467a(v41,v20,v12);
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
        goto label_36fb5;
      case 0x50:
        v3 = 1;
        v31 = 'p';
        goto label_36055;
      case 0x52:
        v43 = "%H:%M";
        goto label_34e0b;
      case 0x53:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)a3;
          goto label_35448;
        }
        break;
      case 0x54:
        v43 = "%H:%M:%S";
label_34e0b:
        v32 = -1;
label_34e15:
        v12 = sub_347a5(0,0xffffffffffffffff,v43,a3,(unsigned char)v24,v26,v32,v4,v5,a9);
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
          sub_347a5(v41,a1 - v40,v43,a3,(unsigned char)v24,v26,v32,v4,v5,a9);
          v41 = &v41[v12];
        }
        v40 += v14;
        goto label_36fb5;
      case 0x55:
        if (v8 != 'E') {
          v27 = 2;
          v28 = ((*(int *)&a3->field_0x1c - *(int *)&a3->field_0x18) + 7) / 7;
          goto label_35448;
        }
        break;
      case 0x57:
        if (v8 != 'E') {
          v27 = 2;
          v28 = ((*(int *)&a3->field_0x1c - (*(int *)&a3->field_0x18 + 6) % 7) + 7) / 7;
          goto label_35448;
        }
        break;
      case 0x58:
        goto label_35345;
      case 0x59:
        if (v8 == 'E') goto label_3501f;
        if (v8 != 'O') {
          v27 = 4;
          v23 = *(int *)&a3->field_0x14 < -0x76c;
          v29 = *(int *)&a3->field_0x14 + 0x76c;
          goto label_353d1;
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
            sub_34622(v41,v39,v12);
          else if (v24) // branch-flip
            sub_3467a(v41,v39,v12);
          else {
            memcpy(v41,v39,v12);
          }
          v41 = &v41[v12];
        }
        v40 += v14;
        goto label_36fb5;
      case 0x61:
        if (!v8) {
          if (v7)
            v24 = 1;
          v3 = 0;
          goto label_3501f;
        }
        break;
      case 0x62:
      case 0x68:
        if (v7)
          v24 = 1;
        v3 = 0;
        if (v8 != 'E') goto label_3501f;
        break;
      case 99:
        goto label_35345;
      case 100:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)&a3->field_0xc;
          goto label_35448;
        }
        break;
      case 0x65:
        if (v8 != 'E') {
          v28 = *(unsigned int *)&a3->field_0xc;
label_35432:
          v27 = 2;
          if (!v26)
            v26 = 0x5f;
          goto label_35448;
        }
        break;
      case 0x6a:
        if (v8 != 'E') {
          v27 = 3;
          v23 = *(int *)&a3->field_0x1c < -1;
          v29 = *(int *)&a3->field_0x1c + 1;
          goto label_35463;
        }
        break;
      case 0x6b:
        if (v8 != 'E') {
          v28 = *(unsigned int *)&a3->field_0x8;
          goto label_35432;
        }
        break;
      case 0x6c:
        if (v8 != 'E') {
          v28 = v25;
          goto label_35432;
        }
        break;
      case 0x6d:
        if (v8 != 'E') {
          v27 = 2;
          v23 = *(int *)&a3->field_0x10 < -1;
          v29 = *(int *)&a3->field_0x10 + 1;
          goto label_35463;
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
        goto label_36fb5;
      case 0x70:
label_36055:
        if (v7) {
          v24 = 0;
          v3 = 1;
        }
        goto label_3501f;
      case 0x71:
        v27 = 1;
        v23 = 0;
        v29 = (*(int *)&a3->field_0x10 * 0xb >> 5) + 1;
        goto label_35463;
      case 0x72:
        goto label_3501f;
      case 0x73:
        v22 = *(unsigned long *)a3;
        v48 = *(unsigned long *)&a3->field_0x8;
        v49 = *(unsigned long *)&a3->field_0x10;
        v51 = *(unsigned long *)&a3->field_0x20;
        v52 = *(unsigned long *)&a3->field_0x28;
        v53 = *(unsigned long *)&a3->field_0x30;
        v50._0_4_ = (unsigned int)*(unsigned long *)&a3->field_0x18;
        v50 = CONCAT44(0xffffffff,(unsigned int)v50);
        v47 = sub_3108c(v5,&v22);
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
        goto label_3554d;
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
        goto label_36fb5;
      case 0x75:
        v27 = 1;
        v28 = (*(int *)&a3->field_0x18 + 6) % 7 + 1;
label_35448:
        v23 = (bool)(v28 >> 0x1f);
        v29 = v28;
label_35463:
        v7 = 0;
label_3546a:
        v30 = 0;
label_35474:
        if ((v8 == 'O') && ((bool)v23 != 1)) goto label_3501f;
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
label_3554d:
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
            sub_3467a(v41,v44,v12);
          else {
            memcpy(v41,v44,v12);
          }
          v41 = &v41[v12];
        }
        v40 += v14;
        goto label_36fb5;
      case 0x77:
        if (v8 != 'E') {
          v27 = 1;
          v28 = *(unsigned int *)&a3->field_0x18;
          goto label_35448;
        }
        break;
      case 0x78:
label_35345:
        if (v8 != 'O') goto label_3501f;
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
          goto label_353d1;
        }
label_3501f:
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
              sub_34622(v41,&v20[1],v12);
            else if (v24) // branch-flip
              sub_3467a(v41,&v20[1],v12);
            else {
              memcpy(v41,&v20[1],v12);
            }
            v41 = &v41[v12];
          }
          v40 += v14;
        }
        goto label_36fb5;
      case 0x7a:
        v45 = 0;
label_36b84:
        if (*(int *)&a3->field_0x20 < 0) goto label_36fb5;
        v9 = (int)*(unsigned long *)&a3->field_0x28;
        v23 = !((0 <= v9) && ((v9 || (*v39 != '-'))));
        v29 = v9 / 0xe10;
        v11 = (v9 / 0x3c) % 0x3c;
        if (v45 != 3) {
          if (v45 <= 3) {
            if (v45 == 2) goto label_36d39;
            if (v45 <= 2) {
              if (!v45) {
                v27 = 5;
                v30 = 0;
                v29 = v11 + v29 * 100;
                goto label_353c5;
              }
              if (v45 == 1) goto label_36d08;
            }
          }
          break;
        }
        if (v9 % 0x3c) { // branch-flip
label_36d39:
          v27 = 9;
          v30 = 0x14;
          v29 = v9 % 0x3c + v29 * 10000 + v11 * 100;
        }
        else if (v11) { // branch-flip
label_36d08:
          v27 = 6;
          v30 = 4;
          v29 = v11 + v29 * 100;
        }
        else {
          v27 = 3;
          v30 = 0;
        }
label_353c5:
        v7 = 1;
        goto label_35474;
      
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
        sub_3467a(v41,v6,v15);
      else {
        memcpy(v41,v6,(unsigned long)v15);
      }
      v41 = &v41[(long)v15];
    }
    v40 += (long)v17;
label_36fb5:
    a6 = -1;
    v42 = &v42[1];
  } while( true );
}

// Function: sub_37022 @ 0x37022
char * sub_37022(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_3704a @ 0x3704a
unsigned long sub_3704a(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_37022(a0);
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

// Function: sub_37102 @ 0x37102
void sub_37102(unsigned int a0,char *a1,unsigned long a2)
{
  sub_3704a(a0,a1,a2);
}

// Function: sub_37130 @ 0x37130
void sub_37130(unsigned int a0)
{
  sub_37022(a0);
}

// Function: sub_37150 @ 0x37150
void sub_37150(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_49008); // tail-call
}

// Function: _DT_FINI @ 0x37164
void _DT_FINI(void)
{
  return;
}
