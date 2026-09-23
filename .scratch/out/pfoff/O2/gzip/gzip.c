// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_17fd0)
    return;
  (*dat_17fd0)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_17d60)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x34e0
void __cxa_finalize(void)
{
  (*dat_17fe0)(); // jump-as-call
}

// Function: __strcat_chk @ 0x34f0
char * __strcat_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17d68)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x3500
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17d70)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x3510
int sigprocmask(int a0,sigset_t *a1,sigset_t *a2)
{
  int v1; // eax
  
  v1 = (*dat_17d78)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x3520
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_17d80)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x3530
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17d88)(); // jump-as-call
  return v1;
}

// Function: free @ 0x3540
void free(void *a0)
{
  (*dat_17d90)(); // jump-as-call
}

// Function: utimensat @ 0x3550
int utimensat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_17d98)(); // jump-as-call
  return v1;
}

// Function: localtime @ 0x3560
tm * localtime(void *a0)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_17da0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x3570
void abort(void)
{
  (*dat_17da8)(); // jump-as-call
}

// Function: __errno_location @ 0x3580
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_17db0)(); // jump-as-call
  return v1;
}

// Function: fdatasync @ 0x3590
int fdatasync(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17db8)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x35a0
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_17dc0)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x35b0
void _exit(int a0)
{
  (*dat_17dc8)(); // jump-as-call
}

// Function: strcpy @ 0x35c0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17dd0)(); // jump-as-call
  return v1;
}

// Function: unlinkat @ 0x35d0
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_17dd8)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x35e0
void qsort(void)
{
  (*dat_17de0)(); // jump-as-call
}

// Function: isatty @ 0x35f0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17de8)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3600
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_17df0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3610
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17df8)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3620
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_17e00)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x3630
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_17e08)(); // jump-as-call
  return v1;
}

// Function: write @ 0x3640
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_17e10)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3650
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17e18)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x3660
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_17e20)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x3670
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17e28)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x3680
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17e30)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x3690
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_17e38)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x36a0
void __stack_chk_fail(void)
{
  (*dat_17e40)(); // jump-as-call
}

// Function: getopt_long @ 0x36b0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_17e48)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x36c0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17e50)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x36d0
void lseek(void)
{
  (*dat_17e58)(); // jump-as-call
}

// Function: memset @ 0x36e0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17e60)(); // jump-as-call
  return v1;
}

// Function: close @ 0x36f0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17e68)(); // jump-as-call
  return v1;
}

// Function: strspn @ 0x3700
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17e70)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x3710
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_17e78)(); // jump-as-call
  return v1;
}

// Function: strcspn @ 0x3720
unsigned long strcspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17e80)(); // jump-as-call
  return v1;
}

// Function: read @ 0x3730
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_17e88)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x3740
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_17e90)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3750
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_17e98)(); // jump-as-call
  return v1;
}

// Function: utimes @ 0x3760
int utimes(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_17ea0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3770
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17ea8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3780
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_17eb0)(); // jump-as-call
  return v1;
}

// Function: putc @ 0x3790
int putc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_17eb8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x37a0
void signal(void)
{
  (*dat_17ec0)(); // jump-as-call
}

// Function: dirfd @ 0x37b0
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_17ec8)(); // jump-as-call
  return v1;
}

// Function: __memcpy_chk @ 0x37c0
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17ed0)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x37d0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_17ed8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x37e0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_17ee0)(); // jump-as-call
  return v1;
}

// Function: strtol @ 0x37f0
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_17ee8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3800
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17ef0)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x3810
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17ef8)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x3820
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_17f00)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3830
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17f08)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3840
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17f10)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3850
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17f18)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3860
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17f20)(); // jump-as-call
  return v1;
}

// Function: __strcpy_chk @ 0x3870
char * __strcpy_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17f28)(); // jump-as-call
  return v1;
}

// Function: fchmod @ 0x3880
int fchmod(int a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = (*dat_17f30)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3890
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17f38)(); // jump-as-call
  return v1;
}

// Function: fsync @ 0x38a0
int fsync(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17f40)(); // jump-as-call
  return v1;
}

// Function: open @ 0x38b0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_17f48)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x38c0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_17f50)(); // jump-as-call
  return v1;
}

// Function: fchown @ 0x38d0
int fchown(int a0,unsigned int a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = (*dat_17f58)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x38e0
void perror(char *a0)
{
  (*dat_17f60)(); // jump-as-call
}

// Function: fdopendir @ 0x38f0
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_17f68)(); // jump-as-call
  return v1;
}

// Function: futimens @ 0x3900
int futimens(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_17f70)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x3910
int sigismember(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_17f78)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3920
void exit(int a0)
{
  (*dat_17f80)(); // jump-as-call
}

// Function: fwrite @ 0x3930
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17f88)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x3940
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_17f90)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x3950
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17f98)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3960
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_17fa0)(); // jump-as-call
  return v1;
}

// Function: futimesat @ 0x3970
int futimesat(int a0,char *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_17fa8)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x3980
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17fb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3990
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17fb8)(); // jump-as-call
  return v1;
}

// Function: main @ 0x39a0
int main(int argc,char **argv,char **envp)
{
  unsigned int *v1;
  char **v10; // stack - 0xe0
  int v11; // stack - 0xe8
  int v12; // stack - 0xe4
  void *v13; // stack - 0xd8
  unsigned int v14 [32];
  unsigned int *v15;
  unsigned int *v16;
  char **v17;
  int *v18; // r15
  long v19; // r15
  unsigned int *v2;
  long v20; // fs_offset
  undefined4 v21; // stack - 0x50
  unsigned long v22; // stack - 0x40
  int v3;
  int v4;
  unsigned int v5;
  int v6; // eax
  char *v7;
  unsigned long v8; // rax
  long v9; // rcx
  
  v22 = *(unsigned long *)(v20 + 0x28);
  v7 = (char *)sub_d290(*argv);
  dat_1a850 = v7;
  v8 = strlen(v7);
  if (5 <= v8) {
    if (!strcmp(&v7[v8 - 4],".exe"))
      v7[v8 - 4] = '\0';
  }
  v10 = argv;
  dat_dc9d0 = sub_d2b0(&v11,&v10,"GZIP");
  v17 = NULL;
  if (dat_dc9d0)
    v17 = v10;
  dat_dc9c8 = ".gz";
  dat_dc9c0 = strlen(".gz");
  v12 = -1;
  if (v17) { // branch-flip
    v7 = v17[optind];
    if ((!v7) || (strcmp(v7,"--"))) {
      v3 = getopt_long(v11,v17,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",(option *)0x177c0,&v12);
      v4 = v3 + 0x83;
      if (v3 < 0) {
        if (optind != v11) {
          sub_f6b0(stderr,"%s: %s: non-option in GZIP environment variable\n",dat_1a850,v17[optind]);
          sub_69d0(); // no-return
        }
        if ((optind != 1) && (!dat_1a85c))
          sub_f6b0(stderr,"%s: warning: GZIP environment variable is deprecated; use an alias or script\n",dat_1a850);
        free(v17);
        v12 = -1;
        optind = 1;
        goto label_3b65;
      }
      v5 = v3 + 0x52;
      goto label_3b44;
    }
    if (!v17) goto label_3b65;
    v4 = 0xb0;
  }
  else {
label_3b65:
    v4 = getopt_long(argc,argv,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",(option *)0x177c0,&v12);
    v3 = optind;
    if (v4 < 0) {
      if (dat_18028 <= -1)
        dat_18028 = dat_dc9f8;
      if (dat_1802c <= -1)
        dat_1802c = dat_dc9f8;
      if (dat_dc9fc) { // branch-flip
        if (!dat_1a85c) {
          sub_f6b0(stderr,"%s: option --ascii ignored on this system\n",dat_1a850);
          goto label_411e;
        }
        if (0x1e <= dat_dc9c0 - 1) {
          sub_f6b0(stderr,"%s: invalid suffix \'%s\'\n",dat_1a850,dat_dc9c8); // return-dupe
          sub_6960(1); // no-return
        }
label_3ff1:
        dat_dc920 = 0xd;
      }
      else {
label_411e:
        if (0x1e <= dat_dc9c0 - 1) {
          sub_f6b0(stderr,"%s: invalid suffix \'%s\'\n",dat_1a850,dat_dc9c8);
          sub_6960(1);
        }
        dat_dc920 = 0;
        if (dat_1a85c) goto label_3ff1;
      }
      v18 = (int *)0x13350;
      sigemptyset((sigset_t *)0xdc940);
      do {
        v4 = *v18;
        sigaction(v4,NULL,(sigaction *)&v13);
        if (v13 != (void *)0x1)
          sigaddset((sigset_t *)0xdc940,v4);
        v18 = &v18[1];
      } while (v18 != (int *)0x13368);
      v19 = 0;
      v9 = 0x20;
      v15 = (unsigned int *)0xdc940;
      v16 = v14;
      while (v9) {
        v2 = &v16[1];
        v1 = &v15[1];
        *v16 = *v15;
        v9 -= 1;
        v15 = v1;
        v16 = v2;
      }
      v21 = 0;
      v13 = sub_5600;
      do {
        v4 = *(int *)(v19 * 4 + 0x13350);
        if (sigismember((void *)0xdc940,v4)) {
          if (!v19)
            dat_dc9e0 = 1;
          sigaction(v4,(sigaction *)&v13,NULL);
        }
        v19 += 1;
      } while (v19 != 6);
      if (argc != v3) { // branch-flip
        while (optind < argc) {
          v7 = argv[optind];
          optind += 1;
          v6 = strcmp(v7,"-");
          v4 = dat_1a860;
          if (v6) // branch-flip
            sub_6e90(v7);
          else {
            sub_6a80();
            dat_1a860 = v4;
          }
        }
      }
      else {
        sub_6a80();
      }
      if ((dat_dc510) && (close(0))) {
        dat_1a420 = 0x69647473;
        dat_1a424 = 0x6e;
        sub_d4e0(); // no-return
      }
      if (dat_dc9e8) {
        if ((!dat_1a85c) && (2 <= argc - v3))
          sub_65d0(0xffffffff);
        v4 = sub_e3e0(stdout);
        if (v4) {
          sub_d550(); // no-return, return-dupe
        }
      }
      if (((!dat_1a860) || ((((!dat_dca00 || (!fdatasync(1))) || (*__errno_location() == 0x16)) && (!close(1))))) || (*__errno_location() == 9))
        sub_6960(dat_1a84c); // no-return
      sub_d550();
    }
    v5 = v4 - 0x31;
label_3b44:
    if (v5 <= 0xd3) {
      v4 = *(int *)((unsigned long)v5 * 4 + 0x12f80);
      return (*(void *)((long)v4 + 0x12f80))(); // jump-as-call
    }
    if ((v4 <= 0x82) || (v4 == 0xc2)) {
      sub_69d0(); // no-return, return-dupe
    }
  }
  sub_f6b0(stderr,"%s: ",dat_1a850);
  if (0 <= v12) // branch-flip
    sub_f6b0(stderr,"--%s: ",*(unsigned long *)((long)v12 * 0x20 + 0x177c0));
  else {
    sub_f6b0(stderr,"-%c: ",v4 + -0x83);
  }
  sub_f6b0(stderr,"option not valid in GZIP environment variable\n");
  sub_69d0();
}

// Function: sub_4290 @ 0x4290
void sub_4290(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_17fc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_42c0 @ 0x42c0
void sub_42c0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4330
void _FINI_0(void)
{
  if (!dat_19048) {
    if (dat_17fe0)
      __cxa_finalize(dat_18008);
    sub_42c0();
    dat_19048 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4370
void _INIT_0(void)
{
  return;
}

// Function: sub_4380 @ 0x4380
void sub_4380(int a0) // return-dupe
{
  dat_19058 = 0;
  dat_1905c = 0;
  if (a0 == -1)
    return;
  dat_19050 = sub_df80;
  dat_19058 = 0;
  dat_1905c = 0;
}

// Function: sub_43b0 @ 0x43b0
void sub_43b0(unsigned int a0,int a1)
{
  int v1;
  unsigned short v2;
  unsigned int v3; // esi
  unsigned long v4; // rsi
  unsigned long v5; // r8
  
  dat_1905c = (unsigned short)(a0 << ((unsigned char)dat_19058 & 0x1f)) | dat_1905c;
  v1 = dat_19058;
  if (0x10U - a1 < dat_19058) {
    v3 = dat_1a004 + 1;
    v4 = (unsigned long)v3;
    if (0x3fffe <= dat_1a004) { // branch-flip
      *(char *)((unsigned long)dat_1a004 + 0x5b000) = (char)dat_1905c;
      v2 = dat_1905c;
      if (v3 == 0x40000) {
        dat_1a004 = v3;
        sub_d600();
        v4 = (unsigned long)dat_1a004;
        v2 = dat_1905c;
      }
      dat_1a004 = (int)v4 + 1;
      *(char *)(v4 + 0x5b000) = (char)(v2 >> 8);
      if (dat_1a004 == 0x40000)
        sub_d600();
    }
    else {
      v5 = (unsigned long)dat_1a004;
      dat_1a004 += 2;
      *(char *)(v5 + 0x5b000) = (char)dat_1905c;
      *(char *)(v4 + 0x5b000) = (char)(dat_1905c >> 8);
    }
    v1 = dat_19058 + -0x10;
    dat_1905c = (unsigned short)((int)(a0 & 0xffff) >> (0x10U - (char)dat_19058 & 0x1fU));
  }
  dat_19058 = a1 + v1;
}

// Function: sub_44a0 @ 0x44a0
unsigned int sub_44a0(unsigned int a0,int a1)
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

// Function: sub_44d0 @ 0x44d0
void sub_44d0(void) // return-dupe x4
{
  unsigned long v1; // rax
  char v2; // cl
  unsigned int v3; // edx
  
  v2 = (char)dat_1905c; // branch-flip
  if (9 <= dat_19058) {
    v3 = dat_1a004 + 1;
    v1 = (unsigned long)v3;
    if (dat_1a004 <= 0x3fffd) {
      *(char *)((unsigned long)dat_1a004 + 0x5b000) = v2;
      *(char *)(v1 + 0x5b000) = (char)((unsigned short)dat_1905c >> 8);
      dat_1a004 += 2;
      dat_19058 = 0;
      dat_1905c = 0;
      return;
    }
    *(char *)((unsigned long)dat_1a004 + 0x5b000) = v2;
    if (v3 == 0x40000) {
      dat_1a004 = v3;
      sub_d600();
      v1 = (unsigned long)dat_1a004;
    }
    dat_1a004 = (int)v1 + 1;
    *(char *)(v1 + 0x5b000) = (char)((unsigned short)dat_1905c >> 8);
    if (dat_1a004 != 0x40000) {
      dat_19058 = 0;
      dat_1905c = 0;
      return;
    }
  }
  else {
    if (dat_19058 <= 0) {
      dat_19058 = 0;
      dat_1905c = 0;
      return;
    }
    v1 = (unsigned long)dat_1a004;
    dat_1a004 += 1;
    *(char *)(v1 + 0x5b000) = v2;
    if (dat_1a004 != 0x40000) {
      dat_19058 = 0;
      dat_1905c = 0;
      return;
    }
  }
  sub_d600();
  dat_19058 = 0;
  dat_1905c = 0;
}

// Function: sub_45c0 @ 0x45c0
void sub_45c0(char *a0,unsigned int a1,int a2) // return-dupe x2
{
  char v1;
  unsigned char v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned int v5;
  char *v6; // r12
  unsigned char v7; // r13b
  
  sub_44d0();
  if (a2) {
    v2 = (unsigned char)(a1 >> 8);
    v5 = dat_1a004 + 1;
    v4 = (unsigned long)v5;
    v7 = (unsigned char)a1; // branch-flip
    if (0x3fffe <= dat_1a004) {
      *(unsigned char *)((unsigned long)dat_1a004 + 0x5b000) = v7;
      if (v5 == 0x40000) {
        dat_1a004 = v5;
        sub_d600();
        v4 = (unsigned long)dat_1a004;
      }
      dat_1a004 = (int)v4 + 1;
      v3 = (unsigned long)dat_1a004;
      *(unsigned char *)(v4 + 0x5b000) = v2;
      if (dat_1a004 == 0x40000) {
        sub_d600();
        v3 = (unsigned long)dat_1a004;
      }
    }
    else {
      v3 = (unsigned long)(dat_1a004 + 2);
      *(unsigned char *)((unsigned long)dat_1a004 + 0x5b000) = v7;
      *(unsigned char *)(v4 + 0x5b000) = v2;
    }
    v5 = (unsigned int)v3;
    dat_1a004 = v5 + 1;
    v4 = (unsigned long)dat_1a004;
    if (0x3fffe <= v5) { // branch-flip
      *(unsigned char *)(v3 + 0x5b000) = ~v7;
      if (dat_1a004 == 0x40000) {
        sub_d600();
        v4 = (unsigned long)dat_1a004;
      }
      dat_1a004 = (int)v4 + 1;
      *(unsigned char *)(v4 + 0x5b000) = ~v2;
      if (dat_1a004 == 0x40000)
        sub_d600();
    }
    else {
      dat_1a004 = v5 + 2;
      *(unsigned char *)(v3 + 0x5b000) = ~v7;
      *(unsigned char *)(v4 + 0x5b000) = ~v2;
    }
  }
  v6 = &a0[a1];
  if (!a1)
    return;
  do {
    while( true ) {
      v4 = (unsigned long)dat_1a004;
      v1 = *a0;
      a0 = &a0[1];
      dat_1a004 += 1;
      *(char *)(v4 + 0x5b000) = v1;
      if (dat_1a004 == 0x40000) break;
      if (a0 == v6)
        return;
    }
    sub_d600();
  } while (a0 != v6);
}

// Function: sub_4740 @ 0x4740
unsigned int sub_4740(unsigned int a0)
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
  
  v5 = (unsigned long)dat_1906c;
  v13 = dat_1906c;
  if (dat_1906c <= 0x7ef9)
    v13 = 0x7efa;
  v11 = (long)(int)dat_19070;
  v1 = (char *)(v5 + 0x3b000);
  v9 = *(char *)(v5 + 0x3afff + v11);
  v15 = v1[v11];
  v8 = dat_19064;
  if (dat_19060 <= dat_19070)
    v8 = dat_19064 >> 2;
  v4 = 0;
  v14 = dat_19070;
  do {
    v6 = (unsigned long)a0;
    if ((((((char *)(v6 + 0x3b000))[v11] != v15) || (*(char *)(v6 + 0x3afff + v11) != v9)) || (*(char *)(v6 + 0x3b000) != *v1)) || (*(char *)(v6 + 0x3b001) != *(char *)(v5 + 0x3b001))) {
label_47b8:
      v2 = *(unsigned short *)((unsigned long)(a0 & 0x7fff) * 2 + 0x1a880);
      v3 = v4;
    }
    else {
      v12 = (char *)(v5 + 0x3b002);
      v7 = (char *)(v6 + 0x3b002);
      while( true ) {
        if (v12[1] != v7[1]) {
          v12 = &v12[1];
          goto label_4890;
        }
        if (v12[2] != v7[2]) break;
        if (v12[3] != v7[3]) {
          v12 = &v12[3];
          goto label_4890;
        }
        if (v12[4] != v7[4]) {
          v12 = &v12[4];
          goto label_4890;
        }
        if (v12[5] != v7[5]) {
          v12 = &v12[5];
          goto label_4890;
        }
        if (v12[6] != v7[6]) {
          v12 = &v12[6];
          goto label_4890;
        }
        if (v12[7] != v7[7]) {
          v12 = &v12[7];
          goto label_4890;
        }
        v12 = &v12[8];
        v7 = &v7[8];
        if ((*v12 != *v7) || ((char *)(v5 + 0x3b102) <= v12)) goto label_4890;
      }
      v12 = &v12[2];
label_4890:
      v10 = 0x102 - ((int)(char *)(v5 + 0x3b102) - (int)v12);
      if (v10 <= v14) goto label_47b8;
      if (dat_19080 <= v10) {
        dat_19068 = a0;
        return v10;
      }
      v2 = *(unsigned short *)((unsigned long)(a0 & 0x7fff) * 2 + 0x1a880);
      v9 = *(char *)(v5 + 0x3afff + (long)v10);
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
      dat_19068 = v16;
      return v14;
    }
    v11 = (long)v14;
  } while( true );
}

// Function: sub_4960 @ 0x4960
void sub_4960(unsigned int a0,unsigned int a1) // return-dupe
{
  unsigned char *v1;
  unsigned char v2;
  bool v3;
  unsigned long v4; // rax
  unsigned char *v5;
  unsigned long v6;
  
  v4 = (unsigned long)a0;
  if (a0 <= 0xfff) {
    v5 = (unsigned char *)((unsigned long)a0 + 0x3b000);
    v1 = &v5[a1];
    v3 = 0;
    v6 = dat_19090;
    do {
      if (v1 == v5) {
        if (!v3)
          return;
        dat_19090 = v6;
        return;
      }
      v2 = *v5;
      v5 = &v5[1];
      v6 += v2;
      v3 = 1;
    } while ((unsigned char *)((unsigned long)a0 + 0x3b001 + (unsigned long)(0xfff - a0)) != v5);
    v4 = 0x1000;
    dat_19090 = v6;
  }
  if (a0 + a1 <= (unsigned int)v4)
    return;
  v3 = 0;
  v6 = dat_19088;
  do {
    dat_19090 += (unsigned long)*(unsigned char *)(v4 + 0x3b000) - (unsigned long)*(unsigned char *)((unsigned long)(unsigned int)((int)v4 - 0x1000) + 0x3b000);
    if ((v6 == 0xffffffff) && (!(dat_19090 & 0xfff))) {
      v3 = 1;
      v6 = v4;
    }
    v4 += 1;
  } while ((unsigned int)v4 < a0 + a1);
  if (!v3)
    return;
  dat_19088 = v6;
}

// Function: sub_4a60 @ 0x4a60
void sub_4a60(void) // return-dupe
{
  short *v1;
  short v2;
  short *v3; // rax
  unsigned long v4; // rbx
  unsigned int v5; // eax
  int v6; // esi
  unsigned int v7; // r12d
  
  v4 = (unsigned long)dat_1909c;
  v5 = dat_1909c + dat_1906c;
  v6 = 0x10000 - v5;
  if (v6 != -1) { // branch-flip
    if (0xfefa <= dat_1906c) {
      v7 = dat_1906c - 0x8000;
      memcpy((void *)0x3b000,(void *)0x43000,0x8000);
      dat_19068 -= 0x8000;
      if (dat_19088 != 0xffffffff)
        dat_19088 -= 0x8000;
      dat_19078 -= 0x8000;
      v1 = (short *)0x2a880;
      do {
        v2 = *v1 + -0x8000;
        if (0 <= *v1)
          v2 = 0;
        v3 = &v1[1];
        *v1 = v2;
        v1 = v3;
      } while (v3 != (short *)0x3a880);
      v1 = (short *)0x1a880;
      do {
        v2 = *v1 + -0x8000;
        if (0 <= *v1)
          v2 = 0;
        v3 = &v1[1];
        *v1 = v2;
        v1 = v3;
      } while (v3 != (short *)0x2a880);
      v6 = 0x18000 - v5;
      dat_1906c = v7;
    }
  }
  else {
    v6 = -2;
  }
  if (dat_190a0)
    return;
  v5 = (*dat_19050)(dat_1906c + v4 + 0x3b000,v6);
  if ((unsigned int)(v5 - 1U) <= 0xfffffffd) {
    dat_1909c += v5;
    return;
  }
  dat_190a0 = 1;
  *(unsigned short *)((unsigned long)dat_1906c + (unsigned long)dat_1909c + 0x3b000) = 0;
}

// Function: sub_4bc0 @ 0x4bc0
void sub_4bc0(int a0)
{
  unsigned short *v1;
  unsigned long v10; // rax
  int v11;
  unsigned int v12;
  long v13; // rdx
  char *v14;
  int v15;
  unsigned long v16; // rax
  int v17;
  short *v2;
  unsigned short v3;
  short v4;
  unsigned int v5;
  unsigned int v6; // eax
  unsigned int v7; // eax
  unsigned int v8;
  unsigned int v9;
  
  if (8 < (unsigned int)(a0 - 1U))
    sub_d3f0("bad pack level"); // no-return
  memset((void *)0x2a880,0,0x10000);
  v13 = (long)a0;
  dat_19088 = 0xffffffff;
  dat_19090 = 0;
  dat_1906c = 0;
  dat_19098 = (unsigned int)*(unsigned short *)(v13 * 8 + 0x12022);
  dat_19078 = 0;
  dat_19060 = (unsigned int)*(unsigned short *)(v13 * 8 + 0x12020);
  dat_19080 = (unsigned int)*(unsigned short *)(v13 * 8 + 0x12024);
  dat_19064 = (unsigned int)*(unsigned short *)(v13 * 8 + 0x12026);
  dat_1909c = (*dat_19050)(0x3b000,0x10000);
  if (0xfffffffe <= dat_1909c - 1) { // branch-flip
    dat_190a0 = 1;
    dat_1909c = 0;
    if (a0 <= 3) goto label_5136;
  }
  else {
    dat_190a0 = 0;
    if (dat_1909c <= 0x105) {
      do {
        sub_4a60();
        if (0x106 <= dat_1909c) break;
      } while (!dat_190a0);
    }
    dat_190a4 = (unsigned int)dat_3b000 << 5 ^ (unsigned int)dat_3b001;
    if (a0 <= 3) {
label_5136:
      dat_19070 = 2;
      if (dat_1909c) {
        v12 = dat_1909c;
label_5160:
        do {
          v16 = (unsigned long)dat_1906c;
          dat_190a4 = (dat_190a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(dat_1906c + 2) + 0x3b000)) & 0x7fff;
          v1 = (unsigned short *)((unsigned long)(dat_190a4 + 0x8000) * 2 + 0x1a880);
          v3 = *v1;
          *(unsigned short *)((unsigned long)(dat_1906c & 0x7fff) * 2 + 0x1a880) = v3;
          *v1 = (unsigned short)dat_1906c;
          if (((v3) && (dat_1906c - v3 <= 0x7efa)) && (dat_1906c <= 0xfefa)) {
            v8 = sub_4740(v3);
            if (v8 <= v12)
              v12 = v8;
            if (v12 <= 2) {
              v16 = (unsigned long)dat_1906c;
              goto label_51ba;
            }
            v15 = sub_b160(dat_1906c - dat_19068,v12 - 3);
            v17 = dat_1a000;
            v8 = dat_1906c;
            dat_1909c -= v12;
            if (dat_1a000) { // branch-flip
              sub_4960(dat_1906c,v12);
              if (dat_19098 < v12) goto label_5338;
label_538f:
              v9 = v8;
              do {
                v6 = v9 + 1;
                dat_190a4 = (dat_190a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(v9 + 3) + 0x3b000)) & 0x7fff;
                v2 = (short *)((unsigned long)(dat_190a4 + 0x8000) * 2 + 0x1a880);
                v4 = *v2;
                *(short *)((unsigned long)(v6 & 0x7fff) * 2 + 0x1a880) = v4;
                *v2 = (short)v6;
                v9 = v6;
              } while (v6 != (v12 - 1) + v8);
              dat_1906c = v8 + v12;
              *(short *)((unsigned long)(v6 & 0x7fff) * 2 + 0x1a880) = v4;
              *v2 = (short)v8 + -1 + (short)v12;
            }
            else {
              if (v12 <= dat_19098) goto label_538f;
label_5338:
              dat_1906c = v12 + v8;
              dat_190a4 = (unsigned int)*(unsigned char *)((unsigned long)dat_1906c + 0x3b000) << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(dat_1906c + 1) + 0x3b000);
            }
          }
          else {
label_51ba:
            v15 = sub_b160(0,(unsigned char)*(char *)(v16 + 0x3b000));
            v17 = dat_1a000;
            v12 = dat_1906c;
            if (dat_1a000)
              sub_4960(dat_1906c,1);
            dat_1906c = v12 + 1;
            dat_1909c -= 1;
          }
          if ((v17) && (dat_19088 < dat_1906c)) {
            v17 = 1;
            v15 = 2;
            dat_19088 = 0xffffffff;
label_5225:
            v14 = (char *)((dat_19078 & 0xffffffff) + 0x3b000);
            if ((long)dat_19078 <= -1)
              v14 = NULL;
            sub_add0(v14,dat_1906c - dat_19078,v17,0);
            dat_19078 = (unsigned long)dat_1906c;
            v12 = dat_1909c;
          }
          else {
            v12 = dat_1909c;
            if (v15) {
              v17 = v15 + -1;
              goto label_5225;
            }
          }
          while (dat_1909c = v12, v12 <= 0x105) {
            if (dat_190a0) {
              if (!v12) {
                v15 -= 1;
                dat_1909c = 0;
                goto label_529c;
              }
              goto label_5160;
            }
            sub_4a60();
            v12 = dat_1909c;
          }
        } while( true );
      }
      v15 = -1;
label_529c:
      v16 = dat_1906c - dat_19078;
      v14 = (char *)((dat_19078 & 0xffffffff) + 0x3b000);
      if ((long)dat_19078 <= -1)
        v14 = NULL;
      sub_add0(v14,v16,v15,1); // return-dupe, tail-call
      return;
    }
    if (dat_1909c) {
      v17 = 0;
      v16 = 0;
      v12 = dat_1909c;
      dat_19070 = 2;
      do {
        v15 = dat_19068;
        v10 = (unsigned long)dat_1906c;
        v8 = 2;
        dat_190a4 = ((unsigned int)*(unsigned char *)((unsigned long)(dat_1906c + 2) + 0x3b000) ^ dat_190a4 << 5) & 0x7fff;
        v1 = (unsigned short *)((unsigned long)(dat_190a4 + 0x8000) * 2 + 0x1a880);
        v3 = *v1;
        *(unsigned short *)((unsigned long)(dat_1906c & 0x7fff) * 2 + 0x1a880) = v3;
        *v1 = (unsigned short)dat_1906c;
        v6 = (unsigned int)v3;
        v9 = dat_19070;
        if ((v6) && (dat_19070 < dat_19098)) {
          if ((0x7efb <= dat_1906c - v6) || (0xfefb <= dat_1906c))
            v8 = 2;
          else {
            v9 = sub_4740(v6);
            v8 = v12;
            if (v9 <= v12)
              v8 = v9;
            v9 = dat_19070;
            if (v8 != 3) // branch-flip
              v10 = (unsigned long)dat_1906c;
            else {
              v10 = (unsigned long)dat_1906c;
              if (0x1001 <= dat_1906c - dat_19068)
                v8 = 2;
            }
          }
        }
        v12 = (unsigned int)v10;
        if ((3 <= v9) && (v8 <= v9)) {
          v6 = sub_b160((v12 - v15) + -1,v9 - 3);
          v15 = dat_1a000;
          v9 = dat_19070;
          v8 = dat_1906c;
          v16 = (unsigned long)v6;
          v17 = dat_19070 - 2;
          v12 = (dat_1909c + 1) - dat_19070;
          dat_1909c = v12;
          if (dat_1a000) {
            v11 = dat_19070 - 1;
            dat_19070 = v17;
            sub_4960(dat_1906c,v11);
          }
          v5 = v8;
          do {
            v7 = v5 + 1;
            dat_190a4 = (dat_190a4 << 5 ^ (unsigned int)*(unsigned char *)((unsigned long)(v5 + 3) + 0x3b000)) & 0x7fff;
            v2 = (short *)((unsigned long)(dat_190a4 + 0x8000) * 2 + 0x1a880);
            v4 = *v2;
            *(short *)((unsigned long)(v7 & 0x7fff) * 2 + 0x1a880) = v4;
            *v2 = (short)v7;
            v5 = v7;
          } while (v7 != v17 + v8);
          dat_1906c = (v9 - 1) + v8;
          *(short *)((unsigned long)(v7 & 0x7fff) * 2 + 0x1a880) = v4;
          *v2 = (short)v8 + -2 + (short)v9;
          dat_19070 = 0;
          if ((v15) && (dat_19088 < dat_1906c)) {
            v15 = 1;
            v16 = 2;
            dat_19088 = 0xffffffff;
          }
          else {
            if (!v6) {
              v17 = 0;
              v8 = 2;
              goto label_4e25;
            }
            v15 = v6 - 1;
          }
          v8 = 2;
          v14 = (char *)((dat_19078 & 0xffffffff) + 0x3b000);
          if ((long)dat_19078 <= -1)
            v14 = NULL;
          v17 = 0;
          sub_add0(v14,dat_1906c - dat_19078,v15,0);
          dat_19078 = (unsigned long)dat_1906c;
          v12 = dat_1909c;
        }
        else if (v17) { // branch-flip
          v16 = sub_b160(0,(unsigned char)*(char *)((unsigned long)(v12 - 1) + 0x3b000));
          v15 = (int)v16;
          v16 &= 0xffffffff;
          if (dat_1a000) { // branch-flip
            v10 = (unsigned long)dat_1906c;
            if (v10 <= dat_19088) { // branch-flip
              v11 = v15 + -1;
              if (v15) goto label_4ebc;
            }
            else {
              v16 = 2;
              dat_19088 = 0xffffffff;
              v11 = v17;
label_4ebc:
              v14 = (char *)((dat_19078 & 0xffffffff) + 0x3b000);
              if ((long)dat_19078 <= -1)
                v14 = NULL;
              sub_add0(v14,dat_1906c - dat_19078,v11,0);
              v10 = (unsigned long)dat_1906c;
              dat_19078 = v10;
              if (!dat_1a000) goto label_4ef8;
            }
            sub_4960(v10,1);
            dat_1906c = (unsigned int)v10;
          }
          else {
            v11 = v15 + -1;
            if (v15) goto label_4ebc;
          }
label_4ef8:
          dat_1906c += 1;
          dat_1909c -= 1;
          v12 = dat_1909c;
        }
        else {
          if (dat_1a000) {
            if (dat_19088 < v10) {
              v16 = 2;
              dat_19088 = 0xffffffff;
              v14 = (char *)((dat_19078 & 0xffffffff) + 0x3b000);
              if ((long)dat_19078 <= -1)
                v14 = NULL;
              sub_add0(v14,v10 - dat_19078,1,0);
              v10 = (unsigned long)dat_1906c;
              v12 = dat_1906c;
              dat_19078 = v10;
              if (!dat_1a000) goto label_4e00;
            }
            sub_4960(v10,1);
            v12 = (unsigned int)v10;
          }
label_4e00:
          dat_1906c = v12 + 1;
          v17 = 1;
          dat_1909c -= 1;
          v12 = dat_1909c;
        }
label_4e25:
        while (v12 <= 0x105) {
          if (dat_190a0) {
            v12 = dat_1909c;
            if (!dat_1909c) {
              v15 = (int)v16 + -1;
              if (v17)
                sub_b160(0,(unsigned char)*(char *)((unsigned long)(dat_1906c - 1) + 0x3b000));
              goto label_4f79;
            }
            break;
          }
          sub_4a60();
          v12 = dat_1909c;
        }
        dat_19070 = v8;
      } while( true );
    }
  }
  v15 = -1;
label_4f79:
  v16 = dat_1906c - dat_19078;
  v14 = (char *)((dat_19078 & 0xffffffff) + 0x3b000);
  if ((long)dat_19078 <= -1)
    v14 = NULL;
  sub_add0(v14,v16,v15,1);
  return;
}

// Function: sub_54d0 @ 0x54d0
void sub_54d0(void)
{
  long *v1; // rbx
  char *v2;
  
  v1 = (long *)0x17b20;
  sub_f830("%s %s\n",dat_1a850,dat_183f8);
  v2 = "Copyright (C) 2018 Free Software Foundation, Inc.";
  do {
    v1 = &v1[1];
    sub_f830("%s\n",v2);
    v2 = (char *)*v1;
  } while (v2);
}

// Function: sub_5530 @ 0x5530
void sub_5530(bool a0)
{
  char *v1;
  char *v2;
  char v3;
  char *v4;
  char *v5;
  sigset_t v6;
  char v7 [1032];
  int v8;
  
  if (a0) { // branch-flip
    v8 = dat_18018;
    if (dat_18018 <= -1)
      return;
  }
  else {
    sigprocmask(0,(sigset_t *)0xdc940,&v6);
    v8 = dat_18018;
    if (dat_18018 <= -1) {
      sigprocmask(2,&v6,NULL); // return-dupe
      return;
    }
  }
  dat_18018 = -1;
  close(v8);
  v4 = (char *)0xdc520;
  v5 = v7;
  while( true ) {
    v1 = &v4[1];
    v3 = *v4;
    v2 = &v5[1];
    *v5 = v3;
    if (!v3) break;
    v4 = v1;
    v5 = v2;
  }
  sub_d2a0();
  if (a0)
    return;
  sigprocmask(2,&v6,NULL);
  return;
}

// Function: sub_5600 @ 0x5600
void sub_5600(int a0)
{
  sub_5530(1);
  if (dat_dc920 != a0) {
    signal(a0,0);
    raise(a0); // tail-call
    return;
  }
  _exit(2); // no-return
}

// Function: sub_5640 @ 0x5640
void sub_5640(char *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  sub_f6b0(stderr,"%s: ",dat_1a850);
  *v2 = v1;
  perror(a0);
  dat_1a84c = 1;
}

// Function: sub_5690 @ 0x5690
void sub_5690(long a0,unsigned int a1)
{
  char v1;
  unsigned long v2; // rax
  char v3; // stack - 0x21
  
label_56ba:
  if (!a0)
    return;
  if (dat_1a00c <= dat_1a008) goto label_5725;
  do {
    v2 = (unsigned long)dat_1a008;
    dat_1a008 += 1;
    v1 = *(char *)(v2 + 0x9c000);
    v3 = v1;
    while( true ) {
      if (a1 & 2) {
        sub_d0d0(&v3,1);
        v1 = v3;
      }
      v3 = v1;
      if (a0 != -1) {
        a0 -= 1;
        goto label_56ba;
      }
      if (!v3)
        return;
      if (dat_1a008 < dat_1a00c) break;
label_5725:
      v1 = sub_d6c0(0);
      v3 = v1;
    }
  } while( true );
}

// Function: sub_5750 @ 0x5750
unsigned int sub_5750(unsigned int a0) // early-return x9, return-dupe x2
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
  
  v15 = (unsigned long)dat_1a008;
  if ((dat_dc9f4) && (dat_1a860)) {
    if (dat_1a00c <= dat_1a008) { // branch-flip
      v15 = sub_d6c0(1);
      v7 = (unsigned int)v15;
      v6 = dat_1a008;
      v10 = dat_1a00c;
    }
    else {
      dat_1a008 += 1;
      v2 = *(unsigned char *)(v15 + 0x9c000);
      v15 = (unsigned long)v2;
      v7 = (unsigned int)v2;
      v6 = dat_1a008;
      v10 = dat_1a00c;
    }
    v12 = (char)v15;
    if (v10 <= v6) { // branch-flip
      v15 = sub_d6c0(1);
      v6 = (unsigned int)v15;
    }
    else {
      dat_1a008 = v6 + 1;
      v15 = (unsigned long)*(unsigned char *)((unsigned long)v6 + 0x9c000);
      v6 = (unsigned int)*(unsigned char *)((unsigned long)v6 + 0x9c000);
    }
  }
  else {
    if (dat_1a00c <= dat_1a008) { // branch-flip
      v12 = sub_d6c0(0);
      v6 = dat_1a008;
      v7 = dat_1a00c;
    }
    else {
      dat_1a008 += 1;
      v12 = *(char *)(v15 + 0x9c000);
      v6 = dat_1a008;
      v7 = dat_1a00c;
    }
    if (v12) { // branch-flip
      if (v7 <= v6) // branch-flip
        v15 = sub_d6c0(0);
      else {
        dat_1a008 = v6 + 1;
        v15 = (unsigned long)*(unsigned char *)((unsigned long)v6 + 0x9c000);
      }
      v6 = 0;
      v7 = 0;
    }
    else {
      if (v7 <= v6) { // branch-flip
        v15 = sub_d6c0(1);
        v6 = (unsigned int)v15;
      }
      else {
        dat_1a008 = v6 + 1;
        v2 = *(unsigned char *)((unsigned long)v6 + 0x9c000);
        v15 = (unsigned long)v2;
        v6 = (unsigned int)v2;
      }
      v7 = 0;
    }
  }
  v18 = (char)v15;
  dat_18020 = 0xffffffff;
  dat_dfea8 = 0;
  v11 = dat_dc9d8 + 1;
  dat_dc9dc = 0;
  dat_dc9d8 = v11;
  if ((memcmp(&v12,(void *)0x12c6d,2)) && (memcmp(&v12,(void *)0x12c70,2))) {
    if (((memcmp(&v12,(void *)0x12c90,2)) || (dat_1a008 != 2)) || (memcmp((void *)0x9c000,(void *)0x12c90,4))) {
      if (!memcmp(&v12,(void *)0x12c95,2)) {
        dat_180d0 = sub_c590;
        dat_18020 = 2;
        return 2;
      }
      if (!memcmp(&v12,(void *)0x12c98,2)) {
        dat_180d0 = sub_be90;
        dat_18020 = 1;
        dat_dc9dc = 1;
        return 1;
      }
      if (!memcmp(&v12,(void *)0x12c9b,2)) {
        dat_180d0 = sub_b920;
        dat_18020 = 3;
        dat_dc9dc = 1;
        return 3;
      }
      if (((dat_dc9f4) && (dat_1a860)) && (!dat_dc9e8)) {
        dat_18020 = 0;
        dat_180d0 = sub_d770;
        if (v6 != 0xffffffff)
          dat_1a008 -= 1;
        dat_dc9dc = 1;
        if (v7 == 0xffffffff) {
          dat_18020 = 0;
          dat_dc9dc = 1;
          return 0;
        }
        sub_d5a0(1,&v12,1);
        goto label_627a;
      }
    }
    else {
      dat_1a008 = 0;
      dat_180d0 = sub_cc20;
      if (sub_cb20(a0))
        return 0xffffffff;
      dat_dc9dc = 1;
label_627a:
      if (0 <= (int)dat_18020)
        return dat_18020;
      v11 = dat_dc9d8;
    }
    if (v11 != 1) {
label_5c60:
      if (!v12) {
        while (!v6) {
          v15 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) // branch-flip
            v6 = sub_d6c0(1);
          else {
            dat_1a008 += 1;
            v6 = (unsigned int)*(unsigned char *)(v15 + 0x9c000);
          }
        }
        if (v6 == 0xffffffff) {
          if (!dat_dc9e4)
            return 0xfffffffd;
          if (!dat_1a85c)
            sub_f6b0(stderr,"\n%s: %s: decompression OK, trailing zero bytes ignored\n",dat_1a850,0x1a420);
          if (dat_1a84c)
            return 0xfffffffd;
          dat_1a84c = 2;
          return 0xfffffffd;
        }
      }
      if (!dat_1a85c)
        sub_f6b0(stderr,"\n%s: %s: decompression OK, trailing garbage ignored\n",dat_1a850,0x1a420);
      if (!dat_1a84c) {
        dat_1a84c = 2;
        return 0xfffffffe;
      }
      return 0xfffffffe;
    }
  }
  else {
    v15 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) // branch-flip
      dat_18020 = sub_d6c0(0);
    else {
      dat_1a008 += 1;
      dat_18020 = (unsigned int)*(unsigned char *)(v15 + 0x9c000);
    }
    if (dat_18020 != 8) {
      sub_f6b0(stderr,"%s: %s: unknown method %d -- not supported\n",dat_1a850,0x1a420,dat_18020);
      dat_1a84c = 1;
      return 0xffffffff;
    }
    dat_180d0 = sub_cc20;
    v15 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) // branch-flip
      v7 = sub_d6c0(0);
    else {
      dat_1a008 += 1;
      v7 = (unsigned int)*(unsigned char *)(v15 + 0x9c000);
    }
    if (v7 & 0x20) {
      v14 = "%s: %s is encrypted -- not supported\n";
      sub_f6b0(stderr,v14,dat_1a850,0x1a420); // return-dupe
      dat_1a84c = 1;
      return 0xffffffff;
    }
    if (v7 & 0xc0) { // branch-flip
      sub_f6b0(stderr,"%s: %s has flags 0x%x -- not supported\n",dat_1a850,0x1a420,v7 & 0xff);
      dat_1a84c = 1;
      if (dat_dc9f4 <= 1) {
        dat_1a84c = 1;
        return 0xffffffff;
      }
      v10 = dat_1a00c;
      if (dat_1a00c <= dat_1a008) goto label_5d62;
label_5a0f:
      v15 = (unsigned long)dat_1a008;
      dat_1a008 += 1;
      v15 = (unsigned long)*(unsigned char *)(v15 + 0x9c000);
      if (dat_1a008 < v10) goto label_5a2c;
label_5d80:
      v15 |= (long)(int)sub_d6c0(0) << 8;
      v10 = dat_1a00c;
      if (dat_1a00c <= dat_1a008) goto label_5da4;
label_5a4f:
      v9 = (unsigned long)dat_1a008;
      dat_1a008 += 1;
      v15 |= (unsigned long)*(unsigned char *)(v9 + 0x9c000) << 0x10;
      if (dat_1a008 < v10) goto label_5a73;
label_5dc9:
      v9 = (unsigned long)(int)sub_d6c0(0);
    }
    else {
      v10 = dat_1a00c;
      if (dat_1a008 < dat_1a00c) goto label_5a0f;
label_5d62:
      v15 = (unsigned long)(int)sub_d6c0(0);
      v10 = dat_1a00c;
      if (dat_1a00c <= dat_1a008) goto label_5d80;
label_5a2c:
      v9 = (unsigned long)dat_1a008;
      dat_1a008 += 1;
      v15 |= (unsigned long)*(unsigned char *)(v9 + 0x9c000) << 8;
      if (dat_1a008 < v10) goto label_5a4f;
label_5da4:
      v15 |= (long)(int)sub_d6c0(0) << 0x10;
      if (dat_1a00c <= dat_1a008) goto label_5dc9;
label_5a73:
      v9 = (unsigned long)dat_1a008;
      dat_1a008 += 1;
      v9 = (unsigned long)*(unsigned char *)(v9 + 0x9c000);
    }
    v15 |= v9 << 0x18;
    if ((v15) && (!dat_18028)) {
      if (0 <= (long)v15) { // branch-flip
        dat_1a838 = 0;
        dat_1a830 = v15;
      }
      else {
        if (!dat_1a85c)
          sub_f6b0(stderr,"%s: %s: MTIME %lu out of range for this platform\n",dat_1a850,0x1a420,v15);
        if (!dat_1a84c)
          dat_1a84c = 2;
        dat_1a838 = 999999999;
        dat_1a830 = 0x7fffffffffffffff;
      }
    }
    v9 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) { // branch-flip
      v22 = sub_d6c0(0);
      v10 = dat_1a008;
      v5 = dat_1a00c;
    }
    else {
      dat_1a008 += 1;
      v22 = *(char *)(v9 + 0x9c000);
      v10 = dat_1a008;
      v5 = dat_1a00c;
    }
    if (v5 <= v10) // branch-flip
      v23 = sub_d6c0(0);
    else {
      dat_1a008 = v10 + 1;
      v23 = *(char *)((unsigned long)v10 + 0x9c000);
    }
    if (v7 & 2) {
      v19 = 8;
      v21 = (unsigned int)v15;
      v20 = (char)v7;
      sub_d0d0(NULL,0);
      sub_d0d0(&v12,10);
    }
    if (v7 & 4) {
      v15 = (unsigned long)dat_1a008;
      if (dat_1a00c <= dat_1a008) { // branch-flip
        v3 = sub_d6c0(0);
        v10 = dat_1a008;
        v5 = dat_1a00c;
      }
      else {
        dat_1a008 += 1;
        v3 = *(char *)(v15 + 0x9c000);
        v10 = dat_1a008;
        v5 = dat_1a00c;
      }
      v13 = v3; // branch-flip
      if (v5 <= v10)
        v17 = sub_d6c0(0);
      else {
        dat_1a008 = v10 + 1;
        v17 = *(char *)((unsigned long)v10 + 0x9c000);
      }
      v1 = CONCAT11(v17,v3);
      if (dat_dc9e4)
        sub_f6b0(stderr,"%s: %s: extra field of %u bytes ignored\n",dat_1a850,0x1a420,v1);
      if (v7 & 2)
        sub_d0d0(&v13,2);
      sub_5690(v1,v7 & 0xff);
    }
    if (v7 & 8) {
      if ((dat_1802c) || (((dat_1a860 && (!dat_dc9e8)) || (2 <= dat_dc9d8))))
        sub_5690(-1,v7 & 0xff);
      else {
        v8 = (unsigned char *)sub_d290((char *)0x1a020);
        v16 = v8;
        while( true ) {
          v15 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) // branch-flip
            v2 = sub_d6c0(0);
          else {
            dat_1a008 += 1;
            v2 = *(unsigned char *)(v15 + 0x9c000);
          }
          *v16 = v2;
          v16 = &v16[1];
          if (!v2) break;
          if ((unsigned char *)0x1a420 <= v16)
            sub_d3f0("corrupted input -- file name too large"); // no-return
        }
        if (v7 & 2)
          sub_d0d0(v8,(int)v16 - (int)v8);
        v14 = (char *)sub_d290(v8);
        memmove(v8,v14,strlen(v14) + 1);
      }
    }
    if (v7 & 0x10)
      sub_5690(-1,v7 & 0xff);
    if (v7 & 2) {
      v4 = sub_d0d0(&v12,0);
      v15 = (unsigned long)dat_1a008;
      if (dat_1a00c <= dat_1a008) { // branch-flip
        v5 = sub_d6c0(0);
        v7 = dat_1a008;
        v10 = dat_1a00c;
      }
      else {
        dat_1a008 += 1;
        v5 = (unsigned int)*(unsigned char *)(v15 + 0x9c000);
        v7 = dat_1a008;
        v10 = dat_1a00c;
      }
      if (v10 <= v7) // branch-flip
        v7 = sub_d6c0(0);
      else {
        dat_1a008 = v7 + 1;
        v7 = (unsigned int)*(unsigned char *)((unsigned long)v7 + 0x9c000);
      }
      v5 = v7 << 8 | v5;
      if (v4 != v5) {
        sub_f6b0(stderr,"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",dat_1a850,0x1a420,v5,(unsigned int)v4);
        dat_1a84c = 1;
        if (dat_dc9f4 <= 1) {
          dat_1a84c = 1;
          return 0xffffffff;
        }
      }
    }
    if (dat_dc9d8 != 1) {
      if (0 <= (int)dat_18020)
        return dat_18020;
      goto label_5c60;
    }
    dat_dfea8 = (unsigned long)(dat_1a008 + 8);
    if (0 <= (int)dat_18020)
      return dat_18020;
  }
  v14 = "\n%s: %s: not in gzip format\n";
  sub_f6b0(stderr,v14,dat_1a850,0x1a420);
  dat_1a84c = 1;
  return 0xffffffff;
}

// Function: sub_63e0 @ 0x63e0
char * sub_63e0(char *a0)
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
  
  v10 = dat_dc9c8;
  v6 = dat_dc9c0;
  if (dat_18048) {
    v7 = 0x18048;
    v11 = dat_18048;
    do {
      v4 = strlen(v11);
      if ((v6 < v4) && (!strcmp(v10,&v11[v4 - v6]))) {
        v8 = (unsigned long *)0x18048;
        v5 = (unsigned char *)sub_f640(v10);
        sub_d230(v5);
        v7 = 8;
        goto label_6479;
      }
      v11 = *(char **)(v7 + 8);
      v7 += 8;
    } while (v11);
  }
  v5 = (unsigned char *)sub_f640(v10);
  sub_d230(v5);
  v7 = 0;
  v8 = (unsigned long *)0x18040;
label_6479:
  *(unsigned char **)(v7 * 8 + 0x18040) = v5;
  v1 = (int)strlen(a0);
  if (0x21 <= v1) // branch-flip
    __strcpy_chk(v9,&a0[(long)v1 + -0x20],0x21);
  else {
    __strcpy_chk(v9,a0,0x21);
  }
  sub_d230(v9);
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

// Function: sub_65d0 @ 0x65d0
void sub_65d0(int a0)
{
  tm *v1; // rax
  long v2;
  long v3;
  long v4;
  unsigned long v5;
  
  if (dat_18010) { // branch-flip
    if (a0 < 0) {
label_6718:
      if ((1 <= dat_dc508) && (1 <= dat_dc500)) {
        if (((dat_dc9e4) && (sub_f830("                            "), v4 = dat_dc508, dat_dc9e4)) || (v4 = dat_dc508, !dat_1a85c)) {
          sub_d8a0(stdout,v4,0x13);
          sub_f830(" ");
          sub_d8a0(stdout,dat_dc500,0x13);
          sub_f830(" ");
          v4 = dat_dc508;
        }
        sub_d850(dat_dc500 - (v4 - dat_dfea8),dat_dc500,stdout);
        sub_f830(" (totals)\n"); // tail-call
        return;
      }
      return;
    }
    dat_18010 = 0;
    if (dat_dc9e4)
      sub_f830("method  crc     date  time  ");
    if (!dat_1a85c)
      sub_f830("%*.*s %*.*s  ratio uncompressed_name\n",0x13,0x13,"compressed",0x13,0x13,"uncompressed");
  }
  else if (a0 <= -1) goto label_6718;
  v5 = 0xffffffffffffffff;
  if ((a0 == 8) && (!dat_dc9dc))
    v5 = dat_dfe90;
  if (dat_dc9e4) {
    v1 = localtime((void *)0x1a830);
    sub_f830("%5s %08lx ",*(unsigned long *)((long)a0 * 8 + 0x17680),v5);
    if (v1) // branch-flip
      sub_f830("%s%3d %02d:%02d ",&"Jan"[(long)*(int *)&v1->field_0x10 * 4],*(unsigned int *)&v1->field_0xc,*(unsigned int *)&v1->field_0x8,*(unsigned int *)&v1->field_0x4);
    else {
      sub_f830("??? ?? ??:?? ");
    }
  }
  sub_d8a0(stdout,dat_1a828,0x13);
  sub_f830(" ");
  sub_d8a0(stdout,dat_1a820,0x13);
  sub_f830(" ");
  if (dat_1a828 != -1) { // branch-flip
    if (0 <= dat_dc508)
      dat_dc508 += dat_1a828;
    v4 = dat_1a828;
    v2 = dat_1a820;
    v3 = dat_1a820;
    if (dat_1a820 == -1) {
      dat_dc500 = -1;
      v4 = 0;
      v3 = 0;
      dat_dfea8 = 0;
      dat_1a820 = 0;
      dat_1a828 = 0;
      sub_d850(v4,v3,stdout); // return-dupe
      sub_f830(" %s\n",0x1a020); // tail-call
      return;
    }
  }
  else {
    dat_dc508 = -1;
    v3 = 0;
    v2 = 0;
    v4 = 0;
    dat_dfea8 = 0;
    dat_1a820 = 0;
    dat_1a828 = 0;
  }
  if (0 <= dat_dc500)
    dat_dc500 += v2;
  v4 = v3 - (v4 - dat_dfea8);
  sub_d850(v4,v3,stdout);
  sub_f830(" %s\n",0x1a020);
  return;
}

// Function: sub_6960 @ 0x6960
void sub_6960(int a0)
{
  if (!dat_dc040) {
    dat_dc040 = 1;
    free(dat_dc9d0);
    dat_dc9d0 = NULL;
  }
  exit(a0); // no-return
}

// Function: sub_69a0 @ 0x69a0
void sub_69a0(void)
{
  int v1; // eax
  
  v1 = sub_e060(stdout);
  if (v1)
    sub_d550(); // no-return
  sub_6960(0); // no-return
}

// Function: sub_69d0 @ 0x69d0
void sub_69d0(void)
{
  sub_f6b0(stderr,"Try `%s --help\' for more information.\n",dat_1a850);
  sub_6960(1); // no-return
}

// Function: sub_6a00 @ 0x6a00
void sub_6a00(void)
{
  dat_1a840 = 0xffffffffffffffff;
  dat_1a838 = 0xffffffffffffffff;
  if ((dat_dc078 & 0xf000) == 0x8000) {
    dat_1a840 = dat_dc090;
    if ((!dat_18028) || (dat_dc9e8)) {
      dat_1a830 = dat_dc0b8;
      dat_1a838 = dat_dc0c0;
      return;
    }
  }
}

// Function: sub_6a80 @ 0x6a80
void sub_6a80(void)
{
  int v1; // eax
  char *v2; // rcx
  char *v3; // r8
  bool v4; // zf
  
  if ((dat_dc9f4 || dat_dc9e8) || ((!dat_dca01 && (v4 = dat_dc9f8 == 0, !isatty((unsigned int)v4))))) {
    dat_1a420 = 0x69647473;
    dat_1a424 = 0x6e;
    dat_1a020 = 0x6f647473;
    dat_1a024 = 0x7475;
    dat_1a026 = 0;
    if (!fstat(0,(stat *)0xdc060)) {
      sub_6a00();
      sub_d150();
      dat_1a860 = 1;
      dat_dc9d8 = 0;
      dat_1a014 = 0;
      dat_dc510 = 1;
      if ((dat_dc9f8) && (dat_18020 = sub_5750(0), dat_18020 <= -1))
        sub_6960(dat_1a84c); // no-return
      while( true ) {
        v1 = (*dat_180d0)(0,1);
        if (v1)
          return;
        if ((!dat_dc9f8) || (dat_dc9dc)) break;
        if (dat_1a008 == dat_1a00c) {
          if ((dat_1a00c != 0x40000) || (sub_d6c0(1) == -1)) break;
          dat_1a008 = 0;
        }
        dat_18020 = sub_5750(dat_1a014);
        if (dat_18020 < 0)
          return;
        dat_1a820 = 0;
      }
      if (dat_dc9e8) {
        sub_65d0(dat_18020); // tail-call
        return;
      }
      if (!dat_dc9e4)
        return;
      if (dat_1a858) {
        sub_f6b0(stderr," OK\n"); // tail-call
        return;
      }
      if (dat_dc9f8)
        return;
      sub_d850((dat_1a828 - dat_1a820) + dat_dfea8,dat_1a828,stderr);
      sub_f6b0(stderr,"\n"); // tail-call
      return;
    }
    sub_5640("standard input");
  }
  else if (!dat_1a85c) {
    v2 = "read from";
    if (!dat_dc9f8)
      v2 = "written to";
    v3 = "de";
    if (!dat_dc9f8)
      v3 = (char *)0x12c94;
    sub_f6b0(stderr,"%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n",dat_1a850,v2,v3,dat_1a850);
  }
  sub_6960(1); // no-return
}

// Function: sub_6d00 @ 0x6d00
void sub_6d00(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a1) // branch-flip
    v1 = a1;
  else {
    v1 = 1;
    a1 = 1;
    a0 = ".";
  }
  if ((!memcmp((void *)0xdc100,a0,v1)) && (!*(char *)(a1 + 0xdc100)))
    return;
  if (0 <= dat_18014)
    close(dat_18014);
  __memcpy_chk((void *)0xdc100,a0,v1,0x400);
  *(char *)(a1 + 0xdc100) = 0;
  dat_18014 = sub_e360((char *)0xdc100,0x10000);
}

// Function: sub_6dc0 @ 0x6dc0
int sub_6dc0(void)
{
  int v1;
  bool v2;
  bool v3;
  int v4; // eax
  int v5;
  int *v6; // rax
  char *v7; // rax
  
  v2 = dat_1a860 == 0;
  v3 = dat_dc9f4 == 0;
  if (!dat_dc9f0) {
    v7 = (char *)sub_dfd0((char *)0x1a420);
    v5 = sub_6d00((char *)0x1a420,&v7[-0x1a420]);
    if (0 <= v5) goto label_6dfb;
  }
  v7 = (char *)0x1a420;
  v5 = -100;
label_6dfb:
  v4 = sub_e4b0(v5,v7,(-(unsigned int)(v2 && v3) & 0x20000) + 0x900);
  v5 = v4;
  if ((0 <= v4) && (fstat(v4,(stat *)0xdc060))) {
    v6 = __errno_location();
    v5 = -1;
    v1 = *v6;
    close(v4);
    *v6 = v1;
  }
  return v5;
}

// Function: sub_6e90 @ 0x6e90
void sub_6e90(char *a0)
{
  char v1;
  char *v10; // rax
  unsigned long *v11;
  char v12 [1032];
  sigset_t v13; // stack - 0x4c8
  char *v14;
  char *v15;
  unsigned int v16; // r13d
  char *v17;
  char *v18; // stack - 0x4e8
  int v19; // stack - 0x4e0
  bool v2;
  int v3;
  int v4;
  int *v5; // rax
  unsigned long v6; // rax
  DIR *v7; // rax
  unsigned long v8; // rax
  char *v9;
  
  dat_180a0 = dat_dc9c8;
  if (0x3ff <= strlen(a0)) {
    sub_f6b0(stderr,"%s: %s: file name too long\n",dat_1a850,a0); // return-dupe
    dat_1a014 = 0xffffffff;
    dat_1a84c = 1;
    return;
  }
  __strcpy_chk((char *)0x1a420,a0,0x400);
  v3 = sub_6dc0();
  if (v3 <= -1) {
    v5 = __errno_location();
    v3 = *v5;
    if ((!dat_dc9f8) || (v3 != 2)) {
      sub_f6b0(stderr,"%s: ",dat_1a850);
      *v5 = v3;
      perror((char *)0x1a420);
      dat_1a014 = 0xffffffff;
      dat_1a84c = 1;
      return;
    }
    if (sub_63e0((char *)0x1a420)) {
      v3 = *v5;
      sub_f6b0(stderr,"%s: ",dat_1a850);
      *v5 = v3;
      perror((char *)0x1a420);
      dat_1a014 = 0xffffffff;
      dat_1a84c = 1;
      return;
    }
    v6 = strlen((char *)0x1a420);
    v11 = (unsigned long *)0x180a0;
    if (!strcmp(dat_dc9c8,".gz"))
      v11 = (unsigned long *)0x180a8;
    v4 = 0;
    v15 = (char *)*v11;
    while( true ) {
      __strcpy_chk((char *)0x1a420,a0,0x400);
      if (0x400 <= strlen(v15) + (long)(int)v6) break;
      __strcat_chk((char *)0x1a420,v15,0x400);
      v3 = sub_6dc0();
      v9 = dat_dc9c8;
      if (0 <= v3) goto label_705b;
      v3 = *v5;
      if (v3 != 2) {
        sub_f6b0(stderr,"%s: ",dat_1a850);
        *v5 = v3;
        perror((char *)0x1a420);
        dat_1a014 = 0xffffffff;
        dat_1a84c = 1;
        return;
      }
      if (!strcmp(v15,dat_dc9c8))
        v4 = 2;
      v15 = (char *)v11[1];
      v11 = &v11[1];
      if (!v15) {
        __strcpy_chk((char *)0x1a420,a0,0x400);
        __strcat_chk((char *)0x1a420,v9,0x400);
        *v5 = v4;
        sub_f6b0(stderr,"%s: ",dat_1a850);
        *v5 = v4;
        perror((char *)0x1a420);
        dat_1a014 = 0xffffffff;
        dat_1a84c = 1;
        return;
      }
    }
    sub_f6b0(stderr,"%s: %s: file name too long\n",dat_1a850,a0);
    dat_1a014 = 0xffffffff;
    dat_1a84c = 1;
    return;
  }
label_705b:
  dat_1a014 = v3;
  if ((dat_dc078 & 0xf000) != 0x4000) { // branch-flip
    if (dat_1a860) {
      sub_6a00();
      if (dat_1a858) { // branch-flip
label_7353:
        __strcpy_chk((char *)0x1a020,(char *)0x1a420,0x400);
        v15 = (char *)sub_63e0((char *)0x1a020);
        if (dat_dc9f8) { // branch-flip
          if (v15) {
            sub_d230(v15);
            if ((strcmp(v15,".tgz")) && (strcmp(v15,".taz")))
              *v15 = '\0';
            else {
              builtin_strncpy(v15,".tar",5);
            }
            goto label_73d0;
          }
          if (dat_dc9ec) { // branch-flip
            if (!dat_dc9e4) {
              v3 = dat_1a014; // return-dupe
              close(v3); // return-dupe, tail-call
              return;
            }
label_77a9:
            if (dat_1a85c) goto label_77b9;
          }
          else {
            if (dat_1a858) goto label_73d0;
            if (dat_dc9e4) goto label_77a9;
            if (dat_1a85c) {
              v3 = dat_1a014;
              close(v3);
              return;
            }
          }
          sub_f6b0(stderr,"%s: %s: unknown suffix -- ignored\n",dat_1a850,0x1a420);
        }
        else {
          if ((v15) && (!dat_dc9f4)) {
            if ((dat_dc9e4) || (!dat_dc9ec && !dat_1a85c))
              sub_f6b0(stderr,"%s: %s already has %s suffix -- unchanged\n",dat_1a850,0x1a420,v15);
            v3 = dat_1a014;
            close(v3);
            return;
          }
          dat_1a848 = 0;
          if (strlen((char *)0x1a020) + dat_dc9c0 <= 0x3ff) {
            __strcat_chk((char *)0x1a020,dat_dc9c8,0x400);
            goto label_73d0;
          }
          if (!dat_1a85c)
            sub_f6b0(stderr,"%s: %s: file name too long\n",dat_1a850,0x1a420);
        }
label_77b9:
        if (!dat_1a84c)
          dat_1a84c = 2;
      }
      else {
        dat_1a026 = 0;
        dat_1a020 = 0x6f647473;
        dat_1a024 = 0x7475;
label_73d0:
        sub_d150();
        dat_dc9d8 = 0;
        if ((!dat_dc9f8) || (dat_18020 = sub_5750(dat_1a014), 0 <= dat_18020)) {
          if (!dat_1a860) {
            if (!dat_dc9f0) {
              v18 = (char *)sub_dfd0((char *)0x1a020);
              v19 = sub_6d00((char *)0x1a020,&v18[-0x1a020]);
              if (0 <= v19) goto label_78d9;
            }
            v19 = -100;
            v18 = (char *)0x1a020;
label_78d9:
            v5 = __errno_location();
            v2 = 0;
label_78ef:
            v15 = (char *)0x1a020;
            v9 = (char *)0xdc520;
            do {
              while( true ) {
                v17 = &v15[1];
                v1 = *v15;
                v14 = &v9[1];
                *v9 = v1;
                if (!v1) break;
                v15 = v17;
                v9 = v14;
              }
              sigprocmask(0,(sigset_t *)0xdc940,&v13);
              dat_18018 = sub_e4b0(v19,v18,0xc1,0x180);
              v3 = *v5;
              dat_1a010 = dat_18018;
              sigprocmask(2,&v13,NULL);
              if (0 <= dat_1a010) {
                if (v2) {
                  if (dat_dc9f8) {
                    if (dat_1a85c) { // branch-flip
                      if (dat_1a84c) goto label_7405;
                    }
                    else {
                      sub_f6b0(stderr,"%s: %s: warning, name truncated\n",dat_1a850,0x1a020);
                      if (dat_1a84c) goto label_8257;
                    }
                    dat_1a84c = 2;
                    goto label_8257;
                  }
                }
                else {
label_8257:
                  if (dat_dc9f8) goto label_7405;
                }
                if (!dat_1a848) goto label_7415;
                v3 = dat_dc9e4;
                if (dat_dc9e4 || dat_1a85c) goto label_7428;
                sub_f6b0(stderr,"%s: %s compressed to %s\n",dat_1a850,0x1a420,0x1a020);
                goto label_7405;
              }
              if (v3 == 0x11) {
                if (!dat_dc9f4) {
                  sub_f6b0(stderr,"%s: %s already exists;",dat_1a850,0x1a020);
                  if ((dat_dc9e0) && ((dat_dca01 || (isatty(0))))) {
                    sub_f6b0(stderr," do you wish to overwrite (y or n)? ");
                    sub_e3e0(stderr);
                    if (sub_f660()) goto label_7a58;
                  }
                  sub_f6b0(stderr,"\tnot overwritten\n");
                  if (!dat_1a84c)
                    dat_1a84c = 2;
                  close(dat_1a014); // return-dupe
                  return;
                }
label_7a58:
                if (sub_d2a0((char *)0x1a020)) {
label_7a70:
                  v3 = *v5;
                  sub_f6b0(stderr,"%s: ",dat_1a850);
                  *v5 = v3;
                  perror((char *)0x1a020);
                  dat_1a84c = 1;
                  close(dat_1a014);
                  return;
                }
                goto label_78ef;
              }
              if (v3 != 0x24) goto label_7a70;
              v6 = strlen((char *)0x1a020);
              v3 = (int)v6;
              if (!dat_dc9f8) goto label_7f55;
              if (v3 <= 1)
                sub_d3f0("name too short"); // no-return
              *(char *)((long)v3 + 0x1a01f) = 0;
              v2 = 1;
              v15 = (char *)0x1a020;
              v9 = (char *)0xdc520;
            } while( true );
          }
          dat_1a010 = 1;
label_7405:
          v3 = dat_dc9e4;
          if (!dat_1a848) {
label_7415:
            dat_1a848 = (unsigned int)(dat_1802c == 0);
            v3 = dat_dc9e4;
          }
label_7428:
          if ((v3) && (!dat_dc9e8))
            sub_f6b0(stderr,"%s:\t",0x1a420);
          while (v3 = (*dat_180d0)(dat_1a014,dat_1a010), !v3) {
            if ((!dat_dc9f8) || (dat_dc9dc)) goto label_74a4;
            if (dat_1a008 == dat_1a00c) {
              if ((dat_1a00c != 0x40000) || (sub_d6c0(1) == -1)) goto label_74a4;
              dat_1a008 = 0;
            }
            dat_18020 = sub_5750(dat_1a014);
            if (dat_18020 < 0) goto label_74a4;
            dat_1a820 = 0;
          }
          dat_18020 = -1;
label_74a4:
          if (close(dat_1a014))
            sub_d4e0(); // no-return
          if (dat_dc9e8) {
            sub_65d0(dat_18020); // tail-call
            return;
          }
          if (dat_1a860) {
            if (dat_18020 == -1)
              return;
            goto label_74e2;
          }
          v13._0_8_ = dat_dc0a8;
          v16 = dat_dc078 & 0x1ff;
          v13._24_8_ = dat_dc0c0;
          v13._8_8_ = dat_dc0b0;
          v13._16_8_ = dat_dc0b8;
          if (((dat_dc9f8) && (0 <= dat_1a838)) && ((dat_1a830 != dat_dc0b8 || (dat_dc0c0 != dat_1a838)))) {
            v13._16_8_ = dat_1a830;
            v13._24_8_ = dat_1a838;
            v3 = sub_ea40(dat_1a010,(char *)0x1a020,&v13);
            if (v3) { // branch-flip
label_7ee0:
              v5 = __errno_location();
              v3 = *v5;
              if (dat_1a85c) { // branch-flip
                if (dat_1a84c) goto label_7c17;
label_7f07:
                dat_1a84c = 2;
                v4 = dat_1a85c;
              }
              else {
                sub_f6b0(stderr,"%s: ",dat_1a850);
                v4 = dat_1a85c;
                if (!dat_1a84c) goto label_7f07;
              }
              if (!v4) {
                *v5 = v3;
                perror((char *)0x1a020);
              }
            }
            else if (2 <= dat_dc9e4)
              sub_f6b0(stderr,"%s: timestamp restored\n",0x1a020);
          }
          else {
            v3 = sub_ea40(dat_1a010,(char *)0x1a020,&v13);
            if (v3) goto label_7ee0;
          }
label_7c17:
          fchown(dat_1a010,0xffffffff,dat_dc080);
          if (fchmod(dat_1a010,v16)) {
            v5 = __errno_location();
            v3 = *v5;
            if (dat_1a85c) { // branch-flip
              if (dat_1a84c) goto label_7c7b;
label_7c62:
              dat_1a84c = 2;
              v4 = dat_1a85c;
            }
            else {
              sub_f6b0(stderr,"%s: ",dat_1a850);
              v4 = dat_1a85c;
              if (!dat_1a84c) goto label_7c62;
            }
            if (!v4) {
              *v5 = v3;
              perror((char *)0x1a020);
            }
          }
label_7c7b:
          fchown(dat_1a010,dat_dc07c,0xffffffff);
          if (((dat_dca00) && ((((0 <= dat_18014 && (fdatasync(dat_18014))) && (*__errno_location() != 0x16)) || ((fsync(dat_1a010) && (*__errno_location() != 0x16)))))) || (close(dat_1a010)))
            sub_d550(); // no-return
          if (!dat_dc9f0) {
            v9 = (char *)sub_dfd0((char *)0x1a420);
            v14 = (char *)0x1a420;
            v17 = &v9[-0x1a420];
            v15 = v17;
            if (!v17) {
              v17 = (char *)0x1;
              v15 = (char *)0x1;
              v14 = ".";
            }
            v4 = memcmp((void *)0xdc100,v14,(unsigned long)v17);
            v3 = dat_18014;
            if ((v4) || (v15[0xdc100])) {
              sigprocmask(0,(sigset_t *)0xdc940,&v13);
label_8152:
              dat_18018 = -1;
              v3 = sub_d2a0((char *)0x1a420);
            }
            else {
              sigprocmask(0,(sigset_t *)0xdc940,&v13);
              dat_18018 = -1;
              if (v3 < 0) goto label_8152;
              v3 = unlinkat(v3,v9,0);
            }
            if (v3) { // branch-flip
              v5 = __errno_location();
              v3 = *v5;
              sigprocmask(2,&v13,NULL);
              if (v3) {
                if (dat_1a85c) { // branch-flip
                  if (dat_1a84c) goto label_7d99;
label_8116:
                  dat_1a84c = 2;
                  v4 = dat_1a85c;
                }
                else {
                  sub_f6b0(stderr,"%s: ",dat_1a850);
                  v4 = dat_1a85c;
                  if (!dat_1a84c) goto label_8116;
                }
                if (!v4) {
                  *v5 = v3;
                  perror((char *)0x1a420);
                }
              }
            }
            else {
              sigprocmask(2,&v13,NULL);
            }
          }
label_7d99:
          if (dat_18020 == -1) {
            if (dat_1a860)
              return;
            sub_5530(0); // tail-call
            return;
          }
label_74e2:
          if (!dat_dc9e4)
            return;
          if (dat_1a858) // branch-flip
            sub_f6b0(stderr," OK");
          else if (dat_dc9f8) // branch-flip
            sub_d850(dat_1a820 - (dat_1a828 - dat_dfea8),dat_1a820);
          else {
            sub_d850(dat_1a828 - (dat_1a820 - dat_dfea8));
          }
          if (!dat_1a858) {
            v15 = "created";
            if (!dat_dc9f0)
              v15 = "replaced with";
            sub_f6b0(stderr," -- %s %s",v15,0x1a020);
          }
          sub_f6b0(stderr,"\n"); // tail-call
          return;
        }
      }
      v3 = dat_1a014;
      close(v3);
      return;
    }
    if ((dat_dc078 & 0xf000) != 0x8000) { // branch-flip
      if (!dat_1a85c) {
        sub_f6b0(stderr,"%s: %s is not a directory or a regular file - ignored\n",dat_1a850,0x1a420);
        v3 = dat_1a014;
      }
    }
    else if (dat_dc078 & 0x800) { // branch-flip
      if (!dat_1a85c) {
        sub_f6b0(stderr,"%s: %s is set-user-ID on execution - ignored\n",dat_1a850,0x1a420);
        v3 = dat_1a014;
      }
    }
    else if (dat_dc078 & 0x400) { // branch-flip
      if (!dat_1a85c) {
        sub_f6b0(stderr,"%s: %s is set-group-ID on execution - ignored\n",dat_1a850,0x1a420);
        v3 = dat_1a014;
      }
    }
    else {
      if (dat_dc9f4) {
label_77f1:
        sub_6a00();
        goto label_7353;
      }
      if (dat_dc078 & 0x200) { // branch-flip
        if (!dat_1a85c) {
          sub_f6b0(stderr,"%s: %s has the sticky bit set - file ignored\n",dat_1a850,0x1a420);
          v3 = dat_1a014;
        }
      }
      else {
        if (dat_dc070 < 2) goto label_77f1;
        if (!dat_1a85c) {
          v15 = (char *)0x12c94;
          if (dat_dc070 != 2)
            v15 = "s";
          sub_f6b0(stderr,"%s: %s has %lu other link%s -- file ignored\n",dat_1a850,0x1a420,dat_dc070 - 1,v15);
          v3 = dat_1a014;
        }
      }
    }
    if (!dat_1a84c)
      dat_1a84c = 2;
  }
  else {
    if (!dat_dc9ec) {
      close(v3);
      if (!dat_1a85c)
        sub_f6b0(stderr,"%s: %s is a directory -- ignored\n",dat_1a850,0x1a420);
      if (dat_1a84c)
        return;
      dat_1a84c = 2;
      return;
    }
    v7 = fdopendir(v3);
    if (v7) {
      v18 = (char *)sub_e540(v7,0);
      if (!v18) {
        sub_5640(a0);
        if (!closedir(v7))
          return;
        v5 = __errno_location();
        v3 = *v5;
        sub_f6b0(stderr,"%s: ",dat_1a850);
        *v5 = v3;
        perror(a0);
        dat_1a84c = 1;
        return;
      }
      if (closedir(v7)) {
        v5 = __errno_location();
        v3 = *v5;
        sub_f6b0(stderr,"%s: ",dat_1a850);
        *v5 = v3;
        perror(a0);
        dat_1a84c = 1;
      }
      v1 = *v18;
      v15 = v18;
      do {
        while( true ) {
          if (!v1) {
            free(v18);
            return;
          }
          v6 = strlen(v15);
          if ((strcmp(v15,".")) && (strcmp(v15,".."))) break;
label_71d8:
          v15 = (char *)((long)v15 + v6 + 1);
          v1 = *v15;
        }
        v8 = strlen(a0);
        if (v8 + v6 <= 0x3fd) {
          __strcpy_chk(v12,a0,0x400);
          if ((*(char *)sub_dfd0(v12)) && (v12[v8 - 1] != '/')) {
            v12[v8] = '/';
            v8 += 1;
          }
          strcpy(&v12[v8],v15);
          v4 = strcmp(v12,"-");
          v3 = dat_1a860;
          if (v4) // branch-flip
            sub_6e90(v12);
          else {
            sub_6a80();
            dat_1a860 = v3;
          }
          goto label_71d8;
        }
        v9 = (char *)((long)v15 + v6 + 1);
        sub_f6b0(stderr,"%s: %s/%s: pathname too long\n",dat_1a850,a0,v15);
        v1 = *v9;
        dat_1a84c = 1;
        v15 = v9;
      } while( true );
    }
    sub_5640(a0);
  }
  close(v3);
  return;
label_7f55:
  v10 = (char *)sub_63e0((char *)0x1a020);
  if (!v10)
    sub_d3f0("can\'t recover suffix\n"); // no-return
  *v10 = 0;
  dat_1a848 = 1;
  if ((5 <= v3) && (!strcmp(&v10[-4],".tar"))) {
    *(unsigned int *)&v10[-4] = 0x7a67742e;
    v2 = 1;
    *v10 = 0;
    goto label_78ef;
  }
  v3 = 3;
  v15 = (char *)sub_dfd0((char *)0x1a020);
  v1 = *v15;
  do {
    if (v1) {
      v9 = NULL;
      v17 = v15;
      do {
        v4 = (int)strcspn(v17,".");
        v14 = &v17[v4];
        if (v4 <= v3) { // branch-flip
          if (!*v14) break;
        }
        else {
          v9 = &v14[-1];
          if (!*v14) goto label_8328;
        }
        v17 = &v14[1];
      } while (v14[1]);
      if (v9) goto label_8328;
    }
    v3 -= 1;
  } while (v3);
  v15 = strrchr((char *)0x1a020,0x2e);
  if (!v15)
    sub_d3f0("internal error in shorten_name"); // no-return
  v15 = &v15[-(unsigned long)(v15[1] == '\0')];
label_802d:
  v2 = 1;
  strcpy(v15,dat_dc9c8);
  goto label_78ef;
label_8328:
  do {
    v15 = v9;
    *v15 = v15[1];
    v9 = &v15[1];
  } while (v15[1]);
  goto label_802d;
}

// Function: sub_8440 @ 0x8440
void sub_8440(void)
{
  sub_5530(0);
  sub_6960(1); // no-return
}

// Function: sub_8460 @ 0x8460
unsigned long sub_8460(long a0,long a1,unsigned int a2,unsigned int a3) // return-dupe
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
  v16 = dat_dca10;
  v13 = dat_dca0c;
  v12 = dat_1a004;
label_84c5:
  do {
    while( true ) {
      for (; v13 < a2; v13 = v13 + 8) {
        v10 = (unsigned long)dat_1a008;
        if (dat_1a00c <= dat_1a008) { // branch-flip
          dat_1a004 = v12;
          v7 = sub_d6c0(0);
        }
        else {
          dat_1a008 += 1;
          v7 = *(unsigned char *)(v10 + 0x9c000);
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
          v10 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) { // branch-flip
            dat_1a004 = v12;
            v7 = sub_d6c0(0);
          }
          else {
            dat_1a008 += 1;
            v7 = *(unsigned char *)(v10 + 0x9c000);
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
      *(char *)(v10 + 0x3b000) = (char)*(unsigned short *)&v15[8];
      if (v12 == 0x8000) {
        dat_1a004 = 0x8000;
        sub_d640();
        v12 = 0;
      }
    }
    if (v11 == 0xf) {
      dat_1a004 = v12;
      dat_dca0c = v13;
      dat_dca10 = v16;
      return 0;
    }
    for (; v13 < v11; v13 = v13 + 8) {
      v10 = (unsigned long)dat_1a008;
      if (dat_1a00c <= dat_1a008) { // branch-flip
        dat_1a004 = v12;
        v6 = sub_d6c0(0);
      }
      else {
        dat_1a008 += 1;
        v6 = *(unsigned char *)(v10 + 0x9c000);
      }
      v16 |= (unsigned long)v6 << ((unsigned char)v13 & 0x3f);
    }
    v4 = *(unsigned short *)&v15[8];
    v10 = v16 >> (v7 & 0x3f);
    v5 = *(unsigned short *)((unsigned long)v7 * 2 + 0x13380);
    for (v13 = v13 - v11; v13 < a3; v13 = v13 + 8) {
      v9 = (unsigned long)dat_1a008;
      if (dat_1a00c <= dat_1a008) { // branch-flip
        dat_1a004 = v12;
        v7 = sub_d6c0(0);
      }
      else {
        dat_1a008 += 1;
        v7 = *(unsigned char *)(v9 + 0x9c000);
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
          v9 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) { // branch-flip
            dat_1a004 = v12;
            v7 = sub_d6c0(0);
          }
          else {
            dat_1a008 += 1;
            v7 = *(unsigned char *)(v9 + 0x9c000);
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
          v9 = (unsigned long)dat_1a008;
          if (dat_1a008 < dat_1a00c) break;
          dat_1a004 = v12;
          v10 |= (unsigned long)(unsigned char)sub_d6c0(0) << ((unsigned char)v8 & 0x3f);
          v13 = v8 + 8;
          if (v11 <= v8 + 8) goto label_883d;
        }
        dat_1a008 += 1;
        v10 |= (unsigned long)*(unsigned char *)(v9 + 0x9c000) << ((unsigned char)v8 & 0x3f);
        v13 = v8 + 8;
      } while (v8 + 8 < v11);
label_883d:
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
        memcpy((void *)((unsigned long)v12 + 0x3b000),(void *)((unsigned long)v11 + 0x3b000),v10);
        if (v12 + v17 != 0x8000) goto label_88c2;
label_8937:
        v11 += v17;
        dat_1a004 = 0x8000;
        v12 = 0;
        sub_d640();
        if (!v14) goto label_84c5;
      }
      v8 = v11;
      do {
        v10 = (unsigned long)v8;
        v1 = v8 + (v12 - v11);
        v8 += 1;
        *(char *)((unsigned long)v1 + 0x3b000) = *(char *)(v10 + 0x3b000);
      } while (v8 != v11 + v17);
      if (v12 + v17 == 0x8000) goto label_8937;
label_88c2:
      v11 += v17;
      v12 += v17;
    } while (v14);
  } while( true );
}

// Function: sub_89b0 @ 0x89b0
bool sub_89b0(unsigned int *a0,unsigned int a1,unsigned int a2,long a3,long a4,long *a5,unsigned int *a6) // early-return x3
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
      dat_dca08 += 3;
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
      goto label_8a8d;
    }
    v3 += 1;
  } while (v3 != 0x11);
  v33 = 0x11;
  v23 = 0x20000;
  v11 = 0x11;
label_8a8d:
  v30 = *a6;
  if (*a6 < v11)
    v30 = v33;
  v4 = v18;
  v26 = 0x10;
label_8aa4:
  if (!*v4) goto label_8aaf;
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
    goto label_8b9a;
  }
  v40 = v23 - *(int *)&v17[(unsigned long)v26 * 4];
  if (v40 < 0)
    return (bool)2;
  *(int *)&v17[(unsigned long)v26 * 4] = v23;
  v19[1] = 0;
  if (v26 != 1) goto label_8bbd;
  goto label_8beb;
label_8aaf:
  v4 = &v4[-1];
  v26 -= 1;
  if (!v26) {
    if (v30) { // branch-flip
      *a6 = 0;
      v30 = 0;
      v39 = 0;
      v40 = v17._0_4_;
label_8b9a:
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
label_8bbd:
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
label_8beb:
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
                  v3 = v22[1];
                  if (v34) {
                    while (v3) {
                      v29 = *(long *)(v3 + -8);
                      free((void *)(v3 + -0x10));
                      v3 = v29;
                    }
                    return (bool)3;
                  }
                  return (bool)3;
                }
                dat_dca08 = v26 + 1 + dat_dca08;
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
            if (v19[v34] != ((1 << (v2 & 0x1f)) - 1U & v11)) {
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
  goto label_8aa4;
}

// Function: sub_90c0 @ 0x90c0
int sub_90c0(void)
{
  void *v1;
  long v10; // stack - 0x4b8
  unsigned int v11; // stack - 0x4bc
  long v12; // stack - 0x4b0
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  unsigned long v15; // stack - 0x38
  unsigned long v16; // stack - 0x30
  long v2;
  long v3;
  int v4;
  unsigned int *v5;
  unsigned int v6 [24];
  unsigned int v7 [144];
  unsigned int v8 [112];
  unsigned int v9; // stack - 0x4c0
  
  v5 = v7;
  do {
    *v5 = 8;
    v5 = &v5[1];
  } while (v5 != v8);
  do {
    *v5 = 9;
    v5 = &v5[1];
  } while (v5 != v6);
  v5 = v6;
  do {
    *v5 = 7;
    v5 = &v5[1];
  } while (&v6[0x18] != v5);
  v9 = 7;
  v13 = 0x800000008;
  v14 = 0x800000008;
  v15 = 0x800000008;
  v16 = 0x800000008;
  v4 = sub_89b0(v7,0x120,0x101,0x181a0,0x18160,&v10,&v9);
  if (!v4) {
    v5 = v7;
    do {
      *v5 = 5;
      v5 = &v5[1];
    } while (v5 != &v7[0x1e]);
    v11 = 5;
    v4 = sub_89b0(v7,0x1e,0,0x18120,0x180e0,&v12,&v11);
    v3 = v10; // branch-flip
    if (2 <= v4) {
      while (v3) {
        v2 = *(long *)(v3 + -8);
        free((void *)(v3 + -0x10));
        v3 = v2;
      }
    }
    else {
      v4 = sub_8460(v10,v12,v9,v11);
      v3 = v10;
      if (v4) // branch-flip
        v4 = 1;
      else {
        while (v3) {
          v2 = *(long *)(v3 + -8);
          free((void *)(v3 + -0x10));
          v3 = v2;
        }
        v4 = 0;
        v3 = v12;
        while (v3) {
          v1 = (void *)(v3 + -0x10);
          v3 = *(long *)(v3 + -8);
          free(v1);
        }
      }
    }
  }
  return v4;
}

// Function: sub_92b0 @ 0x92b0
unsigned int sub_92b0(void)
{
  void *v1;
  unsigned int v10; // eax
  int v11; // eax
  unsigned int v12;
  unsigned long v13;
  unsigned long v14;
  long v15;
  unsigned int v16 [11];
  long v17; // stack - 0x548
  unsigned int v18; // stack - 0x550
  unsigned int v19 [307];
  unsigned short v2;
  unsigned int v20; // stack - 0x54c
  char *v21; // stack - 0x540
  unsigned int v22;
  unsigned int *v23;
  unsigned int v24;
  unsigned int v25;
  unsigned int v26;
  unsigned int v27;
  unsigned int v28; // stack - 0x588
  unsigned int v29; // stack - 0x57c
  unsigned short v3;
  long v4;
  char *v5;
  char *v6;
  unsigned int v7;
  unsigned char v8;
  unsigned char v9;
  
  v13 = dat_dca10;
  v26 = dat_dca0c;
  v7 = dat_1a004;
  v24 = dat_dca0c;
  if (dat_dca0c <= 4) {
    v14 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) // branch-flip
      v8 = sub_d6c0(0);
    else {
      dat_1a008 += 1;
      v8 = *(unsigned char *)(v14 + 0x9c000);
    }
    v24 = v26 + 8;
    v13 |= (unsigned long)v8 << ((unsigned char)v26 & 0x3f);
  }
  v26 = v24 - 5;
  v14 = v13 >> 5;
  v10 = ((unsigned int)v13 & 0x1f) + 0x101;
  v12 = v26;
  if (v26 <= 4) {
    v13 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) { // branch-flip
      dat_1a004 = v7;
      v8 = sub_d6c0(0);
    }
    else {
      dat_1a008 += 1;
      v8 = *(unsigned char *)(v13 + 0x9c000);
    }
    v12 = v24 + 3;
    v14 |= (unsigned long)v8 << ((unsigned char)v26 & 0x3f);
  }
  v25 = v12 - 5;
  v13 = v14 >> 5;
  v24 = ((unsigned int)v14 & 0x1f) + 1;
  v26 = v25;
  if (v25 <= 3) {
    v14 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) { // branch-flip
      dat_1a004 = v7;
      v8 = sub_d6c0(0);
    }
    else {
      dat_1a008 += 1;
      v8 = *(unsigned char *)(v14 + 0x9c000);
    }
    v26 = v12 + 3;
    v13 |= (unsigned long)v8 << ((unsigned char)v25 & 0x3f);
  }
  if ((v10 <= 0x11e) && (v24 <= 0x1e)) {
    v26 -= 4;
    v23 = (unsigned int *)0x133c0;
    v25 = (unsigned int)v13 & 0xf;
    v12 = v25 + 4;
    v13 >>= 4;
    do {
      v27 = v26;
      if (v26 <= 2) {
        v14 = (unsigned long)dat_1a008;
        if (dat_1a00c <= dat_1a008) { // branch-flip
          dat_1a004 = v7;
          v8 = sub_d6c0(0);
        }
        else {
          dat_1a008 += 1;
          v8 = *(unsigned char *)(v14 + 0x9c000);
        }
        v27 = v26 + 8;
        v13 |= (unsigned long)v8 << ((unsigned char)v26 & 0x3f);
      }
      v22 = *v23;
      v23 = &v23[1];
      v14 = v13 >> 3;
      v26 = v27 - 3;
      v16[v22] = (unsigned int)v13 & 7;
      v13 = v14;
    } while ((unsigned int *)((unsigned long)v25 * 4 + 0x133d0) != v23);
    for (; v12 != 0x13; v12 = v12 + 1) {
      v16[*(unsigned int *)((unsigned long)v12 * 4 + 0x133c0)] = 0;
    }
    v18 = 7;
    v25 = sub_89b0(v16,0x13,0x13,0,0,&v17,&v18);
    v12 = v18;
    if (v25) { // branch-flip
      v15 = v17;
      if (v25 != 1)
        return v25;
      while (v15) {
        v4 = *(long *)(v15 + -8);
        free((void *)(v15 + -0x10));
        v15 = v4;
      }
    }
    else {
      if (!v17)
        return 2;
      v29 = 0;
      v27 = v10 + v24;
      v28 = 0;
      v2 = *(unsigned short *)((long)(int)v18 * 2 + 0x13380);
      v25 = 0;
      do {
        while (v26 < v12) {
          v13 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) { // branch-flip
            dat_1a004 = v7;
            v8 = sub_d6c0(0);
          }
          else {
            dat_1a008 += 1;
            v8 = *(unsigned char *)(v13 + 0x9c000);
          }
          v14 |= (unsigned long)v8 << ((unsigned char)v26 & 0x3f);
          v26 += 8;
        }
        v21 = (char *)((unsigned long)((unsigned int)v2 & (unsigned int)v14) * 0x10 + v17);
        v13 = v14 >> (v21[1] & 0x3fU);
        v26 -= (unsigned char)v21[1];
        if (*v21 == 'c') {
          if (!v17)
            return 2;
          v15 = v17;
          do {
            v1 = (void *)(v15 + -0x10);
            v15 = *(long *)(v15 + -8);
            free(v1);
          } while (v15);
          return 2;
        }
        v3 = *(unsigned short *)&v21[8];
        v22 = (unsigned int)v3; // branch-flip
        if (0x10 <= v3) {
          v8 = (unsigned char)v26;
          if (v3 != 0x10) { // branch-flip
            if (v22 != 0x11) { // branch-flip
              if (v26 <= 6) {
                v14 = (unsigned long)dat_1a008;
                if (dat_1a00c <= dat_1a008) { // branch-flip
                  dat_1a004 = v7;
                  v9 = sub_d6c0(0);
                }
                else {
                  dat_1a008 += 1;
                  v9 = *(unsigned char *)(v14 + 0x9c000);
                }
                v26 += 8;
                v13 |= (unsigned long)v9 << (v8 & 0x3f);
              }
              v14 = v13 >> 7;
              v26 -= 7;
              v22 = (unsigned int)v13 & 0x7f;
              if (v27 < v22 + 0xb + v28)
                return 1;
              v23 = &v16[(int)v25];
              do {
                *v23 = 0;
                v23 = &v23[1];
              } while (&v19[(unsigned long)v22 + (long)(int)v25] != v23);
              v29 = 0;
              v28 = v22 + 0xb + v25;
            }
            else {
              if (v26 <= 2) {
                v14 = (unsigned long)dat_1a008;
                if (dat_1a00c <= dat_1a008) { // branch-flip
                  dat_1a004 = v7;
                  v9 = sub_d6c0(0);
                }
                else {
                  dat_1a008 += 1;
                  v9 = *(unsigned char *)(v14 + 0x9c000);
                }
                v26 += 8;
                v13 |= (unsigned long)v9 << (v8 & 0x3f);
              }
              v14 = v13 >> 3;
              v26 -= 3;
              v22 = (unsigned int)v13 & 7;
              if (v27 < v22 + 3 + v28)
                return 1;
              v23 = &v16[(int)v25];
              do {
                *v23 = 0;
                v23 = &v23[1];
              } while (&v16[(unsigned long)v22 + (long)(int)v25 + 3] != v23);
              v29 = 0;
              v28 = v22 + 3 + v25;
            }
          }
          else {
            if (v26 <= 1) {
              v14 = (unsigned long)dat_1a008;
              if (dat_1a00c <= dat_1a008) { // branch-flip
                dat_1a004 = v7;
                v9 = sub_d6c0(0);
              }
              else {
                dat_1a008 += 1;
                v9 = *(unsigned char *)(v14 + 0x9c000);
              }
              v26 += 8;
              v13 |= (unsigned long)v9 << (v8 & 0x3f);
            }
            v14 = v13 >> 2;
            v26 -= 2;
            v22 = (unsigned int)v13 & 3;
            if (v27 < v22 + 3 + v28)
              return 1;
            v23 = &v16[(int)v25];
            do {
              *v23 = v29;
              v23 = &v23[1];
            } while (&v16[(unsigned long)v22 + (long)(int)v25 + 3] != v23);
            v28 = v22 + 3 + v25;
          }
        }
        else {
          v28 = v25 + 1;
          v16[(int)v25] = v22;
          v14 = v13;
          v29 = v22;
        }
        v25 = v28;
        v15 = v17;
      } while (v28 < v27);
      while (v15) {
        v4 = *(long *)(v15 + -8);
        free((void *)(v15 + -0x10));
        v15 = v4;
      }
      v18 = 9;
      dat_dca0c = v26;
      dat_dca10 = v14;
      v26 = sub_89b0(v16,v10,0x101,0x181a0,0x18160,&v17,&v18);
      if (!v26) {
        v20 = 6;
        v26 = sub_89b0(&v16[v10],v24,0,0x18120,0x180e0,&v21,&v20);
        if (!v26) {
          v11 = sub_8460(v17,v21,v18,v20);
          while (v6 = v21, v17) {
            v1 = (void *)(v17 + -0x10);
            v17 = *(long *)(v17 + -8);
            free(v1);
          }
          while (v6) {
            v5 = *(char **)&v6[-8];
            free(&v6[-0x10]);
            v6 = v5;
          }
          return (unsigned int)(v11 != 0);
        }
        v6 = v21;
        v15 = v17;
        if (v26 == 1) {
          while (v15 = v17, v6) {
            v5 = *(char **)&v6[-8];
            free(&v6[-0x10]);
            v6 = v5;
          }
        }
        while (v15) {
          v4 = *(long *)(v15 + -8);
          free((void *)(v15 + -0x10));
          v15 = v4;
        }
        return v26;
      }
      v15 = v17;
      if (v26 != 1)
        return v26;
      while (v15) {
        v4 = *(long *)(v15 + -8);
        free((void *)(v15 + -0x10));
        v15 = v4;
      }
    }
  }
  return 1;
}

// Function: sub_9a50 @ 0x9a50
unsigned long sub_9a50(void) // early-return
{
  unsigned char v1;
  unsigned int v10; // r12d
  unsigned int v11;
  unsigned long v12;
  bool v13; // cf
  unsigned int v2;
  unsigned int v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned int v7;
  unsigned int v8;
  int v9;
  
  v2 = 0;
  v12 = 0;
  dat_1a004 = 0;
  dat_dca0c = 0;
  dat_dca10 = 0;
  do {
    dat_dca08 = 0;
    if (v2) { // branch-flip
      v4 = v12 >> 1;
      v10 = (unsigned int)v12 & 1;
      if (2 <= v2 - 1) // branch-flip
        v3 = v2 - 3;
      else {
        v12 = (unsigned long)dat_1a008;
        if (dat_1a00c <= dat_1a008) // branch-flip
          v1 = sub_d6c0(0);
        else {
          dat_1a008 += 1;
          v1 = *(unsigned char *)(v12 + 0x9c000);
        }
        v3 = v2 + 5;
        v4 |= (unsigned long)v1 << ((unsigned char)(v2 - 1) & 0x3f);
      }
    }
    else {
      v4 = (unsigned long)dat_1a008;
      if (dat_1a00c <= dat_1a008) // branch-flip
        v1 = sub_d6c0(0);
      else {
        dat_1a008 += 1;
        v1 = *(unsigned char *)(v4 + 0x9c000);
      }
      v3 = 5;
      v4 = (v1 | v12) >> 1;
      v10 = (unsigned int)(v1 | v12) & 1;
    }
    v11 = dat_1a004;
    dat_dca10 = v4 >> 2;
    v2 = (unsigned int)v4 & 3;
    dat_dca0c = v3;
    if (v2 != 2) { // branch-flip
      if (v4 & 3) {
        if (v2 != 1)
          return 2;
        v6 = sub_90c0();
        goto label_9d95;
      }
      v2 = v3 & 7;
      v4 = dat_dca10 >> (signed char)v2;
      v8 = v3 - v2;
      if (v8 <= 0xf) {
        v7 = v8;
        do {
          while( true ) {
            v12 = (unsigned long)dat_1a008;
            v1 = (unsigned char)v7;
            if (dat_1a008 < dat_1a00c) break;
            dat_1a004 = v11;
            v7 += 8;
            v4 |= (unsigned long)(unsigned char)sub_d6c0(0) << (v1 & 0x3f);
            if (0x10 <= v7) goto label_9c01;
          }
          dat_1a008 += 1;
          v7 += 8;
          v4 |= (unsigned long)*(unsigned char *)(v12 + 0x9c000) << (v1 & 0x3f);
        } while (v7 <= 0xf);
label_9c01:
        v8 = v8 + 8 + ((v2 - v3) + 0xf & 0xfffffff8);
      }
      v3 = (unsigned int)v4 & 0xffff;
      v12 = v4 >> 0x10;
      v2 = v8 - 0x10;
      if (v2 <= 0xf) {
        do {
          v5 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) { // branch-flip
            dat_1a004 = v11;
            v1 = sub_d6c0(0);
          }
          else {
            dat_1a008 += 1;
            v1 = *(unsigned char *)(v5 + 0x9c000);
          }
          v7 = v2 + 8;
          v12 |= (unsigned long)v1 << ((unsigned char)v2 & 0x3f);
          v2 = v7;
        } while (v7 <= 0xf);
        v2 = (v8 - 8) + (0x1f - v8 & 0xfffffff8);
      }
      if (v3 != (~(unsigned int)v12 & 0xffff))
        return 1;
      v12 >>= 0x10;
      v2 -= 0x10;
      v9 = v3 - 1;
      dat_1a004 = v11;
      if (v4 & 0xffff) {
        do {
          while( true ) {
            if (8 <= v2) // branch-flip
              v2 -= 8;
            else {
              v4 = (unsigned long)dat_1a008;
              if (dat_1a00c <= dat_1a008) { // branch-flip
                dat_1a004 = v11;
                v1 = sub_d6c0(0);
              }
              else {
                dat_1a008 += 1;
                v1 = *(unsigned char *)(v4 + 0x9c000);
              }
              v12 |= (unsigned long)v1 << ((unsigned char)v2 & 0x3f);
            }
            v4 = (unsigned long)v11;
            v11 += 1;
            *(char *)(v4 + 0x3b000) = (char)v12;
            if (v11 == 0x8000) break;
            v12 >>= 8;
            v13 = v9 == 0;
            v9 -= 1;
            dat_1a004 = v11;
            if (v13) goto label_9d53;
          }
          dat_1a004 = 0x8000;
          v11 = 0;
          v12 >>= 8;
          sub_d640();
          v13 = v9 != 0;
          v9 -= 1;
          dat_1a004 = v11;
        } while (v13);
      }
    }
    else {
      v6 = sub_92b0();
label_9d95:
      v12 = dat_dca10;
      v2 = dat_dca0c;
      if ((int)v6)
        return v6;
    }
label_9d53:
    dat_dca0c = v2;
    dat_dca10 = v12;
    if (v10) {
      if (8 <= v2) {
        dat_dca0c = v2 & 7;
        dat_1a008 += ~(v2 - 8 >> 3);
      }
      sub_d640();
      return 0;
    }
  } while( true );
}

// Function: sub_9df0 @ 0x9df0
void sub_9df0(void)
{
  unsigned short *v1; // rax
  unsigned short *v2;
  
  v2 = (unsigned short *)0xdf080;
  do {
    v1 = &v2[2];
    *v2 = 0;
    v2 = v1;
  } while (v1 != (unsigned short *)0xdf4f8);
  v2 = (unsigned short *)0xdef80;
  do {
    v1 = &v2[2];
    *v2 = 0;
    v2 = v1;
  } while (v1 != (unsigned short *)0xdeff8);
  v2 = (unsigned short *)0xde9e0;
  do {
    v1 = &v2[2];
    *v2 = 0;
    v2 = v1;
  } while (v1 != (unsigned short *)0xdea2c);
  dat_dca38 = 0;
  dat_df480 = 1;
  dat_dca40 = 0;
  dat_dca4c = 0;
  dat_dca50 = 0;
  dat_dca54 = 0;
  dat_dca49 = 0;
  dat_dca48 = 1;
}

// Function: sub_9ea0 @ 0x9ea0
void sub_9ea0(long a0,int a1)
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
  v3 = *(int *)(v9 * 4 + 0xde0c0);
  if (a1 * 2 <= dat_de0a4) {
    v1 = *(unsigned short *)(a0 + (long)v3 * 4);
    v6 = a1 * 2;
    do {
      v7 = *(int *)((long)v6 * 4 + 0xde0c0);
      v8 = *(unsigned short *)(a0 + (long)v7 * 4);
      if (v6 < dat_de0a4) {
        v5 = v6 + 1;
        v4 = *(int *)((long)v5 * 4 + 0xde0c0);
        v2 = *(unsigned short *)(a0 + (long)v4 * 4);
        if (v8 <= v2) { // branch-flip
          if (v2 != v8) { // branch-flip
          }
          else if (*(unsigned char *)((long)v7 + 0xdde60) >= *(unsigned char *)((long)v4 + 0xdde60)) {
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
      if ((v1 < v8) || ((v1 == v8 && (*(unsigned char *)((long)v3 + 0xdde60) <= *(unsigned char *)((long)v7 + 0xdde60))))) break;
      v4 = v6 * 2;
      *(int *)(v9 * 4 + 0xde0c0) = v7;
      v9 = (long)v6;
      v6 = v4;
    } while (v4 <= dat_de0a4);
  }
  *(int *)(v9 * 4 + 0xde0c0) = v3;
}

// Function: sub_9f80 @ 0x9f80
void sub_9f80(long a0,int a1) // return-dupe
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
  
  v3 = *(unsigned short *)(a0 + 2);
  *(unsigned short *)(a0 + 6 + (long)a1 * 4) = 0xffff;
  v10 = 4 - (unsigned int)(v3 == 0);
  v9 = (-(unsigned int)(v3 == 0) & 0x83) + 7;
  if (a1 <= -1)
    return;
  v2 = (unsigned short *)(a0 + 10 + (long)a1 * 4);
  v5 = (unsigned short *)(a0 + 6);
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
        v1 = (short *)((long)v6 * 4 + 0xde9e0);
        *v1 = *v1 + (short)v4;
        if (v3) goto label_a030;
        v10 = 3; // crossjump-dupe
        v9 = 0x8a;
        v4 = 0;
        v8 = v6;
      }
label_a005:
      v5 = &v5[2];
      v6 = v7;
      if (v2 == v5)
        return;
    }
    if (v6) { // branch-flip
      if (v6 != v8) {
        v1 = (short *)((long)v6 * 4 + 0xde9e0);
        *v1 = *v1 + 1;
      }
      dat_dea20 += 1;
    }
    else if (0xb <= v4) // branch-flip
      dat_dea28 += 1;
    else {
      dat_dea24 += 1;
    }
    if (!v7) {
      v10 = 3;
      v9 = 0x8a;
      v4 = 0;
      v8 = v6;
      goto label_a005;
    }
    if (v7 == v6) {
      v10 = 3;
      v9 = 6;
      v4 = 0;
      v8 = v7;
      goto label_a005;
    }
label_a030:
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

// Function: sub_a090 @ 0xa090
void sub_a090(struct_3 *a0) // return-dupe
{
  unsigned short *v1;
  bool v10;
  short v11;
  int v12;
  unsigned long v13; // rax
  long v14;
  unsigned long v15; // rcx
  long v16; // rcx
  int v17;
  long v18;
  unsigned short *v19;
  short *v2;
  unsigned short *v20; // rdx
  unsigned short *v21;
  short v22 [20];
  int v23;
  unsigned short v24;
  unsigned int v25;
  long v26;
  int *v27;
  int v28;
  int v29;
  unsigned char v3;
  int v30;
  long v31;
  int v32; // r14d
  short v33; // stack - 0x98
  int v34; // stack - 0x84
  unsigned char v4;
  short v5;
  int v6;
  int v7;
  int v8;
  bool v9;
  
  v21 = a0->field_0x0;
  v31 = a0->field_0x8;
  dat_de0a0 = 0x23d;
  v29 = a0->field_0x1c;
  if (1 <= v29) { // branch-flip
    v9 = 0;
    v13 = 0;
    dat_de0a4 = 0;
    v15 = 0xffffffff;
    do {
      while (v34 = (int)v15, !v21[v13 * 2]) {
        v21[v13 * 2 + 1] = 0;
        v13 += 1;
        if (v13 == (long)v29) goto label_a136;
      }
      dat_de0a4 += 1;
      *(char *)(v13 + 0xdde60) = 0;
      v34 = (int)v13;
      v15 = v13 & 0xffffffff;
      *(int *)((long)dat_de0a4 * 4 + 0xde0c0) = v34;
      v13 += 1;
      v9 = 1;
    } while (v13 != (long)v29);
label_a136:
    if (v9) {
      if (1 < dat_de0a4) {
        v6 = dat_de0a4;
        goto label_a215;
      }
    }
    else {
      dat_de0a4 = 0;
    }
  }
  else {
    v34 = -1;
    dat_de0a4 = 0;
  }
  v18 = (long)dat_de0a4;
  v9 = 0;
  v26 = dat_dca38;
  do {
    if (2 <= v34) { // branch-flip
      v16 = 0;
      v14 = 0;
      v19 = v21;
    }
    else {
      v34 += 1;
      v14 = (long)v34;
      v16 = (long)v34 * 4;
      v19 = &v21[(long)v34 * 2];
    }
    *(int *)(v18 * 4 + 0xde0c4) = (int)v14;
    *v19 = 1;
    *(char *)(v14 + 0xdde60) = 0;
    if (v31) {
      v9 = 1;
      v26 -= (unsigned long)*(unsigned short *)(v31 + 2 + v16);
    }
    v18 += 1;
  } while ((int)v18 <= 1);
  v17 = 0;
  if (dat_de0a4 <= 1)
    v17 = 1U - dat_de0a4;
  v6 = dat_de0a4 + 1 + v17;
  v18 = -(unsigned long)(unsigned int)(1U - dat_de0a4);
  if (2 <= dat_de0a4)
    v18 = 0;
  dat_dca40 = dat_dca40 + -1 + v18;
  dat_de0a4 = v6;
  if (v9)
    dat_dca38 = v26;
label_a215:
  v26 = (long)v29;
  a0->field_0x24 = v34;
  v30 = v6 >> 1;
  do {
    sub_9ea0(v21,v30);
    v30 -= 1;
  } while (v30);
  v18 = (long)v6;
  v33 = (short)v29;
  v14 = 0x23b;
  do {
    v29 = dat_de0c4;
    dat_de0a4 = (int)v18 + -1;
    dat_de0c4 = *(int *)(v18 * 4 + 0xde0c0);
    sub_9ea0(v21,1);
    dat_de0a0 = (unsigned int)v14;
    *(int *)(v14 * 4 + 0xde0c4) = v29;
    v11 = v21[(long)dat_de0c4 * 2];
    v5 = v21[(long)v29 * 2];
    *(int *)(v14 * 4 + 0xde0c0) = dat_de0c4;
    v21[v26 * 2] = v11 + v5;
    v3 = *(unsigned char *)((long)v29 + 0xdde60);
    v4 = *(unsigned char *)((long)dat_de0c4 + 0xdde60);
    if (v4 <= v3)
      v4 = v3;
    v14 -= 2;
    *(unsigned char *)(v26 + 0xdde60) = v4 + 1;
    v11 = (v33 + (short)v6) - (short)v18;
    v18 -= 1;
    (&v21[(long)dat_de0c4 * 2])[1] = v11;
    (&v21[(long)v29 * 2])[1] = v11;
    dat_de0c4 = (int)v26;
    v26 += 1;
    sub_9ea0(v21,1);
  } while (1 < (int)v18);
  v29 = v6 * -2 + 0x23f;
  if (v6 <= 1)
    v29 = 0x23b;
  dat_de0a0 = v6 * -2 + 0x241;
  if (v6 <= 1)
    dat_de0a0 = 0x23d;
  dat_de0a0 -= 3;
  *(int *)((long)dat_de0a0 * 4 + 0xde0c0) = dat_de0c4;
  v26 = a0->field_0x10;
  v6 = a0->field_0x24;
  v17 = a0->field_0x20;
  v30 = a0->field_0x18;
  v19 = (unsigned short *)0xde9c0;
  do {
    v20 = &v19[1];
    *v19 = 0;
    v19 = v20;
  } while (v20 != (unsigned short *)0xde9e0);
  v21[(long)dat_de0c4 * 2 + 1] = 0;
  if (v29 <= 0x23c) {
    v10 = 0;
    v32 = 0;
    v9 = 0;
    v27 = (int *)((long)v29 * 4 + 0xde0c0);
    v18 = dat_dca40;
    v14 = dat_dca38;
    do {
      v7 = *v27;
      v1 = &v21[(long)v7 * 2];
      v28 = (unsigned short)v21[(unsigned long)v1[1] * 2 + 1] + 1;
      if (v17 < (int)((unsigned short)v21[(unsigned long)v1[1] * 2 + 1] + 1)) {
        v32 += 1;
        v28 = v17;
      }
      v1[1] = (unsigned short)v28;
      if (v7 <= v6) {
        v16 = (long)v28;
        v23 = 0;
        v2 = (short *)(v16 * 2 + 0xde9c0);
        *v2 = *v2 + 1;
        if (v30 <= v7) {
          v23 = *(int *)(v26 + (long)(v7 - v30) * 4);
          v16 = (long)(int)(v28 + v23);
        }
        v18 += v16 * (unsigned long)*v1;
        v9 = 1;
        if (v31) {
          v10 = 1;
          v14 += (long)(int)((unsigned int)*(unsigned short *)(v31 + 2 + (long)v7 * 4) + v23) * (unsigned long)*v1;
        }
      }
      v27 = &v27[1];
    } while ((int *)(((unsigned long)(unsigned int)(0x23c - v29) + (long)v29) * 4 + 0xde0c4) != v27);
    if (v10)
      dat_dca38 = v14;
    if (v9)
      dat_dca40 = v18;
    if (v32) {
      v29 = v32 + -2;
      v28 = v17 - 1;
      v31 = (long)v17;
      v30 = v29 - (v32 - 1U & 0xfffffffeU);
      v11 = *(short *)((long)v28 * 2 + 0xde9c0);
      if (v11) goto label_a535;
      do {
        v26 = v31 * 2 + 0xde9c0;
        v25 = v28;
        do {
          v11 = *(short *)(v26 + -4);
          v26 -= 2;
          v12 = v25 - 1;
          v8 = v25;
          v25 = v12;
        } while (!v11);
        while( true ) {
          *(short *)((long)v12 * 2 + 0xde9c0) = v11 + -1;
          v2 = (short *)((long)v8 * 2 + 0xde9c0);
          *v2 = *v2 + 2;
          v24 = *(short *)(v31 * 2 + 0xde9c0) - 1;
          *(unsigned short *)(v31 * 2 + 0xde9c0) = v24;
          if (v29 == v30) {
            if (v17) {
              v9 = 0;
              v29 = 0x23d;
              v26 = dat_dca40;
              while( true ) {
                v25 = (unsigned int)v24;
                if (v25) {
                  v18 = (long)(v29 + -1);
                  do {
                    while( true ) {
                      v30 = *(int *)(v18 * 4 + 0xde0c0);
                      v29 = (int)v18;
                      if (v30 <= v6) break;
                      v18 -= 1;
                    }
                    v1 = &v21[(long)v30 * 2];
                    v24 = v1[1];
                    if (v24 != v17) {
                      v1[1] = (unsigned short)v17;
                      v9 = 1;
                      v26 += (unsigned long)*v1 * (v31 - (unsigned long)v24);
                    }
                    v18 -= 1;
                    v25 -= 1;
                  } while (v25);
                }
                v31 -= 1;
                if (!v28) break;
                v24 = *(unsigned short *)(v31 * 2 + 0xde9c0);
                v17 = v28;
                v28 -= 1;
              }
              if (v9)
                dat_dca40 = v26;
            }
            goto label_a54c;
          }
          v11 = *(short *)((long)v28 * 2 + 0xde9c0);
          v29 -= 2;
          if (!v11) break;
label_a535:
          v8 = v17;
          v12 = v28;
        }
      } while( true );
    }
  }
label_a54c:
  v31 = 0;
  v11 = 0;
  do {
    v11 = (v11 + *(short *)(v31 + 0xde9c0)) * 2;
    *(short *)((long)v22 + v31 + 2) = v11;
    v31 += 2;
  } while (v31 != 0x1e);
  if (v34 == -1)
    return;
  v19 = &v21[(long)v34 * 2 + 2];
  do {
    v24 = v21[1];
    if (v24) {
      v11 = v22[(int)(unsigned int)v24];
      v22[(int)(unsigned int)v24] = v11 + 1;
      *v21 = sub_44a0((unsigned short)v11,v24);
    }
    v21 = &v21[2];
  } while (v19 != v21);
}

// Function: sub_a6b0 @ 0xa6b0
void sub_a6b0(long a0,int a1) // return-dupe
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
  
  v1 = *(unsigned short *)(a0 + 2);
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
    v1 = *(unsigned short *)(a0 + 6 + v8 * 4);
    v5 = (unsigned int)v1;
    if (v2 <= v9) { // branch-flip
label_a780:
      if (v4) { // branch-flip
        if (v7 != v4) {
          sub_43b0(*(unsigned short *)((long)v4 * 4 + 0xde9e0),*(unsigned short *)((long)v4 * 4 + 0xde9e2));
          v9 = v3;
        }
        sub_43b0(dat_dea20,dat_dea22);
        sub_43b0(v9 + -3,2);
      }
      else if (0xb <= v9) { // branch-flip
        sub_43b0(dat_dea28,dat_dea2a);
        sub_43b0(v3 + -10,7);
      }
      else {
        sub_43b0(dat_dea24,dat_dea26);
        sub_43b0(v3 + -2,3);
      }
      if (v1) { // branch-flip
        if (v5 != v4) { // branch-flip
label_a74e:
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
label_a7e8:
        v6 = 3;
        v2 = 0x8a;
        v3 = 0;
        v7 = v4;
      }
    }
    else {
      if (v5 != v4) {
        if (v6 <= v9) goto label_a780;
        do {
          sub_43b0(*(unsigned short *)((long)v4 * 4 + 0xde9e0),*(unsigned short *)((long)v4 * 4 + 0xde9e2));
          v9 -= 1;
        } while (v9);
        if (v1) goto label_a74e;
        goto label_a7e8;
      }
      v3 = v9;
    }
    v8 += 1;
    v4 = v5;
    if (a1 < (int)v8)
      return;
  } while( true );
}

// Function: sub_a870 @ 0xa870
void sub_a870(long a0,long a1) // ternary
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
  
  if (dat_dca54) {
    v7 = 0;
    v9 = 0;
    v6 = 0;
    v10 = 0;
    while( true ) {
      v11 = v10;
      if (!(v6 & 7)) {
        v11 = (unsigned long)((int)v10 + 1);
        v7 = *(unsigned char *)(v10 + 0xdca60);
      }
      v8 = (int)v6 + 1;
      v2 = *(unsigned char *)(v6 + 0x9c000);
      if (v7 & 1) { // branch-flip
        v3 = *(unsigned char *)((unsigned long)v2 + 0xddd60);
        v1 = (unsigned short *)(a0 + (unsigned long)(v3 + 0x101) * 4);
        sub_43b0(*v1,v1[1]);
        v5 = *(int *)((unsigned long)v3 * 4 + 0x182c0);
        if (v5)
          sub_43b0((unsigned int)v2 - *(int *)((unsigned long)v3 * 4 + 0xddae0),v5);
        v4 = *(unsigned short *)(v9 * 2 + 0x4b000);
        v2 = (0x100 <= v4) ? *(unsigned char *)((unsigned long)((v4 >> 7) + 0x100) + 0xddb60) : *(unsigned char *)((unsigned long)(unsigned int)v4 + 0xddb60); // branch-flip
        v6 = (unsigned long)v2;
        v1 = (unsigned short *)(a1 + v6 * 4);
        sub_43b0(*v1,v1[1]);
        v9 = (unsigned long)((int)v9 + 1);
        v5 = *(int *)(v6 * 4 + 0x18240);
        if (v5)
          sub_43b0((unsigned int)v4 - *(int *)(v6 * 4 + 0xdda60),v5);
      }
      else {
        v1 = (unsigned short *)(a0 + (unsigned long)v2 * 4);
        sub_43b0(*v1,v1[1]);
      }
      if (dat_dca54 <= v8) break;
      v7 >>= 1;
      v6 = (unsigned long)v8;
      v10 = v11;
    }
  }
  sub_43b0(*(unsigned short *)(a0 + 0x400),*(unsigned short *)(a0 + 0x402)); // tail-call
}

// Function: sub_aa20 @ 0xaa20
void sub_aa20(unsigned long a0)
{
  unsigned short v1;
  long v10;
  int v11; // r8d
  int v12; // r9d
  char *v2;
  long v3;
  unsigned short *v4;
  unsigned short *v5; // rax
  unsigned short *v6;
  unsigned long v7; // rbx
  short v8 [20];
  short v9;
  
  dat_dca20 = a0;
  dat_dca28 = 0;
  dat_dca30 = 0;
  if (!dat_dea82) {
    v10 = 0;
    v11 = 0;
    do {
      *(int *)(v10 * 4 + 0xddae0) = v11;
      v12 = 1 << ((unsigned char)*(unsigned int *)(v10 * 4 + 0x182c0) & 0x1f);
      if (1 <= v12) {
        v2 = (char *)((long)v11 + 0xddd60);
        do {
          *v2 = (char)v10;
          v2 = &v2[1];
        } while (v2 != (char *)((long)v11 + 0xddd61 + (unsigned long)(unsigned int)(v12 - 1)));
        v11 += v12;
      }
      v10 += 1;
    } while (v10 != 0x1c);
    v10 = 0;
    v12 = 0;
    *(char *)((long)(v11 + -1) + 0xddd60) = 0x1c;
    do {
      *(int *)(v10 * 4 + 0xdda60) = v12;
      v11 = 1 << ((unsigned char)*(unsigned int *)(v10 * 4 + 0x18240) & 0x1f);
      if (1 <= v11) {
        v2 = (char *)((long)v12 + 0xddb60);
        do {
          *v2 = (char)v10;
          v2 = &v2[1];
        } while (v2 != (char *)((long)v12 + 0xddb61 + (unsigned long)(unsigned int)(v11 - 1)));
        v12 += v11;
      }
      v10 += 1;
    } while (v10 != 0x10);
    v12 >>= 7;
    v10 = 0x10;
    do {
      *(int *)(v10 * 4 + 0xdda60) = v12 << 7;
      v11 = 1 << ((char)*(unsigned int *)(v10 * 4 + 0x18240) - 7U & 0x1fU);
      if (1 <= v11) {
        v3 = (long)v12 + 0xddb60;
        do {
          *(char *)(v3 + 0x100) = (char)v10;
          v3 += 1;
        } while (v3 != (long)v12 + 0xddb61 + (unsigned long)(unsigned int)(v11 - 1));
        v12 += v11;
      }
      v10 += 1;
    } while (v10 != 0x1e);
    v4 = (unsigned short *)0xde9c0;
    do {
      v6 = &v4[1];
      *v4 = 0;
      v4 = v6;
    } while (v6 != (unsigned short *)0xde9e0);
    v4 = (unsigned short *)0xdeb00;
    v6 = (unsigned short *)0xdeb02;
    do {
      v5 = &v6[2];
      *v6 = 8;
      v6 = v5;
    } while (v5 != (unsigned short *)0xded42);
    v6 = (unsigned short *)0xded42;
    do {
      v5 = &v6[2];
      *v6 = 9;
      v6 = v5;
    } while (v5 != (unsigned short *)0xdef02);
    dat_de9d2 += 0x70;
    v6 = (unsigned short *)0xdef02;
    do {
      v5 = &v6[2];
      *v6 = 7;
      v6 = v5;
    } while (v5 != (unsigned short *)0xdef62);
    dat_de9ce += 0x18;
    dat_def66 = 8;
    dat_def6a = 8;
    dat_def6e = 8;
    dat_def72 = 8;
    dat_def76 = 8;
    dat_def7a = 8;
    dat_def7e = 8;
    dat_de9d0 += 0x98;
    dat_def62 = 8;
    v10 = 0;
    v9 = 0;
    do {
      v9 = (v9 + *(short *)(v10 + 0xde9c0)) * 2;
      *(short *)((long)v8 + v10 + 2) = v9;
      v10 += 2;
    } while (v10 != 0x1e);
    do {
      v1 = v4[1];
      if (v1) {
        v9 = v8[(int)(unsigned int)v1];
        v8[(int)(unsigned int)v1] = v9 + 1;
        *v4 = sub_44a0((unsigned short)v9,v1);
      }
      v4 = &v4[2];
    } while (v4 != (unsigned short *)0xdef80);
    v7 = 0;
    do {
      *(unsigned short *)(v7 * 4 + 0xdea82) = 5;
      *(unsigned short *)(v7 * 4 + 0xdea80) = sub_44a0(v7 & 0xffffffff,5);
      v7 += 1;
    } while (v7 != 0x1e);
    sub_9df0(); // tail-call
    return;
  }
}

// Function: sub_add0 @ 0xadd0
long sub_add0(char *a0,unsigned long a1,int a2,int a3)
{
  unsigned short v1;
  unsigned long v10; // rdx
  long v11; // rbx
  unsigned int v2;
  int v3;
  long v4;
  unsigned long v5; // rax
  unsigned short *v6; // rax
  unsigned int v7; // ecx
  unsigned int v8; // edx
  unsigned long v9;
  
  v1 = *dat_dca20;
  *(char *)((unsigned long)dat_dca4c + 0xdca60) = dat_dca49;
  if (v1 == 0xffff) {
    v6 = (unsigned short *)0xdf080;
    v8 = 0;
    do {
      v1 = *v6;
      v6 = &v6[2];
      v8 += v1;
    } while (v6 != (unsigned short *)0xdf09c);
    v6 = (unsigned short *)0xdf09c;
    v7 = 0;
    do {
      v1 = *v6;
      v6 = &v6[2];
      v7 += v1;
    } while (v6 != (unsigned short *)0xdf280);
    v6 = (unsigned short *)0xdf280;
    do {
      v1 = *v6;
      v6 = &v6[2];
      v8 += v1;
    } while (v6 != (unsigned short *)0xdf480);
    *dat_dca20 = (unsigned short)(v8 <= v7 >> 2);
  }
  sub_a090((struct_3 *)0x183c0);
  sub_a090((struct_3 *)0x18380);
  sub_9f80(0xdf080,dat_183e4);
  sub_9f80(0xdef80,dat_183a4);
  sub_a090((struct_3 *)0x18340);
  v9 = 0xf;
  v4 = 0x12;
  do {
    v11 = (long)(int)v4;
    if (*(short *)(v9 * 4 + 0xde9e2)) {
      v4 = (long)((int)v4 * 3 + 0x11);
label_aeb0:
      dat_dca40 = v4 + dat_dca40;
      v5 = dat_dca40 + 10U >> 3;
      v10 = dat_dca38 + 10U >> 3;
      dat_dca28 += a1;
      v9 = v10;
      if (v5 <= v10)
        v9 = v5;
      if ((a1 + 4 <= v9) && (a0)) {
        sub_43b0(a3,3);
        dat_dca30 = (dat_dca30 + 10 & 0xfffffffffffffff8) + (a1 + 4) * 8;
        sub_45c0(a0,a1 & 0xffffffff,1);
        sub_9df0();
      }
      else {
        if (v10 <= v5) { // branch-flip
          sub_43b0(a3 + 2,3);
          sub_a870(0xdeb00,0xdea80);
          dat_dca30 += dat_dca38 + 3;
        }
        else {
          sub_43b0(a3 + 4,3);
          v3 = dat_183e4;
          v2 = dat_183a4;
          sub_43b0(dat_183e4 + -0x100,5);
          sub_43b0(v2,5);
          sub_43b0((int)v11 + -3,4);
          v9 = 0x10;
          for (v4 = 0x13410; sub_43b0(*(unsigned short *)(v9 * 4 + 0xde9e2),3), v11 + 0x13410 != v4; v4 = v4 + 1) {
            v9 = (unsigned long)*(unsigned char *)(v4 + 1);
          }
          sub_a6b0(0xdf080,v3);
          sub_a6b0(0xdef80,v2);
          sub_a870(0xdf080,0xdef80);
          dat_dca30 += dat_dca40 + 3;
        }
        sub_9df0();
      }
      if (a3) { // branch-flip
        sub_44d0();
        v9 = dat_dca30 + 7;
        dat_dca30 = v9;
      }
      else {
        v9 = dat_dca30;
        if ((a2) && (dat_dca30 & 7)) {
          sub_43b0(0,3);
          dat_dca30 = dat_dca30 + 10 & 0xfffffffffffffff8;
          sub_45c0(a0,0,1);
          v9 = dat_dca30;
        }
      }
      return (long)v9 >> 3;
    }
    if (v4 + -1 == 2) {
      v4 = 0x17;
      v11 = 2;
      goto label_aeb0;
    }
    v9 = (unsigned long)*(unsigned char *)(v4 + 0x1340f);
    v4 -= 1;
  } while( true );
}

// Function: sub_b160 @ 0xb160
bool sub_b160(int a0,int a1) // ternary
{
  short *v1;
  int *v2;
  unsigned short *v3;
  unsigned char v4;
  unsigned long v5; // rax
  long v6; // rdx
  
  v5 = (unsigned long)dat_dca54;
  dat_dca54 += 1;
  *(char *)(v5 + 0x9c000) = (char)a1;
  if (a0) { // branch-flip
    a0 -= 1;
    v1 = (short *)((unsigned long)*(unsigned char *)((long)a1 + 0xddd60) * 4 + 0xdf484);
    *v1 = *v1 + 1;
    v4 = (0x100 <= a0) ? *(unsigned char *)((long)((a0 >> 7) + 0x100) + 0xddb60) : *(unsigned char *)((long)a0 + 0xddb60); // branch-flip
    v1 = (short *)((unsigned long)v4 * 4 + 0xdef80);
    *v1 = *v1 + 1;
    v5 = (unsigned long)dat_dca50;
    dat_dca50 += 1;
    *(short *)(v5 * 2 + 0x4b000) = (short)a0;
    dat_dca49 |= dat_dca48;
  }
  else {
    v1 = (short *)((long)a1 * 4 + 0xdf080);
    *v1 = *v1 + 1;
  }
  v4 = dat_dca49;
  dat_dca48 *= '\x02';
  if (!(dat_dca54 & 7)) {
    v5 = (unsigned long)dat_dca4c;
    dat_dca48 = '\x01';
    dat_dca49 = 0;
    dat_dca4c += 1;
    *(unsigned char *)(v5 + 0xdca60) = v4;
  }
  if ((3 <= dat_1801c) && (!(dat_dca54 & 0xfff))) {
    v6 = 0;
    v5 = (unsigned long)dat_dca54 << 3;
    do {
      v2 = (int *)(v6 + 0x18240);
      v3 = (unsigned short *)(v6 + 0xdef80);
      v6 += 4;
      v5 += ((long)*v2 + 5) * (unsigned long)*v3;
    } while (v6 != 0x78);
    if ((dat_dca50 < dat_dca54 >> 1) && (v5 >> 3 < (unsigned long)dat_1906c - dat_19078 >> 1))
      return 1;
  }
  return dat_dca54 == 0x7fff || dat_dca50 == 0x8000;
}

// Function: sub_b320 @ 0xb320
void sub_b320(int a0)
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  int v4;
  unsigned short v5; // dx
  unsigned int v6; // edx
  
  v3 = (unsigned long)dat_df990;
  v6 = (unsigned int)dat_df994 << ((unsigned char)a0 & 0x1f);
  v5 = (unsigned short)v6;
  v4 = dat_df98c;
  v1 = dat_df990;
  if (dat_df98c < a0) {
    do {
      a0 -= v4;
      v6 |= (int)v3 << ((unsigned char)a0 & 0x1f);
      v2 = (unsigned long)dat_1a008;
      dat_df994 = (unsigned short)v6;
      if (dat_1a00c <= dat_1a008) { // branch-flip
        v3 = sub_d6c0(1);
        dat_df990 = (unsigned int)v3;
        if (dat_df990 != 0xffffffff) // branch-flip
          v6 = (unsigned int)dat_df994;
        else {
          dat_df990 = 0;
          v6 = (unsigned int)dat_df994;
          v3 = 0;
        }
      }
      else {
        dat_1a008 += 1;
        v3 = (unsigned long)*(unsigned char *)(v2 + 0x9c000);
        dat_df990 = (unsigned int)*(unsigned char *)(v2 + 0x9c000);
      }
      v5 = (unsigned short)v6;
      v1 = (unsigned int)v3;
      dat_df98c = 8;
      v4 = 8;
    } while (9 <= a0);
  }
  dat_df98c = v4 - a0;
  dat_df994 = v5 | (unsigned short)(v1 >> ((unsigned char)(v4 - a0) & 0x1f));
}

// Function: sub_b3f0 @ 0xb3f0
void sub_b3f0(unsigned int a0,unsigned char *a1,unsigned int a2,long a3)
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
                *(unsigned short *)(v12 * 2 + 0x1a880) = 0;
                *(unsigned short *)(v12 * 2 + 0x2a880) = 0;
                v10 = v12 & 0xffff;
                *v9 = (unsigned short)v12;
                v12 = (unsigned long)((int)v12 + 1);
              }
              v5 = (unsigned int)v13;
              if (1 << (0xf - v1 & 0x1f) & v5) break;
              v9 = (unsigned short *)(v10 * 2 + 0x1a880);
              v13 = (unsigned long)(v5 * 2);
              v19 -= 1;
              if (!v19) goto label_b6b2;
            }
            v9 = (unsigned short *)(v10 * 2 + 0x2a880);
            v13 = (unsigned long)(v5 * 2);
          }
label_b6b2:
          *v9 = (unsigned short)v18;
        }
        v16[v20] = v2 + v3;
      }
      v18 += 1;
      if (a0 == v18)
        return;
    } while( true );
  }
  sub_d3f0("Bad table\n"); // no-return
}

// Function: sub_b700 @ 0xb700
void sub_b700(int a0,unsigned int a1,int a2)
{
  int v1;
  int v10; // eax
  unsigned char *v11; // r14
  unsigned char *v12;
  int v13; // r15d
  long v2; // rax
  char *v3;
  unsigned short *v4; // rax
  unsigned char v5; // cl
  unsigned long v6;
  long v7; // rdx
  int v8; // ebx
  unsigned int v9; // edx
  
  v5 = 0x10 - (char)a1;
  v8 = (int)(unsigned int)dat_df994 >> (v5 & 0x1f);
  sub_b320(a1);
  if (!v8) {
    v9 = (unsigned int)dat_df994;
    sub_b320(a1);
    v3 = (char *)0xdfbc0;
    do {
      *v3 = 0;
      v3 = &v3[1];
    } while (v3 != (char *)((unsigned long)(unsigned int)(a0 - 1) + 0xdfbc1));
    v4 = (unsigned short *)0xdf9a0;
    do {
      *v4 = (short)((int)v9 >> (v5 & 0x1f));
      v4 = &v4[1];
    } while (v4 != (unsigned short *)0xdfba0);
    return;
  }
  v13 = 0;
  do {
    v2 = (long)v13;
    v9 = (a2 + -1) - v13;
    v12 = (unsigned char *)(v2 + 0xdfbc0);
    do {
      if (v8 <= v13) {
        if (v13 < a0) {
          v3 = (char *)((long)v13 + 0xdfbc0);
          do {
            *v3 = 0;
            v3 = &v3[1];
          } while (v3 != (char *)((unsigned long)(unsigned int)((a0 + -1) - v13) + (long)v13 + 0xdfbc1));
        }
        sub_b3f0(a0,(unsigned char *)0xdfbc0,8,0xdf9a0); // tail-call
        return;
      }
      v10 = 3;
      v5 = (unsigned char)(dat_df994 >> 0xd);
      if (dat_df994 >> 0xd == 7) {
        if (dat_df994 & 0x1000) { // branch-flip
          v6 = 0x1000;
          v1 = 7;
          do {
            v10 = v1;
            v6 >>= 1;
            v1 = v10 + 1;
          } while ((unsigned int)dat_df994 & (unsigned int)v6);
          if (0x10 < v1)
            sub_d3f0("Bad table\n"); // no-return
          v10 -= 2;
          v5 = (unsigned char)v1;
        }
        else {
          v5 = 7;
          v10 = 4;
        }
      }
      sub_b320(v10);
      v11 = &v12[1];
      *v12 = v5;
      v13 += 1;
      v12 = v11;
    } while (v11 != (unsigned char *)((unsigned long)v9 + v2 + 0xdfbc1));
    v10 = (int)(unsigned int)dat_df994 >> 0xe;
    sub_b320(2);
    if (v10) {
      v2 = (long)v13;
      v7 = v10 + v2;
      do {
        *(char *)(v2 + 0xdfbc0) = 0;
        v2 += 1;
      } while (v2 != v7);
      v13 += v10;
    }
  } while( true );
}

// Function: sub_b920 @ 0xb920
unsigned long sub_b920(unsigned int a0,unsigned int a1) // return-dupe, ternary
{
  unsigned short v1;
  unsigned long v10; // rsi
  int v11;
  int v12; // r13d
  unsigned long v13;
  int v14; // r15d
  long v15; // rcx
  bool v2;
  unsigned char v3;
  unsigned int v4;
  unsigned long v5;
  unsigned int v6; // edx
  int v7;
  char *v8;
  unsigned short *v9; // rdx
  
  dat_df994 = 0;
  dat_df990 = 0;
  dat_df98c = 0;
  dat_1a010 = a1;
  dat_1a014 = a0;
  sub_b320(0x10);
  dat_dfba0 = 0;
  dat_df988 = 0;
  dat_df984 = 0;
label_b98e:
  v5 = 0;
  v2 = 0;
  v4 = dat_df980;
  do {
    v13 = v5 & 0xffffffff;
    dat_df988 -= 1;
    if (dat_df988 <= -1) {
      if (v2)
        dat_df980 = v4;
      goto label_b9f0;
    }
    v13 = (unsigned long)v4;
    v4 = v4 + 1 & 0x1fff;
    *(char *)(v5 + 0x3b000) = *(char *)(v13 + 0x3b000);
    v5 += 1;
    v2 = 1;
  } while (v5 != 0x2000);
  v13 = 0x2000;
  dat_df980 = v4;
  goto label_ba98;
  while( true ) {
    v4 = v12 + 1;
    *(char *)(v13 + 0x3b000) = (char)v5;
    v13 = (unsigned long)v4;
    if (v4 == 0x2000) break;
label_b9f0:
    v4 = (unsigned int)dat_df994;
    v12 = (int)v13;
    if (!dat_dfba0) {
      sub_b320(0x10);
      dat_dfba0 = v4;
      if (!v4) {
        dat_df984 = 1;
        if (!v12) {
          dat_df984 = 1;
          dat_dfba0 = 0;
          return 0;
        }
        break;
      }
      sub_b700(0x13,5,3);
      v11 = 0;
      v14 = (int)(unsigned int)dat_df994 >> 7;
      sub_b320(9);
      if (v14) { // branch-flip
        do {
          while( true ) {
            v1 = *(unsigned short *)((unsigned long)(unsigned char)(dat_df994 >> 8) * 2 + 0xdf9a0);
            v5 = (unsigned long)v1;
            if (0x13 <= v1) {
              v4 = 0x80;
              do {
                v1 = (v4 & dat_df994) ? *(unsigned short *)((long)((int)v5 + 0x8000) * 2 + 0x1a880) : *(unsigned short *)(v5 * 2 + 0x1a880); // branch-flip
                v5 = (unsigned long)v1;
                v4 >>= 1;
              } while (0x13 <= v1);
            }
            v4 = (unsigned int)v5;
            v15 = (long)(int)v4;
            sub_b320((unsigned char)*(char *)(v15 + 0xdfbc0));
            if (v4 <= 2) break;
            v15 = (long)v11;
            v11 += 1;
            *(char *)(v15 + 0x5b000) = (char)v5 + '\xfe';
            if (v14 <= v11) goto label_bc2a;
          }
          v7 = 0;
          if (v4) {
            v6 = (unsigned int)dat_df994;
            if (v4 != 1) { // branch-flip
              sub_b320(9);
              v7 = ((int)v6 >> 7) + 0x13;
              v15 = (long)v7;
            }
            else {
              sub_b320(4);
              v7 = ((int)v6 >> 0xc) + 2;
              v15 = (long)v7;
            }
          }
          v8 = (char *)((long)v11 + 0x5b000);
          do {
            *v8 = 0;
            v8 = &v8[1];
          } while ((char *)((long)v11 + 0x5b001 + v15) != v8);
          v11 = v7 + 1 + v11;
        } while (v11 < v14);
label_bc2a:
        if (v11 <= 0x1fd) {
          v8 = (char *)((long)v11 + 0x5b000);
          do {
            *v8 = 0;
            v8 = &v8[1];
          } while (v8 != (char *)((long)v11 + 0x5b001 + (unsigned long)(unsigned int)(0x1fd - v11)));
        }
        sub_b3f0(0x1fe,(unsigned char *)0x5b000,0xc,0x4b000);
      }
      else {
        v4 = (unsigned int)dat_df994;
        sub_b320(9);
        v8 = (char *)0x5b000;
        do {
          *v8 = 0;
          v8 = &v8[1];
        } while (v8 != (char *)0x5b1fe);
        v9 = (unsigned short *)0x4b000;
        do {
          *v9 = (short)((int)v4 >> 7);
          v9 = &v9[1];
        } while (v9 != (unsigned short *)0x4d000);
      }
      sub_b700(0xe,4,0xffffffff);
      v4 = (unsigned int)dat_df994;
    }
    dat_dfba0 -= 1;
    v1 = *(unsigned short *)((unsigned long)(v4 >> 4) * 2 + 0x4b000);
    v5 = (unsigned long)v1;
    if (0x1fe <= v1) {
      v6 = 8;
      do {
        while (v6 & v4) {
          v6 >>= 1;
          v1 = *(unsigned short *)((unsigned long)((int)v5 + 0x8000) * 2 + 0x1a880);
          v5 = (unsigned long)v1;
          if (v1 <= 0x1fd) goto label_ba66;
        }
        v1 = *(unsigned short *)(v5 * 2 + 0x1a880);
        v5 = (unsigned long)v1;
        v6 >>= 1;
      } while (0x1fe <= v1);
    }
label_ba66:
    sub_b320((unsigned char)*(char *)(v5 + 0x5b000));
    if (0x100 <= (unsigned int)v5) {
      dat_df988 = (unsigned int)v5 - 0xfd;
      v1 = *(unsigned short *)((unsigned long)(unsigned char)(dat_df994 >> 8) * 2 + 0xdf9a0);
      v5 = (unsigned long)v1;
      if (0xe <= v1) {
        v4 = 0x80;
        do {
          while (v4 & dat_df994) {
            v4 >>= 1;
            v1 = *(unsigned short *)((unsigned long)((int)v5 + 0x8000) * 2 + 0x1a880);
            v5 = (unsigned long)v1;
            if (v1 <= 0xd) goto label_bd20;
          }
          v1 = *(unsigned short *)(v5 * 2 + 0x1a880);
          v5 = (unsigned long)v1;
          v4 >>= 1;
        } while (0xe <= v1);
      }
label_bd20:
      sub_b320((unsigned char)*(char *)(v5 + 0xdfbc0));
      v11 = 0;
      if ((int)v5) {
        v11 = (int)v5 + -1;
        v4 = (unsigned int)dat_df994;
        v3 = (unsigned char)v11;
        sub_b320(v11);
        v11 = ((int)v4 >> (0x10 - v3 & 0x1f)) + (1 << (v3 & 0x1f));
      }
      v2 = 0;
      dat_df980 = (v12 + -1) - v11 & 0x1fff;
      v5 = v13;
      v4 = dat_df980;
      while (dat_df988 = dat_df988 + -1, 0 <= dat_df988) {
        v10 = (unsigned long)v4;
        v6 = (int)v5 + 1;
        v13 = (unsigned long)v6;
        v4 = v4 + 1 & 0x1fff;
        *(char *)(v5 + 0x3b000) = *(char *)(v10 + 0x3b000);
        v2 = 1;
        v5 = v13;
        if (v6 == 0x2000) {
          dat_df980 = v4;
          goto label_ba98;
        }
      }
      v13 = v5;
      if (v2)
        dat_df980 = v4;
      goto label_b9f0;
    }
  }
label_ba98:
  sub_d5a0(a1,(void *)0x3b000,v13);
  if (dat_df984)
    return 0;
  goto label_b98e;
}

// Function: sub_be90 @ 0xbe90
unsigned long sub_be90(unsigned int a0,unsigned int a1) // return-dupe x2
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
  
  v5 = (unsigned long)dat_1a008;
  if (dat_1a00c <= dat_1a008) // branch-flip
    dat_18024 = sub_d6c0(0);
  else {
    dat_1a008 += 1;
    dat_18024 = (unsigned int)*(unsigned char *)(v5 + 0x9c000);
  }
  dat_183e8 = dat_18024 & 0x80;
  if (dat_18024 & 0x60) {
    if (!dat_1a85c)
      sub_f6b0(stderr,"\n%s: %s: warning, unknown flags 0x%x\n",dat_1a850,0x1a420);
    if (!dat_1a84c)
      dat_1a84c = 2;
  }
  dat_18024 &= 0x1f;
  v3 = 1L << (signed char)dat_18024;
  if (0x10 < dat_18024) {
    sub_f6b0(stderr,"\n%s: %s: compressed with %d bits, can only handle %d bits\n",dat_1a850,0x1a420,dat_18024,0x10);
    dat_1a84c = 1;
    return 1;
  }
  dat_1a880 = 0;
  v5 = (unsigned long)dat_1a00c;
  dat_1a978 = 0;
  v15 = (unsigned long)(dat_1a008 * 8);
  v20 = 0x101 - (unsigned long)(dat_183e8 == 0);
  v26 = dat_1a00c;
  v7 = 0x1f;
  v12 = (unsigned long *)0x1a888;
  while (v7) {
    v1 = &v12[1];
    *v12 = 0;
    v7 -= 1;
    v12 = v1;
  }
  v7 = 0xff;
  do {
    *(char *)(v7 + 0x3b000) = (char)v7;
    v19 = v7 != 0;
    v7 -= 1;
  } while (v19);
  v25 = 9;
  v16 = 0;
  v23 = 0x1ff;
  v27 = 0x1ff;
  v22 = 0;
  v17 = 0xffffffffffffffff;
label_bfec:
  v14 = (unsigned int)(v15 >> 3);
  v10 = (unsigned int)v5;
  v18 = v17;
  if (v14 <= v10) goto label_c258;
label_bff9:
  dat_1a00c = 0;
  v11 = (void *)0x9c000;
label_c00a:
  v26 = sub_d190(a0,v11,0x40000);
  if (v26 == 0xffffffff)
    sub_d4e0(); // no-return
  dat_1a828 += (int)v26;
  dat_1a00c += v26;
  v5 = (unsigned long)dat_1a00c;
label_c049:
  v14 = v25;
  v10 = (unsigned int)v5;
  v17 = v18;
  if (v26) { // branch-flip
    v15 = 0;
    v24 = (v5 - v5 % (unsigned long)v25) * 8;
    if (v24 <= 0) goto label_bfec;
  }
  else {
    v24 = v5 * 8 - (long)(int)(v25 - 1);
    if (v24 <= 0) {
label_c3f5:
      if (v16 > 0) {
        sub_d5a0(a1,(void *)0x5b000,v16);
        return 0;
      }
      return 0;
    }
  }
  if (v23 < v20) {
    v7 = -1;
    goto label_c1ef;
  }
  v15 = 0;
label_c0a8:
  v7 = v15 >> 3;
  v2 = (unsigned char)v15;
  v15 += (long)(int)v25;
  v18 = (long)((unsigned long)*(unsigned char *)(v7 + 0x9c001) << 8 | (unsigned long)*(unsigned char *)(v7 + 0x9c002) << 0x10 | (unsigned long)*(unsigned char *)(v7 + 0x9c000)) >> (v2 & 7) & (unsigned long)v27;
  if (v17 != 0xffffffffffffffff) { // branch-flip
    if (v18 != 0x100) { // branch-flip
      if (v20 <= (long)v18) {
        if (v20 < (long)v18) {
          if (1 <= v16)
            sub_d5a0(a1,(void *)0x5b000,v16);
          v13 = "corrupt input.";
          if (!dat_1a860)
            v13 = "corrupt input. Use zcat to recover some data.";
          sub_d3f0(v13); // no-return
        }
        goto label_c118;
      }
      v9 = (char *)0x5affe;
      v5 = v18;
    }
    else {
      if (dat_183e8) {
        dat_1a880 = 0;
        dat_1a978 = 0;
        v7 = 0x1f;
        v12 = (unsigned long *)0x1a888;
        while (v7) {
          v1 = &v12[1];
          *v12 = 0;
          v7 -= 1;
          v12 = v1;
        }
        v27 = 0x1ff;
        v20 = 0x100;
        v5 = (unsigned long)dat_1a00c;
        v25 = 9;
        v23 = 0x1ff;
        v7 = (long)(int)(v14 << 3);
        v15 = (v15 - 1) + (v7 - (long)((v15 - 1) + v7) % v7);
        goto label_bfec;
      }
      v5 = 0x100;
      v9 = (char *)0x5affe;
      if (v20 != 0x100) goto label_c138;
label_c118:
      v9 = (char *)0x5affd;
      dat_5affd = (char)v22;
      v5 = v17;
    }
    for (; 0xff < (long)v5; v5 = (unsigned long)*(unsigned short *)(v5 * 2 + 0x1a880)) {
label_c138:
      v9 = &v9[-1];
      *v9 = *(char *)(v5 + 0x3b000);
    }
    v2 = *(unsigned char *)(v5 + 0x3b000);
    v4 = &v9[-1];
    v9[-1] = v2;
    v7 = 0x5affe - (long)v4;
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
          memcpy((void *)(v7 + 0x5b000),v4,(long)v6);
        }
        if (0x40000 <= v16) {
          sub_d5a0(a1,(void *)0x5b000,v16);
          v16 = 0;
        }
        v4 = &v4[v6];
        v7 = 0x5affe - (long)v4;
      } while (1 <= (int)v7);
    }
    else {
      memcpy((void *)((long)v16 + 0x5b000),v4,(long)(int)v7);
      v16 = v6;
    }
    v10 = dat_1a00c;
    if (v20 < v3) {
      v21 = (unsigned short)v17;
      *(unsigned short *)(v20 * 2 + 0x1a880) = v21;
      *(unsigned char *)(v20 + 0x3b000) = v2;
      v20 += 1;
    }
  }
  else {
    if (0xff < v18)
      sub_d3f0("corrupt input."); // no-return
    v7 = (long)v16;
    v22 = (unsigned int)v18;
    v16 += 1;
    *(char *)(v7 + 0x5b000) = (char)v18;
    v10 = dat_1a00c;
  }
  v17 = v18;
  dat_1a00c = v10;
  if (v24 <= v15) {
    if (!v26) goto label_c3f5;
    v5 = (unsigned long)v10;
    goto label_bfec;
  }
  if (v23 < v20) goto label_c1e5;
  goto label_c0a8;
label_c1e5:
  v7 = v15 - 1;
label_c1ef:
  v6 = v25 * 8;
  v25 += 1;
  v8 = (long)v6;
  v23 = v3;
  if (dat_18024 != v25)
    v23 = (1L << ((unsigned char)v25 & 0x3f)) + -1;
  v27 = (1 << ((unsigned char)v25 & 0x1f)) - 1;
  v14 = (unsigned int)(v7 + (v8 - (v8 + v7) % v8) >> 3);
  if (v10 < v14) goto label_bff9;
label_c258:
  dat_1a00c = v10 - v14;
  v5 = (unsigned long)dat_1a00c;
  if (1 <= (int)dat_1a00c) {
    v9 = (char *)0x9c000;
    do {
      v4 = &v9[1];
      *v9 = v9[(int)v14];
      v9 = v4;
    } while ((char *)((unsigned long)(dat_1a00c - 1) + 0x9c001) != v4);
  }
  if (dat_1a00c <= 0x3f) goto label_c297;
  goto label_c049;
label_c297:
  v11 = (void *)(v5 + 0x9c000);
  goto label_c00a;
}

// Function: sub_c590 @ 0xc590
unsigned long sub_c590(unsigned int a0,unsigned int a1)
{
  int *v1;
  char *v10; // rax
  unsigned long v11; // rax
  char *v12; // rdx
  int v13;
  unsigned int v14; // esi
  long v15; // rax
  long v16; // r12
  long v17;
  int v18; // edx
  unsigned int v19;
  long v2;
  long v20;
  unsigned char v3;
  unsigned int v4;
  char v5;
  unsigned int v6;
  unsigned int v7; // eax
  unsigned long v8;
  unsigned long v9; // rax
  
  v8 = 0;
  v13 = 4;
  v11 = (unsigned long)dat_1a008;
  dat_dfe88 = 0;
  dat_1a010 = a1;
  dat_1a014 = a0;
  v6 = dat_1a00c;
  do {
    if (v6 <= (unsigned int)v11) { // branch-flip
      v9 = sub_d6c0(0);
      if ((int)v9 < 0) {
        sub_d3f0("invalid compressed data -- unexpected end of file"); // return-dupe, no-return
      }
      v4 = dat_1a008;
      v6 = dat_1a00c;
    }
    else {
      dat_1a008 = (unsigned int)v11 + 1;
      v9 = (unsigned long)*(unsigned char *)(v11 + 0x9c000);
      v4 = dat_1a008;
    }
    v11 = (unsigned long)v4;
    v8 = v9 & 0xff | v8 << 8;
    v13 -= 1;
    dat_dfe88 = v8;
  } while (v13);
  if (v6 <= v4) { // branch-flip
    v6 = sub_d6c0(0);
    if ((int)v6 < 0)
      sub_d3f0("invalid compressed data -- unexpected end of file");
    dat_dfe80 = v6 & 0xff;
  }
  else {
    dat_1a008 = v4 + 1;
    dat_dfe80 = (unsigned int)*(unsigned char *)(v11 + 0x9c000);
  }
  if (0x18 < dat_dfe80 - 1)
    sub_d3f0("invalid compressed data -- Huffman code bit length out of range"); // no-return
  v15 = 1;
  v18 = 0;
  v13 = 1;
  do {
    v8 = (unsigned long)dat_1a008;
    if (dat_1a00c <= dat_1a008) { // branch-flip
      v6 = sub_d6c0(0);
      if ((int)v6 < 0) {
        sub_d3f0("invalid compressed data -- unexpected end of file");
      }
      v6 &= 0xff;
    }
    else {
      dat_1a008 += 1;
      v6 = (unsigned int)*(unsigned char *)(v8 + 0x9c000);
    }
    *(unsigned int *)(v15 * 4 + 0xdfc80) = v6;
    if ((int)(v13 - (unsigned int)(dat_dfe80 == (unsigned int)v15)) < (int)v6) {
      sub_d3f0("too many leaves in Huffman tree"); // no-return, return-dupe
    }
    v15 += 1;
    v18 += v6;
    v13 = (v13 - v6) * 2 + 1;
  } while ((int)v15 <= (int)dat_dfe80);
  if (0xff < v18)
    sub_d3f0("too many leaves in Huffman tree");
  v15 = (long)(int)dat_dfe80;
  v13 = *(int *)(v15 * 4 + 0xdfc80);
  *(int *)(v15 * 4 + 0xdfc80) = v13 + 1;
  if (1 <= (int)dat_dfe80) { // branch-flip
    v20 = 1;
    v15 = 0;
    v6 = dat_dfe80;
    do {
      *(int *)(v20 * 4 + 0xdfd00) = (int)v15;
      v13 = *(int *)(v20 * 4 + 0xdfc80);
      if (1 <= v13) {
        v2 = v15 + 2 + (unsigned long)(unsigned int)(v13 - 1);
        v17 = (long)((int)v15 + 1);
        do {
          while( true ) {
            v8 = (unsigned long)dat_1a008;
            v15 = (long)(int)v17;
            if (dat_1a008 < dat_1a00c) break;
            v13 = sub_d6c0(0);
            if (v13 < 0) {
              sub_d3f0("invalid compressed data -- unexpected end of file");
            }
            *(char *)(v17 + 0xdfd7f) = (char)v13;
            v17 += 1;
            if (v2 == v17) goto label_c787;
          }
          dat_1a008 += 1;
          v16 = v17 + 1;
          *(char *)(v17 + 0xdfd7f) = *(char *)(v8 + 0x9c000);
          v17 = v16;
        } while (v2 != v16);
label_c787:
        v6 = dat_dfe80;
      }
      v20 += 1;
    } while ((int)v20 <= (int)v6);
    v15 = (long)(int)v6;
    v4 = *(unsigned int *)(v15 * 4 + 0xdfc80);
    v13 = v4 + 1;
    *(int *)(v15 * 4 + 0xdfc80) = v13;
    if (1 <= (int)v6) {
      v18 = 0;
      while( true ) {
        v18 >>= 1;
        *(int *)(v15 * 4 + 0xdfc00) = v18;
        v1 = (int *)(v15 * 4 + 0xdfd00);
        *v1 = *v1 - v18;
        v15 -= 1;
        v18 += v13;
        if ((int)v15 <= 0) break;
        v13 = *(int *)(v15 * 4 + 0xdfc80);
      }
      if (v18 >> 1 == 1) {
        v15 = 1;
        dat_dfbf0 = v6;
        if (0xd <= (int)v6)
          dat_dfbf0 = 0xc;
        v13 = 1 << ((unsigned char)dat_dfbf0 & 0x1f);
        v10 = (char *)((long)v13 + 0x5b000);
        do {
          v18 = *(int *)(v15 * 4 + 0xdfc80) << ((unsigned char)dat_dfbf0 - (char)v15 & 0x1f);
          if (v18) {
            v12 = &v10[~(unsigned long)(unsigned int)(v18 - 1)];
            do {
              v10 = &v10[-1];
              *v10 = (char)v15;
            } while (v10 != v12);
          }
          v15 += 1;
        } while ((int)v15 <= (int)dat_dfbf0);
        if ((char *)0x5b001 <= v10) {
          do {
            v10 = &v10[-1];
            *v10 = '\0';
          } while (v10 != (char *)0x5b000);
        }
        dat_dfbe0 = 0;
        v7 = v13 - 1;
        dat_dfbe8 = 0;
        v8 = 0;
        v6 = dat_dfbf0;
        v13 = 0;
        do {
          if ((int)v6 <= v13) {
            do {
              v14 = v7 & (unsigned int)(v8 >> ((char)v13 - (char)v6 & 0x3fU));
              v3 = *(unsigned char *)((unsigned long)v14 + 0x5b000);
              if (v3) { // branch-flip
                v14 >>= (char)v6 - v3 & 0x1f;
                v6 = (unsigned int)v3;
              }
              else if (v14 < *(unsigned int *)((long)(int)v6 * 4 + 0xdfc00)) {
                v15 = (long)(int)(v6 + 1);
                v19 = v7;
                while( true ) {
                  v6 = (unsigned int)v15;
                  v19 = v19 * 2 + 1;
                  while (v13 < (int)v6) {
                    v11 = (unsigned long)dat_1a008;
                    if (dat_1a00c <= dat_1a008) { // branch-flip
                      v11 = sub_d6c0(0);
                      if ((int)v11 <= -1) {
                        sub_d3f0("invalid compressed data -- unexpected end of file");
                      }
                    }
                    else {
                      dat_1a008 += 1;
                      v11 = (unsigned long)*(unsigned char *)(v11 + 0x9c000);
                    }
                    v8 = v11 & 0xff | v8 << 8;
                    dat_dfbe0 += 8;
                    dat_dfbe8 = v8;
                    v13 = dat_dfbe0;
                  }
                  v5 = (char)v15;
                  v20 = v15 * 4;
                  v15 += 1;
                  v14 = (unsigned int)(v8 >> ((char)v13 - v5 & 0x3fU)) & v19;
                  if (*(unsigned int *)(v20 + 0xdfc00) <= v14) break;
                  v13 = dat_dfbe0;
                }
              }
              if ((v14 == v4) && (dat_dfe80 == v6)) {
                sub_d640();
                if (dat_1a820 == dat_dfe88)
                  return 0;
                sub_d3f0("invalid compressed data--length error"); // no-return
              }
              v8 = (unsigned long)dat_1a004;
              dat_1a004 += 1;
              *(char *)(v8 + 0x3b000) = *(char *)((unsigned long)(v14 + *(int *)((long)(int)v6 * 4 + 0xdfd00)) + 0xdfd80);
              if (dat_1a004 == 0x8000)
                sub_d640();
              dat_dfbe0 -= v6;
              v8 = dat_dfbe8;
              v6 = dat_dfbf0;
              v13 = dat_dfbe0;
            } while ((int)dat_dfbf0 <= dat_dfbe0);
          }
          v11 = (unsigned long)dat_1a008;
          if (dat_1a00c <= dat_1a008) { // branch-flip
            v11 = sub_d6c0(0);
            if ((int)v11 < 0) {
              sub_d3f0("invalid compressed data -- unexpected end of file");
            }
            v6 = dat_dfbf0;
          }
          else {
            dat_1a008 += 1;
            v11 = (unsigned long)*(unsigned char *)(v11 + 0x9c000);
            dat_dfbe0 = v13;
          }
          dat_dfbe0 += 8;
          v8 = v11 & 0xff | v8 << 8;
          dat_dfbe8 = v8;
          v13 = dat_dfbe0;
        } while( true );
      }
    }
  }
  else {
    *(int *)(v15 * 4 + 0xdfc80) = v13 + 2;
  }
  sub_d3f0("too few leaves in Huffman tree"); // no-return
}

// Function: sub_cb20 @ 0xcb20
unsigned long sub_cb20(unsigned int a0)
{
  unsigned char v1;
  unsigned long v2; // rax
  char *v3; // rsi
  
  v2 = (unsigned long)dat_1a008;
  dat_1a014 = a0;
  dat_1a008 = *(unsigned short *)(v2 + 0x9c01a) + 0x1e + (unsigned int)*(unsigned short *)(v2 + 0x9c01c) + dat_1a008;
  if ((dat_1a008 <= dat_1a00c) && (*(int *)(v2 + 0x9c000) == 0x4034b50)) {
    v3 = "\n%s: %s: first entry not deflated or stored -- use unzip\n";
    dat_18020 = (unsigned int)*(unsigned char *)(v2 + 0x9c008);
    if (!(*(unsigned char *)(v2 + 0x9c008) & 0xf7)) {
      v1 = *(unsigned char *)(v2 + 0x9c006);
      dat_dfea0 = v1 & 1;
      if (!(v1 & 1)) {
        dat_dfe9c = 1;
        dat_dfe98 = v1 >> 3 & 1;
        return 0;
      }
      v3 = "\n%s: %s: encrypted file -- use unzip\n";
    }
  }
  else {
    v3 = "\n%s: %s: not a valid zip file\n";
  }
  sub_f6b0(stderr,v3,dat_1a850,0x1a420);
  dat_1a84c = 1;
  return 1;
}

// Function: sub_cc20 @ 0xcc20
unsigned int sub_cc20(unsigned int a0,unsigned int a1) // return-dupe
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
  
  dat_1a010 = a1;
  dat_1a014 = a0;
  sub_d0d0(NULL,0);
  if (dat_dfe9c) { // branch-flip
    if (dat_dfe98) { // branch-flip
      v11 = 0;
      v10 = 0;
    }
    else {
      v10 = dat_9c00e;
      v11 = dat_9c016;
    }
    v4 = (unsigned long)v10;
    if (dat_18020 == 8) goto label_cd42;
    if (dat_18020) {
      sub_d3f0("internal error, invalid method"); // no-return, return-dupe
    }
    v3 = (unsigned long)dat_9c012;
    v12 = (unsigned long)dat_9c016;
    if (dat_dfea0)
      v3 -= 0xc;
    if (v12 != v3) {
      sub_f6b0(stderr,"len %lu, siz %lu\n",v12);
      sub_d3f0("invalid compressed data--length mismatch"); // no-return
    }
    for (; v12; v12 = v12 - 1) {
      while( true ) {
        v3 = (unsigned long)dat_1a008;
        if (dat_1a00c <= dat_1a008) // branch-flip
          v1 = sub_d6c0(0);
        else {
          dat_1a008 += 1;
          v1 = *(char *)(v3 + 0x9c000);
        }
        v3 = (unsigned long)dat_1a004;
        dat_1a004 += 1;
        *(char *)(v3 + 0x3b000) = v1;
        if (dat_1a004 == 0x8000) break;
        v12 -= 1;
        if (!v12) goto label_cf30;
      }
      sub_d640();
    }
label_cf30:
    sub_d640();
    if (!dat_dfe9c) goto label_cd68;
label_cf45:
    if (dat_dfe98) {
      v5 = &v7;
      do {
        while (v4 = (unsigned long)dat_1a008, dat_1a00c <= dat_1a008) {
          v6 = (char *)((long)v5 + 1);
          *(char *)v5 = sub_d6c0(0);
          v5 = (unsigned int *)v6;
          if (v6 == v9) goto label_cfab;
        }
        dat_1a008 += 1;
        v6 = (char *)((long)v5 + 1);
        *(char *)v5 = *(char *)(v4 + 0x9c000);
        v5 = (unsigned int *)v6;
      } while (v6 != v9);
label_cfab:
      v4 = (unsigned long)v14;
      v11 = v15;
    }
  }
  else {
    if (dat_18020 != 8)
      sub_d3f0("internal error, invalid method");
    v11 = 0;
    v4 = 0;
label_cd42:
    v2 = sub_9a50();
    if (v2 == 3)
      sub_d430(); // no-return
    if (v2)
      sub_d3f0("invalid compressed data--format violated"); // no-return
    if (dat_dfe9c) goto label_cf45;
label_cd68:
    v5 = &v7;
    do {
      while (v4 = (unsigned long)dat_1a008, dat_1a00c <= dat_1a008) {
        v6 = (char *)((long)v5 + 1);
        *(char *)v5 = sub_d6c0(0);
        v5 = (unsigned int *)v6;
        if (v6 == v8) goto label_cdbb;
      }
      dat_1a008 += 1;
      v6 = (char *)((long)v5 + 1);
      *(char *)v5 = *(char *)(v4 + 0x9c000);
      v5 = (unsigned int *)v6;
    } while (v6 != v8);
label_cdbb:
    v4 = (unsigned long)v7;
    v11 = v14;
  }
  v13 = sub_d0d0((unsigned char *)0x5b000,0) != v4;
  if (v13)
    sub_f6b0(stderr,"\n%s: %s: invalid compressed data--crc error\n",dat_1a850,0x1a420);
  if (dat_1a820 != v11) { // branch-flip
    sub_f6b0(stderr,"\n%s: %s: invalid compressed data--length error\n",dat_1a850,0x1a420);
    if (!dat_dfe9c) goto label_cecb;
    v13 = 1;
label_ce4a:
    if ((dat_1a008 + 4 < dat_1a00c) && (((long)(int)(unsigned int)*(unsigned short *)((unsigned long)dat_1a008 + 0x9c002) << 0x10 | (long)(int)(unsigned int)*(unsigned short *)((unsigned long)dat_1a008 + 0x9c000)) == 0x4034b50)) {
      if (!dat_1a860) {
        sub_f6b0(stderr,"%s: %s has more than one entry -- unchanged\n",dat_1a850,0x1a420);
        dat_dfe9c = 0;
        goto label_cecb;
      }
      if (!dat_1a85c)
        sub_f6b0(stderr,"%s: %s has more than one entry--rest ignored\n",dat_1a850,0x1a420);
      if (!dat_1a84c)
        dat_1a84c = 2;
    }
  }
  else if (dat_dfe9c) goto label_ce4a;
  dat_dfe9c = 0;
  if (!v13) {
    dat_dfe9c = 0;
    dat_dfe98 = 0;
    dat_dfe90 = v4;
    return 0;
  }
label_cecb:
  dat_dfe98 = 0;
  dat_1a84c = 1;
  if (dat_1a858) {
    dat_1a84c = 1;
    dat_dfe90 = v4;
    dat_dfe98 = 0;
    return 1;
  }
  dat_dfe90 = v4;
  sub_8440(); // no-return
}

// Function: sub_d0d0 @ 0xd0d0
unsigned long sub_d0d0(unsigned char *a0,unsigned int a1) // early-return
{
  unsigned char *v1;
  unsigned char v2;
  
  if (!a0) {
    dat_183f0 = 0xffffffff;
    return 0;
  }
  if (a1) {
    v1 = &a0[a1];
    do {
      v2 = *a0;
      a0 = &a0[1];
      dat_183f0 = dat_183f0 >> 8 ^ *(unsigned long *)((unsigned long)(((unsigned int)v2 ^ (unsigned int)dat_183f0) & 0xff) * 8 + 0x13800);
    } while (a0 != v1);
  }
  return dat_183f0 ^ 0xffffffff;
}

// Function: sub_d130 @ 0xd130
unsigned long sub_d130(void)
{
  return dat_183f0 ^ 0xffffffff;
}

// Function: sub_d150 @ 0xd150
void sub_d150(void)
{
  dat_1a004 = 0;
  dat_1a008 = 0;
  dat_1a00c = 0;
  dat_1a820 = 0;
  dat_1a828 = 0;
}

// Function: sub_d190 @ 0xd190
unsigned long sub_d190(int a0,void *a1,unsigned int a2)
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
      v1 = sub_e0f0(a0,3);
      if (0 <= (int)(unsigned int)v1) {
        if (v1 & 0x800) { // branch-flip
          if (sub_e0f0(a0,4,(unsigned int)v1 & 0xfffff7ff) != -1)
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

// Function: sub_d230 @ 0xd230
unsigned char * sub_d230(unsigned char *a0)
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

// Function: sub_d290 @ 0xd290
char * sub_d290(char *a0)
{
  return sub_dfd0(a0); // tail-call
}

// Function: sub_d2a0 @ 0xd2a0
void sub_d2a0(char *a0)
{
  unlink(a0); // tail-call
}

// Function: sub_d2b0 @ 0xd2b0
char * sub_d2b0(unsigned int *a0,unsigned long *a1,char *a2)
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
    v2 = (char *)sub_f640(v2);
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
      v4 = (unsigned long *)sub_f540((long)(v7 + 2),8);
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

// Function: sub_d3f0 @ 0xd3f0
void sub_d3f0(char *a0)
{
  sub_f6b0(stderr,"\n%s: %s: %s\n",dat_1a850,0x1a420,a0);
  sub_8440(); // no-return
}

// Function: sub_d430 @ 0xd430
void sub_d430(void)
{
  sub_f6b0(stderr,"\n%s: memory_exhausted\n",dat_1a850);
  sub_8440(); // no-return
}

// Function: sub_d460 @ 0xd460
void sub_d460(char *a0) // return-dupe
{
  if (!dat_1a85c) {
    sub_f6b0(stderr,"%s: %s: warning: %s\n",dat_1a850,0x1a420,a0);
    if (dat_1a84c)
      return;
    dat_1a84c = 2;
    return;
  }
  if (dat_1a84c)
    return;
  dat_1a84c = 2;
}

// Function: sub_d4e0 @ 0xd4e0
void sub_d4e0(void)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  sub_f6b0(stderr,"\n%s: ",dat_1a850);
  if (v1) { // branch-flip
    *v2 = v1;
    perror((char *)0x1a420);
  }
  else {
    sub_f6b0(stderr,"%s: unexpected end of file\n",0x1a420);
  }
  sub_8440(); // no-return
}

// Function: sub_d550 @ 0xd550
void sub_d550(void)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  sub_f6b0(stderr,"\n%s: ",dat_1a850);
  *v2 = v1;
  perror((char *)0x1a020);
  sub_8440(); // no-return
}

// Function: sub_d5a0 @ 0xd5a0
void sub_d5a0(int a0,void *a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned long v2;
  unsigned int v3; // ebp
  
  a2 &= 0xffffffff;
  dat_1a820 += a2;
  if (dat_1a858)
    return;
  while( true ) {
    v3 = (unsigned int)a2;
    v2 = 0x7fffffff;
    if (v3 <= 0x7fffffff)
      v2 = a2;
    v2 = write(a0,a1,v2);
    v1 = (unsigned int)v2;
    if (v1 == v3) break;
    if (v1 == 0xffffffff)
      sub_d550(); // no-return
    a2 = (unsigned long)(v3 - v1);
    a1 = (void *)((long)a1 + (v2 & 0xffffffff));
  }
}

// Function: sub_d600 @ 0xd600
void sub_d600(void)
{
  if (!dat_1a004)
    return;
  sub_d5a0(dat_1a010,(void *)0x5b000);
  dat_1a004 = 0;
}

// Function: sub_d640 @ 0xd640
void sub_d640(void)
{
  unsigned char v1;
  unsigned char *v2; // rdx
  
  if (!dat_1a004)
    return;
  v2 = (unsigned char *)0x3b000;
  do {
    v1 = *v2;
    v2 = &v2[1];
    dat_183f0 = dat_183f0 >> 8 ^ *(unsigned long *)((unsigned long)(((unsigned int)v1 ^ (unsigned int)dat_183f0) & 0xff) * 8 + 0x13800);
  } while (v2 != (unsigned char *)((unsigned long)dat_1a004 + 0x3b000));
  sub_d5a0(dat_1a010,(void *)0x3b000,dat_1a004);
  dat_1a004 = 0;
}

// Function: sub_d6c0 @ 0xd6c0
unsigned long sub_d6c0(int a0)
{
  int v1; // eax
  unsigned long v2; // rax
  
  v2 = 0;
  dat_1a00c = 0;
  do {
    v1 = sub_d190(dat_1a014,(void *)(v2 + 0x9c000),0x40000 - (int)v2);
    if (!v1) {
      v2 = (unsigned long)dat_1a00c;
      if (!dat_1a00c) {
        if (a0)
          return 0xffffffff;
        sub_d640();
        *__errno_location() = 0;
        sub_d4e0(); // no-return, return-dupe
      }
      break;
    }
    if (v1 == -1) {
      sub_d4e0();
    }
    dat_1a00c = v1 + dat_1a00c;
    v2 = (unsigned long)dat_1a00c;
  } while (dat_1a00c <= 0x3ffff);
  dat_1a008 = 1;
  dat_1a828 += v2;
  return (unsigned long)dat_9c000;
}

// Function: sub_d770 @ 0xd770
unsigned long sub_d770(unsigned int a0,int a1) // return-dupe
{
  unsigned int v1; // eax
  unsigned int v2;
  unsigned long v3;
  unsigned long v4; // r13
  void *v5; // r14
  
  *__errno_location() = 0;
  v3 = (unsigned long)dat_1a008;
  v2 = dat_1a00c;
  if (dat_1a00c <= dat_1a008)
    return 0;
  while( true ) {
    v4 = (unsigned long)(v2 - (int)v3);
    v5 = (void *)(v3 + 0x9c000);
    dat_1a820 += v4;
    if (!dat_1a858) {
      while( true ) {
        v2 = (unsigned int)v4;
        v3 = 0x7fffffff;
        if (v2 <= 0x7fffffff)
          v3 = v4;
        v3 = write(a1,v5,v3);
        v1 = (unsigned int)v3;
        if (v1 == v2) break;
        if (v1 == 0xffffffff)
          sub_d550(); // no-return
        v4 = (unsigned long)(v2 - v1);
        v5 = (void *)((long)v5 + (v3 & 0xffffffff));
      }
    }
    v2 = sub_d190(a0,(void *)0x9c000,0x40000);
    if (v2 == 0xffffffff) break;
    v3 = 0;
    dat_1a828 += (int)v2;
    dat_1a008 = 0;
    dat_1a00c = v2;
    if (!v2) {
      dat_1a008 = 0;
      return 0;
    }
  }
  sub_d4e0(); // no-return
}

// Function: sub_d850 @ 0xd850
void sub_d850(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_f6b0(a2,"%5.1f%%"); // tail-call
}

// Function: sub_d8a0 @ 0xd8a0
void sub_d8a0(FILE *a0,unsigned long a1,int a2) // return-dupe
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

// Function: sub_d9d0 @ 0xd9d0
char * sub_d9d0(unsigned int a0,unsigned int a1)
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
  dat_5b000 = 0x8b1f;
  v2 = (dat_1a848 != 0) << 3;
  dat_5b003 = v2;
  dat_18020 = 8;
  dat_5b002 = 8;
  dat_1a004 = 4;
  dat_1a010 = a1;
  dat_1a014 = a0;
  if (0 <= dat_1a838) { // branch-flip
    if ((unsigned long)(dat_1a830 - 1U) <= 0xfffffffe) {
      dat_5b004 = (char)dat_1a830;
      v5 = (char)((unsigned long)dat_1a830 >> 8);
      dat_5b005 = v5;
      v13 = (char)((unsigned long)dat_1a830 >> 0x10);
      v5 = (char)((unsigned long)dat_1a830 >> 0x18);
      v7 = v5;
      goto label_da79;
    }
    sub_d460("file timestamp out of range for gzip format");
    if (0x3fffe <= dat_1a004) { // branch-flip
      v3 = dat_1a004 + 1;
      *(char *)((unsigned long)dat_1a004 + 0x5b000) = 0;
      if (v3 == 0x40000) {
        dat_1a004 = v3;
        sub_d600();
        v3 = dat_1a004;
      }
      dat_1a004 = v3 + 1;
      *(char *)((unsigned long)v3 + 0x5b000) = 0;
      if (dat_1a004 != 0x40000) { // branch-flip
        v3 += 2;
        v14 = dat_1a004;
      }
      else {
        sub_d600();
        v3 = dat_1a004 + 1;
        v14 = dat_1a004;
      }
    }
    else {
      v14 = dat_1a004 + 2;
      v3 = dat_1a004 + 3;
      *(char *)((unsigned long)dat_1a004 + 0x5b000) = 0;
      *(char *)((unsigned long)(dat_1a004 + 1) + 0x5b000) = 0;
    }
    v8 = (unsigned long)v3;
    if (0x3fffe <= v14) {
      *(char *)((unsigned long)v14 + 0x5b000) = 0;
      if (v3 == 0x40000) {
        dat_1a004 = v3;
        sub_d600();
        v4 = (unsigned long)(unsigned long)dat_1a004;
        v8 = (unsigned long)v4;
      }
      v4 = (char *)v8;
      dat_1a004 = (int)v4 + 1;
      *(char *)((long)v4 + 0x5b000) = 0;
      if (dat_1a004 == 0x40000)
        sub_d600();
      goto label_daa4;
    }
    v7 = 0;
    v13 = 0;
    v4 = (char *)v8;
  }
  else {
    v7 = 0;
    v13 = 0;
    dat_5b005 = 0;
    dat_5b004 = 0;
label_da79:
    v4 = (char *)0x7;
    v14 = 6;
  }
  dat_1a004 = v14 + 2;
  *(char *)((unsigned long)v14 + 0x5b000) = v13;
  *(char *)((long)v4 + 0x5b000) = v7;
label_daa4:
  sub_d0d0(NULL,0);
  sub_4380(a1);
  sub_aa20(&v12,0x18020);
  v6 = '\x04';
  if (dat_1801c != 1)
    v6 = (dat_1801c == 9) * '\x02';
  v3 = dat_1a004 + 1;
  *(char *)((unsigned long)dat_1a004 + 0x5b000) = v6;
  if (v3 == 0x40000) {
    dat_1a004 = v3;
    sub_d600();
    v3 = dat_1a004;
  }
  dat_1a004 = v3;
  v4 = (unsigned long)(unsigned long)v3;
  dat_1a004 += 1;
  *(char *)((long)v4 + 0x5b000) = 3;
  v3 = dat_1a004;
  if (dat_1a004 == 0x40000) {
    sub_d600();
    v3 = dat_1a004;
  }
  v8 = (unsigned long)v3;
  if (dat_1a848) {
    v4 = (char *)sub_d290((char *)0x1a420);
    v15 = v4;
    do {
      while( true ) {
        v3 = (int)v8 + 1;
        *(char *)(v8 + 0x5b000) = *v15;
        v8 = (unsigned long)v3;
        dat_1a004 = v3;
        if (v3 != 0x40000) break;
        sub_d600();
        v16 = &v15[1];
        v8 = (unsigned long)dat_1a004;
        v2 = *v15;
        v15 = v16;
        if (!v2) goto label_db80;
      }
      v16 = &v15[1];
      v2 = *v15;
      v15 = v16;
    } while (v2);
  }
label_db80:
  dat_dfea8 = v8;
  sub_4bc0(dat_1801c);
  if ((dat_1a840 != -1) && (dat_1a840 != dat_1a828))
    sub_f6b0(stderr,"%s: %s: file size changed while zipping\n",dat_1a850,0x1a420);
  v2 = sub_d130();
  v14 = dat_1a004 + 1;
  v8 = (unsigned long)v14;
  if (0x3fffe <= dat_1a004) { // branch-flip
    *(char *)((unsigned long)dat_1a004 + 0x5b000) = v2;
    dat_1a004 = v14;
    if (v14 == 0x40000) {
      sub_d600();
      v8 = (unsigned long)dat_1a004;
    }
    sub_d130();
    dat_1a004 = (int)v8 + 1;
    v10 = (unsigned long)dat_1a004;
    *(char *)(v8 + 0x5b000) = v5;
    if (dat_1a004 == 0x40000) {
      sub_d600();
      v10 = (unsigned long)dat_1a004;
    }
  }
  else {
    v9 = dat_1a004 + 2;
    v10 = (unsigned long)v9;
    *(char *)((unsigned long)dat_1a004 + 0x5b000) = v2;
    dat_1a004 = v14;
    sub_d130();
    *(char *)(v8 + 0x5b000) = v5;
    dat_1a004 = v9;
  }
  sub_d130();
  v3 = (unsigned int)v10;
  dat_1a004 = v3 + 1;
  v8 = (unsigned long)dat_1a004;
  if (0x3fffe <= v3) { // branch-flip
    *(char *)(v10 + 0x5b000) = dat_2;
    if (dat_1a004 == 0x40000) {
      sub_d600();
      v8 = (unsigned long)dat_1a004;
    }
    sub_d130();
    dat_1a004 = (int)v8 + 1;
    v11 = (unsigned long)dat_1a004;
    *(char *)(v8 + 0x5b000) = dat_3;
    if (dat_1a004 == 0x40000) {
      sub_d600();
      v11 = (unsigned long)dat_1a004;
    }
  }
  else {
    v11 = (unsigned long)(v3 + 2);
    *(char *)(v10 + 0x5b000) = dat_2;
    sub_d130();
    *(char *)(v8 + 0x5b000) = dat_3;
  }
  v14 = (unsigned int)v11;
  dat_1a004 = v14 + 1;
  v8 = (unsigned long)dat_1a004;
  v2 = (char)dat_1a828; // branch-flip
  if (0x3fffe <= v14) {
    *(char *)(v11 + 0x5b000) = v2;
    if (dat_1a004 == 0x40000) {
      sub_d600();
      v8 = (unsigned long)dat_1a004;
    }
    dat_1a004 = (int)v8 + 1;
    v10 = (unsigned long)dat_1a004;
    v5 = (char)((unsigned long)dat_1a828 >> 8);
    *(char *)(v8 + 0x5b000) = v5;
    v4 = (char *)dat_1a828;
    if (dat_1a004 == 0x40000) {
      sub_d600();
      v10 = (unsigned long)dat_1a004;
      v4 = (char *)dat_1a828;
    }
  }
  else {
    v10 = (unsigned long)(v14 + 2);
    *(char *)(v11 + 0x5b000) = v2;
    v5 = (char)((unsigned long)dat_1a828 >> 8);
    *(char *)(v8 + 0x5b000) = v5;
    v4 = (char *)dat_1a828;
  }
  v1 = (long)v4;
  v14 = (unsigned int)v10;
  dat_1a004 = v14 + 1;
  v8 = (unsigned long)dat_1a004;
  v2 = (char)((unsigned long)v4 >> 0x10); // branch-flip
  if (0x3fffe <= v14) {
    *(char *)(v10 + 0x5b000) = v2;
    v4 = (char *)v1;
    if (dat_1a004 == 0x40000) {
      sub_d600();
      v8 = (unsigned long)dat_1a004;
      v4 = (char *)dat_1a828;
    }
    dat_1a004 = (int)v8 + 1;
    v5 = (char)((unsigned long)v4 >> 0x18);
    *(char *)(v8 + 0x5b000) = v5;
    if (dat_1a004 == 0x40000)
      sub_d600();
  }
  else {
    dat_1a004 = v14 + 2;
    *(char *)(v10 + 0x5b000) = v2;
    v5 = (char)((unsigned long)v1 >> 0x18);
    *(char *)(v8 + 0x5b000) = v5;
  }
  dat_dfea8 += 8;
  sub_d600();
  v4 = NULL;
  return v4;
}

// Function: sub_df80 @ 0xdf80
unsigned long sub_df80(unsigned char *a0,unsigned int a1)
{
  unsigned int v1; // eax
  
  v1 = sub_d190(dat_1a014,a0,a1);
  if (v1) {
    if (v1 == 0xffffffff)
      sub_d4e0(); // no-return
    sub_d0d0(a0,v1);
    dat_1a828 += (unsigned long)v1;
  }
  return (unsigned long)v1;
}

// Function: sub_dfd0 @ 0xdfd0
char * sub_dfd0(char *a0)
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

// Function: sub_e030 @ 0xe030
void sub_e030(char *a0)
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

// Function: sub_e060 @ 0xe060
unsigned long sub_e060(FILE *a0) // early-return
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
    if (sub_e3e0(a0)) {
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

// Function: sub_e0f0 @ 0xe0f0
int sub_e0f0(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_dfeb0) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_dfeb0 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_dfeb0 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_dfeb0 != -1)
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

// Function: sub_e360 @ 0xe360
void sub_e360(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_e870(open(a0,a1,v1));
}

// Function: sub_e3e0 @ 0xe3e0
void sub_e3e0(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_e430(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_e430 @ 0xe430
int sub_e430(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_e4b0 @ 0xe4b0
void sub_e4b0(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_e870(openat(a0,a1,a2,v1));
}

// Function: sub_e530 @ 0xe530
void sub_e530(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_e540 @ 0xe540
char * sub_e540(DIR *a0,unsigned int a1)
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
  v14 = *(long *)((unsigned long)a1 * 8 + 0x17b50);
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
            v8 = (void *)sub_f440(v8,&v12,1,-1,8);
          v2 = v15 * 8;
          v15 += 1;
          *(unsigned long *)((long)v8 + v2) = sub_f640(v6);
        }
        else {
          if (v13 - v10 <= v1)
            v9 = (char *)sub_f440(v9,&v13,v1 - (v13 - v10),0x7ffffffffffffffe,1);
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
          v9 = (char *)sub_f180(v10 + 1U);
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
          v6 = (char *)sub_f180(v10 + 1U);
          v9 = v6;
        }
        free(v8);
      }
      else {
        if (v10 == v13)
          v9 = (char *)sub_f1f0(v9,v10 + 1);
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

// Function: sub_e800 @ 0xe800
void * sub_e800(unsigned long a0,unsigned int a1)
{
  DIR *v1; // rax
  void *v2; // rax
  
  v1 = (DIR *)sub_10af0();
  if (!v1)
    return NULL;
  v2 = (void *)sub_e540(v1,a1);
  if (!closedir(v1))
    return v2;
  free(v2);
  return NULL;
}

// Function: sub_e870 @ 0xe870
unsigned int sub_e870(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_10c00(a0);
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_e8d0 @ 0xe8d0
int sub_e8d0(unsigned long *a0)
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

// Function: sub_e980 @ 0xe980
long sub_e980(long a0,long *a1) // return-dupe x3
{
  unsigned long *v1;
  unsigned long v2;
  long v3; // rax
  
  v1 = (unsigned long *)*a1;
  v3 = v1[3];
  if (v1[1] != 0x3ffffffe) { // branch-flip
    if (v1[1] == 0x3fffffff) {
      if (v3 == 0x3fffffff) {
        *a1 = 0;
        v3 = 0;
        return v3;
      }
      sub_10ba0(v1);
      v3 = v1[3];
    }
    if (v3 == 0x3ffffffe) {
      v2 = *(unsigned long *)(a0 + 0x60);
      v1[2] = *(unsigned long *)(a0 + 0x58);
      v1[3] = v2;
      v3 = 0;
      return v3;
    }
  }
  else {
    if (v3 == 0x3ffffffe) {
      v3 = 1;
      return v3;
    }
    v2 = *(unsigned long *)(a0 + 0x50);
    *v1 = *(unsigned long *)(a0 + 0x48);
    v1[1] = v2;
  }
  if (v3 != 0x3fffffff) {
    v3 = 0;
    return v3;
  }
  sub_10ba0((timespec *)&v1[2]);
  v3 = 0;
  return v3;
}

// Function: sub_ea40 @ 0xea40
int sub_ea40(int a0,char *a1,long *a2)
{
  long v1;
  long v10; // stack - 0xe8
  bool v11; // zf
  long v12; // stack - 0x120
  long v13; // stack - 0x118
  long v14; // stack - 0x110
  long v15; // stack - 0x100
  long v16; // stack - 0xf8
  long v17; // stack - 0xf0
  long v18; // stack - 0xe0
  long v19; // stack - 0xd8
  bool v2;
  long v20; // stack - 0xd0
  int v3;
  int v4;
  long *v5;
  long v6; // stack - 0x128
  stat v7; // stack - 0xc8
  long *v8; // stack - 0x130
  long v9; // stack - 0x108
  
  if (a2) { // branch-flip
    v6 = *a2;
    v12 = a2[1];
    v13 = a2[2];
    v14 = a2[3];
    v5 = &v6;
    v8 = v5;
    v3 = sub_e8d0(v5);
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
  if (0 <= dat_dfeb8) {
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
          v5[2] = v7._88_8_;
          v5[3] = v7._96_8_;
        }
      }
      else {
        v3 = 3;
        *v5 = v7._72_8_;
        v5[1] = v7._80_8_;
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
      dat_dfeb8 = 1;
      return v4;
    }
  }
  dat_dfeb8 = 0xffffffff;
  dat_dfeb4 = 0xffffffff;
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
    if ((v5) && (sub_e980(&v7,&v8)))
      return 0;
  }
  v5 = NULL;
  if (v8) {
    v9 = *v8;
    v16 = v8[2];
    v15 = v8[1] / 1000;
    v5 = &v9;
    v17 = v8[3] / 1000;
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
  v2 = v5[3] <= 499999;
  if ((v1 <= 499999) && (v2))
    return 0;
  if (fstat(a0,&v7))
    return 0;
  v10 = *v5;
  v18 = v5[1];
  v19 = v5[2];
  v20 = v5[3];
  v11 = v7._88_8_ - v5[2] != 1;
  if (((v7._72_8_ - *v5 != 1) || (v1 <= 499999)) || (v7._80_8_)) {
    if (v11 || v2)
      return 0;
    if (v7._96_8_)
      return 0;
  }
  else {
    v18 = 0;
    if ((v11 || v2) || (v7._96_8_)) {
      futimesat(a0,NULL,&v10); // return-dupe
      return 0;
    }
  }
  v20 = 0;
  futimesat(a0,NULL,&v10);
  return 0;
}

// Function: sub_ee70 @ 0xee70
int sub_ee70(char *a0,long *a1)
{
  return sub_ea40(0xffffffff,a0,a1); // tail-call
}

// Function: sub_ee90 @ 0xee90
int sub_ee90(char *a0,unsigned long *a1)
{
  int v1; // eax
  int v2; // eax
  unsigned long *v3;
  undefined8 v4; // stack - 0xe8
  stat v5; // stack - 0xc8
  unsigned long *v6; // stack - 0xf0
  long v7; // stack - 0xe0
  undefined8 v8; // stack - 0xd8
  long v9; // stack - 0xd0
  
  if (a1) { // branch-flip
    v4 = *a1;
    v7 = a1[1];
    v8 = a1[2];
    v9 = a1[3];
    v3 = &v4;
    v6 = v3;
    v1 = sub_e8d0(v3);
    if (v1 <= -1)
      return -1;
    if (dat_dfeb4 > -1) {
      if (v1 == 2) {
        if (lstat(a0,&v5))
          return -1;
        if (v7 != 0x3ffffffe) { // branch-flip
          v1 = 3;
          if (v9 == 0x3ffffffe) {
            v8 = v5._88_8_;
            v9 = v5._96_8_;
          }
        }
        else {
          v4 = v5._72_8_;
          v1 = 3;
          v7 = v5._80_8_;
        }
      }
label_ef48:
      v2 = utimensat(-100,a0,v3,0x100);
      if (1 <= v2) // branch-flip
        *__errno_location() = 0x26;
      else if ((!v2) || (*__errno_location() != 0x26)) {
        dat_dfeb4 = 1;
        dat_dfeb8 = 1;
        return v2;
      }
    }
    dat_dfeb4 = 0xffffffff;
    if (v1) {
      if ((v1 != 3) && (lstat(a0,&v5)))
        return -1;
      if ((v3) && (sub_e980(&v5,&v6)))
        return 0;
      goto label_efb3;
    }
  }
  else {
    v1 = 0;
    v6 = NULL;
    v3 = NULL;
    if (0 <= dat_dfeb4) goto label_ef48;
  }
  dat_dfeb4 = 0xffffffff;
  if (lstat(a0,&v5))
    return -1;
label_efb3:
  if ((v5._24_4_ & 0xf000) != 0xa000)
    return sub_ea40(0xffffffff,a0,v6);
  *__errno_location() = 0x26;
  return -1;
}

// Function: sub_f120 @ 0xf120
void sub_f120(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_d430(); // no-return
}

// Function: sub_f160 @ 0xf160
void sub_f160(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_d430(); // no-return
}

// Function: sub_f180 @ 0xf180
void sub_f180(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_d430(); // no-return
}

// Function: sub_f1a0 @ 0xf1a0
void sub_f1a0(void)
{
  sub_f160(); // tail-call
}

// Function: sub_f1b0 @ 0xf1b0
void sub_f1b0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_d430(); // no-return
}

// Function: sub_f1f0 @ 0xf1f0
void sub_f1f0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_d430(); // no-return
}

// Function: sub_f220 @ 0xf220
void sub_f220(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_d430(); // no-return
}

// Function: sub_f260 @ 0xf260
void sub_f260(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_d430(); // no-return
}

// Function: sub_f2a0 @ 0xf2a0
void sub_f2a0(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_d430(); // no-return
}

// Function: sub_f2d0 @ 0xf2d0
void sub_f2d0(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_d430(); // no-return
}

// Function: sub_f320 @ 0xf320
void sub_f320(void *a0,unsigned long *a1)
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
  sub_d430(); // no-return
}

// Function: sub_f3b0 @ 0xf3b0
void sub_f3b0(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_d430(); // no-return
}

// Function: sub_f440 @ 0xf440
void sub_f440(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_f4fe;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_f4fe:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_d430(); // no-return
}

// Function: sub_f540 @ 0xf540
void sub_f540(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_d430(); // no-return
}

// Function: sub_f560 @ 0xf560
void sub_f560(unsigned long a0)
{
  sub_f540(a0,1); // tail-call
}

// Function: sub_f570 @ 0xf570
void sub_f570(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_d430(); // no-return
}

// Function: sub_f590 @ 0xf590
void sub_f590(unsigned long a0)
{
  sub_f570(a0,1); // tail-call
}

// Function: sub_f5a0 @ 0xf5a0
void sub_f5a0(char *a0,unsigned long a1)
{
  memcpy((void *)sub_f160(a1),a0,a1); // tail-call
}

// Function: sub_f5d0 @ 0xf5d0
void sub_f5d0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_f180(a1),a0,a1); // tail-call
}

// Function: sub_f600 @ 0xf600
void sub_f600(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_f180(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_f640 @ 0xf640
void sub_f640(char *a0)
{
  sub_f5a0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_f660 @ 0xf660
unsigned long sub_f660(void)
{
  unsigned int v1; // eax
  unsigned int v2; // edx
  unsigned long v3; // r12
  
  v1 = getc(stdin);
  v2 = v1 & 0xffffffdf;
  if (v1 != 10) {
    while (v1 != 0xffffffff) {
      v1 = getc(stdin);
      if (v1 == 10) break;
    }
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),v2 == 0x59) & 0xffffffff;
}

// Function: sub_f6b0 @ 0xf6b0
unsigned long sub_f6b0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,int *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  v3 = (char *)sub_f900(v4,&v6,a9,&v5);
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
  sub_10b90(a8);
  return 0xffffffff;
}

// Function: sub_f830 @ 0xf830
void sub_f830(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,int *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  sub_10a00(stdout,a8,&v2);
}

// Function: sub_f900 @ 0xf900
int * sub_f900(int *a0,unsigned long *a1,int *a2,struct_0 *a3)
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
  if ((int)sub_10e50(a2,&v19,v17) <= -1)
    return NULL;
  if ((int)sub_10c20(a3,v17) < 0) {
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
label_f9f9:
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
      if (v26 == v39) goto label_fbd2;
      do {
        v27 = (long)v26 - (long)v13;
        v11 = v14 + v27;
        if (CARRY8(v14,v27))
          v11 = 0xffffffffffffffff;
        if (v11 <= v10) goto label_fbb5;
        if (v10) {
          if (0 <= (long)v10) {
            v10 *= 2;
            goto label_fb56;
          }
label_fa70:
          v38 = v12;
label_fa77:
          *(unsigned long *)&v24[-8] = 0xfa7c;
          v39 = __errno_location();
          v12 = v38;
label_fa8a:
          if ((v12 == v40) || (!v12)) goto label_fa9c;
          goto label_10020;
        }
        v10 = 0xc;
label_fb56:
        if (v10 < v11)
          v10 = v11;
        if (v10 == 0xffffffffffffffff) goto label_fa70;
        if ((!v12) || (v12 == v40)) {
          v39 = (int *)CONCAT71(v39._1_7_,v12 == v40);
          *(unsigned long *)&v24[-8] = 0xff6c;
          v37 = v13;
          v38 = v12;
          v12 = malloc(v10);
          if (v12) {
            if ((v14) && ((char)v39)) {
              v39 = v37;
              *(unsigned long *)&v24[-8] = 0xffb3;
              v12 = memcpy(v12,v38,v14);
              v13 = v39;
            }
            else {
              v13 = v37;
            }
            goto label_fbb5;
          }
          goto label_fa77;
        }
        *(unsigned long *)&v24[-8] = 0xfb9b;
        v38 = v13;
        v39 = v12;
        v12 = realloc(v12,v10);
        v26 = v39;
        if (!v12) goto label_ffff;
        v13 = v38;
label_fbb5:
        *(unsigned long *)&v24[-8] = 0xfbcb;
        v39 = v12;
        memcpy((void *)((long)v12 + v14),v13,v27);
        v12 = v39;
label_fbd2:
        do {
          v8 = v41;
          if (v19 == v35) {
            v14 = v11 + 1;
            if (0xffffffffffffffff <= v11) {
              if (v10 == 0xffffffffffffffff) goto label_108bd;
              goto label_fa70;
            }
            if (v14 <= v10) goto label_108bd;
            if (v10) { // branch-flip
              if ((long)v10 < 0) goto label_fa70;
              v10 *= 2;
              if (v10 < v14) {
label_109be:
                if (v14 == 0xffffffffffffffff) goto label_fa70;
                v10 = v14;
              }
            }
            else {
              if (v14 > 0xc) goto label_109be;
              v10 = 0xc;
            }
            v29 = v12 == v40;
            if ((!v12) || (v29)) {
              *(unsigned long *)&v24[-8] = 0x10979;
              v39 = v12;
              v26 = malloc(v10);
              v12 = v39;
              if (!v26) goto label_fa70;
              if ((!v11) || (!v29))
                v12 = v26;
              else {
                *(unsigned long *)&v24[-8] = 0x109a4;
                v12 = memcpy(v26,v39,v11);
              }
            }
            else {
              *(unsigned long *)&v24[-8] = 0x108aa;
              v39 = v12;
              v12 = realloc(v12,v10);
              v26 = v39;
              if (!v12) goto label_ffff;
            }
label_108bd:
            *(char *)((long)v12 + v11) = 0;
            if ((v14 < v10) && (v12 != v40)) {
              *(unsigned long *)&v24[-8] = 0x108e2;
              v40 = v12;
              v26 = realloc(v12,v14);
              v12 = v40;
              if (v26)
                v12 = v26;
            }
            if (v32) {
              *(unsigned long *)&v24[-8] = 0x10908;
              v40 = v12;
              free(v32);
              v12 = v40;
            }
            if (v42 != v20) {
              *(unsigned long *)&v24[-8] = 0x1092e;
              v40 = v12;
              free(v42);
              v12 = v40;
            }
            v40 = v12;
            if (v41 != v21) {
              *(unsigned long *)&v24[-8] = 0x10954;
              free(v41);
            }
            *v33 = v11;
            return v40;
          }
          v5 = v28[10];
          if (*(char *)&v28[9] != '%') { // branch-flip
            if (v5 == -1) {
label_109d0:
              *(unsigned long *)&v24[-8] = 0x109d5; // no-return
              abort();
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
                *(unsigned long *)&v24[-8] = 0xfcbd;
                v37 = v12;
                v39 = v13;
                memcpy(v26,v6,(unsigned long)v13);
                v26 = (int *)((long)v26 + (long)v39);
                v12 = v37;
              }
              v6 = (void *)v28[6];
              v13 = v12;
              if (v6 != (void *)v28[7]) {
                v13 = (int *)((long)(void *)v28[7] - (long)v6);
                *(unsigned long *)&v24[-8] = 0xfcf7;
                v37 = v12;
                v39 = v13;
                memcpy(v26,v6,(unsigned long)v13);
                v26 = (int *)((long)v26 + (long)v39);
                v13 = v37;
              }
              switch(v38._0_4_) {
                case 9:
                case 10:
                  *(char *)v26 = 0x6c;
                  v26 = (int *)((long)v26 + 1);
                  goto label_fd39;
                case 7:
                case 8:
                case 0xe:
                case 0x10:
label_fd39:
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
                if (*(int *)&v8[v28[5] * 0x20] != 5) goto label_109d0;
                v37 = (int *)CONCAT44(v37._4_4_,1);
                v22[0] = *(unsigned int *)((long)&v8[v28[5] * 0x20] + 0x10);
              }
              else {
                v37 = (int *)((unsigned long)v37._4_4_ << 0x20);
              }
              if (v28[8] != -1) {
                if (*(int *)&v8[v28[8] * 0x20] != 5) goto label_109d0;
                v22[(unsigned long)v37 & 0xffffffff] = *(unsigned int *)((long)&v8[v28[8] * 0x20] + 0x10);
                v37 = (int *)CONCAT44(v37._4_4_,(int)v37 + 1);
              }
              v14 = v11 + 2;
              v12 = v13;
              if (0xfffffffffffffffe <= v11) { // branch-flip
                if (v10 != 0xffffffffffffffff) goto label_fa70;
              }
              else if (v14 > v10) {
                if (v10) { // branch-flip
                  if ((long)v10 < 0) goto label_fa70;
                  v10 *= 2;
                  if (v10 < v14) {
label_103e2:
                    if (v14 == 0xffffffffffffffff) goto label_fa70;
                    v10 = v14;
                  }
                }
                else {
                  if (v14 > 0xc) goto label_103e2;
                  v10 = 0xc;
                }
                if ((!v13) || (v13 == v40)) {
                  v39 = (int *)CONCAT71(v39._1_7_,v13 == v40);
                  *(unsigned long *)&v24[-8] = 0x105e5;
                  v34 = v13;
                  v13 = malloc(v10);
                  v12 = v34;
                  if (!v13) goto label_fa70;
                  if ((v11) && ((char)v39)) {
                    *(unsigned long *)&v24[-8] = 0x1061e;
                    memcpy(v13,v34,v11);
                  }
                }
                else {
                  *(unsigned long *)&v24[-8] = 0xfe12;
                  v39 = v13;
                  v13 = realloc(v13,v10);
                  v12 = v39;
                  if (!v13) goto label_fa70;
                }
              }
              *(char *)((long)v13 + v11) = 0;
              *(unsigned long *)&v24[-8] = 0xfe2f;
              v39 = __errno_location();
              v31 = *v39;
              v12 = v13;
              v34 = v26;
label_fe50:
              v23 = -1;
              v27 = v10 - v11;
              *v39 = 0;
              v14 = 0x7fffffff;
              if (v27 <= 0x7fffffff)
                v14 = v27;
              switch((unsigned long)v38 & 0xffffffff) {
                default:
                  goto label_109d0;
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
                  if ((int)v37 == 1) {
                    *(int **)&v24[-8] = &v23;
                    *(int **)&v24[-0x10] = v26;
                    goto label_1056a;
                  }
                  if ((int)v37 != 2) {
                    *(int **)&v24[-0x10] = &v23;
label_10072:
                    *(unsigned long *)&v24[-0x18] = 0x1008f;
                    v30 = v27;
                    v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v26);
                    v27 = v30;
                    goto label_10098;
                  }
                  *(int **)&v24[-0x10] = &v23;
                  *(int **)&v24[-0x18] = v26;
                  goto label_1051e;
                case 0xb:
                  v1 = *(unsigned long *)&v41[v28[10] * 0x20 + 0x10];
                  if ((int)v37 != 1) { // branch-flip
                    if ((int)v37 != 2) {
                      *(unsigned long *)&v24[-8] = 0x1033b;
                      v30 = v27;
                      v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v1,&v23);
                      v27 = v30;
                      goto label_10098;
                    }
                    *(int **)&v24[-8] = &v23;
                    *(unsigned long *)&v24[-0x10] = (unsigned long)v22[1];
                  }
                  else {
                    *(int **)&v24[-0x10] = &v23;
                  }
                  *(unsigned long *)&v24[-0x18] = 0x10776;
                  v30 = v27;
                  v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v1,(unsigned long)v22[0]);
                  v27 = v30;
                  goto label_10098;
                case 0xc:
                  v2 = *(void *)&v41[v28[10] * 0x20 + 0x10];
                  if ((int)v37 != 1) {
                    if ((int)v37 != 2) {
                      v26 = &v23;
                      *(void *)&v24[-0x10] = v2;
                      goto label_10072;
                    }
                    *(int **)&v24[-0x10] = &v23;
                    *(void *)&v24[-0x20] = v2;
                    *(unsigned long *)&v24[-0x30] = (unsigned long)v22[1];
                    *(unsigned long *)&v24[-0x38] = 0x10714;
                    v30 = v27;
                    v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,(unsigned long)v22[0]);
                    v27 = v30;
                    goto label_10098;
                  }
                  *(int **)&v24[-0x10] = &v23;
                  *(void *)&v24[-0x20] = v2;
                  goto label_10525;
                
              }
              if ((int)v37 == 1) {
                *(int **)&v24[-8] = &v23;
                *(unsigned long *)&v24[-0x10] = v15;
label_1056a:
                v15 = (unsigned long)v22[0];
label_101b2:
                *(unsigned long *)&v24[-0x18] = 65999;
                v30 = v27;
                v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,v15);
                v27 = v30;
                if (0 <= v23) goto label_100a6;
label_101e6:
                if (!*(char *)((long)v34 + 1)) {
                  if (v9 <= -1) {
                    if (!*v39) {
                      v9 = 0x16;
                      if ((*(unsigned char *)&v28[9] & 0xef) == 99)
                        v9 = 0x54;
                      *v39 = v9;
                    }
                    if ((v12 != v40) && (v12)) {
                      *(unsigned long *)&v24[-8] = 0x10245;
                      free(v12);
                    }
                    if (v32) {
                      *(unsigned long *)&v24[-8] = 0x10259;
                      free(v32);
                    }
                    if (v42 != v20) {
                      *(unsigned long *)&v24[-8] = 0x10271;
                      free(v42);
                    }
                    if (v41 != v21) {
                      *(unsigned long *)&v24[-8] = 0x1028d;
                      free(v41);
                      return NULL;
                    }
                    return NULL;
                  }
                  goto label_100bf;
                }
                *(char *)((long)v34 + 1) = 0;
                goto label_fe50;
              }
              if ((int)v37 != 2) {
                *(int **)&v24[-0x10] = &v23;
                goto label_101b2;
              }
              *(int **)&v24[-0x10] = &v23;
              *(unsigned long *)&v24[-0x18] = v15;
label_1051e:
              *(unsigned long *)&v24[-0x20] = (unsigned long)v22[1];
label_10525:
              *(unsigned long *)&v24[-0x28] = 0x10549;
              v30 = v27;
              v9 = __snprintf_chk((char *)((long)v12 + v11),v14,1,0xffffffffffffffff,v36,(unsigned long)v22[0]);
              v27 = v30;
label_10098:
              if (v23 <= -1) goto label_101e6;
label_100a6:
              if (((unsigned long)(long)v23 < v14) && (*(char *)((long)v12 + v11 + (long)v23))) goto label_109d0;
              v16 = v23;
              if (v23 < v9) {
label_100bf:
                v16 = v9;
                v23 = v9;
              }
              if (v14 <= v16 + 1) {
                if (0x7fffffff <= v27) {
                  if ((v12 != v40) && (v12)) {
                    *(unsigned long *)&v24[-8] = 0x107de;
                    free(v12);
                  }
                  if (v32) {
                    *(unsigned long *)&v24[-8] = 0x107f2;
                    free(v32);
                  }
                  if (v42 != v20) {
                    *(unsigned long *)&v24[-8] = 0x1080a;
                    free(v42);
                  }
                  if (v41 != v21) {
                    *(unsigned long *)&v24[-8] = 0x10822;
                    free(v41);
                  }
                  *v39 = 0x4b;
                  return NULL;
                }
                v14 = (unsigned long)(v16 + 2);
                if (0 <= (long)v10) { // branch-flip
                  v27 = v10 * 2;
                  if (CARRY8(v14,v11)) goto label_fa8a;
                  v15 = v14 + v11;
                  if (v14 + v11 <= v27)
                    v15 = v27;
                }
                else {
                  if (CARRY8(v14,v11)) {
                    if (v10 != 0xffffffffffffffff) goto label_fa8a;
                    goto label_fe50;
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
                  if (v10 == 0xffffffffffffffff) goto label_fa8a;
                  v29 = v12 == v40;
                  if ((!v12) || (v29)) {
                    *(unsigned long *)&v24[-8] = 0x104e0;
                    v26 = malloc(v10);
                    if (!v26) goto label_fa8a;
                    if ((!v11) || (!v29))
                      v12 = v26;
                    else {
                      *(unsigned long *)&v24[-8] = 0x10508;
                      v12 = memcpy(v26,v12,v11);
                    }
                  }
                  else {
                    *(unsigned long *)&v24[-8] = 0x1015a;
                    v26 = realloc(v12,v10);
                    if (!v26) goto label_fa8a;
                    v12 = v26;
                  }
                }
                goto label_fe50;
              }
              v11 = (long)v16 + v11;
              *v39 = v31;
            }
            else {
              if (5 <= (unsigned int)(v9 - 0x12U)) goto label_109d0;
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
            if (v5 != -1) goto label_109d0;
            v14 = v11 + 1;
            if (v11 == 0xffffffffffffffff)
              v14 = 0xffffffffffffffff;
            if (v10 < v14) {
              if (v10) { // branch-flip
                if ((long)v10 < 0) goto label_fa70;
                v10 *= 2;
              }
              else {
                v10 = 0xc;
              }
              if (v10 < v14)
                v10 = v14;
              if (v10 == 0xffffffffffffffff) goto label_fa70;
              v29 = v12 == v40;
              if ((!v12) || (v29)) {
                *(unsigned long *)&v24[-8] = 0x1063f;
                v39 = v12;
                v26 = malloc(v10);
                v12 = v39;
                if (!v26) goto label_fa70;
                if ((!v11) || (!v29))
                  v12 = v26;
                else {
                  *(unsigned long *)&v24[-8] = 0x10672;
                  v12 = memcpy(v26,v39,v11);
                }
              }
              else {
                *(unsigned long *)&v24[-8] = 0xff11;
                v39 = v12;
                v12 = realloc(v12,v10);
                v26 = v39;
                if (!v12) goto label_ffff;
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
        goto label_f9f9;
      }
    }
  }
  v39 = __errno_location();
  goto label_fab0;
label_ffff:
  *(unsigned long *)&v24[-8] = 0x1000b;
  v40 = v26;
  v39 = __errno_location();
  v12 = v40;
label_10020:
  *(unsigned long *)&v24[-8] = 0x10028;
  free(v12);
label_fa9c:
  if (v32) {
    *(unsigned long *)&v24[-8] = 0xfab0;
    free(v32);
  }
label_fab0:
  if (v42 != v20) {
    *(unsigned long *)&v24[-8] = 0xfac8;
    free(v42);
  }
  if (v41 != v21) {
    *(unsigned long *)&v24[-8] = 0xfae0;
    free(v41);
  }
  *v39 = 0xc;
  return NULL;
}

// Function: sub_10a00 @ 0x10a00
unsigned long sub_10a00(FILE *a0,int *a1,struct_0 *a2)
{
  unsigned long v1;
  char *v2; // rax
  char v3 [2008];
  unsigned long v4; // stack - 0x810
  
  v4 = 2000;
  v2 = (char *)sub_f900(v3,&v4,a1,a2);
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
  sub_10b90(a0);
  return 0xffffffff;
}

// Function: sub_10af0 @ 0x10af0
DIR * sub_10af0(char *a0)
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
      v2 = sub_e0f0(v1,0x406,3);
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

// Function: sub_10b90 @ 0x10b90
void sub_10b90(unsigned int *a0)
{
  *a0 = *a0 | 0x20;
}

// Function: sub_10ba0 @ 0x10ba0
void sub_10ba0(timespec *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_10bb0 @ 0x10bb0
timespec sub_10bb0(void)
{
  timespec v1; // stack - 0x28
  
  clock_gettime(0,&v1);
  return v1;
}

// Function: sub_10c00 @ 0x10c00
void sub_10c00(unsigned long a0)
{
  sub_e0f0(a0,0,3); // tail-call
}

// Function: sub_10c20 @ 0x10c20
unsigned long sub_10c20(struct_0 *a0,struct_1 *a1) // return-dupe x2
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

// Function: sub_10e50 @ 0x10e50
unsigned long sub_10e50(char *a0,unsigned long *a1,struct_2 *a2) // return-dupe
{
  unsigned char v1;
  struct_2 *v10;
  char *v11;
  int v12;
  int v13; // eax
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
  struct_2 *v3;
  char *v4;
  int v5;
  struct_2 *v6; // rax
  unsigned long *v7;
  unsigned long v8;
  struct_2 *v9;
  
  v2 = &a1[4];
  v3 = &a2[1];
  v20 = 7;
  *a1 = 0;
  a1[1] = (unsigned long)v2;
  v8 = 0;
  a2->field_0x0 = 0;
  a2->field_0x8 = &v3->field_0x0;
  v25 = 0;
  v24 = 0;
  v26 = 0;
  v7 = v2;
  v22 = 7;
label_10ecc:
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
label_10f77:
      v18 = &v17[1];
      if (v19 == '\'') goto label_11021;
      do {
        switch(v19) {
          case 0x20:
            v13 = (unsigned int)v7[2] | 8;
            v17 = v18;
            break;
          default:
            goto label_10fa8;
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
label_11021:
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
    if (*v11 != '$') goto label_10f77;
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
        if (9 < v1) goto label_11460;
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
      goto label_10f77;
    }
    goto label_11460;
  }
  goto label_10ec9;
label_10fa8:
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
label_117ec:
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
label_110d5:
      v7[5] = v26;
      v14 = v26 + 1;
      if (v26 == 0xffffffffffffffff) goto label_11460;
      v15 = v26;
      v26 = v14;
    }
    else {
      v11 = v18;
      do {
        v4 = &v11[1];
        v11 = &v11[1];
      } while ((unsigned char)(*v4 - 0x30U) <= 9);
      if (*v4 != '$') goto label_110d5;
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
          if (9 < v1) goto label_11460;
          v14 = (unsigned long)(char)v1;
          v16 = 0xffffffffffffffff;
          v18 = v17;
          v17 = &v17[1];
        }
        if (9 < v1) break;
        v17 = &v17[1];
      }
      v15 -= 1;
      if (0xfffffffffffffffe <= v15) goto label_11460;
      v7[5] = v15;
      v18 = &v18[2];
    }
    v9 = (struct_2 *)a2->field_0x8;
    v10 = v9;
    if (v20 <= v15) {
      v20 *= 2;
      if (v20 <= v15)
        v20 = v15 + 1;
      if (v20 >> 0x3b) goto label_116ab;
      if (v3 != v9) // branch-flip
        v10 = realloc(v9,v20 << 5);
      else {
        v10 = malloc(v20 << 5);
      }
      v9 = (struct_2 *)a2->field_0x8;
      if (!v10) goto label_116ab;
      if (v3 == v9)
        v10 = memcpy(v10,v3,a2->field_0x0 << 5);
      a2->field_0x8 = &v10->field_0x0;
    }
    v14 = a2->field_0x0;
    v9 = &v10[v14 * 2];
    if (v14 <= v15) {
      do {
        v6 = v9;
        v14 += 1;
        *(unsigned int *)&v6->field_0x0 = 0;
        v9 = &v6[2];
      } while (v14 <= v15);
      a2->field_0x0 = v14;
      *(unsigned int *)&v6->field_0x0 = 0;
    }
    v12 = (int)v10[v15 * 2].field_0x0;
    v24 = v8;
    if (v12) {
      if (v12 != 5) goto label_11464;
      goto label_117ec;
    }
    *(unsigned int *)&v10[v15 * 2].field_0x0 = 5;
    v19 = *v18;
  }
  v17 = v18;
  if (v19 != '.') goto label_10fc7;
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
    goto label_10fc7;
  }
  v17 = &v18[2];
  v7[6] = (unsigned long)v18;
  v7[7] = (unsigned long)v17;
  v19 = v18[2];
  v8 = 2;
  if (2 <= v25)
    v8 = v25;
  if (10 <= (unsigned char)(v19 - 0x30U)) { // branch-flip
label_11215:
    v15 = v7[8];
    if (v7[8] == 0xffffffffffffffff) {
      v7[8] = v26;
      if (v26 == 0xffffffffffffffff) goto label_11460;
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
    if (*v4 != '$') goto label_11215;
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
        if (9 < v1) goto label_11460;
        v14 = (unsigned long)(char)v1;
        v16 = 0xffffffffffffffff;
        v17 = v18;
        v18 = &v18[1];
      }
      if (9 < v1) break;
      v18 = &v18[1];
    }
    v15 -= 1;
    if (0xfffffffffffffffe <= v15) goto label_11460;
    v7[8] = v15;
    v17 = &v17[2];
  }
  v9 = (struct_2 *)a2->field_0x8;
  v10 = v9;
  if (v20 <= v15) {
    v20 *= 2;
    if (v20 <= v15)
      v20 = v15 + 1;
    if (v20 >> 0x3b) goto label_116ab;
    if (v3 != v9) // branch-flip
      v10 = realloc(v9,v20 << 5);
    else {
      v10 = malloc(v20 << 5);
    }
    v9 = (struct_2 *)a2->field_0x8;
    if (!v10) goto label_116ab;
    if (v3 == v9)
      v10 = memcpy(v10,v3,a2->field_0x0 << 5);
    a2->field_0x8 = &v10->field_0x0;
  }
  v14 = a2->field_0x0;
  v9 = &v10[v14 * 2];
  if (v14 <= v15) {
    do {
      v6 = v9;
      v14 += 1;
      *(unsigned int *)&v6->field_0x0 = 0;
      v9 = &v6[2];
    } while (v14 <= v15);
    a2->field_0x0 = v14;
    *(unsigned int *)&v6->field_0x0 = 0;
  }
  v12 = (int)v10[v15 * 2].field_0x0;
  v25 = v8;
  if (!v12) {
    *(unsigned int *)&v10[v15 * 2].field_0x0 = 5;
    v19 = *v17;
label_10fc7:
    v13 = 0;
    v17 = &v17[1];
    if (v19 == 'h') goto label_11094;
    do {
      switch(v19) {
        case 0x4c:
          v13 |= 4;
          break;
        default:
          goto label_11060;
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
label_11094:
        v13 |= 1 << ((unsigned char)v13 & 1);
      }
    } while( true );
  }
  if (v12 == 5) {
    v19 = *v17;
    goto label_10fc7;
  }
  goto label_11464;
label_11060:
  switch(v19) {
    case 0x25:
      goto label_1131e;
    default:
      goto label_11460;
    case 0x41:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x61:
    case 0x65:
    case 0x66:
    case 0x67:
      v12 = 0xc;
      if (v13 <= 0xf)
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
      if (((v13 <= 0xf) && (!(v13 & 4))) && ((v12 = 8, v13 <= 7 && (v12 = 2, !(v13 & 2)))))
        v12 = (-(unsigned int)((v13 & 1) == 0) & 2) + 4;
      break;
    case 99:
      v12 = (7 < v13) + 0xd;
      break;
    case 100:
    case 0x69:
      v12 = 9;
      if (((v13 <= 0xf) && (!(v13 & 4))) && ((v12 = 7, v13 <= 7 && (v12 = 1, !(v13 & 2)))))
        v12 = (-(unsigned int)((v13 & 1) == 0) & 2) + 3;
      break;
    case 0x6e:
      v12 = 0x16;
      if ((((v13 <= 0xf) && (!(v13 & 4))) && (v12 = 0x15, v13 <= 7)) && (v12 = 0x12, !(v13 & 2)))
        v12 = 0x14 - (v13 & 1);
      break;
    case 0x70:
      v12 = 0x11;
      break;
    case 0x73:
      v12 = (7 < v13) + 0xf;
    
  }
  if (v23 != 0xffffffffffffffff) // branch-flip
    v7[10] = v23;
  else {
    v7[10] = v26;
    if (v26 == 0xffffffffffffffff) {
label_11460:
      v10 = (struct_2 *)a2->field_0x8;
      goto label_11464;
    }
    v23 = v26;
    v26 += 1;
  }
  v9 = (struct_2 *)a2->field_0x8;
  v10 = v9;
  if (v20 <= v23) {
    v20 *= 2;
    if (v20 <= v23)
      v20 = v23 + 1;
    if (v20 >> 0x3b) goto label_116ab;
    if (v3 != v9) // branch-flip
      v10 = realloc(v9,v20 << 5);
    else {
      v10 = malloc(v20 << 5);
    }
    v9 = (struct_2 *)a2->field_0x8;
    if (!v10) goto label_116ab;
    if (v3 == v9)
      v10 = memcpy(v10,v3,a2->field_0x0 << 5);
    a2->field_0x8 = &v10->field_0x0;
  }
  v8 = a2->field_0x0;
  v9 = &v10[v8 * 2];
  if (v8 <= v23) {
    do {
      v6 = v9;
      v8 += 1;
      *(unsigned int *)&v6->field_0x0 = 0;
      v9 = &v6[2];
    } while (v8 <= v23);
    a2->field_0x0 = v8;
    *(unsigned int *)&v6->field_0x0 = 0;
  }
  v5 = (int)v10[v23 * 2].field_0x0;
  if (v5) { // branch-flip
    if (v5 != v12) {
label_11464:
      if (v3 != v10)
        free(v10);
      if (v2 != (unsigned long *)a1[1])
        free((unsigned long *)a1[1]);
      *__errno_location() = 0x16;
      return 0xffffffff;
    }
  }
  else {
    *(int *)&v10[v23 * 2].field_0x0 = v12;
  }
label_1131e:
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
        goto label_10ec9;
      }
    }
    v9 = (struct_2 *)a2->field_0x8;
label_116ab:
    if (v3 != v9)
      free(v9);
    if (v2 != (unsigned long *)a1[1])
      free((unsigned long *)a1[1]);
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  v7 = (unsigned long *)a1[1];
label_10ec9:
  a0 = v17;
  v22 = v21;
  goto label_10ecc;
}

// Function: _DT_FINI @ 0x11b68
void _DT_FINI(void)
{
  return;
}
