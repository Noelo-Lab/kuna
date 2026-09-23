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
void fgetfilecon(void)
{
  (*dat_24bd0)(); // jump-as-call
}

// Function: __snprintf_chk @ 0x38a0
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_24bd8)(); // jump-as-call
  return v1;
}

// Function: mkfifoat @ 0x38b0
void mkfifoat(void)
{
  (*dat_24be0)(); // jump-as-call
}

// Function: utimensat @ 0x38c0
void utimensat(void)
{
  (*dat_24be8)(); // jump-as-call
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
void error_at_line(void)
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
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24c18)(); // jump-as-call
  return v1;
}

// Function: getfscreatecon @ 0x3930
void getfscreatecon(void)
{
  (*dat_24c20)(); // jump-as-call
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
void faccessat(void)
{
  (*dat_24c50)(); // jump-as-call
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
int clock_gettime(int a0,void *a1)
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
void pathconf(void)
{
  (*dat_24c80)(); // jump-as-call
}

// Function: fclose @ 0x3a00
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_24c88)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x3a10
void * opendir(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24c90)(); // jump-as-call
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
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_24cd8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3ab0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24ce0)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x3ac0
void freecon(void)
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
int fputs(char *a0,void *a1)
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
void copy_file_range(void)
{
  (*dat_24d40)(); // jump-as-call
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
void rewinddir(void)
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
int closedir(void *a0)
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
void getcon(void)
{
  (*dat_24d80)(); // jump-as-call
}

// Function: lsetfilecon @ 0x3c00
void lsetfilecon(void)
{
  (*dat_24d88)(); // jump-as-call
}

// Function: read @ 0x3c10
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_24d90)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x3c20
int lstat(char *a0,void *a1)
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
int fputs_unlocked(char *a0,void *a1)
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
int ferror_unlocked(void *a0)
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
void readlinkat(void)
{
  (*dat_24dd8)(); // jump-as-call
}

// Function: dirfd @ 0x3cb0
int dirfd(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_24de0)(); // jump-as-call
  return v1;
}

// Function: fpathconf @ 0x3cc0
void fpathconf(void)
{
  (*dat_24de8)(); // jump-as-call
}

// Function: mknodat @ 0x3cd0
void mknodat(void)
{
  (*dat_24df0)(); // jump-as-call
}

// Function: rpmatch @ 0x3ce0
void rpmatch(void)
{
  (*dat_24df8)(); // jump-as-call
}

// Function: mkdirat @ 0x3cf0
void mkdirat(void)
{
  (*dat_24e00)(); // jump-as-call
}

// Function: umask @ 0x3d00
unsigned int umask(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_24e08)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3d10
int stat(char *a0,void *a1)
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
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e28)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x3d50
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e30)(); // jump-as-call
  return v1;
}

// Function: context_new @ 0x3d60
void context_new(void)
{
  (*dat_24e38)(); // jump-as-call
}

// Function: putc_unlocked @ 0x3d70
int putc_unlocked(int a0,void *a1)
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
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e50)(); // jump-as-call
  return v1;
}

// Function: fchmodat @ 0x3da0
void fchmodat(void)
{
  (*dat_24e58)(); // jump-as-call
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
void is_selinux_enabled(void)
{
  (*dat_24e70)(); // jump-as-call
}

// Function: renameat2 @ 0x3de0
void renameat2(void)
{
  (*dat_24e78)(); // jump-as-call
}

// Function: attr_copy_fd @ 0x3df0
void attr_copy_fd(void)
{
  (*dat_24e80)(); // jump-as-call
}

// Function: __freading @ 0x3e00
int __freading(void *a0)
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
void linkat(void)
{
  (*dat_24ea0)(); // jump-as-call
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
void chmod(void)
{
  (*dat_24ec8)(); // jump-as-call
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
void fseeko(void)
{
  (*dat_24f00)(); // jump-as-call
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
void * fdopendir(int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24f18)(); // jump-as-call
  return v1;
}

// Function: futimens @ 0x3f30
void futimens(void)
{
  (*dat_24f20)(); // jump-as-call
}

// Function: fstatfs @ 0x3f40
void fstatfs(void)
{
  (*dat_24f28)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x3f50
void __cxa_atexit(void)
{
  (*dat_24f30)(); // jump-as-call
}

// Function: getline @ 0x3f60
long getline(char **a0,void *a1,void *a2)
{
  long v1; // rax
  
  v1 = (*dat_24f38)(); // jump-as-call
  return v1;
}

// Function: fchownat @ 0x3f70
void fchownat(void)
{
  (*dat_24f40)(); // jump-as-call
}

// Function: renameat @ 0x3f80
void renameat(void)
{
  (*dat_24f48)(); // jump-as-call
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
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_24f60)(); // jump-as-call
  return v1;
}

// Function: fsetfilecon @ 0x3fc0
void fsetfilecon(void)
{
  (*dat_24f68)(); // jump-as-call
}

// Function: getrandom @ 0x3fd0
void getrandom(void)
{
  (*dat_24f70)(); // jump-as-call
}

// Function: getfilecon @ 0x3fe0
void getfilecon(void)
{
  (*dat_24f78)(); // jump-as-call
}

// Function: aligned_alloc @ 0x3ff0
void aligned_alloc(void)
{
  (*dat_24f80)(); // jump-as-call
}

// Function: context_free @ 0x4000
void context_free(void)
{
  (*dat_24f88)(); // jump-as-call
}

// Function: mbsinit @ 0x4010
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_24f90)(); // jump-as-call
  return v1;
}

// Function: setfscreatecon @ 0x4020
void setfscreatecon(void)
{
  (*dat_24f98)(); // jump-as-call
}

// Function: lgetfilecon @ 0x4030
void lgetfilecon(void)
{
  (*dat_24fa0)(); // jump-as-call
}

// Function: symlinkat @ 0x4040
void symlinkat(void)
{
  (*dat_24fa8)(); // jump-as-call
}

// Function: iswprint @ 0x4050
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_24fb0)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x4060
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_24fb8)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x4070
int fstatat(int a0,char *a1,void *a2,int a3)
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

// Function: sub_40b0 @ 0x40b0
unsigned long sub_40b0(int a0,char **a1)
{
  bool v1;
  unsigned int v10; // stack - 0x98
  long v11; // stack - 0xc8
  long v12; // stack - 0xc0
  long v13; // stack - 0xb8
  long v14; // stack - 0xb0
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
  char *v3; // rax
  char v30; // stack - 0x66
  char v31; // stack - 0x65
  char v32; // stack - 0x64
  char v33; // stack - 0x63
  unsigned short v34; // stack - 0x61
  char v35; // stack - 0x5e
  char v36; // stack - 0x5d
  char v37; // stack - 0x5c
  int v38; // stack - 0x54
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  long v9;
  
  sub_127f0(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_1a150(sub_e560);
  dat_2515a = 0 < (int)is_selinux_enabled();
  sub_49f0(&v10);
  v15 = 0;
  v14 = 0;
  v11 = 0;
  v16 = 0;
  v12 = 0;
  v13 = 0;
  v1 = 0;
label_4188:
  v2 = getopt_long(a0,a1,"abdfHilLnprst:uvxPRS:TZ",(void *)0x244a0,NULL);
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
              v3 = dcgettext(NULL,"warning: ignoring --context",5);
              error(0,*__errno_location(),v3);
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
        v9 = optarg;
        if (!optarg)
          v9 = v12;
        v12 = v9;
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
          return v6;
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
        if (optarg) { // branch-flip
          v9 = sub_d220("--reflink",optarg,0x24860,0x1cad0,4,dat_25028,1);
          v38 = *(int *)(v9 * 4 + 0x1cad0);
        }
        else {
          v38 = 2;
        }
        break;
      case 0x86:
        v9 = sub_d220("--sparse",optarg,0x24880,0x1cae0,4,dat_25028,1);
        v19 = *(int *)(v9 * 4 + 0x1cae0);
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
    v3 = "cannot make both hard and symbolic links";
    goto label_4754;
  }
  if (v18 != 2) { // branch-flip
    if (!v1) goto label_4679;
    if ((v38 == 2) && (v19 != 2)) goto label_4748;
    v3 = dcgettext(NULL,"backup type",5);
    v10 = sub_db20(v3,v12);
  }
  else {
    v36 = 0;
    if (v1) {
      v3 = "options --backup and --no-clobber are mutually exclusive";
      goto label_4754;
    }
label_4679:
    v10 = 0;
    if ((v38 == 2) && (v10 = 0, v19 != 2)) {
label_4748:
      v3 = "--reflink can be used only with --sparse=auto";
label_4754:
      error(0,0,dcgettext(NULL,v3,5));
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
        return v5;
      }
      if (!dat_2515a) {
        error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5)); // return-dupe
        return v4;
      }
    }
    else {
      v31 = '\0';
    }
    if ((v14) && ((int)setfscreatecon(v14) < 0)) {
      v7 = sub_14e00(v14);
      v3 = dcgettext(NULL,"failed to set default file creation context to %s",5);
      error(1,*__errno_location(),v3,v7);
      return v8;
    }
  }
  else if ((v32) && (!dat_2515a)) {
    error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5));
    return v4;
  }
  sub_bfe0();
  v9 = (long)optind;
  a0 -= optind;
  return (unsigned long)((sub_5ae0(a0,&a1[v9],v11,v15,&v10) ^ 1) & 0xff);
label_4220:
  v21 = 1;
  goto label_4188;
}

// Function: sub_4880 @ 0x4880
void sub_4880(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_24fd8)(sub_40b0,v2,&Stack0000000000000008,0,0,a2,v1);
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
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_49a0 @ 0x49a0
void sub_49a0(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nThe backup suffix is \'~\', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n",5),v1);
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n",5),v1); // tail-call
}

// Function: sub_49f0 @ 0x49f0
void sub_49f0(long a0)
{
  char *v1; // rax
  
  sub_83b0(a0);
  *(unsigned int *)(a0 + 0x44) = 1;
  *(unsigned long *)(a0 + 8) = 0x200000004;
  *(unsigned long *)(a0 + 0x10) = 0x100000000;
  *(unsigned int *)(a0 + 0x1c) = 0;
  *(char *)(a0 + 0x20) = 0;
  *(unsigned int *)(a0 + 4) = 1;
  *(unsigned short *)(a0 + 0x18) = 0;
  *(unsigned long *)(a0 + 0x28) = 0;
  *(unsigned long *)(a0 + 0x30) = 0x100;
  *(unsigned int *)(a0 + 0x38) = 0;
  *(unsigned short *)(a0 + 0x3c) = 0;
  v1 = getenv("POSIXLY_CORRECT");
  *(unsigned long *)(a0 + 0x48) = 0;
  *(unsigned long *)(a0 + 0x50) = 0;
  *(bool *)(a0 + 0x3e) = v1 != NULL;
}

// Function: sub_4a80 @ 0x4a80
void sub_4a80(unsigned long a0,unsigned long a1,char a2)
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
  v4 = sub_d220(v2,v1,0x24460,0x1cab0,4,dat_25028);
  if (*(unsigned int *)(v4 * 4 + 0x1cab0) <= 6) {
    (*(void *)((long)*(int *)((unsigned long)*(unsigned int *)(v4 * 4 + 0x1cab0) * 4 + 0x1c990) + 0x1c990))(); // jump-as-call
    return;
  }
  abort(); // no-return
}

// Function: sub_4bf0 @ 0x4bf0
unsigned long sub_4bf0(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,long a6) // return-dupe x2
{
  unsigned int v1;
  int v10;
  unsigned int v11;
  unsigned long v12; // rax
  unsigned long v13; // rax
  void *v14;
  char *v15;
  unsigned long *v16;
  unsigned long v17; // rax
  unsigned int v18; // ecx
  unsigned long v19; // rdx
  long v2;
  unsigned long *v20; // stack - 0x1a8
  char v21 [24];
  unsigned long v22; // stack - 0xd8
  char *v23; // rsp
  unsigned long *v24; // rsp
  char *v25; // rsp
  char *v26;
  char *v27; // rsp
  char *v28;
  unsigned int v29; // r13d
  char v3;
  char *v30; // stack - 0x1a0
  unsigned long *v31; // stack - 0x198
  long *v32; // stack - 0x190
  char *v33; // stack - 0x188
  char *v34; // stack - 0x180
  char *v35; // stack - 0x178
  void *v36; // stack - 0x170
  unsigned int v37; // stack - 0x150
  unsigned long v38; // stack - 0xd0
  unsigned long v39; // stack - 0xc8
  long v4;
  unsigned int v40; // stack - 0xc0
  unsigned int v41; // stack - 0xbc
  unsigned long v42; // stack - 0xb8
  unsigned long v43; // stack - 0xb0
  unsigned long v44; // stack - 0xa8
  unsigned long v45; // stack - 0xa0
  unsigned long v46; // stack - 0x98
  unsigned long v47; // stack - 0x90
  unsigned long v48; // stack - 0x88
  unsigned long v49; // stack - 0x80
  unsigned long *v5;
  unsigned long v50; // stack - 0x78
  unsigned long v51; // stack - 0x70
  unsigned long v52; // stack - 0x68
  unsigned long v53; // stack - 0x60
  unsigned long v54; // stack - 0x58
  unsigned long v55; // stack - 0x50
  char *v6;
  long v7;
  char v8;
  int v9;
  
  v7 = a6;
  v24 = &v20;
  v16 = &v20;
  v30 = a3;
  v32 = a4;
  v34 = a5;
  v12 = sub_e750(a0);
  *v32 = 0;
  if (v12 <= a1)
    return 1;
  v13 = strlen(a0);
  v5 = &v20;
  while (v24 != (unsigned long *)((long)&v20 - (v13 + 0x18 & 0xfffffffffffff000))) {
    v16 = (char *)((long)v5 + -0x1000);
    v23 = (char *)((long)v5 + -0x1000);
    *(unsigned long *)((long)v5 + -8) = *(unsigned long *)((long)v5 + -8);
    v24 = (char *)((long)v5 + -0x1000);
    v5 = (unsigned long *)v23;
  }
  v19 = (unsigned long)((unsigned int)(v13 + 0x18) & 0xff0);
  v4 = -v19;
  v25 = (char *)((long)v16 + v4);
  v26 = (char *)((long)v16 + v4);
  if (v19)
    *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
  *(unsigned long *)((long)v16 + v4 + -8) = 0x4ce5;
  v36 = memcpy((void *)((unsigned long)((long)v16 + v4 + 0xf) & 0xfffffffffffffff0),a0,v13 + 1,*(char *)((long)v16 + v4 + -8));
  v6 = (char *)((long)v16 + v4);
  while (v25 != (char *)((long)v16 + (v4 - (v12 + 0x18 & 0xfffffffffffff000)))) {
    v26 = &v6[-0x1000];
    v27 = &v6[-0x1000];
    *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
    v25 = &v6[-0x1000];
    v6 = v27;
  }
  v13 = (unsigned long)((unsigned int)(v12 + 0x18) & 0xff0);
  v4 = -v13;
  if (v13)
    *(unsigned long *)&v26[-8] = *(unsigned long *)&v26[-8];
  v14 = (void *)((unsigned long)&v26[v4 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v26[v4 + -8] = 0x4d4f;
  memcpy(v14,v36,v12,v26[v4 + -8]);
  v28 = (char *)((long)v14 + a1);
  *(char *)((long)v14 + v12) = 0;
  v8 = *v28;
  while (v8 == '/') {
    v28 = &v28[1];
    v8 = *v28;
  }
  v33 = v21;
  *(unsigned long *)&v26[v4 + -8] = 0x4d84;
  v3 = v26[v4 + -8];
  if (!fstatat(a2,v28,v21,0,v3)) {
    if ((v37 & 0xf000) == 0x4000) {
      *v34 = '\0';
      return 1;
    }
label_4c55:
    *(unsigned long *)&v26[v4 + -8] = 0x5206;
    v17 = sub_14b40(4,v14);
    *(unsigned long *)&v26[v4 + -8] = 0x521c;
    v28 = dcgettext(NULL,"%s exists but is not a directory",5,v26[v4 + -8]);
    *(unsigned long *)&v26[v4 + -8] = 0x522d;
    error(0,0,v28,v17);
    return 0;
  }
  v28 = (char *)(a1 + (long)v36);
  v35 = v28;
  if (*v28 != '/') // branch-flip
    v28 = v35;
  else {
    do {
      v28 = &v28[1];
    } while (*v28 == '/');
  }
  v20 = &v22;
  v15 = v28;
  do {
    *(unsigned long *)&v26[v4 + -8] = 0x4dda;
    v15 = strchr(v15,0x2f,v26[v4 + -8]);
    if (!v15)
      return 1;
    *v15 = '\0';
    *(unsigned long *)&v26[v4 + -8] = 0x4dfd;
    v9 = fstatat(a2,v28,v33,0,v26[v4 + -8]);
    if ((v9) || (*(unsigned int *)(v7 + 0x1c) & 0xffffff00)) {
      *(unsigned long *)&v26[v4 + -8] = 0x4ed3;
      v10 = stat(v35,v20,v26[v4 + -8]);
      if (v10) { // branch-flip
        *(unsigned long *)&v26[v4 + -8] = 0x4ee0;
        v3 = v26[v4 + -8];
        v10 = *__errno_location(v3);
        if (v10) {
label_4c55:
          v36 = (void *)CONCAT44(v36._4_4_,v10);
          *(unsigned long *)&v26[v4 + -8] = 0x512c;
          v17 = sub_14b40(4,v35);
          *(unsigned long *)&v26[v4 + -8] = 0x5142;
          v28 = dcgettext(NULL,"failed to get attributes of %s",5,v26[v4 + -8]);
          *(unsigned long *)&v26[v4 + -8] = 0x5157;
          error(0,(int)v36,v28,v17);
          return 0;
        }
      }
      else if ((v40 & 0xf000) != 0x4000) {
        v10 = 0x14;
        goto label_4c55;
      }
      *(unsigned long *)&v26[v4 + -8] = 0x4ef6;
      v16 = (unsigned long *)sub_16e90(0xa8);
      *v16 = v22;
      v16[1] = v38;
      v16[2] = v39;
      v16[3] = CONCAT44(v41,v40);
      v16[4] = v42;
      v16[5] = v43;
      v16[6] = v44;
      v16[7] = v45;
      v16[8] = v46;
      v16[9] = v47;
      v16[10] = v48;
      v16[0xb] = v49;
      v16[0xc] = v50;
      v16[0xd] = v51;
      v16[0xe] = v52;
      v16[0xf] = v53;
      v16[0x10] = v54;
      v16[0x11] = v55;
      v16[0x13] = (long)v15 - (long)v36;
      v2 = *v32;
      *(char *)&v16[0x12] = 0;
      v16[0x14] = v2;
      *v32 = (long)v16;
      if (!v9) goto label_4e17;
      v1 = *(unsigned int *)&v16[3];
      v31 = v16;
      *(unsigned long *)&v26[v4 + -8] = 0x4fb7;
      v8 = sub_7ff0(v35,v36,v1,1,v7);
      if (!v8)
        return 0;
      *v34 = '\x01';
      v11 = *(unsigned int *)&v31[3];
      if (*(char *)(v7 + 0x1d)) { // branch-flip
        v29 = v11 & 0x3f;
        v18 = ~v29;
      }
      else if (*(char *)(v7 + 0x1e)) { // branch-flip
        v29 = v11 & 0x12;
        v18 = ~v29;
      }
      else {
        v18 = 0xffffffff;
        v29 = 0;
      }
      if (*(char *)(v7 + 0x20))
        v11 = 0x1ff;
      *(unsigned long *)&v26[v4 + -8] = 0x5014;
      if (mkdirat(a2,v28,v11 & v18 & 0xfff)) {
        *(unsigned long *)&v26[v4 + -8] = 0x52a4;
        v17 = sub_14b40(4,v36);
        v28 = "cannot make directory %s";
        goto label_4c55;
      }
      if (v30) {
        *(unsigned long *)&v26[v4 + -8] = 0x5050;
        __printf_chk(1,v30,v35,v36);
      }
      *(unsigned long *)&v26[v4 + -8] = 0x5075;
      v9 = fstatat(a2,v28,v33,0x100,v26[v4 + -8]);
      if (v9) {
        *(unsigned long *)&v26[v4 + -8] = 0x5251;
        v17 = sub_14b40(4,v36);
        v28 = "failed to get attributes of %s";
        goto label_4c55;
      }
      v11 = v37;
      if (!*(char *)(v7 + 0x1e)) {
        if (~v37 & v29) { // branch-flip
          *(unsigned long *)&v26[v4 + -8] = 0x51b1;
          v29 &= ~sub_8640();
          v16 = v31;
          v11 = v37;
          if (!(~v37 & v29)) goto label_509f;
        }
        else {
label_509f:
          v16 = v31;
          v11 = v37;
          if ((v37 & 0x1c0) == 0x1c0) goto label_50be;
        }
        *(char *)&v16[0x12] = 1;
        *(unsigned int *)&v16[3] = v29 | v11;
      }
label_50be:
      if ((v11 | 0x1c0) != v11) {
        *(unsigned long *)&v26[v4 + -8] = 0x50d5;
        if (sub_e520(a2,v28)) {
          *(unsigned long *)&v26[v4 + -8] = 0x52c6;
          v17 = sub_14b40(4,v36);
          v28 = "setting permissions for %s";
label_4c55:
          *(unsigned long *)&v26[v4 + -8] = 0x5267;
          v28 = dcgettext(NULL,v28,5,v26[v4 + -8]);
          *(unsigned long *)&v26[v4 + -8] = 0x526f;
          v3 = v26[v4 + -8];
          v9 = *__errno_location(v3);
          *(unsigned long *)&v26[v4 + -8] = 0x5280;
          error(0,v9,v28,v17);
          return 0;
        }
      }
      if (!*v34) goto label_4e59;
    }
    else {
label_4e17:
      *(unsigned long *)&v26[v4 + -8] = 0x4e31;
      v8 = sub_7ff0(v35,v36,0,0,v7);
      if (!v8)
        return 0;
      if ((v37 & 0xf000) != 0x4000) {
        v14 = v36;
        goto label_4c55;
      }
      *v34 = '\0';
label_4e59:
      if ((*(long *)(v7 + 0x28)) || (*(char *)(v7 + 0x33))) {
        *(unsigned long *)&v26[v4 + -8] = 0x4e76;
        v8 = sub_8250(v36,0,v7);
        if ((!v8) && (*(char *)(v7 + 0x34)))
          return 0;
      }
    }
    v8 = v15[1];
    *v15 = '/';
    v15 = &v15[1];
    while (v8 == '/') {
      v15 = &v15[1];
      v8 = *v15;
    }
  } while( true );
}

// Function: sub_52e0 @ 0x52e0
unsigned long sub_52e0(char *a0,unsigned int a1,unsigned long a2,long a3,long a4) // return-dupe
{
  long v1;
  void *v10; // rax
  unsigned long v11; // rax
  char *v12; // rax
  unsigned long v13; // rdx
  unsigned long *v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long *v16;
  char *v17; // rsp
  unsigned long *v18; // rsp
  char v19 [16];
  char v2;
  unsigned long v20; // stack - 0x70
  unsigned long v21; // stack - 0x60
  unsigned long v22; // stack - 0x58
  unsigned long v23; // stack - 0x50
  unsigned int v3;
  unsigned int v4;
  char v5;
  long v6;
  unsigned long *v7;
  int v8;
  unsigned long v9; // rax
  
  v16 = &v14;
  v20 = a2;
  v9 = strlen(a0);
  v18 = &v14;
  v7 = &v14;
  while (v18 != (unsigned long *)((long)&v14 - (v9 + 0x18 & 0xfffffffffffff000))) {
    v16 = (char *)((long)v7 + -0x1000);
    v17 = (char *)((long)v7 + -0x1000);
    *(unsigned long *)((long)v7 + -8) = *(unsigned long *)((long)v7 + -8);
    v18 = (char *)((long)v7 + -0x1000);
    v7 = (unsigned long *)v17;
  }
  v13 = (unsigned long)((unsigned int)(v9 + 0x18) & 0xff0);
  v6 = -v13;
  if (v13)
    *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
  *(unsigned long *)((long)v16 + v6 + -8) = 0x5370;
  v10 = memcpy((void *)((unsigned long)((long)v16 + v6 + 0xf) & 0xfffffffffffffff0),a0,v9 + 1,*(char *)((long)v16 + v6 + -8));
  v1 = (long)v10 + (v20 - (long)a0);
  if (!a3)
    return 1;
  v14 = &v15;
  do {
    v2 = *(char *)(a4 + 0x1f);
    *(char *)((long)v10 + *(long *)(a3 + 0x98)) = 0;
    if (v2) {
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53f5;
      v19 = sub_15960(a3);
      v21 = SUB168(v19,8);
      v15 = SUB168(v19,0);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5405;
      v19 = sub_15980(a3);
      v23 = SUB168(v19,8);
      v22 = SUB168(v19,0);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x541e;
      v8 = utimensat(a1,v1,v14,0);
      if (v8) {
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5433;
        v11 = sub_14b40(4,v10);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5449;
        v12 = dcgettext(NULL,"failed to preserve times for %s",5,*(char *)((long)v16 + v6 + -8));
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5451;
        v5 = *(char *)((long)v16 + v6 + -8);
        v8 = *__errno_location(v5);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5462;
        error(0,v8,v12,v11);
        return 0;
      }
    }
    if (*(char *)(a4 + 0x1d)) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x20);
      v3 = *(unsigned int *)(a3 + 0x1c);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5481;
      if (!sub_e540(a1,v1,v3,v4)) goto label_53a3;
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5491;
      if (!sub_8400(a4)) {
        v20 &= 0xffffffffffffff00;
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5592;
        v11 = sub_14b40(4,v10);
        v12 = "failed to preserve ownership for %s";
        goto label_5526;
      }
      v4 = *(unsigned int *)(a3 + 0x20);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x54ac;
      sub_e540(a1,v1,0xffffffff,v4);
      v2 = *(char *)(a4 + 0x1e);
    }
    else {
label_53a3:
      v2 = *(char *)(a4 + 0x1e);
    }
    if (v2) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x18);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x54d9;
      if (sub_caa0(v1,0xffffffff,v10,0xffffffff,v4))
        return 0;
    }
    else if (*(char *)(a3 + 0x90)) {
      v4 = *(unsigned int *)(a3 + 0x18);
      v20 &= 0xffffffffffffff00;
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5502;
      if (sub_e520(a1,v1,v4)) {
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5517;
        v11 = sub_14b40(4,v10);
        v12 = "failed to preserve permissions for %s";
label_5526:
        *(unsigned long *)((long)v16 + v6 + -8) = 0x552d;
        v12 = dcgettext(NULL,v12,5,*(char *)((long)v16 + v6 + -8));
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5535;
        v5 = *(char *)((long)v16 + v6 + -8);
        v8 = *__errno_location(v5);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5546;
        error(0,v8,v12,v11);
        return v20 & 0xff;
      }
    }
    *(char *)((long)v10 + *(long *)(a3 + 0x98)) = 0x2f;
    a3 = *(long *)(a3 + 0xa0);
    if (!a3)
      return 1;
  } while( true );
}

// Function: sub_55b0 @ 0x55b0
void sub_55b0(void)
{
  void *v1;
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
  void *v1;
  unsigned long v2;
  
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
unsigned long sub_5ae0(int a0,unsigned long *a1,long a2,char a3,int *a4)
{
  int *v1;
  char v10; // al
  char v11;
  unsigned int v12; // eax
  unsigned int v13; // eax
  char *v14;
  unsigned long v15; // rax
  void *v16; // rax
  unsigned long v17; // rax
  unsigned long v18; // rax
  unsigned long v19; // rax
  int *v2;
  char *v20;
  unsigned long v21; // rax
  char *v22; // rcx
  unsigned long v23; // rdx
  long v24;
  unsigned char *v25; // stack - 0x128
  char v26 [24];
  char *v27; // stack - 0xe0
  long v28;
  unsigned char v29; // stack - 0xea
  void *v3;
  void *v30; // stack - 0xe8
  char *v31;
  char *v32; // rsp
  unsigned char **v33;
  char *v34; // rsp
  int *v35;
  int *v36;
  long v37;
  unsigned int v38; // eax
  char **v39; // stack - 0x120
  char v4;
  char *v40; // stack - 0x118
  long v41; // stack - 0x110
  int *v42; // stack - 0x108
  unsigned int v43; // stack - 0x100
  int v44; // stack - 0xfc
  unsigned int v45; // stack - 0xc0
  long v5;
  char *v6;
  void *v7;
  int v8;
  unsigned char v9;
  
  v33 = &v25;
  v29 = 0;
  v41 = a2; // branch-flip
  v42 = a4;
  v44 = a0;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (a0 == 1) {
      v19 = sub_14b40(4,*a1);
      v20 = "missing destination file operand after %s";
label_60c1:
      error(0,0,dcgettext(NULL,v20,5),v19);
      sub_57e0(1); // return-dupe, no-return
    }
    v20 = "missing file operand";
  }
  else {
    v45 = 0;
    if (a3) { // branch-flip
      if (a2) {
        error(1,0,dcgettext(NULL,"cannot combine --target-directory (-t) and --no-target-directory (-T)",5));
        return v18;
      }
      if (2 < a0) {
        v19 = sub_14b40(4,a1[2]);
        v20 = "extra operand %s";
        goto label_60c1;
      }
    }
    else {
      if (a2) {
        v12 = sub_15a20(a2,v26);
        v43 = v12;
        if (!sub_15a10(v12)) {
          v19 = sub_14b40(4,v41);
          v20 = dcgettext(NULL,"target directory %s",5);
          error(1,*__errno_location(),v20,v19);
          return v21;
        }
        if (v44 != 1) {
label_5f87:
          v35 = v42;
          sub_8330(v42);
          sub_8370(v35);
        }
label_5b81:
        v24 = 0;
        v38 = 1;
        v39 = &v27;
        v40 = &v28;
        v25 = &v29;
        do {
          v20 = (char *)a1[v24];
          v27 = NULL;
          if (dat_25158) { // branch-flip
            *(unsigned long *)((long)v33 + -8) = 0x5e08;
            sub_e800(v20);
            if (!dat_25159) goto label_5be5;
label_5e18:
            *(unsigned long *)((long)v33 + -8) = 0x5e1d;
            v15 = strlen(v20,*(char *)((long)v33 + -8));
            v22 = (char *)((long)v33 + -(v15 + 0x18 & 0xfffffffffffff000));
            v31 = (char *)v33;
            v6 = (char *)v33;
            while (v33 != (unsigned char **)v22) {
              v31 = &v6[-0x1000];
              v34 = &v6[-0x1000];
              *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
              v33 = (char *)&v6[-0x1000];
              v6 = v34;
            }
            v23 = (unsigned long)((unsigned int)(v15 + 0x18) & 0xff0);
            v5 = -v23;
            v33 = (char *)&v31[v5];
            if (v23)
              *(unsigned long *)&v31[-8] = *(unsigned long *)&v31[-8];
            *(unsigned long *)&v31[v5 + -8] = 0x5e7a;
            v16 = memcpy((void *)((unsigned long)&v31[v5 + 0xf] & 0xfffffffffffffff0),v20,v15 + 1,v31[v5 + -8]);
            *(unsigned long *)&v31[v5 + -8] = 0x5e85;
            sub_e800(v16);
            *(unsigned long *)&v31[v5 + -8] = 0x5e9b;
            v37 = v41;
            v16 = (void *)sub_ec90(v41,v16,v39);
            v14 = NULL;
            v8 = v42[0xf];
            *(long *)&v31[v5 + -8] = v37;
            if ((char)v8)
              v14 = "%s -> %s\n";
            *(int **)&v31[v5 + -0x10] = v42;
            *(unsigned long *)&v31[v5 + -0x18] = 0x5ede;
            v10 = sub_4bf0(v16,(long)v27 - (long)v16,v43,v14,&v30,v25);
            v11 = *v27;
            v14 = v27;
            while (v11 == '/') {
              v14 = &v14[1];
              v27 = v14;
              v11 = *v14;
            }
            if (v10) goto label_5c8c;
            v38 = 0;
            v33 = (unsigned char **)&v31[v5];
            v11 = dat_25159;
label_5f1b:
            v7 = v30;
            if (v11) {
              while (v7) {
                v3 = *(void **)((long)v7 + 0xa0);
                v30 = v3;
                *(unsigned long *)((long)v33 + -8) = 0x5f4e;
                free(v7,*(char *)((long)v33 + -8));
                v7 = v3;
              }
            }
          }
          else {
            if (dat_25159) goto label_5e18;
label_5be5:
            *(unsigned long *)((long)v33 + -8) = 0x5bea;
            v14 = (char *)sub_db60(v20);
            *(unsigned long *)((long)v33 + -8) = 0x5bf5;
            v15 = strlen(v14,*(char *)((long)v33 + -8));
            v22 = (char *)((long)v33 + -(v15 + 0x18 & 0xfffffffffffff000));
            v31 = (char *)v33;
            v6 = (char *)v33;
            while (v33 != (unsigned char **)v22) {
              v31 = &v6[-0x1000];
              v32 = &v6[-0x1000];
              *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
              v33 = (char *)&v6[-0x1000];
              v6 = v32;
            }
            v23 = (unsigned long)((unsigned int)(v15 + 0x18) & 0xff0);
            v5 = -v23;
            v33 = (char *)&v31[v5];
            if (v23)
              *(unsigned long *)&v31[-8] = *(unsigned long *)&v31[-8];
            *(unsigned long *)&v31[v5 + -8] = 0x5c52;
            v14 = memcpy((void *)((unsigned long)&v31[v5 + 0xf] & 0xfffffffffffffff0),v14,v15 + 1,v31[v5 + -8]);
            *(unsigned long *)&v31[v5 + -8] = 0x5c5d;
            sub_e800(v14);
            *(unsigned long *)&v31[v5 + -8] = 0x5c6c;
            v4 = v31[v5 + -8];
            *(unsigned long *)&v31[v5 + -8] = 0x5c89;
            v16 = (void *)sub_ec90(v41,&v14[strcmp(v14,"..",v4) == 0],v39);
label_5c8c:
            *(unsigned long *)((long)v33 + -8) = 0;
            *(char **)((long)v33 + -0x10) = v40;
            *(unsigned long *)((long)v33 + -0x18) = 0x5cbb;
            v13 = sub_bae0(v20,v16,v43,v27,v29,v42);
            v38 &= v13;
            if (dat_25159) {
              *(unsigned long *)((long)v33 + -8) = 0x5f7b;
              v13 = sub_52e0(v16,v43,v27,v30,v42);
              v38 &= v13;
              v11 = dat_25159;
              goto label_5f1b;
            }
          }
          v24 += 1;
          *(unsigned long *)((long)v33 + -8) = 0x5cd9;
          free(v16,*(char *)((long)v33 + -8));
        } while ((int)v24 < v44);
        return (unsigned long)v38; // return-dupe
      }
      v24 = a1[(long)a0 + -1];
      v41 = v24;
      v43 = sub_15a20(v24,v26);
      v38 = sub_15a10(v43);
      if ((char)v38) { // branch-flip
        v44 = a0 + -1;
        if (v24) {
          if (1 < v44) goto label_5f87;
          if (v44 != 1) {
            return (unsigned long)v38;
          }
          goto label_5b81;
        }
      }
      else {
        v8 = *__errno_location();
        if (v8 == 2)
          v29 = 1;
        if (3 <= v44) {
          v19 = sub_14b40(4,v41);
          error(1,v8,dcgettext(NULL,"target %s",5),v19);
          return v17;
        }
      }
    }
    v9 = v29;
    v20 = (char *)*a1;
    v14 = (char *)a1[1];
    if (!dat_25159) {
      if (((((*(char *)((long)v42 + 0x16)) && (*v42)) && (!strcmp(v20,v14))) && ((!v9 && ((v45 || (!stat(v14,v26))))))) && (v35 = v42, (v45 & 0xf000) == 0x8000)) {
        v14 = (char *)sub_daa0(0xffffff9c,v14,*v42);
        v24 = 0x16;
        v36 = (int *)0x25100;
        while (v24) {
          v2 = &v36[1];
          v1 = &v35[1];
          *v36 = *v35;
          v24 -= 1;
          v35 = v1;
          v36 = v2;
        }
        dat_25100 = 0;
        v42 = (int *)0x25100;
      }
      v38 = sub_bae0(v20,v14,0xffffff9c,v14,-(unsigned int)v9,v42,&v27,0);
      return (unsigned long)v38;
    }
    v20 = "with --parents, the destination must be a directory";
  }
  error(0,0,dcgettext(NULL,v20,5));
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
unsigned long sub_61e0(long *a0,long *a1) // return-dupe
{
  if (!a1)
    return 0;
  while ((a1[1] != a0[1] || (a1[2] != *a0))) {
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
int * sub_6230(long a0)
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
    v4 = (unsigned int)*(unsigned char *)(a0 + 0x1b);
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
unsigned long sub_6270(unsigned int *a0)
{
  if (4 <= *a0)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (3 <= a0[3] - 1)
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (2 < a0[0x11])
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((*(char *)((long)a0 + 0x17)) && (*(char *)((long)a0 + 0x3a)))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a0[0x11] == 2) && (a0[3] != 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  return 1;
}

// Function: sub_6360 @ 0x6360
void sub_6360(int a0,char *a1,void *a2,int a3)
{
  fstatat(a0,a1,a2,a3); // tail-call
}

// Function: sub_6370 @ 0x6370
unsigned long sub_6370(unsigned int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  if ((a2 & 0xf000) == 0xa000)
    return 1;
  if (sub_16e20())
    return 1;
  v1 = faccessat(a0,a1,2,0x200);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}

// Function: sub_63e0 @ 0x63e0
void sub_63e0(long a0,unsigned long a1,unsigned int a2,unsigned long a3,long a4)
{
  unsigned int v1;
  unsigned int v2;
  unsigned long v3;
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  char v7 [9];
  char v8; // stack - 0x32
  
  v1 = *(unsigned int *)(a4 + 0x18);
  if (sub_6370(a2,a3,v1)) { // branch-flip
    v4 = sub_14b40(4,a1);
    v3 = dat_255a0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: overwrite %s? ",5),v3,v4);
  }
  else {
    sub_eb60(*(unsigned int *)(a4 + 0x18),&v6);
    v2 = *(unsigned int *)(a4 + 0x18);
    v8 = 0;
    v4 = sub_14b40(4,a1);
    v3 = dat_255a0;
    if ((*(char *)(a0 + 0x18)) || (*(unsigned int *)(a0 + 0x14) & 0xffff00))
      v5 = dcgettext(NULL,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    else {
      v5 = dcgettext(NULL,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    __fprintf_chk(stderr,1,v5,v3,v4,(unsigned long)(v2 & 0xfff),v7);
  }
  sub_17540(); // tail-call
}

// Function: sub_6530 @ 0x6530
unsigned int sub_6530(long a0,unsigned long a1,unsigned int a2,unsigned long a3,long a4) // early-return
{
  int v1;
  unsigned int v2;
  
  if (!*(unsigned char *)(a0 + 0x18))
    __assert_fail("x->move_mode","src/copy.c",0x710,"abandon_move"); // no-return
  v1 = *(int *)(a0 + 8);
  if (v1 == 2)
    return (unsigned int)*(unsigned char *)(a0 + 0x18);
  if ((v1 != 3) && (((v1 != 4 || (!*(char *)(a0 + 0x3d))) || (v2 = *(unsigned int *)(a4 + 0x18), sub_6370(a2,a3,v2)))))
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
bool sub_6640(char *a0,long *a1,int a2,char *a3)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  char *v6;
  long v7; // stack - 0xd8
  bool v8;
  long v9; // stack - 0xd0
  
  v8 = 0;
  v2 = strlen(a0);
  v3 = (char *)sub_db60(a3);
  v4 = strlen(v3);
  v6 = dat_25580;
  v5 = strlen(dat_25580);
  if (((v5 + v4 == v2) && (!memcmp(a0,v3,v4))) && (!strcmp(&a0[v4],v6))) {
    v6 = (char *)sub_65e0(a3,&a3[strlen(a3)],v6);
    v1 = fstatat(a2,v6,&v7,0);
    free(v6);
    v8 = 0;
    if ((!v1) && (a1[1] == v9))
      v8 = *a1 == v7;
  }
  return v8;
}

// Function: sub_6770 @ 0x6770
void sub_6770(int a0,unsigned long a1)
{
  ioctl(a0,0x40049409,a1 & 0xffffffff); // tail-call
}

// Function: sub_6780 @ 0x6780
int sub_6780(unsigned long a0,long a1,long *a2)
{
  int v1;
  long v2;
  long v3;
  int v4; // r12d
  
  v4 = 1;
  if ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000) {
    v3 = *(long *)(a1 + 0x30);
    v2 = v3 + 0x1ff;
    if (0 <= v3)
      v2 = v3;
    if (*(long *)(a1 + 0x40) < v2 >> 9) {
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
unsigned long sub_6910(unsigned long a0,unsigned long a1,char a2,long a3)
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
int * sub_6a40(int a0,unsigned long a1,long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,unsigned long a7,unsigned long a8,unsigned long a9,long *a10,char *a11) // return-dupe x2
{
  bool v1;
  unsigned long v10;
  unsigned long v11;
  char *v12; // rax
  void *v13;
  unsigned long v14;
  int *v15; // rdx
  unsigned long v16;
  int *v17;
  int *v18;
  unsigned int v19; // esi
  long *v2;
  unsigned char v20;
  void *v21;
  unsigned char v22; // r13b
  unsigned long v23;
  unsigned long v24; // stack - 0xb0
  unsigned long v25; // stack - 0x98
  char *v3;
  unsigned int v4;
  unsigned int v5; // eax
  long v6; // rax
  unsigned long v7;
  int *v8; // rax
  unsigned long v9;
  
  v3 = a11;
  v2 = a10;
  v10 = a8;
  v11 = a7;
  v19 = (unsigned int)a1;
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
        v6 = copy_file_range(a0,0,v17,0,v14,0);
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
        v10 = sub_14a70(1,4,v10);
        v11 = sub_14a70(0,4,v11);
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
      v13 = (void *)*a2;
      if (!v13) {
        v13 = (void *)sub_16e60((long)getpagesize(),a3);
        *a2 = (long)v13;
      }
      v7 = v25;
      if (a3 <= v25)
        v7 = a3;
      v7 = read(a0,v13,v7);
      if (0 <= (long)v7) break;
      v8 = __errno_location();
      if (*v8 != 4) {
        v11 = sub_14b40(4,v11);
        v12 = dcgettext(NULL,"error reading %s",5);
        error(0,*v8,v12,v11);
        return NULL;
      }
    }
    if (!v7) {
label_6fc8:
      if ((char)v18)
        return (int *)sub_6910(v19,v10,a5,v23); // tail-call
      return (int *)0x1;
    }
    *v2 = *v2 + v7;
    v9 = v14;
    v21 = v13;
    v24 = v7;
label_6c20:
    do {
      v16 = v9;
      if (v24 < v9)
        v16 = v24;
      v22 = (unsigned char)v18;
      if (v16 && a4) {
        v9 = sub_69e0(v13,v16);
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
          v4 = sub_6910(v19,v10,a5,v23);
          if (!(char)v4)
            return (int *)(unsigned long)v4;
        }
        else if (v23 != sub_11320(v19,v21,v23)) {
          v11 = sub_14b40(4,v10);
          v12 = dcgettext(NULL,"error writing %s",5);
          error(0,*__errno_location(),v12,v11);
          return v18;
        }
        v21 = v13;
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
          v11 = sub_14b40(4,v11);
          error(0,0,dcgettext(NULL,"overflow reading %s",5),v11);
          return NULL;
        }
        v24 -= v16;
        v18 = v17;
      }
      v13 = (void *)((long)v13 + v16);
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
void sub_7020(int a0,unsigned int a1,unsigned long a2,unsigned int a3)
{
  if (a0 <= -1) {
    sub_e520(a1,a2,a3); // tail-call
    return;
  }
  fchmod(a0,a3); // tail-call
}

// Function: sub_7040 @ 0x7040
unsigned long sub_7040(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,long a4)
{
  char v1;
  void *v10; // stack - 0x18
  unsigned int v2; // eax
  unsigned long v3; // rax
  undefined7 v4; // rax
  void *v5; // stack - 0x28
  void *v6; // r8
  void **v7;
  bool v8; // zf
  void *v9; // stack - 0x20
  
  v7 = &v5;
  v1 = *(char *)(a4 + 0x33);
  if ((*(char *)(a4 + 0x31)) && (!*(char *)(a4 + 0x36))) {
    if (*(char *)(a4 + 0x37)) {
      v6 = sub_7330;
      v7 = NULL;
      if (!v1) {
        v6 = NULL;
        v7 = NULL;
        if (*(long *)(a4 + 0x28)) {
          v6 = sub_7330;
          v7 = NULL;
        }
      }
      goto label_70a2;
    }
    v6 = sub_7330;
    v5 = sub_7180;
    if ((!v1) && (!*(long *)(a4 + 0x28)))
      v6 = NULL;
  }
  else {
    v6 = sub_7330;
    v5 = sub_7260;
    if (!v1) {
      v6 = sub_7330;
      if (!*(long *)(a4 + 0x28))
        v6 = NULL;
      v5 = sub_7260;
    }
  }
  v9 = sub_6260;
  v10 = sub_6220;
label_70a2:
  v2 = (unsigned int)a1;
  v2 |= a3;
  if (0 <= (int)v2) { // branch-flip
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
void sub_7260(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x90
  unsigned long v11; // stack - 0x88
  unsigned long v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x38
  unsigned long v17; // stack - 0x28
  unsigned long v18; // stack - 0x18
  unsigned int v2; // stack - 0xd8
  char v3 [16];
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xa8
  unsigned long v8; // stack - 0xa0
  unsigned long v9; // stack - 0x98
  
  if (v1) {
    v11 = a0;
    v12 = a1;
    v13 = a2;
    v14 = a3;
    v15 = a4;
    v16 = a5;
    v17 = a6;
    v18 = a7;
  }
  v7 = a10;
  v8 = a11;
  v9 = a12;
  v10 = a13;
  v5 = &Stack0000000000000008;
  v2 = 0x10;
  v6 = v3;
  v4 = 0x30;
  sub_16760(0,*__errno_location(),a9,&v2);
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
  
  if (!setfscreatecon(0))
    return;
  v1 = dcgettext(NULL,"failed to restore the default file creation context",5);
  error(1,*__errno_location(),v1);
}

// Function: sub_73c0 @ 0x73c0
void sub_73c0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  
  v1 = sub_14a70(1,4,a1);
  __printf_chk(1,"%s -> %s",sub_14a70(0,4,a0),v1);
  if (a2) {
    v1 = sub_14b40(4,a2);
    __printf_chk(1,dcgettext(NULL," (backup: %s)",5),v1);
  }
  putchar_unlocked(10); // tail-call
}

// Function: sub_7460 @ 0x7460
unsigned int sub_7460(void *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned int a4,unsigned long a5,char a6,unsigned int a7,unsigned char a8)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned char v5; // r12b
  unsigned int v6;
  unsigned long v7; // stack - 0x50
  
  v6 = a7;
  v1 = sub_c0c0(a1,a2,a4,a5,(unsigned long)a8 << 10,a6);
  if (1 <= v1) { // branch-flip
    v4 = NULL;
    if (!a0) {
      v4 = (void *)sub_65e0(a3,a5,a2,v7);
      a0 = v4;
    }
    v3 = sub_14a70(1,4,a0);
    v2 = sub_14a70(0,4,a3);
    v6 = 0;
    error(0,v1,dcgettext(NULL,"cannot create hard link %s to %s",5),v2,v3);
    free(v4);
  }
  else {
    v5 = (unsigned char)v6 & (unsigned char)((unsigned int)v1 >> 0x1f);
    v6 = CONCAT31((undefined3)((unsigned int)v6 >> 8),v5);
    if (v5) { // branch-flip
      v3 = sub_14b40(4,a3,0xffffffffffffffff);
      __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v3);
    }
    else {
      v6 = 1;
    }
  }
  return v6;
}

// Function: sub_75b0 @ 0x75b0
unsigned int sub_75b0(char *a0,long *a1,int a2,char *a3,long *a4,int *a5,char *a6) // return-dupe
{
  int v1;
  long *v10;
  long v11; // stack - 0x160
  long v12; // stack - 0x158
  long v13; // stack - 0x150
  long v14; // stack - 0x148
  long v15; // stack - 0x140
  long v16; // stack - 0x138
  long v17; // stack - 0x130
  long v18; // stack - 0x128
  long v19; // stack - 0x120
  bool v2;
  long v20; // stack - 0x118
  long v21; // stack - 0x110
  long v22; // stack - 0x108
  long v23; // stack - 0x100
  long v24; // stack - 0xf8
  long v25; // stack - 0xf0
  long v26; // stack - 0xe8
  long v27; // stack - 0xe0
  long v28; // stack - 0xd0
  long v29; // stack - 0xc8
  bool v3;
  long v30; // stack - 0xc0
  long v31; // stack - 0xb8
  long v32; // stack - 0xb0
  long v33; // stack - 0xa8
  long v34; // stack - 0xa0
  long v35; // stack - 0x98
  long v36; // stack - 0x90
  long v37; // stack - 0x88
  long v38; // stack - 0x80
  long v39; // stack - 0x78
  char *v4;
  long v40; // stack - 0x70
  long v41; // stack - 0x68
  long v42; // stack - 0x60
  long v43; // stack - 0x58
  long v44; // stack - 0x50
  unsigned char v5;
  char *v6; // rax
  char v7;
  long v8; // stack - 0x168
  long v9; // stack - 0xd8
  
  v4 = a6;
  if ((a1[1] != a4[1]) || (*a1 != *a4)) {
    v1 = a5[1];
    *a6 = 0;
    if (v1 != 2)
      return 1;
    v2 = 0;
    v3 = 0;
label_763e:
    v10 = a1;
    if (((*(unsigned int *)&a1[3] & 0xf000) == 0xa000) && ((*(unsigned int *)&a4[3] & 0xf000) == 0xa000)) {
      if (sub_15130(0xffffff9c,a0,a2,a3))
        return 0;
      if (*a5)
        return 1;
      if (!v3)
        return 1;
      *v4 = 1;
      return *(unsigned char *)&a5[6] ^ 1;
    }
  }
  else {
    *a6 = 0;
    if (*(char *)((long)a5 + 0x17)) goto label_760a;
    if (a5[1] == 2) {
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
    v2 = v28 == v11 && v9 == v8;
    if (((((unsigned int)v30 & 0xf000) == 0xa000) && (((unsigned int)v13 & 0xf000) == 0xa000)) && (*(char *)((long)a5 + 0x15)))
      return 1;
  }
  if (*a5) {
    if (v2)
      return sub_15130(0xffffff9c,a0,a2,a3) ^ 1;
    if ((char)a5[6])
      return 1;
    if (a5[1] == 2)
      return 1;
    if ((*(unsigned int *)&v10[3] & 0xf000) != 0xa000)
      return 1;
    if ((*(unsigned int *)&a4[3] & 0xf000) == 0xa000)
      return 1;
    return 0;
  }
  v7 = (char)a5[6];
  if ((v7) || (*(char *)((long)a5 + 0x15))) {
    if ((*(unsigned int *)&a4[3] & 0xf000) == 0xa000)
      return 1;
    if ((v2) && (2 <= (unsigned long)a4[2])) {
      if (!sub_15130(0xffffff9c,a0,a2,a3))
        return *(unsigned char *)&a5[6] ^ 1;
      if ((*(unsigned int *)&v10[3] & 0xf000) != 0xa000) goto label_787c;
      goto label_788f;
    }
    if ((*(unsigned int *)&v10[3] & 0xf000) != 0xa000) goto label_77c9;
  }
  else {
    if ((*(unsigned int *)&v10[3] & 0xf000) == 0xa000) goto label_7900;
label_787c:
    if ((*(unsigned int *)&a4[3] & 0xf000) != 0xa000) {
label_77c9:
      if (v10[1] != a4[1])
        return 1;
      if (*v10 != *a4)
        return 1;
      if (*(char *)((long)a5 + 0x17)) {
label_760a:
        *v4 = 1;
        return 1;
      }
    }
label_788f:
    v7 = (char)a5[6];
  }
  if ((((v7) && ((*(unsigned int *)&a1[3] & 0xf000) == 0xa000)) && (2 <= (unsigned long)a4[2])) && (v6 = canonicalize_file_name(a0), v6)) {
    v5 = sub_15130(0xffffff9c,v6,a2,a3);
    free(v6);
    return (unsigned int)(v5 ^ 1);
  }
label_7900:
  if ((*(char *)((long)a5 + 0x3a)) && ((*(unsigned int *)&a4[3] & 0xf000) == 0xa000))
    return 1;
  if (a5[1] != 2)
    return 0;
  if ((*(unsigned int *)&v10[3] & 0xf000) != 0xa000) { // branch-flip
    v9 = *v10;
    v28 = v10[1];
    v29 = v10[2];
    v30 = v10[3];
    v31 = v10[4];
    v32 = v10[5];
    v33 = v10[6];
    v34 = v10[7];
    v35 = v10[8];
    v36 = v10[9];
    v37 = v10[10];
    v38 = v10[0xb];
    v39 = v10[0xc];
    v40 = v10[0xd];
    v41 = v10[0xe];
    v42 = v10[0xf];
    v43 = v10[0x10];
    v44 = v10[0x11];
  }
  else if (stat(a0,&v9))
    return 1;
  if ((*(unsigned int *)&a4[3] & 0xf000) != 0xa000) { // branch-flip
    v8 = *a4;
    v11 = a4[1];
    v12 = a4[2];
    v13 = a4[3];
    v14 = a4[4];
    v15 = a4[5];
    v16 = a4[6];
    v17 = a4[7];
    v18 = a4[8];
    v19 = a4[9];
    v20 = a4[10];
    v21 = a4[0xb];
    v22 = a4[0xc];
    v23 = a4[0xd];
    v24 = a4[0xe];
    v25 = a4[0xf];
    v26 = a4[0x10];
    v27 = a4[0x11];
  }
  else if (fstatat(a2,a3,&v8,0))
    return 1;
  if (v28 == v11) {
    if (v9 != v8)
      return 1;
    v5 = *(unsigned char *)((long)a5 + 0x17);
    if (!v5)
      return 0;
    *v4 = (*(unsigned int *)&a4[3] & 0xf000) != 0xa000;
    return (unsigned int)v5;
  }
  return 1;
}

// Function: sub_7b50 @ 0x7b50
unsigned long sub_7b50(unsigned int a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,long a5,long a6,int a7,char a8,unsigned long a9,unsigned long a10) // return-dupe x2
{
  bool v1;
  unsigned char v10; // stack - 0x49
  long v11; // stack - 0x48
  char *v12; // rax
  unsigned long v13;
  unsigned char v14; // r13b
  unsigned long v15; // r13
  long v16; // stack - 0xa0
  long v17; // stack - 0x90
  char v2;
  unsigned long v3;
  int *v4; // rax
  unsigned long v5; // rax
  long v6;
  unsigned long v7;
  long v8; // rcx
  long v9; // rbx
  
  v3 = a10;
  v7 = a9;
  v2 = a8;
  if (0 <= a5) { // branch-flip
    v15 = 1;
    v17 = 0;
    v8 = 0;
    v16 = 0;
    while( true ) {
      v9 = a5;
      v6 = v17;
      v17 = lseek(a0,v9,4);
      if (0 <= v17) { // branch-flip
        v6 = v17;
        if (v17 <= a6)
          v6 = a6;
        a6 = v6;
      }
      else {
        v4 = __errno_location();
        if (*v4 != 6) goto label_7d7f;
        v17 = a6;
        if (a6 <= v9) {
          v17 = lseek(a0,0,2);
          if (v17 < 0) goto label_7d7f;
          if (v17 <= v9) {
            v1 = v6 < v17;
            v14 = (unsigned char)v15 | v1;
            goto label_7f22;
          }
          a6 = v17;
        }
      }
      if ((long)lseek(a0,v9,0) < 0) {
        v4 = __errno_location();
        goto label_7d7f;
      }
      v8 = (v9 - v16) - v8;
      if (v8) { // branch-flip
        if (a7 != 1) { // branch-flip
          v5 = sub_6910(a1,v3,a7 == 3);
          v15 = v5 & 0xffffffff;
          if (!(char)v5)
            return 0;
          v8 = v17 - v9;
          v13 = a4;
        }
        else {
          if (!sub_6820(a1,v8)) {
            v7 = sub_14c40(0,3,v3);
            v12 = "%s: write failed";
            goto label_7d7f;
          }
          v8 = v17 - v9;
          v15 = 0;
          v13 = 0;
        }
      }
      else {
        v8 = v17 - v9;
        v15 = 0;
        v13 = 0;
        if (a7 != 1)
          v13 = a4;
      }
      if (!sub_6a40(a0,a1,a2,a3,v13,1,v2,v7,v3,v8,&v11,&v10))
        return 0;
      v17 = v11 + v9;
      if (v11)
        v15 = (unsigned long)v10;
      v14 = (unsigned char)v15;
      v6 = v17;
      if (v11 < v8) {
        v1 = 0;
        goto label_7f22;
      }
      a5 = lseek(a0,v17,3);
      if (a5 < 0) break;
      v16 = v9;
    }
    v4 = __errno_location();
    if (*v4 != 6) {
label_7d7f:
      v7 = sub_14b40(4,v7);
      v12 = dcgettext(NULL,"cannot lseek %s",5);
      error(0,*v4,v12,v7);
      return 0;
    }
    v1 = v17 < a6;
    v14 |= v1;
    v17 = a6;
label_7f22:
    if (!v14)
      return 1;
  }
  else {
    v1 = 0 < a6;
    v6 = 0;
    v17 = a6;
  }
  if (a7 != 1) { // branch-flip
    if (!ftruncate(a1,v17)) {
      if (a7 != 3)
        return 1;
      if (!v1)
        return 1;
      if (0 <= (int)sub_68d0(a1,v6,v17 - v6))
        return 1;
      v7 = sub_14b40(4,v3);
      v12 = "error deallocating %s";
      goto label_7d7f;
    }
  }
  else if (sub_6820(a1,v17 - v6))
    return 1;
  v7 = sub_14b40(4,v3);
  v12 = "failed to extend %s";
label_7d7f:
  v12 = dcgettext(NULL,v12,5);
  error(0,*__errno_location(),v12,v7);
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
unsigned char sub_7ff0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned char a3,long a4)
{
  unsigned char v1;
  long v2;
  int v3;
  int *v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7; // stack - 0x38
  
  v1 = *(unsigned char *)(a4 + 0x33);
  if (!v1) {
    v2 = *(long *)(a4 + 0x28);
    a3 = v2 != 0 & a3;
    if ((a3) && ((int)sub_c760(v2,a1,a2) <= -1)) {
      v4 = __errno_location();
      v3 = *v4;
      if (!sub_61d0(v3)) {
        v5 = sub_14b40(4,a1);
        v6 = dcgettext(NULL,"failed to set default file creation context for %s",5);
        error(0,*v4,v6,v5);
        return a3;
      }
    }
    return 1;
  }
  if ((*(char *)(a4 + 0x31)) && (!*(char *)(a4 + 0x34))) {
    if (*(char *)(a4 + 0x37)) { // branch-flip
      if ((int)sub_158e0(a0,&v7) <= -1) goto label_807e;
      v3 = setfscreatecon(v7);
      if (0 <= v3) {
        freecon(v7); // return-dupe
        return v1;
      }
    }
    else {
      if ((int)sub_158e0(a0,&v7) <= -1) {
        v4 = __errno_location();
        v3 = *v4;
        if (sub_61d0(v3)) goto label_807e;
        goto label_8048;
      }
      v3 = setfscreatecon(v7);
      if (0 <= v3) {
        freecon(v7);
        return v1;
      }
      v4 = __errno_location();
      v3 = *v4;
      if (!sub_61d0(v3)) goto label_81fd;
    }
  }
  else {
    if ((int)sub_158e0(a0,&v7) <= -1) {
      v4 = __errno_location();
label_8048:
      v5 = sub_14b40(4,a0);
      v6 = dcgettext(NULL,"failed to get security context of %s",5);
      error(0,*v4,v6,v5);
label_807e:
      if (!*(char *)(a4 + 0x34))
        return v1;
      return 0;
    }
    v3 = setfscreatecon(v7);
    if (0 <= v3) {
      freecon(v7);
      return v1;
    }
    v4 = __errno_location();
label_81fd:
    v5 = sub_14e00(v7);
    v6 = dcgettext(NULL,"failed to set default file creation context to %s",5);
    error(0,*v4,v6,v5);
  }
  if (*(char *)(a4 + 0x34)) {
    freecon(v7);
    return 0;
  }
  freecon(v7);
  return v1;
}

// Function: sub_8250 @ 0x8250
unsigned long sub_8250(unsigned long a0,char a1,long a2) // return-dupe
{
  int v1;
  unsigned int v2; // eax
  unsigned long v3;
  char *v4; // rax
  int *v5; // rax
  
  v3 = *(unsigned long *)(a2 + 0x28);
  if ((*(char *)(a2 + 0x31)) && (!*(char *)(a2 + 0x34))) {
    if (*(char *)(a2 + 0x37))
      return sub_c960(v3,a0,a1); // tail-call
    v2 = sub_c960(v3,a0,a1);
    if ((char)v2)
      return 1;
    v5 = __errno_location();
    v1 = *v5;
    if (sub_61d0(v1))
      return (unsigned long)v2;
  }
  else {
    if (sub_c960(v3,a0,a1))
      return 1;
    v5 = __errno_location();
  }
  v3 = sub_14a70(0,4,a0);
  v4 = dcgettext(NULL,"failed to set the security context of %s",5);
  error(0,*v5,v4,v3);
  return 0;
}

// Function: sub_8330 @ 0x8330
void sub_8330(long a0)
{
  long v1; // rax
  
  v1 = sub_11cd0(0x3d,0,sub_12540,sub_12510,sub_125b0);
  *(long *)(a0 + 0x48) = v1;
  if (v1)
    return;
  sub_17250(); // no-return
}

// Function: sub_8370 @ 0x8370
void sub_8370(long a0)
{
  long v1; // rax
  
  v1 = sub_11cd0(0x3d,0,sub_124f0,sub_12510,sub_125b0);
  *(long *)(a0 + 0x50) = v1;
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
int * sub_8400(long a0)
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
    v4 = (unsigned int)*(unsigned char *)(a0 + 0x1a);
    v4 ^= 1;
    v5 = (unsigned long)(unsigned long)v4;
  }
  return v5;
}

// Function: sub_8430 @ 0x8430
int sub_8430(long a0,unsigned long a1,unsigned int a2,unsigned long a3,int a4,unsigned int a5,unsigned int a6,unsigned int a7,char a8,long a9)
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
    if (*(unsigned long *)(a0 + 0x18) & 0xff0000000000ff) // branch-flip
      v4 = *(unsigned int *)(a9 + 0x18);
    else {
      if (!*(char *)(a0 + 0x39)) goto label_84a0;
      a5 = *(unsigned int *)(a0 + 0x10);
      v4 = *(unsigned int *)(a9 + 0x18);
    }
    if ((!((unsigned short)(~(unsigned short)a5 | 0xe00) & v4 & 0xfff)) || (!sub_12910(a1,a4,a5 & v4 & 0x1c0))) goto label_84a0;
    if (sub_6230(a0)) {
      return -(unsigned int)*(unsigned char *)(a0 + 0x32);
    }
    v3 = sub_14b40(4,a1);
    v5 = "clearing permissions for %s";
  }
  v5 = dcgettext(NULL,v5,5);
  error(0,*__errno_location(),v5,v3);
  return -(unsigned int)*(unsigned char *)(a0 + 0x32);
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
unsigned int sub_8680(unsigned long a0,char *a1,int a2,char *a3,long a4,unsigned int a5,unsigned int a6,char *a7,long *a8)
{
  char v1;
  unsigned long v10;
  char *v11;
  int *v12; // rax
  unsigned long v13;
  unsigned long v14; // rax
  unsigned long v15; // rax
  char *v16; // rax
  unsigned int v17;
  long v18; // stack - 0xd8
  char v19 [24];
  char v2;
  char v20; // stack - 0x1a1
  unsigned long v21; // stack - 0x1a0
  unsigned long v22; // stack - 0x190
  unsigned long v23; // stack - 0x188
  long v24;
  bool v25; // al
  bool v26; // r9b
  char v27 [16];
  long v28; // stack - 0x1d8
  unsigned int v29; // stack - 0x1c4
  long *v3;
  int v30; // stack - 0x1b8
  unsigned int v31; // stack - 0x1b0
  unsigned long v32; // stack - 0x198
  unsigned long v33; // stack - 0x180
  unsigned long v34; // stack - 0x178
  unsigned long v35; // stack - 0x170
  unsigned int v36; // stack - 0x150
  int v37; // stack - 0x14c
  int v38; // stack - 0x148
  long v39; // stack - 0x130
  char v4;
  long v40; // stack - 0xd0
  unsigned int v41; // stack - 0xc0
  unsigned long v42; // stack - 0xa8
  unsigned long v43; // stack - 0xa0
  unsigned char v5; // al
  int v6; // eax
  unsigned int v7;
  int v8;
  unsigned int v9;
  
  v3 = a8;
  v11 = a7;
  v29 = *(unsigned int *)&a8[3];
  v21 = 0;
  v4 = *(char *)(a4 + 0x31);
  v1 = *(char *)(a4 + 0x35);
  v6 = sub_e880(a0,(unsigned long)(*(int *)(a4 + 4) == 2) << 0x11);
  if (v6 < 0) {
    v10 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"cannot open %s for reading",5);
    error(0,*__errno_location(),v11,v10);
    return 0;
  }
  if (fstat(v6,&v18)) { // branch-flip
    v10 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"cannot fstat %s",5);
    v8 = *__errno_location();
label_8779:
    v7 = 0;
    error(0,v8,v11,v10);
    v10 = 0;
  }
  else {
    if ((v3[1] != v40) || (*v3 != v18)) {
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
      if ((v1) && (!*(char *)(a4 + 0x1b)))
        v9 |= 0x80;
      v30 = sub_12710(a2,a3,0xc1,v9);
      v8 = *v12;
      v5 = -(char)(v30 >> 0x1f);
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
          if (0 <= v30) goto label_88bc;
        }
      }
      else {
        v8 = 0x11;
        if (!*(char *)(a4 + 0x18)) {
          v30 = 0x11;
          v8 = v30;
          if (0 <= (long)readlinkat(a2,a3,&v24,1)) {
            if (!*(char *)(a4 + 0x3e)) {
              v10 = sub_14b40(4,a1);
              error(0,0,dcgettext(NULL,"not writing through dangling symlink %s",5),v10);
              v7 = 0;
label_92ad:
              v10 = 0;
              goto label_8788;
            }
            v30 = sub_12710(a2,a3,0x41,v9);
            v8 = *v12;
            v5 = v8 == 0x15 & (unsigned char)((unsigned int)v30 >> 0x1f);
            goto label_8da9;
          }
        }
      }
label_8d49:
      v10 = sub_14b40(4,a1);
      v11 = dcgettext(NULL,"cannot create regular file %s",5);
      goto label_8779;
    }
    v30 = sub_12710(a2,a3,(-(unsigned int)(v4 == '\0') & 0xfffffe00) + 0x201);
    v8 = *v12;
    if (0 <= v30) { // branch-flip
      if ((*(long *)(a4 + 0x28)) || (*(char *)(a4 + 0x33))) {
        v7 = sub_8250(a1,0,a4);
        if (((char)v7) || (!*(char *)(a4 + 0x34))) goto label_88a0;
      }
      else {
label_88a0:
        if (*v11) goto label_8cdb;
        a6 = 0;
        v9 = 0;
label_88bc:
        if (v4) { // branch-flip
          if (*(int *)(a4 + 0x44)) {
            if (!sub_6770(v30,v6)) goto label_8de0;
            if (*(int *)(a4 + 0x44) == 2) {
              v7 = 0;
              v10 = sub_14a70(1,4,a0);
              v15 = sub_14a70(0,4,a1);
              v11 = dcgettext(NULL,"failed to clone %s from %s",5);
              error(0,*v12,v11,v15,v10);
              goto label_8ba8;
            }
          }
label_88d2:
          v31 = fstat(v30,v19);
          if (v31) {
            v7 = 0;
            v10 = sub_14b40(4,a1);
            v11 = dcgettext(NULL,"cannot fstat %s",5);
            error(0,*v12,v11,v10);
            goto label_8ba8;
          }
          v7 = v36 | v9;
          if (v36 != v7) {
            if (sub_7020(v30,a2,a3,v7))
              v9 = v31;
          }
          if (v4) {
            v13 = sub_7fd0(v39,&v18);
            v28 = 0x200;
            if ((unsigned long)(v39 - 1U) <= 0x1fffffffffffffff)
              v28 = v39;
            v8 = sub_6780(v6);
            if (v8) { // branch-flip
              if (((v36 & 0xf000) != 0x8000) || ((*(int *)(a4 + 0xc) != 3 && ((*(int *)(a4 + 0xc) != 2 || (v8 == 1)))))) {
                sub_e840(v6,0,0,2);
                v14 = sub_dbf0(sub_7fd0(v43),v13,0x7fffffffffffffff);
                if (((v41 & 0xf000) == 0x8000) && (v42 < v13))
                  v13 = v42 + 1;
                v13 = (v13 - 1) + v14;
                v13 -= v13 % v14;
                v25 = *(int *)(a4 + 0x44) != 0;
                if (0 < (long)v13) {
                  v17 = 1;
                  if (v8 == 3) goto label_91e4;
                  v28 = 0;
                  v26 = *(int *)(a4 + 0xc) == 3;
                  goto label_8a67;
                }
                v13 = v14;
                if (v8 != 3) {
                  v28 = 0;
                  v26 = *(int *)(a4 + 0xc) == 3;
                  goto label_8a67;
                }
                v17 = 1;
label_91e4:
                v20 = '\0';
                v4 = sub_7b50(v6,v30,&v21,v13,v28,v32,v42,v17,v25,a0,a1);
              }
              else {
                sub_e840(v6,0,0,2);
                if (v8 == 3) {
                  v25 = *(int *)(a4 + 0x44) != 0;
                  v17 = *(unsigned int *)(a4 + 0xc);
                  goto label_91e4;
                }
                v25 = *(int *)(a4 + 0x44) != 0;
                v26 = *(int *)(a4 + 0xc) == 3;
label_8a67:
                v20 = '\0';
                v4 = sub_6a40(v6,v30,&v21,v13,v28,v26,v25,a0,a1,0xffffffffffffffff,&v22,&v20);
              }
              if (v4 != '\x01') goto label_8eb0;
              if ((!v20) || (0 <= (int)ftruncate(v30,v22))) goto label_8ab7;
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
          if (*(char *)(a4 + 0x1f)) goto label_8e06;
label_8ad0:
          if (*(char *)(a4 + 0x1d)) {
            if ((*(int *)((long)v3 + 0x1c) != v37) || (v38 != (int)v3[4])) {
              v7 = 0;
              v8 = sub_8430(a4,a1,a2,a3,v30,(int)v3[3],*(int *)((long)v3 + 0x1c),(int)v3[4],*v11,v19);
              if (v8 == -1) goto label_8ba8;
              v7 = v29 & 0xfffff1ff;
              if (v8)
                v7 = v29;
              v29 = v7;
            }
          }
        }
        else {
label_8de0:
          if (*(char *)(a4 + 0x1d) || v9) {
            v4 = '\0';
            goto label_88d2;
          }
          v36 = 0;
          v9 = 0;
          if (*(char *)(a4 + 0x1f)) {
label_8e06:
            v27 = sub_15960(v3);
            v33 = SUB168(v27,8);
            v23 = SUB168(v27,0);
            v27 = sub_15980(v3);
            v35 = SUB168(v27,8);
            v34 = SUB168(v27,0);
            if (sub_e900(v30,a2,a3,&v23,0)) {
              v10 = sub_14b40(4,a1);
              v16 = dcgettext(NULL,"preserving times for %s",5);
              error(0,*v12,v16,v10);
              if (*(char *)(a4 + 0x32)) goto label_8eb0;
            }
            goto label_8ad0;
          }
        }
        if ((v1) && (!sub_7040(a0,v6,a1,v30,a4)))
          v7 = *(unsigned char *)(a4 + 0x36) ^ 1;
        else {
          v7 = 1;
        }
        if (*(unsigned long *)(a4 + 0x18) & 0xff0000000000ff) { // branch-flip
          if ((sub_caa0(a0,v6,a1,v30,v29)) && (*(char *)(a4 + 0x32)))
            v7 = 0;
        }
        else if (*(char *)(a4 + 0x39)) { // branch-flip
          v17 = *(unsigned int *)(a4 + 0x10);
          if (sub_cb50(a1,v30,v17))
            v7 = 0;
        }
        else if ((*(char *)(a4 + 0x20)) && (*v11)) {
          v9 = sub_8640();
          if (sub_cb50(a1,v30,~v9 & 0x1b6))
            v7 = 0;
        }
        else if ((v9 || a6) && (~sub_8640() & a6 || v9)) {
          v9 = sub_8640();
          if (sub_7020(v30,a2,a3,a5 & ~v9)) {
            v10 = sub_14b40(4,a1);
            v11 = dcgettext(NULL,"preserving permissions for %s",5);
            error(0,*v12,v11,v10);
            if (*(char *)(a4 + 0x32))
              v7 = 0;
          }
        }
      }
label_8ba8:
      v8 = close(v30);
      v10 = v21;
      if (0 <= v8) {
        v6 = close(v6);
        goto label_8bc6;
      }
      v7 = 0;
      v10 = sub_14b40(4,a1);
      v11 = dcgettext(NULL,"failed to close %s",5);
      error(0,*v12,v11,v10);
      v10 = v21;
    }
    else {
      if (v8 == 2) goto label_8ca8;
      if (!*(char *)(a4 + 0x16)) {
        if (*v11) goto label_8cdb;
        goto label_8d49;
      }
      if (!unlinkat(a2,a3,0)) {
        if (*(char *)(a4 + 0x3c)) {
          v10 = sub_14b40(4,a1);
          __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v10);
        }
label_8ca8:
        if ((!*(long *)(a4 + 0x28)) || (v7 = sub_7ff0(a0,a1,a5,1,a4), (char)v7)) {
          *v11 = '\x01';
          goto label_8cdb;
        }
        goto label_92ad;
      }
      if (*v12 == 2) goto label_8ca8;
      v15 = sub_14b40(4,a1);
      v11 = dcgettext(NULL,"cannot remove %s",5);
      v10 = 0;
      error(0,*v12,v11,v15);
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
unsigned long sub_9470(long *a0,long *a1,int a2,long *a3,unsigned int a4,long *a5,long *a6,int *a7,unsigned int a8,char *a9,char *a10,long *a11)
{
  int v1;
  unsigned long v10;
  unsigned long v11; // rax
  unsigned long v12;
  void *v13; // rax
  long *v14;
  unsigned int v15;
  char v16 [4];
  long v17; // stack - 0x288
  long v18; // stack - 0x1f8
  char v19; // stack - 0x289
  int *v2;
  long v20; // stack - 0x168
  long v21; // stack - 0xd8
  bool v22; // stack - 0x28a
  char *v23;
  char *v24;
  char *v25;
  char *v26;
  char *v27;
  char *v28; // rsp
  char *v29; // rsp
  char v3;
  char v30;
  unsigned int v31;
  int v32;
  unsigned long v33; // r14
  unsigned char v34;
  unsigned int v35;
  long *v36;
  bool v37; // r8b
  unsigned char v38;
  char v39 [16];
  unsigned char v4; // al
  unsigned int v40; // stack - 0x304
  long v41; // stack - 0x300
  unsigned int v42; // stack - 0x2f8
  unsigned char v43; // stack - 0x2f1
  long *v44; // stack - 0x2f0
  char *v45; // stack - 0x2e8
  char *v46; // stack - 0x2e0
  unsigned long *v47; // stack - 0x2d8
  long *v48; // stack - 0x2d0
  long *v49; // stack - 0x2c8
  char v5; // al
  long *v50; // stack - 0x2c0
  unsigned long v51; // stack - 0x2b8
  long *v52; // stack - 0x2b0
  long *v53; // stack - 0x2a8
  int v54; // stack - 0x2a0
  unsigned int v55; // stack - 0x29c
  unsigned long v56; // stack - 0x280
  unsigned long v57; // stack - 0x278
  unsigned int v58; // stack - 0x270
  unsigned int v59; // stack - 0x26c
  int v6;
  unsigned int v60; // stack - 0x268
  unsigned long v61; // stack - 0x260
  unsigned long v62; // stack - 0x258
  unsigned long v63; // stack - 0x1f0
  unsigned long v64; // stack - 0x1e8
  unsigned int v65; // stack - 0x1e0
  unsigned int v66; // stack - 0x1dc
  unsigned int v67; // stack - 0x1d8
  long *v68; // stack - 0x1c8
  long v69; // stack - 0x160
  int *v7; // rax
  long v70; // stack - 0xd0
  unsigned long v71; // stack - 0xc8
  unsigned long v72; // stack - 0xc0
  long v8;
  char *v9;
  
  v2 = a7;
  v24 = v16;
  v23 = v16;
  v29 = v16;
  v25 = v16;
  v26 = v16;
  v27 = v16;
  v48 = a6;
  v46 = a9;
  v51 = CONCAT44(v51._4_4_,a8);
  v45 = a10;
  v32 = a7[0x10];
  v50 = a11;
  v47 = (unsigned long *)CONCAT71(v47._1_7_,(char)a8);
  *a10 = 0;
  v22 = 0 < (int)a4;
  v52 = a1;
  v44 = a5;
  v53 = a3;
  v54 = a2;
  if (!(char)a7[6]) {
    if (!v32) goto label_9531;
label_9560:
    if ((v32 == 0x11) && (v2[2] == 2)) {
      if (!(char)v51) goto label_9767;
      goto label_95c4;
    }
    v6 = -100;
    v14 = a0;
    v36 = a0;
label_9575:
    v1 = v2[1];
    if (!sub_6360(v6,v14,&v17,(unsigned long)(v1 == 2) << 8)) {
      v55 = v58;
      if (((v58 & 0xf000) != 0x4000) || (v36 = NULL, (char)v2[0xe])) goto label_95b7;
      v10 = sub_14b40(4,a0);
      v36 = NULL;
      if (*(char *)((long)v2 + 0x19)) {
        v9 = "omitting directory %s";
        v9 = dcgettext(NULL,v9,5); // return-dupe
        error(0,0,v9,v10); // return-dupe
        return (unsigned long)v36 & 0xffffffff; // return-dupe
      }
      v9 = dcgettext(NULL,"-r not specified; omitting directory %s",5);
      error(0,0,v9,v10);
      return (unsigned long)v36 & 0xffffffff;
    }
label_a440:
    *(unsigned long *)&v24[-8] = 0xa44a;
    v10 = sub_14b40(4,v36);
    v9 = "cannot stat %s";
label_a459:
    v36 = NULL;
    *(unsigned long *)&v24[-8] = 0xa463;
    v9 = dcgettext(NULL,v9,5,v24[-8]);
    *(unsigned long *)&v24[-8] = 0xa46b;
    v30 = v24[-8];
    v32 = *__errno_location(v30);
    *(unsigned long *)&v24[-8] = 0xa47c;
    error(0,v32,v9,v10);
    return (unsigned long)v36 & 0xffffffff;
  }
  if (v32 < 0) {
    if (!sub_14e40(0xffffff9c,a0,a2,a3,1)) {
      v22 = 1;
      a4 = 1;
      *(char *)v50 = '\x01';
      goto label_9531;
    }
    v32 = *__errno_location();
  }
  v22 = v32 == 0;
  *(bool *)v50 = v22;
  a4 = (unsigned int)v22;
  if (v32) goto label_9560;
label_9531:
  if (!*(char *)((long)v2 + 0x3f)) {
    v32 = 0;
    v14 = v53;
    v36 = v52;
    v6 = v54;
    goto label_9575;
  }
  v32 = 0;
label_95b7:
  if (!(char)v51) {
label_9767:
    v6 = v2[1];
    v43 = sub_7fb0(v6,0);
    if ((int)a4 > 0) {
      v49 = NULL;
      goto label_9668;
    }
label_9858:
    if ((v32 == 0x11) && (v2[2] == 2)) {
      v19 = '\0';
      v3 = '\0';
      goto label_9935;
    }
    v15 = v55 & 0xf000;
    if ((((v15 == 0x8000) || (v34 = (v15 == 0xa000 || v15 == 0x4000) | *(unsigned char *)&v2[5] ^ 1, !v34)) && (v34 = *(unsigned char *)&v2[6], !v34)) && ((v34 = *(unsigned char *)((long)v2 + 0x3a), !v34 && (v34 = *(unsigned char *)((long)v2 + 0x17), !v34)))) {
      if (*v2) {
        v34 = 1;
        v31 = 0x100;
        goto label_98a2;
      }
      v34 = *(unsigned char *)((long)v2 + 0x15);
      if (v34) goto label_989c;
      v31 = 0;
      if (!a4) goto label_98a2;
label_a36b:
      v22 = 1;
label_a372:
      v49 = NULL;
      v3 = '\0';
      v49._0_1_ = '\0';
      v23 = v16;
      if (v32 == 0x11) {
        v32 = v2[2];
        goto label_98db;
      }
label_a38a:
      if ((char)v51) goto label_9627;
      goto label_9668;
    }
label_989c:
    v31 = 0x100;
label_98a2:
    v36 = &v18;
    v49 = (long *)CONCAT71(v49._1_7_,v34);
    v6 = sub_6360(v54,v53,v36,v31);
    if (v6) {
      v7 = __errno_location();
      if (*v7 != 0x28) { // branch-flip
        if (*v7 == 2) goto label_a36b;
      }
      else if (*(char *)((long)v2 + 0x16)) goto label_a372;
      v36 = NULL;
      v10 = sub_14b40(4,v52);
      v9 = dcgettext(NULL,"cannot stat %s",5);
      error(0,*v7,v9,v10);
      return (unsigned long)v36 & 0xffffffff;
    }
    v32 = v2[2];
label_98db:
    v19 = '\0';
    v3 = (char)v49;
    if (v32 != 2) {
      v9 = &v19;
      v15 = sub_75b0(a0,&v17,v54,v53,&v18,v2);
      v36 = (long *)(unsigned long)v15;
      v3 = (char)v49;
      if ((char)v15) goto label_9935;
      v10 = sub_14a70(1,4,v52,v9);
      v11 = sub_14a70(0,4,a0);
      v9 = "%s and %s are the same file";
      goto label_abe0;
    }
label_9935:
    if (!*(char *)((long)v2 + 0x3b)) {
      if ((char)v2[6]) goto label_aa71;
      if ((v55 & 0xf000) == 0x4000) goto label_a641;
label_a62c:
      if (v2[2] != 2) {
        if (v2[2] == 3) {
          v49 = (long *)CONCAT71(v49._1_7_,v3);
          v5 = sub_63e0(v2,v52,v54,v53,&v18);
          v3 = (char)v49;
          if (!v5) {
            v36 = (long *)0x1; // return-dupe
            return (unsigned long)v36 & 0xffffffff;
          }
        }
        goto label_a641;
      }
      v36 = (long *)0x1;
      return (unsigned long)v36 & 0xffffffff;
    }
    if ((v55 & 0xf000) == 0x4000) {
      if (!(char)v2[6]) goto label_a641;
label_aa71:
      v49 = (long *)CONCAT71(v49._1_7_,v3);
      v15 = sub_6530(v2,v52,v54,v53,&v18);
      v36 = (long *)(unsigned long)v15;
      v3 = (char)v49;
      if (!(char)v15) {
label_a641:
        if (v19) {
          v36 = (long *)0x1;
          return (unsigned long)v36 & 0xffffffff;
        }
        v36 = NULL;
        if ((v65 & 0xf000) != 0x4000) { // branch-flip
          if ((v55 & 0xf000) != 0x4000) { // branch-flip
            v32 = *v2;
            if (!(char)v51) goto label_a6f6;
label_a68f:
            v49 = (long *)(CONCAT44(v49._4_4_,v55) & 0xffffffff0000f000);
            if (v32 != 3) {
              v41 = CONCAT71(v41._1_7_,v3);
              v3 = sub_ea90(*(unsigned long *)&v2[0x12],v53,&v18);
              if (v3) {
                v10 = sub_14a70(1,4,a0);
                v11 = sub_14a70(0,4,v52);
                v9 = "will not overwrite just-created %s with %s";
                v27 = v16;
                goto label_abe0;
              }
              v3 = (char)v41;
              if (((int)v49 == 0x4000) || ((v65 & 0xf000) != 0x4000)) goto label_a6f4;
              goto label_ae9c;
            }
            v32 = 3;
            v6 = 3;
            if ((char)v2[6]) goto label_b0d1;
label_b25f:
            v15 = v65;
            v49 = (long *)CONCAT44(v49._4_4_,v6);
            v41 = CONCAT71(v41._1_7_,v3);
            v5 = sub_6190(sub_db60(a0));
            v3 = (char)v41;
            if (!v5) {
              v32 = (int)v49;
              if ((v15 & 0xf000) == 0x4000) goto label_a798;
              goto label_aef9;
            }
label_a70f:
            if (((v15 & 0xf000) == 0x4000) || ((!*(char *)((long)v2 + 0x15) && ((!*(char *)((long)v2 + 0x31) || (((!(char)v2[0xc] || (v64 <= 1)) && ((v2[1] != 2 || ((v58 & 0xf000) == 0x8000)))))))))) goto label_a798;
            v49 = (long *)CONCAT71(v49._1_7_,v3);
            v32 = unlinkat(v54,(char *)v53,0);
            if (v32) {
              v7 = __errno_location();
              if (*v7 != 2) {
                v10 = sub_14b40(4,v52);
                v9 = "cannot remove %s";
                v9 = dcgettext(NULL,v9,5); // return-dupe
                error(0,*v7,v9,v10);
                return (unsigned long)v36 & 0xffffffff;
              }
            }
            v22 = 1;
            v3 = (char)v49;
            if ((char)v2[0xf]) {
              v41 = CONCAT71(v41._1_7_,(char)v49);
              v10 = sub_14b40(4,v52);
              __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v10);
              v49 = NULL;
              v3 = (char)v41;
              goto label_a7a3;
            }
          }
          else {
            if ((!(char)v2[6]) || (v32 = *v2, !v32)) {
              v10 = sub_14a70(1,4,a0);
              v11 = sub_14a70(0,4,v52);
              v9 = "cannot overwrite non-directory %s with directory %s";
              v27 = v16;
              goto label_abe0;
            }
            if ((char)v51) goto label_a68f;
label_b0d1:
            if ((v58 & 0xf000) == 0x4000) {
              v15 = v65;
label_b487:
              if ((v15 & 0xf000) != 0x4000) {
                if (!v32) {
                  v10 = sub_14c40(0,3,v52);
                  v11 = sub_14c40(0,3,a0);
                  v9 = "cannot move directory onto non-directory: %s -> %s";
                  v27 = v16;
                  goto label_abe0;
                }
                goto label_aec6;
              }
            }
            if (v32) goto label_aec6;
          }
label_a798:
          v49 = NULL;
          v26 = v16;
        }
        else {
          if ((v55 & 0xf000) == 0x4000) {
label_a6f4:
            v32 = *v2;
label_a6f6:
            if (!(char)v2[6]) {
              v15 = v65;
              v6 = v32;
              if (v32) goto label_b25f;
              goto label_a70f;
            }
            goto label_b0d1;
          }
label_ae9c:
          if ((!(char)v2[6]) || (v32 = *v2, !v32)) {
            v10 = sub_14b40(4,v52);
            v9 = "cannot overwrite directory %s with non-directory";
            v9 = dcgettext(NULL,v9,5);
            error(0,0,v9,v10);
            return (unsigned long)v36 & 0xffffffff;
          }
          v15 = v65;
          if ((v58 & 0xf000) == 0x4000) goto label_b487;
label_aec6:
          v49 = (long *)CONCAT71(v49._1_7_,v3);
          v41 = CONCAT44(v41._4_4_,v32);
          v5 = sub_6190(sub_db60(a0));
          v3 = (char)v49;
          if (v5) goto label_a798;
          v32 = (int)v41;
label_aef9:
          if (v32 != 3) {
            v49 = (long *)CONCAT71(v49._1_7_,v3);
            v5 = sub_6640();
            v3 = (char)v49;
            if (v5) {
              if ((char)v2[6]) // branch-flip
                v9 = dcgettext(NULL,"backing up %s might destroy source;  %s not moved",5);
              else {
                v9 = dcgettext(NULL,"backing up %s might destroy source;  %s not copied",5);
              }
              v10 = sub_14a70(1,4,a0);
              v11 = sub_14a70(0,4,v52);
              error(0,0,v9,v11,v10);
              return (unsigned long)v36 & 0xffffffff;
            }
          }
          v14 = v53;
          v49 = (unsigned long)CONCAT71(v49._1_7_,v3);
          v9 = (char *)sub_da90(v54,v53,*v2);
          if (v9) { // branch-flip
            v33 = (long)v14 - (long)v52;
            v12 = strlen(v9);
            v36 = (long *)(v12 + 1);
            v12 = v33 + 0x18 + v12;
            v26 = v16;
            while (v29 != &v16[-(v12 & 0xfffffffffffff000)]) {
              v25 = &v26[-0x1000];
              v28 = &v26[-0x1000];
              *(unsigned long *)&v26[-8] = *(unsigned long *)&v26[-8];
              v29 = &v26[-0x1000];
              v26 = v28;
            }
            v12 = (unsigned long)((unsigned int)v12 & 0xff0);
            v8 = -v12;
            if (v12)
              *(unsigned long *)&v25[-8] = *(unsigned long *)&v25[-8];
            v41 = CONCAT71(v41._1_7_,(char)v49);
            v49 = (long *)((unsigned long)&v25[v8 + 0xf] & 0xfffffffffffffff0);
            *(unsigned long *)&v25[v8 + -8] = 0xafea;
            v13 = mempcpy((long *)((unsigned long)&v25[v8 + 0xf] & 0xfffffffffffffff0),v52,v33,v25[v8 + -8]);
            *(unsigned long *)&v25[v8 + -8] = 0xaff8;
            memcpy(v13,v9,(unsigned long)v36,v25[v8 + -8]);
            *(unsigned long *)&v25[v8 + -8] = 0xb000;
            free(v9,v25[v8 + -8]);
            v26 = &v25[v8];
            v3 = (char)v41;
          }
          else {
            v41 = CONCAT71(v41._1_7_,(char)v49);
            v7 = __errno_location();
            v49 = NULL;
            v26 = v16;
            v3 = (char)v41;
            if (*v7 != 2) {
              v10 = sub_14b40(4,v52);
              v9 = "cannot backup %s";
              v9 = dcgettext(NULL,v9,5);
              error(0,*v7,v9,v10);
              return (unsigned long)v36 & 0xffffffff;
            }
          }
          v22 = 1;
        }
label_a7a3:
        v32 = 0x11;
        v23 = v26;
        goto label_a38a;
      }
      if (!v50) {
        v36 = (long *)0x1;
        return (unsigned long)v36 & 0xffffffff;
      }
      goto label_aaba;
    }
    v37 = 0;
    if ((*(char *)((long)v2 + 0x1f)) && (v37 = 1, (char)v2[6]))
      v37 = v18 != v17;
    v49 = (long *)CONCAT71(v49._1_7_,v3);
    v32 = sub_15e70(v54,v53,&v18,&v17,v37);
    v14 = v53;
    if (v32 < 0) {
      v3 = (char)v49;
      if (!(char)v2[6]) goto label_a62c;
      goto label_aa71;
    }
    if (v50)
      *(char *)v50 = '\x01';
    v8 = sub_bf60(v53,v56,v17);
    if ((!v8) || (v3 = sub_7460(0,v54,v8,v52,v54,v14,1,(char)v2[0xf],v43), v3)) {
      v36 = (long *)0x1;
      return (unsigned long)v36 & 0xffffffff;
    }
    if (*(char *)((long)v2 + 0x33))
      sub_7380();
    v36 = NULL; // return-dupe
    return (unsigned long)v36 & 0xffffffff;
  }
label_95c4:
  v8 = *(long *)&v2[0x14];
  if (v8) {
    v14 = &v17;
    if (((v55 & 0xf000) == 0x4000) || (*v2)) {
label_95f2:
      sub_ea00(v8,a0,v14);
      goto label_95fd;
    }
    v49 = &v17;
    v15 = sub_ea90(v8,a0,&v17);
    v36 = (long *)(unsigned long)v15;
    if (!(char)v15) {
      v8 = *(long *)&v2[0x14];
      v14 = v49;
      goto label_95f2;
    }
    v10 = sub_14b40(4,a0);
    v9 = "warning: source file %s specified more than once";
    v9 = dcgettext(NULL,v9,5);
    error(0,0,v9,v10);
    return (unsigned long)v36 & 0xffffffff;
  }
label_95fd:
  v6 = v2[1];
  v43 = sub_7fb0(v6,1);
  if ((int)a4 <= 0) goto label_9858;
  v49 = NULL;
  v3 = '\0';
  v23 = v16;
label_9627:
  if ((*(long *)&v2[0x12]) && (v36 = NULL, !(char)v2[6])) {
    if (!*v2) {
      v14 = &v18;
      if (!v3) {
        v14 = &v21;
        *(unsigned long *)&v23[-8] = 0xaae9;
        v6 = fstatat(v54,(char *)v53,v14,0x100,v23[-8]);
        if (v6) goto label_9668;
      }
      if ((*(unsigned int *)&v14[3] & 0xf000) == 0xa000) {
        v10 = *(unsigned long *)&v2[0x12];
        *(unsigned long *)&v23[-8] = 0xaba0;
        v3 = sub_ea90(v10,v53,v14);
        if (v3) {
          *(unsigned long *)&v23[-8] = 0xabbe;
          v10 = sub_14a70(1,4,v52);
          *(unsigned long *)&v23[-8] = 0xabd0;
          v11 = sub_14a70(0,4,a0);
          v9 = "will not copy %s through just-created symlink %s";
          v36 = NULL;
          v27 = v23;
label_abe0:
          *(unsigned long *)&v27[-8] = 0xabe7;
          v9 = dcgettext(NULL,v9,5,v27[-8]);
          *(unsigned long *)&v27[-8] = 0xabfb;
          error(0,0,v9,v11,v10);
          return (unsigned long)v36 & 0xffffffff;
        }
      }
      goto label_9668;
    }
    if ((char)v2[0xf]) goto label_9f88;
    if (!v32) goto label_9ad8;
label_9681:
    if ((!(char)v2[0xe]) || ((v55 & 0xf000) != 0x4000)) {
      v3 = (char)v2[6];
      if (v3) {
        if (v57 == 1) {
          *(unsigned long *)&v23[-8] = 0xaca3;
          v41 = sub_bf00(v56,v17);
label_a961:
          if (v41) {
            if ((v55 & 0xf000) == 0x4000) goto label_96d5;
            *(unsigned long *)&v23[-0x10] = (unsigned long)v43;
            *(unsigned long *)&v23[-0x18] = (unsigned long)*(unsigned char *)&v2[0xf];
            *(unsigned long *)&v23[-0x20] = 1;
            *(unsigned long *)&v23[-0x28] = 0xa9bd;
            v3 = sub_7460(0,v54,v41,v52,v54,v53);
            if (v3) {
              v36 = (long *)0x1;
              return (unsigned long)v36 & 0xffffffff;
            }
            goto label_a9c9;
          }
          goto label_9740;
        }
        if (((char)v2[0xc]) && (!*(char *)((long)v2 + 0x17))) goto label_9f34;
        v41 = 0;
label_97c5:
        if (v32 != 0x11) { // branch-flip
label_97cf:
          if (v32 != 0x16) {
            if (v32 != 0x12) {
              *(unsigned long *)&v23[-8] = 0xa816;
              v10 = sub_14a70(1,4,v52);
              *(unsigned long *)&v23[-8] = 0xa828;
              v11 = sub_14a70(0,4,a0);
              *(unsigned long *)&v23[-8] = 0xa83e;
              v9 = dcgettext(NULL,"cannot move %s to %s",5,v23[-8]);
label_a84a:
              *(unsigned long *)&v23[-8] = 0xa853;
              error(0,v32,v9,v11,v10);
              *(unsigned long *)&v23[-8] = 0xa866;
              sub_bea0(v56,v17);
              v36 = NULL;
              return (unsigned long)v36 & 0xffffffff;
            }
            v15 = v55 & 0xf000;
            *(unsigned long *)&v23[-8] = 0x9812;
            v32 = unlinkat(v54,(char *)v53,(unsigned int)(v15 == 0x4000) << 9,v23[-8]);
            if (v32) {
              *(unsigned long *)&v23[-8] = 0x981b;
              v7 = __errno_location(v23[-8]);
              if (*v7 != 2) {
                *(unsigned long *)&v23[-8] = 0xab3e;
                v10 = sub_14a70(1,4,v52);
                *(unsigned long *)&v23[-8] = 0xab50;
                v11 = sub_14a70(0,4,a0);
                *(unsigned long *)&v23[-8] = 0xab66;
                v9 = dcgettext(NULL,"inter-device move failed: %s to %s; unable to remove target",5,v23[-8]);
                v32 = *v7;
                goto label_a84a;
              }
            }
            if (((char)v2[0xf]) && (v15 != 0x4000)) {
              *(unsigned long *)&v23[-8] = 0xaa33;
              v9 = dcgettext(NULL,"copied ",5,v23[-8]);
              *(unsigned long *)&v23[-8] = 0xaa42;
              __printf_chk(1,v9);
              *(unsigned long *)&v23[-8] = 0xaa58;
              sub_73c0(a0,v52,v49);
            }
            v22 = 1;
            v34 = 1;
            v38 = 1;
            goto label_9aee;
          }
          *(unsigned long *)&v23[-8] = 0xacc6;
          v10 = sub_14a70(1,4,dat_25568);
          *(unsigned long *)&v23[-8] = 0xacdc;
          v11 = sub_14a70(0,4,dat_25570);
          *(unsigned long *)&v23[-8] = 0xacf2;
          v9 = dcgettext(NULL,"cannot move %s to a subdirectory of itself, %s",5,v23[-8]);
          *(unsigned long *)&v23[-8] = 0xad06;
          error(0,0,v9,v11,v10);
          *v45 = 1;
        }
        else {
          *(unsigned long *)&v23[-8] = 0xa7ca;
          v32 = renameat(0xffffff9c,a0,v54,v53);
          if (v32) {
            *(unsigned long *)&v23[-8] = 0xa7d7;
            v30 = v23[-8];
            v32 = *__errno_location(v30);
            if (v32) goto label_97cf;
          }
label_9a66:
          if ((char)v2[0xf]) {
            *(unsigned long *)&v23[-8] = 0xac23;
            v9 = dcgettext(NULL,"renamed ",5,v23[-8]);
            *(unsigned long *)&v23[-8] = 0xac32;
            __printf_chk(1,v9);
            *(unsigned long *)&v23[-8] = 0xac48;
            sub_73c0(a0,v52,v49);
          }
          if (*(long *)&v2[10]) {
            *(unsigned long *)&v23[-8] = 0x9a8b;
            sub_8250(v52,1,v2);
          }
          if (v50)
            *(char *)v50 = '\x01';
          if (((char)v51) && (!*(char *)((long)v2 + 0x3f))) {
            v10 = *(unsigned long *)&v2[0x12];
            *(unsigned long *)&v23[-8] = 0xab17;
            sub_ea00(v10,v53,&v17);
            v36 = (long *)(v51 & 0xff);
            return (unsigned long)v36 & 0xffffffff;
          }
        }
        v36 = (long *)0x1;
        return (unsigned long)v36 & 0xffffffff;
      }
      if (!(char)v2[0xc]) goto label_9ad8;
      if (!*(char *)((long)v2 + 0x17)) {
label_9f34:
        if ((2 <= v57) || ((((char)v51 && (v2[1] == 3)) || (v41 = 0, v2[1] == 4)))) {
          *(unsigned long *)&v23[-8] = 0xa95a;
          v41 = sub_bf60(v53,v56,v17);
          goto label_a961;
        }
        goto label_9f6b;
      }
      v41 = 0;
label_9f73:
      v34 = v22;
      v38 = v22;
      goto label_9aee;
    }
    if ((char)v51) { // branch-flip
      *(unsigned long *)&v23[-8] = 0xa92f;
      v41 = sub_bf60(v53,v56,v17);
    }
    else {
      *(unsigned long *)&v23[-8] = 0x96c4;
      v41 = sub_bf00(v56,v17);
    }
    if (!v41) {
label_9740:
      v3 = (char)v2[6];
label_9f6b:
      if (v3) goto label_97c5;
      goto label_9f73;
    }
label_96d5:
    *(unsigned long *)&v23[-8] = 0x96ef;
    v3 = sub_15130(0xffffff9c,a0,v54,v41);
    if (v3) { // branch-flip
      *(unsigned long *)&v23[-8] = 0xb206;
      v10 = sub_14a70(1,4,dat_25568);
      *(unsigned long *)&v23[-8] = 0xb21c;
      v11 = sub_14a70(0,4,dat_25570);
      *(unsigned long *)&v23[-8] = 0xb232;
      v9 = dcgettext(NULL,"cannot copy a directory, %s, into itself, %s",5,v23[-8]);
      *(unsigned long *)&v23[-8] = 0xb246;
      error(0,0,v9,v11,v10);
      *v45 = 1;
    }
    else {
      *(unsigned long *)&v23[-8] = 0x9712;
      v3 = sub_15130(v54,v53,v54,v41);
      v14 = v52;
      if (v3) {
        *(unsigned long *)&v23[-8] = 0xb6de;
        v10 = sub_14b40(4,dat_25570);
        *(unsigned long *)&v23[-8] = 0xb6f4;
        v9 = dcgettext(NULL,"warning: source directory %s specified more than once",5,v23[-8]);
        *(unsigned long *)&v23[-8] = 0xb705;
        error(0,0,v9,v10);
        v34 = v50 != NULL & *(unsigned char *)&v2[6];
        v36 = (long *)CONCAT71((undefined7)((unsigned long)v36 >> 8),v34);
        if (v34) {
label_aaba:
          *(char *)v50 = '\x01';
          return (unsigned long)v36 & 0xffffffff;
        }
        v36 = (long *)0x1;
        return (unsigned long)v36 & 0xffffffff;
      }
      if ((v2[1] == 4) || ((v2[1] == 3 && ((char)v51)))) goto label_9740;
      *(unsigned long *)&v23[-8] = 0xb51e;
      v13 = (void *)sub_65e0(v52,v53,v41);
      *(unsigned long *)&v23[-8] = 0xb533;
      v10 = sub_14a70(1,4,v13);
      *(unsigned long *)&v23[-8] = 0xb545;
      v11 = sub_14a70(0,4,v14);
      *(unsigned long *)&v23[-8] = 0xb55b;
      v9 = dcgettext(NULL,"will not create hard link %s to directory %s",5,v23[-8]);
      *(unsigned long *)&v23[-8] = 0xb56f;
      error(0,0,v9,v11,v10);
      *(unsigned long *)&v23[-8] = 0xb577;
      free(v13,v23[-8]);
    }
label_a9c9:
    v24 = v23;
    if (*(char *)((long)v2 + 0x33)) {
label_a9d3:
      *(unsigned long *)&v23[-8] = 0xa9d8;
      sub_7380();
label_a562:
      v24 = v23;
      if (!v41) {
        *(unsigned long *)&v23[-8] = 0xac8b;
        sub_bea0(v56,v17);
      }
    }
    if (v49) {
      *(unsigned long *)&v24[-8] = 0xa5a1;
      v32 = renameat(v54,(char *)((long)v49 + ((long)v53 - (long)v52)));
      if (v32) {
        *(unsigned long *)&v24[-8] = 0xac61;
        v10 = sub_14b40(4,v52);
        v9 = "cannot un-backup %s";
        goto label_a459;
      }
      if ((char)v2[0xf]) {
        *(unsigned long *)&v24[-8] = 0xa5c9;
        v10 = sub_14a70(1,4,v52);
        *(unsigned long *)&v24[-8] = 0xa5df;
        v11 = sub_14a70(0,4,v49);
        *(unsigned long *)&v24[-8] = 0xa5f5;
        v9 = dcgettext(NULL,"%s -> %s (unbackup)\n",5,v24[-8]);
        *(unsigned long *)&v24[-8] = 0xa60a;
        __printf_chk(1,v9,v11,v10);
      }
    }
  }
  else {
label_9668:
    if (((char)v2[0xf]) && (!(char)v2[6])) {
label_9f88:
      if ((v55 & 0xf000) != 0x4000) {
        *(unsigned long *)&v23[-8] = 0x9fb4;
        sub_73c0(a0,v52,v49);
      }
    }
    if (v32) goto label_9681;
    if ((char)v2[6]) goto label_9a66;
label_9ad8:
    v41 = 0;
    v34 = v22;
    v38 = v22;
label_9aee:
    v35 = v55;
    v15 = v55;
    if (*(char *)((long)v2 + 0x39))
      v15 = v2[4];
    v50 = (long *)(CONCAT44(v50._4_4_,v15) & 0xffffffff00000fff);
    v25 = v23;
    if (*(char *)((long)v2 + 0x1d)) { // branch-flip
      *(unsigned long *)&v23[-8] = 0x9b8f;
      v3 = sub_7ff0(a0,v52,v55,v38,v2);
      if (v3) {
        v42 = (unsigned int)v50 & 0x3f;
        if ((v35 & 0xf000) == 0x4000) {
label_a06a:
          v12 = (unsigned long)(v35 & 0xf000);
          v14 = &v17;
          *(unsigned long *)&v23[-8] = 0xa080;
          v43 = sub_61e0(v14,v48);
          if (v43) { // branch-flip
            *(unsigned long *)&v23[-8] = 0xae5d;
            v10 = sub_14b40(4,a0);
            v9 = "cannot copy cyclic symbolic link %s";
label_ae6c:
            *(unsigned long *)&v23[-8] = 0xae73;
            v9 = dcgettext(NULL,v9,5,v23[-8]);
            *(unsigned long *)&v23[-8] = 0xae84;
            error(0,0,v9,v10);
          }
          else {
            v26 = &v23[-0x20];
            v24 = &v23[-0x20];
            v25 = &v23[-0x20];
            *(unsigned long *)&v23[-8] = *(unsigned long *)&v23[-8];
            v47 = (unsigned long *)((unsigned long)&v23[-0x11] & 0xfffffffffffffff0);
            *v47 = v48;
            v47[1] = v56;
            v47[2] = v17;
            if ((!v34) && ((v65 & 0xf000) == 0x4000)) {
              if ((*(long *)&v2[10]) || (*(char *)((long)v2 + 0x33))) {
                v50 = v14;
                *(unsigned long *)&v23[-0x28] = 0xb053;
                v3 = sub_8250(v52,0,v2);
                if ((!v3) && (v25 = &v23[-0x20], (char)v2[0xd])) goto label_a558;
              }
              v42 = 0;
              v30 = 0;
label_a240:
              v4 = v44 != NULL & *(unsigned char *)&v2[7];
              v36 = (long *)CONCAT71((undefined7)(v12 >> 8),v4);
              if ((!v4) || (*v44 == v17)) {
                *(char **)&v23[-0x28] = v45;
                *(char **)&v23[-0x30] = v46;
                *(int **)&v23[-0x38] = v2;
                *(unsigned long **)&v23[-0x40] = v47;
                v50 = (long *)CONCAT71(v50._1_7_,v30);
                *(unsigned long *)&v23[-0x48] = 0xa291;
                v15 = sub_bbc0(a0,v52,v54,v53,v38);
                v36 = (long *)(unsigned long)v15;
                v30 = (char)v50;
              }
              if ((char)v51) {
                v48 = (long *)((unsigned long)v48._1_7_ << 8);
                v38 = 0;
                v47 = (unsigned long *)CONCAT71(v47._1_7_,v30);
                goto label_9c9d;
              }
              v48 = (long *)((unsigned long)v48._1_7_ << 8);
              v47 = (unsigned long *)CONCAT71(v47._1_7_,v30);
              goto label_9d14;
            }
            v48 = v14;
            *(unsigned long *)&v23[-0x28] = 0xa12c;
            v32 = mkdirat(v54,v53,~v42 & (unsigned int)v50);
            if (v32) { // branch-flip
              *(unsigned long *)&v23[-0x28] = 0xae39;
              v10 = sub_14b40(4,v52);
              v9 = "cannot create directory %s";
            }
            else {
              v50 = v48;
              *(unsigned long *)&v23[-0x28] = 0xa160;
              v32 = fstatat(v54,(char *)v53,&v18,0x100,v23[-0x28]);
              v15 = v65;
              if (!v32) {
                v12 = (unsigned long)v65;
                v30 = 0;
                if ((v65 & 0x1c0) != 0x1c0) {
                  *(unsigned long *)&v23[-0x28] = 0xa1aa;
                  v32 = sub_e520(v54,v53,v65 | 0x1c0);
                  v40 = v15;
                  v30 = 1;
                  if (v32) {
                    *(unsigned long *)&v23[-0x28] = 0xb76a;
                    v10 = sub_14b40(4,v52);
                    v9 = "setting permissions for %s";
                    v25 = &v23[-0x20];
                    goto label_a538;
                  }
                }
                v14 = v50;
                if (!*v46) {
                  v49 = v50;
                  v50 = (long *)CONCAT71(v50._1_7_,v30);
                  *(unsigned long *)&v23[-0x28] = 0xb305;
                  sub_bf60(v53,v63,v18);
                  *v46 = '\x01';
                  v14 = v49;
                  v30 = (char)v50;
                }
                if ((char)v2[0xf]) {
                  v49 = v14;
                  v50 = (long *)CONCAT71(v50._1_7_,v30);
                  if ((char)v2[6]) { // branch-flip
                    *(unsigned long *)&v23[-0x28] = 0xa205;
                    v12 = sub_14b40(4,v52);
                    *(unsigned long *)&v23[-0x28] = 0xa21b;
                    v9 = dcgettext(NULL,"created directory %s\n",5,v23[-0x28]);
                    *(unsigned long *)&v23[-0x28] = 0xa22d;
                    __printf_chk(1,v9,v12);
                    v30 = (char)v50;
                  }
                  else {
                    *(unsigned long *)&v23[-0x28] = 0xb4ed;
                    sub_73c0(a0,v52,0);
                    v30 = (char)v50;
                  }
                }
                goto label_a240;
              }
              *(unsigned long *)&v23[-0x28] = 0xb3f0;
              v10 = sub_14b40(4,v52);
              v9 = "cannot stat %s";
              v25 = &v23[-0x20];
            }
label_a538:
            *(unsigned long *)&v25[-8] = 0xa53f;
            v9 = dcgettext(NULL,v9,5,v25[-8]);
            *(unsigned long *)&v25[-8] = 0xa547;
            v30 = v25[-8];
            v32 = *__errno_location(v30);
            *(unsigned long *)&v25[-8] = 0xa558;
            error(0,v32,v9,v10);
          }
label_a558:
          v23 = v25;
          if (*(char *)((long)v2 + 0x33)) goto label_a9d3;
          goto label_a562;
        }
label_9bba:
        v35 &= 0xf000;
        v48 = (long *)CONCAT71(v48._1_7_,*(char *)((long)v2 + 0x3a));
        if (*(char *)((long)v2 + 0x3a)) { // branch-flip
          if ((char)*a0 != '/') {
            *(unsigned long *)&v23[-8] = 0x9bdf;
            v9 = (char *)sub_e730(v53);
            if (v54 != -100) { // branch-flip
label_9c13:
              *(unsigned long *)&v23[-8] = 0x9c1f;
              v30 = v23[-8];
              if (!stat(".",&v20,v30)) {
                *(unsigned long *)&v23[-8] = 0xb10f;
                v32 = fstatat(v54,v9,&v21,0,v23[-8]);
                if ((!v32) && ((v69 != v70 || (v20 != v21)))) {
                  *(unsigned long *)&v23[-8] = 0xb133;
                  free(v9,v23[-8]);
                  *(unsigned long *)&v23[-8] = 0xb146;
                  v10 = sub_14c40(0,3,v52);
                  v9 = "%s: can make relative symbolic links only in current directory";
                  goto label_ae6c;
                }
              }
            }
            else {
              v50 = (long *)0x1c7d1;
              *(unsigned long *)&v23[-8] = 0x9c08;
              v30 = v23[-8];
              if (strcmp(".",v9,v30)) goto label_9c13;
            }
            *(unsigned long *)&v23[-8] = 0x9c2f;
            free(v9,v23[-8]);
          }
          v30 = *(char *)((long)v2 + 0x16);
          *(unsigned long *)&v23[-8] = 0x9c4e;
          v32 = sub_c230(a0,v54,v53,v30,0xffffffff);
          v43 = 0;
          if (v32 > 0) {
            *(unsigned long *)&v23[-8] = 0xada2;
            v10 = sub_14a70(1,4,a0);
            *(unsigned long *)&v23[-8] = 0xadb8;
            v11 = sub_14a70(0,4,v52);
            *(unsigned long *)&v23[-8] = 0xadce;
            v9 = dcgettext(NULL,"cannot create symbolic link %s to %s",5,v23[-8]);
            *(unsigned long *)&v23[-8] = 0xade3;
            error(0,v32,v9,v11,v10);
            goto label_a558;
          }
        }
        else if (*(char *)((long)v2 + 0x17)) { // branch-flip
          v12 = 1;
          if (!*(char *)((long)v2 + 0x16))
            v12 = (unsigned long)(v2[2] == 3);
          *(unsigned long *)&v23[-0x10] = (unsigned long)v43;
          *(unsigned long *)&v23[-0x18] = 0;
          *(unsigned long *)&v23[-0x20] = v12;
          *(unsigned long *)&v23[-0x28] = 0xa8b8;
          v3 = sub_7460(a0,0xffffff9c,a0,v52,v54,v53);
          v43 = 0;
          if (!v3) goto label_a558;
        }
        else if ((v35 != 0x8000) && (v43 = v35 != 0xa000 & *(unsigned char *)&v2[5], !v43)) {
          if (v35 != 0x1000) { // branch-flip
            if ((v35 >> 8 & 0xbf) != 0x20 && v35 != 0xc000) { // branch-flip
              if (v35 != 0xa000) {
                *(unsigned long *)&v23[-8] = 0xb734;
                v10 = sub_14b40(4,a0);
                v9 = "%s has unknown file type";
                goto label_ae6c;
              }
              *(unsigned long *)&v23[-8] = 0xb33f;
              v9 = (char *)sub_cbf0(a0,v62);
              if (!v9) {
                *(unsigned long *)&v23[-8] = 0xb990;
                v10 = sub_14b40(4,a0);
                v9 = "cannot read symbolic link %s";
                goto label_a538;
              }
              v30 = *(char *)((long)v2 + 0x16);
              *(unsigned long *)&v23[-8] = 0xb36a;
              v32 = sub_c230(v9,v54,v53,v30,0xffffffff);
              v50 = (long *)CONCAT44(v50._4_4_,v32);
              if (1 <= v32) {
                if (((*(char *)((long)v2 + 0x3b) == '\x01') && (!v34)) && ((v65 & 0xf000) == 0xa000)) {
                  v48 = v68;
                  *(unsigned long *)&v23[-8] = 0xb9d0;
                  v30 = v23[-8];
                  if (v48 == (long *)strlen(v9,v30)) {
                    *(unsigned long *)&v23[-8] = 0xb9f2;
                    v14 = (long *)sub_cd80(v54,v53);
                    if (v14) {
                      v48 = v14;
                      *(unsigned long *)&v23[-8] = 0xba0d;
                      v30 = v23[-8];
                      if (!strcmp((char *)v14,v9,v30)) {
                        *(unsigned long *)&v23[-8] = 0xba1d;
                        free(v48,v23[-8]);
                        *(unsigned long *)&v23[-8] = 0xba25;
                        free(v9,v23[-8]);
                        goto label_b8b9;
                      }
                      *(unsigned long *)&v23[-8] = 0xba87;
                      free(v48,v23[-8]);
                    }
                  }
                }
                *(unsigned long *)&v23[-8] = 0xb38f;
                free(v9,v23[-8]);
                *(unsigned long *)&v23[-8] = 0xb3a0;
                v10 = sub_14b40(4,v52);
                *(unsigned long *)&v23[-8] = 0xb3b6;
                v9 = dcgettext(NULL,"cannot create symbolic link %s",5,v23[-8]);
                *(unsigned long *)&v23[-8] = 0xb3cb;
                error(0,(int)v50,v9,v10);
                goto label_a558;
              }
              *(unsigned long *)&v23[-8] = 0xb8b9;
              free(v9,v23[-8]);
label_b8b9:
              if (*(char *)((long)v2 + 0x33)) {
                *(unsigned long *)&v23[-8] = 0xb97e;
                sub_7380();
              }
              v48 = (long *)CONCAT71(v48._1_7_,1);
              v43 = *(unsigned char *)((long)v2 + 0x1d);
              if (*(unsigned char *)((long)v2 + 0x1d)) {
                *(unsigned long *)&v23[-8] = 0xb8fa;
                v32 = sub_e540(v54,v53,v59,v60);
                if (v32) {
                  *(unsigned long *)&v23[-8] = 0xb90a;
                  v3 = sub_8400(v2);
                  v48 = (long *)CONCAT71(v48._1_7_,v3);
                  if (v3) {
                    v43 = 0;
                    goto label_9c60;
                  }
                  *(unsigned long *)&v23[-8] = 0xba3d;
                  v9 = dcgettext(NULL,"failed to preserve ownership for %s",5,v23[-8]);
                  *(unsigned long *)&v23[-8] = 0xba45;
                  v30 = v23[-8];
                  v32 = *__errno_location(v30);
                  *(unsigned long *)&v23[-8] = 0xba5a;
                  error(0,v32,v9,v52);
                  if (*(char *)((long)v2 + 0x32)) goto label_a558;
                }
                v38 = v43;
                v43 = 0;
                v48 = (long *)CONCAT71(v48._1_7_,v38);
              }
            }
            else {
              *(unsigned long *)&v23[-8] = 0xa50c;
              v32 = mknodat(v54,v53,~v42 & v55,v61);
              if (v32) {
                *(unsigned long *)&v23[-8] = 0xa525;
                v10 = sub_14b40(4,v52);
                v9 = "cannot create special file %s";
                goto label_a538;
              }
            }
          }
          else {
            v15 = ~v42 & v55;
            *(unsigned long *)&v23[-8] = 0xb605;
            v32 = mknodat(v54,v53,v15,0);
            if (v32) {
              *(unsigned long *)&v23[-8] = 0xb625;
              v32 = mkfifoat(v54,v53,v15 & 0xffffefff);
              if (v32) {
                *(unsigned long *)&v23[-8] = 0xb63e;
                v10 = sub_14b40(4,v52);
                v9 = "cannot create fifo %s";
                goto label_a538;
              }
            }
          }
        }
        else {
          *(long **)&v23[-0x10] = &v17;
          *(bool **)&v23[-0x18] = &v22;
          *(unsigned long *)&v23[-0x20] = (unsigned long)v42;
          *(unsigned long *)&v23[-0x28] = 0xad60;
          v43 = sub_8680(a0,v52,v54,v53,v2,(unsigned int)v50 & 0x1ff);
          if (!v43) goto label_a558;
          v48 = (long *)((unsigned long)v48 & 0xffffffffffffff00);
          v34 = v22;
        }
label_9c60:
        v38 = v35 != 0x4000;
        if (!v34) {
          v4 = (*(unsigned char *)&v2[5] ^ 1) & v38;
          if (v4) {
            if ((*(long *)&v2[10]) || (*(char *)((long)v2 + 0x33))) {
              *(unsigned long *)&v23[-8] = 0xa9fc;
              v38 = sub_8250(v52,0,v2);
              if (v38) goto label_9c83;
              v34 = 0;
              if ((char)v2[0xd]) goto label_a558;
            }
            v38 = v4;
          }
        }
label_9c83:
        v36 = (long *)0x1;
        v24 = v23;
        if ((char)v51) {
          v47 = (unsigned long *)((unsigned long)v47 & 0xffffffffffffff00);
          v36 = (long *)(v51 & 0xffffffff);
          v26 = v23;
label_9c9d:
          v24 = v26;
          if (*(long *)&v2[0x12]) {
            v51 = CONCAT71(v51._1_7_,v38);
            *(unsigned long *)&v26[-8] = 0x9ccc;
            v32 = fstatat(v54,(char *)v53,&v21,0x100,v26[-8]);
            v38 = (unsigned char)v51;
            if (!v32) {
              v10 = *(unsigned long *)&v2[0x12];
              *(unsigned long *)&v26[-8] = 0x9ceb;
              sub_ea00(v10,v53,&v21);
              v38 = (unsigned char)v51;
            }
          }
        }
        if (((*(char *)((long)v2 + 0x17)) && (v38)) || (v43)) {
          return (unsigned long)v36 & 0xffffffff;
        }
label_9d14:
        if (*(char *)((long)v2 + 0x1f)) {
          *(unsigned long *)&v24[-8] = 0x9d2d;
          v39 = sub_15960(&v17);
          v21 = SUB168(v39,0);
          v70 = SUB168(v39,8);
          *(unsigned long *)&v24[-8] = 0x9d43;
          v39 = sub_15980(&v17);
          v72 = SUB168(v39,8);
          v71 = SUB168(v39,0);
          *(unsigned long *)&v24[-8] = 0x9d74;
          v32 = utimensat(v54,v53,&v21,((unsigned long)v48 & 0xff) << 8);
          if (v32) {
            *(unsigned long *)&v24[-8] = 0x9d89;
            v51 = sub_14b40(4,v52);
            *(unsigned long *)&v24[-8] = 0x9da3;
            v9 = dcgettext(NULL,"preserving times for %s",5,v24[-8]);
            *(unsigned long *)&v24[-8] = 0x9dab;
            v30 = v24[-8];
            v32 = *__errno_location(v30);
            *(unsigned long *)&v24[-8] = 0x9dc0;
            error(0,v32,v9,v51);
            if (*(char *)((long)v2 + 0x32)) {
              v36 = NULL;
              return (unsigned long)v36 & 0xffffffff;
            }
          }
        }
        if ((char)v48) { // branch-flip
          if (!*(char *)((long)v2 + 0x35)) {
            return (unsigned long)v36 & 0xffffffff;
          }
          *(unsigned long *)&v24[-8] = 0xa906;
          if (sub_7040(a0,0xffffffff,v52,0xffffffff,v2)) {
            return (unsigned long)v36 & 0xffffffff;
          }
          v3 = *(char *)((long)v2 + 0x36);
        }
        else {
          if (*(char *)((long)v2 + 0x1d)) {
            if (((v34) || (v66 != v59)) || (v67 != v60)) {
              *(long **)&v24[-8] = &v18;
              *(unsigned long *)&v24[-0x10] = (unsigned long)v34;
              *(unsigned long *)&v24[-0x18] = (unsigned long)v60;
              *(unsigned long *)&v24[-0x20] = (unsigned long)v59;
              *(unsigned long *)&v24[-0x28] = 0x9e38;
              v32 = sub_8430(v2,v52,v54,v53,0xffffffff,v58);
              if (v32 == -1) {
                v36 = NULL;
                return (unsigned long)v36 & 0xffffffff;
              }
              if (!v32)
                v55 &= 0xfffff1ff;
            }
          }
          if (*(char *)((long)v2 + 0x35)) {
            *(unsigned long *)&v24[-8] = 0xb45f;
            v3 = sub_7040(a0,0xffffffff,v52,0xffffffff,v2);
            if ((!v3) && (*(char *)((long)v2 + 0x36))) {
              v36 = NULL;
              return (unsigned long)v36 & 0xffffffff;
            }
          }
          if (!(*(unsigned long *)&v2[6] & 0xff0000000000ff)) {
            if (*(char *)((long)v2 + 0x39)) // branch-flip
              v15 = v2[4];
            else {
              if (!(v34 & *(unsigned char *)&v2[8])) {
                if (v42) { // branch-flip
                  *(unsigned long *)&v24[-8] = 0xb65d;
                  v42 &= ~sub_8640();
                  if (!v42) goto label_9e9a;
                  if ((char)v47 != '\x01') {
                    if (v34) {
                      *(unsigned long *)&v24[-8] = 0xa42f;
                      v32 = fstatat(v54,(char *)v53,&v18,0x100,v24[-8]);
                      if (v32) {
                        v36 = v52;
                        goto label_a440;
                      }
                    }
                    v40 = v65;
                    if (!(v42 & ~v65)) {
                      return (unsigned long)v36 & 0xffffffff;
                    }
                  }
                }
                else {
label_9e9a:
                  if (!(char)v47) {
                    return (unsigned long)v36 & 0xffffffff;
                  }
                }
                *(unsigned long *)&v24[-8] = 0x9ec5;
                v32 = sub_e520(v54,v53,v40 | v42);
                if (!v32) {
                  return (unsigned long)v36 & 0xffffffff;
                }
                *(unsigned long *)&v24[-8] = 0x9ede;
                v10 = sub_14b40(4,v52);
                *(unsigned long *)&v24[-8] = 0x9ef4;
                v9 = dcgettext(NULL,"preserving permissions for %s",5,v24[-8]);
                *(unsigned long *)&v24[-8] = 0x9efc;
                v30 = v24[-8];
                v32 = *__errno_location(v30);
                *(unsigned long *)&v24[-8] = 0x9f0d;
                error(0,v32,v9,v10);
                goto label_9f0d;
              }
              v15 = 0x1ff;
              if ((v55 & 0x7000) != 0x4000)
                v15 = 0x1b6;
              *(unsigned long *)&v24[-8] = 0xb6c2;
              v15 = ~sub_8640() & v15;
            }
            *(unsigned long *)&v24[-8] = 0xb2d0;
            if (!sub_cb50(v52,0xffffffff,v15)) {
              return (unsigned long)v36 & 0xffffffff;
            }
            v36 = NULL;
            return (unsigned long)v36 & 0xffffffff;
          }
          *(unsigned long *)&v24[-8] = 0xb1c0;
          if (!sub_caa0(a0,0xffffffff,v52,0xffffffff,v55)) {
            return (unsigned long)v36 & 0xffffffff;
          }
label_9f0d:
          v3 = *(char *)((long)v2 + 0x32);
        }
        if (!v3) {
          return (unsigned long)v36 & 0xffffffff;
        }
      }
    }
    else if ((v55 & 0xf000) != 0x4000) { // branch-flip
      *(unsigned long *)&v23[-8] = 0x9b4f;
      v3 = sub_7ff0(a0,v52,v55,v38,v2);
      v42 = 0;
      if (v3) goto label_9bba;
    }
    else {
      *(unsigned long *)&v23[-8] = 0xa053;
      v3 = sub_7ff0(a0,v52,v55,v38,v2);
      if (v3) {
        v42 = (unsigned int)v50 & 0x12;
        goto label_a06a;
      }
    }
  }
  v36 = NULL;
  return (unsigned long)v36 & 0xffffffff;
}

// Function: sub_bae0 @ 0xbae0
void sub_bae0(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long a3,unsigned int a4,unsigned long a5,unsigned long a6,unsigned long a7)
{
  unsigned long v1;
  unsigned long v2;
  char v3; // stack - 0x41
  
  v2 = a7;
  v1 = a6;
  if (sub_6270(a5)) {
    v3 = 0;
    dat_25568 = a1;
    dat_25570 = a0;
    sub_9470(a0,a1,a2,a3,a4,0,0,a5,1,&v3,v1,v2);
    return;
  }
  __assert_fail("valid_options (options)","src/copy.c",0xc2a,"copy"); // no-return
}

// Function: sub_bbc0 @ 0xbbc0
unsigned char sub_bbc0(unsigned long a0,long a1,unsigned int a2,long a3,char a4,unsigned long a5,unsigned long a6,unsigned long *a7,unsigned char *a8,unsigned char *a9)
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
        v7 = (void *)sub_ec90(a0,v14,0);
        v8 = (void *)sub_ec90(a1,v14,0);
        v12 = *v2;
        v5 = sub_9470(v7,v8,a2,(long)v8 + (a3 - a1),a4,a5,v9,&v13,0,&v12,&v11,&v10);
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
unsigned long sub_be60(long *a0,long *a1)
{
  if (*a0 != *a1)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1[1] >> 8),a0[1] == a1[1]);
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
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = sub_12350(dat_25578,&v2);
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
unsigned long sub_bf60(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  unsigned long *v3; // rax
  
  v1 = (unsigned long *)sub_16e90(0x18);
  v2 = sub_17230(a0);
  *v1 = a1;
  v1[2] = v2;
  v1[1] = a2;
  v3 = (unsigned long *)sub_122f0(dat_25578,v1);
  if (!v3)
    sub_17250(); // no-return
  if (v1 == v3)
    return 0;
  sub_be80(v1);
  return v3[2];
}

// Function: sub_bfe0 @ 0xbfe0
void sub_bfe0(void)
{
  dat_25578 = sub_11cd0(0x67,0,sub_be50,sub_be60,sub_be80);
  if (dat_25578)
    return;
  sub_17250(); // no-return
}

// Function: sub_c020 @ 0xc020
void sub_c020(unsigned long a0,unsigned int *a1)
{
  linkat(*a1,*(unsigned long *)&a1[2],a1[4],a0,a1[5]); // tail-call
}

// Function: sub_c040 @ 0xc040
void * sub_c040(void *a0,void *a1)
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
void sub_c0a0(unsigned long a0,unsigned long *a1)
{
  symlinkat(*a1,*(unsigned int *)&a1[1],a0); // tail-call
}

// Function: sub_c0c0 @ 0xc0c0
int sub_c0c0(unsigned long a0,unsigned long a1,int a2,unsigned long a3,unsigned int a4,char a5,int a6)
{
  char *v1;
  int v2; // eax
  int *v3; // rax
  char v4 [264];
  unsigned int v5 [2]; // stack - 0x168
  int v6;
  unsigned long v7; // stack - 0x160
  int v8; // stack - 0x158
  unsigned int v9; // stack - 0x154
  
  v2 = a6;
  if (a6 < 0) {
    v2 = linkat(a0,a1,a2);
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
      v5[0] = (unsigned int)a0;
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
int sub_c230(unsigned long a0,int a1,unsigned long a2,char a3,int a4)
{
  char *v1; // rax
  unsigned long v2; // stack - 0x158
  char v3 [264];
  int v4; // stack - 0x150
  
  if (a4 < 0) {
    if (!symlinkat())
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
unsigned int sub_c370(unsigned long a0,unsigned int a1,unsigned long a2)
{
  int v1;
  short v2; // ax
  unsigned int v3; // eax
  void *v4; // rax
  int *v5; // rax
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v6 = 0;
  v7 = 0;
  v4 = (void *)sub_e730();
  if (((0 <= (int)getcon(&v6)) && (0 <= (int)sub_158c0(v4,&v7))) && (v2 = mode_to_security_class(a1), v2))
    v3 = security_compute_create(v6,v7,v2,a2);
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
  long v10; // stack - 0xe0
  unsigned int v11; // stack - 0xc0
  int v2;
  int v3;
  int *v4; // rax
  long v5; // rax
  long v6; // rax
  long v7; // rax
  unsigned long v8; // stack - 0xe8
  char v9 [24];
  
  v8 = 0;
  v10 = 0;
  if (!a0) {
    if ((int)getfscreatecon(&v10) < 0)
      return -1;
    v4 = __errno_location();
    if (v10) {
      v2 = lsetfilecon(a1,v10);
      v1 = *v4;
      freecon(v10);
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
      if (0 <= lstat(a1,v9)) goto label_c4cd;
      v2 = *v4;
      v6 = 0;
      v5 = 0;
    }
label_c59b:
    v3 = v1;
    goto label_c59e;
  }
  if (0 <= fstat(v1,v9)) { // branch-flip
label_c4cd:
    if (0 <= (int)selabel_lookup(a0,&v8,a1,v11)) { // branch-flip
      v5 = context_new(v8);
      if (v5) { // branch-flip
        if (v1 != -1) {
          if ((int)sub_15900(v1) < 0) {
            v6 = 0;
            v3 = -1;
            goto label_c6bb;
          }
          v6 = context_new(v10);
          if (v6) goto label_c531;
          v2 = *v4;
          v3 = -1;
          goto label_c55f;
        }
        if ((int)sub_158e0(a1,&v10) < 0) {
          v2 = *v4;
          v6 = 0;
          goto label_c59b;
        }
        v6 = context_new(v10);
        if (!v6) {
          v2 = *v4;
          v3 = -1;
          goto label_c59e;
        }
label_c531:
        v7 = context_type_get(v5);
        if (((v7) && (!context_type_set(v6,v7))) && (v7 = context_str(v6), v7)) {
          if (v1 == -1) {
            v1 = lsetfilecon(a1,v7);
            v2 = *v4;
            goto label_c59b;
          }
          v3 = fsetfilecon(v1);
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
  freecon(v8);
  freecon(v10);
  *v4 = v2;
  return v3;
}

// Function: sub_c760 @ 0xc760
unsigned int sub_c760(unsigned long a0,char *a1,unsigned int a2)
{
  unsigned int v1; // eax
  int *v2; // rax
  long v3;
  long v4; // rax
  long v5; // rax
  unsigned long v6; // stack - 0x50
  unsigned long v7; // stack - 0x48
  char *v8;
  int v9;
  
  v6 = 0;
  v7 = 0;
  v2 = __errno_location();
  if (*a1 != '/') { // branch-flip
    a1 = (char *)sub_e4c0(a1,2);
    if (!a1) {
      v9 = *v2;
      v8 = NULL;
      v4 = 0;
      v1 = 0xffffffff;
      v3 = 0;
      goto label_c869;
    }
    v8 = a1;
  }
  else {
    v8 = NULL;
  }
  if (0 <= (int)selabel_lookup(a0,&v6,a1,a2)) { // branch-flip
    if (0 <= (int)sub_c370(a1,a2,&v7)) { // branch-flip
      v3 = context_new(v6);
      if (v3) { // branch-flip
        v4 = context_new(v7);
        if ((((v4) && (v5 = context_type_get(v3), v5)) && (!context_type_set(v4,v5))) && (v5 = context_str(v4), v5)) {
          v1 = setfscreatecon(v5);
          v9 = *v2;
        }
        else {
          v9 = *v2;
          v1 = 0xffffffff;
        }
      }
      else {
        v9 = *v2;
        v4 = 0;
        v1 = 0xffffffff;
      }
    }
    else {
      v9 = *v2;
      v4 = 0;
      v3 = 0;
      v1 = 0xffffffff;
    }
  }
  else {
    v9 = *v2;
    if (v9 != 2) { // branch-flip
      v4 = 0;
      v3 = 0;
      v1 = 0xffffffff;
    }
    else {
      *v2 = 0x3d;
      v9 = 0x3d;
      v4 = 0;
      v3 = 0;
      v1 = 0xffffffff;
    }
  }
label_c869:
  context_free(v3);
  context_free(v4);
  freecon(v6);
  freecon(v7);
  free(v8);
  *v2 = v9;
  return v1;
}

// Function: sub_c960 @ 0xc960
unsigned long sub_c960(unsigned long a0,char *a1,char a2)
{
  unsigned long v1;
  int v10;
  unsigned long v11; // stack - 0x40
  int v2; // eax
  int *v3;
  long v4; // rax
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
    v4 = sub_17290(&v7,0x10,0);
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
        v1 = *(unsigned long *)(v4 + 0x20);
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
int sub_caa0(unsigned long a0,unsigned long a1,unsigned long a2)
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
  v2 = sub_14e00(a0);
  error(0,*__errno_location(),"%s",v2);
  return -2;
}

// Function: sub_cb50 @ 0xcb50
int sub_cb50(unsigned long a0,unsigned long a1,unsigned long a2)
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
void sub_cbe0(void)
{
  aligned_alloc(); // tail-call
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
char * sub_cd80(unsigned int a0,unsigned long a1,unsigned long a2)
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
        v2[v1] = 0;
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
void sub_d090(unsigned long a0,unsigned long a1,long a2)
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
void sub_d120(long *a0,void *a1,unsigned long a2)
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
      __fprintf_chk(stderr,1,", %s",sub_14e00(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_14e00(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_d220 @ 0xd220
long sub_d220(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
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
unsigned char sub_d310(unsigned long a0,long *a1,long a2,long a3,long a4,long *a5,unsigned long a6)
{
  char *v1;
  void *v10;
  long v11; // rcx
  char v12;
  long v13;
  char *v14;
  unsigned long v15;
  unsigned long v16;
  bool v17; // r8b
  unsigned long v18; // stack - 0x78
  long v19; // stack - 0x68
  unsigned int *v2;
  unsigned short v3;
  unsigned char v4;
  unsigned long v5;
  int v6;
  void *v7;
  long v8;
  unsigned short *v9;
  
  v10 = (void *)*a5;
  v13 = *a1;
  v11 = a3 - a4;
  if (v10) // branch-flip
    rewinddir(v10);
  else {
    v9 = (unsigned short *)(v13 + a4);
    v3 = *v9;
    *v9 = 0x2e;
    v10 = (void *)sub_12790(a0,v13,0,a6);
    v2 = (unsigned int *)((long)v9 + v11);
    if (!v10) {
      v6 = *__errno_location();
      *v9 = v3;
      *v2 = 0x7e317e2e;
      *(char *)&v2[1] = 0;
      return (v6 == 0xc) + 2;
    }
    *v9 = v3;
    *v2 = 0x7e317e2e;
    *(char *)&v2[1] = 0;
    *a5 = (long)v10;
  }
  v4 = 2;
  v18 = 1;
  v19 = a2;
label_d370:
  do {
    v7 = readdir(v10);
    v5 = a6;
    while( true ) {
      if (!v7) {
        *a1 = v13;
        return v4;
      }
      v14 = (char *)((long)v7 + 0x13);
      a6 = v5;
      if (strlen(v14) < v11 + 4U) break;
      if (memcmp((void *)(v13 + a4),v14,v11 + 2U)) goto label_d370;
      v14 = &v14[v11 + 2U];
      if (8 < (unsigned char)(*v14 - 0x31U)) goto label_d370;
      v17 = *v14 == '9';
      v12 = v14[1];
      v6 = (int)v12;
      v15 = 1;
      v16 = v15; // branch-flip
      if (10 <= (unsigned int)(v6 - 0x30U))
        v16 = 1;
      else {
        do {
          v16 += 1;
          v17 = (bool)(v17 & (char)v6 == '9');
          v12 = v14[v16];
          v6 = (int)v12;
          v15 = v16;
        } while ((unsigned int)(v6 - 0x30U) <= 9);
      }
      if (((v12 != '~') || (v14[v16 + 1])) || (((long)v15 <= (long)v18 && ((v18 != v15 || (1 <= memcmp((void *)(v13 + a3 + 2),v14,v16))))))) goto label_d370;
      v18 = v17 + v15;
      v8 = v18 + a3 + 4;
      if (v8 <= v19) // branch-flip
        v8 = v13;
      else {
        if (SCARRY8(v8 >> 1,v8))
          v19 = v8;
        else {
          v19 = (v8 >> 1) + v8;
        }
        v8 = sub_126b0(v13,v19);
        if (!v8) {
          *a1 = v13;
          return 3;
        }
      }
      v9 = (unsigned short *)(a3 + v8);
      *v9 = 0x7e2e;
      *(char *)&v9[1] = 0x30;
      v7 = memcpy((void *)((long)v9 + (unsigned long)v17 + 2),v14,v15 + 2);
      v14 = (char *)((long)v7 + (v16 - 1));
      v12 = *(char *)((long)v7 + (v16 - 1));
      while (v12 == '9') {
        *v14 = '0';
        v1 = &v14[-1];
        v14 = &v14[-1];
        v12 = *v1;
      }
      *v14 = v12 + '\x01';
      v7 = readdir(v10);
      v13 = v8;
      v5 = a6;
      v4 = v17;
    }
  } while( true );
}

// Function: sub_d640 @ 0xd640
unsigned long sub_d640(long a0,long a1,int a2,long *a3) // return-dupe
{
  unsigned short v1;
  int v2;
  unsigned short *v3; // rax
  long v4;
  int *v5; // rax
  long v6;
  
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
  v4 = (a0 + a1) - (long)v3;
  if (v6 <= v4)
    v4 = v6 + -1;
  *(char *)((long)v3 + v4) = 0x7e;
  *(char *)((long)v3 + v4 + 1) = 0;
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
void * sub_d7b0(int a0,void *a1,int a2,char a3) // return-dupe
{
  unsigned long v1;
  void *v10; // stack - 0x50
  void *v11; // stack - 0x58
  int v12; // stack - 0x5c
  unsigned long v13; // stack - 0x48
  void *v14;
  long v15; // r15
  void *v16; // stack - 0x78
  long v2;
  char v3; // al
  int v4;
  unsigned long v5; // rax
  unsigned long v6;
  void *v7;
  int *v8; // rax
  void *v9;
  
  v16 = (void *)sub_db60(a1);
  v15 = (long)v16 - (long)a1;
  v1 = v15 + sub_dbc0(v16);
  if (!dat_25580)
    sub_d750(0);
  v5 = strlen(dat_25580);
  v5 += 1;
  v6 = 9;
  if (9 <= (long)v5)
    v6 = v5;
  v2 = v1 + 1 + v6;
  v7 = (void *)sub_126a0(v2);
  if (!v7)
    return NULL;
  v10 = NULL;
  v12 = -100;
  v13 = 0;
  v11 = v7;
  do {
    memcpy(v7,a1,v1);
    if (a2 != 1) { // branch-flip
      v4 = sub_d310(a0,&v11,v2,v1,v15,&v10,&v12);
      v7 = v11;
      if (v4 != 2) { // branch-flip
        if (v4 == 3) {
          if (v10)
            closedir(v10);
          free(v11);
          *__errno_location() = 0xc;
          return NULL;
        }
        v3 = '\x01';
        if (v4 != 1) goto label_d8d8;
      }
      else if (a2 == 2) {
        a2 = 1;
        memcpy((void *)((long)v11 + v1),dat_25580,v5);
      }
      v3 = sub_d640(v7,v1,v12,&v13);
    }
    else {
      v3 = '\x01';
      memcpy((void *)((long)v7 + v1),dat_25580,v5);
    }
label_d8d8:
    if (!a3) {
label_da50:
      if (v10) {
        closedir(v10);
        return v7;
      }
      return v7;
    }
    if (0 <= v12) { // branch-flip
      v9 = (void *)((long)v7 + v15);
      v14 = v16;
      v4 = v12;
    }
    else {
      v9 = v7;
      v14 = a1;
      v4 = a0;
    }
    v4 = sub_14e40(v4,v14,v12,v9,a2 != 1);
    if (!v4) goto label_da50;
    v8 = __errno_location();
    v4 = *v8;
    if ((v4 != 0x11) || (v3 != '\x01')) {
      if (v10)
        closedir(v10);
      free(v7);
      *v8 = v4;
      return NULL;
    }
  } while( true );
}

// Function: sub_da90 @ 0xda90
void sub_da90(void)
{
  sub_d7b0(); // tail-call
}

// Function: sub_daa0 @ 0xdaa0
void sub_daa0(void)
{
  if (sub_d7b0())
    return;
  sub_17250(); // no-return
}

// Function: sub_dac0 @ 0xdac0
unsigned int sub_dac0(unsigned long a0,char *a1)
{
  long v1; // rax
  
  if (!a1)
    return 2;
  if (!*a1)
    return 2;
  v1 = sub_d220(a0,a1,0x248a0,0x1d560,4,dat_25028);
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
  char v3 [16];
  char v4 [16];
  unsigned long v5;
  unsigned long v6; // rdx
  unsigned long v7;
  
  if (a0) { // branch-flip
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
        return v5;
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
void sub_dc70(long *a0)
{
  a0[1] = 0x400;
  *a0 = (long)&a0[2];
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
unsigned int sub_dd00(long *a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2;
  
  v2 = *a0;
  if (!v2) {
    v2 = sub_11cd0(7,0,sub_12540,sub_12570,sub_125b0);
    *a0 = v2;
    if (!v2)
      sub_17250(0); // no-return
  }
  v1 = sub_ea90(v2,a1,a2);
  if (!(char)v1)
    sub_ea00(*a0,a1,a2);
  return v1;
}

// Function: sub_dd90 @ 0xdd90
unsigned int sub_dd90(unsigned long a0)
{
  int v1; // eax
  
  v1 = faccessat(0xffffff9c,a0,0,0x200);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_ddc0 @ 0xddc0
void sub_ddc0(unsigned long a0,unsigned short *a1)
{
  *a1 = dat_1d580;
  sub_dd90(); // tail-call
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
  unsigned long *v10; // rdx
  unsigned int v11; // ebx
  char *v12; // stack - 0x868
  char *v13; // stack - 0x458
  char v14 [144];
  long v15; // stack - 0x900
  char *v16;
  char *v17; // r13
  char *v18;
  char *v19;
  int v2;
  bool v20; // cf
  char *v21; // stack - 0x960
  char *v22; // stack - 0x950
  int v23; // stack - 0x92c
  long v24; // stack - 0x920
  unsigned long v25; // stack - 0x860
  long v26; // stack - 0x450
  bool v3;
  char *v4;
  char v5;
  long v6;
  unsigned long v7; // rax
  char *v8;
  unsigned long v9;
  
  v11 = a1 & 3;
  v15 = 0;
  if ((sub_dcf0(v11)) || (!a0)) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!*a0) {
    *__errno_location() = 2;
    return 0;
  }
  sub_dc70(&v12);
  sub_dc70(&v13);
  sub_dc70(v10);
  v19 = (char *)*v10;
  if (*a0 != '/') { // branch-flip
    while (v9 = v10[1], !getcwd(v19,v9)) {
      v2 = *__errno_location();
      if (v2 == 0xc) {
        sub_17250(); // no-return, return-dupe
      }
      if (v2 != 0x22) goto label_deb6;
      if (!sub_15710(v10)) {
        sub_17250();
      }
      v19 = (char *)*v10;
    }
    v8 = rawmemchr(v19,0);
    v5 = *a0;
    v21 = v8;
    if (v5) goto label_df64;
    v6 = 0;
  }
  else {
    *v19 = '/';
    v5 = *a0;
    v21 = &v19[1];
    if (!v5) {
      v22 = &v19[2];
      goto label_e17b;
    }
label_df64:
    v23 = 0;
    v3 = 0;
    do {
      v8 = v21;
      if (v5 == '/') {
        do {
          v5 = a0[1];
          a0 = &a0[1];
        } while (v5 == '/');
        if (!v5) break;
      }
      v18 = a0;
      do {
        v17 = v18;
        v1 = v17[1];
        v18 = &v17[1];
        if (!v1) break;
      } while (v1 != '/');
      v9 = (long)v18 - (long)a0;
      if (!v9) break;
      if (v9 != 1) { // branch-flip
        if (((v9 != 2) || (v5 != '.')) || (a0[1] != '.')) {
label_e035:
          if (v21[-1] != '/') {
            *v21 = '/';
            v21 = &v21[1];
          }
          if (&v19[v10[1] - (long)v21] < (char *)(v9 + 2)) {
            do {
              v6 = (long)v21 - (long)v19;
              if (!sub_15790(v10)) {
                sub_17250();
              }
              v19 = (char *)*v10;
              v21 = &v19[v6];
            } while ((char *)(v10[1] - v6) < (char *)(v9 + 2));
          }
          v8 = mempcpy(v21,a0,v9);
          *v8 = '\0';
          if (a1 & 4) { // branch-flip
label_e0bd:
            if (v11 != 2) {
              if (sub_dc80(v18)) { // branch-flip
                v5 = sub_ddc0(v19,v8);
label_e246:
                if (v5 != '\x01') {
label_e0ea:
                  if (((v11 != 1) || (*__errno_location() != 2)) || (v18[strspn(v18,"/")])) {
label_e320:
                    if (v15) { // branch-flip
                      sub_11e90(v15);
                      sub_ddd0(&v12);
                      sub_ddd0(&v13);
                    }
                    else {
label_deb6:
                      sub_ddd0(&v12);
                      sub_ddd0(&v13);
                    }
                    sub_ddd0(v10);
                    return 0;
                  }
                }
              }
              else if (a1 & 4) { // branch-flip
                if (!*v18) {
                  v5 = sub_dd90(v19);
                  goto label_e246;
                }
              }
              else if (*__errno_location() != 0x16) goto label_e0ea;
            }
          }
          else {
            while( true ) {
              v4 = v13;
              v9 = v26 - 1;
              v7 = readlink(v19,v13,v9);
              if ((long)v9 > (long)v7) break;
              if (!sub_15710(&v13)) {
                sub_17250();
              }
            }
            if ((long)v7 < 0) goto label_e0bd;
            if (v23 <= 0x13) {
              v23 += 1;
label_e362:
              a0 = v12;
              v4[v7] = '\0';
              v6 = (long)v18 - (long)v12;
              if (!v3)
                v6 = v24;
              v22 = (unsigned long)strlen(v18);
              if (!SCARRY8((long)v22 + 0x8000000000000000,v7)) {
                if (v25 <= v7 + (long)v22) {
                  do {
                    if (!sub_15790(&v12)) {
                      sub_17250();
                    }
                  } while (v25 <= v7 + (long)v22);
                  a0 = v12;
                }
                if (v3)
                  v18 = &a0[v6];
                memmove(&a0[v7],v18,(long)v22 + 1);
                memcpy(a0,v4,v7);
                v21 = &v19[1];
                v24 = v6;
                if (*v4 != '/') { // branch-flip
                  v3 = 1;
                  v5 = *a0;
                  v20 = v21 < v8;
                  v21 = v8;
                  if (v20) {
                    do {
                      v21 = &v8[-1];
                      if (v21 == v19) break;
                      v18 = &v8[-2];
                      v8 = v21;
                    } while (*v18 != '/');
                    v3 = 1;
                  }
                }
                else {
                  *v19 = '/';
                  v5 = *a0;
                  v3 = 1;
                }
                goto label_e12a;
              }
              sub_17250();
            }
            if (!*a0) goto label_e362;
            v8[(long)a0 - (long)v18] = '\0';
            v16 = ".";
            if (*v19)
              v16 = v19;
            if (stat(v16,v14)) goto label_e320;
            v8[(long)a0 - (long)v18] = *a0;
            if (!sub_dd00(&v15,a0,v14)) goto label_e362;
            if (v11 != 2) {
              *__errno_location() = 0x28;
              goto label_e320;
            }
          }
          v5 = v17[1];
          a0 = v18;
          v21 = v8;
        }
        else {
          a0 = v18;
          v5 = v1;
          if (&v19[1] < v21) {
            v21 = &v21[-1];
            if (v19 < v21) {
              do {
                if (v21[-1] == '/') break;
                v21 = &v21[-1];
              } while (v19 != v21);
              a0 = v18; // crossjump-dupe
              v5 = v1;
            }
          }
        }
      }
      else {
        if (v5 != '.') goto label_e035;
label_e028:
        a0 = v18;
        v5 = v1;
      }
label_e12a:
      v8 = v21;
    } while (v5);
    v6 = v15;
  }
  v22 = &v8[1];
  v21 = v8;
  if ((&v19[1] < v8) && (v8[-1] == '/')) {
    v21 = &v8[-1];
    v22 = v8;
  }
  if (v6)
    sub_11e90(v6);
label_e17b:
  sub_ddd0(&v12);
  sub_ddd0(&v13);
  *v21 = '\0';
  v6 = sub_15680(v10,(long)v22 - (long)v19);
  if (v6)
    return v6;
  sub_17250();
}

// Function: sub_e4c0 @ 0xe4c0
void sub_e4c0(unsigned long a0,unsigned long a1)
{
  char v1 [1048];
  
  sub_ddf0(a0,a1,v1);
}

// Function: sub_e510 @ 0xe510
void sub_e510(void)
{
  fchmodat(); // tail-call
}

// Function: sub_e520 @ 0xe520
void sub_e520(void)
{
  fchmodat(); // tail-call
}

// Function: sub_e530 @ 0xe530
void sub_e530(void)
{
  fchownat(); // tail-call
}

// Function: sub_e540 @ 0xe540
void sub_e540(void)
{
  fchownat(); // tail-call
}

// Function: sub_e550 @ 0xe550
void sub_e550(unsigned long a0)
{
  dat_25588 = a0;
}

// Function: sub_e560 @ 0xe560
void sub_e560(void)
{
  int v1; // eax
  long v2;
  char *v3; // rax
  int *v4; // rax
  unsigned long v5;
  
  v5 = stdin;
  v2 = sub_edb0(stdin);
  if (v2) { // branch-flip
    if ((!sub_ede0(v5,0,1)) && (v1 = sub_e9c0(stdin), v1)) {
      sub_179a0(stdin);
      goto label_e5bf;
    }
    v1 = sub_179a0(stdin);
  }
  else {
    v1 = sub_179a0(v5);
  }
  if (!v1) {
    sub_e680(); // tail-call
    return;
  }
label_e5bf:
  v3 = dcgettext(NULL,"error closing file",5);
  v2 = dat_25588;
  v4 = __errno_location();
  if (v2) { // branch-flip
    v5 = sub_14c20(v2);
    error(0,*v4,"%s: %s",v5,v3);
    sub_e680();
  }
  else {
    error(0,*v4,"%s",v3);
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
  unsigned long v4; // rax
  
  v1 = sub_179a0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_25590) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_25598) { // branch-flip
        v4 = sub_14c20(dat_25598);
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
void sub_e730(void)
{
  if (sub_e790())
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
char * sub_e790(void *a0)
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
  
  v2 = (char *)sub_db60(a0);
  if (!*v2)
    v2 = a0;
  v1 = v2[sub_dbc0(v2)];
  v2[sub_dbc0(v2)] = '\0';
  return v1 != '\0';
}

// Function: sub_e840 @ 0xe840
void sub_e840(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_e850 @ 0xe850
void sub_e850(void *a0,unsigned int a1)
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
int * sub_e900(unsigned long a0,unsigned int a1,long a2,unsigned long a3,unsigned int a4)
{
  int v1; // eax
  int *v2; // rax
  
  if (0 <= (int)a0) { // branch-flip
    v2 = (unsigned long)futimens(a0,a3);
    if ((v1 == -1) && (a2)) {
      v2 = __errno_location();
      if (*v2 != 0x26) {
        v2 = (int *)0xffffffff;
        return v2;
      }
      v2 = (unsigned long)utimensat(a1,a2,a3,a4); // crossjump-dupe
    }
  }
  else {
    if (!a2) goto label_e980;
    v2 = (unsigned long)utimensat(a1,a2,a3,a4);
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
void sub_e9a0(unsigned int *a0)
{
  if (!(*a0 & 0x100))
    return;
  sub_ede0(a0,0,1); // tail-call
}

// Function: sub_e9c0 @ 0xe9c0
void sub_e9c0(void *a0)
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
void sub_ea00(long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  if (!a0)
    return;
  v1 = (unsigned long *)sub_16e90(0x18);
  *v1 = sub_17230(a1);
  v1[1] = a2[1];
  v1[2] = *a2;
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
unsigned long sub_ea90(long a0,unsigned long a1,unsigned long *a2) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  if (!a0)
    return 0;
  v3 = a2[1];
  v4 = *a2;
  v2 = a1;
  v1 = sub_11a70(a0,&v2);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
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
void sub_ec80(long a0)
{
  sub_eb60(*(unsigned int *)(a0 + 0x18)); // tail-call
}

// Function: sub_ec90 @ 0xec90
void sub_ec90(void)
{
  if (sub_ecb0())
    return;
  sub_17250(); // no-return
}

// Function: sub_ecb0 @ 0xecb0
void * sub_ecb0(void *a0,char *a1,unsigned long *a2)
{
  unsigned long v1;
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  void *v5; // rax
  char *v6; // rax
  unsigned long v7; // rbx
  char v8; // stack - 0x41
  
  v2 = sub_db60(a0);
  v3 = sub_dbc0(v2);
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

// Function: sub_edb0 @ 0xedb0
long sub_edb0(unsigned int *a0) // early-return
{
  long v1; // rax
  
  if (*(unsigned long *)&a0[8] < *(unsigned long *)&a0[10])
    return 0;
  v1 = *(long *)&a0[4] - *(long *)&a0[2];
  if (*a0 & 0x100)
    v1 += *(long *)&a0[0x16] - *(long *)&a0[0x12];
  return v1;
}

// Function: sub_ede0 @ 0xede0
unsigned long sub_ede0(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
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

// Function: sub_ee60 @ 0xee60
unsigned long sub_ee60(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_ee90 @ 0xee90
unsigned long sub_ee90(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
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
void sub_eef0(long a0,int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if ((unsigned int)(a1 - 1U) <= 0xb)
    v1 = *(unsigned int *)((unsigned long)(unsigned int)(a1 - 1U) * 4 + 0x1d5c0);
  *(unsigned int *)(a0 + 0x18) = v1;
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
unsigned long sub_ef60(long a0,long a1)
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

// Function: sub_efd0 @ 0xefd0
void sub_efd0(long a0,unsigned char a1)
{
  if (*(short *)(a0 + 0x68) == 0xb) {
    *(unsigned long *)(a0 + 0xa0) = (unsigned long)a1 + 1;
    return;
  }
  abort(); // no-return
}

// Function: sub_eff0 @ 0xeff0
int sub_eff0(long a0,long a1,char a2) // early-return x3
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
      goto label_f08b;
    }
  }
  else {
    v3 = *(char **)(a1 + 0x30);
    v5 = *(int *)(a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_f08b:
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

// Function: sub_f1a0 @ 0xf1a0
long sub_f1a0(long a0,long a1,unsigned long a2)
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
label_f1fb:
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
      goto label_f1fb;
    }
    v4 = *(long **)(a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  return a1;
}

// Function: sub_f2b0 @ 0xf2b0
void * sub_f2b0(long a0,void *a1,unsigned long a2)
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

// Function: sub_f350 @ 0xf350
unsigned long sub_f350(long a0)
{
  long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) {
    v1 = sub_11cd0(0x1f,0,sub_ee90,sub_ee60,dat_24fd0);
    *(long *)(a0 + 0x58) = v1;
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)(a0 + 0x58) = v2;
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
      if (*(void **)((long)a0 + 0x18))
        closedir(*(void **)((long)a0 + 0x18));
      free(a0);
      a0 = v1;
    } while (v1);
    return;
  }
}

// Function: sub_f410 @ 0xf410
void sub_f410(unsigned long a0)
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
unsigned long sub_f450(long a0,int a1)
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
    v2 = sub_11cd0(0xd,0,sub_eeb0,sub_eec0,dat_24fd0);
    *(long *)(v1 + 0x50) = v2;
    if (!v2) goto label_f4de;
  }
  v5[0] = *(unsigned long *)(a0 + 0x70);
  v1 = sub_11a70(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_f4de:
  if ((0 <= a1) && (!fstatfs(a1,v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      *v3 = *(unsigned long *)(a0 + 0x70);
      v3[1] = v6[0];
      v4 = (unsigned long *)sub_122f0(v2,v3);
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

// Function: sub_f580 @ 0xf580
bool sub_f580(void)
{
  long v1; // rax
  bool v2; // r8b
  
  v1 = sub_f450();
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
unsigned long sub_f5d0(void)
{
  long v1; // rax
  
  v1 = sub_f450();
  if ((v1 != 0x1021994) && (v1 != 0xff534d42))
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0x6969);
  return 0;
}

// Function: sub_f610 @ 0xf610
void sub_f610(long a0,long a1)
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

// Function: sub_f6a0 @ 0xf6a0
unsigned long sub_f6a0(long a0,unsigned long *a1) // return-dupe x2
{
  unsigned long v1;
  unsigned long *v2; // rax
  unsigned long *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v4 = sub_17a40(*(unsigned long *)(a0 + 0x58),&a1[0xe]);
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
  v3 = (unsigned long *)sub_122f0(v1,v2);
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

// Function: sub_f750 @ 0xf750
void sub_f750(long a0,int a1,char a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_12600(a0 + 0x60);
    if (0 <= v1) {
      close(v1);
      goto label_f794;
    }
  }
  else if ((!(*(unsigned char *)(a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_f794:
    *(int *)(a0 + 0x2c) = a1;
    return;
  }
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_f7b0 @ 0xf7b0
bool sub_f7b0(long a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)(a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
    sub_f750(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)(a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  sub_f410(a0 + 0x60);
  return v2;
}

// Function: sub_f820 @ 0xf820
void sub_f820(long a0,long a1)
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
    v4 = (void *)sub_12350(*(unsigned long *)(a0 + 0x58),&v5);
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

// Function: sub_f8e0 @ 0xf8e0
void sub_f8e0(long a0,long a1,long a2)
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

// Function: sub_f960 @ 0xf960
void sub_f960(unsigned long a0,unsigned int a1,unsigned long a2)
{
  if (a1 & 0x200) {
    sub_12710(a0,a2); // tail-call
    return;
  }
  sub_e880(a2,(a1 & 0x10) << 0xd | 0x90900); // tail-call
}

// Function: sub_f9a0 @ 0xf9a0
int sub_f9a0(long a0,long a1,int a2,char *a3) // return-dupe
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
      v7 = sub_125f0(a0 + 0x60);
      if (v7) goto label_fa1c;
      v3 = sub_12640(a0 + 0x60);
      v6 = *(unsigned int *)(a0 + 0x48);
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
      if ((*(long *)(a1 + 0x70) == v5) && (*(long *)(a1 + 0x78) == v8)) {
        v6 = *(unsigned int *)(a0 + 0x48);
        goto label_fa80;
      }
      *__errno_location() = 2;
    }
    v2 = -1;
  }
  else {
    v6 = *(unsigned int *)(a0 + 0x48);
    if (v6 & 4) goto label_fb30;
    if (0 <= a2) // branch-flip
      v7 = 0;
    else {
      v7 = 0;
label_fa1c:
      v1 = sub_f960(*(unsigned int *)(a0 + 0x2c),v6,a3);
      if (v1 < 0)
        return -1;
      v6 = *(unsigned int *)(a0 + 0x48);
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
void * sub_fc50(long *a0,int a1) // return-dupe
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
label_fcb2:
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
    v12 = sub_12790(v16,*(unsigned long *)(v1 + 0x30),v7,&v14);
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
        sub_f820(a0,v1);
        sub_eff0(a0,v1,0);
        if (!sub_f6a0(a0,v1)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)(v1 + 0x68) = sub_eff0(a0,v1,0);
    }
    v25 = (unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_10360:
      v20 = 0;
      v13 = *(unsigned int *)&a0[9];
      goto label_fcbb;
    }
    v7 = v14;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)(v1 + 0x80) != 2)) {
label_10097:
      v20 = a1 == 3;
      v4 = 1;
    }
    else {
      v6 = sub_f580(v1,v14);
      v7 = v14;
      if (!v6) goto label_10097;
      if (a1 != 3) goto label_10360;
      v4 = 0;
      v20 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v7 = sub_17d10(v7,0x406,3);
      v14 = v7;
    }
    if ((0 <= v7) && (!sub_f9a0(a0,v1,v7,0))) goto label_fcb2;
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
label_fcbb:
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
          goto label_10214;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (*(char *)((long)v9 + 0x13) != '.')) || ((*(char *)((long)v9 + 0x14) && (*(short *)((long)v9 + 0x14) != 0x2e)))) break;
label_fd83:
        v17 = *(void **)(v1 + 0x18);
        if (!v17) goto label_fed0;
      }
      v10 = strlen((char *)((long)v9 + 0x13));
      v11 = (void *)sub_f2b0(a0,(char *)((long)v9 + 0x13),v10);
      if (!v11) {
label_10110:
        v7 = *v8;
        free(v11);
        sub_f3c0(v23);
        closedir(*(void **)(v1 + 0x18));
        *(unsigned long *)(v1 + 0x18) = 0;
        *(unsigned short *)(v1 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = v7;
        return NULL;
      }
      if (v19 <= v10) {
        v3 = a0[4];
        v5 = sub_ef60(a0,v26 + 2 + v10);
        if (!v5) goto label_10110;
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
        sub_f3c0(v23);
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
        *(unsigned short *)((long)v11 + 0x68) = sub_eff0(a0,v11,0);
      else {
        v10 = 1;
        if ((v13 & 8) && (*(unsigned char *)((long)v9 + 0x12) & 0xfb))
          v10 = (unsigned long)((v13 >> 4 & 1 | (unsigned int)(*(unsigned char *)((long)v9 + 0x12) != 10)) ^ 1);
        *(unsigned short *)((long)v11 + 0x68) = 0xb;
        sub_eef0((long)v11 + 0x70);
        sub_efd0(v11,v10 & 0xffffffff);
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
          goto label_fd83;
        }
        goto label_10214;
      }
      if (!a0[8])
        v28 = sub_f5d0(v1,v14);
      v18 = 0x2711;
      v17 = *(void **)(v1 + 0x18);
      v22 = v11;
    } while (v17);
label_fed0:
label_10214:
    if (v27)
      sub_f8e0(a0[1],a0[4],v23);
    if (*(unsigned char *)&a0[9] & 4) {
      if ((a0[6] == v21) || (!v18)) goto label_10244;
label_1024a:
      *v24 = 0;
    }
    if ((v2) || (!v20)) {
label_102f0:
      if (!v18) goto label_102f5;
    }
    else if ((a1 == 1) || (!v18)) {
label_102d0:
      if (*(long *)(v1 + 0x58)) // branch-flip
        v7 = sub_f9a0(a0,*(unsigned long *)(v1 + 8),0xffffffff,"..");
      else {
        v7 = sub_f7b0(a0);
      }
      if (v7) {
        *(unsigned short *)(v1 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        sub_f3c0(v23);
        return NULL;
      }
      goto label_102f0;
    }
    if (v28) { // branch-flip
      a0[8] = (long)sub_eed0;
      v23 = (void *)sub_f1a0(a0,v23,v18);
      a0[8] = 0;
    }
    else if ((a0[8]) && (v18 != 1))
      v23 = (void *)sub_f1a0(a0,v23,v18);
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v23 = NULL;
      v18 = 0;
      v28 = '\0';
label_10244:
      v24 = &v24[-1];
      goto label_1024a;
    }
    v23 = NULL;
    if ((bool)(v20 & v2 == NULL)) {
      v28 = '\0';
      v18 = 0;
      goto label_102d0;
    }
label_102f5:
    if (((a1 == 3) && (*(short *)(v1 + 0x68) != 4)) && (*(short *)(v1 + 0x68) != 7))
      *(unsigned short *)(v1 + 0x68) = 6;
    sub_f3c0(v23);
    v23 = NULL;
  }
  return v23;
}

// Function: sub_105a0 @ 0x105a0
void sub_105a0(unsigned int a0,void *a1)
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
  v4 = sub_ef10(a0);
  v6 = 0x1000;
  if (0x1000 <= v4)
    v6 = v4;
  v12 = sub_ef60(v3,v6);
  if (!v12) {
    free(v3); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v11 = (void *)sub_f2b0(v3,"",0);
    if (v11) {
      *(unsigned long *)((long)v11 + 0x58) = 0xffffffffffffffff;
      v10 = (char *)*a0;
      if (a2) goto label_106b9;
label_106c8:
      if (!v10) goto label_1090d;
      v13 = 0;
      v8 = 0;
      v7 = 0;
label_1076f:
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
        v5 = sub_f2b0(v3,v10,v6);
        if (!v5) goto label_107e0;
        *(unsigned long *)(v5 + 0x58) = 0;
        *(void **)(v5 + 8) = v11;
        *(long *)(v5 + 0x30) = v5 + 0x100;
        if ((v8) && (v12)) {
          *(unsigned short *)(v5 + 0x68) = 0xb;
          sub_efd0(v5,1);
          if (a2) { // branch-flip
label_10757:
            *(long *)(v5 + 0x10) = v8;
          }
          else {
            *(unsigned long *)(v5 + 0x10) = 0;
label_108e8:
            v13 = v5;
            *(long *)(v7 + 0x10) = v5;
            v5 = v8;
          }
          v10 = (char *)a0[v9 + 1];
          v8 = v5;
          v7 = v9 + 1;
          if (!v10) break;
          goto label_1076f;
        }
        *(unsigned short *)(v5 + 0x68) = sub_eff0(v3,v5,0);
        if (a2) goto label_10757;
        *(unsigned long *)(v5 + 0x10) = 0;
        if (v8) goto label_108e8;
        v10 = (char *)a0[v9 + 1];
        v8 = v5;
        v7 = v9 + 1;
        v13 = v5;
      } while (v10);
      if ((a2) && (2 <= v9 + 1U))
        v5 = sub_f1a0(v3,v5,v9 + 1U);
      goto label_10874;
    }
  }
  else {
    v11 = NULL;
    if (a2) {
      v10 = NULL;
label_106b9:
      v12 = (unsigned char)(*(unsigned int *)&v3[9] >> 10) & 1;
      goto label_106c8;
    }
label_1090d:
    v5 = 0;
label_10874:
    v7 = sub_f2b0(v3,"",0);
    *v3 = v7;
    v8 = v5;
    if (v7) {
      *(long *)(v7 + 0x10) = v5;
      *(unsigned short *)(v7 + 0x68) = 9;
      *(unsigned long *)(v7 + 0x58) = 1;
      if (sub_f350(v3)) {
        if (!(*(unsigned int *)&v3[9] & 0x204)) {
          v2 = sub_f960(*(unsigned int *)((long)v3 + 0x2c),*(unsigned int *)&v3[9],".");
          *(int *)&v3[5] = v2;
          if (v2 <= -1)
            *(unsigned int *)&v3[9] = *(unsigned int *)&v3[9] | 4;
        }
        sub_125d0(&v3[0xc],0xffffffff);
        return v3;
      }
    }
label_107e0:
    sub_f3c0(v8);
    free(v11);
  }
  free((void *)v3[4]);
  free(v3);
  return NULL;
}

// Function: sub_10940 @ 0x10940
unsigned int sub_10940(long *a0) // return-dupe
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
    sub_f3c0(a0[1]);
  v1 = &a0[0xc];
  free((void *)a0[2]);
  free((void *)a0[4]);
  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
    v6 = *(int *)((long)a0 + 0x2c);
    if ((v6 <= -1) || (!close(v6))) goto label_109d7;
    v6 = *__errno_location();
label_10a8d:
    sub_f410(v1);
    v5 = a0[10];
    if (v5) goto label_109ec;
  }
  else {
    if (!(*(unsigned int *)&a0[9] & 4)) {
      v5 = a0[5];
      if (fchdir((int)v5)) {
        v3 = __errno_location();
        v5 = a0[5];
        v6 = *v3;
        if (!close((int)v5)) goto label_10a8d;
        if (!v6) goto label_10a48;
        sub_f410(v1);
        v5 = a0[10];
        if (!v5) {
          sub_105a0((int)a0[9],a0[0xb]);
          free(a0);
          goto label_10a11;
        }
        goto label_109ec;
      }
      v5 = a0[5];
      if (close((int)v5)) {
        v3 = __errno_location();
label_10a48:
        v6 = *v3;
        goto label_10a8d;
      }
    }
label_109d7:
    v6 = 0;
    sub_f410(v1);
    v5 = a0[10];
    if (!v5) {
      sub_105a0((int)a0[9],a0[0xb]);
      free(a0);
      return 0;
    }
label_109ec:
    sub_11e90(v5);
  }
  sub_105a0((int)a0[9],a0[0xb]);
  free(a0);
  if (!v6)
    return 0;
  v3 = __errno_location();
label_10a11:
  *v3 = v6;
  return 0xffffffff;
}

// Function: sub_10b30 @ 0x10b30
void * sub_10b30(long *a0)
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
            *a0 = (long)v8;
            *(char *)(a0[4] + v4) = 0;
            v8 = (void *)sub_fc50(a0,3);
            if (v8) {
              free(v7);
              goto label_10e02;
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
                    sub_f750(a0,v6,1);
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
                goto label_10c2b;
              }
              if (*(unsigned short *)((long)v8 + 0x6a) & 1) goto label_10c2b;
              v6 = sub_f9a0(a0,*(unsigned long *)((long)v8 + 8),0xffffffff,"..");
            }
            else {
              v6 = sub_f7b0(a0);
            }
            if (v6) {
              *(int *)((long)v8 + 0x40) = *__errno_location();
              *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
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
          if (sub_f7b0(a0)) {
            *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            return NULL;
          }
          sub_105a0((int)a0[9],a0[0xb]);
          sub_f610(a0,v8);
          sub_f350(a0);
          v5 = *(short *)((long)v8 + 0x68);
          goto label_10da5;
        }
      } while (*(short *)((long)v8 + 0x6c) == 4);
      if (*(short *)((long)v8 + 0x6c) == 2) {
        v5 = sub_eff0(a0,v8,1);
        *(short *)((long)v8 + 0x68) = v5;
        if ((v5 == 1) && (!(*(unsigned int *)&a0[9] & 4))) {
          v6 = sub_f960(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
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
      if ((v2 & 0x40) && (*(long *)((long)v7 + 0x70) != a0[3])) {
label_10d2a:
        if (*(unsigned char *)((long)v7 + 0x6a) & 2)
          close(*(int *)((long)v7 + 0x44));
        if (a0[1]) {
          sub_f3c0(a0[1]);
          a0[1] = 0;
        }
        *(unsigned short *)((long)v7 + 0x68) = 6;
        sub_f820(a0,v7);
        return v7;
      }
      if (a0[1]) { // branch-flip
        if (v2 & 0x1000) {
          *(unsigned int *)&a0[9] = v2 & 0xffffefff;
          sub_f3c0(a0[1]);
          a0[1] = 0;
          goto label_10ea0;
        }
        v3 = *(unsigned long *)((long)v7 + 0x30);
        if (sub_f9a0(a0,v7,0xffffffff,v3)) { // branch-flip
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
label_10ea0:
        v8 = (void *)sub_fc50(a0,3);
        a0[1] = (long)v8;
        if (!v8) {
          if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20)) {
            if ((*(int *)((long)v7 + 0x40)) && (*(short *)((long)v7 + 0x68) != 4))
              *(unsigned short *)((long)v7 + 0x68) = 7;
            sub_f820(a0,v7);
            return v7;
          }
          return NULL;
        }
      }
      a0[1] = 0;
    }
label_10e02:
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
      if (*(unsigned int *)&a0[9] & 4) {
        *a0 = (long)v7;
        goto label_10dbd;
      }
      v6 = sub_f960(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
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
label_10da5:
      *a0 = (long)v7;
      goto label_10dad;
    }
    *a0 = (long)v7;
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
    a0[3] = *(long *)((long)v7 + 0x70);
  if (!sub_f6a0(a0,v7)) {
    v7 = NULL;
    *__errno_location() = 0xc;
  }
  return v7;
}

// Function: sub_11170 @ 0x11170
unsigned long sub_11170(unsigned long a0,long a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)(a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_111a0 @ 0x111a0
int * sub_111a0(long *a0,unsigned int a1) // early-return x2
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
    sub_f3c0(a0[1]);
  v6 = 1;
  if (a1 == 0x1000) {
    *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x1000;
    v6 = 2;
  }
  if (((*(long *)(v1 + 0x58)) || (**(char **)(v1 + 0x30) == '/')) || (*(unsigned int *)&a0[9] & 4)) {
    v5 = (long)sub_fc50(a0,v6);
    a0[1] = (long)v5;
  }
  else {
    v4 = sub_f960(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
    v2 = v4;
    if (v4 < 0) {
      a0[1] = 0;
      v5 = NULL;
      return v5;
    }
    v5 = (long)sub_fc50(a0,v6);
    a0[1] = (long)v5;
    if (*(unsigned char *)((long)a0 + 0x49) & 2) // branch-flip
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
    v5 = (long)a0[1];
  }
  return v5;
}

// Function: sub_11320 @ 0x11320
long sub_11320(unsigned int a0,long a1,long a2)
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
      a1 += v1;
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
unsigned long sub_11480(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1d660)
    return 1;
  v1 = v2[2];
  if ((((dat_1d674 < v1) && (v1 < dat_1d678)) && (dat_1d67c < v2[3])) && (dat_1d690 <= *v2)) {
    v3 = *v2 + dat_1d674;
    if (((v3 < v2[1]) && (v2[1] <= dat_1d680)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1d660;
  return 0;
}

// Function: sub_11510 @ 0x11510
void sub_11510(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_11530 @ 0x11530
long sub_11530(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}

// Function: sub_11560 @ 0x11560
long sub_11560(long a0,long a1,unsigned long *a2,char a3) // early-return
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
void sub_11660(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_11690 @ 0x11690
unsigned long sub_11690(long a0,unsigned long *a1,char a2) // return-dupe x2
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
label_116b8:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_11530(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_11510(a0);
          v3 = v4;
          if (!v4) goto label_11728;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_11728:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_116b8;
    v3 = (long *)sub_11530(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_11660(a0);
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

// Function: sub_117c0 @ 0x117c0
unsigned long sub_117c0(float a0,unsigned long a1,char a2) // early-return, ternary
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
unsigned long sub_11850(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_11860 @ 0x11860
unsigned long sub_11860(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_11870 @ 0x11870
unsigned long sub_11870(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_11880 @ 0x11880
unsigned long sub_11880(unsigned long *a0)
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

// Function: sub_118e0 @ 0x118e0
bool sub_118e0(unsigned long *a0)
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
void sub_11950(unsigned long a0,void *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_11a70 @ 0x11a70
long sub_11a70(long a0,long a1)
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
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}

// Function: sub_11ad0 @ 0x11ad0
long sub_11ad0(unsigned long *a0)
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

// Function: sub_11b20 @ 0x11b20
long sub_11b20(long a0,long a1)
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
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}

// Function: sub_11b90 @ 0x11b90
void sub_11b90(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_11be0 @ 0x11be0
long sub_11be0(unsigned long *a0,void *a1,unsigned long a2)
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
void sub_11ca0(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_11cd0 @ 0x11cd0
unsigned long * sub_11cd0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
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
    a1 = 0x1d660;
  v1[5] = a1;
  if (sub_11480(v1)) {
    v2 = sub_117c0(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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
void sub_11de0(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_11e7b;
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
label_11e7b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_11e90 @ 0x11e90
void sub_11e90(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_11f3c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11eff;
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
label_11eff:
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
label_11f3c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_11f80 @ 0x11f80
unsigned int sub_11f80(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_117c0(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_11690(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_11690(a0,&v2,1)) && (sub_11690(a0,&v2,0))) {
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

// Function: sub_12100 @ 0x12100
unsigned long sub_12100(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
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
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_11480(a0);
      v1 = *(long *)(a0 + 0x28);
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
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (long *)sub_11660(a0);
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

// Function: sub_122f0 @ 0x122f0
unsigned long sub_122f0(unsigned long a0,unsigned long a1) // early-return
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
long sub_12350(long a0,unsigned long a1) // ternary x2
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_11480(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1d688 <= v8) ? (long)(v8 - dat_1d688) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11f80(a0,v6)) {
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

// Function: sub_124e0 @ 0x124e0
void sub_124e0(void)
{
  sub_12350(); // tail-call
}

// Function: sub_124f0 @ 0x124f0
unsigned long sub_124f0(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_12510 @ 0x12510
unsigned long sub_12510(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = *a1;
    v2 = *a0;
    return sub_152d0(v2,v1); // tail-call
  }
  return 0;
}

// Function: sub_12540 @ 0x12540
unsigned long sub_12540(unsigned long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_17e60(*a0);
  return (v1 ^ a0[1]) % a1;
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
void sub_125d0(unsigned int *a0,unsigned int a1)
{
  *(unsigned long *)&a0[5] = 0;
  *(char *)&a0[7] = 1;
  *a0 = a1;
  a0[1] = a1;
  a0[2] = a1;
  a0[3] = a1;
  a0[4] = a1;
}

// Function: sub_125f0 @ 0x125f0
char sub_125f0(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_12600 @ 0x12600
unsigned int sub_12600(long a0,unsigned int a1)
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

// Function: sub_12640 @ 0x12640
unsigned int sub_12640(long a0)
{
  unsigned int v1;
  unsigned int v2;
  char v3; // al
  
  v3 = sub_125f0(a0);
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
void * sub_12790(unsigned long a0,unsigned long a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  void *v3; // rax
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
unsigned int sub_12890(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,unsigned int a4)
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
unsigned int sub_12910(unsigned long a0,unsigned int a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0x24
  
  v2 = a2;
  v1 = sub_176a0(&v2,a0,a1);
  sub_17660(&v2);
  return v1;
}

// Function: sub_12970 @ 0x12970
int * sub_12970(int *a0,int a1)
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

// Function: sub_129c0 @ 0x129c0
bool sub_129c0(long a0,long a1)
{
  return sub_17940(a0 + 9,a1 + 9) == 0;
}

// Function: sub_129e0 @ 0x129e0
unsigned long sub_129e0(long a0,unsigned long a1,unsigned char a2)
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
unsigned long sub_12a40(long a0,unsigned long a1,unsigned char a2,char a3)
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
unsigned long sub_12ac0(long a0,unsigned long a1,unsigned char a2,char a3,char a4)
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
unsigned long sub_12b30(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5)
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
unsigned long sub_12bd0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
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
unsigned long sub_12c70(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
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
unsigned long sub_12d30(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
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
unsigned long sub_12df0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
unsigned long sub_12ed0(unsigned char *a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
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
  unsigned long v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = sub_17ea0();
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
label_13130:
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
label_135b6:
      v2 = 0;
      goto label_13570;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_136d1;
      }
      v24 = 1;
      v2 = 0;
      v12 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_13570:
      v24 = 1;
      v12 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_136d1;
      }
      goto label_135b6;
    case 5:
      if (!v17) goto label_13511;
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
        v9 = (char *)sub_12f90("`",a4);
        v36 = (char *)v9;
        v9 = (char *)sub_12f90("\'",a4);
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
label_131f3:
  do {
label_13206:
    v23 = 0;
    a1 = v13;
label_13210:
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
          if ((a3 < v23 + v30) || (v8 = memcmp(v11,v31,v30), v8)) goto label_13b80;
          if (!v24) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v21 = v18;
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
              v21 = 0x7b;
              if (v7 == 0x7b) goto label_13b4a;
              v21 = 0x7c;
              if (v7 == 0x7c) goto label_13b68;
              goto label_13453;
            }
            if (v7 != 0x7e) goto label_13453;
label_13ca4:
            if (!v23) {
              v7 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_134c9;
            }
            v21 = 0x7e;
            goto label_13cb2;
          }
          goto label_1343c;
        }
label_13b80:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v21 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v22 = 0;
                if (v7 == 0x7e) goto label_13ca4;
                goto label_13450;
              }
              v21 = 0x7b;
              v25 = 0;
              if (v7 != 0x7b) {
                v21 = 0x7c;
                v25 = 0;
                v22 = 0;
                if (v7 == 0x7c) goto label_13b68;
                goto label_13453;
              }
            }
            else {
              v21 = 0x7d;
              v25 = 0;
            }
label_13b4a:
            if (a3 == 0xffffffffffffffff) goto label_13cff;
label_13b58:
            v22 = v25;
            if (a3 == 1) goto label_137e5;
          }
          else {
            if (v7 == 0x40) goto label_13450;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v22 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_13338;
            v25 = 0;
            v22 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_13c24:
              if ((char)v18 != '\\') goto label_13453;
              if (((bool)(v2 & v24)) && (v30)) goto label_13c4a;
              v16 = 0x5c;
              goto label_13672;
            }
          }
label_13b68:
          v7 = a4 == 2;
          v26 = v3;
          v25 = 0;
          goto label_134c9;
        }
        switch(v7) {
          case 0:
            goto label_1376e;
          default:
label_13450:
            v25 = 0;
label_13453:
            v16 = (unsigned char)v18;
            v22 = v25;
            if (v33 != 1) goto label_139d3;
label_13460:
            v9 = __ctype_b_loc();
            v21 = 1;
            v26 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v27 = (bool)(v26 & v2);
            v26 = !v26;
            v22 = v25;
            goto label_134ae;
          case 7:
label_13740:
            v16 = 0x61;
            if (!v24) goto label_13738;
            goto label_1369f;
          case 8:
label_13725:
            v16 = 0x62;
            goto label_1372a;
          case 9:
            v25 = 0;
label_13803:
            v21 = 9;
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
            if (v24) goto label_1369f;
label_13738:
            v25 = 0;
            goto label_133be;
          case 0xd:
label_1366d:
            v16 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_1383d:
            v21 = 0x20;
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
            v25 = 0;
            v26 = 0;
            goto label_13409;
          case 0x23:
            v21 = 0x23;
            v25 = 0;
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
            v22 = 0;
label_13338:
            v7 = 0;
            goto label_13348;
          case 0x27:
            v22 = 0;
            goto label_13652;
          case 0x3f:
            v22 = 0;
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
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_13b58;
label_13cff:
            v22 = v25;
            if (a2[1]) goto label_13b68;
            goto label_137e5;
          }
          if ((char)v16 <= '}') {
            v21 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_13406;
              goto label_13453;
            }
            goto label_13b4a;
          }
          v21 = 0x7e;
          if (v16 == 0x7e) goto label_137e5;
          if (v33 == 1) goto label_13460;
label_139d3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v21 = 0;
          do {
            v18 = v23 + v21;
            v9 = (long)sub_17ee0(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v25 = v26;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_13f3b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_142b5;
              goto label_142bf;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_13b0d;
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
          goto label_13f3b;
        }
        if (v16 == 0x40) goto label_13453;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_134c0;
        if ((unsigned long)v9 & 0xa4000000) goto label_13406;
        if (v16 != 0x5c) goto label_13453;
        if (a4 == 2) {
          if (!v24) goto label_13c4a;
          goto label_13433;
        }
        if (!(bool)(v2 & v24 & v30 != 0)) {
          v21 = 0x5c;
          v16 = 0x5c;
          v25 = 0;
          goto label_13810;
        }
label_13c4a:
        v23 += 1;
        v25 = 0;
        v16 = 0x5c;
        v7 = v20;
        goto label_13c60;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_138e2;
            goto label_1381b;
          }
label_1376e:
          if (!v24) {
            v22 = 0;
label_1377c:
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
                goto label_134f3;
              }
label_14085:
              a0[v13] = 0x5c;
              v12 = v13;
              v20 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v20;
              goto label_14085;
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
label_140bc:
              v16 = (unsigned char)v21;
              v25 = 0;
              v12 = v13;
              v7 = v26;
              if (!v2 || v27) goto label_134f3;
              goto label_13348;
            }
            v16 = 0x30;
            v25 = 0;
            v12 = v13;
            v7 = v26;
            goto label_134f3;
          }
          if (a4 != 2) goto label_1343c;
          goto label_136ab;
        default:
          goto label_13453;
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
          goto label_1389f;
        case 10:
          v21 = 10;
          v16 = 0x6e;
          goto label_1389f;
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
label_1389f:
          v7 = a4 == 2 & v24;
          v25 = 0;
          if (!(bool)v7) goto label_13810;
label_13421:
          a4 = 2;
label_13433:
          if (!v28) goto label_1343c;
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
          v26 = 0;
          goto label_13409;
        case 0x23:
label_137e0:
          v21 = 0x23;
label_137e5:
          v22 = v25;
          if (v23) {
label_13cb2:
            v7 = a4 == 2;
            v26 = v3;
            v25 = 0;
            goto label_134c9;
          }
          v18 = v21;
label_13409:
          v21 = v18;
          v22 = v25;
          v25 = v26;
          if ((a4 == 2) && (v24)) goto label_13421;
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
          v25 = v26;
          if (a4 != 2) {
            v7 = 0;
            v21 = 0x27;
            goto label_134c9;
          }
          if (!v24) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v32) goto label_140db;
            }
            else {
label_140db:
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
            goto label_134f3;
          }
          goto label_13433;
        case 0x3f:
label_13611:
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
                  goto label_134c9;
                }
                if (v24) goto label_1343c;
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
                goto label_140bc;
              }
            }
            v7 = 0;
            v21 = 0x3f;
            v26 = v3;
            v25 = 0;
            goto label_134c9;
          }
          if (v24) goto label_13433;
          v7 = 0;
          v16 = 0x3f;
          v25 = 0;
          goto label_134f3;
        
      }
label_13810:
      v22 = v25;
      if (!v2) {
label_1381b:
        v16 = (unsigned char)v21;
        v25 = 0;
        v7 = 0;
        v26 = v3;
        if (!v24) goto label_134f0;
        goto label_13348;
      }
label_13672:
      v25 = 0;
      goto label_13683;
    }
    if (!(bool)(v12 == 0 & v24 & a4 == 2)) {
      v7 = a4 == 2 & (v24 ^ 1U);
      v24 = (bool)(v24 ^ 1U);
      if ((!(bool)v7) || (v24 = (bool)v7, !v3)) {
label_1433f:
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
        if (a1 || !v32) goto label_1433f;
        v23 = v32;
        goto label_13e0b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v9;
      v31 = "\"";
      if (!(v34 & 2)) goto label_143ef;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v24 = v4;
      goto label_13206;
    }
label_13b0d:
    if (v2) {
label_136ab:
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
label_136d1:
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
label_13e0b:
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
label_142b5:
    if (!a2[(long)v9]) break;
  }
label_142bf:
  v25 = 0;
label_13f3b:
  v18 = (unsigned long)v16;
  v26 = v25;
  if (2 <= v21) {
label_13f45:
    v19 = 0;
    v21 += v23;
    v13 = v23;
    do {
      v16 = (unsigned char)v18;
      if (v27) {
        v28 = a4 == 2;
        if (v24) goto label_13433;
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
        if (v21 <= v23) goto label_133d1;
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
          goto label_13c60;
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
label_134ae:
  if (v27) {
    v25 = 0;
    v27 = v2;
    goto label_13f45;
  }
label_134c0:
  v21 = v18;
  v25 = v26;
label_134c7:
  v7 = a4 == 2;
  v26 = v3;
label_134c9:
  v16 = (unsigned char)v21;
  v27 = (bool)v7 == 0;
  v3 = v26;
  v7 = 0;
  if ((v27 && v2) || (v7 = 0, v24)) {
label_13348:
    v16 = (unsigned char)v21;
    v12 = v13;
    if (!v29) goto label_134f3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_134f3;
  }
  else {
label_134f0:
    v7 = 0;
    v3 = v26;
label_134f3:
    if (!v22) {
      v7 ^= 1;
      v23 += 1;
      v7 &= v20;
label_13c60:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v20 = 0;
        v12 += 2;
      }
      goto label_133d1;
    }
  }
label_13683:
  if (v24) {
    v24 = v2;
label_1369f:
    if ((bool)(v24 & a4 == 2)) goto label_136ab;
label_1343c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_13130;
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
label_133be:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v23 += 1;
label_133d1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v25)
    v4 = 0;
  goto label_13210;
label_138e2:
  v23 += 1;
  goto label_13210;
label_143ef:
  a1 = v32;
label_13511:
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
  goto label_131f3;
}

// Function: sub_144f0 @ 0x144f0
void * sub_144f0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_25080;
    if (dat_25078 <= (int)a0) {
      v10 = (long)dat_25078;
      v9 = (long)(int)((a0 - dat_25078) + 1);
      if (dat_25080 != (unsigned long *)0x25090) // branch-flip
        v6 = (unsigned long *)sub_17050(dat_25080,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_17050(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_25090;
        v6[1] = dat_25098;
      }
      dat_25080 = v6;
      memset(&v6[(long)dat_25078 * 2],0,(v10 - dat_25078) * 0x10);
      dat_25078 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_130c0(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x255c0)
        free(v8);
      v8 = (void *)sub_16ed0(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_130c0(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_146e0 @ 0x146e0
void sub_146e0(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x256c0;
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
unsigned int sub_147a0(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x256c0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
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
void sub_14800(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x256c0;
  v2 = __errno_location();
  v1 = *v2;
  sub_130c0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_14880 @ 0x14880
unsigned long sub_14880(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x256c0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_130c0(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_16ed0(v3 + 1);
  sub_130c0(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_14970 @ 0x14970
void sub_14970(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_14880(a0,a1,0,a2); // tail-call
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
void sub_14a20(unsigned long a0,unsigned long a1)
{
  sub_144f0(a0,a1,0xffffffffffffffff,0x256c0); // tail-call
}

// Function: sub_14a40 @ 0x14a40
void sub_14a40(void)
{
  sub_144f0(); // tail-call
}

// Function: sub_14a50 @ 0x14a50
void sub_14a50(unsigned long a0)
{
  sub_14a20(0,a0); // tail-call
}

// Function: sub_14a60 @ 0x14a60
void sub_14a60(unsigned long a0,unsigned long a1)
{
  sub_14a40(0,a0,a1); // tail-call
}

// Function: sub_14a70 @ 0x14a70
void sub_14a70(unsigned int a0,unsigned long a1,unsigned long a2)
{
  char v1 [56];
  
  sub_12970(v1);
  sub_144f0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_14ad0 @ 0x14ad0
void sub_14ad0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_12970(v1);
  sub_144f0(a0,a2,a3,v1);
}

// Function: sub_14b40 @ 0x14b40
void sub_14b40(unsigned int a0,unsigned long a1)
{
  sub_14a70(0,a0,a1); // tail-call
}

// Function: sub_14b50 @ 0x14b50
void sub_14b50(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_14ad0(0,a0,a1,a2); // tail-call
}

// Function: sub_14b70 @ 0x14b70
void sub_14b70(unsigned long a0,unsigned long a1,char a2,unsigned long a3)
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
void sub_14c00(unsigned long a0,char a1)
{
  sub_14b70(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_14c20 @ 0x14c20
void sub_14c20(unsigned long a0)
{
  sub_14c00(a0,0x3a); // tail-call
}

// Function: sub_14c30 @ 0x14c30
void sub_14c30(unsigned long a0,unsigned long a1)
{
  sub_14b70(a0,a1,0x3a); // tail-call
}

// Function: sub_14c40 @ 0x14c40
void sub_14c40(unsigned int a0,unsigned long a1,unsigned long a2)
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
void sub_14ce0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
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
  sub_147c0(&v1);
  sub_144f0(a0,a3,a4,&v1);
}

// Function: sub_14d80 @ 0x14d80
void sub_14d80(void)
{
  sub_14ce0(); // tail-call
}

// Function: sub_14d90 @ 0x14d90
void sub_14d90(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_14d80(0,a0,a1,a2); // tail-call
}

// Function: sub_14db0 @ 0x14db0
void sub_14db0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_14ce0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_14dd0 @ 0x14dd0
void sub_14dd0(void)
{
  sub_144f0(); // tail-call
}

// Function: sub_14de0 @ 0x14de0
void sub_14de0(unsigned long a0,unsigned long a1)
{
  sub_14dd0(0,a0,a1); // tail-call
}

// Function: sub_14df0 @ 0x14df0
void sub_14df0(unsigned long a0,unsigned long a1)
{
  sub_14dd0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_14e00 @ 0x14e00
void sub_14e00(unsigned long a0)
{
  sub_14df0(0,a0); // tail-call
}

// Function: sub_14e10 @ 0x14e10
unsigned long sub_14e10(int a0)
{
  *__errno_location() = a0;
  return 0xffffffff;
}

// Function: sub_14e30 @ 0x14e30
void sub_14e30(void)
{
  renameat(); // tail-call
}

// Function: sub_14e40 @ 0x14e40
unsigned long sub_14e40(int a0,char *a1,int a2,char *a3,int a4)
{
  int v1;
  unsigned int v10; // stack - 0x150
  unsigned int v11; // stack - 0xc0
  unsigned int v2; // eax
  int *v3; // rax
  unsigned long v4; // rax
  bool v5;
  char v6 [24];
  char v7 [24];
  unsigned long v8; // rax
  bool v9; // zf
  
  v2 = renameat2();
  if (0 <= (int)v2)
    return (unsigned long)v2;
  v3 = __errno_location();
  v1 = *v3;
  v9 = (v1 - 0x16U & 0xffffffefU) != 0;
  v5 = v9 && v1 != 0x5f;
  if (v9 && v1 != 0x5f)
    return (unsigned long)v2;
  if (a4) { // branch-flip
    if (a4 != 1)
      return sub_14e10(0x5f); // tail-call
    if ((!fstatat(a2,a3,v6,0x100)) || (*v3 == 0x4b))
      return (unsigned long)(unsigned int)sub_14e10(0x11);
    if (*v3 == 2) {
      v5 = 1;
      goto label_14f36;
    }
    v8 = 0xffffffff;
  }
  else {
label_14f36:
    v8 = strlen(a1);
    v4 = strlen(a3);
    if (((v8) && (v4)) && ((a1[v8 - 1] == '/' || (a3[v4 - 1] == '/')))) {
      if (fstatat(a0,a1,v7,0x100)) {
        v8 = 0xffffffff;
        return v8;
      }
      if (v5) {
        if ((v10 & 0xf000) != 0x4000)
          return (unsigned long)(unsigned int)sub_14e10(2);
      }
      else if (fstatat(a2,a3,v6,0x100)) { // branch-flip
        if ((*v3 != 2) || ((v10 & 0xf000) != 0x4000)) {
          v8 = 0xffffffff;
          return v8;
        }
      }
      else {
        if ((v11 & 0xf000) != 0x4000)
          return (unsigned long)(unsigned int)sub_14e10(0x14);
        if ((v10 & 0xf000) != 0x4000)
          return (unsigned long)(unsigned int)sub_14e10(0x15);
      }
    }
    v8 = (unsigned long)(unsigned int)sub_14e30(a0,a1,a2,a3);
  }
  return v8;
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
unsigned long sub_15130(int a0,unsigned long a1,int a2,unsigned long a3)
{
  void *v1; // rax
  long v10; // stack - 0x160
  long v11; // stack - 0xd0
  void *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  long v7; // stack - 0x168
  long v8; // stack - 0xd8
  unsigned long v9; // r9
  
  v1 = (void *)sub_db60(a1);
  v2 = (void *)sub_db60(a3);
  v3 = sub_dbc0(v1);
  v9 = 0;
  if (v3 == sub_dbc0(v2)) {
    v9 = 0;
    if (!memcmp(v1,v2,v3)) {
      v4 = (char *)sub_e730(a1);
      if (fstatat(a0,v4,&v7,0x100)) {
        error(1,*__errno_location(),"%s",v4);
        return v6;
      }
      free(v4);
      v4 = (char *)sub_e730(a3);
      if (fstatat(a2,v4,&v8,0x100)) {
        error(1,*__errno_location(),"%s",v4);
        return v5;
      }
      free(v4);
      v9 = (unsigned long)(v10 == v11 && v7 == v8);
    }
  }
  return v9;
}

// Function: sub_152d0 @ 0x152d0
void sub_152d0(unsigned long a0,unsigned long a1)
{
  sub_15130(0xffffff9c,a0,0xffffff9c,a1); // tail-call
}

// Function: sub_152f0 @ 0x152f0
int sub_152f0(long a0,long a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(a1 + 8);
  v2 = *(unsigned long *)(a0 + 8);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_15310 @ 0x15310
void sub_15310(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_15320 @ 0x15320
char * sub_15320(void *a0,unsigned int a1)
{
  unsigned long v1;
  char *v10;
  long v11; // stack - 0x48
  long v12; // stack - 0x50
  unsigned long *v13; // rbp
  long v14;
  long v15; // r12
  unsigned long *v16; // stack - 0x70
  char v2;
  long v3;
  int *v4; // rax
  unsigned long *v5;
  char *v6;
  char *v7; // rax
  char *v8;
  long v9; // rbx
  
  v12 = 0;
  v11 = 0;
  v3 = *(long *)((unsigned long)a1 * 8 + 0x24960);
  if (a0) { // branch-flip
    v9 = 0;
    v14 = 0;
    v8 = NULL;
    v4 = __errno_location();
    v16 = NULL;
    while( true ) {
      *v4 = 0;
      v5 = readdir(a0);
      if (!v5) break;
      v10 = (char *)((long)v5 + 0x13);
      v2 = *(char *)((long)v5 + 0x13);
      if ((v2 == '.') && (v2 = *(char *)((long)v5 + 0x14), v2 == '.'))
        v2 = *(char *)((long)v5 + 0x15);
      if (v2) {
        v1 = strlen(v10) + 1;
        if (v3) { // branch-flip
          if (v11 == v14)
            v16 = (unsigned long *)sub_17050(v16,&v11,1,0xffffffffffffffff,0x10);
          v15 = v14 + 1;
          v16[v14 * 2] = sub_17230(v10);
          (&v16[v14 * 2])[1] = *v5;
          v14 = v15;
        }
        else {
          if (v12 - v9 <= (long)v1)
            v8 = (char *)sub_17050(v8,&v12,v1 - (v12 - v9),0x7ffffffffffffffe,1);
          memcpy(&v8[v9],v10,v1);
        }
        v9 += v1;
      }
    }
    if (*v4) { // branch-flip
      free(v16);
      v6 = NULL;
      free(v8);
    }
    else {
      if (v3) { // branch-flip
        if (v14) { // branch-flip
          v15 = 0;
          qsort(v16,v14,0x10,v3);
          v10 = NULL;
          v8 = (char *)sub_16eb0(v9 + 1);
          v5 = v16;
          do {
            v15 += 1;
            v13 = &v5[2];
            v7 = stpcpy(&v8[(long)v10],(char *)*v5);
            v10 = &v7[(long)&v10[1 - (long)&v8[(long)v10]]];
            free((void *)*v5);
            v5 = v13;
          } while (v14 != v15);
          v6 = &v8[(long)v10];
        }
        else {
          v6 = (char *)sub_16eb0(v9 + 1);
          v8 = v6;
        }
        free(v16);
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
  void *v1; // rax
  void *v2; // rax
  
  v1 = (void *)sub_17af0();
  if (!v1)
    return NULL;
  v2 = (void *)sub_15320(v1,a1);
  if (!closedir(v1))
    return v2;
  free(v2);
  return NULL;
}

// Function: sub_15680 @ 0x15680
unsigned long * sub_15680(unsigned long *a0,unsigned long a1) // early-return
{
  unsigned long *v1;
  unsigned long *v2;
  void *v3; // rax
  
  v1 = (unsigned long *)*a0;
  if (v1 != &a0[2]) {
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
void sub_156e0(long *a0)
{
  a0[1] = 0x400;
  *a0 = (long)&a0[2];
}

// Function: sub_156f0 @ 0x156f0
void sub_156f0(unsigned long *a0)
{
  if ((unsigned long *)*a0 != &a0[2]) {
    free((unsigned long *)*a0); // tail-call
    return;
  }
}

// Function: sub_15710 @ 0x15710
unsigned long sub_15710(unsigned long *a0)
{
  unsigned long v1;
  void *v2; // rax
  
  v1 = a0[1] * 2;
  sub_156f0(a0);
  if ((unsigned long)a0[1] <= v1) { // branch-flip
    v2 = malloc(v1);
    if (v2) {
      *a0 = v2;
      a0[1] = v1;
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
void sub_15780(long *a0)
{
  a0[1] = 0x400;
  *a0 = (long)&a0[2];
}

// Function: sub_15790 @ 0x15790
unsigned long sub_15790(unsigned long *a0) // return-dupe
{
  unsigned long v1;
  void *v2; // rax
  unsigned long *v3;
  
  v3 = (unsigned long *)*a0;
  v1 = a0[1] * 2;
  if (v3 != &a0[2]) {
    if ((unsigned long)a0[1] <= v1) { // branch-flip
      v2 = realloc(v3,v1);
      if (v2) goto label_157c7;
      v3 = (void *)*a0;
    }
    else {
      *__errno_location() = 0xc;
    }
    free(v3);
    sub_15780(a0);
    return 0;
  }
  v2 = malloc(v1);
  if (!v2)
    return 0;
  v2 = memcpy(v2,v3,a0[1]);
label_157c7:
  *a0 = v2;
  a0[1] = v1;
  return 1;
}

// Function: sub_15840 @ 0x15840
int sub_15840(int a0,unsigned long *a1)
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

// Function: sub_158c0 @ 0x158c0
void sub_158c0(unsigned long a0,unsigned long a1)
{
  sub_15840(getfilecon(),a1); // tail-call
}

// Function: sub_158e0 @ 0x158e0
void sub_158e0(unsigned long a0,unsigned long a1)
{
  sub_15840(lgetfilecon(),a1); // tail-call
}

// Function: sub_15900 @ 0x15900
void sub_15900(unsigned long a0,unsigned long a1)
{
  sub_15840(fgetfilecon(),a1); // tail-call
}

// Function: sub_15920 @ 0x15920
unsigned long sub_15920(long a0)
{
  return *(unsigned long *)(a0 + 0x50);
}

// Function: sub_15930 @ 0x15930
unsigned long sub_15930(long a0)
{
  return *(unsigned long *)(a0 + 0x70);
}

// Function: sub_15940 @ 0x15940
unsigned long sub_15940(long a0)
{
  return *(unsigned long *)(a0 + 0x60);
}

// Function: sub_15950 @ 0x15950
unsigned long sub_15950(void)
{
  return 0;
}

// Function: sub_15960 @ 0x15960
undefined16 sub_15960(long a0)
{
  return *(char (*)[16])(a0 + 0x48);
}

// Function: sub_15970 @ 0x15970
undefined16 sub_15970(long a0)
{
  return *(char (*)[16])(a0 + 0x68);
}

// Function: sub_15980 @ 0x15980
undefined16 sub_15980(long a0)
{
  return *(char (*)[16])(a0 + 0x58);
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
  char v3 [152];
  
  v1 = lstat(a0,v3);
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
long sub_15b10(unsigned long a0,char a1)
{
  long v1; // rax
  long v2; // stack - 0x30
  char v3 [8];
  unsigned long v4; // stack - 0x20
  
  if ((a1) && (v1 = getrandom(&v2,8,1), v1 == 8))
    v1 = v2;
  else {
    clock_gettime(1,v3);
    v1 = (a0 ^ v4) * 0x27bb2ee687b0b0fd + 0xb504f32d;
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
void sub_15da0(unsigned long a0,unsigned long a1,unsigned int a2,int a3)
{
  unsigned int v1 [3]; // stack - 0xc
  
  v1[0] = a2;
  sub_15ba0(a0,a1,v1,*(unsigned long *)((long)a3 * 8 + 0x24980));
}

// Function: sub_15dd0 @ 0x15dd0
void sub_15dd0(void)
{
  sub_15da0(); // tail-call
}

// Function: sub_15de0 @ 0x15de0
void sub_15de0(void)
{
  sub_15ba0(); // tail-call
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
unsigned int sub_15e70(int a0,char *a1,unsigned long *a2,long a3,unsigned int a4)
{
  unsigned long v1;
  unsigned long *v10;
  unsigned long v11;
  long v12;
  int v13;
  unsigned int v14;
  unsigned long v15; // stack - 0x108
  unsigned long v16; // stack - 0xf8
  char v17 [88];
  int v18;
  unsigned int v19;
  unsigned long v2;
  unsigned long v20;
  bool v21; // r12b
  bool v22;
  unsigned int v23; // stack - 0x148
  unsigned int v24; // stack - 0x100
  char v25; // stack - 0xfc
  long v26; // stack - 0xf0
  unsigned long v27; // stack - 0xe8
  long v28; // stack - 0xe0
  unsigned long v29; // stack - 0x80
  unsigned long v3;
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8;
  unsigned int v9;
  
  v1 = a2[0xb];
  v20 = *(unsigned long *)(a3 + 0x58);
  v4 = sub_15940(a2);
  v5 = sub_15940(a3);
  v21 = v1 == v20;
  v14 = a4 & 1;
  if (!(a4 & 1)) goto label_15f98;
  if (v4 == v5 && v21)
    return 0;
  if ((long)v1 < (long)(v20 - 1))
    return 0xffffffff;
  if ((long)v20 < (long)(v1 - 1))
    return v14;
  v12 = dat_25700;
  if ((dat_25700) || (v12 = sub_11cd0(0x10,0,sub_15e50,sub_15e60,dat_24fd0), dat_25700 = v12, v12)) {
    v10 = dat_256f8;
    if (dat_256f8) { // branch-flip
label_15f2b:
      *v10 = *a2;
      v10 = (unsigned long *)sub_122f0(v12);
      if (!v10) {
        if (!dat_25700) goto label_16053;
        goto label_164c0;
      }
      if (dat_256f8 == v10)
        dat_256f8 = NULL;
    }
    else {
      v10 = malloc(0x10);
      dat_256f8 = v10;
      if (v10) {
        *(unsigned int *)&v10[1] = 2000000000;
        *(char *)((long)v10 + 0xc) = 0;
        goto label_15f2b;
      }
label_164c0:
      v15 = *a2;
      v10 = (unsigned long *)sub_11a70(dat_25700,&v15);
      if (!v10) goto label_16053;
    }
    v23 = *(unsigned int *)&v10[1];
    if (*(char *)((long)v10 + 0xc)) {
      v20 &= (long)(int)~(unsigned int)(v23 == 2000000000);
      v5 -= v5 % (int)v23;
      v21 = v1 == v20;
      goto label_15f98;
    }
  }
  else {
label_16053:
    v10 = &v15;
    v24 = 2000000000;
    v25 = 0;
    v23 = 2000000000;
  }
  v2 = a2[9];
  v3 = a2[0xd];
  v6 = sub_15920(a2);
  v7 = sub_15930(a2);
  v8 = v6 / 10;
  v13 = v7 / 10;
  v18 = v4 / 10;
  if (!(v4 % 10) && (!(v6 % 10) && !(v7 % 10))) {
    v22 = 0; // branch-flip
    v11 = v20;
    if (0xb <= (int)v23) {
      v19 = 10;
      if (v18 % 10 || (v13 % 10 || v8 % 10)) // branch-flip
        *(unsigned int *)&v10[1] = 10;
      else {
        v7 = 8;
        v9 = v19;
        do {
          v19 = v9 * 10;
          v8 /= 10;
          v13 /= 10;
          v18 /= 10;
          if (((int)v23 <= (int)v19) || (v18 % 10 || (v8 % 10 || v13 % 10))) {
            v22 = v9 == 200000000;
            *(unsigned int *)&v10[1] = v19;
            v11 = (long)(int)~(unsigned int)v22 & v20;
            goto label_162cb;
          }
          v7 -= 1;
          v9 = v19;
        } while (v7);
        if ((v2 | v1 | v3) & 1) { // branch-flip
          *(unsigned int *)&v10[1] = 1000000000;
          v19 = 1000000000;
        }
        else {
          *(unsigned int *)&v10[1] = 2000000000;
          v19 = 2000000000;
          v22 = 1;
          v11 = v20 & 0xfffffffffffffffe;
        }
      }
    }
    else {
      *(unsigned int *)&v10[1] = 10;
      v19 = 10;
    }
label_162cb:
    if ((long)v20 < (long)v1)
      return v14;
    if ((v5 <= v4) && (v21))
      return v14;
    if (((long)v1 < (long)v11) || ((v1 == v11 && (v4 < v5 - v5 % (int)v19))))
      return 0xffffffff;
    v27 = v22 | v1;
    v28 = (long)(int)(v19 / 9 + v4);
    v16 = v2;
    v26 = (long)v6;
    if (utimensat(a0,a1,&v16,0x100))
      return 0xfffffffe;
    v8 = fstatat(a0,a1,v17,0x100);
    v2 = v29;
    v23 = (unsigned int)v29;
    v12 = sub_15940(v17);
    if ((v4 != v12 || v8) || v2 != v1) {
      v27 = v1;
      v28 = (long)v4;
      utimensat(a0,a1,&v16,0x100);
      if (v8)
        return 0xfffffffe;
      v23 = (unsigned int)v29;
      v12 = sub_15940(v17);
    }
    v8 = (v23 & 1) * 1000000000 + (int)v12;
    v9 = v8 * -0x33333333 + 0x19999998;
    if ((v9 >> 1 | (unsigned int)((v9 & 1) != 0) << 0x1f) <= 0x19999998) {
      if (v19 != 10) { // branch-flip
        v13 = 9;
        v14 = 10;
        do {
          v8 /= 10;
          v9 = v8 * -0x33333333 + 0x19999998;
          if (0x19999998 < (v9 >> 1 | (unsigned int)((v9 & 1) != 0) << 0x1f)) break;
          v13 -= 1;
          if (!v13) {
            v14 = 2000000000;
            break;
          }
          v14 *= 10;
        } while (v14 != v19);
      }
      else {
        v14 = 10;
      }
    }
    v20 &= (long)(int)~(unsigned int)(v14 == 2000000000);
    v5 -= v5 % (int)v14;
    v21 = v1 == v20;
  }
  *(unsigned int *)&v10[1] = v14;
  *(char *)((long)v10 + 0xc) = 1;
label_15f98:
  return ((unsigned int)(v5 < v4) - (unsigned int)(v4 < v5) & -(unsigned int)v21) + ((unsigned int)((long)v20 < (long)v1) - (unsigned int)((long)v1 < (long)v20));
}

// Function: sub_16690 @ 0x16690
void sub_16690(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  sub_15e70(0xffffff9c,a0,a1,a2,a3); // tail-call
}

// Function: sub_166b0 @ 0x166b0
void sub_166b0(int a0,int a1,long a2,unsigned int a3,unsigned long a4,unsigned long a5)
{
  void *v1; // rax
  char *v2; // rax
  
  v1 = (void *)sub_17480(a4,a5);
  if (v1) {
    if (a2) // branch-flip
      error_at_line(a0,a1,a2,a3,"%s",v1);
    else {
      error(a0,a1,"%s",v1);
    }
    free(v1); // tail-call
    return;
  }
  v2 = dcgettext(NULL,"unable to display error message",5);
  error(0,*__errno_location(),v2);
  abort(); // no-return
}

// Function: sub_16760 @ 0x16760
void sub_16760(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_166b0(a0,a1,0,0,a2,a3); // tail-call
}

// Function: sub_16780 @ 0x16780
long sub_16780(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
void sub_16c20(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
void sub_16f80(void)
{
  sub_16f40(); // tail-call
}

// Function: sub_16f90 @ 0x16f90
void sub_16f90(void)
{
  sub_16e80(sub_126e0()); // tail-call
}

// Function: sub_16fb0 @ 0x16fb0
void sub_16fb0(unsigned long a0,unsigned long a1)
{
  sub_16f40(0,a0,a1); // tail-call
}

// Function: sub_16fc0 @ 0x16fc0
void sub_16fc0(unsigned long a0,unsigned long a1)
{
  sub_16f90(0,a0,a1); // tail-call
}

// Function: sub_16fd0 @ 0x16fd0
void sub_16fd0(long a0,unsigned long *a1,unsigned long a2)
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
void sub_17040(unsigned long a0,unsigned long a1)
{
  sub_16fd0(a0,a1,1); // tail-call
}

// Function: sub_17050 @ 0x17050
void sub_17050(long a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_170f6;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
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
void sub_17290(unsigned long a0,unsigned int a1)
{
  if (sub_105d0(a0,a1 | 0x200))
    return;
  if (*__errno_location() != 0x16)
    sub_17250(); // no-return
  __assert_fail("errno != EINVAL","lib/xfts.c",0x29,"xfts_open"); // no-return
}

// Function: sub_172e0 @ 0x172e0
bool sub_172e0(long a0,long a1)
{
  unsigned int v1; // eax
  bool v2; // r8b
  
  v2 = 1;
  v1 = *(unsigned int *)(a0 + 0x48) & 0x11;
  if ((v1 != 0x10) && (v2 = 0, v1 == 0x11))
    return *(long *)(a1 + 0x58) != 0;
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
unsigned long sub_17480(char *a0,unsigned long a1) // early-return
{
  char v1;
  unsigned long v2; // stack - 0x18
  long v3;
  
  v1 = *a0;
  v3 = 0;
  while( true ) {
    if (!v1)
      return sub_17320(v3,a1); // tail-call
    if ((v1 != '%') || (a0[v3 * 2 + 1] != 's')) break;
    v3 += 1;
    v1 = a0[v3 * 2];
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

// Function: sub_175e0 @ 0x175e0
unsigned long sub_175e0(unsigned long *a0,unsigned long a1,unsigned long a2) // early-return x2
{
  void *v1; // rax
  unsigned long v2; // stack - 0x18
  
  v1 = (void *)sub_18000(0,&v2,a1,a2);
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
void sub_17680(unsigned long a0,int a1,unsigned int a2)
{
  if (a1 != -1) {
    fchmod(a1,a2); // tail-call
    return;
  }
  chmod(a0); // tail-call
}

// Function: sub_176a0 @ 0x176a0
int sub_176a0(unsigned int *a0,unsigned long a1,unsigned int a2)
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
void sub_17710(void)
{
  sub_176d0(); // tail-call
}

// Function: sub_17720 @ 0x17720
void sub_17720(void)
{
  sub_176e0(); // tail-call
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
    v2 = sub_17900(*a0);
    v1 = sub_17900(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_179a0 @ 0x179a0
unsigned long sub_179a0(void *a0) // early-return x2, return-dupe
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
void sub_17a20(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_17a40 @ 0x17a40
unsigned long sub_17a40(long *a0,long *a1) // return-dupe x3
{
  char v1; // al
  long v2;
  long *v3; // rdx
  long *v4; // rdx
  long *v5;
  long v6;
  
  if ((int)a0[3] != 0x95f616)
    __assert_fail("state->magic == 9827862","lib/cycle-check.c",0x3c,"cycle_check"); // no-return
  if (a0[2]) { // branch-flip
    v6 = a1[1];
    if ((v6 == *a0) && (*a1 == a0[1]))
      return 1;
    v2 = a0[2] + 1;
    a0[2] = v2;
    v1 = sub_17a10(v2);
    if (!v1)
      return 0;
    v5 = v3;
    if (!v2)
      return 1;
  }
  else {
    a0[2] = 1;
    if (!sub_17a10(1))
      return 0;
    v6 = a1[1];
    v5 = v4;
  }
  v2 = *a1;
  *v5 = v6;
  v5[1] = v2;
  return 0;
}

// Function: sub_17af0 @ 0x17af0
void * sub_17af0(char *a0)
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
unsigned long sub_17b90(void *a0) // early-return
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
    if (sub_e9c0(a0)) {
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
unsigned long sub_17ee0(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_19020(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
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
void * sub_18000(void *a0,unsigned long *a1,int *a2,unsigned long a3)
{
  undefined10 v1;
  unsigned long v10;
  void *v11;
  int *v12;
  int *v13; // rax
  unsigned long v14;
  unsigned long v15;
  void *v16;
  char v17 [8];
  char v18 [8];
  long v19; // stack - 0x2c8
  char v2;
  unsigned long v20 [77];
  int v21 [56];
  unsigned int v22 [2]; // stack - 0x3c0
  int v23; // stack - 0x3c4
  char *v24;
  char *v25;
  char *v26; // rsp
  char *v27; // rsp
  unsigned long v28;
  unsigned long v29;
  unsigned int v3;
  char *v30;
  unsigned long *v31;
  bool v32; // zf
  unsigned long v33; // stack - 0x420
  int v34; // stack - 0x414
  char *v35; // stack - 0x410
  unsigned long *v36; // stack - 0x408
  char *v37; // stack - 0x400
  long v38; // stack - 0x3f8
  char *v39; // stack - 0x3f0
  long v4;
  int *v40; // stack - 0x3e8
  int *v41; // stack - 0x3e0
  unsigned long v42; // stack - 0x3d8
  void *v43; // stack - 0x3d0
  int *v44; // stack - 0x3b0
  unsigned long *v45; // stack - 0x2c0
  unsigned long v46; // stack - 0x2b8
  unsigned long v47; // stack - 0x2b0
  unsigned long *v5;
  char v6;
  int v7;
  int v8;
  unsigned long v9;
  
  v27 = v18;
  v25 = v18;
  v24 = v18;
  v36 = a1;
  v43 = a0;
  if ((int)sub_193c0(a2,&v19,v17) <= -1)
    return (void *)0;
  if ((int)sub_19190(a3,v17) < 0) {
    if (v45 != v20)
      free(v45);
    if (v44 != v21)
      free(v44);
    *__errno_location() = 0x16;
    return NULL;
  }
  v9 = sub_17fd0(7,v46,v47,6);
  if (4000 <= v9) { // branch-flip
    if (v9 != 0xffffffffffffffff) {
      v39 = malloc(v9);
      if (v39) {
        v35 = v39;
        v24 = v18;
        goto label_180c1;
      }
    }
    v41 = __errno_location();
  }
  else {
    v24 = v18;
    while (v27 != &v18[-(v9 + 0x17 & 0xfffffffffffff000)]) {
      v25 = &v24[-0x1000];
      v26 = &v24[-0x1000];
      *(unsigned long *)&v24[-8] = *(unsigned long *)&v24[-8];
      v27 = &v24[-0x1000];
      v24 = v26;
    }
    v9 = (unsigned long)((unsigned int)(v9 + 0x17) & 0xff0);
    v4 = -v9;
    v24 = &v25[v4];
    if (v9)
      *(unsigned long *)&v25[-8] = *(unsigned long *)&v25[-8];
    v35 = NULL;
    v39 = (char *)((unsigned long)&v25[v4 + 0xf] & 0xfffffffffffffff0);
label_180c1:
    v9 = 0;
    if (v43)
      v9 = *v36;
    v28 = 0;
    v38 = 0;
    v12 = (int *)*v45;
    v31 = v45;
    v16 = v43;
    if (v12 == a2) goto label_181ba;
    while( true ) {
      v41 = a2;
      v42 = (long)v12 - (long)a2;
      *(unsigned long *)&v24[-8] = 0x18124;
      v10 = sub_17f90(v28,(long)v12 - (long)a2);
      v12 = v41;
      v14 = v42;
      v11 = v16;
      if (v10 <= v9) goto label_181ab;
      if (v9) break;
      v9 = 0xc;
label_1814c:
      if (v9 < v10)
        v9 = v10;
      if (v9 == 0xffffffffffffffff) goto label_18318;
      if ((!v16) || (v16 == v43)) {
        v40 = v41;
        v41 = (int *)CONCAT71(v41._1_7_,v16 == v43);
        *(unsigned long *)&v24[-8] = 0x18b0c;
        v11 = malloc(v9,v24[-8]);
        if (v11) {
          if ((v28) && ((char)v41)) {
            v41 = v40;
            *(unsigned long *)&v24[-8] = 0x18b5a;
            v11 = memcpy(v11,v16,v28,v24[-8]);
            v12 = v41;
            v14 = v42;
          }
          else {
            v12 = v40;
            v14 = v42;
          }
          goto label_181ab;
        }
        goto label_18318;
      }
      *(unsigned long *)&v24[-8] = 0x18191;
      v11 = realloc(v16,v9,v24[-8]);
      if (!v11) {
label_18bce:
        *(unsigned long *)&v24[-8] = 0x18bd3;
        v6 = v24[-8];
        v41 = __errno_location(v6);
        goto label_18be0;
      }
      v12 = v41;
      v14 = v42;
label_181ab:
      *(unsigned long *)&v24[-8] = 0x181ba;
      memcpy((void *)((long)v11 + v28),v12,v14,v24[-8]);
      v28 = v10;
      v16 = v11;
label_181ba:
      do {
        if (v19 == v38) {
          *(unsigned long *)&v24[-8] = 0x18ee5;
          v14 = sub_17f90(v28,1);
          v11 = v16;
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
            v32 = v16 == v43;
            if ((!v16) || (v32)) {
              *(unsigned long *)&v24[-8] = 0x18fc5;
              v11 = malloc(v9,v24[-8]);
              if (!v11) goto label_18318;
              if ((v28) && (v32)) {
                *(unsigned long *)&v24[-8] = 0x18fe9;
                v11 = memcpy(v11,v16,v28,v24[-8]);
              }
            }
            else {
              *(unsigned long *)&v24[-8] = 0x18f39;
              v11 = realloc(v16,v9,v24[-8]);
              if (!v11) goto label_18bce;
            }
          }
          *(char *)((long)v11 + v28) = 0;
          if (v11 != v43) {
            if (v28 + 1 < v9) {
              *(unsigned long *)&v24[-8] = 0x18ff9;
              v16 = realloc(v11,v28 + 1,v24[-8]);
              if (v16)
                v11 = v16;
            }
          }
          if (v35) {
            *(unsigned long *)&v24[-8] = 0x18f74;
            free(v35,v24[-8]);
          }
          if (v45 != v20) {
            *(unsigned long *)&v24[-8] = 0x18f8c;
            free(v45,v24[-8]);
          }
          if (v44 != v21) {
            *(unsigned long *)&v24[-8] = 0x18fa4;
            free(v44,v24[-8]);
          }
          *v36 = v28;
          return v11;
        }
        v4 = v31[10];
        if (*(char *)&v31[9] != '%') { // branch-flip
          if (v4 == -1) {
label_19005:
            *(unsigned long *)&v24[-8] = 0x1900a; // no-return
            abort(v24[-8]);
          }
          v8 = v44[v4 * 8];
          v42 = CONCAT44(v42._4_4_,v8);
          if (*(char *)&v31[9] != 'n') { // branch-flip
            v3 = *(unsigned int *)&v31[2];
            v30 = &v39[1];
            *v39 = '%';
            if (v3 & 1) {
              v39[1] = '\'';
              v30 = &v39[2];
            }
            if (v3 & 2) {
              *v30 = '-';
              v30 = &v30[1];
            }
            if (v3 & 4) {
              *v30 = '+';
              v30 = &v30[1];
            }
            if (v3 & 8) {
              *v30 = ' ';
              v30 = &v30[1];
            }
            if (v3 & 0x10) {
              *v30 = '#';
              v30 = &v30[1];
            }
            if (v3 & 0x40) {
              *v30 = 'I';
              v30 = &v30[1];
            }
            if (v3 & 0x20) {
              *v30 = '0';
              v30 = &v30[1];
            }
            v11 = (void *)v31[3];
            v12 = v44;
            if (v11 != (void *)v31[4]) {
              v12 = (int *)((long)(void *)v31[4] - (long)v11);
              v40 = v44;
              v41 = v12;
              *(unsigned long *)&v24[-8] = 0x182a5;
              memcpy(v30,v11,(unsigned long)v12,v24[-8]);
              v30 = &v30[(long)v41];
              v12 = v40;
            }
            v11 = (void *)v31[6];
            if (v11 != (void *)v31[7]) {
              v13 = (int *)((long)(void *)v31[7] - (long)v11);
              v40 = v12;
              v41 = v13;
              *(unsigned long *)&v24[-8] = 0x182df;
              memcpy(v30,v11,(unsigned long)v13,v24[-8]);
              v30 = &v30[(long)v41];
              v12 = v40;
            }
            switch((unsigned int)v42) {
              case 9:
              case 10:
                *v30 = 'l';
                v30 = &v30[1];
                goto label_18421;
              case 7:
              case 8:
              case 0xe:
              case 0x10:
                *v30 = 'l';
                v30 = &v30[1];
                break;
              case 0xc:
                *v30 = 'L';
                v30 = &v30[1];
              
            }
            v2 = *(char *)&v31[9];
            v30[1] = '\0';
            *v30 = v2;
            if (v31[5] != -1) { // branch-flip
              if (v12[v31[5] * 8] != 5) goto label_19005;
              v40 = (int *)CONCAT44(v40._4_4_,1);
              v22[0] = (&v12[v31[5] * 8])[4];
            }
            else {
              v40 = (int *)((unsigned long)v40._4_4_ << 0x20);
            }
            if (v31[8] != -1) {
              if (v12[v31[8] * 8] != 5) goto label_19005;
              v22[(unsigned long)v40 & 0xffffffff] = (&v12[v31[8] * 8])[4];
              v40 = (int *)CONCAT44(v40._4_4_,(int)v40 + 1);
            }
            *(unsigned long *)&v24[-8] = 0x184aa;
            v14 = sub_17f90(v28,2);
            v11 = v16;
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
              if ((!v16) || (v16 == v43)) {
                v41 = (int *)CONCAT71(v41._1_7_,v16 == v43);
                *(unsigned long *)&v24[-8] = 0x18d2e;
                v11 = malloc(v9,v24[-8]);
                if (!v11) goto label_18318;
                if ((v28) && ((char)v41)) {
                  *(unsigned long *)&v24[-8] = 0x18d60;
                  v11 = memcpy(v11,v16,v28,v24[-8]);
                }
              }
              else {
                *(unsigned long *)&v24[-8] = 0x184f4;
                v11 = realloc(v16,v9,v24[-8]);
                if (!v11) goto label_18318;
              }
            }
            *(char *)((long)v11 + v28) = 0;
            *(unsigned long *)&v24[-8] = 0x1850a;
            v41 = __errno_location(v24[-8]);
            v37 = v30;
            v34 = *v41;
            v16 = v11;
label_18520:
            v23 = -1;
            v10 = v9 - v28;
            *v41 = 0;
            v14 = 0x7fffffff;
            if (v10 <= 0x7fffffff)
              v14 = v10;
            switch(v42 & 0xffffffff) {
              default:
                goto label_19005;
              case 1:
                v29 = (unsigned long)(unsigned int)(int)(char)v44[v31[10] * 8 + 4];
                break;
              case 2:
                v29 = (unsigned long)*(unsigned char *)&v44[v31[10] * 8 + 4];
                break;
              case 3:
                v29 = (unsigned long)(unsigned int)(int)(short)v44[v31[10] * 8 + 4];
                break;
              case 4:
                v29 = (unsigned long)*(unsigned short *)&v44[v31[10] * 8 + 4];
                break;
              case 5:
              case 6:
              case 0xd:
              case 0xe:
                v29 = (unsigned long)(unsigned int)v44[v31[10] * 8 + 4];
                break;
              case 7:
              case 8:
              case 9:
              case 10:
              case 0xf:
              case 0x10:
              case 0x11:
                v12 = *(int **)&v44[v31[10] * 8 + 4];
                if ((int)v40 == 1) {
                  *(int **)&v24[-8] = &v23;
                  *(int **)&v24[-0x10] = v12;
                  goto label_188ca;
                }
                if ((int)v40 != 2) {
                  *(int **)&v24[-0x10] = &v23;
label_185ab:
                  v33 = v14;
                  *(unsigned long *)&v24[-0x18] = 0x185cc;
                  v8 = __snprintf_chk((char *)((long)v16 + v28),v14,1,0xffffffffffffffff,v39,v12);
                  v14 = v33;
                  goto label_185d5;
                }
                *(int **)&v24[-0x10] = &v23;
                *(int **)&v24[-0x18] = v12;
                goto label_1891e;
              case 0xb:
                v15 = *(unsigned long *)&v44[v31[10] * 8 + 4];
                if ((int)v40 != 1) { // branch-flip
                  if ((int)v40 != 2) {
                    v33 = v14;
                    *(unsigned long *)&v24[-8] = 0x18c2b;
                    v8 = __snprintf_chk((char *)((long)v16 + v28),v14,1,0xffffffffffffffff,v39,v15,&v23);
                    v14 = v33;
                    goto label_185d5;
                  }
                  *(int **)&v24[-8] = &v23;
                  *(unsigned long *)&v24[-0x10] = (unsigned long)v22[1];
                }
                else {
                  *(int **)&v24[-0x10] = &v23;
                }
                v33 = v14;
                *(unsigned long *)&v24[-0x18] = 0x18882;
                v8 = __snprintf_chk((char *)((long)v16 + v28),v14,1,0xffffffffffffffff,v39,v15,(unsigned long)v22[0]);
                v14 = v33;
                goto label_185d5;
              case 0xc:
                v1 = *(void *)&v44[v31[10] * 8 + 4];
                if ((int)v40 != 1) {
                  if ((int)v40 != 2) {
                    v12 = &v23;
                    *(void *)&v24[-0x10] = v1;
                    goto label_185ab;
                  }
                  *(int **)&v24[-0x10] = &v23;
                  v33 = v14;
                  *(void *)&v24[-0x20] = v1;
                  *(unsigned long *)&v24[-0x30] = (unsigned long)v22[1];
                  *(unsigned long *)&v24[-0x38] = 0x18c83;
                  v8 = __snprintf_chk((char *)((long)v16 + v28),v14,1,0xffffffffffffffff,v39,(unsigned long)v22[0]);
                  v14 = v33;
                  goto label_185d5;
                }
                *(int **)&v24[-0x10] = &v23;
                *(void *)&v24[-0x20] = v1;
                goto label_18925;
              
            }
            if ((int)v40 == 1) {
              *(int **)&v24[-8] = &v23;
              *(unsigned long *)&v24[-0x10] = v29;
label_188ca:
              v29 = (unsigned long)v22[0];
label_186eb:
              v33 = v14;
              *(unsigned long *)&v24[-0x18] = 0x1870c;
              v8 = __snprintf_chk((char *)((long)v16 + v28),v14,1,0xffffffffffffffff,v39,v29);
              v14 = v33;
              if (0 <= v23) goto label_185e3;
label_18723:
              if (!v37[1]) {
                if (v8 <= -1) {
                  if (!*v41) {
                    v8 = 0x16;
                    if ((*(unsigned char *)&v31[9] & 0xef) == 99)
                      v8 = 0x54;
                    *v41 = v8;
                  }
                  if ((v16 != v43) && (v16)) {
                    *(unsigned long *)&v24[-8] = 0x1877f;
                    free(v16,v24[-8]);
                  }
                  if (v35) {
                    *(unsigned long *)&v24[-8] = 0x18793;
                    free(v35,v24[-8]);
                  }
                  if (v45 != v20) {
                    *(unsigned long *)&v24[-8] = 0x187ab;
                    free(v45,v24[-8]);
                  }
                  if (v44 != v21) {
                    *(unsigned long *)&v24[-8] = 0x187c7;
                    free(v44,v24[-8]);
                    return (void *)0;
                  }
                  return (void *)0;
                }
                goto label_185fd;
              }
              v37[1] = '\0';
              goto label_18520;
            }
            if ((int)v40 != 2) {
              *(int **)&v24[-0x10] = &v23;
              goto label_186eb;
            }
            *(int **)&v24[-0x10] = &v23;
            *(unsigned long *)&v24[-0x18] = v29;
label_1891e:
            *(unsigned long *)&v24[-0x20] = (unsigned long)v22[1];
label_18925:
            v33 = v14;
            *(unsigned long *)&v24[-0x28] = 0x1894d;
            v8 = __snprintf_chk((char *)((long)v16 + v28),v14,1,0xffffffffffffffff,v39,(unsigned long)v22[0]);
            v14 = v33;
label_185d5:
            if (v23 <= -1) goto label_18723;
label_185e3:
            if (((unsigned long)(long)v23 < v14) && (*(char *)((long)v16 + v28 + (long)v23))) goto label_19005;
            v7 = v23;
            if (v23 < v8) {
label_185fd:
              v23 = v8;
              v7 = v8;
            }
            if (v14 <= v7 + 1) {
              if (0x7fffffff <= v10) {
                if ((v16 != v43) && (v16)) {
                  *(unsigned long *)&v24[-8] = 0x18e4e;
                  free(v16,v24[-8]);
                }
                if (v35) {
                  *(unsigned long *)&v24[-8] = 0x18e62;
                  free(v35,v24[-8]);
                }
                if (v45 != v20) {
                  *(unsigned long *)&v24[-8] = 0x18e7a;
                  free(v45,v24[-8]);
                }
                if (v44 != v21) {
                  *(unsigned long *)&v24[-8] = 0x18e92;
                  free(v44,v24[-8]);
                }
                *v41 = 0x4b;
                return NULL;
              }
              v14 = v9 * 2;
              if ((long)v9 <= -1)
                v14 = 0xffffffffffffffff;
              *(unsigned long *)&v24[-8] = 0x1863b;
              v15 = sub_17f90(v28,v7 + 2);
              *(unsigned long *)&v24[-8] = 0x18646;
              v10 = sub_17ff0(v15,v14);
              if (v9 < v10) {
                if (!v9)
                  v14 = 0xc;
                v9 = v10;
                if (v10 <= v14)
                  v9 = v14;
                if (v9 == 0xffffffffffffffff) goto label_18324;
                v32 = v16 == v43;
                if ((!v16) || (v32)) {
                  *(unsigned long *)&v24[-8] = 0x189c8;
                  v11 = malloc(v9,v24[-8]);
                  if (!v11) goto label_18324;
                  if ((!v28) || (!v32))
                    v16 = v11;
                  else {
                    *(unsigned long *)&v24[-8] = 0x189f4;
                    v16 = memcpy(v11,v16,v28,v24[-8]);
                  }
                }
                else {
                  *(unsigned long *)&v24[-8] = 0x18697;
                  v11 = realloc(v16,v9,v24[-8]);
                  if (!v11) goto label_18324;
                  v16 = v11;
                }
              }
              goto label_18520;
            }
            v28 = (long)v7 + v28;
            *v41 = v34;
          }
          else {
            if (5 <= (unsigned int)(v8 - 0x12U)) goto label_19005;
            v5 = *(unsigned long **)&(&v44[v4 * 8])[4];
            switch(v8) {
              case 0x12:
                *(char *)v5 = (char)v28;
                break;
              case 0x13:
                *(short *)v5 = (short)v28;
                break;
              case 0x14:
                *(int *)v5 = (int)v28;
                break;
              default:
                *v5 = v28;
              
            }
          }
        }
        else {
          if (v4 != -1) goto label_19005;
          *(unsigned long *)&v24[-8] = 0x18a57;
          v14 = sub_17f90(v28,1);
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
            if ((!v16) || (v16 == v43)) {
              v42 = CONCAT71(v42._1_7_,v16 == v43);
              *(unsigned long *)&v24[-8] = 0x18d7e;
              v11 = malloc(v9,v24[-8]);
              if (!v11) goto label_18318;
              if ((v28) && ((char)v42)) {
                *(unsigned long *)&v24[-8] = 0x18db0;
                v16 = memcpy(v11,v16,v28,v24[-8]);
              }
              else {
                v16 = v11;
              }
            }
            else {
              *(unsigned long *)&v24[-8] = 0x18aa4;
              v11 = realloc(v16,v9,v24[-8]);
              if (!v11) goto label_18bce;
              v16 = v11;
            }
          }
          *(char *)((long)v16 + v28) = 0x25;
          v28 = v14;
        }
        a2 = (int *)v31[1];
        v12 = (int *)v31[0xb];
        v31 = &v31[0xb];
        v38 += 1;
      } while (v12 == a2);
    }
    if (0 <= (long)v9) {
      v9 *= 2;
      goto label_1814c;
    }
label_18318:
    *(unsigned long *)&v24[-8] = 0x1831d;
    v6 = v24[-8];
    v41 = __errno_location(v6);
label_18324:
    if ((v16 != v43) && (v16)) {
label_18be0:
      *(unsigned long *)&v24[-8] = 0x18be8;
      free(v16,v24[-8]);
    }
    if (v35) {
      *(unsigned long *)&v24[-8] = 0x1834a;
      free(v35,v24[-8]);
    }
  }
  if (v45 != v20) {
    *(unsigned long *)&v24[-8] = 0x18362;
    free(v45,v24[-8]);
  }
  if (v44 != v21) {
    *(unsigned long *)&v24[-8] = 0x1837a;
    free(v44,v24[-8]);
  }
  *v41 = 0xc;
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
void sub_19170(void)
{
  sub_190c0(); // tail-call
}

// Function: sub_19180 @ 0x19180
void sub_19180(void)
{
  sub_190b0(); // tail-call
}

// Function: sub_19190 @ 0x19190
unsigned long sub_19190(unsigned int *a0,long *a1) // return-dupe x2
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
  
  v2 = *a1;
  v3 = (unsigned int *)a1[1];
  if (v2) {
    v6 = 0;
    do {
      switch(*v3) {
        default:
          return 0xffffffff;
        case 1:
        case 2:
          v1 = *a0;
          if (0x30 <= v1) { // branch-flip
            v8 = *(unsigned int **)&a0[2];
            *(unsigned int **)&a0[2] = &v8[2];
          }
          else {
            v8 = (unsigned int *)((unsigned long)v1 + *(long *)&a0[4]);
            *a0 = v1 + 8;
          }
          *(char *)&v3[4] = (char)*v8;
          break;
        case 3:
        case 4:
          v1 = *a0;
          if (0x30 <= v1) { // branch-flip
            v8 = *(unsigned int **)&a0[2];
            *(unsigned int **)&a0[2] = &v8[2];
          }
          else {
            v8 = (unsigned int *)((unsigned long)v1 + *(long *)&a0[4]);
            *a0 = v1 + 8;
          }
          *(short *)&v3[4] = (short)*v8;
          break;
        case 5:
        case 6:
        case 0xd:
        case 0xe:
          v1 = *a0;
          if (0x30 <= v1) { // branch-flip
            v8 = *(unsigned int **)&a0[2];
            *(unsigned int **)&a0[2] = &v8[2];
          }
          else {
            v8 = (unsigned int *)((unsigned long)v1 + *(long *)&a0[4]);
            *a0 = v1 + 8;
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
          v1 = *a0;
          if (0x30 <= v1) { // branch-flip
            v7 = *(unsigned long **)&a0[2];
            *(unsigned long **)&a0[2] = &v7[1];
          }
          else {
            v7 = (unsigned long *)((unsigned long)v1 + *(long *)&a0[4]);
            *a0 = v1 + 8;
          }
          *(unsigned long *)&v3[4] = *v7;
          break;
        case 0xb:
          v1 = a0[1];
          if (0xb0 <= v1) { // branch-flip
            v7 = *(unsigned long **)&a0[2];
            *(unsigned long **)&a0[2] = &v7[1];
          }
          else {
            v7 = (unsigned long *)((unsigned long)v1 + *(long *)&a0[4]);
            a0[1] = v1 + 0x10;
          }
          *(unsigned long *)&v3[4] = *v7;
          break;
        case 0xc:
          v4 = (void *)(*(long *)&a0[2] + 0xfU & 0xfffffffffffffff0);
          *(void **)&a0[2] = &v4[1];
          *(void *)&v3[4] = *v4;
          break;
        case 0xf:
          v1 = *a0;
          if (0x30 <= v1) { // branch-flip
            v9 = *(long **)&a0[2];
            *(long **)&a0[2] = &v9[1];
          }
          else {
            v9 = (long *)((unsigned long)v1 + *(long *)&a0[4]);
            *a0 = v1 + 8;
          }
          v5 = (char *)*v9;
          if (!v5)
            v5 = "(NULL)";
          *(char **)&v3[4] = v5;
          break;
        case 0x10:
          v1 = *a0;
          if (0x30 <= v1) { // branch-flip
            v9 = *(long **)&a0[2];
            *(long **)&a0[2] = &v9[1];
          }
          else {
            v9 = (long *)((unsigned long)v1 + *(long *)&a0[4]);
            *a0 = v1 + 8;
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
unsigned long sub_193c0(unsigned char *a0,unsigned long *a1,unsigned long *a2) // return-dupe x2
{
  unsigned long *v1;
  unsigned long v10;
  int v11;
  unsigned int v12;
  unsigned int v13; // edx
  unsigned long v14;
  unsigned char *v15;
  unsigned char *v16;
  unsigned char v17;
  char v18;
  unsigned int v19;
  unsigned long *v2;
  long v20;
  unsigned long v21;
  unsigned long v22;
  unsigned long v23;
  unsigned long v24;
  unsigned long v25; // stack - 0x78
  unsigned long v26; // stack - 0x70
  unsigned long v27; // stack - 0x60
  int v3;
  unsigned long *v4;
  unsigned long *v5; // rax
  unsigned long v6;
  unsigned long *v7;
  unsigned long *v8;
  unsigned char *v9;
  
  v1 = &a1[4];
  v2 = &a2[2];
  v21 = 7;
  *a1 = 0;
  a1[1] = (unsigned long)v1;
  v6 = 0;
  *a2 = 0;
  a2[1] = (unsigned long)v2;
  v26 = 0;
  v25 = 0;
  v27 = 0;
  v4 = v1;
  v23 = 7;
label_19444:
  if (!*a0) {
    v4[v6 * 0xb] = (unsigned long)a0;
    a1[2] = v25;
    a1[3] = v26;
    return 0;
  }
  v16 = &a0[1];
  v22 = v23;
  if (*a0 == 0x25) {
    v24 = 0xffffffffffffffff;
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
    v19 = (unsigned int)a0[1];
    if (10 <= (unsigned char)(a0[1] - 0x30)) {
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
          v19 = (unsigned int)*v16;
          v9 = &v16[1];
          if (*v16 != 0x27) break;
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
    } while ((unsigned char)(*v15 - 0x30) <= 9);
    v24 = 0xffffffffffffffff;
    if (*v15 != 0x24) goto label_194ee;
    v24 = 0;
    while( true ) {
      v20 = -1;
      if (v24 <= 0x1999999999999999)
        v20 = v24 * 10;
      v24 = sub_17f90(v20,(long)(char)(v19 - 0x30),v19 - 0x30);
      v19 = (unsigned int)v16[1];
      v9 = &v16[1];
      if (10 <= (unsigned char)(v16[1] - 0x30)) break;
      v16 = v9;
    }
    v24 -= 1;
    if (v24 <= 0xfffffffffffffffd) {
      v19 = (unsigned int)v16[2];
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
      if ((unsigned char)(*v16 - 0x30) <= 9) {
        do {
          v15 = &v9[1];
          v9 = &v9[1];
        } while ((unsigned char)(*v15 - 0x30) <= 9);
        v6 = (long)v9 - (long)v16;
        if ((unsigned long)((long)v9 - (long)v16) <= v25)
          v6 = v25;
        v25 = v6;
      }
      v4[4] = (unsigned long)v9;
label_19a4c:
      v19 = (unsigned int)*v9;
    }
    v17 = (unsigned char)v19;
  }
  else {
    v4[3] = (unsigned long)v16;
    v4[4] = (unsigned long)v9;
    v17 = v16[1];
    v6 = 1;
    if (v25)
      v6 = v25;
    if (10 <= (unsigned char)(v17 - 0x30)) { // branch-flip
label_1975d:
      v4[5] = v27;
      v14 = v27 + 1;
      if (v27 == 0xffffffffffffffff) goto label_199d0;
      v10 = v27;
      v27 = v14;
    }
    else {
      v16 = v9;
      do {
        v15 = &v16[1];
        v16 = &v16[1];
      } while ((unsigned char)(*v15 - 0x30) <= 9);
      if (*v15 != 0x24) goto label_1975d;
      v10 = 0;
      while( true ) {
        v20 = -1;
        if (v10 <= 0x1999999999999999)
          v20 = v10 * 10;
        v10 = sub_17f90(v20,(long)(char)(v17 - 0x30));
        v17 = v9[1];
        v16 = &v9[1];
        if (10 <= (unsigned char)(v17 - 0x30)) break;
        v9 = v16;
      }
      v10 -= 1;
      if (0xfffffffffffffffe <= v10) goto label_199d0;
      v4[5] = v10;
      v9 = &v9[2];
    }
    if (v21 <= v10) { // branch-flip
      v21 *= 2;
      if (v21 <= v10)
        v21 = sub_17f90(v10,1);
      v7 = (unsigned long *)a2[1];
      if (0x800000000000000 <= v21) goto label_19cdd;
      if (v2 != v7) // branch-flip
        v8 = realloc(v7,v21 << 5);
      else {
        v8 = malloc(v21 << 5);
      }
      v7 = (unsigned long *)a2[1];
      if (!v8) goto label_19cdd;
      if (v2 == v7)
        v8 = memcpy(v8,v2,*a2 << 5);
      a2[1] = (unsigned long)v8;
    }
    else {
      v8 = (unsigned long *)a2[1];
    }
    v14 = *a2;
    v7 = &v8[v14 * 4];
    if (v14 <= v10) {
      do {
        v5 = v7;
        v14 += 1;
        *(unsigned int *)v5 = 0;
        v7 = &v5[4];
      } while (v14 <= v10);
      *a2 = v14;
      *(unsigned int *)v5 = 0;
    }
    v11 = (int)v8[v10 * 4];
    v25 = v6;
    if (v11) {
      if (v11 != 5) goto label_199d4;
      goto label_19a4c;
    }
    *(int *)&v8[v10 * 4] = 5;
    v17 = *v9;
    v19 = (unsigned int)v17;
  }
  v16 = v9;
  if (v17 != 0x2e) goto label_19537;
  if (v9[1] != 0x2a) {
    v4[6] = (unsigned long)v9;
    v15 = &v9[1];
    if (10 <= (unsigned char)(v9[1] - 0x30)) { // branch-flip
      v6 = 1;
      v16 = v15;
    }
    else {
      do {
        v16 = &v15[1];
        v15 = &v15[1];
      } while ((unsigned char)(*v16 - 0x30) <= 9);
      v6 = (long)v15 - (long)v9;
      v16 = v15;
    }
    v4[7] = (unsigned long)v15;
    v19 = (unsigned int)*v15;
    if (v6 <= v26)
      v6 = v26;
    v26 = v6;
    goto label_19537;
  }
  v16 = &v9[2];
  v4[6] = (unsigned long)v9;
  v4[7] = (unsigned long)v16;
  v6 = 2;
  if (2 <= v26)
    v6 = v26;
  v17 = v9[2];
  if (10 <= (unsigned char)(v17 - 0x30)) { // branch-flip
label_198a5:
    v10 = v4[8];
    if (v4[8] == 0xffffffffffffffff) {
      v4[8] = v27;
      if (v27 == 0xffffffffffffffff) goto label_199d0;
      v10 = v27;
      v27 += 1;
    }
  }
  else {
    v9 = v16;
    do {
      v15 = &v9[1];
      v9 = &v9[1];
    } while ((unsigned char)(*v15 - 0x30) <= 9);
    if (*v15 != 0x24) goto label_198a5;
    v10 = 0;
    while( true ) {
      v20 = -1;
      if (v10 <= 0x1999999999999999)
        v20 = v10 * 10;
      v10 = sub_17f90(v20,(long)(char)(v17 - 0x30));
      v17 = v16[1];
      v9 = &v16[1];
      if (10 <= (unsigned char)(v17 - 0x30)) break;
      v16 = v9;
    }
    v10 -= 1;
    if (0xfffffffffffffffe <= v10) goto label_199d0;
    v4[8] = v10;
    v16 = &v16[2];
  }
  if (v21 <= v10) { // branch-flip
    v21 *= 2;
    if (v21 <= v10)
      v21 = sub_17f90(v10,1);
    v7 = (unsigned long *)a2[1];
    if (0x800000000000000 <= v21) goto label_19cdd;
    if (v2 != v7) // branch-flip
      v8 = realloc(v7,v21 << 5);
    else {
      v8 = malloc(v21 << 5);
    }
    v7 = (unsigned long *)a2[1];
    if (!v8) goto label_19cdd;
    if (v2 == v7)
      v8 = memcpy(v8,v2,*a2 << 5);
    a2[1] = (unsigned long)v8;
  }
  else {
    v8 = (unsigned long *)a2[1];
  }
  v14 = *a2;
  v7 = &v8[v14 * 4];
  if (v14 <= v10) {
    do {
      v5 = v7;
      v14 += 1;
      *(unsigned int *)v5 = 0;
      v7 = &v5[4];
    } while (v14 <= v10);
    *a2 = v14;
    *(unsigned int *)v5 = 0;
  }
  v11 = (int)v8[v10 * 4];
  v26 = v6;
  if (!v11) {
    *(int *)&v8[v10 * 4] = 5;
    v19 = (unsigned int)*v16;
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
        v19 = (unsigned int)v17;
        v16 = &v16[1];
        v12 = v13;
        if (v17 != 0x68) break;
label_195a0:
        v13 |= 1 << ((unsigned char)v13 & 1);
      }
    } while( true );
  }
  if (v11 == 5) {
    v19 = (unsigned int)*v16;
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
      if ((int)v12 <= 0xf)
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
      if ((((int)v12 <= 0xf) && (!(v12 & 4))) && ((v11 = 8, (int)v12 <= 7 && (v11 = 2, !(v12 & 2)))))
        v11 = (-(unsigned int)((v12 & 1) == 0) & 2) + 4;
      break;
    case 99:
      v11 = (7 < (int)v12) + 0xd;
      break;
    case 100:
    case 0x69:
      v11 = 9;
      if ((((int)v12 <= 0xf) && (!(v12 & 4))) && ((v11 = 7, (int)v12 <= 7 && (v11 = 1, !(v12 & 2)))))
        v11 = (-(unsigned int)((v12 & 1) == 0) & 2) + 3;
      break;
    case 0x6e:
      v11 = 0x16;
      if (((((int)v12 <= 0xf) && (!(v12 & 4))) && (v11 = 0x15, (int)v12 <= 7)) && (v11 = 0x12, !(v12 & 2)))
        v11 = 0x14 - (v12 & 1);
      break;
    case 0x70:
      v11 = 0x11;
      break;
    case 0x73:
      v11 = (7 < (int)v12) + 0xf;
    
  }
  if (v24 != 0xffffffffffffffff) { // branch-flip
    v4[10] = v24;
    v6 = v27;
  }
  else {
    v4[10] = v27;
    v6 = v27 + 1;
    v24 = v27;
    if (v27 == 0xffffffffffffffff) {
label_199d0:
      v8 = (unsigned long *)a2[1];
      goto label_199d4;
    }
  }
  v27 = v6;
  if (v21 <= v24) { // branch-flip
    v21 *= 2;
    if (v21 <= v24)
      v21 = sub_17f90(v24,1);
    v7 = (unsigned long *)a2[1];
    if (0x800000000000000 <= v21) goto label_19cdd;
    if (v2 != v7) // branch-flip
      v8 = realloc(v7,v21 << 5);
    else {
      v8 = malloc(v21 << 5);
    }
    v7 = (unsigned long *)a2[1];
    if (!v8) goto label_19cdd;
    if (v2 == v7)
      v8 = memcpy(v8,v2,*a2 << 5);
    a2[1] = (unsigned long)v8;
  }
  else {
    v8 = (unsigned long *)a2[1];
  }
  v6 = *a2;
  v7 = &v8[v6 * 4];
  if (v6 <= v24) {
    do {
      v5 = v7;
      v6 += 1;
      *(unsigned int *)v5 = 0;
      v7 = &v5[4];
    } while (v6 <= v24);
    *a2 = v6;
    *(unsigned int *)v5 = 0;
  }
  v3 = (int)v8[v24 * 4];
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
    *(int *)&v8[v24 * 4] = v11;
  }
label_19686:
  *(char *)&v4[9] = v18;
  v6 = *a1;
  v4[1] = (unsigned long)v16;
  v6 += 1;
  *a1 = v6;
  if (v6 >= v23) {
    if ((0 <= (long)v23) && (v22 = v23 * 2, v22 <= 0x2e8ba2e8ba2e8ba)) {
      if (v1 != (unsigned long *)a1[1]) // branch-flip
        v4 = realloc((unsigned long *)a1[1],v23 * 0xb0);
      else {
        v4 = malloc(v23 * 0xb0);
      }
      if (v4) {
        if (v1 == (unsigned long *)a1[1])
          v4 = memcpy(v4,v1,*a1 * 0x58);
        a1[1] = (unsigned long)v4;
        v6 = *a1;
        goto label_19441;
      }
    }
    v7 = (unsigned long *)a2[1];
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
  v23 = v22;
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
