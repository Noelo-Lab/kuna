// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_18fd0)
    return;
  (*dat_18fd0)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_18d60)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x34e0
void __cxa_finalize(void)
{
  (*dat_18fe0)(); // jump-as-call
}

// Function: __strcat_chk @ 0x34f0
char * __strcat_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_18d68)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x3500
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_18d70)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x3510
int sigprocmask(int a0,sigset_t *a1,sigset_t *a2)
{
  int v1; // eax
  
  v1 = (*dat_18d78)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x3520
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_18d80)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x3530
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_18d88)(); // jump-as-call
  return v1;
}

// Function: free @ 0x3540
void free(void *a0)
{
  (*dat_18d90)(); // jump-as-call
}

// Function: utimensat @ 0x3550
int utimensat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_18d98)(); // jump-as-call
  return v1;
}

// Function: localtime @ 0x3560
tm * localtime(void *a0)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_18da0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x3570
void abort(void)
{
  (*dat_18da8)(); // jump-as-call
}

// Function: __errno_location @ 0x3580
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_18db0)(); // jump-as-call
  return v1;
}

// Function: fdatasync @ 0x3590
int fdatasync(int a0)
{
  int v1; // eax
  
  v1 = (*dat_18db8)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x35a0
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_18dc0)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x35b0
void _exit(int a0)
{
  (*dat_18dc8)(); // jump-as-call
}

// Function: strcpy @ 0x35c0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_18dd0)(); // jump-as-call
  return v1;
}

// Function: unlinkat @ 0x35d0
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_18dd8)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x35e0
void qsort(void)
{
  (*dat_18de0)(); // jump-as-call
}

// Function: isatty @ 0x35f0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_18de8)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3600
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_18df0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3610
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18df8)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3620
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_18e00)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x3630
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_18e08)(); // jump-as-call
  return v1;
}

// Function: write @ 0x3640
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_18e10)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3650
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_18e18)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x3660
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_18e20)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x3670
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_18e28)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x3680
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_18e30)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x3690
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_18e38)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x36a0
void __stack_chk_fail(void)
{
  (*dat_18e40)(); // jump-as-call
}

// Function: getopt_long @ 0x36b0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_18e48)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x36c0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_18e50)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x36d0
void lseek(void)
{
  (*dat_18e58)(); // jump-as-call
}

// Function: memset @ 0x36e0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18e60)(); // jump-as-call
  return v1;
}

// Function: close @ 0x36f0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_18e68)(); // jump-as-call
  return v1;
}

// Function: strspn @ 0x3700
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_18e70)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x3710
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_18e78)(); // jump-as-call
  return v1;
}

// Function: strcspn @ 0x3720
unsigned long strcspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_18e80)(); // jump-as-call
  return v1;
}

// Function: read @ 0x3730
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_18e88)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x3740
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_18e90)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3750
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_18e98)(); // jump-as-call
  return v1;
}

// Function: utimes @ 0x3760
int utimes(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_18ea0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3770
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18ea8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3780
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_18eb0)(); // jump-as-call
  return v1;
}

// Function: putc @ 0x3790
int putc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_18eb8)(); // jump-as-call
  return v1;
}

// Function: getchar @ 0x37a0
int getchar(void)
{
  int v1; // eax
  
  v1 = (*dat_18ec0)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x37b0
void signal(void)
{
  (*dat_18ec8)(); // jump-as-call
}

// Function: dirfd @ 0x37c0
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_18ed0)(); // jump-as-call
  return v1;
}

// Function: __memcpy_chk @ 0x37d0
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18ed8)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x37e0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_18ee0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x37f0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_18ee8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3800
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18ef0)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x3810
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_18ef8)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x3820
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_18f00)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3830
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18f08)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3840
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_18f10)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3850
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_18f18)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3860
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18f20)(); // jump-as-call
  return v1;
}

// Function: __strcpy_chk @ 0x3870
char * __strcpy_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_18f28)(); // jump-as-call
  return v1;
}

// Function: fchmod @ 0x3880
int fchmod(int a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_18f30)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3890
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18f38)(); // jump-as-call
  return v1;
}

// Function: fsync @ 0x38a0
int fsync(int a0)
{
  int v1; // eax
  
  v1 = (*dat_18f40)(); // jump-as-call
  return v1;
}

// Function: open @ 0x38b0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_18f48)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x38c0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_18f50)(); // jump-as-call
  return v1;
}

// Function: fchown @ 0x38d0
int fchown(int a0,unsigned int a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = (*dat_18f58)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x38e0
void perror(char *a0)
{
  (*dat_18f60)(); // jump-as-call
}

// Function: fdopendir @ 0x38f0
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_18f68)(); // jump-as-call
  return v1;
}

// Function: futimens @ 0x3900
int futimens(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_18f70)(); // jump-as-call
  return v1;
}

// Function: atoi @ 0x3910
int atoi(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_18f78)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x3920
int sigismember(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_18f80)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3930
void exit(int a0)
{
  (*dat_18f88)(); // jump-as-call
}

// Function: fwrite @ 0x3940
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_18f90)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x3950
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_18f98)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3960
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_18fa0)(); // jump-as-call
  return v1;
}

// Function: futimesat @ 0x3970
int futimesat(int a0,char *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_18fa8)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x3980
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18fb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3990
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_18fb8)(); // jump-as-call
  return v1;
}

// Function: sub_39a0 @ 0x39a0
undefined16 sub_39a0(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x39c0
int main(int argc,char **argv,char **envp)
{
  int v1;
  char **v10;
  long v11; // fs_offset
  unsigned long v12; // stack - 0x40
  int v2;
  unsigned int v3;
  char *v4;
  unsigned long v5; // rax
  long v6; // rax
  char **v7; // stack - 0x48
  int v8; // stack - 0x50
  int v9; // stack - 0x4c
  
  v12 = *(unsigned long *)(v11 + 0x28);
  v4 = (char *)sub_d3b0(*argv);
  dat_1b850 = v4;
  v5 = strlen(v4);
  if (5 <= v5) {
    if (!strcmp(&v4[v5 - 4],".exe"))
      v4[v5 - 4] = '\0';
  }
  v7 = argv;
  dat_dd9d0 = sub_d3d0(&v8,&v7,"GZIP");
  v10 = NULL;
  if (dat_dd9d0)
    v10 = v7;
  dat_dd9c8 = ".gz";
  dat_dd9c0 = strlen(".gz");
  v9 = -1;
  if (v10) { // branch-flip
    v4 = v10[optind];
    if ((!v4) || (strcmp(v4,"--"))) {
      v1 = getopt_long(v8,v10,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",(option *)0x187c0,&v9);
      v2 = v1 + 0x83;
      if (v1 < 0) {
        if (optind != v8) {
          sub_f630(stderr,"%s: %s: non-option in GZIP environment variable\n",dat_1b850,v10[optind]);
          sub_7040(); // no-return
        }
        if ((optind != 1) && (!dat_1b85c))
          sub_f630(stderr,"%s: warning: GZIP environment variable is deprecated; use an alias or script\n",dat_1b850);
        free(v10);
        v9 = -1;
        optind = 1;
        goto label_3b75;
      }
      v3 = v1 + 0x52;
      goto label_3b5c;
    }
    if (!v10) goto label_3b75;
    v2 = 0xb0;
  }
  else {
label_3b75:
    v2 = getopt_long(argc,argv,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",(option *)0x187c0,&v9);
    v1 = optind;
    if (v2 < 0) {
      if (dat_19028 <= -1)
        dat_19028 = dat_dd9f8;
      if (dat_1902c <= -1)
        dat_1902c = dat_dd9f8;
      if (dat_dd9fc) { // branch-flip
        if (!dat_1b85c) {
          sub_f630(stderr,"%s: option --ascii ignored on this system\n",dat_1b850);
          goto label_4085;
        }
        if (0x1e <= dat_dd9c0 - 1) {
          sub_f630(stderr,"%s: invalid suffix \'%s\'\n",dat_1b850,dat_dd9c8); // return-dupe
          sub_6fd0(1); // no-return
        }
label_3f9e:
        dat_dd920 = 0xd;
      }
      else {
label_4085:
        if (0x1e <= dat_dd9c0 - 1) {
          sub_f630(stderr,"%s: invalid suffix \'%s\'\n",dat_1b850,dat_dd9c8);
          sub_6fd0(1);
        }
        dat_dd920 = 0;
        if (dat_1b85c) goto label_3f9e;
      }
      sub_5570();
      if (argc != v1) { // branch-flip
        while (v6 = (long)optind, optind < argc) {
          optind += 1;
          sub_7b60(argv[v6]);
        }
      }
      else {
        sub_7070();
      }
      if ((dat_dd510) && (close(0))) {
        dat_1b420 = 0x69647473;
        dat_1b424 = 0x6e;
        sub_d600(); // no-return
      }
      if (dat_dd9e8) {
        if ((!dat_1b85c) && (2 <= argc - v1))
          sub_6c40(0xffffffff);
        v2 = sub_e460(stdout);
        if (v2) {
          sub_d670(); // no-return, return-dupe
        }
      }
      if (((!dat_1b860) || ((((!dat_dda00 || (!fdatasync(1))) || (*__errno_location() == 0x16)) && (!close(1))))) || (*__errno_location() == 9))
        sub_6fd0(dat_1b84c); // no-return
      sub_d670();
    }
    v3 = v2 - 0x31;
label_3b5c:
    if (v3 <= 0xd3) {
      v2 = *(int *)((unsigned long)v3 * 4 + 0x12f80);
      return (*(void *)((long)v2 + 0x12f80))(); // jump-as-call
    }
    if ((v2 <= 0x82) || (v2 == 0xc2)) {
      sub_7040(); // no-return, return-dupe
    }
  }
  sub_f630(stderr,"%s: ",dat_1b850);
  if (0 <= v9) // branch-flip
    sub_f630(stderr,"--%s: ",*(unsigned long *)((long)v9 * 0x20 + 0x187c0));
  else {
    sub_f630(stderr,"-%c: ",v2 + -0x83);
  }
  sub_f630(stderr,"option not valid in GZIP environment variable\n");
  sub_7040();
}

// Function: sub_4170 @ 0x4170
void sub_4170(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_18fc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_41a0 @ 0x41a0
void sub_41a0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4210
void _FINI_0(void)
{
  if (!dat_1a048) {
    if (dat_18fe0)
      __cxa_finalize(dat_19008);
    sub_41a0();
    dat_1a048 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4250
void _INIT_0(void)
{
  return;
}

// Function: sub_4260 @ 0x4260
void sub_4260(int a0) // return-dupe
{
  dat_1a058 = 0;
  dat_1a05c = 0;
  if (a0 == -1)
    return;
  dat_1a050 = sub_e010;
  dat_1a058 = 0;
  dat_1a05c = 0;
}

// Function: sub_4290 @ 0x4290
void sub_4290(unsigned int a0,int a1)
{
  int v1;
  unsigned short v2;
  unsigned int v3; // esi
  unsigned long v4; // rsi
  unsigned long v5; // r8
  
  dat_1a05c = (unsigned short)(a0 << ((unsigned char)dat_1a058 & 0x1f)) | dat_1a05c;
  v1 = dat_1a058;
  if (0x10U - a1 < dat_1a058) {
    v3 = dat_1b004 + 1;
    v4 = (unsigned long)v3;
    if (0x3fffe <= dat_1b004) { // branch-flip
      *(char *)((unsigned long)dat_1b004 + 0x5c000) = (char)dat_1a05c;
      v2 = dat_1a05c;
      if (v3 == 0x40000) {
        dat_1b004 = v3;
        sub_d7a0();
        v4 = (unsigned long)dat_1b004;
        v2 = dat_1a05c;
      }
      dat_1b004 = (int)v4 + 1;
      *(char *)(v4 + 0x5c000) = (char)(v2 >> 8);
      if (dat_1b004 == 0x40000)
        sub_d7a0();
    }
    else {
      v5 = (unsigned long)dat_1b004;
      dat_1b004 += 2;
      *(char *)(v5 + 0x5c000) = (char)dat_1a05c;
      *(char *)(v4 + 0x5c000) = (char)(dat_1a05c >> 8);
    }
    v1 = dat_1a058 + -0x10;
    dat_1a05c = (unsigned short)((int)(a0 & 0xffff) >> (0x10U - (char)dat_1a058 & 0x1fU));
  }
  dat_1a058 = a1 + v1;
}

// Function: sub_4380 @ 0x4380
unsigned int sub_4380(unsigned int a0,int a1)
{
  unsigned int v1; // eax
  unsigned int v2;
  unsigned int v3; // edi
  
  v2 = 0;
  do {
    a1 -= 1;
    v3 = a0 >> 1;
    v1 = a0 & 1 | v2;
    v2 = v1 * 2;
    a0 = v3;
  } while (0 < a1);
  return v1 & 0x7fffffff;
}

// Function: sub_43b0 @ 0x43b0
void sub_43b0(void) // return-dupe x4
{
  unsigned long v1; // rax
  char v2; // cl
  unsigned int v3; // edx
  
  v2 = (char)dat_1a05c; // branch-flip
  if (9 <= dat_1a058) {
    v3 = dat_1b004 + 1;
    v1 = (unsigned long)v3;
    if (dat_1b004 <= 0x3fffd) {
      *(char *)((unsigned long)dat_1b004 + 0x5c000) = v2;
      *(char *)(v1 + 0x5c000) = (char)((unsigned short)dat_1a05c >> 8);
      dat_1b004 += 2;
      dat_1a058 = 0;
      dat_1a05c = 0;
      return;
    }
    *(char *)((unsigned long)dat_1b004 + 0x5c000) = v2;
    if (v3 == 0x40000) {
      dat_1b004 = v3;
      sub_d7a0();
      v1 = (unsigned long)dat_1b004;
    }
    dat_1b004 = (int)v1 + 1;
    *(char *)(v1 + 0x5c000) = (char)((unsigned short)dat_1a05c >> 8);
    if (dat_1b004 != 0x40000) {
      dat_1a058 = 0;
      dat_1a05c = 0;
      return;
    }
  }
  else {
    if (dat_1a058 <= 0) {
      dat_1a058 = 0;
      dat_1a05c = 0;
      return;
    }
    v1 = (unsigned long)dat_1b004;
    dat_1b004 += 1;
    *(char *)(v1 + 0x5c000) = v2;
    if (dat_1b004 != 0x40000) {
      dat_1a058 = 0;
      dat_1a05c = 0;
      return;
    }
  }
  sub_d7a0();
  dat_1a058 = 0;
  dat_1a05c = 0;
}

// Function: sub_44a0 @ 0x44a0
void sub_44a0(char *a0,unsigned int a1,int a2) // return-dupe x2
{
  char v1;
  unsigned char v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned int v5;
  char *v6; // r12
  unsigned char v7; // r13b
  
  sub_43b0();
  if (a2) {
    v2 = (unsigned char)(a1 >> 8);
    v5 = dat_1b004 + 1;
    v4 = (unsigned long)v5;
    v7 = (unsigned char)a1; // branch-flip
    if (0x3fffe <= dat_1b004) {
      *(unsigned char *)((unsigned long)dat_1b004 + 0x5c000) = v7;
      if (v5 == 0x40000) {
        dat_1b004 = v5;
        sub_d7a0();
        v4 = (unsigned long)dat_1b004;
      }
      dat_1b004 = (int)v4 + 1;
      v3 = (unsigned long)dat_1b004;
      *(unsigned char *)(v4 + 0x5c000) = v2;
      if (dat_1b004 == 0x40000) {
        sub_d7a0();
        v3 = (unsigned long)dat_1b004;
      }
    }
    else {
      v3 = (unsigned long)(dat_1b004 + 2);
      *(unsigned char *)((unsigned long)dat_1b004 + 0x5c000) = v7;
      *(unsigned char *)(v4 + 0x5c000) = v2;
    }
    v5 = (unsigned int)v3;
    dat_1b004 = v5 + 1;
    v4 = (unsigned long)dat_1b004;
    if (0x3fffe <= v5) { // branch-flip
      *(unsigned char *)(v3 + 0x5c000) = ~v7;
      if (dat_1b004 == 0x40000) {
        sub_d7a0();
        v4 = (unsigned long)dat_1b004;
      }
      dat_1b004 = (int)v4 + 1;
      *(unsigned char *)(v4 + 0x5c000) = ~v2;
      if (dat_1b004 == 0x40000)
        sub_d7a0();
    }
    else {
      dat_1b004 = v5 + 2;
      *(unsigned char *)(v3 + 0x5c000) = ~v7;
      *(unsigned char *)(v4 + 0x5c000) = ~v2;
    }
  }
  v6 = &a0[a1];
  if (!a1)
    return;
  do {
    while( true ) {
      v4 = (unsigned long)dat_1b004;
      v1 = *a0;
      a0 = &a0[1];
      dat_1b004 += 1;
      *(char *)(v4 + 0x5c000) = v1;
      if (dat_1b004 == 0x40000) break;
      if (a0 == v6)
        return;
    }
    sub_d7a0();
  } while (a0 != v6);
}

// Function: sub_4620 @ 0x4620
unsigned int sub_4620(unsigned int a0)
{
  char *v1;
  int v10; // esi
  long v11; // rsi
  char *v12;
  unsigned int v13;
  int v14;
  char v15;
  unsigned int v16; // stack - 0x34
  unsigned short v2;
  bool v3;
  bool v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7; // rax
  unsigned int v8;
  char v9;
  
  v5 = (unsigned long)dat_1a06c;
  v13 = dat_1a06c;
  if (dat_1a06c <= 0x7ef9)
    v13 = 0x7efa;
  v11 = (long)(int)dat_1a070;
  v1 = (char *)(v5 + 0x3c000);
  v9 = *(char *)(v5 + 0x3bfff + v11);
  v15 = v1[v11];
  v8 = dat_1a064;
  if (dat_1a060 <= dat_1a070)
    v8 = dat_1a064 >> 2;
  v4 = 0;
  v14 = dat_1a070;
  do {
    v6 = (unsigned long)a0;
    if ((((((char *)(v6 + 0x3c000))[v11] != v15) || (*(char *)(v6 + 0x3bfff + v11) != v9)) || (*(char *)(v6 + 0x3c000) != *v1)) || (*(char *)(v6 + 0x3c001) != *(char *)(v5 + 0x3c001))) {
label_4698:
      v2 = *(unsigned short *)((unsigned long)(a0 & 0x7fff) * 2 + 0x1b880);
      v3 = v4;
    }
    else {
      v12 = (char *)(v5 + 0x3c002);
      v7 = (char *)(v6 + 0x3c002);
      while( true ) {
        if (v12[1] != v7[1]) {
          v12 = &v12[1];
          goto label_4770;
        }
        if (v12[2] != v7[2]) break;
        if (v12[3] != v7[3]) {
          v12 = &v12[3];
          goto label_4770;
        }
        if (v12[4] != v7[4]) {
          v12 = &v12[4];
          goto label_4770;
        }
        if (v12[5] != v7[5]) {
          v12 = &v12[5];
          goto label_4770;
        }
        if (v12[6] != v7[6]) {
          v12 = &v12[6];
          goto label_4770;
        }
        if (v12[7] != v7[7]) {
          v12 = &v12[7];
          goto label_4770;
        }
        v12 = &v12[8];
        v7 = &v7[8];
        if ((*v12 != *v7) || ((char *)(v5 + 0x3c102) <= v12)) goto label_4770;
      }
      v12 = &v12[2];
label_4770:
      v10 = 0x102 - ((int)(char *)(v5 + 0x3c102) - (int)v12);
      if (v10 <= v14) goto label_4698;
      if (dat_1a080 <= v10) {
        dat_1a068 = a0;
        return v10;
      }
      v2 = *(unsigned short *)((unsigned long)(a0 & 0x7fff) * 2 + 0x1b880);
      v9 = *(char *)(v5 + 0x3bfff + (long)v10);
      v4 = 1;
      v3 = 1;
      v15 = v1[v10];
      v14 = v10;
      v16 = a0;
    }
    a0 = (unsigned int)v2;
    if ((a0 <= v13 - 0x7efa) || (v8 = v8 - 1, v3 = v4, !v8)) {
      if (!v3)
        return v14;
      dat_1a068 = v16;
      return v14;
    }
    v11 = (long)v14;
  } while( true );
}

// Function: sub_4840 @ 0x4840
void sub_4840(unsigned int a0,unsigned int a1) // return-dupe
{
  unsigned char *v1;
  unsigned char v2;
  bool v3;
  unsigned long v4; // rax
  unsigned char *v5;
  unsigned long v6;
  
  v4 = (unsigned long)a0;
  if (a0 <= 0xfff) {
    v5 = (unsigned char *)((unsigned long)a0 + 0x3c000);
    v1 = &v5[a1];
    v3 = 0;
    v6 = dat_1a090;
    do {
      if (v1 == v5) {
        if (!v3)
          return;
        dat_1a090 = v6;
        return;
      }
      v2 = *v5;
      v5 = &v5[1];
      v6 += v2;
      v3 = 1;
    } while ((unsigned char *)((unsigned long)a0 + 0x3c001 + (unsigned long)(0xfff - a0)) != v5);
    v4 = 0x1000;
    dat_1a090 = v6;
  }
  if (a0 + a1 <= (unsigned int)v4)
    return;
  v3 = 0;
  v6 = dat_1a088;
  do {
    dat_1a090 += (unsigned long)*(unsigned char *)(v4 + 0x3c000) - (unsigned long)*(unsigned char *)((unsigned long)(unsigned int)((int)v4 - 0x1000) + 0x3c000);
    if ((v6 == 0xffffffff) && (!(dat_1a090 & 0xfff))) {
      v3 = 1;
      v6 = v4;
    }
    v4 += 1;
  } while ((unsigned int)v4 < a0 + a1);
  if (!v3)
    return;
  dat_1a088 = v6;
}

// Function: sub_4940 @ 0x4940
void sub_4940(void) // return-dupe
{
  short *v1;
  short v2;
  short *v3; // rax
  unsigned long v4; // rbx
  unsigned int v5; // eax
  int v6; // esi
  unsigned int v7; // r12d
  
  v4 = (unsigned long)dat_1a09c;
  v5 = dat_1a09c + dat_1a06c;
  v6 = 0x10000 - v5;
  if (v6 != -1) { // branch-flip
    if (0xfefa <= dat_1a06c) {
      v7 = dat_1a06c - 0x8000;
      memcpy((void *)0x3c000,(void *)0x44000,0x8000);
      dat_1a068 -= 0x8000;
      if (dat_1a088 != 0xffffffff)
        dat_1a088 -= 0x8000;
      dat_1a078 -= 0x8000;
      v1 = (short *)0x2b880;
      do {
        v2 = *v1 + -0x8000;
        if (0 <= *v1)
          v2 = 0;
        v3 = &v1[1];
        *v1 = v2;
        v1 = v3;
      } while (v3 != (short *)0x3b880);
      v1 = (short *)0x1b880;
      do {
        v2 = *v1 + -0x8000;
        if (0 <= *v1)
          v2 = 0;
        v3 = &v1[1];
        *v1 = v2;
        v1 = v3;
      } while (v3 != (short *)0x2b880);
      v6 = 0x18000 - v5;
      dat_1a06c = v7;
    }
  }
  else {
    v6 = -2;
  }
  if (dat_1a0a0)
    return;
  v5 = (*dat_1a050)(dat_1a06c + v4 + 0x3c000,v6);
  if ((unsigned int)(v5 - 1U) <= 0xfffffffd) {
    dat_1a09c += v5;
    return;
  }
  dat_1a0a0 = 1;
  *(unsigned short *)((unsigned long)dat_1a06c + (unsigned long)dat_1a09c + 0x3c000) = 0;
}

// Function: sub_4aa0 @ 0x4aa0
void sub_4aa0(void)
{
  unsigned short *v1;
  unsigned long v10; // rdx
  char *v11;
  unsigned int v12;
  short *v2;
  unsigned short v3;
  short v4;
  int v5; // eax
  unsigned int v6;
  unsigned int v7;
  unsigned int v8; // eax
  int v9;
  
  dat_1a070 = 2;
  if (!dat_1a09c) {
    v5 = -1;
label_4c1a:
    v11 = NULL;
    if (0 <= (long)dat_1a078)
      v11 = (char *)((dat_1a078 & 0xffffffff) + 0x3c000);
    sub_b070(v11,dat_1a06c - dat_1a078,v5,1); // tail-call
    return;
  }
  v12 = dat_1a09c;
label_4ae0:
  do {
    v10 = (unsigned long)dat_1a06c;
    dat_1a0a4 = (dat_1a0a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(dat_1a06c + 2) + 0x3c000)) & 0x7fff;
    v1 = (unsigned short *)((unsigned long)(dat_1a0a4 + 0x8000) * 2 + 0x1b880);
    v3 = *v1;
    *(unsigned short *)((unsigned long)(dat_1a06c & 0x7fff) * 2 + 0x1b880) = v3;
    *v1 = (unsigned short)dat_1a06c;
    if (((v3) && (dat_1a06c - v3 <= 0x7efa)) && (dat_1a06c <= 0xfefa)) {
      v6 = sub_4620(v3);
      if (v6 <= v12)
        v12 = v6;
      if (v12 <= 2) {
        v10 = (unsigned long)dat_1a06c;
        goto label_4b3a;
      }
      v5 = sub_b290(dat_1a06c - dat_1a068,v12 - 3);
      v9 = dat_1b000;
      v6 = dat_1a06c;
      dat_1a09c -= v12;
      if (dat_1b000) { // branch-flip
        sub_4840(dat_1a06c,v12);
        if (dat_1a098 < v12) goto label_4cc0;
label_4d21:
        v7 = v6;
        do {
          v8 = v7 + 1;
          dat_1a0a4 = (dat_1a0a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(v7 + 3) + 0x3c000)) & 0x7fff;
          v2 = (short *)((unsigned long)(dat_1a0a4 + 0x8000) * 2 + 0x1b880);
          v4 = *v2;
          *(short *)((unsigned long)(v8 & 0x7fff) * 2 + 0x1b880) = v4;
          *v2 = (short)v8;
          v7 = v8;
        } while (v8 != (v12 - 1) + v6);
        dat_1a06c = v6 + v12;
        *(short *)((unsigned long)(v8 & 0x7fff) * 2 + 0x1b880) = v4;
        *v2 = (short)v6 + -1 + (short)v12;
      }
      else {
        if (v12 <= dat_1a098) goto label_4d21;
label_4cc0:
        dat_1a06c = v12 + v6;
        dat_1a0a4 = (unsigned int)*(unsigned char *)((unsigned long)dat_1a06c + 0x3c000) << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(dat_1a06c + 1) + 0x3c000);
      }
    }
    else {
label_4b3a:
      v5 = sub_b290(0,(unsigned char)*(char *)(v10 + 0x3c000));
      v9 = dat_1b000;
      v12 = dat_1a06c;
      if (dat_1b000)
        sub_4840(dat_1a06c,1);
      dat_1a06c = v12 + 1;
      dat_1a09c -= 1;
    }
    if ((v9) && (dat_1a088 < dat_1a06c)) {
      v9 = 1;
      v5 = 2;
      dat_1a088 = 0xffffffff;
label_4ba5:
      v11 = (char *)((dat_1a078 & 0xffffffff) + 0x3c000);
      if ((long)dat_1a078 <= -1)
        v11 = NULL;
      sub_b070(v11,dat_1a06c - dat_1a078,v9,0);
      dat_1a078 = (unsigned long)dat_1a06c;
      v12 = dat_1a09c;
    }
    else {
      v12 = dat_1a09c;
      if (v5) {
        v9 = v5 + -1;
        goto label_4ba5;
      }
    }
    while (dat_1a09c = v12, v12 <= 0x105) {
      if (dat_1a0a0) {
        if (!v12) {
          v5 -= 1;
          dat_1a09c = 0;
          goto label_4c1a;
        }
        goto label_4ae0;
      }
      sub_4940();
      v12 = dat_1a09c;
    }
  } while( true );
}

// Function: sub_4db0 @ 0x4db0
void sub_4db0(int a0)
{
  long v1; // rdi
  
  if (8 < (unsigned int)(a0 - 1U))
    sub_d510("bad pack level"); // no-return
  memset((void *)0x2b880,0,0x10000);
  v1 = (long)a0;
  dat_1a090 = 0;
  dat_1a088 = 0xffffffff;
  dat_1a06c = 0;
  dat_1a078 = 0;
  dat_1a098 = (unsigned int)*(unsigned short *)(v1 * 8 + 0x12022);
  dat_1a060 = (unsigned int)*(unsigned short *)(v1 * 8 + 0x12020);
  dat_1a080 = (unsigned int)*(unsigned short *)(v1 * 8 + 0x12024);
  dat_1a064 = (unsigned int)*(unsigned short *)(v1 * 8 + 0x12026);
  dat_1a09c = (*dat_1a050)(0x3c000,0x10000);
  if (0xfffffffe <= dat_1a09c - 1) {
    dat_1a0a0 = 1;
    dat_1a09c = 0;
    return;
  }
  dat_1a0a0 = 0;
  if (dat_1a09c <= 0x105) {
    do {
      sub_4940();
      if (0x106 <= dat_1a09c) break;
    } while (!dat_1a0a0);
  }
  dat_1a0a4 = (unsigned int)dat_3c000 << 5 ^ (unsigned int)dat_3c001;
}

// Function: sub_4ed0 @ 0x4ed0
void sub_4ed0(unsigned long a0)
{
  int v1;
  int v10;
  unsigned int v11;
  unsigned int v12;
  char *v13; // rdi
  int v14; // eax
  int v15;
  unsigned short *v2;
  short *v3;
  unsigned short v4;
  short v5;
  unsigned int v6;
  unsigned int v7; // eax
  unsigned int v8;
  unsigned long v9; // rax
  
  sub_4db0(a0);
  if ((int)a0 <= 3) {
    sub_4aa0(); // tail-call
    return;
  }
  v14 = -1;
  if (!dat_1a09c) {
label_509d:
    v13 = NULL;
    if (0 <= (long)dat_1a078)
      v13 = (char *)((dat_1a078 & 0xffffffff) + 0x3c000);
    sub_b070(v13,dat_1a06c - dat_1a078,v14,1); // tail-call
    return;
  }
  v15 = 0;
  v14 = 0;
  v11 = dat_1a09c;
  dat_1a070 = 2;
  do {
    v10 = dat_1a068;
    v9 = (unsigned long)dat_1a06c;
    v12 = 2;
    dat_1a0a4 = (dat_1a0a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(dat_1a06c + 2) + 0x3c000)) & 0x7fff;
    v2 = (unsigned short *)((unsigned long)(dat_1a0a4 + 0x8000) * 2 + 0x1b880);
    v4 = *v2;
    *(unsigned short *)((unsigned long)(dat_1a06c & 0x7fff) * 2 + 0x1b880) = v4;
    *v2 = (unsigned short)dat_1a06c;
    v8 = dat_1a070;
    if ((v4) && (dat_1a070 < dat_1a098)) {
      if ((0x7efb <= dat_1a06c - v4) || (0xfefb <= dat_1a06c))
        v12 = 2;
      else {
        v8 = sub_4620(v4);
        v12 = v11;
        if (v8 <= v11)
          v12 = v8;
        v8 = dat_1a070;
        if (v12 != 3) // branch-flip
          v9 = (unsigned long)dat_1a06c;
        else {
          v9 = (unsigned long)dat_1a06c;
          if (0x1001 <= dat_1a06c - dat_1a068)
            v12 = 2;
        }
      }
    }
    v11 = (unsigned int)v9;
    if ((3 <= v8) && (v12 <= v8)) {
      v14 = sub_b290((v11 - v10) + -1,v8 - 3);
      v10 = dat_1b000;
      v8 = dat_1a070;
      v12 = dat_1a06c;
      v15 = dat_1a070 - 2;
      v11 = (dat_1a09c + 1) - dat_1a070;
      dat_1a09c = v11;
      if (dat_1b000) {
        v1 = dat_1a070 - 1;
        dat_1a070 = v15;
        sub_4840(dat_1a06c,v1);
      }
      v6 = v12;
      do {
        v7 = v6 + 1;
        dat_1a0a4 = (dat_1a0a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(v6 + 3) + 0x3c000)) & 0x7fff;
        v3 = (short *)((unsigned long)(dat_1a0a4 + 0x8000) * 2 + 0x1b880);
        v5 = *v3;
        *(short *)((unsigned long)(v7 & 0x7fff) * 2 + 0x1b880) = v5;
        *v3 = (short)v7;
        v6 = v7;
      } while (v7 != v15 + v12);
      dat_1a06c = (v8 - 1) + v12;
      *(short *)((unsigned long)(v7 & 0x7fff) * 2 + 0x1b880) = v5;
      *v3 = (short)v12 + -2 + (short)v8;
      dat_1a070 = 0;
      if ((v10) && (dat_1a088 < dat_1a06c)) {
        v10 = 1;
        v14 = 2;
        dat_1a088 = 0xffffffff;
      }
      else {
        if (!v14) {
          v15 = 0;
          v12 = 2;
          goto label_5055;
        }
        v10 = v14 + -1;
      }
      v12 = 2;
      v13 = (char *)((dat_1a078 & 0xffffffff) + 0x3c000);
      if ((long)dat_1a078 <= -1)
        v13 = NULL;
      v15 = 0;
      sub_b070(v13,dat_1a06c - dat_1a078,v10,0);
      dat_1a078 = (unsigned long)dat_1a06c;
      v11 = dat_1a09c;
    }
    else if (v15) { // branch-flip
      v14 = sub_b290(0,(unsigned char)*(char *)((unsigned long)(v11 - 1) + 0x3c000));
      if (dat_1b000) { // branch-flip
        v9 = (unsigned long)dat_1a06c;
        if (v9 <= dat_1a088) { // branch-flip
          v10 = v14 + -1;
          if (v14) goto label_512c;
        }
        else {
          v14 = 2;
          dat_1a088 = 0xffffffff;
          v10 = v15;
label_512c:
          v13 = (char *)((dat_1a078 & 0xffffffff) + 0x3c000);
          if ((long)dat_1a078 <= -1)
            v13 = NULL;
          sub_b070(v13,dat_1a06c - dat_1a078,v10,0);
          v9 = (unsigned long)dat_1a06c;
          dat_1a078 = v9;
          if (!dat_1b000) goto label_5168;
        }
        sub_4840(v9,1);
        dat_1a06c = (unsigned int)v9;
      }
      else {
        v10 = v14 + -1;
        if (v14) goto label_512c;
      }
label_5168:
      dat_1a06c += 1;
      dat_1a09c -= 1;
      v11 = dat_1a09c;
    }
    else {
      if (dat_1b000) {
        if (dat_1a088 < v9) {
          v14 = 2;
          dat_1a088 = 0xffffffff;
          v13 = (char *)((dat_1a078 & 0xffffffff) + 0x3c000);
          if ((long)dat_1a078 <= -1)
            v13 = NULL;
          sub_b070(v13,v9 - dat_1a078,1,0);
          v9 = (unsigned long)dat_1a06c;
          v11 = dat_1a06c;
          dat_1a078 = v9;
          if (!dat_1b000) goto label_5030;
        }
        v11 = (unsigned int)v9;
        sub_4840(v9,1);
      }
label_5030:
      dat_1a06c = v11 + 1;
      v15 = 1;
      dat_1a09c -= 1;
      v11 = dat_1a09c;
    }
label_5055:
    while (v11 <= 0x105) {
      if (dat_1a0a0) {
        v11 = dat_1a09c;
        if (!dat_1a09c) {
          v14 -= 1;
          if (v15)
            sub_b290(0,(unsigned char)*(char *)((unsigned long)(dat_1a06c - 1) + 0x3c000));
          goto label_509d;
        }
        break;
      }
      sub_4940();
      v11 = dat_1a09c;
    }
    dat_1a070 = v12;
  } while( true );
}

// Function: sub_53f0 @ 0x53f0
void sub_53f0(void) // return-dupe
{
  char v1 [16];
  
  dat_1b838 = 0xffffffffffffffff;
  if ((dat_dd078 & 0xf000) != 0x8000) {
    dat_1b840 = 0xffffffffffffffff;
    return;
  }
  dat_1b840 = dat_dd090;
  if ((dat_19028) && (!dat_dd9e8)) {
    dat_1b838 = 0xffffffffffffffff;
    return;
  }
  v1 = sub_e940((void *)0xdd060);
  dat_1b830 = SUB168(v1,0);
  dat_1b838 = SUB168(v1,8);
}

// Function: sub_5470 @ 0x5470
void sub_5470(char *a0,char *a1)
{
  char *v1;
  char v2;
  
  while( true ) {
    v2 = *a1;
    a1 = &a1[1];
    v1 = &a0[1];
    *a0 = v2;
    if (!v2) break;
    a0 = v1;
  }
}

// Function: sub_5490 @ 0x5490
void sub_5490(void)
{
  long *v1; // rbx
  char *v2;
  
  v1 = (long *)0x186e0;
  sub_f7b0("Usage: %s [OPTION]... [FILE]...\n",dat_1b850);
  v2 = "Compress or uncompress FILEs (by default, compress FILES in-place).";
  do {
    v1 = &v1[1];
    sub_f7b0("%s\n",v2);
    v2 = (char *)*v1;
  } while (v2);
}

// Function: sub_54e0 @ 0x54e0
void sub_54e0(void)
{
  long *v1; // rbx
  char *v2;
  
  v1 = (long *)0x18b20;
  sub_f7b0("%s %s\n",dat_1b850,dat_193f8);
  v2 = "Copyright (C) 2018 Free Software Foundation, Inc.";
  do {
    v1 = &v1[1];
    sub_f7b0("%s\n",v2);
    v2 = (char *)*v1;
  } while (v2);
}

// Function: sub_5540 @ 0x5540
void sub_5540(void)
{
  sub_54e0();
  sub_f7b0("\n");
  sub_f7b0("Written by Jean-loup Gailly.\n"); // tail-call
}

// Function: sub_5570 @ 0x5570
void sub_5570(void)
{
  int *v1; // rbx
  long v2; // rbx
  void *v3 [17]; // stack - 0xd8
  int v4;
  undefined4 v5; // stack - 0x50
  
  v4 = 2;
  v1 = (int *)0x13350;
  sigemptyset((sigset_t *)0xdd940);
  while( true ) {
    sigaction(v4,NULL,(sigaction *)v3);
    if (v3[0] != (void *)0x1)
      sigaddset((sigset_t *)0xdd940,v4);
    v1 = &v1[1];
    if (v1 == (int *)0x13368) break;
    v4 = *v1;
  }
  v5 = 0;
  v3[0] = sub_57e0;
  v2 = 0;
  v3[1] = (void *)dat_dd940;
  v3[2] = (void *)dat_dd948;
  v4 = 2;
  v3[3] = (void *)dat_dd950;
  v3[4] = (void *)dat_dd958;
  v3[5] = (void *)dat_dd960;
  v3[6] = (void *)dat_dd968;
  v3[7] = (void *)dat_dd970;
  v3[8] = (void *)dat_dd978;
  v3[9] = (void *)dat_dd980;
  v3[10] = (void *)dat_dd988;
  v3[0xb] = (void *)dat_dd990;
  v3[0xc] = (void *)dat_dd998;
  v3[0xd] = (void *)dat_dd9a0;
  v3[0xe] = (void *)dat_dd9a8;
  v3[0xf] = (void *)dat_dd9b0;
  v3[0x10] = (void *)dat_dd9b8;
  while( true ) {
    if (sigismember((void *)0xdd940,v4)) {
      if (!v2)
        dat_dd9e0 = 1;
      sigaction(v4,(sigaction *)v3,NULL);
    }
    v2 += 1;
    if (v2 == 6) break;
    v4 = *(int *)(v2 * 4 + 0x13350);
  }
}

// Function: sub_56e0 @ 0x56e0
void sub_56e0(bool a0) // return-dupe x3
{
  int v1;
  sigset_t v2;
  char v3 [1032];
  char *v4;
  
  v1 = dat_19018;
  if (!a0) {
    sigprocmask(0,(sigset_t *)0xdd940,&v2);
    v1 = dat_19018;
    if (dat_19018 <= -1) {
      sigprocmask(2,&v2,NULL);
      return;
    }
    dat_19018 = 0xffffffff;
    close(v1);
    v4 = v3;
    sub_5470(v4,(char *)0xdd520);
    sub_d3c0(v4);
    sigprocmask(2,&v2,NULL);
    return;
  }
  if (dat_19018 <= -1)
    return;
  dat_19018 = 0xffffffff;
  close(v1);
  v4 = v3;
  sub_5470(v4,(char *)0xdd520);
  sub_d3c0(v4);
}

// Function: sub_57e0 @ 0x57e0
void sub_57e0(int a0)
{
  sub_56e0(1);
  if (dat_dd920 != a0) {
    signal(a0,0);
    raise(a0); // tail-call
    return;
  }
  _exit(2); // no-return
}

// Function: sub_5820 @ 0x5820
void sub_5820(char *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  sub_f630(stderr,"%s: ",dat_1b850);
  *v2 = v1;
  perror(a0);
  dat_1b84c = 1;
}

// Function: sub_5870 @ 0x5870
unsigned long sub_5870(void) // return-dupe
{
  unsigned long v1; // rax
  
  if (!dat_dd9f4) {
    sub_f630(stderr,"%s: %s already exists;",dat_1b850,0x1b020);
    if ((dat_dd9e0) && ((dat_dda01 || (isatty(0))))) {
      sub_f630(stderr," do you wish to overwrite (y or n)? ");
      sub_e460(stderr);
      if (sub_f5f0()) goto label_58de;
    }
    sub_f630(stderr,"\tnot overwritten\n");
    if (dat_1b84c)
      return 1;
    dat_1b84c = 2;
    return 1;
  }
label_58de:
  v1 = sub_d3c0((char *)0x1b020);
  if ((int)v1) {
    sub_5820((char *)0x1b020);
    return 1;
  }
  return v1;
}

// Function: sub_5940 @ 0x5940
unsigned int sub_5940(void)
{
  if ((dat_dd9f8) && (!dat_dd9dc)) {
    if (dat_1b008 != dat_1b00c)
      return 0;
    if ((dat_1b00c == 0x40000) && (sub_d830(1) != -1)) {
      dat_1b008 = 0;
      return 0;
    }
  }
  return 1;
}

// Function: sub_59a0 @ 0x59a0
bool sub_59a0(char *a0,unsigned long a1)
{
  unsigned long v1;
  bool v2;
  
  if (a1) // branch-flip
    v1 = a1;
  else {
    a1 = 1;
    v1 = 1;
    a0 = ".";
  }
  v2 = 0;
  if (!memcmp((void *)0xdd100,a0,a1))
    v2 = *(char *)(v1 + 0xdd100) == '\0';
  return v2;
}

// Function: sub_5a00 @ 0x5a00
void sub_5a00(char *a0,unsigned long a1) // return-dupe
{
  char *v1; // rax
  unsigned long v2;
  
  if (sub_59a0(a0,a1))
    return;
  if (0 <= dat_19014)
    close(dat_19014);
  v2 = a1;
  if (!a1) {
    a1 = 1;
    v2 = 1;
    a0 = ".";
  }
  v1 = __memcpy_chk((void *)0xdd100,a0,a1,0x400);
  v1[v2] = '\0';
  dat_19014 = sub_e3c0(v1,0x10000);
}

// Function: sub_5a90 @ 0x5a90
void sub_5a90(long a0,unsigned int a1)
{
  char v1;
  unsigned long v2; // rax
  char v3; // stack - 0x21
  
label_5aba:
  if (!a0)
    return;
  if (dat_1b00c <= dat_1b008) goto label_5b25;
  do {
    v2 = (unsigned long)dat_1b008;
    dat_1b008 += 1;
    v1 = *(char *)(v2 + 0x9d000);
    v3 = v1;
    while( true ) {
      if (a1 & 2) {
        sub_d1f0(&v3,1);
        v1 = v3;
      }
      v3 = v1;
      if (a0 != -1) {
        a0 -= 1;
        goto label_5aba;
      }
      if (!v3)
        return;
      if (dat_1b008 < dat_1b00c) break;
label_5b25:
      v1 = sub_d830(0);
      v3 = v1;
    }
  } while( true );
}

// Function: sub_5b50 @ 0x5b50
unsigned int sub_5b50(unsigned int a0) // early-return x9, return-dupe x2
{
  unsigned short v1;
  unsigned int v10;
  int v11;
  char v12; // stack - 0x4a
  char v13; // stack - 0x4c
  char *v14; // rax
  unsigned long v15; // rax
  unsigned char *v16;
  char v17; // stack - 0x4b
  char v18; // stack - 0x49
  char v19; // stack - 0x48
  unsigned char v2;
  char v20; // stack - 0x47
  unsigned int v21; // stack - 0x46
  char v22; // stack - 0x42
  char v23; // stack - 0x41
  char v3;
  unsigned short v4; // ax
  unsigned int v5;
  unsigned int v6;
  unsigned int v7;
  unsigned char *v8; // rax
  unsigned long v9; // rax
  
  v15 = (unsigned long)dat_1b008;
  if ((dat_dd9f4) && (dat_1b860)) {
    if (dat_1b00c <= dat_1b008) { // branch-flip
      v15 = sub_d830(1);
      v7 = (unsigned int)v15;
      v6 = dat_1b008;
      v10 = dat_1b00c;
    }
    else {
      dat_1b008 += 1;
      v2 = *(unsigned char *)(v15 + 0x9d000);
      v15 = (unsigned long)v2;
      v7 = (unsigned int)v2;
      v6 = dat_1b008;
      v10 = dat_1b00c;
    }
    v12 = (char)v15;
    if (v10 <= v6) { // branch-flip
      v15 = sub_d830(1);
      v6 = (unsigned int)v15;
    }
    else {
      dat_1b008 = v6 + 1;
      v15 = (unsigned long)*(unsigned char *)((unsigned long)v6 + 0x9d000);
      v6 = (unsigned int)*(unsigned char *)((unsigned long)v6 + 0x9d000);
    }
  }
  else {
    if (dat_1b00c <= dat_1b008) { // branch-flip
      v12 = sub_d830(0);
      v6 = dat_1b008;
      v7 = dat_1b00c;
    }
    else {
      dat_1b008 += 1;
      v12 = *(char *)(v15 + 0x9d000);
      v6 = dat_1b008;
      v7 = dat_1b00c;
    }
    if (v12) { // branch-flip
      if (v7 <= v6) // branch-flip
        v15 = sub_d830(0);
      else {
        dat_1b008 = v6 + 1;
        v15 = (unsigned long)*(unsigned char *)((unsigned long)v6 + 0x9d000);
      }
      v6 = 0;
      v7 = 0;
    }
    else {
      if (v7 <= v6) { // branch-flip
        v15 = sub_d830(1);
        v6 = (unsigned int)v15;
      }
      else {
        dat_1b008 = v6 + 1;
        v2 = *(unsigned char *)((unsigned long)v6 + 0x9d000);
        v15 = (unsigned long)v2;
        v6 = (unsigned int)v2;
      }
      v7 = 0;
    }
  }
  v18 = (char)v15;
  dat_19020 = 0xffffffff;
  dat_e0ea8 = 0;
  v11 = dat_dd9d8 + 1;
  dat_dd9dc = 0;
  dat_dd9d8 = v11;
  if ((memcmp(&v12,(void *)0x12cb4,2)) && (memcmp(&v12,(void *)0x12cb7,2))) {
    if (((memcmp(&v12,(void *)0x12cd7,2)) || (dat_1b008 != 2)) || (memcmp((void *)0x9d000,(void *)0x12cd7,4))) {
      if (!memcmp(&v12,(void *)0x12cdc,2)) {
        dat_190d0 = sub_c9f0;
        dat_19020 = 2;
        return 2;
      }
      if (!memcmp(&v12,(void *)0x12cdf,2)) {
        dat_190d0 = sub_c040;
        dat_19020 = 1;
        dat_dd9dc = 1;
        return 1;
      }
      if (!memcmp(&v12,(void *)0x12ce2,2)) {
        dat_190d0 = sub_bfd0;
        dat_19020 = 3;
        dat_dd9dc = 1;
        return 3;
      }
      if (((dat_dd9f4) && (dat_1b860)) && (!dat_dd9e8)) {
        dat_19020 = 0;
        dat_190d0 = sub_d720;
        if (v6 != 0xffffffff)
          dat_1b008 -= 1;
        dat_dd9dc = 1;
        if (v7 == 0xffffffff) {
          dat_19020 = 0;
          dat_dd9dc = 1;
          return 0;
        }
        sub_d6c0(1,&v12,1);
        goto label_667a;
      }
    }
    else {
      dat_1b008 = 0;
      dat_190d0 = sub_cd20;
      if (sub_cc20(a0))
        return 0xffffffff;
      dat_dd9dc = 1;
label_667a:
      if (0 <= (int)dat_19020)
        return dat_19020;
      v11 = dat_dd9d8;
    }
    if (v11 != 1) {
label_6060:
      if (!v12) {
        while (!v6) {
          v15 = (unsigned long)dat_1b008;
          if (dat_1b00c <= dat_1b008) // branch-flip
            v6 = sub_d830(1);
          else {
            dat_1b008 += 1;
            v6 = (unsigned int)*(unsigned char *)(v15 + 0x9d000);
          }
        }
        if (v6 == 0xffffffff) {
          if (!dat_dd9e4)
            return 0xfffffffd;
          if (!dat_1b85c)
            sub_f630(stderr,"\n%s: %s: decompression OK, trailing zero bytes ignored\n",dat_1b850,0x1b420);
          if (dat_1b84c)
            return 0xfffffffd;
          dat_1b84c = 2;
          return 0xfffffffd;
        }
      }
      if (!dat_1b85c)
        sub_f630(stderr,"\n%s: %s: decompression OK, trailing garbage ignored\n",dat_1b850,0x1b420);
      if (!dat_1b84c) {
        dat_1b84c = 2;
        return 0xfffffffe;
      }
      return 0xfffffffe;
    }
  }
  else {
    v15 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) // branch-flip
      dat_19020 = sub_d830(0);
    else {
      dat_1b008 += 1;
      dat_19020 = (unsigned int)*(unsigned char *)(v15 + 0x9d000);
    }
    if (dat_19020 != 8) {
      sub_f630(stderr,"%s: %s: unknown method %d -- not supported\n",dat_1b850,0x1b420,dat_19020);
      dat_1b84c = 1;
      return 0xffffffff;
    }
    dat_190d0 = sub_cd20;
    v15 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) // branch-flip
      v7 = sub_d830(0);
    else {
      dat_1b008 += 1;
      v7 = (unsigned int)*(unsigned char *)(v15 + 0x9d000);
    }
    if (v7 & 0x20) {
      v14 = "%s: %s is encrypted -- not supported\n";
      sub_f630(stderr,v14,dat_1b850,0x1b420); // return-dupe
      dat_1b84c = 1;
      return 0xffffffff;
    }
    if (v7 & 0xc0) { // branch-flip
      sub_f630(stderr,"%s: %s has flags 0x%x -- not supported\n",dat_1b850,0x1b420,v7 & 0xff);
      dat_1b84c = 1;
      if (dat_dd9f4 <= 1) {
        dat_1b84c = 1;
        return 0xffffffff;
      }
      v10 = dat_1b00c;
      if (dat_1b00c <= dat_1b008) goto label_6162;
label_5e0f:
      v15 = (unsigned long)dat_1b008;
      dat_1b008 += 1;
      v15 = (unsigned long)*(unsigned char *)(v15 + 0x9d000);
      if (dat_1b008 < v10) goto label_5e2c;
label_6180:
      v15 |= (long)(int)sub_d830(0) << 8;
      v10 = dat_1b00c;
      if (dat_1b00c <= dat_1b008) goto label_61a4;
label_5e4f:
      v9 = (unsigned long)dat_1b008;
      dat_1b008 += 1;
      v15 |= (unsigned long)*(unsigned char *)(v9 + 0x9d000) << 0x10;
      if (dat_1b008 < v10) goto label_5e73;
label_61c9:
      v9 = (unsigned long)(int)sub_d830(0);
    }
    else {
      v10 = dat_1b00c;
      if (dat_1b008 < dat_1b00c) goto label_5e0f;
label_6162:
      v15 = (unsigned long)(int)sub_d830(0);
      v10 = dat_1b00c;
      if (dat_1b00c <= dat_1b008) goto label_6180;
label_5e2c:
      v9 = (unsigned long)dat_1b008;
      dat_1b008 += 1;
      v15 |= (unsigned long)*(unsigned char *)(v9 + 0x9d000) << 8;
      if (dat_1b008 < v10) goto label_5e4f;
label_61a4:
      v15 |= (long)(int)sub_d830(0) << 0x10;
      if (dat_1b00c <= dat_1b008) goto label_61c9;
label_5e73:
      v9 = (unsigned long)dat_1b008;
      dat_1b008 += 1;
      v9 = (unsigned long)*(unsigned char *)(v9 + 0x9d000);
    }
    v15 |= v9 << 0x18;
    if ((v15) && (!dat_19028)) {
      if (0 <= (long)v15) { // branch-flip
        dat_1b838 = 0;
        dat_1b830 = v15;
      }
      else {
        if (!dat_1b85c)
          sub_f630(stderr,"%s: %s: MTIME %lu out of range for this platform\n",dat_1b850,0x1b420,v15);
        if (!dat_1b84c)
          dat_1b84c = 2;
        dat_1b838 = 999999999;
        dat_1b830 = 0x7fffffffffffffff;
      }
    }
    v9 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) { // branch-flip
      v22 = sub_d830(0);
      v10 = dat_1b008;
      v5 = dat_1b00c;
    }
    else {
      dat_1b008 += 1;
      v22 = *(char *)(v9 + 0x9d000);
      v10 = dat_1b008;
      v5 = dat_1b00c;
    }
    if (v5 <= v10) // branch-flip
      v23 = sub_d830(0);
    else {
      dat_1b008 = v10 + 1;
      v23 = *(char *)((unsigned long)v10 + 0x9d000);
    }
    if (v7 & 2) {
      v19 = 8;
      v21 = (unsigned int)v15;
      v20 = (char)v7;
      sub_d1f0(NULL,0);
      sub_d1f0(&v12,10);
    }
    if (v7 & 4) {
      v15 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        v3 = sub_d830(0);
        v10 = dat_1b008;
        v5 = dat_1b00c;
      }
      else {
        dat_1b008 += 1;
        v3 = *(char *)(v15 + 0x9d000);
        v10 = dat_1b008;
        v5 = dat_1b00c;
      }
      v13 = v3; // branch-flip
      if (v5 <= v10)
        v17 = sub_d830(0);
      else {
        dat_1b008 = v10 + 1;
        v17 = *(char *)((unsigned long)v10 + 0x9d000);
      }
      v1 = CONCAT11(v17,v3);
      if (dat_dd9e4)
        sub_f630(stderr,"%s: %s: extra field of %u bytes ignored\n",dat_1b850,0x1b420,v1);
      if (v7 & 2)
        sub_d1f0(&v13,2);
      sub_5a90(v1,v7 & 0xff);
    }
    if (v7 & 8) {
      if ((dat_1902c) || (((dat_1b860 && (!dat_dd9e8)) || (2 <= dat_dd9d8))))
        sub_5a90(-1,v7 & 0xff);
      else {
        v8 = (unsigned char *)sub_d3b0((char *)0x1b020);
        v16 = v8;
        while( true ) {
          v15 = (unsigned long)dat_1b008;
          if (dat_1b00c <= dat_1b008) // branch-flip
            v2 = sub_d830(0);
          else {
            dat_1b008 += 1;
            v2 = *(unsigned char *)(v15 + 0x9d000);
          }
          *v16 = v2;
          v16 = &v16[1];
          if (!v2) break;
          if ((unsigned char *)0x1b420 <= v16)
            sub_d510("corrupted input -- file name too large"); // no-return
        }
        if (v7 & 2)
          sub_d1f0(v8,(int)v16 - (int)v8);
        v14 = (char *)sub_d3b0(v8);
        memmove(v8,v14,strlen(v14) + 1);
      }
    }
    if (v7 & 0x10)
      sub_5a90(-1,v7 & 0xff);
    if (v7 & 2) {
      v4 = sub_d1f0(&v12,0);
      v15 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        v5 = sub_d830(0);
        v7 = dat_1b008;
        v10 = dat_1b00c;
      }
      else {
        dat_1b008 += 1;
        v5 = (unsigned int)*(unsigned char *)(v15 + 0x9d000);
        v7 = dat_1b008;
        v10 = dat_1b00c;
      }
      if (v10 <= v7) // branch-flip
        v7 = sub_d830(0);
      else {
        dat_1b008 = v7 + 1;
        v7 = (unsigned int)*(unsigned char *)((unsigned long)v7 + 0x9d000);
      }
      v5 = v7 << 8 | v5;
      if (v4 != v5) {
        sub_f630(stderr,"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",dat_1b850,0x1b420,v5,(unsigned int)v4);
        dat_1b84c = 1;
        if (dat_dd9f4 <= 1) {
          dat_1b84c = 1;
          return 0xffffffff;
        }
      }
    }
    if (dat_dd9d8 != 1) {
      if (0 <= (int)dat_19020)
        return dat_19020;
      goto label_6060;
    }
    dat_e0ea8 = (unsigned long)(dat_1b008 + 8);
    if (0 <= (int)dat_19020)
      return dat_19020;
  }
  v14 = "\n%s: %s: not in gzip format\n";
  sub_f630(stderr,v14,dat_1b850,0x1b420);
  dat_1b84c = 1;
  return 0xffffffff;
}

// Function: sub_67e0 @ 0x67e0
char * sub_67e0(char *a0)
{
  int v1; // eax
  char *v10;
  char *v11;
  int v2; // eax
  int v3; // eax
  unsigned long v4; // rax
  unsigned char *v5; // rax
  unsigned long v6;
  long v7; // rdx
  unsigned long *v8; // rbx
  char v9 [40];
  
  v10 = dat_dd9c8;
  v6 = dat_dd9c0;
  if (dat_19048) {
    v7 = 0x19048;
    v11 = dat_19048;
    do {
      v4 = strlen(v11);
      if ((v6 < v4) && (!strcmp(v10,&v11[v4 - v6]))) {
        v8 = (unsigned long *)0x19048;
        v5 = (unsigned char *)sub_f5d0(v10);
        sub_d350(v5);
        v7 = 8;
        goto label_6879;
      }
      v11 = *(char **)(v7 + 8);
      v7 += 8;
    } while (v11);
  }
  v5 = (unsigned char *)sub_f5d0(v10);
  sub_d350(v5);
  v7 = 0;
  v8 = (unsigned long *)0x19040;
label_6879:
  *(unsigned char **)(v7 * 8 + 0x19040) = v5;
  v1 = (int)strlen(a0);
  if (0x21 <= v1) // branch-flip
    __strcpy_chk(v9,&a0[(long)v1 + -0x20],0x21);
  else {
    __strcpy_chk(v9,a0,0x21);
  }
  sub_d350(v9);
  v6 = strlen(v9);
  v10 = (char *)*v8;
  v2 = (int)v6;
  do {
    v3 = (int)strlen(v10);
    if ((v3 < v2) && (v9[(v2 - v3) + -1] != '/')) {
      if (!strcmp(&v9[(long)v2 - (long)v3],v10)) {
        v11 = &a0[(long)v1 - (long)v3];
        break;
      }
    }
    v10 = (char *)v8[1];
    v8 = &v8[1];
    v11 = NULL;
  } while (v10);
  free(v5);
  return v11;
}

// Function: sub_69d0 @ 0x69d0
unsigned int sub_69d0(void) // return-dupe x2
{
  char *v1; // rax
  
  __strcpy_chk((char *)0x1b020,(char *)0x1b420,0x400);
  v1 = (char *)sub_67e0((char *)0x1b020);
  if (!dat_dd9f8) {
    if ((v1) && (!dat_dd9f4)) {
      if ((!dat_dd9e4) && (dat_dd9ec || dat_1b85c))
        return 2;
      sub_f630(stderr,"%s: %s already has %s suffix -- unchanged\n",dat_1b850,0x1b420,v1);
      return 2;
    }
    dat_1b848 = 0;
    if (strlen((char *)0x1b020) + dat_dd9c0 <= 0x3ff) {
      __strcat_chk((char *)0x1b020,dat_dd9c8,0x400);
      return 0;
    }
    if (!dat_1b85c)
      sub_f630(stderr,"%s: %s: file name too long\n",dat_1b850,0x1b420);
    goto label_6aa0;
  }
  if (v1) {
    sub_d350(v1);
    if ((strcmp(v1,".tgz")) && (strcmp(v1,".taz"))) {
      *v1 = '\0';
      return 0;
    }
    builtin_strncpy(v1,".tar",5);
    return 0;
  }
  if (dat_dd9ec) { // branch-flip
    if (!dat_dd9e4)
      return 2;
label_6a8a:
    if (dat_1b85c) goto label_6aa0;
  }
  else {
    if (dat_1b858)
      return 0;
    if (dat_dd9e4) goto label_6a8a;
    if (dat_1b85c)
      return 2;
  }
  sub_f630(stderr,"%s: %s: unknown suffix -- ignored\n",dat_1b850,0x1b420);
label_6aa0:
  if (dat_1b84c)
    return 2;
  dat_1b84c = 2;
  return 2;
}

// Function: sub_6c40 @ 0x6c40
void sub_6c40(int a0)
{
  tm *v1; // rax
  long v2;
  long v3;
  long v4;
  unsigned long v5;
  
  if (dat_19010) { // branch-flip
    if (a0 < 0) {
label_6d88:
      if ((1 <= dat_dd508) && (1 <= dat_dd500)) {
        if (((dat_dd9e4) && (sub_f7b0("                            "), v4 = dat_dd508, dat_dd9e4)) || (v4 = dat_dd508, !dat_1b85c)) {
          sub_d930(stdout,v4,0x13);
          sub_f7b0(" ");
          sub_d930(stdout,dat_dd500,0x13);
          sub_f7b0(" ");
          v4 = dat_dd508;
        }
        sub_d8e0(dat_dd500 - (v4 - dat_e0ea8),dat_dd500,stdout);
        sub_f7b0(" (totals)\n"); // tail-call
        return;
      }
      return;
    }
    dat_19010 = 0;
    if (dat_dd9e4)
      sub_f7b0("method  crc     date  time  ");
    if (!dat_1b85c)
      sub_f7b0("%*.*s %*.*s  ratio uncompressed_name\n",0x13,0x13,"compressed",0x13,0x13,"uncompressed");
  }
  else if (a0 <= -1) goto label_6d88;
  v5 = 0xffffffffffffffff;
  if ((a0 == 8) && (!dat_dd9dc))
    v5 = dat_e0e90;
  if (dat_dd9e4) {
    v1 = localtime((void *)0x1b830);
    sub_f7b0("%5s %08lx ",*(unsigned long *)((long)a0 * 8 + 0x18680),v5);
    if (v1) // branch-flip
      sub_f7b0("%s%3d %02d:%02d ",&"Jan"[(long)*(int *)&v1->field_0x10 * 4],*(unsigned int *)&v1->field_0xc,*(unsigned int *)&v1->field_0x8,*(unsigned int *)&v1->field_0x4);
    else {
      sub_f7b0("??? ?? ??:?? ");
    }
  }
  sub_d930(stdout,dat_1b828,0x13);
  sub_f7b0(" ");
  sub_d930(stdout,dat_1b820,0x13);
  sub_f7b0(" ");
  if (dat_1b828 != -1) { // branch-flip
    if (0 <= dat_dd508)
      dat_dd508 += dat_1b828;
    v4 = dat_1b828;
    v2 = dat_1b820;
    v3 = dat_1b820;
    if (dat_1b820 == -1) {
      dat_dd500 = -1;
      v4 = 0;
      v3 = 0;
      dat_e0ea8 = 0;
      dat_1b820 = 0;
      dat_1b828 = 0;
      sub_d8e0(v4,v3,stdout); // return-dupe
      sub_f7b0(" %s\n",0x1b020); // tail-call
      return;
    }
  }
  else {
    dat_dd508 = -1;
    v3 = 0;
    v2 = 0;
    v4 = 0;
    dat_e0ea8 = 0;
    dat_1b820 = 0;
    dat_1b828 = 0;
  }
  if (0 <= dat_dd500)
    dat_dd500 += v2;
  v4 = v3 - (v4 - dat_e0ea8);
  sub_d8e0(v4,v3,stdout);
  sub_f7b0(" %s\n",0x1b020);
  return;
}

// Function: sub_6fd0 @ 0x6fd0
void sub_6fd0(int a0)
{
  if (!dat_dd040) {
    dat_dd040 = 1;
    free(dat_dd9d0);
    dat_dd9d0 = NULL;
  }
  exit(a0); // no-return
}

// Function: sub_7010 @ 0x7010
void sub_7010(void)
{
  int v1; // eax
  
  v1 = sub_e0f0(stdout);
  if (v1)
    sub_d670(); // no-return
  sub_6fd0(0); // no-return
}

// Function: sub_7040 @ 0x7040
void sub_7040(void)
{
  sub_f630(stderr,"Try `%s --help\' for more information.\n",dat_1b850);
  sub_6fd0(1); // no-return
}

// Function: sub_7070 @ 0x7070
void sub_7070(void)
{
  int v1; // eax
  char *v2; // rcx
  char *v3; // r8
  bool v4; // zf
  
  if ((dat_dd9f4 || dat_dd9e8) || ((!dat_dda01 && (v4 = dat_dd9f8 == 0, !isatty((unsigned int)v4))))) {
    dat_1b420 = 0x69647473;
    dat_1b424 = 0x6e;
    dat_1b020 = 0x6f647473;
    dat_1b024 = 0x7475;
    dat_1b026 = 0;
    if (!fstat(0,(stat *)0xdd060)) {
      sub_53f0();
      sub_d270();
      dat_1b860 = 1;
      dat_dd9d8 = 0;
      dat_1b014 = 0;
      dat_dd510 = 1;
      if ((dat_dd9f8) && (dat_19020 = sub_5b50(0), dat_19020 <= -1))
        sub_6fd0(dat_1b84c); // no-return
      while( true ) {
        v1 = (*dat_190d0)(0,1);
        if (v1)
          return;
        if (sub_5940()) break;
        dat_19020 = sub_5b50(dat_1b014);
        if (dat_19020 < 0)
          return;
        dat_1b820 = 0;
      }
      if (dat_dd9e8) {
        sub_6c40(dat_19020); // tail-call
        return;
      }
      if (!dat_dd9e4)
        return;
      if (dat_1b858) {
        sub_f630(stderr," OK\n"); // tail-call
        return;
      }
      if (dat_dd9f8)
        return;
      sub_d8e0((dat_1b828 - dat_1b820) + dat_e0ea8,dat_1b828,stderr);
      sub_f630(stderr,"\n"); // tail-call
      return;
    }
    sub_5820("standard input");
  }
  else if (!dat_1b85c) {
    v2 = "read from";
    if (!dat_dd9f8)
      v2 = "written to";
    v3 = "de";
    if (!dat_dd9f8)
      v3 = (char *)0x12cdb;
    sub_f630(stderr,"%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n",dat_1b850,v2,v3,dat_1b850);
  }
  sub_6fd0(1); // no-return
}

// Function: sub_72b0 @ 0x72b0
int sub_72b0(void)
{
  int v1;
  bool v2;
  bool v3;
  int v4; // eax
  int v5;
  int *v6; // rax
  char *v7; // rax
  
  v2 = dat_1b860 == 0;
  v3 = dat_dd9f4 == 0;
  if (!dat_dd9f0) {
    v7 = (char *)sub_e060((char *)0x1b420);
    v5 = sub_5a00((char *)0x1b420,&v7[-0x1b420]);
    if (0 <= v5) goto label_72eb;
  }
  v7 = (char *)0x1b420;
  v5 = -100;
label_72eb:
  v4 = sub_e520(v5,v7,(-(unsigned int)(v2 && v3) & 0x20000) + 0x900);
  v5 = v4;
  if ((0 <= v4) && (fstat(v4,(stat *)0xdd060))) {
    v6 = __errno_location();
    v5 = -1;
    v1 = *v6;
    close(v4);
    *v6 = v1;
  }
  return v5;
}

// Function: sub_7380 @ 0x7380
unsigned long sub_7380(char *a0) // early-return
{
  char *v1;
  unsigned long v2; // rax
  int *v3; // rax
  unsigned long v4; // rax
  unsigned long *v5;
  int v6; // r13d
  char *v7;
  
  dat_190a0 = dat_dd9c8;
  if (strlen(a0) <= 0x3fe) {
    __strcpy_chk((char *)0x1b420,a0,0x400);
    v2 = sub_72b0();
    if (0 <= (int)v2)
      return v2;
    if (((dat_dd9f8) && (v3 = __errno_location(), *v3 == 2)) && (!sub_67e0((char *)0x1b420))) {
      v4 = strlen((char *)0x1b420);
      v5 = (unsigned long *)0x190a0;
      if (!strcmp(dat_dd9c8,".gz"))
        v5 = (unsigned long *)0x190a8;
      v6 = 0;
      v7 = (char *)*v5;
      do {
        __strcpy_chk((char *)0x1b420,a0,0x400);
        if (0x400 <= strlen(v7) + (long)(int)v4) {
          sub_f630(stderr,"%s: %s: file name too long\n",dat_1b850,a0); // return-dupe
          dat_1b84c = 1;
          return 0xffffffff;
        }
        __strcat_chk((char *)0x1b420,v7,0x400);
        v2 = sub_72b0();
        v1 = dat_dd9c8;
        if (0 <= (int)v2)
          return v2;
        if (*v3 != 2) {
          sub_5820((char *)0x1b420); // return-dupe
          return 0xffffffff;
        }
        if (!strcmp(v7,dat_dd9c8))
          v6 = 2;
        v7 = (char *)v5[1];
        v5 = &v5[1];
      } while (v7);
      __strcpy_chk((char *)0x1b420,a0,0x400);
      __strcat_chk((char *)0x1b420,v1,0x400);
      *v3 = v6;
    }
    sub_5820((char *)0x1b420);
    return 0xffffffff;
  }
  sub_f630(stderr,"%s: %s: file name too long\n",dat_1b850,a0);
  dat_1b84c = 1;
  return 0xffffffff;
}

// Function: sub_7550 @ 0x7550
void sub_7550(void) // return-dupe
{
  char v1;
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6; // rdx
  int v7; // ebx
  char *v8;
  char *v9;
  
  v3 = strlen((char *)0x1b020);
  v7 = (int)v3;
  if (dat_dd9f8) {
    if (2 <= v7) {
      *(char *)((long)v7 + 0x1b01f) = 0;
      return;
    }
    sub_d510("name too short"); // no-return
  }
  v4 = (char *)sub_67e0((char *)0x1b020);
  if (!v4)
    sub_d510("can\'t recover suffix\n"); // no-return
  *v4 = 0;
  dat_1b848 = 1;
  if ((4 < v7) && (!strcmp(&v4[-4],".tar"))) {
    *(unsigned int *)&v4[-4] = 0x7a67742e;
    *v4 = 0;
    return;
  }
  v7 = 3;
  v5 = (char *)sub_e060((char *)0x1b020);
  v1 = *v5;
  do {
    if (v1) {
      v8 = NULL;
      v9 = v5;
      do {
        v2 = (int)strcspn(v9,".");
        v6 = &v9[v2];
        if (v2 <= v7) { // branch-flip
          if (!*v6) break;
        }
        else {
          v8 = &v6[-1];
          if (!*v6) goto label_7678;
        }
        v9 = &v6[1];
      } while (v6[1]);
      if (v8) goto label_7678;
    }
    v7 -= 1;
  } while (v7);
  v5 = strrchr((char *)0x1b020,0x2e);
  if (!v5)
    sub_d510("internal error in shorten_name"); // no-return
  v5 = &v5[-(unsigned long)(v5[1] == '\0')];
  strcpy(v5,dat_dd9c8); // return-dupe, tail-call
  return;
label_7678:
  do {
    v5 = v8;
    *v5 = v5[1];
    v8 = &v5[1];
  } while (v5[1]);
  strcpy(v5,dat_dd9c8);
  return;
}

// Function: sub_76e0 @ 0x76e0
unsigned int sub_76e0(void)
{
  int v1;
  bool v2;
  int v3; // eax
  int *v4; // rax
  sigset_t v5;
  char *v6; // rax
  
  if (!dat_dd9f0) {
    v6 = (char *)sub_e060((char *)0x1b020);
    v3 = sub_5a00((char *)0x1b020,&v6[-0x1b020]);
    if (0 <= v3) goto label_7726;
  }
  v3 = -100;
  v6 = (char *)0x1b020;
label_7726:
  v4 = __errno_location();
  v2 = 0;
  do {
    while( true ) {
      sub_5470((char *)0xdd520,(char *)0x1b020);
      sigprocmask(0,(sigset_t *)0xdd940,&v5);
      dat_19018 = sub_e520(v3,v6,0xc1,0x180);
      v1 = *v4;
      dat_1b010 = dat_19018;
      sigprocmask(2,&v5,NULL);
      if (0 <= dat_1b010) {
        if ((v2) && (dat_dd9f8)) {
          if (!dat_1b85c)
            sub_f630(stderr,"%s: %s: warning, name truncated\n",dat_1b850,0x1b020);
          if (!dat_1b84c) {
            dat_1b84c = 2;
            return 0;
          }
        }
        return 0;
      }
      if (v1 != 0x11) break;
      if (sub_5870()) {
        close(dat_1b014); // return-dupe
        return 1;
      }
    }
    if (v1 != 0x24) {
      sub_5820((char *)0x1b020);
      close(dat_1b014);
      return 1;
    }
    sub_7550();
    v2 = 1;
  } while( true );
}

// Function: sub_78d0 @ 0x78d0
void sub_78d0(int a0,unsigned int a1,unsigned int a2)
{
  fchown(a0,a1,a2); // tail-call
}

// Function: sub_78e0 @ 0x78e0
void sub_78e0(void)
{
  unsigned int v1;
  int v2;
  int v3;
  int *v4; // rax
  unsigned long v5; // stack - 0x58
  char v6 [16];
  unsigned long v7; // stack - 0x50
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v1 = dat_dd078;
  v6 = sub_e920((void *)0xdd060);
  v7 = SUB168(v6,8);
  v5 = SUB168(v6,0);
  v6 = sub_e940((void *)0xdd060);
  v9 = SUB168(v6,8);
  v8 = SUB168(v6,0);
  if (((dat_dd9f8) && (0 <= dat_1b838)) && ((v8 != dat_1b830 || (dat_1b838 != v9)))) {
    v8 = dat_1b830;
    v9 = dat_1b838;
    v2 = sub_eb50(dat_1b010,(char *)0x1b020,&v5);
    if (v2) { // branch-flip
label_7a68:
      v4 = __errno_location();
      v2 = *v4;
      if (dat_1b85c) { // branch-flip
        if (dat_1b84c) goto label_79a0;
label_7a8b:
        dat_1b84c = 2;
        v3 = dat_1b85c;
      }
      else {
        sub_f630(stderr,"%s: ",dat_1b850);
        v3 = dat_1b85c;
        if (!dat_1b84c) goto label_7a8b;
      }
      if (!v3) {
        *v4 = v2;
        perror((char *)0x1b020);
      }
    }
    else if (1 < dat_dd9e4)
      sub_f630(stderr,"%s: timestamp restored\n",0x1b020);
  }
  else {
    v2 = sub_eb50(dat_1b010,(char *)0x1b020,&v5);
    if (v2) goto label_7a68;
  }
label_79a0:
  sub_78d0(dat_1b010,0xffffffff,dat_dd080);
  if (!fchmod(dat_1b010,v1 & 0x1ff)) {
    sub_78d0(dat_1b010,dat_dd07c,0xffffffff); // return-dupe
    return;
  }
  v4 = __errno_location();
  v2 = *v4;
  if (dat_1b85c) { // branch-flip
    if (dat_1b84c) {
      sub_78d0(dat_1b010,dat_dd07c,0xffffffff);
      return;
    }
label_79e9:
    dat_1b84c = 2;
    v3 = dat_1b85c;
  }
  else {
    sub_f630(stderr,"%s: ",dat_1b850);
    v3 = dat_1b85c;
    if (!dat_1b84c) goto label_79e9;
  }
  if (!v3) {
    *v4 = v2;
    perror((char *)0x1b020);
  }
  sub_78d0(dat_1b010,dat_dd07c,0xffffffff);
  return;
}

// Function: sub_7b60 @ 0x7b60
void sub_7b60(char *a0)
{
  char v1; // al
  int v2;
  int v3;
  int *v4; // rax
  char *v5; // rax
  sigset_t v6;
  
  v2 = strcmp(a0,"-");
  v3 = dat_1b860;
  if (!v2) {
    sub_7070();
    dat_1b860 = v3;
    return;
  }
  dat_1b014 = sub_7380(a0);
  if (dat_1b014 <= -1)
    return;
  if ((dat_dd078 & 0xf000) == 0x4000) {
    if (dat_dd9ec) {
      sub_8400(dat_1b014,a0); // tail-call
      return;
    }
    close(dat_1b014);
    if (!dat_1b85c)
      sub_f630(stderr,"%s: %s is a directory -- ignored\n",dat_1b850,0x1b420);
    if (dat_1b84c)
      return;
    dat_1b84c = 2;
    return;
  }
  if (!dat_1b860) {
    v3 = dat_1b014;
    if ((dat_dd078 & 0xf000) != 0x8000) { // branch-flip
      if (!dat_1b85c) {
        sub_f630(stderr,"%s: %s is not a directory or a regular file - ignored\n",dat_1b850,0x1b420);
        v3 = dat_1b014;
      }
    }
    else if (dat_dd078 & 0x800) { // branch-flip
      if (!dat_1b85c) {
        sub_f630(stderr,"%s: %s is set-user-ID on execution - ignored\n",dat_1b850,0x1b420);
        v3 = dat_1b014;
      }
    }
    else if (dat_dd078 & 0x400) { // branch-flip
      if (!dat_1b85c) {
        sub_f630(stderr,"%s: %s is set-group-ID on execution - ignored\n",dat_1b850,0x1b420);
        v3 = dat_1b014;
      }
    }
    else {
      if (dat_dd9f4) {
label_7ca8:
        sub_53f0();
        goto label_7d03;
      }
      if (dat_dd078 & 0x200) { // branch-flip
        if (!dat_1b85c) {
          sub_f630(stderr,"%s: %s has the sticky bit set - file ignored\n",dat_1b850,0x1b420);
          v3 = dat_1b014;
        }
      }
      else {
        if (dat_dd070 <= 1) goto label_7ca8;
        if (!dat_1b85c) {
          v5 = (char *)0x12cdb;
          if (dat_dd070 != 2)
            v5 = "s";
          sub_f630(stderr,"%s: %s has %lu other link%s -- file ignored\n",dat_1b850,0x1b420,dat_dd070 - 1,v5);
          v3 = dat_1b014;
        }
      }
    }
    if (!dat_1b84c)
      dat_1b84c = 2;
    close(v3); // return-dupe, tail-call
    return;
  }
  sub_53f0();
  if (dat_1b858) { // branch-flip
label_7d03:
    if (!sub_69d0()) goto label_7d10;
  }
  else {
    dat_1b026 = 0;
    dat_1b020 = 0x6f647473;
    dat_1b024 = 0x7475;
label_7d10:
    sub_d270();
    dat_dd9d8 = 0;
    if ((!dat_dd9f8) || (dat_19020 = sub_5b50(dat_1b014), 0 <= dat_19020)) {
      if (dat_1b860) { // branch-flip
        dat_1b010 = 1;
        v3 = dat_dd9e4;
label_7d4d:
        if (!dat_1b848) {
label_7f20:
          dat_1b848 = (unsigned int)(dat_1902c == 0);
        }
      }
      else {
        if (sub_76e0())
          return;
        v3 = dat_dd9e4;
        if (dat_dd9f8) goto label_7d4d;
        if (!dat_1b848) goto label_7f20;
        if (!dat_dd9e4 && !dat_1b85c) {
          sub_f630(stderr,"%s: %s compressed to %s\n",dat_1b850,0x1b420,0x1b020);
          v3 = dat_dd9e4;
          goto label_7d4d;
        }
      }
      if ((v3) && (!dat_dd9e8))
        sub_f630(stderr,"%s:\t",0x1b420);
      while (v3 = (*dat_190d0)(dat_1b014,dat_1b010), !v3) {
        if ((sub_5940()) || (dat_19020 = sub_5b50(dat_1b014), dat_19020 < 0)) goto label_7dd9;
        dat_1b820 = 0;
      }
      dat_19020 = -1;
label_7dd9:
      if (close(dat_1b014))
        sub_d600(); // no-return
      if (dat_dd9e8) {
        sub_6c40(dat_19020); // tail-call
        return;
      }
      if (dat_1b860) {
        if (dat_19020 == -1)
          return;
        goto label_7e15;
      }
      sub_78e0();
      if (((dat_dda00) && ((((0 <= dat_19014 && (fdatasync(dat_19014))) && (*__errno_location() != 0x16)) || ((fsync(dat_1b010) && (*__errno_location() != 0x16)))))) || (close(dat_1b010)))
        sub_d670(); // no-return
      if (!dat_dd9f0) {
        v5 = (char *)sub_e060((char *)0x1b420);
        v1 = sub_59a0((char *)0x1b420,&v5[-0x1b420]);
        v3 = dat_19014;
        if (v1) { // branch-flip
          sigprocmask(0,(sigset_t *)0xdd940,&v6);
          dat_19018 = 0xffffffff;
          if (v3 < 0) goto label_81b1;
          v3 = unlinkat(v3,v5,0);
        }
        else {
          sigprocmask(0,(sigset_t *)0xdd940,&v6);
label_81b1:
          dat_19018 = 0xffffffff;
          v3 = sub_d3c0((char *)0x1b420);
        }
        if (v3) { // branch-flip
          v4 = __errno_location();
          v3 = *v4;
          sigprocmask(2,&v6,NULL);
          if (v3) {
            if (dat_1b85c) { // branch-flip
              if (dat_1b84c) goto label_81d0;
label_834d:
              dat_1b84c = 2;
              v2 = dat_1b85c;
            }
            else {
              sub_f630(stderr,"%s: ",dat_1b850);
              v2 = dat_1b85c;
              if (!dat_1b84c) goto label_834d;
            }
            if (!v2) {
              *v4 = v3;
              perror((char *)0x1b420);
            }
          }
        }
        else {
          sigprocmask(2,&v6,NULL);
        }
      }
label_81d0:
      if (dat_19020 == -1) {
        if (dat_1b860)
          return;
        sub_56e0(0); // tail-call
        return;
      }
label_7e15:
      if (!dat_dd9e4)
        return;
      if (dat_1b858) // branch-flip
        sub_f630(stderr," OK");
      else if (dat_dd9f8) // branch-flip
        sub_d8e0(dat_1b820 - (dat_1b828 - dat_e0ea8),dat_1b820);
      else {
        sub_d8e0(dat_1b828 - (dat_1b820 - dat_e0ea8));
      }
      if (!dat_1b858) {
        v5 = "created";
        if (!dat_dd9f0)
          v5 = "replaced with";
        sub_f630(stderr," -- %s %s",v5,0x1b020);
      }
      sub_f630(stderr,"\n"); // tail-call
      return;
    }
  }
  v3 = dat_1b014;
  close(v3);
  return;
}

// Function: sub_8400 @ 0x8400
void sub_8400(int a0,char *a1)
{
  char *v1;
  char v2;
  char *v3;
  DIR *v4; // rax
  char *v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  char v8 [1032];
  char *v9; // stack - 0x450
  
  v4 = fdopendir(a0);
  if (!v4) {
    sub_5820(a1);
    close(a0); // tail-call
    return;
  }
  v5 = (char *)sub_e5b0(v4,0);
  v9 = v5;
  if (!v5) {
    sub_5820(a1);
    if (!closedir(v4))
      return;
    sub_5820(a1); // tail-call
    return;
  }
  if (closedir(v4))
    sub_5820(a1);
  v2 = *v9;
  v5 = v9;
  v3 = v9;
  do {
    while( true ) {
      if (!v2) {
        free(v3);
        return;
      }
      v6 = strlen(v5);
      v9 = v3;
      if ((strcmp(v5,".")) && (strcmp(v5,".."))) break;
label_84d6:
      v5 = &v5[v6 + 1];
      v2 = *v5;
      v3 = v9;
    }
    v7 = strlen(a1);
    if (v7 + v6 <= 0x3fd) {
      __strcpy_chk(v8,a1,0x400);
      if ((*(char *)sub_e060(v8)) && (v8[v7 - 1] != '/')) {
        v8[v7] = '/';
        v7 += 1;
      }
      strcpy(&v8[v7],v5);
      sub_7b60(v8);
      goto label_84d6;
    }
    v1 = &v5[v6 + 1];
    sub_f630(stderr,"%s: %s/%s: pathname too long\n",dat_1b850,a1,v5);
    v2 = *v1;
    dat_1b84c = 1;
    v5 = v1;
    v3 = v9;
  } while( true );
}

// Function: sub_8620 @ 0x8620
void sub_8620(void)
{
  sub_56e0(0);
  sub_6fd0(1); // no-return
}

// Function: sub_8640 @ 0x8640
unsigned long sub_8640(void) // return-dupe
{
  unsigned char v1;
  unsigned int v10;
  unsigned int v2;
  unsigned long v3; // rax
  unsigned int v4;
  unsigned int v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned int v8; // r14d
  unsigned int v9;
  
  v10 = dat_dda0c;
  v5 = dat_1b004;
  v4 = dat_dda0c & 7;
  v2 = dat_dda0c - v4;
  v6 = dat_dda10 >> (signed char)v4;
  if (v2 <= 0xf) {
    v9 = v2;
    do {
      v7 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        dat_1b004 = v5;
        v1 = sub_d830(0);
      }
      else {
        dat_1b008 += 1;
        v1 = *(unsigned char *)(v7 + 0x9d000);
      }
      v8 = v9 + 8;
      v6 |= (unsigned long)v1 << ((unsigned char)v9 & 0x3f);
      v9 = v8;
    } while (v8 <= 0xf);
    v2 = ((v4 - v10) + 0xf & 0xfffffff8) + 8 + v2;
  }
  v4 = (unsigned int)v6 & 0xffff;
  v10 = v2 - 0x10;
  v7 = v6 >> 0x10;
  if (v10 <= 0xf) {
    do {
      v3 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        dat_1b004 = v5;
        v1 = sub_d830(0);
      }
      else {
        dat_1b008 += 1;
        v1 = *(unsigned char *)(v3 + 0x9d000);
      }
      v9 = v10 + 8;
      v7 |= (unsigned long)v1 << ((unsigned char)v10 & 0x3f);
      v10 = v9;
    } while (v9 <= 0xf);
    v10 = (v2 - 8) + (0x1f - v2 & 0xfffffff8);
  }
  if ((~(unsigned int)v7 & 0xffff) == v4) {
    v7 >>= 0x10;
    v10 -= 0x10;
    if (!(v6 & 0xffff)) {
      dat_1b004 = v5;
      dat_dda0c = v10;
      dat_dda10 = v7;
      return 0;
    }
    do {
      v4 -= 1;
      if (8 <= v10) // branch-flip
        v10 -= 8;
      else {
        v6 = (unsigned long)dat_1b008;
        if (dat_1b00c <= dat_1b008) { // branch-flip
          dat_1b004 = v5;
          v1 = sub_d830(0);
        }
        else {
          dat_1b008 += 1;
          v1 = *(unsigned char *)(v6 + 0x9d000);
        }
        v7 |= (unsigned long)v1 << ((unsigned char)v10 & 0x3f);
      }
      v6 = (unsigned long)v5;
      v5 += 1;
      *(char *)(v6 + 0x3c000) = (char)v7;
      if (v5 == 0x8000) {
        dat_1b004 = 0x8000;
        v5 = 0;
        sub_d7e0();
      }
      v7 >>= 8;
    } while (v4);
    dat_1b004 = v5;
    dat_dda0c = v10;
    dat_dda10 = v7;
    return 0;
  }
  return 1;
}

// Function: sub_8840 @ 0x8840
unsigned long sub_8840(long a0,long a1,unsigned int a2,unsigned int a3) // return-dupe
{
  unsigned int v1;
  unsigned long v10;
  unsigned int v11; // ebx
  unsigned int v12;
  unsigned int v13;
  unsigned int v14; // r13d
  unsigned char *v15;
  unsigned long v16;
  unsigned int v17; // r15d
  unsigned short v2;
  unsigned short v3;
  unsigned short v4;
  unsigned short v5;
  unsigned char v6;
  unsigned char v7;
  unsigned int v8;
  unsigned long v9; // rax
  
  v2 = *(unsigned short *)((long)(int)a2 * 2 + 0x13380);
  v3 = *(unsigned short *)((long)(int)a3 * 2 + 0x13380);
  v16 = dat_dda10;
  v13 = dat_dda0c;
  v12 = dat_1b004;
label_88a5:
  do {
    while( true ) {
      for (; v13 < a2; v13 = v13 + 8) {
        v10 = (unsigned long)dat_1b008;
        if (dat_1b00c <= dat_1b008) { // branch-flip
          dat_1b004 = v12;
          v7 = sub_d830(0);
        }
        else {
          dat_1b008 += 1;
          v7 = *(unsigned char *)(v10 + 0x9d000);
        }
        v16 |= (unsigned long)v7 << ((unsigned char)v13 & 0x3f);
      }
      v15 = (unsigned char *)(a0 + (unsigned long)((unsigned int)v2 & (unsigned int)v16) * 0x10);
      v7 = *v15;
      while (0x11 <= v7) {
        if (v7 == 99)
          return 1;
        v11 = (unsigned int)v7 - 0x10;
        v16 >>= v15[1] & 0x3f;
        for (v13 = v13 - v15[1]; v13 < v11; v13 = v13 + 8) {
          v10 = (unsigned long)dat_1b008;
          if (dat_1b00c <= dat_1b008) { // branch-flip
            dat_1b004 = v12;
            v7 = sub_d830(0);
          }
          else {
            dat_1b008 += 1;
            v7 = *(unsigned char *)(v10 + 0x9d000);
          }
          v16 |= (unsigned long)v7 << ((unsigned char)v13 & 0x3f);
        }
        v15 = (unsigned char *)((unsigned long)((unsigned int)*(unsigned short *)((unsigned long)v11 * 2 + 0x13380) & (unsigned int)v16) * 0x10 + *(long *)&v15[8]);
        v7 = *v15;
      }
      v13 -= v15[1];
      v16 >>= v15[1] & 0x3f;
      v11 = (unsigned int)v7;
      if (v11 != 0x10) break;
      v10 = (unsigned long)v12;
      v12 += 1;
      *(char *)(v10 + 0x3c000) = (char)*(unsigned short *)&v15[8];
      if (v12 == 0x8000) {
        dat_1b004 = 0x8000;
        sub_d7e0();
        v12 = 0;
      }
    }
    if (v11 == 0xf) {
      dat_1b004 = v12;
      dat_dda0c = v13;
      dat_dda10 = v16;
      return 0;
    }
    for (; v13 < v11; v13 = v13 + 8) {
      v10 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        dat_1b004 = v12;
        v6 = sub_d830(0);
      }
      else {
        dat_1b008 += 1;
        v6 = *(unsigned char *)(v10 + 0x9d000);
      }
      v16 |= (unsigned long)v6 << ((unsigned char)v13 & 0x3f);
    }
    v4 = *(unsigned short *)&v15[8];
    v10 = v16 >> (v7 & 0x3f);
    v5 = *(unsigned short *)((unsigned long)v7 * 2 + 0x13380);
    for (v13 = v13 - v11; v13 < a3; v13 = v13 + 8) {
      v9 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        dat_1b004 = v12;
        v7 = sub_d830(0);
      }
      else {
        dat_1b008 += 1;
        v7 = *(unsigned char *)(v9 + 0x9d000);
      }
      v10 |= (unsigned long)v7 << ((unsigned char)v13 & 0x3f);
    }
    v15 = (unsigned char *)((unsigned long)((unsigned int)v3 & (unsigned int)v10) * 0x10 + a1);
    v11 = (unsigned int)*v15;
    if (0x11 <= *v15) {
      do {
        if (v11 == 99)
          return 1;
        v10 >>= v15[1] & 0x3f;
        for (v13 = v13 - v15[1]; v13 < v11 - 0x10; v13 = v13 + 8) {
          v9 = (unsigned long)dat_1b008;
          if (dat_1b00c <= dat_1b008) { // branch-flip
            dat_1b004 = v12;
            v7 = sub_d830(0);
          }
          else {
            dat_1b008 += 1;
            v7 = *(unsigned char *)(v9 + 0x9d000);
          }
          v10 |= (unsigned long)v7 << ((unsigned char)v13 & 0x3f);
        }
        v15 = (unsigned char *)((unsigned long)((unsigned int)*(unsigned short *)((unsigned long)(v11 - 0x10) * 2 + 0x13380) & (unsigned int)v10) * 0x10 + *(long *)&v15[8]);
        v11 = (unsigned int)*v15;
      } while (0x10 < v11);
    }
    v13 -= v15[1];
    v10 >>= v15[1] & 0x3f;
    if (v13 < v11) {
      do {
        while( true ) {
          v8 = v13;
          v9 = (unsigned long)dat_1b008;
          if (dat_1b008 < dat_1b00c) break;
          dat_1b004 = v12;
          v10 |= (unsigned long)(unsigned char)sub_d830(0) << ((unsigned char)v8 & 0x3f);
          v13 = v8 + 8;
          if (v11 <= v8 + 8) goto label_8c1d;
        }
        dat_1b008 += 1;
        v10 |= (unsigned long)*(unsigned char *)(v9 + 0x9d000) << ((unsigned char)v8 & 0x3f);
        v13 = v8 + 8;
      } while (v8 + 8 < v11);
label_8c1d:
      v13 = v8 + 8;
    }
    v13 -= v11;
    v9 = (unsigned long)(((unsigned int)v5 & (unsigned int)v16) + (unsigned int)v4);
    v16 = v10 >> ((unsigned char)v11 & 0x3f);
    v11 = (v12 - *(unsigned short *)&v15[8]) - ((unsigned int)*(unsigned short *)((unsigned long)v11 * 2 + 0x13380) & (unsigned int)v10);
    do {
      while( true ) {
        v11 &= 0x7fff;
        v8 = v12;
        if (v12 < v11)
          v8 = v11;
        v10 = (unsigned long)(0x8000 - v8);
        if ((unsigned int)v9 <= 0x8000 - v8)
          v10 = v9;
        v17 = (unsigned int)v10;
        v14 = (unsigned int)v9 - v17;
        v9 = (unsigned long)v14;
        v8 = v11 - v12;
        if (v11 < v12)
          v8 = v12 - v11;
        if (v17 > v8) break;
        memcpy((void *)((unsigned long)v12 + 0x3c000),(void *)((unsigned long)v11 + 0x3c000),v10);
        if (v12 + v17 != 0x8000) goto label_8ca2;
label_8d17:
        v11 += v17;
        dat_1b004 = 0x8000;
        v12 = 0;
        sub_d7e0();
        if (!v14) goto label_88a5;
      }
      v8 = v11;
      do {
        v10 = (unsigned long)v8;
        v1 = v8 + (v12 - v11);
        v8 += 1;
        *(char *)((unsigned long)v1 + 0x3c000) = *(char *)(v10 + 0x3c000);
      } while (v8 != v11 + v17);
      if (v12 + v17 == 0x8000) goto label_8d17;
label_8ca2:
      v11 += v17;
      v12 += v17;
    } while (v14);
  } while( true );
}

// Function: sub_8d90 @ 0x8d90
void sub_8d90(void *a0)
{
  void *v1;
  
  if (!a0)
    return;
  do {
    v1 = (void *)((long)a0 + -0x10);
    a0 = *(void **)((long)a0 + -8);
    free(v1);
  } while (a0);
}

// Function: sub_8dc0 @ 0x8dc0
bool sub_8dc0(unsigned int *a0,unsigned int a1,unsigned int a2,long a3,long a4,long *a5,unsigned int *a6)
{
  unsigned int *v1;
  unsigned int *v10;
  unsigned int v11;
  unsigned int v12; // edx
  int v13;
  unsigned int v14;
  char *v15;
  long v16; // rdx
  char v17 [8]; // stack - 0x5e8
  int v18 [4]; // stack - 0x5a8
  unsigned int v19 [18]; // stack - 0x598
  unsigned char v2;
  int v20 [2];
  unsigned int v21 [290];
  long v22 [17]; // stack - 0x550
  int v23;
  char v24;
  unsigned long *v25; // rdx
  unsigned int v26;
  unsigned int v27;
  unsigned long v28; // rax
  long v29;
  long v3;
  unsigned int v30;
  unsigned char v31; // stack - 0x648
  unsigned int v32; // stack - 0x644
  unsigned int v33; // stack - 0x640
  int v34; // stack - 0x63c
  unsigned int v35; // stack - 0x638
  long v36; // stack - 0x630
  unsigned int *v37; // stack - 0x628
  unsigned int *v38; // stack - 0x620
  unsigned int v39; // stack - 0x614
  int *v4;
  int v40; // stack - 0x5f0
  char v41 [16]; // stack - 0x5d8
  char v42 [16]; // stack - 0x5c8
  char v43 [16]; // stack - 0x5b8
  int *v5; // rax
  unsigned long v6; // rax
  void *v7; // rax
  unsigned int v8;
  unsigned int *v9;
  
  v17._0_16_ = 0;
  v18[0] = 0;
  v41._0_16_ = 0;
  v42._0_16_ = 0;
  v43._0_16_ = 0;
  v9 = a0;
  do {
    v11 = *v9;
    v9 = &v9[1];
    *(int *)&v17[(unsigned long)v11 * 4] = *(int *)&v17[(unsigned long)v11 * 4] + 1;
  } while (&a0[a1] != v9);
  v17._0_4_ = 0;
  if (!a1) {
    v7 = malloc(0x30);
    if (v7) {
      *(unsigned long *)((long)v7 + 8) = 0;
      *(unsigned short *)((long)v7 + 0x10) = 0x163;
      *(unsigned short *)((long)v7 + 0x20) = 0x163;
      dat_dda08 += 3;
      *a5 = (long)v7 + 0x10;
      *a6 = 1;
      return 0;
    }
    return (bool)3;
  }
  v3 = 1;
  do {
    v11 = (unsigned int)v3;
    if (*(int *)&v17[v3 * 4]) {
      v23 = 1 << ((unsigned char)v3 & 0x1f);
      v33 = v11;
      goto label_8e9d;
    }
    v3 += 1;
  } while (v3 != 0x11);
  v33 = 0x11;
  v23 = 0x20000;
  v11 = 0x11;
label_8e9d:
  v30 = *a6;
  if (*a6 < v11)
    v30 = v33;
  v4 = v18;
  v26 = 0x10;
label_8eb4:
  if (!*v4) goto label_8ebf;
  if (v26 < v30)
    v30 = v26;
  *a6 = v30;
  v39 = v26;
  if (v11 < v26) {
    v6 = (unsigned long)v11;
    v23 -= *(int *)&v17[v6 * 4];
    if (v23 < 0)
      return (bool)2;
    v15 = &v17[v6 * 4];
    while (v23 = v23 * 2, &v17[(((v26 - v11) - 1) + v6) * 4] != v15) {
      v23 -= *(int *)&v15[4];
      v15 = &v15[4];
      if (v23 < 0)
        return (bool)2;
    }
    v40 = *(int *)&v17[(unsigned long)v26 * 4];
    goto label_8faa;
  }
  v40 = v23 - *(int *)&v17[(unsigned long)v26 * 4];
  if (v40 < 0)
    return (bool)2;
  *(int *)&v17[(unsigned long)v26 * 4] = v23;
  v19[1] = 0;
  if (v26 != 1) goto label_8fcd;
  goto label_8ffb;
label_8ebf:
  v4 = &v4[-1];
  v26 -= 1;
  if (!v26) {
    if (v30) { // branch-flip
      *a6 = 0;
      v30 = 0;
      v39 = 0;
      v40 = v17._0_4_;
label_8faa:
      v40 = v23 - v40;
      if (v40 < 0)
        return (bool)2;
      v19[1] = 0;
      *(int *)&v17[(unsigned long)v26 * 4] = v23;
    }
    else {
      *a6 = 0;
      if (v23 < 0)
        return (bool)2;
      v17._4_12_ = SUB1612(0,4);
      v17._0_4_ = v23;
      v19[1] = 0;
      v39 = 0;
      v40 = v23;
    }
label_8fcd:
    v11 = 0;
    v4 = (int *)&v17[4];
    v9 = &v19[2];
    do {
      v5 = &v4[1];
      v10 = &v9[1];
      v11 += *v4;
      *v9 = v11;
      v4 = v5;
      v9 = v10;
    } while (&v20[v26 - 2] != v5);
label_8ffb:
    v19[1] = 0;
    v6 = 0;
    do {
      v11 = a0[v6];
      if (v11) {
        v26 = v19[v11];
        v21[v26] = (unsigned int)v6;
        v19[v11] = v26 + 1;
      }
      v6 += 1;
    } while (a1 != v6);
    v22[1] = 0;
    v23 = -v30;
    v19[0] = 0;
    if ((int)v33 <= (int)v39) {
      v38 = v21;
      v26 = 0;
      v9 = &v38[v19[(int)v39]];
      v11 = 0;
      v34 = -1;
      v37 = (unsigned int *)&v17[(long)(int)v33 * 4];
      v3 = 0;
      do {
        v32 = *v37;
        if (v32) {
          v12 = 1 << ((char)v33 - 1U & 0x1fU);
          do {
            v13 = v30 + v23;
            if ((int)v33 <= v13) // branch-flip
              v35 = v33 - v23;
            else {
              v27 = v39 - v13;
              v29 = (long)(v34 + 1);
              v14 = v33 - v13;
              do {
                v35 = v14;
                v23 = v13;
                v31 = (char)v23 - (char)v30;
                v34 = (int)v29;
                v26 = 1 << ((unsigned char)v35 & 0x1f);
                v14 = v35;
                if (v32 < v26) {
                  v8 = v30;
                  if (v27 <= v30)
                    v8 = v27;
                  if (v35 < v8) {
                    v13 = v26 - v32;
                    v10 = v37;
                    while (v14 = v14 + 1, v14 < v8) {
                      v1 = &v10[1];
                      v10 = &v10[1];
                      if ((unsigned int)(v13 * 2) <= *v1) break;
                      v13 = v13 * 2 - *v1;
                    }
                    v26 = 1 << ((unsigned char)v14 & 0x1f);
                  }
                }
                v7 = malloc((unsigned long)(v26 + 1) << 4);
                if (!v7) {
                  if (v34) {
                    sub_8d90(v22[1]);
                    return (bool)3;
                  }
                  return (bool)3;
                }
                dat_dda08 = v26 + 1 + dat_dda08;
                v3 = (long)v7 + 0x10;
                *a5 = v3;
                a5 = (long *)((long)v7 + 8);
                *(unsigned long *)((long)v7 + 8) = 0;
                v22[v29 + 1] = v3;
                if (v34) {
                  v19[v29] = v11;
                  v25 = (unsigned long *)(v22[v29] + (unsigned long)(v11 >> (v31 & 0x1f)) * 0x10);
                  v28 = CONCAT62((undefined6)((unsigned long)v28 >> 0x10),CONCAT11((char)v30,(char)v14 + '\x10'));
                  v25[1] = v3;
                  *v25 = v28;
                  v36 = v3;
                }
                v29 += 1;
                v27 -= v30;
                v13 = v23 + v30;
                v14 = v35 - v30;
              } while ((int)(v23 + v30) < (int)v33);
            }
            v24 = 'c';
            v2 = (unsigned char)v23;
            if (v38 < v9) {
              v14 = *v38;
              v38 = &v38[1];
              dat_a = (undefined6)((unsigned long)v36 >> 0x10);
              if (a2 <= v14) { // branch-flip
                v36 = CONCAT62(dat_a,*(unsigned short *)(a3 + (unsigned long)(v14 - a2) * 2));
                v24 = *(char *)(a4 + (unsigned long)(v14 - a2) * 2);
              }
              else {
                v36 = CONCAT62(dat_a,(short)v14);
                v24 = (v14 != 0x100) + '\x0f';
              }
            }
            v14 = v11 >> (v2 & 0x1f);
            if (v14 < v26) {
              do {
                v6 = (unsigned long)v14;
                v14 += 1 << ((unsigned char)v35 & 0x1f);
                v28 = CONCAT62((undefined6)((unsigned long)v28 >> 0x10),CONCAT11((char)v33 - v2,v24));
                v25 = (unsigned long *)(v6 * 0x10 + v3);
                *v25 = v28;
                v25[1] = v36;
              } while (v14 < v26);
            }
            v14 = v12 & v11;
            v27 = v12;
            while (v14) {
              v11 ^= v27;
              v27 >>= 1;
              v14 = v11 & v27;
            }
            v11 ^= v27;
            if (((1 << (v2 & 0x1f)) - 1U & v11) != v19[v34]) {
              v29 = (long)(v34 + -1);
              do {
                v16 = v29;
                v23 -= v30;
                v29 = v16 + -1;
              } while (((1 << ((unsigned char)v23 & 0x1f)) - 1U & v11) != v19[v16]);
              v34 = (int)v16;
            }
            v32 -= 1;
          } while (v32);
        }
        v33 += 1;
        v37 = &v37[1];
      } while (v33 != v39 + 1);
    }
    return v39 != 1 && v40;
  }
  goto label_8eb4;
}

// Function: sub_94b0 @ 0x94b0
unsigned int sub_94b0(void)
{
  unsigned short v1;
  unsigned long v10;
  unsigned int v11 [11];
  long v12; // stack - 0x548
  unsigned int v13; // stack - 0x550
  unsigned int v14; // stack - 0x54c
  char *v15; // stack - 0x540
  unsigned int v16 [307];
  unsigned int v17;
  unsigned int *v18;
  unsigned int v19;
  unsigned short v2;
  unsigned int v20;
  unsigned int v21;
  unsigned int v22;
  unsigned int v23; // stack - 0x588
  unsigned int v24; // stack - 0x57c
  unsigned int v3;
  unsigned char v4;
  unsigned char v5;
  unsigned int v6; // eax
  int v7; // eax
  unsigned int v8;
  unsigned long v9;
  
  v9 = dat_dda10;
  v21 = dat_dda0c;
  v3 = dat_1b004;
  v19 = dat_dda0c;
  if (dat_dda0c <= 4) {
    v10 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) // branch-flip
      v4 = sub_d830(0);
    else {
      dat_1b008 += 1;
      v4 = *(unsigned char *)(v10 + 0x9d000);
    }
    v19 = v21 + 8;
    v9 |= (unsigned long)v4 << ((unsigned char)v21 & 0x3f);
  }
  v21 = v19 - 5;
  v10 = v9 >> 5;
  v6 = ((unsigned int)v9 & 0x1f) + 0x101;
  v8 = v21;
  if (v21 <= 4) {
    v9 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) { // branch-flip
      dat_1b004 = v3;
      v4 = sub_d830(0);
    }
    else {
      dat_1b008 += 1;
      v4 = *(unsigned char *)(v9 + 0x9d000);
    }
    v8 = v19 + 3;
    v10 |= (unsigned long)v4 << ((unsigned char)v21 & 0x3f);
  }
  v20 = v8 - 5;
  v9 = v10 >> 5;
  v19 = ((unsigned int)v10 & 0x1f) + 1;
  v21 = v20;
  if (v20 <= 3) {
    v10 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) { // branch-flip
      dat_1b004 = v3;
      v4 = sub_d830(0);
    }
    else {
      dat_1b008 += 1;
      v4 = *(unsigned char *)(v10 + 0x9d000);
    }
    v21 = v8 + 3;
    v9 |= (unsigned long)v4 << ((unsigned char)v20 & 0x3f);
  }
  if ((0x11f <= v6) || (0x1f <= v19))
    return 1;
  v21 -= 4;
  v18 = (unsigned int *)0x133c0;
  v20 = (unsigned int)v9 & 0xf;
  v8 = v20 + 4;
  v9 >>= 4;
  do {
    v22 = v21;
    if (v21 <= 2) {
      v10 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        dat_1b004 = v3;
        v4 = sub_d830(0);
      }
      else {
        dat_1b008 += 1;
        v4 = *(unsigned char *)(v10 + 0x9d000);
      }
      v22 = v21 + 8;
      v9 |= (unsigned long)v4 << ((unsigned char)v21 & 0x3f);
    }
    v17 = *v18;
    v18 = &v18[1];
    v10 = v9 >> 3;
    v21 = v22 - 3;
    v11[v17] = (unsigned int)v9 & 7;
    v9 = v10;
  } while ((unsigned int *)((unsigned long)v20 * 4 + 0x133d0) != v18);
  for (; v8 != 0x13; v8 = v8 + 1) {
    v11[*(unsigned int *)((unsigned long)v8 * 4 + 0x133c0)] = 0;
  }
  v13 = 7;
  v20 = sub_8dc0(v11,0x13,0x13,0,0,&v12,&v13);
  v8 = v13;
  if (!v20) {
    if (!v12)
      return 2;
    v24 = 0;
    v22 = v6 + v19;
    v23 = 0;
    v1 = *(unsigned short *)((long)(int)v13 * 2 + 0x13380);
    v20 = 0;
    do {
      while (v21 < v8) {
        v9 = (unsigned long)dat_1b008;
        if (dat_1b00c <= dat_1b008) { // branch-flip
          dat_1b004 = v3;
          v4 = sub_d830(0);
        }
        else {
          dat_1b008 += 1;
          v4 = *(unsigned char *)(v9 + 0x9d000);
        }
        v10 |= (unsigned long)v4 << ((unsigned char)v21 & 0x3f);
        v21 += 8;
      }
      v15 = (char *)((unsigned long)((unsigned int)v1 & (unsigned int)v10) * 0x10 + v12);
      v9 = v10 >> (v15[1] & 0x3fU);
      v21 -= (unsigned char)v15[1];
      if (*v15 == 'c') {
        sub_8d90(v12);
        return 2;
      }
      v2 = *(unsigned short *)&v15[8];
      v17 = (unsigned int)v2; // branch-flip
      if (0x10 <= v2) {
        v4 = (unsigned char)v21;
        if (v2 != 0x10) { // branch-flip
          if (v17 != 0x11) { // branch-flip
            if (v21 <= 6) {
              v10 = (unsigned long)dat_1b008;
              if (dat_1b00c <= dat_1b008) { // branch-flip
                dat_1b004 = v3;
                v5 = sub_d830(0);
              }
              else {
                dat_1b008 += 1;
                v5 = *(unsigned char *)(v10 + 0x9d000);
              }
              v21 += 8;
              v9 |= (unsigned long)v5 << (v4 & 0x3f);
            }
            v10 = v9 >> 7;
            v21 -= 7;
            v17 = (unsigned int)v9 & 0x7f;
            if (v22 < v17 + 0xb + v23)
              return 1;
            v18 = &v11[(int)v20];
            do {
              *v18 = 0;
              v18 = &v18[1];
            } while (&v16[(unsigned long)v17 + (long)(int)v20] != v18);
            v24 = 0;
            v23 = v17 + 0xb + v20;
          }
          else {
            if (v21 <= 2) {
              v10 = (unsigned long)dat_1b008;
              if (dat_1b00c <= dat_1b008) { // branch-flip
                dat_1b004 = v3;
                v5 = sub_d830(0);
              }
              else {
                dat_1b008 += 1;
                v5 = *(unsigned char *)(v10 + 0x9d000);
              }
              v21 += 8;
              v9 |= (unsigned long)v5 << (v4 & 0x3f);
            }
            v10 = v9 >> 3;
            v21 -= 3;
            v17 = (unsigned int)v9 & 7;
            if (v22 < v17 + 3 + v23)
              return 1;
            v18 = &v11[(int)v20];
            do {
              *v18 = 0;
              v18 = &v18[1];
            } while (&v11[(unsigned long)v17 + (long)(int)v20 + 3] != v18);
            v24 = 0;
            v23 = v17 + 3 + v20;
          }
        }
        else {
          if (v21 <= 1) {
            v10 = (unsigned long)dat_1b008;
            if (dat_1b00c <= dat_1b008) { // branch-flip
              dat_1b004 = v3;
              v5 = sub_d830(0);
            }
            else {
              dat_1b008 += 1;
              v5 = *(unsigned char *)(v10 + 0x9d000);
            }
            v21 += 8;
            v9 |= (unsigned long)v5 << (v4 & 0x3f);
          }
          v10 = v9 >> 2;
          v21 -= 2;
          v17 = (unsigned int)v9 & 3;
          if (v22 < v17 + 3 + v23)
            return 1;
          v18 = &v11[(int)v20];
          do {
            *v18 = v24;
            v18 = &v18[1];
          } while (&v11[(unsigned long)v17 + (long)(int)v20 + 3] != v18);
          v23 = v17 + 3 + v20;
        }
      }
      else {
        v23 = v20 + 1;
        v11[(int)v20] = v17;
        v10 = v9;
        v24 = v17;
      }
      v20 = v23;
    } while (v23 < v22);
    sub_8d90(v12);
    v13 = 9;
    dat_dda0c = v21;
    dat_dda10 = v10;
    v20 = sub_8dc0(v11,v6,0x101,0x191a0,0x19160,&v12,&v13);
    if (!v20) {
      v14 = 6;
      v21 = sub_8dc0(&v11[v6],v19,0,0x19120,0x190e0,&v15,&v14);
      if (!v21) {
        v7 = sub_8840(v12,v15,v13,v14);
        sub_8d90(v12);
        sub_8d90(v15);
        return (unsigned int)(v7 != 0);
      }
      if (v21 == 1)
        sub_8d90(v15);
      sub_8d90(v12); // return-dupe
      return v21;
    }
  }
  v21 = 1;
  if (v20 != 1)
    return v20;
  sub_8d90(v12);
  return v21;
}

// Function: sub_9b70 @ 0x9b70
unsigned long sub_9b70(void) // early-return x2
{
  unsigned int v1; // eax
  unsigned long v10; // stack - 0x4b0
  unsigned int v11; // stack - 0x4bc
  unsigned long v12; // stack - 0x48
  unsigned long v13; // stack - 0x40
  unsigned long v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  int v2; // eax
  unsigned int *v3;
  unsigned long v4; // rax
  unsigned int v5 [24];
  unsigned int v6 [144];
  unsigned int v7 [112];
  unsigned int v8; // stack - 0x4c0
  unsigned long v9; // stack - 0x4b8
  
  v3 = v6;
  do {
    *v3 = 8;
    v3 = &v3[1];
  } while (v3 != v7);
  do {
    *v3 = 9;
    v3 = &v3[1];
  } while (v5 != v3);
  v3 = v5;
  do {
    *v3 = 7;
    v3 = &v3[1];
  } while (&v5[0x18] != v3);
  v8 = 7;
  v12 = 0x800000008;
  v13 = 0x800000008;
  v14 = 0x800000008;
  v15 = 0x800000008;
  v4 = sub_8dc0(v6,0x120,0x101,0x191a0,0x19160,&v9,&v8);
  if (!(int)v4) {
    v3 = v6;
    do {
      *v3 = 5;
      v3 = &v3[1];
    } while (v3 != &v6[0x1e]);
    v11 = 5;
    v1 = sub_8dc0(v6,0x1e,0,0x19120,0x190e0,&v10);
    if ((int)v1 <= 1) {
      v2 = sub_8840(v9,v10,v8,v11);
      if (v2)
        return 1;
      sub_8d90(v9);
      sub_8d90(v10);
      return 0;
    }
    sub_8d90(v9);
    v4 = (unsigned long)v1;
  }
  return v4;
}

// Function: sub_9d20 @ 0x9d20
unsigned long sub_9d20(unsigned int *a0)
{
  int v1;
  unsigned char v2;
  unsigned long v3;
  unsigned long v4; // rax
  unsigned int v5;
  unsigned long v6; // rax
  
  v3 = dat_dda10;
  v1 = dat_dda0c;
  if (dat_dda0c) { // branch-flip
    v5 = dat_dda0c - 1;
    v6 = dat_dda10 >> 1;
    *a0 = (unsigned int)dat_dda10 & 1;
    if (2 <= v5) // branch-flip
      dat_dda0c -= 3;
    else {
      v3 = (unsigned long)dat_1b008;
      if (dat_1b00c <= dat_1b008) // branch-flip
        v2 = sub_d830(0);
      else {
        dat_1b008 += 1;
        v2 = *(unsigned char *)(v3 + 0x9d000);
      }
      v6 |= (unsigned long)v2 << ((unsigned char)v5 & 0x3f);
      dat_dda0c = v1 + 5;
    }
  }
  else {
    v6 = (unsigned long)dat_1b008;
    if (dat_1b00c <= dat_1b008) // branch-flip
      v2 = sub_d830(0);
    else {
      dat_1b008 += 1;
      v2 = *(unsigned char *)(v6 + 0x9d000);
    }
    v6 = (v2 | v3) >> 1;
    *a0 = (unsigned int)(v2 | v3) & 1;
    dat_dda0c = 5;
  }
  dat_dda10 = v6 >> 2;
  v5 = (unsigned int)v6 & 3;
  if (v5 == 2) {
    v4 = sub_94b0(); // tail-call
    return v4;
  }
  if (!(v6 & 3)) {
    v4 = sub_8640(); // tail-call
    return v4;
  }
  if (v5 != 1)
    return 2;
  v4 = sub_9b70(); // tail-call
  return v4;
}

// Function: sub_9e80 @ 0x9e80
unsigned long sub_9e80(void)
{
  unsigned long v1; // rax
  unsigned int v2; // ecx
  int v3; // stack - 0x14
  
  dat_1b004 = 0;
  dat_dda0c = 0;
  dat_dda10 = 0;
  do {
    dat_dda08 = 0;
    v1 = sub_9d20(&v3);
    if ((int)v1)
      return v1;
  } while (!v3);
  if (8 <= dat_dda0c) {
    v2 = dat_dda0c - 8;
    dat_dda0c &= 7;
    dat_1b008 += ~(v2 >> 3);
  }
  sub_d7e0();
  return v1 & 0xffffffff;
}

// Function: sub_9f30 @ 0x9f30
void sub_9f30(void)
{
  unsigned short *v1; // rax
  unsigned short *v2;
  
  v2 = (unsigned short *)0xe0080;
  do {
    v1 = &v2[2];
    *v2 = 0;
    v2 = v1;
  } while (v1 != (unsigned short *)0xe04f8);
  v2 = (unsigned short *)0xdff80;
  do {
    v1 = &v2[2];
    *v2 = 0;
    v2 = v1;
  } while (v1 != (unsigned short *)0xdfff8);
  v2 = (unsigned short *)0xdf9e0;
  do {
    v1 = &v2[2];
    *v2 = 0;
    v2 = v1;
  } while (v1 != (unsigned short *)0xdfa2c);
  dat_dda38 = 0;
  dat_e0480 = 1;
  dat_dda40 = 0;
  dat_dda4c = 0;
  dat_dda50 = 0;
  dat_dda54 = 0;
  dat_dda49 = 0;
  dat_dda48 = 1;
}

// Function: sub_9fe0 @ 0x9fe0
void sub_9fe0(long a0,int a1)
{
  unsigned short v1;
  unsigned short v2;
  int v3;
  int v4;
  int v5; // eax
  int v6;
  int v7;
  unsigned short v8;
  long v9; // r8
  
  v9 = (long)a1;
  v3 = *(int *)(v9 * 4 + 0xdf0c0);
  if (a1 * 2 <= dat_df0a4) {
    v1 = *(unsigned short *)(a0 + (long)v3 * 4);
    v6 = a1 * 2;
    do {
      v7 = *(int *)((long)v6 * 4 + 0xdf0c0);
      v8 = *(unsigned short *)(a0 + (long)v7 * 4);
      if (v6 < dat_df0a4) {
        v5 = v6 + 1;
        v4 = *(int *)((long)v5 * 4 + 0xdf0c0);
        v2 = *(unsigned short *)(a0 + (long)v4 * 4);
        if (v8 <= v2) { // branch-flip
          if (v2 != v8) { // branch-flip
          }
          else if (*(unsigned char *)((long)v7 + 0xdee60) >= *(unsigned char *)((long)v4 + 0xdee60)) {
            v6 = v5;
            v7 = v4;
          }
        }
        else {
          v6 = v5;
          v8 = v2;
          v7 = v4;
        }
      }
      if ((v1 < v8) || ((v1 == v8 && (*(unsigned char *)((long)v3 + 0xdee60) <= *(unsigned char *)((long)v7 + 0xdee60))))) break;
      v4 = v6 * 2;
      *(int *)(v9 * 4 + 0xdf0c0) = v7;
      v9 = (long)v6;
      v6 = v4;
    } while (v4 <= dat_df0a4);
  }
  *(int *)(v9 * 4 + 0xdf0c0) = v3;
}

// Function: sub_a0c0 @ 0xa0c0
void sub_a0c0(struct_5 *a0) // return-dupe
{
  unsigned short *v1;
  unsigned short v10;
  int v11;
  int v12;
  unsigned short *v13;
  unsigned short *v14; // rax
  long v15; // rax
  long v16;
  int v17;
  int v18;
  long v19; // rsi
  short *v2;
  int v20;
  long v21;
  int v22;
  unsigned int v23;
  long v24;
  long v25;
  long v26; // stack - 0x40
  short v3;
  int v4;
  int v5;
  long v6;
  int v7;
  bool v8;
  bool v9;
  
  v24 = a0->field_0x10;
  v6 = a0->field_0x0;
  v4 = a0->field_0x24;
  v20 = a0->field_0x20;
  v16 = a0->field_0x8;
  v18 = a0->field_0x18;
  v13 = (unsigned short *)0xdf9c0;
  do {
    v14 = &v13[1];
    *v13 = 0;
    v13 = v14;
  } while (v14 != (unsigned short *)0xdf9e0);
  v15 = (long)dat_df0a0;
  *(unsigned short *)(v6 + 2 + (long)*(int *)(v15 * 4 + 0xdf0c0) * 4) = 0;
  if (0x23c <= dat_df0a0)
    return;
  v9 = 0;
  v22 = 0;
  v8 = 0;
  v21 = v15 * 4 + 0xdf0c0;
  v26 = dat_dda38;
  v25 = dat_dda40;
  do {
    v5 = *(int *)(v21 + 4);
    v1 = (unsigned short *)(v6 + (long)v5 * 4);
    v11 = *(unsigned short *)(v6 + 2 + (unsigned long)v1[1] * 4) + 1;
    if (v20 < v11) {
      v22 += 1;
      v11 = v20;
    }
    v1[1] = (unsigned short)v11;
    if (v5 <= v4) {
      v19 = (long)v11;
      v17 = 0;
      v2 = (short *)(v19 * 2 + 0xdf9c0);
      *v2 = *v2 + 1;
      if (v18 <= v5) {
        v17 = *(int *)(v24 + (long)(v5 - v18) * 4);
        v19 = (long)(int)(v11 + v17);
      }
      v25 += v19 * (unsigned long)*v1;
      v9 = 1;
      if (v16) {
        v8 = 1;
        v26 += (long)(int)((unsigned int)*(unsigned short *)(v16 + 2 + (long)v5 * 4) + v17) * (unsigned long)*v1;
      }
    }
    v21 += 4;
  } while (v21 != ((unsigned long)(unsigned int)(0x23b - dat_df0a0) + v15) * 4 + 0xdf0c4);
  if (v8)
    dat_dda38 = v26;
  if (v9)
    dat_dda40 = v25;
  if (!v22)
    return;
  v18 = v22 + -2;
  v11 = v20 - 1;
  v22 = v18 - (v22 - 1U & 0xfffffffeU);
  v3 = *(short *)((long)v11 * 2 + 0xdf9c0);
  v24 = (long)v20;
  if (v3) goto label_a2a8;
  do {
    v16 = v24 * 2 + 0xdf9c0;
    v23 = v11;
    do {
      v3 = *(short *)(v16 + -4);
      v16 -= 2;
      v12 = v23 - 1;
      v7 = v23;
      v23 = v12;
    } while (!v3);
    while( true ) {
      *(short *)((long)v12 * 2 + 0xdf9c0) = v3 + -1;
      v2 = (short *)((long)v7 * 2 + 0xdf9c0);
      *v2 = *v2 + 2;
      v10 = *(short *)(v24 * 2 + 0xdf9c0) - 1;
      *(unsigned short *)(v24 * 2 + 0xdf9c0) = v10;
      if (v18 == v22) {
        if (!v20)
          return;
        v8 = 0;
        v18 = 0x23d;
        v16 = dat_dda40;
        while( true ) {
          v23 = (unsigned int)v10;
          if (v23) {
            v15 = (long)(v18 + -1);
            do {
              while( true ) {
                v22 = *(int *)(v15 * 4 + 0xdf0c0);
                v18 = (int)v15;
                if (v22 <= v4) break;
                v15 -= 1;
              }
              v1 = (unsigned short *)(v6 + (long)v22 * 4);
              v10 = v1[1];
              if (v10 != v20) {
                v1[1] = (unsigned short)v20;
                v8 = 1;
                v16 += (unsigned long)*v1 * (v24 - (unsigned long)v10);
              }
              v15 -= 1;
              v23 -= 1;
            } while (v23);
          }
          v24 -= 1;
          if (!v11) break;
          v10 = *(unsigned short *)(v24 * 2 + 0xdf9c0);
          v20 = v11;
          v11 -= 1;
        }
        if (!v8)
          return;
        dat_dda40 = v16;
        return;
      }
      v3 = *(short *)((long)v11 * 2 + 0xdf9c0);
      v18 -= 2;
      if (!v3) break;
label_a2a8:
      v7 = v20;
      v12 = v11;
    }
  } while( true );
}

// Function: sub_a370 @ 0xa370
void sub_a370(void *a0,int a1) // return-dupe
{
  short *v1;
  int v10; // r9d
  unsigned short *v2;
  unsigned short v3;
  int v4; // eax
  unsigned short *v5;
  int v6;
  unsigned int v7; // edx
  unsigned int v8;
  int v9; // r8d
  
  v3 = *(unsigned short *)((long)a0 + 2);
  *(unsigned short *)((long)a0 + (long)a1 * 4 + 6) = 0xffff;
  v10 = 4 - (unsigned int)(v3 == 0);
  v9 = (-(unsigned int)(v3 == 0) & 0x83) + 7;
  if (a1 <= -1)
    return;
  v2 = (unsigned short *)((long)a0 + (long)a1 * 4 + 10);
  v5 = (unsigned short *)((long)a0 + 6);
  v4 = 0;
  v8 = 0xffffffff;
  v6 = (unsigned int)v3;
  do {
    while( true ) {
      v4 += 1;
      v3 = *v5;
      v7 = (unsigned int)v3;
      if (v9 <= v4) break;
      if (v7 != v6) {
        if (v10 <= v4) break;
        v1 = (short *)((long)v6 * 4 + 0xdf9e0);
        *v1 = *v1 + (short)v4;
        if (v3) goto label_a420;
        v10 = 3; // crossjump-dupe
        v9 = 0x8a;
        v4 = 0;
        v8 = v6;
      }
label_a3f5:
      v5 = &v5[2];
      v6 = v7;
      if (v2 == v5)
        return;
    }
    if (v6) { // branch-flip
      if (v6 != v8) {
        v1 = (short *)((long)v6 * 4 + 0xdf9e0);
        *v1 = *v1 + 1;
      }
      dat_dfa20 += 1;
    }
    else if (0xb <= v4) // branch-flip
      dat_dfa28 += 1;
    else {
      dat_dfa24 += 1;
    }
    if (!v7) {
      v10 = 3;
      v9 = 0x8a;
      v4 = 0;
      v8 = v6;
      goto label_a3f5;
    }
    if (v7 == v6) {
      v10 = 3;
      v9 = 6;
      v4 = 0;
      v8 = v7;
      goto label_a3f5;
    }
label_a420:
    v5 = &v5[2];
    v10 = 4;
    v4 = 0;
    v9 = 7;
    v8 = v6;
    v6 = v7;
    if (v2 == v5)
      return;
  } while( true );
}

// Function: sub_a480 @ 0xa480
void sub_a480(void)
{
  unsigned short v1;
  unsigned short *v2;
  unsigned int v3; // ecx
  unsigned int v4; // edx
  
  v4 = 0;
  v2 = (unsigned short *)0xe0080;
  do {
    v1 = *v2;
    v2 = &v2[2];
    v4 += v1;
  } while (v2 != (unsigned short *)0xe009c);
  v2 = (unsigned short *)0xe009c;
  v3 = 0;
  do {
    v1 = *v2;
    v2 = &v2[2];
    v3 += v1;
  } while (v2 != (unsigned short *)0xe0280);
  v2 = (unsigned short *)0xe0280;
  do {
    v1 = *v2;
    v2 = &v2[2];
    v4 += v1;
  } while (v2 != (unsigned short *)0xe0480);
  *dat_dda20 = (unsigned short)(v4 <= v3 >> 2);
}

// Function: sub_a500 @ 0xa500
void sub_a500(unsigned short *a0,int a1) // return-dupe
{
  unsigned short *v1;
  unsigned short v2;
  short v3;
  long v4; // rdx
  short v5 [20];
  
  v4 = 0;
  v3 = 0;
  do {
    v3 = (v3 + *(short *)(v4 + 0xdf9c0)) * 2;
    *(short *)((long)v5 + v4 + 2) = v3;
    v4 += 2;
  } while (v4 != 0x1e);
  if (a1 <= -1)
    return;
  v1 = &a0[(long)a1 * 2 + 2];
  do {
    v2 = a0[1];
    if (v2) {
      v3 = v5[(int)(unsigned int)v2];
      v5[(int)(unsigned int)v2] = v3 + 1;
      *a0 = sub_4380((unsigned short)v3,v2);
    }
    a0 = &a0[2];
  } while (a0 != v1);
}

// Function: sub_a5a0 @ 0xa5a0
void sub_a5a0(struct_6 *a0)
{
  unsigned char v1;
  unsigned long v10; // rcx
  long v11; // rcx
  unsigned int v12;
  long v13; // rdx
  long v14;
  long v15;
  int v16; // r14d
  unsigned short *v17;
  short v18; // stack - 0x50
  int v19; // stack - 0x48
  unsigned char v2;
  int v20; // stack - 0x44
  short v3;
  int v4;
  unsigned short *v5;
  bool v6;
  short v7;
  unsigned long v8; // rax
  long v9; // rax
  
  v4 = a0->field_0x1c;
  v5 = a0->field_0x0;
  v14 = a0->field_0x8;
  dat_df0a0 = 0x23d;
  if (1 <= v4) { // branch-flip
    v6 = 0;
    v8 = 0;
    dat_df0a4 = 0;
    v10 = 0xffffffff;
    do {
      while (v19 = (int)v10, !v5[v8 * 2]) {
        v5[v8 * 2 + 1] = 0;
        v8 += 1;
        if (v8 == (long)v4) goto label_a637;
      }
      dat_df0a4 += 1;
      *(char *)(v8 + 0xdee60) = 0;
      v19 = (int)v8;
      v10 = v8 & 0xffffffff;
      *(int *)((long)dat_df0a4 * 4 + 0xdf0c0) = v19;
      v8 += 1;
      v6 = 1;
    } while (v8 != (long)v4);
label_a637:
    if (v6) {
      if (1 < dat_df0a4) {
        v20 = dat_df0a4;
        goto label_a715;
      }
    }
    else {
      dat_df0a4 = 0;
    }
  }
  else {
    v19 = -1;
    dat_df0a4 = 0;
  }
  v13 = (long)dat_df0a4;
  v6 = 0;
  v15 = dat_dda38;
  do {
    if (2 <= v19) { // branch-flip
      v11 = 0;
      v9 = 0;
      v17 = v5;
    }
    else {
      v19 += 1;
      v9 = (long)v19;
      v11 = (long)v19 * 4;
      v17 = &v5[(long)v19 * 2];
    }
    *(int *)(v13 * 4 + 0xdf0c4) = (int)v9;
    *v17 = 1;
    *(char *)(v9 + 0xdee60) = 0;
    if (v14) {
      v6 = 1;
      v15 -= (unsigned long)*(unsigned short *)(v14 + 2 + v11);
    }
    v13 += 1;
  } while ((int)v13 <= 1);
  v12 = 0;
  if (dat_df0a4 <= 1)
    v12 = 1U - dat_df0a4;
  v20 = dat_df0a4 + 1 + v12;
  v14 = -(unsigned long)(unsigned int)(1U - dat_df0a4);
  if (2 <= dat_df0a4)
    v14 = 0;
  dat_dda40 = dat_dda40 + -1 + v14;
  dat_df0a4 = v20;
  if (v6)
    dat_dda38 = v15;
label_a715:
  v14 = (long)v4;
  a0->field_0x24 = v19;
  v16 = v20 >> 1;
  do {
    sub_9fe0(v5,v16);
    v16 -= 1;
  } while (v16);
  v13 = (long)v20;
  v18 = (short)v4;
  v15 = 0x23b;
  do {
    v4 = dat_df0c4;
    dat_df0a4 = (int)v13 + -1;
    dat_df0c4 = *(int *)(v13 * 4 + 0xdf0c0);
    sub_9fe0(v5,1);
    dat_df0a0 = (unsigned int)v15;
    v7 = v5[(long)dat_df0c4 * 2];
    v3 = v5[(long)v4 * 2];
    *(int *)(v15 * 4 + 0xdf0c4) = v4;
    *(int *)(v15 * 4 + 0xdf0c0) = dat_df0c4;
    v5[v14 * 2] = v7 + v3;
    v1 = *(unsigned char *)((long)v4 + 0xdee60);
    v2 = *(unsigned char *)((long)dat_df0c4 + 0xdee60);
    if (v2 <= v1)
      v2 = v1;
    v15 -= 2;
    *(unsigned char *)(v14 + 0xdee60) = v2 + 1;
    v7 = (v18 + (short)v20) - (short)v13;
    v13 -= 1;
    (&v5[(long)dat_df0c4 * 2])[1] = v7;
    (&v5[(long)v4 * 2])[1] = v7;
    dat_df0c4 = (int)v14;
    v14 += 1;
    sub_9fe0(v5,1);
  } while (1 < (int)v13);
  dat_df0a0 = v20 * -2 + 0x241;
  if (v20 <= 1)
    dat_df0a0 = 0x23d;
  dat_df0a0 -= 3;
  *(int *)((long)dat_df0a0 * 4 + 0xdf0c0) = dat_df0c4;
  sub_a0c0(a0);
  sub_a500(v5,v19); // tail-call
}

// Function: sub_a8c0 @ 0xa8c0
unsigned long sub_a8c0(void)
{
  unsigned long v1;
  unsigned long v2; // rdx
  
  sub_a370((void *)0xe0080,dat_193e4);
  sub_a370((void *)0xdff80,dat_193a4);
  sub_a5a0((struct_6 *)0x19340);
  v2 = 0xf;
  v1 = 0x12;
  while( true ) {
    if (*(short *)(v2 * 4 + 0xdf9e2)) {
      dat_dda40 += (int)v1 * 3 + 0x11;
      return v1 & 0xffffffff;
    }
    if (v1 - 1 == 2) break;
    v2 = (unsigned long)*(unsigned char *)(v1 + 0x1340f);
    v1 -= 1;
  }
  dat_dda40 += 0x17;
  return 2;
}

// Function: sub_a960 @ 0xa960
void sub_a960(void *a0,int a1) // return-dupe
{
  unsigned short v1;
  int v2; // ecx
  int v3;
  int v4;
  unsigned int v5; // ebp
  int v6; // esi
  unsigned int v7;
  long v8; // r12
  int v9;
  
  v1 = *(unsigned short *)((long)a0 + 2);
  v6 = 4 - (unsigned int)(v1 == 0);
  v2 = (-(unsigned int)(v1 == 0) & 0x83) + 7;
  if (a1 <= -1)
    return;
  v8 = 0;
  v3 = 0;
  v7 = 0xffffffff;
  v4 = (unsigned int)v1;
  do {
    v9 = v3 + 1;
    v1 = *(unsigned short *)((long)a0 + v8 * 4 + 6);
    v5 = (unsigned int)v1;
    if (v2 <= v9) { // branch-flip
label_aa30:
      if (v4) { // branch-flip
        if (v7 != v4) {
          sub_4290(*(unsigned short *)((long)v4 * 4 + 0xdf9e0),*(unsigned short *)((long)v4 * 4 + 0xdf9e2));
          v9 = v3;
        }
        sub_4290(dat_dfa20,dat_dfa22);
        sub_4290(v9 + -3,2);
      }
      else if (0xb <= v9) { // branch-flip
        sub_4290(dat_dfa28,dat_dfa2a);
        sub_4290(v3 + -10,7);
      }
      else {
        sub_4290(dat_dfa24,dat_dfa26);
        sub_4290(v3 + -2,3);
      }
      if (v1) { // branch-flip
        if (v5 != v4) { // branch-flip
label_a9fe:
          v6 = 4;
          v2 = 7;
          v3 = 0;
          v7 = v4;
        }
        else {
          v6 = 3;
          v2 = 6;
          v3 = 0;
          v7 = v5;
        }
      }
      else {
label_aa98:
        v6 = 3;
        v2 = 0x8a;
        v3 = 0;
        v7 = v4;
      }
    }
    else {
      if (v5 != v4) {
        if (v6 <= v9) goto label_aa30;
        do {
          sub_4290(*(unsigned short *)((long)v4 * 4 + 0xdf9e0),*(unsigned short *)((long)v4 * 4 + 0xdf9e2));
          v9 -= 1;
        } while (v9);
        if (v1) goto label_a9fe;
        goto label_aa98;
      }
      v3 = v9;
    }
    v8 += 1;
    v4 = v5;
    if (a1 < (int)v8)
      return;
  } while( true );
}

// Function: sub_ab20 @ 0xab20
void sub_ab20(int a0,int a1,int a2)
{
  unsigned long v1; // rax
  unsigned char *v2; // r14
  
  sub_4290(a0 + -0x101,5);
  sub_4290(a1 + -1,5);
  sub_4290(a2 + -4,4);
  if (1 <= a2) {
    v2 = (unsigned char *)0x13411;
    v1 = 0x10;
    while( true ) {
      sub_4290(*(unsigned short *)(v1 * 4 + 0xdf9e2),3);
      if (v2 == (unsigned char *)((unsigned long)(unsigned int)(a2 - 1) + 0x13411)) break;
      v1 = (unsigned long)*v2;
      v2 = &v2[1];
    }
  }
  sub_a960((void *)0xe0080,a0 + -1);
  sub_a960((void *)0xdff80,a1 + -1); // tail-call
}

// Function: sub_abd0 @ 0xabd0
void sub_abd0(void *a0,long a1) // ternary
{
  unsigned short *v1;
  unsigned long v10;
  unsigned long v11;
  unsigned char v2;
  unsigned char v3;
  unsigned short v4;
  int v5;
  unsigned long v6;
  unsigned char v7;
  unsigned int v8; // ebp
  unsigned long v9;
  
  if (dat_dda54) {
    v7 = 0;
    v9 = 0;
    v6 = 0;
    v10 = 0;
    while( true ) {
      v11 = v10;
      if (!(v6 & 7)) {
        v11 = (unsigned long)((int)v10 + 1);
        v7 = *(unsigned char *)(v10 + 0xdda60);
      }
      v8 = (int)v6 + 1;
      v2 = *(unsigned char *)(v6 + 0x9d000);
      if (v7 & 1) { // branch-flip
        v3 = *(unsigned char *)((unsigned long)v2 + 0xded60);
        v1 = (unsigned short *)((long)a0 + (unsigned long)(v3 + 0x101) * 4);
        sub_4290(*v1,v1[1]);
        v5 = *(int *)((unsigned long)v3 * 4 + 0x192c0);
        if (v5)
          sub_4290((unsigned int)v2 - *(int *)((unsigned long)v3 * 4 + 0xdeae0),v5);
        v4 = *(unsigned short *)(v9 * 2 + 0x4c000);
        v2 = (0x100 <= v4) ? *(unsigned char *)((unsigned long)((v4 >> 7) + 0x100) + 0xdeb60) : *(unsigned char *)((unsigned long)(unsigned int)v4 + 0xdeb60); // branch-flip
        v6 = (unsigned long)v2;
        v1 = (unsigned short *)(a1 + v6 * 4);
        sub_4290(*v1,v1[1]);
        v9 = (unsigned long)((int)v9 + 1);
        v5 = *(int *)(v6 * 4 + 0x19240);
        if (v5)
          sub_4290((unsigned int)v4 - *(int *)(v6 * 4 + 0xdea60),v5);
      }
      else {
        v1 = (unsigned short *)((long)a0 + (unsigned long)v2 * 4);
        sub_4290(*v1,v1[1]);
      }
      if (dat_dda54 <= v8) break;
      v7 >>= 1;
      v6 = (unsigned long)v8;
      v10 = v11;
    }
  }
  sub_4290(*(unsigned short *)((long)a0 + 0x400),*(unsigned short *)((long)a0 + 0x402)); // tail-call
}

// Function: sub_ad80 @ 0xad80
void sub_ad80(unsigned long a0)
{
  char *v1;
  long v2;
  unsigned short *v3; // rax
  unsigned short *v4;
  unsigned long v5; // rbx
  long v6;
  int v7; // edi
  int v8; // edi
  
  dat_dda20 = a0;
  dat_dda28 = 0;
  dat_dda30 = 0;
  if (!dat_dfa82) {
    v6 = 0;
    v7 = 0;
    do {
      *(int *)(v6 * 4 + 0xdeae0) = v7;
      v8 = 1 << ((unsigned char)*(unsigned int *)(v6 * 4 + 0x192c0) & 0x1f);
      if (1 <= v8) {
        v1 = (char *)((long)v7 + 0xded60);
        do {
          *v1 = (char)v6;
          v1 = &v1[1];
        } while (v1 != (char *)((long)v7 + 0xded61 + (unsigned long)(unsigned int)(v8 - 1)));
        v7 += v8;
      }
      v6 += 1;
    } while (v6 != 0x1c);
    v6 = 0;
    v8 = 0;
    *(char *)((long)(v7 + -1) + 0xded60) = 0x1c;
    do {
      *(int *)(v6 * 4 + 0xdea60) = v8;
      v7 = 1 << ((unsigned char)*(unsigned int *)(v6 * 4 + 0x19240) & 0x1f);
      if (1 <= v7) {
        v1 = (char *)((long)v8 + 0xdeb60);
        do {
          *v1 = (char)v6;
          v1 = &v1[1];
        } while (v1 != (char *)((long)v8 + 0xdeb61 + (unsigned long)(unsigned int)(v7 - 1)));
        v8 += v7;
      }
      v6 += 1;
    } while (v6 != 0x10);
    v8 >>= 7;
    v6 = 0x10;
    do {
      *(int *)(v6 * 4 + 0xdea60) = v8 << 7;
      v7 = 1 << ((char)*(unsigned int *)(v6 * 4 + 0x19240) - 7U & 0x1fU);
      if (1 <= v7) {
        v2 = (long)v8 + 0xdeb60;
        do {
          *(char *)(v2 + 0x100) = (char)v6;
          v2 += 1;
        } while (v2 != (long)v8 + 0xdeb61 + (unsigned long)(unsigned int)(v7 - 1));
        v8 += v7;
      }
      v6 += 1;
    } while (v6 != 0x1e);
    v4 = (unsigned short *)0xdf9c0;
    do {
      v3 = &v4[1];
      *v4 = 0;
      v4 = v3;
    } while (v3 != (unsigned short *)0xdf9e0);
    v4 = (unsigned short *)0xdfb02;
    do {
      v3 = &v4[2];
      *v4 = 8;
      v4 = v3;
    } while (v3 != (unsigned short *)0xdfd42);
    v4 = (unsigned short *)0xdfd42;
    do {
      v3 = &v4[2];
      *v4 = 9;
      v4 = v3;
    } while (v3 != (unsigned short *)0xdff02);
    dat_df9d2 += 0x70;
    v4 = (unsigned short *)0xdff02;
    do {
      v3 = &v4[2];
      *v4 = 7;
      v4 = v3;
    } while (v3 != (unsigned short *)0xdff62);
    dat_df9ce += 0x18;
    dat_dff66 = 8;
    dat_df9d0 += 0x98;
    dat_dff6a = 8;
    dat_dff7e = 8;
    v5 = 0;
    dat_dff62 = 8;
    dat_dff6e = 8;
    dat_dff72 = 8;
    dat_dff76 = 8;
    dat_dff7a = 8;
    sub_a500((unsigned short *)0xdfb00,0x11f);
    do {
      *(unsigned short *)(v5 * 4 + 0xdfa82) = 5;
      *(unsigned short *)(v5 * 4 + 0xdfa80) = sub_4380(v5 & 0xffffffff,5);
      v5 += 1;
    } while (v5 != 0x1e);
    sub_9f30(); // tail-call
    return;
  }
}

// Function: sub_b070 @ 0xb070
long sub_b070(char *a0,unsigned long a1,int a2,int a3)
{
  int v1; // eax
  unsigned long v2;
  unsigned long v3; // rcx
  unsigned long v4; // rdi
  
  *(char *)((unsigned long)dat_dda4c + 0xdda60) = dat_dda49;
  if (*dat_dda20 == -1)
    sub_a480();
  sub_a5a0((struct_6 *)0x193c0);
  sub_a5a0((struct_6 *)0x19380);
  v1 = sub_a8c0();
  dat_dda28 += a1;
  v4 = dat_dda40 + 10U >> 3;
  v3 = dat_dda38 + 10U >> 3;
  v2 = v3;
  if (v4 <= v3)
    v2 = v4;
  if ((a1 + 4 <= v2) && (a0)) {
    sub_4290(a3,3);
    dat_dda30 = (dat_dda30 + 10 & 0xfffffffffffffff8) + (a1 + 4) * 8;
    sub_44a0(a0,a1 & 0xffffffff,1);
    sub_9f30();
  }
  else {
    if (v3 <= v4) { // branch-flip
      sub_4290(a3 + 2,3);
      sub_abd0((void *)0xdfb00,0xdfa80);
      dat_dda30 += dat_dda38 + 3;
    }
    else {
      sub_4290(a3 + 4,3);
      sub_ab20(dat_193e4 + 1,dat_193a4 + 1,v1 + 1);
      sub_abd0((void *)0xe0080,0xdff80);
      dat_dda30 += dat_dda40 + 3;
    }
    sub_9f30();
  }
  if (a3) { // branch-flip
    sub_43b0();
    v2 = dat_dda30 + 7;
    dat_dda30 = v2;
  }
  else {
    v2 = dat_dda30;
    if ((!a2) || (!(dat_dda30 & 7))) {
      return (long)v2 >> 3; // return-dupe
    }
    sub_4290(0,3);
    dat_dda30 = dat_dda30 + 10 & 0xfffffffffffffff8;
    sub_44a0(a0,0,1);
    v2 = dat_dda30;
  }
  return (long)v2 >> 3;
}

// Function: sub_b290 @ 0xb290
bool sub_b290(int a0,int a1) // ternary
{
  short *v1;
  int *v2;
  unsigned short *v3;
  unsigned char v4;
  unsigned long v5; // rax
  long v6; // rdx
  
  v5 = (unsigned long)dat_dda54;
  dat_dda54 += 1;
  *(char *)(v5 + 0x9d000) = (char)a1;
  if (a0) { // branch-flip
    a0 -= 1;
    v1 = (short *)((unsigned long)*(unsigned char *)((long)a1 + 0xded60) * 4 + 0xe0484);
    *v1 = *v1 + 1;
    v4 = (0x100 <= a0) ? *(unsigned char *)((long)((a0 >> 7) + 0x100) + 0xdeb60) : *(unsigned char *)((long)a0 + 0xdeb60); // branch-flip
    v1 = (short *)((unsigned long)v4 * 4 + 0xdff80);
    *v1 = *v1 + 1;
    v5 = (unsigned long)dat_dda50;
    dat_dda50 += 1;
    *(short *)(v5 * 2 + 0x4c000) = (short)a0;
    dat_dda49 |= dat_dda48;
  }
  else {
    v1 = (short *)((long)a1 * 4 + 0xe0080);
    *v1 = *v1 + 1;
  }
  v4 = dat_dda49;
  dat_dda48 *= '\x02';
  if (!(dat_dda54 & 7)) {
    v5 = (unsigned long)dat_dda4c;
    dat_dda48 = '\x01';
    dat_dda49 = 0;
    dat_dda4c += 1;
    *(unsigned char *)(v5 + 0xdda60) = v4;
  }
  if ((3 <= dat_1901c) && (!(dat_dda54 & 0xfff))) {
    v6 = 0;
    v5 = (unsigned long)dat_dda54 << 3;
    do {
      v2 = (int *)(v6 + 0x19240);
      v3 = (unsigned short *)(v6 + 0xdff80);
      v6 += 4;
      v5 += ((long)*v2 + 5) * (unsigned long)*v3;
    } while (v6 != 0x78);
    if ((dat_dda50 < dat_dda54 >> 1) && (v5 >> 3 < (unsigned long)dat_1a06c - dat_1a078 >> 1))
      return 1;
  }
  return dat_dda54 == 0x7fff || dat_dda50 == 0x8000;
}

// Function: sub_b450 @ 0xb450
void sub_b450(int a0)
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  int v4;
  unsigned short v5; // dx
  unsigned int v6; // edx
  
  v3 = (unsigned long)dat_e0990;
  v6 = (unsigned int)dat_e0994 << ((unsigned char)a0 & 0x1f);
  v5 = (unsigned short)v6;
  v4 = dat_e098c;
  v1 = dat_e0990;
  if (dat_e098c < a0) {
    do {
      a0 -= v4;
      v6 |= (int)v3 << ((unsigned char)a0 & 0x1f);
      v2 = (unsigned long)dat_1b008;
      dat_e0994 = (unsigned short)v6;
      if (dat_1b00c <= dat_1b008) { // branch-flip
        v3 = sub_d830(1);
        dat_e0990 = (unsigned int)v3;
        if (dat_e0990 != 0xffffffff) // branch-flip
          v6 = (unsigned int)dat_e0994;
        else {
          dat_e0990 = 0;
          v6 = (unsigned int)dat_e0994;
          v3 = 0;
        }
      }
      else {
        dat_1b008 += 1;
        v3 = (unsigned long)*(unsigned char *)(v2 + 0x9d000);
        dat_e0990 = (unsigned int)*(unsigned char *)(v2 + 0x9d000);
      }
      v5 = (unsigned short)v6;
      v1 = (unsigned int)v3;
      dat_e098c = 8;
      v4 = 8;
    } while (9 <= a0);
  }
  dat_e098c = v4 - a0;
  dat_e0994 = v5 | (unsigned short)(v1 >> ((unsigned char)(v4 - a0) & 0x1f));
}

// Function: sub_b520 @ 0xb520
void sub_b520(void)
{
  dat_e0990 = 0;
  dat_e0994 = 0;
  dat_e098c = 0;
  sub_b450(0x10); // tail-call
}

// Function: sub_b550 @ 0xb550
void sub_b550(void)
{
  sub_b520();
  dat_e0ba0 = 0;
}

// Function: sub_b570 @ 0xb570
void sub_b570(void)
{
  sub_b550();
  dat_e0988 = 0;
  dat_e0984 = 0;
}

// Function: sub_b5a0 @ 0xb5a0
int sub_b5a0(int a0)
{
  int v1; // eax
  
  v1 = (unsigned int)dat_e0994;
  sub_b450(a0);
  return v1 >> (0x10U - (char)a0 & 0x1fU);
}

// Function: sub_b5c0 @ 0xb5c0
unsigned long sub_b5c0(void)
{
  unsigned short v1;
  unsigned int v2; // eax
  int v3; // r12d
  unsigned long v4; // r12
  
  v1 = *(unsigned short *)((unsigned long)(unsigned char)(dat_e0994 >> 8) * 2 + 0xe09a0);
  v4 = (unsigned long)v1;
  if (0xe <= v1) {
    v2 = 0x80;
    do {
      while (!(v2 & dat_e0994)) {
        v1 = *(unsigned short *)(v4 * 2 + 0x1b880);
        v4 = (unsigned long)v1;
        v2 >>= 1;
        if (v1 <= 0xd) goto label_b61e;
      }
      v2 >>= 1;
      v1 = *(unsigned short *)((unsigned long)((int)v4 + 0x8000) * 2 + 0x1b880);
      v4 = (unsigned long)v1;
    } while (0xe <= v1);
  }
label_b61e:
  sub_b450((unsigned char)*(char *)(v4 + 0xe0bc0));
  if (!(int)v4)
    return v4;
  v3 = (int)v4 + -1;
  return (unsigned long)(unsigned int)((1 << ((unsigned char)v3 & 0x1f)) + sub_b5a0(v3));
}

// Function: sub_b670 @ 0xb670
void sub_b670(unsigned int a0,unsigned char *a1,unsigned int a2,long a3)
{
  unsigned char v1;
  unsigned long v10; // rax
  unsigned char v11; // cl
  unsigned long v12;
  unsigned long v13;
  unsigned short v14 [7];
  unsigned short v15 [17];
  unsigned short v16 [20]; // stack - 0x68
  unsigned short v17 [24];
  unsigned long v18; // rsi
  int v19;
  unsigned short v2;
  unsigned long v20; // r11
  unsigned int v21; // r12d
  long v22; // fs_offset
  unsigned long v23; // stack - 0x40
  unsigned short v3;
  char v4;
  unsigned int v5;
  unsigned short *v6; // rax
  unsigned char *v7;
  long v8; // rax
  unsigned short *v9;
  
  v23 = *(unsigned long *)(v22 + 0x28);
  v9 = &v15[1];
  do {
    v6 = &v9[1];
    *v9 = 0;
    v9 = v6;
  } while (v6 != v14);
  v12 = (unsigned long)a0;
  v7 = a1;
  do {
    v1 = *v7;
    v7 = &v7[1];
    v15[v1] = v15[v1] + 1;
  } while (v7 != &a1[(unsigned long)(a0 - 1) + 1]);
  v19 = 0;
  v16[1] = 0;
  v8 = 1;
  do {
    v19 += (unsigned int)v15[v8] << (0x10U - (char)v8 & 0x1fU);
    v16[v8 + 1] = (unsigned short)v19;
    v8 += 1;
  } while (v8 != 0x11);
  if (!v16[0x11]) {
    v8 = 1;
    v1 = (unsigned char)a2;
    v11 = 0x10 - v1;
    v2 = 0;
    while( true ) {
      v16[v8] = (unsigned short)((int)(unsigned int)v2 >> (v11 & 0x1f));
      v17[v8] = (unsigned short)(1 << (v1 - (char)v8 & 0x1f));
      v8 += 1;
      if (a2 < (unsigned int)v8) break;
      v2 = v16[v8];
    }
    v5 = a2 + 1;
    do {
      v18 = (unsigned long)v5;
      v4 = (char)v5;
      v5 += 1;
      v17[v18] = (unsigned short)(1 << (0x10U - v4 & 0x1fU));
    } while (v5 != 0x11);
    v5 = (int)(unsigned int)v16[(int)(a2 + 1)] >> (v11 & 0x1f);
    if (v5) {
      for (; v5 != 1 << (v1 & 0x1f); v5 = v5 + 1) {
        *(unsigned short *)(a3 + (unsigned long)v5 * 2) = 0;
      }
    }
    v18 = 0;
    do {
      if (a1[v18]) {
        v5 = (unsigned int)a1[v18];
        v20 = (unsigned long)v5;
        v2 = v16[v20];
        v13 = (unsigned long)v2;
        v3 = v17[v20];
        if (v5 <= a2) { // branch-flip
          v21 = (unsigned int)v2;
          v5 = v3 + v21;
          if ((unsigned int)(1 << (v1 & 0x1f)) < v5) break;
          if (v21 < v5) {
            v9 = (unsigned short *)(a3 + v13 * 2);
            do {
              *v9 = (unsigned short)v18;
              v9 = &v9[1];
            } while ((unsigned short *)(a3 + 2 + (((v5 - v21) - 1) + v13) * 2) != v9);
          }
        }
        else {
          v9 = (unsigned short *)(a3 + (unsigned long)(v2 >> (v11 & 0x1f)) * 2);
          for (v19 = v5 - a2; v19; v19 = v19 + -1) {
            while( true ) {
              v10 = (unsigned long)*v9;
              if (!*v9) {
                *(unsigned short *)(v12 * 2 + 0x1b880) = 0;
                *(unsigned short *)(v12 * 2 + 0x2b880) = 0;
                v10 = v12 & 0xffff;
                *v9 = (unsigned short)v12;
                v12 = (unsigned long)((int)v12 + 1);
              }
              v5 = (unsigned int)v13;
              if (1 << (0xf - v1 & 0x1f) & v5) break;
              v9 = (unsigned short *)(v10 * 2 + 0x1b880);
              v13 = (unsigned long)(v5 * 2);
              v19 -= 1;
              if (!v19) goto label_b932;
            }
            v9 = (unsigned short *)(v10 * 2 + 0x2b880);
            v13 = (unsigned long)(v5 * 2);
          }
label_b932:
          *v9 = (unsigned short)v18;
        }
        v16[v20] = v2 + v3;
      }
      v18 += 1;
      if (a0 == v18)
        return;
    } while( true );
  }
  sub_d510("Bad table\n"); // no-return
}

// Function: sub_b980 @ 0xb980
void sub_b980(void) // ternary x2
{
  unsigned short v1;
  int v10; // r14d
  long v11; // rax
  int v2;
  unsigned short v3; // ax
  int v4; // eax
  unsigned int v5; // eax
  char *v6; // rax
  unsigned short *v7; // rdx
  int v8;
  unsigned long v9; // rbx
  
  v4 = sub_b5a0(9);
  if (!v4) {
    v3 = sub_b5a0(9);
    v6 = (char *)0x5c000;
    do {
      *v6 = 0;
      v6 = &v6[1];
    } while (v6 != (char *)0x5c1fe);
    v7 = (unsigned short *)0x4c000;
    do {
      *v7 = v3;
      v7 = &v7[1];
    } while (v7 != (unsigned short *)0x4e000);
    return;
  }
  v10 = 0;
  if (0 < v4) {
    do {
      while( true ) {
        v1 = *(unsigned short *)((unsigned long)(unsigned char)(dat_e0994 >> 8) * 2 + 0xe09a0);
        v9 = (unsigned long)v1;
        if (0x13 <= v1) {
          v5 = 0x80;
          do {
            v1 = (v5 & dat_e0994) ? *(unsigned short *)((long)((int)v9 + 0x8000) * 2 + 0x1b880) : *(unsigned short *)(v9 * 2 + 0x1b880); // branch-flip
            v9 = (unsigned long)v1;
            v5 >>= 1;
          } while (0x13 <= v1);
        }
        v5 = (unsigned int)v9;
        sub_b450((unsigned char)*(char *)((long)(int)v5 + 0xe0bc0));
        if (v5 <= 2) break;
        v11 = (long)v10;
        v10 += 1;
        *(char *)(v11 + 0x5c000) = (char)v9 + '\xfe';
label_b9ee:
        if (v4 <= v10) goto label_ba89;
      }
      v8 = 0;
      if (v5) {
        v8 = (v5 != 1) ? sub_b5a0(9) + 0x14 : sub_b5a0(4) + 3; // branch-flip
        v8 -= 1;
        if (v8 < 0) goto label_b9ee;
      }
      v6 = (char *)((long)v10 + 0x5c000);
      v2 = (int)v6;
      do {
        *v6 = 0;
        v6 = &v6[1];
      } while (0 <= (v8 + v2) - (int)v6);
      v10 = v10 + 1 + v8;
    } while (v10 < v4);
label_ba89:
    if (0x1fd < v10) {
      sub_b670(0x1fe,(unsigned char *)0x5c000,0xc,0x4c000); // return-dupe, tail-call
      return;
    }
  }
  v11 = (long)v10;
  do {
    *(char *)(v11 + 0x5c000) = 0;
    v11 += 1;
  } while ((int)v11 <= 0x1fd);
  sub_b670(0x1fe,(unsigned char *)0x5c000,0xc,0x4c000);
  return;
}

// Function: sub_bb40 @ 0xbb40
void sub_bb40(int a0,unsigned int a1,int a2)
{
  unsigned short v1; // ax
  unsigned char v10; // r12b
  unsigned char *v11; // r13
  unsigned char *v12;
  int v13; // r15d
  int v2; // eax
  int v3;
  long v4; // rax
  unsigned short *v5; // rax
  unsigned long v6;
  char *v7;
  unsigned int v8; // ebp
  int v9; // eax
  
  v2 = sub_b5a0(a1);
  if (!v2) {
    v1 = sub_b5a0(a1);
    v7 = (char *)0xe0bc0;
    do {
      *v7 = 0;
      v7 = &v7[1];
    } while (v7 != (char *)((unsigned long)(unsigned int)(a0 - 1) + 0xe0bc1));
    v5 = (unsigned short *)0xe09a0;
    do {
      *v5 = v1;
      v5 = &v5[1];
    } while (v5 != (unsigned short *)0xe0ba0);
    return;
  }
  v13 = 0;
  do {
    v4 = (long)v13;
    v8 = (a2 + -1) - v13;
    v12 = (unsigned char *)(v4 + 0xe0bc0);
    do {
      if (v2 <= v13) {
        if (v13 < a0) {
          v7 = (char *)((long)v13 + 0xe0bc0);
          do {
            *v7 = 0;
            v7 = &v7[1];
          } while (v7 != (char *)((unsigned long)(unsigned int)((a0 + -1) - v13) + (long)v13 + 0xe0bc1));
        }
        sub_b670(a0,(unsigned char *)0xe0bc0,8,0xe09a0); // tail-call
        return;
      }
      v9 = 3;
      v10 = (unsigned char)(dat_e0994 >> 0xd);
      if (dat_e0994 >> 0xd == 7) {
        if (dat_e0994 & 0x1000) { // branch-flip
          v6 = 0x1000;
          v3 = 7;
          do {
            v9 = v3;
            v6 >>= 1;
            v3 = v9 + 1;
          } while ((unsigned int)dat_e0994 & (unsigned int)v6);
          if (0x10 < v3)
            sub_d510("Bad table\n"); // no-return
          v9 -= 2;
          v10 = (unsigned char)v3;
        }
        else {
          v10 = 7;
          v9 = 4;
        }
      }
      sub_b450(v9);
      v11 = &v12[1];
      *v12 = v10;
      v13 += 1;
      v12 = v11;
    } while (v11 != (unsigned char *)((unsigned long)v8 + v4 + 0xe0bc1));
    v9 = sub_b5a0(2);
    if (1 <= v9) {
      v7 = (char *)((long)v13 + 0xe0bc0);
      do {
        *v7 = 0;
        v7 = &v7[1];
      } while (v7 != (char *)((long)v13 + 0xe0bc1 + (unsigned long)(unsigned int)(v9 - 1)));
      v13 += v9;
    }
  } while( true );
}

// Function: sub_bd50 @ 0xbd50
unsigned long sub_bd50(void)
{
  unsigned short v1;
  unsigned int v2; // eax
  unsigned long v3; // r12
  
  if (!dat_e0ba0) {
    dat_e0ba0 = sub_b5a0(0x10);
    if (!dat_e0ba0)
      return 0x1fe;
    sub_bb40(0x13,5,3);
    sub_b980();
    sub_bb40(0xe,4,0xffffffff);
  }
  dat_e0ba0 -= 1;
  v1 = *(unsigned short *)((unsigned long)(dat_e0994 >> 4) * 2 + 0x4c000);
  v3 = (unsigned long)v1;
  if (0x1fe <= v1) {
    v2 = 8;
    do {
      while (v2 & dat_e0994) {
        v2 >>= 1;
        v1 = *(unsigned short *)((unsigned long)((int)v3 + 0x8000) * 2 + 0x1b880);
        v3 = (unsigned long)v1;
        if (v1 <= 0x1fd) {
          sub_b450((unsigned char)*(char *)(v3 + 0x5c000)); // return-dupe
          return v3;
        }
      }
      v1 = *(unsigned short *)(v3 * 2 + 0x1b880);
      v3 = (unsigned long)v1;
      v2 >>= 1;
    } while (0x1fe <= v1);
  }
  sub_b450((unsigned char)*(char *)(v3 + 0x5c000));
  return v3;
}

// Function: sub_be40 @ 0xbe40
unsigned long sub_be40(void)
{
  bool v1;
  int v2; // eax
  unsigned int v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned int v7;
  
  v5 = 0;
  v1 = 0;
  v7 = dat_e0980;
  while( true ) {
    v6 = v5 & 0xffffffff;
    dat_e0988 -= 1;
    if (dat_e0988 <= -1) break;
    v6 = (unsigned long)v7;
    v7 = v7 + 1 & 0x1fff;
    *(char *)(v5 + 0x3c000) = *(char *)(v6 + 0x3c000);
    v5 += 1;
    v1 = 1;
    if (v5 == 0x2000) {
      dat_e0980 = v7;
      return 0x2000;
    }
  }
  v3 = dat_e0980;
  if (v1)
    v3 = v7;
label_beb0:
  do {
    dat_e0980 = v3;
    v4 = sub_bd50();
    v2 = (int)v4;
    v5 = v6;
    while( true ) {
      if (v2 == 0x1fe) {
        dat_e0984 = 1;
        return v5;
      }
      if ((unsigned int)v4 <= 0xff) break;
      dat_e0988 = (unsigned int)v4 - 0xfd;
      v1 = 0;
      v3 = ((int)v5 + -1) - sub_b5c0() & 0x1fff;
      v6 = v5;
      dat_e0980 = v3;
      while (dat_e0988 = dat_e0988 + -1, 0 <= dat_e0988) {
        v5 = (unsigned long)dat_e0980;
        v7 = (int)v6 + 1;
        dat_e0980 = dat_e0980 + 1 & 0x1fff;
        *(char *)(v6 + 0x3c000) = *(char *)(v5 + 0x3c000);
        v1 = 1;
        v6 = (unsigned long)v7;
        if (v7 == 0x2000)
          return 0x2000;
      }
      if (!v1) goto label_beb0;
      v4 = sub_bd50();
      v2 = (int)v4;
      v5 = v6;
    }
    v7 = (int)v5 + 1;
    v6 = (unsigned long)v7;
    *(char *)(v5 + 0x3c000) = (char)v4;
    v3 = dat_e0980;
    if (v7 == 0x2000)
      return 0x2000;
  } while( true );
}

// Function: sub_bfd0 @ 0xbfd0
unsigned long sub_bfd0(unsigned int a0,unsigned int a1)
{
  int v1; // eax
  
  dat_1b010 = a1;
  dat_1b014 = a0;
  sub_b570();
  while (!dat_e0984) {
    v1 = sub_be40(0x2000,0x3c000);
    if (v1)
      sub_d6c0(a1,(void *)0x3c000,v1);
  }
  return 0;
}

// Function: sub_c040 @ 0xc040
unsigned long sub_c040(unsigned int a0,unsigned int a1) // return-dupe x2
{
  unsigned long *v1;
  unsigned int v10;
  void *v11; // rsi
  unsigned long *v12;
  char *v13; // rdi
  unsigned int v14;
  long v15; // r13
  int v16;
  unsigned long v17;
  unsigned long v18;
  bool v19; // cf
  unsigned char v2;
  long v20; // stack - 0xa0
  unsigned short v21; // stack - 0x98
  unsigned int v22; // stack - 0x8c
  long v23; // stack - 0x88
  long v24; // stack - 0x70
  unsigned int v25; // stack - 0x48
  unsigned int v26; // stack - 0x44
  unsigned int v27; // stack - 0x40
  long v3; // rax
  char *v4;
  unsigned long v5;
  int v6;
  long v7; // rax
  long v8; // rcx
  char *v9;
  
  v5 = (unsigned long)dat_1b008;
  if (dat_1b00c <= dat_1b008) // branch-flip
    dat_19024 = sub_d830(0);
  else {
    dat_1b008 += 1;
    dat_19024 = (unsigned int)*(unsigned char *)(v5 + 0x9d000);
  }
  dat_193e8 = dat_19024 & 0x80;
  if (dat_19024 & 0x60) {
    if (!dat_1b85c)
      sub_f630(stderr,"\n%s: %s: warning, unknown flags 0x%x\n",dat_1b850,0x1b420);
    if (!dat_1b84c)
      dat_1b84c = 2;
  }
  dat_19024 &= 0x1f;
  v3 = 1L << (signed char)dat_19024;
  if (0x10 < dat_19024) {
    sub_f630(stderr,"\n%s: %s: compressed with %d bits, can only handle %d bits\n",dat_1b850,0x1b420,dat_19024,0x10);
    dat_1b84c = 1;
    return 1;
  }
  dat_1b880 = 0;
  v5 = (unsigned long)dat_1b00c;
  dat_1b978 = 0;
  v15 = (unsigned long)(dat_1b008 * 8);
  v20 = 0x101 - (unsigned long)(dat_193e8 == 0);
  v26 = dat_1b00c;
  v7 = 0x1f;
  v12 = (unsigned long *)0x1b888;
  while (v7) {
    v1 = &v12[1];
    *v12 = 0;
    v7 -= 1;
    v12 = v1;
  }
  v7 = 0xff;
  do {
    *(char *)(v7 + 0x3c000) = (char)v7;
    v19 = v7 != 0;
    v7 -= 1;
  } while (v19);
  v25 = 9;
  v16 = 0;
  v23 = 0x1ff;
  v27 = 0x1ff;
  v22 = 0;
  v17 = 0xffffffffffffffff;
label_c19c:
  v14 = (unsigned int)(v15 >> 3);
  v10 = (unsigned int)v5;
  v18 = v17;
  if (v14 <= v10) goto label_c408;
label_c1a9:
  dat_1b00c = 0;
  v11 = (void *)0x9d000;
label_c1ba:
  v26 = sub_d2b0(a0,v11,0x40000);
  if (v26 == 0xffffffff)
    sub_d600(); // no-return
  dat_1b828 += (int)v26;
  dat_1b00c += v26;
  v5 = (unsigned long)dat_1b00c;
label_c1f9:
  v14 = v25;
  v10 = (unsigned int)v5;
  v17 = v18;
  if (v26) { // branch-flip
    v15 = 0;
    v24 = (v5 - v5 % (unsigned long)v25) * 8;
    if (v24 <= 0) goto label_c19c;
  }
  else {
    v24 = v5 * 8 - (long)(int)(v25 - 1);
    if (v24 <= 0) {
label_c5a5:
      if (v16 > 0) {
        sub_d6c0(a1,(void *)0x5c000,v16);
        return 0;
      }
      return 0;
    }
  }
  if (v23 < v20) {
    v7 = -1;
    goto label_c39f;
  }
  v15 = 0;
label_c258:
  v7 = v15 >> 3;
  v2 = (unsigned char)v15;
  v15 += (long)(int)v25;
  v18 = (long)((unsigned long)*(unsigned char *)(v7 + 0x9d001) << 8 | (unsigned long)*(unsigned char *)(v7 + 0x9d002) << 0x10 | (unsigned long)*(unsigned char *)(v7 + 0x9d000)) >> (v2 & 7) & (unsigned long)v27;
  if (v17 != 0xffffffffffffffff) { // branch-flip
    if (v18 != 0x100) { // branch-flip
      if (v20 <= (long)v18) {
        if (v20 < (long)v18) {
          if (1 <= v16)
            sub_d6c0(a1,(void *)0x5c000,v16);
          v13 = "corrupt input.";
          if (!dat_1b860)
            v13 = "corrupt input. Use zcat to recover some data.";
          sub_d510(v13); // no-return
        }
        goto label_c2c8;
      }
      v9 = (char *)0x5bffe;
      v5 = v18;
    }
    else {
      if (dat_193e8) {
        dat_1b880 = 0;
        dat_1b978 = 0;
        v7 = 0x1f;
        v12 = (unsigned long *)0x1b888;
        while (v7) {
          v1 = &v12[1];
          *v12 = 0;
          v7 -= 1;
          v12 = v1;
        }
        v27 = 0x1ff;
        v20 = 0x100;
        v5 = (unsigned long)dat_1b00c;
        v25 = 9;
        v23 = 0x1ff;
        v7 = (long)(int)(v14 << 3);
        v15 = (v15 - 1) + (v7 - (long)((v15 - 1) + v7) % v7);
        goto label_c19c;
      }
      v5 = 0x100;
      v9 = (char *)0x5bffe;
      if (v20 != 0x100) goto label_c2e8;
label_c2c8:
      v9 = (char *)0x5bffd;
      dat_5bffd = (char)v22;
      v5 = v17;
    }
    for (; 0xff < (long)v5; v5 = (unsigned long)*(unsigned short *)(v5 * 2 + 0x1b880)) {
label_c2e8:
      v9 = &v9[-1];
      *v9 = *(char *)(v5 + 0x3c000);
    }
    v2 = *(unsigned char *)(v5 + 0x3c000);
    v4 = &v9[-1];
    v9[-1] = v2;
    v7 = 0x5bffe - (long)v4;
    v6 = v16 + (int)v7;
    v22 = (unsigned int)v2;
    if (0x40000 <= v6) { // branch-flip
      do {
        v6 = 0x40000U - v16;
        if ((int)v7 < 0x40000U - v16)
          v6 = (int)v7;
        if (1 <= v6) {
          v7 = (long)v16;
          v16 += v6;
          memcpy((void *)(v7 + 0x5c000),v4,(long)v6);
        }
        if (0x40000 <= v16) {
          sub_d6c0(a1,(void *)0x5c000,v16);
          v16 = 0;
        }
        v4 = &v4[v6];
        v7 = 0x5bffe - (long)v4;
      } while (1 <= (int)v7);
    }
    else {
      memcpy((void *)((long)v16 + 0x5c000),v4,(long)(int)v7);
      v16 = v6;
    }
    v10 = dat_1b00c;
    if (v20 < v3) {
      v21 = (unsigned short)v17;
      *(unsigned short *)(v20 * 2 + 0x1b880) = v21;
      *(unsigned char *)(v20 + 0x3c000) = v2;
      v20 += 1;
    }
  }
  else {
    if (0xff < v18)
      sub_d510("corrupt input."); // no-return
    v7 = (long)v16;
    v22 = (unsigned int)v18;
    v16 += 1;
    *(char *)(v7 + 0x5c000) = (char)v18;
    v10 = dat_1b00c;
  }
  v17 = v18;
  dat_1b00c = v10;
  if (v24 <= v15) {
    if (!v26) goto label_c5a5;
    v5 = (unsigned long)v10;
    goto label_c19c;
  }
  if (v23 < v20) goto label_c395;
  goto label_c258;
label_c395:
  v7 = v15 - 1;
label_c39f:
  v6 = v25 * 8;
  v25 += 1;
  v8 = (long)v6;
  v23 = v3;
  if (dat_19024 != v25)
    v23 = (1L << ((unsigned char)v25 & 0x3f)) + -1;
  v27 = (1 << ((unsigned char)v25 & 0x1f)) - 1;
  v14 = (unsigned int)(v7 + (v8 - (v8 + v7) % v8) >> 3);
  if (v10 < v14) goto label_c1a9;
label_c408:
  dat_1b00c = v10 - v14;
  v5 = (unsigned long)dat_1b00c;
  if (1 <= (int)dat_1b00c) {
    v9 = (char *)0x9d000;
    do {
      v4 = &v9[1];
      *v9 = v9[(int)v14];
      v9 = v4;
    } while ((char *)((unsigned long)(dat_1b00c - 1) + 0x9d001) != v4);
  }
  if (dat_1b00c <= 0x3f) goto label_c447;
  goto label_c1f9;
label_c447:
  v11 = (void *)(v5 + 0x9d000);
  goto label_c1ba;
}

// Function: sub_c740 @ 0xc740
void sub_c740(void)
{
  int *v1;
  unsigned char v2;
  int v3;
  char *v4; // rax
  long v5; // rdx
  char *v6; // rdx
  
  if (1 <= dat_e0e80) {
    v5 = (long)dat_e0e80;
    v3 = 0;
    do {
      v3 >>= 1;
      v1 = (int *)(v5 * 4 + 0xe0d00);
      *v1 = *v1 - v3;
      *(int *)(v5 * 4 + 0xe0c00) = v3;
      v3 += *(int *)(v5 * 4 + 0xe0c80);
      v5 -= 1;
    } while (0 < (int)v5);
    if (v3 >> 1 == 1) {
      v5 = 1;
      dat_e0bf0 = dat_e0e80;
      if (0xd <= dat_e0e80)
        dat_e0bf0 = 0xc;
      v4 = (char *)((long)(1 << ((unsigned char)dat_e0bf0 & 0x1f)) + 0x5c000);
      do {
        v3 = *(int *)(v5 * 4 + 0xe0c80);
        v2 = (unsigned char)dat_e0bf0 - (char)v5 & 0x1f;
        if (v3 << v2) {
          v6 = &v4[~(unsigned long)(unsigned int)((v3 << v2) - 1)];
          do {
            v4 = &v4[-1];
            *v4 = (char)v5;
          } while (v6 != v4);
        }
        v5 += 1;
      } while ((int)v5 <= dat_e0bf0);
      if ((char *)0x5c001 <= v4) {
        do {
          v4 = &v4[-1];
          *v4 = '\0';
        } while (v4 != (char *)0x5c000);
        return;
      }
      return;
    }
  }
  sub_d510("too few leaves in Huffman tree"); // no-return
}

// Function: sub_c820 @ 0xc820
unsigned long sub_c820(void)
{
  unsigned long v1; // rax
  
  v1 = (unsigned long)dat_1b008;
  if (dat_1b008 < dat_1b00c) {
    dat_1b008 += 1;
    return (unsigned long)*(unsigned char *)(v1 + 0x9d000);
  }
  v1 = sub_d830(0);
  if (0 <= (int)v1)
    return v1;
  sub_d510("invalid compressed data -- unexpected end of file"); // no-return
}

// Function: sub_c870 @ 0xc870
void sub_c870(void)
{
  int *v1;
  unsigned char v2; // al
  unsigned int v3;
  unsigned long v4; // rbx
  int v5;
  int v6;
  long v7; // rax
  char *v8;
  char *v9; // r15
  
  v5 = 4;
  v4 = 0;
  dat_e0e88 = 0;
  do {
    v4 = v4 << 8 | (unsigned long)(unsigned char)sub_c820();
    v5 -= 1;
    dat_e0e88 = v4;
  } while (v5);
  v2 = sub_c820();
  dat_e0e80 = (unsigned int)v2;
  if (0x19 <= (unsigned int)v2 - 1)
    sub_d510("invalid compressed data -- Huffman code bit length out of range"); // no-return
  if (dat_e0e80) { // branch-flip
    v7 = 1;
    v5 = 1;
    v6 = 0;
    do {
      v2 = sub_c820();
      *(unsigned int *)(v7 * 4 + 0xe0c80) = (unsigned int)v2;
      if ((int)(v5 - (unsigned int)(dat_e0e80 == (unsigned int)v7)) < (int)(unsigned int)v2) {
        sub_d510("too many leaves in Huffman tree"); // no-return, return-dupe
      }
      v7 += 1;
      v6 += (unsigned int)v2;
      v5 = (v5 - (unsigned int)v2) * 2 + 1;
    } while ((int)v7 <= (int)dat_e0e80);
    if (0xff < v6)
      sub_d510("too many leaves in Huffman tree");
    v7 = (long)(int)dat_e0e80;
    v1 = (int *)(v7 * 4 + 0xe0c80);
    *v1 = *v1 + 1;
    if (1 <= (int)dat_e0e80) {
      v7 = 1;
      v5 = 0;
      v3 = dat_e0e80;
      do {
        v6 = *(int *)(v7 * 4 + 0xe0c80);
        *(int *)(v7 * 4 + 0xe0d00) = v5;
        if (1 <= v6) {
          v3 = v6 - 1;
          v8 = (char *)((long)v5 + 0xe0d80);
          do {
            v9 = &v8[1];
            *v8 = sub_c820();
            v8 = v9;
          } while (v9 != (char *)((long)v5 + 0xe0d81 + (unsigned long)v3));
          v5 = v5 + 1 + v3;
          v3 = dat_e0e80;
        }
        v7 += 1;
      } while ((int)v7 <= (int)v3);
      v7 = (long)(int)v3;
    }
  }
  else {
    dat_e0c80 += 1;
    v7 = 0;
  }
  v1 = (int *)(v7 * 4 + 0xe0c80);
  *v1 = *v1 + 1;
}

// Function: sub_c9f0 @ 0xc9f0
unsigned long sub_c9f0(unsigned int a0,unsigned int a1)
{
  long v1;
  long v10; // rbx
  unsigned int v11;
  unsigned char v2;
  int v3;
  char v4;
  unsigned int v5; // eax
  unsigned int v6; // eax
  int v7;
  unsigned long v8;
  int v9;
  
  v8 = 0;
  dat_1b010 = a1;
  dat_1b014 = a0;
  sub_c870();
  sub_c740();
  dat_e0be0 = 0;
  dat_e0be8 = 0;
  v5 = (1 << ((unsigned char)dat_e0bf0 & 0x1f)) - 1;
  v3 = *(int *)((long)(int)dat_e0e80 * 4 + 0xe0c80);
  v7 = 0;
  do {
    if ((int)dat_e0bf0 <= v7) { // branch-flip
      v6 = (unsigned int)(v8 >> ((char)v7 - (char)dat_e0bf0 & 0x3fU)) & v5;
      v2 = *(unsigned char *)((unsigned long)v6 + 0x5c000);
      if (v2) { // branch-flip
        v6 >>= (char)dat_e0bf0 - v2 & 0x1f;
        v9 = (unsigned int)v2;
      }
      else {
        v9 = dat_e0bf0;
        if (v6 < *(unsigned int *)((long)(int)dat_e0bf0 * 4 + 0xe0c00)) {
          v10 = (long)(int)(dat_e0bf0 + 1);
          v11 = v5;
          while( true ) {
            v9 = (unsigned int)v10;
            v11 = v11 * 2 + 1;
            while (v7 < v9) {
              v8 = v8 << 8 | (unsigned long)(unsigned char)sub_c820();
              dat_e0be0 += 8;
              dat_e0be8 = v8;
              v7 = dat_e0be0;
            }
            v4 = (char)v10;
            v1 = v10 * 4;
            v10 += 1;
            v6 = (unsigned int)(v8 >> ((char)v7 - v4 & 0x3fU)) & v11;
            if (*(unsigned int *)(v1 + 0xe0c00) <= v6) break;
            v7 = dat_e0be0;
          }
        }
      }
      if ((v6 == v3 - 1U) && (dat_e0e80 == v9)) {
        sub_d7e0();
        if (dat_1b820 == dat_e0e88)
          return 0;
        sub_d510("invalid compressed data--length error"); // no-return
      }
      v8 = (unsigned long)dat_1b004;
      dat_1b004 += 1;
      *(char *)(v8 + 0x3c000) = *(char *)((unsigned long)(v6 + *(int *)((long)v9 * 4 + 0xe0d00)) + 0xe0d80);
      if (dat_1b004 == 0x8000)
        sub_d7e0();
      dat_e0be0 -= v9;
      v8 = dat_e0be8;
    }
    else {
      dat_e0be8 = v8 << 8 | (unsigned long)(unsigned char)sub_c820();
      dat_e0be0 += 8;
      v8 = dat_e0be8;
    }
    v7 = dat_e0be0;
  } while( true );
}

// Function: sub_cc20 @ 0xcc20
unsigned long sub_cc20(unsigned int a0)
{
  unsigned char v1;
  unsigned long v2; // rax
  char *v3; // rsi
  
  v2 = (unsigned long)dat_1b008;
  dat_1b014 = a0;
  dat_1b008 = *(unsigned short *)(v2 + 0x9d01a) + 0x1e + (unsigned int)*(unsigned short *)(v2 + 0x9d01c) + dat_1b008;
  if ((dat_1b008 <= dat_1b00c) && (*(int *)(v2 + 0x9d000) == 0x4034b50)) {
    v3 = "\n%s: %s: first entry not deflated or stored -- use unzip\n";
    dat_19020 = (unsigned int)*(unsigned char *)(v2 + 0x9d008);
    if (!(*(unsigned char *)(v2 + 0x9d008) & 0xf7)) {
      v1 = *(unsigned char *)(v2 + 0x9d006);
      dat_e0ea0 = v1 & 1;
      if (!(v1 & 1)) {
        dat_e0e9c = 1;
        dat_e0e98 = v1 >> 3 & 1;
        return 0;
      }
      v3 = "\n%s: %s: encrypted file -- use unzip\n";
    }
  }
  else {
    v3 = "\n%s: %s: not a valid zip file\n";
  }
  sub_f630(stderr,v3,dat_1b850,0x1b420);
  dat_1b84c = 1;
  return 1;
}

// Function: sub_cd20 @ 0xcd20
unsigned int sub_cd20(unsigned int a0,unsigned int a1) // return-dupe
{
  char v1;
  unsigned int v10;
  unsigned int v11;
  unsigned long v12;
  bool v13;
  unsigned int v14; // stack - 0x44
  unsigned int v15; // stack - 0x3c
  int v2; // eax
  unsigned long v3;
  unsigned long v4; // rax
  unsigned int *v5;
  char *v6; // rbx
  unsigned int v7; // stack - 0x48
  char v8 [4];
  char v9 [8];
  
  dat_1b010 = a1;
  dat_1b014 = a0;
  sub_d1f0(NULL,0);
  if (dat_e0e9c) { // branch-flip
    if (dat_e0e98) { // branch-flip
      v11 = 0;
      v10 = 0;
    }
    else {
      v10 = dat_9d00e;
      v11 = dat_9d016;
    }
    v4 = (unsigned long)v10;
    if (dat_19020 == 8) goto label_ce42;
    if (dat_19020) {
      sub_d510("internal error, invalid method"); // no-return, return-dupe
    }
    v3 = (unsigned long)dat_9d012;
    v12 = (unsigned long)dat_9d016;
    if (dat_e0ea0)
      v3 -= 0xc;
    if (v12 != v3) {
      sub_f630(stderr,"len %lu, siz %lu\n",v12);
      sub_d510("invalid compressed data--length mismatch"); // no-return
    }
    for (; v12; v12 = v12 - 1) {
      while( true ) {
        v3 = (unsigned long)dat_1b008;
        if (dat_1b00c <= dat_1b008) // branch-flip
          v1 = sub_d830(0);
        else {
          dat_1b008 += 1;
          v1 = *(char *)(v3 + 0x9d000);
        }
        v3 = (unsigned long)dat_1b004;
        dat_1b004 += 1;
        *(char *)(v3 + 0x3c000) = v1;
        if (dat_1b004 == 0x8000) break;
        v12 -= 1;
        if (!v12) goto label_d030;
      }
      sub_d7e0();
    }
label_d030:
    sub_d7e0();
    if (!dat_e0e9c) goto label_ce68;
label_d045:
    if (dat_e0e98) {
      v5 = &v7;
      do {
        while (v4 = (unsigned long)dat_1b008, dat_1b00c <= dat_1b008) {
          v6 = (char *)((long)v5 + 1);
          *(char *)v5 = sub_d830(0);
          v5 = (unsigned int *)v6;
          if (v6 == v9) goto label_d0ab;
        }
        dat_1b008 += 1;
        v6 = (char *)((long)v5 + 1);
        *(char *)v5 = *(char *)(v4 + 0x9d000);
        v5 = (unsigned int *)v6;
      } while (v6 != v9);
label_d0ab:
      v4 = (unsigned long)v14;
      v11 = v15;
    }
  }
  else {
    if (dat_19020 != 8)
      sub_d510("internal error, invalid method");
    v11 = 0;
    v4 = 0;
label_ce42:
    v2 = sub_9e80();
    if (v2 == 3)
      sub_d550(); // no-return
    if (v2)
      sub_d510("invalid compressed data--format violated"); // no-return
    if (dat_e0e9c) goto label_d045;
label_ce68:
    v5 = &v7;
    do {
      while (v4 = (unsigned long)dat_1b008, dat_1b00c <= dat_1b008) {
        v6 = (char *)((long)v5 + 1);
        *(char *)v5 = sub_d830(0);
        v5 = (unsigned int *)v6;
        if (v6 == v8) goto label_cebb;
      }
      dat_1b008 += 1;
      v6 = (char *)((long)v5 + 1);
      *(char *)v5 = *(char *)(v4 + 0x9d000);
      v5 = (unsigned int *)v6;
    } while (v6 != v8);
label_cebb:
    v4 = (unsigned long)v7;
    v11 = v14;
  }
  v13 = sub_d1f0((unsigned char *)0x5c000,0) != v4;
  if (v13)
    sub_f630(stderr,"\n%s: %s: invalid compressed data--crc error\n",dat_1b850,0x1b420);
  if (dat_1b820 != v11) { // branch-flip
    sub_f630(stderr,"\n%s: %s: invalid compressed data--length error\n",dat_1b850,0x1b420);
    if (!dat_e0e9c) goto label_cfcb;
    v13 = 1;
label_cf4a:
    if ((dat_1b008 + 4 < dat_1b00c) && (((long)(int)(unsigned int)*(unsigned short *)((unsigned long)dat_1b008 + 0x9d002) << 0x10 | (long)(int)(unsigned int)*(unsigned short *)((unsigned long)dat_1b008 + 0x9d000)) == 0x4034b50)) {
      if (!dat_1b860) {
        sub_f630(stderr,"%s: %s has more than one entry -- unchanged\n",dat_1b850,0x1b420);
        dat_e0e9c = 0;
        goto label_cfcb;
      }
      if (!dat_1b85c)
        sub_f630(stderr,"%s: %s has more than one entry--rest ignored\n",dat_1b850,0x1b420);
      if (!dat_1b84c)
        dat_1b84c = 2;
    }
  }
  else if (dat_e0e9c) goto label_cf4a;
  dat_e0e9c = 0;
  if (!v13) {
    dat_e0e9c = 0;
    dat_e0e98 = 0;
    dat_e0e90 = v4;
    return 0;
  }
label_cfcb:
  dat_e0e98 = 0;
  dat_1b84c = 1;
  if (dat_1b858) {
    dat_1b84c = 1;
    dat_e0e90 = v4;
    dat_e0e98 = 0;
    return 1;
  }
  dat_e0e90 = v4;
  sub_8620(); // no-return
}

// Function: sub_d1d0 @ 0xd1d0
void sub_d1d0(int a0,void *a1,unsigned int a2)
{
  if (0x80000000 <= a2)
    a2 = 0x7fffffff;
  write(a0,a1,(unsigned long)a2);
}

// Function: sub_d1f0 @ 0xd1f0
unsigned long sub_d1f0(unsigned char *a0,unsigned int a1) // early-return
{
  unsigned char *v1;
  unsigned char v2;
  
  if (!a0) {
    dat_193f0 = 0xffffffff;
    return 0;
  }
  if (a1) {
    v1 = &a0[a1];
    do {
      v2 = *a0;
      a0 = &a0[1];
      dat_193f0 = dat_193f0 >> 8 ^ *(unsigned long *)((unsigned long)(((unsigned int)v2 ^ (unsigned int)dat_193f0) & 0xff) * 8 + 0x13800);
    } while (a0 != v1);
  }
  return dat_193f0 ^ 0xffffffff;
}

// Function: sub_d250 @ 0xd250
unsigned long sub_d250(void)
{
  return dat_193f0 ^ 0xffffffff;
}

// Function: sub_d270 @ 0xd270
void sub_d270(void)
{
  dat_1b004 = 0;
  dat_1b008 = 0;
  dat_1b00c = 0;
  dat_1b820 = 0;
  dat_1b828 = 0;
}

// Function: sub_d2b0 @ 0xd2b0
unsigned long sub_d2b0(int a0,void *a1,unsigned int a2)
{
  unsigned long v1; // rax
  int *v2; // rax
  unsigned long v3; // r14
  
  if (0x80000000 <= a2)
    a2 = 0x7fffffff;
  v1 = read(a0,a1,(unsigned long)a2);
  v3 = v1 & 0xffffffff;
  if ((int)v1 < 0) {
    v2 = __errno_location();
    if (*v2 == 0xb) {
      v1 = sub_e270(a0,3);
      if (0 <= (int)(unsigned int)v1) {
        if (v1 & 0x800) { // branch-flip
          if (sub_e270(a0,4,(unsigned int)v1 & 0xfffff7ff) != -1)
            v3 = read(a0,a1,(unsigned long)a2) & 0xffffffff;
        }
        else {
          *v2 = 0xb;
        }
      }
    }
  }
  return v3;
}

// Function: sub_d350 @ 0xd350
unsigned char * sub_d350(unsigned char *a0)
{
  long *v1; // rax
  unsigned char v2;
  unsigned char *v3;
  
  v2 = *a0;
  if (v2) {
    v1 = __ctype_b_loc();
    v3 = a0;
    do {
      if (*(unsigned char *)(*v1 + 1 + (unsigned long)v2 * 2) & 1)
        v2 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v2 * 4);
      *v3 = v2;
      v2 = v3[1];
      v3 = &v3[1];
    } while (v2);
  }
  return a0;
}

// Function: sub_d3b0 @ 0xd3b0
char * sub_d3b0(char *a0)
{
  return sub_e060(a0); // tail-call
}

// Function: sub_d3c0 @ 0xd3c0
void sub_d3c0(char *a0)
{
  unlink(a0); // tail-call
}

// Function: sub_d3d0 @ 0xd3d0
char * sub_d3d0(unsigned int *a0,unsigned long *a1,char *a2)
{
  char v1;
  char *v2; // rax
  char *v3;
  unsigned long *v4; // rax
  unsigned long v5; // rax
  char *v6;
  int v7; // r12d
  unsigned long *v8;
  unsigned long *v9; // r12
  
  v2 = getenv(a2);
  v6 = NULL;
  if (v2) {
    v7 = 0;
    v2 = (char *)sub_f5d0(v2);
    v6 = v2;
    if (*v2) { // branch-flip
      do {
        v6 = &v6[strspn(v6," \t")];
        if (!*v6) {
          if (!v7) {
            v6 = NULL; // return-dupe
            free(v2);
            return v6;
          }
          break;
        }
        v7 += 1;
        v3 = &v6[strcspn(v6," \t")];
        if (!*v3) break;
        *v3 = '\0';
        v6 = &v3[1];
      } while (v3[1]);
      *a0 = v7 + 1U;
      v4 = (unsigned long *)sub_f4d0((long)(v7 + 2),8);
      v8 = (unsigned long *)*a1;
      *a1 = v4;
      *v4 = *v8;
      v6 = v2;
      v8 = &v4[1];
      do {
        v9 = &v8[1];
        v5 = strspn(v6," \t");
        *v8 = &v6[v5];
        v3 = &v6[v5];
        do {
          v6 = &v3[1];
          v1 = *v3;
          v3 = v6;
        } while (v1);
        v8 = v9;
      } while (&v4[v7 + 1U] != v9);
      v4[v7 + 1U] = 0;
      v6 = v2;
    }
    else {
      v6 = NULL;
      free(v2);
    }
  }
  return v6;
}

// Function: sub_d510 @ 0xd510
void sub_d510(char *a0)
{
  sub_f630(stderr,"\n%s: %s: %s\n",dat_1b850,0x1b420,a0);
  sub_8620(); // no-return
}

// Function: sub_d550 @ 0xd550
void sub_d550(void)
{
  sub_f630(stderr,"\n%s: memory_exhausted\n",dat_1b850);
  sub_8620(); // no-return
}

// Function: sub_d580 @ 0xd580
void sub_d580(char *a0) // return-dupe
{
  if (!dat_1b85c) {
    sub_f630(stderr,"%s: %s: warning: %s\n",dat_1b850,0x1b420,a0);
    if (dat_1b84c)
      return;
    dat_1b84c = 2;
    return;
  }
  if (dat_1b84c)
    return;
  dat_1b84c = 2;
}

// Function: sub_d600 @ 0xd600
void sub_d600(void)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  sub_f630(stderr,"\n%s: ",dat_1b850);
  if (v1) { // branch-flip
    *v2 = v1;
    perror((char *)0x1b420);
  }
  else {
    sub_f630(stderr,"%s: unexpected end of file\n",0x1b420);
  }
  sub_8620(); // no-return
}

// Function: sub_d670 @ 0xd670
void sub_d670(void)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  sub_f630(stderr,"\n%s: ",dat_1b850);
  *v2 = v1;
  perror((char *)0x1b020);
  sub_8620(); // no-return
}

// Function: sub_d6c0 @ 0xd6c0
void sub_d6c0(unsigned int a0,void *a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  a2 &= 0xffffffff;
  dat_1b820 += a2;
  if (dat_1b858)
    return;
  while( true ) {
    v2 = sub_d1d0(a0,a1,a2);
    v1 = (int)v2;
    if ((int)a2 == v1)
      return;
    if (v1 == -1) break;
    a2 = (unsigned long)(unsigned int)((int)a2 - v1);
    a1 = (void *)((long)a1 + (v2 & 0xffffffff));
  }
  sub_d670(); // no-return
}

// Function: sub_d720 @ 0xd720
unsigned long sub_d720(unsigned int a0,unsigned int a1) // return-dupe
{
  int v1; // eax
  unsigned long v2; // rsi
  
  *__errno_location() = 0;
  v2 = (unsigned long)dat_1b008;
  if (dat_1b00c <= dat_1b008)
    return 0;
  do {
    sub_d6c0(a1,(void *)(v2 + 0x9d000),dat_1b00c - (int)v2);
    v1 = sub_d2b0(a0,(void *)0x9d000,0x40000);
    if (v1 == 0xffffffff)
      sub_d600(); // no-return
    v2 = 0;
    dat_1b828 += v1;
    dat_1b008 = 0;
    dat_1b00c = v1;
  } while (v1);
  dat_1b008 = 0;
  return 0;
}

// Function: sub_d7a0 @ 0xd7a0
void sub_d7a0(void)
{
  if (!dat_1b004)
    return;
  sub_d6c0(dat_1b010,(void *)0x5c000);
  dat_1b004 = 0;
}

// Function: sub_d7e0 @ 0xd7e0
void sub_d7e0(void)
{
  unsigned long v1; // r8
  void *v2; // r9
  
  v1 = (unsigned long)dat_1b004;
  if (!dat_1b004)
    return;
  v2 = (void *)0x3c000;
  sub_d1f0((unsigned char *)0x3c000,v1);
  sub_d6c0(dat_1b010,v2,v1 & 0xffffffff);
  dat_1b004 = 0;
}

// Function: sub_d830 @ 0xd830
unsigned long sub_d830(int a0)
{
  int v1; // eax
  unsigned long v2; // rax
  
  v2 = 0;
  dat_1b00c = 0;
  do {
    v1 = sub_d2b0(dat_1b014,(void *)(v2 + 0x9d000),0x40000 - (int)v2);
    if (!v1) {
      v2 = (unsigned long)dat_1b00c;
      if (!dat_1b00c) {
        if (a0)
          return 0xffffffff;
        sub_d7e0();
        *__errno_location() = 0;
        sub_d600(); // no-return, return-dupe
      }
      break;
    }
    if (v1 == -1) {
      sub_d600();
    }
    dat_1b00c = v1 + dat_1b00c;
    v2 = (unsigned long)dat_1b00c;
  } while (dat_1b00c <= 0x3ffff);
  dat_1b008 = 1;
  dat_1b828 += v2;
  return (unsigned long)dat_9d000;
}

// Function: sub_d8e0 @ 0xd8e0
void sub_d8e0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_f630(a2,"%5.1f%%"); // tail-call
}

// Function: sub_d930 @ 0xd930
void sub_d930(FILE *a0,unsigned long a1,int a2) // return-dupe
{
  char *v1;
  char v2;
  char *v3;
  unsigned long v4;
  char *v5;
  char v6 [10];
  
  v1 = &v6[2];
  v5 = v1;
  if (0 <= (long)a1) { // branch-flip
    do {
      v5 = &v5[-1];
      v4 = a1 / 10;
      *v5 = (char)a1 + (char)v4 * '\xf6' + '0';
      a1 = v4;
    } while (v4);
  }
  else {
    v3 = v1;
    do {
      v5 = v3;
      v4 = (long)a1 / 10;
      v5[-1] = ((char)v4 * '\n' + '0') - (char)a1;
      a1 = v4;
      v3 = &v5[-1];
    } while (v4);
    v5[-2] = '-';
    v5 = &v5[-2];
  }
  a2 -= (int)v1 - (int)v5;
  if (1 <= a2) {
    do {
      a2 -= 1;
      putc(0x20,a0);
    } while (a2);
  }
  if (v1 <= v5)
    return;
  do {
    v2 = *v5;
    v5 = &v5[1];
    putc((int)v2,a0);
  } while (v5 != v1);
}

// Function: sub_da60 @ 0xda60
char * sub_da60(unsigned int a0,unsigned int a1)
{
  long v1;
  unsigned long v10; // rbx
  unsigned long v11; // rbx
  unsigned short v12; // stack - 0x22
  char v13;
  unsigned int v14;
  char *v15;
  char *v16; // r12
  char v2;
  unsigned int v3;
  char *v4; // rax
  char v5; // ah
  char v6;
  char v7;
  unsigned long v8;
  unsigned int v9; // ebx
  
  v12 = 0;
  dat_5c000 = 0x8b1f;
  v2 = (dat_1b848 != 0) << 3;
  dat_5c003 = v2;
  dat_19020 = 8;
  dat_5c002 = 8;
  dat_1b004 = 4;
  dat_1b010 = a1;
  dat_1b014 = a0;
  if (0 <= dat_1b838) { // branch-flip
    if ((unsigned long)(dat_1b830 - 1U) <= 0xfffffffe) {
      dat_5c004 = (char)dat_1b830;
      v5 = (char)((unsigned long)dat_1b830 >> 8);
      dat_5c005 = v5;
      v13 = (char)((unsigned long)dat_1b830 >> 0x10);
      v5 = (char)((unsigned long)dat_1b830 >> 0x18);
      v7 = v5;
      goto label_db09;
    }
    sub_d580("file timestamp out of range for gzip format");
    if (0x3fffe <= dat_1b004) { // branch-flip
      v3 = dat_1b004 + 1;
      *(char *)((unsigned long)dat_1b004 + 0x5c000) = 0;
      if (v3 == 0x40000) {
        dat_1b004 = v3;
        sub_d7a0();
        v3 = dat_1b004;
      }
      dat_1b004 = v3 + 1;
      *(char *)((unsigned long)v3 + 0x5c000) = 0;
      if (dat_1b004 != 0x40000) { // branch-flip
        v3 += 2;
        v14 = dat_1b004;
      }
      else {
        sub_d7a0();
        v3 = dat_1b004 + 1;
        v14 = dat_1b004;
      }
    }
    else {
      v14 = dat_1b004 + 2;
      v3 = dat_1b004 + 3;
      *(char *)((unsigned long)dat_1b004 + 0x5c000) = 0;
      *(char *)((unsigned long)(dat_1b004 + 1) + 0x5c000) = 0;
    }
    v8 = (unsigned long)v3;
    if (0x3fffe <= v14) {
      *(char *)((unsigned long)v14 + 0x5c000) = 0;
      if (v3 == 0x40000) {
        dat_1b004 = v3;
        sub_d7a0();
        v4 = (unsigned long)(unsigned long)dat_1b004;
        v8 = (unsigned long)v4;
      }
      v4 = (char *)v8;
      dat_1b004 = (int)v4 + 1;
      *(char *)((long)v4 + 0x5c000) = 0;
      if (dat_1b004 == 0x40000)
        sub_d7a0();
      goto label_db34;
    }
    v7 = 0;
    v13 = 0;
    v4 = (char *)v8;
  }
  else {
    v7 = 0;
    v13 = 0;
    dat_5c005 = 0;
    dat_5c004 = 0;
label_db09:
    v4 = (char *)0x7;
    v14 = 6;
  }
  dat_1b004 = v14 + 2;
  *(char *)((unsigned long)v14 + 0x5c000) = v13;
  *(char *)((long)v4 + 0x5c000) = v7;
label_db34:
  sub_d1f0(NULL,0);
  sub_4260(a1);
  sub_ad80(&v12,0x19020);
  v6 = '\x04';
  if (dat_1901c != 1)
    v6 = (dat_1901c == 9) * '\x02';
  v3 = dat_1b004 + 1;
  *(char *)((unsigned long)dat_1b004 + 0x5c000) = v6;
  if (v3 == 0x40000) {
    dat_1b004 = v3;
    sub_d7a0();
    v3 = dat_1b004;
  }
  dat_1b004 = v3;
  v4 = (unsigned long)(unsigned long)v3;
  dat_1b004 += 1;
  *(char *)((long)v4 + 0x5c000) = 3;
  v3 = dat_1b004;
  if (dat_1b004 == 0x40000) {
    sub_d7a0();
    v3 = dat_1b004;
  }
  v8 = (unsigned long)v3;
  if (dat_1b848) {
    v4 = (char *)sub_d3b0((char *)0x1b420);
    v15 = v4;
    do {
      while( true ) {
        v3 = (int)v8 + 1;
        *(char *)(v8 + 0x5c000) = *v15;
        v8 = (unsigned long)v3;
        dat_1b004 = v3;
        if (v3 != 0x40000) break;
        sub_d7a0();
        v16 = &v15[1];
        v8 = (unsigned long)dat_1b004;
        v2 = *v15;
        v15 = v16;
        if (!v2) goto label_dc10;
      }
      v16 = &v15[1];
      v2 = *v15;
      v15 = v16;
    } while (v2);
  }
label_dc10:
  dat_e0ea8 = v8;
  sub_4ed0(dat_1901c);
  if ((dat_1b840 != -1) && (dat_1b840 != dat_1b828))
    sub_f630(stderr,"%s: %s: file size changed while zipping\n",dat_1b850,0x1b420);
  v2 = sub_d250();
  v14 = dat_1b004 + 1;
  v8 = (unsigned long)v14;
  if (0x3fffe <= dat_1b004) { // branch-flip
    *(char *)((unsigned long)dat_1b004 + 0x5c000) = v2;
    dat_1b004 = v14;
    if (v14 == 0x40000) {
      sub_d7a0();
      v8 = (unsigned long)dat_1b004;
    }
    sub_d250();
    dat_1b004 = (int)v8 + 1;
    v10 = (unsigned long)dat_1b004;
    *(char *)(v8 + 0x5c000) = v5;
    if (dat_1b004 == 0x40000) {
      sub_d7a0();
      v10 = (unsigned long)dat_1b004;
    }
  }
  else {
    v9 = dat_1b004 + 2;
    v10 = (unsigned long)v9;
    *(char *)((unsigned long)dat_1b004 + 0x5c000) = v2;
    dat_1b004 = v14;
    sub_d250();
    *(char *)(v8 + 0x5c000) = v5;
    dat_1b004 = v9;
  }
  sub_d250();
  v3 = (unsigned int)v10;
  dat_1b004 = v3 + 1;
  v8 = (unsigned long)dat_1b004;
  if (0x3fffe <= v3) { // branch-flip
    *(char *)(v10 + 0x5c000) = dat_2;
    if (dat_1b004 == 0x40000) {
      sub_d7a0();
      v8 = (unsigned long)dat_1b004;
    }
    sub_d250();
    dat_1b004 = (int)v8 + 1;
    v11 = (unsigned long)dat_1b004;
    *(char *)(v8 + 0x5c000) = dat_3;
    if (dat_1b004 == 0x40000) {
      sub_d7a0();
      v11 = (unsigned long)dat_1b004;
    }
  }
  else {
    v11 = (unsigned long)(v3 + 2);
    *(char *)(v10 + 0x5c000) = dat_2;
    sub_d250();
    *(char *)(v8 + 0x5c000) = dat_3;
  }
  v14 = (unsigned int)v11;
  dat_1b004 = v14 + 1;
  v8 = (unsigned long)dat_1b004;
  v2 = (char)dat_1b828; // branch-flip
  if (0x3fffe <= v14) {
    *(char *)(v11 + 0x5c000) = v2;
    if (dat_1b004 == 0x40000) {
      sub_d7a0();
      v8 = (unsigned long)dat_1b004;
    }
    dat_1b004 = (int)v8 + 1;
    v10 = (unsigned long)dat_1b004;
    v5 = (char)((unsigned long)dat_1b828 >> 8);
    *(char *)(v8 + 0x5c000) = v5;
    v4 = (char *)dat_1b828;
    if (dat_1b004 == 0x40000) {
      sub_d7a0();
      v10 = (unsigned long)dat_1b004;
      v4 = (char *)dat_1b828;
    }
  }
  else {
    v10 = (unsigned long)(v14 + 2);
    *(char *)(v11 + 0x5c000) = v2;
    v5 = (char)((unsigned long)dat_1b828 >> 8);
    *(char *)(v8 + 0x5c000) = v5;
    v4 = (char *)dat_1b828;
  }
  v1 = (long)v4;
  v14 = (unsigned int)v10;
  dat_1b004 = v14 + 1;
  v8 = (unsigned long)dat_1b004;
  v2 = (char)((unsigned long)v4 >> 0x10); // branch-flip
  if (0x3fffe <= v14) {
    *(char *)(v10 + 0x5c000) = v2;
    v4 = (char *)v1;
    if (dat_1b004 == 0x40000) {
      sub_d7a0();
      v8 = (unsigned long)dat_1b004;
      v4 = (char *)dat_1b828;
    }
    dat_1b004 = (int)v8 + 1;
    v5 = (char)((unsigned long)v4 >> 0x18);
    *(char *)(v8 + 0x5c000) = v5;
    if (dat_1b004 == 0x40000)
      sub_d7a0();
  }
  else {
    dat_1b004 = v14 + 2;
    *(char *)(v10 + 0x5c000) = v2;
    v5 = (char)((unsigned long)v1 >> 0x18);
    *(char *)(v8 + 0x5c000) = v5;
  }
  dat_e0ea8 += 8;
  sub_d7a0();
  v4 = NULL;
  return v4;
}

// Function: sub_e010 @ 0xe010
unsigned long sub_e010(unsigned char *a0,unsigned int a1)
{
  unsigned int v1; // eax
  
  v1 = sub_d2b0(dat_1b014,a0,a1);
  if (v1) {
    if (v1 == 0xffffffff)
      sub_d600(); // no-return
    sub_d1f0(a0,v1);
    dat_1b828 += (unsigned long)v1;
  }
  return (unsigned long)v1;
}

// Function: sub_e060 @ 0xe060
char * sub_e060(char *a0)
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

// Function: sub_e0c0 @ 0xe0c0
void sub_e0c0(char *a0)
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

// Function: sub_e0f0 @ 0xe0f0
unsigned long sub_e0f0(FILE *a0) // early-return
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
    if (sub_e460(a0)) {
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

// Function: sub_e180 @ 0xe180
void sub_e180(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_e190 @ 0xe190
int sub_e190(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_e0eb0) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_e0eb0 = 1;
      return v2;
    }
    v2 = sub_e180(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_e0eb0 = -1;
  }
  else {
    v2 = sub_e180(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_e0eb0 != -1)
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

// Function: sub_e270 @ 0xe270
void sub_e270(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_e180(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_e190(a0,a2);
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

// Function: sub_e3c0 @ 0xe3c0
void sub_e3c0(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_e980(open(a0,a1,v1));
}

// Function: sub_e440 @ 0xe440
void sub_e440(FILE *a0)
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_e4a0(a0,0,1); // tail-call
}

// Function: sub_e460 @ 0xe460
void sub_e460(FILE *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_e440(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_e4a0 @ 0xe4a0
int sub_e4a0(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_e520 @ 0xe520
void sub_e520(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_e980(openat(a0,a1,a2,v1));
}

// Function: sub_e5a0 @ 0xe5a0
void sub_e5a0(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_e5b0 @ 0xe5b0
char * sub_e5b0(DIR *a0,unsigned int a1)
{
  long v1;
  long v10;
  char *v11;
  long v12; // stack - 0x48
  long v13; // stack - 0x50
  long v14;
  long v15; // r15
  long v2;
  char v3;
  int *v4; // rax
  dirent *v5; // rax
  char *v6;
  char *v7; // rax
  void *v8; // rax
  char *v9;
  
  v13 = 0;
  v12 = 0;
  v14 = *(long *)((unsigned long)a1 * 8 + 0x18b50);
  if (a0) { // branch-flip
    v10 = 0;
    v15 = 0;
    v8 = NULL;
    v4 = __errno_location();
    v9 = NULL;
    while( true ) {
      *v4 = 0;
      v5 = readdir(a0);
      if (!v5) break;
      v3 = v5->field_0x13;
      v6 = &v5->field_0x13;
      if ((v3 == '.') && (v3 = v5->field_0x14, v3 == '.'))
        v3 = v5->field_0x15;
      if (v3) {
        v1 = strlen(v6) + 1;
        if (v14) { // branch-flip
          if (v12 == v15)
            v8 = (void *)sub_f3f0(v8,&v12,1,-1,8);
          v2 = v15 * 8;
          v15 += 1;
          *(unsigned long *)((long)v8 + v2) = sub_f5d0(v6);
        }
        else {
          if (v13 - v10 <= v1)
            v9 = (char *)sub_f3f0(v9,&v13,v1 - (v13 - v10),0x7ffffffffffffffe,1);
          memcpy(&v9[v10],v6,v1);
        }
        v10 += v1;
      }
    }
    if (*v4) { // branch-flip
      free(v8);
      v6 = NULL;
      free(v9);
    }
    else {
      if (v14) { // branch-flip
        if (v15) { // branch-flip
          v11 = NULL;
          qsort(v8,v15,8,v14);
          v14 = 0;
          v9 = (char *)sub_f250(v10 + 1);
          do {
            v7 = stpcpy(&v9[(long)v11],*(char **)((long)v8 + v14 * 8));
            v10 = v14 * 8;
            v14 += 1;
            v11 = &v7[(long)&v11[1 - (long)&v9[(long)v11]]];
            free(*(void **)((long)v8 + v10));
          } while (v15 != v14);
          v6 = &v9[(long)v11];
        }
        else {
          v6 = (char *)sub_f250(v10 + 1);
          v9 = v6;
        }
        free(v8);
      }
      else {
        if (v10 == v13)
          v9 = (char *)sub_f2c0(v9,v10 + 1);
        v6 = &v9[v10];
      }
      *v6 = 0;
      v6 = v9;
    }
  }
  else {
    v6 = NULL;
  }
  return v6;
}

// Function: sub_e870 @ 0xe870
void * sub_e870(unsigned long a0,unsigned int a1)
{
  DIR *v1; // rax
  void *v2; // rax
  
  v1 = (DIR *)sub_10990();
  if (!v1)
    return NULL;
  v2 = (void *)sub_e5b0(v1,a1);
  if (!closedir(v1))
    return v2;
  free(v2);
  return NULL;
}

// Function: sub_e8e0 @ 0xe8e0
unsigned long sub_e8e0(struct_3 *a0)
{
  return a0->field_0x50;
}

// Function: sub_e8f0 @ 0xe8f0
unsigned long sub_e8f0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x70);
}

// Function: sub_e900 @ 0xe900
unsigned long sub_e900(struct_4 *a0)
{
  return a0->field_0x60;
}

// Function: sub_e910 @ 0xe910
unsigned long sub_e910(void)
{
  return 0;
}

// Function: sub_e920 @ 0xe920
undefined16 sub_e920(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x48);
}

// Function: sub_e930 @ 0xe930
undefined16 sub_e930(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x68);
}

// Function: sub_e940 @ 0xe940
undefined16 sub_e940(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x58);
}

// Function: sub_e950 @ 0xe950
undefined16 sub_e950(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_e970 @ 0xe970
unsigned int sub_e970(unsigned int a0)
{
  return a0;
}

// Function: sub_e980 @ 0xe980
unsigned int sub_e980(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_10b10();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_e9e0 @ 0xe9e0
int sub_e9e0(unsigned long *a0)
{
  unsigned long v1;
  unsigned long v2;
  bool v3;
  char v4; // dl
  unsigned int v5; // r9d
  
  v1 = a0[1];
  if ((v1 - 0x3ffffffe < 2) || (v1 <= 999999999)) {
    v2 = a0[3];
    if ((v2 - 0x3ffffffe < 2) || (v2 <= 999999999)) {
      v3 = v1 - 0x3ffffffe <= 1;
      if (v3)
        *a0 = 0;
      v4 = v3 && v1 == 0x3ffffffe;
      v5 = (unsigned int)v3;
      if (v2 - 0x3ffffffe <= 1) {
        a0[2] = 0;
        v5 = 1;
        v4 += v2 == 0x3ffffffe;
      }
      return (v4 == '\x01') + v5;
    }
  }
  *__errno_location() = 0x16;
  return -1;
}

// Function: sub_ea90 @ 0xea90
unsigned long sub_ea90(void *a0,long *a1) // return-dupe x3
{
  unsigned long *v1;
  long v2;
  char v3 [16];
  
  v1 = (unsigned long *)*a1;
  v2 = v1[3];
  if (v1[1] != 0x3ffffffe) { // branch-flip
    if (v1[1] == 0x3fffffff) {
      if (v2 == 0x3fffffff) {
        *a1 = 0;
        return 0;
      }
      sub_10a40(v1);
      v2 = v1[3];
    }
    if (v2 == 0x3ffffffe) {
      v3 = sub_e940(a0);
      v1[3] = SUB168(v3,8);
      v1[2] = SUB168(v3,0);
      return 0;
    }
  }
  else {
    if (v2 == 0x3ffffffe)
      return 1;
    v3 = sub_e920(a0);
    *v1 = SUB168(v3,0);
    v1[1] = SUB168(v3,8);
  }
  if (v2 != 0x3fffffff)
    return 0;
  sub_10a40((timespec *)&v1[2]);
  return 0;
}

// Function: sub_eb50 @ 0xeb50
int sub_eb50(int a0,char *a1,long *a2)
{
  long v1;
  long v10; // stack - 0xf8
  bool v11; // zf
  char v12 [16];
  long v13; // stack - 0x130
  long v14; // stack - 0x128
  long v15; // stack - 0x120
  long v16; // stack - 0x110
  long v17; // stack - 0x108
  long v18; // stack - 0x100
  long v19; // stack - 0xf0
  long v2;
  long v20; // stack - 0xe8
  long v21; // stack - 0xe0
  int v3;
  int v4;
  long *v5;
  long v6; // stack - 0x138
  stat v7; // stack - 0xd8
  long *v8; // stack - 0x140
  long v9; // stack - 0x118
  
  if (a2) { // branch-flip
    v6 = *a2;
    v13 = a2[1];
    v14 = a2[2];
    v15 = a2[3];
    v5 = &v6;
    v8 = v5;
    v3 = sub_e9e0(v5);
    if (v3 <= -1)
      return -1;
  }
  else {
    v8 = NULL;
    v3 = 0;
    v5 = NULL;
  }
  if ((a0 <= -1) && (!a1)) {
    *__errno_location() = 9;
    return -1;
  }
  if (0 <= dat_e0eb8) {
    if (v3 == 2) {
      if (0 <= a0) // branch-flip
        v3 = fstat(a0,&v7);
      else {
        v3 = stat(a1,&v7);
      }
      if (v3)
        return -1;
      if (v5[1] != 0x3ffffffe) { // branch-flip
        v3 = 3;
        if (v5[3] == 0x3ffffffe) {
          v12 = sub_e940(&v7);
          v5[2] = SUB168(v12,0);
          v5[3] = SUB168(v12,8);
        }
      }
      else {
        v3 = 3;
        v12 = sub_e920(&v7);
        *v5 = SUB168(v12,0);
        v5[1] = SUB168(v12,8);
      }
    }
    if (0 <= a0) // branch-flip
      v4 = futimens(a0,v5);
    else {
      v4 = utimensat(-100,a1,v5,0);
    }
    if (1 <= v4) // branch-flip
      *__errno_location() = 0x26;
    else if ((!v4) || (*__errno_location() != 0x26)) {
      dat_e0eb8 = 1;
      return v4;
    }
  }
  dat_e0eb8 = 0xffffffff;
  dat_e0eb4 = 0xffffffff;
  if (v3) {
    if (v3 != 3) {
      if (0 <= a0) // branch-flip
        v3 = fstat(a0,&v7);
      else {
        v3 = stat(a1,&v7);
      }
      if (v3)
        return -1;
    }
    if ((v5) && (sub_ea90(&v7,&v8)))
      return 0;
  }
  v5 = NULL;
  if (v8) {
    v9 = *v8;
    v17 = v8[2];
    v16 = v8[1] / 1000;
    v5 = &v9;
    v18 = v8[3] / 1000;
  }
  if (a0 < 0)
    return futimesat(-100,a1,v5);
  if (futimesat(a0,NULL,v5)) {
    if (!a1)
      return -1;
    return utimes(a1,v5);
  }
  if (!v5)
    return 0;
  v1 = v5[1];
  v2 = v5[3];
  if ((v1 <= 499999) && (v2 < 500000))
    return 0;
  if (fstat(a0,&v7))
    return 0;
  v10 = *v5;
  v19 = v5[1];
  v20 = v5[2];
  v21 = v5[3];
  v11 = v7._88_8_ - v5[2] != 1;
  if (((v7._72_8_ - *v5 != 1) || (v1 <= 499999)) || (sub_e8e0(&v7))) {
    if (v2 <= 499999 || v11)
      return 0;
    if (sub_e900(&v7))
      return 0;
  }
  else {
    v19 = 0;
    if ((v2 <= 499999 || v11) || (sub_e900(&v7))) {
      futimesat(a0,NULL,&v10); // return-dupe
      return 0;
    }
  }
  v21 = 0;
  futimesat(a0,NULL,&v10);
  return 0;
}

// Function: sub_ef70 @ 0xef70
int sub_ef70(char *a0,long *a1)
{
  return sub_eb50(0xffffffff,a0,a1); // tail-call
}

// Function: sub_ef90 @ 0xef90
int sub_ef90(char *a0,unsigned long *a1)
{
  int v1; // eax
  long v10; // stack - 0xd0
  int v2; // eax
  unsigned long *v3;
  unsigned long v4; // stack - 0xe8
  stat v5; // stack - 0xc8
  unsigned long *v6; // stack - 0xf0
  char v7 [16];
  long v8; // stack - 0xe0
  unsigned long v9; // stack - 0xd8
  
  if (a1) { // branch-flip
    v4 = *a1;
    v8 = a1[1];
    v9 = a1[2];
    v10 = a1[3];
    v3 = &v4;
    v6 = v3;
    v1 = sub_e9e0(v3);
    if (v1 <= -1)
      return -1;
    if (dat_e0eb4 > -1) {
      if (v1 == 2) {
        if (lstat(a0,&v5))
          return -1;
        if (v8 != 0x3ffffffe) { // branch-flip
          v1 = 3;
          if (v10 == 0x3ffffffe) {
            v7 = sub_e940(&v5);
            v10 = SUB168(v7,8);
            v9 = SUB168(v7,0);
          }
        }
        else {
          v1 = 3;
          v7 = sub_e920(&v5);
          v8 = SUB168(v7,8);
          v4 = SUB168(v7,0);
        }
      }
label_f050:
      v2 = utimensat(-100,a0,v3,0x100);
      if (1 <= v2) // branch-flip
        *__errno_location() = 0x26;
      else if ((!v2) || (*__errno_location() != 0x26)) {
        dat_e0eb4 = 1;
        dat_e0eb8 = 1;
        return v2;
      }
    }
    dat_e0eb4 = 0xffffffff;
    if (v1) {
      if ((v1 != 3) && (lstat(a0,&v5)))
        return -1;
      if ((v3) && (sub_ea90(&v5,&v6)))
        return 0;
      goto label_f0bb;
    }
  }
  else {
    v1 = 0;
    v6 = NULL;
    v3 = NULL;
    if (0 <= dat_e0eb4) goto label_f050;
  }
  dat_e0eb4 = 0xffffffff;
  if (lstat(a0,&v5))
    return -1;
label_f0bb:
  if ((v5._24_4_ & 0xf000) != 0xa000)
    return sub_eb50(0xffffffff,a0,v6);
  *__errno_location() = 0x26;
  return -1;
}

// Function: sub_f220 @ 0xf220
long sub_f220(long a0)
{
  if (a0)
    return a0;
  sub_d550(); // no-return
}

// Function: sub_f230 @ 0xf230
void sub_f230(unsigned long a0)
{
  sub_f220(malloc(a0)); // tail-call
}

// Function: sub_f250 @ 0xf250
void sub_f250(void)
{
  sub_f220(sub_10aa0()); // tail-call
}

// Function: sub_f270 @ 0xf270
void sub_f270(void)
{
  sub_f230(); // tail-call
}

// Function: sub_f280 @ 0xf280
void sub_f280(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_d550(); // no-return
}

// Function: sub_f2c0 @ 0xf2c0
void sub_f2c0(void)
{
  sub_f220(sub_10ab0()); // tail-call
}

// Function: sub_f2e0 @ 0xf2e0
void sub_f2e0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_d550(); // no-return
}

// Function: sub_f320 @ 0xf320
void sub_f320(void *a0,unsigned long a1,unsigned long a2)
{
  sub_f2e0(a0,a1,a2); // tail-call
}

// Function: sub_f330 @ 0xf330
void sub_f330(void)
{
  sub_f220(sub_10ae0()); // tail-call
}

// Function: sub_f350 @ 0xf350
void sub_f350(unsigned long a0,unsigned long a1)
{
  sub_f2e0(NULL,a0,a1); // tail-call
}

// Function: sub_f360 @ 0xf360
void sub_f360(unsigned long a0,unsigned long a1)
{
  sub_f330(0,a0,a1); // tail-call
}

// Function: sub_f370 @ 0xf370
void sub_f370(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_d550(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_f2e0(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_f3e0 @ 0xf3e0
void sub_f3e0(void *a0,unsigned long *a1)
{
  sub_f370(a0,a1,1); // tail-call
}

// Function: sub_f3f0 @ 0xf3f0
void sub_f3f0(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_f496;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_f496:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_d550(a0,v3); // no-return
  sub_f280(a0,v3);
  *a1 = v2;
}

// Function: sub_f4d0 @ 0xf4d0
void sub_f4d0(unsigned long a0,unsigned long a1)
{
  sub_f220(calloc(a0,a1)); // tail-call
}

// Function: sub_f4f0 @ 0xf4f0
void sub_f4f0(unsigned long a0)
{
  sub_f4d0(a0,1); // tail-call
}

// Function: sub_f500 @ 0xf500
void sub_f500(void)
{
  sub_f220(sub_10ad0()); // tail-call
}

// Function: sub_f520 @ 0xf520
void sub_f520(unsigned long a0)
{
  sub_f500(a0,1); // tail-call
}

// Function: sub_f530 @ 0xf530
void sub_f530(char *a0,unsigned long a1)
{
  memcpy((void *)sub_f230(a1),a0,a1); // tail-call
}

// Function: sub_f560 @ 0xf560
void sub_f560(void *a0,unsigned long a1)
{
  memcpy((void *)sub_f250(a1),a0,a1); // tail-call
}

// Function: sub_f590 @ 0xf590
void sub_f590(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_f250(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_f5d0 @ 0xf5d0
void sub_f5d0(char *a0)
{
  sub_f530(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_f5f0 @ 0xf5f0
unsigned long sub_f5f0(void)
{
  unsigned int v1; // eax
  unsigned int v2; // edx
  unsigned long v3; // r12
  
  v1 = getchar();
  v2 = v1 & 0xffffffdf;
  if (v1 != 10) {
    while (v1 != 0xffffffff) {
      v1 = getchar();
      if (v1 == 10) break;
    }
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),v2 == 0x59) & 0xffffffff;
}

// Function: sub_f630 @ 0xf630
unsigned long sub_f630(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,int *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  unsigned long v1;
  char *v10; // stack - 0x8c0
  unsigned long v11; // stack - 0xc8
  unsigned long v12; // stack - 0xc0
  unsigned long v13; // stack - 0xb8
  unsigned long v14; // stack - 0xb0
  unsigned long v15; // stack - 0xa8
  unsigned long v16; // stack - 0x98
  unsigned long v17; // stack - 0x88
  unsigned long v18; // stack - 0x78
  unsigned long v19; // stack - 0x68
  char v2; // al
  unsigned long v20; // stack - 0x58
  unsigned long v21; // stack - 0x48
  unsigned long v22; // stack - 0x38
  char *v3; // rax
  char v4 [2008];
  unsigned int v5; // stack - 0x8d0
  unsigned long v6; // stack - 0x8d8
  char v7 [16];
  unsigned int v8; // stack - 0x8cc
  char *v9; // stack - 0x8c8
  
  if (v2) {
    v15 = a0;
    v16 = a1;
    v17 = a2;
    v18 = a3;
    v19 = a4;
    v20 = a5;
    v21 = a6;
    v22 = a7;
  }
  v9 = &Stack0000000000000008;
  v6 = 2000;
  v10 = v7;
  v5 = 0x10;
  v8 = 0x30;
  v11 = a10;
  v12 = a11;
  v13 = a12;
  v14 = a13;
  v3 = (char *)sub_f880(v4,&v6,a9,&v5);
  v1 = v6;
  if (v3) {
    if (fwrite(v3,1,v6,a8) < v1) {
      if (v3 == v4)
        return 0xffffffff;
      free(v3);
      return 0xffffffff;
    }
    if (v3 != v4)
      free(v3);
    if (v1 <= 0x7fffffff)
      return v1 & 0xffffffff;
    *__errno_location() = 0x4b;
  }
  sub_10a30(a8);
  return 0xffffffff;
}

// Function: sub_f7b0 @ 0xf7b0
void sub_f7b0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,int *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  v5 = &Stack0000000000000008;
  v2 = 8;
  v6 = v3;
  v4 = 0x30;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  sub_108a0(stdout,a8,&v2);
}

// Function: sub_f880 @ 0xf880
void * sub_f880(void *a0,unsigned long *a1,int *a2,struct_0 *a3)
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
  if ((int)sub_10dd0(a2,&v18,v16) <= -1)
    return (void *)0;
  if ((int)sub_10ba0(a3,v16) < 0) {
    if (v44 != v19)
      free(v44);
    if (v43 != v20)
      free(v43);
    *__errno_location() = 0x16;
    return NULL;
  }
  v9 = sub_10b70(7,v45,v46,6);
  if (4000 <= v9) { // branch-flip
    if (v9 != 0xffffffffffffffff) {
      v38 = malloc(v9);
      if (v38) {
        v34 = v38;
        v23 = v17;
        goto label_f941;
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
label_f941:
    v9 = 0;
    if (v42)
      v9 = *v35;
    v27 = 0;
    v37 = 0;
    v12 = (int *)*v44;
    v30 = v44;
    v15 = v42;
    if (v12 == a2) goto label_fa3a;
    while( true ) {
      *(unsigned long *)&v23[-8] = 0xf9a4;
      v40 = a2;
      v41 = (long)v12 - (long)a2;
      v10 = sub_10b30(v27,(long)v12 - (long)a2);
      v12 = v40;
      v14 = v41;
      v11 = v15;
      if (v10 <= v9) goto label_fa2b;
      if (v9) break;
      v9 = 0xc;
label_f9cc:
      if (v9 < v10)
        v9 = v10;
      if (v9 == 0xffffffffffffffff) goto label_fb98;
      if ((!v15) || (v15 == v42)) {
        v39 = v40;
        v40 = (int *)CONCAT71(v40._1_7_,v15 == v42);
        *(unsigned long *)&v23[-8] = 0x1038c;
        v11 = malloc(v9);
        if (v11) {
          if ((v27) && ((char)v40)) {
            v40 = v39;
            *(unsigned long *)&v23[-8] = 0x103da;
            v11 = memcpy(v11,v15,v27);
            v12 = v40;
            v14 = v41;
          }
          else {
            v12 = v39;
            v14 = v41;
          }
          goto label_fa2b;
        }
        goto label_fb98;
      }
      *(unsigned long *)&v23[-8] = 0xfa11;
      v11 = realloc(v15,v9);
      if (!v11) {
label_1044e:
        *(unsigned long *)&v23[-8] = 0x10453;
        v40 = __errno_location();
        goto label_10460;
      }
      v12 = v40;
      v14 = v41;
label_fa2b:
      *(unsigned long *)&v23[-8] = 0xfa3a;
      memcpy((void *)((long)v11 + v27),v12,v14);
      v27 = v10;
      v15 = v11;
label_fa3a:
      do {
        if (v18 == v37) {
          *(unsigned long *)&v23[-8] = 0x10765;
          v14 = sub_10b30(v27,1);
          v11 = v15;
          if (v9 < v14) {
            if (v9) { // branch-flip
              v10 = v9 * 2;
              if ((long)v9 < 0) goto label_fb98;
            }
            else {
              v10 = 0xc;
            }
            v9 = v10;
            if (v10 <= v14)
              v9 = v14;
            if (v9 == 0xffffffffffffffff) goto label_fb98;
            v31 = v15 == v42;
            if ((!v15) || (v31)) {
              *(unsigned long *)&v23[-8] = 0x10845;
              v11 = malloc(v9);
              if (!v11) goto label_fb98;
              if ((v27) && (v31)) {
                *(unsigned long *)&v23[-8] = 0x10869;
                v11 = memcpy(v11,v15,v27);
              }
            }
            else {
              *(unsigned long *)&v23[-8] = 0x107b9;
              v11 = realloc(v15,v9);
              if (!v11) goto label_1044e;
            }
          }
          *(char *)((long)v11 + v27) = 0;
          if (v11 != v42) {
            if (v27 + 1 < v9) {
              *(unsigned long *)&v23[-8] = 0x10879;
              v15 = realloc(v11,v27 + 1);
              if (v15)
                v11 = v15;
            }
          }
          if (v34) {
            *(unsigned long *)&v23[-8] = 0x107f4;
            free(v34);
          }
          if (v44 != v19) {
            *(unsigned long *)&v23[-8] = 0x1080c;
            free(v44);
          }
          if (v43 != v20) {
            *(unsigned long *)&v23[-8] = 0x10824;
            free(v43);
          }
          *v35 = v27;
          return v11;
        }
        v5 = v30[10];
        if (*(char *)&v30[9] != '%') { // branch-flip
          if (v5 == -1) {
label_10885:
            *(unsigned long *)&v23[-8] = 0x1088a; // no-return
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
              *(unsigned long *)&v23[-8] = 0xfb25;
              v40 = v12;
              memcpy(v29,v11,(unsigned long)v12);
              v29 = &v29[(long)v40];
              v12 = v39;
            }
            v11 = (void *)v30[6];
            if (v11 != (void *)v30[7]) {
              v13 = (int *)((long)(void *)v30[7] - (long)v11);
              *(unsigned long *)&v23[-8] = 0xfb5f;
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
                goto label_fca1;
              case 7:
              case 8:
              case 0xe:
              case 0x10:
label_fca1:
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
              if (v12[v30[5] * 8] != 5) goto label_10885;
              v39 = (int *)CONCAT44(v39._4_4_,1);
              v21[0] = (&v12[v30[5] * 8])[4];
            }
            else {
              v39 = (int *)((unsigned long)v39._4_4_ << 0x20);
            }
            if (v30[8] != -1) {
              if (v12[v30[8] * 8] != 5) goto label_10885;
              v21[(unsigned long)v39 & 0xffffffff] = (&v12[v30[8] * 8])[4];
              v39 = (int *)CONCAT44(v39._4_4_,(int)v39 + 1);
            }
            *(unsigned long *)&v23[-8] = 0xfd2a;
            v14 = sub_10b30(v27,2);
            v11 = v15;
            if (v9 < v14) {
              if (v9) { // branch-flip
                if ((long)v9 < 0) goto label_fb98;
                v9 *= 2;
              }
              else {
                v9 = 0xc;
              }
              if (v9 <= v14)
                v9 = v14;
              if (v9 == 0xffffffffffffffff) goto label_fb98;
              if ((!v15) || (v15 == v42)) {
                v40 = (int *)CONCAT71(v40._1_7_,v15 == v42);
                *(unsigned long *)&v23[-8] = 0x105ae;
                v11 = malloc(v9);
                if (!v11) goto label_fb98;
                if ((v27) && ((char)v40)) {
                  *(unsigned long *)&v23[-8] = 0x105e0;
                  v11 = memcpy(v11,v15,v27);
                }
              }
              else {
                *(unsigned long *)&v23[-8] = 0xfd74;
                v11 = realloc(v15,v9);
                if (!v11) goto label_fb98;
              }
            }
            *(char *)((long)v11 + v27) = 0;
            *(unsigned long *)&v23[-8] = 0xfd8a;
            v40 = __errno_location();
            v33 = *v40;
            v15 = v11;
            v36 = v29;
label_fda0:
            v22 = -1;
            v10 = v9 - v27;
            *v40 = 0;
            v14 = 0x7fffffff;
            if (v10 <= 0x7fffffff)
              v14 = v10;
            switch(v41 & 0xffffffff) {
              default:
                goto label_10885;
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
                  goto label_1014a;
                }
                if ((int)v39 != 2) {
                  *(int **)&v23[-0x10] = &v22;
label_fe2b:
                  *(unsigned long *)&v23[-0x18] = 0xfe4c;
                  v32 = v14;
                  v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v12);
                  v14 = v32;
                  goto label_fe55;
                }
                *(int **)&v23[-0x10] = &v22;
                *(int **)&v23[-0x18] = v12;
                goto label_1019e;
              case 0xb:
                v1 = *(unsigned long *)&v43[v30[10] * 8 + 4];
                if ((int)v39 != 1) { // branch-flip
                  if ((int)v39 != 2) {
                    *(unsigned long *)&v23[-8] = 0x104ab;
                    v32 = v14;
                    v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v1,&v22);
                    v14 = v32;
                    goto label_fe55;
                  }
                  *(int **)&v23[-8] = &v22;
                  *(unsigned long *)&v23[-0x10] = (unsigned long)v21[1];
                }
                else {
                  *(int **)&v23[-0x10] = &v22;
                }
                *(unsigned long *)&v23[-0x18] = 0x10102;
                v32 = v14;
                v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v1,(unsigned long)v21[0]);
                v14 = v32;
                goto label_fe55;
              case 0xc:
                v2 = *(void *)&v43[v30[10] * 8 + 4];
                if ((int)v39 != 1) {
                  if ((int)v39 != 2) {
                    v12 = &v22;
                    *(void *)&v23[-0x10] = v2;
                    goto label_fe2b;
                  }
                  *(int **)&v23[-0x10] = &v22;
                  *(void *)&v23[-0x20] = v2;
                  *(unsigned long *)&v23[-0x30] = (unsigned long)v21[1];
                  *(unsigned long *)&v23[-0x38] = 0x10503;
                  v32 = v14;
                  v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,(unsigned long)v21[0]);
                  v14 = v32;
                  goto label_fe55;
                }
                *(int **)&v23[-0x10] = &v22;
                *(void *)&v23[-0x20] = v2;
                goto label_101a5;
              
            }
            if ((int)v39 == 1) {
              *(int **)&v23[-8] = &v22;
              *(unsigned long *)&v23[-0x10] = v28;
label_1014a:
              v28 = (unsigned long)v21[0];
label_ff6b:
              *(unsigned long *)&v23[-0x18] = 0xff8c;
              v32 = v14;
              v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,v28);
              v14 = v32;
              if (0 <= v22) goto label_fe63;
label_ffa3:
              if (!v36[1]) {
                if (v8 <= -1) {
                  if (!*v40) {
                    v8 = 0x16;
                    if ((*(unsigned char *)&v30[9] & 0xef) == 99)
                      v8 = 0x54;
                    *v40 = v8;
                  }
                  if ((v15 != v42) && (v15)) {
                    *(unsigned long *)&v23[-8] = 0xffff;
                    free(v15);
                  }
                  if (v34) {
                    *(unsigned long *)&v23[-8] = 0x10013;
                    free(v34);
                  }
                  if (v44 != v19) {
                    *(unsigned long *)&v23[-8] = 0x1002b;
                    free(v44);
                  }
                  if (v43 != v20) {
                    *(unsigned long *)&v23[-8] = 0x10047;
                    free(v43);
                    return (void *)0;
                  }
                  return (void *)0;
                }
                goto label_fe7d;
              }
              v36[1] = '\0';
              goto label_fda0;
            }
            if ((int)v39 != 2) {
              *(int **)&v23[-0x10] = &v22;
              goto label_ff6b;
            }
            *(int **)&v23[-0x10] = &v22;
            *(unsigned long *)&v23[-0x18] = v28;
label_1019e:
            *(unsigned long *)&v23[-0x20] = (unsigned long)v21[1];
label_101a5:
            *(unsigned long *)&v23[-0x28] = 0x101cd;
            v32 = v14;
            v8 = __snprintf_chk((char *)((long)v15 + v27),v14,1,0xffffffffffffffff,v38,(unsigned long)v21[0]);
            v14 = v32;
label_fe55:
            if (v22 <= -1) goto label_ffa3;
label_fe63:
            if (((unsigned long)(long)v22 < v14) && (*(char *)((long)v15 + v27 + (long)v22))) goto label_10885;
            v7 = v22;
            if (v22 < v8) {
label_fe7d:
              v7 = v8;
              v22 = v8;
            }
            if (v14 <= v7 + 1) {
              if (0x7fffffff <= v10) {
                if ((v15 != v42) && (v15)) {
                  *(unsigned long *)&v23[-8] = 0x106ce;
                  free(v15);
                }
                if (v34) {
                  *(unsigned long *)&v23[-8] = 0x106e2;
                  free(v34);
                }
                if (v44 != v19) {
                  *(unsigned long *)&v23[-8] = 0x106fa;
                  free(v44);
                }
                if (v43 != v20) {
                  *(unsigned long *)&v23[-8] = 0x10712;
                  free(v43);
                }
                *v40 = 0x4b;
                return NULL;
              }
              v14 = v9 * 2;
              if ((long)v9 <= -1)
                v14 = 0xffffffffffffffff;
              *(unsigned long *)&v23[-8] = 0xfebb;
              v10 = sub_10b30(v27,v7 + 2);
              *(unsigned long *)&v23[-8] = 0xfec6;
              v10 = sub_10b90(v10,v14);
              if (v9 < v10) {
                if (!v9)
                  v14 = 0xc;
                v9 = v10;
                if (v10 <= v14)
                  v9 = v14;
                if (v9 == 0xffffffffffffffff) goto label_fba4;
                v31 = v15 == v42;
                if ((!v15) || (v31)) {
                  *(unsigned long *)&v23[-8] = 0x10248;
                  v11 = malloc(v9);
                  if (!v11) goto label_fba4;
                  if ((!v27) || (!v31))
                    v15 = v11;
                  else {
                    *(unsigned long *)&v23[-8] = 0x10274;
                    v15 = memcpy(v11,v15,v27);
                  }
                }
                else {
                  *(unsigned long *)&v23[-8] = 0xff17;
                  v11 = realloc(v15,v9);
                  if (!v11) goto label_fba4;
                  v15 = v11;
                }
              }
              goto label_fda0;
            }
            v27 = (long)v7 + v27;
            *v40 = v33;
          }
          else {
            if (5 <= (unsigned int)(v8 - 0x12U)) goto label_10885;
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
          if (v5 != -1) goto label_10885;
          *(unsigned long *)&v23[-8] = 0x102d7;
          v14 = sub_10b30(v27,1);
          if (v9 < v14) {
            if (v9) { // branch-flip
              if ((long)v9 < 0) goto label_fb98;
              v9 *= 2;
            }
            else {
              v9 = 0xc;
            }
            if (v9 < v14)
              v9 = v14;
            if (v9 == 0xffffffffffffffff) goto label_fb98;
            if ((!v15) || (v15 == v42)) {
              v41 = CONCAT71(v41._1_7_,v15 == v42);
              *(unsigned long *)&v23[-8] = 0x105fe;
              v11 = malloc(v9);
              if (!v11) goto label_fb98;
              if ((v27) && ((char)v41)) {
                *(unsigned long *)&v23[-8] = 0x10630;
                v15 = memcpy(v11,v15,v27);
              }
              else {
                v15 = v11;
              }
            }
            else {
              *(unsigned long *)&v23[-8] = 0x10324;
              v11 = realloc(v15,v9);
              if (!v11) goto label_1044e;
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
      goto label_f9cc;
    }
label_fb98:
    *(unsigned long *)&v23[-8] = 0xfb9d;
    v40 = __errno_location();
label_fba4:
    if ((v15 != v42) && (v15)) {
label_10460:
      *(unsigned long *)&v23[-8] = 0x10468;
      free(v15);
    }
    if (v34) {
      *(unsigned long *)&v23[-8] = 0xfbca;
      free(v34);
    }
  }
  if (v44 != v19) {
    *(unsigned long *)&v23[-8] = 0xfbe2;
    free(v44);
  }
  if (v43 != v20) {
    *(unsigned long *)&v23[-8] = 0xfbfa;
    free(v43);
  }
  *v40 = 0xc;
  return NULL;
}

// Function: sub_108a0 @ 0x108a0
unsigned long sub_108a0(FILE *a0,int *a1,struct_0 *a2)
{
  unsigned long v1;
  char *v2; // rax
  char v3 [2008];
  unsigned long v4; // stack - 0x810
  
  v4 = 2000;
  v2 = (char *)sub_f880(v3,&v4,a1,a2);
  v1 = v4;
  if (v2) {
    if (fwrite(v2,1,v4,a0) < v1) {
      if (v2 == v3)
        return 0xffffffff;
      free(v2);
      return 0xffffffff;
    }
    if (v2 != v3)
      free(v2);
    if (v1 <= 0x7fffffff)
      return v1 & 0xffffffff;
    *__errno_location() = 0x4b;
  }
  sub_10a30(a0);
  return 0xffffffff;
}

// Function: sub_10990 @ 0x10990
DIR * sub_10990(char *a0)
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
      v2 = sub_e270(v1,0x406,3);
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

// Function: sub_10a30 @ 0x10a30
void sub_10a30(unsigned int *a0)
{
  *a0 = *a0 | 0x20;
}

// Function: sub_10a40 @ 0x10a40
void sub_10a40(timespec *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_10a50 @ 0x10a50
undefined16 sub_10a50(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_10a40(v1);
  return v1._0_16_;
}

// Function: sub_10aa0 @ 0x10aa0
void sub_10aa0(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_10ab0 @ 0x10ab0
void sub_10ab0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_10ad0 @ 0x10ad0
void sub_10ad0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_10ae0 @ 0x10ae0
void sub_10ae0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_10b10 @ 0x10b10
void sub_10b10(unsigned long a0)
{
  sub_e270(a0,0,3); // tail-call
}

// Function: sub_10b30 @ 0x10b30
long sub_10b30(unsigned long a0,unsigned long a1) // early-return
{
  if (CARRY8(a0,a1))
    return -1;
  return a0 + a1;
}

// Function: sub_10b50 @ 0x10b50
void sub_10b50(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  
  sub_10b30(a0,a1);
  sub_10b30(v1,v2); // tail-call
}

// Function: sub_10b70 @ 0x10b70
void sub_10b70(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  
  sub_10b30(a0,a1);
  sub_10b30(sub_10b30(v1,v2),a3); // tail-call
}

// Function: sub_10b90 @ 0x10b90
unsigned long sub_10b90(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = a1;
  if (a1 <= a0)
    v1 = a0;
  return v1;
}

// Function: sub_10ba0 @ 0x10ba0
unsigned long sub_10ba0(struct_0 *a0,struct_1 *a1) // return-dupe x2
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

// Function: sub_10dd0 @ 0x10dd0
unsigned long sub_10dd0(char *a0,unsigned long *a1,struct_2 *a2) // return-dupe x2
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
  struct_2 *v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22;
  unsigned long v23;
  unsigned long v24; // stack - 0x78
  unsigned long v25; // stack - 0x70
  unsigned long v26; // stack - 0x60
  int v3;
  unsigned long *v4;
  struct_2 *v5; // rax
  unsigned long v6;
  struct_2 *v7;
  struct_2 *v8;
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
label_10e54:
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
label_10efe:
      v9 = &v16[1];
      if ((char)v19 == '\'') goto label_10fe1;
      do {
        switch(v19 - 0x20 & 0xff) {
          case 0:
            v19 = (unsigned int)v4[2] | 8;
            v16 = v9;
            break;
          default:
            goto label_10f28;
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
label_10fe1:
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
    if (*v15 != '$') goto label_10efe;
    v23 = 0;
    while( true ) {
      v6 = 0xffffffffffffffff;
      if (v23 <= 0x1999999999999999)
        v6 = v23 * 10;
      v23 = sub_10b30(v6,(long)(char)(v19 - 0x30),v19 - 0x30);
      v19 = (unsigned int)(unsigned char)v16[1];
      v9 = &v16[1];
      if (10 <= (unsigned char)(v16[1] - 0x30U)) break;
      v16 = v9;
    }
    v23 -= 1;
    if (v23 <= 0xfffffffffffffffd) {
      v19 = (unsigned int)(unsigned char)v16[2];
      v16 = &v16[2];
      goto label_10efe;
    }
    goto label_113e0;
  }
  goto label_10e51;
label_10f28:
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
label_1145c:
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
label_1116d:
      v4[5] = v26;
      v14 = v26 + 1;
      if (v26 == 0xffffffffffffffff) goto label_113e0;
      v10 = v26;
      v26 = v14;
    }
    else {
      v16 = v9;
      do {
        v15 = &v16[1];
        v16 = &v16[1];
      } while ((unsigned char)(*v15 - 0x30U) <= 9);
      if (*v15 != '$') goto label_1116d;
      v10 = 0;
      while( true ) {
        v14 = 0xffffffffffffffff;
        if (v10 <= 0x1999999999999999)
          v14 = v10 * 10;
        v10 = sub_10b30(v14,(long)(char)(v17 + '\xd0'));
        v17 = v9[1];
        v16 = &v9[1];
        if (10 <= (unsigned char)(v17 - 0x30U)) break;
        v9 = v16;
      }
      v10 -= 1;
      if (0xfffffffffffffffe <= v10) goto label_113e0;
      v4[5] = v10;
      v9 = &v9[2];
    }
    if (v20 <= v10) { // branch-flip
      v20 *= 2;
      if (v20 <= v10)
        v20 = sub_10b30(v10,1);
      v7 = (struct_2 *)a2->field_0x8;
      if (0x800000000000000 <= v20) goto label_116ed;
      if (v2 != v7) // branch-flip
        v8 = realloc(v7,v20 << 5);
      else {
        v8 = malloc(v20 << 5);
      }
      v7 = (struct_2 *)a2->field_0x8;
      if (!v8) goto label_116ed;
      if (v2 == v7)
        v8 = memcpy(v8,v2,a2->field_0x0 << 5);
      a2->field_0x8 = &v8->field_0x0;
    }
    else {
      v8 = (struct_2 *)a2->field_0x8;
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
      if (v11 != 5) goto label_113e4;
      goto label_1145c;
    }
    *(unsigned int *)&v8[v10 * 2].field_0x0 = 5;
    v17 = *v9;
    v19 = (unsigned int)(unsigned char)v17;
  }
  v16 = v9;
  if (v17 != '.') goto label_10f47;
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
    goto label_10f47;
  }
  v16 = &v9[2];
  v4[6] = (unsigned long)v9;
  v4[7] = (unsigned long)v16;
  v6 = 2;
  if (2 <= v25)
    v6 = v25;
  v17 = v9[2];
  if (10 <= (unsigned char)(v17 - 0x30U)) { // branch-flip
label_112b5:
    v10 = v4[8];
    if (v4[8] == 0xffffffffffffffff) {
      v4[8] = v26;
      if (v26 == 0xffffffffffffffff) goto label_113e0;
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
    if (*v15 != '$') goto label_112b5;
    v10 = 0;
    while( true ) {
      v14 = 0xffffffffffffffff;
      if (v10 <= 0x1999999999999999)
        v14 = v10 * 10;
      v10 = sub_10b30(v14,(long)(char)(v17 + '\xd0'));
      v17 = v16[1];
      v9 = &v16[1];
      if (10 <= (unsigned char)(v17 - 0x30U)) break;
      v16 = v9;
    }
    v10 -= 1;
    if (0xfffffffffffffffe <= v10) goto label_113e0;
    v4[8] = v10;
    v16 = &v16[2];
  }
  if (v20 <= v10) { // branch-flip
    v20 *= 2;
    if (v20 <= v10)
      v20 = sub_10b30(v10,1);
    v7 = (struct_2 *)a2->field_0x8;
    if (0x800000000000000 <= v20) goto label_116ed;
    if (v2 != v7) // branch-flip
      v8 = realloc(v7,v20 << 5);
    else {
      v8 = malloc(v20 << 5);
    }
    v7 = (struct_2 *)a2->field_0x8;
    if (!v8) goto label_116ed;
    if (v2 == v7)
      v8 = memcpy(v8,v2,a2->field_0x0 << 5);
    a2->field_0x8 = &v8->field_0x0;
  }
  else {
    v8 = (struct_2 *)a2->field_0x8;
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
label_10f47:
    v13 = 0;
    v16 = &v16[1];
    v12 = 0;
    if ((char)v19 == 'h') goto label_10fb0;
    do {
      v18 = (char)v19;
      switch(v18) {
        case 0x4c:
          v13 = v12 | 4;
          break;
        default:
          goto label_10f78;
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
label_10fb0:
        v13 |= 1 << ((unsigned char)v13 & 1);
      }
    } while( true );
  }
  if (v11 == 5) {
    v19 = (unsigned int)(unsigned char)*v16;
    goto label_10f47;
  }
  goto label_113e4;
label_10f78:
  switch(v18) {
    case 0x25:
      goto label_11096;
    default:
      goto label_113e0;
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
label_113e0:
      v8 = (struct_2 *)a2->field_0x8;
      goto label_113e4;
    }
  }
  v26 = v6;
  if (v20 <= v23) { // branch-flip
    v20 *= 2;
    if (v20 <= v23)
      v20 = sub_10b30(v23,1);
    v7 = (struct_2 *)a2->field_0x8;
    if (0x800000000000000 <= v20) goto label_116ed;
    if (v2 != v7) // branch-flip
      v8 = realloc(v7,v20 << 5);
    else {
      v8 = malloc(v20 << 5);
    }
    v7 = (struct_2 *)a2->field_0x8;
    if (!v8) goto label_116ed;
    if (v2 == v7)
      v8 = memcpy(v8,v2,a2->field_0x0 << 5);
    a2->field_0x8 = &v8->field_0x0;
  }
  else {
    v8 = (struct_2 *)a2->field_0x8;
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
label_113e4:
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
label_11096:
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
        goto label_10e51;
      }
    }
    v7 = (struct_2 *)a2->field_0x8;
label_116ed:
    if (v2 != v7)
      free(v7);
    if (v1 != (unsigned long *)a1[1])
      free((unsigned long *)a1[1]);
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  v4 = (unsigned long *)a1[1];
label_10e51:
  a0 = v16;
  v22 = v21;
  goto label_10e54;
}

// Function: _DT_FINI @ 0x11b58
void _DT_FINI(void)
{
  return;
}
