// Function: _DT_INIT @ 0x6000
void _DT_INIT(void) // return-dupe
{
  if (!dat_3df40)
    return;
  (*dat_3df40)();
}

// Function: sub_6020 @ 0x6020
void sub_6020(void)
{
  (*dat_3da60)(); // jump-as-call
}

// Function: free @ 0x69b0
void free(void *a0)
{
  (*dat_3df78)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x69c0
void __cxa_finalize(void)
{
  (*dat_3df90)(); // jump-as-call
}

// Function: ctime @ 0x69d0
void ctime(void)
{
  (*dat_3da68)(); // jump-as-call
}

// Function: setmntent @ 0x69e0
void setmntent(void)
{
  (*dat_3da70)(); // jump-as-call
}

// Function: chdir @ 0x69f0
int chdir(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_3da78)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x6a00
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3da80)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x6a10
void strtoumax(void)
{
  (*dat_3da88)(); // jump-as-call
}

// Function: endmntent @ 0x6a20
void endmntent(void)
{
  (*dat_3da90)(); // jump-as-call
}

// Function: mktime @ 0x6a30
void mktime(void)
{
  (*dat_3da98)(); // jump-as-call
}

// Function: memset @ 0x6a40
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3daa0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x6a50
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3daa8)(); // jump-as-call
  return v1;
}

// Function: getgrnam @ 0x6a60
void * getgrnam(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dab0)(); // jump-as-call
  return v1;
}

// Function: snprintf @ 0x6a70
int snprintf(char *a0,unsigned long a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_3dab8)(); // jump-as-call
  return v1;
}

// Function: close @ 0x6a80
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_3dac0)(); // jump-as-call
  return v1;
}

// Function: hasmntopt @ 0x6a90
void hasmntopt(void)
{
  (*dat_3dac8)(); // jump-as-call
}

// Function: abort @ 0x6aa0
void abort(void)
{
  (*dat_3dad0)(); // jump-as-call
}

// Function: memchr @ 0x6ab0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dad8)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x6ac0
void fstatfs(void)
{
  (*dat_3dae0)(); // jump-as-call
}

// Function: clock_gettime @ 0x6ad0
int clock_gettime(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3dae8)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x6ae0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3daf0)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x6af0
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_3daf8)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x6b00
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_3db00)(); // jump-as-call
  return v1;
}

// Function: uname @ 0x6b10
void uname(void)
{
  (*dat_3db08)(); // jump-as-call
}

// Function: textdomain @ 0x6b20
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3db10)(); // jump-as-call
  return v1;
}

// Function: __isoc99_sscanf @ 0x6b30
int __isoc99_sscanf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3db18)(); // jump-as-call
  return v1;
}

// Function: execvp @ 0x6b40
void execvp(void)
{
  (*dat_3db20)(); // jump-as-call
}

// Function: exit @ 0x6b50
void exit(int a0)
{
  (*dat_3db28)(); // jump-as-call
}

// Function: __assert_fail @ 0x6b60
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_3db30)(); // jump-as-call
}

// Function: fstat @ 0x6b70
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3db38)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x6b80
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3db40)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x6b90
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3db48)(); // jump-as-call
  return v1;
}

// Function: gettimeofday @ 0x6ba0
int gettimeofday(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3db50)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x6bb0
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_3db58)(); // jump-as-call
  return v1;
}

// Function: strpbrk @ 0x6bc0
char * strpbrk(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3db60)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x6bd0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3db68)(); // jump-as-call
  return v1;
}

// Function: getmntent @ 0x6be0
void getmntent(void)
{
  (*dat_3db70)(); // jump-as-call
}

// Function: strncmp @ 0x6bf0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_3db78)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x6c00
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3db80)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x6c10
void freecon(void)
{
  (*dat_3db88)(); // jump-as-call
}

// Function: fopen @ 0x6c20
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3db90)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_minor @ 0x6c30
void gnu_dev_minor(void)
{
  (*dat_3db98)(); // jump-as-call
}

// Function: fchdir @ 0x6c40
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_3dba0)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x6c50
void _exit(int a0)
{
  (*dat_3dba8)(); // jump-as-call
}

// Function: __memcpy_chk @ 0x6c60
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dbb0)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x6c70
void __cxa_atexit(void)
{
  (*dat_3dbb8)(); // jump-as-call
}

// Function: sysconf @ 0x6c80
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_3dbc0)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x6c90
void fgetfilecon(void)
{
  (*dat_3dbc8)(); // jump-as-call
}

// Function: tzset @ 0x6ca0
void tzset(void)
{
  (*dat_3dbd0)(); // jump-as-call
}

// Function: getpwuid @ 0x6cb0
void * getpwuid(unsigned int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dbd8)(); // jump-as-call
  return v1;
}

// Function: atof @ 0x6cc0
void atof(void)
{
  (*dat_3dbe0)(); // jump-as-call
}

// Function: lsetfilecon @ 0x6cd0
void lsetfilecon(void)
{
  (*dat_3dbe8)(); // jump-as-call
}

// Function: re_compile_pattern @ 0x6ce0
void re_compile_pattern(void)
{
  (*dat_3dbf0)(); // jump-as-call
}

// Function: strnlen @ 0x6cf0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3dbf8)(); // jump-as-call
  return v1;
}

// Function: is_selinux_enabled @ 0x6d00
void is_selinux_enabled(void)
{
  (*dat_3dc00)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x6d10
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dc08)(); // jump-as-call
  return v1;
}

// Function: fputc @ 0x6d20
int fputc(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3dc10)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x6d30
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_3dc18)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x6d40
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3dc20)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x6d50
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3dc28)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x6d60
void * opendir(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dc30)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x6d70
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3dc38)(); // jump-as-call
  return v1;
}

// Function: __vfprintf_chk @ 0x6d80
int __vfprintf_chk(void *a0,int a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_3dc40)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x6d90
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3dc48)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x6da0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dc50)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x6db0
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dc58)(); // jump-as-call
  return v1;
}

// Function: bsearch @ 0x6dc0
void bsearch(void)
{
  (*dat_3dc60)(); // jump-as-call
}

// Function: sprintf @ 0x6dd0
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3dc68)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x6de0
void * fdopen(int a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dc70)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x6df0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dc78)(); // jump-as-call
  return v1;
}

// Function: strtok_r @ 0x6e00
char * strtok_r(char *a0,char *a1,char **a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dc80)(); // jump-as-call
  return v1;
}

// Function: timegm @ 0x6e10
void timegm(void)
{
  (*dat_3dc88)(); // jump-as-call
}

// Function: poll @ 0x6e20
void poll(void)
{
  (*dat_3dc90)(); // jump-as-call
}

// Function: gmtime_r @ 0x6e30
void gmtime_r(void)
{
  (*dat_3dc98)(); // jump-as-call
}

// Function: strstr @ 0x6e40
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dca0)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x6e50
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3dca8)(); // jump-as-call
  return v1;
}

// Function: abs @ 0x6e60
int abs(int a0)
{
  int v1; // eax
  
  v1 = (*dat_3dcb0)(); // jump-as-call
  return v1;
}

// Function: strcat @ 0x6e70
char * strcat(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dcb8)(); // jump-as-call
  return v1;
}

// Function: memrchr @ 0x6e80
void memrchr(void)
{
  (*dat_3dcc0)(); // jump-as-call
}

// Function: fputs @ 0x6e90
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3dcc8)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x6ea0
void lseek(void)
{
  (*dat_3dcd0)(); // jump-as-call
}

// Function: strtol @ 0x6eb0
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_3dcd8)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x6ec0
int dirfd(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3dce0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x6ed0
void iswcntrl(void)
{
  (*dat_3dce8)(); // jump-as-call
}

// Function: endpwent @ 0x6ee0
void endpwent(void)
{
  (*dat_3dcf0)(); // jump-as-call
}

// Function: putc @ 0x6ef0
int putc(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3dcf8)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x6f00
void re_set_syntax(void)
{
  (*dat_3dd00)(); // jump-as-call
}

// Function: signal @ 0x6f10
void signal(void)
{
  (*dat_3dd08)(); // jump-as-call
}

// Function: strspn @ 0x6f20
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3dd10)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x6f30
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dd18)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x6f40
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dd20)(); // jump-as-call
  return v1;
}

// Function: waitpid @ 0x6f50
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_3dd28)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x6f60
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3dd30)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x6f70
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_3dd38)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x6f80
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dd40)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x6f90
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3dd48)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x6fa0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_3dd50)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x6fb0
void qsort(void)
{
  (*dat_3dd58)(); // jump-as-call
}

// Function: strdup @ 0x6fc0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dd60)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x6fd0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dd68)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x6fe0
void __stack_chk_fail(void)
{
  (*dat_3dd70)(); // jump-as-call
}

// Function: strcmp @ 0x6ff0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_3dd78)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x7000
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_3dd80)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x7010
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dd88)(); // jump-as-call
  return v1;
}

// Function: getgrgid @ 0x7020
void * getgrgid(unsigned int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dd90)(); // jump-as-call
  return v1;
}

// Function: getline @ 0x7030
long getline(char **a0,void *a1,void *a2)
{
  long v1; // rax
  
  v1 = (*dat_3dd98)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x7040
void localeconv(void)
{
  (*dat_3dda0)(); // jump-as-call
}

// Function: strcpy @ 0x7050
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3dda8)(); // jump-as-call
  return v1;
}

// Function: setfilecon @ 0x7060
void setfilecon(void)
{
  (*dat_3ddb0)(); // jump-as-call
}

// Function: endgrent @ 0x7070
void endgrent(void)
{
  (*dat_3ddb8)(); // jump-as-call
}

// Function: rpmatch @ 0x7080
void rpmatch(void)
{
  (*dat_3ddc0)(); // jump-as-call
}

// Function: __ctype_tolower_loc @ 0x7090
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3ddc8)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x70a0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_3ddd0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x70b0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3ddd8)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x70c0
int feof(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3dde0)(); // jump-as-call
  return v1;
}

// Function: faccessat @ 0x70d0
void faccessat(void)
{
  (*dat_3dde8)(); // jump-as-call
}

// Function: fclose @ 0x70e0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3ddf0)(); // jump-as-call
  return v1;
}

// Function: strncpy @ 0x70f0
char * strncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3ddf8)(); // jump-as-call
  return v1;
}

// Function: getfilecon @ 0x7100
void getfilecon(void)
{
  (*dat_3de00)(); // jump-as-call
}

// Function: getrlimit @ 0x7110
void getrlimit(void)
{
  (*dat_3de08)(); // jump-as-call
}

// Function: difftime @ 0x7120
void difftime(void)
{
  (*dat_3de10)(); // jump-as-call
}

// Function: localtime_r @ 0x7130
void * localtime_r(void *a0,void *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3de18)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_major @ 0x7140
void gnu_dev_major(void)
{
  (*dat_3de20)(); // jump-as-call
}

// Function: fseeko @ 0x7150
void fseeko(void)
{
  (*dat_3de28)(); // jump-as-call
}

// Function: gnu_dev_makedev @ 0x7160
void gnu_dev_makedev(void)
{
  (*dat_3de30)(); // jump-as-call
}

// Function: unsetenv @ 0x7170
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_3de38)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x7180
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3de40)(); // jump-as-call
  return v1;
}

// Function: __sprintf_chk @ 0x7190
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_3de48)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x71a0
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_3de50)(); // jump-as-call
  return v1;
}

// Function: access @ 0x71b0
int access(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_3de58)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x71c0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_3de60)(); // jump-as-call
  return v1;
}

// Function: modf @ 0x71d0
void modf(void)
{
  (*dat_3de68)(); // jump-as-call
}

// Function: unlinkat @ 0x71e0
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_3de70)(); // jump-as-call
  return v1;
}

// Function: fwrite @ 0x71f0
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3de78)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x7200
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3de80)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x7210
int lstat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3de88)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x7220
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3de90)(); // jump-as-call
  return v1;
}

// Function: error @ 0x7230
void error(int a0,int a1,char *a2,...)
{
  (*dat_3de98)(); // jump-as-call
}

// Function: localtime @ 0x7240
void * localtime(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3dea0)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x7250
void getdelim(void)
{
  (*dat_3dea8)(); // jump-as-call
}

// Function: strftime @ 0x7260
unsigned long strftime(char *a0,unsigned long a1,char *a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3deb0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x7270
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_3deb8)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x7280
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3dec0)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x7290
int fstatat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_3dec8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x72a0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3ded0)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x72b0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3ded8)(); // jump-as-call
  return v1;
}

// Function: open @ 0x72c0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3dee0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x72d0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_3dee8)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x72e0
void * getpwnam(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3def0)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x72f0
void lgetfilecon(void)
{
  (*dat_3def8)(); // jump-as-call
}

// Function: re_match @ 0x7300
void re_match(void)
{
  (*dat_3df00)(); // jump-as-call
}

// Function: strndup @ 0x7310
char * strndup(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_3df08)(); // jump-as-call
  return v1;
}

// Function: time @ 0x7320
void time(void)
{
  (*dat_3df10)(); // jump-as-call
}

// Function: fflush @ 0x7330
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3df18)(); // jump-as-call
  return v1;
}

// Function: fdopendir @ 0x7340
void * fdopendir(int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3df20)(); // jump-as-call
  return v1;
}

// Function: sub_7350 @ 0x7350
undefined16 sub_7350(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: sub_7370 @ 0x7370
unsigned long sub_7370(int a0,long *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  
  v3 = (char *)*a1;
  if (!v3)
    v3 = "find";
  sub_22cb0(v3);
  sub_d970();
  dat_3e618 = 0;
  dat_3e604 = 0;
  dat_3e608 = 0;
  dat_3e5f8 = 0xffffff9c;
  if (sub_15dd0())
    sub_15c30();
  dat_3e610 = sub_e4d0("w");
  if (dat_3e610) {
    sub_de90(0x3e620);
    setlocale(6,"");
    bindtextdomain("findutils","/usr/local/share/locale");
    textdomain("findutils");
    if (!sub_2a3c0(sub_175a0)) {
      v1 = sub_dd00(a0,a1);
      if (dat_3e660 & 2)
        dat_3e670 = sub_db70;
      if (dat_3e660 & 0x80)
        __fprintf_chk(stderr,1,"cur_day_start = %s",ctime(0x3e648));
      v2 = sub_c6f0(a0,a1,v1);
      if (sub_8210(a0 - v1,&a1[v1])) {
        sub_9ee0(v2);
        sub_e360();
      }
      return (unsigned long)dat_3e604;
    }
    v3 = dcgettext(NULL,"The atexit library function failed",5);
    error(1,*__errno_location(),v3);
    return v4;
  }
  v3 = dcgettext(NULL,"Failed to initialize shared-file hash table",5);
  error(1,*__errno_location(),v3);
  return v5;
}

// Function: sub_7550 @ 0x7550
void sub_7550(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3dfe8)(sub_7370,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_7580 @ 0x7580
void sub_7580(void)
{
  return;
}

// Function: _FINI_0 @ 0x75f0
void _FINI_0(void)
{
  if (!dat_3e590) {
    if (dat_3df90)
      __cxa_finalize(dat_3e008);
    sub_7580();
    dat_3e590 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x7630
void _INIT_0(void)
{
  return;
}

// Function: sub_7640 @ 0x7640
unsigned long sub_7640(unsigned long a0) // early-return
{
  int v1; // eax
  int *v2; // rax
  char v3 [152];
  
  v1 = (*dat_3e670)(a0,v3);
  if (!v1)
    return 0;
  v2 = __errno_location();
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),*v2 == 0x28);
}

// Function: sub_76a0 @ 0x76a0
void sub_76a0(int a0)
{
  if (!(dat_3e019 & 2))
    return;
  if ((a0 != -100) && (a0 < 0))
    __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir"); // no-return
  dat_3e5f8 = a0;
  if (0 <= dat_3e010)
    return;
  if (a0 == -100) {
    dat_3e010 = 0xffffff9c;
    return;
  }
  dat_3e010 = sub_17560();
}

// Function: sub_7720 @ 0x7720
void sub_7720(void)
{
  if ((dat_3e019 & 2) && (0 <= dat_3e010)) {
    close(dat_3e010);
    dat_3e010 = 0xffffffff;
    return;
  }
}

// Function: sub_7750 @ 0x7750
char * sub_7750(unsigned int a0)
{
  switch(a0) {
    default:
      __sprintf_chk((char *)0x3e598,1,0xe,"[%d]",(unsigned long)a0);
      return (char *)0x3e598;
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

// Function: sub_7880 @ 0x7880
void sub_7880(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  dat_3e5f0 = *(unsigned long *)(a1 + 0x30);
  dat_3e5e4 = 2 <= (unsigned short)(*(short *)(a1 + 0x68) - 10U);
  dat_3e5f8 = *(unsigned int *)(a0 + 0x2c);
  v1 = sub_b5e0();
  sub_e2a0(*(unsigned long *)(a1 + 0x38),a2,v1);
  if (!dat_3e600)
    return;
  sub_1a190(a0,a1,4); // tail-call
}

// Function: sub_7900 @ 0x7900
void sub_7900(long a0,long a1,unsigned int a2)
{
  char *v1;
  char v2;
  
  if (!a1) {
    sub_24e50(1,a2,""); // tail-call
    return;
  }
  v1 = (char *)(a0 + a1);
  v2 = *v1;
  *v1 = 0;
  sub_24e50(1,a2,a0);
  *v1 = v2;
}

// Function: sub_7950 @ 0x7950
void sub_7950(long *a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  if ((*(unsigned int *)&a0[0x11] & 0xf000) != 0xa000) {
    v1 = sub_7900(*(unsigned long *)(*a0 + 0x38),*(unsigned long *)(*a0 + 0x48),dat_3e68c);
    v2 = sub_dfb0(0,a0[7]);
    error(0,0,dcgettext(NULL,"File system loop detected; %s is part of the same file system loop as %s.",5),v2,v1); // tail-call
    return;
  }
  v1 = sub_dfb0(0,a0[7]);
  error(0,0,dcgettext(NULL,"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.",5),v1); // tail-call
}

// Function: sub_7a00 @ 0x7a00
void sub_7a00(long a0,long a1)
{
  unsigned char v1;
  unsigned long v10; // stack - 0xd8
  unsigned int v11; // stack - 0xdc
  short v12;
  unsigned long v13;
  unsigned long v14; // r15
  unsigned long v15; // stack - 0xd0
  unsigned long v16; // stack - 0xc8
  unsigned long v17; // stack - 0xc0
  unsigned long v18; // stack - 0xb8
  unsigned long v19; // stack - 0xb0
  char v2; // al
  unsigned long v20; // stack - 0xa8
  unsigned long v21; // stack - 0xa0
  unsigned long v22; // stack - 0x98
  unsigned long v23; // stack - 0x90
  unsigned long v24; // stack - 0x88
  unsigned long v25; // stack - 0x80
  unsigned long v26; // stack - 0x78
  unsigned long v27; // stack - 0x70
  unsigned long v28; // stack - 0x68
  unsigned long v29; // stack - 0x60
  short v3;
  unsigned long v30; // stack - 0x58
  unsigned long v31; // stack - 0x50
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned int v8;
  unsigned long v9; // rbx
  
  if (dat_3e660 & 4) {
    v6 = sub_24e50(2,dat_3e68c,*(unsigned long *)(a1 + 0x30));
    v7 = sub_24e50(1,dat_3e68c,*(unsigned long *)(a1 + 0x38));
    v13 = *(unsigned long *)(a1 + 0x58);
    v9 = (unsigned long)dat_3e014;
    v4 = sub_7750(*(unsigned short *)(a1 + 0x68));
    v5 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + 0x38));
    __fprintf_chk(stderr,1,"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",v5,v4,v13,v9,v7,v6);
  }
  if (((*(short *)(a1 + 0x68) == 6) || ((long)(int)dat_3e014 < *(long *)(a1 + 0x58))) || (!*(long *)(a1 + 0x58)))
    sub_7720();
  sub_76a0(*(unsigned int *)(a0 + 0x2c));
  v15 = *(unsigned long *)(a1 + 0x78);
  v3 = *(short *)(a1 + 0x68);
  dat_3e014 = (unsigned int)*(long *)(a1 + 0x58);
  if (v3 == 7) {
    sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38)); // return-dupe
    return;
  }
  if (v3 != 4) { // branch-flip
    if (v3 == 2) {
      sub_7950(a1);
      dat_3e604 = 1;
      return;
    }
    if (v3 == 0xd) {
      v13 = *(unsigned long *)(a1 + 0x30);
      if (sub_7640(v13)) {
        sub_e060(0x28,*(unsigned long *)(a1 + 0x38));
        return;
      }
      goto label_7e31;
    }
    if (v3 == 10) {
      if (!*(long *)(a1 + 0x58)) {
        sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
        return;
      }
      v2 = sub_7640(*(unsigned long *)(a1 + 0x30));
      if (v2) {
        sub_e060(0x28,*(unsigned long *)(a1 + 0x38));
        return;
      }
      sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
      v3 = *(short *)(a1 + 0x68); // crossjump-dupe
    }
  }
  else {
    sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
    if (dat_3e620)
      return;
label_7e31:
    v3 = *(short *)(a1 + 0x68);
  }
  if (2 <= (unsigned short)(v3 - 10U)) { // branch-flip
    v10 = *(unsigned long *)(a1 + 0x70);
    v15 = *(unsigned long *)(a1 + 0x78);
    v16 = *(unsigned long *)(a1 + 0x80);
    v17 = *(unsigned long *)(a1 + 0x88);
    dat_3e5e4 = 0x101;
    v18 = *(unsigned long *)(a1 + 0x90);
    v19 = *(unsigned long *)(a1 + 0x98);
    v20 = *(unsigned long *)(a1 + 0xa0);
    v21 = *(unsigned long *)(a1 + 0xa8);
    v22 = *(unsigned long *)(a1 + 0xb0);
    v23 = *(unsigned long *)(a1 + 0xb8);
    v24 = *(unsigned long *)(a1 + 0xc0);
    v25 = *(unsigned long *)(a1 + 200);
    v26 = *(unsigned long *)(a1 + 0xd0);
    v27 = *(unsigned long *)(a1 + 0xd8);
    v28 = *(unsigned long *)(a1 + 0xe0);
    v29 = *(unsigned long *)(a1 + 0xe8);
    v30 = *(unsigned long *)(a1 + 0xf0);
    v31 = *(unsigned long *)(a1 + 0xf8);
    v11 = (unsigned int)v17;
    dat_3e5e8 = (unsigned int)v17;
    if (!(unsigned int)v17) {
      v13 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + 0x38));
      error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v13);
      v8 = v11;
      goto label_7d35;
    }
    dat_3e5e0 = (unsigned int)*(unsigned long *)(a1 + 0x58);
label_7b66:
    v13 = *(unsigned long *)(a1 + 0x38);
    if (!sub_e150(&v11,v13,a1 + 0x100,&v10,0))
      return;
    v3 = *(short *)(a1 + 0x68);
    v12 = v3;
    if ((v11 & 0xf000) != 0x4000) goto label_7d4b;
label_7ba8:
    if (v3 == 0xb) {
      sub_1a190(a0,a1,1);
      return;
    }
    v13 = 1;
    v9 = 0;
    if (0 <= dat_3e624) {
label_7bc5:
      if ((long)dat_3e624 <= *(long *)(a1 + 0x58)) { // branch-flip
        sub_1a190(a0,a1,4);
        v12 = *(short *)(a1 + 0x68);
        v9 = (unsigned long)((long)dat_3e624 < *(long *)(a1 + 0x58));
      }
      else {
        v9 = 0;
        v12 = v3;
      }
    }
    v8 = (unsigned int)CONCAT71(dat_3e661,dat_3e660);
    v3 = v12;
    if (v12 != 1) { // branch-flip
      if ((v12 != 6) || (!dat_3e620)) goto label_7bf5;
      if (!(dat_3e660 & 4)) {
        dat_3e600 = 0;
        return;
      }
    }
    else {
      if (dat_3e620) goto label_7bf5;
      if (!(dat_3e660 & 4))
        return;
    }
label_7d91:
    v9 = 1;
label_7d96:
    v1 = dat_3e5e5;
    v14 = (unsigned long)dat_3e5e4;
    v6 = sub_7750(v12);
    v7 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + 0x38));
    __fprintf_chk(stderr,1,"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",v7,v6,v13,v9,v14,(unsigned long)v1);
  }
  else {
    if (dat_3e5e4)
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting"); // no-return
    if ((v3 != 0xb) && (dat_3e5e8))
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,"consider_visiting"); // no-return
    v11 = dat_3e5e8;
    v8 = dat_3e5e8;
label_7d35:
    dat_3e5e0 = (unsigned int)*(unsigned long *)(a1 + 0x58);
    if (v8) goto label_7b66;
    v3 = *(short *)(a1 + 0x68);
label_7d4b:
    v12 = v3;
    if (((unsigned short)(v3 - 1U) < 2) || (v3 == 6)) goto label_7ba8;
    v13 = 0;
    if (0 <= dat_3e624) goto label_7bc5;
    v8 = (unsigned int)CONCAT71(dat_3e661,dat_3e660);
    v9 = 0;
label_7bf5:
    v12 = v3;
    if (*(long *)(a1 + 0x58) < (long)dat_3e628) {
      if (!(v8 & 4)) goto label_7c26;
      goto label_7d91;
    }
    if (v8 & 4) goto label_7d96;
  }
  if (!(int)v9)
    sub_7880(a0,a1,&v10);
label_7c26:
  if (*(short *)(a1 + 0x68) == 6) {
    dat_3e600 = 0;
    return;
  }
}

// Function: sub_7fe0 @ 0x7fe0
unsigned long sub_7fe0(char *a0) // return-dupe
{
  int v1;
  unsigned long v10; // stack - 0x40
  long v2; // rax
  int *v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x48
  unsigned int v8;
  int v9; // r13d
  
  dat_3e5fc = (unsigned int)strlen(a0);
  sub_76a0(0xffffff9c);
  v10 = 0;
  if (dat_3e668 != 1) { // branch-flip
    if (dat_3e668 != 2) { // branch-flip
      v8 = dat_3e018;
      if (!dat_3e668) {
        dat_3e018 |= 0x10;
        v8 = dat_3e018;
      }
    }
    else {
      dat_3e018 |= 0x11;
      v8 = dat_3e018;
    }
  }
  else {
    dat_3e018 |= 3;
    v8 = dat_3e018;
  }
  if (dat_3e62d) {
    dat_3e018 = v8 | 0x40;
    v8 = dat_3e018;
  }
  v9 = -0x80000000;
  v7 = a0;
  v2 = sub_195e0(&v7,v8,0);
  v3 = __errno_location();
  if (!v2) {
    v5 = sub_dfb0(0,a0);
    v6 = dcgettext(NULL,"cannot search %s",5);
    error(0,*v3,v6,v5);
    dat_3e604 = 1;
    return 1;
  }
  while( true ) {
    *v3 = 0;
    v4 = sub_19b40(v2);
    if (!v4) break;
    v1 = (int)*(unsigned long *)(v4 + 0x58);
    if ((dat_3e608) && (v9 != v1)) {
      sub_d940();
      v1 = *(int *)(v4 + 0x58);
    }
    v9 = v1;
    dat_3e5e8 = *(int *)(v4 + 0x88);
    dat_3e618 = 0;
    dat_3e5e4 = 0;
    dat_3e5e5 = dat_3e5e8 != 0;
    sub_7a00(v2);
  }
  if (*v3) { // branch-flip
    a0 = (char *)sub_dfb0(0,a0);
    v6 = "failed to read file names from file system at or below %s";
  }
  else {
    if (!sub_19950(v2))
      return 1;
    v6 = dcgettext(NULL,"failed to restore working directory after searching %s",5);
  }
  error(0,*v3,v6,a0);
  dat_3e604 = 1;
  return 0;
}

// Function: sub_8210 @ 0x8210
unsigned long sub_8210(int a0,unsigned long *a1)
{
  int v1; // eax
  unsigned long v10; // rax
  int v11; // stack - 0x16c
  long v12; // stack - 0x168
  unsigned short v13; // stack - 0x42
  long v14; // stack - 0xd8
  unsigned long v15; // r12
  char *v16; // stack - 0x180
  long v17; // stack - 0x160
  long v18; // stack - 0xd0
  unsigned long v2;
  long v3; // rax
  char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  if ((1 <= a0) && (v2 = *a1, !sub_dca0(v2,1))) {
    v16 = dat_3e690;
    if (dat_3e690) {
      v2 = sub_dfb0(0,*a1);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v2);
      error(1,0,"%s",dcgettext(NULL,"file operands cannot be combined with -files0-from",5));
      return v10;
    }
    v2 = 0;
    v3 = sub_171b0(a1);
  }
  else {
    v4 = dat_3e690;
    if (!dat_3e690) {
      v13 = 0x2e;
      return (unsigned long)(unsigned int)sub_7fe0(&v13);
    }
    v1 = strcmp(dat_3e690,"-");
    if (v1) { // branch-flip
      v2 = sub_dfb0(0,v4);
      v16 = fopen(dat_3e690,"r");
      if (!v16) {
        v4 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v4,v2);
        return v8;
      }
      v1 = fileno(v16);
      if (v1 < 0)
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints"); // no-return
      if ((((dat_3e698) && (!fstat(v1,&v12))) && (!fstat(0,&v14))) && ((v17 == v18 && (v12 == v14)))) {
        error(1,0,"%s: %s\n",dcgettext(NULL,"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir",5),v2);
        return v6;
      }
      sub_174f0(v1,1);
    }
    else {
      if (dat_3e698) {
        error(1,0,"%s\n",dcgettext(NULL,"option -files0-from reading from standard input cannot be combined with -ok, -okdir",5));
        return v9;
      }
      v2 = sub_dfb0(0,dcgettext(NULL,"(standard input)",5));
      v16 = stdin;
    }
    v3 = sub_171e0(v16);
  }
  if (!v3)
    sub_26d50(); // no-return, return-dupe
  v15 = 1;
  do {
    while( true ) {
      v4 = (char *)sub_17220(v3,&v11);
      if (!v4) {
        if (v11 == 3) {
          sub_26d50();
        }
        if (v11 == 4) {
          v4 = dcgettext(NULL,"%s: read error",5);
          error(0,*__errno_location(),v4,v2);
          dat_3e604 = 1;
          sub_172e0(v3);
          return 0;
        }
        if (v11 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,"process_all_startpoints"); // no-return
        goto label_8316;
      }
      if (*v4) break;
      if (dat_3e690) { // branch-flip
        v5 = sub_172c0(v3);
        error(0,0,"%s:%lu: %s",v2,v5,dcgettext(NULL,"invalid zero-length file name",5));
      }
      else {
        error(0,2,"%s",sub_dfb0(0,v4));
      }
      dat_3e604 = 1;
      v15 = 0;
    }
    if ((!dat_3e690) && (sub_dca0(v4,1))) goto label_8316;
    dat_3e5fc = (unsigned int)strlen(v4);
  } while (sub_7fe0(v4));
  v15 = 0;
label_8316:
  sub_172e0(v3);
  if ((((char)v15) && (dat_3e690)) && ((ferror(v16) || (sub_17850(v16))))) {
    error(1,0,dcgettext(NULL,"error reading %s",5),v2);
    return v7;
  }
  return v15;
}

// Function: sub_86d0 @ 0x86d0
unsigned long sub_86d0(unsigned int *a0)
{
  *a0 = dat_3e018;
  return 1;
}

// Function: sub_86f0 @ 0x86f0
unsigned long sub_86f0(unsigned long a0,long a1)
{
  if (dat_3e620) {
    if (!dat_3e5e4)
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune"); // no-return
    if ((a1) && ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000)) {
      dat_3e600 = 1;
      return 1;
    }
  }
  return 1;
}

// Function: sub_8750 @ 0x8750
double sub_8750(unsigned long a0,long a1,unsigned long a2,long a3)
{
  double v1; // xmm0_qa
  
  difftime(a0,a2);
  return (double)(a1 - a3) * dat_2b960 + v1;
}

// Function: sub_8790 @ 0x8790
unsigned long sub_8790(long a0,long a1,long a2,long a3) // return-dupe
{
  if ((a0 == a2) && (a1 == a3))
    return 0;
  if ((double)sub_8750() < 0.0)
    return 0xffffffff;
  return 1;
}

// Function: sub_87d0 @ 0x87d0
unsigned long sub_87d0(unsigned long a0,unsigned long a1,long a2,int a3) // early-return
{
  int v1;
  unsigned long v2;
  unsigned long v3;
  double v4; // xmm0_qa
  
  v1 = *(int *)(a2 + 0x3c);
  if (v1 == 1) {
    v3 = *(unsigned long *)(a2 + 0x40);
    v2 = *(unsigned long *)(a2 + 0x48);
    return (unsigned long)((unsigned int)sub_8790(a0,a1,v3,v2) >> 0x1f);
  }
  if (v1 == 2) {
    v4 = (double)sub_8750(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    if (v4 <= dat_2b968)
      return 0;
    return (unsigned long)(v4 <= (double)a3);
  }
  if (!v1) {
    v3 = sub_8790(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),0 < (int)v3);
  }
  __assert_fail("0","pred.c",0x8a,"pred_timewindow"); // no-return
}

// Function: sub_8870 @ 0x8870
void sub_8870(unsigned long a0,unsigned long a1,long a2)
{
  long v1;
  
  v1 = *(long *)(a2 + 0x110);
  if (v1) {
    if (!sub_e2a0(a0,a1,v1))
      return;
  }
  sub_e2a0(a0,a1,*(unsigned long *)(a2 + 0x118)); // tail-call
}

// Function: sub_88c0 @ 0x88c0
void sub_88c0(unsigned long a0,unsigned long a1,long a2)
{
  long v1;
  
  v1 = *(long *)(a2 + 0x110);
  if (v1) {
    if (sub_e2a0(a0,a1,v1))
      return;
  }
  sub_e2a0(a0,a1,*(unsigned long *)(a2 + 0x118)); // tail-call
}

// Function: sub_8910 @ 0x8910
unsigned int sub_8910(int a0)
{
  int v1; // eax
  
  v1 = unlinkat(dat_3e5f8,dat_3e5f0,a0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_8940 @ 0x8940
int sub_8940(unsigned long a0,char *a1,int a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  int v4; // eax
  char *v5; // rax
  
  v5 = (char *)sub_176f0(a0);
  v3 = v5;
  sub_17810(v5);
  v4 = fnmatch(a1,v5,a2);
  v2 = v4;
  free(v3);
  v1 = (unsigned int)v4 >> 8;
  v4 = CONCAT31((undefined3)v1,v2 == 0);
  return v4;
}

// Function: sub_8990 @ 0x8990
void sub_8990(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  char *v2; // rax
  
  sub_17bc0(stdout);
  v1 = __fprintf_chk(stderr,1,dcgettext(NULL,"< %s ... %s > ? ",5),a0,a1);
  if (0 <= v1) {
    sub_17bc0(stderr);
    sub_276e0(); // tail-call
    return;
  }
  v2 = dcgettext(NULL,"Failed to write prompt for -ok",5);
  error(1,*__errno_location(),v2);
}

// Function: sub_8a20 @ 0x8a20
void sub_8a20(void)
{
  sub_e360();
  exit(dat_3e604); // no-return
}

// Function: sub_8a40 @ 0x8a40
char * sub_8a40(char *a0,char *a1)
{
  char *v1;
  int v2; // eax
  int v3; // ecx
  char *v4;
  
  if (!a0)
    return NULL;
  strcpy(a1,a0);
  v2 = (int)strlen(a1);
  v3 = v2 + -1;
  if (0 <= v3) { // branch-flip
    v1 = &a1[v3];
    do {
      v4 = v1;
      if ((*v4 != ' ') && (*v4 != '\t')) {
        v4 = &a1[v3 + 1];
        break;
      }
      v3 -= 1;
      v1 = &v4[-1];
    } while (v3 != -1);
  }
  else {
    v4 = &a1[v2];
  }
  *v4 = '\0';
  return a1;
}

// Function: sub_8ac0 @ 0x8ac0
unsigned long sub_8ac0(void)
{
  return 1;
}

// Function: sub_8ad0 @ 0x8ad0
bool sub_8ad0(unsigned long a0,unsigned int a1,long a2,unsigned int a3)
{
  char *v1;
  char *v2; // rax
  bool v3; // r12b
  
  v3 = 0;
  if ((a1 & 0xf000) != 0xa000)
    return 0;
  v2 = (char *)sub_17190(dat_3e5f8,dat_3e5f0);
  if (v2) { // branch-flip
    v1 = *(char **)(a2 + 0x38);
    v3 = fnmatch(v1,v2,(a3 & 0xff) << 4) == 0;
  }
  else {
    sub_e060(*__errno_location(),a0);
    dat_3e604 = 1;
  }
  free(v2);
  return v3;
}

// Function: sub_8b70 @ 0x8b70
void sub_8b70(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c40(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x3c); // tail-call
}

// Function: sub_8ba0 @ 0x8ba0
unsigned long sub_8ba0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (!*(int *)(a2 + 0x3c)) {
    v2 = sub_25c40(a1);
    v1 = sub_8790(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
  }
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer"); // no-return
}

// Function: sub_8c00 @ 0x8c00
void sub_8c00(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c40(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x15180); // tail-call
}

// Function: sub_8c30 @ 0x8c30
unsigned long sub_8c30(void)
{
  return 1;
}

// Function: sub_8c40 @ 0x8c40
void sub_8c40(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c50(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x3c); // tail-call
}

// Function: sub_8c70 @ 0x8c70
unsigned long sub_8c70(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (!*(int *)(a2 + 0x3c)) {
    v2 = sub_25c50(a1);
    v1 = sub_8790(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
  }
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer"); // no-return
}

// Function: sub_8cd0 @ 0x8cd0
void sub_8cd0(unsigned long a0,unsigned long a1,long a2)
{
  if (*(long *)(a2 + 0x110))
    sub_e2a0(a0,a1,*(long *)(a2 + 0x110));
  sub_e2a0(a0,a1,*(unsigned long *)(a2 + 0x118)); // tail-call
}

// Function: sub_8d10 @ 0x8d10
void sub_8d10(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c50(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x15180); // tail-call
}

// Function: sub_8d40 @ 0x8d40
unsigned int sub_8d40(unsigned long a0,long a1) // return-dupe
{
  int v1; // eax
  unsigned int v2; // eax
  int *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  
  v1 = strcmp(dat_3e5f0,".");
  if (!v1)
    return 1;
  v1 = 0;
  if (dat_3e5e4)
    v1 = (unsigned int)((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000) << 9;
  v2 = sub_8910(v1);
  if ((char)v2)
    return 1;
  v3 = __errno_location();
  if (*v3 != 2) { // branch-flip
    if (((*v3 == 0x15) && (!v1)) && (sub_8910(0x200)))
      return 1;
  }
  else if (dat_3e62e) {
    *v3 = 0;
    return (unsigned int)dat_3e62e;
  }
  v4 = sub_dfb0(0,a0);
  v5 = dcgettext(NULL,"cannot delete %s",5);
  error(0,*v3,v5,v4);
  dat_3e604 = 1;
  return v2;
}

// Function: sub_8e50 @ 0x8e50
bool sub_8e50(unsigned long a0,long a1)
{
  unsigned int v1; // eax
  int v2; // eax
  int *v3; // rax
  void *v4; // rax
  void *v5; // rax
  unsigned long v6; // rax
  bool v7; // r12b
  
  v1 = *(unsigned int *)(a1 + 0x18) & 0xf000;
  if (v1 != 0x4000) {
    if (v1 != 0x8000)
      return 0;
    return *(long *)(a1 + 0x30) == 0;
  }
  v3 = __errno_location();
  *v3 = 0;
  v2 = sub_1e690(dat_3e5f8,dat_3e5f0,0x90900);
  if (0 <= v2) {
    v4 = fdopendir(v2);
    if (!v4) {
      v6 = sub_dfb0(0,a0);
      error(0,*v3,"%s",v6);
      dat_3e604 = 1;
      close(v2);
      return 0;
    }
    *v3 = 0;
    do {
      v5 = readdir(v4);
      if (!v5) {
        v7 = 1;
        goto label_8f03;
      }
    } while ((*(char *)((long)v5 + 0x13) == '.') && ((!*(char *)((long)v5 + 0x14) || (*(short *)((long)v5 + 0x14) == 0x2e))));
    v7 = 0;
label_8f03:
    if (*v3) {
      v6 = sub_dfb0(0,a0);
      error(0,*v3,"%s",v6);
      dat_3e604 = 1;
      closedir(v4);
      return 0;
    }
    if (!closedir(v4))
      return v7;
  }
  v6 = sub_dfb0(0,a0);
  error(0,*v3,"%s",v6);
  dat_3e604 = 1;
  return 0;
}

// Function: sub_8fe0 @ 0x8fe0
void sub_8fe0(void)
{
  sub_a390(); // tail-call
}

// Function: sub_8ff0 @ 0x8ff0
void sub_8ff0(void)
{
  sub_a390(dat_3e5f0); // tail-call
}

// Function: sub_9000 @ 0x9000
unsigned long sub_9000(void)
{
  return 0;
}

// Function: sub_9010 @ 0x9010
unsigned long sub_9010(unsigned long a0,unsigned long a1,long a2)
{
  sub_16170(a0,dat_3e5f8,dat_3e5f0,a1,dat_3e638,dat_3e65c,*(char *)(a2 + 0x28),*(unsigned long *)(a2 + 0x40));
  return 1;
}

// Function: sub_9050 @ 0x9050
unsigned long sub_9050(unsigned long a0,unsigned long a1,long a2)
{
  sub_16960(*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x58),*(char *)(a2 + 0x50),"%s\n",a0);
  return 1;
}

// Function: sub_9090 @ 0x9090
unsigned long sub_9090(char *a0,unsigned long a1,long a2)
{
  void *v1;
  
  v1 = *(void **)(a2 + 0x40);
  fputs(a0,v1);
  putc(0,v1);
  return 1;
}

// Function: sub_90c0 @ 0x90c0
unsigned int sub_90c0(unsigned long a0,unsigned long a1,long a2)
{
  char *v1;
  int v2; // eax
  
  v1 = *(char **)(a2 + 0x38);
  v2 = strcmp((char *)sub_104c0(a1,a0),v1);
  return CONCAT31((undefined3)((unsigned int)v2 >> 8),v2 == 0);
}

// Function: sub_90f0 @ 0x90f0
unsigned long sub_90f0(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x20) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < (unsigned long)*(unsigned int *)(a1 + 0x20));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x20) == *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
}

// Function: sub_9140 @ 0x9140
unsigned int sub_9140(unsigned long a0,long a1,long a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x20) >> 8),*(int *)(a2 + 0x38) == *(int *)(a1 + 0x20));
}

// Function: sub_9150 @ 0x9150
void sub_9150(unsigned long a0,long a1,unsigned long a2)
{
  sub_8ad0(a0,*(unsigned int *)(a1 + 0x18),a2,1); // tail-call
}

// Function: sub_9170 @ 0x9170
void sub_9170(unsigned long a0,unsigned long a1,long a2)
{
  sub_8940(a0,*(unsigned long *)(a2 + 0x38),0x10); // tail-call
}

// Function: sub_9190 @ 0x9190
unsigned long sub_9190(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)(a1 + 8) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < *(unsigned long *)(a1 + 8));
  }
  return CONCAT71(v2,*(long *)(a1 + 8) == *(long *)(a2 + 0x40)) & 0xffffffff;
}

// Function: sub_91e0 @ 0x91e0
unsigned int sub_91e0(char *a0,unsigned long a1,long a2)
{
  int v1; // eax
  
  v1 = fnmatch(*(char **)(a2 + 0x38),a0,0x10);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_9210 @ 0x9210
unsigned long sub_9210(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)(a1 + 0x10) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < *(unsigned long *)(a1 + 0x10));
  }
  return CONCAT71(v2,*(long *)(a1 + 0x10) == *(long *)(a2 + 0x40)) & 0xffffffff;
}

// Function: sub_9260 @ 0x9260
void sub_9260(unsigned long a0,long a1,unsigned long a2)
{
  sub_8ad0(a0,*(unsigned int *)(a1 + 0x18),a2,0); // tail-call
}

// Function: sub_9270 @ 0x9270
void sub_9270(void)
{
  sub_9010(); // tail-call
}

// Function: sub_9280 @ 0x9280
void sub_9280(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c60(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x3c); // tail-call
}

// Function: sub_92b0 @ 0x92b0
void sub_92b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c60(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x15180); // tail-call
}

// Function: sub_92e0 @ 0x92e0
void sub_92e0(unsigned long a0,unsigned long a1,long a2)
{
  sub_8940(a0,*(unsigned long *)(a2 + 0x38),0); // tail-call
}

// Function: sub_92f0 @ 0x92f0
unsigned int sub_92f0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a2 + 0x118);
  return sub_e2a0(a0,a1,v1) ^ 1;
}

// Function: sub_9310 @ 0x9310
unsigned long sub_9310(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (!*(int *)(a2 + 0x3c)) {
    v2 = sub_25c60(a1);
    v1 = sub_8790(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
  }
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer"); // no-return
}

// Function: sub_9370 @ 0x9370
unsigned long sub_9370(unsigned long a0,unsigned long a1,long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // rdx
  char v3 [16];
  
  if (*(int *)(a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY"); // no-return
  switch(*(unsigned int *)(a2 + 0x38)) {
    case 0:
      v3 = sub_25c40(a1);
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      break;
    case 1:
      v3 = sub_25c70();
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      if (v2 <= -1) {
        v1 = sub_dfb0(0,a0);
        error(0,0,dcgettext(NULL,"WARNING: cannot determine birth time of file %s",5),v1);
        return 0;
      }
      break;
    case 2:
      v3 = sub_25c50(a1);
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      break;
    case 3:
      v3 = sub_25c60(a1);
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      break;
    case 4:
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY"); // no-return
    default:
      __assert_fail("collected","pred.c",0x29b,"pred_newerXY"); // no-return
    
  }
  v1 = sub_8790(v1,v2,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}

// Function: sub_94a0 @ 0x94a0
unsigned long sub_94a0(unsigned long a0,long a1)
{
  void *v1; // rax
  
  v1 = getgrgid(*(unsigned int *)(a1 + 0x20));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}

// Function: sub_94c0 @ 0x94c0
unsigned long sub_94c0(unsigned long a0,long a1)
{
  void *v1; // rax
  
  v1 = getpwuid(*(unsigned int *)(a1 + 0x1c));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}

// Function: sub_94e0 @ 0x94e0
unsigned long sub_94e0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = **(unsigned long **)(a2 + 0xe8);
  if (!sub_8990(v1,a0))
    return 0;
  return sub_a390(a0,a1,a2); // tail-call
}

// Function: sub_9530 @ 0x9530
unsigned long sub_9530(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = **(unsigned long **)(a2 + 0xe8);
  if (!sub_8990(v1,a0))
    return 0;
  v1 = sub_a390(dat_3e5f0,a1,a2); // tail-call
  return v1;
}

// Function: sub_9590 @ 0x9590
void sub_9590(void)
{
  sub_8ac0(); // tail-call
}

// Function: sub_95a0 @ 0x95a0
unsigned int sub_95a0(char *a0,unsigned long a1,long a2)
{
  int v1; // eax
  
  v1 = fnmatch(*(char **)(a2 + 0x38),a0,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_95c0 @ 0x95c0
unsigned long sub_95c0(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned int v1;
  unsigned int v2;
  int v3;
  undefined7 v4; // r8
  
  v1 = *(unsigned int *)(a1 + 0x18);
  v2 = *(unsigned int *)(a2 + 0x3c + (unsigned long)((v1 & 0xf000) == 0x4000) * 4);
  v3 = *(int *)(a2 + 0x38);
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

// Function: sub_9640 @ 0x9640
unsigned int sub_9640(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_3e5f8,dat_3e5f0,1,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_9670 @ 0x9670
unsigned int sub_9670(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_3e5f8,dat_3e5f0,4,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_96a0 @ 0x96a0
unsigned int sub_96a0(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_3e5f8,dat_3e5f0,2,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_96d0 @ 0x96d0
void sub_96d0(void)
{
  sub_9050(); // tail-call
}

// Function: sub_96e0 @ 0x96e0
void sub_96e0(void)
{
  sub_9090(); // tail-call
}

// Function: sub_96f0 @ 0x96f0
unsigned long sub_96f0(char *a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  v1 = strlen(a0);
  v2 = re_match(*(unsigned long *)(a2 + 0x38),a0,v1 & 0xffffffff,0,0);
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),(int)v2 == (int)v1);
}

// Function: sub_9730 @ 0x9730
unsigned long sub_9730(unsigned long a0,long a1,long a2)
{
  int v1;
  unsigned long v2; // rax
  undefined7 v3; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (*(long *)(a1 + 0x30) / (long)*(int *)(a2 + 0x3c) + 1) - (unsigned long)(*(long *)(a1 + 0x30) % (long)*(int *)(a2 + 0x3c) == 0);
  v3 = (undefined7)((unsigned long)a1 >> 8);
  if (v1 == 1)
    return CONCAT71(v3,v2 < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < v2);
  }
  return CONCAT71(v3,*(unsigned long *)(a2 + 0x40) == v2) & 0xffffffff;
}

// Function: sub_97a0 @ 0x97a0
unsigned long sub_97a0(unsigned long a0,long *a1,long a2)
{
  unsigned long v1; // rax
  
  if (((!a1[1]) || (a1[1] == *(long *)(a2 + 0x38))) && (v1 = sub_e080(a0,dat_3e5f0,a1), !(int)v1)) {
    if (a1[1] != *(long *)(a2 + 0x38))
      return v1;
    return CONCAT71((undefined7)((unsigned long)*(long *)(a2 + 0x40) >> 8),*a1 == *(long *)(a2 + 0x40));
  }
  return 0;
}

// Function: sub_9800 @ 0x9800
void sub_9800(void)
{
  sub_8ac0(); // tail-call
}

// Function: sub_9810 @ 0x9810
char sub_9810(unsigned long a0,long a1,long a2)
{
  unsigned int v1;
  long v2; // rcx
  char v3;
  
  if (!dat_3e5e5)
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type"); // no-return
  v3 = 0;
  if (dat_3e5e8) {
    v1 = dat_3e5e8;
    if (dat_3e5e4)
      v1 = *(unsigned int *)(a1 + 0x18);
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

// Function: sub_98f0 @ 0x98f0
unsigned long sub_98f0(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x1c) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < (unsigned long)*(unsigned int *)(a1 + 0x1c));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x1c) == *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
}

// Function: sub_9940 @ 0x9940
unsigned long sub_9940(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rbx
  long v2; // rdi
  char v3 [16];
  char v4 [16];
  
  v3 = sub_25c40(a1);
  v4 = sub_25c50(a1);
  if (0 <= (int)sub_8790(SUB168(v3,0),SUB168(v3,8),SUB168(v4,0),SUB168(v4,8))) {
    v2 = SUB168(v4,0) - SUB168(v3,0);
    v1 = SUB168(v4,8) - SUB168(v3,8);
    if (v1 < 0) {
      v1 += 1000000000;
      v2 -= 1;
    }
    return sub_87d0(v2,v1,a2,0x15180); // tail-call
  }
  return 0;
}

// Function: sub_99d0 @ 0x99d0
unsigned int sub_99d0(unsigned long a0,long a1,long a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x1c) >> 8),*(int *)(a2 + 0x38) == *(int *)(a1 + 0x1c));
}

// Function: sub_99e0 @ 0x99e0
int * sub_99e0(unsigned long a0,unsigned long a1,unsigned long a2) // early-return, ternary
{
  char v1;
  int *v2;
  char v3; // al
  int v4; // eax
  int *v5; // rax
  void *v6; // rbx
  char v7 [152];
  
  v3 = sub_dc30();
  v6 = (v3) ? sub_db20 : sub_daa0; // branch-flip
  sub_d860(v7);
  v4 = (*v6)(dat_3e5f0,v7);
  if (v4) { // branch-flip
    v3 = sub_dc30();
    v1 = v3;
    v5 = __errno_location();
    v2 = v5;
    if ((!v1) || (*v5 != 2)) {
      v5 = (unsigned long)sub_dfb0(0,a0);
      error(0,*v2,"%s",v5);
      v5 = NULL;
      dat_3e604 = 1;
      return v5;
    }
    v5 = (unsigned long)sub_9810(a0,a1,a2);
  }
  else {
    v5 = (unsigned long)sub_9810(a0,v7,a2);
  }
  return v5;
}

// Function: sub_9af0 @ 0x9af0
unsigned long sub_9af0(unsigned long a0,unsigned long a1,long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // stack - 0x38
  unsigned long v5; // r12
  unsigned long v6; // r12
  
  v1 = (*dat_3e680)(dat_3e5f8,dat_3e5f0,&v4);
  if (0 <= v1) { // branch-flip
    v1 = fnmatch(*(char **)(a2 + 0x38),v4,0);
    v6 = CONCAT71((undefined7)((unsigned long)v5 >> 8),v1 == 0);
    freecon(v4);
  }
  else {
    v2 = sub_dfb0(0,a0);
    v3 = dcgettext(NULL,"getfilecon failed: %s",5);
    v6 = 0;
    error(0,*__errno_location(),v3,v2);
  }
  return v6 & 0xffffffff;
}

// Function: sub_9bc0 @ 0x9bc0
void sub_9bc0(void *a0,long a1)
{
  unsigned long v1;
  char v2 [264];
  
  if (a1) {
    do {
      v1 = *(unsigned long *)(a1 + 8);
      __fprintf_chk(a0,1,"[%s] ",sub_8a40(v1,v2));
      a1 = *(long *)(a1 + 0x108);
    } while (a1);
  }
  __fprintf_chk(a0,1,"\n");
}

// Function: sub_9c70 @ 0x9c70
void sub_9c70(void *a0,long a1)
{
  char *v1; // rcx
  char *v2; // r8
  char *v3; // r9
  
  if (a1) {
    sub_9e10(a0,*(unsigned long *)(a1 + 0x110));
    v3 = "[need inum] ";
    if (!*(char *)(a1 + 0x1c))
      v3 = "";
    v2 = "[need type] ";
    if (!*(char *)(a1 + 0x1b))
      v2 = "";
    v1 = "[call stat] ";
    if (!*(char *)(a1 + 0x1a))
      v1 = "";
    __fprintf_chk(a0,1,"%s%s%s",v1,v2,v3);
    sub_b5a0(a0,a1);
    __fprintf_chk(a0,1," [est success rate %.4g] ");
    if (dat_3e660 & 0x40) {
      __fprintf_chk(a0,1,"[real success rate %lu/%lu",*(unsigned long *)(a1 + 0x128),*(unsigned long *)(a1 + 0x120));
      if (*(long *)(a1 + 0x120)) // branch-flip
        __fprintf_chk(a0,1,"=%.4g] ");
      else {
        __fprintf_chk(a0,1,"=_] ");
      }
    }
    sub_9e10(a0,*(unsigned long *)(a1 + 0x118)); // tail-call
    return;
  }
}

// Function: sub_9e10 @ 0x9e10
void sub_9e10(void *a0,long *a1)
{
  if (!a1)
    return;
  while( true ) {
    if (((void *)*a1 != sub_88c0) && ((void *)*a1 != sub_8870)) break;
    if (a1[0x22]) goto label_9e5e;
    a1 = (long *)a1[0x23];
    if (!a1)
      return;
  }
  if ((!a1[0x22]) && (!a1[0x23])) {
    sub_9c70(a0,a1); // tail-call
    return;
  }
label_9e5e:
  __fprintf_chk(a0,1,"%s"," ( ");
  sub_9c70(a0,a1);
  __fprintf_chk(a0,1,"%s"," ) "); // tail-call
  return;
}

// Function: sub_9ee0 @ 0x9ee0
void sub_9ee0(unsigned long a0)
{
  if (!(dat_3e660 & 0x40))
    return;
  __fprintf_chk(stderr,1,"Predicate success rates after completion:\n");
  sub_9c70(stderr,a0);
  __fprintf_chk(stderr,1,"\n"); // tail-call
}

// Function: sub_9f40 @ 0x9f40
void sub_9f40(long *a0)
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
        if (((v1 != sub_86f0) && (v1 != sub_8a20)) && (!*(char *)((long)a0 + 0x19)))
          __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check"); // no-return
      
    }
    a0 = (long *)a0[0x21];
    if (!a0)
      return;
  } while( true );
}

// Function: sub_a120 @ 0xa120
bool sub_a120(long a0,unsigned int a1,unsigned long a2)
{
  int v1;
  unsigned int *v2; // rax
  
  v2 = (unsigned int *)sub_26990(0x10);
  *(unsigned int **)(a0 + 0xc0) = v2;
  *(unsigned long *)&v2[2] = 0;
  *v2 = sub_1e690(a1,a2,0);
  v1 = **(int **)(a0 + 0xc0);
  if (0 <= v1)
    sub_174f0(v1,1);
  return 0 <= v1;
}

// Function: sub_a190 @ 0xa190
unsigned long sub_a190(long a0)
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  void *v4; // rax
  unsigned long v5; // rax
  
  v1 = dat_3e5f0;
  if (*(int *)(a0 + 0x98))
    return 1;
  v3 = strchr(dat_3e5f0,0x2f);
  if (v3) {
    v4 = (void *)sub_177a0(v1);
    v2 = sub_a120(a0,dat_3e5f8,v4);
    free(v4);
    return (unsigned long)v2;
  }
  v5 = sub_a120(a0,dat_3e5f8,"."); // tail-call
  return v5;
}

// Function: sub_a220 @ 0xa220
unsigned int sub_a220(unsigned int a0,int *a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x3a
  unsigned int v4;
  char *v5;
  char *v6;
  unsigned short v7; // stack - 0x32
  
  v4 = 1;
  if ((char)a0) {
    v3 = 0x6c756e2f7665642f;
    v7 = 0x6c;
    if (0 <= close(0)) { // branch-flip
      v4 = a0;
      if ((int)sub_17b20(&v3,0) < 0) {
        v2 = sub_dfb0(0,&v3);
        error(0,*__errno_location(),"%s",v2);
      }
    }
    else {
      v4 = 0;
      v5 = dcgettext(NULL,"Cannot close standard input",5);
      error(0,*__errno_location(),v5);
    }
  }
  if (sub_25250(a1)) {
    v5 = "";
    v6 = "";
    if (*a1 < 0) {
      v5 = *(char **)&a1[2];
      v6 = ": ";
      if (!v5) {
        v5 = "";
        v6 = v5;
      }
    }
    v1 = dcgettext(NULL,"Failed to change directory%s%s",5);
    v4 = 0;
    error(0,*__errno_location(),v1,v6,v5);
  }
  return v4;
}

// Function: sub_a390 @ 0xa390
unsigned long sub_a390(char *a0,unsigned long a1,unsigned long *a2)
{
  unsigned long *v1;
  char *v10; // stack - 0x50
  unsigned char v11; // stack - 0x41
  char *v12; // stack - 0x40
  unsigned long *v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5;
  unsigned long v6; // rax
  long v7; // rbx
  char v8;
  bool v9; // zf
  
  v11 = sub_df90(*a2);
  if (v11) { // branch-flip
    if (!sub_a190(&a2[7])) {
      v4 = sub_dfb0(0,a0);
      v5 = dcgettext(NULL,"Failed to save working directory in order to run a command on %s",5);
      error(1,*__errno_location(),v5,v4);
      return v6;
    }
    a0 = (char *)sub_176f0(dat_3e5f0);
    v12 = a0;
    v8 = (*a0 != '/') * '\x02';
    v10 = NULL;
    if (*a0 != '/')
      v10 = "./";
  }
  else {
    if (a2[0x1f] != dat_3e5c0)
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec"); // no-return
    v10 = NULL;
    v8 = '\0';
    v12 = NULL;
  }
  v3 = (unsigned long)*(unsigned char *)&a2[7];
  v2 = &a2[0x13];
  v1 = &a2[8];
  if (*(unsigned char *)&a2[7]) { // branch-flip
    sub_15000(v1,v2,a0,strlen(a0) + 1,v10,v8,0);
    if (*(int *)&a2[0x1a])
      dat_3e608 = 1;
    if (!v12) {
      return v3 & 0xffffffff; // return-dupe
    }
    if (!v11)
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec"); // return-dupe, no-return
  }
  else {
    v7 = 0;
    if (1 <= *(int *)&a2[0x1e]) {
      do {
        v3 = strlen(a0);
        v5 = *(char **)(a2[0x1d] + v7 * 8);
        v7 += 1;
        sub_15290(v1,v2,v5,strlen(v5),v10,v8,a0,v3,0);
      } while ((int)v7 < *(int *)&a2[0x1e]);
    }
    sub_14e90(v1,v2);
    v9 = !(*(unsigned int *)&a2[0x20] >> 8 & 0xff) && !(*(unsigned int *)&a2[0x20] & 0x7f);
    v3 = CONCAT71((undefined7)((unsigned long)v7 >> 8),v9);
    if (!v11) {
      if (v12) {
        __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
      }
      return v3 & 0xffffffff;
    }
    sub_25280(a2[0x1f]);
    if (!v12) {
      return v3 & 0xffffffff;
    }
    v11 = v9;
  }
  free(v12);
  v3 = (unsigned long)v11;
  return v3 & 0xffffffff;
}

// Function: sub_a610 @ 0xa610
int * sub_a610(unsigned long a0,char *a1,unsigned long a2,unsigned long *a3) // return-dupe x2
{
  long v1;
  unsigned int v10;
  unsigned long v11; // rbx
  char v2;
  int v3;
  int *v4;
  unsigned long v5;
  char *v6;
  char v7; // al
  int v8; // eax
  int *v9; // rax
  
  if (dat_3e660 & 0x20) {
    v11 = 0;
    __fprintf_chk(stderr,1,"DebugExec: launching process (argc=%lu):",*(long *)&a1[0x60] + -1);
    if (*(long *)&a1[0x60] != 1) {
      do {
        v1 = v11 * 8;
        v11 += 1;
        v9 = (unsigned long)sub_dfb0(0,*(unsigned long *)(*(long *)&a1[0x68] + v1));
        __fprintf_chk(stderr,1," %s",v9);
        v9 = (unsigned long)(*(long *)&a1[0x60] - 1);
      } while (v11 < v9);
    }
    __fprintf_chk(stderr,1,"\n");
  }
  sub_17bc0(stdout);
  sub_17bc0(stderr);
  if (dat_3e01c) {
    dat_3e01c = 0;
    signal(0x11,0);
  }
  v8 = fork();
  v3 = v8;
  if (v8 != -1) {
    if (!v8) {
      if (!*(long *)&a1[0xc0])
        __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch"); // no-return
      v7 = sub_a220(a1[0xbc]);
      if (v7) {
        v7 = sub_15dd0();
        if (v7)
          sub_15df0();
        v7 = sub_15470(a3);
        v2 = v7;
        v9 = __errno_location();
        v4 = v9;
        if (v2) // branch-flip
          *v9 = 7;
        else {
          execvp(*a3,a3);
        }
        v9 = (unsigned long)sub_dfb0(0,*a3);
        error(0,*v4,"%s",v9);
      }
      _exit(1); // no-return
    }
    while (v8 = waitpid(v3,(int *)&a1[200],0), v8 == -1) {
      v9 = __errno_location();
      v4 = v9;
      if (*v9 != 4) {
        v9 = (unsigned long)sub_dfb0(0,*a3);
        v5 = v9;
        v9 = (char *)dcgettext(NULL,"error waiting for %s",5);
        error(0,*v4,(char *)v9,v5);
        v9 = NULL;
        dat_3e604 = 1;
        return v9;
      }
    }
    v10 = *(unsigned int *)&a1[200];
    v7 = ((unsigned char)v10 & 0x7f) + 1;
    if ('\x02' <= v7) { // branch-flip
      v9 = (unsigned long)sub_24e50(0,dat_3e68c,*a3);
      v5 = v9;
      v9 = (char *)dcgettext(NULL,"%s terminated by signal %d",5);
      error(0,0,(char *)v9,v5,(unsigned long)(v10 & 0x7f));
    }
    else {
      v10 = v10 >> 8 & 0xff;
      if (dat_3e660 & 0x20)
        __fprintf_chk(stderr,1,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)v3,(unsigned long)v10);
      if (!v10) {
        v9 = (int *)0x1;
        return v9;
      }
    }
    if (!*a1) {
      v9 = (int *)0x1;
      return v9;
    }
    v9 = (int *)0x1;
    dat_3e604 = 1;
    return v9;
  }
  v9 = (char *)dcgettext(NULL,"cannot fork",5);
  v6 = (char *)v9;
  v9 = __errno_location();
  error(1,*v9,v6);
  return v9;
}

// Function: sub_a950 @ 0xa950
unsigned int sub_a950(long *a0,unsigned long a1)
{
  void *v1;
  unsigned char v2; // al
  unsigned int v3; // eax
  unsigned long v4; // rax
  undefined7 v5; // rsi
  
  v1 = (void *)*a0;
  v5 = (undefined7)((unsigned long)a1 >> 8);
  v3 = (unsigned int)CONCAT71((undefined7)((unsigned long)v4 >> 8),v1 == sub_92e0) | (unsigned int)CONCAT71(v5,v1 == sub_95a0) | (unsigned int)CONCAT71(v5,v1 == sub_9170);
  v2 = (unsigned char)v3 | v1 == sub_91e0;
  if (((!v2) && (((v1 != sub_8870 && v1 != sub_92f0) && dat_3e688) && v1 != sub_8cd0)) && (v1 != sub_88c0))
    return CONCAT31((undefined3)((unsigned int)(int)a0[4] >> 8),(int)a0[4] == 0);
  return CONCAT31((undefined3)(v3 >> 8),v2);
}

// Function: sub_a9e0 @ 0xa9e0
void sub_a9e0(unsigned long *a0)
{
  a0[1] = 0;
  *a0 = 0;
}

// Function: sub_a9f0 @ 0xa9f0
void sub_a9f0(long *a0,long a1,unsigned long *a2)
{
  long v1;
  
  *a2 = *(unsigned long *)(a1 + 0x110);
  v1 = a0[1];
  *(long *)(a1 + 0x110) = *a0;
  *a0 = a1;
  if (v1)
    return;
  a0[1] = a1;
}

// Function: sub_aa20 @ 0xaa20
unsigned int sub_aa20(long a0,long a1,char a2) // return-dupe, early-return
{
  float v1;
  float v2;
  
  if (*(unsigned int *)(a0 + 0x20) != *(unsigned int *)(a1 + 0x20))
    return -(unsigned int)(*(unsigned int *)(a0 + 0x20) < *(unsigned int *)(a1 + 0x20)) | 1;
  v1 = *(float *)(a0 + 0x24);
  v2 = *(float *)(a1 + 0x24);
  if (v1 == v2)
    return 0;
  if (!a2)
    return ((unsigned int)(v1 < v2) - 1) + (unsigned int)(v1 < v2);
  if (v1 < v2)
    return 0xffffffff;
  return 1;
}

// Function: sub_aa70 @ 0xaa70
unsigned long sub_aa70(long a0)
{
  unsigned long v1; // rax
  
  while( true ) {
    if (!a0)
      return 0;
    if (*(unsigned char *)(a0 + 0x18)) break;
    v1 = sub_aa70(*(unsigned long *)(a0 + 0x110));
    if ((char)v1)
      return v1;
    a0 = *(long *)(a0 + 0x118);
  }
  return (unsigned long)*(unsigned char *)(a0 + 0x18);
}

// Function: sub_aab0 @ 0xaab0
unsigned int sub_aab0(long a0)
{
  unsigned int v1;
  unsigned int v2; // eax
  
  if (a0) {
    v2 = sub_aab0(*(unsigned long *)(a0 + 0x110));
    v1 = v2;
    v2 = sub_aab0(*(unsigned long *)(a0 + 0x118));
    if (v2 <= v1)
      v2 = v1;
    if (v2 < *(unsigned int *)(a0 + 0x20))
      v2 = *(unsigned int *)(a0 + 0x20);
    return v2;
  }
  v2 = 0;
  return v2;
}

// Function: sub_ab00 @ 0xab00
void sub_ab00(long a0)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(*(long *)(a0 + 0x110) + 0x118);
  *(unsigned long *)(*(long *)(a0 + 0x110) + 0x118) = *(unsigned long *)(a0 + 0x118);
  *(unsigned long *)(a0 + 0x118) = v1;
}

// Function: sub_ab30 @ 0xab30
void sub_ab30(float a0)
{
  if (a0 <= dat_2c588)
    return;
}

// Function: sub_ab60 @ 0xab60
void sub_ab60(unsigned long a0,long a1,unsigned long *a2)
{
  *(unsigned long *)(a1 + 0x110) = *a2;
  *a2 = a0;
}

// Function: sub_ab70 @ 0xab70
unsigned int sub_ab70(long a0)
{
  unsigned int v1;
  
  v1 = dat_2c588;
  if (a0)
    v1 = *(unsigned int *)(a0 + 0x24);
  return v1;
}

// Function: sub_ab90 @ 0xab90
char * sub_ab90(int a0)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  int *v3; // rdx
  
  v2 = 0;
  v3 = (int *)0x3e020;
  do {
    if (*v3 == a0)
      return *(char **)(v2 * 0x10 + 0x3e028);
    v1 = (int)v2 + 1;
    v2 = (unsigned long)v1;
    v3 = &v3[4];
  } while (v1 != 0xb);
  return "unknown";
}

// Function: sub_abd0 @ 0xabd0
char * sub_abd0(short a0)
{
  long v1;
  long v2; // rax
  int v3; // ecx
  short v4;
  
  if (a0) {
    v4 = 1;
    v3 = 0;
    v1 = 0x3c040;
    while( true ) {
      v3 += 1;
      v2 = v1 + 0x10;
      if ((a0 == v4) || (v4 == -1)) break;
      v4 = *(short *)(v1 + 0x20);
      v1 = v2;
    }
    return *(char **)((long)v3 * 0x10 + 0x3c048);
  }
  return "no";
}

// Function: sub_ac30 @ 0xac30
char * sub_ac30(short a0)
{
  long v1;
  long v2; // rax
  int v3; // ecx
  short v4;
  
  if (a0) {
    v4 = 1;
    v3 = 0;
    v1 = 0x3c0c0;
    while( true ) {
      v3 += 1;
      v2 = v1 + 0x10;
      if ((a0 == v4) || (v4 == -1)) break;
      v4 = *(short *)(v1 + 0x20);
      v1 = v2;
    }
    return *(char **)((long)v3 * 0x10 + 0x3c0c8);
  }
  return "no";
}

// Function: sub_ac90 @ 0xac90
void sub_ac90(long a0,char a1)
{
  long v1;
  
  while( true ) {
    if ((a1) && (*(int *)(a0 + 0x10) != 3))
      __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization"); // no-return
    v1 = *(long *)(a0 + 0x110);
    if (v1) {
      if (*(int *)(v1 + 0x10) != 3)
        __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization"); // no-return
      sub_ac90(v1,0);
    }
    a0 = *(long *)(a0 + 0x118);
    if (!a0) break;
    a1 = '\0';
  }
}

// Function: sub_ad10 @ 0xad10
char * sub_ad10(long *a0,long a1,short a2)
{
  short v1;
  unsigned int v2;
  char *v3; // rax
  long v4;
  
  v4 = *a0;
  if ((!v4) || (*(int *)(v4 + 0x10) == 5)) {
    v3 = NULL;
    return v3;
  }
  while( true ) {
    if (*(int *)(v4 + 0x14) <= (int)a2) {
      v3 = (char *)a1;
      return v3;
    }
    v2 = *(unsigned int *)(v4 + 0x10);
    if (v2 == 4) break;
    if (4 < v2) {
      if (v2 == 5) {
        v3 = (char *)a1;
        return v3;
      }
      v3 = dcgettext(NULL,"oops -- invalid expression type (%d)!",5);
      error(1,0,v3,(unsigned long)v2);
      return v3;
    }
    if (v2 != 3) break;
    *(long *)(v4 + 0x110) = a1;
    a1 = *a0;
    v1 = *(short *)(a1 + 0x14);
    *a0 = *(long *)(a1 + 0x108);
    v3 = (unsigned long)sub_ae10(a0,(int)v1);
    *(char **)(a1 + 0x118) = v3;
    v4 = *a0;
    if (!v4) {
      v3 = (char *)a1;
      return v3;
    }
  }
  v3 = dcgettext(NULL,"invalid expression",5);
  error(1,0,v3);
  return v3;
}

// Function: sub_ae10 @ 0xae10
char * sub_ae10(long *a0,short a1,long a2)
{
  int v1;
  long v2;
  unsigned long v3;
  char *v4; // rax
  long *v5;
  long v6;
  
  v6 = *a0;
  if (!v6) {
    v4 = dcgettext(NULL,"invalid expression",5);
    error(1,0,v4);
    return v4;
  }
  v5 = a0;
  switch(*(unsigned int *)(v6 + 0x10)) {
    case 0:
label_b048:
      v4 = dcgettext(NULL,"invalid expression",5);
      error(1,0,v4);
label_b06c:
      v4 = dcgettext(NULL,"oops -- invalid expression type!",5);
      error(1,0,v4);
      return v4;
    case 1:
      v2 = *(long *)(v6 + 0x108);
      *a0 = v2;
      break;
    case 2:
label_aeb0:
      *v5 = *(long *)(v6 + 0x108);
      v4 = (unsigned long)sub_ae10();
      *(char **)(v6 + 0x118) = v4;
      v2 = *a0;
      break;
    case 3:
      goto label_b01c;
    case 4:
      v2 = *(long *)(v6 + 0x108);
      if ((!v2) || (*(char *)(v2 + 0x29))) {
        v3 = *(unsigned long *)(v6 + 8);
        v4 = dcgettext(NULL,"invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'",5);
        error(1,0,v4,v3);
        return v4;
      }
      v1 = *(int *)(v2 + 0x10);
      *a0 = v2;
      if (v1 != 5) {
label_af48:
        v4 = (long)sub_ae10();
        v6 = *a0;
        if ((!v6) || (*(int *)(v6 + 0x10) != 5)) {
          v4 = dcgettext(NULL,"invalid expression; I was expecting to find a \')\' somewhere but did not see one.",5);
          error(1,0,v4);
          return v4;
        }
        v2 = *(long *)(v6 + 0x108);
        *a0 = v2;
        v6 = (long)v4;
        break;
      }
      if (*(char *)(v6 + 0x29)) {
        v6 = *(long *)(v2 + 8);
        v4 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        v5 = (long *)0x1;
        error(1,0,v4,v6);
        goto label_aeb0;
      }
      goto label_aff8;
    case 5:
      if (!a2) {
        v3 = *(unsigned long *)(v6 + 8);
        v4 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        error(1,0,v4,v3);
        return v4;
      }
      if (2 <= (unsigned int)(*(int *)(a2 + 0x10) - 2U)) { // branch-flip
        if (!*(char *)(v6 + 0x29)) {
          v4 = dcgettext(NULL,"invalid expression; you have too many \')\'",5);
          error(1,0,v4);
          goto label_af48;
        }
      }
      else if (!*(char *)(v6 + 0x29)) {
        v3 = *(unsigned long *)(a2 + 8);
        v4 = dcgettext(NULL,"expected an expression between \'%s\' and \')\'",5);
        error(1,0,v4,v3);
        return v4;
      }
      v6 = *(long *)(a2 + 8);
      v4 = dcgettext(NULL,"expected an expression after \'%s\'",5);
      error(1,0,v4,v6);
      goto label_afb8;
    default:
      goto label_b06c;
    
  }
  v4 = (char *)v2;
  if ((v4) && ((int)a1 < *(int *)((long)v4 + 0x14))) {
label_afb8:
    v4 = (long)sub_ad10(a0,v6);
    v6 = (long)v4;
    if (!v4) {
      v4 = dcgettext(NULL,"invalid expression",5);
      error(1,0,v4);
      v6 = 0;
label_aff8:
      v4 = dcgettext(NULL,"invalid expression; empty parentheses are not allowed.",5);
      error(1,0,v4);
label_b01c:
      v3 = *(unsigned long *)(v6 + 8);
      v4 = dcgettext(NULL,"invalid expression; you have used a binary operator \'%s\' with nothing before it.",5);
      error(1,0,v4,v3);
      goto label_b048;
    }
  }
  v4 = (char *)v6;
  return v4;
}

// Function: sub_b160 @ 0xb160
void sub_b160(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  v1 = *a0;
  v2 = *a1;
  memcmp(&v1,&v2,8);
}

// Function: sub_b1c0 @ 0xb1c0
void sub_b1c0(long a0,int a1,long *a2)
{
  unsigned int v1;
  unsigned long *v2; // rax
  long v3; // rax
  unsigned int v4; // xmm0_da
  unsigned int v5; // xmm0_da
  
  v2 = (unsigned long *)sub_26c50(0x138);
  *(unsigned int *)&v2[2] = 3;
  *(int *)((long)v2 + 0x14) = a1;
  *(unsigned int *)&v2[4] = 0;
  if (a1 == 2) {
    v1 = *(unsigned int *)(a0 + 0x24);
    v2[1] = "-o";
    *v2 = sub_88c0;
    v3 = sub_ab30(v1);
    *(long *)(v3 + 0x118) = a0;
    *(unsigned int *)(v3 + 0x24) = v4;
    *a2 = v3;
    return;
  }
  if (a1 != 3) {
    if (a1 != 1) {
      v2[0x23] = a0;
      *a2 = (long)v2;
      return;
    }
    *a2 = (long)v2;
    v2[1] = ",";
    *v2 = sub_8cd0;
    *(unsigned int *)((long)v2 + 0x24) = 0x3f800000;
    v2[0x23] = a0;
    return;
  }
  v1 = *(unsigned int *)(a0 + 0x24);
  v2[1] = "-a";
  *v2 = sub_8870;
  v3 = sub_ab30(v1);
  *(long *)(v3 + 0x118) = a0;
  *(unsigned int *)(v3 + 0x24) = v5;
  *a2 = v3;
}

// Function: sub_b2b0 @ 0xb2b0
void sub_b2b0(long a0)
{
  *(unsigned long *)(a0 + 0x128) = 0;
  *(unsigned long *)(a0 + 0x120) = 0;
}

// Function: sub_b2d0 @ 0xb2d0
unsigned long sub_b2d0(void)
{
  long v1; // rbx
  
  v1 = 0x3e0f0;
  while( true ) {
    if ((int)sub_b160(v1,v1 + -0x10) < 0)
      return 0;
    if (0 < (int)sub_b160(v1 + -0x10,v1)) break;
    v1 += 0x10;
    if (v1 == 0x3e490)
      return 1;
  }
  __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted"); // no-return
}

// Function: sub_b350 @ 0xb350
void * sub_b350(long *a0)
{
  long v1;
  unsigned int v2;
  void *v3; // rax
  void *v4 [3]; // stack - 0x38
  unsigned int v5;
  
  v5 = 3;
  if ((!*(char *)((long)a0 + 0x1a)) && (v5 = 1, !*(char *)((long)a0 + 0x1c)))
    v5 = (unsigned int)*(unsigned char *)((long)a0 + 0x1b) * 2;
  v3 = (void *)*a0;
  if ((v3 != sub_8fe0) && (v3 != sub_8ff0)) {
    if (v3 != sub_fad0) { // branch-flip
      if (!dat_3e6b0) {
        qsort(0x3e0e0,0x3b,0x10,sub_b160);
        if (!sub_b2d0()) {
          error(1,0,"failed to sort the costlookup array");
          return v3;
        }
        dat_3e6b0 = 1;
        v3 = (void *)*a0;
      }
      v4[0] = v3;
      v3 = (long)bsearch(v4,0x3e0e0,0x3b,0x10,sub_b160);
      if (v3) // branch-flip
        v2 = *(unsigned int *)((long)v3 + 8);
      else {
        v1 = a0[1];
        v3 = (char *)dcgettext(NULL,"warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug",5);
        error(0,0,(char *)v3,v1);
        v2 = 10;
      }
    }
    else {
      v2 = *(unsigned int *)&a0[4];
    }
    if (v2 <= v5)
      v2 = v5;
  }
  else {
    v2 = 8 - (unsigned int)*(unsigned char *)&a0[7];
  }
  v3 = (unsigned long)(unsigned long)v2;
  return v3;
}

// Function: sub_b4d0 @ 0xb4d0
void sub_b4d0(long a0)
{
  if (a0) {
    sub_b4d0(*(unsigned long *)(a0 + 0x118));
    sub_b4d0(*(unsigned long *)(a0 + 0x110));
    *(unsigned int *)(a0 + 0x20) = sub_b350(a0);
    return;
  }
}

// Function: sub_b510 @ 0xb510
unsigned long sub_b510(char *a0,unsigned char a1)
{
  char *v1;
  int v2; // eax
  unsigned long v3; // rbx
  
  if (!dat_3e6b8) {
    v2 = fnmatch(a0,".",(unsigned int)a1 << 4);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
  }
  v3 = 0;
  do {
    v1 = *(char **)(dat_3e6c0 + v3 * 8);
    if (!fnmatch(a0,v1,(unsigned int)a1 << 4))
      return 1;
    v3 += 1;
  } while (v3 < dat_3e6b8);
  return 0;
}

// Function: sub_b5a0 @ 0xb5a0
void sub_b5a0(void *a0,long a1)
{
  if (*(long *)(a1 + 0x30)) {
    __fprintf_chk(a0,1,"%s %s",*(unsigned long *)(a1 + 8)); // tail-call
    return;
  }
  __fprintf_chk(a0,1,"%s",*(unsigned long *)(a1 + 8)); // tail-call
}

// Function: sub_b5e0 @ 0xb5e0
unsigned long sub_b5e0(void)
{
  return dat_3e6d0;
}

// Function: sub_b5f0 @ 0xb5f0
unsigned long sub_b5f0(long *a0)
{
  unsigned int v1;
  long v2;
  long v3;
  void *v4;
  float v5;
  unsigned long v6; // xmm0_qa
  
  if (!a0)
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates"); // no-return
  v2 = a0[0x23];
  if (v2)
    sub_b5f0(v2);
  v3 = a0[0x22];
  if (v3)
    sub_b5f0(v3);
  v1 = *(unsigned int *)&a0[2];
  if (v1 == 5)
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates"); // no-return
  if (v1 == 4)
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates"); // no-return
  if (v1 != 2) {
    if (2 < v1) {
      if (v1 != 3)
        __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates"); // no-return
      v4 = (void *)*a0;
      if (v4 != sub_8870) { // branch-flip
        if (v4 != sub_8cd0) { // branch-flip
          if (v4 != sub_88c0)
            __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates"); // no-return
          v5 = (float)sub_ab70(v2);
          v5 = (float)sub_ab70(v3) + v5;
        }
        else {
          v5 = dat_2c588;
        }
      }
      else {
        v5 = (float)sub_ab70(v2);
        v5 = (float)sub_ab70(v3) * v5;
      }
      v6 = sub_ab30(v5);
      *(int *)((long)a0 + 0x24) = (int)v6;
      return v6;
    }
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
    return (unsigned long)*(unsigned int *)((long)a0 + 0x24);
  }
  if ((void *)*a0 != sub_92f0)
    __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates"); // no-return
  if (!v3) {
    v5 = dat_2c588 - *(float *)(v2 + 0x24);
    *(float *)((long)a0 + 0x24) = v5;
    return (unsigned long)(unsigned int)v5;
  }
  __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates"); // no-return
}

// Function: sub_b8a0 @ 0xb8a0
void sub_b8a0(int *a0)
{
  long v1; // rax
  
  if (!*a0)
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred"); // no-return
  if (*a0 != 2) {
    v1 = sub_26c50(0x138);
    if (dat_3e6d8) // branch-flip
      *(long *)(dat_3e6c8 + 0x108) = v1;
    else {
      dat_3e6d8 = v1;
    }
    *(unsigned short *)(v1 + 0x1a) = 0x101;
    *(int **)(v1 + 0x130) = a0;
    *(unsigned long *)(v1 + 0x10) = 0;
    *(char **)(v1 + 0x30) = "ThisShouldBeSetToSomethingElse";
    *(char *)(v1 + 0x28) = dat_3e62f;
    *(unsigned long *)(v1 + 0x20) = 0x3f8000000000000a;
    dat_3e6c8 = v1;
    sub_b2b0(v1);
    return;
  }
  __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred"); // no-return
}

// Function: sub_b970 @ 0xb970
void sub_b970(void) // return-dupe
{
  long v1; // rax
  
  v1 = sub_b8a0();
  if (!v1)
    return;
  *(unsigned long *)(v1 + 0x30) = 0;
}

// Function: sub_b990 @ 0xb990
void sub_b990(unsigned long a0,unsigned long a1)
{
  long v1;
  unsigned long *v2; // rax
  
  v1 = dat_3e6a0;
  if (!dat_3e6a0) {
    v1 = sub_147f0("and");
    dat_3e6a0 = v1;
    if (!v1)
      __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op"); // no-return
  }
  if (dat_3e6c8) {
    if ((*(unsigned int *)(dat_3e6c8 + 0x10) & 0xfffffffb) != 1) { // branch-flip
      if (!*(unsigned int *)(dat_3e6c8 + 0x10)) {
        error(1,0,dcgettext(NULL,"oops -- invalid default insertion of and!",5));
        return;
      }
    }
    else {
      v2 = (unsigned long *)sub_b970(v1);
      v2[1] = "-a";
      *v2 = sub_8870;
      v2[6] = 0;
      v2[7] = 0;
      v2[2] = 0x300000003;
      *(unsigned int *)&v2[3] = 0;
      *(char *)((long)v2 + 0x1c) = 0;
    }
  }
  v1 = sub_b8a0(a0);
  *(unsigned long *)(v1 + 0x30) = a1;
  *(unsigned long *)(v1 + 0x130) = a0;
}

// Function: sub_baa0 @ 0xbaa0
void sub_baa0(void *a0,long a1,int a2)
{
  char v1;
  short v2;
  unsigned int v3;
  char *v4; // rcx
  int v5; // esi
  
  if (!a1)
    return;
  do {
    if (1 <= a2) {
      v5 = 0;
      do {
        __fprintf_chk(a0,1,"    ");
        v5 += 1;
      } while (a2 != v5);
    }
    __fprintf_chk(a0,1,"pred=[");
    sub_b5a0(a0,a1);
    sub_ac30((int)*(short *)(a1 + 0x14));
    v2 = *(short *)(a1 + 0x10);
    __fprintf_chk(a0,1,"] type=%s prec=%s",sub_abd0((int)v2));
    v3 = *(unsigned int *)(a1 + 0x20);
    v5 = 1;
    __fprintf_chk(a0,v5," cost=%s est_success_rate=%#.4g %sside effects ",sub_ab90(v3));
    if (*(unsigned long *)(a1 + 0x18) & 0xffffff0000) {
      __fprintf_chk(a0,1,"Needs ");
      if (*(char *)(a1 + 0x1a)) { // branch-flip
        __fprintf_chk(a0,1,"stat");
        if (*(char *)(a1 + 0x1c)) { // branch-flip
          v4 = ",";
label_bcc1:
          __fprintf_chk(a0,1,"%sinode",v4);
          v1 = *(char *)(a1 + 0x1b);
        }
        else {
          v1 = *(char *)(a1 + 0x1b);
        }
        if (!v1) goto label_bba8;
        v4 = ",";
      }
      else {
        if (*(char *)(a1 + 0x1c)) {
          v4 = "";
          goto label_bcc1;
        }
        if (!*(char *)(a1 + 0x1b)) goto label_bba8;
        v4 = "";
      }
      __fprintf_chk(a0,1,"%stype",v4);
    }
label_bba8:
    __fprintf_chk(a0,1,"\n");
    if (1 <= a2) {
      v5 = 0;
      do {
        __fprintf_chk(a0,1,"    ");
        v5 += 1;
      } while (a2 != v5);
    }
    if (*(long *)(a1 + 0x110)) { // branch-flip
      __fprintf_chk(a0,1,"left:\n");
      sub_baa0(a0,*(unsigned long *)(a1 + 0x110),a2 + 1);
    }
    else {
      if (!*(long *)(a1 + 0x118)) {
        v4 = "no children.\n";
        __fprintf_chk(a0,1,v4); // return-dupe, tail-call
        return;
      }
      __fprintf_chk(a0,1,"no left.\n");
    }
    v5 = 0;
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v5 += 1;
      } while (a2 != v5);
    }
    if (!*(long *)(a1 + 0x118)) {
      v4 = "no right.\n";
      __fprintf_chk(a0,1,v4);
      return;
    }
    a2 += 1;
    __fprintf_chk(a0,1,"right:\n");
    a1 = *(long *)(a1 + 0x118);
    if (!a1)
      return;
  } while( true );
}

// Function: sub_bdc0 @ 0xbdc0
void sub_bdc0(long *a0,unsigned long a1)
{
  long v1;
  unsigned long v2;
  long v3;
  long v4;
  long v5; // stack - 0x48
  long v6;
  long v7;
  long v8; // stack - 0x40
  
  v4 = *a0;
  if (!v4)
    return;
  if (dat_3e660 & 8) { // branch-flip
    __fprintf_chk(stderr,1,"%s:\n","predlist before merge sort");
    sub_baa0(stderr,*a0,2);
    v4 = *a0;
    sub_b5f0(v4);
    sub_a9e0(&v5);
    v7 = v5;
    if (!v4) goto label_be98;
  }
  else {
    sub_b5f0(v4);
    sub_a9e0(&v5);
  }
  v6 = v5;
  do {
    v1 = *(long *)(v4 + 0x110);
    *a0 = v1;
    *(unsigned long *)(v4 + 0x110) = 0;
    v7 = v4;
    if (v6) {
      v3 = v6;
      do {
        v2 = *(unsigned long *)(v3 + 0x118);
        if (0 <= (int)sub_aa20(v2)) {
          v4 = *(long *)(v3 + 0x110);
          *(long *)(v7 + 0x110) = v4;
          if (!v4)
            v8 = v7;
          *(long *)(v3 + 0x110) = v7;
          v7 = v6;
          goto label_be93;
        }
        v3 = *(long *)(v3 + 0x110);
      } while (v3);
    }
    *(long *)(v7 + 0x110) = v6;
    if (!v8)
      v8 = v7;
    v5 = v7;
label_be93:
    v4 = v1;
    v6 = v7;
  } while (v1);
label_be98:
  if (dat_3e660 & 8) {
    __fprintf_chk(stderr,1,"%s:\n","predlist after merge sort");
    sub_baa0(stderr,v7,2);
  }
  sub_b5f0(v7);
  sub_ab60(v7,v8,a1);
  sub_a9e0(a0); // tail-call
}

// Function: sub_bfd0 @ 0xbfd0
void sub_bfd0(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  long v1;
  long v2; // rbx
  
  dat_3e6a8 = sub_bdc0;
  sub_bdc0(a2,a4);
  (*dat_3e6a8)(a3,a4);
  v1 = a0;
  do {
    v2 = v1 + 0x10;
    (*dat_3e6a8)(v1,a4);
    v1 = v2;
  } while (v2 != a0 + 0xb0);
}

// Function: sub_c030 @ 0xc030
unsigned long sub_c030(long *a0)
{
  unsigned int v1;
  char v10 [176];
  char v11 [8];
  unsigned long v12; // stack - 0x108
  int v13;
  unsigned int v14;
  int v15;
  long *v16;
  void *v17;
  unsigned long v18;
  unsigned long *v19;
  long v2;
  unsigned long v20; // stack - 0x110
  unsigned long v21; // stack - 0x100
  char *v3;
  char *v4; // rax
  long v5;
  unsigned long v6; // rax
  char *v7;
  long v8;
  unsigned long v9; // stack - 0x118
  
  v5 = *a0;
  v9 = 0;
  v20 = 0;
  v12 = 0;
  v21 = 0;
  if (!v5)
    return 0;
  v7 = v11;
  v3 = v10;
  do {
    v4 = &v3[0x10];
    sub_a9e0(v3);
    v3 = v4;
  } while (v7 != v4);
  v8 = v5;
  v2 = *(long *)(v5 + 0x110);
  if (*(long *)(v5 + 0x110)) { // branch-flip
    while (v5 = v2, *(long *)(v5 + 0x110)) {
      v8 = v5;
      v2 = *(long *)(v5 + 0x110);
    }
    v14 = *(unsigned int *)(v8 + 0x14);
    v16 = (long *)(v8 + 0x110);
  }
  else {
    v14 = 3;
    v16 = a0;
  }
  if (*(int *)(v5 + 0x10) != 3) { // branch-flip
    sub_b1c0(v5,v14,v16);
    if (dat_3e660 & 9) goto label_c2e0;
  }
  else {
    if (!(dat_3e660 & 9)) {
      v5 = *a0;
      goto label_c0fe;
    }
label_c2e0:
    __fprintf_chk(stderr,1,"Normalized Eval Tree:\n");
    sub_baa0(stderr,*a0,0);
  }
  v5 = *a0;
  if (!v5) {
    v18 = 0;
    sub_bfd0(v10,0xb,&v12,&v9,a0); // return-dupe
    return v18;
  }
label_c0fe:
  v15 = *(int *)(v5 + 0x10);
  v13 = 0;
  if (v15 == 3)
    v13 = *(int *)(v5 + 0x14);
  v18 = 0;
  do {
    if ((v15 == 3) && (*(int *)(v5 + 0x14) != v13))
      v5 = sub_b1c0(v5,v13,a0);
    v19 = *(unsigned long **)(v5 + 0x118);
    v1 = *(unsigned int *)&v19[2];
    v17 = (void *)*v19;
    if (v1 != 2) { // branch-flip
      if (2 < v1) {
        if (v1 != 3) {
          error(1,0,dcgettext(NULL,"oops -- invalid expression type!",5));
          return v6;
        }
        *(char *)&v19[3] = sub_c030(v5 + 0x118);
        v19 = *(unsigned long **)(v5 + 0x118);
        goto label_c250;
      }
      if (v13 == 1) goto label_c250;
      if (*(char *)&v19[3]) goto label_c2a0;
      if (!sub_a950(v19)) {
        if (v17 == sub_96f0) {
          sub_a9f0(&v9,v5);
          goto label_c25e;
        }
        if ((2 <= dat_3e688) && (((v1 = *(unsigned int *)&v19[4], v1 - 1 < 2 && (!*(char *)((long)v19 + 0x1a))) || (dat_3e688 != 2)))) {
          if (dat_3e660 & 8) {
            __fprintf_chk(stderr,1,"-O%d: categorising predicate ");
            sub_b5a0(stderr,*(unsigned long *)(v5 + 0x118));
            v15 = 1;
            v14 = *(unsigned int *)(*(long *)(v5 + 0x118) + 0x20);
            __fprintf_chk(stderr,v15," by cost (%s)\n",sub_ab90(v14));
            v1 = *(unsigned int *)(*(long *)(v5 + 0x118) + 0x20);
          }
          sub_a9f0(&v10[(unsigned long)v1 * 0x10],v5);
          goto label_c25e;
        }
        goto label_c257;
      }
      if (dat_3e660 & 8) {
        __fprintf_chk(stderr,1,"-O%d: promoting cheap predicate ",(unsigned long)dat_3e688);
        sub_b5a0(stderr,*(unsigned long *)(v5 + 0x118));
        __fprintf_chk(stderr,1," into name_list\n");
      }
      sub_a9f0(&v12,v5);
    }
    else {
      *(char *)&v19[3] = sub_c030(&v19[0x23]);
      v19 = *(unsigned long **)(v5 + 0x118);
label_c250:
      if (*(char *)&v19[3]) {
label_c2a0:
        v18 = 1;
        sub_bfd0(v10,0xb,&v12,&v9,a0);
      }
label_c257:
      a0 = (long *)(v5 + 0x110);
    }
label_c25e:
    v5 = *a0;
    if (!v5) {
      sub_bfd0(v10,0xb,&v12,&v9,a0);
      return v18;
    }
    v15 = *(int *)(v5 + 0x10);
  } while( true );
}

// Function: sub_c450 @ 0xc450
unsigned int sub_c450(long *a0)
{
  float v1;
  long v10;
  float v2;
  char v3; // al
  unsigned int v4; // eax
  int v5; // eax
  int v6; // eax
  unsigned long v7; // rcx
  long v8;
  char *v9; // r8
  
  v9 = "Not a binary operation";
  v7 = (unsigned long)((unsigned int)dat_3e660 & 8);
  if ((int)a0[2] != 3) goto label_c471;
  if ((!a0[0x22]) || (v10 = a0[0x23], !v10)) {
    v9 = "Doesn\'t have two arms";
    goto label_c471;
  }
  v8 = *(long *)(a0[0x22] + 0x118);
  if (!v8) {
    v9 = "Left arm has no child on RHS";
    goto label_c471;
  }
  v3 = sub_aa70(v8);
  if (v3) {
    v9 = "Left subtree has side-effects";
    goto label_c471;
  }
  v4 = sub_aa70(v10);
  if ((char)v4) {
    v9 = "Right subtree has side-effects";
    goto label_c471;
  }
  v5 = sub_aab0(v8);
  v6 = sub_aab0(v10);
  if (v5 < v6) {
    v9 = "efficient as-is";
    goto label_c471;
  }
  if (v5 == v6) {
    v1 = *(float *)(v8 + 0x24);
    v2 = *(float *)(v10 + 0x24);
    if (v7) { // branch-flip
      __fprintf_chk(stderr,1,"Success rates: l=%f, r=%f\n");
      v7 = (unsigned long)((unsigned int)dat_3e660 & 8);
      if ((void *)*a0 == sub_88c0) {
        v9 = "Operation is OR; right success rate >= left";
        if (v1 <= v2) goto label_c471;
        goto label_c5c0;
      }
      v9 = "Not \'AND\' or \'OR\'";
      if ((void *)*a0 != sub_8870) goto label_c471;
    }
    else {
      if ((void *)*a0 == sub_88c0) {
        if (v1 <= v2)
          return v4;
        sub_ab00(a0); // return-dupe
        return 1;
      }
      if ((void *)*a0 != sub_8870)
        return v4;
    }
    if (v2 <= v1) {
      v9 = "Operation is AND; right success rate <= left";
label_c471:
      if (!v7)
        return 0;
      __fprintf_chk(stderr,1,"Not an arm swap candidate (%s):\n",v9);
      sub_baa0(stderr,a0,0);
      return 0;
    }
  }
label_c5c0:
  if (v7) {
    __fprintf_chk(stderr,1,"Performing arm swap on:\n");
    sub_baa0(stderr,a0,0);
  }
  sub_ab00(a0);
  return 1;
}

// Function: sub_c6b0 @ 0xc6b0
unsigned long sub_c6b0(long a0) // return-dupe
{
  unsigned long v1;
  
  if (!a0)
    return 0;
  do {
    do {
    } while (sub_c450(a0));
    v1 = *(unsigned long *)(a0 + 0x110);
  } while ((sub_c6b0(v1)) || (v1 = *(unsigned long *)(a0 + 0x118), sub_c6b0(v1)));
  return 0;
}

// Function: sub_c6f0 @ 0xc6f0
int * sub_c6f0(int a0,long a1,int a2)
{
  long *v1;
  long *v10; // stack - 0x48
  unsigned long v11;
  int v12;
  bool v13; // zf
  long v14; // stack - 0x70
  long v2;
  char *v3;
  long v4;
  int *v5;
  char v6; // al
  int *v7; // rax
  int v8 [4]; // stack - 0x5c
  int v9; // stack - 0x4c
  
  v7 = (long)(long)a2;
  dat_3e6d8 = NULL;
  dat_3e6c0 = a1 + (long)v7 * 8;
  v8[0] = a0;
  v9 = a2;
  if (a2 < a0) {
    while (v6 = sub_dca0(*(unsigned long *)(a1 + (long)v7 * 8),1), !v6) {
      dat_3e6b8 += 1;
      v9 += 1;
      if (v8[0] <= v9) break;
      v7 = (long)(long)v9;
    }
  }
  v7 = (long)sub_147f0("(");
  v2 = (long)v7;
  v7 = (long)sub_147f0(")");
  v14 = (long)v7;
  v7 = (long)sub_147f0("print");
  v4 = (long)v7;
  if (!v2)
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree"); // no-return
  if (!v14)
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree"); // no-return
  if (!v7)
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree"); // no-return
  sub_13ec0(v2,a1,v8);
  *(char **)(dat_3e6c8 + 8) = "(";
  *(char *)((long)dat_3e6d8 + 0x29) = 1;
  sub_147d0(a1,v8[0]);
  sub_9f40(dat_3e6c8);
  if (v9 < v8[0]) {
    do {
      v7 = (long)(long)v9;
      dat_3e618 = 0;
      v6 = sub_dca0(*(unsigned long *)(a1 + (long)v7 * 8),0);
      v13 = v6 == '\0';
      v7 = (long)(long)v9;
      v11 = *(unsigned long *)(a1 + (long)v7 * 8);
      if (v13) {
        v7 = (char *)dcgettext(NULL,"paths must precede expression: `%s\'",5);
        error(0,0,(char *)v7,v11);
        v7 = (long)(long)v9;
        v3 = *(char **)(a1 + (long)v7 * 8);
        if (!access(v3,0)) {
          v11 = *(unsigned long *)(dat_3e6c8 + 8);
          v7 = (char *)dcgettext(NULL,"possible unquoted pattern after predicate `%s\'?",5);
          error(0,0,(char *)v7,v11);
        }
        exit(1); // no-return
      }
      v7 = (int *)sub_147f0(v11);
      v5 = v7;
      if (!v7) {
        v7 = (char *)dcgettext(NULL,"unknown predicate `%s\'",5);
        error(1,0,(char *)v7,v11);
        return v7;
      }
      v12 = v9;
      if (*v7 != 4) {
        v12 = v9 + 1;
        v9 = v12;
      }
      v6 = (**(void **)&v7[4])(v7,a1,&v9);
      if (!v6) {
        v2 = *(long *)(a1 + (long)v9 * 8);
        if (!v2) {
          v7 = (char *)dcgettext(NULL,"missing argument to `%s\'",5);
          error(1,0,(char *)v7,v11);
          return v7;
        }
        if ((*v5 == 4) && (v9 == v12)) {
          v7 = (char *)dcgettext(NULL,"invalid predicate `%s\'",5);
          error(1,0,(char *)v7,v11);
          return v7;
        }
        v7 = (char *)dcgettext(NULL,"invalid argument `%s\' to `%s\'",5);
        error(1,0,(char *)v7,v2,v11);
        return v7;
      }
      v7 = NULL;
      *(unsigned long *)(dat_3e6c8 + 8) = v11;
      if (v9 != v12)
        v7 = *(int **)(a1 + (long)v12 * 8);
      *(int **)(dat_3e6c8 + 0x30) = v7;
      sub_9f40(dat_3e6c8);
      sub_9f40(dat_3e6d8);
    } while (v9 < v8[0]);
  }
  sub_147e0(a1,v8[0],dat_3e6c8,dat_3e6d8);
  v1 = dat_3e6d8;
  if (dat_3e6d8[0x21]) { // branch-flip
    v6 = sub_dc60(dat_3e6d8[0x21]);
    if (v6) { // branch-flip
      sub_13690(v14,a1,v8);
      *(char **)(dat_3e6c8 + 8) = ")";
      *(char *)(dat_3e6c8 + 0x29) = 1;
      sub_9f40(dat_3e6c8);
      sub_12910(v4,a1,v8);
      *(char **)(dat_3e6c8 + 8) = "-print";
      *(char *)(dat_3e6c8 + 0x29) = 1;
      sub_9f40(dat_3e6c8);
      sub_9f40(dat_3e6d8);
    }
    else {
      v1 = &dat_3e6d8[0x21];
      v10 = dat_3e6d8;
      dat_3e6d8 = (long *)*v1;
      sub_9f40((long *)*v1);
      free(v10);
    }
  }
  else {
    v10 = dat_3e6d8;
    dat_3e6c8 = 0;
    dat_3e6d8 = NULL;
    free(v1);
    sub_12910(v4,a1,v8);
    *(char **)(dat_3e6c8 + 8) = "-print";
    sub_9f40(dat_3e6c8);
    sub_9f40(dat_3e6d8);
  }
  if (dat_3e660 & 9) {
    __fprintf_chk(stderr,1,"Predicate List:\n");
    sub_9bc0(stderr,dat_3e6d8);
  }
  sub_14670(dat_3e6d8);
  sub_9f40(dat_3e6d8);
  v10 = dat_3e6d8;
  v7 = (unsigned long)sub_ae10(&v10,0,0);
  dat_3e6d0 = v7;
  sub_b5f0(v7);
  if (!v10) {
    if (dat_3e660 & 9) {
      __fprintf_chk(stderr,1,"Eval Tree:\n");
      sub_baa0(stderr,dat_3e6d0,0);
      v7 = (int *)dat_3e6d0;
    }
    sub_b4d0(v7);
    sub_c030(0x3e6d0);
    v11 = dat_3e6d0;
    sub_ac90(dat_3e6d0,1);
    sub_c6b0(v11);
    v11 = dat_3e6d0;
    sub_ac90(dat_3e6d0,1);
    if (dat_3e660 & 9) {
      __fprintf_chk(stderr,1,"Optimized Eval Tree:\n");
      sub_baa0(stderr,dat_3e6d0,0);
      __fprintf_chk(stderr,1,"Optimized command line:\n");
      sub_9c70(stderr,dat_3e6d0);
      __fprintf_chk(stderr,1,"\n");
      v11 = dat_3e6d0;
    }
    v7 = (int *)v11;
    return v7;
  }
  if ((void *)*v10 == sub_8c30) {
    v7 = (char *)dcgettext(NULL,"you have too many \')\'",5);
    error(1,0,(char *)v7);
    return v7;
  }
  v2 = v10[1];
  if (v2) {
    v7 = (char *)dcgettext(NULL,"unexpected extra predicate \'%s\'",5);
    error(1,0,(char *)v7,v2);
    return v7;
  }
  v7 = (char *)dcgettext(NULL,"unexpected extra predicate",5);
  error(1,0,(char *)v7);
  return v7;
}

// Function: sub_cdf0 @ 0xcdf0
void sub_cdf0(long a0,void *a1)
{
  do {
    if (*(long *)(a0 + 0x110))
      sub_cdf0(*(long *)(a0 + 0x110),a1);
    (*a1)(a0);
    a0 = *(long *)(a0 + 0x118);
  } while (a0);
}

// Function: sub_ce30 @ 0xce30
void sub_ce30(long *a0) // return-dupe
{
  void *v1;
  
  v1 = (void *)*a0;
  if (((v1 != sub_9050 && v1 != sub_fad0) && v1 != sub_9010) && (v1 != sub_9090))
    return;
  a0[8] = 0;
}

// Function: sub_ce70 @ 0xce70
void sub_ce70(int a0)
{
  void *v1;
  long v2;
  char *v3;
  unsigned long *v4; // rbx
  char *v5;
  
  v1 = stdout;
  fputs(dcgettext(NULL,"Valid arguments for -D:\n",5),v1);
  v4 = (unsigned long *)0x3c158;
  if (a0) {
    v3 = "exec";
    v5 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
    for (; __fprintf_chk(stdout,1,"%-10s %s\n",v3,v5), v4 != (unsigned long *)0x3c218; v4 = &v4[3]) {
      v5 = (char *)v4[2];
      v3 = (char *)*v4;
    }
    return;
  }
  v5 = "exec";
  v2 = 0;
  do {
    if (v2) { // branch-flip
      __fprintf_chk(stdout,1,"%s%s",", ",v5);
      if (v2 + 1 == 9)
        return;
    }
    else {
      __fprintf_chk(stdout,1,"%s%s","",v5);
    }
    v5 = (char *)*v4;
    v4 = &v4[3];
    v2 += 1;
  } while( true );
}

// Function: sub_cf70 @ 0xcf70
void sub_cf70(long a0) // return-dupe
{
  sub_154f0(*(unsigned long *)(a0 + 0xc0),sub_d060,a0);
  if (*(long *)(a0 + 0xc0) == dat_3e5c0)
    return;
  sub_25280(*(long *)(a0 + 0xc0));
  free(*(void **)(a0 + 0xc0));
  *(unsigned long *)(a0 + 0xc0) = 0;
}

// Function: sub_cfc0 @ 0xcfc0
void sub_cfc0(long *a0) // return-dupe
{
  if (!a0)
    return;
  do {
    while( true ) {
      if (!dat_3e608)
        __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs"); // no-return
      sub_cfc0(a0[0x22]);
      if (((((void *)*a0 == sub_9530) || ((void *)*a0 == sub_8ff0)) && ((char)a0[7])) && ((int)a0[0x1a])) break;
      a0 = (long *)a0[0x23];
      if (!a0)
        return;
    }
    sub_cf70(&a0[7]);
    a0 = (long *)a0[0x23];
  } while (a0);
}

// Function: sub_d060 @ 0xd060
unsigned long sub_d060(long a0)
{
  sub_14e90(a0 + 8,a0 + 0x60);
  return 0;
}

// Function: sub_d080 @ 0xd080
void sub_d080(long *a0)
{
  if (!a0)
    return;
  do {
    sub_d080(a0[0x22]);
    if ((((void *)*a0 == sub_8fe0) && ((char)a0[7])) && ((int)a0[0x1a]))
      sub_14e90(&a0[8],&a0[0x13]);
    a0 = (long *)a0[0x23];
  } while (a0);
}

// Function: sub_d0f0 @ 0xd0f0
void sub_d0f0(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  char *v4;
  
  v1 = sub_25250(dat_3e5c0);
  if (!v1) {
    sub_25280(dat_3e5c0);
    free(dat_3e5c0);
    dat_3e5c0 = NULL;
    return;
  }
  v3 = "";
  v4 = "";
  if (*dat_3e5c0 < 0) {
    v3 = *(char **)&dat_3e5c0[2];
    v4 = ": ";
    if (!v3) {
      v3 = "";
      v4 = v3;
    }
  }
  v2 = dcgettext(NULL,"Failed to restore initial working directory%s%s",5);
  error(0,*__errno_location(),v2,v4,v3);
  _exit(1); // no-return
}

// Function: sub_d1a0 @ 0xd1a0
unsigned long sub_d1a0(char *a0,void *a1,unsigned int a2)
{
  int v1;
  
  v1 = *__errno_location();
  if ((v1 != 2) && (v1 != 0x14))
    return (unsigned long)a2;
  if (dat_3e660 & 2)
    __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
  v1 = fstatat(dat_3e5f8,a0,a1,0x100); // tail-call
  return v1;
}

// Function: sub_d220 @ 0xd220
void sub_d220(unsigned char *a0)
{
  unsigned char v1;
  int v2;
  int *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned char *v6; // stack - 0x38
  
  v1 = *a0;
  if (!v1) {
    error(1,0,dcgettext(NULL,"The -O option must be immediately followed by a decimal integer",5));
    return;
  }
  if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v1 * 2) & 8)) {
    error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
    return;
  }
  v3 = __errno_location();
  v2 = *v3;
  *v3 = 0;
  v4 = strtoul((char *)a0,(char **)&v6,10);
  if (v4) { // branch-flip
    if (!*v6) {
      if (v4 != 0xffffffffffffffff) { // branch-flip
        if (v4 <= 0xffff) goto label_d295;
      }
      else if (*v3) {
        v5 = dcgettext(NULL,"Invalid optimisation level %s",5);
        error(1,*v3,v5,a0);
        return;
      }
      error(1,0,dcgettext(NULL,"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",5),v4);
      return;
    }
  }
  else {
    if (v6 == a0) {
      error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
      return;
    }
    if (!*v6) {
label_d295:
      dat_3e688 = (short)v4;
      *v3 = v2;
      return;
    }
  }
  error(1,0,dcgettext(NULL,"Invalid optimisation level %s",5),a0);
}

// Function: sub_d3d0 @ 0xd3d0
undefined16 sub_d3d0(void)
{
  char v1 [16];
  long v2; // rdx
  long v3; // stack - 0x28
  long v4; // stack - 0x20
  
  if (gettimeofday(&v3,NULL)) { // branch-flip
    v3 = time(0);
    if (v3 == -1)
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now"); // no-return
    v2 = 0;
  }
  else {
    v2 = v4 * 1000;
  }
  v1._8_8_ = v2;
  v1._0_8_ = v3;
  return v1._0_16_;
}

// Function: sub_d450 @ 0xd450
void sub_d450(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_b990(a0,a2);
  v1 = *(unsigned long *)(a0 + 8);
  *v2 = a1;
  v2[1] = v1;
  v2[7] = 0;
  v2[2] = 1;
}

// Function: sub_d490 @ 0xd490
void sub_d490(long a0,unsigned long a1)
{
  if (*(long *)(a0 + 0x18)) {
    sub_d450(a0,*(long *)(a0 + 0x18),a1); // tail-call
    return;
  }
  __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary"); // no-return
}

// Function: sub_d4d0 @ 0xd4d0
void sub_d4d0(unsigned long a0)
{
  sub_d490(a0,0); // tail-call
}

// Function: sub_d4e0 @ 0xd4e0
void sub_d4e0(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_3eed0;
  if (a0) {
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
    exit(a0); // no-return
  }
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
  sub_ce70(0);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nUse \'-D help\' for a description of the options, or see find(1)\n\n",5),v1);
  sub_17130(stdout,dat_3eed0);
  exit(0); // no-return
}

// Function: sub_d6f0 @ 0xd6f0
void sub_d6f0(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // stack - 0x50
  char v4 [2]; // stack - 0x42
  char *v5;
  unsigned long *v6; // r13
  long v7; // r15
  
  v3 = NULL;
  v4[0] = ',';
  v4[1] = '\0';
  v1 = strtok_r(a0,v4,&v3);
  if (!v1) {
    error(0,0,dcgettext(NULL,"Empty argument to the -D option.",5));
    sub_d4e0(1); // no-return
  }
  do {
    v6 = (unsigned long *)0x3c158;
    v5 = "exec";
    v7 = 0;
    while (strcmp(v5,v1)) {
      v7 += 1;
      if (v7 == 9) {
        v2 = sub_24e50(0,dat_3e68c,a0);
        error(0,0,dcgettext(NULL,"Ignoring unrecognised debug flag %s",5),v2);
        goto label_d7bb;
      }
      v5 = (char *)*v6;
      v6 = &v6[3];
    }
    dat_3e660 |= (long)*(int *)(v7 * 0x18 + 0x3c148);
label_d7bb:
    v1 = strtok_r(NULL,v4,&v3);
    if (!v1) {
      if (dat_3e660 & 0x10) {
        sub_ce70(1);
        exit(0); // no-return
      }
      return;
    }
  } while( true );
}

// Function: sub_d860 @ 0xd860
void sub_d860(void)
{
  return;
}

// Function: sub_d870 @ 0xd870
bool sub_d870(void)
{
  char v1 [130];
  char v2 [262];
  bool v3;
  float v4; // xmm1_da
  
  v3 = 1;
  if (!uname(v1)) {
    v4 = (float)(double)atof(v2);
    if (strcmp("Linux",v1)) { // branch-flip
      if (!strcmp("FreeBSD",v1))
        v3 = (float)s_2d2e3._1_4_ <= v4;
    }
    else {
      v3 = dat_2d2e0 <= v4;
    }
  }
  return v3;
}

// Function: sub_d940 @ 0xd940
void sub_d940(void)
{
  if (!dat_3e608)
    return;
  sub_cfc0(sub_b5e0());
  dat_3e608 = 0;
}

// Function: sub_d970 @ 0xd970
void sub_d970(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  char *v4;
  
  dat_3e5c0 = (int *)sub_26990(0x10);
  v1 = sub_251f0(dat_3e5c0);
  if (!v1)
    return;
  v3 = "";
  v4 = "";
  if (*dat_3e5c0 < 0) {
    v3 = *(char **)&dat_3e5c0[2];
    v4 = ": ";
    if (!v3) {
      v3 = "";
      v4 = v3;
    }
  }
  v2 = dcgettext(NULL,"Failed to save initial working directory%s%s",5);
  error(1,*__errno_location(),v2,v4,v3);
}

// Function: sub_da00 @ 0xda00
unsigned long sub_da00(char *a0,void *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_3e5f8 != -100) && (dat_3e5f8 < 0))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat"); // no-return
  if (dat_3e5e0) {
    v1 = fstatat(dat_3e5f8,a0,a1,0x100); // tail-call
    return v1;
  }
  v1 = fstatat(dat_3e5f8,a0,a1,0);
  if (!v1)
    return 0;
  v2 = sub_d1a0(a0,a1,v1); // tail-call
  return v2;
}

// Function: sub_daa0 @ 0xdaa0
unsigned long sub_daa0(char *a0,void *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_3e5f8 != -100) && (dat_3e5f8 < 0))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat"); // no-return
  v1 = fstatat(dat_3e5f8,a0,a1,0);
  if (!v1)
    return 0;
  v2 = sub_d1a0(a0,a1,v1); // tail-call
  return v2;
}

// Function: sub_db20 @ 0xdb20
void sub_db20(char *a0,void *a1)
{
  if ((dat_3e5f8 <= -1) && (dat_3e5f8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  fstatat(dat_3e5f8,a0,a1,0x100); // tail-call
}

// Function: sub_db70 @ 0xdb70
void sub_db70(unsigned long a0,unsigned long a1)
{
  dat_3e6e0 += 1;
  __fprintf_chk(stderr,1,"debug_stat (%s)\n",a0);
  if (dat_3e668 == 1) {
    sub_daa0(a0,a1); // tail-call
    return;
  }
  if (dat_3e668 != 2) {
    if (dat_3e668)
      __assert_fail("0","util.c",0x2a5,"debug_stat"); // no-return
    sub_db20(a0,a1); // tail-call
    return;
  }
  sub_da00(a0,a1); // tail-call
}

// Function: sub_dc30 @ 0xdc30
unsigned int sub_dc30(void) // early-return x2
{
  if (dat_3e668 == 1)
    return 1;
  if (dat_3e668 != 2)
    return 0;
  return CONCAT31((undefined3)((unsigned int)dat_3e5e0 >> 8),dat_3e5e0 == 0);
}

// Function: sub_dc60 @ 0xdc60
unsigned long sub_dc60(long a0)
{
  while( true ) {
    if (!a0)
      return 1;
    if (*(char *)(a0 + 0x19)) break;
    a0 = *(long *)(a0 + 0x108);
  }
  return 0;
}

// Function: sub_dca0 @ 0xdca0
unsigned long sub_dca0(char *a0,unsigned int a1) // return-dupe
{
  undefined7 v1; // rax
  
  v1 = (undefined7)((unsigned long)((long)*(int *)((unsigned long)(unsigned char)(*a0 - 0x21) * 4 + 0x2d230) + 0x2d230) >> 8);
  switch(*a0) {
    case 0x21:
    case 0x28:
      return CONCAT71(v1,a0[1] == '\0');
    default:
      return 0;
    case 0x29:
    case 0x2c:
      goto label_dcd0;
    case 0x2d:
      return CONCAT71(v1,a0[1] != '\0');
    
  }
label_dcd0:
  if (!a0[1])
    return (unsigned long)(a1 ^ 1);
  return 0;
}

// Function: sub_dd00 @ 0xdd00
int sub_dd00(int a0,long a1)
{
  char *v1;
  long v2; // rax
  int v3; // r12d
  
  if (2 <= a0) { // branch-flip
    v3 = 1;
    do {
      while( true ) {
        v2 = (long)v3;
        v1 = *(char **)(a1 + v2 * 8);
        if (strcmp("-H",v1)) break;
        v3 += 1;
        sub_146f0(2);
        if (a0 <= v3)
          return v3;
      }
      if (strcmp("-L",v1)) { // branch-flip
        if (strcmp("-P",v1)) { // branch-flip
          if (!strcmp("--",v1))
            return v3 + 1;
          if (strcmp("-D",v1)) { // branch-flip
            if (strncmp("-O",v1,2))
              return v3;
            sub_d220(&v1[2]);
          }
          else {
            v3 += 1;
            if (a0 <= v3) {
              error(0,0,dcgettext(NULL,"Missing argument after the -D option.",5));
              sub_d4e0(1); // no-return
            }
            sub_d6f0(*(unsigned long *)(a1 + 8 + v2 * 8));
          }
        }
        else {
          sub_146f0(0);
        }
      }
      else {
        sub_146f0(1);
      }
      v3 += 1;
    } while (v3 < a0);
  }
  else {
    v3 = 1;
  }
  return v3;
}

// Function: sub_de90 @ 0xde90
void sub_de90(unsigned short *a0)
{
  char v1; // al
  int v2; // eax
  char v3 [16];
  
  *(bool *)((long)a0 + 0x11) = getenv("POSIXLY_CORRECT") != NULL;
  v1 = sub_d870();
  *(unsigned int *)&a0[0x2e] = 0;
  *(char *)&a0[0x2c] = v1;
  v2 = isatty(0);
  *(char *)((long)a0 + 0xf) = 0;
  *(bool *)&a0[8] = v2 != 0;
  if (*(char *)((long)a0 + 0x11))
    *(char *)&a0[8] = 0;
  *(unsigned long *)&a0[2] = 0xffffffffffffffff;
  *a0 = 1;
  v3 = sub_d3d0();
  *(char *)&a0[0x1c] = 0;
  *(long *)&a0[0xc] = SUB168(v3,0);
  *(long *)&a0[0x10] = SUB168(v3,8);
  *(long *)&a0[0x18] = SUB168(v3,8);
  *(long *)&a0[0x14] = SUB168(v3,0) + -0x15180;
  *(unsigned short *)((long)a0 + 0xd) = 0;
  *(unsigned long *)&a0[0x20] = 0;
  *(unsigned int *)&a0[0x1e] = (-(unsigned int)(*(char *)((long)a0 + 0x11) == '\0') & 0x200) + 0x200;
  a0[0x34] = 2;
  if (!getenv("FIND_BLOCK_SIZE")) {
    *(char *)&a0[6] = 0;
    sub_146f0(0);
    *(unsigned int *)&a0[0x36] = 8;
    *(unsigned long *)&a0[0x38] = 0;
    *(char *)&a0[0x3c] = 0;
    return;
  }
  error(1,0,dcgettext(NULL,"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable",5));
}

// Function: sub_df90 @ 0xdf90
unsigned int sub_df90(void *a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == sub_9530) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == sub_8ff0);
}

// Function: sub_dfb0 @ 0xdfb0
void sub_dfb0(unsigned long a0,unsigned long a1)
{
  sub_24e50(a0,dat_3e68c,a1); // tail-call
}

// Function: sub_dfd0 @ 0xdfd0
void sub_dfd0(int a0,int a1,char a2,unsigned long a3) // return-dupe
{
  if (!a2) {
    error(a0,a1,"%s",sub_dfb0(0,a3));
    dat_3e604 = 1;
    return;
  }
  if (dat_3e618) {
    dat_3e618 = 1;
    return;
  }
  error(a0,a1,"%s",sub_dfb0(0,a3));
  dat_3e604 = 1;
  dat_3e618 = 1;
}

// Function: sub_e060 @ 0xe060
void sub_e060(unsigned int a0,unsigned long a1)
{
  sub_dfd0(0,a0,1,a1); // tail-call
}

// Function: sub_e080 @ 0xe080
unsigned long sub_e080(unsigned long a0,unsigned long a1,long a2) // return-dupe x2
{
  int v1; // eax
  unsigned long v2; // rax
  int *v3; // rax
  
  if (dat_3e5e4) {
    dat_3e5e4 = 0x101;
    dat_3e5e8 = *(unsigned int *)(a2 + 0x18);
    return 0;
  }
  v1 = (*dat_3e670)(a1,a2);
  if (!v1) {
    if (*(int *)(a2 + 0x18)) {
      dat_3e5e8 = *(int *)(a2 + 0x18);
      dat_3e5e4 = 0x101;
      return 0;
    }
    v2 = sub_24e50(0,dat_3e68c,a1);
    error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v2);
    dat_3e5e4 = 0x101;
    dat_3e5e8 = *(unsigned int *)(a2 + 0x18);
    dat_3e604 = 1;
    return 0;
  }
  v3 = __errno_location();
  if ((dat_3e62e) && (*v3 == 2))
    return 0xffffffff;
  sub_e060(*v3,a0);
  return 0xffffffff;
}

// Function: sub_e150 @ 0xe150
unsigned int sub_e150(unsigned int *a0,unsigned long a1,unsigned long a2,long a3,unsigned int a4) // return-dupe
{
  unsigned int v1; // eax
  unsigned int v2;
  
  v2 = *a0;
  if (v2) {
    if (((v2 & 0xf000) == 0xa000) && (v1 = sub_dc30(), (char)v1)) {
      if (sub_e080())
        return 0;
      dat_3e5e8 = *(unsigned int *)(a3 + 0x18);
      *a0 = dat_3e5e8;
      dat_3e5e4 = CONCAT11(1,dat_3e5e4);
      return v1;
    }
    *(unsigned int *)(a3 + 0x18) = v2;
    dat_3e5e4 = CONCAT11(1,dat_3e5e4);
    dat_3e5e8 = v2;
    return 1;
  }
  if ((char)a4) {
    dat_3e5e4 = 0;
    dat_3e5e8 = 0;
    return a4;
  }
  if (sub_e080(a1,a2,a3))
    return 0;
  dat_3e5e8 = *(unsigned int *)(a3 + 0x18);
  *a0 = dat_3e5e8;
  dat_3e5e4 = CONCAT11(1,dat_3e5e4);
  return 1;
}

// Function: sub_e230 @ 0xe230
int sub_e230(unsigned long a0,long a1,long a2)
{
  int v1; // eax
  
  if ((((!*(char *)(a2 + 0x1a)) || (dat_3e5e4)) && ((!*(char *)(a2 + 0x1b) || (dat_3e5e5)))) && ((!*(char *)(a2 + 0x1c) || (((*(long *)(a1 + 8) && (dat_3e5e5)) && ((*(unsigned int *)(a1 + 0x18) & 0xf000) != 0x4000))))))
    return 0;
  v1 = sub_e080(a0,dat_3e5f0,a1);
  return -(unsigned int)(v1 != 0);
}

// Function: sub_e2a0 @ 0xe2a0
unsigned long sub_e2a0(unsigned long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long v1; // rax
  
  a2[0x24] = a2[0x24] + 1;
  if (((!(a2[3] & 0xffffff0000)) || (!sub_e230())) && (v1 = (*(void *)*a2)(a0,a1,a2), (char)v1)) {
    a2[0x25] = a2[0x25] + 1;
    return v1;
  }
  return 0;
}

// Function: sub_e310 @ 0xe310
void sub_e310(unsigned int a0,unsigned long a1)
{
  sub_dfd0(1,a0,1,a1);
  abort(); // no-return
}

// Function: sub_e340 @ 0xe340
void sub_e340(unsigned int a0,unsigned long a1)
{
  sub_dfd0(0,a0,0,a1); // tail-call
}

// Function: sub_e360 @ 0xe360
void sub_e360(void)
{
  char v1; // al
  int v2; // eax
  long v3; // rax
  
  v3 = sub_b5e0();
  if (v3) { // branch-flip
    sub_cdf0(v3,sub_d080);
    sub_d940();
    sub_e560(dat_3e610);
    sub_cdf0(v3,sub_ce30);
    sub_d0f0();
    v1 = sub_15dd0();
  }
  else {
    sub_e560(dat_3e610);
    sub_d0f0();
    v1 = sub_15dd0();
  }
  if (v1) {
    sub_15df0();
    sub_15da0();
  }
  v2 = sub_17bc0(stdout);
  if (v2 == -1) {
    sub_e340(*__errno_location(),"standard output"); // tail-call
    return;
  }
}

// Function: sub_e410 @ 0xe410
void sub_e410(unsigned int a0,unsigned long a1)
{
  dat_3e618 = 0;
  sub_dfd0(1,a0,0,a1);
  abort(); // no-return
}

// Function: sub_e440 @ 0xe440
unsigned long sub_e440(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_e470 @ 0xe470
unsigned long sub_e470(unsigned long *a0,unsigned long a1)
{
  return (*a0 ^ a0[1]) % a1;
}

// Function: sub_e490 @ 0xe490
void sub_e490(void *a0)
{
  long v1;
  int *v2; // rax
  
  v1 = *(long *)((long)a0 + 0x18);
  if (v1) {
    if (sub_17850(v1)) {
      v2 = __errno_location();
      sub_e410(*v2,*(unsigned long *)((long)a0 + 0x10)); // no-return
    }
  }
  free(*(void **)((long)a0 + 0x10));
  free(a0); // tail-call
}

// Function: sub_e4d0 @ 0xe4d0
unsigned long * sub_e4d0(char *a0)
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
      v3 = sub_1ac70(0xb,0,sub_e470,sub_e440,sub_e490);
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

// Function: sub_e560 @ 0xe560
void sub_e560(unsigned long *a0)
{
  free((void *)*a0);
  sub_1ae30(a0[1]); // tail-call
}

// Function: sub_e580 @ 0xe580
unsigned long sub_e580(unsigned long *a0,char *a1)
{
  int v1;
  unsigned long *v2; // rax
  char *v3; // rax
  void *v4; // rax
  long v5; // rax
  int *v6; // rax
  unsigned long v7; // stack - 0xc8
  unsigned long v8;
  unsigned long v9; // stack - 0xc0
  
  v2 = malloc(0x20);
  if (v2) { // branch-flip
    v3 = strdup(a1);
    v2[2] = v3;
    if (v3) { // branch-flip
      v4 = (void *)sub_17da0(a1,*a0);
      v2[3] = v4;
      if (v4) { // branch-flip
        v1 = fileno(v4);
        if (v1 < 0)
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen"); // no-return
        sub_174f0(v1,1);
        if (0 <= fstat(v1,&v7)) { // branch-flip
          v8 = a0[1];
          *v2 = v7;
          v2[1] = v9;
          v5 = sub_1aa10(v8,v2);
          if (v5) { // branch-flip
            sub_e490(v2);
            v8 = *(unsigned long *)(v5 + 0x18);
          }
          else {
            v8 = a0[1];
            if (sub_1b290(v8,v2)) // branch-flip
              v8 = v2[3];
            else {
              v6 = __errno_location();
              v1 = *v6;
              sub_e490(v2);
              *v6 = v1;
              v8 = 0;
            }
          }
        }
        else {
          v8 = 0;
          sub_e490(v2);
        }
      }
      else {
        sub_e490(v2);
        v8 = 0;
      }
    }
    else {
      free(v2);
      v8 = 0;
    }
  }
  else {
    v8 = 0;
  }
  return v8;
}

// Function: sub_e720 @ 0xe720
bool sub_e720(char a0)
{
  return (unsigned char)(a0 - 0x30U) <= 7;
}

// Function: sub_e730 @ 0xe730
int sub_e730(char a0) // early-return
{
  if (0x1b <= (unsigned char)(a0 + 0xa4U))
    return 0;
  return (int)"\\"[(unsigned char)(a0 + 0xa4U)];
}

// Function: sub_e750 @ 0xe750
char * sub_e750(int a0) // early-return x7
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
  if (a0 != 0x1000)
    return "U";
  return "p";
}

// Function: sub_e7d0 @ 0xe7d0
char sub_e7d0(char a0)
{
  if (strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)a0))
    return '\x01';
  return (strchr("ABCT",(int)a0) != NULL) * '\x02';
}

// Function: sub_e820 @ 0xe820
long sub_e820(long a0)
{
  long v1;
  unsigned char v2;
  long v3; // r12
  
  v3 = 0;
  while( true ) {
    v3 += 1;
    v2 = *(unsigned char *)(a0 + v3);
    if (!v2) break;
    if (!strchr("-+ #",(int)(char)v2)) break;
  }
  v1 = *(long *)__ctype_b_loc();
  while (*(unsigned char *)(v1 + 1 + (unsigned long)v2 * 2) & 8) {
    v3 += 1;
    v2 = *(unsigned char *)(a0 + v3);
  }
  if (v2 == 0x2e) {
    do {
      v3 += 1;
    } while (*(unsigned char *)(v1 + 1 + (unsigned long)*(unsigned char *)(a0 + v3) * 2) & 8);
  }
  return v3;
}

// Function: sub_e8c0 @ 0xe8c0
void sub_e8c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,long a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  void *v1;
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
  v1 = *(void **)(a8 + 8);
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
  sub_e340(*v3,*(unsigned long *)(a8 + 0x10));
}

// Function: sub_e9a0 @ 0xe9a0
void sub_e9a0(long a0)
{
  unsigned long v1;
  int *v2; // rax
  
  v1 = *(unsigned long *)(a0 + 8);
  if (!sub_17bc0(v1))
    return;
  v2 = __errno_location();
  sub_e340(*v2,*(unsigned long *)(a0 + 0x10)); // tail-call
}

// Function: sub_e9d0 @ 0xe9d0
void sub_e9d0(long a0,unsigned long a1,unsigned long a2)
{
  char v1;
  unsigned long v2;
  unsigned long v3;
  int *v4; // rax
  
  v2 = *(unsigned long *)(a0 + 0x20);
  v1 = *(char *)(a0 + 0x18);
  v3 = *(unsigned long *)(a0 + 8);
  if (0 <= (int)sub_16960(v3,v2,v1,a1,a2))
    return;
  v4 = __errno_location();
  sub_e340(*v4,*(unsigned long *)(a0 + 0x10)); // tail-call
}

// Function: sub_ea10 @ 0xea10
void sub_ea10(void *a0,unsigned long a1,long a2)
{
  void *v1;
  int *v2; // rax
  
  v1 = *(void **)(a2 + 8);
  if (a1 <= fwrite(a0,1,a1,v1))
    return;
  v2 = __errno_location();
  sub_e340(*v2,*(unsigned long *)(a2 + 0x10)); // tail-call
}

// Function: sub_ea60 @ 0xea60
unsigned long sub_ea60(unsigned char *a0,char *a1,long *a2,long *a3)
{
  unsigned char v1;
  long v2;
  bool v3;
  long *v4;
  unsigned long v5; // rbx
  long v6; // r15
  
  v3 = 0;
  v5 = (unsigned long)*a0;
  v2 = 0;
  if (*a0) {
label_eab6:
    do {
      v6 = v2;
      v1 = a1[v6];
      if (!v1)
        return 0;
      if (v1 != (unsigned char)v5) {
        v2 = *(long *)__ctype_b_loc();
        if (!(*(unsigned char *)(v2 + 1 + v5 * 2) & 8))
          return 0;
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)v1 * 2) & 8))
          return 0;
        if (v3) {
          if (v6 - *a2 != *a3)
            return 0;
          v4 = (long *)((v6 - *a2) + 1);
          *a3 = (long)v4;
          v5 = (unsigned long)a0[v6 + 1];
          v2 = v6 + 1;
          if (!a0[v6 + 1]) break;
          goto label_eab6;
        }
        v3 = 1;
        *a2 = v6;
        *a3 = 1;
        v4 = a3;
      }
      v5 = (unsigned long)a0[v6 + 1];
      v2 = v6 + 1;
    } while (a0[v6 + 1]);
    a1 = &a1[v6 + 1];
  }
  return CONCAT71((undefined7)((unsigned long)v4 >> 8),*a1 == '\0');
}

// Function: sub_eb50 @ 0xeb50
long sub_eb50(char *a0,int *a1,void *a2,unsigned long a3)
{
  unsigned char *v1;
  long v10; // rax
  unsigned long v11; // stack - 0x78
  long v12; // stack - 0x80
  long v13; // stack - 0x88
  unsigned int v14; // stack - 0x74
  unsigned long v15; // stack - 0x70
  unsigned long v16; // stack - 0x68
  unsigned long v17; // stack - 0x60
  unsigned long v18; // stack - 0x58
  unsigned long v19; // stack - 0x50
  unsigned long v2;
  unsigned long v20; // stack - 0x48
  int v3;
  long v4;
  char v5; // al
  int v6;
  char *v7; // rax
  unsigned long v8;
  char *v9;
  
  v7 = (char *)sub_26990(strlen(a0) + 2);
  *v7 = '_';
  memcpy(&v7[1],a0,strlen(a0) + 1);
  v3 = *a1;
  v20 = *(unsigned long *)&a1[0xc];
  v15 = *(unsigned long *)&a1[2];
  v16 = *(unsigned long *)&a1[4];
  v17 = *(unsigned long *)&a1[6];
  v18 = *(unsigned long *)&a1[8];
  v19 = *(unsigned long *)&a1[10];
  v6 = v3 + 0xb;
  if (0xb <= v3)
    v6 = v3 + -0xb;
  v14 = (unsigned int)((unsigned long)*(unsigned long *)a1 >> 0x20);
  v11 = CONCAT44(v14,v6);
  v9 = dat_3eb08;
  if (!dat_3eb08) {
    dat_3eb00 = 1;
    v9 = (char *)sub_26990(1);
    dat_3eb08 = v9;
  }
  while ((v8 = strftime(v9,dat_3eb00,v7,a1), !v8 || (dat_3eb00 <= v8))) {
    v9 = (char *)sub_26ad0(v9,0x3eb00,1);
    dat_3eb08 = v9;
  }
  v2 = v8 + 1 + a3;
  v13 = 0;
  v12 = 0;
  dat_3eb08 = (long)sub_269e0(v9,v2);
  dat_3eb00 = v2;
  v9 = (char *)sub_26990(v2);
  strftime(v9,dat_3eb00,v7,&v11);
  v4 = (long)dat_3eb08;
  v5 = sub_ea60(dat_3eb08,v9,&v13,&v12);
  if ((v5) && (v12 == 2)) {
    v10 = v13 + 2;
    v1 = (unsigned char *)(v4 + v10);
    if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)*v1 * 2) & 8)) {
      v8 = (v8 - v10) + 1;
      if (v10 + a3 + v8 != v2)
        __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,"do_time_format"); // no-return
      memmove((void *)(v4 + v10 + a3),v1,v8);
      memcpy(v1,a2,a3);
    }
  }
  free(v7);
  free(v9);
  return v4 + 1;
}

// Function: sub_eda0 @ 0xeda0
char * sub_eda0(long a0,unsigned long a1,int a2)
{
  bool v1;
  unsigned long v10; // r13
  unsigned long v11; // stack - 0x70
  unsigned int v12; // stack - 0x5c
  bool v2;
  int v3;
  char *v4; // rax
  void *v5; // rax
  long v6; // stack - 0x78
  char v7 [40]; // stack - 0x58
  unsigned long v8; // stack - 0x64
  long v9;
  
  v6 = a0;
  v11 = a1;
  if (a2 != 0x2b) { // branch-flip
    v8._0_2_ = CONCAT11((char)a2,0x25);
    v8._0_3_ = (undefined3)(unsigned short)v8;
    if ((0x19 <= (unsigned int)(a2 - 0x40U)) || (!(0x1180001UL >> ((unsigned long)(unsigned int)(a2 - 0x40U) & 0x3f) & 1))) {
      v7[0] = '\0';
      v2 = 0;
      v1 = 0;
      v3 = 0;
    }
    else {
      v2 = 1;
      v1 = 1;
      v3 = __snprintf_chk(v7,0x20,1,0x20,".%09ld0",a1);
    }
    if (a2 == 0x40) goto label_ee0a;
  }
  else {
    v8 = 0x64252d6d252d5925;
    v12 = 0x54252b;
    v3 = __snprintf_chk(v7,0x20,1,0x20,".%09ld0",a1);
    v2 = 1;
  }
  v1 = v2;
  v5 = localtime(&v6);
  if ((v5) && (v4 = (char *)sub_eb50(&v8,v5,v7,(long)v3), v4))
    return v4;
label_ee0a:
  v9 = -v6;
  if (-v6 <= -1)
    v9 = v6;
  v4 = (char *)sub_1b800(v9,0x3eb21,0,1,1);
  if ((char *)0x3eb21 <= v4) {
    if ((char *)0x3ee37 <= v4)
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date"); // no-return
    if (v6 <= -1) {
      v4[-1] = '-';
      v4 = &v4[-1];
    }
    if (v1) {
      if ((char *)0x317 <= &v4[strlen(v4) - 0x3eb20])
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date"); // no-return
      v10 = 0x316 - (long)&v4[strlen(v4) - 0x3eb20];
      if (v10 <= strlen(v7)) {
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)v3,v10,v7);
        if (v10 <= strlen(v7))
          __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date"); // no-return
      }
      strcat(v4,v7);
    }
    return v4;
  }
  __assert_fail("p > buf","print.c",0x2b9,"format_date"); // no-return
}

// Function: sub_f050 @ 0xf050
unsigned long sub_f050(unsigned long a0,unsigned long a1) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  int v4; // eax
  unsigned int *v5; // rax
  unsigned long v6; // stack - 0x18
  unsigned long v7; // stack - 0x10
  
  v6 = a0;
  v7 = a1;
  v5 = localtime(&v6);
  if (!v5)
    return sub_eda0(v6,v7,0x40);
  v1 = v5[6];
  if ((int)v1 < 0)
    __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format"); // no-return
  if (6 < (int)v1)
    __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format"); // no-return
  v2 = v5[4];
  if ((int)v2 < 0)
    __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format"); // no-return
  if (0xb < (int)v2)
    __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format"); // no-return
  v3 = v5[2];
  if ((int)v3 < 0)
    __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format"); // no-return
  if (0x17 < (int)v3)
    __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format"); // no-return
  if ((int)v5[1] <= 0x3b) {
    if (0x3d < (int)*v5)
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format"); // no-return
    v4 = __snprintf_chk((char *)0x3e700,0x400,1,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",*(unsigned long *)((long)(int)v1 * 8 + 0x3c280),*(unsigned long *)((long)(int)v2 * 8 + 0x3c220),(unsigned long)v5[3],(unsigned long)v3,(unsigned long)v5[1],(unsigned long)*v5,v7,(unsigned long)(v5[5] + 0x76c));
    if (v4 <= 0x3ff)
      return 0x3e700;
    __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format"); // no-return
  }
  __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format"); // no-return
}

// Function: sub_f270 @ 0xf270
unsigned long sub_f270(long a0,long *a1)
{
  char v1; // al
  unsigned long v2; // rcx
  long v3; // rdx
  long v4; // rdx
  int v5; // r8d
  
  v4 = 0;
  v2 = 0;
  do {
    v5 = (int)*(char *)(a0 + v4);
    v1 = sub_e720(v5);
    if (!v1) {
      *a1 = v3 + -1;
      return v2 & 0xffffffff;
    }
    v4 = v3 + 1;
    v2 = (unsigned long)(unsigned int)(v5 + -0x30 + (int)v2 * 8);
  } while (v4 != 3);
  *a1 = 2;
  return v2;
}

// Function: sub_f2c0 @ 0xf2c0
double sub_f2c0(long a0,long a1)
{
  double v1;
  
  if (a0)
    return ((double)a1 * dat_2da98) / (double)a0;
  v1 = dat_2da80;
  if ((a1) && (v1 = dat_2da90, 0 <= a1))
    return dat_2da88;
  return v1;
}

// Function: sub_f310 @ 0xf310
void sub_f310(unsigned long a0,int *a1,char *a2,unsigned long *a3) // return-dupe
{
  int v1;
  char v10 [24];
  unsigned int v11;
  unsigned long v12; // r14
  char v13 [16];
  unsigned int v14; // stack - 0x340
  char v15; // stack - 0x2be
  unsigned long v2;
  unsigned long v3;
  unsigned long *v4; // rax
  char *v5;
  int *v6; // rax
  char *v7;
  char v8 [10];
  unsigned long v9; // stack - 0x360
  
  v1 = *a1;
  if (v1 == 1) {
    sub_ea10(*(unsigned long *)&a1[2],(long)a1[4],a0);
    sub_e9a0(a0); // tail-call
    return;
  }
  if (v1 != 2) {
    if (v1)
      return;
label_f378:
    sub_ea10(*(unsigned long *)&a1[2],(long)a1[4],a0); // tail-call
    return;
  }
  switch((char)a1[1]) {
    case 0:
      error(1,0,dcgettext(NULL,"error: %s at end of format string",5),"%");
      goto label_f34a;
    default:
      return;
    case 0x25:
      goto label_f378;
    case 0x44:
      v2 = sub_1b800(*a3,v8,0,1,1);
      break;
    case 0x46:
      v7 = (char *)sub_104c0(a3,a2);
      goto label_f4a3;
    case 0x47:
      goto label_f45f;
    case 0x48:
      v7 = (char *)sub_26990((long)(dat_3e5fc + 1));
      v12 = (unsigned long)dat_3e5fc;
      memcpy(v7,a2,v12);
      v7[v12] = '\0';
      sub_e8c0(a0,*(unsigned long *)&a1[2],v7);
      free(v7); // return-dupe, tail-call
      return;
    case 0x4d:
      sub_17d90(a3,v8);
      v15 = 0;
      sub_e8c0(a0,*(unsigned long *)&a1[2],v8);
      return;
    case 0x50:
      v7 = "";
      if (1 <= (int)dat_3e5e0)
        v7 = &(&a2[dat_3e5fc])[a2[dat_3e5fc] == '/'];
label_f4a3:
      a2 = v7;
      v2 = *(unsigned long *)&a1[2];
      sub_e9d0(a0,v2,a2); // return-dupe, tail-call
      return;
    case 0x53:
      sub_f2c0(a3[6],a3[8]);
      sub_e8c0(a0,*(unsigned long *)&a1[2]); // tail-call
      return;
    case 0x55:
      goto label_f415;
    case 0x59:
      v11 = *(unsigned int *)&a3[3] & 0xf000;
      if (v11 == 0xa000) {
        v1 = fstatat(dat_3e5f8,dat_3e5f0,v10,0);
        if (v1) {
          v6 = __errno_location();
          v2 = *(unsigned long *)&a1[2];
          v1 = *v6;
          if ((v1 != 2) && (v1 != 0x14)) {
            if (v1 != 0x28) {
              sub_e8c0(a0,v2,"?");
              v2 = sub_dfb0(0,a2);
              error(0,*v6,"%s",v2);
              return;
            }
            sub_e8c0(a0,v2,"L");
            return;
          }
          sub_e8c0(a0,v2,"N");
          return;
        }
        v2 = sub_e750(v14 & 0xf000);
        break;
      }
      v3 = sub_e750(v11); // return-dupe
      v2 = *(unsigned long *)&a1[2];
      sub_e8c0(a0,v2,v3); // return-dupe, tail-call
      return;
    case 0x5a:
      v1 = (*dat_3e680)(dat_3e5f8,dat_3e5f0,&v9);
      if (0 <= v1) {
        sub_e8c0(a0,*(unsigned long *)&a1[2],v9);
        freecon(v9);
        return;
      }
label_f34a:
      sub_e8c0(a0);
      v2 = sub_dfb0(0,a2);
      v7 = dcgettext(NULL,"getfilecon failed: %s",5);
      error(0,*__errno_location(),v7,v2);
      dat_3e604 = 1;
      return;
    case 0x61:
      v13 = sub_25c40(a3);
      v3 = SUB168(v13,8);
      v2 = SUB168(v13,0);
      goto label_f690;
    case 0x62:
      v2 = sub_1b800(a3[8],v8,0,0x200,0x200);
      break;
    case 99:
      v13 = sub_25c50(a3);
      v3 = SUB168(v13,8);
      v2 = SUB168(v13,0);
      goto label_f690;
    case 100:
      v2 = *(unsigned long *)&a1[2];
      v11 = dat_3e5e0;
      sub_e8c0(a0,v2,v11); // return-dupe, tail-call
      return;
    case 0x66:
      v7 = (char *)sub_176f0(a2);
      goto label_f84e;
    case 0x67:
      v4 = getgrgid(*(unsigned int *)&a3[4]);
      if (v4) {
label_f659:
        *(char *)(*(long *)&a1[2] + (long)a1[4]) = 0x73;
        v2 = *(unsigned long *)&a1[2];
        v3 = *v4;
        sub_e8c0(a0,v2,v3);
        return;
      }
      v2 = sub_1b800(*(unsigned int *)&a3[4],v8,0,1,1); // return-dupe
      sub_e8c0(a0,*(unsigned long *)&a1[2],v2); // return-dupe
      return;
    case 0x68:
      v7 = (char *)sub_26d30(a2);
      for (v5 = &v7[strlen(v7) - 1]; v7 <= v5; v5 = &v5[-1]) {
        if (*v5 != '/') {
          if ((v7 < v5) && (v5[1] == '/'))
            v5[1] = '\0';
          break;
        }
      }
      v5 = strrchr(v7,0x2f);
      if (!v5) {
        sub_e9d0(a0,*(unsigned long *)&a1[2],".");
        free(v7);
        return;
      }
      *v5 = '\0';
      sub_e9d0(a0,*(unsigned long *)&a1[2],v7); // return-dupe
      free(v7);
      return;
    case 0x69:
      v2 = sub_1b800(a3[1],v8,0,1,1);
      break;
    case 0x6b:
      v2 = sub_1b800(a3[8],v8,0,0x200,0x400);
      break;
    case 0x6c:
      if ((*(unsigned int *)&a3[3] & 0xf000) == 0xa000) {
        v7 = (char *)sub_17190(dat_3e5f8,dat_3e5f0);
        if (v7) {
          sub_e9d0(a0,*(unsigned long *)&a1[2],v7);
          free(v7);
          return;
        }
        sub_e060(*__errno_location(),a2);
        dat_3e604 = 1;
      }
      v7 = NULL;
      sub_e9d0(a0,*(unsigned long *)&a1[2],"");
      free(v7);
      return;
    case 0x6d:
      v2 = *(unsigned long *)&a1[2];
      v11 = *(unsigned int *)&a3[3] & 0xfff;
      sub_e8c0(a0,v2,v11);
      return;
    case 0x6e:
      v2 = sub_1b800(a3[2],v8,0,1,1);
      break;
    case 0x70:
      v2 = *(unsigned long *)&a1[2];
      sub_e9d0(a0,v2,a2);
      return;
    case 0x73:
      v2 = sub_1b800(a3[6],v8,0,1,1);
      break;
    case 0x74:
      v13 = sub_25c60(a3);
      v3 = SUB168(v13,8);
      v2 = SUB168(v13,0);
label_f690:
      v3 = sub_f050(v2,v3);
label_f58c:
      v2 = *(unsigned long *)&a1[2];
      sub_e8c0(a0,v2,v3);
      return;
    case 0x75:
      v4 = getpwuid(*(unsigned int *)((long)a3 + 0x1c));
      if (v4) goto label_f659;
      v2 = sub_1b800(*(unsigned int *)((long)a3 + 0x1c),v8,0,1,1); // return-dupe
      sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
      return;
    case 0x79:
      v11 = *(unsigned int *)&a3[3] & 0xf000;
      v3 = sub_e750(v11);
      goto label_f58c;
    
  }
label_f34a:
  sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
  return;
  v2 = sub_1b800(*(unsigned int *)((long)a3 + 0x1c),v8,0,1,1);
  sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
  return;
  v2 = sub_1b800(*(unsigned int *)&a3[4],v8,0,1,1);
  sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
  return;
}

// Function: sub_fad0 @ 0xfad0
unsigned long sub_fad0(unsigned long a0,unsigned long a1,long a2) // return-dupe
{
  char v1;
  char v2;
  unsigned long v3; // rax
  int *v4;
  long v5; // rdx
  char v6 [16];
  
  v4 = *(int **)(a2 + 0x38);
  if (!v4)
    return 1;
  a2 += 0x38;
  do {
    while ((*v4 != 2 || (v1 = *(char *)((long)v4 + 5), !v1))) {
      sub_f310(a2,v4,a0,a1);
      v4 = *(int **)&v4[6];
      if (!v4)
        return 1;
    }
    v2 = (char)v4[1];
    if (v2 != 'C') { // branch-flip
      if ('C' < v2) {
        if (v2 != 'T')
          __assert_fail("0","print.c",0x518,"pred_fprintf"); // no-return, return-dupe
        v6 = sub_25c60(a1);
        v5 = SUB168(v6,8);
        v3 = SUB168(v6,0);
        goto label_fb35;
      }
      if (v2 == 'A') {
        v6 = sub_25c40(a1);
        v5 = SUB168(v6,8);
        v3 = SUB168(v6,0);
        goto label_fb35;
      }
      if (v2 != 'B') {
        __assert_fail("0","print.c",0x518,"pred_fprintf");
      }
      v6 = sub_25c70(a1);
      v5 = SUB168(v6,8);
      v3 = SUB168(v6,0);
      if ((v1 == '@') || (0 <= v5)) goto label_fb35;
      sub_e8c0(a2,*(unsigned long *)&v4[2],"");
    }
    else {
      v6 = sub_25c50(a1);
      v5 = SUB168(v6,8);
      v3 = SUB168(v6,0);
label_fb35:
      v3 = sub_eda0(v3,v5,(int)v1);
      sub_e8c0(a2,*(unsigned long *)&v4[2],v3);
    }
    v4 = *(int **)&v4[6];
    if (!v4)
      return 1;
  } while( true );
}

// Function: sub_fc20 @ 0xfc20
long sub_fc20(long *a0,char *a1,unsigned int a2,unsigned int a3,char a4,char a5,long a6)
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
  v3 = (unsigned int *)sub_26990(0x20);
  *a0 = (long)v3;
  *v3 = a3;
  *(char *)&v3[1] = a4;
  *(char *)((long)v3 + 5) = a5;
  v3[6] = 0;
  v3[7] = 0;
  v3[4] = a2;
  v4 = (char *)sub_26990(v5 + 2);
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
      goto label_fd69;
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
      *(char *)(a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x48:
    case 0x50:
    case 0x66:
    case 0x68:
    case 0x70:
      v6 = 's';
label_fd69:
      *v4 = v6;
      v4[1] = '\0';
      return *a0 + 0x18; // return-dupe
    case 0x53:
      *v4 = 'g';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x59:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
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
      *(char *)(a6 + 0x1c) = 1;
      v2 = 1;
      break;
    case 0x6c:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 4;
      break;
    case 0x6d:
      *v4 = 'o';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x79:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1b) = 1;
      v2 = 2;
    
  }
  *v4 = '\0';
  if (*(unsigned int *)(a6 + 0x20) < v2)
    *(unsigned int *)(a6 + 0x20) = v2;
  return *a0 + 0x18;
}

// Function: sub_fef0 @ 0xfef0
unsigned long sub_fef0(unsigned long *a0,unsigned long a1,char *a2) // return-dupe x2
{
  unsigned long v1;
  char *v10;
  int v11; // r9d
  int v12; // r12d
  char *v13;
  unsigned int v14; // r14d
  long v15;
  char *v16;
  char v2;
  long v3; // rax
  long v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char v7;
  long v8; // stack - 0x48
  char *v9;
  
  v3 = sub_d450(a1,sub_fad0);
  v1 = a0[1];
  *(unsigned long *)(v3 + 0x38) = *a0;
  *(unsigned long *)(v3 + 0x40) = v1;
  v1 = a0[3];
  *(unsigned long *)(v3 + 0x48) = a0[2];
  *(unsigned long *)(v3 + 0x50) = v1;
  v1 = a0[4];
  *(unsigned int *)(v3 + 0x18) = 0x101;
  *(unsigned long *)(v3 + 0x58) = v1;
  *(unsigned int *)(v3 + 0x20) = 0;
  *(unsigned long *)(v3 + 0x38) = 0;
  v2 = *a2;
  if (!v2)
    return 1;
  v4 = v3 + 0x38;
  v10 = a2;
  do {
    while( true ) {
      v7 = v10[1];
      v12 = (int)a2;
      v13 = a2;
      if (v2 != '\\') break;
      if (v7 == 'c') {
        sub_fc20(v4,a2,(int)v10 - v12,1,0,0,v3);
        if (!*(char *)(v3 + 0x1a))
          return 1;
        if (3 <= *(unsigned int *)(v3 + 0x20))
          return 1;
        *(unsigned int *)(v3 + 0x20) = 3;
        return 1;
      }
      if (v7) { // branch-flip
        v14 = (unsigned int)v7;
        v9 = &v10[1];
        if (sub_e720(v14)) { // branch-flip
          v8 = 0;
          *v10 = sub_f270(v9,&v8);
          v15 = v8 + 2;
          v9 = &v10[v8 + 1];
        }
        else {
          v2 = sub_e730(v14);
          if (!v2) {
            v16 = &v10[2];
            error(0,0,dcgettext(NULL,"warning: unrecognized escape `\\%c\'",5),(unsigned long)v14);
            v10 = v9;
            goto label_ff7f;
          }
          *v10 = v2;
          v15 = 2;
        }
      }
      else {
        v15 = 1;
        error(0,0,dcgettext(NULL,"warning: escape `\\\' followed by nothing at all",5));
        v9 = v10;
      }
      v13 = &v10[v15];
      v4 = sub_fc20(v4,a2,((int)v10 - v12) + 1,0,0,0,v3);
      v2 = v9[1];
      a2 = v13;
      v10 = v13;
      v16 = v13;
      if (!v2) goto label_10011;
    }
    if (v2 != '%') // branch-flip
      v16 = &v10[1];
    else {
      if (!v7) {
        error(1,0,dcgettext(NULL,"error: %s at end of format string",5),v10);
        return v6;
      }
      v13 = &v10[1];
      if (v7 != '%') {
        v7 = v10[sub_e820(v10)];
        v13 = &v10[sub_e820(v10)];
      }
      v10 = v13;
      v14 = (unsigned int)v7;
      v15 = sub_e7d0(v14);
      if (v15) { // branch-flip
        if (!v10[v15 + -1]) {
          if (strchr("{[(",v14)) {
            error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(unsigned long)v14); // return-dupe
            return v5;
          }
          if ((v15 != 2) || (v16 = "warning: format directive `%%%c\' should be followed by another character", v10[1])) goto label_100f0;
          goto label_100fc;
        }
        v11 = 0;
        if (v15 == 2)
          v11 = (int)v10[1];
        v13 = &v10[v15];
        v4 = sub_fc20(v4,a2,(int)v10 - v12,2,v14,v11,v3);
        v10 = &v10[v15 + -1];
      }
      else {
        if (strchr("{[(",v14)) {
          error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(unsigned long)v14);
          return v5;
        }
label_100f0:
        v16 = "warning: unrecognized format directive `%%%c\'";
label_100fc:
        v13 = &v10[1];
        error(0,0,dcgettext(NULL,v16,5),(unsigned long)v14);
        v4 = sub_fc20(v4,a2,(int)v13 - v12,0,0,0,v3);
      }
      v16 = v13;
    }
label_ff7f:
    v2 = v10[1];
    a2 = v13;
    v10 = v16;
  } while (v2);
label_10011:
  if (v16 <= v13)
    return 1;
  sub_fc20(v4,v13,(int)v16 - (int)v13,0,0,0,v3);
  return 1;
}

// Function: sub_102e0 @ 0x102e0
void sub_102e0(long a0)
{
  long v1;
  
  if (a0) {
    do {
      v1 = *(long *)(a0 + 0x30);
      sub_1dc40(a0);
      a0 = v1;
    } while (v1);
    return;
  }
}

// Function: sub_10310 @ 0x10310
void sub_10310(char a0)
{
  if (dat_3ee40) {
    if ((dat_3ee38 == '\x01') || (!a0))
      return;
    sub_102e0(dat_3ee40);
    dat_3ee40 = 0;
  }
  dat_3ee38 = a0;
  dat_3ee40 = sub_1dc90(a0);
}

// Function: sub_10360 @ 0x10360
unsigned long sub_10360(long a0) // early-return x2
{
  unsigned long v1; // rax
  unsigned long v2 [19]; // stack - 0xb8
  
  if (*(long *)(a0 + 0x20) != -1)
    return 0;
  sub_d860(v2);
  v1 = (*dat_3e670)(*(unsigned long *)(a0 + 8),v2);
  if (!(int)v1) {
    *(unsigned long *)(a0 + 0x20) = v2[0];
    return v1;
  }
  return 0xffffffff;
}

// Function: sub_103e0 @ 0x103e0
void sub_103e0(long *a0)
{
  char *v1;
  long v2;
  long v3;
  
  v2 = sub_10310(1);
  if (!v2) {
    error(1,0,dcgettext(NULL,"Cannot read mounted file system list",5));
    return;
  }
  v3 = 0;
  do {
    v1 = *(char **)(v2 + 0x18);
    if (strcmp(v1,"ignore")) {
      if ((!sub_10360(v2)) && (*(long *)(v2 + 0x20) == *a0))
        v3 = v2;
    }
    v2 = *(long *)(v2 + 0x30);
  } while (v2);
  if (v3) {
    sub_26d30(*(unsigned long *)(v3 + 0x18));
    dat_3ee50 = 1;
    return;
  }
  dat_3ee50 = 0;
  sub_26d30(dcgettext(NULL,"unknown",5)); // tail-call
}

// Function: sub_104c0 @ 0x104c0
void sub_104c0(long *a0)
{
  if (dat_3ee58) {
    if ((dat_3ee50) && (*a0 == dat_3ee48))
      return;
    free(dat_3ee58);
  }
  dat_3ee48 = *a0;
  dat_3ee58 = (void *)sub_103e0(a0);
}

// Function: sub_10510 @ 0x10510
unsigned long sub_10510(char *a0)
{
  char *v1;
  long v2;
  
  if (strcmp("afs",a0)) {
    v2 = sub_10310(0);
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

// Function: sub_10580 @ 0x10580
void * sub_10580(long *a0)
{
  long v1;
  long v2;
  long v3; // rax
  void *v4; // rax
  unsigned long v5; // stack - 0x48
  long v6;
  long v7;
  void *v8;
  
  v5 = 0;
  v3 = sub_1dc90(0);
  if (v3) { // branch-flip
    v6 = 0;
    v7 = v3;
    v8 = NULL;
    do {
      while( true ) {
        v2 = v6 + 1;
        v1 = v2 * 8;
        v4 = (void *)sub_15710(v8,v1,&v5);
        if (v4) break;
        v4 = NULL;
        free(v8);
        v7 = *(long *)(v7 + 0x30);
        v8 = v4;
        if (!v7) goto label_1063a;
      }
      if (!sub_10360(v7)) {
        *(unsigned long *)((long)v4 + v1 + -8) = *(unsigned long *)(v7 + 0x20);
        v6 = v2;
      }
      v7 = *(long *)(v7 + 0x30);
      v8 = v4;
    } while (v7);
label_1063a:
    sub_102e0(v3);
    if (v4)
      *a0 = v6;
  }
  else {
    v4 = NULL;
    sub_102e0(0);
  }
  return v4;
}

// Function: sub_10690 @ 0x10690
long sub_10690(void)
{
  int v1;
  int *v2;
  int *v3; // rax
  int v4; // edx
  long v5;
  
  v4 = 0;
  v2 = (int *)0x3cd40;
  v5 = dat_3ee68;
  if (!dat_3ee68) {
    while( true ) {
      v5 = *(long *)&v2[2];
      v4 += 1;
      if (!v5) break;
      v3 = &v2[8];
      v1 = *v2;
      v2 = v3;
      if (v1 == 1) {
        dat_3ee68 = (long)v4 * 0x20 + 0x3cd20;
        return dat_3ee68;
      }
    }
  }
  return v5;
}

// Function: sub_106f0 @ 0x106f0
char sub_106f0(void) // early-return
{
  if (dat_3e631)
    return 0;
  return dat_3e630;
}

// Function: sub_10710 @ 0x10710
unsigned int sub_10710(float a0)
{
  unsigned int v1;
  
  if (dat_2ec74 <= a0) {
    v1 = dat_2ec6c;
    if ((dat_2c588 <= a0) && (a0 <= dat_2ec78))
      v1 = dat_2ec70;
    return v1;
  }
  return dat_2ec68;
}

// Function: sub_10760 @ 0x10760
void sub_10760(long a0,unsigned long a1)
{
  sub_10710(a0,a1,(dat_3e648 - a0) / 0x15180); // tail-call
}

// Function: sub_10790 @ 0x10790
unsigned long sub_10790(long a0,int *a1,long *a2)
{
  int v1;
  long v2;
  
  if (a0) {
    v1 = *a1;
    v2 = *(long *)(a0 + (long)v1 * 8);
    if (v2) {
      *a2 = v2;
      *a1 = v1 + 1;
      return 1;
    }
  }
  *a2 = 0;
  return 0;
}

// Function: sub_107d0 @ 0x107d0
void sub_107d0(unsigned long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long v1; // stack - 0x18
  
  sub_10790(a0,a1,&v1);
  *a2 = v1;
}

// Function: sub_10810 @ 0x10810
void sub_10810(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_107d0(a1,a2,&v1))
    return;
  dat_3e690 = v1;
}

// Function: sub_10860 @ 0x10860
unsigned long sub_10860(unsigned long *a0,unsigned int *a1)
{
  char *v1;
  
  v1 = (char *)*a0;
  if (*v1 == '+') {
    *a1 = 0;
    *a0 = &v1[1];
    return 1;
  }
  if (*v1 != '-') {
    *a1 = 2;
    return 1;
  }
  *a1 = 1;
  *a0 = &v1[1];
  return 1;
}

// Function: sub_108b0 @ 0x108b0
int * sub_108b0(long a0,int *a1)
{
  int v1;
  long v2;
  
  v2 = dat_3ee70;
  v1 = *a1;
  if (v1 != 2) {
    if (v1 != 1) { // branch-flip
      if (v1) { // branch-flip
        if (!dat_3ee70)
          dat_3ee70 = a0;
      }
      else if (dat_3ee70) {
        if (sub_106f0())
          error(0,0,dcgettext(NULL,"warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments.",5),a0,v2,a0);
      }
    }
    else {
      a1 = NULL;
    }
  }
  return a1;
}

// Function: sub_10940 @ 0x10940
void sub_10940(void)
{
  sub_254c0(); // tail-call
}

// Function: sub_10950 @ 0x10950
unsigned long sub_10950(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if ((v1 != 2) && (v1 != 0x14))
    return (unsigned long)a3;
  if (dat_3e660 & 2)
    __fprintf_chk(stderr,1,"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n",a1);
  v2 = sub_254c0(a0,a1,a2); // tail-call
  return v2;
}

// Function: sub_109e0 @ 0x109e0
unsigned long sub_109e0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_3e5e0) {
    v2 = sub_254c0(a0,a1,a2); // tail-call
    return v2;
  }
  v1 = sub_252a0(a0,a1,a2);
  if (!v1)
    return 0;
  v2 = sub_10950(a0 & 0xffffffff,a1,a2,v1); // tail-call
  return v2;
}

// Function: sub_10a40 @ 0x10a40
unsigned long sub_10a40(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = sub_252a0(a0,a1,a2);
  if (!v1)
    return 0;
  return sub_10950(a0 & 0xffffffff,a1,a2,v1); // tail-call
}

// Function: sub_10a80 @ 0x10a80
void sub_10a80(void)
{
  char *v1; // rdx
  unsigned int v2; // stack - 0x14
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  sub_15e60("find");
  __printf_chk(1,dcgettext(NULL,"Features enabled: ",5));
  __printf_chk(1,"D_TYPE ");
  v1 = "enabled";
  if (!dat_3e678)
    v1 = "disabled";
  __printf_chk(1,"O_NOFOLLOW(%s) ",v1);
  __printf_chk(1,"LEAF_OPTIMISATION ");
  if (1 <= (int)is_selinux_enabled())
    __printf_chk(1,"SELINUX ");
  v2 = 0;
  if (sub_86d0(&v2)) {
    __printf_chk(1,"FTS(");
    if (v2 & 0x200)
      __printf_chk(1,"FTS_CWDFD");
    __printf_chk(1,") ");
  }
  __printf_chk(1,"CBO(level=%d) ",(unsigned long)dat_3e688);
  __printf_chk(1,"\n");
  exit(0); // no-return
}

// Function: sub_10bd0 @ 0x10bd0
void sub_10bd0(void)
{
  sub_d4e0(0); // no-return
}

// Function: sub_10bf0 @ 0x10bf0
unsigned long sub_10bf0(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_10c20 @ 0x10c20
unsigned long sub_10c20(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  *(unsigned int *)(v1 + 0x18) = 1;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_10c50 @ 0x10c50
unsigned long sub_10c50(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  if (!dat_3e620)
    *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned short *)(v1 + 0x18) = 1;
  return 1;
}

// Function: sub_10c80 @ 0x10c80
unsigned long sub_10c80(void)
{
  *(unsigned int *)(sub_d4d0() + 0x24) = 0x3a83126f;
  return 1;
}

// Function: sub_10ca0 @ 0x10ca0
unsigned long sub_10ca0(void)
{
  long *v1; // rax
  unsigned int v2;
  
  v1 = (long *)sub_d4d0();
  *(unsigned int *)&v1[3] = 0;
  v2 = dat_2ec7c;
  if ((void *)*v1 != sub_9640)
    v2 = dat_2ec80;
  *(unsigned int *)((long)v1 + 0x24) = v2;
  return 1;
}

// Function: sub_10ce0 @ 0x10ce0
unsigned long sub_10ce0(void)
{
  *(unsigned int *)(sub_d4d0() + 0x24) = 0x3c23d70a;
  return 1;
}

// Function: sub_10d00 @ 0x10d00
unsigned long sub_10d00(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  dat_3e620 = 0;
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_10d30 @ 0x10d30
char * sub_10d30(unsigned long a0,unsigned long a1,unsigned long a2,void *a3)
{
  char v1;
  unsigned long v10; // r13
  float v11;
  unsigned int v2;
  unsigned int v3;
  long v4;
  char v5; // al
  unsigned int v6;
  char *v7; // rax
  char *v8; // stack - 0x48
  char *v9; // r12
  
  v9 = "-xtype";
  if (a3 != sub_99e0)
    v9 = "-type";
  v6 = sub_107d0(a0,a1,&v8);
  v3 = v6;
  v10 = (unsigned long)v6;
  if (v5) {
    if (!*v8) {
      v7 = dcgettext(NULL,"Arguments to %s should contain at least one letter",5);
      error(1,0,v7,v9);
      return v7;
    }
    v7 = (long)sub_d450(a2,a3,v8);
    v4 = (long)v7;
    v6 = 0;
    if (a3 == sub_99e0)
      v6 = v3;
    *(unsigned int *)(v4 + 0x24) = 0;
    v2 = 0;
    if (a3 != sub_99e0)
      v2 = v3;
    *(char *)(v4 + 0x1a) = v5;
    *(char *)(v4 + 0x1b) = (char)v2;
    v1 = *v8;
    while (v1) {
      switch(v1) {
        case 0x44:
          v7 = dcgettext(NULL,"%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.",5);
          error(1,0,v7,v9,0x44);
          return v7;
        default:
          v7 = dcgettext(NULL,"Unknown argument to %s: %c",5);
          error(1,0,v7,v9,(unsigned long)(unsigned int)(int)v1);
          return v7;
        case 0x62:
          v7 = NULL;
          v11 = dat_2ec84;
          break;
        case 99:
          v7 = (char *)0x1;
          v11 = dat_2ec88;
          break;
        case 100:
          v7 = (char *)0x2;
          v11 = dat_2ec8c;
          break;
        case 0x66:
          v7 = (char *)0x3;
          v11 = dat_2ec90;
          break;
        case 0x6c:
          v7 = (char *)0x4;
          v11 = dat_2ec94;
          break;
        case 0x70:
          v7 = (char *)0x5;
          v11 = dat_2ec98;
          break;
        case 0x73:
          v7 = (char *)0x6;
          v11 = dat_2ec9c;
        
      }
      if (*(char *)(v4 + 0x38 + (long)v7)) {
        v7 = dcgettext(NULL,"Duplicate file type \'%c\' in the argument list to %s.",5);
        error(1,0,v7,(unsigned long)(unsigned int)(int)v1,v9);
        return v7;
      }
      *(float *)(v4 + 0x24) = v11 + *(float *)(v4 + 0x24);
      *(char *)(v4 + 0x38 + (long)v7) = 1;
      if (!v8[1]) {
        v7 = (char *)v10;
        return v7;
      }
      if (v8[1] != ',') {
        v7 = dcgettext(NULL,"Must separate multiple arguments to %s using: \',\'",5);
        error(1,0,v7,v9);
        return v7;
      }
      v1 = v8[2];
      v8 = &v8[2];
      if (!v1) {
        v7 = dcgettext(NULL,"Last file type in list argument to %s is missing, i.e., list is ending on: \',\'",5);
        error(1,0,v7,v9);
        return v7;
      }
    }
  }
  v7 = (char *)v10;
  return v7;
}

// Function: sub_10fe0 @ 0x10fe0
void sub_10fe0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_10d30(a1,a2,a0,sub_99e0); // tail-call
}

// Function: sub_11000 @ 0x11000
void sub_11000(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_10d30(a1,a2,a0,sub_9810); // tail-call
}

// Function: sub_11020 @ 0x11020
char * sub_11020(void)
{
  int v1; // eax
  char *v2; // rax
  
  if (dat_3ee60) {
    v2 = (char *)0x1;
    return v2;
  }
  v1 = fnmatch("foo","foo",0);
  if (!v1) {
    v1 = fnmatch("Foo","foo",0);
    if (v1) {
      v1 = fnmatch("Foo","foo",0x10);
      if (!v1) {
        dat_3ee60 = 1;
        v2 = (char *)0x1;
        return v2;
      }
    }
  }
  v2 = dcgettext(NULL,"sanity check of the fnmatch() library function failed.",5);
  error(1,0,v2);
  return v2;
}

// Function: sub_110c0 @ 0x110c0
unsigned int sub_110c0(char *a0,int a1)
{
  unsigned int v1;
  char *v2; // rax
  unsigned int v3;
  
  v2 = strpbrk(a0,"*?[");
  v1 = dat_2ec74;
  v3 = dat_2eca0;
  if ((!v2) && (v3 = v1, a1)) {
    if (strpbrk(a0,"."))
      return dat_2eca0;
  }
  return v3;
}

// Function: sub_11130 @ 0x11130
unsigned long sub_11130(char *a0,char a1)
{
  char *v1; // rax
  
  v1 = strrchr(a0,0x2f);
  if ((v1) && (!v1[1]))
    return sub_b510(a0,a1); // tail-call
  return 1;
}

// Function: sub_11180 @ 0x11180
unsigned int sub_11180(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,void *a4)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x48
  unsigned int v4; // xmm0_da
  
  sub_11020();
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d450(a0,a4,v3);
    *(unsigned short *)(v2 + 0x1a) = 0;
    *(unsigned long *)(v2 + 0x38) = v3;
    v4 = sub_110c0(v3,0);
    *(unsigned int *)(v2 + 0x24) = v4;
    if (dat_3e631)
      return v1;
    if (sub_11130(v3,a4 == sub_91e0))
      return v1;
    error(0,0,dcgettext(NULL,"warning: -%s %s will not match anything because it ends with /.",5),a3,v3);
    *(unsigned int *)(v2 + 0x24) = 0x322bcc77;
  }
  return v1;
}

// Function: sub_11280 @ 0x11280
void sub_11280(void)
{
  sub_11180(); // tail-call
}

// Function: sub_112a0 @ 0x112a0
void sub_112a0(void)
{
  sub_11180(); // tail-call
}

// Function: sub_112c0 @ 0x112c0
void sub_112c0(void)
{
  sub_11180(); // tail-call
}

// Function: sub_112e0 @ 0x112e0
void sub_112e0(void)
{
  sub_11180(); // tail-call
}

// Function: sub_11300 @ 0x11300
char * sub_11300(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned int v2; // eax
  char *v3; // rax
  unsigned int v4;
  char *v5; // stack - 0x38
  unsigned long v6; // r12
  unsigned int v7;
  
  v2 = sub_107d0(a1,a2,&v5);
  v6 = (unsigned long)v2;
  if ((char)v2) {
    v3 = getpwnam(v5);
    endpwent();
    if (v3) // branch-flip
      v4 = *(unsigned int *)((long)v3 + 0x10);
    else {
      v3 = (unsigned long)strspn(v5,"0123456789");
      if ((!v3) || (v5[(long)v3])) {
        if (!*v5) {
          v3 = dcgettext(NULL,"The argument to -user should not be empty",5);
          error(1,0,v3);
          return v3;
        }
        v3 = (unsigned long)sub_24e50(0,dat_3e68c,v5);
        v1 = v3;
        v3 = dcgettext(NULL,"%s is not the name of a known user",5);
        error(1,0,v3,v1);
        return v3;
      }
      v2 = sub_16f40(v5,dat_3e68c);
      v4 = v2;
    }
    v3 = (long)sub_d490(a0,v5);
    *(unsigned int *)((long)v3 + 0x38) = v4;
    v7 = dat_2eca4;
    if (100 <= v4)
      v7 = dat_2ec7c;
    *(unsigned int *)((long)v3 + 0x24) = v7;
  }
  v3 = (char *)v6;
  return v3;
}

// Function: sub_11460 @ 0x11460
unsigned long sub_11460(unsigned long a0)
{
  *(unsigned int *)(sub_d490(a0,0) + 0x24) = 0x38d1b717;
  return 1;
}

// Function: sub_11480 @ 0x11480
unsigned int sub_11480(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x38
  
  sub_11020();
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d490(a0,v3);
    *(unsigned long *)(v2 + 0x38) = v3;
    *(float *)(v2 + 0x24) = (float)sub_110c0(v3,0) * dat_2ec74;
  }
  return v1;
}

// Function: sub_11520 @ 0x11520
unsigned int sub_11520(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d490(a0,v3);
    *(unsigned long *)(v2 + 0x38) = v3;
    *(float *)(v2 + 0x24) = (float)sub_110c0(v3,0) * dat_2ec74;
  }
  return v1;
}

// Function: sub_115b0 @ 0x115b0
char * sub_115b0(unsigned long a0,long a1,int *a2)
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
    if (1 <= (int)is_selinux_enabled()) {
      v3 = (long)sub_d490(a0,0);
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

// Function: sub_11650 @ 0x11650
unsigned long sub_11650(double a0,unsigned long a1,long a2,long a3,long a4,char *a5) // return-dupe
{
  unsigned long v1;
  double v10; // xmm0_qa
  char v2; // al
  unsigned int v3; // eax
  long v4; // rax
  unsigned long v5; // rax
  long v6; // rdx
  unsigned long v7 [2]; // stack - 0x60
  double v8; // stack - 0x50
  double v9; // stack - 0x48
  
  v7[0] = a1;
  v2 = sub_10860(v7,a2 + 4);
  v1 = v7[0];
  if (!v2)
    return 0;
  if (*(int *)(a2 + 4)) { // branch-flip
    if (*(int *)(a2 + 4) == 1)
      *(unsigned int *)(a2 + 4) = 0;
  }
  else {
    *(unsigned int *)(a2 + 4) = 1;
  }
  v3 = sub_26d90(v7[0],0,&v8,dat_3dff0);
  if (!(char)v3)
    return 0;
  v10 = (double)modf(a0 * v8,&v9) * dat_2ecb0;
  if (dat_2ecb0 <= v10)
    __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp"); // no-return
  v4 = (long)((double)a3 - v9);
  *(long *)(a2 + 8) = v4;
  if (a3 < v4 != v9 < 0.0) {
    error(1,0,a5,v1);
    return v5;
  }
  v6 = (long)((double)a4 - v10);
  if (v10 <= (double)a4) {
    *(long *)(a2 + 0x10) = v6;
    return (unsigned long)v3;
  }
  *(long *)(a2 + 0x10) = v6 + 1000000000;
  *(long *)(a2 + 8) = v4 + -1;
  return (unsigned long)v3;
}

// Function: sub_11800 @ 0x11800
char * sub_11800(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1;
  unsigned long v10; // stack - 0x28
  char v2; // al
  unsigned int v3;
  char *v4; // rax
  unsigned long v5; // stack - 0x40
  unsigned long v6; // stack - 0x38
  bool v7; // zf
  unsigned int v8; // xmm0_da
  unsigned long v9; // stack - 0x30
  
  v3 = sub_107d0(a1,a2,&v5);
  v7 = v2 != '\0';
  if (v7) {
    v3 = sub_11650(v5,&v6,0,0,"arithmetic overflow while converting %s days to a number of seconds");
    v1 = v3;
    if (!v2) {
      v4 = dcgettext(NULL,"Invalid argument %s to -used",5);
      error(1,0,v4,v5);
      return v4;
    }
    v4 = (long)sub_d490(a0,v5);
    *(unsigned long *)((long)v4 + 0x38) = v6;
    *(unsigned long *)((long)v4 + 0x40) = v9;
    *(unsigned long *)((long)v4 + 0x48) = v10;
    sub_10710();
    *(unsigned int *)((long)v4 + 0x24) = v8;
    v3 = v1;
  }
  v4 = (char *)(unsigned long)v3;
  return v4;
}

// Function: sub_11920 @ 0x11920
unsigned int sub_11920(unsigned long a0,unsigned long a1,unsigned int *a2,unsigned int a3)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x50
  unsigned int v4; // stack - 0x48
  unsigned int v5;
  unsigned int v6; // stack - 0x44
  unsigned long v7; // stack - 0x40
  unsigned long v8; // stack - 0x38
  
  v5 = *a2;
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v4 = a3;
    v1 = sub_11650(dat_2ecc0,v3,&v4,dat_3e648 + 0x15180,dat_3e650,"arithmetic overflow while converting %s minutes to a number of seconds");
    if ((char)v1) { // branch-flip
      v2 = sub_d490(a0,v3);
      *(unsigned long *)(v2 + 0x38) = CONCAT44(v6,v4);
      *(unsigned long *)(v2 + 0x40) = v7;
      *(unsigned long *)(v2 + 0x48) = v8;
      v5 = sub_10760(v7);
      *(unsigned int *)(v2 + 0x24) = v5;
    }
    else {
      *a2 = v5;
    }
  }
  return v1;
}

// Function: sub_11a10 @ 0x11a10
void sub_11a10(void)
{
  sub_11920(); // tail-call
}

// Function: sub_11a20 @ 0x11a20
void sub_11a20(void)
{
  sub_11920(); // tail-call
}

// Function: sub_11a30 @ 0x11a30
void sub_11a30(void)
{
  sub_11920(); // tail-call
}

// Function: sub_11a40 @ 0x11a40
unsigned long sub_11a40(unsigned long a0,char a1,unsigned long *a2)
{
  char v1 [16];
  
  if (a1 == 'c') {
    v1 = sub_25c50(a0);
    *a2 = SUB168(v1,0);
    a2[1] = SUB168(v1,8);
    return 1;
  }
  if ('d' <= a1) { // branch-flip
    if (a1 == 'm') {
      v1 = sub_25c60(a0);
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return 1;
    }
  }
  else {
    if (a1 == 'B') {
      v1 = sub_25c70();
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return (unsigned long)~a2[1] >> 0x3f;
    }
    if (a1 == 'a') {
      v1 = sub_25c40(a0);
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return 1;
    }
  }
  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime"); // no-return
}

// Function: sub_11ae0 @ 0x11ae0
unsigned long sub_11ae0(long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x20
  char v3 [8];
  
  if ((a0) && (v2 = a0, sub_10860(&v2,a2,0,a1))) {
    v1 = sub_27300(v2,v3,10);
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
  }
  return 0;
}

// Function: sub_11b60 @ 0x11b60
long sub_11b60(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rax
  char *v2; // rcx
  unsigned long v3; // stack - 0x30
  int v4; // stack - 0x34
  unsigned long v5; // stack - 0x28
  char *v6; // r8
  
  v1 = 0;
  if ((sub_107d0(a0,a1,&v3)) && (sub_11ae0(v3,&v5,&v4))) {
    v1 = sub_d490(a2,v3);
    *(int *)(v1 + 0x38) = v4;
    *(unsigned long *)(v1 + 0x40) = v5;
    if (dat_3e660 & 1) {
      __fprintf_chk(stderr,1,"inserting %s\n",*(unsigned long *)(v1 + 8));
      v6 = " >";
      v2 = "gt";
      if (v4) {
        v6 = " <";
        v2 = "lt";
        if (v4 != 1) {
          v6 = " ?";
          if (v4 == 2)
            v6 = " =";
          v2 = "?";
          if (v4 == 2)
            v2 = "eq";
        }
      }
      __fprintf_chk(stderr,1,"    type: %s    %s  ",v2,v6);
      __fprintf_chk(stderr,1,"%lu\n",*(unsigned long *)(v1 + 0x40));
    }
  }
  return v1;
}

// Function: sub_11cb0 @ 0x11cb0
unsigned long sub_11cb0(unsigned long a0,unsigned long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_11b60(a1,a2,a0);
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

// Function: sub_11d20 @ 0x11d20
unsigned long sub_11d20(unsigned long a0,unsigned long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_11b60(a1,a2,a0);
  if (v1) {
    *(unsigned int *)(v1 + 0x24) = 0x358637bd;
    *(unsigned short *)(v1 + 0x1a) = 0;
    *(char *)(v1 + 0x1c) = 1;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}

// Function: sub_11d70 @ 0x11d70
unsigned long sub_11d70(unsigned long a0,unsigned long a1,int *a2) // return-dupe
{
  long v1; // rax
  
  v1 = sub_11b60(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(unsigned long *)(v1 + 0x40) <= 99) {
    *(unsigned int *)(v1 + 0x24) = dat_2eca4;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = dat_2ec7c;
  return 1;
}

// Function: sub_11dd0 @ 0x11dd0
void sub_11dd0(void)
{
  sub_11d70(); // tail-call
}

// Function: sub_11de0 @ 0x11de0
char * sub_11de0(unsigned long a0,long a1,int *a2)
{
  char *v1;
  unsigned long v10; // r15
  unsigned int v11;
  char *v2;
  unsigned int v3; // eax
  char *v4; // rax
  char v5;
  int v6; // stack - 0x54
  long v7; // stack - 0x50
  char v8 [2]; // stack - 0x42
  long v9; // r13
  
  if ((a1) && (v4 = (long)(long)*a2, v2 = *(char **)(a1 + (long)v4 * 8), v2)) {
    v4 = (unsigned long)strlen(v2);
    if (!v3) {
      v4 = dcgettext(NULL,"invalid null argument to -size",5);
      error(1,0,v4);
      return v4;
    }
    v4 = (long)(long)(int)v3;
    v1 = &v2[(long)v4 + -1];
    v5 = *v1;
    if (v5 != 'b') { // branch-flip
      if ('c' <= v5) { // branch-flip
        if (v5 != 'k') { // branch-flip
          if (v5 != 'w') { // branch-flip
            if (v5 != 'c') {
              v4 = dcgettext(NULL,"invalid -size type `%c\'",5); // return-dupe
              error(1,0,v4,(unsigned long)(unsigned int)(int)v5);
              return v4;
            }
            *v1 = '\0';
            v9 = 1;
          }
          else {
            *v1 = '\0';
            v9 = 2;
          }
        }
        else {
          *v1 = '\0';
          v9 = 0x400;
        }
      }
      else if (v5 != 'G') { // branch-flip
        if ('H' <= v5) { // branch-flip
          if (v5 != 'M') {
            v4 = dcgettext(NULL,"invalid -size type `%c\'",5);
            error(1,0,v4,(unsigned long)(unsigned int)(int)v5);
            return v4;
          }
          *v1 = '\0';
          v9 = 0x100000;
        }
        else {
          if (9 < (unsigned char)(v5 - 0x30U)) {
            v4 = dcgettext(NULL,"invalid -size type `%c\'",5);
            error(1,0,v4,(unsigned long)(unsigned int)(int)v5);
            return v4;
          }
          v9 = 0x200;
          v5 = '\0';
        }
      }
      else {
        *v1 = '\0';
        v9 = 0x40000000;
      }
    }
    else {
      *v1 = '\0';
      v9 = 0x200;
    }
    v3 = sub_11ae0(v2,&v7,&v6);
    v10 = (unsigned long)v3;
    if (!(char)v3) {
      v8[1] = 0;
      v8[0] = v5;
      v4 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
      error(1,0,v4,v2,v8);
      return v4;
    }
    v4 = (long)sub_d490(a0,v2);
    *(int *)((long)v4 + 0x3c) = (int)v9;
    *(int *)((long)v4 + 0x38) = v6;
    if (v6) { // branch-flip
      v11 = dat_2ec68;
      if ((v6 == 1) && (v11 = dat_2ec80, (unsigned long)(v9 * v7) <= 0x5000))
        v11 = dat_2ec74;
    }
    else {
      v11 = dat_2ec74;
      if ((unsigned long)(v9 * v7) <= 0x5000)
        v11 = dat_2ec80;
    }
    *(long *)((long)v4 + 0x40) = v7;
    *(unsigned short *)((long)v4 + 0x1a) = 1;
    *(unsigned int *)((long)v4 + 0x24) = v11;
    *a2 = *a2 + 1;
  }
  else {
    v10 = 0;
  }
  v4 = (char *)v10;
  return v4;
}

// Function: sub_12080 @ 0x12080
unsigned int sub_12080(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  unsigned int v1; // eax
  int v2; // eax
  unsigned long v3; // stack - 0x38
  
  v1 = sub_107d0(a0,a1,&v3);
  if ((char)v1) { // branch-flip
    *a3 = v3;
    v2 = (*dat_3e670)(v3,a2);
    if (v2)
      sub_e310(*__errno_location(),v3); // no-return
  }
  else {
    *a3 = 0;
  }
  return v1;
}

// Function: sub_12110 @ 0x12110
unsigned int sub_12110(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [88];
  unsigned long v4; // stack - 0xd0
  char v5 [16];
  unsigned long v6; // stack - 0x70
  
  sub_d860(v3);
  v1 = sub_12080(a1,a2,v3,&v4);
  if ((char)v1) {
    v2 = sub_d490(a0,v4);
    v5 = sub_25c60(v3);
    *(unsigned long *)(v2 + 0x38) = 3;
    *(long *)(v2 + 0x40) = SUB168(v5,0);
    *(long *)(v2 + 0x48) = SUB168(v5,8);
    *(unsigned int *)(v2 + 0x24) = sub_10760(v6);
  }
  return v1;
}

// Function: sub_121d0 @ 0x121d0
unsigned int sub_121d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [88];
  unsigned long v4; // stack - 0xd0
  char v5 [16];
  unsigned long v6; // stack - 0x70
  
  sub_d860(v3);
  v1 = sub_12080(a1,a2,v3,&v4);
  if ((char)v1) {
    v2 = sub_d490(a0,v4);
    *(unsigned int *)(v2 + 0x38) = 2;
    v5 = sub_25c60(v3);
    *(unsigned int *)(v2 + 0x3c) = 0;
    *(long *)(v2 + 0x40) = SUB168(v5,0);
    *(long *)(v2 + 0x48) = SUB168(v5,8);
    *(unsigned int *)(v2 + 0x24) = sub_10760(v6);
  }
  return v1;
}

// Function: sub_122a0 @ 0x122a0
unsigned int sub_122a0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [88];
  unsigned long v4; // stack - 0xd0
  char v5 [16];
  unsigned long v6; // stack - 0x70
  
  sub_d860(v3);
  v1 = sub_12080(a1,a2,v3,&v4);
  if ((char)v1) {
    v2 = sub_d490(a0,v4);
    *(unsigned int *)(v2 + 0x38) = 0;
    v5 = sub_25c60(v3);
    *(unsigned int *)(v2 + 0x3c) = 0;
    *(long *)(v2 + 0x40) = SUB168(v5,0);
    *(long *)(v2 + 0x48) = SUB168(v5,8);
    *(unsigned int *)(v2 + 0x24) = sub_10760(v6);
  }
  return v1;
}

// Function: sub_12370 @ 0x12370
unsigned int sub_12370(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v10; // stack - 0xd0
  int v2;
  int v3; // eax
  long v4; // rax
  long v5; // stack - 0x168
  unsigned long v6; // stack - 0x170
  long v7; // stack - 0xd8
  long v8; // stack - 0x160
  unsigned int v9; // stack - 0x150
  
  sub_d860(&v5);
  v1 = sub_12080(a1,a2,&v5,&v6);
  if (!(char)v1)
    return v1;
  sub_d860(&v7);
  if (dat_3e668) { // branch-flip
label_12440:
    v2 = sub_15cc0(v6,0);
  }
  else {
    if (!dat_3e678) {
      if ((v9 & 0xf000) == 0xa000) {
        v2 = -2;
        goto label_123c8;
      }
      goto label_12440;
    }
    v2 = sub_15cc0(v6,0x20000);
  }
  if (0 <= v2) {
    if ((fstat(v2,&v7)) || (v3 = (*dat_3e670)(v6,&v5), v3))
      sub_e310(*__errno_location(),v6); // no-return
    if ((dat_3e668) || (dat_3e678)) {
      v5 = v7;
      v8 = v10;
    }
    else if ((((v9 & 0xf000) == 0xa000) || (v5 != v7)) || (v8 != v10)) {
      close(v2);
      v2 = -1;
    }
  }
label_123c8:
  v4 = sub_d490(a0,v6);
  *(int *)(v4 + 0x48) = v2;
  *(long *)(v4 + 0x38) = v8;
  *(unsigned int *)(v4 + 0x24) = 0x3c23d70a;
  *(long *)(v4 + 0x40) = v5;
  *(unsigned short *)(v4 + 0x1a) = 1;
  return v1;
}

// Function: sub_125d0 @ 0x125d0
unsigned long sub_125d0(unsigned long a0,unsigned long a1,unsigned long a2,int a3)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long *v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  char *v7; // stack - 0x38
  
  v1 = sub_107d0(a0,a1,&v7);
  if ((char)v1) {
    v2 = sub_d450(a2,sub_96f0,v7);
    *(unsigned short *)(v2 + 0x1a) = 0;
    v3 = (unsigned long *)sub_26990(0x40);
    *(unsigned long **)(v2 + 0x38) = v3;
    v3[1] = 100;
    v4 = sub_26990(100);
    v3[4] = 0;
    *v3 = v4;
    re_set_syntax((long)a3);
    v3[3] = (long)a3;
    v3[5] = 0;
    v5 = re_compile_pattern(v7,strlen(v7),v3);
    if (v5) {
      error(1,0,dcgettext(NULL,"failed to compile regular expression \'%s\': %s",5),v7,v5);
      return v6;
    }
    *(unsigned int *)(v2 + 0x24) = sub_110c0(v7,1);
  }
  return (unsigned long)v1;
}

// Function: sub_126f0 @ 0x126f0
void sub_126f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_125d0(a1,a2,a0,dat_3e67c); // tail-call
}

// Function: sub_12710 @ 0x12710
void sub_12710(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_125d0(a1,a2,a0,dat_3e67c | 0x400000); // tail-call
}

// Function: sub_12740 @ 0x12740
unsigned long sub_12740(void *a0) // early-return
{
  int v1; // eax
  
  v1 = fileno(a0);
  if (v1 == -1)
    return 0;
  v1 = isatty(v1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 != 0);
}

// Function: sub_12770 @ 0x12770
void sub_12770(char *a0,unsigned long *a1)
{
  unsigned long v1;
  long v2; // rax
  char *v3; // rax
  
  *a1 = 0;
  a1[4] = sub_24ac0(0);
  if (!strcmp(a0,"/dev/stderr")) {
    a1[1] = stderr;
    v3 = dcgettext(NULL,"standard error",5);
    v1 = a1[1];
    a1[2] = v3;
    *(char *)&a1[3] = sub_12740(v1);
    return;
  }
  if (strcmp(a0,"/dev/stdout")) {
    v2 = sub_e580(dat_3e610,a0);
    a1[2] = a0;
    a1[1] = v2;
    if (v2) {
      *(char *)&a1[3] = sub_12740(v2);
      return;
    }
    sub_e410(*__errno_location(),a0); // no-return
  }
  a1[1] = stdout;
  v3 = dcgettext(NULL,"standard output",5);
  v1 = a1[1];
  a1[2] = v3;
  *(char *)&a1[3] = sub_12740(v1);
}

// Function: sub_12870 @ 0x12870
void sub_12870(unsigned long a0)
{
  sub_12770("/dev/stdout",a0); // tail-call
}

// Function: sub_12880 @ 0x12880
void sub_12880(unsigned long a0,unsigned long a1,unsigned int *a2) // return-dupe x2
{
  unsigned int v1;
  unsigned long v2; // stack - 0x60
  char v3 [40];
  
  v1 = *a2;
  if (!sub_10790(a1,a2,&v2))
    return;
  sub_12870(v3);
  if (sub_fef0(v3,a0,v2))
    return;
  *a2 = v1;
}

// Function: sub_12910 @ 0x12910
unsigned long sub_12910(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_d4d0(a0);
  *(unsigned int *)(v1 + 0x18) = 0x101;
  sub_12870(v1 + 0x38);
  return 1;
}

// Function: sub_12940 @ 0x12940
unsigned long sub_12940(unsigned long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_d490(a0,a1);
  if (a1) // branch-flip
    sub_12770(a1,v1 + 0x38);
  else {
    sub_12870(v1 + 0x38);
  }
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_12990 @ 0x12990
void sub_12990(unsigned long a0)
{
  sub_12940(a0,0); // tail-call
}

// Function: sub_129a0 @ 0x129a0
void sub_129a0(unsigned long a0,unsigned long a1,int *a2)
{
  unsigned long v1; // stack - 0x28
  
  if ((sub_107d0(a1,a2,&v1)) && (!sub_12940(a0,v1)))
    *a2 = *a2 + -1;
}

// Function: sub_12a10 @ 0x12a10
unsigned long sub_12a10(unsigned long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_d4d0(a0);
  if (a1) // branch-flip
    sub_12770(a1,v1 + 0x38);
  else {
    sub_12870(v1 + 0x38);
  }
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  *(unsigned short *)(v1 + 0x18) = 0x101;
  return 1;
}

// Function: sub_12a60 @ 0x12a60
void sub_12a60(unsigned long a0)
{
  sub_12a10(a0,0); // tail-call
}

// Function: sub_12a70 @ 0x12a70
void sub_12a70(unsigned long a0,unsigned long a1,int *a2)
{
  unsigned long v1; // stack - 0x28
  
  if ((sub_107d0(a1,a2,&v1)) && (!sub_12a10(a0,v1)))
    *a2 = *a2 + -1;
}

// Function: sub_12ae0 @ 0x12ae0
unsigned long sub_12ae0(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x68
  unsigned long v3; // stack - 0x60
  char v4 [40];
  unsigned int v5;
  
  v5 = *a2;
  if ((sub_107d0(a1,a2,&v2)) && (sub_10790(a1,a2,&v3))) {
    sub_12770(v2,v4);
    v5 = *a2;
    v1 = sub_fef0(v4,a0,v3);
    if ((char)v1)
      return v1;
  }
  *a2 = v5;
  return 0;
}

// Function: sub_12b90 @ 0x12b90
unsigned int sub_12b90(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d490(a0,v3);
    sub_12770(v3,v2 + 0x38);
    *(unsigned int *)(v2 + 0x18) = 0x101;
    *(unsigned int *)(v2 + 0x24) = 0x3f800000;
  }
  return v1;
}

// Function: sub_12c20 @ 0x12c20
unsigned long sub_12c20(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned int v10; // r14d
  unsigned int v11; // stack - 0x54
  unsigned int v2; // eax
  void *v3; // rax
  long v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  char *v8; // stack - 0x48
  char *v9;
  
  v1 = sub_107d0(a1,a2,&v8);
  if ((char)v1) {
    if (*v8 != '-') { // branch-flip
      if (*v8 != '/') { // branch-flip
        v10 = 2;
        v11 = dat_2ec68;
        v9 = v8;
      }
      else {
        v11 = dat_2ec6c;
        v9 = &v8[1];
        v10 = 1;
      }
    }
    else {
      v9 = &v8[1];
      v10 = 0;
      v11 = dat_2ec7c;
    }
    v3 = (void *)sub_1d600(v9);
    if ((!v3) || ((*v8 == '+' && ((unsigned char)(v8[1] - 0x30U) <= 7)))) {
      v6 = sub_24e50(0,dat_3e68c,v8);
      error(1,0,dcgettext(NULL,"invalid mode %s",5),v6);
      return v7;
    }
    v2 = sub_1d9d0(0,0,0,v3,0);
    v4 = sub_1d9d0(0,1,0,v3,0);
    free(v3);
    if ((*v8 == '/') && (!(int)v4 && !v2)) {
      v10 = 0;
      error(0,0,dcgettext(NULL,"warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.",5),v8);
      v11 = dat_2eca8;
    }
    v5 = sub_d490(a0,v8);
    *(unsigned int *)(v5 + 0x38) = v10;
    *(unsigned long *)(v5 + 0x3c) = (unsigned long)v2 | v4 << 0x20;
    *(unsigned int *)(v5 + 0x24) = v11;
  }
  return (unsigned long)v1;
}

// Function: sub_12e20 @ 0x12e20
void sub_12e20(unsigned long a0) // return-dupe
{
  char v1;
  char *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x38
  long v6; // stack - 0x40
  
  v2 = getenv("PATH");
  if (!v2)
    return;
  sub_170d0(v2,":",1,&v6,&v5);
  while (v5) {
    v1 = v2[v6];
    if ((v5 == 1) && (v1 == '.')) break;
    if (v1 != '/') {
      v3 = strndup(&v2[v6],v5);
      if (!v3)
        v3 = &v2[v6];
      v4 = sub_dfb0(0,v3);
      error(1,0,dcgettext(NULL,"The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH",5),v4,a0);
      return;
    }
    if (!sub_170d0(v2,":",0,&v6,&v5))
      return;
  }
  error(1,0,dcgettext(NULL,"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)",5),a0);
}

// Function: sub_12f60 @ 0x12f60
unsigned long sub_12f60(unsigned long a0,long a1,long a2,int *a3)
{
  int v1;
  unsigned long v10; // rax
  long v11;
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14; // rax
  long *v15;
  char v16 [24];
  char *v17;
  int v18;
  int v19; // ebp
  void *v2;
  unsigned long *v20;
  long *v21;
  char *v22; // stack - 0xa8
  bool v3;
  void *v4;
  void *v5; // rax
  long v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v2 = *(void **)(a1 + 0x18);
  if ((!a2) || (!*(long *)(a2 + (long)*a3 * 8)))
    return 0;
  v5 = (void *)sub_d450(a1,v2,"(some -exec* arguments)");
  *(unsigned int *)((long)v5 + 0x18) = 0x101;
  *(unsigned long *)((long)v5 + 0xf8) = 0;
  if (v2 != sub_94e0 && v2 != sub_9530) {
    *(char *)((long)v5 + 0xf4) = 0;
    v4 = sub_8ff0;
label_13056:
    if (v2 != v4) {
      if (!dat_3e5c0)
        __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok"); // no-return
      *(long *)((long)v5 + 0xf8) = dat_3e5c0;
      goto label_13074;
    }
  }
  else {
    dat_3e698 = 1;
    *(char *)((long)v5 + 0xf4) = 1;
    if (v2 != sub_8ff0) {
      v4 = sub_9530;
      goto label_13056;
    }
  }
  dat_3e62e = 0;
  sub_12e20(a0);
label_13074:
  *(char *)((long)v5 + 0x38) = 0;
  v1 = *a3;
  v6 = (long)v1 * 8;
  v21 = (long *)(a2 + v6);
  v17 = (char *)*v21;
  v18 = v1;
  if (v17) {
    v20 = (unsigned long *)(a2 + 8 + v6);
    v22 = NULL;
    v19 = 0;
    v3 = 0;
    v15 = v21;
    do {
      if (*v17 != ';') { // branch-flip
        if ((((*v17 == '+') && (v2 != sub_94e0 && v2 != sub_9530)) && (!v17[1])) && (v3)) {
          *(char *)((long)v5 + 0x38) = 1;
label_1315e:
          if ((v1 != v18) && (*v15)) {
            if (*(char *)((long)v5 + 0x38)) {
              v17 = "dir";
              if (v2 != sub_8ff0)
                v17 = "";
              if (1 < v19) {
                error(1,0,dcgettext(NULL,"Only one instance of {} is supported with -exec%s ... +",5),v17);
                return v14;
              }
              if (strlen(v22) != 2) {
                __snprintf_chk(v16,0x13,1,0x13,"-exec%s ... {} +",v17);
                v7 = sub_24e50(2,dat_3e68c,v22);
                v8 = sub_24e50(1,dat_3e68c,"{}");
                v9 = sub_24e50(0,dat_3e68c,v16);
                error(1,0,dcgettext(NULL,"In %s the %s must appear by itself, but you specified %s",5),v9,v8,v7);
                return v10;
              }
            }
            v6 = (long)v5 + 0x40;
            if ((unsigned int)(sub_14ca0(v6,0x800) - 1) < 2) {
              error(1,0,dcgettext(NULL,"The environment is too large for exec().",5));
              return v13;
            }
            sub_14d80(v6);
            v19 = v18 - v1;
            v11 = (long)v5 + 0x98;
            *(void **)((long)v5 + 0x80) = sub_a610;
            if (*(char *)((long)v5 + 0x38)) { // branch-flip
              *(unsigned long *)((long)v5 + 0xe8) = 0;
              *(unsigned long *)((long)v5 + 0x70) = 0;
              *(long *)((long)v5 + 0x78) = (long)(v19 + -1);
              *(unsigned long *)((long)v5 + 0x68) = 0;
              *(unsigned long *)((long)v5 + 0x88) = 0;
              *(unsigned long *)((long)v5 + 0x90) = 0;
              sub_14db0(v6,v11,(long)v5 + 0x38);
              if (v1 < v18 + -1) {
                do {
                  v17 = (char *)*v21;
                  v21 = &v21[1];
                  sub_15000(v6,v11,v17,strlen(v17) + 1,0,0,1);
                } while (v21 != (long *)(a2 + 8 + ((unsigned long)(unsigned int)((v18 - v1) - 2) + (long)v1) * 8));
              }
            }
            else {
              *(int *)((long)v5 + 0xf0) = v19;
              *(char **)((long)v5 + 0x70) = "{}";
              *(unsigned long *)((long)v5 + 0x68) = strlen("{}");
              *(unsigned long *)((long)v5 + 0x88) = 0;
              *(unsigned long *)((long)v5 + 0x90) = 0;
              *(unsigned long *)((long)v5 + 0xe8) = sub_26990((long)v19 << 3);
              sub_14db0(v6,v11,(long)v5 + 0x38);
              v1 = *(int *)((long)v5 + 0xf0);
              if (1 <= v1) {
                v6 = *(long *)((long)v5 + 0xe8);
                v11 = 0;
                do {
                  *(long *)(v6 + v11 * 8) = v21[v11];
                  v11 += 1;
                } while (v11 != v1);
              }
            }
            if (*v15) {
              *a3 = v18 + 1;
              return 1;
            }
            *a3 = v18;
            return 1;
          }
          goto label_1328a;
        }
      }
      else if (!v17[1]) goto label_1315e;
      if (sub_1caf0(v17,"{}")) { // branch-flip
        v19 += 1;
        if ((!v18) && ((v2 == sub_8ff0 || (v2 == sub_9530)))) {
          error(1,0,dcgettext(NULL,"You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.",5));
          return v12;
        }
        v3 = 1;
        v22 = v17;
      }
      else {
        v3 = 0;
      }
      v17 = (char *)*v20;
      v18 += 1;
      v20 = &v20[1];
      v15 = &v15[1];
    } while (v17);
  }
label_1328a:
  *a3 = v18;
  free(v5);
  return 0;
}

// Function: sub_13550 @ 0x13550
void sub_13550(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_12f60("-okdir",a0,a1,a2); // tail-call
}

// Function: sub_13570 @ 0x13570
void sub_13570(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_12f60("-ok",a0,a1,a2); // tail-call
}

// Function: sub_13590 @ 0x13590
void sub_13590(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_12f60("-execdir",a0,a1,a2); // tail-call
}

// Function: sub_135b0 @ 0x135b0
void sub_135b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_12f60("-exec",a0,a1,a2); // tail-call
}

// Function: sub_135d0 @ 0x135d0
unsigned long sub_135d0(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 0x200000003;
  *v1 = sub_88c0;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}

// Function: sub_13610 @ 0x13610
unsigned long sub_13610(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 0x300000003;
  *v1 = sub_8870;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}

// Function: sub_13650 @ 0x13650
unsigned long sub_13650(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 0x100000003;
  *v1 = sub_8cd0;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  *(unsigned int *)((long)v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_13690 @ 0x13690
unsigned long sub_13690(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 5;
  *v1 = sub_8c30;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}

// Function: sub_136c0 @ 0x136c0
unsigned long sub_136c0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // rdx
  
  if (sub_106f0()) {
    if (strchr(v4,0x2f)) {
      v1 = sub_dfb0(2,a1);
      v2 = sub_dfb0(1,"/");
      v3 = sub_dfb0(0,a0);
      error(0,0,dcgettext(NULL,"warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?",5),v3,v2,v1);
    }
  }
  return 1;
}

// Function: sub_13750 @ 0x13750
unsigned int sub_13750(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned int v1;
  unsigned int v2; // eax
  long v3; // rax
  unsigned long v4; // stack - 0x38
  
  v1 = *a2;
  v2 = sub_107d0(a1,a2,&v4);
  if ((char)v2) {
    sub_11020();
    v2 = sub_136c0("-name","-wholename",v4);
    if ((char)v2) { // branch-flip
      v3 = sub_d490(a0,v4);
      *(unsigned short *)(v3 + 0x1a) = 0;
      *(unsigned long *)(v3 + 0x38) = v4;
      *(unsigned int *)(v3 + 0x24) = sub_110c0(v4,0);
    }
    else {
      *a2 = v1;
    }
  }
  return v2;
}

// Function: sub_13810 @ 0x13810
unsigned int sub_13810(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  sub_11020();
  if ((sub_107d0(a1,a2,&v3)) && (v1 = sub_136c0("-iname","-iwholename",v3), (char)v1)) {
    v2 = sub_d490(a0,v3);
    *(unsigned short *)(v2 + 0x1a) = 0;
    *(unsigned long *)(v2 + 0x38) = v3;
    *(unsigned int *)(v2 + 0x24) = sub_110c0(v3,0);
  }
  else {
    v1 = 0;
  }
  return v1;
}

// Function: sub_138d0 @ 0x138d0
unsigned long sub_138d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned long v10; // rax
  char *v11; // stack - 0x38
  int v2; // eax
  unsigned int v3;
  void *v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v1 = sub_107d0(a1,a2,&v11);
  if ((char)v1) {
    v4 = getgrnam(v11);
    endgrent();
    if (v4) // branch-flip
      v3 = *(unsigned int *)((long)v4 + 0x10);
    else {
      v2 = (int)strspn(v11,"0123456789");
      if (!v2) {
        if (!*v11) {
          error(1,0,dcgettext(NULL,"argument to -group is empty, but should be a group name",5));
          return v7;
        }
        v8 = sub_24e50(0,dat_3e68c,v11);
        error(1,0,dcgettext(NULL,"%s is not the name of an existing group",5),v8);
        return v6;
      }
      if (v11[v2]) {
        v8 = sub_24e50(1,dat_3e68c,&v11[v2]);
        v9 = sub_24e50(0,dat_3e68c,v11);
        error(1,0,dcgettext(NULL,"%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s",5),v9,v8);
        return v10;
      }
      v3 = sub_16f40(v11);
    }
    v5 = sub_d490(a0,v11);
    *(unsigned int *)(v5 + 0x38) = v3;
    v3 = dat_2eca4;
    if (100 <= *(unsigned long *)(v5 + 0x40))
      v3 = dat_2ec7c;
    *(unsigned int *)(v5 + 0x24) = v3;
  }
  return (unsigned long)v1;
}

// Function: sub_13a70 @ 0x13a70
char sub_13a70(char *a0)
{
  char *v1; // rax
  char v2 [152];
  char v3; // xmm0_ba
  undefined4 v4;
  
  v4 = dat_2c588;
  if (!stat("/",v2)) {
    v1 = (char *)sub_104c0(dat_2c588,v2,"/");
    v4 = s_2ecab._1_4_;
    if (strcmp(a0,v1))
      v4 = dat_2ec6c;
  }
  v3 = (char)v4;
  return v3;
}

// Function: sub_13b10 @ 0x13b10
unsigned long sub_13b10(void) // ternary
{
  int *v1; // rax
  long v2; // rdx
  
  if (dat_3e658)
    return 1;
  dat_3e648 += 0x15180;
  dat_3e650 = 0;
  v1 = localtime((void *)0x3e648);
  v2 = (v1) ? (long)(v1[1] * 0x3c + *v1 + v1[2] * 0xe10) : dat_3e648 % 0x15180; // branch-flip
  dat_3e658 = 1;
  dat_3e648 -= v2;
  return 1;
}

// Function: sub_13bc0 @ 0x13bc0
unsigned int sub_13bc0(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned int v1;
  char *v10; // r8
  int v11; // stack - 0x54
  unsigned long v12; // stack - 0x50
  unsigned long v13; // stack - 0x48
  unsigned int v2; // eax
  long v3;
  unsigned long v4; // rax
  char *v5; // rcx
  unsigned long v6; // stack - 0x68
  int v7; // stack - 0x6c
  unsigned int v8; // stack - 0x58
  long v9; // stack - 0x60
  
  dcgettext(NULL,"arithmetic overflow while converting %s days to a number of seconds",5);
  v1 = *a2;
  v2 = sub_107d0(a1,a2,&v6);
  if ((char)v2) {
    v3 = dat_3e648;
    if ((sub_10860(&v6,&v7)) && (v7 == 1))
      v3 += 0x1517f;
    v2 = sub_11650(dat_2ecb8,v6,&v8,v3);
    if ((char)v2) { // branch-flip
      v3 = sub_d490(a0,v6);
      *(unsigned long *)(v3 + 0x38) = CONCAT44(v11,v8);
      *(unsigned long *)(v3 + 0x40) = v12;
      *(unsigned long *)(v3 + 0x48) = v13;
      *(unsigned int *)(v3 + 0x24) = sub_10760(v12);
      if (dat_3e660 & 1) {
        __fprintf_chk(stderr,1,"inserting %s\n",*(unsigned long *)(v3 + 8));
        if (v11) { // branch-flip
          if (v11 != 1) { // branch-flip
            v10 = " ?";
            v5 = "?";
            if (v11 == 2) {
              __fprintf_chk(stderr,1,"    type: %s    %s  ","eq",">=");
              v9 = *(long *)(v3 + 0x40);
              v4 = ctime(&v9);
              __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v3 + 0x40),v4);
              v9 = *(long *)(v3 + 0x40) + 0x15180;
              __fprintf_chk(stderr,1,"                 <  %lu %s",v9,ctime(&v9));
              return v2;
            }
          }
          else {
            v10 = " <";
            v5 = "lt";
          }
        }
        else {
          v10 = " >";
          v5 = "gt";
        }
        __fprintf_chk(stderr,1,"    type: %s    %s  ",v5,v10);
        v9 = *(long *)(v3 + 0x40);
        v4 = ctime(&v9);
        __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v3 + 0x40),v4);
      }
    }
    else {
      *a2 = v1;
    }
  }
  return v2;
}

// Function: sub_13e80 @ 0x13e80
unsigned long sub_13e80(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b990(a0,0);
  v1[2] = 0x400000002;
  *v1 = sub_92f0;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}

// Function: sub_13ec0 @ 0x13ec0
unsigned long sub_13ec0(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b990(a0,0);
  v1[2] = 4;
  *v1 = sub_9590;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}

// Function: sub_13f00 @ 0x13f00
unsigned long sub_13f00(unsigned long a0,long a1,int *a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  char v1;
  unsigned long v10; // rax
  int *v11; // rax
  char v12 [6]; // stack - 0x46
  char v13 [146];
  bool v14; // r9b
  unsigned long v15; // r9
  unsigned int v16; // eax
  char v2;
  char *v3;
  void *v4;
  int v5;
  long *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  if (a1) {
    v5 = *a2;
    v3 = *(char **)(a1 + (long)v5 * 8);
    if ((v3) && (strlen(v3) == 8)) {
      builtin_strncpy(v12,"aBcmt",6);
      if (strncmp("-newer",v3,6))
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY"); // no-return
      v1 = v3[6];
      v2 = v3[7];
      v14 = v1 == 'B' || v2 == 'B';
      v15 = CONCAT71((undefined7)((unsigned long)a5 >> 8),v14);
      if (v14) {
        error(0,0,dcgettext(NULL,"This system does not provide a way to find the birth time of a file.",5));
        v15 = 0;
      }
      else if (v1 != 't') {
        v15 = 0;
        if (strchr(v12,(int)v1)) {
          v15 = 0;
          if (strchr(v12,(int)v2)) {
            if (!*(long *)(a1 + 8 + (long)v5 * 8)) {
              v9 = sub_24e50(0,dat_3e68c,v3);
              error(1,0,dcgettext(NULL,"The %s test needs an argument",5),v9);
              return v8;
            }
            *a2 = v5 + 1;
            v6 = (long *)sub_d490(a0);
            if (v1 != 'c') { // branch-flip
              if (v1 != 'm') { // branch-flip
                if (v1 != 'a') {
                  if (!strchr(v12,(int)v1))
                    __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY"); // no-return
                  __assert_fail("0","parser.c",0x64c,"parse_newerXY"); // no-return
                }
                v16 = 0;
              }
              else {
                v16 = 3;
              }
            }
            else {
              v16 = 2;
            }
            *(unsigned int *)&v6[7] = v16;
            if (v2 != 't') { // branch-flip
              sub_d860(v13);
              v5 = (*dat_3e670)(*(unsigned long *)(a1 + (long)*a2 * 8),v13);
              if (v5) {
                v11 = __errno_location();
                sub_e310(*v11,*(unsigned long *)(a1 + (long)*a2 * 8)); // no-return
              }
              if (!sub_11a40(v13,(int)v2,&v6[8])) {
                v9 = sub_dfb0(0,*(unsigned long *)(a1 + (long)*a2 * 8));
                error(1,0,dcgettext(NULL,"Cannot obtain birth time of file %s",5),v9);
                return v10;
              }
            }
            else {
              v9 = *(unsigned long *)(a1 + (long)*a2 * 8);
              if (!sub_22c40(&v6[8],v9,0x3e638)) {
                v9 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + (long)*a2 * 8));
                error(1,0,dcgettext(NULL,"I cannot figure out how to interpret %s as a date or time",5),v9);
                return v7;
              }
            }
            *(unsigned int *)((long)v6 + 0x3c) = 0;
            v16 = sub_10760(v6[8]);
            v4 = (void *)*v6;
            *(unsigned int *)((long)v6 + 0x24) = v16;
            *a2 = *a2 + 1;
            if (!v4)
              __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY"); // no-return
            if (v4 != sub_9370)
              __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"); // no-return
            v15 = (unsigned long)*(unsigned char *)((long)v6 + 0x1a);
            if (!*(unsigned char *)((long)v6 + 0x1a))
              __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY"); // no-return
          }
        }
      }
      return v15 & 0xffffffff; // return-dupe
    }
  }
  v15 = 0;
  return v15 & 0xffffffff;
}

// Function: sub_14350 @ 0x14350
void sub_14350(unsigned long a0,unsigned long a1)
{
  sub_10bf0(sub_10690(),a0,a1); // tail-call
}

// Function: sub_14370 @ 0x14370
unsigned long sub_14370(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x28
  
  if (!sub_107d0(a1,a2,&v2))
    return 0;
  dat_3e67c = sub_16d30(v2);
  v1 = sub_14350(a1,a2); // tail-call
  return v1;
}

// Function: sub_14400 @ 0x14400
char * sub_14400(unsigned long a0,long a1,int *a2,int *a3)
{
  unsigned long v1;
  unsigned long v2;
  int v3; // eax
  char *v4; // rax
  char *v5; // stack - 0x38
  unsigned int v6;
  
  v4 = (long)(long)*a2;
  v1 = *(unsigned long *)(a1 + -8 + (long)v4 * 8);
  if (!sub_107d0(a1,a2,&v5)) {
    v4 = NULL;
    return v4;
  }
  v4 = (unsigned long)strspn(v5,"0123456789");
  v6 = dat_3e68c;
  if ((1 <= v3) && (v4 = (long)(long)v3, !v5[(long)v4])) {
    v3 = sub_16f40(v5,dat_3e68c);
    *a3 = v3;
    if (0 <= v3) {
      v4 = (unsigned long)sub_14350(a1,a2); // tail-call
      return v4;
    }
    v6 = dat_3e68c;
  }
  v4 = (unsigned long)sub_24e50(0,v6,v5);
  v2 = v4;
  v4 = dcgettext(NULL,"Expected a positive decimal integer argument to %s, but got %s",5);
  error(1,0,v4,v1,v2);
  return v4;
}

// Function: sub_14510 @ 0x14510
void sub_14510(void)
{
  sub_14400(); // tail-call
}

// Function: sub_14520 @ 0x14520
void sub_14520(void)
{
  sub_14400(); // tail-call
}

// Function: sub_14530 @ 0x14530
void sub_14530(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e620 = 0x100;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_14550 @ 0x14550
void sub_14550(unsigned long a0,unsigned long a1,unsigned long a2)
{
  if (sub_106f0())
    error(0,0,dcgettext(NULL,"warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.",5));
  sub_14530(a0,a1,a2); // tail-call
}

// Function: sub_145b0 @ 0x145b0
void sub_145b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62e = 1;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_145d0 @ 0x145d0
void sub_145d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62d = 1;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_145f0 @ 0x145f0
void sub_145f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62c = 1;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_14610 @ 0x14610
void sub_14610(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62e = 0;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_14630 @ 0x14630
void sub_14630(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e630 = 0;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_14650 @ 0x14650
void sub_14650(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e630 = 1;
  sub_14350(a1,a2); // tail-call
}

// Function: sub_14670 @ 0x14670
void sub_14670(long *a0)
{
  void *v1;
  unsigned int v2; // eax
  
  if (a0) {
    v2 = 0;
    do {
      while( true ) {
        v1 = (void *)*a0;
        if (v1 != sub_8d40) break;
        a0 = (long *)a0[0x21];
        v2 |= 1;
        if (!a0) goto label_146ba;
      }
      a0 = (long *)a0[0x21];
      if (v1 == sub_86f0)
        v2 |= 2;
    } while (a0);
label_146ba:
    if ((v2 == 3) && (!dat_3e621)) {
      error(1,0,dcgettext(NULL,"The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.",5));
      return;
    }
  }
}

// Function: sub_146f0 @ 0x146f0
void sub_146f0(int a0) // return-dupe
{
  if (a0 != 1) { // branch-flip
    if (a0 != 2) { // branch-flip
      if (!a0) {
        dat_3e670 = sub_db20;
        dat_3e680 = sub_10940;
      }
    }
    else {
      dat_3e62c = 1;
      dat_3e670 = sub_da00;
      dat_3e680 = sub_109e0;
    }
  }
  else {
    dat_3e62c = 1;
    dat_3e670 = sub_daa0;
    dat_3e680 = sub_10a40;
  }
  dat_3e668 = a0;
  if (!(dat_3e660 & 2))
    return;
  dat_3e670 = sub_db70;
}

// Function: sub_147a0 @ 0x147a0
void sub_147a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_146f0(1,a2,a2,a3,a1);
  sub_14350(a1); // tail-call
}

// Function: sub_147d0 @ 0x147d0
void sub_147d0(void)
{
  dat_3ee70 = 0;
}

// Function: sub_147e0 @ 0x147e0
void sub_147e0(void)
{
  return;
}

// Function: sub_147f0 @ 0x147f0
unsigned long sub_147f0(char *a0)
{
  char v1;
  long *v2; // rbx
  int v3; // ebp
  long v4;
  char *v5;
  
  if (!strncmp("-newer",a0,6)) {
    v4 = 0x3d800;
    if (strlen(a0) == 8) {
      return sub_108b0(a0,v4); // return-dupe, tail-call
    }
  }
  v1 = *a0;
  v2 = (long *)0x3cd48;
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
  v4 = (long)v3 * 0x20 + 0x3cd20;
  return sub_108b0(a0,v4);
}

// Function: sub_148a0 @ 0x148a0
unsigned long sub_148a0(void)
{
  long v1; // rax
  
  v1 = sub_d4d0(sub_147f0("false"));
  *(unsigned int *)(v1 + 0x18) = 0;
  *(unsigned int *)(v1 + 0x24) = 0;
  return 1;
}

// Function: sub_148d0 @ 0x148d0
void sub_148d0(void)
{
  sub_148a0(); // tail-call
}

// Function: sub_148e0 @ 0x148e0
unsigned long sub_148e0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x28
  
  v1 = sub_107d0(a1,a2,&v4);
  if ((char)v1) {
    if ((2 <= dat_3e688) && (!sub_10510(v4))) {
      if (dat_3e660 & 8)
        __fprintf_chk(stderr,1,"-fstype %s can never succeed, substituting -false\n",v4);
      v2 = sub_148a0(); // tail-call
      return v2;
    }
    v3 = sub_d490(a0,v4);
    *(unsigned long *)(v3 + 0x38) = v4;
    *(unsigned int *)(v3 + 0x24) = sub_13a70(v4);
  }
  return (unsigned long)v1;
}

// Function: sub_149d0 @ 0x149d0
unsigned long sub_149d0(void)
{
  return 0;
}

// Function: sub_149e0 @ 0x149e0
unsigned long sub_149e0(char *a0,unsigned long a1) // early-return
{
  char *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x28
  char v4 [8];
  
  v1 = getenv(a0);
  if (!v1)
    return 0;
  if (!sub_26ec0(v1,v4,10,&v3,0))
    return (unsigned long)(v3 < a1);
  v1 = dcgettext(NULL,"Environment variable %s is not set to a valid decimal number",5);
  error(1,*__errno_location(),v1,a0);
  return v2;
}

// Function: sub_14a90 @ 0x14a90
bool sub_14a90(int a0,long a1,long a2) // early-return
{
  if (((!a0) && (*(long *)(a1 + 0x50))) && (*(long *)(a1 + 0x50) == a2 - *(long *)(a1 + 0x38)))
    return 1;
  return *(long *)(a1 + 0x20) == a2;
}

// Function: sub_14ac0 @ 0x14ac0
void sub_14ac0(unsigned long a0,unsigned long *a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  
  if (a0) { // branch-flip
    v3 = a1[1];
    v2 = 0;
    do {
      v2 += 1;
      *(unsigned long *)(a2 + -8 + v2 * 8) = *(unsigned long *)((v3 - 8) + v2 * 8);
    } while (v2 != a0);
    a4 += a0;
    v3 = a0;
  }
  else {
    v3 = 0;
  }
  v2 = *a1;
  if ((v3 < a3) && (a4 < v2)) {
    v1 = a1[1];
    do {
      a4 += 1;
      v3 += 1;
      *(unsigned long *)(a2 + -8 + v3 * 8) = *(unsigned long *)((v1 - 8) + a4 * 8);
      if (a3 <= v3) break;
    } while (a4 < v2);
    if (v3 < a0)
      __assert_fail("dst_pos >= ctl->initial_argc","buildcmd.c",0xf2,"copy_args"); // no-return
  }
  *(unsigned long *)(a2 + v3 * 8) = 0;
}

// Function: sub_14b60 @ 0x14b60
unsigned long sub_14b60(long a0,long a1,char a2,unsigned long a3) // early-return
{
  unsigned long v1;
  unsigned long v2;
  
  v2 = *(unsigned long *)(a1 + 0x40);
  if (a2) { // branch-flip
    if (v2 < a3) {
      *(unsigned long *)(a1 + 0x40) = a3;
      v2 = a3;
    }
    if ((v2) && (v2 < *(unsigned long *)(a1 + 0x48))) {
      v2 = *(unsigned long *)(a1 + 0x48) - v2;
      if (v2 != 1) {
        v1 = a3 + (v2 >> 1);
        goto label_14b8a;
      }
    }
    else {
      v1 = a3;
      if (a3 == 0xffffffffffffffff) goto label_14b8a;
    }
    v1 = a3 + 1;
  }
  else {
    if (a3 <= (unsigned long)(*(long *)(a1 + 0x48) - 1U))
      *(unsigned long *)(a1 + 0x48) = a3;
    if ((v2) && (v2 < *(unsigned long *)(a1 + 0x48))) {
      v2 = *(unsigned long *)(a1 + 0x48) - v2;
      v1 = a3 - (v2 >> 1);
      if (v2 == 1)
        v1 = a3 - 1;
    }
    else {
      v1 = a3 >> 1;
    }
  }
label_14b8a:
  v2 = a0 + 1U;
  if (a0 + 1U <= v1)
    v2 = v1;
  if (a0)
    v1 = v2;
  if (v1)
    return v1;
  return 1;
}

// Function: sub_14c20 @ 0x14c20
long sub_14c20(void) // early-return
{
  long v1; // rax
  
  v1 = sysconf(0);
  if (1 <= v1)
    return v1;
  return 0x7fffffffffffffff;
}

// Function: sub_14c50 @ 0x14c50
long sub_14c50(void)
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

// Function: sub_14ca0 @ 0x14ca0
unsigned long sub_14ca0(unsigned int *a0,long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  long v3; // rdx
  
  v1 = sub_14c50();
  *(unsigned long *)&a0[4] = 0x1000;
  v2 = sub_14c20();
  *a0 = 0;
  *(unsigned long *)&a0[2] = v2;
  if (v2 < v1)
    return 1;
  if (v1 + a1 < v2) {
    v1 = (v2 - a1) - v1;
    *(unsigned long *)&a0[2] = v1;
    v3 = (v1 >> 3) - 2;
    *(long *)&a0[8] = v3;
    if (v3) {
      *(unsigned long *)&a0[6] = v1;
      *(unsigned long *)&a0[10] = 0;
      *(unsigned long *)&a0[0xc] = 0;
      *(unsigned long *)&a0[0xe] = 0;
      *(void **)&a0[0x10] = sub_149d0;
      *(unsigned long *)&a0[0x12] = 0;
      *(unsigned long *)&a0[0x14] = 0;
      return 0;
    }
    __assert_fail("ctl->max_arg_count > 0","buildcmd.c",0x206,"bc_init_controlinfo"); // no-return
  }
  return 2;
}

// Function: sub_14d80 @ 0x14d80
void sub_14d80(long a0)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 8);
  if ((0x20000 <= v1) && (v1 = *(unsigned long *)(a0 + 0x10), v1 <= 0x1ffff))
    v1 = 0x20000;
  *(unsigned long *)(a0 + 0x18) = v1;
}

// Function: sub_14db0 @ 0x14db0
void sub_14db0(long a0,unsigned long *a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v1 = *(unsigned long *)(a0 + 0x18);
  *a1 = 0;
  a1[4] = 0;
  a1[1] = 0;
  a1[2] = 0;
  a1[8] = 0;
  a1[9] = 0;
  if (v1 <= 0x7ffffffffffff7ff) {
    v2 = sub_26990(v1 + 1);
    a1[5] = 0;
    a1[3] = v2;
    a1[4] = 0;
    a1[7] = 0xffffffff00000000;
    a1[6] = a2;
    return;
  }
  __assert_fail("ctl->arg_max <= (LONG_MAX - 2048L)","buildcmd.c",0x23c,"bc_init_state"); // no-return
}

// Function: sub_14e60 @ 0x14e60
void sub_14e60(long a0,unsigned long *a1)
{
  *a1 = *(unsigned long *)(a0 + 0x38);
  a1[4] = a1[5];
  a1[7] = 0xffffffff00000000;
}

// Function: sub_14e90 @ 0x14e90
void sub_14e90(long a0,unsigned long *a1)
{
  int v1; // eax
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  long v5; // rbx
  long v6;
  
  sub_15450();
  v4 = *a1;
  if (!v4)
    __assert_fail("state->cmd_argc > 0","buildcmd.c",0x105,"bc_do_exec"); // no-return
  if (*(long *)((a1[1] - 8) + v4 * 8))
    __assert_fail("state->cmd_argv[state->cmd_argc-1] == NULL","buildcmd.c",0x106,"bc_do_exec"); // no-return
  v5 = 0;
  v2 = (void *)sub_26990(v4 * 8 + 8);
  v4 = *a1;
  do {
    while( true ) {
      v3 = sub_14ac0(*(unsigned long *)(a0 + 0x38),a1,v2,v4,v5);
      v1 = (**(void **)(a0 + 0x40))(a0,a1[6],v3 & 0xffffffff,v2);
      v6 = *(long *)(a0 + 0x38);
      if (!v1) break;
      v4 = sub_14b60(v6,a1,1,v4);
      v5 = (v3 + v5) - v6;
      if (*a1 - v6 <= v5 + 1U) {
        free(v2); // return-dupe
        sub_14e60(a0,a1); // tail-call
        return;
      }
    }
    if (v4 <= v6 + 1U) {
      error(1,0,dcgettext(NULL,"can\'t call exec() due to argument size restrictions",5));
      return;
    }
    v4 = sub_14b60(v6,a1,0,v4);
  } while (v5 + 1U < *a1 - v6);
  free(v2);
  sub_14e60(a0,a1);
  return;
}

// Function: sub_15000 @ 0x15000
void sub_15000(int *a0,unsigned long *a1,char *a2,long a3,char *a4,long a5,int a6) // return-dupe x3
{
  long *v1;
  int v2;
  int v3; // eax
  unsigned long v4;
  unsigned long v5; // rdx
  unsigned long v6;
  unsigned long v7; // rdx
  unsigned long v8;
  
  v2 = a6;
  if (!a2)
    __assert_fail("arg != NULL","buildcmd.c",0x155,"bc_push_arg"); // no-return
  if (a2 != "do_not_care") { // branch-flip
    if (a5 + a3 + a1[4] <= *(unsigned long *)&a0[6]) {
      v3 = sub_14a90(a6,a0,*a1);
      v6 = v7;
      if (v3) goto label_1518a;
      goto label_15199;
    }
    if ((a6) || (*a1 == *(unsigned long *)&a0[0xe])) {
      error(1,0,dcgettext(NULL,"cannot fit single argument within argument list size limit",5));
      return;
    }
    if ((*(long *)&a0[0xc]) || ((*a0 && ((*(long *)&a0[0x12] || (*(long *)&a0[0x14])))))) {
      error(1,0,dcgettext(NULL,"argument list too long",5));
      return;
    }
    sub_14e90(a0,a1);
    v8 = *a1;
    v6 = v5;
    if (sub_14a90(0,a0,v8)) {
label_1518a:
      sub_14e90(a0,a1);
      goto label_15195;
    }
label_150b8:
    v8 = a1[2];
    v4 = a1[1];
    *(unsigned int *)&a1[7] = 1;
    if (v6 < v8) goto label_150f2;
  }
  else {
label_15195:
    v6 = *a1;
label_15199:
    if (!v2) goto label_150b8;
    v8 = a1[2];
    v4 = a1[1];
    if (v6 < v8) goto label_150f2;
  }
  if (v4) { // branch-flip
    a1[2] = v8 * 2;
    v4 = sub_269e0(v4,v8 << 4);
    v6 = *a1;
    a1[1] = v4;
  }
  else {
    a1[2] = 0x40;
    v4 = sub_26990(0x200);
    v6 = *a1;
    a1[1] = v4;
  }
label_150f2:
  v1 = (long *)(v4 + v6 * 8);
  if (a2 != "do_not_care") { // branch-flip
    v8 = a1[4];
    *a1 = v6 + 1;
    *v1 = a1[3] + v8;
    if (a4) {
      strcpy((char *)(v8 + a1[3]),a4);
      v8 = a5 + a1[4];
      a1[4] = v8;
    }
    strcpy((char *)(v8 + a1[3]),a2);
    a1[4] = a1[4] + a3;
    v6 = *a1;
    if (sub_14a90(v2,a0,v6)) { // branch-flip
      sub_14e90(a0,a1);
      if (!v2)
        return;
    }
    else if (!v2)
      return;
  }
  else {
    *a1 = v6 + 1;
    *v1 = 0;
    if (!v2)
      return;
  }
  a1[5] = a1[4];
}

// Function: sub_15290 @ 0x15290
void sub_15290(long a0,unsigned long a1,char *a2,unsigned long a3,char *a4,long a5,char *a6,long a7,unsigned int a8)
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
  v7 = *(long *)(a0 + 0x18) - 1;
  if (!dat_3ee78)
    dat_3ee78 = (char *)sub_26990(*(long *)(a0 + 0x18) + 1);
  v1 = dat_3ee78;
  v8 = dat_3ee78;
  do {
    while( true ) {
      v5 = (char *)sub_1caf0(a2,*(unsigned long *)(a0 + 0x30));
      if (v5) break;
      if (v7 <= a3) {
label_15404:
        if (*a2) {
          error(1,0,dcgettext(NULL,"command too long",5));
          return;
        }
        goto label_153c0;
      }
      v5 = &a2[a3];
      strncpy(v8,a2,a3);
      v7 -= a3;
      v8 = &v8[a3];
      a3 = 0;
      a2 = v5;
      if (!*v5) goto label_153c0;
    }
    v6 = (long)v5 - (long)a2;
    if (v7 <= v6) goto label_15404;
    strncpy(v8,a2,v6);
    v8 = &v8[v6];
    if (v7 - v6 <= (unsigned long)(v3 + a5)) {
      a2 = v5;
      goto label_15404;
    }
    v7 = (v7 - v6) - (v3 + a5);
    if (a4) {
      strcpy(v8,a4);
      v8 = &v8[a5];
    }
    strcpy(v8,v2);
    a2 = &v5[*(long *)(a0 + 0x28)];
    v8 = &v8[v3];
    a3 = (a3 - *(long *)(a0 + 0x28)) - v6;
  } while (*a2);
label_153c0:
  *v8 = '\0';
  a6._0_4_ = v4;
  sub_15000(a0,a1,v1,&v8[1 - (long)v1],0,0); // tail-call
  return;
}

// Function: sub_15450 @ 0x15450
void sub_15450(unsigned long a0,unsigned long a1)
{
  sub_15000(a0,a1,"do_not_care",0,0,0,0);
}

// Function: sub_15470 @ 0x15470
unsigned long sub_15470(unsigned long *a0)
{
  char *v1;
  unsigned long v2; // rax
  long v3;
  long v4;
  
  v4 = 0;
  v3 = 0;
  v1 = (char *)*a0;
  while (v1) {
    v3 += 1;
    v4 += strlen(v1);
    v1 = (char *)a0[v3];
  }
  if (sub_149e0("__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT",v3))
    return 1;
  v2 = sub_149e0("__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT",v4);
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),(int)v2 != 0);
}

// Function: sub_154f0 @ 0x154f0
int sub_154f0(unsigned long a0,void *a1,unsigned long a2)
{
  int v1;
  int v2; // eax
  int *v3; // rax
  char v4 [24];
  
  if (sub_251f0(v4))
    sub_1e610(*__errno_location()); // no-return
  if (!sub_25250(a0)) {
    v2 = (*a1)(a2);
    if (0 <= v2) { // branch-flip
      if (!sub_25250(v4)) {
        sub_25280(v4);
        return v2;
      }
    }
    else {
      v3 = __errno_location();
      v1 = *v3;
      if (!sub_25250(v4)) {
        sub_25280(v4);
        if (!v1)
          return v2;
        *v3 = v1;
        return v2;
      }
    }
  }
  sub_1e650(*__errno_location()); // no-return
}

// Function: sub_155c0 @ 0x155c0
unsigned long sub_155c0(int a0,void *a1,unsigned long a2)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  char v4 [24];
  unsigned long v5; // r12
  
  if (a0 == -100)
    return (*a1)(a2); // jump-as-call
  if (sub_251f0(v4))
    sub_1e610(*__errno_location()); // no-return
  if (fchdir(a0)) { // branch-flip
    v3 = __errno_location();
    v5 = 0xffffffff;
    v1 = *v3;
    sub_25280(v4);
    *v3 = v1;
  }
  else {
    v2 = (*a1)(a2);
    v5 = (unsigned long)v2;
    if (0 <= (int)v2) { // branch-flip
      if (sub_25250(v4))
        sub_1e650(*__errno_location()); // return-dupe, no-return
      sub_25280(v4);
    }
    else {
      v3 = __errno_location();
      v1 = *v3;
      if (sub_25250(v4)) {
        sub_1e650(*__errno_location());
      }
      sub_25280(v4);
      if (v1)
        *v3 = v1;
    }
  }
  return v5;
}

// Function: sub_156e0 @ 0x156e0
unsigned long sub_156e0(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  
  v1 = 0x10;
  if (a0)
    v1 = a0;
  while( true ) {
    if (a1 <= v1)
      return v1;
    if (v1 * 2 < v1) break;
    v1 *= 2;
  }
  return a1;
}

// Function: sub_15710 @ 0x15710
void * sub_15710(void *a0,long a1,unsigned long *a2)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a1)
    __assert_fail("wanted > 0u","extendbuf.c",0x47,"extendbuf"); // no-return
  v4 = *a2;
  v3 = sub_156e0(v4,a1);
  if (v4) { // branch-flip
    if (v4 != v3) {
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
  *v2 = v1;
  return a0;
}

// Function: sub_157e0 @ 0x157e0
void sub_157e0(void *a0)
{
  if (sub_15710())
    return;
  free(a0);
  sub_26d50(); // no-return
}

// Function: sub_15810 @ 0x15810
unsigned int sub_15810(unsigned long a0)
{
  return sub_179d0(a0,1) & 1;
}

// Function: sub_15830 @ 0x15830
unsigned long sub_15830(unsigned int a0,long *a1) // return-dupe
{
  long v1;
  long v2; // rax
  
  if (sub_15810())
    return 0;
  v2 = sub_15710(*a1,a1[1] * 4 + 4,&a1[2]);
  if (!v2)
    return 0xffffffff;
  v1 = a1[1];
  *a1 = v2;
  *(unsigned int *)(v2 + v1 * 4) = a0;
  a1[1] = v1 + 1;
  return 0;
}

// Function: sub_158a0 @ 0x158a0
unsigned long sub_158a0(int a0,long *a1)
{
  int v1;
  unsigned long v2;
  
  if (sub_15810())
    return 0;
  v2 = a1[2];
  if (v2 < (unsigned long)a1[1]) {
    do {
      v1 = *(int *)(*a1 + v2 * 4);
      if (a0 <= v1) {
        if (v1 == a0)
          return 0;
        break;
      }
      v2 += 1;
      a1[2] = v2;
    } while (v2 != a1[1]);
  }
  *(int *)&a1[3] = a0;
  return 0xffffffff;
}

// Function: sub_15900 @ 0x15900
int sub_15900(void) // return-dupe x2
{
  bool v1;
  int v2; // eax
  void *v3; // rax
  int *v4; // rax
  void *v5; // rax
  unsigned long v6; // rax
  int v7;
  
  v3 = (void *)sub_17650("/proc/self/fd");
  if (!v3)
    return -1;
  v1 = 0;
  v4 = __errno_location();
  v7 = -1;
  while( true ) {
    *v4 = 0;
    v5 = readdir(v3);
    if (!v5) break;
    if ((*(char *)((long)v5 + 0x13) != '.') || ((*(char *)((long)v5 + 0x14) && (*(short *)((long)v5 + 0x14) != 0x2e)))) {
      v1 = 1;
      v2 = sub_16f40((long)v5 + 0x13,0);
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
  v6 = sub_24e50(0,8,"/proc/self/fd");
  error(0,*v4,"%s",v6);
  closedir(v3);
  return -1;
}

// Function: sub_159e0 @ 0x159e0
unsigned long sub_159e0(void)
{
  int v1;
  int v2; // eax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long v5 [3]; // stack - 0x28
  
  v3 = sub_15900();
  if (v2 < 0) {
    v3 = sysconf(4);
    v4 = v3;
    if (v3 == 0xffffffffffffffff)
      v4 = 0x14;
    v2 = getrlimit(7,v5);
    v1 = v2;
    v3 = v4 & 0xffffffff;
    if (!v1) {
      if (v5[0] == 0xffffffffffffffff)
        v5[0] = v4;
      v3 = v5[0] & 0xffffffff;
    }
  }
  return v3;
}

// Function: sub_15a60 @ 0x15a60
bool sub_15a60(void)
{
  int v1; // eax
  bool v2; // r12b
  
  v2 = 0;
  v1 = sub_17b20("/",0x80000);
  if (0 <= v1) {
    v2 = sub_15810(v1) != 0;
    close(v1);
  }
  return v2;
}

// Function: sub_15ab0 @ 0x15ab0
void sub_15ab0(int a0,void *a1,unsigned long a2) // return-dupe
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

// Function: sub_15bb0 @ 0x15bb0
unsigned int sub_15bb0(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  unsigned long v2; // stack - 0x48
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned int v5; // stack - 0x30
  
  v1 = sub_159e0();
  v5 = 0xffffffff;
  v4 = 0;
  v2 = a0;
  v3 = a1;
  sub_15ab0(v1 + (unsigned int)(v1 != 0x7fffffff),sub_158a0,&v2);
  return v5;
}

// Function: sub_15c30 @ 0x15c30
void sub_15c30(void)
{
  int v1; // eax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v1 = sub_159e0();
  v2 = 0;
  v4 = 0;
  v3 = 0;
  sub_15ab0(v1 + (unsigned int)(v1 != 0x7fffffff),sub_15830,&v2);
  dat_3ee90 = v2;
  dat_3ee88 = v3;
}

// Function: sub_15cc0 @ 0x15cc0
int sub_15cc0(unsigned long a0,unsigned int a1,unsigned int a2)
{
  int v1; // eax
  unsigned int v2;
  
  v2 = 0;
  if (a1 & 0x40)
    v2 = a2;
  if (!dat_3ee81) {
    dat_3ee80 = sub_15a60();
    dat_3ee81 = '\x01';
  }
  v1 = sub_17b20(a0,a1 | 0x80000,v2);
  if ((0 <= v1) && (!dat_3ee80))
    sub_174f0(v1,1);
  return v1;
}

// Function: sub_15da0 @ 0x15da0
void sub_15da0(void)
{
  free(dat_3ee90);
  dat_3ee90 = NULL;
  dat_3ee88 = 0;
}

// Function: sub_15dd0 @ 0x15dd0
unsigned long sub_15dd0(void)
{
  char *v1; // rax
  
  v1 = getenv("GNU_FINDUTILS_FD_LEAK_CHECK");
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != NULL);
}

// Function: sub_15df0 @ 0x15df0
void sub_15df0(void)
{
  unsigned int v1; // eax
  
  v1 = sub_15bb0(dat_3ee90,dat_3ee88);
  if ((int)v1 <= -1)
    return;
  error(0,0,dcgettext(NULL,"File descriptor %d will leak; please report this as a bug, remembering to include a detailed description of the simplest way to reproduce this problem.",5),(unsigned long)v1);
  __assert_fail("no_leaks","fdleak.c",0x18c,"complain_about_leaky_fds"); // no-return
}

// Function: sub_15e60 @ 0x15e60
undefined16 sub_15e60(unsigned long a0)
{
  char v1 [16];
  char *v2; // rax
  char *v3; // rax
  
  sub_17bc0(stderr);
  v2 = dcgettext(NULL,"Kevin Dalley",5);
  v3 = dcgettext(NULL,"James Youngman",5);
  sub_26820(stdout,a0,"GNU findutils",dat_3e490,dcgettext(NULL,"Eric B. Decker",5),v3);
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_;
}

// Function: sub_15ef0 @ 0x15ef0
unsigned long sub_15ef0(void *a0,unsigned long a1,unsigned int *a2)
{
  unsigned int v1; // eax
  unsigned long v2; // r8
  
  v1 = __fprintf_chk(a0,1,"%*lu",(unsigned long)*a2,a1);
  v2 = 0;
  if ((0 <= (int)v1) && (v2 = 1, (int)*a2 < (int)v1)) {
    *a2 = v1;
    return 1;
  }
  return v2;
}

// Function: sub_15f40 @ 0x15f40
unsigned int sub_15f40(unsigned long a0,void *a1)
{
  return (unsigned int)~__fprintf_chk(a1,1,"%s",a0) >> 0x1f;
}

// Function: sub_15f70 @ 0x15f70
unsigned long sub_15f70(unsigned char *a0,void *a1) // return-dupe
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
        goto label_16038;
      }
label_15fd8:
      if (0x5e <= (unsigned char)(v1 - 0x21)) {
        v2 = __fprintf_chk(a1,1,"\\%03o");
        goto label_16038;
      }
      if (putc((unsigned int)v1,a1) == -1)
        return 0;
    }
    else {
      if ((char)v1 <= '\a') goto label_15fd8;
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
          goto label_15fd8;
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
label_16038:
      if (v2 <= -1)
        return 0;
    }
    v1 = *a0;
    if (!v1)
      return 1;
  } while( true );
}

// Function: sub_16150 @ 0x16150
void sub_16150(unsigned long a0,unsigned long a1,int a2)
{
  if (a2) {
    sub_15f40(); // tail-call
    return;
  }
  sub_15f70(); // tail-call
}

// Function: sub_16170 @ 0x16170
void sub_16170(unsigned long a0,unsigned int a1,unsigned long a2,unsigned long a3,long a4,unsigned int a5,unsigned int a6,void *a7)
{
  char v1;
  long v10;
  void *v11; // rax
  unsigned long v12; // rax
  int *v13; // rax
  char *v14; // rax
  char *v15; // rcx
  char *v16;
  char v17 [12];
  char v18 [664];
  char v19 [8];
  void *v2;
  char v20 [256];
  char *v21; // rsp
  char *v22; // rsp
  char *v23;
  char *v24;
  unsigned long v25;
  long v26; // stack - 0x420
  unsigned int v27; // stack - 0x414
  unsigned long v28; // stack - 0x410
  char v29; // stack - 0x405
  char v3; // al
  unsigned int v30; // stack - 0x404
  long v31; // stack - 0x400
  unsigned long v32; // stack - 0x3f8
  unsigned long v33; // stack - 0x3f0
  unsigned int v4;
  int v5;
  unsigned int v6; // eax
  unsigned int v7; // eax
  unsigned int v8;
  unsigned long v9; // rax
  
  v2 = a7;
  v21 = v19;
  v24 = v19;
  v23 = v19;
  v33 = CONCAT44(v33._4_4_,a5);
  v28 = a2;
  v30 = a1;
  v31 = a4;
  v32 = a0;
  sub_17c70(*(unsigned int *)(a3 + 0x18),v17);
  v9 = sub_1b800(*(unsigned long *)(a3 + 8),v18,0,1,1);
  v4 = __fprintf_chk(v2,1,"%*s",(unsigned long)dat_3e4b4,v9);
  if (0 <= (int)v4) { // branch-flip
    if ((int)dat_3e4b4 < (int)v4)
      dat_3e4b4 = v4;
    v5 = putc(0x20,v2);
    v27 = 0;
    v29 = v5 != -1;
    if (!(bool)v29)
      v27 = 0x96;
    v26 = (long)(int)v33;
    v9 = sub_1b800(*(unsigned long *)(a3 + 0x40),v18,0,0x200,v26);
    v4 = __fprintf_chk(v2,1,"%*s",(unsigned long)dat_3e4b0,v9);
    if (0 <= (int)v4) { // branch-flip
      if ((int)dat_3e4b0 < (int)v4)
        dat_3e4b0 = v4;
      v23 = v19;
      v8 = v27;
      if (v29) {
        if (putc(0x20,v2) != -1) { // branch-flip
          if (0 <= fputs(v17,v2)) { // branch-flip
            v4 = __fprintf_chk(v2,1,"%*lu",(unsigned long)dat_3e4ac,*(unsigned long *)(a3 + 0x10));
            if (0 <= (int)v4) { // branch-flip
              if ((int)dat_3e4ac < (int)v4)
                dat_3e4ac = v4;
              putc(0x20,v2);
              v10 = sub_1c170(*(unsigned int *)(a3 + 0x1c));
              if (v10) { // branch-flip
                v6 = sub_1d3b0(v10,0);
                v4 = dat_3e4a8;
                if ((int)dat_3e4a8 < (int)v6) {
                  v4 = v6;
                  dat_3e4a8 = v6;
                }
                if (__fprintf_chk(v2,1,"%-*s ",(unsigned long)v4,v10) < 0) {
                  v8 = 400;
                  v23 = v19;
                  goto label_16536;
                }
              }
              else {
                v4 = __fprintf_chk(v2,1,"%-8lu ",(unsigned long)*(unsigned int *)(a3 + 0x1c));
                if ((int)dat_3e4a8 < (int)v4)
                  dat_3e4a8 = v4;
                if ((int)v4 <= 0) {
                  v8 = 0x1c2;
                  v23 = v19;
                  goto label_16536;
                }
              }
              v10 = sub_1c310(*(unsigned int *)(a3 + 0x20));
              if (v10) { // branch-flip
                v6 = sub_1d3b0(v10,0);
                v4 = dat_3e4a4;
                if ((int)dat_3e4a4 < (int)v6) {
                  v4 = v6;
                  dat_3e4a4 = v6;
                }
                if (__fprintf_chk(v2,1,"%-*s ",(unsigned long)v4,v10) < 0) {
                  v8 = 500;
                  v23 = v19;
                  goto label_16536;
                }
              }
              else {
                v4 = __fprintf_chk(v2,1,"%-*lu",(unsigned long)dat_3e4a4,(unsigned long)*(unsigned int *)(a3 + 0x20));
                if ((int)dat_3e4a4 < (int)v4)
                  dat_3e4a4 = v4;
                v8 = 0x226;
                v23 = v19;
                if ((int)v4 < 0) goto label_16536;
                if (putc(0x20,v2) == -1) {
                  v8 = 0x20d;
                  v23 = v19;
                  goto label_16536;
                }
              }
              if ((*(unsigned int *)(a3 + 0x18) & 0xb000) != 0x2000) { // branch-flip
                v10 = 1;
                if ((int)v33 <= -1)
                  v10 = v26;
                v9 = sub_1b800(*(unsigned long *)(a3 + 0x30),v18,0,1,v10);
                v4 = __fprintf_chk(v2,1,"%*s",(unsigned long)dat_3e498,v9);
                if ((int)v4 < 0) {
                  v8 = 800;
                  v23 = v19;
                  goto label_16536;
                }
                if ((int)dat_3e498 < (int)v4)
                  dat_3e498 = v4;
              }
              else {
                v7 = gnu_dev_major(*(unsigned long *)(a3 + 0x28));
                v8 = 600;
                v23 = v19;
                if (!sub_15ef0(v2,v7,0x3e4a0)) goto label_16536;
                if (__fprintf_chk(v2,1,", ") < 0) {
                  v8 = 0x271;
                  v23 = v19;
                  goto label_16536;
                }
                v8 = gnu_dev_minor(*(unsigned long *)(a3 + 0x28));
                if (!sub_15ef0(v2,v8,0x3e49c)) {
                  v8 = 0x28a;
                  v23 = v19;
                  goto label_16536;
                }
              }
              if (putc(0x20,v2) != -1) { // branch-flip
                v11 = localtime((void *)(a3 + 0x58));
                if (v11) { // branch-flip
                  v14 = "%b %e  %Y";
                  if ((v31 + -0xed4e00 <= *(long *)(a3 + 0x58)) && (v14 = "%b %e %H:%M", v31 + 0xe10 < *(long *)(a3 + 0x58)))
                    v14 = "%b %e  %Y";
                  v16 = v20;
                  v33 = a3;
                  v25 = 0x100;
                  while( true ) {
                    *(unsigned long *)&v21[-8] = 0x164c8;
                    v12 = strftime(v16,v25,v14,v11,v21[-8]);
                    a3 = v33;
                    if (v12) break;
                    v25 *= 2;
                    v15 = &v21[-(v25 + 0x17 & 0xfffffffffffff000)];
                    v23 = v21;
                    v24 = v21;
                    while (v21 != v15) {
                      v23 = &v24[-0x1000];
                      v22 = &v24[-0x1000];
                      *(unsigned long *)&v24[-8] = *(unsigned long *)&v24[-8];
                      v21 = &v24[-0x1000];
                      v24 = v22;
                    }
                    v12 = (unsigned long)((unsigned int)(v25 + 0x17) & 0xff0);
                    v10 = -v12;
                    v21 = &v23[v10];
                    if (v12)
                      *(unsigned long *)&v23[-8] = *(unsigned long *)&v23[-8];
                    v16 = (char *)((unsigned long)&v23[v10 + 0xf] & 0xfffffffffffffff0);
                  }
                  *(unsigned long *)&v21[-8] = 0x165e3;
                  v8 = 900;
                  v23 = v21;
                  v24 = v21;
                  if (__fprintf_chk(v2,1,"%s ",v16) < 0) goto label_16536;
                }
                else {
                  v10 = *(long *)(a3 + 0x58);
                  if (0 <= v10) { // branch-flip
                    v9 = sub_1b800(v10,v18,0,1,1);
                    v24 = v19;
                    if (__fprintf_chk(v2,1,"%*s ",0xc,v9) <= -1) {
                      v8 = 0x44c;
                      v23 = v19;
                      goto label_16536;
                    }
                  }
                  else {
                    v14 = (char *)sub_1b800(-v10,v18,0,1,1);
                    v4 = 0xc - (int)strlen(v14);
                    if ((int)v4 <= -1)
                      v4 = 0;
                    if (__fprintf_chk(v2,1,"%*s%s ",(unsigned long)v4,"-",v14) <= -1) {
                      v8 = 1000;
                      v23 = v19;
                      goto label_16536;
                    }
                  }
                }
                *(unsigned long *)&v24[-8] = 0x16603;
                v3 = sub_16150(v32,v2,a6);
                v23 = v24;
                if (v3) { // branch-flip
                  if ((*(unsigned int *)(a3 + 0x18) & 0xf000) == 0xa000) {
                    *(unsigned long *)&v24[-8] = 0x16702;
                    v11 = (void *)sub_17190(v30,v28);
                    if (v11) { // branch-flip
                      *(unsigned long *)&v24[-8] = 0x1671d;
                      v1 = v24[-8];
                      if (fputs(" -> ",v2,v1) <= -1) {
                        *(unsigned long *)&v24[-8] = 0x1672d;
                        free(v11,v24[-8]);
                        v8 = 0x514;
                        goto label_16536;
                      }
                      *(unsigned long *)&v24[-8] = 0x16886;
                      v3 = sub_16150(v11,v2,a6);
                      if (!v3) {
                        *(unsigned long *)&v24[-8] = 0x16896;
                        free(v11,v24[-8]);
                        v8 = 0x546;
                        goto label_16536;
                      }
                      *(unsigned long *)&v24[-8] = 0x1694f;
                      free(v11,v24[-8]);
                    }
                    else {
                      *(unsigned long *)&v24[-8] = 0x16925;
                      v1 = v24[-8];
                      v5 = *__errno_location(v1);
                      *(unsigned long *)&v24[-8] = 0x1693e;
                      error(0,v5,"%s",v32);
                      *(unsigned long *)&v24[-8] = 0x16945;
                      free(NULL,v24[-8]);
                    }
                  }
                  *(unsigned long *)&v24[-8] = 0x16638;
                  v1 = v24[-8];
                  if (putc(10,v2,v1) != -1)
                    return;
                  v8 = 0x578;
                }
                else {
                  v8 = 0x4b0;
                }
              }
              else {
                v8 = 0x352;
                v23 = v19;
              }
            }
            else {
              v8 = 300;
              v23 = v19;
            }
          }
          else {
            v8 = 0x113;
            v23 = v19;
          }
        }
        else {
          v8 = 0xfa;
          v23 = v19;
        }
      }
    }
    else {
      v8 = 200;
      v23 = v19;
    }
  }
  else {
    v8 = 100;
  }
label_16536:
  v33 = CONCAT44(v33._4_4_,v8);
  *(unsigned long *)&v23[-8] = 0x16550;
  v14 = dcgettext(NULL,"Failed to write output (at stage %d)",5,v23[-8]);
  *(unsigned long *)&v23[-8] = 0x16558;
  v13 = __errno_location(v23[-8]);
  v5 = *v13;
  *(unsigned long *)&v23[-8] = 0x1656f;
  error(1,v5,v14,v33 & 0xffffffff);
  return;
}

// Function: sub_16960 @ 0x16960
unsigned long sub_16960(void *a0,unsigned long a1,char a2,char *a3,unsigned long a4)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  void *v3; // rax
  char v4 [8200];
  
  if (a2) {
    v2 = sub_24be0(v4,0x2000,a4,0xffffffffffffffff,a1);
    if (0x2000 <= v2) { // branch-flip
      v3 = (void *)sub_26990(v2 + 1);
      sub_24be0(v3,v2 + 1,a4,0xffffffffffffffff,a1);
      *(char *)((long)v3 + sub_16b50(v3,v2)) = 0;
      v1 = __fprintf_chk(a0,1,a3,v3);
      free(v3);
    }
    else {
      v4[sub_16b50(v4,v2)] = 0;
      v1 = __fprintf_chk(a0,1,a3,v4);
    }
    return (unsigned long)v1;
  }
  return __fprintf_chk(a0,1,a3,a4); // tail-call
}

// Function: sub_16af0 @ 0x16af0
unsigned int sub_16af0(unsigned int a0)
{
  return a0;
}

// Function: sub_16b00 @ 0x16b00
long sub_16b00(char *a0,long a1)
{
  char *v1;
  unsigned char v2; // al
  long *v3; // rax
  
  v1 = &a0[a1];
  if (a0 < v1) {
    v3 = __ctype_b_loc();
    do {
      v2 = sub_16af0((int)*a0);
      if (!(*(unsigned char *)(*v3 + 1 + (unsigned long)v2 * 2) & 0x40))
        *a0 = '?';
      a0 = &a0[1];
    } while (v1 != a0);
  }
  return a1;
}

// Function: sub_16b50 @ 0x16b50
long sub_16b50(char *a0,long a1) // early-return
{
  char v1;
  char *v10;
  int v2; // eax
  long v3; // rax
  char *v4; // rcx
  long v5;
  char *v6;
  unsigned long v7; // stack - 0x48
  int v8; // stack - 0x4c
  char *v9;
  
  if (__ctype_get_mb_cur_max() < 2)
    return sub_16b00(a0,a1); // tail-call
  v4 = &a0[a1];
  if (v4 <= a0)
    return 0;
  v6 = a0;
  v9 = a0;
  do {
    v1 = *v6;
    v10 = &v9[1];
    if ('`' <= v1) { // branch-flip
      if (0x1e <= (unsigned char)(v1 + 0x9fU)) goto label_16bd7;
label_16c8c:
      *v9 = v1;
      v6 = &v6[1];
    }
    else {
      if ('@' < v1) goto label_16c8c;
      if ('$' <= v1) { // branch-flip
        if ((unsigned char)(v1 - 0x25U) <= 0x1a) goto label_16c8c;
      }
      else if ('\x1f' < v1) goto label_16c8c;
label_16bd7:
      v7 = 0;
      while (v3 = sub_1c4b0(&v8,v6,(long)v4 - (long)v6,&v7), v3 != -1) {
        if (v3 == -2) {
          v6 = v4;
          goto label_16ce4;
        }
        v5 = 1;
        if (v3)
          v5 = v3;
        v2 = wcwidth(v8);
        if (0 <= v2) { // branch-flip
          v3 = 0;
          do {
            v9[v3] = v6[v3];
            v3 += 1;
          } while (v5 != v3);
          v10 = &v9[v5];
        }
        else {
          *v9 = '?';
        }
        v6 = &v6[v5];
        if (mbsinit(&v7)) goto label_16c97;
        v9 = v10;
        v10 = &v10[1];
      }
      v6 = &v6[1];
label_16ce4:
      *v9 = '?';
    }
label_16c97:
    v9 = v10;
    if (v4 <= v6)
      return (long)v10 - (long)a0;
  } while( true );
}

// Function: sub_16d30 @ 0x16d30
unsigned long sub_16d30(char *a0)
{
  int v1;
  char *v2; // rax
  unsigned long v3;
  unsigned long v4; // rax
  unsigned int v5; // ebx
  unsigned long v6; // rbx
  unsigned long *v7; // r12
  long v8;
  char *v9;
  
  v9 = "findutils-default";
  v8 = 0;
  v7 = (unsigned long *)0x3c2d0;
  v6 = 0;
  while( true ) {
    if (!strcmp(v9,a0))
      return (unsigned long)*(unsigned int *)(v6 * 0x10 + 0x3c2cc);
    v5 = (int)v6 + 1;
    v6 = (unsigned long)v5;
    v9 = (char *)sub_251e0(v9);
    v8 = v8 + 2 + strlen(v9);
    if (v5 == 0xd) break;
    v9 = (char *)*v7;
    v7 = &v7[2];
  }
  v7 = (unsigned long *)0x3c2c0;
  v2 = (char *)sub_26990(v8 + 1);
  v9 = v2;
  v1 = 0;
  do {
    if (v1) { // branch-flip
      v9[2] = '\0';
      v3 = *v7;
      v9[0] = ',';
      v9[1] = ' ';
      v3 = sub_251e0(v3);
      v9 = &(&v9[2])[sprintf(&v9[2],"%s",v3)];
      if (v1 + 1 == 0xd) {
        v3 = sub_251e0(a0);
        error(1,0,dcgettext(NULL,"Unknown regular expression type %s; valid types are %s.",5),v3,v2);
        return v4;
      }
    }
    else {
      v3 = sub_251e0(*v7);
      v9 = &v9[sprintf(v9,"%s",v3)];
    }
    v7 = &v7[2];
    v1 += 1;
  } while( true );
}

// Function: sub_16e80 @ 0x16e80
unsigned long sub_16e80(unsigned int a0) // early-return
{
  if (0xd <= a0)
    return 0;
  return *(unsigned long *)((unsigned long)a0 * 0x10 + 0x3c2c0);
}

// Function: sub_16ea0 @ 0x16ea0
unsigned int sub_16ea0(unsigned int a0)
{
  if (a0 <= 0xc)
    return *(unsigned int *)((unsigned long)a0 * 0x10 + 0x3c2cc);
  return 0xffffffff;
}

// Function: sub_16ed0 @ 0x16ed0
unsigned int sub_16ed0(unsigned int a0) // early-return
{
  if (0xd <= a0)
    return 0;
  return *(unsigned int *)((unsigned long)a0 * 0x10 + 0x3c2c8);
}

// Function: sub_16ef0 @ 0x16ef0
unsigned int sub_16ef0(unsigned int a0,unsigned int a1)
{
  unsigned int v1; // eax
  unsigned int v2;
  int *v3;
  int *v4; // rdx
  
  if ((a0 <= 0xc) && (a0)) {
    v2 = 1;
    v1 = 0;
    v3 = (int *)0x3c2cc;
    while( true ) {
      if ((v2 & a1) && (*v3 == *(int *)((unsigned long)a0 * 0x10 + 0x3c2cc)))
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

// Function: sub_16f40 @ 0x16f40
void sub_16f40(char *a0,unsigned int a1)
{
  int *v1; // rax
  long v2;
  long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  char *v7; // stack - 0x38
  
  v1 = __errno_location();
  *v1 = 0;
  v2 = strtol(a0,&v7,10);
  if (0xfffffffffffffffe <= v2 + 0x7fffffffffffffffU) {
    if (*v1 == 0x22) {
      error(1,0x22,"%s",a0);
      goto label_16ff5;
    }
    error(1,*v1,"%s",a0);
    v2 = v3;
  }
  if (!(v2 + 0x80000000U >> 0x20)) {
    if (*v7) {
      v4 = sub_24e50(1,a1,a0);
      v6 = sub_24e50(0,a1,v7);
      v5 = dcgettext(NULL,"Unexpected suffix %s on %s",5);
      error(1,*v1,v5,v6,v4);
      return;
    }
    if (v7 == a0) {
      v4 = sub_24e50(0,a1,v7);
      v5 = dcgettext(NULL,"Expected an integer: %s",5);
      error(1,*v1,v5,v4);
      return;
    }
    return;
  }
label_16ff5:
  *v1 = 0x22;
  error(1,0x22,"%s",a0);
  return;
}

// Function: sub_170a0 @ 0x170a0
unsigned long sub_170a0(char *a0,char *a1)
{
  char *v1; // rax
  
  if (*a1) {
    v1 = strpbrk(a0,a1);
    if (v1)
      return (long)v1 - (long)a0;
  }
  return strlen(a0); // tail-call
}

// Function: sub_170d0 @ 0x170d0
unsigned long sub_170d0(long a0,unsigned long a1,char a2,long *a3,long *a4)
{
  long v1; // rax
  
  if (a2) { // branch-flip
    *a3 = 0;
    v1 = 0;
    *a4 = 0;
  }
  else {
    v1 = *a4 + *a3;
    *a3 = v1;
    if (!*(char *)(a0 + v1))
      return 0;
    v1 += 1;
    *a3 = v1;
  }
  *a4 = sub_170a0(a0 + v1);
  return 1;
}

// Function: sub_17130 @ 0x17130
void sub_17130(void *a0,unsigned long a1)
{
  __fprintf_chk(a0,1,dcgettext(NULL,"Please see also the documentation at %s.\nYou can report (and track progress on fixing) bugs in the \"%s\"\nprogram via the %s bug-reporting page at\n%s or, if\nyou have no web access, by sending email to <%s>.\n",5),"http://www.gnu.org/software/findutils/",a1,"GNU findutils","https://savannah.gnu.org/bugs/?group=findutils","bug-findutils@gnu.org");
}

// Function: sub_17190 @ 0x17190
void sub_17190(unsigned long a0,unsigned long a1)
{
  sub_174a0(a0,a1,0,0,0,dat_3df98); // tail-call
}

// Function: sub_171b0 @ 0x171b0
void sub_171b0(unsigned long a0) // return-dupe
{
  unsigned long *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  *v1 = 0;
  v1[4] = a0;
  v1[5] = a0;
}

// Function: sub_171e0 @ 0x171e0
void sub_171e0(unsigned long a0) // return-dupe
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

// Function: sub_17220 @ 0x17220
long sub_17220(long *a0,int *a1) // early-return x2
{
  long *v1;
  long v2;
  void *v3;
  
  if (!*a0) {
    v1 = (long *)a0[5];
    v2 = *v1;
    if (!v2) {
      *a1 = 2;
      return 0;
    }
    *a1 = 1;
    a0[5] = (long)&v1[1];
    return v2;
  }
  if (0 <= (long)getdelim(&a0[2],&a0[3],0)) {
    *a1 = 1;
    a0[1] = a0[1] + 1;
    return a0[2];
  }
  v3 = (void *)*a0;
  *a1 = (-(unsigned int)(feof(v3) == 0) & 2) + 2;
  return 0;
}

// Function: sub_172c0 @ 0x172c0
long sub_172c0(long *a0)
{
  if (*a0)
    return a0[1];
  return a0[5] - a0[4] >> 3;
}

// Function: sub_172e0 @ 0x172e0
void sub_172e0(long *a0)
{
  if (*a0)
    free((void *)a0[2]);
  free(a0); // tail-call
}

// Function: sub_17300 @ 0x17300
void * sub_17300(unsigned int a0,unsigned long a1,void *a2,unsigned long a3,unsigned long *a4,void *a5,void *a6)
{
  int v1;
  long v2; // rax
  void *v3;
  int *v4; // rax
  void *v5;
  unsigned long v6;
  unsigned long v7;
  
  v5 = a6;
  if (!a4)
    a4 = (unsigned long *)0x3d820;
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
    if (v2 < (long)v7) {
      *(char *)((long)v3 + v2) = 0;
      v6 = v2 + 1;
      if (v5 != v3) {
        if ((long)v7 <= (long)v6)
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
    if (0x3ffffffffffffffe < (long)v7) {
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

// Function: sub_174a0 @ 0x174a0
void sub_174a0(void)
{
  sub_17300();
}

// Function: sub_174f0 @ 0x174f0
int sub_174f0(unsigned long a0,char a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  int v3; // r8d
  
  v2 = sub_179d0(a0,1,0);
  if (0 <= (int)v2) { // branch-flip
    v1 = v2 & 0xfffffffe;
    if (a1)
      v1 = v2 | 1;
    v3 = 0;
    if (v1 != v2)
      v3 = -(unsigned int)(sub_179d0(a0 & 0xffffffff,2) == -1);
  }
  else {
    v3 = -1;
  }
  return v3;
}

// Function: sub_17560 @ 0x17560
void sub_17560(unsigned long a0)
{
  sub_179d0(a0,0x406,0); // tail-call
}

// Function: sub_17580 @ 0x17580
void sub_17580(unsigned long a0)
{
  dat_3eea0 = a0;
}

// Function: sub_17590 @ 0x17590
void sub_17590(char a0)
{
  dat_3ee98 = a0;
}

// Function: sub_175a0 @ 0x175a0
void sub_175a0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_283d0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_3ee98) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_3eea0) { // branch-flip
        v4 = sub_25000(dat_3eea0);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_3e4b8); // return-dupe, no-return
    }
  }
  v1 = sub_283d0(stderr);
  if (!v1)
    return;
  _exit(dat_3e4b8);
}

// Function: sub_17650 @ 0x17650
void * sub_17650(char *a0)
{
  unsigned int v1; // eax
  int v2; // eax
  void *v3; // rax
  int *v4; // rax
  void *v5; // rax
  int v6;
  
  v3 = opendir(a0);
  if (v3) {
    v1 = dirfd(v3);
    if (v1 <= 2) {
      v2 = sub_179d0(v1,0x406,3);
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

// Function: sub_176f0 @ 0x176f0
void sub_176f0(char *a0)
{
  char *v1; // rax
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  
  v1 = (char *)sub_27cf0(a0);
  if (*v1) { // branch-flip
    v2 = sub_27d50(v1);
    v3 = v2 + (unsigned long)(v1[v2] == '/');
    a0 = v1;
  }
  else {
    v3 = sub_27d50(a0);
  }
  v4 = (void *)sub_269b0(v3 + 1);
  *(char *)((long)memcpy(v4,a0,v3) + v3) = 0;
}

// Function: sub_17760 @ 0x17760
void sub_17760(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_27cf0(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_177a0 @ 0x177a0
char * sub_177a0(void *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  char *v3;
  
  v1 = sub_17760(a0);
  v2 = malloc(v1 + 1 + (unsigned long)(v1 == 0));
  v3 = NULL;
  if (v2) {
    v3 = memcpy(v2,a0,v1);
    if (!v1) {
      *v3 = 0x2e;
      v1 = 1;
    }
    v3[v1] = 0;
  }
  return v3;
}

// Function: sub_17810 @ 0x17810
bool sub_17810(char *a0)
{
  char v1;
  char *v2;
  
  v2 = (char *)sub_27cf0(a0);
  if (!*v2)
    v2 = a0;
  v1 = v2[sub_27d50(v2)];
  v2[sub_27d50(v2)] = '\0';
  return v1 != '\0';
}

// Function: sub_17850 @ 0x17850
unsigned long sub_17850(void *a0) // early-return
{
  int v1;
  int *v2; // rax
  
  if (0 <= fileno(a0)) {
    if (__freading(a0)) {
      v1 = fileno(a0);
      if (lseek(v1,0,1) == -1) {
        return fclose(a0); // tail-call, return-dupe
      }
    }
    if (sub_17bc0(a0)) {
      v2 = __errno_location();
      v1 = *v2;
      if (!v1)
        return CONCAT44(dat_4,fclose(a0));
      *v2 = v1;
      return 0xffffffff;
    }
  }
  return fclose(a0);
}

// Function: sub_178e0 @ 0x178e0
void sub_178e0(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_178f0 @ 0x178f0
int sub_178f0(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_3eea8) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_3eea8 = 1;
      return v2;
    }
    v2 = sub_178e0(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_3eea8 = -1;
  }
  else {
    v2 = sub_178e0(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_3eea8 != -1)
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

// Function: sub_179d0 @ 0x179d0
void sub_179d0(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_178e0(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_178f0(a0,a2);
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

// Function: sub_17b20 @ 0x17b20
void sub_17b20(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_26280(open(a0,a1,v1));
}

// Function: sub_17ba0 @ 0x17ba0
void sub_17ba0(unsigned int *a0)
{
  if (!(*a0 & 0x100))
    return;
  sub_17e40(a0,0,1); // tail-call
}

// Function: sub_17bc0 @ 0x17bc0
void sub_17bc0(void *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_17ba0(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_17c00 @ 0x17c00
unsigned long sub_17c00(unsigned int a0) // early-return x7
{
  a0 &= 0xf000;
  if (a0 == 0x8000)
    return 0x2d;
  if (a0 == 0x4000)
    return 100;
  if (a0 == 0x6000)
    return 0x62;
  if (a0 == 0x2000)
    return 99;
  if (a0 == 0xa000)
    return 0x6c;
  if (a0 == 0x1000)
    return 0x70;
  if (a0 != 0xc000)
    return 0x3f;
  return 0x73;
}

// Function: sub_17c70 @ 0x17c70
void sub_17c70(unsigned int a0,char *a1) // ternary x2
{
  unsigned char v1;
  char v2; // al
  unsigned long v3; // rcx
  
  v3 = (unsigned long)a0;
  *a1 = sub_17c00(a0);
  a1[1] = (-((v3 & 0x100) == 0) & 0xbbU) + 0x72;
  a1[2] = (-((v3 & 0x80) == 0) & 0xb6U) + 0x77;
  v1 = -((v3 & 0x40) == 0);
  v2 = (v3 & 0x800) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1[3] = v2;
  a1[4] = (-((v3 & 0x20) == 0) & 0xbbU) + 0x72;
  a1[5] = (-((v3 & 0x10) == 0) & 0xb6U) + 0x77;
  v1 = -((v3 & 8) == 0);
  v2 = (v3 & 0x400) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1[6] = v2;
  a1[7] = (-((v3 & 4) == 0) & 0xbbU) + 0x72;
  a1[8] = (-((v3 & 2) == 0) & 0xb6U) + 0x77;
  v1 = -((v3 & 1) == 0);
  if (v3 & 0x200) {
    a1[9] = (v1 & 0xe0) + 0x74;
    *(unsigned short *)&a1[10] = 0x20;
    return;
  }
  a1[9] = (v1 & 0xb5) + 0x78;
  *(unsigned short *)&a1[10] = 0x20;
}

// Function: sub_17d90 @ 0x17d90
void sub_17d90(long a0)
{
  sub_17c70(*(unsigned int *)(a0 + 0x18)); // tail-call
}

// Function: sub_17da0 @ 0x17da0
void * sub_17da0(char *a0,char *a1)
{
  int v1;
  unsigned int v2; // eax
  int v3;
  void *v4; // rax
  void *v5;
  int *v6; // rax
  
  v4 = fopen(a0,a1);
  v5 = v4;
  if ((v4) && (v2 = fileno(v4), v2 <= 2)) {
    v3 = sub_26260(v2);
    if (0 <= v3) { // branch-flip
      if ((!sub_17850(v4)) && (v5 = fdopen(v3,a1), v5))
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
      sub_17850(v4);
      *v6 = v3;
    }
  }
  return v5;
}

// Function: sub_17e40 @ 0x17e40
unsigned long sub_17e40(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  long v1; // rax
  
  if (((*(long *)&a0[4] == *(long *)&a0[2]) && (*(long *)&a0[10] == *(long *)&a0[8])) && (!*(long *)&a0[0x12])) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 != -1) {
      *a0 = *a0 & 0xffffffef;
      *(long *)&a0[0x24] = v1;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko(a0); // tail-call
}

// Function: sub_17ec0 @ 0x17ec0
unsigned long sub_17ec0(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_17ef0 @ 0x17ef0
unsigned long sub_17ef0(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_17f10 @ 0x17f10
unsigned long sub_17f10(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_17f20 @ 0x17f20
unsigned long sub_17f20(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_17f30 @ 0x17f30
int sub_17f30(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_17f50 @ 0x17f50
void sub_17f50(long a0,int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if ((unsigned int)(a1 - 1U) <= 0xb)
    v1 = *(unsigned int *)((unsigned long)(unsigned int)(a1 - 1U) * 4 + 0x2f3a0);
  *(unsigned int *)(a0 + 0x18) = v1;
}

// Function: sub_17f70 @ 0x17f70
long sub_17f70(long *a0)
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

// Function: sub_17fc0 @ 0x17fc0
unsigned long sub_17fc0(long a0,long a1)
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

// Function: sub_18030 @ 0x18030
void sub_18030(long a0,unsigned char a1)
{
  if (*(short *)(a0 + 0x68) == 0xb) {
    *(unsigned long *)(a0 + 0xa0) = (unsigned long)a1 + 1;
    return;
  }
  abort(); // no-return
}

// Function: sub_18050 @ 0x18050
long sub_18050(long a0,long a1,unsigned long a2)
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
label_180ab:
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
      goto label_180ab;
    }
    v4 = *(long **)(a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  return a1;
}

// Function: sub_18160 @ 0x18160
void * sub_18160(long a0,void *a1,unsigned long a2)
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

// Function: sub_18200 @ 0x18200
unsigned long sub_18200(long a0)
{
  long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) {
    v1 = sub_1ac70(0x1f,0,sub_17ef0,sub_17ec0,dat_3df78);
    *(long *)(a0 + 0x58) = v1;
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)(a0 + 0x58) = v2;
  if (v2) {
    sub_28450(v2);
    return 1;
  }
  return 0;
}

// Function: sub_18270 @ 0x18270
void sub_18270(void *a0)
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

// Function: sub_182c0 @ 0x182c0
void sub_182c0(unsigned long a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_1c0c0(a0)) break;
    v1 = sub_1c110(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_18300 @ 0x18300
unsigned long sub_18300(long a0,int a1)
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
    v2 = sub_1ac70(0xd,0,sub_17f10,sub_17f20,dat_3df78);
    *(long *)(v1 + 0x50) = v2;
    if (!v2) goto label_1838e;
  }
  v5[0] = *(unsigned long *)(a0 + 0x70);
  v1 = sub_1aa10(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_1838e:
  if ((0 <= a1) && (!fstatfs(a1,v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      *v3 = *(unsigned long *)(a0 + 0x70);
      v3[1] = v6[0];
      v4 = (unsigned long *)sub_1b290(v2,v3);
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

// Function: sub_18430 @ 0x18430
bool sub_18430(void)
{
  long v1; // rax
  bool v2; // r8b
  
  v1 = sub_18300();
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

// Function: sub_18480 @ 0x18480
unsigned long sub_18480(void)
{
  long v1; // rax
  
  v1 = sub_18300();
  if ((v1 != 0x1021994) && (v1 != 0xff534d42))
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0x6969);
  return 0;
}

// Function: sub_184c0 @ 0x184c0
void sub_184c0(long a0,long a1)
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

// Function: sub_18550 @ 0x18550
unsigned long sub_18550(long a0,unsigned long *a1) // return-dupe x2
{
  unsigned long v1;
  unsigned long *v2; // rax
  unsigned long *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v4 = sub_28470(*(unsigned long *)(a0 + 0x58),&a1[0xe]);
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
  v3 = (unsigned long *)sub_1b290(v1,v2);
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

// Function: sub_18600 @ 0x18600
void sub_18600(long a0,int a1,char a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_1c0d0(a0 + 0x60);
    if (0 <= v1) {
      close(v1);
      goto label_18644;
    }
  }
  else if ((!(*(unsigned char *)(a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_18644:
    *(int *)(a0 + 0x2c) = a1;
    return;
  }
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_18660 @ 0x18660
bool sub_18660(long a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)(a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
    sub_18600(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)(a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  sub_182c0(a0 + 0x60);
  return v2;
}

// Function: sub_186d0 @ 0x186d0
void sub_186d0(long a0,long a1)
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
    v4 = (void *)sub_1b2f0(*(unsigned long *)(a0 + 0x58),&v5);
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

// Function: sub_18790 @ 0x18790
void sub_18790(long a0,long a1,long a2)
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

// Function: sub_18810 @ 0x18810
int sub_18810(int a0,unsigned long a1,long a2,char a3) // early-return x3
{
  void *v1;
  unsigned long *v2;
  char *v3;
  unsigned int v4; // eax
  int v5;
  int *v6; // rax
  unsigned long v7; // rcx
  unsigned long *v8;
  
  v1 = (void *)(a2 + 0x70);
  if ((((*(long *)(a2 + 0x58)) || (!(a1 & 1))) && (!(a1 & 2))) && (!a3)) {
    v3 = *(char **)(a2 + 0x30);
    if (!fstatat(a0,v3,v1,0x100)) {
label_18892:
      v4 = *(unsigned int *)(a2 + 0x88) & 0xf000;
      if (v4 != 0x4000) { // branch-flip
        if (v4 == 0xa000)
          return 0xc;
        v5 = (unsigned int)(v4 == 0x8000) * 5 + 3;
      }
      else {
        if (*(char *)(a2 + 0x100) != '.')
          return 1;
        if ((*(char *)(a2 + 0x101)) && ((*(unsigned int *)(a2 + 0x100) & 0xffff00) != 0x2e00))
          return 1;
        v5 = (-(unsigned int)(*(long *)(a2 + 0x58) == 0) & 0xfffffffc) + 5;
      }
      return v5;
    }
    v5 = *__errno_location();
  }
  else {
    v3 = *(char **)(a2 + 0x30);
    if (!stat(v3,v1)) goto label_18892;
    v6 = __errno_location();
    v5 = *v6;
    if (v5 == 2) {
      v3 = *(char **)(a2 + 0x30);
      if (!lstat(v3,v1)) {
        *v6 = 0;
        return 0xd;
      }
      v5 = *v6;
    }
  }
  *(int *)(a2 + 0x40) = v5;
  v8 = (unsigned long *)(a2 + 0x78U & 0xfffffffffffffff8);
  *(unsigned long *)(a2 + 0x70) = 0;
  *(unsigned long *)(a2 + 0xf8) = 0;
  v7 = (unsigned long)(((int)v1 - (int)v8) + 0x90U >> 3);
  while (v7) {
    v2 = &v8[1];
    *v8 = 0;
    v7 -= 1;
    v8 = v2;
  }
  return 10;
}

// Function: sub_18970 @ 0x18970
void sub_18970(unsigned long a0,unsigned int a1,unsigned long a2)
{
  if (a1 & 0x200) {
    sub_1e690(a0,a2); // tail-call
    return;
  }
  sub_17b20(a2,(a1 & 0x10) << 0xd | 0x90900); // tail-call
}

// Function: sub_189b0 @ 0x189b0
int sub_189b0(long a0,long a1,int a2,char *a3) // return-dupe
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
label_18b40:
      if ((v6 & 0x200) && (0 <= a2)) {
        close(a2);
        return 0;
      }
      return 0;
    }
    if (a2 <= -1) {
      if (!(v6 & 0x200)) {
label_18a27:
        v7 = 1;
        goto label_18a2c;
      }
      v7 = sub_1c0c0(a0 + 0x60);
      if (v7) goto label_18a2c;
      v3 = sub_1c110(a0 + 0x60);
      v6 = *(unsigned int *)(a0 + 0x48);
      if (v3 < 0) goto label_18a27;
      v7 = 1;
      a2 = v3;
      v1 = v3;
      if (v6 & 2) goto label_18a58;
      goto label_18a90;
    }
    v7 = 1;
    if (!(v6 & 2)) goto label_18b1d;
label_18a58:
    v3 = a2;
    if (!fstat(v1,&v5)) {
      if ((*(long *)(a1 + 0x70) == v5) && (*(long *)(a1 + 0x78) == v8)) {
        v6 = *(unsigned int *)(a0 + 0x48);
        goto label_18a90;
      }
      *__errno_location() = 2;
    }
    v2 = -1;
  }
  else {
    v6 = *(unsigned int *)(a0 + 0x48);
    if (v6 & 4) goto label_18b40;
    if (0 <= a2) // branch-flip
      v7 = 0;
    else {
      v7 = 0;
label_18a2c:
      v1 = sub_18970(*(unsigned int *)(a0 + 0x2c),v6,a3);
      if (v1 < 0)
        return -1;
      v6 = *(unsigned int *)(a0 + 0x48);
    }
    if (v6 & 2) goto label_18a58;
    v3 = a2;
    if (a3) {
label_18b1d:
      v3 = a2;
      if (!strcmp(a3,"..")) goto label_18a58;
    }
label_18a90:
    if (v6 & 0x200) {
      sub_18600(a0,v1,v7 ^ 1);
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

// Function: sub_18c60 @ 0x18c60
void * sub_18c60(long *a0,int a1) // return-dupe
{
  unsigned char v1;
  unsigned long *v10; // rax
  unsigned long v11;
  void *v12; // rax
  long v13;
  unsigned int v14;
  int v15; // stack - 0x44
  char *v16;
  unsigned int v17;
  void *v18;
  unsigned long v19;
  long v2;
  unsigned long v20;
  bool v21;
  unsigned long v22; // stack - 0xa0
  void *v23; // stack - 0x98
  void *v24; // stack - 0x90
  char *v25; // stack - 0x70
  unsigned long v26; // stack - 0x68
  unsigned long v27; // stack - 0x60
  char v28; // stack - 0x4b
  char v29; // stack - 0x4a
  void *v3;
  long v4;
  bool v5;
  char v6; // al
  int v7; // eax
  int v8;
  int *v9; // rax
  
  v2 = *a0;
  v3 = *(void **)(v2 + 0x18);
  if (v3) { // branch-flip
    v15 = dirfd(v3);
    if (v15 < 0) {
      closedir(*(void **)(v2 + 0x18));
      *(unsigned long *)(v2 + 0x18) = 0;
      if (a1 == 3) {
        *(unsigned short *)(v2 + 0x68) = 4;
        *(int *)(v2 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (a0[8]) { // branch-flip
      v26 = 0xffffffffffffffff;
label_18cc2:
      v21 = 1;
      v14 = *(unsigned int *)&a0[9];
    }
    else {
      v26 = 100000;
      v14 = *(unsigned int *)&a0[9];
      v21 = 1;
    }
  }
  else {
    v14 = *(unsigned int *)&a0[9];
    v8 = 0;
    if ((v14 & 0x10) && (v8 = 0x20000, v14 & 1))
      v8 = (unsigned int)(*(long *)(v2 + 0x58) != 0) << 0x11;
    v17 = 0xffffff9c;
    if ((v14 & 0x204) == 0x200)
      v17 = *(unsigned int *)((long)a0 + 0x2c);
    v13 = sub_1e710(v17,*(unsigned long *)(v2 + 0x30),v8,&v15);
    *(long *)(v2 + 0x18) = v13;
    if (!v13) {
      if (a1 == 3) {
        *(unsigned short *)(v2 + 0x68) = 4;
        *(int *)(v2 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    v14 = *(unsigned int *)&a0[9];
    if (*(short *)(v2 + 0x68) != 0xb) { // branch-flip
      if (v14 & 0x100) {
        sub_186d0(a0,v2);
        sub_18810(*(unsigned int *)((long)a0 + 0x2c),(int)a0[9],v2,0);
        if (!sub_18550(a0,v2)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      v17 = *(unsigned int *)((long)a0 + 0x2c);
      *(unsigned short *)(v2 + 0x68) = sub_18810(v17,v14,v2,0);
    }
    v26 = (unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_19370:
      v21 = 0;
      v14 = *(unsigned int *)&a0[9];
      goto label_18ccb;
    }
    v8 = v15;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)(v2 + 0x80) != 2)) {
label_190ac:
      v21 = a1 == 3;
      v5 = 1;
    }
    else {
      v7 = sub_18430(v2,v15);
      v8 = v15;
      if (!v7) goto label_190ac;
      if (a1 != 3) goto label_19370;
      v5 = 0;
      v21 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v8 = sub_179d0(v8,0x406,3);
      v15 = v8;
    }
    if ((0 <= v8) && (!sub_189b0(a0,v2,v8,0))) goto label_18cc2;
    if ((v21) && (v5))
      *(int *)(v2 + 0x40) = *__errno_location();
    *(unsigned short *)(v2 + 0x6a) = *(unsigned short *)(v2 + 0x6a) | 1;
    closedir(*(void **)(v2 + 0x18));
    v14 = *(unsigned int *)&a0[9];
    *(unsigned long *)(v2 + 0x18) = 0;
    if ((v14 & 0x200) && (0 <= v15)) {
      close(v15);
      v14 = *(unsigned int *)&a0[9];
    }
    *(unsigned long *)(v2 + 0x18) = 0;
    v21 = 0;
  }
label_18ccb:
  v20 = *(unsigned long *)(v2 + 0x48);
  v22 = v20;
  v27 = v20 - 1;
  if (*(char *)(*(long *)(v2 + 0x38) + -1 + v20) != '/') {
    v22 = v20 + 1;
    v27 = v20;
  }
  v25 = NULL;
  if (v14 & 4) {
    v25 = &((char *)(v27 + a0[4]))[1];
    *(char *)(v27 + a0[4]) = 0x2f;
  }
  v18 = *(void **)(v2 + 0x18);
  v13 = *(long *)(v2 + 0x58);
  v20 = a0[6] - v22;
  if (v18) { // branch-flip
    v9 = __errno_location();
    v19 = 0;
    v29 = '\0';
    v28 = '\0';
    v23 = NULL;
    v24 = NULL;
    do {
      while( true ) {
        *v9 = 0;
        v10 = readdir(v18);
        if (!v10) {
          if (*v9) {
            *(int *)(v2 + 0x40) = *v9;
            *(unsigned short *)(v2 + 0x68) = (-(unsigned short)(!v3 && !v19) & 0xfffd) + 7;
          }
          if (*(void **)(v2 + 0x18)) {
            closedir(*(void **)(v2 + 0x18));
            *(unsigned long *)(v2 + 0x18) = 0;
          }
          goto label_19224;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (*(char *)((long)v10 + 0x13) != '.')) || ((*(char *)((long)v10 + 0x14) && (*(short *)((long)v10 + 0x14) != 0x2e)))) break;
label_18d93:
        v18 = *(void **)(v2 + 0x18);
        if (!v18) goto label_18ee8;
      }
      v11 = strlen((char *)((long)v10 + 0x13));
      v12 = (void *)sub_18160(a0,(char *)((long)v10 + 0x13),v11);
      if (!v12) {
label_19120:
        v8 = *v9;
        free(v12);
        sub_18270(v24);
        closedir(*(void **)(v2 + 0x18));
        *(unsigned long *)(v2 + 0x18) = 0;
        *(unsigned short *)(v2 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v9 = v8;
        return NULL;
      }
      if (v20 <= v11) {
        v4 = a0[4];
        v6 = sub_17fc0(a0,v27 + 2 + v11);
        if (!v6) goto label_19120;
        if (a0[4] != v4) {
          v16 = (char *)(a0[4] + v22);
          if (!(*(unsigned char *)&a0[9] & 4))
            v16 = v25;
          v28 = v6;
          v25 = v16;
        }
        v20 = a0[6] - v22;
      }
      if (CARRY8(v11,v22)) {
        free(v12);
        sub_18270(v24);
        closedir(*(void **)(v2 + 0x18));
        *(unsigned long *)(v2 + 0x18) = 0;
        *(unsigned short *)(v2 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v9 = 0x24;
        return NULL;
      }
      v14 = *(unsigned int *)&a0[9];
      *(long *)((long)v12 + 0x58) = v13 + 1;
      v4 = *a0;
      *(unsigned long *)((long)v12 + 0x48) = v11 + v22;
      *(long *)((long)v12 + 8) = v4;
      *(unsigned long *)((long)v12 + 0x78) = *v10;
      if (v14 & 4) { // branch-flip
        *(unsigned long *)((long)v12 + 0x30) = *(unsigned long *)((long)v12 + 0x38);
        memmove(v25,(void *)((long)v12 + 0x100),*(long *)((long)v12 + 0x60) + 1);
        v14 = *(unsigned int *)&a0[9];
      }
      else {
        *(void **)((long)v12 + 0x30) = (void *)((long)v12 + 0x100);
      }
      if ((a0[8]) && (!(v14 & 0x400))) {
        v17 = *(unsigned int *)((long)a0 + 0x2c);
        *(unsigned short *)((long)v12 + 0x68) = sub_18810(v17,v14,v12,0);
      }
      else {
        v1 = *(unsigned char *)((long)v10 + 0x12);
        v11 = 1;
        if ((v14 & 8) && (v1 & 0xfb))
          v11 = (unsigned long)((v14 >> 4 & 1 | (unsigned int)(v1 != 10)) ^ 1);
        *(unsigned short *)((long)v12 + 0x68) = 0xb;
        sub_17f50((long)v12 + 0x70,v1);
        sub_18030(v12,v11 & 0xffffffff);
      }
      *(unsigned long *)((long)v12 + 0x10) = 0;
      if (v24) // branch-flip
        *(void **)((long)v23 + 0x10) = v12;
      else {
        v24 = v12;
      }
      v11 = v19 + 1;
      if (v19 != 10000) {
        v19 = v11;
        if (v11 < v26) {
          v23 = v12;
          goto label_18d93;
        }
        goto label_19224;
      }
      if (!a0[8])
        v29 = sub_18480(v2,v15);
      v19 = 0x2711;
      v18 = *(void **)(v2 + 0x18);
      v23 = v12;
    } while (v18);
label_18ee8:
label_19224:
    if (v28)
      sub_18790(a0[1],a0[4],v24);
    if (*(unsigned char *)&a0[9] & 4) {
      if ((a0[6] == v22) || (!v19)) goto label_19254;
label_1925a:
      *v25 = 0;
    }
    if ((v3) || (!v21)) {
label_19300:
      if (!v19) goto label_19305;
    }
    else if ((a1 == 1) || (!v19)) {
label_192e0:
      if (*(long *)(v2 + 0x58)) // branch-flip
        v8 = sub_189b0(a0,*(unsigned long *)(v2 + 8),0xffffffff,"..");
      else {
        v8 = sub_18660(a0);
      }
      if (v8) {
        *(unsigned short *)(v2 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        sub_18270(v24);
        return NULL;
      }
      goto label_19300;
    }
    if (v29) { // branch-flip
      a0[8] = (long)sub_17f30;
      v24 = (void *)sub_18050(a0,v24,v19);
      a0[8] = 0;
    }
    else if ((a0[8]) && (v19 != 1))
      v24 = (void *)sub_18050(a0,v24,v19);
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v24 = NULL;
      v19 = 0;
      v29 = '\0';
label_19254:
      v25 = &v25[-1];
      goto label_1925a;
    }
    v24 = NULL;
    if ((bool)(v21 & v3 == NULL)) {
      v29 = '\0';
      v19 = 0;
      goto label_192e0;
    }
label_19305:
    if (((a1 == 3) && (*(short *)(v2 + 0x68) != 7)) && (*(short *)(v2 + 0x68) != 4))
      *(unsigned short *)(v2 + 0x68) = 6;
    sub_18270(v24);
    v24 = NULL;
  }
  return v24;
}

// Function: sub_195b0 @ 0x195b0
void sub_195b0(unsigned int a0,void *a1)
{
  if (!(a0 & 0x102)) {
    free(a1); // tail-call
    return;
  }
  if (a1) {
    sub_1ae30(a1); // tail-call
    return;
  }
}

// Function: sub_195e0 @ 0x195e0
long * sub_195e0(long *a0,unsigned int a1,long a2)
{
  unsigned int v1;
  long v10;
  long v11; // r13
  char *v12;
  void *v13; // stack - 0x50
  unsigned char v14; // stack - 0x42
  long v15; // stack - 0x40
  long v2;
  unsigned int v3;
  int v4; // eax
  long *v5; // rax
  unsigned long v6; // rax
  long v7;
  unsigned long v8;
  long v9;
  
  if (((a1 & 0xfffff000) || ((a1 & 0x204) == 0x204)) || (!(a1 & 0x12))) {
    *__errno_location() = 0x16;
    return NULL;
  }
  v5 = calloc(1,0x80);
  if (!v5)
    return NULL;
  v5[8] = a2;
  *(unsigned int *)((long)v5 + 0x2c) = 0xffffff9c;
  v3 = a1 & 0xfffffdff | 4;
  if (!(a1 & 2))
    v3 = a1;
  *(unsigned int *)&v5[9] = v3;
  v6 = sub_17f70(a0);
  v8 = 0x1000;
  if (0x1000 <= v6)
    v8 = v6;
  v14 = sub_17fc0(v5,v8);
  if (!v14) {
    free(v5); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v13 = (void *)sub_18160(v5,"",0);
    if (v13) {
      *(unsigned long *)((long)v13 + 0x58) = 0xffffffffffffffff;
      v12 = (char *)*a0;
      if (a2) goto label_196c9;
label_196d8:
      if (!v12) goto label_1991d;
      v15 = 0;
      v10 = 0;
      v9 = 0;
label_1977f:
      do {
        v11 = v9;
        v9 = v15;
        v8 = strlen(v12);
        if (((3 <= v8) && (((unsigned char)(a1 >> 0xb) ^ 1) & 1)) && (v12[v8 - 1] == '/')) {
          do {
            if (v12[v8 - 2] != '/') break;
            v8 -= 1;
          } while (v8 != 1);
        }
        v7 = sub_18160(v5,v12,v8);
        if (!v7) goto label_197f0;
        *(unsigned long *)(v7 + 0x58) = 0;
        *(void **)(v7 + 8) = v13;
        *(long *)(v7 + 0x30) = v7 + 0x100;
        if ((v10) && (v14)) {
          *(unsigned short *)(v7 + 0x68) = 0xb;
          sub_18030(v7,1);
          if (a2) { // branch-flip
label_19767:
            *(long *)(v7 + 0x10) = v10;
          }
          else {
            *(unsigned long *)(v7 + 0x10) = 0;
label_198f8:
            v15 = v7;
            *(long *)(v9 + 0x10) = v7;
            v7 = v10;
          }
          v12 = (char *)a0[v11 + 1];
          v10 = v7;
          v9 = v11 + 1;
          if (!v12) break;
          goto label_1977f;
        }
        v2 = v5[9];
        v1 = *(unsigned int *)((long)v5 + 0x2c);
        *(unsigned short *)(v7 + 0x68) = sub_18810(v1,(int)v2,v7,0);
        if (a2) goto label_19767;
        *(unsigned long *)(v7 + 0x10) = 0;
        if (v10) goto label_198f8;
        v12 = (char *)a0[v11 + 1];
        v10 = v7;
        v9 = v11 + 1;
        v15 = v7;
      } while (v12);
      if ((a2) && (2 <= v11 + 1U))
        v7 = sub_18050(v5,v7,v11 + 1U);
      goto label_1988c;
    }
  }
  else {
    v13 = NULL;
    if (a2) {
      v12 = NULL;
label_196c9:
      v14 = (unsigned char)(*(unsigned int *)&v5[9] >> 10) & 1;
      goto label_196d8;
    }
label_1991d:
    v7 = 0;
label_1988c:
    v9 = sub_18160(v5,"",0);
    *v5 = v9;
    v10 = v7;
    if (v9) {
      *(long *)(v9 + 0x10) = v7;
      *(unsigned short *)(v9 + 0x68) = 9;
      *(unsigned long *)(v9 + 0x58) = 1;
      if (sub_18200(v5)) {
        if (!(*(unsigned int *)&v5[9] & 0x204)) {
          v4 = sub_18970(*(unsigned int *)((long)v5 + 0x2c),*(unsigned int *)&v5[9],".");
          *(int *)&v5[5] = v4;
          if (v4 <= -1)
            *(unsigned int *)&v5[9] = *(unsigned int *)&v5[9] | 4;
        }
        sub_1c0a0(&v5[0xc],0xffffffff);
        return v5;
      }
    }
label_197f0:
    sub_18270(v10);
    free(v13);
  }
  free((void *)v5[4]);
  free(v5);
  return NULL;
}

// Function: sub_19950 @ 0x19950
unsigned int sub_19950(long *a0) // return-dupe
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
    sub_18270(a0[1]);
  v1 = &a0[0xc];
  free((void *)a0[2]);
  free((void *)a0[4]);
  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
    v6 = *(int *)((long)a0 + 0x2c);
    if ((v6 <= -1) || (!close(v6))) goto label_199e7;
    v6 = *__errno_location();
label_19a9d:
    sub_182c0(v1);
    v5 = a0[10];
    if (v5) goto label_199fc;
  }
  else {
    if (!(*(unsigned int *)&a0[9] & 4)) {
      v5 = a0[5];
      if (fchdir((int)v5)) {
        v3 = __errno_location();
        v5 = a0[5];
        v6 = *v3;
        if (!close((int)v5)) goto label_19a9d;
        if (!v6) goto label_19a58;
        sub_182c0(v1);
        v5 = a0[10];
        if (!v5) {
          sub_195b0((int)a0[9],a0[0xb]);
          free(a0);
          goto label_19a21;
        }
        goto label_199fc;
      }
      v5 = a0[5];
      if (close((int)v5)) {
        v3 = __errno_location();
label_19a58:
        v6 = *v3;
        goto label_19a9d;
      }
    }
label_199e7:
    v6 = 0;
    sub_182c0(v1);
    v5 = a0[10];
    if (!v5) {
      sub_195b0((int)a0[9],a0[0xb]);
      free(a0);
      return 0;
    }
label_199fc:
    sub_1ae30(v5);
  }
  sub_195b0((int)a0[9],a0[0xb]);
  free(a0);
  if (!v6)
    return 0;
  v3 = __errno_location();
label_19a21:
  *v3 = v6;
  return 0xffffffff;
}

// Function: sub_19b40 @ 0x19b40
void * sub_19b40(long *a0)
{
  short v1;
  long v10;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4;
  long v5;
  short v6;
  int v7;
  void *v8;
  void *v9;
  
  v8 = (void *)*a0;
  if (!v8)
    return NULL;
  v2 = *(unsigned int *)&a0[9];
  if (v2 & 0x2000)
    return NULL;
  v6 = *(short *)((long)v8 + 0x6c);
  *(unsigned short *)((long)v8 + 0x6c) = 3;
  if (v6 == 1) {
    v3 = *(unsigned int *)((long)a0 + 0x2c);
    *(unsigned short *)((long)v8 + 0x68) = sub_18810(v3,v2,v8,0);
    return v8;
  }
  v1 = *(short *)((long)v8 + 0x68);
  if (v6 != 2) { // branch-flip
    if (v1 != 1) { // branch-flip
label_19bd0:
      do {
        v9 = *(void **)((long)v8 + 0x10);
        if (!v9) {
          v9 = *(void **)((long)v8 + 8);
          if (*(long *)((long)v9 + 0x18)) {
            v5 = *(long *)((long)v9 + 0x48);
            *a0 = (long)v9;
            *(char *)(a0[4] + v5) = 0;
            v9 = (void *)sub_18c60(a0,3);
            if (v9) {
              free(v8);
              goto label_19e12;
            }
            if (*(unsigned char *)((long)a0 + 0x49) & 0x20)
              return NULL;
            v9 = *(void **)((long)v8 + 8);
          }
          *a0 = (long)v9;
          free(v8);
          if (*(long *)((long)v9 + 0x58) == -1) {
            free(v9);
            *__errno_location() = 0;
            *a0 = 0;
            return NULL;
          }
          if (*(short *)((long)v9 + 0x68) != 0xb) {
            *(char *)(a0[4] + *(long *)((long)v9 + 0x48)) = 0;
            if (*(long *)((long)v9 + 0x58)) { // branch-flip
              if (*(unsigned short *)((long)v9 + 0x6a) & 2) {
                v7 = *(int *)((long)v9 + 0x44);
                if (!(*(unsigned int *)&a0[9] & 4)) {
                  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
                    sub_18600(a0,v7,1);
                    v7 = *(int *)((long)v9 + 0x44);
                  }
                  else {
                    if (fchdir(v7)) {
                      *(int *)((long)v9 + 0x40) = *__errno_location();
                      *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
                    }
                    v7 = *(int *)((long)v9 + 0x44);
                  }
                }
                close(v7);
                goto label_19c3b;
              }
              if (*(unsigned short *)((long)v9 + 0x6a) & 1) goto label_19c3b;
              v7 = sub_189b0(a0,*(unsigned long *)((long)v9 + 8),0xffffffff,"..");
            }
            else {
              v7 = sub_18660(a0);
            }
            if (v7) {
              *(int *)((long)v9 + 0x40) = *__errno_location();
              *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            }
label_19c3b:
            if (*(short *)((long)v9 + 0x68) != 2) {
              if (*(int *)((long)v9 + 0x40)) // branch-flip
                *(unsigned short *)((long)v9 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v9 + 0x68) = 6;
                sub_186d0(a0,v9);
              }
            }
            if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20))
              return v9;
            return NULL;
          }
          abort(); // no-return, return-dupe
        }
        *a0 = (long)v9;
        free(v8);
        v8 = v9;
        if (!*(long *)((long)v9 + 0x58)) {
          if (sub_18660(a0)) {
            *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            return NULL;
          }
          sub_195b0((int)a0[9],a0[0xb]);
          sub_184c0(a0,v9);
          sub_18200(a0);
          v6 = *(short *)((long)v9 + 0x68);
          goto label_19db5;
        }
      } while (*(short *)((long)v9 + 0x6c) == 4);
      if (*(short *)((long)v9 + 0x6c) == 2) {
        v6 = sub_18810(*(unsigned int *)((long)a0 + 0x2c),(int)a0[9],v9,1);
        *(short *)((long)v9 + 0x68) = v6;
        if ((v6 == 1) && (!(*(unsigned int *)&a0[9] & 4))) {
          v7 = sub_18970(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
          *(int *)((long)v9 + 0x44) = v7;
          if (0 <= v7) // branch-flip
            *(unsigned short *)((long)v9 + 0x6a) = *(unsigned short *)((long)v9 + 0x6a) | 2;
          else {
            v7 = *__errno_location();
            *(unsigned short *)((long)v9 + 0x68) = 7;
            *(int *)((long)v9 + 0x40) = v7;
          }
        }
        *(unsigned short *)((long)v9 + 0x6c) = 3;
      }
    }
    else {
      if (v6 == 4) goto label_19d3a;
label_19c9f:
      if ((v2 & 0x40) && (*(long *)((long)v8 + 0x70) != a0[3])) {
label_19d3a:
        if (*(unsigned char *)((long)v8 + 0x6a) & 2)
          close(*(int *)((long)v8 + 0x44));
        if (a0[1]) {
          sub_18270(a0[1]);
          a0[1] = 0;
        }
        *(unsigned short *)((long)v8 + 0x68) = 6;
        sub_186d0(a0,v8);
        return v8;
      }
      if (a0[1]) { // branch-flip
        if (v2 & 0x1000) {
          *(unsigned int *)&a0[9] = v2 & 0xffffefff;
          sub_18270(a0[1]);
          a0[1] = 0;
          goto label_19eb6;
        }
        v4 = *(unsigned long *)((long)v8 + 0x30);
        if (sub_189b0(a0,v8,0xffffffff,v4)) { // branch-flip
          v7 = *__errno_location();
          *(unsigned short *)((long)v8 + 0x6a) = *(unsigned short *)((long)v8 + 0x6a) | 1;
          *(int *)((long)v8 + 0x40) = v7;
          v9 = (void *)a0[1];
          if (v9) {
            v8 = v9;
            do {
              *(unsigned long *)((long)v8 + 0x30) = *(unsigned long *)(*(long *)((long)v8 + 8) + 0x30);
              v8 = *(void **)((long)v8 + 0x10);
            } while (v8);
          }
        }
        else {
          v9 = (void *)a0[1];
        }
      }
      else {
label_19eb6:
        v9 = (void *)sub_18c60(a0,3);
        a0[1] = (long)v9;
        if (!v9) {
          if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20)) {
            if ((*(int *)((long)v8 + 0x40)) && (*(short *)((long)v8 + 0x68) != 4))
              *(unsigned short *)((long)v8 + 0x68) = 7;
            sub_186d0(a0,v8);
            return v8;
          }
          return NULL;
        }
      }
      a0[1] = 0;
    }
label_19e12:
    v5 = *(long *)(*(long *)((long)v9 + 8) + 0x48);
    v10 = v5 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v9 + 8) + 0x38) + -1 + v5) != '/')
      v10 = v5;
    v5 = a0[4];
    *(char *)(v10 + v5) = 0x2f;
    memmove(&((char *)(v10 + v5))[1],(void *)((long)v9 + 0x100),*(long *)((long)v9 + 0x60) + 1);
    v6 = *(short *)((long)v9 + 0x68);
    *a0 = (long)v9;
    v8 = v9;
label_19dbd:
    if (v6 == 0xb) goto label_19e61;
  }
  else {
    if (2 <= (unsigned short)(v1 - 0xcU)) {
      if (v1 == 1) goto label_19c9f;
      goto label_19bd0;
    }
    v6 = sub_18810(*(unsigned int *)((long)a0 + 0x2c),v2,v8,1);
    *(short *)((long)v8 + 0x68) = v6;
    if (v6 == 1) {
      if (*(unsigned int *)&a0[9] & 4) {
        *a0 = (long)v8;
        goto label_19dcd;
      }
      v7 = sub_18970(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
      *(int *)((long)v8 + 0x44) = v7;
      if (v7 < 0) {
        v7 = *__errno_location();
        *(unsigned short *)((long)v8 + 0x68) = 7;
        *(int *)((long)v8 + 0x40) = v7;
        *a0 = (long)v8;
        return v8;
      }
      *(unsigned short *)((long)v8 + 0x6a) = *(unsigned short *)((long)v8 + 0x6a) | 2;
      v6 = *(short *)((long)v8 + 0x68);
label_19db5:
      *a0 = (long)v8;
      goto label_19dbd;
    }
    *a0 = (long)v8;
    if (v6 != 0xb)
      return v8;
label_19e61:
    if (*(long *)((long)v8 + 0xa0) != 2) {
      if (*(long *)((long)v8 + 0xa0) == 1)
        return v8;
      abort();
    }
    v6 = sub_18810(*(unsigned int *)((long)a0 + 0x2c),(int)a0[9],v8,0);
    *(short *)((long)v8 + 0x68) = v6;
  }
  if (v6 != 1)
    return v8;
label_19dcd:
  if (!*(long *)((long)v8 + 0x58))
    a0[3] = *(long *)((long)v8 + 0x70);
  if (!sub_18550(a0,v8)) {
    v8 = NULL;
    *__errno_location() = 0xc;
  }
  return v8;
}

// Function: sub_1a190 @ 0x1a190
unsigned long sub_1a190(unsigned long a0,long a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)(a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_1a1c0 @ 0x1a1c0
int * sub_1a1c0(long *a0,unsigned int a1) // early-return x2
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
    sub_18270(a0[1]);
  v6 = 1;
  if (a1 == 0x1000) {
    *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x1000;
    v6 = 2;
  }
  if (((*(long *)(v1 + 0x58)) || (**(char **)(v1 + 0x30) == '/')) || (*(unsigned int *)&a0[9] & 4)) {
    v5 = (long)sub_18c60(a0,v6);
    a0[1] = (long)v5;
  }
  else {
    v4 = sub_18970(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
    v2 = v4;
    if (v4 < 0) {
      a0[1] = 0;
      v5 = NULL;
      return v5;
    }
    v5 = (long)sub_18c60(a0,v6);
    a0[1] = (long)v5;
    if (*(unsigned char *)((long)a0 + 0x49) & 2) // branch-flip
      sub_18600(a0,v2,1);
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

// Function: sub_1a340 @ 0x1a340
unsigned long sub_1a340(unsigned long a0)
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

// Function: sub_1a3b0 @ 0x1a3b0
unsigned long sub_1a3b0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_1a340(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_1a3f0 @ 0x1a3f0
unsigned long sub_1a3f0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_27dd0(a0,3) % a1;
}

// Function: sub_1a410 @ 0x1a410
bool sub_1a410(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_1a420 @ 0x1a420
unsigned long sub_1a420(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x2f440)
    return 1;
  v1 = v2[2];
  if ((((dat_2ec74 < v1) && (v1 < dat_2ec80)) && (dat_2f454 < v2[3])) && (dat_2f460 <= *v2)) {
    v3 = *v2 + dat_2ec74;
    if (((v3 < v2[1]) && (v2[1] <= dat_2c588)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x2f440;
  return 0;
}

// Function: sub_1a4b0 @ 0x1a4b0
void sub_1a4b0(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_1a4d0 @ 0x1a4d0
long sub_1a4d0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}

// Function: sub_1a500 @ 0x1a500
long sub_1a500(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_1a4d0(a0,a1);
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
      sub_1a4b0(a0);
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
      sub_1a4b0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_1a600 @ 0x1a600
void sub_1a600(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_1a630 @ 0x1a630
unsigned long sub_1a630(long a0,unsigned long *a1,char a2) // return-dupe x2
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
label_1a658:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_1a4d0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_1a4b0(a0);
          v3 = v4;
          if (!v4) goto label_1a6c8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_1a6c8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_1a658;
    v3 = (long *)sub_1a4d0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_1a600(a0);
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

// Function: sub_1a760 @ 0x1a760
unsigned long sub_1a760(float a0,unsigned long a1,char a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_2f458 <= a0)
      return 0;
    a1 = (a0 < dat_2f45c) ? (unsigned long)a0 : (long)(a0 - dat_2f45c) ^ 0x8000000000000000;
  }
  v1 = sub_1a3b0(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_1a7f0 @ 0x1a7f0
unsigned long sub_1a7f0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_1a800 @ 0x1a800
unsigned long sub_1a800(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_1a810 @ 0x1a810
unsigned long sub_1a810(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_1a820 @ 0x1a820
unsigned long sub_1a820(unsigned long *a0)
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

// Function: sub_1a880 @ 0x1a880
bool sub_1a880(unsigned long *a0)
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
label_1a8a0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_1a8a0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_1a8f0 @ 0x1a8f0
void sub_1a8f0(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_1a810(a0);
  v2 = sub_1a7f0();
  v3 = sub_1a800();
  v4 = sub_1a820();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_1aa10 @ 0x1aa10
long sub_1aa10(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_1a4d0(a0,a1);
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

// Function: sub_1aa70 @ 0x1aa70
long sub_1aa70(unsigned long *a0)
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

// Function: sub_1aac0 @ 0x1aac0
long sub_1aac0(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_1a4d0(a0,a1);
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

// Function: sub_1ab30 @ 0x1ab30
void sub_1ab30(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_1ab80 @ 0x1ab80
long sub_1ab80(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_1ac00 @ 0x1ac00
unsigned long sub_1ac00(unsigned char *a0,unsigned long a1)
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

// Function: sub_1ac40 @ 0x1ac40
void sub_1ac40(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_1ac70 @ 0x1ac70
unsigned long * sub_1ac70(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_1a3f0;
  if (!a3)
    a3 = sub_1a410;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x2f440;
  v1[5] = a1;
  if (sub_1a420(v1)) {
    v2 = sub_1a760(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_1ad80 @ 0x1ad80
void sub_1ad80(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_1ae1b;
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
label_1ae1b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_1ae30 @ 0x1ae30
void sub_1ae30(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_1aedc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_1ae9f;
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
label_1ae9f:
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
label_1aedc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_1af20 @ 0x1af20
unsigned int sub_1af20(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_1a760(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_1a630(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_1a630(a0,&v2,1)) && (sub_1a630(a0,&v2,0))) {
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

// Function: sub_1b0a0 @ 0x1b0a0
unsigned long sub_1b0a0(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_1a500(a0,a1,&v3,0);
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
      sub_1a420(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_2f458 <= v5)
          return 0xffffffff;
        v4 = (dat_2f45c <= v5) ? (long)(v5 - dat_2f45c) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_1af20(a0,v4))
          return 0xffffffff;
        if (sub_1a500(a0,a1,&v3,0)) {
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
    v2 = (long *)sub_1a600(a0);
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

// Function: sub_1b290 @ 0x1b290
unsigned long sub_1b290(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_1b0a0(a0,a1,&v3);
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

// Function: sub_1b2f0 @ 0x1b2f0
long sub_1b2f0(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_1a500(a0,a1,&v7,1);
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
      sub_1a420(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_2f45c <= v8) ? (long)(v8 - dat_2f45c) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_1af20(a0,v6)) {
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

// Function: sub_1b480 @ 0x1b480
void sub_1b480(void)
{
  sub_1b2f0(); // tail-call
}

// Function: sub_1b490 @ 0x1b490
void sub_1b490(void)
{
  return;
}

// Function: sub_1b560 @ 0x1b560
void * sub_1b560(void *a0,unsigned long a1,unsigned char *a2,char *a3)
{
  unsigned char v1;
  unsigned long v2; // rax
  void *v3; // rax
  char v4 [56];
  char *v5;
  unsigned long v6;
  unsigned long v7; // r15
  
  v2 = strlen(a3);
  __memcpy_chk(v4,a0,a1,0x29);
  a0 = (void *)((long)a0 + a1);
  v6 = 0xffffffffffffffff;
  while( true ) {
    v1 = *a2;
    if (v1) { // branch-flip
      if (0x7f <= v1) { // branch-flip
        v7 = 0;
        v5 = v4;
      }
      else {
        v6 = (unsigned long)v1;
        if (a1 < v1)
          v6 = a1;
        v7 = a1 - v6;
        v5 = &v4[v7];
        a1 = v6;
      }
      a2 = &a2[1];
      v6 = a1;
    }
    else {
      if (a1 < v6)
        v6 = a1;
      v7 = a1 - v6;
      v5 = &v4[v7];
    }
    v3 = memcpy((void *)((long)a0 - v6),v5,v6);
    if (!v7) break;
    a0 = memcpy((void *)((long)v3 - v2),a3,v2);
    a1 = v7;
  }
  return v3;
}

// Function: sub_1b660 @ 0x1b660
int sub_1b660(void)
{
  return (-(unsigned int)(getenv("POSIXLY_CORRECT") == NULL) & 0x200) + 0x200;
}

// Function: sub_1b690 @ 0x1b690
unsigned long sub_1b690(char *a0,unsigned long *a1,unsigned int *a2)
{
  char *v1;
  char v2;
  int v3; // eax
  unsigned long v4; // rax
  char *v5; // stack - 0x38
  unsigned int v6; // r13d
  
  if (((a0) || (a0 = getenv("BLOCK_SIZE"), a0)) || (a0 = getenv("BLOCKSIZE"), a0)) {
    v6 = 0;
    if (*a0 == '\'') {
      a0 = &a0[1];
      v6 = 4;
    }
    v3 = sub_277a0(a0,0x3c390,0x2f4a8,4);
    if (0 <= v3) { // branch-flip
      *a1 = 1;
      v6 |= *(unsigned int *)((long)v3 * 4 + 0x2f4a8);
    }
    else {
      v4 = sub_27300(a0,&v5,0,a1,"eEgGkKmMpPtTyYzZ0");
      if ((int)v4) {
        *a2 = 0;
        return v4;
      }
      v2 = *a0;
      while (10 <= (unsigned char)(v2 - 0x30U)) {
        if (v5 == a0) {
          if (v5[-1] != 'B') // branch-flip
            v6 |= 0x80;
          else {
            v6 |= 0x180;
            if (v5[-2] != 'i') break;
          }
          v6 |= 0x20;
          break;
        }
        v1 = &a0[1];
        a0 = &a0[1];
        v2 = *v1;
      }
    }
  }
  else {
    v6 = 0;
    *a1 = sub_1b660();
  }
  *a2 = v6;
  return 0;
}

// Function: sub_1b800 @ 0x1b800
char * sub_1b800(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary
{
  unsigned int v1;
  char v10; // al
  unsigned int v11; // eax
  unsigned int v12; // eax
  unsigned long *v13; // rax
  unsigned long v14;
  unsigned long v15;
  unsigned long v16;
  unsigned long v17;
  long v18; // rax
  unsigned int v19; // edx
  unsigned long v2;
  unsigned int v20; // ebx
  char *v21;
  int v22; // esi
  unsigned int v23; // eax
  int v24;
  char *v25;
  unsigned long v26;
  char *v27;
  long double v28; // st0
  long double v29;
  bool v3;
  char *v30; // stack - 0x78
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  unsigned long v9;
  
  v11 = a2 & 3;
  v13 = (unsigned long *)localeconv();
  v21 = (char *)*v13;
  v20 = (-(unsigned int)((a2 & 0x20) == 0) & 0xffffffe8) + 0x400;
  v14 = strlen(v21);
  if (0x10 <= v14 - 1) {
    v14 = 1;
    v21 = ".";
  }
  v2 = v13[2];
  v27 = (char *)v13[1];
  if (0x11 <= strlen(v27))
    v27 = "";
  v30 = &a1[0x287];
  v19 = (unsigned int)v14; // branch-flip
  v25 = v30;
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
        v23 = 0;
        v17 = 0;
        v12 = 0;
        if (a2 & 0x10) goto label_1bb60;
label_1bcf5:
        v24 = -1;
label_1bb6f:
        if (v11 != 1) goto label_1bb7e;
        if (5 < (int)((((unsigned int)v15 & 1) + v23 != 0) + v12)) goto label_1bb8f;
        goto label_1bbb0;
      }
    }
label_1b8da:
    v28 = (long double)(long)a3;
    if ((long)a3 < 0)
      v28 = v28 + (long double)dat_2f458;
    if (0 <= (long)a4) { // branch-flip
      v28 = v28 / (long double)(long)a4;
      if ((long)a0 > -1) goto label_1b917;
label_1ba00:
      v28 = ((long double)(long)a0 + (long double)dat_2f458) * v28;
      if (!(a2 & 0x10)) goto label_1ba48;
label_1b923:
      v24 = 0;
      v29 = (long double)(int)v20;
      do {
        v24 += 1;
        v29 = v29 * (long double)(int)v20;
        if (v28 < v29) break;
      } while (v24 != 8);
      v22 = 1;
      v21 = "%.1Lf";
      sub_1b490(v11);
      __sprintf_chk(a1,v22,0xffffffffffffffff,v21);
      v16 = strlen(a1);
      if ((v16 <= v14 + 2 + (unsigned long)((a2 & 0x20) == 0)) && ((!(a2 & 8) || (a1[v16 - 1] != '0'))))
        v14 = v16 - (v14 + 1);
      else {
        v21 = "%.0Lf";
        v22 = 1;
        sub_1b490(v11);
        __sprintf_chk(a1,v22,0xffffffffffffffff,v21);
        v16 = strlen(a1);
        v14 = v16;
      }
    }
    else {
      v28 = v28 / ((long double)(long)a4 + (long double)dat_2f458);
      if (0 > (long)a0) goto label_1ba00;
label_1b917:
      v28 = (long double)(long)a0 * v28;
      if (a2 & 0x10) goto label_1b923;
label_1ba48:
      v24 = 1;
      v21 = "%.0Lf";
      sub_1b490(v11);
      __sprintf_chk(a1,v24,0xffffffffffffffff,v21);
      v16 = strlen(a1);
      v24 = -1;
      v14 = v16;
    }
    v21 = memmove(&v30[-v16],a1,v16);
    v25 = &v21[v14];
  }
  else {
    if ((!a3) || (v16 = a4 / a3, a4 % a3)) goto label_1b8da;
    v15 = a0 / v16;
    v17 = (a0 % v16) * 10;
    v9 = v17 / v16;
    v26 = (v17 % v16) * 2;
    v8._8_8_ = v26;
    v8._0_8_ = v15;
    v17 = v9 & 0xffffffff;
    v12 = (unsigned int)v9;
    if (v16 <= v26) { // branch-flip
      v23 = 2;
      if (v16 < v26)
        v23 = 3;
    }
    else {
      v23 = (unsigned int)(v26 != 0);
    }
    if (!(a2 & 0x10)) goto label_1bcf5;
label_1bb60:
    v12 = (unsigned int)v17;
    v15 = SUB168(v8._0_16_,0);
    v26 = (unsigned long)v20;
    v24 = 0;
    v16 = v15;
    if (v15 < v26) goto label_1bb6f;
    while( true ) {
      v15 = v16 / v26;
      v12 = (int)v17 + (int)(v16 % v26) * 10;
      v17 = (unsigned long)v12 / (unsigned long)v20;
      v1 = ((int)v23 >> 1) + (v12 % v20) * 2;
      v12 = (unsigned int)v17;
      v23 += v1;
      v23 = (v20 <= v1) ? (v20 < v23) + 2 : (unsigned int)(v23 != 0); // branch-flip
      v24 += 1;
      if (v15 < v26) break;
      v16 = v15;
      if (v24 == 8) goto label_1bb6f;
    }
    if (10 <= v15) goto label_1bb6f;
    v16 = v15;
    if (v11 != 1) { // branch-flip
      if ((!(a2 & 3)) && (v23)) goto label_1be72;
label_1beea:
      if (!v12) goto label_1bef3;
      v10 = (char)v17 + '0';
    }
    else {
      if (3 > (v12 & 1) + v23) goto label_1beea;
label_1be72:
      v10 = (char)v17 + '1';
      if (v12 == 9) {
        v16 = v15 + 1;
        if (v15 == 9) goto label_1bebc;
        v23 = 0;
        v15 = v16;
label_1bef3:
        if (a2 & 8) goto label_1bebe;
        v10 = '0';
        v16 = v15;
      }
    }
    a1[0x286] = v10;
    v17 = v14 & 0xffffffff;
    v25 = &a1[0x286 - v14];
    if (8 <= v19) { // branch-flip
      *(unsigned long *)v25 = *(unsigned long *)v21;
      *(unsigned long *)&v25[(v14 & 0xffffffff) - 8] = *(unsigned long *)&v21[(v14 & 0xffffffff) - 8];
      v18 = (long)v25 - ((unsigned long)&v25[8] & 0xfffffffffffffff8);
      v12 = (int)v18 + v19 & 0xfffffff8;
      if (8 <= v12) {
        v23 = 0;
        do {
          v17 = (unsigned long)v23;
          v23 += 8;
          *(unsigned long *)(((unsigned long)&v25[8] & 0xfffffffffffffff8) + v17) = *(unsigned long *)&v21[v17 - v18];
        } while (v23 < v12);
      }
    }
    else if (v14 & 4) { // branch-flip
      *(unsigned int *)v25 = *(unsigned int *)v21;
      *(unsigned int *)&v25[v17 - 4] = *(unsigned int *)&v21[v17 - 4];
    }
    else if (v19) {
      *v25 = *v21;
      if (v14 & 2)
        *(unsigned short *)&v25[v17 - 2] = *(unsigned short *)&v21[v17 - 2];
    }
label_1bebc:
    v23 = 0;
    v15 = v16;
label_1bebe:
    v12 = 0;
    if (v11 == 1) goto label_1bbb0;
label_1bb7e:
    if (a2 & 3) { // branch-flip
label_1bbb0:
      v21 = v25;
      do {
        v21 = &v21[-1];
        *v21 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
        v3 = 10 <= v15;
        v15 /= 10;
      } while (v3);
    }
    else {
      if (1 > (int)(v23 + v12)) goto label_1bbb0;
label_1bb8f:
      v15 += 1;
      if (((!(a2 & 0x10)) || (v20 != v15)) || (v24 == 8)) goto label_1bbb0;
      v24 += 1;
      if (!(a2 & 8)) {
        v25[-1] = '0';
        v25 = &v25[~v14];
        v16 = v14 & 0xffffffff;
        if (8 <= v19) { // branch-flip
          *(unsigned long *)v25 = *(unsigned long *)v21;
          *(unsigned long *)&v25[(v14 & 0xffffffff) - 8] = *(unsigned long *)&v21[(v14 & 0xffffffff) - 8];
          v18 = (long)v25 - ((unsigned long)&v25[8] & 0xfffffffffffffff8);
          v11 = (int)v18 + v19 & 0xfffffff8;
          if (8 <= v11) {
            v19 = 0;
            do {
              v14 = (unsigned long)v19;
              v19 += 8;
              *(unsigned long *)(((unsigned long)&v25[8] & 0xfffffffffffffff8) + v14) = *(unsigned long *)&v21[v14 - v18];
            } while (v19 < v11);
          }
        }
        else if (v14 & 4) { // branch-flip
          *(unsigned int *)v25 = *(unsigned int *)v21;
          *(unsigned int *)&v25[v16 - 4] = *(unsigned int *)&v21[v16 - 4];
        }
        else if ((v19) && (*v25 = *v21, v14 & 2))
          *(unsigned short *)&v25[v16 - 2] = *(unsigned short *)&v21[v16 - 2];
      }
      v25[-1] = '1';
      v21 = &v25[-1];
    }
  }
  if (a2 & 4)
    v21 = (char *)sub_1b560(v21,(long)v25 - (long)v21,v2,v27);
  if (!(a2 & 0x80)) goto label_1bade;
  if (v24 != -1) { // branch-flip
label_1bacb:
    if (!(a2 & 0x100) && !v24) goto label_1bade;
    if (a2 & 0x40) {
label_1bd41:
      v30 = &a1[0x288];
      a1[0x287] = ' ';
    }
    if (v24) goto label_1bd5f;
    v27 = v30;
    if (!(a2 & 0x100)) goto label_1bade;
  }
  else {
    if (a4 < 2) {
      v24 = 0;
      goto label_1bacb;
    }
    v24 = 1;
    v14 = 1;
    do {
      v14 *= v20;
      if (a4 <= v14) break;
      v24 += 1;
    } while (v24 != 8);
    if (a2 & 0x40) goto label_1bd41;
label_1bd5f:
    v27 = &v30[1];
    if ((a2 & 0x20) || (v24 != 1)) {
      *v30 = *(char *)((long)v24 + 0x2f4b0);
      if (!(a2 & 0x100)) {
label_1be3b:
        v30 = v27;
        goto label_1bade;
      }
      if (a2 & 0x20) {
        v30[1] = 'i';
        v27 = &v30[2];
      }
    }
    else {
      *v30 = 'k';
      if (!(a2 & 0x100)) goto label_1be3b;
    }
  }
  *v27 = 'B';
  v30 = &v27[1];
label_1bade:
  *v30 = '\0';
  return v21;
}

// Function: sub_1c070 @ 0x1c070
unsigned long sub_1c070(unsigned long a0,unsigned long a1,long *a2)
{
  unsigned long v1; // rax
  
  v1 = sub_1b690(a0,a2,a1);
  if (*a2)
    return v1;
  *a2 = sub_1b660();
  return 4;
}

// Function: sub_1c0a0 @ 0x1c0a0
void sub_1c0a0(unsigned int *a0,unsigned int a1)
{
  *(unsigned long *)&a0[5] = 0;
  *(char *)&a0[7] = 1;
  *a0 = a1;
  a0[1] = a1;
  a0[2] = a1;
  a0[3] = a1;
  a0[4] = a1;
}

// Function: sub_1c0c0 @ 0x1c0c0
char sub_1c0c0(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_1c0d0 @ 0x1c0d0
unsigned int sub_1c0d0(long a0,unsigned int a1)
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

// Function: sub_1c110 @ 0x1c110
unsigned int sub_1c110(long a0)
{
  unsigned int v1;
  unsigned int v2;
  char v3; // al
  
  v3 = sub_1c0c0(a0);
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

// Function: sub_1c170 @ 0x1c170
unsigned int * sub_1c170(unsigned int a0) // early-return
{
  unsigned long *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_3eec8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1c19d;
  }
  v3 = "";
  v1 = getpwuid(a0);
  if (v1)
    v3 = (char *)*v1;
  v2 = (unsigned int *)sub_26990(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_3eec8;
  dat_3eec8 = v2;
label_1c19d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_1c200 @ 0x1c200
unsigned int * sub_1c200(char *a0)
{
  char v1;
  void *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_3eec8) {
    v1 = *a0;
    v3 = dat_3eec8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_3eec0) {
    v1 = *a0;
    v4 = (long)dat_3eec0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getpwnam(a0);
  v3 = (unsigned int *)sub_26990(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_3eec0;
  if (v2) {
    *v3 = *(unsigned int *)((long)v2 + 0x10);
    *(unsigned int **)&v3[2] = dat_3eec8;
    dat_3eec8 = v3;
    return v3;
  }
  dat_3eec0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_1c310 @ 0x1c310
unsigned int * sub_1c310(unsigned int a0) // early-return
{
  unsigned long *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_3eeb8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1c33d;
  }
  v3 = "";
  v1 = getgrgid(a0);
  if (v1)
    v3 = (char *)*v1;
  v2 = (unsigned int *)sub_26990(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_3eeb8;
  dat_3eeb8 = v2;
label_1c33d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_1c3a0 @ 0x1c3a0
unsigned int * sub_1c3a0(char *a0)
{
  char v1;
  void *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_3eeb8) {
    v1 = *a0;
    v3 = dat_3eeb8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_3eeb0) {
    v1 = *a0;
    v4 = (long)dat_3eeb0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getgrnam(a0);
  v3 = (unsigned int *)sub_26990(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_3eeb0;
  if (v2) {
    *v3 = *(unsigned int *)((long)v2 + 0x10);
    *(unsigned int **)&v3[2] = dat_3eeb8;
    dat_3eeb8 = v3;
    return v3;
  }
  dat_3eeb0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_1c4b0 @ 0x1c4b0
unsigned long sub_1c4b0(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_28580(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_1c540 @ 0x1c540
unsigned long sub_1c540(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_286c0(v9);
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
              goto label_1c638;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_1c638:
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
      *(unsigned long *)&v8[-8] = 0x1c68e;
      sub_28710(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_1c6e0 @ 0x1c6e0
char sub_1c6e0(void *a0,void *a1,long *a2)
{
  char v1;
  void *v10; // stack - 0xf8
  void *v11; // stack - 0x78
  char v12 [4]; // stack - 0x88
  char v13 [4]; // stack - 0xc8
  void **v14;
  char *v15; // rsp
  void **v16; // rsp
  void **v17;
  unsigned long v18;
  long v19;
  char v2 [16];
  void *v20;
  unsigned long *v21;
  long *v22; // stack - 0xf0
  char v23; // stack - 0xe1
  void *v24; // stack - 0xe0
  void *v25; // stack - 0xd8
  char *v26; // stack - 0xd0
  unsigned long v27; // stack - 0xc4
  char v28; // stack - 0xbc
  void *v29; // stack - 0xb8
  char v3;
  long v30; // stack - 0xb0
  char v31; // stack - 0xa8
  int v32; // stack - 0xa4
  unsigned long v33; // stack - 0x84
  char v34; // stack - 0x7c
  unsigned long v35; // stack - 0x70
  char v36; // stack - 0x68
  int v37; // stack - 0x64
  void *v4;
  int v5; // eax
  unsigned long *v6; // rax
  unsigned long *v7;
  void *v8;
  unsigned long *v9; // rbx
  
  v16 = &v10;
  v14 = &v10;
  v17 = &v10;
  v10 = a0;
  v22 = a2;
  v24 = (void *)sub_287f0(a1);
  v2._8_8_ = 0;
  v2._0_8_ = v24;
  if ((0 <= SUB168(ZEXT816(0x38) * v2._0_16_,0)) && (!SUB168(ZEXT816(0x38) * v2._0_16_,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v21 = (unsigned long *)sub_286c0(v18);
      v4 = v24;
    }
    else {
      v17 = &v10;
      while (v16 != (void **)((long)&v10 - (v18 + 0x36 & 0xfffffffffffff000))) {
        v14 = (char *)((long)v17 + -0x1000);
        v15 = (char *)((long)v17 + -0x1000);
        *(unsigned long *)((long)v17 + -8) = *(unsigned long *)((long)v17 + -8);
        v16 = (char *)((long)v17 + -0x1000);
        v17 = (void **)v15;
      }
      v18 = (unsigned long)((unsigned int)(v18 + 0x36) & 0xff0);
      v19 = -v18;
      v17 = (char *)((long)v14 + v19);
      if (v18)
        *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
      v21 = (unsigned long *)(((unsigned long)((long)v14 + v19 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v4 = v24;
    }
    if (v21) {
      v12[0] = 0;
      v26 = v12;
      v33 = 0;
      v7 = v21;
      v24 = v4;
      v11 = a1;
      while( true ) {
        v34 = 0;
        *(unsigned long *)((long)v17 + -8) = 0x1c85f;
        sub_1d3e0(v26);
        v23 = v36;
        if ((v36) && (!v37)) break;
        v9 = &v7[6];
        *(unsigned long *)((long)v17 + -8) = 0x1c847;
        sub_28770(v7,&v11);
        v11 = (void *)((long)v11 + v35);
        v7 = v9;
      }
      v19 = 0;
      v21[(long)v4 * 6 + 1] = 1;
      v25 = (void *)0x2;
      v7 = v21;
      if ((void *)0x3 <= v24) {
label_1c8a0:
        v9 = &v7[6];
        v1 = *(char *)&v7[8];
        do {
          v6 = &v21[v19 * 6];
          if ((v1) && (*(char *)&v6[2])) {
            if (*(int *)((long)v7 + 0x44) == *(int *)((long)v6 + 0x14)) {
label_1c8f5:
              v19 += 1;
              v21[(long)((long)v4 * 6 + (long)v25)] = (long)v25 - v19;
              goto label_1c90a;
            }
          }
          else {
            v18 = v7[7];
            if (v18 == v6[1]) {
              v20 = (void *)*v6;
              v8 = (void *)*v9;
              *(unsigned long *)((long)v17 + -8) = 0x1c8f1;
              v3 = *(char *)((long)v17 + -8);
              if (!memcmp(v8,v20,v18,v3)) goto label_1c8f5;
            }
          }
          if (!v19) goto label_1cad0;
          v19 -= v21[(long)v4 * 6 + v19];
        } while( true );
      }
label_1c92a:
      v12[0] = 0;
      v20 = NULL;
      v13[0] = 0;
      *v22 = 0;
      v27 = 0;
      v29 = v10;
      v28 = 0;
      v11 = v10;
      v33 = 0;
      v34 = 0;
      do {
        while( true ) {
          *(unsigned long *)((long)v17 + -8) = 0x1c98c;
          sub_1d3e0(v26);
          v18 = v35;
          if (!v36) break;
          if (!v37) goto label_1c7c7;
          v7 = &v21[(long)v20 * 6];
          if (!*(char *)&v7[2]) goto label_1c9fb;
          if (*(int *)((long)v7 + 0x14) == v37) {
            v8 = v11;
            goto label_1c9b7;
          }
label_1ca7b:
          if (v20) { // branch-flip
            v19 = v21[(long)((long)v4 * 6 + (long)v20)];
            v20 = (void *)((long)v20 - v19);
            for (; v19; v19 = v19 + -1) {
              *(unsigned long *)((long)v17 + -8) = 0x1cab7;
              sub_1d3e0(v13);
              if ((v31) && (!v32)) goto label_1caca;
              v28 = 0;
              v29 = (void *)((long)v29 + v30);
            }
          }
          else {
            *(unsigned long *)((long)v17 + -8) = 0x1ca12;
            sub_1d3e0(v13);
            if ((v31) && (!v32)) {
label_1caca:
              *(unsigned long *)((long)v17 + -8) = 0x1cacf; // no-return
              abort(*(char *)((long)v17 + -8));
            }
            v28 = 0;
            v29 = (void *)((long)v29 + v30);
            v34 = 0;
            v11 = (void *)((long)v11 + v35);
          }
        }
        v7 = &v21[(long)v20 * 6];
label_1c9fb:
        if (v7[1] != v35) goto label_1ca7b;
        v8 = (void *)*v7;
        v25 = v11;
        *(unsigned long *)((long)v17 + -8) = 0x1ca69;
        v5 = memcmp(v8,v11,v35,*(char *)((long)v17 + -8));
        v8 = v25;
        if (v5) goto label_1ca7b;
label_1c9b7:
        v11 = (void *)((long)v8 + v18);
        v34 = 0;
        v20 = (void *)((long)v20 + 1);
      } while (v24 != v20);
      *v22 = (long)v29;
label_1c7c7:
      *(unsigned long *)((long)v17 + -8) = 0x1c9e8;
      sub_28710(v21);
      return v23;
    }
  }
  return '\0';
label_1cad0:
  v21[(long)((long)v4 * 6 + (long)v25)] = v25;
label_1c90a:
  v25 = (void *)((long)v25 + 1);
  v7 = v9;
  if (v24 == v25) goto label_1c92a;
  goto label_1c8a0;
}

// Function: sub_1caf0 @ 0x1caf0
char * sub_1caf0(char *a0,char *a1) // early-return, return-dupe x4
{
  char *v1;
  unsigned long v10; // rdx
  char *v11; // stack - 0x190
  char v12 [4]; // stack - 0x188
  char v13 [4]; // stack - 0x88
  unsigned long v14; // stack - 0xc8
  unsigned int v15; // stack - 0x108
  char v16 [4]; // stack - 0x148
  long v17;
  unsigned long v18;
  unsigned long v19; // stack - 0x1d8
  char v2;
  unsigned char v20; // stack - 0x1c9
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
  unsigned long v44; // stack - 0xc0
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
    sub_1d3e0(v12);
    if ((!v26) || (v27)) {
      v20 = 1;
      v16[0] = 0;
      v28 = 0;
      v29 = 0;
      v15 &= 0xffffff00;
      v35 = 0;
      v34 = 0;
      v19 = 0;
      v21 = 0;
      v9 = 0;
      v30 = a1;
      v37 = a0;
      do {
        v36 &= 0xffffff00;
        sub_1d3e0(&v15);
        if ((char)v39) { // branch-flip
          if (!v40)
            return NULL;
          v4 = 10 <= v19 & v20;
          if (v4) {
            if (v19 * 5 <= v9) { // branch-flip
label_1d070:
              v17 = v9 - v21;
              while ((v17 && ((sub_1d3e0(v16), !v32 || (v33))))) {
                v29 = 0;
                v30 = &v30[v31];
                v17 -= 1;
              }
              sub_1d3e0(v16);
              v20 = v32;
              if (v32) { // branch-flip
                if (!v33) {
                  v20 = sub_1c6e0(a0,a1,&v11);
                  if (v20)
                    return v11;
                }
              }
              else {
                v20 = 1;
              }
              v21 = v9;
              if (!(char)v39) goto label_1ce46;
            }
            else {
              v20 = v4;
            }
          }
          v10 = v9 + 1;
          if (!v26) goto label_1ce46;
          if (v40 == v27) goto label_1ce86;
label_1cdd8:
          v7 = v37;
          v18 = v38;
        }
        else {
          v4 = 10 <= v19 & v20;
          if (v4) { // branch-flip
            if (v19 * 5 <= v9) goto label_1d070;
            v20 = v4;
          }
          else {
          }
label_1ce46:
          v18 = v38;
          v7 = v37;
          v10 = v9 + 1;
          if (v38 == v25) {
            v6 = memcmp(v37,v24,v38);
            if (!v6) {
label_1ce86:
              v10 = v9 + 1;
              v14 = CONCAT44(v34,v15);
              v13[0] = 0;
              v48 = CONCAT44(v40,v39);
              v53 = 0;
              v47 = v38;
              v46 = &v37[v38];
              v45 = (undefined3)(v36 >> 8);
              v44 = (undefined5)v35;
              v52 = 0;
              v49 = v41;
              v50 = v42;
              v51 = v43;
              v54 = a1;
              sub_1d3e0(v13);
              if ((v56) && (!v57))
                abort(); // no-return
              v54 = &v54[v55];
              do {
                v10 += 1;
                v53 = 0;
                sub_1d3e0(v13);
                if ((v56) && (!v57))
                  return v37;
                sub_1d3e0(&v14);
                v18 = v55;
                v1 = v54;
                v9 = v47;
                v7 = v46;
                if ((char)v48) { // branch-flip
                  if (!v48._4_4_)
                    return NULL;
                  if (!v56) goto label_1cff3;
                  if (v48._4_4_ != v57) goto label_1cdd8;
                }
                else {
label_1cff3:
                  if ((v47 != v55) || (v6 = memcmp(v46,v54,v47), v6)) goto label_1cdd8;
                }
                v46 = &v7[v9];
                v44 = (undefined5)(unsigned int)v44;
                v54 = &v1[v18];
              } while( true );
            }
          }
        }
        v19 += 1;
        v37 = &v7[v18];
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
      v18 = 0;
      v10 = 0;
      v9 = 0;
      v5 = 1;
      v7 = a1;
      do {
        v18 += 1;
        v8 = v10 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v17 = v8 - v10;
            if (!a0[v17])
              return NULL;
            v8 += 1;
            if (a0[v17] != v3) break;
            v3 = a1[v8 - v10];
            if (!v3)
              return a0;
          }
        }
        v10 = v8;
        if (!*v1)
          return NULL;
        v4 = 10 <= v18 & (unsigned char)v5;
        if (v4) {
          if (v18 * 5 <= v10) { // branch-flip
            if (v7) {
              v7 = &v7[strnlen(v7,v10 - v9)];
              v9 = v10;
              if (*v7) goto label_1cc1d;
            }
            v5 = sub_1c540(v1,a1,strlen(a1),&v11);
            if ((char)v5)
              return v11;
            v7 = NULL;
          }
          else {
label_1cc1d:
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

// Function: sub_1d190 @ 0x1d190
int sub_1d190(unsigned char *a0,long a1,unsigned int a2)
{
  unsigned char *v1;
  unsigned char v2;
  unsigned short v3;
  int v4; // eax
  long v5; // rax
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // stack - 0x4c
  int v9; // r12d
  
  v1 = &a0[a1];
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v9 = 0;
    if (a0 < v1) {
      do {
        v2 = *a0;
        if ('`' <= (char)v2) { // branch-flip
          if (0x1e <= (unsigned char)(v2 + 0x9f)) goto label_1d214;
label_1d2c3:
          a0 = &a0[1];
          v9 += 1;
        }
        else {
          if ('@' < (char)v2) goto label_1d2c3;
          if ('#' < (char)v2) {
            if (0x1b <= (unsigned char)(v2 - 0x25)) goto label_1d214;
            goto label_1d2c3;
          }
          if ('\x1f' < (char)v2) goto label_1d2c3;
label_1d214:
          v7 = 0;
          do {
            v5 = sub_1c4b0(&v8,a0,(long)v1 - (long)a0,&v7);
            if (v5 == -1) {
              if (a2 & 1)
                return -1;
              goto label_1d2c3;
            }
            if (v5 == -2) {
              if (a2 & 1)
                return -1;
              v9 += 1;
              a0 = v1;
              break;
            }
            v6 = 1;
            if (v5)
              v6 = v5;
            v4 = wcwidth(v8);
            if (0 <= v4) { // branch-flip
              if (0x7fffffffU - v9 < v4)
                return 0x7fffffff;
              v9 += v4;
            }
            else {
              if (a2 & 2)
                return -1;
              v4 = iswcntrl(v8);
              if (!v4) {
                if (v9 == 0x7fffffff)
                  return 0x7fffffff;
                v9 += 1;
              }
            }
            a0 = &a0[v6];
          } while (!mbsinit(&v7));
        }
      } while (a0 < v1);
    }
  }
  else {
    v9 = 0;
    if (a0 < v1) {
      v9 = 0;
      do {
        v2 = *a0;
        a0 = &a0[1];
        v3 = *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)v2 * 2);
        if (v3 & 0x4000) { // branch-flip
label_1d349:
          if (v9 == 0x7fffffff)
            return 0x7fffffff;
          v9 += 1;
        }
        else {
          if (a2 & 2)
            return -1;
          if (!(v3 & 2)) goto label_1d349;
        }
      } while (v1 != a0);
    }
  }
  return v9;
}

// Function: sub_1d3b0 @ 0x1d3b0
void sub_1d3b0(char *a0,unsigned int a1)
{
  sub_1d190(a0,strlen(a0),a1); // tail-call
}

// Function: sub_1d3e0 @ 0x1d3e0
void sub_1d3e0(char *a0)
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
    if (sub_287d0((int)v1)) {
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
      goto label_1d422;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v4 = __ctype_get_mb_cur_max();
  v3 = *(unsigned long *)&a0[0x10];
  v5 = sub_1c4b0(&a0[0x24],v3,sub_25ca0(v3,v4),&a0[4]);
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
label_1d422:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_1d560 @ 0x1d560
void sub_1d560(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_1d570 @ 0x1d570
void sub_1d570(char *a0,char *a1)
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
  sub_28770(&a0[0x10],&a1[0x10]); // tail-call
}

// Function: sub_1d5b0 @ 0x1d5b0
unsigned int sub_1d5b0(unsigned int a0)
{
  return a0;
}

// Function: sub_1d5c0 @ 0x1d5c0
void sub_1d5c0(unsigned int a0,unsigned int a1)
{
  unsigned short *v1; // rax
  
  v1 = (unsigned short *)sub_26990(0x20);
  *v1 = 0x13d;
  *(unsigned int *)&v1[2] = 0xfff;
  *(unsigned int *)&v1[4] = a0;
  *(unsigned int *)&v1[6] = a1;
  *(char *)((long)v1 + 0x11) = 0;
}

// Function: sub_1d600 @ 0x1d600
void * sub_1d600(unsigned char *a0) // return-dupe
{
  unsigned char v1;
  unsigned long v10; // rdx
  long v11; // rdx
  char v12; // bl
  unsigned long v13;
  long v14;
  char *v15; // r8
  long v16;
  bool v17;
  char v2;
  unsigned int v3; // eax
  unsigned int v4; // eax
  void *v5; // rax
  unsigned int v6;
  unsigned char v7;
  unsigned char *v8;
  unsigned long v9; // rdx
  
  v7 = *a0;
  v6 = (unsigned int)(char)v7;
  if ((unsigned char)(v7 - 0x30) <= 7) {
    v4 = 0;
    v8 = a0;
    do {
      v4 = (v6 - 0x30) + v4 * 8;
      v8 = &v8[1];
      if (0x1000 <= v4)
        return NULL;
      v7 = *v8;
      v6 = (unsigned int)(char)v7;
    } while ((unsigned char)(v7 - 0x30) <= 7);
    if (!v7) {
      v4 = sub_1d5b0(v4);
      v6 = v4 & 0xc00 | 0x3ff;
      if (5 <= v11 - (long)a0)
        v6 = 0xfff;
      return (void *)sub_1d5c0(v4,v6); // tail-call
    }
    return NULL;
  }
  v14 = 1;
  v8 = a0;
  while (v7) {
    if ((unsigned char)v6 <= 0x3d)
      v14 = (v14 + 1) - (unsigned long)((0x2000280000000000U >> ((unsigned char)v6 & 0x3f) & 1) == 0);
    v7 = v8[1];
    v6 = (unsigned int)v7;
    v8 = &v8[1];
  }
  v5 = (void *)sub_26ab0(v14,0x10);
  v14 = 0;
label_1d66f:
  v7 = *a0;
  v4 = 0;
  v2 = v7 + 0x99;
  v17 = v7 == 0x67;
  v6 = 0;
  v1 = v7;
  if (v17) goto label_1d7bd;
  do {
    if (v17 || SBORROW1(v7,'g') != v2 < '\0') {
      if (v1 != 0x61) goto label_1d68e;
      v4 = 0xfff;
      a0 = &a0[1];
    }
    else if (v1 != 0x6f) { // branch-flip
      if (v1 != 0x75) {
        free(v5); // return-dupe
        return NULL;
      }
      v4 = v6 | 0x9c0;
      a0 = &a0[1];
    }
    else {
      v4 = v6 | 0x207;
      a0 = &a0[1];
    }
    while( true ) {
      v7 = *a0;
      v2 = v7 + 0x99;
      v17 = v2 == '\0';
      v6 = v4;
      v1 = v7;
      if (!v17) break;
label_1d7bd:
      v4 |= 0x438;
      a0 = &a0[1];
    }
  } while( true );
label_1d68e:
  if (((char)v1 <= 'a') && (((v1 & 0xef) == 0x2d || (v1 == 0x2b)))) {
    v7 = a0[1];
    v16 = v14 + 1;
    v15 = (char *)(v14 * 0x10 + (long)v5);
    v2 = v7 + 0x91;
    v17 = 0;
    v8 = a0;
    v13 = (unsigned long)v1;
    v14 = v16;
    if (v7 == 0x6f) goto label_1d735;
label_1d6d0:
    v12 = (char)v13;
    v9 = (unsigned long)(unsigned int)(int)(char)v7;
    a0 = &v8[1];
    if (v17 || SBORROW1(v7,'o') != v2 < '\0') {
      if ('8' <= (char)v7) { // branch-flip
        if (v7 != 0x67) goto label_1d7d0;
        v9 = (unsigned long)v8[2];
        a0 = &v8[2];
        v4 = 0x38;
        v12 = 3;
      }
      else {
        if ('0' <= (char)v7) {
          v4 = 0;
          goto label_1d7fa;
        }
        v12 = 1;
        v4 = 0;
      }
    }
    else {
      if (v7 != 0x75) {
label_1d7d0:
        v12 = 1;
        v4 = 0;
        do {
          switch((int)v9 - 0x58U & 0xff) {
            case 0:
              v12 = 2;
              break;
            default:
              goto label_1d6eb;
            case 0x1a:
              v4 |= 0x124;
              break;
            case 0x1b:
              v4 |= 0xc00;
              break;
            case 0x1c:
              v4 |= 0x200;
              break;
            case 0x1f:
              v4 |= 0x92;
              break;
            case 0x20:
              v4 |= 0x49;
            
          }
          a0 = &a0[1];
          v9 = (unsigned long)*a0;
        } while( true );
      }
      v9 = (unsigned long)v8[2];
      a0 = &v8[2];
      v4 = 0x1c0;
      v12 = 3;
    }
label_1d6eb:
    do {
      v7 = (unsigned char)v9;
      *v15 = (char)v13;
      v15[1] = v12;
      *(unsigned int *)&v15[4] = v6;
      *(unsigned int *)&v15[8] = v4;
      if (v6)
        v4 &= v6;
label_1d707:
      *(unsigned int *)&v15[0xc] = v4;
      v16 = v14 + 1;
      v15 = &v15[0x10];
      if (((v7 & 0xef) != 0x2d) && (v7 != 0x2b)) {
        if (v7 == 0x2c) {
          a0 = &a0[1];
          goto label_1d66f;
        }
        if (!v7) {
          *(char *)((long)v5 + v14 * 0x10 + 1) = 0;
          return v5;
        }
        break;
      }
      v7 = a0[1];
      v2 = v7 + 0x91;
      v17 = v2 == '\0';
      v8 = a0;
      v13 = v9;
      v14 = v16;
      if (!v17) goto label_1d6d0;
label_1d735:
      v9 = (unsigned long)a0[2];
      a0 = &a0[2];
      v4 = 7;
      v12 = 3;
      v14 = v16;
    } while( true );
  }
  free(v5);
  return NULL;
  while( true ) {
    v7 = *a0;
    v9 = (unsigned long)(unsigned int)(int)(char)v7;
    if (8 <= (unsigned char)(v7 - 0x30)) break;
label_1d7fa:
    v4 = (int)v9 + -0x30 + v4 * 8;
    a0 = &a0[1];
    if (0x1000 <= v4) {
      free(v5);
      return NULL;
    }
  }
  if ((!v6) && ((!v7 || (v7 == 0x2c)))) {
    v3 = sub_1d5b0(v4);
    *v15 = v12;
    v6 = 0xfff;
    *(unsigned int *)&v15[8] = v3;
    v7 = (unsigned char)v10;
    v9 = v10 & 0xffffffff;
    v4 = 0xfff;
    v15[1] = 1;
    *(unsigned int *)&v15[4] = 0xfff;
    goto label_1d707;
  }
label_1d690:
  free(v5);
  return NULL;
}

// Function: sub_1d970 @ 0x1d970
unsigned long sub_1d970(char *a0) // early-return
{
  char v1 [24];
  unsigned int v2; // stack - 0x90
  
  if (stat(a0,v1))
    return 0;
  return sub_1d5c0(v2,0xfff);
}

// Function: sub_1d9d0 @ 0x1d9d0
unsigned int sub_1d9d0(unsigned int a0,char a1,unsigned int a2,char *a3,unsigned int *a4)
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
      if (v5 != '\x02') goto label_1da51;
label_1db05:
      v2 |= 0x49;
      v4 = ~v3;
    }
    else if (v5 != '\x02') { // branch-flip
      v4 = 0xffffffff;
      v3 = 0;
label_1da51:
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
        goto label_1db05;
      }
      v4 = 0xffffffff;
      v3 = 0;
    }
    v5 = *a3;
    if (v1) { // branch-flip
      v2 = v2 & v4 & v1;
      if (v5 == '-') {
label_1db18:
        v7 |= v2;
        a0 &= ~v2;
        goto label_1da20;
      }
      if (v5 != '=') goto label_1da16;
      v3 |= ~v1;
      v4 = ~v3;
label_1daaf:
      v5 = a3[0x11];
      a0 = v3 & a0 | v2;
      v7 |= v4 & 0xfff;
    }
    else {
      v2 = v2 & v4 & ~a2;
      if (v5 == '-') goto label_1db18;
      if (v5 == '=') goto label_1daaf;
label_1da16:
      if (v5 == '+') {
        v7 |= v2;
        a0 |= v2;
      }
label_1da20:
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

// Function: sub_1db70 @ 0x1db70
void sub_1db70(char *a0) // return-dupe
{
  char *v1; // rax
  
  v1 = strchr(a0,0x20);
  if (!v1)
    return;
  *v1 = '\0';
}

// Function: sub_1db90 @ 0x1db90
void sub_1db90(char *a0)
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

// Function: sub_1dc30 @ 0x1dc30
unsigned long sub_1dc30(void)
{
  return 0xffffffffffffffff;
}

// Function: sub_1dc40 @ 0x1dc40
void sub_1dc40(unsigned long *a0)
{
  free((void *)*a0);
  free((void *)a0[1]);
  free((void *)a0[2]);
  if (!(*(unsigned char *)&a0[5] & 4)) {
    free(a0); // tail-call
    return;
  }
  free((void *)a0[3]);
  free(a0); // tail-call
}

// Function: sub_1dc90 @ 0x1dc90
long sub_1dc90(void)
{
  int v1;
  unsigned long v10; // rax
  long v11; // stack - 0x58
  unsigned long v12; // stack - 0x48
  char *v13; // stack - 0x50
  unsigned int v14; // stack - 0x60
  unsigned int v15; // stack - 0x64
  int v16; // stack - 0x5c
  char *v17; // rax
  bool v18; // dl
  bool v19; // r12b
  void *v2; // rax
  long *v20; // stack - 0x88
  long v3;
  long v4;
  char *v5;
  unsigned long *v6; // rax
  unsigned long v7;
  int *v8; // rax
  unsigned long *v9; // rax
  
  v2 = fopen("/proc/self/mountinfo","re");
  if (v2) { // branch-flip
    v20 = &v11;
    v13 = NULL;
    v12 = 0;
    while (getline(&v13,&v12,v2) != -1) {
      v1 = __isoc99_sscanf(v13,"%*u %*u %u:%u %n",&v15,&v14,&v16);
      if ((unsigned int)(v1 - 2U) <= 1) {
        v17 = &v13[v16];
        v3 = sub_1db70(v17);
        if (v3) {
          v3 += 1;
          v4 = sub_1db70(v3);
          if ((v4) && (v5 = strstr((char *)(v4 + 1)," - "), v5)) {
            v5 = &v5[3];
            v4 = sub_1db70(v5);
            if (v4) {
              v4 += 1;
              if (sub_1db70(v4)) {
                sub_1db90(v4);
                sub_1db90(v3);
                sub_1db90(v17);
                sub_1db90(v5);
                v6 = (unsigned long *)sub_26990(0x38);
                *v6 = sub_26d30(v4);
                v6[1] = sub_26d30(v3);
                v6[2] = sub_26d30(v17);
                v17 = (char *)sub_26d30(v5);
                *(unsigned char *)&v6[5] = *(unsigned char *)&v6[5] | 4;
                v6[3] = v17;
                v7 = gnu_dev_makedev(v15,v14);
                v6[4] = v7;
                v18 = 1;
                if (strcmp(v17,"autofs")) {
                  v18 = 1;
                  if (strcmp(v17,"proc")) {
                    v18 = 1;
                    if (strcmp(v17,"subfs")) {
                      v18 = 1;
                      if (strcmp(v17,"debugfs")) {
                        v18 = 1;
                        if (strcmp(v17,"devpts")) {
                          v18 = 1;
                          if (strcmp(v17,"fusectl")) {
                            v18 = 1;
                            if (strcmp(v17,"fuse.portal")) {
                              v18 = 1;
                              if (strcmp(v17,"mqueue")) {
                                v18 = 1;
                                if (strcmp(v17,"rpc_pipefs")) {
                                  v18 = 1;
                                  if (strcmp(v17,"sysfs")) {
                                    v18 = 1;
                                    if (strcmp(v17,"devfs")) {
                                      v18 = 1;
                                      if (strcmp(v17,"kernfs")) {
                                        v18 = 1;
                                        if (strcmp(v17,"ignore"))
                                          v18 = strcmp(v17,"none") == 0;
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
                *(unsigned char *)&v6[5] = *(unsigned char *)&v6[5] & 0xfe | v18;
                v5 = (char *)*v6;
                v18 = 1;
                if (!strchr(v5,0x3a)) {
                  if ((*v5 != '/') || (v5[1] != '/')) {
label_1e096:
                    v18 = 1;
                    if (strcmp(v17,"acfs")) {
                      v18 = 1;
                      if (strcmp(v17,"afs")) {
                        v18 = 1;
                        if (strcmp(v17,"coda")) {
                          v18 = 1;
                          if (strcmp(v17,"auristorfs")) {
                            v18 = 1;
                            if (strcmp(v17,"fhgfs")) {
                              v18 = 1;
                              if (strcmp(v17,"gpfs")) {
                                v18 = 1;
                                if (strcmp(v17,"ibrix")) {
                                  v18 = 1;
                                  if (strcmp(v17,"ocfs2")) {
                                    v18 = 1;
                                    if (strcmp(v17,"vxfs"))
                                      v18 = strcmp("-hosts",v5) == 0;
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
                    v18 = 1;
                    if (strcmp(v17,"smbfs")) {
                      v18 = 1;
                      if (strcmp(v17,"smb3")) {
                        v18 = 1;
                        if (strcmp(v17,"cifs")) goto label_1e096;
                      }
                    }
                  }
                }
                *(unsigned char *)&v6[5] = *(unsigned char *)&v6[5] & 0xfd | v18 * '\x02';
                *v20 = (long)v6;
                v20 = &v6[6];
              }
            }
          }
        }
      }
    }
    free(v13);
    if (ferror_unlocked(v2)) {
      v8 = __errno_location();
      v1 = *v8;
      sub_17850(v2);
      *v8 = v1;
      goto label_1e1cf;
    }
    if (sub_17850(v2) != -1) goto label_1dedb;
  }
  else {
    v3 = setmntent("/etc/mtab","r");
    if (!v3)
      return 0;
    v20 = &v11;
    while (v6 = (unsigned long *)getmntent(v3), v6) {
      v18 = 1;
      v4 = hasmntopt(v6,"bind");
      v9 = (unsigned long *)sub_26990(0x38);
      v10 = sub_26d30(*v6);
      v7 = v6[1];
      *v9 = v10;
      v10 = sub_26d30(v7);
      v9[2] = 0;
      v7 = v6[2];
      v9[1] = v10;
      v17 = (char *)sub_26d30(v7);
      *(unsigned char *)&v9[5] = *(unsigned char *)&v9[5] | 4;
      v9[3] = v17;
      if ((((((strcmp(v17,"autofs")) && (strcmp(v17,"proc"))) && (strcmp(v17,"subfs"))) && ((strcmp(v17,"debugfs") && (strcmp(v17,"devpts"))))) && (strcmp(v17,"fusectl"))) && ((((strcmp(v17,"fuse.portal") && (strcmp(v17,"mqueue"))) && ((strcmp(v17,"rpc_pipefs") && (((strcmp(v17,"sysfs") && (strcmp(v17,"devfs"))) && (strcmp(v17,"kernfs"))))))) && (strcmp(v17,"ignore")))))
        v18 = !v4 && !strcmp(v17,"none");
      v5 = (char *)*v9;
      v19 = 1;
      *(unsigned char *)&v9[5] = *(unsigned char *)&v9[5] & 0xfe | v18;
      if ((!strchr(v5,0x3a)) && (((*v5 != '/' || (v5[1] != '/')) || ((strcmp(v17,"smbfs") && ((strcmp(v17,"smb3") && (strcmp(v17,"cifs"))))))))) {
        v19 = 1;
        if ((strcmp(v17,"acfs")) && ((((strcmp(v17,"afs") && (strcmp(v17,"coda"))) && (strcmp(v17,"auristorfs"))) && (((strcmp(v17,"fhgfs") && (strcmp(v17,"gpfs"))) && ((strcmp(v17,"ibrix") && ((strcmp(v17,"ocfs2") && (strcmp(v17,"vxfs")))))))))))
          v19 = strcmp("-hosts",v5) == 0;
      }
      *(unsigned char *)&v9[5] = *(unsigned char *)&v9[5] & 0xfd | v19 * '\x02';
      v9[4] = sub_1dc30();
      *v20 = (long)v9;
      v20 = &v9[6];
    }
    if (endmntent(v3)) {
label_1dedb:
      *v20 = 0;
      return v11;
    }
  }
  v8 = __errno_location();
  v1 = *v8;
label_1e1cf:
  *v20 = 0;
  while (v11) {
    v3 = *(long *)(v11 + 0x30);
    sub_1dc40(v11);
    v11 = v3;
  }
  *v8 = v1;
  return 0;
}

// Function: sub_1e610 @ 0x1e610
void sub_1e610(int a0)
{
  error(dat_3e4b8,a0,dcgettext(NULL,"unable to record current working directory",5));
  abort(); // no-return
}

// Function: sub_1e650 @ 0x1e650
void sub_1e650(int a0)
{
  error(dat_3e4b8,a0,dcgettext(NULL,"failed to return to initial working directory",5));
  abort(); // no-return
}

// Function: sub_1e690 @ 0x1e690
void sub_1e690(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_26280(openat(a0,a1,a2,v1));
}

// Function: sub_1e710 @ 0x1e710
void * sub_1e710(unsigned long a0,unsigned long a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  void *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_1e690(a0,a1,a2 | 0x90900);
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

// Function: sub_1e770 @ 0x1e770
unsigned int sub_1e770(unsigned int a0)
{
  return a0;
}

// Function: sub_1e780 @ 0x1e780
long sub_1e780(long a0)
{
  long v1; // rax
  long v2;
  unsigned long v3; // stack + 0x8
  long v4; // stack + 0x10
  long v5; // stack + 0x18
  
  if (((*(long *)(a0 + 0xa8)) && (!*(long *)(a0 + 0x30))) && (!*(char *)(a0 + 0xa1))) {
    if ((*(long *)(a0 + 200)) || (3 <= v5)) {
      v1 = v5;
      *(char *)(a0 + 0xd8) = 1;
      *(long *)(a0 + 0x30) = v5;
      *(unsigned long *)(a0 + 0x20) = v3;
      *(long *)(a0 + 0x28) = v4;
      return v1;
    }
    *(unsigned long *)(a0 + 200) = 1;
  }
  else {
    if (5 <= v5) {
      v1 = *(long *)(a0 + 0xa8) + 1;
      *(long *)(a0 + 0xa8) = v1;
      *(long *)(a0 + 0x30) = v5 + -4;
      *(long *)(a0 + 0x40) = v4 % 100;
      v1 = v4 * 0x346dc5d63886594b;
      *(long *)(a0 + 0x38) = (v4 / 100) % 100;
      *(long *)(a0 + 0x28) = v4 / 10000;
      return v1;
    }
    *(long *)(a0 + 200) = *(long *)(a0 + 200) + 1;
    if (3 <= v5) {
      v2 = v4 / 100;
      v1 = v4 % 100;
      goto label_1e8c4;
    }
  }
  v1 = 0;
  v2 = v4;
label_1e8c4:
  *(long *)(a0 + 0x48) = v2;
  *(long *)(a0 + 0x50) = v1;
  *(unsigned long *)(a0 + 0x58) = 0;
  *(unsigned long *)(a0 + 0x60) = 0;
  *(unsigned int *)(a0 + 0x1c) = 2;
  return v1;
}

// Function: sub_1e900 @ 0x1e900
void sub_1e900(long a0,unsigned long a1,unsigned long a2,unsigned long a3,int a4)
{
  *(unsigned long *)(a0 + 0x48) = a1;
  *(unsigned long *)(a0 + 0x50) = a2;
  *(unsigned long *)(a0 + 0x58) = a3;
  *(long *)(a0 + 0x60) = (long)a4;
}

// Function: sub_1e920 @ 0x1e920
unsigned long sub_1e920(unsigned long a0,int a1) // return-dupe
{
  if (a1) { // branch-flip
    if (a1 == 1) {
      if (a0 - 1 <= 10)
        return (unsigned long)((int)a0 + 0xc);
      if (a0 == 0xc)
        return 0xc;
      return 0xffffffff;
    }
    if (0x18 <= a0)
      return 0xffffffff;
  }
  else if (0xb <= a0 - 1)
    return (unsigned long)-(unsigned int)(a0 != 0xc);
  return a0 & 0xffffffff;
}

// Function: sub_1e990 @ 0x1e990
unsigned int sub_1e990(unsigned int *a0,unsigned int *a1) // early-return
{
  unsigned int v1; // eax
  
  if ((int)a1[6] <= -1)
    return 0;
  v1 = *a0 ^ *a1 | a0[1] ^ a1[1] | a0[2] ^ a1[2] | a0[3] ^ a1[3] | a0[4] ^ a1[4] | a0[5] ^ a1[5];
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_1e9d0 @ 0x1e9d0
long sub_1e9d0(long a0,char *a1)
{
  unsigned long *v1;
  char *v2;
  long v3;
  
  v3 = 0x3c860;
  v2 = "GMT";
  do {
    if (!strcmp(a1,v2))
      return v3;
    v2 = *(char **)(v3 + 0x10);
    v3 += 0x10;
  } while (v2);
  v3 = a0 + 0xe0;
  v2 = *(char **)(a0 + 0xe0);
  while (v2) {
    if (!strcmp(a1,v2))
      return v3;
    v1 = (unsigned long *)(v3 + 0x10);
    v3 += 0x10;
    v2 = (char *)*v1;
  }
  v3 = 0x3c560;
  v2 = "WET";
  do {
    if (!strcmp(a1,v2))
      return v3;
    v2 = *(char **)(v3 + 0x10);
    v3 += 0x10;
  } while (v2);
  return 0;
}

// Function: sub_1eaa0 @ 0x1eaa0
long sub_1eaa0(unsigned long a0,char *a1)
{
  unsigned long *v1;
  bool v10;
  char v2; // al
  char v3;
  int v4;
  unsigned long v5; // rax
  char v6;
  char *v7;
  char *v8;
  long v9; // rax
  
  v3 = *a1;
  v7 = a1;
  while (v3) {
    v2 = sub_1e770((int)v3);
    v8 = &v7[1];
    *v7 = sub_27fd0(v2);
    v3 = *v8;
    v7 = v8;
  }
  v9 = 0x3cc60;
  v7 = "AM";
  do {
    if (!strcmp(a1,v7))
      return v9;
    v7 = *(char **)(v9 + 0x10);
    v9 += 0x10;
  } while (v7);
  v10 = 1;
  v5 = strlen(a1);
  if ((v5 != 3) && (v10 = 0, v5 == 4))
    v10 = a1[3] == '.';
  v9 = 0x3cac0;
  v7 = "JANUARY";
  do {
    if (v10)
      v4 = strncmp(a1,v7,3);
    else {
      v4 = strcmp(a1,v7);
    }
    if (!v4)
      return v9;
    v7 = *(char **)(v9 + 0x10);
    v9 += 0x10;
  } while (v7);
  v9 = sub_1e9d0(a0,a1);
  if (!v9) {
    if (strcmp(a1,"DST")) { // branch-flip
      v9 = 0x3ca00;
      v7 = "YEAR";
      do {
        if (!strcmp(a1,v7))
          return v9;
        v7 = *(char **)(v9 + 0x10);
        v9 += 0x10;
      } while (v7);
      v7 = &a1[v5 - 1];
      if (*v7 == 'S') {
        *v7 = '\0';
        v9 = 0x3ca00;
        v8 = "YEAR";
        do {
          if (!strcmp(a1,v8))
            return v9;
          v8 = *(char **)(v9 + 0x10);
          v9 += 0x10;
        } while (v8);
        *v7 = 'S';
      }
      v9 = 0x3c8a0;
      v7 = "TOMORROW";
      do {
        if (!strcmp(a1,v7))
          return v9;
        v7 = *(char **)(v9 + 0x10);
        v9 += 0x10;
      } while (v7);
      v3 = *a1;
      if (v5 != 1) { // branch-flip
label_1ec5d:
        v10 = 0;
        v7 = a1;
        v8 = a1;
        if (v3) {
          do {
            if (v3 != '.') // branch-flip
              v7 = &v7[1];
            else {
              v10 = 1;
            }
            v3 = v8[1];
            v8 = &v8[1];
            *v7 = v3;
          } while (v3);
          if (v10)
            return sub_1e9d0(a0,a1); // tail-call
        }
        v9 = 0;
      }
      else {
        v9 = 0x3c3c0;
        v6 = 'A';
        while (v3 != v6) {
          v1 = (unsigned long *)(v9 + 0x10);
          v9 += 0x10;
          if (!(char *)*v1) goto label_1ec5d;
          v6 = *(char *)*v1;
        }
      }
    }
    else {
      v9 = 0x3cc50;
    }
  }
  return v9;
}

// Function: sub_1ed30 @ 0x1ed30
char * sub_1ed30(int a0,char *a1)
{
  char v1;
  int v2; // eax
  int v3; // eax
  char *v4; // rcx
  char *v5;
  
  v2 = __sprintf_chk(a1,1,0xffffffffffffffff,"%c%02d",(unsigned long)((a0 >> 0x1f & 2U) + 0x2b),(unsigned long)(unsigned int)abs(a0 / 0xe10));
  v3 = abs(a0 % 0xe10);
  if (v3) {
    v4 = &a1[v2];
    *v4 = ':';
    v1 = (char)((v3 / 0x3c) / 10);
    v4[1] = v1 + '0';
    v5 = &v4[3];
    v2 = v3 % 0x3c;
    v4[2] = (char)(v3 / 0x3c) + v1 * '\xf6' + '0';
    if (v2) {
      v4[3] = ':';
      v1 = (char)(v2 / 10);
      v4[4] = v1 + '0';
      v5 = &v4[6];
      v4[5] = (char)v2 + v1 * '\xf6' + '0';
    }
    *v5 = '\0';
  }
  return a1;
}

// Function: sub_1ee60 @ 0x1ee60
char * sub_1ee60(int a0,char *a1)
{
  unsigned int v1; // eax
  
  v1 = abs(a0 % 100);
  __sprintf_chk(a1,1,0xffffffffffffffff,&"-%02d%02d"[-0x76c <= a0],(unsigned long)(unsigned int)abs(a0 / 100 + 0x13),(unsigned long)v1);
  return a1;
}

// Function: sub_1eee0 @ 0x1eee0
unsigned int sub_1eee0(unsigned int a0,long a1,unsigned long a2)
{
  if (!a1)
    return a0;
  __fprintf_chk(stderr,1,&" %+ld %s"[(a0 ^ 1) & 0xff],a1,a2);
  return 1;
}

// Function: sub_1ef30 @ 0x1ef30
void sub_1ef30(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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

// Function: sub_1f010 @ 0x1f010
long sub_1f010(unsigned long a0,long a1,long a2)
{
  unsigned int v1;
  int v2; // eax
  char v3 [40];
  
  v2 = sub_2a2c0(a2,100,"(Y-M-D) %Y-%m-%d %H:%M:%S",a0,0,0);
  if (((a1) && (v2 <= 99)) && (*(long *)(a1 + 0xd0))) {
    v1 = *(unsigned int *)(a1 + 0x18);
    __snprintf_chk((char *)(v2 + a2),(long)(100 - v2),1,0xffffffffffffffff," TZ=%s",sub_1ed30(v1,v3));
  }
  return a2;
}

// Function: sub_1f0c0 @ 0x1f0c0
char * sub_1f0c0(long a0,char *a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned long v3;
  unsigned long v4; // rsi
  char *v5;
  
  if (*(char *)(a0 + 0xdf)) { // branch-flip
    v3 = *(long *)(a0 + 8) + 1;
    if (0xe <= v3) // branch-flip
      v2 = __snprintf_chk(a1,100,1,0xffffffffffffffff,"%ld");
    else {
      v2 = snprintf(a1,100,"%s",&"last"[v3 * 0xb]);
    }
    v1 = *(unsigned int *)(a0 + 0x10);
    if ((v1 <= 6) && (v2 <= 99)) {
      v3 = (unsigned long)(v2 == 0);
      v4 = (unsigned long)(int)(100 - v2);
      v5 = &a1[(int)v2];
      __snprintf_chk(v5,v4,1,0xffffffffffffffff,&" %s"[v3],&"Sun"[(long)(int)v1 * 4]); // return-dupe
      return a1;
    }
  }
  else {
    *a1 = '\0';
    v1 = *(unsigned int *)(a0 + 0x10);
    if (v1 <= 6) {
      v3 = 1;
      v4 = 100;
      v5 = a1;
      __snprintf_chk(v5,v4,1,0xffffffffffffffff,&" %s"[v3],&"Sun"[(long)(int)v1 * 4]);
      return a1;
    }
  }
  return a1;
}

// Function: sub_1f1d0 @ 0x1f1d0
unsigned long sub_1f1d0(void)
{
  return 0;
}

// Function: sub_1f1e0 @ 0x1f1e0
void sub_1f1e0(unsigned long a0,long a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned int v4;
  char v5 [104];
  char *v6; // r8
  unsigned long v7;
  
  v2 = sub_1f1d0();
  if (!(char)v2)
    return;
  sub_1ef30(dcgettext(NULL,"parsed %s part: ",5),a0);
  if ((*(long *)(a1 + 0xa8)) && (!*(char *)(a1 + 0xd9))) {
    __fprintf_chk(stderr,1,"(Y-M-D) %04ld-%02ld-%02ld",*(unsigned long *)(a1 + 0x28),*(unsigned long *)(a1 + 0x38),*(unsigned long *)(a1 + 0x40));
    *(char *)(a1 + 0xd9) = 1;
    v4 = v2;
    if (*(char *)(a1 + 0xd8) != *(char *)(a1 + 0xde)) {
      fputc(0x20,stderr);
label_1f280:
      v7 = *(unsigned long *)(a1 + 0x28);
      __fprintf_chk(stderr,1,dcgettext(NULL,"year: %04ld",5),v7);
      *(char *)(a1 + 0xde) = *(char *)(a1 + 0xd8);
      v4 = v2;
    }
  }
  else {
    v4 = 0;
    if (*(char *)(a1 + 0xd8) != *(char *)(a1 + 0xde)) goto label_1f280;
  }
  if ((*(long *)(a1 + 200)) && (!*(char *)(a1 + 0xdc))) {
    __fprintf_chk(stderr,1,&" %02ld:%02ld:%02ld"[(v4 ^ 1) & 0xff],*(unsigned long *)(a1 + 0x48),*(unsigned long *)(a1 + 0x50),*(unsigned long *)(a1 + 0x58));
    if (*(long *)(a1 + 0x60))
      __fprintf_chk(stderr,1,".%09d");
    if (*(int *)(a1 + 0x1c) == 1)
      fputs("pm",stderr);
    *(char *)(a1 + 0xdc) = 1;
    v4 = v2;
    if ((*(long *)(a1 + 0xb0)) && (v4 = (unsigned int)*(unsigned char *)(a1 + 0xda), !*(unsigned char *)(a1 + 0xda))) {
label_1f51c:
      fputc(0x20,stderr);
label_1f2f1:
      v4 = *(unsigned int *)(a1 + 0x10);
      v7 = *(unsigned long *)(a1 + 8);
      v3 = sub_1f0c0(a1,v5);
      __fprintf_chk(stderr,1,dcgettext(NULL,"%s (day ordinal=%ld number=%d)",5),v3,v7,(unsigned long)v4);
      *(char *)(a1 + 0xda) = 1;
      v4 = v2;
    }
  }
  else if ((*(long *)(a1 + 0xb0)) && (!*(char *)(a1 + 0xda))) {
    if ((char)v4) goto label_1f51c;
    goto label_1f2f1;
  }
  if ((*(long *)(a1 + 0xb8)) && (!*(char *)(a1 + 0xdb))) {
    v6 = " DST";
    if (!*(long *)(a1 + 0xc0))
      v6 = "";
    __fprintf_chk(stderr,1,&" isdst=%d%s"[(v4 ^ 1) & 0xff],(unsigned long)*(unsigned int *)(a1 + 0x14),v6);
    *(char *)(a1 + 0xdb) = 1;
    if ((*(long *)(a1 + 0xd0)) && (!*(char *)(a1 + 0xdd))) {
      v6 = " UTC%s";
label_1f3ec:
      v1 = *(unsigned int *)(a1 + 0x18);
      __fprintf_chk(stderr,1,v6,sub_1ed30(v1,v5));
      *(char *)(a1 + 0xdd) = 1;
    }
    if (!*(char *)(a1 + 0xa0)) {
      fputc(10,stderr); // return-dupe, tail-call
      return;
    }
    v7 = *(unsigned long *)(a1 + 0x58);
label_1f424:
    fputc(0x20,stderr);
  }
  else {
    if ((*(long *)(a1 + 0xd0)) && (!*(char *)(a1 + 0xdd))) {
      v6 = &" UTC%s"[(unsigned char)((unsigned char)v4 ^ 1)];
      goto label_1f3ec;
    }
    if (!*(char *)(a1 + 0xa0)) {
      fputc(10,stderr);
      return;
    }
    v7 = *(unsigned long *)(a1 + 0x58);
    if ((unsigned char)v4) goto label_1f424;
  }
  __fprintf_chk(stderr,1,dcgettext(NULL,"number of seconds: %ld",5),v7);
  fputc(10,stderr);
  return;
}

// Function: sub_1f5d0 @ 0x1f5d0
unsigned int sub_1f5d0(unsigned char *a0,long *a1) // return-dupe
{
  unsigned char v1; // al
  unsigned char *v10;
  unsigned char v11;
  unsigned int v12;
  int v13;
  unsigned char *v14;
  int v15; // r15d
  unsigned long v2; // rax
  long v3; // rax
  unsigned char v4;
  long v5;
  unsigned char v6 [24];
  int v7; // edi
  int v8;
  int v9; // r12d
  
  v14 = (unsigned char *)*a1;
  v11 = *v14;
  v2 = sub_27f50(v11);
  v4 = v11;
label_1f608:
  do {
    while (v10 = &v14[1], (char)v2) {
      *a1 = (long)v10;
      v11 = *v10;
      v2 = sub_27f50(v11);
      v14 = v10;
      v4 = v11;
    }
    v1 = v4 - 0x2b & 0xfd;
    if (sub_27ec0(v11)) { // branch-flip
      if (v1) {
        v15 = 0;
        break;
      }
    }
    else if (v1) {
      if (sub_27e50(v11)) {
        v10 = v6;
        do {
          v14 = &v14[1];
          if (v10 < &v6[0x13]) {
            *v10 = v4;
            v10 = &v10[1];
          }
          *a1 = (long)v14;
          v4 = *v14;
        } while ((sub_27e50(v4)) || (v4 == 0x2e));
        *v10 = 0;
        v5 = sub_1eaa0(a1,v6);
        if (!v5) {
          if (sub_1f1d0()) {
            sub_1ef30(dcgettext(NULL,"error: unknown word \'%s\'\n",5),v6);
            return 0x3f;
          }
          return 0x3f;
        }
        v12 = *(unsigned int *)(v5 + 8);
        *(long *)a0 = (long)*(int *)(v5 + 0xc);
        return v12;
      }
      if (v4 != 0x28) {
        *a1 = (long)&v14[1];
        v11 = *v14;
        return (unsigned int)(unsigned char)sub_1e770((int)(char)v11);
      }
      v5 = 0;
      do {
        while( true ) {
          v10 = v14;
          v14 = &v10[1];
          *a1 = (long)v14;
          v11 = *v10;
          if (!v11)
            return 0;
          if (v11 != 0x28) break;
          v5 += 1;
          if (!v5) goto label_1f771;
        }
        v5 -= (unsigned long)(v11 == 0x29);
      } while (v5);
label_1f771:
      v11 = v10[1];
      v2 = sub_27f50(v11);
      v4 = v11;
      goto label_1f608;
    }
    v15 = (-(unsigned int)(v4 != 0x2d) & 2) - 1;
    do {
      v14 = &v14[1];
      *a1 = (long)v14;
      v11 = *v14;
    } while (sub_27f50(v11));
    v2 = sub_27ec0(v11);
    v4 = v11;
  } while (!(char)v2);
  v5 = 0;
  v12 = (unsigned int)v4;
  while( true ) {
    v13 = v12 - 0x30;
    if (v15 == -1)
      v13 = 0x30 - v12;
    v3 = v5 + v13;
    if (SCARRY8(v5,(long)v13))
      return 0x3f;
    v11 = v14[1];
    v12 = (unsigned int)v11;
    v10 = &v14[1];
    if (!sub_27ec0(v12)) break;
    v5 = v3 * 10;
    if (SEXT816(v5) != SEXT816(v3) * SEXT816(10))
      return 0x3f;
    v14 = v10;
  }
  if ((v11 & 0xfd) == 0x2c) {
    v11 = v14[2];
    if (sub_27ec0((int)(char)v11)) {
      v7 = (int)(char)v14[3];
      v14 = &v14[3];
      v13 = (char)v11 + -0x30;
      v9 = 8;
      v2 = sub_27ec0(v7);
      v8 = v7;
      do {
        v13 *= 10;
        if ((char)v2) {
          v13 = v13 + -0x30 + v8;
          v7 = (int)(char)v14[1];
          v14 = &v14[1];
          v2 = sub_27ec0(v7);
          v8 = v7;
        }
        v9 -= 1;
      } while (v9);
      if (v15 != -1) { // branch-flip
label_1f87f:
        while ((char)v2) {
          v10 = &v14[1];
          v14 = &v14[1];
          v2 = sub_27ec0((int)(char)*v10);
        }
        if ((0 <= v15) || (!v13)) {
          *(long *)a0 = v3;
          *(long *)&a0[8] = (long)v13;
          *a1 = (long)v14;
          return 0x115 - (unsigned int)(v15 != 0);
        }
      }
      else {
        while ((char)v2) {
          if ((char)v7 != '0') {
            v13 += 1;
            goto label_1f87f;
          }
          v7 = (int)(char)v14[1];
          v14 = &v14[1];
          v2 = sub_27ec0(v7);
        }
        if (!v13) {
          *(long *)a0 = v3;
          a0[8] = 0;
          a0[9] = 0;
          a0[10] = 0;
          a0[0xb] = 0;
          a0[0xc] = 0;
          a0[0xd] = 0;
          a0[0xe] = 0;
          a0[0xf] = 0;
          *a1 = (long)v14;
          return 0x114;
        }
      }
      if (SCARRY8(v3,-1))
        return 0x3f;
      *(long *)a0 = v3 + -1;
      *(long *)&a0[8] = (long)(1000000000 - v13);
      *a1 = (long)v14;
      return 0x114;
    }
  }
  *a0 = (unsigned char)((unsigned int)v15 >> 0x1f);
  v5 = *a1;
  *(long *)&a0[8] = v3;
  *(long *)&a0[0x10] = (long)v10 - v5;
  *a1 = (long)v10;
  return (v15 == 0) + 0x112;
}

// Function: sub_1fa50 @ 0x1fa50
void sub_1fa50(int *a0,int *a1,unsigned long a2,char a3)
{
  int v1;
  int v10;
  bool v11;
  char v12; // al
  int v13;
  unsigned long v14; // rax
  char *v15; // rax
  long v16; // rax
  char *v17; // rcx
  char v18; // dl
  char *v19; // rdx
  int v2;
  long v20; // rdx
  char v21 [104];
  int v22;
  char *v23;
  int v24;
  char *v25; // rdi
  int v26;
  int v27;
  char *v28; // r9
  int v29;
  int v3;
  int v30;
  int v31; // stack - 0xac
  int v4;
  int v5;
  int v6;
  int v7;
  int v8;
  int v9;
  
  v13 = a0[1];
  v1 = a1[1];
  v2 = a1[5];
  v3 = a0[3];
  v31 = *a1;
  v4 = a1[3];
  v5 = a0[2];
  v6 = a0[4];
  v7 = *a0;
  v8 = a1[4];
  v9 = a1[2];
  v10 = a0[5];
  v22 = v5;
  v24 = v9;
  v29 = v3;
  v30 = v4;
  v26 = v6;
  v27 = v8;
  v12 = sub_1f1d0(v9,v5,CONCAT31((undefined3)((unsigned int)v2 >> 8),v7 == v31) & CONCAT31((undefined3)((unsigned int)v7 >> 8),v13 == v1));
  if (v18) { // branch-flip
    if ((((v22 == v24) || (v29 != v30)) || (v26 != v27)) || (v10 != v2)) {
      if (!v12)
        return;
      sub_1ef30(dcgettext(NULL,"error: invalid date/time value:\n",5));
      v14 = sub_1f010(a0,a2,v21);
      sub_1ef30(dcgettext(NULL,"    user provided time: \'%s\'\n",5),v14);
      v14 = sub_1f010(a1,a2,v21);
      sub_1ef30(dcgettext(NULL,"       normalized time: \'%s\'\n",5),v14);
      v17 = "";
label_1fca6:
      v23 = v17;
      goto label_1fca9;
    }
    if (!v12)
      return;
    v11 = 1;
    sub_1ef30(dcgettext(NULL,"error: invalid date/time value:\n",5));
    v14 = sub_1f010(a0,a2,v21);
    sub_1ef30(dcgettext(NULL,"    user provided time: \'%s\'\n",5),v14);
    v14 = sub_1f010(a1,a2,v21);
    sub_1ef30(dcgettext(NULL,"       normalized time: \'%s\'\n",5),v14);
    v23 = "";
    v19 = "--";
    v17 = "";
  }
  else {
    if (!v12)
      return;
    sub_1ef30(dcgettext(NULL,"error: invalid date/time value:\n",5));
    v14 = sub_1f010(a0,a2,v21);
    sub_1ef30(dcgettext(NULL,"    user provided time: \'%s\'\n",5),v14);
    v14 = sub_1f010(a1,a2,v21);
    sub_1ef30(dcgettext(NULL,"       normalized time: \'%s\'\n",5),v14);
    if (v7 != v31) { // branch-flip
      v17 = "--";
      if (v13 != v1) goto label_1fca6;
      v23 = "";
    }
    else {
      v17 = "";
      v23 = "--";
    }
label_1fca9:
    v19 = "";
    if (v5 != v9)
      v19 = "--";
    v11 = 0;
    v25 = "--";
    if (v3 != v4) goto label_1fcd8;
  }
  v25 = "";
label_1fcd8:
  v28 = "";
  v15 = "--";
  if (v6 == v8)
    v15 = "";
  if (v10 != v2)
    v28 = "----";
  v13 = __snprintf_chk(v21,100,1,100,"                                 %4s %2s %2s %2s %2s %2s",v28,v15,v25,v19,v23,v17);
  if (0 <= v13) {
    if (100 <= v13)
      v13 = 99;
    v16 = (long)v13;
    do {
      v20 = (long)(int)v16;
      if (!v16) {
        v20 = 0;
        break;
      }
      v16 -= 1;
    } while (v21[v16] == ' ');
    v21[v20] = '\0';
  }
  sub_1ef30("%s\n",v21);
  sub_1ef30(dcgettext(NULL,"     possible reasons:\n",5));
  if (v11)
    sub_1ef30(dcgettext(NULL,"       non-existing due to daylight-saving time;\n",5));
  if ((v3 != v4) && (v6 != v8))
    sub_1ef30(dcgettext(NULL,"       invalid day/month combination;\n",5));
  sub_1ef30(dcgettext(NULL,"       numeric values overflow;\n",5));
  if (a3) // branch-flip
    v23 = dcgettext(NULL,"incorrect timezone",5);
  else {
    v23 = dcgettext(NULL,"missing timezone",5);
  }
  sub_1ef30("       %s\n",v23);
}

// Function: sub_1ff30 @ 0x1ff30
void sub_1ff30(unsigned long a0,long a1)
{
  void *v1;
  char v2; // al
  
  if (!sub_1f1d0())
    return;
  sub_1ef30(dcgettext(NULL,"parsed %s part: ",5),a0);
  v1 = stderr;
  if ((((!*(long *)(a1 + 0x68)) && (!*(long *)(a1 + 0x70))) && (!*(long *)(a1 + 0x78))) && (((!*(long *)(a1 + 0x80) && (!*(long *)(a1 + 0x88))) && ((!*(long *)(a1 + 0x90) && (!*(int *)(a1 + 0x98))))))) {
    fputs(dcgettext(NULL,"today/this/now\n",5),v1); // tail-call
    return;
  }
  v2 = sub_1eee0(0,*(long *)(a1 + 0x68),"year(s)");
  v2 = sub_1eee0(v2,*(unsigned long *)(a1 + 0x70),"month(s)");
  v2 = sub_1eee0(v2,*(unsigned long *)(a1 + 0x78),"day(s)");
  v2 = sub_1eee0(v2,*(unsigned long *)(a1 + 0x80),"hour(s)");
  v2 = sub_1eee0(v2,*(unsigned long *)(a1 + 0x88),"minutes");
  v2 = sub_1eee0(v2,*(unsigned long *)(a1 + 0x90),"seconds");
  sub_1eee0(v2,(long)*(int *)(a1 + 0x98),"nanoseconds");
  fputc(10,stderr); // tail-call
}

// Function: sub_20090 @ 0x20090
unsigned long sub_20090(void)
{
  return 0;
}

// Function: sub_200a0 @ 0x200a0
undefined16 sub_200a0(unsigned long a0,int a1,unsigned int a2,char *a3)
{
  char v1 [16];
  char v2 [13];
  
  __snprintf_chk(a3,100,1,0xffffffffffffffff,"(Y-M-D) %s-%02d-%02d",sub_1ee60(a2,v2),(unsigned long)(a1 + 1));
  v1._8_8_ = a0 & 0xffffffff;
  v1._0_8_ = a3;
  return v1._0_16_;
}

// Function: sub_20120 @ 0x20120
undefined16 sub_20120(unsigned long a0,unsigned long a1,unsigned int a2,char *a3)
{
  char v1 [16];
  
  __snprintf_chk(a3,100,1,0xffffffffffffffff,"%02d:%02d:%02d",(unsigned long)a2,a1);
  v1._8_8_ = a0;
  v1._0_8_ = a3;
  return v1._0_16_;
}

// Function: sub_20160 @ 0x20160
unsigned long sub_20160(unsigned long a0,long a1,char a2,int *a3) // return-dupe x2
{
  bool v1;
  int v2; // eax
  long v3;
  unsigned long v4;
  
  if ((long)a0 < 0) {
    v2 = (int)(0xfffffffffffff894 - a0);
    *a3 = v2;
    if (0xfffffffffffff894 - a0 == (long)v2 && !SBORROW8(-0x76c,a0))
      return 1;
    goto label_20219;
  }
  v4 = a0;
  if (a1 == 2) {
    v3 = 2000;
    if (0x45 <= (long)a0)
      v3 = 0x76c;
    v4 = v3 + a0;
    if (a2)
      sub_1ef30(dcgettext(NULL,"warning: adjusting year value %ld to %ld\n",5),a0,v4);
  }
  v1 = 0;
  v3 = v4 - 0x76c;
  if (0x76c <= v4) { // branch-flip
    if (v3 <= -1) {
label_201c5:
      v1 = 1;
    }
  }
  else {
    v1 = 0;
    if (0 <= v3) goto label_201c5;
  }
  *a3 = (int)v3;
  if (v3 != (int)v3)
    v1 = 1;
  a0 = v4;
  if (!v1)
    return 1;
label_20219:
  if (!a2)
    return 0;
  sub_1ef30(dcgettext(NULL,"error: out-of-range year %ld\n",5),a0);
  return 0;
}

// Function: sub_20280 @ 0x20280
unsigned long sub_20280(long a0,char a1,long a2,long a3,long a4) // return-dupe
{
  long v1;
  bool v2;
  
  if (3 <= a3) { // branch-flip
    if (0 <= a4) goto label_20295;
    v1 = a2 / 100;
    a4 = a2 % 100;
  }
  else {
    if (0 <= a4) {
label_20295:
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
      goto label_202b1;
    }
    a4 = 0;
    v1 = a2;
  }
  a4 += v1 * 0x3c;
label_202b1:
  if (0xb41 <= a4 + 0x5a0U)
    return 0;
  *(int *)(a0 + 0x18) = (int)a4 * 0x3c;
  return 1;
}

// Function: sub_20340 @ 0x20340
bool sub_20340(long a0,long a1,long a2,long a3,long a4,long a5,long a6,int a7,int a8)
{
  int v1;
  bool v10;
  bool v11;
  bool v12;
  bool v13;
  bool v14;
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  long v7;
  bool v8;
  bool v9;
  
  v1 = *(int *)(a0 + 0x98);
  v2 = *(long *)(a0 + 0x90);
  v3 = *(long *)(a0 + 0x88);
  v4 = *(long *)(a0 + 0x80);
  v5 = *(long *)(a0 + 0x78);
  v6 = *(long *)(a0 + 0x70);
  v7 = *(long *)(a0 + 0x68);
  if (0 <= a8) { // branch-flip
    v8 = SCARRY4(v1,a7);
    v9 = SCARRY8(v2,a6);
    v10 = SCARRY8(v3,a5);
    v11 = SCARRY8(v4,a4);
    v12 = SCARRY8(v5,a3);
    v13 = SCARRY8(v6,a2);
    v14 = SCARRY8(v7,a1);
  }
  else {
    v8 = SBORROW4(v1,a7);
    a7 = -a7;
    v9 = SBORROW8(v2,a6);
    a6 = -a6;
    v10 = SBORROW8(v3,a5);
    a5 = -a5;
    v11 = SBORROW8(v4,a4);
    a4 = -a4;
    v12 = SBORROW8(v5,a3);
    a3 = -a3;
    v13 = SBORROW8(v6,a2);
    a2 = -a2;
    v14 = SBORROW8(v7,a1);
    a1 = -a1;
  }
  *(int *)(a0 + 0x98) = v1 + a7;
  *(long *)(a0 + 0x90) = v2 + a6;
  *(long *)(a0 + 0x88) = v3 + a5;
  *(long *)(a0 + 0x80) = v4 + a4;
  *(long *)(a0 + 0x78) = v5 + a3;
  v14 = (bool)(v9 | v8 | v10 | v11 | v12 | v13 | v14);
  *(long *)(a0 + 0x70) = v6 + a2;
  *(long *)(a0 + 0x68) = v7 + a1;
  if (!v14)
    *(char *)(a0 + 0xa1) = 1;
  return !v14;
}

// Function: sub_20480 @ 0x20480
unsigned long sub_20480(long a0)
{
  char v1;
  int v10; // eax
  long v11;
  unsigned int v12; // edx
  char v13 [19]; // stack - 0x58
  long v14 [140];
  char v15 [5];
  long v16; // stack - 0x4f8
  int v17; // eax
  long v18;
  long v19;
  long v2;
  char *v20;
  long v21;
  long *v22;
  long *v23;
  long v24;
  long v25;
  long v26; // stack - 0x550
  long v27; // stack - 0x548
  long v28; // stack - 0x540
  long v29; // stack - 0x538
  long v3;
  long v30; // stack - 0x530
  int v31; // stack - 0x528
  int v32; // stack - 0x524
  long v33; // stack - 0x4f0
  long v34; // stack - 0x4e8
  long v35; // stack - 0x4e0
  long v36; // stack - 0x4d8
  long v37; // stack - 0x4d0
  long v38; // stack - 0x4c8
  unsigned int v4;
  long v5; // rcx
  char *v6;
  long v7;
  long v8;
  int v9;
  
  v9 = 0;
  v12 = 0x26;
  v17 = -2;
  v13[0] = '\0';
  v32 = 0;
  v6 = v13;
  v22 = v14;
label_204e9:
  if (v12 != 0xffffffa3) {
    if (v17 == -2)
      v17 = sub_1f5d0(&v16,a0);
    if (1 <= v17) { // branch-flip
      if (v17 == 0x100) {
        v17 = 0x101;
        goto label_20630;
      }
      if (0x116 <= v17) { // branch-flip
        v10 = 2;
        v4 = v12 + 2;
      }
      else {
        v10 = (int)*(char *)((long)v17 + 0x30a40);
        v4 = v12 + v10;
      }
    }
    else {
      v10 = 0;
      v17 = 0;
      v4 = v12;
    }
    if ((0x71 <= v4) || (*(char *)((long)(int)v4 + 0x30800) != v10)) goto label_205a0;
    v1 = *(char *)((long)(int)v4 + 0x30880);
    v9 = (int)v1;
    if (v1 <= '\0') {
      v10 = -v9;
      goto label_20652;
    }
    v17 = -2;
    v22[0xd] = v38;
    v32 = v32 + -1 + (unsigned int)(v32 == 0);
    v22[7] = v16;
    v22[8] = v33;
    v23 = &v22[7];
    v22[9] = v34;
    v22[10] = v35;
    v22[0xb] = v36;
    v22[0xc] = v37;
    goto label_207c0;
  }
label_205a0:
  v10 = (int)*(char *)((long)v9 + 0x30940);
  if (!*(char *)((long)v9 + 0x30940)) {
    if (v32 == 3) {
      if (1 <= v17) // branch-flip
        v17 = -2;
      else if (!v17)
        return 1;
    }
label_20630:
    do {
      if (((v12 != 0xffffffa3) && (v12 = v12 + 1, v12 <= 0x70)) && (*(char *)((long)(int)v12 + 0x30800) == '\x01')) {
        v1 = *(char *)((long)(int)v12 + 0x30880);
        v9 = (int)v1;
        if ('\0' < v1) goto label_20872;
      }
      if (v6 == v13)
        return 1;
      v20 = &v6[-1];
      v6 = &v6[-1];
      v22 = &v22[-7];
      v12 = (unsigned int)*(char *)((long)*v20 + 0x309c0);
    } while( true );
  }
label_20652:
  v21 = (long)v10;
  v5 = (long)*(char *)(v21 + 0x30740);
  v23 = &v22[(long)(1 - (int)*(char *)(v21 + 0x30740)) * 7];
  v26 = *v23;
  v30 = v23[2];
  v2 = v23[1];
  v27 = v23[3];
  v29 = v23[4];
  v28 = v23[5];
  v31 = (int)v23[6];
  v8 = a0;
  switch(v10) {
    case 4:
      v8 = *v22;
      v3 = v22[1];
      v20 = "number of seconds";
      *(char *)(a0 + 0xa0) = 1;
      *(long *)(a0 + 0x58) = v8;
      *(long *)(a0 + 0x60) = v3;
      goto label_206f8;
    case 7:
      v20 = "datetime";
      *(long *)(a0 + 200) = *(long *)(a0 + 200) + 1;
      *(long *)(a0 + 0xa8) = *(long *)(a0 + 0xa8) + 1;
      goto label_206f8;
    case 8:
      v20 = "time";
      *(long *)(a0 + 200) = *(long *)(a0 + 200) + 1;
      goto label_206f8;
    case 9:
      v20 = "local_zone";
      *(long *)(a0 + 0xb8) = *(long *)(a0 + 0xb8) + 1;
      goto label_206f8;
    case 10:
      v20 = "zone";
      *(long *)(a0 + 0xd0) = *(long *)(a0 + 0xd0) + 1;
      goto label_206f8;
    case 0xb:
      v20 = "date";
      *(long *)(a0 + 0xa8) = *(long *)(a0 + 0xa8) + 1;
      goto label_206f8;
    case 0xc:
      v20 = "day";
      *(long *)(a0 + 0xb0) = *(long *)(a0 + 0xb0) + 1;
      goto label_206f8;
    case 0xd:
      v20 = "relative";
      goto label_21080;
    case 0xe:
      v20 = "number";
label_206f8:
      sub_1f1e0(dcgettext(NULL,v20,5),a0);
      break;
    case 0xf:
      v20 = "hybrid";
      goto label_21080;
    case 0x12:
      sub_1e900(a0,v22[-6],0,0,0);
      *(int *)(v8 + 0x1c) = (int)*v22;
      break;
    case 0x13:
      v3 = v22[-6];
      v19 = v22[-0x14];
      v9 = 0;
      v8 = 0;
      goto label_21021;
    case 0x14:
      v8 = v22[-7];
      v3 = v22[-0x14];
      v19 = v22[-0x22];
      v9 = (int)v22[-6];
label_21021:
      v7 = a0;
      sub_1e900(a0,v19,v3,v8,v9);
      *(int *)(v7 + 0x1c) = (int)*v22;
      break;
    case 0x16:
      sub_1e900(a0,v22[-6],0,0,0);
      *(unsigned int *)(a0 + 0x1c) = 2;
      break;
    case 0x17:
      sub_1e900(a0,v22[-0x14],v22[-6],0,0);
      *(unsigned int *)(a0 + 0x1c) = 2;
      break;
    case 0x18:
      sub_1e900(a0,v22[-0x22],v22[-0x14],v22[-7],(int)v22[-6]);
      *(unsigned int *)(a0 + 0x1c) = 2;
      break;
    case 0x1b:
      v8 = v22[-5];
      v3 = v22[-6];
      v19 = v22[-7];
      *(long *)(a0 + 0xd0) = *(long *)(a0 + 0xd0) + 1;
      v1 = sub_20280(a0,(char)v19,v3,v8,*v22);
      goto label_209a0;
    case 0x1c:
      *(int *)(a0 + 0x14) = (int)*v22;
      break;
    case 0x1d:
      *(long *)(a0 + 0xc0) = *(long *)(a0 + 0xc0) + 1;
      *(unsigned int *)(a0 + 0x14) = 1;
      break;
    case 0x1e:
      *(int *)(a0 + 0x18) = (int)*v22;
      break;
    case 0x1f:
      *(unsigned int *)(a0 + 0x18) = 0xffff9d90;
      break;
    case 0x20:
      *(int *)(a0 + 0x18) = (int)v22[-7];
      goto label_21206;
    case 0x21:
      *(unsigned int *)(a0 + 0x18) = 0xffff9d90;
label_21206:
      v8 = v22[4];
      v3 = v22[3];
      v19 = v22[2];
      v7 = v22[1];
      v18 = v22[6];
      v11 = v22[5];
      v24 = *v22;
      if (!sub_20340(a0,v24,v7,v19,v3,v8,v11,(int)v18,1))
        return 1;
      v20 = "relative";
label_21080:
      sub_1ff30(dcgettext(NULL,v20,5),a0);
      break;
    case 0x22:
      v8 = v22[-5];
      v3 = v22[-6];
      v7 = v22[-7];
      v19 = *v22;
      if (!sub_20280(a0,(char)v7,v3,v8,v19))
        return 1;
      v9 = *(int *)(a0 + 0x18);
      v8 = v22[-0xe];
      v3 = (long)v9 + v22[-0xe];
      v10 = (int)v3;
      *(int *)(a0 + 0x18) = v10;
      if (v3 != v10 || SCARRY8((long)v9,v8))
        return 1;
      break;
    case 0x23:
      *(int *)(a0 + 0x18) = (int)*v22 + 0xe10;
      break;
    case 0x24:
      *(int *)(a0 + 0x18) = (int)v22[-7] + 0xe10;
      break;
    case 0x25:
      v8 = *v22;
      *(unsigned long *)(a0 + 8) = 0;
      *(int *)(a0 + 0x10) = (int)v8;
      break;
    case 0x26:
      v8 = v22[-7];
      *(unsigned long *)(a0 + 8) = 0;
      *(int *)(a0 + 0x10) = (int)v8;
      break;
    case 0x27:
      *(long *)(a0 + 8) = v22[-7];
      v8 = *v22;
      *(char *)(a0 + 0xdf) = 1;
      *(int *)(a0 + 0x10) = (int)v8;
      break;
    case 0x28:
      *(long *)(a0 + 8) = v22[-6];
      v8 = *v22;
      *(char *)(a0 + 0xdf) = 1;
      *(int *)(a0 + 0x10) = (int)v8;
      break;
    case 0x29:
      *(long *)(a0 + 0x38) = v22[-0xd];
      *(long *)(a0 + 0x40) = v22[1];
      break;
    case 0x2a:
      v8 = v22[-0x1a];
      v3 = v8;
      v1 = sub_1f1d0();
      if (4 <= v3) { // branch-flip
        if (v1) {
          v3 = v22[-0x1b];
          sub_1ef30(dcgettext(NULL,"warning: value %ld has %ld digits. Assuming YYYY/MM/DD\n",5),v3,v8);
        }
        v3 = v22[-0x1c];
        v19 = v22[-0x1b];
        *(long *)(a0 + 0x30) = v22[-0x1a];
        v8 = v22[-0xd];
        *(long *)(a0 + 0x20) = v3;
        *(long *)(a0 + 0x28) = v19;
        *(long *)(a0 + 0x38) = v8;
        *(long *)(a0 + 0x40) = v22[1];
      }
      else {
        v8 = v22[-0x1b];
        if (v1)
          sub_1ef30(dcgettext(NULL,"warning: value %ld has less than 4 digits. Assuming MM/DD/YY[YY]\n",5),v8);
        v3 = *v22;
        v19 = v22[1];
        *(long *)(a0 + 0x38) = v8;
        v8 = v22[-0xd];
        *(long *)(a0 + 0x20) = v3;
        *(long *)(a0 + 0x28) = v19;
        *(long *)(a0 + 0x40) = v8;
        *(long *)(a0 + 0x30) = v22[2];
      }
      break;
    case 0x2b:
      *(long *)(a0 + 0x40) = v22[-0xd];
      *(long *)(a0 + 0x38) = v22[-7];
      v8 = v22[1];
      *(long *)(a0 + 0x28) = -v8;
      goto label_20e87;
    case 0x2c:
      *(long *)(a0 + 0x38) = v22[-0xe];
      v8 = v22[-6];
      *(long *)(a0 + 0x40) = -v8;
      if (v8 == -0x8000000000000000)
        return 1;
      v8 = v22[1];
      *(long *)(a0 + 0x28) = -v8;
label_20e87:
      if (v8 == -0x8000000000000000)
        return 1;
label_20b89:
      *(long *)(a0 + 0x30) = v22[2];
      break;
    case 0x2d:
      *(long *)(a0 + 0x38) = v22[-7];
      *(long *)(a0 + 0x40) = v22[1];
      break;
    case 0x2e:
      v3 = *v22;
      v19 = v22[1];
      *(long *)(a0 + 0x38) = v22[-0x15];
      v8 = v22[-0xd];
      *(long *)(a0 + 0x20) = v3;
      *(long *)(a0 + 0x28) = v19;
      *(long *)(a0 + 0x40) = v8;
      goto label_20b89;
    case 0x2f:
      *(long *)(a0 + 0x40) = v22[-6];
      *(long *)(a0 + 0x38) = *v22;
      break;
    case 0x30:
      v3 = *v22;
      v19 = v22[1];
      *(long *)(a0 + 0x40) = v22[-0xd];
      v8 = v22[-7];
      *(long *)(a0 + 0x20) = v3;
      *(long *)(a0 + 0x28) = v19;
      *(long *)(a0 + 0x38) = v8;
      *(long *)(a0 + 0x30) = v22[2];
      break;
    case 0x32:
      v3 = v22[-0xe];
      v19 = v22[-0xd];
      *(long *)(a0 + 0x30) = v22[-0xc];
      v8 = v22[-6];
      *(long *)(a0 + 0x20) = v3;
      *(long *)(a0 + 0x28) = v19;
      *(long *)(a0 + 0x38) = -v8;
      if (v8 == -0x8000000000000000)
        return 1;
      v8 = v22[1];
      *(long *)(a0 + 0x40) = -v8;
      if (v8 == -0x8000000000000000)
        return 1;
      break;
    case 0x33:
      v25 = *v22;
      v3 = v22[-3];
      v19 = v22[-4];
      v7 = v22[-5];
      v11 = v22[-6];
      v18 = v22[-7];
      v9 = (int)v22[-1];
      v24 = v22[-2];
      goto label_2098b;
    case 0x34:
    case 0x35:
      v3 = v22[4];
      v19 = v22[3];
      v7 = v22[2];
      v11 = v22[1];
      v25 = 1;
      v9 = (int)v22[6];
      v24 = v22[5];
      v18 = *v22;
      goto label_2098b;
    case 0x36:
      v26 = v22[-7];
      goto label_20a81;
    case 0x37:
    case 0x4b:
      v26 = v22[-6];
label_20a81:
      v31 = 0;
      v28 = 0;
      v29 = 0;
      v27 = 0;
      v30 = 0;
label_20914:
      v2 = 0;
      break;
    case 0x38:
      v31 = 0;
      v2 = 0;
      v28 = 0;
      v29 = 0;
      v27 = 0;
      v30 = 0;
      v26 = 1;
      break;
    case 0x39:
      v2 = v22[-7];
      goto label_208c4;
    case 0x3a:
    case 0x4c:
      v2 = v22[-6];
label_208c4:
      v31 = 0;
      v28 = 0;
      v29 = 0;
      v27 = 0;
      v30 = 0;
      v26 = 0;
      break;
    case 0x3b:
      v31 = 0;
      v2 = 1;
      v28 = 0;
      v29 = 0;
      v27 = 0;
      v30 = 0;
      v26 = 0;
      break;
    case 0x3c:
      v2 = v22[-7];
      goto label_20abb;
    case 0x3d:
    case 0x4d:
      v2 = v22[-6];
label_20abb:
      v30 = v2 * *v22;
      if (SEXT816(v30) != SEXT816(v2) * SEXT816(*v22))
        return 1;
      v28 = 0;
      v2 = 0;
      v29 = 0;
      v27 = 0;
      v31 = 0;
      v26 = 0;
      break;
    case 0x3e:
    case 0x51:
      v30 = *v22;
      v2 = 0;
      v31 = 0;
      v28 = 0;
      v29 = 0;
      v27 = 0;
      v26 = 0;
      break;
    case 0x3f:
      v27 = v22[-7];
      goto label_20a44;
    case 0x40:
    case 0x4e:
      v27 = v22[-6];
label_20a44:
      v2 = 0;
      v31 = 0;
      v28 = 0;
      v29 = 0;
      v30 = 0;
      v26 = 0;
      break;
    case 0x41:
      v31 = 0;
      v2 = 0;
      v28 = 0;
      v29 = 0;
      v27 = 1;
      v30 = 0;
      v26 = 0;
      break;
    case 0x42:
      v29 = v22[-7];
      goto label_209ca;
    case 0x43:
    case 0x4f:
      v29 = v22[-6];
label_209ca:
      v2 = 0;
      v31 = 0;
      v28 = 0;
      v27 = 0;
      v30 = 0;
      v26 = 0;
      break;
    case 0x44:
      v31 = 0;
      v2 = 0;
      v28 = 0;
      v29 = 1;
      v27 = 0;
      v30 = 0;
      v26 = 0;
      break;
    case 0x45:
      v28 = v22[-7];
      goto label_20920;
    case 0x46:
    case 0x50:
      v28 = v22[-6];
label_20920:
      v31 = 0;
      goto label_2092d;
    case 0x47:
    case 0x48:
      v28 = v22[-7];
      v31 = (int)v22[-6];
      goto label_2092d;
    case 0x49:
      v31 = 0;
      v28 = 1;
label_2092d:
      v29 = 0;
      v2 = 0;
      v27 = 0;
      v30 = 0;
      v26 = 0;
      break;
    case 0x55:
    case 0x57:
      v26 = v22[1];
      if (sub_20090())
        return 1;
      goto label_20914;
    case 0x58:
      sub_1e780(a0);
      break;
    case 0x59:
      sub_1e780(a0);
      v3 = v22[4];
      v19 = v22[3];
      v7 = v22[2];
      v11 = v22[1];
      v25 = 1;
      v9 = (int)v22[6];
      v24 = v22[5];
      v18 = *v22;
label_2098b:
      v1 = sub_20340(v8,v18,v11,v7,v19,v3,v24,v9,v25);
label_209a0:
      if (!v1)
        return 1;
      break;
    case 0x5a:
      v26 = -1;
      break;
    case 0x5b:
      v26 = v22[1];
    
  }
  v6 = &v6[-v5];
  v22[v5 * -7 + 7] = v26;
  v23 = &v22[v5 * -7 + 7];
  v22[v5 * -7 + 9] = v30;
  v22[v5 * -7 + 10] = v27;
  v22[v5 * -7 + 0xb] = v29;
  v22[v5 * -7 + 0xc] = v28;
  *(int *)&v22[v5 * -7 + 0xd] = v31;
  v1 = *v6;
  v22[v5 * -7 + 8] = v2;
  v2 = (long)(*(char *)(v21 + 0x307a0) + -0x1c);
  v12 = (int)*(char *)(v2 + 0x30920) + (int)v1;
  if ((0x71 <= v12) || (v1 != *(char *)((long)(int)v12 + 0x30800))) {
    v1 = *(char *)(v2 + 0x30900);
    v9 = (int)v1;
  }
  else {
    v1 = *(char *)((long)(int)v12 + 0x30880);
    v9 = (int)v1;
  }
  goto label_207c0;
label_20872:
  v32 = 3;
  v23 = &v22[7];
  *v23 = v16;
  v22[8] = v33;
  v22[0xd] = v38;
  v22[9] = v34;
  v22[10] = v35;
  v22[0xb] = v36;
  v22[0xc] = v37;
label_207c0:
  v6[1] = v1;
  v6 = &v6[1];
  if (v15 <= v6)
    return 2;
  if (v9 == 0xc)
    return 0;
  v12 = (unsigned int)*(char *)((long)v9 + 0x309c0);
  v22 = v23;
  goto label_204e9;
}

// Function: sub_21400 @ 0x21400
unsigned int sub_21400(unsigned long *a0,char *a1,long *a2,long a3,char *a4)
{
  char *v1;
  int v10;
  unsigned int v100; // stack - 0x1e0
  int v101; // stack - 0x1dc
  char *v102; // stack - 0x1d8
  unsigned int v103; // stack - 0x1d0
  unsigned long v104; // stack - 0x1c8
  char v105; // stack - 0xa6
  unsigned long v11; // rax
  unsigned long v12; // rax
  char *v13;
  long v14;
  char *v15;
  long v16;
  long v17;
  unsigned long v18;
  long v19;
  unsigned long v2;
  int v20; // edx
  char *v21;
  long v22; // rdx
  int v23; // stack - 0x388
  char v24 [32];
  long v25 [8]; // stack - 0x348
  char *v26; // stack - 0x2c8
  char v27 [112];
  char v28 [32];
  char v29 [112];
  unsigned int v3;
  unsigned long v30; // stack - 0x3f8
  unsigned long v31; // stack - 0x408
  unsigned int v32; // stack - 0x3c8
  long v33 [2];
  char v34 [101];
  unsigned short v35; // stack - 0xa8
  char v36 [13];
  int v37;
  int v38;
  unsigned long v39;
  unsigned int v4;
  void *v40;
  long v41; // rax
  long v42; // r11
  char *v43;
  bool v44; // of
  char *v45; // stack - 0x468
  char *v46; // stack - 0x460
  long v47; // stack - 0x458
  int v48; // stack - 0x450
  unsigned long v49; // stack - 0x440
  long v5;
  unsigned long v50; // stack - 0x438
  unsigned long v51; // stack - 0x400
  int v52; // stack - 0x3f0
  int v53; // stack - 0x3e8
  long v54; // stack - 0x3e0
  unsigned int v55; // stack - 0x3c4
  int v56; // stack - 0x3c0
  int v57; // stack - 0x3bc
  int v58; // stack - 0x3b8
  unsigned int v59; // stack - 0x3b4
  char v6;
  int v60; // stack - 0x3a8
  int v61; // stack - 0x384
  int v62; // stack - 0x380
  int v63; // stack - 0x37c
  int v64; // stack - 0x378
  int v65; // stack - 0x374
  int v66; // stack - 0x368
  char *v67; // stack - 0x358
  int v68; // stack - 0x2e8
  unsigned int v69; // stack - 0x2e0
  char v7;
  char *v70; // stack - 0x2d8
  long v71; // stack - 0x2c0
  int v72; // stack - 0x2b8
  int v73; // stack - 0x2b4
  int v74; // stack - 0x2b0
  int v75; // stack - 0x2ac
  long v76; // stack - 0x2a0
  unsigned long v77; // stack - 0x298
  long v78; // stack - 0x290
  long v79; // stack - 0x288
  unsigned int v8; // eax
  long v80; // stack - 0x280
  unsigned long v81; // stack - 0x278
  unsigned long v82; // stack - 0x270
  unsigned long v83; // stack - 0x268
  long v84; // stack - 0x260
  long v85; // stack - 0x258
  long v86; // stack - 0x250
  long v87; // stack - 0x248
  long v88; // stack - 0x240
  long v89; // stack - 0x238
  int v9;
  int v90; // stack - 0x230
  unsigned short v91; // stack - 0x228
  unsigned long v92; // stack - 0x220
  unsigned long v93; // stack - 0x218
  long v94; // stack - 0x210
  unsigned long v95; // stack - 0x208
  unsigned long v96; // stack - 0x200
  long v97; // stack - 0x1f8
  unsigned long v98; // stack - 0x1f0
  char *v99; // stack - 0x1e8
  
  v49 = strlen(a1);
  if (!a2) {
    a2 = v33;
    sub_28520(a2);
  }
  v19 = *a2;
  v16 = a2[1];
  v43 = a1;
  while (v6 = *v43, sub_27f50(v6)) {
    v43 = &v43[1];
  }
  if (strncmp(v43,"TZ=\"",4)) { // branch-flip
label_214e8:
    if (!sub_26090(a3,a2,&v23)) {
      v45 = NULL; // return-dupe
      v8 = 0;
      free(v45); // return-dupe
      return v8;
    }
    v45 = NULL;
    v14 = a3;
    v46 = a4;
label_2150f:
    v37 = 0x76a700;
    v77 = 0;
    v75 = 2;
    if (!*v43)
      v43 = "0";
    v84 = 0;
    v76 = (long)v65 + 0x76c;
    v48 = (int)v16;
    v83 = (unsigned long)v48;
    v85 = 0;
    v78 = (long)(v64 + 1);
    v86 = 0;
    v79 = (long)v63;
    v87 = 0;
    v80 = (long)v62;
    v88 = 0;
    v81 = (unsigned long)v61;
    v89 = 0;
    v82 = (unsigned long)v23;
    v90 = 0;
    v91 = 0;
    v53 = v66;
    v92 = 0;
    v93 = 0;
    v94 = 0;
    v95 = 0;
    v96 = 0;
    v100 = 0x10d;
    v97 = 0;
    v98 = 0;
    v99 = v67;
    v101 = v66;
    v102 = NULL;
    v26 = v43;
    do {
      v25[0] = v37 + v19;
      if (SCARRY8((long)v37,v19)) break;
      if (((sub_26090(v14,v25,v24)) && (v70)) && (v68 != v101)) {
        v103 = 0x10d;
        v104 = 0;
        v102 = v70;
        break;
      }
      v37 += 0x76a700;
    } while (v37 != 0x1da9c00);
    if (((v99) && (v102)) && (v37 = strcmp(v99,v102), !v37)) {
      v101 = -1;
      v102 = NULL;
    }
    sub_20480(&v26);
    v8 = sub_1f1d0();
    v43 = v26;
    v6 = (char)v8;
    if (v20) { // branch-flip
      if (v6) {
        if (&a1[v49] <= v26) // branch-flip
          v13 = dcgettext(NULL,"error: parsing failed\n",5);
        else {
          v13 = dcgettext(NULL,"error: parsing failed, stopped at \'%s\'\n",5);
        }
        v8 = 0;
        sub_1ef30(v13,v43);
      }
    }
    else {
      if (v6) { // branch-flip
        sub_1ef30(dcgettext(NULL,"input timezone: ",5));
        v43 = "\'@timespec\' - always UTC";
        if (((char)v91) || (v43 = "parsed date/time string", v97)) {
label_21da0:
          __fprintf_chk(stderr,1,dcgettext(NULL,v43,5));
        }
        else {
          if (!v46) {
            v43 = "system default";
            goto label_21da0;
          }
          if (a3 != v14) // branch-flip
            __fprintf_chk(stderr,1,dcgettext(NULL,"TZ=\"%s\" in date string",5),v46);
          else {
            v43 = "TZ=\"UTC0\" environment value or -u";
            if (!strcmp(v46,"UTC0")) goto label_21da0;
            __fprintf_chk(stderr,1,dcgettext(NULL,"TZ=\"%s\" environment value",5),v46);
          }
        }
        if (v94) { // branch-flip
          if (v97) goto label_21e14;
          v40 = stderr;
          if (1 <= v73) {
            __fprintf_chk(stderr,1,", dst");
            goto label_21e00;
          }
        }
        else {
label_21e00:
          if (v97) { // branch-flip
label_21e14:
            v11 = sub_1ed30(v74,v28);
            __fprintf_chk(stderr,1," (%s)",v11);
            v40 = stderr;
          }
          else {
            v40 = stderr;
          }
        }
        fputc(10,v40);
        if ((char)v91) goto label_21769;
label_21900:
        if (2 <= (long)(v96 | v92 | v93 | v95 | v97 + v94)) {
          if (v6) {
            if (1 < (long)v96)
              sub_1ef30("error: seen multiple time parts\n");
            if (1 < (long)v92)
              sub_1ef30("error: seen multiple date parts\n");
            if (1 < (long)v93)
              sub_1ef30("error: seen multiple days parts\n");
            if (1 < (long)v95)
              sub_1ef30("error: seen multiple daylight-saving parts\n");
            if (1 < v97 + v94)
              sub_1ef30(0x30000);
          }
          goto label_21998;
        }
        v7 = sub_20160(v76,v77,v8 & 0xff,(long)&v30 + 4);
        v19 = v80;
        if (!v7) {
label_21d38:
          if (v6) {
            v43 = "error: year, month, or day overflow\n";
label_21d4d:
            sub_1ef30(dcgettext(NULL,v43,5));
          }
label_21998:
          v8 = 0;
          goto label_218b8;
        }
        v37 = (int)(v78 + -1);
        v30 = CONCAT44(v30._4_4_,v37);
        if (v78 + -1 != (long)v37 || SCARRY8(v78,-1)) goto label_21d38;
        v9 = (int)v79;
        v51 = CONCAT44(v9,(unsigned int)v51);
        if (v79 != v9) goto label_21d38;
        v39 = v96;
        if (v96) { // branch-flip
label_22430:
          v38 = v75;
          v10 = sub_1e920(v80);
          v51 = CONCAT44(v51._4_4_,v10);
          if (v10 < 0) {
            v43 = "am";
            if ((v38) && (v43 = "", v38 == 1))
              v43 = "pm";
            if (v6)
              sub_1ef30(dcgettext(NULL,"error: invalid hour %ld%s\n",5),v19,v43);
            goto label_21998;
          }
          v55 = (unsigned int)v81;
          v32 = (unsigned int)v82;
          v31 = CONCAT44(v55,v32);
          if (v6) {
            v11 = sub_20120(v82 & 0xffffffff,v81 & 0xffffffff,v10,v27);
            if (v96) // branch-flip
              v43 = dcgettext(NULL,"using specified time as starting value: \'%s\'\n",5);
            else {
              v43 = dcgettext(NULL,"using current time as starting value: \'%s\'\n",5);
            }
            sub_1ef30(v43,v11);
            v39 = v96; // crossjump-dupe
            v10 = (int)v51;
            v32 = (unsigned int)v31;
            v55 = v31._4_4_;
            v9 = v51._4_4_;
            v37 = (int)v30;
          }
label_2247a:
          if ((v92 || v93) || v39) goto label_21c4d;
        }
        else {
          if ((!v91._1_1_) || (v92)) {
            v31 = 0;
            v10 = 0;
            v55 = 0;
            v51 = v79 << 0x20;
            v83 = 0;
            v32 = 0;
            if (v6) {
label_224d1:
              v51 = v79 << 0x20;
              v83 = 0;
              v31 = 0;
              sub_1ef30("warning: using midnight as starting time: 00:00:00\n");
              v39 = v96;
              v10 = (int)v51;
              v32 = (unsigned int)v31;
              v55 = v31._4_4_;
              v9 = v51._4_4_;
              v37 = (int)v30;
            }
            goto label_2247a;
          }
          if (!v93) goto label_22430;
          v31 = 0;
          v51 = v79 << 0x20;
          v83 = 0;
          if (v6) goto label_224d1;
          v10 = 0;
          v55 = 0;
          v32 = 0;
label_21c4d:
          v53 = -1;
        }
        if (v94) { // branch-flip
          v53 = v73;
          v60 = v73;
        }
        else {
          v60 = v53;
        }
        v59 = v30._4_4_;
        v52 = -1;
        v56 = v10;
        v57 = v9;
        v58 = v37;
        v47 = sub_26130(v14,&v31);
        v7 = sub_1e990(&v32,&v31);
        v19 = v97;
        if (!v7) {
          if (v97) {
            v35 = 0x5858;
            v105 = 0x58;
            sub_1ed30(v74,v34);
            v16 = sub_25d70(&v35);
            if (!v16) {
              if (v6)
                sub_1ef30(dcgettext(NULL,"error: tzalloc (\"%s\") failed\n",5),&v35);
              goto label_21998;
            }
            v31 = CONCAT44(v55,v32);
            v52 = -1;
            v51 = CONCAT44(v57,v56);
            v30 = CONCAT44(v59,v58);
            v53 = v60;
            v47 = sub_26130(v16,&v31);
            v7 = sub_1e990(&v32,&v31);
            sub_25f40(v16);
            if (v7) goto label_2204e;
          }
          sub_1fa50(&v32,&v31,&v26,v19 != 0);
          goto label_21998;
        }
label_2204e:
        if (v93) {
          if (!v92) {
            v19 = v71;
            if (1 <= v71)
              v19 = v71 - (unsigned long)(v52 != v72);
            v16 = v19 * 7;
            if ((SEXT816(v16) == SEXT816(v19) * SEXT816(7)) && (v19 = (long)(((v72 - v52) + 7) % 7), v17 = v19 + v16, !SCARRY8(v19,v16))) {
              v16 = (long)v51._4_4_;
              v19 = v17 + v16;
              v37 = (int)v19;
              v51 = CONCAT44(v37,(unsigned int)v51);
              if (v19 == v37 && !SCARRY8(v17,v16)) {
                v53 = -1;
                v47 = sub_26130(v14,&v31);
                if (v47 != -1) {
                  if (v6) {
                    v11 = sub_1f010(&v31,&v26,v27);
                    v12 = sub_1f0c0(&v26,&v35);
                    sub_1ef30(dcgettext(NULL,"new start date: \'%s\' is \'%s\'\n",5),v12,v11);
                    v39 = v51;
                    v18 = v30;
                    v2 = v93;
                    v19 = v47;
                    if (!v92) goto label_2251b;
                    goto label_22969;
                  }
                  goto label_2212d;
                }
              }
            }
            if (v6) {
              v11 = sub_1f010(&v31,&v26,v27);
              v12 = sub_1f0c0(&v26,&v35);
              sub_1ef30(dcgettext(NULL,"error: day \'%s\' (day ordinal=%ld number=%d) resulted in an invalid date: \'%s\'\n",5),v12,v71,v72,v11);
            }
            goto label_21998;
          }
          v19 = v47;
          if (v6) {
label_22654:
            v47 = v19;
            v11 = sub_1f0c0(&v26,&v35);
            sub_1ef30(dcgettext(NULL,"warning: day (%s) ignored when explicit dates are given\n",5),v11);
            goto label_22521;
          }
label_2212d:
          if ((v84 || v85) || v86) { // branch-flip
            v39 = v30._4_4_ + v84;
            if ((v39 == (long)(int)v39) && (!SCARRY8((long)v30._4_4_,v84))) {
              v19 = (int)v30 + v85;
              v49._0_4_ = (int)v19;
              if (!(v19 != (int)v49 || SCARRY8((long)(int)v30,v85))) {
label_2219e:
                v50 = v39 & 0xffffffff;
                v18 = v51._4_4_ + v86;
                v37 = (int)v18;
                if ((v18 != (long)v37) || (SCARRY8((long)v51._4_4_,v86))) goto label_22604;
                v30 = CONCAT44((int)v39,(int)v49);
                v31 = CONCAT44(v55,v32);
                v51 = CONCAT44(v37,v56);
                v53 = v60;
                v47 = sub_26130(v14,&v31);
                v17 = v86;
                v16 = v85;
                v19 = v84;
                if (v47 != -1) { // branch-flip
                  if (v6) {
                    sub_1ef30(dcgettext(NULL,"after date adjustment (%+ld years, %+ld months, %+ld days),\n",5),v19,v16,v17);
                    v11 = sub_1f010(&v31,&v26,v27);
                    sub_1ef30(dcgettext(NULL,"    new date/time = \'%s\'\n",5),v11);
                    if ((v60 != -1) && (v60 != v53))
                      sub_1ef30(dcgettext(NULL,"warning: daylight saving time changed after date adjustment\n",5));
                    if ((!v86) && ((v37 != v51._4_4_ || ((!v85 && ((int)v49 != (int)v30)))))) {
                      sub_1ef30(dcgettext(NULL,"warning: month/year adjustment resulted in shifted dates:\n",5));
                      v11 = sub_1ee60(v50,v36);
                      sub_1ef30(dcgettext(NULL,"     adjusted Y M D: %s %02d %02d\n",5),v11,(int)v49 + 1,v18 & 0xffffffff);
                      v39 = v51 >> 0x20;
                      v37 = (int)v30 + 1;
                      v11 = sub_1ee60(v30._4_4_,v36);
                      sub_1ef30(dcgettext(NULL,"   normalized Y M D: %s %02d %02d\n",5),v11,v37,v39);
                    }
                    if (v97) {
                      v39 = (unsigned long)v74;
                      v44 = SBORROW8(v47,v39 - v54);
                      v47 -= v39 - v54;
                      if (v44 || SBORROW8(v39,v54)) goto label_22985;
                    }
label_22749:
                    v11 = sub_1f010(&v31,&v26,v27);
                    sub_1ef30(dcgettext(NULL,"\'%s\' = %ld epoch-seconds\n",5),v11,v47);
                    v19 = v87 * 0xe10;
                    if (SEXT816(v19) == SEXT816(v87) * SEXT416(0xe10)) goto label_2228e;
                    goto label_227a6;
                  }
                  if ((!v97) || (v19 = v74 - v54, v44 = SBORROW8(v47,v19), v47 = v47 - v19, !(v44 || SBORROW8((long)v74,v54)))) goto label_22269;
                }
                else if (v6) {
                  v11 = sub_1f010(&v31,&v26,v27);
                  sub_1ef30(dcgettext(NULL,"error: adding relative date resulted in an invalid date: \'%s\'\n",5),v11);
                }
              }
            }
          }
          else {
            if (!v97) goto label_22269;
label_22704:
            v39 = (unsigned long)v74;
            v44 = SBORROW8(v47,v39 - v54);
            v47 -= v39 - v54;
            if (!SBORROW8(v39,v54) && !v44) goto label_22738;
            if (!v6) goto label_21998;
label_22985:
            sub_1ef30(dcgettext(NULL,"error: timezone %d caused time_t overflow\n",5),v39 & 0xffffffff);
          }
          goto label_21998;
        }
        v39 = v51;
        v18 = v30;
        v2 = v92;
        if (!v6) goto label_2212d;
label_2251b:
        v51 = v39;
        v30 = v18;
        if (!v2) {
          v30._4_4_ = (unsigned int)(v18 >> 0x20);
          v4 = v30._4_4_;
          v51._4_4_ = (unsigned int)(v39 >> 0x20);
          v3 = v51._4_4_;
          v11 = sub_200a0(v3,v18 & 0xffffffff,v4,v27);
          sub_1ef30(dcgettext(NULL,"using current date as starting value: \'%s\'\n",5),v11);
          v19 = v47;
          v2 = v92;
          if (v93) {
label_22969:
            if (v2) goto label_22654;
          }
        }
label_22521:
        v11 = sub_1f010(&v31,&v26,v27);
        sub_1ef30(dcgettext(NULL,"starting date/time: \'%s\'\n",5),v11);
        if (!((!v84 && !v85) && !v86)) {
          if (v84 || v85) { // branch-flip
            if (v51._4_4_ != 0xf)
              sub_1ef30(dcgettext(NULL,"warning: when adding relative months/years, it is recommended to specify the 15th of the months\n",5));
            if (v86) goto label_22871;
          }
          else {
label_22871:
            if ((int)v51 != 0xc)
              sub_1ef30(dcgettext(NULL,"warning: when adding relative days, it is recommended to specify noon\n",5));
          }
          v39 = v30._4_4_ + v84;
          if ((v39 == (long)(int)v39) && (!SCARRY8((long)v30._4_4_,v84))) {
            v19 = (int)v30 + v85;
            v49._0_4_ = (int)v19;
            if (v19 == (int)v49 && !SCARRY8((long)(int)v30,v85)) goto label_2219e;
label_22604:
            if (!v6) goto label_21998;
          }
          sub_1ef30(dcgettext(NULL,"error: %s:%d\n",5),"parse-datetime.y",0x865);
          goto label_21998;
        }
        if (v97) goto label_22704;
label_22738:
        if (v6) goto label_22749;
label_22269:
        v19 = v87 * 0xe10;
        if (SEXT816(v19) != SEXT816(v87) * SEXT416(0xe10)) goto label_21998;
label_2228e:
        v37 = v90;
        v5 = v89;
        v17 = v88;
        v16 = v87;
        v41 = v83 + (long)v90;
        v39 = (v41 % 1000000000 + 1000000000U) % 1000000000;
        if ((((SCARRY8(v19,v47)) || (v22 = v88 * 0x3c, SEXT816(v22) != SEXT816(v88) * SEXT816(0x3c))) || (v42 = v19 + v47 + v22, SCARRY8(v19 + v47,v22))) || ((v19 = v42 + v89, SCARRY8(v42,v89) || (v41 = (long)(int)((long)(v41 - v39) / 1000000000), v18 = v19 + v41, SCARRY8(v19,v41))))) {
          if (!v6) goto label_21998;
label_227a6:
          v43 = "error: adding relative time caused an overflow\n";
          goto label_21d4d;
        }
        *a0 = v18;
        a0[1] = v39;
        if (v6) {
          if (((v88 || v89) || (long)v90) || v87) {
            sub_1ef30(dcgettext(NULL,"after time adjustment (%+ld hours, %+ld minutes, %+ld seconds, %+d ns),\n",5),v16,v17,v5,v37);
            sub_1ef30(dcgettext(NULL,"    new time = %ld epoch-seconds\n",5),v18);
            if (((v53 != -1) && (sub_26090(v14,a0,v24))) && (v53 != v68))
              sub_1ef30(dcgettext(NULL,"warning: daylight saving time changed after time adjustment\n",5));
          }
          goto label_2177a;
        }
      }
      else {
        if (!(char)v91) goto label_21900;
label_21769:
        *a0 = v82;
        a0[1] = v83;
label_2177a:
        if (v6) {
          v43 = "timezone: system default\n";
          if (v46) { // branch-flip
            if (!strcmp(v46,"UTC0")) {
              v43 = "timezone: Universal Time\n";
              goto label_217bc;
            }
            sub_1ef30(dcgettext(NULL,"timezone: TZ=\"%s\" environment value\n",5),v46);
          }
          else {
label_217bc:
            sub_1ef30(dcgettext(NULL,v43,5));
          }
          v39 = *a0;
          v18 = a0[1];
          sub_1ef30(dcgettext(NULL,"final: %ld.%09d (epoch-seconds)\n",5),v39,(int)v18);
          if (gmtime_r(a0,v25)) {
            v11 = sub_1f010(v25,0,v27);
            sub_1ef30(dcgettext(NULL,"final: %s (UTC)\n",5),v11);
          }
          if (sub_26090(v14,a0,v24)) {
            v11 = sub_1ed30(v69,v28);
            v12 = sub_1f010(v24,0,v27);
            sub_1ef30(dcgettext(NULL,"final: %s (UTC%s)\n",5),v12,v11);
          }
          goto label_218b8;
        }
      }
      v8 = 1;
    }
  }
  else {
    v6 = v43[4];
    v13 = &v43[4];
    if (!v6) goto label_214e8;
    v39 = 1;
    v15 = v13;
    v7 = v6;
label_214cb:
    if (v7 == '\\') {
      v21 = &v15[1];
      v1 = &v15[1];
      v15 = v21;
      if ((*v1 != '\\') && (*v1 != '\"')) goto label_214e8;
label_214bb:
      v21 = &v15[1];
      v7 = v15[1];
      v39 += 1;
      v15 = v21;
      if (!v7) goto label_214e8;
      goto label_214cb;
    }
    if (v7 != '\"') goto label_214bb;
    v15 = v29;
    v45 = NULL;
    v46 = v15;
    if ((long)v39 <= 100) goto label_21a18;
    v15 = malloc(v39);
    if (!v15) {
      v45 = NULL;
      v8 = 0;
      free(v45);
      return v8;
    }
    v6 = v43[4];
    v46 = v15;
    v45 = v15;
label_21a18:
    while (v43 = v13, v6 != '\"') {
      v21 = &v15[1];
      v43 = &v43[v6 == '\\'];
      v6 = v43[1];
      *v15 = *v43;
      v13 = &v43[1];
      v15 = v21;
    }
    *v15 = '\0';
    v14 = sub_25d70(v46);
    if (!v14) {
      v8 = 0;
      free(v45);
      return v8;
    }
    do {
      v43 = &v43[1];
      v8 = sub_27f50(*v43);
    } while ((char)v8);
    if (sub_26090(v14,a2,&v23)) goto label_2150f;
  }
label_218b8:
  if (a3 != v14)
    sub_25f40(v14);
  free(v45);
  return v8;
}

// Function: sub_22c40 @ 0x22c40
unsigned int sub_22c40(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  char *v2; // rax
  long v3; // rax
  
  v1 = 0;
  v2 = getenv("TZ");
  v3 = sub_25d70(v2);
  if (v3) {
    v1 = sub_21400(a0,a1,a2,v3,v2);
    sub_25f40(v3);
  }
  return v1;
}

// Function: sub_22cb0 @ 0x22cb0
void sub_22cb0(char *a0)
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
    dat_3eed0 = a0;
    __progname_full = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_22d50 @ 0x22d50
int * sub_22d50(int *a0,int a1)
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

// Function: sub_22da0 @ 0x22da0
bool sub_22da0(long a0,long a1)
{
  return sub_27ff0(a0 + 9,a1 + 9) == 0;
}

// Function: sub_22dc0 @ 0x22dc0
unsigned long sub_22dc0(long a0,unsigned long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_27f70((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_22da0(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_22e20 @ 0x22e20
unsigned long sub_22e20(long a0,unsigned long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_27f70((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_22dc0(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_22ea0 @ 0x22ea0
unsigned long sub_22ea0(long a0,unsigned long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_27f70((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_22e20(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_22f10 @ 0x22f10
unsigned long sub_22f10(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_27f70((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_22ea0(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_22fb0 @ 0x22fb0
unsigned long sub_22fb0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_27f70((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_22f10(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_23050 @ 0x23050
unsigned long sub_23050(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_27f70((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_22fb0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_23110 @ 0x23110
unsigned long sub_23110(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_27f70((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_23050(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_231d0 @ 0x231d0
unsigned long sub_231d0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_27f70((int)(char)a2)) // branch-flip
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
      return sub_23110(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_232b0 @ 0x232b0
unsigned long sub_232b0(unsigned char *a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_27f70((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_231d0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_23370 @ 0x23370
char * sub_23370(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = sub_28680();
    if (sub_232b0(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_232b0(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x30ba0 : (char *)0x30ba3; // branch-flip
    }
  }
  return v1;
}

// Function: sub_234a0 @ 0x234a0
long * sub_234a0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_23510:
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
label_23996:
      v2 = 0;
      goto label_23950;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_23ab1;
      }
      v24 = 1;
      v2 = 0;
      v12 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_23950:
      v24 = 1;
      v12 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_23ab1;
      }
      goto label_23996;
    case 5:
      if (!v17) goto label_238f1;
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
        v9 = (char *)sub_23370("`",a4);
        v36 = (char *)v9;
        v9 = (char *)sub_23370("\'",a4);
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
label_235d3:
  do {
label_235e6:
    v23 = 0;
    a1 = v13;
label_235f0:
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
          if ((a3 < v23 + v30) || (v8 = memcmp(v11,v31,v30), v8)) goto label_23f60;
          if (!v24) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v21 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_23b5c;
                default:
                  goto label_23833;
                case 7:
                  goto label_23b20;
                case 8:
                  goto label_23b05;
                case 9:
                  goto label_23be3;
                case 10:
                  goto label_23afb;
                case 0xb:
                  goto label_23c10;
                case 0xc:
                  goto label_23b47;
                case 0xd:
                  goto label_23a4d;
                case 0x20:
                  goto label_23c1d;
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
                  goto label_237e6;
                case 0x23:
                  goto label_23bc0;
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
                  goto label_23718;
                case 0x27:
                  goto label_23a32;
                case 0x3f:
                  goto label_239f1;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_23833;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_23718;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_24004;
              goto label_23f48;
            }
            if (v7 == 0x7d) goto label_240d0;
            if ((char)v7 <= '}') {
              v21 = 0x7b;
              if (v7 == 0x7b) goto label_23f2a;
              v21 = 0x7c;
              if (v7 == 0x7c) goto label_23f48;
              goto label_23833;
            }
            if (v7 != 0x7e) goto label_23833;
label_24084:
            if (!v23) {
              v7 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_238a9;
            }
            v21 = 0x7e;
            goto label_24092;
          }
          goto label_2381c;
        }
label_23f60:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v21 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v22 = 0;
                if (v7 == 0x7e) goto label_24084;
                goto label_23830;
              }
              v21 = 0x7b;
              v25 = 0;
              if (v7 != 0x7b) {
                v21 = 0x7c;
                v25 = 0;
                v22 = 0;
                if (v7 == 0x7c) goto label_23f48;
                goto label_23833;
              }
            }
            else {
              v21 = 0x7d;
              v25 = 0;
            }
label_23f2a:
            if (a3 == 0xffffffffffffffff) goto label_240df;
label_23f38:
            v22 = v25;
            if (a3 == 1) goto label_23bc5;
          }
          else {
            if (v7 == 0x40) goto label_23830;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v22 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_23718;
            v25 = 0;
            v22 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_24004:
              if ((char)v18 != '\\') goto label_23833;
              if (((bool)(v2 & v24)) && (v30)) goto label_2402a;
              v16 = 0x5c;
              goto label_23a52;
            }
          }
label_23f48:
          v7 = a4 == 2;
          v26 = v3;
          v25 = 0;
          goto label_238a9;
        }
        switch(v7) {
          case 0:
            goto label_23b4e;
          default:
label_23830:
            v25 = 0;
label_23833:
            v16 = (unsigned char)v18;
            v22 = v25;
            if (v33 != 1) goto label_23db3;
label_23840:
            v9 = __ctype_b_loc();
            v21 = 1;
            v26 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v27 = (bool)(v26 & v2);
            v26 = !v26;
            v22 = v25;
            goto label_2388e;
          case 7:
label_23b20:
            v16 = 0x61;
            if (!v24) goto label_23b18;
            goto label_23a7f;
          case 8:
label_23b05:
            v16 = 0x62;
            goto label_23b0a;
          case 9:
            v25 = 0;
label_23be3:
            v21 = 9;
            v16 = 0x74;
            goto label_23bf0;
          case 10:
label_23afb:
            v16 = 0x6e;
            break;
          case 0xb:
label_23c10:
            v16 = 0x76;
            break;
          case 0xc:
label_23b47:
            v16 = 0x66;
label_23b0a:
            if (v24) goto label_23a7f;
label_23b18:
            v25 = 0;
            goto label_2379e;
          case 0xd:
label_23a4d:
            v16 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_23c1d:
            v21 = 0x20;
            goto label_23718;
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
            goto label_237e9;
          case 0x23:
            v21 = 0x23;
            v25 = 0;
            goto label_23bc5;
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
label_23718:
            v7 = 0;
            goto label_23728;
          case 0x27:
            v22 = 0;
            goto label_23a32;
          case 0x3f:
            v22 = 0;
            goto label_239f1;
          
        }
        goto label_23a52;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_240d0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_23f38;
label_240df:
            v22 = v25;
            if (a2[1]) goto label_23f48;
            goto label_23bc5;
          }
          if ((char)v16 <= '}') {
            v21 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_237e6;
              goto label_23833;
            }
            goto label_23f2a;
          }
          v21 = 0x7e;
          if (v16 == 0x7e) goto label_23bc5;
          if (v33 == 1) goto label_23840;
label_23db3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v21 = 0;
          do {
            v18 = v23 + v21;
            v9 = (long)sub_1c4b0(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v25 = v26;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_2431b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_24695;
              goto label_2469f;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_23eed;
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
          goto label_2431b;
        }
        if (v16 == 0x40) goto label_23833;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_238a0;
        if ((unsigned long)v9 & 0xa4000000) goto label_237e6;
        if (v16 != 0x5c) goto label_23833;
        if (a4 == 2) {
          if (!v24) goto label_2402a;
          goto label_23813;
        }
        if (!(bool)(v2 & v24 & v30 != 0)) {
          v21 = 0x5c;
          v16 = 0x5c;
          v25 = 0;
          goto label_23bf0;
        }
label_2402a:
        v23 += 1;
        v25 = 0;
        v16 = 0x5c;
        v7 = v20;
        goto label_24040;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_23cc2;
            goto label_23bfb;
          }
label_23b4e:
          if (!v24) {
            v22 = 0;
label_23b5c:
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
                goto label_238d3;
              }
label_24465:
              a0[v13] = 0x5c;
              v12 = v13;
              v20 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v20;
              goto label_24465;
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
label_2449c:
              v16 = (unsigned char)v21;
              v25 = 0;
              v12 = v13;
              v7 = v26;
              if (!v2 || v27) goto label_238d3;
              goto label_23728;
            }
            v16 = 0x30;
            v25 = 0;
            v12 = v13;
            v7 = v26;
            goto label_238d3;
          }
          if (a4 != 2) goto label_2381c;
          goto label_23a8b;
        default:
          goto label_23833;
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
          goto label_23c7f;
        case 10:
          v21 = 10;
          v16 = 0x6e;
          goto label_23c7f;
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
label_23c7f:
          v7 = a4 == 2 & v24;
          v25 = 0;
          if (!(bool)v7) goto label_23bf0;
label_23801:
          a4 = 2;
label_23813:
          if (!v28) goto label_2381c;
          goto label_23a8b;
        case 0x20:
          v18 = 0x20;
          goto label_237e9;
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
label_237e6:
          v26 = 0;
          goto label_237e9;
        case 0x23:
label_23bc0:
          v21 = 0x23;
label_23bc5:
          v22 = v25;
          if (v23) {
label_24092:
            v7 = a4 == 2;
            v26 = v3;
            v25 = 0;
            goto label_238a9;
          }
          v18 = v21;
label_237e9:
          v21 = v18;
          v22 = v25;
          v25 = v26;
          if ((a4 == 2) && (v24)) goto label_23801;
          goto label_238a7;
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
          goto label_238a0;
        case 0x27:
label_23a32:
          v25 = v26;
          if (a4 != 2) {
            v7 = 0;
            v21 = 0x27;
            goto label_238a9;
          }
          if (!v24) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v32) goto label_244bb;
            }
            else {
label_244bb:
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
            goto label_238d3;
          }
          goto label_23813;
        case 0x3f:
label_239f1:
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
                  goto label_238a9;
                }
                if (v24) goto label_2381c;
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
                goto label_2449c;
              }
            }
            v7 = 0;
            v21 = 0x3f;
            v26 = v3;
            v25 = 0;
            goto label_238a9;
          }
          if (v24) goto label_23813;
          v7 = 0;
          v16 = 0x3f;
          v25 = 0;
          goto label_238d3;
        
      }
label_23bf0:
      v22 = v25;
      if (!v2) {
label_23bfb:
        v16 = (unsigned char)v21;
        v25 = 0;
        v7 = 0;
        v26 = v3;
        if (!v24) goto label_238d0;
        goto label_23728;
      }
label_23a52:
      v25 = 0;
      goto label_23a63;
    }
    if (!(bool)(v12 == 0 & v24 & a4 == 2)) {
      v7 = a4 == 2 & (v24 ^ 1U);
      v24 = (bool)(v24 ^ 1U);
      if ((!(bool)v7) || (v24 = (bool)v7, !v3)) {
label_2471f:
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
        if (a1 || !v32) goto label_2471f;
        v23 = v32;
        goto label_241eb;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v9;
      v31 = "\"";
      if (!(v34 & 2)) goto label_247cf;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v24 = v4;
      goto label_235e6;
    }
label_23eed:
    if (v2) {
label_23a8b:
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
label_23ab1:
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
label_241eb:
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
label_24695:
    if (!a2[(long)v9]) break;
  }
label_2469f:
  v25 = 0;
label_2431b:
  v18 = (unsigned long)v16;
  v26 = v25;
  if (2 <= v21) {
label_24325:
    v19 = 0;
    v21 += v23;
    v13 = v23;
    do {
      v16 = (unsigned char)v18;
      if (v27) {
        v28 = a4 == 2;
        if (v24) goto label_23813;
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
        if (v21 <= v23) goto label_237b1;
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
          goto label_24040;
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
label_2388e:
  if (v27) {
    v25 = 0;
    v27 = v2;
    goto label_24325;
  }
label_238a0:
  v21 = v18;
  v25 = v26;
label_238a7:
  v7 = a4 == 2;
  v26 = v3;
label_238a9:
  v16 = (unsigned char)v21;
  v27 = (bool)v7 == 0;
  v3 = v26;
  v7 = 0;
  if ((v27 && v2) || (v7 = 0, v24)) {
label_23728:
    v16 = (unsigned char)v21;
    v12 = v13;
    if (!v29) goto label_238d3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_238d3;
  }
  else {
label_238d0:
    v7 = 0;
    v3 = v26;
label_238d3:
    if (!v22) {
      v7 ^= 1;
      v23 += 1;
      v7 &= v20;
label_24040:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v20 = 0;
        v12 += 2;
      }
      goto label_237b1;
    }
  }
label_23a63:
  if (v24) {
    v24 = v2;
label_23a7f:
    if ((bool)(v24 & a4 == 2)) goto label_23a8b;
label_2381c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_23510;
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
label_2379e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v23 += 1;
label_237b1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v25)
    v4 = 0;
  goto label_235f0;
label_23cc2:
  v23 += 1;
  goto label_235f0;
label_247cf:
  a1 = v32;
label_238f1:
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
  goto label_235d3;
}

// Function: sub_248d0 @ 0x248d0
void * sub_248d0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_3e500;
    if (dat_3e4f8 <= (int)a0) {
      v10 = (long)dat_3e4f8;
      v9 = (long)(int)((a0 - dat_3e4f8) + 1);
      if (dat_3e500 != (unsigned long *)0x3e510) // branch-flip
        v6 = (unsigned long *)sub_26b50(dat_3e500,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_26b50(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_3e510;
        v6[1] = dat_3e518;
      }
      dat_3e500 = v6;
      memset(&v6[(long)dat_3e4f8 * 2],0,(v10 - dat_3e4f8) * 0x10);
      dat_3e4f8 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_234a0(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x3eee0)
        free(v8);
      v8 = (void *)sub_269d0(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_234a0(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_24ac0 @ 0x24ac0
void sub_24ac0(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x3efe0;
  sub_26c90(a0,0x38);
  *v2 = v1;
}

// Function: sub_24b00 @ 0x24b00
unsigned int sub_24b00(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x3efe0;
  return *a0;
}

// Function: sub_24b20 @ 0x24b20
void sub_24b20(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x3efe0;
  *a0 = a1;
}

// Function: sub_24b40 @ 0x24b40
unsigned int sub_24b40(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x3efe0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_24b80 @ 0x24b80
unsigned int sub_24b80(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x3efe0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_24ba0 @ 0x24ba0
void sub_24ba0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x3efe0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_24be0 @ 0x24be0
void sub_24be0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x3efe0;
  v2 = __errno_location();
  v1 = *v2;
  sub_234a0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_24c60 @ 0x24c60
unsigned long sub_24c60(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x3efe0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_234a0(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_269d0(v3 + 1);
  sub_234a0(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_24d50 @ 0x24d50
void sub_24d50(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_24c60(a0,a1,0,a2); // tail-call
}

// Function: sub_24d60 @ 0x24d60
void sub_24d60(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_3e500;
  if (2 <= dat_3e4f8) {
    v4 = (unsigned long *)((long)dat_3e500 + 0x18);
    v1 = (unsigned long *)((long)dat_3e500 + (unsigned long)(unsigned int)(dat_3e4f8 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x3eee0) {
    free(*(void **)((long)v3 + 8));
    dat_3e518 = 0x3eee0;
    dat_3e510 = 0x100;
  }
  if (v3 == (void *)0x3e510) {
    dat_3e4f8 = 1;
    return;
  }
  free(v3);
  dat_3e4f8 = 1;
  dat_3e500 = (void *)0x3e510;
}

// Function: sub_24e00 @ 0x24e00
void sub_24e00(unsigned long a0,unsigned long a1)
{
  sub_248d0(a0,a1,0xffffffffffffffff,0x3efe0); // tail-call
}

// Function: sub_24e20 @ 0x24e20
void sub_24e20(void)
{
  sub_248d0(); // tail-call
}

// Function: sub_24e30 @ 0x24e30
void sub_24e30(unsigned long a0)
{
  sub_24e00(0,a0); // tail-call
}

// Function: sub_24e40 @ 0x24e40
void sub_24e40(unsigned long a0,unsigned long a1)
{
  sub_24e20(0,a0,a1); // tail-call
}

// Function: sub_24e50 @ 0x24e50
void sub_24e50(unsigned int a0,unsigned long a1,unsigned long a2)
{
  char v1 [56];
  
  sub_22d50(v1);
  sub_248d0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_24eb0 @ 0x24eb0
void sub_24eb0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_22d50(v1);
  sub_248d0(a0,a2,a3,v1);
}

// Function: sub_24f20 @ 0x24f20
void sub_24f20(unsigned int a0,unsigned long a1)
{
  sub_24e50(0,a0,a1); // tail-call
}

// Function: sub_24f30 @ 0x24f30
void sub_24f30(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_24eb0(0,a0,a1,a2); // tail-call
}

// Function: sub_24f50 @ 0x24f50
void sub_24f50(unsigned long a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_3efe0;
  v3 = dat_3efe8;
  v4 = dat_3eff0;
  v5 = dat_3eff8;
  v8 = dat_3f010;
  v6 = dat_3f000;
  v7 = dat_3f008;
  sub_24b40(&v1,(int)a2,1,a3,a0,a1);
  sub_248d0(0,a0,a1,v2);
}

// Function: sub_24fe0 @ 0x24fe0
void sub_24fe0(unsigned long a0,char a1)
{
  sub_24f50(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_25000 @ 0x25000
void sub_25000(unsigned long a0)
{
  sub_24fe0(a0,0x3a); // tail-call
}

// Function: sub_25010 @ 0x25010
void sub_25010(unsigned long a0,unsigned long a1)
{
  sub_24f50(a0,a1,0x3a); // tail-call
}

// Function: sub_25020 @ 0x25020
void sub_25020(unsigned int a0,unsigned long a1,unsigned long a2)
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
  
  sub_22d50(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_24b40(v3,0x3a,1);
  sub_248d0(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_250c0 @ 0x250c0
void sub_250c0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_3efe0;
  v2 = dat_3efe8;
  v7 = dat_3f010;
  v3 = dat_3eff0;
  v4 = dat_3eff8;
  v5 = dat_3f000;
  v6 = dat_3f008;
  sub_24ba0(&v1);
  sub_248d0(a0,a3,a4,&v1);
}

// Function: sub_25160 @ 0x25160
void sub_25160(void)
{
  sub_250c0(); // tail-call
}

// Function: sub_25170 @ 0x25170
void sub_25170(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_25160(0,a0,a1,a2); // tail-call
}

// Function: sub_25190 @ 0x25190
void sub_25190(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_250c0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_251b0 @ 0x251b0
void sub_251b0(void)
{
  sub_248d0(); // tail-call
}

// Function: sub_251c0 @ 0x251c0
void sub_251c0(unsigned long a0,unsigned long a1)
{
  sub_251b0(0,a0,a1); // tail-call
}

// Function: sub_251d0 @ 0x251d0
void sub_251d0(unsigned long a0,unsigned long a1)
{
  sub_251b0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_251e0 @ 0x251e0
void sub_251e0(unsigned long a0)
{
  sub_251d0(0,a0); // tail-call
}

// Function: sub_251f0 @ 0x251f0
int sub_251f0(int *a0)
{
  int v1; // eax
  char *v2; // rax
  
  a0[2] = 0;
  a0[3] = 0;
  v1 = sub_17b20(".",0x80000);
  *a0 = v1;
  if (0 <= v1)
    return 0;
  v2 = getcwd(NULL,0);
  *(char **)&a0[2] = v2;
  return -(unsigned int)(v2 == NULL);
}

// Function: sub_25250 @ 0x25250
void sub_25250(int *a0)
{
  if (0 <= *a0) {
    fchdir(*a0); // tail-call
    return;
  }
  sub_28110(*(unsigned long *)&a0[2]); // tail-call
}

// Function: sub_25280 @ 0x25280
void sub_25280(int *a0)
{
  if (0 <= *a0)
    close(*a0);
  free(*(void **)&a0[2]); // tail-call
}

// Function: sub_252a0 @ 0x252a0
unsigned long sub_252a0(int a0,char *a1,unsigned long a2)
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
    return sub_25ba0(a1,a2); // tail-call
  v4 = (char *)sub_27b90(v7,a0,a1);
  if (v4) {
    v2 = sub_25ba0(v4,a2);
    v5 = (unsigned int *)__errno_location();
    v3 = *v5;
    if (v4 != v7)
      free(v4);
    if (v2 != 0xffffffff)
      return (unsigned long)v2;
    if (0x27 <= v3) { // branch-flip
      if (v3 != 0x5f) goto label_253b9;
    }
    else if (0xffffffbfffefdff9U >> ((unsigned long)v3 & 0x3f) & 1) {
label_253b9:
      *v5 = v3;
      return 0xffffffff;
    }
  }
  if (sub_251f0(v8))
    sub_1e610(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v9 = 0xffffffff;
    sub_25280(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v9 = 0xffffffff;
    v1 = *v6;
    sub_25280(v8);
    *v6 = v1;
  }
  else {
    v3 = sub_25ba0(a1,a2);
    v9 = (unsigned long)v3;
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_25250(v8))
        sub_1e650(*__errno_location()); // return-dupe, no-return
      sub_25280(v8);
    }
    else {
      v6 = __errno_location();
      v1 = *v6;
      if (sub_25250(v8)) {
        sub_1e650(*__errno_location());
      }
      sub_25280(v8);
      if (v1)
        *v6 = v1;
    }
  }
  return v9;
}

// Function: sub_254c0 @ 0x254c0
unsigned long sub_254c0(int a0,char *a1,unsigned long a2)
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
    return sub_25bc0(a1,a2); // tail-call
  v4 = (char *)sub_27b90(v7,a0,a1);
  if (v4) {
    v2 = sub_25bc0(v4,a2);
    v5 = (unsigned int *)__errno_location();
    v3 = *v5;
    if (v4 != v7)
      free(v4);
    if (v2 != 0xffffffff)
      return (unsigned long)v2;
    if (0x27 <= v3) { // branch-flip
      if (v3 != 0x5f) goto label_255d9;
    }
    else if (0xffffffbfffefdff9U >> ((unsigned long)v3 & 0x3f) & 1) {
label_255d9:
      *v5 = v3;
      return 0xffffffff;
    }
  }
  if (sub_251f0(v8))
    sub_1e610(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v9 = 0xffffffff;
    sub_25280(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v9 = 0xffffffff;
    v1 = *v6;
    sub_25280(v8);
    *v6 = v1;
  }
  else {
    v3 = sub_25bc0(a1,a2);
    v9 = (unsigned long)v3;
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_25250(v8))
        sub_1e650(*__errno_location()); // return-dupe, no-return
      sub_25280(v8);
    }
    else {
      v6 = __errno_location();
      v1 = *v6;
      if (sub_25250(v8)) {
        sub_1e650(*__errno_location());
      }
      sub_25280(v8);
      if (v1)
        *v6 = v1;
    }
  }
  return v9;
}

// Function: sub_256e0 @ 0x256e0
unsigned long sub_256e0(int a0,char *a1,unsigned long a2)
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
    return setfilecon(a1,a2); // tail-call
  v4 = (char *)sub_27b90(v7,a0,a1);
  if (v4) {
    v2 = setfilecon(v4,a2);
    v5 = (unsigned int *)__errno_location();
    v3 = *v5;
    if (v4 != v7)
      free(v4);
    if (v2 != 0xffffffff)
      return (unsigned long)v2;
    if (0x27 <= v3) { // branch-flip
      if (v3 != 0x5f) goto label_257f9;
    }
    else if (0xffffffbfffefdff9U >> ((unsigned long)v3 & 0x3f) & 1) {
label_257f9:
      *v5 = v3;
      return 0xffffffff;
    }
  }
  if (sub_251f0(v8))
    sub_1e610(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v9 = 0xffffffff;
    sub_25280(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v9 = 0xffffffff;
    v1 = *v6;
    sub_25280(v8);
    *v6 = v1;
  }
  else {
    v3 = setfilecon(a1,a2);
    v9 = (unsigned long)v3;
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_25250(v8))
        sub_1e650(*__errno_location()); // return-dupe, no-return
      sub_25280(v8);
    }
    else {
      v6 = __errno_location();
      v1 = *v6;
      if (sub_25250(v8)) {
        sub_1e650(*__errno_location());
      }
      sub_25280(v8);
      if (v1)
        *v6 = v1;
    }
  }
  return v9;
}

// Function: sub_25900 @ 0x25900
unsigned long sub_25900(int a0,char *a1,unsigned long a2)
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
    return lsetfilecon(a1,a2); // tail-call
  v4 = (char *)sub_27b90(v7,a0,a1);
  if (v4) {
    v2 = lsetfilecon(v4,a2);
    v5 = (unsigned int *)__errno_location();
    v3 = *v5;
    if (v4 != v7)
      free(v4);
    if (v2 != 0xffffffff)
      return (unsigned long)v2;
    if (0x27 <= v3) { // branch-flip
      if (v3 != 0x5f) goto label_25a19;
    }
    else if (0xffffffbfffefdff9U >> ((unsigned long)v3 & 0x3f) & 1) {
label_25a19:
      *v5 = v3;
      return 0xffffffff;
    }
  }
  if (sub_251f0(v8))
    sub_1e610(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v9 = 0xffffffff;
    sub_25280(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v9 = 0xffffffff;
    v1 = *v6;
    sub_25280(v8);
    *v6 = v1;
  }
  else {
    v3 = lsetfilecon(a1,a2);
    v9 = (unsigned long)v3;
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_25250(v8))
        sub_1e650(*__errno_location()); // return-dupe, no-return
      sub_25280(v8);
    }
    else {
      v6 = __errno_location();
      v1 = *v6;
      if (sub_25250(v8)) {
        sub_1e650(*__errno_location());
      }
      sub_25280(v8);
      if (v1)
        *v6 = v1;
    }
  }
  return v9;
}

// Function: sub_25b20 @ 0x25b20
int sub_25b20(int a0,unsigned long *a1)
{
  char *v1;
  
  if (a0) { // branch-flip
    if (a0 == 10) {
      v1 = (char *)*a1;
      if (!strcmp(v1,"unlabeled")) {
        a0 = -1;
        freecon(v1);
        *a1 = 0;
        *__errno_location() = 0x3d;
      }
    }
  }
  else {
    a0 = -1;
    *__errno_location() = 0x5f;
  }
  return a0;
}

// Function: sub_25ba0 @ 0x25ba0
void sub_25ba0(unsigned long a0,unsigned long a1)
{
  sub_25b20(getfilecon(),a1); // tail-call
}

// Function: sub_25bc0 @ 0x25bc0
void sub_25bc0(unsigned long a0,unsigned long a1)
{
  sub_25b20(lgetfilecon(),a1); // tail-call
}

// Function: sub_25be0 @ 0x25be0
void sub_25be0(unsigned long a0,unsigned long a1)
{
  sub_25b20(fgetfilecon(),a1); // tail-call
}

// Function: sub_25c00 @ 0x25c00
unsigned long sub_25c00(long a0)
{
  return *(unsigned long *)(a0 + 0x50);
}

// Function: sub_25c10 @ 0x25c10
unsigned long sub_25c10(long a0)
{
  return *(unsigned long *)(a0 + 0x70);
}

// Function: sub_25c20 @ 0x25c20
unsigned long sub_25c20(long a0)
{
  return *(unsigned long *)(a0 + 0x60);
}

// Function: sub_25c30 @ 0x25c30
unsigned long sub_25c30(void)
{
  return 0;
}

// Function: sub_25c40 @ 0x25c40
undefined16 sub_25c40(long a0)
{
  return *(char (*)[16])(a0 + 0x48);
}

// Function: sub_25c50 @ 0x25c50
undefined16 sub_25c50(long a0)
{
  return *(char (*)[16])(a0 + 0x68);
}

// Function: sub_25c60 @ 0x25c60
undefined16 sub_25c60(long a0)
{
  return *(char (*)[16])(a0 + 0x58);
}

// Function: sub_25c70 @ 0x25c70
undefined16 sub_25c70(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_25c90 @ 0x25c90
unsigned int sub_25c90(unsigned int a0)
{
  return a0;
}

// Function: sub_25ca0 @ 0x25ca0
void * sub_25ca0(void *a0,unsigned long a1)
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

// Function: sub_25ce0 @ 0x25ce0
void sub_25ce0(void *a0,void *a1,unsigned long a2)
{
  *(char *)((long)memcpy(a0,a1,a2) + a2) = 0;
}

// Function: sub_25cf0 @ 0x25cf0
void sub_25cf0(void)
{
  getenv("TZ"); // tail-call
}

// Function: sub_25d00 @ 0x25d00
void sub_25d00(char *a0)
{
  if (a0) {
    setenv("TZ",a0,1); // tail-call
    return;
  }
  unsetenv("TZ"); // tail-call
}

// Function: sub_25d30 @ 0x25d30
bool sub_25d30(long a0)
{
  long v1;
  bool v2; // zf
  
  v1 = a0 + 9;
  if (!*(char *)(a0 + 8))
    v1 = 0;
  v2 = sub_25d00(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_25d70 @ 0x25d70
unsigned long * sub_25d70(char *a0)
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
      sub_25ce0((long)v3 + 9,a0,v1);
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

// Function: sub_25e10 @ 0x25e10
unsigned long sub_25e10(long *a0,char *a1) // return-dupe x2
{
  char *v1;
  long *v2;
  int v3; // eax
  long v4; // rax
  char *v5;
  
  v1 = *(char **)&a1[0x30];
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[0x38]))
    return 1;
  v5 = (char *)((long)a0 + 9);
  if (!*v1) {
    v5 = "";
label_25ea8:
    *(char **)&a1[0x30] = v5;
    return 1;
  }
  do {
    v3 = strcmp(v5,v1);
    while( true ) {
      if (!v3) goto label_25ea8;
      if ((!*v5) && ((v5 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        if ((long)a0 + (0x80U - (long)v5) <= (long)(strlen(v1) + 1)) { // branch-flip
          v4 = sub_25d70(v1);
          *a0 = v4;
          if (!v4)
            return 0;
          *(char *)(v4 + 8) = 0;
          v5 = (char *)(v4 + 9);
        }
        else {
          sub_25ce0(v5,v1);
        }
        goto label_25ea8;
      }
      v5 = &v5[strlen(v5) + 1];
      if ((*v5) || (v2 = (long *)*a0, !v2)) break;
      v5 = (char *)((long)v2 + 9);
      v3 = strcmp(v5,v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_25f40 @ 0x25f40
void sub_25f40(long *a0) // return-dupe
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

// Function: sub_25f80 @ 0x25f80
long sub_25f80(long a0)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  long v5;
  
  v2 = (char *)sub_25cf0();
  if (v2) { // branch-flip
    if (*(char *)(a0 + 8)) {
      if (!strcmp((char *)(a0 + 9),v2))
        return 1;
    }
  }
  else if (!*(char *)(a0 + 8))
    return 1;
  v3 = sub_25d70(v2);
  v5 = v3;
  if (v3) {
    if (!sub_25d30(a0)) {
      v4 = __errno_location();
      v5 = 0;
      v1 = *v4;
      sub_25f40(v3);
      *v4 = v1;
    }
  }
  return v5;
}

// Function: sub_26030 @ 0x26030
unsigned int sub_26030(long a0)
{
  unsigned int v1; // eax
  int *v2; // rax
  int v3;
  
  v1 = 1;
  if (a0 != 1) {
    v2 = __errno_location();
    v3 = *v2;
    v1 = sub_25d30(a0);
    if (!(char)v1)
      v3 = *v2;
    sub_25f40(a0);
    *v2 = v3;
  }
  return v1;
}

// Function: sub_26090 @ 0x26090
void * sub_26090(long a0,void *a1,void *a2) // return-dupe
{
  long v1; // rax
  
  if (!a0)
    return (void *)gmtime_r(a1,a2); // tail-call
  v1 = sub_25f80(a0);
  if (!v1)
    return NULL;
  if ((localtime_r(a1,a2)) && (sub_25e10(a0,a2))) {
    if (!sub_26030(v1))
      return NULL;
    return a2;
  }
  sub_26030(v1);
  return NULL;
}

// Function: sub_26130 @ 0x26130
unsigned long sub_26130(long a0,unsigned long *a1) // return-dupe x2
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
  v1 = sub_25f80(a0);
  if (!v1)
    return 0xffffffffffffffff;
  v3 = *a1;
  v7 = -1;
  v4 = a1[1];
  v5 = a1[2];
  v8 = *(unsigned int *)&a1[4];
  v2 = mktime(&v3);
  if ((0 <= v7) && (sub_25e10(a0,&v3))) {
    if (!sub_26030(v1))
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
  sub_26030(v1);
  return 0xffffffffffffffff;
}

// Function: sub_26260 @ 0x26260
void sub_26260(unsigned long a0)
{
  sub_179d0(a0,0,3); // tail-call
}

// Function: sub_26280 @ 0x26280
unsigned int sub_26280(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_26260();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_262e0 @ 0x262e0
long sub_262e0(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_2640d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_26446:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x31280) + 0x31280;
  switch(a5) {
    case 0:
      goto label_26446;
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
      goto label_2640d;
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

// Function: sub_26760 @ 0x26760
void sub_26760(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_262e0(); // tail-call
}

// Function: sub_26780 @ 0x26780
void sub_26780(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_262e0(a0,a1,a2,a3);
}

// Function: sub_26820 @ 0x26820
void sub_26820(void)
{
  sub_26780();
}

// Function: sub_268d0 @ 0x268d0
void sub_268d0(void)
{
  fputs("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-findutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU findutils","http://www.gnu.org/software/findutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_26980 @ 0x26980
long sub_26980(long a0)
{
  if (a0)
    return a0;
  sub_26d50(); // no-return
}

// Function: sub_26990 @ 0x26990
void sub_26990(unsigned long a0)
{
  sub_26980(malloc(a0)); // tail-call
}

// Function: sub_269b0 @ 0x269b0
void sub_269b0(void)
{
  sub_26980(sub_28610()); // tail-call
}

// Function: sub_269d0 @ 0x269d0
void sub_269d0(void)
{
  sub_26990(); // tail-call
}

// Function: sub_269e0 @ 0x269e0
void sub_269e0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_26d50(); // no-return
}

// Function: sub_26a20 @ 0x26a20
void sub_26a20(void)
{
  sub_26980(sub_28620()); // tail-call
}

// Function: sub_26a40 @ 0x26a40
void sub_26a40(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_26d50(); // no-return
}

// Function: sub_26a80 @ 0x26a80
void sub_26a80(void)
{
  sub_26a40(); // tail-call
}

// Function: sub_26a90 @ 0x26a90
void sub_26a90(void)
{
  sub_26980(sub_28650()); // tail-call
}

// Function: sub_26ab0 @ 0x26ab0
void sub_26ab0(unsigned long a0,unsigned long a1)
{
  sub_26a40(0,a0,a1); // tail-call
}

// Function: sub_26ac0 @ 0x26ac0
void sub_26ac0(unsigned long a0,unsigned long a1)
{
  sub_26a90(0,a0,a1); // tail-call
}

// Function: sub_26ad0 @ 0x26ad0
void sub_26ad0(long a0,unsigned long *a1,unsigned long a2)
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
      sub_26d50(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_26a40(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_26b40 @ 0x26b40
void sub_26b40(unsigned long a0,unsigned long a1)
{
  sub_26ad0(a0,a1,1); // tail-call
}

// Function: sub_26b50 @ 0x26b50
void sub_26b50(long a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_26bf6;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_26bf6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_26d50(a0,v3); // no-return
  sub_269e0(a0,v3);
  *a1 = v2;
}

// Function: sub_26c30 @ 0x26c30
void sub_26c30(unsigned long a0,unsigned long a1)
{
  sub_26980(calloc(a0,a1)); // tail-call
}

// Function: sub_26c50 @ 0x26c50
void sub_26c50(unsigned long a0)
{
  sub_26c30(a0,1); // tail-call
}

// Function: sub_26c60 @ 0x26c60
void sub_26c60(void)
{
  sub_26980(sub_28640()); // tail-call
}

// Function: sub_26c80 @ 0x26c80
void sub_26c80(unsigned long a0)
{
  sub_26c60(a0,1); // tail-call
}

// Function: sub_26c90 @ 0x26c90
void sub_26c90(void *a0,unsigned long a1)
{
  memcpy((void *)sub_26990(a1),a0,a1); // tail-call
}

// Function: sub_26cc0 @ 0x26cc0
void sub_26cc0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_269b0(a1),a0,a1); // tail-call
}

// Function: sub_26cf0 @ 0x26cf0
void sub_26cf0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_269b0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_26d30 @ 0x26d30
void sub_26d30(char *a0)
{
  sub_26c90(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_26d50 @ 0x26d50
void sub_26d50(void)
{
  error(dat_3e4b8,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_26d90 @ 0x26d90
bool sub_26d90(char *a0,unsigned long *a1,double *a2,void *a3)
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
      if ((*v2) || (v3 = 1, v4 == dat_2b968)) goto label_26e05;
label_26df3:
      v3 = *v1 != 0x22;
      goto label_26dfc;
    }
    v3 = 1;
    if (v4 != dat_2b968) goto label_26df3;
  }
  else {
    v3 = 0;
label_26dfc:
    if (!a1) goto label_26e05;
  }
  *a1 = v2;
label_26e05:
  *a2 = v4;
  return v3;
}

// Function: sub_26e60 @ 0x26e60
bool sub_26e60(unsigned long *a0,int a1)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3; // rax
  bool v4; // cf
  
  v1._8_8_ = 0;
  v1._0_8_ = (long)a1;
  v2._8_8_ = 0;
  v2._0_8_ = *a0;
  v3 = SUB168(v1._0_16_ * v2._0_16_,0);
  v4 = SUB168(v1._0_16_ * v2._0_16_,8) != 0;
  if (v4)
    v3 = 0xffffffffffffffff;
  *a0 = v3;
  return v4;
}

// Function: sub_26e90 @ 0x26e90
unsigned int sub_26e90(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_26e60();
  } while (a2);
  return v1;
}

// Function: sub_26ec0 @ 0x26ec0
unsigned int sub_26ec0(unsigned char *a0,char **a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned int v10;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned char *v4;
  unsigned char v5;
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // ecx
  unsigned long v9; // r8
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoul"); // no-return
  if (!a1)
    a1 = &v6;
  v3 = __errno_location();
  *v3 = 0;
  v5 = *a0;
  v4 = a0;
  while (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v5 * 2) & 0x20) {
    v5 = v4[1];
    v4 = &v4[1];
  }
  if (v5 == 0x2d)
    return 4;
  v7 = strtoul((char *)a0,a1,a2);
  v4 = (unsigned char *)*a1;
  if (v4 != a0) { // branch-flip
    if (*v3) { // branch-flip
      v10 = 1;
      if (*v3 != 0x22)
        return 4;
    }
    else {
      v10 = 0;
    }
    if (!a4) {
      *a3 = v7;
      return v10;
    }
    v5 = *v4;
    if (!v5) goto label_26f82;
    v8 = (int)(char)v5;
  }
  else {
    if ((!a4) || (v5 = *a0, !v5))
      return 4;
    v8 = (int)(char)v5;
    if (!strchr(a4,v8))
      return 4;
    v7 = 1;
    v10 = 0;
  }
  if (!strchr(a4,v8)) {
label_26f89:
    *a3 = v7;
    return v10 | 2;
  }
  v2 = (unsigned int)v5 - 0x45;
  v8 = 1;
  v9 = 0x400;
  if (((unsigned char)v2 <= 0x2f) && (0x814400308945U >> ((unsigned long)v2 & 0x3f) & 1)) {
    v9 = 0x400;
    v8 = 1;
    if (strchr(a4,0x30)) {
      v1 = v4[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v8 = (v1 == 0x42) + 1;
          v9 = 0x400;
          if (v1 == 0x42)
            v9 = 1000;
        }
        else {
          v8 = (v4[2] == 0x42) + 1 + (unsigned int)(v4[2] == 0x42);
        }
      }
      else {
        v8 = 2;
        v9 = 1000;
      }
    }
  }
  switch((unsigned int)v5 - 0x42 & 0xff) {
    case 0:
      v10 |= sub_26e60(&v7,0x400);
      break;
    default:
      goto label_26f89;
    case 3:
      v10 |= sub_26e90(&v7,v9,6);
      break;
    case 5:
    case 0x25:
      v10 |= sub_26e90(&v7,v9,3);
      break;
    case 9:
    case 0x29:
      v10 |= sub_26e90(&v7,v9,1);
      break;
    case 0xb:
    case 0x2b:
      v10 |= sub_26e90(&v7,v9,2);
      break;
    case 0xe:
      v10 |= sub_26e90(&v7,v9,5);
      break;
    case 0x12:
    case 0x32:
      v10 |= sub_26e90(&v7,v9,4);
      break;
    case 0x17:
      v10 |= sub_26e90(&v7,v9,8);
      break;
    case 0x18:
      v10 |= sub_26e90(&v7,v9,7);
      break;
    case 0x20:
      v10 |= sub_26e60(&v7,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v10 |= sub_26e60(&v7,2);
    
  }
  *a1 = (char *)&v4[v8];
  if (v4[v8])
    v10 |= 2;
label_26f82:
  *a3 = v7;
  return v10;
}

// Function: sub_272a0 @ 0x272a0
bool sub_272a0(unsigned long *a0,int a1)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3; // rax
  bool v4; // cf
  
  v1._8_8_ = 0;
  v1._0_8_ = (long)a1;
  v2._8_8_ = 0;
  v2._0_8_ = *a0;
  v3 = SUB168(v1._0_16_ * v2._0_16_,0);
  v4 = SUB168(v1._0_16_ * v2._0_16_,8) != 0;
  if (v4)
    v3 = 0xffffffffffffffff;
  *a0 = v3;
  return v4;
}

// Function: sub_272d0 @ 0x272d0
unsigned int sub_272d0(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_272a0();
  } while (a2);
  return v1;
}

// Function: sub_27300 @ 0x27300
unsigned int sub_27300(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned int v10;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned char *v4;
  unsigned char v5;
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // ecx
  unsigned long v9; // r8
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v6;
  v3 = __errno_location();
  *v3 = 0;
  v5 = *a0;
  v4 = a0;
  while (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v5 * 2) & 0x20) {
    v5 = v4[1];
    v4 = &v4[1];
  }
  if (v5 == 0x2d)
    return 4;
  v7 = strtoumax(a0,a1,a2);
  v4 = (unsigned char *)*a1;
  if (v4 != a0) { // branch-flip
    if (*v3) { // branch-flip
      v10 = 1;
      if (*v3 != 0x22)
        return 4;
    }
    else {
      v10 = 0;
    }
    if (!a4) {
      *a3 = v7;
      return v10;
    }
    v5 = *v4;
    if (!v5) goto label_273c2;
    v8 = (int)(char)v5;
  }
  else {
    if ((!a4) || (v5 = *a0, !v5))
      return 4;
    v8 = (int)(char)v5;
    if (!strchr(a4,v8))
      return 4;
    v7 = 1;
    v10 = 0;
  }
  if (!strchr(a4,v8)) {
label_273c9:
    *a3 = v7;
    return v10 | 2;
  }
  v2 = (unsigned int)v5 - 0x45;
  v8 = 1;
  v9 = 0x400;
  if (((unsigned char)v2 <= 0x2f) && (0x814400308945U >> ((unsigned long)v2 & 0x3f) & 1)) {
    v9 = 0x400;
    v8 = 1;
    if (strchr(a4,0x30)) {
      v1 = v4[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v8 = (v1 == 0x42) + 1;
          v9 = 0x400;
          if (v1 == 0x42)
            v9 = 1000;
        }
        else {
          v8 = (v4[2] == 0x42) + 1 + (unsigned int)(v4[2] == 0x42);
        }
      }
      else {
        v8 = 2;
        v9 = 1000;
      }
    }
  }
  switch((unsigned int)v5 - 0x42 & 0xff) {
    case 0:
      v10 |= sub_272a0(&v7,0x400);
      break;
    default:
      goto label_273c9;
    case 3:
      v10 |= sub_272d0(&v7,v9,6);
      break;
    case 5:
    case 0x25:
      v10 |= sub_272d0(&v7,v9,3);
      break;
    case 9:
    case 0x29:
      v10 |= sub_272d0(&v7,v9,1);
      break;
    case 0xb:
    case 0x2b:
      v10 |= sub_272d0(&v7,v9,2);
      break;
    case 0xe:
      v10 |= sub_272d0(&v7,v9,5);
      break;
    case 0x12:
    case 0x32:
      v10 |= sub_272d0(&v7,v9,4);
      break;
    case 0x17:
      v10 |= sub_272d0(&v7,v9,8);
      break;
    case 0x18:
      v10 |= sub_272d0(&v7,v9,7);
      break;
    case 0x20:
      v10 |= sub_272a0(&v7,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v10 |= sub_272a0(&v7,2);
    
  }
  *a1 = &v4[v8];
  if (v4[v8])
    v10 |= 2;
label_273c2:
  *a3 = v7;
  return v10;
}

// Function: sub_276e0 @ 0x276e0
bool sub_276e0(void)
{
  int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x18
  char *v4; // stack - 0x20
  bool v5;
  
  v5 = 0;
  v4 = NULL;
  v3 = 0;
  v2 = getline(&v4,&v3,stdin);
  if (1 <= v2) {
    if (v4[v2 + -1] == '\n')
      v4[v2 + -1] = '\0';
    v1 = rpmatch(v4);
    v5 = 0 < v1;
  }
  free(v4);
  return v5;
}

// Function: sub_27780 @ 0x27780
void sub_27780(void)
{
  exit(dat_3e4b8); // no-return
}

// Function: sub_277a0 @ 0x277a0
unsigned long sub_277a0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_278b0 @ 0x278b0
long sub_278b0(char *a0,long *a1) // return-dupe
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

// Function: sub_27910 @ 0x27910
void sub_27910(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_251d0(1,a0);
  error(0,0,v1,sub_24e50(0,8,a1),v2); // tail-call
}

// Function: sub_279a0 @ 0x279a0
void sub_279a0(long *a0,void *a1,unsigned long a2)
{
  long v1;
  void *v2;
  long v3;
  void *v4;
  
  v2 = stderr;
  v4 = NULL;
  fputs(dcgettext(NULL,"Valid arguments are:",5),v2);
  v3 = 0;
  v1 = *a0;
  while (v1) {
    if ((v3) && (!memcmp(v4,a1,a2)))
      __fprintf_chk(stderr,1,", %s",sub_251e0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_251e0(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v3];
  }
  putc(10,stderr); // tail-call
}

// Function: sub_27aa0 @ 0x27aa0
long sub_27aa0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_277a0(a1,a2,a3,a4);
  else {
    v1 = sub_278b0(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_27910(a0,a1,v1);
  sub_279a0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_27b30 @ 0x27b30
long sub_27b30(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_27b90 @ 0x27b90
char * sub_27b90(char *a0,unsigned int a1,char *a2) // early-return, return-dupe
{
  unsigned long v1;
  unsigned int v2; // eax
  char v3 [40];
  
  if (!*a2) {
    *a0 = '\0';
    return a0;
  }
  if (!dat_3f018) {
    v2 = open("/proc/self/fd",0x90900);
    if ((int)v2 <= -1) {
      dat_3f018 = 0xffffffff;
      return NULL;
    }
    __sprintf_chk(v3,1,0x20,"/proc/self/fd/%d/../fd",(unsigned long)v2);
    dat_3f018 = (-(unsigned int)(access(v3,0) == 0) & 2) - 1;
    close(v2);
  }
  if (dat_3f018 <= -1)
    return NULL;
  v1 = strlen(a2) + 0x1b;
  if ((0xfc1 <= v1) && (a0 = malloc(v1), !a0))
    return NULL;
  strcpy(&a0[__sprintf_chk(a0,1,0xffffffffffffffff,"/proc/self/fd/%d/",(unsigned long)a1)],a2);
  return a0;
}

// Function: sub_27cf0 @ 0x27cf0
char * sub_27cf0(char *a0)
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

// Function: sub_27d50 @ 0x27d50
void sub_27d50(char *a0)
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

// Function: sub_27d80 @ 0x27d80
unsigned long sub_27d80(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_27d90 @ 0x27d90
unsigned long sub_27d90(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_27da0 @ 0x27da0
unsigned int sub_27da0(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_27db0 @ 0x27db0
unsigned int sub_27db0(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_27dc0 @ 0x27dc0
void sub_27dc0(void)
{
  sub_27d80(); // tail-call
}

// Function: sub_27dd0 @ 0x27dd0
void sub_27dd0(void)
{
  sub_27d90(); // tail-call
}

// Function: sub_27de0 @ 0x27de0
unsigned int sub_27de0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_27df0 @ 0x27df0
unsigned int sub_27df0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_27e00 @ 0x27e00
unsigned int sub_27e00(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_27e10 @ 0x27e10
unsigned int sub_27e10(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_27e20 @ 0x27e20
unsigned long sub_27e20(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_27e50 @ 0x27e50
bool sub_27e50(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_27e70 @ 0x27e70
bool sub_27e70(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_27e80 @ 0x27e80
unsigned int sub_27e80(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_27ea0 @ 0x27ea0
unsigned long sub_27ea0(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_27ec0 @ 0x27ec0
bool sub_27ec0(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_27ed0 @ 0x27ed0
bool sub_27ed0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_27ee0 @ 0x27ee0
bool sub_27ee0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_27ef0 @ 0x27ef0
bool sub_27ef0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_27f00 @ 0x27f00
unsigned long sub_27f00(int a0)
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

// Function: sub_27f50 @ 0x27f50
bool sub_27f50(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_27f70 @ 0x27f70
bool sub_27f70(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_27f80 @ 0x27f80
unsigned int sub_27f80(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_27fb0 @ 0x27fb0
int sub_27fb0(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_27fd0 @ 0x27fd0
int sub_27fd0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_27ff0 @ 0x27ff0
int sub_27ff0(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_27fb0(*a0);
    v1 = sub_27fb0(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_28050 @ 0x28050
void sub_28050(unsigned int *a0)
{
  *a0 = 0xffffff9c;
}

// Function: sub_28060 @ 0x28060
char * sub_28060(char *a0)
{
  return &a0[strspn(a0,"/")];
}

// Function: sub_28080 @ 0x28080
void sub_28080(int a0)
{
  if (a0 <= -1)
    return;
  if (!close(a0))
    return;
  __assert_fail("! close_fail","chdir-long.c",0x40,"cdb_free"); // no-return
}

// Function: sub_280c0 @ 0x280c0
unsigned long sub_280c0(int *a0,char *a1) // return-dupe
{
  int v1; // eax
  
  v1 = openat(*a0,a1,0x10900);
  if (0 <= v1) {
    sub_28080(*a0);
    *a0 = v1;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_28100 @ 0x28100
void sub_28100(int a0)
{
  fchdir(a0); // tail-call
}

// Function: sub_28110 @ 0x28110
int sub_28110(char *a0)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char *v6;
  unsigned int v7; // stack - 0x44
  
  v1 = chdir(a0);
  if (!v1)
    return 0;
  v2 = __errno_location();
  if (*v2 != 0x24)
    return v1;
  v3 = strlen(a0);
  sub_28050(&v7);
  if (!v3)
    __assert_fail("0 < len","chdir-long.c",0x7e,"chdir_long"); // no-return
  if (v3 <= 0xfff)
    __assert_fail("4096 <= len","chdir-long.c",0x7f,"chdir_long"); // no-return
  v4 = strspn(a0,"/");
  if (v4 != 2) { // branch-flip
    v6 = a0;
    if (v4) {
      v6 = &a0[v4];
      if (sub_280c0(&v7,"/")) goto label_2814e;
    }
  }
  else {
    v5 = memchr(&a0[3],0x2f,v3 - 3);
    if (!v5) {
label_2814e:
      *v2 = 0x24;
      return -1;
    }
    *v5 = 0;
    v1 = sub_280c0(&v7,a0);
    *v5 = 0x2f;
    if (v1) goto label_2814e;
    v6 = (char *)sub_28060(&v5[1]);
  }
  if (*v6 == '/')
    __assert_fail("*dir != \'/\'","chdir-long.c",0xa2,"chdir_long"); // no-return
  a0 = &a0[v3];
  if (a0 < v6)
    __assert_fail("dir <= dir_end","chdir-long.c",0xa3,"chdir_long"); // no-return
  while (0xfff < (long)a0 - (long)v6) {
    v5 = (char *)memrchr(v6,0x2f,0x1000);
    if (!v5) goto label_2814e;
    *v5 = 0;
    if (0xfff < (long)v5 - (long)v6)
      __assert_fail("slash - dir < 4096","chdir-long.c",0xb3,"chdir_long"); // no-return
    v1 = sub_280c0(&v7,v6);
    *v5 = 0x2f;
    if (v1) goto label_2814e;
    v6 = (char *)sub_28060(&v5[1]);
  }
  if (((a0 <= v6) || (!sub_280c0(&v7,v6))) && (!sub_28100(v7))) {
    sub_28080(v7);
    return 0;
  }
label_2814e:
  v1 = *v2;
  sub_28080(v7);
  *v2 = v1;
  return -1;
}

// Function: sub_283d0 @ 0x283d0
unsigned long sub_283d0(void *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror(a0);
  v3 = sub_17850(a0);
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

// Function: sub_28440 @ 0x28440
unsigned long sub_28440(unsigned long a0)
{
  return CONCAT71((undefined7)(a0 - 1 >> 8),(a0 - 1 & a0) == 0);
}

// Function: sub_28450 @ 0x28450
void sub_28450(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_28470 @ 0x28470
unsigned long sub_28470(long *a0,long *a1) // return-dupe x3
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
    v1 = sub_28440(v2);
    if (!v1)
      return 0;
    v5 = v3;
    if (!v2)
      return 1;
  }
  else {
    a0[2] = 1;
    if (!sub_28440(1))
      return 0;
    v6 = a1[1];
    v5 = v4;
  }
  v2 = *a1;
  *v5 = v6;
  v5[1] = v2;
  return 0;
}

// Function: sub_28520 @ 0x28520
void sub_28520(void *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_28530 @ 0x28530
undefined16 sub_28530(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_28520(v1);
  return v1._0_16_;
}

// Function: sub_28580 @ 0x28580
bool sub_28580(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_2a3a0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_28610 @ 0x28610
void sub_28610(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_28620 @ 0x28620
void sub_28620(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_28640 @ 0x28640
void sub_28640(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_28650 @ 0x28650
void sub_28650(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_28680 @ 0x28680
char * sub_28680(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_286c0 @ 0x286c0
long sub_286c0(unsigned long a0)
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

// Function: sub_28710 @ 0x28710
void sub_28710(unsigned long a0)
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1))); // tail-call
}

// Function: sub_28740 @ 0x28740
int sub_28740(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_28770 @ 0x28770
void sub_28770(long *a0,long *a1) // return-dupe
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

// Function: sub_287d0 @ 0x287d0
unsigned int sub_287d0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x31660) >> (a0 & 0x1f) & 1;
}

// Function: sub_287f0 @ 0x287f0
unsigned long sub_287f0(char *a0)
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
    sub_1d3e0(v1);
    if ((v7) && (!v8)) break;
    v5 = &v5[v6];
    v2 += 1;
  }
  return v2;
}

// Function: sub_288b0 @ 0x288b0
int sub_288b0(int a0,int a1)
{
  int v1; // edx
  
  v1 = (a0 - a1) + 0x17e;
  return (a0 - v1) + 3 + (v1 / 7) * 7;
}

// Function: sub_288f0 @ 0x288f0
void sub_288f0(long a0,long a1,long a2)
{
  long *v1; // rax
  
  if (a2) {
    v1 = __ctype_toupper_loc();
    do {
      a2 -= 1;
      *(char *)(a0 + a2) = (char)*(unsigned int *)(*v1 + (unsigned long)*(unsigned char *)(a1 + a2) * 4);
    } while (a2);
    return;
  }
}

// Function: sub_28940 @ 0x28940
void sub_28940(long a0,long a1,long a2)
{
  long *v1; // rax
  
  if (a2) {
    v1 = __ctype_tolower_loc();
    do {
      a2 -= 1;
      *(char *)(a0 + a2) = (char)*(unsigned int *)(*v1 + (unsigned long)*(unsigned char *)(a1 + a2) * 4);
    } while (a2);
    return;
  }
}

// Function: sub_28990 @ 0x28990
long sub_28990(char *a0,long a1,char *a2,unsigned int *a3,char a4,unsigned int a5,int a6,unsigned long a7,int a8)
{
  int v1;
  char v10;
  bool v11;
  unsigned int v12;
  unsigned int v13; // eax
  unsigned long v14;
  unsigned char *v15;
  long v16;
  int v17;
  char *v18;
  long v19;
  bool v2;
  char v20;
  unsigned int v21;
  char *v22;
  unsigned long v23;
  char *v24;
  unsigned long v25; // stack - 0x450
  unsigned long v26; // stack - 0x448
  char v27 [1010];
  unsigned long v28; // stack - 0x488
  char v29;
  long v3;
  int v30; // ebp
  char *v31;
  char *v32;
  int v33;
  unsigned char v34;
  int v35;
  unsigned int v36;
  unsigned int v37;
  char *v38;
  long v39;
  unsigned char v4;
  int v40;
  unsigned long v41;
  char *v42;
  int *v43; // stack - 0x4e8
  unsigned int v44; // stack - 0x4e0
  unsigned long v45; // stack - 0x4c0
  unsigned long v46; // stack - 0x4b8
  int v47; // stack - 0x4a8
  unsigned long v48; // stack - 0x480
  unsigned long v49; // stack - 0x478
  int *v5;
  long v50; // stack - 0x470
  unsigned long v51; // stack - 0x468
  unsigned long v52; // stack - 0x460
  unsigned long v53; // stack - 0x458
  unsigned int v6;
  unsigned int v7;
  unsigned long v8;
  unsigned long v9;
  
  v9 = a7;
  v41 = (unsigned long)a6;
  v43 = __errno_location();
  v38 = *(char **)&a3[0xc];
  v37 = a3[2];
  v1 = *v43;
  if (!v38)
    v38 = "";
  if (0xd <= (int)v37) // branch-flip
    v44 = v37 - 0xc;
  else {
    v44 = 0xc;
    if (v37)
      v44 = v37;
  }
  v10 = *a2;
  v39 = 0;
  v5 = v43;
  v6 = (unsigned int)v26;
  v7 = v26._4_4_;
  v8 = a7;
  v35 = a6;
  v33 = a8;
label_28a28:
  if (!v10) {
    if ((a0) && (a1))
      *a0 = '\0';
    *v5 = v1;
    return v39;
  }
  v24 = a2;
  a6 = v35;
  a7 = v8;
  a8 = v33;
  v26._0_4_ = v6;
  v26._4_4_ = v7;
  if (v10 != '%') {
    v23 = 0;
    if (0 <= (int)v41)
      v23 = v41;
    v14 = 1;
    if (v23)
      v14 = v23;
    if ((unsigned long)(a1 - v39) <= v14) goto label_28ab0;
    if (a0) {
      v42 = a0;
      if (1 < (int)v41) {
        v42 = &a0[v23 - 1];
        memset(a0,0x20,v23 - 1);
      }
      a0 = &v42[1];
      *v42 = *a2;
    }
    v39 += v14;
    goto label_28a78;
  }
  v12 = 0;
  v37 = 0;
  v10 = a4;
  while( true ) {
    v20 = (char)v12;
    v42 = &v24[1];
    v21 = (unsigned int)*v42;
    v24 = &v24[1];
    v34 = *v42 - 0x23;
    v23 = (unsigned long)v21;
    if (0x3d <= v34) break;
    v14 = 1L << (v34 & 0x3f);
    if (v14 & 0x1000000000002500) // branch-flip
      v37 = v21;
    else if (v34 != 0x3b) { // branch-flip
      v12 = (unsigned int)v14 & 1;
      if (!(v14 & 1)) break;
    }
    else {
      v10 = '\x01';
    }
  }
  v14 = (unsigned long)v21;
  if (v21 - 0x30 <= 9) {
    v41 = 0;
    do {
      v16 = (long)(int)v41 * 10;
      v40 = (int)v16;
      if ((v40 != v16) || (v41 = (unsigned long)(unsigned int)(v40 + *v24 + -0x30), SCARRY4(v40,*v24 + -0x30)))
        v41 = 0x7fffffff;
      v42 = &v24[1];
      v23 = (unsigned long)(unsigned int)(int)*v42;
      v24 = &v24[1];
      v14 = v23;
    } while ((unsigned int)((int)*v42 - 0x30U) <= 9);
  }
  if (((char)v23 != 'E') && ((char)v23 != 'O'))
    v14 = 0;
  else {
    v23 = (unsigned long)(unsigned char)v24[1];
    v24 = &v24[1];
  }
  v34 = (unsigned char)v23;
  v40 = (int)v41;
  v42 = v24;
  v32 = a0;
  if (0x7b <= v34) goto label_28be0;
  v17 = (int)v14;
  v12 = v44;
  switch(v23 & 0xff) {
    case 0:
      v42 = &v24[-1];
      break;
    case 0x25:
      v42 = &v24[-1];
      if (v42 == a2) {
        if ((v37 != 0x2d) && (0 <= v40)) {
          v23 = (unsigned long)v40;
          v41 = 1;
          if (v23)
            v41 = v23;
          if ((unsigned long)(a1 - v39) <= v41) goto label_28ab0;
          if (a0) {
            if (2 <= v40) {
              v23 -= 1;
              v42 = &a0[v23];
              if ((v37 != 0x30) && (v37 != 0x2b)) {
                memset(a0,0x20,v23);
                a0 = v42;
              }
              else {
                memset(a0,0x30,v23);
                a0 = v42;
              }
            }
label_29f27:
            v42 = &a0[1];
            *a0 = *v24;
            a0 = v42;
          }
        }
        else {
          if ((unsigned long)(a1 - v39) <= 1) goto label_28ab0;
          v41 = 1;
          if (a0) goto label_29f27;
        }
        goto label_29277;
      }
      break;
    case 0x3a:
      v18 = &v24[1];
      v41 = 1;
      v20 = v24[1];
      while (v20 == ':') {
        v41 += 1;
        v18 = &v24[v41];
        v20 = v24[v41];
      }
      if (v20 == 'z') goto label_295a0;
      break;
    case 0x41:
    case 0x61:
      if (v17) break;
      if (v20)
        v10 = v20;
      v29 = '\0';
      v15 = (unsigned char *)((long)&v25 + 5);
      goto label_28cce;
    case 0x42:
      if (v17 != 0x45) {
        if (v20)
          v10 = v20;
        v29 = '\0';
        goto label_28cae;
      }
      break;
    case 0x43:
      if (v17 != 0x45) {
        v21 = a3[5];
        v46._0_1_ = (int)v21 < -0x76c;
        v12 = (int)((-(unsigned int)(v21 + 0x76c < 0x76c) & 0xffffff9d) + v21) / 100 + 0x13;
        v11 = -0x76c <= (int)v21;
        if (!v37)
          v37 = a5;
        if (v37 != 0x2b) { // branch-flip
          v2 = 0;
          v47 = 2;
        }
        else {
          v47 = 2;
          v21 = 99;
label_29f5d:
          if (v12 <= v21) { // branch-flip
label_29e99:
            v2 = v47 < v40;
            v11 = (bool)(v46._0_1_ ^ 1);
            v37 = 0x2b;
          }
          else {
            v2 = 1;
            v37 = 0x2b;
            v11 = (bool)(v46._0_1_ ^ 1);
          }
        }
label_28f35:
        v21 = 0;
        goto label_28f40;
      }
      goto label_28f52;
    case 0x44:
      if (!v17) {
        v17 = -1;
        v42 = "%m/%d/%y";
        goto label_2901d;
      }
      break;
    case 0x46:
      if (v17) break;
      if ((v37) || (0 <= v40)) {
        v42 = "%Y-%m-%d";
        v17 = 0;
        if (0 <= v40 + -6)
          v17 = v40 + -6;
        goto label_2901d;
      }
      v41 = sub_28990(0,0xffffffffffffffff,"%Y-%m-%d",a3,v10,0x2b,4,v9,v33,(long)*(int *)((v23 & 0xff) * 4 + 0x31694) + 0x31694);
      v42 = "%Y-%m-%d";
      v23 = 0;
      v37 = 0x2b;
      v45 = v41;
      v17 = 4;
      goto label_2909b;
    case 0x47:
    case 0x56:
    case 0x67:
      if (v17 != 0x45) {
        v12 = a3[5];
        v13 = (v12 - 100) + ((int)v12 >> 0x1f & 400U);
        v21 = a3[7];
        v36 = v12;
        v35 = sub_288b0(v21,a3[6]);
        if (0 <= v35) { // branch-flip
          v33 = 0x16d;
          if ((!(v13 & 3)) && (v33 = 0x16e, (v13 * -0x3d70a3d7 + 0x51eb850 >> 2 | v12 * 0x40000000) <= 0x28f5c28))
            v33 = 0x16e - (unsigned int)((int)v13 % 400 != 0);
          v33 = sub_288b0(v21 - v33);
          if (0 <= v33)
            v35 = v33;
          v33 = (v33 >> 0x1f) + 1;
        }
        else {
          v35 = 0x16d;
          v13 -= 1;
          if ((!(v13 & 3)) && (v35 = 0x16e, (v13 * -0x3d70a3d7 + 0x51eb850 >> 2 | v13 * 0x40000000) <= 0x28f5c28))
            v35 = 0x16e - (unsigned int)((int)v13 % 400 != 0);
          v35 = sub_288b0(v21 + v35);
          v33 = -1;
        }
        if (v34 == 0x47) {
          v12 = v36 + 0x76c + v33;
          v46._0_1_ = (int)v36 < 0xfffff894U - v33;
          if (v37) { // branch-flip
label_29f46:
            v47 = 4;
            v21 = 9999;
            if (v37 == 0x2b) goto label_29f5d;
            v2 = 0;
            v11 = (bool)(v46._0_1_ ^ 1);
          }
          else {
            if (a5 == 0x2b) {
label_2a166:
              v47 = 4;
              v21 = 9999;
              goto label_29f5d;
            }
            v2 = 0;
            v47 = 4;
            v11 = 0xfffff894U - v33 <= (int)v36;
            v37 = a5;
          }
          goto label_28f35;
        }
        if (v34 == 0x67) {
          v12 = ((int)v36 % 100 + v33) % 100;
          if (0 <= (int)v12) { // branch-flip
            if (!v37)
              v37 = a5;
          }
          else {
            if ((int)v36 < 0xfffff894U - v33) {
              v12 = -v12;
              goto label_28fea;
            }
            v12 += 100;
            if (!v37) goto label_28ff0;
          }
          goto label_29e86;
        }
        v47 = 2;
        v12 = v35 / 7 + 1;
        goto label_291a2;
      }
      break;
    case 0x48:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = a3[2];
        goto label_291a2;
      }
      break;
    case 0x49:
      if (v17 != 0x45) {
        v47 = 2;
        goto label_291a2;
      }
      break;
    case 0x4d:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = a3[1];
        goto label_291a2;
      }
      break;
    case 0x4e:
      if (v17 != 0x45) {
        v30 = 9;
        v17 = v33;
        if (v40 <= 0)
          v40 = 9;
        do {
          if (v30 <= v40) {
            if (v30 == 1) {
              v23 = 1;
              v41 = 1;
              goto label_29865;
            }
            if (v17 % 10) goto label_29857;
          }
          v30 -= 1;
          v17 /= 10;
        } while( true );
      }
      break;
    case 0x50:
      v29 = '\x01';
      goto label_29b8c;
    case 0x52:
      v17 = -1;
      v42 = "%H:%M";
      goto label_2901d;
    case 0x53:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = *a3;
        goto label_291a2;
      }
      break;
    case 0x54:
      v17 = -1;
      v42 = "%H:%M:%S";
label_2901d:
      v41 = sub_28990(0,0xffffffffffffffff,v42,a3,v10,v37,v17,v9,v33);
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v45 = v23;
        if (v23 <= v41)
          v45 = v41;
      }
      else {
        v45 = v41;
        v23 = 0;
      }
label_2909b:
      if ((unsigned long)(a1 - v39) <= v45) goto label_28ab0;
      if (a0) {
        if (v41 < v23) {
          v23 -= v41;
          v32 = &a0[v23];
          if ((v37 != 0x30) && (v37 != 0x2b))
            memset(a0,0x20,v23);
          else {
            memset(a0,0x30,v23);
          }
        }
        a0 = &v32[v41];
        sub_28990(v32,a1 - v39,v42,a3,v10,v37,v17,v9,a8);
      }
      v39 += v45;
      goto label_28a78;
    case 0x55:
      if (v17 != 0x45) {
        v17 = (a3[7] - a3[6]) + 7;
label_29297:
        v47 = 2;
        v12 = v17 / 7;
        goto label_291a2;
      }
      break;
    case 0x57:
      if (v17 != 0x45) {
        v17 = (((int)(a3[6] + 6) / 7) * 7 - (a3[6] + 6)) + 7 + a3[7];
        goto label_29297;
      }
      break;
    case 0x58:
    case 99:
    case 0x78:
      if (v17 != 0x4f) goto label_28cac;
      break;
    case 0x59:
      if (v17 == 0x45) goto label_28f52;
      if (v17 == 0x4f) break;
      v46._0_1_ = (int)a3[5] < -0x76c;
      v12 = a3[5] + 0x76c;
      if (v37) goto label_29f46;
      if (a5 == 0x2b) goto label_2a166;
      v2 = 0;
      v21 = 0;
      v47 = 4;
      v37 = a5;
      goto label_29360;
    case 0x5a:
      if (v20)
        v10 = '\0';
      v41 = strlen(v38);
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v46 = v23;
        if (v23 <= v41)
          v46 = v41;
      }
      else {
        v46 = v41;
        v23 = 0;
      }
      if ((unsigned long)(a1 - v39) <= v46) goto label_28ab0;
      if (a0) {
        v42 = a0;
        if (v41 < v23) {
          v23 -= v41;
          v42 = &a0[v23];
          if ((v37 != 0x30) && (v37 != 0x2b))
            memset(a0,0x20,v23);
          else {
            memset(a0,0x30,v23);
          }
        }
        if (v20) // branch-flip
          sub_28940(v42);
        else if (v10) // branch-flip
          sub_288f0(v42,v38,v41);
        else {
          memcpy(v42,v38,v41);
        }
        a0 = &v42[v41];
      }
      v39 += v46;
      goto label_28a78;
    case 0x62:
    case 0x68:
      if (v20)
        v10 = v20;
      if (v17 != 0x45) goto label_28cac;
      break;
    case 100:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = a3[3];
        goto label_291a2;
      }
      break;
    case 0x65:
      if (v17 != 0x45) {
        v12 = a3[3];
label_2918e:
        v47 = 2;
        if (!v37)
          v37 = 0x5f;
        goto label_291a2;
      }
      break;
    case 0x6a:
      if (v17 != 0x45) {
        v2 = 0;
        v47 = 3;
        v21 = a3[7];
        v12 = v21 + 1;
        v46._0_1_ = (int)v21 < -1;
        v11 = -1 <= (int)v21;
        v21 = 0;
        goto label_28f40;
      }
      break;
    case 0x6b:
      if (v17 != 0x45) {
        v12 = a3[2];
        goto label_2918e;
      }
      break;
    case 0x6c:
      if (v17 != 0x45) goto label_2918e;
      break;
    case 0x6d:
      if (v17 != 0x45) {
        v2 = 0;
        v47 = 2;
        v21 = a3[4];
        v12 = v21 + 1;
        v46._0_1_ = (int)v21 < -1;
        v11 = -1 <= (int)v21;
        v21 = 0;
        goto label_28f40;
      }
      break;
    case 0x6e:
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v41 = 1;
        if (v23)
          v41 = v23;
        if ((unsigned long)(a1 - v39) <= v41) goto label_28ab0;
        if (a0) {
          if (2 <= v40) {
            v23 -= 1;
            v42 = &a0[v23];
            if ((v37 != 0x30) && (v37 != 0x2b)) {
              memset(a0,0x20,v23);
              a0 = v42;
            }
            else {
              memset(a0,0x30,v23);
              a0 = v42;
            }
          }
label_2926f:
          *a0 = '\n';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v39) <= 1) goto label_28ab0;
        v41 = 1;
        if (a0) goto label_2926f;
      }
      goto label_29277;
    case 0x70:
      v29 = '\0';
label_29b8c:
      v34 = 0x70;
      if (v20) {
        v10 = '\0';
        v29 = v20;
      }
label_28cae:
      if (v17) goto label_28f61;
      v15 = (unsigned char *)((long)&v25 + 5);
      goto label_28cce;
    case 0x71:
      if (v17 == 0x4f) goto label_28f52;
      v2 = 0;
      v21 = 0;
      v47 = 1;
      v46._0_1_ = 0;
      v41 = (unsigned long)(((int)(a3[4] * 0xb) >> 5) + 1);
      goto label_2936c;
    case 0x72:
label_28cac:
      v29 = '\0';
      goto label_28cae;
    case 0x73:
      v51 = *(unsigned long *)&a3[8];
      v52 = *(unsigned long *)&a3[10];
      v28 = *(unsigned long *)a3;
      v48 = *(unsigned long *)&a3[2];
      v49 = *(unsigned long *)&a3[4];
      v53 = *(unsigned long *)&a3[0xc];
      v50._0_4_ = (unsigned int)*(unsigned long *)&a3[6];
      v50 = CONCAT44(0xffffffff,(unsigned int)v50);
      v16 = sub_26130(v9,&v28);
      if (v50 <= -1) {
        *v43 = 0x4b;
        return 0;
      }
      v46._0_1_ = SUB81((unsigned long)v16 >> 0x3f,0);
      v42 = &v27[1];
      v19 = v16;
      do {
        v3 = v19 / 10;
        v20 = (char)v19 + (char)v3 * '\xf6';
        if (v16 <= -1)
          v20 = -v20;
        v42 = &v42[-1];
        *v42 = v20 + '0';
        v19 = v3;
      } while (v3);
      v2 = 0;
      v47 = 1;
      goto label_294b0;
    case 0x74:
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v41 = 1;
        if (v23)
          v41 = v23;
        if ((unsigned long)(a1 - v39) <= v41) goto label_28ab0;
        if (a0) {
          if (2 <= v40) {
            v23 -= 1;
            v42 = &a0[v23];
            if ((v37 != 0x30) && (v37 != 0x2b)) {
              memset(a0,0x20,v23);
              a0 = v42;
            }
            else {
              memset(a0,0x30,v23);
              a0 = v42;
            }
          }
label_29b2b:
          *a0 = '\t';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v39) <= 1) goto label_28ab0;
        v41 = 1;
        if (a0) goto label_29b2b;
      }
label_29277:
      v39 += v41;
      goto label_28a78;
    case 0x75:
      v47 = 1;
      v12 = (int)(a3[6] + 6) % 7 + 1;
label_291a2:
      v2 = 0;
      v21 = 0;
      v4 = (unsigned char)(v12 >> 0x18);
      v46._0_1_ = (bool)(v4 >> 7);
      v11 = (bool)((unsigned char)~v4 >> 7);
      goto label_28f40;
    case 0x77:
      if (v17 != 0x45) {
        v47 = 1;
        v12 = a3[6];
        goto label_291a2;
      }
      break;
    case 0x79:
      if (v17 != 0x45) {
        v21 = (int)a3[5] % 100;
        v12 = v21;
        if ((int)v21 <= -1) {
          v12 = v21 + 100;
          if ((int)a3[5] <= -0x76d)
            v12 = -v21;
        }
label_28fea:
        if (v37) { // branch-flip
label_29e86:
          if (v37 == 0x2b) {
label_29e8c:
            v46._0_1_ = 0;
            v47 = 2;
            goto label_29e99;
          }
        }
        else {
label_28ff0:
          if (a5 == 0x2b) goto label_29e8c;
          v37 = a5;
        }
        v2 = 0;
        v11 = 1;
        v46._0_1_ = 0;
        v47 = 2;
        goto label_28f35;
      }
label_28f52:
      v29 = '\0';
label_28f61:
      v25._5_1_ = (char)v14;
      v15 = (unsigned char *)((long)&v25 + 6);
label_28cce:
      v25._3_2_ = 0x2520;
      *v15 = v34;
      v15[1] = 0;
      v41 = strftime((char *)&v26,0x400,(char *)((long)&v25 + 3),a3);
      if (v41) {
        v18 = (char *)(v41 - 1);
        if ((v37 != 0x2d) && (0 <= v40)) {
          v42 = (char *)(long)v40;
          v31 = v42;
          if (v42 <= v18)
            v31 = v18;
        }
        else {
          v42 = NULL;
          v31 = v18;
        }
        if ((char *)(a1 - v39) <= v31) goto label_28ab0;
        if (a0) {
          if (v18 < v42) {
            v41 = (long)v42 - (long)v18;
            v32 = &a0[v41];
            if ((v37 != 0x30) && (v37 != 0x2b))
              memset(a0,0x20,v41);
            else {
              memset(a0,0x30,v41);
            }
          }
          a2 = (char *)((long)&v26 + 1);
          if (!v29) goto label_28da7;
          sub_28940(v32);
          a0 = &v32[(long)v18]; // crossjump-dupe
        }
        v39 += (long)v31; // crossjump-dupe
      }
      goto label_28a78;
    case 0x7a:
      v41 = 0;
      v18 = v24;
label_295a0:
      v24 = v18;
      if ((int)a3[8] < 0) goto label_28a78;
      v46._0_1_ = 1;
      v35 = (int)*(unsigned long *)&a3[10];
      if ((0 <= v35) && (v46._0_1_ = 0, !v35))
        v46._0_1_ = *v38 == '-';
      v12 = v35 / 0xe10;
      v33 = (v35 / 0x3c) % 0x3c;
      if (v41 != 2) { // branch-flip
        if (3 <= v41) { // branch-flip
          v42 = v18;
          if (v41 != 3) break;
          if (v35 % 0x3c) goto label_2a073;
          if (v33) goto label_2963b;
          v2 = 1;
          v47 = 3;
          v11 = (bool)(v46._0_1_ ^ 1);
          v21 = 0;
        }
        else if (v41) { // branch-flip
label_2963b:
          v2 = 1;
          v47 = 6;
          v11 = (bool)(v46._0_1_ ^ 1);
          v12 = v12 * 100 + v33;
          v21 = 4;
        }
        else {
          v2 = 1;
          v47 = 5;
          v11 = (bool)(v46._0_1_ ^ 1);
          v12 = v12 * 100 + v33;
          v21 = 0;
        }
      }
      else {
label_2a073:
        v2 = 1;
        v47 = 9;
        v11 = (bool)(v46._0_1_ ^ 1);
        v21 = 0x14;
        v12 = v12 * 10000 + v33 * 100 + v35 % 0x3c;
      }
label_28f40:
      if ((int)v14 == 0x4f) {
        if (v11) goto label_28f52;
      }
label_29360:
      v41 = (unsigned long)v12;
      if (v46._0_1_)
        v41 = (unsigned long)-v12;
label_2936c:
      v32 = &v27[1];
      while( true ) {
        if (v21 & 1) {
          v32[-1] = ':';
          v32 = &v32[-1];
        }
        v21 = (int)v21 >> 1;
        v42 = &v32[-1];
        v32[-1] = (char)v41 + (char)(v41 / 10) * '\xf6' + '0';
        if (((unsigned int)v41 <= 9) && (!v21)) break;
        v41 /= 10;
        v32 = v42;
      }
label_294b0:
      v12 = v37;
      if (!v37)
        v12 = 0x30;
      if (v40 <= -1)
        v40 = v47;
      v35 = ((int)v27 + 1) - (int)v42;
      if (v46._0_1_) { // branch-flip
        v20 = '-';
label_29c15:
        v33 = v40 + -1;
        v17 = v33 - v35;
        if ((v17 <= 0) || (v37 == 0x2d))
          v17 = 0;
        if (v12 != 0x5f) { // branch-flip
          if ((unsigned long)(a1 - v39) <= 1) goto label_28ab0;
          if (a0) goto label_29c47;
label_29c4e:
          v39 += 1;
          v40 = v33;
          if (v12 != 0x2d) goto label_29c5b;
label_29c8e:
          v31 = (char *)(long)v35;
          v32 = NULL;
        }
        else {
          v41 = (unsigned long)v17;
          v39 += v41;
          if (a0) {
            memset(a0,0x20,v41);
            a0 = &a0[v41];
            if ((unsigned long)(a1 - v39) <= 1) goto label_28ab0;
            v33 = (v40 - v17) + -1;
label_29c47:
            *a0 = v20;
            a0 = &a0[1];
            goto label_29c4e;
          }
          if ((unsigned long)(a1 - v39) <= 1) goto label_28ab0;
          v39 += 1;
          v40 = (v40 - v17) + -1;
label_29c5b:
          v31 = (char *)(long)v35;
          v32 = NULL;
          if (0 <= v40) goto label_294f6;
        }
      }
      else {
        if (v2) {
          v20 = '+';
          goto label_29c15;
        }
        if (((v35 >= v40) || (v37 == 0x2d)) && (v12 == 0x2d)) goto label_29c8e;
label_294f6:
        v32 = (char *)(long)v40;
        v31 = (char *)(long)v35;
        if ((char *)(long)v35 <= v32)
          v31 = v32;
      }
      v18 = (char *)(long)v35;
      if ((char *)(a1 - v39) <= v31) goto label_28ab0;
      if (a0) {
        v22 = a0;
        if (v18 < v32) {
          v41 = (long)v32 - (long)v18;
          v22 = &a0[v41];
          if ((v12 != 0x30) && (v12 != 0x2b))
            memset(a0,0x20,v41);
          else {
            memset(a0,0x30,v41);
          }
        }
        if (v10) // branch-flip
          sub_288f0(v22,v42,v18);
        else {
          memcpy(v22,v42,(unsigned long)v18);
        }
        a0 = &v22[(long)v18];
      }
      goto label_28c8a;
    
  }
label_28be0:
  v18 = &v42[1 - (long)a2];
  if ((0 <= v40) && (v37 != 0x2d)) {
    v22 = (char *)(long)v40;
    v31 = v22;
    if (v22 <= v18)
      v31 = v18;
  }
  else {
    v22 = NULL;
    v31 = v18;
  }
  if ((char *)(a1 - v39) <= v31) goto label_28ab0;
  v24 = v42;
  if (a0) {
    if (v18 < v22) {
      v41 = (long)v22 - (long)v18;
      v32 = &a0[v41];
      if ((v37 != 0x30) && (v37 != 0x2b))
        memset(a0,0x20,v41);
      else {
        memset(a0,0x30,v41);
      }
    }
label_28da7:
    if (v10) // branch-flip
      sub_288f0(v32,a2,v18);
    else {
      memcpy(v32,a2,(unsigned long)v18);
    }
    a0 = &v32[(long)v18];
  }
label_28c8a:
  v39 += (long)v31;
  goto label_28a78;
label_29857:
  v23 = (unsigned long)v30;
  v41 = v23;
  if (v30) { // branch-flip
label_29865:
    v42 = (char *)((long)&v26 + v41);
    do {
      v42 = &v42[-1];
      *v42 = (char)v17 + (char)(v17 / 10) * '\xf6' + '0';
      v17 /= 10;
    } while (v42 != (char *)((long)&v25 + (v41 - (unsigned int)(v30 - 1)) + 7));
  }
  else {
    v23 = 0;
  }
  if (!v37)
    v37 = 0x30;
  if ((unsigned long)(a1 - v39) > v23) {
    if (a0) {
      if (v10) // branch-flip
        sub_288f0(a0,&v26,v23);
      else if (8 <= v23) { // branch-flip
        *(unsigned long *)a0 = CONCAT44(v7,v6);
        *(unsigned long *)&a0[v23 - 8] = *(unsigned long *)((long)&v25 + v23);
        v16 = (long)a0 - ((unsigned long)&a0[8] & 0xfffffffffffffff8);
        v41 = v16 + v23 & 0xfffffffffffffff8;
        if (8 <= v41) {
          v14 = 0;
          do {
            *(unsigned long *)(((unsigned long)&a0[8] & 0xfffffffffffffff8) + v14) = *(unsigned long *)((long)&v26 + (v14 - v16));
            v14 += 8;
          } while (v14 < v41);
        }
      }
      else if (v23 & 4) { // branch-flip
        *(unsigned int *)a0 = v6;
        *(unsigned int *)&a0[v23 - 4] = *(unsigned int *)((long)&v25 + v23 + 4);
      }
      else if (v23) {
        v26._0_1_ = (char)v6;
        *a0 = (char)v26;
        if (v23 & 2)
          *(unsigned short *)&a0[v23 - 2] = *(unsigned short *)((long)&v25 + v23 + 6);
      }
      a0 = &a0[v23];
    }
    v39 += v23;
    if ((v37 != 0x2d) && (0 <= v40 - v30)) {
      v41 = (unsigned long)(v40 - v30);
      if ((unsigned long)(a1 - v39) <= v41) goto label_28ab0;
      if (a0) { // branch-flip
        if (v41) {
          v42 = &a0[v41];
          v39 += v41;
          if ((v37 != 0x30) && (v37 != 0x2b)) {
            memset(a0,0x20,v41);
            a0 = v42;
          }
          else {
            memset(a0,0x30,v41);
            a0 = v42;
          }
        }
      }
      else {
        v39 += v41;
      }
    }
    else if (a1 == v39) goto label_28ab0;
label_28a78:
    v10 = v24[1];
    a2 = &v24[1];
    v41 = 0xffffffffffffffff;
    v5 = v43;
    v6 = (unsigned int)v26;
    v7 = v26._4_4_;
    v8 = a7;
    v35 = a6;
    v33 = a8;
    goto label_28a28;
  }
label_28ab0:
  *v43 = 0x22;
  return 0;
}

// Function: sub_2a2c0 @ 0x2a2c0
void sub_2a2c0(void)
{
  sub_28990();
}

// Function: sub_2a2e0 @ 0x2a2e0
void sub_2a2e0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_2a2f0 @ 0x2a2f0
unsigned long sub_2a2f0(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_2a2e0();
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

// Function: sub_2a3a0 @ 0x2a3a0
void sub_2a3a0(void)
{
  sub_2a2f0(); // tail-call
}

// Function: sub_2a3b0 @ 0x2a3b0
void sub_2a3b0(void)
{
  sub_2a2e0(); // tail-call
}

// Function: sub_2a3c0 @ 0x2a3c0
void sub_2a3c0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_3e008); // tail-call
}

// Function: _DT_FINI @ 0x2a3d4
void _DT_FINI(void)
{
  return;
}
