// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_13fd8)
    return;
  (*dat_13fd8)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_13d38)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2540
void __cxa_finalize(void)
{
  (*dat_13fe8)(); // jump-as-call
}

// Function: raise @ 0x2550
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_13d40)(); // jump-as-call
  return v1;
}

// Function: free @ 0x2560
void free(void *a0)
{
  (*dat_13d48)(); // jump-as-call
}

// Function: abort @ 0x2570
void abort(void)
{
  (*dat_13d50)(); // jump-as-call
}

// Function: __errno_location @ 0x2580
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_13d58)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2590
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_13d60)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x25a0
void _exit(int a0)
{
  (*dat_13d68)(); // jump-as-call
}

// Function: iconv @ 0x25b0
void iconv(void)
{
  (*dat_13d70)(); // jump-as-call
}

// Function: getc_unlocked @ 0x25c0
int getc_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_13d78)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x25d0
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_13d80)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x25e0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_13d88)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x25f0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13d90)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x2600
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_13d98)(); // jump-as-call
  return v1;
}

// Function: write @ 0x2610
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_13da0)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2620
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13da8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2630
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_13db0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2640
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13db8)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2650
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_13dc0)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2660
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_13dc8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2670
void __stack_chk_fail(void)
{
  (*dat_13dd0)(); // jump-as-call
}

// Function: mmap @ 0x2680
void mmap(void)
{
  (*dat_13dd8)(); // jump-as-call
}

// Function: getopt_long @ 0x2690
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_13de0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x26a0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_13de8)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x26b0
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13df0)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x26c0
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_13df8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x26d0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13e00)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x26e0
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_13e08)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x26f0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13e10)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2700
void lseek(void)
{
  (*dat_13e18)(); // jump-as-call
}

// Function: __assert_fail @ 0x2710
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_13e20)(); // jump-as-call
}

// Function: fputs @ 0x2720
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_13e28)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x2730
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13e30)(); // jump-as-call
  return v1;
}

// Function: freopen @ 0x2740
FILE * freopen(char *a0,char *a1,FILE *a2)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_13e38)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x2750
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_13e40)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2760
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_13e48)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x2770
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_13e50)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x2780
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13e58)(); // jump-as-call
  return v1;
}

// Function: read @ 0x2790
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_13e60)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x27a0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_13e68)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x27b0
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_13e70)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x27c0
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_13e78)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x27d0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13e80)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x27e0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_13e88)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x27f0
void signal(void)
{
  (*dat_13e90)(); // jump-as-call
}

// Function: fputc_unlocked @ 0x2800
int fputc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_13e98)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x2810
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_13ea0)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x2820
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_13ea8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2830
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_13eb0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2840
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13eb8)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x2850
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_13ec0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x2860
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_13ec8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x2870
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_13ed0)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x2880
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_13ed8)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x2890
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_13ee0)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x28a0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13ee8)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x28b0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13ef0)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x28c0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_13ef8)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x28d0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13f00)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x28e0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_13f08)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x28f0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13f10)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x2900
void iconv_close(void)
{
  (*dat_13f18)(); // jump-as-call
}

// Function: iswspace @ 0x2910
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_13f20)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x2920
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13f28)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x2930
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13f30)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2940
void error(int a0,int a1,char *a2,...)
{
  (*dat_13f38)(); // jump-as-call
}

// Function: waitpid @ 0x2950
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_13f40)(); // jump-as-call
  return v1;
}

// Function: open @ 0x2960
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_13f48)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x2970
void perror(char *a0)
{
  (*dat_13f50)(); // jump-as-call
}

// Function: execvp @ 0x2980
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_13f58)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x2990
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_13f60)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x29a0
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_13f68)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x29b0
void exit(int a0)
{
  (*dat_13f70)(); // jump-as-call
}

// Function: strdup @ 0x29c0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_13f78)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x29d0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_13f80)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x29e0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_13f88)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x29f0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_13f90)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x2a00
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_13f98)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x2a10
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_13fa0)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x2a20
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_13fa8)(); // jump-as-call
  return v1;
}

// Function: feof_unlocked @ 0x2a30
int feof_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_13fb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2a40
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_13fb8)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x2a50
void iconv_open(void)
{
  (*dat_13fc0)(); // jump-as-call
}

// Function: sub_2a60 @ 0x2a60
void sub_2a60(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_13fc8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_2a90 @ 0x2a90
void sub_2a90(void)
{
  return;
}

// Function: _FINI_0 @ 0x2b00
void _FINI_0(void)
{
  if (!dat_14108) {
    if (dat_13fe8)
      __cxa_finalize(dat_14008);
    sub_2a90();
    dat_14108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x2b40
void _INIT_0(void)
{
  return;
}

// Function: sub_2b49 @ 0x2b49
void sub_2b49(void)
{
  return;
}

// Function: sub_2b58 @ 0x2b58
void sub_2b58(void)
{
  return;
}

// Function: main @ 0x2b67
int main(int argc,char **argv,char **envp) // return-dupe
{
  char *v1;
  void *v10; // stack - 0x118
  unsigned char v11; // stack - 0x135
  int v12; // stack - 0x134
  int v13; // stack - 0x130
  unsigned int v14; // stack - 0x12c
  int v15; // stack - 0x128
  char **v16; // stack - 0x108
  char *v17; // stack - 0x100
  long v18; // stack - 0xf8
  long v19; // stack - 0xf0
  bool v2;
  unsigned long v20; // stack - 0xe8
  int v21; // stack - 0xd0
  int v22; // stack - 0xcc
  bool v3;
  int v4;
  unsigned long v5 [4]; // stack - 0xc8
  int v6 [4]; // stack - 0xe0
  stat v7; // stack - 0xa8
  void *v8; // stack - 0x110
  char v9 [8];
  
  v14 = 0;
  v15 = 0;
  dat_14030 = 2;
  sub_637f(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_62bb(NULL);
  sub_9533();
  while (v4 = getopt_long(argc,argv,"aeimvx3AEL:TX",(option *)0x13880,NULL), v4 != -1) {
    switch(v4) {
      case 0x33:
        dat_1410e = 1;
        v14 |= 1;
        break;
      default:
        sub_3489(NULL,0); // no-return
      case 0x41:
        dat_14110 = '\x01';
        dat_1410c = '\x01';
        v14 |= 2;
        break;
      case 0x45:
        dat_1410c = '\x01';
        v14 |= 4;
        break;
      case 0x4c:
        if (2 < v15)
          sub_3489("too many file label options",0); // no-return
        v4 = v15 + 1;
        v5[v15] = optarg;
        v15 = v4;
        break;
      case 0x54:
        dat_1410d = 1;
        break;
      case 0x58:
        dat_1410f = 1;
        v14 |= 8;
        break;
      case 0x61:
        dat_14109 = 1;
        break;
      case 0x65:
        v14 |= 0x10;
        break;
      case 0x69:
        dat_14111 = 1;
        break;
      case 0x6d:
        dat_14112 = 1;
        break;
      case 0x76:
        sub_8be7(stdout,"diff3","GNU diffutils",dat_14028,sub_6940("Randy Smith"),0);
        sub_34ff();
        return 0;
      case 0x78:
        dat_1410f = 1;
        v14 |= 0x20;
        break;
      case 0x80:
        dat_14020 = optarg;
        break;
      case 0x81:
        sub_3556();
        sub_34ff();
        return 0;
      case 0x82:
        dat_1410a = 1;
      
    }
  }
  dat_1410b = ((dat_14112 ^ 1) & v14 != 0) != 0;
  dat_14110 = dat_14112 & v14 == 0 || dat_14110;
  v2 = (dat_14112 & v14 == 0) != 0;
  v3 = dat_1410c != '\0';
  dat_1410c = v2 || v3;
  if (((!(v14 - 1 & v14)) && (!(dat_14112 & dat_14111))) && ((!v15 || (v2 || v3)))) {
    if (argc - optind != 3) {
      if (3 <= argc - optind)
        sub_3489("extra operand \'%s\'",argv[(long)optind + 3]); // no-return
      sub_3489("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
    }
    v16 = &argv[optind];
    for (v12 = v15; v12 <= 2; v12 = v12 + 1) {
      v5[v12] = v16[v12];
    }
    v13 = 2 - (unsigned int)(unsigned char)(dat_14112 | dat_1410b);
    v1 = v16[v13];
    if (!strcmp(v1,"-")) {
      v13 = 3 - v13;
      v1 = *v16;
      if ((!strcmp(v1,"-")) || (v1 = v16[v13], !strcmp(v1,"-")))
        sub_5fcf("\'-\' specified for more than one input file"); // no-return
    }
    v6[0] = 0;
    v6[1] = 3 - v13;
    v6[2] = v13;
    for (v12 = 0; v12 <= 2; v12 = v12 + 1) {
      v6[(long)v6[v12] + 3] = v12;
    }
    v12 = 0;
    do {
      if (3 <= v12) {
        signal(0x11,0);
        v17 = v16[v22];
        v1 = v16[v21];
        v18 = sub_43e5(v1,v17,v9,&v8);
        v19 = sub_43e5(v16[v6[3]],v17,v9,&v10);
        sub_2b58(v19);
        sub_2b58(v18);
        v20 = sub_3696(v19,v18);
        sub_2b49(v19);
        sub_2b49(v18);
        if (dat_1410b) // branch-flip
          v11 = sub_54de(stdout,v20,v6,&v6[3],v5[0],v5[1],v5[2]);
        else if (dat_14112) { // branch-flip
          sub_9414(v16[v6[3]],"r",stdin);
          v11 = sub_5a29(stdin,stdout,v20,v6,&v6[3],v5[0],v5[1],v5[2]);
          if (ferror_unlocked(stdin))
            sub_5fcf("read failed"); // no-return
        }
        else {
          sub_5012(stdout,v20,v6,&v6[3]);
          v11 = 0;
        }
        free(v10);
        free(v8);
        sub_34ff();
        exit((unsigned int)v11); // no-return
      }
      v1 = v16[v12];
      if (strcmp(v1,"-")) {
        v1 = v16[v12];
        if (stat(v1,&v7) <= -1)
          sub_600c(v16[v12]); // no-return
        if ((v7._24_4_ & 0xf000) == 0x4000)
          error(2,0x15,"%s",v16[v12]); // no-return
      }
      v12 += 1;
    } while( true );
  }
  sub_3489("incompatible options",0); // no-return
}

// Function: sub_3489 @ 0x3489
void sub_3489(char *a0,unsigned long a1)
{
  char *v1;
  
  v1 = dat_24148;
  if (a0)
    error(0,0,gettext(a0),a1); // no-return
  error(2,0,gettext("Try \'%s --help\' for more information."),v1); // no-return
}

// Function: sub_34ff @ 0x34ff
void sub_34ff(void)
{
  if (ferror_unlocked(stdout))
    sub_5fcf("write failed"); // no-return
  if (fclose(stdout))
    sub_600c(gettext("standard output")); // no-return
}

// Function: sub_3556 @ 0x3556
void sub_3556(void)
{
  FILE *v1;
  char *v2;
  long *v3; // stack - 0x20
  
  v2 = dat_24148;
  printf(gettext("Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n"),v2);
  printf("%s\n\n",gettext("Compare three files line by line."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  for (v3 = (long *)0x13a40; v1 = stdout, *v3; v3 = &v3[1]) {
    if (*(char *)*v3) { // branch-flip
      v2 = (char *)*v3;
      printf("  %s\n",gettext(v2));
    }
    else {
      putchar_unlocked(10);
    }
  }
  fputs_unlocked(gettext("\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n"),v1);
  v2 = gettext("Exit status is 0 if successful, 1 if conflicts, 2 if trouble.");
  printf("\n%s\n%s\n",gettext("If a FILE is \'-\', read standard input."),v2);
  sub_8ccb();
}

// Function: sub_3696 @ 0x3696
long sub_3696(long a0,long a1)
{
  long v1; // rax
  long v10; // stack - 0x60
  long v11; // stack - 0x58
  long v12; // stack - 0x50
  long v13; // stack - 0x20
  long v2; // stack - 0x80
  long v3 [5]; // stack - 0x48
  unsigned int v4; // stack - 0x8c
  unsigned int v5; // stack - 0x88
  unsigned int v6; // stack - 0x84
  long v7; // stack - 0x78
  long v8; // stack - 0x70
  long *v9; // stack - 0x68
  
  v2 = 0;
  v9 = &v2;
  v3[4] = a0;
  v13 = a1;
  v10 = 0xfd80;
  while( true ) {
    if ((!v3[4]) && (!v13))
      return v2;
    v3[3] = 0;
    v3[2] = 0;
    v3[1] = 0;
    v3[0] = 0;
    if (v3[4]) { // branch-flip
      if (v13) // branch-flip
        v5 = (unsigned int)(*(long *)(v13 + 0x10) < *(long *)(v3[4] + 0x10));
      else {
        v5 = 0;
      }
    }
    else {
      v5 = 1;
    }
    v4 = v5;
    v11 = v3[(long)(int)v5 + 4];
    v7 = *(long *)(v11 + 0x18);
    v3[(long)(int)v5 + 2] = v11;
    v3[(int)v5] = v3[(long)(int)v5 + 2];
    v3[(long)(int)v5 + 4] = *(long *)(v11 + 0x40);
    *(unsigned long *)(v3[(long)(int)v5 + 2] + 0x40) = 0;
    v6 = v5 ^ 1;
    v8 = v3[(long)(int)v6 + 4];
    while ((v8 && (*(long *)(v8 + 0x10) <= v7 + 1))) {
      if (v3[(int)v6]) // branch-flip
        *(long *)(v3[(long)(int)v6 + 2] + 0x40) = v8;
      else {
        v3[(int)v6] = v8;
      }
      v3[(long)(int)v6 + 2] = v8;
      v3[(long)(int)v6 + 4] = *(long *)(v3[(long)(int)v6 + 4] + 0x40);
      *(unsigned long *)(v8 + 0x40) = 0;
      if (v7 < *(long *)(v8 + 0x18)) {
        v4 ^= 1;
        v7 = *(long *)(v8 + 0x18);
      }
      v6 = v4 ^ 1;
      v8 = v3[(long)(int)v6 + 4];
    }
    v1 = sub_393d(v3,&v3[2],v5,v4,v10);
    v12 = v1;
    if (!v1) break;
    *v9 = v1;
    v9 = (long *)(v1 + 0x68);
    v10 = v1;
  }
  sub_5fcf("internal error: screwup in format of diff blocks"); // no-return
}

// Function: sub_393d @ 0x393d
unsigned int * sub_393d(long *a0,long a1,int a2,int a3,long a4)
{
  long v1;
  unsigned int *v10; // rax
  long v11;
  long v12;
  long v13;
  long v14 [5]; // stack - 0x38
  int v15; // stack - 0xa4
  long *v16; // stack - 0xa0
  long v17; // stack - 0x98
  long v18; // stack - 0x90
  long v2;
  long v3;
  long v4;
  long v5;
  unsigned long *v6;
  long *v7;
  unsigned long *v8;
  long *v9;
  
  v13 = *(long *)(a0[a2] + 0x10);
  v1 = *(long *)(*(long *)(a1 + (long)a3 * 8) + 0x18);
  for (v15 = 0; v15 <= 1; v15 = v15 + 1) {
    if (a0[v15]) { // branch-flip
      v14[v15] = (v13 - *(long *)(a0[v15] + 0x10)) + *(long *)a0[v15];
      v14[(long)v15 + 2] = (v1 - *(long *)(*(long *)(a1 + (long)v15 * 8) + 0x18)) + *(long *)(*(long *)(a1 + (long)v15 * 8) + 8);
    }
    else {
      v14[v15] = (v13 - *(long *)(a4 + 0x30)) + *(long *)((long)v15 * 0x10 + a4 + 0x10);
      v14[(long)v15 + 2] = (v1 - *(long *)(a4 + 0x30)) + *(long *)((long)v15 * 0x10 + a4 + 0x10);
    }
  }
  v10 = (unsigned int *)sub_4116(v14[0],v14[2],v14[1],v14[3],v13,v1);
  for (v15 = 0; v15 <= 1; v15 = v15 + 1) {
    for (v16 = (long)a0[v15]; v16; v16 = *(long **)((long)v16 + 0x40)) {
      v11 = *(long *)((long)v16 + 0x10) - v13;
      v1 = *(long *)((long)v16 + 0x18);
      v2 = *(long *)((long)v16 + 0x10);
      v12 = *(long *)&v10[0x18];
      v3 = *(long *)&v10[0x12];
      v6 = *(unsigned long **)((long)v16 + 0x38);
      v7 = *(long **)((long)v16 + 0x28);
      if (sub_4067(v7,v6,(long *)(v3 + v11 * 8),(unsigned long *)(v12 + v11 * 8),(v1 - v2) + 1) != '\x01')
        return NULL;
    }
  }
  v15 = 0;
  do {
    if (2 <= v15) {
      if (*a0) { // branch-flip
        if (a0[1]) { // branch-flip
          v13 = (*(long *)&v10[4] - *(long *)&v10[2]) + 1;
          if ((v13 != (*(long *)&v10[8] - *(long *)&v10[6]) + 1) || (v6 = *(unsigned long **)&v10[0x16], v7 = *(long **)&v10[0x10], v8 = *(unsigned long **)&v10[0x14], v9 = *(long **)&v10[0xe], sub_4311(v9,v8,v7,v6,v13) != '\x01'))
            *v10 = 4;
          else {
            *v10 = 7;
          }
        }
        else {
          *v10 = 5;
        }
      }
      else {
        *v10 = 6;
      }
      return v10;
    }
    v16 = (long *)a0[v15];
    v1 = v14[v15];
    v2 = v14[(long)v15 + 2];
    v17 = 0;
    while( true ) {
      if (v16) // branch-flip
        v12 = *v16;
      else {
        v12 = v2 + 1;
      }
      if (v12 <= v17 + v1) break;
      *(unsigned long *)(*(long *)&v10[((long)v15 + 6) * 2 + 2] + v17 * 8) = *(unsigned long *)(*(long *)&v10[0x12] + v17 * 8);
      *(unsigned long *)(*(long *)&v10[((long)v15 + 10) * 2] + v17 * 8) = *(unsigned long *)(*(long *)&v10[0x18] + v17 * 8);
      v17 += 1;
    }
    for (; v16; v16 = (long *)v16[8]) {
      v12 = *v16;
      v3 = v16[1];
      v11 = *v16;
      v4 = *(long *)&v10[((long)v15 + 10) * 2];
      v5 = *(long *)&v10[((long)v15 + 6) * 2 + 2];
      v6 = (unsigned long *)v16[6];
      v7 = (long *)v16[4];
      if (sub_4067(v7,v6,(long *)(v5 + (v12 - v1) * 8),(unsigned long *)(v4 + (v12 - v1) * 8),(v3 - v11) + 1) != '\x01')
        return NULL;
      v18 = (v16[3] + 1) - v13;
      v17 = (v16[1] + 1) - v1;
      while( true ) {
        if (v16[8]) // branch-flip
          v12 = *(long *)v16[8];
        else {
          v12 = v2 + 1;
        }
        if (v12 - v1 <= v17) break;
        *(unsigned long *)(*(long *)&v10[((long)v15 + 6) * 2 + 2] + v17 * 8) = *(unsigned long *)(*(long *)&v10[0x12] + v18 * 8);
        *(unsigned long *)(*(long *)&v10[((long)v15 + 10) * 2] + v17 * 8) = *(unsigned long *)(*(long *)&v10[0x18] + v18 * 8);
        v18 += 1;
        v17 += 1;
      }
    }
    v15 += 1;
  } while( true );
}

// Function: sub_4067 @ 0x4067
unsigned long sub_4067(long *a0,unsigned long *a1,long *a2,unsigned long *a3,long a4)
{
  long v1;
  unsigned long v2;
  void *v3;
  void *v4;
  long v5; // stack - 0x50
  
  v5 = a4;
  do {
    v1 = v5 + -1;
    if (!v5)
      return 1;
    if (*a2) { // branch-flip
      if ((*a1 != *a3) || (v2 = *a1, v3 = (void *)*a2, v4 = (void *)*a0, memcmp(v4,v3,v2)))
        return 0;
    }
    else {
      *a2 = *a0;
      *a3 = *a1;
    }
    a2 = &a2[1];
    a0 = &a0[1];
    a3 = &a3[1];
    a1 = &a1[1];
    v5 = v1;
  } while( true );
}

// Function: sub_4116 @ 0x4116
struct_0 * sub_4116(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  struct_0 *v1; // rax
  unsigned long v2; // rax
  
  v1 = (struct_0 *)sub_8e10(0x70);
  v1->field_0x0 = 0;
  v1->field_0x68 = 0;
  v1->field_0x8 = a0;
  v1->field_0x10 = a1;
  v1->field_0x18 = a2;
  v1->field_0x20 = a3;
  v1->field_0x28 = a4;
  v1->field_0x30 = a5;
  v2 = (v1->field_0x10 - v1->field_0x8) + 1;
  if (v2) { // branch-flip
    v1->field_0x38 = sub_927a(v2,8);
    v1->field_0x50 = sub_927a(v2,8);
  }
  else {
    v1->field_0x38 = 0;
    v1->field_0x50 = 0;
  }
  v2 = (v1->field_0x20 - v1->field_0x18) + 1;
  if (v2) { // branch-flip
    v1->field_0x40 = sub_927a(v2,8);
    v1->field_0x58 = sub_927a(v2,8);
  }
  else {
    v1->field_0x40 = 0;
    v1->field_0x58 = 0;
  }
  v2 = (v1->field_0x30 - v1->field_0x28) + 1;
  if (v2) { // branch-flip
    v1->field_0x48 = sub_927a(v2,8);
    v1->field_0x60 = sub_927a(v2,8);
  }
  else {
    v1->field_0x48 = 0;
    v1->field_0x60 = 0;
  }
  return v1;
}

// Function: sub_4311 @ 0x4311
unsigned long sub_4311(long *a0,unsigned long *a1,long *a2,unsigned long *a3,long a4)
{
  unsigned long *v1;
  long *v10; // stack - 0x28
  long *v11; // stack - 0x20
  unsigned long *v12; // stack - 0x18
  unsigned long *v13; // stack - 0x10
  unsigned long *v2;
  long *v3;
  long *v4;
  long v5;
  unsigned long v6;
  void *v7;
  void *v8;
  long v9; // stack - 0x50
  
  v9 = a4;
  v10 = a0;
  v11 = a2;
  v12 = a1;
  v13 = a3;
  while( true ) {
    v5 = v9 + -1;
    if (!v9)
      return 1;
    if (!*v10)
      return 0;
    if (!*v11)
      return 0;
    v1 = &v13[1];
    if (*v12 != *v13) break;
    v2 = &v12[1];
    v6 = *v12;
    v3 = &v11[1];
    v7 = (void *)*v11;
    v4 = &v10[1];
    v8 = (void *)*v10;
    v9 = v5;
    v10 = v4;
    v11 = v3;
    v12 = v2;
    v13 = v1;
    if (memcmp(v8,v7,v6))
      return 0;
  }
  return 0;
}

// Function: sub_43e5 @ 0x43e5
long sub_43e5(char *a0,char *a1,unsigned long *a2,unsigned long *a3)
{
  char *v1;
  char *v10; // stack - 0x68
  long v11; // stack - 0x58
  long *v12; // stack - 0x50
  long *v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  char *v15; // stack - 0x38
  unsigned long v16; // stack - 0x30
  unsigned long v17; // stack - 0x28
  char v2;
  long v3;
  unsigned int v4; // eax
  long v5;
  long v6; // rax
  long v7; // rax
  long v8; // stack - 0x60
  char *v9; // stack - 0x70
  
  v12 = &v8;
  v14 = 0xfffffffffffffff;
  v15 = (char *)sub_4a3a(a0,a1,&v9);
  *a3 = v9;
  v10 = v9;
  do {
    if (v15 <= v10) {
      *v12 = 0;
      *a2 = v13;
      return v8;
    }
    v13 = (long *)sub_8e10(0x48);
    v13[5] = 0;
    v13[4] = v13[5];
    v13[7] = 0;
    v13[6] = v13[7];
    v4 = sub_4887(&v10,v13);
    v1 = dat_24148;
    if ((!v4) || (*v10 != '\n')) {
      fprintf(stderr,gettext("%s: diff failed: "),v1);
      do {
        putc_unlocked((int)*v10,stderr);
        v1 = &v10[1];
        v2 = *v10;
        v10 = v1;
      } while (v2 != '\n');
      exit(2); // no-return
    }
    v10 = &v10[1];
    if (v4 != 3) { // branch-flip
      if (4 <= v4)
        sub_5fcf("internal error: invalid diff type in process_diff"); // no-return, return-dupe
      if (v4 != 1) { // branch-flip
        if (v4 != 2) {
          sub_5fcf("internal error: invalid diff type in process_diff");
        }
      }
      else {
        *v13 = *v13 + 1;
      }
    }
    else {
      v13[2] = v13[2] + 1;
    }
    if (v4 != 1) {
      v16 = (v13[1] - *v13) + 1;
      if (v14 <= v16)
        sub_93d4(); // no-return
      v5 = sub_8e10(v16 * 8);
      v13[4] = v5;
      v5 = sub_8e10(v16 << 3);
      v13[6] = v5;
      for (v11 = 0; v11 < (long)v16; v11 = v11 + 1) {
        v5 = v13[6];
        v6 = v11 * 8;
        v3 = v13[4];
        v7 = v11 * 8;
        v10 = (char *)sub_4edc(v10,(unsigned long *)(v3 + v7),(long *)(v5 + v6),v15,0x3c);
      }
    }
    if (v4 == 2) {
      if (strncmp(v10,"---\n",4))
        sub_5fcf("invalid diff format; invalid change separator"); // no-return
      v10 = &v10[4];
    }
    if (v4 != 3) {
      v17 = (v13[3] - v13[2]) + 1;
      if (v14 <= v17)
        sub_93d4(); // no-return
      v5 = sub_8e10(v17 * 8);
      v13[5] = v5;
      v5 = sub_8e10(v17 << 3);
      v13[7] = v5;
      for (v11 = 0; v11 < (long)v17; v11 = v11 + 1) {
        v5 = v13[7];
        v6 = v11 * 8;
        v3 = v13[5];
        v7 = v11 * 8;
        v10 = (char *)sub_4edc(v10,(unsigned long *)(v3 + v7),(long *)(v5 + v6),v15,0x3e);
      }
    }
    *v12 = (long)v13;
    v12 = &v13[8];
  } while( true );
}

// Function: sub_47d5 @ 0x47d5
char * sub_47d5(char *a0)
{
  char *v1; // stack - 0x10
  
  for (v1 = a0; (*v1 == ' ' || (*v1 == '\t')); v1 = &v1[1]) {
  }
  return v1;
}

// Function: sub_4804 @ 0x4804
unsigned char * sub_4804(unsigned char *a0,long *a1) // early-return
{
  unsigned char *v1; // stack - 0x20
  unsigned char v2; // stack - 0x11
  long v3; // stack - 0x10
  
  v2 = *a0;
  v3 = 0;
  v1 = a0;
  if (10 <= (unsigned int)v2 - 0x30)
    return NULL;
  do {
    v3 = v3 * 10 + (long)(int)((unsigned int)v2 - 0x30);
    v1 = &v1[1];
    v2 = *v1;
  } while ((unsigned int)v2 - 0x30 <= 9);
  *a1 = v3;
  return v1;
}

// Function: sub_4887 @ 0x4887
unsigned int sub_4887(unsigned long *a0,long *a1) // return-dupe x5
{
  char v1;
  char *v2;
  unsigned int v3; // stack - 0x1c
  char *v4; // stack - 0x18
  
  v2 = (char *)*a0;
  v2 = (char *)sub_4804((unsigned char *)sub_47d5(v2),a1);
  if (!v2)
    return 0;
  v4 = (char *)sub_47d5(v2);
  if (*v4 != ',') // branch-flip
    a1[1] = *a1;
  else {
    v4 = (char *)sub_4804(&v4[1],&a1[1]);
    if (!v4)
      return 0;
  }
  v2 = (char *)sub_47d5(v4);
  v1 = *v2;
  if (v1 != 'd') { // branch-flip
    if ('d' < v1)
      return 0;
    if (v1 != 'a') { // branch-flip
      if (v1 != 'c')
        return 0;
      v3 = 2;
    }
    else {
      v3 = 1;
    }
  }
  else {
    v3 = 3;
  }
  v2 = (char *)sub_4804((unsigned char *)sub_47d5(&v2[1]),&a1[2]);
  if (v2) {
    v4 = (char *)sub_47d5(v2);
    if (*v4 != ',') // branch-flip
      a1[3] = a1[2];
    else {
      v2 = (char *)sub_4804(&v4[1],&a1[3]);
      if (!v2)
        return 0;
      v4 = (char *)sub_47d5(v2);
    }
    *a0 = v4;
    return v3;
  }
  return 0;
}

// Function: sub_4a3a @ 0x4a3a
long sub_4a3a(char *a0,char *a1,unsigned long *a2) // ternary x2
{
  char **v1;
  int v10; // stack - 0x150
  int v11; // stack - 0x14c
  int v12; // stack - 0x148
  unsigned int v13; // stack - 0x144
  void *v14; // stack - 0x140
  unsigned long v15; // stack - 0x138
  long v16; // stack - 0x130
  char **v17; // stack - 0x128
  long v18; // stack - 0x120
  long v19; // stack - 0x118
  char *v2;
  int v20; // stack - 0x10c
  int v3; // eax
  char *v4; // rax
  char *v5 [9]; // stack - 0x100
  int v6; // stack - 0x110
  char *v7; // stack - 0x108
  stat v8; // stack - 0xb8
  unsigned int v9; // stack - 0x154
  
  v10 = 0;
  v17 = v5;
  v7 = dat_14020;
  if (dat_14109) {
    v17 = &v5[1];
    v5[0] = "-a";
  }
  if (dat_1410a) {
    v1 = &v17[1];
    *v17 = "--strip-trailing-cr";
    v17 = v1;
  }
  *v17 = "--horizon-lines=100";
  v17[1] = "--";
  v1 = &v17[3];
  v17[2] = a0;
  v17 = &v17[4];
  *v1 = a1;
  *v17 = NULL;
  if (pipe(&v6))
    sub_600c("pipe"); // no-return
  v11 = fork();
  if (!v11) {
    close(v6);
    if (v20 != 1) {
      dup2(v20,1);
      close(v20);
    }
    execvp(dat_14020,&v7);
    v3 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
    _exit(v3); // no-return
  }
  if (v11 != -1) {
    close(v20);
    v12 = v6;
    if (fstat(v6,&v8))
      sub_600c("fstat"); // no-return
    if ((long)v8._56_8_ <= 0)
      v8._56_8_ = 1;
    v15 = v8._56_8_;
    v14 = (void *)sub_8e10(v8._56_8_);
    v16 = 0;
    while( true ) {
      v18 = v15 - v16;
      v19 = sub_9671(v12,(void *)((long)v14 + v16),v18);
      v16 += v19;
      if (v19 != v18) break;
      if (0x3fffffffffffffff <= v15)
        sub_93d4(); // no-return
      v15 <<= 1;
      v14 = (void *)sub_8e5c(v14,v15);
    }
    if (v19 == -1)
      sub_600c(gettext("read failed")); // no-return
    if ((v16) && (*(char *)((long)v14 + v16 + -1) != '\n'))
      sub_5fcf("invalid diff format; incomplete last line"); // no-return
    *a2 = v14;
    if (close(v12))
      sub_600c("close"); // no-return
    v3 = waitpid(v11,(int *)&v9,0);
    v2 = dat_14020;
    if (0 <= v3) {
      if ((v10) || (v9 & 0x7f))
        v13 = 0x7fffffff;
      else {
        v13 = (int)v9 >> 8 & 0xff;
      }
      if (v13 <= 1)
        return v16 + (long)v14;
      if (v13 != 0x7e) { // branch-flip
        if (v13 != 0x7f) { // branch-flip
          v4 = (v13 != 0x7fffffff) ? "subsidiary program \'%s\' failed (exit status %d)" : "subsidiary program \'%s\' failed"; // branch-flip
        }
        else {
          v4 = "subsidiary program \'%s\' not found";
        }
      }
      else {
        v4 = "subsidiary program \'%s\' could not be invoked";
      }
      v4 = gettext(v4);
      error(2,v10,v4,v2,(unsigned long)v13); // no-return
    }
    sub_600c("waitpid"); // no-return
  }
  sub_600c("fork"); // no-return
}

// Function: sub_4edc @ 0x4edc
char * sub_4edc(char *a0,unsigned long *a1,long *a2,char *a3,char a4)
{
  char *v1;
  char v2;
  char *v3; // stack - 0x10
  
  if ((a4 == *a0) && (a0[1] == ' ')) {
    *a1 = (unsigned long)&a0[2];
    v1 = &a0[2];
    do {
      v3 = v1;
      v1 = &v3[1];
    } while (*v3 != '\n');
    *a2 = (long)v1 - *a1;
    if ((v1 < a3) && (*v1 == '\\')) {
      if (dat_1410b) // branch-flip
        fprintf(stderr,"%s:",dat_24148);
      else {
        *a2 = *a2 + -1;
      }
      v3 = &v3[2];
      do {
        if (dat_1410b)
          putc_unlocked((int)*v3,stderr);
        v1 = &v3[1];
        v2 = *v3;
        v3 = v1;
      } while (v2 != '\n');
    }
    v3 = v1;
    return v3;
  }
  sub_5fcf("invalid diff format; incorrect leading line chars"); // no-return
}

// Function: sub_5012 @ 0x5012
void sub_5012(FILE *a0,unsigned int *a1,long a2,long a3) // ternary
{
  unsigned int v1;
  int v10; // stack - 0x6c
  unsigned int v11; // stack - 0x68
  unsigned int *v12; // stack - 0x60
  long v13; // stack - 0x58
  int v2;
  long v3;
  long v4;
  void *v5;
  unsigned long v6;
  char *v7; // rax
  char v8 [2]; // stack - 0x12
  unsigned int v9; // stack - 0x70
  
  v7 = (dat_1410d) ? "\t" : "  "; // branch-flip
  v12 = a1;
  do {
    if (!v12)
      return;
    v1 = *v12;
    if (v1 != 4) { // branch-flip
      if ((v1 < 4) || (2 < v1 - 5))
        sub_5fcf("internal error: invalid diff type passed to output"); // no-return
      v10 = *(int *)(a3 + (unsigned long)(*v12 - 5) * 4);
      v8[0] = (char)v10 + '1';
      v8[1] = 0;
      v11 = (unsigned int)(v10 == 0);
    }
    else {
      v8[0] = '\0';
      v11 = 3;
      v10 = 3;
    }
    fprintf(a0,"====%s\n",v8);
    v9 = 0;
    while ((int)v9 <= 2) {
      v2 = *(int *)(a2 + (long)(int)v9 * 4);
      v3 = *(long *)&v12[(long)v2 * 4 + 2];
      v4 = *(long *)&v12[(long)v2 * 4 + 4];
      fprintf(a0,"%d:",v9 + 1);
      if (v3 != v4) { // branch-flip
        if (v3 - v4 != 1) // branch-flip
          fprintf(a0,"%ld,%ldc\n",v3,v4);
        else {
          fprintf(a0,"%lda\n",v3 + -1);
        }
      }
      else {
        fprintf(a0,"%ldc\n",v3);
      }
      if ((v9 != v11) && (v3 <= v4)) {
        v13 = 0;
        do {
          fputs_unlocked(v7,a0);
          v5 = *(void **)(v13 * 8 + *(long *)&v12[((long)v2 + 6) * 2 + 2]);
          v6 = *(unsigned long *)(v13 * 8 + *(long *)&v12[((long)v2 + 10) * 2]);
          fwrite_unlocked(v5,1,v6,a0);
          v13 += 1;
        } while (v13 <= v4 - v3);
        if (*(char *)((long)v5 + (v6 - 1)) != '\n')
          fprintf(a0,"\n\\ %s\n",gettext("No newline at end of file"));
      }
      if (v10 != 1) // branch-flip
        v9 += 1;
      else {
        v9 = *(unsigned int *)((long)(int)v9 * 4 + 0x14010);
      }
    }
    v12 = *(unsigned int **)&v12[0x1a];
  } while( true );
}

// Function: sub_5369 @ 0x5369
char sub_5369(FILE *a0,long a1,int a2)
{
  char *v1;
  char v2; // stack - 0x19
  long v3; // stack - 0x18
  
  v2 = 0;
  for (v3 = 0; v3 <= *(long *)((long)a2 * 0x10 + a1 + 0x10) - *(long *)((long)a2 * 0x10 + a1 + 8); v3 = v3 + 1) {
    v1 = *(char **)(v3 * 8 + *(long *)(a1 + 8 + ((long)a2 + 6) * 8));
    if (*v1 == '.') {
      v2 = 1;
      fputc_unlocked(0x2e,a0);
    }
    fwrite_unlocked(v1,1,*(unsigned long *)(v3 * 8 + *(long *)(a1 + ((long)a2 + 10) * 8)),a0);
  }
  return v2;
}

// Function: sub_544f @ 0x544f
void sub_544f(FILE *a0,bool a1,long a2,long a3) // return-dupe x2
{
  fputs_unlocked(".\n",a0);
  if (!a1)
    return;
  if (a3 == 1) {
    fprintf(a0,"%lds/^\\.//\n",a2);
    return;
  }
  fprintf(a0,"%ld,%lds/^\\.//\n",a2,a3 + a2 + -1);
}

// Function: sub_54de @ 0x54de
char sub_54de(FILE *a0,long a1,int *a2,long a3,char *a4,char *a5,char *a6) // ternary
{
  int v1;
  int *v10; // stack - 0x30
  long v2;
  long v3;
  long v4;
  int v5;
  char *v6;
  bool v7; // stack - 0x37
  char v8; // stack - 0x36
  bool v9; // stack - 0x35
  
  v8 = 0;
  v10 = (int *)sub_5f95(a1);
  do {
    if (!v10) {
      if (dat_14111)
        fputs_unlocked("w\nq\n",a0);
      return v8;
    }
    v5 = (*v10 != 4) ? *(int *)(a3 + (unsigned long)(unsigned int)(*v10 - 5) * 4) + 5 : 4; // branch-flip
    switch(v5) { // branch-flip
      case 4:
        if (!dat_1410e) {
          v9 = dat_1410c;
label_55b7:
          v2 = *(long *)&v10[(long)*a2 * 4 + 2];
          v3 = *(long *)&v10[(long)*a2 * 4 + 4];
          if (v9) { // branch-flip
            v8 = 1;
            fprintf(a0,"%lda\n",v3);
            v7 = 0;
            if (v5 == 4) {
              if (dat_14110) {
                fprintf(a0,"||||||| %s\n",a5);
                v1 = a2[1];
                v7 = (bool)sub_5369(a0,v10,v1);
              }
              fputs_unlocked("=======\n",a0);
              v1 = a2[2];
              v7 = sub_5369(a0,v10,v1) || v7;
            }
            fprintf(a0,">>>>>>> %s\n",a6);
            sub_544f(a0,v7,v3 + 2,(*(long *)&v10[(long)a2[2] * 4 + 4] - *(long *)&v10[(long)a2[2] * 4 + 2]) + (*(long *)&v10[(long)a2[1] * 4 + 4] - *(long *)&v10[(long)a2[1] * 4 + 2]) + 3);
            if (v5 != 4) // branch-flip
              v6 = a5;
            else {
              v6 = a4;
            }
            fprintf(a0,"%lda\n<<<<<<< %s\n",v2 + -1,v6);
            v7 = 0;
            if (v5 == 6) {
              v5 = a2[1];
              v7 = (bool)sub_5369(a0,v10,v5);
              fputs_unlocked("=======\n",a0);
            }
            sub_544f(a0,v7,v2 + 1,(*(long *)&v10[(long)a2[1] * 4 + 4] - *(long *)&v10[(long)a2[1] * 4 + 2]) + 1);
          }
          else if (*(long *)&v10[(long)a2[2] * 4 + 4] - *(long *)&v10[(long)a2[2] * 4 + 2] != -1) { // branch-flip
            if (v3 - v2 != -1) { // branch-flip
              if (v3 != v2) // branch-flip
                fprintf(a0,"%ld,%ldc\n",v2,v3);
              else {
                fprintf(a0,"%ldc\n",v3);
              }
            }
            else {
              fprintf(a0,"%lda\n",v3);
            }
            v3 = *(long *)&v10[(long)a2[2] * 4 + 4];
            v4 = *(long *)&v10[(long)a2[2] * 4 + 2];
            v5 = a2[2];
            sub_544f(a0,(unsigned char)sub_5369(a0,v10,v5),v2,(v3 - v4) + 1);
          }
          else if (v2 != v3) // branch-flip
            fprintf(a0,"%ld,%ldd\n",v2,v3);
          else {
            fprintf(a0,"%ldd\n",v2);
          }
        }
        break;
      case 6:
        if (dat_14110 == '\x01') {
          v9 = 1;
          goto label_55b7;
        }
        break;
      case 7:
        if (!dat_1410f) {
          v9 = 0;
          goto label_55b7;
        }
      
    }
    v10 = *(int **)&v10[0x1a];
  } while( true );
}

// Function: sub_5a29 @ 0x5a29
char sub_5a29(FILE *a0,FILE *a1,int *a2,int *a3,long a4,char *a5,unsigned long a6,char *a7) // ternary
{
  int v1; // eax
  int v2; // eax
  char v3; // stack - 0x42
  bool v4; // stack - 0x41
  long v5; // stack - 0x38
  int *v6; // stack - 0x30
  long v7; // stack - 0x28
  char *v8; // stack - 0x20
  
  v3 = 0;
  v7 = 0;
  v6 = a2;
  do {
    if (!v6) {
      do {
        v1 = getc_unlocked(a0);
        if (v1 == -1) {
          v2 = ferror_unlocked(a0);
          if (feof_unlocked(a0) || v2)
            return v3;
        }
        putc_unlocked(v1,a1);
      } while( true );
    }
    v1 = (*v6 != 4) ? *(int *)(a4 + (unsigned long)(unsigned int)(*v6 - 5) * 4) + 5 : 4; // branch-flip
    v8 = "<<<<<<< %s\n";
    switch(0xfd54) { // branch-flip
      case 4:
        if (!dat_1410e) {
          v4 = dat_1410c;
          v8 = "||||||| %s\n";
label_5b18:
          v5 = (*(long *)&v6[2] - v7) + -1;
          v7 += v5;
          while (v5 = v5 + -1, 0 <= v5) {
            do {
              v2 = getc_unlocked(a0);
              if (v2 == -1) {
                if (ferror_unlocked(a0))
                  sub_600c(gettext("read failed")); // no-return
                if (feof_unlocked(a0))
                  sub_5fcf("input file shrank"); // no-return
              }
              putc_unlocked(v2,a1);
            } while (v2 != 10);
          }
          if (v4) {
            v3 = 1;
            if (v1 == 4) {
              fprintf(a1,"<<<<<<< %s\n",a5);
              for (v5 = 0; v5 <= *(long *)&v6[(long)*a3 * 4 + 4] - *(long *)&v6[(long)*a3 * 4 + 2]; v5 = v5 + 1) {
                fwrite_unlocked(*(void **)(v5 * 8 + *(long *)&v6[((long)*a3 + 6) * 2 + 2]),1,*(unsigned long *)(v5 * 8 + *(long *)&v6[((long)*a3 + 10) * 2]),a1);
              }
            }
            if (dat_14110) {
              fprintf(a1,v8,a6);
              for (v5 = 0; v5 <= *(long *)&v6[(long)a3[1] * 4 + 4] - *(long *)&v6[(long)a3[1] * 4 + 2]; v5 = v5 + 1) {
                fwrite_unlocked(*(void **)(v5 * 8 + *(long *)&v6[((long)a3[1] + 6) * 2 + 2]),1,*(unsigned long *)(v5 * 8 + *(long *)&v6[((long)a3[1] + 10) * 2]),a1);
              }
            }
            fputs_unlocked("=======\n",a1);
          }
          for (v5 = 0; v5 <= *(long *)&v6[(long)a3[2] * 4 + 4] - *(long *)&v6[(long)a3[2] * 4 + 2]; v5 = v5 + 1) {
            fwrite_unlocked(*(void **)(v5 * 8 + *(long *)&v6[((long)a3[2] + 6) * 2 + 2]),1,*(unsigned long *)(v5 * 8 + *(long *)&v6[((long)a3[2] + 10) * 2]),a1);
          }
          if (v4)
            fprintf(a1,">>>>>>> %s\n",a7);
          v5 = (*(long *)&v6[4] - *(long *)&v6[2]) + 1;
          v7 += v5;
          while (v5 = v5 + -1, 0 <= v5) {
            while (v1 = getc_unlocked(a0), v1 != 10) {
              if (v1 == -1) {
                if (ferror_unlocked(a0))
                  sub_600c(gettext("read failed")); // no-return
                if (feof_unlocked(a0)) {
                  if ((!v5) && (!*(long *)&v6[0x1a]))
                    return v3;
                  sub_5fcf("input file shrank"); // no-return
                }
              }
            }
          }
        }
        break;
      case 6:
        if (dat_14110 == '\x01') {
          v4 = 1;
          goto label_5b18;
        }
        break;
      case 7:
        if (!dat_1410f) {
          v4 = 0;
          goto label_5b18;
        }
      
    }
    v6 = *(int **)&v6[0x1a];
  } while( true );
}

// Function: sub_5f95 @ 0x5f95
long sub_5f95(long a0)
{
  long v1; // rbx
  long v2;
  
  v2 = 0;
  while (v1 = a0, v1) {
    a0 = *(long *)(v1 + 0x68);
    *(long *)(v1 + 0x68) = v2;
    v2 = v1;
  }
  return v2;
}

// Function: sub_5fcf @ 0x5fcf
void sub_5fcf(char *a0)
{
  error(2,0,"%s",gettext(a0)); // no-return
}

// Function: sub_600c @ 0x600c
void sub_600c(char *a0)
{
  error(2,*__errno_location(),"%s",a0); // no-return
}

// Function: sub_6042 @ 0x6042
void sub_6042(int a0)
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
  (*dat_24120)(a0);
  if (a0) // branch-flip
    v1 = dat_24128;
  else {
    v1 = dat_24130;
  }
  v2 = strlen(dat_24138);
  v3 = strlen(v1);
  if (0x1000 <= v3 + v2) { // branch-flip
    write(2,dat_24138,v2);
    write(2,": ",2);
    write(2,v1,v3);
    v5[0] = 10;
    v7 = (char *)0x1;
  }
  else {
    v4 = mempcpy(v5,dat_24138,v2);
    v7 = mempcpy(mempcpy(v4,": ",2),v1,v3);
    *v7 = 10;
    v7 = &v7[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v7);
  if (!a0)
    _exit(dat_14030); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_624b @ 0x624b
void sub_624b(void)
{
  return;
}

// Function: sub_6259 @ 0x6259
unsigned long sub_6259(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_6042(0xb); // no-return
}

// Function: sub_6285 @ 0x6285
void sub_6285(int a0)
{
  unsigned int v1; // eax
  
  if ((a0) && (!dat_24140))
    v1 = 0xb;
  else {
    v1 = 0;
  }
  sub_6042(v1); // no-return
}

// Function: sub_62bb @ 0x62bb
void * sub_62bb(void *a0) // return-dupe
{
  int v1; // eax
  void *v2; // rax
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = sub_624b;
  }
  dat_24120 = v2;
  v2 = (char *)gettext("program error");
  dat_24128 = (char *)v2;
  v2 = (char *)gettext("stack overflow");
  dat_24130 = (char *)v2;
  v2 = (unsigned long)sub_636e();
  dat_24138 = v2;
  v1 = sub_71e1(sub_6285,0x14120,0x10000);
  if (v1) {
    v2 = (int *)__errno_location();
    *(int *)v2 = 0x5f;
    v2 = (void *)0xffffffff;
    return v2;
  }
  v1 = sub_7131(sub_6259);
  dat_24140 = v1;
  v2 = NULL;
  return v2;
}

// Function: sub_636e @ 0x636e
unsigned long sub_636e(void)
{
  return __progname;
}

// Function: sub_637f @ 0x637f
void sub_637f(char *a0)
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
  dat_24148 = v2;
  program_invocation_name = v2;
}

// Function: sub_646a @ 0x646a
char sub_646a(char *a0,char *a1)
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
  
  v4 = (char *)sub_8113(a1,2);
  v11 = 0;
  v10 = a0;
  while( true ) {
    while( true ) {
      if ((!*v10) || (v5 = (char *)sub_a72c(v10,v4), !v5)) {
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
        sub_ae9a(v8);
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
      sub_ae9a(&v9);
      if (((char)v24 != '\x01') || (v24._4_4_))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_ae9a(v8);
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
    sub_ae9a(v8);
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
    sub_ae9a(v8);
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

// Function: sub_6940 @ 0x6940
char * sub_6940(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = gettext(a0);
  if (v1 != a0) {
    if (sub_646a(v1,a0)) // branch-flip
      a0 = v1;
    else {
      v2 = strlen(v1);
      v3 = (char *)sub_8e10(strlen(a0) + v2 + 4);
      sprintf(v3,"%s (%s)",v1,a0);
      a0 = v3;
    }
  }
  return a0;
}

// Function: sub_69e6 @ 0x69e6
char * sub_69e6(char *a0,char *a1)
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
  v2 = (char *)sub_9968();
  v6 = NULL;
  v7 = NULL;
  v9 = NULL;
  if (sub_9807(v2,"UTF-8")) { // branch-flip
    v6 = (char *)sub_9620(a1,"UTF-8",v2);
    v8 = v6;
    v3 = strlen(v2);
    v4 = (char *)sub_8e10(v3 + 0xb);
    memcpy(v4,v2,v3);
    memcpy(&v4[v3],"//TRANSLIT",0xb);
    v2 = (char *)sub_9620(a1,"UTF-8",v4);
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
  if (((sub_646a(v1,a0)) || ((v8 && (sub_646a(v1,v8))))) || ((v9 && (sub_646a(v1,v9))))) {
    if (v6)
      free(v6);
    if (v7)
      free(v7);
    v5 = v1;
  }
  else {
    v3 = strlen(v1);
    v5 = (char *)sub_8e10(strlen(v2) + v3 + 4);
    sprintf(v5,"%s (%s)",v1,v2);
    if (v6)
      free(v6);
    if (v7)
      free(v7);
  }
  return v5;
}

// Function: sub_6cc9 @ 0x6cc9
void sub_6cc9(void)
{
  return;
}

// Function: sub_6cd4 @ 0x6cd4
void sub_6cd4(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_807f(a0,v1) <= -1)
    return;
  dat_24150 = v2 + -1;
}

// Function: sub_6d30 @ 0x6d30
void sub_6d30(unsigned long a0,long a1,long a2) // return-dupe x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_6d30
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
  
  v1 = *(unsigned long *)(a1 + 0x10);
  if ((dat_24170) && (v3 = (*dat_24170)(v1,0), v3))
    return;
  if ((dat_24158) && (v2 = *(unsigned long *)(a2 + 0xa0), dat_24150)) {
    v3 = *__errno_location();
    v4 = sub_807f(dat_24150,&v6);
    *__errno_location() = v3;
    if (0 <= v4) {
      if (v6 <= v1) // branch-flip
        v7 = v1 <= (unsigned long)(v8 - 1U);
      else {
        v7 = (*v9)(v1,&v6) != 0;
      }
      if (v7) {
        v5 = ((dat_24160 <= v2) && (v2 <= dat_24168 + dat_24160));
        (*dat_24158)(v5,a2);
      }
    }
  }
  if ((dat_24170) && (v3 = (*dat_24170)(v1,1), v3))
    return;
  signal(0xb,0);
}

// Function: sub_6ed3 @ 0x6ed3
void sub_6ed3(int a0)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  undefined4 v3; // stack - 0x20
  
  v1 = sub_6d30;
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
  if (dat_24158)
    v3 = 0x8000004;
  sigaction(a0,(sigaction *)&v1,NULL);
}

// Function: sub_7131 @ 0x7131
unsigned long sub_7131(unsigned long a0)
{
  dat_24170 = a0;
  sub_6ed3(0xb);
  return 0;
}

// Function: sub_7164 @ 0x7164
void sub_7164(void) // return-dupe
{
  dat_24170 = 0;
  if (dat_24158) {
    dat_24170 = 0;
    return;
  }
  signal(0xb,0);
}

// Function: sub_71a0 @ 0x71a0
unsigned long sub_71a0(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_6cc9();
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_71e1 @ 0x71e1
unsigned long sub_71e1(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x28
  unsigned int v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  
  if ((!dat_24150) && (sub_6cd4(&v1), !dat_24150))
    return 0xffffffff;
  v2 = 0;
  dat_24158 = a0;
  dat_24160 = a1;
  dat_24168 = a2;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_6ed3(0xb);
  return 0;
}

// Function: sub_72b3 @ 0x72b3
void sub_72b3(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_24158 = 0;
  if (dat_24170) // branch-flip
    sub_6ed3(0xb);
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_7350 @ 0x7350
bool sub_7350(long a0,struct_1 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_7391 @ 0x7391
unsigned long sub_7391(struct_2 *a0,char *a1) // return-dupe x3, return-dupe
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
              if ((v4 <= -1) && (*__errno_location() == 4)) goto label_7640;
              if (v4 < 0) goto label_7692;
              if ((unsigned long)((long)v7 - a0->field_0x8) < (unsigned long)(long)(v4 + 0x1049)) break;
              if (!v4) {
                close(v5);
                return 0;
              }
              a0->field_0x8 = a0->field_0x8 + (long)v4;
            }
          }
          goto label_7540;
        }
label_7692:
        close(v5);
        goto label_769f;
      }
    }
    else {
label_7540:
      if (v6) { // branch-flip
        v7 = (char *)((long)v7 << 1);
        if (!v7) goto label_7692;
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
label_7640:
    if ((long)lseek(v5,0,0) <= -1) {
      close(v5);
      v5 = open(a1,0);
      if (v5 < 0) {
label_769f:
        if (!a0->field_0x20)
          return 0xffffffff;
        munmap((void *)a0->field_0x20,a0->field_0x28);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_76ce @ 0x76ce
unsigned long sub_76ce(struct_3 *a0) // early-return
{
  if (a0->field_0x0 == a0->field_0x8) {
    a0->field_0x10 = 1;
    return 0xffffffff;
  }
  return (unsigned long)*(unsigned char *)(a0->field_0x0 + a0->field_0x18);
}

// Function: sub_771a @ 0x771a
int sub_771a(struct_3 *a0)
{
  int v1; // eax
  
  v1 = sub_76ce(a0);
  if (0 <= v1)
    a0->field_0x0 = a0->field_0x0 + 1;
  return v1;
}

// Function: sub_7756 @ 0x7756
unsigned long sub_7756(struct_3 *a0,long *a1) // return-dupe
{
  int v1; // eax
  int v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = 0;
  v2 = 0;
  do {
    v1 = sub_76ce(a0);
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
    sub_771a(a0);
    v2 += 1;
  } while( true );
}

// Function: sub_7832 @ 0x7832
void sub_7832(long a0) // return-dupe
{
  if (!*(long *)(a0 + 0x20))
    return;
  munmap(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28));
}

// Function: sub_786d @ 0x786d
unsigned long sub_786d(struct_4 *a0) // return-dupe
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
  
  if ((int)sub_7391(v3,"/proc/self/maps") < 0)
    return 0xffffffff;
  v6 = v8;
  v7 = v9;
  do {
    while( true ) {
      if ((((int)sub_7756(v3,&v4) < 0) || (sub_771a(v3) != 0x2d)) || ((int)sub_7756(v3,&v5) < 0)) {
        sub_7832(v3); // return-dupe
        return 0;
      }
      do {
        v2 = sub_771a(v3);
        if ((v2 != -1) && (v2 != 10))
          v1 = 1;
        else {
          v1 = 0;
        }
      } while (v1);
      if ((v6 < v4) || ((unsigned long)(v5 - 1U) < v7 - 1)) break;
      if (((v4 < v6) && (v2 = sub_7ff2(a0,v4,v6), v2)) || ((v7 - 1 < (unsigned long)(v5 - 1U) && (v2 = sub_7ff2(a0,v7,v5), v2)))) {
        sub_7832(v3);
        return 0;
      }
    }
    v2 = sub_7ff2(a0,v4,v5);
  } while (!v2);
  sub_7832(v3);
  return 0;
}

// Function: sub_7a0e @ 0x7a0e
unsigned long sub_7a0e(struct_4 *a0) // return-dupe
{
  if (!sub_786d(a0))
    return 0;
  return 0xffffffff;
}

// Function: sub_7a41 @ 0x7a41
void sub_7a41(void)
{
  dat_24178 = (long)getpagesize();
}

// Function: sub_7a5a @ 0x7a5a
unsigned int sub_7a5a(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_24178,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_7ab1 @ 0x7ab1
unsigned long sub_7ab1(unsigned long a0)
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
    if (v5 / dat_24178 < v6)
      v6 = v5 / dat_24178;
    v1 = dat_24178 * v6;
    v3 = dat_24178 * v6;
    if (mincore((void *)(v5 - v3),v1,v4) <= -1) break;
    v5 -= dat_24178 * v6;
  }
  while (v6 != 1) {
    v1 = v6 + 1 >> 1;
    v2 = dat_24178 * v1;
    v3 = dat_24178 * v1;
    if (0 <= mincore((void *)(v5 - v3),v2,v4)) { // branch-flip
      v5 -= dat_24178 * v1;
      v6 >>= 1;
    }
    else {
      v6 = v1;
    }
  }
  return v5;
}

// Function: sub_7c65 @ 0x7c65
void * sub_7c65(long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [1032];
  void *v4; // stack - 0x440
  unsigned long v5; // stack - 0x438
  
  v5 = 0x400;
  v4 = (void *)(a0 + dat_24178);
  while( true ) {
    if (!v4)
      return NULL;
    if ((unsigned long)-(long)v4 / dat_24178 < v5)
      v5 = (unsigned long)-(long)v4 / dat_24178;
    v1 = dat_24178 * v5;
    if (mincore(v4,v1,v3) <= -1) break;
    v4 = (void *)((long)v4 + dat_24178 * v5);
  }
  while (v5 != 1) {
    v1 = v5 + 1 >> 1;
    v2 = dat_24178 * v1;
    if (0 <= mincore(v4,v2,v3)) { // branch-flip
      v4 = (void *)((long)v4 + dat_24178 * v1);
      v5 >>= 1;
    }
    else {
      v5 = v1;
    }
  }
  return v4;
}

// Function: sub_7dfd @ 0x7dfd
unsigned long sub_7dfd(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  void *v6; // stack - 0x20
  
  v1 = dat_24178 * (a0 / dat_24178);
  v2 = (dat_24178 * (a1 / dat_24178 + 1) - v1) / dat_24178;
  for (v4 = 1; v4 < v2; v4 = v4 << 1) {
  }
  do {
    v4 >>= 1;
    if (!v4)
      return 1;
    v3 = dat_24178 * v4;
    v6 = (void *)(v3 + v1);
    for (v5 = v4; v5 < v2; v5 = v5 + v4 * 2) {
      if (sub_7a5a(v6))
        return 0;
      v6 = (void *)((long)v6 + v3 * 2);
    }
  } while( true );
}

// Function: sub_7f0e @ 0x7f0e
unsigned long sub_7f0e(unsigned long a0,long *a1) // early-return
{
  long v1;
  unsigned long v2; // rax
  
  v2 = a0 * 2 - *a1;
  if (a0 < v2)
    return 0;
  v1 = *a1;
  return sub_7dfd(v2,v1 - 1);
}

// Function: sub_7f6b @ 0x7f6b
unsigned long sub_7f6b(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // rax
  
  if (!dat_24178)
    sub_7a41();
  v1 = dat_24178 * (a0 / dat_24178);
  *a1 = sub_7ab1(v1);
  a1[1] = sub_7c65(v1);
  a1[2] = sub_7f0e;
  return 0;
}

// Function: sub_7ff2 @ 0x7ff2
unsigned long sub_7ff2(struct_4 *a0,unsigned long a1,unsigned long a2) // return-dupe
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

// Function: sub_807f @ 0x807f
unsigned long sub_807f(unsigned long a0,unsigned long *a1) // early-return
{
  unsigned long v1; // stack - 0x38
  unsigned long *v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  int v4; // stack - 0x20
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_7a0e(&v1);
  if (!v4) {
    a1[2] = sub_7350;
    return 0;
  }
  return sub_7f6b(a0,a1);
}

// Function: sub_8113 @ 0x8113
unsigned char * sub_8113(char *a0,int a1)
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
        while (((v9 = 0, v10 < v3 && (sub_99b0(&v3), v12)) && (iswspace(v13)))) {
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
          sub_99b0(&v3);
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
  sub_93d4(); // no-return
}

// Function: sub_8472 @ 0x8472
void sub_8472(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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

// Function: sub_8a6e @ 0x8a6e
void sub_8a6e(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_8472(a0,a1,a2,a3,a4,v1);
}

// Function: sub_8ae1 @ 0x8ae1
void sub_8ae1(FILE *a0,char *a1,char *a2,char *a3,struct_10 *a4)
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
  sub_8472(a0,a1,a2,a3,v2,v3);
}

// Function: sub_8be7 @ 0x8be7
void sub_8be7(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_8ae1(a8,a9,a10,a11,&v3);
}

// Function: sub_8ccb @ 0x8ccb
void sub_8ccb(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-diffutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_8d74 @ 0x8d74
void sub_8d74(unsigned long a0,unsigned long a1)
{
  sub_8ed8(NULL,a0,a1);
}

// Function: sub_8d9f @ 0x8d9f
void sub_8d9f(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8ed8(a0,a1,a2);
}

// Function: sub_8dd0 @ 0x8dd0
void sub_8dd0(unsigned long a0)
{
  sub_8e10(a0);
}

// Function: sub_8dee @ 0x8dee
long sub_8dee(long a0)
{
  if (!a0)
    sub_93d4(); // no-return
  return a0;
}

// Function: sub_8e10 @ 0x8e10
void sub_8e10(unsigned long a0)
{
  sub_8dee(malloc(a0));
}

// Function: sub_8e36 @ 0x8e36
void sub_8e36(unsigned long a0)
{
  sub_8dee(sub_989b(a0));
}

// Function: sub_8e5c @ 0x8e5c
void * sub_8e5c(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_93d4(); // no-return
  return v1;
}

// Function: sub_8ea7 @ 0x8ea7
void sub_8ea7(void *a0,unsigned long a1)
{
  sub_8dee(sub_98b9(a0,a1));
}

// Function: sub_8ed8 @ 0x8ed8
void * sub_8ed8(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_93d4(); // no-return
  return v1;
}

// Function: sub_8f32 @ 0x8f32
void sub_8f32(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8dee(sub_9919(a0,a1,a2));
}

// Function: sub_8f6b @ 0x8f6b
void sub_8f6b(void *a0,unsigned long *a1)
{
  sub_8f99(a0,a1,1);
}

// Function: sub_8f99 @ 0x8f99
unsigned long sub_8f99(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_93d4(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_8ed8(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_9078 @ 0x9078
unsigned long sub_9078(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_93d4(); // no-return
  v3 = sub_8e5c(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_9234 @ 0x9234
void sub_9234(unsigned long a0)
{
  sub_927a(a0,1);
}

// Function: sub_9257 @ 0x9257
void sub_9257(unsigned long a0)
{
  sub_92ab(a0,1);
}

// Function: sub_927a @ 0x927a
void sub_927a(unsigned long a0,unsigned long a1)
{
  sub_8dee(calloc(a0,a1));
}

// Function: sub_92ab @ 0x92ab
void sub_92ab(unsigned long a0,unsigned long a1)
{
  sub_8dee(sub_98f0(a0,a1));
}

// Function: sub_92dc @ 0x92dc
void sub_92dc(void *a0,unsigned long a1)
{
  memcpy((void *)sub_8e10(a1),a0,a1);
}

// Function: sub_9314 @ 0x9314
void sub_9314(void *a0,unsigned long a1)
{
  memcpy((void *)sub_8e36(a1),a0,a1);
}

// Function: sub_9354 @ 0x9354
void sub_9354(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_8e36(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_93a3 @ 0x93a3
void sub_93a3(char *a0)
{
  sub_92dc(a0,strlen(a0) + 1);
}

// Function: sub_93d4 @ 0x93d4
void sub_93d4(void)
{
  error(dat_14030,0,"%s",gettext("memory exhausted")); // no-return
}

// Function: sub_9414 @ 0x9414
void sub_9414(char *a0,char *a1,FILE *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  if (freopen(a0,a1,a2))
    return;
  if (!a0) {
    if (a2 != stdin) { // branch-flip
      if (a2 != stdout) { // branch-flip
        if (a2 != stderr) // branch-flip
          a0 = gettext("unknown stream");
        else {
          a0 = gettext("stderr");
        }
      }
      else {
        a0 = gettext("stdout");
      }
    }
    else {
      a0 = gettext("stdin");
    }
  }
  v1 = sub_d099(1,a1);
  v2 = sub_d099(0,a0);
  v3 = gettext("failed to reopen %s with mode %s");
  error(dat_14030,*__errno_location(),v3,v2,v1); // no-return
}

// Function: sub_9533 @ 0x9533
void sub_9533(void)
{
  int v1; // eax
  
  v1 = sub_d0e9();
  if (v1)
    error(dat_14030,v1,gettext("standard file descriptors")); // no-return
}

// Function: sub_9579 @ 0x9579
int sub_9579(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_d1b5(a0,a1,a2,a3,a4);
  if (v1 <= -1) {
    if (*__errno_location() == 0xc)
      sub_93d4(); // no-return
  }
  return v1;
}

// Function: sub_95d7 @ 0x95d7
long sub_95d7(char *a0,unsigned long a1)
{
  long v1; // rax
  
  v1 = sub_d57e(a0,a1);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_93d4(); // no-return
  }
  return v1;
}

// Function: sub_9620 @ 0x9620
long sub_9620(char *a0,char *a1,char *a2)
{
  long v1; // rax
  
  v1 = sub_d8a1(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_93d4(); // no-return
  }
  return v1;
}

// Function: sub_9671 @ 0x9671
long sub_9671(int a0,void *a1,long a2) // early-return
{
  unsigned long v1; // rax
  unsigned long v2;
  long v3; // rax
  void *v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  
  v5 = 0x7fffffffffffffff;
  v4 = a1;
  do {
    v1 = (a2 + (long)a1) - (long)v4;
    v2 = v5;
    if (v1 <= v5)
      v2 = v1;
    v3 = read(a0,v4,v2);
    if (1 <= v3) // branch-flip
      v4 = (void *)((long)v4 + v3);
    else {
      if (!v3) break;
      if ((*__errno_location() != 0x16) || (v2 < 0x80000000))
        return -1;
      v5 = 0x7fffffff;
    }
  } while (v4 < (void *)(a2 + (long)a1));
  return (long)v4 - (long)a1;
}

// Function: sub_9740 @ 0x9740
void * sub_9740(void *a0,void *a1,void *a2) // early-return
{
  void *v1; // rax
  void *v2; // stack - 0x30
  void *v3; // stack - 0x28
  
  if (a0) { // branch-flip
    if (a1) { // branch-flip
      v2 = a0;
      v3 = a1;
      while (v1 = (void *)((unsigned long)v2 % (unsigned long)v3), v1) {
        v2 = v3;
        v3 = v1;
      }
      v1 = (void *)(((unsigned long)a0 / (unsigned long)v3) * (long)a1);
      if ((a2 >= v1) && ((unsigned long)a0 / (unsigned long)v3 == (unsigned long)v1 / (unsigned long)a1))
        a0 = v1;
    }
    else {
    }
  }
  else {
    if (!a1)
      return _DT_INIT;
    a0 = a1;
  }
  return a0;
}

// Function: sub_9807 @ 0x9807
int sub_9807(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_e185((unsigned char)*a0);
    v2 = sub_e185((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_9881 @ 0x9881
unsigned long sub_9881(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_989b @ 0x989b
void sub_989b(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_98b9 @ 0x98b9
void sub_98b9(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_98f0 @ 0x98f0
void sub_98f0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_9919 @ 0x9919
void sub_9919(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_9968 @ 0x9968
char * sub_9968(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_99b0 @ 0x99b0
void sub_99b0(struct_6 *a0)
{
  char v1;
  long v2;
  char *v3;
  char *v4;
  
  if (a0->field_0x14)
    return;
  if (!a0->field_0x8) {
    v1 = *a0->field_0x18;
    if (sub_e482((int)v1)) {
      a0->field_0x20 = 1;
      a0->field_0x2c = (int)*a0->field_0x18;
      a0->field_0x28 = 1;
      goto label_9bbd;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x9[3]))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    a0->field_0x8 = 1;
  }
  v2 = a0->field_0x0;
  v3 = a0->field_0x18;
  v4 = a0->field_0x18;
  a0->field_0x20 = sub_de82(&a0->field_0x2c,v4,v2 - (long)v3,(mbstate_t *)&a0->field_0x9[3]);
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
label_9bbd:
  a0->field_0x14 = 1;
  return;
}

// Function: sub_9bca @ 0x9bca
void sub_9bca(struct_1 *a0,long a1)
{
  a0->field_0x18 = a0->field_0x18 + a1;
  a0->field_0x0 = a0->field_0x0 + a1;
}

// Function: sub_9c09 @ 0x9c09
void sub_9c09(struct_11 *a0,struct_11 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  a0->field_0x8 = a1->field_0x8;
  if (a0->field_0x8) // branch-flip
    memcpy(&a0->field_0x9[3],&a1->field_0x9[3],8);
  else {
    memset(&a0->field_0x9[3],0,8);
  }
  a0->field_0x14 = a1->field_0x14;
  sub_e3dc((struct_8 *)&a0[1],(struct_9 *)&a1[1]);
}

// Function: sub_9caf @ 0x9caf
unsigned long sub_9caf(char *a0,long a1,unsigned long a2,unsigned long *a3) // return-dupe
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
      v2 = sub_e275(a2 << 3);
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
        goto label_9ea4;
      }
    }
    v11 += 1;
    *(unsigned long *)(v10 * 8 + v2) = v10 - v11;
label_9ea4:
  }
  *v3 = 0;
  v12 = 0;
  v13 = v8;
  v14 = v8;
  do {
    while( true ) {
      if (!*v14) goto label_9f6f;
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
label_9f6f:
  *(unsigned long *)((long)v5 + -8) = 0x9f7b;
  v16 = v2;
  sub_e346(v2);
  return 1;
}

// Function: sub_9f96 @ 0x9f96
unsigned long sub_9f96(void *a0,void *a1,long *a2)
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
  v22 = sub_e4c7(a1);
  if (0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0) && !SUB168(ZEXT816(0x38) * ZEXT816(v22),8)) { // branch-flip
    if (0xfa1 <= v22 * 0x38) { // branch-flip
      v3 = sub_e275(v22 * 0x38);
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
  *(unsigned long *)&v12[-8] = 0xa18e;
  v23 = v3;
  v24 = v3;
  v25 = v4;
  v26 = v4;
  memset(v6,0,8);
  while( true ) {
    v33 = 0;
    *(unsigned long *)&v12[-8] = 0xa1ef;
    sub_ae9a(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) break;
    *(unsigned long *)&v12[-8] = 0xa1c8;
    sub_e3dc((struct_8 *)(v24 + v17 * 0x30),&v7);
    v7 = (void *)(v34 + (long)v7);
    v17 += 1;
  }
  *(unsigned long *)(v26 + 8) = 1;
  v19 = 0;
  v18 = 2;
label_a3fa:
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
          *(unsigned long *)&v12[-8] = 0xa350;
          if (!memcmp(v1,v2,v3)) {
            v13 = 1;
            goto label_a363;
          }
        }
        v13 = 0;
      }
label_a363:
      if (v13) {
        v19 += 1;
        *(unsigned long *)(v18 * 8 + v26) = v18 - v19;
        goto label_a3f2;
      }
      if (!v19) goto label_a3a5;
      v19 -= *(long *)(v26 + v19 * 8);
    } while( true );
  }
  *v14 = 0;
  v20 = 0;
  v29 = v16;
  v10[0] = 0;
  *(unsigned long *)&v12[-8] = 0xa456;
  memset(v9,0,8);
  v28 = 0;
  v7 = v16;
  v8[0] = 0;
  *(unsigned long *)&v12[-8] = 0xa486;
  memset(v6,0,8);
  v33 = 0;
  do {
    *(unsigned long *)&v12[-8] = 0xa6dc;
    sub_ae9a(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) goto label_a702;
    if ((*(char *)(v24 + v20 * 0x30 + 0x10)) && (v35))
      v13 = *(int *)(v24 + v20 * 0x30 + 0x14) == v36;
    else {
      if (*(long *)(v24 + v20 * 0x30 + 8) == v34) {
        v3 = *(unsigned long *)(v24 + v20 * 0x30 + 8);
        v2 = *(void **)(v24 + v20 * 0x30);
        *(unsigned long *)&v12[-8] = 0xa577;
        if (!memcmp(v2,v7,v3)) {
          v13 = 1;
          goto label_a58a;
        }
      }
      v13 = 0;
    }
label_a58a:
    if (v13) {
      v20 += 1;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
      if (v20 == v22) {
        *v14 = (long)v29;
label_a702:
        *(unsigned long *)&v12[-8] = 0xa711;
        sub_e346(v23);
        return 1;
      }
    }
    else if (v20) { // branch-flip
      v21 = *(long *)(v26 + v20 * 8);
      v20 -= v21;
      for (; v21; v21 = v21 + -1) {
        *(unsigned long *)&v12[-8] = 0xa620;
        sub_ae9a(v10);
        if ((v31 != '\x01') || (v32))
          v13 = 1;
        else {
          v13 = 0;
        }
        if (!v13) {
          *(unsigned long *)&v12[-8] = 0xa647; // no-return
          abort();
        }
        v29 = (void *)(v30 + (long)v29);
        v28 = 0;
      }
    }
    else {
      *(unsigned long *)&v12[-8] = 0xa680;
      sub_ae9a(v10);
      if ((v31 != '\x01') || (v32))
        v13 = 1;
      else {
        v13 = 0;
      }
      if (!v13) {
        *(unsigned long *)&v12[-8] = 0xa6a7; // no-return
        abort();
      }
      v29 = (void *)(v30 + (long)v29);
      v28 = 0;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
    }
  } while( true );
label_a3a5:
  *(unsigned long *)(v18 * 8 + v26) = v18;
label_a3f2:
  v18 += 1;
  goto label_a3fa;
}

// Function: sub_a72c @ 0xa72c
char * sub_a72c(char *a0,char *a1) // early-return, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_a72c
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
    sub_ae9a(v7);
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
        sub_ae9a(v10);
        if ((v41 != '\x01') || (v42))
          v16 = 1;
        else {
          v16 = 0;
        }
        if (!v16)
          return NULL;
        if (((v2) && (10 <= v18)) && (v18 * 5 <= v19)) {
          for (v21 = v19 - v20; v21; v21 = v21 + -1) {
            sub_ae9a(v11);
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
          sub_ae9a(v11);
          if ((v36 != '\x01') || (v37))
            v16 = 1;
          else {
            v16 = 0;
          }
          if (!v16) {
            if (sub_9f96(a0,a1,&v12))
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
          sub_ae9a(v15);
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
            sub_ae9a(v15);
            if ((v51 != '\x01') || (v52))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16)
              return v39;
            sub_ae9a(v13);
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
          if (sub_9caf(v17,a1,v5,&v12))
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

// Function: sub_ae9a @ 0xae9a
void sub_ae9a(struct_7 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_e482((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_b0a0;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_d994(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_de82(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
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
label_b0a0:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_b0b1 @ 0xb0b1
void sub_b0b1(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_b0db @ 0xb0db
void sub_b0db(struct_12 *a0,struct_12 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_e3dc((struct_8 *)&a0[1].field_0x1[2],(struct_9 *)&a1[1].field_0x1[2]);
}

// Function: sub_b170 @ 0xb170
unsigned long sub_b170(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x24180;
  v2 = sub_92dc(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_b1bf @ 0xb1bf
unsigned int sub_b1bf(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x24180;
  return *a0;
}

// Function: sub_b1e3 @ 0xb1e3
void sub_b1e3(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x24180;
  *a0 = a1;
}

// Function: sub_b20e @ 0xb20e
unsigned int sub_b20e(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x24180;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_b29e @ 0xb29e
unsigned int sub_b29e(long a0,unsigned int a1)
{
  unsigned int v1;
  long v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = 0x24180;
  v1 = *(unsigned int *)(v2 + 4);
  *(unsigned int *)(v2 + 4) = a1;
  return v1;
}

// Function: sub_b2d8 @ 0xb2d8
void sub_b2d8(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x24180;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_b33a @ 0xb33a
unsigned long * sub_b33a(unsigned long *a0,unsigned int a1)
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

// Function: sub_b3ff @ 0xb3ff
char * sub_b3ff(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_9968();
  if (!sub_9807(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_9807(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x1047e;
  return (char *)0x10482;
}

// Function: sub_b4c4 @ 0xb4c4
unsigned long sub_b4c4(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_b57e:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_b6d2:
      v14 = 1;
      goto label_b6d6;
    case 2:
label_b6e5:
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
      goto label_b6d2;
    case 4:
label_b6d6:
      if (!v14)
        v2 = 1;
      goto label_b6e5;
    case 5:
label_b5c0:
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
      goto label_b5c0;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_b3ff("`",v19);
        v17 = (char *)sub_b3ff("\'",v19);
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
label_c384:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_c3bc;
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
      if (v14) goto label_c500;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_bca3:
    if (!v24) {
label_bcae:
      v16 = 1;
label_bcb2:
      if (v19 != 2) goto label_c1c9;
label_ba8b:
      if (v14) goto label_c500;
    }
    goto label_c1c9;
  }
  if (0x7e < v22) {
label_bd8f:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_be09:
      v31 = sub_de82(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_beb7;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_bf66;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_bf66:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_c1c9;
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
        if (v14) goto label_c500;
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
    goto label_c2ef;
  }
  if (v22 == 0x7d) {
label_bc71:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_c1c9;
    goto label_bca3;
  }
  if (0x7d < v22) goto label_bd8f;
  if (v22 == 0x7c) goto label_bcb2;
  if (0x7c < v22) goto label_bd8f;
  if (v22 == 0x7b) goto label_bc71;
  if (0x7b < v22) goto label_bd8f;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_bd8f;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_bd86;
    if (v11 & 0xa4000000) goto label_bcb2;
    if (!(v11 & 0x8000000)) goto label_bd8f;
    if (v19 == 2) {
      if (!v14) goto label_c2ef;
      goto label_c500;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_bc47;
    goto label_c2ef;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_c1c9;
        goto label_c37f;
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
        goto label_c1c9;
      }
      goto label_c500;
    default:
      goto label_bd8f;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_bc47;
    case 10:
      v23 = 0x6e;
      goto label_bc47;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_bc47:
      if ((v19 == 2) && (v14)) goto label_c500;
      break;
    case 0x20:
      goto label_bcae;
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
      goto label_bcb2;
    case 0x23:
      goto label_bca3;
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
label_bd86:
      v16 = 1;
      goto label_c1c9;
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
          goto label_c1c9;
        }
        goto label_c500;
      }
      goto label_c1c9;
    case 0x3f:
      if (v19 == 2) goto label_ba8b;
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
          goto label_c1c9;
        }
        goto label_c500;
      }
      goto label_c1c9;
    
  }
  if (v2) {
    v22 = v23;
label_c234:
    if (v14) goto label_c500;
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
label_c1c9:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_c234;
  }
label_c2ef:
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
label_c37f:
  v24 += 1;
  goto label_c384;
label_beb7:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_c500;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_bf66;
  goto label_be09;
label_c3bc:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_c500:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_b4c4(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_b4c4(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_b57e;
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

// Function: sub_c585 @ 0xc585
unsigned long sub_c585(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x24180;
  v1 = *__errno_location();
  v2 = sub_b4c4(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_c62d @ 0xc62d
void sub_c62d(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_c663(a0,a1,NULL,a2);
}

// Function: sub_c663 @ 0xc663
long sub_c663(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x24180;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_b4c4(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_8dd0(v3 + 1U);
  sub_b4c4(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_c78f @ 0xc78f
void sub_c78f(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_140b0;
  for (v2 = 1; v2 < dat_14040; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x241c0) {
    free(*(void **)((long)v1 + 8));
    dat_140a0 = 0x100;
    dat_140a8 = 0x241c0;
  }
  if (v1 == (void *)0x140a0) {
    dat_14040 = 1;
    return;
  }
  free(v1);
  dat_14040 = 1;
  dat_140b0 = (void *)0x140a0;
}

// Function: sub_c84e @ 0xc84e
void * sub_c84e(int a0,char *a1,unsigned long a2,struct_5 *a3)
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
  v6 = dat_140b0;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_14040 <= a0) {
      v5 = dat_140b0 == (unsigned long *)0x140a0;
      v4 = (long)dat_14040;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_140b0;
      }
      dat_140b0 = (unsigned long *)sub_9078(v3,&v4,(long)((a0 - dat_14040) + 1),0x7fffffff,0x10);
      v6 = dat_140b0;
      if (v5) {
        *dat_140b0 = dat_140a0;
        dat_140b0[1] = dat_140a8;
      }
      memset(&dat_140b0[(long)dat_14040 * 2],0,(v4 - dat_14040) * 0x10);
      dat_14040 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_b4c4(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x241c0)
        free(v7);
      v7 = (void *)sub_8dd0(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_b4c4(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_cafd @ 0xcafd
void sub_cafd(unsigned int a0,char *a1)
{
  sub_c84e(a0,a1,0xffffffffffffffff,(struct_5 *)0x24180);
}

// Function: sub_cb31 @ 0xcb31
void sub_cb31(unsigned int a0,char *a1,unsigned long a2)
{
  sub_c84e(a0,a1,a2,(struct_5 *)0x24180);
}

// Function: sub_cb63 @ 0xcb63
void sub_cb63(char *a0)
{
  sub_cafd(0,a0);
}

// Function: sub_cb86 @ 0xcb86
void sub_cb86(char *a0,unsigned long a1)
{
  sub_cb31(0,a0,a1);
}

// Function: sub_cbb1 @ 0xcbb1
void sub_cbb1(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_b33a(v1,a1);
  sub_c84e(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_cc19 @ 0xcc19
void sub_cc19(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_b33a(v1,a1);
  sub_c84e(a0,a2,a3,v1);
}

// Function: sub_cc7f @ 0xcc7f
void sub_cc7f(unsigned int a0,char *a1)
{
  sub_cbb1(0,a0,a1);
}

// Function: sub_cca7 @ 0xcca7
void sub_cca7(unsigned int a0,char *a1,unsigned long a2)
{
  sub_cc19(0,a0,a1,a2);
}

// Function: sub_ccd7 @ 0xccd7
void sub_ccd7(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_24180;
  v2 = dat_24188;
  v3 = dat_24190;
  v4 = dat_24198;
  v5 = dat_241a0;
  v6 = dat_241a8;
  v7 = dat_241b0;
  sub_b20e(&v1,(int)a2,1);
  sub_c84e(0,a0,a1,&v1);
}

// Function: sub_cd92 @ 0xcd92
void sub_cd92(char *a0,char a1)
{
  sub_ccd7(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_cdc0 @ 0xcdc0
void sub_cdc0(char *a0)
{
  sub_cd92(a0,0x3a);
}

// Function: sub_cde3 @ 0xcde3
void sub_cde3(char *a0,unsigned long a1)
{
  sub_ccd7(a0,a1,0x3a);
}

// Function: sub_ce11 @ 0xce11
void sub_ce11(unsigned int a0,unsigned int a1,char *a2)
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
  sub_b33a(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_b20e(&v2,0x3a,1);
  sub_c84e(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_ced3 @ 0xced3
void sub_ced3(unsigned int a0,long a1,long a2,char *a3)
{
  sub_cf0d(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_cf0d @ 0xcf0d
void sub_cf0d(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_24180;
  v2 = dat_24188;
  v3 = dat_24190;
  v4 = dat_24198;
  v5 = dat_241a0;
  v6 = dat_241a8;
  v7 = dat_241b0;
  sub_b2d8(&v1,a1,a2);
  sub_c84e(a0,a3,a4,&v1);
}

// Function: sub_cfcb @ 0xcfcb
void sub_cfcb(long a0,long a1,char *a2)
{
  sub_ced3(0,a0,a1,a2);
}

// Function: sub_cffe @ 0xcffe
void sub_cffe(long a0,long a1,char *a2,unsigned long a3)
{
  sub_cf0d(0,a0,a1,a2,a3);
}

// Function: sub_d03c @ 0xd03c
void sub_d03c(unsigned int a0,char *a1,unsigned long a2)
{
  sub_c84e(a0,a1,a2,(struct_5 *)0x14060);
}

// Function: sub_d06e @ 0xd06e
void sub_d06e(char *a0,unsigned long a1)
{
  sub_d03c(0,a0,a1);
}

// Function: sub_d099 @ 0xd099
void sub_d099(unsigned int a0,char *a1)
{
  sub_d03c(a0,a1,0xffffffffffffffff);
}

// Function: sub_d0c6 @ 0xd0c6
void sub_d0c6(char *a0)
{
  sub_d099(0,a0);
}

// Function: sub_d0e9 @ 0xd0e9
int sub_d0e9(void)
{
  int v1; // eax
  int v2; // stack - 0x18
  
  v2 = 0;
  do {
    if (3 <= v2)
      return 0;
    if ((int)sub_d9dc(v2,1) <= -1) {
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

// Function: sub_d1b5 @ 0xd1b5
unsigned long sub_d1b5(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe x2, return-dupe
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
    if (!v4) goto label_d4c5;
    v11 = iconv(a2,&v5,&v4,&v3,&v2);
  } while (v11 != -1);
  if (*__errno_location() == 0x16) {
label_d4c5:
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

// Function: sub_d57e @ 0xd57e
char * sub_d57e(char *a0,unsigned long a1) // return-dupe
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

// Function: sub_d8a1 @ 0xd8a1
char * sub_d8a1(char *a0,char *a1,char *a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  
  if ((*a0) && (sub_9807(a1,a2))) {
    v3 = iconv_open(a2,a1);
    if (v3 == -1)
      return NULL;
    v2 = (char *)sub_d57e(a0,v3);
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

// Function: sub_d994 @ 0xd994
unsigned long sub_d994(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_d9dc @ 0xd9dc
int sub_d9dc(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_dd4e(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_dd7f(a0,(unsigned int)v2);
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

// Function: sub_dd4e @ 0xdd4e
int sub_dd4e(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_dd7f @ 0xdd7f
int sub_dd7f(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_242c0) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_242c0 = 1;
    else {
      v3 = sub_dd4e(a0,a1);
      if (0 <= v3)
        dat_242c0 = -1;
    }
  }
  else {
    v3 = sub_dd4e(a0,a1);
  }
  if ((0 <= v3) && (dat_242c0 == -1)) {
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

// Function: sub_de82 @ 0xde82
unsigned long sub_de82(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_e1cb(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_df2c @ 0xdf2c
unsigned long sub_df2c(int a0) // return-dupe
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

// Function: sub_df6a @ 0xdf6a
unsigned long sub_df6a(int a0) // return-dupe
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

// Function: sub_df9c @ 0xdf9c
bool sub_df9c(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_dfbd @ 0xdfbd
unsigned int sub_dfbd(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_dfe5 @ 0xdfe5
unsigned long sub_dfe5(int a0) // return-dupe
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

// Function: sub_e012 @ 0xe012
bool sub_e012(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_e036 @ 0xe036
bool sub_e036(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_e05a @ 0xe05a
bool sub_e05a(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_e07e @ 0xe07e
bool sub_e07e(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_e0a2 @ 0xe0a2
unsigned long sub_e0a2(int a0) // return-dupe, return-dupe x3
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

// Function: sub_e0ec @ 0xe0ec
unsigned long sub_e0ec(int a0) // return-dupe
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

// Function: sub_e119 @ 0xe119
bool sub_e119(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_e13d @ 0xe13d
unsigned long sub_e13d(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_e185 @ 0xe185
int sub_e185(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_e1a8 @ 0xe1a8
int sub_e1a8(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_e1cb @ 0xe1cb
unsigned int sub_e1cb(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_e677(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_e275 @ 0xe275
long sub_e275(unsigned long a0) // early-return
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

// Function: sub_e346 @ 0xe346
void sub_e346(unsigned long a0) // return-dupe
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1)));
}

// Function: sub_e3a1 @ 0xe3a1
unsigned int sub_e3a1(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_e3dc @ 0xe3dc
void sub_e3dc(struct_8 *a0,struct_9 *a1) // return-dupe
{
  if ((struct_9 *)a1->field_0x0 != &a1[1]) // branch-flip
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

// Function: sub_e482 @ 0xe482
bool sub_e482(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0x105e0) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_e4c7 @ 0xe4c7
char * sub_e4c7(char *a0)
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
      sub_ae9a(v4);
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

// Function: sub_e597 @ 0xe597
char * sub_e597(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_e5bf @ 0xe5bf
unsigned long sub_e5bf(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_e597(a0);
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

// Function: sub_e677 @ 0xe677
void sub_e677(unsigned int a0,char *a1,unsigned long a2)
{
  sub_e5bf(a0,a1,a2);
}

// Function: sub_e6a5 @ 0xe6a5
void sub_e6a5(unsigned int a0)
{
  sub_e597(a0);
}

// Function: _DT_FINI @ 0xe6c0
void _DT_FINI(void)
{
  return;
}
