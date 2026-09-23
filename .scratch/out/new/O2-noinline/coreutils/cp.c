// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_24fe8)
    return;
  (*dat_24fe8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_24bb8)(); // jump-as-call
}

// Function: free @ 0x3850
void free(void *a0)
{
  (*dat_24fd0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3860
void __cxa_finalize(void)
{
  (*dat_24ff8)(); // jump-as-call
}

// Function: context_type_get @ 0x3870
void context_type_get(void)
{
  (*dat_24bc0)(); // jump-as-call
}

// Function: getenv @ 0x3880
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24bc8)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x3890
int fgetfilecon(int a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_24bd0)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x38a0
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_24bd8)(); // jump-as-call
  return v1;
}

// Function: mkfifoat @ 0x38b0
int mkfifoat(int a0,char *a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = (*dat_24be0)(); // jump-as-call
  return v1;
}

// Function: utimensat @ 0x38c0
int utimensat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_24be8)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x38d0
void abort(void)
{
  (*dat_24bf0)(); // jump-as-call
}

// Function: __errno_location @ 0x38e0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_24bf8)(); // jump-as-call
  return v1;
}

// Function: error_at_line @ 0x38f0
void error_at_line(int a0,int a1,char *a2,unsigned int a3,char *a4,...)
{
  (*dat_24c00)(); // jump-as-call
}

// Function: strncmp @ 0x3900
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_24c08)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3910
void _exit(int a0)
{
  (*dat_24c10)(); // jump-as-call
}

// Function: __fpending @ 0x3920
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24c18)(); // jump-as-call
  return v1;
}

// Function: getfscreatecon @ 0x3930
int getfscreatecon(char **a0)
{
  int v1; // eax
  
  v1 = (*dat_24c20)(); // jump-as-call
  return v1;
}

// Function: mkdir @ 0x3940
int mkdir(char *a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_24c28)(); // jump-as-call
  return v1;
}

// Function: unlinkat @ 0x3950
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_24c30)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3960
void qsort(void)
{
  (*dat_24c38)(); // jump-as-call
}

// Function: reallocarray @ 0x3970
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24c40)(); // jump-as-call
  return v1;
}

// Function: mode_to_security_class @ 0x3980
void mode_to_security_class(void)
{
  (*dat_24c48)(); // jump-as-call
}

// Function: faccessat @ 0x3990
int faccessat(int a0,char *a1,int a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_24c50)(); // jump-as-call
  return v1;
}

// Function: readlink @ 0x39a0
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_24c58)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x39b0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_24c60)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x39c0
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_24c68)(); // jump-as-call
  return v1;
}

// Function: write @ 0x39d0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_24c70)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x39e0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24c78)(); // jump-as-call
  return v1;
}

// Function: pathconf @ 0x39f0
long pathconf(char *a0,int a1)
{
  long v1; // rax
  
  v1 = (*dat_24c80)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3a00
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24c88)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x3a10
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_24c90)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3a20
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24c98)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x3a30
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24ca0)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x3a40
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24ca8)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x3a50
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24cb0)(); // jump-as-call
  return v1;
}

// Function: selabel_lookup @ 0x3a60
void selabel_lookup(void)
{
  (*dat_24cb8)(); // jump-as-call
}

// Function: strlen @ 0x3a70
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24cc0)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x3a80
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_24cc8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3a90
void __stack_chk_fail(void)
{
  (*dat_24cd0)(); // jump-as-call
}

// Function: getopt_long @ 0x3aa0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_24cd8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3ab0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24ce0)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x3ac0
void freecon(char *a0)
{
  (*dat_24ce8)(); // jump-as-call
}

// Function: strchr @ 0x3ad0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24cf0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3ae0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24cf8)(); // jump-as-call
  return v1;
}

// Function: ftruncate @ 0x3af0
void ftruncate(void)
{
  (*dat_24d00)(); // jump-as-call
}

// Function: lseek @ 0x3b00
void lseek(void)
{
  (*dat_24d08)(); // jump-as-call
}

// Function: __assert_fail @ 0x3b10
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_24d10)(); // jump-as-call
}

// Function: fputs @ 0x3b20
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_24d18)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x3b30
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24d20)(); // jump-as-call
  return v1;
}

// Function: geteuid @ 0x3b40
unsigned int geteuid(void)
{
  unsigned int v1; // eax
  
  v1 = (*dat_24d28)(); // jump-as-call
  return v1;
}

// Function: ioctl @ 0x3b50
int ioctl(int a0,unsigned long a1,...)
{
  int v1; // eax
  
  v1 = (*dat_24d30)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x3b60
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24d38)(); // jump-as-call
  return v1;
}

// Function: copy_file_range @ 0x3b70
long copy_file_range(int a0,void *a1,int a2,void *a3,unsigned long a4,unsigned int a5)
{
  long v1; // rax
  
  v1 = (*dat_24d40)(); // jump-as-call
  return v1;
}

// Function: canonicalize_file_name @ 0x3b80
char * canonicalize_file_name(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24d48)(); // jump-as-call
  return v1;
}

// Function: close @ 0x3b90
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24d50)(); // jump-as-call
  return v1;
}

// Function: rewinddir @ 0x3ba0
void rewinddir(DIR *a0)
{
  (*dat_24d58)(); // jump-as-call
}

// Function: strspn @ 0x3bb0
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24d60)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x3bc0
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_24d68)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x3bd0
void posix_fadvise(void)
{
  (*dat_24d70)(); // jump-as-call
}

// Function: selabel_open @ 0x3be0
void selabel_open(void)
{
  (*dat_24d78)(); // jump-as-call
}

// Function: getcon @ 0x3bf0
int getcon(char **a0)
{
  int v1; // eax
  
  v1 = (*dat_24d80)(); // jump-as-call
  return v1;
}

// Function: lsetfilecon @ 0x3c00
int lsetfilecon(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_24d88)(); // jump-as-call
  return v1;
}

// Function: read @ 0x3c10
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_24d90)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x3c20
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_24d98)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3c30
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_24da0)(); // jump-as-call
  return v1;
}

// Function: fallocate @ 0x3c40
void fallocate(void)
{
  (*dat_24da8)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x3c50
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_24db0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x3c60
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24db8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3c70
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24dc0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3c80
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24dc8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3c90
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_24dd0)(); // jump-as-call
  return v1;
}

// Function: readlinkat @ 0x3ca0
long readlinkat(int a0,char *a1,char *a2,unsigned long a3)
{
  long v1; // rax
  
  v1 = (*dat_24dd8)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x3cb0
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_24de0)(); // jump-as-call
  return v1;
}

// Function: fpathconf @ 0x3cc0
long fpathconf(int a0,int a1)
{
  long v1; // rax
  
  v1 = (*dat_24de8)(); // jump-as-call
  return v1;
}

// Function: mknodat @ 0x3cd0
void mknodat(void)
{
  (*dat_24df0)(); // jump-as-call
}

// Function: rpmatch @ 0x3ce0
int rpmatch(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_24df8)(); // jump-as-call
  return v1;
}

// Function: mkdirat @ 0x3cf0
int mkdirat(int a0,char *a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = (*dat_24e00)(); // jump-as-call
  return v1;
}

// Function: umask @ 0x3d00
unsigned int umask(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_24e08)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3d10
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_24e10)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3d20
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e18)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x3d30
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24e20)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x3d40
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e28)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x3d50
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_24e30)(); // jump-as-call
  return v1;
}

// Function: context_new @ 0x3d60
void context_new(void)
{
  (*dat_24e38)(); // jump-as-call
}

// Function: putc_unlocked @ 0x3d70
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_24e40)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3d80
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e48)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3d90
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e50)(); // jump-as-call
  return v1;
}

// Function: fchmodat @ 0x3da0
int fchmodat(int a0,char *a1,unsigned int a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_24e58)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3db0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24e60)(); // jump-as-call
  return v1;
}

// Function: context_str @ 0x3dc0
void context_str(void)
{
  (*dat_24e68)(); // jump-as-call
}

// Function: is_selinux_enabled @ 0x3dd0
int is_selinux_enabled(void)
{
  int v1; // eax
  
  v1 = (*dat_24e70)(); // jump-as-call
  return v1;
}

// Function: renameat2 @ 0x3de0
int renameat2(int a0,char *a1,int a2,char *a3,unsigned int a4)
{
  int v1; // eax
  
  v1 = (*dat_24e78)(); // jump-as-call
  return v1;
}

// Function: attr_copy_fd @ 0x3df0
void attr_copy_fd(void)
{
  (*dat_24e80)(); // jump-as-call
}

// Function: __freading @ 0x3e00
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e88)(); // jump-as-call
  return v1;
}

// Function: fchdir @ 0x3e10
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24e90)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3e20
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e98)(); // jump-as-call
  return v1;
}

// Function: linkat @ 0x3e30
int linkat(int a0,char *a1,int a2,char *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_24ea0)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3e40
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24ea8)(); // jump-as-call
  return v1;
}

// Function: context_type_set @ 0x3e50
void context_type_set(void)
{
  (*dat_24eb0)(); // jump-as-call
}

// Function: __printf_chk @ 0x3e60
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_24eb8)(); // jump-as-call
  return v1;
}

// Function: fchmod @ 0x3e70
int fchmod(int a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_24ec0)(); // jump-as-call
  return v1;
}

// Function: chmod @ 0x3e80
int chmod(char *a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_24ec8)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x3e90
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24ed0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3ea0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24ed8)(); // jump-as-call
  return v1;
}

// Function: attr_copy_file @ 0x3eb0
void attr_copy_file(void)
{
  (*dat_24ee0)(); // jump-as-call
}

// Function: error @ 0x3ec0
void error(int a0,int a1,char *a2,...)
{
  (*dat_24ee8)(); // jump-as-call
}

// Function: attr_copy_check_permissions @ 0x3ed0
void attr_copy_check_permissions(void)
{
  (*dat_24ef0)(); // jump-as-call
}

// Function: open @ 0x3ee0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_24ef8)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x3ef0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_24f00)(); // jump-as-call
  return v1;
}

// Function: security_compute_create @ 0x3f00
void security_compute_create(void)
{
  (*dat_24f08)(); // jump-as-call
}

// Function: fchown @ 0x3f10
int fchown(int a0,unsigned int a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = (*dat_24f10)(); // jump-as-call
  return v1;
}

// Function: fdopendir @ 0x3f20
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_24f18)(); // jump-as-call
  return v1;
}

// Function: futimens @ 0x3f30
int futimens(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_24f20)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x3f40
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_24f28)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x3f50
void __cxa_atexit(void)
{
  (*dat_24f30)(); // jump-as-call
}

// Function: getline @ 0x3f60
long getline(char **a0,void *a1,FILE *a2)
{
  long v1; // rax
  
  v1 = (*dat_24f38)(); // jump-as-call
  return v1;
}

// Function: fchownat @ 0x3f70
int fchownat(int a0,char *a1,unsigned int a2,unsigned int a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_24f40)(); // jump-as-call
  return v1;
}

// Function: renameat @ 0x3f80
int renameat(int a0,char *a1,int a2,char *a3)
{
  int v1; // eax
  
  v1 = (*dat_24f48)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x3f90
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_24f50)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3fa0
void exit(int a0)
{
  (*dat_24f58)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x3fb0
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_24f60)(); // jump-as-call
  return v1;
}

// Function: fsetfilecon @ 0x3fc0
int fsetfilecon(int a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_24f68)(); // jump-as-call
  return v1;
}

// Function: getrandom @ 0x3fd0
void getrandom(void)
{
  (*dat_24f70)(); // jump-as-call
}

// Function: getfilecon @ 0x3fe0
int getfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_24f78)(); // jump-as-call
  return v1;
}

// Function: aligned_alloc @ 0x3ff0
void * aligned_alloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24f80)(); // jump-as-call
  return v1;
}

// Function: context_free @ 0x4000
void context_free(void)
{
  (*dat_24f88)(); // jump-as-call
}

// Function: mbsinit @ 0x4010
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_24f90)(); // jump-as-call
  return v1;
}

// Function: setfscreatecon @ 0x4020
int setfscreatecon(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_24f98)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x4030
int lgetfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_24fa0)(); // jump-as-call
  return v1;
}

// Function: symlinkat @ 0x4040
int symlinkat(char *a0,int a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_24fa8)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x4050
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_24fb0)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x4060
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_24fb8)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x4070
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_24fc0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4080
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24fc8)(); // jump-as-call
  return v1;
}

// Function: sub_4090 @ 0x4090
undefined16 sub_4090(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x40b0
int main(int argc,char **argv,char **envp)
{
  bool v1;
  unsigned int v10; // stack - 0x98
  char *v11; // stack - 0xc8
  char *v12; // stack - 0xc0
  char *v13; // stack - 0xb8
  char *v14; // stack - 0xb0
  char v15; // stack - 0xa2
  char v16; // stack - 0xa1
  int v17; // stack - 0x94
  int v18; // stack - 0x90
  int v19; // stack - 0x8c
  int v2; // eax
  char v20; // stack - 0x84
  char v21; // stack - 0x83
  char v22; // stack - 0x82
  char v23; // stack - 0x81
  char v24; // stack - 0x7c
  char v25; // stack - 0x7b
  unsigned short v26; // stack - 0x7a
  long v27; // stack - 0x70
  char v28; // stack - 0x68
  char v29; // stack - 0x67
  int v3; // eax
  char v30; // stack - 0x66
  char v31; // stack - 0x65
  char v32; // stack - 0x64
  char v33; // stack - 0x63
  unsigned short v34; // stack - 0x61
  char v35; // stack - 0x5e
  char v36; // stack - 0x5d
  char v37; // stack - 0x5c
  int v38; // stack - 0x54
  int v4; // eax
  int v5; // eax
  int v6; // eax
  char *v7;
  unsigned long v8; // rax
  long v9; // rdx
  
  sub_127f0(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_1a150(sub_e560);
  dat_2515a = 0 < is_selinux_enabled();
  sub_49f0(&v10);
  v15 = 0;
  v14 = NULL;
  v11 = NULL;
  v16 = 0;
  v12 = NULL;
  v13 = NULL;
  v1 = 0;
label_4188:
  v2 = getopt_long(argc,argv,"abdfHilLnprst:uvxPRS:TZ",(option *)0x244a0,NULL);
  if (v2 != -1) {
    if (0x88 < v2) {
      sub_57e0(1); // return-dupe, no-return
    }
    if (v2 <= 0x47) {
      if (v2 == -0x83) {
        sub_16cc0(stdout,"cp","GNU coreutils",dat_25020,"Torbjorn Granlund","David MacKenzie","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_57e0(0); // no-return
      sub_57e0(1);
    }
    switch(v2) {
      case 0x48:
        v17 = 3;
        break;
      default:
        goto label_4769;
      case 0x4c:
        v17 = 4;
        break;
      case 0x50:
        v17 = 2;
        break;
      case 0x52:
      case 0x72:
        v34 = CONCAT11(1,(char)v34);
        break;
      case 0x53:
        v13 = optarg;
        v1 = 1;
        break;
      case 0x54:
        v15 = 1;
        break;
      case 0x5a:
        if (dat_2515a) { // branch-flip
          if (optarg) // branch-flip
            v14 = optarg;
          else {
            v27 = selabel_open(0,0,0);
            if (!v27) {
              v7 = dcgettext(NULL,"warning: ignoring --context",5);
              error(0,*__errno_location(),v7);
            }
          }
        }
        else if (optarg)
          error(0,0,dcgettext(NULL,"warning: ignoring --context; it requires an SELinux-enabled kernel",5));
        break;
      case 0x61:
        v28 = 1;
        v17 = 2;
        v25 = 1;
        v26 = 0x101;
        v30 = 1;
        if (dat_2515a)
          v31 = '\x01';
        v33 = 1;
        v34 = 0x101;
        break;
      case 0x62:
        v1 = 1;
        v7 = optarg;
        if (!optarg)
          v7 = v12;
        v12 = v7;
        break;
      case 100:
        v28 = 1;
        v17 = 2;
        break;
      case 0x66:
        v22 = 1;
        break;
      case 0x69:
        v18 = 3;
        break;
      case 0x6c:
        v23 = '\x01';
        break;
      case 0x6e:
        v18 = 2;
        break;
      case 0x70:
label_41e0:
        v25 = 1;
        v26 = 0x101;
        v30 = 1;
        break;
      case 0x73:
        v35 = '\x01';
        break;
      case 0x74:
        if (v11) {
          error(1,0,dcgettext(NULL,"multiple target directories specified",5));
          return v5;
        }
        v11 = optarg;
        break;
      case 0x75:
        v36 = 1;
        break;
      case 0x76:
        v37 = 1;
        break;
      case 0x78:
        v24 = 1;
        break;
      case 0x80:
        v29 = 0;
        break;
      case 0x81:
        v16 = 1;
        break;
      case 0x82:
        sub_4a80(optarg,&v10,0);
        break;
      case 0x83:
        dat_25159 = 1;
        break;
      case 0x84:
        if (optarg) {
          sub_4a80(optarg,&v10,1);
          v30 = 1;
          break;
        }
        goto label_41e0;
      case 0x85:
        if (optarg) // branch-flip
          v38 = *(int *)(sub_d220("--reflink",optarg,(long *)0x24860,(void *)0x1cad0,4,dat_25028,1) * 4 + 0x1cad0);
        else {
          v38 = 2;
        }
        break;
      case 0x86:
        v19 = *(int *)(sub_d220("--sparse",optarg,(long *)0x24880,(void *)0x1cae0,4,dat_25028,1) * 4 + 0x1cae0);
        break;
      case 0x87:
        dat_25158 = 1;
        break;
      case 0x88:
        goto label_4220;
      
    }
    goto label_4188;
  }
  if ((v23) && (v35)) {
    v7 = "cannot make both hard and symbolic links";
    goto label_4754;
  }
  if (v18 != 2) { // branch-flip
    if (!v1) goto label_4679;
    if ((v38 == 2) && (v19 != 2)) goto label_4748;
    v7 = dcgettext(NULL,"backup type",5);
    v10 = sub_db20(v7,v12);
  }
  else {
    v36 = 0;
    if (v1) {
      v7 = "options --backup and --no-clobber are mutually exclusive";
      goto label_4754;
    }
label_4679:
    v10 = 0;
    if ((v38 == 2) && (v10 = 0, v19 != 2)) {
label_4748:
      v7 = "--reflink can be used only with --sparse=auto";
label_4754:
      error(0,0,dcgettext(NULL,v7,5));
label_4769:
      sub_57e0(1);
    }
  }
  sub_d750(v13);
  if (v17 != 1) { // branch-flip
label_456f:
    if (v34._1_1_) goto label_46e5;
  }
  else {
    if ((!v34._1_1_) || (v23)) {
      v17 = 4;
      goto label_456f;
    }
    v17 = 2;
label_46e5:
    v20 = v16;
  }
  if (v14 || v27) { // branch-flip
    if (v32) { // branch-flip
      if (v31) {
        error(1,0,dcgettext(NULL,"cannot set target context and preserve it",5));
        return v4;
      }
      if (!dat_2515a) {
        error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5)); // return-dupe
        return v3;
      }
    }
    else {
      v31 = '\0';
    }
    if ((v14) && (setfscreatecon(v14) < 0)) {
      v8 = sub_14e00(v14);
      v7 = dcgettext(NULL,"failed to set default file creation context to %s",5);
      error(1,*__errno_location(),v7,v8);
      return v6;
    }
  }
  else if ((v32) && (!dat_2515a)) {
    error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5));
    return v3;
  }
  sub_bfe0();
  v9 = (long)optind;
  argc -= optind;
  return (sub_5ae0(argc,&argv[v9],v11,(unsigned char)v15,&v10) ^ 1) & 0xff;
label_4220:
  v21 = 1;
  goto label_4188;
}

// Function: sub_4880 @ 0x4880
void sub_4880(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_24fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_48b0 @ 0x48b0
void sub_48b0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4920
void _FINI_0(void)
{
  if (!dat_250e8) {
    if (dat_24ff8)
      __cxa_finalize(dat_25008);
    sub_48b0();
    dat_250e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4960
void _INIT_0(void)
{
  return;
}

// Function: sub_4970 @ 0x4970
void sub_4970(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_49a0 @ 0x49a0
void sub_49a0(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nThe backup suffix is \'~\', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n",5),v1);
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n",5),v1); // tail-call
}

// Function: sub_49f0 @ 0x49f0
void sub_49f0(unsigned long *a0)
{
  char *v1; // rax
  
  sub_83b0(a0);
  *(unsigned int *)((long)a0 + 0x44) = 1;
  a0[1] = 0x200000004;
  a0[2] = 0x100000000;
  *(unsigned int *)((long)a0 + 0x1c) = 0;
  *(char *)&a0[4] = 0;
  *(unsigned int *)((long)a0 + 4) = 1;
  *(unsigned short *)&a0[3] = 0;
  a0[5] = 0;
  a0[6] = 0x100;
  *(unsigned int *)&a0[7] = 0;
  *(unsigned short *)((long)a0 + 0x3c) = 0;
  v1 = getenv("POSIXLY_CORRECT");
  a0[9] = 0;
  a0[10] = 0;
  *(bool *)((long)a0 + 0x3e) = v1 != NULL;
}

// Function: sub_4a80 @ 0x4a80
void sub_4a80(char *a0,unsigned long a1,bool a2)
{
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  long v4; // rax
  
  v1 = (char *)sub_17230(a0);
  v2 = "--no-preserve";
  if (a2)
    v2 = "--preserve";
  v3 = strchr(v1,0x2c);
  if (v3)
    *v3 = '\0';
  v4 = sub_d220(v2,v1,(long *)0x24460,(void *)0x1cab0,4,dat_25028);
  if (*(unsigned int *)(v4 * 4 + 0x1cab0) <= 6) {
    (*(void *)((long)*(int *)((unsigned long)*(unsigned int *)(v4 * 4 + 0x1cab0) * 4 + 0x1c990) + 0x1c990))(); // jump-as-call
    return;
  }
  abort(); // no-return
}

// Function: sub_4bf0 @ 0x4bf0
unsigned long sub_4bf0(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,void *a6) // return-dupe x2
{
  unsigned int v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  void *v12;
  char *v13;
  unsigned long *v14;
  unsigned long v15; // rax
  unsigned int v16; // ecx
  unsigned long v17; // rdx
  stat *v18; // stack - 0x1a8
  stat v19; // stack - 0x168
  long v2;
  stat v20; // stack - 0xd8
  stat **v21;
  char *v22; // rsp
  stat **v23; // rsp
  char *v24; // rsp
  char *v25;
  char *v26; // rsp
  char *v27;
  int v28;
  unsigned int v29; // r13d
  long v3;
  char *v30; // stack - 0x1a0
  unsigned long *v31; // stack - 0x198
  long *v32; // stack - 0x190
  stat *v33; // stack - 0x188
  char *v34; // stack - 0x180
  char *v35; // stack - 0x178
  void *v36; // stack - 0x170
  stat **v4;
  char *v5;
  void *v6;
  char v7;
  int v8;
  unsigned int v9;
  
  v6 = a6;
  v23 = &v18;
  v21 = &v18;
  v30 = a3;
  v32 = a4;
  v34 = a5;
  v10 = sub_e750(a0);
  *v32 = 0;
  if (v10 <= a1)
    return 1;
  v11 = strlen(a0);
  v4 = &v18;
  while (v23 != (stat **)((long)&v18 - (v11 + 0x18 & 0xfffffffffffff000))) {
    v21 = (char *)((long)v4 + -0x1000);
    v22 = (char *)((long)v4 + -0x1000);
    *(unsigned long *)((long)v4 + -8) = *(unsigned long *)((long)v4 + -8);
    v23 = (char *)((long)v4 + -0x1000);
    v4 = (stat **)v22;
  }
  v17 = (unsigned long)((unsigned int)(v11 + 0x18) & 0xff0);
  v3 = -v17;
  v24 = (char *)((long)v21 + v3);
  v25 = (char *)((long)v21 + v3);
  if (v17)
    *(unsigned long *)((long)v21 + -8) = *(unsigned long *)((long)v21 + -8);
  *(unsigned long *)((long)v21 + v3 + -8) = 0x4ce5;
  v36 = memcpy((void *)((unsigned long)((long)v21 + v3 + 0xf) & 0xfffffffffffffff0),a0,v11 + 1);
  v5 = (char *)((long)v21 + v3);
  while (v24 != (char *)((long)v21 + (v3 - (v10 + 0x18 & 0xfffffffffffff000)))) {
    v25 = &v5[-0x1000];
    v26 = &v5[-0x1000];
    *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
    v24 = &v5[-0x1000];
    v5 = v26;
  }
  v11 = (unsigned long)((unsigned int)(v10 + 0x18) & 0xff0);
  v3 = -v11;
  if (v11)
    *(unsigned long *)&v25[-8] = *(unsigned long *)&v25[-8];
  v12 = (void *)((unsigned long)&v25[v3 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v25[v3 + -8] = 0x4d4f;
  memcpy(v12,v36,v10);
  v27 = (char *)((long)v12 + a1);
  *(char *)((long)v12 + v10) = 0;
  v7 = *v27;
  while (v7 == '/') {
    v27 = &v27[1];
    v7 = *v27;
  }
  v33 = &v19;
  *(unsigned long *)&v25[v3 + -8] = 0x4d84;
  if (!fstatat(a2,v27,v33,0)) {
    if ((v19._24_4_ & 0xf000) == 0x4000) {
      *v34 = '\0';
      return 1;
    }
label_4c55:
    *(unsigned long *)&v25[v3 + -8] = 0x5206;
    v27 = (char *)sub_14b40(4,v12);
    *(unsigned long *)&v25[v3 + -8] = 0x521c;
    v13 = dcgettext(NULL,"%s exists but is not a directory",5);
    *(unsigned long *)&v25[v3 + -8] = 0x522d;
    error(0,0,v13,v27);
    return 0;
  }
  v35 = (char *)(a1 + (long)v36);
  v27 = v35;
  if (*v35 == '/') {
    do {
      v27 = &v27[1];
    } while (*v27 == '/');
  }
  v18 = &v20;
  v13 = v27;
  do {
    *(unsigned long *)&v25[v3 + -8] = 0x4dda;
    v13 = strchr(v13,0x2f);
    if (!v13)
      return 1;
    *v13 = '\0';
    *(unsigned long *)&v25[v3 + -8] = 0x4dfd;
    v8 = fstatat(a2,v27,v33,0);
    if ((v8) || (*(unsigned int *)((long)v6 + 0x1c) & 0xffffff00)) {
      *(unsigned long *)&v25[v3 + -8] = 0x4ed3;
      if (stat(v35,v18)) { // branch-flip
        *(unsigned long *)&v25[v3 + -8] = 0x4ee0;
        v28 = *__errno_location();
        if (v28) {
label_4c55:
          v36 = (void *)CONCAT44(v36._4_4_,v28);
          *(unsigned long *)&v25[v3 + -8] = 0x512c;
          v27 = (char *)sub_14b40(4,v35);
          *(unsigned long *)&v25[v3 + -8] = 0x5142;
          v13 = dcgettext(NULL,"failed to get attributes of %s",5);
          *(unsigned long *)&v25[v3 + -8] = 0x5157;
          error(0,(int)v36,v13,v27);
          return 0;
        }
      }
      else if ((v20._24_4_ & 0xf000) != 0x4000) {
        v28 = 0x14;
        goto label_4c55;
      }
      *(unsigned long *)&v25[v3 + -8] = 0x4ef6;
      v14 = (unsigned long *)sub_16e90(0xa8);
      *v14 = v20._0_8_;
      v14[1] = v20._8_8_;
      v14[2] = v20._16_8_;
      v14[3] = CONCAT44(v20._28_4_,v20._24_4_);
      v14[4] = v20._32_8_;
      v14[5] = v20._40_8_;
      v14[6] = v20._48_8_;
      v14[7] = v20._56_8_;
      v14[8] = v20._64_8_;
      v14[9] = v20._72_8_;
      v14[10] = v20._80_8_;
      v14[0xb] = v20._88_8_;
      v14[0xc] = v20._96_8_;
      v14[0xd] = v20._104_8_;
      v14[0xe] = v20._112_8_;
      v14[0xf] = v20._120_8_;
      v14[0x10] = v20._128_8_;
      v14[0x11] = v20._136_8_;
      v14[0x13] = (long)v13 - (long)v36;
      v2 = *v32;
      *(char *)&v14[0x12] = 0;
      v14[0x14] = v2;
      *v32 = (long)v14;
      if (!v8) goto label_4e17;
      v1 = *(unsigned int *)&v14[3];
      v31 = v14;
      *(unsigned long *)&v25[v3 + -8] = 0x4fb7;
      v7 = sub_7ff0(v35,v36,v1,1,v6);
      if (!v7)
        return 0;
      *v34 = '\x01';
      v9 = *(unsigned int *)&v31[3];
      if (*(char *)((long)v6 + 0x1d)) { // branch-flip
        v29 = v9 & 0x3f;
        v16 = ~v29;
      }
      else if (*(char *)((long)v6 + 0x1e)) { // branch-flip
        v29 = v9 & 0x12;
        v16 = ~v29;
      }
      else {
        v16 = 0xffffffff;
        v29 = 0;
      }
      if (*(char *)((long)v6 + 0x20))
        v9 = 0x1ff;
      *(unsigned long *)&v25[v3 + -8] = 0x5014;
      if (mkdirat(a2,v27,v9 & v16 & 0xfff)) {
        *(unsigned long *)&v25[v3 + -8] = 0x52a4;
        v15 = sub_14b40(4,v36);
        v27 = "cannot make directory %s";
        goto label_4c55;
      }
      if (v30) {
        *(unsigned long *)&v25[v3 + -8] = 0x5050;
        __printf_chk(1,v30,v35,v36);
      }
      *(unsigned long *)&v25[v3 + -8] = 0x5075;
      if (fstatat(a2,v27,v33,0x100)) {
        *(unsigned long *)&v25[v3 + -8] = 0x5251;
        v15 = sub_14b40(4,v36);
        v27 = "failed to get attributes of %s";
        goto label_4c55;
      }
      v9 = v19._24_4_;
      if (!*(char *)((long)v6 + 0x1e)) {
        if (~v19._24_4_ & v29) { // branch-flip
          *(unsigned long *)&v25[v3 + -8] = 0x51b1;
          v29 &= ~sub_8640();
          v14 = v31;
          v9 = v19._24_4_;
          if (!(~v19._24_4_ & v29)) goto label_509f;
        }
        else {
label_509f:
          v14 = v31;
          v9 = v19._24_4_;
          if ((v19._24_4_ & 0x1c0) == 0x1c0) goto label_50be;
        }
        *(char *)&v14[0x12] = 1;
        *(unsigned int *)&v14[3] = v29 | v9;
      }
label_50be:
      if ((v9 | 0x1c0) != v9) {
        *(unsigned long *)&v25[v3 + -8] = 0x50d5;
        if (sub_e520(a2,v27)) {
          *(unsigned long *)&v25[v3 + -8] = 0x52c6;
          v15 = sub_14b40(4,v36);
          v27 = "setting permissions for %s";
label_4c55:
          *(unsigned long *)&v25[v3 + -8] = 0x5267;
          v27 = dcgettext(NULL,v27,5);
          *(unsigned long *)&v25[v3 + -8] = 0x526f;
          v8 = *__errno_location();
          *(unsigned long *)&v25[v3 + -8] = 0x5280;
          error(0,v8,v27,v15);
          return 0;
        }
      }
      if (!*v34) goto label_4e59;
    }
    else {
label_4e17:
      *(unsigned long *)&v25[v3 + -8] = 0x4e31;
      v7 = sub_7ff0(v35,v36,0,0,v6);
      if (!v7)
        return 0;
      if ((v19._24_4_ & 0xf000) != 0x4000) {
        v12 = v36;
        goto label_4c55;
      }
      *v34 = '\0';
label_4e59:
      if ((*(long *)((long)v6 + 0x28)) || (*(char *)((long)v6 + 0x33))) {
        *(unsigned long *)&v25[v3 + -8] = 0x4e76;
        v7 = sub_8250(v36,0,v6);
        if ((!v7) && (*(char *)((long)v6 + 0x34)))
          return 0;
      }
    }
    v7 = v13[1];
    *v13 = '/';
    v13 = &v13[1];
    while (v7 == '/') {
      v13 = &v13[1];
      v7 = *v13;
    }
  } while( true );
}

// Function: sub_52e0 @ 0x52e0
unsigned long sub_52e0(char *a0,int a1,unsigned long a2,void *a3,void *a4) // return-dupe
{
  char v1;
  char *v10;
  unsigned long v11; // rdx
  unsigned long *v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long *v14;
  char *v15; // rsp
  unsigned long *v16; // rsp
  char v17 [16];
  unsigned long v18; // stack - 0x70
  unsigned long v19; // stack - 0x60
  int v2;
  unsigned long v20; // stack - 0x58
  unsigned long v21; // stack - 0x50
  unsigned int v3;
  unsigned int v4;
  long v5;
  unsigned long *v6;
  unsigned long v7; // rax
  char *v8; // rax
  unsigned long v9; // rax
  
  v14 = &v12;
  v18 = a2;
  v7 = strlen(a0);
  v16 = &v12;
  v6 = &v12;
  while (v16 != (unsigned long *)((long)&v12 - (v7 + 0x18 & 0xfffffffffffff000))) {
    v14 = (char *)((long)v6 + -0x1000);
    v15 = (char *)((long)v6 + -0x1000);
    *(unsigned long *)((long)v6 + -8) = *(unsigned long *)((long)v6 + -8);
    v16 = (char *)((long)v6 + -0x1000);
    v6 = (unsigned long *)v15;
  }
  v11 = (unsigned long)((unsigned int)(v7 + 0x18) & 0xff0);
  v5 = -v11;
  if (v11)
    *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
  *(unsigned long *)((long)v14 + v5 + -8) = 0x5370;
  v8 = memcpy((void *)((unsigned long)((long)v14 + v5 + 0xf) & 0xfffffffffffffff0),a0,v7 + 1);
  v10 = &v8[v18 - (long)a0];
  if (!a3)
    return 1;
  v12 = &v13;
  do {
    v1 = *(char *)((long)a4 + 0x1f);
    v8[*(long *)((long)a3 + 0x98)] = '\0';
    if (v1) {
      *(unsigned long *)((long)v14 + v5 + -8) = 0x53f5;
      v17 = sub_15960(a3);
      v19 = SUB168(v17,8);
      v13 = SUB168(v17,0);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5405;
      v17 = sub_15980(a3);
      v21 = SUB168(v17,8);
      v20 = SUB168(v17,0);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x541e;
      if (utimensat(a1,v10,v12,0)) {
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5433;
        v9 = sub_14b40(4,v8);
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5449;
        v10 = dcgettext(NULL,"failed to preserve times for %s",5);
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5451;
        v2 = *__errno_location();
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5462;
        error(0,v2,v10,v9);
        return 0;
      }
    }
    if (*(char *)((long)a4 + 0x1d)) { // branch-flip
      v4 = *(unsigned int *)((long)a3 + 0x20);
      v3 = *(unsigned int *)((long)a3 + 0x1c);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5481;
      if (!sub_e540(a1,v10,v3,v4)) goto label_53a3;
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5491;
      if (!sub_8400(a4)) {
        v18 &= 0xffffffffffffff00;
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5592;
        v9 = sub_14b40(4,v8);
        v10 = "failed to preserve ownership for %s";
        goto label_5526;
      }
      v4 = *(unsigned int *)((long)a3 + 0x20);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x54ac;
      sub_e540(a1,v10,0xffffffff,v4);
      v1 = *(char *)((long)a4 + 0x1e);
    }
    else {
label_53a3:
      v1 = *(char *)((long)a4 + 0x1e);
    }
    if (v1) { // branch-flip
      v4 = *(unsigned int *)((long)a3 + 0x18);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x54d9;
      if (sub_caa0(v10,0xffffffff,v8,0xffffffff,v4))
        return 0;
    }
    else if (*(char *)((long)a3 + 0x90)) {
      v4 = *(unsigned int *)((long)a3 + 0x18);
      v18 &= 0xffffffffffffff00;
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5502;
      if (sub_e520(a1,v10,v4)) {
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5517;
        v9 = sub_14b40(4,v8);
        v10 = "failed to preserve permissions for %s";
label_5526:
        *(unsigned long *)((long)v14 + v5 + -8) = 0x552d;
        v10 = dcgettext(NULL,v10,5);
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5535;
        v2 = *__errno_location();
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5546;
        error(0,v2,v10,v9);
        return v18 & 0xff;
      }
    }
    v8[*(long *)((long)a3 + 0x98)] = '/';
    a3 = *(void **)((long)a3 + 0xa0);
    if (!a3)
      return 1;
  } while( true );
}

// Function: sub_55b0 @ 0x55b0
void sub_55b0(void)
{
  FILE *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x50
  char *v16 [4]; // stack - 0xa8
  int v2; // eax
  char *v3; // rax
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7; // stack - 0xb0
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v6 = "[";
  v4 = &v5;
  v5 = "[";
  v7 = "test invocation";
  v16[0] = "coreutils";
  v16[1] = "Multi-call invocation";
  v16[2] = "sha224sum";
  v8 = "sha256sum";
  v16[3] = "sha2 utilities";
  v9 = "sha2 utilities";
  v10 = "sha384sum";
  v11 = "sha2 utilities";
  v12 = "sha512sum";
  v13 = "sha2 utilities";
  v14 = 0;
  v15 = 0;
  do {
    if (!strcmp("cp",v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!v6)
    v6 = "cp";
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "cp";
  if (!strcmp("cp","["))
    v3 = "test";
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == "cp")
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_57e0 @ 0x57e0
void sub_57e0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_255a0;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",5),v2,v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n",5),v1);
    sub_4970();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don\'t copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --no-preserve=ATTR_LIST  don\'t preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n",5),v1);
    sub_49a0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n",5),v1);
    sub_55b0();
  }
  exit(a0); // no-return
}

// Function: sub_5ae0 @ 0x5ae0
unsigned long sub_5ae0(int a0,unsigned long *a1,long a2,bool a3,int *a4)
{
  int *v1;
  unsigned int v10; // eax
  int v11;
  unsigned int v12; // eax
  char *v13;
  unsigned long v14; // rax
  void *v15; // rax
  unsigned long v16; // rax
  unsigned long v17; // rax
  unsigned long v18; // rax
  char *v19;
  int *v2;
  unsigned long v20; // rax
  char *v21; // rcx
  unsigned long v22; // rdx
  long v23;
  unsigned char *v24; // stack - 0x128
  stat v25; // stack - 0xd8
  char *v26; // stack - 0xe0
  long v27;
  unsigned char v28; // stack - 0xea
  void *v29; // stack - 0xe8
  void *v3;
  char *v30;
  char *v31; // rsp
  unsigned char **v32;
  char *v33; // rsp
  int *v34;
  int *v35;
  long v36;
  unsigned int v37; // eax
  char **v38; // stack - 0x120
  char *v39; // stack - 0x118
  long v4;
  long v40; // stack - 0x110
  int *v41; // stack - 0x108
  unsigned int v42; // stack - 0x100
  int v43; // stack - 0xfc
  char *v5;
  void *v6;
  unsigned char v7;
  char v8; // al
  char v9;
  
  v32 = &v24;
  v28 = 0;
  v40 = a2; // branch-flip
  v41 = a4;
  v43 = a0;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (a0 == 1) {
      v18 = sub_14b40(4,(char *)*a1);
      v19 = "missing destination file operand after %s";
label_60c1:
      error(0,0,dcgettext(NULL,v19,5),v18);
      sub_57e0(1); // return-dupe, no-return
    }
    v19 = "missing file operand";
  }
  else {
    v25._24_4_ = 0;
    if (a3) { // branch-flip
      if (a2) {
        error(1,0,dcgettext(NULL,"cannot combine --target-directory (-t) and --no-target-directory (-T)",5));
        return v17;
      }
      if (2 < a0) {
        v18 = sub_14b40(4,(char *)a1[2]);
        v19 = "extra operand %s";
        goto label_60c1;
      }
    }
    else {
      if (a2) {
        v10 = sub_15a20(a2,&v25);
        v42 = v10;
        if (!sub_15a10(v10)) {
          v18 = sub_14b40(4,v40);
          v19 = dcgettext(NULL,"target directory %s",5);
          error(1,*__errno_location(),v19,v18);
          return v20;
        }
        if (v43 != 1) {
label_5f87:
          v34 = v41;
          sub_8330(v41);
          sub_8370(v34);
        }
label_5b81:
        v38 = &v26;
        v23 = 0;
        v37 = 1;
        v39 = &v27;
        v24 = &v28;
        do {
          v19 = (char *)a1[v23];
          v26 = NULL;
          if (dat_25158) { // branch-flip
            *(unsigned long *)((long)v32 + -8) = 0x5e08;
            sub_e800(v19);
            if (!dat_25159) goto label_5be5;
label_5e18:
            *(unsigned long *)((long)v32 + -8) = 0x5e1d;
            v14 = strlen(v19);
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v5 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v5[-0x1000];
              v33 = &v5[-0x1000];
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
              v32 = (char *)&v5[-0x1000];
              v5 = v33;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v4 = -v22;
            v32 = (char *)&v30[v4];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v4 + -8] = 0x5e7a;
            v13 = memcpy((void *)((unsigned long)&v30[v4 + 0xf] & 0xfffffffffffffff0),v19,v14 + 1);
            *(unsigned long *)&v30[v4 + -8] = 0x5e85;
            sub_e800(v13);
            *(unsigned long *)&v30[v4 + -8] = 0x5e9b;
            v36 = v40;
            v15 = (void *)sub_ec90(v40,v13,v38);
            v13 = NULL;
            v11 = v41[0xf];
            *(long *)&v30[v4 + -8] = v36;
            if ((char)v11)
              v13 = "%s -> %s\n";
            *(int **)&v30[v4 + -0x10] = v41;
            *(unsigned long *)&v30[v4 + -0x18] = 0x5ede;
            v8 = sub_4bf0(v15,(long)v26 - (long)v15,v42,v13,&v29,v24);
            v9 = *v26;
            while (v9 == '/') {
              v26 = &v26[1];
              v9 = *v26;
            }
            if (v8) goto label_5c8c;
            v37 = 0;
            v32 = (unsigned char **)&v30[v4];
            v9 = dat_25159;
label_5f1b:
            v6 = v29;
            if (v9) {
              while (v6) {
                v3 = *(void **)((long)v6 + 0xa0);
                *(unsigned long *)((long)v32 + -8) = 0x5f4e;
                v29 = v3;
                free(v6);
                v6 = v3;
              }
            }
          }
          else {
            if (dat_25159) goto label_5e18;
label_5be5:
            *(unsigned long *)((long)v32 + -8) = 0x5bea;
            v13 = (char *)sub_db60(v19);
            *(unsigned long *)((long)v32 + -8) = 0x5bf5;
            v14 = strlen(v13);
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v5 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v5[-0x1000];
              v31 = &v5[-0x1000];
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
              v32 = (char *)&v5[-0x1000];
              v5 = v31;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v4 = -v22;
            v32 = (char *)&v30[v4];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v4 + -8] = 0x5c52;
            v13 = memcpy((void *)((unsigned long)&v30[v4 + 0xf] & 0xfffffffffffffff0),v13,v14 + 1);
            *(unsigned long *)&v30[v4 + -8] = 0x5c5d;
            sub_e800(v13);
            *(unsigned long *)&v30[v4 + -8] = 0x5c6c;
            v11 = strcmp(v13,"..");
            *(unsigned long *)&v30[v4 + -8] = 0x5c89;
            v15 = (void *)sub_ec90(v40,&v13[v11 == 0],v38);
label_5c8c:
            *(unsigned long *)((long)v32 + -8) = 0;
            *(char **)((long)v32 + -0x10) = v39;
            *(unsigned long *)((long)v32 + -0x18) = 0x5cbb;
            v12 = sub_bae0(v19,v15,v42,v26,v28,v41);
            v37 &= v12;
            if (dat_25159) {
              *(unsigned long *)((long)v32 + -8) = 0x5f7b;
              v12 = sub_52e0(v15,v42,v26,v29,v41);
              v37 &= v12;
              v9 = dat_25159;
              goto label_5f1b;
            }
          }
          v23 += 1;
          *(unsigned long *)((long)v32 + -8) = 0x5cd9;
          free(v15);
        } while ((int)v23 < v43);
        return (unsigned long)v37; // return-dupe
      }
      v23 = a1[(long)a0 + -1];
      v40 = v23;
      v42 = sub_15a20(v23,&v25);
      v37 = sub_15a10(v42);
      if ((char)v37) { // branch-flip
        v43 = a0 + -1;
        if (v23) {
          if (1 < v43) goto label_5f87;
          if (v43 != 1) {
            return (unsigned long)v37;
          }
          goto label_5b81;
        }
      }
      else {
        v11 = *__errno_location();
        if (v11 == 2)
          v28 = 1;
        if (3 <= v43) {
          v19 = (char *)sub_14b40(4,v40);
          error(1,v11,dcgettext(NULL,"target %s",5),v19);
          return v16;
        }
      }
    }
    v7 = v28;
    v19 = (char *)*a1;
    v13 = (char *)a1[1];
    if (!dat_25159) {
      if (((((*(char *)((long)v41 + 0x16)) && (*v41)) && (!strcmp(v19,v13))) && ((!v7 && ((v25._24_4_ || (!stat(v13,&v25))))))) && (v34 = v41, (v25._24_4_ & 0xf000) == 0x8000)) {
        v13 = (char *)sub_daa0(0xffffff9c,v13,*v41);
        v23 = 0x16;
        v35 = (int *)0x25100;
        while (v23) {
          v2 = &v35[1];
          v1 = &v34[1];
          *v35 = *v34;
          v23 -= 1;
          v34 = v1;
          v35 = v2;
        }
        dat_25100 = 0;
        v41 = (int *)0x25100;
      }
      v37 = sub_bae0(v19,v13,0xffffff9c,v13,-(unsigned int)v7,v41,&v26,NULL);
      return (unsigned long)v37;
    }
    v19 = "with --parents, the destination must be a directory";
  }
  error(0,0,dcgettext(NULL,v19,5));
  sub_57e0(1);
}

// Function: sub_6190 @ 0x6190
bool sub_6190(char *a0)
{
  if (*a0 != '.')
    return 0;
  return !a0[(unsigned long)(a0[1] == '.') + 1] || a0[(unsigned long)(a0[1] == '.') + 1] == '/';
}

// Function: sub_61c0 @ 0x61c0
bool sub_61c0(int a0)
{
  return a0 == 0x5f;
}

// Function: sub_61d0 @ 0x61d0
unsigned int sub_61d0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x5f) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 0x3d);
}

// Function: sub_61e0 @ 0x61e0
unsigned long sub_61e0(struct_7 *a0,long *a1) // return-dupe
{
  if (!a1)
    return 0;
  while ((a1[1] != a0->field_0x8 || (a1[2] != a0->field_0x0))) {
    a1 = (long *)*a1;
    if (!a1)
      return 0;
  }
  return 1;
}

// Function: sub_6220 @ 0x6220
void sub_6220(void)
{
  return;
}

// Function: sub_6230 @ 0x6230
int * sub_6230(void *a0)
{
  int v1;
  unsigned long v2;
  bool v3; // al
  unsigned int v4; // eax
  int *v5; // rax
  undefined7 v6; // rax
  
  v5 = __errno_location();
  v1 = *v5;
  v2 = (unsigned long)v5 >> 8;
  v6 = (undefined7)v2;
  v3 = v1 == 1 || v1 == 0x16;
  if (v1 == 1 || v1 == 0x16) {
    v4 = (unsigned int)*(unsigned char *)((long)a0 + 0x1b);
    v4 ^= 1;
    v5 = (unsigned long)(unsigned long)v4;
  }
  return v5;
}

// Function: sub_6260 @ 0x6260
void sub_6260(void)
{
  sub_14b40(4); // tail-call
}

// Function: sub_6270 @ 0x6270
unsigned long sub_6270(struct_26 *a0)
{
  if (4 <= a0->field_0x0)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (3 <= a0->field_0xc - 1)
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (2 < a0->field_0x44)
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((a0->field_0x17) && (a0->field_0x3a))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a0->field_0x44 == 2) && (a0->field_0xc != 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  return 1;
}

// Function: sub_6360 @ 0x6360
void sub_6360(int a0,char *a1,stat *a2,int a3)
{
  fstatat(a0,a1,a2,a3); // tail-call
}

// Function: sub_6370 @ 0x6370
unsigned long sub_6370(int a0,char *a1,unsigned int a2)
{
  int v1; // eax
  
  if ((a2 & 0xf000) == 0xa000)
    return 1;
  if (sub_16e20())
    return 1;
  v1 = faccessat(a0,a1,2,0x200);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_63e0 @ 0x63e0
void sub_63e0(void *a0,char *a1,unsigned int a2,char *a3,struct_19 *a4)
{
  unsigned int v1;
  unsigned int v2;
  char *v3;
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  char v7 [9];
  char v8; // stack - 0x32
  
  v1 = a4->field_0x18;
  if (sub_6370(a2,a3,v1)) { // branch-flip
    v5 = (char *)sub_14b40(4,a1);
    v3 = dat_255a0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: overwrite %s? ",5),v3,v5);
  }
  else {
    sub_eb60(a4->field_0x18,&v6);
    v2 = a4->field_0x18;
    v8 = 0;
    v4 = sub_14b40(4,a1);
    v3 = dat_255a0;
    if ((*(char *)((long)a0 + 0x18)) || (*(unsigned int *)((long)a0 + 0x14) & 0xffff00))
      v5 = dcgettext(NULL,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    else {
      v5 = dcgettext(NULL,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    __fprintf_chk(stderr,1,v5,v3,v4,(unsigned long)(v2 & 0xfff),v7);
  }
  sub_17540(); // tail-call
}

// Function: sub_6530 @ 0x6530
unsigned int sub_6530(void *a0,char *a1,unsigned int a2,char *a3,struct_19 *a4) // early-return
{
  int v1;
  unsigned int v2;
  
  if (!*(unsigned char *)((long)a0 + 0x18))
    __assert_fail("x->move_mode","src/copy.c",0x710,"abandon_move"); // no-return
  v1 = *(int *)((long)a0 + 8);
  if (v1 == 2)
    return (unsigned int)*(unsigned char *)((long)a0 + 0x18);
  if ((v1 != 3) && (((v1 != 4 || (!*(char *)((long)a0 + 0x3d))) || (v2 = a4->field_0x18, sub_6370(a2,a3,v2)))))
    return 0;
  return sub_63e0(a0,a1,a2,a3,a4) ^ 1;
}

// Function: sub_65e0 @ 0x65e0
void sub_65e0(void *a0,long a1,char *a2)
{
  unsigned long v1;
  void *v2; // rax
  unsigned long v3; // rsi
  
  v3 = a1 - (long)a0;
  v1 = strlen(a2) + 1;
  v2 = (void *)sub_16eb0(v1 + v3);
  memcpy((void *)((long)v2 + v3),a2,v1);
  memcpy(v2,a0,v3); // tail-call
}

// Function: sub_6640 @ 0x6640
bool sub_6640(char *a0,struct_7 *a1,int a2,char *a3)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  char *v5;
  stat v6; // stack - 0xd8
  bool v7;
  
  v7 = 0;
  v2 = strlen(a0);
  v3 = (char *)sub_db60(a3);
  v4 = strlen(v3);
  v5 = dat_25580;
  if (((strlen(dat_25580) + v4 == v2) && (!memcmp(a0,v3,v4))) && (!strcmp(&a0[v4],v5))) {
    v5 = (char *)sub_65e0(a3,&a3[strlen(a3)],v5);
    v1 = fstatat(a2,v5,&v6,0);
    free(v5);
    v7 = 0;
    if ((!v1) && (a1->field_0x8 == v6._8_8_))
      v7 = a1->field_0x0 == v6._0_8_;
  }
  return v7;
}

// Function: sub_6770 @ 0x6770
void sub_6770(int a0,unsigned long a1)
{
  ioctl(a0,0x40049409,a1 & 0xffffffff); // tail-call
}

// Function: sub_6780 @ 0x6780
int sub_6780(unsigned long a0,void *a1,long *a2)
{
  int v1;
  long v2;
  long v3;
  int v4; // r12d
  
  v4 = 1;
  if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x8000) {
    v3 = *(long *)((long)a1 + 0x30);
    v2 = v3 + 0x1ff;
    if (0 <= v3)
      v2 = v3;
    if (*(long *)((long)a1 + 0x40) < v2 >> 9) {
      v4 = 3;
      v3 = lseek(a0,0,3);
      *a2 = v3;
      if (v3 <= -1) {
        v1 = *__errno_location();
        if ((v1 != 6) && (v4 = 2, v1 != 0x16))
          return (unsigned int)(unsigned char)sub_61c0(v1) * 2;
      }
    }
  }
  return v4;
}

// Function: sub_6820 @ 0x6820
unsigned long sub_6820(unsigned int a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if ((!dat_25560) && (dat_25560 = calloc(dat_25018,1), !dat_25560)) {
    dat_25018 = 0x400;
    dat_25560 = (void *)0x25160;
  }
  if (!a1)
    return 1;
  while( true ) {
    v2 = dat_25018;
    if (a1 <= dat_25018)
      v2 = a1;
    v1 = sub_11320(a0,dat_25560,v2);
    if (v1 != v2) break;
    a1 -= v1;
    if (!a1)
      return 1;
  }
  return 0;
}

// Function: sub_68d0 @ 0x68d0
int sub_68d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1; // al
  int v2; // eax
  int v3; // edx
  
  v2 = fallocate(a0,3,a1,a2);
  if (v2 < 0) {
    v1 = sub_61c0(*__errno_location());
    if ((v3 == 0x26) || (v1))
      return 0;
  }
  return v2;
}

// Function: sub_6910 @ 0x6910
unsigned long sub_6910(unsigned long a0,char *a1,bool a2,long a3)
{
  long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = lseek(a0,a3,1);
  if (v1 < 0) {
    v2 = sub_14b40(4,a1);
    v3 = "cannot lseek %s";
label_69a4:
    v3 = dcgettext(NULL,v3,5);
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  if (a2) {
    if ((int)sub_68d0(a0 & 0xffffffff,v1 - a3,a3) <= -1) {
      v2 = sub_14b40(4,a1);
      v3 = "error deallocating %s";
      goto label_69a4;
    }
  }
  return 1;
}

// Function: sub_69e0 @ 0x69e0
unsigned long sub_69e0(char *a0,unsigned long a1)
{
  int v1; // eax
  char *v2;
  
  if (!a1)
    return 1;
  v2 = a0;
  while( true ) {
    if (*v2)
      return 0;
    v2 = &v2[1];
    a1 -= 1;
    if (!a1) break;
    if (!(a1 & 0xf)) {
      v1 = memcmp(a0,v2,a1);
      return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
    }
  }
  return 1;
}

// Function: sub_6a40 @ 0x6a40
int * sub_6a40(int a0,unsigned long a1,long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,char *a7,char *a8,unsigned long a9,long *a10,char *a11) // return-dupe x2
{
  bool v1;
  char *v10;
  char *v11;
  char *v12;
  unsigned long v13; // rax
  unsigned long v14;
  int *v15; // rdx
  unsigned long v16;
  int *v17;
  int *v18;
  int v19; // esi
  long *v2;
  unsigned char v20;
  char *v21;
  unsigned char v22; // r13b
  unsigned long v23;
  unsigned long v24; // stack - 0xb0
  unsigned long v25; // stack - 0x98
  char *v3;
  unsigned int v4;
  unsigned int v5; // eax
  long v6; // rax
  long v7;
  int *v8; // rax
  unsigned long v9;
  
  v3 = a11;
  v2 = a10;
  v10 = a8;
  v11 = a7;
  v19 = (int)a1;
  *a11 = 0;
  v25 = a9;
  *a10 = 0;
  if ((!a4) && ((char)a6)) {
    if (!a9)
      return (int *)0x1;
    v17 = (int *)(a1 & 0xffffffff);
    do {
      while( true ) {
        v14 = 0x7fffffffc0000000;
        if (v25 <= 0x7fffffffc0000000)
          v14 = v25;
        v6 = copy_file_range(a0,NULL,v19,NULL,v14,0);
        if (!v6) {
          if (*v2)
            return (int *)0x1;
          goto label_6b30;
        }
        if (v6 < 0) break;
        *v2 = *v2 + v6;
        v25 -= v6;
        if (!v25)
          return (int *)0x1;
      }
      v4 = *__errno_location();
      if ((v4 == 0x26) || (v5 = sub_61c0(v4), (char)v5)) goto label_6b30;
      if (v4 <= 0x1a) {
        if (1L << ((unsigned char)v4 & 0x3f) & 0x4440200U) goto label_6b30;
        if (v4 == 1) {
          v17 = v15;
          if (!*v2) goto label_6b30;
          goto label_6bda;
        }
      }
      if (v4 != 4) {
label_6bda:
        v10 = (char *)sub_14a70(1,4,v10);
        v11 = (char *)sub_14a70(0,4,v11);
        v12 = dcgettext(NULL,"error copying %s to %s",5);
        error(0,*v15,v12,v11,v10);
        return (int *)(unsigned long)v5;
      }
    } while( true );
  }
  if (!a9)
    return (int *)0x1;
label_6b30:
  v14 = a3;
  if (a4)
    v14 = a4;
  v23 = 0;
  v18 = NULL;
  do {
    while( true ) {
      v12 = (char *)*a2;
      if (!v12) {
        v12 = (char *)sub_16e60((long)getpagesize(),a3);
        *a2 = (long)v12;
      }
      v7 = v25;
      if (a3 <= v25)
        v7 = a3;
      v7 = read(a0,v12,v7);
      if (0 <= v7) break;
      v8 = __errno_location();
      if (*v8 != 4) {
        v11 = (char *)sub_14b40(4,v11);
        v10 = dcgettext(NULL,"error reading %s",5);
        error(0,*v8,v10,v11);
        return NULL;
      }
    }
    if (!v7) {
label_6fc8:
      if ((char)v18)
        return (int *)sub_6910(v19,v10,(unsigned char)a5,v23); // tail-call
      return (int *)0x1;
    }
    *v2 = *v2 + v7;
    v9 = v14;
    v21 = v12;
    v24 = v7;
label_6c20:
    do {
      v16 = v9;
      if (v24 < v9)
        v16 = v24;
      v22 = (unsigned char)v18;
      if (v16 && a4) {
        v9 = sub_69e0(v12,v16);
        v17 = (int *)(v9 & 0xffffffff);
        v20 = ((unsigned char)v9 ^ v22) & v23 != 0;
        if (((char)v17 == '\x01') || (v16 != v24)) {
          if (!v20) goto label_6d71;
          v1 = 0;
        }
        else {
          if (!v20) {
            v17 = NULL;
            goto label_6d13;
          }
          v1 = 1;
          v17 = NULL;
        }
label_6d2c:
        if (v22) { // branch-flip
          v4 = sub_6910(v19,v10,(unsigned char)a5,v23);
          if (!(char)v4)
            return (int *)(unsigned long)v4;
        }
        else if (v23 != sub_11320(v19,v21,v23)) {
          v13 = sub_14b40(4,v10);
          v11 = dcgettext(NULL,"error writing %s",5);
          error(0,*__errno_location(),v11,v13);
          return v18;
        }
        v21 = v12;
        v23 = v16;
        if (v1) {
          if (v16) { // branch-flip
            if (v20) {
              v18 = (int *)((unsigned long)v17 & 0xffffffff);
              v9 = 0;
              goto label_6c20;
            }
          }
          else {
            if (v20) {
              v23 = 0;
              break;
            }
            v24 = 0;
          }
          v24 -= v16;
          v18 = (int *)((unsigned long)v17 & 0xffffffff);
          v23 = 0;
        }
        else {
          v24 -= v16;
          v18 = (int *)((unsigned long)v17 & 0xffffffff);
        }
      }
      else {
        if (((v22 != 1) && (v17 = (int *)CONCAT71((undefined7)((unsigned long)v17 >> 8),v16 && a4), v16 == v24)) || (v17 = v18, !v16)) {
label_6d13:
          v23 += v16;
          v1 = 1;
          v20 = 0;
          goto label_6d2c;
        }
label_6d71:
        v23 += v16;
        if (v23 + 0x8000000000000000 < v16) {
          v11 = (char *)sub_14b40(4,v11);
          error(0,0,dcgettext(NULL,"overflow reading %s",5),v11);
          return NULL;
        }
        v24 -= v16;
        v18 = v17;
      }
      v12 = &v12[v16];
      v9 = v16;
    } while (v24);
    v25 -= v7;
    *v3 = (char)v17;
    if (!v25) {
      v18 = v17;
      goto label_6fc8;
    }
    v18 = (int *)((unsigned long)v17 & 0xffffffff);
  } while( true );
}

// Function: sub_7020 @ 0x7020
void sub_7020(int a0,unsigned int a1,char *a2,unsigned int a3)
{
  if (a0 <= -1) {
    sub_e520(a1,a2,a3); // tail-call
    return;
  }
  fchmod(a0,a3); // tail-call
}

// Function: sub_7040 @ 0x7040
unsigned long sub_7040(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,void *a4)
{
  char v1;
  void *v10; // stack - 0x18
  int v2; // eax
  unsigned long v3; // rax
  undefined7 v4; // rax
  void *v5; // stack - 0x28
  void *v6; // r8
  void **v7;
  bool v8; // zf
  void *v9; // stack - 0x20
  
  v7 = &v5;
  v1 = *(char *)((long)a4 + 0x33);
  if ((*(char *)((long)a4 + 0x31)) && (!*(char *)((long)a4 + 0x36))) {
    if (*(char *)((long)a4 + 0x37)) {
      v6 = sub_7330;
      v7 = NULL;
      if (!v1) {
        v6 = NULL;
        v7 = NULL;
        if (*(long *)((long)a4 + 0x28)) {
          v6 = sub_7330;
          v7 = NULL;
        }
      }
      goto label_70a2;
    }
    v6 = sub_7330;
    v5 = sub_7180;
    if ((!v1) && (!*(long *)((long)a4 + 0x28)))
      v6 = NULL;
  }
  else {
    v6 = sub_7330;
    v5 = sub_7260;
    if (!v1) {
      v6 = sub_7330;
      if (!*(long *)((long)a4 + 0x28))
        v6 = NULL;
      v5 = sub_7260;
    }
  }
  v9 = sub_6260;
  v10 = sub_6220;
label_70a2:
  v2 = (unsigned int)a1;
  v2 |= a3;
  if (0 <= v2) { // branch-flip
    v3 = attr_copy_fd(a0,a1,a2);
    v8 = v2 == 0;
    v4 = (undefined7)((unsigned long)v3 >> 8);
    v3 = CONCAT71(v4,v8);
  }
  else {
    v3 = attr_copy_file(a0,a2,v6,v7);
    v8 = v2 == 0;
    v4 = (undefined7)((unsigned long)v3 >> 8);
    v3 = CONCAT71(v4,v8);
  }
  return v3;
}

// Function: sub_7180 @ 0x7180
void sub_7180(void) // return-dupe
{
  int v1;
  
  v1 = *__errno_location();
  if (sub_61d0(v1))
    return;
  sub_16760(0);
}

// Function: sub_7260 @ 0x7260
void sub_7260(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,int *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  int *v2; // rax
  unsigned int v3; // stack - 0xd8
  char v4 [16];
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
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
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  v2 = __errno_location();
  v6 = &Stack0000000000000008;
  v3 = 0x10;
  v7 = v4;
  v5 = 0x30;
  sub_16760(0,*v2,a9,&v3);
}

// Function: sub_7330 @ 0x7330
int sub_7330(char *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = strncmp(a0,"security.selinux",0x10);
  if (!v1)
    return v1;
  return (unsigned long)(attr_copy_check_permissions(a0,a1) != 0);
}

// Function: sub_7380 @ 0x7380
void sub_7380(void)
{
  char *v1; // rax
  
  if (!setfscreatecon(NULL))
    return;
  v1 = dcgettext(NULL,"failed to restore the default file creation context",5);
  error(1,*__errno_location(),v1);
}

// Function: sub_73c0 @ 0x73c0
void sub_73c0(char *a0,char *a1,char *a2)
{
  char *v1; // rax
  
  v1 = (char *)sub_14a70(1,4,a1);
  __printf_chk(1,"%s -> %s",(char *)sub_14a70(0,4,a0),v1);
  if (a2) {
    v1 = (char *)sub_14b40(4,a2);
    __printf_chk(1,dcgettext(NULL," (backup: %s)",5),v1);
  }
  putchar_unlocked(10); // tail-call
}

// Function: sub_7460 @ 0x7460
unsigned int sub_7460(char *a0,unsigned int a1,char *a2,char *a3,unsigned int a4,unsigned long a5,char a6,unsigned int a7,unsigned char a8)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  unsigned char v5; // r12b
  unsigned int v6;
  unsigned long v7; // stack - 0x50
  
  v6 = a7;
  v1 = sub_c0c0(a1,a2,a4,a5,(unsigned long)a8 << 10,(unsigned char)a6);
  if (1 <= v1) { // branch-flip
    v4 = NULL;
    if (!a0) {
      v4 = (char *)sub_65e0(a3,a5,a2,v7);
      a0 = v4;
    }
    v2 = (char *)sub_14a70(1,4,a0);
    v3 = (char *)sub_14a70(0,4,a3);
    v6 = 0;
    error(0,v1,dcgettext(NULL,"cannot create hard link %s to %s",5),v3,v2);
    free(v4);
  }
  else {
    v5 = (unsigned char)v6 & (unsigned char)((unsigned int)v1 >> 0x1f);
    v6 = CONCAT31((undefined3)((unsigned int)v6 >> 8),v5);
    if (v5) { // branch-flip
      v4 = (char *)sub_14b40(4,a3,0xffffffffffffffff);
      __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v4);
    }
    else {
      v6 = 1;
    }
  }
  return v6;
}

// Function: sub_75b0 @ 0x75b0
unsigned int sub_75b0(char *a0,stat *a1,int a2,char *a3,stat *a4,struct_31 *a5,char *a6) // return-dupe
{
  int v1;
  stat *v10;
  bool v2;
  bool v3;
  char *v4;
  unsigned char v5;
  char *v6; // rax
  char v7;
  stat v8; // stack - 0x168
  stat v9; // stack - 0xd8
  
  v4 = a6;
  if ((*(long *)&a1->field_0x8 != *(long *)&a4->field_0x8) || (*(long *)a1 != *(long *)a4)) {
    v1 = a5->field_0x4;
    *a6 = 0;
    if (v1 != 2)
      return 1;
    v2 = 0;
    v3 = 0;
label_763e:
    v10 = a1;
    if (((*(unsigned int *)&a1->field_0x18 & 0xf000) == 0xa000) && ((*(unsigned int *)&a4->field_0x18 & 0xf000) == 0xa000)) {
      if (sub_15130(0xffffff9c,a0,a2,a3))
        return 0;
      if (a5->field_0x0)
        return 1;
      if (!v3)
        return 1;
      *v4 = 1;
      return (unsigned char)a5->field_0x18 ^ 1;
    }
  }
  else {
    *a6 = 0;
    if (a5->field_0x17) goto label_760a;
    if (a5->field_0x4 == 2) {
      v2 = 1;
      v3 = 1;
      goto label_763e;
    }
    a4 = &v8;
    if (fstatat(a2,a3,a4,0x100))
      return 1;
    v10 = &v9;
    if (lstat(a0,v10))
      return 1;
    v2 = v9._8_8_ == v8._8_8_ && v9._0_8_ == v8._0_8_;
    if ((((v9._24_4_ & 0xf000) == 0xa000) && ((v8._24_4_ & 0xf000) == 0xa000)) && (a5->field_0x15))
      return 1;
  }
  if (a5->field_0x0) {
    if (v2)
      return sub_15130(0xffffff9c,a0,a2,a3) ^ 1;
    if (a5->field_0x18)
      return 1;
    if (a5->field_0x4 == 2)
      return 1;
    if ((*(unsigned int *)&v10->field_0x18 & 0xf000) != 0xa000)
      return 1;
    if ((*(unsigned int *)&a4->field_0x18 & 0xf000) == 0xa000)
      return 1;
    return 0;
  }
  v7 = a5->field_0x18;
  if ((v7) || (a5->field_0x15)) {
    if ((*(unsigned int *)&a4->field_0x18 & 0xf000) == 0xa000)
      return 1;
    if ((v2) && (2 <= *(unsigned long *)&a4->field_0x10)) {
      if (!sub_15130(0xffffff9c,a0,a2,a3))
        return (unsigned char)a5->field_0x18 ^ 1;
      if ((*(unsigned int *)&v10->field_0x18 & 0xf000) != 0xa000) goto label_787c;
      goto label_788f;
    }
    if ((*(unsigned int *)&v10->field_0x18 & 0xf000) != 0xa000) goto label_77c9;
  }
  else {
    if ((*(unsigned int *)&v10->field_0x18 & 0xf000) == 0xa000) goto label_7900;
label_787c:
    if ((*(unsigned int *)&a4->field_0x18 & 0xf000) != 0xa000) {
label_77c9:
      if (*(long *)&v10->field_0x8 != *(long *)&a4->field_0x8)
        return 1;
      if (*(long *)v10 != *(long *)a4)
        return 1;
      if (a5->field_0x17) {
label_760a:
        *v4 = 1;
        return 1;
      }
    }
label_788f:
    v7 = a5->field_0x18;
  }
  if ((((v7) && ((*(unsigned int *)&a1->field_0x18 & 0xf000) == 0xa000)) && (2 <= *(unsigned long *)&a4->field_0x10)) && (v6 = canonicalize_file_name(a0), v6)) {
    v5 = sub_15130(0xffffff9c,v6,a2,a3);
    free(v6);
    return (unsigned int)(v5 ^ 1);
  }
label_7900:
  if ((a5->field_0x3a) && ((*(unsigned int *)&a4->field_0x18 & 0xf000) == 0xa000))
    return 1;
  if (a5->field_0x4 != 2)
    return 0;
  if ((*(unsigned int *)&v10->field_0x18 & 0xf000) != 0xa000) { // branch-flip
    v9._0_8_ = *(long *)v10;
    v9._8_8_ = *(long *)&v10->field_0x8;
    v9._16_8_ = *(unsigned long *)&v10->field_0x10;
    v9._24_8_ = *(unsigned long *)&v10->field_0x18;
    v9._32_8_ = *(unsigned long *)&v10->field_0x20;
    v9._40_8_ = *(unsigned long *)&v10->field_0x28;
    v9._48_8_ = *(unsigned long *)&v10->field_0x30;
    v9._56_8_ = *(unsigned long *)&v10->field_0x38;
    v9._64_8_ = *(unsigned long *)&v10->field_0x40;
    v9._72_8_ = *(unsigned long *)&v10->field_0x48;
    v9._80_8_ = *(unsigned long *)&v10->field_0x50;
    v9._88_8_ = *(unsigned long *)&v10->field_0x58;
    v9._96_8_ = *(unsigned long *)&v10->field_0x60;
    v9._104_8_ = *(unsigned long *)&v10->field_0x68;
    v9._112_8_ = *(unsigned long *)&v10->field_0x70;
    v9._120_8_ = *(unsigned long *)&v10->field_0x78;
    v9._128_8_ = *(unsigned long *)&v10->field_0x80;
    v9._136_8_ = *(unsigned long *)&v10->field_0x88;
  }
  else if (stat(a0,&v9))
    return 1;
  if ((*(unsigned int *)&a4->field_0x18 & 0xf000) != 0xa000) { // branch-flip
    v8._0_8_ = *(long *)a4;
    v8._8_8_ = *(long *)&a4->field_0x8;
    v8._16_8_ = *(unsigned long *)&a4->field_0x10;
    v8._24_8_ = *(unsigned long *)&a4->field_0x18;
    v8._32_8_ = *(unsigned long *)&a4->field_0x20;
    v8._40_8_ = *(unsigned long *)&a4->field_0x28;
    v8._48_8_ = *(unsigned long *)&a4->field_0x30;
    v8._56_8_ = *(unsigned long *)&a4->field_0x38;
    v8._64_8_ = *(unsigned long *)&a4->field_0x40;
    v8._72_8_ = *(unsigned long *)&a4->field_0x48;
    v8._80_8_ = *(unsigned long *)&a4->field_0x50;
    v8._88_8_ = *(unsigned long *)&a4->field_0x58;
    v8._96_8_ = *(unsigned long *)&a4->field_0x60;
    v8._104_8_ = *(unsigned long *)&a4->field_0x68;
    v8._112_8_ = *(unsigned long *)&a4->field_0x70;
    v8._120_8_ = *(unsigned long *)&a4->field_0x78;
    v8._128_8_ = *(unsigned long *)&a4->field_0x80;
    v8._136_8_ = *(unsigned long *)&a4->field_0x88;
  }
  else if (fstatat(a2,a3,&v8,0))
    return 1;
  if (v9._8_8_ == v8._8_8_) {
    if (v9._0_8_ != v8._0_8_)
      return 1;
    v5 = a5->field_0x17;
    if (!v5)
      return 0;
    *v4 = (*(unsigned int *)&a4->field_0x18 & 0xf000) != 0xa000;
    return (unsigned int)v5;
  }
  return 1;
}

// Function: sub_7b50 @ 0x7b50
unsigned long sub_7b50(unsigned int a0,unsigned int a1,long *a2,unsigned long a3,unsigned long a4,long a5,long a6,int a7,char a8,char *a9,char *a10) // return-dupe x2
{
  bool v1;
  long v10; // stack - 0x48
  char *v11;
  unsigned long v12;
  unsigned char v13; // r13b
  unsigned long v14; // r13
  long v15; // stack - 0xa0
  long v16; // stack - 0x90
  char v2;
  int *v3; // rax
  long v4;
  char *v5;
  unsigned long v6; // rax
  long v7; // rax
  long v8; // rbx
  unsigned char v9; // stack - 0x49
  
  v5 = a10;
  v11 = a9;
  v2 = a8;
  if (0 <= a5) { // branch-flip
    v14 = 1;
    v16 = 0;
    v7 = 0;
    v15 = 0;
    while( true ) {
      v8 = a5;
      v4 = v16;
      v16 = lseek(a0,v8,4);
      if (0 <= v16) { // branch-flip
        v4 = v16;
        if (v16 <= a6)
          v4 = a6;
        a6 = v4;
      }
      else {
        v3 = __errno_location();
        if (*v3 != 6) goto label_7d7f;
        v16 = a6;
        if (a6 <= v8) {
          v16 = lseek(a0,0,2);
          if (v16 < 0) goto label_7d7f;
          if (v16 <= v8) {
            v1 = v4 < v16;
            v13 = (unsigned char)v14 | v1;
            goto label_7f22;
          }
          a6 = v16;
        }
      }
      if ((long)lseek(a0,v8,0) < 0) {
        v3 = __errno_location();
        goto label_7d7f;
      }
      v7 = (v8 - v15) - v7;
      if (v7) { // branch-flip
        if (a7 != 1) { // branch-flip
          v7 = sub_6910(a1,v5,a7 == 3);
          v14 = v7 & 0xffffffff;
          if (!(char)v7)
            return 0;
          v7 = v16 - v8;
          v12 = a4;
        }
        else {
          if (!sub_6820(a1,v7)) {
            v6 = sub_14c40(0,3,v5);
            v11 = "%s: write failed";
            goto label_7d7f;
          }
          v7 = v16 - v8;
          v14 = 0;
          v12 = 0;
        }
      }
      else {
        v7 = v16 - v8;
        v14 = 0;
        v12 = 0;
        if (a7 != 1)
          v12 = a4;
      }
      if (!sub_6a40(a0,a1,a2,a3,v12,1,(unsigned char)v2,v11,v5,v7,&v10,&v9))
        return 0;
      v16 = v10 + v8;
      if (v10)
        v14 = (unsigned long)v9;
      v13 = (unsigned char)v14;
      v4 = v16;
      if (v10 < v7) {
        v1 = 0;
        goto label_7f22;
      }
      a5 = lseek(a0,v16,3);
      if (a5 < 0) break;
      v15 = v8;
    }
    v3 = __errno_location();
    if (*v3 != 6) {
label_7d7f:
      v11 = (char *)sub_14b40(4,v11);
      v5 = dcgettext(NULL,"cannot lseek %s",5);
      error(0,*v3,v5,v11);
      return 0;
    }
    v1 = v16 < a6;
    v13 |= v1;
    v16 = a6;
label_7f22:
    if (!v13)
      return 1;
  }
  else {
    v1 = 0 < a6;
    v4 = 0;
    v16 = a6;
  }
  if (a7 != 1) { // branch-flip
    if (!ftruncate(a1,v16)) {
      if (a7 != 3)
        return 1;
      if (!v1)
        return 1;
      if (0 <= (int)sub_68d0(a1,v4,v16 - v4))
        return 1;
      v6 = sub_14b40(4,v5);
      v11 = "error deallocating %s";
      goto label_7d7f;
    }
  }
  else if (sub_6820(a1,v16 - v4))
    return 1;
  v6 = sub_14b40(4,v5);
  v11 = "failed to extend %s";
label_7d7f:
  v11 = dcgettext(NULL,v11,5);
  error(0,*__errno_location(),v11,v6);
  return 0;
}

// Function: sub_7fb0 @ 0x7fb0
unsigned char sub_7fb0(int a0,unsigned char a1) // early-return
{
  if (a0 == 4)
    return 1;
  return a0 == 3 & a1;
}

// Function: sub_7fd0 @ 0x7fd0
long sub_7fd0(long a0) // early-return
{
  if (0x1ffffffffffe0001 <= (unsigned long)(a0 - 0x20000U))
    return 0x20000;
  return a0;
}

// Function: sub_7ff0 @ 0x7ff0
unsigned char sub_7ff0(char *a0,char *a1,unsigned long a2,unsigned char a3,void *a4)
{
  unsigned char v1;
  int v2;
  long v3;
  int *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x38
  
  v1 = *(unsigned char *)((long)a4 + 0x33);
  if (!v1) {
    v3 = *(long *)((long)a4 + 0x28);
    a3 = v3 != 0 & a3;
    if ((a3) && ((int)sub_c760(v3,a1,a2) <= -1)) {
      v4 = __errno_location();
      v2 = *v4;
      if (!sub_61d0(v2)) {
        v5 = (char *)sub_14b40(4,a1);
        v6 = dcgettext(NULL,"failed to set default file creation context for %s",5);
        error(0,*v4,v6,v5);
        return a3;
      }
    }
    return 1;
  }
  if ((*(char *)((long)a4 + 0x31)) && (!*(char *)((long)a4 + 0x34))) {
    if (*(char *)((long)a4 + 0x37)) { // branch-flip
      if ((int)sub_158e0(a0,&v7) <= -1) goto label_807e;
      if (0 <= setfscreatecon(v7)) {
        freecon(v7); // return-dupe
        return v1;
      }
    }
    else {
      if ((int)sub_158e0(a0,&v7) <= -1) {
        v4 = __errno_location();
        v2 = *v4;
        if (sub_61d0(v2)) goto label_807e;
        goto label_8048;
      }
      if (0 <= setfscreatecon(v7)) {
        freecon(v7);
        return v1;
      }
      v4 = __errno_location();
      v2 = *v4;
      if (!sub_61d0(v2)) goto label_81fd;
    }
  }
  else {
    if ((int)sub_158e0(a0,&v7) <= -1) {
      v4 = __errno_location();
label_8048:
      v5 = (char *)sub_14b40(4,a0);
      v6 = dcgettext(NULL,"failed to get security context of %s",5);
      error(0,*v4,v6,v5);
label_807e:
      if (!*(char *)((long)a4 + 0x34))
        return v1;
      return 0;
    }
    if (0 <= setfscreatecon(v7)) {
      freecon(v7);
      return v1;
    }
    v4 = __errno_location();
label_81fd:
    v5 = (char *)sub_14e00(v7);
    v6 = dcgettext(NULL,"failed to set default file creation context to %s",5);
    error(0,*v4,v6,v5);
  }
  if (*(char *)((long)a4 + 0x34)) {
    freecon(v7);
    return 0;
  }
  freecon(v7);
  return v1;
}

// Function: sub_8250 @ 0x8250
unsigned long sub_8250(char *a0,char a1,void *a2) // return-dupe
{
  int v1;
  long v2;
  unsigned int v3; // eax
  char *v4; // rax
  char *v5; // rax
  int *v6; // rax
  
  v2 = *(long *)((long)a2 + 0x28);
  if ((*(char *)((long)a2 + 0x31)) && (!*(char *)((long)a2 + 0x34))) {
    if (*(char *)((long)a2 + 0x37))
      return sub_c960(v2,a0,(unsigned char)a1); // tail-call
    v3 = sub_c960(v2,a0,(unsigned char)a1);
    if ((char)v3)
      return 1;
    v6 = __errno_location();
    v1 = *v6;
    if (sub_61d0(v1))
      return (unsigned long)v3;
  }
  else {
    if (sub_c960(v2,a0,(unsigned char)a1))
      return 1;
    v6 = __errno_location();
  }
  v4 = (char *)sub_14a70(0,4,a0);
  v5 = dcgettext(NULL,"failed to set the security context of %s",5);
  error(0,*v6,v5,v4);
  return 0;
}

// Function: sub_8330 @ 0x8330
void sub_8330(struct_24 *a0)
{
  long v1; // rax
  
  v1 = sub_11cd0(0x3d,NULL,sub_12540,sub_12510,sub_125b0);
  a0->field_0x48 = v1;
  if (v1)
    return;
  sub_17250(); // no-return
}

// Function: sub_8370 @ 0x8370
void sub_8370(struct_25 *a0)
{
  long v1; // rax
  
  v1 = sub_11cd0(0x3d,NULL,sub_124f0,sub_12510,sub_125b0);
  a0->field_0x50 = v1;
  if (v1)
    return;
  sub_17250(); // no-return
}

// Function: sub_83b0 @ 0x83b0
void sub_83b0(unsigned long *a0)
{
  unsigned long *v1;
  unsigned int v2; // eax
  unsigned long v3; // rcx
  unsigned long *v4;
  
  *a0 = 0;
  a0[10] = 0;
  v3 = (unsigned long)(((int)a0 - (int)(unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8)) + 0x58U >> 3);
  v4 = (unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8);
  while (v3) {
    v1 = &v4[1];
    *v4 = 0;
    v3 -= 1;
    v4 = v1;
  }
  v2 = geteuid();
  *(unsigned int *)&a0[8] = 0xffffffff;
  *(bool *)((long)a0 + 0x1b) = v2 == 0;
  *(bool *)((long)a0 + 0x1a) = v2 == 0;
}

// Function: sub_8400 @ 0x8400
int * sub_8400(void *a0)
{
  int v1;
  unsigned long v2;
  bool v3; // al
  unsigned int v4; // eax
  int *v5; // rax
  undefined7 v6; // rax
  
  v5 = __errno_location();
  v1 = *v5;
  v2 = (unsigned long)v5 >> 8;
  v6 = (undefined7)v2;
  v3 = v1 == 1 || v1 == 0x16;
  if (v1 == 1 || v1 == 0x16) {
    v4 = (unsigned int)*(unsigned char *)((long)a0 + 0x1a);
    v4 ^= 1;
    v5 = (unsigned long)(unsigned long)v4;
  }
  return v5;
}

// Function: sub_8430 @ 0x8430
int sub_8430(void *a0,char *a1,unsigned int a2,char *a3,int a4,unsigned int a5,unsigned int a6,unsigned int a7,char a8,void *a9)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // rax
  unsigned int v4;
  char *v5; // rax
  
  if (a8) { // branch-flip
label_84a0:
    if (a4 != -1) { // branch-flip
      if (!fchown(a4,a6,a7))
        return 1;
      v2 = __errno_location();
      v1 = *v2;
      if ((v1 == 1) || (v1 == 0x16)) {
        fchown(a4,0xffffffff,a7);
        *v2 = v1;
      }
    }
    else {
      v1 = sub_e540(a2,a3,a6,a7);
      if (!v1)
        return 1;
      v2 = __errno_location();
      v1 = *v2;
      if ((v1 == 1) || (v1 == 0x16)) {
        sub_e540(a2,a3,0xffffffff,a7);
        *v2 = v1;
      }
    }
    if (sub_8400(a0))
      return 0;
    v3 = sub_14b40(4,a1);
    v5 = "failed to preserve ownership for %s";
  }
  else {
    if (*(unsigned long *)((long)a0 + 0x18) & 0xff0000000000ff) // branch-flip
      v4 = *(unsigned int *)((long)a9 + 0x18);
    else {
      if (!*(char *)((long)a0 + 0x39)) goto label_84a0;
      a5 = *(unsigned int *)((long)a0 + 0x10);
      v4 = *(unsigned int *)((long)a9 + 0x18);
    }
    if ((!((unsigned short)(~(unsigned short)a5 | 0xe00) & v4 & 0xfff)) || (!sub_12910(a1,a4,a5 & v4 & 0x1c0))) goto label_84a0;
    if (sub_6230(a0)) {
      return -(unsigned int)*(unsigned char *)((long)a0 + 0x32);
    }
    v3 = sub_14b40(4,a1);
    v5 = "clearing permissions for %s";
  }
  v5 = dcgettext(NULL,v5,5);
  error(0,*__errno_location(),v5,v3);
  return -(unsigned int)*(unsigned char *)((long)a0 + 0x32);
}

// Function: sub_8640 @ 0x8640
unsigned int sub_8640(void)
{
  unsigned int v1; // eax
  
  if (dat_25010 != 0xffffffff)
    return dat_25010;
  v1 = umask(0);
  dat_25010 = v1;
  umask(v1);
  return v1;
}

// Function: sub_8680 @ 0x8680
unsigned int sub_8680(char *a0,char *a1,int a2,char *a3,void *a4,unsigned int a5,unsigned int a6,char *a7,long *a8)
{
  char v1;
  unsigned long v10;
  char *v11;
  int *v12; // rax
  unsigned long v13;
  unsigned long v14; // rax
  unsigned long v15; // rax
  char *v16; // rax
  char *v17; // rax
  long v18; // rax
  unsigned int v19;
  char v2;
  stat v20; // stack - 0xd8
  stat v21; // stack - 0x168
  char v22; // stack - 0x1a1
  unsigned long v23; // stack - 0x1a0
  unsigned long v24; // stack - 0x190
  unsigned long v25; // stack - 0x188
  long v26;
  bool v27; // al
  bool v28; // r9b
  char v29 [16];
  long *v3;
  long v30; // stack - 0x1d8
  unsigned int v31; // stack - 0x1c4
  int v32; // stack - 0x1b8
  unsigned int v33; // stack - 0x1b0
  long v34; // stack - 0x198
  unsigned long v35; // stack - 0x180
  unsigned long v36; // stack - 0x178
  unsigned long v37; // stack - 0x170
  char v4;
  unsigned char v5; // al
  int v6; // eax
  unsigned int v7;
  int v8;
  unsigned int v9;
  
  v3 = a8;
  v11 = a7;
  v31 = *(unsigned int *)&a8[3];
  v23 = 0;
  v4 = *(char *)((long)a4 + 0x31);
  v1 = *(char *)((long)a4 + 0x35);
  v6 = sub_e880(a0,(unsigned long)(*(int *)((long)a4 + 4) == 2) << 0x11);
  if (v6 < 0) {
    v10 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"cannot open %s for reading",5);
    error(0,*__errno_location(),v11,v10);
    return 0;
  }
  if (fstat(v6,&v20)) { // branch-flip
    v10 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"cannot fstat %s",5);
    v8 = *__errno_location();
label_8779:
    v7 = 0;
    error(0,v8,v11,v10);
    v10 = 0;
  }
  else {
    if ((v3[1] != v20._8_8_) || (*v3 != v20._0_8_)) {
      v10 = sub_14b40(4,a0);
      v11 = dcgettext(NULL,"skipping file %s, as it was replaced while being copied",5);
      v8 = 0;
      goto label_8779;
    }
    v2 = *v11;
    v12 = __errno_location();
    if (v2) {
label_8cdb:
      v9 = ~a6 & a5;
      if ((v1) && (!*(char *)((long)a4 + 0x1b)))
        v9 |= 0x80;
      v32 = sub_12710(a2,a3,0xc1,v9);
      v8 = *v12;
      v5 = -(char)(v32 >> 0x1f);
      if ((v8 != 0x11) || (!v5)) {
        v5 &= v8 == 0x15;
label_8da9:
        if (v5) { // branch-flip
          v8 = 0x15;
          if (*a1)
            v8 = (a1[strlen(a1) - 1] != '/') + 0x14;
        }
        else {
          v9 = ~a5 & v9;
          if (0 <= v32) goto label_88bc;
        }
      }
      else {
        v8 = 0x11;
        if (!*(char *)((long)a4 + 0x18)) {
          v18 = readlinkat(a2,a3,&v26,1);
          v32 = 0x11;
          v8 = v32;
          if (0 <= v18) {
            if (!*(char *)((long)a4 + 0x3e)) {
              v11 = (char *)sub_14b40(4,a1);
              error(0,0,dcgettext(NULL,"not writing through dangling symlink %s",5),v11);
              v7 = 0;
label_92ad:
              v10 = 0;
              goto label_8788;
            }
            v32 = sub_12710(a2,a3,0x41,v9);
            v8 = *v12;
            v5 = v8 == 0x15 & (unsigned char)((unsigned int)v32 >> 0x1f);
            goto label_8da9;
          }
        }
      }
label_8d49:
      v10 = sub_14b40(4,a1);
      v11 = dcgettext(NULL,"cannot create regular file %s",5);
      goto label_8779;
    }
    v32 = sub_12710(a2,a3,(-(unsigned int)(v4 == '\0') & 0xfffffe00) + 0x201);
    v8 = *v12;
    if (0 <= v32) { // branch-flip
      if ((*(long *)((long)a4 + 0x28)) || (*(char *)((long)a4 + 0x33))) {
        v7 = sub_8250(a1,0,a4);
        if (((char)v7) || (!*(char *)((long)a4 + 0x34))) goto label_88a0;
      }
      else {
label_88a0:
        if (*v11) goto label_8cdb;
        a6 = 0;
        v9 = 0;
label_88bc:
        if (v4) { // branch-flip
          if (*(int *)((long)a4 + 0x44)) {
            if (!sub_6770(v32,v6)) goto label_8de0;
            if (*(int *)((long)a4 + 0x44) == 2) {
              v7 = 0;
              v11 = (char *)sub_14a70(1,4,a0);
              v17 = (char *)sub_14a70(0,4,a1);
              v16 = dcgettext(NULL,"failed to clone %s from %s",5);
              error(0,*v12,v16,v17,v11);
              goto label_8ba8;
            }
          }
label_88d2:
          v33 = fstat(v32,&v21);
          if (v33) {
            v7 = 0;
            v11 = (char *)sub_14b40(4,a1);
            v17 = dcgettext(NULL,"cannot fstat %s",5);
            error(0,*v12,v17,v11);
            goto label_8ba8;
          }
          if (v21._24_4_ != (v21._24_4_ | v9)) {
            v8 = sub_7020(v32,a2,a3,v21._24_4_ | v9);
            if (v8)
              v9 = v33;
          }
          if (v4) {
            v13 = sub_7fd0(v21._56_8_,&v20);
            v30 = 0x200;
            if ((unsigned long)(v21._56_8_ - 1U) <= 0x1fffffffffffffff)
              v30 = v21._56_8_;
            v8 = sub_6780(v6);
            if (v8) { // branch-flip
              if (((v21._24_4_ & 0xf000) != 0x8000) || ((*(int *)((long)a4 + 0xc) != 3 && ((*(int *)((long)a4 + 0xc) != 2 || (v8 == 1)))))) {
                sub_e840(v6,0,0,2);
                v14 = sub_dbf0(sub_7fd0(v20._56_8_),v13,0x7fffffffffffffff);
                if (((v20._24_4_ & 0xf000) == 0x8000) && ((unsigned long)v20._48_8_ < v13))
                  v13 = v20._48_8_ + 1;
                v13 = (v13 - 1) + v14;
                v13 -= v13 % v14;
                v27 = *(int *)((long)a4 + 0x44) != 0;
                if (0 < (long)v13) {
                  v19 = 1;
                  if (v8 == 3) goto label_91e4;
                  v30 = 0;
                  v28 = *(int *)((long)a4 + 0xc) == 3;
                  goto label_8a67;
                }
                v13 = v14;
                if (v8 != 3) {
                  v30 = 0;
                  v28 = *(int *)((long)a4 + 0xc) == 3;
                  goto label_8a67;
                }
                v19 = 1;
label_91e4:
                v22 = '\0';
                v4 = sub_7b50(v6,v32,&v23,v13,v30,v34,v20._48_8_,v19,v27,a0,a1);
              }
              else {
                sub_e840(v6,0,0,2);
                if (v8 == 3) {
                  v27 = *(int *)((long)a4 + 0x44) != 0;
                  v19 = *(unsigned int *)((long)a4 + 0xc);
                  goto label_91e4;
                }
                v27 = *(int *)((long)a4 + 0x44) != 0;
                v28 = *(int *)((long)a4 + 0xc) == 3;
label_8a67:
                v22 = '\0';
                v4 = sub_6a40(v6,v32,&v23,v13,v30,v28,v27,a0,a1,0xffffffffffffffff,&v24,&v22);
              }
              if (v4 != '\x01') goto label_8eb0;
              if ((!v22) || (0 <= (int)ftruncate(v32,v24))) goto label_8ab7;
              v10 = sub_14b40(4,a1);
              v11 = "failed to extend %s";
            }
            else {
              v10 = sub_14b40(4,a0);
              v11 = "cannot lseek %s";
            }
            v11 = dcgettext(NULL,v11,5);
            error(0,*v12,v11,v10);
label_8eb0:
            v7 = 0;
            goto label_8ba8;
          }
label_8ab7:
          if (*(char *)((long)a4 + 0x1f)) goto label_8e06;
label_8ad0:
          if (*(char *)((long)a4 + 0x1d)) {
            if ((*(int *)((long)v3 + 0x1c) != v21._28_4_) || (v21._32_4_ != (int)v3[4])) {
              v7 = 0;
              v8 = sub_8430(a4,a1,a2,a3,v32,*(unsigned int *)&v3[3],*(int *)((long)v3 + 0x1c),(int)v3[4],(unsigned char)*v11,&v21);
              if (v8 == -1) goto label_8ba8;
              v7 = v31 & 0xfffff1ff;
              if (v8)
                v7 = v31;
              v31 = v7;
            }
          }
        }
        else {
label_8de0:
          if (*(char *)((long)a4 + 0x1d) || v9) {
            v4 = '\0';
            goto label_88d2;
          }
          v21._24_4_ = 0;
          v9 = 0;
          if (*(char *)((long)a4 + 0x1f)) {
label_8e06:
            v29 = sub_15960(v3);
            v35 = SUB168(v29,8);
            v25 = SUB168(v29,0);
            v29 = sub_15980(v3);
            v37 = SUB168(v29,8);
            v36 = SUB168(v29,0);
            if (sub_e900(v32,a2,a3,&v25,0)) {
              v17 = (char *)sub_14b40(4,a1);
              v16 = dcgettext(NULL,"preserving times for %s",5);
              error(0,*v12,v16,v17);
              if (*(char *)((long)a4 + 0x32)) goto label_8eb0;
            }
            goto label_8ad0;
          }
        }
        if ((v1) && (!sub_7040(a0,v6,a1,v32,a4)))
          v7 = *(unsigned char *)((long)a4 + 0x36) ^ 1;
        else {
          v7 = 1;
        }
        if (*(unsigned long *)((long)a4 + 0x18) & 0xff0000000000ff) { // branch-flip
          if ((sub_caa0(a0,v6,a1,v32,v31)) && (*(char *)((long)a4 + 0x32)))
            v7 = 0;
        }
        else if (*(char *)((long)a4 + 0x39)) { // branch-flip
          v19 = *(unsigned int *)((long)a4 + 0x10);
          if (sub_cb50(a1,v32,v19))
            v7 = 0;
        }
        else if ((*(char *)((long)a4 + 0x20)) && (*v11)) {
          v9 = sub_8640();
          if (sub_cb50(a1,v32,~v9 & 0x1b6))
            v7 = 0;
        }
        else if ((v9 || a6) && (~sub_8640() & a6 || v9)) {
          v9 = sub_8640();
          if (sub_7020(v32,a2,a3,a5 & ~v9)) {
            v11 = (char *)sub_14b40(4,a1);
            v17 = dcgettext(NULL,"preserving permissions for %s",5);
            error(0,*v12,v17,v11);
            if (*(char *)((long)a4 + 0x32))
              v7 = 0;
          }
        }
      }
label_8ba8:
      v8 = close(v32);
      v10 = v23;
      if (0 <= v8) {
        v6 = close(v6);
        goto label_8bc6;
      }
      v7 = 0;
      v11 = (char *)sub_14b40(4,a1);
      v17 = dcgettext(NULL,"failed to close %s",5);
      error(0,*v12,v17,v11);
      v10 = v23;
    }
    else {
      if (v8 == 2) goto label_8ca8;
      if (!*(char *)((long)a4 + 0x16)) {
        if (*v11) goto label_8cdb;
        goto label_8d49;
      }
      if (!unlinkat(a2,a3,0)) {
        if (*(char *)((long)a4 + 0x3c)) {
          v17 = (char *)sub_14b40(4,a1);
          __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v17);
        }
label_8ca8:
        if ((!*(long *)((long)a4 + 0x28)) || (v7 = sub_7ff0(a0,a1,a5,1,a4), (char)v7)) {
          *v11 = '\x01';
          goto label_8cdb;
        }
        goto label_92ad;
      }
      if (*v12 == 2) goto label_8ca8;
      v11 = (char *)sub_14b40(4,a1);
      v17 = dcgettext(NULL,"cannot remove %s",5);
      v10 = 0;
      error(0,*v12,v17,v11);
      v7 = 0;
    }
  }
label_8788:
  v6 = close(v6);
label_8bc6:
  if (v6 <= -1) {
    v15 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"failed to close %s",5);
    v7 = 0;
    error(0,*__errno_location(),v11,v15);
  }
  sub_cbd0(v10);
  return v7;
}

// Function: sub_9470 @ 0x9470
unsigned long sub_9470(char *a0,char *a1,int a2,char *a3,unsigned int a4,long *a5,long *a6,int *a7,unsigned int a8,char *a9,char *a10,long *a11)
{
  int v1;
  unsigned long v10; // rax
  unsigned long *v11; // rax
  char *v12; // rax
  char *v13; // rax
  unsigned long v14;
  unsigned long v15; // rax
  void *v16;
  char *v17; // rax
  long *v18;
  unsigned int v19;
  long v2;
  stat *v20;
  char v21 [4];
  long v22; // stack - 0x288
  char v23 [144]; // stack - 0x1f8
  char v24; // stack - 0x289
  stat v25; // stack - 0x168
  stat v26; // stack - 0xd8
  bool v27; // stack - 0x28a
  char *v28;
  char *v29;
  int *v3;
  char *v30;
  char *v31;
  char *v32;
  char *v33; // rsp
  char *v34; // rsp
  struct_5 *v35;
  char v36;
  undefined4 v37;
  int v38;
  unsigned long v39; // r14
  char v4;
  unsigned char v40;
  unsigned int v41;
  char *v42;
  bool v43; // r8b
  unsigned char v44;
  char v45 [16];
  unsigned int v46; // stack - 0x304
  long v47; // stack - 0x300
  unsigned int v48; // stack - 0x2f8
  unsigned char v49; // stack - 0x2f1
  unsigned char v5; // al
  long *v50; // stack - 0x2f0
  char *v51; // stack - 0x2e8
  char *v52; // stack - 0x2e0
  unsigned long *v53; // stack - 0x2d8
  long *v54; // stack - 0x2d0
  long *v55; // stack - 0x2c8
  long *v56; // stack - 0x2c0
  unsigned long v57; // stack - 0x2b8
  char *v58; // stack - 0x2b0
  char *v59; // stack - 0x2a8
  char v6; // al
  int v60; // stack - 0x2a0
  unsigned int v61; // stack - 0x29c
  unsigned long v62; // stack - 0x280
  unsigned long v63; // stack - 0x278
  unsigned int v64; // stack - 0x270
  unsigned int v65; // stack - 0x26c
  unsigned int v66; // stack - 0x268
  unsigned long v67; // stack - 0x260
  unsigned long v68; // stack - 0x258
  int v7;
  int *v8; // rax
  char *v9;
  
  v3 = a7;
  v29 = v21;
  v28 = v21;
  v34 = v21;
  v30 = v21;
  v31 = v21;
  v32 = v21;
  v54 = a6;
  v52 = a9;
  v57 = CONCAT44(v57._4_4_,a8);
  v51 = a10;
  v38 = a7[0x10];
  v56 = a11;
  v53 = (unsigned long *)CONCAT71(v53._1_7_,(char)a8);
  *a10 = 0;
  v27 = 0 < (int)a4;
  v58 = a1;
  v50 = a5;
  v59 = a3;
  v60 = a2;
  if (!(char)a7[6]) {
    if (!v38) goto label_9531;
label_9560:
    if ((v38 == 0x11) && (v3[2] == 2)) {
      if (!(char)v57) goto label_9767;
      goto label_95c4;
    }
    v7 = -100;
    v9 = a0;
    v42 = a0;
label_9575:
    v1 = v3[1];
    if (!sub_6360(v7,v9,&v22,(unsigned long)(v1 == 2) << 8)) {
      v61 = v64;
      if (((v64 & 0xf000) != 0x4000) || (v42 = NULL, (char)v3[0xe])) goto label_95b7;
      v10 = sub_14b40(4,a0);
      v42 = NULL;
      if (*(char *)((long)v3 + 0x19)) {
        v9 = "omitting directory %s";
        v9 = dcgettext(NULL,v9,5); // return-dupe
        error(0,0,v9,v10); // return-dupe
        return (unsigned long)v42 & 0xffffffff; // return-dupe
      }
      v9 = dcgettext(NULL,"-r not specified; omitting directory %s",5);
      error(0,0,v9,v10);
      return (unsigned long)v42 & 0xffffffff;
    }
label_a440:
    *(unsigned long *)&v29[-8] = 0xa44a;
    v10 = sub_14b40(4,v42);
    v9 = "cannot stat %s";
label_a459:
    v42 = NULL;
    *(unsigned long *)&v29[-8] = 0xa463;
    v9 = dcgettext(NULL,v9,5);
    *(unsigned long *)&v29[-8] = 0xa46b;
    v38 = *__errno_location();
    *(unsigned long *)&v29[-8] = 0xa47c;
    error(0,v38,v9,v10);
    return (unsigned long)v42 & 0xffffffff;
  }
  if (v38 < 0) {
    if (!sub_14e40(0xffffff9c,a0,a2,a3,1)) {
      v27 = 1;
      a4 = 1;
      *(char *)v56 = '\x01';
      goto label_9531;
    }
    v38 = *__errno_location();
  }
  v27 = v38 == 0;
  *(bool *)v56 = v27;
  a4 = (unsigned int)v27;
  if (v38) goto label_9560;
label_9531:
  if (!*(char *)((long)v3 + 0x3f)) {
    v38 = 0;
    v9 = v59;
    v42 = v58;
    v7 = v60;
    goto label_9575;
  }
  v38 = 0;
label_95b7:
  if (!(char)v57) {
label_9767:
    v7 = v3[1];
    v49 = sub_7fb0(v7,0);
    if ((int)a4 > 0) {
      v55 = NULL;
      goto label_9668;
    }
label_9858:
    if ((v38 == 0x11) && (v3[2] == 2)) {
      v24 = '\0';
      v4 = '\0';
      goto label_9935;
    }
    v19 = v61 & 0xf000;
    if ((((v19 == 0x8000) || (v40 = (v19 == 0xa000 || v19 == 0x4000) | *(unsigned char *)&v3[5] ^ 1, !v40)) && (v40 = *(unsigned char *)&v3[6], !v40)) && ((v40 = *(unsigned char *)((long)v3 + 0x3a), !v40 && (v40 = *(unsigned char *)((long)v3 + 0x17), !v40)))) {
      if (*v3) {
        v40 = 1;
        v37 = 0x100;
        goto label_98a2;
      }
      v40 = *(unsigned char *)((long)v3 + 0x15);
      if (v40) goto label_989c;
      v37 = 0;
      if (!a4) goto label_98a2;
label_a36b:
      v27 = 1;
label_a372:
      v55 = NULL;
      v4 = 0;
      v55._0_1_ = '\0';
      v28 = v21;
      if (v38 == 0x11) {
        v38 = v3[2];
        goto label_98db;
      }
label_a38a:
      if ((char)v57) goto label_9627;
      goto label_9668;
    }
label_989c:
    v37 = 0x100;
label_98a2:
    v42 = v23;
    v55 = (long *)CONCAT71(v55._1_7_,v40);
    v7 = sub_6360(v60,v59,v42,v37);
    if (v7) {
      v8 = __errno_location();
      if (*v8 != 0x28) { // branch-flip
        if (*v8 == 2) goto label_a36b;
      }
      else if (*(char *)((long)v3 + 0x16)) goto label_a372;
      v42 = NULL;
      v9 = (char *)sub_14b40(4,v58);
      v12 = dcgettext(NULL,"cannot stat %s",5);
      error(0,*v8,v12,v9);
      return (unsigned long)v42 & 0xffffffff;
    }
    v38 = v3[2];
label_98db:
    v24 = '\0';
    v4 = (char)v55;
    if (v38 != 2) {
      v9 = &v24;
      v19 = sub_75b0(a0,&v22,v60,v59,v23,v3);
      v42 = (char *)(unsigned long)v19;
      v4 = (char)v55;
      if ((char)v19) goto label_9935;
      v10 = sub_14a70(1,4,v58,v9);
      v15 = sub_14a70(0,4,a0);
      v9 = "%s and %s are the same file";
      goto label_abe0;
    }
label_9935:
    if (!*(char *)((long)v3 + 0x3b)) {
      if ((char)v3[6]) goto label_aa71;
      if ((v61 & 0xf000) == 0x4000) goto label_a641;
label_a62c:
      if (v3[2] != 2) {
        if (v3[2] == 3) {
          v55 = (long *)CONCAT71(v55._1_7_,v4);
          v6 = sub_63e0(v3,v58,v60,v59,v23);
          v4 = (char)v55;
          if (!v6) {
            v42 = (char *)0x1; // return-dupe
            return (unsigned long)v42 & 0xffffffff;
          }
        }
        goto label_a641;
      }
      v42 = (char *)0x1;
      return (unsigned long)v42 & 0xffffffff;
    }
    if ((v61 & 0xf000) == 0x4000) {
      if (!(char)v3[6]) goto label_a641;
label_aa71:
      v55 = (long *)CONCAT71(v55._1_7_,v4);
      v19 = sub_6530(v3,v58,v60,v59,v23);
      v42 = (char *)(unsigned long)v19;
      v4 = (char)v55;
      if (!(char)v19) {
label_a641:
        if (v24) {
          v42 = (char *)0x1;
          return (unsigned long)v42 & 0xffffffff;
        }
        v42 = NULL;
        if ((v23._24_4_ & 0xf000) != 0x4000) { // branch-flip
          if ((v61 & 0xf000) != 0x4000) { // branch-flip
            v38 = *v3;
            if (!(char)v57) goto label_a6f6;
label_a68f:
            v55 = (long *)(CONCAT44(v55._4_4_,v61) & 0xffffffff0000f000);
            if (v38 != 3) {
              v47 = CONCAT71(v47._1_7_,v4);
              v4 = sub_ea90(*(struct_5 **)&v3[0x12],v59,v23);
              if (v4) {
                v10 = sub_14a70(1,4,a0);
                v15 = sub_14a70(0,4,v58);
                v9 = "will not overwrite just-created %s with %s";
                v32 = v21;
                goto label_abe0;
              }
              v4 = (char)v47;
              if (((int)v55 == 0x4000) || ((v23._24_4_ & 0xf000) != 0x4000)) goto label_a6f4;
              goto label_ae9c;
            }
            v38 = 3;
            v7 = 3;
            if ((char)v3[6]) goto label_b0d1;
label_b25f:
            v19 = v23._24_4_;
            v55 = (long *)CONCAT44(v55._4_4_,v7);
            v47 = CONCAT71(v47._1_7_,v4);
            v6 = sub_6190((char *)sub_db60(a0));
            v4 = (char)v47;
            if (!v6) {
              v38 = (int)v55;
              if ((v19 & 0xf000) == 0x4000) goto label_a798;
              goto label_aef9;
            }
label_a70f:
            if (((v19 & 0xf000) == 0x4000) || ((!*(char *)((long)v3 + 0x15) && ((!*(char *)((long)v3 + 0x31) || (((!(char)v3[0xc] || ((unsigned long)v23._16_8_ <= 1)) && ((v3[1] != 2 || ((v64 & 0xf000) == 0x8000)))))))))) goto label_a798;
            v55 = (long *)CONCAT71(v55._1_7_,v4);
            v38 = unlinkat(v60,v59,0);
            if (v38) {
              v8 = __errno_location();
              if (*v8 != 2) {
                v10 = sub_14b40(4,v58);
                v9 = "cannot remove %s";
                v9 = dcgettext(NULL,v9,5); // return-dupe
                error(0,*v8,v9,v10);
                return (unsigned long)v42 & 0xffffffff;
              }
            }
            v27 = 1;
            v4 = (char)v55;
            if ((char)v3[0xf]) {
              v47 = CONCAT71(v47._1_7_,(char)v55);
              v9 = (char *)sub_14b40(4,v58);
              __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v9);
              v55 = NULL;
              v4 = (char)v47;
              goto label_a7a3;
            }
          }
          else {
            if ((!(char)v3[6]) || (v38 = *v3, !v38)) {
              v10 = sub_14a70(1,4,a0);
              v15 = sub_14a70(0,4,v58);
              v9 = "cannot overwrite non-directory %s with directory %s";
              v32 = v21;
              goto label_abe0;
            }
            if ((char)v57) goto label_a68f;
label_b0d1:
            if ((v64 & 0xf000) == 0x4000) {
              v19 = v23._24_4_;
label_b487:
              if ((v19 & 0xf000) != 0x4000) {
                if (!v38) {
                  v10 = sub_14c40(0,3,v58);
                  v15 = sub_14c40(0,3,a0);
                  v9 = "cannot move directory onto non-directory: %s -> %s";
                  v32 = v21;
                  goto label_abe0;
                }
                goto label_aec6;
              }
            }
            if (v38) goto label_aec6;
          }
label_a798:
          v55 = NULL;
          v31 = v21;
        }
        else {
          if ((v61 & 0xf000) == 0x4000) {
label_a6f4:
            v38 = *v3;
label_a6f6:
            if (!(char)v3[6]) {
              v19 = v23._24_4_;
              v7 = v38;
              if (v38) goto label_b25f;
              goto label_a70f;
            }
            goto label_b0d1;
          }
label_ae9c:
          if ((!(char)v3[6]) || (v38 = *v3, !v38)) {
            v10 = sub_14b40(4,v58);
            v9 = "cannot overwrite directory %s with non-directory";
            v9 = dcgettext(NULL,v9,5);
            error(0,0,v9,v10);
            return (unsigned long)v42 & 0xffffffff;
          }
          v19 = v23._24_4_;
          if ((v64 & 0xf000) == 0x4000) goto label_b487;
label_aec6:
          v55 = (long *)CONCAT71(v55._1_7_,v4);
          v47 = CONCAT44(v47._4_4_,v38);
          v6 = sub_6190((char *)sub_db60(a0));
          v4 = (char)v55;
          if (v6) goto label_a798;
          v38 = (int)v47;
label_aef9:
          if (v38 != 3) {
            v55 = (long *)CONCAT71(v55._1_7_,v4);
            v6 = sub_6640();
            v4 = (char)v55;
            if (v6) {
              if ((char)v3[6]) // branch-flip
                v9 = dcgettext(NULL,"backing up %s might destroy source;  %s not moved",5);
              else {
                v9 = dcgettext(NULL,"backing up %s might destroy source;  %s not copied",5);
              }
              v10 = sub_14a70(1,4,a0);
              error(0,0,v9,sub_14a70(0,4,v58),v10);
              return (unsigned long)v42 & 0xffffffff;
            }
          }
          v9 = v59;
          v55 = (unsigned long)CONCAT71(v55._1_7_,v4);
          v12 = (char *)sub_da90(v60,v59,*v3);
          if (v12) { // branch-flip
            v39 = (long)v9 - (long)v58;
            v14 = strlen(v12);
            v42 = (char *)(v14 + 1);
            v14 = v39 + 0x18 + v14;
            v31 = v21;
            while (v34 != &v21[-(v14 & 0xfffffffffffff000)]) {
              v30 = &v31[-0x1000];
              v33 = &v31[-0x1000];
              *(unsigned long *)&v31[-8] = *(unsigned long *)&v31[-8];
              v34 = &v31[-0x1000];
              v31 = v33;
            }
            v14 = (unsigned long)((unsigned int)v14 & 0xff0);
            v2 = -v14;
            if (v14)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            v47 = CONCAT71(v47._1_7_,(char)v55);
            v55 = (long *)((unsigned long)&v30[v2 + 0xf] & 0xfffffffffffffff0);
            *(unsigned long *)&v30[v2 + -8] = 0xafea;
            v16 = mempcpy(v55,v58,v39);
            *(unsigned long *)&v30[v2 + -8] = 0xaff8;
            memcpy(v16,v12,(unsigned long)v42);
            *(unsigned long *)&v30[v2 + -8] = 0xb000;
            free(v12);
            v31 = &v30[v2];
            v4 = (char)v47;
          }
          else {
            v47 = CONCAT71(v47._1_7_,(char)v55);
            v8 = __errno_location();
            v55 = NULL;
            v31 = v21;
            v4 = (char)v47;
            if (*v8 != 2) {
              v10 = sub_14b40(4,v58);
              v9 = "cannot backup %s";
              v9 = dcgettext(NULL,v9,5);
              error(0,*v8,v9,v10);
              return (unsigned long)v42 & 0xffffffff;
            }
          }
          v27 = 1;
        }
label_a7a3:
        v38 = 0x11;
        v28 = v31;
        goto label_a38a;
      }
      if (!v56) {
        v42 = (char *)0x1;
        return (unsigned long)v42 & 0xffffffff;
      }
      goto label_aaba;
    }
    v43 = 0;
    if ((*(char *)((long)v3 + 0x1f)) && (v43 = 1, (char)v3[6]))
      v43 = v23._0_8_ != v22;
    v55 = (long *)CONCAT71(v55._1_7_,v4);
    v38 = sub_15e70(v60,v59,v23,&v22,v43);
    v9 = v59;
    if (v38 < 0) {
      v4 = (char)v55;
      if (!(char)v3[6]) goto label_a62c;
      goto label_aa71;
    }
    if (v56)
      *(char *)v56 = '\x01';
    v12 = (char *)sub_bf60(v59,v62,v22);
    if ((!v12) || (v4 = sub_7460(NULL,v60,v12,v58,v60,v9,1,(unsigned char)*(char *)&v3[0xf],v49), v4)) {
      v42 = (char *)0x1;
      return (unsigned long)v42 & 0xffffffff;
    }
    if (*(char *)((long)v3 + 0x33))
      sub_7380();
    v42 = NULL; // return-dupe
    return (unsigned long)v42 & 0xffffffff;
  }
label_95c4:
  v35 = *(struct_5 **)&v3[0x14];
  if (v35) {
    v18 = &v22;
    if (((v61 & 0xf000) == 0x4000) || (*v3)) {
label_95f2:
      sub_ea00(v35,a0,v18);
      goto label_95fd;
    }
    v55 = &v22;
    v19 = sub_ea90(v35,a0,&v22);
    v42 = (char *)(unsigned long)v19;
    if (!(char)v19) {
      v35 = *(struct_5 **)&v3[0x14];
      v18 = v55;
      goto label_95f2;
    }
    v10 = sub_14b40(4,a0);
    v9 = "warning: source file %s specified more than once";
    v9 = dcgettext(NULL,v9,5);
    error(0,0,v9,v10);
    return (unsigned long)v42 & 0xffffffff;
  }
label_95fd:
  v7 = v3[1];
  v49 = sub_7fb0(v7,1);
  if ((int)a4 <= 0) goto label_9858;
  v55 = NULL;
  v4 = 0;
  v28 = v21;
label_9627:
  if ((*(long *)&v3[0x12]) && (v42 = NULL, !(char)v3[6])) {
    if (!*v3) {
      v20 = (stat *)v23;
      if (!v4) {
        v20 = &v26;
        *(unsigned long *)&v28[-8] = 0xaae9;
        if (fstatat(v60,v59,v20,0x100)) goto label_9668;
      }
      if ((*(unsigned int *)&v20->field_0x18 & 0xf000) == 0xa000) {
        v35 = *(struct_5 **)&v3[0x12];
        *(unsigned long *)&v28[-8] = 0xaba0;
        v4 = sub_ea90(v35,v59,v20);
        if (v4) {
          *(unsigned long *)&v28[-8] = 0xabbe;
          v10 = sub_14a70(1,4,v58);
          *(unsigned long *)&v28[-8] = 0xabd0;
          v15 = sub_14a70(0,4,a0);
          v9 = "will not copy %s through just-created symlink %s";
          v42 = NULL;
          v32 = v28;
label_abe0:
          *(unsigned long *)&v32[-8] = 0xabe7;
          v9 = dcgettext(NULL,v9,5);
          *(unsigned long *)&v32[-8] = 0xabfb;
          error(0,0,v9,v15,v10);
          return (unsigned long)v42 & 0xffffffff;
        }
      }
      goto label_9668;
    }
    if ((char)v3[0xf]) goto label_9f88;
    if (!v38) goto label_9ad8;
label_9681:
    if ((!(char)v3[0xe]) || ((v61 & 0xf000) != 0x4000)) {
      v4 = (char)v3[6];
      if (v4) {
        if (v63 == 1) {
          *(unsigned long *)&v28[-8] = 0xaca3;
          v47 = sub_bf00(v62,v22);
label_a961:
          if (v47) {
            if ((v61 & 0xf000) == 0x4000) goto label_96d5;
            *(unsigned long *)&v28[-0x10] = (unsigned long)v49;
            *(unsigned long *)&v28[-0x18] = (unsigned long)*(unsigned char *)&v3[0xf];
            *(unsigned long *)&v28[-0x20] = 1;
            *(unsigned long *)&v28[-0x28] = 0xa9bd;
            v4 = sub_7460(NULL,v60,v47,v58,v60,v59);
            if (v4) {
              v42 = (char *)0x1;
              return (unsigned long)v42 & 0xffffffff;
            }
            goto label_a9c9;
          }
          goto label_9740;
        }
        if (((char)v3[0xc]) && (!*(char *)((long)v3 + 0x17))) goto label_9f34;
        v47 = 0;
label_97c5:
        if (v38 != 0x11) { // branch-flip
label_97cf:
          if (v38 != 0x16) {
            if (v38 != 0x12) {
              *(unsigned long *)&v28[-8] = 0xa816;
              v10 = sub_14a70(1,4,v58);
              *(unsigned long *)&v28[-8] = 0xa828;
              v15 = sub_14a70(0,4,a0);
              *(unsigned long *)&v28[-8] = 0xa83e;
              v9 = dcgettext(NULL,"cannot move %s to %s",5);
label_a84a:
              *(unsigned long *)&v28[-8] = 0xa853;
              error(0,v38,v9,v15,v10);
              *(unsigned long *)&v28[-8] = 0xa866;
              sub_bea0(v62,v22);
              v42 = NULL;
              return (unsigned long)v42 & 0xffffffff;
            }
            v19 = v61 & 0xf000;
            *(unsigned long *)&v28[-8] = 0x9812;
            if (unlinkat(v60,v59,(unsigned int)(v19 == 0x4000) << 9)) {
              *(unsigned long *)&v28[-8] = 0x981b;
              v8 = __errno_location();
              if (*v8 != 2) {
                *(unsigned long *)&v28[-8] = 0xab3e;
                v10 = sub_14a70(1,4,v58);
                *(unsigned long *)&v28[-8] = 0xab50;
                v15 = sub_14a70(0,4,a0);
                *(unsigned long *)&v28[-8] = 0xab66;
                v9 = dcgettext(NULL,"inter-device move failed: %s to %s; unable to remove target",5);
                v38 = *v8;
                goto label_a84a;
              }
            }
            if (((char)v3[0xf]) && (v19 != 0x4000)) {
              *(unsigned long *)&v28[-8] = 0xaa33;
              v9 = dcgettext(NULL,"copied ",5);
              *(unsigned long *)&v28[-8] = 0xaa42;
              __printf_chk(1,v9);
              *(unsigned long *)&v28[-8] = 0xaa58;
              sub_73c0(a0,v58,v55);
            }
            v27 = 1;
            v44 = 1;
            v40 = 1;
            goto label_9aee;
          }
          *(unsigned long *)&v28[-8] = 0xacc6;
          v9 = (char *)sub_14a70(1,4,dat_25568);
          *(unsigned long *)&v28[-8] = 0xacdc;
          v12 = (char *)sub_14a70(0,4,dat_25570);
          *(unsigned long *)&v28[-8] = 0xacf2;
          v13 = dcgettext(NULL,"cannot move %s to a subdirectory of itself, %s",5);
          *(unsigned long *)&v28[-8] = 0xad06;
          error(0,0,v13,v12,v9);
          *v51 = 1;
        }
        else {
          *(unsigned long *)&v28[-8] = 0xa7ca;
          if (renameat(-100,a0,v60,v59)) {
            *(unsigned long *)&v28[-8] = 0xa7d7;
            v38 = *__errno_location();
            if (v38) goto label_97cf;
          }
label_9a66:
          if ((char)v3[0xf]) {
            *(unsigned long *)&v28[-8] = 0xac23;
            v9 = dcgettext(NULL,"renamed ",5);
            *(unsigned long *)&v28[-8] = 0xac32;
            __printf_chk(1,v9);
            *(unsigned long *)&v28[-8] = 0xac48;
            sub_73c0(a0,v58,v55);
          }
          if (*(long *)&v3[10]) {
            *(unsigned long *)&v28[-8] = 0x9a8b;
            sub_8250(v58,1,v3);
          }
          if (v56)
            *(char *)v56 = '\x01';
          if (((char)v57) && (!*(char *)((long)v3 + 0x3f))) {
            v16 = *(void **)&v3[0x12];
            *(unsigned long *)&v28[-8] = 0xab17;
            sub_ea00(v16,v59,&v22);
            v42 = (char *)(v57 & 0xff);
            return (unsigned long)v42 & 0xffffffff;
          }
        }
        v42 = (char *)0x1;
        return (unsigned long)v42 & 0xffffffff;
      }
      if (!(char)v3[0xc]) goto label_9ad8;
      if (!*(char *)((long)v3 + 0x17)) {
label_9f34:
        if ((2 <= v63) || ((((char)v57 && (v3[1] == 3)) || (v47 = 0, v3[1] == 4)))) {
          *(unsigned long *)&v28[-8] = 0xa95a;
          v47 = sub_bf60(v59,v62,v22);
          goto label_a961;
        }
        goto label_9f6b;
      }
      v47 = 0;
label_9f73:
      v44 = v27;
      v40 = v27;
      goto label_9aee;
    }
    if ((char)v57) { // branch-flip
      *(unsigned long *)&v28[-8] = 0xa92f;
      v47 = sub_bf60(v59,v62,v22);
    }
    else {
      *(unsigned long *)&v28[-8] = 0x96c4;
      v47 = sub_bf00(v62,v22);
    }
    if (!v47) {
label_9740:
      v4 = (char)v3[6];
label_9f6b:
      if (v4) goto label_97c5;
      goto label_9f73;
    }
label_96d5:
    *(unsigned long *)&v28[-8] = 0x96ef;
    v4 = sub_15130(0xffffff9c,a0,v60,v47);
    if (v4) { // branch-flip
      *(unsigned long *)&v28[-8] = 0xb206;
      v9 = (char *)sub_14a70(1,4,dat_25568);
      *(unsigned long *)&v28[-8] = 0xb21c;
      v12 = (char *)sub_14a70(0,4,dat_25570);
      *(unsigned long *)&v28[-8] = 0xb232;
      v13 = dcgettext(NULL,"cannot copy a directory, %s, into itself, %s",5);
      *(unsigned long *)&v28[-8] = 0xb246;
      error(0,0,v13,v12,v9);
      *v51 = 1;
    }
    else {
      *(unsigned long *)&v28[-8] = 0x9712;
      v4 = sub_15130(v60,v59,v60,v47);
      v9 = v58;
      if (v4) {
        *(unsigned long *)&v28[-8] = 0xb6de;
        v9 = (char *)sub_14b40(4,dat_25570);
        *(unsigned long *)&v28[-8] = 0xb6f4;
        v12 = dcgettext(NULL,"warning: source directory %s specified more than once",5);
        *(unsigned long *)&v28[-8] = 0xb705;
        error(0,0,v12,v9);
        v40 = v56 != NULL & *(unsigned char *)&v3[6];
        v42 = (char *)CONCAT71((undefined7)((unsigned long)v42 >> 8),v40);
        if (v40) {
label_aaba:
          *(char *)v56 = '\x01';
          return (unsigned long)v42 & 0xffffffff;
        }
        v42 = (char *)0x1;
        return (unsigned long)v42 & 0xffffffff;
      }
      if ((v3[1] == 4) || ((v3[1] == 3 && ((char)v57)))) goto label_9740;
      *(unsigned long *)&v28[-8] = 0xb51e;
      v12 = (char *)sub_65e0(v58,v59,v47);
      *(unsigned long *)&v28[-8] = 0xb533;
      v13 = (char *)sub_14a70(1,4,v12);
      *(unsigned long *)&v28[-8] = 0xb545;
      v9 = (char *)sub_14a70(0,4,v9);
      *(unsigned long *)&v28[-8] = 0xb55b;
      v17 = dcgettext(NULL,"will not create hard link %s to directory %s",5);
      *(unsigned long *)&v28[-8] = 0xb56f;
      error(0,0,v17,v9,v13);
      *(unsigned long *)&v28[-8] = 0xb577;
      free(v12);
    }
label_a9c9:
    v29 = v28;
    if (*(char *)((long)v3 + 0x33)) {
label_a9d3:
      *(unsigned long *)&v28[-8] = 0xa9d8;
      sub_7380();
label_a562:
      v29 = v28;
      if (!v47) {
        *(unsigned long *)&v28[-8] = 0xac8b;
        sub_bea0(v62,v22);
      }
    }
    if (v55) {
      v9 = (char *)((long)v55 + ((long)v59 - (long)v58));
      *(unsigned long *)&v29[-8] = 0xa5a1;
      if (renameat(v60,v9,v60,v59)) {
        *(unsigned long *)&v29[-8] = 0xac61;
        v10 = sub_14b40(4,v58);
        v9 = "cannot un-backup %s";
        goto label_a459;
      }
      if ((char)v3[0xf]) {
        *(unsigned long *)&v29[-8] = 0xa5c9;
        v9 = (char *)sub_14a70(1,4,v58);
        *(unsigned long *)&v29[-8] = 0xa5df;
        v12 = (char *)sub_14a70(0,4,v55);
        *(unsigned long *)&v29[-8] = 0xa5f5;
        v13 = dcgettext(NULL,"%s -> %s (unbackup)\n",5);
        *(unsigned long *)&v29[-8] = 0xa60a;
        __printf_chk(1,v13,v12,v9);
      }
    }
  }
  else {
label_9668:
    if (((char)v3[0xf]) && (!(char)v3[6])) {
label_9f88:
      if ((v61 & 0xf000) != 0x4000) {
        *(unsigned long *)&v28[-8] = 0x9fb4;
        sub_73c0(a0,v58,v55);
      }
    }
    if (v38) goto label_9681;
    if ((char)v3[6]) goto label_9a66;
label_9ad8:
    v47 = 0;
    v44 = v27;
    v40 = v27;
label_9aee:
    v41 = v61;
    v19 = v61;
    if (*(char *)((long)v3 + 0x39))
      v19 = v3[4];
    v56 = (long *)(CONCAT44(v56._4_4_,v19) & 0xffffffff00000fff);
    v30 = v28;
    if (*(char *)((long)v3 + 0x1d)) { // branch-flip
      *(unsigned long *)&v28[-8] = 0x9b8f;
      v4 = sub_7ff0(a0,v58,v61,v44,v3);
      if (v4) {
        v48 = (unsigned int)v56 & 0x3f;
        if ((v41 & 0xf000) == 0x4000) {
label_a06a:
          v9 = (char *)(unsigned long)(v41 & 0xf000);
          v18 = &v22;
          *(unsigned long *)&v28[-8] = 0xa080;
          v49 = sub_61e0(v18,v54);
          if (v49) { // branch-flip
            *(unsigned long *)&v28[-8] = 0xae5d;
            v10 = sub_14b40(4,a0);
            v9 = "cannot copy cyclic symbolic link %s";
label_ae6c:
            *(unsigned long *)&v28[-8] = 0xae73;
            v9 = dcgettext(NULL,v9,5);
            *(unsigned long *)&v28[-8] = 0xae84;
            error(0,0,v9,v10);
          }
          else {
            v31 = &v28[-0x20];
            v29 = &v28[-0x20];
            v30 = &v28[-0x20];
            *(unsigned long *)&v28[-8] = *(unsigned long *)&v28[-8];
            v11 = (unsigned long *)((unsigned long)&v28[-0x11] & 0xfffffffffffffff0);
            *v11 = v54;
            v11[1] = v62;
            v11[2] = v22;
            v53 = v11;
            if ((!v40) && ((v23._24_4_ & 0xf000) == 0x4000)) {
              if ((*(long *)&v3[10]) || (*(char *)((long)v3 + 0x33))) {
                v56 = v18;
                *(unsigned long *)&v28[-0x28] = 0xb053;
                v4 = sub_8250(v58,0,v3);
                if (!v4) {
                  v30 = &v28[-0x20];
                  if ((char)v3[0xd]) goto label_a558;
                }
              }
              v48 = 0;
              v36 = 0;
label_a240:
              v5 = v50 != NULL & *(unsigned char *)&v3[7];
              v42 = (char *)CONCAT71((undefined7)((unsigned long)v9 >> 8),v5);
              if ((!v5) || (*v50 == v22)) {
                *(char **)&v28[-0x28] = v51;
                *(char **)&v28[-0x30] = v52;
                *(int **)&v28[-0x38] = v3;
                *(unsigned long **)&v28[-0x40] = v53;
                v56 = (long *)CONCAT71(v56._1_7_,v36);
                *(unsigned long *)&v28[-0x48] = 0xa291;
                v19 = sub_bbc0(a0,v58,v60,v59,v44);
                v42 = (char *)(unsigned long)v19;
                v36 = (char)v56;
              }
              if ((char)v57) {
                v54 = (long *)((unsigned long)v54._1_7_ << 8);
                v44 = 0;
                v53 = (unsigned long *)CONCAT71(v53._1_7_,v36);
                goto label_9c9d;
              }
              v54 = (long *)((unsigned long)v54._1_7_ << 8);
              v53 = (unsigned long *)CONCAT71(v53._1_7_,v36);
              goto label_9d14;
            }
            v54 = v18;
            v19 = ~v48 & (unsigned int)v56;
            *(unsigned long *)&v28[-0x28] = 0xa12c;
            if (mkdirat(v60,v59,v19)) { // branch-flip
              *(unsigned long *)&v28[-0x28] = 0xae39;
              v10 = sub_14b40(4,v58);
              v9 = "cannot create directory %s";
            }
            else {
              v56 = v54;
              *(unsigned long *)&v28[-0x28] = 0xa160;
              v38 = fstatat(v60,v59,(stat *)v23,0x100);
              v37 = v23._24_4_;
              if (!v38) {
                v9 = (char *)(unsigned long)(unsigned int)v23._24_4_;
                v36 = 0;
                if ((v23._24_4_ & 0x1c0) != 0x1c0) {
                  *(unsigned long *)&v28[-0x28] = 0xa1aa;
                  v38 = sub_e520(v60,v59,v23._24_4_ | 0x1c0);
                  v46 = v37;
                  v36 = 1;
                  if (v38) {
                    *(unsigned long *)&v28[-0x28] = 0xb76a;
                    v10 = sub_14b40(4,v58);
                    v9 = "setting permissions for %s";
                    v30 = &v28[-0x20];
                    goto label_a538;
                  }
                }
                v18 = v56;
                if (!*v52) {
                  v55 = v56;
                  v56 = (long *)CONCAT71(v56._1_7_,v36);
                  *(unsigned long *)&v28[-0x28] = 0xb305;
                  sub_bf60(v59,v23._8_8_,v23._0_8_);
                  *v52 = '\x01';
                  v18 = v55;
                  v36 = (char)v56;
                }
                if ((char)v3[0xf]) {
                  v56 = (long *)CONCAT71(v56._1_7_,v36);
                  v55 = v18;
                  if ((char)v3[6]) { // branch-flip
                    *(unsigned long *)&v28[-0x28] = 0xa205;
                    v9 = (char *)sub_14b40(4,v58);
                    *(unsigned long *)&v28[-0x28] = 0xa21b;
                    v12 = dcgettext(NULL,"created directory %s\n",5);
                    *(unsigned long *)&v28[-0x28] = 0xa22d;
                    __printf_chk(1,v12,v9);
                    v36 = (char)v56;
                  }
                  else {
                    *(unsigned long *)&v28[-0x28] = 0xb4ed;
                    sub_73c0(a0,v58,NULL);
                    v36 = (char)v56;
                  }
                }
                goto label_a240;
              }
              *(unsigned long *)&v28[-0x28] = 0xb3f0;
              v10 = sub_14b40(4,v58);
              v9 = "cannot stat %s";
              v30 = &v28[-0x20];
            }
label_a538:
            *(unsigned long *)&v30[-8] = 0xa53f;
            v9 = dcgettext(NULL,v9,5);
            *(unsigned long *)&v30[-8] = 0xa547;
            v38 = *__errno_location();
            *(unsigned long *)&v30[-8] = 0xa558;
            error(0,v38,v9,v10);
          }
label_a558:
          v28 = v30;
          if (*(char *)((long)v3 + 0x33)) goto label_a9d3;
          goto label_a562;
        }
label_9bba:
        v41 &= 0xf000;
        v54 = (long *)CONCAT71(v54._1_7_,*(char *)((long)v3 + 0x3a));
        if (*(char *)((long)v3 + 0x3a)) { // branch-flip
          if (*a0 != '/') {
            *(unsigned long *)&v28[-8] = 0x9bdf;
            v9 = (char *)sub_e730(v59);
            if (v60 != -100) { // branch-flip
label_9c13:
              *(unsigned long *)&v28[-8] = 0x9c1f;
              if (!stat(".",&v25)) {
                *(unsigned long *)&v28[-8] = 0xb10f;
                if ((!fstatat(v60,v9,&v26,0)) && ((v25._8_8_ != v26._8_8_ || (v25._0_8_ != v26._0_8_)))) {
                  *(unsigned long *)&v28[-8] = 0xb133;
                  free(v9);
                  *(unsigned long *)&v28[-8] = 0xb146;
                  v10 = sub_14c40(0,3,v58);
                  v9 = "%s: can make relative symbolic links only in current directory";
                  goto label_ae6c;
                }
              }
            }
            else {
              v56 = (long *)0x1c7d1;
              *(unsigned long *)&v28[-8] = 0x9c08;
              if (strcmp(".",v9)) goto label_9c13;
            }
            *(unsigned long *)&v28[-8] = 0x9c2f;
            free(v9);
          }
          v36 = *(char *)((long)v3 + 0x16);
          *(unsigned long *)&v28[-8] = 0x9c4e;
          v38 = sub_c230(a0,v60,v59,(unsigned char)v36,0xffffffff);
          v49 = 0;
          if (v38 > 0) {
            *(unsigned long *)&v28[-8] = 0xada2;
            v9 = (char *)sub_14a70(1,4,a0);
            *(unsigned long *)&v28[-8] = 0xadb8;
            v12 = (char *)sub_14a70(0,4,v58);
            *(unsigned long *)&v28[-8] = 0xadce;
            v13 = dcgettext(NULL,"cannot create symbolic link %s to %s",5);
            *(unsigned long *)&v28[-8] = 0xade3;
            error(0,v38,v13,v12,v9);
            goto label_a558;
          }
        }
        else if (*(char *)((long)v3 + 0x17)) { // branch-flip
          v14 = 1;
          if (!*(char *)((long)v3 + 0x16))
            v14 = (unsigned long)(v3[2] == 3);
          *(unsigned long *)&v28[-0x10] = (unsigned long)v49;
          *(unsigned long *)&v28[-0x18] = 0;
          *(unsigned long *)&v28[-0x20] = v14;
          *(unsigned long *)&v28[-0x28] = 0xa8b8;
          v4 = sub_7460(a0,0xffffff9c,a0,v58,v60,v59);
          v49 = 0;
          if (!v4) goto label_a558;
        }
        else if ((v41 != 0x8000) && (v44 = v41 != 0xa000 & *(unsigned char *)&v3[5], v49 = v44, !v44)) {
          if (v41 != 0x1000) { // branch-flip
            if ((v41 >> 8 & 0xbf) != 0x20 && v41 != 0xc000) { // branch-flip
              if (v41 != 0xa000) {
                *(unsigned long *)&v28[-8] = 0xb734;
                v10 = sub_14b40(4,a0);
                v9 = "%s has unknown file type";
                goto label_ae6c;
              }
              *(unsigned long *)&v28[-8] = 0xb33f;
              v9 = (char *)sub_cbf0(a0,v68);
              if (!v9) {
                *(unsigned long *)&v28[-8] = 0xb990;
                v10 = sub_14b40(4,a0);
                v9 = "cannot read symbolic link %s";
                goto label_a538;
              }
              v36 = *(char *)((long)v3 + 0x16);
              *(unsigned long *)&v28[-8] = 0xb36a;
              v38 = sub_c230(v9,v60,v59,(unsigned char)v36,0xffffffff);
              v56 = (long *)CONCAT44(v56._4_4_,v38);
              if (1 <= v38) {
                if (((*(char *)((long)v3 + 0x3b) == '\x01') && (!v40)) && ((v23._24_4_ & 0xf000) == 0xa000)) {
                  v54 = (long *)v23._48_8_;
                  *(unsigned long *)&v28[-8] = 0xb9d0;
                  if (v54 == (long *)strlen(v9)) {
                    *(unsigned long *)&v28[-8] = 0xb9f2;
                    v18 = (long *)sub_cd80(v60,v59);
                    if (v18) {
                      v54 = v18;
                      *(unsigned long *)&v28[-8] = 0xba0d;
                      if (!strcmp((char *)v18,v9)) {
                        *(unsigned long *)&v28[-8] = 0xba1d;
                        free(v54);
                        *(unsigned long *)&v28[-8] = 0xba25;
                        free(v9);
                        goto label_b8b9;
                      }
                      *(unsigned long *)&v28[-8] = 0xba87;
                      free(v54);
                    }
                  }
                }
                *(unsigned long *)&v28[-8] = 0xb38f;
                free(v9);
                *(unsigned long *)&v28[-8] = 0xb3a0;
                v9 = (char *)sub_14b40(4,v58);
                *(unsigned long *)&v28[-8] = 0xb3b6;
                v12 = dcgettext(NULL,"cannot create symbolic link %s",5);
                *(unsigned long *)&v28[-8] = 0xb3cb;
                error(0,(int)v56,v12,v9);
                goto label_a558;
              }
              *(unsigned long *)&v28[-8] = 0xb8b9;
              free(v9);
label_b8b9:
              if (*(char *)((long)v3 + 0x33)) {
                *(unsigned long *)&v28[-8] = 0xb97e;
                sub_7380();
              }
              v44 = *(unsigned char *)((long)v3 + 0x1d);
              v54 = (long *)CONCAT71(v54._1_7_,1);
              v49 = v44;
              if (v44) {
                *(unsigned long *)&v28[-8] = 0xb8fa;
                v38 = sub_e540(v60,v59,v65,v66);
                if (v38) {
                  *(unsigned long *)&v28[-8] = 0xb90a;
                  v4 = sub_8400(v3);
                  v54 = (long *)CONCAT71(v54._1_7_,v4);
                  if (v4) {
                    v49 = 0;
                    goto label_9c60;
                  }
                  *(unsigned long *)&v28[-8] = 0xba3d;
                  v9 = dcgettext(NULL,"failed to preserve ownership for %s",5);
                  *(unsigned long *)&v28[-8] = 0xba45;
                  v38 = *__errno_location();
                  *(unsigned long *)&v28[-8] = 0xba5a;
                  error(0,v38,v9,v58);
                  if (*(char *)((long)v3 + 0x32)) goto label_a558;
                }
                v44 = v49;
                v49 = 0;
                v54 = (long *)CONCAT71(v54._1_7_,v44);
              }
            }
            else {
              *(unsigned long *)&v28[-8] = 0xa50c;
              v38 = mknodat(v60,v59,~v48 & v61,v67);
              if (v38) {
                *(unsigned long *)&v28[-8] = 0xa525;
                v10 = sub_14b40(4,v58);
                v9 = "cannot create special file %s";
                goto label_a538;
              }
            }
          }
          else {
            v19 = ~v48 & v61;
            *(unsigned long *)&v28[-8] = 0xb605;
            v38 = mknodat(v60,v59,v19,0);
            if (v38) {
              *(unsigned long *)&v28[-8] = 0xb625;
              if (mkfifoat(v60,v59,v19 & 0xffffefff)) {
                *(unsigned long *)&v28[-8] = 0xb63e;
                v10 = sub_14b40(4,v58);
                v9 = "cannot create fifo %s";
                goto label_a538;
              }
            }
          }
        }
        else {
          *(long **)&v28[-0x10] = &v22;
          *(bool **)&v28[-0x18] = &v27;
          *(unsigned long *)&v28[-0x20] = (unsigned long)v48;
          *(unsigned long *)&v28[-0x28] = 0xad60;
          v49 = sub_8680(a0,v58,v60,v59,v3,(unsigned int)v56 & 0x1ff);
          if (!v49) goto label_a558;
          v54 = (long *)((unsigned long)v54 & 0xffffffffffffff00);
          v40 = v27;
        }
label_9c60:
        v44 = v41 != 0x4000;
        if (!v40) {
          v5 = (*(unsigned char *)&v3[5] ^ 1) & v44;
          if (v5) {
            if ((*(long *)&v3[10]) || (*(char *)((long)v3 + 0x33))) {
              *(unsigned long *)&v28[-8] = 0xa9fc;
              v44 = sub_8250(v58,0,v3);
              if (v44) goto label_9c83;
              v40 = 0;
              if ((char)v3[0xd]) goto label_a558;
            }
            v44 = v5;
          }
        }
label_9c83:
        v42 = (char *)0x1;
        v29 = v28;
        if ((char)v57) {
          v53 = (unsigned long *)((unsigned long)v53 & 0xffffffffffffff00);
          v42 = (char *)(v57 & 0xffffffff);
          v31 = v28;
label_9c9d:
          v29 = v31;
          if (*(long *)&v3[0x12]) {
            v57 = CONCAT71(v57._1_7_,v44);
            *(unsigned long *)&v31[-8] = 0x9ccc;
            v38 = fstatat(v60,v59,&v26,0x100);
            v44 = (unsigned char)v57;
            if (!v38) {
              v16 = *(void **)&v3[0x12];
              *(unsigned long *)&v31[-8] = 0x9ceb;
              sub_ea00(v16,v59,&v26);
              v44 = (unsigned char)v57;
            }
          }
        }
        if (((*(char *)((long)v3 + 0x17)) && (v44)) || (v49)) {
          return (unsigned long)v42 & 0xffffffff;
        }
label_9d14:
        if (*(char *)((long)v3 + 0x1f)) {
          *(unsigned long *)&v29[-8] = 0x9d2d;
          v45 = sub_15960(&v22);
          *(unsigned long *)&v29[-8] = 0x9d43;
          v26._0_8_ = SUB168(v45,0);
          v26._8_8_ = SUB168(v45,8);
          v45 = sub_15980(&v22);
          v19 = (unsigned int)(unsigned char)v54;
          *(unsigned long *)&v29[-8] = 0x9d74;
          v26._16_8_ = SUB168(v45,0);
          v26._24_8_ = SUB168(v45,8);
          if (utimensat(v60,v59,&v26,v19 << 8)) {
            *(unsigned long *)&v29[-8] = 0x9d89;
            v57 = sub_14b40(4,v58);
            *(unsigned long *)&v29[-8] = 0x9da3;
            v9 = dcgettext(NULL,"preserving times for %s",5);
            *(unsigned long *)&v29[-8] = 0x9dab;
            v38 = *__errno_location();
            *(unsigned long *)&v29[-8] = 0x9dc0;
            error(0,v38,v9,v57);
            if (*(char *)((long)v3 + 0x32)) {
              v42 = NULL;
              return (unsigned long)v42 & 0xffffffff;
            }
          }
        }
        if ((char)v54) { // branch-flip
          if (!*(char *)((long)v3 + 0x35)) {
            return (unsigned long)v42 & 0xffffffff;
          }
          *(unsigned long *)&v29[-8] = 0xa906;
          if (sub_7040(a0,0xffffffff,v58,0xffffffff,v3)) {
            return (unsigned long)v42 & 0xffffffff;
          }
          v4 = *(char *)((long)v3 + 0x36);
        }
        else {
          if (*(char *)((long)v3 + 0x1d)) {
            if (((v40) || (v23._28_4_ != v65)) || (v23._32_4_ != v66)) {
              *(char **)&v29[-8] = v23;
              *(unsigned long *)&v29[-0x10] = (unsigned long)v40;
              *(unsigned long *)&v29[-0x18] = (unsigned long)v66;
              *(unsigned long *)&v29[-0x20] = (unsigned long)v65;
              *(unsigned long *)&v29[-0x28] = 0x9e38;
              v38 = sub_8430(v3,v58,v60,v59,0xffffffff,v64);
              if (v38 == -1) {
                v42 = NULL;
                return (unsigned long)v42 & 0xffffffff;
              }
              if (!v38)
                v61 &= 0xfffff1ff;
            }
          }
          if (*(char *)((long)v3 + 0x35)) {
            *(unsigned long *)&v29[-8] = 0xb45f;
            v4 = sub_7040(a0,0xffffffff,v58,0xffffffff,v3);
            if ((!v4) && (*(char *)((long)v3 + 0x36))) {
              v42 = NULL;
              return (unsigned long)v42 & 0xffffffff;
            }
          }
          if (!(*(unsigned long *)&v3[6] & 0xff0000000000ff)) {
            if (*(char *)((long)v3 + 0x39)) // branch-flip
              v19 = v3[4];
            else {
              if (!(v40 & *(unsigned char *)&v3[8])) {
                if (v48) { // branch-flip
                  *(unsigned long *)&v29[-8] = 0xb65d;
                  v48 &= ~sub_8640();
                  if (!v48) goto label_9e9a;
                  if ((char)v53 != '\x01') {
                    if (v40) {
                      *(unsigned long *)&v29[-8] = 0xa42f;
                      if (fstatat(v60,v59,(stat *)v23,0x100)) {
                        v42 = v58;
                        goto label_a440;
                      }
                    }
                    v46 = v23._24_4_;
                    if (!(v48 & ~v23._24_4_)) {
                      return (unsigned long)v42 & 0xffffffff;
                    }
                  }
                }
                else {
label_9e9a:
                  if (!(char)v53) {
                    return (unsigned long)v42 & 0xffffffff;
                  }
                }
                *(unsigned long *)&v29[-8] = 0x9ec5;
                v38 = sub_e520(v60,v59,v46 | v48);
                if (!v38) {
                  return (unsigned long)v42 & 0xffffffff;
                }
                *(unsigned long *)&v29[-8] = 0x9ede;
                v10 = sub_14b40(4,v58);
                *(unsigned long *)&v29[-8] = 0x9ef4;
                v9 = dcgettext(NULL,"preserving permissions for %s",5);
                *(unsigned long *)&v29[-8] = 0x9efc;
                v38 = *__errno_location();
                *(unsigned long *)&v29[-8] = 0x9f0d;
                error(0,v38,v9,v10);
                goto label_9f0d;
              }
              v19 = 0x1ff;
              if ((v61 & 0x7000) != 0x4000)
                v19 = 0x1b6;
              *(unsigned long *)&v29[-8] = 0xb6c2;
              v19 = ~sub_8640() & v19;
            }
            *(unsigned long *)&v29[-8] = 0xb2d0;
            if (!sub_cb50(v58,0xffffffff,v19)) {
              return (unsigned long)v42 & 0xffffffff;
            }
            v42 = NULL;
            return (unsigned long)v42 & 0xffffffff;
          }
          *(unsigned long *)&v29[-8] = 0xb1c0;
          if (!sub_caa0(a0,0xffffffff,v58,0xffffffff,v61)) {
            return (unsigned long)v42 & 0xffffffff;
          }
label_9f0d:
          v4 = *(char *)((long)v3 + 0x32);
        }
        if (!v4) {
          return (unsigned long)v42 & 0xffffffff;
        }
      }
    }
    else if ((v61 & 0xf000) != 0x4000) { // branch-flip
      *(unsigned long *)&v28[-8] = 0x9b4f;
      v4 = sub_7ff0(a0,v58,v61,v44,v3);
      v48 = 0;
      if (v4) goto label_9bba;
    }
    else {
      *(unsigned long *)&v28[-8] = 0xa053;
      v4 = sub_7ff0(a0,v58,v61,v44,v3);
      if (v4) {
        v48 = (unsigned int)v56 & 0x12;
        goto label_a06a;
      }
    }
  }
  v42 = NULL;
  return (unsigned long)v42 & 0xffffffff;
}

// Function: sub_bae0 @ 0xbae0
void sub_bae0(unsigned long a0,unsigned long a1,unsigned int a2,char *a3,unsigned int a4,struct_26 *a5,char *a6,long *a7)
{
  char *v1;
  long *v2;
  char v3; // stack - 0x41
  
  v2 = a7;
  v1 = a6;
  if (sub_6270(a5)) {
    v3 = 0;
    dat_25568 = a1;
    dat_25570 = a0;
    sub_9470(a0,a1,a2,a3,a4,NULL,NULL,a5,1,&v3,v1,v2);
    return;
  }
  __assert_fail("valid_options (options)","src/copy.c",0xc2a,"copy"); // no-return
}

// Function: sub_bbc0 @ 0xbbc0
unsigned char sub_bbc0(char *a0,long a1,unsigned int a2,long a3,char a4,unsigned long a5,unsigned long a6,unsigned long *a7,unsigned char *a8,unsigned char *a9)
{
  unsigned long *v1;
  long v10;
  unsigned char v11; // stack - 0x9b
  unsigned char v12; // stack - 0x9a
  unsigned long v13; // stack - 0x98
  char *v14;
  unsigned char v15; // stack - 0xfa
  unsigned char v16; // stack - 0xf9
  unsigned long v17; // stack - 0x90
  unsigned long v18; // stack - 0x88
  unsigned long v19; // stack - 0x80
  unsigned char *v2;
  unsigned long v20; // stack - 0x78
  unsigned long v21; // stack - 0x70
  unsigned long v22; // stack - 0x68
  unsigned long v23; // stack - 0x60
  unsigned long v24; // stack - 0x58
  unsigned long v25; // stack - 0x50
  unsigned long v26; // stack - 0x48
  unsigned char *v3;
  unsigned char v4;
  unsigned char v5; // al
  char *v6; // rax
  void *v7; // rax
  void *v8; // rax
  unsigned long v9;
  
  v3 = a9;
  v2 = a8;
  v1 = a7;
  v9 = a6;
  v13 = *a7;
  v17 = a7[1];
  v18 = a7[2];
  v19 = a7[3];
  v20 = a7[4];
  v21 = a7[5];
  v22 = a7[6];
  v23 = a7[7];
  v24 = a7[8];
  v25 = a7[9];
  v26 = a7[10];
  v6 = (char *)sub_15610(a0,2);
  if (v6) { // branch-flip
    if (*(int *)((long)v1 + 4) == 3)
      v13 = CONCAT44(2,(unsigned int)v13);
    if (*v6) { // branch-flip
      v16 = 0;
      v15 = 1;
      v14 = v6;
      do {
        v7 = (void *)sub_ec90(a0,v14,NULL);
        v8 = (void *)sub_ec90(a1,v14,NULL);
        v12 = *v2;
        v5 = sub_9470(v7,v8,a2,(long)v8 + (a3 - a1),(unsigned char)a4,a5,v9,&v13,0,&v12,&v11,&v10);
        v4 = v11;
        v15 &= v5;
        *v3 = *v3 | v11;
        free(v8);
        free(v7);
        if (v4) break;
        v16 |= v12;
        v14 = &v14[strlen(v14) + 1];
      } while (*v14);
    }
    else {
      v16 = 0;
      v15 = 1;
    }
    free(v6);
    *v2 = v16;
  }
  else {
    v9 = sub_14b40(4,a0);
    v6 = dcgettext(NULL,"cannot access %s",5);
    error(0,*__errno_location(),v6,v9);
    v15 = 0;
  }
  return v15;
}

// Function: sub_be50 @ 0xbe50
unsigned long sub_be50(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_be60 @ 0xbe60
unsigned long sub_be60(struct_7 *a0,struct_7 *a1)
{
  if (a0->field_0x0 != a1->field_0x0)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x8 >> 8),a0->field_0x8 == a1->field_0x8);
}

// Function: sub_be80 @ 0xbe80
void sub_be80(void *a0)
{
  free(*(void **)((long)a0 + 0x10));
  free(a0); // tail-call
}

// Function: sub_bea0 @ 0xbea0
void sub_bea0(unsigned long a0,unsigned long a1) // return-dupe
{
  void *v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = (void *)sub_12350(dat_25578,&v2);
  if (!v1)
    return;
  sub_be80(v1);
}

// Function: sub_bf00 @ 0xbf00
unsigned long sub_bf00(unsigned long a0,unsigned long a1) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  
  v2 = a0;
  v3 = a1;
  v1 = sub_11a70(dat_25578,&v2);
  if (!v1)
    return 0;
  return *(unsigned long *)(v1 + 0x10);
}

// Function: sub_bf60 @ 0xbf60
unsigned long sub_bf60(char *a0,unsigned long a1,unsigned long a2) // early-return
{
  struct_18 *v1; // rax
  unsigned long v2; // rax
  struct_18 *v3; // rax
  
  v1 = (struct_18 *)sub_16e90(0x18);
  v2 = sub_17230(a0);
  *(unsigned long *)v1->field_0x0 = a1;
  v1->field_0x10 = v2;
  *(unsigned long *)((long)v1->field_0x0 + 8) = a2;
  v3 = (struct_18 *)sub_122f0(dat_25578,v1);
  if (!v3)
    sub_17250(); // no-return
  if (v1 == v3)
    return 0;
  sub_be80(v1);
  return v3->field_0x10;
}

// Function: sub_bfe0 @ 0xbfe0
void sub_bfe0(void)
{
  dat_25578 = sub_11cd0(0x67,NULL,sub_be50,sub_be60,sub_be80);
  if (dat_25578)
    return;
  sub_17250(); // no-return
}

// Function: sub_c020 @ 0xc020
void sub_c020(char *a0,struct_30 *a1)
{
  linkat(a1->field_0x0,a1->field_0x8,a1->field_0x10,a0,a1->field_0x14); // tail-call
}

// Function: sub_c040 @ 0xc040
char * sub_c040(void *a0,char *a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3; // rax
  
  v2 = sub_db60(a0) - (long)a0;
  v1 = v2 + 9;
  if ((0x101 <= v1) && (a1 = malloc(v1), !a1))
    return NULL;
  v3 = mempcpy(a1,a0,v2);
  *(unsigned long *)v3 = s_1d490._0_8_;
  v3[8] = s_1d490[8];
  return a1;
}

// Function: sub_c0a0 @ 0xc0a0
void sub_c0a0(char *a0,struct_32 *a1)
{
  symlinkat(a1->field_0x0,a1->field_0x8,a0); // tail-call
}

// Function: sub_c0c0 @ 0xc0c0
int sub_c0c0(int a0,char *a1,int a2,char *a3,int a4,char a5,int a6)
{
  char *v1;
  int v2; // eax
  int *v3; // rax
  char v4 [264];
  int v5 [2]; // stack - 0x168
  int v6;
  char *v7; // stack - 0x160
  int v8; // stack - 0x158
  int v9; // stack - 0x154
  
  v2 = a6;
  if (a6 < 0) {
    v2 = linkat(a0,a1,a2,a3,a4);
    if (!v2) {
      v2 = 0;
      return v2;
    }
    v3 = __errno_location();
    v2 = *v3;
  }
  if ((a5 == '\x01') && (v2 == 0x11)) {
    v3 = (char *)sub_c040(a3,v4);
    v1 = (char *)v3;
    if (v3) { // branch-flip
      v5[0] = a0;
      v7 = a1;
      v8 = a2;
      v9 = a4;
      v2 = sub_15ba0(v3,0,v5,sub_c020,6);
      if (v2) { // branch-flip
        v3 = __errno_location();
        v6 = *v3;
      }
      else {
        v2 = renameat(a2,v1,a2,a3);
        v6 = -1;
        if (v2) {
          v3 = __errno_location();
          v6 = *v3;
        }
        unlinkat(a2,v1,0);
      }
      v2 = v6;
      if (v1 != v4)
        free(v1);
    }
    else {
      v3 = __errno_location();
      v2 = *v3;
    }
  }
  return v2;
}

// Function: sub_c230 @ 0xc230
int sub_c230(char *a0,int a1,char *a2,char a3,int a4)
{
  char *v1; // rax
  char *v2; // stack - 0x158
  char v3 [264];
  int v4; // stack - 0x150
  
  if (a4 < 0) {
    if (!symlinkat(a0,a1,a2))
      return 0;
    a4 = *__errno_location();
  }
  if ((a3 == '\x01') && (a4 == 0x11)) {
    v1 = (char *)sub_c040(a2,v3);
    if (v1) { // branch-flip
      v2 = a0;
      v4 = a1;
      if (sub_15ba0(v1,0,&v2,sub_c0a0,6)) // branch-flip
        a4 = *__errno_location();
      else {
        a4 = -1;
        if (renameat(a1,v1,a1,a2)) {
          a4 = *__errno_location();
          unlinkat(a1,v1,0);
        }
      }
      if (v1 != v3)
        free(v1);
    }
    else {
      a4 = *__errno_location();
    }
  }
  return a4;
}

// Function: sub_c370 @ 0xc370
unsigned int sub_c370(unsigned long a0,unsigned int a1,char **a2)
{
  int v1;
  short v2; // ax
  unsigned int v3; // eax
  char *v4; // rax
  int *v5; // rax
  char *v6; // stack - 0x40
  char *v7; // stack - 0x38
  
  v6 = NULL;
  v7 = NULL;
  v4 = (char *)sub_e730();
  if (((0 <= getcon(&v6)) && (0 <= (int)sub_158c0(v4,&v7))) && (v2 = mode_to_security_class(a1), v2))
    v3 = security_compute_create(v6,v7,(unsigned short)v2,a2);
  else {
    v3 = 0xffffffff;
  }
  v5 = __errno_location();
  v1 = *v5;
  free(v4);
  freecon(v6);
  freecon(v7);
  *v5 = v1;
  return v3;
}

// Function: sub_c450 @ 0xc450
int sub_c450(long a0,char *a1)
{
  int v1;
  stat v10; // stack - 0xd8
  char *v11; // stack - 0xe0
  int v2;
  int v3;
  int *v4; // rax
  long v5; // rax
  long v6; // rax
  long v7; // rax
  char *v8; // rax
  char *v9; // stack - 0xe8
  
  v9 = NULL;
  v11 = NULL;
  if (!a0) {
    if (getfscreatecon(&v11) < 0)
      return -1;
    v4 = __errno_location();
    if (v11) {
      v2 = lsetfilecon(a1,v11);
      v1 = *v4;
      freecon(v11);
      *v4 = v1;
      return v2;
    }
    *v4 = 0x3d;
    return -1;
  }
  v1 = open(a1,0x20000);
  v4 = __errno_location();
  if (v1 == -1) {
    v2 = *v4;
    if (v2 != 0x28) { // branch-flip
      v6 = 0;
      v5 = 0;
    }
    else {
      if (0 <= lstat(a1,&v10)) goto label_c4cd;
      v2 = *v4;
      v6 = 0;
      v5 = 0;
    }
label_c59b:
    v3 = v1;
    goto label_c59e;
  }
  if (0 <= fstat(v1,&v10)) { // branch-flip
label_c4cd:
    if (0 <= (int)selabel_lookup(a0,&v9,a1,v10._24_4_)) { // branch-flip
      v5 = context_new(v9);
      if (v5) { // branch-flip
        if (v1 != -1) {
          if ((int)sub_15900(v1) < 0) {
            v6 = 0;
            v3 = -1;
            goto label_c6bb;
          }
          v6 = context_new(v11);
          if (v6) goto label_c531;
          v2 = *v4;
          v3 = -1;
          goto label_c55f;
        }
        if ((int)sub_158e0(a1,&v11) < 0) {
          v2 = *v4;
          v6 = 0;
          goto label_c59b;
        }
        v6 = context_new(v11);
        if (!v6) {
          v2 = *v4;
          v3 = -1;
          goto label_c59e;
        }
label_c531:
        v7 = context_type_get(v5);
        if (((v7) && (!context_type_set(v6,v7))) && (v8 = (char *)context_str(v6), v8)) {
          if (v1 == -1) {
            v1 = lsetfilecon(a1,v8);
            v2 = *v4;
            goto label_c59b;
          }
          v3 = fsetfilecon(v1,v8);
          goto label_c6bb;
        }
        v2 = *v4;
      }
      else {
        v2 = *v4;
        v6 = 0;
      }
    }
    else {
      v2 = *v4;
      v6 = 0;
      v5 = 0;
      if (v2 == 2) {
        *v4 = 0x3d;
        v2 = 0x3d;
      }
    }
    v3 = -1;
    if (v1 == -1) goto label_c59e;
  }
  else {
    v6 = 0;
    v5 = 0;
    v3 = -1;
label_c6bb:
    v2 = *v4;
  }
label_c55f:
  close(v1);
label_c59e:
  context_free(v5);
  context_free(v6);
  freecon(v9);
  freecon(v11);
  *v4 = v2;
  return v3;
}

// Function: sub_c760 @ 0xc760
int sub_c760(unsigned long a0,char *a1,unsigned int a2)
{
  int v1; // eax
  int v10;
  int *v2; // rax
  long v3;
  long v4; // rax
  long v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x50
  char *v8; // stack - 0x48
  char *v9;
  
  v7 = NULL;
  v8 = NULL;
  v2 = __errno_location();
  if (*a1 != '/') { // branch-flip
    a1 = (char *)sub_e4c0(a1,2);
    if (!a1) {
      v10 = *v2;
      v9 = NULL;
      v4 = 0;
      v1 = -1;
      v3 = 0;
      goto label_c869;
    }
    v9 = a1;
  }
  else {
    v9 = NULL;
  }
  if (0 <= (int)selabel_lookup(a0,&v7,a1,a2)) { // branch-flip
    if (0 <= (int)sub_c370(a1,a2,&v8)) { // branch-flip
      v3 = context_new(v7);
      if (v3) { // branch-flip
        v4 = context_new(v8);
        if ((((v4) && (v5 = context_type_get(v3), v5)) && (!context_type_set(v4,v5))) && (v6 = (char *)context_str(v4), v6)) {
          v1 = setfscreatecon(v6);
          v10 = *v2;
        }
        else {
          v10 = *v2;
          v1 = -1;
        }
      }
      else {
        v10 = *v2;
        v4 = 0;
        v1 = -1;
      }
    }
    else {
      v10 = *v2;
      v4 = 0;
      v3 = 0;
      v1 = -1;
    }
  }
  else {
    v10 = *v2;
    if (v10 != 2) { // branch-flip
      v4 = 0;
      v3 = 0;
      v1 = -1;
    }
    else {
      *v2 = 0x3d;
      v10 = 0x3d;
      v4 = 0;
      v3 = 0;
      v1 = -1;
    }
  }
label_c869:
  context_free(v3);
  context_free(v4);
  freecon(v7);
  freecon(v8);
  free(v9);
  *v2 = v10;
  return v1;
}

// Function: sub_c960 @ 0xc960
unsigned long sub_c960(long a0,char *a1,bool a2)
{
  char *v1;
  int v10;
  unsigned long v11; // stack - 0x40
  int v2; // eax
  int *v3;
  struct_14 *v4; // rax
  long v5; // rax
  char *v6;
  char *v7; // stack - 0x48
  unsigned long v8; // r12
  undefined7 v9; // r12
  
  v6 = NULL;
  v9 = (undefined7)((unsigned long)a0 >> 8);
  if (*a1 != '/') { // branch-flip
    v6 = (char *)sub_e4c0(a1,2);
    if (!v6) {
      v8 = 0;
      return v8 & 0xffffffff; // return-dupe
    }
    v3 = __errno_location();
    v7 = v6;
  }
  else {
    v3 = __errno_location();
    v7 = a1;
  }
  if (a2) {
    v11 = 0;
    v10 = 0;
    v4 = (struct_14 *)sub_17290(&v7,0x10,0);
    do {
      v5 = sub_10b30(v4);
      while( true ) {
        if (!v5) {
          if (*v3)
            v10 = *v3;
          if (sub_10940(v4))
            v10 = *v3;
          free(v6);
          v8 = CONCAT71(v9,v10 == 0);
          return v8 & 0xffffffff;
        }
        v1 = v4->field_0x20;
        if (0 <= (int)sub_c450(a0,v1)) break;
        v10 = *v3;
        v5 = sub_10b30(v4);
      }
    } while( true );
  }
  v2 = sub_c450(a0,v7);
  v10 = *v3;
  v8 = CONCAT71(v9,v2 != -1);
  free(v6);
  *v3 = v10;
  return v8 & 0xffffffff;
}

// Function: sub_caa0 @ 0xcaa0
int sub_caa0(char *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = sub_12890();
  if (v1 != -2) {
    if (v1 == -1) {
      v2 = sub_14e00(a2);
      v3 = dcgettext(NULL,"preserving permissions for %s",5);
      error(0,*__errno_location(),v3,v2);
    }
    return v1;
  }
  v3 = (char *)sub_14e00(a0);
  error(0,*__errno_location(),"%s",v3);
  return -2;
}

// Function: sub_cb50 @ 0xcb50
int sub_cb50(char *a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = sub_12910(a0,a1,a2);
  if (!v1)
    return 0;
  v2 = sub_14e00(a0);
  v3 = dcgettext(NULL,"setting permissions for %s",5);
  error(0,*__errno_location(),v3,v2);
  return v1;
}

// Function: sub_cbd0 @ 0xcbd0
void sub_cbd0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_cbe0 @ 0xcbe0
void sub_cbe0(unsigned long a0,unsigned long a1)
{
  aligned_alloc(a0,a1); // tail-call
}

// Function: sub_cbf0 @ 0xcbf0
char * sub_cbf0(char *a0,unsigned long a1)
{
  unsigned long v1;
  char *v2;
  char *v3;
  unsigned long v4;
  char v5 [136];
  
  v4 = 0x80;
  if ((a1) && (v4 = a1 + 1, 0x401 <= a1))
    v4 = 0x401;
label_cc58:
  if (v4 != 0x80) goto label_cca3;
label_cc61:
  v2 = v5;
  v3 = NULL;
  if (a1) goto label_cca3;
  do {
    v1 = readlink(a0,v2,v4);
    if ((long)v1 < 0) {
      free(v3);
      return NULL;
    }
    if (v1 < v4) {
      v2[v1] = '\0';
      v1 += 1;
      if (!v3) {
        v3 = malloc(v1);
        if (!v3)
          return NULL;
        memcpy(v3,v2,v1);
        return v3;
      }
      if (v4 <= v1)
        return v3;
      v2 = realloc(v3,v1);
      if (!v2)
        return v3;
      return v2;
    }
    free(v3);
    if (0x3fffffffffffffff < v4) {
      if (v4 == 0x7fffffffffffffff) {
label_cceb:
        *__errno_location() = 0xc;
        return NULL;
      }
      v4 = 0x7fffffffffffffff;
      goto label_cc58;
    }
    v4 *= 2;
    if (v4 == 0x80) goto label_cc61;
label_cca3:
    v2 = malloc(v4);
    if (!v2) goto label_cceb;
    v3 = v2;
  } while( true );
}

// Function: sub_cd80 @ 0xcd80
char * sub_cd80(int a0,char *a1,unsigned long a2)
{
  unsigned long v1;
  char *v2;
  void *v3; // rax
  char v4 [136];
  unsigned long v5;
  char *v6;
  
  v5 = 0x80;
  if (a2) {
    v5 = 0x401;
    if (a2 <= 0x400)
      v5 = a2 + 1;
  }
label_cde0:
  if (v5 != 0x80) goto label_ce37;
  do {
    v2 = v4;
    v6 = NULL;
    if (a2) goto label_ce37;
    while( true ) {
      v1 = readlinkat(a0,a1,v2,v5);
      if ((long)v1 < 0) {
        free(v6);
        return NULL;
      }
      if (v1 < v5) {
        v2[v1] = '\0';
        v1 += 1;
        if (!v6) {
          v3 = malloc(v1);
          if (v3)
            return memcpy(v3,v2,v1);
          return NULL;
        }
        if (v5 > v1) {
          v2 = realloc(v6,v1);
          if (!v2)
            return v6;
          return v2;
        }
        return v6;
      }
      free(v6);
      if (0x3fffffffffffffff < v5) {
        if (v5 == 0x7fffffffffffffff) {
          *__errno_location() = 0xc;
          return NULL;
        }
        v5 = 0x7fffffffffffffff;
        goto label_cde0;
      }
      v5 *= 2;
      if (v5 == 0x80) break;
label_ce37:
      v2 = malloc(v5);
      if (!v2)
        return NULL;
      v6 = v2;
    }
  } while( true );
}

// Function: sub_cf10 @ 0xcf10
void sub_cf10(void)
{
  sub_57e0(1); // tail-call, no-return
}

// Function: sub_cf20 @ 0xcf20
unsigned long sub_cf20(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_d030 @ 0xd030
long sub_d030(char *a0,long *a1) // return-dupe
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

// Function: sub_d090 @ 0xd090
void sub_d090(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_14df0(1,a0);
  error(0,0,v1,sub_14a70(0,8,a1),v2); // tail-call
}

// Function: sub_d120 @ 0xd120
void sub_d120(unsigned long *a0,void *a1,unsigned long a2)
{
  char *v1;
  FILE *v2;
  long v3;
  void *v4;
  
  v2 = stderr;
  v4 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v2);
  v3 = 0;
  v1 = (char *)*a0;
  while (v1) {
    if ((v3) && (!memcmp(v4,a1,a2)))
      __fprintf_chk(stderr,1,", %s",(char *)sub_14e00(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_14e00(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = (char *)a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_d220 @ 0xd220
long sub_d220(char *a0,char *a1,long *a2,void *a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_cf20(a1,a2,a3,a4);
  else {
    v1 = sub_d030(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_d090(a0,a1,v1);
  sub_d120(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_d2b0 @ 0xd2b0
long sub_d2b0(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_d310 @ 0xd310
unsigned char sub_d310(unsigned long a0,unsigned long *a1,unsigned long a2,long a3,long a4,long *a5,int *a6)
{
  char *v1;
  void *v10; // rax
  DIR *v11;
  long v12; // rcx
  char v13;
  char *v14;
  long v15;
  unsigned long v16;
  bool v17; // r8b
  unsigned long v18; // stack - 0x78
  unsigned long v19; // stack - 0x68
  long v2;
  unsigned short v3;
  unsigned char v4;
  int *v5;
  unsigned int v6;
  dirent *v7;
  char *v8;
  char *v9;
  
  v11 = (DIR *)*a5;
  v9 = (char *)*a1;
  v12 = a3 - a4;
  if (v11) // branch-flip
    rewinddir(v11);
  else {
    v8 = &v9[a4];
    v3 = *(unsigned short *)v8;
    v8[0] = '.';
    v8[1] = '\0';
    v11 = (DIR *)sub_12790(a0,v9,0,a6);
    if (!v11) {
      v6 = *__errno_location();
      *(unsigned short *)v8 = v3;
      builtin_strncpy(&v9[v12 + a4],".~1~",5);
      return (v6 == 0xc) + 2;
    }
    *(unsigned short *)v8 = v3;
    builtin_strncpy(&v9[v12 + a4],".~1~",5);
    *a5 = (long)v11;
  }
  v4 = 2;
  v18 = 1;
  v19 = a2;
label_d370:
  do {
    v7 = readdir(v11);
    v5 = a6;
    while( true ) {
      if (!v7) {
        *a1 = v9;
        return v4;
      }
      v1 = &v7->field_0x13;
      a6 = v5;
      if (strlen(v1) < v12 + 4U) break;
      if (memcmp(&v9[a4],v1,v12 + 2U)) goto label_d370;
      v14 = &v1[v12 + 2U];
      if (8 < (unsigned char)(*v14 - 0x31U)) goto label_d370;
      v17 = *v14 == '9';
      v13 = v14[1];
      v6 = (int)v13;
      v15 = 1;
      v16 = v15; // branch-flip
      if (10 <= (unsigned int)(v6 - 0x30U))
        v16 = 1;
      else {
        do {
          v16 += 1;
          v17 = (bool)(v17 & (char)v6 == '9');
          v13 = v14[v16];
          v6 = (int)v13;
          v15 = v16;
        } while ((unsigned int)(v6 - 0x30U) <= 9);
      }
      if (((v13 != '~') || (v14[v16 + 1])) || ((v15 <= (long)v18 && ((v18 != v15 || (1 <= memcmp(&v9[a3 + 2],v14,v16))))))) goto label_d370;
      v18 = v17 + v15;
      v2 = v18 + a3 + 4;
      if (v2 <= (long)v19) // branch-flip
        v8 = v9;
      else {
        if (SCARRY8(v2 >> 1,v2))
          v19 = v2;
        else {
          v19 = (v2 >> 1) + v2;
        }
        v8 = (char *)sub_126b0(v9,v19);
        if (!v8) {
          *a1 = v9;
          return 3;
        }
      }
      v9 = &v8[a3];
      v9[0] = '.';
      v9[1] = '~';
      v9[2] = '0';
      v10 = memcpy(&v9[(unsigned long)v17 + 2],v14,v15 + 2);
      v9 = (char *)((long)v10 + (v16 - 1));
      v13 = *(char *)((long)v10 + (v16 - 1));
      while (v13 == '9') {
        *v9 = '0';
        v14 = &v9[-1];
        v9 = &v9[-1];
        v13 = *v14;
      }
      *v9 = v13 + '\x01';
      v7 = readdir(v11);
      v9 = v8;
      v5 = a6;
      v4 = v17;
    }
  } while( true );
}

// Function: sub_d640 @ 0xd640
unsigned long sub_d640(char *a0,long a1,int a2,long *a3) // return-dupe
{
  unsigned short v1;
  int v2;
  unsigned short *v3; // rax
  long v4; // rax
  int *v5; // rax
  long v6;
  char *v7;
  
  v3 = (unsigned short *)sub_db60(a0);
  v4 = sub_dbc0(v3);
  if (v4 <= 0xe)
    return 1;
  v6 = *a3;
  if (!v6) {
    v5 = __errno_location();
    if (0 <= a2) { // branch-flip
      *v5 = 0;
      v6 = fpathconf(a2,3);
      v6 -= (unsigned long)(*v5 == 0);
    }
    else {
      v1 = *v3;
      *v3 = 0x2e;
      *v5 = 0;
      v6 = pathconf(a0,3);
      v2 = *v5;
      *v3 = v1;
      v6 -= (unsigned long)(v2 == 0);
    }
    if ((v6 <= -1) && (v6 != -1))
      v6 = 0xe;
    *a3 = v6;
  }
  if (v4 <= v6)
    return 1;
  v7 = &a0[a1 - (long)v3];
  if (v6 <= (long)&a0[a1 - (long)v3])
    v7 = (char *)(v6 + -1);
  *(char *)((long)v3 + (long)v7) = '~';
  *(char *)((long)v3 + (long)&v7[1]) = '\0';
  return 0;
}

// Function: sub_d750 @ 0xd750
void sub_d750(char *a0)
{
  if ((((a0) || (a0 = getenv("SIMPLE_BACKUP_SUFFIX"), a0)) && (*a0)) && (a0 == (char *)sub_db60(a0))) {
    dat_25580 = a0;
    return;
  }
  dat_25580 = "~";
}

// Function: sub_d7b0 @ 0xd7b0
void * sub_d7b0(int a0,void *a1,int a2,bool a3) // return-dupe
{
  unsigned long v1;
  void *v10; // stack - 0x58
  int v11; // stack - 0x5c
  unsigned long v12; // stack - 0x48
  void *v13;
  long v14; // r15
  void *v15; // stack - 0x78
  char v2; // al
  int v3;
  long v4; // rax
  unsigned long v5;
  void *v6;
  int *v7; // rax
  void *v8;
  DIR *v9; // stack - 0x50
  
  v15 = (void *)sub_db60(a1);
  v14 = (long)v15 - (long)a1;
  v1 = v14 + sub_dbc0(v15);
  if (!dat_25580)
    sub_d750(NULL);
  v4 = strlen(dat_25580) + 1;
  v5 = 9;
  if (9 <= v4)
    v5 = v4;
  v5 = v1 + 1 + v5;
  v6 = (void *)sub_126a0(v5);
  if (!v6)
    return NULL;
  v9 = NULL;
  v11 = -100;
  v12 = 0;
  v10 = v6;
  do {
    memcpy(v6,a1,v1);
    if (a2 != 1) { // branch-flip
      v3 = sub_d310(a0,&v10,v5,v1,v14,&v9,&v11);
      v6 = v10;
      if (v3 != 2) { // branch-flip
        if (v3 == 3) {
          if (v9)
            closedir(v9);
          free(v10);
          *__errno_location() = 0xc;
          return NULL;
        }
        v2 = '\x01';
        if (v3 != 1) goto label_d8d8;
      }
      else if (a2 == 2) {
        a2 = 1;
        memcpy((void *)((long)v10 + v1),dat_25580,v4);
      }
      v2 = sub_d640(v6,v1,v11,&v12);
    }
    else {
      v2 = '\x01';
      memcpy((void *)((long)v6 + v1),dat_25580,v4);
    }
label_d8d8:
    if (!a3) {
label_da50:
      if (v9) {
        closedir(v9);
        return v6;
      }
      return v6;
    }
    if (0 <= v11) { // branch-flip
      v8 = (void *)((long)v6 + v14);
      v13 = v15;
      v3 = v11;
    }
    else {
      v8 = v6;
      v13 = a1;
      v3 = a0;
    }
    v3 = sub_14e40(v3,v13,v11,v8,a2 != 1);
    if (!v3) goto label_da50;
    v7 = __errno_location();
    v3 = *v7;
    if ((v3 != 0x11) || (v2 != '\x01')) {
      if (v9)
        closedir(v9);
      free(v6);
      *v7 = v3;
      return NULL;
    }
  } while( true );
}

// Function: sub_da90 @ 0xda90
void * sub_da90(int a0,char *a1,int a2)
{
  return sub_d7b0(a0,a1,a2); // tail-call
}

// Function: sub_daa0 @ 0xdaa0
void sub_daa0(int a0,char *a1,int a2)
{
  if (sub_d7b0(a0,a1,a2))
    return;
  sub_17250(); // no-return
}

// Function: sub_dac0 @ 0xdac0
unsigned int sub_dac0(char *a0,char *a1)
{
  long v1; // rax
  
  if (!a1)
    return 2;
  if (!*a1)
    return 2;
  v1 = sub_d220(a0,a1,(long *)0x248a0,(void *)0x1d560,4,dat_25028);
  return *(unsigned int *)(v1 * 4 + 0x1d560);
}

// Function: sub_db20 @ 0xdb20
void sub_db20(char *a0,char *a1)
{
  if ((!a1) || (!*a1)) {
    a1 = getenv("VERSION_CONTROL");
    a0 = "$VERSION_CONTROL";
  }
  sub_dac0(a0,a1); // tail-call
}

// Function: sub_db60 @ 0xdb60
char * sub_db60(char *a0)
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

// Function: sub_dbc0 @ 0xdbc0
void sub_dbc0(char *a0)
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

// Function: sub_dbf0 @ 0xdbf0
unsigned long sub_dbf0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  
  if (a0) { // branch-flip
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
        return v3;
    }
  }
  else {
    a0 = 0x2000;
    if (a1)
      a0 = a1;
  }
  if (a0 <= a2)
    a2 = a0;
  return a2;
}

// Function: sub_dc70 @ 0xdc70
void sub_dc70(struct_0 *a0)
{
  a0->field_0x8 = 0x400;
  a0->field_0x0 = (long *)&a0[1];
}

// Function: sub_dc80 @ 0xdc80
bool sub_dc80(char *a0) // return-dupe
{
  char v1;
  char *v2; // rdi
  
  if (*a0 != '/')
    return 0;
  while( true ) {
    do {
      v2 = a0;
      v1 = v2[1];
      a0 = &v2[1];
    } while (v1 == '/');
    if (!v1)
      return 1;
    if (v1 != '.')
      return 0;
    v1 = v2[2];
    if (!v1) break;
    if (v1 == '.')
      return !v2[3] || v2[3] == '/';
    if (v1 != '/')
      return 0;
    a0 = &v2[2];
  }
  return 1;
}

// Function: sub_dcf0 @ 0xdcf0
unsigned int sub_dcf0(unsigned int a0)
{
  return CONCAT31((undefined3)(a0 - 1 >> 8),(a0 - 1 & a0) != 0);
}

// Function: sub_dd00 @ 0xdd00
unsigned int sub_dd00(long *a0,char *a1,struct_2 *a2)
{
  unsigned int v1; // eax
  struct_5 *v2;
  
  v2 = (struct_5 *)*a0;
  if (!v2) {
    v2 = (struct_5 *)sub_11cd0(7,NULL,sub_12540,sub_12570,sub_125b0);
    *a0 = (long)v2;
    if (!v2)
      sub_17250(0); // no-return
  }
  v1 = sub_ea90(v2,a1,a2);
  if (!(char)v1)
    sub_ea00((void *)*a0,a1,a2);
  return v1;
}

// Function: sub_dd90 @ 0xdd90
unsigned int sub_dd90(char *a0)
{
  unsigned int v1; // eax
  
  v1 = faccessat(-100,a0,0,0x200);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_ddc0 @ 0xddc0
void sub_ddc0(char *a0,unsigned short *a1)
{
  *a1 = dat_1d580;
  sub_dd90(a0); // tail-call
}

// Function: sub_ddd0 @ 0xddd0
void sub_ddd0(unsigned long *a0)
{
  if ((unsigned long *)*a0 != &a0[2]) {
    free((unsigned long *)*a0); // tail-call
    return;
  }
}

// Function: sub_ddf0 @ 0xddf0
long sub_ddf0(char *a0,unsigned int a1)
{
  char v1;
  struct_0 *v10; // rdx
  char *v11; // rdx
  unsigned int v12; // ebx
  char *v13; // stack - 0x868
  char *v14; // stack - 0x458
  stat v15;
  long v16; // stack - 0x900
  long *v17;
  char *v18; // r13
  char *v19;
  int v2;
  long *v20;
  bool v21; // cf
  long *v22; // stack - 0x960
  long *v23; // stack - 0x950
  int v24; // stack - 0x92c
  long v25; // stack - 0x920
  unsigned long v26; // stack - 0x860
  long v27; // stack - 0x450
  bool v3;
  char *v4;
  char v5;
  long v6;
  unsigned long v7; // rax
  long *v8;
  unsigned long v9;
  
  v12 = a1 & 3;
  v16 = 0;
  if ((sub_dcf0(v12)) || (!a0)) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!*a0) {
    *__errno_location() = 2;
    return 0;
  }
  sub_dc70(&v13);
  sub_dc70(&v14);
  sub_dc70(v10);
  v20 = v10->field_0x0;
  if (*a0 != '/') { // branch-flip
    while (v9 = v10->field_0x8, !getcwd((char *)v20,v9)) {
      v2 = *__errno_location();
      if (v2 == 0xc) {
        sub_17250(); // no-return, return-dupe
      }
      if (v2 != 0x22) goto label_deb6;
      if (!sub_15710(v10)) {
        sub_17250();
      }
      v20 = v10->field_0x0;
    }
    v8 = rawmemchr(v20,0);
    v5 = *a0;
    v22 = v8;
    if (v5) goto label_df64;
    v6 = 0;
  }
  else {
    *(char *)v20 = 0x2f;
    v5 = *a0;
    v22 = (long *)((long)v20 + 1);
    if (!v5) {
      v23 = (long *)((long)v20 + 2);
      goto label_e17b;
    }
label_df64:
    v24 = 0;
    v3 = 0;
    do {
      v8 = v22;
      if (v5 == '/') {
        do {
          v5 = a0[1];
          a0 = &a0[1];
        } while (v5 == '/');
        if (!v5) break;
      }
      v19 = a0;
      do {
        v18 = v19;
        v1 = v18[1];
        v19 = &v18[1];
        if (!v1) break;
      } while (v1 != '/');
      v9 = (long)v19 - (long)a0;
      if (!v9) break;
      if (v9 != 1) { // branch-flip
        if (((v9 != 2) || (v5 != '.')) || (a0[1] != '.')) {
label_e035:
          if (*(char *)((long)v22 + -1) != '/') {
            *(char *)v22 = 0x2f;
            v22 = (long *)((long)v22 + 1);
          }
          if ((char *)((long)v20 + (v10->field_0x8 - (long)v22)) < (char *)(v9 + 2)) {
            do {
              v6 = (long)v22 - (long)v20;
              if (!sub_15790(v10)) {
                sub_17250();
              }
              v20 = v10->field_0x0;
              v22 = (long *)((long)v20 + v6);
            } while ((char *)(v10->field_0x8 - v6) < (char *)(v9 + 2));
          }
          v8 = mempcpy(v22,a0,v9);
          *(char *)v8 = 0;
          if (a1 & 4) { // branch-flip
label_e0bd:
            if (v12 != 2) {
              if (sub_dc80(v19)) { // branch-flip
                v5 = sub_ddc0(v20,v8);
label_e246:
                if (v5 != '\x01') {
label_e0ea:
                  if (((v12 != 1) || (*__errno_location() != 2)) || (v19[strspn(v19,"/")])) {
label_e320:
                    if (v16) { // branch-flip
                      sub_11e90(v16);
                      sub_ddd0(&v13);
                      sub_ddd0(&v14);
                    }
                    else {
label_deb6:
                      sub_ddd0(&v13);
                      sub_ddd0(&v14);
                    }
                    sub_ddd0(v10);
                    return 0;
                  }
                }
              }
              else if (a1 & 4) { // branch-flip
                if (!*v19) {
                  v5 = sub_dd90(v20);
                  goto label_e246;
                }
              }
              else if (*__errno_location() != 0x16) goto label_e0ea;
            }
          }
          else {
            while( true ) {
              v4 = v14;
              v9 = v27 - 1;
              v7 = readlink((char *)v20,v14,v9);
              if ((long)v9 > (long)v7) break;
              if (!sub_15710(&v14)) {
                sub_17250();
              }
            }
            if ((long)v7 < 0) goto label_e0bd;
            if (v24 <= 0x13) {
              v24 += 1;
label_e362:
              a0 = v13;
              v4[v7] = '\0';
              v6 = (long)v19 - (long)v13;
              if (!v3)
                v6 = v25;
              v23 = (unsigned long)strlen(v19);
              if (!SCARRY8((long)v23 + 0x8000000000000000,v7)) {
                if (v26 <= v7 + (long)v23) {
                  do {
                    if (!sub_15790(&v13)) {
                      sub_17250();
                    }
                  } while (v26 <= v7 + (long)v23);
                  a0 = v13;
                }
                if (v3)
                  v19 = &a0[v6];
                memmove(&a0[v7],v19,(long)v23 + 1);
                memcpy(a0,v4,v7);
                v22 = (long *)((long)v20 + 1);
                v25 = v6;
                if (*v4 != '/') { // branch-flip
                  v3 = 1;
                  v5 = *a0;
                  v21 = v22 < v8;
                  v22 = v8;
                  if (v21) {
                    do {
                      v22 = (long *)((long)v8 + -1);
                      if (v22 == v20) break;
                      v19 = (char *)((long)v8 + -2);
                      v8 = v22;
                    } while (*v19 != '/');
                    v3 = 1;
                  }
                }
                else {
                  *(char *)v20 = 0x2f;
                  v5 = *a0;
                  v3 = 1;
                }
                goto label_e12a;
              }
              sub_17250();
            }
            if (!*a0) goto label_e362;
            v11 = (char *)((long)v8 + ((long)a0 - (long)v19));
            *v11 = '\0';
            v17 = (long *)0x1c7d1;
            if ((char)*v20)
              v17 = v20;
            if (stat((char *)v17,&v15)) goto label_e320;
            *v11 = *a0;
            if (!sub_dd00(&v16,a0,&v15)) goto label_e362;
            if (v12 != 2) {
              *__errno_location() = 0x28;
              goto label_e320;
            }
          }
          v5 = v18[1];
          a0 = v19;
          v22 = v8;
        }
        else {
          a0 = v19;
          v5 = v1;
          if ((long *)((long)v20 + 1) < v22) {
            v22 = (long *)((long)v22 + -1);
            if (v20 < v22) {
              do {
                if (*(char *)((long)v22 + -1) == '/') break;
                v22 = (long *)((long)v22 + -1);
              } while (v20 != v22);
              a0 = v19; // crossjump-dupe
              v5 = v1;
            }
          }
        }
      }
      else {
        if (v5 != '.') goto label_e035;
label_e028:
        a0 = v19;
        v5 = v1;
      }
label_e12a:
      v8 = v22;
    } while (v5);
    v6 = v16;
  }
  v23 = (long *)((long)v8 + 1);
  v22 = v8;
  if (((long *)((long)v20 + 1) < v8) && (*(char *)((long)v8 + -1) == '/')) {
    v22 = (long *)((long)v8 + -1);
    v23 = v8;
  }
  if (v6)
    sub_11e90(v6);
label_e17b:
  sub_ddd0(&v13);
  sub_ddd0(&v14);
  *(char *)v22 = 0;
  v6 = sub_15680(v10,(long)v23 - (long)v20);
  if (v6)
    return v6;
  sub_17250();
}

// Function: sub_e4c0 @ 0xe4c0
void sub_e4c0(char *a0,unsigned long a1)
{
  char v1 [1048];
  
  sub_ddf0(a0,a1,v1);
}

// Function: sub_e510 @ 0xe510
void sub_e510(int a0,char *a1,unsigned int a2)
{
  fchmodat(a0,a1,a2,0); // tail-call
}

// Function: sub_e520 @ 0xe520
void sub_e520(int a0,char *a1,unsigned int a2)
{
  fchmodat(a0,a1,a2,0x100); // tail-call
}

// Function: sub_e530 @ 0xe530
void sub_e530(int a0,char *a1,unsigned int a2,unsigned int a3)
{
  fchownat(a0,a1,a2,a3,0); // tail-call
}

// Function: sub_e540 @ 0xe540
void sub_e540(int a0,char *a1,unsigned int a2,unsigned int a3)
{
  fchownat(a0,a1,a2,a3,0x100); // tail-call
}

// Function: sub_e550 @ 0xe550
void sub_e550(unsigned long a0)
{
  dat_25588 = a0;
}

// Function: sub_e560 @ 0xe560
void sub_e560(void)
{
  FILE *v1;
  long v2;
  int v3; // eax
  char *v4; // rax
  int *v5; // rax
  char *v6; // rax
  
  v1 = stdin;
  if (sub_edb0(stdin)) { // branch-flip
    if ((!sub_ede0(v1,0,1)) && (v3 = sub_e9c0(stdin), v3)) {
      sub_179a0(stdin);
      goto label_e5bf;
    }
    v3 = sub_179a0(stdin);
  }
  else {
    v3 = sub_179a0(v1);
  }
  if (!v3) {
    sub_e680(); // tail-call
    return;
  }
label_e5bf:
  v4 = dcgettext(NULL,"error closing file",5);
  v2 = dat_25588;
  v5 = __errno_location();
  if (v2) { // branch-flip
    v6 = (char *)sub_14c20(v2);
    error(0,*v5,"%s: %s",v6,v4);
    sub_e680();
  }
  else {
    error(0,*v5,"%s",v4);
    sub_e680();
  }
  _exit(dat_25030); // no-return
}

// Function: sub_e660 @ 0xe660
void sub_e660(unsigned long a0)
{
  dat_25598 = a0;
}

// Function: sub_e670 @ 0xe670
void sub_e670(char a0)
{
  dat_25590 = a0;
}

// Function: sub_e680 @ 0xe680
void sub_e680(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_179a0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_25590) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_25598) { // branch-flip
        v4 = (char *)sub_14c20(dat_25598);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_25030); // return-dupe, no-return
    }
  }
  v1 = sub_179a0(stderr);
  if (!v1)
    return;
  _exit(dat_25030);
}

// Function: sub_e730 @ 0xe730
void sub_e730(char *a0)
{
  if (sub_e790(a0))
    return;
  sub_17250(); // no-return
}

// Function: sub_e750 @ 0xe750
void sub_e750(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_db60(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_e790 @ 0xe790
char * sub_e790(char *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  char *v3;
  
  v1 = sub_e750(a0);
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

// Function: sub_e800 @ 0xe800
bool sub_e800(char *a0)
{
  char v1;
  char *v2;
  long v3; // rax
  
  v2 = (char *)sub_db60(a0);
  if (!*v2)
    v2 = a0;
  v3 = sub_dbc0(v2);
  v1 = v2[v3];
  v2[v3] = '\0';
  return v1 != '\0';
}

// Function: sub_e840 @ 0xe840
void sub_e840(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_e850 @ 0xe850
void sub_e850(FILE *a0,unsigned int a1)
{
  if (a0) {
    sub_e840(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_e880 @ 0xe880
void sub_e880(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_15df0(open(a0,a1,v1));
}

// Function: sub_e900 @ 0xe900
int * sub_e900(int a0,int a1,char *a2,void *a3,int a4)
{
  int v1; // eax
  int *v2; // rax
  
  if (0 <= a0) { // branch-flip
    v1 = futimens(a0,a3);
    if ((v1 == -1) && (a2)) {
      v2 = __errno_location();
      if (*v2 != 0x26) {
        v2 = (int *)0xffffffff;
        return v2;
      }
      v1 = utimensat(a1,a2,a3,a4); // crossjump-dupe
    }
  }
  else {
    if (!a2) goto label_e980;
    v1 = utimensat(a1,a2,a3,a4);
  }
  if (v1 != 1)
    return v2;
label_e980:
  v2 = __errno_location();
  *v2 = 9;
  v2 = (int *)0xffffffff;
  return v2;
}

// Function: sub_e9a0 @ 0xe9a0
void sub_e9a0(FILE *a0)
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_ede0(a0,0,1); // tail-call
}

// Function: sub_e9c0 @ 0xe9c0
void sub_e9c0(FILE *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_e9a0(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_ea00 @ 0xea00
void sub_ea00(void *a0,char *a1,struct_2 *a2)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  if (!a0)
    return;
  v1 = (unsigned long *)sub_16e90(0x18);
  *v1 = sub_17230(a1);
  v1[1] = a2->field_0x8;
  v1[2] = a2->field_0x0;
  v2 = (unsigned long *)sub_122f0(a0,v1);
  if (v2) {
    if (v1 != v2) {
      sub_125b0(v1); // tail-call
      return;
    }
    return;
  }
  sub_17250(); // no-return
}

// Function: sub_ea90 @ 0xea90
unsigned long sub_ea90(struct_5 *a0,unsigned long a1,struct_2 *a2) // early-return
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
  v1 = sub_11a70(a0,&v2);
  return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
}

// Function: sub_eaf0 @ 0xeaf0
unsigned long sub_eaf0(unsigned int a0) // early-return x7
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

// Function: sub_eb60 @ 0xeb60
void sub_eb60(unsigned int a0,char *a1) // ternary x2
{
  unsigned char v1;
  char v2; // al
  unsigned long v3; // rcx
  
  v3 = (unsigned long)a0;
  *a1 = sub_eaf0(a0);
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

// Function: sub_ec80 @ 0xec80
void sub_ec80(void *a0)
{
  sub_eb60(*(unsigned int *)((long)a0 + 0x18)); // tail-call
}

// Function: sub_ec90 @ 0xec90
void sub_ec90(void *a0,char *a1,unsigned long *a2)
{
  if (sub_ecb0(a0,a1,a2))
    return;
  sub_17250(); // no-return
}

// Function: sub_ecb0 @ 0xecb0
void * sub_ecb0(void *a0,char *a1,unsigned long *a2)
{
  char *v1;
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char *v5; // rax
  unsigned long v6; // rbx
  char v7; // stack - 0x41
  
  v1 = (char *)sub_db60(a0);
  v2 = sub_dbc0(v1);
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

// Function: sub_edb0 @ 0xedb0
long sub_edb0(struct_38 *a0) // early-return
{
  long v1; // rax
  
  if (a0->field_0x20 < a0->field_0x28)
    return 0;
  v1 = a0->field_0x10 - a0->field_0x8;
  if (a0->field_0x0 & 0x100)
    v1 += a0->field_0x58 - a0->field_0x48;
  return v1;
}

// Function: sub_ede0 @ 0xede0
int sub_ede0(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_ee60 @ 0xee60
unsigned long sub_ee60(struct_7 *a0,struct_7 *a1)
{
  if (a0->field_0x8 != a1->field_0x8)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x0 >> 8),a0->field_0x0 == a1->field_0x0);
}

// Function: sub_ee90 @ 0xee90
unsigned long sub_ee90(void *a0,unsigned long a1)
{
  return *(unsigned long *)((long)a0 + 8) % a1;
}

// Function: sub_eeb0 @ 0xeeb0
unsigned long sub_eeb0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_eec0 @ 0xeec0
unsigned long sub_eec0(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_eed0 @ 0xeed0
int sub_eed0(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_eef0 @ 0xeef0
void sub_eef0(struct_19 *a0,int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if ((unsigned int)(a1 - 1U) <= 0xb)
    v1 = *(unsigned int *)((unsigned long)(unsigned int)(a1 - 1U) * 4 + 0x1d5c0);
  a0->field_0x18 = v1;
}

// Function: sub_ef10 @ 0xef10
long sub_ef10(long *a0)
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

// Function: sub_ef60 @ 0xef60
unsigned long sub_ef60(void *a0,long a1)
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

// Function: sub_efd0 @ 0xefd0
void sub_efd0(void *a0,unsigned char a1)
{
  if (*(short *)((long)a0 + 0x68) == 0xb) {
    *(unsigned long *)((long)a0 + 0xa0) = (unsigned long)a1 + 1;
    return;
  }
  abort(); // no-return
}

// Function: sub_eff0 @ 0xeff0
int sub_eff0(void *a0,void *a1,bool a2) // early-return x3
{
  stat *v1;
  unsigned long *v2;
  char *v3;
  unsigned int v4; // eax
  int v5;
  int *v6; // rax
  unsigned long v7; // rcx
  unsigned long *v8;
  
  v1 = (stat *)((long)a1 + 0x70);
  if ((*(unsigned int *)((long)a0 + 0x48) & 2) || (((*(unsigned int *)((long)a0 + 0x48) & 1 && (!*(long *)((long)a1 + 0x58))) || (a2)))) {
    v3 = *(char **)((long)a1 + 0x30);
    v5 = *(int *)((long)a0 + 0x2c);
    if (fstatat(v5,v3,v1,0) < 0) {
      v6 = __errno_location();
      v5 = *v6;
      if (v5 == 2) {
        v3 = *(char **)((long)a1 + 0x30);
        v5 = *(int *)((long)a0 + 0x2c);
        if (0 <= fstatat(v5,v3,v1,0x100)) {
          *v6 = 0;
          return 0xd;
        }
        v5 = *v6;
      }
      goto label_f08b;
    }
  }
  else {
    v3 = *(char **)((long)a1 + 0x30);
    v5 = *(int *)((long)a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_f08b:
      *(int *)((long)a1 + 0x40) = v5;
      v8 = (unsigned long *)((long)a1 + 0x78U & 0xfffffffffffffff8);
      *(unsigned long *)((long)a1 + 0x70) = 0;
      *(unsigned long *)((long)a1 + 0xf8) = 0;
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
  v4 = *(unsigned int *)((long)a1 + 0x88) & 0xf000;
  if (v4 != 0x4000) { // branch-flip
    if (v4 == 0xa000)
      return 0xc;
    v5 = (unsigned int)(v4 == 0x8000) * 5 + 3;
  }
  else {
    if (*(char *)((long)a1 + 0x100) != '.')
      return 1;
    if ((*(char *)((long)a1 + 0x101)) && ((*(unsigned int *)((long)a1 + 0x100) & 0xffff00) != 0x2e00))
      return 1;
    v5 = (-(unsigned int)(*(long *)((long)a1 + 0x58) == 0) & 0xfffffffc) + 5;
  }
  return v5;
}

// Function: sub_f1a0 @ 0xf1a0
void * sub_f1a0(void *a0,void *a1,unsigned long a2)
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
label_f1fb:
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
      goto label_f1fb;
    }
    v4 = *(long **)((long)a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned long *)((long)a0 + 0x38) = 0;
  return a1;
}

// Function: sub_f2b0 @ 0xf2b0
void * sub_f2b0(struct_20 *a0,void *a1,unsigned long a2)
{
  unsigned long v1;
  void *v2; // rax
  
  v2 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (v2) {
    memcpy((void *)((long)v2 + 0x100),a1,a2);
    v1 = a0->field_0x20;
    *(char *)((long)v2 + a2 + 0x100) = 0;
    *(unsigned long *)((long)v2 + 0x60) = a2;
    *(struct_20 **)((long)v2 + 0x50) = a0;
    *(unsigned long *)((long)v2 + 0x38) = v1;
    *(unsigned int *)((long)v2 + 0x40) = 0;
    *(unsigned long *)((long)v2 + 0x18) = 0;
    *(unsigned int *)((long)v2 + 0x6a) = 0x30000;
    *(unsigned long *)((long)v2 + 0x20) = 0;
    *(unsigned long *)((long)v2 + 0x28) = 0;
  }
  return v2;
}

// Function: sub_f350 @ 0xf350
unsigned long sub_f350(void *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)((long)a0 + 0x48) & 0x102) {
    v1 = sub_11cd0(0x1f,NULL,sub_ee90,sub_ee60,dat_24fd0);
    *(long *)((long)a0 + 0x58) = v1;
    return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)((long)a0 + 0x58) = v2;
  if (v2) {
    sub_17a20(v2);
    return 1;
  }
  return 0;
}

// Function: sub_f3c0 @ 0xf3c0
void sub_f3c0(void *a0)
{
  void *v1;
  
  if (a0) {
    do {
      v1 = *(void **)((long)a0 + 0x10);
      if (*(DIR **)((long)a0 + 0x18))
        closedir(*(DIR **)((long)a0 + 0x18));
      free(a0);
      a0 = v1;
    } while (v1);
    return;
  }
}

// Function: sub_f410 @ 0xf410
void sub_f410(void *a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_125f0(a0)) break;
    v1 = sub_12640(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_f450 @ 0xf450
unsigned long sub_f450(void *a0,int a1)
{
  long v1;
  struct_5 *v2;
  struct_2 *v3; // rax
  struct_2 *v4; // rax
  unsigned long v5 [2]; // stack - 0xb8
  statfs v6; // stack - 0xa8
  
  v1 = *(long *)((long)a0 + 0x50);
  v2 = *(struct_5 **)(v1 + 0x50);
  if (!(*(unsigned char *)(v1 + 0x49) & 2))
    return 0;
  if (!v2) {
    v2 = (struct_5 *)sub_11cd0(0xd,NULL,sub_eeb0,sub_eec0,dat_24fd0);
    *(struct_5 **)(v1 + 0x50) = v2;
    if (!v2) goto label_f4de;
  }
  v5[0] = *(unsigned long *)((long)a0 + 0x70);
  v1 = sub_11a70(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_f4de:
  if ((0 <= a1) && (!fstatfs(a1,&v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      v3->field_0x0 = *(unsigned long *)((long)a0 + 0x70);
      v3->field_0x8 = v6._0_8_;
      v4 = (struct_2 *)sub_122f0(v2,v3);
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

// Function: sub_f580 @ 0xf580
bool sub_f580(void *a0,int a1)
{
  long v1; // rax
  bool v2; // r8b
  
  v1 = sub_f450(a0,a1);
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

// Function: sub_f5d0 @ 0xf5d0
unsigned long sub_f5d0(void *a0,int a1)
{
  unsigned long v1; // rax
  
  v1 = sub_f450(a0,a1);
  if ((v1 != 0x1021994) && (v1 != 0xff534d42))
    return CONCAT71((undefined7)(v1 >> 8),v1 != 0x6969);
  return 0;
}

// Function: sub_f610 @ 0xf610
void sub_f610(struct_22 *a0,void *a1)
{
  char *v1;
  void *v2;
  long v3;
  char *v4; // rax
  unsigned long v5; // rax
  
  v1 = (char *)((long)a1 + 0x100);
  v2 = (void *)a0->field_0x20;
  *(long *)((long)a1 + 0x48) = *(long *)((long)a1 + 0x60);
  memmove(v2,v1,*(long *)((long)a1 + 0x60) + 1);
  v4 = strrchr(v1,0x2f);
  if ((v4) && ((v1 != v4 || (*(char *)((long)a1 + 0x101))))) {
    v5 = strlen(&v4[1]);
    memmove(v1,&v4[1],v5 + 1);
    *(unsigned long *)((long)a1 + 0x60) = v5;
  }
  v3 = a0->field_0x20;
  *(long *)((long)a1 + 0x38) = v3;
  *(long *)((long)a1 + 0x30) = v3;
}

// Function: sub_f6a0 @ 0xf6a0
unsigned long sub_f6a0(void *a0,struct_17 *a1) // return-dupe x2
{
  void *v1;
  struct_18 *v2; // rax
  struct_18 *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)((long)a0 + 0x48) & 0x102)) {
    v4 = sub_17a40(*(struct_16 **)((long)a0 + 0x58),&a1->field_0x70);
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
  v3 = (struct_18 *)sub_122f0(v1,v2);
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

// Function: sub_f750 @ 0xf750
void sub_f750(void *a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)((long)a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_12600((void *)((long)a0 + 0x60));
    if (0 <= v1) {
      close(v1);
      goto label_f794;
    }
  }
  else if ((!(*(unsigned char *)((long)a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_f794:
    *(int *)((long)a0 + 0x2c) = a1;
    return;
  }
  *(int *)((long)a0 + 0x2c) = a1;
}

// Function: sub_f7b0 @ 0xf7b0
bool sub_f7b0(void *a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)((long)a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)((long)a0 + 0x48) & 0x200) { // branch-flip
    sub_f750(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)((long)a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  sub_f410((void *)((long)a0 + 0x60));
  return v2;
}

// Function: sub_f820 @ 0xf820
void sub_f820(void *a0,void *a1)
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
    v4 = (void *)sub_12350(*(void **)((long)a0 + 0x58),&v5);
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

// Function: sub_f8e0 @ 0xf8e0
void sub_f8e0(void *a0,long a1,void *a2)
{
  void *v1;
  
  for (; a0; a0 = *(void **)((long)a0 + 0x10)) {
    if (*(long *)((long)a0 + 0x30) != (long)a0 + 0x100)
      *(long *)((long)a0 + 0x30) = (*(long *)((long)a0 + 0x30) - *(long *)((long)a0 + 0x38)) + a1;
    *(long *)((long)a0 + 0x38) = a1;
  }
  do {
    if (*(long *)((long)a2 + 0x58) <= -1)
      return;
    while( true ) {
      if (*(long *)((long)a2 + 0x30) != (long)a2 + 0x100)
        *(long *)((long)a2 + 0x30) = (*(long *)((long)a2 + 0x30) - *(long *)((long)a2 + 0x38)) + a1;
      v1 = *(void **)((long)a2 + 0x10);
      *(long *)((long)a2 + 0x38) = a1;
      if (!v1) break;
      a2 = v1;
      if (*(long *)((long)v1 + 0x58) <= -1)
        return;
    }
    a2 = *(void **)((long)a2 + 8);
  } while( true );
}

// Function: sub_f960 @ 0xf960
void sub_f960(unsigned long a0,unsigned int a1,char *a2)
{
  if (a1 & 0x200) {
    sub_12710(a0,a2); // tail-call
    return;
  }
  sub_e880(a2,(a1 & 0x10) << 0xd | 0x90900); // tail-call
}

// Function: sub_f9a0 @ 0xf9a0
int sub_f9a0(void *a0,void *a1,int a2,char *a3) // return-dupe
{
  int v1;
  int v2;
  int v3;
  int *v4; // rax
  stat v5; // stack - 0xd8
  unsigned int v6;
  unsigned char v7; // stack - 0xdd
  
  v1 = a2;
  if ((a3) && (!strcmp(a3,".."))) {
    v6 = *(unsigned int *)((long)a0 + 0x48);
    if (v6 & 4) {
label_fb30:
      if ((v6 & 0x200) && (0 <= a2)) {
        close(a2);
        return 0;
      }
      return 0;
    }
    if (a2 <= -1) {
      if (!(v6 & 0x200)) {
label_fa17:
        v7 = 1;
        goto label_fa1c;
      }
      v7 = sub_125f0((void *)((long)a0 + 0x60));
      if (v7) goto label_fa1c;
      v3 = sub_12640((void *)((long)a0 + 0x60));
      v6 = *(unsigned int *)((long)a0 + 0x48);
      if (v3 < 0) goto label_fa17;
      v7 = 1;
      a2 = v3;
      v1 = v3;
      if (v6 & 2) goto label_fa48;
      goto label_fa80;
    }
    v7 = 1;
    if (!(v6 & 2)) goto label_fb0d;
label_fa48:
    v3 = a2;
    if (!fstat(v1,&v5)) {
      if ((*(long *)((long)a1 + 0x70) == v5._0_8_) && (*(long *)((long)a1 + 0x78) == v5._8_8_)) {
        v6 = *(unsigned int *)((long)a0 + 0x48);
        goto label_fa80;
      }
      *__errno_location() = 2;
    }
    v2 = -1;
  }
  else {
    v6 = *(unsigned int *)((long)a0 + 0x48);
    if (v6 & 4) goto label_fb30;
    if (0 <= a2) // branch-flip
      v7 = 0;
    else {
      v7 = 0;
label_fa1c:
      v1 = sub_f960(*(unsigned int *)((long)a0 + 0x2c),v6,a3);
      if (v1 < 0)
        return -1;
      v6 = *(unsigned int *)((long)a0 + 0x48);
    }
    if (v6 & 2) goto label_fa48;
    v3 = a2;
    if (a3) {
label_fb0d:
      v3 = a2;
      if (!strcmp(a3,"..")) goto label_fa48;
    }
label_fa80:
    if (v6 & 0x200) {
      sub_f750(a0,v1,v7 ^ 1);
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

// Function: sub_fc50 @ 0xfc50
void * sub_fc50(struct_21 *a0,int a1) // return-dupe
{
  void *v1;
  int *v10; // rax
  dirent *v11; // rax
  unsigned long v12;
  void *v13; // rax
  long v14;
  unsigned int v15;
  int v16; // stack - 0x44
  unsigned int v17;
  DIR *v18;
  unsigned long v19;
  DIR *v2;
  unsigned long v20;
  bool v21;
  unsigned long v22; // stack - 0xa0
  void *v23; // stack - 0x98
  void *v24; // stack - 0x90
  char *v25; // stack - 0x70
  unsigned long v26; // stack - 0x68
  unsigned long v27; // stack - 0x60
  bool v28; // stack - 0x4b
  bool v29; // stack - 0x4a
  void *v3;
  long v4;
  bool v5;
  char v6; // al
  int v7; // eax
  int v8;
  char *v9;
  
  v1 = a0->field_0x0;
  v2 = *(DIR **)((long)v1 + 0x18);
  if (v2) { // branch-flip
    v16 = dirfd(v2);
    if (v16 < 0) {
      closedir(*(DIR **)((long)v1 + 0x18));
      *(unsigned long *)((long)v1 + 0x18) = 0;
      if (a1 == 3) {
        *(unsigned short *)((long)v1 + 0x68) = 4;
        *(int *)((long)v1 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (a0->field_0x40) { // branch-flip
      v26 = 0xffffffffffffffff;
label_fcb2:
      v21 = 1;
      v15 = a0->field_0x48;
    }
    else {
      v26 = 100000;
      v15 = a0->field_0x48;
      v21 = 1;
    }
  }
  else {
    v15 = a0->field_0x48;
    v8 = 0;
    if ((v15 & 0x10) && (v8 = 0x20000, v15 & 1))
      v8 = (unsigned int)(*(long *)((long)v1 + 0x58) != 0) << 0x11;
    v17 = 0xffffff9c;
    if ((v15 & 0x204) == 0x200)
      v17 = a0->field_0x2c;
    v14 = sub_12790(v17,*(char **)((long)v1 + 0x30),v8,&v16);
    *(long *)((long)v1 + 0x18) = v14;
    if (!v14) {
      if (a1 == 3) {
        *(unsigned short *)((long)v1 + 0x68) = 4;
        *(int *)((long)v1 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (*(short *)((long)v1 + 0x68) != 0xb) { // branch-flip
      if (a0->field_0x48 & 0x100) {
        sub_f820(a0,v1);
        sub_eff0(a0,v1,0);
        if (!sub_f6a0(a0,v1)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)((long)v1 + 0x68) = sub_eff0(a0,v1,0);
    }
    v26 = (unsigned long)(-(unsigned int)(a0->field_0x40 == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_10360:
      v21 = 0;
      v15 = a0->field_0x48;
      goto label_fcbb;
    }
    v8 = v16;
    if (((a0->field_0x48 & 0x38) != 0x18) || (*(long *)((long)v1 + 0x80) != 2)) {
label_10097:
      v21 = a1 == 3;
      v5 = 1;
    }
    else {
      v7 = sub_f580(v1,v16);
      v8 = v16;
      if (!v7) goto label_10097;
      if (a1 != 3) goto label_10360;
      v5 = 0;
      v21 = 1;
    }
    if (a0->field_0x48 & 0x200) {
      v8 = sub_17d10(v8,0x406,3);
      v16 = v8;
    }
    if ((0 <= v8) && (!sub_f9a0(a0,v1,v8,NULL))) goto label_fcb2;
    if ((v21) && (v5))
      *(int *)((long)v1 + 0x40) = *__errno_location();
    *(unsigned short *)((long)v1 + 0x6a) = *(unsigned short *)((long)v1 + 0x6a) | 1;
    closedir(*(DIR **)((long)v1 + 0x18));
    v15 = a0->field_0x48;
    *(unsigned long *)((long)v1 + 0x18) = 0;
    if ((v15 & 0x200) && (0 <= v16)) {
      close(v16);
      v15 = a0->field_0x48;
    }
    *(unsigned long *)((long)v1 + 0x18) = 0;
    v21 = 0;
  }
label_fcbb:
  v20 = *(unsigned long *)((long)v1 + 0x48);
  v22 = v20;
  v27 = v20 - 1;
  if (*(char *)(*(long *)((long)v1 + 0x38) + -1 + v20) != '/') {
    v22 = v20 + 1;
    v27 = v20;
  }
  v25 = NULL;
  if (v15 & 4) {
    v9 = (char *)(v27 + a0->field_0x20);
    v25 = &v9[1];
    *v9 = 0x2f;
  }
  v18 = *(DIR **)((long)v1 + 0x18);
  v14 = *(long *)((long)v1 + 0x58);
  v20 = a0->field_0x30 - v22;
  if (v18) { // branch-flip
    v10 = __errno_location();
    v19 = 0;
    v29 = 0;
    v28 = 0;
    v23 = NULL;
    v24 = NULL;
    do {
      while( true ) {
        *v10 = 0;
        v11 = readdir(v18);
        if (!v11) {
          if (*v10) {
            *(int *)((long)v1 + 0x40) = *v10;
            *(unsigned short *)((long)v1 + 0x68) = (-(unsigned short)(!v2 && !v19) & 0xfffd) + 7;
          }
          if (*(DIR **)((long)v1 + 0x18)) {
            closedir(*(DIR **)((long)v1 + 0x18));
            *(unsigned long *)((long)v1 + 0x18) = 0;
          }
          goto label_10214;
        }
        if (((a0->field_0x48 & 0x20) || (v11->field_0x13 != '.')) || ((v11->field_0x14 && (*(short *)&v11->field_0x14 != 0x2e)))) break;
label_fd83:
        v18 = *(DIR **)((long)v1 + 0x18);
        if (!v18) goto label_fed0;
      }
      v12 = strlen(&v11->field_0x13);
      v13 = (void *)sub_f2b0(a0,&v11->field_0x13,v12);
      if (!v13) {
label_10110:
        v8 = *v10;
        free(v13);
        sub_f3c0(v24);
        closedir(*(DIR **)((long)v1 + 0x18));
        *(unsigned long *)((long)v1 + 0x18) = 0;
        *(unsigned short *)((long)v1 + 0x68) = 7;
        a0->field_0x48 = a0->field_0x48 | 0x2000;
        *v10 = v8;
        return NULL;
      }
      if (v20 <= v12) {
        v4 = a0->field_0x20;
        v6 = sub_ef60(a0,v27 + 2 + v12);
        if (!v6) goto label_10110;
        if (a0->field_0x20 != v4) {
          v9 = (char *)(a0->field_0x20 + v22);
          if (!(a0->field_0x48 & 4))
            v9 = v25;
          v28 = v6;
          v25 = v9;
        }
        v20 = a0->field_0x30 - v22;
      }
      if (CARRY8(v12,v22)) {
        free(v13);
        sub_f3c0(v24);
        closedir(*(DIR **)((long)v1 + 0x18));
        *(unsigned long *)((long)v1 + 0x18) = 0;
        *(unsigned short *)((long)v1 + 0x68) = 7;
        a0->field_0x48 = a0->field_0x48 | 0x2000;
        *v10 = 0x24;
        return NULL;
      }
      *(long *)((long)v13 + 0x58) = v14 + 1;
      v3 = a0->field_0x0;
      *(unsigned long *)((long)v13 + 0x48) = v12 + v22;
      *(void **)((long)v13 + 8) = v3;
      *(unsigned long *)((long)v13 + 0x78) = *(unsigned long *)v11;
      v15 = a0->field_0x48;
      if (v15 & 4) { // branch-flip
        *(unsigned long *)((long)v13 + 0x30) = *(unsigned long *)((long)v13 + 0x38);
        memmove(v25,(void *)((long)v13 + 0x100),*(long *)((long)v13 + 0x60) + 1);
        v15 = a0->field_0x48;
      }
      else {
        *(void **)((long)v13 + 0x30) = (void *)((long)v13 + 0x100);
      }
      if ((a0->field_0x40) && (!(v15 & 0x400)))
        *(unsigned short *)((long)v13 + 0x68) = sub_eff0(a0,v13,0);
      else {
        v12 = 1;
        if ((v15 & 8) && (v11->field_0x12 & 0xfb))
          v12 = (unsigned long)((v15 >> 4 & 1 | (unsigned int)(v11->field_0x12 != 10)) ^ 1);
        *(unsigned short *)((long)v13 + 0x68) = 0xb;
        sub_eef0((struct_19 *)((long)v13 + 0x70));
        sub_efd0(v13,v12 & 0xffffffff);
      }
      *(unsigned long *)((long)v13 + 0x10) = 0;
      if (v24) // branch-flip
        *(void **)((long)v23 + 0x10) = v13;
      else {
        v24 = v13;
      }
      v12 = v19 + 1;
      if (v19 != 10000) {
        v19 = v12;
        if (v12 < v26) {
          v23 = v13;
          goto label_fd83;
        }
        goto label_10214;
      }
      if (!a0->field_0x40)
        v29 = sub_f5d0(v1,v16);
      v19 = 0x2711;
      v18 = *(DIR **)((long)v1 + 0x18);
      v23 = v13;
    } while (v18);
label_fed0:
label_10214:
    if (v28)
      sub_f8e0(a0->field_0x8,a0->field_0x20,v24);
    if (a0->field_0x48 & 4) {
      if ((a0->field_0x30 == v22) || (!v19)) goto label_10244;
label_1024a:
      *v25 = 0;
    }
    if ((v2) || (!v21)) {
label_102f0:
      if (!v19) goto label_102f5;
    }
    else if ((a1 == 1) || (!v19)) {
label_102d0:
      if (*(long *)((long)v1 + 0x58)) // branch-flip
        v8 = sub_f9a0(a0,*(void **)((long)v1 + 8),0xffffffff,"..");
      else {
        v8 = sub_f7b0(a0);
      }
      if (v8) {
        *(unsigned short *)((long)v1 + 0x68) = 7;
        a0->field_0x48 = a0->field_0x48 | 0x2000;
        sub_f3c0(v24);
        return NULL;
      }
      goto label_102f0;
    }
    if (v29) { // branch-flip
      a0->field_0x40 = sub_eed0;
      v24 = (void *)sub_f1a0(a0,v24,v19);
      a0->field_0x40 = 0;
    }
    else if ((a0->field_0x40) && (v19 != 1))
      v24 = (void *)sub_f1a0(a0,v24,v19);
  }
  else {
    if (a0->field_0x48 & 4) {
      v24 = NULL;
      v19 = 0;
      v29 = 0;
label_10244:
      v25 = &v25[-1];
      goto label_1024a;
    }
    v24 = NULL;
    if ((bool)(v21 & v2 == NULL)) {
      v29 = 0;
      v19 = 0;
      goto label_102d0;
    }
label_102f5:
    if (((a1 == 3) && (*(short *)((long)v1 + 0x68) != 4)) && (*(short *)((long)v1 + 0x68) != 7))
      *(unsigned short *)((long)v1 + 0x68) = 6;
    sub_f3c0(v24);
    v24 = NULL;
  }
  return v24;
}

// Function: sub_105a0 @ 0x105a0
void sub_105a0(unsigned int a0,struct_11 *a1)
{
  if (!(a0 & 0x102)) {
    free(a1); // tail-call
    return;
  }
  if (a1) {
    sub_11e90(a1); // tail-call
    return;
  }
}

// Function: sub_105d0 @ 0x105d0
long * sub_105d0(long *a0,unsigned int a1,long a2)
{
  void *v1;
  long v10; // r13
  char *v11;
  void *v12; // stack - 0x50
  unsigned char v13; // stack - 0x42
  void *v14; // stack - 0x40
  unsigned int v2;
  int v3; // eax
  long *v4; // rax
  unsigned long v5; // rax
  void *v6;
  unsigned long v7;
  long v8;
  void *v9;
  
  if (((a1 & 0xfffff000) || ((a1 & 0x204) == 0x204)) || (!(a1 & 0x12))) {
    *__errno_location() = 0x16;
    return NULL;
  }
  v4 = calloc(1,0x80);
  if (!v4)
    return NULL;
  v4[8] = a2;
  *(unsigned int *)((long)v4 + 0x2c) = 0xffffff9c;
  v2 = a1 & 0xfffffdff | 4;
  if (!(a1 & 2))
    v2 = a1;
  *(unsigned int *)&v4[9] = v2;
  v5 = sub_ef10(a0);
  v7 = 0x1000;
  if (0x1000 <= v5)
    v7 = v5;
  v13 = sub_ef60(v4,v7);
  if (!v13) {
    free(v4); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v12 = (void *)sub_f2b0(v4,"",0);
    if (v12) {
      *(unsigned long *)((long)v12 + 0x58) = 0xffffffffffffffff;
      v11 = (char *)*a0;
      if (a2) goto label_106b9;
label_106c8:
      if (!v11) goto label_1090d;
      v14 = NULL;
      v9 = NULL;
      v8 = 0;
label_1076f:
      do {
        v10 = v8;
        v1 = v14;
        v7 = strlen(v11);
        if (((3 <= v7) && (((unsigned char)(a1 >> 0xb) ^ 1) & 1)) && (v11[v7 - 1] == '/')) {
          do {
            if (v11[v7 - 2] != '/') break;
            v7 -= 1;
          } while (v7 != 1);
        }
        v6 = (void *)sub_f2b0(v4,v11,v7);
        if (!v6) goto label_107e0;
        *(unsigned long *)((long)v6 + 0x58) = 0;
        *(void **)((long)v6 + 8) = v12;
        *(long *)((long)v6 + 0x30) = (long)v6 + 0x100;
        if ((v9) && (v13)) {
          *(unsigned short *)((long)v6 + 0x68) = 0xb;
          sub_efd0(v6,1);
          if (a2) { // branch-flip
label_10757:
            *(void **)((long)v6 + 0x10) = v9;
          }
          else {
            *(unsigned long *)((long)v6 + 0x10) = 0;
label_108e8:
            v14 = v6;
            *(void **)((long)v1 + 0x10) = v6;
            v6 = v9;
          }
          v11 = (char *)a0[v10 + 1];
          v9 = v6;
          v8 = v10 + 1;
          if (!v11) break;
          goto label_1076f;
        }
        *(unsigned short *)((long)v6 + 0x68) = sub_eff0(v4,v6,0);
        if (a2) goto label_10757;
        *(unsigned long *)((long)v6 + 0x10) = 0;
        if (v9) goto label_108e8;
        v11 = (char *)a0[v10 + 1];
        v9 = v6;
        v8 = v10 + 1;
        v14 = v6;
      } while (v11);
      if ((a2) && (2 <= v10 + 1U))
        v6 = (void *)sub_f1a0(v4,v6,v10 + 1U);
      goto label_10874;
    }
  }
  else {
    v12 = NULL;
    if (a2) {
      v11 = NULL;
label_106b9:
      v13 = (unsigned char)(*(unsigned int *)&v4[9] >> 10) & 1;
      goto label_106c8;
    }
label_1090d:
    v6 = NULL;
label_10874:
    v8 = sub_f2b0(v4,"",0);
    *v4 = v8;
    v9 = v6;
    if (v8) {
      *(void **)(v8 + 0x10) = v6;
      *(unsigned short *)(v8 + 0x68) = 9;
      *(unsigned long *)(v8 + 0x58) = 1;
      if (sub_f350(v4)) {
        if (!(*(unsigned int *)&v4[9] & 0x204)) {
          v3 = sub_f960(*(unsigned int *)((long)v4 + 0x2c),*(unsigned int *)&v4[9],".");
          *(int *)&v4[5] = v3;
          if (v3 <= -1)
            *(unsigned int *)&v4[9] = *(unsigned int *)&v4[9] | 4;
        }
        sub_125d0((struct_23 *)&v4[0xc],0xffffffff);
        return v4;
      }
    }
label_107e0:
    sub_f3c0(v9);
    free(v12);
  }
  free((void *)v4[4]);
  free(v4);
  return NULL;
}

// Function: sub_10940 @ 0x10940
unsigned int sub_10940(struct_14 *a0) // return-dupe
{
  struct_14 *v1;
  int v2;
  void *v3;
  long v4;
  int *v5; // rax
  void *v6;
  struct_11 *v7;
  int v8;
  
  v6 = a0->field_0x0;
  if (v6) {
    if (*(long *)((long)v6 + 0x58) > -1) {
      do {
        v3 = *(void **)((long)v6 + 0x10);
        if (v3) { // branch-flip
          free(v6);
          v4 = *(long *)((long)v3 + 0x58);
          v6 = v3;
        }
        else {
          v3 = *(void **)((long)v6 + 8);
          free(v6);
          v4 = *(long *)((long)v3 + 0x58);
          v6 = v3;
        }
      } while (v4 >= 0);
    }
    free(v6);
  }
  if (a0->field_0x8)
    sub_f3c0(a0->field_0x8);
  v1 = &a0[1];
  free(a0->field_0x10);
  free(a0->field_0x20);
  if (a0->field_0x48 & 0x200) { // branch-flip
    v2 = a0->field_0x2c;
    if ((v2 <= -1) || (!close(v2))) goto label_109d7;
    v8 = *__errno_location();
label_10a8d:
    sub_f410(v1);
    v7 = a0->field_0x50;
    if (v7) goto label_109ec;
  }
  else {
    if (!(a0->field_0x48 & 4)) {
      v2 = a0->field_0x28;
      if (fchdir(v2)) {
        v5 = __errno_location();
        v2 = a0->field_0x28;
        v8 = *v5;
        if (!close(v2)) goto label_10a8d;
        if (!v8) goto label_10a48;
        sub_f410(v1);
        v7 = a0->field_0x50;
        if (!v7) {
          sub_105a0(a0->field_0x48,a0->field_0x58);
          free(a0);
          goto label_10a11;
        }
        goto label_109ec;
      }
      v2 = a0->field_0x28;
      if (close(v2)) {
        v5 = __errno_location();
label_10a48:
        v8 = *v5;
        goto label_10a8d;
      }
    }
label_109d7:
    v8 = 0;
    sub_f410(v1);
    v7 = a0->field_0x50;
    if (!v7) {
      sub_105a0(a0->field_0x48,a0->field_0x58);
      free(a0);
      return 0;
    }
label_109ec:
    sub_11e90(v7);
  }
  sub_105a0(a0->field_0x48,a0->field_0x58);
  free(a0);
  if (!v8)
    return 0;
  v5 = __errno_location();
label_10a11:
  *v5 = v8;
  return 0xffffffff;
}

// Function: sub_10b30 @ 0x10b30
void * sub_10b30(struct_22 *a0)
{
  short v1;
  long v10;
  unsigned int v2;
  char *v3;
  long v4;
  short v5;
  int v6;
  void *v7;
  void *v8;
  char *v9; // rdi
  
  v7 = a0->field_0x0;
  if (!v7)
    return NULL;
  v2 = a0->field_0x48;
  if (v2 & 0x2000)
    return NULL;
  v5 = *(short *)((long)v7 + 0x6c);
  *(unsigned short *)((long)v7 + 0x6c) = 3;
  if (v5 == 1) {
    *(unsigned short *)((long)v7 + 0x68) = sub_eff0(a0,v7,0);
    return v7;
  }
  v1 = *(short *)((long)v7 + 0x68);
  if (v5 != 2) { // branch-flip
    if (v1 != 1) { // branch-flip
label_10bc0:
      do {
        v8 = *(void **)((long)v7 + 0x10);
        if (!v8) {
          v8 = *(void **)((long)v7 + 8);
          if (*(long *)((long)v8 + 0x18)) {
            v4 = *(long *)((long)v8 + 0x48);
            a0->field_0x0 = v8;
            *(char *)(a0->field_0x20 + v4) = 0;
            v8 = (void *)sub_fc50(a0,3);
            if (v8) {
              free(v7);
              goto label_10e02;
            }
            if (a0->field_0x48 & 0x2000)
              return NULL;
            v8 = *(void **)((long)v7 + 8);
          }
          a0->field_0x0 = v8;
          free(v7);
          if (*(long *)((long)v8 + 0x58) == -1) {
            free(v8);
            *__errno_location() = 0;
            a0->field_0x0 = NULL;
            return NULL;
          }
          if (*(short *)((long)v8 + 0x68) != 0xb) {
            *(char *)(a0->field_0x20 + *(long *)((long)v8 + 0x48)) = 0;
            if (*(long *)((long)v8 + 0x58)) { // branch-flip
              if (*(unsigned short *)((long)v8 + 0x6a) & 2) {
                v6 = *(int *)((long)v8 + 0x44);
                if (!(a0->field_0x48 & 4)) {
                  if (a0->field_0x48 & 0x200) { // branch-flip
                    sub_f750(a0,v6,1);
                    v6 = *(int *)((long)v8 + 0x44);
                  }
                  else {
                    if (fchdir(v6)) {
                      *(int *)((long)v8 + 0x40) = *__errno_location();
                      a0->field_0x48 = a0->field_0x48 | 0x2000;
                    }
                    v6 = *(int *)((long)v8 + 0x44);
                  }
                }
                close(v6);
                goto label_10c2b;
              }
              if (*(unsigned short *)((long)v8 + 0x6a) & 1) goto label_10c2b;
              v6 = sub_f9a0(a0,*(void **)((long)v8 + 8),0xffffffff,"..");
            }
            else {
              v6 = sub_f7b0(a0);
            }
            if (v6) {
              *(int *)((long)v8 + 0x40) = *__errno_location();
              a0->field_0x48 = a0->field_0x48 | 0x2000;
            }
label_10c2b:
            if (*(short *)((long)v8 + 0x68) != 2) {
              if (*(int *)((long)v8 + 0x40)) // branch-flip
                *(unsigned short *)((long)v8 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v8 + 0x68) = 6;
                sub_f820(a0,v8);
              }
            }
            if (!(a0->field_0x48 & 0x2000))
              return v8;
            return NULL;
          }
          abort(); // no-return, return-dupe
        }
        a0->field_0x0 = v8;
        free(v7);
        v7 = v8;
        if (!*(long *)((long)v8 + 0x58)) {
          if (sub_f7b0(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          sub_105a0(a0->field_0x48,a0->field_0x58);
          sub_f610(a0,v8);
          sub_f350(a0);
          v5 = *(short *)((long)v8 + 0x68);
          goto label_10da5;
        }
      } while (*(short *)((long)v8 + 0x6c) == 4);
      if (*(short *)((long)v8 + 0x6c) == 2) {
        v5 = sub_eff0(a0,v8,1);
        *(short *)((long)v8 + 0x68) = v5;
        if ((v5 == 1) && (!(a0->field_0x48 & 4))) {
          v6 = sub_f960(a0->field_0x2c,a0->field_0x48,".");
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
      if (v5 == 4) goto label_10d2a;
label_10c8f:
      if ((v2 & 0x40) && (*(long *)((long)v7 + 0x70) != a0->field_0x18)) {
label_10d2a:
        if (*(unsigned char *)((long)v7 + 0x6a) & 2)
          close(*(int *)((long)v7 + 0x44));
        if (a0->field_0x8) {
          sub_f3c0(a0->field_0x8);
          a0->field_0x8 = NULL;
        }
        *(unsigned short *)((long)v7 + 0x68) = 6;
        sub_f820(a0,v7);
        return v7;
      }
      if (a0->field_0x8) { // branch-flip
        if (v2 & 0x1000) {
          a0->field_0x48 = v2 & 0xffffefff;
          sub_f3c0(a0->field_0x8);
          a0->field_0x8 = NULL;
          goto label_10ea0;
        }
        v3 = *(char **)((long)v7 + 0x30);
        if (sub_f9a0(a0,v7,0xffffffff,v3)) { // branch-flip
          v6 = *__errno_location();
          *(unsigned short *)((long)v7 + 0x6a) = *(unsigned short *)((long)v7 + 0x6a) | 1;
          *(int *)((long)v7 + 0x40) = v6;
          v8 = a0->field_0x8;
          if (v8) {
            v7 = v8;
            do {
              *(unsigned long *)((long)v7 + 0x30) = *(unsigned long *)(*(long *)((long)v7 + 8) + 0x30);
              v7 = *(void **)((long)v7 + 0x10);
            } while (v7);
          }
        }
        else {
          v8 = a0->field_0x8;
        }
      }
      else {
label_10ea0:
        v8 = (void *)sub_fc50(a0,3);
        a0->field_0x8 = v8;
        if (!v8) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)((long)v7 + 0x40)) && (*(short *)((long)v7 + 0x68) != 4))
              *(unsigned short *)((long)v7 + 0x68) = 7;
            sub_f820(a0,v7);
            return v7;
          }
          return NULL;
        }
      }
      a0->field_0x8 = NULL;
    }
label_10e02:
    v4 = *(long *)(*(long *)((long)v8 + 8) + 0x48);
    v10 = v4 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v8 + 8) + 0x38) + -1 + v4) != '/')
      v10 = v4;
    v9 = (char *)(v10 + a0->field_0x20);
    *v9 = 0x2f;
    memmove(&v9[1],(void *)((long)v8 + 0x100),*(long *)((long)v8 + 0x60) + 1);
    v5 = *(short *)((long)v8 + 0x68);
    a0->field_0x0 = v8;
    v7 = v8;
label_10dad:
    if (v5 == 0xb) goto label_10e51;
  }
  else {
    if (2 <= (unsigned short)(v1 - 0xcU)) {
      if (v1 == 1) goto label_10c8f;
      goto label_10bc0;
    }
    v5 = sub_eff0(a0,v7,1);
    *(short *)((long)v7 + 0x68) = v5;
    if (v5 == 1) {
      if (a0->field_0x48 & 4) {
        a0->field_0x0 = v7;
        goto label_10dbd;
      }
      v6 = sub_f960(a0->field_0x2c,a0->field_0x48,".");
      *(int *)((long)v7 + 0x44) = v6;
      if (v6 < 0) {
        v6 = *__errno_location();
        *(unsigned short *)((long)v7 + 0x68) = 7;
        *(int *)((long)v7 + 0x40) = v6;
        a0->field_0x0 = v7;
        return v7;
      }
      *(unsigned short *)((long)v7 + 0x6a) = *(unsigned short *)((long)v7 + 0x6a) | 2;
      v5 = *(short *)((long)v7 + 0x68);
label_10da5:
      a0->field_0x0 = v7;
      goto label_10dad;
    }
    a0->field_0x0 = v7;
    if (v5 != 0xb)
      return v7;
label_10e51:
    if (*(long *)((long)v7 + 0xa0) != 2) {
      if (*(long *)((long)v7 + 0xa0) == 1)
        return v7;
      abort();
    }
    v5 = sub_eff0(a0,v7,0);
    *(short *)((long)v7 + 0x68) = v5;
  }
  if (v5 != 1)
    return v7;
label_10dbd:
  if (!*(long *)((long)v7 + 0x58))
    a0->field_0x18 = *(long *)((long)v7 + 0x70);
  if (!sub_f6a0(a0,v7)) {
    v7 = NULL;
    *__errno_location() = 0xc;
  }
  return v7;
}

// Function: sub_11170 @ 0x11170
unsigned long sub_11170(unsigned long a0,void *a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)((long)a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_111a0 @ 0x111a0
int * sub_111a0(struct_40 *a0,unsigned int a1) // early-return x2
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
  v1 = a0->field_0x0;
  *v5 = 0;
  if (a0->field_0x48 & 0x2000) {
    v5 = NULL;
    return v5;
  }
  if (*(short *)(v1 + 0x68) == 9) {
    v5 = *(void **)(v1 + 0x10);
    return v5;
  }
  if (*(short *)(v1 + 0x68) != 1) {
    v5 = NULL;
    return v5;
  }
  if (a0->field_0x8)
    sub_f3c0(a0->field_0x8);
  v6 = 1;
  if (a1 == 0x1000) {
    a0->field_0x48 = a0->field_0x48 | 0x1000;
    v6 = 2;
  }
  if (((*(long *)(v1 + 0x58)) || (**(char **)(v1 + 0x30) == '/')) || (a0->field_0x48 & 4)) {
    v5 = (void *)sub_fc50(a0,v6);
    a0->field_0x8 = v5;
  }
  else {
    v4 = sub_f960(a0->field_0x2c,a0->field_0x48,".");
    v2 = v4;
    if (v4 < 0) {
      a0->field_0x8 = NULL;
      v5 = NULL;
      return v5;
    }
    v5 = (void *)sub_fc50(a0,v6);
    a0->field_0x8 = v5;
    if (a0->field_0x48 & 0x200) // branch-flip
      sub_f750(a0,v2,1);
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
    v5 = a0->field_0x8;
  }
  return v5;
}

// Function: sub_11320 @ 0x11320
long sub_11320(unsigned int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  long v2; // r13
  
  if (a2) { // branch-flip
    v2 = 0;
    do {
      v1 = sub_150c0(a0,a1,a2);
      if (v1 == -1)
        return v2;
      if (!v1) {
        *__errno_location() = 0x1c;
        return v2;
      }
      v2 += v1;
      a1 = (void *)((long)a1 + v1);
      a2 -= v1;
    } while (a2);
  }
  else {
    v2 = 0;
  }
  return v2;
}

// Function: sub_113a0 @ 0x113a0
unsigned long sub_113a0(unsigned long a0)
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

// Function: sub_11410 @ 0x11410
unsigned long sub_11410(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_113a0(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_11450 @ 0x11450
unsigned long sub_11450(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_17720(a0,3) % a1;
}

// Function: sub_11470 @ 0x11470
bool sub_11470(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_11480 @ 0x11480
unsigned long sub_11480(struct_1 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x1d660)
    return 1;
  v1 = v2[2];
  if ((((dat_1d674 < v1) && (v1 < dat_1d678)) && (dat_1d67c < v2[3])) && (dat_1d690 <= *v2)) {
    v3 = *v2 + dat_1d674;
    if (((v3 < v2[1]) && (v2[1] <= dat_1d680)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x1d660;
  return 0;
}

// Function: sub_11510 @ 0x11510
void sub_11510(struct_3 *a0,struct_2 *a1)
{
  unsigned long v1;
  
  v1 = a0->field_0x48;
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  a0->field_0x48 = a1;
}

// Function: sub_11530 @ 0x11530
long sub_11530(struct_4 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}

// Function: sub_11560 @ 0x11560
long sub_11560(struct_5 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_11530(a0,a1);
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
      sub_11510(a0);
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
      sub_11510(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_11660 @ 0x11660
void sub_11660(struct_3 *a0)
{
  if (a0->field_0x48) {
    a0->field_0x48 = *(unsigned long *)(a0->field_0x48 + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_11690 @ 0x11690
unsigned long sub_11690(struct_8 *a0,struct_42 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_7 *v3; // rax
  struct_7 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_116b8:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_7 *)sub_11530(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_11510(a0);
          v6 = v1;
          if (!v1) goto label_11728;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_11728:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_116b8;
    v3 = (struct_7 *)sub_11530(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_7 *)sub_11660(a0);
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

// Function: sub_117c0 @ 0x117c0
unsigned long sub_117c0(float a0,unsigned long a1,bool a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_1d684 <= a0)
      return 0;
    a1 = (a0 < dat_1d688) ? (unsigned long)a0 : (long)(a0 - dat_1d688) ^ 0x8000000000000000;
  }
  v1 = sub_11410(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_11850 @ 0x11850
unsigned long sub_11850(struct_18 *a0)
{
  return a0->field_0x10;
}

// Function: sub_11860 @ 0x11860
unsigned long sub_11860(struct_41 *a0)
{
  return a0->field_0x18;
}

// Function: sub_11870 @ 0x11870
unsigned long sub_11870(struct_20 *a0)
{
  return a0->field_0x20;
}

// Function: sub_11880 @ 0x11880
unsigned long sub_11880(struct_42 *a0)
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

// Function: sub_118e0 @ 0x118e0
bool sub_118e0(struct_42 *a0)
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
label_11900:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_11900;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_11950 @ 0x11950
void sub_11950(struct_20 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_11870(a0);
  v2 = sub_11850();
  v3 = sub_11860();
  v4 = sub_11880();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_1d690) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_11a70 @ 0x11a70
long sub_11a70(struct_5 *a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_11530(a0,a1);
  v3 = *v2;
  if (!v3)
    return 0;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = a0->field_0x38;
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}

// Function: sub_11ad0 @ 0x11ad0
long sub_11ad0(struct_42 *a0)
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

// Function: sub_11b20 @ 0x11b20
long sub_11b20(void *a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_11530(a0,a1);
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

// Function: sub_11b90 @ 0x11b90
void sub_11b90(struct_42 *a0,long a1,unsigned long a2)
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

// Function: sub_11be0 @ 0x11be0
long sub_11be0(struct_42 *a0,void *a1,unsigned long a2)
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

// Function: sub_11c60 @ 0x11c60
unsigned long sub_11c60(unsigned char *a0,unsigned long a1)
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

// Function: sub_11ca0 @ 0x11ca0
void sub_11ca0(struct_43 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_11cd0 @ 0x11cd0
unsigned long * sub_11cd0(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_11450;
  if (!a3)
    a3 = sub_11470;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x1d660;
  v1[5] = a1;
  if (sub_11480(v1)) {
    v2 = sub_117c0(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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

// Function: sub_11de0 @ 0x11de0
void sub_11de0(struct_44 *a0)
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
        if (a0->field_0x8 <= v5) goto label_11e7b;
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
label_11e7b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_11e90 @ 0x11e90
void sub_11e90(struct_11 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_11f3c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11eff;
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
label_11eff:
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
label_11f3c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_11f80 @ 0x11f80
unsigned int sub_11f80(struct_9 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_117c0(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_11690(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_11690(a0,&v2,1)) && (sub_11690(a0,&v2,0))) {
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

// Function: sub_12100 @ 0x12100
unsigned long sub_12100(void *a0,long a1,long *a2) // ternary
{
  long v1;
  struct_7 *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_11560(a0,a1,&v3,0);
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
      sub_11480(a0);
      v1 = *(long *)((long)a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_1d684 <= v5)
          return 0xffffffff;
        v4 = (dat_1d688 <= v5) ? (long)(v5 - dat_1d688) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_11f80(a0,v4))
          return 0xffffffff;
        if (sub_11560(a0,a1,&v3,0)) {
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
    v2 = (struct_7 *)sub_11660(a0);
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

// Function: sub_122f0 @ 0x122f0
unsigned long sub_122f0(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_12100(a0,a1,&v3);
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

// Function: sub_12350 @ 0x12350
long sub_12350(void *a0,long a1) // ternary x2
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
  
  v5 = sub_11560(a0,a1,&v7,1);
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
      sub_11480(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1d688 <= v8) ? (long)(v8 - dat_1d688) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11f80(a0,v6)) {
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

// Function: sub_124e0 @ 0x124e0
long sub_124e0(void *a0)
{
  return sub_12350(a0); // tail-call
}

// Function: sub_124f0 @ 0x124f0
unsigned long sub_124f0(void *a0,unsigned long a1)
{
  return *(unsigned long *)((long)a0 + 8) % a1;
}

// Function: sub_12510 @ 0x12510
unsigned long sub_12510(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    return sub_152d0(v2,v1); // tail-call
  }
  return 0;
}

// Function: sub_12540 @ 0x12540
unsigned long sub_12540(struct_10 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_17e60(a0->field_0x0,a1);
  return (v1 ^ a0->field_0x8) % a1;
}

// Function: sub_12570 @ 0x12570
unsigned long sub_12570(unsigned long *a0,unsigned long *a1)
{
  int v1; // eax
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = strcmp((char *)*a0,(char *)*a1);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
  }
  return 0;
}

// Function: sub_125b0 @ 0x125b0
void sub_125b0(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_125d0 @ 0x125d0
void sub_125d0(struct_23 *a0,unsigned int a1)
{
  *(unsigned long *)a0->field_0x14 = 0;
  a0->field_0x1c = 1;
  a0->field_0x0 = a1;
  a0->field_0x4 = a1;
  a0->field_0x8 = a1;
  a0->field_0xc = a1;
  a0->field_0x10 = a1;
}

// Function: sub_125f0 @ 0x125f0
char sub_125f0(void *a0)
{
  return *(char *)((long)a0 + 0x1c);
}

// Function: sub_12600 @ 0x12600
unsigned int sub_12600(void *a0,unsigned int a1)
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

// Function: sub_12640 @ 0x12640
unsigned int sub_12640(void *a0)
{
  unsigned int v1;
  unsigned int v2;
  char v3; // al
  
  v3 = sub_125f0(a0);
  if (v3)
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

// Function: sub_126a0 @ 0x126a0
void sub_126a0(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_126b0 @ 0x126b0
void sub_126b0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_126d0 @ 0x126d0
void sub_126d0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_126e0 @ 0x126e0
void sub_126e0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_12710 @ 0x12710
void sub_12710(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_15df0(openat(a0,a1,a2,v1));
}

// Function: sub_12790 @ 0x12790
DIR * sub_12790(unsigned long a0,char *a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_12710(a0,a1,a2 | 0x90900);
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

// Function: sub_127f0 @ 0x127f0
void sub_127f0(char *a0)
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
    dat_255a0 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_12890 @ 0x12890
unsigned int sub_12890(unsigned long a0,unsigned long a1,char *a2,unsigned int a3,unsigned int a4)
{
  unsigned int v1; // eax
  char v2 [4];
  
  if (sub_17670(a0,a1,a4,v2)) // branch-flip
    v1 = 0xfffffffe;
  else {
    v1 = sub_176a0(v2,a2,a3);
    sub_17660(v2);
  }
  return v1;
}

// Function: sub_12910 @ 0x12910
unsigned int sub_12910(char *a0,unsigned int a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0x24
  
  v2 = a2;
  v1 = sub_176a0(&v2,a0,a1);
  sub_17660(&v2);
  return v1;
}

// Function: sub_12970 @ 0x12970
struct_12 * sub_12970(struct_12 *a0,int a1)
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

// Function: sub_129c0 @ 0x129c0
bool sub_129c0(long a0,long a1)
{
  return sub_17940((char *)(a0 + 9),(char *)(a1 + 9)) == 0;
}

// Function: sub_129e0 @ 0x129e0
unsigned long sub_129e0(long a0,long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_178c0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_129c0(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_12a40 @ 0x12a40
unsigned long sub_12a40(long a0,long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_178c0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_129e0(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_12ac0 @ 0x12ac0
unsigned long sub_12ac0(long a0,long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_178c0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_12a40(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_12b30 @ 0x12b30
unsigned long sub_12b30(long a0,long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_178c0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_12ac0(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_12bd0 @ 0x12bd0
unsigned long sub_12bd0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_178c0((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_12b30(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_12c70 @ 0x12c70
unsigned long sub_12c70(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_178c0((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_12bd0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_12d30 @ 0x12d30
unsigned long sub_12d30(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_178c0((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_12c70(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_12df0 @ 0x12df0
unsigned long sub_12df0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_178c0((int)(char)a2)) // branch-flip
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
      return sub_12d30(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_12ed0 @ 0x12ed0
unsigned long sub_12ed0(unsigned char *a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_178c0((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_12df0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_12f90 @ 0x12f90
char * sub_12f90(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned char *v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = (unsigned char *)sub_17ea0();
    if (sub_12ed0(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_12ed0(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x1d6e0 : (char *)0x1d6e7; // branch-flip
    }
  }
  return v1;
}

// Function: sub_130c0 @ 0x130c0
long * sub_130c0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_13130:
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
label_135b6:
      v2 = 0;
      goto label_13570;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_136d1;
      }
      v23 = 1;
      v2 = 0;
      v12 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v2 = 1;
label_13570:
      v23 = 1;
      v12 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_136d1;
      }
      goto label_135b6;
    case 5:
      if (!v17) goto label_13511;
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
        v9 = (char *)sub_12f90("`",a4);
        v35 = (char *)v9;
        v9 = (char *)sub_12f90("\'",a4);
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
label_131f3:
  do {
label_13206:
    v22 = 0;
    a1 = v13;
label_13210:
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
          if ((a3 < v22 + v29) || (v8 = memcmp(v11,v30,v29), v8)) goto label_13b80;
          if (!v23) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v20 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_1377c;
                default:
                  goto label_13453;
                case 7:
                  goto label_13740;
                case 8:
                  goto label_13725;
                case 9:
                  goto label_13803;
                case 10:
                  goto label_1371b;
                case 0xb:
                  goto label_13830;
                case 0xc:
                  goto label_13767;
                case 0xd:
                  goto label_1366d;
                case 0x20:
                  goto label_1383d;
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
                  goto label_13406;
                case 0x23:
                  goto label_137e0;
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
                  goto label_13338;
                case 0x27:
                  goto label_13652;
                case 0x3f:
                  goto label_13611;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_13453;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_13338;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_13c24;
              goto label_13b68;
            }
            if (v7 == 0x7d) goto label_13cf0;
            if ((char)v7 <= '}') {
              v20 = 0x7b;
              if (v7 == 0x7b) goto label_13b4a;
              v20 = 0x7c;
              if (v7 == 0x7c) goto label_13b68;
              goto label_13453;
            }
            if (v7 != 0x7e) goto label_13453;
label_13ca4:
            if (!v22) {
              v7 = a4 == 2;
              v20 = 0x7e;
              v25 = v3;
              goto label_134c9;
            }
            v20 = 0x7e;
            goto label_13cb2;
          }
          goto label_1343c;
        }
label_13b80:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v20 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v21 = 0;
                if (v7 == 0x7e) goto label_13ca4;
                goto label_13450;
              }
              v20 = 0x7b;
              v24 = 0;
              if (v7 != 0x7b) {
                v20 = 0x7c;
                v24 = 0;
                v21 = 0;
                if (v7 == 0x7c) goto label_13b68;
                goto label_13453;
              }
            }
            else {
              v20 = 0x7d;
              v24 = 0;
            }
label_13b4a:
            if (a3 == 0xffffffffffffffff) goto label_13cff;
label_13b58:
            v21 = v24;
            if (a3 == 1) goto label_137e5;
          }
          else {
            if (v7 == 0x40) goto label_13450;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v21 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_13338;
            v24 = 0;
            v21 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_13c24:
              if ((char)v18 != '\\') goto label_13453;
              if (((bool)(v2 & v23)) && (v29)) goto label_13c4a;
              v16 = 0x5c;
              goto label_13672;
            }
          }
label_13b68:
          v7 = a4 == 2;
          v25 = v3;
          v24 = 0;
          goto label_134c9;
        }
        switch(v7) {
          case 0:
            goto label_1376e;
          default:
label_13450:
            v24 = 0;
label_13453:
            v16 = (unsigned char)v18;
            v21 = v24;
            if (v32 != 1) goto label_139d3;
label_13460:
            v9 = __ctype_b_loc();
            v20 = 1;
            v25 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v26 = (bool)(v25 & v2);
            v25 = !v25;
            v21 = v24;
            goto label_134ae;
          case 7:
label_13740:
            v16 = 0x61;
            if (!v23) goto label_13738;
            goto label_1369f;
          case 8:
label_13725:
            v16 = 0x62;
            goto label_1372a;
          case 9:
            v24 = 0;
label_13803:
            v20 = 9;
            v16 = 0x74;
            goto label_13810;
          case 10:
label_1371b:
            v16 = 0x6e;
            break;
          case 0xb:
label_13830:
            v16 = 0x76;
            break;
          case 0xc:
label_13767:
            v16 = 0x66;
label_1372a:
            if (v23) goto label_1369f;
label_13738:
            v24 = 0;
            goto label_133be;
          case 0xd:
label_1366d:
            v16 = 0x72;
            break;
          case 0x20:
            v21 = 0;
label_1383d:
            v20 = 0x20;
            goto label_13338;
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
            goto label_13409;
          case 0x23:
            v20 = 0x23;
            v24 = 0;
            goto label_137e5;
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
label_13338:
            v7 = 0;
            goto label_13348;
          case 0x27:
            v21 = 0;
            goto label_13652;
          case 0x3f:
            v21 = 0;
            goto label_13611;
          
        }
        goto label_13672;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_13cf0:
            v20 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_13b58;
label_13cff:
            v21 = v24;
            if (a2[1]) goto label_13b68;
            goto label_137e5;
          }
          if ((char)v16 <= '}') {
            v20 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_13406;
              goto label_13453;
            }
            goto label_13b4a;
          }
          v20 = 0x7e;
          if (v16 == 0x7e) goto label_137e5;
          if (v32 == 1) goto label_13460;
label_139d3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v20 = 0;
          do {
            v18 = v22 + v20;
            v9 = (long)sub_17ee0(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v24 = v25;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_13f3b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_142b5;
              goto label_142bf;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_13b0d;
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
          goto label_13f3b;
        }
        if (v16 == 0x40) goto label_13453;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_134c0;
        if ((unsigned long)v9 & 0xa4000000) goto label_13406;
        if (v16 != 0x5c) goto label_13453;
        if (a4 == 2) {
          if (!v23) goto label_13c4a;
          goto label_13433;
        }
        if (!(bool)(v2 & v23 & v29 != 0)) {
          v20 = 0x5c;
          v16 = 0x5c;
          v24 = 0;
          goto label_13810;
        }
label_13c4a:
        v22 += 1;
        v24 = 0;
        v16 = 0x5c;
        v7 = v19;
        goto label_13c60;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v21 = 0;
            v20 = 0;
            if (v33 & 1) goto label_138e2;
            goto label_1381b;
          }
label_1376e:
          if (!v23) {
            v21 = 0;
label_1377c:
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
                goto label_134f3;
              }
label_14085:
              a0[v13] = 0x5c;
              v12 = v13;
              v19 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v19;
              goto label_14085;
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
label_140bc:
              v16 = (unsigned char)v20;
              v24 = 0;
              v12 = v13;
              v7 = v25;
              if (!v2 || v26) goto label_134f3;
              goto label_13348;
            }
            v16 = 0x30;
            v24 = 0;
            v12 = v13;
            v7 = v25;
            goto label_134f3;
          }
          if (a4 != 2) goto label_1343c;
          goto label_136ab;
        default:
          goto label_13453;
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
          goto label_1389f;
        case 10:
          v20 = 10;
          v16 = 0x6e;
          goto label_1389f;
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
label_1389f:
          v7 = a4 == 2 & v23;
          v24 = 0;
          if (!(bool)v7) goto label_13810;
label_13421:
          a4 = 2;
label_13433:
          if (!v27) goto label_1343c;
          goto label_136ab;
        case 0x20:
          v18 = 0x20;
          goto label_13409;
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
label_13406:
          v25 = 0;
          goto label_13409;
        case 0x23:
label_137e0:
          v20 = 0x23;
label_137e5:
          v21 = v24;
          if (v22) {
label_13cb2:
            v7 = a4 == 2;
            v25 = v3;
            v24 = 0;
            goto label_134c9;
          }
          v18 = v20;
label_13409:
          v20 = v18;
          v21 = v24;
          v24 = v25;
          if ((a4 == 2) && (v23)) goto label_13421;
          goto label_134c7;
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
          goto label_134c0;
        case 0x27:
label_13652:
          v24 = v25;
          if (a4 != 2) {
            v7 = 0;
            v20 = 0x27;
            goto label_134c9;
          }
          if (!v23) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v31) goto label_140db;
            }
            else {
label_140db:
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
            goto label_134f3;
          }
          goto label_13433;
        case 0x3f:
label_13611:
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
                  goto label_134c9;
                }
                if (v23) goto label_1343c;
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
                goto label_140bc;
              }
            }
            v7 = 0;
            v20 = 0x3f;
            v25 = v3;
            v24 = 0;
            goto label_134c9;
          }
          if (v23) goto label_13433;
          v7 = 0;
          v16 = 0x3f;
          v24 = 0;
          goto label_134f3;
        
      }
label_13810:
      v21 = v24;
      if (!v2) {
label_1381b:
        v16 = (unsigned char)v20;
        v24 = 0;
        v7 = 0;
        v25 = v3;
        if (!v23) goto label_134f0;
        goto label_13348;
      }
label_13672:
      v24 = 0;
      goto label_13683;
    }
    if (!(bool)(v12 == 0 & v23 & a4 == 2)) {
      v7 = a4 == 2 & (v23 ^ 1U);
      v23 = (bool)(v23 ^ 1U);
      if ((!(bool)v7) || (v23 = (bool)v7, !v3)) {
label_1433f:
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
        if (a1 || !v31) goto label_1433f;
        v22 = v31;
        goto label_13e0b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 1;
      v32 = (unsigned long)v9;
      v30 = "\"";
      if (!(v33 & 2)) goto label_143ef;
      v3 = 0;
      v19 = 0;
      v31 = 0;
      v2 = v4;
      v23 = v4;
      goto label_13206;
    }
label_13b0d:
    if (v2) {
label_136ab:
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
label_136d1:
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
label_13e0b:
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
label_142b5:
    if (!a2[(long)v9]) break;
  }
label_142bf:
  v24 = 0;
label_13f3b:
  v18 = (unsigned long)v16;
  v25 = v24;
  if (2 <= v20) {
label_13f45:
    v25 = 0;
    v20 += v22;
    v13 = v22;
    do {
      v16 = (unsigned char)v18;
      if (v26) {
        v27 = a4 == 2;
        if (v23) goto label_13433;
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
        if (v20 <= v22) goto label_133d1;
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
          goto label_13c60;
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
label_134ae:
  if (v26) {
    v24 = 0;
    v26 = v2;
    goto label_13f45;
  }
label_134c0:
  v20 = v18;
  v24 = v25;
label_134c7:
  v7 = a4 == 2;
  v25 = v3;
label_134c9:
  v16 = (unsigned char)v20;
  v26 = (bool)v7 == 0;
  v3 = v25;
  v7 = 0;
  if ((v26 && v2) || (v7 = 0, v23)) {
label_13348:
    v16 = (unsigned char)v20;
    v12 = v13;
    if (!v28) goto label_134f3;
    if (!(*(unsigned int *)(v28 + (v20 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_134f3;
  }
  else {
label_134f0:
    v7 = 0;
    v3 = v25;
label_134f3:
    if (!v21) {
      v7 ^= 1;
      v22 += 1;
      v7 &= v19;
label_13c60:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v19 = 0;
        v12 += 2;
      }
      goto label_133d1;
    }
  }
label_13683:
  if (v23) {
    v23 = v2;
label_1369f:
    if ((bool)(v23 & a4 == 2)) goto label_136ab;
label_1343c:
    v33 &= 0xfffffffd;
    v28 = 0;
    goto label_13130;
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
label_133be:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v22 += 1;
label_133d1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v24)
    v4 = 0;
  goto label_13210;
label_138e2:
  v22 += 1;
  goto label_13210;
label_143ef:
  a1 = v31;
label_13511:
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
  goto label_131f3;
}

// Function: sub_144f0 @ 0x144f0
char * sub_144f0(unsigned int a0,char *a1,unsigned long a2,struct_13 *a3)
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
    v5 = dat_25080;
    if (dat_25078 <= (int)a0) {
      v10 = (long)dat_25078;
      v8 = (long)(int)((a0 - dat_25078) + 1);
      if (dat_25080 != (unsigned long *)0x25090) // branch-flip
        v5 = (unsigned long *)sub_17050(dat_25080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_17050(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_25090;
        v5[1] = dat_25098;
      }
      dat_25080 = v5;
      memset(&v5[(long)dat_25078 * 2],0,(v10 - dat_25078) * 0x10);
      dat_25078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_130c0(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x255c0)
        free(v7);
      v7 = (char *)sub_16ed0(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_130c0(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_146e0 @ 0x146e0
void sub_146e0(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x256c0;
  sub_17190(a0,0x38);
  *v2 = v1;
}

// Function: sub_14720 @ 0x14720
unsigned int sub_14720(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x256c0;
  return *a0;
}

// Function: sub_14740 @ 0x14740
void sub_14740(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x256c0;
  *a0 = a1;
}

// Function: sub_14760 @ 0x14760
unsigned int sub_14760(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x256c0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_147a0 @ 0x147a0
unsigned int sub_147a0(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0x256c0;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_147c0 @ 0x147c0
void sub_147c0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x256c0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_14800 @ 0x14800
void sub_14800(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x256c0;
  v2 = __errno_location();
  v1 = *v2;
  sub_130c0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_14880 @ 0x14880
char * sub_14880(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x256c0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_130c0(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_16ed0(v3 + 1U);
  sub_130c0(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_14970 @ 0x14970
char * sub_14970(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_14880(a0,a1,NULL,a2); // tail-call
}

// Function: sub_14980 @ 0x14980
void sub_14980(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_25080;
  if (2 <= dat_25078) {
    v4 = (unsigned long *)((long)dat_25080 + 0x18);
    v1 = (unsigned long *)((long)dat_25080 + (unsigned long)(unsigned int)(dat_25078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x255c0) {
    free(*(void **)((long)v3 + 8));
    dat_25098 = 0x255c0;
    dat_25090 = 0x100;
  }
  if (v3 == (void *)0x25090) {
    dat_25078 = 1;
    return;
  }
  free(v3);
  dat_25078 = 1;
  dat_25080 = (void *)0x25090;
}

// Function: sub_14a20 @ 0x14a20
char * sub_14a20(unsigned int a0,char *a1)
{
  return sub_144f0(a0,a1,0xffffffffffffffff,(struct_13 *)0x256c0); // tail-call
}

// Function: sub_14a40 @ 0x14a40
char * sub_14a40(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_144f0(a0,a1,a2); // tail-call
}

// Function: sub_14a50 @ 0x14a50
char * sub_14a50(char *a0)
{
  return sub_14a20(0,a0); // tail-call
}

// Function: sub_14a60 @ 0x14a60
char * sub_14a60(char *a0,unsigned long a1)
{
  return sub_14a40(0,a0,a1); // tail-call
}

// Function: sub_14a70 @ 0x14a70
void sub_14a70(unsigned int a0,unsigned long a1,char *a2)
{
  char v1 [56];
  
  sub_12970(v1);
  sub_144f0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_14ad0 @ 0x14ad0
void sub_14ad0(unsigned int a0,unsigned long a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_12970(v1);
  sub_144f0(a0,a2,a3,v1);
}

// Function: sub_14b40 @ 0x14b40
void sub_14b40(unsigned int a0,char *a1)
{
  sub_14a70(0,a0,a1); // tail-call
}

// Function: sub_14b50 @ 0x14b50
void sub_14b50(unsigned int a0,char *a1,unsigned long a2)
{
  sub_14ad0(0,a0,a1,a2); // tail-call
}

// Function: sub_14b70 @ 0x14b70
void sub_14b70(char *a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_256c0;
  v3 = dat_256c8;
  v4 = dat_256d0;
  v5 = dat_256d8;
  v8 = dat_256f0;
  v6 = dat_256e0;
  v7 = dat_256e8;
  sub_14760(&v1,(int)a2,1,a3,a0,a1);
  sub_144f0(0,a0,a1,v2);
}

// Function: sub_14c00 @ 0x14c00
void sub_14c00(char *a0,char a1)
{
  sub_14b70(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_14c20 @ 0x14c20
void sub_14c20(char *a0)
{
  sub_14c00(a0,0x3a); // tail-call
}

// Function: sub_14c30 @ 0x14c30
void sub_14c30(char *a0,unsigned long a1)
{
  sub_14b70(a0,a1,0x3a); // tail-call
}

// Function: sub_14c40 @ 0x14c40
void sub_14c40(unsigned int a0,unsigned long a1,char *a2)
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
  
  sub_12970(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_14760(v3,0x3a,1);
  sub_144f0(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_14ce0 @ 0x14ce0
void sub_14ce0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_256c0;
  v2 = dat_256c8;
  v7 = dat_256f0;
  v3 = dat_256d0;
  v4 = dat_256d8;
  v5 = dat_256e0;
  v6 = dat_256e8;
  sub_147c0(&v1,a1,a2);
  sub_144f0(a0,a3,a4,&v1);
}

// Function: sub_14d80 @ 0x14d80
void sub_14d80(unsigned int a0)
{
  sub_14ce0(a0); // tail-call
}

// Function: sub_14d90 @ 0x14d90
void sub_14d90(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_14d80(0,a0,a1,a2); // tail-call
}

// Function: sub_14db0 @ 0x14db0
void sub_14db0(long a0,long a1,char *a2,unsigned long a3)
{
  sub_14ce0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_14dd0 @ 0x14dd0
char * sub_14dd0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_144f0(a0,a1,a2); // tail-call
}

// Function: sub_14de0 @ 0x14de0
char * sub_14de0(char *a0,unsigned long a1)
{
  return sub_14dd0(0,a0,a1); // tail-call
}

// Function: sub_14df0 @ 0x14df0
char * sub_14df0(unsigned int a0,char *a1)
{
  return sub_14dd0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_14e00 @ 0x14e00
char * sub_14e00(char *a0)
{
  return sub_14df0(0,a0); // tail-call
}

// Function: sub_14e10 @ 0x14e10
unsigned long sub_14e10(int a0)
{
  *__errno_location() = a0;
  return 0xffffffff;
}

// Function: sub_14e30 @ 0x14e30
void sub_14e30(int a0,char *a1,int a2,char *a3)
{
  renameat(a0,a1,a2,a3); // tail-call
}

// Function: sub_14e40 @ 0x14e40
unsigned long sub_14e40(int a0,char *a1,int a2,char *a3,unsigned int a4)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  bool v6;
  stat v7; // stack - 0xd8
  stat v8; // stack - 0x168
  bool v9; // zf
  
  v2 = renameat2(a0,a1,a2,a3,a4);
  if (0 <= (int)v2) {
    return (unsigned long)v2; // return-dupe
  }
  v3 = __errno_location();
  v1 = *v3;
  v9 = (v1 - 0x16U & 0xffffffefU) != 0;
  v6 = v9 && v1 != 0x5f;
  if (v9 && v1 != 0x5f) {
    return (unsigned long)v2;
  }
  if (a4) { // branch-flip
    if (a4 != 1)
      return sub_14e10(0x5f); // tail-call
    if ((!fstatat(a2,a3,&v7,0x100)) || (*v3 == 0x4b)) {
      v2 = sub_14e10(0x11);
      return (unsigned long)v2;
    }
    if (*v3 == 2) {
      v6 = 1;
      goto label_14f36;
    }
    v2 = 0xffffffff;
  }
  else {
label_14f36:
    v4 = strlen(a1);
    v5 = strlen(a3);
    if (((v4) && (v5)) && ((a1[v4 - 1] == '/' || (a3[v5 - 1] == '/')))) {
      if (fstatat(a0,a1,&v8,0x100)) {
        v2 = 0xffffffff;
        return (unsigned long)v2;
      }
      if (v6) {
        if ((v8._24_4_ & 0xf000) != 0x4000) {
          v2 = sub_14e10(2);
          return (unsigned long)v2;
        }
      }
      else if (fstatat(a2,a3,&v7,0x100)) { // branch-flip
        if ((*v3 != 2) || ((v8._24_4_ & 0xf000) != 0x4000)) {
          v2 = 0xffffffff;
          return (unsigned long)v2;
        }
      }
      else {
        if ((v7._24_4_ & 0xf000) != 0x4000) {
          v2 = sub_14e10(0x14);
          return (unsigned long)v2;
        }
        if ((v8._24_4_ & 0xf000) != 0x4000) {
          v2 = sub_14e10(0x15);
          return (unsigned long)v2;
        }
      }
    }
    v2 = sub_14e30(a0,a1,a2,a3);
  }
  return (unsigned long)v2;
}

// Function: sub_150c0 @ 0x150c0
long sub_150c0(int a0,void *a1,unsigned long a2)
{
  int v1;
  long v2; // rax
  
  while( true ) {
    do {
      v2 = write(a0,a1,a2);
      if (0 <= v2)
        return v2;
      v1 = *__errno_location();
    } while (v1 == 4);
    if (v1 != 0x16) break;
    if (a2 <= 0x7ff00000)
      return v2;
    a2 = 0x7ff00000;
  }
  return v2;
}

// Function: sub_15130 @ 0x15130
unsigned long sub_15130(int a0,char *a1,int a2,char *a3)
{
  char *v1; // rax
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  stat v6; // stack - 0x168
  stat v7; // stack - 0xd8
  unsigned long v8; // r9
  
  v1 = (char *)sub_db60(a1);
  v2 = (char *)sub_db60(a3);
  v3 = sub_dbc0(v1);
  v8 = 0;
  if (v3 == sub_dbc0(v2)) {
    v8 = 0;
    if (!memcmp(v1,v2,v3)) {
      v1 = (char *)sub_e730(a1);
      if (fstatat(a0,v1,&v6,0x100)) {
        error(1,*__errno_location(),"%s",v1);
        return v5;
      }
      free(v1);
      v1 = (char *)sub_e730(a3);
      if (fstatat(a2,v1,&v7,0x100)) {
        error(1,*__errno_location(),"%s",v1);
        return v4;
      }
      free(v1);
      v8 = (unsigned long)(v6._8_8_ == v7._8_8_ && v6._0_8_ == v7._0_8_);
    }
  }
  return v8;
}

// Function: sub_152d0 @ 0x152d0
void sub_152d0(char *a0,char *a1)
{
  sub_15130(0xffffff9c,a0,0xffffff9c,a1); // tail-call
}

// Function: sub_152f0 @ 0x152f0
int sub_152f0(void *a0,void *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)((long)a1 + 8);
  v2 = *(unsigned long *)((long)a0 + 8);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_15310 @ 0x15310
void sub_15310(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_15320 @ 0x15320
char * sub_15320(DIR *a0,unsigned int a1)
{
  long v1;
  char *v10;
  long v11; // stack - 0x48
  long v12; // stack - 0x50
  unsigned long *v13;
  unsigned long *v14; // rbp
  long v15;
  long v16; // r12
  unsigned long *v17; // stack - 0x70
  char v2;
  long v3;
  int *v4; // rax
  dirent *v5; // rax
  char *v6;
  char *v7; // rax
  char *v8;
  long v9; // rbx
  
  v12 = 0;
  v11 = 0;
  v3 = *(long *)((unsigned long)a1 * 8 + 0x24960);
  if (a0) { // branch-flip
    v9 = 0;
    v15 = 0;
    v8 = NULL;
    v4 = __errno_location();
    v17 = NULL;
    while( true ) {
      *v4 = 0;
      v5 = readdir(a0);
      if (!v5) break;
      v6 = &v5->field_0x13;
      v2 = v5->field_0x13;
      if ((v2 == '.') && (v2 = v5->field_0x14, v2 == '.'))
        v2 = v5->field_0x15;
      if (v2) {
        v1 = strlen(v6) + 1;
        if (v3) { // branch-flip
          if (v11 == v15)
            v17 = (unsigned long *)sub_17050(v17,&v11,1,-1,0x10);
          v16 = v15 + 1;
          v17[v15 * 2] = sub_17230(v6);
          (&v17[v15 * 2])[1] = *(unsigned long *)v5;
          v15 = v16;
        }
        else {
          if (v12 - v9 <= v1)
            v8 = (char *)sub_17050(v8,&v12,v1 - (v12 - v9),0x7ffffffffffffffe,1);
          memcpy(&v8[v9],v6,v1);
        }
        v9 += v1;
      }
    }
    if (*v4) { // branch-flip
      free(v17);
      v6 = NULL;
      free(v8);
    }
    else {
      if (v3) { // branch-flip
        if (v15) { // branch-flip
          v16 = 0;
          qsort(v17,v15,0x10,v3);
          v10 = NULL;
          v8 = (char *)sub_16eb0(v9 + 1);
          v13 = v17;
          do {
            v16 += 1;
            v14 = &v13[2];
            v7 = stpcpy(&v8[(long)v10],(char *)*v13);
            v10 = &v7[(long)&v10[1 - (long)&v8[(long)v10]]];
            free((void *)*v13);
            v13 = v14;
          } while (v15 != v16);
          v6 = &v8[(long)v10];
        }
        else {
          v6 = (char *)sub_16eb0(v9 + 1);
          v8 = v6;
        }
        free(v17);
      }
      else {
        if (v9 == v12)
          v8 = (char *)sub_16f20(v8,v9 + 1);
        v6 = &v8[v9];
      }
      *v6 = 0;
      v6 = v8;
    }
  }
  else {
    v6 = NULL;
  }
  return v6;
}

// Function: sub_15610 @ 0x15610
void * sub_15610(unsigned long a0,unsigned int a1)
{
  DIR *v1; // rax
  void *v2; // rax
  
  v1 = (DIR *)sub_17af0();
  if (!v1)
    return NULL;
  v2 = (void *)sub_15320(v1,a1);
  if (!closedir(v1))
    return v2;
  free(v2);
  return NULL;
}

// Function: sub_15680 @ 0x15680
struct_0 * sub_15680(struct_0 *a0,unsigned long a1) // early-return
{
  struct_0 *v1;
  struct_0 *v2;
  void *v3; // rax
  
  v1 = (struct_0 *)a0->field_0x0;
  if (v1 != &a0[1]) {
    v2 = realloc(v1,a1);
    if (!v2)
      v2 = v1;
    return v2;
  }
  v3 = malloc(a1);
  if (!v3)
    return NULL;
  return memcpy(v3,v1,a1); // tail-call
}

// Function: sub_156e0 @ 0x156e0
void sub_156e0(struct_0 *a0)
{
  a0->field_0x8 = 0x400;
  a0->field_0x0 = (long *)&a0[1];
}

// Function: sub_156f0 @ 0x156f0
void sub_156f0(struct_0 *a0)
{
  if ((struct_0 *)a0->field_0x0 != &a0[1]) {
    free((struct_0 *)a0->field_0x0); // tail-call
    return;
  }
}

// Function: sub_15710 @ 0x15710
unsigned long sub_15710(struct_0 *a0)
{
  unsigned long v1;
  long *v2; // rax
  
  v1 = a0->field_0x8 * 2;
  sub_156f0(a0);
  if ((unsigned long)a0->field_0x8 <= v1) { // branch-flip
    v2 = malloc(v1);
    if (v2) {
      a0->field_0x0 = v2;
      a0->field_0x8 = v1;
      return 1;
    }
  }
  else {
    *__errno_location() = 0xc;
  }
  sub_156e0(a0);
  return 0;
}

// Function: sub_15780 @ 0x15780
void sub_15780(struct_0 *a0)
{
  a0->field_0x8 = 0x400;
  a0->field_0x0 = (long *)&a0[1];
}

// Function: sub_15790 @ 0x15790
unsigned long sub_15790(struct_0 *a0) // return-dupe
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // rax
  struct_0 *v4;
  
  v4 = (struct_0 *)a0->field_0x0;
  v1 = a0->field_0x8 * 2;
  if (v4 != &a0[1]) {
    if ((unsigned long)a0->field_0x8 <= v1) { // branch-flip
      v2 = realloc(v4,v1);
      if (v2) goto label_157c7;
      v4 = (struct_0 *)a0->field_0x0;
    }
    else {
      *__errno_location() = 0xc;
    }
    free(v4);
    sub_15780(a0);
    return 0;
  }
  v3 = malloc(v1);
  if (!v3)
    return 0;
  v2 = memcpy(v3,v4,a0->field_0x8);
label_157c7:
  a0->field_0x0 = v2;
  a0->field_0x8 = v1;
  return 1;
}

// Function: sub_15840 @ 0x15840
int sub_15840(int a0,char **a1)
{
  char *v1;
  
  if (a0) { // branch-flip
    if (a0 == 10) {
      v1 = *a1;
      if (!strcmp(v1,"unlabeled")) {
        a0 = -1;
        freecon(v1);
        *a1 = NULL;
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

// Function: sub_158c0 @ 0x158c0
void sub_158c0(char *a0,char **a1)
{
  sub_15840(getfilecon(a0,a1),a1); // tail-call
}

// Function: sub_158e0 @ 0x158e0
void sub_158e0(char *a0,char **a1)
{
  sub_15840(lgetfilecon(a0,a1),a1); // tail-call
}

// Function: sub_15900 @ 0x15900
void sub_15900(int a0,char **a1)
{
  sub_15840(fgetfilecon(a0,a1),a1); // tail-call
}

// Function: sub_15920 @ 0x15920
unsigned long sub_15920(struct_33 *a0)
{
  return a0->field_0x50;
}

// Function: sub_15930 @ 0x15930
unsigned long sub_15930(struct_34 *a0)
{
  return a0->field_0x70;
}

// Function: sub_15940 @ 0x15940
unsigned long sub_15940(struct_35 *a0)
{
  return a0->field_0x60;
}

// Function: sub_15950 @ 0x15950
unsigned long sub_15950(void)
{
  return 0;
}

// Function: sub_15960 @ 0x15960
undefined16 sub_15960(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x48);
}

// Function: sub_15970 @ 0x15970
undefined16 sub_15970(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x68);
}

// Function: sub_15980 @ 0x15980
undefined16 sub_15980(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x58);
}

// Function: sub_15990 @ 0x15990
undefined16 sub_15990(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_159b0 @ 0x159b0
unsigned int sub_159b0(unsigned int a0)
{
  return a0;
}

// Function: sub_159c0 @ 0x159c0
bool sub_159c0(char *a0)
{
  char v1;
  char *v2;
  
  v1 = *a0;
  while( true ) {
    v2 = &a0[1];
    if (v1 != '.')
      return 0;
    if (a0[1] != '/') break;
    do {
      v1 = v2[1];
      v2 = &v2[1];
    } while (v1 == '/');
    a0 = v2;
    if (!v1)
      return 1;
  }
  return a0[1] == '\0';
}

// Function: sub_15a10 @ 0x15a10
bool sub_15a10(int a0)
{
  return a0 != -1;
}

// Function: sub_15a20 @ 0x15a20
int sub_15a20(char *a0)
{
  char v1; // al
  
  v1 = sub_159c0(a0);
  if (!v1)
    return open(a0,0x210000); // tail-call
  return 0xffffff9c;
}

// Function: sub_15a60 @ 0x15a60
int sub_15a60(char *a0) // early-return
{
  int v1; // eax
  int *v2; // rax
  stat v3;
  
  v1 = lstat(a0,&v3);
  v2 = __errno_location();
  if ((v1) && (*v2 != 0x4b))
    return -(unsigned int)(*v2 != 2);
  *v2 = 0x11;
  return -1;
}

// Function: sub_15ae0 @ 0x15ae0
void sub_15ae0(char *a0)
{
  mkdir(a0,0x1c0); // tail-call
}

// Function: sub_15af0 @ 0x15af0
void sub_15af0(char *a0,unsigned int *a1)
{
  open(a0,*a1 & 0xffffff3c | 0xc2,0x180); // tail-call
}

// Function: sub_15b10 @ 0x15b10
long sub_15b10(unsigned long a0,bool a1)
{
  long v1; // rax
  long v2; // stack - 0x30
  timespec v3; // stack - 0x28
  
  if ((a1) && (v1 = getrandom(&v2,8,1), v1 == 8))
    v1 = v2;
  else {
    clock_gettime(1,&v3);
    v1 = (a0 ^ v3._8_8_) * 0x27bb2ee687b0b0fd + 0xb504f32d;
  }
  return v1;
}

// Function: sub_15ba0 @ 0x15ba0
unsigned long sub_15ba0(char *a0,int a1,unsigned long a2,void *a3,unsigned long a4)
{
  int v1;
  bool v10; // stack - 0x79
  int v11; // stack - 0x50
  int *v2; // rax
  unsigned long v3;
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rbx
  char *v7;
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x80
  
  v2 = __errno_location();
  v1 = *v2;
  v8 = (unsigned long)&v8 >> 4;
  v10 = a3 == sub_15a60;
  v3 = strlen(a0);
  if ((long)a1 + a4 <= v3) {
    v4 = v3 - ((long)a1 + a4);
    if (a4 <= strspn(&a0[v4],"X")) {
      v11 = 0x3a2f8;
      v9 = 0;
      while( true ) {
        if (a4) {
          v7 = &a0[v4];
          v3 = v8;
          do {
            if (v9) // branch-flip
              v9 -= 1;
            else {
              do {
                v3 = sub_15b10(v3,v10);
                v10 = 1;
                v8 = v3;
              } while (0xf49998db0aa753ff < v3);
              v9 = 9;
            }
            v6 = &v7[1];
            v8 = v3 / 0x3e;
            *v7 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"[v3 % 0x3e];
            v7 = v6;
            v3 = v8;
          } while (&a0[v4 + a4] != v6);
        }
        v5 = (*a3)(a0,a2);
        if (0 <= (int)v5) break;
        if ((*v2 != 0x11) || (v11 = v11 + -1, !v11))
          return 0xffffffff;
      }
      *v2 = v1;
      return v5;
    }
  }
  *v2 = 0x16;
  return 0xffffffff;
}

// Function: sub_15da0 @ 0x15da0
void sub_15da0(char *a0,int a1,unsigned int a2,int a3,unsigned long a4)
{
  unsigned int v1 [3]; // stack - 0xc
  
  v1[0] = a2;
  sub_15ba0(a0,a1,v1,*(void **)((long)a3 * 8 + 0x24980),a4);
}

// Function: sub_15dd0 @ 0x15dd0
void sub_15dd0(void)
{
  sub_15da0(); // tail-call
}

// Function: sub_15de0 @ 0x15de0
unsigned long sub_15de0(char *a0,int a1,unsigned long a2,void *a3)
{
  return sub_15ba0(a0,a1,a2,a3); // tail-call
}

// Function: sub_15df0 @ 0x15df0
unsigned int sub_15df0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_17f70();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_15e50 @ 0x15e50
unsigned long sub_15e50(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_15e60 @ 0x15e60
unsigned long sub_15e60(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_15e70 @ 0x15e70
unsigned int sub_15e70(int a0,char *a1,struct_36 *a2,struct_37 *a3,unsigned int a4)
{
  long v1;
  unsigned int v10;
  unsigned long *v11;
  long v12;
  long v13;
  int v14;
  int v15;
  int v16;
  unsigned long v17; // stack - 0x108
  unsigned long v18; // stack - 0xf8
  stat v19; // stack - 0xd8
  unsigned long v2;
  int v20;
  unsigned int v21;
  long v22;
  bool v23; // r12b
  bool v24;
  unsigned int v25; // stack - 0x148
  unsigned int v26; // stack - 0x100
  char v27; // stack - 0xfc
  long v28; // stack - 0xf0
  unsigned long v29; // stack - 0xe8
  unsigned long v3;
  long v30; // stack - 0xe0
  undefined8 v4;
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  int v9;
  
  v1 = a2->field_0x58;
  v22 = a3->field_0x58;
  v5 = sub_15940(a2);
  v6 = sub_15940(a3);
  v23 = v1 == v22;
  v16 = a4 & 1;
  if (!(a4 & 1)) goto label_15f98;
  if (v5 == v6 && v23)
    return 0;
  if (v1 < (long)(v22 - 1))
    return 0xffffffff;
  if (v22 < (long)(v1 - 1))
    return v16;
  v13 = dat_25700;
  if ((dat_25700) || (v13 = sub_11cd0(0x10,NULL,sub_15e50,sub_15e60,dat_24fd0), dat_25700 = v13, v13)) {
    v11 = dat_256f8;
    if (dat_256f8) { // branch-flip
label_15f2b:
      *v11 = a2->field_0x0;
      v11 = (unsigned long *)sub_122f0(v13);
      if (!v11) {
        if (!dat_25700) goto label_16053;
        goto label_164c0;
      }
      if (dat_256f8 == v11)
        dat_256f8 = NULL;
    }
    else {
      v11 = malloc(0x10);
      dat_256f8 = v11;
      if (v11) {
        *(unsigned int *)&v11[1] = 2000000000;
        *(char *)((long)v11 + 0xc) = 0;
        goto label_15f2b;
      }
label_164c0:
      v17 = a2->field_0x0;
      v11 = (unsigned long *)sub_11a70(dat_25700,&v17);
      if (!v11) goto label_16053;
    }
    v9 = *(int *)&v11[1];
    if (*(char *)((long)v11 + 0xc)) {
      v22 &= (long)(int)~(unsigned int)(v9 == 2000000000);
      v6 -= v6 % v9;
      v23 = v1 == v22;
      goto label_15f98;
    }
  }
  else {
label_16053:
    v11 = &v17;
    v26 = 2000000000;
    v27 = 0;
    v9 = 2000000000;
  }
  v2 = a2->field_0x48;
  v3 = a2->field_0x68;
  v7 = sub_15920(a2);
  v8 = sub_15930(a2);
  v15 = v7 / 10;
  v14 = v8 / 10;
  v20 = v5 / 10;
  if (!(v5 % 10) && (!(v7 % 10) && !(v8 % 10))) {
    v24 = 0; // branch-flip
    v12 = v22;
    if (0xb <= v9) {
      v21 = 10;
      if (v20 % 10 || (v14 % 10 || v15 % 10)) // branch-flip
        *(unsigned int *)&v11[1] = 10;
      else {
        v8 = 8;
        v10 = v21;
        do {
          v21 = v10 * 10;
          v15 /= 10;
          v14 /= 10;
          v20 /= 10;
          if ((v9 <= (int)v21) || (v20 % 10 || (v15 % 10 || v14 % 10))) {
            v24 = v10 == 200000000;
            *(unsigned int *)&v11[1] = v21;
            v12 = (long)(int)~(unsigned int)v24 & v22;
            goto label_162cb;
          }
          v8 -= 1;
          v10 = v21;
        } while (v8);
        if ((v2 | v1 | v3) & 1) { // branch-flip
          *(unsigned int *)&v11[1] = 1000000000;
          v21 = 1000000000;
        }
        else {
          *(unsigned int *)&v11[1] = 2000000000;
          v21 = 2000000000;
          v24 = 1;
          v12 = v22 & 0xfffffffffffffffe;
        }
      }
    }
    else {
      *(unsigned int *)&v11[1] = 10;
      v21 = 10;
    }
label_162cb:
    if (v22 < v1)
      return v16;
    if ((v6 <= v5) && (v23))
      return v16;
    if ((v1 < v12) || ((v1 == v12 && (v5 < v6 - v6 % (int)v21))))
      return 0xffffffff;
    v29 = v24 | v1;
    v30 = (long)(int)(v21 / 9 + v5);
    v18 = v2;
    v28 = (long)v7;
    if (utimensat(a0,a1,&v18,0x100))
      return 0xfffffffe;
    v9 = fstatat(a0,a1,&v19,0x100);
    v4 = v19._88_8_;
    v25 = v19._88_4_;
    v13 = sub_15940(&v19);
    if ((v5 != v13 || v9) || v4 != v1) {
      v29 = v1;
      v30 = (long)v5;
      utimensat(a0,a1,&v18,0x100);
      if (v9)
        return 0xfffffffe;
      v25 = v19._88_4_;
      v13 = sub_15940(&v19);
    }
    v9 = (v25 & 1) * 1000000000 + (int)v13;
    v10 = v9 * -0x33333333 + 0x19999998;
    if ((v10 >> 1 | (unsigned int)((v10 & 1) != 0) << 0x1f) <= 0x19999998) {
      if (v21 != 10) { // branch-flip
        v15 = 9;
        v16 = 10;
        do {
          v9 /= 10;
          v10 = v9 * -0x33333333 + 0x19999998;
          if (0x19999998 < (v10 >> 1 | (unsigned int)((v10 & 1) != 0) << 0x1f)) break;
          v15 -= 1;
          if (!v15) {
            v16 = 2000000000;
            break;
          }
          v16 *= 10;
        } while (v16 != v21);
      }
      else {
        v16 = 10;
      }
    }
    v22 &= (long)(int)~(unsigned int)(v16 == 2000000000);
    v6 -= v6 % v16;
    v23 = v1 == v22;
  }
  *(unsigned int *)&v11[1] = v16;
  *(char *)((long)v11 + 0xc) = 1;
label_15f98:
  return ((unsigned int)(v6 < v5) - (unsigned int)(v5 < v6) & -(unsigned int)v23) + ((unsigned int)(v22 < v1) - (unsigned int)(v1 < v22));
}

// Function: sub_16690 @ 0x16690
unsigned int sub_16690(char *a0,struct_36 *a1,struct_37 *a2,unsigned int a3)
{
  return sub_15e70(0xffffff9c,a0,a1,a2,a3); // tail-call
}

// Function: sub_166b0 @ 0x166b0
void sub_166b0(int a0,int a1,char *a2,unsigned int a3,int *a4,unsigned long *a5)
{
  char *v1; // rax
  
  v1 = (char *)sub_17480(a4,a5);
  if (v1) {
    if (a2) // branch-flip
      error_at_line(a0,a1,a2,a3,"%s",v1);
    else {
      error(a0,a1,"%s",v1);
    }
    free(v1); // tail-call
    return;
  }
  v1 = dcgettext(NULL,"unable to display error message",5);
  error(0,*__errno_location(),v1);
  abort(); // no-return
}

// Function: sub_16760 @ 0x16760
void sub_16760(unsigned long a0,unsigned long a1,int *a2,unsigned long *a3)
{
  sub_166b0(a0,a1,NULL,0,a2,a3); // tail-call
}

// Function: sub_16780 @ 0x16780
long sub_16780(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_168ad:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_168e6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x1de40) + 0x1de40;
  switch(a5) {
    case 0:
      goto label_168e6;
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
      goto label_168ad;
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

// Function: sub_16c00 @ 0x16c00
void sub_16c00(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_16780(); // tail-call
}

// Function: sub_16c20 @ 0x16c20
void sub_16c20(FILE *a0,char *a1,char *a2,char *a3,struct_39 *a4)
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
  sub_16780(a0,a1,a2,a3);
}

// Function: sub_16cc0 @ 0x16cc0
void sub_16cc0(void)
{
  sub_16c20();
}

// Function: sub_16d70 @ 0x16d70
void sub_16d70(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_16e20 @ 0x16e20
unsigned long sub_16e20(void)
{
  unsigned int v1; // eax
  
  if (dat_25709)
    return (unsigned long)dat_25708;
  v1 = geteuid();
  dat_25709 = 1;
  dat_25708 = v1 == 0;
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_16e60 @ 0x16e60
void sub_16e60(void)
{
  if (sub_cbe0())
    return;
  sub_17250(); // no-return
}

// Function: sub_16e80 @ 0x16e80
long sub_16e80(long a0)
{
  if (a0)
    return a0;
  sub_17250(); // no-return
}

// Function: sub_16e90 @ 0x16e90
void sub_16e90(unsigned long a0)
{
  sub_16e80(malloc(a0)); // tail-call
}

// Function: sub_16eb0 @ 0x16eb0
void sub_16eb0(void)
{
  sub_16e80(sub_126a0()); // tail-call
}

// Function: sub_16ed0 @ 0x16ed0
void sub_16ed0(void)
{
  sub_16e90(); // tail-call
}

// Function: sub_16ee0 @ 0x16ee0
void sub_16ee0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_17250(); // no-return
}

// Function: sub_16f20 @ 0x16f20
void sub_16f20(void)
{
  sub_16e80(sub_126b0()); // tail-call
}

// Function: sub_16f40 @ 0x16f40
void sub_16f40(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_17250(); // no-return
}

// Function: sub_16f80 @ 0x16f80
void sub_16f80(void *a0,unsigned long a1,unsigned long a2)
{
  sub_16f40(a0,a1,a2); // tail-call
}

// Function: sub_16f90 @ 0x16f90
void sub_16f90(void)
{
  sub_16e80(sub_126e0()); // tail-call
}

// Function: sub_16fb0 @ 0x16fb0
void sub_16fb0(unsigned long a0,unsigned long a1)
{
  sub_16f40(NULL,a0,a1); // tail-call
}

// Function: sub_16fc0 @ 0x16fc0
void sub_16fc0(unsigned long a0,unsigned long a1)
{
  sub_16f90(0,a0,a1); // tail-call
}

// Function: sub_16fd0 @ 0x16fd0
void sub_16fd0(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_17250(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_16f40(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_17040 @ 0x17040
void sub_17040(void *a0,unsigned long *a1)
{
  sub_16fd0(a0,a1,1); // tail-call
}

// Function: sub_17050 @ 0x17050
void sub_17050(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_170f6;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_170f6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_17250(a0,v3); // no-return
  sub_16ee0(a0,v3);
  *a1 = v2;
}

// Function: sub_17130 @ 0x17130
void sub_17130(unsigned long a0,unsigned long a1)
{
  sub_16e80(calloc(a0,a1)); // tail-call
}

// Function: sub_17150 @ 0x17150
void sub_17150(unsigned long a0)
{
  sub_17130(a0,1); // tail-call
}

// Function: sub_17160 @ 0x17160
void sub_17160(void)
{
  sub_16e80(sub_126d0()); // tail-call
}

// Function: sub_17180 @ 0x17180
void sub_17180(unsigned long a0)
{
  sub_17160(a0,1); // tail-call
}

// Function: sub_17190 @ 0x17190
void sub_17190(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16e90(a1),a0,a1); // tail-call
}

// Function: sub_171c0 @ 0x171c0
void sub_171c0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16eb0(a1),a0,a1); // tail-call
}

// Function: sub_171f0 @ 0x171f0
void sub_171f0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_16eb0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_17230 @ 0x17230
void sub_17230(char *a0)
{
  sub_17190(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_17250 @ 0x17250
void sub_17250(void)
{
  error(dat_25030,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_17290 @ 0x17290
void sub_17290(long *a0,unsigned int a1,long a2)
{
  if (sub_105d0(a0,a1 | 0x200,a2))
    return;
  if (*__errno_location() != 0x16)
    sub_17250(); // no-return
  __assert_fail("errno != EINVAL","lib/xfts.c",0x29,"xfts_open"); // no-return
}

// Function: sub_172e0 @ 0x172e0
bool sub_172e0(void *a0,void *a1)
{
  unsigned int v1; // eax
  bool v2; // r8b
  
  v2 = 1;
  v1 = *(unsigned int *)((long)a0 + 0x48) & 0x11;
  if ((v1 != 0x10) && (v2 = 0, v1 == 0x11))
    return *(long *)((long)a1 + 0x58) != 0;
  return v2;
}

// Function: sub_17320 @ 0x17320
char * sub_17320(long a0,unsigned long *a1)
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
      v4 = sub_17f90(v4,strlen(v3));
      v9 -= 1;
    } while (v9);
    if (0x80000000 <= v4) {
      *__errno_location() = 0x4b;
      return NULL;
    }
    v6 = (char *)sub_16e90(v4 + 1);
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
    v10 = (char *)sub_16e90(1);
    v6 = v10;
  }
  *v10 = 0;
  return v6;
}

// Function: sub_17480 @ 0x17480
unsigned long sub_17480(int *a0,unsigned long *a1) // early-return
{
  char v1;
  unsigned long v2; // stack - 0x18
  long v3;
  
  v1 = (char)*a0;
  v3 = 0;
  while( true ) {
    if (!v1)
      return sub_17320(v3,a1); // tail-call
    if ((v1 != '%') || (*(char *)((long)a0 + v3 * 2 + 1) != 's')) break;
    v3 += 1;
    v1 = *(char *)((long)a0 + v3 * 2);
  }
  if (0 <= (int)sub_175e0(&v2,a0))
    return v2;
  if (*__errno_location() != 0xc)
    return 0;
  sub_17250(); // no-return
}

// Function: sub_17540 @ 0x17540
bool sub_17540(void)
{
  long v1; // rax
  unsigned long v2; // stack - 0x18
  char *v3; // stack - 0x20
  bool v4;
  
  v4 = 0;
  v3 = NULL;
  v2 = 0;
  v1 = getline(&v3,&v2,stdin);
  if (1 <= v1) {
    if (v3[v1 + -1] == '\n')
      v3[v1 + -1] = '\0';
    v4 = 0 < rpmatch(v3);
  }
  free(v3);
  return v4;
}

// Function: sub_175e0 @ 0x175e0
unsigned long sub_175e0(unsigned long *a0,int *a1,struct_27 *a2) // early-return x2
{
  void *v1; // rax
  unsigned long v2; // stack - 0x18
  
  v1 = (void *)sub_18000(NULL,&v2,a1,a2);
  if (!v1)
    return 0xffffffff;
  if (v2 <= 0x7fffffff) {
    *a0 = v1;
    return v2;
  }
  free(v1);
  *__errno_location() = 0x4b;
  return 0xffffffff;
}

// Function: sub_17660 @ 0x17660
void sub_17660(void)
{
  return;
}

// Function: sub_17670 @ 0x17670
unsigned long sub_17670(unsigned long a0,unsigned long a1,unsigned int a2,unsigned int *a3)
{
  *a3 = a2;
  return 0;
}

// Function: sub_17680 @ 0x17680
void sub_17680(char *a0,int a1,unsigned int a2)
{
  if (a1 != -1) {
    fchmod(a1,a2); // tail-call
    return;
  }
  chmod(a0,a2); // tail-call
}

// Function: sub_176a0 @ 0x176a0
int sub_176a0(unsigned int *a0,char *a1,unsigned int a2)
{
  unsigned int v1;
  
  v1 = *a0;
  return -(unsigned int)(sub_17680(a1,a2,v1) != 0);
}

// Function: sub_176d0 @ 0x176d0
unsigned long sub_176d0(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_176e0 @ 0x176e0
unsigned long sub_176e0(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_176f0 @ 0x176f0
unsigned int sub_176f0(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_17700 @ 0x17700
unsigned int sub_17700(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_17710 @ 0x17710
unsigned long sub_17710(unsigned long a0,unsigned char a1)
{
  return sub_176d0(a0,a1); // tail-call
}

// Function: sub_17720 @ 0x17720
unsigned long sub_17720(unsigned long a0,unsigned char a1)
{
  return sub_176e0(a0,a1); // tail-call
}

// Function: sub_17730 @ 0x17730
unsigned int sub_17730(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_17740 @ 0x17740
unsigned int sub_17740(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_17750 @ 0x17750
unsigned int sub_17750(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_17760 @ 0x17760
unsigned int sub_17760(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_17770 @ 0x17770
unsigned long sub_17770(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_177a0 @ 0x177a0
bool sub_177a0(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_177c0 @ 0x177c0
bool sub_177c0(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_177d0 @ 0x177d0
unsigned int sub_177d0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_177f0 @ 0x177f0
unsigned long sub_177f0(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_17810 @ 0x17810
bool sub_17810(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_17820 @ 0x17820
bool sub_17820(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_17830 @ 0x17830
bool sub_17830(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_17840 @ 0x17840
bool sub_17840(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_17850 @ 0x17850
unsigned long sub_17850(int a0)
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

// Function: sub_178a0 @ 0x178a0
bool sub_178a0(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_178c0 @ 0x178c0
bool sub_178c0(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_178d0 @ 0x178d0
unsigned int sub_178d0(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_17900 @ 0x17900
int sub_17900(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_17920 @ 0x17920
int sub_17920(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_17940 @ 0x17940
int sub_17940(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_17900((unsigned char)*a0);
    v1 = sub_17900((unsigned char)*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_179a0 @ 0x179a0
unsigned long sub_179a0(FILE *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_17b90(a0);
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

// Function: sub_17a10 @ 0x17a10
unsigned long sub_17a10(unsigned long a0)
{
  return CONCAT71((undefined7)(a0 - 1 >> 8),(a0 - 1 & a0) == 0);
}

// Function: sub_17a20 @ 0x17a20
void sub_17a20(void *a0)
{
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned int *)((long)a0 + 0x18) = 0x95f616;
}

// Function: sub_17a40 @ 0x17a40
unsigned long sub_17a40(struct_16 *a0,long *a1) // return-dupe x3
{
  long v1;
  char v2; // al
  unsigned long v3;
  long *v4; // rdx
  long *v5; // rdx
  long *v6;
  long v7;
  
  if (a0->field_0x18 != 0x95f616)
    __assert_fail("state->magic == 9827862","lib/cycle-check.c",0x3c,"cycle_check"); // no-return
  if (a0->field_0x10) { // branch-flip
    v7 = a1[1];
    if ((v7 == a0->field_0x0) && (*a1 == a0->field_0x8))
      return 1;
    v3 = a0->field_0x10 + 1;
    a0->field_0x10 = v3;
    v2 = sub_17a10(v3);
    if (!v2)
      return 0;
    v6 = v4;
    if (!v3)
      return 1;
  }
  else {
    a0->field_0x10 = 1;
    if (!sub_17a10(1))
      return 0;
    v7 = a1[1];
    v6 = v5;
  }
  v1 = *a1;
  *v6 = v7;
  v6[1] = v1;
  return 0;
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
      v2 = sub_17d10(v1,0x406,3);
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
unsigned long sub_17b90(FILE *a0) // early-return
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
    if (sub_e9c0(a0)) {
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

// Function: sub_17c20 @ 0x17c20
void sub_17c20(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_17c30 @ 0x17c30
int sub_17c30(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_2570c) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_2570c = 1;
      return v2;
    }
    v2 = sub_17c20(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_2570c = -1;
  }
  else {
    v2 = sub_17c20(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_2570c != -1)
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

// Function: sub_17d10 @ 0x17d10
void sub_17d10(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_17c20(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_17c30(a0,a2);
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

// Function: sub_17e60 @ 0x17e60
unsigned long sub_17e60(char *a0,unsigned long a1)
{
  char v1;
  unsigned long v2; // rax
  
  v1 = *a0;
  if (v1) {
    v2 = 0;
    do {
      a0 = &a0[1];
      v2 = (v2 << 9 | v2 >> 0x37) + (long)v1;
      v1 = *a0;
    } while (v1);
    return v2 % a1;
  }
  return 0;
}

// Function: sub_17ea0 @ 0x17ea0
char * sub_17ea0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_17ee0 @ 0x17ee0
unsigned long sub_17ee0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_19020(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_17f70 @ 0x17f70
void sub_17f70(unsigned long a0)
{
  sub_17d10(a0,0,3); // tail-call
}

// Function: sub_17f90 @ 0x17f90
long sub_17f90(unsigned long a0,unsigned long a1) // early-return
{
  if (CARRY8(a0,a1))
    return -1;
  return a0 + a1;
}

// Function: sub_17fb0 @ 0x17fb0
void sub_17fb0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  
  sub_17f90(a0,a1);
  sub_17f90(v1,v2); // tail-call
}

// Function: sub_17fd0 @ 0x17fd0
void sub_17fd0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  
  sub_17f90(a0,a1);
  sub_17f90(sub_17f90(v1,v2),a3); // tail-call
}

// Function: sub_17ff0 @ 0x17ff0
unsigned long sub_17ff0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = a1;
  if (a1 <= a0)
    v1 = a0;
  return v1;
}

// Function: sub_18000 @ 0x18000
void * sub_18000(void *a0,unsigned long *a1,int *a2,struct_27 *a3)
{
  unsigned long v1;
  unsigned long v10;
  void *v11;
  int *v12;
  int *v13; // rax
  unsigned long v14;
  void *v15;
  char v16 [8];
  char v17 [8];
  long v18; // stack - 0x2c8
  unsigned long v19 [77];
  undefined10 v2;
  int v20 [56];
  unsigned int v21 [2]; // stack - 0x3c0
  int v22; // stack - 0x3c4
  char *v23;
  char *v24;
  char *v25; // rsp
  char *v26; // rsp
  unsigned long v27;
  unsigned long v28;
  char *v29;
  char v3;
  unsigned long *v30;
  bool v31; // zf
  unsigned long v32; // stack - 0x420
  int v33; // stack - 0x414
  char *v34; // stack - 0x410
  unsigned long *v35; // stack - 0x408
  char *v36; // stack - 0x400
  long v37; // stack - 0x3f8
  char *v38; // stack - 0x3f0
  int *v39; // stack - 0x3e8
  unsigned int v4;
  int *v40; // stack - 0x3e0
  unsigned long v41; // stack - 0x3d8
  void *v42; // stack - 0x3d0
  int *v43; // stack - 0x3b0
  unsigned long *v44; // stack - 0x2c0
  unsigned long v45; // stack - 0x2b8
  unsigned long v46; // stack - 0x2b0
  long v5;
  unsigned long *v6;
  int v7;
  int v8;
  unsigned long v9;
  
  v26 = v17;
  v24 = v17;
  v23 = v17;
  v35 = a1;
  v42 = a0;
  if ((int)sub_193c0(a2,&v18,v16) <= -1)
    return (void *)0;
  if ((int)sub_19190(a3,v16) < 0) {
    if (v44 != v19)
      free(v44);
    if (v43 != v20)
      free(v43);
    *__errno_location() = 0x16;
    return NULL;
  }
  v9 = sub_17fd0(7,v45,v46,6);
  if (4000 <= v9) { // branch-flip
    if (v9 != 0xffffffffffffffff) {
      v38 = malloc(v9);
      if (v38) {
        v34 = v38;
        v23 = v17;
        goto label_180c1;
      }
    }
    v40 = __errno_location();
  }
  else {
    v23 = v17;
    while (v26 != &v17[-(v9 + 0x17 & 0xfffffffffffff000)]) {
      v24 = &v23[-0x1000];
      v25 = &v23[-0x1000];
      *(unsigned long *)&v23[-8] = *(unsigned long *)&v23[-8];
      v26 = &v23[-0x1000];
      v23 = v25;
    }
    v9 = (unsigned long)((unsigned int)(v9 + 0x17) & 0xff0);
    v5 = -v9;
    v23 = &v24[v5];
    if (v9)
      *(unsigned long *)&v24[-8] = *(unsigned long *)&v24[-8];
    v34 = NULL;
    v38 = (char *)((unsigned long)&v24[v5 + 0xf] & 0xfffffffffffffff0);
label_180c1:
    v9 = 0;
    if (v42)
      v9 = *v35;
    v27 = 0;
    v37 = 0;
    v12 = (int *)*v44;
    v30 = v44;
    v15 = v42;
    if (v12 == a2) goto label_181ba;
    while( true ) {
      *(unsigned long *)&v23[-8] = 0x18124;
      v40 = a2;
      v41 = (long)v12 - (long)a2;
      v10 = sub_17f90(v27,(long)v12 - (long)a2);
      v12 = v40;
      v14 = v41;
      v11 = v15;
      if (v10 <= v9) goto label_181ab;
      if (v9) break;
      v9 = 0xc;
label_1814c:
      if (v9 < v10)
        v9 = v10;
      if (v9 == 0xffffffffffffffff) goto label_18318;
      if ((!v15) || (v15 == v42)) {
        v39 = v40;
        v40 = (int *)CONCAT71(v40._1_7_,v15 == v42);
        *(unsigned long *)&v23[-8] = 0x18b0c;
        v11 = malloc(v9);
        if (v11) {
          if ((v27) && ((char)v40)) {
            v40 = v39;
            *(unsigned long *)&v23[-8] = 0x18b5a;
            v11 = memcpy(v11,v15,v27);
            v12 = v40;
            v14 = v41;
          }
          else {
            v12 = v39;
            v14 = v41;
          }
          goto label_181ab;
        }
        goto label_18318;
      }
      *(unsigned long *)&v23[-8] = 0x18191;
      v11 = realloc(v15,v9);
      if (!v11) {
label_18bce:
        *(unsigned long *)&v23[-8] = 0x18bd3;
        v40 = __errno_location();
        goto label_18be0;
      }
      v12 = v40;
      v14 = v41;
label_181ab:
      *(unsigned long *)&v23[-8] = 0x181ba;
      memcpy((void *)((long)v11 + v27),v12,v14);
      v27 = v10;
      v15 = v11;
label_181ba:
      do {
        if (v18 == v37) {
          *(unsigned long *)&v23[-8] = 0x18ee5;
          v14 = sub_17f90(v27,1);
          v11 = v15;
          if (v9 < v14) {
            if (v9) { // branch-flip
              v10 = v9 * 2;
              if ((long)v9 < 0) goto label_18318;
            }
            else {
              v10 = 0xc;
            }
            v9 = v10;
            if (v10 <= v14)
              v9 = v14;
            if (v9 == 0xffffffffffffffff) goto label_18318;
            v31 = v15 == v42;
            if ((!v15) || (v31)) {
              *(unsigned long *)&v23[-8] = 0x18fc5;
              v11 = malloc(v9);
              if (!v11) goto label_18318;
              if ((v27) && (v31)) {
                *(unsigned long *)&v23[-8] = 0x18fe9;
                v11 = memcpy(v11,v15,v27);
              }
            }
            else {
              *(unsigned long *)&v23[-8] = 0x18f39;
              v11 = realloc(v15,v9);
              if (!v11) goto label_18bce;
            }
          }
          *(char *)((long)v11 + v27) = 0;
          if (v11 != v42) {
            if (v27 + 1 < v9) {
              *(unsigned long *)&v23[-8] = 0x18ff9;
              v15 = realloc(v11,v27 + 1);
              if (v15)
                v11 = v15;
            }
          }
          if (v34) {
            *(unsigned long *)&v23[-8] = 0x18f74;
            free(v34);
          }
          if (v44 != v19) {
            *(unsigned long *)&v23[-8] = 0x18f8c;
            free(v44);
          }
          if (v43 != v20) {
            *(unsigned long *)&v23[-8] = 0x18fa4;
            free(v43);
          }
          *v35 = v27;
          return v11;
        }
        v5 = v30[10];
        if (*(char *)&v30[9] != '%') { // branch-flip
          if (v5 == -1) {
label_19005:
            *(unsigned long *)&v23[-8] = 0x1900a; // no-return
            abort();
          }
          v8 = v43[v5 * 8];
          v41 = CONCAT44(v41._4_4_,v8);
          if (*(char *)&v30[9] != 'n') { // branch-flip
            v4 = *(unsigned int *)&v30[2];
            v29 = &v38[1];
            *v38 = '%';
            if (v4 & 1) {
              v38[1] = '\'';
              v29 = &v38[2];
            }
            if (v4 & 2) {
              *v29 = '-';
              v29 = &v29[1];
            }
            if (v4 & 4) {
              *v29 = '+';
              v29 = &v29[1];
            }
            if (v4 & 8) {
              *v29 = ' ';
              v29 = &v29[1];
            }
            if (v4 & 0x10) {
              *v29 = '#';
              v29 = &v29[1];
            }
            if (v4 & 0x40) {
              *v29 = 'I';
              v29 = &v29[1];
            }
            if (v4 & 0x20) {
              *v29 = '0';
              v29 = &v29[1];
            }
            v11 = (void *)v30[3];
            v12 = v43;
            if (v11 != (void *)v30[4]) {
              v12 = (int *)((long)(void *)v30[4] - (long)v11);
              v39 = v43;
              *(unsigned long *)&v23[-8] = 0x182a5;
              v40 = v12;
              memcpy(v29,v11,(unsigned long)v12);
              v29 = &v29[(long)v40];
              v12 = v39;
            }
            v11 = (void *)v30[6];
            if (v11 != (void *)v30[7]) {
              v13 = (int *)((long)(void *)v30[7] - (long)v11);
              *(unsigned long *)&v23[-8] = 0x182df;
              v39 = v12;
              v40 = v13;
              memcpy(v29,v11,(unsigned long)v13);
              v29 = &v29[(long)v40];
              v12 = v39;
            }
            switch((unsigned int)v41) {
              case 9:
              case 10:
                *v29 = 'l';
                v29 = &v29[1];
                goto label_18421;
              case 7:
              case 8:
              case 0xe:
              case 0x10:
label_18421:
                *v29 = 'l';
                v29 = &v29[1];
                break;
              case 0xc:
                *v29 = 'L';
                v29 = &v29[1];
              
            }
            v3 = *(char *)&v30[9];
            v29[1] = '\0';
            *v29 = v3;
            if (v30[5] != -1) { // branch-flip
              if (v12[v30[5] * 8] != 5) goto label_19005;
              v39 = (int *)CONCAT44(v39._4_4_,1);
              v21[0] = (&v12[v30[5] * 8])[4];
            }
            else {
              v39 = (int *)((unsigned long)v39._4_4_ << 0x20);
            }
            if (v30[8] != -1) {
              if (v12[v30[8] * 8] != 5) goto label_19005;
              v21[(unsigned long)v39 & 0xffffffff] = (&v12[v30[8] * 8])[4];
              v39 = (int *)CONCAT44(v39._4_4_,(int)v39 + 1);
            }
            *(unsigned long *)&v23[-8] = 0x184aa;
            v14 = sub_17f90(v27,2);
            v11 = v15;
            if (v9 < v14) {
              if (v9) { // branch-flip
                if ((long)v9 < 0) goto label_18318;
                v9 *= 2;
              }
              else {
                v9 = 0xc;
              }
              if (v9 <= v14)
                v9 = v14;
              if (v9 == 0xffffffffffffffff) goto label_18318;
              if ((!v15) || (v15 == v42)) {
                v40 = (int *)CONCAT71(v40._1_7_,v15 == v42);
                *(unsigned long *)&v23[-8] = 0x18d2e;
                v11 = malloc(v9);
                if (!v11) goto label_18318;
                if ((v27) && ((char)v40)) {
                  *(unsigned long *)&v23[-8] = 0x18d60;
                  v11 = memcpy(v11,v15,v27);
                }
              }
              else {
                *(unsigned long *)&v23[-8] = 0x184f4;
                v11 = realloc(v15,v9);
                if (!v11) goto label_18318;
              }
            }
            *(char *)((long)v11 + v27) = 0;
            *(unsigned long *)&v23[-8] = 0x1850a;
            v40 = __errno_location();
            v33 = *v40;
            v15 = v11;
            v36 = v29;
label_18520:
            v22 = -1;
            v10 = v9 - v27;
            *v40 = 0;
            v14 = 0x7fffffff;
            if (v10 <= 0x7fffffff)
              v14 = v10;
            switch(v41 & 0xffffffff) {
              default:
                goto label_19005;
              case 1:
                v28 = (unsigned long)(unsigned int)(int)(char)v43[v30[10] * 8 + 4];
                break;
              case 2:
                v28 = (unsigned long)*(unsigned char *)&v43[v30[10] * 8 + 4];
                break;
              case 3:
                v28 = (unsigned long)(unsigned int)(int)(short)v43[v30[10] * 8 + 4];
                break;
              case 4:
                v28 = (unsigned long)*(unsigned short *)&v43[v30[10] * 8 + 4];
                break;
              case 5:
              case 6:
              case 0xd:
              case 0xe:
                v28 = (unsigned long)(unsigned int)v43[v30[10] * 8 + 4];
                break;
              case 7:
              case 8:
              case 9:
              case 10:
              case 0xf:
              case 0x10:
              case 0x11:
                v12 = *(int **)&v43[v30[10] * 8 + 4];
                if ((int)v39 == 1) {
                  *(int **)&v23[-8] = &v22;
                  *(int **)&v23[-0x10] = v12;
                  goto label_188ca;
                }
                if ((int)v39 != 2) {
                  *(int **)&v23[-0x10] = &v22;
label_185ab:
                  *(unsigned long *)&v23[-0x18] = 0x185cc;
                  v32 = v14;
                  v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v12);
                  v14 = v32;
                  goto label_185d5;
                }
                *(int **)&v23[-0x10] = &v22;
                *(int **)&v23[-0x18] = v12;
                goto label_1891e;
              case 0xb:
                v1 = *(unsigned long *)&v43[v30[10] * 8 + 4];
                if ((int)v39 != 1) { // branch-flip
                  if ((int)v39 != 2) {
                    *(unsigned long *)&v23[-8] = 0x18c2b;
                    v32 = v14;
                    v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v1,&v22);
                    v14 = v32;
                    goto label_185d5;
                  }
                  *(int **)&v23[-8] = &v22;
                  *(unsigned long *)&v23[-0x10] = (unsigned long)v21[1];
                }
                else {
                  *(int **)&v23[-0x10] = &v22;
                }
                *(unsigned long *)&v23[-0x18] = 0x18882;
                v32 = v14;
                v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v1,(unsigned long)v21[0]);
                v14 = v32;
                goto label_185d5;
              case 0xc:
                v2 = *(void *)&v43[v30[10] * 8 + 4];
                if ((int)v39 != 1) {
                  if ((int)v39 != 2) {
                    v12 = &v22;
                    *(void *)&v23[-0x10] = v2;
                    goto label_185ab;
                  }
                  *(int **)&v23[-0x10] = &v22;
                  *(void *)&v23[-0x20] = v2;
                  *(unsigned long *)&v23[-0x30] = (unsigned long)v21[1];
                  *(unsigned long *)&v23[-0x38] = 0x18c83;
                  v32 = v14;
                  v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,(unsigned long)v21[0]);
                  v14 = v32;
                  goto label_185d5;
                }
                *(int **)&v23[-0x10] = &v22;
                *(void *)&v23[-0x20] = v2;
                goto label_18925;
              
            }
            if ((int)v39 == 1) {
              *(int **)&v23[-8] = &v22;
              *(unsigned long *)&v23[-0x10] = v28;
label_188ca:
              v28 = (unsigned long)v21[0];
label_186eb:
              *(unsigned long *)&v23[-0x18] = 0x1870c;
              v32 = v14;
              v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v28);
              v14 = v32;
              if (0 <= v22) goto label_185e3;
label_18723:
              if (!v36[1]) {
                if (v8 <= -1) {
                  if (!*v40) {
                    v8 = 0x16;
                    if ((*(unsigned char *)&v30[9] & 0xef) == 99)
                      v8 = 0x54;
                    *v40 = v8;
                  }
                  if ((v15 != v42) && (v15)) {
                    *(unsigned long *)&v23[-8] = 0x1877f;
                    free(v15);
                  }
                  if (v34) {
                    *(unsigned long *)&v23[-8] = 0x18793;
                    free(v34);
                  }
                  if (v44 != v19) {
                    *(unsigned long *)&v23[-8] = 0x187ab;
                    free(v44);
                  }
                  if (v43 != v20) {
                    *(unsigned long *)&v23[-8] = 0x187c7;
                    free(v43);
                    return (void *)0;
                  }
                  return (void *)0;
                }
                goto label_185fd;
              }
              v36[1] = '\0';
              goto label_18520;
            }
            if ((int)v39 != 2) {
              *(int **)&v23[-0x10] = &v22;
              goto label_186eb;
            }
            *(int **)&v23[-0x10] = &v22;
            *(unsigned long *)&v23[-0x18] = v28;
label_1891e:
            *(unsigned long *)&v23[-0x20] = (unsigned long)v21[1];
label_18925:
            *(unsigned long *)&v23[-0x28] = 0x1894d;
            v32 = v14;
            v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,(unsigned long)v21[0]);
            v14 = v32;
label_185d5:
            if (v22 <= -1) goto label_18723;
label_185e3:
            if (((unsigned long)(long)v22 < v14) && (*(char *)((long)v15 + v27 + (long)v22))) goto label_19005;
            v7 = v22;
            if (v22 < v8) {
label_185fd:
              v7 = v8;
              v22 = v8;
            }
            if (v14 <= v7 + 1) {
              if (0x7fffffff <= v10) {
                if ((v15 != v42) && (v15)) {
                  *(unsigned long *)&v23[-8] = 0x18e4e;
                  free(v15);
                }
                if (v34) {
                  *(unsigned long *)&v23[-8] = 0x18e62;
                  free(v34);
                }
                if (v44 != v19) {
                  *(unsigned long *)&v23[-8] = 0x18e7a;
                  free(v44);
                }
                if (v43 != v20) {
                  *(unsigned long *)&v23[-8] = 0x18e92;
                  free(v43);
                }
                *v40 = 0x4b;
                return NULL;
              }
              v14 = v9 * 2;
              if ((long)v9 <= -1)
                v14 = 0xffffffffffffffff;
              *(unsigned long *)&v23[-8] = 0x1863b;
              v10 = sub_17f90(v27,v7 + 2);
              *(unsigned long *)&v23[-8] = 0x18646;
              v10 = sub_17ff0(v10,v14);
              if (v9 < v10) {
                if (!v9)
                  v14 = 0xc;
                v9 = v10;
                if (v10 <= v14)
                  v9 = v14;
                if (v9 == 0xffffffffffffffff) goto label_18324;
                v31 = v15 == v42;
                if ((!v15) || (v31)) {
                  *(unsigned long *)&v23[-8] = 0x189c8;
                  v11 = malloc(v9);
                  if (!v11) goto label_18324;
                  if ((!v27) || (!v31))
                    v15 = v11;
                  else {
                    *(unsigned long *)&v23[-8] = 0x189f4;
                    v15 = memcpy(v11,v15,v27);
                  }
                }
                else {
                  *(unsigned long *)&v23[-8] = 0x18697;
                  v11 = realloc(v15,v9);
                  if (!v11) goto label_18324;
                  v15 = v11;
                }
              }
              goto label_18520;
            }
            v27 = (long)v7 + v27;
            *v40 = v33;
          }
          else {
            if (5 <= (unsigned int)(v8 - 0x12U)) goto label_19005;
            v6 = *(unsigned long **)&(&v43[v5 * 8])[4];
            switch(v8) {
              case 0x12:
                *(char *)v6 = (char)v27;
                break;
              case 0x13:
                *(short *)v6 = (short)v27;
                break;
              case 0x14:
                *(int *)v6 = (int)v27;
                break;
              default:
                *v6 = v27;
              
            }
          }
        }
        else {
          if (v5 != -1) goto label_19005;
          *(unsigned long *)&v23[-8] = 0x18a57;
          v14 = sub_17f90(v27,1);
          if (v9 < v14) {
            if (v9) { // branch-flip
              if ((long)v9 < 0) goto label_18318;
              v9 *= 2;
            }
            else {
              v9 = 0xc;
            }
            if (v9 < v14)
              v9 = v14;
            if (v9 == 0xffffffffffffffff) goto label_18318;
            if ((!v15) || (v15 == v42)) {
              v41 = CONCAT71(v41._1_7_,v15 == v42);
              *(unsigned long *)&v23[-8] = 0x18d7e;
              v11 = malloc(v9);
              if (!v11) goto label_18318;
              if ((v27) && ((char)v41)) {
                *(unsigned long *)&v23[-8] = 0x18db0;
                v15 = memcpy(v11,v15,v27);
              }
              else {
                v15 = v11;
              }
            }
            else {
              *(unsigned long *)&v23[-8] = 0x18aa4;
              v11 = realloc(v15,v9);
              if (!v11) goto label_18bce;
              v15 = v11;
            }
          }
          *(char *)((long)v15 + v27) = 0x25;
          v27 = v14;
        }
        a2 = (int *)v30[1];
        v12 = (int *)v30[0xb];
        v30 = &v30[0xb];
        v37 += 1;
      } while (v12 == a2);
    }
    if (0 <= (long)v9) {
      v9 *= 2;
      goto label_1814c;
    }
label_18318:
    *(unsigned long *)&v23[-8] = 0x1831d;
    v40 = __errno_location();
label_18324:
    if ((v15 != v42) && (v15)) {
label_18be0:
      *(unsigned long *)&v23[-8] = 0x18be8;
      free(v15);
    }
    if (v34) {
      *(unsigned long *)&v23[-8] = 0x1834a;
      free(v34);
    }
  }
  if (v44 != v19) {
    *(unsigned long *)&v23[-8] = 0x18362;
    free(v44);
  }
  if (v43 != v20) {
    *(unsigned long *)&v23[-8] = 0x1837a;
    free(v43);
  }
  *v40 = 0xc;
  return NULL;
}

// Function: sub_19020 @ 0x19020
bool sub_19020(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_19170(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_190b0 @ 0x190b0
void sub_190b0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_190c0 @ 0x190c0
unsigned long sub_190c0(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_190b0();
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

// Function: sub_19170 @ 0x19170
unsigned long sub_19170(void)
{
  return sub_190c0(); // tail-call
}

// Function: sub_19180 @ 0x19180
void sub_19180(void)
{
  sub_190b0(); // tail-call
}

// Function: sub_19190 @ 0x19190
unsigned long sub_19190(struct_27 *a0,struct_28 *a1) // return-dupe x2
{
  unsigned int v1;
  long v2;
  unsigned int *v3;
  void *v4; // rdx
  char *v5;
  long v6; // rsi
  unsigned long *v7;
  unsigned int *v8;
  long *v9;
  
  v2 = a1->field_0x0;
  v3 = a1->field_0x8;
  if (v2) {
    v6 = 0;
    do {
      switch(*v3) {
        default:
          return 0xffffffff;
        case 1:
        case 2:
          v1 = a0->field_0x0;
          if (0x30 <= v1) { // branch-flip
            v8 = (unsigned int *)a0->field_0x8;
            a0->field_0x8 = (unsigned long *)&v8[2];
          }
          else {
            v8 = (unsigned int *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x0 = v1 + 8;
          }
          *(char *)&v3[4] = (char)*v8;
          break;
        case 3:
        case 4:
          v1 = a0->field_0x0;
          if (0x30 <= v1) { // branch-flip
            v8 = (unsigned int *)a0->field_0x8;
            a0->field_0x8 = (unsigned long *)&v8[2];
          }
          else {
            v8 = (unsigned int *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x0 = v1 + 8;
          }
          *(short *)&v3[4] = (short)*v8;
          break;
        case 5:
        case 6:
        case 0xd:
        case 0xe:
          v1 = a0->field_0x0;
          if (0x30 <= v1) { // branch-flip
            v8 = (unsigned int *)a0->field_0x8;
            a0->field_0x8 = (unsigned long *)&v8[2];
          }
          else {
            v8 = (unsigned int *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x0 = v1 + 8;
          }
          v3[4] = *v8;
          break;
        case 7:
        case 8:
        case 9:
        case 10:
        case 0x11:
        case 0x12:
        case 0x13:
        case 0x14:
        case 0x15:
        case 0x16:
          v1 = a0->field_0x0;
          if (0x30 <= v1) { // branch-flip
            v7 = a0->field_0x8;
            a0->field_0x8 = &v7[1];
          }
          else {
            v7 = (unsigned long *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x0 = v1 + 8;
          }
          *(unsigned long *)&v3[4] = *v7;
          break;
        case 0xb:
          v1 = a0->field_0x4;
          if (0xb0 <= v1) { // branch-flip
            v7 = a0->field_0x8;
            a0->field_0x8 = &v7[1];
          }
          else {
            v7 = (unsigned long *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x4 = v1 + 0x10;
          }
          *(unsigned long *)&v3[4] = *v7;
          break;
        case 0xc:
          v4 = (void *)((long)a0->field_0x8 + 0xfU & 0xfffffffffffffff0);
          a0->field_0x8 = (unsigned long *)&v4[1];
          *(void *)&v3[4] = *v4;
          break;
        case 0xf:
          v1 = a0->field_0x0;
          if (0x30 <= v1) { // branch-flip
            v9 = a0->field_0x8;
            a0->field_0x8 = &v9[1];
          }
          else {
            v9 = (long *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x0 = v1 + 8;
          }
          v5 = (char *)*v9;
          if (!v5)
            v5 = "(NULL)";
          *(char **)&v3[4] = v5;
          break;
        case 0x10:
          v1 = a0->field_0x0;
          if (0x30 <= v1) { // branch-flip
            v9 = a0->field_0x8;
            a0->field_0x8 = &v9[1];
          }
          else {
            v9 = (long *)((unsigned long)v1 + a0->field_0x10);
            a0->field_0x0 = v1 + 8;
          }
          v5 = (char *)*v9;
          if (!v5)
            v5 = "(";
          *(char **)&v3[4] = v5;
        
      }
      v6 += 1;
      v3 = &v3[8];
    } while (v6 != v2);
    return 0;
  }
  return 0;
}

// Function: sub_193c0 @ 0x193c0
unsigned long sub_193c0(char *a0,unsigned long *a1,struct_29 *a2) // return-dupe x2
{
  unsigned long *v1;
  unsigned long v10;
  int v11;
  int v12;
  unsigned int v13; // edx
  unsigned long v14;
  char *v15;
  char *v16;
  char v17;
  char v18;
  unsigned int v19;
  struct_29 *v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22;
  unsigned long v23;
  unsigned long v24; // stack - 0x78
  unsigned long v25; // stack - 0x70
  unsigned long v26; // stack - 0x60
  int v3;
  unsigned long *v4;
  struct_29 *v5; // rax
  unsigned long v6;
  struct_29 *v7;
  struct_29 *v8;
  char *v9;
  
  v1 = &a1[4];
  v2 = &a2[1];
  v20 = 7;
  *a1 = 0;
  a1[1] = (unsigned long)v1;
  v6 = 0;
  a2->field_0x0 = 0;
  a2->field_0x8 = &v2->field_0x0;
  v25 = 0;
  v24 = 0;
  v26 = 0;
  v4 = v1;
  v22 = 7;
label_19444:
  if (!*a0) {
    v4[v6 * 0xb] = (unsigned long)a0;
    a1[2] = v24;
    a1[3] = v25;
    return 0;
  }
  v16 = &a0[1];
  v21 = v22;
  if (*a0 == '%') {
    v23 = 0xffffffffffffffff;
    v4 = &v4[v6 * 0xb];
    *v4 = (unsigned long)a0;
    *(unsigned int *)&v4[2] = 0;
    v4[3] = 0;
    v4[4] = 0;
    v4[5] = 0xffffffffffffffff;
    v4[6] = 0;
    v4[7] = 0;
    v4[8] = 0xffffffffffffffff;
    v4[10] = 0xffffffffffffffff;
    v19 = (unsigned int)(unsigned char)a0[1];
    if (10 <= (unsigned char)(a0[1] - 0x30U)) {
label_194ee:
      v9 = &v16[1];
      if ((char)v19 == '\'') goto label_195d1;
      do {
        switch(v19 - 0x20 & 0xff) {
          case 0:
            v19 = (unsigned int)v4[2] | 8;
            v16 = v9;
            break;
          default:
            goto label_19518;
          case 3:
            v19 = (unsigned int)v4[2] | 0x10;
            v16 = v9;
            break;
          case 0xb:
            v19 = (unsigned int)v4[2] | 4;
            v16 = v9;
            break;
          case 0xd:
            v19 = (unsigned int)v4[2] | 2;
            v16 = v9;
            break;
          case 0x10:
            v19 = (unsigned int)v4[2] | 0x20;
            v16 = v9;
            break;
          case 0x29:
            v19 = (unsigned int)v4[2] | 0x40;
            v16 = v9;
          
        }
        while( true ) {
          *(unsigned int *)&v4[2] = v19;
          v19 = (unsigned int)(unsigned char)*v16;
          v9 = &v16[1];
          if (*v16 != '\'') break;
label_195d1:
          v19 = (unsigned int)v4[2] | 1;
          v16 = v9;
        }
      } while( true );
    }
    v9 = v16;
    do {
      v15 = &v9[1];
      v9 = &v9[1];
    } while ((unsigned char)(*v15 - 0x30U) <= 9);
    v23 = 0xffffffffffffffff;
    if (*v15 != '$') goto label_194ee;
    v23 = 0;
    while( true ) {
      v6 = 0xffffffffffffffff;
      if (v23 <= 0x1999999999999999)
        v6 = v23 * 10;
      v23 = sub_17f90(v6,(long)(char)(v19 - 0x30),v19 - 0x30);
      v19 = (unsigned int)(unsigned char)v16[1];
      v9 = &v16[1];
      if (10 <= (unsigned char)(v16[1] - 0x30U)) break;
      v16 = v9;
    }
    v23 -= 1;
    if (v23 <= 0xfffffffffffffffd) {
      v19 = (unsigned int)(unsigned char)v16[2];
      v16 = &v16[2];
      goto label_194ee;
    }
    goto label_199d0;
  }
  goto label_19441;
label_19518:
  if ((char)v19 != '*') { // branch-flip
    v9 = v16;
    if ((unsigned char)((char)v19 - 0x30U) <= 9) {
      v4[3] = (unsigned long)v16;
      if ((unsigned char)(*v16 - 0x30U) <= 9) {
        do {
          v15 = &v9[1];
          v9 = &v9[1];
        } while ((unsigned char)(*v15 - 0x30U) <= 9);
        v6 = (long)v9 - (long)v16;
        if ((unsigned long)((long)v9 - (long)v16) <= v24)
          v6 = v24;
        v24 = v6;
      }
      v4[4] = (unsigned long)v9;
label_19a4c:
      v19 = (unsigned int)(unsigned char)*v9;
    }
    v17 = (char)v19;
  }
  else {
    v4[3] = (unsigned long)v16;
    v4[4] = (unsigned long)v9;
    v17 = v16[1];
    v6 = 1;
    if (v24)
      v6 = v24;
    if (10 <= (unsigned char)(v17 - 0x30U)) { // branch-flip
label_1975d:
      v4[5] = v26;
      v14 = v26 + 1;
      if (v26 == 0xffffffffffffffff) goto label_199d0;
      v10 = v26;
      v26 = v14;
    }
    else {
      v16 = v9;
      do {
        v15 = &v16[1];
        v16 = &v16[1];
      } while ((unsigned char)(*v15 - 0x30U) <= 9);
      if (*v15 != '$') goto label_1975d;
      v10 = 0;
      while( true ) {
        v14 = 0xffffffffffffffff;
        if (v10 <= 0x1999999999999999)
          v14 = v10 * 10;
        v10 = sub_17f90(v14,(long)(char)(v17 + '\xd0'));
        v17 = v9[1];
        v16 = &v9[1];
        if (10 <= (unsigned char)(v17 - 0x30U)) break;
        v9 = v16;
      }
      v10 -= 1;
      if (0xfffffffffffffffe <= v10) goto label_199d0;
      v4[5] = v10;
      v9 = &v9[2];
    }
    if (v20 <= v10) { // branch-flip
      v20 *= 2;
      if (v20 <= v10)
        v20 = sub_17f90(v10,1);
      v7 = (struct_29 *)a2->field_0x8;
      if (0x800000000000000 <= v20) goto label_19cdd;
      if (v2 != v7) // branch-flip
        v8 = realloc(v7,v20 << 5);
      else {
        v8 = malloc(v20 << 5);
      }
      v7 = (struct_29 *)a2->field_0x8;
      if (!v8) goto label_19cdd;
      if (v2 == v7)
        v8 = memcpy(v8,v2,a2->field_0x0 << 5);
      a2->field_0x8 = &v8->field_0x0;
    }
    else {
      v8 = (struct_29 *)a2->field_0x8;
    }
    v14 = a2->field_0x0;
    v7 = &v8[v14 * 2];
    if (v14 <= v10) {
      do {
        v5 = v7;
        v14 += 1;
        *(unsigned int *)&v5->field_0x0 = 0;
        v7 = &v5[2];
      } while (v14 <= v10);
      a2->field_0x0 = v14;
      *(unsigned int *)&v5->field_0x0 = 0;
    }
    v11 = (int)v8[v10 * 2].field_0x0;
    v24 = v6;
    if (v11) {
      if (v11 != 5) goto label_199d4;
      goto label_19a4c;
    }
    *(unsigned int *)&v8[v10 * 2].field_0x0 = 5;
    v17 = *v9;
    v19 = (unsigned int)(unsigned char)v17;
  }
  v16 = v9;
  if (v17 != '.') goto label_19537;
  if (v9[1] != '*') {
    v4[6] = (unsigned long)v9;
    v15 = &v9[1];
    if (10 <= (unsigned char)(v9[1] - 0x30U)) { // branch-flip
      v6 = 1;
      v16 = v15;
    }
    else {
      do {
        v16 = &v15[1];
        v15 = &v15[1];
      } while ((unsigned char)(*v16 - 0x30U) <= 9);
      v6 = (long)v15 - (long)v9;
      v16 = v15;
    }
    v4[7] = (unsigned long)v15;
    v19 = (unsigned int)(unsigned char)*v15;
    if (v6 <= v25)
      v6 = v25;
    v25 = v6;
    goto label_19537;
  }
  v16 = &v9[2];
  v4[6] = (unsigned long)v9;
  v4[7] = (unsigned long)v16;
  v6 = 2;
  if (2 <= v25)
    v6 = v25;
  v17 = v9[2];
  if (10 <= (unsigned char)(v17 - 0x30U)) { // branch-flip
label_198a5:
    v10 = v4[8];
    if (v4[8] == 0xffffffffffffffff) {
      v4[8] = v26;
      if (v26 == 0xffffffffffffffff) goto label_199d0;
      v10 = v26;
      v26 += 1;
    }
  }
  else {
    v9 = v16;
    do {
      v15 = &v9[1];
      v9 = &v9[1];
    } while ((unsigned char)(*v15 - 0x30U) <= 9);
    if (*v15 != '$') goto label_198a5;
    v10 = 0;
    while( true ) {
      v14 = 0xffffffffffffffff;
      if (v10 <= 0x1999999999999999)
        v14 = v10 * 10;
      v10 = sub_17f90(v14,(long)(char)(v17 + '\xd0'));
      v17 = v16[1];
      v9 = &v16[1];
      if (10 <= (unsigned char)(v17 - 0x30U)) break;
      v16 = v9;
    }
    v10 -= 1;
    if (0xfffffffffffffffe <= v10) goto label_199d0;
    v4[8] = v10;
    v16 = &v16[2];
  }
  if (v20 <= v10) { // branch-flip
    v20 *= 2;
    if (v20 <= v10)
      v20 = sub_17f90(v10,1);
    v7 = (struct_29 *)a2->field_0x8;
    if (0x800000000000000 <= v20) goto label_19cdd;
    if (v2 != v7) // branch-flip
      v8 = realloc(v7,v20 << 5);
    else {
      v8 = malloc(v20 << 5);
    }
    v7 = (struct_29 *)a2->field_0x8;
    if (!v8) goto label_19cdd;
    if (v2 == v7)
      v8 = memcpy(v8,v2,a2->field_0x0 << 5);
    a2->field_0x8 = &v8->field_0x0;
  }
  else {
    v8 = (struct_29 *)a2->field_0x8;
  }
  v14 = a2->field_0x0;
  v7 = &v8[v14 * 2];
  if (v14 <= v10) {
    do {
      v5 = v7;
      v14 += 1;
      *(unsigned int *)&v5->field_0x0 = 0;
      v7 = &v5[2];
    } while (v14 <= v10);
    a2->field_0x0 = v14;
    *(unsigned int *)&v5->field_0x0 = 0;
  }
  v11 = (int)v8[v10 * 2].field_0x0;
  v25 = v6;
  if (!v11) {
    *(unsigned int *)&v8[v10 * 2].field_0x0 = 5;
    v19 = (unsigned int)(unsigned char)*v16;
label_19537:
    v13 = 0;
    v16 = &v16[1];
    v12 = 0;
    if ((char)v19 == 'h') goto label_195a0;
    do {
      v18 = (char)v19;
      switch(v18) {
        case 0x4c:
          v13 = v12 | 4;
          break;
        default:
          goto label_19568;
        case 0x5a:
        case 0x6a:
        case 0x6c:
        case 0x74:
        case 0x7a:
          v13 = v12 + 8;
        
      }
      while( true ) {
        v17 = *v16;
        v19 = (unsigned int)(unsigned char)v17;
        v16 = &v16[1];
        v12 = v13;
        if (v17 != 'h') break;
label_195a0:
        v13 |= 1 << ((unsigned char)v13 & 1);
      }
    } while( true );
  }
  if (v11 == 5) {
    v19 = (unsigned int)(unsigned char)*v16;
    goto label_19537;
  }
  goto label_199d4;
label_19568:
  switch(v18) {
    case 0x25:
      goto label_19686;
    default:
      goto label_199d0;
    case 0x41:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x61:
    case 0x65:
    case 0x66:
    case 0x67:
      v11 = 0xc;
      if (v12 <= 0xf)
        v11 = ((v12 & 4) != 0) + 0xb;
      break;
    case 0x43:
      v11 = 0xe;
      v18 = 99;
      break;
    case 0x53:
      v11 = 0x10;
      v18 = 0x73;
      break;
    case 0x58:
    case 0x6f:
    case 0x75:
    case 0x78:
      v11 = 10;
      if (((v12 <= 0xf) && (!(v12 & 4))) && ((v11 = 8, v12 <= 7 && (v11 = 2, !(v12 & 2)))))
        v11 = (-(unsigned int)((v12 & 1) == 0) & 2) + 4;
      break;
    case 99:
      v11 = (7 < v12) + 0xd;
      break;
    case 100:
    case 0x69:
      v11 = 9;
      if (((v12 <= 0xf) && (!(v12 & 4))) && ((v11 = 7, v12 <= 7 && (v11 = 1, !(v12 & 2)))))
        v11 = (-(unsigned int)((v12 & 1) == 0) & 2) + 3;
      break;
    case 0x6e:
      v11 = 0x16;
      if ((((v12 <= 0xf) && (!(v12 & 4))) && (v11 = 0x15, v12 <= 7)) && (v11 = 0x12, !(v12 & 2)))
        v11 = 0x14 - (v12 & 1);
      break;
    case 0x70:
      v11 = 0x11;
      break;
    case 0x73:
      v11 = (7 < v12) + 0xf;
    
  }
  if (v23 != 0xffffffffffffffff) { // branch-flip
    v4[10] = v23;
    v6 = v26;
  }
  else {
    v4[10] = v26;
    v6 = v26 + 1;
    v23 = v26;
    if (v26 == 0xffffffffffffffff) {
label_199d0:
      v8 = (struct_29 *)a2->field_0x8;
      goto label_199d4;
    }
  }
  v26 = v6;
  if (v20 <= v23) { // branch-flip
    v20 *= 2;
    if (v20 <= v23)
      v20 = sub_17f90(v23,1);
    v7 = (struct_29 *)a2->field_0x8;
    if (0x800000000000000 <= v20) goto label_19cdd;
    if (v2 != v7) // branch-flip
      v8 = realloc(v7,v20 << 5);
    else {
      v8 = malloc(v20 << 5);
    }
    v7 = (struct_29 *)a2->field_0x8;
    if (!v8) goto label_19cdd;
    if (v2 == v7)
      v8 = memcpy(v8,v2,a2->field_0x0 << 5);
    a2->field_0x8 = &v8->field_0x0;
  }
  else {
    v8 = (struct_29 *)a2->field_0x8;
  }
  v6 = a2->field_0x0;
  v7 = &v8[v6 * 2];
  if (v6 <= v23) {
    do {
      v5 = v7;
      v6 += 1;
      *(unsigned int *)&v5->field_0x0 = 0;
      v7 = &v5[2];
    } while (v6 <= v23);
    a2->field_0x0 = v6;
    *(unsigned int *)&v5->field_0x0 = 0;
  }
  v3 = (int)v8[v23 * 2].field_0x0;
  if (v3) { // branch-flip
    if (v3 != v11) {
label_199d4:
      if (v2 != v8)
        free(v8);
      if (v1 != (unsigned long *)a1[1])
        free((unsigned long *)a1[1]);
      *__errno_location() = 0x16;
      return 0xffffffff;
    }
  }
  else {
    *(int *)&v8[v23 * 2].field_0x0 = v11;
  }
label_19686:
  *(char *)&v4[9] = v18;
  v6 = *a1;
  v4[1] = (unsigned long)v16;
  v6 += 1;
  *a1 = v6;
  if (v6 >= v22) {
    if ((0 <= (long)v22) && (v21 = v22 * 2, v21 <= 0x2e8ba2e8ba2e8ba)) {
      if (v1 != (unsigned long *)a1[1]) // branch-flip
        v4 = realloc((unsigned long *)a1[1],v22 * 0xb0);
      else {
        v4 = malloc(v22 * 0xb0);
      }
      if (v4) {
        if (v1 == (unsigned long *)a1[1])
          v4 = memcpy(v4,v1,*a1 * 0x58);
        a1[1] = (unsigned long)v4;
        v6 = *a1;
        goto label_19441;
      }
    }
    v7 = (struct_29 *)a2->field_0x8;
label_19cdd:
    if (v2 != v7)
      free(v7);
    if (v1 != (unsigned long *)a1[1])
      free((unsigned long *)a1[1]);
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  v4 = (unsigned long *)a1[1];
label_19441:
  a0 = v16;
  v22 = v21;
  goto label_19444;
}

// Function: sub_1a150 @ 0x1a150
void sub_1a150(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_25008); // tail-call
}

// Function: _DT_FINI @ 0x1a164
void _DT_FINI(void)
{
  return;
}
