// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_11fe0)
    return;
  (*dat_11fe0)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_11d10)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x35a0
void __cxa_finalize(void)
{
  (*dat_11ff0)(); // jump-as-call
}

// Function: getenv @ 0x35b0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11d18)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x35c0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_11d20)(); // jump-as-call
  return v1;
}

// Function: free @ 0x35d0
void free(void *a0)
{
  (*dat_11d28)(); // jump-as-call
}

// Function: strtoimax @ 0x35e0
void strtoimax(void)
{
  (*dat_11d30)(); // jump-as-call
}

// Function: abort @ 0x35f0
void abort(void)
{
  (*dat_11d38)(); // jump-as-call
}

// Function: __errno_location @ 0x3600
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_11d40)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x3610
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_11d48)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x3620
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_11d50)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3630
void _exit(int a0)
{
  (*dat_11d58)(); // jump-as-call
}

// Function: iconv @ 0x3640
void iconv(void)
{
  (*dat_11d60)(); // jump-as-call
}

// Function: puts @ 0x3650
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_11d68)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3660
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_11d70)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3670
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_11d78)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3680
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11d80)(); // jump-as-call
  return v1;
}

// Function: getchar_unlocked @ 0x3690
void getchar_unlocked(void)
{
  (*dat_11d88)(); // jump-as-call
}

// Function: fcntl @ 0x36a0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_11d90)(); // jump-as-call
  return v1;
}

// Function: write @ 0x36b0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_11d98)(); // jump-as-call
  return v1;
}

// Function: fread_unlocked @ 0x36c0
unsigned long fread_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_11da0)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x36d0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11da8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x36e0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_11db0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x36f0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11db8)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x3700
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_11dc0)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x3710
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_11dc8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3720
void __stack_chk_fail(void)
{
  (*dat_11dd0)(); // jump-as-call
}

// Function: mmap @ 0x3730
void mmap(void)
{
  (*dat_11dd8)(); // jump-as-call
}

// Function: getopt_long @ 0x3740
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_11de0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3750
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_11de8)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x3760
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11df0)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x3770
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_11df8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3780
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11e00)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x3790
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_11e08)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x37a0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11e10)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x37b0
void lseek(void)
{
  (*dat_11e18)(); // jump-as-call
}

// Function: __assert_fail @ 0x37c0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_11e20)(); // jump-as-call
}

// Function: fputs @ 0x37d0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_11e28)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x37e0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11e30)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x37f0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_11e38)(); // jump-as-call
  return v1;
}

// Function: close @ 0x3800
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_11e40)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x3810
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_11e48)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x3820
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11e50)(); // jump-as-call
  return v1;
}

// Function: read @ 0x3830
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_11e58)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3840
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_11e60)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3850
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_11e68)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x3860
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11e70)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3870
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_11e78)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3880
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11e80)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3890
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_11e88)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x38a0
void signal(void)
{
  (*dat_11e90)(); // jump-as-call
}

// Function: fprintf @ 0x38b0
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_11e98)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x38c0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_11ea0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x38d0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_11ea8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x38e0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11eb0)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x38f0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_11eb8)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x3900
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_11ec0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x3910
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_11ec8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3920
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_11ed0)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x3930
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_11ed8)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3940
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11ee0)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3950
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11ee8)(); // jump-as-call
  return v1;
}

// Function: mkstemp @ 0x3960
int mkstemp(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_11ef0)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x3970
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_11ef8)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3980
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11f00)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x3990
FILE * fdopen(int a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_11f08)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x39a0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_11f10)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x39b0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11f18)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x39c0
void iconv_close(void)
{
  (*dat_11f20)(); // jump-as-call
}

// Function: iswspace @ 0x39d0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_11f28)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x39e0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11f30)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x39f0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11f38)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3a00
void error(int a0,int a1,char *a2,...)
{
  (*dat_11f40)(); // jump-as-call
}

// Function: waitpid @ 0x3a10
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_11f48)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3a20
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_11f50)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3a30
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_11f58)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x3a40
void perror(char *a0)
{
  (*dat_11f60)(); // jump-as-call
}

// Function: execvp @ 0x3a50
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_11f68)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x3a60
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_11f70)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x3a70
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_11f78)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3a80
void exit(int a0)
{
  (*dat_11f80)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x3a90
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_11f88)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x3aa0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_11f90)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3ab0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_11f98)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x3ac0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_11fa0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3ad0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_11fa8)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x3ae0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_11fb0)(); // jump-as-call
  return v1;
}

// Function: feof_unlocked @ 0x3af0
int feof_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_11fb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3b00
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_11fc0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x3b10
void iconv_open(void)
{
  (*dat_11fc8)(); // jump-as-call
}

// Function: sub_3b20 @ 0x3b20
void sub_3b20(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_11fd0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_3b50 @ 0x3b50
void sub_3b50(void)
{
  return;
}

// Function: _FINI_0 @ 0x3bc0
void _FINI_0(void)
{
  if (!dat_12088) {
    if (dat_11ff0)
      __cxa_finalize(dat_12008);
    sub_3b50();
    dat_12088 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x3c00
void _INIT_0(void)
{
  return;
}

// Function: sub_3c09 @ 0x3c09
void sub_3c09(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,gettext(a0),a1);
  v1 = dat_225e8;
  error(2,0,gettext("Try \'%s --help\' for more information."),v1);
}

// Function: sub_3c7f @ 0x3c7f
void sub_3c7f(void)
{
  if (ferror_unlocked(stdout))
    sub_3e69("write failed"); // no-return
  if (fclose(stdout))
    sub_3eab(gettext("standard output")); // no-return
}

// Function: sub_3cd6 @ 0x3cd6
void sub_3cd6(void)
{
  FILE *v1;
  char *v2;
  long *v3; // stack - 0x20
  
  v2 = dat_225e8;
  printf(gettext("Usage: %s [OPTION]... FILE1 FILE2\n"),v2);
  printf("%s\n\n",gettext("Side-by-side merge of differences between FILE1 and FILE2."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  for (v3 = (long *)0x11a40; *v3; v3 = &v3[1]) {
    if (*(char *)*v3) { // branch-flip
      v2 = (char *)*v3;
      printf("  %s\n",gettext(v2));
    }
    else {
      putchar_unlocked(10);
    }
  }
  v2 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  printf("\n%s\n%s\n",gettext("If a FILE is \'-\', read standard input."),v2);
  sub_8c7b();
}

// Function: sub_3df5 @ 0x3df5
void sub_3df5(void) // return-dupe
{
  if (1 <= dat_120b8)
    kill(dat_120b8,0xd);
  if (!dat_120a8)
    return;
  unlink(dat_120a8);
}

// Function: sub_3e3e @ 0x3e3e
void sub_3e3e(void)
{
  sub_3df5(0);
  sub_510d(0);
  sub_51e3();
  exit(2); // no-return
}

// Function: sub_3e69 @ 0x3e69
void sub_3e69(char *a0)
{
  error(0,0,"%s",gettext(a0));
  sub_3e3e(); // no-return
}

// Function: sub_3eab @ 0x3eab
void sub_3eab(char *a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_51e3();
  error(0,v1,"%s",a0);
  sub_3e3e(); // no-return
}

// Function: sub_3ef1 @ 0x3ef1
void sub_3ef1(int a0,unsigned int a1,int a2,unsigned long a3) // ternary
{
  unsigned int v1; // eax
  char *v2; // rax
  
  if ((a0) || (a1 & 0x7f))
    v1 = 0x7fffffff;
  else {
    v1 = (int)a1 >> 8 & 0xff;
  }
  if (a2 < (int)v1) {
    if (v1 != 0x7e) { // branch-flip
      if (v1 != 0x7f) { // branch-flip
        v2 = (v1 != 0x7fffffff) ? "subsidiary program \'%s\' failed (exit status %d)" : "subsidiary program \'%s\' failed"; // branch-flip
      }
      else {
        v2 = "subsidiary program \'%s\' not found";
      }
    }
    else {
      v2 = "subsidiary program \'%s\' could not be invoked";
    }
    error(0,a0,gettext(v2),a3,(unsigned long)v1);
    sub_3e3e(); // no-return
  }
}

// Function: sub_3fa3 @ 0x3fa3
FILE * sub_3fa3(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = fopen(a0,a1);
  if (!v1)
    sub_3eab(a0); // no-return
  return v1;
}

// Function: sub_3fe7 @ 0x3fe7
void sub_3fe7(FILE *a0)
{
  if (fclose(a0))
    sub_3eab("fclose"); // no-return
}

// Function: sub_4019 @ 0x4019
unsigned long sub_4019(void *a0,unsigned long a1,FILE *a2)
{
  unsigned long v1; // rax
  
  v1 = fread_unlocked(a0,1,a1,a2);
  if (!v1) {
    if (ferror_unlocked(a2))
      sub_3eab(gettext("read failed")); // no-return
  }
  return v1;
}

// Function: sub_4082 @ 0x4082
void sub_4082(void *a0,unsigned long a1,FILE *a2)
{
  if (a1 != fwrite_unlocked(a0,1,a1,a2))
    sub_3eab(gettext("write failed")); // no-return
}

// Function: sub_40d3 @ 0x40d3
void sub_40d3(FILE *a0)
{
  if (fflush_unlocked(a0))
    sub_3eab(gettext("write failed")); // no-return
}

// Function: sub_410d @ 0x410d
char * sub_410d(char *a0,char a1,char *a2) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_410d
{
  unsigned char v1; // al
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5;
  
  if (!strcmp(a0,"-"))
    sub_3e69("cannot interactively merge standard input"); // no-return
  if (a1 != '\x01') // branch-flip
    v5 = a0;
  else {
    v2 = (char *)sub_5f32(a2);
    v3 = strlen(a0);
    v4 = sub_5f9f(v2);
    v1 = ((*(char *)sub_5f32(a0)) && (a0[v3 - 1] != '/'));
    v5 = (char *)sub_8dc0(v4 + v1 + v3 + 1);
    memcpy(v5,a0,v3);
    v5[v3] = '/';
    memcpy(&v5[v1 + v3],v2,v4);
    v5[v1 + v3 + v4] = '\0';
  }
  return v5;
}

// Function: sub_425e @ 0x425e
void sub_425e(unsigned long *a0,unsigned long a1)
{
  *a0 = a1;
  a0[3] = sub_8dc0(0x10001);
  a0[2] = a0[3];
  a0[1] = a0[2];
  *(char *)a0[3] = 10;
}

// Function: sub_42c0 @ 0x42c0
long sub_42c0(unsigned long *a0)
{
  long v1; // rax
  
  v1 = sub_4019((void *)a0[2],0x10000,(FILE *)*a0);
  a0[1] = a0[2];
  a0[3] = a0[2] + v1;
  *(char *)a0[3] = 10;
  sub_51e3();
  return v1;
}

// Function: sub_432d @ 0x432d
void sub_432d(unsigned long *a0,long a1,FILE *a2)
{
  void *v1;
  long v2; // stack - 0x28
  long v3; // stack - 0x10
  
  v3 = a0[1];
  v2 = a1;
  while( true ) {
    while( true ) {
      if (!v2) {
        sub_4082(v3,a0[1] - v3,a2);
        return;
      }
      v1 = (void *)a0[1];
      a0[1] = rawmemchr(v1,10);
      if (a0[1] == a0[3]) break;
      v2 -= 1;
      a0[1] = a0[1] + 1;
    }
    sub_4082(v3,a0[3] - v3,a2);
    if (!sub_42c0(a0)) break;
    v3 = a0[1];
  }
}

// Function: sub_4414 @ 0x4414
void sub_4414(unsigned long *a0,long a1) // return-dupe
{
  void *v1;
  long v2; // stack - 0x18
  
  v2 = a1;
  do {
    while( true ) {
      if (!v2)
        return;
      v1 = (void *)a0[1];
      a0[1] = rawmemchr(v1,10);
      if (a0[1] == a0[3]) break;
      v2 -= 1;
      a0[1] = a0[1] + 1;
    }
  } while (sub_42c0(a0));
}

// Function: sub_4493 @ 0x4493
unsigned long sub_4493(unsigned long *a0,char *a1,unsigned long a2)
{
  void *v1;
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x40
  char *v5; // stack - 0x38
  
  v4 = a2;
  v5 = a1;
  while( true ) {
    v1 = (void *)a0[1];
    v2 = rawmemchr(v1,10);
    v3 = (long)v2 - (long)v1;
    if (v4 <= v3)
      return 0;
    memcpy(v5,v1,v3);
    if (v2 < (void *)a0[3]) break;
    if (!sub_42c0(a0)) {
      if (v3)
        return 0;
      return 0xffffffff;
    }
    v5 = &v5[v3];
    v4 -= v3;
  }
  v5[v3] = '\0';
  a0[1] = (long)v2 + 1;
  return 1;
}

// Function: main @ 0x4573
int main(int argc,char **argv,char **envp) // return-dupe, ternary
{
  unsigned char v1; // al
  int v10; // stack - 0xbc
  int v11; // stack - 0xb8
  unsigned int v12; // stack - 0xb4
  char *v13; // stack - 0xb0
  char *v14; // stack - 0xa8
  FILE *v15; // stack - 0xa0
  char *v16; // stack - 0x98
  FILE *v17; // stack - 0x90
  FILE *v18; // stack - 0x88
  FILE *v19; // stack - 0x80
  unsigned char v2; // al
  int v20; // stack - 0x14
  char v3; // al
  int v4; // eax
  char *v5; // rax
  int v6; // stack - 0x18
  char v7 [32];
  char v8 [32];
  char v9 [32];
  
  dat_12020 = 2;
  sub_632f(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_626b(sub_3df5);
  sub_93c4();
  v13 = getenv("EDITOR");
  if (v13)
    dat_12010 = v13;
  sub_4ebd("diff");
  while( true ) {
    v11 = getopt_long(argc,argv,"abBdEHiI:lo:stvw:WZ",(option *)0x117a0,NULL);
    if (v11 == -1) break;
    switch(v11) {
      case 0x42:
        sub_4ebd("-B");
        break;
      default:
        sub_3c09(NULL,0); // no-return
      case 0x45:
        sub_4ebd("-E");
        break;
      case 0x48:
        sub_4ebd("-H");
        break;
      case 0x49:
        sub_4ebd("-I");
        sub_4ebd(optarg);
        break;
      case 0x57:
        sub_4ebd("-w");
        break;
      case 0x5a:
        sub_4ebd("-Z");
        break;
      case 0x61:
        sub_4ebd("-a");
        break;
      case 0x62:
        sub_4ebd("-b");
        break;
      case 100:
        sub_4ebd("-d");
        break;
      case 0x69:
        sub_4ebd("-i");
        break;
      case 0x6c:
        sub_4ebd("--left-column");
        break;
      case 0x6f:
        dat_124e8 = optarg;
        break;
      case 0x73:
        dat_124f0 = '\x01';
        break;
      case 0x74:
        sub_4ebd("-t");
        break;
      case 0x76:
        sub_8b97(stdout,"sdiff","GNU diffutils",dat_12018,sub_68f0("Thomas Lord"),0);
        sub_3c7f();
        return 0;
      case 0x77:
        sub_4ebd("-W");
        sub_4ebd(optarg);
        break;
      case 0x80:
        *dat_120a0 = optarg;
        break;
      case 0x81:
        sub_3cd6();
        sub_3c7f();
        return 0;
      case 0x82:
        sub_4ebd("--strip-trailing-cr");
        break;
      case 0x83:
        sub_4ebd("--tabsize");
        sub_4ebd(optarg);
      
    }
  }
  if (argc - optind != 2) {
    if (2 <= argc - optind)
      sub_3c09("extra operand \'%s\'",argv[(long)optind + 2]); // no-return
    sub_3c09("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  }
  if (!dat_124e8) {
    if (dat_124f0)
      sub_4ebd("--suppress-common-lines");
    sub_4ebd("-y");
    sub_4ebd("--");
    sub_4ebd(argv[optind]);
    sub_4ebd(argv[(long)optind + 1]);
    sub_4ebd(0);
    execvp(*dat_120a0,dat_120a0);
    sub_3eab(*dat_120a0); // no-return
  }
  v1 = sub_5e22(argv[optind]);
  v2 = sub_5e22(argv[(long)optind + 1]);
  if (v2 & v1)
    sub_3e69("both files to be compared are directories"); // no-return
  v5 = (char *)sub_410d(argv[optind],v1,argv[(long)optind + 1]);
  v14 = v5;
  v15 = (FILE *)sub_3fa3(v5,"r");
  v5 = (char *)sub_410d(argv[(long)optind + 1],v2,argv[optind]);
  v16 = v5;
  v17 = (FILE *)sub_3fa3(v5,"r");
  v18 = (FILE *)sub_3fa3(dat_124e8,"w");
  sub_4ebd("--sdiff-merge-assist");
  sub_4ebd("--");
  sub_4ebd(argv[optind]);
  sub_4ebd(argv[(long)optind + 1]);
  sub_4ebd(0);
  sub_4fdd();
  if (pipe(&v6))
    sub_3eab("pipe"); // no-return
  dat_120b8 = fork();
  if (dat_120b8 <= -1)
    sub_3eab("fork"); // no-return
  if (!dat_120b8) {
    if (dat_12450 != 1)
      sub_4fa3(2,1);
    sub_4fa3(0xd,0);
    close(v6);
    if (v20 != 1) {
      dup2(v20,1);
      close(v20);
    }
    execvp(*dat_120a0,dat_120a0);
    v4 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
    _exit(v4); // no-return
  }
  close(v20);
  v19 = fdopen(v6,"r");
  if (v19) {
    sub_425e(v7,v19);
    sub_425e(v8,v15);
    sub_425e(v9,v17);
    v3 = sub_5b03(v7,v8,v14,v9,v16,v18);
    sub_3fe7(v15);
    sub_3fe7(v17);
    sub_3fe7(v18);
    v12 = 0;
    sub_3fe7(v19);
    while (waitpid(dat_120b8,&v10,0) <= -1) {
      if (*__errno_location() != 4)
        sub_3eab("waitpid"); // no-return
      sub_51e3();
    }
    dat_120b8 = 0;
    if (dat_120a8) {
      unlink(dat_120a8);
      dat_120a8 = NULL;
    }
    if (v3 == '\x01') {
      sub_3ef1(v12,v10,1,*dat_120a0);
      sub_510d(0);
      sub_51e3();
      exit(v10 >> 8 & 0xff); // no-return
    }
    sub_3e3e(); // no-return
  }
  sub_3eab("fdopen"); // no-return
}

// Function: sub_4ebd @ 0x4ebd
void sub_4ebd(unsigned long a0)
{
  unsigned long v1;
  
  if (dat_12598 == dat_125a0) {
    if (dat_125a0) { // branch-flip
      if (0x7ffffffffffffff <= dat_125a0)
        sub_9384(); // no-return
      dat_125a0 *= 2;
    }
    else {
      dat_125a0 = 0x10;
    }
    dat_120a0 = sub_8e0c(dat_120a0,dat_125a0 * 8);
  }
  v1 = dat_12598;
  dat_12598 += 1;
  *(unsigned long *)(dat_120a0 + v1 * 8) = a0;
}

// Function: sub_4f78 @ 0x4f78
void sub_4f78(int a0) // return-dupe
{
  if ((a0 == 2) && (dat_124f1 == '\x01'))
    return;
  dat_124f4 = a0;
}

// Function: sub_4fa3 @ 0x4fa3
void sub_4fa3(int a0,unsigned long a1)
{
  dat_12500 = a1;
  sigaction(a0,(sigaction *)0x12500,NULL);
}

// Function: sub_4fdd @ 0x4fdd
void sub_4fdd(void)
{
  int v1; // stack - 0xc
  
  dat_12588 = 0x10000000;
  sigemptyset((sigset_t *)0x12508);
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    sigaddset((sigset_t *)0x12508,*(int *)((long)v1 * 4 + 0xd020));
  }
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    sigaction(*(int *)((long)v1 * 4 + 0xd020),NULL,(sigaction *)((long)v1 * 0x98 + 0x120c0));
    if (*(long *)((long)v1 * 0x98 + 0x120c0) != 1)
      sub_4fa3(*(unsigned int *)((long)v1 * 4 + 0xd020),sub_4f78);
  }
  signal(0x11,0);
  dat_124f8 = 1;
}

// Function: sub_510d @ 0x510d
void sub_510d(int a0) // return-dupe
{
  int v1; // stack - 0xc
  
  if (!dat_124f8)
    return;
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    if (((!a0) || (a0 == *(int *)((long)v1 * 4 + 0xd020))) && (*(long *)((long)v1 * 0x98 + 0x120c0) != 1))
      sigaction(*(int *)((long)v1 * 4 + 0xd020),(sigaction *)((long)v1 * 0x98 + 0x120c0),NULL);
  }
}

// Function: sub_51e3 @ 0x51e3
void sub_51e3(void)
{
  int v1;
  
  v1 = dat_124f4;
  if (dat_124f4) {
    sub_3df5(0);
    sub_510d(v1);
    raise(v1);
    exit(2); // no-return
  }
}

// Function: sub_5229 @ 0x5229
void sub_5229(void)
{
  fprintf(stderr,"%s",gettext("ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"));
}

// Function: sub_5264 @ 0x5264
int sub_5264(void)
{
  int v1; // eax
  
  while( true ) {
    v1 = getchar_unlocked();
    if ((!(*(unsigned short *)((long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) || (v1 == 10)) break;
    sub_51e3();
  }
  if (ferror_unlocked(stdin))
    sub_3eab(gettext("read failed")); // no-return
  return v1;
}

// Function: sub_52d6 @ 0x52d6
void sub_52d6(void)
{
  int v1; // eax
  
  do {
    v1 = getchar_unlocked();
    if (v1 == 10) break;
  } while (v1 != -1);
  if (ferror_unlocked(stdin))
    sub_3eab(gettext("read failed")); // no-return
}

// Function: sub_5326 @ 0x5326
unsigned long sub_5326(unsigned long *a0,unsigned long a1,long a2,long a3,unsigned long *a4,unsigned long a5,long a6,long a7,FILE *a8) // ternary
{
  bool v1;
  unsigned int v10; // stack - 0x10068
  int v11; // stack - 0x10064
  long v12; // stack - 0x10060
  long v13; // stack - 0x10058
  long v14; // stack - 0x10050
  long v15; // stack - 0x10048
  unsigned long v16; // stack - 0x10040
  char *v17; // stack - 0x10030
  unsigned long v18; // stack - 0x10028
  FILE *v2;
  int v3; // eax
  char *v4; // stack - 0x10038
  int v5; // stack - 0x10078
  char v6 [65552];
  int v7; // stack - 0x10074
  int v8; // stack - 0x10070
  int v9; // stack - 0x1006c
  
  v2 = a8;
label_5392:
  do {
    v1 = 0;
label_555d:
    if (!v1) {
      if (putchar_unlocked(0x25) != 0x25)
        sub_3eab(gettext("write failed")); // no-return
      sub_40d3(stdout);
      v7 = sub_5264();
      if (v7 == 0x76) {
label_546f:
        if (sub_5264() != 10) { // branch-flip
          sub_5229();
          sub_52d6();
        }
        else {
          v1 = 1;
        }
        goto label_555d;
      }
      if ((0x76 < v7) || (0x73 < v7)) goto label_5552;
      if ((0x71 <= v7) || (v7 == 0x6c)) goto label_546f;
      if (0x6c < v7) goto label_5552;
      if (v7 != 0x65) {
        if ((0x65 < v7) || (0x32 < v7)) goto label_5552;
        if (0x31 <= v7) goto label_546f;
        if (v7 != -1) { // branch-flip
          if (v7 != 10) goto label_5552;
        }
        else {
          if (feof_unlocked(stdin)) {
            v1 = 1;
            v7 = 0x71;
            goto label_555d;
          }
label_5552:
          sub_52d6();
        }
        sub_5229();
        goto label_555d;
      }
      v8 = sub_5264();
      if (0x73 <= v8) { // branch-flip
label_551e:
        sub_5229();
        sub_52d6();
      }
      else if (0x62 <= v8) { // branch-flip
        if (!(0x10405UL >> ((char)v8 + 0x9eU & 0x3f) & 1)) goto label_551e;
label_54f6:
        if (sub_5264() == 10) goto label_552a;
        sub_5229();
        sub_52d6();
      }
      else {
        if (v8 != 10) {
          if ((10 <= v8) && ((unsigned int)(v8 - 0x31U) <= 1)) goto label_54f6;
          goto label_551e;
        }
label_552a:
        v1 = 1;
      }
      goto label_555d;
    }
    if (0x76 < v7) {
label_5ae2:
      sub_5229();
      goto label_5392;
    }
    if (v7 <= 100) {
      if (v7 == 0x31) {
        sub_432d(a0,a3,v2); // return-dupe
        sub_4414(a4,a7);
        return 1;
      }
      if (v7 == 0x32) {
        sub_432d(a4,a7,v2); // return-dupe
        sub_4414(a0,a3);
        return 1;
      }
      goto label_5ae2;
    }
    switch(v7) {
      case 0x65:
        goto label_5673;
      default:
        goto label_5ae2;
      case 0x6c:
        sub_432d(a0,a3,v2);
        sub_4414(a4,a7);
        return 1;
      case 0x71:
        return 0;
      case 0x72:
        sub_432d(a4,a7,v2);
        sub_4414(a0,a3);
        return 1;
      case 0x73:
        dat_124f0 = 1;
        goto label_5392;
      case 0x76:
        dat_124f0 = 0;
      
    }
  } while( true );
label_5673:
  if (dat_120a8) // branch-flip
    dat_120b0 = fopen(dat_120a8,"w");
  else {
    v9 = sub_5e9b();
    if (v9 <= -1)
      sub_3eab("mkstemp"); // no-return
    dat_120b0 = fdopen(v9,"w");
  }
  if (!dat_120b0)
    sub_3eab(dat_120a8); // no-return
  if (v8 != 0x6c) { // branch-flip
    if (v8 <= 0x6c) {
      if (v8 != 100) { // branch-flip
        if ((0x65 <= v8) || ((v8 != 0x31 && (v8 != 0x62)))) goto label_580e;
      }
      else if (a3) {
        v13 = a3 + a2 + -1;
        v12 = a2;
        if (a3 != 1) // branch-flip
          fprintf(dat_120b0,"--- %s %ld,%ld\n",a1,a2,v13);
        else {
          fprintf(dat_120b0,"--- %s %ld\n",a1,a2);
        }
      }
      goto label_57ec;
    }
label_580e:
    sub_4414(a0,a3);
  }
  else {
label_57ec:
    sub_432d(a0,a3,dat_120b0);
  }
  if (v8 != 0x72) {
    if (0x73 <= v8) {
label_5927:
      sub_4414(a4,a7);
      goto label_593e;
    }
    if (v8 != 100) { // branch-flip
      if ((0x65 <= v8) || ((v8 != 0x32 && (v8 != 0x62)))) goto label_5927;
    }
    else if (a7) {
      v14 = a6;
      v15 = a7 + a6 + -1;
      if (a7 != 1) // branch-flip
        fprintf(dat_120b0,"+++ %s %ld,%ld\n",a5,a6,v15);
      else {
        fprintf(dat_120b0,"+++ %s %ld\n",a5,a6);
      }
    }
  }
  sub_432d(a4,a7,dat_120b0);
label_593e:
  sub_3fe7(dat_120b0);
  v10 = 0;
  dat_124f1 = 1;
  sub_51e3();
  v4 = dat_12010;
  v17 = dat_120a8;
  v18 = 0;
  v11 = fork();
  if (!v11) {
    execvp(dat_12010,&v4);
    v3 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
    _exit(v3); // no-return
  }
  if (0 <= v11) {
    while( true ) {
      if (0 <= waitpid(v11,&v5,0)) {
        dat_124f1 = 0;
        sub_3ef1(v10,v5,0,dat_12010);
        dat_120b0 = (unsigned long)sub_3fa3(dat_120a8,"r");
        while( true ) {
          v16 = sub_4019(v6,0x10000,dat_120b0);
          if (!v16) break;
          sub_51e3();
          sub_4082(v6,v16,v2);
        }
        sub_3fe7(dat_120b0);
        return 1;
      }
      if (*__errno_location() != 4) break;
      sub_51e3();
    }
    sub_3eab("waitpid"); // no-return
  }
  sub_3eab("fork"); // no-return
}

// Function: sub_5b03 @ 0x5b03
unsigned long sub_5b03(unsigned long *a0,unsigned long *a1,unsigned long a2,unsigned long *a3,unsigned long a4,FILE *a5)
{
  char v1; // al
  long v10; // stack - 0x128
  long v11; // stack - 0x120
  unsigned long v2; // rax
  char v3; // stack - 0x118
  char v4 [263];
  char *v5; // stack - 0x150
  long v6; // stack - 0x148
  long v7; // stack - 0x140
  long v8; // stack - 0x138
  long v9; // stack - 0x130
  
  v6 = 1;
  v7 = 1;
  while( true ) {
    while( true ) {
      v2 = sub_4493(a0,&v3,0x100);
      if ((int)v2 <= 0)
        return CONCAT71((undefined7)((unsigned long)v2 >> 8),(int)v2 != 0);
      sub_51e3();
      if (v3 != ' ') break;
      puts(v4);
    }
    *__errno_location() = 0;
    v8 = strtoimax(v4,&v5,10);
    if (((v8 < 0) || (*__errno_location())) || (*v5 != ',')) break;
    v9 = v8;
    v8 = strtoimax(&v5[1],&v5,10);
    if (((v8 < 0) || (*__errno_location())) || (*v5))
      sub_3e69(&v3); // no-return
    v10 = v8;
    v11 = v8;
    if (v8 <= v9)
      v11 = v9;
    if (v3 != 'c') { // branch-flip
      if (v3 != 'i')
        sub_3e69(&v3); // no-return
      if (dat_124f0) // branch-flip
        sub_4414(a0,v11);
      else {
        sub_432d(a0,v11,stdout);
      }
      sub_432d(a1,v9,a5);
      sub_4414(a3,v10);
    }
    else {
      sub_432d(a0,v11,stdout);
      v1 = sub_5326(a1,a2,v6,v9,a3,a4,v7,v10,a5);
      if (v1 != '\x01')
        return 0;
    }
    v6 += v9;
    v7 += v10;
  }
  sub_3e69(&v3); // no-return
}

// Function: sub_5e22 @ 0x5e22
unsigned int sub_5e22(char *a0) // return-dupe
{
  stat v1; // stack - 0xa8
  
  if ((!stat(a0,&v1)) && ((v1._24_4_ & 0xf000) == 0x4000))
    return 1;
  return 0;
}

// Function: sub_5e9b @ 0x5e9b
int sub_5e9b(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v2 = getenv("TMPDIR");
  if (!v2)
    v2 = "/tmp";
  v3 = (char *)sub_8dc0(strlen(v2) + 0xd);
  sprintf(v3,"%s/sdiffXXXXXX",v2);
  v1 = mkstemp(v3);
  if (0 <= v1)
    dat_120a8 = v3;
  return v1;
}

// Function: sub_5f32 @ 0x5f32
char * sub_5f32(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_5f89;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_5f89:
  }
  return v2;
}

// Function: sub_5f9f @ 0x5f9f
unsigned long sub_5f9f(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}

// Function: sub_5ff2 @ 0x5ff2
void sub_5ff2(int a0)
{
  char *v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char v5 [4104]; // stack - 0x1018
  long v6; // fs_offset
  char *v7; // stack - 0x1060
  unsigned long v8; // stack - 0x10
  
  v8 = *(unsigned long *)(v6 + 0x28);
  (*dat_225c0)(a0);
  if (a0) // branch-flip
    v1 = dat_225c8;
  else {
    v1 = dat_225d0;
  }
  v2 = strlen(dat_225d8);
  v3 = strlen(v1);
  if (0x1000 <= v3 + v2) { // branch-flip
    write(2,dat_225d8,v2);
    write(2,": ",2);
    write(2,v1,v3);
    v5[0] = 10;
    v7 = (char *)0x1;
  }
  else {
    v4 = mempcpy(v5,dat_225d8,v2);
    v7 = mempcpy(mempcpy(v4,": ",2),v1,v3);
    *v7 = 10;
    v7 = &v7[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v7);
  if (!a0)
    _exit(dat_12020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_61fb @ 0x61fb
void sub_61fb(void)
{
  return;
}

// Function: sub_6209 @ 0x6209
unsigned long sub_6209(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_5ff2(0xb); // no-return
}

// Function: sub_6235 @ 0x6235
void sub_6235(int a0)
{
  unsigned int v1; // eax
  
  if ((a0) && (!dat_225e0))
    v1 = 0xb;
  else {
    v1 = 0;
  }
  sub_5ff2(v1); // no-return
}

// Function: sub_626b @ 0x626b
void * sub_626b(void *a0) // return-dupe
{
  int v1; // eax
  void *v2; // rax
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = sub_61fb;
  }
  dat_225c0 = v2;
  v2 = (char *)gettext("program error");
  dat_225c8 = (char *)v2;
  v2 = (char *)gettext("stack overflow");
  dat_225d0 = (char *)v2;
  v2 = (unsigned long)sub_631e();
  dat_225d8 = v2;
  v1 = sub_7191(sub_6235,0x125c0,0x10000);
  if (v1) {
    v2 = (int *)__errno_location();
    *(int *)v2 = 0x5f;
    v2 = (void *)0xffffffff;
    return v2;
  }
  v1 = sub_70e1(sub_6209);
  dat_225e0 = v1;
  v2 = NULL;
  return v2;
}

// Function: sub_631e @ 0x631e
unsigned long sub_631e(void)
{
  return __progname;
}

// Function: sub_632f @ 0x632f
void sub_632f(char *a0)
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
  dat_225e8 = v2;
  program_invocation_name = v2;
}

// Function: sub_641a @ 0x641a
char sub_641a(char *a0,char *a1)
{
  bool v1;
  char *v10; // stack - 0xc0
  char v11; // stack - 0xb5
  char v12; // stack - 0x8c
  char *v13; // stack - 0x88
  long v14; // stack - 0x80
  unsigned int v15; // stack - 0x78
  unsigned int v16; // stack - 0x74
  long v17; // stack - 0x70
  unsigned long v18; // stack - 0x68
  unsigned long v19; // stack - 0x60
  bool v2;
  long v20; // stack - 0x50
  undefined3 v21; // stack - 0x4b
  char *v22; // stack - 0x48
  long v23; // stack - 0x40
  unsigned long v24; // stack - 0x38
  unsigned long v25; // stack - 0x30
  bool v3;
  char *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  char v7 [8];
  char v8 [4]; // stack - 0x98
  char *v9; // stack - 0x58
  
  v4 = (char *)sub_80c3(a1,2);
  v11 = 0;
  v10 = a0;
  while( true ) {
    while( true ) {
      if ((!*v10) || (v5 = (char *)sub_a427(v10,v4), !v5)) {
        free(v4); // return-dupe
        return v11;
      }
      if (2 <= __ctype_get_mb_cur_max()) break;
      v3 = 1;
      if ((v10 < v5) && (*(unsigned short *)((unsigned long)(unsigned char)v5[-1] * 2 + *(long *)__ctype_b_loc()) & 8))
        v3 = 0;
      v6 = strlen(v4);
      v1 = 1;
      if ((v5[v6]) && (*(unsigned short *)((unsigned long)(unsigned char)v5[v6] * 2 + *(long *)__ctype_b_loc()) & 8))
        v1 = 0;
      if ((v3) && (v1)) {
        v11 = 1;
        free(v4);
        return v11;
      }
      if (!*v5) {
        free(v4);
        return v11;
      }
      v10 = &v5[1];
    }
    v13 = v10;
    v8[0] = 0;
    memset(v7,0,8);
    v12 = 0;
    v3 = 1;
    if (v13 < v5) {
      do {
        v12 = 0;
        sub_ab95(v8);
        if (((char)v15 != '\x01') || (v16))
          v1 = 1;
        else {
          v1 = 0;
        }
        if (!v1)
          abort(); // no-return
        v9 = v13;
        v20 = v14;
        v22 = (char *)CONCAT44(v16,v15);
        v23 = v17;
        v24 = v18;
        v25 = v19;
        v13 = &v13[v14];
        v12 = 0;
      } while (v13 < v5);
      if (((char)v15) && (iswalnum(v16)))
        v3 = 0;
    }
    v13 = v5;
    v8[0] = 0;
    memset(v7,0,8);
    v12 = 0;
    v22 = v4;
    v9 = (char *)((unsigned long)v9 & 0xffffffffffffff00);
    memset((void *)((long)&v9 + 4),0,8);
    v20 = (undefined5)(unsigned int)v20;
    while( true ) {
      sub_ab95(&v9);
      if (((char)v24 != '\x01') || (v24._4_4_))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_ab95(v8);
      if (((char)v15 != '\x01') || (v16))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1)
        abort(); // no-return
      v13 = &v13[v14];
      v12 = 0;
      v22 = &v22[v23];
      v20 = (undefined5)(unsigned int)v20;
    }
    v1 = 1;
    sub_ab95(v8);
    if (((char)v15 != '\x01') || (v16))
      v2 = 1;
    else {
      v2 = 0;
    }
    if (v2) {
      v9 = v13;
      v20 = v14;
      v22 = (char *)CONCAT44(v16,v15);
      v23 = v17;
      v24 = v18;
      v25 = v19;
      if (((char)v15) && (iswalnum(v16)))
        v1 = 0;
    }
    if ((v3) && (v1)) break;
    v13 = v5;
    v8[0] = 0;
    memset(v7,0,8);
    v12 = 0;
    sub_ab95(v8);
    if (((char)v15 != '\x01') || (v16))
      v3 = 1;
    else {
      v3 = 0;
    }
    if (!v3) {
      free(v4);
      return v11;
    }
    v10 = &v5[v14];
  }
  v11 = 1;
  free(v4);
  return v11;
}

// Function: sub_68f0 @ 0x68f0
char * sub_68f0(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = gettext(a0);
  if (v1 != a0) {
    if (sub_641a(v1,a0)) // branch-flip
      a0 = v1;
    else {
      v2 = strlen(v1);
      v3 = (char *)sub_8dc0(strlen(a0) + v2 + 4);
      sprintf(v3,"%s (%s)",v1,a0);
      a0 = v3;
    }
  }
  return a0;
}

// Function: sub_6996 @ 0x6996
char * sub_6996(char *a0,char *a1)
{
  char *v1; // rax
  char *v2;
  unsigned long v3; // rax
  char *v4; // rax
  char *v5;
  char *v6; // stack - 0x70
  char *v7; // stack - 0x68
  char *v8; // stack - 0x60
  char *v9; // stack - 0x58
  
  v1 = gettext(a0);
  v2 = (char *)sub_9663();
  v6 = NULL;
  v7 = NULL;
  v9 = NULL;
  if (sub_9502(v2,"UTF-8")) { // branch-flip
    v6 = (char *)sub_94b1(a1,"UTF-8",v2);
    v8 = v6;
    v3 = strlen(v2);
    v4 = (char *)sub_8dc0(v3 + 0xb);
    memcpy(v4,v2,v3);
    memcpy(&v4[v3],"//TRANSLIT",0xb);
    v2 = (char *)sub_94b1(a1,"UTF-8",v4);
    free(v4);
    if (v2) {
      if (strchr(v2,0x3f)) // branch-flip
        free(v2);
      else {
        v7 = v2;
        v9 = v2;
      }
    }
  }
  else {
    v8 = a1;
    v9 = a1;
  }
  if (v8) // branch-flip
    v2 = v8;
  else if (v9) // branch-flip
    v2 = v9;
  else {
    v2 = a0;
  }
  if (!strcmp(v1,a0)) {
    if ((v6) && (v6 != v2))
      free(v6);
    if ((v7) && (v7 != v2))
      free(v7);
    return v2;
  }
  if (((sub_641a(v1,a0)) || ((v8 && (sub_641a(v1,v8))))) || ((v9 && (sub_641a(v1,v9))))) {
    if (v6)
      free(v6);
    if (v7)
      free(v7);
    v5 = v1;
  }
  else {
    v3 = strlen(v1);
    v5 = (char *)sub_8dc0(strlen(v2) + v3 + 4);
    sprintf(v5,"%s (%s)",v1,v2);
    if (v6)
      free(v6);
    if (v7)
      free(v7);
  }
  return v5;
}

// Function: sub_6c79 @ 0x6c79
void sub_6c79(void)
{
  return;
}

// Function: sub_6c84 @ 0x6c84
void sub_6c84(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_802f(a0,v1) <= -1)
    return;
  dat_225f0 = v2 + -1;
}

// Function: sub_6ce0 @ 0x6ce0
void sub_6ce0(unsigned long a0,void *a1,void *a2) // return-dupe x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_6ce0
{
  unsigned long v1;
  unsigned long v2;
  int v3;
  int v4; // eax
  unsigned int v5; // eax
  unsigned long v6; // stack - 0x38
  bool v7;
  long v8; // stack - 0x30
  void *v9; // stack - 0x28
  
  v1 = *(unsigned long *)((long)a1 + 0x10);
  if ((dat_22610) && (v3 = (*dat_22610)(v1,0), v3))
    return;
  if ((dat_225f8) && (v2 = *(unsigned long *)((long)a2 + 0xa0), dat_225f0)) {
    v3 = *__errno_location();
    v4 = sub_802f(dat_225f0,&v6);
    *__errno_location() = v3;
    if (0 <= v4) {
      if (v6 <= v1) // branch-flip
        v7 = v1 <= (unsigned long)(v8 - 1U);
      else {
        v7 = (*v9)(v1,&v6) != 0;
      }
      if (v7) {
        v5 = ((dat_22600 <= v2) && (v2 <= dat_22608 + dat_22600));
        (*dat_225f8)(v5,a2);
      }
    }
  }
  if ((dat_22610) && (v3 = (*dat_22610)(v1,1), v3))
    return;
  signal(0xb,0);
}

// Function: sub_6e83 @ 0x6e83
void sub_6e83(int a0)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  undefined4 v3; // stack - 0x20
  
  v1 = sub_6ce0;
  sigemptyset(&v2);
  sigaddset(&v2,1);
  sigaddset(&v2,2);
  sigaddset(&v2,3);
  sigaddset(&v2,0xd);
  sigaddset(&v2,0xe);
  sigaddset(&v2,0xf);
  sigaddset(&v2,10);
  sigaddset(&v2,0xc);
  sigaddset(&v2,0x11);
  sigaddset(&v2,0x11);
  sigaddset(&v2,0x17);
  sigaddset(&v2,0x1d);
  sigaddset(&v2,0x1d);
  sigaddset(&v2,0x18);
  sigaddset(&v2,0x19);
  sigaddset(&v2,0x1a);
  sigaddset(&v2,0x1b);
  sigaddset(&v2,0x1e);
  sigaddset(&v2,0x1c);
  v3 = 4;
  if (dat_225f8)
    v3 = 0x8000004;
  sigaction(a0,(sigaction *)&v1,NULL);
}

// Function: sub_70e1 @ 0x70e1
unsigned long sub_70e1(unsigned long a0)
{
  dat_22610 = a0;
  sub_6e83(0xb);
  return 0;
}

// Function: sub_7114 @ 0x7114
void sub_7114(void) // return-dupe
{
  dat_22610 = 0;
  if (dat_225f8) {
    dat_22610 = 0;
    return;
  }
  signal(0xb,0);
}

// Function: sub_7150 @ 0x7150
unsigned long sub_7150(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_6c79();
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_7191 @ 0x7191
unsigned long sub_7191(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x28
  unsigned int v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  
  if ((!dat_225f0) && (sub_6c84(&v1), !dat_225f0))
    return 0xffffffff;
  v2 = 0;
  dat_225f8 = a0;
  dat_22600 = a1;
  dat_22608 = a2;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_6e83(0xb);
  return 0;
}

// Function: sub_7263 @ 0x7263
void sub_7263(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_225f8 = 0;
  if (dat_22610) // branch-flip
    sub_6e83(0xb);
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_7300 @ 0x7300
bool sub_7300(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_7341 @ 0x7341
unsigned long sub_7341(struct_1 *a0,char *a1) // return-dupe x3, return-dupe
{
  void *v1;
  long v2;
  long v3;
  int v4; // eax
  int v5; // stack - 0x20
  char *v6; // stack - 0x18
  char *v7; // stack - 0x10
  
  v5 = open(a1,0);
  if (v5 <= -1)
    return 0xffffffff;
  a0->field_0x0 = 0;
  a0->field_0x10 = 0;
  v6 = NULL;
  a0->field_0x18 = &a0[1];
  v7 = (char *)0x1;
  a0->field_0x20 = 0;
  a0->field_0x30 = 0;
  a0->field_0x38 = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = a0->field_0x18;
      v4 = (int)read(v5,v1,(unsigned long)v7);
      if ((0 <= v4) || (*__errno_location() != 4)) {
        if (1 <= v4) {
          if ((char *)(long)(v4 + 0x1049) <= v7) {
            a0->field_0x8 = (long)v4;
            while( true ) {
              v2 = a0->field_0x8;
              v1 = a0->field_0x18;
              v3 = a0->field_0x8;
              v4 = (int)read(v5,(void *)((long)v1 + v3),(long)v7 - v2);
              if ((v4 <= -1) && (*__errno_location() == 4)) goto label_75f0;
              if (v4 < 0) goto label_7642;
              if ((unsigned long)((long)v7 - a0->field_0x8) < (unsigned long)(long)(v4 + 0x1049)) break;
              if (!v4) {
                close(v5);
                return 0;
              }
              a0->field_0x8 = a0->field_0x8 + (long)v4;
            }
          }
          goto label_74f0;
        }
label_7642:
        close(v5);
        goto label_764f;
      }
    }
    else {
label_74f0:
      if (v6) { // branch-flip
        v7 = (char *)((long)v7 << 1);
        if (!v7) goto label_7642;
        if (a0->field_0x20)
          munmap((void *)a0->field_0x20,a0->field_0x28);
      }
      else {
        v6 = (char *)(long)getpagesize();
        for (v7 = v6; v7 <= (char *)0x1049; v7 = (char *)((long)v7 << 1)) {
        }
      }
      a0->field_0x20 = mmap(0,v7,3,0x22,0xffffffff,0);
      if (a0->field_0x20 == -1) {
        close(v5);
        return 0xffffffff;
      }
      a0->field_0x28 = (unsigned long)v7;
      a0->field_0x30 = a0->field_0x20;
      a0->field_0x38 = &v7[a0->field_0x30];
      a0->field_0x18 = (void *)a0->field_0x20;
    }
label_75f0:
    if ((long)lseek(v5,0,0) <= -1) {
      close(v5);
      v5 = open(a1,0);
      if (v5 < 0) {
label_764f:
        if (!a0->field_0x20)
          return 0xffffffff;
        munmap((void *)a0->field_0x20,a0->field_0x28);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_767e @ 0x767e
unsigned long sub_767e(struct_2 *a0) // early-return
{
  if (a0->field_0x0 == a0->field_0x8) {
    a0->field_0x10 = 1;
    return 0xffffffff;
  }
  return (unsigned long)*(unsigned char *)(a0->field_0x0 + a0->field_0x18);
}

// Function: sub_76ca @ 0x76ca
int sub_76ca(struct_2 *a0)
{
  int v1; // eax
  
  v1 = sub_767e(a0);
  if (0 <= v1)
    a0->field_0x0 = a0->field_0x0 + 1;
  return v1;
}

// Function: sub_7706 @ 0x7706
unsigned long sub_7706(struct_2 *a0,long *a1) // return-dupe
{
  int v1; // eax
  int v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = 0;
  v2 = 0;
  do {
    v1 = sub_767e(a0);
    if ((0x30 <= v1) && (v1 <= 0x39))
      v3 = (long)(v1 + -0x30) + v3 * 0x10;
    else if ((0x41 <= v1) && (v1 <= 0x46))
      v3 = (long)(v1 + -0x37) + v3 * 0x10;
    else {
      if ((v1 <= 0x60) || (0x66 < v1)) {
        if (!v2)
          return 0xffffffff;
        *a1 = v3;
        return 0;
      }
      v3 = (long)(v1 + -0x57) + v3 * 0x10;
    }
    sub_76ca(a0);
    v2 += 1;
  } while( true );
}

// Function: sub_77e2 @ 0x77e2
void sub_77e2(void *a0) // return-dupe
{
  if (!*(long *)((long)a0 + 0x20))
    return;
  munmap(*(void **)((long)a0 + 0x20),*(unsigned long *)((long)a0 + 0x28));
}

// Function: sub_781d @ 0x781d
unsigned long sub_781d(struct_3 *a0) // return-dupe
{
  bool v1;
  int v2; // eax
  char v3 [48];
  unsigned long v4; // stack - 0x78
  long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x28
  unsigned long v9; // stack - 0x20
  
  if ((int)sub_7341(v3,"/proc/self/maps") < 0)
    return 0xffffffff;
  v6 = v8;
  v7 = v9;
  do {
    while( true ) {
      if ((((int)sub_7706(v3,&v4) < 0) || (sub_76ca(v3) != 0x2d)) || ((int)sub_7706(v3,&v5) < 0)) {
        sub_77e2(v3); // return-dupe
        return 0;
      }
      do {
        v2 = sub_76ca(v3);
        if ((v2 != -1) && (v2 != 10))
          v1 = 1;
        else {
          v1 = 0;
        }
      } while (v1);
      if ((v6 < v4) || ((unsigned long)(v5 - 1U) < v7 - 1)) break;
      if (((v4 < v6) && (v2 = sub_7fa2(a0,v4,v6), v2)) || ((v7 - 1 < (unsigned long)(v5 - 1U) && (v2 = sub_7fa2(a0,v7,v5), v2)))) {
        sub_77e2(v3);
        return 0;
      }
    }
    v2 = sub_7fa2(a0,v4,v5);
  } while (!v2);
  sub_77e2(v3);
  return 0;
}

// Function: sub_79be @ 0x79be
unsigned long sub_79be(struct_3 *a0) // return-dupe
{
  if (!sub_781d(a0))
    return 0;
  return 0xffffffff;
}

// Function: sub_79f1 @ 0x79f1
void sub_79f1(void)
{
  dat_22618 = (long)getpagesize();
}

// Function: sub_7a0a @ 0x7a0a
unsigned int sub_7a0a(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_22618,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_7a61 @ 0x7a61
unsigned long sub_7a61(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  long v3; // rcx
  char v4 [1032];
  unsigned long v5; // stack - 0x440
  unsigned long v6; // stack - 0x438
  
  v6 = 0x400;
  v5 = a0;
  while( true ) {
    if (!v5)
      return 0;
    if (v5 / dat_22618 < v6)
      v6 = v5 / dat_22618;
    v1 = dat_22618 * v6;
    v3 = dat_22618 * v6;
    if (mincore((void *)(v5 - v3),v1,v4) <= -1) break;
    v5 -= dat_22618 * v6;
  }
  while (v6 != 1) {
    v1 = v6 + 1 >> 1;
    v2 = dat_22618 * v1;
    v3 = dat_22618 * v1;
    if (0 <= mincore((void *)(v5 - v3),v2,v4)) { // branch-flip
      v5 -= dat_22618 * v1;
      v6 >>= 1;
    }
    else {
      v6 = v1;
    }
  }
  return v5;
}

// Function: sub_7c15 @ 0x7c15
void * sub_7c15(long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [1032];
  void *v4; // stack - 0x440
  unsigned long v5; // stack - 0x438
  
  v5 = 0x400;
  v4 = (void *)(a0 + dat_22618);
  while( true ) {
    if (!v4)
      return NULL;
    if ((unsigned long)-(long)v4 / dat_22618 < v5)
      v5 = (unsigned long)-(long)v4 / dat_22618;
    v1 = dat_22618 * v5;
    if (mincore(v4,v1,v3) <= -1) break;
    v4 = (void *)((long)v4 + dat_22618 * v5);
  }
  while (v5 != 1) {
    v1 = v5 + 1 >> 1;
    v2 = dat_22618 * v1;
    if (0 <= mincore(v4,v2,v3)) { // branch-flip
      v4 = (void *)((long)v4 + dat_22618 * v1);
      v5 >>= 1;
    }
    else {
      v5 = v1;
    }
  }
  return v4;
}

// Function: sub_7dad @ 0x7dad
unsigned long sub_7dad(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  void *v6; // stack - 0x20
  
  v1 = dat_22618 * (a0 / dat_22618);
  v2 = (dat_22618 * (a1 / dat_22618 + 1) - v1) / dat_22618;
  for (v4 = 1; v4 < v2; v4 = v4 << 1) {
  }
  do {
    v4 >>= 1;
    if (!v4)
      return 1;
    v3 = dat_22618 * v4;
    v6 = (void *)(v3 + v1);
    for (v5 = v4; v5 < v2; v5 = v5 + v4 * 2) {
      if (sub_7a0a(v6))
        return 0;
      v6 = (void *)((long)v6 + v3 * 2);
    }
  } while( true );
}

// Function: sub_7ebe @ 0x7ebe
unsigned long sub_7ebe(unsigned long a0,long *a1) // early-return
{
  long v1;
  unsigned long v2; // rax
  
  v2 = a0 * 2 - *a1;
  if (a0 < v2)
    return 0;
  v1 = *a1;
  return sub_7dad(v2,v1 - 1);
}

// Function: sub_7f1b @ 0x7f1b
unsigned long sub_7f1b(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // rax
  
  if (!dat_22618)
    sub_79f1();
  v1 = dat_22618 * (a0 / dat_22618);
  *a1 = sub_7a61(v1);
  a1[1] = sub_7c15(v1);
  a1[2] = sub_7ebe;
  return 0;
}

// Function: sub_7fa2 @ 0x7fa2
unsigned long sub_7fa2(struct_3 *a0,unsigned long a1,unsigned long a2) // return-dupe
{
  if ((a1 <= a0->field_0x0) && (a0->field_0x0 <= a2 - 1)) {
    *a0->field_0x8 = a1;
    a0->field_0x8[1] = a2;
    a0->field_0x8[3] = a0->field_0x10;
    a0->field_0x18 = 0;
    return 1;
  }
  a0->field_0x10 = a2;
  return 0;
}

// Function: sub_802f @ 0x802f
unsigned long sub_802f(unsigned long a0,unsigned long *a1) // early-return
{
  unsigned long v1; // stack - 0x38
  unsigned long *v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  int v4; // stack - 0x20
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_79be(&v1);
  if (!v4) {
    a1[2] = sub_7300;
    return 0;
  }
  return sub_7f1b(a0,a1);
}

// Function: sub_80c3 @ 0x80c3
unsigned char * sub_80c3(char *a0,int a1)
{
  unsigned long v1; // rax
  unsigned char *v10; // stack - 0x50
  long v11; // stack - 0x48
  char v12; // stack - 0x40
  unsigned int v13; // stack - 0x3c
  char v2 [8];
  unsigned char *v3; // stack - 0x68
  int v4; // stack - 0x84
  unsigned char *v5; // stack - 0x80
  unsigned char *v6; // stack - 0x78
  unsigned char *v7; // stack - 0x70
  char v8; // stack - 0x60
  char v9; // stack - 0x54
  
  v7 = (unsigned char *)strdup(a0);
  if (v7) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      if (a1) {
        v10 = v7;
        v3 = &v7[strlen((char *)v7)];
        v8 = 0;
        memset(v2,0,8);
        while (((v9 = 0, v10 < v3 && (sub_96ab(&v3), v12)) && (iswspace(v13)))) {
          v10 = &v10[v11];
        }
        v1 = strlen((char *)v10);
        memmove(v7,v10,v1 + 1);
      }
      if (a1 != 1) {
        v4 = 0;
        v10 = v7;
        v3 = &v7[strlen((char *)v7)];
        v8 = 0;
        memset(v2,0,8);
        for (; v9 = 0, v10 < v3; v10 = &v10[v11]) {
          sub_96ab(&v3);
          if (((v4) || (!v12)) || (!iswspace(v13))) {
            if ((v4) || ((v12 == '\x01' && (iswspace(v13))))) {
              if ((v4 != 1) || ((v12 == '\x01' && (iswspace(v13))))) {
                if ((v4 != 1) || ((!v12 || (!iswspace(v13))))) {
                  if (((v4 != 2) || (!v12)) || (!iswspace(v13)))
                    v4 = 1;
                }
                else {
                  v4 = 2;
                  v5 = v10;
                }
              }
            }
            else {
              v4 = 1;
            }
          }
        }
        if (v4 == 2)
          *v5 = 0;
      }
    }
    else {
      if (a1) {
        v6 = v7;
        while ((*v6 && (*(unsigned short *)((unsigned long)*v6 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
          v6 = &v6[1];
        }
        memmove(v7,v6,strlen((char *)v6) + 1);
      }
      if (a1 != 1) {
        v6 = &v7[strlen((char *)v7) - 1];
        while ((v7 <= v6 && (*(unsigned short *)((unsigned long)*v6 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
          *v6 = 0;
          v6 = &v6[-1];
        }
      }
    }
    return v7;
  }
  sub_9384(); // no-return
}

// Function: sub_8422 @ 0x8422
void sub_8422(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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
  fprintf(a0,"Copyright %s %d Free Software Foundation, Inc.",gettext("(C)"),0x7e5);
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

// Function: sub_8a1e @ 0x8a1e
void sub_8a1e(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_8422(a0,a1,a2,a3,a4,v1);
}

// Function: sub_8a91 @ 0x8a91
void sub_8a91(FILE *a0,char *a1,char *a2,char *a3,struct_8 *a4)
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
  sub_8422(a0,a1,a2,a3,v2,v3);
}

// Function: sub_8b97 @ 0x8b97
void sub_8b97(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_8a91(a8,a9,a10,a11,&v3);
}

// Function: sub_8c7b @ 0x8c7b
void sub_8c7b(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-diffutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_8d24 @ 0x8d24
void sub_8d24(unsigned long a0,unsigned long a1)
{
  sub_8e88(NULL,a0,a1);
}

// Function: sub_8d4f @ 0x8d4f
void sub_8d4f(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8e88(a0,a1,a2);
}

// Function: sub_8d80 @ 0x8d80
void sub_8d80(unsigned long a0)
{
  sub_8dc0(a0);
}

// Function: sub_8d9e @ 0x8d9e
long sub_8d9e(long a0)
{
  if (!a0)
    sub_9384(); // no-return
  return a0;
}

// Function: sub_8dc0 @ 0x8dc0
void sub_8dc0(unsigned long a0)
{
  sub_8d9e(malloc(a0));
}

// Function: sub_8de6 @ 0x8de6
void sub_8de6(unsigned long a0)
{
  sub_8d9e(sub_9596(a0));
}

// Function: sub_8e0c @ 0x8e0c
void * sub_8e0c(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_9384(); // no-return
  return v1;
}

// Function: sub_8e57 @ 0x8e57
void sub_8e57(void *a0,unsigned long a1)
{
  sub_8d9e(sub_95b4(a0,a1));
}

// Function: sub_8e88 @ 0x8e88
void * sub_8e88(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_9384(); // no-return
  return v1;
}

// Function: sub_8ee2 @ 0x8ee2
void sub_8ee2(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8d9e(sub_9614(a0,a1,a2));
}

// Function: sub_8f1b @ 0x8f1b
void sub_8f1b(void *a0,unsigned long *a1)
{
  sub_8f49(a0,a1,1);
}

// Function: sub_8f49 @ 0x8f49
unsigned long sub_8f49(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_9384(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_8e88(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_9028 @ 0x9028
unsigned long sub_9028(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_9384(); // no-return
  v3 = sub_8e0c(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_91e4 @ 0x91e4
void sub_91e4(unsigned long a0)
{
  sub_922a(a0,1);
}

// Function: sub_9207 @ 0x9207
void sub_9207(unsigned long a0)
{
  sub_925b(a0,1);
}

// Function: sub_922a @ 0x922a
void sub_922a(unsigned long a0,unsigned long a1)
{
  sub_8d9e(calloc(a0,a1));
}

// Function: sub_925b @ 0x925b
void sub_925b(unsigned long a0,unsigned long a1)
{
  sub_8d9e(sub_95eb(a0,a1));
}

// Function: sub_928c @ 0x928c
void sub_928c(char *a0,unsigned long a1)
{
  memcpy((void *)sub_8dc0(a1),a0,a1);
}

// Function: sub_92c4 @ 0x92c4
void sub_92c4(void *a0,unsigned long a1)
{
  memcpy((void *)sub_8de6(a1),a0,a1);
}

// Function: sub_9304 @ 0x9304
void sub_9304(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_8de6(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_9353 @ 0x9353
void sub_9353(char *a0)
{
  sub_928c(a0,strlen(a0) + 1);
}

// Function: sub_9384 @ 0x9384
void sub_9384(void)
{
  error(dat_12020,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_93c4 @ 0x93c4
void sub_93c4(void) // return-dupe
{
  int v1; // eax
  
  v1 = sub_ae6b();
  if (!v1)
    return;
  error(dat_12020,v1,gettext("standard file descriptors"));
}

// Function: sub_940a @ 0x940a
int sub_940a(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_af37(a0,a1,a2,a3,a4);
  if (v1 <= -1) {
    if (*__errno_location() == 0xc)
      sub_9384(); // no-return
  }
  return v1;
}

// Function: sub_9468 @ 0x9468
long sub_9468(char *a0,unsigned long a1)
{
  long v1; // rax
  
  v1 = sub_b300(a0,a1);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_9384(); // no-return
  }
  return v1;
}

// Function: sub_94b1 @ 0x94b1
long sub_94b1(char *a0,char *a1,char *a2)
{
  long v1; // rax
  
  v1 = sub_b623(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_9384(); // no-return
  }
  return v1;
}

// Function: sub_9502 @ 0x9502
int sub_9502(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_bf07((unsigned char)*a0);
    v2 = sub_bf07((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_957c @ 0x957c
unsigned long sub_957c(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_9596 @ 0x9596
void sub_9596(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_95b4 @ 0x95b4
void sub_95b4(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_95eb @ 0x95eb
void sub_95eb(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_9614 @ 0x9614
void sub_9614(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_9663 @ 0x9663
char * sub_9663(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_96ab @ 0x96ab
void sub_96ab(struct_4 *a0)
{
  char v1;
  long v2;
  char *v3;
  char *v4;
  
  if (a0->field_0x14)
    return;
  if (!a0->field_0x8) {
    v1 = *a0->field_0x18;
    if (sub_c204((int)v1)) {
      a0->field_0x20 = 1;
      a0->field_0x2c = (int)*a0->field_0x18;
      a0->field_0x28 = 1;
      goto label_98b8;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x9[3]))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    a0->field_0x8 = 1;
  }
  v2 = a0->field_0x0;
  v3 = a0->field_0x18;
  v4 = a0->field_0x18;
  a0->field_0x20 = sub_bc04(&a0->field_0x2c,v4,v2 - (long)v3,(mbstate_t *)&a0->field_0x9[3]);
  if (a0->field_0x20 != -1) { // branch-flip
    if (a0->field_0x20 != -2) { // branch-flip
      if (!a0->field_0x20) {
        a0->field_0x20 = 1;
        if (*a0->field_0x18)
          __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next"); // no-return
        if (a0->field_0x2c)
          __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
      }
      a0->field_0x28 = 1;
      if (mbsinit((mbstate_t *)&a0->field_0x9[3]))
        a0->field_0x8 = 0;
    }
    else {
      a0->field_0x20 = a0->field_0x0 - (long)a0->field_0x18;
      a0->field_0x28 = 0;
    }
  }
  else {
    a0->field_0x20 = 1;
    a0->field_0x28 = 0;
  }
label_98b8:
  a0->field_0x14 = 1;
  return;
}

// Function: sub_98c5 @ 0x98c5
void sub_98c5(struct_0 *a0,long a1)
{
  a0->field_0x18 = a0->field_0x18 + a1;
  a0->field_0x0 = a0->field_0x0 + a1;
}

// Function: sub_9904 @ 0x9904
void sub_9904(struct_9 *a0,struct_9 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  a0->field_0x8 = a1->field_0x8;
  if (a0->field_0x8) // branch-flip
    memcpy(&a0->field_0x9[3],&a1->field_0x9[3],8);
  else {
    memset(&a0->field_0x9[3],0,8);
  }
  a0->field_0x14 = a1->field_0x14;
  sub_c15e((struct_6 *)&a0[1],(struct_7 *)&a1[1]);
}

// Function: sub_99aa @ 0x99aa
unsigned long sub_99aa(char *a0,long a1,unsigned long a2,unsigned long *a3) // return-dupe
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
      v2 = sub_bff7(a2 << 3);
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
        goto label_9b9f;
      }
    }
    v11 += 1;
    *(unsigned long *)(v10 * 8 + v2) = v10 - v11;
label_9b9f:
  }
  *v3 = 0;
  v12 = 0;
  v13 = v8;
  v14 = v8;
  do {
    while( true ) {
      if (!*v14) goto label_9c6a;
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
label_9c6a:
  *(unsigned long *)((long)v5 + -8) = 0x9c76;
  v16 = v2;
  sub_c0c8(v2);
  return 1;
}

// Function: sub_9c91 @ 0x9c91
unsigned long sub_9c91(void *a0,void *a1,long *a2)
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
  v22 = sub_c249(a1);
  if (0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0) && !SUB168(ZEXT816(0x38) * ZEXT816(v22),8)) { // branch-flip
    if (0xfa1 <= v22 * 0x38) { // branch-flip
      v3 = sub_bff7(v22 * 0x38);
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
  *(unsigned long *)&v12[-8] = 0x9e89;
  v23 = v3;
  v24 = v3;
  v25 = v4;
  v26 = v4;
  memset(v6,0,8);
  while( true ) {
    v33 = 0;
    *(unsigned long *)&v12[-8] = 0x9eea;
    sub_ab95(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) break;
    *(unsigned long *)&v12[-8] = 0x9ec3;
    sub_c15e((struct_6 *)(v24 + v17 * 0x30),&v7);
    v7 = (void *)(v34 + (long)v7);
    v17 += 1;
  }
  *(unsigned long *)(v26 + 8) = 1;
  v19 = 0;
  v18 = 2;
label_a0f5:
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
          *(unsigned long *)&v12[-8] = 0xa04b;
          if (!memcmp(v1,v2,v3)) {
            v13 = 1;
            goto label_a05e;
          }
        }
        v13 = 0;
      }
label_a05e:
      if (v13) {
        v19 += 1;
        *(unsigned long *)(v18 * 8 + v26) = v18 - v19;
        goto label_a0ed;
      }
      if (!v19) goto label_a0a0;
      v19 -= *(long *)(v26 + v19 * 8);
    } while( true );
  }
  *v14 = 0;
  v20 = 0;
  v29 = v16;
  v10[0] = 0;
  *(unsigned long *)&v12[-8] = 0xa151;
  memset(v9,0,8);
  v28 = 0;
  v7 = v16;
  v8[0] = 0;
  *(unsigned long *)&v12[-8] = 0xa181;
  memset(v6,0,8);
  v33 = 0;
  do {
    *(unsigned long *)&v12[-8] = 0xa3d7;
    sub_ab95(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) goto label_a3fd;
    if ((*(char *)(v24 + v20 * 0x30 + 0x10)) && (v35))
      v13 = *(int *)(v24 + v20 * 0x30 + 0x14) == v36;
    else {
      if (*(long *)(v24 + v20 * 0x30 + 8) == v34) {
        v3 = *(unsigned long *)(v24 + v20 * 0x30 + 8);
        v2 = *(void **)(v24 + v20 * 0x30);
        *(unsigned long *)&v12[-8] = 0xa272;
        if (!memcmp(v2,v7,v3)) {
          v13 = 1;
          goto label_a285;
        }
      }
      v13 = 0;
    }
label_a285:
    if (v13) {
      v20 += 1;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
      if (v20 == v22) {
        *v14 = (long)v29;
label_a3fd:
        *(unsigned long *)&v12[-8] = 0xa40c;
        sub_c0c8(v23);
        return 1;
      }
    }
    else if (v20) { // branch-flip
      v21 = *(long *)(v26 + v20 * 8);
      v20 -= v21;
      for (; v21; v21 = v21 + -1) {
        *(unsigned long *)&v12[-8] = 0xa31b;
        sub_ab95(v10);
        if ((v31 != '\x01') || (v32))
          v13 = 1;
        else {
          v13 = 0;
        }
        if (!v13) {
          *(unsigned long *)&v12[-8] = 0xa342; // no-return
          abort();
        }
        v29 = (void *)(v30 + (long)v29);
        v28 = 0;
      }
    }
    else {
      *(unsigned long *)&v12[-8] = 0xa37b;
      sub_ab95(v10);
      if ((v31 != '\x01') || (v32))
        v13 = 1;
      else {
        v13 = 0;
      }
      if (!v13) {
        *(unsigned long *)&v12[-8] = 0xa3a2; // no-return
        abort();
      }
      v29 = (void *)(v30 + (long)v29);
      v28 = 0;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
    }
  } while( true );
label_a0a0:
  *(unsigned long *)(v18 * 8 + v26) = v18;
label_a0ed:
  v18 += 1;
  goto label_a0f5;
}

// Function: sub_a427 @ 0xa427
char * sub_a427(char *a0,char *a1) // early-return, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_a427
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
    sub_ab95(v7);
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
        sub_ab95(v10);
        if ((v41 != '\x01') || (v42))
          v16 = 1;
        else {
          v16 = 0;
        }
        if (!v16)
          return NULL;
        if (((v2) && (10 <= v18)) && (v18 * 5 <= v19)) {
          for (v21 = v19 - v20; v21; v21 = v21 + -1) {
            sub_ab95(v11);
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
          sub_ab95(v11);
          if ((v36 != '\x01') || (v37))
            v16 = 1;
          else {
            v16 = 0;
          }
          if (!v16) {
            if (sub_9c91(a0,a1,&v12))
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
          sub_ab95(v15);
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
            sub_ab95(v15);
            if ((v51 != '\x01') || (v52))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16)
              return v39;
            sub_ab95(v13);
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
          if (sub_99aa(v17,a1,v5,&v12))
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

// Function: sub_ab95 @ 0xab95
void sub_ab95(struct_5 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_c204((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_ad9b;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_b716(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_bc04(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
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
label_ad9b:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_adac @ 0xadac
void sub_adac(void *a0,long a1)
{
  *(long *)((long)a0 + 0x10) = *(long *)((long)a0 + 0x10) + a1;
}

// Function: sub_add6 @ 0xadd6
void sub_add6(struct_10 *a0,struct_10 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_c15e((struct_6 *)&a0[1].field_0x1[2],(struct_7 *)&a1[1].field_0x1[2]);
}

// Function: sub_ae6b @ 0xae6b
int sub_ae6b(void)
{
  int v1; // eax
  int v2; // stack - 0x18
  
  v2 = 0;
  do {
    if (3 <= v2)
      return 0;
    if ((int)sub_b75e(v2,1) <= -1) {
      if (v2) // branch-flip
        v1 = -1;
      else {
        v1 = open("/dev/full",1);
      }
      if (v1 <= -1)
        v1 = open("/dev/null",(unsigned int)(v2 == 0));
      if (v1 <= -1)
        return *__errno_location();
      if (3 <= v1) {
        close(v1);
        return 0;
      }
    }
    v2 += 1;
  } while( true );
}

// Function: sub_af37 @ 0xaf37
unsigned long sub_af37(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe x2, return-dupe
{
  char v1 [4104];
  char *v10; // stack - 0x1030
  long v11; // stack - 0x1028
  long v12; // stack - 0x1020
  char *v2; // stack - 0x1058
  char *v3; // stack - 0x1060
  long v4; // stack - 0x1068
  unsigned long v5; // stack - 0x1070
  char *v6; // stack - 0x1050
  char *v7; // stack - 0x1048
  long v8; // stack - 0x1040
  long v9; // stack - 0x1038
  
  iconv(a2,0,0,0,0);
  v7 = NULL;
  v5 = a0;
  v4 = a1;
  while (v4) {
    v3 = v1;
    v2 = (char *)0x1000;
    v8 = iconv(a2,&v5,&v4,&v3,&v2);
    if ((v8 == -1) && (*__errno_location() != 7)) {
      if (*__errno_location() != 0x16)
        return 0xffffffff;
      break;
    }
    v7 = &v3[(long)v7 - (long)v1];
  }
  v3 = v1;
  v2 = (char *)0x1000;
  v9 = iconv(a2,0,0,&v3,&v2);
  if (v9 == -1)
    return 0xffffffff;
  v7 = &v3[(long)v7 - (long)v1];
  if (!v7) {
    *a4 = 0;
    return 0;
  }
  v10 = v7;
  if ((*a3) && (v7 <= (char *)*a4))
    v6 = (char *)*a3;
  else {
    v6 = malloc((unsigned long)v7);
    if (!v6) {
      *__errno_location() = 0xc;
      return 0xffffffff;
    }
  }
  iconv(a2,0,0,0,0);
  v3 = v6;
  v2 = v10;
  v5 = a0;
  v4 = a1;
  do {
    if (!v4) goto label_b247;
    v11 = iconv(a2,&v5,&v4,&v3,&v2);
  } while (v11 != -1);
  if (*__errno_location() == 0x16) {
label_b247:
    v12 = iconv(a2,0,0,&v3,&v2);
    if (v12 != -1) {
      if (v2)
        abort(); // no-return
      *a3 = (long)v6;
      *a4 = (unsigned long)v10;
      return 0;
    }
  }
  if (v6 == (char *)*a3)
    return 0xffffffff;
  free(v6);
  return 0xffffffff;
}

// Function: sub_b300 @ 0xb300
char * sub_b300(char *a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v10; // stack - 0x60
  long v11; // stack - 0x58
  unsigned long v12; // stack - 0x50
  char *v13; // stack - 0x48
  long v14; // stack - 0x40
  unsigned long v15; // stack - 0x38
  long v16; // stack - 0x28
  unsigned long v17; // stack - 0x20
  char *v18; // stack - 0x18
  char *v2;
  long v3; // stack - 0x80
  char *v4; // stack - 0x88
  unsigned long v5; // stack - 0x90
  char *v6; // stack - 0x98
  char *v7; // stack - 0x78
  unsigned long v8; // stack - 0x70
  unsigned long v9; // stack - 0x68
  
  v6 = a0;
  v5 = strlen(a0);
  v8 = v5;
  v9 = 0xffffffff;
  if (v5 <= 0xfffffff)
    v8 = v5 << 4;
  v8 += 1;
  v7 = malloc(v8);
  if (!v7) {
    *__errno_location() = 0xc;
    return NULL;
  }
  iconv(a1,0,0,0,0);
  v4 = v7;
  v1 = v8;
  while( true ) {
    v3 = v1 - 1;
    v10 = iconv(a1,&v6,&v5,&v4,&v3);
    if ((v10 != -1) || (*__errno_location() == 0x16)) break;
    if (*__errno_location() != 7) {
      free(v7); // return-dupe
      return NULL;
    }
    v11 = (long)v4 - (long)v7;
    v12 = v8 * 2;
    if (v12 <= v8) {
      *__errno_location() = 0xc;
      free(v7);
      return NULL;
    }
    v13 = realloc(v7,v12);
    if (!v13) {
      *__errno_location() = 0xc;
      free(v7);
      return NULL;
    }
    v8 = v12;
    v4 = &v13[v11];
    v1 = v12 - v11;
    v7 = v13;
  }
  while( true ) {
    v14 = iconv(a1,0,0,&v4,&v3);
    if (v14 != -1) {
      v2 = &v4[1];
      *v4 = 0;
      v15 = (long)v2 - (long)v7;
      if ((v15 < v8) && (v4 = v2, v2 = realloc(v7,v15), v2))
        v7 = v2;
      return v7;
    }
    if (*__errno_location() != 7) {
      free(v7);
      return NULL;
    }
    v16 = (long)v4 - (long)v7;
    v17 = v8 * 2;
    if (v17 <= v8) break;
    v18 = realloc(v7,v17);
    if (!v18) {
      *__errno_location() = 0xc;
      free(v7);
      return NULL;
    }
    v8 = v17;
    v4 = &v18[v16];
    v3 = (v17 - v16) + -1;
    v7 = v18;
  }
  *__errno_location() = 0xc;
  free(v7);
  return NULL;
}

// Function: sub_b623 @ 0xb623
char * sub_b623(char *a0,char *a1,char *a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  
  if ((*a0) && (sub_9502(a1,a2))) {
    v3 = iconv_open(a2,a1);
    if (v3 == -1)
      return NULL;
    v2 = (char *)sub_b300(a0,v3);
    if (v2) { // branch-flip
      if ((int)iconv_close(v3) <= -1) {
        free(v2);
        return NULL;
      }
    }
    else {
      v1 = *__errno_location();
      iconv_close(v3);
      *__errno_location() = v1;
    }
    return v2;
  }
  v2 = strdup(a0);
  if (!v2)
    *__errno_location() = 0xc;
  return v2;
}

// Function: sub_b716 @ 0xb716
unsigned long sub_b716(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_b75e @ 0xb75e
int sub_b75e(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_bad0(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_bb01(a0,(unsigned int)v2);
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

// Function: sub_bad0 @ 0xbad0
int sub_bad0(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_bb01 @ 0xbb01
int sub_bb01(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_22620) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_22620 = 1;
    else {
      v3 = sub_bad0(a0,a1);
      if (0 <= v3)
        dat_22620 = -1;
    }
  }
  else {
    v3 = sub_bad0(a0,a1);
  }
  if ((0 <= v3) && (dat_22620 == -1)) {
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

// Function: sub_bc04 @ 0xbc04
unsigned long sub_bc04(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_bf4d(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_bcae @ 0xbcae
unsigned long sub_bcae(int a0) // return-dupe
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

// Function: sub_bcec @ 0xbcec
unsigned long sub_bcec(int a0) // return-dupe
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

// Function: sub_bd1e @ 0xbd1e
bool sub_bd1e(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_bd3f @ 0xbd3f
unsigned int sub_bd3f(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_bd67 @ 0xbd67
unsigned long sub_bd67(int a0) // return-dupe
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

// Function: sub_bd94 @ 0xbd94
bool sub_bd94(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_bdb8 @ 0xbdb8
bool sub_bdb8(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_bddc @ 0xbddc
bool sub_bddc(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_be00 @ 0xbe00
bool sub_be00(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_be24 @ 0xbe24
unsigned long sub_be24(int a0) // return-dupe, return-dupe x3
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

// Function: sub_be6e @ 0xbe6e
unsigned long sub_be6e(int a0) // return-dupe
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

// Function: sub_be9b @ 0xbe9b
bool sub_be9b(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_bebf @ 0xbebf
unsigned long sub_bebf(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_bf07 @ 0xbf07
int sub_bf07(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_bf2a @ 0xbf2a
int sub_bf2a(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_bf4d @ 0xbf4d
unsigned int sub_bf4d(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_c3f9(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_bff7 @ 0xbff7
long sub_bff7(unsigned long a0) // early-return
{
  void *v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  
  if ((a0 <= a0 + 0x8000000000000020) && (v1 = malloc(a0 + 0x20), v1)) {
    v2 = (long)v1 + 0x10U & 0xffffffffffffffe0;
    v3 = v2 + 0x10;
    *(char *)(v2 + 0xf) = (char)v3 - (char)v1;
    return v3;
  }
  return 0;
}

// Function: sub_c0c8 @ 0xc0c8
void sub_c0c8(void *a0) // return-dupe
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1)));
}

// Function: sub_c123 @ 0xc123
unsigned int sub_c123(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_c15e @ 0xc15e
void sub_c15e(struct_6 *a0,struct_7 *a1) // return-dupe
{
  if ((struct_7 *)a1->field_0x0 != &a1[1]) // branch-flip
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

// Function: sub_c204 @ 0xc204
bool sub_c204(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0xe360) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_c249 @ 0xc249
char * sub_c249(char *a0)
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
      sub_ab95(v4);
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

// Function: sub_c319 @ 0xc319
char * sub_c319(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_c341 @ 0xc341
unsigned long sub_c341(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_c319(a0);
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

// Function: sub_c3f9 @ 0xc3f9
void sub_c3f9(unsigned int a0,char *a1,unsigned long a2)
{
  sub_c341(a0,a1,a2);
}

// Function: sub_c427 @ 0xc427
void sub_c427(unsigned int a0)
{
  sub_c319(a0);
}

// Function: _DT_FINI @ 0xc444
void _DT_FINI(void)
{
  return;
}
