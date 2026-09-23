// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_1afe8)
    return;
  (*dat_1afe8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_1acb8)(); // jump-as-call
}

// Function: free @ 0x3620
void free(void *a0)
{
  (*dat_1afb8)(); // jump-as-call
}

// Function: localtime_r @ 0x3630
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_1afd0)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x3640
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_1afd8)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x3650
void __cxa_finalize(void)
{
  (*dat_1aff8)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x3660
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1acc0)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x3670
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1acc8)(); // jump-as-call
  return v1;
}

// Function: strtoimax @ 0x3680
void strtoimax(void)
{
  (*dat_1acd0)(); // jump-as-call
}

// Function: endmntent @ 0x3690
int endmntent(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1acd8)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x36a0
void abort(void)
{
  (*dat_1ace0)(); // jump-as-call
}

// Function: __errno_location @ 0x36b0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_1ace8)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x36c0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1acf0)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x36d0
void _exit(int a0)
{
  (*dat_1acf8)(); // jump-as-call
}

// Function: strcpy @ 0x36e0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ad00)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x36f0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1ad08)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x3700
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ad10)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3710
void qsort(void)
{
  (*dat_1ad18)(); // jump-as-call
}

// Function: iswcntrl @ 0x3720
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_1ad20)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3730
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ad28)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x3740
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_1ad30)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3750
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1ad38)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x3760
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1ad40)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x3770
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ad48)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3780
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ad50)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3790
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ad58)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x37a0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ad60)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x37b0
void regfree(re_pattern_buffer *a0)
{
  (*dat_1ad68)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x37c0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1ad70)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x37d0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1ad78)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x37e0
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_1ad80)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x37f0
void __stack_chk_fail(void)
{
  (*dat_1ad88)(); // jump-as-call
}

// Function: getopt_long @ 0x3800
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_1ad90)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3810
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1ad98)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x3820
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1ada0)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3830
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ada8)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x3840
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1adb0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3850
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1adb8)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3860
void lseek(void)
{
  (*dat_1adc0)(); // jump-as-call
}

// Function: __assert_fail @ 0x3870
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_1adc8)(); // jump-as-call
}

// Function: fputs @ 0x3880
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1add0)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x3890
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1add8)(); // jump-as-call
  return v1;
}

// Function: freopen @ 0x38a0
FILE * freopen(char *a0,char *a1,FILE *a2)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_1ade0)(); // jump-as-call
  return v1;
}

// Function: close @ 0x38b0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1ade8)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x38c0
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_1adf0)(); // jump-as-call
  return v1;
}

// Function: regcomp @ 0x38d0
int regcomp(re_pattern_buffer *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1adf8)(); // jump-as-call
  return v1;
}

// Function: fputc @ 0x38e0
int fputc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ae00)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x38f0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae08)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3900
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1ae10)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3910
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ae18)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3920
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae20)(); // jump-as-call
  return v1;
}

// Function: __getdelim @ 0x3930
long __getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_1ae28)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3940
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ae30)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x3950
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ae38)(); // jump-as-call
  return v1;
}

// Function: __memcpy_chk @ 0x3960
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae40)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x3970
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ae48)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3980
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ae50)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3990
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae58)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x39a0
void tzset(void)
{
  (*dat_1ae60)(); // jump-as-call
}

// Function: fileno @ 0x39b0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ae68)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x39c0
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_1ae70)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x39d0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1ae78)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x39e0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae80)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x39f0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ae88)(); // jump-as-call
  return v1;
}

// Function: getmntent @ 0x3a00
void * getmntent(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae90)(); // jump-as-call
  return v1;
}

// Function: setmntent @ 0x3a10
void * setmntent(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ae98)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3a20
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1aea0)(); // jump-as-call
  return v1;
}

// Function: __isoc99_sscanf @ 0x3a30
int __isoc99_sscanf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1aea8)(); // jump-as-call
  return v1;
}

// Function: regexec @ 0x3a40
int regexec(re_pattern_buffer *a0,char *a1,unsigned long a2,void *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_1aeb0)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3a50
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1aeb8)(); // jump-as-call
  return v1;
}

// Function: fchdir @ 0x3a60
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1aec0)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3a70
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1aec8)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3a80
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1aed0)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x3a90
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1aed8)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x3aa0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1aee0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3ab0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1aee8)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3ac0
void error(int a0,int a1,char *a2,...)
{
  (*dat_1aef0)(); // jump-as-call
}

// Function: open @ 0x3ad0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1aef8)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x3ae0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1af00)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3af0
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_1af08)(); // jump-as-call
  return v1;
}

// Function: towlower @ 0x3b00
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_1af10)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x3b10
void strtoumax(void)
{
  (*dat_1af18)(); // jump-as-call
}

// Function: fdopendir @ 0x3b20
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_1af20)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x3b30
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_1af28)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x3b40
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_1af30)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x3b50
void __cxa_atexit(void)
{
  (*dat_1af38)(); // jump-as-call
}

// Function: exit @ 0x3b60
void exit(int a0)
{
  (*dat_1af40)(); // jump-as-call
}

// Function: fwrite @ 0x3b70
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1af48)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x3b80
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_1af50)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3b90
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1af58)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3ba0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_1af60)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3bb0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_1af68)(); // jump-as-call
  return v1;
}

// Function: hasmntopt @ 0x3bc0
char * hasmntopt(void *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1af70)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x3bd0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_1af78)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x3be0
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_1af80)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x3bf0
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_1af88)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x3c00
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1af90)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x3c10
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_1af98)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x3c20
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1afa0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3c30
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1afa8)(); // jump-as-call
  return v1;
}

// Function: __sprintf_chk @ 0x3c40
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_1afb0)(); // jump-as-call
  return v1;
}

// Function: main @ 0x3c50
int main(int argc,char **argv,char **envp)
{
  unsigned long v1;
  char *v10;
  long v11; // rax
  long *v12;
  long v13;
  unsigned long *v14; // rax
  unsigned long v15;
  char *v16; // rax
  unsigned long v17;
  int *v18; // rax
  char *v19;
  long v2;
  long v20; // rcx
  unsigned long v21; // rdx
  unsigned long v22;
  long v23;
  unsigned long v24; // rdx
  unsigned long v25; // rdx
  unsigned long v26; // rdx
  unsigned long v27; // rdx
  unsigned long v28; // rdx
  unsigned long v29; // rdx
  long *v3;
  bool v30;
  int v31; // stack - 0x10c
  unsigned long v32; // stack - 0x108
  stat v33; // stack - 0xd8
  char *v34; // stack - 0xe8
  unsigned short v35;
  unsigned long v36;
  char **v37;
  unsigned long v38;
  long v39;
  unsigned int v4;
  bool v40;
  bool v41;
  bool v42; // stack - 0x160
  char *v43; // stack - 0x158
  unsigned int v44; // stack - 0x148
  long v45; // stack - 0x138
  bool v46; // stack - 0x125
  unsigned long v47; // stack - 0x100
  unsigned long v48; // stack - 0xf8
  unsigned long v49; // stack - 0xf0
  char v5; // al
  unsigned long v50; // stack - 0xe0
  int v6;
  int v7; // eax
  int v8; // eax
  struct_4 *v9; // rax
  
  v50 = 0;
  v34 = ".";
  sub_eb20(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_13be0(sub_5f80);
  dat_1b140 = sub_6800();
  sub_d520(getenv("DU_BLOCK_SIZE"),(unsigned int *)0x1b170,(unsigned long *)0x1b148);
  v30 = 0;
  v4 = 0x10;
  v44 = 8;
  v43 = NULL;
  v42 = 1;
  v40 = 0;
label_3d50:
  do {
    v31 = -1;
    v15 = (unsigned long)(unsigned int)argc;
    v6 = getopt_long(argc,argv,"0abd:chHklmst:xB:DLPSX:",(option *)0x1a700,&v31);
    if (v6 == -1) {
      if (!v42)
        sub_5540(1); // return-dupe, no-return
      v17 = v21;
      if (dat_1b186) { // branch-flip
        if (v30) {
          error(0,0,dcgettext(NULL,"cannot both summarize and show all entries",5));
          sub_5540(1);
        }
      }
      else if (v40) {
        if (v30) {
          if (dat_1b010) {
label_445d:
            v15 = dat_1b010;
            error(0,0,dcgettext(NULL,"warning: summarizing conflicts with --max-depth=%lu",5),v15);
            sub_5540(1); // no-return
          }
          error(0,0,dcgettext(NULL,"warning: summarizing is the same as using --max-depth=0",5));
          v17 = v24;
          if (dat_1b010) goto label_445d;
          dat_1b010 = 0; // crossjump-dupe
        }
      }
      else if (v30)
        dat_1b010 = 0;
      if (dat_1b16d) {
        if (dat_1b185) {
          error(0,0,dcgettext(NULL,"warning: options --apparent-size and -b are ineffective with --inodes",5));
          v17 = v27;
        }
        dat_1b148 = 1;
      }
      if (!dat_1b16c) goto label_41b8;
      if (dat_1b160) {
        v10 = dat_1b160;
        goto label_41a2;
      }
      v10 = getenv("TIME_STYLE");
      v17 = v25;
      if (v10) {
        dat_1b160 = v10;
        v17 = v26;
        if (strcmp(v10,"locale")) {
          if (*v10 != '+') goto label_4fc8;
          v19 = strchr(v10,10);
          if (!v19) goto label_41ad;
          *v19 = '\0';
          v17 = v29;
          goto label_41a2;
        }
      }
      v10 = "long-iso";
      dat_1b160 = "long-iso";
      goto label_4d36;
    }
    if (0x87 < v6) {
label_3dae:
      v42 = 0;
      goto label_3d50;
    }
    if (v6 <= 0x2f) {
      if (v6 == -0x83) {
        sub_11ab0(stdout,"du","GNU coreutils",dat_1b018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0,v15);
        exit(0); // no-return
      }
      if (v6 == -0x82)
        sub_5540(0); // no-return
      goto label_3dae;
    }
    switch(v6) {
      case 0x30:
        dat_1b182 = 1;
        goto label_3d50;
      default:
        goto label_3dae;
      case 0x42:
label_3f40:
        v6 = sub_d520(optarg,(unsigned int *)0x1b170,(unsigned long *)0x1b148);
        if (v6)
          sub_12850(v6,v31,0x42,0x1a700,optarg); // no-return
        break;
      case 0x44:
      case 0x48:
        v4 = 0x11;
        goto label_3d50;
      case 0x4c:
        v4 = 2;
        goto label_3d50;
      case 0x50:
        v4 = 0x10;
        goto label_3d50;
      case 0x53:
        dat_1b180 = '\x01';
        goto label_3d50;
      case 0x58:
        v6 = sub_70c0(sub_6b70,dat_1b140,optarg,0x10000000,10);
        if (v6) {
          v17 = sub_10b80(0,3,optarg);
          error(0,*__errno_location(),"%s",v17);
          v42 = 0;
        }
        goto label_3d50;
      case 0x61:
        dat_1b186 = '\x01';
        goto label_3d50;
      case 0x62:
        dat_1b185 = '\x01';
        dat_1b170 = 0;
        dat_1b148 = 1;
        goto label_3d50;
      case 99:
        dat_1b181 = '\x01';
        goto label_3d50;
      case 100:
        v6 = sub_12910(optarg,NULL,0,&v32,"");
        if (v6) { // branch-flip
          v10 = (char *)sub_10ef0(optarg);
          error(0,0,dcgettext(NULL,"invalid maximum depth %s",5),v10);
          v42 = 0;
        }
        else {
          dat_1b010 = v32;
          v40 = 1;
        }
        goto label_3d50;
      case 0x68:
        dat_1b170 = 0xb0;
        dat_1b148 = 1;
        goto label_3d50;
      case 0x6b:
        dat_1b170 = 0;
        dat_1b148 = 0x400;
        goto label_3d50;
      case 0x6c:
        dat_1b184 = '\x01';
        goto label_3d50;
      case 0x6d:
        dat_1b170 = 0;
        dat_1b148 = 0x100000;
        goto label_3d50;
      case 0x73:
        v30 = 1;
        goto label_3d50;
      case 0x74:
        v6 = sub_12250(optarg,0,0,0x1b178,"kKmMGTPEZY0");
        if (v6)
          sub_12850(v6,v31,0x74,0x1a700,optarg); // no-return
        if ((dat_1b178) || (*optarg != '-')) goto label_3d50;
        error(1,0,dcgettext(NULL,"invalid --threshold argument \'-0\'",5));
        goto label_3f40;
      case 0x78:
        v44 = 0x48;
        break;
      case 0x80:
        dat_1b185 = '\x01';
        break;
      case 0x81:
        sub_6b70(dat_1b140,optarg,0x10000000);
        break;
      case 0x82:
        v43 = optarg;
        break;
      case 0x83:
        dat_1b170 = 0x90;
        dat_1b148 = 1;
        break;
      case 0x85:
        dat_1b16c = '\x01';
        v6 = 0;
        if (optarg)
          v6 = *(int *)(sub_5ce0("--time",optarg,(long *)0x1a6c0,(void *)0x156f0,4,dat_1b020,1) * 4 + 0x156f0);
        dat_1b168 = v6;
        dat_1b150 = sub_10fe0(getenv("TZ"));
        break;
      case 0x86:
        dat_1b160 = optarg;
        break;
      case 0x87:
        dat_1b16d = '\x01';
        goto label_3d50;
      
    }
  } while( true );
label_4b60:
  v18 = __errno_location();
  if (*v18) {
    v40 = 0;
    v10 = (char *)sub_10b80(0,3,*(char **)(v11 + 0x20));
    v19 = dcgettext(NULL,"fts_read failed: %s",5);
    error(0,*v18,v19,v10);
  }
  dat_1b188 = 0;
  if (sub_abe0(v11)) {
    v40 = 0;
    v10 = dcgettext(NULL,"fts_close failed",5);
    error(0,*v18,v10);
  }
  v46 = (bool)(v46 & v40);
  goto label_4290;
label_4fc8:
  while( true ) {
    v10 = dat_1b160;
    v6 = strncmp(dat_1b160,"posix-",6);
    v17 = v28;
    if (v6) break;
    dat_1b160 = &v10[6];
  }
label_41a2:
  if (*v10 != '+') { // branch-flip
label_4d36:
    v6 = *(int *)(sub_5ce0("time style",v10,(long *)0x1a6a0,(void *)0x156e0,4,dat_1b020,1,v17) * 4 + 0x156e0);
    if (v6 != 1) { // branch-flip
      if (v6 != 2) { // branch-flip
        if (!v6)
          dat_1b158 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      else {
        dat_1b158 = "%Y-%m-%d";
      }
    }
    else {
      dat_1b158 = "%Y-%m-%d %H:%M";
    }
  }
  else {
label_41ad:
    dat_1b158 = &v10[1];
  }
label_41b8:
  if (v43) { // branch-flip
    if (optind < argc) {
      v10 = (char *)sub_10ef0(argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v10);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_5540(1); // no-return
    }
    if ((strcmp(v43,"-")) && (!sub_8e20(v43,"r",stdin))) {
      v17 = sub_10800(4,v43);
      v10 = dcgettext(NULL,"cannot open %s for reading",5);
      error(1,*__errno_location(),v10,v17);
      return v8;
    }
    v9 = (struct_4 *)sub_5e40(stdin);
    dat_1b183 = v42;
  }
  else {
    v37 = &v34;
    if (optind < argc)
      v37 = &argv[optind];
    v9 = (struct_4 *)sub_5e10(v37);
    dat_1b183 = optind + 1 < argc || v4 == 2;
  }
  if ((v9) && (dat_1b198 = sub_6130(), dat_1b198)) {
    if ((dat_1b184) || (!dat_1b183))
      v44 |= 0x100;
    v46 = v42;
label_4290:
    while (v10 = (char *)sub_5e80(v9,&v31), v10) {
      if (v43) { // branch-flip
        if ((strcmp(v43,"-")) || (strcmp(v10,"-"))) {
          if (*v10) goto label_42e9;
label_4788:
          v15 = sub_5f20(v9);
          v10 = dcgettext(NULL,"invalid zero-length file name",5);
          error(0,0,"%s:%lu: %s",(char *)sub_10b80(0,3,v43),v15,v10);
        }
        else {
          v19 = (char *)sub_10800(4,v10);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v19);
          if (!*v10) goto label_4788;
        }
        v46 = 0;
      }
      else {
        if (*v10) {
label_42e9:
          dat_1b110 = v10;
          v11 = sub_121c0((long *)0x1b110,v44 | v4,0);
          v40 = v42;
label_430c:
          v12 = (long *)sub_add0(v11);
          if (!v12) goto label_4b60;
          v10 = (char *)v12[7];
          v35 = *(unsigned short *)&v12[0xd];
          if (v35 != 4) { // branch-flip
            v30 = v42;
            if (v35 == 6) goto label_4510;
            v5 = sub_69c0(dat_1b140,v10);
            if (v5) {
label_43a8:
              if (v35 == 1) {
                sub_b520(v11,v12,4);
                if (v12 != (long *)sub_add0(v11))
                  __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
              }
              goto label_430c;
            }
            if (v35 == 0xb) {
              sub_b520(v11,v12,1);
              if (v12 != (long *)sub_add0(v11))
                __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
              v35 = *(unsigned short *)&v12[0xd];
            }
            if (v35 == 10 || v35 == 0xd) {
              v40 = 0;
              v10 = (char *)sub_10800(4,v10);
              v19 = dcgettext(NULL,"cannot access %s",5);
              error(0,(int)v12[8],v19,v10);
              goto label_430c;
            }
            if (((*(unsigned char *)(v11 + 0x48) & 0x40) && (1 <= v12[0xb])) && (*(long *)(v11 + 0x18) != v12[0xe])) goto label_43a8;
            if ((!dat_1b184) && ((dat_1b183 || (((*(unsigned int *)&v12[0x11] & 0xf000) != 0x4000 && (2 <= (unsigned long)v12[0x10])))))) {
              v6 = sub_61d0(dat_1b198,v12[0xe],v12[0xf]);
              if (v6 < 0) {
                sub_12180(); // no-return, return-dupe
              }
              if (!v6) goto label_43a8;
            }
            if (v35 == 2) {
              if (sub_12210(v11,v12)) {
                v3 = (long *)*v12;
                if (!dat_1b190) {
                  dat_1b190 = sub_6130();
                  if (!dat_1b190) {
                    sub_12180();
                  }
                  v23 = sub_df90(0);
                  while (v23) {
                    if (((!(*(unsigned char *)(v23 + 0x28) & 3)) && (v19 = *(char **)(v23 + 8), !stat(v19,&v33))) && (v6 = sub_61d0(dat_1b190,v33._0_8_,v33._8_8_), v6 <= -1)) {
                      sub_12180();
                    }
                    v13 = *(long *)(v23 + 0x30);
                    sub_e9f0(v23);
                    v23 = v13;
                  }
                }
                do {
                  if (v12 == v3) break;
                  v6 = sub_6260(dat_1b190,v12[0xe],v12[0xf]);
                  if (0 < v6) goto label_430c;
                  v12 = (long *)v12[1];
                } while (v12);
                v40 = 0;
                v10 = (char *)sub_10b80(0,3,v10);
                error(0,0,dcgettext(NULL,"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n",5),v10);
              }
              goto label_430c;
            }
            if (v35 != 7) {
              if (v35 != 1) goto label_4510;
              goto label_430c;
            }
            v19 = (char *)sub_10b80(0,3,v10);
            error(0,(int)v12[8],"%s",v19);
            v30 = 0;
          }
          else {
            v30 = 0;
            v19 = (char *)sub_10800(4,v10);
            v16 = dcgettext(NULL,"cannot read directory %s",5);
            error(0,(int)v12[8],v16,v19);
          }
label_4510:
          if (dat_1b168) { // branch-flip
            if (dat_1b168 != 2) { // branch-flip
              v15 = v12[0x1b];
              v39 = v12[0x1c];
            }
            else {
              v15 = v12[0x17];
              v39 = v12[0x18];
            }
          }
          else {
            v15 = v12[0x19];
            v39 = v12[0x1a];
          }
          if (dat_1b185) { // branch-flip
            v38 = 0;
            if (0 <= v12[0x14])
              v38 = v12[0x14];
          }
          else {
            v38 = v12[0x16] << 9;
          }
          v1 = v12[0xb];
          v47 = 1;
          v32 = v38;
          v48 = v15;
          v49 = v39;
          if (dat_1b108) { // branch-flip
            if (v1 != dat_1b188) {
              if (v1 <= dat_1b188) { // branch-flip
                if (v1 != dat_1b188 - 1)
                  __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
                v14 = (unsigned long *)(dat_1b188 * 0x40 + dat_1b100);
                v32 = v38 + *v14;
                if (CARRY8(v38,*v14))
                  v32 = 0xffffffffffffffff;
                v47 = v14[1] + 1;
                if ((int)(((unsigned int)((long)v14[3] < v39) - (unsigned int)(v39 < (long)v14[3])) + ((unsigned int)((long)v14[2] < (long)v15) - (unsigned int)((long)v15 < (long)v14[2])) * 2) <= -1) {
                  v48 = v14[2];
                  v49 = v14[3];
                }
                if (!dat_1b180) {
                  v41 = CARRY8(v32,v14[4]);
                  v32 += v14[4];
                  if (v41)
                    v32 = 0xffffffffffffffff;
                  v47 += v14[5];
                  if ((int)(((unsigned int)((long)v14[7] < (long)v49) - (unsigned int)((long)v49 < (long)v14[7])) + ((unsigned int)((long)v14[6] < (long)v48) - (unsigned int)((long)v48 < (long)v14[6])) * 2) <= -1) {
                    v48 = v14[6];
                    v49 = v14[7];
                  }
                }
                v23 = v1 * 0x40 + dat_1b100;
                v22 = *(unsigned long *)(v23 + 0x30);
                v36 = 0xffffffffffffffff;
                if (!CARRY8(*v14,*(unsigned long *)(v23 + 0x20)))
                  v36 = *v14 + *(unsigned long *)(v23 + 0x20);
                v45 = *(long *)(v23 + 0x38);
                *(unsigned long *)(v23 + 0x20) = v36;
                v13 = v14[1] + *(long *)(v23 + 0x28);
                v2 = v14[2];
                *(long *)(v23 + 0x28) = v13;
                if ((int)(((unsigned int)((long)v14[3] < v45) - (unsigned int)(v45 < (long)v14[3])) + ((unsigned int)(v2 < (long)v22) - (unsigned int)((long)v22 < v2)) * 2) <= -1) {
                  v2 = v14[3];
                  v22 = v14[2];
                  *(unsigned long *)(v23 + 0x30) = v14[2];
                  *(unsigned long *)(v23 + 0x38) = v2;
                  v45 = *(long *)(v23 + 0x38);
                }
                v20 = v36 + v14[4];
                if (CARRY8(v36,v14[4]))
                  v20 = -1;
                *(unsigned long *)(v23 + 0x28) = v13 + v14[5];
                v36 = v14[6];
                *(long *)(v23 + 0x20) = v20;
                if ((int)(((unsigned int)((long)v14[7] < v45) - (unsigned int)(v45 < (long)v14[7])) + ((unsigned int)((long)v36 < (long)v22) - (unsigned int)((long)v22 < (long)v36)) * 2) <= -1) {
                  v22 = v14[7];
                  *(unsigned long *)(v23 + 0x30) = v14[6];
                  *(unsigned long *)(v23 + 0x38) = v22;
                }
              }
              else {
                if (dat_1b108 <= v1) {
                  dat_1b100 = sub_11d40(dat_1b100,v1,0x80);
                  dat_1b108 = v1 * 2;
                }
                v22 = dat_1b188 + 1;
                if (v22 <= v1) {
                  v23 = dat_1b188 * 0x40 + dat_1b100;
                  do {
                    v22 += 1;
                    *(unsigned long *)(v23 + 0x40) = 0;
                    v13 = v23 + 0x40;
                    *(unsigned long *)(v23 + 0x48) = 0;
                    *(unsigned long *)(v23 + 0x50) = 0x8000000000000000;
                    *(unsigned long *)(v23 + 0x58) = 0xffffffffffffffff;
                    *(unsigned long *)(v23 + 0x60) = 0;
                    *(unsigned long *)(v23 + 0x68) = 0;
                    *(unsigned long *)(v23 + 0x70) = 0x8000000000000000;
                    *(unsigned long *)(v23 + 0x78) = 0xffffffffffffffff;
                    v23 = v13;
                  } while (v22 <= v1);
                }
              }
            }
          }
          else {
            dat_1b108 = v1 + 10;
            dat_1b100 = sub_12060(dat_1b108,0x40);
          }
          if ((!dat_1b180) || ((v35 & 0xfffd) != 4)) {
            v14 = (unsigned long *)(v1 * 0x40 + dat_1b100);
            v22 = v38 + *v14;
            if (CARRY8(v38,*v14))
              v22 = 0xffffffffffffffff;
            v14[1] = v14[1] + 1;
            *v14 = v22;
            if ((int)(((unsigned int)(v39 < (long)v14[3]) - (unsigned int)((long)v14[3] < v39)) + ((unsigned int)((long)v15 < (long)v14[2]) - (unsigned int)((long)v14[2] < (long)v15)) * 2) <= -1) {
              v14[2] = v15;
              v14[3] = v39;
            }
          }
          v41 = CARRY8(v38,dat_1b120);
          dat_1b120 = v38 + dat_1b120;
          if (v41)
            dat_1b120 = 0xffffffffffffffff;
          dat_1b128 += 1;
          if ((int)(((unsigned int)(v39 < (long)dat_1b138) - (unsigned int)((long)dat_1b138 < v39)) + ((unsigned int)((long)v15 < (long)dat_1b130) - (unsigned int)((long)dat_1b130 < (long)v15)) * 2) <= -1) {
            dat_1b130 = v15;
            dat_1b138 = v39;
          }
          dat_1b188 = v1;
          if (((((v35 & 0xfffd) != 4) && (!dat_1b186)) || (dat_1b010 < v1)) && (v1))
            v40 = (bool)(v40 & v30);
          else {
            v15 = v47;
            if (!dat_1b16d)
              v15 = v32;
            v41 = dat_1b178 <= v15;
            if ((long)dat_1b178 <= -1)
              v41 = v15 <= -dat_1b178;
            v40 = (bool)(v40 & v30);
            if (v41)
              sub_53d0(&v32,v10);
          }
          goto label_430c;
        }
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
        v46 = 0;
      }
    }
    if (v31 != 3) {
      if (v31 != 4) { // branch-flip
        if (v31 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
      }
      else {
        v17 = sub_10b80(0,3,v43);
        v10 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v10,v17);
        v46 = 0;
      }
      sub_5f40(v9);
      sub_61a0(dat_1b198);
      if (dat_1b190)
        sub_61a0(dat_1b190);
      if (((v43) && ((*(unsigned char *)stdin & 0x20 || (v6 = sub_71a0(stdin), v6)))) && (v46)) {
        v10 = (char *)sub_10800(4,v43);
        error(1,0,dcgettext(NULL,"error reading %s",5),v10);
        return v7;
      }
      if (dat_1b181)
        sub_53d0((unsigned long *)0x1b120,dcgettext(NULL,"total",5));
      return (int)(v46 ^ 1);
    }
  }
  sub_12180();
}

// Function: sub_5250 @ 0x5250
void sub_5250(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_1afc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_5280 @ 0x5280
void sub_5280(void)
{
  return;
}

// Function: _FINI_0 @ 0x52f0
void _FINI_0(void)
{
  if (!dat_1b0e8) {
    if (dat_1aff8)
      __cxa_finalize(dat_1b008);
    sub_5280();
    dat_1b0e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x5330
void _INIT_0(void)
{
  return;
}

// Function: sub_5340 @ 0x5340
void sub_5340(unsigned long a0)
{
  FILE *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != 0xffffffffffffffff) // branch-flip
    v2 = (char *)sub_c920(a0,v3,dat_1b170,1,dat_1b148);
  else {
    v2 = dcgettext(NULL,"Infinity",5);
  }
  fputs_unlocked(v2,v1);
}

// Function: sub_53d0 @ 0x53d0
void sub_53d0(unsigned long *a0,char *a1)
{
  char *v1;
  unsigned long v10; // stack - 0x90
  unsigned long v2;
  unsigned long v3;
  char *v4; // rax
  char *v5; // rax
  unsigned long v6; // stack - 0x98
  char v7 [64];
  char v8 [24];
  unsigned long v9;
  
  v9 = a0[1];
  if (!dat_1b16d)
    v9 = *a0;
  sub_5340(v9);
  if (dat_1b16c) {
    v1 = *(char **)&stdout->field_0x28;
    if (*(char **)&stdout->field_0x30 <= v1) // branch-flip
      __overflow(stdout,9);
    else {
      *(char **)&stdout->field_0x28 = &v1[1];
      *v1 = 9;
    }
    v3 = dat_1b158;
    v2 = dat_1b150;
    v6 = a0[2];
    v10 = a0[3];
    if (sub_112f0(dat_1b150,&v6,v7)) // branch-flip
      sub_8e00(stdout,v3,v7,v2,v10 & 0xffffffff);
    else {
      v4 = (char *)sub_d920(v6,v8);
      v5 = (char *)sub_10ef0(v4);
      error(0,0,dcgettext(NULL,"time %s is out of range",5),v5);
      fputs_unlocked(v4,stdout);
    }
  }
  __printf_chk(1,"\t%s%c",a1,-(dat_1b182 == '\0') & 10);
  fflush_unlocked(stdout); // tail-call
}

// Function: sub_5540 @ 0x5540
void sub_5540(int a0)
{
  FILE *v1;
  char *v10; // stack - 0x88
  char *v11; // stack - 0x80
  char *v12; // stack - 0x78
  char *v13; // stack - 0x70
  char *v14; // stack - 0x68
  char *v15; // stack - 0x60
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x50
  unsigned long v18; // stack - 0x40
  char *v19 [4]; // stack - 0xa8
  int v2; // eax
  char *v3; // rax
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7; // r13
  long v8; // fs_offset
  char *v9; // stack - 0xb0
  
  v6 = dat_1b1b0;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v6,v6);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Summarize device usage of the set of FILEs, recursively for directories.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
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
    __printf_chk(1,dcgettext(NULL,"\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n",5),"DU");
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",5),v1);
    v5 = "[";
    v6 = "[";
    v9 = "test invocation";
    v19[0] = "coreutils";
    v19[1] = "Multi-call invocation";
    v10 = "sha256sum";
    v19[2] = "sha224sum";
    v12 = "sha384sum";
    v19[3] = "sha2 utilities";
    v11 = "sha2 utilities";
    v13 = "sha2 utilities";
    v14 = "sha512sum";
    v15 = "sha2 utilities";
    v16 = 0;
    v17 = 0;
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
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "du";
    if (!strcmp("du","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "du")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}

// Function: sub_59a0 @ 0x59a0
void sub_59a0(void)
{
  sub_5540(1); // tail-call, no-return
}

// Function: sub_59b0 @ 0x59b0
unsigned long sub_59b0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_5ac0 @ 0x5ac0
long sub_5ac0(char *a0,long *a1) // return-dupe
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

// Function: sub_5b20 @ 0x5b20
void sub_5b20(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_10ed0(1,a0);
  error(0,0,v1,sub_106d0(0,8,a1),v2); // tail-call
}

// Function: sub_5bb0 @ 0x5bb0
void sub_5bb0(unsigned long *a0,void *a1,unsigned long a2)
{
  char *v1;
  char *v2;
  FILE *v3;
  long v4;
  void *v5;
  
  v3 = stderr;
  v5 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v3);
  v4 = 0;
  v1 = (char *)*a0;
  while (v1) {
    if ((v4) && (!memcmp(v5,a1,a2)))
      __fprintf_chk(stderr,1,", %s",(char *)sub_10ef0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_10ef0(v1));
      v5 = a1;
    }
    v4 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = (char *)a0[v4];
  }
  v2 = *(char **)&stderr->field_0x28;
  if (v2 < *(char **)&stderr->field_0x30) {
    *(char **)&stderr->field_0x28 = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stderr,10); // tail-call
}

// Function: sub_5ce0 @ 0x5ce0
long sub_5ce0(char *a0,char *a1,long *a2,void *a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_59b0(a1,a2,a3,a4);
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
  sub_5b20(a0,a1,v2);
  sub_5bb0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_5db0 @ 0x5db0
long sub_5db0(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_5e10 @ 0x5e10
void sub_5e10(char **a0) // return-dupe
{
  struct_44 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
}

// Function: sub_5e40 @ 0x5e40
void sub_5e40(unsigned long a0) // return-dupe
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

// Function: sub_5e80 @ 0x5e80
long sub_5e80(struct_4 *a0,int *a1) // early-return x2
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

// Function: sub_5f20 @ 0x5f20
long sub_5f20(struct_5 *a0)
{
  if (a0->field_0x0)
    return a0->field_0x8;
  return a0->field_0x28 - a0->field_0x20 >> 3;
}

// Function: sub_5f40 @ 0x5f40
void sub_5f40(struct_6 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_5f60 @ 0x5f60
void sub_5f60(unsigned long a0)
{
  dat_1b1a8 = a0;
}

// Function: sub_5f70 @ 0x5f70
void sub_5f70(char a0)
{
  dat_1b1a0 = a0;
}

// Function: sub_5f80 @ 0x5f80
void sub_5f80(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_13620(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_1b1a0) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_1b1a8) { // branch-flip
        v4 = (char *)sub_10a60(dat_1b1a8);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_1b028); // return-dupe, no-return
    }
  }
  v1 = sub_13620(stderr);
  if (!v1)
    return;
  _exit(dat_1b028);
}

// Function: sub_6030 @ 0x6030
unsigned long sub_6030(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_6040 @ 0x6040
unsigned long sub_6040(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_6050 @ 0x6050
unsigned long sub_6050(unsigned long a0,unsigned long a1)
{
  return a0 % a1;
}

// Function: sub_6060 @ 0x6060
void sub_6060(void *a0)
{
  sub_c280(*(struct_7 **)((long)a0 + 8));
  free(a0); // tail-call
}

// Function: sub_6080 @ 0x6080
long sub_6080(struct_14 *a0,long a1) // early-return
{
  struct_15 *v1; // rax
  struct_15 *v2;
  long v3;
  
  v2 = (struct_15 *)a0->field_0x10;
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
  v1 = (struct_15 *)sub_c720(a0->field_0x0,v2);
  if (!v1)
    return 0;
  if (v2 == v1) {
    a0->field_0x10 = NULL;
    v3 = sub_c0c0(0x3fd,NULL,sub_6050,NULL,0);
    v2->field_0x8 = v3;
    return v3;
  }
  v3 = v1->field_0x8;
  v2->field_0x8 = v3;
  return v3;
}

// Function: sub_6130 @ 0x6130
long * sub_6130(void)
{
  long *v1; // rax
  long v2; // rax
  long *v3;
  
  v1 = malloc(0x18);
  v3 = v1;
  if (v1) {
    v2 = sub_c0c0(0xb,NULL,sub_6030,sub_6040,sub_6060);
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

// Function: sub_61a0 @ 0x61a0
void sub_61a0(unsigned long *a0)
{
  sub_c280((struct_7 *)*a0);
  if ((struct_9 *)a0[1])
    sub_d850((struct_9 *)a0[1]);
  free((void *)a0[2]);
  free(a0); // tail-call
}

// Function: sub_61d0 @ 0x61d0
unsigned long sub_61d0(struct_16 *a0,long a1,long a2)
{
  void *v1; // rax
  unsigned long *v2;
  
  v1 = (void *)sub_6080(a0,a1);
  if (!v1)
    return 0xffffffff;
  if (0x7ffffffffffffffe <= (unsigned long)(a2 - 1U)) {
    v2 = a0->field_0x8;
    if (!v2) {
      v2 = (unsigned long *)sub_d7e0(0x7fffffffffffffff);
      a0->field_0x8 = v2;
      if (!v2)
        return 0xffffffff;
    }
    a2 = sub_d880(v2,a2);
    if (a2 == -1)
      return 0xffffffff;
  }
  return sub_c4f0(v1,a2,NULL); // tail-call
}

// Function: sub_6260 @ 0x6260
unsigned long sub_6260(struct_16 *a0,long a1,long a2)
{
  struct_17 *v1; // rax
  unsigned long *v2;
  
  v1 = (struct_17 *)sub_6080(a0,a1);
  if (!v1)
    return 0xffffffff;
  if (0x7ffffffffffffffe <= (unsigned long)(a2 - 1U)) {
    v2 = a0->field_0x8;
    if (!v2) {
      v2 = (unsigned long *)sub_d7e0(0x7fffffffffffffff);
      a0->field_0x8 = v2;
      if (!v2)
        return 0xffffffff;
    }
    a2 = sub_d880(v2,a2);
    if (a2 == -1)
      return 0xffffffff;
  }
  return (unsigned long)(sub_be30(v1,a2) != 0);
}

// Function: sub_62f0 @ 0x62f0
void sub_62f0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  (*(void *)*a3)(); // jump-as-call
}

// Function: sub_6300 @ 0x6300
void sub_6300(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_6310 @ 0x6310
unsigned int sub_6310(unsigned char *a0,unsigned char *a1)
{
  unsigned int v1; // eax
  
  v1 = sub_dc70(a0,a1);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_6330 @ 0x6330
unsigned int sub_6330(char *a0,char *a1)
{
  unsigned int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_6350 @ 0x6350
unsigned long sub_6350(unsigned char *a0,unsigned long a1)
{
  return sub_c050(a0,a1); // tail-call
}

// Function: sub_6360 @ 0x6360
unsigned long sub_6360(unsigned char *a0,unsigned char *a1,unsigned int a2) // early-return
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
    return sub_dc70(a0,a1); // tail-call
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
  v5 = (unsigned char *)sub_12160(a1);
  v7 = v5;
  while (v6 = strchr((char *)v7,0x2f), v6) {
    *v6 = '\0';
    v2 = sub_dc70(a0,v5);
    if ((int)v2 <= 0) {
      free(v5); // return-dupe
      return (unsigned long)v2;
    }
    *v6 = '/';
    v7 = (unsigned char *)&v6[1];
  }
  v2 = sub_dc70(a0,v5);
  free(v5);
  return (unsigned long)v2;
}

// Function: sub_6450 @ 0x6450
unsigned long sub_6450(unsigned char *a0,unsigned long a1)
{
  bool v1;
  unsigned long v10; // stack - 0x70
  char v11; // stack - 0x68
  int v2;
  unsigned long v3; // rax
  mbstate_t v4; // stack - 0x84
  unsigned int v5 [9]; // stack - 0x64
  unsigned char *v6;
  unsigned long v7;
  char v8; // stack - 0x7c
  unsigned char *v9; // stack - 0x78
  
  v7 = 0;
  v1 = 0;
  v4 = 0;
  v9 = a0;
  do {
    v8 = 0;
    if (v1) {
label_6523:
      v3 = __ctype_get_mb_cur_max();
      v6 = v9;
      v10 = sub_da70(v5,v6,sub_10f10(v9,v3),&v4);
      v2 = v5[0];
      v6 = v9;
      if (v10 != 0xffffffffffffffff) { // branch-flip
        if (v10 != 0xfffffffffffffffe) {
          if (!v10) {
            v10 = 1;
            if (*v9)
              __assert_fail("*iter->cur.ptr == \'\\0\'","lib/mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
            if (v5[0])
              __assert_fail("iter->cur.wc == 0","lib/mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
          }
          v11 = '\x01';
          if (!mbsinit(&v4)) goto label_64d4;
          v1 = 0;
          if (v2) goto label_64dd;
          return v7 % a1; // return-dupe
        }
        v10 = strlen((char *)v9);
        v11 = '\0';
        v2 = v5[0];
      }
      else {
        v11 = '\0';
        v10 = 1;
      }
    }
    else {
      if (!(*(unsigned int *)((unsigned long)(*v9 >> 5) * 4 + 0x15b40) >> (*v9 & 0x1f) & 1)) {
        if (!mbsinit(&v4))
          __assert_fail("mbsinit (&iter->state)","lib/mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_6523;
      }
      v10 = 1;
      v2 = (unsigned int)(char)*v9;
      v11 = '\x01';
      v5[0] = v2;
label_64d4:
      if (!v2)
        return v7 % a1;
label_64dd:
      v6 = v9;
    }
    v8 = 1;
    if (v11) // branch-flip
      v2 = towlower(v2);
    else {
      v2 = (unsigned int)(char)*v6;
      v9 = v6;
    }
    v9 = &v9[v10];
    v7 = (long)v2 + v7 * 0x1f;
  } while( true );
}

// Function: sub_6700 @ 0x6700
void sub_6700(void *a0,unsigned long a1)
{
  unsigned long v1;
  struct_25 *v2; // rax
  
  v2 = (struct_25 *)sub_11c80(0x10);
  v1 = *(unsigned long *)((long)a0 + 8);
  v2->field_0x8 = a1;
  v2->field_0x0 = v1;
  *(struct_25 **)((long)a0 + 8) = v2;
}

// Function: sub_6730 @ 0x6730
unsigned long sub_6730(char *a0,unsigned int a1) // return-dupe x2
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
          if (!(v1 + 0x85U & 0xfd)) goto label_67c0;
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
label_67c0:
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

// Function: sub_6800 @ 0x6800
void sub_6800(void)
{
  sub_12080(0x10); // tail-call
}

// Function: sub_6810 @ 0x6810
void sub_6810(struct_31 *a0)
{
  unsigned int *v1;
  long *v2;
  unsigned long *v3;
  unsigned long *v4;
  long v5; // rbx
  void *v6;
  long *v7;
  
  v2 = a0->field_0x0;
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
              if (v7[4] <= v5) goto label_6891;
            }
            v5 += 1;
            regfree((re_pattern_buffer *)&v1[2]);
            v6 = (void *)v7[2];
          } while (v5 < v7[4]);
        }
label_6891:
        free(v6);
      }
      free(v7);
      v7 = v2;
      if (!v2) goto label_68a6;
    }
    sub_c280((struct_7 *)v7[2]);
    free(v7);
  }
label_68a6:
  v4 = a0->field_0x8;
  while (v4) {
    v3 = (unsigned long *)*v4;
    free((void *)v4[1]);
    free(v4);
    v4 = v3;
  }
  free(a0); // tail-call
}

// Function: sub_6900 @ 0x6900
unsigned long sub_6900(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  char v2;
  char v3;
  char *v4;
  unsigned long v5; // r8
  char *v6;
  void *v7;
  
  v7 = sub_6360;
  if (a2 & 0x10000000)
    v7 = dat_1afe0;
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

// Function: sub_69c0 @ 0x69c0
unsigned int sub_69c0(long *a0,char *a1)
{
  int v1;
  unsigned long *v10; // rbx
  long v11; // r13
  struct_17 *v2;
  long *v3;
  unsigned long v4;
  long v5;
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
      v5 = a0[4];
      if (1 <= v5) {
        v10 = (unsigned long *)(a0[2] + 8);
        v11 = 0;
        do {
          if (*(unsigned int *)&v10[-1] & 0x8000000) { // branch-flip
            if (!regexec((re_pattern_buffer *)v10,a1,0,NULL,0)) {
              v9 = 0; // return-dupe
              free(v8); // return-dupe
              return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
            }
          }
          else {
            v4 = *v10;
            if (sub_6900(v4,a1)) {
              v9 = 0;
              free(v8);
              return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
            }
          }
          v11 += 1;
          v10 = &v10[9];
        } while (v5 != v11);
      }
    }
    else {
      if (!v8)
        v8 = (char *)sub_11c80(strlen(a1) + 1);
      v9 = *(unsigned int *)((long)a0 + 0xc);
      v2 = (struct_17 *)a0[2];
      v7 = a1;
      while( true ) {
        strcpy(v8,v7);
        v5 = sub_be30(v2,v8);
        while( true ) {
          if (v5) {
            v9 = 0;
            free(v8);
            return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
          }
          if ((!(v9 & 8)) || (v6 = strrchr(v8,0x2f), !v6)) break;
          *v6 = '\0';
          v5 = sub_be30(v2,v8);
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

// Function: sub_6b70 @ 0x6b70
void sub_6b70(struct_32 *a0,char *a1,unsigned int a2) // return-dupe x2
{
  unsigned int *v1;
  struct_33 *v10; // rax
  void *v11;
  char *v12;
  char v13;
  void *v14; // rdx
  unsigned long v15;
  unsigned int v16; // r12d
  long v17;
  long *v2;
  char *v3;
  int v4; // eax
  long *v5;
  unsigned long v6; // rax
  char *v7; // rax
  void *v8;
  char *v9;
  
  v5 = a0->field_0x0;
  if ((!(a2 & 0x18000000)) || (!sub_6730(a1,a2))) {
    if (((!v5) || ((int)v5[1])) || ((*(unsigned int *)((long)v5 + 0xc) ^ a2) & 0x60000018)) {
      v5 = (long *)sub_12080(0x28);
      v11 = sub_6310;
      v14 = sub_6450;
      *(unsigned int *)&v5[1] = 0;
      *(unsigned int *)((long)v5 + 0xc) = a2;
      if (!(a2 & 0x10)) {
        v11 = sub_6330;
        v14 = sub_6350;
      }
      v5[2] = sub_c0c0(0,NULL,v14,v11,sub_6300);
      v2 = a0->field_0x0;
      a0->field_0x0 = v5;
      *v5 = (long)v2;
    }
    v7 = (char *)sub_12160(a1);
    if ((a2 & 0x10000002) != 0x10000000) {
label_6d4b:
      v8 = (void *)v5[2];
      if (v7 == (char *)sub_c720(v8,v7))
        return;
      free(v7); // tail-call
      return;
    }
    v9 = v7;
    v12 = v7;
    do {
      v13 = *v12;
      v3 = v9;
      while (v13 != '\\') {
        v9 = &v3[1];
        v12 = &v12[1];
        *v3 = v13;
        if (!v13) goto label_6d4b;
        v13 = *v12;
        v3 = v9;
      }
      v9 = &v12[1];
      v13 = '\\';
      if (*v9) {
        v12 = &v12[1];
        v13 = *v9;
      }
      v9 = &v3[1];
      v12 = &v12[1];
      *v3 = v13;
    } while( true );
  }
  if ((!v5) || (((int)v5[1] != 1 || ((*(unsigned int *)((long)v5 + 0xc) ^ a2) & 0x20000000)))) {
    v5 = (long *)sub_12080(0x28);
    *(unsigned int *)&v5[1] = 1;
    *(unsigned int *)((long)v5 + 0xc) = a2;
    v2 = a0->field_0x0;
    a0->field_0x0 = v5;
    *v5 = (long)v2;
  }
  v17 = v5[4];
  v8 = (void *)v5[2];
  if (v17 == v5[3]) {
    v8 = (void *)sub_11f60(v8,&v5[3],1,-1,0x48);
    v17 = v5[4];
    v5[2] = (long)v8;
  }
  v5[4] = v17 + 1;
  v1 = (unsigned int *)((long)v8 + v17 * 0x48);
  *v1 = a2;
  if (!(a2 & 0x8000000)) {
    if (a2 & 0x4000000) {
      a1 = (char *)sub_12160(a1);
      v10 = (struct_33 *)sub_11c80(0x10);
      v17 = a0->field_0x8;
      v10->field_0x8 = a1;
      v10->field_0x0 = v17;
      a0->field_0x8 = (long)v10;
    }
    *(char **)&v1[2] = a1;
    return;
  }
  v16 = (int)a2 >> 3 & 2U | 9;
  if (a2 & 8) { // branch-flip
    v6 = strlen(a1);
    if (0 < (long)v6) {
      do {
        v15 = v6;
        if (a1[v6 - 1] != '/') goto label_6c51;
        v6 -= 1;
      } while (v6);
      goto label_6ca6;
    }
    v15 = v6;
    if (!v6) goto label_6ca6;
label_6c51:
    v7 = (char *)sub_11ca0(v15 + 7);
    memcpy(v7,a1,v6);
    builtin_strncpy(&v7[v6],"(/.*)?",7);
    v4 = regcomp((re_pattern_buffer *)&v1[2],v7,v16);
    free(v7);
  }
  else {
    v4 = regcomp((re_pattern_buffer *)&v1[2],a1,v16);
  }
  if (!v4)
    return;
  v17 = v5[4] + -1;
label_6ca6:
  v5[4] = v17;
  return;
}

// Function: sub_6eb0 @ 0x6eb0
int sub_6eb0(void *a0,struct_35 *a1,FILE *a2,unsigned int a3,unsigned char a4,unsigned long a5)
{
  unsigned long v1;
  unsigned char *v10;
  int v11; // stack - 0x5c
  int *v12; // stack - 0x58
  int v13; // stack - 0x50
  int v2; // eax
  void *v3; // rax
  unsigned char *v4;
  struct_34 *v5; // rax
  long v6; // stack - 0x48
  unsigned char *v7; // rbp
  unsigned char *v8; // r14
  long v9;
  
  v9 = 0;
  v3 = NULL;
  v6 = 0;
  while( true ) {
    v2 = getc(a2);
    if (v2 == -1) break;
    if (v6 == v9)
      v3 = (void *)sub_11f60(v3,&v6,1,-1,1);
    *(char *)((long)v3 + v9) = (char)v2;
    v9 += 1;
  }
  v11 = ferror(a2);
  v12 = __errno_location();
  v13 = 0;
  if (v11) {
    v11 = *v12;
    v13 = -(unsigned int)(v11 != 0);
  }
  v4 = (unsigned char *)sub_11d10(v3,v9 + 1);
  v4[v9] = a4;
  if (v9) { // branch-flip
    v7 = &v4[(unsigned long)(v4[v9 + -1] != a4) + v9];
    v5 = (struct_34 *)sub_11c80(0x10);
    v1 = a1->field_0x8;
    v5->field_0x8 = v4;
    v5->field_0x0 = v1;
    a1->field_0x8 = v5;
    if (v4 < v7) {
      v10 = v4;
      do {
        v8 = &v10[1];
        if (*v10 == a4) {
          v9 = *(long *)__ctype_b_loc();
          if (*(unsigned short *)(v9 + (unsigned long)a4 * 2) & 0x2000) { // branch-flip
            for (; v4 != v10; v10 = &v10[-1]) {
              if (!(*(unsigned char *)(v9 + 1 + (unsigned long)v10[-1] * 2) & 0x20)) goto label_706d;
            }
          }
          else {
label_706d:
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
    v5 = (struct_34 *)sub_11c80(0x10);
    v1 = a1->field_0x8;
    v5->field_0x8 = v4;
    v5->field_0x0 = v1;
    a1->field_0x8 = v5;
  }
  *v12 = v11;
  return v13;
}

// Function: sub_70c0 @ 0x70c0
unsigned int sub_70c0(unsigned long a0,struct_35 *a1,char *a2,unsigned int a3,char a4) // return-dupe
{
  int v1;
  unsigned int v2; // eax
  FILE *v3; // rax
  int *v4; // rax
  unsigned long v5 [2]; // stack - 0x30
  
  v5[0] = a0;
  if (!strcmp(a2,"-")) {
    v2 = sub_6eb0(sub_62f0,a1,stdin,a3,(int)a4,v5);
    return v2;
  }
  v3 = fopen(a2,"re");
  if (!v3)
    return 0xffffffff;
  v2 = sub_6eb0(sub_62f0,a1,v3,a3,(int)a4,v5);
  v4 = __errno_location();
  v1 = *v4;
  if (sub_71a0(v3))
    return 0xffffffff;
  *v4 = v1;
  return v2;
}

// Function: sub_71a0 @ 0x71a0
unsigned long sub_71a0(FILE *a0) // early-return
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
    if (sub_7230(a0)) {
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

// Function: sub_7230 @ 0x7230
void sub_7230(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_90f0(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_7280 @ 0x7280
unsigned long sub_7280(FILE *a0,char *a1,tm *a2,char a3,unsigned int a4,int a5,long *a6,int a7)
{
  char v1;
  unsigned char *v10;
  unsigned long v11;
  long *v12;
  long *v13; // rax
  unsigned char v14;
  int v15;
  long v16;
  char v17;
  unsigned int v18;
  int v19; // ecx
  int v2;
  int v20;
  unsigned int v21;
  char *v22;
  long v23;
  char v24; // stack - 0x44b
  char v25 [23];
  unsigned short v26; // stack - 0x44d
  long v27;
  unsigned long v28; // stack - 0x488
  unsigned char v29 [1010];
  int v3;
  unsigned char *v30;
  unsigned char *v31;
  unsigned int v32;
  int v33;
  int v34;
  unsigned int v35;
  int v36;
  int v37;
  unsigned long v38;
  char *v39;
  bool v4;
  unsigned long v40;
  char *v41;
  unsigned long v42;
  bool v43;
  int *v44; // stack - 0x4f0
  unsigned int v45; // stack - 0x4e8
  unsigned long v46; // stack - 0x4c0
  char *v47; // stack - 0x4b8
  unsigned long v48; // stack - 0x4a8
  bool v49; // stack - 0x4a0
  long v5;
  unsigned long v50; // stack - 0x480
  unsigned long v51; // stack - 0x478
  long v52; // stack - 0x470
  unsigned long v53; // stack - 0x468
  unsigned long v54; // stack - 0x460
  unsigned long v55; // stack - 0x458
  int *v6;
  long *v7;
  char v8;
  bool v9;
  
  v7 = a6;
  v38 = (unsigned long)a5;
  v44 = __errno_location();
  v34 = *(unsigned int *)&a2->field_0x8;
  v2 = *v44;
  v10 = (unsigned char *)0x160e1;
  if (*(unsigned char **)&a2->field_0x30)
    v10 = *(unsigned char **)&a2->field_0x30;
  if (0xd <= v34) // branch-flip
    v45 = v34 - 0xc;
  else {
    v45 = 0xc;
    if (v34)
      v45 = v34;
  }
  v8 = *a1;
  v42 = 0;
  v6 = v44;
  v12 = a6;
  v33 = a7;
label_7313:
  if (!v8) {
    *v6 = v2;
    return v42;
  }
  v22 = a1;
  a6 = v12;
  a7 = v33;
  if (v8 != '%') {
    v11 = 0;
    if (0 <= (int)v38)
      v11 = v38;
    v40 = 1;
    if (v11)
      v40 = v11;
    if (~v42 <= v40) goto label_7388;
    if (a0) {
      if (1 < (int)v38) {
        v38 = 0;
        do {
          v38 += 1;
          fputc(0x20,a0);
        } while (v38 < v11 - 1);
      }
      fputc((int)*a1,a0);
    }
    v42 += v40;
    goto label_735f;
  }
  v34 = 0;
  v21 = 0;
  v8 = a3;
  while( true ) {
    v41 = &v22[1];
    v18 = (unsigned int)*v41;
    v22 = &v22[1];
    v14 = *v41 - 0x23;
    if (0x3d <= v14) break;
    v11 = 1L << (v14 & 0x3f);
    if (v11 & 0x1000000000002500) // branch-flip
      v34 = v18;
    else if (v14 != 0x3b) { // branch-flip
      if (!(v11 & 1)) break;
      v21 = (unsigned int)v11 & 1;
    }
    else {
      v8 = '\x01';
    }
  }
  v32 = v18;
  if (v18 - 0x30 <= 9) {
    v38 = 0;
    do {
      v23 = (long)(int)v38 * 10;
      v37 = (int)v23;
      if ((v37 != v23) || (v38 = (unsigned long)(unsigned int)(v37 + *v22 + -0x30), SCARRY4(v37,*v22 + -0x30)))
        v38 = 0x7fffffff;
      v18 = (unsigned int)v22[1];
      v22 = &v22[1];
      v32 = v18;
    } while (v18 - 0x30 <= 9);
  }
  if (((char)v18 != 'E') && ((char)v18 != 'O'))
    v32 = 0;
  else {
    v18 = (unsigned int)(unsigned char)v22[1];
    v22 = &v22[1];
  }
  v17 = (char)v18;
  v37 = (int)v38;
  v43 = (bool)v21;
  v21 = v45;
  v35 = a4;
  switch(v18 & 0xff) {
    case 0:
      v22 = &v22[-1];
      break;
    case 0x25:
      if (&v22[-1] == a1) {
        if ((v34 != 0x2d) && (0 <= v37)) {
          v11 = (unsigned long)v37;
          v38 = 1;
          if (v11)
            v38 = v11;
          if (~v42 <= v38) goto label_7388;
          if (a0) {
            if (2 <= v37) {
              if ((v34 != 0x30) && (v23 = 0, v34 != 0x2b)) {
                do {
                  v23 += 1;
                  fputc(0x20,a0);
                } while (v11 - 1 != v23);
              }
              else {
                v23 = 0;
                do {
                  v23 += 1;
                  fputc(0x30,a0);
                } while (v11 - 1 != v23);
              }
            }
label_89ef:
            fputc((int)*v22,a0);
          }
        }
        else {
          if (0xfffffffffffffffe <= v42) goto label_7388;
          v38 = 1;
          if (a0) goto label_89ef;
        }
        goto label_829a;
      }
      v22 = &v22[-1];
      break;
    case 0x3a:
      v41 = &v22[1];
      v38 = 1;
      v1 = v22[1];
      while (v1 == ':') {
        v38 += 1;
        v41 = &v22[v38];
        v1 = v22[v38];
      }
      if (v1 == 'z') {
        v22 = v41;
        goto label_8483;
      }
      break;
    case 0x41:
    case 0x61:
      if (v32) break;
      v49 = 0;
      if (v43)
        v8 = v43;
      v41 = &v24;
      goto label_7643;
    case 0x42:
      if (v32 != 0x45) {
        v49 = 0;
        if (v43)
          v8 = v43;
        goto label_7626;
      }
      break;
    case 0x43:
      if (v32 != 0x45) {
        v15 = *(int *)&a2->field_0x14;
        v43 = v15 < -0x76c;
        v21 = (int)((-(unsigned int)(v15 + 0x76cU < 0x76c) & 0xffffff9d) + v15) / 100 + 0x13;
        v9 = -0x76c <= v15;
        if (!v34)
          v34 = a4;
        if (v34 != 0x2b) { // branch-flip
          v4 = 0;
          v47._0_4_ = 2;
        }
        else {
          v47._0_4_ = 2;
          v18 = 99;
label_8a70:
          if (v21 <= v18) goto label_8a2f;
          v4 = 1;
          v34 = 0x2b;
          v9 = (bool)(v43 ^ 1);
        }
        goto label_78ed;
      }
      goto label_7901;
    case 0x44:
      if (!v32) {
        v15 = -1;
        v47 = "%m/%d/%y";
        goto label_808e;
      }
      break;
    case 0x46:
      if (v32) break;
      if ((0 <= v37) || (v34)) {
        v15 = v37 + -6;
        if (v15 <= -1)
          v15 = 0;
        v47 = "%Y-%m-%d";
        goto label_808e;
      }
      v15 = 4;
      v38 = sub_7280(0,"%Y-%m-%d",a2,(unsigned char)v8,0x2b,4,v7,v33);
      v34 = 0x2b;
      v47 = "%Y-%m-%d";
      v11 = 0;
      v46 = v38;
      goto label_80e8;
    case 0x47:
    case 0x56:
    case 0x67:
      if (v32 != 0x45) {
        v36 = *(int *)&a2->field_0x1c;
        v20 = *(int *)&a2->field_0x18;
        v3 = *(int *)&a2->field_0x14;
        v15 = (v36 - v20) + 0x17e;
        v21 = v3 + -100 + (v3 >> 0x1f & 400U);
        v15 = (v36 - v15) + 3 + (v15 / 7) * 7;
        if (0 <= v15) { // branch-flip
          v19 = 0x16d;
          if ((!(v21 & 3)) && (v19 = 0x16e, (v21 * -0x3d70a3d7 + 0x51eb850 >> 2 | v3 * 0x40000000) <= 0x28f5c28))
            v19 = 0x16e - (unsigned int)((int)v21 % 400 != 0);
          v20 = ((v36 - v19) - v20) + 0x17e;
          v36 = ((v36 - v19) - v20) + 3 + (v20 / 7) * 7;
          if (0 <= v36)
            v15 = v36;
          v20 = (v36 >> 0x1f) + 1;
        }
        else {
          v21 -= 1;
          v15 = 0x16d;
          if ((!(v21 & 3)) && (v15 = 0x16e, (v21 * -0x3d70a3d7 + 0x51eb850 >> 2 | v21 * 0x40000000) <= 0x28f5c28))
            v15 = 0x16e - (unsigned int)((int)v21 % 400 != 0);
          v19 = ((v15 + v36) - v20) + 0x17e;
          v20 = -1;
          v15 = ((v15 + v36) - v19) + 3 + (v19 / 7) * 7;
        }
        if (v17 == 'G') {
          v21 = v3 + 0x76c + v20;
          v43 = v3 < 0xfffff894U - v20;
          if (v34) { // branch-flip
label_8a59:
            v47._0_4_ = 4;
            v18 = 9999;
            if (v34 == 0x2b) goto label_8a70;
            v4 = 0;
            v9 = (bool)(v43 ^ 1);
          }
          else {
            if (a4 == 0x2b) {
label_8ce5:
              v47._0_4_ = 4;
              v18 = 9999;
              goto label_8a70;
            }
            v4 = 0;
            v47._0_4_ = 4;
            v9 = 0xfffff894U - v20 <= v3;
            v34 = a4;
          }
          goto label_78ed;
        }
        if (v17 != 'g') {
          v47._0_4_ = 2;
          v21 = v15 / 7 + 1;
          goto label_7b80;
        }
        v21 = (v3 % 100 + v20) % 100;
        if (0 <= (int)v21) goto label_78c0;
        if (v3 < 0xfffff894U - v20) {
          v21 = -v21;
          if (!v34) goto label_78d2;
label_8a1a:
          v35 = v34;
          goto label_78d2;
        }
        v21 += 100;
        if (v34) goto label_8a1a;
        if (a4 == 0x2b) goto label_8a24;
        goto label_78d8;
      }
      break;
    case 0x48:
      if (v32 != 0x45) {
        v47._0_4_ = 2;
        v21 = *(unsigned int *)&a2->field_0x8;
        goto label_7b80;
      }
      break;
    case 0x49:
      if (v32 != 0x45) {
        v47._0_4_ = 2;
        goto label_7b80;
      }
      break;
    case 0x4d:
      if (v32 != 0x45) {
        v47._0_4_ = 2;
        v21 = *(unsigned int *)&a2->field_0x4;
        goto label_7b80;
      }
      break;
    case 0x4e:
      if (v32 != 0x45) {
        v36 = 9;
        v15 = v33;
        if (v37 <= 0)
          v37 = 9;
        do {
          if (v36 <= v37) {
            if (v36 == 1) {
              v11 = 1;
              v38 = 1;
              goto label_7a00;
            }
            if (v15 % 10) goto label_79f1;
          }
          v36 -= 1;
          v15 /= 10;
        } while( true );
      }
      break;
    case 0x50:
      v49 = 1;
      goto label_85f6;
    case 0x52:
      v15 = -1;
      v47 = "%H:%M";
      goto label_808e;
    case 0x53:
      if (v32 != 0x45) {
        v47._0_4_ = 2;
        v21 = *(unsigned int *)a2;
        goto label_7b80;
      }
      break;
    case 0x54:
      v15 = -1;
      v47 = "%H:%M:%S";
label_808e:
      v38 = sub_7280(0,v47,a2,(unsigned char)v8,v34,v15,v7,v33);
      if ((v34 != 0x2d) && (0 <= v37)) {
        v11 = (unsigned long)v37;
        v46 = v11;
        if (v11 <= v38)
          v46 = v38;
      }
      else {
        v46 = v38;
        v11 = 0;
      }
label_80e8:
      if (~v42 <= v46) goto label_7388;
      if (a0) {
        if (v38 < v11) {
          if ((v34 != 0x30) && (v23 = 0, v34 != 0x2b)) {
            do {
              v23 += 1;
              fputc(0x20,a0);
            } while (v11 - v38 != v23);
          }
          else {
            v23 = 0;
            do {
              v23 += 1;
              fputc(0x30,a0);
            } while (v11 - v38 != v23);
          }
        }
        sub_7280(a0,v47,a2,(unsigned char)v8,v34,v15,v7,a7);
      }
      v42 += v46;
      goto label_735f;
    case 0x55:
      if (v32 != 0x45) {
        v15 = (*(int *)&a2->field_0x1c - *(int *)&a2->field_0x18) + 7;
label_7c2e:
        v47._0_4_ = 2;
        v21 = v15 / 7;
        goto label_7b80;
      }
      break;
    case 0x57:
      if (v32 != 0x45) {
        v15 = *(int *)&a2->field_0x18 + 6;
        v15 = ((v15 / 7) * 7 - v15) + 7 + *(int *)&a2->field_0x1c;
        goto label_7c2e;
      }
      break;
    case 0x58:
    case 99:
    case 0x78:
      if (v32 != 0x4f) goto label_7621;
      break;
    case 0x59:
      if (v32 == 0x45) goto label_7901;
      if (v32 == 0x4f) break;
      v43 = *(int *)&a2->field_0x14 < -0x76c;
      v21 = *(int *)&a2->field_0x14 + 0x76c;
      if (v34) goto label_8a59;
      if (a4 == 0x2b) goto label_8ce5;
      v4 = 0;
      v18 = 0;
      v47._0_4_ = 4;
      v34 = a4;
      goto label_8310;
    case 0x5a:
      if (v43)
        v8 = 0;
      v38 = strlen((char *)v10);
      if ((0 <= v37) && (v34 != 0x2d)) {
        v11 = (unsigned long)v37;
        v48 = v38;
        if (v38 < v11)
          v48 = v11;
      }
      else {
        v11 = 0;
        v48 = v38;
      }
      if (~v42 <= v48) goto label_7388;
      if (a0) {
        if (v38 < v11) {
          if ((v34 != 0x30) && (v34 != 0x2b)) {
            v23 = 0;
            if (v11 != v38) {
              do {
                v23 += 1;
                fputc(0x20,a0);
              } while (v11 - v38 != v23);
            }
          }
          else if (v11 != v38) {
            v23 = 0;
            do {
              v23 += 1;
              fputc(0x30,a0);
            } while (v11 - v38 != v23);
          }
        }
        if (v43) { // branch-flip
          if (v38) {
            v13 = __ctype_tolower_loc();
            v30 = v10;
            do {
              v14 = *v30;
              v30 = &v30[1];
              fputc(*(int *)(*v13 + (unsigned long)v14 * 4),a0);
            } while (v30 != &v10[v38]);
          }
        }
        else if (v8) { // branch-flip
          if (v38) {
            v13 = __ctype_toupper_loc();
            v30 = v10;
            do {
              v14 = *v30;
              v30 = &v30[1];
              fputc(*(int *)(*v13 + (unsigned long)v14 * 4),a0);
            } while (v30 != &v10[v38]);
          }
        }
        else {
          fwrite(v10,v38,1,a0);
        }
      }
      goto label_7770;
    case 0x62:
    case 0x68:
      if (v43)
        v8 = v43;
      if (v32 != 0x45) goto label_7621;
      break;
    case 100:
      if (v32 != 0x45) {
        v47._0_4_ = 2;
        v21 = *(unsigned int *)&a2->field_0xc;
        goto label_7b80;
      }
      break;
    case 0x65:
      if (v32 != 0x45) {
        v21 = *(unsigned int *)&a2->field_0xc;
label_81d4:
        v47._0_4_ = 2;
        if (!v34)
          v34 = 0x5f;
        goto label_7b80;
      }
      break;
    case 0x6a:
      if (v32 != 0x45) {
        v4 = 0;
        v47._0_4_ = 3;
        v15 = *(int *)&a2->field_0x1c;
        v21 = v15 + 1;
        v43 = v15 < -1;
        v9 = -1 <= v15;
        v18 = 0;
        goto label_78f0;
      }
      break;
    case 0x6b:
      if (v32 != 0x45) {
        v21 = *(unsigned int *)&a2->field_0x8;
        goto label_81d4;
      }
      break;
    case 0x6c:
      if (v32 != 0x45) goto label_81d4;
      break;
    case 0x6d:
      if (v32 != 0x45) {
        v4 = 0;
        v47._0_4_ = 2;
        v15 = *(int *)&a2->field_0x10;
        v21 = v15 + 1;
        v43 = v15 < -1;
        v9 = -1 <= v15;
        v18 = 0;
        goto label_78f0;
      }
      break;
    case 0x6e:
      if ((v34 != 0x2d) && (0 <= v37)) {
        v11 = (unsigned long)v37;
        v38 = 1;
        if (v11)
          v38 = v11;
        if (~v42 <= v38) goto label_7388;
        if (a0) {
          if (2 <= v37) {
            if ((v34 != 0x30) && (v23 = 0, v34 != 0x2b)) {
              do {
                v23 += 1;
                fputc(0x20,a0);
              } while (v11 - 1 != v23);
            }
            else {
              v23 = 0;
              do {
                v23 += 1;
                fputc(0x30,a0);
              } while (v11 - 1 != v23);
            }
          }
label_828d:
          fputc(10,a0);
        }
      }
      else {
        if (0xfffffffffffffffe <= v42) goto label_7388;
        v38 = 1;
        if (a0) goto label_828d;
      }
      goto label_829a;
    case 0x70:
      v49 = 0;
label_85f6:
      v17 = 'p';
      if (v43)
        v49 = v43;
      if (v43)
        v8 = 0;
label_7626:
      if (v32) goto label_7915;
      v41 = &v24;
      goto label_7643;
    case 0x71:
      if (v32 == 0x4f) goto label_7901;
      v4 = 0;
      v43 = 0;
      v18 = 0;
      v47._0_4_ = 1;
      v21 = (*(int *)&a2->field_0x10 * 0xb >> 5) + 1;
      goto label_7f40;
    case 0x72:
label_7621:
      v49 = 0;
      goto label_7626;
    case 0x73:
      v53 = *(unsigned long *)&a2->field_0x20;
      v54 = *(unsigned long *)&a2->field_0x28;
      v28 = *(unsigned long *)a2;
      v50 = *(unsigned long *)&a2->field_0x8;
      v51 = *(unsigned long *)&a2->field_0x10;
      v55 = *(unsigned long *)&a2->field_0x30;
      v52._0_4_ = (unsigned int)*(unsigned long *)&a2->field_0x18;
      v52 = CONCAT44(0xffffffff,(unsigned int)v52);
      v23 = sub_113a0(v7,&v28);
      if (v52 <= -1) {
        *v44 = 0x4b;
        return 0;
      }
      v43 = (bool)((unsigned long)v23 >> 0x3f);
      v30 = &v29[1];
      v16 = v23;
      do {
        v5 = v16 / 10;
        v17 = (char)v16 + (char)v5 * '\xf6';
        if (v23 <= -1)
          v17 = -v17;
        v30 = &v30[-1];
        *v30 = v17 + 0x30;
        v16 = v5;
      } while (v5);
      v4 = 0;
      v47._0_4_ = 1;
      goto label_7d40;
    case 0x74:
      if ((v34 != 0x2d) && (0 <= v37)) {
        v11 = (unsigned long)v37;
        v38 = 1;
        if (v11)
          v38 = v11;
        if (~v42 <= v38) goto label_7388;
        if (a0) {
          if (2 <= v37) {
            if ((v34 != 0x30) && (v23 = 0, v34 != 0x2b)) {
              do {
                v23 += 1;
                fputc(0x20,a0);
              } while (v11 - 1 != v23);
            }
            else {
              v23 = 0;
              do {
                v23 += 1;
                fputc(0x30,a0);
              } while (v11 - 1 != v23);
            }
          }
label_7ede:
          fputc(9,a0);
        }
      }
      else {
        if (0xfffffffffffffffe <= v42) goto label_7388;
        v38 = 1;
        if (a0) goto label_7ede;
      }
      goto label_829a;
    case 0x75:
      v47._0_4_ = 1;
      v21 = (*(int *)&a2->field_0x18 + 6) % 7 + 1;
label_7b80:
      v4 = 0;
      v18 = 0;
      v14 = (unsigned char)(v21 >> 0x18);
      v43 = (bool)(v14 >> 7);
      v9 = (bool)((unsigned char)~v14 >> 7);
      goto label_78f0;
    case 0x77:
      if (v32 != 0x45) {
        v47._0_4_ = 1;
        v21 = *(unsigned int *)&a2->field_0x18;
        goto label_7b80;
      }
      break;
    case 0x79:
      if (v32 == 0x45) goto label_7901;
      v18 = *(int *)&a2->field_0x14 % 100;
      v21 = v18;
      if ((int)v18 <= -1) {
        v21 = v18 + 100;
        if (*(int *)&a2->field_0x14 <= -0x76d)
          v21 = -v18;
      }
label_78c0:
      if (v34) goto label_8a1a;
label_78d2:
      if (v35 != 0x2b) { // branch-flip
label_78d8:
        v4 = 0;
        v9 = 1;
        v43 = 0;
        v47._0_4_ = 2;
        v34 = v35;
      }
      else {
label_8a24:
        v47._0_4_ = 2;
        v43 = 0;
label_8a2f:
        v4 = (int)v47 < v37;
        v9 = (bool)(v43 ^ 1);
        v34 = 0x2b;
      }
label_78ed:
      v18 = 0;
      goto label_78f0;
    case 0x7a:
      v38 = 0;
label_8483:
      if (*(int *)&a2->field_0x20 < 0) goto label_735f;
      v43 = 1;
      v15 = (int)*(unsigned long *)&a2->field_0x28;
      if ((0 <= v15) && (v43 = 0, !v15))
        v43 = *v10 == 0x2d;
      v21 = v15 / 0xe10;
      v36 = (v15 / 0x3c) % 0x3c;
      if (v38 != 2) { // branch-flip
        if (3 <= v38) { // branch-flip
          if (v38 != 3) break;
          if (v15 % 0x3c) goto label_8baf;
          if (v36) goto label_8533;
          v4 = 1;
          v47._0_4_ = 3;
          v9 = (bool)(v43 ^ 1);
          v18 = 0;
        }
        else if (v38) { // branch-flip
label_8533:
          v4 = 1;
          v47._0_4_ = 6;
          v9 = (bool)(v43 ^ 1);
          v21 = v21 * 100 + v36;
          v18 = 4;
        }
        else {
          v4 = 1;
          v47._0_4_ = 5;
          v9 = (bool)(v43 ^ 1);
          v21 = v21 * 100 + v36;
          v18 = 0;
        }
      }
      else {
label_8baf:
        v4 = 1;
        v47._0_4_ = 9;
        v18 = 0x14;
        v21 = v15 % 0x3c + v21 * 10000 + v36 * 100;
        v9 = (bool)(v43 ^ 1);
      }
label_78f0:
      if ((v32 != 0x4f) || (!v9)) {
label_8310:
        if (v43)
          v21 = -v21;
label_7f40:
        v31 = &v29[1];
        while( true ) {
          if (v18 & 1) {
            v31[-1] = 0x3a;
            v31 = &v31[-1];
          }
          v18 = (int)v18 >> 1;
          v30 = &v31[-1];
          v31[-1] = (char)v21 + (char)(v21 / 10) * '\xf6' + 0x30;
          if ((v21 <= 9) && (!v18)) break;
          v21 /= 10;
          v31 = v30;
        }
label_7d40:
        v21 = v34;
        if (!v34)
          v21 = 0x30;
        if (v37 <= -1)
          v37 = (int)v47;
        v33 = ((int)v29 + 1) - (int)v30;
        if (v43) { // branch-flip
          v47._0_1_ = '-';
label_8675:
          v15 = (v37 + -1) - v33;
          if ((v15 <= 0) || (v34 == 0x2d))
            v15 = 0;
          if (v21 == 0x5f) {
            if ((a0) && (v15)) {
              v23 = 0;
              do {
                v23 += 1;
                fputc(0x20,a0);
              } while (v23 != v15);
            }
            v42 += (long)v15;
            v37 -= v15;
          }
          if (0xfffffffffffffffe <= v42) goto label_7388;
          if (a0)
            fputc((int)(char)v47,a0);
          v11 = (unsigned long)v33;
          v42 += 1;
          v38 = 0;
          if ((v21 != 0x2d) && (v37 = v37 + -1, 0 <= v37)) {
label_7d87:
            v38 = (unsigned long)v37;
            v11 = (long)v33;
            if ((unsigned long)(long)v33 <= v38)
              v11 = v38;
          }
        }
        else {
          if (v4) {
            v47._0_1_ = '+';
            goto label_8675;
          }
          v11 = (unsigned long)v33;
          if (((v33 < v37) && (v34 != 0x2d)) || (v38 = 0, v21 != 0x2d)) goto label_7d87;
        }
        v40 = (unsigned long)v33;
        if (~v42 <= v11) goto label_7388;
        if (a0) {
          if (v40 < v38) {
            if ((v21 != 0x30) && (v23 = 0, v21 != 0x2b)) {
              do {
                v23 += 1;
                fputc(0x20,a0);
              } while (v38 - v40 != v23);
            }
            else {
              v23 = 0;
              do {
                v23 += 1;
                fputc(0x30,a0);
              } while (v38 - v40 != v23);
            }
          }
          if (v8) { // branch-flip
            if (v40) {
              v12 = __ctype_toupper_loc();
              v31 = &v30[v40];
              do {
                v14 = *v30;
                v30 = &v30[1];
                fputc(*(int *)(*v12 + (unsigned long)v14 * 4),a0);
              } while (v30 != v31);
            }
          }
          else {
            fwrite(v30,v40,1,a0);
          }
        }
        goto label_7e50;
      }
label_7901:
      v49 = 0;
label_7915:
      v24 = (char)v32;
      v41 = &v27;
label_7643:
      v26 = 0x2520;
      *v41 = v17;
      v39 = &v25[1];
      v41[1] = '\0';
      v38 = strftime(v39,0x400,(char *)&v26,a2);
      if (v38) {
        v11 = v38 - 1;
        v48 = v11;
        if ((v34 != 0x2d) && (0 <= v37)) {
          v40 = (unsigned long)v37;
          if (v11 < v40)
            v48 = v40;
        }
        else {
          v40 = 0;
        }
        if (~v42 <= v48) goto label_7388;
        if (a0) {
          if (v11 < v40) {
            if ((v34 != 0x30) && (v34 != 0x2b)) {
              v23 = 0;
              if (v40 != v11) {
                do {
                  v23 += 1;
                  fputc(0x20,a0);
                } while (v40 - v11 != v23);
              }
            }
            else if (v40 != v11) {
              v23 = 0;
              do {
                v23 += 1;
                fputc(0x30,a0);
              } while (v40 - v11 != v23);
            }
          }
          if (v49) { // branch-flip
            if (v11) {
              v12 = __ctype_tolower_loc();
              v41 = &v25[2];
              do {
                v8 = *v41;
                v41 = &v41[1];
                fputc(*(int *)(*v12 + (unsigned long)(unsigned char)v8 * 4),a0);
              } while (v41 != &v39[v38]);
            }
          }
          else if (v8) { // branch-flip
            if (v11) {
              v12 = __ctype_toupper_loc();
              v41 = &v25[2];
              do {
                v8 = *v41;
                v41 = &v41[1];
                fputc(*(int *)(*v12 + (unsigned long)(unsigned char)v8 * 4),a0);
              } while (v41 != &v39[v38]);
            }
          }
          else {
            fwrite(&v25[2],v11,1,a0);
          }
        }
label_7770:
        v42 += v48;
      }
      goto label_735f;
    
  }
  v41 = &v22[1 - (long)a1];
  if ((v34 != 0x2d) && (0 <= v37)) {
    v39 = (char *)(long)v37;
    v47 = v39;
    if (v39 <= v41)
      v47 = v41;
  }
  else {
    v39 = NULL;
    v47 = v41;
  }
  if ((char *)~v42 <= v47) goto label_7388;
  if (a0) {
    if (v41 < v39) {
      if ((v34 != 0x30) && (v34 != 0x2b)) {
        v23 = 0;
        if (v39 != v41) {
          do {
            v23 += 1;
            fputc(0x20,a0);
          } while ((long)v39 - (long)v41 != v23);
        }
      }
      else if (v39 != v41) {
        v23 = 0;
        do {
          v23 += 1;
          fputc(0x30,a0);
        } while ((long)v39 - (long)v41 != v23);
      }
    }
    if (v8) { // branch-flip
      if (v41) {
        v12 = __ctype_toupper_loc();
        v41 = &a1[(long)&v22[1 - (long)a1]];
        do {
          v8 = *a1;
          a1 = &a1[1];
          fputc(*(int *)(*v12 + (unsigned long)(unsigned char)v8 * 4),a0);
        } while (a1 != v41);
      }
    }
    else {
      fwrite(a1,(unsigned long)v41,1,a0);
    }
  }
  v42 += (long)v47;
  goto label_735f;
label_79f1:
  v11 = (unsigned long)v36;
  v38 = v11;
  if (v36) { // branch-flip
label_7a00:
    v41 = &v25[v38 + 1];
    do {
      v41 = &v41[-1];
      *v41 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
      v15 /= 10;
    } while (v41 != &v25[v38 - (unsigned int)(v36 - 1)]);
  }
  else {
    v11 = 0;
  }
  if (!v34)
    v34 = 0x30;
  if (~v42 <= v11) goto label_7388;
  v42 += v11;
  if (a0) { // branch-flip
    if (!v8) {
      fwrite(&v25[1],v11,1,a0);
      goto label_7aec;
    }
    if (v11) {
      v13 = __ctype_toupper_loc();
      v31 = (unsigned char *)&v25[1];
      v30 = &v31[v11];
      do {
        v14 = *v31;
        v31 = &v31[1];
        fputc(*(int *)(*v13 + (unsigned long)v14 * 4),a0);
      } while (v30 != v31);
      goto label_7aec;
    }
    if (v34 != 0x2d) {
      v38 = (unsigned long)(v37 - v36);
      if (~v42 <= v38) goto label_7388;
      goto label_7b1d;
    }
  }
  else {
label_7aec:
    if ((v34 != 0x2d) && (0 <= v37 - v36)) {
      v38 = (unsigned long)(v37 - v36);
      if (~v42 <= v38) goto label_7388;
      if (!a0) {
label_829a:
        v42 += v38;
        goto label_735f;
      }
label_7b1d:
      if (v38) {
        if ((v34 != 0x30) && (v11 = 0, v34 != 0x2b)) {
          do {
            v11 += 1;
            fputc(0x20,a0);
          } while (v11 != v38);
        }
        else {
          v11 = 0;
          do {
            v11 += 1;
            fputc(0x30,a0);
          } while (v11 != v38);
        }
label_7e50:
        v42 += v11;
      }
      goto label_735f;
    }
  }
  if (v42 == 0xffffffffffffffff) {
label_7388:
    *v44 = 0x22;
    return 0;
  }
label_735f:
  v8 = v22[1];
  a1 = &v22[1];
  v38 = 0xffffffffffffffff;
  v6 = v44;
  v12 = a6;
  v33 = a7;
  goto label_7313;
}

// Function: sub_8e00 @ 0x8e00
unsigned long sub_8e00(void)
{
  return sub_7280();
}

// Function: sub_8e20 @ 0x8e20
FILE * sub_8e20(char *a0,char *a1,FILE *a2)
{
  bool v1;
  int v2; // eax
  int v3;
  int *v4;
  FILE *v5;
  bool v6;
  bool v7;
  
  v2 = fileno(a2);
  if (v2 != 1) { // branch-flip
    if (v2 != 2) { // branch-flip
      if (!v2) goto label_8f9d;
      v6 = dup2(2,2) != 2;
    }
    else {
      v6 = 0;
    }
    v3 = dup2(1,1);
    v7 = v3 != 1;
    v2 = dup2(0,0);
    v4 = __errno_location();
    if (v2) {
      v2 = open("/dev/null",0);
      if (!v2) {
        v1 = 1;
        goto label_906a;
      }
label_8f68:
      if (0 <= v2) { // branch-flip
        v3 = 9;
        v5 = NULL;
        close(v2);
        *v4 = 9;
        v1 = 1;
      }
      else {
        v1 = 1;
        v3 = *v4;
        v5 = NULL;
      }
      goto label_8fc3;
    }
    v1 = 0;
label_906a:
    if (v3 == 1) {
      v7 = 0;
label_8edd:
      if ((!v6) || (v2 = open("/dev/null",0), v2 == 2)) goto label_8fa8;
      if (0 <= v2) { // branch-flip
        v3 = 9;
        close(v2);
        *v4 = 9;
      }
      else {
        v3 = *v4;
      }
      v5 = NULL;
      goto label_8f0c;
    }
    v2 = open("/dev/null",0);
    if (v2 == 1) {
      v7 = 1;
      goto label_8edd;
    }
    if (0 <= v2) { // branch-flip
      v3 = 9;
      close(v2);
      *v4 = 9;
    }
    else {
      v3 = *v4;
    }
    v5 = NULL;
    if (v6)
      close(2);
  }
  else {
    v2 = dup2(0,0);
    v4 = __errno_location();
    if (v2) { // branch-flip
      v2 = open("/dev/null",0);
      if (v2) {
        v6 = 0;
        v7 = 0;
        goto label_8f68;
      }
      v1 = 1;
      v6 = 0;
      v7 = 0;
    }
    else {
label_8f9d:
      v1 = 0;
      v6 = 0;
      v7 = 0;
    }
label_8fa8:
    v5 = freopen(a0,a1,a2);
    v4 = __errno_location();
    v3 = *v4;
label_8fc3:
    if (v6) {
label_8f0c:
      close(2);
    }
    if (!v7) goto label_8f2f;
  }
  close(1);
label_8f2f:
  if (v1)
    close(0);
  if (!v5)
    *v4 = v3;
  return v5;
}

// Function: sub_90f0 @ 0x90f0
int sub_90f0(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_9170 @ 0x9170
unsigned long sub_9170(struct_20 *a0,struct_20 *a1)
{
  if (a0->field_0x8 != a1->field_0x8)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x0 >> 8),a0->field_0x0 == a1->field_0x0);
}

// Function: sub_91a0 @ 0x91a0
unsigned long sub_91a0(void *a0,unsigned long a1)
{
  return *(unsigned long *)((long)a0 + 8) % a1;
}

// Function: sub_91c0 @ 0x91c0
unsigned long sub_91c0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_91d0 @ 0x91d0
unsigned long sub_91d0(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_91e0 @ 0x91e0
int sub_91e0(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_9200 @ 0x9200
int sub_9200(void *a0,void *a1,bool a2) // early-return x3
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
      goto label_929b;
    }
  }
  else {
    v3 = *(char **)((long)a1 + 0x30);
    v5 = *(int *)((long)a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_929b:
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

// Function: sub_93b0 @ 0x93b0
void * sub_93b0(void *a0,void *a1,unsigned long a2)
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
label_940b:
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
      goto label_940b;
    }
    v4 = *(long **)((long)a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned long *)((long)a0 + 0x38) = 0;
  return a1;
}

// Function: sub_94c0 @ 0x94c0
void * sub_94c0(struct_24 *a0,void *a1,unsigned long a2)
{
  unsigned long v1;
  void *v2; // rax
  
  v2 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (v2) {
    memcpy((void *)((long)v2 + 0x100),a1,a2);
    v1 = a0->field_0x20;
    *(char *)((long)v2 + a2 + 0x100) = 0;
    *(unsigned long *)((long)v2 + 0x60) = a2;
    *(struct_24 **)((long)v2 + 0x50) = a0;
    *(unsigned long *)((long)v2 + 0x38) = v1;
    *(unsigned int *)((long)v2 + 0x40) = 0;
    *(unsigned long *)((long)v2 + 0x18) = 0;
    *(unsigned int *)((long)v2 + 0x6a) = 0x30000;
    *(unsigned long *)((long)v2 + 0x20) = 0;
    *(unsigned long *)((long)v2 + 0x28) = 0;
  }
  return v2;
}

// Function: sub_9560 @ 0x9560
unsigned long sub_9560(void *a0,int a1)
{
  long v1;
  struct_17 *v2;
  struct_25 *v3; // rax
  struct_25 *v4; // rax
  unsigned long v5 [2]; // stack - 0xb8
  statfs v6; // stack - 0xa8
  
  v1 = *(long *)((long)a0 + 0x50);
  v2 = *(struct_17 **)(v1 + 0x50);
  if (!(*(unsigned char *)(v1 + 0x49) & 2))
    return 0;
  if (!v2) {
    v2 = (struct_17 *)sub_c0c0(0xd,NULL,sub_91c0,sub_91d0,dat_1afb8);
    *(struct_17 **)(v1 + 0x50) = v2;
    if (!v2) goto label_95ee;
  }
  v5[0] = *(unsigned long *)((long)a0 + 0x70);
  v1 = sub_be30(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_95ee:
  if ((0 <= a1) && (!fstatfs(a1,&v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      v3->field_0x0 = *(unsigned long *)((long)a0 + 0x70);
      v3->field_0x8 = v6._0_8_;
      v4 = (struct_25 *)sub_c720(v2,v3);
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

// Function: sub_9690 @ 0x9690
void sub_9690(void *a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)((long)a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_d740((void *)((long)a0 + 0x60));
    if (0 <= v1) {
      close(v1);
      goto label_96d4;
    }
  }
  else if ((!(*(unsigned char *)((long)a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_96d4:
    *(int *)((long)a0 + 0x2c) = a1;
    return;
  }
  *(int *)((long)a0 + 0x2c) = a1;
}

// Function: sub_96f0 @ 0x96f0
unsigned long sub_96f0(void *a0,long a1)
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

// Function: sub_9760 @ 0x9760
unsigned long sub_9760(void *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)((long)a0 + 0x48) & 0x102) {
    v1 = sub_c0c0(0x1f,NULL,sub_91a0,sub_9170,dat_1afb8);
    *(long *)((long)a0 + 0x58) = v1;
    return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)((long)a0 + 0x58) = v2;
  if (v2) {
    sub_13690(v2);
    return 1;
  }
  return 0;
}

// Function: sub_97d0 @ 0x97d0
unsigned long sub_97d0(void *a0,struct_22 *a1) // return-dupe x2
{
  void *v1;
  struct_23 *v2; // rax
  struct_23 *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)((long)a0 + 0x48) & 0x102)) {
    v4 = sub_136b0(*(struct_21 **)((long)a0 + 0x58),&a1->field_0x70);
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
  v3 = (struct_23 *)sub_c720(v1,v2);
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

// Function: sub_9880 @ 0x9880
void sub_9880(void *a0,void *a1)
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
    v4 = (void *)sub_c780(*(void **)((long)a0 + 0x58),&v5);
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

// Function: sub_9940 @ 0x9940
bool sub_9940(void *a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)((long)a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)((long)a0 + 0x48) & 0x200) { // branch-flip
    sub_9690(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)((long)a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  while( true ) {
    if (sub_d730((void *)((long)a0 + 0x60))) break;
    v1 = sub_d780((void *)((long)a0 + 0x60));
    if (0 <= v1)
      close(v1);
  }
  return v2;
}

// Function: sub_99d0 @ 0x99d0
unsigned int sub_99d0(void *a0,void *a1,int a2,char *a3) // return-dupe
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
label_9b80:
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
      if (!(v7 & 2)) goto label_9b5f;
    }
    else {
      if (!(v7 & 0x200)) {
        v10 = 1;
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        goto label_9be4;
      }
      v4 = sub_d730((void *)((long)a0 + 0x60));
      if (v4) {
        v1 = *(unsigned int *)((long)a0 + 0x2c);
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        v10 = v4;
        goto label_9a75;
      }
      v5 = sub_d780((void *)((long)a0 + 0x60));
      v7 = *(unsigned int *)((long)a0 + 0x48);
      if (v5 < 0) {
        v10 = 1;
        v5 = v7;
        goto label_9c74;
      }
      v10 = 1;
      a2 = v5;
      v9 = v5;
      if (!(v7 & 2)) goto label_9ae0;
    }
label_9aa0:
    v5 = fstat(v9,&v8);
    if (!v5) {
      if ((*(long *)((long)a1 + 0x70) == v8._0_8_) && (*(long *)((long)a1 + 0x78) == v8._8_8_)) {
        v7 = *(unsigned int *)((long)a0 + 0x48);
        goto label_9ae0;
      }
      v6 = __errno_location();
      *v6 = 2;
    }
    v3 = -1;
  }
  else {
    v7 = *(unsigned int *)((long)a0 + 0x48);
    if (v7 & 4) goto label_9b80;
    if (0 <= a2) // branch-flip
      v10 = 0;
    else {
      v10 = 0;
      v5 = v7;
label_9c74:
      v5 &= 0x200;
      v1 = *(unsigned int *)((long)a0 + 0x2c);
      v7 = (v7 & 0x10) << 0xd | 0x90900;
      if (v5) { // branch-flip
label_9a75:
        v5 = sub_ea40(v1,a3,v7);
      }
      else {
label_9be4:
        v5 = sub_139b0(a3,v7);
      }
      if (v5 < 0) {
        v5 = -1;
        return v5;
      }
      v7 = *(unsigned int *)((long)a0 + 0x48);
      v9 = v5;
    }
    if (v7 & 2) goto label_9aa0;
    if (a3) {
label_9b5f:
      v5 = strcmp(a3,"..");
      if (!v5) goto label_9aa0;
    }
label_9ae0:
    if (v7 & 0x200) {
      sub_9690(a0,v9,v10 ^ 1);
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

// Function: sub_9cd0 @ 0x9cd0
void * sub_9cd0(unsigned long *a0,int a1)
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
  char *v21;
  unsigned long v22;
  bool v23;
  unsigned long v24; // stack - 0xa0
  void *v25; // stack - 0x98
  void *v26; // stack - 0x90
  char *v27; // stack - 0x70
  char *v28; // stack - 0x68
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
      v28 = (char *)0xffffffffffffffff;
label_9d32:
      v23 = 1;
      v16 = *(unsigned int *)&a0[9];
    }
    else {
      v28 = ",";
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
    v14 = sub_eac0(v6,*(char **)((long)v15 + 0x30),v7,&v18);
    *(long *)((long)v15 + 0x18) = v14;
    if (!v14) {
      if (a1 == 3) {
        *(unsigned short *)((long)v15 + 0x68) = 4;
        *(int *)((long)v15 + 0x40) = *__errno_location();
        return NULL;
      }
      return (void *)0;
    }
    if (*(short *)((long)v15 + 0x68) != 0xb) { // branch-flip
      if (*(unsigned char *)((long)a0 + 0x49) & 1) {
        sub_9880(a0,v15);
        sub_9200(a0,v15,0);
        if (!sub_97d0(a0,v15)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)((long)v15 + 0x68) = sub_9200(a0,v15,0);
    }
    v28 = (char *)((unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1);
    if (a1 == 2) {
label_a560:
      v23 = 0;
      v16 = *(unsigned int *)&a0[9];
      goto label_9d3b;
    }
    v7 = v18;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)((long)v15 + 0x80) != 2)) {
label_a205:
      v23 = a1 == 3;
      v3 = 1;
    }
    else {
      v14 = sub_9560(v15,v18);
      v7 = v18;
      if (v14 == 0x9fa0) goto label_a205;
      if (v14 <= 0x9fa0) {
        if ((v14) && (v14 != 0x6969)) goto label_a3a1;
        goto label_a205;
      }
      if ((v14 == 0x5346414f) || (v14 == 0xff534d42)) goto label_a205;
label_a3a1:
      if (a1 != 3) goto label_a560;
      v3 = 0;
      v23 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v7 = sub_13740(v7,0x406,3);
      v18 = v7;
    }
    if ((0 <= v7) && (!sub_99d0(a0,v15,v7,NULL))) goto label_9d32;
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
label_9d3b:
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
    v21 = NULL;
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
          goto label_a40e;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (v9->field_0x13 != '.')) || ((v9->field_0x14 && (*(short *)&v9->field_0x14 != 0x2e)))) break;
label_9e04:
        v20 = *(DIR **)((long)v15 + 0x18);
        if (!v20) goto label_a40e;
      }
      v10 = strlen(&v9->field_0x13);
      v11 = (void *)sub_94c0(a0,&v9->field_0x13,v10);
      if (!v11) {
label_a2a8:
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
        v4 = sub_96f0(a0,v29 + 2 + v10);
        if (!v4) goto label_a2a8;
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
      *(long *)((long)v11 + 0x58) = v14 + 1;
      v17 = *a0;
      *(unsigned long *)((long)v11 + 0x48) = v10 + v24;
      v16 = *(unsigned int *)&a0[9];
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
      if ((a0[8]) && (!(v16 & 0x400)))
        *(unsigned short *)((long)v11 + 0x68) = sub_9200(a0,v11,0);
      else {
        v1 = v9->field_0x12;
        v5 = (unsigned int)v1 - 1;
        if ((v16 & 8) && (v1 & 0xfb)) {
          if ((v16 & 0x10) || (v1 != 10)) {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
            if (0xc <= v5) {
              v6 = 0;
              v17 = 1;
              goto label_9ef8;
            }
            v17 = 1;
          }
          else {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
label_a068:
            v17 = 2;
          }
          v6 = *(unsigned int *)((unsigned long)v5 * 4 + 0x15a00);
        }
        else {
          *(unsigned short *)((long)v11 + 0x68) = 0xb;
          if (v5 <= 0xb) goto label_a068;
          v6 = 0;
          v17 = 2;
        }
label_9ef8:
        *(unsigned int *)((long)v11 + 0x88) = v6;
        *(unsigned long *)((long)v11 + 0xa0) = v17;
      }
      *(unsigned long *)((long)v11 + 0x10) = 0;
      if (v26) // branch-flip
        *(void **)((long)v25 + 0x10) = v11;
      else {
        v26 = v11;
      }
      if (v21 != (char *)0x2710) {
        v21 = &v21[1];
        if (v21 < v28) {
          v25 = v11;
          goto label_9e04;
        }
        break;
      }
      if (!a0[8]) {
        v13 = sub_9560(v15,v18);
        if (((v13 == 0x1021994) || (v13 == 0xff534d42)) || (v13 == 0x6969)) {
          v25 = v11;
          v21 = (char *)0x2711;
          v3 = 0;
          goto label_9e04;
        }
        v3 = 1;
      }
      v21 = (char *)0x2711;
      v20 = *(DIR **)((long)v15 + 0x18);
      v25 = v11;
    } while (v20);
label_a40e:
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
      if ((a0[6] == v24) || (!v21)) goto label_a42b;
label_a431:
      *v27 = 0;
    }
    if ((!v2) && (v23)) {
      if ((a1 != 1) && (v21)) goto label_a45c;
      goto label_a4b0;
    }
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v3 = 0;
      v26 = NULL;
      v21 = NULL;
label_a42b:
      v27 = &v27[-1];
      goto label_a431;
    }
    v26 = NULL;
    if (!(bool)(v23 & v2 == NULL)) goto label_a4d5;
    v3 = 0;
    v21 = NULL;
label_a4b0:
    if (*(long *)((long)v15 + 0x58)) // branch-flip
      v7 = sub_99d0(a0,*(void **)((long)v15 + 8),0xffffffff,"..");
    else {
      v7 = sub_9940(a0);
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
label_a4d5:
    if (((a1 == 3) && (*(short *)((long)v15 + 0x68) != 7)) && (*(short *)((long)v15 + 0x68) != 4))
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
label_a45c:
  if (v3) {
    a0[8] = sub_91e0;
    v15 = (void *)sub_93b0(a0,v26,v21);
    a0[8] = 0;
    return v15;
  }
  if (!a0[8])
    return v26;
  if (v21 == (char *)0x1)
    return v26;
  return (void *)sub_93b0(a0,v26,v21);
}

// Function: sub_a7c0 @ 0xa7c0
long * sub_a7c0(long *a0,unsigned long a1,long a2)
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
  v15 = sub_96f0(v4,v10);
  if (!v15) {
    free(v4); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v14 = (void *)sub_94c0(v4,"",0);
    if (v14) {
      *(unsigned long *)((long)v14 + 0x58) = 0xffffffffffffffff;
      v11 = (char *)*a0;
      goto label_a8c3;
    }
  }
  else {
    v14 = NULL;
    v11 = NULL;
label_a8c3:
    if (a2)
      v15 = (unsigned char)(*(unsigned int *)&v4[9] >> 10) & 1;
    if (v11) { // branch-flip
      v16 = NULL;
      v8 = NULL;
      v7 = 0;
label_a97d:
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
        v6 = (void *)sub_94c0(v4,v11,v10);
        if (!v6) goto label_aa0d;
        *(unsigned long *)((long)v6 + 0x58) = 0;
        *(void **)((long)v6 + 8) = v14;
        *(long *)((long)v6 + 0x30) = (long)v6 + 0x100;
        if ((v8) && (v15)) {
          *(unsigned long *)((long)v6 + 0xa0) = 2;
          *(unsigned short *)((long)v6 + 0x68) = 0xb;
          if (a2) { // branch-flip
label_a965:
            *(void **)((long)v6 + 0x10) = v8;
          }
          else {
            *(unsigned long *)((long)v6 + 0x10) = 0;
label_ab08:
            v16 = v6;
            *(void **)((long)v1 + 0x10) = v6;
            v6 = v8;
          }
          v11 = (char *)a0[v12 + 1];
          v8 = v6;
          v7 = v12 + 1;
          if (!v11) break;
          goto label_a97d;
        }
        *(unsigned short *)((long)v6 + 0x68) = sub_9200(v4,v6,0);
        if (a2) goto label_a965;
        *(unsigned long *)((long)v6 + 0x10) = 0;
        if (v8) goto label_ab08;
        v11 = (char *)a0[v12 + 1];
        v8 = v6;
        v7 = v12 + 1;
        v16 = v6;
      } while (v11);
      if ((a2) && (2 <= v12 + 1U))
        v6 = (void *)sub_93b0(v4,v6,v12 + 1U);
      v7 = sub_94c0(v4,"",0);
      *v4 = v7;
      v8 = v6;
      if (v7) {
        *(void **)(v7 + 0x10) = v6;
        *(unsigned short *)(v7 + 0x68) = 9;
        *(unsigned long *)(v7 + 0x58) = 1;
        if (sub_9760(v4)) {
label_aadf:
          v9 = *(unsigned int *)&v4[9];
          if (!(v9 & 0x204)) {
            if (v9 & 0x200) // branch-flip
              v3 = sub_ea40(*(unsigned int *)((long)v4 + 0x2c),".");
            else {
              v3 = sub_139b0(".",(v9 & 0x10) << 0xd | 0x90900);
            }
            *(int *)&v4[5] = v3;
            if (v3 <= -1)
              *(unsigned int *)&v4[9] = *(unsigned int *)&v4[9] | 4;
          }
          sub_d710((struct_28 *)&v4[0xc],0xffffffff);
          return v4;
        }
      }
label_aa0d:
      while (v8) {
        v1 = *(void **)((long)v8 + 0x10);
        if (*(DIR **)((long)v8 + 0x18))
          closedir(*(DIR **)((long)v8 + 0x18));
        free(v8);
        v8 = v1;
      }
    }
    else {
      v7 = sub_94c0(v4,"",0);
      *v4 = v7;
      if (v7) {
        *(unsigned long *)(v7 + 0x10) = 0;
        *(unsigned short *)(v7 + 0x68) = 9;
        *(unsigned long *)(v7 + 0x58) = 1;
        if (sub_9760(v4)) goto label_aadf;
      }
    }
    free(v14);
  }
  free((void *)v4[4]);
  free(v4);
  return NULL;
}

// Function: sub_abe0 @ 0xabe0
unsigned int sub_abe0(struct_19 *a0) // return-dupe
{
  long v1;
  void *v2;
  struct_7 *v3;
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
      goto label_ac9c;
    }
  }
  else if (!(a0->field_0x48 & 4)) {
    v7 = a0->field_0x28;
    if (fchdir(v7)) { // branch-flip
      v5 = __errno_location();
      v7 = *v5;
      v4 = close(a0->field_0x28);
      if ((v7) || (!v4)) goto label_ac9c;
    }
    else {
      v7 = a0->field_0x28;
      if (!close(v7)) goto label_ac99;
      v5 = __errno_location();
    }
    v7 = *v5;
    goto label_ac9c;
  }
label_ac99:
  v7 = 0;
label_ac9c:
  while (!sub_d730(&a0[1])) {
    v4 = sub_d780(&a0[1]);
    if (0 <= v4)
      close(v4);
  }
  if (a0->field_0x50)
    sub_c280(a0->field_0x50);
  v3 = a0->field_0x58;
  if (a0->field_0x48 & 0x102) { // branch-flip
    if (v3)
      sub_c280(v3);
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

// Function: sub_add0 @ 0xadd0
void * sub_add0(struct_26 *a0)
{
  short v1;
  char *v10; // rax
  unsigned long v11; // rax
  void *v12;
  char *v13; // rdi
  long v14;
  unsigned int v2;
  char *v3;
  void *v4;
  struct_7 *v5;
  long v6;
  short v7;
  int v8;
  void *v9;
  
  v9 = a0->field_0x0;
  if (!v9)
    return NULL;
  v2 = a0->field_0x48;
  if (v2 & 0x2000)
    return NULL;
  v7 = *(short *)((long)v9 + 0x6c);
  *(unsigned short *)((long)v9 + 0x6c) = 3;
  if (v7 == 1) {
    *(unsigned short *)((long)v9 + 0x68) = sub_9200(a0,v9,0);
    return v9;
  }
  v1 = *(short *)((long)v9 + 0x68);
  if (v7 != 2) { // branch-flip
    if (v1 != 1) { // branch-flip
label_ae58:
      do {
        v12 = *(void **)((long)v9 + 0x10);
        if (!v12) {
          v12 = *(void **)((long)v9 + 8);
          if (*(long *)((long)v12 + 0x18)) {
            v6 = *(long *)((long)v12 + 0x48);
            a0->field_0x0 = v12;
            *(char *)(a0->field_0x20 + v6) = 0;
            v12 = (void *)sub_9cd0(a0,3);
            if (v12) {
              free(v9);
              goto label_b0ea;
            }
            if (a0->field_0x48 & 0x2000)
              return NULL;
            v12 = *(void **)((long)v9 + 8);
          }
          a0->field_0x0 = v12;
          free(v9);
          if (*(long *)((long)v12 + 0x58) == -1) {
            free(v12);
            *__errno_location() = 0;
            a0->field_0x0 = NULL;
            return NULL;
          }
          if (*(short *)((long)v12 + 0x68) != 0xb) {
            *(char *)(a0->field_0x20 + *(long *)((long)v12 + 0x48)) = 0;
            if (*(long *)((long)v12 + 0x58)) { // branch-flip
              if (*(unsigned short *)((long)v12 + 0x6a) & 2) {
                v8 = *(int *)((long)v12 + 0x44);
                if (!(a0->field_0x48 & 4)) {
                  if (a0->field_0x48 & 0x200) { // branch-flip
                    sub_9690(a0,v8,1);
                    v8 = *(int *)((long)v12 + 0x44);
                  }
                  else {
                    if (fchdir(v8)) {
                      *(int *)((long)v12 + 0x40) = *__errno_location();
                      a0->field_0x48 = a0->field_0x48 | 0x2000;
                    }
                    v8 = *(int *)((long)v12 + 0x44);
                  }
                }
                close(v8);
                goto label_aec3;
              }
              if (*(unsigned short *)((long)v12 + 0x6a) & 1) goto label_aec3;
              v8 = sub_99d0(a0,*(void **)((long)v12 + 8),0xffffffff,"..");
            }
            else {
              v8 = sub_9940(a0);
            }
            if (v8) {
              *(int *)((long)v12 + 0x40) = *__errno_location();
              a0->field_0x48 = a0->field_0x48 | 0x2000;
            }
label_aec3:
            if (*(short *)((long)v12 + 0x68) != 2) {
              if (*(int *)((long)v12 + 0x40)) // branch-flip
                *(unsigned short *)((long)v12 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v12 + 0x68) = 6;
                sub_9880(a0,v12);
              }
            }
            if (!(a0->field_0x48 & 0x2000))
              return v12;
            return NULL;
          }
          abort(); // no-return, return-dupe
        }
        a0->field_0x0 = v12;
        free(v9);
        v9 = v12;
        if (!*(long *)((long)v12 + 0x58)) {
          if (sub_9940(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          v5 = a0->field_0x58;
          if (a0->field_0x48 & 0x102) { // branch-flip
            if (v5)
              sub_c280(v5);
          }
          else {
            free(v5);
          }
          v4 = (void *)a0->field_0x20;
          v3 = (char *)((long)v12 + 0x100);
          *(long *)((long)v12 + 0x48) = *(long *)((long)v12 + 0x60);
          memmove(v4,v3,*(long *)((long)v12 + 0x60) + 1);
          v10 = strrchr(v3,0x2f);
          if ((v10) && ((v3 != v10 || (*(char *)((long)v12 + 0x101))))) {
            v11 = strlen(&v10[1]);
            memmove(v3,&v10[1],v11 + 1);
            *(unsigned long *)((long)v12 + 0x60) = v11;
          }
          v6 = a0->field_0x20;
          *(long *)((long)v12 + 0x38) = v6;
          *(long *)((long)v12 + 0x30) = v6;
          sub_9760(a0);
          v7 = *(short *)((long)v12 + 0x68);
          goto label_b12b;
        }
      } while (*(short *)((long)v12 + 0x6c) == 4);
      if (*(short *)((long)v12 + 0x6c) == 2) {
        v7 = sub_9200(a0,v12,1);
        *(short *)((long)v12 + 0x68) = v7;
        if ((v7 == 1) && (v2 = a0->field_0x48, !(v2 & 4))) {
          if (v2 & 0x200) // branch-flip
            v8 = sub_ea40(a0->field_0x2c,".");
          else {
            v8 = sub_139b0(".",(v2 & 0x10) << 0xd | 0x90900);
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
      if (v7 == 4) goto label_afc2;
label_af27:
      if ((v2 & 0x40) && (*(long *)((long)v9 + 0x70) != a0->field_0x18)) {
label_afc2:
        if (*(unsigned char *)((long)v9 + 0x6a) & 2)
          close(*(int *)((long)v9 + 0x44));
        v12 = a0->field_0x8;
        if (v12) {
          do {
            v4 = *(void **)((long)v12 + 0x10);
            if (*(DIR **)((long)v12 + 0x18))
              closedir(*(DIR **)((long)v12 + 0x18));
            free(v12);
            v12 = v4;
          } while (v4);
          a0->field_0x8 = NULL;
        }
        *(unsigned short *)((long)v9 + 0x68) = 6;
        sub_9880(a0,v9);
        return v9;
      }
      if (a0->field_0x8) { // branch-flip
        if (v2 & 0x1000) {
          a0->field_0x48 = v2 & 0xffffefff;
          v12 = a0->field_0x8;
          do {
            v4 = *(void **)((long)v12 + 0x10);
            if (*(DIR **)((long)v12 + 0x18))
              closedir(*(DIR **)((long)v12 + 0x18));
            free(v12);
            v12 = v4;
          } while (v4);
          a0->field_0x8 = NULL;
          goto label_b21a;
        }
        v3 = *(char **)((long)v9 + 0x30);
        if (sub_99d0(a0,v9,0xffffffff,v3)) { // branch-flip
          v8 = *__errno_location();
          *(unsigned short *)((long)v9 + 0x6a) = *(unsigned short *)((long)v9 + 0x6a) | 1;
          *(int *)((long)v9 + 0x40) = v8;
          v12 = a0->field_0x8;
          if (v12) {
            v9 = v12;
            do {
              *(unsigned long *)((long)v9 + 0x30) = *(unsigned long *)(*(long *)((long)v9 + 8) + 0x30);
              v9 = *(void **)((long)v9 + 0x10);
            } while (v9);
          }
        }
        else {
          v12 = a0->field_0x8;
        }
      }
      else {
label_b21a:
        v12 = (void *)sub_9cd0(a0,3);
        a0->field_0x8 = v12;
        if (!v12) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)((long)v9 + 0x40)) && (*(short *)((long)v9 + 0x68) != 4))
              *(unsigned short *)((long)v9 + 0x68) = 7;
            sub_9880(a0,v9);
            return v9;
          }
          return NULL;
        }
      }
      a0->field_0x8 = NULL;
    }
label_b0ea:
    v6 = *(long *)(*(long *)((long)v12 + 8) + 0x48);
    v14 = v6 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v12 + 8) + 0x38) + -1 + v6) != '/')
      v14 = v6;
    v13 = (char *)(v14 + a0->field_0x20);
    *v13 = 0x2f;
    memmove(&v13[1],(void *)((long)v12 + 0x100),*(long *)((long)v12 + 0x60) + 1);
    v7 = *(short *)((long)v12 + 0x68);
    v9 = v12;
label_b12b:
    a0->field_0x0 = v9;
    if (v7 == 0xb) goto label_b1a8;
  }
  else {
    if (2 <= (unsigned short)(v1 - 0xcU)) {
      if (v1 == 1) goto label_af27;
      goto label_ae58;
    }
    v7 = sub_9200(a0,v9,1);
    *(short *)((long)v9 + 0x68) = v7;
    if (v7 == 1) {
      v2 = a0->field_0x48;
      if (v2 & 4) {
        a0->field_0x0 = v9;
        goto label_b13f;
      }
      if (v2 & 0x200) // branch-flip
        v8 = sub_ea40(a0->field_0x2c,".");
      else {
        v8 = sub_139b0(".",(v2 & 0x10) << 0xd | 0x90900);
      }
      *(int *)((long)v9 + 0x44) = v8;
      if (v8 < 0) {
        v8 = *__errno_location();
        *(unsigned short *)((long)v9 + 0x68) = 7;
        *(int *)((long)v9 + 0x40) = v8;
        a0->field_0x0 = v9;
        return v9;
      }
      *(unsigned short *)((long)v9 + 0x6a) = *(unsigned short *)((long)v9 + 0x6a) | 2;
      v7 = *(short *)((long)v9 + 0x68);
      goto label_b12b;
    }
    a0->field_0x0 = v9;
    if (v7 != 0xb)
      return v9;
label_b1a8:
    if (*(long *)((long)v9 + 0xa0) != 2) {
      if (*(long *)((long)v9 + 0xa0) == 1)
        return v9;
      abort();
    }
    v7 = sub_9200(a0,v9,0);
    *(short *)((long)v9 + 0x68) = v7;
  }
  if (v7 != 1)
    return v9;
label_b13f:
  if (!*(long *)((long)v9 + 0x58))
    a0->field_0x18 = *(long *)((long)v9 + 0x70);
  if (!sub_97d0(a0,v9)) {
    v9 = NULL;
    *__errno_location() = 0xc;
  }
  return v9;
}

// Function: sub_b520 @ 0xb520
unsigned long sub_b520(unsigned long a0,void *a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)((long)a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_b550 @ 0xb550
int * sub_b550(struct_36 *a0,unsigned int a1) // early-return x2
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
    v7 = (void *)sub_9cd0(a0,v8);
    a0->field_0x8 = v7;
  }
  else {
    if (v1 & 0x200) { // branch-flip
      v6 = sub_ea40(a0->field_0x2c,".");
      v9 = v6;
    }
    else {
      v6 = sub_139b0(".",(v1 & 0x10) << 0xd | 0x90900);
      v9 = v6;
    }
    if (v9 < 0) {
      a0->field_0x8 = NULL;
      v7 = NULL;
      return v7;
    }
    v7 = (void *)sub_9cd0(a0,v8);
    a0->field_0x8 = v7;
    if (a0->field_0x48 & 0x200) // branch-flip
      sub_9690(a0,v9,1);
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

// Function: sub_b750 @ 0xb750
unsigned long sub_b750(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_b770 @ 0xb770
bool sub_b770(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_b780 @ 0xb780
unsigned long sub_b780(struct_8 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x15aa0)
    return 1;
  v1 = v2[2];
  if ((((dat_15ab4 < v1) && (v1 < dat_15ab8)) && (dat_15abc < v2[3])) && (dat_15ad0 <= *v2)) {
    v3 = *v2 + dat_15ab4;
    if (((v3 < v2[1]) && (v2[1] <= dat_15ac0)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x15aa0;
  return 0;
}

// Function: sub_b810 @ 0xb810
unsigned long sub_b810(struct_10 *a0,long a1,unsigned long *a2,bool a3) // early-return
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

// Function: sub_b920 @ 0xb920
unsigned long sub_b920(float a0,unsigned long a1,bool a2) // ternary
{
  unsigned long v1; // rax
  unsigned long v2; // rcx
  unsigned long v3; // rsi
  long v4; // rdi
  unsigned long v5;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_15ac4 <= a0) {
      v1 = 0;
      return v1;
    }
    a1 = (a0 < dat_15ac8) ? (unsigned long)a0 : (long)(a0 - dat_15ac8) ^ 0x8000000000000000;
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
          if (v5 <= v3) goto label_b9f5;
          v4 += 8;
        } while (v5 % v2);
      }
    }
    else {
      v2 = 3;
label_b9f5:
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

// Function: sub_ba60 @ 0xba60
unsigned long sub_ba60(struct_11 *a0,struct_37 *a1,bool a2) // return-dupe x2
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
label_ba96:
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
        goto label_ba96;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}

// Function: sub_bbd0 @ 0xbbd0
unsigned long sub_bbd0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x10);
}

// Function: sub_bbe0 @ 0xbbe0
unsigned long sub_bbe0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x18);
}

// Function: sub_bbf0 @ 0xbbf0
unsigned long sub_bbf0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x20);
}

// Function: sub_bc00 @ 0xbc00
unsigned long sub_bc00(struct_37 *a0)
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

// Function: sub_bc60 @ 0xbc60
bool sub_bc60(struct_37 *a0)
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
label_bc80:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_bc80;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_bcd0 @ 0xbcd0
void sub_bcd0(unsigned long *a0,FILE *a1)
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
      if ((long *)a0[1] <= v4) goto label_bd3d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_bd3d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_15ad0) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_be30 @ 0xbe30
long sub_be30(struct_17 *a0,long a1) // return-dupe
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

// Function: sub_bea0 @ 0xbea0
long sub_bea0(struct_37 *a0)
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

// Function: sub_bef0 @ 0xbef0
long sub_bef0(struct_38 *a0,long a1)
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

// Function: sub_bf80 @ 0xbf80
void sub_bf80(struct_37 *a0,long a1,unsigned long a2)
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

// Function: sub_bfd0 @ 0xbfd0
long sub_bfd0(struct_37 *a0,void *a1,unsigned long a2)
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

// Function: sub_c050 @ 0xc050
unsigned long sub_c050(unsigned char *a0,unsigned long a1)
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

// Function: sub_c090 @ 0xc090
void sub_c090(struct_39 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_c0c0 @ 0xc0c0
unsigned long * sub_c0c0(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_b750;
  if (!a3)
    a3 = sub_b770;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x15aa0;
  v1[5] = a1;
  if (sub_b780(v1)) {
    v2 = sub_b920(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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

// Function: sub_c1d0 @ 0xc1d0
void sub_c1d0(struct_40 *a0)
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
        if (a0->field_0x8 <= v5) goto label_c26b;
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
label_c26b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_c280 @ 0xc280
void sub_c280(struct_7 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_c32c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_c2ef;
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
label_c2ef:
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
label_c32c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_c370 @ 0xc370
unsigned int sub_c370(struct_13 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_b920(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_ba60(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_ba60(a0,&v2,1)) && (sub_ba60(a0,&v2,0))) {
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

// Function: sub_c4f0 @ 0xc4f0
long * sub_c4f0(void *a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_b810(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_c57f;
label_c606:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)((long)a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_c606;
label_c57f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
    sub_b780(a0);
    v1 = *(long *)((long)a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_15ac4 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_15ac8 <= v5) ? (long)(v5 - dat_15ac8) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_c370(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_b810(a0,a1,&v3,0);
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

// Function: sub_c720 @ 0xc720
unsigned long sub_c720(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_c4f0(a0,a1,&v3);
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

// Function: sub_c780 @ 0xc780
long sub_c780(void *a0,long a1) // ternary x2
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
  
  v5 = sub_b810(a0,a1,&v7,1);
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
      sub_b780(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_15ac8 <= v8) ? (long)(v8 - dat_15ac8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_c370(a0,v6)) {
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

// Function: sub_c910 @ 0xc910
long sub_c910(void *a0)
{
  return sub_c780(a0); // tail-call
}

// Function: sub_c920 @ 0xc920
char * sub_c920(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary
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
        if (a2 & 0x10) goto label_cb9c;
label_cfab:
        v21 = -1;
label_cbaf:
        if (v33 != 1) goto label_cbbf;
        if (5 < (int)((((unsigned int)v15 & 1) + v25 != 0) + v20)) goto label_cbcf;
        goto label_cbe8;
      }
    }
label_ca04:
    v30 = (long double)(long)a3;
    if ((long)a3 < 0)
      v30 = v30 + (long double)dat_15ac4;
    if (0 <= (long)a4) { // branch-flip
      v30 = v30 / (long double)(long)a4;
      if ((long)a0 > -1) goto label_ca41;
label_ccf0:
      v30 = ((long double)(long)a0 + (long double)dat_15ac4) * v30;
      if (!(a2 & 0x10)) goto label_cd38;
label_ca4e:
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
      v30 = v30 / ((long double)(long)a4 + (long double)dat_15ac4);
      if (0 > (long)a0) goto label_ccf0;
label_ca41:
      v30 = (long double)(long)a0 * v30;
      if (a2 & 0x10) goto label_ca4e;
label_cd38:
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
    if ((!a3) || (v17 = a4 / a3, a4 % a3)) goto label_ca04;
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
    if (!(a2 & 0x10)) goto label_cfab;
label_cb9c:
    v20 = (unsigned int)v16;
    v15 = SUB168(v8._0_16_,0);
    v22 = (unsigned long)v11;
    v21 = 0;
    v17 = v15;
    if (v15 < v22) goto label_cbaf;
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
      if (v21 == 8) goto label_cbaf;
    }
    if (10 <= v15) goto label_cbaf;
    v17 = v15;
    if (v33 != 1) { // branch-flip
      if ((!v33) && (v25)) goto label_d337;
label_d3ac:
      if (!v20) goto label_d3b4;
      v10 = (char)v16 + '0';
    }
    else {
      if (3 > (v20 & 1) + v25) goto label_d3ac;
label_d337:
      if (v20 != 9) // branch-flip
        v10 = (char)v16 + '1';
      else {
        v17 = v15 + 1;
        if (v15 == 9) goto label_d37a;
        v25 = 0;
        v15 = v17;
label_d3b4:
        if (a2 & 8) goto label_d37c;
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
label_d37a:
    v25 = 0;
    v15 = v17;
label_d37c:
    v20 = 0;
    if (v33 == 1) goto label_cbe8;
label_cbbf:
    if (v33) { // branch-flip
label_cbe8:
      v29 = v24;
      do {
        v29 = &v29[-1];
        *v29 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
        v3 = 10 <= v15;
        v15 /= 10;
      } while (v3);
    }
    else {
      if (1 > (int)(v25 + v20)) goto label_cbe8;
label_cbcf:
      v15 += 1;
      if (((!(a2 & 0x10)) || (v11 != v15)) || (v21 == 8)) goto label_cbe8;
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
  if (!(a2 & 0x80)) goto label_ce4f;
  if (v21 != -1) { // branch-flip
label_ce3c:
    if (!(a2 & 0x100) && !v21) goto label_ce4f;
    if (a2 & 0x40) {
label_cff9:
      v32 = &a1[0x288];
      a1[0x287] = ' ';
    }
    if (v21) goto label_d018;
    v28 = v32;
    if (!(a2 & 0x100)) goto label_ce4f;
  }
  else {
    if (a4 < 2) {
      v21 = 0;
      goto label_ce3c;
    }
    v21 = 1;
    v14 = 1;
    do {
      v14 *= v11;
      if (a4 <= v14) break;
      v21 += 1;
    } while (v21 != 8);
    if (a2 & 0x40) goto label_cff9;
label_d018:
    v28 = &v32[1];
    if ((v19) || (v21 != 1)) {
      *v32 = *(char *)((long)v21 + 0x15b18);
      if (!(a2 & 0x100)) {
label_d30b:
        v32 = v28;
        goto label_ce4f;
      }
      if (v19) {
        v32[1] = 'i';
        v28 = &v32[2];
      }
    }
    else {
      *v32 = 'k';
      if (!(a2 & 0x100)) goto label_d30b;
    }
  }
  *v28 = 'B';
  v32 = &v28[1];
label_ce4f:
  *v32 = '\0';
  return v29;
}

// Function: sub_d520 @ 0xd520
unsigned long sub_d520(unsigned char *a0,unsigned int *a1,unsigned long *a2) // early-return x4
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
  v3 = sub_59b0(a0,(long *)0x1aa40,(void *)0x15b10,4);
  if (0 <= v3) {
    *a2 = 1;
    *a1 = v7 | *(unsigned int *)((long)v3 * 4 + 0x15b10);
    return 0;
  }
  v4 = sub_12910(a0,&v6,0,a2,"eEgGkKmMpPtTyYzZ0");
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

// Function: sub_d710 @ 0xd710
void sub_d710(struct_28 *a0,unsigned int a1)
{
  *(unsigned long *)a0->field_0x14 = 0;
  a0->field_0x1c = 1;
  a0->field_0x0 = a1;
  a0->field_0x4 = a1;
  a0->field_0x8 = a1;
  a0->field_0xc = a1;
  a0->field_0x10 = a1;
}

// Function: sub_d730 @ 0xd730
char sub_d730(void *a0)
{
  return *(char *)((long)a0 + 0x1c);
}

// Function: sub_d740 @ 0xd740
unsigned int sub_d740(void *a0,unsigned int a1)
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

// Function: sub_d780 @ 0xd780
unsigned int sub_d780(void *a0)
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

// Function: sub_d7c0 @ 0xd7c0
unsigned long sub_d7c0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_d7d0 @ 0xd7d0
unsigned long sub_d7d0(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_d7e0 @ 0xd7e0
long * sub_d7e0(long a0)
{
  long *v1; // rax
  long v2; // rax
  long *v3;
  
  v1 = malloc(0x18);
  v3 = v1;
  if (v1) {
    v2 = sub_c0c0(0x3fd,NULL,sub_d7c0,sub_d7d0,dat_1afb8);
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

// Function: sub_d850 @ 0xd850
void sub_d850(struct_9 *a0)
{
  sub_c280(a0->field_0x0);
  free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_d880 @ 0xd880
long sub_d880(unsigned long *a0,long a1)
{
  long v1;
  struct_15 *v2; // rax
  struct_15 *v3;
  
  v3 = (struct_15 *)a0[2];
  if (v3) { // branch-flip
    if (*(long *)v3->field_0x0 == a1)
      return v3->field_0x8;
  }
  else {
    v3 = malloc(0x10);
    a0[2] = v3;
    if (!v3)
      return -1;
  }
  *(long *)v3->field_0x0 = a1;
  v2 = (struct_15 *)sub_c720((void *)*a0,v3);
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

// Function: sub_d920 @ 0xd920
char * sub_d920(unsigned long a0,void *a1)
{
  unsigned long v1;
  char *v2;
  char *v3; // r8
  
  *(char *)((long)a1 + 0x14) = 0;
  v2 = (char *)((long)a1 + 0x14);
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

// Function: sub_d9c0 @ 0xd9c0
int sub_d9c0(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_d9f0 @ 0xd9f0
void sub_d9f0(struct_41 *a0,struct_42 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_42 *v3;
  
  v3 = (struct_42 *)a1->field_0x0;
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

// Function: sub_da50 @ 0xda50
unsigned int sub_da50(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x15b40) >> (a0 & 0x1f) & 1;
}

// Function: sub_da70 @ 0xda70
unsigned long sub_da70(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_13a30(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_db00 @ 0xdb00
void sub_db00(struct_18 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x15b40) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_db41;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","lib/mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_da70(&a0->field_0x24,v2,sub_10f10(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
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
      __assert_fail("*iter->cur.ptr == \'\\0\'","lib/mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (a0->field_0x24)
      __assert_fail("iter->cur.wc == 0","lib/mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0->field_0x20 = '\x01';
  if (mbsinit((mbstate_t *)&a0->field_0x1[3])) {
    a0->field_0x0 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
label_db41:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_dc70 @ 0xdc70
unsigned long sub_dc70(unsigned char *a0,unsigned char *a1) // early-return
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
  unsigned int v3; // eax
  int v4; // eax
  unsigned char v5;
  char v6 [4]; // stack - 0xb8
  char v7 [4]; // stack - 0x78
  unsigned long v8; // stack - 0xb4
  char v9; // stack - 0xac
  
  if (a1 == a0)
    return 0;
  if (__ctype_get_mb_cur_max() <= 1) {
    v2 = *(long *)__ctype_b_loc();
    while( true ) {
      v5 = *a0;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v5 * 2) & 1)
        v5 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v5 * 4);
      v1 = *a1;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v1 * 2) & 1)
        v1 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v1 * 4);
      if (!v5) break;
      a0 = &a0[1];
      a1 = &a1[1];
      if (v5 != v1)
        return (unsigned long)((unsigned int)v5 - (unsigned int)v1);
    }
    return (unsigned long)-(unsigned int)v1;
  }
  v6[0] = 0;
  v8 = 0;
  v7[0] = 0;
  v14 = 0;
  v10 = a0;
  v16 = a1;
  while( true ) {
    v9 = '\0';
    v15 = '\0';
    sub_db00(v6);
    if ((v12) && (!v13)) break;
    if (!v15)
      sub_db00(v7);
    if (v18) { // branch-flip
      if (!v19) {
        if (!v9) goto label_de57;
        goto label_de5f;
      }
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
        if (v17 <= v11) {
          if (0 <= memcmp(v10,v16,v17))
            return 1;
          return 0xffffffff;
        }
        if (memcmp(v10,v16,v11) <= 0)
          return 0xffffffff;
        return 1;
      }
      v4 = memcmp(v10,v16,v11);
      if (v4)
        return CONCAT44(dat_4,v4);
    }
    v10 = &v10[v11];
    v16 = &v16[v17];
  }
  if (v9) goto label_de7a;
label_de57:
  sub_db00(v6);
label_de5f:
  if ((!v12) || (v13))
    return 1;
label_de7a:
  if (!v15)
    sub_db00(v7);
  if ((v18) && (!v19))
    return 0;
  return 0xffffffff;
}

// Function: sub_def0 @ 0xdef0
void sub_def0(char *a0)
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

// Function: sub_df90 @ 0xdf90
unsigned long * sub_df90(void)
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
                  sub_def0(v5);
                  sub_def0(v3);
                  sub_def0(v18);
                  sub_def0(v4);
                  v7 = (unsigned long *)sub_11c80(0x38);
                  *v7 = sub_12160(v5);
                  v19 = 1;
                  v7[1] = sub_12160(v3);
                  v7[2] = sub_12160(v18);
                  v3 = (char *)sub_12160(v4);
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
label_e40b:
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
                          if (strcmp(v3,"cifs")) goto label_e40b;
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
      sub_71a0(v2);
      *v8 = v1;
      goto label_e577;
    }
    if (sub_71a0(v2) != -1) goto label_e256;
  }
  else {
    v9 = setmntent("/etc/mtab","r");
    if (!v9)
      return NULL;
    v21 = &v12;
    while (v7 = getmntent(v9), v7) {
      v19 = 1;
      v18 = hasmntopt(v7,"bind");
      v10 = (unsigned long *)sub_11c80(0x38);
      v11 = sub_12160((char *)*v7);
      v3 = (char *)v7[1];
      *v10 = v11;
      v11 = sub_12160(v3);
      v10[2] = 0;
      v3 = (char *)v7[2];
      v10[1] = v11;
      v3 = (char *)sub_12160(v3);
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
label_e256:
      *v21 = 0;
      return v12;
    }
  }
  v8 = __errno_location();
  v1 = *v8;
label_e577:
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

// Function: sub_e9f0 @ 0xe9f0
void sub_e9f0(struct_27 *a0)
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

// Function: sub_ea40 @ 0xea40
void sub_ea40(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_11510(openat(a0,a1,a2,v1));
}

// Function: sub_eac0 @ 0xeac0
DIR * sub_eac0(unsigned long a0,char *a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_ea40(a0,a1,a2 | 0x90900);
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

// Function: sub_eb20 @ 0xeb20
void sub_eb20(char *a0)
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
    dat_1b1b0 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_ebc0 @ 0xebc0
char * sub_ebc0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_13ac0();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x15cad;
      return (char *)0x15ca8;
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

// Function: sub_eca0 @ 0xeca0
long * sub_eca0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_ed10:
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
label_f1b6:
      v2 = 0;
      goto label_f170;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_f2d1;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_f170:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_f2d1;
      }
      goto label_f1b6;
    case 5:
      if (!v18) goto label_f111;
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
          v10 = (char *)sub_ebc0("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)0x157af) {
          v10 = (char *)sub_ebc0("\'",a4);
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
label_edfb:
  do {
label_ee0e:
    v23 = 0;
    a1 = v14;
label_ee18:
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
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_f780;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_f37c;
                default:
                  goto label_f053;
                case 7:
                  goto label_f367;
                case 8:
                  goto label_f336;
                case 9:
                  goto label_f403;
                case 10:
                  goto label_f35d;
                case 0xb:
                  goto label_f430;
                case 0xc:
                  goto label_f31b;
                case 0xd:
                  goto label_f26d;
                case 0x20:
                  goto label_f43d;
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
                  goto label_f006;
                case 0x23:
                  goto label_f3e0;
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
                  goto label_ef38;
                case 0x27:
                  goto label_f252;
                case 0x3f:
                  goto label_f211;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_f053;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_ef38;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_f824;
              goto label_f768;
            }
            if (v8 == 0x7d) goto label_f8f0;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_f74a;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_f768;
              goto label_f053;
            }
            if (v8 != 0x7e) goto label_f053;
label_f8a4:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_f0c9;
            }
            v21 = 0x7e;
            goto label_f8b2;
          }
          goto label_f03c;
        }
label_f780:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_f8a4;
                goto label_f050;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_f053;
                goto label_f768;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_f74a:
            if (a3 == 0xffffffffffffffff) goto label_f8ff;
label_f758:
            v5 = v22;
            if (a3 == 1) goto label_f3e5;
label_f768:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_f0c9;
          }
          if (v8 == 0x40) goto label_f050;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_ef38;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_f768;
label_f824:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_f84a;
            v17 = 0x5c;
            goto label_f272;
          }
label_f053:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_f5d3;
label_f060:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_f0ae;
        }
        switch(v8) {
          case 0:
            goto label_f36e;
          default:
label_f050:
            v22 = 0;
            goto label_f053;
          case 7:
label_f367:
            v17 = 0x61;
            goto label_f347;
          case 8:
label_f336:
            v17 = 0x62;
            goto label_f347;
          case 9:
            v22 = 0;
label_f403:
            v21 = 9;
            v17 = 0x74;
            goto label_f410;
          case 10:
label_f35d:
            v17 = 0x6e;
            break;
          case 0xb:
label_f430:
            v17 = 0x76;
            break;
          case 0xc:
label_f31b:
            v17 = 0x66;
label_f347:
            if (v25) {
              v2 = 1;
              goto label_f29f;
            }
            v24 = 0;
            goto label_efbe;
          case 0xd:
label_f26d:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_f43d:
            v21 = 0x20;
            goto label_ef38;
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
            goto label_f009;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_f3e5;
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
label_ef38:
            v8 = 0;
            goto label_ef48;
          case 0x27:
            v22 = 0;
            goto label_f252;
          case 0x3f:
            v22 = 0;
            goto label_f211;
          
        }
        goto label_f272;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_f8f0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_f758;
label_f8ff:
            v5 = v22;
            if (a2[1]) goto label_f768;
            goto label_f3e5;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_f006;
              goto label_f053;
            }
            goto label_f74a;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_f3e5;
          if (v33 == 1) goto label_f060;
label_f5d3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_da70(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_fb3b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_febf;
              goto label_feb5;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_f70d;
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
          goto label_fb3b;
        }
        if (v17 == 0x40) goto label_f053;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_f0c0;
        if ((unsigned long)v10 & 0xa4000000) goto label_f006;
        if (v17 != 0x5c) goto label_f053;
        if (a4 == 2) {
          if (!v25) goto label_f84a;
          goto label_f033;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_f410;
        }
label_f84a:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_f860;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_f4e2;
            goto label_f41b;
          }
label_f36e:
          if (!v25) {
            v22 = 0;
label_f37c:
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
                goto label_f0f3;
              }
label_fc85:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_fc85;
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
label_fcbc:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_f0f3;
              goto label_ef48;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_f0f3;
          }
          if (a4 != 2) goto label_f03c;
          goto label_f2ab;
        default:
          goto label_f053;
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
          goto label_f49f;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_f49f;
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
label_f49f:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_f410;
label_f021:
          a4 = 2;
label_f033:
          if (!v28) goto label_f03c;
          goto label_f2ab;
        case 0x20:
          v19 = 0x20;
          goto label_f009;
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
label_f006:
          v26 = 0;
label_f009:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_f021;
          goto label_f0c7;
        case 0x23:
label_f3e0:
          v21 = 0x23;
label_f3e5:
          if (!v23) {
            v19 = v21;
            goto label_f009;
          }
label_f8b2:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_f0c9;
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
          goto label_f0c0;
        case 0x27:
label_f252:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_f0c9;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_fcdb;
            }
            else {
label_fcdb:
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
            goto label_f0f3;
          }
          goto label_f033;
        case 0x3f:
label_f211:
          if (a4 == 2) {
            if (v25) goto label_f033;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_f0f3;
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
                goto label_f0c9;
              }
              if (v25) goto label_f03c;
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
              goto label_fcbc;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_f0c9;
        
      }
label_f410:
      if (!v2) {
label_f41b:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_f0f0;
        goto label_ef48;
      }
label_f272:
      v24 = 0;
      goto label_f283;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_ff7b:
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
        if (a1 || !v32) goto label_ff7b;
        v23 = v32;
        goto label_fa0b;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_1002b;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_ee0e;
    }
label_f70d:
    if (v2) {
label_f2ab:
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
label_f2d1:
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
label_fa0b:
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
label_feb5:
    if (!a2[(long)v10]) break;
  }
label_febf:
  v24 = 0;
label_fb3b:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_fb45:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_f033;
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
        if (v21 <= v23) goto label_efd1;
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
          goto label_f860;
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
label_f0ae:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_fb45;
  }
label_f0c0:
  v21 = v19;
  v24 = v26;
label_f0c7:
  v8 = a4 == 2;
  v26 = v3;
label_f0c9:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_ef48:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_f0f3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_f0f3;
  }
  else {
label_f0f0:
    v8 = 0;
    v3 = v26;
label_f0f3:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_f860:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_efd1;
    }
  }
label_f283:
  if (v25) {
label_f29f:
    if ((bool)(v2 & a4 == 2)) goto label_f2ab;
label_f03c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_ed10;
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
label_efbe:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_efd1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_ee18;
label_f4e2:
  v23 += 1;
  goto label_ee18;
label_1002b:
  a1 = v32;
label_f111:
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
  goto label_edfb;
}

// Function: sub_10130 @ 0x10130
char * sub_10130(unsigned int a0,char *a1,unsigned long a2,struct_2 *a3)
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
    v5 = dat_1b080;
    if (dat_1b078 <= (int)a0) {
      v10 = (long)dat_1b078;
      v8 = (long)(int)((a0 - dat_1b078) + 1);
      if (dat_1b080 != (unsigned long *)0x1b090) // branch-flip
        v5 = (unsigned long *)sub_11f60(dat_1b080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_11f60(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_1b090;
        v5[1] = dat_1b098;
      }
      dat_1b080 = v5;
      memset(&v5[(long)dat_1b078 * 2],0,(v10 - dat_1b078) * 0x10);
      dat_1b078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_eca0(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x1b1c0)
        free(v7);
      v7 = (char *)sub_11cc0(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_eca0(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_10320 @ 0x10320
void sub_10320(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x1b2c0;
  sub_120c0(a0,0x38);
  *v2 = v1;
}

// Function: sub_10360 @ 0x10360
unsigned int sub_10360(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x1b2c0;
  return *a0;
}

// Function: sub_10380 @ 0x10380
void sub_10380(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x1b2c0;
  *a0 = a1;
}

// Function: sub_103a0 @ 0x103a0
unsigned int sub_103a0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x1b2c0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_103e0 @ 0x103e0
unsigned int sub_103e0(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0x1b2c0;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_10400 @ 0x10400
void sub_10400(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x1b2c0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_10440 @ 0x10440
void sub_10440(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x1b2c0;
  v2 = __errno_location();
  v1 = *v2;
  sub_eca0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_104c0 @ 0x104c0
char * sub_104c0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x1b2c0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_eca0(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_11cc0(v3 + 1U);
  sub_eca0(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_105b0 @ 0x105b0
char * sub_105b0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_104c0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_105c0 @ 0x105c0
void sub_105c0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_1b080;
  if (2 <= dat_1b078) {
    v4 = (unsigned long *)((long)dat_1b080 + 0x18);
    v1 = (unsigned long *)((long)dat_1b080 + (unsigned long)(unsigned int)(dat_1b078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x1b1c0) {
    free(*(void **)((long)v3 + 8));
    dat_1b098 = 0x1b1c0;
    dat_1b090 = 0x100;
  }
  if (v3 == (void *)0x1b090) {
    dat_1b078 = 1;
    return;
  }
  free(v3);
  dat_1b078 = 1;
  dat_1b080 = (void *)0x1b090;
}

// Function: sub_10660 @ 0x10660
char * sub_10660(unsigned int a0,char *a1)
{
  return sub_10130(a0,a1,0xffffffffffffffff,(struct_2 *)0x1b2c0); // tail-call
}

// Function: sub_10680 @ 0x10680
char * sub_10680(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_10130(a0,a1,a2); // tail-call
}

// Function: sub_10690 @ 0x10690
char * sub_10690(char *a0)
{
  return sub_10130(0,a0,0xffffffffffffffff,(struct_2 *)0x1b2c0); // tail-call
}

// Function: sub_106b0 @ 0x106b0
char * sub_106b0(char *a0,unsigned long a1)
{
  return sub_10130(0,a0,a1,(struct_2 *)0x1b2c0); // tail-call
}

// Function: sub_106d0 @ 0x106d0
void sub_106d0(unsigned int a0,int a1,char *a2)
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
    sub_10130(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10770 @ 0x10770
void sub_10770(unsigned int a0,int a1,char *a2,unsigned long a3)
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
    sub_10130(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10800 @ 0x10800
void sub_10800(int a0,char *a1)
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
    sub_10130(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10890 @ 0x10890
void sub_10890(int a0,char *a1,unsigned long a2)
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
    sub_10130(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10920 @ 0x10920
void sub_10920(char *a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_1b2c0;
  v4 = dat_1b2c8;
  v9 = dat_1b2f0;
  v5 = dat_1b2d0;
  v6 = dat_1b2d8;
  v7 = dat_1b2e0;
  v8 = dat_1b2e8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_10130(0,a0,a1,&v3);
}

// Function: sub_109c0 @ 0x109c0
void sub_109c0(char *a0,unsigned char a1)
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
  
  v3 = dat_1b2c0;
  v4 = dat_1b2c8;
  v9 = dat_1b2f0;
  v5 = dat_1b2d0;
  v6 = dat_1b2d8;
  v7 = dat_1b2e0;
  v8 = dat_1b2e8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_10130(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_10a60 @ 0x10a60
void sub_10a60(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_1b2c0;
  v3 = (unsigned int)((unsigned long)dat_1b2c8 >> 0x20);
  v8 = dat_1b2f0;
  v4 = dat_1b2d0;
  v5 = dat_1b2d8;
  v6 = dat_1b2e0;
  v7 = dat_1b2e8;
  v2 = (unsigned int)dat_1b2c8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_10130(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_10af0 @ 0x10af0
void sub_10af0(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_1b2c0;
  v3 = (unsigned int)((unsigned long)dat_1b2c8 >> 0x20);
  v8 = dat_1b2f0;
  v4 = dat_1b2d0;
  v5 = dat_1b2d8;
  v6 = dat_1b2e0;
  v7 = dat_1b2e8;
  v2 = (unsigned int)dat_1b2c8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_10130(0,a0,a1,&v1);
}

// Function: sub_10b80 @ 0x10b80
void sub_10b80(unsigned int a0,int a1,char *a2)
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
    sub_10130(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10c20 @ 0x10c20
void sub_10c20(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1b2c8;
  v4 = dat_1b2d0;
  v5 = dat_1b2d8;
  v8 = dat_1b2f0;
  v2 = (unsigned int)((unsigned long)dat_1b2c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1b2e0;
  v7 = dat_1b2e8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_10130(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10cc0 @ 0x10cc0
void sub_10cc0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1b2c8;
  v4 = dat_1b2d0;
  v5 = dat_1b2d8;
  v8 = dat_1b2f0;
  v2 = (unsigned int)((unsigned long)dat_1b2c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1b2e0;
  v7 = dat_1b2e8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_10130(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10d60 @ 0x10d60
void sub_10d60(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1b2c8;
  v4 = dat_1b2d0;
  v5 = dat_1b2d8;
  v8 = dat_1b2f0;
  v2 = (unsigned int)((unsigned long)dat_1b2c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1b2e0;
  v7 = dat_1b2e8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_10130(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10e00 @ 0x10e00
void sub_10e00(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1b2c8;
  v4 = dat_1b2d0;
  v5 = dat_1b2d8;
  v8 = dat_1b2f0;
  v2 = (unsigned int)((unsigned long)dat_1b2c0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1b2e0;
  v7 = dat_1b2e8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_10130(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10ea0 @ 0x10ea0
char * sub_10ea0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_10130(a0,a1,a2); // tail-call
}

// Function: sub_10eb0 @ 0x10eb0
char * sub_10eb0(char *a0,unsigned long a1)
{
  return sub_10130(0,a0,a1,(struct_2 *)0x1b040); // tail-call
}

// Function: sub_10ed0 @ 0x10ed0
char * sub_10ed0(unsigned int a0,char *a1)
{
  return sub_10130(a0,a1,0xffffffffffffffff,(struct_2 *)0x1b040); // tail-call
}

// Function: sub_10ef0 @ 0x10ef0
char * sub_10ef0(char *a0)
{
  return sub_10130(0,a0,0xffffffffffffffff,(struct_2 *)0x1b040); // tail-call
}

// Function: sub_10f10 @ 0x10f10
void * sub_10f10(void *a0,unsigned long a1)
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

// Function: sub_10f50 @ 0x10f50
unsigned long sub_10f50(long *a0)
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

// Function: sub_10fe0 @ 0x10fe0
unsigned long * sub_10fe0(char *a0)
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

// Function: sub_11080 @ 0x11080
unsigned long sub_11080(long *a0,struct_0 *a1) // return-dupe
{
  struct_0 *v1;
  long *v2;
  int v3; // eax
  unsigned long v4; // rax
  long v5; // rax
  char *v6;
  
  v1 = (struct_0 *)a1->field_0x30;
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[1]))
    return 1;
  v6 = (char *)((long)a0 + 9);
  if (!v1->field_0x0[0]) {
    v6 = "";
label_11118:
    a1->field_0x30 = v6;
    return 1;
  }
  do {
    v3 = strcmp(v6,(char *)v1);
    while( true ) {
      if (!v3) goto label_11118;
      if ((!*v6) && ((v6 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        v4 = strlen((char *)v1);
        if ((long)a0 + (0x80U - (long)v6) <= (long)(v4 + 1)) { // branch-flip
          v5 = sub_10fe0(v1);
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
        goto label_11118;
      }
      v6 = &v6[strlen(v6) + 1];
      if ((*v6) || (v2 = (long *)*a0, !v2)) break;
      v6 = (char *)((long)v2 + 9);
      v3 = strcmp(v6,(char *)v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_111b0 @ 0x111b0
long * sub_111b0(void *a0)
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
  v4 = (long *)sub_10fe0(v3);
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

// Function: sub_112b0 @ 0x112b0
void sub_112b0(long *a0) // return-dupe
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

// Function: sub_112f0 @ 0x112f0
struct_0 * sub_112f0(long *a0,void *a1,struct_0 *a2) // return-dupe x3
{
  long *v1; // rax
  
  if (!a0)
    return (struct_0 *)gmtime_r(a1,(tm *)a2); // tail-call
  v1 = (long *)sub_111b0(a0);
  if (!v1)
    return NULL;
  if ((!localtime_r(a1,(tm *)a2)) || (!sub_11080(a0,a2))) {
    if (v1 == (long *)0x1)
      return NULL;
    sub_10f50(v1);
    return NULL;
  }
  if ((v1 != (long *)0x1) && (!sub_10f50(v1)))
    return NULL;
  return a2;
}

// Function: sub_113a0 @ 0x113a0
unsigned long sub_113a0(long *a0,unsigned long *a1) // return-dupe x3
{
  long *v1; // rax
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
    return sub_114f0(a1); // tail-call
  v1 = (long *)sub_111b0(a0);
  if (!v1)
    return 0xffffffffffffffff;
  v3 = *a1;
  v7 = -1;
  v4 = a1[1];
  v5 = a1[2];
  v8 = *(unsigned int *)&a1[4];
  v2 = sub_135f0(&v3);
  if ((v7 <= -1) || (!sub_11080(a0,&v3))) {
    if (v1 == (long *)0x1)
      return 0xffffffffffffffff;
    sub_10f50(v1);
    return 0xffffffffffffffff;
  }
  if ((v1 != (long *)0x1) && (!sub_10f50(v1)))
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

// Function: sub_114f0 @ 0x114f0
unsigned long sub_114f0(struct_1 *a0)
{
  unsigned long v1; // rax
  
  a0->field_0x20 = 0;
  v1 = sub_13040(a0,dat_1afd8,(long *)0x1b2f8); // tail-call
  return v1;
}

// Function: sub_11510 @ 0x11510
unsigned int sub_11510(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_13bc0(a0);
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_11570 @ 0x11570
long sub_11570(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_1169d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_116d6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x16388) + 0x16388;
  switch(a5) {
    case 0:
      goto label_116d6;
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
      goto label_1169d;
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

// Function: sub_119f0 @ 0x119f0
void sub_119f0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_11570(); // tail-call
}

// Function: sub_11a10 @ 0x11a10
void sub_11a10(FILE *a0,char *a1,char *a2,char *a3,struct_43 *a4)
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
  sub_11570(a0,a1,a2,a3);
}

// Function: sub_11ab0 @ 0x11ab0
void sub_11ab0(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_11570(a0,a1,a2);
}

// Function: sub_11b90 @ 0x11b90
void sub_11b90(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_11c40 @ 0x11c40
void sub_11c40(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_12180(); // no-return
}

// Function: sub_11c80 @ 0x11c80
void sub_11c80(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_12180(); // no-return
}

// Function: sub_11ca0 @ 0x11ca0
void sub_11ca0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_12180(); // no-return
}

// Function: sub_11cc0 @ 0x11cc0
void sub_11cc0(void)
{
  sub_11c80(); // tail-call
}

// Function: sub_11cd0 @ 0x11cd0
void sub_11cd0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_12180(); // no-return
}

// Function: sub_11d10 @ 0x11d10
void sub_11d10(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_12180(); // no-return
}

// Function: sub_11d40 @ 0x11d40
void sub_11d40(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_12180(); // no-return
}

// Function: sub_11d80 @ 0x11d80
void sub_11d80(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_12180(); // no-return
}

// Function: sub_11dc0 @ 0x11dc0
void sub_11dc0(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_12180(); // no-return
}

// Function: sub_11df0 @ 0x11df0
void sub_11df0(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_12180(); // no-return
}

// Function: sub_11e40 @ 0x11e40
void sub_11e40(void *a0,unsigned long *a1)
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
  sub_12180(); // no-return
}

// Function: sub_11ed0 @ 0x11ed0
void sub_11ed0(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_12180(); // no-return
}

// Function: sub_11f60 @ 0x11f60
void sub_11f60(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_1201e;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_1201e:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_12180(); // no-return
}

// Function: sub_12060 @ 0x12060
void sub_12060(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_12180(); // no-return
}

// Function: sub_12080 @ 0x12080
void sub_12080(unsigned long a0)
{
  sub_12060(a0,1); // tail-call
}

// Function: sub_12090 @ 0x12090
void sub_12090(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_12180(); // no-return
}

// Function: sub_120b0 @ 0x120b0
void sub_120b0(unsigned long a0)
{
  sub_12090(a0,1); // tail-call
}

// Function: sub_120c0 @ 0x120c0
void sub_120c0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_11c80(a1),a0,a1); // tail-call
}

// Function: sub_120f0 @ 0x120f0
void sub_120f0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_11ca0(a1),a0,a1); // tail-call
}

// Function: sub_12120 @ 0x12120
void sub_12120(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_11ca0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_12160 @ 0x12160
void sub_12160(char *a0)
{
  sub_120c0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_12180 @ 0x12180
void sub_12180(void)
{
  error(dat_1b028,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_121c0 @ 0x121c0
void sub_121c0(long *a0,unsigned int a1,long a2)
{
  if (sub_a7c0(a0,a1 | 0x200,a2))
    return;
  if (*__errno_location() != 0x16)
    sub_12180(); // no-return
  __assert_fail("errno != EINVAL","lib/xfts.c",0x29,"xfts_open"); // no-return
}

// Function: sub_12210 @ 0x12210
bool sub_12210(struct_30 *a0,struct_29 *a1)
{
  unsigned int v1; // eax
  bool v2; // r8b
  
  v2 = 1;
  v1 = a0->field_0x48 & 0x11;
  if ((v1 != 0x10) && (v2 = 0, v1 == 0x11))
    return a1->field_0x58 != 0;
  return v2;
}

// Function: sub_12250 @ 0x12250
unsigned int sub_12250(unsigned char *a0,long *a1,unsigned int a2,long *a3,char *a4) // ternary x6
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoimax"); // no-return
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
    if (!a4) goto label_122db;
    v1 = *v3;
    if (!v1) goto label_122db;
    if (!strchr(a4,(int)(char)v1)) goto label_122e2;
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
    if (!strchr(a4,0x30)) goto label_1238c;
    v2 = v3[1];
    if (v2 == 0x44) {
label_12738:
      v13 = 2;
      v8 = 1000;
      goto label_1238c;
    }
    if (v2 == 0x69) {
      v13 = (v3[2] == 0x42) + 1 + (unsigned int)(v3[2] == 0x42);
      goto label_1238c;
    }
    if (v2 == 0x42) goto label_12738;
    switch(v11) {
      case 0:
        v14 = 1;
        v8 = 0x400;
        goto label_125bd;
      default:
        goto label_122e2;
      case 2:
      case 0x22:
        v14 = 1;
        v8 = 0x400;
        goto label_124fb;
      case 6:
      case 0x26:
        v14 = 1;
        v8 = 0x400;
        goto label_124d8;
      case 8:
      case 0x28:
        v14 = 1;
        v8 = 0x400;
        goto label_124b8;
      case 0xb:
        v14 = 1;
        v8 = 0x400;
        goto label_1257b;
      case 0xf:
      case 0x2f:
        v14 = 1;
        v8 = 0x400;
        goto label_1245e;
      case 0x14:
        v14 = 1;
        v8 = 0x400;
        goto label_12645;
      case 0x15:
        v14 = 1;
        v8 = 0x400;
        goto label_12605;
      case 0x1d:
        v14 = 1;
        goto label_12685;
      case 0x1e:
        v14 = 1;
      
    }
    goto label_1249b;
  }
label_1238c:
  switch((unsigned int)v1 - 0x42 & 0xff) {
    case 0:
      v7 = v5 * 0x400;
      v14 = (long)v13;
      if (SEXT816(v7) == SEXT816(v5) * SEXT416(0x400)) goto label_124d0;
label_12546:
      v14 = (long)v13;
      if (v5 > -1) goto label_124ec;
label_12557:
      v15 = 1;
      v5 = -0x8000000000000000;
      goto label_1249b;
    default:
label_122e2:
      *a3 = v5;
      return v15 | 2;
    case 3:
      v14 = (long)v13;
label_125bd:
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
label_124fb:
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
label_124d8:
      v7 = v8 * v5;
      if (SEXT816(v7) == SEXT816((long)v8) * SEXT816(v5)) goto label_124d0;
label_124e1:
      if (0 > v5) goto label_12557;
label_124ec:
      v15 = 1;
      v5 = 0x7fffffffffffffff;
      goto label_1249b;
    case 0xb:
    case 0x2b:
      v14 = (long)v13;
label_124b8:
      v6 = (long)v8;
      v9 = v5 * v6;
      if (SEXT816(v9) != SEXT816(v5) * SEXT816(v6)) goto label_124e1;
      v7 = v6 * v9;
      if (SEXT816(v7) == SEXT816(v6) * SEXT816(v9)) goto label_124d0;
      v15 = 1;
      v5 = -0x8000000000000000;
      if (0 <= v9)
        v5 = 0x7fffffffffffffff;
      goto label_1249b;
    case 0xe:
      v14 = (long)v13;
label_1257b:
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
label_1245e:
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
label_12645:
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
label_12605:
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
label_12685:
      v7 = v5 * 0x200;
      if (SEXT816(v7) != SEXT816(v5) * SEXT416(0x200)) goto label_124e1;
      goto label_124d0;
    case 0x21:
      v14 = (long)v13;
      goto label_1249b;
    case 0x35:
      v7 = v5 * 2;
      v14 = (long)v13;
      if (SEXT816(v7) != SEXT816(v5) * SEXT816(2)) goto label_12546;
label_124d0:
      v5 = v7;
      goto label_1249b;
    
  }
  v15 |= v12;
label_1249b:
  *a1 = (long)&v3[v14];
  if (v3[v14])
    v15 |= 2;
label_122db:
  *a3 = v5;
  return v15;
}

// Function: sub_12850 @ 0x12850
void sub_12850(unsigned int a0,int a1,char a2,long a3,unsigned long a4)
{
  int v1;
  char v2; // stack - 0x32
  char *v3; // r9
  char *v4; // r12
  char *v5;
  long v6; // fs_offset
  char v7; // stack - 0x31
  unsigned long v8; // stack - 0x30
  
  v1 = dat_1b028;
  v8 = *(unsigned long *)(v6 + 0x28);
  if (4 <= a0) { // branch-flip
    if (a0 != 4) {
      abort(); // no-return, return-dupe
    }
    v3 = "invalid %s%s argument \'%s\'";
    if (a1 <= -1) goto label_128e3;
label_12891:
    v4 = "--";
    v5 = *(char **)(a3 + (long)a1 * 0x20);
  }
  else {
    if (2 <= a0) // branch-flip
      v3 = "invalid suffix in %s%s argument \'%s\'";
    else {
      if (a0 != 1) {
        abort();
      }
      v3 = "%s%s argument \'%s\' too large";
    }
    if (0 <= a1) goto label_12891;
label_128e3:
    v5 = &v2;
    v7 = 0;
    v4 = &"--"[-(long)a1];
    v2 = a2;
  }
  error(v1,0,dcgettext(NULL,v3,5),v4,v5,a4);
  abort();
}

// Function: sub_12910 @ 0x12910
unsigned int sub_12910(unsigned char *a0,long *a1,unsigned int a2,unsigned long *a3,char *a4)
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
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
    if (!a4) goto label_129cd;
    v9 = *v6;
    if (!v9) goto label_129cd;
    v10 = v7;
    if (!strchr(a4,(int)(char)v9)) goto label_129d5;
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
label_129d5:
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
      goto label_12bd4;
    case 5:
    case 0x25:
      if ((SUB168(ZEXT816(v10) * ZEXT816(v8),8)) || (v2 = ZEXT816(SUB168(ZEXT816(v10) * ZEXT816(v8),0)) * ZEXT816(v8), v7 = SUB168(v2,0), SUB168(v2,8))) goto label_12b99;
label_12b67:
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
      if (SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_12b99;
      break;
    case 0xb:
    case 0x2b:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (!SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_12b67;
label_12b99:
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
      goto label_12bd4;
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
      goto label_12bd4;
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
      goto label_12bd4;
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
label_12bd4:
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
label_129cd:
  *a3 = v7;
  return v14;
}

// Function: sub_12d40 @ 0x12d40
long sub_12d40(unsigned long a0,long a1,int a2,int a3,int a4,unsigned int a5,int a6,int a7,int a8,int a9)
{
  int v1; // eax
  int v2; // ecx
  int v3; // edx
  int v4; // r9d
  
  v1 = ((int)((long)a0 >> 2) + 0x1db) - (unsigned int)((a0 & 3) == 0);
  v4 = ((int)((long)(int)a5 >> 2) + 0x1db) - (unsigned int)((a5 & 3) == 0);
  v3 = (v1 - (v1 >> 0x1f)) / 0x19 + (v1 >> 0x1f);
  v2 = (v4 - (v4 >> 0x1f)) / 0x19 + (v4 >> 0x1f);
  return ((long)a4 + (((long)a3 + (((long)a2 + ((long)(((v1 - v4) - (v3 - v2)) + ((v3 >> 2) - (v2 >> 2))) + (((a0 - (long)(int)a5) * 0x16d + a1) - (long)a6)) * 0x18) - (long)a7) * 0x3c) - (long)a8) * 0x3c) - (long)a9;
}

// Function: sub_12e40 @ 0x12e40
int * sub_12e40(void *a0,long *a1,int *a2)
{
  int *v1; // rax
  int v10; // stack - 0x80
  int v11; // stack - 0x7c
  int v12; // stack - 0x78
  int v13; // stack - 0x74
  int v14; // stack - 0x70
  int v15; // stack - 0x6c
  unsigned long v16; // stack - 0x68
  int *v17; // stack - 0x60
  long v2; // stack - 0x48
  long v3;
  long v4;
  long v5;
  int v6; // stack - 0x94
  int v7; // stack - 0x90
  int v8; // stack - 0x8c
  unsigned long v9; // stack - 0x88
  
  v5 = *a1;
  v2 = v5;
  v17 = (int *)(*a0)(&v2,a2);
  if (v17) // branch-flip
    *a1 = v5;
  else {
    v1 = __errno_location();
    if (((*v1 == 0x4b) && (v3 = (unsigned long)((unsigned int)v5 & 1) + (v5 >> 1), v5 != v3)) && (v3)) {
      v4 = 0;
      v6 = -1;
      do {
        v2 = v3;
        if ((*a0)(&v2,a2)) { // branch-flip
          v6 = *a2;
          v15 = a2[1];
          v14 = a2[2];
          v13 = a2[3];
          v12 = a2[4];
          v8 = a2[5];
          v10 = a2[6];
          v11 = a2[7];
          v7 = a2[8];
          v9 = *(unsigned long *)&a2[10];
          v16 = *(unsigned long *)&a2[0xc];
          v4 = v3;
        }
        else {
          if (*v1 != 0x4b)
            return NULL;
          v5 = v3;
        }
        v3 = (v4 >> 1) + (v5 >> 1) + (unsigned long)(((unsigned int)v4 | (unsigned int)v5) & 1);
      } while ((v3 != v4) && (v3 != v5));
      if (0 <= v6) {
        *a1 = v4;
        *a2 = v6;
        a2[1] = v15;
        a2[2] = v14;
        a2[3] = v13;
        a2[4] = v12;
        a2[5] = v8;
        a2[6] = v10;
        a2[7] = v11;
        a2[8] = v7;
        *(unsigned long *)&a2[10] = v9;
        *(unsigned long *)&a2[0xc] = v16;
        v17 = a2;
      }
    }
  }
  return v17;
}

// Function: sub_13040 @ 0x13040
unsigned long sub_13040(int *a0,void *a1,long *a2)
{
  int v1;
  unsigned long v10;
  int v11;
  int v12; // stack - 0xb8
  unsigned long v13; // stack - 0xd0
  unsigned long v14; // stack - 0x78
  long v15; // stack - 0xc8
  unsigned long v16; // stack - 0xc0
  unsigned long v17; // rbp
  long v18; // rax
  int v19; // eax
  int v2;
  long v20;
  int v21; // r12d
  int v22;
  int v23;
  bool v24;
  bool v25; // zf
  unsigned long v26; // stack - 0x150
  int v27; // stack - 0x130
  unsigned int v28; // stack - 0xb4
  unsigned int v29; // stack - 0xb0
  int v3;
  unsigned int v30; // stack - 0xac
  unsigned int v31; // stack - 0xa8
  unsigned int v32; // stack - 0xa4
  unsigned int v33; // stack - 0xa0
  unsigned int v34; // stack - 0x9c
  unsigned int v35; // stack - 0x98
  unsigned int v36; // stack - 0x94
  unsigned long v37; // stack - 0x90
  unsigned long v38; // stack - 0x88
  unsigned int v39; // stack - 0x70
  int v4;
  unsigned int v40; // stack - 0x64
  unsigned int v41; // stack - 0x5c
  int v42; // stack - 0x58
  int v5;
  unsigned long v6; // rax
  long v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  v1 = *a0;
  v2 = a0[1];
  v3 = a0[2];
  v4 = a0[8];
  v19 = a0[4] % 0xc;
  v5 = v19 >> 0x1f;
  v9 = (long)(int)(a0[4] / 0xc + v5) + (long)a0[5];
  v10 = 0;
  if ((!(v9 & 3)) && (v10 = 1, (v9 * -0x70a3d70a3d70a3d7 + 0x51eb851eb851eb8 >> 2 | v9 * -0x70a3d70a3d70a3d7 << 0x3e) <= 0x28f5c28f5c28f5c))
    v10 = (unsigned long)(((unsigned int)((long)v9 / 100) & 3) == 1);
  v18 = (long)a0[3] + (long)(int)((unsigned int)*(unsigned short *)(((long)(int)((v5 & 0xc) + v19) + v10 * 0xd) * 2 + 92000) - 1);
  v10 = 0x3b;
  if (v1 <= 0x3b)
    v10 = (long)v1;
  if ((int)v10 <= -1)
    v10 = 0;
  v21 = (int)v10;
  v26._0_4_ = (int)*a2;
  v19 = -(int)v26;
  v6 = sub_12d40(v9,v18,v3,v2,v10 & 0xffffffff,0x46,0,0,0,v19);
  v13 = v6;
  v26 = v6;
  v27 = 6;
  v24 = 0;
  v20 = v6;
  while( true ) {
    v17 = v20;
    v7 = sub_12e40(a1,&v13,&v12);
    v23 = v12;
    if (!v7)
      return 0xffffffffffffffff;
    v7 = sub_12d40(v9,v18,v3,v2,v10 & 0xffffffff,v32,v34,v29,v28,v12);
    v20 = v13;
    if (!v7) break;
    if ((v17 != v13) && (v26 == v13)) {
      if ((int)v35 < 0) goto label_132a0;
      if (0 <= v4) { // branch-flip
        if ((v4 != 0) != (v35 != 0)) goto label_132a0;
      }
      else if (v24 <= (v35 != 0)) goto label_132a0;
    }
    v27 -= 1;
    if (!v27) goto label_13350;
    v8 = v7 + v13;
    v24 = v35 != 0;
    v26 = v17;
    v13 = v8;
  }
  v24 = v4 == 0;
  v25 = v35 == 0;
  if ((v25 != v24) && (0 <= (int)(v35 | v4))) {
    v23 = 0x92c70;
    do {
      v22 = 2;
      v11 = -v23;
      while( true ) {
        v15 = (long)v11 + v20;
        if (!SCARRY8((long)v11,v20)) {
          if (!sub_12e40(a1,&v15,&v14))
            return 0xffffffffffffffff;
          if ((v24 == (v42 == 0)) || (v42 <= -1)) {
            v7 = sub_12d40(v9,v18,v3,v2,v21,v40,v41,v39,v14 >> 0x20,v14 & 0xffffffff);
            v8 = v7 + v15;
            v16 = v8;
            if ((*a1)(&v16,&v12)) {
              v10 = (unsigned long)v21;
              v13 = v8;
              v23 = v12;
              goto label_132a0;
            }
            if (*__errno_location() != 0x4b)
              return 0xffffffffffffffff;
          }
        }
        v11 += v23 * 2;
        if (v22 == 1) break;
        v22 = 1;
      }
      v23 += 0x92c70;
    } while (v23 != 0xdb04f20);
    v10 = (unsigned long)v21;
    v20 += (long)(int)(((unsigned int)v24 - (unsigned int)v25) * 0xe10);
    v13 = v20;
    v14 = v20;
    if (!(*a1)(&v14,&v12)) {
label_13350:
      *__errno_location() = 0x4b;
      return 0xffffffffffffffff;
    }
    v13 = v20;
    v23 = v12;
  }
label_132a0:
  *a2 = v13 - ((long)v19 + v6);
  v9 = v13;
  if (v1 != v23) {
    v18 = (long)v1 + ((v1 <= 0 && v23 == 0x3c) - v10);
    v24 = SCARRY8(v18,v13);
    v9 = v18 + v13;
    v13 = v9;
    if (v24) goto label_13350;
    v14 = v9;
    if (!(*a1)(&v14,&v12))
      return 0xffffffffffffffff;
  }
  *(unsigned long *)a0 = CONCAT44(v28,v12);
  *(unsigned long *)&a0[2] = CONCAT44(v30,v29);
  *(unsigned long *)&a0[0xc] = v38;
  *(unsigned long *)&a0[4] = CONCAT44(v32,v31);
  *(unsigned long *)&a0[6] = CONCAT44(v34,v33);
  *(unsigned long *)&a0[8] = CONCAT44(v36,v35);
  *(unsigned long *)&a0[10] = v37;
  return v9;
}

// Function: sub_135f0 @ 0x135f0
unsigned long sub_135f0(int *a0)
{
  unsigned long v1; // rax
  
  tzset();
  v1 = sub_13040(a0,dat_1afd0,(long *)0x1b300); // tail-call
  return v1;
}

// Function: sub_13620 @ 0x13620
unsigned long sub_13620(FILE *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *(unsigned int *)a0;
  v3 = sub_71a0(a0);
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

// Function: sub_13690 @ 0x13690
void sub_13690(void *a0)
{
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned int *)((long)a0 + 0x18) = 0x95f616;
}

// Function: sub_136b0 @ 0x136b0
unsigned long sub_136b0(struct_21 *a0,struct_20 *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long v4;
  
  if (a0->field_0x18 != 0x95f616)
    __assert_fail("state->magic == 9827862","lib/cycle-check.c",0x3c,"cycle_check"); // no-return
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

// Function: sub_13740 @ 0x13740
int sub_13740(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_1b308) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_1b308 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_1b308 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_1b308 != -1)
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

// Function: sub_139b0 @ 0x139b0
void sub_139b0(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_11510(open(a0,a1,v1));
}

// Function: sub_13a30 @ 0x13a30
bool sub_13a30(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_13b00(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_13ac0 @ 0x13ac0
char * sub_13ac0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_13b00 @ 0x13b00
unsigned long sub_13b00(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_13bb0 @ 0x13bb0
void sub_13bb0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_13bc0 @ 0x13bc0
void sub_13bc0(unsigned long a0)
{
  sub_13740(a0,0,3); // tail-call
}

// Function: sub_13be0 @ 0x13be0
void sub_13be0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_1b008); // tail-call
}

// Function: _DT_FINI @ 0x13bf4
void _DT_FINI(void)
{
  return;
}
