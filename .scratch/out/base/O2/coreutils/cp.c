// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_22fd8)
    return;
  (*dat_22fd8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_22bb8)(); // jump-as-call
}

// Function: free @ 0x3830
void free(void *a0)
{
  (*dat_22fc0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3840
void __cxa_finalize(void)
{
  (*dat_22fe8)(); // jump-as-call
}

// Function: context_type_get @ 0x3850
void context_type_get(void)
{
  (*dat_22bc0)(); // jump-as-call
}

// Function: getenv @ 0x3860
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22bc8)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x3870
void fgetfilecon(void)
{
  (*dat_22bd0)(); // jump-as-call
}

// Function: __snprintf_chk @ 0x3880
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_22bd8)(); // jump-as-call
  return v1;
}

// Function: mkfifoat @ 0x3890
void mkfifoat(void)
{
  (*dat_22be0)(); // jump-as-call
}

// Function: utimensat @ 0x38a0
void utimensat(void)
{
  (*dat_22be8)(); // jump-as-call
}

// Function: abort @ 0x38b0
void abort(void)
{
  (*dat_22bf0)(); // jump-as-call
}

// Function: __errno_location @ 0x38c0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_22bf8)(); // jump-as-call
  return v1;
}

// Function: error_at_line @ 0x38d0
void error_at_line(void)
{
  (*dat_22c00)(); // jump-as-call
}

// Function: strncmp @ 0x38e0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_22c08)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x38f0
void _exit(int a0)
{
  (*dat_22c10)(); // jump-as-call
}

// Function: __fpending @ 0x3900
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22c18)(); // jump-as-call
  return v1;
}

// Function: getfscreatecon @ 0x3910
void getfscreatecon(void)
{
  (*dat_22c20)(); // jump-as-call
}

// Function: mkdir @ 0x3920
int mkdir(char *a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_22c28)(); // jump-as-call
  return v1;
}

// Function: unlinkat @ 0x3930
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_22c30)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3940
void qsort(void)
{
  (*dat_22c38)(); // jump-as-call
}

// Function: reallocarray @ 0x3950
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22c40)(); // jump-as-call
  return v1;
}

// Function: mode_to_security_class @ 0x3960
void mode_to_security_class(void)
{
  (*dat_22c48)(); // jump-as-call
}

// Function: faccessat @ 0x3970
void faccessat(void)
{
  (*dat_22c50)(); // jump-as-call
}

// Function: readlink @ 0x3980
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_22c58)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3990
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22c60)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x39a0
int clock_gettime(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22c68)(); // jump-as-call
  return v1;
}

// Function: write @ 0x39b0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_22c70)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x39c0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22c78)(); // jump-as-call
  return v1;
}

// Function: pathconf @ 0x39d0
void pathconf(void)
{
  (*dat_22c80)(); // jump-as-call
}

// Function: fclose @ 0x39e0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22c88)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x39f0
void * opendir(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22c90)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3a00
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22c98)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x3a10
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22ca0)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x3a20
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22ca8)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x3a30
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22cb0)(); // jump-as-call
  return v1;
}

// Function: selabel_lookup @ 0x3a40
void selabel_lookup(void)
{
  (*dat_22cb8)(); // jump-as-call
}

// Function: strlen @ 0x3a50
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22cc0)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x3a60
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_22cc8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3a70
void __stack_chk_fail(void)
{
  (*dat_22cd0)(); // jump-as-call
}

// Function: getopt_long @ 0x3a80
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_22cd8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3a90
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22ce0)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x3aa0
void freecon(void)
{
  (*dat_22ce8)(); // jump-as-call
}

// Function: strchr @ 0x3ab0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22cf0)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x3ac0
int __overflow(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_22cf8)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3ad0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22d00)(); // jump-as-call
  return v1;
}

// Function: ftruncate @ 0x3ae0
void ftruncate(void)
{
  (*dat_22d08)(); // jump-as-call
}

// Function: lseek @ 0x3af0
void lseek(void)
{
  (*dat_22d10)(); // jump-as-call
}

// Function: __assert_fail @ 0x3b00
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_22d18)(); // jump-as-call
}

// Function: fputs @ 0x3b10
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22d20)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x3b20
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22d28)(); // jump-as-call
  return v1;
}

// Function: geteuid @ 0x3b30
unsigned int geteuid(void)
{
  unsigned int v1; // eax
  
  v1 = (*dat_22d30)(); // jump-as-call
  return v1;
}

// Function: ioctl @ 0x3b40
int ioctl(int a0,unsigned long a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22d38)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x3b50
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22d40)(); // jump-as-call
  return v1;
}

// Function: copy_file_range @ 0x3b60
void copy_file_range(void)
{
  (*dat_22d48)(); // jump-as-call
}

// Function: canonicalize_file_name @ 0x3b70
char * canonicalize_file_name(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22d50)(); // jump-as-call
  return v1;
}

// Function: close @ 0x3b80
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22d58)(); // jump-as-call
  return v1;
}

// Function: rewinddir @ 0x3b90
void rewinddir(void)
{
  (*dat_22d60)(); // jump-as-call
}

// Function: strspn @ 0x3ba0
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22d68)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x3bb0
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22d70)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x3bc0
void posix_fadvise(void)
{
  (*dat_22d78)(); // jump-as-call
}

// Function: selabel_open @ 0x3bd0
void selabel_open(void)
{
  (*dat_22d80)(); // jump-as-call
}

// Function: getcon @ 0x3be0
void getcon(void)
{
  (*dat_22d88)(); // jump-as-call
}

// Function: lsetfilecon @ 0x3bf0
void lsetfilecon(void)
{
  (*dat_22d90)(); // jump-as-call
}

// Function: read @ 0x3c00
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_22d98)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x3c10
int lstat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22da0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3c20
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_22da8)(); // jump-as-call
  return v1;
}

// Function: fallocate @ 0x3c30
void fallocate(void)
{
  (*dat_22db0)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x3c40
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22db8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x3c50
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22dc0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3c60
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22dc8)(); // jump-as-call
  return v1;
}

// Function: __getdelim @ 0x3c70
void __getdelim(void)
{
  (*dat_22dd0)(); // jump-as-call
}

// Function: strcmp @ 0x3c80
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_22dd8)(); // jump-as-call
  return v1;
}

// Function: readlinkat @ 0x3c90
void readlinkat(void)
{
  (*dat_22de0)(); // jump-as-call
}

// Function: dirfd @ 0x3ca0
int dirfd(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22de8)(); // jump-as-call
  return v1;
}

// Function: fpathconf @ 0x3cb0
void fpathconf(void)
{
  (*dat_22df0)(); // jump-as-call
}

// Function: mknodat @ 0x3cc0
void mknodat(void)
{
  (*dat_22df8)(); // jump-as-call
}

// Function: rpmatch @ 0x3cd0
void rpmatch(void)
{
  (*dat_22e00)(); // jump-as-call
}

// Function: mkdirat @ 0x3ce0
void mkdirat(void)
{
  (*dat_22e08)(); // jump-as-call
}

// Function: umask @ 0x3cf0
unsigned int umask(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_22e10)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3d00
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22e18)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3d10
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22e20)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x3d20
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22e28)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x3d30
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22e30)(); // jump-as-call
  return v1;
}

// Function: context_new @ 0x3d40
void context_new(void)
{
  (*dat_22e38)(); // jump-as-call
}

// Function: malloc @ 0x3d50
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22e40)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3d60
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22e48)(); // jump-as-call
  return v1;
}

// Function: fchmodat @ 0x3d70
void fchmodat(void)
{
  (*dat_22e50)(); // jump-as-call
}

// Function: nl_langinfo @ 0x3d80
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22e58)(); // jump-as-call
  return v1;
}

// Function: context_str @ 0x3d90
void context_str(void)
{
  (*dat_22e60)(); // jump-as-call
}

// Function: is_selinux_enabled @ 0x3da0
void is_selinux_enabled(void)
{
  (*dat_22e68)(); // jump-as-call
}

// Function: renameat2 @ 0x3db0
void renameat2(void)
{
  (*dat_22e70)(); // jump-as-call
}

// Function: attr_copy_fd @ 0x3dc0
void attr_copy_fd(void)
{
  (*dat_22e78)(); // jump-as-call
}

// Function: __freading @ 0x3dd0
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22e80)(); // jump-as-call
  return v1;
}

// Function: fchdir @ 0x3de0
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22e88)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3df0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22e90)(); // jump-as-call
  return v1;
}

// Function: linkat @ 0x3e00
void linkat(void)
{
  (*dat_22e98)(); // jump-as-call
}

// Function: setlocale @ 0x3e10
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22ea0)(); // jump-as-call
  return v1;
}

// Function: context_type_set @ 0x3e20
void context_type_set(void)
{
  (*dat_22ea8)(); // jump-as-call
}

// Function: __printf_chk @ 0x3e30
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22eb0)(); // jump-as-call
  return v1;
}

// Function: fchmod @ 0x3e40
int fchmod(int a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_22eb8)(); // jump-as-call
  return v1;
}

// Function: chmod @ 0x3e50
void chmod(void)
{
  (*dat_22ec0)(); // jump-as-call
}

// Function: mempcpy @ 0x3e60
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22ec8)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3e70
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22ed0)(); // jump-as-call
  return v1;
}

// Function: attr_copy_file @ 0x3e80
void attr_copy_file(void)
{
  (*dat_22ed8)(); // jump-as-call
}

// Function: error @ 0x3e90
void error(int a0,int a1,char *a2,...)
{
  (*dat_22ee0)(); // jump-as-call
}

// Function: attr_copy_check_permissions @ 0x3ea0
void attr_copy_check_permissions(void)
{
  (*dat_22ee8)(); // jump-as-call
}

// Function: open @ 0x3eb0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22ef0)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x3ec0
void fseeko(void)
{
  (*dat_22ef8)(); // jump-as-call
}

// Function: security_compute_create @ 0x3ed0
void security_compute_create(void)
{
  (*dat_22f00)(); // jump-as-call
}

// Function: fchown @ 0x3ee0
int fchown(int a0,unsigned int a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = (*dat_22f08)(); // jump-as-call
  return v1;
}

// Function: fdopendir @ 0x3ef0
void * fdopendir(int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22f10)(); // jump-as-call
  return v1;
}

// Function: futimens @ 0x3f00
void futimens(void)
{
  (*dat_22f18)(); // jump-as-call
}

// Function: fstatfs @ 0x3f10
void fstatfs(void)
{
  (*dat_22f20)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x3f20
void __cxa_atexit(void)
{
  (*dat_22f28)(); // jump-as-call
}

// Function: fchownat @ 0x3f30
void fchownat(void)
{
  (*dat_22f30)(); // jump-as-call
}

// Function: renameat @ 0x3f40
void renameat(void)
{
  (*dat_22f38)(); // jump-as-call
}

// Function: getpagesize @ 0x3f50
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_22f40)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3f60
void exit(int a0)
{
  (*dat_22f48)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x3f70
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_22f50)(); // jump-as-call
  return v1;
}

// Function: fsetfilecon @ 0x3f80
void fsetfilecon(void)
{
  (*dat_22f58)(); // jump-as-call
}

// Function: getrandom @ 0x3f90
void getrandom(void)
{
  (*dat_22f60)(); // jump-as-call
}

// Function: getfilecon @ 0x3fa0
void getfilecon(void)
{
  (*dat_22f68)(); // jump-as-call
}

// Function: aligned_alloc @ 0x3fb0
void aligned_alloc(void)
{
  (*dat_22f70)(); // jump-as-call
}

// Function: context_free @ 0x3fc0
void context_free(void)
{
  (*dat_22f78)(); // jump-as-call
}

// Function: mbsinit @ 0x3fd0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22f80)(); // jump-as-call
  return v1;
}

// Function: setfscreatecon @ 0x3fe0
void setfscreatecon(void)
{
  (*dat_22f88)(); // jump-as-call
}

// Function: lgetfilecon @ 0x3ff0
void lgetfilecon(void)
{
  (*dat_22f90)(); // jump-as-call
}

// Function: symlinkat @ 0x4000
void symlinkat(void)
{
  (*dat_22f98)(); // jump-as-call
}

// Function: iswprint @ 0x4010
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_22fa0)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x4020
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22fa8)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x4030
int fstatat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_22fb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4040
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22fb8)(); // jump-as-call
  return v1;
}

// Function: sub_4050 @ 0x4050
unsigned long sub_4050(int a0,char **a1)
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
  unsigned long v20; // stack - 0x88
  unsigned short v21; // stack - 0x80
  unsigned int v22; // stack - 0x7c
  char v23; // stack - 0x78
  long v24; // stack - 0x70
  char v25; // stack - 0x68
  char v26; // stack - 0x67
  char v27; // stack - 0x66
  char v28; // stack - 0x65
  char v29; // stack - 0x64
  char *v3; // rax
  char v30; // stack - 0x63
  char v31; // stack - 0x62
  char v32; // stack - 0x61
  char v33; // stack - 0x60
  char v34; // stack - 0x5f
  char v35; // stack - 0x5e
  char v36; // stack - 0x5d
  char v37; // stack - 0x5c
  char v38; // stack - 0x5b
  char v39; // stack - 0x5a
  unsigned long v4; // rax
  int v40; // stack - 0x54
  unsigned long v41; // stack - 0x50
  unsigned long v42; // stack - 0x48
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  long v9;
  
  sub_12680(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_19d70(sub_e2b0);
  dat_2315a = 0 < (int)is_selinux_enabled();
  sub_7670(&v10);
  v23 = 0;
  v40 = 1;
  v18 = 4;
  v19 = 2;
  v22 = 0;
  v17 = 1;
  v20 = 0x100000000;
  v21 = 0;
  v24 = 0;
  v32 = 0;
  v31 = 0;
  v30 = 0;
  v29 = '\0';
  v28 = '\0';
  v27 = 0;
  v26 = 1;
  v25 = 0;
  v36 = 0;
  v35 = '\0';
  v34 = 0;
  v33 = '\0';
  v37 = 0;
  v38 = 0;
  v15 = 0;
  v41 = 0;
  v39 = getenv("POSIXLY_CORRECT") != NULL;
  v42 = 0;
  v14 = 0;
  v11 = 0;
  v16 = 0;
  v12 = 0;
  v13 = 0;
  v1 = 0;
label_41b8:
  v2 = getopt_long(a0,a1,"abdfHilLnprst:uvxPRS:TZ",(void *)0x224a0,NULL);
  if (v2 != -1) {
    if (0x88 < v2) {
      sub_54f0(1); // return-dupe, no-return
    }
    if (v2 <= 0x47) {
      if (v2 == -0x83) {
        sub_16a20(stdout,"cp","GNU coreutils",dat_23020,"Torbjorn Granlund","David MacKenzie","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_54f0(0); // no-return
      sub_54f0(1);
    }
    switch(v2) {
      case 0x48:
        v17 = 3;
        break;
      default:
        goto label_4799;
      case 0x4c:
        v17 = 4;
        break;
      case 0x50:
        v17 = 2;
        break;
      case 0x52:
      case 0x72:
        v33 = '\x01';
        break;
      case 0x53:
        v13 = optarg;
        v1 = 1;
        break;
      case 0x54:
        v15 = 1;
        break;
      case 0x5a:
        if (dat_2315a) { // branch-flip
          if (optarg) // branch-flip
            v14 = optarg;
          else {
            v24 = selabel_open(0,0,0);
            if (!v24) {
              v3 = dcgettext(NULL,"warning: ignoring --context",5);
              error(0,*__errno_location(),v3);
            }
          }
        }
        else if (optarg)
          error(0,0,dcgettext(NULL,"warning: ignoring --context; it requires an SELinux-enabled kernel",5));
        break;
      case 0x61:
        v25 = 1;
        v17 = 2;
        v22._0_2_ = CONCAT11(1,(char)v22);
        v22 = CONCAT22(0x101,(unsigned short)v22);
        v27 = 1;
        if (dat_2315a)
          v28 = '\x01';
        v30 = 1;
        v33 = '\x01';
        v32 = 1;
        break;
      case 0x62:
        v1 = 1;
        v9 = optarg;
        if (!optarg)
          v9 = v12;
        v12 = v9;
        break;
      case 100:
        v25 = 1;
        v17 = 2;
        break;
      case 0x66:
        v20._0_7_ = CONCAT16(1,(undefined6)v20);
        break;
      case 0x69:
        v18 = 3;
        break;
      case 0x6c:
        v20 = CONCAT17(1,(undefined7)v20);
        break;
      case 0x6e:
        v18 = 2;
        break;
      case 0x70:
label_4210:
        v22._0_2_ = CONCAT11(1,(char)v22);
        v22 = CONCAT22(0x101,(unsigned short)v22);
        v27 = 1;
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
        v22 = CONCAT31(v22._1_3_,1);
        break;
      case 0x80:
        v26 = 0;
        break;
      case 0x81:
        v16 = 1;
        break;
      case 0x82:
        sub_49a0(optarg,&v10,0);
        break;
      case 0x83:
        dat_23159 = 1;
        break;
      case 0x84:
        if (optarg) {
          sub_49a0(optarg,&v10,1);
          v27 = 1;
          break;
        }
        goto label_4210;
      case 0x85:
        if (optarg) { // branch-flip
          v9 = sub_cfc0("--reflink",optarg,0x22860,0x1a460,4,dat_23028,1);
          v40 = *(int *)(v9 * 4 + 0x1a460);
        }
        else {
          v40 = 2;
        }
        break;
      case 0x86:
        v9 = sub_cfc0("--sparse",optarg,0x22880,0x1a470,4,dat_23028,1);
        v19 = *(int *)(v9 * 4 + 0x1a470);
        break;
      case 0x87:
        dat_23158 = 1;
        break;
      case 0x88:
        goto label_4250;
      
    }
    goto label_41b8;
  }
  if ((v20._7_1_) && (v35)) {
    v3 = "cannot make both hard and symbolic links";
    goto label_4784;
  }
  if (v18 != 2) { // branch-flip
    if (!v1) goto label_46a9;
    if ((v40 == 2) && (v19 != 2)) goto label_4778;
    v3 = dcgettext(NULL,"backup type",5);
    v10 = sub_d8d0(v3,v12);
  }
  else {
    v36 = 0;
    if (v1) {
      v3 = "options --backup and --no-clobber are mutually exclusive";
      goto label_4784;
    }
label_46a9:
    v10 = 0;
    if ((v40 == 2) && (v10 = 0, v19 != 2)) {
label_4778:
      v3 = "--reflink can be used only with --sparse=auto";
label_4784:
      error(0,0,dcgettext(NULL,v3,5));
      sub_54f0(1);
    }
  }
  sub_d0f0(v13);
  if (v17 != 1) { // branch-flip
label_459f:
    if (v33) goto label_4715;
  }
  else {
    if ((!v33) || (v20._7_1_)) {
      v17 = 4;
      goto label_459f;
    }
    v17 = 2;
label_4715:
    v20._0_5_ = CONCAT14(v16,(unsigned int)v20);
  }
  if (v14 || v24) { // branch-flip
    if (v29) { // branch-flip
      if (v28) {
        error(1,0,dcgettext(NULL,"cannot set target context and preserve it",5));
        return v5;
      }
      if (!dat_2315a) {
        error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5)); // return-dupe
        return v4;
      }
    }
    else {
      v28 = '\0';
    }
    if ((v14) && ((int)setfscreatecon(v14) < 0)) {
      v7 = sub_14b30(v14);
      v3 = dcgettext(NULL,"failed to set default file creation context to %s",5);
      error(1,*__errno_location(),v3,v7);
      return v8;
    }
  }
  else if ((v29) && (!dat_2315a)) {
    error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5));
    return v4;
  }
  sub_bdb0();
  v9 = (long)optind;
  a0 -= optind;
  return (unsigned long)((sub_5a50(a0,&a1[v9],v11,v15,&v10) ^ 1) & 0xff);
label_4250:
  v20._0_6_ = CONCAT15(1,(undefined5)v20);
  goto label_41b8;
}

// Function: sub_48b0 @ 0x48b0
void sub_48b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_22fc8)(sub_4050,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_48e0 @ 0x48e0
void sub_48e0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4950
void _FINI_0(void)
{
  if (!dat_230e8) {
    if (dat_22fe8)
      __cxa_finalize(dat_23008);
    sub_48e0();
    dat_230e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4990
void _INIT_0(void)
{
  return;
}

// Function: sub_49a0 @ 0x49a0
void sub_49a0(unsigned long a0,unsigned long a1,char a2)
{
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  long v4; // rax
  
  v1 = (char *)sub_17130(a0);
  v2 = "--no-preserve";
  if (a2)
    v2 = "--preserve";
  v3 = strchr(v1,0x2c);
  if (v3)
    *v3 = '\0';
  v4 = sub_cfc0(v2,v1,0x22460,0x1a440,4,dat_23028);
  if (*(unsigned int *)(v4 * 4 + 0x1a440) <= 6) {
    (*(void *)((long)*(int *)((unsigned long)*(unsigned int *)(v4 * 4 + 0x1a440) * 4 + 0x1a320) + 0x1a320))(); // jump-as-call
    return;
  }
  abort(); // no-return
}

// Function: sub_4b10 @ 0x4b10
unsigned long sub_4b10(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,long a6) // return-dupe x2
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
  v12 = sub_e4a0(a0);
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
  *(unsigned long *)((long)v16 + v4 + -8) = 0x4c05;
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
  *(unsigned long *)&v26[v4 + -8] = 0x4c6f;
  memcpy(v14,v36,v12,v26[v4 + -8]);
  v28 = (char *)((long)v14 + a1);
  *(char *)((long)v14 + v12) = 0;
  v8 = *v28;
  while (v8 == '/') {
    v28 = &v28[1];
    v8 = *v28;
  }
  v33 = v21;
  *(unsigned long *)&v26[v4 + -8] = 0x4ca4;
  v3 = v26[v4 + -8];
  if (!fstatat(a2,v28,v21,0,v3)) {
    if ((v37 & 0xf000) == 0x4000) {
      *v34 = '\0';
      return 1;
    }
label_4b75:
    *(unsigned long *)&v26[v4 + -8] = 0x5136;
    v17 = sub_14440(4,v14);
    *(unsigned long *)&v26[v4 + -8] = 0x514c;
    v28 = dcgettext(NULL,"%s exists but is not a directory",5,v26[v4 + -8]);
    *(unsigned long *)&v26[v4 + -8] = 0x515d;
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
    *(unsigned long *)&v26[v4 + -8] = 0x4cfa;
    v15 = strchr(v15,0x2f,v26[v4 + -8]);
    if (!v15)
      return 1;
    *v15 = '\0';
    *(unsigned long *)&v26[v4 + -8] = 0x4d1d;
    v9 = fstatat(a2,v28,v33,0,v26[v4 + -8]);
    if ((v9) || (*(unsigned int *)(v7 + 0x1c) & 0xffffff00)) {
      *(unsigned long *)&v26[v4 + -8] = 0x4df3;
      v10 = stat(v35,v20,v26[v4 + -8]);
      if (v10) { // branch-flip
        *(unsigned long *)&v26[v4 + -8] = 0x4e00;
        v3 = v26[v4 + -8];
        v10 = *__errno_location(v3);
        if (v10) {
label_4b75:
          v36 = (void *)CONCAT44(v36._4_4_,v10);
          *(unsigned long *)&v26[v4 + -8] = 0x5054;
          v17 = sub_14440(4,v35);
          *(unsigned long *)&v26[v4 + -8] = 0x506a;
          v28 = dcgettext(NULL,"failed to get attributes of %s",5,v26[v4 + -8]);
          *(unsigned long *)&v26[v4 + -8] = 0x507f;
          error(0,(int)v36,v28,v17);
          return 0;
        }
      }
      else if ((v40 & 0xf000) != 0x4000) {
        v10 = 0x14;
        goto label_4b75;
      }
      *(unsigned long *)&v26[v4 + -8] = 0x4e16;
      v16 = (unsigned long *)sub_16c50(0xa8);
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
      if (!v9) goto label_4d37;
      v1 = *(unsigned int *)&v16[3];
      v31 = v16;
      *(unsigned long *)&v26[v4 + -8] = 0x4ed7;
      v8 = sub_72a0(v35,v36,v1,1,v7);
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
      *(unsigned long *)&v26[v4 + -8] = 0x4f34;
      if (mkdirat(a2,v28,v11 & v18 & 0xfff)) {
        *(unsigned long *)&v26[v4 + -8] = 0x51d4;
        v17 = sub_14440(4,v36);
        v28 = "cannot make directory %s";
        goto label_4b75;
      }
      if (v30) {
        *(unsigned long *)&v26[v4 + -8] = 0x4f70;
        __printf_chk(1,v30,v35,v36);
      }
      *(unsigned long *)&v26[v4 + -8] = 0x4f95;
      v9 = fstatat(a2,v28,v33,0x100,v26[v4 + -8]);
      if (v9) {
        *(unsigned long *)&v26[v4 + -8] = 0x5181;
        v17 = sub_14440(4,v36);
        v28 = "failed to get attributes of %s";
        goto label_4b75;
      }
      v11 = v37;
      if (!*(char *)(v7 + 0x1e)) {
        if (~v37 & v29) { // branch-flip
          *(unsigned long *)&v26[v4 + -8] = 0x50e1;
          v29 &= ~sub_bbd0();
          v16 = v31;
          v11 = v37;
          if (!(~v37 & v29)) goto label_4fbf;
        }
        else {
label_4fbf:
          v16 = v31;
          v11 = v37;
          if ((v37 & 0x1c0) == 0x1c0) goto label_4fde;
        }
        *(char *)&v16[0x12] = 1;
        *(unsigned int *)&v16[3] = v29 | v11;
      }
label_4fde:
      if ((v11 | 0x1c0) != v11) {
        *(unsigned long *)&v26[v4 + -8] = 0x4ffa;
        if (fchmodat(a2,v28,v11 | 0x1c0,0x100)) {
          *(unsigned long *)&v26[v4 + -8] = 0x51f6;
          v17 = sub_14440(4,v36);
          v28 = "setting permissions for %s";
label_4b75:
          *(unsigned long *)&v26[v4 + -8] = 0x5197;
          v28 = dcgettext(NULL,v28,5,v26[v4 + -8]);
          *(unsigned long *)&v26[v4 + -8] = 0x519f;
          v3 = v26[v4 + -8];
          v9 = *__errno_location(v3);
          *(unsigned long *)&v26[v4 + -8] = 0x51b0;
          error(0,v9,v28,v17);
          return 0;
        }
      }
      if (!*v34) goto label_4d79;
    }
    else {
label_4d37:
      *(unsigned long *)&v26[v4 + -8] = 0x4d51;
      v8 = sub_72a0(v35,v36,0,0,v7);
      if (!v8)
        return 0;
      if ((v37 & 0xf000) != 0x4000) {
        v14 = v36;
        goto label_4b75;
      }
      *v34 = '\0';
label_4d79:
      if ((*(long *)(v7 + 0x28)) || (*(char *)(v7 + 0x33))) {
        *(unsigned long *)&v26[v4 + -8] = 0x4d96;
        v8 = sub_7510(v36,0,v7);
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

// Function: sub_5210 @ 0x5210
unsigned long sub_5210(char *a0,unsigned int a1,unsigned long a2,long a3,long a4) // return-dupe
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
  unsigned long v19; // stack - 0x70
  char v2;
  unsigned long v20; // stack - 0x60
  unsigned long v21; // stack - 0x58
  unsigned long v22; // stack - 0x50
  unsigned int v3;
  unsigned int v4;
  char v5;
  long v6;
  unsigned long *v7;
  int v8;
  unsigned long v9; // rax
  
  v16 = &v14;
  v19 = a2;
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
  *(unsigned long *)((long)v16 + v6 + -8) = 0x52a0;
  v10 = memcpy((void *)((unsigned long)((long)v16 + v6 + 0xf) & 0xfffffffffffffff0),a0,v9 + 1,*(char *)((long)v16 + v6 + -8));
  v1 = (long)v10 + (v19 - (long)a0);
  if (!a3)
    return 1;
  v14 = &v15;
  do {
    v2 = *(char *)(a4 + 0x1f);
    *(char *)((long)v10 + *(long *)(a3 + 0x98)) = 0;
    if (v2) {
      v15 = *(unsigned long *)(a3 + 0x48);
      v20 = *(unsigned long *)(a3 + 0x50);
      v21 = *(unsigned long *)(a3 + 0x58);
      v22 = *(unsigned long *)(a3 + 0x60);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x534e;
      v8 = utimensat(a1,v1,v14,0);
      if (v8) {
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5363;
        v11 = sub_14440(4,v10);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5379;
        v12 = dcgettext(NULL,"failed to preserve times for %s",5,*(char *)((long)v16 + v6 + -8));
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5381;
        v5 = *(char *)((long)v16 + v6 + -8);
        v8 = *__errno_location(v5);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5392;
        error(0,v8,v12,v11);
        return 0;
      }
    }
    if (*(char *)(a4 + 0x1d)) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x20);
      v3 = *(unsigned int *)(a3 + 0x1c);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53b7;
      if (!fchownat(a1,v1,v3,v4,0x100)) goto label_52d3;
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53c7;
      if (!sub_76c0(a4)) {
        v19 &= 0xffffffffffffff00;
        *(unsigned long *)((long)v16 + v6 + -8) = 0x54da;
        v11 = sub_14440(4,v10);
        v12 = "failed to preserve ownership for %s";
        goto label_546b;
      }
      v4 = *(unsigned int *)(a3 + 0x20);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53e8;
      fchownat(a1,v1,0xffffffff,v4,0x100);
      v2 = *(char *)(a4 + 0x1e);
    }
    else {
label_52d3:
      v2 = *(char *)(a4 + 0x1e);
    }
    if (v2) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x18);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5419;
      if (sub_c830(v1,0xffffffff,v10,0xffffffff,v4))
        return 0;
    }
    else if (*(char *)(a3 + 0x90)) {
      v4 = *(unsigned int *)(a3 + 0x18);
      v19 &= 0xffffffffffffff00;
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5447;
      if (fchmodat(a1,v1,v4,0x100)) {
        *(unsigned long *)((long)v16 + v6 + -8) = 0x545c;
        v11 = sub_14440(4,v10);
        v12 = "failed to preserve permissions for %s";
label_546b:
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5472;
        v12 = dcgettext(NULL,v12,5,*(char *)((long)v16 + v6 + -8));
        *(unsigned long *)((long)v16 + v6 + -8) = 0x547a;
        v5 = *(char *)((long)v16 + v6 + -8);
        v8 = *__errno_location(v5);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x548b;
        error(0,v8,v12,v11);
        return v19 & 0xff;
      }
    }
    *(char *)((long)v10 + *(long *)(a3 + 0x98)) = 0x2f;
    a3 = *(long *)(a3 + 0xa0);
    if (!a3)
      return 1;
  } while( true );
}

// Function: sub_54f0 @ 0x54f0
void sub_54f0(int a0)
{
  void *v1;
  char *v10; // stack - 0xb0
  char *v11; // stack - 0x88
  char *v12; // stack - 0x80
  char *v13; // stack - 0x78
  char *v14; // stack - 0x70
  char *v15; // stack - 0x68
  char *v16; // stack - 0x60
  unsigned long v17; // stack - 0x58
  unsigned long v18; // stack - 0x50
  unsigned long v19; // stack - 0x40
  unsigned long v2;
  char *v20 [4]; // stack - 0xa8
  int v3; // eax
  char *v4; // rax
  char **v5; // rbx
  char *v6; // stack - 0xb8
  char *v7;
  char *v8; // r13
  long v9; // fs_offset
  
  v2 = dat_235a0;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",5),v2,v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
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
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe backup suffix is \'~\', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n",5),v1);
    v6 = "[";
    v7 = "[";
    v10 = "test invocation";
    v20[0] = "coreutils";
    v20[1] = "Multi-call invocation";
    v11 = "sha256sum";
    v20[2] = "sha224sum";
    v13 = "sha384sum";
    v20[3] = "sha2 utilities";
    v12 = "sha2 utilities";
    v14 = "sha2 utilities";
    v15 = "sha512sum";
    v16 = "sha2 utilities";
    v17 = 0;
    v18 = 0;
    do {
      if (!strcmp("cp",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "cp";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "cp";
    if (!strcmp("cp","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "cp")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}

// Function: sub_5a50 @ 0x5a50
unsigned long sub_5a50(int a0,unsigned long *a1,long a2,char a3,int *a4)
{
  int *v1;
  char v10; // al
  char v11;
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
  char v25 [24];
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
  char v4;
  long v40; // stack - 0x110
  int *v41; // stack - 0x108
  int v42; // stack - 0x100
  int v43; // stack - 0xfc
  unsigned int v44; // stack - 0xc0
  long v5;
  char *v6;
  void *v7;
  int v8;
  unsigned char v9;
  
  v32 = &v24;
  v28 = 0;
  v40 = a2; // branch-flip
  v41 = a4;
  v43 = a0;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (a0 == 1) {
      v18 = sub_14440(4,*a1);
      v19 = "missing destination file operand after %s";
label_6026:
      error(0,0,dcgettext(NULL,v19,5),v18);
      sub_54f0(1); // return-dupe, no-return
    }
    v19 = "missing file operand";
  }
  else {
    v44 = 0;
    if (a3) { // branch-flip
      if (a2) {
        error(1,0,dcgettext(NULL,"cannot combine --target-directory (-t) and --no-target-directory (-T)",5));
        return v17;
      }
      if (2 < a0) {
        v18 = sub_14440(4,a1[2]);
        v19 = "extra operand %s";
        goto label_6026;
      }
    }
    else {
      if (a2) {
        v42 = sub_156a0(a2,v25);
        if (v42 == -1) {
          v18 = sub_14440(4,v40);
          v19 = dcgettext(NULL,"target directory %s",5);
          error(1,*__errno_location(),v19,v18);
          return v20;
        }
        if (v43 != 1) {
label_5eef:
          v34 = v41;
          sub_75f0(v41);
          sub_7630(v34);
        }
label_5aeb:
        v23 = 0;
        v37 = 1;
        v38 = &v26;
        v39 = &v27;
        v24 = &v28;
        do {
          v19 = (char *)a1[v23];
          v26 = NULL;
          if (dat_23158) { // branch-flip
            *(unsigned long *)((long)v32 + -8) = 0x5d70;
            sub_e5a0(v19);
            if (!dat_23159) goto label_5b4d;
label_5d80:
            *(unsigned long *)((long)v32 + -8) = 0x5d85;
            v14 = strlen(v19,*(char *)((long)v32 + -8));
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v6 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v6[-0x1000];
              v33 = &v6[-0x1000];
              *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
              v32 = (char *)&v6[-0x1000];
              v6 = v33;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v5 = -v22;
            v32 = (char *)&v30[v5];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v5 + -8] = 0x5de2;
            v15 = memcpy((void *)((unsigned long)&v30[v5 + 0xf] & 0xfffffffffffffff0),v19,v14 + 1,v30[v5 + -8]);
            *(unsigned long *)&v30[v5 + -8] = 0x5ded;
            sub_e5a0(v15);
            *(unsigned long *)&v30[v5 + -8] = 0x5e03;
            v36 = v40;
            v15 = (void *)sub_ea10(v40,v15,v38);
            v13 = NULL;
            v8 = v41[0xf];
            *(long *)&v30[v5 + -8] = v36;
            if ((char)v8)
              v13 = "%s -> %s\n";
            *(int **)&v30[v5 + -0x10] = v41;
            *(unsigned long *)&v30[v5 + -0x18] = 0x5e46;
            v10 = sub_4b10(v15,(long)v26 - (long)v15,v42,v13,&v29,v24);
            v11 = *v26;
            v13 = v26;
            while (v11 == '/') {
              v13 = &v13[1];
              v26 = v13;
              v11 = *v13;
            }
            if (v10) goto label_5bf4;
            v37 = 0;
            v32 = (unsigned char **)&v30[v5];
            v11 = dat_23159;
label_5e83:
            v7 = v29;
            if (v11) {
              while (v7) {
                v3 = *(void **)((long)v7 + 0xa0);
                v29 = v3;
                *(unsigned long *)((long)v32 + -8) = 0x5eb6;
                free(v7,*(char *)((long)v32 + -8));
                v7 = v3;
              }
            }
          }
          else {
            if (dat_23159) goto label_5d80;
label_5b4d:
            *(unsigned long *)((long)v32 + -8) = 0x5b52;
            v13 = (char *)sub_d970(v19);
            *(unsigned long *)((long)v32 + -8) = 0x5b5d;
            v14 = strlen(v13,*(char *)((long)v32 + -8));
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v6 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v6[-0x1000];
              v31 = &v6[-0x1000];
              *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
              v32 = (char *)&v6[-0x1000];
              v6 = v31;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v5 = -v22;
            v32 = (char *)&v30[v5];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v5 + -8] = 0x5bba;
            v13 = memcpy((void *)((unsigned long)&v30[v5 + 0xf] & 0xfffffffffffffff0),v13,v14 + 1,v30[v5 + -8]);
            *(unsigned long *)&v30[v5 + -8] = 0x5bc5;
            sub_e5a0(v13);
            *(unsigned long *)&v30[v5 + -8] = 0x5bd4;
            v4 = v30[v5 + -8];
            *(unsigned long *)&v30[v5 + -8] = 0x5bf1;
            v15 = (void *)sub_ea10(v40,&v13[strcmp(v13,"..",v4) == 0],v38);
label_5bf4:
            *(unsigned long *)((long)v32 + -8) = 0;
            *(char **)((long)v32 + -0x10) = v39;
            *(unsigned long *)((long)v32 + -0x18) = 0x5c23;
            v12 = sub_ba80(v19,v15,v42,v26,v28,v41);
            v37 &= v12;
            if (dat_23159) {
              *(unsigned long *)((long)v32 + -8) = 0x5ee3;
              v12 = sub_5210(v15,v42,v26,v29,v41);
              v37 &= v12;
              v11 = dat_23159;
              goto label_5e83;
            }
          }
          v23 += 1;
          *(unsigned long *)((long)v32 + -8) = 0x5c41;
          free(v15,*(char *)((long)v32 + -8));
        } while ((int)v23 < v43);
        return (unsigned long)v37; // return-dupe
      }
      v23 = a1[(long)a0 + -1];
      v40 = v23;
      v42 = sub_156a0(v23,v25);
      if (v42 != -1) { // branch-flip
        v43 = a0 + -1;
        if (v23) {
          if (1 < v43) goto label_5eef;
          v37 = 1;
          if (v43 != 1) {
            return (unsigned long)v37;
          }
          goto label_5aeb;
        }
      }
      else {
        v8 = *__errno_location();
        if (v8 == 2)
          v28 = 1;
        if (3 <= v43) {
          v18 = sub_14440(4,v40);
          error(1,v8,dcgettext(NULL,"target %s",5),v18);
          return v16;
        }
      }
    }
    v9 = v28;
    v19 = (char *)*a1;
    v13 = (char *)a1[1];
    if (!dat_23159) {
      if (((((*(char *)((long)v41 + 0x16)) && (*v41)) && (!strcmp(v19,v13))) && ((!v9 && ((v44 || (!stat(v13,v25))))))) && (v34 = v41, (v44 & 0xf000) == 0x8000)) {
        v13 = (char *)sub_d850(0xffffff9c,v13,*v41);
        v23 = 0x16;
        v35 = (int *)0x23100;
        while (v23) {
          v2 = &v35[1];
          v1 = &v34[1];
          *v35 = *v34;
          v23 -= 1;
          v34 = v1;
          v35 = v2;
        }
        dat_23100 = 0;
        v41 = (int *)0x23100;
      }
      v37 = sub_ba80(v19,v13,0xffffff9c,v13,-(unsigned int)v9,v41,&v26,0);
      return (unsigned long)v37;
    }
    v19 = "with --parents, the destination must be a directory";
  }
  error(0,0,dcgettext(NULL,v19,5));
  sub_54f0(1);
}

// Function: sub_60f0 @ 0x60f0
void sub_60f0(void)
{
  return;
}

// Function: sub_6100 @ 0x6100
void sub_6100(void)
{
  sub_14440(4); // tail-call
}

// Function: sub_6110 @ 0x6110
void sub_6110(void *a0,long a1,char *a2)
{
  unsigned long v1;
  void *v2; // rax
  unsigned long v3; // rsi
  
  v3 = a1 - (long)a0;
  v1 = strlen(a2) + 1;
  v2 = (void *)sub_16c70(v1 + v3);
  memcpy((void *)((long)v2 + v3),a2,v1);
  memcpy(v2,a0,v3); // tail-call
}

// Function: sub_6170 @ 0x6170
unsigned long sub_6170(unsigned int a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if ((!dat_23560) && (dat_23560 = calloc(dat_23018,1), !dat_23560)) {
    dat_23018 = 0x400;
    dat_23560 = (void *)0x23160;
  }
  if (!a1)
    return 1;
  while( true ) {
    v2 = dat_23018;
    if (a1 <= dat_23018)
      v2 = a1;
    v1 = sub_111c0(a0,dat_23560,v2);
    if (v1 != v2) break;
    a1 -= v1;
    if (!a1)
      return 1;
  }
  return 0;
}

// Function: sub_6220 @ 0x6220
unsigned long sub_6220(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,long a4)
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
      v6 = sub_6720;
      v7 = NULL;
      if (!v1) {
        v6 = NULL;
        v7 = NULL;
        if (*(long *)(a4 + 0x28)) {
          v6 = sub_6720;
          v7 = NULL;
        }
      }
      goto label_6282;
    }
    v6 = sub_6720;
    v5 = sub_6360;
    if ((!v1) && (!*(long *)(a4 + 0x28)))
      v6 = NULL;
  }
  else {
    v6 = sub_6720;
    v5 = sub_6440;
    if (!v1) {
      v6 = sub_6720;
      if (!*(long *)(a4 + 0x28))
        v6 = NULL;
      v5 = sub_6440;
    }
  }
  v9 = sub_6100;
  v10 = sub_60f0;
label_6282:
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

// Function: sub_6360 @ 0x6360
void sub_6360(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  int v1;
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
  char v2; // al
  unsigned int v3; // stack - 0xd8
  char v4 [16];
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v2) {
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
  v1 = *__errno_location();
  if ((v1 != 0x5f) && (v1 != 0x3d)) {
    v6 = &Stack0000000000000008;
    v7 = v4;
    v3 = 0x10;
    v5 = 0x30;
    sub_163b0(0,v1,a9,&v3);
  }
}

// Function: sub_6440 @ 0x6440
void sub_6440(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  sub_163b0(0,*__errno_location(),a9,&v2);
}

// Function: sub_6510 @ 0x6510
void sub_6510(unsigned long a0,unsigned long a1,long a2)
{
  char *v1;
  unsigned long v2; // rax
  
  v2 = sub_14310(1,4,a1);
  __printf_chk(1,"%s -> %s",sub_14310(0,4,a0),v2);
  if (a2) {
    v2 = sub_14440(4,a2);
    __printf_chk(1,dcgettext(NULL," (backup: %s)",5),v2);
  }
  v1 = *(char **)((long)stdout + 0x28);
  if (v1 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}

// Function: sub_65d0 @ 0x65d0
unsigned int sub_65d0(void *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned int a4,unsigned long a5,char a6,unsigned int a7,unsigned char a8)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned char v5; // r12b
  unsigned int v6;
  unsigned long v7; // stack - 0x50
  
  v6 = a7;
  v1 = sub_be90(a1,a2,a4,a5,(unsigned long)a8 << 10,a6);
  if (1 <= v1) { // branch-flip
    v4 = NULL;
    if (!a0) {
      v4 = (void *)sub_6110(a3,a5,a2,v7);
      a0 = v4;
    }
    v3 = sub_14310(1,4,a0);
    v2 = sub_14310(0,4,a3);
    v6 = 0;
    error(0,v1,dcgettext(NULL,"cannot create hard link %s to %s",5),v2,v3);
    free(v4);
  }
  else {
    v5 = (unsigned char)v6 & (unsigned char)((unsigned int)v1 >> 0x1f);
    v6 = CONCAT31((undefined3)((unsigned int)v6 >> 8),v5);
    if (v5) { // branch-flip
      v3 = sub_14440(4,a3,0xffffffffffffffff);
      __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v3);
    }
    else {
      v6 = 1;
    }
  }
  return v6;
}

// Function: sub_6720 @ 0x6720
int sub_6720(char *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = strncmp(a0,"security.selinux",0x10);
  if (!v1)
    return v1;
  return (unsigned long)(attr_copy_check_permissions(a0,a1) != 0);
}

// Function: sub_6770 @ 0x6770
void sub_6770(void)
{
  char *v1; // rax
  
  v1 = dcgettext(NULL,"failed to restore the default file creation context",5);
  error(1,*__errno_location(),v1);
}

// Function: sub_67a0 @ 0x67a0
void sub_67a0(long a0,unsigned long a1,unsigned int a2,unsigned long a3,long a4)
{
  unsigned int v1;
  unsigned long v2;
  unsigned long v3; // rax
  char *v4; // rax
  long v5;
  char v6 [9];
  char v7; // stack - 0x32
  
  if ((((*(unsigned int *)(a4 + 0x18) & 0xf000) != 0xa000) && (!sub_16bb0())) && (faccessat(a2,a3,2,0x200))) {
    sub_e880(*(unsigned int *)(a4 + 0x18),&v5);
    v1 = *(unsigned int *)(a4 + 0x18);
    v7 = 0;
    v3 = sub_14440(4,a1);
    v2 = dat_235a0;
    if ((*(char *)(a0 + 0x18)) || (*(unsigned int *)(a0 + 0x14) & 0xffff00))
      v4 = dcgettext(NULL,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    else {
      v4 = dcgettext(NULL,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    __fprintf_chk(stderr,1,v4,v2,v3,(unsigned long)(v1 & 0xfff),v6);
  }
  else {
    v3 = sub_14440(4,a1);
    v2 = dat_235a0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: overwrite %s? ",5),v2,v3);
  }
  sub_17440(); // tail-call
}

// Function: sub_6920 @ 0x6920
unsigned long sub_6920(unsigned long a0,unsigned long a1,char a2,long a3)
{
  int v1;
  long v2; // rax
  int *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6; // r12
  
  v2 = lseek(a0,a3,1);
  if (v2 < 0) {
    v4 = sub_14440(4,a1);
    v5 = dcgettext(NULL,"cannot lseek %s",5);
    error(0,*__errno_location(),v5,v4);
    return 0;
  }
  if (a2) {
    if ((int)fallocate(a0 & 0xffffffff,3,v2 - a3,a3) <= -1) {
      v3 = __errno_location();
      v1 = *v3;
      v6 = CONCAT71((undefined7)((unsigned long)a3 >> 8),v1 == 0x5f || v1 == 0x26);
      if (v1 != 0x5f && v1 != 0x26) {
        v4 = sub_14440(4,a1);
        v5 = dcgettext(NULL,"error deallocating %s",5);
        error(0,*v3,v5,v4);
        return v6 & 0xffffffff; // return-dupe
      }
    }
  }
  v6 = 1;
  return v6 & 0xffffffff;
}

// Function: sub_6a30 @ 0x6a30
unsigned long sub_6a30(int a0,unsigned int a1,unsigned long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,unsigned long a7,unsigned long a8,unsigned long a9,long *a10,unsigned char *a11) // return-dupe
{
  bool v1;
  int v10;
  unsigned int v11; // eax
  long v12; // rax
  unsigned long v13;
  int *v14; // rax
  unsigned long v15;
  unsigned long v16;
  char *v17;
  bool v18;
  bool v19;
  unsigned int v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22;
  char *v23;
  unsigned long v24;
  unsigned char v25;
  char *v26;
  bool v27;
  unsigned long v28;
  unsigned long v29; // stack - 0x98
  unsigned long v3;
  unsigned long v4;
  long *v5;
  unsigned char *v6;
  long *v7;
  unsigned char *v8;
  bool v9;
  
  v8 = a11;
  v7 = a10;
  v15 = a8;
  v16 = a7;
  *a11 = 0;
  v29 = a9;
  *a10 = 0;
  if ((a4) || (!(char)a6)) {
    if (!a9)
      return 1;
label_6b28:
    v24 = a3;
    if (a4)
      v24 = a4;
    v28 = 0;
    v27 = 0;
    do {
      v17 = (char *)*a2;
      v2 = a6;
      v3 = a7;
      v4 = a8;
      v13 = a9;
      v5 = a10;
      v6 = a11;
      while( true ) {
        a6 = v2;
        a7 = v3;
        a8 = v4;
        a9 = v13;
        a10 = v5;
        a11 = v6;
        if (!v17) {
          v17 = (char *)sub_16bf0((long)getpagesize(),a3);
          *a2 = v17;
        }
        v13 = a3;
        if (v29 <= a3)
          v13 = v29;
        v13 = read(a0,v17,v13);
        if ((long)v13 <= -1) break;
        if (!v13) {
label_7023:
          if (v27)
            return sub_6920(a1,v15,a5,v28); // tail-call
          return 1;
        }
        *v7 = *v7 + v13;
        v20 = v24;
        v22 = v13;
        v26 = v17;
        v18 = v27;
label_6c20:
        do {
          v21 = v20;
          if (v22 < v20)
            v21 = v22;
          v27 = v21 && a4;
          if (v21 && a4) {
            v20 = v21;
            v23 = v17;
            do {
              if (*v23) {
                v19 = v18;
                v9 = 0;
                v1 = v27;
                goto label_6c89;
              }
              v23 = &v23[1];
              v20 -= 1;
              if (!v20) {
                v25 = (v18 ^ 1U) & v28 != 0;
                goto label_6d90;
              }
            } while (v20 & 0xf);
            v10 = memcmp(v17,v23,v20);
            v19 = (bool)(v10 == 0 ^ v18);
            v9 = v10 == 0;
            v1 = v10 != 0;
label_6c89:
            v27 = v9;
            v25 = v28 != 0 & v19;
            if ((v21 != v22) || (!v1)) {
label_6d90:
              if (!v25) goto label_6d99;
              v1 = 0;
            }
            else {
              if (!v25) {
                v27 = 0;
                goto label_6d32;
              }
              v1 = 1;
              v27 = 0;
            }
label_6d46:
            if (v18) { // branch-flip
              v11 = sub_6920(a1,v15,a5,v28);
              if (!(char)v11)
                return (unsigned long)v11;
            }
            else if (v28 != sub_111c0(a1,v26,v28)) {
              v16 = sub_14440(4,v15);
              v17 = dcgettext(NULL,"error writing %s",5);
              error(0,*__errno_location(),v17,v16);
              return 0;
            }
            v26 = v17;
            v28 = v21;
            v18 = v27;
            if (v1) {
              if (v21) { // branch-flip
                if (v25) {
                  v20 = 0;
                  goto label_6c20;
                }
              }
              else {
                if (v25) {
                  v28 = 0;
                  break;
                }
                v22 = 0;
              }
              v22 -= v21;
              v28 = 0;
            }
            else {
              v22 -= v21;
            }
          }
          else {
            if (((v21 == v22) && (v18 != 1)) || (v27 = v18, !v21)) {
label_6d32:
              v28 += v21;
              v25 = 0;
              v1 = 1;
              goto label_6d46;
            }
label_6d99:
            v28 += v21;
            if (v28 + 0x8000000000000000 < v21) {
              v16 = sub_14440(4,v16);
              error(0,0,dcgettext(NULL,"overflow reading %s",5),v16);
              return 0;
            }
            v22 -= v21;
            v18 = v27;
          }
          v17 = &v17[v21];
          v20 = v21;
        } while (v22);
        v29 -= v13;
        *v8 = v27;
        if (!v29) goto label_7023;
        v17 = (char *)*a2;
        v2 = a6;
        v3 = a7;
        v4 = a8;
        v13 = a9;
        v5 = a10;
        v6 = a11;
      }
      v14 = __errno_location();
      if (*v14 != 4) {
        v16 = sub_14440(4,v16);
        v17 = dcgettext(NULL,"error reading %s",5);
        error(0,*v14,v17,v16);
        return 0;
      }
    } while( true );
  }
  if (!a9)
    return 1;
  do {
    while( true ) {
      v24 = 0x7fffffffc0000000;
      if (v29 <= 0x7fffffffc0000000)
        v24 = v29;
      v12 = copy_file_range(a0,0,a1,0,v24,0);
      if (!v12) {
        if (*v7)
          return 1;
        goto label_6b28;
      }
      if (v12 < 0) break;
      *v7 = *v7 + v12;
      v29 -= v12;
      if (!v29)
        return 1;
    }
    v14 = __errno_location();
    v10 = *v14;
    if (v10 == 0x26) goto label_6b28;
    if (0x1a < v10) {
      if (v10 == 0x5f) goto label_6b28;
      break;
    }
    if (1 <= v10) {
      if (1L << ((unsigned char)v10 & 0x3f) & 0x4440200U) goto label_6b28;
      if (v10 == 1) {
        if (!*v7) goto label_6b28;
        break;
      }
    }
  } while (v10 == 4);
  v15 = sub_14310(1,4,v15);
  v16 = sub_14310(0,4,v16);
  v17 = dcgettext(NULL,"error copying %s to %s",5);
  error(0,*v14,v17,v16,v15);
  return 0;
}

// Function: sub_7080 @ 0x7080
int sub_7080(long a0,unsigned long a1,unsigned int a2,unsigned long a3,int a4,unsigned int a5,unsigned int a6,unsigned int a7,char a8,long a9) // return-dupe
{
  int v1;
  int *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  unsigned int v5;
  
  if (!a8) {
    if (*(unsigned long *)(a0 + 0x18) & 0xff0000000000ff) // branch-flip
      v5 = *(unsigned int *)(a9 + 0x18);
    else {
      if (!*(char *)(a0 + 0x39)) goto label_70f0;
      a5 = *(unsigned int *)(a0 + 0x10);
      v5 = *(unsigned int *)(a9 + 0x18);
    }
    if (((unsigned short)(~(unsigned short)a5 | 0xe00) & v5 & 0xfff) && (sub_127a0(a1,a4,a5 & v5 & 0x1c0))) {
      v2 = __errno_location();
      if (((*v2 != 1) && (*v2 != 0x16)) || (*(char *)(a0 + 0x1b))) {
        v3 = sub_14440(4,a1);
        v4 = dcgettext(NULL,"clearing permissions for %s",5);
        error(0,*v2,v4,v3);
      }
      return -(unsigned int)*(unsigned char *)(a0 + 0x32); // return-dupe
    }
  }
label_70f0:
  if (a4 != -1) { // branch-flip
    if (!fchown(a4,a6,a7))
      return 1;
    v2 = __errno_location();
    v1 = *v2;
    if ((v1 == 1) || (v1 == 0x16)) {
      fchown(a4,0xffffffff,a7);
      *v2 = v1;
label_720a:
      if (!*(char *)(a0 + 0x1a))
        return 0;
    }
  }
  else {
    v1 = fchownat(a2,a3,a6,a7,0x100);
    if (!v1)
      return 1;
    v2 = __errno_location();
    v1 = *v2;
    if ((v1 == 1) || (v1 == 0x16)) {
      fchownat(a2,a3,0xffffffff,a7,0x100);
      *v2 = v1;
      goto label_720a;
    }
  }
  v3 = sub_14440(4,a1);
  v4 = dcgettext(NULL,"failed to preserve ownership for %s",5);
  error(0,*v2,v4,v3);
  return -(unsigned int)*(unsigned char *)(a0 + 0x32);
}

// Function: sub_72a0 @ 0x72a0
unsigned char sub_72a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned char a3,long a4)
{
  unsigned char v1;
  long v2;
  int v3; // eax
  int *v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7; // stack - 0x38
  
  v1 = *(unsigned char *)(a4 + 0x33);
  if (!v1) {
    v2 = *(long *)(a4 + 0x28);
    a3 = v2 != 0 & a3;
    if ((a3) && ((int)sub_c450(v2,a1,a2) <= -1)) {
      v4 = __errno_location();
      if ((*v4 != 0x5f) && (*v4 != 0x3d)) {
        v5 = sub_14440(4,a1);
        v6 = dcgettext(NULL,"failed to set default file creation context for %s",5);
        error(0,*v4,v6,v5);
        return a3;
      }
    }
    return 1;
  }
  if ((*(char *)(a4 + 0x31)) && (!*(char *)(a4 + 0x34))) {
    if (*(char *)(a4 + 0x37)) { // branch-flip
      if ((int)sub_15590(a0,&v7) <= -1) goto label_732e;
      v3 = setfscreatecon(v7);
      if (0 <= v3) {
        freecon(v7); // return-dupe
        return v1;
      }
    }
    else {
      if ((int)sub_15590(a0,&v7) <= -1) {
        v4 = __errno_location();
        if ((*v4 == 0x5f) || (*v4 == 0x3d)) goto label_732e;
        goto label_72f8;
      }
      v3 = setfscreatecon(v7);
      if (0 <= v3) {
        freecon(v7);
        return v1;
      }
      v4 = __errno_location();
      if ((*v4 != 0x5f) && (*v4 != 0x3d)) goto label_74ba;
    }
  }
  else {
    if ((int)sub_15590(a0,&v7) <= -1) {
      v4 = __errno_location();
label_72f8:
      v5 = sub_14440(4,a0);
      v6 = dcgettext(NULL,"failed to get security context of %s",5);
      error(0,*v4,v6,v5);
label_732e:
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
label_74ba:
    v5 = sub_14b30(v7);
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

// Function: sub_7510 @ 0x7510
unsigned long sub_7510(unsigned long a0,char a1,long a2) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v2;
  char *v3; // rax
  int *v4; // rax
  
  v2 = *(unsigned long *)(a2 + 0x28);
  if ((*(char *)(a2 + 0x31)) && (!*(char *)(a2 + 0x34))) {
    if (*(char *)(a2 + 0x37))
      return sub_c6f0(v2,a0,a1); // tail-call
    v1 = sub_c6f0(v2,a0,a1);
    if ((char)v1)
      return 1;
    v4 = __errno_location();
    if (*v4 == 0x5f)
      return (unsigned long)v1;
    if (*v4 == 0x3d)
      return (unsigned long)v1;
  }
  else {
    if (sub_c6f0(v2,a0,a1))
      return 1;
    v4 = __errno_location();
  }
  v2 = sub_14310(0,4,a0);
  v3 = dcgettext(NULL,"failed to set the security context of %s",5);
  error(0,*v4,v3,v2);
  return 0;
}

// Function: sub_75f0 @ 0x75f0
void sub_75f0(long a0)
{
  long v1; // rax
  
  v1 = sub_11bb0(0x3d,0,sub_12460,sub_12430,sub_124d0);
  *(long *)(a0 + 0x48) = v1;
  if (v1)
    return;
  sub_17150(); // no-return
}

// Function: sub_7630 @ 0x7630
void sub_7630(long a0)
{
  long v1; // rax
  
  v1 = sub_11bb0(0x3d,0,sub_12410,sub_12430,sub_124d0);
  *(long *)(a0 + 0x50) = v1;
  if (v1)
    return;
  sub_17150(); // no-return
}

// Function: sub_7670 @ 0x7670
void sub_7670(unsigned long *a0)
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

// Function: sub_76c0 @ 0x76c0
int * sub_76c0(long a0)
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

// Function: sub_76f0 @ 0x76f0
unsigned long sub_76f0(void **a0,void **a1,int a2,void **a3,unsigned int a4,void **a5,unsigned long *a6,unsigned int *a7,unsigned int a8,void **a9,void **a10,char *a11)
{
  char *v1;
  unsigned int *v10; // rax
  unsigned int v100; // stack - 0x1dc
  unsigned int v101; // stack - 0x1d8
  void **v102; // stack - 0x1c8
  void *v103; // stack - 0x160
  void *v104; // stack - 0x158
  void *v105; // stack - 0x150
  void *v106; // stack - 0x148
  void *v107; // stack - 0x140
  void *v108; // stack - 0x138
  void **v109; // stack - 0x130
  char *v11;
  void *v110; // stack - 0x128
  void *v111; // stack - 0x120
  void *v112; // stack - 0x118
  void *v113; // stack - 0x110
  void *v114; // stack - 0x108
  void *v115; // stack - 0x100
  void *v116; // stack - 0xf8
  void *v117; // stack - 0xf0
  void *v118; // stack - 0xe8
  void *v119; // stack - 0xe0
  unsigned long v12;
  void *v120; // stack - 0xd0
  void *v121; // stack - 0xc8
  void *v122; // stack - 0xc0
  void *v123; // stack - 0xb8
  void *v124; // stack - 0xb0
  void **v125; // stack - 0xa8
  void *v126; // stack - 0xa0
  void *v127; // stack - 0x98
  void *v128; // stack - 0x90
  void *v129; // stack - 0x88
  unsigned long v13; // rax
  void *v130; // stack - 0x80
  void *v131; // stack - 0x78
  void *v132; // stack - 0x70
  void *v133; // stack - 0x68
  void *v134; // stack - 0x60
  void *v135; // stack - 0x58
  void *v136; // stack - 0x50
  void *v14; // rax
  int *v15; // rax
  void **v16;
  long v17;
  unsigned int v18;
  char v19 [8];
  char v2;
  void *v20; // stack - 0x288
  void *v21; // stack - 0x1f8
  void *v22; // stack - 0x168
  void *v23; // stack - 0xd8
  void *v24; // stack - 0x310
  char v25 [2]; // stack - 0x312
  long v26; // stack - 0x308
  void *v27; // stack - 0x2a8
  long v28;
  char *v29;
  unsigned char v3;
  char *v30;
  char *v31;
  char *v32;
  char *v33; // rsp
  char *v34; // rsp
  void **v35;
  void *v36;
  char v37;
  unsigned long v38;
  unsigned long *v39;
  char v4;
  unsigned long v40; // rax
  unsigned long v41; // r13
  unsigned int *v42;
  unsigned char v43;
  bool v44;
  unsigned long *v45; // stack - 0x3e0
  unsigned int v46; // stack - 0x3d8
  unsigned int v47; // stack - 0x3d4
  void **v48; // stack - 0x3d0
  unsigned int v49; // stack - 0x3c8
  int v5;
  unsigned int v50; // stack - 0x3c4
  unsigned int *v51; // stack - 0x3c0
  void **v52; // stack - 0x3b8
  char *v53; // stack - 0x3b0
  unsigned long *v54; // stack - 0x3a8
  unsigned int *v55; // stack - 0x3a0
  unsigned long *v56; // stack - 0x398
  void **v57; // stack - 0x390
  char *v58; // stack - 0x388
  void **v59; // stack - 0x380
  unsigned int v6;
  void **v60; // stack - 0x378
  void **v61; // stack - 0x370
  unsigned long v62; // stack - 0x368
  unsigned long *v63; // stack - 0x360
  void **v64; // stack - 0x358
  void **v65; // stack - 0x350
  char *v66; // stack - 0x348
  void **v67; // stack - 0x340
  void **v68; // stack - 0x338
  int v69; // stack - 0x330
  unsigned int v7;
  unsigned int v70; // stack - 0x32c
  void **v71; // stack - 0x328
  void **v72; // stack - 0x320
  unsigned long v73; // stack - 0x300
  unsigned long v74; // stack - 0x2f8
  unsigned long v75; // stack - 0x2f0
  unsigned long v76; // stack - 0x2e8
  unsigned long v77; // stack - 0x2e0
  unsigned long v78; // stack - 0x2d8
  unsigned long v79; // stack - 0x2d0
  unsigned int v8;
  unsigned long v80; // stack - 0x2c8
  unsigned long v81; // stack - 0x2c0
  unsigned long v82; // stack - 0x2b8
  void *v83; // stack - 0x2a0
  void *v84; // stack - 0x298
  void *v85; // stack - 0x290
  void *v86; // stack - 0x280
  unsigned long v87; // stack - 0x278
  unsigned int v88; // stack - 0x270
  unsigned int v89; // stack - 0x26c
  unsigned long *v9;
  unsigned int v90; // stack - 0x268
  unsigned long v91; // stack - 0x260
  unsigned long v92; // stack - 0x258
  void *v93; // stack - 0x240
  void *v94; // stack - 0x238
  void *v95; // stack - 0x230
  void *v96; // stack - 0x228
  void *v97; // stack - 0x1f0
  unsigned long v98; // stack - 0x1e8
  unsigned int v99; // stack - 0x1e0
  
  v42 = a7;
  v31 = v19;
  v32 = v19;
  v34 = v19;
  v29 = v19;
  v68 = a1;
  v67 = a3;
  v63 = a6;
  v69 = a2;
  v60 = a5;
  v71 = (void **)CONCAT44(v71._4_4_,a8);
  v64 = a9;
  v8 = a7[0x10];
  v61 = a10;
  v66 = a11;
  v62 = CONCAT71(v62._1_7_,(char)a8);
  *(unsigned char *)a10 = 0;
  v72 = (void **)CONCAT71(v72._1_7_,(char)a7[6]);
  if ((char)a7[6]) {
    if ((int)v8 < 0) {
      if (!sub_14b50(0xffffff9c,a0,a2,a3,1)) {
        a4 = 1;
        *v66 = 1;
        goto label_79ae;
      }
      v8 = *__errno_location();
    }
    v44 = v8 == 0;
    v72 = (void **)CONCAT71(v72._1_7_,v44);
    a4 = (unsigned int)v44;
    *v66 = v44;
    if (!v8) goto label_79ae;
label_779d:
    if ((v8 != 0x11) || (v42[2] != 2)) {
      v35 = a0;
      v16 = a0;
      v5 = -100;
      v70 = a4;
      goto label_77b2;
    }
label_7803:
    if (((char)v71) && (v17 = *(long *)&v42[0x14], v17)) {
      v16 = &v20;
      if (((v70 & 0xf000) != 0x4000) && (!*v42)) {
        v59 = (void **)CONCAT44(v59._4_4_,a4);
        v65 = &v20;
        v7 = sub_e820(v17,a0,&v20);
        v38 = (unsigned long)v7;
        if ((char)v7) {
          v12 = sub_14440(4,a0);
          v11 = "warning: source file %s specified more than once";
          v11 = dcgettext(NULL,v11,5); // return-dupe
          error(0,0,v11,v12); // return-dupe
          return v38 & 0xffffffff; // return-dupe
        }
        v17 = *(long *)&v42[0x14];
        v16 = v65;
        a4 = (unsigned int)v59;
      }
      v65 = (void **)CONCAT44(v65._4_4_,a4);
      sub_e790(v17,a0,v16);
      a4 = (unsigned int)v65;
    }
    if (v42[1] == 4) {
      v59 = (void **)CONCAT44(v59._4_4_,1);
label_787c:
      v65 = NULL;
      v43 = 0;
      v32 = v19;
      if (1 <= (int)a4) goto label_7892;
label_7b88:
      if ((v8 == 0x11) && (v42[2] == 2)) {
        v43 = 0;
        v2 = '\0';
label_8af3:
        if (!*(char *)((long)v42 + 0x3b)) {
          if ((char)v42[6]) goto label_910b;
          if ((v70 & 0xf000) != 0x4000) {
label_8c1d:
            if ((v42[2] == 2) || ((v42[2] == 3 && (v4 = sub_67a0(v42,v68,v69,v67,&v21), !v4)))) {
              v38 = 1; // return-dupe
              return v38 & 0xffffffff;
            }
          }
          goto label_8c33;
        }
        if ((v70 & 0xf000) != 0x4000) { // branch-flip
          v44 = 0;
          if ((*(char *)((long)v42 + 0x1f)) && (v44 = 1, (char)v42[6]))
            v44 = v21 != v20;
          v5 = sub_15b50(v69,v67,&v21,&v20,v44);
          if (v5 < 0) {
            if (!(char)v42[6]) goto label_8c1d;
            goto label_910b;
          }
          if (v66)
            *v66 = 1;
          v17 = sub_bd30(v67,v86,v20);
          if ((v17) && (v2 = sub_65d0(0,v69,v17,v68,v69,v67,1,(char)v42[0xf],(unsigned long)v59 & 0xffffffff), !v2)) {
            if (*(char *)((long)v42 + 0x33)) {
              v32 = v19;
              if (setfscreatecon(0)) goto label_8bf7;
            }
            v38 = 0; // return-dupe
            return v38 & 0xffffffff;
          }
        }
        else {
          if (!(char)v42[6]) goto label_8c33;
label_910b:
          v8 = v42[2];
          if (v8 != 2) {
            if (v8 != 3) {
              if ((v8 != 4) || (!*(char *)((long)v42 + 0x3d))) {
label_8c33:
                if (v2) {
                  v38 = 1;
                  return v38 & 0xffffffff;
                }
                if ((v99 & 0xf000) != 0x4000) goto label_8c56;
                if ((v70 & 0xf000) != 0x4000) {
label_9818:
                  if ((char)v42[6]) {
                    v38 = (unsigned long)*v42;
                    v8 = v99;
                    if (*v42) goto label_982f;
                  }
                  v12 = sub_14440(4,v68);
                  v38 = 0;
                  error(0,0,dcgettext(NULL,"cannot overwrite directory %s with non-directory",5),v12);
                  return v38 & 0xffffffff;
                }
label_8cc9:
                v38 = (unsigned long)*v42;
label_8ccc:
                if ((char)v42[6]) { // branch-flip
                  if ((v88 & 0xf000) == 0x4000) {
                    v8 = v99;
                    goto label_a0b6;
                  }
label_9e4c:
                  if ((int)v38) goto label_9847;
label_8d80:
                  v65 = NULL;
                  v8 = 0x11;
                  v32 = v19;
                }
                else {
                  v3 = 0;
                  if ((int)v38) goto label_9e1b;
label_8ce2:
                  v8 = 0x11;
                  v65 = NULL;
                  v3 = (v99 & 0xf000) == 0x4000 | v3;
                  v38 = CONCAT71((undefined7)(v38 >> 8),v3);
                  v32 = v19;
                  if (!v3) {
                    if (!*(char *)((long)v42 + 0x15)) {
                      v32 = v19;
                      if (!*(char *)((long)v42 + 0x31)) goto label_7892;
                      if ((!(char)v42[0xc]) || (v98 <= 1)) {
                        v8 = 0x11;
                        v65 = NULL;
                        v32 = v19;
                        if ((v42[1] != 2) || (v32 = v19, (v88 & 0xf000) == 0x8000)) goto label_7892;
                      }
                    }
                    v65 = NULL;
                    v5 = unlinkat(v69,(char *)v67,0);
                    if ((v5) && (v15 = __errno_location(), *v15 != 2)) {
                      v12 = sub_14440(4,v68);
                      v11 = dcgettext(NULL,"cannot remove %s",5);
                      error(0,*v15,v11,v12);
                      return v38 & 0xffffffff;
                    }
                    v72 = (void **)CONCAT71(v72._1_7_,(char)v42[0xf]);
                    if (!(char)v42[0xf]) {
                      v72 = (void **)CONCAT71(v72._1_7_,1);
                      goto label_8d80;
                    }
                    v8 = 0x11;
                    v12 = sub_14440(4,v68);
                    __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v12);
                    v65 = NULL;
                    v32 = v19;
                  }
                }
              }
              else {
                if ((v99 & 0xf000) != 0xa000) {
                  if ((!sub_16bb0()) && (v5 = faccessat(v69,v67,2,0x200), v5)) goto label_917c;
                  goto label_8c33;
                }
                if (v2) {
                  v38 = 1;
                  return v38 & 0xffffffff;
                }
label_8c56:
                v7 = v70 & 0xf000;
                v8 = v99;
                if (v7 != 0x4000) { // branch-flip
                  v38 = (unsigned long)*v42;
                  if (!(char)v71) goto label_8ccc;
label_8c7d:
                  if ((int)v38 != 3) {
                    v2 = sub_e820(*(unsigned long *)&v42[0x12],v67,&v21);
                    if (v2) {
                      v12 = sub_14310(1,4,a0);
                      v13 = sub_14310(0,4,v68);
                      v11 = "will not overwrite just-created %s with %s";
                      error(0,0,dcgettext(NULL,v11,5),v13,v12); // return-dupe
                      v38 = 0;
                      return v38 & 0xffffffff;
                    }
                    if ((v7 == 0x4000) || ((v99 & 0xf000) != 0x4000)) goto label_8cc9;
                    goto label_9818;
                  }
                  if ((char)v42[6]) goto label_982f;
label_9e1b:
                  v5 = (int)v38;
                  v11 = (char *)sub_d970(a0);
                  if (*v11 == '.') {
                    v3 = 0;
                    goto label_985e;
                  }
label_9883:
                  if ((v99 & 0xf000) == 0x4000) goto label_8d80;
                }
                else {
                  if ((!(char)v42[6]) || (v38 = (unsigned long)*v42, !*v42)) {
                    v12 = sub_14310(1,4,a0);
                    v13 = sub_14310(0,4,v68);
                    v11 = "cannot overwrite non-directory %s with directory %s";
                    error(0,0,dcgettext(NULL,v11,5),v13,v12);
                    v38 = 0;
                    return v38 & 0xffffffff;
                  }
                  if ((char)v71) goto label_8c7d;
label_982f:
                  if ((v88 & 0xf000) == 0x4000) {
label_a0b6:
                    if ((v8 & 0xf000) == 0x4000) goto label_9e4c;
                    if (!(int)v38) {
                      v12 = sub_147c0(0,3,v68);
                      v13 = sub_147c0(0,3,a0);
                      v11 = "cannot move directory onto non-directory: %s -> %s";
                      error(0,0,dcgettext(NULL,v11,5),v13,v12);
                      v38 = 0;
                      return v38 & 0xffffffff;
                    }
                  }
label_9847:
                  v5 = (int)v38;
                  v11 = (char *)sub_d970(a0);
                  v3 = 1;
                  if (*v11 == '.') {
label_985e:
                    v5 = (int)v38;
                    if ((!v11[(unsigned long)(v11[1] == '.') + 1]) || (v11[(unsigned long)(v11[1] == '.') + 1] == '/')) goto label_8ce2;
                    if (!v3) goto label_9883;
                  }
                }
                if (v5 != 3) {
                  v72 = (void **)strlen(v11);
                  v58 = (char *)sub_d970(v67);
                  v65 = (void **)strlen(v58);
                  v1 = dat_23580;
                  v38 = strlen(dat_23580);
                  if (v72 == (void **)((long)v65 + v38)) {
                    v72 = v65;
                    v5 = memcmp(v11,v58,(unsigned long)v65);
                    if (!v5) {
                      v5 = strcmp(&v11[(long)v72],v1);
                      v16 = v67;
                      if (!v5) {
                        v38 = strlen((char *)v67);
                        v11 = (char *)sub_6110(v16,(long)v16 + v38,v1);
                        v5 = fstatat(v69,v11,&v23,0);
                        free(v11);
                        if (((!v5) && (v86 == v120)) && (v20 == v23)) {
                          if ((char)v42[6]) // branch-flip
                            v11 = dcgettext(NULL,"backing up %s might destroy source;  %s not moved",5);
                          else {
                            v11 = dcgettext(NULL,"backing up %s might destroy source;  %s not copied",5);
                          }
                          v12 = sub_14310(1,4,a0);
                          v13 = sub_14310(0,4,v68);
                          v38 = 0;
                          error(0,0,v11,v13,v12);
                          return v38 & 0xffffffff;
                        }
                      }
                    }
                  }
                }
                v16 = v67;
                v11 = (char *)sub_d840(v69,v67,*v42);
                if (v11) { // branch-flip
                  v41 = (long)v16 - (long)v68;
                  v40 = strlen(v11);
                  v38 = v41 + 0x18 + v40;
                  v32 = v19;
                  while (v34 != &v19[-(v38 & 0xfffffffffffff000)]) {
                    v29 = &v32[-0x1000];
                    v33 = &v32[-0x1000];
                    *(unsigned long *)&v32[-8] = *(unsigned long *)&v32[-8];
                    v34 = &v32[-0x1000];
                    v32 = v33;
                  }
                  v38 = (unsigned long)((unsigned int)v38 & 0xff0);
                  v17 = -v38;
                  if (v38)
                    *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
                  v72 = (void **)(v40 + 1);
                  v8 = 0x11;
                  v65 = (void **)((unsigned long)&v29[v17 + 0xf] & 0xfffffffffffffff0);
                  *(unsigned long *)&v29[v17 + -8] = 0x9a35;
                  v36 = mempcpy((void **)((unsigned long)&v29[v17 + 0xf] & 0xfffffffffffffff0),v68,v41,v29[v17 + -8]);
                  *(unsigned long *)&v29[v17 + -8] = 0x9a47;
                  memcpy(v36,v11,(unsigned long)v72,v29[v17 + -8]);
                  *(unsigned long *)&v29[v17 + -8] = 0x9a4f;
                  free(v11,v29[v17 + -8]);
                  v72 = (void **)CONCAT71(v72._1_7_,1);
                  v32 = &v29[v17];
                }
                else {
                  v15 = __errno_location();
                  if (*v15 != 2) {
                    v12 = sub_14440(4,v68);
                    v11 = dcgettext(NULL,"cannot backup %s",5);
                    v38 = 0;
                    error(0,*v15,v11,v12);
                    return v38 & 0xffffffff;
                  }
                  v72 = (void **)CONCAT71(v72._1_7_,1);
                  v8 = 0x11;
                  v65 = NULL;
                  v32 = v19;
                }
              }
label_7892:
              if ((char)v71) goto label_7ad0;
              goto label_789f;
            }
label_917c:
            v4 = sub_67a0(v42,v68,v69,v67,&v21);
            if (v4) goto label_8c33;
          }
label_91b0:
          if (v66)
            *v66 = 1;
        }
        v38 = 1;
        return v38 & 0xffffffff;
      }
      v7 = v70 & 0xf000;
      if ((((v7 != 0x8000) && (v3 = (v7 == 0xa000 || v7 == 0x4000) | (unsigned char)v42[5] ^ 1, v3)) || (v3 = (unsigned char)v42[6], v3)) || ((v3 = *(unsigned char *)((long)v42 + 0x3a), v3 || (v3 = *(unsigned char *)((long)v42 + 0x17), v3)))) {
label_7bca:
        v5 = 0x100;
label_7bd0:
        v5 = fstatat(v69,(char *)v67,&v21,v5);
        if (v5) {
          v15 = __errno_location();
          if (*v15 != 0x28) { // branch-flip
            if (*v15 == 2) goto label_8460;
          }
          else if (*(char *)((long)v42 + 0x16)) goto label_8467;
          v12 = sub_14440(4,v68);
          v11 = dcgettext(NULL,"cannot stat %s",5);
          v38 = 0;
          error(0,*v15,v11,v12);
          return v38 & 0xffffffff;
        }
      }
      else {
        if (*v42) {
          v3 = 1;
          goto label_7bca;
        }
        v3 = *(unsigned char *)((long)v42 + 0x15);
        if (v3) goto label_7bca;
        v5 = 0;
        if (!a4) goto label_7bd0;
label_8460:
        v72 = (void **)CONCAT71(v72._1_7_,1);
label_8467:
        v65 = NULL;
        v43 = 0;
        v32 = v19;
        v3 = 0;
        if (v8 != 0x11) goto label_7892;
      }
      v43 = v3;
      if (v42[2] == 2) goto label_8af0;
      if ((v86 == v97) && (v20 == v21)) {
        v2 = *(char *)((long)v42 + 0x17);
        if (!v2) {
          if (v42[1] == 2) {
            v44 = 1;
            v18 = 1;
            goto label_7c23;
          }
          v16 = &v22;
          v5 = fstatat(v69,(char *)v67,v16,0x100);
          if (!v5) {
            v65 = &v23;
            v5 = lstat((char *)a0,v65);
            if (!v5) {
              v44 = v120 == v103 && v23 == v22;
              v35 = v65;
              if (((((unsigned int)v122 & 0xf000) != 0xa000) || (((unsigned int)v105 & 0xf000) != 0xa000)) || (!*(char *)((long)v42 + 0x15))) goto label_7c47;
            }
          }
        }
        goto label_8af3;
      }
      if (v42[1] != 2) goto label_8af0;
      v44 = 0;
      v18 = 0;
label_7c23:
      v16 = &v21;
      v35 = &v20;
      if (((v88 & 0xf000) != 0xa000) || ((v99 & 0xf000) != 0xa000)) {
label_7c47:
        if (*v42) { // branch-flip
          if (!v44) {
            if (((((char)v42[6]) || (v42[1] == 2)) || (((unsigned long)v35[3] & 0xf000) != 0xa000)) || (((unsigned long)v16[3] & 0xf000) == 0xa000)) goto label_8af0;
            goto label_7c99;
          }
          v2 = '\0';
          v3 = sub_14e20(0xffffff9c,a0,v69,v67);
          v3 ^= 1;
label_a173:
          if (v3) goto label_8af3;
        }
        else {
          v2 = (char)v42[6];
          if ((v2) || (*(char *)((long)v42 + 0x15))) {
            if (((unsigned long)v16[3] & 0xf000) == 0xa000) goto label_8af0;
            if ((v44) && ((void *)0x2 <= v16[2])) {
              v65 = v35;
              v2 = sub_14e20(0xffffff9c,a0,v69,v67);
              if (v2) {
                v35 = v65;
                if (((unsigned long)v65[3] & 0xf000) != 0xa000) goto label_9bb4;
                goto label_9bc9;
              }
              v3 = (unsigned char)v42[6] ^ 1;
              v2 = '\0';
              goto label_a173;
            }
            if (((unsigned long)v35[3] & 0xf000) != 0xa000) goto label_9e99;
label_9bce:
            if (((v2) && ((v88 & 0xf000) == 0xa000)) && ((void *)0x2 <= v16[2])) {
              v65 = v35;
              v11 = canonicalize_file_name((char *)a0);
              v35 = v65;
              if (v11) {
                v3 = sub_14e20(0xffffff9c,v11,v69,v67);
                v2 = '\0';
                v3 ^= 1;
                free(v11);
                goto label_a173;
              }
            }
          }
          else if (((unsigned long)v35[3] & 0xf000) != 0xa000) {
label_9bb4:
            if (((unsigned long)v16[3] & 0xf000) != 0xa000) {
label_9e99:
              if ((v35[1] != v16[1]) || (*v35 != *v16)) goto label_8af0;
              v2 = *(char *)((long)v42 + 0x17);
              if (v2) goto label_8af3;
            }
label_9bc9:
            v2 = (char)v42[6];
            goto label_9bce;
          }
          if ((*(char *)((long)v42 + 0x3a)) && (((unsigned long)v16[3] & 0xf000) == 0xa000)) {
label_8af0:
            v2 = '\0';
            goto label_8af3;
          }
          if (v42[1] == 2) {
            if (((unsigned long)v35[3] & 0xf000) != 0xa000) { // branch-flip
              v23 = *v35;
              v120 = v35[1];
              v121 = v35[2];
              v122 = v35[3];
              v123 = v35[4];
              v124 = v35[5];
              v125 = v35[6];
              v126 = v35[7];
              v127 = v35[8];
              v128 = v35[9];
              v129 = v35[10];
              v130 = v35[0xb];
              v131 = v35[0xc];
              v132 = v35[0xd];
              v133 = v35[0xe];
              v134 = v35[0xf];
              v135 = v35[0x10];
              v136 = v35[0x11];
            }
            else if (stat((char *)a0,&v23)) goto label_8af0;
            if (((unsigned long)v16[3] & 0xf000) != 0xa000) { // branch-flip
              v22 = *v16;
              v103 = v16[1];
              v104 = v16[2];
              v105 = v16[3];
              v106 = v16[4];
              v107 = v16[5];
              v108 = v16[6];
              v109 = v16[7];
              v110 = v16[8];
              v111 = v16[9];
              v112 = v16[10];
              v113 = v16[0xb];
              v114 = v16[0xc];
              v115 = v16[0xd];
              v116 = v16[0xe];
              v117 = v16[0xf];
              v118 = v16[0x10];
              v119 = v16[0x11];
            }
            else {
              v5 = fstatat(v69,(char *)v67,&v22,0);
              if (v5) goto label_8af0;
            }
            if ((v120 != v103) || (v23 != v22)) goto label_8af0;
            if (*(char *)((long)v42 + 0x17)) {
              v2 = ((unsigned long)v16[3] & 0xf000) != 0xa000;
              goto label_8af3;
            }
          }
        }
      }
      else {
        v65 = (void **)CONCAT44(v65._4_4_,v18);
        v2 = sub_14e20(0xffffff9c,a0,v69,v67);
        if (!v2) {
          v2 = '\0';
          if ((!*v42) && ((int)v65)) {
            v2 = '\x01';
            v3 = (unsigned char)v42[6] ^ 1;
            goto label_a173;
          }
          goto label_8af3;
        }
      }
label_7c99:
      v12 = sub_14310(1,4,v68);
      v13 = sub_14310(0,4,a0);
      v11 = "%s and %s are the same file";
      error(0,0,dcgettext(NULL,v11,5),v13,v12);
      v38 = 0;
      return v38 & 0xffffffff;
    }
    if ((v42[1] != 3) || (!(char)v71)) {
      v59 = (void **)((unsigned long)v59 & 0xffffffff00000000);
      goto label_787c;
    }
    v59 = (void **)CONCAT44(v59._4_4_,1);
    if ((int)a4 <= 0) goto label_7b88;
    v65 = NULL;
    v43 = 0;
label_7ad0:
    if ((!*(long *)&v42[0x12]) || ((char)v42[6])) {
label_789f:
      if (((char)v42[0xf]) && (!(char)v42[6])) {
label_8258:
        if ((v70 & 0xf000) != 0x4000) {
          *(unsigned long *)&v32[-8] = 0x8284;
          sub_6510(a0,v68,v65);
        }
      }
      if (v8) goto label_78ba;
      v29 = v32;
      if ((char)v42[6]) goto label_7d27;
label_7d98:
      v40 = (unsigned long)v72 & 0xff;
      v9 = NULL;
label_7da3:
      v8 = v70;
      if (*(char *)((long)v42 + 0x39))
        v8 = v42[4];
      v32 = v29;
      if (*(char *)((long)v42 + 0x1d)) {
        *(unsigned long *)&v29[-8] = 0x7e4b;
        v2 = sub_72a0(a0,v68,v70,v40,v42);
        if (!v2) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
        v58 = (char *)(CONCAT44(v58._4_4_,v8) & 0xffffffff0000003f);
        v66 = (char *)(CONCAT44(v66._4_4_,v70) & 0xffffffff0000f000);
        v39 = v63;
        if ((v70 & 0xf000) == 0x4000) goto label_8356;
label_7e7a:
        v7 = (unsigned int)*(unsigned char *)((long)v42 + 0x3a);
        if (!*(unsigned char *)((long)v42 + 0x3a)) {
          if (!*(char *)((long)v42 + 0x17)) {
            if (((unsigned int)v66 != 0x8000) && (!((unsigned int)v66 != 0xa000 & (unsigned char)v42[5]))) {
              v64 = (void **)((unsigned long)v64._1_7_ << 8);
              if ((unsigned int)v66 == 0x1000) {
                v8 = ~(unsigned int)v58 & v70;
                *(unsigned long *)&v29[-8] = 0xa2d6;
                v5 = mknodat(v69,v67,v8,0);
                v2 = (char)v64;
                if (v5) {
                  *(unsigned long *)&v29[-8] = 0xa2fd;
                  v5 = mkfifoat(v69,v67,v8 & 0xffffefff);
                  v2 = (char)v64;
                  if (v5) {
                    *(unsigned long *)&v29[-8] = 0xa31e;
                    v12 = sub_14440(4,v68);
                    v11 = "cannot create fifo %s";
                    goto label_8638;
                  }
                }
                goto label_7f17;
              }
              if (((unsigned int)v66 & 0xffffbfff) == 0x2000 || (unsigned int)v66 == 0xc000) {
                *(unsigned long *)&v29[-8] = 0x8601;
                v5 = mknodat(v69,v67,~(unsigned int)v58 & v70,v91);
                v2 = (char)v64;
                if (v5) {
                  *(unsigned long *)&v29[-8] = 0x8622;
                  v12 = sub_14440(4,v68);
                  v11 = "cannot create special file %s";
                  goto label_8638;
                }
                goto label_7f17;
              }
              if ((unsigned int)v66 == 0xa000) {
                *(unsigned long *)&v29[-8] = 0x9f44;
                v11 = (char *)sub_c960(a0,v92);
                if (!v11) {
                  *(unsigned long *)&v29[-8] = 0xae5a;
                  v12 = sub_14440(4,a0);
                  v11 = "cannot read symbolic link %s";
                  goto label_8638;
                }
                v37 = *(char *)((long)v42 + 0x16);
                *(unsigned long *)&v29[-8] = 0x9f70;
                v5 = sub_c000(v11,v69,v67,v37,0xffffffff);
                if (v5 <= 0) {
                  *(unsigned long *)&v29[-8] = 0xa90a;
                  free(v11,v29[-8]);
label_a90a:
                  if (*(char *)((long)v42 + 0x33)) {
                    *(unsigned long *)&v29[-8] = 0xae40;
                    if (setfscreatecon(0)) goto label_8bf7;
                  }
                  v7 = 1;
                  v2 = '\0';
                  if (*(char *)((long)v42 + 0x1d)) {
                    v64 = (void **)CONCAT71(v64._1_7_,*(char *)((long)v42 + 0x1d));
                    *(unsigned long *)&v29[-8] = 0xa954;
                    v5 = fchownat(v69,v67,v89,v90,0x100);
                    if (v5) { // branch-flip
                      *(unsigned long *)&v29[-8] = 0xa96c;
                      v7 = sub_76c0(v42);
                      v2 = '\0';
                      if (!(char)v7) {
                        *(unsigned long *)&v29[-8] = 0xa98d;
                        v11 = dcgettext(NULL,"failed to preserve ownership for %s",5,v29[-8]);
                        *(unsigned long *)&v29[-8] = 0xa995;
                        v37 = v29[-8];
                        v5 = *__errno_location(v37);
                        *(unsigned long *)&v29[-8] = 0xa9aa;
                        error(0,v5,v11,v68);
                        if (*(char *)((long)v42 + 0x32)) goto label_8658;
                        v7 = (unsigned int)(unsigned char)v64;
                        v2 = '\0';
                      }
                    }
                    else {
                      v7 = (unsigned int)(unsigned char)v64;
                      v2 = '\0';
                    }
                  }
                  goto label_7f17;
                }
                if (((*(char *)((long)v42 + 0x3b) == '\x01') && (!(char)v72)) && ((v99 & 0xf000) == 0xa000)) {
                  v64 = v102;
                  *(unsigned long *)&v29[-8] = 0xafb5;
                  v37 = v29[-8];
                  if (v64 == (void **)strlen(v11,v37)) {
                    *(unsigned long *)&v29[-8] = 0xafd7;
                    v16 = (void **)sub_caf0(v69,v67);
                    if (v16) {
                      v64 = v16;
                      *(unsigned long *)&v29[-8] = 0xaff2;
                      v37 = v29[-8];
                      if (!strcmp((char *)v16,v11,v37)) {
                        *(unsigned long *)&v29[-8] = 0xb006;
                        free(v64,v29[-8]);
                        *(unsigned long *)&v29[-8] = 0xb00e;
                        free(v11,v29[-8]);
                        goto label_a90a;
                      }
                      *(unsigned long *)&v29[-8] = 0xba71;
                      free(v64,v29[-8]);
                    }
                  }
                }
                *(unsigned long *)&v29[-8] = 0x9f97;
                free(v11,v29[-8]);
                *(unsigned long *)&v29[-8] = 0x9fa8;
                v12 = sub_14440(4,v68);
                *(unsigned long *)&v29[-8] = 0x9fbe;
                v11 = dcgettext(NULL,"cannot create symbolic link %s",5,v29[-8]);
                *(unsigned long *)&v29[-8] = 0x9fd0;
                error(0,v5,v11,v12);
                goto label_8658;
              }
              *(unsigned long *)&v29[-8] = 0xa42d;
              v12 = sub_14440(4,a0);
              v11 = "%s has unknown file type";
label_8394:
              *(unsigned long *)&v29[-8] = 0x839b;
              v11 = dcgettext(NULL,v11,5,v29[-8]);
              *(unsigned long *)&v29[-8] = 0x83ac;
              error(0,0,v11,v12);
              goto label_8658;
            }
            v7 = v42[1];
            v24 = NULL;
            v61 = (void **)CONCAT44(v61._4_4_,v88);
            v64 = (void **)CONCAT71(v64._1_7_,*(char *)((long)v42 + 0x31));
            v63 = (unsigned long *)CONCAT71(v63._1_7_,*(char *)((long)v42 + 0x35));
            *(unsigned long *)&v29[-8] = 0x946d;
            v7 = sub_e620(a0,(unsigned long)(v7 == 2) << 0x11);
            v38 = (unsigned long)v7;
            if ((int)v7 < 0) {
              *(unsigned long *)&v29[-8] = 0xa25e;
              v12 = sub_14440(4,a0);
              v11 = "cannot open %s for reading";
              goto label_8638;
            }
            *(unsigned long *)&v29[-8] = 0x9486;
            v37 = v29[-8];
            if (fstat(v7,&v23,v37)) { // branch-flip
              *(unsigned long *)&v29[-8] = 0xa4a8;
              v64 = (void **)sub_14440(4,a0);
              *(unsigned long *)&v29[-8] = 0xa4c2;
              v11 = dcgettext(NULL,"cannot fstat %s",5,v29[-8]);
              *(unsigned long *)&v29[-8] = 0xa4ca;
              v37 = v29[-8];
              v5 = *__errno_location(v37);
              v16 = v64;
label_a000:
              v36 = NULL;
              *(unsigned long *)&v29[-8] = 0xa00b;
              error(0,v5,v11,v16);
              v37 = 0;
            }
            else {
              if ((v86 != v120) || (v20 != v23)) {
                *(unsigned long *)&v29[-8] = 0x9fe2;
                v16 = (void **)sub_14440(4,a0);
                *(unsigned long *)&v29[-8] = 0x9ff8;
                v11 = dcgettext(NULL,"skipping file %s, as it was replaced while being copied",5,v29[-8]);
                v5 = 0;
                goto label_a000;
              }
              v53 = (char *)(CONCAT44(v53._4_4_,v8) & 0xffffffff000001ff);
              if ((char)v72) goto label_a500;
              *(unsigned long *)&v29[-8] = 0x94f8;
              v8 = sub_125a0(v69,v67,(-(unsigned int)((char)v64 == '\0') & 0xfffffe00) + 0x201);
              v40 = (unsigned long)v8;
              *(unsigned long *)&v29[-8] = 0x94ff;
              v16 = (void **)__errno_location(v29[-8]);
              v5 = *(int *)v16;
              if (0 <= (int)v8) {
                if ((!*(long *)&v42[10]) && (!*(char *)((long)v42 + 0x33))) {
label_953b:
                  v46 = 0;
                  v51 = (unsigned int *)((unsigned long)v51 & 0xffffffff00000000);
label_954f:
                  v8 = (unsigned int)v40;
                  if ((char)v64) {
                    if (v42[0x11]) {
                      *(unsigned long *)&v29[-8] = 0xaeee;
                      if (!ioctl(v8,0x40049409,v38)) goto label_a64a;
                      if (v42[0x11] == 2) {
                        *(unsigned long *)&v29[-8] = 0xaf13;
                        v61 = (void **)sub_14310(1,4,a0);
                        *(unsigned long *)&v29[-8] = 0xaf2d;
                        v63 = (unsigned long *)sub_14310(0,4,v68);
                        *(unsigned long *)&v29[-8] = 0xaf47;
                        v37 = v29[-8];
                        v64 = (void **)dcgettext(NULL,"failed to clone %s from %s",5,v37);
                        *(unsigned long *)&v29[-8] = 0xaf53;
                        v37 = v29[-8];
                        v5 = *__errno_location(v37);
                        *(unsigned long *)&v29[-8] = 0xaf73;
                        error(0,v5,(char *)v64,v63,v61);
                        v43 = 0;
                        goto label_96e5;
                      }
                    }
label_9568:
                    *(unsigned long *)&v29[-8] = 0x9576;
                    v37 = v29[-8];
                    if (fstat(v8,&v22,v37)) {
                      *(unsigned long *)&v29[-8] = 0xb1d5;
                      v63 = (unsigned long *)sub_14440(4,v68);
                      v11 = "cannot fstat %s";
                      goto label_b133;
                    }
                    v60 = (void **)((unsigned long)v60 & 0xffffffff00000000);
                    if ((unsigned int)v105 != ((unsigned int)v51 | (unsigned int)v105)) {
                      *(unsigned long *)&v29[-8] = 0x959d;
                      v5 = fchmod(v8,(unsigned int)v51 | (unsigned int)v105,v29[-8]);
                      v18 = v60._0_4_;
                      if (!v5)
                        v18 = v51._0_4_;
                      v51 = (unsigned int *)CONCAT44(v51._4_4_,v18);
                    }
                    if (!(char)v64) goto label_ac58;
                    v48 = v109;
                    v57 = v109;
                    if ((void **)0x1ffffffffffe0001 <= &v109[-0x4000]) {
                      v57 = (void **)0x20000;
                      v48 = (void **)0x200;
                      if ((char *)((long)v109 + -1) <= (char *)0x1fffffffffffffff)
                        v48 = v109;
                    }
                    if (((unsigned int)v122 & 0xf000) != 0x8000) {
                      v5 = 1;
label_ab10:
                      if ((((unsigned int)v105 & 0xf000) != 0x8000) || ((v42[3] != 3 && ((v5 == 1 || (v42[3] != 2)))))) {
                        v60 = (void **)CONCAT44(v60._4_4_,v5);
                        *(unsigned long *)&v29[-8] = 0xab3e;
                        sub_e5e0(v38,0,0,2);
                        v36 = v126;
                        if (0x1ffffffffffe0001 <= (unsigned long)((long)v126 - 0x20000U))
                          v36 = (void *)0x20000;
                        *(unsigned long *)&v29[-8] = 0xab78;
                        v16 = (void **)sub_da00(v36,v57,0x7fffffffffffffff);
                        if ((((unsigned int)v122 & 0xf000) == 0x8000) && (v125 < v57))
                          v57 = (void **)((long)v125 + 1);
                        v25[0] = 0;
                        v11 = &((char *)((long)v16 + -1))[(long)v57];
                        if (1 <= (long)&v11[-((unsigned long)v11 % (unsigned long)v16)])
                          v16 = (void **)&v11[-((unsigned long)v11 % (unsigned long)v16)];
                        v57 = v16;
                        v47 = (unsigned int)(v42[0x11] != 0);
                        if ((int)v60 == 3) {
                          v49 = 1;
label_b1f7:
                          v39 = v56;
                          v25[0] = 0;
                          v60 = v125;
                          if (0 <= (long)v56) {
                            v59 = a0;
                            v52 = NULL;
                            v45 = NULL;
                            v54 = (unsigned long *)CONCAT71(v54._1_7_,v64._0_1_);
                            v17 = 0;
                            v56 = v9;
                            v55 = v42;
                            do {
                              *(unsigned long *)&v29[-8] = 0xb266;
                              v16 = (void **)lseek(v38,v39,4);
                              if ((long)v16 <= -1) {
                                *(unsigned long *)&v29[-8] = 0xb277;
                                v37 = v29[-8];
                                if (*__errno_location(v37) != 6) goto label_b50c;
                                v16 = v60;
                                if ((long)v39 < (long)v60) goto label_b293;
                                *(unsigned long *)&v29[-8] = 0xb443;
                                v16 = (void **)lseek(v38,0,2);
                                if ((long)v16 < 0) goto label_b50c;
                                if ((long)v39 < (long)v16) {
                                  v60 = v16;
                                  goto label_b293;
                                }
label_b500:
                                v60 = v16;
                                v9 = v56;
                                v42 = v55;
                                a0 = v59;
                                goto label_b3ff;
                              }
                              if ((long)v60 < (long)v16)
                                v60 = v16;
label_b293:
                              *(unsigned long *)&v29[-8] = 0xb2a0;
                              if ((long)lseek(v38,v39,0) < 0) {
label_b50c:
                                a0 = v59;
                                goto label_b521;
                              }
                              v17 = (long)v39 + (-v17 - (long)v45);
                              if (v17) {
                                if (v49 != 1) {
                                  *(unsigned long *)&v29[-8] = 0xb2e1;
                                  v2 = sub_6920(v40,v68,v49 == 3);
                                  v54 = (unsigned long *)CONCAT71(v54._1_7_,v2);
                                  if (v2) {
                                    v17 = (long)v16 - (long)v39;
                                    v16 = v48;
                                    goto label_b2fc;
                                  }
label_b575:
                                  v9 = v56;
                                  v42 = v55;
                                  a0 = v59;
                                  goto label_b56d;
                                }
                                *(unsigned long *)&v29[-8] = 0xb46e;
                                v2 = sub_6170(v40,v17);
                                a0 = v59;
                                v9 = v56;
                                v42 = v55;
                                if (v2) {
                                  v17 = (long)v16 - (long)v39;
                                  v54 = (unsigned long *)((unsigned long)v54 & 0xffffffffffffff00);
                                  v16 = NULL;
                                  goto label_b2fc;
                                }
                                *(unsigned long *)&v29[-8] = 0xb982;
                                v60 = (void **)sub_147c0(0,3,v68);
                                v11 = "%s: write failed";
                                goto label_b541;
                              }
                              v17 = (long)v16 - (long)v39;
                              v16 = NULL;
                              if (v49 != 1)
                                v16 = v48;
                              v54 = (unsigned long *)((unsigned long)v54 & 0xffffffffffffff00);
label_b2fc:
                              a0 = v59;
                              *(char **)&v29[-8] = &v25[1];
                              *(unsigned long **)&v29[-0x10] = &v26;
                              *(long *)&v29[-0x18] = v17;
                              *(void ***)&v29[-0x20] = v68;
                              *(void ***)&v29[-0x28] = v59;
                              *(unsigned long *)&v29[-0x30] = (unsigned long)v47;
                              *(unsigned long *)&v29[-0x38] = 0xb342;
                              v2 = sub_6a30(v38,v40,&v24,v57,v16,1);
                              if (!v2) goto label_b575;
                              v16 = (void **)(v26 + (long)v39);
                              v43 = (unsigned char)v54;
                              if (v26)
                                v43 = v25[1];
                              v52 = v16;
                              v54 = (unsigned long *)CONCAT71(v54._1_7_,v43);
                              if (v26 < v17) goto label_b500;
                              *(unsigned long *)&v29[-8] = 0xb391;
                              v9 = (unsigned long *)lseek(v38,v16,3);
                              if ((long)v9 < 0) goto label_b3e3;
                              v45 = v39;
                              v39 = v9;
                            } while( true );
                          }
                          v52 = NULL;
                          v44 = 0 < (long)v125;
                          goto label_b5a2;
                        }
                        v48 = NULL;
                        v6 = v42[3];
                      }
                      else {
                        v60 = (void **)CONCAT44(v60._4_4_,v5);
                        *(unsigned long *)&v29[-8] = 0xb0b5;
                        sub_e5e0(v38,0,0,2);
                        v25[0] = 0;
                        if ((int)v60 == 3) {
                          v47 = (unsigned int)(v42[0x11] != 0);
                          v49 = v42[3];
                          goto label_b1f7;
                        }
                        v6 = v42[3];
                        v47 = (unsigned int)(v42[0x11] != 0);
                      }
                      v25[0] = 0;
                      *(char **)&v29[-8] = v25;
                      *(unsigned long **)&v29[-0x10] = &v26;
                      *(unsigned long *)&v29[-0x18] = 0xffffffffffffffff;
                      *(void ***)&v29[-0x20] = v68;
                      *(void ***)&v29[-0x28] = a0;
                      *(unsigned long *)&v29[-0x30] = (unsigned long)v47;
                      *(unsigned long *)&v29[-0x38] = 0xac38;
                      v2 = sub_6a30(v38,v40,&v24,v57,v48,v6 == 3);
                      goto label_ac42;
                    }
                    v5 = 1;
                    if ((long)v125 / 0x200 <= (long)v127) goto label_ab10;
                    *(unsigned long *)&v29[-8] = 0x9662;
                    v56 = (unsigned long *)lseek(v38,0,3);
                    if (0 <= (long)v56) {
label_b4d9:
                      v5 = 3;
                      goto label_ab10;
                    }
                    *(unsigned long *)&v29[-8] = 0x9677;
                    v60 = (void **)__errno_location(v29[-8]);
                    v5 = *(int *)v60;
                    if (v5 == 6) goto label_b4d9;
                    if ((v5 == 0x16) || (v5 == 0x5f)) {
                      v5 = 2;
                      goto label_ab10;
                    }
                    *(unsigned long *)&v29[-8] = 0x96ab;
                    v64 = (void **)sub_14440(4,a0);
                    *(unsigned long *)&v29[-8] = 0x96c5;
                    v11 = dcgettext(NULL,"cannot lseek %s",5,v29[-8]);
                    v5 = *(int *)v60;
                    *(unsigned long *)&v29[-8] = 0x96e2;
                    error(0,v5,v11,v64);
                    goto label_96e2;
                  }
label_a64a:
                  v6 = *(unsigned char *)((long)v42 + 0x1d) | (unsigned int)v51;
                  v64 = (void **)CONCAT44(v64._4_4_,v6);
                  if (v6) {
                    v64 = (void **)((unsigned long)v64 & 0xffffffffffffff00);
                    goto label_9568;
                  }
                  v105 = (void *)((unsigned long)v105 & 0xffffffff00000000);
                  if (*(char *)((long)v42 + 0x1f)) goto label_a676;
                  goto label_a74a;
                }
                *(unsigned long *)&v29[-8] = 0x9529;
                v2 = sub_7510(v68,0,v42);
                if ((v2) || (!(char)v42[0xd])) goto label_953b;
                v72 = (void **)((unsigned long)v72 & 0xffffffffffffff00);
                v43 = 0;
                goto label_96e5;
              }
              if (v5 != 2) {
                v2 = '\0';
                if (*(char *)((long)v42 + 0x16)) {
                  v60 = v16;
                  *(unsigned long *)&v29[-8] = 0xad87;
                  v5 = unlinkat(v69,(char *)v67,0,v29[-8]);
                  if (v5) { // branch-flip
                    if (*(int *)v60 != 2) {
                      v64 = v60;
                      *(unsigned long *)&v29[-8] = 0xb6b1;
                      v16 = (void **)sub_14440(4,v68);
                      *(unsigned long *)&v29[-8] = 0xb6c7;
                      v11 = dcgettext(NULL,"cannot remove %s",5,v29[-8]);
                      v5 = *(int *)v64;
                      goto label_a000;
                    }
                  }
                  else if ((char)v42[0xf]) {
                    *(unsigned long *)&v29[-8] = 0xb7dc;
                    v12 = sub_14440(4,v68);
                    *(unsigned long *)&v29[-8] = 0xb7f2;
                    v11 = dcgettext(NULL,"removed %s\n",5,v29[-8]);
                    *(unsigned long *)&v29[-8] = 0xb804;
                    __printf_chk(1,v11,v12);
                  }
                  goto label_ada1;
                }
label_a58d:
                v72 = (void **)CONCAT71(v72._1_7_,v2);
label_a593:
                v64 = (void **)CONCAT44(v64._4_4_,v5);
                *(unsigned long *)&v29[-8] = 0xa5ab;
                v16 = (void **)sub_14440(4,v68);
                *(unsigned long *)&v29[-8] = 0xa5c1;
                v11 = dcgettext(NULL,"cannot create regular file %s",5,v29[-8]);
                v5 = (int)v64;
                goto label_a000;
              }
label_ada1:
              if (!*(long *)&v42[10]) {
label_a500:
                v8 = ~(unsigned int)v58 & (unsigned int)v53;
                if (((char)v63) && (!*(char *)((long)v42 + 0x1b)))
                  v8 |= 0x80;
                v59 = (void **)CONCAT44(v59._4_4_,v8);
                *(unsigned long *)&v29[-8] = 0xa548;
                v8 = sub_125a0(v69,v67,0xc1,v8);
                v40 = (unsigned long)v8;
                *(unsigned long *)&v29[-8] = 0xa54f;
                v60 = (void **)__errno_location(v29[-8]);
                v5 = *(int *)v60;
                v43 = v5 == 0x11 & (unsigned char)(v8 >> 0x1f);
                v72 = (void **)CONCAT71(v72._1_7_,v43);
                v8 = (unsigned int)v59;
                if (v43) {
                  v2 = (char)v42[6];
                  v5 = 0x11;
                  if (v2) goto label_a58d;
                  v57 = (void **)CONCAT44(v57._4_4_,0x11);
                  *(unsigned long *)&v29[-8] = 0xaa39;
                  v17 = readlinkat(v69,v67,&v28,1);
                  v5 = (int)v57;
                  v8 = (unsigned int)v59;
                  if (0 <= v17) {
                    if (!*(char *)((long)v42 + 0x3e)) {
                      v64 = (void **)((unsigned long)v64 & 0xffffffffffffff00);
                      *(unsigned long *)&v29[-8] = 0xb873;
                      v12 = sub_14440(4,v68);
                      *(unsigned long *)&v29[-8] = 0xb889;
                      v11 = dcgettext(NULL,"not writing through dangling symlink %s",5,v29[-8]);
                      v36 = NULL;
                      *(unsigned long *)&v29[-8] = 0xb89c;
                      error(0,0,v11,v12);
                      v37 = SUB81(v64,0);
                      goto label_9710;
                    }
                    v72 = (void **)CONCAT44(v72._4_4_,(unsigned int)v59);
                    *(unsigned long *)&v29[-8] = 0xaa7d;
                    v8 = sub_125a0(v69,v67,0x41,(unsigned long)v59 & 0xffffffff);
                    v40 = (unsigned long)v8;
                    v5 = *(int *)v60;
                    v8 = (unsigned int)v72;
                  }
                }
                v16 = v68;
                v43 = v5 == 0x15 & (unsigned char)(v40 >> 0x1f);
                v72 = (void **)CONCAT71(v72._1_7_,v43);
                if (v43) { // branch-flip
                  v5 = 0x15;
                  if (*(char *)v68) {
                    *(unsigned long *)&v29[-8] = 0xaac4;
                    v38 = strlen((char *)v68,v29[-8]);
                    v5 = (*(char *)((long)v16 + (v38 - 1)) != '/') + 0x14;
                  }
                }
                else {
                  v51 = (unsigned int *)CONCAT44(v51._4_4_,~(unsigned int)v53 & v8);
                  if (0 <= (int)v40) {
                    v72 = (void **)CONCAT71(v72._1_7_,1);
                    v46 = (unsigned int)v58;
                    goto label_954f;
                  }
                  v72 = (void **)CONCAT71(v72._1_7_,1);
                }
                goto label_a593;
              }
              *(unsigned long *)&v29[-8] = 0xadc9;
              v2 = sub_72a0(a0,v68,(unsigned long)v53 & 0xffffffff,1,v42);
              if (v2) goto label_a500;
              v72 = (void **)((unsigned long)v72 & 0xffffffffffffff00);
              v36 = NULL;
              v37 = 0;
            }
            goto label_9710;
          }
          v38 = 1;
          if (!*(char *)((long)v42 + 0x16))
            v38 = (unsigned long)(v42[2] == 3);
          *(unsigned long *)&v29[-0x10] = (unsigned long)v59 & 0xffffffff;
          *(unsigned long *)&v29[-0x18] = 0;
          *(unsigned long *)&v29[-0x20] = v38;
          *(unsigned long *)&v29[-0x28] = 0x8ea9;
          v4 = sub_65d0(a0,0xffffff9c,a0,v68,v69,v67);
          v2 = '\0';
          goto label_8eb2;
        }
        if (*(char *)a0 != '/') {
          *(unsigned long *)&v29[-8] = 0x7e9a;
          v11 = (char *)sub_e480(v67);
          if (v69 != -100) { // branch-flip
label_7ece:
            *(unsigned long *)&v29[-8] = 0x7eda;
            v37 = v29[-8];
            if (!stat(".",&v22,v37)) {
              *(unsigned long *)&v29[-8] = 0x9aff;
              v5 = fstatat(v69,v11,&v23,0,v29[-8]);
              if ((!v5) && ((v103 != v120 || (v22 != v23)))) {
                *(unsigned long *)&v29[-8] = 0x9b23;
                free(v11,v29[-8]);
                *(unsigned long *)&v29[-8] = 0x9b36;
                v12 = sub_147c0(0,3,v68);
                v11 = "%s: can make relative symbolic links only in current directory";
                goto label_8394;
              }
            }
          }
          else {
            v64 = (void **)0x1a14b;
            *(unsigned long *)&v29[-8] = 0x7ec3;
            v37 = v29[-8];
            if (strcmp(".",v11,v37)) goto label_7ece;
          }
          *(unsigned long *)&v29[-8] = 0x7eea;
          free(v11,v29[-8]);
        }
        v37 = *(char *)((long)v42 + 0x16);
        *(unsigned long *)&v29[-8] = 0x7f0a;
        v5 = sub_c000(a0,v69,v67,v37,0xffffffff);
        v2 = '\0';
        if (1 <= v5) {
          *(unsigned long *)&v29[-8] = 0x9772;
          v12 = sub_14310(1,4,a0);
          *(unsigned long *)&v29[-8] = 0x9788;
          v13 = sub_14310(0,4,v68);
          *(unsigned long *)&v29[-8] = 0x979e;
          v11 = dcgettext(NULL,"cannot create symbolic link %s to %s",5,v29[-8]);
          *(unsigned long *)&v29[-8] = 0x97b2;
          error(0,v5,v11,v13,v12);
          goto label_8658;
        }
        goto label_7f17;
      }
      v66 = (char *)(CONCAT44(v66._4_4_,v70) & 0xffffffff0000f000);
      if ((v70 & 0xf000) == 0x4000) {
        *(unsigned long *)&v29[-8] = 0x832b;
        v2 = sub_72a0(a0,v68,v70,v40,v42);
        if (!v2) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
        v58 = (char *)(CONCAT44(v58._4_4_,v8) & 0xffffffff00000012);
        v39 = v63;
label_8356:
        for (; v39; v39 = (unsigned long *)*v39) {
          if (((void *)v39[1] == v86) && ((void *)v39[2] == v20)) {
            *(unsigned long *)&v29[-8] = 0x8385;
            v12 = sub_14440(4,a0);
            v11 = "cannot copy cyclic symbolic link %s";
            goto label_8394;
          }
        }
        v30 = &v29[-0x20];
        v31 = &v29[-0x20];
        v32 = &v29[-0x20];
        *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
        v54 = (unsigned long *)((unsigned long)&v29[-0x11] & 0xfffffffffffffff0);
        *v54 = v63;
        v54[1] = v86;
        v54[2] = v20;
        if (((char)v72) || ((v99 & 0xf000) != 0x4000)) {
          *(unsigned long *)&v29[-0x28] = 0x8794;
          v5 = mkdirat(v69,v67,~(unsigned int)v58 & v8 & 0xfff);
          if (v5) { // branch-flip
            *(unsigned long *)&v29[-0x28] = 0x97e9;
            v12 = sub_14440(4,v68);
            v11 = "cannot create directory %s";
          }
          else {
            *(unsigned long *)&v29[-0x28] = 0x87ba;
            v5 = fstatat(v69,(char *)v67,&v21,0x100,v29[-0x28]);
            v8 = v99;
            if (!v5) {
              v52 = (void **)((unsigned long)v52 & 0xffffffffffffff00);
              if ((v99 & 0x1c0) != 0x1c0) {
                *(unsigned long *)&v29[-0x28] = 0x87fc;
                v5 = fchmodat(v69,v67,v99 | 0x1c0,0x100);
                v50 = v8;
                v52 = (void **)CONCAT71(v52._1_7_,1);
                if (v5) {
                  *(unsigned long *)&v29[-0x28] = 0xa4ec;
                  v12 = sub_14440(4,v68);
                  v11 = "setting permissions for %s";
                  v32 = &v29[-0x20];
                  goto label_8638;
                }
              }
              if (!*(char *)v64) {
                *(unsigned long *)&v29[-0x28] = 0x9f16;
                sub_bd30(v67,v97,v21);
                *(char *)v64 = '\x01';
              }
              if ((char)v42[0xf]) {
                if ((char)v42[6]) { // branch-flip
                  *(unsigned long *)&v29[-0x28] = 0x8844;
                  v9 = (unsigned long *)sub_14440(4,v68);
                  *(unsigned long *)&v29[-0x28] = 0x885a;
                  v11 = dcgettext(NULL,"created directory %s\n",5,v29[-0x28]);
                  *(unsigned long *)&v29[-0x28] = 0x886c;
                  __printf_chk(1,v11,v9);
                }
                else {
                  *(unsigned long *)&v29[-0x28] = 0xa199;
                  sub_6510(a0,v68,0);
                }
              }
              goto label_8870;
            }
            *(unsigned long *)&v29[-0x28] = 0xa034;
            v12 = sub_14440(4,v68);
            v11 = "cannot stat %s";
            v32 = &v29[-0x20];
          }
label_8638:
          *(unsigned long *)&v32[-8] = 0x863f;
          v11 = dcgettext(NULL,v11,5,v32[-8]);
          *(unsigned long *)&v32[-8] = 0x8647;
          v37 = v32[-8];
          v5 = *__errno_location(v37);
          *(unsigned long *)&v32[-8] = 0x8658;
          error(0,v5,v11,v12);
          goto label_8658;
        }
        if ((*(long *)&v42[10]) || (*(char *)((long)v42 + 0x33))) {
          *(unsigned long *)&v29[-0x28] = 0x9a8c;
          v2 = sub_7510(v68,0,v42);
          if ((!v2) && (v32 = &v29[-0x20], (char)v42[0xd])) goto label_8658;
        }
        v52 = (void **)((unsigned long)v52 & 0xffffffffffffff00);
        v58 = (char *)((unsigned long)v58 & 0xffffffff00000000);
label_8870:
        v43 = v60 != NULL & (unsigned char)v42[7];
        v38 = CONCAT71((undefined7)((unsigned long)v9 >> 8),v43);
        if ((!v43) || (*v60 == v20)) {
          v26 = *(long *)v42;
          v73 = *(unsigned long *)&v42[2];
          v74 = *(unsigned long *)&v42[4];
          v75 = *(unsigned long *)&v42[6];
          v82 = *(unsigned long *)&v42[0x14];
          v76 = *(unsigned long *)&v42[8];
          v77 = *(unsigned long *)&v42[10];
          v78 = *(unsigned long *)&v42[0xc];
          v79 = *(unsigned long *)&v42[0xe];
          v80 = *(unsigned long *)&v42[0x10];
          v81 = *(unsigned long *)&v42[0x12];
          *(unsigned long *)&v29[-0x28] = 0x88e0;
          v11 = (char *)sub_15300(a0,2);
          v16 = v61;
          v53 = v11;
          if (v11) { // branch-flip
            if (v42[1] == 3)
              v26 = CONCAT44(2,(unsigned int)v26);
            if (*v11) { // branch-flip
              v62 = (long)v67 - (long)v68;
              v57 = &v24;
              v60 = (void **)v25;
              v59 = (void **)&v25[1];
              v51 = v42;
              v63 = (unsigned long *)((unsigned long)v63._1_7_ << 8);
              v56 = &v26;
              v55 = (unsigned int *)CONCAT44(v55._4_4_,(int)v40);
              v66 = (char *)CONCAT71(v66._1_7_,1);
              v65 = a0;
              v61 = &v20;
              do {
                *(unsigned long *)&v29[-0x28] = 0x89dc;
                v36 = (void *)sub_ea10(v65,v11,0);
                *(unsigned long *)&v29[-0x28] = 0x89f0;
                v14 = (void *)sub_ea10(v68,v11,0);
                v43 = *(unsigned char *)v64;
                *(void ***)&v29[-0x28] = v57;
                *(void ***)&v29[-0x30] = v60;
                v25[1] = v43;
                *(void ***)&v29[-0x38] = v59;
                *(unsigned long *)&v29[-0x40] = 0;
                *(unsigned long **)&v29[-0x48] = v56;
                *(unsigned long **)&v29[-0x50] = v54;
                *(unsigned long *)&v29[-0x58] = 0x8a4d;
                v3 = sub_76f0(v36,v14,v69,(long)v14 + v62,(unsigned long)v55 & 0xffffffff,v61);
                v43 = v25[0];
                *(unsigned char *)v16 = *(unsigned char *)v16 | v25[0];
                v66 = (char *)CONCAT71(v66._1_7_,(unsigned char)v66 & v3);
                *(unsigned long *)&v29[-0x28] = 0x8a6a;
                free(v14,v29[-0x28]);
                *(unsigned long *)&v29[-0x28] = 0x8a72;
                free(v36,v29[-0x28]);
                if (v43) break;
                v63 = (unsigned long *)CONCAT71(v63._1_7_,(unsigned char)v63 | v25[1]);
                *(unsigned long *)&v29[-0x28] = 0x89bd;
                v37 = v29[-0x28];
                v11 = &v11[strlen(v11,v37) + 1];
              } while (*v11);
              v38 = (unsigned long)v66 & 0xff;
              v42 = v51;
              a0 = v65;
            }
            else {
              v63 = (unsigned long *)((unsigned long)v63._1_7_ << 8);
              v38 = 1;
            }
            *(unsigned long *)&v29[-0x28] = 0x8a9d;
            free(v53,v29[-0x28]);
            *(unsigned char *)v64 = (char)v63;
          }
          else {
            *(unsigned long *)&v29[-0x28] = 0xb9a7;
            v12 = sub_14440(4,a0);
            *(unsigned long *)&v29[-0x28] = 0xb9bd;
            v11 = dcgettext(NULL,"cannot access %s",5,v29[-0x28]);
            *(unsigned long *)&v29[-0x28] = 0xb9c5;
            v37 = v29[-0x28];
            v5 = *__errno_location(v37);
            v38 = 0;
            *(unsigned long *)&v29[-0x28] = 0xb9d9;
            error(0,v5,v11,v12);
          }
        }
        if ((char)v71) {
          v2 = '\0';
          v7 = 0;
          v43 = 0;
          v62 = CONCAT71(v62._1_7_,v52._0_1_);
          goto label_7f59;
        }
        v7 = 0;
        v62 = CONCAT71(v62._1_7_,v52._0_1_);
        goto label_7fd8;
      }
      *(unsigned long *)&v29[-8] = 0x7dfe;
      v2 = sub_72a0(a0,v68,v70,v40,v42);
      v58 = (char *)((unsigned long)v58 & 0xffffffff00000000);
      if (v2) goto label_7e7a;
      goto label_7cf0;
    }
    if (!*v42) {
      v16 = &v21;
      if (!v43) {
        v16 = &v23;
        *(unsigned long *)&v32[-8] = 0x91f1;
        v5 = fstatat(v69,(char *)v67,v16,0x100,v32[-8]);
        if (v5) goto label_789f;
      }
      if (((unsigned long)v16[3] & 0xf000) == 0xa000) {
        v12 = *(unsigned long *)&v42[0x12];
        *(unsigned long *)&v32[-8] = 0x7b26;
        v2 = sub_e820(v12,v67,v16);
        if (v2) {
          *(unsigned long *)&v32[-8] = 0x7b44;
          v12 = sub_14310(1,4,v68);
          *(unsigned long *)&v32[-8] = 0x7b56;
          v13 = sub_14310(0,4,a0);
          *(unsigned long *)&v32[-8] = 0x7b6c;
          v11 = dcgettext(NULL,"will not copy %s through just-created symlink %s",5,v32[-8]);
          *(unsigned long *)&v32[-8] = 0x7b80;
          error(0,0,v11,v13,v12);
          v38 = 0;
          return v38 & 0xffffffff;
        }
      }
      goto label_789f;
    }
    if ((char)v42[0xf]) goto label_8258;
    v29 = v32;
    if (!v8) goto label_7d98;
label_78ba:
    v29 = v32;
    if ((!(char)v42[0xe]) || ((v70 & 0xf000) != 0x4000)) {
      v2 = (char)v42[6];
      if (!v2) {
        if (!(char)v42[0xc]) goto label_7d98;
        if (!*(char *)((long)v42 + 0x17)) goto label_820e;
        v9 = NULL;
        goto label_8246;
      }
      if (v87 == 1) {
        *(unsigned long *)&v32[-8] = 0x9343;
        v9 = (unsigned long *)sub_bcd0(v86,v20);
label_8f3d:
        if (v9) {
          if ((v70 & 0xf000) == 0x4000) goto label_790d;
          *(unsigned long *)&v32[-0x10] = (unsigned long)v59 & 0xffffffff;
          *(unsigned long *)&v32[-0x18] = (unsigned long)(unsigned char)v42[0xf];
          *(unsigned long *)&v32[-0x20] = 1;
          *(unsigned long *)&v32[-0x28] = 0x8f90;
          v2 = sub_65d0(0,v69,v9,v68,v69,v67);
          if (v2) {
            v38 = 1;
            return v38 & 0xffffffff;
          }
          goto label_8f9c;
        }
        goto label_7970;
      }
      if (((char)v42[0xc]) && (!*(char *)((long)v42 + 0x17))) {
label_820e:
        if ((2 <= v87) || ((((char)v71 && (v42[1] == 3)) || (v9 = NULL, v42[1] == 4)))) {
          *(unsigned long *)&v32[-8] = 0x8f3a;
          v9 = (unsigned long *)sub_bd30(v67,v86,v20);
          goto label_8f3d;
        }
        goto label_823e;
      }
      v9 = NULL;
label_7a20:
      if (v8 == 0x11) {
        *(unsigned long *)&v32[-8] = 0x8dba;
        v5 = renameat(0xffffff9c,a0,v69,v67);
        if (v5) {
          *(unsigned long *)&v32[-8] = 0x8dc7;
          v37 = v32[-8];
          v8 = *__errno_location(v37);
          if (v8) goto label_7a2a;
        }
label_7d27:
        if ((char)v42[0xf]) {
          *(unsigned long *)&v32[-8] = 0x92a3;
          v11 = dcgettext(NULL,"renamed ",5,v32[-8]);
          *(unsigned long *)&v32[-8] = 0x92b2;
          __printf_chk(1,v11);
          *(unsigned long *)&v32[-8] = 0x92c8;
          sub_6510(a0,v68,v65);
        }
        if (*(long *)&v42[10]) {
          *(unsigned long *)&v32[-8] = 0x7d4d;
          sub_7510(v68,1,v42);
        }
        if (v66)
          *v66 = 1;
        if (((char)v71) && (!*(char *)((long)v42 + 0x3f))) {
          v12 = *(unsigned long *)&v42[0x12];
          *(unsigned long *)&v32[-8] = 0x921f;
          sub_e790(v12,v67,&v20);
          v38 = (unsigned long)v71 & 0xff;
          return v38 & 0xffffffff;
        }
        v38 = 1;
        return v38 & 0xffffffff;
      }
label_7a2a:
      if (v8 == 0x16) {
        *(unsigned long *)&v32[-8] = 0x93ae;
        v12 = sub_14310(1,4,dat_23568);
        *(unsigned long *)&v32[-8] = 0x93c4;
        v13 = sub_14310(0,4,dat_23570);
        *(unsigned long *)&v32[-8] = 0x93da;
        v11 = dcgettext(NULL,"cannot move %s to a subdirectory of itself, %s",5,v32[-8]);
        v38 = 1;
        *(unsigned long *)&v32[-8] = 0x93f4;
        error(0,0,v11,v13,v12);
        *(unsigned char *)v61 = 1;
        return v38 & 0xffffffff;
      }
      if (v8 != 0x12) {
        *(unsigned long *)&v32[-8] = 0x8e06;
        v12 = sub_14310(1,4,v68);
        *(unsigned long *)&v32[-8] = 0x8e18;
        v13 = sub_14310(0,4,a0);
        *(unsigned long *)&v32[-8] = 0x8e2e;
        v11 = dcgettext(NULL,"cannot move %s to %s",5,v32[-8]);
label_8e3a:
        *(unsigned long *)&v32[-8] = 0x8e43;
        error(0,v8,v11,v13,v12);
        *(unsigned long *)&v32[-8] = 0x8e56;
        sub_bc60(v86,v20);
        v38 = 0;
        return v38 & 0xffffffff;
      }
      v8 = v70 & 0xf000;
      *(unsigned long *)&v32[-8] = 0x7a6a;
      v5 = unlinkat(v69,(char *)v67,(unsigned int)(v8 == 0x4000) << 9,v32[-8]);
      if (v5) {
        *(unsigned long *)&v32[-8] = 0x7a73;
        v10 = (unsigned int *)__errno_location(v32[-8]);
        if (*v10 != 2) {
          *(unsigned long *)&v32[-8] = 0x9246;
          v12 = sub_14310(1,4,v68);
          *(unsigned long *)&v32[-8] = 0x9258;
          v13 = sub_14310(0,4,a0);
          *(unsigned long *)&v32[-8] = 0x926e;
          v11 = dcgettext(NULL,"inter-device move failed: %s to %s; unable to remove target",5,v32[-8]);
          v8 = *v10;
          goto label_8e3a;
        }
      }
      v43 = v8 != 0x4000 & (unsigned char)v42[0xf];
      v72 = (void **)CONCAT71(v72._1_7_,v43);
      if (v43) { // branch-flip
        v40 = 1;
        *(unsigned long *)&v32[-8] = 0x9369;
        v11 = dcgettext(NULL,"copied ",5,v32[-8]);
        *(unsigned long *)&v32[-8] = 0x9378;
        __printf_chk(1,v11);
        *(unsigned long *)&v32[-8] = 0x938e;
        sub_6510(a0,v68,v65);
      }
      else {
        v72 = (void **)CONCAT71(v72._1_7_,1);
        v40 = 1;
      }
      goto label_7da3;
    }
    if ((char)v71) { // branch-flip
      *(unsigned long *)&v32[-8] = 0x7905;
      v9 = (unsigned long *)sub_bd30(v67,v86,v20);
    }
    else {
      *(unsigned long *)&v32[-8] = 0x8f18;
      v9 = (unsigned long *)sub_bcd0(v86,v20);
    }
    if (!v9) {
label_7970:
      v2 = (char)v42[6];
label_823e:
      if (v2) goto label_7a20;
label_8246:
      v40 = (unsigned long)v72 & 0xff;
      goto label_7da3;
    }
label_790d:
    *(unsigned long *)&v32[-8] = 0x7923;
    v2 = sub_14e20(0xffffff9c,a0,v69,v9);
    if (v2) { // branch-flip
      *(unsigned long *)&v32[-8] = 0x9dc1;
      v12 = sub_14310(1,4,dat_23568);
      *(unsigned long *)&v32[-8] = 0x9dd7;
      v13 = sub_14310(0,4,dat_23570);
      *(unsigned long *)&v32[-8] = 0x9ded;
      v11 = dcgettext(NULL,"cannot copy a directory, %s, into itself, %s",5,v32[-8]);
      *(unsigned long *)&v32[-8] = 0x9e01;
      error(0,0,v11,v13,v12);
      *(unsigned char *)v61 = 1;
    }
    else {
      *(unsigned long *)&v32[-8] = 0x7942;
      v2 = sub_14e20(v69,v67,v69,v9);
      v16 = v68;
      if (v2) {
        *(unsigned long *)&v32[-8] = 0xa3e9;
        v12 = sub_14440(4,dat_23570);
        *(unsigned long *)&v32[-8] = 0xa3ff;
        v11 = dcgettext(NULL,"warning: source directory %s specified more than once",5,v32[-8]);
        *(unsigned long *)&v32[-8] = 42000;
        error(0,0,v11,v12);
        if ((char)v42[6]) goto label_91b0;
        v38 = 1;
        return v38 & 0xffffffff;
      }
      if ((v42[1] == 4) || ((v42[1] == 3 && ((char)v71)))) goto label_7970;
      *(unsigned long *)&v32[-8] = 0xa1b7;
      v36 = (void *)sub_6110(v68,v67,v9);
      *(unsigned long *)&v32[-8] = 0xa1cc;
      v12 = sub_14310(1,4,v36);
      *(unsigned long *)&v32[-8] = 0xa1de;
      v13 = sub_14310(0,4,v16);
      *(unsigned long *)&v32[-8] = 0xa1f4;
      v11 = dcgettext(NULL,"will not create hard link %s to directory %s",5,v32[-8]);
      *(unsigned long *)&v32[-8] = 0xa208;
      error(0,0,v11,v13,v12);
      *(unsigned long *)&v32[-8] = 0xa210;
      free(v36,v32[-8]);
    }
label_8f9c:
    if (*(char *)((long)v42 + 0x33)) {
      *(unsigned long *)&v32[-8] = 0x8fae;
      if (setfscreatecon(0)) goto label_8bf7;
    }
    goto label_866c;
  }
  v72 = (void **)CONCAT71(v72._1_7_,0 < (int)a4);
  if (v8) goto label_779d;
label_79ae:
  if (*(char *)((long)v42 + 0x3f)) {
    v8 = 0;
    goto label_7803;
  }
  v8 = 0;
  v35 = v67;
  v16 = v68;
  v5 = v69;
  v70 = a4;
label_77b2:
  v5 = fstatat(v5,(char *)v35,&v20,(unsigned int)(v42[1] == 2) << 8);
  a4 = v70;
  if (!v5) {
    v70 = v88;
    if (((v88 & 0xf000) != 0x4000) || ((char)v42[0xe])) goto label_7803;
    v12 = sub_14440(4,a0);
    v38 = 0;
    if (*(char *)((long)v42 + 0x19)) {
      v11 = "omitting directory %s";
      v11 = dcgettext(NULL,v11,5);
      error(0,0,v11,v12);
      return v38 & 0xffffffff;
    }
    v11 = dcgettext(NULL,"-r not specified; omitting directory %s",5);
    error(0,0,v11,v12);
    return v38 & 0xffffffff;
  }
label_8528:
  *(unsigned long *)&v31[-8] = 0x8532;
  v12 = sub_14440(4,v16);
  v11 = "cannot stat %s";
  v32 = v31;
label_8541:
  *(unsigned long *)&v32[-8] = 0x8548;
  v11 = dcgettext(NULL,v11,5,v32[-8]);
  *(unsigned long *)&v32[-8] = 0x8550;
  v37 = v32[-8];
  v5 = *__errno_location(v37);
  *(unsigned long *)&v32[-8] = 0x8561;
  error(0,v5,v11,v12);
  goto label_7cf0;
label_b3e3:
  *(unsigned long *)&v29[-8] = 0xb3e8;
  v37 = v29[-8];
  v9 = v56;
  v42 = v55;
  if (*__errno_location(v37) != 6) { // branch-flip
label_b521:
    v9 = v56;
    v42 = v55;
    *(unsigned long *)&v29[-8] = 0xb52e;
    v11 = "cannot lseek %s";
    v60 = (void **)sub_14440(4,a0);
label_b541:
    *(unsigned long *)&v29[-8] = 0xb548;
    v37 = v29[-8];
    v64 = (void **)dcgettext(NULL,v11,5,v37);
    *(unsigned long *)&v29[-8] = 0xb554;
    v37 = v29[-8];
    v5 = *__errno_location(v37);
    *(unsigned long *)&v29[-8] = 0xb56d;
    error(0,v5,(char *)v64,v60);
label_b56d:
    v2 = '\0';
  }
  else {
label_b3ff:
    v44 = (long)v52 < (long)v60;
    if (((long)v52 < (long)v60) || ((char)v54)) {
label_b5a2:
      if (v49 != 1) { // branch-flip
        v59 = (void **)CONCAT71(v59._1_7_,v44);
        *(unsigned long *)&v29[-8] = 0xb5c3;
        v5 = ftruncate(v40,v60);
        if (v5) {
label_b908:
          *(unsigned long *)&v29[-8] = 0xb919;
          v60 = (void **)sub_14440(4,v68);
          v11 = "failed to extend %s";
          goto label_b541;
        }
        if ((v49 == 3) && ((char)v59)) {
          *(unsigned long *)&v29[-8] = 0xb604;
          v5 = fallocate(v40,3,v52,(long)v60 - (long)v52);
          if (v5 <= -1) {
            *(unsigned long *)&v29[-8] = 0xb611;
            v16 = (void **)__errno_location(v29[-8]);
            v5 = *(int *)v16;
            if (v5 != 0x5f && v5 != 0x26) {
              v60 = (void **)CONCAT71(v60._1_7_,v5 == 0x5f || v5 == 0x26);
              v59 = v16;
              *(unsigned long *)&v29[-8] = 0xb64b;
              v64 = (void **)sub_14440(4,v68);
              *(unsigned long *)&v29[-8] = 0xb665;
              v11 = dcgettext(NULL,"error deallocating %s",5,v29[-8]);
              v5 = *(int *)v59;
              *(unsigned long *)&v29[-8] = 0xb682;
              error(0,v5,v11,v64);
              v2 = (char)v60;
              goto label_ac42;
            }
          }
        }
      }
      else {
        v17 = (long)v60 - (long)v52;
        *(unsigned long *)&v29[-8] = 0xb900;
        if (!sub_6170(v40,v17)) goto label_b908;
      }
    }
    v2 = (char)v64;
  }
label_ac42:
  if (v2 != '\x01') { // branch-flip
label_96e2:
    v43 = 0;
  }
  else {
    if (v25[0]) {
      *(unsigned long *)&v29[-8] = 0xb107;
      v5 = ftruncate(v40,v26);
      if (v5 <= -1) {
        *(unsigned long *)&v29[-8] = 0xb120;
        v63 = (unsigned long *)sub_14440(4,v68);
        v11 = "failed to extend %s";
label_b133:
        *(unsigned long *)&v29[-8] = 0xb13a;
        v37 = v29[-8];
        v64 = (void **)dcgettext(NULL,v11,5,v37);
        *(unsigned long *)&v29[-8] = 0xb146;
        v37 = v29[-8];
        v5 = *__errno_location(v37);
        *(unsigned long *)&v29[-8] = 0xb15f;
        error(0,v5,(char *)v64,v63);
        goto label_96e2;
      }
    }
label_ac58:
    v64 = (void **)CONCAT44(v64._4_4_,v51._0_4_);
    if (*(char *)((long)v42 + 0x1f)) {
label_a676:
      v27 = v93;
      v83 = v94;
      v84 = v95;
      v85 = v96;
      *(unsigned long *)&v29[-8] = 0xa6cc;
      v5 = sub_e6a0(v40,v69,v67,&v27,0);
      if (v5) {
        *(unsigned long *)&v29[-8] = 0xb175;
        v59 = (void **)sub_14440(4,v68);
        *(unsigned long *)&v29[-8] = 0xb18f;
        v37 = v29[-8];
        v60 = (void **)dcgettext(NULL,"preserving times for %s",5,v37);
        *(unsigned long *)&v29[-8] = 0xb19b;
        v37 = v29[-8];
        v5 = *__errno_location(v37);
        *(unsigned long *)&v29[-8] = 0xb1b4;
        error(0,v5,(char *)v60,v59);
        if (*(char *)((long)v42 + 0x32)) goto label_96e2;
      }
    }
    if (*(char *)((long)v42 + 0x1d)) {
      if ((v89 != v105._4_4_) || (v90 != (unsigned int)v106)) {
        *(void ***)&v29[-8] = &v22;
        *(unsigned long *)&v29[-0x10] = (unsigned long)v72 & 0xff;
        *(unsigned long *)&v29[-0x18] = (unsigned long)v90;
        *(unsigned long *)&v29[-0x20] = (unsigned long)v89;
        *(unsigned long *)&v29[-0x28] = 0xa72f;
        v5 = sub_7080(v42,v68,v69,v67,v40,v88);
        if (v5 == -1) goto label_96e2;
        if (!v5)
          v61 = (void **)((unsigned long)v61 & 0xfffffffffffff1ff);
      }
    }
label_a74a:
    if ((char)v63) { // branch-flip
      *(unsigned long *)&v29[-8] = 0xaec7;
      v2 = sub_6220(a0,v38,v68,v40,v42);
      if (v2) goto label_a757;
      v43 = *(unsigned char *)((long)v42 + 0x36) ^ 1;
    }
    else {
label_a757:
      v43 = 1;
    }
    if (*(unsigned long *)&v42[6] & 0xff0000000000ff) { // branch-flip
      v64 = (void **)CONCAT71(v64._1_7_,v43);
      *(unsigned long *)&v29[-8] = 0xae90;
      v5 = sub_c830(a0,v38,v68,v40,(unsigned long)v61 & 0xffffffff);
      v43 = (unsigned char)v64;
      if ((v5) && (*(char *)((long)v42 + 0x32)))
        v43 = 0;
    }
    else if (*(char *)((long)v42 + 0x39)) { // branch-flip
      v6 = v42[4];
      v64 = (void **)CONCAT71(v64._1_7_,v43);
      *(unsigned long *)&v29[-8] = 0xb041;
      v5 = sub_c8e0(v68,v40,v6);
      v43 = (unsigned char)v64;
      if (v5)
        v43 = 0;
    }
    else {
      v3 = (unsigned char)v72 & (unsigned char)v42[8];
      v63 = (unsigned long *)CONCAT71(v63._1_7_,v3);
      if (v3) { // branch-flip
        if (dat_23010 == 0xffffffff) {
          v72 = (void **)CONCAT71(v72._1_7_,v43);
          *(unsigned long *)&v29[-8] = 0xb817;
          dat_23010 = umask(0,v29[-8]);
          *(unsigned long *)&v29[-8] = 0xb824;
          umask(dat_23010,v29[-8]);
          v43 = (unsigned char)v72;
        }
        v64 = (void **)CONCAT71(v64._1_7_,v43);
        *(unsigned long *)&v29[-8] = 0xb7a5;
        v5 = sub_c8e0(v68,v40,~dat_23010 & 0x1b6);
        v43 = (unsigned char)v64;
        v72 = (void **)CONCAT71(v72._1_7_,v63._0_1_);
        if (v5)
          v43 = 0;
      }
      else if ((int)v64 || v46) {
        if (dat_23010 != 0xffffffff) { // branch-flip
          v6 = dat_23010;
          if (~dat_23010 & v46 || (int)v64) {
label_a7ca:
            v64 = (void **)CONCAT71(v64._1_7_,v43);
            *(unsigned long *)&v29[-8] = 0xa7e2;
            v5 = fchmod(v8,(unsigned int)v53 & ~v6,v29[-8]);
            v43 = (unsigned char)v64;
            if (v5) {
              v61 = (void **)CONCAT71(v61._1_7_,(unsigned char)v64);
              *(unsigned long *)&v29[-8] = 0xa80a;
              v63 = (unsigned long *)sub_14440(4,v68);
              *(unsigned long *)&v29[-8] = 0xa824;
              v37 = v29[-8];
              v64 = (void **)dcgettext(NULL,"preserving permissions for %s",5,v37);
              *(unsigned long *)&v29[-8] = 0xa830;
              v37 = v29[-8];
              v5 = *__errno_location(v37);
              *(unsigned long *)&v29[-8] = 0xa849;
              error(0,v5,(char *)v64,v63);
              v43 = (unsigned char)v61;
              if (*(char *)((long)v42 + 0x32))
                v43 = 0;
            }
          }
        }
        else {
          v61 = (void **)CONCAT71(v61._1_7_,v43);
          *(unsigned long *)&v29[-8] = 0xb9ec;
          dat_23010 = umask(0,v29[-8]);
          v63 = (unsigned long *)CONCAT44(v63._4_4_,dat_23010);
          *(unsigned long *)&v29[-8] = 0xb9ff;
          umask(dat_23010,v29[-8]);
          v43 = (unsigned char)v61;
          if (~(unsigned int)v63 & v46 || (int)v64) {
            v6 = dat_23010;
            if ((unsigned int)v63 == 0xffffffff) {
              v64 = (void **)CONCAT71(v64._1_7_,(unsigned char)v61);
              *(unsigned long *)&v29[-8] = 0xba40;
              dat_23010 = umask(0,v29[-8]);
              *(unsigned long *)&v29[-8] = 0xba4d;
              umask(dat_23010,v29[-8]);
              v43 = (unsigned char)v64;
              v6 = dat_23010;
            }
            goto label_a7ca;
          }
        }
      }
    }
  }
label_96e5:
  v64 = (void **)CONCAT71(v64._1_7_,v43);
  *(unsigned long *)&v29[-8] = 0x96f3;
  v37 = v29[-8];
  if (0 <= close(v8,v37)) { // branch-flip
    v37 = SUB81(v64,0);
    v36 = v24;
  }
  else {
    *(unsigned long *)&v29[-8] = 0xadf3;
    v64 = (void **)sub_14440(4,v68);
    *(unsigned long *)&v29[-8] = 0xae0d;
    v11 = dcgettext(NULL,"failed to close %s",5,v29[-8]);
    *(unsigned long *)&v29[-8] = 0xae15;
    v37 = v29[-8];
    v5 = *__errno_location(v37);
    *(unsigned long *)&v29[-8] = 0xae2a;
    error(0,v5,v11,v64);
    v37 = 0;
    v36 = v24;
  }
label_9710:
  v64 = (void **)CONCAT71(v64._1_7_,v37);
  *(unsigned long *)&v29[-8] = 0x971f;
  v37 = v29[-8];
  if (0 <= close(v7,v37)) { // branch-flip
    v7 = 0;
    *(unsigned long *)&v29[-8] = 0x9741;
    free(v36,v29[-8]);
    v2 = (char)v64;
    v4 = (char)v64;
label_8eb2:
    if (!v4) goto label_8658;
label_7f17:
    v43 = (int)v66 != 0x4000;
    if (!(char)v72) {
      v3 = ((unsigned char)v42[5] ^ 1) & v43;
      if (v3) {
        if ((*(long *)&v42[10]) || (*(char *)((long)v42 + 0x33))) {
          v66 = (char *)CONCAT71(v66._1_7_,v2);
          v64 = (void **)CONCAT71(v64._1_7_,v3);
          *(unsigned long *)&v29[-8] = 0x8fe9;
          v43 = sub_7510(v68,0,v42);
          v2 = (char)v66;
          if (!v43) {
            v72 = (void **)CONCAT71(v72._1_7_,(char)v42[0xd]);
            if ((char)v42[0xd]) goto label_8658;
            v43 = (unsigned char)v64;
          }
        }
        else {
          v43 = v3;
        }
      }
    }
    v38 = 1;
    v31 = v29;
    if ((char)v71) {
      v62 &= 0xffffffffffffff00;
      v38 = (unsigned long)v71 & 0xffffffff;
      v30 = v29;
label_7f59:
      v31 = v30;
      if (*(long *)&v42[0x12]) {
        v66 = (char *)CONCAT71(v66._1_7_,v2);
        v71 = &v23;
        *(unsigned long *)&v30[-8] = 0x7f8c;
        v5 = fstatat(v69,(char *)v67,&v23,0x100,v30[-8]);
        v16 = v71;
        v2 = (char)v66;
        if (!v5) {
          v12 = *(unsigned long *)&v42[0x12];
          v71 = (void **)CONCAT71(v71._1_7_,(char)v66);
          *(unsigned long *)&v30[-8] = 0x7fb6;
          sub_e790(v12,v67,v16);
          v2 = (char)v71;
        }
      }
    }
    if (((*(char *)((long)v42 + 0x17)) && (v43)) || (v2)) {
      return v38 & 0xffffffff;
    }
label_7fd8:
    if (*(char *)((long)v42 + 0x1f)) {
      v23 = v93;
      v120 = v94;
      v121 = v95;
      v122 = v96;
      *(unsigned long *)&v31[-8] = 0x803b;
      v5 = utimensat(v69,v67,&v23,(v7 & 0xff) << 8);
      if (v5) {
        *(unsigned long *)&v31[-8] = 0x8050;
        v71 = (void **)sub_14440(4,v68);
        *(unsigned long *)&v31[-8] = 0x806a;
        v11 = dcgettext(NULL,"preserving times for %s",5,v31[-8]);
        *(unsigned long *)&v31[-8] = 0x8072;
        v37 = v31[-8];
        v5 = *__errno_location(v37);
        *(unsigned long *)&v31[-8] = 0x8087;
        error(0,v5,v11,v71);
        if (*(char *)((long)v42 + 0x32)) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
      }
    }
    if ((char)v7) { // branch-flip
      if (!*(char *)((long)v42 + 0x35)) {
        return v38 & 0xffffffff;
      }
      *(unsigned long *)&v31[-8] = 0x8ef7;
      if (sub_6220(a0,0xffffffff,v68,0xffffffff,v42)) {
        return v38 & 0xffffffff;
      }
      v2 = *(char *)((long)v42 + 0x36);
    }
    else {
      if (*(char *)((long)v42 + 0x1d)) {
        if ((((char)v72) || (v100 != v89)) || (v101 != v90)) {
          *(void ***)&v31[-8] = &v21;
          *(unsigned long *)&v31[-0x10] = (unsigned long)v72 & 0xff;
          *(unsigned long *)&v31[-0x18] = (unsigned long)v90;
          *(unsigned long *)&v31[-0x20] = (unsigned long)v89;
          *(unsigned long *)&v31[-0x28] = 0x8104;
          v5 = sub_7080(v42,v68,v69,v67,0xffffffff,v88);
          if (v5 == -1) {
            v38 = 0;
            return v38 & 0xffffffff;
          }
          if (!v5)
            v70 &= 0xfffff1ff;
        }
      }
      if (*(char *)((long)v42 + 0x35)) {
        *(unsigned long *)&v31[-8] = 0xa094;
        v2 = sub_6220(a0,0xffffffff,v68,0xffffffff,v42);
        if ((!v2) && (*(char *)((long)v42 + 0x36))) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
      }
      if (!(*(unsigned long *)&v42[6] & 0xff0000000000ff)) {
        if (*(char *)((long)v42 + 0x39)) // branch-flip
          v8 = v42[4];
        else {
          if (!((unsigned char)v72 & (unsigned char)v42[8])) {
            if ((int)v58) { // branch-flip
              v8 = dat_23010;
              if (dat_23010 == 0xffffffff) {
                *(unsigned long *)&v31[-8] = 0xad45;
                v8 = umask(0,v31[-8]);
                *(unsigned long *)&v31[-8] = 0xad54;
                dat_23010 = v8;
                umask(v8,v31[-8]);
              }
              v8 = (unsigned int)v58 & ~v8;
              v58 = (char *)CONCAT44(v58._4_4_,v8);
              if (!v8) goto label_8169;
              if ((char)v62 != '\x01') {
                if ((char)v72) {
                  *(unsigned long *)&v31[-8] = 0x8513;
                  v5 = fstatat(v69,(char *)v67,&v21,0x100,v31[-8]);
                  if (v5) {
                    v16 = v68;
                    goto label_8528;
                  }
                }
                v50 = v99;
                if (!((unsigned int)v58 & ~v99)) {
                  return v38 & 0xffffffff;
                }
              }
            }
            else {
label_8169:
              if (!(char)v62) {
                return v38 & 0xffffffff;
              }
            }
            *(unsigned long *)&v31[-8] = 0x8199;
            v5 = fchmodat(v69,v67,v50 | (unsigned int)v58,0x100);
            if (!v5) {
              return v38 & 0xffffffff;
            }
            *(unsigned long *)&v31[-8] = 0x81b2;
            v12 = sub_14440(4,v68);
            *(unsigned long *)&v31[-8] = 0x81c8;
            v11 = dcgettext(NULL,"preserving permissions for %s",5,v31[-8]);
            *(unsigned long *)&v31[-8] = 0x81d0;
            v37 = v31[-8];
            v5 = *__errno_location(v37);
            *(unsigned long *)&v31[-8] = 0x81e1;
            error(0,v5,v11,v12);
            goto label_81e1;
          }
          v8 = 0x1ff;
          if ((v70 & 0x7000) != 0x4000)
            v8 = 0x1b6;
          v7 = dat_23010;
          if (dat_23010 == 0xffffffff) {
            *(unsigned long *)&v31[-8] = 0xad2a;
            v7 = umask(0,v31[-8]);
            *(unsigned long *)&v31[-8] = 0xad39;
            dat_23010 = v7;
            umask(v7,v31[-8]);
          }
          v8 = ~v7 & v8;
        }
        *(unsigned long *)&v31[-8] = 0x9eef;
        if (!sub_c8e0(v68,0xffffffff,v8)) {
          return v38 & 0xffffffff;
        }
        v38 = 0;
        return v38 & 0xffffffff;
      }
      *(unsigned long *)&v31[-8] = 0x9b70;
      if (!sub_c830(a0,0xffffffff,v68,0xffffffff,v70)) {
        return v38 & 0xffffffff;
      }
label_81e1:
      v2 = *(char *)((long)v42 + 0x32);
    }
    if (!v2) {
      return v38 & 0xffffffff;
    }
  }
  else {
    *(unsigned long *)&v29[-8] = 0xa45f;
    v12 = sub_14440(4,a0);
    *(unsigned long *)&v29[-8] = 0xa475;
    v11 = dcgettext(NULL,"failed to close %s",5,v29[-8]);
    *(unsigned long *)&v29[-8] = 0xa47d;
    v37 = v29[-8];
    v5 = *__errno_location(v37);
    *(unsigned long *)&v29[-8] = 0xa48e;
    error(0,v5,v11,v12);
    *(unsigned long *)&v29[-8] = 0xa496;
    free(v36,v29[-8]);
label_8658:
    if (*(char *)((long)v42 + 0x33)) {
      *(unsigned long *)&v32[-8] = 0x92d7;
      if (setfscreatecon(0)) {
label_8bf7:
        *(unsigned long *)&v32[-8] = 0x8bfc; // no-return
        sub_6770();
      }
    }
    if (!v9) {
      *(unsigned long *)&v32[-8] = 0x9323;
      sub_bc60(v86,v20);
    }
label_866c:
    if (v65) {
      *(unsigned long *)&v32[-8] = 0x869d;
      v5 = renameat(v69,(long)v65 + ((long)v67 - (long)v68));
      if (v5) {
        *(unsigned long *)&v32[-8] = 0x92f9;
        v12 = sub_14440(4,v68);
        v11 = "cannot un-backup %s";
        goto label_8541;
      }
      if ((char)v42[0xf]) {
        *(unsigned long *)&v32[-8] = 0x86c6;
        v12 = sub_14310(1,4,v68);
        *(unsigned long *)&v32[-8] = 0x86dc;
        v13 = sub_14310(0,4,v65);
        *(unsigned long *)&v32[-8] = 0x86f2;
        v11 = dcgettext(NULL,"%s -> %s (unbackup)\n",5,v32[-8]);
        *(unsigned long *)&v32[-8] = 0x8707;
        __printf_chk(1,v11,v13,v12);
      }
    }
  }
  v38 = 0;
  return v38 & 0xffffffff;
}

// Function: sub_ba80 @ 0xba80
void sub_ba80(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long a3,unsigned long a4,unsigned int *a5,unsigned long a6,unsigned long a7)
{
  char v1; // stack - 0x11
  
  if (4 <= *a5)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (3 <= a5[3] - 1)
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (2 < a5[0x11])
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((*(char *)((long)a5 + 0x17)) && (*(char *)((long)a5 + 0x3a)))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a5[3] != 2) && (a5[0x11] == 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  v1 = 0;
  dat_23568 = a1;
  dat_23570 = a0;
  sub_76f0(a0,a1,a2,a3,a4,0,0,a5,1,&v1,a6,a7);
}

// Function: sub_bbd0 @ 0xbbd0
unsigned int sub_bbd0(void)
{
  unsigned int v1; // eax
  
  if (dat_23010 != 0xffffffff)
    return dat_23010;
  v1 = umask(0);
  dat_23010 = v1;
  umask(v1);
  return v1;
}

// Function: sub_bc10 @ 0xbc10
unsigned long sub_bc10(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_bc20 @ 0xbc20
unsigned long sub_bc20(long *a0,long *a1)
{
  if (*a0 != *a1)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1[1] >> 8),a0[1] == a1[1]);
}

// Function: sub_bc40 @ 0xbc40
void sub_bc40(void *a0)
{
  free(*(void **)((long)a0 + 0x10));
  free(a0); // tail-call
}

// Function: sub_bc60 @ 0xbc60
void sub_bc60(unsigned long a0,unsigned long a1) // return-dupe
{
  void *v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = (void *)sub_12270(dat_23578,&v2);
  if (!v1)
    return;
  free(*(void **)((long)v1 + 0x10));
  free(v1);
}

// Function: sub_bcd0 @ 0xbcd0
unsigned long sub_bcd0(unsigned long a0,unsigned long a1) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  
  v2 = a0;
  v3 = a1;
  v1 = sub_11920(dat_23578,&v2);
  if (!v1)
    return 0;
  return *(unsigned long *)(v1 + 0x10);
}

// Function: sub_bd30 @ 0xbd30
unsigned long sub_bd30(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  unsigned long *v3; // rax
  
  v1 = (unsigned long *)sub_16c50(0x18);
  v2 = sub_17130(a0);
  *v1 = a1;
  v1[2] = v2;
  v1[1] = a2;
  v3 = (unsigned long *)sub_12210(dat_23578,v1);
  if (!v3)
    sub_17150(); // no-return
  if (v1 == v3)
    return 0;
  free((void *)v1[2]);
  free(v1);
  return v3[2];
}

// Function: sub_bdb0 @ 0xbdb0
void sub_bdb0(void)
{
  dat_23578 = sub_11bb0(0x67,0,sub_bc10,sub_bc20,sub_bc40);
  if (dat_23578)
    return;
  sub_17150(); // no-return
}

// Function: sub_bdf0 @ 0xbdf0
void sub_bdf0(unsigned long a0,unsigned int *a1)
{
  linkat(*a1,*(unsigned long *)&a1[2],a1[4],a0,a1[5]); // tail-call
}

// Function: sub_be10 @ 0xbe10
void * sub_be10(void *a0,void *a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3; // rax
  
  v2 = sub_d970(a0) - (long)a0;
  v1 = v2 + 9;
  if ((0x101 <= v1) && (a1 = malloc(v1), !a1))
    return NULL;
  v3 = mempcpy(a1,a0,v2);
  *(unsigned long *)v3 = s_1c458._0_8_;
  v3[8] = s_1c458[8];
  return a1;
}

// Function: sub_be70 @ 0xbe70
void sub_be70(unsigned long a0,unsigned long *a1)
{
  symlinkat(*a1,*(unsigned int *)&a1[1],a0); // tail-call
}

// Function: sub_be90 @ 0xbe90
int sub_be90(unsigned long a0,unsigned long a1,int a2,unsigned long a3,unsigned int a4,char a5,int a6)
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
    v3 = (char *)sub_be10(a3,v4);
    v1 = (char *)v3;
    if (v3) { // branch-flip
      v5[0] = (unsigned int)a0;
      v7 = a1;
      v8 = a2;
      v9 = a4;
      v2 = sub_157b0(v3,0,v5,sub_bdf0,6);
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

// Function: sub_c000 @ 0xc000
int sub_c000(unsigned long a0,int a1,unsigned long a2,char a3,int a4)
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
    v1 = (char *)sub_be10(a2,v3);
    if (v1) { // branch-flip
      v2 = a0;
      v4 = a1;
      if (sub_157b0(v1,0,&v2,sub_be70,6)) // branch-flip
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

// Function: sub_c140 @ 0xc140
int sub_c140(long a0,char *a1)
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
      if (0 <= lstat(a1,v9)) goto label_c1bd;
      v2 = *v4;
      v6 = 0;
      v5 = 0;
    }
label_c28b:
    v3 = v1;
    goto label_c28e;
  }
  if (0 <= fstat(v1,v9)) { // branch-flip
label_c1bd:
    if (0 <= (int)selabel_lookup(a0,&v8,a1,v11)) { // branch-flip
      v5 = context_new(v8);
      if (v5) { // branch-flip
        if (v1 != -1) {
          if ((int)sub_15610(v1) < 0) {
            v6 = 0;
            v3 = -1;
            goto label_c3ab;
          }
          v6 = context_new(v10);
          if (v6) goto label_c221;
          v2 = *v4;
          v3 = -1;
          goto label_c24f;
        }
        if ((int)sub_15590(a1,&v10) < 0) {
          v2 = *v4;
          v6 = 0;
          goto label_c28b;
        }
        v6 = context_new(v10);
        if (!v6) {
          v2 = *v4;
          v3 = -1;
          goto label_c28e;
        }
label_c221:
        v7 = context_type_get(v5);
        if (((v7) && (!context_type_set(v6,v7))) && (v7 = context_str(v6), v7)) {
          if (v1 == -1) {
            v1 = lsetfilecon(a1,v7);
            v2 = *v4;
            goto label_c28b;
          }
          v3 = fsetfilecon(v1);
          goto label_c3ab;
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
    if (v1 == -1) goto label_c28e;
  }
  else {
    v6 = 0;
    v5 = 0;
    v3 = -1;
label_c3ab:
    v2 = *v4;
  }
label_c24f:
  close(v1);
label_c28e:
  context_free(v5);
  context_free(v6);
  freecon(v8);
  freecon(v10);
  *v4 = v2;
  return v3;
}

// Function: sub_c450 @ 0xc450
unsigned int sub_c450(unsigned long a0,char *a1,unsigned int a2)
{
  short v1; // ax
  unsigned long v10; // stack - 0x50
  unsigned long v11; // stack - 0x48
  unsigned long v12; // stack - 0x58
  char *v13;
  int v14;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  void *v5; // rax
  long v6;
  long v7; // rax
  long v8; // rax
  unsigned long v9; // stack - 0x60
  
  v9 = 0;
  v12 = 0;
  v4 = __errno_location();
  if (*a1 != '/') { // branch-flip
    a1 = (char *)sub_da80(a1,2);
    if (!a1) {
      v14 = *v4;
      v13 = NULL;
      v7 = 0;
      v3 = 0xffffffff;
      v6 = 0;
      goto label_c5df;
    }
    v13 = a1;
  }
  else {
    v13 = NULL;
  }
  if (0 <= (int)selabel_lookup(a0,&v9,a1,a2)) { // branch-flip
    v10 = 0;
    v11 = 0;
    v5 = (void *)sub_e480(a1);
    if ((((int)getcon(&v10) <= -1) || ((int)sub_15510(v5,&v11) <= -1)) || (v1 = mode_to_security_class(a2), !v1)) {
      v14 = *v4;
      v7 = 0;
      v6 = 0;
      free(v5);
      v3 = 0xffffffff;
      freecon(v10);
      freecon(v11);
      *v4 = v14;
      goto label_c5df;
    }
    v2 = security_compute_create(v10,v11,v1,&v12);
    v14 = *v4;
    free(v5);
    freecon(v10);
    freecon(v11);
    *v4 = v14;
    if (0 <= v2) {
      v6 = context_new(v9);
      if (v6) { // branch-flip
        v7 = context_new(v12);
        if (((v7) && (v8 = context_type_get(v6), v8)) && ((!context_type_set(v7,v8) && (v8 = context_str(v7), v8)))) {
          v3 = setfscreatecon(v8);
          v14 = *v4;
        }
        else {
          v14 = *v4;
          v3 = 0xffffffff;
        }
      }
      else {
        v14 = *v4;
        v7 = 0;
        v3 = 0xffffffff;
      }
      goto label_c5df;
    }
  }
  else {
    v14 = *v4;
    if (v14 == 2) {
      *v4 = 0x3d;
      v14 = 0x3d;
      v7 = 0;
      v6 = 0;
      v3 = 0xffffffff;
      goto label_c5df;
    }
  }
  v7 = 0;
  v6 = 0;
  v3 = 0xffffffff;
label_c5df:
  context_free(v6);
  context_free(v7);
  freecon(v9);
  freecon(v12);
  free(v13);
  *v4 = v14;
  return v3;
}

// Function: sub_c6f0 @ 0xc6f0
unsigned long sub_c6f0(unsigned long a0,char *a1,char a2)
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
    v6 = (char *)sub_da80(a1,2);
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
    v4 = sub_17190(&v7,0x10,0);
    do {
      v5 = sub_10840(v4);
      while( true ) {
        if (!v5) {
          if (*v3)
            v10 = *v3;
          if (sub_10650(v4))
            v10 = *v3;
          free(v6);
          v8 = CONCAT71(v9,v10 == 0);
          return v8 & 0xffffffff;
        }
        v1 = *(unsigned long *)(v4 + 0x20);
        if (0 <= (int)sub_c140(a0,v1)) break;
        v10 = *v3;
        v5 = sub_10840(v4);
      }
    } while( true );
  }
  v2 = sub_c140(a0,v7);
  v10 = *v3;
  v8 = CONCAT71(v9,v2 != -1);
  free(v6);
  *v3 = v10;
  return v8 & 0xffffffff;
}

// Function: sub_c830 @ 0xc830
int sub_c830(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = sub_12720();
  if (v1 != -2) {
    if (v1 == -1) {
      v2 = sub_14b30(a2);
      v3 = dcgettext(NULL,"preserving permissions for %s",5);
      error(0,*__errno_location(),v3,v2);
    }
    return v1;
  }
  v2 = sub_14b30(a0);
  error(0,*__errno_location(),"%s",v2);
  return -2;
}

// Function: sub_c8e0 @ 0xc8e0
int sub_c8e0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = sub_127a0(a0,a1,a2);
  if (!v1)
    return 0;
  v2 = sub_14b30(a0);
  v3 = dcgettext(NULL,"setting permissions for %s",5);
  error(0,*__errno_location(),v3,v2);
  return v1;
}

// Function: sub_c960 @ 0xc960
char * sub_c960(char *a0,unsigned long a1)
{
  unsigned long v1;
  char *v2;
  char *v3;
  unsigned long v4;
  char v5 [136];
  
  v4 = 0x80;
  if ((a1) && (v4 = a1 + 1, 0x401 <= a1))
    v4 = 0x401;
label_c9c8:
  if (v4 != 0x80) goto label_ca13;
label_c9d1:
  v2 = v5;
  v3 = NULL;
  if (a1) goto label_ca13;
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
label_ca5b:
        *__errno_location() = 0xc;
        return NULL;
      }
      v4 = 0x7fffffffffffffff;
      goto label_c9c8;
    }
    v4 *= 2;
    if (v4 == 0x80) goto label_c9d1;
label_ca13:
    v2 = malloc(v4);
    if (!v2) goto label_ca5b;
    v3 = v2;
  } while( true );
}

// Function: sub_caf0 @ 0xcaf0
char * sub_caf0(unsigned int a0,unsigned long a1,unsigned long a2)
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
label_cb50:
  if (v5 != 0x80) goto label_cba7;
  do {
    v2 = v4;
    v6 = NULL;
    if (a2) goto label_cba7;
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
        goto label_cb50;
      }
      v5 *= 2;
      if (v5 == 0x80) break;
label_cba7:
      v2 = malloc(v5);
      if (!v2)
        return NULL;
      v6 = v2;
    }
  } while( true );
}

// Function: sub_cc80 @ 0xcc80
void sub_cc80(void)
{
  sub_54f0(1); // tail-call, no-return
}

// Function: sub_cc90 @ 0xcc90
unsigned long sub_cc90(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_cda0 @ 0xcda0
long sub_cda0(char *a0,long *a1) // return-dupe
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

// Function: sub_ce00 @ 0xce00
void sub_ce00(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_14b10(1,a0);
  error(0,0,v1,sub_14310(0,8,a1),v2); // tail-call
}

// Function: sub_ce90 @ 0xce90
void sub_ce90(long *a0,void *a1,unsigned long a2)
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
      __fprintf_chk(stderr,1,", %s",sub_14b30(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_14b30(v1));
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

// Function: sub_cfc0 @ 0xcfc0
long sub_cfc0(unsigned long a0,char *a1,unsigned long *a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_cc90(a1,a2,a3,a4);
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
  sub_ce00(a0,a1,v2);
  sub_ce90(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_d090 @ 0xd090
long sub_d090(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_d0f0 @ 0xd0f0
void sub_d0f0(char *a0)
{
  if ((((a0) || (a0 = getenv("SIMPLE_BACKUP_SUFFIX"), a0)) && (*a0)) && (a0 == (char *)sub_d970(a0))) {
    dat_23580 = a0;
    return;
  }
  dat_23580 = "~";
}

// Function: sub_d150 @ 0xd150
void * sub_d150(int a0,void *a1,int a2,char a3) // return-dupe
{
  unsigned int *v1;
  void *v10;
  void *v11;
  unsigned short *v12;
  void *v13;
  int *v14; // rax
  long v15;
  void *v16; // rax
  char *v17;
  char v18;
  int v19; // stack - 0x44
  unsigned long v2;
  char *v20;
  long v21; // r14
  unsigned long v22; // rcx
  unsigned long v23;
  unsigned char v24;
  long v25; // stack - 0xc0
  int v26; // stack - 0xa4
  unsigned long v27; // stack - 0xa0
  unsigned long v28; // stack - 0x98
  long v29; // stack - 0x70
  unsigned short v3;
  void *v30; // stack - 0x60
  bool v4;
  unsigned char v5;
  int v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  v30 = (void *)sub_d970(a1);
  v25 = sub_d9d0(v30);
  v21 = (long)v30 - (long)a1;
  v7 = v25 + v21;
  v20 = dat_23580;
  if (!dat_23580) {
    v17 = getenv("SIMPLE_BACKUP_SUFFIX");
    v20 = "~";
    if (((v17) && (*v17)) && (v20 = (char *)sub_d970(v17), v17 != v20))
      v20 = "~";
    dat_23580 = v20;
  }
  v8 = strlen(v20) + 1;
  v9 = 9;
  if (9 <= (long)v8)
    v9 = v8;
  v9 = v7 + 1 + v9;
  v10 = malloc(v9);
  if (!v10)
    return NULL;
  v29 = 0;
  v16 = NULL;
  v19 = -100;
  v26 = a2;
  do {
    memcpy(v10,a1,v7);
    if (v26 != 1) { // branch-flip
      if (v16) { // branch-flip
        rewinddir(v16);
label_d251:
        v27 = 1;
        v28 = v9;
        v5 = 2;
label_d270:
        v24 = v5;
        v11 = readdir(v16);
        v13 = v10;
        v5 = v24;
        if (v11) {
          while( true ) {
            v20 = (char *)((long)v11 + 0x13);
            v10 = v13;
            if (strlen(v20) < v25 + 4U) break;
            if (memcmp((void *)((long)v13 + v21),v20,v25 + 2U)) break;
            v20 = &v20[v25 + 2U];
            if (8 < (unsigned char)(*v20 - 0x31U)) break;
            v24 = *v20 == '9';
            v18 = v20[1];
            v6 = (int)v18;
            if (10 <= (unsigned int)(v6 - 0x30U)) { // branch-flip
              v22 = 1;
              v23 = 1;
            }
            else {
              v22 = 1;
              do {
                v22 += 1;
                v24 &= (char)v6 == '9';
                v18 = v20[v22];
                v6 = (int)v18;
                v23 = v22;
              } while ((unsigned int)(v6 - 0x30U) <= 9);
            }
            if ((v18 != '~') || (v20[v22 + 1])) break;
            if ((long)v23 <= (long)v27) {
              if (v27 != v23) break;
              if (1 <= memcmp((void *)((long)v13 + v7 + 2),v20,v22)) break;
            }
            v27 = v24 + v23;
            v2 = v27 + v7 + 4;
            if ((long)v28 < (long)v2) {
              if (SCARRY8((long)v2 >> 1,v2))
                v28 = v2;
              else {
                v28 = ((long)v2 >> 1) + v2;
              }
              v10 = realloc(v13,v28 == 0 | v28);
              if (!v10) {
                closedir(v16);
                v10 = v13;
                goto label_d4c8;
              }
            }
            v12 = (unsigned short *)(v7 + (long)v10);
            *v12 = 0x7e2e;
            *(char *)&v12[1] = 0x30;
            v13 = memcpy((void *)((long)v12 + (unsigned long)v24 + 2),v20,v23 + 2);
            v20 = (char *)((long)v13 + (v22 - 1));
            v18 = *(char *)((long)v13 + (v22 - 1));
            while (v18 == '9') {
              *v20 = '0';
              v17 = &v20[-1];
              v20 = &v20[-1];
              v18 = *v17;
            }
            *v20 = v18 + '\x01';
            v11 = readdir(v16);
            v13 = v10;
            v5 = v24;
            if (!v11) goto label_d420;
          }
          goto label_d270;
        }
label_d420:
        if (v24 == 2) goto label_d588;
label_d42b:
        if (v24 == 1) goto label_d593;
label_d436:
        v4 = 1;
      }
      else {
        v12 = (unsigned short *)((long)v10 + v21);
        v3 = *v12;
        *v12 = 0x2e;
        v16 = (void *)sub_12620(a0,v10,0,&v19);
        v1 = (unsigned int *)((long)v12 + v25);
        if (v16) {
          *v12 = v3;
          *v1 = 0x7e317e2e;
          *(char *)&v1[1] = 0;
          goto label_d251;
        }
        v6 = *__errno_location();
        *v12 = v3;
        *v1 = 0x7e317e2e;
        *(char *)&v1[1] = 0;
        v24 = (v6 == 0xc) + 2;
        if (v24 != 2) {
          if (v24 == 3) {
label_d4c8:
            free(v10);
            *__errno_location() = 0xc;
            return NULL;
          }
          goto label_d42b;
        }
label_d588:
        if (v26 == 2) {
          memcpy((void *)((long)v10 + v7),dat_23580,v8);
          v26 = 1;
        }
label_d593:
        v4 = 1;
        v12 = (unsigned short *)sub_d970(v10);
        v15 = sub_d9d0(v12);
        v6 = v19;
        if (v15 <= 0xe) goto label_d43c;
        if (v29) { // branch-flip
label_d635:
          if (v15 <= v29) goto label_d436;
        }
        else {
          v14 = __errno_location();
          if (0 <= v6) { // branch-flip
            *v14 = 0;
            v29 = fpathconf(v6,3);
            v29 -= (unsigned long)(*v14 == 0);
          }
          else {
            v3 = *v12;
            *v12 = 0x2e;
            *v14 = 0;
            v29 = pathconf(v10,3);
            v29 -= (unsigned long)(*v14 == 0);
            *v12 = v3;
          }
          if (0 <= v29) goto label_d635;
          v15 = 0xe;
          if (v29 == -1)
            v15 = v29;
          v29 = v15;
        }
        v15 = (long)v10 + (v7 - (long)v12);
        if (v29 <= v15)
          v15 = v29 + -1;
        v4 = 0;
        *(char *)((long)v12 + v15) = 0x7e;
        *(char *)((long)v12 + v15 + 1) = 0;
      }
    }
    else {
      v4 = 1;
      memcpy((void *)((long)v10 + v7),dat_23580,v8);
    }
label_d43c:
    if (!a3) {
label_d738:
      if (!v16)
        return v10;
      closedir(v16);
      return v10;
    }
    if (0 <= v19) { // branch-flip
      v13 = (void *)((long)v10 + v21);
      v11 = v30;
      v6 = v19;
    }
    else {
      v13 = v10;
      v11 = a1;
      v6 = a0;
    }
    v6 = sub_14b50(v6,v11,v19,v13,v26 != 1);
    if (!v6) goto label_d738;
    v14 = __errno_location();
    v6 = *v14;
    if ((v6 != 0x11) || (!v4)) {
      if (v16)
        closedir(v16);
      free(v10);
      *v14 = v6;
      return NULL;
    }
  } while( true );
}

// Function: sub_d840 @ 0xd840
void sub_d840(void)
{
  sub_d150(); // tail-call
}

// Function: sub_d850 @ 0xd850
void sub_d850(void)
{
  if (sub_d150())
    return;
  sub_17150(); // no-return
}

// Function: sub_d870 @ 0xd870
unsigned int sub_d870(unsigned long a0,char *a1)
{
  long v1; // rax
  
  if (!a1)
    return 2;
  if (!*a1)
    return 2;
  v1 = sub_cfc0(a0,a1,0x228a0,0x1c520,4,dat_23028);
  return *(unsigned int *)(v1 * 4 + 0x1c520);
}

// Function: sub_d8d0 @ 0xd8d0
unsigned int sub_d8d0(unsigned long a0,char *a1)
{
  char *v1; // rax
  long v2; // rax
  
  if ((a1) && (*a1)) {
    v2 = sub_cfc0(a0,a1,0x228a0,0x1c520,4,dat_23028,1);
    return *(unsigned int *)(v2 * 4 + 0x1c520);
  }
  v1 = getenv("VERSION_CONTROL");
  if ((v1) && (*v1)) {
    v2 = sub_cfc0("$VERSION_CONTROL",v1,0x228a0,0x1c520,4,dat_23028,1);
    return *(unsigned int *)(v2 * 4 + 0x1c520);
  }
  return 2;
}

// Function: sub_d970 @ 0xd970
char * sub_d970(char *a0)
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

// Function: sub_d9d0 @ 0xd9d0
void sub_d9d0(char *a0)
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

// Function: sub_da00 @ 0xda00
unsigned long sub_da00(unsigned long a0,unsigned long a1,unsigned long a2)
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

// Function: sub_da80 @ 0xda80
long sub_da80(char *a0,unsigned int a1) // early-return x2
{
  char v1;
  char v10 [1024];
  char v11; // stack - 0xc68
  char v12 [1032];
  char *v13; // stack - 0xc78
  char v14 [1023];
  char *v15; // stack - 0x458
  char v16 [144];
  char *v17; // stack - 0x868
  char *v18;
  unsigned long v19;
  int v2;
  char *v20;
  char *v21;
  char *v22;
  unsigned int v23; // r15d
  bool v24;
  char *v25; // stack - 0xd50
  long v26; // stack - 0xd40
  int v27; // stack - 0xd24
  long v28; // stack - 0xd18
  unsigned long v29; // stack - 0xc70
  bool v3;
  unsigned long v30; // stack - 0x860
  long v31; // stack - 0x450
  char *v4;
  char v5;
  long v6;
  char *v7;
  unsigned long v8; // rax
  char *v9;
  
  v23 = a1 & 3;
  if ((v23 - 1 & v23) || (!a0)) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!*a0) {
    *__errno_location() = 2;
    return 0;
  }
  v30 = 0x400;
  v15 = v12;
  v17 = v10;
  v31 = 0x400;
  v29 = 0x400;
  v9 = &v11;
  v19 = 0x400;
  v13 = &v11;
  if (*a0 != '/') { // branch-flip
    while (!getcwd(v9,v19)) {
      v2 = *__errno_location();
      if (v2 == 0xc) {
        sub_17150(); // no-return, return-dupe
      }
      if (v2 != 0x22) {
        v3 = 1;
        v7 = v9;
        goto label_db7f;
      }
      if (!sub_153d0(&v13)) {
        sub_17150();
      }
      v9 = v13;
      v19 = v29;
    }
    v7 = rawmemchr(v9,0);
    v5 = *a0;
    if (!v5) {
      if ((v7 <= &v9[1]) || (v7[-1] != '/'))
        v3 = 0;
      else {
        v7 = &v7[-1];
        v3 = 0;
      }
      goto label_db7f;
    }
  }
  else {
    v11 = '/';
    v7 = v14;
    v5 = '/';
  }
  v3 = 0;
  v26 = 0;
  v27 = 0;
  v22 = v7;
label_dd0f:
  do {
    v7 = v22;
    if (v5 != '/') // branch-flip
      v25 = a0;
    else {
      do {
        v5 = a0[1];
        a0 = &a0[1];
      } while (v5 == '/');
      if (!v5) break;
      v25 = a0;
    }
    do {
      v18 = a0;
      v1 = v18[1];
      a0 = &v18[1];
      if (!v1) break;
    } while (v1 != '/');
    v19 = (long)a0 - (long)v25;
    if (!v19) break;
    if (v19 != 1) { // branch-flip
      if (((v5 != '.') || (v19 != 2)) || (v25[1] != '.')) {
label_dc49:
        if (v22[-1] != '/') {
          *v22 = '/';
          v22 = &v22[1];
        }
        if (&v9[v29 - (long)v22] < (char *)(v19 + 2)) {
          do {
            v6 = (long)v22 - (long)v9;
            if (!sub_15450(&v13)) {
              sub_17150();
            }
            v22 = &v13[v6];
            v9 = v13;
          } while ((char *)(v29 - v6) < (char *)(v19 + 2));
        }
        v7 = mempcpy(v22,v25,v19);
        *v7 = '\0';
        v22 = v7;
        if (!(a1 & 4)) {
          while( true ) {
            v4 = v15;
            v19 = v31 - 1;
            v8 = readlink(v9,v15,v19);
            if ((long)v19 > (long)v8) break;
            if (!sub_153d0(&v15)) {
              sub_17150();
            }
          }
          if ((long)v8 < 0) goto label_de62;
          if (v27 <= 0x13) {
            v27 += 1;
label_e0d7:
            v21 = v17;
            v4[v8] = '\0';
            v6 = (long)a0 - (long)v17;
            if (!v3)
              v6 = v28;
            v25 = (unsigned long)strlen(a0);
            if (!SCARRY8((long)v25 + 0x8000000000000000,v8)) {
              if (v30 <= v8 + (long)v25) {
                do {
                  if (!sub_15450(&v17)) {
                    sub_17150();
                  }
                } while (v30 <= v8 + (long)v25);
                v21 = v17;
              }
              if (v3)
                a0 = &v21[v6];
              memmove(&v21[v8],a0,(long)v25 + 1);
              a0 = memcpy(v21,v4,v8);
              v22 = &v9[1];
              v28 = v6;
              if (*v4 != '/') { // branch-flip
                v3 = 1;
                v24 = v22 < v7;
                v22 = v7;
                v5 = *a0;
                if (v24) {
                  do {
                    v22 = &v7[-1];
                    if (v9 == v22) break;
                    v18 = &v7[-2];
                    v7 = v22;
                  } while (*v18 != '/');
                  v3 = 1;
                }
              }
              else {
                *v9 = '/';
                v3 = 1;
                v5 = *a0;
              }
              goto label_ddb8;
            }
            sub_17150();
          }
          if (!*v25) goto label_e0d7;
          v7[(long)v25 - (long)a0] = '\0';
          v20 = ".";
          if (*v9)
            v20 = v9;
          if (!stat(v20,v16)) {
            v7[(long)v25 - (long)a0] = *v25;
            if (!v26) {
              v26 = sub_11bb0(7,0,sub_12460,sub_12490,sub_124d0);
              if (!v26) {
                sub_17150();
              }
            }
            if (!sub_e820(v26,v25,v16)) {
              sub_e790(v26,v25,v16);
              goto label_e0d7;
            }
            if (v23 != 2) {
              *__errno_location() = 0x28;
              goto label_df58;
            }
            goto label_dee0;
          }
label_df58:
          v3 = 1;
          goto label_ddd8;
        }
label_de62:
        if (v23 != 2) {
          v5 = *a0;
          if (v5 == '/') {
            v4 = a0;
label_de80:
            do {
              v20 = v4;
              v1 = v20[1];
              v4 = &v20[1];
            } while (v1 == '/');
            if (!v1) {
label_dfe8:
              *(unsigned short *)v7 = dat_1c540;
              goto label_dec4;
            }
            if (v1 != '.') goto label_df20;
            v1 = v20[2];
            if (!v1) goto label_dfe8;
            if (v1 != '.') {
              if (v1 != '/') goto label_df20;
              v4 = &v20[2];
              goto label_de80;
            }
            if ((!v20[3]) || (v20[3] == '/')) goto label_dfe8;
          }
label_df20:
          if (a1 & 4) { // branch-flip
            if (v5) goto label_dd0f;
label_dec4:
            if (faccessat(0xffffff9c,v9,0,0x200)) {
label_df32:
              if (((v23 != 1) || (*__errno_location() != 2)) || (a0[strspn(a0,"/")])) goto label_df58;
            }
          }
          else if (*__errno_location() != 0x16) goto label_df32;
        }
label_dee0:
        v5 = v18[1];
        if (!v5) break;
        goto label_dd0f;
      }
      v5 = v1;
      if (&v9[1] < v22) {
        v22 = &v22[-1];
        if (v9 < v22) {
          do {
            if (v22[-1] == '/') break;
            v22 = &v22[-1];
          } while (v22 != v9);
        }
      }
    }
    else {
      v24 = v5 != '.';
      v5 = v1;
      if (v24) goto label_dc49;
    }
label_ddb8:
    v7 = v22;
  } while (v5);
  v3 = 0;
  if (&v9[1] < v7) {
    v7 = &v7[-(unsigned long)(v7[-1] == '/')];
    v3 = 0;
  }
label_ddd8:
  if (v26)
    sub_11d70(v26);
label_db7f:
  if (v17 != v10)
    free(v17);
  if (v15 != v12)
    free(v15);
  if (v3) {
    if (v13 != &v11) {
      free(v13);
      return 0;
    }
    return 0;
  }
  *v7 = '\0';
  v6 = sub_15370(&v13,&v7[1 - (long)v9]);
  if (v6)
    return v6;
  sub_17150();
}

// Function: sub_e2a0 @ 0xe2a0
void sub_e2a0(unsigned long a0)
{
  dat_23588 = a0;
}

// Function: sub_e2b0 @ 0xe2b0
void sub_e2b0(void)
{
  int v1; // eax
  long v2;
  char *v3; // rax
  int *v4; // rax
  unsigned long v5;
  
  v5 = stdin;
  v2 = sub_eb30(stdin);
  if (v2) { // branch-flip
    if ((!sub_eb60(v5,0,1)) && (v1 = sub_e740(stdin), v1)) {
      sub_175e0(stdin);
      goto label_e30f;
    }
    v1 = sub_175e0(stdin);
  }
  else {
    v1 = sub_175e0(v5);
  }
  if (!v1) {
    sub_e3d0(); // tail-call
    return;
  }
label_e30f:
  v3 = dcgettext(NULL,"error closing file",5);
  v2 = dat_23588;
  v4 = __errno_location();
  if (v2) { // branch-flip
    v5 = sub_146a0(v2);
    error(0,*v4,"%s: %s",v5,v3);
    sub_e3d0();
  }
  else {
    error(0,*v4,"%s",v3);
    sub_e3d0();
  }
  _exit(dat_23030); // no-return
}

// Function: sub_e3b0 @ 0xe3b0
void sub_e3b0(unsigned long a0)
{
  dat_23598 = a0;
}

// Function: sub_e3c0 @ 0xe3c0
void sub_e3c0(char a0)
{
  dat_23590 = a0;
}

// Function: sub_e3d0 @ 0xe3d0
void sub_e3d0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_175e0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_23590) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_23598) { // branch-flip
        v4 = sub_146a0(dat_23598);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_23030); // return-dupe, no-return
    }
  }
  v1 = sub_175e0(stderr);
  if (!v1)
    return;
  _exit(dat_23030);
}

// Function: sub_e480 @ 0xe480
void sub_e480(void)
{
  if (sub_e4e0())
    return;
  sub_17150(); // no-return
}

// Function: sub_e4a0 @ 0xe4a0
void sub_e4a0(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_d970(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_e4e0 @ 0xe4e0
char * sub_e4e0(char *a0)
{
  unsigned long v1;
  char v2;
  void *v3; // rax
  char *v4; // rax
  unsigned long v5;
  
  v2 = *a0;
  v5 = sub_d970(a0) - (long)a0;
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
label_e54e:
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
      goto label_e54e;
    }
    v5 = v1;
  } while( true );
}

// Function: sub_e5a0 @ 0xe5a0
bool sub_e5a0(char *a0)
{
  char v1;
  char *v2;
  
  v2 = (char *)sub_d970(a0);
  if (!*v2)
    v2 = a0;
  v1 = v2[sub_d9d0(v2)];
  v2[sub_d9d0(v2)] = '\0';
  return v1 != '\0';
}

// Function: sub_e5e0 @ 0xe5e0
void sub_e5e0(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_e5f0 @ 0xe5f0
void sub_e5f0(void *a0,unsigned int a1)
{
  if (a0) {
    posix_fadvise(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_e620 @ 0xe620
void sub_e620(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_15ad0(open(a0,a1,v1));
}

// Function: sub_e6a0 @ 0xe6a0
int * sub_e6a0(unsigned long a0,unsigned int a1,long a2,unsigned long a3,unsigned int a4)
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
    if (!a2) goto label_e720;
    v2 = (unsigned long)utimensat(a1,a2,a3,a4);
  }
  if (v1 != 1)
    return v2;
label_e720:
  v2 = __errno_location();
  *v2 = 9;
  v2 = (int *)0xffffffff;
  return v2;
}

// Function: sub_e740 @ 0xe740
void sub_e740(unsigned int *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*a0 & 0x100)) {
      sub_eb60(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_e790 @ 0xe790
void sub_e790(long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  if (!a0)
    return;
  v1 = (unsigned long *)sub_16c50(0x18);
  *v1 = sub_17130(a1);
  v1[1] = a2[1];
  v1[2] = *a2;
  v2 = (unsigned long *)sub_12210(a0,v1);
  if (v2) {
    if (v1 != v2) {
      sub_124d0(v1); // tail-call
      return;
    }
    return;
  }
  sub_17150(); // no-return
}

// Function: sub_e820 @ 0xe820
unsigned long sub_e820(long a0,unsigned long a1,unsigned long *a2) // early-return
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
  v1 = sub_11920(a0,&v2);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
}

// Function: sub_e880 @ 0xe880
void sub_e880(unsigned int a0,char *a1) // ternary x2
{
  unsigned char v1;
  char v2; // cl
  char v3; // cl
  unsigned int v4; // esi
  
  v2 = 0x2d;
  v4 = a0 & 0xf000;
  if ((((v4 != 0x8000) && (v2 = 100, v4 != 0x4000)) && (v2 = 0x62, v4 != 0x6000)) && (((v2 = 99, v4 != 0x2000 && (v2 = 0x6c, v4 != 0xa000)) && ((v2 = 0x70, v4 != 0x1000 && (v2 = 0x73, v4 != 0xc000))))))
    v2 = 0x3f;
  *a1 = v2;
  a1[1] = (-((a0 & 0x100) == 0) & 0xbbU) + 0x72;
  a1[2] = (-((a0 & 0x80) == 0) & 0xb6U) + 0x77;
  v1 = -((a0 & 0x40) == 0);
  v3 = (a0 & 0x800) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1[3] = v3;
  a1[4] = (-((a0 & 0x20) == 0) & 0xbbU) + 0x72;
  a1[5] = (-((a0 & 0x10) == 0) & 0xb6U) + 0x77;
  v1 = -((a0 & 8) == 0);
  v3 = (a0 & 0x400) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1[6] = v3;
  a1[7] = (-((a0 & 4) == 0) & 0xbbU) + 0x72;
  a1[8] = (-((a0 & 2) == 0) & 0xb6U) + 0x77;
  if (a0 & 0x200) {
    a1[9] = (-((a0 & 1) == 0) & 0xe0U) + 0x74;
    *(unsigned short *)&a1[10] = 0x20;
    return;
  }
  a1[9] = (-((a0 & 1) == 0) & 0xb5U) + 0x78;
  *(unsigned short *)&a1[10] = 0x20;
}

// Function: sub_ea00 @ 0xea00
void sub_ea00(long a0)
{
  sub_e880(*(unsigned int *)(a0 + 0x18)); // tail-call
}

// Function: sub_ea10 @ 0xea10
void sub_ea10(void)
{
  if (sub_ea30())
    return;
  sub_17150(); // no-return
}

// Function: sub_ea30 @ 0xea30
void * sub_ea30(void *a0,char *a1,unsigned long *a2)
{
  unsigned long v1;
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  void *v5; // rax
  char *v6; // rax
  unsigned long v7; // rbx
  char v8; // stack - 0x41
  
  v2 = sub_d970(a0);
  v3 = sub_d9d0(v2);
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

// Function: sub_eb30 @ 0xeb30
long sub_eb30(unsigned int *a0) // early-return
{
  long v1; // rax
  
  if (*(unsigned long *)&a0[8] < *(unsigned long *)&a0[10])
    return 0;
  v1 = *(long *)&a0[4] - *(long *)&a0[2];
  if (*a0 & 0x100)
    v1 += *(long *)&a0[0x16] - *(long *)&a0[0x12];
  return v1;
}

// Function: sub_eb60 @ 0xeb60
unsigned long sub_eb60(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
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

// Function: sub_ebe0 @ 0xebe0
unsigned long sub_ebe0(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_ec10 @ 0xec10
unsigned long sub_ec10(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_ec30 @ 0xec30
unsigned long sub_ec30(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_ec40 @ 0xec40
unsigned long sub_ec40(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_ec50 @ 0xec50
int sub_ec50(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_ec70 @ 0xec70
int sub_ec70(long a0,long a1,char a2) // early-return x3
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
      goto label_ed0b;
    }
  }
  else {
    v3 = *(char **)(a1 + 0x30);
    v5 = *(int *)(a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_ed0b:
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

// Function: sub_ee20 @ 0xee20
long sub_ee20(long a0,long a1,unsigned long a2)
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
label_ee7b:
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
      goto label_ee7b;
    }
    v4 = *(long **)(a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  return a1;
}

// Function: sub_ef30 @ 0xef30
void * sub_ef30(long a0,void *a1,unsigned long a2)
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

// Function: sub_efd0 @ 0xefd0
unsigned long sub_efd0(long a0,int a1)
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
    v2 = sub_11bb0(0xd,0,sub_ec30,sub_ec40,dat_22fc0);
    *(long *)(v1 + 0x50) = v2;
    if (!v2) goto label_f05e;
  }
  v5[0] = *(unsigned long *)(a0 + 0x70);
  v1 = sub_11920(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_f05e:
  if ((0 <= a1) && (!fstatfs(a1,v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      *v3 = *(unsigned long *)(a0 + 0x70);
      v3[1] = v6[0];
      v4 = (unsigned long *)sub_12210(v2,v3);
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

// Function: sub_f100 @ 0xf100
void sub_f100(long a0,int a1,char a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_12520(a0 + 0x60);
    if (0 <= v1) {
      close(v1);
      goto label_f144;
    }
  }
  else if ((!(*(unsigned char *)(a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_f144:
    *(int *)(a0 + 0x2c) = a1;
    return;
  }
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_f160 @ 0xf160
unsigned long sub_f160(long a0,long a1)
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

// Function: sub_f1d0 @ 0xf1d0
unsigned long sub_f1d0(long a0)
{
  long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) {
    v1 = sub_11bb0(0x1f,0,sub_ec10,sub_ebe0,dat_22fc0);
    *(long *)(a0 + 0x58) = v1;
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)(a0 + 0x58) = v2;
  if (v2) {
    sub_17650(v2);
    return 1;
  }
  return 0;
}

// Function: sub_f240 @ 0xf240
unsigned long sub_f240(long a0,unsigned long *a1) // return-dupe x2
{
  unsigned long v1;
  unsigned long *v2; // rax
  unsigned long *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v4 = sub_17670(*(unsigned long *)(a0 + 0x58),&a1[0xe]);
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
  v3 = (unsigned long *)sub_12210(v1,v2);
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

// Function: sub_f2f0 @ 0xf2f0
void sub_f2f0(long a0,long a1)
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
    v4 = (void *)sub_12270(*(unsigned long *)(a0 + 0x58),&v5);
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

// Function: sub_f3b0 @ 0xf3b0
bool sub_f3b0(long a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)(a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
    sub_f100(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)(a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  while( true ) {
    if (sub_12510(a0 + 0x60)) break;
    v1 = sub_12560(a0 + 0x60);
    if (0 <= v1)
      close(v1);
  }
  return v2;
}

// Function: sub_f440 @ 0xf440
unsigned int sub_f440(long a0,long a1,int a2,char *a3) // return-dupe
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
label_f5f0:
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
      if (!(v7 & 2)) goto label_f5cf;
    }
    else {
      if (!(v7 & 0x200)) {
        v10 = 1;
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        goto label_f654;
      }
      v4 = sub_12510(a0 + 0x60);
      if (v4) {
        v1 = *(unsigned int *)(a0 + 0x2c);
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        v10 = v4;
        goto label_f4e5;
      }
      v5 = sub_12560(a0 + 0x60);
      v7 = *(unsigned int *)(a0 + 0x48);
      if ((int)v5 < 0) {
        v10 = 1;
        v5 = v7;
        goto label_f6e4;
      }
      v10 = 1;
      a2 = v5;
      v9 = v5;
      if (!(v7 & 2)) goto label_f550;
    }
label_f510:
    v5 = fstat(v9,&v8);
    if (!v5) {
      if ((*(long *)(a1 + 0x70) == v8) && (*(long *)(a1 + 0x78) == v11)) {
        v7 = *(unsigned int *)(a0 + 0x48);
        goto label_f550;
      }
      v6 = __errno_location();
      *v6 = 2;
    }
    v3 = -1;
  }
  else {
    v7 = *(unsigned int *)(a0 + 0x48);
    if (v7 & 4) goto label_f5f0;
    if (0 <= a2) // branch-flip
      v10 = 0;
    else {
      v10 = 0;
      v5 = v7;
label_f6e4:
      v5 &= 0x200;
      v1 = *(unsigned int *)(a0 + 0x2c);
      v7 = (v7 & 0x10) << 0xd | 0x90900;
      if (v5) { // branch-flip
label_f4e5:
        v5 = sub_125a0(v1,a3,v7);
      }
      else {
label_f654:
        v5 = sub_e620(a3,v7);
      }
      if ((int)v5 < 0) {
        v5 = -1;
        return v5;
      }
      v7 = *(unsigned int *)(a0 + 0x48);
      v9 = v5;
    }
    if (v7 & 2) goto label_f510;
    if (a3) {
label_f5cf:
      v5 = strcmp(a3,"..");
      if (!v5) goto label_f510;
    }
label_f550:
    if (v7 & 0x200) {
      sub_f100(a0,v9,v10 ^ 1);
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

// Function: sub_f740 @ 0xf740
void * sub_f740(long *a0,int a1)
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
label_f7a2:
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
    v13 = sub_12620(v6,*(unsigned long *)(v2 + 0x30),v7,&v17);
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
        sub_f2f0(a0,v2);
        sub_ec70(a0,v2,0);
        if (!sub_f240(a0,v2)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)(v2 + 0x68) = sub_ec70(a0,v2,0);
    }
    v27 = (unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_ffd0:
      v22 = 0;
      v15 = *(unsigned int *)&a0[9];
      goto label_f7ab;
    }
    v7 = v17;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)(v2 + 0x80) != 2)) {
label_fc75:
      v22 = a1 == 3;
      v3 = 1;
    }
    else {
      v13 = sub_efd0(v2,v17);
      v7 = v17;
      if (v13 == 0x9fa0) goto label_fc75;
      if (v13 <= 0x9fa0) {
        if ((v13) && (v13 != 0x6969)) goto label_fe11;
        goto label_fc75;
      }
      if ((v13 == 0x5346414f) || (v13 == 0xff534d42)) goto label_fc75;
label_fe11:
      if (a1 != 3) goto label_ffd0;
      v3 = 0;
      v22 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v7 = sub_17830(v7,0x406,3);
      v17 = v7;
    }
    if ((0 <= v7) && (!sub_f440(a0,v2,v7,0))) goto label_f7a2;
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
label_f7ab:
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
          goto label_fe7e;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (*(char *)((long)v9 + 0x13) != '.')) || ((*(char *)((long)v9 + 0x14) && (*(short *)((long)v9 + 0x14) != 0x2e)))) break;
label_f874:
        v19 = *(void **)(v2 + 0x18);
        if (!v19) goto label_fe7e;
      }
      v10 = strlen((char *)((long)v9 + 0x13));
      v11 = (void *)sub_ef30(a0,(char *)((long)v9 + 0x13),v10);
      if (!v11) {
label_fd18:
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
        v4 = sub_f160(a0,v28 + 2 + v10);
        if (!v4) goto label_fd18;
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
        *(unsigned short *)((long)v11 + 0x68) = sub_ec70(a0,v11,0);
      else {
        v1 = *(unsigned char *)((long)v9 + 0x12);
        v5 = (unsigned int)v1 - 1;
        if ((v15 & 8) && (v1 & 0xfb)) {
          if ((v15 & 0x10) || (v1 != 10)) {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
            if (0xc <= v5) {
              v6 = 0;
              v16 = 1;
              goto label_f968;
            }
            v16 = 1;
          }
          else {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
label_fad8:
            v16 = 2;
          }
          v6 = *(unsigned int *)((unsigned long)v5 * 4 + 0x1c580);
        }
        else {
          *(unsigned short *)((long)v11 + 0x68) = 0xb;
          if (v5 <= 0xb) goto label_fad8;
          v6 = 0;
          v16 = 2;
        }
label_f968:
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
          goto label_f874;
        }
        break;
      }
      if (!a0[8]) {
        v12 = sub_efd0(v2,v17);
        if (((v12 == 0x1021994) || (v12 == 0xff534d42)) || (v12 == 0x6969)) {
          v24 = v11;
          v20 = 0x2711;
          v3 = 0;
          goto label_f874;
        }
        v3 = 1;
      }
      v20 = 0x2711;
      v19 = *(void **)(v2 + 0x18);
      v24 = v11;
    } while (v19);
label_fe7e:
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
      if ((a0[6] == v23) || (!v20)) goto label_fe9b;
label_fea1:
      *v26 = 0;
    }
    if ((!v14) && (v22)) {
      if ((a1 != 1) && (v20)) goto label_fecc;
      goto label_ff20;
    }
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v3 = 0;
      v25 = NULL;
      v20 = 0;
label_fe9b:
      v26 = &v26[-1];
      goto label_fea1;
    }
    v25 = NULL;
    if (!(bool)(v22 & v14 == NULL)) goto label_ff45;
    v3 = 0;
    v20 = 0;
label_ff20:
    if (*(long *)(v2 + 0x58)) // branch-flip
      v7 = sub_f440(a0,*(unsigned long *)(v2 + 8),0xffffffff,"..");
    else {
      v7 = sub_f3b0(a0);
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
label_ff45:
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
label_fecc:
  if (v3) {
    a0[8] = (long)sub_ec50;
    v14 = (void *)sub_ee20(a0,v25,v20);
    a0[8] = 0;
    return v14;
  }
  if (!a0[8])
    return v25;
  if (v20 == 1)
    return v25;
  return (void *)sub_ee20(a0,v25,v20);
}

// Function: sub_10230 @ 0x10230
long * sub_10230(long *a0,unsigned long a1,long a2)
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
  v15 = sub_f160(v4,v10);
  if (!v15) {
    free(v4); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v14 = (void *)sub_ef30(v4,"",0);
    if (v14) {
      *(unsigned long *)((long)v14 + 0x58) = 0xffffffffffffffff;
      v11 = (char *)*a0;
      goto label_10333;
    }
  }
  else {
    v14 = NULL;
    v11 = NULL;
label_10333:
    if (a2)
      v15 = (unsigned char)(*(unsigned int *)&v4[9] >> 10) & 1;
    if (v11) { // branch-flip
      v16 = NULL;
      v8 = NULL;
      v7 = 0;
label_103ed:
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
        v6 = (void *)sub_ef30(v4,v11,v10);
        if (!v6) goto label_1047d;
        *(unsigned long *)((long)v6 + 0x58) = 0;
        *(void **)((long)v6 + 8) = v14;
        *(long *)((long)v6 + 0x30) = (long)v6 + 0x100;
        if ((v8) && (v15)) {
          *(unsigned long *)((long)v6 + 0xa0) = 2;
          *(unsigned short *)((long)v6 + 0x68) = 0xb;
          if (a2) { // branch-flip
label_103d5:
            *(void **)((long)v6 + 0x10) = v8;
          }
          else {
            *(unsigned long *)((long)v6 + 0x10) = 0;
label_10578:
            v16 = v6;
            *(void **)((long)v1 + 0x10) = v6;
            v6 = v8;
          }
          v11 = (char *)a0[v12 + 1];
          v8 = v6;
          v7 = v12 + 1;
          if (!v11) break;
          goto label_103ed;
        }
        *(unsigned short *)((long)v6 + 0x68) = sub_ec70(v4,v6,0);
        if (a2) goto label_103d5;
        *(unsigned long *)((long)v6 + 0x10) = 0;
        if (v8) goto label_10578;
        v11 = (char *)a0[v12 + 1];
        v8 = v6;
        v7 = v12 + 1;
        v16 = v6;
      } while (v11);
      if ((a2) && (2 <= v12 + 1U))
        v6 = (void *)sub_ee20(v4,v6,v12 + 1U);
      v7 = sub_ef30(v4,"",0);
      *v4 = v7;
      v8 = v6;
      if (v7) {
        *(void **)(v7 + 0x10) = v6;
        *(unsigned short *)(v7 + 0x68) = 9;
        *(unsigned long *)(v7 + 0x58) = 1;
        if (sub_f1d0(v4)) {
label_1054f:
          v9 = *(unsigned int *)&v4[9];
          if (!(v9 & 0x204)) {
            if (v9 & 0x200) // branch-flip
              v3 = sub_125a0(*(unsigned int *)((long)v4 + 0x2c),".");
            else {
              v3 = sub_e620(".",(v9 & 0x10) << 0xd | 0x90900);
            }
            *(int *)&v4[5] = v3;
            if (v3 <= -1)
              *(unsigned int *)&v4[9] = *(unsigned int *)&v4[9] | 4;
          }
          sub_124f0(&v4[0xc],0xffffffff);
          return v4;
        }
      }
label_1047d:
      while (v8) {
        v1 = *(void **)((long)v8 + 0x10);
        if (*(void **)((long)v8 + 0x18))
          closedir(*(void **)((long)v8 + 0x18));
        free(v8);
        v8 = v1;
      }
    }
    else {
      v7 = sub_ef30(v4,"",0);
      *v4 = v7;
      if (v7) {
        *(unsigned long *)(v7 + 0x10) = 0;
        *(unsigned short *)(v7 + 0x68) = 9;
        *(unsigned long *)(v7 + 0x58) = 1;
        if (sub_f1d0(v4)) goto label_1054f;
      }
    }
    free(v14);
  }
  free((void *)v4[4]);
  free(v4);
  return NULL;
}

// Function: sub_10650 @ 0x10650
unsigned int sub_10650(long *a0) // return-dupe
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
      goto label_1070c;
    }
  }
  else if (!(*(unsigned int *)&a0[9] & 4)) {
    v2 = a0[5];
    if (fchdir((int)v2)) { // branch-flip
      v4 = __errno_location();
      v6 = *v4;
      v3 = close((int)a0[5]);
      if ((v6) || (!v3)) goto label_1070c;
    }
    else {
      v2 = a0[5];
      if (!close((int)v2)) goto label_10709;
      v4 = __errno_location();
    }
    v6 = *v4;
    goto label_1070c;
  }
label_10709:
  v6 = 0;
label_1070c:
  while (!sub_12510(&a0[0xc])) {
    v3 = sub_12560(&a0[0xc]);
    if (0 <= v3)
      close(v3);
  }
  if (a0[10])
    sub_11d70(a0[10]);
  v5 = (void *)a0[0xb];
  if (*(unsigned int *)&a0[9] & 0x102) { // branch-flip
    if (v5)
      sub_11d70(v5);
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

// Function: sub_10840 @ 0x10840
void * sub_10840(long *a0)
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
    *(unsigned short *)((long)v9 + 0x68) = sub_ec70(a0,v9,0);
    return v9;
  }
  v2 = *(short *)((long)v9 + 0x68);
  if (v7 != 2) { // branch-flip
    if (v2 != 1) { // branch-flip
label_108c8:
      do {
        v12 = *(void **)((long)v9 + 0x10);
        if (!v12) {
          v12 = *(void **)((long)v9 + 8);
          if (*(long *)((long)v12 + 0x18)) {
            v6 = *(long *)((long)v12 + 0x48);
            *a0 = (long)v12;
            *(char *)(a0[4] + v6) = 0;
            v12 = (void *)sub_f740(a0,3);
            if (v12) {
              free(v9);
              goto label_10b5a;
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
                    sub_f100(a0,v8,1);
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
                goto label_10933;
              }
              if (*(unsigned short *)((long)v12 + 0x6a) & 1) goto label_10933;
              v8 = sub_f440(a0,*(unsigned long *)((long)v12 + 8),0xffffffff,"..");
            }
            else {
              v8 = sub_f3b0(a0);
            }
            if (v8) {
              *(int *)((long)v12 + 0x40) = *__errno_location();
              *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            }
label_10933:
            if (*(short *)((long)v12 + 0x68) != 2) {
              if (*(int *)((long)v12 + 0x40)) // branch-flip
                *(unsigned short *)((long)v12 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v12 + 0x68) = 6;
                sub_f2f0(a0,v12);
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
          if (sub_f3b0(a0)) {
            *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            return NULL;
          }
          v5 = (void *)a0[0xb];
          if (*(unsigned int *)&a0[9] & 0x102) { // branch-flip
            if (v5)
              sub_11d70(v5);
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
          sub_f1d0(a0);
          v7 = *(short *)((long)v12 + 0x68);
          goto label_10b9b;
        }
      } while (*(short *)((long)v12 + 0x6c) == 4);
      if (*(short *)((long)v12 + 0x6c) == 2) {
        v7 = sub_ec70(a0,v12,1);
        *(short *)((long)v12 + 0x68) = v7;
        if ((v7 == 1) && (v3 = *(unsigned int *)&a0[9], !(v3 & 4))) {
          if (v3 & 0x200) // branch-flip
            v8 = sub_125a0(*(unsigned int *)((long)a0 + 0x2c),".");
          else {
            v8 = sub_e620(".",(v3 & 0x10) << 0xd | 0x90900);
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
      if (v7 == 4) goto label_10a32;
label_10997:
      if ((v3 & 0x40) && (*(long *)((long)v9 + 0x70) != a0[3])) {
label_10a32:
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
        sub_f2f0(a0,v9);
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
          goto label_10c8a;
        }
        v4 = *(unsigned long *)((long)v9 + 0x30);
        if (sub_f440(a0,v9,0xffffffff,v4)) { // branch-flip
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
label_10c8a:
        v12 = (void *)sub_f740(a0,3);
        a0[1] = (long)v12;
        if (!v12) {
          if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20)) {
            if ((*(int *)((long)v9 + 0x40)) && (*(short *)((long)v9 + 0x68) != 4))
              *(unsigned short *)((long)v9 + 0x68) = 7;
            sub_f2f0(a0,v9);
            return v9;
          }
          return NULL;
        }
      }
      a0[1] = 0;
    }
label_10b5a:
    v6 = *(long *)(*(long *)((long)v12 + 8) + 0x48);
    v13 = v6 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v12 + 8) + 0x38) + -1 + v6) != '/')
      v13 = v6;
    v6 = a0[4];
    *(char *)(v13 + v6) = 0x2f;
    memmove(&((char *)(v13 + v6))[1],(void *)((long)v12 + 0x100),*(long *)((long)v12 + 0x60) + 1);
    v7 = *(short *)((long)v12 + 0x68);
    v9 = v12;
label_10b9b:
    *a0 = (long)v9;
    if (v7 == 0xb) goto label_10c18;
  }
  else {
    if (2 <= (unsigned short)(v2 - 0xcU)) {
      if (v2 == 1) goto label_10997;
      goto label_108c8;
    }
    v7 = sub_ec70(a0,v9,1);
    *(short *)((long)v9 + 0x68) = v7;
    if (v7 == 1) {
      v3 = *(unsigned int *)&a0[9];
      if (v3 & 4) {
        *a0 = (long)v9;
        goto label_10baf;
      }
      if (v3 & 0x200) // branch-flip
        v8 = sub_125a0(*(unsigned int *)((long)a0 + 0x2c),".");
      else {
        v8 = sub_e620(".",(v3 & 0x10) << 0xd | 0x90900);
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
      goto label_10b9b;
    }
    *a0 = (long)v9;
    if (v7 != 0xb)
      return v9;
label_10c18:
    if (*(long *)((long)v9 + 0xa0) != 2) {
      if (*(long *)((long)v9 + 0xa0) == 1)
        return v9;
      abort();
    }
    v7 = sub_ec70(a0,v9,0);
    *(short *)((long)v9 + 0x68) = v7;
  }
  if (v7 != 1)
    return v9;
label_10baf:
  if (!*(long *)((long)v9 + 0x58))
    a0[3] = *(long *)((long)v9 + 0x70);
  if (!sub_f240(a0,v9)) {
    v9 = NULL;
    *__errno_location() = 0xc;
  }
  return v9;
}

// Function: sub_10f90 @ 0x10f90
unsigned long sub_10f90(unsigned long a0,long a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)(a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_10fc0 @ 0x10fc0
int * sub_10fc0(long *a0,unsigned int a1) // early-return x2
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
    v7 = (long)sub_f740(a0,v8);
    a0[1] = (long)v7;
  }
  else {
    if (v1 & 0x200) { // branch-flip
      v6 = sub_125a0(*(unsigned int *)((long)a0 + 0x2c),".");
      v9 = v6;
    }
    else {
      v6 = sub_e620(".",(v1 & 0x10) << 0xd | 0x90900);
      v9 = v6;
    }
    if (v9 < 0) {
      a0[1] = 0;
      v7 = NULL;
      return v7;
    }
    v7 = (long)sub_f740(a0,v8);
    a0[1] = (long)v7;
    if (*(unsigned char *)((long)a0 + 0x49) & 2) // branch-flip
      sub_f100(a0,v9,1);
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

// Function: sub_111c0 @ 0x111c0
long sub_111c0(unsigned int a0,long a1,long a2)
{
  long v1; // rax
  long v2; // r13
  
  if (a2) { // branch-flip
    v2 = 0;
    do {
      v1 = sub_14db0(a0,a1,a2);
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

// Function: sub_11240 @ 0x11240
unsigned long sub_11240(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_11260 @ 0x11260
bool sub_11260(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_11270 @ 0x11270
unsigned long sub_11270(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1c620)
    return 1;
  v1 = v2[2];
  if ((((dat_1c634 < v1) && (v1 < dat_1c638)) && (dat_1c63c < v2[3])) && (dat_1c650 <= *v2)) {
    v3 = *v2 + dat_1c634;
    if (((v3 < v2[1]) && (v2[1] <= dat_1c640)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1c620;
  return 0;
}

// Function: sub_11300 @ 0x11300
unsigned long sub_11300(long *a0,long a1,unsigned long *a2,char a3) // early-return
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

// Function: sub_11410 @ 0x11410
unsigned long sub_11410(float a0,unsigned long a1,char a2) // ternary
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3; // rcx
  unsigned long v4; // rsi
  long v5; // rdi
  unsigned long v6;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_1c644 <= a0) {
      v2 = 0;
      return v2;
    }
    a1 = (a0 < dat_1c648) ? (unsigned long)a0 : (long)(a0 - dat_1c648) ^ 0x8000000000000000;
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
          if (v6 <= v4) goto label_114e5;
          v5 += 8;
        } while (v6 % v3);
      }
    }
    else {
      v3 = 3;
label_114e5:
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

// Function: sub_11550 @ 0x11550
unsigned long sub_11550(long *a0,unsigned long *a1,char a2) // return-dupe x2
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
label_11586:
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
        goto label_11586;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}

// Function: sub_116c0 @ 0x116c0
unsigned long sub_116c0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_116d0 @ 0x116d0
unsigned long sub_116d0(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_116e0 @ 0x116e0
unsigned long sub_116e0(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_116f0 @ 0x116f0
unsigned long sub_116f0(unsigned long *a0)
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

// Function: sub_11750 @ 0x11750
bool sub_11750(unsigned long *a0)
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
label_11770:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_11770;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_117c0 @ 0x117c0
void sub_117c0(unsigned long *a0,void *a1)
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
      if ((long *)a0[1] <= v4) goto label_1182d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_1182d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_11920 @ 0x11920
long sub_11920(long *a0,long a1) // return-dupe
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

// Function: sub_11990 @ 0x11990
long sub_11990(unsigned long *a0)
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

// Function: sub_119e0 @ 0x119e0
long sub_119e0(long *a0,long a1)
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

// Function: sub_11a70 @ 0x11a70
void sub_11a70(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_11ac0 @ 0x11ac0
long sub_11ac0(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_11b40 @ 0x11b40
unsigned long sub_11b40(unsigned char *a0,unsigned long a1)
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

// Function: sub_11b80 @ 0x11b80
void sub_11b80(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_11bb0 @ 0x11bb0
unsigned long * sub_11bb0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_11240;
  if (!a3)
    a3 = sub_11260;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x1c620;
  v1[5] = a1;
  if (sub_11270(v1)) {
    v2 = sub_11410(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_11cc0 @ 0x11cc0
void sub_11cc0(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_11d5b;
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
label_11d5b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_11d70 @ 0x11d70
void sub_11d70(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_11e1c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11ddf;
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
label_11ddf:
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
label_11e1c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_11e60 @ 0x11e60
unsigned int sub_11e60(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_11410(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_11550(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_11550(a0,&v2,1)) && (sub_11550(a0,&v2,0))) {
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

// Function: sub_11fe0 @ 0x11fe0
long * sub_11fe0(long a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_11300(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_1206f;
label_120f6:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_120f6;
label_1206f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_11270(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_1c644 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_1c648 <= v5) ? (long)(v5 - dat_1c648) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_11e60(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_11300(a0,a1,&v3,0);
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

// Function: sub_12210 @ 0x12210
unsigned long sub_12210(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_11fe0(a0,a1,&v3);
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

// Function: sub_12270 @ 0x12270
long sub_12270(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_11300(a0,a1,&v7,1);
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
      sub_11270(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1c648 <= v8) ? (long)(v8 - dat_1c648) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11e60(a0,v6)) {
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

// Function: sub_12400 @ 0x12400
void sub_12400(void)
{
  sub_12270(); // tail-call
}

// Function: sub_12410 @ 0x12410
unsigned long sub_12410(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_12430 @ 0x12430
unsigned long sub_12430(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = *a1;
    v2 = *a0;
    return sub_14fc0(v2,v1); // tail-call
  }
  return 0;
}

// Function: sub_12460 @ 0x12460
unsigned long sub_12460(unsigned long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_17aa0(*a0);
  return (v1 ^ a0[1]) % a1;
}

// Function: sub_12490 @ 0x12490
unsigned long sub_12490(unsigned long *a0,unsigned long *a1)
{
  int v1; // eax
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = strcmp((char *)*a0,(char *)*a1);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
  }
  return 0;
}

// Function: sub_124d0 @ 0x124d0
void sub_124d0(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_124f0 @ 0x124f0
void sub_124f0(unsigned int *a0,unsigned int a1)
{
  *(unsigned long *)&a0[5] = 0;
  *(char *)&a0[7] = 1;
  *a0 = a1;
  a0[1] = a1;
  a0[2] = a1;
  a0[3] = a1;
  a0[4] = a1;
}

// Function: sub_12510 @ 0x12510
char sub_12510(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_12520 @ 0x12520
unsigned int sub_12520(long a0,unsigned int a1)
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

// Function: sub_12560 @ 0x12560
unsigned int sub_12560(long a0)
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

// Function: sub_125a0 @ 0x125a0
void sub_125a0(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_15ad0(openat(a0,a1,a2,v1));
}

// Function: sub_12620 @ 0x12620
void * sub_12620(unsigned long a0,unsigned long a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  void *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_125a0(a0,a1,a2 | 0x90900);
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

// Function: sub_12680 @ 0x12680
void sub_12680(char *a0)
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
    dat_235a0 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_12720 @ 0x12720
unsigned int sub_12720(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,unsigned int a4)
{
  unsigned int v1; // eax
  char v2 [4];
  
  if (sub_17570(a0,a1,a4,v2)) // branch-flip
    v1 = 0xfffffffe;
  else {
    v1 = sub_175a0(v2,a2,a3);
    sub_17560(v2);
  }
  return v1;
}

// Function: sub_127a0 @ 0x127a0
unsigned int sub_127a0(unsigned long a0,unsigned int a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0x24
  
  v2 = a2;
  v1 = sub_175a0(&v2,a0,a1);
  sub_17560(&v2);
  return v1;
}

// Function: sub_12800 @ 0x12800
char * sub_12800(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_17ae0();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x1c6a7;
      return (char *)0x1c6a0;
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

// Function: sub_128e0 @ 0x128e0
long * sub_128e0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_12950:
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
label_12df6:
      v2 = 0;
      goto label_12db0;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_12f11;
      }
      v26 = 1;
      v2 = 0;
      v13 = 0;
      v31 = 1;
      v32 = "\'";
      break;
    case 3:
      v2 = 1;
label_12db0:
      v26 = 1;
      v13 = 0;
      a4 = 2;
      v31 = 1;
      v32 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_12f11;
      }
      goto label_12df6;
    case 5:
      if (!v18) goto label_12d51;
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
          v10 = (char *)sub_12800("`",a4);
          v37 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v36 = (char *)v10;
        if (v10 == (long *)"\'") {
          v10 = (char *)sub_12800("\'",a4);
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
label_12a3b:
  do {
label_12a4e:
    v24 = 0;
    a1 = v14;
label_12a58:
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
          if ((a3 < v24 + v31) || (v9 = memcmp(v12,v32,v31), v9)) goto label_133c0;
          if (!v26) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v22 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_12fbc;
                default:
                  goto label_12c93;
                case 7:
                  goto label_12fa7;
                case 8:
                  goto label_12f76;
                case 9:
                  goto label_13043;
                case 10:
                  goto label_12f9d;
                case 0xb:
                  goto label_13070;
                case 0xc:
                  goto label_12f5b;
                case 0xd:
                  goto label_12ead;
                case 0x20:
                  goto label_1307d;
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
                  goto label_12c46;
                case 0x23:
                  goto label_13020;
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
                  goto label_12b78;
                case 0x27:
                  goto label_12e92;
                case 0x3f:
                  goto label_12e51;
                
              }
            }
            v5 = v23;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_12c93;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_12b78;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_13464;
              goto label_133a8;
            }
            if (v8 == 0x7d) goto label_13530;
            if ((char)v8 <= '}') {
              v22 = 0x7b;
              if (v8 == 0x7b) goto label_1338a;
              v22 = 0x7c;
              if (v8 == 0x7c) goto label_133a8;
              goto label_12c93;
            }
            if (v8 != 0x7e) goto label_12c93;
label_134e4:
            if (!v24) {
              v8 = a4 == 2;
              v22 = 0x7e;
              v27 = v3;
              goto label_12d09;
            }
            v22 = 0x7e;
            goto label_134f2;
          }
          goto label_12c7c;
        }
label_133c0:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v22 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v23 = 0;
                if (v8 == 0x7e) goto label_134e4;
                goto label_12c90;
              }
              v22 = 0x7b;
              v23 = 0;
              if (v8 != 0x7b) {
                v22 = 0x7c;
                v23 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_12c93;
                goto label_133a8;
              }
            }
            else {
              v22 = 0x7d;
              v23 = 0;
            }
label_1338a:
            if (a3 == 0xffffffffffffffff) goto label_1353f;
label_13398:
            v5 = v23;
            if (a3 == 1) goto label_13025;
label_133a8:
            v8 = a4 == 2;
            v27 = v3;
            v25 = 0;
            v23 = v5;
            goto label_12d09;
          }
          if (v8 == 0x40) goto label_12c90;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v23 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_12b78;
          v23 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_133a8;
label_13464:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v26)) && (v31)) goto label_1348a;
            v17 = 0x5c;
            goto label_12eb2;
          }
label_12c93:
          v17 = (unsigned char)v19;
          if (v34 != 1) goto label_13213;
label_12ca0:
          v10 = __ctype_b_loc();
          v22 = 1;
          v27 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v28 = (bool)(v27 & v2);
          v27 = !v27;
          goto label_12cee;
        }
        switch(v8) {
          case 0:
            goto label_12fae;
          default:
label_12c90:
            v23 = 0;
            goto label_12c93;
          case 7:
label_12fa7:
            v17 = 0x61;
            goto label_12f87;
          case 8:
label_12f76:
            v17 = 0x62;
            goto label_12f87;
          case 9:
            v23 = 0;
label_13043:
            v22 = 9;
            v17 = 0x74;
            goto label_13050;
          case 10:
label_12f9d:
            v17 = 0x6e;
            break;
          case 0xb:
label_13070:
            v17 = 0x76;
            break;
          case 0xc:
label_12f5b:
            v17 = 0x66;
label_12f87:
            if (v26) {
              v2 = 1;
              goto label_12edf;
            }
            v25 = 0;
            goto label_12bfe;
          case 0xd:
label_12ead:
            v17 = 0x72;
            break;
          case 0x20:
            v23 = 0;
label_1307d:
            v22 = 0x20;
            goto label_12b78;
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
            goto label_12c49;
          case 0x23:
            v22 = 0x23;
            v23 = 0;
            goto label_13025;
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
label_12b78:
            v8 = 0;
            goto label_12b88;
          case 0x27:
            v23 = 0;
            goto label_12e92;
          case 0x3f:
            v23 = 0;
            goto label_12e51;
          
        }
        goto label_12eb2;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_13530:
            v22 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_13398;
label_1353f:
            v5 = v23;
            if (a2[1]) goto label_133a8;
            goto label_13025;
          }
          if ((char)v17 <= '}') {
            v22 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_12c46;
              goto label_12c93;
            }
            goto label_1338a;
          }
          v22 = 0x7e;
          if (v17 == 0x7e) goto label_13025;
          if (v34 == 1) goto label_12ca0;
label_13213:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v22 = 0;
          do {
            v19 = v24 + v22;
            v10 = (long)sub_17b20(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v25 = v27;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_1377b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_13aff;
              goto label_13af5;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_1334d;
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
          goto label_1377b;
        }
        if (v17 == 0x40) goto label_12c93;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_12d00;
        if ((unsigned long)v10 & 0xa4000000) goto label_12c46;
        if (v17 != 0x5c) goto label_12c93;
        if (a4 == 2) {
          if (!v26) goto label_1348a;
          goto label_12c73;
        }
        if (!(bool)(v2 & v26 & v31 != 0)) {
          v22 = 0x5c;
          v17 = 0x5c;
          v23 = 0;
          goto label_13050;
        }
label_1348a:
        v24 += 1;
        v25 = 0;
        v17 = 0x5c;
        v8 = v21;
        goto label_134a0;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v23 = 0;
            v22 = 0;
            if (v35 & 1) goto label_13122;
            goto label_1305b;
          }
label_12fae:
          if (!v26) {
            v23 = 0;
label_12fbc:
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
                goto label_12d33;
              }
label_138c5:
              a0[v14] = 0x5c;
              v13 = v14;
              v21 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v21;
              goto label_138c5;
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
label_138fc:
              v17 = (unsigned char)v22;
              v25 = 0;
              v13 = v14;
              v8 = v27;
              if (!v2 || v28) goto label_12d33;
              goto label_12b88;
            }
            v17 = 0x30;
            v25 = 0;
            v13 = v14;
            v8 = v27;
            goto label_12d33;
          }
          if (a4 != 2) goto label_12c7c;
          goto label_12eeb;
        default:
          goto label_12c93;
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
          goto label_130df;
        case 10:
          v22 = 10;
          v17 = 0x6e;
          goto label_130df;
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
label_130df:
          v8 = a4 == 2 & v26;
          v23 = 0;
          if (!(bool)v8) goto label_13050;
label_12c61:
          a4 = 2;
label_12c73:
          if (!v29) goto label_12c7c;
          goto label_12eeb;
        case 0x20:
          v19 = 0x20;
          goto label_12c49;
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
label_12c46:
          v27 = 0;
label_12c49:
          v22 = v19;
          v25 = v27;
          if ((a4 == 2) && (v26)) goto label_12c61;
          goto label_12d07;
        case 0x23:
label_13020:
          v22 = 0x23;
label_13025:
          if (!v24) {
            v19 = v22;
            goto label_12c49;
          }
label_134f2:
          v8 = a4 == 2;
          v27 = v3;
          v25 = 0;
          goto label_12d09;
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
          goto label_12d00;
        case 0x27:
label_12e92:
          v25 = v27;
          if (a4 != 2) {
            v8 = 0;
            v22 = 0x27;
            goto label_12d09;
          }
          if (!v26) {
            if (a1) { // branch-flip
              v14 = 0;
              v22 = a1;
              if (v33) goto label_1391b;
            }
            else {
label_1391b:
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
            goto label_12d33;
          }
          goto label_12c73;
        case 0x3f:
label_12e51:
          if (a4 == 2) {
            if (v26) goto label_12c73;
            v8 = 0;
            v17 = 0x3f;
            v25 = 0;
            goto label_12d33;
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
                goto label_12d09;
              }
              if (v26) goto label_12c7c;
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
              goto label_138fc;
            }
          }
          v8 = 0;
          v22 = 0x3f;
          v27 = v3;
          v25 = 0;
          goto label_12d09;
        
      }
label_13050:
      if (!v2) {
label_1305b:
        v17 = (unsigned char)v22;
        v25 = 0;
        v8 = 0;
        v27 = v3;
        if (!v26) goto label_12d30;
        goto label_12b88;
      }
label_12eb2:
      v25 = 0;
      goto label_12ec3;
    }
    if (!(bool)(v13 == 0 & v26 & a4 == 2)) {
      v8 = a4 == 2 & (v26 ^ 1U);
      v26 = (bool)(v26 ^ 1U);
      if ((!(bool)v8) || (v26 = (bool)v8, !v3)) {
label_13bbb:
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
        if (a1 || !v33) goto label_13bbb;
        v24 = v33;
        goto label_1364b;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v31 = 1;
      v34 = (unsigned long)v10;
      v32 = "\"";
      if (!(v35 & 2)) goto label_13c6b;
      v3 = 0;
      v21 = 0;
      v33 = 0;
      v2 = v4;
      v26 = v4;
      goto label_12a4e;
    }
label_1334d:
    if (v2) {
label_12eeb:
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
label_12f11:
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
label_1364b:
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
label_13af5:
    if (!a2[(long)v10]) break;
  }
label_13aff:
  v25 = 0;
label_1377b:
  v19 = (unsigned long)v17;
  v27 = v25;
  if (2 <= v22) {
label_13785:
    v20 = 0;
    v22 += v24;
    v14 = v24;
    do {
      v17 = (unsigned char)v19;
      if (v28) {
        v29 = a4 == 2;
        if (v26) goto label_12c73;
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
        if (v22 <= v24) goto label_12c11;
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
          goto label_134a0;
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
label_12cee:
  if (v28) {
    v25 = 0;
    v28 = v2;
    goto label_13785;
  }
label_12d00:
  v22 = v19;
  v25 = v27;
label_12d07:
  v8 = a4 == 2;
  v27 = v3;
label_12d09:
  v17 = (unsigned char)v22;
  v28 = (bool)v8 == 0;
  v3 = v27;
  v8 = 0;
  if ((v28 && v2) || (v8 = 0, v26)) {
label_12b88:
    v17 = (unsigned char)v22;
    v13 = v14;
    if (!v30) goto label_12d33;
    if (!(*(unsigned int *)(v30 + (v22 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_12d33;
  }
  else {
label_12d30:
    v8 = 0;
    v3 = v27;
label_12d33:
    if (!v23) {
      v8 ^= 1;
      v24 += 1;
      v8 &= v21;
label_134a0:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v21 = 0;
        v13 += 2;
      }
      goto label_12c11;
    }
  }
label_12ec3:
  if (v26) {
label_12edf:
    if ((bool)(v2 & a4 == 2)) goto label_12eeb;
label_12c7c:
    v35 &= 0xfffffffd;
    v30 = 0;
    goto label_12950;
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
label_12bfe:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v24 += 1;
label_12c11:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v25)
    v4 = 0;
  goto label_12a58;
label_13122:
  v24 += 1;
  goto label_12a58;
label_13c6b:
  a1 = v33;
label_12d51:
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
  goto label_12a3b;
}

// Function: sub_13d70 @ 0x13d70
void * sub_13d70(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_23080;
    if (dat_23078 <= (int)a0) {
      v10 = (long)dat_23078;
      v9 = (long)(int)((a0 - dat_23078) + 1);
      if (dat_23080 != (unsigned long *)0x23090) // branch-flip
        v6 = (unsigned long *)sub_16f30(dat_23080,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_16f30(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_23090;
        v6[1] = dat_23098;
      }
      dat_23080 = v6;
      memset(&v6[(long)dat_23078 * 2],0,(v10 - dat_23078) * 0x10);
      dat_23078 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_128e0(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x235c0)
        free(v8);
      v8 = (void *)sub_16c90(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_128e0(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_13f60 @ 0x13f60
void sub_13f60(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x236c0;
  sub_17090(a0,0x38);
  *v2 = v1;
}

// Function: sub_13fa0 @ 0x13fa0
unsigned int sub_13fa0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x236c0;
  return *a0;
}

// Function: sub_13fc0 @ 0x13fc0
void sub_13fc0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x236c0;
  *a0 = a1;
}

// Function: sub_13fe0 @ 0x13fe0
unsigned int sub_13fe0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x236c0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_14020 @ 0x14020
unsigned int sub_14020(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x236c0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_14040 @ 0x14040
void sub_14040(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x236c0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_14080 @ 0x14080
void sub_14080(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x236c0;
  v2 = __errno_location();
  v1 = *v2;
  sub_128e0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_14100 @ 0x14100
unsigned long sub_14100(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x236c0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_128e0(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_16c90(v3 + 1);
  sub_128e0(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_141f0 @ 0x141f0
void sub_141f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_14100(a0,a1,0,a2); // tail-call
}

// Function: sub_14200 @ 0x14200
void sub_14200(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_23080;
  if (2 <= dat_23078) {
    v4 = (unsigned long *)((long)dat_23080 + 0x18);
    v1 = (unsigned long *)((long)dat_23080 + (unsigned long)(unsigned int)(dat_23078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x235c0) {
    free(*(void **)((long)v3 + 8));
    dat_23098 = 0x235c0;
    dat_23090 = 0x100;
  }
  if (v3 == (void *)0x23090) {
    dat_23078 = 1;
    return;
  }
  free(v3);
  dat_23078 = 1;
  dat_23080 = (void *)0x23090;
}

// Function: sub_142a0 @ 0x142a0
void sub_142a0(unsigned long a0,unsigned long a1)
{
  sub_13d70(a0,a1,0xffffffffffffffff,0x236c0); // tail-call
}

// Function: sub_142c0 @ 0x142c0
void sub_142c0(void)
{
  sub_13d70(); // tail-call
}

// Function: sub_142d0 @ 0x142d0
void sub_142d0(unsigned long a0)
{
  sub_13d70(0,a0,0xffffffffffffffff,0x236c0); // tail-call
}

// Function: sub_142f0 @ 0x142f0
void sub_142f0(unsigned long a0,unsigned long a1)
{
  sub_13d70(0,a0,a1,0x236c0); // tail-call
}

// Function: sub_14310 @ 0x14310
void sub_14310(unsigned long a0,int a1,unsigned long a2)
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
    sub_13d70(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_143b0 @ 0x143b0
void sub_143b0(unsigned long a0,int a1,unsigned long a2,unsigned long a3)
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
    sub_13d70(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_14440 @ 0x14440
void sub_14440(int a0,unsigned long a1)
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
    sub_13d70(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_144d0 @ 0x144d0
void sub_144d0(int a0,unsigned long a1,unsigned long a2)
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
    sub_13d70(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_14560 @ 0x14560
void sub_14560(unsigned long a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_236c0;
  v4 = dat_236c8;
  v9 = dat_236f0;
  v5 = dat_236d0;
  v6 = dat_236d8;
  v7 = dat_236e0;
  v8 = dat_236e8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_13d70(0,a0,a1,&v3);
}

// Function: sub_14600 @ 0x14600
void sub_14600(unsigned long a0,unsigned char a1)
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
  
  v3 = dat_236c0;
  v4 = dat_236c8;
  v9 = dat_236f0;
  v5 = dat_236d0;
  v6 = dat_236d8;
  v7 = dat_236e0;
  v8 = dat_236e8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_13d70(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_146a0 @ 0x146a0
void sub_146a0(unsigned long a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_236c0;
  v3 = (unsigned int)((unsigned long)dat_236c8 >> 0x20);
  v8 = dat_236f0;
  v4 = dat_236d0;
  v5 = dat_236d8;
  v6 = dat_236e0;
  v7 = dat_236e8;
  v2 = (unsigned int)dat_236c8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_13d70(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_14730 @ 0x14730
void sub_14730(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_236c0;
  v3 = (unsigned int)((unsigned long)dat_236c8 >> 0x20);
  v8 = dat_236f0;
  v4 = dat_236d0;
  v5 = dat_236d8;
  v6 = dat_236e0;
  v7 = dat_236e8;
  v2 = (unsigned int)dat_236c8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_13d70(0,a0,a1,&v1);
}

// Function: sub_147c0 @ 0x147c0
void sub_147c0(unsigned long a0,int a1,unsigned long a2)
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
    sub_13d70(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_14860 @ 0x14860
void sub_14860(unsigned long a0,long a1,long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_236c8;
  v4 = dat_236d0;
  v5 = dat_236d8;
  v8 = dat_236f0;
  v2 = (unsigned int)((unsigned long)dat_236c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_236e0;
  v7 = dat_236e8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_13d70(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_14900 @ 0x14900
void sub_14900(unsigned int a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_236c8;
  v4 = dat_236d0;
  v5 = dat_236d8;
  v8 = dat_236f0;
  v2 = (unsigned int)((unsigned long)dat_236c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_236e0;
  v7 = dat_236e8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_13d70(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_149a0 @ 0x149a0
void sub_149a0(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_236c8;
  v4 = dat_236d0;
  v5 = dat_236d8;
  v8 = dat_236f0;
  v2 = (unsigned int)((unsigned long)dat_236c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_236e0;
  v7 = dat_236e8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_13d70(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_14a40 @ 0x14a40
void sub_14a40(long a0,long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_236c8;
  v4 = dat_236d0;
  v5 = dat_236d8;
  v8 = dat_236f0;
  v2 = (unsigned int)((unsigned long)dat_236c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_236e0;
  v7 = dat_236e8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_13d70(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_14ae0 @ 0x14ae0
void sub_14ae0(void)
{
  sub_13d70(); // tail-call
}

// Function: sub_14af0 @ 0x14af0
void sub_14af0(unsigned long a0,unsigned long a1)
{
  sub_13d70(0,a0,a1,0x23040); // tail-call
}

// Function: sub_14b10 @ 0x14b10
void sub_14b10(unsigned long a0,unsigned long a1)
{
  sub_13d70(a0,a1,0xffffffffffffffff,0x23040); // tail-call
}

// Function: sub_14b30 @ 0x14b30
void sub_14b30(unsigned long a0)
{
  sub_13d70(0,a0,0xffffffffffffffff,0x23040); // tail-call
}

// Function: sub_14b50 @ 0x14b50
int sub_14b50(int a0,char *a1,int a2,char *a3,int a4)
{
  bool v1;
  int v2; // eax
  int *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  char v6 [24];
  char v7 [24];
  unsigned int v8; // stack - 0x150
  unsigned int v9; // stack - 0xc0
  
  v2 = renameat2();
  if (0 <= v2)
    return v2;
  v3 = __errno_location();
  if (*v3 - 0x16U & 0xffffffefU && *v3 != 0x5f)
    return v2;
  v1 = 0;
  if (a4) { // branch-flip
    if (a4 != 1) {
      *v3 = 0x5f;
      return -1;
    }
    if ((!fstatat(a2,a3,v6,0x100)) || (*v3 == 0x4b)) {
      *v3 = 0x11;
      return -1;
    }
    if (*v3 == 2) {
      v1 = 1;
      goto label_14c46;
    }
    v2 = -1; // return-dupe
  }
  else {
label_14c46:
    v4 = strlen(a1);
    v5 = strlen(a3);
    if (((v4) && (v5)) && ((a1[v4 - 1] == '/' || (a3[v5 - 1] == '/')))) {
      if (fstatat(a0,a1,v7,0x100)) {
        v2 = -1;
        return v2;
      }
      if (v1) {
        if ((v8 & 0xf000) != 0x4000) {
          *v3 = 2;
          return -1;
        }
      }
      else if (fstatat(a2,a3,v6,0x100)) { // branch-flip
        if ((*v3 != 2) || ((v8 & 0xf000) != 0x4000)) {
          v2 = -1;
          return v2;
        }
      }
      else {
        if ((v9 & 0xf000) != 0x4000) {
          *v3 = 0x14;
          v2 = -1;
          return v2;
        }
        if ((v8 & 0xf000) != 0x4000) {
          *v3 = 0x15;
          return -1;
        }
      }
    }
    v2 = renameat(a0,a1,a2,a3);
  }
  return v2;
}

// Function: sub_14db0 @ 0x14db0
long sub_14db0(int a0,void *a1,unsigned long a2)
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

// Function: sub_14e20 @ 0x14e20
unsigned long sub_14e20(int a0,unsigned long a1,int a2,unsigned long a3)
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
  
  v1 = (void *)sub_d970(a1);
  v2 = (void *)sub_d970(a3);
  v3 = sub_d9d0(v1);
  v9 = 0;
  if (v3 == sub_d9d0(v2)) {
    v9 = 0;
    if (!memcmp(v1,v2,v3)) {
      v4 = (char *)sub_e480(a1);
      if (fstatat(a0,v4,&v7,0x100)) {
        error(1,*__errno_location(),"%s",v4);
        return v6;
      }
      free(v4);
      v4 = (char *)sub_e480(a3);
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

// Function: sub_14fc0 @ 0x14fc0
void sub_14fc0(unsigned long a0,unsigned long a1)
{
  sub_14e20(0xffffff9c,a0,0xffffff9c,a1); // tail-call
}

// Function: sub_14fe0 @ 0x14fe0
int sub_14fe0(long a0,long a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(a1 + 8);
  v2 = *(unsigned long *)(a0 + 8);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_15000 @ 0x15000
void sub_15000(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_15010 @ 0x15010
char * sub_15010(void *a0,unsigned int a1)
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
  v3 = *(long *)((unsigned long)a1 * 8 + 0x22960);
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
            v16 = (unsigned long *)sub_16f30(v16,&v11,1,0xffffffffffffffff,0x10);
          v15 = v14 + 1;
          v16[v14 * 2] = sub_17130(v10);
          (&v16[v14 * 2])[1] = *v5;
          v14 = v15;
        }
        else {
          if (v12 - v9 <= (long)v1)
            v8 = (char *)sub_16f30(v8,&v12,v1 - (v12 - v9),0x7ffffffffffffffe,1);
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
          v8 = (char *)sub_16c70(v9 + 1);
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
          v6 = (char *)sub_16c70(v9 + 1);
          v8 = v6;
        }
        free(v16);
      }
      else {
        if (v9 == v12)
          v8 = (char *)sub_16ce0(v8,v9 + 1);
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

// Function: sub_15300 @ 0x15300
void * sub_15300(unsigned long a0,unsigned int a1)
{
  void *v1; // rax
  void *v2; // rax
  
  v1 = (void *)sub_17700();
  if (!v1)
    return NULL;
  v2 = (void *)sub_15010(v1,a1);
  if (!closedir(v1))
    return v2;
  free(v2);
  return NULL;
}

// Function: sub_15370 @ 0x15370
unsigned long * sub_15370(unsigned long *a0,unsigned long a1) // early-return
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

// Function: sub_153d0 @ 0x153d0
int * sub_153d0(long *a0)
{
  unsigned long v1;
  int *v2; // rax
  
  v2 = (unsigned long)a0[1];
  v1 = (long)v2 * 2;
  if ((void *)*a0 != &a0[2]) {
    free((void *)*a0);
    v2 = (unsigned long)a0[1];
  }
  if (v2 <= v1) { // branch-flip
    v2 = malloc(v1);
    if (v2) {
      *a0 = (long)v2;
      a0[1] = v1;
      v2 = (int *)0x1;
      return v2;
    }
  }
  else {
    v2 = __errno_location();
    *v2 = 0xc;
  }
  *a0 = (long)&a0[2];
  v2 = NULL;
  a0[1] = 0x400;
  return v2;
}

// Function: sub_15450 @ 0x15450
unsigned long sub_15450(long *a0) // return-dupe
{
  unsigned long v1;
  void *v2;
  void *v3; // rax
  
  v2 = (void *)*a0;
  v1 = a0[1] * 2;
  if (v2 != &a0[2]) {
    if ((unsigned long)a0[1] <= v1) { // branch-flip
      v2 = realloc(v2,v1);
      if (v2) goto label_1548d;
      v2 = (void *)*a0;
    }
    else {
      *__errno_location() = 0xc;
    }
    free(v2);
    *a0 = (long)&a0[2];
    a0[1] = 0x400;
    return 0;
  }
  v3 = malloc(v1);
  if (!v3)
    return 0;
  v2 = memcpy(v3,v2,a0[1]);
label_1548d:
  *a0 = (long)v2;
  a0[1] = v1;
  return 1;
}

// Function: sub_15510 @ 0x15510
int sub_15510(unsigned long a0,unsigned long *a1)
{
  char *v1;
  int v2; // eax
  
  v2 = getfilecon();
  if (v2) { // branch-flip
    if (v2 == 10) {
      v1 = (char *)*a1;
      if (!strcmp(v1,"unlabeled")) {
        v2 = -1;
        freecon(v1);
        *a1 = 0;
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

// Function: sub_15590 @ 0x15590
int sub_15590(unsigned long a0,unsigned long *a1)
{
  char *v1;
  int v2; // eax
  
  v2 = lgetfilecon();
  if (v2) { // branch-flip
    if (v2 == 10) {
      v1 = (char *)*a1;
      if (!strcmp(v1,"unlabeled")) {
        v2 = -1;
        freecon(v1);
        *a1 = 0;
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

// Function: sub_15610 @ 0x15610
int sub_15610(unsigned long a0,unsigned long *a1)
{
  char *v1;
  int v2; // eax
  
  v2 = fgetfilecon();
  if (v2) { // branch-flip
    if (v2 == 10) {
      v1 = (char *)*a1;
      if (!strcmp(v1,"unlabeled")) {
        v2 = -1;
        freecon(v1);
        *a1 = 0;
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

// Function: sub_15690 @ 0x15690
bool sub_15690(int a0)
{
  return a0 != -1;
}

// Function: sub_156a0 @ 0x156a0
unsigned long sub_156a0(char *a0)
{
  char *v1;
  char *v2;
  char v3;
  char *v4;
  
  v3 = *a0;
  v4 = a0;
  do {
    v1 = &v4[1];
    if (v3 != '.')
      return open(a0,0x210000); // tail-call, return-dupe
    v2 = &v4[1];
    v4 = v1;
    if (*v2 != '/') {
      if (!*v2)
        return 0xffffff9c;
      return open(a0,0x210000);
    }
    do {
      v3 = v4[1];
      v4 = &v4[1];
    } while (v3 == '/');
    if (!v3)
      return 0xffffff9c;
  } while( true );
}

// Function: sub_15700 @ 0x15700
int sub_15700(char *a0) // early-return
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

// Function: sub_15780 @ 0x15780
void sub_15780(char *a0)
{
  mkdir(a0,0x1c0); // tail-call
}

// Function: sub_15790 @ 0x15790
void sub_15790(char *a0,unsigned int *a1)
{
  open(a0,*a1 & 0xffffff3c | 0xc2,0x180); // tail-call
}

// Function: sub_157b0 @ 0x157b0
unsigned long sub_157b0(char *a0,int a1,unsigned long a2,void *a3,unsigned long a4)
{
  int v1;
  unsigned long v10; // stack - 0x60
  bool v11;
  int v12; // stack - 0x9c
  int v13; // stack - 0x70
  unsigned long v14; // stack - 0x50
  int *v2; // rax
  unsigned long v3;
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rbx
  char *v7;
  unsigned long v8; // stack - 0x68
  char v9 [8];
  
  v2 = __errno_location();
  v1 = *v2;
  v8 = (unsigned long)&v8 >> 4;
  v11 = a3 == sub_15700;
  v3 = strlen(a0);
  if ((long)a1 + a4 <= v3) {
    v4 = v3 - ((long)a1 + a4);
    if (a4 <= strspn(&a0[v4],"X")) {
      v13 = 0x3a2f8;
      v12 = 0;
      do {
        if (a4) {
          v7 = &a0[v4];
          v3 = v8;
          do {
            if (!v12) {
              if (v11) goto label_15969;
              do {
                clock_gettime(1,v9);
                v8 = (v14 ^ v3) * 0x27bb2ee687b0b0fd + 0xb504f32d;
                v3 = v8;
                while( true ) {
                  if (v8 <= 0xf49998db0aa753ff) {
                    v12 = 9;
                    v11 = 1;
                    goto label_158e4;
                  }
label_15969:
                  if (getrandom(&v10,8,1) != 8) break;
                  v8 = v10;
                  v3 = v10;
                }
              } while( true );
            }
            v12 -= 1;
label_158e4:
            v6 = &v7[1];
            v8 = v3 / 0x3e;
            *v7 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"[v3 % 0x3e];
            v7 = v6;
            v3 = v8;
          } while (v6 != &a0[v4 + a4]);
        }
        v5 = (*a3)(a0,a2);
        if (0 <= (int)v5) {
          *v2 = v1;
          return v5;
        }
        if ((*v2 != 0x11) || (v13 = v13 + -1, !v13))
          return 0xffffffff;
      } while( true );
    }
  }
  *v2 = 0x16;
  return 0xffffffff;
}

// Function: sub_15a30 @ 0x15a30
void sub_15a30(unsigned long a0,unsigned long a1,unsigned int a2,int a3)
{
  unsigned int v1 [3]; // stack - 0xc
  
  v1[0] = a2;
  sub_157b0(a0,a1,v1,*(unsigned long *)((long)a3 * 8 + 0x22980));
}

// Function: sub_15a60 @ 0x15a60
void sub_15a60(unsigned long a0,unsigned long a1,unsigned int a2,int a3)
{
  unsigned int v1; // stack - 0x14
  
  v1 = a2;
  sub_157b0(a0,a1,&v1,*(unsigned long *)((long)a3 * 8 + 0x22980),6);
}

// Function: sub_15ac0 @ 0x15ac0
void sub_15ac0(void)
{
  sub_157b0(); // tail-call
}

// Function: sub_15ad0 @ 0x15ad0
unsigned int sub_15ad0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_17bb0();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_15b30 @ 0x15b30
unsigned long sub_15b30(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_15b40 @ 0x15b40
unsigned long sub_15b40(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_15b50 @ 0x15b50
unsigned int sub_15b50(int a0,char *a1,unsigned long *a2,long a3,unsigned int a4)
{
  unsigned long v1;
  int v10; // edx
  int v11;
  unsigned long v12; // stack - 0x108
  unsigned long v13; // stack - 0xf8
  char v14 [88];
  int v15; // r10d
  int v16; // r9d
  int v17; // eax
  unsigned long v18;
  bool v19; // bpl
  unsigned long v2;
  unsigned long v20; // stack - 0x160
  unsigned long *v21; // stack - 0x158
  int v22; // stack - 0x150
  unsigned int v23; // stack - 0x138
  unsigned int v24; // stack - 0x100
  char v25; // stack - 0xfc
  long v26; // stack - 0xf0
  unsigned long v27; // stack - 0xe8
  long v28; // stack - 0xe0
  unsigned long v29; // stack - 0x80
  bool v3; // al
  long v30; // stack - 0x78
  int v4;
  unsigned int v5;
  long v6;
  unsigned long v7;
  unsigned long *v8;
  int v9;
  
  v1 = a2[0xb];
  v18 = *(unsigned long *)(a3 + 0x58);
  v16 = (int)*(unsigned long *)(a3 + 0x60);
  v19 = v1 == v18;
  a4 &= 1;
  v15 = (int)a2[0xc];
  if (!a4) goto label_15c80;
  v3 = v15 == v16 && v19;
  v20 = (unsigned long)v3;
  if (v3)
    return 0;
  if ((long)v1 < (long)(v18 - 1))
    return 0xffffffff;
  if ((long)v18 < (long)(v1 - 1))
    return a4;
  v6 = dat_23700;
  if (dat_23700) { // branch-flip
label_15be6:
    v8 = dat_236f8;
    if (dat_236f8) { // branch-flip
label_15bf6:
      *v8 = *a2;
      v21 = (unsigned long *)sub_12210(v6);
      if (!v21) {
        if (!dat_23700) goto label_15d47;
        goto label_161a0;
      }
      if (dat_236f8 == v21)
        dat_236f8 = NULL;
    }
    else {
      v8 = malloc(0x10);
      dat_236f8 = v8;
      if (v8) {
        *(unsigned int *)&v8[1] = 2000000000;
        *(char *)((long)v8 + 0xc) = 0;
        goto label_15bf6;
      }
label_161a0:
      v12 = *a2;
      v21 = (unsigned long *)sub_11920(dat_23700,&v12);
      if (!v21) goto label_15d47;
    }
    v22 = *(int *)&v21[1];
    if (*(char *)((long)v21 + 0xc)) {
      v18 &= (long)(int)~(unsigned int)(v22 == 2000000000);
      v16 -= v16 % v22;
      v19 = v1 == v18;
      goto label_15c80;
    }
  }
  else {
    v6 = sub_11bb0(0x10,0,sub_15b30,sub_15b40,dat_22fc0);
    dat_23700 = v6;
    if (v6) goto label_15be6;
label_15d47:
    v21 = &v12;
    v24 = 2000000000;
    v25 = 0;
    v22 = 2000000000;
  }
  v2 = a2[9];
  v10 = (int)a2[10];
  v9 = v15 / 10;
  v11 = v10 / 10;
  v4 = (int)a2[0xe] / 10;
  if (!(v15 % 10) && (!(v10 % 10) && !((int)a2[0xe] % 10))) {
    v7 = v18; // branch-flip
    if (0xb <= v22) {
      v23 = 10;
      if (v9 % 10 || (v4 % 10 || v11 % 10)) // branch-flip
        *(unsigned int *)&v21[1] = 10;
      else {
        v17 = 8;
        v5 = 10;
        do {
          v23 = v5 * 10;
          v11 /= 10;
          v4 /= 10;
          v9 /= 10;
          if ((v22 <= (int)v23) || (v9 % 10 || (v11 % 10 || v4 % 10))) {
            v20 = (unsigned long)(v5 == 200000000);
            *(unsigned int *)&v21[1] = v23;
            v7 = (long)(int)~(unsigned int)(v5 == 200000000) & v18;
            goto label_15fb1;
          }
          v17 -= 1;
          v5 = v23;
        } while (v17);
        if ((v2 | v1 | a2[0xd]) & 1) { // branch-flip
          v23 = 1000000000;
          *(unsigned int *)&v21[1] = 1000000000;
        }
        else {
          *(unsigned int *)&v21[1] = 2000000000;
          v20 = 1;
          v23 = 2000000000;
          v7 = v18 & 0xfffffffffffffffe;
        }
      }
    }
    else {
      v23 = 10;
      *(unsigned int *)&v21[1] = 10;
    }
label_15fb1:
    if ((long)v18 < (long)v1)
      return a4;
    if ((v16 <= v15) && (v19))
      return a4;
    if (((long)v1 < (long)v7) || ((v1 == v7 && (v15 < v16 - v16 % (int)v23))))
      return 0xffffffff;
    v27 = v20 | v1;
    v28 = (long)(int)(v23 / 9 + v15);
    v13 = v2;
    v26 = (long)v10;
    if (utimensat(a0,a1,&v13,0x100))
      return 0xfffffffe;
    v4 = fstatat(a0,a1,v14,0x100);
    if (v4 || (v15 != v30 || v29 != v1)) {
      v27 = v1;
      v28 = (long)v15;
      utimensat(a0,a1,&v13,0x100);
      if (v4)
        return 0xfffffffe;
    }
    v4 = ((unsigned int)v29 & 1) * 1000000000 + (int)v30;
    v5 = v4 * -0x33333333 + 0x19999998;
    if ((v5 >> 1 | (unsigned int)((v5 & 1) != 0) << 0x1f) <= 0x19999998) {
      if (v23 != 10) { // branch-flip
        v9 = 9;
        a4 = 10;
        do {
          v4 /= 10;
          v5 = v4 * -0x33333333 + 0x19999998;
          if (0x19999998 < (v5 >> 1 | (unsigned int)((v5 & 1) != 0) << 0x1f)) break;
          v9 -= 1;
          if (!v9) {
            a4 = 2000000000;
            break;
          }
          a4 *= 10;
        } while (a4 != v23);
      }
      else {
        a4 = 10;
      }
    }
    v18 &= (long)(int)~(unsigned int)(a4 == 2000000000);
    v16 -= v16 % (int)a4;
    v19 = v1 == v18;
  }
  *(unsigned int *)&v21[1] = a4;
  *(char *)((long)v21 + 0xc) = 1;
label_15c80:
  return ((unsigned int)(v16 < v15) - (unsigned int)(v15 < v16) & -(unsigned int)v19) + ((unsigned int)((long)v18 < (long)v1) - (unsigned int)((long)v1 < (long)v18));
}

// Function: sub_16390 @ 0x16390
void sub_16390(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  sub_15b50(0xffffff9c,a0,a1,a2,a3); // tail-call
}

// Function: sub_163b0 @ 0x163b0
void sub_163b0(int a0,int a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  char *v2; // rax
  
  v1 = (void *)sub_17380(a2,a3);
  if (v1) {
    error(a0,a1,"%s",v1);
    free(v1); // tail-call
    return;
  }
  v2 = dcgettext(NULL,"unable to display error message",5);
  error(0,*__errno_location(),v2);
  abort(); // no-return
}

// Function: sub_16430 @ 0x16430
void sub_16430(int a0,int a1,long a2,unsigned int a3,unsigned long a4,unsigned long a5)
{
  void *v1; // rax
  char *v2; // rax
  
  v1 = (void *)sub_17380(a4,a5);
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

// Function: sub_164e0 @ 0x164e0
long sub_164e0(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_1660d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_16646:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x1cde0) + 0x1cde0;
  switch(a5) {
    case 0:
      goto label_16646;
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
      goto label_1660d;
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

// Function: sub_16960 @ 0x16960
void sub_16960(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_164e0(); // tail-call
}

// Function: sub_16980 @ 0x16980
void sub_16980(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_164e0(a0,a1,a2,a3);
}

// Function: sub_16a20 @ 0x16a20
void sub_16a20(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_164e0(a0,a1,a2);
}

// Function: sub_16b00 @ 0x16b00
void sub_16b00(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_16bb0 @ 0x16bb0
unsigned long sub_16bb0(void)
{
  unsigned int v1; // eax
  
  if (dat_23709)
    return (unsigned long)dat_23708;
  v1 = geteuid();
  dat_23709 = 1;
  dat_23708 = v1 == 0;
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_16bf0 @ 0x16bf0
void sub_16bf0(void)
{
  if (aligned_alloc())
    return;
  sub_17150(); // no-return
}

// Function: sub_16c10 @ 0x16c10
void sub_16c10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_17150(); // no-return
}

// Function: sub_16c50 @ 0x16c50
void sub_16c50(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_17150(); // no-return
}

// Function: sub_16c70 @ 0x16c70
void sub_16c70(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_17150(); // no-return
}

// Function: sub_16c90 @ 0x16c90
void sub_16c90(void)
{
  sub_16c50(); // tail-call
}

// Function: sub_16ca0 @ 0x16ca0
void sub_16ca0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_17150(); // no-return
}

// Function: sub_16ce0 @ 0x16ce0
void sub_16ce0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_17150(); // no-return
}

// Function: sub_16d10 @ 0x16d10
void sub_16d10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_17150(); // no-return
}

// Function: sub_16d50 @ 0x16d50
void sub_16d50(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_17150(); // no-return
}

// Function: sub_16d90 @ 0x16d90
void sub_16d90(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_17150(); // no-return
}

// Function: sub_16dc0 @ 0x16dc0
void sub_16dc0(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_17150(); // no-return
}

// Function: sub_16e10 @ 0x16e10
void sub_16e10(void *a0,unsigned long *a1)
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
  sub_17150(); // no-return
}

// Function: sub_16ea0 @ 0x16ea0
void sub_16ea0(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_17150(); // no-return
}

// Function: sub_16f30 @ 0x16f30
void sub_16f30(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= (long)v3) goto label_16fee;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_16fee:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816((long)v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_17150(); // no-return
}

// Function: sub_17030 @ 0x17030
void sub_17030(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_17150(); // no-return
}

// Function: sub_17050 @ 0x17050
void sub_17050(unsigned long a0)
{
  sub_17030(a0,1); // tail-call
}

// Function: sub_17060 @ 0x17060
void sub_17060(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_17150(); // no-return
}

// Function: sub_17080 @ 0x17080
void sub_17080(unsigned long a0)
{
  sub_17060(a0,1); // tail-call
}

// Function: sub_17090 @ 0x17090
void sub_17090(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16c50(a1),a0,a1); // tail-call
}

// Function: sub_170c0 @ 0x170c0
void sub_170c0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16c70(a1),a0,a1); // tail-call
}

// Function: sub_170f0 @ 0x170f0
void sub_170f0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_16c70(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_17130 @ 0x17130
void sub_17130(char *a0)
{
  sub_17090(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_17150 @ 0x17150
void sub_17150(void)
{
  error(dat_23030,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_17190 @ 0x17190
void sub_17190(unsigned long a0,unsigned int a1)
{
  if (sub_10230(a0,a1 | 0x200))
    return;
  if (*__errno_location() != 0x16)
    sub_17150(); // no-return
  __assert_fail("errno != EINVAL","lib/xfts.c",0x29,"xfts_open"); // no-return
}

// Function: sub_171e0 @ 0x171e0
bool sub_171e0(long a0,long a1)
{
  unsigned int v1; // eax
  bool v2; // r8b
  
  v2 = 1;
  v1 = *(unsigned int *)(a0 + 0x48) & 0x11;
  if ((v1 != 0x10) && (v2 = 0, v1 == 0x11))
    return *(long *)(a1 + 0x58) != 0;
  return v2;
}

// Function: sub_17220 @ 0x17220
char * sub_17220(long a0,unsigned long *a1)
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
    v4 = (char *)sub_16c50(v7 + 1);
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
    v9 = (char *)sub_16c50(1);
    v4 = v9;
  }
  *v9 = 0;
  return v4;
}

// Function: sub_17380 @ 0x17380
unsigned long sub_17380(char *a0,unsigned long a1) // early-return
{
  char v1;
  unsigned long v2; // stack - 0x18
  long v3;
  
  v1 = *a0;
  v3 = 0;
  while( true ) {
    if (!v1)
      return sub_17220(v3,a1); // tail-call
    if ((v1 != '%') || (a0[v3 * 2 + 1] != 's')) break;
    v3 += 1;
    v1 = a0[v3 * 2];
  }
  if (0 <= (int)sub_174e0(&v2,a0))
    return v2;
  if (*__errno_location() != 0xc)
    return 0;
  sub_17150(); // no-return
}

// Function: sub_17440 @ 0x17440
bool sub_17440(void)
{
  char *v1;
  int v2; // eax
  long v3; // rax
  unsigned long v4; // stack - 0x18
  void *v5; // stack - 0x20
  bool v6;
  
  v6 = 0;
  v5 = NULL;
  v4 = 0;
  v3 = __getdelim(&v5,&v4,10,stdin);
  if (1 <= v3) {
    v1 = (char *)((long)v5 + v3 + -1);
    if (*v1 == '\n')
      *v1 = '\0';
    v2 = rpmatch(v5);
    v6 = 0 < v2;
  }
  free(v5);
  return v6;
}

// Function: sub_174e0 @ 0x174e0
unsigned long sub_174e0(unsigned long *a0,unsigned long a1,unsigned long a2) // early-return x2
{
  void *v1; // rax
  unsigned long v2; // stack - 0x18
  
  v1 = (void *)sub_17bd0(0,&v2,a1,a2);
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

// Function: sub_17560 @ 0x17560
void sub_17560(void)
{
  return;
}

// Function: sub_17570 @ 0x17570
unsigned long sub_17570(unsigned long a0,unsigned long a1,unsigned int a2,unsigned int *a3)
{
  *a3 = a2;
  return 0;
}

// Function: sub_17580 @ 0x17580
void sub_17580(unsigned long a0,int a1,unsigned int a2)
{
  if (a1 != -1) {
    fchmod(a1,a2); // tail-call
    return;
  }
  chmod(a0); // tail-call
}

// Function: sub_175a0 @ 0x175a0
int sub_175a0(unsigned int *a0,unsigned long a1,int a2)
{
  unsigned int v1;
  
  v1 = *a0;
  if (a2 != -1)
    return -(unsigned int)(fchmod(a2,v1) != 0);
  return -(unsigned int)(chmod(a1) != 0);
}

// Function: sub_175e0 @ 0x175e0
unsigned long sub_175e0(unsigned int *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *a0;
  v3 = sub_177a0(a0);
  if (v1 & 0x20) {
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

// Function: sub_17650 @ 0x17650
void sub_17650(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_17670 @ 0x17670
unsigned long sub_17670(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long v4;
  
  if ((int)a0[3] != 0x95f616)
    __assert_fail("state->magic == 9827862","lib/cycle-check.c",0x3c,"cycle_check"); // no-return
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

// Function: sub_17700 @ 0x17700
void * sub_17700(char *a0)
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
      v2 = sub_17830(v1,0x406,3);
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

// Function: sub_177a0 @ 0x177a0
unsigned long sub_177a0(void *a0) // early-return
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
    if (sub_e740(a0)) {
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

// Function: sub_17830 @ 0x17830
int sub_17830(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_2370c) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_2370c = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_2370c = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_2370c != -1)
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

// Function: sub_17aa0 @ 0x17aa0
unsigned long sub_17aa0(char *a0,unsigned long a1)
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

// Function: sub_17ae0 @ 0x17ae0
char * sub_17ae0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_17b20 @ 0x17b20
unsigned long sub_17b20(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_18cd0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_17bb0 @ 0x17bb0
void sub_17bb0(unsigned long a0)
{
  sub_17830(a0,0,3); // tail-call
}

// Function: sub_17bd0 @ 0x17bd0
int * sub_17bd0(int *a0,unsigned long *a1,int *a2,unsigned long a3)
{
  unsigned long v1;
  unsigned long v10;
  unsigned long v11;
  int *v12;
  int *v13;
  unsigned long v14;
  unsigned long v15;
  int v16;
  char v17 [8];
  char v18 [8];
  long v19; // stack - 0x2c8
  undefined10 v2;
  unsigned long v20 [77];
  char v21 [224];
  unsigned int v22 [2]; // stack - 0x3c0
  int v23; // stack - 0x3c4
  char *v24;
  char *v25; // rsp
  int *v26;
  unsigned long v27;
  unsigned long *v28;
  bool v29; // zf
  char v3;
  unsigned long v30; // stack - 0x420
  int v31; // stack - 0x414
  char *v32; // stack - 0x410
  unsigned long *v33; // stack - 0x408
  int *v34; // stack - 0x400
  long v35; // stack - 0x3f8
  char *v36; // stack - 0x3f0
  int *v37; // stack - 0x3e8
  int *v38; // stack - 0x3e0
  int *v39; // stack - 0x3d8
  unsigned int v4;
  int *v40; // stack - 0x3d0
  char *v41; // stack - 0x3b0
  unsigned long *v42; // stack - 0x2c0
  unsigned long v43; // stack - 0x2b8
  unsigned long v44; // stack - 0x2b0
  long v5;
  void *v6;
  unsigned long *v7;
  char *v8;
  int v9;
  
  v25 = v18;
  v24 = v18;
  v33 = a1;
  v39 = a2;
  v40 = a0;
  if ((int)sub_19050(a2,&v19,v17) <= -1)
    return NULL;
  if ((int)sub_18e20(a3,v17) < 0) {
    if (v42 != v20)
      free(v42);
    if (v41 != v21)
      free(v41);
    *__errno_location() = 0x16;
    return NULL;
  }
  v10 = v43 + 7;
  if (0xfffffffffffffff9 <= v43)
    v10 = 0xffffffffffffffff;
  v11 = v10 + v44;
  if ((!CARRY8(v10,v44)) && (v10 = v11 + 6, v11 <= 0xfffffffffffffff9)) {
    if (v10 <= 3999) {
      for (; v25 != &v18[-(v11 + 0x1d & 0xfffffffffffff000)]; v25 = &v25[-0x1000]) {
        *(unsigned long *)&v25[-8] = *(unsigned long *)&v25[-8];
      }
      v10 = (unsigned long)((unsigned int)(v11 + 0x1d) & 0xff0);
      v5 = -v10;
      v24 = &v25[v5];
      if (v10)
        *(unsigned long *)&v25[-8] = *(unsigned long *)&v25[-8];
      v32 = NULL;
      v36 = (char *)((unsigned long)&v25[v5 + 0xf] & 0xfffffffffffffff0);
label_17cc9:
      v10 = 0;
      if (v40)
        v10 = *v33;
      v35 = 0;
      v26 = (int *)*v42;
      v11 = 0;
      v13 = v39;
      v12 = v40;
      v14 = v11;
      v28 = v42;
      if (v26 == v39) goto label_17ea2;
      do {
        v27 = (long)v26 - (long)v13;
        v11 = v14 + v27;
        if (CARRY8(v14,v27))
          v11 = 0xffffffffffffffff;
        if (v11 <= v10) goto label_17e85;
        if (v10) {
          if (0 <= (long)v10) {
            v10 *= 2;
            goto label_17e26;
          }
label_17d40:
          v38 = v12;
label_17d47:
          *(unsigned long *)&v24[-8] = 0x17d4c;
          v3 = v24[-8];
          v39 = __errno_location(v3);
          v12 = v38;
label_17d5a:
          if ((v12 == v40) || (!v12)) goto label_17d6c;
          goto label_182f0;
        }
        v10 = 0xc;
label_17e26:
        if (v10 < v11)
          v10 = v11;
        if (v10 == 0xffffffffffffffff) goto label_17d40;
        if ((!v12) || (v12 == v40)) {
          v37 = v13;
          v38 = v12;
          v39 = (int *)CONCAT71(v39._1_7_,v12 == v40);
          *(unsigned long *)&v24[-8] = 0x1823c;
          v12 = malloc(v10,v24[-8]);
          if (v12) {
            if ((v14) && ((char)v39)) {
              v39 = v37;
              *(unsigned long *)&v24[-8] = 0x18283;
              v12 = memcpy(v12,v38,v14,v24[-8]);
              v13 = v39;
            }
            else {
              v13 = v37;
            }
            goto label_17e85;
          }
          goto label_17d47;
        }
        v39 = v12;
        v38 = v13;
        *(unsigned long *)&v24[-8] = 0x17e6b;
        v12 = realloc(v12,v10,v24[-8]);
        v26 = v39;
        if (!v12) goto label_182cf;
        v13 = v38;
label_17e85:
        v39 = v12;
        *(unsigned long *)&v24[-8] = 0x17e9b;
        memcpy((void *)((long)v12 + v14),v13,v27,v24[-8]);
        v12 = v39;
label_17ea2:
        do {
          v8 = v41;
          if (v19 == v35) {
            v14 = v11 + 1;
            if (0xffffffffffffffff <= v11) {
              if (v10 == 0xffffffffffffffff) goto label_18b8d;
              goto label_17d40;
            }
            if (v14 <= v10) goto label_18b8d;
            if (v10) { // branch-flip
              if ((long)v10 < 0) goto label_17d40;
              v10 *= 2;
              if (v10 < v14) {
label_18c8e:
                if (v14 == 0xffffffffffffffff) goto label_17d40;
                v10 = v14;
              }
            }
            else {
              if (v14 > 0xc) goto label_18c8e;
              v10 = 0xc;
            }
            v29 = v12 == v40;
            if ((!v12) || (v29)) {
              v39 = v12;
              *(unsigned long *)&v24[-8] = 0x18c49;
              v26 = malloc(v10,v24[-8]);
              v12 = v39;
              if (!v26) goto label_17d40;
              if ((!v11) || (!v29))
                v12 = v26;
              else {
                *(unsigned long *)&v24[-8] = 0x18c74;
                v12 = memcpy(v26,v39,v11,v24[-8]);
              }
            }
            else {
              v39 = v12;
              *(unsigned long *)&v24[-8] = 0x18b7a;
              v12 = realloc(v12,v10,v24[-8]);
              v26 = v39;
              if (!v12) goto label_182cf;
            }
label_18b8d:
            *(char *)((long)v12 + v11) = 0;
            if ((v14 < v10) && (v12 != v40)) {
              v40 = v12;
              *(unsigned long *)&v24[-8] = 0x18bb2;
              v26 = realloc(v12,v14,v24[-8]);
              v12 = v40;
              if (v26)
                v12 = v26;
            }
            v40 = v12;
            if (v32) {
              *(unsigned long *)&v24[-8] = 0x18bd8;
              free(v32,v24[-8]);
            }
            if (v42 != v20) {
              *(unsigned long *)&v24[-8] = 0x18bfe;
              free(v42,v24[-8]);
            }
            if (v41 != v21) {
              *(unsigned long *)&v24[-8] = 0x18c24;
              free(v41,v24[-8]);
            }
            *v33 = v11;
            return v40;
          }
          v5 = v28[10];
          if (*(char *)&v28[9] != '%') { // branch-flip
            if (v5 == -1) {
label_18ca0:
              *(unsigned long *)&v24[-8] = 0x18ca5; // no-return
              abort(v24[-8]);
            }
            v9 = *(int *)&v41[v5 * 0x20];
            v38 = (int *)CONCAT44(v38._4_4_,v9);
            if (*(char *)&v28[9] != 'n') { // branch-flip
              v4 = *(unsigned int *)&v28[2];
              v26 = (int *)&v36[1];
              *v36 = '%';
              if (v4 & 1) {
                v36[1] = '\'';
                v26 = (int *)&v36[2];
              }
              if (v4 & 2) {
                *(char *)v26 = 0x2d;
                v26 = (int *)((long)v26 + 1);
              }
              if (v4 & 4) {
                *(char *)v26 = 0x2b;
                v26 = (int *)((long)v26 + 1);
              }
              if (v4 & 8) {
                *(char *)v26 = 0x20;
                v26 = (int *)((long)v26 + 1);
              }
              if (v4 & 0x10) {
                *(char *)v26 = 0x23;
                v26 = (int *)((long)v26 + 1);
              }
              if (v4 & 0x40) {
                *(char *)v26 = 0x49;
                v26 = (int *)((long)v26 + 1);
              }
              if (v4 & 0x20) {
                *(char *)v26 = 0x30;
                v26 = (int *)((long)v26 + 1);
              }
              v6 = (void *)v28[3];
              if (v6 != (void *)v28[4]) {
                v13 = (int *)((long)(void *)v28[4] - (long)v6);
                v37 = v12;
                v39 = v13;
                *(unsigned long *)&v24[-8] = 0x17f8d;
                memcpy(v26,v6,(unsigned long)v13,v24[-8]);
                v26 = (int *)((long)v26 + (long)v39);
                v12 = v37;
              }
              v6 = (void *)v28[6];
              v13 = v12;
              if (v6 != (void *)v28[7]) {
                v13 = (int *)((long)(void *)v28[7] - (long)v6);
                v37 = v12;
                v39 = v13;
                *(unsigned long *)&v24[-8] = 0x17fc7;
                memcpy(v26,v6,(unsigned long)v13,v24[-8]);
                v26 = (int *)((long)v26 + (long)v39);
                v13 = v37;
              }
              switch(v38._0_4_) {
                case 9:
                case 10:
                  *(char *)v26 = 0x6c;
                  v26 = (int *)((long)v26 + 1);
                  goto label_18009;
                case 7:
                case 8:
                case 0xe:
                case 0x10:
                  *(char *)v26 = 0x6c;
                  v26 = (int *)((long)v26 + 1);
                  break;
                case 0xc:
                  *(char *)v26 = 0x4c;
                  v26 = (int *)((long)v26 + 1);
                
              }
              v3 = *(char *)&v28[9];
              *(char *)((long)v26 + 1) = 0;
              *(char *)v26 = v3;
              if (v28[5] != -1) { // branch-flip
                if (*(int *)&v8[v28[5] * 0x20] != 5) goto label_18ca0;
                v37 = (int *)CONCAT44(v37._4_4_,1);
                v22[0] = *(unsigned int *)((long)&v8[v28[5] * 0x20] + 0x10);
              }
              else {
                v37 = (int *)((unsigned long)v37._4_4_ << 0x20);
              }
              if (v28[8] != -1) {
                if (*(int *)&v8[v28[8] * 0x20] != 5) goto label_18ca0;
                v22[(unsigned long)v37 & 0xffffffff] = *(unsigned int *)((long)&v8[v28[8] * 0x20] + 0x10);
                v37 = (int *)CONCAT44(v37._4_4_,(int)v37 + 1);
              }
              v14 = v11 + 2;
              v12 = v13;
              if (0xfffffffffffffffe <= v11) { // branch-flip
                if (v10 != 0xffffffffffffffff) goto label_17d40;
              }
              else if (v14 > v10) {
                if (v10) { // branch-flip
                  if ((long)v10 < 0) goto label_17d40;
                  v10 *= 2;
                  if (v10 < v14) {
label_186b2:
                    if (v14 == 0xffffffffffffffff) goto label_17d40;
                    v10 = v14;
                  }
                }
                else {
                  if (v14 > 0xc) goto label_186b2;
                  v10 = 0xc;
                }
                if ((!v13) || (v13 == v40)) {
                  v34 = v13;
                  v39 = (int *)CONCAT71(v39._1_7_,v13 == v40);
                  *(unsigned long *)&v24[-8] = 0x188b5;
                  v13 = malloc(v10,v24[-8]);
                  v12 = v34;
                  if (!v13) goto label_17d40;
                  if ((v11) && ((char)v39)) {
                    *(unsigned long *)&v24[-8] = 0x188ee;
                    memcpy(v13,v34,v11,v24[-8]);
                  }
                }
                else {
                  v39 = v13;
                  *(unsigned long *)&v24[-8] = 0x180e2;
                  v13 = realloc(v13,v10,v24[-8]);
                  v12 = v39;
                  if (!v13) goto label_17d40;
                }
              }
              *(char *)((long)v13 + v11) = 0;
              *(unsigned long *)&v24[-8] = 0x180ff;
              v39 = __errno_location(v24[-8]);
              v34 = v26;
              v31 = *v39;
              v12 = v13;
label_18120:
              v23 = -1;
              v27 = v10 - v11;
              *v39 = 0;
              v14 = 0x7fffffff;
              if (v27 <= 0x7fffffff)
                v14 = v27;
              switch((unsigned long)v38 & 0xffffffff) {
                default:
                  goto label_18ca0;
                case 1:
                  v15 = (unsigned long)(unsigned int)(int)(char)v41[v28[10] * 0x20 + 0x10];
                  break;
                case 2:
                  v15 = (unsigned long)(unsigned char)v41[v28[10] * 0x20 + 0x10];
                  break;
                case 3:
                  v15 = (unsigned long)(unsigned int)(int)*(short *)&v41[v28[10] * 0x20 + 0x10];
                  break;
                case 4:
                  v15 = (unsigned long)*(unsigned short *)&v41[v28[10] * 0x20 + 0x10];
                  break;
                case 5:
                case 6:
                case 0xd:
                case 0xe:
                  v15 = (unsigned long)*(unsigned int *)&v41[v28[10] * 0x20 + 0x10];
                  break;
                case 7:
                case 8:
                case 9:
                case 10:
                case 0xf:
                case 0x10:
                case 0x11:
                  v26 = *(int **)&v41[v28[10] * 0x20 + 0x10];
                  v30 = v27;
                  if ((int)v37 == 1) {
                    *(int **)&v24[-8] = &v23;
                    *(int **)&v24[-0x10] = v26;
                    goto label_1883a;
                  }
                  if ((int)v37 != 2) {
                    *(int **)&v24[-0x10] = &v23;
label_18342:
                    *(unsigned long *)&v24[-0x18] = 0x1835f;
                    v30 = v27;
                    v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v26);
                    v27 = v30;
                    goto label_18368;
                  }
                  *(int **)&v24[-0x10] = &v23;
                  *(int **)&v24[-0x18] = v26;
                  goto label_187ee;
                case 0xb:
                  v30 = v27;
                  v1 = *(unsigned long *)&v41[v28[10] * 0x20 + 0x10];
                  if ((int)v37 != 1) { // branch-flip
                    if ((int)v37 != 2) {
                      *(unsigned long *)&v24[-8] = 0x1860b;
                      v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v1,&v23);
                      v27 = v30;
                      goto label_18368;
                    }
                    *(int **)&v24[-8] = &v23;
                    *(unsigned long *)&v24[-0x10] = (unsigned long)v22[1];
                  }
                  else {
                    *(int **)&v24[-0x10] = &v23;
                  }
                  *(unsigned long *)&v24[-0x18] = 0x18a46;
                  v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v1,(unsigned long)v22[0]);
                  v27 = v30;
                  goto label_18368;
                case 0xc:
                  v30 = v27;
                  v2 = *(void *)&v41[v28[10] * 0x20 + 0x10];
                  if ((int)v37 != 1) {
                    if ((int)v37 != 2) {
                      v26 = &v23;
                      *(void *)&v24[-0x10] = v2;
                      goto label_18342;
                    }
                    *(int **)&v24[-0x10] = &v23;
                    *(void *)&v24[-0x20] = v2;
                    *(unsigned long *)&v24[-0x30] = (unsigned long)v22[1];
                    *(unsigned long *)&v24[-0x38] = 0x189e4;
                    v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,(unsigned long)v22[0]);
                    v27 = v30;
                    goto label_18368;
                  }
                  *(int **)&v24[-0x10] = &v23;
                  *(void *)&v24[-0x20] = v2;
                  goto label_187f5;
                
              }
              v30 = v27;
              if ((int)v37 == 1) {
                *(int **)&v24[-8] = &v23;
                *(unsigned long *)&v24[-0x10] = v15;
label_1883a:
                v15 = (unsigned long)v22[0];
label_18482:
                *(unsigned long *)&v24[-0x18] = 0x1849f;
                v30 = v27;
                v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v15);
                v27 = v30;
                if (0 <= v23) goto label_18376;
label_184b6:
                if (!*(char *)((long)v34 + 1)) {
                  if (v9 <= -1) {
                    if (!*v39) {
                      v9 = 0x16;
                      if ((*(unsigned char *)&v28[9] & 0xef) == 99)
                        v9 = 0x54;
                      *v39 = v9;
                    }
                    if ((v12 != v40) && (v12)) {
                      *(unsigned long *)&v24[-8] = 0x18515;
                      free(v12,v24[-8]);
                    }
                    if (v32) {
                      *(unsigned long *)&v24[-8] = 0x18529;
                      free(v32,v24[-8]);
                    }
                    if (v42 != v20) {
                      *(unsigned long *)&v24[-8] = 0x18541;
                      free(v42,v24[-8]);
                    }
                    if (v41 != v21) {
                      *(unsigned long *)&v24[-8] = 0x1855d;
                      free(v41,v24[-8]);
                      return NULL;
                    }
                    return NULL;
                  }
                  goto label_1838f;
                }
                *(char *)((long)v34 + 1) = 0;
                goto label_18120;
              }
              if ((int)v37 != 2) {
                *(int **)&v24[-0x10] = &v23;
                goto label_18482;
              }
              *(int **)&v24[-0x10] = &v23;
              *(unsigned long *)&v24[-0x18] = v15;
label_187ee:
              *(unsigned long *)&v24[-0x20] = (unsigned long)v22[1];
label_187f5:
              *(unsigned long *)&v24[-0x28] = 0x18819;
              v30 = v27;
              v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,(unsigned long)v22[0]);
              v27 = v30;
label_18368:
              if (v23 <= -1) goto label_184b6;
label_18376:
              if (((unsigned long)(long)v23 < v14) && (*(char *)((long)v12 + v11 + (long)v23))) goto label_18ca0;
              v16 = v23;
              if (v23 < v9) {
label_1838f:
                v23 = v9;
                v16 = v9;
              }
              if (v14 <= v16 + 1) {
                if (0x7fffffff <= v27) {
                  if ((v12 != v40) && (v12)) {
                    *(unsigned long *)&v24[-8] = 0x18aae;
                    free(v12,v24[-8]);
                  }
                  if (v32) {
                    *(unsigned long *)&v24[-8] = 0x18ac2;
                    free(v32,v24[-8]);
                  }
                  if (v42 != v20) {
                    *(unsigned long *)&v24[-8] = 0x18ada;
                    free(v42,v24[-8]);
                  }
                  if (v41 != v21) {
                    *(unsigned long *)&v24[-8] = 0x18af2;
                    free(v41,v24[-8]);
                  }
                  *v39 = 0x4b;
                  return NULL;
                }
                v14 = (unsigned long)(v16 + 2);
                if (0 <= (long)v10) { // branch-flip
                  v27 = v10 * 2;
                  if (CARRY8(v14,v11)) goto label_17d5a;
                  v15 = v14 + v11;
                  if (v14 + v11 <= v27)
                    v15 = v27;
                }
                else {
                  if (CARRY8(v14,v11)) {
                    if (v10 != 0xffffffffffffffff) goto label_17d5a;
                    goto label_18120;
                  }
                  v15 = 0xffffffffffffffff;
                  v27 = 0xffffffffffffffff;
                }
                if (v15 > v10) {
                  if (!v10)
                    v27 = 0xc;
                  v10 = v15;
                  if (v15 <= v27)
                    v10 = v27;
                  if (v10 == 0xffffffffffffffff) goto label_17d5a;
                  v29 = v12 == v40;
                  if ((!v12) || (v29)) {
                    *(unsigned long *)&v24[-8] = 0x187b0;
                    v26 = malloc(v10,v24[-8]);
                    if (!v26) goto label_17d5a;
                    if ((!v11) || (!v29))
                      v12 = v26;
                    else {
                      *(unsigned long *)&v24[-8] = 0x187d8;
                      v12 = memcpy(v26,v12,v11,v24[-8]);
                    }
                  }
                  else {
                    *(unsigned long *)&v24[-8] = 0x1842a;
                    v26 = realloc(v12,v10,v24[-8]);
                    if (!v26) goto label_17d5a;
                    v12 = v26;
                  }
                }
                goto label_18120;
              }
              v11 = (long)v16 + v11;
              *v39 = v31;
            }
            else {
              if (5 <= (unsigned int)(v9 - 0x12U)) goto label_18ca0;
              v7 = *(unsigned long **)((long)&v41[v5 * 0x20] + 0x10);
              switch(v9) {
                case 0x12:
                  *(char *)v7 = (char)v11;
                  break;
                case 0x13:
                  *(short *)v7 = (short)v11;
                  break;
                case 0x14:
                  *(int *)v7 = (int)v11;
                  break;
                default:
                  *v7 = v11;
                
              }
            }
          }
          else {
            if (v5 != -1) goto label_18ca0;
            v14 = v11 + 1;
            if (v11 == 0xffffffffffffffff)
              v14 = 0xffffffffffffffff;
            if (v10 < v14) {
              if (v10) { // branch-flip
                if ((long)v10 < 0) goto label_17d40;
                v10 *= 2;
              }
              else {
                v10 = 0xc;
              }
              if (v10 < v14)
                v10 = v14;
              if (v10 == 0xffffffffffffffff) goto label_17d40;
              v29 = v12 == v40;
              if ((!v12) || (v29)) {
                v39 = v12;
                *(unsigned long *)&v24[-8] = 0x1890f;
                v26 = malloc(v10,v24[-8]);
                v12 = v39;
                if (!v26) goto label_17d40;
                if ((!v11) || (!v29))
                  v12 = v26;
                else {
                  *(unsigned long *)&v24[-8] = 0x18942;
                  v12 = memcpy(v26,v39,v11,v24[-8]);
                }
              }
              else {
                v39 = v12;
                *(unsigned long *)&v24[-8] = 0x181e1;
                v12 = realloc(v12,v10,v24[-8]);
                v26 = v39;
                if (!v12) goto label_182cf;
              }
            }
            *(char *)((long)v12 + v11) = 0x25;
            v11 = v14;
          }
          v13 = (int *)v28[1];
          v26 = (int *)v28[0xb];
          v28 = &v28[0xb];
          v35 += 1;
          v14 = v11;
        } while (v26 == v13);
      } while( true );
    }
    if (v10 != 0xffffffffffffffff) {
      v36 = malloc(v10);
      if (v36) {
        v32 = v36;
        v24 = v18;
        goto label_17cc9;
      }
    }
  }
  v39 = __errno_location();
  goto label_17d80;
label_182cf:
  v40 = v26;
  *(unsigned long *)&v24[-8] = 0x182db;
  v3 = v24[-8];
  v39 = __errno_location(v3);
  v12 = v40;
label_182f0:
  *(unsigned long *)&v24[-8] = 0x182f8;
  free(v12,v24[-8]);
label_17d6c:
  if (v32) {
    *(unsigned long *)&v24[-8] = 0x17d80;
    free(v32,v24[-8]);
  }
label_17d80:
  if (v42 != v20) {
    *(unsigned long *)&v24[-8] = 0x17d98;
    free(v42,v24[-8]);
  }
  if (v41 != v21) {
    *(unsigned long *)&v24[-8] = 0x17db0;
    free(v41,v24[-8]);
  }
  *v39 = 0xc;
  return NULL;
}

// Function: sub_18cd0 @ 0x18cd0
bool sub_18cd0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_18d60(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_18d60 @ 0x18d60
unsigned long sub_18d60(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_18e10 @ 0x18e10
void sub_18e10(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_18e20 @ 0x18e20
unsigned long sub_18e20(unsigned int *a0,long *a1) // return-dupe x2
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

// Function: sub_19050 @ 0x19050
unsigned long sub_19050(char *a0,unsigned long *a1,unsigned long *a2) // return-dupe
{
  unsigned char v1;
  unsigned long *v10;
  char *v11;
  int v12;
  unsigned int v13; // eax
  unsigned long v14;
  unsigned long v15;
  unsigned long v16;
  char *v17;
  char *v18;
  char v19;
  unsigned long *v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22;
  unsigned long v23;
  unsigned long v24; // stack - 0x78
  unsigned long v25; // stack - 0x70
  unsigned long v26; // stack - 0x68
  unsigned long *v3;
  char *v4;
  int v5;
  unsigned long *v6; // rax
  unsigned long *v7;
  unsigned long v8;
  unsigned long *v9;
  
  v2 = &a1[4];
  v3 = &a2[2];
  v20 = 7;
  *a1 = 0;
  a1[1] = (unsigned long)v2;
  v8 = 0;
  *a2 = 0;
  a2[1] = (unsigned long)v3;
  v25 = 0;
  v24 = 0;
  v26 = 0;
  v7 = v2;
  v22 = 7;
label_190cc:
  if (!*a0) {
    v7[v8 * 0xb] = (unsigned long)a0;
    a1[2] = v24;
    a1[3] = v25;
    return 0;
  }
  v17 = &a0[1];
  v21 = v22;
  if (*a0 == '%') {
    v23 = 0xffffffffffffffff;
    v7 = &v7[v8 * 0xb];
    *v7 = (unsigned long)a0;
    *(unsigned int *)&v7[2] = 0;
    v7[3] = 0;
    v7[4] = 0;
    v7[5] = 0xffffffffffffffff;
    v7[6] = 0;
    v7[7] = 0;
    v7[8] = 0xffffffffffffffff;
    v7[10] = 0xffffffffffffffff;
    v19 = a0[1];
    if (10 <= (unsigned char)(v19 - 0x30U)) {
label_19177:
      v18 = &v17[1];
      if (v19 == '\'') goto label_19221;
      do {
        switch(v19) {
          case 0x20:
            v13 = (unsigned int)v7[2] | 8;
            v17 = v18;
            break;
          default:
            goto label_191a8;
          case 0x23:
            v13 = (unsigned int)v7[2] | 0x10;
            v17 = v18;
            break;
          case 0x2b:
            v13 = (unsigned int)v7[2] | 4;
            v17 = v18;
            break;
          case 0x2d:
            v13 = (unsigned int)v7[2] | 2;
            v17 = v18;
            break;
          case 0x30:
            v13 = (unsigned int)v7[2] | 0x20;
            v17 = v18;
            break;
          case 0x49:
            v13 = (unsigned int)v7[2] | 0x40;
            v17 = v18;
          
        }
        while( true ) {
          *(unsigned int *)&v7[2] = v13;
          v19 = *v17;
          v18 = &v17[1];
          if (v19 != '\'') break;
label_19221:
          v13 = (unsigned int)v7[2] | 1;
          v17 = v18;
        }
      } while( true );
    }
    v18 = v17;
    do {
      v11 = &v18[1];
      v18 = &v18[1];
    } while ((unsigned char)(*v11 - 0x30U) <= 9);
    v23 = 0xffffffffffffffff;
    if (*v11 != '$') goto label_19177;
    a0 = &a0[2];
    v23 = 0;
    while( true ) {
      v17 = &a0[-1];
      v8 = (unsigned long)(char)(v19 + '\xd0');
      if (0x199999999999999a <= v23) // branch-flip
        v15 = 0xffffffffffffffff;
      else {
        v15 = v23 * 10;
      }
      while( true ) {
        v18 = a0;
        v19 = *v18;
        v23 = v15 + v8;
        v1 = v19 - 0x30;
        if (!CARRY8(v15,v8)) break;
        if (9 < v1) goto label_19660;
        v8 = (unsigned long)(char)v1;
        v15 = 0xffffffffffffffff;
        a0 = &v18[1];
        v17 = v18;
      }
      if (9 < v1) break;
      a0 = &v18[1];
    }
    v23 -= 1;
    if (v23 <= 0xfffffffffffffffd) {
      v19 = v17[2];
      v17 = &v17[2];
      goto label_19177;
    }
    goto label_19660;
  }
  goto label_190c9;
label_191a8:
  if (v19 != '*') { // branch-flip
    v18 = v17;
    if ((unsigned char)(v19 - 0x30U) <= 9) {
      v7[3] = (unsigned long)v17;
      if ((unsigned char)(*v17 - 0x30U) <= 9) {
        do {
          v11 = &v18[1];
          v18 = &v18[1];
        } while ((unsigned char)(*v11 - 0x30U) <= 9);
        v8 = (long)v18 - (long)v17;
        if ((unsigned long)((long)v18 - (long)v17) <= v24)
          v8 = v24;
        v24 = v8;
      }
      v7[4] = (unsigned long)v18;
label_199ec:
      v19 = *v18;
    }
  }
  else {
    v7[3] = (unsigned long)v17;
    v7[4] = (unsigned long)v18;
    v19 = v17[1];
    v8 = 1;
    if (v24)
      v8 = v24;
    if (10 <= (unsigned char)(v19 - 0x30U)) { // branch-flip
label_192d5:
      v7[5] = v26;
      v14 = v26 + 1;
      if (v26 == 0xffffffffffffffff) goto label_19660;
      v15 = v26;
      v26 = v14;
    }
    else {
      v11 = v18;
      do {
        v4 = &v11[1];
        v11 = &v11[1];
      } while ((unsigned char)(*v4 - 0x30U) <= 9);
      if (*v4 != '$') goto label_192d5;
      v17 = &v17[2];
      v15 = 0;
      while( true ) {
        v18 = &v17[-1];
        v14 = (unsigned long)(char)(v19 + '\xd0');
        if (0x199999999999999a <= v15) // branch-flip
          v16 = 0xffffffffffffffff;
        else {
          v16 = v15 * 10;
        }
        while( true ) {
          v15 = v16 + v14;
          v19 = *v17;
          v1 = v19 - 0x30;
          if (!CARRY8(v16,v14)) break;
          if (9 < v1) goto label_19660;
          v14 = (unsigned long)(char)v1;
          v16 = 0xffffffffffffffff;
          v18 = v17;
          v17 = &v17[1];
        }
        if (9 < v1) break;
        v17 = &v17[1];
      }
      v15 -= 1;
      if (0xfffffffffffffffe <= v15) goto label_19660;
      v7[5] = v15;
      v18 = &v18[2];
    }
    v9 = (unsigned long *)a2[1];
    v10 = v9;
    if (v20 <= v15) {
      v20 *= 2;
      if (v20 <= v15)
        v20 = v15 + 1;
      if (v20 >> 0x3b) goto label_198ab;
      if (v3 != v9) // branch-flip
        v10 = realloc(v9,v20 << 5);
      else {
        v10 = malloc(v20 << 5);
      }
      v9 = (unsigned long *)a2[1];
      if (!v10) goto label_198ab;
      if (v3 == v9)
        v10 = memcpy(v10,v3,*a2 << 5);
      a2[1] = (unsigned long)v10;
    }
    v14 = *a2;
    v9 = &v10[v14 * 4];
    if (v14 <= v15) {
      do {
        v6 = v9;
        v14 += 1;
        *(unsigned int *)v6 = 0;
        v9 = &v6[4];
      } while (v14 <= v15);
      *a2 = v14;
      *(unsigned int *)v6 = 0;
    }
    v12 = (int)v10[v15 * 4];
    v24 = v8;
    if (v12) {
      if (v12 != 5) goto label_19664;
      goto label_199ec;
    }
    *(int *)&v10[v15 * 4] = 5;
    v19 = *v18;
  }
  v17 = v18;
  if (v19 != '.') goto label_191c7;
  if (v18[1] != '*') {
    v7[6] = (unsigned long)v18;
    v11 = &v18[1];
    if (10 <= (unsigned char)(v18[1] - 0x30U)) { // branch-flip
      v8 = 1;
      v17 = v11;
    }
    else {
      do {
        v17 = &v11[1];
        v11 = &v11[1];
      } while ((unsigned char)(*v17 - 0x30U) <= 9);
      v8 = (long)v11 - (long)v18;
      v17 = v11;
    }
    v7[7] = (unsigned long)v11;
    v19 = *v11;
    if (v8 <= v25)
      v8 = v25;
    v25 = v8;
    goto label_191c7;
  }
  v17 = &v18[2];
  v7[6] = (unsigned long)v18;
  v7[7] = (unsigned long)v17;
  v19 = v18[2];
  v8 = 2;
  if (2 <= v25)
    v8 = v25;
  if (10 <= (unsigned char)(v19 - 0x30U)) { // branch-flip
label_19415:
    v15 = v7[8];
    if (v7[8] == 0xffffffffffffffff) {
      v7[8] = v26;
      if (v26 == 0xffffffffffffffff) goto label_19660;
      v15 = v26;
      v26 += 1;
    }
  }
  else {
    v11 = v17;
    do {
      v4 = &v11[1];
      v11 = &v11[1];
    } while ((unsigned char)(*v4 - 0x30U) <= 9);
    if (*v4 != '$') goto label_19415;
    v18 = &v18[3];
    v15 = 0;
    while( true ) {
      v17 = &v18[-1];
      v14 = (unsigned long)(char)(v19 + '\xd0');
      if (0x199999999999999a <= v15) // branch-flip
        v16 = 0xffffffffffffffff;
      else {
        v16 = v15 * 10;
      }
      while( true ) {
        v19 = *v18;
        v15 = v16 + v14;
        v1 = v19 - 0x30;
        if (!CARRY8(v16,v14)) break;
        if (9 < v1) goto label_19660;
        v14 = (unsigned long)(char)v1;
        v16 = 0xffffffffffffffff;
        v17 = v18;
        v18 = &v18[1];
      }
      if (9 < v1) break;
      v18 = &v18[1];
    }
    v15 -= 1;
    if (0xfffffffffffffffe <= v15) goto label_19660;
    v7[8] = v15;
    v17 = &v17[2];
  }
  v9 = (unsigned long *)a2[1];
  v10 = v9;
  if (v20 <= v15) {
    v20 *= 2;
    if (v20 <= v15)
      v20 = v15 + 1;
    if (v20 >> 0x3b) goto label_198ab;
    if (v3 != v9) // branch-flip
      v10 = realloc(v9,v20 << 5);
    else {
      v10 = malloc(v20 << 5);
    }
    v9 = (unsigned long *)a2[1];
    if (!v10) goto label_198ab;
    if (v3 == v9)
      v10 = memcpy(v10,v3,*a2 << 5);
    a2[1] = (unsigned long)v10;
  }
  v14 = *a2;
  v9 = &v10[v14 * 4];
  if (v14 <= v15) {
    do {
      v6 = v9;
      v14 += 1;
      *(unsigned int *)v6 = 0;
      v9 = &v6[4];
    } while (v14 <= v15);
    *a2 = v14;
    *(unsigned int *)v6 = 0;
  }
  v12 = (int)v10[v15 * 4];
  v25 = v8;
  if (!v12) {
    *(int *)&v10[v15 * 4] = 5;
    v19 = *v17;
label_191c7:
    v13 = 0;
    v17 = &v17[1];
    if (v19 == 'h') goto label_19294;
    do {
      switch(v19) {
        case 0x4c:
          v13 |= 4;
          break;
        default:
          goto label_19260;
        case 0x5a:
        case 0x6a:
        case 0x6c:
        case 0x74:
        case 0x7a:
          v13 += 8;
        
      }
      while( true ) {
        v19 = *v17;
        v17 = &v17[1];
        if (v19 != 'h') break;
label_19294:
        v13 |= 1 << ((unsigned char)v13 & 1);
      }
    } while( true );
  }
  if (v12 == 5) {
    v19 = *v17;
    goto label_191c7;
  }
  goto label_19664;
label_19260:
  switch(v19) {
    case 0x25:
      goto label_1951e;
    default:
      goto label_19660;
    case 0x41:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x61:
    case 0x65:
    case 0x66:
    case 0x67:
      v12 = 0xc;
      if ((int)v13 <= 0xf)
        v12 = ((v13 & 4) != 0) + 0xb;
      break;
    case 0x43:
      v12 = 0xe;
      v19 = 'c';
      break;
    case 0x53:
      v12 = 0x10;
      v19 = 's';
      break;
    case 0x58:
    case 0x6f:
    case 0x75:
    case 0x78:
      v12 = 10;
      if ((((int)v13 <= 0xf) && (!(v13 & 4))) && ((v12 = 8, (int)v13 <= 7 && (v12 = 2, !(v13 & 2)))))
        v12 = (-(unsigned int)((v13 & 1) == 0) & 2) + 4;
      break;
    case 99:
      v12 = (7 < (int)v13) + 0xd;
      break;
    case 100:
    case 0x69:
      v12 = 9;
      if ((((int)v13 <= 0xf) && (!(v13 & 4))) && ((v12 = 7, (int)v13 <= 7 && (v12 = 1, !(v13 & 2)))))
        v12 = (-(unsigned int)((v13 & 1) == 0) & 2) + 3;
      break;
    case 0x6e:
      v12 = 0x16;
      if (((((int)v13 <= 0xf) && (!(v13 & 4))) && (v12 = 0x15, (int)v13 <= 7)) && (v12 = 0x12, !(v13 & 2)))
        v12 = 0x14 - (v13 & 1);
      break;
    case 0x70:
      v12 = 0x11;
      break;
    case 0x73:
      v12 = (7 < (int)v13) + 0xf;
    
  }
  if (v23 != 0xffffffffffffffff) // branch-flip
    v7[10] = v23;
  else {
    v7[10] = v26;
    if (v26 == 0xffffffffffffffff) {
label_19660:
      v10 = (unsigned long *)a2[1];
      goto label_19664;
    }
    v23 = v26;
    v26 += 1;
  }
  v9 = (unsigned long *)a2[1];
  v10 = v9;
  if (v20 <= v23) {
    v20 *= 2;
    if (v20 <= v23)
      v20 = v23 + 1;
    if (v20 >> 0x3b) goto label_198ab;
    if (v3 != v9) // branch-flip
      v10 = realloc(v9,v20 << 5);
    else {
      v10 = malloc(v20 << 5);
    }
    v9 = (unsigned long *)a2[1];
    if (!v10) goto label_198ab;
    if (v3 == v9)
      v10 = memcpy(v10,v3,*a2 << 5);
    a2[1] = (unsigned long)v10;
  }
  v8 = *a2;
  v9 = &v10[v8 * 4];
  if (v8 <= v23) {
    do {
      v6 = v9;
      v8 += 1;
      *(unsigned int *)v6 = 0;
      v9 = &v6[4];
    } while (v8 <= v23);
    *a2 = v8;
    *(unsigned int *)v6 = 0;
  }
  v5 = (int)v10[v23 * 4];
  if (v5) { // branch-flip
    if (v5 != v12) {
label_19664:
      if (v3 != v10)
        free(v10);
      if (v2 != (unsigned long *)a1[1])
        free((unsigned long *)a1[1]);
      *__errno_location() = 0x16;
      return 0xffffffff;
    }
  }
  else {
    *(int *)&v10[v23 * 4] = v12;
  }
label_1951e:
  *(char *)&v7[9] = v19;
  v8 = *a1;
  v7[1] = (unsigned long)v17;
  v8 += 1;
  *a1 = v8;
  if (v8 >= v22) {
    if ((0 <= (long)v22) && (v21 = v22 * 2, v21 <= 0x2e8ba2e8ba2e8ba)) {
      if (v2 != (unsigned long *)a1[1]) // branch-flip
        v7 = realloc((unsigned long *)a1[1],v22 * 0xb0);
      else {
        v7 = malloc(v22 * 0xb0);
      }
      if (v7) {
        if (v2 == (unsigned long *)a1[1])
          v7 = memcpy(v7,v2,*a1 * 0x58);
        a1[1] = (unsigned long)v7;
        v8 = *a1;
        goto label_190c9;
      }
    }
    v9 = (unsigned long *)a2[1];
label_198ab:
    if (v3 != v9)
      free(v9);
    if (v2 != (unsigned long *)a1[1])
      free((unsigned long *)a1[1]);
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  v7 = (unsigned long *)a1[1];
label_190c9:
  a0 = v17;
  v22 = v21;
  goto label_190cc;
}

// Function: sub_19d70 @ 0x19d70
void sub_19d70(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_23008); // tail-call
}

// Function: _DT_FINI @ 0x19d84
void _DT_FINI(void)
{
  return;
}
