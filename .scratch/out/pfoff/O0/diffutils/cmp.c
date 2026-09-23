// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_ffd0)
    return;
  (*dat_ffd0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_fd88)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2490
void __cxa_finalize(void)
{
  (*dat_ffe0)(); // jump-as-call
}

// Function: raise @ 0x24a0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_fd90)(); // jump-as-call
  return v1;
}

// Function: free @ 0x24b0
void free(void *a0)
{
  (*dat_fd98)(); // jump-as-call
}

// Function: strtoimax @ 0x24c0
void strtoimax(void)
{
  (*dat_fda0)(); // jump-as-call
}

// Function: abort @ 0x24d0
void abort(void)
{
  (*dat_fda8)(); // jump-as-call
}

// Function: __errno_location @ 0x24e0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_fdb0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x24f0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_fdb8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x2500
void _exit(int a0)
{
  (*dat_fdc0)(); // jump-as-call
}

// Function: iconv @ 0x2510
void iconv(void)
{
  (*dat_fdc8)(); // jump-as-call
}

// Function: sigaction @ 0x2520
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_fdd0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x2530
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_fdd8)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x2540
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fde0)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x2550
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_fde8)(); // jump-as-call
  return v1;
}

// Function: write @ 0x2560
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_fdf0)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2570
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fdf8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2580
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_fe00)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2590
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fe08)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x25a0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fe10)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x25b0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fe18)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x25c0
void __stack_chk_fail(void)
{
  (*dat_fe20)(); // jump-as-call
}

// Function: mmap @ 0x25d0
void mmap(void)
{
  (*dat_fe28)(); // jump-as-call
}

// Function: getopt_long @ 0x25e0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_fe30)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x25f0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fe38)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x2600
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fe40)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2610
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fe48)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x2620
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_fe50)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x2630
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fe58)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2640
void lseek(void)
{
  (*dat_fe60)(); // jump-as-call
}

// Function: __assert_fail @ 0x2650
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_fe68)(); // jump-as-call
}

// Function: fputs @ 0x2660
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_fe70)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x2670
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fe78)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x2680
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fe80)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2690
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_fe88)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x26a0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fe90)(); // jump-as-call
  return v1;
}

// Function: read @ 0x26b0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_fe98)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x26c0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_fea0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x26d0
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_fea8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x26e0
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_feb0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x26f0
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_feb8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2700
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fec0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2710
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_fec8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x2720
void signal(void)
{
  (*dat_fed0)(); // jump-as-call
}

// Function: fprintf @ 0x2730
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_fed8)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x2740
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_fee0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2750
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_fee8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2760
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fef0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x2770
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_fef8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x2780
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ff00)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x2790
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_ff08)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x27a0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff10)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x27b0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ff18)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x27c0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff20)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x27d0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_ff28)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x27e0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ff30)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x27f0
void iconv_close(void)
{
  (*dat_ff38)(); // jump-as-call
}

// Function: iswspace @ 0x2800
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_ff40)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x2810
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff48)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x2820
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff50)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2830
void error(int a0,int a1,char *a2,...)
{
  (*dat_ff58)(); // jump-as-call
}

// Function: open @ 0x2840
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ff60)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x2850
void perror(char *a0)
{
  (*dat_ff68)(); // jump-as-call
}

// Function: getpagesize @ 0x2860
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_ff70)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x2870
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ff78)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x2880
void exit(int a0)
{
  (*dat_ff80)(); // jump-as-call
}

// Function: strdup @ 0x2890
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ff88)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x28a0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_ff90)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x28b0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ff98)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x28c0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_ffa0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x28d0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_ffa8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x28e0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ffb0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x28f0
void iconv_open(void)
{
  (*dat_ffb8)(); // jump-as-call
}

// Function: sub_2900 @ 0x2900
void sub_2900(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_ffc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_2930 @ 0x2930
void sub_2930(void)
{
  return;
}

// Function: _FINI_0 @ 0x29a0
void _FINI_0(void)
{
  if (!dat_10088) {
    if (dat_ffe0)
      __cxa_finalize(dat_10008);
    sub_2930();
    dat_10088 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x29e0
void _INIT_0(void)
{
  return;
}

// Function: sub_29e9 @ 0x29e9
void sub_29e9(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,gettext(a0),a1);
  v1 = dat_20268;
  error(2,0,gettext("Try \'%s --help\' for more information."),v1);
}

// Function: sub_2a5f @ 0x2a5f
void sub_2a5f(int a0,long *a1,char a2) // return-dupe
{
  int v1; // eax
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  v3 = *a1;
  v1 = sub_7e2e(v3,a1,0,&v2,"kKMGTPEZY0");
  if (((!v1) || ((v1 == 2 && (a2 == *(char *)*a1)))) && (0 <= v2)) {
    if (v2 <= *(long *)((long)a0 * 8 + 0x10200))
      return;
    *(long *)((long)a0 * 8 + 0x10200) = v2;
    return;
  }
  sub_29e9("invalid --ignore-initial value \'%s\'",v3); // no-return
}

// Function: sub_2b46 @ 0x2b46
void sub_2b46(int a0)
{
  if ((dat_10210) && (a0 != dat_10210))
    sub_29e9("options -l and -s are incompatible",0); // no-return
  dat_10210 = a0;
}

// Function: sub_2b8a @ 0x2b8a
void sub_2b8a(void)
{
  char *v1; // rax
  
  if (ferror_unlocked(stdout)) {
    error(2,0,"%s",gettext("write failed"));
    return;
  }
  if (fclose(stdout)) {
    v1 = gettext("standard output");
    error(2,*__errno_location(),"%s",v1);
    return;
  }
}

// Function: sub_2c28 @ 0x2c28
void sub_2c28(void)
{
  FILE *v1;
  char *v2;
  char *v3; // rax
  long *v4; // stack - 0x20
  
  v2 = dat_20268;
  printf(gettext("Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n"),v2);
  printf("%s\n",gettext("Compare two files byte by byte."));
  printf("\n%s\n\n",gettext("The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default)."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  for (v4 = (long *)0xfb40; *v4; v4 = &v4[1]) {
    v2 = (char *)*v4;
    printf("  %s\n",gettext(v2));
  }
  v2 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  v3 = gettext("If a FILE is \'-\' or missing, read standard input.");
  printf("\n%s\n\n%s\n%s\n",gettext("SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y."),v3,v2);
  sub_74c0();
}

// Function: main @ 0x2d6d
int main(int argc,char **argv,char **envp)
{
  int v1;
  long v10; // rax
  char *v11; // stack - 0x168
  stat v12; // stack - 0x148
  stat v13; // stack - 0xb8
  int v14; // stack - 0x17c
  int v15; // stack - 0x178
  int v16; // stack - 0x174
  long v17; // stack - 0x160
  long v18; // stack - 0x158
  unsigned long v19; // stack - 0x150
  char *v2;
  bool v3;
  long v4;
  int v5;
  int v6; // eax
  int v7; // eax
  unsigned long v8; // rax
  long v9;
  
  dat_10020 = 2;
  sub_4a4b(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_479f(NULL);
  sub_7c09();
  while (v5 = getopt_long(argc,argv,"bci:ln:sv",(option *)0xfa00,NULL), v5 != -1) {
    switch(v5) {
      case 0x62:
      case 99:
        dat_10214 = 1;
        break;
      default:
        sub_29e9(NULL,0); // no-return
      case 0x69:
        sub_2a5f(0,&optarg,0x3a);
        v2 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v2;
          if (dat_10208 < dat_10200)
            dat_10208 = dat_10200;
        }
        else {
          optarg = v2;
          sub_2a5f(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_2b46(1);
        break;
      case 0x6e:
        v5 = sub_7e2e(optarg,NULL,0,&v11,"kKMGTPEZY0");
        if ((v5) || ((long)v11 <= -1))
          sub_29e9("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_10010 < 0) || ((long)v11 <= (long)dat_10010))
          dat_10010 = v11;
        break;
      case 0x73:
        sub_2b46(3);
        break;
      case 0x76:
        v8 = sub_500c("David MacKenzie");
        sub_73dc(stdout,"cmp","GNU diffutils",dat_10018,sub_50b2("Torbjorn Granlund",0xc60c),v8,0);
        sub_2b8a();
        return 0;
      case 0x80:
        sub_2c28();
        sub_2b8a();
        return 0;
      
    }
  }
  if (argc == optind)
    sub_29e9("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  v5 = optind + 1;
  dat_100a0 = argv[optind];
  if (argc <= v5) { // branch-flip
    dat_100a8 = "-";
    optind = v5;
  }
  else {
    optind += 2;
    dat_100a8 = argv[v5];
  }
  v14 = 0;
  while ((v14 <= 1 && (optind < argc))) {
    v5 = optind + 1;
    v11 = argv[optind];
    optind = v5;
    sub_2a5f(v14,&v11,0);
    v14 += 1;
  }
  if (optind < argc)
    sub_29e9("extra operand \'%s\'",argv[optind]); // no-return
  for (v15 = 0; v15 <= 1; v15 = v15 + 1) {
    if (((v15) && (dat_10200 == dat_10208)) && (!strcmp(dat_100a0,dat_100a8)))
      return 0;
    v2 = *(char **)((long)v15 * 8 + 0x100a0);
    if (strcmp(v2,"-")) { // branch-flip
      v2 = *(char **)((long)v15 * 8 + 0x100a0);
      *(int *)((long)v15 * 4 + 0x100b0) = open(v2,0,0);
    }
    else {
      *(unsigned int *)((long)v15 * 4 + 0x100b0) = 0;
    }
    if ((*(int *)((long)v15 * 4 + 0x100b0) < 0) || (v5 = *(int *)((long)v15 * 4 + 0x100b0), fstat(v5,(stat *)((long)v15 * 0x90 + 0x100c0)))) {
      if ((*(int *)((long)v15 * 4 + 0x100b0) <= -1) && (dat_10210 == 3))
        exit(2); // no-return
      v2 = *(char **)((long)v15 * 8 + 0x100a0);
      error(2,*__errno_location(),"%s",v2);
      return v6;
    }
  }
  if (((dat_100c8 != dat_10158) || (dat_100c0 != dat_10150)) && (((((dat_100d8 & 0xf000) != 0x6000 || ((dat_10168 & 0xf000) != 0x6000)) && (((dat_100d8 & 0xf000) != 0x2000 || ((dat_10168 & 0xf000) != 0x2000)))) || (dat_100e8 != dat_10178))))
    v3 = 0;
  else {
    v3 = 1;
  }
  if ((((((v3) && (dat_100d8 == dat_10168)) && (dat_100d0 == dat_10160)) && ((dat_100dc == dat_1016c && (dat_100e0 == dat_10170)))) && (dat_100f0 == dat_10180)) && ((dat_10118 == dat_101a8 && (dat_10128 == dat_101b8)))) {
    v9 = sub_447f(0);
    if (v9 == sub_447f(1))
      return 0;
  }
  if (((dat_10210 != 3) && (!fstat(1,&v12))) && (!stat("/dev/null",&v13))) {
    if (((v12._8_8_ != v13._8_8_) || (v12._0_8_ != v13._0_8_)) && (((((v12._24_4_ & 0xf000) != 0x6000 || ((v13._24_4_ & 0xf000) != 0x6000)) && (((v12._24_4_ & 0xf000) != 0x2000 || ((v13._24_4_ & 0xf000) != 0x2000)))) || (v12._40_8_ != v13._40_8_))))
      v3 = 0;
    else {
      v3 = 1;
    }
    if (v3)
      dat_10210 = 2;
  }
  v9 = dat_100f0;
  if (((dat_10210 == 3) && ((dat_100d8 & 0xf000) == 0x8000)) && ((dat_10168 & 0xf000) == 0x8000)) {
    v10 = sub_447f(0);
    v4 = dat_10180;
    v17 = v9 - v10;
    v18 = v4 - sub_447f(1);
    if (v17 <= -1)
      v17 = 0;
    if (v18 <= -1)
      v18 = 0;
    if (v17 != v18) {
      if (0 <= (long)dat_10010) {
        v9 = v17;
        if (v18 < v17)
          v9 = v18;
        if ((long)dat_10010 <= v9) goto label_368d;
      }
      exit(1); // no-return
    }
  }
label_368d:
  dat_101f0 = sub_8307(dat_100f8,dat_10188,(void *)0x7ffffffffffffff7);
  v19 = dat_101f0 + 0xfU >> 3;
  dat_101e0 = sub_7605(v19 << 4);
  dat_101e8 = v19 * 8 + dat_101e0;
  v5 = sub_37d2();
  v16 = 0;
  while( true ) {
    if (2 <= v16) {
      if ((v5) && (dat_10210 <= 1))
        sub_2b8a();
      exit(v5); // no-return
    }
    v1 = *(int *)((long)v16 * 4 + 0x100b0);
    if (close(v1)) break;
    v16 += 1;
  }
  v2 = *(char **)((long)v16 * 8 + 0x100a0);
  error(2,*__errno_location(),"%s",v2);
  return v7;
}

// Function: sub_37d2 @ 0x37d2
unsigned long sub_37d2(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_37d2
{
  unsigned char v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  char *v13;
  char *v14; // rax
  unsigned long v15; // rax
  char *v16;
  char v17 [32];
  char v18 [24];
  char v19 [5];
  unsigned char v2;
  char v20 [5];
  bool v21; // stack - 0x14a
  int v22; // stack - 0x144
  int v23; // stack - 0x140
  int v24; // stack - 0x13c
  long v25; // stack - 0x138
  long v26; // stack - 0x130
  unsigned long v27; // stack - 0x128
  unsigned long v28; // stack - 0x120
  unsigned long v29; // stack - 0x118
  long v3;
  unsigned long v30; // stack - 0x110
  unsigned long v31; // stack - 0x108
  unsigned long v32; // stack - 0xd8
  unsigned long v33; // stack - 0xd0
  char *v4;
  void *v5;
  void *v6;
  bool v7; // al
  unsigned long v8;
  unsigned long v9; // rax
  
  v6 = dat_101e8;
  v5 = dat_101e0;
  v21 = 1;
  v25 = 1;
  v26 = 1;
  v27 = dat_10010;
  v22 = 0;
  if (dat_10210 == 1) {
    if (0 <= (long)dat_10010) // branch-flip
      v29 = dat_10010;
    else {
      v29 = 0x7fffffffffffffff;
    }
    for (v23 = 0; v23 <= 1; v23 = v23 + 1) {
      if ((*(unsigned int *)((long)v23 * 0x90 + 0x100d8) & 0xf000) == 0x8000) {
        v3 = *(long *)((long)v23 * 0x90 + 0x100f0);
        v8 = v3 - sub_447f(v23);
        if ((long)v8 < (long)v29)
          v29 = v8;
      }
    }
    v24 = 1;
    while (v29 = (long)v29 / 10, v29) {
      v24 += 1;
    }
  }
  v23 = 0;
  do {
    if (2 <= v23) {
      while( true ) {
        v31 = dat_101f0;
        if (0 <= (long)v27) {
          if (v27 < dat_101f0)
            v31 = v27;
          v27 -= v31;
        }
        v32 = sub_8238(dat_100b0,v5,v31);
        v13 = dat_100a0;
        if (v32 == 0xffffffffffffffff) {
          error(2,*__errno_location(),"%s",v13);
          return v11;
        }
        v33 = sub_8238(dat_100b4,v6,v31);
        v13 = dat_100a8;
        if (v33 == 0xffffffffffffffff) {
          error(2,*__errno_location(),"%s",v13);
          return v12;
        }
        v8 = v33;
        if (v32 <= v33)
          v8 = v32;
        if (memcmp(v5,v6,v8)) { // branch-flip
          if (v33 <= v32)
            *(char *)(v32 + (long)v6) = 0x55;
          if (v32 <= v33)
            *(char *)(v33 + (long)v5) = 0x79;
          *(unsigned char *)(v32 + (long)v5) = ~*(unsigned char *)(v32 + (long)v6);
          *(unsigned char *)(v33 + (long)v6) = ~*(unsigned char *)(v33 + (long)v5);
          memset((void *)((long)v5 + v32 + 1),0,(unsigned long)(~(unsigned int)v32 & 7));
          memset((void *)((long)v6 + v33 + 1),0,(unsigned long)(~(unsigned int)v33 & 7));
          v28 = sub_42d7(v5,v6);
        }
        else {
          v28 = v8;
        }
        v26 = v28 + v26;
        if ((!dat_10210) && (v28)) {
          v25 = sub_4350(v5,v28) + v25;
          v21 = *(char *)((long)v5 + (v28 - 1)) == '\n';
        }
        if (v28 < v8) {
          switch(dat_10210) {
            case 0:
              v13 = (char *)sub_490d(v26,v17);
              v14 = (char *)sub_490d(v25,v18);
              if (dat_10214 == '\x01') {
                v1 = *(unsigned char *)(v28 + (long)v5);
                v2 = *(unsigned char *)(v28 + (long)v6);
                sub_43cf(v19,v1);
                sub_43cf(v20,v2);
                v4 = dat_100a8;
                v16 = dat_100a0;
                printf(gettext("%s %s differ: byte %s, line %s is %3o %s %3o %s\n"),v16,v4,v13,v14,(unsigned int)v1,v19,(unsigned int)v2,v20);
                return 1;
              }
              v16 = gettext("%s %s differ: byte %s, line %s\n");
              v7 = ((v16 != "%s %s differ: byte %s, line %s\n") || (sub_4863(5)));
              if (!v7)
                v16 = "%s %s differ: char %s, line %s\n";
              printf(v16,dat_100a0,dat_100a8,v13,v14);
              return 1;
            case 1:
              do {
                v1 = *(unsigned char *)(v28 + (long)v5);
                v2 = *(unsigned char *)(v28 + (long)v6);
                if (v1 != v2) {
                  v13 = (char *)sub_490d(v26,v18);
                  if (dat_10214 != '\x01') // branch-flip
                    printf("%*s %3o %3o\n",v24,v13,(unsigned int)v1,(unsigned int)v2);
                  else {
                    sub_43cf(v20,v1);
                    sub_43cf(v17,v2);
                    printf("%*s %3o %-4s %3o %s\n",v24,v13,(unsigned int)v1,v20,(unsigned int)v2,v17);
                  }
                }
                v26 += 1;
                v28 += 1;
              } while (v28 < v8);
              v22 = -1;
              break;
            case 2:
              v22 = 1;
              break;
            case 3:
              return 1;
            
          }
        }
        if (v32 != v33) break;
        if ((1 <= v22) || (v32 != dat_101f0))
          return (unsigned long)(v22 != 0);
      }
      if ((v22 <= 0) && (dat_10210 != 3)) {
        v13 = *(char **)((long)(int)(unsigned int)(v33 < v32) * 8 + 0x100a0);
        if (v26 != 1) { // branch-flip
          v14 = (char *)sub_490d(v26 + -1,v17);
          if (dat_10210) // branch-flip
            fprintf(stderr,gettext("cmp: EOF on %s after byte %s\n"),v13,v14);
          else {
            v15 = sub_490d(v25 - (unsigned long)v21,v18);
            if (v21) // branch-flip
              v16 = gettext("cmp: EOF on %s after byte %s, line %s\n");
            else {
              v16 = gettext("cmp: EOF on %s after byte %s, in line %s\n");
            }
            fprintf(stderr,v16,v13,v14,v15);
          }
        }
        else {
          fprintf(stderr,gettext("cmp: EOF on %s which is empty\n"),v13);
        }
      }
      return 1;
    }
    v30 = *(unsigned long *)((long)v23 * 8 + 0x10200);
    if ((v30) && (sub_447f(v23) == -1)) {
      do {
        v8 = dat_101f0;
        if (v30 <= dat_101f0)
          v8 = v30;
        v9 = sub_8238(*(unsigned int *)((long)v23 * 4 + 0x100b0),v5,v8);
        if (v9 != v8) {
          if (v9 == 0xffffffffffffffff) {
            v13 = *(char **)((long)v23 * 8 + 0x100a0);
            error(2,*__errno_location(),"%s",v13);
            return v10;
          }
          break;
        }
        v30 -= v9;
      } while (v30);
    }
    v23 += 1;
  } while( true );
}

// Function: sub_42d7 @ 0x42d7
long sub_42d7(long *a0,long *a1)
{
  long *v1; // stack - 0x28
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = a1;
  for (v1 = a0; *v1 == *v2; v1 = &v1[1]) {
    v2 = &v2[1];
  }
  v4 = v2;
  for (v3 = v1; (char)*v3 == (char)*v4; v3 = (long *)((long)v3 + 1)) {
    v4 = (long *)((long)v4 + 1);
  }
  return (long)v3 - (long)a0;
}

// Function: sub_4350 @ 0x4350
long sub_4350(char *a0,long a1)
{
  char v1;
  char *v2; // rax
  long v3; // stack - 0x20
  char *v4; // stack - 0x18
  
  v3 = 0;
  v2 = &a0[a1];
  v1 = *v2;
  *v2 = 10;
  v4 = a0;
  while( true ) {
    v4 = rawmemchr(v4,10);
    if (v4 == v2) break;
    v3 += 1;
    v4 = &v4[1];
  }
  *v2 = v1;
  return v3;
}

// Function: sub_43cf @ 0x43cf
void sub_43cf(unsigned char *a0,unsigned char a1)
{
  unsigned char *v1;
  unsigned char v2; // stack - 0x14
  unsigned char *v3; // stack - 0x10
  
  v2 = a1;
  v3 = a0;
  if (!(*(unsigned short *)((unsigned long)a1 * 2 + *(long *)__ctype_b_loc()) & 0x4000)) {
    v3 = a0;
    if ((char)a1 <= '\xff') {
      *a0 = 0x4d;
      v3 = &a0[2];
      a0[1] = 0x2d;
      v2 = a1 + 0x80;
    }
    if (0x20 <= v2) { // branch-flip
      if (v2 == 0x7f) {
        v1 = &v3[1];
        *v3 = 0x5e;
        v2 = 0x3f;
        v3 = v1;
      }
    }
    else {
      v1 = &v3[1];
      *v3 = 0x5e;
      v2 += 0x40;
      v3 = v1;
    }
  }
  *v3 = v2;
  v3[1] = 0;
}

// Function: sub_447f @ 0x447f
unsigned long sub_447f(int a0)
{
  unsigned int v1;
  unsigned long v2;
  
  if (*(char *)((long)a0 + 0x10215) != '\x01') {
    *(char *)((long)a0 + 0x10215) = 1;
    v2 = *(unsigned long *)((long)a0 * 8 + 0x10200);
    v1 = *(unsigned int *)((long)a0 * 4 + 0x100b0);
    *(unsigned long *)((long)a0 * 8 + 0x10220) = lseek(v1,v2,1);
  }
  return *(unsigned long *)((long)a0 * 8 + 0x10220);
}

// Function: sub_4526 @ 0x4526
void sub_4526(int a0)
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
  (*dat_20240)(a0);
  if (a0) // branch-flip
    v1 = dat_20248;
  else {
    v1 = dat_20250;
  }
  v2 = strlen(dat_20258);
  v3 = strlen(v1);
  if (0x1000 <= v3 + v2) { // branch-flip
    write(2,dat_20258,v2);
    write(2,": C",2);
    write(2,v1,v3);
    v5[0] = 10;
    v7 = (char *)0x1;
  }
  else {
    v4 = mempcpy(v5,dat_20258,v2);
    v7 = mempcpy(mempcpy(v4,": C",2),v1,v3);
    *v7 = 10;
    v7 = &v7[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v7);
  if (!a0)
    _exit(dat_10020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_472f @ 0x472f
void sub_472f(void)
{
  return;
}

// Function: sub_473d @ 0x473d
unsigned long sub_473d(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_4526(0xb); // no-return
}

// Function: sub_4769 @ 0x4769
void sub_4769(int a0)
{
  unsigned int v1; // eax
  
  if ((a0) && (!dat_20260))
    v1 = 0xb;
  else {
    v1 = 0;
  }
  sub_4526(v1); // no-return
}

// Function: sub_479f @ 0x479f
void * sub_479f(void *a0) // return-dupe
{
  int v1; // eax
  void *v2; // rax
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = sub_472f;
  }
  dat_20240 = v2;
  v2 = (char *)gettext("program error");
  dat_20248 = (char *)v2;
  v2 = (char *)gettext("stack overflow");
  dat_20250 = (char *)v2;
  v2 = (unsigned long)sub_4852();
  dat_20258 = v2;
  v1 = sub_59d6(sub_4769,0x10240,0x10000);
  if (v1) {
    v2 = (int *)__errno_location();
    *(int *)v2 = 0x5f;
    v2 = (void *)0xffffffff;
    return v2;
  }
  v1 = sub_5926(sub_473d);
  dat_20260 = v1;
  v2 = NULL;
  return v2;
}

// Function: sub_4852 @ 0x4852
unsigned long sub_4852(void)
{
  return __progname;
}

// Function: sub_4863 @ 0x4863
unsigned int sub_4863(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_5475(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_490d @ 0x490d
char * sub_490d(long a0,long a1)
{
  char *v1;
  long v2; // stack - 0x20
  char *v3; // stack - 0x10
  
  v3 = (char *)(a1 + 0x14);
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

// Function: sub_4a4b @ 0x4a4b
void sub_4a4b(char *a0)
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
  dat_20268 = v2;
  program_invocation_name = v2;
}

// Function: sub_4b36 @ 0x4b36
char sub_4b36(char *a0,char *a1)
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
  
  v4 = (char *)sub_6908(a1,2);
  v11 = 0;
  v10 = a0;
  while( true ) {
    while( true ) {
      if ((!*v10) || (v5 = (char *)sub_92f3(v10,v4), !v5)) {
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
        sub_9a61(v8);
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
      sub_9a61(&v9);
      if (((char)v24 != '\x01') || (v24._4_4_))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_9a61(v8);
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
    sub_9a61(v8);
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
    sub_9a61(v8);
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

// Function: sub_500c @ 0x500c
char * sub_500c(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = gettext(a0);
  if (v1 != a0) {
    if (sub_4b36(v1,a0)) // branch-flip
      a0 = v1;
    else {
      v2 = strlen(v1);
      v3 = (char *)sub_7605(strlen(a0) + v2 + 4);
      sprintf(v3,"%s (%s)",v1,a0);
      a0 = v3;
    }
  }
  return a0;
}

// Function: sub_50b2 @ 0x50b2
char * sub_50b2(char *a0,char *a1)
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
  v2 = (char *)sub_852f();
  v6 = NULL;
  v7 = NULL;
  v9 = NULL;
  if (sub_83ce(v2,"UTF-8")) { // branch-flip
    v6 = (char *)sub_7cf6(a1,"UTF-8",v2);
    v8 = v6;
    v3 = strlen(v2);
    v4 = (char *)sub_7605(v3 + 0xb);
    memcpy(v4,v2,v3);
    memcpy(&v4[v3],"//TRANSLIT",0xb);
    v2 = (char *)sub_7cf6(a1,"UTF-8",v4);
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
  if (((sub_4b36(v1,a0)) || ((v8 && (sub_4b36(v1,v8))))) || ((v9 && (sub_4b36(v1,v9))))) {
    if (v6)
      free(v6);
    if (v7)
      free(v7);
    v5 = v1;
  }
  else {
    v3 = strlen(v1);
    v5 = (char *)sub_7605(strlen(v2) + v3 + 4);
    sprintf(v5,"%s (%s)",v1,v2);
    if (v6)
      free(v6);
    if (v7)
      free(v7);
  }
  return v5;
}

// Function: sub_5395 @ 0x5395
char * sub_5395(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_53bd @ 0x53bd
unsigned long sub_53bd(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_5395(a0);
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

// Function: sub_5475 @ 0x5475
void sub_5475(unsigned int a0,char *a1,unsigned long a2)
{
  sub_53bd(a0,a1,a2);
}

// Function: sub_54a3 @ 0x54a3
void sub_54a3(unsigned int a0)
{
  sub_5395(a0);
}

// Function: sub_54be @ 0x54be
void sub_54be(void)
{
  return;
}

// Function: sub_54c9 @ 0x54c9
void sub_54c9(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_6874(a0,v1) <= -1)
    return;
  dat_20270 = v2 + -1;
}

// Function: sub_5525 @ 0x5525
void sub_5525(unsigned long a0,long a1,long a2) // return-dupe x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_5525
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
  if ((dat_20290) && (v3 = (*dat_20290)(v1,0), v3))
    return;
  if ((dat_20278) && (v2 = *(unsigned long *)(a2 + 0xa0), dat_20270)) {
    v3 = *__errno_location();
    v4 = sub_6874(dat_20270,&v6);
    *__errno_location() = v3;
    if (0 <= v4) {
      if (v6 <= v1) // branch-flip
        v7 = v1 <= (unsigned long)(v8 - 1U);
      else {
        v7 = (*v9)(v1,&v6) != 0;
      }
      if (v7) {
        v5 = ((dat_20280 <= v2) && (v2 <= dat_20288 + dat_20280));
        (*dat_20278)(v5,a2);
      }
    }
  }
  if ((dat_20290) && (v3 = (*dat_20290)(v1,1), v3))
    return;
  signal(0xb,0);
}

// Function: sub_56c8 @ 0x56c8
void sub_56c8(int a0)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  undefined4 v3; // stack - 0x20
  
  v1 = sub_5525;
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
  if (dat_20278)
    v3 = 0x8000004;
  sigaction(a0,(sigaction *)&v1,NULL);
}

// Function: sub_5926 @ 0x5926
unsigned long sub_5926(unsigned long a0)
{
  dat_20290 = a0;
  sub_56c8(0xb);
  return 0;
}

// Function: sub_5959 @ 0x5959
void sub_5959(void) // return-dupe
{
  dat_20290 = 0;
  if (dat_20278) {
    dat_20290 = 0;
    return;
  }
  signal(0xb,0);
}

// Function: sub_5995 @ 0x5995
unsigned long sub_5995(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_54be();
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_59d6 @ 0x59d6
unsigned long sub_59d6(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x28
  unsigned int v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  
  if ((!dat_20270) && (sub_54c9(&v1), !dat_20270))
    return 0xffffffff;
  v2 = 0;
  dat_20278 = a0;
  dat_20280 = a1;
  dat_20288 = a2;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_56c8(0xb);
  return 0;
}

// Function: sub_5aa8 @ 0x5aa8
void sub_5aa8(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_20278 = 0;
  if (dat_20290) // branch-flip
    sub_56c8(0xb);
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_5b45 @ 0x5b45
bool sub_5b45(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_5b86 @ 0x5b86
unsigned long sub_5b86(struct_1 *a0,char *a1) // return-dupe x3, return-dupe
{
  void *v1;
  long v2;
  long v3;
  int v4; // eax
  int v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  unsigned long v7; // stack - 0x10
  
  v5 = open(a1,0);
  if (v5 <= -1)
    return 0xffffffff;
  a0->field_0x0 = 0;
  a0->field_0x10 = 0;
  v6 = 0;
  a0->field_0x18 = &a0[1];
  v7 = 1;
  a0->field_0x20 = 0;
  a0->field_0x30 = 0;
  a0->field_0x38 = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = a0->field_0x18;
      v4 = (int)read(v5,v1,v7);
      if ((0 <= v4) || (*__errno_location() != 4)) {
        if (1 <= v4) {
          if ((unsigned long)(long)(v4 + 0x1049) <= v7) {
            a0->field_0x8 = (long)v4;
            while( true ) {
              v2 = a0->field_0x8;
              v1 = a0->field_0x18;
              v3 = a0->field_0x8;
              v4 = (int)read(v5,(void *)((long)v1 + v3),v7 - v2);
              if ((v4 <= -1) && (*__errno_location() == 4)) goto label_5e35;
              if (v4 < 0) goto label_5e87;
              if (v7 - a0->field_0x8 < (unsigned long)(long)(v4 + 0x1049)) break;
              if (!v4) {
                close(v5);
                return 0;
              }
              a0->field_0x8 = a0->field_0x8 + (long)v4;
            }
          }
          goto label_5d35;
        }
label_5e87:
        close(v5);
        goto label_5e94;
      }
    }
    else {
label_5d35:
      if (v6) { // branch-flip
        v7 <<= 1;
        if (!v7) goto label_5e87;
        if (a0->field_0x20)
          munmap((void *)a0->field_0x20,a0->field_0x28);
      }
      else {
        v6 = (unsigned long)getpagesize();
        for (v7 = v6; v7 <= 0x1049; v7 = v7 << 1) {
        }
      }
      a0->field_0x20 = mmap(0,v7,3,0x22,0xffffffff,0);
      if (a0->field_0x20 == -1) {
        close(v5);
        return 0xffffffff;
      }
      a0->field_0x28 = v7;
      a0->field_0x30 = a0->field_0x20;
      a0->field_0x38 = a0->field_0x30 + v7;
      a0->field_0x18 = (void *)a0->field_0x20;
    }
label_5e35:
    if ((long)lseek(v5,0,0) <= -1) {
      close(v5);
      v5 = open(a1,0);
      if (v5 < 0) {
label_5e94:
        if (!a0->field_0x20)
          return 0xffffffff;
        munmap((void *)a0->field_0x20,a0->field_0x28);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_5ec3 @ 0x5ec3
unsigned long sub_5ec3(struct_2 *a0) // early-return
{
  if (a0->field_0x0 == a0->field_0x8) {
    a0->field_0x10 = 1;
    return 0xffffffff;
  }
  return (unsigned long)*(unsigned char *)(a0->field_0x0 + a0->field_0x18);
}

// Function: sub_5f0f @ 0x5f0f
int sub_5f0f(struct_2 *a0)
{
  int v1; // eax
  
  v1 = sub_5ec3(a0);
  if (0 <= v1)
    a0->field_0x0 = a0->field_0x0 + 1;
  return v1;
}

// Function: sub_5f4b @ 0x5f4b
unsigned long sub_5f4b(struct_2 *a0,long *a1) // return-dupe
{
  int v1; // eax
  int v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = 0;
  v2 = 0;
  do {
    v1 = sub_5ec3(a0);
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
    sub_5f0f(a0);
    v2 += 1;
  } while( true );
}

// Function: sub_6027 @ 0x6027
void sub_6027(long a0) // return-dupe
{
  if (!*(long *)(a0 + 0x20))
    return;
  munmap(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28));
}

// Function: sub_6062 @ 0x6062
unsigned long sub_6062(struct_3 *a0) // return-dupe
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
  
  if ((int)sub_5b86(v3,"/proc/self/maps") < 0)
    return 0xffffffff;
  v6 = v8;
  v7 = v9;
  do {
    while( true ) {
      if ((((int)sub_5f4b(v3,&v4) < 0) || (sub_5f0f(v3) != 0x2d)) || ((int)sub_5f4b(v3,&v5) < 0)) {
        sub_6027(v3); // return-dupe
        return 0;
      }
      do {
        v2 = sub_5f0f(v3);
        if ((v2 != -1) && (v2 != 10))
          v1 = 1;
        else {
          v1 = 0;
        }
      } while (v1);
      if ((v6 < v4) || ((unsigned long)(v5 - 1U) < v7 - 1)) break;
      if (((v4 < v6) && (v2 = sub_67e7(a0,v4,v6), v2)) || ((v7 - 1 < (unsigned long)(v5 - 1U) && (v2 = sub_67e7(a0,v7,v5), v2)))) {
        sub_6027(v3);
        return 0;
      }
    }
    v2 = sub_67e7(a0,v4,v5);
  } while (!v2);
  sub_6027(v3);
  return 0;
}

// Function: sub_6203 @ 0x6203
unsigned long sub_6203(struct_3 *a0) // return-dupe
{
  if (!sub_6062(a0))
    return 0;
  return 0xffffffff;
}

// Function: sub_6236 @ 0x6236
void sub_6236(void)
{
  dat_20298 = (long)getpagesize();
}

// Function: sub_624f @ 0x624f
unsigned int sub_624f(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_20298,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_62a6 @ 0x62a6
unsigned long sub_62a6(unsigned long a0)
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
    if (v5 / dat_20298 < v6)
      v6 = v5 / dat_20298;
    v1 = dat_20298 * v6;
    v3 = dat_20298 * v6;
    if (mincore((void *)(v5 - v3),v1,v4) <= -1) break;
    v5 -= dat_20298 * v6;
  }
  while (v6 != 1) {
    v1 = v6 + 1 >> 1;
    v2 = dat_20298 * v1;
    v3 = dat_20298 * v1;
    if (0 <= mincore((void *)(v5 - v3),v2,v4)) { // branch-flip
      v5 -= dat_20298 * v1;
      v6 >>= 1;
    }
    else {
      v6 = v1;
    }
  }
  return v5;
}

// Function: sub_645a @ 0x645a
void * sub_645a(long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [1032];
  void *v4; // stack - 0x440
  unsigned long v5; // stack - 0x438
  
  v5 = 0x400;
  v4 = (void *)(a0 + dat_20298);
  while( true ) {
    if (!v4)
      return NULL;
    if ((unsigned long)-(long)v4 / dat_20298 < v5)
      v5 = (unsigned long)-(long)v4 / dat_20298;
    v1 = dat_20298 * v5;
    if (mincore(v4,v1,v3) <= -1) break;
    v4 = (void *)((long)v4 + dat_20298 * v5);
  }
  while (v5 != 1) {
    v1 = v5 + 1 >> 1;
    v2 = dat_20298 * v1;
    if (0 <= mincore(v4,v2,v3)) { // branch-flip
      v4 = (void *)((long)v4 + dat_20298 * v1);
      v5 >>= 1;
    }
    else {
      v5 = v1;
    }
  }
  return v4;
}

// Function: sub_65f2 @ 0x65f2
unsigned long sub_65f2(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  void *v6; // stack - 0x20
  
  v1 = dat_20298 * (a0 / dat_20298);
  v2 = (dat_20298 * (a1 / dat_20298 + 1) - v1) / dat_20298;
  for (v4 = 1; v4 < v2; v4 = v4 << 1) {
  }
  do {
    v4 >>= 1;
    if (!v4)
      return 1;
    v3 = dat_20298 * v4;
    v6 = (void *)(v3 + v1);
    for (v5 = v4; v5 < v2; v5 = v5 + v4 * 2) {
      if (sub_624f(v6))
        return 0;
      v6 = (void *)((long)v6 + v3 * 2);
    }
  } while( true );
}

// Function: sub_6703 @ 0x6703
unsigned long sub_6703(unsigned long a0,long *a1) // early-return
{
  long v1;
  unsigned long v2; // rax
  
  v2 = a0 * 2 - *a1;
  if (a0 < v2)
    return 0;
  v1 = *a1;
  return sub_65f2(v2,v1 - 1);
}

// Function: sub_6760 @ 0x6760
unsigned long sub_6760(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // rax
  
  if (!dat_20298)
    sub_6236();
  v1 = dat_20298 * (a0 / dat_20298);
  *a1 = sub_62a6(v1);
  a1[1] = sub_645a(v1);
  a1[2] = sub_6703;
  return 0;
}

// Function: sub_67e7 @ 0x67e7
unsigned long sub_67e7(struct_3 *a0,unsigned long a1,unsigned long a2) // return-dupe
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

// Function: sub_6874 @ 0x6874
unsigned long sub_6874(unsigned long a0,unsigned long *a1) // early-return
{
  unsigned long v1; // stack - 0x38
  unsigned long *v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  int v4; // stack - 0x20
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_6203(&v1);
  if (!v4) {
    a1[2] = sub_5b45;
    return 0;
  }
  return sub_6760(a0,a1);
}

// Function: sub_6908 @ 0x6908
unsigned char * sub_6908(char *a0,int a1)
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
        while (((v9 = 0, v10 < v3 && (sub_8577(&v3), v12)) && (iswspace(v13)))) {
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
          sub_8577(&v3);
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
  sub_7bc9(); // no-return
}

// Function: sub_6c67 @ 0x6c67
void sub_6c67(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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

// Function: sub_7263 @ 0x7263
void sub_7263(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_6c67(a0,a1,a2,a3,a4,v1);
}

// Function: sub_72d6 @ 0x72d6
void sub_72d6(FILE *a0,char *a1,char *a2,char *a3,struct_8 *a4)
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
  sub_6c67(a0,a1,a2,a3,v2,v3);
}

// Function: sub_73dc @ 0x73dc
void sub_73dc(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_72d6(a8,a9,a10,a11,&v3);
}

// Function: sub_74c0 @ 0x74c0
void sub_74c0(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-diffutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_7569 @ 0x7569
void sub_7569(unsigned long a0,unsigned long a1)
{
  sub_76cd(NULL,a0,a1);
}

// Function: sub_7594 @ 0x7594
void sub_7594(void *a0,unsigned long a1,unsigned long a2)
{
  sub_76cd(a0,a1,a2);
}

// Function: sub_75c5 @ 0x75c5
void sub_75c5(unsigned long a0)
{
  sub_7605(a0);
}

// Function: sub_75e3 @ 0x75e3
long sub_75e3(long a0)
{
  if (!a0)
    sub_7bc9(); // no-return
  return a0;
}

// Function: sub_7605 @ 0x7605
void sub_7605(unsigned long a0)
{
  sub_75e3(malloc(a0));
}

// Function: sub_762b @ 0x762b
void sub_762b(unsigned long a0)
{
  sub_75e3(sub_8462(a0));
}

// Function: sub_7651 @ 0x7651
void * sub_7651(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_7bc9(); // no-return
  return v1;
}

// Function: sub_769c @ 0x769c
void sub_769c(void *a0,unsigned long a1)
{
  sub_75e3(sub_8480(a0,a1));
}

// Function: sub_76cd @ 0x76cd
void * sub_76cd(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_7bc9(); // no-return
  return v1;
}

// Function: sub_7727 @ 0x7727
void sub_7727(void *a0,unsigned long a1,unsigned long a2)
{
  sub_75e3(sub_84e0(a0,a1,a2));
}

// Function: sub_7760 @ 0x7760
void sub_7760(void *a0,unsigned long *a1)
{
  sub_778e(a0,a1,1);
}

// Function: sub_778e @ 0x778e
unsigned long sub_778e(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_7bc9(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_76cd(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_786d @ 0x786d
unsigned long sub_786d(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_7bc9(); // no-return
  v3 = sub_7651(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_7a29 @ 0x7a29
void sub_7a29(unsigned long a0)
{
  sub_7a6f(a0,1);
}

// Function: sub_7a4c @ 0x7a4c
void sub_7a4c(unsigned long a0)
{
  sub_7aa0(a0,1);
}

// Function: sub_7a6f @ 0x7a6f
void sub_7a6f(unsigned long a0,unsigned long a1)
{
  sub_75e3(calloc(a0,a1));
}

// Function: sub_7aa0 @ 0x7aa0
void sub_7aa0(unsigned long a0,unsigned long a1)
{
  sub_75e3(sub_84b7(a0,a1));
}

// Function: sub_7ad1 @ 0x7ad1
void sub_7ad1(char *a0,unsigned long a1)
{
  memcpy((void *)sub_7605(a1),a0,a1);
}

// Function: sub_7b09 @ 0x7b09
void sub_7b09(void *a0,unsigned long a1)
{
  memcpy((void *)sub_762b(a1),a0,a1);
}

// Function: sub_7b49 @ 0x7b49
void sub_7b49(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_762b(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_7b98 @ 0x7b98
void sub_7b98(char *a0)
{
  sub_7ad1(a0,strlen(a0) + 1);
}

// Function: sub_7bc9 @ 0x7bc9
void sub_7bc9(void)
{
  error(dat_10020,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_7c09 @ 0x7c09
void sub_7c09(void) // return-dupe
{
  int v1; // eax
  
  v1 = sub_9d37();
  if (!v1)
    return;
  error(dat_10020,v1,gettext("standard file descriptors"));
}

// Function: sub_7c4f @ 0x7c4f
int sub_7c4f(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_9e03(a0,a1,a2,a3,a4);
  if (v1 <= -1) {
    if (*__errno_location() == 0xc)
      sub_7bc9(); // no-return
  }
  return v1;
}

// Function: sub_7cad @ 0x7cad
long sub_7cad(char *a0,unsigned long a1)
{
  long v1; // rax
  
  v1 = sub_a1cc(a0,a1);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_7bc9(); // no-return
  }
  return v1;
}

// Function: sub_7cf6 @ 0x7cf6
long sub_7cf6(char *a0,char *a1,char *a2)
{
  long v1; // rax
  
  v1 = sub_a4ef(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_7bc9(); // no-return
  }
  return v1;
}

// Function: sub_7d47 @ 0x7d47
unsigned long sub_7d47(long *a0,int a1) // return-dupe x2
{
  char v1 [16];
  char v2 [16];
  
  v1._8_8_ = 0;
  v1._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
  if (*a0 < SUB168((v1._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)a1),0)) {
    *a0 = -0x8000000000000000;
    return 1;
  }
  v2._8_8_ = 0;
  v2._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
  if (SUB168((v2._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)a1),0) < *a0) {
    *a0 = 0x7fffffffffffffff;
    return 1;
  }
  *a0 = *a0 * (long)a1;
  return 0;
}

// Function: sub_7de9 @ 0x7de9
unsigned int sub_7de9(long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_7d47(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_7e2e @ 0x7e2e
unsigned int sub_7e2e(long a0,long *a1,int a2,unsigned long *a3,char *a4)
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x54,"xstrtoimax"); // no-return
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
        v7 = sub_7d47(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v4 | 2;
      case 0x45:
        v7 = sub_7de9(&v3,v5,6);
        break;
      case 0x47:
      case 0x67:
        v7 = sub_7de9(&v3,v5,3);
        break;
      case 0x4b:
      case 0x6b:
        v7 = sub_7de9(&v3,v5,1);
        break;
      case 0x4d:
      case 0x6d:
        v7 = sub_7de9(&v3,v5,2);
        break;
      case 0x50:
        v7 = sub_7de9(&v3,v5,5);
        break;
      case 0x54:
      case 0x74:
        v7 = sub_7de9(&v3,v5,4);
        break;
      case 0x59:
        v7 = sub_7de9(&v3,v5,8);
        break;
      case 0x5a:
        v7 = sub_7de9(&v3,v5,7);
        break;
      case 0x62:
        v7 = sub_7d47(&v3,0x200);
        break;
      case 99:
        v7 = 0;
        break;
      case 0x77:
        v7 = sub_7d47(&v3,2);
      
    }
    v4 |= v7;
    *v8 = *v8 + (long)v6;
    if (*(char *)*v8)
      v4 |= 2;
  }
  *a3 = v3;
  return v4;
}

// Function: sub_8238 @ 0x8238
long sub_8238(int a0,void *a1,long a2) // early-return
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

// Function: sub_8307 @ 0x8307
void * sub_8307(void *a0,void *a1,void *a2) // early-return
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

// Function: sub_83ce @ 0x83ce
int sub_83ce(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_add3((unsigned char)*a0);
    v2 = sub_add3((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_8448 @ 0x8448
unsigned long sub_8448(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_8462 @ 0x8462
void sub_8462(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_8480 @ 0x8480
void sub_8480(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_84b7 @ 0x84b7
void sub_84b7(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_84e0 @ 0x84e0
void sub_84e0(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_852f @ 0x852f
char * sub_852f(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_8577 @ 0x8577
void sub_8577(struct_4 *a0)
{
  char v1;
  long v2;
  char *v3;
  char *v4;
  
  if (a0->field_0x14)
    return;
  if (!a0->field_0x8) {
    v1 = *a0->field_0x18;
    if (sub_b026((int)v1)) {
      a0->field_0x20 = 1;
      a0->field_0x2c = (int)*a0->field_0x18;
      a0->field_0x28 = 1;
      goto label_8784;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x9[3]))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    a0->field_0x8 = 1;
  }
  v2 = a0->field_0x0;
  v3 = a0->field_0x18;
  v4 = a0->field_0x18;
  a0->field_0x20 = sub_aad0(&a0->field_0x2c,v4,v2 - (long)v3,(mbstate_t *)&a0->field_0x9[3]);
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
label_8784:
  a0->field_0x14 = 1;
  return;
}

// Function: sub_8791 @ 0x8791
void sub_8791(struct_0 *a0,long a1)
{
  a0->field_0x18 = a0->field_0x18 + a1;
  a0->field_0x0 = a0->field_0x0 + a1;
}

// Function: sub_87d0 @ 0x87d0
void sub_87d0(struct_9 *a0,struct_9 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  a0->field_0x8 = a1->field_0x8;
  if (a0->field_0x8) // branch-flip
    memcpy(&a0->field_0x9[3],&a1->field_0x9[3],8);
  else {
    memset(&a0->field_0x9[3],0,8);
  }
  a0->field_0x14 = a1->field_0x14;
  sub_af80((struct_6 *)&a0[1],(struct_7 *)&a1[1]);
}

// Function: sub_8876 @ 0x8876
unsigned long sub_8876(char *a0,long a1,unsigned long a2,unsigned long *a3) // return-dupe
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
      v2 = sub_ae19(a2 << 3);
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
        goto label_8a6b;
      }
    }
    v11 += 1;
    *(unsigned long *)(v10 * 8 + v2) = v10 - v11;
label_8a6b:
  }
  *v3 = 0;
  v12 = 0;
  v13 = v8;
  v14 = v8;
  do {
    while( true ) {
      if (!*v14) goto label_8b36;
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
label_8b36:
  *(unsigned long *)((long)v5 + -8) = 0x8b42;
  v16 = v2;
  sub_aeea(v2);
  return 1;
}

// Function: sub_8b5d @ 0x8b5d
unsigned long sub_8b5d(void *a0,void *a1,long *a2)
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
  v22 = sub_b06b(a1);
  if (0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0) && !SUB168(ZEXT816(0x38) * ZEXT816(v22),8)) { // branch-flip
    if (0xfa1 <= v22 * 0x38) { // branch-flip
      v3 = sub_ae19(v22 * 0x38);
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
  *(unsigned long *)&v12[-8] = 0x8d55;
  v23 = v3;
  v24 = v3;
  v25 = v4;
  v26 = v4;
  memset(v6,0,8);
  while( true ) {
    v33 = 0;
    *(unsigned long *)&v12[-8] = 0x8db6;
    sub_9a61(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) break;
    *(unsigned long *)&v12[-8] = 0x8d8f;
    sub_af80((struct_6 *)(v24 + v17 * 0x30),&v7);
    v7 = (void *)(v34 + (long)v7);
    v17 += 1;
  }
  *(unsigned long *)(v26 + 8) = 1;
  v19 = 0;
  v18 = 2;
label_8fc1:
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
          *(unsigned long *)&v12[-8] = 0x8f17;
          if (!memcmp(v1,v2,v3)) {
            v13 = 1;
            goto label_8f2a;
          }
        }
        v13 = 0;
      }
label_8f2a:
      if (v13) {
        v19 += 1;
        *(unsigned long *)(v18 * 8 + v26) = v18 - v19;
        goto label_8fb9;
      }
      if (!v19) goto label_8f6c;
      v19 -= *(long *)(v26 + v19 * 8);
    } while( true );
  }
  *v14 = 0;
  v20 = 0;
  v29 = v16;
  v10[0] = 0;
  *(unsigned long *)&v12[-8] = 0x901d;
  memset(v9,0,8);
  v28 = 0;
  v7 = v16;
  v8[0] = 0;
  *(unsigned long *)&v12[-8] = 0x904d;
  memset(v6,0,8);
  v33 = 0;
  do {
    *(unsigned long *)&v12[-8] = 0x92a3;
    sub_9a61(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) goto label_92c9;
    if ((*(char *)(v24 + v20 * 0x30 + 0x10)) && (v35))
      v13 = *(int *)(v24 + v20 * 0x30 + 0x14) == v36;
    else {
      if (*(long *)(v24 + v20 * 0x30 + 8) == v34) {
        v3 = *(unsigned long *)(v24 + v20 * 0x30 + 8);
        v2 = *(void **)(v24 + v20 * 0x30);
        *(unsigned long *)&v12[-8] = 0x913e;
        if (!memcmp(v2,v7,v3)) {
          v13 = 1;
          goto label_9151;
        }
      }
      v13 = 0;
    }
label_9151:
    if (v13) {
      v20 += 1;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
      if (v20 == v22) {
        *v14 = (long)v29;
label_92c9:
        *(unsigned long *)&v12[-8] = 0x92d8;
        sub_aeea(v23);
        return 1;
      }
    }
    else if (v20) { // branch-flip
      v21 = *(long *)(v26 + v20 * 8);
      v20 -= v21;
      for (; v21; v21 = v21 + -1) {
        *(unsigned long *)&v12[-8] = 0x91e7;
        sub_9a61(v10);
        if ((v31 != '\x01') || (v32))
          v13 = 1;
        else {
          v13 = 0;
        }
        if (!v13) {
          *(unsigned long *)&v12[-8] = 0x920e; // no-return
          abort();
        }
        v29 = (void *)(v30 + (long)v29);
        v28 = 0;
      }
    }
    else {
      *(unsigned long *)&v12[-8] = 0x9247;
      sub_9a61(v10);
      if ((v31 != '\x01') || (v32))
        v13 = 1;
      else {
        v13 = 0;
      }
      if (!v13) {
        *(unsigned long *)&v12[-8] = 0x926e; // no-return
        abort();
      }
      v29 = (void *)(v30 + (long)v29);
      v28 = 0;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
    }
  } while( true );
label_8f6c:
  *(unsigned long *)(v18 * 8 + v26) = v18;
label_8fb9:
  v18 += 1;
  goto label_8fc1;
}

// Function: sub_92f3 @ 0x92f3
char * sub_92f3(char *a0,char *a1) // early-return, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_92f3
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
    sub_9a61(v7);
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
        sub_9a61(v10);
        if ((v41 != '\x01') || (v42))
          v16 = 1;
        else {
          v16 = 0;
        }
        if (!v16)
          return NULL;
        if (((v2) && (10 <= v18)) && (v18 * 5 <= v19)) {
          for (v21 = v19 - v20; v21; v21 = v21 + -1) {
            sub_9a61(v11);
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
          sub_9a61(v11);
          if ((v36 != '\x01') || (v37))
            v16 = 1;
          else {
            v16 = 0;
          }
          if (!v16) {
            if (sub_8b5d(a0,a1,&v12))
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
          sub_9a61(v15);
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
            sub_9a61(v15);
            if ((v51 != '\x01') || (v52))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16)
              return v39;
            sub_9a61(v13);
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
          if (sub_8876(v17,a1,v5,&v12))
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

// Function: sub_9a61 @ 0x9a61
void sub_9a61(struct_5 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_b026((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_9c67;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_a5e2(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_aad0(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
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
label_9c67:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_9c78 @ 0x9c78
void sub_9c78(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_9ca2 @ 0x9ca2
void sub_9ca2(struct_10 *a0,struct_10 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_af80((struct_6 *)&a0[1].field_0x1[2],(struct_7 *)&a1[1].field_0x1[2]);
}

// Function: sub_9d37 @ 0x9d37
int sub_9d37(void)
{
  int v1; // eax
  int v2; // stack - 0x18
  
  v2 = 0;
  do {
    if (3 <= v2)
      return 0;
    if ((int)sub_a62a(v2,1) <= -1) {
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

// Function: sub_9e03 @ 0x9e03
unsigned long sub_9e03(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe x2, return-dupe
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
    if (!v4) goto label_a113;
    v11 = iconv(a2,&v5,&v4,&v3,&v2);
  } while (v11 != -1);
  if (*__errno_location() == 0x16) {
label_a113:
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

// Function: sub_a1cc @ 0xa1cc
char * sub_a1cc(char *a0,unsigned long a1) // return-dupe
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

// Function: sub_a4ef @ 0xa4ef
char * sub_a4ef(char *a0,char *a1,char *a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  
  if ((*a0) && (sub_83ce(a1,a2))) {
    v3 = iconv_open(a2,a1);
    if (v3 == -1)
      return NULL;
    v2 = (char *)sub_a1cc(a0,v3);
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

// Function: sub_a5e2 @ 0xa5e2
unsigned long sub_a5e2(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_a62a @ 0xa62a
int sub_a62a(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_a99c(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_a9cd(a0,(unsigned int)v2);
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

// Function: sub_a99c @ 0xa99c
int sub_a99c(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_a9cd @ 0xa9cd
int sub_a9cd(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_202a0) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_202a0 = 1;
    else {
      v3 = sub_a99c(a0,a1);
      if (0 <= v3)
        dat_202a0 = -1;
    }
  }
  else {
    v3 = sub_a99c(a0,a1);
  }
  if ((0 <= v3) && (dat_202a0 == -1)) {
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

// Function: sub_aad0 @ 0xaad0
unsigned long sub_aad0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_4863(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_ab7a @ 0xab7a
unsigned long sub_ab7a(int a0) // return-dupe
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

// Function: sub_abb8 @ 0xabb8
unsigned long sub_abb8(int a0) // return-dupe
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

// Function: sub_abea @ 0xabea
bool sub_abea(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_ac0b @ 0xac0b
unsigned int sub_ac0b(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_ac33 @ 0xac33
unsigned long sub_ac33(int a0) // return-dupe
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

// Function: sub_ac60 @ 0xac60
bool sub_ac60(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_ac84 @ 0xac84
bool sub_ac84(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_aca8 @ 0xaca8
bool sub_aca8(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_accc @ 0xaccc
bool sub_accc(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_acf0 @ 0xacf0
unsigned long sub_acf0(int a0) // return-dupe, return-dupe x3
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

// Function: sub_ad3a @ 0xad3a
unsigned long sub_ad3a(int a0) // return-dupe
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

// Function: sub_ad67 @ 0xad67
bool sub_ad67(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_ad8b @ 0xad8b
unsigned long sub_ad8b(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_add3 @ 0xadd3
int sub_add3(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_adf6 @ 0xadf6
int sub_adf6(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_ae19 @ 0xae19
long sub_ae19(unsigned long a0) // early-return
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

// Function: sub_aeea @ 0xaeea
void sub_aeea(unsigned long a0) // return-dupe
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1)));
}

// Function: sub_af45 @ 0xaf45
unsigned int sub_af45(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_af80 @ 0xaf80
void sub_af80(struct_6 *a0,struct_7 *a1) // return-dupe
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

// Function: sub_b026 @ 0xb026
bool sub_b026(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0xcf00) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_b06b @ 0xb06b
char * sub_b06b(char *a0)
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
      sub_9a61(v4);
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

// Function: _DT_FINI @ 0xb13c
void _DT_FINI(void)
{
  return;
}
