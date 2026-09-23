// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_1cfe0)
    return;
  (*dat_1cfe0)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_1cc98)(); // jump-as-call
}

// Function: free @ 0x3650
void free(void *a0)
{
  (*dat_1cfb0)(); // jump-as-call
}

// Function: localtime_r @ 0x3660
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_1cfc8)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x3670
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_1cfd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x3680
void __cxa_finalize(void)
{
  (*dat_1cff0)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x3690
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cca0)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x36a0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cca8)(); // jump-as-call
  return v1;
}

// Function: strtoimax @ 0x36b0
void strtoimax(void)
{
  (*dat_1ccb0)(); // jump-as-call
}

// Function: endmntent @ 0x36c0
int endmntent(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ccb8)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x36d0
void abort(void)
{
  (*dat_1ccc0)(); // jump-as-call
}

// Function: __errno_location @ 0x36e0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_1ccc8)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x36f0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1ccd0)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3700
void _exit(int a0)
{
  (*dat_1ccd8)(); // jump-as-call
}

// Function: strcpy @ 0x3710
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cce0)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x3720
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cce8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x3730
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ccf0)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3740
void qsort(void)
{
  (*dat_1ccf8)(); // jump-as-call
}

// Function: iswcntrl @ 0x3750
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cd00)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3760
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cd08)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x3770
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_1cd10)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3780
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cd18)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x3790
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1cd20)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x37a0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cd28)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x37b0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cd30)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x37c0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cd38)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x37d0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cd40)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x37e0
void regfree(re_pattern_buffer *a0)
{
  (*dat_1cd48)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x37f0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cd50)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x3800
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cd58)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x3810
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_1cd60)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3820
void __stack_chk_fail(void)
{
  (*dat_1cd68)(); // jump-as-call
}

// Function: getopt_long @ 0x3830
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_1cd70)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3840
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cd78)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x3850
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1cd80)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3860
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cd88)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3870
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cd90)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3880
void lseek(void)
{
  (*dat_1cd98)(); // jump-as-call
}

// Function: __assert_fail @ 0x3890
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_1cda0)(); // jump-as-call
}

// Function: fputs @ 0x38a0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cda8)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x38b0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cdb0)(); // jump-as-call
  return v1;
}

// Function: freopen @ 0x38c0
FILE * freopen(char *a0,char *a1,FILE *a2)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_1cdb8)(); // jump-as-call
  return v1;
}

// Function: close @ 0x38d0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cdc0)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x38e0
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cdc8)(); // jump-as-call
  return v1;
}

// Function: regcomp @ 0x38f0
int regcomp(re_pattern_buffer *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1cdd0)(); // jump-as-call
  return v1;
}

// Function: fputc @ 0x3900
int fputc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cdd8)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x3910
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cde0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3920
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1cde8)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3930
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cdf0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3940
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cdf8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3950
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce00)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3960
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ce08)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x3970
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce10)(); // jump-as-call
  return v1;
}

// Function: __memcpy_chk @ 0x3980
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce18)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x3990
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce20)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x39a0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ce28)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x39b0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce30)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x39c0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce38)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x39d0
void tzset(void)
{
  (*dat_1ce40)(); // jump-as-call
}

// Function: fileno @ 0x39e0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce48)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x39f0
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_1ce50)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3a00
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce58)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x3a10
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ce60)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3a20
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce68)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3a30
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce70)(); // jump-as-call
  return v1;
}

// Function: getmntent @ 0x3a40
void * getmntent(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce78)(); // jump-as-call
  return v1;
}

// Function: setmntent @ 0x3a50
void * setmntent(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce80)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3a60
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ce88)(); // jump-as-call
  return v1;
}

// Function: __isoc99_sscanf @ 0x3a70
int __isoc99_sscanf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1ce90)(); // jump-as-call
  return v1;
}

// Function: regexec @ 0x3a80
int regexec(re_pattern_buffer *a0,char *a1,unsigned long a2,void *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_1ce98)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3a90
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cea0)(); // jump-as-call
  return v1;
}

// Function: fchdir @ 0x3aa0
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cea8)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3ab0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ceb0)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3ac0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ceb8)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x3ad0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cec0)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x3ae0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cec8)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3af0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ced0)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3b00
void error(int a0,int a1,char *a2,...)
{
  (*dat_1ced8)(); // jump-as-call
}

// Function: open @ 0x3b10
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cee0)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x3b20
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1cee8)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3b30
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_1cef0)(); // jump-as-call
  return v1;
}

// Function: towlower @ 0x3b40
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_1cef8)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x3b50
void strtoumax(void)
{
  (*dat_1cf00)(); // jump-as-call
}

// Function: fdopendir @ 0x3b60
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_1cf08)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x3b70
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf10)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x3b80
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cf18)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x3b90
void __cxa_atexit(void)
{
  (*dat_1cf20)(); // jump-as-call
}

// Function: getline @ 0x3ba0
long getline(char **a0,void *a1,FILE *a2)
{
  long v1; // rax
  
  v1 = (*dat_1cf28)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3bb0
void exit(int a0)
{
  (*dat_1cf30)(); // jump-as-call
}

// Function: fwrite @ 0x3bc0
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cf38)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x3bd0
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_1cf40)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3be0
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf48)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3bf0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf50)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3c00
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf58)(); // jump-as-call
  return v1;
}

// Function: hasmntopt @ 0x3c10
char * hasmntopt(void *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cf60)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_makedev @ 0x3c20
void gnu_dev_makedev(void)
{
  (*dat_1cf68)(); // jump-as-call
}

// Function: fstat @ 0x3c30
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cf70)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x3c40
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf78)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x3c50
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_1cf80)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x3c60
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cf88)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x3c70
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_1cf90)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x3c80
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cf98)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3c90
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cfa0)(); // jump-as-call
  return v1;
}

// Function: __sprintf_chk @ 0x3ca0
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_1cfa8)(); // jump-as-call
  return v1;
}

// Function: sub_3cb0 @ 0x3cb0
undefined16 sub_3cb0(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x3cd0
int main(int argc,char **argv,char **envp)
{
  bool v1;
  char *v10;
  unsigned long v11;
  unsigned long v12; // rax
  char *v13; // rax
  char *v14; // rax
  int v15; // stack - 0x64
  int v16; // stack - 0x60
  char *v17; // stack - 0x58
  char **v18;
  unsigned int v19; // r15d
  unsigned int v2;
  char *v20; // stack - 0x80
  unsigned int v21; // stack - 0x70
  unsigned int v22; // stack - 0x5c
  unsigned long v23; // stack - 0x50
  bool v3;
  int v4;
  unsigned int v5;
  int v6; // eax
  int v7; // eax
  struct_23 *v8; // rax
  char *v9;
  
  v19 = 1;
  v18 = argv;
  v17 = (char *)sub_5630();
  v23 = 0;
  sub_e310(*v18);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_137d0(sub_6020);
  dat_1d140 = sub_68c0();
  sub_d070(getenv("DU_BLOCK_SIZE"),(unsigned int *)0x1d170,(long *)0x1d148);
  v1 = 0;
  v2 = 0x10;
  v21 = 8;
  v20 = NULL;
  v3 = 0;
label_3dc0:
  do {
    v15 = -1;
    v18 = argv;
    v4 = getopt_long(argc,argv,"0abd:chHklmst:xB:DLPSX:",(option *)0x1c6e0,&v15);
    v9 = (char *)CONCAT44(dat_4,v4);
    if (v4 == -1) {
      if (!(bool)v19)
        sub_58b0(1); // return-dupe, no-return
      if (dat_1d186) { // branch-flip
        if (v1) {
          error(0,0,dcgettext(NULL,"cannot both summarize and show all entries",5));
          sub_58b0(1);
        }
      }
      else if (v3) {
        if (v1) {
          if (dat_1d010) {
label_43b7:
            v11 = dat_1d010;
            error(0,0,dcgettext(NULL,"warning: summarizing conflicts with --max-depth=%lu",5),v11);
            sub_58b0(1); // no-return
          }
          error(0,0,dcgettext(NULL,"warning: summarizing is the same as using --max-depth=0",5));
          v9 = v13;
          if (dat_1d010) goto label_43b7;
label_45ed:
          dat_1d010 = 0;
        }
      }
      else if (v1) goto label_45ed;
      if (dat_1d16d) {
        if (dat_1d185) {
          error(0,0,dcgettext(NULL,"warning: options --apparent-size and -b are ineffective with --inodes",5));
          v9 = v14;
        }
        dat_1d148 = 1;
      }
      if (!dat_1d16c) goto label_4226;
      if (dat_1d160) {
        v10 = dat_1d160;
        goto label_4210;
      }
      v10 = getenv("TIME_STYLE");
      v9 = NULL;
      if (v10) {
        dat_1d160 = v10;
        v4 = strcmp(v10,"locale");
        v9 = (char *)CONCAT44(dat_4,v4);
        if (v4) {
          if (*v10 != '+') goto label_477a;
          v9 = strchr(v10,10);
          if (!v9) goto label_421b;
          *v9 = '\0';
          goto label_4210;
        }
      }
      v10 = "long-iso";
      dat_1d160 = "long-iso";
      goto label_4697;
    }
    if (0x87 < v4) {
label_3e26:
      v19 = 0;
      goto label_3dc0;
    }
    if (v4 <= 0x2f) {
      if (v4 == -0x83) {
        sub_11510(stdout,"du","GNU coreutils",dat_1d018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0,v18);
        exit(0); // no-return
      }
      if (v4 == -0x82)
        sub_58b0(0); // no-return
      goto label_3e26;
    }
    switch(v4) {
      case 0x30:
        dat_1d182 = 1;
        goto label_3dc0;
      default:
        goto label_3e26;
      case 0x42:
label_3fb0:
        v4 = sub_d070(optarg,(unsigned int *)0x1d170,(long *)0x1d148);
        if (v4)
          sub_12020(v4,v15,0x42,0x1c6e0,optarg); // no-return
        break;
      case 0x44:
      case 0x48:
        v2 = 0x11;
        goto label_3dc0;
      case 0x4c:
        v2 = 2;
        goto label_3dc0;
      case 0x50:
        v2 = 0x10;
        goto label_3dc0;
      case 0x53:
        dat_1d180 = 1;
        goto label_3dc0;
      case 0x58:
        v4 = sub_6fe0(sub_6b50,dat_1d140,optarg,0x10000000,10);
        if (v4) {
          v12 = sub_10680(0,3,optarg);
          v19 = 0;
          error(0,*__errno_location(),"%s",v12);
        }
        goto label_3dc0;
      case 0x61:
        dat_1d186 = '\x01';
        goto label_3dc0;
      case 0x62:
        dat_1d185 = '\x01';
        dat_1d170 = 0;
        dat_1d148 = 1;
        goto label_3dc0;
      case 99:
        dat_1d181 = '\x01';
        goto label_3dc0;
      case 100:
        v4 = sub_120a0(optarg,NULL,0,&v16,"");
        if (v4) { // branch-flip
          v12 = sub_10840(optarg);
          v19 = 0;
          error(0,0,dcgettext(NULL,"invalid maximum depth %s",5),v12);
        }
        else {
          dat_1d010 = CONCAT44(v22,v16);
          v3 = 1;
        }
        goto label_3dc0;
      case 0x68:
        dat_1d170 = 0xb0;
        dat_1d148 = 1;
        goto label_3dc0;
      case 0x6b:
        dat_1d170 = 0;
        dat_1d148 = 0x400;
        goto label_3dc0;
      case 0x6c:
        dat_1d184 = '\x01';
        goto label_3dc0;
      case 0x6d:
        dat_1d170 = 0;
        dat_1d148 = 0x100000;
        goto label_3dc0;
      case 0x73:
        v1 = 1;
        goto label_3dc0;
      case 0x74:
        v4 = sub_11b80(optarg,NULL,0,(unsigned long *)0x1d178,"kKmMGTPEZY0");
        if (v4)
          sub_12020(v4,v15,0x74,0x1c6e0,optarg); // no-return
        if ((dat_1d178) || (*optarg != '-')) goto label_3dc0;
        error(1,0,dcgettext(NULL,"invalid --threshold argument \'-0\'",5));
        goto label_3fb0;
      case 0x78:
        v21 = 0x48;
        break;
      case 0x80:
        dat_1d185 = '\x01';
        break;
      case 0x81:
        sub_6b50(dat_1d140,optarg,0x10000000);
        break;
      case 0x82:
        v20 = optarg;
        break;
      case 0x83:
        dat_1d170 = 0x90;
        dat_1d148 = 1;
        break;
      case 0x85:
        dat_1d16c = '\x01';
        v5 = 0;
        if (optarg)
          v5 = *(unsigned int *)(sub_5dc0("--time",optarg,(long *)0x1c6a0,(void *)0x15700,4,dat_1d020,1) * 4 + 0x15700);
        dat_1d168 = v5;
        dat_1d150 = sub_109c0(getenv("TZ"));
        break;
      case 0x86:
        dat_1d160 = optarg;
        break;
      case 0x87:
        dat_1d16d = '\x01';
        goto label_3dc0;
      
    }
  } while( true );
label_477a:
  while( true ) {
    v10 = dat_1d160;
    v4 = strncmp(dat_1d160,"posix-",6);
    v9 = (char *)CONCAT44(dat_4,v4);
    if (v4) break;
    dat_1d160 = &v10[6];
  }
label_4210:
  if (*v10 != '+') { // branch-flip
label_4697:
    v4 = *(int *)(sub_5dc0("time style",v10,(long *)0x1c680,(void *)0x156f0,4,dat_1d020,1,v9) * 4 + 0x156f0);
    if (v4 != 1) { // branch-flip
      if (v4 != 2) { // branch-flip
        if (!v4)
          dat_1d158 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      else {
        dat_1d158 = "%Y-%m-%d";
      }
    }
    else {
      dat_1d158 = "%Y-%m-%d %H:%M";
    }
  }
  else {
label_421b:
    dat_1d158 = &v10[1];
  }
label_4226:
  if (v20) { // branch-flip
    if (optind < argc) {
      v9 = (char *)sub_10840(argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v9);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_58b0(1); // no-return
    }
    if ((strcmp(v20,"-")) && (!sub_8c10(v20,"r",stdin))) {
      v12 = sub_10580(4,v20);
      v9 = dcgettext(NULL,"cannot open %s for reading",5);
      error(1,*__errno_location(),v9,v12);
      return v6;
    }
    v8 = (struct_23 *)sub_5ee0(stdin);
    dat_1d183 = (bool)v19;
  }
  else {
    v18 = &v17;
    if (optind < argc)
      v18 = &argv[optind];
    v8 = (struct_23 *)sub_5eb0(v18);
    dat_1d183 = optind + 1 < argc || v2 == 2;
  }
  if ((v8) && (dat_1d198 = sub_6240(), dat_1d198)) {
    if ((dat_1d184) || (!dat_1d183))
      v21 |= 0x100;
    while (v9 = (char *)sub_5f20(v8,&v16), v9) {
      if (v20) { // branch-flip
        if ((strcmp(v20,"-")) || (strcmp(v9,"-"))) {
          if (*v9) goto label_4341;
label_4428:
          v11 = sub_5fc0(v8);
          v9 = dcgettext(NULL,"invalid zero-length file name",5);
          error(0,0,"%s:%lu: %s",(char *)sub_10680(0,3,v20),v11,v9);
        }
        else {
          v10 = (char *)sub_10580(4,v9);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v10);
          if (!*v9) goto label_4428;
        }
        v19 = 0;
      }
      else if (*v9) { // branch-flip
label_4341:
        dat_1d110 = v9;
        v19 &= sub_5540(v21 | v2);
      }
      else {
        v19 = 0;
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
      }
    }
    if (v16 != 3) {
      if (v16 != 4) { // branch-flip
        if (v16 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
      }
      else {
        v19 = 0;
        v12 = sub_10680(0,3,v20);
        v9 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v9,v12);
      }
      sub_5fe0(v8);
      sub_62b0(dat_1d198);
      if (dat_1d190)
        sub_62b0(dat_1d190);
      if ((v20) && (((ferror_unlocked(stdin) || (v4 = sub_70c0(stdin), v4)) && ((char)v19)))) {
        v9 = (char *)sub_10580(4,v20);
        error(1,0,dcgettext(NULL,"error reading %s",5),v9);
        return v7;
      }
      if (dat_1d181)
        sub_4e30((unsigned long *)0x1d120,dcgettext(NULL,"total",5));
      return v19 ^ 1;
    }
  }
  sub_11a40(); // no-return
}

// Function: sub_4950 @ 0x4950
void sub_4950(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_1cfb8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4980 @ 0x4980
void sub_4980(void)
{
  return;
}

// Function: _FINI_0 @ 0x49f0
void _FINI_0(void)
{
  if (!dat_1d0e8) {
    if (dat_1cff0)
      __cxa_finalize(dat_1d008);
    sub_4980();
    dat_1d0e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4a30
void _INIT_0(void)
{
  return;
}

// Function: sub_4a40 @ 0x4a40
void sub_4a40(unsigned long *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0x8000000000000000;
  a0[3] = 0xffffffffffffffff;
}

// Function: sub_4a70 @ 0x4a70
void sub_4a70(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  *a0 = a1;
  a0[1] = 1;
  a0[2] = a2;
  a0[3] = a3;
}

// Function: sub_4a90 @ 0x4a90
void sub_4a90(unsigned long *a0,unsigned long *a1) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  
  v4 = *a1 + *a0;
  v1 = a1[2];
  if (CARRY8(*a1,*a0))
    v4 = 0xffffffffffffffff;
  v2 = a1[3];
  *a0 = v4;
  a0[1] = a0[1] + a1[1];
  v4 = a0[3];
  v3 = a0[2];
  if (0 <= (int)sub_10ee0(v3,v4,v1,v2))
    return;
  a0[2] = v1;
  a0[3] = v2;
}

// Function: sub_4af0 @ 0x4af0
void sub_4af0(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_4b20 @ 0x4b20
void sub_4b20(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",5),v1); // tail-call
}

// Function: sub_4b50 @ 0x4b50
unsigned int sub_4b50(unsigned long a0,long a1,long a2)
{
  int v1; // eax
  
  v1 = sub_62e0(a0,a2,a1);
  if (0 <= v1)
    return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 != 0);
  sub_11a40(); // no-return
}

// Function: sub_4b80 @ 0x4b80
void sub_4b80(void) // return-dupe
{
  long v1;
  char *v2;
  long v3;
  stat v4; // stack - 0xb8
  
  v3 = sub_d8b0(0);
  if (!v3)
    return;
  while( true ) {
    if ((!(*(unsigned char *)(v3 + 0x28) & 3)) && (v2 = *(char **)(v3 + 8), !stat(v2,&v4)))
      sub_4b50(dat_1d190,v4._8_8_,v4._0_8_);
    v1 = *(long *)(v3 + 0x30);
    sub_d860(v3);
    if (!v1) break;
    v3 = v1;
  }
}

// Function: sub_4c30 @ 0x4c30
unsigned long sub_4c30(long *a0) // return-dupe
{
  long *v1;
  int v2; // eax
  
  v1 = (long *)*a0;
  if (!dat_1d190) {
    dat_1d190 = sub_6240();
    if (!dat_1d190)
      sub_11a40(); // no-return
    sub_4b80();
  }
  while( true ) {
    if (a0 == v1)
      return 0;
    v2 = sub_6330(dat_1d190,a0[0xe],a0[0xf]);
    if (1 <= v2) break;
    a0 = (long *)a0[1];
    if (!a0)
      return 0;
  }
  return 1;
}

// Function: sub_4cc0 @ 0x4cc0
void sub_4cc0(unsigned long a0)
{
  FILE *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != 0xffffffffffffffff) // branch-flip
    v2 = (char *)sub_c800(a0,v3,dat_1d170,1,dat_1d148);
  else {
    v2 = dcgettext(NULL,"Infinity",5);
  }
  fputs_unlocked(v2,v1);
}

// Function: sub_4d50 @ 0x4d50
char * sub_4d50(unsigned long a0,long a1)
{
  return sub_d2d0(a0,a1); // tail-call
}

// Function: sub_4d60 @ 0x4d60
void sub_4d60(unsigned long a0,unsigned long a1,unsigned long a2,long a3) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  unsigned long v3; // stack - 0x88
  char v4 [64];
  char v5 [24];
  unsigned long v6; // stack - 0x80
  
  v3 = a1;
  v6 = a2;
  if (sub_10ce0(a3,&v3,v4)) {
    sub_8b90(stdout,a0,v4,a3,v6 & 0xffffffff);
    return;
  }
  v1 = (char *)sub_4d50(v3,v5);
  v2 = (char *)sub_10840(v1);
  error(0,0,dcgettext(NULL,"time %s is out of range",5),v2);
  fputs_unlocked(v1,stdout);
}

// Function: sub_4e30 @ 0x4e30
void sub_4e30(unsigned long *a0,char *a1)
{
  unsigned long v1;
  
  v1 = a0[1];
  if (!dat_1d16d)
    v1 = *a0;
  sub_4cc0(v1);
  if (dat_1d16c) {
    putchar_unlocked(9);
    sub_4d60(dat_1d158,a0[2],a0[3],dat_1d150);
  }
  __printf_chk(1,"\t%s%c",a1,-(dat_1d182 == '\0') & 10);
  fflush_unlocked(stdout); // tail-call
}

// Function: sub_4ec0 @ 0x4ec0
unsigned int sub_4ec0(long a0,long a1) // return-dupe
{
  unsigned long *v1;
  char *v10;
  unsigned long *v11; // rcx
  unsigned long v12; // rdx
  unsigned long v13; // rdx
  unsigned short v14;
  unsigned long v15; // stack - 0x88
  unsigned long v16; // stack - 0x68
  long v17;
  char v18 [16];
  unsigned long v19; // stack - 0x80
  unsigned long *v2;
  unsigned long v20; // stack - 0x78
  unsigned long v21; // stack - 0x70
  unsigned long v22; // stack - 0x60
  unsigned long v23; // stack - 0x58
  unsigned long v24; // stack - 0x50
  unsigned long *v3;
  char v4;
  unsigned int v5; // eax
  unsigned long v6; // rax
  unsigned long v7;
  char *v8; // rax
  char *v9; // rax
  
  v14 = *(unsigned short *)(a1 + 0x68);
  v10 = *(char **)(a1 + 0x38);
  if (v14 != 4) { // branch-flip
    v5 = 1;
    if (v14 != 6) {
      v5 = sub_6a90(dat_1d140,v10);
      if ((char)v5) {
label_50e0:
        if (v14 != 1)
          return 1;
        sub_b190(a0,a1,4);
        if (a1 == sub_ab50(a0))
          return 1;
        __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
      }
      if (v14 == 0xb) {
        sub_b190(a0,a1,1);
        if (a1 != sub_ab50(a0))
          __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
        v14 = *(unsigned short *)(a1 + 0x68);
      }
      if (v14 == 10 || v14 == 0xd) {
        v10 = (char *)sub_10580(4,v10);
        v9 = dcgettext(NULL,"cannot access %s",5);
        error(0,*(int *)(a1 + 0x40),v9,v10);
        return v5;
      }
      if ((((*(unsigned char *)(a0 + 0x48) & 0x40) && (1 <= *(long *)(a1 + 0x58))) && (*(long *)(a0 + 0x18) != *(long *)(a1 + 0x70))) || ((!dat_1d184 && (((dat_1d183 || (((*(unsigned int *)(a1 + 0x88) & 0xf000) != 0x4000 && (2 <= *(unsigned long *)(a1 + 0x80))))) && (v4 = sub_4b50(dat_1d198,*(long *)(a1 + 0x78),*(long *)(a1 + 0x70)), !v4)))))) goto label_50e0;
      if (v14 == 2) {
        if (!sub_11ad0(a0,a1))
          return 1;
        v5 = sub_4c30(a1);
        if ((char)v5)
          return 1;
        v10 = (char *)sub_10680(0,3,v10);
        error(0,0,dcgettext(NULL,"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n",5),v10);
        return v5;
      }
      if (v14 != 7) { // branch-flip
        v5 = 1;
        if (v14 == 1)
          return 1;
      }
      else {
        v5 = 0;
        v9 = (char *)sub_10680(0,3,v10);
        error(0,*(int *)(a1 + 0x40),"%s",v9);
      }
    }
    if (dat_1d168) goto label_4f13;
label_5118:
    v18 = sub_108b0(a1 + 0x70);
    v12 = SUB168(v18,8);
    v6 = SUB168(v18,0);
    if (dat_1d185) goto label_4f2e;
label_512a:
    v17 = *(long *)(a1 + 0xb0) << 9;
  }
  else {
    v9 = (char *)sub_10580(4,v10);
    v8 = dcgettext(NULL,"cannot read directory %s",5);
    v5 = 0;
    error(0,*(int *)(a1 + 0x40),v8,v9);
    if (!dat_1d168) goto label_5118;
label_4f13:
    if (dat_1d168 != 2) { // branch-flip
      v18 = sub_108a0(a1 + 0x70);
      v12 = SUB168(v18,8);
      v6 = SUB168(v18,0);
    }
    else {
      v18 = sub_10890(a1 + 0x70);
      v12 = SUB168(v18,8);
      v6 = SUB168(v18,0);
    }
    if (!dat_1d185) goto label_512a;
label_4f2e:
    v17 = *(long *)(a1 + 0xa0);
    if (*(long *)(a1 + 0xa0) <= -1)
      v17 = 0;
  }
  sub_4a70(&v15,v17,v6,v12);
  v17 = dat_1d100;
  v7 = *(unsigned long *)(a1 + 0x58);
  v16 = v15;
  v22 = v19;
  v23 = v20;
  v24 = v21;
  if (dat_1d108) { // branch-flip
    v4 = dat_1d180;
    if (dat_1d188 != v7) {
      if (v7 <= dat_1d188) { // branch-flip
        if (dat_1d188 - 1 != v7)
          __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
        v2 = (unsigned long *)(dat_1d100 + dat_1d188 * 0x40);
        sub_4a90(&v16,v2);
        v4 = dat_1d180;
        if (!dat_1d180)
          sub_4a90(&v16,&v2[4]);
        v3 = (unsigned long *)(v17 + 0x20 + v7 * 0x40);
        sub_4a90(v3,v2);
        sub_4a90(v3,&v2[4]);
      }
      else {
        if (dat_1d108 <= v7) {
          dat_1d100 = sub_11770(dat_1d100,v7,0x80);
          dat_1d108 = v7 * 2;
        }
        v4 = dat_1d180;
        v13 = dat_1d188 + 1;
        if (v13 <= v7) {
          v11 = (unsigned long *)(v13 * 0x40 + dat_1d100);
          do {
            v13 += 1;
            sub_4a40(v11);
            v1 = &v11[4];
            v11 = &v11[8];
            sub_4a40(v1);
          } while (v13 <= v7);
        }
      }
    }
  }
  else {
    dat_1d108 = v7 + 10;
    dat_1d100 = sub_11920(dat_1d108,0x40);
    v4 = dat_1d180;
  }
  dat_1d188 = v7;
  if (v4) { // branch-flip
    if ((v14 & 0xfffd) == 4) {
      sub_4a90((unsigned long *)0x1d120,&v15);
      goto label_502a;
    }
    sub_4a90((unsigned long *)(v7 * 0x40 + dat_1d100),&v15);
    sub_4a90((unsigned long *)0x1d120,&v15);
label_5021:
    if (dat_1d186) goto label_502a;
  }
  else {
    sub_4a90((unsigned long *)(v7 * 0x40 + dat_1d100),&v15);
    sub_4a90((unsigned long *)0x1d120,&v15);
    if ((v14 & 0xfffd) != 4) goto label_5021;
label_502a:
    if (v7 <= dat_1d010) goto label_503c;
  }
  if (v7)
    return v5;
label_503c:
  v7 = v22;
  if (!dat_1d16d)
    v7 = v16;
  if (0 <= (long)dat_1d178) { // branch-flip
    if (v7 < dat_1d178)
      return v5;
  }
  else if (-dat_1d178 < v7)
    return v5;
  sub_4e30(&v16,v10);
  return v5;
}

// Function: sub_5540 @ 0x5540
unsigned int sub_5540(unsigned int a0)
{
  long v1; // rax
  long v2; // rax
  int *v3; // rax
  char *v4; // rax
  char *v5; // rax
  unsigned int v6; // r12d
  
  v6 = 1;
  if (dat_1d110) {
    v1 = sub_11a80((long *)0x1d110,a0,0);
    while( true ) {
      v2 = sub_ab50(v1);
      if (!v2) break;
      v6 &= sub_4ec0(v1,v2);
    }
    v3 = __errno_location();
    if (*v3) {
      v4 = (char *)sub_10680(0,3,*(char **)(v1 + 0x20));
      v5 = dcgettext(NULL,"fts_read failed: %s",5);
      v6 = 0;
      error(0,*v3,v5,v4);
    }
    dat_1d188 = 0;
    if (sub_a960(v1)) {
      v4 = dcgettext(NULL,"fts_close failed",5);
      error(0,*v3,v4);
      return 0;
    }
  }
  return v6;
}

// Function: sub_5630 @ 0x5630
char * sub_5630(void)
{
  return ".";
}

// Function: sub_5640 @ 0x5640
void sub_5640(void)
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
    if (!strcmp("du",v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!v6)
    v6 = "du";
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "du";
  if (!strcmp("du","["))
    v3 = "test";
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == "du")
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_5870 @ 0x5870
void sub_5870(void)
{
  __printf_chk(1,dcgettext(NULL,"\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n",5),"DU"); // tail-call
}

// Function: sub_58b0 @ 0x58b0
void sub_58b0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_1d1b0;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Summarize device usage of the set of FILEs, recursively for directories.\n",5),v1);
    sub_4af0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_5870();
    sub_4b20();
    sub_5640();
  }
  exit(a0); // no-return
}

// Function: sub_5ab0 @ 0x5ab0
void sub_5ab0(void)
{
  sub_58b0(1); // tail-call, no-return
}

// Function: sub_5ac0 @ 0x5ac0
unsigned long sub_5ac0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_5bd0 @ 0x5bd0
long sub_5bd0(char *a0,long *a1) // return-dupe
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

// Function: sub_5c30 @ 0x5c30
void sub_5c30(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_10830(1,a0);
  error(0,0,v1,sub_104b0(0,8,a1),v2); // tail-call
}

// Function: sub_5cc0 @ 0x5cc0
void sub_5cc0(unsigned long *a0,void *a1,unsigned long a2)
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
      __fprintf_chk(stderr,1,", %s",(char *)sub_10840(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_10840(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = (char *)a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_5dc0 @ 0x5dc0
long sub_5dc0(char *a0,char *a1,long *a2,void *a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_5ac0(a1,a2,a3,a4);
  else {
    v1 = sub_5bd0(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_5c30(a0,a1,v1);
  sub_5cc0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_5e50 @ 0x5e50
long sub_5e50(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_5eb0 @ 0x5eb0
void sub_5eb0(char **a0) // return-dupe
{
  struct_37 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
}

// Function: sub_5ee0 @ 0x5ee0
void sub_5ee0(unsigned long a0) // return-dupe
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

// Function: sub_5f20 @ 0x5f20
long sub_5f20(struct_23 *a0,int *a1) // early-return x2
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

// Function: sub_5fc0 @ 0x5fc0
long sub_5fc0(struct_24 *a0)
{
  if (a0->field_0x0)
    return a0->field_0x8;
  return a0->field_0x28 - a0->field_0x20 >> 3;
}

// Function: sub_5fe0 @ 0x5fe0
void sub_5fe0(struct_25 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_6000 @ 0x6000
void sub_6000(unsigned long a0)
{
  dat_1d1a8 = a0;
}

// Function: sub_6010 @ 0x6010
void sub_6010(char a0)
{
  dat_1d1a0 = a0;
}

// Function: sub_6020 @ 0x6020
void sub_6020(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_13040(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_1d1a0) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_1d1a8) { // branch-flip
        v4 = (char *)sub_10660(dat_1d1a8);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_1d028); // return-dupe, no-return
    }
  }
  v1 = sub_13040(stderr);
  if (!v1)
    return;
  _exit(dat_1d028);
}

// Function: sub_60d0 @ 0x60d0
unsigned long sub_60d0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_60e0 @ 0x60e0
unsigned long sub_60e0(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_60f0 @ 0x60f0
unsigned long sub_60f0(unsigned long a0,unsigned long a1)
{
  return a0 % a1;
}

// Function: sub_6100 @ 0x6100
void sub_6100(void *a0)
{
  sub_be30(*(struct_11 **)((long)a0 + 8));
  free(a0); // tail-call
}

// Function: sub_6120 @ 0x6120
long sub_6120(struct_8 *a0,long a1) // early-return
{
  struct_9 *v1; // rax
  struct_9 *v2;
  long v3;
  
  v2 = (struct_9 *)a0->field_0x10;
  if (v2) { // branch-flip
    if (*(long *)v2->field_0x0 == a1)
      return v2->field_0x8;
  }
  else {
    v2 = malloc(0x10);
    a0->field_0x10 = (long *)v2;
    if (!v2)
      return 0;
  }
  *(long *)v2->field_0x0 = a1;
  v1 = (struct_9 *)sub_c290(a0->field_0x0,v2);
  if (!v1)
    return 0;
  if (v2 == v1) {
    a0->field_0x10 = NULL;
    v3 = sub_bc70(0x3fd,0,sub_60f0,NULL,0);
    v2->field_0x8 = v3;
    return v3;
  }
  v3 = v1->field_0x8;
  v2->field_0x8 = v3;
  return v3;
}

// Function: sub_61d0 @ 0x61d0
long sub_61d0(long a0,long a1)
{
  long *v1;
  
  if ((unsigned long)(a1 - 1U) <= 0x7ffffffffffffffd)
    return a1;
  v1 = *(long **)(a0 + 8);
  if (!v1) {
    v1 = (long *)sub_d190(0x7fffffffffffffff);
    *(long **)(a0 + 8) = v1;
    if (!v1)
      return -1;
  }
  return sub_d230(v1,a1); // tail-call
}

// Function: sub_6240 @ 0x6240
long * sub_6240(void)
{
  long *v1; // rax
  long v2; // rax
  long *v3;
  
  v1 = malloc(0x18);
  v3 = v1;
  if (v1) {
    v2 = sub_bc70(0xb,0,sub_60d0,sub_60e0,sub_6100);
    *v1 = v2;
    if (v2) { // branch-flip
      v1[1] = 0;
      v1[2] = 0;
    }
    else {
      v3 = NULL;
      free(v1);
    }
  }
  return v3;
}

// Function: sub_62b0 @ 0x62b0
void sub_62b0(unsigned long *a0)
{
  sub_be30((struct_11 *)*a0);
  if ((struct_26 *)a0[1])
    sub_d200((struct_26 *)a0[1]);
  free((void *)a0[2]);
  free(a0); // tail-call
}

// Function: sub_62e0 @ 0x62e0
unsigned long sub_62e0(unsigned long a0,long a1,long a2)
{
  long v1; // rax
  long v2; // rax
  
  v1 = sub_6120(a0,a1);
  if (v1) {
    v2 = sub_61d0(a0,a2);
    if (v2 != -1)
      return sub_c0a0(v1,v2,NULL); // tail-call
  }
  return 0xffffffff;
}

// Function: sub_6330 @ 0x6330
unsigned long sub_6330(unsigned long a0,long a1,long a2)
{
  long v1; // rax
  long v2; // rax
  
  v1 = sub_6120(a0,a1);
  if (v1) {
    v2 = sub_61d0(a0,a2);
    if (v2 != -1)
      return (unsigned long)(sub_ba10(v1,v2) != 0);
  }
  return 0xffffffff;
}

// Function: sub_6390 @ 0x6390
void sub_6390(char *a0)
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

// Function: sub_63d0 @ 0x63d0
void sub_63d0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  (*(void *)*a3)(); // jump-as-call
}

// Function: sub_63e0 @ 0x63e0
void sub_63e0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_63f0 @ 0x63f0
void sub_63f0(void *a0)
{
  unsigned int *v1;
  long v2; // rbx
  void *v3;
  
  if (!*(int *)((long)a0 + 8)) {
    sub_be30(*(unsigned long *)((long)a0 + 0x10));
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
          regfree((re_pattern_buffer *)&v1[2]);
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

// Function: sub_6480 @ 0x6480
unsigned int sub_6480(unsigned char *a0,unsigned char *a1)
{
  unsigned int v1; // eax
  
  v1 = sub_d370(a0,a1);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_64a0 @ 0x64a0
unsigned int sub_64a0(char *a0,char *a1)
{
  unsigned int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_64c0 @ 0x64c0
unsigned long sub_64c0(unsigned char *a0,unsigned char *a1,unsigned int a2) // early-return
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned char *v5; // rax
  char *v6; // rax
  unsigned char *v7;
  
  if (!(a2 & 8)) {
    if (!(a2 & 0x10))
      return strcmp((char *)a0,(char *)a1); // tail-call
    return sub_d370(a0,a1); // tail-call
  }
  if (!(a2 & 0x10)) {
    v3 = strlen((char *)a0);
    v1 = strncmp((char *)a0,(char *)a1,v3);
    v4 = CONCAT44(dat_4,v1);
    if (!v1) {
      if ((int)(char)a1[v3] == 0x2f)
        return 0;
      v4 = (unsigned long)(unsigned int)(int)(char)a1[v3];
    }
    return v4;
  }
  v5 = (unsigned char *)sub_11a20(a1);
  v7 = v5;
  while (v6 = strchr((char *)v7,0x2f), v6) {
    *v6 = '\0';
    v2 = sub_d370(a0,v5);
    if ((int)v2 <= 0) {
      free(v5); // return-dupe
      return (unsigned long)v2;
    }
    *v6 = '/';
    v7 = (unsigned char *)&v6[1];
  }
  v2 = sub_d370(a0,v5);
  free(v5);
  return (unsigned long)v2;
}

// Function: sub_65b0 @ 0x65b0
void sub_65b0(unsigned long *a0,int a1,unsigned int a2)
{
  unsigned long v1;
  struct_27 *v2; // rax
  void *v3; // rcx
  void *v4; // rdx
  bool v5; // zf
  
  v2 = (struct_27 *)sub_11940(0x28);
  v2->field_0x8 = a1;
  v2->field_0xc = a2;
  if (!a1) {
    v5 = (a2 & 0x10) == 0;
    v3 = sub_6480;
    if (v5)
      v3 = sub_64a0;
    v4 = sub_6640;
    if (v5)
      v4 = sub_6630;
    v2->field_0x10 = sub_bc70(0,0,v4,v3,sub_63e0);
  }
  v1 = *a0;
  *a0 = v2;
  v2->field_0x0 = v1;
}

// Function: sub_6630 @ 0x6630
unsigned long sub_6630(unsigned char *a0,unsigned long a1)
{
  return sub_bc00(a0,a1); // tail-call
}

// Function: sub_6640 @ 0x6640
unsigned long sub_6640(char *a0,unsigned long a1)
{
  int v1; // eax
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
    sub_d5c0(v2);
    if ((v8) && (!v9)) break;
    if (v8) // branch-flip
      v1 = towlower(v9);
    else {
      v1 = (unsigned int)*v6;
    }
    v6 = &v6[v7];
    v3 = (long)v1 + v3 * 0x1f;
  }
  return v3 % a1;
}

// Function: sub_6710 @ 0x6710
char * sub_6710(unsigned int a0,long a1,char *a2,char *a3)
{
  char *v1; // rax
  
  while( true ) {
    strcpy(a3,a2);
    v1 = (long)sub_ba10(a1,a3);
    while( true ) {
      if (v1) {
        v1 = (char *)0x1;
        return v1;
      }
      if ((!(a0 & 8)) || (v1 = strrchr(a3,0x2f), !v1)) break;
      *v1 = '\0';
      v1 = (long)sub_ba10(a1,a3);
    }
    if ((a0 & 0x40000000) || (v1 = strchr(a2,0x2f), !v1)) break;
    a2 = &v1[1];
  }
  v1 = NULL;
  return v1;
}

// Function: sub_67c0 @ 0x67c0
void sub_67c0(long a0,unsigned long a1)
{
  unsigned long v1;
  struct_3 *v2; // rax
  
  v2 = (struct_3 *)sub_11680(0x10);
  v1 = *(unsigned long *)(a0 + 8);
  v2->field_0x8 = a1;
  v2->field_0x0 = v1;
  *(struct_3 **)(a0 + 8) = v2;
}

// Function: sub_67f0 @ 0x67f0
unsigned long sub_67f0(char *a0,unsigned int a1) // return-dupe x2
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
          if (!(v1 + 0x85U & 0xfd)) goto label_6880;
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
label_6880:
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

// Function: sub_68c0 @ 0x68c0
void sub_68c0(void)
{
  sub_11940(0x10); // tail-call
}

// Function: sub_68d0 @ 0x68d0
void sub_68d0(struct_29 *a0)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = a0->field_0x0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_63f0(v2);
    v2 = v1;
  }
  v2 = a0->field_0x8;
  while (v2) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[1]);
    free(v2);
    v2 = v1;
  }
  free(a0); // tail-call
}

// Function: sub_6930 @ 0x6930
unsigned long sub_6930(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  char v2;
  char v3;
  char *v4;
  unsigned long v5; // r8
  char *v6;
  void *v7;
  
  v7 = sub_64c0;
  if (a2 & 0x10000000)
    v7 = dat_1cfd8;
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

// Function: sub_69f0 @ 0x69f0
unsigned int sub_69f0(struct_13 *a0,char *a1)
{
  unsigned long v1;
  unsigned int v2; // eax
  
  if (!(a0->field_0x0 & 0x8000000)) {
    v1 = a0->field_0x8;
    return sub_6930(v1,a1); // tail-call
  }
  v2 = regexec((re_pattern_buffer *)&a0->field_0x8,a1,0,NULL,0);
  return CONCAT31((undefined3)(v2 >> 8),v2 == 0);
}

// Function: sub_6a30 @ 0x6a30
unsigned long sub_6a30(long a0,long a1,char *a2)
{
  unsigned long v1; // rax
  long v2; // r13
  
  if (a1 <= 0)
    return 0;
  v2 = 0;
  do {
    v1 = sub_69f0(a0,a2);
    if ((char)v1)
      return v1;
    v2 += 1;
    a0 += 0x48;
  } while (v2 != a1);
  return 0;
}

// Function: sub_6a90 @ 0x6a90
unsigned int sub_6a90(long *a0,char *a1)
{
  unsigned int v1;
  long v2;
  long v3;
  char *v4; // rax
  long *v5;
  unsigned int v6; // r12d
  
  if (!(long *)*a0)
    return 0;
  v4 = NULL;
  v5 = (long *)*a0;
  do {
    if (!(int)v5[1]) {
      if (!v4)
        v4 = (char *)sub_11680(strlen(a1) + 1);
      v2 = v5[2];
      v1 = *(unsigned int *)((long)v5 + 0xc);
      if (!sub_6710(v1,v2,a1,v4)) goto label_6acb;
      v6 = 0; // return-dupe
      free(v4); // return-dupe
      return (*(unsigned int *)((long)v5 + 0xc) >> 0x1d ^ 1) & 1 ^ v6;
    }
    v2 = v5[4];
    v3 = v5[2];
    if (sub_6a30(v3,v2,a1)) {
      v6 = 0;
      free(v4);
      return (*(unsigned int *)((long)v5 + 0xc) >> 0x1d ^ 1) & 1 ^ v6;
    }
label_6acb:
    if (!(long *)*v5) {
      v6 = 1;
      free(v4);
      return (*(unsigned int *)((long)v5 + 0xc) >> 0x1d ^ 1) & 1 ^ v6;
    }
    v5 = (long *)*v5;
  } while( true );
}

// Function: sub_6b50 @ 0x6b50
void sub_6b50(long *a0,char *a1,unsigned int a2) // return-dupe x2
{
  unsigned int *v1;
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  void *v5;
  unsigned long v6;
  long v7;
  unsigned int v8; // r14d
  long v9;
  
  v7 = *a0;
  if ((!(a2 & 0x18000000)) || (!sub_67f0(a1,a2))) {
    if (((!v7) || (*(int *)(v7 + 8))) || ((*(unsigned int *)(v7 + 0xc) ^ a2) & 0x60000018)) {
      sub_65b0(a0,0,a2);
      v7 = *a0;
    }
    v4 = (char *)sub_11a20(a1);
    if ((a2 & 0x10000002) == 0x10000000)
      sub_6390(v4);
    v7 = *(long *)(v7 + 0x10);
    if (v4 == (char *)sub_c290(v7,v4))
      return;
    free(v4); // tail-call
    return;
  }
  if ((!v7) || ((*(int *)(v7 + 8) != 1 || ((*(unsigned int *)(v7 + 0xc) ^ a2) & 0x20000000)))) {
    sub_65b0(a0,1,a2);
    v7 = *a0;
  }
  v9 = *(long *)(v7 + 0x20);
  v5 = *(void **)(v7 + 0x10);
  if (v9 == *(long *)(v7 + 0x18)) {
    v5 = (void *)sub_11840(v5,(long *)(v7 + 0x18),1,-1,0x48);
    v9 = *(long *)(v7 + 0x20);
    *(void **)(v7 + 0x10) = v5;
  }
  *(long *)(v7 + 0x20) = v9 + 1;
  v1 = (unsigned int *)((long)v5 + v9 * 0x48);
  *v1 = a2;
  if (!(a2 & 0x8000000)) {
    if (a2 & 0x4000000) {
      a1 = (char *)sub_11a20(a1);
      sub_67c0(a0,a1);
    }
    *(char **)&v1[2] = a1;
    return;
  }
  v8 = (int)a2 >> 3 & 2U | 9;
  if (a2 & 8) { // branch-flip
    v3 = strlen(a1);
    if (0 < (long)v3) {
      do {
        v6 = v3;
        if (a1[v3 - 1] != '/') goto label_6c31;
        v3 -= 1;
      } while (v3);
      goto label_6c91;
    }
    v6 = v3;
    if (!v3) goto label_6c91;
label_6c31:
    v4 = (char *)sub_116a0(v6 + 7);
    memcpy(v4,a1,v3);
    builtin_strncpy(&v4[v3],"(/.*)?",7);
    v2 = regcomp((re_pattern_buffer *)&v1[2],v4,v8);
    free(v4);
  }
  else {
    v2 = regcomp((re_pattern_buffer *)&v1[2],a1,v8);
  }
  if (!v2)
    return;
  v9 = *(long *)(v7 + 0x20) + -1;
label_6c91:
  *(long *)(v7 + 0x20) = v9;
  return;
}

// Function: sub_6df0 @ 0x6df0
int sub_6df0(void *a0,long a1,FILE *a2,unsigned int a3,unsigned char a4,unsigned long a5)
{
  int v1; // eax
  int *v10; // stack - 0x58
  int v11; // stack - 0x50
  void *v2; // rax
  unsigned char *v3;
  long v4; // stack - 0x48
  unsigned char *v5; // rbp
  unsigned char *v6; // r14
  long v7;
  unsigned char *v8;
  int v9; // stack - 0x5c
  
  v7 = 0;
  v2 = NULL;
  v4 = 0;
  while( true ) {
    v1 = getc(a2);
    if (v1 == -1) break;
    if (v4 == v7)
      v2 = (void *)sub_11840(v2,&v4,1,-1,1);
    *(char *)((long)v2 + v7) = (char)v1;
    v7 += 1;
  }
  v9 = ferror(a2);
  v10 = __errno_location();
  v11 = 0;
  if (v9) {
    v9 = *v10;
    v11 = -(unsigned int)(v9 != 0);
  }
  v3 = (unsigned char *)sub_11710(v2,v7 + 1);
  v3[v7] = a4;
  if (v7) { // branch-flip
    v5 = &v3[(unsigned long)(v3[v7 + -1] != a4) + v7];
    sub_67c0(a1,v3);
    if (v3 < v5) {
      v8 = v3;
      do {
        v6 = &v8[1];
        if (*v8 == a4) {
          v7 = *(long *)__ctype_b_loc();
          if (*(unsigned short *)(v7 + (unsigned long)a4 * 2) & 0x2000) { // branch-flip
            for (; v3 != v8; v8 = &v8[-1]) {
              if (!(*(unsigned char *)(v7 + 1 + (unsigned long)v8[-1] * 2) & 0x20)) goto label_6f8d;
            }
          }
          else {
label_6f8d:
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
    sub_67c0(a1,v3);
  }
  *v10 = v9;
  return v11;
}

// Function: sub_6fe0 @ 0x6fe0
unsigned int sub_6fe0(unsigned long a0,long a1,char *a2,unsigned int a3,char a4) // return-dupe
{
  int v1;
  unsigned int v2; // eax
  FILE *v3; // rax
  int *v4; // rax
  unsigned long v5 [2]; // stack - 0x30
  
  v5[0] = a0;
  if (!strcmp(a2,"-")) {
    v2 = sub_6df0(sub_63d0,a1,stdin,a3,(int)a4,v5);
    return v2;
  }
  v3 = fopen(a2,"re");
  if (!v3)
    return 0xffffffff;
  v2 = sub_6df0(sub_63d0,a1,v3,a3,(int)a4,v5);
  v4 = __errno_location();
  v1 = *v4;
  if (sub_70c0(v3))
    return 0xffffffff;
  *v4 = v1;
  return v2;
}

// Function: sub_70c0 @ 0x70c0
unsigned long sub_70c0(FILE *a0) // early-return
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
    if (sub_7170(a0)) {
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

// Function: sub_7150 @ 0x7150
void sub_7150(FILE *a0)
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_8e00(a0,0,1); // tail-call
}

// Function: sub_7170 @ 0x7170
void sub_7170(FILE *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_7150(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_71b0 @ 0x71b0
int sub_71b0(int a0,int a1)
{
  int v1; // edx
  
  v1 = (a0 - a1) + 0x17e;
  return (a0 - v1) + 3 + (v1 / 7) * 7;
}

// Function: sub_71f0 @ 0x71f0
void sub_71f0(FILE *a0,unsigned char *a1,long a2)
{
  unsigned char v1;
  long *v2; // rax
  unsigned char *v3; // r12
  
  if (a2) {
    v3 = &a1[a2];
    v2 = __ctype_tolower_loc();
    do {
      v1 = *a1;
      a1 = &a1[1];
      fputc(*(int *)(*v2 + (unsigned long)v1 * 4),a0);
    } while (a1 != v3);
    return;
  }
}

// Function: sub_7250 @ 0x7250
void sub_7250(FILE *a0,unsigned char *a1,long a2)
{
  unsigned char v1;
  long *v2; // rax
  unsigned char *v3; // r12
  
  if (a2) {
    v3 = &a1[a2];
    v2 = __ctype_toupper_loc();
    do {
      v1 = *a1;
      a1 = &a1[1];
      fputc(*(int *)(*v2 + (unsigned long)v1 * 4),a0);
    } while (a1 != v3);
    return;
  }
}

// Function: sub_72b0 @ 0x72b0
unsigned long sub_72b0(FILE *a0,char *a1,tm *a2,char a3,unsigned int a4,int a5,long a6,int a7)
{
  int v1;
  unsigned long v10;
  char *v11;
  unsigned char v12;
  int v13; // edx
  unsigned int v14;
  int v15;
  char v16;
  int v17;
  char *v18;
  unsigned long v19;
  bool v2;
  char v20; // stack - 0x44b
  unsigned short v21; // stack - 0x44d
  char v22 [23];
  long v23;
  unsigned long v24; // stack - 0x488
  char v25 [1010];
  int v26;
  unsigned int v27;
  int v28;
  char v29;
  long v3;
  char *v30;
  int v31;
  unsigned long v32;
  long v33;
  long v34;
  unsigned long v35;
  bool v36;
  int *v37; // stack - 0x4f0
  unsigned int v38; // stack - 0x4e8
  unsigned long v39; // stack - 0x4c0
  int *v4;
  unsigned long v40; // stack - 0x4b8
  char *v41; // stack - 0x4a8
  unsigned long v42; // stack - 0x480
  unsigned long v43; // stack - 0x478
  long v44; // stack - 0x470
  unsigned long v45; // stack - 0x468
  unsigned long v46; // stack - 0x460
  unsigned long v47; // stack - 0x458
  long v5;
  char v6;
  bool v7;
  int v8; // eax
  unsigned char *v9;
  
  v5 = a6;
  v32 = (unsigned long)a5;
  v37 = __errno_location();
  v28 = *(unsigned int *)&a2->field_0x8;
  v1 = *v37;
  v9 = (unsigned char *)0x160e9;
  if (*(unsigned char **)&a2->field_0x30)
    v9 = *(unsigned char **)&a2->field_0x30;
  if (0xd <= v28) // branch-flip
    v38 = v28 - 0xc;
  else {
    v38 = 0xc;
    if (v28)
      v38 = v28;
  }
  v6 = *a1;
  v35 = 0;
  v4 = v37;
  v33 = a6;
  v26 = a7;
label_7342:
  if (!v6) {
    *v4 = v1;
    return v35;
  }
  v30 = a1;
  a6 = v33;
  a7 = v26;
  if (v6 != '%') {
    v19 = 0;
    if (0 <= (int)v32)
      v19 = v32;
    v10 = 1;
    if (v19)
      v10 = v19;
    if (~v35 <= v10) goto label_73c0;
    if (a0) {
      if (1 < (int)v32) {
        v32 = 0;
        do {
          v32 += 1;
          fputc(0x20,a0);
        } while (v32 < v19 - 1);
      }
      fputc((int)*a1,a0);
    }
    v35 += v10;
    goto label_738d;
  }
  v14 = 0;
  v28 = 0;
  v6 = a3;
  while( true ) {
    v11 = &v30[1];
    v27 = (unsigned int)*v11;
    v30 = &v30[1];
    v12 = *v11 - 0x23;
    v19 = (unsigned long)v27;
    if (0x3d <= v12) break;
    v10 = 1L << (v12 & 0x3f);
    if (v10 & 0x1000000000002500) // branch-flip
      v28 = v27;
    else if (v12 != 0x3b) { // branch-flip
      if (!(v10 & 1)) break;
      v14 = (unsigned int)v10 & 1;
    }
    else {
      v6 = '\x01';
    }
  }
  v10 = (unsigned long)v27;
  if (v27 - 0x30 <= 9) {
    v32 = 0;
    do {
      v34 = (long)(int)v32 * 10;
      v31 = (int)v34;
      if ((v31 != v34) || (v32 = (unsigned long)(unsigned int)(v31 + *v30 + -0x30), SCARRY4(v31,*v30 + -0x30)))
        v32 = 0x7fffffff;
      v11 = &v30[1];
      v19 = (unsigned long)(unsigned int)(int)*v11;
      v30 = &v30[1];
      v10 = v19;
    } while ((unsigned int)((int)*v11 - 0x30U) <= 9);
  }
  if (((char)v19 != 'E') && ((char)v19 != 'O'))
    v10 = 0;
  else {
    v19 = (unsigned long)(unsigned char)v30[1];
    v30 = &v30[1];
  }
  v29 = (char)v19;
  v31 = (int)v32;
  v15 = (int)v10;
  v16 = (char)v14;
  v14 = v38;
  v27 = a4;
  switch(v19 & 0xff) {
    case 0:
      v30 = &v30[-1];
      break;
    case 0x25:
      if (&v30[-1] == a1) {
        if ((v28 != 0x2d) && (0 <= v31)) {
          v19 = (unsigned long)v31;
          v32 = 1;
          if (v19)
            v32 = v19;
          if (~v35 <= v32) goto label_73c0;
          if (a0) {
            if (2 <= v31) {
              if ((v28 != 0x30) && (v33 = 0, v28 != 0x2b)) {
                do {
                  v33 += 1;
                  fputc(0x20,a0);
                } while (v19 - 1 != v33);
              }
              else {
                v33 = 0;
                do {
                  v33 += 1;
                  fputc(0x30,a0);
                } while (v19 - 1 != v33);
              }
            }
label_881f:
            fputc((int)*v30,a0);
          }
        }
        else {
          if (0xfffffffffffffffe <= v35) goto label_73c0;
          v32 = 1;
          if (a0) goto label_881f;
        }
        goto label_8401;
      }
      v30 = &v30[-1];
      break;
    case 0x3a:
      v11 = &v30[1];
      v32 = 1;
      v16 = v30[1];
      while (v16 == ':') {
        v32 += 1;
        v11 = &v30[v32];
        v16 = v30[v32];
      }
      if (v16 == 'z') goto label_82c7;
      break;
    case 0x41:
    case 0x61:
      if (v15) break;
      v41._0_1_ = '\0';
      if (v16)
        v6 = v16;
      v11 = &v20;
      goto label_7657;
    case 0x42:
      if (v15 != 0x45) {
        v41._0_1_ = '\0';
        if (v16)
          v6 = v16;
        goto label_7639;
      }
      break;
    case 0x43:
      if (v15 != 0x45) {
        v26 = *(int *)&a2->field_0x14;
        v36 = v26 < -0x76c;
        v14 = (int)((-(unsigned int)(v26 + 0x76cU < 0x76c) & 0xffffff9d) + v26) / 100 + 0x13;
        v7 = -0x76c <= v26;
        if (!v28)
          v28 = a4;
        if (v28 != 0x2b) { // branch-flip
          v2 = 0;
          v15 = 2;
        }
        else {
          v15 = 2;
          v27 = 99;
label_8899:
          if (v14 <= v27) goto label_885d;
          v2 = 1;
          v28 = 0x2b;
          v7 = (bool)(v36 ^ 1);
        }
        goto label_785a;
      }
      goto label_7872;
    case 0x44:
      if (!v15) {
        v15 = -1;
        v11 = "%m/%d/%y";
        goto label_7db9;
      }
      break;
    case 0x46:
      if (v15) break;
      if ((0 <= v31) || (v28)) {
        v15 = v31 + -6;
        v11 = "%Y-%m-%d";
        if (v15 <= -1)
          v15 = 0;
        goto label_7db9;
      }
      v19 = 0;
      v15 = 4;
      v11 = "%Y-%m-%d";
      v32 = sub_72b0(0,"%Y-%m-%d",a2,(unsigned char)v6,0x2b,4,v5,v26);
      v28 = 0x2b;
      v39 = v32;
      goto label_7e11;
    case 0x47:
    case 0x56:
    case 0x67:
      if (v15 != 0x45) {
        v17 = *(int *)&a2->field_0x14;
        v14 = v17 + -100 + (v17 >> 0x1f & 400U);
        v15 = *(int *)&a2->field_0x1c;
        v26 = sub_71b0(v15,*(unsigned int *)&a2->field_0x18);
        if (0 <= v26) { // branch-flip
          v8 = 0x16d;
          if ((!(v14 & 3)) && (v8 = 0x16e, (v14 * -0x3d70a3d7 + 0x51eb850 >> 2 | v17 * 0x40000000) <= 0x28f5c28))
            v8 = 0x16e - (unsigned int)((int)v14 % 400 != 0);
          v15 = sub_71b0(v15 - v8);
          if (0 <= v15)
            v26 = v15;
          v8 = (v15 >> 0x1f) + 1;
        }
        else {
          v14 -= 1;
          v26 = 0x16d;
          if ((!(v14 & 3)) && (v26 = 0x16e, (v14 * -0x3d70a3d7 + 0x51eb850 >> 2 | v14 * 0x40000000) <= 0x28f5c28))
            v26 = 0x16e - (unsigned int)((int)v14 % 400 != 0);
          v26 = sub_71b0(v15 + v26);
          v8 = -1;
        }
        if (v29 == 'G') {
          v14 = v17 + 0x76c + v8;
          v36 = v17 < 0xfffff894U - v8;
          if (v28) { // branch-flip
label_8884:
            v15 = 4;
            v27 = 9999;
            if (v28 == 0x2b) goto label_8899;
            v2 = 0;
            v7 = (bool)(v36 ^ 1);
          }
          else {
            if (a4 == 0x2b) {
label_8aa6:
              v15 = 4;
              v27 = 9999;
              goto label_8899;
            }
            v2 = 0;
            v15 = 4;
            v7 = 0xfffff894U - v8 <= v17;
            v28 = a4;
          }
          goto label_785a;
        }
        if (v29 != 'g') {
          v15 = 2;
          v14 = v26 / 7 + 1;
          goto label_7a78;
        }
        v14 = (v17 % 100 + v8) % 100;
        if (0 <= (int)v14) goto label_7830;
        if (v17 < 0xfffff894U - v8) {
          v14 = -v14;
          if (!v28) goto label_7842;
label_884b:
          v27 = v28;
          goto label_7842;
        }
        v14 += 100;
        if (v28) goto label_884b;
        if (a4 == 0x2b) goto label_8855;
        goto label_7848;
      }
      break;
    case 0x48:
      if (v15 != 0x45) {
        v15 = 2;
        v14 = *(unsigned int *)&a2->field_0x8;
        goto label_7a78;
      }
      break;
    case 0x49:
      if (v15 != 0x45) {
        v15 = 2;
        goto label_7a78;
      }
      break;
    case 0x4d:
      if (v15 != 0x45) {
        v15 = 2;
        v14 = *(unsigned int *)&a2->field_0x4;
        goto label_7a78;
      }
      break;
    case 0x4e:
      if (v15 != 0x45) {
        v17 = 9;
        v15 = v26;
        if (v31 <= 0)
          v31 = 9;
        do {
          if (v17 <= v31) {
            if (v17 == 1) {
              v19 = 1;
              v32 = 1;
              goto label_7963;
            }
            if (v15 % 10) goto label_7955;
          }
          v17 -= 1;
          v15 /= 10;
        } while( true );
      }
      break;
    case 0x50:
      v41._0_1_ = '\x01';
      goto label_8474;
    case 0x52:
      v15 = -1;
      v11 = "%H:%M";
      goto label_7db9;
    case 0x53:
      if (v15 != 0x45) {
        v15 = 2;
        v14 = *(unsigned int *)a2;
        goto label_7a78;
      }
      break;
    case 0x54:
      v15 = -1;
      v11 = "%H:%M:%S";
label_7db9:
      v32 = sub_72b0(0,v11,a2,(unsigned char)v6,v28,v15,v5,v26);
      if ((v28 != 0x2d) && (0 <= v31)) {
        v19 = (unsigned long)v31;
        v39 = v19;
        if (v19 <= v32)
          v39 = v32;
      }
      else {
        v39 = v32;
        v19 = 0;
      }
label_7e11:
      if (~v35 <= v39) goto label_73c0;
      if (a0) {
        if (v32 < v19) {
          if ((v28 != 0x30) && (v33 = 0, v28 != 0x2b)) {
            do {
              v33 += 1;
              fputc(0x20,a0);
            } while (v19 - v32 != v33);
          }
          else {
            v33 = 0;
            do {
              v33 += 1;
              fputc(0x30,a0);
            } while (v19 - v32 != v33);
          }
        }
        sub_72b0(a0,v11,a2,(unsigned char)v6,v28,v15,v5,a7);
      }
      v35 += v39;
      goto label_738d;
    case 0x55:
      if (v15 != 0x45) {
        v17 = (*(int *)&a2->field_0x1c - *(int *)&a2->field_0x18) + 7;
label_7b23:
        v15 = 2;
        v14 = v17 / 7;
        goto label_7a78;
      }
      break;
    case 0x57:
      if (v15 != 0x45) {
        v15 = *(int *)&a2->field_0x18 + 6;
        v17 = ((v15 / 7) * 7 - v15) + 7 + *(int *)&a2->field_0x1c;
        goto label_7b23;
      }
      break;
    case 0x58:
    case 99:
    case 0x78:
      if (v15 != 0x4f) goto label_7634;
      break;
    case 0x59:
      if (v15 == 0x45) goto label_7872;
      if (v15 == 0x4f) break;
      v36 = *(int *)&a2->field_0x14 < -0x76c;
      v14 = *(int *)&a2->field_0x14 + 0x76c;
      if (v28) goto label_8884;
      if (a4 == 0x2b) goto label_8aa6;
      v2 = 0;
      v27 = 0;
      v15 = 4;
      v28 = a4;
      goto label_8160;
    case 0x5a:
      if (v16)
        v6 = 0;
      v32 = strlen((char *)v9);
      if ((0 <= v31) && (v28 != 0x2d)) {
        v19 = (unsigned long)v31;
        v40 = v19;
        if (v19 <= v32)
          v40 = v32;
      }
      else {
        v40 = v32;
        v19 = 0;
      }
      if (~v35 <= v40) goto label_73c0;
      if (a0) {
        if (v32 < v19) {
          if ((v28 != 0x30) && (v28 != 0x2b)) {
            v33 = 0;
            if (v19 != v32) {
              do {
                v33 += 1;
                fputc(0x20,a0);
              } while (v19 - v32 != v33);
            }
          }
          else if (v19 != v32) {
            v33 = 0;
            do {
              v33 += 1;
              fputc(0x30,a0);
            } while (v19 - v32 != v33);
          }
        }
        if (v16) // branch-flip
          sub_71f0(a0,v9,v32);
        else if (v6) // branch-flip
          sub_7250(a0,v9,v32);
        else {
          fwrite(v9,v32,1,a0);
        }
      }
      goto label_7740;
    case 0x62:
    case 0x68:
      if (v16)
        v6 = v16;
      if (v15 != 0x45) goto label_7634;
      break;
    case 100:
      if (v15 != 0x45) {
        v15 = 2;
        v14 = *(unsigned int *)&a2->field_0xc;
        goto label_7a78;
      }
      break;
    case 0x65:
      if (v15 != 0x45) {
        v14 = *(unsigned int *)&a2->field_0xc;
label_8075:
        v15 = 2;
        if (!v28)
          v28 = 0x5f;
        goto label_7a78;
      }
      break;
    case 0x6a:
      if (v15 != 0x45) {
        v2 = 0;
        v15 = 3;
        v17 = *(int *)&a2->field_0x1c;
        v14 = v17 + 1;
        v36 = v17 < -1;
        v7 = -1 <= v17;
        v27 = 0;
        goto label_7860;
      }
      break;
    case 0x6b:
      if (v15 != 0x45) {
        v14 = *(unsigned int *)&a2->field_0x8;
        goto label_8075;
      }
      break;
    case 0x6c:
      if (v15 != 0x45) goto label_8075;
      break;
    case 0x6d:
      if (v15 != 0x45) {
        v2 = 0;
        v15 = 2;
        v17 = *(int *)&a2->field_0x10;
        v14 = v17 + 1;
        v36 = v17 < -1;
        v7 = -1 <= v17;
        v27 = 0;
        goto label_7860;
      }
      break;
    case 0x6e:
      if ((v28 != 0x2d) && (0 <= v31)) {
        v19 = (unsigned long)v31;
        v32 = 1;
        if (v19)
          v32 = v19;
        if (~v35 <= v32) goto label_73c0;
        if (a0) {
          if (2 <= v31) {
            if ((v28 != 0x30) && (v33 = 0, v28 != 0x2b)) {
              do {
                v33 += 1;
                fputc(0x20,a0);
              } while (v19 - 1 != v33);
            }
            else {
              v33 = 0;
              do {
                v33 += 1;
                fputc(0x30,a0);
              } while (v19 - 1 != v33);
            }
          }
label_83f4:
          fputc(10,a0);
        }
      }
      else {
        if (0xfffffffffffffffe <= v35) goto label_73c0;
        v32 = 1;
        if (a0) goto label_83f4;
      }
      goto label_8401;
    case 0x70:
      v41._0_1_ = '\0';
label_8474:
      v29 = 'p';
      if (v16)
        v41._0_1_ = v16;
      if (v16)
        v6 = 0;
label_7639:
      if (v15) goto label_7884;
      v11 = &v20;
      goto label_7657;
    case 0x71:
      if (v15 == 0x4f) goto label_7872;
      v2 = 0;
      v36 = 0;
      v27 = 0;
      v15 = 1;
      v14 = (*(int *)&a2->field_0x10 * 0xb >> 5) + 1;
      goto label_7ff0;
    case 0x72:
label_7634:
      v41._0_1_ = '\0';
      goto label_7639;
    case 0x73:
      v45 = *(unsigned long *)&a2->field_0x20;
      v46 = *(unsigned long *)&a2->field_0x28;
      v24 = *(unsigned long *)a2;
      v42 = *(unsigned long *)&a2->field_0x8;
      v43 = *(unsigned long *)&a2->field_0x10;
      v47 = *(unsigned long *)&a2->field_0x30;
      v44._0_4_ = (unsigned int)*(unsigned long *)&a2->field_0x18;
      v44 = CONCAT44(0xffffffff,(unsigned int)v44);
      v33 = sub_10d80(v5,&v24);
      if (v44 <= -1) {
        *v37 = 0x4b;
        return 0;
      }
      v36 = (bool)((unsigned long)v33 >> 0x3f);
      v11 = &v25[1];
      v34 = v33;
      do {
        v3 = v34 / 10;
        v29 = (char)v34 + (char)v3 * '\xf6';
        if (v33 <= -1)
          v29 = -v29;
        v11 = &v11[-1];
        *v11 = v29 + '0';
        v34 = v3;
      } while (v3);
      v2 = 0;
      v15 = 1;
      goto label_7c30;
    case 0x74:
      if ((v28 != 0x2d) && (0 <= v31)) {
        v19 = (unsigned long)v31;
        v32 = 1;
        if (v19)
          v32 = v19;
        if (~v35 <= v32) goto label_73c0;
        if (a0) {
          if (2 <= v31) {
            if ((v28 != 0x30) && (v34 = 0, v28 != 0x2b)) {
              do {
                v34 += 1;
                fputc(0x20,a0);
              } while (v19 - 1 != v34);
            }
            else {
              v34 = 0;
              do {
                v34 += 1;
                fputc(0x30,a0);
              } while (v19 - 1 != v34);
            }
          }
label_7f52:
          fputc(9,a0);
        }
      }
      else {
        if (0xfffffffffffffffe <= v35) goto label_73c0;
        v32 = 1;
        if (a0) goto label_7f52;
      }
label_8401:
      v35 += v32;
      goto label_738d;
    case 0x75:
      v15 = 1;
      v14 = (*(int *)&a2->field_0x18 + 6) % 7 + 1;
label_7a78:
      v2 = 0;
      v27 = 0;
      v12 = (unsigned char)(v14 >> 0x18);
      v36 = (bool)(v12 >> 7);
      v7 = (bool)((unsigned char)~v12 >> 7);
      goto label_7860;
    case 0x77:
      if (v15 != 0x45) {
        v15 = 1;
        v14 = *(unsigned int *)&a2->field_0x18;
        goto label_7a78;
      }
      break;
    case 0x79:
      if (v15 == 0x45) goto label_7872;
      v13 = *(int *)&a2->field_0x14 % 100;
      v14 = v13;
      if (v13 <= -1) {
        v14 = v13 + 100;
        if (*(int *)&a2->field_0x14 <= -0x76d)
          v14 = -v13;
      }
label_7830:
      if (v28) goto label_884b;
label_7842:
      if (v27 != 0x2b) { // branch-flip
label_7848:
        v2 = 0;
        v7 = 1;
        v36 = 0;
        v15 = 2;
        v28 = v27;
      }
      else {
label_8855:
        v36 = 0;
        v15 = 2;
label_885d:
        v2 = v15 < v31;
        v7 = (bool)(v36 ^ 1);
        v28 = 0x2b;
      }
label_785a:
      v27 = 0;
      goto label_7860;
    case 0x7a:
      v32 = 0;
      v11 = v30;
label_82c7:
      v30 = v11;
      if (*(int *)&a2->field_0x20 < 0) goto label_738d;
      v36 = 1;
      v26 = (int)*(unsigned long *)&a2->field_0x28;
      if ((0 <= v26) && (v36 = 0, !v26))
        v36 = *v9 == 0x2d;
      v14 = v26 / 0xe10;
      v17 = (v26 / 0x3c) % 0x3c;
      if (v32 != 2) { // branch-flip
        if (3 <= v32) { // branch-flip
          if (v32 != 3) break;
          if (v26 % 0x3c) goto label_89cb;
          if (v17) goto label_8367;
          v2 = 1;
          v15 = 3;
          v7 = (bool)(v36 ^ 1);
          v27 = 0;
        }
        else if (v32) { // branch-flip
label_8367:
          v2 = 1;
          v15 = 6;
          v7 = (bool)(v36 ^ 1);
          v14 = v14 * 100 + v17;
          v27 = 4;
        }
        else {
          v2 = 1;
          v15 = 5;
          v7 = (bool)(v36 ^ 1);
          v14 = v14 * 100 + v17;
          v27 = 0;
        }
      }
      else {
label_89cb:
        v2 = 1;
        v15 = 9;
        v27 = 0x14;
        v14 = v26 % 0x3c + v14 * 10000 + v17 * 100;
        v7 = (bool)(v36 ^ 1);
      }
label_7860:
      if ((int)v10 != 0x4f) { // branch-flip
label_8160:
        if (v36)
          v14 = -v14;
label_7ff0:
        v18 = &v25[1];
        while( true ) {
          if (v27 & 1) {
            v18[-1] = ':';
            v18 = &v18[-1];
          }
          v27 = (int)v27 >> 1;
          v11 = &v18[-1];
          v18[-1] = (char)v14 + (char)(v14 / 10) * '\xf6' + '0';
          if ((v14 <= 9) && (!v27)) break;
          v14 /= 10;
          v18 = v11;
        }
label_7c30:
        v14 = v28;
        if (!v28)
          v14 = 0x30;
        if (v31 <= -1)
          v31 = v15;
        v26 = ((int)v25 + 1) - (int)v11;
        if (v36) { // branch-flip
          v40._0_1_ = '-';
label_84e5:
          v15 = (v31 + -1) - v26;
          if ((v15 <= 0) || (v28 == 0x2d))
            v15 = 0;
          if (v14 == 0x5f) {
            if ((a0) && (v15)) {
              v33 = 0;
              do {
                v33 += 1;
                fputc(0x20,a0);
              } while (v33 != v15);
            }
            v35 += (long)v15;
            v31 -= v15;
          }
          if (0xfffffffffffffffe <= v35) goto label_73c0;
          if (a0)
            fputc((int)(char)v40,a0);
          v40 = (unsigned long)v26;
          v35 += 1;
          v32 = 0;
          if ((v14 != 0x2d) && (v31 = v31 + -1, 0 <= v31)) {
label_7c77:
            v32 = (unsigned long)v31;
            v40 = (long)v26;
            if ((unsigned long)(long)v26 <= v32)
              v40 = v32;
          }
        }
        else {
          if (v2) {
            v40._0_1_ = '+';
            goto label_84e5;
          }
          v40 = (unsigned long)v26;
          if (((v26 < v31) && (v28 != 0x2d)) || (v14 != 0x2d)) goto label_7c77;
          v32 = 0;
        }
        v19 = (unsigned long)v26;
        if (~v35 <= v40) goto label_73c0;
        if (a0) {
          if (v19 < v32) {
            if ((v14 != 0x30) && (v33 = 0, v14 != 0x2b)) {
              do {
                v33 += 1;
                fputc(0x20,a0);
              } while (v32 - v19 != v33);
            }
            else {
              v33 = 0;
              do {
                v33 += 1;
                fputc(0x30,a0);
              } while (v32 - v19 != v33);
            }
          }
          if (v6) // branch-flip
            sub_7250(a0,v11,v19);
          else {
            fwrite(v11,v19,1,a0);
          }
        }
label_7740:
        v35 += v40; // crossjump-dupe
      }
      else {
        if (!v7) goto label_8160;
label_7872:
        v41._0_1_ = '\0';
label_7884:
        v20 = (char)v10;
        v11 = &v23;
label_7657:
        v21 = 0x2520;
        *v11 = v29;
        v11[1] = '\0';
        v32 = strftime(&v22[1],0x400,(char *)&v21,a2);
        if (v32) {
          v32 -= 1;
          if ((v28 != 0x2d) && (0 <= v31)) {
            v19 = (unsigned long)v31;
            v40 = v19;
            if (v19 <= v32)
              v40 = v32;
          }
          else {
            v19 = 0;
            v40 = v32;
          }
          if (~v35 <= v40) goto label_73c0;
          if (a0) {
            if (v32 < v19) {
              if ((v28 != 0x30) && (v28 != 0x2b)) {
                v33 = 0;
                if (v19 != v32) {
                  do {
                    v33 += 1;
                    fputc(0x20,a0);
                  } while (v19 - v32 != v33);
                }
              }
              else if (v19 != v32) {
                v33 = 0;
                do {
                  v33 += 1;
                  fputc(0x30,a0);
                } while (v19 - v32 != v33);
              }
            }
            if ((char)v41) // branch-flip
              sub_71f0(a0,&v22[2],v32);
            else if (v6) // branch-flip
              sub_7250(a0,&v22[2],v32);
            else {
              fwrite(&v22[2],v32,1,a0);
            }
          }
          v35 += v40;
        }
      }
      goto label_738d;
    
  }
  v11 = &v30[1 - (long)a1];
  v41 = v11;
  if ((v28 != 0x2d) && (0 <= v31)) {
    v18 = (char *)(long)v31;
    if (v11 < v18)
      v41 = v18;
  }
  else {
    v18 = NULL;
  }
  if ((char *)~v35 <= v41) goto label_73c0;
  if (a0) {
    if (v11 < v18) {
      if ((v28 != 0x30) && (v28 != 0x2b)) {
        v33 = 0;
        if (v18 != v11) {
          do {
            v33 += 1;
            fputc(0x20,a0);
          } while ((long)v18 - (long)v11 != v33);
        }
      }
      else if (v18 != v11) {
        v33 = 0;
        do {
          v33 += 1;
          fputc(0x30,a0);
        } while ((long)v18 - (long)v11 != v33);
      }
    }
    if (v6) // branch-flip
      sub_7250(a0,a1,v11);
    else {
      fwrite(a1,(unsigned long)v11,1,a0);
    }
  }
  v35 += (long)v41;
  goto label_738d;
label_7955:
  v19 = (unsigned long)v17;
  v32 = v19;
  if (v17) { // branch-flip
label_7963:
    v11 = &v22[v32 + 1];
    do {
      v11 = &v11[-1];
      *v11 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
      v15 /= 10;
    } while (&v22[v32 - (unsigned int)(v17 - 1)] != v11);
  }
  else {
    v19 = 0;
  }
  if (!v28)
    v28 = 0x30;
  if (v19 >= ~v35) {
label_73c0:
    *v37 = 0x22;
    return 0;
  }
  if (a0) {
    if (v6) // branch-flip
      sub_7250(a0,&v22[1],v19);
    else {
      fwrite(&v22[1],v19,1,a0);
    }
  }
  v35 += v19;
  if ((v28 != 0x2d) && (0 <= v31 - v17)) {
    v32 = (unsigned long)(v31 - v17);
    if (~v35 <= v32) goto label_73c0;
    if (a0) {
      if (!v32) goto label_738d;
      if ((v28 != 0x30) && (v19 = 0, v28 != 0x2b)) {
        do {
          v19 += 1;
          fputc(0x20,a0);
        } while (v32 != v19);
      }
      else {
        v19 = 0;
        do {
          v19 += 1;
          fputc(0x30,a0);
        } while (v32 != v19);
      }
    }
    v35 += v32;
  }
  else if (v35 == 0xffffffffffffffff) goto label_73c0;
label_738d:
  v6 = v30[1];
  a1 = &v30[1];
  v32 = 0xffffffffffffffff;
  v4 = v37;
  v33 = a6;
  v26 = a7;
  goto label_7342;
}

// Function: sub_8b90 @ 0x8b90
unsigned long sub_8b90(void)
{
  return sub_72b0();
}

// Function: sub_8bb0 @ 0x8bb0
unsigned long sub_8bb0(int a0)
{
  int v1; // eax
  unsigned long v2; // r12
  
  v2 = 1;
  v1 = open("/dev/null",0);
  if ((a0 != v1) && (v2 = 0, 0 <= v1)) {
    close(v1);
    *__errno_location() = 9;
    return 0;
  }
  return v2;
}

// Function: sub_8c10 @ 0x8c10
FILE * sub_8c10(char *a0,char *a1,FILE *a2)
{
  char v1; // al
  int v2;
  int *v3;
  FILE *v4;
  char v5;
  char v6;
  bool v7;
  
  v2 = fileno(a2);
  if (v2 != 1) { // branch-flip
    if (v2 != 2) { // branch-flip
      if (!v2) goto label_8d4d;
      v5 = dup2(2,2) != 2;
    }
    else {
      v5 = 0;
    }
    v6 = dup2(1,1) != 1;
    if (dup2(0,0)) { // branch-flip
      v1 = sub_8bb0(0);
      if (v1) goto label_8c8f;
      v1 = 1;
      v4 = NULL;
label_8cdd:
      v3 = __errno_location();
      v2 = *v3;
      if (v5) {
label_8dd6:
        close(2);
      }
      if (v6) goto label_8dc0;
    }
    else {
      v1 = 0;
label_8c8f:
      v3 = __errno_location();
      v7 = !(bool)v6;
      v6 = '\0';
      if ((v7) || (v6 = sub_8bb0(1), v6)) {
        v7 = (bool)v5 == 0;
        v5 = '\0';
        if ((v7) || (v5 = sub_8bb0(2), v5)) goto label_8cca;
        v2 = *v3;
        v4 = NULL;
        goto label_8dd6;
      }
      v2 = *v3;
      v4 = NULL;
      if ((bool)v5)
        close(2);
label_8dc0:
      close(1);
    }
    if (!v1) goto label_8da2;
  }
  else {
    if (!dup2(0,0)) {
label_8d4d:
      v5 = 0;
      v6 = 0;
      v1 = 0;
label_8cca:
      v4 = freopen(a0,a1,a2);
      goto label_8cdd;
    }
    v5 = 0;
    v6 = 0;
    v1 = sub_8bb0(0);
    if (v1) goto label_8c8f;
    v3 = __errno_location();
    v4 = NULL;
    v2 = *v3;
  }
  close(0);
label_8da2:
  if (!v4)
    *v3 = v2;
  return v4;
}

// Function: sub_8e00 @ 0x8e00
int sub_8e00(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_8e80 @ 0x8e80
unsigned long sub_8e80(struct_6 *a0,struct_6 *a1)
{
  if (a0->field_0x8 != a1->field_0x8)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x0 >> 8),a0->field_0x0 == a1->field_0x0);
}

// Function: sub_8eb0 @ 0x8eb0
unsigned long sub_8eb0(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_8ed0 @ 0x8ed0
unsigned long sub_8ed0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_8ee0 @ 0x8ee0
unsigned long sub_8ee0(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_8ef0 @ 0x8ef0
int sub_8ef0(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_8f10 @ 0x8f10
void sub_8f10(long a0,int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if ((unsigned int)(a1 - 1U) <= 0xb)
    v1 = *(unsigned int *)((unsigned long)(unsigned int)(a1 - 1U) * 4 + 0x159a0);
  *(unsigned int *)(a0 + 0x18) = v1;
}

// Function: sub_8f30 @ 0x8f30
long sub_8f30(long *a0)
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

// Function: sub_8f80 @ 0x8f80
unsigned long sub_8f80(long a0,long a1)
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

// Function: sub_8ff0 @ 0x8ff0
void sub_8ff0(long a0,unsigned char a1)
{
  if (*(short *)(a0 + 0x68) == 0xb) {
    *(unsigned long *)(a0 + 0xa0) = (unsigned long)a1 + 1;
    return;
  }
  abort(); // no-return
}

// Function: sub_9010 @ 0x9010
int sub_9010(long a0,long a1,bool a2) // early-return x3
{
  stat *v1;
  unsigned long *v2;
  char *v3;
  unsigned int v4; // eax
  int v5;
  int *v6; // rax
  unsigned long v7; // rcx
  unsigned long *v8;
  
  v1 = (stat *)(a1 + 0x70);
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
      goto label_90ab;
    }
  }
  else {
    v3 = *(char **)(a1 + 0x30);
    v5 = *(int *)(a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_90ab:
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

// Function: sub_91c0 @ 0x91c0
long sub_91c0(long a0,long a1,unsigned long a2)
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
label_921b:
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
      goto label_921b;
    }
    v4 = *(long **)(a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  return a1;
}

// Function: sub_92d0 @ 0x92d0
void * sub_92d0(long a0,void *a1,unsigned long a2)
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

// Function: sub_9370 @ 0x9370
unsigned long sub_9370(long a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) {
    v1 = sub_bc70(0x1f,0,sub_8eb0,sub_8e80,dat_1cfb0);
    *(long *)(a0 + 0x58) = v1;
    return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)(a0 + 0x58) = v2;
  if (v2) {
    sub_130c0(v2);
    return 1;
  }
  return 0;
}

// Function: sub_93e0 @ 0x93e0
void sub_93e0(void *a0)
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

// Function: sub_9430 @ 0x9430
void sub_9430(long a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_d0c0(a0)) break;
    v1 = sub_d110(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_9470 @ 0x9470
unsigned long sub_9470(long a0,int a1)
{
  long v1;
  long v2;
  struct_3 *v3; // rax
  struct_3 *v4; // rax
  unsigned long v5 [2]; // stack - 0xb8
  statfs v6; // stack - 0xa8
  
  v1 = *(long *)(a0 + 0x50);
  v2 = *(long *)(v1 + 0x50);
  if (!(*(unsigned char *)(v1 + 0x49) & 2))
    return 0;
  if (!v2) {
    v2 = sub_bc70(0xd,0,sub_8ed0,sub_8ee0,dat_1cfb0);
    *(long *)(v1 + 0x50) = v2;
    if (!v2) goto label_94fe;
  }
  v5[0] = *(unsigned long *)(a0 + 0x70);
  v1 = sub_ba10(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_94fe:
  if ((0 <= a1) && (!fstatfs(a1,&v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      v3->field_0x0 = *(unsigned long *)(a0 + 0x70);
      v3->field_0x8 = v6._0_8_;
      v4 = (struct_3 *)sub_c290(v2,v3);
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

// Function: sub_95a0 @ 0x95a0
bool sub_95a0(long a0,int a1)
{
  long v1; // rax
  bool v2; // r8b
  
  v1 = sub_9470(a0,a1);
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

// Function: sub_95f0 @ 0x95f0
unsigned long sub_95f0(long a0,int a1)
{
  unsigned long v1; // rax
  
  v1 = sub_9470(a0,a1);
  if ((v1 != 0x1021994) && (v1 != 0xff534d42))
    return CONCAT71((undefined7)(v1 >> 8),v1 != 0x6969);
  return 0;
}

// Function: sub_9630 @ 0x9630
void sub_9630(struct_18 *a0,long a1)
{
  char *v1;
  void *v2;
  long v3;
  char *v4; // rax
  unsigned long v5; // rax
  
  v1 = (char *)(a1 + 0x100);
  v2 = (void *)a0->field_0x20;
  *(long *)(a1 + 0x48) = *(long *)(a1 + 0x60);
  memmove(v2,v1,*(long *)(a1 + 0x60) + 1);
  v4 = strrchr(v1,0x2f);
  if ((v4) && ((v1 != v4 || (*(char *)(a1 + 0x101))))) {
    v5 = strlen(&v4[1]);
    memmove(v1,&v4[1],v5 + 1);
    *(unsigned long *)(a1 + 0x60) = v5;
  }
  v3 = a0->field_0x20;
  *(long *)(a1 + 0x38) = v3;
  *(long *)(a1 + 0x30) = v3;
}

// Function: sub_96c0 @ 0x96c0
unsigned long sub_96c0(long a0,struct_15 *a1) // return-dupe x2
{
  long v1;
  struct_16 *v2; // rax
  struct_16 *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v4 = sub_130e0(*(struct_14 **)(a0 + 0x58),&a1->field_0x70);
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
  v1 = *(long *)(a0 + 0x58);
  v2->field_0x10 = a1;
  *(unsigned long *)v2->field_0x0 = v4;
  *(unsigned long *)((long)v2->field_0x0 + 8) = a1->field_0x78;
  v3 = (struct_16 *)sub_c290(v1,v2);
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

// Function: sub_9770 @ 0x9770
void sub_9770(long a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_d0d0(a0 + 0x60);
    if (0 <= v1) {
      close(v1);
      goto label_97b4;
    }
  }
  else if ((!(*(unsigned char *)(a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_97b4:
    *(int *)(a0 + 0x2c) = a1;
    return;
  }
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_97d0 @ 0x97d0
bool sub_97d0(long a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)(a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
    sub_9770(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)(a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  sub_9430(a0 + 0x60);
  return v2;
}

// Function: sub_9840 @ 0x9840
void sub_9840(long a0,long a1)
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
    v4 = (void *)sub_c2f0(*(long *)(a0 + 0x58),&v5);
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

// Function: sub_9900 @ 0x9900
void sub_9900(long a0,long a1,long a2)
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

// Function: sub_9980 @ 0x9980
void sub_9980(unsigned long a0,unsigned int a1,char *a2)
{
  if (a1 & 0x200) {
    sub_e230(a0,a2); // tail-call
    return;
  }
  sub_133d0(a2,(a1 & 0x10) << 0xd | 0x90900); // tail-call
}

// Function: sub_99c0 @ 0x99c0
int sub_99c0(long a0,long a1,int a2,char *a3) // return-dupe
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
    v6 = *(unsigned int *)(a0 + 0x48);
    if (v6 & 4) {
label_9b50:
      if ((v6 & 0x200) && (0 <= a2)) {
        close(a2);
        return 0;
      }
      return 0;
    }
    if (a2 <= -1) {
      if (!(v6 & 0x200)) {
label_9a37:
        v7 = 1;
        goto label_9a3c;
      }
      v7 = sub_d0c0(a0 + 0x60);
      if (v7) goto label_9a3c;
      v3 = sub_d110(a0 + 0x60);
      v6 = *(unsigned int *)(a0 + 0x48);
      if (v3 < 0) goto label_9a37;
      v7 = 1;
      a2 = v3;
      v1 = v3;
      if (v6 & 2) goto label_9a68;
      goto label_9aa0;
    }
    v7 = 1;
    if (!(v6 & 2)) goto label_9b2d;
label_9a68:
    v3 = a2;
    if (!fstat(v1,&v5)) {
      if ((*(long *)(a1 + 0x70) == v5._0_8_) && (*(long *)(a1 + 0x78) == v5._8_8_)) {
        v6 = *(unsigned int *)(a0 + 0x48);
        goto label_9aa0;
      }
      *__errno_location() = 2;
    }
    v2 = -1;
  }
  else {
    v6 = *(unsigned int *)(a0 + 0x48);
    if (v6 & 4) goto label_9b50;
    if (0 <= a2) // branch-flip
      v7 = 0;
    else {
      v7 = 0;
label_9a3c:
      v1 = sub_9980(*(unsigned int *)(a0 + 0x2c),v6,a3);
      if (v1 < 0)
        return -1;
      v6 = *(unsigned int *)(a0 + 0x48);
    }
    if (v6 & 2) goto label_9a68;
    v3 = a2;
    if (a3) {
label_9b2d:
      v3 = a2;
      if (!strcmp(a3,"..")) goto label_9a68;
    }
label_9aa0:
    if (v6 & 0x200) {
      sub_9770(a0,v1,v7 ^ 1);
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

// Function: sub_9c70 @ 0x9c70
void * sub_9c70(struct_17 *a0,int a1) // return-dupe
{
  long v1;
  dirent *v10; // rax
  unsigned long v11;
  void *v12; // rax
  long v13;
  unsigned int v14;
  int v15; // stack - 0x44
  unsigned int v16;
  DIR *v17;
  unsigned long v18;
  unsigned long v19;
  DIR *v2;
  bool v20;
  unsigned long v21; // stack - 0xa0
  void *v22; // stack - 0x98
  void *v23; // stack - 0x90
  char *v24; // stack - 0x70
  unsigned long v25; // stack - 0x68
  unsigned long v26; // stack - 0x60
  bool v27; // stack - 0x4b
  bool v28; // stack - 0x4a
  long v3;
  bool v4;
  char v5; // al
  int v6; // eax
  int v7;
  char *v8;
  int *v9; // rax
  
  v1 = a0->field_0x0;
  v2 = *(DIR **)(v1 + 0x18);
  if (v2) { // branch-flip
    v15 = dirfd(v2);
    if (v15 < 0) {
      closedir(*(DIR **)(v1 + 0x18));
      *(unsigned long *)(v1 + 0x18) = 0;
      if (a1 == 3) {
        *(unsigned short *)(v1 + 0x68) = 4;
        *(int *)(v1 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (a0->field_0x40) { // branch-flip
      v25 = 0xffffffffffffffff;
label_9cd2:
      v20 = 1;
      v14 = a0->field_0x48;
    }
    else {
      v25 = 100000;
      v14 = a0->field_0x48;
      v20 = 1;
    }
  }
  else {
    v14 = a0->field_0x48;
    v7 = 0;
    if ((v14 & 0x10) && (v7 = 0x20000, v14 & 1))
      v7 = (unsigned int)(*(long *)(v1 + 0x58) != 0) << 0x11;
    v16 = 0xffffff9c;
    if ((v14 & 0x204) == 0x200)
      v16 = a0->field_0x2c;
    v13 = sub_e2b0(v16,*(char **)(v1 + 0x30),v7,&v15);
    *(long *)(v1 + 0x18) = v13;
    if (!v13) {
      if (a1 == 3) {
        *(unsigned short *)(v1 + 0x68) = 4;
        *(int *)(v1 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (*(short *)(v1 + 0x68) != 0xb) { // branch-flip
      if (a0->field_0x48 & 0x100) {
        sub_9840(a0,v1);
        sub_9010(a0,v1,0);
        if (!sub_96c0(a0,v1)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)(v1 + 0x68) = sub_9010(a0,v1,0);
    }
    v25 = (unsigned long)(-(unsigned int)(a0->field_0x40 == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_a380:
      v20 = 0;
      v14 = a0->field_0x48;
      goto label_9cdb;
    }
    v7 = v15;
    if (((a0->field_0x48 & 0x38) != 0x18) || (*(long *)(v1 + 0x80) != 2)) {
label_a0b7:
      v20 = a1 == 3;
      v4 = 1;
    }
    else {
      v6 = sub_95a0(v1,v15);
      v7 = v15;
      if (!v6) goto label_a0b7;
      if (a1 != 3) goto label_a380;
      v4 = 0;
      v20 = 1;
    }
    if (a0->field_0x48 & 0x200) {
      v7 = sub_13280(v7,0x406,3);
      v15 = v7;
    }
    if ((0 <= v7) && (!sub_99c0(a0,v1,v7,NULL))) goto label_9cd2;
    if ((v20) && (v4))
      *(int *)(v1 + 0x40) = *__errno_location();
    *(unsigned short *)(v1 + 0x6a) = *(unsigned short *)(v1 + 0x6a) | 1;
    closedir(*(DIR **)(v1 + 0x18));
    v14 = a0->field_0x48;
    *(unsigned long *)(v1 + 0x18) = 0;
    if ((v14 & 0x200) && (0 <= v15)) {
      close(v15);
      v14 = a0->field_0x48;
    }
    *(unsigned long *)(v1 + 0x18) = 0;
    v20 = 0;
  }
label_9cdb:
  v19 = *(unsigned long *)(v1 + 0x48);
  v21 = v19;
  v26 = v19 - 1;
  if (*(char *)(*(long *)(v1 + 0x38) + -1 + v19) != '/') {
    v21 = v19 + 1;
    v26 = v19;
  }
  v24 = NULL;
  if (v14 & 4) {
    v8 = (char *)(v26 + a0->field_0x20);
    v24 = &v8[1];
    *v8 = 0x2f;
  }
  v17 = *(DIR **)(v1 + 0x18);
  v13 = *(long *)(v1 + 0x58);
  v19 = a0->field_0x30 - v21;
  if (v17) { // branch-flip
    v9 = __errno_location();
    v18 = 0;
    v28 = 0;
    v27 = 0;
    v22 = NULL;
    v23 = NULL;
    do {
      while( true ) {
        *v9 = 0;
        v10 = readdir(v17);
        if (!v10) {
          if (*v9) {
            *(int *)(v1 + 0x40) = *v9;
            *(unsigned short *)(v1 + 0x68) = (-(unsigned short)(!v2 && !v18) & 0xfffd) + 7;
          }
          if (*(DIR **)(v1 + 0x18)) {
            closedir(*(DIR **)(v1 + 0x18));
            *(unsigned long *)(v1 + 0x18) = 0;
          }
          goto label_a234;
        }
        if (((a0->field_0x48 & 0x20) || (v10->field_0x13 != '.')) || ((v10->field_0x14 && (*(short *)&v10->field_0x14 != 0x2e)))) break;
label_9da3:
        v17 = *(DIR **)(v1 + 0x18);
        if (!v17) goto label_9ef0;
      }
      v11 = strlen(&v10->field_0x13);
      v12 = (void *)sub_92d0(a0,&v10->field_0x13,v11);
      if (!v12) {
label_a130:
        v7 = *v9;
        free(v12);
        sub_93e0(v23);
        closedir(*(DIR **)(v1 + 0x18));
        *(unsigned long *)(v1 + 0x18) = 0;
        *(unsigned short *)(v1 + 0x68) = 7;
        a0->field_0x48 = a0->field_0x48 | 0x2000;
        *v9 = v7;
        return NULL;
      }
      if (v19 <= v11) {
        v3 = a0->field_0x20;
        v5 = sub_8f80(a0,v26 + 2 + v11);
        if (!v5) goto label_a130;
        if (a0->field_0x20 != v3) {
          v8 = (char *)(a0->field_0x20 + v21);
          if (!(a0->field_0x48 & 4))
            v8 = v24;
          v27 = v5;
          v24 = v8;
        }
        v19 = a0->field_0x30 - v21;
      }
      if (CARRY8(v11,v21)) {
        free(v12);
        sub_93e0(v23);
        closedir(*(DIR **)(v1 + 0x18));
        *(unsigned long *)(v1 + 0x18) = 0;
        *(unsigned short *)(v1 + 0x68) = 7;
        a0->field_0x48 = a0->field_0x48 | 0x2000;
        *v9 = 0x24;
        return NULL;
      }
      *(long *)((long)v12 + 0x58) = v13 + 1;
      v3 = a0->field_0x0;
      *(unsigned long *)((long)v12 + 0x48) = v11 + v21;
      *(long *)((long)v12 + 8) = v3;
      *(unsigned long *)((long)v12 + 0x78) = *(unsigned long *)v10;
      v14 = a0->field_0x48;
      if (v14 & 4) { // branch-flip
        *(unsigned long *)((long)v12 + 0x30) = *(unsigned long *)((long)v12 + 0x38);
        memmove(v24,(void *)((long)v12 + 0x100),*(long *)((long)v12 + 0x60) + 1);
        v14 = a0->field_0x48;
      }
      else {
        *(void **)((long)v12 + 0x30) = (void *)((long)v12 + 0x100);
      }
      if ((a0->field_0x40) && (!(v14 & 0x400)))
        *(unsigned short *)((long)v12 + 0x68) = sub_9010(a0,v12,0);
      else {
        v11 = 1;
        if ((v14 & 8) && (v10->field_0x12 & 0xfb))
          v11 = (unsigned long)((v14 >> 4 & 1 | (unsigned int)(v10->field_0x12 != 10)) ^ 1);
        *(unsigned short *)((long)v12 + 0x68) = 0xb;
        sub_8f10((long)v12 + 0x70);
        sub_8ff0(v12,v11 & 0xffffffff);
      }
      *(unsigned long *)((long)v12 + 0x10) = 0;
      if (v23) // branch-flip
        *(void **)((long)v22 + 0x10) = v12;
      else {
        v23 = v12;
      }
      v11 = v18 + 1;
      if (v18 != 10000) {
        v18 = v11;
        if (v11 < v25) {
          v22 = v12;
          goto label_9da3;
        }
        goto label_a234;
      }
      if (!a0->field_0x40)
        v28 = sub_95f0(v1,v15);
      v18 = 0x2711;
      v17 = *(DIR **)(v1 + 0x18);
      v22 = v12;
    } while (v17);
label_9ef0:
label_a234:
    if (v27)
      sub_9900(a0->field_0x8,a0->field_0x20,v23);
    if (a0->field_0x48 & 4) {
      if ((a0->field_0x30 == v21) || (!v18)) goto label_a264;
label_a26a:
      *v24 = 0;
    }
    if ((v2) || (!v20)) {
label_a310:
      if (!v18) goto label_a315;
    }
    else if ((a1 == 1) || (!v18)) {
label_a2f0:
      if (*(long *)(v1 + 0x58)) // branch-flip
        v7 = sub_99c0(a0,*(long *)(v1 + 8),0xffffffff,"..");
      else {
        v7 = sub_97d0(a0);
      }
      if (v7) {
        *(unsigned short *)(v1 + 0x68) = 7;
        a0->field_0x48 = a0->field_0x48 | 0x2000;
        sub_93e0(v23);
        return NULL;
      }
      goto label_a310;
    }
    if (v28) { // branch-flip
      a0->field_0x40 = (long)sub_8ef0;
      v23 = (void *)sub_91c0(a0,v23,v18);
      a0->field_0x40 = 0;
    }
    else if ((a0->field_0x40) && (v18 != 1))
      v23 = (void *)sub_91c0(a0,v23,v18);
  }
  else {
    if (a0->field_0x48 & 4) {
      v23 = NULL;
      v18 = 0;
      v28 = 0;
label_a264:
      v24 = &v24[-1];
      goto label_a26a;
    }
    v23 = NULL;
    if ((bool)(v20 & v2 == NULL)) {
      v28 = 0;
      v18 = 0;
      goto label_a2f0;
    }
label_a315:
    if (((a1 == 3) && (*(short *)(v1 + 0x68) != 4)) && (*(short *)(v1 + 0x68) != 7))
      *(unsigned short *)(v1 + 0x68) = 6;
    sub_93e0(v23);
    v23 = NULL;
  }
  return v23;
}

// Function: sub_a5c0 @ 0xa5c0
void sub_a5c0(unsigned int a0,struct_11 *a1)
{
  if (!(a0 & 0x102)) {
    free(a1); // tail-call
    return;
  }
  if (a1) {
    sub_be30(a1); // tail-call
    return;
  }
}

// Function: sub_a5f0 @ 0xa5f0
long * sub_a5f0(long *a0,unsigned int a1,long a2)
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
  v4 = sub_8f30(a0);
  v6 = 0x1000;
  if (0x1000 <= v4)
    v6 = v4;
  v12 = sub_8f80(v3,v6);
  if (!v12) {
    free(v3); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v11 = (void *)sub_92d0(v3,"",0);
    if (v11) {
      *(unsigned long *)((long)v11 + 0x58) = 0xffffffffffffffff;
      v10 = (char *)*a0;
      if (a2) goto label_a6d9;
label_a6e8:
      if (!v10) goto label_a92d;
      v13 = 0;
      v8 = 0;
      v7 = 0;
label_a78f:
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
        v5 = sub_92d0(v3,v10,v6);
        if (!v5) goto label_a800;
        *(unsigned long *)(v5 + 0x58) = 0;
        *(void **)(v5 + 8) = v11;
        *(long *)(v5 + 0x30) = v5 + 0x100;
        if ((v8) && (v12)) {
          *(unsigned short *)(v5 + 0x68) = 0xb;
          sub_8ff0(v5,1);
          if (a2) { // branch-flip
label_a777:
            *(long *)(v5 + 0x10) = v8;
          }
          else {
            *(unsigned long *)(v5 + 0x10) = 0;
label_a908:
            v13 = v5;
            *(long *)(v7 + 0x10) = v5;
            v5 = v8;
          }
          v10 = (char *)a0[v9 + 1];
          v8 = v5;
          v7 = v9 + 1;
          if (!v10) break;
          goto label_a78f;
        }
        *(unsigned short *)(v5 + 0x68) = sub_9010(v3,v5,0);
        if (a2) goto label_a777;
        *(unsigned long *)(v5 + 0x10) = 0;
        if (v8) goto label_a908;
        v10 = (char *)a0[v9 + 1];
        v8 = v5;
        v7 = v9 + 1;
        v13 = v5;
      } while (v10);
      if ((a2) && (2 <= v9 + 1U))
        v5 = sub_91c0(v3,v5,v9 + 1U);
      goto label_a894;
    }
  }
  else {
    v11 = NULL;
    if (a2) {
      v10 = NULL;
label_a6d9:
      v12 = (unsigned char)(*(unsigned int *)&v3[9] >> 10) & 1;
      goto label_a6e8;
    }
label_a92d:
    v5 = 0;
label_a894:
    v7 = sub_92d0(v3,"",0);
    *v3 = v7;
    v8 = v5;
    if (v7) {
      *(long *)(v7 + 0x10) = v5;
      *(unsigned short *)(v7 + 0x68) = 9;
      *(unsigned long *)(v7 + 0x58) = 1;
      if (sub_9370(v3)) {
        if (!(*(unsigned int *)&v3[9] & 0x204)) {
          v2 = sub_9980(*(unsigned int *)((long)v3 + 0x2c),*(unsigned int *)&v3[9],".");
          *(int *)&v3[5] = v2;
          if (v2 <= -1)
            *(unsigned int *)&v3[9] = *(unsigned int *)&v3[9] | 4;
        }
        sub_d0a0((struct_21 *)&v3[0xc],0xffffffff);
        return v3;
      }
    }
label_a800:
    sub_93e0(v8);
    free(v11);
  }
  free((void *)v3[4]);
  free(v3);
  return NULL;
}

// Function: sub_a960 @ 0xa960
unsigned int sub_a960(struct_20 *a0) // return-dupe
{
  struct_20 *v1;
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
    sub_93e0(a0->field_0x8);
  v1 = &a0[1];
  free(a0->field_0x10);
  free(a0->field_0x20);
  if (a0->field_0x48 & 0x200) { // branch-flip
    v2 = a0->field_0x2c;
    if ((v2 <= -1) || (!close(v2))) goto label_a9f7;
    v8 = *__errno_location();
label_aaad:
    sub_9430(v1);
    v7 = a0->field_0x50;
    if (v7) goto label_aa0c;
  }
  else {
    if (!(a0->field_0x48 & 4)) {
      v2 = a0->field_0x28;
      if (fchdir(v2)) {
        v5 = __errno_location();
        v2 = a0->field_0x28;
        v8 = *v5;
        if (!close(v2)) goto label_aaad;
        if (!v8) goto label_aa68;
        sub_9430(v1);
        v7 = a0->field_0x50;
        if (!v7) {
          sub_a5c0(a0->field_0x48,a0->field_0x58);
          free(a0);
          goto label_aa31;
        }
        goto label_aa0c;
      }
      v2 = a0->field_0x28;
      if (close(v2)) {
        v5 = __errno_location();
label_aa68:
        v8 = *v5;
        goto label_aaad;
      }
    }
label_a9f7:
    v8 = 0;
    sub_9430(v1);
    v7 = a0->field_0x50;
    if (!v7) {
      sub_a5c0(a0->field_0x48,a0->field_0x58);
      free(a0);
      return 0;
    }
label_aa0c:
    sub_be30(v7);
  }
  sub_a5c0(a0->field_0x48,a0->field_0x58);
  free(a0);
  if (!v8)
    return 0;
  v5 = __errno_location();
label_aa31:
  *v5 = v8;
  return 0xffffffff;
}

// Function: sub_ab50 @ 0xab50
void * sub_ab50(struct_18 *a0)
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
    *(unsigned short *)((long)v7 + 0x68) = sub_9010(a0,v7,0);
    return v7;
  }
  v1 = *(short *)((long)v7 + 0x68);
  if (v5 != 2) { // branch-flip
    if (v1 != 1) { // branch-flip
label_abe0:
      do {
        v8 = *(void **)((long)v7 + 0x10);
        if (!v8) {
          v8 = *(void **)((long)v7 + 8);
          if (*(long *)((long)v8 + 0x18)) {
            v4 = *(long *)((long)v8 + 0x48);
            a0->field_0x0 = v8;
            *(char *)(a0->field_0x20 + v4) = 0;
            v8 = (void *)sub_9c70(a0,3);
            if (v8) {
              free(v7);
              goto label_ae22;
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
                    sub_9770(a0,v6,1);
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
                goto label_ac4b;
              }
              if (*(unsigned short *)((long)v8 + 0x6a) & 1) goto label_ac4b;
              v6 = sub_99c0(a0,*(long *)((long)v8 + 8),0xffffffff,"..");
            }
            else {
              v6 = sub_97d0(a0);
            }
            if (v6) {
              *(int *)((long)v8 + 0x40) = *__errno_location();
              a0->field_0x48 = a0->field_0x48 | 0x2000;
            }
label_ac4b:
            if (*(short *)((long)v8 + 0x68) != 2) {
              if (*(int *)((long)v8 + 0x40)) // branch-flip
                *(unsigned short *)((long)v8 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v8 + 0x68) = 6;
                sub_9840(a0,v8);
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
          if (sub_97d0(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          sub_a5c0(a0->field_0x48,a0->field_0x58);
          sub_9630(a0,v8);
          sub_9370(a0);
          v5 = *(short *)((long)v8 + 0x68);
          goto label_adc5;
        }
      } while (*(short *)((long)v8 + 0x6c) == 4);
      if (*(short *)((long)v8 + 0x6c) == 2) {
        v5 = sub_9010(a0,v8,1);
        *(short *)((long)v8 + 0x68) = v5;
        if ((v5 == 1) && (!(a0->field_0x48 & 4))) {
          v6 = sub_9980(a0->field_0x2c,a0->field_0x48,".");
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
      if (v5 == 4) goto label_ad4a;
label_acaf:
      if ((v2 & 0x40) && (*(long *)((long)v7 + 0x70) != a0->field_0x18)) {
label_ad4a:
        if (*(unsigned char *)((long)v7 + 0x6a) & 2)
          close(*(int *)((long)v7 + 0x44));
        if (a0->field_0x8) {
          sub_93e0(a0->field_0x8);
          a0->field_0x8 = NULL;
        }
        *(unsigned short *)((long)v7 + 0x68) = 6;
        sub_9840(a0,v7);
        return v7;
      }
      if (a0->field_0x8) { // branch-flip
        if (v2 & 0x1000) {
          a0->field_0x48 = v2 & 0xffffefff;
          sub_93e0(a0->field_0x8);
          a0->field_0x8 = NULL;
          goto label_aec0;
        }
        v3 = *(char **)((long)v7 + 0x30);
        if (sub_99c0(a0,v7,0xffffffff,v3)) { // branch-flip
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
label_aec0:
        v8 = (void *)sub_9c70(a0,3);
        a0->field_0x8 = v8;
        if (!v8) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)((long)v7 + 0x40)) && (*(short *)((long)v7 + 0x68) != 4))
              *(unsigned short *)((long)v7 + 0x68) = 7;
            sub_9840(a0,v7);
            return v7;
          }
          return NULL;
        }
      }
      a0->field_0x8 = NULL;
    }
label_ae22:
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
label_adcd:
    if (v5 == 0xb) goto label_ae71;
  }
  else {
    if (2 <= (unsigned short)(v1 - 0xcU)) {
      if (v1 == 1) goto label_acaf;
      goto label_abe0;
    }
    v5 = sub_9010(a0,v7,1);
    *(short *)((long)v7 + 0x68) = v5;
    if (v5 == 1) {
      if (a0->field_0x48 & 4) {
        a0->field_0x0 = v7;
        goto label_addd;
      }
      v6 = sub_9980(a0->field_0x2c,a0->field_0x48,".");
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
label_adc5:
      a0->field_0x0 = v7;
      goto label_adcd;
    }
    a0->field_0x0 = v7;
    if (v5 != 0xb)
      return v7;
label_ae71:
    if (*(long *)((long)v7 + 0xa0) != 2) {
      if (*(long *)((long)v7 + 0xa0) == 1)
        return v7;
      abort();
    }
    v5 = sub_9010(a0,v7,0);
    *(short *)((long)v7 + 0x68) = v5;
  }
  if (v5 != 1)
    return v7;
label_addd:
  if (!*(long *)((long)v7 + 0x58))
    a0->field_0x18 = *(long *)((long)v7 + 0x70);
  if (!sub_96c0(a0,v7)) {
    v7 = NULL;
    *__errno_location() = 0xc;
  }
  return v7;
}

// Function: sub_b190 @ 0xb190
unsigned long sub_b190(unsigned long a0,long a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)(a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_b1c0 @ 0xb1c0
int * sub_b1c0(struct_30 *a0,unsigned int a1) // early-return x2
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
    sub_93e0(a0->field_0x8);
  v6 = 1;
  if (a1 == 0x1000) {
    a0->field_0x48 = a0->field_0x48 | 0x1000;
    v6 = 2;
  }
  if (((*(long *)(v1 + 0x58)) || (**(char **)(v1 + 0x30) == '/')) || (a0->field_0x48 & 4)) {
    v5 = (void *)sub_9c70(a0,v6);
    a0->field_0x8 = v5;
  }
  else {
    v4 = sub_9980(a0->field_0x2c,a0->field_0x48,".");
    v2 = v4;
    if (v4 < 0) {
      a0->field_0x8 = NULL;
      v5 = NULL;
      return v5;
    }
    v5 = (void *)sub_9c70(a0,v6);
    a0->field_0x8 = v5;
    if (a0->field_0x48 & 0x200) // branch-flip
      sub_9770(a0,v2,1);
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

// Function: sub_b340 @ 0xb340
unsigned long sub_b340(unsigned long a0)
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

// Function: sub_b3b0 @ 0xb3b0
unsigned long sub_b3b0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_b340(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_b3f0 @ 0xb3f0
unsigned long sub_b3f0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_12dc0(a0,3) % a1;
}

// Function: sub_b410 @ 0xb410
bool sub_b410(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_b420 @ 0xb420
unsigned long sub_b420(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x15a40)
    return 1;
  v1 = v2[2];
  if ((((dat_15a54 < v1) && (v1 < dat_15a58)) && (dat_15a5c < v2[3])) && (dat_15a70 <= *v2)) {
    v3 = *v2 + dat_15a54;
    if (((v3 < v2[1]) && (v2[1] <= dat_15a60)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x15a40;
  return 0;
}

// Function: sub_b4b0 @ 0xb4b0
void sub_b4b0(long a0,struct_3 *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  *(struct_3 **)(a0 + 0x48) = a1;
}

// Function: sub_b4d0 @ 0xb4d0
long sub_b4d0(struct_4 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}

// Function: sub_b500 @ 0xb500
long sub_b500(long a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_b4d0(a0,a1);
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
      sub_b4b0(a0);
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
      sub_b4b0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_b600 @ 0xb600
void sub_b600(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_b630 @ 0xb630
unsigned long sub_b630(long a0,struct_31 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_6 *v3; // rax
  struct_6 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_b658:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_6 *)sub_b4d0(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_b4b0(a0);
          v6 = v1;
          if (!v1) goto label_b6c8;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_b6c8:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_b658;
    v3 = (struct_6 *)sub_b4d0(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_6 *)sub_b600(a0);
      if (!v4)
        return 0;
      v2 = v3->field_0x8;
      v4->field_0x0 = v7;
      v4->field_0x8 = v2;
      v3->field_0x8 = (long)v4;
    }
    else {
      v3->field_0x0 = v7;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1->field_0x18 = a1->field_0x18 + -1;
    if (a1->field_0x8 <= v5)
      return 1;
  } while( true );
}

// Function: sub_b760 @ 0xb760
unsigned long sub_b760(float a0,unsigned long a1,bool a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_15a64 <= a0)
      return 0;
    a1 = (a0 < dat_15a68) ? (unsigned long)a0 : (long)(a0 - dat_15a68) ^ 0x8000000000000000;
  }
  v1 = sub_b3b0(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_b7f0 @ 0xb7f0
unsigned long sub_b7f0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_b800 @ 0xb800
unsigned long sub_b800(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_b810 @ 0xb810
unsigned long sub_b810(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_b820 @ 0xb820
unsigned long sub_b820(struct_31 *a0)
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

// Function: sub_b880 @ 0xb880
bool sub_b880(struct_31 *a0)
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
label_b8a0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_b8a0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_b8f0 @ 0xb8f0
void sub_b8f0(long a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_b810(a0);
  v2 = sub_b7f0();
  v3 = sub_b800();
  v4 = sub_b820();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_15a70) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_ba10 @ 0xba10
long sub_ba10(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_b4d0(a0,a1);
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

// Function: sub_ba70 @ 0xba70
long sub_ba70(struct_31 *a0)
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

// Function: sub_bac0 @ 0xbac0
long sub_bac0(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_b4d0(a0,a1);
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

// Function: sub_bb30 @ 0xbb30
void sub_bb30(struct_31 *a0,long a1,unsigned long a2)
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

// Function: sub_bb80 @ 0xbb80
long sub_bb80(struct_31 *a0,void *a1,unsigned long a2)
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

// Function: sub_bc00 @ 0xbc00
unsigned long sub_bc00(unsigned char *a0,unsigned long a1)
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

// Function: sub_bc40 @ 0xbc40
void sub_bc40(struct_32 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_bc70 @ 0xbc70
unsigned long * sub_bc70(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_b3f0;
  if (!a3)
    a3 = sub_b410;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x15a40;
  v1[5] = a1;
  if (sub_b420(v1)) {
    v2 = sub_b760(*(float *)(a1 + 8),a0,(unsigned char)*(char *)(a1 + 0x10));
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

// Function: sub_bd80 @ 0xbd80
void sub_bd80(struct_33 *a0)
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
        if (a0->field_0x8 <= v5) goto label_be1b;
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
label_be1b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_be30 @ 0xbe30
void sub_be30(struct_11 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_bedc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_be9f;
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
label_be9f:
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
label_bedc:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_bf20 @ 0xbf20
unsigned int sub_bf20(struct_7 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_b760(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_b630(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_b630(a0,&v2,1)) && (sub_b630(a0,&v2,0))) {
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

// Function: sub_c0a0 @ 0xc0a0
unsigned long sub_c0a0(long a0,long a1,long *a2) // ternary
{
  long v1;
  struct_6 *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_b500(a0,a1,&v3,0);
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
      sub_b420(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_15a64 <= v5)
          return 0xffffffff;
        v4 = (dat_15a68 <= v5) ? (long)(v5 - dat_15a68) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_bf20(a0,v4))
          return 0xffffffff;
        if (sub_b500(a0,a1,&v3,0)) {
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
    v2 = (struct_6 *)sub_b600(a0);
    if (!v2)
      return 0xffffffff;
    v2->field_0x0 = a1;
    v2->field_0x8 = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}

// Function: sub_c290 @ 0xc290
unsigned long sub_c290(long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_c0a0(a0,a1,&v3);
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

// Function: sub_c2f0 @ 0xc2f0
long sub_c2f0(long a0,long a1) // ternary x2
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
  
  v5 = sub_b500(a0,a1,&v7,1);
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
      sub_b420(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_15a68 <= v8) ? (long)(v8 - dat_15a68) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_bf20(a0,v6)) {
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

// Function: sub_c480 @ 0xc480
long sub_c480(long a0)
{
  return sub_c2f0(a0); // tail-call
}

// Function: sub_c490 @ 0xc490
void sub_c490(void)
{
  return;
}

// Function: sub_c560 @ 0xc560
void * sub_c560(void *a0,unsigned long a1,unsigned char *a2,char *a3)
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

// Function: sub_c660 @ 0xc660
int sub_c660(void)
{
  return (-(unsigned int)(getenv("POSIXLY_CORRECT") == NULL) & 0x200) + 0x200;
}

// Function: sub_c690 @ 0xc690
unsigned long sub_c690(unsigned char *a0,unsigned long *a1,unsigned int *a2)
{
  unsigned char *v1;
  unsigned char v2;
  int v3; // eax
  unsigned long v4; // rax
  unsigned char *v5; // stack - 0x38
  unsigned int v6; // r13d
  
  if (((a0) || (a0 = (unsigned char *)getenv("BLOCK_SIZE"), a0)) || (a0 = (unsigned char *)getenv("BLOCKSIZE"), a0)) {
    v6 = 0;
    if (*a0 == 0x27) {
      a0 = &a0[1];
      v6 = 4;
    }
    v3 = sub_5ac0(a0,(long *)0x1ca20,(void *)0x15ab0,4);
    if (0 <= v3) { // branch-flip
      *a1 = 1;
      v6 |= *(unsigned int *)((long)v3 * 4 + 0x15ab0);
    }
    else {
      v4 = sub_120a0(a0,&v5,0,a1,"eEgGkKmMpPtTyYzZ0");
      if ((int)v4) {
        *a2 = 0;
        return v4;
      }
      v2 = *a0;
      while (10 <= (unsigned char)(v2 - 0x30)) {
        if (v5 == a0) {
          if (v5[-1] != 0x42) // branch-flip
            v6 |= 0x80;
          else {
            v6 |= 0x180;
            if (v5[-2] != 0x69) break;
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
    *a1 = sub_c660();
  }
  *a2 = v6;
  return 0;
}

// Function: sub_c800 @ 0xc800
char * sub_c800(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary
{
  unsigned int v1;
  char v10; // al
  unsigned int v11; // eax
  unsigned int v12; // eax
  unsigned int v13; // eax
  lconv *v14; // rax
  unsigned long v15;
  unsigned long v16;
  unsigned long v17;
  unsigned long v18;
  long v19; // rax
  unsigned char *v2;
  unsigned int v20; // edx
  unsigned int v21; // ebx
  char *v22;
  int v23; // esi
  unsigned int v24; // eax
  int v25;
  char *v26;
  unsigned long v27;
  char *v28;
  long double v29; // st0
  bool v3;
  long double v30;
  char *v31; // stack - 0x78
  unsigned int v32; // stack - 0x68
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  unsigned long v9;
  
  v11 = a2 & 3;
  v12 = a2 & 0x20;
  v14 = localeconv();
  v22 = *(char **)v14;
  v21 = (-(unsigned int)(v12 == 0) & 0xffffffe8) + 0x400;
  v15 = strlen(v22);
  if (0x10 <= v15 - 1) {
    v15 = 1;
    v22 = ".";
  }
  v2 = *(unsigned char **)&v14->field_0x10;
  v28 = *(char **)&v14->field_0x8;
  if (0x11 <= strlen(v28))
    v28 = "";
  v31 = &a1[0x287];
  v20 = (unsigned int)v15; // branch-flip
  v26 = v31;
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
      v16 = SUB168(v8._0_16_,0);
      if (!SUB168(v8._0_16_,8)) {
        v24 = 0;
        v18 = 0;
        v13 = 0;
        if (a2 & 0x10) goto label_cb60;
label_ccf5:
        v25 = -1;
label_cb6f:
        if (v11 != 1) goto label_cb7e;
        if (5 < (int)((((unsigned int)v16 & 1) + v24 != 0) + v13)) goto label_cb8f;
        goto label_cbb0;
      }
    }
label_c8da:
    v29 = (long double)(long)a3;
    if ((long)a3 < 0)
      v29 = v29 + (long double)dat_15a64;
    if (0 <= (long)a4) { // branch-flip
      v29 = v29 / (long double)(long)a4;
      if ((long)a0 > -1) goto label_c917;
label_ca00:
      v29 = ((long double)(long)a0 + (long double)dat_15a64) * v29;
      if (!(a2 & 0x10)) goto label_ca48;
label_c923:
      v25 = 0;
      v30 = (long double)(int)v21;
      do {
        v25 += 1;
        v30 = v30 * (long double)(int)v21;
        if (v29 < v30) break;
      } while (v25 != 8);
      v23 = 1;
      v22 = "%.1Lf";
      sub_c490(v11);
      __sprintf_chk(a1,v23,0xffffffffffffffff,v22);
      v17 = strlen(a1);
      if ((v17 <= v15 + 2 + (unsigned long)(v12 == 0)) && ((!(a2 & 8) || (a1[v17 - 1] != '0'))))
        v15 = v17 - (v15 + 1);
      else {
        v22 = "%.0Lf";
        v23 = 1;
        sub_c490(v11);
        __sprintf_chk(a1,v23,0xffffffffffffffff,v22);
        v17 = strlen(a1);
        v15 = v17;
      }
    }
    else {
      v29 = v29 / ((long double)(long)a4 + (long double)dat_15a64);
      if (0 > (long)a0) goto label_ca00;
label_c917:
      v29 = (long double)(long)a0 * v29;
      if (a2 & 0x10) goto label_c923;
label_ca48:
      v25 = 1;
      v22 = "%.0Lf";
      sub_c490(v11);
      __sprintf_chk(a1,v25,0xffffffffffffffff,v22);
      v17 = strlen(a1);
      v25 = -1;
      v15 = v17;
    }
    v22 = memmove(&v31[-v17],a1,v17);
    v26 = &v22[v15];
  }
  else {
    if ((!a3) || (v17 = a4 / a3, a4 % a3)) goto label_c8da;
    v16 = a0 / v17;
    v18 = (a0 % v17) * 10;
    v9 = v18 / v17;
    v27 = (v18 % v17) * 2;
    v8._8_8_ = v27;
    v8._0_8_ = v16;
    v18 = v9 & 0xffffffff;
    v13 = (unsigned int)v9;
    if (v17 <= v27) { // branch-flip
      v24 = 2;
      if (v17 < v27)
        v24 = 3;
    }
    else {
      v24 = (unsigned int)(v27 != 0);
    }
    if (!(a2 & 0x10)) goto label_ccf5;
label_cb60:
    v13 = (unsigned int)v18;
    v16 = SUB168(v8._0_16_,0);
    v27 = (unsigned long)v21;
    v25 = 0;
    v17 = v16;
    if (v16 < v27) goto label_cb6f;
    while( true ) {
      v16 = v17 / v27;
      v13 = (int)v18 + (int)(v17 % v27) * 10;
      v18 = (unsigned long)v13 / (unsigned long)v21;
      v1 = ((int)v24 >> 1) + (v13 % v21) * 2;
      v13 = (unsigned int)v18;
      v24 += v1;
      v24 = (v21 <= v1) ? (v21 < v24) + 2 : (unsigned int)(v24 != 0); // branch-flip
      v25 += 1;
      if (v16 < v27) break;
      v17 = v16;
      if (v25 == 8) goto label_cb6f;
    }
    if (10 <= v16) goto label_cb6f;
    v17 = v16;
    if (v11 != 1) { // branch-flip
      if ((!v11) && (v24)) goto label_ce72;
label_ceea:
      if (!v13) goto label_cef3;
      v10 = (char)v18 + '0';
    }
    else {
      if (3 > (v13 & 1) + v24) goto label_ceea;
label_ce72:
      v10 = (char)v18 + '1';
      if (v13 == 9) {
        v17 = v16 + 1;
        if (v16 == 9) goto label_cebc;
        v24 = 0;
        v16 = v17;
label_cef3:
        if (a2 & 8) goto label_cebe;
        v10 = '0';
        v17 = v16;
      }
    }
    a1[0x286] = v10;
    v18 = v15 & 0xffffffff;
    v26 = &a1[0x286 - v15];
    if (8 <= v20) { // branch-flip
      *(unsigned long *)v26 = *(unsigned long *)v22;
      *(unsigned long *)&v26[(v15 & 0xffffffff) - 8] = *(unsigned long *)&v22[(v15 & 0xffffffff) - 8];
      v19 = (long)v26 - ((unsigned long)&v26[8] & 0xfffffffffffffff8);
      v13 = (int)v19 + v20 & 0xfffffff8;
      if (8 <= v13) {
        v24 = 0;
        do {
          v18 = (unsigned long)v24;
          v24 += 8;
          *(unsigned long *)(((unsigned long)&v26[8] & 0xfffffffffffffff8) + v18) = *(unsigned long *)&v22[v18 - v19];
        } while (v24 < v13);
      }
    }
    else if (v15 & 4) { // branch-flip
      *(unsigned int *)v26 = *(unsigned int *)v22;
      *(unsigned int *)&v26[v18 - 4] = *(unsigned int *)&v22[v18 - 4];
    }
    else if (v20) {
      *v26 = *v22;
      if (v15 & 2)
        *(unsigned short *)&v26[v18 - 2] = *(unsigned short *)&v22[v18 - 2];
    }
label_cebc:
    v24 = 0;
    v16 = v17;
label_cebe:
    v13 = 0;
    if (v11 == 1) goto label_cbb0;
label_cb7e:
    if (v11) { // branch-flip
label_cbb0:
      v22 = v26;
      do {
        v22 = &v22[-1];
        *v22 = (char)v16 + (char)(v16 / 10) * '\xf6' + '0';
        v3 = 10 <= v16;
        v16 /= 10;
      } while (v3);
    }
    else {
      if (1 > (int)(v24 + v13)) goto label_cbb0;
label_cb8f:
      v32 = a2 & 0x10;
      v16 += 1;
      if (((!v32) || (v21 != v16)) || (v25 == 8)) goto label_cbb0;
      v25 += 1;
      if (!(a2 & 8)) {
        v26[-1] = '0';
        v26 = &v26[~v15];
        v17 = v15 & 0xffffffff;
        if (8 <= v20) { // branch-flip
          *(unsigned long *)v26 = *(unsigned long *)v22;
          *(unsigned long *)&v26[(v15 & 0xffffffff) - 8] = *(unsigned long *)&v22[(v15 & 0xffffffff) - 8];
          v19 = (long)v26 - ((unsigned long)&v26[8] & 0xfffffffffffffff8);
          v11 = (int)v19 + v20 & 0xfffffff8;
          if (8 <= v11) {
            v20 = 0;
            do {
              v15 = (unsigned long)v20;
              v20 += 8;
              *(unsigned long *)(((unsigned long)&v26[8] & 0xfffffffffffffff8) + v15) = *(unsigned long *)&v22[v15 - v19];
            } while (v20 < v11);
          }
        }
        else if (v15 & 4) { // branch-flip
          *(unsigned int *)v26 = *(unsigned int *)v22;
          *(unsigned int *)&v26[v17 - 4] = *(unsigned int *)&v22[v17 - 4];
        }
        else if ((v20) && (*v26 = *v22, v15 & 2))
          *(unsigned short *)&v26[v17 - 2] = *(unsigned short *)&v22[v17 - 2];
      }
      v26[-1] = '1';
      v22 = &v26[-1];
    }
  }
  if (a2 & 4)
    v22 = (char *)sub_c560(v22,(long)v26 - (long)v22,v2,v28);
  if (!(a2 & 0x80)) goto label_cade;
  if (v25 != -1) { // branch-flip
label_cacb:
    if (!(a2 & 0x100) && !v25) goto label_cade;
    if (a2 & 0x40) {
label_cd41:
      v31 = &a1[0x288];
      a1[0x287] = ' ';
    }
    if (v25) goto label_cd5f;
    v28 = v31;
    if (!(a2 & 0x100)) goto label_cade;
  }
  else {
    if (a4 < 2) {
      v25 = 0;
      goto label_cacb;
    }
    v25 = 1;
    v15 = 1;
    do {
      v15 *= v21;
      if (a4 <= v15) break;
      v25 += 1;
    } while (v25 != 8);
    if (a2 & 0x40) goto label_cd41;
label_cd5f:
    v28 = &v31[1];
    if ((v12) || (v25 != 1)) {
      *v31 = *(char *)((long)v25 + 0x15ab8);
      if (!(a2 & 0x100)) {
label_ce3b:
        v31 = v28;
        goto label_cade;
      }
      if (v12) {
        v31[1] = 'i';
        v28 = &v31[2];
      }
    }
    else {
      *v31 = 'k';
      if (!(a2 & 0x100)) goto label_ce3b;
    }
  }
  *v28 = 'B';
  v31 = &v28[1];
label_cade:
  *v31 = '\0';
  return v22;
}

// Function: sub_d070 @ 0xd070
unsigned long sub_d070(unsigned char *a0,unsigned int *a1,long *a2)
{
  unsigned long v1; // rax
  
  v1 = sub_c690(a0,a2,a1);
  if (*a2)
    return v1;
  *a2 = sub_c660();
  return 4;
}

// Function: sub_d0a0 @ 0xd0a0
void sub_d0a0(struct_21 *a0,unsigned int a1)
{
  *(unsigned long *)a0->field_0x14 = 0;
  a0->field_0x1c = 1;
  a0->field_0x0 = a1;
  a0->field_0x4 = a1;
  a0->field_0x8 = a1;
  a0->field_0xc = a1;
  a0->field_0x10 = a1;
}

// Function: sub_d0c0 @ 0xd0c0
char sub_d0c0(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_d0d0 @ 0xd0d0
unsigned int sub_d0d0(long a0,unsigned int a1)
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

// Function: sub_d110 @ 0xd110
unsigned int sub_d110(long a0)
{
  unsigned int v1;
  unsigned int v2;
  char v3; // al
  
  v3 = sub_d0c0(a0);
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

// Function: sub_d170 @ 0xd170
unsigned long sub_d170(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_d180 @ 0xd180
unsigned long sub_d180(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_d190 @ 0xd190
long * sub_d190(long a0)
{
  long *v1; // rax
  long v2; // rax
  long *v3;
  
  v1 = malloc(0x18);
  v3 = v1;
  if (v1) {
    v2 = sub_bc70(0x3fd,0,sub_d170,sub_d180,dat_1cfb0);
    *v1 = v2;
    if (v2) { // branch-flip
      v1[1] = a0;
      v1[2] = 0;
    }
    else {
      v3 = NULL;
      free(v1);
    }
  }
  return v3;
}

// Function: sub_d200 @ 0xd200
void sub_d200(struct_26 *a0)
{
  sub_be30(a0->field_0x0);
  free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_d230 @ 0xd230
long sub_d230(long *a0,long a1)
{
  long v1;
  struct_9 *v2; // rax
  struct_9 *v3;
  
  v3 = (struct_9 *)a0[2];
  if (v3) { // branch-flip
    if (*(long *)v3->field_0x0 == a1)
      return v3->field_0x8;
  }
  else {
    v3 = malloc(0x10);
    a0[2] = (long)v3;
    if (!v3)
      return -1;
  }
  *(long *)v3->field_0x0 = a1;
  v2 = (struct_9 *)sub_c290(*a0,v3);
  if (!v2)
    return -1;
  if (v3 == v2) {
    v1 = a0[1];
    a0[2] = 0;
    a0[1] = v1 + 1;
    v3->field_0x8 = v1;
    return v1;
  }
  v1 = v2->field_0x8;
  v3->field_0x8 = v1;
  return v1;
}

// Function: sub_d2d0 @ 0xd2d0
char * sub_d2d0(unsigned long a0,long a1)
{
  unsigned long v1;
  char *v2;
  char *v3; // r8
  
  *(char *)(a1 + 0x14) = 0;
  v2 = (char *)(a1 + 0x14);
  if ((long)a0 <= -1) {
    do {
      v3 = v2;
      v1 = (long)a0 / 10;
      v3[-1] = ((char)v1 * '\n' + '0') - (char)a0;
      a0 = v1;
      v2 = &v3[-1];
    } while (v1);
    v3[-2] = '-';
    return &v3[-2];
  }
  do {
    v2 = &v2[-1];
    v1 = a0 / 10;
    *v2 = (char)a0 + (char)v1 * '\xf6' + '0';
    a0 = v1;
  } while (v1);
  return v2;
}

// Function: sub_d370 @ 0xd370
unsigned long sub_d370(unsigned char *a0,unsigned char *a1) // early-return x3
{
  unsigned char v1;
  unsigned char *v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  char v12; // stack - 0x98
  unsigned int v13; // stack - 0x94
  unsigned long v14; // stack - 0x74
  char v15; // stack - 0x6c
  unsigned char *v16; // stack - 0x68
  unsigned long v17; // stack - 0x60
  char v18; // stack - 0x58
  unsigned int v19; // stack - 0x54
  long v2;
  unsigned int v3;
  int v4; // eax
  unsigned char v5;
  char v6 [4]; // stack - 0xb8
  char v7 [4]; // stack - 0x78
  unsigned long v8; // stack - 0xb4
  char v9; // stack - 0xac
  
  if (a1 == a0)
    return 0;
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v6[0] = 0;
    v8 = 0;
    v7[0] = 0;
    v14 = 0;
    v10 = a0;
    v16 = a1;
    while( true ) {
      v9 = 0;
      v15 = 0;
      sub_d5c0(v6);
      if ((v12) && (!v13)) break;
      sub_d5c0(v7);
      if (v18) { // branch-flip
        if (!v19) break;
        if (!v12)
          return 1;
        v3 = towlower(v13);
        v3 -= towlower(v19);
        if (v3)
          return (unsigned long)v3;
      }
      else {
        if (v12)
          return 0xffffffff;
        if (v11 != v17) {
          if (v11 < v17) {
            if (memcmp(v10,v16,v11) <= 0)
              return 0xffffffff;
            return 1;
          }
          if (0 <= memcmp(v10,v16,v17))
            return 1;
          return 0xffffffff;
        }
        v4 = memcmp(v10,v16,v11);
        if (v4)
          return CONCAT44(dat_4,v4);
      }
      v10 = &v10[v11];
      v16 = &v16[v17];
    }
    sub_d5c0(v6);
    if ((!v12) || (v13))
      return 1;
    sub_d5c0(v7);
    if ((!v18) || (v3 = 0, v19))
      return 0xffffffff;
  }
  else {
    v2 = *(long *)__ctype_b_loc();
    do {
      v5 = *a0;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v5 * 2) & 1)
        v5 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v5 * 4);
      v1 = *a1;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v1 * 2) & 1)
        v1 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v1 * 4);
      if (!v5)
        return (unsigned long)-(unsigned int)v1;
      a0 = &a0[1];
      a1 = &a1[1];
    } while (v5 == v1);
    v3 = (unsigned int)v5 - (unsigned int)v1;
  }
  return (unsigned long)v3;
}

// Function: sub_d5c0 @ 0xd5c0
void sub_d5c0(struct_12 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v2 = a0->field_0x10;
    v1 = *v2;
    if (sub_13590((int)v1)) {
      a0->field_0x18 = 1;
      v1 = *v2;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)v1;
      goto label_d602;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","lib/mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_135b0(&a0->field_0x24,v2,sub_108f0(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
  a0->field_0x18 = v4;
  if (v4 == -1) {
    a0->field_0x18 = 1;
    a0->field_0x20 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
  if (v4 != -2) { // branch-flip
    if (!v4) {
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
    v3 = strlen(a0->field_0x10);
    a0->field_0x20 = '\0';
    a0->field_0x18 = v3;
  }
label_d602:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_d740 @ 0xd740
void sub_d740(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_d750 @ 0xd750
void sub_d750(struct_36 *a0,struct_36 *a1)
{
  char v1;
  
  v1 = a1->field_0x0;
  a0->field_0x0 = v1;
  if (v1) // branch-flip
    *(unsigned long *)&a0->field_0x1[3] = *(unsigned long *)&a1->field_0x1[3];
  else {
    *(unsigned long *)&a0->field_0x1[3] = 0;
  }
  a0->field_0xc = a1->field_0xc;
  sub_13530((struct_34 *)&a0[1].field_0x1[2],(struct_35 *)&a1[1].field_0x1[2]); // tail-call
}

// Function: sub_d790 @ 0xd790
void sub_d790(char *a0) // return-dupe
{
  char *v1; // rax
  
  v1 = strchr(a0,0x20);
  if (!v1)
    return;
  *v1 = '\0';
}

// Function: sub_d7b0 @ 0xd7b0
void sub_d7b0(char *a0)
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

// Function: sub_d850 @ 0xd850
unsigned long sub_d850(void)
{
  return 0xffffffffffffffff;
}

// Function: sub_d860 @ 0xd860
void sub_d860(struct_10 *a0)
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

// Function: sub_d8b0 @ 0xd8b0
long sub_d8b0(void)
{
  char *v1;
  unsigned long *v10; // rax
  unsigned long v11; // rax
  long v12; // stack - 0x58
  unsigned long v13; // stack - 0x48
  char *v14; // stack - 0x50
  unsigned int v15; // stack - 0x60
  unsigned int v16; // stack - 0x64
  int v17; // stack - 0x5c
  char *v18;
  bool v19; // dl
  int v2;
  bool v20; // r12b
  long *v21; // stack - 0x88
  FILE *v3; // rax
  long v4;
  char *v5; // rax
  unsigned long *v6; // rax
  int *v7; // rax
  void *v8; // rax
  char *v9;
  
  v3 = fopen("/proc/self/mountinfo","re");
  if (v3) { // branch-flip
    v21 = &v12;
    v14 = NULL;
    v13 = 0;
    while (getline(&v14,&v13,v3) != -1) {
      if ((unsigned int)(__isoc99_sscanf(v14,"%*u %*u %u:%u %n",&v16,&v15,&v17) - 2U) <= 1) {
        v18 = &v14[v17];
        v4 = sub_d790(v18);
        if (v4) {
          v9 = (char *)(v4 + 1);
          v4 = sub_d790(v9);
          if ((v4) && (v5 = strstr((char *)(v4 + 1)," - "), v5)) {
            v5 = &v5[3];
            v4 = sub_d790(v5);
            if (v4) {
              v1 = (char *)(v4 + 1);
              if (sub_d790(v1)) {
                sub_d7b0(v1);
                sub_d7b0(v9);
                sub_d7b0(v18);
                sub_d7b0(v5);
                v6 = (unsigned long *)sub_11680(0x38);
                *v6 = sub_11a20(v1);
                v6[1] = sub_11a20(v9);
                v6[2] = sub_11a20(v18);
                v18 = (char *)sub_11a20(v5);
                *(unsigned char *)&v6[5] = *(unsigned char *)&v6[5] | 4;
                v6[3] = v18;
                v6[4] = gnu_dev_makedev(v16,v15);
                v19 = 1;
                if (strcmp(v18,"autofs")) {
                  v19 = 1;
                  if (strcmp(v18,"proc")) {
                    v19 = 1;
                    if (strcmp(v18,"subfs")) {
                      v19 = 1;
                      if (strcmp(v18,"debugfs")) {
                        v19 = 1;
                        if (strcmp(v18,"devpts")) {
                          v19 = 1;
                          if (strcmp(v18,"fusectl")) {
                            v19 = 1;
                            if (strcmp(v18,"fuse.portal")) {
                              v19 = 1;
                              if (strcmp(v18,"mqueue")) {
                                v19 = 1;
                                if (strcmp(v18,"rpc_pipefs")) {
                                  v19 = 1;
                                  if (strcmp(v18,"sysfs")) {
                                    v19 = 1;
                                    if (strcmp(v18,"devfs")) {
                                      v19 = 1;
                                      if (strcmp(v18,"kernfs")) {
                                        v19 = 1;
                                        if (strcmp(v18,"ignore"))
                                          v19 = strcmp(v18,"none") == 0;
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
                *(unsigned char *)&v6[5] = *(unsigned char *)&v6[5] & 0xfe | v19;
                v9 = (char *)*v6;
                v19 = 1;
                if (!strchr(v9,0x3a)) {
                  if ((*v9 != '/') || (v9[1] != '/')) {
label_dcb6:
                    v19 = 1;
                    if (strcmp(v18,"acfs")) {
                      v19 = 1;
                      if (strcmp(v18,"afs")) {
                        v19 = 1;
                        if (strcmp(v18,"coda")) {
                          v19 = 1;
                          if (strcmp(v18,"auristorfs")) {
                            v19 = 1;
                            if (strcmp(v18,"fhgfs")) {
                              v19 = 1;
                              if (strcmp(v18,"gpfs")) {
                                v19 = 1;
                                if (strcmp(v18,"ibrix")) {
                                  v19 = 1;
                                  if (strcmp(v18,"ocfs2")) {
                                    v19 = 1;
                                    if (strcmp(v18,"vxfs"))
                                      v19 = strcmp("-hosts",v9) == 0;
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
                    if (strcmp(v18,"smbfs")) {
                      v19 = 1;
                      if (strcmp(v18,"smb3")) {
                        v19 = 1;
                        if (strcmp(v18,"cifs")) goto label_dcb6;
                      }
                    }
                  }
                }
                *(unsigned char *)&v6[5] = *(unsigned char *)&v6[5] & 0xfd | v19 * '\x02';
                *v21 = (long)v6;
                v21 = &v6[6];
              }
            }
          }
        }
      }
    }
    free(v14);
    if (ferror_unlocked(v3)) {
      v7 = __errno_location();
      v2 = *v7;
      sub_70c0(v3);
      *v7 = v2;
      goto label_ddef;
    }
    if (sub_70c0(v3) != -1) goto label_dafb;
  }
  else {
    v8 = setmntent("/etc/mtab","r");
    if (!v8)
      return 0;
    v21 = &v12;
    while (v6 = getmntent(v8), v6) {
      v19 = 1;
      v9 = hasmntopt(v6,"bind");
      v10 = (unsigned long *)sub_11680(0x38);
      v11 = sub_11a20((char *)*v6);
      v18 = (char *)v6[1];
      *v10 = v11;
      v11 = sub_11a20(v18);
      v10[2] = 0;
      v18 = (char *)v6[2];
      v10[1] = v11;
      v18 = (char *)sub_11a20(v18);
      *(unsigned char *)&v10[5] = *(unsigned char *)&v10[5] | 4;
      v10[3] = v18;
      if ((((((strcmp(v18,"autofs")) && (strcmp(v18,"proc"))) && (strcmp(v18,"subfs"))) && ((strcmp(v18,"debugfs") && (strcmp(v18,"devpts"))))) && (strcmp(v18,"fusectl"))) && ((((strcmp(v18,"fuse.portal") && (strcmp(v18,"mqueue"))) && ((strcmp(v18,"rpc_pipefs") && (((strcmp(v18,"sysfs") && (strcmp(v18,"devfs"))) && (strcmp(v18,"kernfs"))))))) && (strcmp(v18,"ignore"))))) {
        v2 = strcmp(v18,"none");
        v19 = !v9 && !v2;
      }
      v9 = (char *)*v10;
      v20 = 1;
      *(unsigned char *)&v10[5] = *(unsigned char *)&v10[5] & 0xfe | v19;
      if ((!strchr(v9,0x3a)) && (((*v9 != '/' || (v9[1] != '/')) || ((strcmp(v18,"smbfs") && ((strcmp(v18,"smb3") && (strcmp(v18,"cifs"))))))))) {
        v20 = 1;
        if ((strcmp(v18,"acfs")) && ((((strcmp(v18,"afs") && (strcmp(v18,"coda"))) && (strcmp(v18,"auristorfs"))) && (((strcmp(v18,"fhgfs") && (strcmp(v18,"gpfs"))) && ((strcmp(v18,"ibrix") && ((strcmp(v18,"ocfs2") && (strcmp(v18,"vxfs")))))))))))
          v20 = strcmp("-hosts",v9) == 0;
      }
      *(unsigned char *)&v10[5] = *(unsigned char *)&v10[5] & 0xfd | v20 * '\x02';
      v10[4] = sub_d850();
      *v21 = (long)v10;
      v21 = &v10[6];
    }
    if (endmntent(v8)) {
label_dafb:
      *v21 = 0;
      return v12;
    }
  }
  v7 = __errno_location();
  v2 = *v7;
label_ddef:
  *v21 = 0;
  while (v12) {
    v4 = *(long *)(v12 + 0x30);
    sub_d860(v12);
    v12 = v4;
  }
  *v7 = v2;
  return 0;
}

// Function: sub_e230 @ 0xe230
void sub_e230(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_10f70(openat(a0,a1,a2,v1));
}

// Function: sub_e2b0 @ 0xe2b0
DIR * sub_e2b0(unsigned long a0,char *a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_e230(a0,a1,a2 | 0x90900);
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

// Function: sub_e310 @ 0xe310
void sub_e310(char *a0)
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
    dat_1d1b0 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_e3b0 @ 0xe3b0
struct_19 * sub_e3b0(struct_19 *a0,int a1)
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

// Function: sub_e400 @ 0xe400
bool sub_e400(long a0,long a1)
{
  return sub_12fe0((char *)(a0 + 9),(char *)(a1 + 9)) == 0;
}

// Function: sub_e420 @ 0xe420
unsigned long sub_e420(long a0,long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_12f60((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_e400(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_e480 @ 0xe480
unsigned long sub_e480(long a0,long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_12f60((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_e420(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_e500 @ 0xe500
unsigned long sub_e500(long a0,long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_12f60((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_e480(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_e570 @ 0xe570
unsigned long sub_e570(long a0,long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_12f60((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_e500(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_e610 @ 0xe610
unsigned long sub_e610(long a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_12f60((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_e570(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_e6b0 @ 0xe6b0
unsigned long sub_e6b0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_12f60((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_e610(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_e770 @ 0xe770
unsigned long sub_e770(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_12f60((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_e6b0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_e830 @ 0xe830
unsigned long sub_e830(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_12f60((int)(char)a2)) // branch-flip
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
      return sub_e770(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_e910 @ 0xe910
unsigned long sub_e910(unsigned char *a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_12f60((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_e830(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_e9d0 @ 0xe9d0
char * sub_e9d0(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned char *v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = (unsigned char *)sub_134c0();
    if (sub_e910(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_e910(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x15c98 : (char *)0x15c9d; // branch-flip
    }
  }
  return v1;
}

// Function: sub_eb00 @ 0xeb00
long * sub_eb00(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_eb70:
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
label_eff6:
      v2 = 0;
      goto label_efb0;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_f111;
      }
      v23 = 1;
      v2 = 0;
      v12 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v2 = 1;
label_efb0:
      v23 = 1;
      v12 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_f111;
      }
      goto label_eff6;
    case 5:
      if (!v17) goto label_ef51;
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
        v9 = (char *)sub_e9d0("`",a4);
        v35 = (char *)v9;
        v9 = (char *)sub_e9d0("\'",a4);
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
label_ec33:
  do {
label_ec46:
    v22 = 0;
    a1 = v13;
label_ec50:
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
          if ((a3 < v22 + v29) || (v8 = memcmp(v11,v30,v29), v8)) goto label_f5c0;
          if (!v23) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v20 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_f1bc;
                default:
                  goto label_ee93;
                case 7:
                  goto label_f180;
                case 8:
                  goto label_f165;
                case 9:
                  goto label_f243;
                case 10:
                  goto label_f15b;
                case 0xb:
                  goto label_f270;
                case 0xc:
                  goto label_f1a7;
                case 0xd:
                  goto label_f0ad;
                case 0x20:
                  goto label_f27d;
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
                  goto label_ee46;
                case 0x23:
                  goto label_f220;
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
                  goto label_ed78;
                case 0x27:
                  goto label_f092;
                case 0x3f:
                  goto label_f051;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_ee93;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_ed78;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_f664;
              goto label_f5a8;
            }
            if (v7 == 0x7d) goto label_f730;
            if ((char)v7 <= '}') {
              v20 = 0x7b;
              if (v7 == 0x7b) goto label_f58a;
              v20 = 0x7c;
              if (v7 == 0x7c) goto label_f5a8;
              goto label_ee93;
            }
            if (v7 != 0x7e) goto label_ee93;
label_f6e4:
            if (!v22) {
              v7 = a4 == 2;
              v20 = 0x7e;
              v25 = v3;
              goto label_ef09;
            }
            v20 = 0x7e;
            goto label_f6f2;
          }
          goto label_ee7c;
        }
label_f5c0:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v20 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v21 = 0;
                if (v7 == 0x7e) goto label_f6e4;
                goto label_ee90;
              }
              v20 = 0x7b;
              v24 = 0;
              if (v7 != 0x7b) {
                v20 = 0x7c;
                v24 = 0;
                v21 = 0;
                if (v7 == 0x7c) goto label_f5a8;
                goto label_ee93;
              }
            }
            else {
              v20 = 0x7d;
              v24 = 0;
            }
label_f58a:
            if (a3 == 0xffffffffffffffff) goto label_f73f;
label_f598:
            v21 = v24;
            if (a3 == 1) goto label_f225;
          }
          else {
            if (v7 == 0x40) goto label_ee90;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v21 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_ed78;
            v24 = 0;
            v21 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_f664:
              if ((char)v18 != '\\') goto label_ee93;
              if (((bool)(v2 & v23)) && (v29)) goto label_f68a;
              v16 = 0x5c;
              goto label_f0b2;
            }
          }
label_f5a8:
          v7 = a4 == 2;
          v25 = v3;
          v24 = 0;
          goto label_ef09;
        }
        switch(v7) {
          case 0:
            goto label_f1ae;
          default:
label_ee90:
            v24 = 0;
label_ee93:
            v16 = (unsigned char)v18;
            v21 = v24;
            if (v32 != 1) goto label_f413;
label_eea0:
            v9 = __ctype_b_loc();
            v20 = 1;
            v25 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v26 = (bool)(v25 & v2);
            v25 = !v25;
            v21 = v24;
            goto label_eeee;
          case 7:
label_f180:
            v16 = 0x61;
            if (!v23) goto label_f178;
            goto label_f0df;
          case 8:
label_f165:
            v16 = 0x62;
            goto label_f16a;
          case 9:
            v24 = 0;
label_f243:
            v20 = 9;
            v16 = 0x74;
            goto label_f250;
          case 10:
label_f15b:
            v16 = 0x6e;
            break;
          case 0xb:
label_f270:
            v16 = 0x76;
            break;
          case 0xc:
label_f1a7:
            v16 = 0x66;
label_f16a:
            if (v23) goto label_f0df;
label_f178:
            v24 = 0;
            goto label_edfe;
          case 0xd:
label_f0ad:
            v16 = 0x72;
            break;
          case 0x20:
            v21 = 0;
label_f27d:
            v20 = 0x20;
            goto label_ed78;
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
            goto label_ee49;
          case 0x23:
            v20 = 0x23;
            v24 = 0;
            goto label_f225;
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
label_ed78:
            v7 = 0;
            goto label_ed88;
          case 0x27:
            v21 = 0;
            goto label_f092;
          case 0x3f:
            v21 = 0;
            goto label_f051;
          
        }
        goto label_f0b2;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_f730:
            v20 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_f598;
label_f73f:
            v21 = v24;
            if (a2[1]) goto label_f5a8;
            goto label_f225;
          }
          if ((char)v16 <= '}') {
            v20 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_ee46;
              goto label_ee93;
            }
            goto label_f58a;
          }
          v20 = 0x7e;
          if (v16 == 0x7e) goto label_f225;
          if (v32 == 1) goto label_eea0;
label_f413:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v20 = 0;
          do {
            v18 = v22 + v20;
            v9 = (long)sub_135b0(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v24 = v25;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_f97b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_fcf5;
              goto label_fcff;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_f54d;
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
          goto label_f97b;
        }
        if (v16 == 0x40) goto label_ee93;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_ef00;
        if ((unsigned long)v9 & 0xa4000000) goto label_ee46;
        if (v16 != 0x5c) goto label_ee93;
        if (a4 == 2) {
          if (!v23) goto label_f68a;
          goto label_ee73;
        }
        if (!(bool)(v2 & v23 & v29 != 0)) {
          v20 = 0x5c;
          v16 = 0x5c;
          v24 = 0;
          goto label_f250;
        }
label_f68a:
        v22 += 1;
        v24 = 0;
        v16 = 0x5c;
        v7 = v19;
        goto label_f6a0;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v21 = 0;
            v20 = 0;
            if (v33 & 1) goto label_f322;
            goto label_f25b;
          }
label_f1ae:
          if (!v23) {
            v21 = 0;
label_f1bc:
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
                goto label_ef33;
              }
label_fac5:
              a0[v13] = 0x5c;
              v12 = v13;
              v19 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v19;
              goto label_fac5;
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
label_fafc:
              v16 = (unsigned char)v20;
              v24 = 0;
              v12 = v13;
              v7 = v25;
              if (!v2 || v26) goto label_ef33;
              goto label_ed88;
            }
            v16 = 0x30;
            v24 = 0;
            v12 = v13;
            v7 = v25;
            goto label_ef33;
          }
          if (a4 != 2) goto label_ee7c;
          goto label_f0eb;
        default:
          goto label_ee93;
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
          goto label_f2df;
        case 10:
          v20 = 10;
          v16 = 0x6e;
          goto label_f2df;
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
label_f2df:
          v7 = a4 == 2 & v23;
          v24 = 0;
          if (!(bool)v7) goto label_f250;
label_ee61:
          a4 = 2;
label_ee73:
          if (!v27) goto label_ee7c;
          goto label_f0eb;
        case 0x20:
          v18 = 0x20;
          goto label_ee49;
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
label_ee46:
          v25 = 0;
          goto label_ee49;
        case 0x23:
label_f220:
          v20 = 0x23;
label_f225:
          v21 = v24;
          if (v22) {
label_f6f2:
            v7 = a4 == 2;
            v25 = v3;
            v24 = 0;
            goto label_ef09;
          }
          v18 = v20;
label_ee49:
          v20 = v18;
          v21 = v24;
          v24 = v25;
          if ((a4 == 2) && (v23)) goto label_ee61;
          goto label_ef07;
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
          goto label_ef00;
        case 0x27:
label_f092:
          v24 = v25;
          if (a4 != 2) {
            v7 = 0;
            v20 = 0x27;
            goto label_ef09;
          }
          if (!v23) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v31) goto label_fb1b;
            }
            else {
label_fb1b:
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
            goto label_ef33;
          }
          goto label_ee73;
        case 0x3f:
label_f051:
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
                  goto label_ef09;
                }
                if (v23) goto label_ee7c;
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
                goto label_fafc;
              }
            }
            v7 = 0;
            v20 = 0x3f;
            v25 = v3;
            v24 = 0;
            goto label_ef09;
          }
          if (v23) goto label_ee73;
          v7 = 0;
          v16 = 0x3f;
          v24 = 0;
          goto label_ef33;
        
      }
label_f250:
      v21 = v24;
      if (!v2) {
label_f25b:
        v16 = (unsigned char)v20;
        v24 = 0;
        v7 = 0;
        v25 = v3;
        if (!v23) goto label_ef30;
        goto label_ed88;
      }
label_f0b2:
      v24 = 0;
      goto label_f0c3;
    }
    if (!(bool)(v12 == 0 & v23 & a4 == 2)) {
      v7 = a4 == 2 & (v23 ^ 1U);
      v23 = (bool)(v23 ^ 1U);
      if ((!(bool)v7) || (v23 = (bool)v7, !v3)) {
label_fd7f:
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
        if (a1 || !v31) goto label_fd7f;
        v22 = v31;
        goto label_f84b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 1;
      v32 = (unsigned long)v9;
      v30 = "\"";
      if (!(v33 & 2)) goto label_fe2f;
      v3 = 0;
      v19 = 0;
      v31 = 0;
      v2 = v4;
      v23 = v4;
      goto label_ec46;
    }
label_f54d:
    if (v2) {
label_f0eb:
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
label_f111:
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
label_f84b:
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
label_fcf5:
    if (!a2[(long)v9]) break;
  }
label_fcff:
  v24 = 0;
label_f97b:
  v18 = (unsigned long)v16;
  v25 = v24;
  if (2 <= v20) {
label_f985:
    v25 = 0;
    v20 += v22;
    v13 = v22;
    do {
      v16 = (unsigned char)v18;
      if (v26) {
        v27 = a4 == 2;
        if (v23) goto label_ee73;
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
        if (v20 <= v22) goto label_ee11;
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
          goto label_f6a0;
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
label_eeee:
  if (v26) {
    v24 = 0;
    v26 = v2;
    goto label_f985;
  }
label_ef00:
  v20 = v18;
  v24 = v25;
label_ef07:
  v7 = a4 == 2;
  v25 = v3;
label_ef09:
  v16 = (unsigned char)v20;
  v26 = (bool)v7 == 0;
  v3 = v25;
  v7 = 0;
  if ((v26 && v2) || (v7 = 0, v23)) {
label_ed88:
    v16 = (unsigned char)v20;
    v12 = v13;
    if (!v28) goto label_ef33;
    if (!(*(unsigned int *)(v28 + (v20 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_ef33;
  }
  else {
label_ef30:
    v7 = 0;
    v3 = v25;
label_ef33:
    if (!v21) {
      v7 ^= 1;
      v22 += 1;
      v7 &= v19;
label_f6a0:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v19 = 0;
        v12 += 2;
      }
      goto label_ee11;
    }
  }
label_f0c3:
  if (v23) {
    v23 = v2;
label_f0df:
    if ((bool)(v23 & a4 == 2)) goto label_f0eb;
label_ee7c:
    v33 &= 0xfffffffd;
    v28 = 0;
    goto label_eb70;
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
label_edfe:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v22 += 1;
label_ee11:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v24)
    v4 = 0;
  goto label_ec50;
label_f322:
  v22 += 1;
  goto label_ec50;
label_fe2f:
  a1 = v31;
label_ef51:
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
  goto label_ec33;
}

// Function: sub_ff30 @ 0xff30
char * sub_ff30(unsigned int a0,char *a1,unsigned long a2,struct_2 *a3)
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
    v5 = dat_1d080;
    if (dat_1d078 <= (int)a0) {
      v10 = (long)dat_1d078;
      v8 = (long)(int)((a0 - dat_1d078) + 1);
      if (dat_1d080 != (unsigned long *)0x1d090) // branch-flip
        v5 = (unsigned long *)sub_11840(dat_1d080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_11840(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_1d090;
        v5[1] = dat_1d098;
      }
      dat_1d080 = v5;
      memset(&v5[(long)dat_1d078 * 2],0,(v10 - dat_1d078) * 0x10);
      dat_1d078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_eb00(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x1d1c0)
        free(v7);
      v7 = (char *)sub_116c0(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_eb00(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_10120 @ 0x10120
void sub_10120(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x1d2c0;
  sub_11980(a0,0x38);
  *v2 = v1;
}

// Function: sub_10160 @ 0x10160
unsigned int sub_10160(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x1d2c0;
  return *a0;
}

// Function: sub_10180 @ 0x10180
void sub_10180(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x1d2c0;
  *a0 = a1;
}

// Function: sub_101a0 @ 0x101a0
unsigned int sub_101a0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x1d2c0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_101e0 @ 0x101e0
unsigned int sub_101e0(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x1d2c0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_10200 @ 0x10200
void sub_10200(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x1d2c0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_10240 @ 0x10240
void sub_10240(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x1d2c0;
  v2 = __errno_location();
  v1 = *v2;
  sub_eb00(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_102c0 @ 0x102c0
char * sub_102c0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x1d2c0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_eb00(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_116c0(v3 + 1U);
  sub_eb00(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_103b0 @ 0x103b0
char * sub_103b0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_102c0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_103c0 @ 0x103c0
void sub_103c0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_1d080;
  if (2 <= dat_1d078) {
    v4 = (unsigned long *)((long)dat_1d080 + 0x18);
    v1 = (unsigned long *)((long)dat_1d080 + (unsigned long)(unsigned int)(dat_1d078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x1d1c0) {
    free(*(void **)((long)v3 + 8));
    dat_1d098 = 0x1d1c0;
    dat_1d090 = 0x100;
  }
  if (v3 == (void *)0x1d090) {
    dat_1d078 = 1;
    return;
  }
  free(v3);
  dat_1d078 = 1;
  dat_1d080 = (void *)0x1d090;
}

// Function: sub_10460 @ 0x10460
char * sub_10460(unsigned int a0,char *a1)
{
  return sub_ff30(a0,a1,0xffffffffffffffff,(struct_2 *)0x1d2c0); // tail-call
}

// Function: sub_10480 @ 0x10480
char * sub_10480(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_ff30(a0,a1,a2); // tail-call
}

// Function: sub_10490 @ 0x10490
char * sub_10490(char *a0)
{
  return sub_10460(0,a0); // tail-call
}

// Function: sub_104a0 @ 0x104a0
char * sub_104a0(char *a0,unsigned long a1)
{
  return sub_10480(0,a0,a1); // tail-call
}

// Function: sub_104b0 @ 0x104b0
void sub_104b0(unsigned int a0,unsigned long a1,char *a2)
{
  char v1 [56];
  
  sub_e3b0(v1);
  sub_ff30(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_10510 @ 0x10510
void sub_10510(unsigned int a0,unsigned long a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_e3b0(v1);
  sub_ff30(a0,a2,a3,v1);
}

// Function: sub_10580 @ 0x10580
void sub_10580(unsigned int a0,char *a1)
{
  sub_104b0(0,a0,a1); // tail-call
}

// Function: sub_10590 @ 0x10590
void sub_10590(unsigned int a0,char *a1,unsigned long a2)
{
  sub_10510(0,a0,a1,a2); // tail-call
}

// Function: sub_105b0 @ 0x105b0
void sub_105b0(char *a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_1d2c0;
  v3 = dat_1d2c8;
  v4 = dat_1d2d0;
  v5 = dat_1d2d8;
  v8 = dat_1d2f0;
  v6 = dat_1d2e0;
  v7 = dat_1d2e8;
  sub_101a0(&v1,(int)a2,1,a3,a0,a1);
  sub_ff30(0,a0,a1,v2);
}

// Function: sub_10640 @ 0x10640
void sub_10640(char *a0,char a1)
{
  sub_105b0(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_10660 @ 0x10660
void sub_10660(char *a0)
{
  sub_10640(a0,0x3a); // tail-call
}

// Function: sub_10670 @ 0x10670
void sub_10670(char *a0,unsigned long a1)
{
  sub_105b0(a0,a1,0x3a); // tail-call
}

// Function: sub_10680 @ 0x10680
void sub_10680(unsigned int a0,unsigned long a1,char *a2)
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
  
  sub_e3b0(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_101a0(v3,0x3a,1);
  sub_ff30(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_10720 @ 0x10720
void sub_10720(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_1d2c0;
  v2 = dat_1d2c8;
  v7 = dat_1d2f0;
  v3 = dat_1d2d0;
  v4 = dat_1d2d8;
  v5 = dat_1d2e0;
  v6 = dat_1d2e8;
  sub_10200(&v1,a1,a2);
  sub_ff30(a0,a3,a4,&v1);
}

// Function: sub_107c0 @ 0x107c0
void sub_107c0(unsigned int a0)
{
  sub_10720(a0); // tail-call
}

// Function: sub_107d0 @ 0x107d0
void sub_107d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_107c0(0,a0,a1,a2); // tail-call
}

// Function: sub_107f0 @ 0x107f0
void sub_107f0(long a0,long a1,char *a2,unsigned long a3)
{
  sub_10720(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_10810 @ 0x10810
char * sub_10810(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_ff30(a0,a1,a2); // tail-call
}

// Function: sub_10820 @ 0x10820
char * sub_10820(char *a0,unsigned long a1)
{
  return sub_10810(0,a0,a1); // tail-call
}

// Function: sub_10830 @ 0x10830
char * sub_10830(unsigned int a0,char *a1)
{
  return sub_10810(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_10840 @ 0x10840
char * sub_10840(char *a0)
{
  return sub_10830(0,a0); // tail-call
}

// Function: sub_10850 @ 0x10850
unsigned long sub_10850(long a0)
{
  return *(unsigned long *)(a0 + 0x50);
}

// Function: sub_10860 @ 0x10860
unsigned long sub_10860(long a0)
{
  return *(unsigned long *)(a0 + 0x70);
}

// Function: sub_10870 @ 0x10870
unsigned long sub_10870(long a0)
{
  return *(unsigned long *)(a0 + 0x60);
}

// Function: sub_10880 @ 0x10880
unsigned long sub_10880(void)
{
  return 0;
}

// Function: sub_10890 @ 0x10890
undefined16 sub_10890(long a0)
{
  return *(char (*)[16])(a0 + 0x48);
}

// Function: sub_108a0 @ 0x108a0
undefined16 sub_108a0(long a0)
{
  return *(char (*)[16])(a0 + 0x68);
}

// Function: sub_108b0 @ 0x108b0
undefined16 sub_108b0(long a0)
{
  return *(char (*)[16])(a0 + 0x58);
}

// Function: sub_108c0 @ 0x108c0
undefined16 sub_108c0(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_108e0 @ 0x108e0
unsigned int sub_108e0(unsigned int a0)
{
  return a0;
}

// Function: sub_108f0 @ 0x108f0
void * sub_108f0(void *a0,unsigned long a1)
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

// Function: sub_10930 @ 0x10930
void sub_10930(void *a0,void *a1,unsigned long a2)
{
  *(char *)((long)memcpy(a0,a1,a2) + a2) = 0;
}

// Function: sub_10940 @ 0x10940
void sub_10940(void)
{
  getenv("TZ"); // tail-call
}

// Function: sub_10950 @ 0x10950
void sub_10950(char *a0)
{
  if (a0) {
    setenv("TZ",a0,1); // tail-call
    return;
  }
  unsetenv("TZ"); // tail-call
}

// Function: sub_10980 @ 0x10980
bool sub_10980(long a0)
{
  char *v1;
  bool v2; // zf
  
  v1 = (char *)(a0 + 9);
  if (!*(char *)(a0 + 8))
    v1 = NULL;
  v2 = sub_10950(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_109c0 @ 0x109c0
unsigned long * sub_109c0(char *a0)
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
      sub_10930((void *)((long)v3 + 9),a0,v1);
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

// Function: sub_10a60 @ 0x10a60
unsigned long sub_10a60(long *a0,struct_0 *a1) // return-dupe x2
{
  struct_0 *v1;
  long *v2;
  int v3; // eax
  long v4; // rax
  char *v5;
  
  v1 = (struct_0 *)a1->field_0x30;
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[1]))
    return 1;
  v5 = (char *)((long)a0 + 9);
  if (!v1->field_0x0[0]) {
    v5 = "";
label_10af8:
    a1->field_0x30 = v5;
    return 1;
  }
  do {
    v3 = strcmp(v5,(char *)v1);
    while( true ) {
      if (!v3) goto label_10af8;
      if ((!*v5) && ((v5 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        if ((long)a0 + (0x80U - (long)v5) <= (long)(strlen((char *)v1) + 1)) { // branch-flip
          v4 = sub_109c0(v1);
          *a0 = v4;
          if (!v4)
            return 0;
          *(char *)(v4 + 8) = 0;
          v5 = (char *)(v4 + 9);
        }
        else {
          sub_10930(v5,v1);
        }
        goto label_10af8;
      }
      v5 = &v5[strlen(v5) + 1];
      if ((*v5) || (v2 = (long *)*a0, !v2)) break;
      v5 = (char *)((long)v2 + 9);
      v3 = strcmp(v5,(char *)v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_10b90 @ 0x10b90
void sub_10b90(long *a0) // return-dupe
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

// Function: sub_10bd0 @ 0x10bd0
long * sub_10bd0(long a0)
{
  int v1;
  char *v2; // rax
  long *v3; // rax
  int *v4; // rax
  long *v5;
  
  v2 = (char *)sub_10940();
  if (v2) { // branch-flip
    if (*(char *)(a0 + 8)) {
      if (!strcmp((char *)(a0 + 9),v2))
        return (long *)0x1;
    }
  }
  else if (!*(char *)(a0 + 8))
    return (long *)0x1;
  v3 = (long *)sub_109c0(v2);
  v5 = v3;
  if (v3) {
    if (!sub_10980(a0)) {
      v4 = __errno_location();
      v5 = NULL;
      v1 = *v4;
      sub_10b90(v3);
      *v4 = v1;
    }
  }
  return v5;
}

// Function: sub_10c80 @ 0x10c80
unsigned int sub_10c80(long a0)
{
  unsigned int v1; // eax
  int *v2; // rax
  int v3;
  
  v1 = 1;
  if (a0 != 1) {
    v2 = __errno_location();
    v3 = *v2;
    v1 = sub_10980(a0);
    if (!(char)v1)
      v3 = *v2;
    sub_10b90(a0);
    *v2 = v3;
  }
  return v1;
}

// Function: sub_10ce0 @ 0x10ce0
struct_0 * sub_10ce0(long a0,void *a1,struct_0 *a2) // return-dupe
{
  long v1; // rax
  
  if (!a0)
    return (struct_0 *)gmtime_r(a1,(tm *)a2); // tail-call
  v1 = sub_10bd0(a0);
  if (!v1)
    return NULL;
  if ((localtime_r(a1,(tm *)a2)) && (sub_10a60(a0,a2))) {
    if (!sub_10c80(v1))
      return NULL;
    return a2;
  }
  sub_10c80(v1);
  return NULL;
}

// Function: sub_10d80 @ 0x10d80
unsigned long sub_10d80(long a0,unsigned long *a1) // return-dupe x2
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
    return sub_10eb0(a1); // tail-call
  v1 = sub_10bd0(a0);
  if (!v1)
    return 0xffffffffffffffff;
  v3 = *a1;
  v7 = -1;
  v4 = a1[1];
  v5 = a1[2];
  v8 = *(unsigned int *)&a1[4];
  v2 = sub_12d40(&v3);
  if ((0 <= v7) && (sub_10a60(a0,&v3))) {
    if (!sub_10c80(v1))
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
  sub_10c80(v1);
  return 0xffffffffffffffff;
}

// Function: sub_10eb0 @ 0x10eb0
long sub_10eb0(struct_1 *a0)
{
  long v1; // rax
  
  a0->field_0x20 = 0;
  v1 = sub_128b0(a0,dat_1cfd0,(long *)0x1d2f8); // tail-call
  return v1;
}

// Function: sub_10ed0 @ 0x10ed0
undefined16 sub_10ed0(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_10ee0 @ 0x10ee0
int sub_10ee0(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_10f10 @ 0x10f10
int sub_10f10(long a0,long a1,unsigned long a2)
{
  return ((unsigned int)(a0 == 0) & (unsigned int)CONCAT71((undefined7)((unsigned long)a2 >> 8),a1 != 0)) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_10f40 @ 0x10f40
double sub_10f40(long a0,long a1)
{
  return (double)a1 / dat_16088 + (double)a0;
}

// Function: sub_10f70 @ 0x10f70
unsigned int sub_10f70(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_13640();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_10fd0 @ 0x10fd0
long sub_10fd0(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_110fd:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_11136:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x16390) + 0x16390;
  switch(a5) {
    case 0:
      goto label_11136;
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
      goto label_110fd;
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

// Function: sub_11450 @ 0x11450
void sub_11450(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_10fd0(); // tail-call
}

// Function: sub_11470 @ 0x11470
void sub_11470(FILE *a0,char *a1,char *a2,char *a3,struct_28 *a4)
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
  sub_10fd0(a0,a1,a2,a3);
}

// Function: sub_11510 @ 0x11510
void sub_11510(void)
{
  sub_11470();
}

// Function: sub_115c0 @ 0x115c0
void sub_115c0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_11670 @ 0x11670
long sub_11670(long a0)
{
  if (a0)
    return a0;
  sub_11a40(); // no-return
}

// Function: sub_11680 @ 0x11680
void sub_11680(unsigned long a0)
{
  sub_11670(malloc(a0)); // tail-call
}

// Function: sub_116a0 @ 0x116a0
void sub_116a0(void)
{
  sub_11670(sub_13450()); // tail-call
}

// Function: sub_116c0 @ 0x116c0
void sub_116c0(void)
{
  sub_11680(); // tail-call
}

// Function: sub_116d0 @ 0x116d0
void sub_116d0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_11a40(); // no-return
}

// Function: sub_11710 @ 0x11710
void sub_11710(void)
{
  sub_11670(sub_13460()); // tail-call
}

// Function: sub_11730 @ 0x11730
void sub_11730(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_11a40(); // no-return
}

// Function: sub_11770 @ 0x11770
void sub_11770(void *a0,unsigned long a1,unsigned long a2)
{
  sub_11730(a0,a1,a2); // tail-call
}

// Function: sub_11780 @ 0x11780
void sub_11780(void)
{
  sub_11670(sub_13490()); // tail-call
}

// Function: sub_117a0 @ 0x117a0
void sub_117a0(unsigned long a0,unsigned long a1)
{
  sub_11730(NULL,a0,a1); // tail-call
}

// Function: sub_117b0 @ 0x117b0
void sub_117b0(unsigned long a0,unsigned long a1)
{
  sub_11780(0,a0,a1); // tail-call
}

// Function: sub_117c0 @ 0x117c0
void sub_117c0(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_11a40(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_11730(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_11830 @ 0x11830
void sub_11830(void *a0,unsigned long *a1)
{
  sub_117c0(a0,a1,1); // tail-call
}

// Function: sub_11840 @ 0x11840
void sub_11840(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_118e6;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_118e6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_11a40(a0,v3); // no-return
  sub_116d0(a0,v3);
  *a1 = v2;
}

// Function: sub_11920 @ 0x11920
void sub_11920(unsigned long a0,unsigned long a1)
{
  sub_11670(calloc(a0,a1)); // tail-call
}

// Function: sub_11940 @ 0x11940
void sub_11940(unsigned long a0)
{
  sub_11920(a0,1); // tail-call
}

// Function: sub_11950 @ 0x11950
void sub_11950(void)
{
  sub_11670(sub_13480()); // tail-call
}

// Function: sub_11970 @ 0x11970
void sub_11970(unsigned long a0)
{
  sub_11950(a0,1); // tail-call
}

// Function: sub_11980 @ 0x11980
void sub_11980(void *a0,unsigned long a1)
{
  memcpy((void *)sub_11680(a1),a0,a1); // tail-call
}

// Function: sub_119b0 @ 0x119b0
void sub_119b0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_116a0(a1),a0,a1); // tail-call
}

// Function: sub_119e0 @ 0x119e0
void sub_119e0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_116a0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_11a20 @ 0x11a20
void sub_11a20(char *a0)
{
  sub_11980(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_11a40 @ 0x11a40
void sub_11a40(void)
{
  error(dat_1d028,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_11a80 @ 0x11a80
void sub_11a80(long *a0,unsigned int a1,long a2)
{
  if (sub_a5f0(a0,a1 | 0x200,a2))
    return;
  if (*__errno_location() != 0x16)
    sub_11a40(); // no-return
  __assert_fail("errno != EINVAL","lib/xfts.c",0x29,"xfts_open"); // no-return
}

// Function: sub_11ad0 @ 0x11ad0
bool sub_11ad0(long a0,long a1)
{
  unsigned int v1; // eax
  bool v2; // r8b
  
  v2 = 1;
  v1 = *(unsigned int *)(a0 + 0x48) & 0x11;
  if ((v1 != 0x10) && (v2 = 0, v1 == 0x11))
    return *(long *)(a1 + 0x58) != 0;
  return v2;
}

// Function: sub_11b10 @ 0x11b10
unsigned long sub_11b10(long *a0,int a1)
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

// Function: sub_11b50 @ 0x11b50
unsigned int sub_11b50(long *a0,unsigned long a1,int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // r8d
  
  a1 &= 0xffffffff;
  v2 = 0;
  do {
    a2 -= 1;
    v1 = sub_11b10(a0,a1 & 0xffffffff);
    v2 |= v1;
  } while (a2);
  return v2;
}

// Function: sub_11b80 @ 0x11b80
unsigned int sub_11b80(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoimax"); // no-return
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
    if ((!a4) || (v1 = *v3, !v1)) goto label_11c0c;
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
        v10 |= sub_11b10(&v7,0x400);
        break;
      default:
        goto label_11c0f;
      case 3:
        v10 |= sub_11b50(&v7,v9,6);
        break;
      case 5:
      case 0x25:
        v10 |= sub_11b50(&v7,v9,3);
        break;
      case 9:
      case 0x29:
        v10 |= sub_11b50(&v7,v9,1);
        break;
      case 0xb:
      case 0x2b:
        v10 |= sub_11b50(&v7,v9,2);
        break;
      case 0xe:
        v10 |= sub_11b50(&v7,v9,5);
        break;
      case 0x12:
      case 0x32:
        v10 |= sub_11b50(&v7,v9,4);
        break;
      case 0x17:
        v10 |= sub_11b50(&v7,v9,8);
        break;
      case 0x18:
        v10 |= sub_11b50(&v7,v9,7);
        break;
      case 0x20:
        v10 |= sub_11b10(&v7,0x200);
        break;
      case 0x21:
        break;
      case 0x35:
        v10 |= sub_11b10(&v7,2);
      
    }
    *a1 = &v3[v8];
    if (v3[v8])
      v10 |= 2;
label_11c0c:
    *a3 = v7;
    return v10;
  }
label_11c0f:
  *a3 = v7;
  return v10 | 2;
}

// Function: sub_11f40 @ 0x11f40
void sub_11f40(unsigned int a0,int a1,char a2,long a3,unsigned long a4,int a5)
{
  char v1; // stack - 0x32
  char *v2;
  char *v3; // r12
  char *v4;
  char v5; // stack - 0x31
  
  if (4 <= a0) { // branch-flip
    if (a0 != 4)
      abort(); // no-return, return-dupe
    v2 = "invalid %s%s argument \'%s\'";
  }
  else if (2 <= a0) // branch-flip
    v2 = "invalid suffix in %s%s argument \'%s\'";
  else {
    if (a0 != 1) {
      abort();
    }
    v2 = "%s%s argument \'%s\' too large";
  }
  if (0 <= a1) { // branch-flip
    v3 = "--";
    v4 = *(char **)(a3 + (long)a1 * 0x20);
  }
  else {
    v4 = &v1;
    v5 = 0;
    v3 = &"--"[-(long)a1];
    v1 = a2;
  }
  error(a5,0,dcgettext(NULL,v2,5),v3,v4,a4);
}

// Function: sub_12020 @ 0x12020
void sub_12020(unsigned long a0,unsigned long a1,char a2,long a3,char *a4)
{
  sub_11f40(a0,a1,(int)a2,a3,a4,dat_1d028);
  abort(); // no-return
}

// Function: sub_12040 @ 0x12040
bool sub_12040(unsigned long *a0,int a1)
{
  unsigned long v1; // rax
  bool v2; // cf
  
  v1 = SUB168(ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0),0);
  v2 = SUB168(ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0),8) != 0;
  if (v2)
    v1 = 0xffffffffffffffff;
  *a0 = v1;
  return v2;
}

// Function: sub_12070 @ 0x12070
unsigned int sub_12070(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_12040();
  } while (a2);
  return v1;
}

// Function: sub_120a0 @ 0x120a0
unsigned int sub_120a0(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned long v10; // r8
  unsigned int v11;
  unsigned int v2; // eax
  int *v3; // rax
  long *v4; // rax
  unsigned char *v5;
  unsigned char v6;
  long v7;
  unsigned long v8; // stack - 0x48
  int v9; // ecx
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v7;
  v3 = __errno_location();
  *v3 = 0;
  v6 = *a0;
  v4 = __ctype_b_loc();
  v5 = a0;
  while (*(unsigned char *)(*v4 + 1 + (unsigned long)v6 * 2) & 0x20) {
    v6 = v5[1];
    v5 = &v5[1];
  }
  if (v6 == 0x2d)
    return 4;
  v8 = strtoumax(a0,a1,a2);
  v5 = (unsigned char *)*a1;
  if (v5 != a0) { // branch-flip
    if (*v3) { // branch-flip
      v11 = 1;
      if (*v3 != 0x22)
        return 4;
    }
    else {
      v11 = 0;
    }
    if (!a4) {
      *a3 = v8;
      return v11;
    }
    v6 = *v5;
    if (!v6) goto label_12162;
    v9 = (int)(char)v6;
  }
  else {
    if ((!a4) || (v6 = *a0, !v6))
      return 4;
    v9 = (int)(char)v6;
    if (!strchr(a4,v9))
      return 4;
    v8 = 1;
    v11 = 0;
  }
  if (!strchr(a4,v9)) {
label_12169:
    *a3 = v8;
    return v11 | 2;
  }
  v2 = (unsigned int)v6 - 0x45;
  v9 = 1;
  v10 = 0x400;
  if (((unsigned char)v2 <= 0x2f) && (0x814400308945U >> ((unsigned long)v2 & 0x3f) & 1)) {
    v10 = 0x400;
    v9 = 1;
    if (strchr(a4,0x30)) {
      v1 = v5[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v9 = (v1 == 0x42) + 1;
          v10 = 0x400;
          if (v1 == 0x42)
            v10 = 1000;
        }
        else {
          v9 = (v5[2] == 0x42) + 1 + (unsigned int)(v5[2] == 0x42);
        }
      }
      else {
        v9 = 2;
        v10 = 1000;
      }
    }
  }
  switch((unsigned int)v6 - 0x42 & 0xff) {
    case 0:
      v11 |= sub_12040(&v8,0x400);
      break;
    default:
      goto label_12169;
    case 3:
      v11 |= sub_12070(&v8,v10,6);
      break;
    case 5:
    case 0x25:
      v11 |= sub_12070(&v8,v10,3);
      break;
    case 9:
    case 0x29:
      v11 |= sub_12070(&v8,v10,1);
      break;
    case 0xb:
    case 0x2b:
      v11 |= sub_12070(&v8,v10,2);
      break;
    case 0xe:
      v11 |= sub_12070(&v8,v10,5);
      break;
    case 0x12:
    case 0x32:
      v11 |= sub_12070(&v8,v10,4);
      break;
    case 0x17:
      v11 |= sub_12070(&v8,v10,8);
      break;
    case 0x18:
      v11 |= sub_12070(&v8,v10,7);
      break;
    case 0x20:
      v11 |= sub_12040(&v8,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v11 |= sub_12040(&v8,2);
    
  }
  *a1 = &v5[v9];
  if (v5[v9])
    v11 |= 2;
label_12162:
  *a3 = v8;
  return v11;
}

// Function: sub_12480 @ 0x12480
long sub_12480(long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f);
}

// Function: sub_12490 @ 0x12490
bool sub_12490(unsigned long a0)
{
  bool v1; // r8b
  
  v1 = 0;
  if ((!(a0 & 3)) && (v1 = 1, (a0 * -0x70a3d70a3d70a3d7 + 0x51eb851eb851eb8 >> 2 | a0 * -0x70a3d70a3d70a3d7 << 0x3e) <= 0x28f5c28f5c28f5c))
    return ((unsigned int)((long)a0 / 100) & 3) == 1;
  return v1;
}

// Function: sub_12510 @ 0x12510
unsigned int sub_12510(unsigned int a0,unsigned int a1) // early-return
{
  if ((a0 == 0) == (a1 == 0))
    return 0;
  return ~(a0 | a1) >> 0x1f;
}

// Function: sub_12530 @ 0x12530
long sub_12530(long a0,long a1,unsigned long a2,unsigned long a3,int a4,unsigned int a5,int a6,int a7,int a8,int a9)
{
  int v1;
  int v2;
  int v3; // eax
  int v4; // edx
  unsigned long v5; // rdx
  long v6; // r8
  long v7; // r11
  long v8; // r12
  int v9; // r15d
  
  v7 = (long)(int)a3;
  v6 = (long)a4;
  v8 = (long)(int)a5;
  sub_12480(a0,2,a2,a3,v6);
  v2 = sub_12480(0x76c);
  a5 &= 3;
  v9 = (v2 + sub_12480(v8)) - (unsigned int)(a5 == 0);
  v1 = (unsigned int)(v5 >> 0x1f) & 1;
  a0 -= v8;
  v2 = v9 >> 0x1f;
  v3 = sub_12480((long)(int)((int)(v1 + (int)v5) / 0x19 - v1));
  v2 = sub_12480((long)((v9 - v2) / 0x19 + v2));
  return (v6 + ((v7 + (((long)(int)a2 + ((long)(v4 + (v3 - v2)) + ((a0 * 0x16d + a1) - (long)a6)) * 0x18) - (long)a7) * 0x3c) - (long)a8) * 0x3c) - (long)a9;
}

// Function: sub_12660 @ 0x12660
long sub_12660(long a0,long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  long v1; // rax
  long v2; // rdx
  long v3;
  
  v1 = sub_12480(a0,1,a0,a3,a4,a1);
  v3 = v1;
  sub_12480(a1);
  v1 = v2 + v3 + v1;
  return v1;
}

// Function: sub_12690 @ 0x12690
void sub_12690(void)
{
  sub_12530();
}

// Function: sub_126c0 @ 0x126c0
void sub_126c0(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x18
  
  v1 = a1;
  (*a0)(&v1,a2);
}

// Function: sub_12710 @ 0x12710
int * sub_12710(void *a0,long *a1,int *a2)
{
  int *v1; // rax
  int v10; // stack - 0x6c
  int v11; // stack - 0x68
  int v12; // stack - 0x64
  int v13; // stack - 0x60
  int v14; // stack - 0x5c
  unsigned long v15; // stack - 0x58
  int *v16; // stack - 0x50
  long v2; // rax
  long v3;
  long v4;
  int v5;
  int v6; // stack - 0x80
  int v7; // stack - 0x7c
  unsigned long v8; // stack - 0x78
  int v9; // stack - 0x70
  
  v4 = *a1;
  v16 = (int *)sub_126c0(a0,v4,a2);
  if (v16) // branch-flip
    *a1 = v4;
  else {
    v1 = __errno_location();
    v3 = 0;
    v5 = -1;
    if (*v1 == 0x4b) {
      while ((v2 = sub_12660(v3,v4), v3 != v2 && (v4 != v2))) {
        if (sub_126c0(a0,v2,a2)) { // branch-flip
          v7 = a2[1];
          v5 = *a2;
          v10 = a2[2];
          v11 = a2[3];
          v12 = a2[4];
          v9 = a2[5];
          v13 = a2[6];
          v14 = a2[7];
          v6 = a2[8];
          v8 = *(unsigned long *)&a2[10];
          v15 = *(unsigned long *)&a2[0xc];
          v3 = v2;
        }
        else {
          if (*v1 != 0x4b)
            return NULL;
          v4 = v2;
        }
      }
      if (0 <= v5) {
        *a1 = v3;
        *a2 = v5;
        a2[1] = v7;
        a2[2] = v10;
        a2[3] = v11;
        a2[4] = v12;
        a2[5] = v9;
        a2[6] = v13;
        a2[7] = v14;
        a2[8] = v6;
        *(unsigned long *)&a2[10] = v8;
        *(unsigned long *)&a2[0xc] = v15;
        v16 = a2;
      }
    }
  }
  return v16;
}

// Function: sub_128a0 @ 0x128a0
void sub_128a0(void)
{
  tzset(); // tail-call
}

// Function: sub_128b0 @ 0x128b0
long sub_128b0(int *a0,void *a1,long *a2)
{
  unsigned long v1;
  long v10; // rax
  long v11; // rax
  long v12;
  long v13;
  int v14;
  int v15; // stack - 0xb8
  long v16; // stack - 0xc8
  char v17 [32];
  long v18; // stack - 0xc0
  long v19; // rbp
  int v2;
  int v20;
  int *v21;
  long *v22;
  unsigned long v23; // r10
  int v24;
  int v25; // r14d
  bool v26;
  long v27; // stack - 0x130
  int v28; // stack - 0x118
  unsigned int v29; // stack - 0xb4
  int v3;
  unsigned long v30; // stack - 0xb0
  unsigned long v31; // stack - 0xa8
  unsigned long v32; // stack - 0xa0
  int v33; // stack - 0x98
  unsigned int v34; // stack - 0x94
  unsigned long v35; // stack - 0x90
  unsigned long v36; // stack - 0x88
  unsigned int v37; // stack - 0x58
  unsigned int v4;
  int v5;
  int v6;
  unsigned char v7; // al
  char v8; // al
  int v9;
  
  v2 = *a0;
  v3 = a0[1];
  v4 = a0[2];
  v23 = (unsigned long)v4;
  v5 = a0[8];
  v20 = a0[4] % 0xc;
  v1 = (long)(a0[4] / 0xc + (v20 >> 0x1f)) + (long)a0[5];
  v21 = a0;
  v22 = a2;
  v9 = v2;
  v7 = sub_12490(v1);
  v12 = (long)(int)((unsigned int)*(unsigned short *)(((long)(int)((v20 >> 0x1f & 0xcU) + v20) + (unsigned long)v7 * 0xd) * 2 + 0x166a0) - 1) + (long)v21[3];
  v20 = 0x3b;
  if (v9 <= 0x3b)
    v20 = v9;
  if (v20 <= -1)
    v20 = 0;
  v27._0_4_ = (int)*v22;
  v9 = -(int)v27;
  v10 = sub_12530(v1,v12,v23 & 0xffffffff,v3,v20,0x46,0,0,0,v9);
  v16 = v10;
  v27 = v10;
  v28 = 6;
  v26 = 0;
  v13 = v10;
  while( true ) {
    v19 = v13;
    if (!sub_12710(a1,&v16,&v15))
      return -1;
    v11 = sub_12690(v1,v12,v4,v3,v20,&v15);
    v6 = v33;
    v13 = v16;
    if (!v11) break;
    if ((v19 != v16) && (v27 == v16)) {
      if (v33 < 0) goto label_12aa0;
      if (0 <= v5) { // branch-flip
        if ((v5 != 0) != (v33 != 0)) goto label_12aa0;
      }
      else if (v26 <= (v33 != 0)) goto label_12aa0;
    }
    v28 -= 1;
    if (!v28) goto label_12ba0;
    v11 += v16;
    v26 = v33 != 0;
    v27 = v19;
    v16 = v11;
  }
  v8 = sub_12510(v5,v33);
  if (v8) {
    v25 = 0x92c70;
    do {
      v24 = 2;
      v14 = -v25;
      while( true ) {
        v18 = v14 + v13;
        if (!SCARRY8((long)v14,v13)) {
          if (!sub_12710(a1,&v18,v17))
            return -1;
          v8 = sub_12510(v5,v37);
          if (!v8) {
            v11 = sub_12690(v1,v12,v4,v3,v20,v17) + v18;
            if (sub_126c0(a1,v11,&v15)) {
              v13 = v11;
              goto label_12aa0;
            }
            if (*__errno_location() != 0x4b)
              return -1;
          }
        }
        v14 += v25 * 2;
        if (v24 == 1) break;
        v24 = 1;
      }
      v25 += 0x92c70;
    } while (v25 != 0xdb04f20);
    v13 += (int)(((unsigned int)(v5 == 0) - (unsigned int)(v6 == 0)) * 0xe10);
    v16 = v13;
    if (!sub_126c0(a1,v13,&v15)) {
label_12ba0:
      *__errno_location() = 0x4b;
      return -1;
    }
  }
label_12aa0:
  *a2 = v13 - (v9 + v10);
  if (v15 != v2) {
    v12 = (long)v2 + (((unsigned long)(v2 <= 0) & (unsigned long)CONCAT31((undefined3)((unsigned int)v15 >> 8),v15 == 0x3c)) - (long)v20);
    v26 = SCARRY8(v12,v13);
    v13 = v12 + v13;
    v16 = v13;
    if (v26) goto label_12ba0;
    if (!sub_126c0(a1,v13,&v15))
      return -1;
  }
  *(unsigned long *)a0 = CONCAT44(v29,v15);
  *(unsigned long *)&a0[2] = v30;
  *(unsigned long *)&a0[0xc] = v36;
  *(unsigned long *)&a0[4] = v31;
  *(unsigned long *)&a0[6] = v32;
  *(unsigned long *)&a0[8] = CONCAT44(v34,v33);
  *(unsigned long *)&a0[10] = v35;
  return v13;
}

// Function: sub_12d40 @ 0x12d40
long sub_12d40(int *a0)
{
  long v1; // rax
  
  sub_128a0();
  v1 = sub_128b0(a0,dat_1cfc8,(long *)0x1d300); // tail-call
  return v1;
}

// Function: sub_12d70 @ 0x12d70
unsigned long sub_12d70(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_12d80 @ 0x12d80
unsigned long sub_12d80(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_12d90 @ 0x12d90
unsigned int sub_12d90(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_12da0 @ 0x12da0
unsigned int sub_12da0(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_12db0 @ 0x12db0
unsigned long sub_12db0(unsigned long a0,unsigned char a1)
{
  return sub_12d70(a0,a1); // tail-call
}

// Function: sub_12dc0 @ 0x12dc0
unsigned long sub_12dc0(unsigned long a0,unsigned char a1)
{
  return sub_12d80(a0,a1); // tail-call
}

// Function: sub_12dd0 @ 0x12dd0
unsigned int sub_12dd0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_12de0 @ 0x12de0
unsigned int sub_12de0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_12df0 @ 0x12df0
unsigned int sub_12df0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_12e00 @ 0x12e00
unsigned int sub_12e00(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_12e10 @ 0x12e10
unsigned long sub_12e10(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_12e40 @ 0x12e40
bool sub_12e40(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_12e60 @ 0x12e60
bool sub_12e60(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_12e70 @ 0x12e70
unsigned int sub_12e70(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_12e90 @ 0x12e90
unsigned long sub_12e90(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_12eb0 @ 0x12eb0
bool sub_12eb0(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_12ec0 @ 0x12ec0
bool sub_12ec0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_12ed0 @ 0x12ed0
bool sub_12ed0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_12ee0 @ 0x12ee0
bool sub_12ee0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_12ef0 @ 0x12ef0
unsigned long sub_12ef0(int a0)
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

// Function: sub_12f40 @ 0x12f40
bool sub_12f40(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_12f60 @ 0x12f60
bool sub_12f60(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_12f70 @ 0x12f70
unsigned int sub_12f70(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_12fa0 @ 0x12fa0
int sub_12fa0(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_12fc0 @ 0x12fc0
int sub_12fc0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_12fe0 @ 0x12fe0
int sub_12fe0(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_12fa0((unsigned char)*a0);
    v1 = sub_12fa0((unsigned char)*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_13040 @ 0x13040
unsigned long sub_13040(FILE *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_70c0(a0);
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

// Function: sub_130b0 @ 0x130b0
unsigned long sub_130b0(unsigned long a0)
{
  return CONCAT71((undefined7)(a0 - 1 >> 8),(a0 - 1 & a0) == 0);
}

// Function: sub_130c0 @ 0x130c0
void sub_130c0(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_130e0 @ 0x130e0
unsigned long sub_130e0(struct_14 *a0,long *a1) // return-dupe x3
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
    v2 = sub_130b0(v3);
    if (!v2)
      return 0;
    v6 = v4;
    if (!v3)
      return 1;
  }
  else {
    a0->field_0x10 = 1;
    if (!sub_130b0(1))
      return 0;
    v7 = a1[1];
    v6 = v5;
  }
  v1 = *a1;
  *v6 = v7;
  v6[1] = v1;
  return 0;
}

// Function: sub_13190 @ 0x13190
void sub_13190(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_131a0 @ 0x131a0
int sub_131a0(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_1d308) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_1d308 = 1;
      return v2;
    }
    v2 = sub_13190(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_1d308 = -1;
  }
  else {
    v2 = sub_13190(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_1d308 != -1)
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

// Function: sub_13280 @ 0x13280
void sub_13280(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_13190(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_131a0(a0,a2);
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

// Function: sub_133d0 @ 0x133d0
void sub_133d0(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_10f70(open(a0,a1,v1));
}

// Function: sub_13450 @ 0x13450
void sub_13450(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_13460 @ 0x13460
void sub_13460(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_13480 @ 0x13480
void sub_13480(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_13490 @ 0x13490
void sub_13490(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_134c0 @ 0x134c0
char * sub_134c0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_13500 @ 0x13500
int sub_13500(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_13530 @ 0x13530
void sub_13530(struct_34 *a0,struct_35 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_35 *v3;
  
  v3 = (struct_35 *)a1->field_0x0;
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

// Function: sub_13590 @ 0x13590
unsigned int sub_13590(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x16720) >> (a0 & 0x1f) & 1;
}

// Function: sub_135b0 @ 0x135b0
unsigned long sub_135b0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_13660(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_13640 @ 0x13640
void sub_13640(unsigned long a0)
{
  sub_13280(a0,0,3); // tail-call
}

// Function: sub_13660 @ 0x13660
bool sub_13660(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_137b0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_136f0 @ 0x136f0
void sub_136f0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_13700 @ 0x13700
unsigned long sub_13700(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_136f0();
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

// Function: sub_137b0 @ 0x137b0
unsigned long sub_137b0(void)
{
  return sub_13700(); // tail-call
}

// Function: sub_137c0 @ 0x137c0
void sub_137c0(void)
{
  sub_136f0(); // tail-call
}

// Function: sub_137d0 @ 0x137d0
void sub_137d0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_1d008); // tail-call
}

// Function: _DT_FINI @ 0x137e4
void _DT_FINI(void)
{
  return;
}
