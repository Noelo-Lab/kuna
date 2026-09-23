// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_12fe8)
    return;
  (*dat_12fe8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_12d38)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3560
void __cxa_finalize(void)
{
  (*dat_12ff8)(); // jump-as-call
}

// Function: raise @ 0x3570
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12d40)(); // jump-as-call
  return v1;
}

// Function: free @ 0x3580
void free(void *a0)
{
  (*dat_12d48)(); // jump-as-call
}

// Function: __mempcpy_chk @ 0x3590
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12d50)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x35a0
void abort(void)
{
  (*dat_12d58)(); // jump-as-call
}

// Function: __errno_location @ 0x35b0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_12d60)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x35c0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_12d68)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x35d0
void _exit(int a0)
{
  (*dat_12d70)(); // jump-as-call
}

// Function: iconv @ 0x35e0
void iconv(void)
{
  (*dat_12d78)(); // jump-as-call
}

// Function: getc_unlocked @ 0x35f0
int getc_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12d80)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3600
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_12d88)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3610
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_12d90)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3620
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12d98)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3630
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_12da0)(); // jump-as-call
  return v1;
}

// Function: write @ 0x3640
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_12da8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x3650
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12db0)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x3660
int __open_2(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_12db8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3670
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12dc0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3680
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12dc8)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x3690
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12dd0)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x36a0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12dd8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x36b0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12de0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x36c0
void __stack_chk_fail(void)
{
  (*dat_12de8)(); // jump-as-call
}

// Function: mmap @ 0x36d0
void mmap(void)
{
  (*dat_12df0)(); // jump-as-call
}

// Function: getopt_long @ 0x36e0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_12df8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x36f0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12e00)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x3700
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_12e08)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3710
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12e10)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3720
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12e18)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3730
void lseek(void)
{
  (*dat_12e20)(); // jump-as-call
}

// Function: __assert_fail @ 0x3740
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_12e28)(); // jump-as-call
}

// Function: fputs @ 0x3750
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_12e30)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x3760
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12e38)(); // jump-as-call
  return v1;
}

// Function: freopen @ 0x3770
FILE * freopen(char *a0,char *a1,FILE *a2)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_12e40)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x3780
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12e48)(); // jump-as-call
  return v1;
}

// Function: close @ 0x3790
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12e50)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x37a0
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_12e58)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x37b0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12e60)(); // jump-as-call
  return v1;
}

// Function: read @ 0x37c0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_12e68)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x37d0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_12e70)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x37e0
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_12e78)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x37f0
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12e80)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3800
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12e88)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3810
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_12e90)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x3820
void signal(void)
{
  (*dat_12e98)(); // jump-as-call
}

// Function: fputc_unlocked @ 0x3830
int fputc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_12ea0)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x3840
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_12ea8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3850
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_12eb0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3860
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12eb8)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x3870
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12ec0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x3880
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_12ec8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3890
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12ed0)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x38a0
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_12ed8)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x38b0
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_12ee0)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x38c0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12ee8)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x38d0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12ef0)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x38e0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12ef8)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x38f0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12f00)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x3900
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_12f08)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3910
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12f10)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x3920
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_12f18)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x3930
void iconv_close(void)
{
  (*dat_12f20)(); // jump-as-call
}

// Function: iswspace @ 0x3940
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_12f28)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x3950
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12f30)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3960
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12f38)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3970
void error(int a0,int a1,char *a2,...)
{
  (*dat_12f40)(); // jump-as-call
}

// Function: waitpid @ 0x3980
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_12f48)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3990
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_12f50)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x39a0
void perror(char *a0)
{
  (*dat_12f58)(); // jump-as-call
}

// Function: execvp @ 0x39b0
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_12f60)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x39c0
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_12f68)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x39d0
void exit(int a0)
{
  (*dat_12f70)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x39e0
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_12f78)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x39f0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12f80)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3a00
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_12f88)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x3a10
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_12f90)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3a20
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_12f98)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x3a30
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_12fa0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3a40
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_12fa8)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x3a50
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_12fb0)(); // jump-as-call
  return v1;
}

// Function: feof_unlocked @ 0x3a60
int feof_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12fb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3a70
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12fc0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x3a80
void iconv_open(void)
{
  (*dat_12fc8)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x3a90
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_12fd0)(); // jump-as-call
  return v1;
}

// Function: sub_3aa0 @ 0x3aa0
undefined16 sub_3aa0(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x3ac0
int main(int argc,char **argv,char **envp) // return-dupe
{
  char *v1;
  char **v10;
  unsigned long v11 [4]; // stack - 0xf8
  stat v12; // stack - 0xd8
  int v13 [3]; // stack - 0x104
  char v14 [8];
  void *v15; // stack - 0x118
  void *v16; // stack - 0x120
  unsigned int v17; // stack - 0x110
  int v18; // stack - 0x12c
  int v19; // stack - 0x10c
  char *v2;
  int v20; // stack - 0x108
  unsigned char v3; // al
  int v4;
  int v5; // eax
  long v6; // rax
  unsigned long v7; // rax
  int *v8; // rax
  unsigned int v9;
  
  v9 = 0;
  dat_13020 = 2;
  sub_62e0(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_6220(NULL);
  sub_86a0();
  v18 = 0;
  while (v4 = getopt_long(argc,argv,"aeimvx3AEL:TX",(option *)0x12920,NULL), v4 != -1) {
    switch(v4) {
      case 0x33:
        dat_130ed = 1;
        v9 |= 1;
        break;
      default:
        sub_41b0(NULL,0); // no-return
      case 0x41:
        dat_130eb = 1;
        v9 |= 2;
        dat_130ef = 1;
        break;
      case 0x45:
        dat_130ef = 1;
        v9 |= 4;
        break;
      case 0x4c:
        if (2 < v18)
          sub_41b0("too many file label options",0); // no-return
        v11[v18] = optarg;
        v18 += 1;
        break;
      case 0x54:
        dat_130ee = 1;
        break;
      case 0x58:
        dat_130ec = 1;
        v9 |= 8;
        break;
      case 0x61:
        dat_130f2 = 1;
        break;
      case 0x65:
        v9 |= 0x10;
        break;
      case 0x69:
        dat_130ea = '\x01';
        break;
      case 0x6d:
        dat_130e9 = 1;
        break;
      case 0x76:
        sub_8010(stdout,"diff3","GNU diffutils",dat_13018,sub_65f0("Randy Smith"),0);
        sub_5c30();
        return 0;
      case 0x78:
        dat_130ec = 1;
        v9 |= 0x20;
        break;
      case 0x80:
        dat_13010 = optarg;
        break;
      case 0x81:
        sub_4240();
        sub_5c30();
        return 0;
      case 0x82:
        dat_130f1 = 1;
      
    }
  }
  dat_130f0 = (dat_130e9 ^ 1) & v9 != 0;
  v3 = v9 == 0 & dat_130e9;
  dat_130eb |= v3;
  dat_130ef = v3 | dat_130ef;
  if (((v9 - 1 & v9) || ((dat_130ea && (dat_130e9)))) || ((v18 && (!dat_130ef))))
    sub_41b0("incompatible options",0); // no-return
  if (argc - optind != 3) {
    if (3 <= argc - optind)
      sub_41b0("extra operand \'%s\'",argv[(long)optind + 3]); // no-return
    sub_41b0("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  }
  argv = &argv[optind];
  for (v6 = (long)v18; (int)v6 <= 2; v6 = v6 + 1) {
    v11[v6] = argv[v6];
  }
  v9 = (unsigned int)(v9 != 0 | dat_130e9);
  v4 = 2 - v9;
  v6 = (long)v4;
  v1 = argv[v6];
  if (strcmp(v1,"-")) {
label_3def:
    v19 = 3 - v4;
    v17 = 0;
    v13[0] = 0;
    v13[v19] = 1;
    v13[v6] = 2;
    v10 = argv;
    v20 = v4;
    do {
      v1 = *v10;
      if (strcmp(v1,"-")) {
        if (stat(v1,&v12) < 0)
          sub_43a0(*v10); // no-return
        if ((v12._24_4_ & 0xf000) == 0x4000) {
          error(2,0x15,"%s",*v10);
          return v5;
        }
      }
      v10 = &v10[1];
      if (&argv[3] == v10) {
        signal(0x11,0);
        v1 = argv[v13[2]];
        v7 = sub_5d70(argv[v13[1]],v1,v14,&v15);
        v2 = argv[v13[0]];
        v8 = (int *)sub_4e40((void *)sub_5d70(v2,v1,v14,&v16));
        if (dat_130f0) // branch-flip
          v9 = sub_50f0(stdout,v8,&v17,v13,v11[0],v11[1],v11[2],v7);
        else if (dat_130e9) { // branch-flip
          sub_8570(argv[v13[0]],"r",stdin);
          v9 = sub_57d0(stdin,stdout,v8,&v17,v13,v11[0]);
          if (ferror_unlocked(stdin))
            sub_4210("read failed"); // no-return
        }
        else {
          sub_5520(stdout,v8,&v17,v13);
          v9 = 0;
        }
        free(v16);
        free(v15);
        sub_5c30();
        exit(v9 & 0xff); // no-return
      }
    } while( true );
  }
  v1 = *argv;
  v4 = v9 + 1;
  if (strcmp(v1,"-")) {
    v6 = (long)v4;
    v1 = argv[v6];
    if (strcmp(v1,"-")) goto label_3def;
  }
  sub_4210("\'-\' specified for more than one input file"); // no-return
}

// Function: sub_4020 @ 0x4020
void sub_4020(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_12fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4050 @ 0x4050
void sub_4050(void)
{
  return;
}

// Function: _FINI_0 @ 0x40c0
void _FINI_0(void)
{
  if (!dat_130e8) {
    if (dat_12ff8)
      __cxa_finalize(dat_13008);
    sub_4050();
    dat_130e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4100
void _INIT_0(void)
{
  return;
}

// Function: sub_4110 @ 0x4110
void sub_4110(char *a0)
{
  char v1;
  
  v1 = *a0;
  if (v1 != ' ') goto label_412d;
  do {
    do {
      v1 = a0[1];
      a0 = &a0[1];
    } while (v1 == ' ');
label_412d:
  } while (v1 == '\t');
}

// Function: sub_4140 @ 0x4140
unsigned char * sub_4140(unsigned char *a0,long *a1)
{
  unsigned char *v1;
  unsigned int v2; // edx
  
  v2 = (unsigned int)*a0;
  v1 = NULL;
  if ((unsigned int)*a0 - 0x30 <= 9) {
    do {
      a0 = &a0[1];
      v1 = (unsigned char *)((long)(int)(v2 - 0x30) + (long)v1 * 10);
      v2 = (unsigned int)*a0;
    } while (v2 - 0x30 <= 9);
    *a1 = (long)v1;
    return a0;
  }
  return v1;
}

// Function: sub_4180 @ 0x4180
void sub_4180(void *a0) // return-dupe
{
  void *v1; // rax
  void *v2;
  
  if (!a0)
    return;
  v2 = NULL;
  while( true ) {
    v1 = a0;
    a0 = *(void **)((long)v1 + 0x68);
    *(void **)((long)v1 + 0x68) = v2;
    if (!a0) break;
    v2 = v1;
  }
}

// Function: sub_41b0 @ 0x41b0
void sub_41b0(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_23140;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}

// Function: sub_4210 @ 0x4210
void sub_4210(char *a0)
{
  error(2,0,"%s",dcgettext(NULL,a0,5));
}

// Function: sub_4240 @ 0x4240
void sub_4240(void)
{
  FILE *v1;
  char v2;
  long v3; // rbx
  char *v4;
  
  v4 = dat_23140;
  v3 = 0x12880;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5),v4);
  __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare three files line by line.",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v2 = '-';
  v4 = "-A, --show-all              output all changes, bracketing conflicts";
  while( true ) {
    if (v2) { // branch-flip
      __printf_chk(1,"  %s\n",dcgettext(NULL,v4,5));
      v4 = *(char **)(v3 + 8);
      v1 = stdout;
    }
    else {
      putchar_unlocked(10);
      v4 = *(char **)(v3 + 8);
      v1 = stdout;
    }
    stdout = v1;
    if (!v4) break;
    v3 += 8;
    v2 = *v4;
  }
  fputs_unlocked(dcgettext(NULL,"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n",5),v1);
  v4 = dcgettext(NULL,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
  __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v4);
  sub_80c0(); // tail-call
}

// Function: sub_43a0 @ 0x43a0
void sub_43a0(char *a0)
{
  error(2,*__errno_location(),"%s",a0);
}

// Function: sub_43d0 @ 0x43d0
long sub_43d0(char *a0,char *a1,unsigned long *a2)
{
  char *v1;
  long v10; // rbx
  char *v11 [9]; // stack - 0x120
  int v12; // stack - 0x130
  stat v13; // stack - 0xd8
  unsigned int v14; // stack - 0x134
  char *v15; // stack - 0x128
  char *v16; // rsi
  long v17; // r14
  int v18; // stack - 0x12c
  int v2; // eax
  char **v3;
  char **v4;
  unsigned long v5;
  void *v6; // rax
  long v7; // rax
  long v8; // rax
  unsigned int v9; // ebx
  
  v15 = dat_13010;
  v3 = v11;
  if (dat_130f2) {
    v11[0] = "-a";
    v3 = &v11[1];
  }
  v4 = v3;
  if (dat_130f1) {
    v4 = &v3[1];
    *v3 = "--strip-trailing-cr";
  }
  v4[2] = a0;
  *v4 = "--horizon-lines=100";
  v4[1] = "--";
  v4[3] = a1;
  v4[4] = NULL;
  if (pipe(&v12))
    sub_43a0("pipe"); // no-return
  v2 = fork();
  if (!v2) {
    close(v12);
    if (v18 != 1) {
      dup2(v18,1);
      close(v18);
    }
    execvp(dat_13010,&v15);
    _exit((*__errno_location() == 2) + 0x7e); // no-return
  }
  if (v2 != -1) {
    close(v18);
    if (fstat(v12,&v13))
      sub_43a0("fstat"); // no-return
    v5 = 1;
    if (1 <= (long)v13._56_8_)
      v5 = v13._56_8_;
    v10 = 0;
    v6 = (void *)sub_8180(v5);
    while( true ) {
      v17 = v5 - v10;
      v7 = sub_8770(v12,(void *)((long)v6 + v10),v17);
      v10 += v7;
      if (v17 != v7) break;
      if (0x3ffffffffffffffe < v5)
        sub_8530(); // no-return
      v5 *= 2;
      v6 = (void *)sub_81d0(v6,v5);
    }
    if (v7 == -1)
      sub_43a0(dcgettext(NULL,"read failed",5)); // no-return
    if ((v10) && (*(char *)((long)v6 + v10 + -1) != '\n'))
      sub_4210("invalid diff format; incomplete last line"); // no-return
    *a2 = v6;
    if (close(v12))
      sub_43a0("close"); // no-return
    v2 = waitpid(v2,(int *)&v14,0);
    v1 = dat_13010;
    if (v2 <= -1)
      sub_43a0("waitpid"); // no-return
    if (v14 & 0x7f) { // branch-flip
      v9 = 0x7fffffff;
      v16 = "subsidiary program \'%s\' failed";
    }
    else {
      if (!((int)v14 >> 8 & 0xfeU))
        return (long)v6 + v10;
      v9 = (int)v14 >> 8 & 0xff;
      if ((char)(v14 >> 8) != '~') { // branch-flip
        v16 = "subsidiary program \'%s\' failed (exit status %d)";
        if (v9 == 0x7f)
          v16 = "subsidiary program \'%s\' not found";
      }
      else {
        v16 = "subsidiary program \'%s\' could not be invoked";
      }
    }
    error(2,0,dcgettext(NULL,v16,5),v1,(unsigned long)v9);
    return v8;
  }
  sub_43a0("fork"); // no-return
}

// Function: sub_46d0 @ 0x46d0
void sub_46d0(FILE *a0,bool a1,long a2,long a3)
{
  fputs_unlocked(".\n",a0);
  if (!a1)
    return;
  if (a3 != 1) {
    __fprintf_chk(a0,1,"%ld,%lds/^\\.//\n",a2,a2 + -1 + a3); // tail-call
    return;
  }
  __fprintf_chk(a0,1,"%lds/^\\.//\n",a2); // tail-call
}

// Function: sub_4770 @ 0x4770
char * sub_4770(char *a0,unsigned long *a1,long *a2,char *a3,char a4)
{
  char v1;
  char *v2;
  char *v3;
  
  if ((*a0 == a4) && (a0[1] == ' ')) {
    a0 = &a0[2];
    *a1 = (unsigned long)a0;
    v2 = a0;
    while (v3 = &v2[1], *v2 != '\n') {
      v2 = v3;
    }
    *a2 = (long)v3 - (long)a0;
    if ((v3 < a3) && (*v3 == '\\')) {
      if (dat_130f0) // branch-flip
        __fprintf_chk(stderr,1,"%s:",dat_23140);
      else {
        *a2 = ((long)v3 - (long)a0) + -1;
      }
      v2 = &v2[2];
      v1 = dat_130f0;
      while( true ) {
        if (v1)
          putc_unlocked((int)*v2,stderr);
        v3 = &v2[1];
        if (*v2 == '\n') break;
        v2 = v3;
        v1 = dat_130f0;
      }
    }
    return v3;
  }
  sub_4210("invalid diff format; incorrect leading line chars"); // no-return
}

// Function: sub_4850 @ 0x4850
unsigned int * sub_4850(long a0,long a1,long a2,long a3,unsigned long a4,unsigned long a5)
{
  unsigned int *v1; // rax
  unsigned long v2; // rax
  unsigned long v3;
  
  v1 = (unsigned int *)sub_8180(0x70);
  *(long *)&v1[4] = a1;
  *v1 = 0;
  *(unsigned long *)&v1[0x1a] = 0;
  *(long *)&v1[2] = a0;
  *(long *)&v1[6] = a2;
  *(long *)&v1[8] = a3;
  *(unsigned long *)&v1[10] = a4;
  *(unsigned long *)&v1[0xc] = a5;
  v3 = (a1 - a0) + 1;
  if (v3) { // branch-flip
    *(unsigned long *)&v1[0xe] = sub_8410(v3,8);
    v2 = sub_8410(v3,8);
    *(unsigned long *)&v1[0x14] = v2;
    a3 = *(long *)&v1[8] - *(long *)&v1[6];
  }
  else {
    a3 -= a2;
    *(unsigned long *)&v1[0xe] = 0;
    *(unsigned long *)&v1[0x14] = 0;
  }
  v3 = a3 + 1;
  if (v3) { // branch-flip
    *(unsigned long *)&v1[0x10] = sub_8410(v3,8);
    v2 = sub_8410(v3,8);
  }
  else {
    *(unsigned long *)&v1[0x10] = 0;
    v2 = 0;
  }
  *(unsigned long *)&v1[0x16] = v2;
  v3 = (*(long *)&v1[0xc] - *(long *)&v1[10]) + 1;
  if (v3) { // branch-flip
    *(unsigned long *)&v1[0x12] = sub_8410(v3,8);
    v2 = sub_8410(v3,8);
  }
  else {
    *(unsigned long *)&v1[0x12] = 0;
    v2 = 0;
  }
  *(unsigned long *)&v1[0x18] = v2;
  return v1;
}

// Function: sub_49b0 @ 0x49b0
unsigned long sub_49b0(long a0,long a1,long a2,long a3,long a4)
{
  void *v1;
  void *v2;
  unsigned long v3;
  long v4; // rbx
  
  if (!a4)
    return 1;
  v4 = 0;
  while( true ) {
    while( true ) {
      v2 = *(void **)(a2 + v4 * 8);
      v3 = *(unsigned long *)(a1 + v4 * 8);
      if (v2) break;
      *(unsigned long *)(a2 + v4 * 8) = *(unsigned long *)(a0 + v4 * 8);
      *(unsigned long *)(a3 + v4 * 8) = v3;
      v4 += 1;
      if (v4 == a4)
        return 1;
    }
    if (*(unsigned long *)(a3 + v4 * 8) != v3)
      return 0;
    v1 = *(void **)(a0 + v4 * 8);
    if (memcmp(v1,v2,v3)) break;
    v4 += 1;
    if (v4 == a4)
      return 1;
  }
  return 0;
}

// Function: sub_4a50 @ 0x4a50
unsigned long sub_4a50(long a0,long a1,long a2,long a3,long a4)
{
  void *v1;
  void *v2;
  unsigned long v3;
  long v4; // rbx
  
  v4 = 0;
  while( true ) {
    if (a4 == v4)
      return 1;
    v1 = *(void **)(a0 + v4 * 8);
    if (!v1)
      return 0;
    v2 = *(void **)(a2 + v4 * 8);
    if (!v2)
      return 0;
    v3 = *(unsigned long *)(a1 + v4 * 8);
    if (v3 != *(unsigned long *)(a3 + v4 * 8)) break;
    v4 += 1;
    if (memcmp(v1,v2,v3))
      return 0;
  }
  return 0;
}

// Function: sub_4ae0 @ 0x4ae0
unsigned int * sub_4ae0(long *a0,long a1,int a2,int a3,void *a4)
{
  long *v1;
  long v10;
  long v11; // rdx
  long v12;
  long *v13;
  long v14 [5]; // stack - 0x68
  long v15;
  long v16; // stack - 0x90
  long *v2;
  long v3;
  long v4;
  long v5;
  unsigned int *v6; // rax
  long v7;
  long v8;
  long v9;
  
  v3 = *(long *)(a0[a2] + 0x10);
  v15 = *(long *)(*(long *)(a1 + (long)a3 * 8) + 0x18);
  v5 = 0;
  do {
    v13 = *(long **)((long)a0 + v5);
    if (v13) { // branch-flip
      *(long *)((long)v14 + v5) = (v3 - v13[2]) + *v13;
      v9 = (v15 - *(long *)(*(long *)(a1 + v5) + 0x18)) + *(long *)(*(long *)(a1 + v5) + 8);
    }
    else {
      *(long *)((long)v14 + v5) = (v3 - *(long *)((long)a4 + 0x30)) + *(long *)((long)a4 + v5 * 2 + 0x10);
      v9 = (v15 - *(long *)((long)a4 + 0x30)) + *(long *)((long)a4 + v5 * 2 + 0x10);
    }
    *(long *)((long)v14 + v5 + 0x10) = v9;
    v5 += 8;
  } while (v5 != 0x10);
  v6 = (unsigned int *)sub_4850(v14[0],v14[2],v14[1],v14[3],v3);
  v16 = 0;
  while( true ) {
    v15 = a0[v16];
    if (v15) {
      v5 = *(long *)&v6[0x18];
      v9 = *(long *)&v6[0x12];
      do {
        v7 = *(long *)(v15 + 0x10);
        v12 = *(long *)(v15 + 0x18);
        v8 = *(long *)(v15 + 0x38);
        v4 = *(long *)(v15 + 0x28);
        v10 = (v7 - v3) * 8;
        if (!sub_49b0(v4,v8,v10 + v9,v5 + v10,(v12 - v7) + 1))
          return NULL;
        v15 = *(long *)(v15 + 0x40);
      } while (v15);
    }
    if (v16 == 1) break;
    v16 = 1;
  }
  v15 = v14[0];
  v5 = 0;
  while( true ) {
    v13 = *(long **)((long)a0 + v5);
    v7 = 0;
    v9 = v15;
    while( true ) {
      v12 = v14[2] + 1;
      if (v13)
        v12 = *v13;
      if (v12 <= v9) break;
      v9 += 1;
      *(unsigned long *)(*(long *)((long)v6 + v5 + 0x38) + v7) = *(unsigned long *)(*(long *)&v6[0x12] + v7);
      *(unsigned long *)(*(long *)((long)v6 + v5 + 0x50) + v7) = *(unsigned long *)(*(long *)&v6[0x18] + v7);
      v7 += 8;
    }
    if (v13) break;
label_4d8b:
    if (v5 + 8 == 0x10) {
      if (*a0) { // branch-flip
        if (a0[1]) { // branch-flip
          v3 = *(long *)&v6[4];
          v15 = *(long *)&v6[2];
          if ((v3 - v15 != *(long *)&v6[8] - *(long *)&v6[6]) || (v5 = *(long *)&v6[0x16], v9 = *(long *)&v6[0x10], v7 = *(long *)&v6[0x14], v12 = *(long *)&v6[0xe], !sub_4a50(v12,v7,v9,v5,(v3 - v15) + 1)))
            *v6 = 4;
          else {
            *v6 = 7;
          }
        }
        else {
          *v6 = 5;
        }
      }
      else {
        *v6 = 6;
      }
      return v6;
    }
    v15 = *(long *)((long)v14 + v5 + 8);
    v14[2] = *(long *)((long)v14 + v5 + 0x18);
    v5 += 8;
  }
  v9 = *(long *)((long)v6 + v5 + 0x38);
  v7 = *(long *)((long)v6 + v5 + 0x50);
label_4cf0:
  v12 = *v13;
  v8 = v13[1];
  v4 = v13[6];
  v10 = v13[4];
  v11 = (v12 - v15) * 8;
  if (!sub_49b0(v10,v4,v11 + v9,v7 + v11,(v8 - v12) + 1))
    return NULL;
  v1 = &v13[1];
  v2 = &v13[3];
  v13 = (long *)v13[8];
  v12 = (*v2 - v3) * 8;
  v8 = (*v1 + 1) - v15;
  do {
    v12 += 8;
    if (v13) { // branch-flip
      if (*v13 - v15 <= v8) goto label_4cf0;
    }
    else if ((v14[2] + 1) - v15 <= v8) goto label_4d8b;
    *(unsigned long *)(v9 + v8 * 8) = *(unsigned long *)(*(long *)&v6[0x12] + v12);
    *(unsigned long *)(v7 + v8 * 8) = *(unsigned long *)(*(long *)&v6[0x18] + v12);
    v8 += 1;
  } while( true );
}

// Function: sub_4e40 @ 0x4e40
void * sub_4e40(void *a0,void *a1)
{
  long v1;
  long v10 [5]; // stack - 0x50
  void *v11 [3]; // stack - 0x28
  unsigned long v12; // rsi
  long v13;
  unsigned int v14;
  unsigned int v15; // r10d
  void *v2;
  void *v3;
  unsigned int v4;
  unsigned int v5;
  int v6; // eax
  void *v7; // rax
  long v8;
  long *v9;
  
  v13 = 0xdd60;
  v9 = v10;
  v11[0] = a0;
  v10[0] = 0;
  v11[1] = a1;
  v2 = a1;
  if (!a0) goto label_4fab;
  do {
    v7 = v2;
    v10[4] = 0;
    v10[3] = 0;
    v10[2] = 0;
    v10[1] = 0;
    if (v7) { // branch-flip
      v8 = *(long *)((long)v7 + 0x10);
      v1 = *(long *)((long)a0 + 0x10);
      v15 = (unsigned int)(v8 < v1);
      v12 = (unsigned long)(v8 < v1);
      v7 = (unsigned long)(unsigned long)(v1 <= v8);
      a0 = v11[v12];
    }
    else {
      v7 = (void *)0x1;
      v15 = 0;
      v12 = 0;
    }
    while( true ) {
      v10[4] = 0;
      v10[3] = 0;
      v10[2] = 0;
      v10[1] = 0;
      v2 = *(void **)((long)a0 + 0x40);
      v8 = *(long *)((long)a0 + 0x18);
      v10[v12 + 3] = (long)a0;
      v10[v12 + 1] = (long)a0;
      v11[v12] = v2;
      *(unsigned long *)((long)a0 + 0x40) = 0;
      v2 = v11[(long)v7];
      v4 = v15;
      while ((v2 && (*(long *)((long)v2 + 0x10) <= v8 + 1))) {
        if (v10[(long)v7 + 1]) // branch-flip
          *(void **)(v10[(long)v7 + 3] + 0x40) = v2;
        else {
          v10[(long)v7 + 1] = (long)v2;
        }
        v3 = v11[(long)v7];
        v10[(long)v7 + 3] = (long)v2;
        v3 = *(void **)((long)v3 + 0x40);
        *(unsigned long *)((long)v2 + 0x40) = 0;
        v1 = *(long *)((long)v2 + 0x18);
        v11[(long)v7] = v3;
        v6 = v4 ^ 1;
        v5 = v6;
        v14 = v4;
        if (v8 < v1) {
          v8 = v1;
          v5 = v4;
          v14 = v6;
        }
        v6 = v5;
        v7 = (unsigned long)(long)v6;
        v2 = v11[(long)v7];
        v4 = v14;
      }
      v7 = (long)sub_4ae0(&v10[1],&v10[3],v15,v4,v13);
      if (!v7)
        sub_4210("internal error: screwup in format of diff blocks"); // no-return
      *v9 = (long)v7;
      v9 = (long *)((long)v7 + 0x68);
      v2 = v11[1];
      a0 = v11[0];
      v13 = (long)v7;
      if (v11[0]) break;
label_4fab:
      v7 = v2;
      a0 = v7;
      if (!v7) {
        v7 = (void *)v10[0];
        return v7;
      }
      v12 = 1;
      v7 = NULL;
      v10[4] = 0;
      v10[3] = 0;
      v15 = 1;
      v10[2] = 0;
      v10[1] = 0;
    }
  } while( true );
}

// Function: sub_5030 @ 0x5030
char sub_5030(FILE *a0,long a1,int a2)
{
  long v1;
  char *v2;
  long v3; // rbx
  long v4; // r13
  long v5; // r14
  char v6; // stack - 0x41
  
  v4 = (long)a2;
  v5 = v4 * 0x10 + a1;
  if (*(long *)(v5 + 8) <= *(long *)(v5 + 0x10)) { // branch-flip
    v6 = 0;
    v3 = 0;
    do {
      v1 = v3 * 8;
      v2 = *(char **)(*(long *)(a1 + 0x38 + v4 * 8) + v3 * 8);
      if (*v2 == '.') {
        fputc_unlocked(0x2e,a0);
        v6 = 1;
      }
      v3 += 1;
      fwrite_unlocked(v2,1,*(unsigned long *)(*(long *)(a1 + 0x50 + v4 * 8) + v1),a0);
    } while (v3 <= *(long *)(v5 + 0x10) - *(long *)(v5 + 8));
  }
  else {
    v6 = 0;
  }
  return v6;
}

// Function: sub_50f0 @ 0x50f0
unsigned long sub_50f0(FILE *a0,void *a1,int *a2,long a3,char *a4,char *a5,char *a6)
{
  long v1;
  int *v2;
  long v3; // rax
  long v4;
  long v5;
  unsigned int v6; // eax
  int v7;
  unsigned long v8; // r14
  
  v2 = (int *)sub_4180(a1);
  if (v2) { // branch-flip
    v8 = 0;
label_515f:
    do {
      if (*v2 != 4) { // branch-flip
        v7 = *(int *)(a3 + (unsigned long)(unsigned int)(*v2 - 5) * 4);
        if (v7 != 1) { // branch-flip
          if (v7 != 2) { // branch-flip
            if (v7 == -1) goto label_5167;
          }
          else if (!dat_130ec) {
            v5 = *(long *)&v2[(long)*a2 * 4 + 2];
            v4 = *(long *)&v2[(long)*a2 * 4 + 4];
            goto label_518f;
          }
        }
        else if (dat_130eb) {
          v7 = 6;
          v4 = *(long *)&v2[(long)*a2 * 4 + 4];
          v5 = *(long *)&v2[(long)*a2 * 4 + 2];
          __fprintf_chk(a0,1,"%lda\n",v4);
          v6 = 0;
          goto label_52b9;
        }
      }
      else {
label_5167:
        if (!dat_130ed) {
          v5 = *(long *)&v2[(long)*a2 * 4 + 2];
          v4 = *(long *)&v2[(long)*a2 * 4 + 4];
          if (dat_130ef) { // branch-flip
            __fprintf_chk(a0,1,"%lda\n",v4);
            v6 = (unsigned int)dat_130eb;
            if (dat_130eb) {
              __fprintf_chk(a0,1,"||||||| %s\n",a5);
              v6 = sub_5030(a0,v2,a2[1]);
            }
            fputs_unlocked("=======\n",a0);
            v7 = a2[2];
            v6 = (v6 | sub_5030(a0,v2,v7)) & 0xff;
            v7 = 4;
label_52b9:
            __fprintf_chk(a0,1,">>>>>>> %s\n",a6);
            sub_46d0(a0,v6,v4 + 2,(*(long *)&v2[(long)a2[1] * 4 + 4] - *(long *)&v2[(long)a2[1] * 4 + 2]) + 3 + (*(long *)&v2[(long)a2[2] * 4 + 4] - *(long *)&v2[(long)a2[2] * 4 + 2]));
            if (v7 != 4) { // branch-flip
              __fprintf_chk(a0,1,"%lda\n<<<<<<< %s\n",v5 + -1,a5);
              v6 = 0;
              if (v7 == 6) {
                v6 = sub_5030(a0,v2,a2[1]);
                fputs_unlocked("=======\n",a0);
                v6 &= 0xff;
              }
            }
            else {
              __fprintf_chk(a0,1,"%lda\n<<<<<<< %s\n",v5 + -1,a4);
              v6 = 0;
            }
            v8 = 1;
            sub_46d0(a0,v6,v5 + 1,(*(long *)&v2[(long)a2[1] * 4 + 4] - *(long *)&v2[(long)a2[1] * 4 + 2]) + 1);
          }
          else {
label_518f:
            if (*(long *)&v2[(long)a2[2] * 4 + 4] - *(long *)&v2[(long)a2[2] * 4 + 2] != -1) {
              if (v4 - v5 != -1) { // branch-flip
                if (v4 != v5) // branch-flip
                  __fprintf_chk(a0,1,"%ld,%ldc\n",v5,v4);
                else {
                  __fprintf_chk(a0,1,"%ldc\n",v4);
                }
              }
              else {
                __fprintf_chk(a0,1,"%lda\n",v4);
              }
              v3 = (long)a2[2];
              v4 = *(long *)&v2[v3 * 4 + 4];
              v1 = *(long *)&v2[v3 * 4 + 2];
              sub_46d0(a0,(unsigned char)sub_5030(a0,v2,v3),v5,(v4 - v1) + 1);
              v2 = *(int **)&v2[0x1a];
              if (!v2) break;
              goto label_515f;
            }
            if (v5 != v4) // branch-flip
              __fprintf_chk(a0,1,"%ld,%ldd\n",v5,v4);
            else {
              __fprintf_chk(a0,1,"%ldd\n",v5);
            }
          }
        }
      }
      v2 = *(int **)&v2[0x1a];
    } while (v2);
  }
  else {
    v8 = 0;
  }
  if (dat_130ea)
    fputs_unlocked("w\nq\n",a0);
  return v8;
}

// Function: sub_5520 @ 0x5520
void sub_5520(FILE *a0,int *a1,long a2,long a3) // return-dupe
{
  long v1;
  long v10; // r12
  int v11; // stack - 0x58
  unsigned int v12; // stack - 0x54
  long v2;
  void *v3;
  unsigned long v4;
  int v5;
  unsigned int v6;
  char *v7; // rax
  char v8 [2]; // stack - 0x42
  long v9; // rbp
  
  v7 = "  ";
  if (dat_130ee)
    v7 = "\t";
  if (a1) {
    do {
      if (*a1 != 4) { // branch-flip
        v6 = *a1 - 5;
        if (3 <= v6)
          sub_4210("internal error: invalid diff type passed to output"); // no-return
        v8[1] = 0;
        v11 = *(int *)(a3 + (unsigned long)v6 * 4);
        v8[0] = (char)v11 + '1';
        v12 = (unsigned int)(v11 == 0);
      }
      else {
        v8[0] = '\0';
        v12 = 3;
        v11 = 3;
      }
      __fprintf_chk(a0,1,"====%s\n",v8);
      v6 = 0;
      do {
        v9 = (long)*(int *)(a2 + (long)(int)v6 * 4);
        v1 = *(long *)&a1[v9 * 4 + 2];
        v2 = *(long *)&a1[v9 * 4 + 4];
        v5 = v6 + 1;
        __fprintf_chk(a0,1,"%d:",v5);
        if (v1 != v2) { // branch-flip
          if (v1 - v2 != 1) // branch-flip
            __fprintf_chk(a0,1,"%ld,%ldc\n",v1,v2);
          else {
            __fprintf_chk(a0,1,"%lda\n",v1 + -1);
          }
        }
        else {
          __fprintf_chk(a0,1,"%ldc\n",v1);
        }
        if ((v1 <= v2) && (v12 != v6)) {
          v10 = 0;
          do {
            fputs_unlocked(v7,a0);
            v3 = *(void **)(*(long *)&a1[(v9 + 6) * 2 + 2] + v10 * 8);
            v4 = *(unsigned long *)(*(long *)&a1[v9 * 2 + 0x14] + v10 * 8);
            v10 += 1;
            fwrite_unlocked(v3,1,v4,a0);
          } while (v10 <= v2 - v1);
          if (*(char *)((long)v3 + (v4 - 1)) != '\n')
            __fprintf_chk(a0,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5));
        }
        if (v11 == 1)
          v5 = *(unsigned int *)((long)(int)v6 * 4 + 0xdd40);
        v6 = v5;
      } while (v5 <= 2);
      a1 = *(int **)&a1[0x1a];
    } while (a1);
    return;
  }
}

// Function: sub_57d0 @ 0x57d0
char sub_57d0(FILE *a0,FILE *a1,int *a2,int *a3,long a4,char *a5,unsigned long a6,char *a7)
{
  long v1;
  long v10; // stack - 0x58
  int v11; // stack - 0x50
  bool v12; // stack - 0x4a
  char v13; // stack - 0x49
  long v2;
  unsigned int v3;
  int v4; // eax
  int v5; // eax
  long v6;
  long v7; // rdx
  bool v8; // cf
  char *v9; // stack - 0x60
  
  if (a2) {
    v6 = 0;
    v13 = 0;
    v3 = *a2;
    if (v3 == 4) goto label_584a;
    do {
      v3 = *(int *)(a4 + (unsigned long)(unsigned int)(v3 - 5) * 4);
      if (v3 != 1) { // branch-flip
        if (v3 != 2) { // branch-flip
          if (v3 == -1) goto label_584a;
        }
        else {
          v12 = dat_130ec;
          if (!dat_130ec) {
            v11 = 7;
            v10 = *(long *)&a2[2] + -1;
            v6 = (*(long *)&a2[2] - v6) + -2;
            v9 = "<<<<<<< %s\n";
            if (0 <= v6) goto label_5898;
            goto label_58fa;
          }
        }
      }
      else {
        v12 = dat_130eb;
        if (dat_130eb) {
          v11 = 6;
          v10 = *(long *)&a2[2] + -1;
          v6 = (*(long *)&a2[2] - v6) + -2;
          v9 = "<<<<<<< %s\n";
          if (0 <= v6) goto label_5898;
          goto label_5aef;
        }
      }
      while( true ) {
        a2 = *(int **)&a2[0x1a];
        if (!a2) goto label_59ac;
        v3 = *a2;
        if (v3 != 4) break;
label_584a:
        if (!dat_130ed) {
          v12 = dat_130ef;
          v10 = *(long *)&a2[2] + -1;
          v6 = (*(long *)&a2[2] - v6) + -2;
          if (0 <= v6) { // branch-flip
            v11 = 4;
            v9 = "||||||| %s\n";
label_5898:
            do {
              while (v3 = getc_unlocked(a0), v3 == -1) {
                if (ferror_unlocked(a0)) {
                  sub_43a0(dcgettext(NULL,"read failed",5)); // return-dupe, no-return
                }
                if (feof_unlocked(a0)) {
                  sub_4210("input file shrank"); // return-dupe, no-return
                }
                putc_unlocked(-1,a1);
              }
              putc_unlocked(v3,a1);
            } while ((v3 != 10) || (v8 = v6 != 0, v6 = v6 + -1, v8));
            if (v12) {
              if (v11 == 4) goto label_5b9f;
label_58d4:
              if (dat_130eb) {
label_5aef:
                __fprintf_chk(a1,1,v9,a6);
                v6 = (long)a3[1];
                if (*(long *)&a2[v6 * 4 + 2] <= *(long *)&a2[v6 * 4 + 4]) {
                  v7 = 0;
                  do {
                    v1 = v7 * 8;
                    v2 = v7 * 8;
                    v7 += 1;
                    fwrite_unlocked(*(void **)(*(long *)&a2[v6 * 2 + 0xe] + v2),1,*(unsigned long *)(*(long *)&a2[v6 * 2 + 0x14] + v1),a1);
                    v6 = (long)a3[1];
                  } while (v7 <= *(long *)&a2[v6 * 4 + 4] - *(long *)&a2[v6 * 4 + 2]);
                }
              }
              fputs_unlocked("=======\n",a1);
              v12 = 1;
              v13 = 1;
            }
          }
          else if (dat_130ef) {
            v9 = "||||||| %s\n";
label_5b9f:
            __fprintf_chk(a1,1,"<<<<<<< %s\n",a5);
            for (v6 = 0; v7 = (long)*a3, v6 <= *(long *)&a2[v7 * 4 + 4] - *(long *)&a2[v7 * 4 + 2]; v6 = v6 + 1) {
              fwrite_unlocked(*(void **)(*(long *)&a2[v7 * 2 + 0xe] + v6 * 8),1,*(unsigned long *)(*(long *)&a2[v7 * 2 + 0x14] + v6 * 8),a1);
            }
            goto label_58d4;
          }
label_58fa:
          v6 = (long)a3[2];
          if (*(long *)&a2[v6 * 4 + 2] <= *(long *)&a2[v6 * 4 + 4]) {
            v7 = 0;
            do {
              v1 = v7 * 8;
              v2 = v7 * 8;
              v7 += 1;
              fwrite_unlocked(*(void **)(*(long *)&a2[v6 * 2 + 0xe] + v2),1,*(unsigned long *)(*(long *)&a2[v6 * 2 + 0x14] + v1),a1);
              v6 = (long)a3[2];
            } while (v7 <= *(long *)&a2[v6 * 4 + 4] - *(long *)&a2[v6 * 4 + 2]);
          }
          if (v12)
            __fprintf_chk(a1,1,">>>>>>> %s\n",a7);
          v7 = *(long *)&a2[4] - *(long *)&a2[2];
          v6 = v10 + 1 + v7;
          if (0 <= v7) {
            do {
              while (v3 = getc_unlocked(a0), v3 != 10) {
                if (v3 == -1) {
                  if (ferror_unlocked(a0))
                    sub_43a0(dcgettext(NULL,"read failed",5));
                  if (feof_unlocked(a0)) {
                    if ((!v7) && (!*(long *)&a2[0x1a]))
                      return v13;
                    sub_4210("input file shrank");
                  }
                }
              }
              v8 = v7 != 0;
              v7 -= 1;
            } while (v8);
          }
        }
      }
    } while( true );
  }
  v13 = 0;
label_59ac:
  do {
    v3 = getc_unlocked(a0);
    if (v3 == -1) {
      v4 = ferror_unlocked(a0);
      v5 = feof_unlocked(a0);
      if (v4 || v5)
        return v13;
    }
    putc_unlocked(v3,a1);
  } while( true );
}

// Function: sub_5c30 @ 0x5c30
void sub_5c30(void)
{
  if (ferror_unlocked(stdout))
    sub_4210("write failed"); // no-return
  if (!fclose(stdout))
    return;
  sub_43a0(dcgettext(NULL,"standard output",5)); // no-return
}

// Function: sub_5c80 @ 0x5c80
unsigned long sub_5c80(unsigned long *a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  char v2;
  char *v3;
  unsigned long *v4;
  unsigned long v5; // r10
  
  v3 = (char *)*a0;
  v4 = a1;
  v3 = (char *)sub_4140((unsigned char *)sub_4110(v3));
  if (!v3)
    return 0;
  v3 = (char *)sub_4110(v3);
  if (*v3 != ',') // branch-flip
    a1[1] = *a1;
  else {
    v3 = (char *)sub_4140(&v3[1],&a1[1]);
    if (!v3)
      return 0;
  }
  v3 = (char *)sub_4110(v3);
  v2 = *v3;
  if (v2 != 'c') { // branch-flip
    if (v2 != 'd') { // branch-flip
      if (v2 != 'a')
        return 0;
      v5 = 1;
    }
    else {
      v5 = 3;
    }
  }
  else {
    v5 = 2;
  }
  v1 = &v4[2];
  v3 = (char *)sub_4140((unsigned char *)sub_4110(&v3[1],v1));
  if (!v3)
    return 0;
  v3 = (char *)sub_4110(v3);
  if (*v3 != ',') // branch-flip
    v4[3] = v4[2];
  else {
    v3 = (char *)sub_4140(&v3[1],&v4[3]);
    if (!v3)
      return 0;
    v3 = (char *)sub_4110(v3);
  }
  *a0 = v3;
  return v5 & 0xffffffff;
}

// Function: sub_5d70 @ 0x5d70
long sub_5d70(char *a0,char *a1,unsigned long *a2,unsigned long *a3)
{
  long v1;
  char *v10; // stack - 0x58
  long v11; // stack - 0x48
  char *v12; // stack - 0x50
  long v13;
  long *v14;
  long *v15; // rax
  unsigned long v2;
  long v3;
  long v4;
  int v5; // eax
  char *v6;
  long v7;
  char *v8;
  char *v9;
  
  v14 = &v11;
  v6 = (char *)sub_43d0(a0,a1,&v10);
  *a3 = v10;
  v12 = v10;
  if (v10 < v6) {
label_5dd0:
    do {
      v15 = (long *)sub_8180(0x48);
      v15[5] = 0;
      v15[4] = 0;
      v15[7] = 0;
      v15[6] = 0;
      v5 = sub_5c80(&v12,v15);
      v9 = dat_23140;
      if ((!v5) || (v8 = &v12[1], *v12 != '\n')) {
        __fprintf_chk(stderr,1,dcgettext(NULL,"%s: diff failed: ",5),v9);
        do {
          v6 = v12;
          putc_unlocked((int)*v12,stderr);
          v12 = &v6[1];
        } while (*v6 != '\n');
        exit(2); // no-return
      }
      v12 = v8;
      if (v5 != 2) { // branch-flip
        if (v5 == 3) {
          v15[2] = v15[2] + 1;
          goto label_5e3d;
        }
        if (v5 != 1)
          sub_4210("internal error: invalid diff type in process_diff"); // no-return
        *v15 = *v15 + 1;
      }
      else {
label_5e3d:
        v3 = v15[1];
        v4 = *v15;
        v2 = (v3 - v4) + 1;
        if (0xffffffffffffffe < v2) {
          sub_8530(); // no-return, return-dupe
        }
        v15[4] = sub_8180(v2 * 8);
        v7 = sub_8180(v2 * 8);
        v15[6] = v7;
        if (v2) {
          v13 = 0;
          while( true ) {
            v8 = (char *)sub_4770(v8,(unsigned long *)(v13 * 8 + v15[4]),(long *)(v7 + v13 * 8),v6,0x3c);
            v12 = v8;
            v1 = v13 + 1;
            if (v3 - v4 == v13) break;
            v7 = v15[6];
            v13 = v1;
          }
        }
        v9 = v12;
        if (v5 != 2) { // branch-flip
          if (v5 == 3) {
            *v14 = (long)v15;
            v14 = &v15[8];
            if (v6 <= v12) break;
            goto label_5dd0;
          }
        }
        else {
          if (strncmp(v12,"---\n",4))
            sub_4210("invalid diff format; invalid change separator"); // no-return
          v12 = &v9[4];
        }
      }
      v3 = v15[3];
      v4 = v15[2];
      v2 = (v3 - v4) + 1;
      if (0xffffffffffffffe < v2)
        sub_8530();
      v15[5] = sub_8180(v2 * 8);
      v7 = sub_8180(v2 * 8);
      v15[7] = v7;
      v9 = v12;
      if (v2) {
        v13 = 0;
        while( true ) {
          v9 = (char *)sub_4770(v9,(unsigned long *)(v13 * 8 + v15[5]),(long *)(v7 + v13 * 8),v6,0x3e);
          v12 = v9;
          v1 = v13 + 1;
          if (v3 - v4 == v13) break;
          v7 = v15[7];
          v13 = v1;
        }
      }
      *v14 = (long)v15;
      v14 = &v15[8];
    } while (v9 < v6);
  }
  *v14 = 0;
  *a2 = v15;
  return v11;
}

// Function: sub_60a0 @ 0x60a0
void sub_60a0(void)
{
  return;
}

// Function: sub_60b0 @ 0x60b0
void sub_60b0(int a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // rax
  char v5 [4104]; // stack - 0x1038
  char *v6;
  long v7; // fs_offset
  unsigned long v8; // stack - 0x30
  
  v8 = *(unsigned long *)(v7 + 0x28);
  (*dat_13120)();
  if (a0) // branch-flip
    v6 = dat_13118;
  else {
    v6 = dat_13110;
  }
  v1 = strlen(dat_13108);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_13108,v1);
    write(2,": ",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_13108,v1,(unsigned long)0x1002);
    *(unsigned short *)v3 = s_ddf1._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_13020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_61d0 @ 0x61d0
unsigned long sub_61d0(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_60b0(0xb); // no-return
}

// Function: sub_61f0 @ 0x61f0
void sub_61f0(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_13100 == 0) & 0xb);
  sub_60b0(a0); // no-return
}

// Function: sub_6220 @ 0x6220
unsigned long sub_6220(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_60a0;
  dat_13120 = a0;
  dat_13118 = dcgettext(NULL,"program error",5);
  dat_13110 = dcgettext(NULL,"stack overflow",5);
  dat_13108 = sub_62d0();
  if (!sub_6d60(sub_61f0,0x13140,0x10000)) {
    dat_13100 = sub_6ce0(sub_61d0);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_62d0 @ 0x62d0
unsigned long sub_62d0(void)
{
  return __progname;
}

// Function: sub_62e0 @ 0x62e0
void sub_62e0(char *a0)
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
    dat_23140 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_6380 @ 0x6380
unsigned long sub_6380(char *a0,char *a1)
{
  char v1;
  char v10; // stack - 0xac
  char *v11; // stack - 0xa8
  long v12; // stack - 0xa0
  char v13; // stack - 0x98
  unsigned int v14; // stack - 0x94
  unsigned long v15; // stack - 0x74
  char v16; // stack - 0x6c
  char *v17; // stack - 0x68
  long v18; // stack - 0x60
  char v19; // stack - 0x58
  unsigned char v2;
  int v20; // stack - 0x54
  char *v3; // rax
  long *v4; // rax
  char *v5; // rax
  char v6 [4]; // stack - 0xb8
  char v7 [4]; // stack - 0x78
  unsigned char *v8;
  unsigned long v9; // stack - 0xb4
  
  v3 = (char *)sub_77c0(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_9120(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_64a9:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = 0;
          sub_97c0(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_64a9;
        v8 = (unsigned char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),iswalnum(v14) == 0);
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v17 = v3;
      v7[0] = 0;
      v15 = 0;
      while( true ) {
        v10 = 0;
        v16 = 0;
        sub_97c0(v7);
        if ((v19) && (!v20)) break;
        sub_97c0(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      sub_97c0(v6);
      if ((((v13) && (!v14)) || ((!v13 || (!iswalnum(v14))))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = 0;
      sub_97c0(v6);
      if ((v13) && (!v14)) break;
      a0 = &v5[v12];
      v1 = *a0;
    }
    else {
      v8 = (unsigned char *)&v5[strlen(v3)];
      if (((v5 <= a0) || (v4 = __ctype_b_loc(), !(*(unsigned char *)(*v4 + (unsigned long)(unsigned char)v5[-1] * 2) & 8))) && ((v2 = *v8, !v2 || (!(*(unsigned char *)(*(long *)__ctype_b_loc() + (unsigned long)v2 * 2) & 8))))) {
        v8 = (unsigned char *)0x1;
        free(v3);
        return (unsigned long)v8 & 0xffffffff;
      }
      if (!*v5) break;
      v1 = v5[1];
      a0 = &v5[1];
    }
  }
  v8 = NULL;
  free(v3);
  return (unsigned long)v8 & 0xffffffff;
}

// Function: sub_65f0 @ 0x65f0
char * sub_65f0(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_6380(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_8180(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_6680 @ 0x6680
char * sub_6680(char *a0,char *a1)
{
  char v1; // al
  char *v10;
  char *v11; // stack - 0x48
  int v2; // eax
  char *v3; // rax
  char *v4;
  unsigned long v5; // rax
  char *v6;
  char *v7; // rax
  char *v8; // rax
  unsigned long *v9; // rbx
  
  v3 = dcgettext(NULL,a0,5);
  v4 = (char *)sub_8960();
  v6 = v3;
  if (sub_8890(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_8740(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_8180(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)&v8[v5];
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_8740(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_6883;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_66ea;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_6883:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_66ea;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_6380(v3,a0)) {
      a0 = v7;
label_670f:
      if (((!v11) || (!sub_6380(v3,v11))) && ((!a1 || (!sub_6380(v3,a1))))) goto label_678c;
      if (!v7) goto label_673d;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_66ea:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_6380(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_673d;
      goto label_670f;
    }
    if (!v2)
      return a0;
    if (sub_6380(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_678c:
    v5 = strlen(v3);
    v6 = (char *)sub_8180(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_673d:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_6980 @ 0x6980
void sub_6980(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_7720(a0,v1) <= -1)
    return;
  dat_23168 = v2 + -1;
}

// Function: sub_69d0 @ 0x69d0
void sub_69d0(unsigned long a0,void *a1,void *a2)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  int v4;
  int v5; // eax
  int *v6; // rax
  unsigned long v7; // stack - 0x68
  long v8; // stack - 0x60
  void *v9; // stack - 0x58
  
  v1 = *(unsigned long *)((long)a1 + 0x10);
  if (dat_23148) { // branch-flip
    v4 = (*dat_23148)(v1,0);
    if (v4)
      return;
    if (dat_23160) goto label_6a1e;
  }
  else {
    if (!dat_23160) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_6a1e:
    v3 = dat_23168;
    v2 = *(unsigned long *)((long)a2 + 0xa0);
    if (dat_23168) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_7720(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_6b05;
        }
        else if ((*v9)(v1,&v7)) {
label_6b05:
          (*dat_23160)(dat_23158 <= v2 && v2 <= dat_23158 + dat_23150,a2);
        }
      }
    }
  }
  if ((dat_23148) && (v4 = (*dat_23148)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_6b60 @ 0x6b60
void sub_6b60(void)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  int v3; // stack - 0x20
  
  v1 = sub_69d0;
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
  v3 = (-(unsigned int)(dat_23160 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,(sigaction *)&v1,NULL);
}

// Function: sub_6ce0 @ 0x6ce0
unsigned long sub_6ce0(unsigned long a0)
{
  dat_23148 = a0;
  sub_6b60();
  return 0;
}

// Function: sub_6d00 @ 0x6d00
void sub_6d00(void)
{
  dat_23148 = 0;
  if (dat_23160)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_6d30 @ 0x6d30
unsigned long sub_6d30(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_6d60 @ 0x6d60
unsigned long sub_6d60(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  
  if ((!dat_23168) && (sub_6980(&v1), !dat_23168))
    return 0xffffffff;
  v2 = 0;
  dat_23150 = a2;
  dat_23158 = a1;
  dat_23160 = a0;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_6b60();
  return 0;
}

// Function: sub_6e10 @ 0x6e10
void sub_6e10(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_23160 = 0;
  if (dat_23148) // branch-flip
    sub_6b60();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_6ea0 @ 0x6ea0
bool sub_6ea0(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_6ec0 @ 0x6ec0
unsigned long sub_6ec0(struct_1 *a0)
{
  if (a0->field_0x0 != a0->field_0x8)
    return (unsigned long)*(unsigned char *)(a0->field_0x18 + a0->field_0x0);
  a0->field_0x10 = 1;
  return 0xffffffff;
}

// Function: sub_6ee0 @ 0x6ee0
void sub_6ee0(struct_1 *a0) // return-dupe
{
  int v1; // eax
  
  v1 = sub_6ec0(a0);
  if (v1 <= -1)
    return;
  a0->field_0x0 = a0->field_0x0 + 1;
}

// Function: sub_6ef0 @ 0x6ef0
unsigned long sub_6ef0(unsigned long a0,long *a1)
{
  int v1; // eax
  long v2; // rcx
  int v3; // esi
  
  v2 = 0;
  v3 = 0;
  do {
    v1 = sub_6ec0();
    if (10 <= (unsigned int)(v1 - 0x30U)) { // branch-flip
      if (6 <= (unsigned int)(v1 - 0x41U)) { // branch-flip
        if (6 <= (unsigned int)(v1 - 0x61U)) {
          if (v3) {
            *a1 = v2;
            return 0;
          }
          return 0xffffffff;
        }
        v2 = v2 * 0x10 + (long)(v1 + -0x57);
      }
      else {
        v2 = v2 * 0x10 + (long)(v1 + -0x37);
      }
    }
    else {
      v2 = v2 * 0x10 + (long)(v1 - 0x30U);
    }
    sub_6ee0();
    v3 += 1;
  } while( true );
}

// Function: sub_6f60 @ 0x6f60
unsigned long sub_6f60(struct_2 *a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  
  if ((a1 <= a0->field_0x0) && (a0->field_0x0 <= a2 - 1)) {
    v1 = a0->field_0x8;
    v1[1] = a2;
    v2 = a0->field_0x10;
    *v1 = a1;
    v1[3] = v2;
    a0->field_0x18 = 0;
    return 1;
  }
  a0->field_0x10 = a2;
  return 0;
}

// Function: sub_6fa0 @ 0x6fa0
void sub_6fa0(void)
{
  dat_23170 = (long)getpagesize();
}

// Function: sub_6fc0 @ 0x6fc0
void sub_6fc0(void *a0)
{
  if (*(void **)((long)a0 + 0x20)) {
    munmap(*(void **)((long)a0 + 0x20),*(unsigned long *)((long)a0 + 0x28)); // tail-call
    return;
  }
}

// Function: sub_6fe0 @ 0x6fe0
unsigned int sub_6fe0(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_23170,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_7030 @ 0x7030
unsigned long sub_7030(unsigned long a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  void *v5;
  unsigned long v6; // r14
  unsigned long v7;
  
  v1 = dat_23170;
  v2 = (a0 / dat_23170) * dat_23170;
  v3 = ((a1 / dat_23170 + 1) * dat_23170 - v2) / dat_23170;
  if (v3 <= 1)
    return 1;
  v4 = 1;
  do {
    v4 *= 2;
  } while (v4 < v3);
  do {
    v6 = v4 >> 1;
    v5 = (void *)(v2 + v1 * v6);
    if (v6 < v3) {
      v7 = v6;
      do {
        if (sub_6fe0(v5))
          return 0;
        v7 += v4 & 0xfffffffffffffffe;
        v5 = (void *)((long)v5 + v1 * v6 * 2);
      } while (v7 < v3);
    }
    v4 = v6;
  } while (2 <= v6);
  return 1;
}

// Function: sub_7120 @ 0x7120
unsigned long sub_7120(unsigned long a0,long *a1)
{
  long v1;
  unsigned long v2; // rdi
  
  v1 = *a1;
  v2 = a0 * 2 - v1;
  if (a0 < v2)
    return 0;
  return sub_7030(v2,v1 - 1); // tail-call
}

// Function: sub_7150 @ 0x7150
void * sub_7150(void *a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  unsigned long v4; // rsi
  unsigned long v5; // rsi
  void *v6; // r14
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_23170;
    if (!a0)
      return NULL;
    if ((unsigned long)a0 / dat_23170 < v2)
      v2 = (unsigned long)a0 / dat_23170;
    v4 = dat_23170 * v2;
    v6 = (void *)((long)a0 - v4);
    if (mincore(v6,v4,v3) < 0) break;
    a0 = v6;
  }
  do {
    if (v2 == 1)
      return a0;
    while( true ) {
      v4 = v2 + 1 >> 1;
      v5 = v1 * v4;
      v6 = (void *)((long)a0 - v5);
      if (mincore(v6,v5,v3) < 0) break;
      v2 >>= 1;
      a0 = v6;
      if (v2 == 1)
        return v6;
    }
    v2 = v4;
  } while( true );
}

// Function: sub_7230 @ 0x7230
void * sub_7230(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  void *v5; // rdi
  
  v1 = dat_23170;
  v5 = (void *)(a0 + dat_23170);
  if (!v5)
    return NULL;
  v2 = 0x400;
  while( true ) {
    if ((unsigned long)-(long)v5 / v1 < v2)
      v2 = (unsigned long)-(long)v5 / v1;
    if (mincore(v5,v1 * v2,v4) <= -1) break;
    v5 = (void *)((long)v5 + v1 * v2);
    if (!v5)
      return NULL;
  }
  do {
    v3 = v2;
    if (v2 == 1)
      return v5;
    while( true ) {
      v2 = v3 + 1 >> 1;
      if (mincore(v5,v1 * v2,v4) < 0) break;
      v3 >>= 1;
      v5 = (void *)((long)v5 + v1 * v2);
      if (v3 == 1)
        return v5;
    }
  } while( true );
}

// Function: sub_7330 @ 0x7330
unsigned long sub_7330(unsigned long a0,unsigned long *a1)
{
  long v1; // rbp
  
  if (!dat_23170)
    sub_6fa0();
  v1 = (a0 / dat_23170) * dat_23170;
  *a1 = sub_7150(v1);
  a1[1] = sub_7230(v1);
  a1[2] = sub_7120;
  return 0;
}

// Function: sub_73a0 @ 0x73a0
unsigned long sub_73a0(unsigned long *a0) // return-dupe x2
{
  void *v1;
  int v2; // eax
  int v3; // eax
  long v4;
  unsigned long v5; // rax
  char *v6; // r12
  char *v7;
  
  v2 = open("/proc/self/maps",0);
  if (v2 <= -1)
    return 0xffffffff;
  *a0 = 0;
  v7 = (char *)0x1;
  *(unsigned int *)&a0[2] = 0;
  v6 = NULL;
  a0[3] = &a0[8];
  a0[4] = 0;
  a0[6] = 0;
  a0[7] = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = (void *)a0[3];
      v3 = (int)read(v2,v1,(unsigned long)v7);
      if (0 <= v3) {
        if (!v3) goto label_756c;
        if (v7 < (char *)(long)(v3 + 0x1049)) goto label_743f;
        v4 = (long)v3;
        while( true ) {
          a0[1] = v4;
          v5 = read(v2,(void *)(v4 + a0[3]),(long)v7 - v4);
          v3 = (int)v5;
          if (v3 <= -1) break;
          if ((unsigned long)((long)v7 - a0[1]) < (unsigned long)(long)(v3 + 0x1049)) goto label_743f;
          if (!v3) {
            close(v2);
            return v5 & 0xffffffff;
          }
          v4 = (long)v3 + a0[1];
        }
      }
      if (*__errno_location() != 4) {
label_756c:
        close(v2);
        v1 = (void *)a0[4];
        goto label_757a;
      }
    }
    else {
label_743f:
      if (v6) { // branch-flip
        v7 = (char *)((long)v7 * 2);
        if (!v7) goto label_756c;
        if ((void *)a0[4])
          munmap((void *)a0[4],a0[5]);
      }
      else {
        v6 = (char *)(long)getpagesize();
        for (v7 = v6; v7 <= (char *)0x1049; v7 = (char *)((long)v7 * 2)) {
        }
      }
      v4 = mmap(0,v7,3,0x22,0xffffffff,0);
      a0[4] = v4;
      if (v4 == -1) {
        close(v2);
        return 0xffffffff;
      }
      a0[5] = v7;
      a0[6] = v4;
      a0[7] = &v7[v4];
      a0[3] = v4;
    }
    if ((long)lseek(v2,0,0) <= -1) {
      close(v2);
      v2 = open("/proc/self/maps",0);
      if (v2 <= -1) {
        v1 = (void *)a0[4];
label_757a:
        if (!v1)
          return 0xffffffff;
        munmap(v1,a0[5]);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_75e0 @ 0x75e0
unsigned long sub_75e0(struct_2 *a0) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0x48
  unsigned long v11; // stack - 0x40
  int v2; // eax
  unsigned long v3; // stack - 0x88
  char v4 [48];
  long v5; // stack - 0x80
  long v6;
  unsigned long v7;
  unsigned long *v8;
  long *v9;
  
  v8 = &v3;
  if ((int)sub_73a0(v4) <= -1)
    return 0xffffffff;
  v9 = &v5;
  v7 = v10;
label_7630:
  do {
    v2 = sub_6ef0(v4,v8);
    if (((v2 < 0) || (sub_6ee0() != 0x2d)) || (v2 = sub_6ef0(v9), v2 < 0)) {
      sub_6fc0(v4); // return-dupe
      return 0;
    }
    do {
      v2 = sub_6ee0(v4);
      if (v2 == -1) break;
    } while (v2 != 10);
    if (v3 <= v7) { // branch-flip
      v1 = v5 - 1;
      if (v1 < v11 - 1) goto label_76d0;
      v6 = v5;
      if ((v3 < v7) && (v2 = sub_6f60(a0,v3,v7), v2)) {
        sub_6fc0(v4);
        return 0;
      }
      if (v1 <= v11 - 1) goto label_7630;
      v2 = sub_6f60(a0,v11,v6);
    }
    else {
label_76d0:
      v2 = sub_6f60(a0,v3,v5);
    }
    if (v2) {
      sub_6fc0(v4);
      return 0;
    }
  } while( true );
}

// Function: sub_7710 @ 0x7710
unsigned long sub_7710(struct_2 *a0)
{
  return sub_75e0(a0); // tail-call
}

// Function: sub_7720 @ 0x7720
unsigned long sub_7720(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long *v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  int v4; // stack - 0x30
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_7710(&v1);
  if (!v4) {
    a1[2] = sub_6ea0;
    return 0;
  }
  return sub_7330(a0,a1); // tail-call
}

// Function: sub_77c0 @ 0x77c0
unsigned char * sub_77c0(char *a0,int a1)
{
  unsigned char v1;
  char v10; // stack - 0x70
  unsigned long v11; // stack - 0x6c
  char v12; // stack - 0x64
  unsigned char *v13; // stack - 0x60
  long v14; // stack - 0x58
  char v15; // stack - 0x50
  unsigned int v16; // stack - 0x4c
  long v2;
  unsigned char *v3; // rax
  unsigned long v4; // rax
  long *v5; // rax
  unsigned char *v6;
  unsigned char *v7; // stack - 0x78
  int v8; // ebp
  unsigned char *v9;
  
  v3 = (unsigned char *)strdup(a0);
  if (!v3)
    sub_8530(); // no-return
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    if (a1) {
      v13 = v3;
      v4 = strlen((char *)v3);
      v10 = 0;
      v7 = &v3[v4];
      v12 = 0;
      v11 = 0;
      v6 = v3;
      if (v3 < v7) {
        do {
          v12 = 0;
          sub_89a0(&v7);
          if ((!v15) || (!iswspace(v16))) {
            v6 = v13;
            break;
          }
          v13 = &v13[v14];
          v12 = 0;
          v6 = v13;
        } while (v13 < v7);
      }
      memmove(v3,v6,strlen((char *)v6) + 1);
      if (a1 == 1)
        return v3;
    }
    v13 = v3;
    v8 = 0;
    v4 = strlen((char *)v3);
    v10 = 0;
    v7 = &v3[v4];
    v11 = 0;
    if (v3 < v7) {
      do {
        v12 = 0;
        sub_89a0(&v7);
        if (v8) { // branch-flip
          if (v8 != 1) { // branch-flip
            if ((v8 != 2) || (!v15)) goto label_784b;
            if (!iswspace(v16))
              v8 = 1;
          }
          else if ((((v15) && (iswspace(v16))) && (v15)) && (iswspace(v16))) {
            v8 = 2;
            v9 = v13;
          }
          else {
label_798f:
          }
        }
        else {
          if (v15) {
            if (iswspace(v16)) goto label_798f;
            if (v15) {
              v8 = 1;
              iswspace(v16);
              goto label_7855;
            }
          }
label_784b:
          v8 = 1;
        }
label_7855:
        v13 = &v13[v14];
      } while (v13 < v7);
      if (v8 == 2)
        *v9 = 0;
    }
  }
  else {
    if (a1) {
      v1 = *v3;
      v6 = v3;
      if (v1) {
        v5 = __ctype_b_loc();
        do {
          if (!(*(unsigned char *)(*v5 + 1 + (unsigned long)v1 * 2) & 0x20)) break;
          v1 = v6[1];
          v6 = &v6[1];
        } while (v1);
      }
      memmove(v3,v6,strlen((char *)v6) + 1);
      if (a1 == 1)
        return v3;
    }
    v6 = &v3[strlen((char *)v3) - 1];
    if (v3 <= v6) {
      v2 = *(long *)__ctype_b_loc();
      do {
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)*v6 * 2) & 0x20))
          return v3;
        *v6 = 0;
        v6 = &v6[-1];
      } while (v3 <= v6);
    }
  }
  return v3;
}

// Function: sub_7ad0 @ 0x7ad0
long sub_7ad0(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
  __fprintf_chk(a0,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7e5);
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
label_7bfd:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_7c36:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xe1b8) + 0xe1b8;
  switch(a5) {
    case 0:
      goto label_7c36;
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
      goto label_7bfd;
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

// Function: sub_7f50 @ 0x7f50
void sub_7f50(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_7ad0(); // tail-call
}

// Function: sub_7f70 @ 0x7f70
void sub_7f70(FILE *a0,char *a1,char *a2,char *a3,struct_8 *a4)
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
  sub_7ad0(a0,a1,a2,a3);
}

// Function: sub_8010 @ 0x8010
void sub_8010(void)
{
  sub_7f70();
}

// Function: sub_80c0 @ 0x80c0
void sub_80c0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_8170 @ 0x8170
long sub_8170(long a0)
{
  if (a0)
    return a0;
  sub_8530(); // no-return
}

// Function: sub_8180 @ 0x8180
void sub_8180(unsigned long a0)
{
  sub_8170(malloc(a0)); // tail-call
}

// Function: sub_81a0 @ 0x81a0
void sub_81a0(void)
{
  sub_8180(); // tail-call
}

// Function: sub_81b0 @ 0x81b0
void sub_81b0(void)
{
  sub_8170(sub_88f0()); // tail-call
}

// Function: sub_81d0 @ 0x81d0
void sub_81d0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_8530(); // no-return
}

// Function: sub_8210 @ 0x8210
void sub_8210(void)
{
  sub_8170(sub_8900()); // tail-call
}

// Function: sub_8230 @ 0x8230
void sub_8230(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_8530(); // no-return
}

// Function: sub_8270 @ 0x8270
void sub_8270(unsigned long a0,unsigned long a1)
{
  sub_8230(NULL,a0,a1); // tail-call
}

// Function: sub_8280 @ 0x8280
void sub_8280(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8230(a0,a1,a2); // tail-call
}

// Function: sub_8290 @ 0x8290
void sub_8290(void)
{
  sub_8170(sub_8930()); // tail-call
}

// Function: sub_82b0 @ 0x82b0
void sub_82b0(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_8530(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_8230(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_8320 @ 0x8320
void sub_8320(void *a0,unsigned long *a1)
{
  sub_82b0(a0,a1,1); // tail-call
}

// Function: sub_8330 @ 0x8330
void sub_8330(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_83d6;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_83d6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_8530(a0,v3); // no-return
  sub_81d0(a0,v3);
  *a1 = v2;
}

// Function: sub_8410 @ 0x8410
void sub_8410(unsigned long a0,unsigned long a1)
{
  sub_8170(calloc(a0,a1)); // tail-call
}

// Function: sub_8430 @ 0x8430
void sub_8430(unsigned long a0)
{
  sub_8410(a0,1); // tail-call
}

// Function: sub_8440 @ 0x8440
void sub_8440(void)
{
  sub_8170(sub_8920()); // tail-call
}

// Function: sub_8460 @ 0x8460
void sub_8460(unsigned long a0)
{
  sub_8440(a0,1); // tail-call
}

// Function: sub_8470 @ 0x8470
void sub_8470(void *a0,unsigned long a1)
{
  memcpy((void *)sub_8180(a1),a0,a1); // tail-call
}

// Function: sub_84a0 @ 0x84a0
void sub_84a0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_81b0(a1),a0,a1); // tail-call
}

// Function: sub_84d0 @ 0x84d0
void sub_84d0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_81b0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_8510 @ 0x8510
void sub_8510(char *a0)
{
  sub_8470(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_8530 @ 0x8530
void sub_8530(void)
{
  error(dat_13020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_8570 @ 0x8570
void sub_8570(char *a0,char *a1,FILE *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  if (freopen(a0,a1,a2))
    return;
  if (!a0) {
    if (stdin != a2) { // branch-flip
      if (stdout != a2) { // branch-flip
        if (stderr != a2) // branch-flip
          a0 = dcgettext(NULL,"unknown stream",5);
        else {
          a0 = dcgettext(NULL,"stderr",5);
        }
      }
      else {
        a0 = dcgettext(NULL,"stdout",5);
      }
    }
    else {
      a0 = dcgettext(NULL,"stdin",5);
    }
  }
  v1 = sub_be10(1,a1);
  v2 = sub_be10(0,a0);
  v3 = dcgettext(NULL,"failed to reopen %s with mode %s",5);
  error(dat_13020,*__errno_location(),v3,v2,v1); // tail-call
}

// Function: sub_86a0 @ 0x86a0
void sub_86a0(void)
{
  int v1; // eax
  
  v1 = sub_be30();
  if (!v1)
    return;
  error(dat_13020,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_86e0 @ 0x86e0
int sub_86e0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_bee0(a0,a1,a2,a3,a4);
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_8530(); // no-return
  }
  return v1;
}

// Function: sub_8710 @ 0x8710
long sub_8710(void)
{
  long v1; // rax
  
  v1 = sub_c160();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_8530(); // no-return
  }
  return v1;
}

// Function: sub_8740 @ 0x8740
long sub_8740(char *a0,char *a1,char *a2)
{
  long v1; // rax
  
  v1 = sub_c3b0(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_8530(); // no-return
  }
  return v1;
}

// Function: sub_8770 @ 0x8770
long sub_8770(int a0,void *a1,long a2)
{
  void *v1;
  unsigned long v2;
  long v3; // rax
  unsigned long v4; // r14
  void *v5;
  
  v4 = 0x7fffffffffffffff;
  v1 = (void *)((long)a1 + a2);
  v5 = a1;
  do {
    while( true ) {
      v2 = (long)v1 - (long)v5;
      if (v4 < (unsigned long)((long)v1 - (long)v5))
        v2 = v4;
      v3 = read(a0,v5,v2);
      if (0 < v3) break;
      if (!v3) {
        return (long)v5 - (long)a1;
      }
      if ((*__errno_location() != 0x16) || (v2 < 0x80000000))
        return -1;
      v4 = 0x7fffffff;
      if (v1 <= v5) {
        return (long)v5 - (long)a1;
      }
    }
    v5 = (void *)((long)v5 + v3);
  } while (v5 < v1);
  return (long)v5 - (long)a1;
}

// Function: sub_8820 @ 0x8820
unsigned long sub_8820(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  
  if (a0) {
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
        a0 = v3;
    }
    return a0;
  }
  v3 = 0x2000;
  if (a1)
    v3 = a1;
  return v3;
}

// Function: sub_8890 @ 0x8890
int sub_8890(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_c910((unsigned char)*a0);
    v1 = sub_c910((unsigned char)*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_88f0 @ 0x88f0
void sub_88f0(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_8900 @ 0x8900
void sub_8900(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_8920 @ 0x8920
void sub_8920(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_8930 @ 0x8930
void sub_8930(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_8960 @ 0x8960
char * sub_8960(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_89a0 @ 0x89a0
void sub_89a0(struct_4 *a0)
{
  char v1;
  char *v2;
  long v3; // rax
  
  if (a0->field_0x14)
    return;
  v2 = a0->field_0x18;
  if (!a0->field_0x8) {
    v1 = *v2;
    if (sub_caf0((int)v1)) {
      a0->field_0x20 = 1;
      v1 = *v2;
      a0->field_0x28 = 1;
      a0->field_0x2c = (int)v1;
      goto label_89e3;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x9[3]))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    a0->field_0x8 = 1;
  }
  v3 = sub_c6f0(&a0->field_0x2c,v2,a0->field_0x0 - (long)v2,(mbstate_t *)&a0->field_0x9[3]);
  a0->field_0x20 = v3;
  if (v3 == -1) {
    a0->field_0x20 = 1;
    a0->field_0x28 = 0;
    a0->field_0x14 = 1;
    return;
  }
  if (v3 != -2) { // branch-flip
    if (!v3) {
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
    a0->field_0x28 = 0;
    a0->field_0x20 = a0->field_0x0 - (long)a0->field_0x18;
  }
label_89e3:
  a0->field_0x14 = 1;
  return;
}

// Function: sub_8b10 @ 0x8b10
void sub_8b10(struct_0 *a0,long a1)
{
  a0->field_0x18 = a0->field_0x18 + a1;
  a0->field_0x0 = a0->field_0x0 + a1;
}

// Function: sub_8b20 @ 0x8b20
void sub_8b20(struct_9 *a0,struct_9 *a1)
{
  char v1;
  
  a0->field_0x0 = a1->field_0x0;
  v1 = a1->field_0x8;
  a0->field_0x8 = v1;
  if (v1) // branch-flip
    *(unsigned long *)&a0->field_0x9[3] = *(unsigned long *)&a1->field_0x9[3];
  else {
    *(unsigned long *)&a0->field_0x9[3] = 0;
  }
  a0->field_0x14 = a1->field_0x14;
  sub_ca90((struct_6 *)&a0[1],(struct_7 *)&a1[1]); // tail-call
}

// Function: sub_8b70 @ 0x8b70
unsigned long sub_8b70(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_c9e0(v9);
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
              goto label_8c68;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_8c68:
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
      *(unsigned long *)&v8[-8] = 0x8cbe;
      sub_ca30(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_8d10 @ 0x8d10
char sub_8d10(void *a0,void *a1,long *a2)
{
  char v1;
  char v10 [4]; // stack - 0x88
  char v11 [4]; // stack - 0xc8
  void **v12;
  char *v13; // rsp
  void **v14; // rsp
  void **v15;
  unsigned long v16;
  long v17;
  void *v18;
  unsigned long *v19;
  void *v2;
  long *v20; // stack - 0xf0
  char v21; // stack - 0xe1
  void *v22; // stack - 0xe0
  void *v23; // stack - 0xd8
  char *v24; // stack - 0xd0
  unsigned long v25; // stack - 0xc4
  char v26; // stack - 0xbc
  void *v27; // stack - 0xb8
  long v28; // stack - 0xb0
  char v29; // stack - 0xa8
  int v3; // eax
  int v30; // stack - 0xa4
  unsigned long v31; // stack - 0x84
  char v32; // stack - 0x7c
  unsigned long v33; // stack - 0x70
  char v34; // stack - 0x68
  int v35; // stack - 0x64
  unsigned long *v4; // rax
  unsigned long *v5;
  void *v6;
  unsigned long *v7; // rbx
  void *v8; // stack - 0xf8
  void *v9; // stack - 0x78
  
  v14 = &v8;
  v12 = &v8;
  v15 = &v8;
  v8 = a0;
  v20 = a2;
  v22 = (void *)sub_cb10(a1);
  if ((0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0)) && (!SUB168(ZEXT816(0x38) * ZEXT816(v22),8))) {
    v16 = (long)v22 * 0x38;
    if (0xfa1 <= v16) { // branch-flip
      v19 = (unsigned long *)sub_c9e0(v16);
      v2 = v22;
    }
    else {
      v15 = &v8;
      while (v14 != (void **)((long)&v8 - (v16 + 0x36 & 0xfffffffffffff000))) {
        v12 = (char *)((long)v15 + -0x1000);
        v13 = (char *)((long)v15 + -0x1000);
        *(unsigned long *)((long)v15 + -8) = *(unsigned long *)((long)v15 + -8);
        v14 = (char *)((long)v15 + -0x1000);
        v15 = (void **)v13;
      }
      v16 = (unsigned long)((unsigned int)(v16 + 0x36) & 0xff0);
      v17 = -v16;
      v15 = (char *)((long)v12 + v17);
      if (v16)
        *(unsigned long *)((long)v12 + -8) = *(unsigned long *)((long)v12 + -8);
      v19 = (unsigned long *)(((unsigned long)((long)v12 + v17 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v2 = v22;
    }
    if (v19) {
      v10[0] = 0;
      v24 = v10;
      v31 = 0;
      v5 = v19;
      v22 = v2;
      v9 = a1;
      while( true ) {
        v32 = 0;
        *(unsigned long *)((long)v15 + -8) = 0x8e8f;
        sub_97c0(v24);
        v21 = v34;
        if ((v34) && (!v35)) break;
        v7 = &v5[6];
        *(unsigned long *)((long)v15 + -8) = 0x8e77;
        sub_ca90(v5,&v9);
        v9 = (void *)((long)v9 + v33);
        v5 = v7;
      }
      v17 = 0;
      v19[(long)v2 * 6 + 1] = 1;
      v23 = (void *)0x2;
      v5 = v19;
      if ((void *)0x3 <= v22) {
label_8ed0:
        v7 = &v5[6];
        v1 = *(char *)&v5[8];
        do {
          v4 = &v19[v17 * 6];
          if ((v1) && (*(char *)&v4[2])) {
            if (*(int *)((long)v5 + 0x44) == *(int *)((long)v4 + 0x14)) {
label_8f25:
              v17 += 1;
              v19[(long)((long)v2 * 6 + (long)v23)] = (long)v23 - v17;
              goto label_8f3a;
            }
          }
          else {
            v16 = v5[7];
            if (v16 == v4[1]) {
              v18 = (void *)*v4;
              v6 = (void *)*v7;
              *(unsigned long *)((long)v15 + -8) = 0x8f21;
              if (!memcmp(v6,v18,v16)) goto label_8f25;
            }
          }
          if (!v17) goto label_9100;
          v17 -= v19[(long)v2 * 6 + v17];
        } while( true );
      }
label_8f5a:
      v10[0] = 0;
      v18 = NULL;
      v11[0] = 0;
      *v20 = 0;
      v25 = 0;
      v27 = v8;
      v26 = 0;
      v9 = v8;
      v31 = 0;
      v32 = 0;
      do {
        while( true ) {
          *(unsigned long *)((long)v15 + -8) = 0x8fbc;
          sub_97c0(v24);
          v16 = v33;
          if (!v34) break;
          if (!v35) goto label_8df7;
          v5 = &v19[(long)v18 * 6];
          if (!*(char *)&v5[2]) goto label_902b;
          if (*(int *)((long)v5 + 0x14) == v35) {
            v6 = v9;
            goto label_8fe7;
          }
label_90ab:
          if (v18) { // branch-flip
            v17 = v19[(long)((long)v2 * 6 + (long)v18)];
            v18 = (void *)((long)v18 - v17);
            for (; v17; v17 = v17 + -1) {
              *(unsigned long *)((long)v15 + -8) = 0x90e7;
              sub_97c0(v11);
              if ((v29) && (!v30)) goto label_90fa;
              v26 = 0;
              v27 = (void *)((long)v27 + v28);
            }
          }
          else {
            *(unsigned long *)((long)v15 + -8) = 0x9042;
            sub_97c0(v11);
            if ((v29) && (!v30)) {
label_90fa:
              *(unsigned long *)((long)v15 + -8) = 0x90ff; // no-return
              abort();
            }
            v26 = 0;
            v27 = (void *)((long)v27 + v28);
            v32 = 0;
            v9 = (void *)((long)v9 + v33);
          }
        }
        v5 = &v19[(long)v18 * 6];
label_902b:
        if (v5[1] != v33) goto label_90ab;
        v6 = (void *)*v5;
        v23 = v9;
        *(unsigned long *)((long)v15 + -8) = 0x9099;
        v3 = memcmp(v6,v9,v33);
        v6 = v23;
        if (v3) goto label_90ab;
label_8fe7:
        v9 = (void *)((long)v6 + v16);
        v32 = 0;
        v18 = (void *)((long)v18 + 1);
      } while (v22 != v18);
      *v20 = (long)v27;
label_8df7:
      *(unsigned long *)((long)v15 + -8) = 0x9018;
      sub_ca30(v19);
      return v21;
    }
  }
  return '\0';
label_9100:
  v19[(long)((long)v2 * 6 + (long)v23)] = v23;
label_8f3a:
  v23 = (void *)((long)v23 + 1);
  v5 = v7;
  if (v22 == v23) goto label_8f5a;
  goto label_8ed0;
}

// Function: sub_9120 @ 0x9120
char * sub_9120(char *a0,char *a1) // early-return, return-dupe x4
{
  char *v1;
  char *v10; // stack - 0x190
  char v11 [4]; // stack - 0x188
  char v12 [4]; // stack - 0x88
  unsigned long v13; // stack - 0xc8
  unsigned int v14; // stack - 0x108
  char v15 [4]; // stack - 0x148
  long v16;
  unsigned long v17;
  unsigned long v18; // stack - 0x1d8
  unsigned char v19; // stack - 0x1c9
  char v2;
  unsigned long v20; // stack - 0x1a8
  unsigned long v21; // stack - 0x184
  char v22; // stack - 0x17c
  char *v23; // stack - 0x178
  unsigned long v24; // stack - 0x170
  char v25; // stack - 0x168
  int v26; // stack - 0x164
  unsigned long v27; // stack - 0x144
  char v28; // stack - 0x13c
  char *v29; // stack - 0x138
  char v3;
  long v30; // stack - 0x130
  unsigned char v31; // stack - 0x128
  int v32; // stack - 0x124
  unsigned int v33; // stack - 0x104
  unsigned int v34; // stack - 0x100
  unsigned int v35; // stack - 0xfc
  char *v36; // stack - 0xf8
  unsigned long v37; // stack - 0xf0
  unsigned int v38; // stack - 0xe8
  int v39; // stack - 0xe4
  unsigned char v4; // al
  unsigned long v40; // stack - 0xe0
  unsigned long v41; // stack - 0xd8
  unsigned long v42; // stack - 0xd0
  unsigned long v43; // stack - 0xc0
  undefined3 v44; // stack - 0xbb
  char *v45; // stack - 0xb8
  unsigned long v46; // stack - 0xb0
  unsigned long v47; // stack - 0xa8
  unsigned long v48; // stack - 0xa0
  unsigned long v49; // stack - 0x98
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x90
  unsigned long v51; // stack - 0x84
  char v52; // stack - 0x7c
  char *v53; // stack - 0x78
  unsigned long v54; // stack - 0x70
  char v55; // stack - 0x68
  int v56; // stack - 0x64
  char *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9; // rdx
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v11[0] = 0;
    v21 = 0;
    v22 = 0;
    v23 = a1;
    sub_97c0(v11);
    if ((!v25) || (v26)) {
      v19 = 1;
      v15[0] = 0;
      v27 = 0;
      v28 = 0;
      v14 &= 0xffffff00;
      v34 = 0;
      v33 = 0;
      v18 = 0;
      v20 = 0;
      v8 = 0;
      v29 = a1;
      v36 = a0;
      do {
        v35 &= 0xffffff00;
        sub_97c0(&v14);
        if ((char)v38) { // branch-flip
          if (!v39)
            return NULL;
          v4 = 10 <= v18 & v19;
          if (v4) {
            if (v18 * 5 <= v8) { // branch-flip
label_96a0:
              v16 = v8 - v20;
              while ((v16 && ((sub_97c0(v15), !v31 || (v32))))) {
                v28 = 0;
                v29 = &v29[v30];
                v16 -= 1;
              }
              sub_97c0(v15);
              v19 = v31;
              if (v31) { // branch-flip
                if (!v32) {
                  v19 = sub_8d10(a0,a1,&v10);
                  if (v19)
                    return v10;
                }
              }
              else {
                v19 = 1;
              }
              v20 = v8;
              if (!(char)v38) goto label_9476;
            }
            else {
              v19 = v4;
            }
          }
          v9 = v8 + 1;
          if (!v25) goto label_9476;
          if (v39 == v26) goto label_94b6;
label_9408:
          v6 = v36;
          v17 = v37;
        }
        else {
          v4 = 10 <= v18 & v19;
          if (v4) { // branch-flip
            if (v18 * 5 <= v8) goto label_96a0;
            v19 = v4;
          }
          else {
          }
label_9476:
          v17 = v37;
          v6 = v36;
          v9 = v8 + 1;
          if (v37 == v24) {
            if (!memcmp(v36,v23,v37)) {
label_94b6:
              v9 = v8 + 1;
              v13 = CONCAT44(v33,v14);
              v12[0] = 0;
              v47 = CONCAT44(v39,v38);
              v52 = 0;
              v46 = v37;
              v45 = &v36[v37];
              v44 = (undefined3)(v35 >> 8);
              v43 = (undefined5)v34;
              v51 = 0;
              v48 = v40;
              v49 = v41;
              v50 = v42;
              v53 = a1;
              sub_97c0(v12);
              if ((v55) && (!v56))
                abort(); // no-return
              v53 = &v53[v54];
              do {
                v9 += 1;
                v52 = 0;
                sub_97c0(v12);
                if ((v55) && (!v56))
                  return v36;
                sub_97c0(&v13);
                v17 = v54;
                v1 = v53;
                v8 = v46;
                v6 = v45;
                if ((char)v47) { // branch-flip
                  if (!v47._4_4_)
                    return NULL;
                  if (!v55) goto label_9623;
                  if (v47._4_4_ != v56) goto label_9408;
                }
                else {
label_9623:
                  if ((v46 != v54) || (memcmp(v45,v53,v46))) goto label_9408;
                }
                v45 = &v6[v8];
                v43 = (undefined5)(unsigned int)v43;
                v53 = &v1[v17];
              } while( true );
            }
          }
        }
        v18 += 1;
        v36 = &v6[v17];
        v8 = v9;
      } while( true );
    }
  }
  else {
    v2 = *a1;
    if (v2) {
      v3 = *a0;
      if (!v3)
        return NULL;
      v17 = 0;
      v9 = 0;
      v8 = 0;
      v5 = 1;
      v6 = a1;
      do {
        v17 += 1;
        v7 = v9 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v16 = v7 - v9;
            if (!a0[v16])
              return NULL;
            v7 += 1;
            if (a0[v16] != v3) break;
            v3 = a1[v7 - v9];
            if (!v3)
              return a0;
          }
        }
        v9 = v7;
        if (!*v1)
          return NULL;
        v4 = 10 <= v17 & (unsigned char)v5;
        if (v4) {
          if (v17 * 5 <= v9) { // branch-flip
            if (v6) {
              v6 = &v6[strnlen(v6,v9 - v8)];
              v8 = v9;
              if (*v6) goto label_924d;
            }
            v5 = sub_8b70(v1,a1,strlen(a1),&v10);
            if ((char)v5)
              return v10;
            v6 = NULL;
          }
          else {
label_924d:
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

// Function: sub_97c0 @ 0x97c0
void sub_97c0(struct_5 *a0)
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
    if (sub_caf0((int)v1)) {
      a0->field_0x18 = 1;
      v1 = *v2;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)v1;
      goto label_9802;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_c6f0(&a0->field_0x24,v2,sub_c470(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
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
        __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
      if (a0->field_0x24)
        __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
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
label_9802:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_9940 @ 0x9940
void sub_9940(void *a0,long a1)
{
  *(long *)((long)a0 + 0x10) = *(long *)((long)a0 + 0x10) + a1;
}

// Function: sub_9950 @ 0x9950
void sub_9950(struct_10 *a0,struct_10 *a1)
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
  sub_ca90((struct_6 *)&a0[1].field_0x1[2],(struct_7 *)&a1[1].field_0x1[2]); // tail-call
}

// Function: sub_9990 @ 0x9990
struct_11 * sub_9990(struct_11 *a0,int a1)
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

// Function: sub_99e0 @ 0x99e0
bool sub_99e0(long a0,long a1)
{
  return sub_8890((char *)(a0 + 9),(char *)(a1 + 9)) == 0;
}

// Function: sub_9a00 @ 0x9a00
unsigned long sub_9a00(long a0,long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_99e0(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9a60 @ 0x9a60
unsigned long sub_9a60(long a0,long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_9a00(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9ae0 @ 0x9ae0
unsigned long sub_9ae0(long a0,long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_9a60(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9b50 @ 0x9b50
unsigned long sub_9b50(long a0,long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_9ae0(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9bf0 @ 0x9bf0
unsigned long sub_9bf0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_9b50(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9c90 @ 0x9c90
unsigned long sub_9c90(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_9bf0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_9d50 @ 0x9d50
unsigned long sub_9d50(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_c8d0((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_9c90(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_9e10 @ 0x9e10
unsigned long sub_9e10(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_c8d0((int)(char)a2)) // branch-flip
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
      return sub_9d50(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_9ef0 @ 0x9ef0
unsigned long sub_9ef0(unsigned char *a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_c8d0((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_9e10(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_9fb0 @ 0x9fb0
char * sub_9fb0(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned char *v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = (unsigned char *)sub_8960();
    if (sub_9ef0(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_9ef0(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0xe317 : (char *)0xe31c; // branch-flip
    }
  }
  return v1;
}

// Function: sub_a0e0 @ 0xa0e0
long * sub_a0e0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_a150:
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
label_a5d6:
      v2 = 0;
      goto label_a590;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_a6f1;
      }
      v23 = 1;
      v2 = 0;
      v12 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v2 = 1;
label_a590:
      v23 = 1;
      v12 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_a6f1;
      }
      goto label_a5d6;
    case 5:
      if (!v17) goto label_a531;
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
        v9 = (char *)sub_9fb0("`",a4);
        v35 = (char *)v9;
        v9 = (char *)sub_9fb0("\'",a4);
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
label_a213:
  do {
label_a226:
    v22 = 0;
    a1 = v13;
label_a230:
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
          if ((a3 < v22 + v29) || (v8 = memcmp(v11,v30,v29), v8)) goto label_aba0;
          if (!v23) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v20 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_a79c;
                default:
                  goto label_a473;
                case 7:
                  goto label_a760;
                case 8:
                  goto label_a745;
                case 9:
                  goto label_a823;
                case 10:
                  goto label_a73b;
                case 0xb:
                  goto label_a850;
                case 0xc:
                  goto label_a787;
                case 0xd:
                  goto label_a68d;
                case 0x20:
                  goto label_a85d;
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
                  goto label_a426;
                case 0x23:
                  goto label_a800;
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
                  goto label_a358;
                case 0x27:
                  goto label_a672;
                case 0x3f:
                  goto label_a631;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_a473;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_a358;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_ac44;
              goto label_ab88;
            }
            if (v7 == 0x7d) goto label_ad10;
            if ((char)v7 <= '}') {
              v20 = 0x7b;
              if (v7 == 0x7b) goto label_ab6a;
              v20 = 0x7c;
              if (v7 == 0x7c) goto label_ab88;
              goto label_a473;
            }
            if (v7 != 0x7e) goto label_a473;
label_acc4:
            if (!v22) {
              v7 = a4 == 2;
              v20 = 0x7e;
              v25 = v3;
              goto label_a4e9;
            }
            v20 = 0x7e;
            goto label_acd2;
          }
          goto label_a45c;
        }
label_aba0:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v20 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v21 = 0;
                if (v7 == 0x7e) goto label_acc4;
                goto label_a470;
              }
              v20 = 0x7b;
              v24 = 0;
              if (v7 != 0x7b) {
                v20 = 0x7c;
                v24 = 0;
                v21 = 0;
                if (v7 == 0x7c) goto label_ab88;
                goto label_a473;
              }
            }
            else {
              v20 = 0x7d;
              v24 = 0;
            }
label_ab6a:
            if (a3 == 0xffffffffffffffff) goto label_ad1f;
label_ab78:
            v21 = v24;
            if (a3 == 1) goto label_a805;
          }
          else {
            if (v7 == 0x40) goto label_a470;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v21 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_a358;
            v24 = 0;
            v21 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_ac44:
              if ((char)v18 != '\\') goto label_a473;
              if (((bool)(v2 & v23)) && (v29)) goto label_ac6a;
              v16 = 0x5c;
              goto label_a692;
            }
          }
label_ab88:
          v7 = a4 == 2;
          v25 = v3;
          v24 = 0;
          goto label_a4e9;
        }
        switch(v7) {
          case 0:
            goto label_a78e;
          default:
label_a470:
            v24 = 0;
label_a473:
            v16 = (unsigned char)v18;
            v21 = v24;
            if (v32 != 1) goto label_a9f3;
label_a480:
            v9 = __ctype_b_loc();
            v20 = 1;
            v25 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v26 = (bool)(v25 & v2);
            v25 = !v25;
            v21 = v24;
            goto label_a4ce;
          case 7:
label_a760:
            v16 = 0x61;
            if (!v23) goto label_a758;
            goto label_a6bf;
          case 8:
label_a745:
            v16 = 0x62;
            goto label_a74a;
          case 9:
            v24 = 0;
label_a823:
            v20 = 9;
            v16 = 0x74;
            goto label_a830;
          case 10:
label_a73b:
            v16 = 0x6e;
            break;
          case 0xb:
label_a850:
            v16 = 0x76;
            break;
          case 0xc:
label_a787:
            v16 = 0x66;
label_a74a:
            if (v23) goto label_a6bf;
label_a758:
            v24 = 0;
            goto label_a3de;
          case 0xd:
label_a68d:
            v16 = 0x72;
            break;
          case 0x20:
            v21 = 0;
label_a85d:
            v20 = 0x20;
            goto label_a358;
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
            goto label_a429;
          case 0x23:
            v20 = 0x23;
            v24 = 0;
            goto label_a805;
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
label_a358:
            v7 = 0;
            goto label_a368;
          case 0x27:
            v21 = 0;
            goto label_a672;
          case 0x3f:
            v21 = 0;
            goto label_a631;
          
        }
        goto label_a692;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_ad10:
            v20 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_ab78;
label_ad1f:
            v21 = v24;
            if (a2[1]) goto label_ab88;
            goto label_a805;
          }
          if ((char)v16 <= '}') {
            v20 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_a426;
              goto label_a473;
            }
            goto label_ab6a;
          }
          v20 = 0x7e;
          if (v16 == 0x7e) goto label_a805;
          if (v32 == 1) goto label_a480;
label_a9f3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v20 = 0;
          do {
            v18 = v22 + v20;
            v9 = (long)sub_c6f0(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v24 = v25;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_af5b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_b2d5;
              goto label_b2df;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_ab2d;
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
          goto label_af5b;
        }
        if (v16 == 0x40) goto label_a473;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_a4e0;
        if ((unsigned long)v9 & 0xa4000000) goto label_a426;
        if (v16 != 0x5c) goto label_a473;
        if (a4 == 2) {
          if (!v23) goto label_ac6a;
          goto label_a453;
        }
        if (!(bool)(v2 & v23 & v29 != 0)) {
          v20 = 0x5c;
          v16 = 0x5c;
          v24 = 0;
          goto label_a830;
        }
label_ac6a:
        v22 += 1;
        v24 = 0;
        v16 = 0x5c;
        v7 = v19;
        goto label_ac80;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v21 = 0;
            v20 = 0;
            if (v33 & 1) goto label_a902;
            goto label_a83b;
          }
label_a78e:
          if (!v23) {
            v21 = 0;
label_a79c:
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
                goto label_a513;
              }
label_b0a5:
              a0[v13] = 0x5c;
              v12 = v13;
              v19 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v19;
              goto label_b0a5;
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
label_b0dc:
              v16 = (unsigned char)v20;
              v24 = 0;
              v12 = v13;
              v7 = v25;
              if (!v2 || v26) goto label_a513;
              goto label_a368;
            }
            v16 = 0x30;
            v24 = 0;
            v12 = v13;
            v7 = v25;
            goto label_a513;
          }
          if (a4 != 2) goto label_a45c;
          goto label_a6cb;
        default:
          goto label_a473;
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
          goto label_a8bf;
        case 10:
          v20 = 10;
          v16 = 0x6e;
          goto label_a8bf;
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
label_a8bf:
          v7 = a4 == 2 & v23;
          v24 = 0;
          if (!(bool)v7) goto label_a830;
label_a441:
          a4 = 2;
label_a453:
          if (!v27) goto label_a45c;
          goto label_a6cb;
        case 0x20:
          v18 = 0x20;
          goto label_a429;
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
label_a426:
          v25 = 0;
          goto label_a429;
        case 0x23:
label_a800:
          v20 = 0x23;
label_a805:
          v21 = v24;
          if (v22) {
label_acd2:
            v7 = a4 == 2;
            v25 = v3;
            v24 = 0;
            goto label_a4e9;
          }
          v18 = v20;
label_a429:
          v20 = v18;
          v21 = v24;
          v24 = v25;
          if ((a4 == 2) && (v23)) goto label_a441;
          goto label_a4e7;
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
          goto label_a4e0;
        case 0x27:
label_a672:
          v24 = v25;
          if (a4 != 2) {
            v7 = 0;
            v20 = 0x27;
            goto label_a4e9;
          }
          if (!v23) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v31) goto label_b0fb;
            }
            else {
label_b0fb:
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
            goto label_a513;
          }
          goto label_a453;
        case 0x3f:
label_a631:
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
                  goto label_a4e9;
                }
                if (v23) goto label_a45c;
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
                goto label_b0dc;
              }
            }
            v7 = 0;
            v20 = 0x3f;
            v25 = v3;
            v24 = 0;
            goto label_a4e9;
          }
          if (v23) goto label_a453;
          v7 = 0;
          v16 = 0x3f;
          v24 = 0;
          goto label_a513;
        
      }
label_a830:
      v21 = v24;
      if (!v2) {
label_a83b:
        v16 = (unsigned char)v20;
        v24 = 0;
        v7 = 0;
        v25 = v3;
        if (!v23) goto label_a510;
        goto label_a368;
      }
label_a692:
      v24 = 0;
      goto label_a6a3;
    }
    if (!(bool)(v12 == 0 & v23 & a4 == 2)) {
      v7 = a4 == 2 & (v23 ^ 1U);
      v23 = (bool)(v23 ^ 1U);
      if ((!(bool)v7) || (v23 = (bool)v7, !v3)) {
label_b35f:
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
        if (a1 || !v31) goto label_b35f;
        v22 = v31;
        goto label_ae2b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 1;
      v32 = (unsigned long)v9;
      v30 = "\"";
      if (!(v33 & 2)) goto label_b40f;
      v3 = 0;
      v19 = 0;
      v31 = 0;
      v2 = v4;
      v23 = v4;
      goto label_a226;
    }
label_ab2d:
    if (v2) {
label_a6cb:
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
label_a6f1:
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
label_ae2b:
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
label_b2d5:
    if (!a2[(long)v9]) break;
  }
label_b2df:
  v24 = 0;
label_af5b:
  v18 = (unsigned long)v16;
  v25 = v24;
  if (2 <= v20) {
label_af65:
    v25 = 0;
    v20 += v22;
    v13 = v22;
    do {
      v16 = (unsigned char)v18;
      if (v26) {
        v27 = a4 == 2;
        if (v23) goto label_a453;
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
        if (v20 <= v22) goto label_a3f1;
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
          goto label_ac80;
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
label_a4ce:
  if (v26) {
    v24 = 0;
    v26 = v2;
    goto label_af65;
  }
label_a4e0:
  v20 = v18;
  v24 = v25;
label_a4e7:
  v7 = a4 == 2;
  v25 = v3;
label_a4e9:
  v16 = (unsigned char)v20;
  v26 = (bool)v7 == 0;
  v3 = v25;
  v7 = 0;
  if ((v26 && v2) || (v7 = 0, v23)) {
label_a368:
    v16 = (unsigned char)v20;
    v12 = v13;
    if (!v28) goto label_a513;
    if (!(*(unsigned int *)(v28 + (v20 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_a513;
  }
  else {
label_a510:
    v7 = 0;
    v3 = v25;
label_a513:
    if (!v21) {
      v7 ^= 1;
      v22 += 1;
      v7 &= v19;
label_ac80:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v19 = 0;
        v12 += 2;
      }
      goto label_a3f1;
    }
  }
label_a6a3:
  if (v23) {
    v23 = v2;
label_a6bf:
    if ((bool)(v23 & a4 == 2)) goto label_a6cb;
label_a45c:
    v33 &= 0xfffffffd;
    v28 = 0;
    goto label_a150;
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
label_a3de:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v22 += 1;
label_a3f1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v24)
    v4 = 0;
  goto label_a230;
label_a902:
  v22 += 1;
  goto label_a230;
label_b40f:
  a1 = v31;
label_a531:
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
  goto label_a213;
}

// Function: sub_b510 @ 0xb510
char * sub_b510(unsigned int a0,char *a1,unsigned long a2,struct_3 *a3)
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
    v5 = dat_13080;
    if (dat_13078 <= (int)a0) {
      v10 = (long)dat_13078;
      v8 = (long)(int)((a0 - dat_13078) + 1);
      if (dat_13080 != (unsigned long *)0x13090) // branch-flip
        v5 = (unsigned long *)sub_8330(dat_13080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_8330(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_13090;
        v5[1] = dat_13098;
      }
      dat_13080 = v5;
      memset(&v5[(long)dat_13078 * 2],0,(v10 - dat_13078) * 0x10);
      dat_13078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_a0e0(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x23180)
        free(v7);
      v7 = (char *)sub_81a0(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_a0e0(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_b700 @ 0xb700
void sub_b700(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x23280;
  sub_8470(a0,0x38);
  *v2 = v1;
}

// Function: sub_b740 @ 0xb740
unsigned int sub_b740(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x23280;
  return *a0;
}

// Function: sub_b760 @ 0xb760
void sub_b760(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x23280;
  *a0 = a1;
}

// Function: sub_b780 @ 0xb780
unsigned int sub_b780(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x23280;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_b7c0 @ 0xb7c0
unsigned int sub_b7c0(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0x23280;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_b7e0 @ 0xb7e0
void sub_b7e0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x23280;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_b820 @ 0xb820
void sub_b820(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x23280;
  v2 = __errno_location();
  v1 = *v2;
  sub_a0e0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_b8a0 @ 0xb8a0
char * sub_b8a0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x23280;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_a0e0(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_81a0(v3 + 1U);
  sub_a0e0(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_b990 @ 0xb990
char * sub_b990(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_b8a0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_b9a0 @ 0xb9a0
void sub_b9a0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_13080;
  if (2 <= dat_13078) {
    v4 = (unsigned long *)((long)dat_13080 + 0x18);
    v1 = (unsigned long *)((long)dat_13080 + (unsigned long)(unsigned int)(dat_13078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x23180) {
    free(*(void **)((long)v3 + 8));
    dat_13098 = 0x23180;
    dat_13090 = 0x100;
  }
  if (v3 == (void *)0x13090) {
    dat_13078 = 1;
    return;
  }
  free(v3);
  dat_13078 = 1;
  dat_13080 = (void *)0x13090;
}

// Function: sub_ba40 @ 0xba40
char * sub_ba40(unsigned int a0,char *a1)
{
  return sub_b510(a0,a1,0xffffffffffffffff,(struct_3 *)0x23280); // tail-call
}

// Function: sub_ba60 @ 0xba60
char * sub_ba60(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_b510(a0,a1,a2); // tail-call
}

// Function: sub_ba70 @ 0xba70
char * sub_ba70(char *a0)
{
  return sub_ba40(0,a0); // tail-call
}

// Function: sub_ba80 @ 0xba80
char * sub_ba80(char *a0,unsigned long a1)
{
  return sub_ba60(0,a0,a1); // tail-call
}

// Function: sub_ba90 @ 0xba90
void sub_ba90(unsigned int a0,unsigned long a1,char *a2)
{
  char v1 [56];
  
  sub_9990(v1);
  sub_b510(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_baf0 @ 0xbaf0
void sub_baf0(unsigned int a0,unsigned long a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_9990(v1);
  sub_b510(a0,a2,a3,v1);
}

// Function: sub_bb60 @ 0xbb60
void sub_bb60(unsigned int a0,char *a1)
{
  sub_ba90(0,a0,a1); // tail-call
}

// Function: sub_bb70 @ 0xbb70
void sub_bb70(unsigned int a0,char *a1,unsigned long a2)
{
  sub_baf0(0,a0,a1,a2); // tail-call
}

// Function: sub_bb90 @ 0xbb90
void sub_bb90(char *a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_23280;
  v3 = dat_23288;
  v4 = dat_23290;
  v5 = dat_23298;
  v8 = dat_232b0;
  v6 = dat_232a0;
  v7 = dat_232a8;
  sub_b780(&v1,(int)a2,1,a3,a0,a1);
  sub_b510(0,a0,a1,v2);
}

// Function: sub_bc20 @ 0xbc20
void sub_bc20(char *a0,char a1)
{
  sub_bb90(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_bc40 @ 0xbc40
void sub_bc40(char *a0)
{
  sub_bc20(a0,0x3a); // tail-call
}

// Function: sub_bc50 @ 0xbc50
void sub_bc50(char *a0,unsigned long a1)
{
  sub_bb90(a0,a1,0x3a); // tail-call
}

// Function: sub_bc60 @ 0xbc60
void sub_bc60(unsigned int a0,unsigned long a1,char *a2)
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
  
  sub_9990(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_b780(v3,0x3a,1);
  sub_b510(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_bd00 @ 0xbd00
void sub_bd00(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_23280;
  v2 = dat_23288;
  v7 = dat_232b0;
  v3 = dat_23290;
  v4 = dat_23298;
  v5 = dat_232a0;
  v6 = dat_232a8;
  sub_b7e0(&v1,a1,a2);
  sub_b510(a0,a3,a4,&v1);
}

// Function: sub_bda0 @ 0xbda0
void sub_bda0(unsigned int a0)
{
  sub_bd00(a0); // tail-call
}

// Function: sub_bdb0 @ 0xbdb0
void sub_bdb0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_bda0(0,a0,a1,a2); // tail-call
}

// Function: sub_bdd0 @ 0xbdd0
void sub_bdd0(long a0,long a1,char *a2,unsigned long a3)
{
  sub_bd00(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_bdf0 @ 0xbdf0
char * sub_bdf0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_b510(a0,a1,a2); // tail-call
}

// Function: sub_be00 @ 0xbe00
char * sub_be00(char *a0,unsigned long a1)
{
  return sub_bdf0(0,a0,a1); // tail-call
}

// Function: sub_be10 @ 0xbe10
char * sub_be10(unsigned int a0,char *a1)
{
  return sub_bdf0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_be20 @ 0xbe20
char * sub_be20(char *a0)
{
  return sub_be10(0,a0); // tail-call
}

// Function: sub_be30 @ 0xbe30
int sub_be30(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_c5a0(v2,1);
    if (0 <= v1) { // branch-flip
label_be63:
      if (v3 == 3) {
        v1 = 0;
        return v1;
      }
    }
    else {
      if ((v2) || (v1 = __open_2("/dev/full",1), v1 < 0)) {
        v1 = __open_2("/dev/null",(unsigned int)(v2 == 0));
        if (v1 < 0) {
          v1 = *__errno_location();
          return v1;
        }
        if (3 <= v1) {
          close(v1); // return-dupe
          v1 = 0;
          return v1;
        }
        goto label_be63;
      }
      if (3 <= v1) {
        close(v1);
        v1 = 0;
        return v1;
      }
    }
    v2 += 1;
    v3 += 1;
  } while( true );
}

// Function: sub_bee0 @ 0xbee0
unsigned long sub_bee0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
{
  int v1;
  char *v2; // rax
  char *v3;
  char *v4; // stack - 0x1050
  char *v5; // stack - 0x1058
  char v6 [4104];
  unsigned long v7; // stack - 0x1068
  long v8; // stack - 0x1060
  
  v2 = NULL;
  iconv(a2,0,0,0,0);
  v7 = a0;
  v8 = a1;
  if (a1) {
    do {
      v4 = (char *)0x1000;
      v5 = v6;
      if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
        v1 = *__errno_location();
        if (v1 != 7) {
          if (v1 != 0x16)
            return 0xffffffff;
          break;
        }
      }
      v2 = &v5[(long)v2 - (long)v6];
    } while (v8);
  }
  v4 = (char *)0x1000;
  v5 = v6;
  if (iconv(a2,0,0,&v5,&v4) == -1)
    return 0xffffffff;
  v2 = &v5[(long)v2 - (long)v6];
  if (!v2) {
    *a4 = 0;
    return 0;
  }
  v3 = (char *)*a3;
  if (((!v3) || ((char *)*a4 < v2)) && (v3 = malloc((unsigned long)v2), !v3)) {
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  iconv(a2,0,0,0,0);
  v7 = a0;
  v8 = a1;
  v5 = v3;
  v4 = v2;
  do {
    if (!a1) {
label_c082:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_c12e:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_c12e;
      goto label_c082;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_c160 @ 0xc160
char * sub_c160(char *a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  unsigned long v10;
  char *v11;
  long v12; // rdx
  char *v2;
  char *v3; // rax
  int *v4; // rax
  unsigned long v5;
  unsigned long v6; // stack - 0x48
  char *v7; // stack - 0x50
  unsigned long v8; // stack - 0x58
  char *v9; // stack - 0x60
  
  v9 = a0;
  v8 = strlen(a0);
  v10 = v8;
  if (v8 <= 0xfffffff)
    v10 = v8 << 4;
  v5 = v10 + 1;
  v2 = malloc(v5);
  if (!v2) {
    *__errno_location() = 0xc;
    return NULL;
  }
  iconv(a1,0,0,0,0,v10,v10);
  v11 = v2;
  while (v7 = v2, v6 = v10, iconv(a1,&v9,&v8,&v7,&v6) == -1) {
    v4 = __errno_location();
    if (*v4 == 0x16) goto label_c2fd;
    if (*v4 != 7) {
      free(v11); // return-dupe
      return NULL;
    }
    v1 = v5 * 2;
    v12 = (long)v7 - (long)v11;
    if ((v1 <= v5) || (v3 = realloc(v11,v1), !v3)) {
      *v4 = 0xc;
      free(v11);
      return NULL;
    }
    v2 = &v3[v12];
    v10 = (v1 - 1) - v12;
    v5 = v1;
    v11 = v3;
  }
label_c2fd:
  do {
    if (iconv(a1,0,0,&v7,&v6) != -1) {
      v2 = &v7[1];
      *v7 = 0;
      v10 = (long)v2 - (long)v11;
      if (v5 <= v10)
        return v11;
      v7 = v2;
      v2 = realloc(v11,v10);
      if (!v2)
        return v11;
      return v2;
    }
    v4 = __errno_location();
    if (*v4 != 7) {
      free(v11);
      return NULL;
    }
    v10 = v5 * 2;
    v12 = (long)v7 - (long)v11;
    if ((v10 <= v5) || (v2 = realloc(v11,v10), !v2)) {
      *v4 = 0xc;
      free(v11);
      return NULL;
    }
    v7 = &v2[v12];
    v6 = (v10 - 1) - v12;
    v5 = v10;
    v11 = v2;
  } while( true );
}

// Function: sub_c3b0 @ 0xc3b0
char * sub_c3b0(char *a0,char *a1,char *a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_8890(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_c160(a0,v3);
  if (v2) {
    if (0 <= (int)iconv_close(v3))
      return v2;
    free(v2);
    return NULL;
  }
  v4 = __errno_location();
  v1 = *v4;
  iconv_close(v3);
  *v4 = v1;
  return NULL;
}

// Function: sub_c470 @ 0xc470
void * sub_c470(void *a0,unsigned long a1)
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

// Function: sub_c4b0 @ 0xc4b0
void sub_c4b0(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_c4c0 @ 0xc4c0
int sub_c4c0(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_232b8) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_232b8 = 1;
      return v2;
    }
    v2 = sub_c4b0(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_232b8 = -1;
  }
  else {
    v2 = sub_c4b0(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_232b8 != -1)
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

// Function: sub_c5a0 @ 0xc5a0
void sub_c5a0(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_c4b0(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_c4c0(a0,a2);
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

// Function: sub_c6f0 @ 0xc6f0
unsigned long sub_c6f0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_c950(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_c780 @ 0xc780
unsigned long sub_c780(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_c7b0 @ 0xc7b0
bool sub_c7b0(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_c7d0 @ 0xc7d0
bool sub_c7d0(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_c7e0 @ 0xc7e0
unsigned int sub_c7e0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_c800 @ 0xc800
unsigned long sub_c800(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_c820 @ 0xc820
bool sub_c820(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_c830 @ 0xc830
bool sub_c830(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_c840 @ 0xc840
bool sub_c840(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_c850 @ 0xc850
bool sub_c850(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_c860 @ 0xc860
unsigned long sub_c860(int a0)
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

// Function: sub_c8b0 @ 0xc8b0
bool sub_c8b0(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_c8d0 @ 0xc8d0
bool sub_c8d0(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_c8e0 @ 0xc8e0
unsigned int sub_c8e0(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_c910 @ 0xc910
int sub_c910(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_c930 @ 0xc930
int sub_c930(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_c950 @ 0xc950
bool sub_c950(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_cc90(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_c9e0 @ 0xc9e0
long sub_c9e0(unsigned long a0)
{
  long v1;
  void *v2; // rax
  unsigned long v3; // rax
  
  v3 = a0 + 0x20;
  if ((0 <= (long)v3) && (a0 <= v3)) {
    v2 = malloc(v3);
    if (v2) {
      v3 = (long)v2 + 0x10U & 0xffffffffffffffe0;
      v1 = v3 + 0x10;
      *(char *)(v3 + 0xf) = (char)v1 - (char)v2;
      return v1;
    }
    return 0;
  }
  return 0;
}

// Function: sub_ca30 @ 0xca30
void sub_ca30(void *a0)
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1))); // tail-call
}

// Function: sub_ca60 @ 0xca60
int sub_ca60(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_ca90 @ 0xca90
void sub_ca90(struct_6 *a0,struct_7 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_7 *v3;
  
  v3 = (struct_7 *)a1->field_0x0;
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

// Function: sub_caf0 @ 0xcaf0
unsigned int sub_caf0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0xe720) >> (a0 & 0x1f) & 1;
}

// Function: sub_cb10 @ 0xcb10
unsigned long sub_cb10(char *a0)
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
    sub_97c0(v1);
    if ((v7) && (!v8)) break;
    v5 = &v5[v6];
    v2 += 1;
  }
  return v2;
}

// Function: sub_cbd0 @ 0xcbd0
void sub_cbd0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_cbe0 @ 0xcbe0
unsigned long sub_cbe0(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_cbd0();
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

// Function: sub_cc90 @ 0xcc90
unsigned long sub_cc90(void)
{
  return sub_cbe0(); // tail-call
}

// Function: sub_cca0 @ 0xcca0
void sub_cca0(void)
{
  sub_cbd0(); // tail-call
}

// Function: _DT_FINI @ 0xccac
void _DT_FINI(void)
{
  return;
}
