// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_20fd8)
    return;
  (*dat_20fd8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_20c98)(); // jump-as-call
}

// Function: free @ 0x3640
void free(void *a0)
{
  (*dat_20fa8)(); // jump-as-call
}

// Function: localtime_r @ 0x3650
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_20fc0)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x3660
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_20fc8)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x3670
void __cxa_finalize(void)
{
  (*dat_20fe8)(); // jump-as-call
}

// Function: getenv @ 0x3680
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20ca0)(); // jump-as-call
  return v1;
}

// Function: strtoimax @ 0x3690
void strtoimax(void)
{
  (*dat_20ca8)(); // jump-as-call
}

// Function: endmntent @ 0x36a0
int endmntent(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_20cb0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x36b0
void abort(void)
{
  (*dat_20cb8)(); // jump-as-call
}

// Function: __errno_location @ 0x36c0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_20cc0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x36d0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_20cc8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x36e0
void _exit(int a0)
{
  (*dat_20cd0)(); // jump-as-call
}

// Function: strcpy @ 0x36f0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20cd8)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x3700
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_20ce0)(); // jump-as-call
  return v1;
}

// Function: toupper @ 0x3710
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_20ce8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x3720
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20cf0)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3730
void qsort(void)
{
  (*dat_20cf8)(); // jump-as-call
}

// Function: iswcntrl @ 0x3740
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_20d00)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3750
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20d08)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x3760
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_20d10)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3770
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_20d18)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x3780
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_20d20)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x3790
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20d28)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x37a0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20d30)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x37b0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20d38)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x37c0
void regfree(re_pattern_buffer *a0)
{
  (*dat_20d40)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x37d0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_20d48)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x37e0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_20d50)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x37f0
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_20d58)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3800
void __stack_chk_fail(void)
{
  (*dat_20d60)(); // jump-as-call
}

// Function: getopt_long @ 0x3810
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_20d68)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3820
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_20d70)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x3830
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20d78)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x3840
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_20d80)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3850
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20d88)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x3860
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_20d90)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3870
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20d98)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3880
void lseek(void)
{
  (*dat_20da0)(); // jump-as-call
}

// Function: __assert_fail @ 0x3890
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_20da8)(); // jump-as-call
}

// Function: fputs @ 0x38a0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_20db0)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x38b0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20db8)(); // jump-as-call
  return v1;
}

// Function: freopen @ 0x38c0
FILE * freopen(char *a0,char *a1,FILE *a2)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_20dc0)(); // jump-as-call
  return v1;
}

// Function: close @ 0x38d0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_20dc8)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x38e0
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_20dd0)(); // jump-as-call
  return v1;
}

// Function: regcomp @ 0x38f0
int regcomp(re_pattern_buffer *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_20dd8)(); // jump-as-call
  return v1;
}

// Function: fputc @ 0x3900
int fputc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_20de0)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x3910
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20de8)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3920
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_20df0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3930
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_20df8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3940
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20e00)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3950
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20e08)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3960
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_20e10)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x3970
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_20e18)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x3980
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_20e20)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x3990
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20e28)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x39a0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_20e30)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x39b0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20e38)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x39c0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_20e40)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x39d0
void tzset(void)
{
  (*dat_20e48)(); // jump-as-call
}

// Function: fileno @ 0x39e0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20e50)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x39f0
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_20e58)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3a00
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_20e60)(); // jump-as-call
  return v1;
}

// Function: tolower @ 0x3a10
int tolower(int a0)
{
  int v1; // eax
  
  v1 = (*dat_20e68)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x3a20
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_20e70)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3a30
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20e78)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3a40
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20e80)(); // jump-as-call
  return v1;
}

// Function: getmntent @ 0x3a50
void * getmntent(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20e88)(); // jump-as-call
  return v1;
}

// Function: setmntent @ 0x3a60
void * setmntent(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20e90)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3a70
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20e98)(); // jump-as-call
  return v1;
}

// Function: __isoc99_sscanf @ 0x3a80
int __isoc99_sscanf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_20ea0)(); // jump-as-call
  return v1;
}

// Function: regexec @ 0x3a90
int regexec(re_pattern_buffer *a0,char *a1,unsigned long a2,void *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_20ea8)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3aa0
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20eb0)(); // jump-as-call
  return v1;
}

// Function: fchdir @ 0x3ab0
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_20eb8)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3ac0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20ec0)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3ad0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20ec8)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x3ae0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_20ed0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3af0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20ed8)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3b00
void error(int a0,int a1,char *a2,...)
{
  (*dat_20ee0)(); // jump-as-call
}

// Function: open @ 0x3b10
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_20ee8)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x3b20
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_20ef0)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3b30
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_20ef8)(); // jump-as-call
  return v1;
}

// Function: towlower @ 0x3b40
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_20f00)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x3b50
void strtoumax(void)
{
  (*dat_20f08)(); // jump-as-call
}

// Function: fdopendir @ 0x3b60
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_20f10)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x3b70
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_20f18)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x3b80
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_20f20)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x3b90
void __cxa_atexit(void)
{
  (*dat_20f28)(); // jump-as-call
}

// Function: getline @ 0x3ba0
long getline(char **a0,void *a1,FILE *a2)
{
  long v1; // rax
  
  v1 = (*dat_20f30)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x3bb0
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_20f38)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3bc0
void exit(int a0)
{
  (*dat_20f40)(); // jump-as-call
}

// Function: fwrite @ 0x3bd0
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_20f48)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3be0
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20f50)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3bf0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_20f58)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3c00
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_20f60)(); // jump-as-call
  return v1;
}

// Function: hasmntopt @ 0x3c10
char * hasmntopt(void *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20f68)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_makedev @ 0x3c20
void gnu_dev_makedev(void)
{
  (*dat_20f70)(); // jump-as-call
}

// Function: fstat @ 0x3c30
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_20f78)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x3c40
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_20f80)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x3c50
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_20f88)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x3c60
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_20f90)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x3c70
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_20f98)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3c80
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_20fa0)(); // jump-as-call
  return v1;
}

// Function: sub_3c90 @ 0x3c90
void sub_3c90(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_20fb0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_3cc0 @ 0x3cc0
void sub_3cc0(void)
{
  return;
}

// Function: _FINI_0 @ 0x3d30
void _FINI_0(void)
{
  if (!dat_21108) {
    if (dat_20fe8)
      __cxa_finalize(dat_21008);
    sub_3cc0();
    dat_21108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x3d70
void _INIT_0(void)
{
  return;
}

// Function: sub_3d79 @ 0x3d79
void sub_3d79(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nMandatory arguments to long options are mandatory for short options too.\n"),v1);
}

// Function: sub_3daa @ 0x3daa
void sub_3daa(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n"),v1);
}

// Function: sub_3ddb @ 0x3ddb
void sub_3ddb(char *a0)
{
  printf(gettext("\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n"),a0);
}

// Function: sub_3e10 @ 0x3e10
void sub_3e10(char *a0) // ternary
{
  FILE *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  char *v14; // stack - 0x58
  char *v15; // stack - 0x50
  char *v16; // stack - 0x48
  char *v17; // stack - 0x40
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x30
  int v2; // eax
  char *v3;
  char *v4; // stack - 0x98
  char *v5; // stack - 0xb8
  char **v6; // stack - 0xb0
  char *v7; // stack - 0x90
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v4 = "[";
  v7 = "test invocation";
  v8 = "coreutils";
  v9 = "Multi-call invocation";
  v10 = "sha224sum";
  v11 = "sha2 utilities";
  v12 = "sha256sum";
  v13 = "sha2 utilities";
  v14 = "sha384sum";
  v15 = "sha2 utilities";
  v16 = "sha512sum";
  v17 = "sha2 utilities";
  v18 = 0;
  v19 = 0;
  v6 = &v4;
  while ((*v6 && (v3 = *v6, strcmp(a0,v3)))) {
    v6 = &v6[2];
  }
  v5 = a0;
  if (v6[1])
    v5 = v6[1];
  printf(gettext("\n%s online help: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(gettext("Report any translation bugs to <https://translationproject.org/team/>\n"),v1);
  if (strcmp(a0,"[")) // branch-flip
    v3 = a0;
  else {
    v3 = "test";
  }
  printf(gettext("Full documentation <%s%s>\n"),"https://www.gnu.org/software/coreutils/",v3);
  v3 = (v5 != a0) ? "" : " invocation"; // branch-flip
  printf(gettext("or available locally via: info \'(coreutils) %s%s\'\n"),v5,v3);
}

// Function: sub_40a7 @ 0x40a7
void sub_40a7(long a0,char *a1)
{
  sub_10715(a0,a1);
}

// Function: sub_40cc @ 0x40cc
char * sub_40cc(char *a0)
{
  return a0;
}

// Function: sub_40da @ 0x40da
void sub_40da(unsigned long *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0x8000000000000000;
  a0[3] = 0xffffffffffffffff;
}

// Function: sub_411a @ 0x411a
void sub_411a(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  *a0 = a1;
  a0[1] = 1;
  a0[2] = a2;
  a0[3] = a3;
}

// Function: sub_415c @ 0x415c
void sub_415c(unsigned long *a0,long *a1) // return-dupe
{
  long v1;
  long v2;
  unsigned long v3;
  unsigned long v4;
  
  v4 = *a1 + *a0;
  if (v4 < *a0)
    v4 = 0xffffffffffffffff;
  *a0 = v4;
  a0[1] = a0[1] + a1[1];
  v1 = a1[3];
  v2 = a1[2];
  v4 = a0[2];
  v3 = a0[3];
  if (0 > (int)sub_1489d(v4,v3,v2,v1)) {
    v4 = a1[3];
    a0[2] = a1[2];
    a0[3] = v4;
    return;
  }
}

// Function: sub_41ff @ 0x41ff
void sub_41ff(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_211d0;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"),v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Summarize device usage of the set of FILEs, recursively for directories.\n"),v1);
    sub_3d79();
    v1 = stdout;
    fputs_unlocked(gettext("  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_3ddb("DU");
    sub_3daa();
    sub_3e10("du");
  }
  exit(a0); // no-return
}

// Function: sub_43fb @ 0x43fb
unsigned long sub_43fb(struct_11 *a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  
  v1 = sub_6bcf(a0,a2,a1);
  if ((int)v1 <= -1)
    sub_15992(); // no-return
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 != 0);
}

// Function: sub_4441 @ 0x4441
void sub_4441(char *a0,unsigned long a1,unsigned long a2,long *a3) // return-dupe
{
  char *v1; // rax
  char v2 [64];
  unsigned long v3; // stack - 0xa8
  char v4 [24];
  unsigned long v5; // stack - 0xa0
  long *v6; // stack - 0x98
  char *v7; // stack - 0x90
  char *v8; // stack - 0x80
  
  v3 = a1;
  v5 = a2;
  v6 = a3;
  v7 = a0;
  if (sub_14623(a3,&v3,v2)) {
    sub_80da(stdout,v7,v2,v6,v5 & 0xffffffff);
    return;
  }
  v8 = (char *)sub_40a7(v3,v4);
  v1 = (char *)sub_1401b(v8);
  error(0,0,gettext("time %s is out of range"),v1);
  fputs_unlocked(v8,stdout);
}

// Function: sub_4559 @ 0x4559
void sub_4559(unsigned long a0)
{
  FILE *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != 0xffffffffffffffff) // branch-flip
    v2 = (char *)sub_f661(a0,v3,dat_21148,1,dat_21170);
  else {
    v2 = gettext("Infinity");
  }
  fputs_unlocked(v2,v1);
}

// Function: sub_45f2 @ 0x45f2
void sub_45f2(unsigned long *a0,char *a1) // ternary x2
{
  unsigned long v1;
  char v2; // dl
  
  v1 = (dat_2114c) ? a0[1] : *a0; // branch-flip
  sub_4559(v1);
  if (dat_2114d) {
    putchar_unlocked(9);
    sub_4441(dat_21160,a0[2],a0[3],dat_21168);
  }
  v2 = (dat_2113c) ? '\0' : '\n'; // branch-flip
  printf("\t%s%c",a1,v2);
  fflush_unlocked(stdout);
}

// Function: sub_46a8 @ 0x46a8
void sub_46a8(void)
{
  char *v1;
  long v2;
  stat v3; // stack - 0xa8
  long v4; // stack - 0xb8
  
  v4 = sub_11107(0);
  while (v4) {
    if (((!(*(unsigned char *)(v4 + 0x28) & 2)) && (!(*(unsigned char *)(v4 + 0x28) & 1))) && (v1 = *(char **)(v4 + 8), !stat(v1,&v3)))
      sub_43fb(dat_21128,v3._8_8_,v3._0_8_);
    v2 = *(long *)(v4 + 0x30);
    sub_11d9d(v4);
    v4 = v2;
  }
}

// Function: sub_4795 @ 0x4795
unsigned long sub_4795(long *a0)
{
  long *v1;
  int v2; // eax
  long *v3; // stack - 0x20
  
  v1 = (long *)*a0;
  v3 = a0;
  if (!dat_21128) {
    dat_21128 = sub_6936();
    if (!dat_21128)
      sub_15992(); // no-return
    sub_46a8();
  }
  for (; (v3 && (v3 != v1)); v3 = (long *)v3[1]) {
    v2 = sub_6c4b(dat_21128,v3[0xe],v3[0xf]);
    if (1 <= v2)
      return 1;
  }
  return 0;
}

// Function: sub_4831 @ 0x4831
char sub_4831(struct_25 *a0,long *a1)
{
  unsigned short v1;
  unsigned long v10; // rdx
  unsigned long v11; // stack - 0x68
  unsigned long v12; // stack - 0x48
  bool v13;
  char v14 [16];
  char v15; // stack - 0xa6
  bool v16; // stack - 0xa5
  unsigned long v17; // stack - 0xa0
  unsigned long v18; // stack - 0x60
  unsigned long v19; // stack - 0x58
  char v2; // al
  unsigned long v20; // stack - 0x50
  unsigned long v21; // stack - 0x40
  unsigned long v22; // stack - 0x38
  unsigned long v23; // stack - 0x30
  long *v3; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6;
  unsigned long v7; // rax
  unsigned long v8;
  long v9;
  
  v15 = 1;
  v6 = (char *)a1[7];
  v3 = &a1[0xe];
  v1 = *(unsigned short *)&a1[0xd];
  if (v1 != 4) { // branch-flip
    if (v1 != 6) {
      v16 = sub_76f0(dat_21178,v6);
      if (v16 != '\x01') {
        if (v1 == 0xb) {
          sub_c3cb(a0,a1,1);
          if ((long *)sub_bd55(a0) != a1)
            __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
          v1 = *(unsigned short *)&a1[0xd];
        }
        if ((v1 == 10) || (v1 == 0xd)) {
          v6 = (char *)sub_13bd4(4,v6);
          v4 = gettext("cannot access %s");
          error(0,(int)a1[8],v4,v6);
          return 0;
        }
        if (((a0->field_0x48 & 0x40) && (1 <= a1[0xb])) && (a0->field_0x18 != *v3))
          v16 = 1;
      }
      if ((v16) || (((dat_2113a != '\x01' && ((dat_2113b || (((*(unsigned int *)&a1[0x11] & 0xf000) != 0x4000 && (2 <= (unsigned long)a1[0x10])))))) && (v2 = sub_43fb(dat_21120,a1[0xf],*v3), v2 != '\x01')))) {
        if (v1 == 1) {
          sub_c3cb(a0,a1,4);
          if ((long *)sub_bd55(a0) != a1)
            __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
        }
        return 1;
      }
      if (v1 != 7) { // branch-flip
        if (v1 <= 7) {
          if (v1 == 1)
            return 1;
          if (v1 == 2) {
            if ((sub_15a4d(a0,a1)) && (sub_4795(a1) != '\x01')) {
              v6 = (char *)sub_13d66(0,3,v6);
              error(0,0,gettext("WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n"),v6);
              return 0;
            }
            return 1;
          }
        }
      }
      else {
        v4 = (char *)sub_13d66(0,3,v6);
        error(0,(int)a1[8],"%s",v4);
        v15 = 0;
      }
    }
  }
  else {
    v4 = (char *)sub_13bd4(4,v6);
    v5 = gettext("cannot read directory %s");
    error(0,(int)a1[8],v5,v4);
    v15 = 0;
  }
  if (dat_21150) { // branch-flip
    if (dat_21150 != 2) { // branch-flip
      v14 = sub_140ad(v3);
      v10 = SUB168(v14,8);
      v7 = SUB168(v14,0);
    }
    else {
      v14 = sub_14093(v3);
      v10 = SUB168(v14,8);
      v7 = SUB168(v14,0);
    }
  }
  else {
    v14 = sub_140c7(v3);
    v10 = SUB168(v14,8);
    v7 = SUB168(v14,0);
  }
  if (dat_21139) { // branch-flip
    v9 = a1[0x14];
    if (v9 <= -1)
      v9 = 0;
  }
  else {
    v9 = a1[0x16] << 9;
  }
  sub_411a(&v11,v9,v7,v10);
  v8 = a1[0xb];
  v12 = v11;
  v21 = v18;
  v22 = v19;
  v23 = v20;
  if (dat_211a0) { // branch-flip
    if (v8 != dat_21130) {
      if (v8 <= dat_21130) { // branch-flip
        if (v8 != dat_21130 - 1)
          __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
        sub_415c(&v12,(long *)(dat_21130 * 0x40 + dat_211a8));
        if (dat_2113e != '\x01')
          sub_415c(&v12,(long *)(dat_21130 * 0x40 + dat_211a8 + 0x20));
        sub_415c((unsigned long *)(v8 * 0x40 + dat_211a8 + 0x20),(long *)(dat_21130 * 0x40 + dat_211a8));
        sub_415c((unsigned long *)(v8 * 0x40 + dat_211a8 + 0x20),(long *)(dat_21130 * 0x40 + dat_211a8 + 0x20));
      }
      else {
        v17 = dat_21130;
        if (dat_211a0 <= v8) {
          dat_211a8 = sub_15307(dat_211a8,v8,0x80);
          dat_211a0 = v8 * 2;
          v17 = dat_21130;
        }
        while (v17 = v17 + 1, v17 <= v8) {
          sub_40da((unsigned long *)(v17 * 0x40 + dat_211a8));
          sub_40da((unsigned long *)(v17 * 0x40 + dat_211a8 + 0x20));
        }
      }
    }
  }
  else {
    dat_211a0 = v8 + 10;
    dat_211a8 = sub_15838(dat_211a0,0x40);
  }
  dat_21130 = v8;
  if ((dat_2113e != '\x01') || ((v1 != 6 && (v1 != 4))))
    sub_415c((unsigned long *)(v8 * 0x40 + dat_211a8),&v11);
  sub_415c((unsigned long *)0x21180,&v11);
  if (((((v1 == 6) || (v1 == 4)) && (v8 <= dat_21010)) || ((dat_21138 && (v8 <= dat_21010)))) || (!v8)) {
    if (dat_2114c) // branch-flip
      v8 = v21;
    else {
      v8 = v12;
    }
    if (0 <= (long)dat_21140) // branch-flip
      v13 = dat_21140 <= v8;
    else {
      v13 = v8 <= -dat_21140;
    }
    if (v13)
      sub_45f2(&v12,v6);
  }
  return v15;
}

// Function: sub_505b @ 0x505b
bool sub_505b(long *a0,unsigned int a1)
{
  struct_27 *v1; // rax
  long *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  bool v5; // stack - 0x29
  
  v5 = 1;
  if (*a0) {
    v1 = (struct_27 *)sub_159d2(a0,a1,0);
    while( true ) {
      v2 = (long *)sub_bd55(v1);
      if (!v2) break;
      v5 = (sub_4831(v1,v2) & v5) != 0;
    }
    if (*__errno_location()) {
      v3 = sub_13d66(0,3,v1->field_0x20);
      v4 = gettext("fts_read failed: %s");
      error(0,*__errno_location(),v4,v3);
      v5 = 0;
    }
    dat_21130 = 0;
    if (sub_b89b(v1)) {
      v4 = gettext("fts_close failed");
      error(0,*__errno_location(),v4);
      v5 = 0;
    }
  }
  return v5;
}

// Function: main @ 0x5197
int main(int argc,char **argv,char **envp) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in main
{
  unsigned int v1;
  char *v10; // rax
  int v11; // stack - 0x90
  unsigned int v12; // stack - 0x78
  char *v13; // stack - 0x38
  bool v14; // stack - 0x93
  unsigned int v15; // stack - 0x8c
  unsigned int v16; // stack - 0x88
  int v17; // stack - 0x84
  int v18; // stack - 0x80
  int v19; // stack - 0x7c
  bool v2;
  unsigned int v20; // stack - 0x74
  char *v21; // stack - 0x70
  struct_30 *v22; // stack - 0x68
  unsigned long v23; // stack - 0x60
  char *v24; // stack - 0x58
  char **v25; // stack - 0x50
  char *v26; // stack - 0x48
  unsigned long v27; // stack - 0x40
  unsigned long v28; // stack - 0x30
  bool v3;
  unsigned char v4; // al
  int v5; // eax
  int v6; // eax
  int v7; // eax
  char *v8; // rax
  unsigned long v9; // rax
  
  v3 = 0;
  v14 = 1;
  v21 = NULL;
  v15 = 8;
  v16 = 0x10;
  v2 = 0;
  v13 = (char *)sub_40cc(".");
  v28 = 0;
  sub_11fda(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_186c0(sub_6762);
  dat_21178 = sub_6ebe();
  sub_102cd(getenv("DU_BLOCK_SIZE"),(unsigned int *)0x21148,(long *)0x21170);
label_52a5:
  v11 = -1;
  v17 = getopt_long(argc,argv,"0abd:chHklmst:xB:DLPSX:",(option *)0x20680,&v11);
  if (v17 != -1) {
    if (0x87 < v17) {
label_57be:
      v14 = 0;
      goto label_52a5;
    }
    if (v17 <= 0x2f) {
      if (v17 == -0x83) {
        sub_1517a(stdout,"du","GNU coreutils",dat_21018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v17 == -0x82)
        sub_41ff(0); // no-return
      goto label_57be;
    }
    switch(v17) {
      case 0x30:
        dat_2113c = 1;
        break;
      default:
        goto label_57be;
      case 0x42:
        v19 = sub_102cd(optarg,(unsigned int *)0x21148,(long *)0x21170);
        if (v19)
          sub_160c2(v19,v11,(int)(char)v17,0x20680,optarg); // no-return
        v19 = 0;
        break;
      case 0x44:
      case 0x48:
        v16 = 0x11;
        break;
      case 0x4c:
        v16 = 2;
        break;
      case 0x50:
        v16 = 0x10;
        break;
      case 0x53:
        dat_2113e = 1;
        break;
      case 0x58:
        v6 = sub_7dac(sub_77e3,dat_21178,optarg,0x10000000,10);
        if (v6) {
          v9 = sub_13d66(0,3,optarg);
          error(0,*__errno_location(),"%s",v9);
          v14 = 0;
        }
        break;
      case 0x61:
        dat_21138 = '\x01';
        break;
      case 0x62:
        dat_21139 = '\x01';
        dat_21148 = 0;
        dat_21170 = 1;
        break;
      case 99:
        dat_2113d = '\x01';
        break;
      case 100:
        v6 = sub_161e3(optarg,NULL,0,&v12,"");
        if (v6) { // branch-flip
          v9 = sub_1401b(optarg);
          error(0,0,gettext("invalid maximum depth %s"),v9);
          v14 = 0;
        }
        else {
          v3 = 1;
          dat_21010 = CONCAT44(v20,v12);
        }
        break;
      case 0x68:
        dat_21148 = 0xb0;
        dat_21170 = 1;
        break;
      case 0x6b:
        dat_21148 = 0;
        dat_21170 = 0x400;
        break;
      case 0x6c:
        dat_2113a = '\x01';
        break;
      case 0x6d:
        dat_21148 = 0;
        dat_21170 = 0x100000;
        break;
      case 0x73:
        v2 = 1;
        break;
      case 0x74:
        v18 = sub_15b9a(optarg,0,0,0x21140,"kKmMGTPEZY0");
        if (v18)
          sub_160c2(v18,v11,(int)(char)v17,0x20680,optarg); // no-return
        if ((dat_21140) || (*optarg != '-')) break;
        error(1,0,gettext("invalid --threshold argument \'-0\'"));
        goto label_555e;
      case 0x78:
label_555e:
        v15 |= 0x40;
        break;
      case 0x80:
        dat_21139 = '\x01';
        break;
      case 0x81:
        sub_77e3(dat_21178,optarg,0x10000000);
        break;
      case 0x82:
        v21 = optarg;
        break;
      case 0x83:
        dat_21148 = 0x90;
        dat_21170 = 1;
        break;
      case 0x85:
        dat_2114d = '\x01';
        if (optarg) // branch-flip
          dat_21150 = *(unsigned int *)(sub_63c9("--time",optarg,0x209c0,0x194a0,4,dat_21020,1) * 4 + 0x194a0);
        else {
          dat_21150 = 0;
        }
        dat_21168 = sub_141a3(getenv("TZ"));
        break;
      case 0x86:
        dat_21158 = optarg;
        break;
      case 0x87:
        goto label_569e;
      
    }
    goto label_52a5;
  }
  if (v14 != 1)
    sub_41ff(1); // no-return
  if ((dat_21138) && (v2)) {
    error(0,0,gettext("cannot both summarize and show all entries"));
    sub_41ff(1); // no-return
  }
  if (((v2) && (v3)) && (!dat_21010))
    error(0,0,gettext("warning: summarizing is the same as using --max-depth=0"));
  if (((v2) && (v3)) && (dat_21010)) {
    v23 = dat_21010;
    error(0,0,gettext("warning: summarizing conflicts with --max-depth=%lu"),v23);
    sub_41ff(1); // no-return
  }
  if (v2)
    dat_21010 = 0;
  if (dat_2114c) {
    if (dat_21139)
      error(0,0,gettext("warning: options --apparent-size and -b are ineffective with --inodes"));
    dat_21170 = 1;
  }
  if (dat_2114d) {
    if (!dat_21158) {
      dat_21158 = getenv("TIME_STYLE");
      if ((dat_21158) && (strcmp(dat_21158,"locale"))) {
        if (*dat_21158 != '+') { // branch-flip
          while (!strncmp(dat_21158,"posix-",dat_1a750)) {
            dat_21158 = &dat_21158[dat_1a750];
          }
        }
        else {
          v24 = strchr(dat_21158,10);
          if (v24)
            *v24 = '\0';
        }
      }
      else {
        dat_21158 = "long-iso";
      }
    }
    if (*dat_21158 != '+') { // branch-flip
      v1 = *(unsigned int *)(sub_63c9("time style",dat_21158,0x20a00,0x194d0,4,dat_21020,1) * 4 + 0x194d0);
      if (v1 != 2) { // branch-flip
        if (v1 <= 2) {
          if (v1) { // branch-flip
            if (v1 == 1)
              dat_21160 = "%Y-%m-%d %H:%M";
          }
          else {
            dat_21160 = "%Y-%m-%d %H:%M:%S.%N %z";
          }
        }
      }
      else {
        dat_21160 = "%Y-%m-%d";
      }
    }
    else {
      dat_21160 = &dat_21158[1];
    }
  }
  if (v21) { // branch-flip
    if (optind < argc) {
      v8 = (char *)sub_1401b(argv[optind]);
      error(0,0,gettext("extra operand %s"),v8);
      fprintf(stderr,"%s\n",gettext("file operands cannot be combined with --files0-from"));
      sub_41ff(1); // no-return
    }
    if ((strcmp(v21,"-")) && (!sub_abea(v21,"r",stdin))) {
      v9 = sub_13bd4(4,v21);
      v8 = gettext("cannot open %s for reading");
      error(1,*__errno_location(),v8,v9);
      return v5;
    }
    v22 = (struct_30 *)sub_655d(stdin);
    dat_2113b = '\x01';
  }
  else {
    if (argc <= optind) // branch-flip
      v25 = &v13;
    else {
      v25 = &argv[optind];
    }
    v22 = (struct_30 *)sub_6508(v25);
    dat_2113b = !((argc <= optind + 1) && (v16 != 2));
  }
  if (!v22)
    sub_15992(); // no-return
  dat_21120 = sub_6936();
  if (!dat_21120)
    sub_15992(); // no-return
  if ((dat_2113a) || (dat_2113b != '\x01'))
    v15 |= 0x100;
  v15 |= v16;
  while( true ) {
    v2 = 0;
    v26 = (char *)sub_65ca(v22,&v12);
    if (!v26) break;
    if (((v21) && (!strcmp(v21,"-"))) && (!strcmp(v26,"-"))) {
      v8 = (char *)sub_13bd4(4,v26);
      error(0,0,gettext("when reading file names from stdin, no file name of %s allowed"),v8);
      v2 = 1;
    }
    if (!*v26) {
      if (v21) { // branch-flip
        v27 = sub_66b5(v22);
        v8 = gettext("invalid zero-length file name");
        v10 = (char *)sub_13d66(0,3,v21);
        error(0,0,"%s:%lu: %s",v10,v27,v8);
      }
      else {
        error(0,0,"%s",gettext("invalid zero-length file name"));
      }
      v2 = 1;
    }
    if (v2)
      v14 = 0;
    else {
      dat_211b0 = v26;
      v4 = sub_505b((long *)0x211b0,v15);
      v14 = (v4 & v14) != 0;
    }
  }
  if (v12 != 4) {
    if (v12 <= 4) {
      if (v12 == 2) goto label_5f1c;
      if (v12 == 3)
        sub_15992(); // no-return
    }
    __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
  }
  v9 = sub_13d66(0,3,v21);
  v8 = gettext("%s: read error");
  error(0,*__errno_location(),v8,v9);
  v14 = 0;
label_5f1c:
  sub_66f3(v22);
  sub_69c8(dat_21120);
  if (dat_21128)
    sub_69c8(dat_21128);
  if ((v21) && (((ferror_unlocked(stdin) || (v6 = sub_7ea0(stdin), v6)) && (v14)))) {
    v8 = (char *)sub_13bd4(4,v21);
    error(1,0,gettext("error reading %s"),v8);
    return v7;
  }
  if (dat_2113d)
    sub_45f2((unsigned long *)0x21180,gettext("total"));
  return (int)(v14 ^ 1);
label_569e:
  dat_2114c = '\x01';
  goto label_52a5;
}

// Function: sub_601b @ 0x601b
void sub_601b(void)
{
  sub_41ff(1); // no-return
}

// Function: sub_6030 @ 0x6030
long sub_6030(char *a0,long a1,long a2,unsigned long a3) // early-return
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

// Function: sub_6163 @ 0x6163
long sub_6163(char *a0,long a1)
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

// Function: sub_61d9 @ 0x61d9
void sub_61d9(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = gettext("ambiguous argument %s for %s");
  else {
    v1 = gettext("invalid argument %s for %s");
  }
  v2 = sub_13fee(1,a0);
  error(0,0,v1,sub_13b06(0,8,a1),v2);
}

// Function: sub_6272 @ 0x6272
void sub_6272(long a0,long a1,unsigned long a2)
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
      if (memcmp(v4,(void *)(a2 * v3 + a1),a2)) goto label_62f3;
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,", %s",(char *)sub_1401b(v1));
    }
    else {
label_62f3:
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,"\n  - %s",(char *)sub_1401b(v1));
      v4 = (void *)(a1 + a2 * v3);
    }
    v3 += 1;
  } while( true );
}

// Function: sub_63c9 @ 0x63c9
long sub_63c9(char *a0,char *a1,long a2,long a3,unsigned long a4,void *a5,unsigned int a6) // early-return, ternary
{
  long v1; // stack - 0x10
  
  v1 = ((char)a6) ? sub_6030(a1,a2,a3,a4) : sub_6163(a1,a2); // branch-flip
  if (0 <= v1)
    return v1;
  sub_61d9(a0,a1,v1);
  sub_6272(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_6478 @ 0x6478
unsigned long sub_6478(void *a0,long a1,long a2,unsigned long a3)
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

// Function: sub_6508 @ 0x6508
struct_44 * sub_6508(char **a0) // early-return
{
  struct_44 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return NULL;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
  return v1;
}

// Function: sub_655d @ 0x655d
unsigned long * sub_655d(unsigned long a0) // early-return
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

// Function: sub_65ca @ 0x65ca
long sub_65ca(struct_29 *a0,unsigned int *a1) // early-return x2, ternary
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

// Function: sub_66b5 @ 0x66b5
long sub_66b5(struct_30 *a0)
{
  long v1;
  
  if (a0->field_0x0) // branch-flip
    v1 = a0->field_0x8;
  else {
    v1 = a0->field_0x28 - a0->field_0x20 >> 3;
  }
  return v1;
}

// Function: sub_66f3 @ 0x66f3
void sub_66f3(struct_31 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0);
}

// Function: sub_672e @ 0x672e
void sub_672e(unsigned long a0)
{
  dat_211c0 = a0;
}

// Function: sub_6748 @ 0x6748
void sub_6748(char a0)
{
  dat_211c8 = a0;
}

// Function: sub_6762 @ 0x6762
void sub_6762(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_1777f(stdout);
  if (v1) {
    if (dat_211c8 == '\x01') {
      if (*__errno_location() == 0x20) goto label_6833;
    }
    v2 = gettext("write error");
    if (dat_211c0) { // branch-flip
      v3 = (char *)sub_13d15(dat_211c0);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_21028); // no-return
  }
label_6833:
  v1 = sub_1777f(stderr);
  if (v1)
    _exit(dat_21028); // no-return
}

// Function: sub_685a @ 0x685a
unsigned long sub_685a(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_68c9 @ 0x68c9
unsigned long sub_68c9(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_68ff @ 0x68ff
void sub_68ff(void *a0)
{
  sub_e5e1(*(struct_14 **)((long)a0 + 8));
  free(a0);
}

// Function: sub_6936 @ 0x6936
long * sub_6936(void)
{
  long *v1; // rax
  
  v1 = malloc(0x18);
  if (v1) {
    *v1 = sub_e361(0xb,NULL,sub_685a,sub_68c9,sub_68ff);
    if (!*v1) {
      free(v1);
      return NULL;
    }
    v1[1] = 0;
    v1[2] = 0;
  }
  return v1;
}

// Function: sub_69c8 @ 0x69c8
void sub_69c8(unsigned long *a0)
{
  sub_e5e1((struct_14 *)*a0);
  if (a0[1])
    sub_105e6((struct_32 *)a0[1]);
  free((void *)a0[2]);
  free(a0);
}

// Function: sub_6a23 @ 0x6a23
unsigned long sub_6a23(unsigned long a0,unsigned long a1)
{
  return a0 % a1;
}

// Function: sub_6a45 @ 0x6a45
long sub_6a45(struct_11 *a0,long a1) // early-return x2
{
  long *v1; // rax
  long *v2; // stack - 0x18
  
  v2 = a0->field_0x10;
  if (v2) { // branch-flip
    if (a1 == *v2)
      return v2[1];
  }
  else {
    v2 = malloc(0x10);
    a0->field_0x10 = v2;
    if (!v2)
      return 0;
  }
  *v2 = a1;
  v1 = (long *)sub_f0a2(a0->field_0x0,v2);
  if (v1) {
    if (v1 != v2) // branch-flip
      v2[1] = v1[1];
    else {
      a0->field_0x10 = NULL;
      v2[1] = sub_e361(0x3fd,NULL,sub_6a23,NULL,0);
    }
    return v2[1];
  }
  return 0;
}

// Function: sub_6b45 @ 0x6b45
unsigned long sub_6b45(struct_12 *a0,unsigned long a1) // early-return
{
  unsigned long v1; // rax
  
  if ((a1) && (a1 <= 0x7ffffffffffffffe))
    v1 = a1;
  else {
    if (!a0->field_0x8) {
      v1 = sub_1054d(0x7fffffffffffffff);
      a0->field_0x8 = v1;
      if (!a0->field_0x8) {
        v1 = 0xffffffffffffffff;
        return v1;
      }
    }
    v1 = sub_10624((unsigned long *)a0->field_0x8,a1);
  }
  return v1;
}

// Function: sub_6bcf @ 0x6bcf
unsigned long sub_6bcf(struct_11 *a0,long a1,unsigned long a2) // early-return x2
{
  char *v1; // rax
  long v2; // rax
  
  v1 = (char *)sub_6a45(a0,a1);
  if (!v1)
    return 0xffffffff;
  v2 = sub_6b45(a0,a2);
  if (v2 == -1)
    return 0xffffffff;
  return sub_ed31(v1,v2,NULL);
}

// Function: sub_6c4b @ 0x6c4b
unsigned long sub_6c4b(struct_11 *a0,long a1,unsigned long a2) // early-return x2
{
  struct_6 *v1; // rax
  long v2; // rax
  
  v1 = (struct_6 *)sub_6a45(a0,a1);
  if (!v1)
    return 0xffffffff;
  v2 = sub_6b45(a0,a2);
  if (v2 == -1)
    return 0xffffffff;
  return (unsigned long)(sub_dcac(v1,v2) != 0);
}

// Function: sub_6ccb @ 0x6ccb
void sub_6ccb(struct_12 *a0,unsigned long a1)
{
  struct_5 *v1; // rax
  
  v1 = (struct_5 *)sub_1535a(0x10);
  v1->field_0x8 = a1;
  v1->field_0x0 = a0->field_0x8;
  a0->field_0x8 = v1;
}

// Function: sub_6d17 @ 0x6d17
unsigned long sub_6d17(char *a0,unsigned int a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_6d17
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
label_6dd0:
      if (a1 & 0x8000000)
        return 1;
    }
    if (v2 <= '}') {
      if (v2 == '{') goto label_6dd0;
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
                  if (v4 & 0x2180) goto label_6dd0;
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

// Function: sub_6e56 @ 0x6e56
void sub_6e56(char *a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_6e56
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

// Function: sub_6ebe @ 0x6ebe
void sub_6ebe(void)
{
  sub_157f2(0x10);
}

// Function: sub_6ed2 @ 0x6ed2
void sub_6ed2(unsigned char *a0,unsigned long a1)
{
  sub_df79(a0,a1);
}

// Function: sub_6f03 @ 0x6f03
unsigned long sub_6f03(char *a0,unsigned long a1)
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
    sub_10c3a(v3);
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

// Function: sub_706b @ 0x706b
unsigned long sub_706b(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_70a9 @ 0x70a9
unsigned long sub_70a9(unsigned char *a0,unsigned char *a1)
{
  unsigned long v1; // rax
  
  v1 = sub_10853(a0,a1);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}

// Function: sub_70e7 @ 0x70e7
void sub_70e7(void *a0)
{
  free(a0);
}

// Function: sub_7106 @ 0x7106
void sub_7106(unsigned long *a0,int a1,unsigned int a2) // ternary x2
{
  struct_34 *v1; // rax
  void *v2; // rax
  void *v3; // rdx
  
  v1 = (struct_34 *)sub_157f2(0x28);
  v1->field_0x8 = a1;
  v1->field_0xc = a2;
  if (!a1) {
    v3 = (a2 & 0x10) ? sub_70a9 : sub_706b; // branch-flip
    v2 = (a2 & 0x10) ? sub_6f03 : sub_6ed2; // branch-flip
    v1->field_0x10 = sub_e361(0,NULL,v2,v3,sub_70e7);
  }
  v1->field_0x0 = *a0;
  *a0 = v1;
}

// Function: sub_71c2 @ 0x71c2
void sub_71c2(void *a0)
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
    sub_e5e1(*(unsigned long *)((long)a0 + 0x10));
  }
  free(a0);
}

// Function: sub_728a @ 0x728a
void sub_728a(struct_35 *a0)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x28
  unsigned long *v3; // stack - 0x20
  
  v2 = a0->field_0x0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_71c2(v2);
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

// Function: sub_7320 @ 0x7320
unsigned long sub_7320(unsigned char *a0,unsigned char *a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  unsigned char *v3; // rax
  char *v4; // rax
  unsigned int v5; // stack - 0x28
  unsigned char *v6; // stack - 0x20
  
  if (a2 & 8) { // branch-flip
    if (a2 & 0x10) { // branch-flip
      v3 = (unsigned char *)sub_15961(a1);
      v6 = v3;
      while( true ) {
        v4 = strchr((char *)v6,0x2f);
        if (v4)
          *v4 = '\0';
        v1 = sub_10853(a0,v3);
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
    v2 = sub_10853(a0,a1);
  else {
    v2 = CONCAT44(dat_4,strcmp((char *)a0,(char *)a1));
  }
  return v2;
}

// Function: sub_745b @ 0x745b
bool sub_745b(unsigned long a0,char *a1,unsigned int a2)
{
  void *v1;
  bool v2; // stack - 0x19
  char *v3; // stack - 0x18
  
  if (a2 & 0x10000000) // branch-flip
    v1 = dat_20fd0;
  else {
    v1 = sub_7320;
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

// Function: sub_7523 @ 0x7523
unsigned long sub_7523(struct_16 *a0,char *a1)
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
    v5 = (unsigned long)(sub_745b(v2,a1,v1) != '\0');
  }
  return v5;
}

// Function: sub_759b @ 0x759b
unsigned long sub_759b(char *a0,char *a1)
{
  long v1;
  long v2;
  long v3; // stack - 0x20
  
  v1 = *(long *)&a0[0x20];
  v2 = *(long *)&a0[0x10];
  v3 = 0;
  while( true ) {
    if (v1 <= v3)
      return 0;
    if (sub_7523((struct_16 *)(v3 * 0x48 + v2),a1)) break;
    v3 += 1;
  }
  return 1;
}

// Function: sub_761d @ 0x761d
unsigned long sub_761d(char *a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1;
  struct_6 *v2;
  char *v3; // rax
  char *v4; // stack - 0x38
  
  v1 = *(unsigned int *)&a0[0xc];
  v2 = *(struct_6 **)&a0[0x10];
  v4 = a1;
  while( true ) {
    strcpy(a2,v4);
    while( true ) {
      if (sub_dcac(v2,a2))
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

// Function: sub_76f0 @ 0x76f0
char sub_76f0(long *a0,char *a1) // early-return
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
      v1 = sub_759b(v4,a1);
    else {
      if (!v5) {
        v2 = (unsigned long)strlen(a1);
        v2 = (unsigned long)((long)v2 + 1);
        v2 = (char *)sub_1535a(v2);
        v5 = v2;
      }
      v1 = sub_761d(v4,a1,v5);
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

// Function: sub_77e3 @ 0x77e3
void sub_77e3(long *a0,char *a1,unsigned int a2) // return-dupe
{
  long v1;
  int v10; // stack - 0x4c
  unsigned long v11; // stack - 0x40
  void *v2;
  long v3;
  char *v4;
  unsigned int v5; // eax
  long *v6; // rax
  unsigned int *v7; // rax
  char *v8; // rax
  char *v9; // stack - 0x68
  
  if ((!(a2 & 0x18000000)) || (!sub_6d17(a1,a2))) {
    if (((!*a0) || (*(int *)(*a0 + 8))) || ((*(unsigned int *)(*a0 + 0xc) ^ a2) & 0x60000018))
      sub_7106(a0,0,a2);
    v1 = *a0;
    v8 = (char *)sub_15961(a1);
    if ((a2 & 0x10000002) == 0x10000000)
      sub_6e56(v8);
    v4 = *(char **)(v1 + 0x10);
    if ((char *)sub_f0a2(v4,v8) == v8)
      return;
    free(v8);
    return;
  }
  if ((!*a0) || ((*(int *)(*a0 + 8) != 1 || ((*(unsigned int *)(*a0 + 0xc) ^ a2) & 0x20000000))))
    sub_7106(a0,1,a2);
  v1 = *a0;
  v6 = (long *)(v1 + 0x10);
  if (*(long *)(v1 + 0x20) == *(long *)(v1 + 0x18)) {
    v2 = (void *)*v6;
    *v6 = sub_15636(v2,(long *)(v1 + 0x18),1,-1,0x48);
  }
  v3 = *(long *)(v1 + 0x20);
  *(long *)(v1 + 0x20) = v3 + 1;
  v7 = (unsigned int *)(v3 * 0x48 + *v6);
  *v7 = a2;
  if (a2 & 0x8000000) {
    v5 = (int)a2 >> 3 & 2U | 9;
    if (a2 & 8) { // branch-flip
      for (v11 = strlen(a1); (1 <= (long)v11 && (a1[v11 - 1] == '/')); v11 = v11 - 1) {
      }
      if (v11) { // branch-flip
        v8 = (char *)sub_15380(v11 + 7);
        memcpy(v8,a1,v11);
        strcpy(&v8[v11],"(/.*)?");
        v10 = regcomp((re_pattern_buffer *)&v7[2],v8,v5);
        free(v8);
      }
      else {
        v10 = 1;
      }
    }
    else {
      v10 = regcomp((re_pattern_buffer *)&v7[2],a1,v5);
    }
    if (!v10)
      return;
    *(long *)(v1 + 0x20) = *(long *)(v1 + 0x20) + -1;
    return;
  }
  v9 = a1;
  if (a2 & 0x4000000) {
    v9 = (char *)sub_15961(a1);
    sub_6ccb(a0,v9);
  }
  *(char **)&v7[2] = v9;
}

// Function: sub_7b1c @ 0x7b1c
unsigned long sub_7b1c(void *a0,struct_12 *a1,FILE *a2,unsigned int a3,unsigned char a4,unsigned long a5) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_7b1c
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
      v5 = (void *)sub_15636(v5,&v3,1,-1,1);
    v1 = v8 + 1;
    *(char *)((long)v5 + v8) = (char)v2;
    v8 = v1;
  }
  if (ferror(a2))
    v4 = *__errno_location();
  v5 = (unsigned char *)sub_1540f(v5,v8 + 1);
  v5[v8] = a4;
  v2 = ((v8) && (a4 != v5[v8 + -1]));
  v10 = &v5[v2 + v8];
  sub_6ccb(a1,v5);
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
          if (!(*(unsigned short *)((unsigned long)v9[-1] * 2 + *(long *)__ctype_b_loc()) & 0x2000)) goto label_7cf7;
        }
      }
      else {
label_7cf7:
        *v9 = 0;
        (*a0)(a1,v7,a3,a5);
      }
      v7 = &v6[1];
    }
    v6 = &v6[1];
  } while( true );
}

// Function: sub_7d6b @ 0x7d6b
void sub_7d6b(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long *a3)
{
  (*(void *)*a3)(a0,a1,a2);
}

// Function: sub_7dac @ 0x7dac
FILE * sub_7dac(unsigned long a0,struct_12 *a1,char *a2,unsigned int a3,char a4) // early-return x2
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
    v1 = sub_7b1c(sub_7d6b,a1,v2,a3,(int)a4,&v3);
    v4 = v1;
    v2 = (int *)__errno_location();
    v5 = *(int *)v2;
    v1 = sub_7ea0(v6);
    if (v1) {
      v2 = (FILE *)0xffffffff;
      return v2;
    }
    v2 = (int *)__errno_location();
    *(int *)v2 = v5;
    v2 = (unsigned long)(unsigned long)v4;
  }
  else {
    v2 = (unsigned long)sub_7b1c(sub_7d6b,a1,stdin,a3,(int)a4,&v3);
  }
  return v2;
}

// Function: sub_7ea0 @ 0x7ea0
unsigned long sub_7ea0(FILE *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_7f2e;
  }
  if (sub_7f91(a0))
    v2 = *__errno_location();
label_7f2e:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_7f59 @ 0x7f59
void sub_7f59(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_ad44(a0,0,1);
}

// Function: sub_7f91 @ 0x7f91
void sub_7f91(FILE *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_7f59(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_7fe0 @ 0x7fe0
void sub_7fe0(FILE *a0,unsigned char *a1,long a2)
{
  long v1;
  unsigned char v2;
  long v3; // stack - 0x20
  unsigned char *v4; // stack - 0x18
  
  v3 = a2;
  v4 = a1;
  while (v1 = v3 + -1, v3) {
    v2 = *v4;
    fputc(tolower((unsigned int)v2),a0);
    v4 = &v4[1];
    v3 = v1;
  }
}

// Function: sub_8035 @ 0x8035
void sub_8035(FILE *a0,unsigned char *a1,long a2)
{
  long v1;
  unsigned char v2;
  long v3; // stack - 0x20
  unsigned char *v4; // stack - 0x18
  
  v3 = a2;
  v4 = a1;
  while (v1 = v3 + -1, v3) {
    v2 = *v4;
    fputc(toupper((unsigned int)v2),a0);
    v4 = &v4[1];
    v3 = v1;
  }
}

// Function: sub_808a @ 0x808a
int sub_808a(int a0,int a1)
{
  return (a0 - ((a0 - a1) + 0x17e) % 7) + 3;
}

// Function: sub_80da @ 0x80da
void sub_80da(FILE *a0,char *a1,tm *a2,unsigned long a3,unsigned int a4)
{
  char v1; // stack - 0x11
  
  v1 = 0;
  sub_815c(a0,a1,a2,0,0,0xffffffff,&v1,a3,a4);
}

// Function: sub_815c @ 0x815c
long sub_815c(FILE *a0,char *a1,tm *a2,char a3,int a4,int a5,unsigned long a6,unsigned long a7,int a8) // early-return x13, ternary x6, warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_815c
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
  int v23; // stack - 0x7bc
  bool v24; // stack - 0x794
  bool v25; // stack - 0x791
  unsigned int v26; // stack - 0x78c
  int v27; // stack - 0x788
  int v28; // stack - 0x780
  unsigned int v29; // stack - 0x77c
  bool v3;
  unsigned int v30; // stack - 0x778
  unsigned int v31; // stack - 0x774
  char v32; // stack - 0x770
  int v33; // stack - 0x76c
  int v34; // stack - 0x768
  int v35; // stack - 0x764
  int v36; // stack - 0x760
  int v37; // stack - 0x75c
  int v38; // stack - 0x758
  unsigned int v39; // stack - 0x754
  unsigned long v4;
  unsigned char *v40; // stack - 0x718
  long v41; // stack - 0x710
  char *v42; // stack - 0x708
  char *v43; // stack - 0x700
  char *v44; // stack - 0x6f8
  unsigned long v45; // stack - 0x6f0
  unsigned long v46; // stack - 0x6e0
  unsigned long v47; // stack - 0x6d8
  unsigned long v48; // stack - 0x6d0
  unsigned long v49; // stack - 0x6c8
  unsigned long v5;
  unsigned long v50; // stack - 0x6c0
  char *v51; // stack - 0x6b8
  unsigned long v52; // stack - 0x6b0
  unsigned long v53; // stack - 0x6a8
  unsigned long v54; // stack - 0x6a0
  unsigned long v55; // stack - 0x688
  unsigned long v56; // stack - 0x680
  unsigned long v57; // stack - 0x668
  unsigned long v58; // stack - 0x660
  unsigned long v59; // stack - 0x658
  char *v6;
  unsigned long v60; // stack - 0x650
  long v61; // stack - 0x648
  unsigned long v62; // stack - 0x640
  unsigned long v63; // stack - 0x638
  unsigned long v64; // stack - 0x630
  unsigned long v65; // stack - 0x628
  unsigned long v66; // stack - 0x620
  unsigned long v67; // stack - 0x618
  unsigned long v68; // stack - 0x460
  unsigned long v69; // stack - 0x458
  bool v7;
  long v70; // stack - 0x450
  unsigned long v71; // stack - 0x448
  unsigned long v72; // stack - 0x440
  unsigned long v73; // stack - 0x438
  char v8;
  int v9;
  
  v5 = a7;
  v4 = a6;
  v1 = *__errno_location();
  v26 = *(unsigned int *)&a2->field_0x8;
  v41 = 0;
  v40 = *(unsigned char **)&a2->field_0x30;
  if (!v40)
    v40 = (unsigned char *)0x1a7e4;
  if (0xd <= (int)v26) // branch-flip
    v26 -= 0xc;
  else if (!v26)
    v26 = 0xc;
  v23 = a5;
  v42 = a1;
  do {
    v6 = v42;
    if (!*v42) {
      *__errno_location() = v1;
      return v41;
    }
    v27 = 0;
    v3 = 0;
    v7 = 0;
    v25 = a3;
    if (*v42 != '%') {
      v12 = (0 <= v23) ? (unsigned long)v23 : 0; // branch-flip
      v13 = v12;
      if (v12 <= 1)
        v13 = 1;
      if ((unsigned long)(-v41 - 1U) <= v13) {
        *__errno_location() = 0x22;
        return 0;
      }
      if (a0) {
        if (1 < v12) {
          for (v46 = 0; v46 < v12 - 1; v46 = v46 + 1) {
            fputc(0x20,a0);
          }
        }
        fputc((int)*v42,a0);
      }
      v41 += v13;
      goto label_ab3c;
    }
    while( true ) {
      v42 = &v42[1];
      if (0x3c < (unsigned int)((int)*v42 - 0x23U)) break;
      v12 = 1L << ((unsigned char)((int)*v42 - 0x23U) & 0x3f);
      if (v12 & 0x1000000000002500) // branch-flip
        v27 = (int)*v42;
      else if (v12 & 1) // branch-flip
        v7 = 1;
      else {
        if (!(v12 & 0x800000000000000)) break;
        v25 = '\x01';
      }
    }
    if ((unsigned int)((int)*v42 - 0x30U) <= 9) {
      v23 = 0;
      do {
        v9 = (int)((long)v23 * 10);
        if (((long)v9 != (long)v23 * 10) || (v23 = *v42 + -0x30 + v9, SCARRY4(*v42 + -0x30,v9)))
          v23 = 0x7fffffff;
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
    v32 = *v42;
    switch(v32) {
      case 0:
label_a919:
        v42 = &v42[-1];
        break;
      case 0x25:
        if (v6 != &v42[-1]) goto label_a919;
        if ((v27 != 0x2d) && (0 <= v23))
          v12 = (unsigned long)v23;
        else {
          v12 = 0;
        }
        v13 = v12;
        if (v12 <= 1)
          v13 = 1;
        if ((unsigned long)(-v41 - 1U) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (1 < v12) {
            if ((v27 != 0x30) && (v27 != 0x2b)) {
              for (v48 = 0; v48 < v12 - 1; v48 = v48 + 1) {
                fputc(0x20,a0);
              }
            }
            else {
              for (v47 = 0; v47 < v12 - 1; v47 = v47 + 1) {
                fputc(0x30,a0);
              }
            }
          }
          fputc((int)*v42,a0);
        }
        v41 += v13;
        goto label_ab3c;
      case 0x3a:
        for (v45 = 1; v42[v45] == ':'; v45 = v45 + 1) {
        }
        if (v42[v45] == 'z') {
          v42 = &v42[v45];
          goto label_a6ea;
        }
        break;
      case 0x41:
        if (!v8) {
          if (v7)
            v25 = 1;
          v3 = 0;
          goto label_8a4c;
        }
        break;
      case 0x42:
        if (v8 != 'E') {
          if (v7)
            v25 = 1;
          v3 = 0;
          goto label_8a4c;
        }
        break;
      case 0x43:
        if (v8 == 'E') goto label_8a4c;
        v24 = *(int *)&a2->field_0x14 < -0x76c;
        v30 = (int)((unsigned int)((*(unsigned int *)&a2->field_0x14 >> 0x1f & (unsigned int)!(bool)v24) != 0) * -99 + *(int *)&a2->field_0x14) / 100 + 0x13;
        v28 = 2;
label_8e1f:
        if (!v27)
          v27 = a4;
        if (v27 != 0x2b) { // branch-flip
label_8e6f:
          v7 = 0;
        }
        else {
          v10 = (v28 != 2) ? 9999 : 99; // branch-flip
          if ((v30 <= v10) && (v23 <= v28)) goto label_8e6f;
          v7 = 1;
        }
        goto label_8ebb;
      case 0x44:
        if (!v8) {
          v43 = "%m/%d/%y";
          goto label_8830;
        }
        break;
      case 0x46:
        if (v8) break;
        if ((v27) || (0 <= v23)) {
          v33 = v23 + -6;
          if (v33 <= -1)
            v33 = 0;
        }
        else {
          v27 = 0x2b;
          v33 = 4;
        }
        v43 = "%Y-%m-%d";
        goto label_883a;
      case 0x47:
      case 0x56:
      case 0x67:
        if (v8 != 'E') {
          v9 = (0 <= *(int *)&a2->field_0x14) ? -100 : 300; // branch-flip
          v10 = v9 + *(int *)&a2->field_0x14;
          v37 = 0;
          v38 = sub_808a(*(unsigned int *)&a2->field_0x1c,*(unsigned int *)&a2->field_0x18);
          if (0 <= v38) { // branch-flip
            v9 = !((v10 & 3) || ((!((int)v10 % 100) && ((int)v10 % 400))));
            v9 = sub_808a(*(int *)&a2->field_0x1c - (v9 + 0x16d),*(unsigned int *)&a2->field_0x18);
            if (0 <= v9) {
              v37 = 1;
              v38 = v9;
            }
          }
          else {
            v37 = -1;
            v2 = *(unsigned int *)&a2->field_0x18;
            v9 = *(int *)&a2->field_0x1c;
            v11 = !((v10 - 1 & 3) || ((!((int)(v10 - 1) % 100) && ((int)(v10 - 1) % 400))));
            v38 = sub_808a(v11 + 0x16d + v9,v2);
          }
          if (*v42 != 'G') { // branch-flip
            if (*v42 != 'g') {
              v28 = 2;
              v29 = v38 / 7 + 1;
              goto label_8e99;
            }
            v30 = (v37 + *(int *)&a2->field_0x14 % 100) % 100;
            v28 = 2;
            v24 = 0;
            if ((int)v30 <= -1) {
              v30 = (0xfffff894U - v37 <= *(int *)&a2->field_0x14) ? v30 + 100 : -v30; // branch-flip
            }
          }
          else {
            v28 = 4;
            v24 = *(int *)&a2->field_0x14 < 0xfffff894U - v37;
            v30 = v37 + *(int *)&a2->field_0x14 + 0x76c;
          }
          goto label_8e1f;
        }
        break;
      case 0x48:
        if (v8 != 'E') {
          v28 = 2;
          v29 = *(unsigned int *)&a2->field_0x8;
          goto label_8e99;
        }
        break;
      case 0x49:
        if (v8 != 'E') {
          v28 = 2;
          v29 = v26;
          goto label_8e99;
        }
        break;
      case 0x4d:
        if (v8 != 'E') {
          v28 = 2;
          v29 = *(unsigned int *)&a2->field_0x4;
          goto label_8e99;
        }
        break;
      case 0x4e:
        if (v8 == 'E') break;
        v34 = a8;
        if (v23 <= 0)
          v23 = 9;
        v35 = 9;
        for (; (v23 < v35 || ((2 <= v35 && (!(v34 % 10))))); v34 = v34 / 10) {
          v35 -= 1;
        }
        for (v36 = v35; 0 < v36; v36 = v36 + -1) {
          v20[v36 + -1] = (char)v34 + (char)(v34 / 10) * '\xf6' + '0';
          v34 /= 10;
        }
        if (!v27)
          v27 = 0x30;
        v12 = (unsigned long)v35;
        if ((unsigned long)(-v41 - 1U) <= v12) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (v25) // branch-flip
            sub_8035(a0,v20,v12);
          else {
            fwrite(v20,v12,1,a0);
          }
        }
        if ((v27 != 0x2d) && (0 <= v23 - v35))
          v13 = (unsigned long)(v23 - v35);
        else {
          v13 = 0;
        }
        v14 = v13;
        if (!v13)
          v14 = 0;
        if (-(v41 + v12) - 1 <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if ((a0) && (v13)) {
          if ((v27 != 0x30) && (v27 != 0x2b)) {
            for (v58 = 0; v58 < v13; v58 = v58 + 1) {
              fputc(0x20,a0);
            }
          }
          else {
            for (v57 = 0; v57 < v13; v57 = v57 + 1) {
              fputc(0x30,a0);
            }
          }
        }
        v41 = v41 + v12 + v14;
        goto label_ab3c;
      case 0x50:
        v3 = 1;
        v32 = 'p';
        goto label_9b74;
      case 0x52:
        v43 = "%H:%M";
        goto label_8830;
      case 0x53:
        if (v8 != 'E') {
          v28 = 2;
          v29 = *(unsigned int *)a2;
          goto label_8e99;
        }
        break;
      case 0x54:
        v43 = "%H:%M:%S";
label_8830:
        v33 = -1;
label_883a:
        v12 = sub_815c(0,v43,a2,(unsigned char)v25,v27,v33,v4,v5,a8);
        if ((v27 != 0x2d) && (0 <= v23))
          v13 = (unsigned long)v23;
        else {
          v13 = 0;
        }
        v14 = v13;
        if (v13 <= v12)
          v14 = v12;
        if ((unsigned long)(-v41 - 1U) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (v12 < v13) {
            if ((v27 != 0x30) && (v27 != 0x2b)) {
              for (v50 = 0; v50 < v13 - v12; v50 = v50 + 1) {
                fputc(0x20,a0);
              }
            }
            else {
              for (v49 = 0; v49 < v13 - v12; v49 = v49 + 1) {
                fputc(0x30,a0);
              }
            }
          }
          sub_815c(a0,v43,a2,(unsigned char)v25,v27,v33,v4,v5,a8);
        }
        v41 += v14;
        goto label_ab3c;
      case 0x55:
        if (v8 != 'E') {
          v28 = 2;
          v29 = ((*(int *)&a2->field_0x1c - *(int *)&a2->field_0x18) + 7) / 7;
          goto label_8e99;
        }
        break;
      case 0x57:
        if (v8 != 'E') {
          v28 = 2;
          v29 = ((*(int *)&a2->field_0x1c - (*(int *)&a2->field_0x18 + 6) % 7) + 7) / 7;
          goto label_8e99;
        }
        break;
      case 0x58:
        goto label_8d93;
      case 0x59:
        if (v8 == 'E') goto label_8a4c;
        if (v8 != 'O') {
          v28 = 4;
          v24 = *(int *)&a2->field_0x14 < -0x76c;
          v30 = *(int *)&a2->field_0x14 + 0x76c;
          goto label_8e1f;
        }
        break;
      case 0x5a:
        if (v7)
          v25 = 0;
        v12 = strlen((char *)v40);
        if ((v27 != 0x2d) && (0 <= v23))
          v13 = (unsigned long)v23;
        else {
          v13 = 0;
        }
        v14 = v13;
        if (v13 <= v12)
          v14 = v12;
        if ((unsigned long)(-v41 - 1U) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (v12 < v13) {
            if ((v27 != 0x30) && (v27 != 0x2b)) {
              for (v65 = 0; v65 < v13 - v12; v65 = v65 + 1) {
                fputc(0x20,a0);
              }
            }
            else {
              for (v64 = 0; v64 < v13 - v12; v64 = v64 + 1) {
                fputc(0x30,a0);
              }
            }
          }
          if (v7)
            sub_7fe0(a0,v40,v12);
          else if (v25) // branch-flip
            sub_8035(a0,v40,v12);
          else {
            fwrite(v40,v12,1,a0);
          }
        }
        v41 += v14;
        goto label_ab3c;
      case 0x61:
        if (!v8) {
          if (v7)
            v25 = 1;
          v3 = 0;
          goto label_8a4c;
        }
        break;
      case 0x62:
      case 0x68:
        if (v7)
          v25 = 1;
        v3 = 0;
        if (v8 != 'E') goto label_8a4c;
        break;
      case 99:
        goto label_8d93;
      case 100:
        if (v8 != 'E') {
          v28 = 2;
          v29 = *(unsigned int *)&a2->field_0xc;
          goto label_8e99;
        }
        break;
      case 0x65:
        if (v8 != 'E') {
          v29 = *(unsigned int *)&a2->field_0xc;
label_8e83:
          v28 = 2;
          if (!v27)
            v27 = 0x5f;
          goto label_8e99;
        }
        break;
      case 0x6a:
        if (v8 != 'E') {
          v28 = 3;
          v24 = *(int *)&a2->field_0x1c < -1;
          v30 = *(int *)&a2->field_0x1c + 1;
          goto label_8eb4;
        }
        break;
      case 0x6b:
        if (v8 != 'E') {
          v29 = *(unsigned int *)&a2->field_0x8;
          goto label_8e83;
        }
        break;
      case 0x6c:
        if (v8 != 'E') {
          v29 = v26;
          goto label_8e83;
        }
        break;
      case 0x6d:
        if (v8 != 'E') {
          v28 = 2;
          v24 = *(int *)&a2->field_0x10 < -1;
          v30 = *(int *)&a2->field_0x10 + 1;
          goto label_8eb4;
        }
        break;
      case 0x6e:
        if ((v27 != 0x2d) && (0 <= v23))
          v12 = (unsigned long)v23;
        else {
          v12 = 0;
        }
        v13 = v12;
        if (v12 <= 1)
          v13 = 1;
        if ((unsigned long)(-v41 - 1U) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (1 < v12) {
            if ((v27 != 0x30) && (v27 != 0x2b)) {
              for (v60 = 0; v60 < v12 - 1; v60 = v60 + 1) {
                fputc(0x20,a0);
              }
            }
            else {
              for (v59 = 0; v59 < v12 - 1; v59 = v59 + 1) {
                fputc(0x30,a0);
              }
            }
          }
          fputc(10,a0);
        }
        v41 += v13;
        goto label_ab3c;
      case 0x70:
label_9b74:
        if (v7) {
          v25 = 0;
          v3 = 1;
        }
        goto label_8a4c;
      case 0x71:
        v28 = 1;
        v24 = 0;
        v30 = (*(int *)&a2->field_0x10 * 0xb >> 5) + 1;
        goto label_8eb4;
      case 0x72:
        goto label_8a4c;
      case 0x73:
        v22 = *(unsigned long *)a2;
        v68 = *(unsigned long *)&a2->field_0x8;
        v69 = *(unsigned long *)&a2->field_0x10;
        v71 = *(unsigned long *)&a2->field_0x20;
        v72 = *(unsigned long *)&a2->field_0x28;
        v73 = *(unsigned long *)&a2->field_0x30;
        v70._0_4_ = (unsigned int)*(unsigned long *)&a2->field_0x18;
        v70 = CONCAT44(0xffffffff,(unsigned int)v70);
        v61 = sub_146d3(v5,&v22);
        if (v70 <= -1) {
          *__errno_location() = 0x4b;
          return 0;
        }
        v44 = &v21[1];
        v24 = -(char)(v61 >> 0x3f);
        do {
          v8 = (char)v61 + (char)(v61 / 10) * '\xf6';
          v61 /= 10;
          if (v24)
            v8 = -v8;
          v44 = &v44[-1];
          *v44 = v8 + '0';
        } while (v61);
        v28 = 1;
        v7 = 0;
        goto label_8f9e;
      case 0x74:
        if ((v27 != 0x2d) && (0 <= v23))
          v12 = (unsigned long)v23;
        else {
          v12 = 0;
        }
        v13 = v12;
        if (v12 <= 1)
          v13 = 1;
        if ((unsigned long)(-v41 - 1U) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (1 < v12) {
            if ((v27 != 0x30) && (v27 != 0x2b)) {
              for (v63 = 0; v63 < v12 - 1; v63 = v63 + 1) {
                fputc(0x20,a0);
              }
            }
            else {
              for (v62 = 0; v62 < v12 - 1; v62 = v62 + 1) {
                fputc(0x30,a0);
              }
            }
          }
          fputc(9,a0);
        }
        v41 += v13;
        goto label_ab3c;
      case 0x75:
        v28 = 1;
        v29 = (*(int *)&a2->field_0x18 + 6) % 7 + 1;
label_8e99:
        v24 = (bool)(v29 >> 0x1f);
        v30 = v29;
label_8eb4:
        v7 = 0;
label_8ebb:
        v31 = 0;
label_8ec5:
        if ((v8 == 'O') && ((bool)v24 != 1)) goto label_8a4c;
        v44 = &v21[1];
        if ((bool)v24)
          v30 = -v30;
        do {
          if (v31 & 1) {
            v44 = &v44[-1];
            *v44 = ':';
          }
          v31 = (int)v31 >> 1;
          v44 = &v44[-1];
          *v44 = (char)v30 + (char)(v30 / 10) * '\xf6' + '0';
          v30 /= 10;
        } while ((v30) || (v31));
label_8f9e:
        if (!v27)
          v27 = 0x30;
        if (v23 <= -1)
          v23 = v28;
        if (v24) // branch-flip
          v8 = '-';
        else {
          v8 = (v7) ? '+' : '\0';
        }
        v9 = ((int)v20 + 0x17) - (int)v44;
        v11 = (v23 - (unsigned int)(v8 != '\0')) - v9;
        if ((v27 == 0x2d) || (v11 <= 0))
          v11 = 0;
        if (v8) {
          if (v27 == 0x5f) {
            if (a0) {
              for (v54 = 0; v54 < (unsigned long)(long)v11; v54 = v54 + 1) {
                fputc(0x20,a0);
              }
            }
            v41 += v11;
            v23 -= v11;
          }
          if ((unsigned long)(-v41 - 1U) <= 1) {
            *__errno_location() = 0x22;
            return 0;
          }
          if (a0)
            fputc((int)v8,a0);
          v41 += 1;
          v23 -= 1;
        }
        v12 = (unsigned long)v9;
        if ((v27 != 0x2d) && (0 <= v23))
          v13 = (unsigned long)v23;
        else {
          v13 = 0;
        }
        v14 = v13;
        if (v13 <= v12)
          v14 = v12;
        if ((unsigned long)(-v41 - 1U) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (a0) {
          if (v12 < v13) {
            if ((v27 != 0x30) && (v27 != 0x2b)) {
              for (v56 = 0; v56 < v13 - v12; v56 = v56 + 1) {
                fputc(0x20,a0);
              }
            }
            else {
              for (v55 = 0; v55 < v13 - v12; v55 = v55 + 1) {
                fputc(0x30,a0);
              }
            }
          }
          if (v25) // branch-flip
            sub_8035(a0,v44,v12);
          else {
            fwrite(v44,v12,1,a0);
          }
        }
        v41 += v14;
        goto label_ab3c;
      case 0x77:
        if (v8 != 'E') {
          v28 = 1;
          v29 = *(unsigned int *)&a2->field_0x18;
          goto label_8e99;
        }
        break;
      case 0x78:
label_8d93:
        if (v8 != 'O') goto label_8a4c;
        break;
      case 0x79:
        if (v8 != 'E') {
          v39 = *(int *)&a2->field_0x14 % 100;
          if ((int)v39 <= -1) {
            v39 = (-0x76c <= *(int *)&a2->field_0x14) ? v39 + 100 : -v39; // branch-flip
          }
          v28 = 2;
          v24 = 0;
          v30 = v39;
          goto label_8e1f;
        }
label_8a4c:
        v18[0] = ' ';
        v51 = &v18[2];
        v18[1] = 0x25;
        if (v8) {
          v51 = v19;
          v18[2] = v8;
        }
        *v51 = v32;
        v51[1] = '\0';
        v12 = strftime(v20,0x400,v18,a2);
        if (v12) {
          v12 -= 1;
          if ((v27 != 0x2d) && (0 <= v23))
            v13 = (unsigned long)v23;
          else {
            v13 = 0;
          }
          v14 = v13;
          if (v13 <= v12)
            v14 = v12;
          if ((unsigned long)(-v41 - 1U) <= v14) {
            *__errno_location() = 0x22;
            return 0;
          }
          if (a0) {
            if (v12 < v13) {
              if ((v27 != 0x30) && (v27 != 0x2b)) {
                for (v53 = 0; v53 < v13 - v12; v53 = v53 + 1) {
                  fputc(0x20,a0);
                }
              }
              else {
                for (v52 = 0; v52 < v13 - v12; v52 = v52 + 1) {
                  fputc(0x30,a0);
                }
              }
            }
            if (v3)
              sub_7fe0(a0,&v20[1],v12);
            else if (v25) // branch-flip
              sub_8035(a0,&v20[1],v12);
            else {
              fwrite(&v20[1],v12,1,a0);
            }
          }
          v41 += v14;
        }
        goto label_ab3c;
      case 0x7a:
        v45 = 0;
label_a6ea:
        if (*(int *)&a2->field_0x20 < 0) goto label_ab3c;
        v9 = (int)*(unsigned long *)&a2->field_0x28;
        v24 = !((0 <= v9) && ((v9 || (*v40 != 0x2d))));
        v30 = v9 / 0xe10;
        v11 = (v9 / 0x3c) % 0x3c;
        if (v45 != 3) {
          if (v45 <= 3) {
            if (v45 == 2) goto label_a89f;
            if (v45 <= 2) {
              if (!v45) {
                v28 = 5;
                v31 = 0;
                v30 = v11 + v30 * 100;
                goto label_8e13;
              }
              if (v45 == 1) goto label_a86e;
            }
          }
          break;
        }
        if (v9 % 0x3c) { // branch-flip
label_a89f:
          v28 = 9;
          v31 = 0x14;
          v30 = v9 % 0x3c + v30 * 10000 + v11 * 100;
        }
        else if (v11) { // branch-flip
label_a86e:
          v28 = 6;
          v31 = 4;
          v30 = v11 + v30 * 100;
        }
        else {
          v28 = 3;
          v31 = 0;
        }
label_8e13:
        v7 = 1;
        goto label_8ec5;
      
    }
    v15 = &v42[1 - (long)v6];
    if ((v27 != 0x2d) && (0 <= v23))
      v16 = (char *)(long)v23;
    else {
      v16 = NULL;
    }
    v17 = v16;
    if (v16 <= v15)
      v17 = v15;
    if ((char *)(0xffffffffffffffffU - v41) <= v17) {
      *__errno_location() = 0x22;
      return 0;
    }
    if (a0) {
      if (v15 < v16) {
        if ((v27 != 0x30) && (v27 != 0x2b)) {
          for (v67 = 0; v67 < (unsigned long)((long)v16 - (long)v15); v67 = v67 + 1) {
            fputc(0x20,a0);
          }
        }
        else {
          for (v66 = 0; v66 < (unsigned long)((long)v16 - (long)v15); v66 = v66 + 1) {
            fputc(0x30,a0);
          }
        }
      }
      if (v25) // branch-flip
        sub_8035(a0,v6,v15);
      else {
        fwrite(v6,(unsigned long)v15,1,a0);
      }
    }
    v41 += (long)v17;
label_ab3c:
    v23 = -1;
    v42 = &v42[1];
  } while( true );
}

// Function: sub_ab8e @ 0xab8e
unsigned long sub_ab8e(int a0) // return-dupe
{
  int v1; // eax
  
  v1 = open("/dev/null",0);
  if (v1 == a0)
    return 1;
  if (v1 <= -1)
    return 0;
  close(v1);
  *__errno_location() = 9;
  return 0;
}

// Function: sub_abea @ 0xabea
FILE * sub_abea(char *a0,char *a1,FILE *a2)
{
  bool v1;
  bool v2;
  bool v3;
  int v4;
  FILE *v5; // stack - 0x30
  
  v3 = 0;
  v2 = 0;
  v1 = 0;
  v4 = fileno(a2);
  if (v4 != 2) { // branch-flip
    if (2 < v4) {
label_ac2d:
      if (dup2(2,2) != 2)
        v1 = 1;
      goto label_ac45;
    }
    if (v4) {
      if (v4 != 1) goto label_ac2d;
      goto label_ac5d;
    }
  }
  else {
label_ac45:
    if (dup2(1,1) != 1)
      v2 = 1;
label_ac5d:
    if (dup2(0,0))
      v3 = 1;
  }
  if (v3) {
    if (sub_ab8e(0) != '\x01') {
      v5 = NULL;
      goto label_acf3;
    }
  }
  if (v2) {
    if (sub_ab8e(1) != '\x01') {
      v5 = NULL;
      goto label_acf3;
    }
  }
  if (v1) {
    if (sub_ab8e(2) != '\x01') {
      v5 = NULL;
      goto label_acf3;
    }
  }
  v5 = freopen(a0,a1,a2);
label_acf3:
  v4 = *__errno_location();
  if (v1)
    close(2);
  if (v2)
    close(1);
  if (v3)
    close(0);
  if (!v5)
    *__errno_location() = v4;
  return v5;
}

// Function: sub_ad44 @ 0xad44
unsigned long sub_ad44(FILE *a0,long a1,int a2) // early-return x2
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

// Function: sub_ae02 @ 0xae02
unsigned int sub_ae02(struct_8 *a0,struct_8 *a1) // return-dupe
{
  if ((a0->field_0x8 == a1->field_0x8) && (a0->field_0x0 == a1->field_0x0))
    return 1;
  return 0;
}

// Function: sub_ae5b @ 0xae5b
unsigned long sub_ae5b(char *a0,unsigned long a1)
{
  return *(unsigned long *)&a0[8] % a1;
}

// Function: sub_ae89 @ 0xae89
unsigned long sub_ae89(char *a0) // return-dupe x2, return-dupe
{
  if (!(*(unsigned int *)&a0[0x48] & 0x102)) {
    *(void **)&a0[0x58] = malloc(0x20);
    if (*(long *)&a0[0x58]) {
      sub_1782e(*(char **)&a0[0x58]);
      return 1;
    }
    return 0;
  }
  *(unsigned long *)&a0[0x58] = sub_e361(0x1f,NULL,sub_ae5b,sub_ae02,dat_20fa8);
  if (*(long *)&a0[0x58])
    return 1;
  return 0;
}

// Function: sub_af35 @ 0xaf35
unsigned long sub_af35(char *a0,struct_18 *a1) // return-dupe x3
{
  struct_17 *v1;
  struct_19 *v2; // rax
  struct_19 *v3; // rax
  
  if (!(*(unsigned int *)&a0[0x48] & 0x102)) {
    v1 = *(struct_17 **)&a0[0x58];
    if (!sub_17854(v1,&a1->field_0x70))
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
  v3 = (struct_19 *)sub_f0a2(*(char **)&a0[0x58],v2);
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

// Function: sub_b047 @ 0xb047
void sub_b047(char *a0,char *a1)
{
  long v1;
  void *v2; // rax
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  
  if (*(unsigned int *)&a0[0x48] & 0x102) { // branch-flip
    v3 = *(long *)&a1[0x70];
    v4 = *(unsigned long *)&a1[0x78];
    v2 = (void *)sub_f112(*(char **)&a0[0x58],&v3);
    if (!v2)
      abort(); // no-return
    free(v2);
  }
  else {
    v1 = *(long *)&a1[8];
    if ((v1) && (0 <= *(long *)(v1 + 0x58))) {
      if (!*(long *)(*(long *)&a0[0x58] + 0x10))
        abort(); // no-return
      if ((**(long **)&a0[0x58] == *(long *)&a1[0x78]) && (*(long *)(*(long *)&a0[0x58] + 8) == *(long *)&a1[0x70])) {
        *(unsigned long *)(*(long *)&a0[0x58] + 8) = *(unsigned long *)(v1 + 0x70);
        **(unsigned long **)&a0[0x58] = *(unsigned long *)(v1 + 0x78);
      }
    }
  }
}

// Function: sub_b179 @ 0xb179
void sub_b179(char *a0) // return-dupe
{
  if (!(*(unsigned int *)&a0[0x48] & 0x102)) {
    free(*(void **)&a0[0x58]);
    return;
  }
  if (*(long *)&a0[0x58]) {
    sub_e5e1(*(struct_14 **)&a0[0x58]);
    return;
  }
}

// Function: sub_b1cb @ 0xb1cb
void sub_b1cb(char *a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_10380(a0) == '\x01') break;
    v1 = sub_10422(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_b213 @ 0xb213
void sub_b213(char *a0,bool a1) // ternary
{
  unsigned long v1; // rdx
  
  if (*(short *)&a0[0x68] != 0xb)
    abort(); // no-return
  v1 = (a1) ? 2 : 1; // branch-flip
  *(unsigned long *)&a0[0xa0] = v1;
}

// Function: sub_b25b @ 0xb25b
void sub_b25b(char *a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)&a0[0x2c];
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_10396(&a0[0x60],v1);
    if (0 <= v1)
      close(v1);
  }
  else if ((!(*(unsigned int *)&a0[0x48] & 4)) && (0 <= v1))
    close(v1);
  *(int *)&a0[0x2c] = a1;
}

// Function: sub_b2eb @ 0xb2eb
unsigned int sub_b2eb(char *a0)
{
  unsigned int v1;
  int v2;
  
  if (!(*(unsigned int *)&a0[0x48] & 4)) {
    if (*(unsigned int *)&a0[0x48] & 0x200) { // branch-flip
      if (*(unsigned int *)&a0[0x48] & 0x200) // branch-flip
        v1 = 0xffffff9c;
      else {
        v1 = *(unsigned int *)&a0[0x28];
      }
      sub_b25b(a0,v1,1);
    }
    else {
      if (*(unsigned int *)&a0[0x48] & 0x200) // branch-flip
        v2 = -100;
      else {
        v2 = *(int *)&a0[0x28];
      }
      if (fchdir(v2)) {
        v1 = 1;
        sub_b1cb(&a0[0x60]); // return-dupe
        return v1;
      }
    }
  }
  v1 = 0;
  sub_b1cb(&a0[0x60]);
  return v1;
}

// Function: sub_b399 @ 0xb399
unsigned int sub_b399(char *a0,char *a1)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  
  v1 = (*(unsigned int *)&a0[0x48] & 0x10) << 0xd | 0x90900;
  if (*(unsigned int *)&a0[0x48] & 0x200) // branch-flip
    v2 = sub_11e0a(*(unsigned int *)&a0[0x2c],a1,v1);
  else {
    v2 = sub_17de1(a1,v1);
  }
  return v2;
}

// Function: sub_b411 @ 0xb411
long * sub_b411(long *a0,unsigned int a1,long a2) // early-return x5
{
  bool v1; // al
  long *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  char *v5;
  long *v6; // stack - 0x70
  void *v7; // stack - 0x60
  char *v8; // stack - 0x58
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
  v3 = sub_d67f(a0);
  if (v3 <= 0xfff)
    v3 = 0x1000;
  if (sub_d49b(v2,v3) != '\x01') {
    free(v2); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v7 = (void *)sub_d3a4(v2,(void *)0x1aa00,0);
    if (v7) {
      *(unsigned long *)((long)v7 + 0x58) = 0xffffffffffffffff;
      goto label_b570;
    }
  }
  else {
label_b570:
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
      v4 = (char *)sub_d3a4(v2,(void *)*v6,v9);
      if (!v4) goto label_b79a;
      *(unsigned long *)&v4[0x58] = 0;
      *(void **)&v4[8] = v7;
      *(char **)&v4[0x30] = &v4[0x100];
      if ((v1) && (v5)) {
        *(unsigned short *)&v4[0x68] = 0xb;
        sub_b213(v4,1);
      }
      else {
        *(unsigned short *)&v4[0x68] = sub_d069(v2,v4,0);
      }
      if (a2) { // branch-flip
        *(char **)&v4[0x10] = v5;
        v5 = v4;
      }
      else {
        *(unsigned long *)&v4[0x10] = 0;
        if (v5) { // branch-flip
          *(char **)&v8[0x10] = v4;
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
      v5 = (char *)sub_d236(v2,v5,v3);
    *v2 = sub_d3a4(v2,(void *)0x1aa00,0);
    if (*v2) {
      *(char **)(*v2 + 0x10) = v5;
      *(unsigned short *)(*v2 + 0x68) = 9;
      *(unsigned long *)(*v2 + 0x58) = 1;
      if (sub_ae89(v2) == '\x01') {
        if ((!(*(unsigned int *)&v2[9] & 4)) && (!(*(unsigned int *)&v2[9] & 0x200))) {
          *(unsigned int *)&v2[5] = sub_b399(v2,".");
          if ((int)v2[5] <= -1)
            *(unsigned int *)&v2[9] = *(unsigned int *)&v2[9] | 4;
        }
        sub_10323(&v2[0xc],0xffffffff);
        return v2;
      }
    }
label_b79a:
    sub_d45a(v5);
    free(v7);
  }
  free((void *)v2[4]);
  free(v2);
  return NULL;
}

// Function: sub_b7dc @ 0xb7dc
void sub_b7dc(struct_25 *a0,char *a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  *(unsigned long *)&a1[0x48] = *(unsigned long *)&a1[0x60];
  memmove((void *)a0->field_0x20,&a1[0x100],*(long *)&a1[0x48] + 1);
  v1 = strrchr(&a1[0x100],0x2f);
  if ((v1) && ((v1 != &a1[0x100] || (v1[1])))) {
    v2 = strlen(&v1[1]);
    memmove(&a1[0x100],&v1[1],v2 + 1);
    *(unsigned long *)&a1[0x60] = v2;
  }
  *(long *)&a1[0x38] = a0->field_0x20;
  *(unsigned long *)&a1[0x30] = *(unsigned long *)&a1[0x38];
}

// Function: sub_b89b @ 0xb89b
unsigned long sub_b89b(struct_27 *a0) // return-dupe, ternary
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
    sub_d45a((char *)a0->field_0x8);
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
  sub_b1cb(&a0[1].field_0x8);
  if (a0->field_0x50)
    sub_e5e1((struct_14 *)a0->field_0x50);
  sub_b179(a0);
  free(a0);
  if (v4) {
    *__errno_location() = v4;
    return 0xffffffff;
  }
  return 0;
}

// Function: sub_ba28 @ 0xba28
unsigned long sub_ba28(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_ba5d @ 0xba5d
unsigned long sub_ba5d(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_ba93 @ 0xba93
unsigned long sub_ba93(char *a0,int a1) // early-return x2
{
  long v1;
  struct_5 *v2; // rax
  struct_5 *v3; // rax
  unsigned long v4 [2]; // stack - 0x98
  statfs v5; // stack - 0x88
  struct_6 *v6; // stack - 0xb8
  
  v1 = *(long *)&a0[0x50];
  v6 = *(struct_6 **)(v1 + 0x50);
  if (!(*(unsigned int *)(v1 + 0x48) & 0x200))
    return 0;
  if (!v6) {
    *(unsigned long *)(v1 + 0x50) = sub_e361(0xd,NULL,sub_ba28,sub_ba5d,dat_20fa8);
    v6 = *(struct_6 **)(v1 + 0x50);
  }
  if (v6) {
    v4[0] = *(unsigned long *)&a0[0x70];
    v1 = sub_dcac(v6,v4);
    if (v1)
      return *(unsigned long *)(v1 + 8);
  }
  if ((0 <= a1) && (!fstatfs(a1,&v5))) {
    if ((v6) && (v2 = malloc(0x10), v2)) {
      v2->field_0x0 = *(unsigned long *)&a0[0x70];
      v2->field_0x8 = v5._0_8_;
      v3 = (struct_5 *)sub_f0a2(v6,v2);
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

// Function: sub_bc8c @ 0xbc8c
unsigned long sub_bc8c(char *a0,unsigned int a1) // early-return
{
  long v1; // rax
  
  v1 = sub_ba93(a0,a1);
  if ((v1 != 0xff534d42) && ((0xff534d43 <= v1 || ((v1 != 0x6969 && (v1 != 0x1021994))))))
    return 1;
  return 0;
}

// Function: sub_bce2 @ 0xbce2
unsigned long sub_bce2(char *a0,unsigned int a1) // early-return
{
  long v1; // rax
  
  v1 = sub_ba93(a0,a1);
  if ((v1 != 0xff534d42) && ((0xff534d43 <= v1 || ((v1 != 0x5346414f && ((0x53464150 <= v1 || ((v1 != 0x9fa0 && ((0x9fa1 <= v1 || ((v1 && (v1 != 0x6969))))))))))))))
    return 1;
  return 0;
}

// Function: sub_bd55 @ 0xbd55
char * sub_bd55(struct_25 *a0) // early-return x7, ternary
{
  short v1;
  int v2;
  char *v3;
  unsigned short v4; // ax
  char *v5;
  long v6;
  char *v7;
  bool v8;
  
  if ((!a0->field_0x0) || (a0->field_0x48 & 0x2000))
    return NULL;
  v5 = (char *)a0->field_0x0;
  v1 = *(short *)&v5[0x6c];
  *(unsigned short *)&v5[0x6c] = 3;
  if (v1 == 1) {
    *(unsigned short *)&v5[0x68] = sub_d069(a0,v5,0);
    return v5;
  }
  if ((v1 != 2) || ((*(short *)&v5[0x68] != 0xc && (*(short *)&v5[0x68] != 0xd)))) {
    v7 = v5;
    if (*(short *)&v5[0x68] != 1) { // branch-flip
      do {
        if ((!*(long *)&v7[0x10]) && (*(long *)(*(long *)&v7[8] + 0x18))) {
          v6 = *(long *)&v7[8];
          a0->field_0x0 = v6;
          *(char *)(*(long *)(v6 + 0x48) + a0->field_0x20) = 0;
          v5 = (char *)sub_c6a5(a0,3);
          if (!v5) {
            if (a0->field_0x48 & 0x2000)
              return NULL;
label_c225:
            v5 = *(char **)&v7[8];
            a0->field_0x0 = (long)v5;
            free(v7);
            if (*(long *)&v5[0x58] == -1) {
              free(v5);
              *__errno_location() = 0;
              a0->field_0x0 = 0;
              return NULL;
            }
            if (*(short *)&v5[0x68] != 0xb) {
              *(char *)(*(long *)&v5[0x48] + a0->field_0x20) = 0;
              if (*(long *)&v5[0x58]) { // branch-flip
                if (*(unsigned short *)&v5[0x6a] & 2) { // branch-flip
                  if (!(a0->field_0x48 & 4)) {
                    if (a0->field_0x48 & 0x200) { // branch-flip
                      sub_b25b(a0,*(unsigned int *)&v5[0x44],1);
                      v8 = 0;
                    }
                    else {
                      v2 = *(int *)&v5[0x44];
                      v8 = fchdir(v2) != 0;
                    }
                    if (v8) {
                      *(int *)&v5[0x40] = *__errno_location();
                      a0->field_0x48 = a0->field_0x48 | 0x2000;
                    }
                  }
                  close(*(int *)&v5[0x44]);
                }
                else if ((!(*(unsigned short *)&v5[0x6a] & 1)) && (v7 = *(char **)&v5[8], sub_d6d9(a0,v7,0xffffffff,".."))) {
                  *(int *)&v5[0x40] = *__errno_location();
                  a0->field_0x48 = a0->field_0x48 | 0x2000;
                }
              }
              else if (sub_b2eb(a0)) {
                *(int *)&v5[0x40] = *__errno_location();
                a0->field_0x48 = a0->field_0x48 | 0x2000;
              }
              if (*(short *)&v5[0x68] != 2) {
                v4 = (*(int *)&v5[0x40]) ? 7 : 6; // branch-flip
                *(unsigned short *)&v5[0x68] = v4;
                if (!*(int *)&v5[0x40])
                  sub_b047(a0,v5);
              }
              if (!(a0->field_0x48 & 0x2000))
                return v5;
              return NULL;
            }
            abort(); // no-return
          }
          free(v7);
          goto label_c12f;
        }
        v5 = *(char **)&v7[0x10];
        if (!v5) goto label_c225;
        a0->field_0x0 = (long)v5;
        free(v7);
        if (!*(long *)&v5[0x58]) {
          if (sub_b2eb(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          sub_b179(a0);
          sub_b7dc(a0,v5);
          sub_ae89(a0);
          goto label_c193;
        }
        v7 = v5;
      } while (*(short *)&v5[0x6c] == 4);
      if (*(short *)&v5[0x6c] == 2) {
        *(unsigned short *)&v5[0x68] = sub_d069(a0,v5,1);
        if ((*(short *)&v5[0x68] == 1) && (!(a0->field_0x48 & 4))) {
          *(unsigned int *)&v5[0x44] = sub_b399(a0,".");
          if (0 <= *(int *)&v5[0x44]) // branch-flip
            *(unsigned short *)&v5[0x6a] = *(unsigned short *)&v5[0x6a] | 2;
          else {
            *(int *)&v5[0x40] = *__errno_location();
            *(unsigned short *)&v5[0x68] = 7;
          }
        }
        *(unsigned short *)&v5[0x6c] = 3;
      }
    }
    else {
      if ((v1 == 4) || ((a0->field_0x48 & 0x40 && (*(long *)&v5[0x70] != a0->field_0x18)))) {
        if (*(unsigned short *)&v5[0x6a] & 2)
          close(*(int *)&v5[0x44]);
        if (a0->field_0x8) {
          sub_d45a((char *)a0->field_0x8);
          a0->field_0x8 = 0;
        }
        *(unsigned short *)&v5[0x68] = 6;
        sub_b047(a0,v5);
        return v5;
      }
      if ((a0->field_0x8) && (a0->field_0x48 & 0x1000)) {
        a0->field_0x48 = a0->field_0x48 & 0xffffefff;
        sub_d45a((char *)a0->field_0x8);
        a0->field_0x8 = 0;
      }
      if (a0->field_0x8) { // branch-flip
        v3 = *(char **)&v5[0x30];
        if (sub_d6d9(a0,v5,0xffffffff,v3)) {
          *(int *)&v5[0x40] = *__errno_location();
          *(unsigned short *)&v5[0x6a] = *(unsigned short *)&v5[0x6a] | 1;
          for (v6 = a0->field_0x8; v6; v6 = *(long *)(v6 + 0x10)) {
            *(unsigned long *)(v6 + 0x30) = *(unsigned long *)(*(long *)(v6 + 8) + 0x30);
          }
        }
      }
      else {
        a0->field_0x8 = sub_c6a5(a0,3);
        if (!a0->field_0x8) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)&v5[0x40]) && (*(short *)&v5[0x68] != 4))
              *(unsigned short *)&v5[0x68] = 7;
            sub_b047(a0,v5);
            return v5;
          }
          return NULL;
        }
      }
      v5 = (char *)a0->field_0x8;
      a0->field_0x8 = 0;
    }
label_c12f:
    if (*(char *)(*(long *)(*(long *)&v5[8] + 0x48) + -1 + *(long *)(*(long *)&v5[8] + 0x38)) != '/') // branch-flip
      v6 = *(long *)(*(long *)&v5[8] + 0x48);
    else {
      v6 = *(long *)(*(long *)&v5[8] + 0x48) + -1;
    }
    v7 = (char *)(a0->field_0x20 + v6);
    *v7 = 0x2f;
    memmove(&v7[1],&v5[0x100],*(long *)&v5[0x60] + 1);
  }
  else {
    *(unsigned short *)&v5[0x68] = sub_d069(a0,v5,1);
    if ((*(short *)&v5[0x68] == 1) && (!(a0->field_0x48 & 4))) {
      *(unsigned int *)&v5[0x44] = sub_b399(a0,".");
      if (0 <= *(int *)&v5[0x44]) // branch-flip
        *(unsigned short *)&v5[0x6a] = *(unsigned short *)&v5[0x6a] | 2;
      else {
        *(int *)&v5[0x40] = *__errno_location();
        *(unsigned short *)&v5[0x68] = 7;
      }
    }
  }
label_c193:
  a0->field_0x0 = (long)v5;
  if (*(short *)&v5[0x68] == 0xb) {
    if (*(long *)&v5[0xa0] != 2) { // branch-flip
      if (*(long *)&v5[0xa0] != 1)
        abort(); // no-return
    }
    else {
      *(unsigned short *)&v5[0x68] = sub_d069(a0,v5,0);
    }
  }
  if (*(short *)&v5[0x68] == 1) {
    if (!*(long *)&v5[0x58])
      a0->field_0x18 = *(long *)&v5[0x70];
    if (sub_af35(a0,v5) != '\x01') {
      *__errno_location() = 0xc;
      return NULL;
    }
  }
  return v5;
}

// Function: sub_c3cb @ 0xc3cb
unsigned long sub_c3cb(unsigned long a0,char *a1,int a2) // return-dupe
{
  if ((((a2) && (a2 != 1)) && (a2 != 2)) && ((a2 != 3 && (a2 != 4)))) {
    *__errno_location() = 0x16;
    return 1;
  }
  *(short *)&a1[0x6c] = (short)a2;
  return 0;
}

// Function: sub_c426 @ 0xc426
int * sub_c426(struct_25 *a0,int a1) // early-return x4
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
      sub_d45a((char *)a0->field_0x8);
    if (a1 != 0x1000) // branch-flip
      v6 = 1;
    else {
      v4 = a0->field_0x48 | 0x1000;
      a0->field_0x48 = v4;
      v6 = 2;
    }
    if (((*(long *)(v2 + 0x58)) || (**(char **)(v2 + 0x30) == '/')) || (v4 = a0->field_0x48 & 4, v4)) {
      v5 = (long)sub_c6a5(a0,v6);
      a0->field_0x8 = (long)v5;
      v5 = (long)a0->field_0x8;
    }
    else {
      v4 = sub_b399(a0,".");
      v3 = v4;
      if (v4 <= -1) {
        a0->field_0x8 = 0;
        v5 = NULL;
        return v5;
      }
      v5 = (long)sub_c6a5(a0,v6);
      a0->field_0x8 = (long)v5;
      v4 = a0->field_0x48 & 0x200;
      if (v4) // branch-flip
        sub_b25b(a0,v3,1);
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

// Function: sub_c5c5 @ 0xc5c5
int sub_c5c5(long *a0,long *a1)
{
  return (unsigned int)(*(unsigned long *)(*a1 + 0x78) < *(unsigned long *)(*a0 + 0x78)) - (unsigned int)(*(unsigned long *)(*a0 + 0x78) < *(unsigned long *)(*a1 + 0x78));
}

// Function: sub_c619 @ 0xc619
void sub_c619(struct_21 *a0,unsigned int a1)
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

// Function: sub_c6a5 @ 0xc6a5
char * sub_c6a5(struct_24 *a0,int a1) // early-return, return-dupe x2, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_c6a5
{
  bool v1;
  char *v10; // rax
  int v11; // stack - 0x98
  unsigned long v12; // rax
  unsigned long v13; // r14
  bool v14; // zf
  bool v15; // stack - 0x9b
  int v16; // stack - 0x94
  char *v17; // stack - 0x90
  unsigned long v18; // stack - 0x88
  char *v19; // stack - 0x80
  char v2; // al
  char *v20; // stack - 0x78
  DIR *v21; // stack - 0x70
  unsigned long v22; // stack - 0x68
  unsigned long v23; // stack - 0x60
  long v24; // stack - 0x58
  dirent *v25; // stack - 0x50
  unsigned long v26; // stack - 0x48
  long v27; // stack - 0x40
  unsigned long v28; // stack - 0x38
  bool v3;
  bool v4; // al
  unsigned short v5; // ax
  unsigned int v6;
  int v7; // eax
  long v8;
  char *v9;
  
  v20 = a0->field_0x0;
  v14 = *(long *)&v20[0x18] != 0;
  v15 = 0;
  if (v14) {
    v21 = *(DIR **)&v20[0x18];
    v11 = dirfd(v21);
    if (v11 <= -1) {
      closedir(*(DIR **)&v20[0x18]);
      *(unsigned long *)&v20[0x18] = 0;
      if (a1 != 3)
        return NULL;
      *(unsigned short *)&v20[0x68] = 4;
      *(int *)&v20[0x40] = *__errno_location();
      return NULL;
    }
  }
  else {
    if ((a0->field_0x48 & 0x10) && ((!(a0->field_0x48 & 1) || (*(long *)&v20[0x58]))))
      v12 = 0x20000;
    else {
      v12 = 0;
    }
    if ((a0->field_0x48 & 4) || (!(a0->field_0x48 & 0x200)))
      v6 = 0xffffff9c;
    else {
      v6 = a0->field_0x2c;
    }
    v12 = sub_11f48(v6,*(char **)&v20[0x30],v12,&v11);
    *(unsigned long *)&v20[0x18] = v12;
    if (!*(long *)&v20[0x18]) {
      if (a1 != 3)
        return NULL;
      *(unsigned short *)&v20[0x68] = 4;
      *(int *)&v20[0x40] = *__errno_location();
      return NULL;
    }
    if (*(short *)&v20[0x68] != 0xb) { // branch-flip
      if (a0->field_0x48 & 0x100) {
        sub_b047(a0,v20);
        sub_d069(a0,v20,0);
        v2 = sub_af35(a0,v20);
        if (v2 != '\x01') {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      v5 = sub_d069(a0,v20,0);
      *(unsigned short *)&v20[0x68] = v5;
    }
  }
  v22 = (a0->field_0x40) ? 0xffffffffffffffff : 100000; // branch-flip
  if (v14)
    v1 = 1;
  else {
    v3 = ((a1 != 2) && ((((!(a0->field_0x48 & 8) || (!(a0->field_0x48 & 0x10))) || (a0->field_0x48 & 0x20)) || ((*(long *)&v20[0x80] != 2 || (v7 = sub_bce2(v20,v11), !v7))))));
    if ((v3) || (v1 = 0, a1 == 3)) {
      if (a0->field_0x48 & 0x200)
        v11 = sub_1793b(v11,0x406,3);
      if ((0 <= v11) && (v7 = sub_d6d9(a0,v20,v11,NULL), !v7))
        v1 = 1;
      else {
        if ((v3) && (a1 == 3))
          *(int *)&v20[0x40] = *__errno_location();
        *(unsigned short *)&v20[0x6a] = *(unsigned short *)&v20[0x6a] | 1;
        v1 = 0;
        closedir(*(DIR **)&v20[0x18]);
        *(unsigned long *)&v20[0x18] = 0;
        if ((a0->field_0x48 & 0x200) && (0 <= v11))
          close(v11);
        *(unsigned long *)&v20[0x18] = 0;
      }
    }
  }
  if (*(char *)(*(long *)&v20[0x48] + -1 + *(long *)&v20[0x38]) != '/') // branch-flip
    v8 = *(long *)&v20[0x48];
  else {
    v8 = *(long *)&v20[0x48] + -1;
  }
  if (a0->field_0x48 & 4) { // branch-flip
    v9 = (char *)(v8 + a0->field_0x20);
    v19 = &v9[1];
    *v9 = 0x2f;
  }
  else {
    v19 = NULL;
  }
  v23 = v8 + 1;
  v18 = a0->field_0x30 - v23;
  v24 = *(long *)&v20[0x58] + 1;
  v3 = 0;
  v9 = NULL;
  v17 = NULL;
  v13 = 0;
  do {
    do {
      if (!*(long *)&v20[0x18]) {
label_ceb5:
        if (*(long *)&v20[0x18]) {
          closedir(*(DIR **)&v20[0x18]);
          *(unsigned long *)&v20[0x18] = 0;
        }
        goto label_cee4;
      }
      *__errno_location() = 0;
      v25 = readdir(*(DIR **)&v20[0x18]);
      if (!v25) {
        if (*__errno_location()) {
          *(int *)&v20[0x40] = *__errno_location();
          if ((v14) || (v13))
            v5 = 7;
          else {
            v5 = 4;
          }
          *(unsigned short *)&v20[0x68] = v5;
        }
        goto label_ceb5;
      }
    } while (((!(a0->field_0x48 & 0x20)) && (v25->field_0x13 == '.')) && ((!v25->field_0x14 || ((v25->field_0x14 == '.' && (!v25->field_0x15))))));
    v26 = strlen(&v25->field_0x13);
    v10 = (char *)sub_d3a4(a0,&v25->field_0x13,v26);
    if (!v10) {
label_cc06:
      v16 = *__errno_location();
      free(v10);
      sub_d45a(v9);
      closedir(*(DIR **)&v20[0x18]);
      *(unsigned long *)&v20[0x18] = 0;
      *(unsigned short *)&v20[0x68] = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      *__errno_location() = v16;
      return NULL;
    }
    if (v18 <= v26) {
      v27 = a0->field_0x20;
      v8 = v23 + v26;
      if (sub_d49b(a0,v8 + 1) != '\x01') goto label_cc06;
      if ((v27 != a0->field_0x20) && (v3 = 1, a0->field_0x48 & 4))
        v19 = (char *)(v23 + a0->field_0x20);
      v18 = a0->field_0x30 - v23;
    }
    v28 = v26 + v23;
    if (v28 < v23) {
      free(v10);
      sub_d45a(v9);
      closedir(*(DIR **)&v20[0x18]);
      *(unsigned long *)&v20[0x18] = 0;
      *(unsigned short *)&v20[0x68] = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      *__errno_location() = 0x24;
      return NULL;
    }
    *(long *)&v10[0x58] = v24;
    *(char **)&v10[8] = a0->field_0x0;
    *(unsigned long *)&v10[0x48] = v28;
    *(unsigned long *)&v10[0x78] = *(unsigned long *)v25;
    if (a0->field_0x48 & 4) { // branch-flip
      *(unsigned long *)&v10[0x30] = *(unsigned long *)&v10[0x38];
      memmove(v19,&v10[0x100],*(long *)&v10[0x60] + 1);
    }
    else {
      *(char **)&v10[0x30] = &v10[0x100];
    }
    if ((a0->field_0x40) && (!(a0->field_0x48 & 0x400)))
      *(unsigned short *)&v10[0x68] = sub_d069(a0,v10,0);
    else {
      v4 = (((a0->field_0x48 & 8) && ((v25->field_0x12 && (v25->field_0x12 != '\x04')))) && ((a0->field_0x48 & 0x10 || (v25->field_0x12 != '\n'))));
      *(unsigned short *)&v10[0x68] = 0xb;
      sub_c619((struct_21 *)&v10[0x70],(unsigned char)v25->field_0x12);
      sub_b213(v10,!v4);
    }
    *(unsigned long *)&v10[0x10] = 0;
    if (v9) // branch-flip
      *(char **)&v17[0x10] = v10;
    else {
      v9 = v10;
    }
    v17 = v10;
    if ((v13 == 10000) && (!a0->field_0x40))
      v15 = sub_bc8c(v20,v11);
    v13 += 1;
  } while (v13 < v22);
label_cee4:
  if (v3)
    sub_d56c(a0,v9);
  if (a0->field_0x48 & 4) {
    if ((v23 == a0->field_0x30) || (!v13))
      v19 = &v19[-1];
    *v19 = 0;
  }
  if (((!v14) && (v1)) && ((a1 == 1 || (!v13)))) {
    if (*(long *)&v20[0x58]) // branch-flip
      v7 = sub_d6d9(a0,*(char **)&v20[8],0xffffffff,"..");
    else {
      v7 = sub_b2eb(a0);
    }
    if (v7) {
      *(unsigned short *)&v20[0x68] = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      sub_d45a(v9);
      return NULL;
    }
  }
  if (v13) {
    if (v15) {
      a0->field_0x40 = (long)sub_c5c5;
      v9 = (char *)sub_d236(a0,v9,v13);
      a0->field_0x40 = 0;
    }
    if ((a0->field_0x40) && (2 <= v13))
      v9 = (char *)sub_d236(a0,v9,v13);
    return v9;
  }
  if (((a1 == 3) && (*(short *)&v20[0x68] != 4)) && (*(short *)&v20[0x68] != 7))
    *(unsigned short *)&v20[0x68] = 6;
  sub_d45a(v9);
  return NULL;
}

// Function: sub_d069 @ 0xd069
unsigned long sub_d069(char *a0,char *a1,bool a2) // early-return x7, ternary
{
  stat *v1;
  int v2;
  char *v3;
  int v4;
  bool v5; // stack - 0x34
  
  v1 = (stat *)&a1[0x70];
  if ((*(unsigned int *)&a0[0x48] & 2) || ((v5 = a2, *(unsigned int *)&a0[0x48] & 1 && (!*(long *)&a1[0x58]))))
    v5 = 1;
  v4 = (v5) ? 0 : 0x100; // branch-flip
  v3 = *(char **)&a1[0x30];
  v2 = *(int *)&a0[0x2c];
  if (fstatat(v2,v3,v1,v4) <= -1) {
    if (((v5) && (*__errno_location() == 2)) && (v3 = *(char **)&a1[0x30], v4 = *(int *)&a0[0x2c], 0 <= fstatat(v4,v3,v1,0x100))) {
      *__errno_location() = 0;
      return 0xd;
    }
    *(int *)&a1[0x40] = *__errno_location();
    memset(v1,0,0x90);
    return 10;
  }
  if ((*(unsigned int *)&a1[0x88] & 0xf000) == 0x4000) {
    if ((a1[0x100] == '.') && ((!a1[0x101] || ((a1[0x101] == '.' && (!a1[0x102])))))) {
      if (!*(long *)&a1[0x58])
        return 1;
      return 5;
    }
    return 1;
  }
  if ((*(unsigned int *)&a1[0x88] & 0xf000) == 0xa000)
    return 0xc;
  if ((*(unsigned int *)&a1[0x88] & 0xf000) == 0x8000)
    return 8;
  return 3;
}

// Function: sub_d1f1 @ 0xd1f1
void sub_d1f1(long *a0,unsigned long a1)
{
  (**(void **)(*(long *)(*a0 + 0x50) + 0x40))(a0,a1);
}

// Function: sub_d236 @ 0xd236
char * sub_d236(char *a0,char *a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  char *v3;
  void *v4; // rax
  unsigned long *v5;
  long *v6;
  
  v2 = *(unsigned long *)&a0[0x40];
  if (*(unsigned long *)&a0[0x38] < a2) {
    *(unsigned long *)&a0[0x38] = a2 + 0x28;
    if ((0x2000000000000000 <= *(unsigned long *)&a0[0x38]) || (v4 = realloc(*(void **)&a0[0x10],*(long *)&a0[0x38] * 8), !v4)) {
      free(*(void **)&a0[0x10]);
      *(unsigned long *)&a0[0x10] = 0;
      *(unsigned long *)&a0[0x38] = 0;
      return a1;
    }
    *(void **)&a0[0x10] = v4;
  }
  v5 = *(unsigned long **)&a0[0x10];
  while (a1) {
    v1 = &v5[1];
    *v5 = a1;
    a1 = *(char **)&a1[0x10];
    v5 = v1;
  }
  qsort(*(unsigned long *)&a0[0x10],a2,8,v2);
  v6 = *(long **)&a0[0x10];
  v3 = (char *)*v6;
  while (a2 = a2 - 1, a2) {
    *(long *)(*v6 + 0x10) = v6[1];
    v6 = &v6[1];
  }
  *(unsigned long *)(*v6 + 0x10) = 0;
  return v3;
}

// Function: sub_d3a4 @ 0xd3a4
void * sub_d3a4(struct_22 *a0,void *a1,unsigned long a2) // early-return
{
  void *v1; // rax
  
  v1 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (!v1)
    return NULL;
  memcpy((void *)((long)v1 + 0x100),a1,a2);
  *(char *)((long)v1 + a2 + 0x100) = 0;
  *(unsigned long *)((long)v1 + 0x60) = a2;
  *(struct_22 **)((long)v1 + 0x50) = a0;
  *(unsigned long *)((long)v1 + 0x38) = a0->field_0x20;
  *(unsigned int *)((long)v1 + 0x40) = 0;
  *(unsigned long *)((long)v1 + 0x18) = 0;
  *(unsigned short *)((long)v1 + 0x6a) = 0;
  *(unsigned short *)((long)v1 + 0x6c) = 3;
  *(unsigned long *)((long)v1 + 0x20) = 0;
  *(unsigned long *)((long)v1 + 0x28) = 0;
  return v1;
}

// Function: sub_d45a @ 0xd45a
void sub_d45a(char *a0)
{
  char *v1;
  
  while (a0) {
    v1 = *(char **)&a0[0x10];
    if (*(long *)&a0[0x18])
      closedir(*(DIR **)&a0[0x18]);
    free(a0);
    a0 = v1;
  }
}

// Function: sub_d49b @ 0xd49b
unsigned long sub_d49b(char *a0,long a1) // return-dupe x2
{
  unsigned long v1; // rax
  void *v2; // rax
  
  v1 = a1 + *(long *)&a0[0x30] + 0x100;
  if (v1 < *(unsigned long *)&a0[0x30]) {
    free(*(void **)&a0[0x20]);
    *(unsigned long *)&a0[0x20] = 0;
    *__errno_location() = 0x24;
    return 0;
  }
  *(unsigned long *)&a0[0x30] = v1;
  v2 = realloc(*(void **)&a0[0x20],*(unsigned long *)&a0[0x30]);
  if (!v2) {
    free(*(void **)&a0[0x20]);
    *(unsigned long *)&a0[0x20] = 0;
    return 0;
  }
  *(void **)&a0[0x20] = v2;
  return 1;
}

// Function: sub_d56c @ 0xd56c
void sub_d56c(char *a0,char *a1) // ternary
{
  long v1;
  char *v2; // stack - 0x18
  
  v1 = *(long *)&a0[0x20];
  for (v2 = *(char **)&a0[8]; v2; v2 = *(char **)((long)v2 + 0x10)) {
    if (*(long *)((long)v2 + 0x30) != (long)v2 + 0x100)
      *(long *)((long)v2 + 0x30) = (*(long *)((long)v2 + 0x30) - *(long *)((long)v2 + 0x38)) + v1;
    *(long *)((long)v2 + 0x38) = v1;
  }
  v2 = a1;
  while (0 <= *(long *)&v2[0x58]) {
    if (*(char **)&v2[0x30] != &v2[0x100])
      *(long *)&v2[0x30] = (*(long *)&v2[0x30] - *(long *)&v2[0x38]) + v1;
    *(long *)&v2[0x38] = v1;
    v2 = (*(long *)&v2[0x10]) ? *(char **)&v2[0x10] : *(char **)&v2[8]; // branch-flip
  }
}

// Function: sub_d67f @ 0xd67f
long sub_d67f(long *a0)
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

// Function: sub_d6d9 @ 0xd6d9
int sub_d6d9(struct_23 *a0,char *a1,int a2,char *a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_d6d9
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
  if ((((a2 <= -1) && (v1)) && (a0->field_0x48 & 0x200)) && ((sub_10380(&a0[1].field_0x0[0x14]) != '\x01' && (v2 = sub_10422(&a0[1].field_0x0[0x14]), 0 <= v2)))) {
    v5 = v2;
    v4 = NULL;
  }
  v7 = v5;
  if (v5 <= -1) {
    v7 = sub_b399(a0,v4);
    if (v7 <= -1)
      return -1;
  }
  if ((a0->field_0x48 & 2) || ((v4 && (!strcmp(v4,".."))))) {
    if (fstat(v7,&v3)) {
      v6 = -1;
      goto label_d95c;
    }
    if ((*(long *)&a1[0x70] != v3._0_8_) || (*(long *)&a1[0x78] != v3._8_8_)) {
      *__errno_location() = 2;
      v6 = -1;
      goto label_d95c;
    }
  }
  if (a0->field_0x48 & 0x200) {
    sub_b25b(a0,v7,!v1);
    return 0;
  }
  v6 = fchdir(v7);
label_d95c:
  if (v5 <= -1) {
    v2 = *__errno_location();
    close(v7);
    *__errno_location() = v2;
  }
  return v6;
}

// Function: sub_d9a8 @ 0xd9a8
unsigned long sub_d9a8(struct_19 *a0)
{
  return a0->field_0x10;
}

// Function: sub_d9be @ 0xd9be
unsigned long sub_d9be(struct_36 *a0)
{
  return a0->field_0x18;
}

// Function: sub_d9d4 @ 0xd9d4
unsigned long sub_d9d4(struct_22 *a0)
{
  return a0->field_0x20;
}

// Function: sub_d9ea @ 0xd9ea
unsigned long sub_d9ea(struct_37 *a0)
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

// Function: sub_da6c @ 0xda6c
unsigned long sub_da6c(struct_37 *a0) // return-dupe
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

// Function: sub_db0a @ 0xdb0a
void sub_db0a(struct_22 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_d9d4(a0);
  v2 = sub_d9a8(a0);
  v3 = sub_d9be(a0);
  v4 = sub_d9ea(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_1aac8 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_dc51 @ 0xdc51
long sub_dc51(struct_4 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}

// Function: sub_dcac @ 0xdcac
long sub_dcac(struct_6 *a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_dc51(a0,a1);
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

// Function: sub_dd43 @ 0xdd43
long sub_dd43(struct_37 *a0) // early-return
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

// Function: sub_dda3 @ 0xdda3
long sub_dda3(char *a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_dc51(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)&a0[8] <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}

// Function: sub_de41 @ 0xde41
unsigned long sub_de41(struct_37 *a0,long a1,unsigned long a2)
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

// Function: sub_dee1 @ 0xdee1
long sub_dee1(struct_37 *a0,void *a1,unsigned long a2)
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

// Function: sub_df79 @ 0xdf79
unsigned long sub_df79(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_dfd3 @ 0xdfd3
unsigned long sub_dfd3(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_e042 @ 0xe042
unsigned long sub_e042(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_dfd3(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_e08d @ 0xe08d
void sub_e08d(struct_38 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_1aad0;
  a0->field_0x8 = dat_1aad4;
  a0->field_0xc = dat_1aad8;
  a0->field_0x10 = 0;
}

// Function: sub_e0e3 @ 0xe0e3
unsigned long sub_e0e3(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_175f6(a0,3) % a1;
}

// Function: sub_e11e @ 0xe11e
unsigned long sub_e11e(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_e13b @ 0xe13b
unsigned long sub_e13b(struct_2 *a0) // return-dupe x2
{
  float *v1;
  
  v1 = a0->field_0x28;
  if (v1 == (float *)0x1aa40)
    return 1;
  if ((((dat_1aadc < v1[2]) && (v1[2] < dat_1aad0 - dat_1aadc)) && (dat_1aadc + dat_1aad0 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_1aadc < v1[1])) && ((v1[1] <= dat_1aad0 && (*v1 + dat_1aadc < v1[2]))))))
    return 1;
  a0->field_0x28 = (float *)0x1aa40;
  return 0;
}

// Function: sub_e248 @ 0xe248
unsigned long sub_e248(unsigned long a0,char *a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (a1[0x10] != '\x01') {
    v2 = (float)a0 / *(float *)&a1[8];
    if (dat_1aae0 <= v2)
      return 0;
    if (dat_1aae4 <= v2)
      v3 = (long)(v2 - dat_1aae4) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_e042(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_e361 @ 0xe361
long * sub_e361(unsigned long a0,char *a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  char *v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_e0e3;
  v3 = a3;
  if (!a3)
    v3 = sub_e11e;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = (char *)0x1aa40;
  v2[5] = (long)v5;
  if (sub_e13b(v2) == '\x01') {
    v2[2] = sub_e248(a0,v5);
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

// Function: sub_e4d6 @ 0xe4d6
void sub_e4d6(struct_39 *a0)
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

// Function: sub_e5e1 @ 0xe5e1
void sub_e5e1(struct_14 *a0)
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

// Function: sub_e710 @ 0xe710
void * sub_e710(struct_3 *a0)
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

// Function: sub_e75f @ 0xe75f
void sub_e75f(struct_3 *a0,struct_5 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = a0->field_0x48;
  a0->field_0x48 = a1;
}

// Function: sub_e799 @ 0xe799
long sub_e799(struct_6 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_5 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_dc51(a0,a1);
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
      v3 = (struct_5 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_e75f(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_5 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_e75f(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_e932 @ 0xe932
unsigned long sub_e932(struct_9 *a0,struct_37 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_8 *v3; // rax
  struct_8 *v4; // rax
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
        v3 = (struct_8 *)sub_dc51(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_e75f(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_8 *)sub_dc51(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_8 *)sub_e710(a0);
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

// Function: sub_eb06 @ 0xeb06
unsigned long sub_eb06(struct_10 *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  char *v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_e248(a1,a0->field_0x28);
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
  if (sub_e932(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_e932(a0,&v1,1) == '\x01') && (sub_e932(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_ed31 @ 0xed31
unsigned long sub_ed31(char *a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_e799(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)&a0[0x10] * *(float *)(*(long *)&a0[0x28] + 8) < (float)*(unsigned long *)&a0[0x18]) && (sub_e13b(a0), (float)*(unsigned long *)&a0[0x10] * *(float *)(*(long *)&a0[0x28] + 8) < (float)*(unsigned long *)&a0[0x18])) {
    v6 = *(long *)&a0[0x28];
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)&a0[0x10] * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)&a0[0x10]); // branch-flip
    if (dat_1aae0 <= v4)
      return 0xffffffff;
    v1 = (dat_1aae4 <= v4) ? (long)(v4 - dat_1aae4) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_eb06(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_e799(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)&a0[0x20] = *(long *)&a0[0x20] + 1;
    *(long *)&a0[0x18] = *(long *)&a0[0x18] + 1;
    return 1;
  }
  v2 = (long *)sub_e710(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)&a0[0x20] = *(long *)&a0[0x20] + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_f0a2 @ 0xf0a2
unsigned long sub_f0a2(char *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_ed31(a0,a1,&v3);
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

// Function: sub_f112 @ 0xf112
long sub_f112(char *a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_e799(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)&a0[0x20] = *(long *)&a0[0x20] + -1;
  if (((!*v2) && (*(long *)&a0[0x18] = *(long *)&a0[0x18] + -1, (float)*(unsigned long *)&a0[0x18] < (float)*(unsigned long *)&a0[0x10] * **(float **)&a0[0x28])) && (sub_e13b(a0), (float)*(unsigned long *)&a0[0x18] < (float)*(unsigned long *)&a0[0x10] * **(float **)&a0[0x28])) {
    v6 = *(long *)&a0[0x28];
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)&a0[0x10] * *(float *)(v6 + 4);
      v7 = (dat_1aae4 <= v3) ? (long)(v3 - dat_1aae4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)&a0[0x10]);
      v7 = (dat_1aae4 <= v3) ? (long)(v3 - dat_1aae4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_eb06(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)&a0[0x48];
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)&a0[0x48] = 0;
    }
  }
  return v5;
}

// Function: sub_f418 @ 0xf418
void sub_f418(char *a0,long a1)
{
  sub_f112(a0,a1);
}

// Function: sub_f441 @ 0xf441
void sub_f441(void)
{
  return;
}

// Function: sub_f52a @ 0xf52a
void * sub_f52a(void *a0,unsigned long a1,unsigned char *a2,char *a3)
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

// Function: sub_f661 @ 0xf661
char * sub_f661(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_f661
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
label_fbb0:
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
      goto label_ff6f;
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
    goto label_fbb0;
  }
  v11 = (long double)(long)a4;
  if ((long)a4 <= -1)
    v11 = dat_1ab80 + v11;
  v12 = (long double)(long)a0;
  if ((long)a0 <= -1)
    v12 = dat_1ab80 + v12;
  v13 = (long double)(long)a3;
  if ((long)a3 <= -1)
    v13 = dat_1ab80 + v13;
  if (a2 & 0x10) { // branch-flip
    v27 = (long double)1;
    v15 = 0;
    do {
      v27 = v27 * (long double)v5;
      v15 += 1;
      if ((v13 / v11) * v12 < v27 * (long double)v5) break;
    } while (v15 < 8);
    sub_f441(v4);
    sprintf(a1,"%.1Lf");
    v24 = strlen(a1);
    v25 = v22 + 1;
    if ((v25 + (unsigned long)((a2 & 0x20) == 0) + 1 < v24) || ((a2 & 8 && (a1[v24 - 1] == '0')))) {
      sub_f441(v4);
      sprintf(a1,"%.0Lf");
      v24 = strlen(a1);
      v25 = 0;
    }
  }
  else {
    sub_f441(v4);
    sprintf(a1,"%.0Lf");
    v24 = strlen(a1);
    v25 = 0;
  }
  v18 = &v9[-v24];
  memmove(v18,a1,v24);
  v20 = &v18[v24 - v25];
label_ff6f:
  if (a2 & 4)
    v18 = (char *)sub_f52a(v18,(long)v20 - (long)v18,v1,v23);
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
        v10 = *(char *)((long)v15 + 0x1aaf0);
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

// Function: sub_100fb @ 0x100fb
unsigned long sub_100fb(void) // return-dupe
{
  if (getenv("POSIXLY_CORRECT"))
    return 0x200;
  return 0x400;
}

// Function: sub_10125 @ 0x10125
int sub_10125(unsigned char *a0,unsigned long *a1,unsigned int *a2)
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
        *a1 = sub_100fb();
        goto label_102a9;
      }
    }
  }
  if (*v4 == 0x27) {
    v5 = 4;
    v4 = &v4[1];
  }
  v2 = sub_6030(v4,0x20a20,0x1ab20,4);
  if (0 <= v2) { // branch-flip
    v5 |= *(unsigned int *)((long)v2 * 4 + 0x1ab20);
    *a1 = 1;
  }
  else {
    v2 = sub_161e3(v4,&v3,0,a1,"eEgGkKmMpPtTyYzZ0");
    if (v2) {
      *a2 = 0;
      return v2;
    }
    while( true ) {
      if (('0' <= (char)*v4) && ((char)*v4 <= '9')) goto label_102a9;
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
label_102a9:
  *a2 = v5;
  return 0;
}

// Function: sub_102cd @ 0x102cd
unsigned int sub_102cd(unsigned char *a0,unsigned int *a1,long *a2)
{
  unsigned int v1; // stack - 0xc
  
  v1 = sub_10125(a0,a2,a1);
  if (!*a2) {
    *a2 = sub_100fb();
    v1 = 4;
  }
  return v1;
}

// Function: sub_10323 @ 0x10323
void sub_10323(char *a0,unsigned int a1)
{
  int v1; // stack - 0xc
  
  a0[0x1c] = 1;
  *(unsigned int *)&a0[0x14] = 0;
  *(unsigned int *)&a0[0x18] = 0;
  for (v1 = 0; v1 <= 3; v1 = v1 + 1) {
    *(unsigned int *)&a0[(long)v1 * 4] = a1;
  }
  *(unsigned int *)&a0[0x10] = a1;
}

// Function: sub_10380 @ 0x10380
char sub_10380(char *a0)
{
  return a0[0x1c];
}

// Function: sub_10396 @ 0x10396
unsigned int sub_10396(char *a0,unsigned int a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  
  v2 = (unsigned int)(a0[0x1c] ^ 1) + *(int *)&a0[0x14] & 3;
  v1 = *(unsigned int *)&a0[(unsigned long)v2 * 4];
  *(unsigned int *)&a0[(unsigned long)v2 * 4] = a1;
  *(unsigned int *)&a0[0x14] = v2;
  if (v2 == *(unsigned int *)&a0[0x18])
    *(unsigned int *)&a0[0x18] = (unsigned int)(a0[0x1c] ^ 1) + *(int *)&a0[0x18] & 3;
  a0[0x1c] = 0;
  return v1;
}

// Function: sub_10422 @ 0x10422
unsigned int sub_10422(char *a0)
{
  unsigned int v1;
  
  if (sub_10380(a0))
    abort(); // no-return
  v1 = *(unsigned int *)&a0[(unsigned long)*(unsigned int *)&a0[0x14] * 4];
  *(unsigned int *)&a0[(unsigned long)*(unsigned int *)&a0[0x14] * 4] = *(unsigned int *)&a0[0x10];
  if (*(int *)&a0[0x14] != *(int *)&a0[0x18]) // branch-flip
    *(unsigned int *)&a0[0x14] = *(int *)&a0[0x14] + 3U & 3;
  else {
    a0[0x1c] = 1;
  }
  return v1;
}

// Function: sub_104a8 @ 0x104a8
unsigned long sub_104a8(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_10517 @ 0x10517
unsigned long sub_10517(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_1054d @ 0x1054d
long * sub_1054d(long a0)
{
  long *v1; // rax
  
  v1 = malloc(0x18);
  if (v1) {
    *v1 = sub_e361(0x3fd,NULL,sub_104a8,sub_10517,dat_20fa8);
    if (!*v1) {
      free(v1);
      return NULL;
    }
    v1[1] = a0;
    v1[2] = 0;
  }
  return v1;
}

// Function: sub_105e6 @ 0x105e6
void sub_105e6(struct_32 *a0)
{
  sub_e5e1(a0->field_0x0);
  free(a0->field_0x10);
  free(a0);
}

// Function: sub_10624 @ 0x10624
long sub_10624(unsigned long *a0,long a1) // early-return x2
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x18
  
  v3 = (long *)a0[2];
  if (v3) { // branch-flip
    if (a1 == *v3)
      return v3[1];
  }
  else {
    v3 = malloc(0x10);
    a0[2] = v3;
    if (!v3)
      return -1;
  }
  *v3 = a1;
  v2 = (long *)sub_f0a2((char *)*a0,v3);
  if (v2) {
    if (v2 != v3) // branch-flip
      v3[1] = v2[1];
    else {
      a0[2] = 0;
      v1 = a0[1];
      a0[1] = v1 + 1;
      v3[1] = v1;
    }
    return v3[1];
  }
  return -1;
}

// Function: sub_10715 @ 0x10715
char * sub_10715(long a0,char *a1)
{
  char *v1;
  long v2; // stack - 0x20
  char *v3; // stack - 0x10
  
  v3 = &a1[0x14];
  *v3 = '\0';
  v2 = a0;
  v1 = v3;
  if (0 <= a0) { // branch-flip
    do {
      v3 = &v3[-1];
      *v3 = (char)v2 + (char)(v2 / 10) * '\xf6' + '0';
      v2 /= 10;
    } while (v2);
  }
  else {
    do {
      v3 = v1;
      v3[-1] = 0x30 - ((char)v2 + (char)(v2 / 10) * '\xf6');
      v2 /= 10;
      v1 = &v3[-1];
    } while (v2);
    v3 = &v3[-2];
    *v3 = '-';
  }
  return v3;
}

// Function: sub_10853 @ 0x10853
int sub_10853(unsigned char *a0,unsigned char *a1) // early-return, ternary x2
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
      sub_10c3a(v8);
      if ((v15 != '\x01') || (v16))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_10c3a(v9);
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
    sub_10c3a(v8);
    if ((v15 != '\x01') || (v16))
      v1 = 1;
    else {
      v1 = 0;
    }
    if (v1)
      return 1;
    sub_10c3a(v9);
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

// Function: sub_10c3a @ 0x10c3a
void sub_10c3a(struct_15 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_1812d((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_10e40;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","lib/mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_1411b(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_18172(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
  if (a0->field_0x18 != -1) { // branch-flip
    if (a0->field_0x18 != -2) { // branch-flip
      if (!a0->field_0x18) {
        a0->field_0x18 = 1;
        if (*a0->field_0x10)
          __assert_fail("*iter->cur.ptr == \'\\0\'","lib/mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
        if (a0->field_0x24)
          __assert_fail("iter->cur.wc == 0","lib/mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
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
label_10e40:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_10e51 @ 0x10e51
void sub_10e51(char *a0,long a1)
{
  *(long *)&a0[0x10] = *(long *)&a0[0x10] + a1;
}

// Function: sub_10e7b @ 0x10e7b
void sub_10e7b(struct_42 *a0,struct_42 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_18087((struct_40 *)&a0[1].field_0x1[2],(struct_41 *)&a1[1].field_0x1[2]);
}

// Function: sub_10f10 @ 0x10f10
unsigned long sub_10f10(void)
{
  return 0xffffffffffffffff;
}

// Function: sub_10f25 @ 0x10f25
void sub_10f25(char *a0)
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

// Function: sub_110ce @ 0x110ce
char * sub_110ce(char *a0)
{
  char *v1; // rax
  
  v1 = strchr(a0,0x20);
  if (v1)
    *v1 = '\0';
  return v1;
}

// Function: sub_11107 @ 0x11107
long * sub_11107(void) // warn: iteboolean: re-rolled 4 0/1 select diamond(s) to a boolean assignment in sub_11107
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
        v23 = sub_110ce(v22);
        if (v23) {
          v24 = (char *)(v23 + 1);
          v23 = sub_110ce(v24);
          if (v23) {
            v25 = strstr((char *)(v23 + 1)," - ");
            if (v25) {
              v26 = &v25[3];
              v23 = sub_110ce(v26);
              if (v23) {
                v27 = (char *)(v23 + 1);
                if (sub_110ce(v27)) {
                  sub_10f25(v27);
                  sub_10f25(v24);
                  sub_10f25(v22);
                  sub_10f25(v26);
                  v21 = (long *)sub_1535a(0x38);
                  v5 = sub_15961(v27);
                  *v21 = v5;
                  v5 = sub_15961(v24);
                  v21[1] = v5;
                  v5 = sub_15961(v22);
                  v21[2] = v5;
                  v5 = sub_15961(v26);
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
      sub_7ea0(v18);
      *__errno_location() = v13;
    }
    else {
      v4 = sub_7ea0(v18);
      if (v4 != -1) goto label_11d20;
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
      v21 = (long *)sub_1535a(0x38);
      v1 = (char *)*v20;
      *v21 = sub_15961(v1);
      v5 = sub_15961((char *)v20[1]);
      v21[1] = v5;
      v21[2] = 0;
      v5 = sub_15961((char *)v20[2]);
      v21[3] = v5;
      *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] | 4;
      v1 = (char *)v21[3];
      v2 = !(((((((strcmp(v1,"autofs")) && (v1 = (char *)v21[3], strcmp(v1,"proc"))) && (v1 = (char *)v21[3], strcmp(v1,"subfs"))) && ((v1 = (char *)v21[3], strcmp(v1,"debugfs") && (v1 = (char *)v21[3], strcmp(v1,"devpts"))))) && (v1 = (char *)v21[3], strcmp(v1,"fusectl"))) && (((v1 = (char *)v21[3], strcmp(v1,"fuse.portal") && (v1 = (char *)v21[3], strcmp(v1,"mqueue"))) && ((v1 = (char *)v21[3], strcmp(v1,"rpc_pipefs") && (((v1 = (char *)v21[3], strcmp(v1,"sysfs") && (v1 = (char *)v21[3], strcmp(v1,"devfs"))) && (v1 = (char *)v21[3], strcmp(v1,"kernfs"))))))))) && ((v1 = (char *)v21[3], strcmp(v1,"ignore") && ((v1 = (char *)v21[3], strcmp(v1,"none") || (v6))))));
      *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] & 0xfe | v2;
      v1 = (char *)*v21;
      v3 = (((strchr(v1,0x3a)) || ((((*(char *)*v21 == '/' && (*(char *)(*v21 + 1) == '/')) && ((v1 = (char *)v21[3], !strcmp(v1,"smbfs") || ((v1 = (char *)v21[3], !strcmp(v1,"smb3") || (v1 = (char *)v21[3], !strcmp(v1,"cifs"))))))) || (v1 = (char *)v21[3], !strcmp(v1,"acfs"))))) || (((((v1 = (char *)v21[3], !strcmp(v1,"afs") || (v1 = (char *)v21[3], !strcmp(v1,"coda"))) || (v1 = (char *)v21[3], !strcmp(v1,"auristorfs"))) || ((v1 = (char *)v21[3], !strcmp(v1,"fhgfs") || (v1 = (char *)v21[3], !strcmp(v1,"gpfs"))))) || (((v1 = (char *)v21[3], !strcmp(v1,"ibrix") || ((v1 = (char *)v21[3], !strcmp(v1,"ocfs2") || (v1 = (char *)v21[3], !strcmp(v1,"vxfs"))))) || (v1 = (char *)*v21, !strcmp("-hosts",v1)))))));
      *(unsigned char *)&v21[5] = *(unsigned char *)&v21[5] & 0xfd | v3 * '\x02';
      v5 = sub_10f10(v20[3]);
      v21[4] = v5;
      *v16 = v21;
      v16 = (long **)&v21[6];
    }
    if (endmntent(v18)) {
label_11d20:
      *v16 = NULL;
      return v7;
    }
  }
  v15 = *__errno_location();
  *v16 = NULL;
  while (v7) {
    v21 = (long *)v7[6];
    sub_11d9d(v7);
    v7 = v21;
  }
  *__errno_location() = v15;
  return NULL;
}

// Function: sub_11d9d @ 0x11d9d
void sub_11d9d(struct_13 *a0)
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0->field_0x10);
  if (a0->field_0x28 & 4)
    free(a0->field_0x18);
  free(a0);
}

// Function: sub_11e0a @ 0x11e0a
void sub_11e0a(int a0,char *a1,unsigned int a2)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa0
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = v2;
  sub_149b4(openat(a0,a1,a2,(unsigned long)v1));
}

// Function: sub_11f48 @ 0x11f48
DIR * sub_11f48(unsigned int a0,char *a1,unsigned int a2,int *a3) // early-return
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  
  v2 = sub_11e0a(a0,a1,a2 | 0x90900);
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

// Function: sub_11fda @ 0x11fda
void sub_11fda(char *a0)
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
        __progname = &v1[3];
        v2 = __progname;
      }
    }
  }
  dat_211d0 = v2;
  program_invocation_name = v2;
}

// Function: sub_120c5 @ 0x120c5
unsigned long sub_120c5(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x211e0;
  v2 = sub_1589a(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_12114 @ 0x12114
unsigned int sub_12114(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x211e0;
  return *a0;
}

// Function: sub_12138 @ 0x12138
void sub_12138(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x211e0;
  *a0 = a1;
}

// Function: sub_12163 @ 0x12163
unsigned int sub_12163(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x211e0;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_121f3 @ 0x121f3
unsigned int sub_121f3(char *a0,unsigned int a1)
{
  unsigned int v1;
  char *v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = (char *)0x211e0;
  v1 = *(unsigned int *)&v2[4];
  *(unsigned int *)&v2[4] = a1;
  return v1;
}

// Function: sub_1222d @ 0x1222d
void sub_1222d(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x211e0;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_1228f @ 0x1228f
unsigned long * sub_1228f(unsigned long *a0,unsigned int a1)
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

// Function: sub_12354 @ 0x12354
char * sub_12354(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_18004();
  if (!sub_17705(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_17705(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x1adde;
  return (char *)0x1ade2;
}

// Function: sub_12419 @ 0x12419
unsigned long sub_12419(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_124d3:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_12627:
      v14 = 1;
      goto label_1262b;
    case 2:
label_1263a:
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
      goto label_12627;
    case 4:
label_1262b:
      if (!v14)
        v2 = 1;
      goto label_1263a;
    case 5:
label_12515:
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
      goto label_12515;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_12354("`",v19);
        v17 = (char *)sub_12354("\'",v19);
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
label_132d9:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_13311;
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
      if (v14) goto label_13455;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_12bf8:
    if (!v24) {
label_12c03:
      v16 = 1;
label_12c07:
      if (v19 != 2) goto label_1311e;
label_129e0:
      if (v14) goto label_13455;
    }
    goto label_1311e;
  }
  if (0x7e < v22) {
label_12ce4:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_12d5e:
      v31 = sub_18172(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_12e0c;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_12ebb;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_12ebb:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_1311e;
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
        if (v14) goto label_13455;
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
    goto label_13244;
  }
  if (v22 == 0x7d) {
label_12bc6:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_1311e;
    goto label_12bf8;
  }
  if (0x7d < v22) goto label_12ce4;
  if (v22 == 0x7c) goto label_12c07;
  if (0x7c < v22) goto label_12ce4;
  if (v22 == 0x7b) goto label_12bc6;
  if (0x7b < v22) goto label_12ce4;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_12ce4;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_12cdb;
    if (v11 & 0xa4000000) goto label_12c07;
    if (!(v11 & 0x8000000)) goto label_12ce4;
    if (v19 == 2) {
      if (!v14) goto label_13244;
      goto label_13455;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_12b9c;
    goto label_13244;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_1311e;
        goto label_132d4;
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
        goto label_1311e;
      }
      goto label_13455;
    default:
      goto label_12ce4;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_12b9c;
    case 10:
      v23 = 0x6e;
      goto label_12b9c;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_12b9c:
      if ((v19 == 2) && (v14)) goto label_13455;
      break;
    case 0x20:
      goto label_12c03;
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
      goto label_12c07;
    case 0x23:
      goto label_12bf8;
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
label_12cdb:
      v16 = 1;
      goto label_1311e;
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
          goto label_1311e;
        }
        goto label_13455;
      }
      goto label_1311e;
    case 0x3f:
      if (v19 == 2) goto label_129e0;
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
          goto label_1311e;
        }
        goto label_13455;
      }
      goto label_1311e;
    
  }
  if (v2) {
    v22 = v23;
label_13189:
    if (v14) goto label_13455;
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
label_1311e:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_13189;
  }
label_13244:
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
label_132d4:
  v24 += 1;
  goto label_132d9;
label_12e0c:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_13455;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_12ebb;
  goto label_12d5e;
label_13311:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_13455:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_12419(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_12419(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_124d3;
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

// Function: sub_134da @ 0x134da
unsigned long sub_134da(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x211e0;
  v1 = *__errno_location();
  v2 = sub_12419(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_13582 @ 0x13582
void sub_13582(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_135b8(a0,a1,NULL,a2);
}

// Function: sub_135b8 @ 0x135b8
long sub_135b8(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x211e0;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_12419(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_153a6(v3 + 1U);
  sub_12419(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_136e4 @ 0x136e4
void sub_136e4(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_210b0;
  for (v2 = 1; v2 < dat_21040; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x21220) {
    free(*(void **)((long)v1 + 8));
    dat_210a0 = 0x100;
    dat_210a8 = 0x21220;
  }
  if (v1 == (void *)0x210a0) {
    dat_21040 = 1;
    return;
  }
  free(v1);
  dat_21040 = 1;
  dat_210b0 = (void *)0x210a0;
}

// Function: sub_137a3 @ 0x137a3
void * sub_137a3(int a0,char *a1,unsigned long a2,struct_0 *a3)
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
  v6 = dat_210b0;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_21040 <= a0) {
      v5 = dat_210b0 == (unsigned long *)0x210a0;
      v4 = (long)dat_21040;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_210b0;
      }
      dat_210b0 = (unsigned long *)sub_15636(v3,&v4,(long)((a0 - dat_21040) + 1),0x7fffffff,0x10);
      v6 = dat_210b0;
      if (v5) {
        *dat_210b0 = dat_210a0;
        dat_210b0[1] = dat_210a8;
      }
      memset(&dat_210b0[(long)dat_21040 * 2],0,(v4 - dat_21040) * 0x10);
      dat_21040 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_12419(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x21220)
        free(v7);
      v7 = (void *)sub_153a6(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_12419(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_13a52 @ 0x13a52
void sub_13a52(unsigned int a0,char *a1)
{
  sub_137a3(a0,a1,0xffffffffffffffff,(struct_0 *)0x211e0);
}

// Function: sub_13a86 @ 0x13a86
void sub_13a86(unsigned int a0,char *a1,unsigned long a2)
{
  sub_137a3(a0,a1,a2,(struct_0 *)0x211e0);
}

// Function: sub_13ab8 @ 0x13ab8
void sub_13ab8(char *a0)
{
  sub_13a52(0,a0);
}

// Function: sub_13adb @ 0x13adb
void sub_13adb(char *a0,unsigned long a1)
{
  sub_13a86(0,a0,a1);
}

// Function: sub_13b06 @ 0x13b06
void sub_13b06(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_1228f(v1,a1);
  sub_137a3(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_13b6e @ 0x13b6e
void sub_13b6e(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_1228f(v1,a1);
  sub_137a3(a0,a2,a3,v1);
}

// Function: sub_13bd4 @ 0x13bd4
void sub_13bd4(unsigned int a0,char *a1)
{
  sub_13b06(0,a0,a1);
}

// Function: sub_13bfc @ 0x13bfc
void sub_13bfc(unsigned int a0,char *a1,unsigned long a2)
{
  sub_13b6e(0,a0,a1,a2);
}

// Function: sub_13c2c @ 0x13c2c
void sub_13c2c(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_211e0;
  v2 = dat_211e8;
  v3 = dat_211f0;
  v4 = dat_211f8;
  v5 = dat_21200;
  v6 = dat_21208;
  v7 = dat_21210;
  sub_12163(&v1,(int)a2,1);
  sub_137a3(0,a0,a1,&v1);
}

// Function: sub_13ce7 @ 0x13ce7
void sub_13ce7(char *a0,char a1)
{
  sub_13c2c(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_13d15 @ 0x13d15
void sub_13d15(char *a0)
{
  sub_13ce7(a0,0x3a);
}

// Function: sub_13d38 @ 0x13d38
void sub_13d38(char *a0,unsigned long a1)
{
  sub_13c2c(a0,a1,0x3a);
}

// Function: sub_13d66 @ 0x13d66
void sub_13d66(unsigned int a0,unsigned int a1,char *a2)
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
  sub_1228f(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_12163(&v2,0x3a,1);
  sub_137a3(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_13e28 @ 0x13e28
void sub_13e28(unsigned int a0,long a1,long a2,char *a3)
{
  sub_13e62(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_13e62 @ 0x13e62
void sub_13e62(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_211e0;
  v2 = dat_211e8;
  v3 = dat_211f0;
  v4 = dat_211f8;
  v5 = dat_21200;
  v6 = dat_21208;
  v7 = dat_21210;
  sub_1222d(&v1,a1,a2);
  sub_137a3(a0,a3,a4,&v1);
}

// Function: sub_13f20 @ 0x13f20
void sub_13f20(long a0,long a1,char *a2)
{
  sub_13e28(0,a0,a1,a2);
}

// Function: sub_13f53 @ 0x13f53
void sub_13f53(long a0,long a1,char *a2,unsigned long a3)
{
  sub_13e62(0,a0,a1,a2,a3);
}

// Function: sub_13f91 @ 0x13f91
void sub_13f91(unsigned int a0,char *a1,unsigned long a2)
{
  sub_137a3(a0,a1,a2,(struct_0 *)0x21060);
}

// Function: sub_13fc3 @ 0x13fc3
void sub_13fc3(char *a0,unsigned long a1)
{
  sub_13f91(0,a0,a1);
}

// Function: sub_13fee @ 0x13fee
void sub_13fee(unsigned int a0,char *a1)
{
  sub_13f91(a0,a1,0xffffffffffffffff);
}

// Function: sub_1401b @ 0x1401b
void sub_1401b(char *a0)
{
  sub_13fee(0,a0);
}

// Function: sub_1403e @ 0x1403e
unsigned long sub_1403e(char *a0)
{
  return *(unsigned long *)&a0[0x50];
}

// Function: sub_14054 @ 0x14054
unsigned long sub_14054(char *a0)
{
  return *(unsigned long *)&a0[0x70];
}

// Function: sub_1406a @ 0x1406a
unsigned long sub_1406a(char *a0)
{
  return *(unsigned long *)&a0[0x60];
}

// Function: sub_14080 @ 0x14080
unsigned long sub_14080(void)
{
  return 0;
}

// Function: sub_14093 @ 0x14093
undefined16 sub_14093(char *a0)
{
  return *(char (*)[16])&a0[0x48];
}

// Function: sub_140ad @ 0x140ad
undefined16 sub_140ad(char *a0)
{
  return *(char (*)[16])&a0[0x68];
}

// Function: sub_140c7 @ 0x140c7
undefined16 sub_140c7(char *a0)
{
  return *(char (*)[16])&a0[0x58];
}

// Function: sub_140e1 @ 0x140e1
undefined16 sub_140e1(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_14107 @ 0x14107
unsigned int sub_14107(unsigned int a0)
{
  return a0;
}

// Function: sub_1411b @ 0x1411b
unsigned long sub_1411b(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_14163 @ 0x14163
void sub_14163(void *a0,void *a1,unsigned long a2)
{
  memcpy(a0,a1,a2);
  *(char *)(a2 + (long)a0) = 0;
}

// Function: sub_141a3 @ 0x141a3
unsigned long * sub_141a3(char *a0) // ternary
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
      sub_14163((void *)((long)v3 + 9),a0,v1);
  }
  return v3;
}

// Function: sub_14258 @ 0x14258
unsigned long sub_14258(long *a0,struct_1 *a1) // return-dupe
{
  struct_1 *v1;
  long v2;
  long v3; // rax
  long *v4; // stack - 0x30
  char *v5; // stack - 0x20
  
  v5 = "";
  v1 = (struct_1 *)a1->field_0x30;
  if ((!v1) || ((a1 <= v1 && (v1 < &a1[1]))))
    return 1;
  if (v1->field_0x0[0]) {
    v5 = (char *)((long)a0 + 9);
    v4 = a0;
    while (strcmp(v5,(char *)v1)) {
      if ((!*v5) && ((v5 != (char *)((long)v4 + 9) || (!(char)v4[1])))) {
        v3 = strlen((char *)v1) + 1;
        if ((long)v4 + (0x80U - (long)v5) <= v3) { // branch-flip
          *v4 = sub_141a3(v1);
          v2 = *v4;
          if (!v2)
            return 0;
          *(char *)(v2 + 8) = 0;
          v5 = (char *)(v2 + 9);
        }
        else {
          sub_14163(v5,v1,v3);
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

// Function: sub_143fc @ 0x143fc
void sub_143fc(unsigned long *a0) // return-dupe
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

// Function: sub_14442 @ 0x14442
void sub_14442(void)
{
  getenv("TZ");
}

// Function: sub_1445b @ 0x1445b
void sub_1445b(char *a0) // return-dupe
{
  if (a0) {
    setenv("TZ",a0,1);
    return;
  }
  unsetenv("TZ");
}

// Function: sub_144a0 @ 0x144a0
bool sub_144a0(char *a0) // ternary
{
  char *v1; // rax
  bool v2; // zf
  
  v1 = (a0[8]) ? &a0[9] : NULL; // branch-flip
  v2 = sub_1445b(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_144ea @ 0x144ea
unsigned long * sub_144ea(char *a0) // early-return x2
{
  int v1;
  char *v2; // rax
  unsigned long *v3; // rax
  bool v4;
  
  v2 = (char *)sub_14442();
  if (v2) { // branch-flip
    if ((a0[8]) && (!strcmp(&a0[9],v2)))
      v4 = 1;
    else {
      v4 = 0;
    }
  }
  else {
    v4 = a0[8] == '\0';
  }
  if (v4)
    return (unsigned long *)0x1;
  v3 = (unsigned long *)sub_141a3(v2);
  if ((v3) && (sub_144a0(a0) != '\x01')) {
    v1 = *__errno_location();
    sub_143fc(v3);
    *__errno_location() = v1;
    return NULL;
  }
  return v3;
}

// Function: sub_145b7 @ 0x145b7
char sub_145b7(unsigned long *a0) // early-return
{
  char v1; // al
  int v2; // stack - 0xc
  
  if (a0 == (unsigned long *)0x1)
    return '\x01';
  v2 = *__errno_location();
  v1 = sub_144a0(a0);
  if (v1 != '\x01')
    v2 = *__errno_location();
  sub_143fc(a0);
  *__errno_location() = v2;
  return v1;
}

// Function: sub_14623 @ 0x14623
struct_1 * sub_14623(long *a0,void *a1,struct_1 *a2) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_14623
{
  bool v1; // al
  unsigned long *v2; // rax
  
  if (!a0)
    return (struct_1 *)gmtime_r(a1,(tm *)a2);
  v2 = (unsigned long *)sub_144ea(a0);
  if (v2) {
    v1 = ((localtime_r(a1,(tm *)a2)) && (sub_14258(a0,a2)));
    if ((sub_145b7(v2)) && (v1))
      return a2;
  }
  return NULL;
}

// Function: sub_146d3 @ 0x146d3
unsigned long sub_146d3(long *a0,unsigned long *a1) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_146d3
{
  bool v1; // al
  unsigned int v10; // stack - 0x40
  int v11; // stack - 0x3c
  unsigned int v12; // stack - 0x38
  unsigned int v13; // stack - 0x34
  unsigned long v14; // stack - 0x30
  unsigned long v15; // stack - 0x28
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned int v4; // stack - 0x58
  unsigned int v5; // stack - 0x54
  unsigned int v6; // stack - 0x50
  unsigned int v7; // stack - 0x4c
  unsigned int v8; // stack - 0x48
  unsigned int v9; // stack - 0x44
  
  if (!a0)
    return sub_14839(a1);
  v2 = (unsigned long *)sub_144ea(a0);
  if (v2) {
    v4 = *(unsigned int *)a1;
    v5 = *(unsigned int *)((long)a1 + 4);
    v6 = *(unsigned int *)&a1[1];
    v7 = *(unsigned int *)((long)a1 + 0xc);
    v8 = *(unsigned int *)&a1[2];
    v9 = *(unsigned int *)((long)a1 + 0x14);
    v11 = -1;
    v12 = *(unsigned int *)&a1[4];
    v3 = sub_17511(&v4);
    v1 = ((0 <= v11) && (sub_14258(a0,&v4)));
    if ((sub_145b7(v2)) && (v1)) {
      *a1 = CONCAT44(v5,v4);
      a1[1] = CONCAT44(v7,v6);
      a1[2] = CONCAT44(v9,v8);
      a1[3] = CONCAT44(v11,v10);
      a1[4] = CONCAT44(v13,v12);
      a1[5] = v14;
      a1[6] = v15;
      return v3;
    }
  }
  return 0xffffffffffffffff;
}

// Function: sub_14839 @ 0x14839
void sub_14839(struct_43 *a0)
{
  a0->field_0x20 = 0;
  sub_16cf3(a0,dat_20fc8,(long *)0x21320);
}

// Function: sub_14873 @ 0x14873
undefined16 sub_14873(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_1489d @ 0x1489d
int sub_1489d(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_14913 @ 0x14913
int sub_14913(long a0,long a1)
{
  return (unsigned int)(a1 && !a0) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_14965 @ 0x14965
double sub_14965(long a0,long a1)
{
  return (double)a1 / dat_1af28 + (double)a0;
}

// Function: sub_149b4 @ 0x149b4
int sub_149b4(int a0)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_1821c(a0);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}

// Function: sub_14a05 @ 0x14a05
void sub_14a05(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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

// Function: sub_15001 @ 0x15001
void sub_15001(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_14a05(a0,a1,a2,a3,a4,v1);
}

// Function: sub_15074 @ 0x15074
void sub_15074(FILE *a0,char *a1,char *a2,char *a3,struct_33 *a4)
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
  sub_14a05(a0,a1,a2,a3,v2,v3);
}

// Function: sub_1517a @ 0x1517a
void sub_1517a(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_15074(a8,a9,a10,a11,&v3);
}

// Function: sub_1525e @ 0x1525e
void sub_1525e(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-coreutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_15307 @ 0x15307
void sub_15307(void *a0,unsigned long a1,unsigned long a2)
{
  sub_15440(a0,a1,a2);
}

// Function: sub_15338 @ 0x15338
long sub_15338(long a0)
{
  if (!a0)
    sub_15992(); // no-return
  return a0;
}

// Function: sub_1535a @ 0x1535a
void sub_1535a(unsigned long a0)
{
  sub_15338(malloc(a0));
}

// Function: sub_15380 @ 0x15380
void sub_15380(unsigned long a0)
{
  sub_15338(sub_17f37(a0));
}

// Function: sub_153a6 @ 0x153a6
void sub_153a6(unsigned long a0)
{
  sub_1535a(a0);
}

// Function: sub_153c4 @ 0x153c4
void * sub_153c4(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_15992(); // no-return
  return v1;
}

// Function: sub_1540f @ 0x1540f
void sub_1540f(void *a0,unsigned long a1)
{
  sub_15338(sub_17f55(a0,a1));
}

// Function: sub_15440 @ 0x15440
void * sub_15440(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_15992(); // no-return
  return v1;
}

// Function: sub_1549a @ 0x1549a
void sub_1549a(void *a0,unsigned long a1,unsigned long a2)
{
  sub_15338(sub_17fb5(a0,a1,a2));
}

// Function: sub_154d3 @ 0x154d3
void sub_154d3(unsigned long a0,unsigned long a1)
{
  sub_15440(NULL,a0,a1);
}

// Function: sub_154fe @ 0x154fe
void sub_154fe(unsigned long a0,unsigned long a1)
{
  sub_1549a(NULL,a0,a1);
}

// Function: sub_15529 @ 0x15529
void sub_15529(void *a0,unsigned long *a1)
{
  sub_15557(a0,a1,1);
}

// Function: sub_15557 @ 0x15557
unsigned long sub_15557(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_15992(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_15440(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_15636 @ 0x15636
unsigned long sub_15636(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_15992(); // no-return
  v3 = sub_153c4(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_157f2 @ 0x157f2
void sub_157f2(unsigned long a0)
{
  sub_15838(a0,1);
}

// Function: sub_15815 @ 0x15815
void sub_15815(unsigned long a0)
{
  sub_15869(a0,1);
}

// Function: sub_15838 @ 0x15838
void sub_15838(unsigned long a0,unsigned long a1)
{
  sub_15338(calloc(a0,a1));
}

// Function: sub_15869 @ 0x15869
void sub_15869(unsigned long a0,unsigned long a1)
{
  sub_15338(sub_17f8c(a0,a1));
}

// Function: sub_1589a @ 0x1589a
void sub_1589a(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1535a(a1),a0,a1);
}

// Function: sub_158d2 @ 0x158d2
void sub_158d2(void *a0,unsigned long a1)
{
  memcpy((void *)sub_15380(a1),a0,a1);
}

// Function: sub_15912 @ 0x15912
void sub_15912(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_15380(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_15961 @ 0x15961
void sub_15961(char *a0)
{
  sub_1589a(a0,strlen(a0) + 1);
}

// Function: sub_15992 @ 0x15992
void sub_15992(void)
{
  error(dat_21028,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_159d2 @ 0x159d2
long sub_159d2(long *a0,unsigned int a1,long a2)
{
  long v1; // rax
  
  v1 = sub_b411(a0,a1 | 0x200,a2);
  if (v1)
    return v1;
  if (*__errno_location() == 0x16)
    __assert_fail("errno != EINVAL","lib/xfts.c",0x29,"xfts_open"); // no-return
  sub_15992(); // no-return
}

// Function: sub_15a4d @ 0x15a4d
unsigned int sub_15a4d(struct_25 *a0,struct_26 *a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_15a4d
{
  unsigned int v1; // eax
  
  v1 = (((a0->field_0x48 & 0x10) && (!(a0->field_0x48 & 1))) || (((a0->field_0x48 & 0x10 && (a0->field_0x48 & 1)) && (a1->field_0x58))));
  return v1;
}

// Function: sub_15ab3 @ 0x15ab3
unsigned long sub_15ab3(long *a0,int a1) // return-dupe, ternary
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

// Function: sub_15b55 @ 0x15b55
unsigned int sub_15b55(long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_15ab3(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_15b9a @ 0x15b9a
unsigned int sub_15b9a(long a0,long *a1,int a2,unsigned long *a3,char *a4)
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoimax"); // no-return
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
        v7 = sub_15ab3(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v4 | 2;
      case 0x45:
        v7 = sub_15b55(&v3,v5,6);
        break;
      case 0x47:
      case 0x67:
        v7 = sub_15b55(&v3,v5,3);
        break;
      case 0x4b:
      case 0x6b:
        v7 = sub_15b55(&v3,v5,1);
        break;
      case 0x4d:
      case 0x6d:
        v7 = sub_15b55(&v3,v5,2);
        break;
      case 0x50:
        v7 = sub_15b55(&v3,v5,5);
        break;
      case 0x54:
      case 0x74:
        v7 = sub_15b55(&v3,v5,4);
        break;
      case 0x59:
        v7 = sub_15b55(&v3,v5,8);
        break;
      case 0x5a:
        v7 = sub_15b55(&v3,v5,7);
        break;
      case 0x62:
        v7 = sub_15ab3(&v3,0x200);
        break;
      case 99:
        v7 = 0;
        break;
      case 0x77:
        v7 = sub_15ab3(&v3,2);
      
    }
    v4 |= v7;
    *v8 = *v8 + (long)v6;
    if (*(char *)*v8)
      v4 |= 2;
  }
  *a3 = v3;
  return v4;
}

// Function: sub_15fa4 @ 0x15fa4
void sub_15fa4(unsigned int a0,int a1,char a2,long a3,unsigned long a4,int a5)
{
  char v1; // stack - 0x12
  char *v2; // stack - 0x30
  char *v3; // stack - 0x28
  char *v4; // stack - 0x20
  char v5; // stack - 0x11
  
  v2 = "--";
  if (a0 != 4) { // branch-flip
    if (5 <= a0)
      abort(); // no-return, return-dupe
    if (a0 != 1) { // branch-flip
      if ((!a0) || (2 <= a0 - 2)) {
        abort();
      }
      v3 = "invalid suffix in %s%s argument \'%s\'";
    }
    else {
      v3 = "%s%s argument \'%s\' too large";
    }
  }
  else {
    v3 = "invalid %s%s argument \'%s\'";
  }
  if (0 <= a1) // branch-flip
    v4 = *(char **)(a3 + (long)a1 * 0x20);
  else {
    v2 = &"--"[-(long)a1];
    v5 = 0;
    v4 = &v1;
    v1 = a2;
  }
  error(a5,0,gettext(v3),v2,v4,a4);
}

// Function: sub_160c2 @ 0x160c2
void sub_160c2(unsigned int a0,unsigned int a1,char a2,long a3,unsigned long a4)
{
  sub_15fa4(a0,a1,(int)a2,a3,a4,dat_21028);
  abort(); // no-return
}

// Function: sub_1610c @ 0x1610c
bool sub_1610c(unsigned long *a0,int a1)
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

// Function: sub_1619e @ 0x1619e
unsigned int sub_1619e(unsigned long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_1610c(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_161e3 @ 0x161e3
unsigned int sub_161e3(unsigned char *a0,long *a1,int a2,unsigned long *a3,char *a4) // early-return x2
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
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
        v8 = sub_1610c(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_1619e(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_1619e(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_1619e(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_1619e(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_1619e(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_1619e(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_1619e(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_1619e(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_1610c(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_1610c(&v3,2);
      
    }
    v5 |= v8;
    *v10 = *v10 + (long)v7;
    if (*(char *)*v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_16641 @ 0x16641
void sub_16641(void)
{
  tzset();
}

// Function: sub_16651 @ 0x16651
long sub_16651(long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f);
}

// Function: sub_166c8 @ 0x166c8
unsigned int sub_166c8(unsigned long a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_166c8
{
  unsigned int v1; // eax
  
  v1 = !((a0 & 3) || ((!((long)a0 % 100) && (((unsigned int)((long)a0 / 100) & 3) != 1))));
  return v1;
}

// Function: sub_16771 @ 0x16771
unsigned int sub_16771(int a0,int a1) // return-dupe
{
  if ((((a1 == 0) != (a0 == 0)) && (0 <= a0)) && (0 <= a1))
    return 1;
  return 0;
}

// Function: sub_167b0 @ 0x167b0
long sub_167b0(unsigned long a0,long a1,int a2,int a3,int a4,unsigned int a5,int a6,int a7,int a8,int a9)
{
  int v1; // eax
  int v2; // eax
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  
  v1 = sub_16651(a0,2);
  v1 = (v1 + sub_16651(0x76c,2)) - (unsigned int)((a0 & 3) == 0);
  v2 = sub_16651((long)(int)a5,2);
  v2 = (v2 + sub_16651(0x76c,2)) - (unsigned int)((a5 & 3) == 0);
  v3 = (v1 - (v1 >> 0x1f)) / 0x19 + (v1 >> 0x1f);
  v4 = (v2 - (v2 >> 0x1f)) / 0x19 + (v2 >> 0x1f);
  v5 = sub_16651((long)v3,2);
  v6 = sub_16651((long)v4,2);
  return (((((((long)((v5 - v6) + ((v1 - v2) - (v3 - v4))) + (((a0 - (long)(int)a5) * 0x16d + a1) - (long)a6)) * 0x18 + (long)a2) - (long)a7) * 0x3c + (long)a3) - (long)a8) * 0x3c + (long)a4) - (long)a9;
}

// Function: sub_169d3 @ 0x169d3
long sub_169d3(long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_16651(a0,1);
  return (unsigned long)(((unsigned int)a0 | (unsigned int)a1) & 1) + v1 + sub_16651(a1,1);
}

// Function: sub_16a25 @ 0x16a25
void sub_16a25(unsigned long a0,long a1,unsigned int a2,unsigned int a3,unsigned int a4,unsigned int *a5)
{
  sub_167b0(a0,a1,a2,a3,a4,a5[5],a5[7],a5[2],a5[1],*a5);
}

// Function: sub_16a9e @ 0x16a9e
void sub_16a9e(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x18
  
  v1 = a1;
  (*a0)(&v1,a2);
}

// Function: sub_16af7 @ 0x16af7
unsigned long * sub_16af7(void *a0,long *a1,unsigned long *a2) // early-return x3
{
  unsigned long *v1; // rax
  unsigned long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  long v2; // rax
  long v3; // stack - 0x80
  long v4; // stack - 0x78
  unsigned long v5; // stack - 0x58
  unsigned long v6; // stack - 0x50
  unsigned long v7; // stack - 0x48
  unsigned long v8; // stack - 0x40
  unsigned long v9; // stack - 0x38
  
  if (-0x8000000000000000 <= *a1) { // branch-flip
    v3 = *a1;
    if (0x7fffffffffffffff < v3)
      v3 = 0x7fffffffffffffff;
  }
  else {
    v3 = -0x8000000000000000;
  }
  v1 = (unsigned long *)sub_16a9e(a0,v3,a2);
  if (v1) { // branch-flip
    *a1 = v3;
    a2 = v1;
  }
  else {
    if (*__errno_location() != 0x4b)
      return NULL;
    v4 = 0;
    v5 = CONCAT44(v5._4_4_,0xffffffff);
    while ((v2 = sub_169d3(v4,v3), v2 != v4 && (v2 != v3))) {
      if (sub_16a9e(a0,v2,a2)) { // branch-flip
        v5 = *a2;
        v6 = a2[1];
        v7 = a2[2];
        v8 = a2[3];
        v9 = a2[4];
        v10 = a2[5];
        v11 = a2[6];
        v4 = v2;
      }
      else {
        if (*__errno_location() != 0x4b)
          return NULL;
        v3 = v2;
      }
    }
    if ((int)v5 <= -1)
      return NULL;
    *a1 = v4;
    *a2 = v5;
    a2[1] = v6;
    a2[2] = v7;
    a2[3] = v8;
    a2[4] = v9;
    a2[5] = v10;
    a2[6] = v11;
  }
  return a2;
}

// Function: sub_16cf3 @ 0x16cf3
long sub_16cf3(int *a0,void *a1,long *a2) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_16cf3
{
  int v1;
  int v10;
  long v11; // rax
  int v12; // edx
  int v13; // stack - 0x98
  long v14; // stack - 0x100
  char v15 [32];
  long v16; // stack - 0xf8
  bool v17;
  int v18; // stack - 0x150
  int v19; // stack - 0x14c
  int v2;
  int v20; // stack - 0x144
  int v21; // stack - 0x140
  long v22; // stack - 0xf0
  long v23; // stack - 0xe8
  long v24; // stack - 0xe0
  unsigned long v25; // stack - 0xd8
  long v26; // stack - 0xd0
  long v27; // stack - 0xc8
  long v28; // stack - 0xc0
  long v29; // stack - 0xb8
  int v3;
  long v30; // stack - 0xb0
  long v31; // stack - 0xa8
  long v32; // stack - 0xa0
  unsigned int v33; // stack - 0x94
  unsigned long v34; // stack - 0x90
  unsigned long v35; // stack - 0x88
  unsigned long v36; // stack - 0x80
  int v37; // stack - 0x78
  unsigned int v38; // stack - 0x74
  unsigned long v39; // stack - 0x70
  int v4;
  unsigned long v40; // stack - 0x68
  unsigned int v41; // stack - 0x38
  int v5;
  int v6;
  int v7;
  unsigned char v8; // al
  char v9; // al
  
  v18 = 6;
  v1 = *a0;
  v10 = a0[1];
  v2 = a0[2];
  v3 = a0[3];
  v4 = a0[4];
  v5 = a0[8];
  v17 = 0;
  v6 = v4 >> 0x1f;
  v12 = v4 + ((v4 / 6 + v6 >> 1) - v6) * -0xc;
  v7 = v12 >> 0x1f;
  v24 = (long)a0[5];
  v25 = v24 + (((v4 / 6 + v6 >> 1) - v6) + v7);
  v8 = sub_166c8(v25);
  v26 = (long)v3;
  v27 = v26 + (int)((unsigned int)*(unsigned short *)(((long)(int)(unsigned int)v8 * 0xd + (long)(v12 + v7 * -0xc)) * 2 + 0x1b5e0) - 1);
  v28 = *a2;
  v19 = v1;
  if (v1 <= -1)
    v19 = 0;
  if (0x3c <= v19)
    v19 = 0x3b;
  v3 = (int)v28;
  v29 = sub_167b0(v25,v27,v2,v10,v19,0x46,0,0,0,-v3);
  v14 = v29;
  v22 = v29;
  v23 = v29;
  while( true ) {
    if (!sub_16af7(a1,&v14,&v13))
      return -1;
    v30 = sub_16a25(v25,v27,v2,v10,v19,&v13);
    if (!v30) break;
    if ((v22 == v14) && (v23 != v14)) {
      if (v37 < 0) goto label_17379;
      if (0 <= v5) { // branch-flip
        if ((v37 != 0) != (v5 != 0)) goto label_17379;
      }
      else if (v17 <= (v37 != 0)) goto label_17379;
    }
    v18 -= 1;
    if (!v18) {
      *__errno_location() = 0x4b;
      return -1;
    }
    v22 = v23;
    v23 = v14;
    v14 = v30 + v14;
    v17 = v37 != 0;
  }
  v9 = sub_16771(v5,v37);
  if (v9) {
    v17 = v37 == 0;
    for (v20 = 0x92c70; v20 < 0xda9a994; v20 = v20 + 0x92c70) {
      for (v21 = -1; v21 <= 1; v21 = v21 + 2) {
        v16 = v20 * v21 + v14;
        if (!SCARRY8((long)(v20 * v21),v14)) {
          if (!sub_16af7(a1,&v16,v15))
            return -1;
          v9 = sub_16771(v5,v41);
          if (v9 != '\x01') {
            v11 = sub_16a25(v25,v27,v2,v10,v19,v15);
            v31 = v11 + v16;
            if (-0x8000000000000000 <= v31) {
              if (sub_16a9e(a1,v31,&v13)) {
                v14 = v31;
                goto label_17379;
              }
              if (*__errno_location() != 0x4b)
                return -1;
            }
          }
        }
      }
    }
    v14 += (int)(((unsigned int)(v5 == 0) - (unsigned int)v17) * 0xe10);
    if (!sub_16a9e(a1,v14,&v13)) {
      *__errno_location() = 0x4b;
      return -1;
    }
  }
label_17379:
  *a2 = v14 - v29;
  *a2 = *a2 - (long)-v3;
  if (v1 != v13) {
    v10 = !((v19) || (v13 != 0x3c));
    v32 = ((long)v10 - (long)v19) + (long)v1;
    v17 = SCARRY8(v32,v14);
    v14 = v32 + v14;
    if ((v17) || (0x7fffffffffffffff < v14)) {
      *__errno_location() = 0x4b;
      return -1;
    }
    if (!sub_16a9e(a1,v14,&v13))
      return -1;
  }
  *(unsigned long *)a0 = CONCAT44(v33,v13);
  *(unsigned long *)&a0[2] = v34;
  *(unsigned long *)&a0[4] = v35;
  *(unsigned long *)&a0[6] = v36;
  *(unsigned long *)&a0[8] = CONCAT44(v38,v37);
  *(unsigned long *)&a0[10] = v39;
  *(unsigned long *)&a0[0xc] = v40;
  return v14;
}

// Function: sub_17511 @ 0x17511
void sub_17511(int *a0)
{
  sub_16641();
  sub_16cf3(a0,dat_20fc0,(long *)0x21328);
}

// Function: sub_17545 @ 0x17545
unsigned long sub_17545(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_17565 @ 0x17565
unsigned long sub_17565(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_17585 @ 0x17585
unsigned int sub_17585(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_175a1 @ 0x175a1
unsigned int sub_175a1(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_175bd @ 0x175bd
unsigned long sub_175bd(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_175f6 @ 0x175f6
unsigned long sub_175f6(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_1762f @ 0x1762f
unsigned int sub_1762f(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_17665 @ 0x17665
unsigned int sub_17665(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_1769b @ 0x1769b
unsigned int sub_1769b(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_176d0 @ 0x176d0
unsigned int sub_176d0(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_17705 @ 0x17705
int sub_17705(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_1849f((unsigned char)*a0);
    v2 = sub_1849f((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_1777f @ 0x1777f
unsigned long sub_1777f(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_7ea0(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_1780e @ 0x1780e
unsigned long sub_1780e(unsigned long a0)
{
  a0 = a0 - 1 & a0;
  return CONCAT71((undefined7)(a0 >> 8),a0 == 0);
}

// Function: sub_1782e @ 0x1782e
void sub_1782e(char *a0)
{
  *(unsigned long *)&a0[0x10] = 0;
  *(unsigned int *)&a0[0x18] = 0x95f616;
}

// Function: sub_17854 @ 0x17854
unsigned long sub_17854(struct_17 *a0,struct_8 *a1) // return-dupe
{
  unsigned long v1;
  
  if (a0->field_0x18 != 0x95f616)
    __assert_fail("state->magic == 9827862","lib/cycle-check.c",0x3c,"cycle_check"); // no-return
  if (((a0->field_0x10) && (a1->field_0x8 == a0->field_0x0)) && (a1->field_0x0 == a0->field_0x8))
    return 1;
  a0->field_0x10 = a0->field_0x10 + 1;
  v1 = a0->field_0x10;
  if (!sub_1780e(v1))
    return 0;
  if (a0->field_0x10) {
    a0->field_0x8 = a1->field_0x0;
    a0->field_0x0 = a1->field_0x8;
    return 0;
  }
  return 1;
}

// Function: sub_1793b @ 0x1793b
int sub_1793b(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_17cad(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_17cde(a0,(unsigned int)v2);
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

// Function: sub_17cad @ 0x17cad
int sub_17cad(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_17cde @ 0x17cde
int sub_17cde(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_21330) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_21330 = 1;
    else {
      v3 = sub_17cad(a0,a1);
      if (0 <= v3)
        dat_21330 = -1;
    }
  }
  else {
    v3 = sub_17cad(a0,a1);
  }
  if ((0 <= v3) && (dat_21330 == -1)) {
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

// Function: sub_17de1 @ 0x17de1
void sub_17de1(char *a0,unsigned int a1)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa8
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = v2;
  sub_149b4(open(a0,a1,(unsigned long)v1));
}

// Function: sub_17f1d @ 0x17f1d
unsigned long sub_17f1d(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_17f37 @ 0x17f37
void sub_17f37(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_17f55 @ 0x17f55
void sub_17f55(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_17f8c @ 0x17f8c
void sub_17f8c(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_17fb5 @ 0x17fb5
void sub_17fb5(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_18004 @ 0x18004
char * sub_18004(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_1804c @ 0x1804c
unsigned int sub_1804c(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_18087 @ 0x18087
void sub_18087(struct_40 *a0,struct_41 *a1) // return-dupe
{
  if ((struct_41 *)a1->field_0x0 != &a1[1]) // branch-flip
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

// Function: sub_1812d @ 0x1812d
bool sub_1812d(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0x1b660) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_18172 @ 0x18172
unsigned long sub_18172(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_184e5(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_1821c @ 0x1821c
void sub_1821c(unsigned int a0)
{
  sub_1793b(a0,0,3);
}

// Function: sub_18246 @ 0x18246
unsigned long sub_18246(int a0) // return-dupe
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

// Function: sub_18284 @ 0x18284
unsigned long sub_18284(int a0) // return-dupe
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

// Function: sub_182b6 @ 0x182b6
bool sub_182b6(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_182d7 @ 0x182d7
unsigned int sub_182d7(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_182ff @ 0x182ff
unsigned long sub_182ff(int a0) // return-dupe
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

// Function: sub_1832c @ 0x1832c
bool sub_1832c(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_18350 @ 0x18350
bool sub_18350(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_18374 @ 0x18374
bool sub_18374(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_18398 @ 0x18398
bool sub_18398(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_183bc @ 0x183bc
unsigned long sub_183bc(int a0) // return-dupe, return-dupe x3
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

// Function: sub_18406 @ 0x18406
unsigned long sub_18406(int a0) // return-dupe
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

// Function: sub_18433 @ 0x18433
bool sub_18433(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_18457 @ 0x18457
unsigned long sub_18457(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_1849f @ 0x1849f
int sub_1849f(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_184c2 @ 0x184c2
int sub_184c2(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_184e5 @ 0x184e5
unsigned int sub_184e5(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_1866f(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_1858f @ 0x1858f
char * sub_1858f(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_185b7 @ 0x185b7
unsigned long sub_185b7(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_1858f(a0);
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

// Function: sub_1866f @ 0x1866f
void sub_1866f(unsigned int a0,char *a1,unsigned long a2)
{
  sub_185b7(a0,a1,a2);
}

// Function: sub_1869d @ 0x1869d
void sub_1869d(unsigned int a0)
{
  sub_1858f(a0);
}

// Function: sub_186c0 @ 0x186c0
void sub_186c0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_21008); // tail-call
}

// Function: _DT_FINI @ 0x186d4
void _DT_FINI(void)
{
  return;
}

