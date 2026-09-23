// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_cfe8)
    return;
  (*dat_cfe8)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_cda0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2490
void __cxa_finalize(void)
{
  (*dat_cff8)(); // jump-as-call
}

// Function: raise @ 0x24a0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cda8)(); // jump-as-call
  return v1;
}

// Function: free @ 0x24b0
void free(void *a0)
{
  (*dat_cdb0)(); // jump-as-call
}

// Function: strtoimax @ 0x24c0
void strtoimax(void)
{
  (*dat_cdb8)(); // jump-as-call
}

// Function: __mempcpy_chk @ 0x24d0
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cdc0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x24e0
void abort(void)
{
  (*dat_cdc8)(); // jump-as-call
}

// Function: __errno_location @ 0x24f0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_cdd0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2500
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_cdd8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x2510
void _exit(int a0)
{
  (*dat_cde0)(); // jump-as-call
}

// Function: iconv @ 0x2520
void iconv(void)
{
  (*dat_cde8)(); // jump-as-call
}

// Function: sigaction @ 0x2530
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_cdf0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x2540
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cdf8)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x2550
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ce00)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x2560
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ce08)(); // jump-as-call
  return v1;
}

// Function: write @ 0x2570
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_ce10)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2580
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce18)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x2590
int __open_2(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_ce20)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x25a0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ce28)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x25b0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce30)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x25c0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce38)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x25d0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce40)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x25e0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce48)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x25f0
void __stack_chk_fail(void)
{
  (*dat_ce50)(); // jump-as-call
}

// Function: mmap @ 0x2600
void mmap(void)
{
  (*dat_ce58)(); // jump-as-call
}

// Function: getopt_long @ 0x2610
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ce60)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2620
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce68)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2630
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce70)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x2640
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce78)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2650
void lseek(void)
{
  (*dat_ce80)(); // jump-as-call
}

// Function: __assert_fail @ 0x2660
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_ce88)(); // jump-as-call
}

// Function: fputs @ 0x2670
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_ce90)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x2680
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce98)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2690
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cea0)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x26a0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cea8)(); // jump-as-call
  return v1;
}

// Function: read @ 0x26b0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_ceb0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x26c0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_ceb8)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x26d0
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_cec0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x26e0
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cec8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x26f0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ced0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2700
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_ced8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x2710
void signal(void)
{
  (*dat_cee0)(); // jump-as-call
}

// Function: sigemptyset @ 0x2720
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_cee8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2730
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_cef0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2740
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cef8)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x2750
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_cf00)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x2760
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf08)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x2770
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf10)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x2780
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf18)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x2790
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf20)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x27a0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf28)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x27b0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_cf30)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x27c0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf38)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x27d0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf40)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x27e0
void iconv_close(void)
{
  (*dat_cf48)(); // jump-as-call
}

// Function: iswspace @ 0x27f0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf50)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x2800
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf58)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x2810
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf60)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2820
void error(int a0,int a1,char *a2,...)
{
  (*dat_cf68)(); // jump-as-call
}

// Function: open @ 0x2830
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf70)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x2840
void perror(char *a0)
{
  (*dat_cf78)(); // jump-as-call
}

// Function: getpagesize @ 0x2850
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_cf80)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x2860
void exit(int a0)
{
  (*dat_cf88)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2870
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_cf90)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x2880
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf98)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2890
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_cfa0)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x28a0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_cfa8)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x28b0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_cfb0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x28c0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_cfb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x28d0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cfc0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x28e0
void iconv_open(void)
{
  (*dat_cfc8)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x28f0
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_cfd0)(); // jump-as-call
  return v1;
}

// Function: main @ 0x2900
int main(int argc,char **argv,char **envp)
{
  char *v1;
  int v10; // eax
  int v11; // eax
  long v12;
  long *v13; // rax
  char *v14;
  char *v15;
  char *v16;
  char *v17; // rax
  long v18; // rax
  long v19; // rax
  unsigned char v2;
  long v20;
  long v21;
  unsigned long v22;
  char *v23;
  unsigned long v24; // rcx
  unsigned long v25; // rcx
  long *v26;
  long *v27;
  char *v28; // stack - 0x1b0
  char v29 [24];
  unsigned char v3;
  char v30 [5];
  char v31 [32];
  stat v32; // stack - 0x1a8
  char v33 [5];
  stat v34; // stack - 0x118
  char *v35;
  long *v36;
  unsigned int v37; // ebp
  char *v38;
  char *v39; // stack - 0x210
  char v4;
  char *v40; // stack - 0x208
  int v41; // stack - 0x200
  int v42; // stack - 0x1ec
  unsigned long v43; // stack - 0x1e8
  bool v44; // stack - 0x1bd
  int v45; // stack - 0x1bc
  FILE *v5;
  int v6;
  int v7; // eax
  int v8; // eax
  int v9; // eax
  
  dat_d020 = 2;
  sub_4080(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_3e90(NULL);
  sub_6870();
  while( true ) {
    v24 = 0xca60;
    v6 = getopt_long(argc,argv,"bci:ln:sv",(option *)0xca60,NULL);
    v22 = dat_1d280;
    if (v6 == -1) break;
    switch(v6) {
      case 0x62:
      case 99:
        dat_d0b2 = '\x01';
        break;
      default:
        sub_3a80(NULL,0); // no-return
      case 0x69:
        sub_3ae0(0,&optarg,0x3a);
        v15 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v15;
          if (dat_d0c8 < dat_d0c0)
            dat_d0c8 = dat_d0c0;
        }
        else {
          optarg = v15;
          sub_3ae0(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_3c60(1);
        break;
      case 0x6e:
        v6 = sub_6940(optarg,NULL,0,&v28,"kKMGTPEZY0");
        if ((v6) || ((long)v28 < 0))
          sub_3a80("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_d010 < 0) || ((long)v28 <= (long)dat_d010))
          dat_d010 = v28;
        break;
      case 0x73:
        sub_3c60(3);
        break;
      case 0x76:
        v22 = sub_4520("David MacKenzie");
        sub_61b0(stdout,"cmp","GNU diffutils",dat_d018,sub_45b0("Torbjorn Granlund",0x962a),v22,0,v24);
        sub_3c90();
        return 0;
      case 0x80:
        v27 = (long *)0xca00;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5),v22);
        __printf_chk(1,"%s\n",dcgettext(NULL,"Compare two files byte by byte.",5));
        __printf_chk(1,"\n%s\n\n",dcgettext(NULL,"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).",5));
        v5 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v5);
        v15 = "-b, --print-bytes          print differing bytes";
        do {
          v27 = &v27[1];
          __printf_chk(1,"  %s\n",dcgettext(NULL,v15,5));
          v15 = (char *)*v27;
        } while (v15);
        v15 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
        v14 = dcgettext(NULL,"If a FILE is \'-\' or missing, read standard input.",5);
        __printf_chk(1,"\n%s\n\n%s\n%s\n",dcgettext(NULL,"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",5),v14,v15);
        sub_6290();
        sub_3c90();
        return 0;
      
    }
  }
  if (optind == argc)
    sub_3a80("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  dat_d230 = argv[optind];
  dat_d238 = "-";
  v6 = optind + 1;
  if (optind + 1 < argc) {
    dat_d238 = argv[(long)optind + 1];
    v6 = optind + 2;
  }
  optind = v6;
  v12 = (long)optind;
  if (optind < argc) {
    optind += 1;
    v28 = argv[v12];
    sub_3ae0(0,&v28,0);
    v12 = (long)optind;
    if (optind < argc) {
      optind += 1;
      v28 = argv[v12];
      sub_3ae0(1,&v28,0);
      if (optind < argc)
        sub_3a80("extra operand \'%s\'",argv[optind]); // no-return
    }
  }
  v12 = 0;
  do {
    if (((v12) && (dat_d0c0 == dat_d0c8)) && (!strcmp(dat_d230,dat_d238)))
      return 0;
    v15 = *(char **)(v12 * 8 + 0xd230);
    if (strcmp(v15,"-")) { // branch-flip
      v6 = open(v15,0,0);
      *(int *)(v12 * 4 + 0xd220) = v6;
      if (v6 <= -1) {
        if (dat_d0b4 == 3)
          exit(2); // no-return
        error(2,*__errno_location(),"%s",*(char **)((long)(int)v12 * 8 + 0xd230)); // return-dupe
        return v8;
      }
    }
    else {
      *(unsigned int *)(v12 * 4 + 0xd220) = 0;
      v6 = 0;
    }
    v45 = fstat(v6,(stat *)(v12 * 0x90 + 0xd100));
    if (v45) {
      error(2,*__errno_location(),"%s",*(char **)((long)(int)v12 * 8 + 0xd230));
      return v8;
    }
    if (v12 == 1) goto label_2da0;
    v12 = 1;
  } while( true );
label_2da0:
  if ((dat_d108 != dat_d198) || (dat_d100 != dat_d190)) {
    if ((dat_d118 & 0xf000) != 0x6000) {
      if ((dat_d118 & 0xf000) == 0x2000) {
        if ((dat_d1a8 & 0xf000) == 0x2000) goto label_3531;
        goto label_323a;
      }
      goto label_2ddb;
    }
    if ((dat_d1a8 & 0xf000) == 0x6000) {
label_3531:
      if (dat_d128 == dat_d1b8) goto label_3120;
      goto label_2ddb;
    }
label_323a:
    if (dat_d0b4 == 3) goto label_2e10;
label_3247:
    if ((fstat(1,&v32)) || (stat("/dev/null",&v34))) goto label_325e;
    if ((v32._8_8_ != v34._8_8_) || (v32._0_8_ != v34._0_8_)) {
      if ((v32._24_4_ & 0xf000) != 0x6000) { // branch-flip
        if (((v32._24_4_ & 0xf000) == 0x2000) && ((v34._24_4_ & 0xf000) == 0x2000)) {
label_3649:
          if (v32._40_8_ == v34._40_8_) goto label_365f;
        }
      }
      else if ((v34._24_4_ & 0xf000) == 0x6000) goto label_3649;
    }
    else {
label_365f:
      dat_d0b4 = 2;
    }
label_325e:
    if (dat_d0b4 != 3) goto label_2e10;
  }
  else {
label_3120:
    if (((dat_d118 == dat_d1a8) && (dat_d110 == dat_d1a0)) && ((dat_d11c == dat_d1ac && ((((dat_d120 == dat_d1b0 && (dat_d130 == dat_d1c0)) && (dat_d158 == dat_d1e8)) && (dat_d168 == dat_d1f8)))))) {
      v12 = sub_3b80(0);
      if (v12 == sub_3b80(1))
        return 0;
    }
label_2ddb:
    if (dat_d0b4 != 3) goto label_3247;
  }
  v12 = dat_d130;
  if (((dat_d118 & 0xf000) == 0x8000) && ((dat_d1a8 & 0xf000) == 0x8000)) {
    v18 = sub_3b80(0);
    v21 = dat_d1c0;
    v19 = sub_3b80(1);
    v20 = 0;
    if (0 <= v12 - v18)
      v20 = v12 - v18;
    v21 -= v19;
    if (v21 <= -1)
      v21 = 0;
    if (v21 != v20) {
      if (0 <= (long)dat_d010) {
        if (v20 < v21)
          v21 = v20;
        if ((long)dat_d010 <= v21) goto label_2e10;
      }
      exit(1); // no-return
    }
  }
label_2e10:
  dat_d0d0 = (char *)sub_7070(dat_d138,dat_d1c8,(void *)0x7ffffffffffffff7);
  v15 = &dat_d0d0[0xf];
  v13 = (long *)sub_63b0(((unsigned long)v15 >> 3) << 4);
  v12 = dat_d130;
  v27 = (long *)((long)v13 + ((unsigned long)v15 & 0xfffffffffffffff8));
  v40 = dat_d010;
  dat_d0e0 = v13;
  dat_d0e8 = v27;
  if (dat_d0b4 == 1) {
    v15 = (char *)0x7fffffffffffffff;
    if (0 <= (long)dat_d010)
      v15 = dat_d010;
    if ((dat_d118 & 0xf000) == 0x8000) {
      v14 = (char *)(v12 - sub_3b80(0));
      if ((long)v14 < (long)v15)
        v15 = v14;
    }
    v12 = dat_d1c0;
    if ((dat_d1a8 & 0xf000) == 0x8000) {
      v14 = (char *)(v12 - sub_3b80(1));
      if ((long)v14 < (long)v15)
        v15 = v14;
    }
    v42 = 1;
    while (v15 = (char *)((long)v15 / 10), v15) {
      v42 += 1;
    }
  }
  v12 = 0;
  do {
    v15 = *(char **)(v12 * 8 + 0xd0c0);
    if ((v15) && (sub_3b80(v12) == -1)) {
      do {
        v14 = dat_d0d0;
        if (v15 <= dat_d0d0)
          v14 = v15;
        v23 = (char *)sub_6fc0(*(unsigned int *)(v12 * 4 + 0xd220),v13,v14);
        if (v14 != v23) {
          if (v23 == (char *)0xffffffffffffffff) {
            error(2,*__errno_location(),"%s",*(char **)((long)(int)v12 * 8 + 0xd230));
            return v7;
          }
          break;
        }
        v15 = &v15[-(long)v14];
      } while (v15);
    }
    if (v12 == 1) goto label_2e9d;
    v12 = 1;
  } while( true );
label_2e9d:
  v41 = 0;
  v39 = (char *)0x1;
  v43 = 1;
  v44 = 1;
label_2ebc:
  v15 = dat_d0d0;
  if (0 <= (long)v40) {
    if (v40 < dat_d0d0)
      v15 = v40;
    v40 = &v40[-(long)v15];
  }
  v14 = (char *)sub_6fc0(dat_d220,v13,v15);
  if (v14 == (char *)0xffffffffffffffff) {
    error(2,*__errno_location(),"%s",dat_d230);
    return v10;
  }
  v15 = (char *)sub_6fc0(dat_d224,v27,v15);
  if (v15 == (char *)0xffffffffffffffff) {
    error(2,*__errno_location(),"%s",dat_d238);
    return v11;
  }
  v23 = v15;
  if (v14 <= v15)
    v23 = v14;
  v6 = memcmp(v13,v27,(unsigned long)v23);
  if (v6) { // branch-flip
    if ((v14 < v15) || (*(unsigned char *)((long)v27 + (long)v14) = 0x55, v14 == v15))
      *(char *)((long)v13 + (long)v15) = 'y';
    *(unsigned char *)((long)v13 + (long)v14) = ~*(unsigned char *)((long)v27 + (long)v14);
    *(unsigned char *)((long)v27 + (long)v15) = ~*(unsigned char *)((long)v13 + (long)v15);
    v25 = (unsigned long)(~(unsigned int)v14 & 7);
    v35 = &((char *)((long)v13 + 1))[(long)v14];
    while (v25) {
      v1 = &v35[1];
      *v35 = 0;
      v25 -= 1;
      v35 = v1;
    }
    v25 = (unsigned long)(~(unsigned int)v15 & 7);
    v35 = &((char *)((long)v27 + 1))[(long)v15];
    while (v25) {
      v1 = &v35[1];
      *v35 = 0;
      v25 -= 1;
      v35 = v1;
    }
    v36 = v27;
    for (v26 = v13; *v26 == *v36; v26 = &v26[1]) {
      v36 = &v36[1];
    }
    v12 = 0;
    do {
      v21 = v12;
      v12 = v21 + 1;
    } while (*(char *)((long)v26 + v21) == *(char *)((long)v36 + v21));
    v38 = (char *)((long)v26 + (v21 - (long)v13));
    v16 = &v39[(long)v38];
    if ((v38) && (!dat_d0b4)) goto label_32fc;
    if (v23 <= v38) goto label_328b;
    if (dat_d0b4 != 2) {
      if (3 <= dat_d0b4) {
        if (dat_d0b4 != 3) goto label_328b;
        goto label_3380;
      }
      if (!dat_d0b4) goto label_3496;
      do {
        v2 = *(unsigned char *)((long)v13 + (long)v38);
        v3 = *(unsigned char *)((long)v27 + (long)v38);
        if (v2 != v3) {
          v16 = (char *)sub_3fe0(&v38[(long)v39],v29);
          v37 = (unsigned int)v2;
          if (dat_d0b2) { // branch-flip
            sub_3bd0(v30,v37);
            sub_3bd0(v31,v3);
            __printf_chk(1,"%*s %3o %-4s %3o %s\n",v42,v16,v37,v30,(unsigned int)v3,v31);
          }
          else {
            __printf_chk(1,"%*s %3o %3o\n",v42,v16,v37,(unsigned int)v3);
          }
        }
        v38 = &v38[1];
      } while (v23 != v38);
      v41 = -1;
      v16 = &v23[(long)v39];
      goto label_328b;
    }
    if (v14 == v15) {
      v41 = 1;
      goto label_32a1;
    }
  }
  else {
    v16 = &v39[(long)v23];
    if ((!v23) || (dat_d0b4)) goto label_328b;
    v38 = v23;
label_32fc:
    v39 = v16;
    v16 = (char *)((long)v13 + (long)v38);
    v4 = *v16;
    v12 = 0;
    *v16 = '\n';
    v26 = v13;
    while (v17 = rawmemchr(v26,10), v16 != v17) {
      v12 += 1;
      v26 = (long *)&v17[1];
    }
    v43 += v12;
    *v16 = v4;
    v16 = v39;
    if (v38 < v23) {
label_3496:
      v39 = v16;
      v15 = (char *)sub_3fe0(v39,v31);
      v14 = (char *)sub_3fe0(v43,v29);
      if (dat_d0b2) { // branch-flip
        v2 = *(unsigned char *)((long)v13 + (long)v38);
        v3 = *(unsigned char *)((long)v27 + (long)v38);
        sub_3bd0(v33,v2);
        sub_3bd0(v30,(unsigned int)v3);
        v16 = dat_d238;
        v23 = dat_d230;
        __printf_chk(1,dcgettext(NULL,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5),v23,v16,v15,v14,(unsigned int)v2,v33,(unsigned int)v3,v30);
      }
      else {
        v23 = dcgettext(NULL,"%s %s differ: byte %s, line %s\n",5);
        if ((v23 == "%s %s differ: byte %s, line %s\n") && (!sub_3f50(5)))
          v23 = "%s %s differ: char %s, line %s\n";
        __printf_chk(1,v23,dat_d230,dat_d238,v15,v14);
      }
      goto label_3380;
    }
    v44 = ((char *)((long)v13 + -1))[(long)v38] == '\n';
label_328b:
    v39 = v16;
    if (v14 == v15) {
      if (v14 != dat_d0d0) {
label_32a1:
        v37 = (unsigned int)(v41 != 0);
label_32ac:
        if (!close(dat_d220)) {
          if (!close(dat_d224)) {
            if ((v37) && (dat_d0b4 < 2))
              sub_3c90();
            exit(v37); // no-return
          }
          v45 = 1;
        }
        error(2,*__errno_location(),"%s",*(char **)((long)v45 * 8 + 0xd230));
        return v9;
      }
      goto label_2ebc;
    }
    if (dat_d0b4 == 3) goto label_3380;
    v15 = *(char **)((unsigned long)(v15 < v14) * 8 + 0xd230);
    if (v39 == (char *)0x1) {
      v37 = 1;
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s which is empty\n",5),v15);
      goto label_32ac;
    }
    v14 = (char *)sub_3fe0(&v39[-1],v31);
    if (dat_d0b4) // branch-flip
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s after byte %s\n",5),v15,v14);
    else {
      v22 = sub_3fe0(v43 - v44,v29);
      if (v44) // branch-flip
        v23 = dcgettext(NULL,"cmp: EOF on %s after byte %s, line %s\n",5);
      else {
        v23 = dcgettext(NULL,"cmp: EOF on %s after byte %s, in line %s\n",5);
      }
      __fprintf_chk(stderr,1,v23,v15,v14,v22);
    }
  }
label_3380:
  v37 = 1;
  goto label_32ac;
}

// Function: sub_3990 @ 0x3990
void sub_3990(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_cfd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_39c0 @ 0x39c0
void sub_39c0(void)
{
  return;
}

// Function: _FINI_0 @ 0x3a30
void _FINI_0(void)
{
  if (!dat_d088) {
    if (dat_cff8)
      __cxa_finalize(dat_d008);
    sub_39c0();
    dat_d088 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x3a70
void _INIT_0(void)
{
  return;
}

// Function: sub_3a80 @ 0x3a80
void sub_3a80(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_1d280;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}

// Function: sub_3ae0 @ 0x3ae0
void sub_3ae0(int a0,unsigned long *a1,char a2) // return-dupe
{
  unsigned char *v1;
  int v2; // eax
  long v3; // stack - 0x38
  
  v1 = (unsigned char *)*a1;
  v2 = sub_6940(v1,a1,0,&v3,"kKMGTPEZY0");
  if (((!v2) || ((v2 == 2 && (*(char *)*a1 == a2)))) && (0 <= v3)) {
    if (v3 <= *(long *)((long)a0 * 8 + 0xd0c0))
      return;
    *(long *)((long)a0 * 8 + 0xd0c0) = v3;
    return;
  }
  sub_3a80("invalid --ignore-initial value \'%s\'",v1); // no-return
}

// Function: sub_3b80 @ 0x3b80
unsigned long sub_3b80(int a0)
{
  unsigned long v1; // rax
  long v2; // rbx
  
  v2 = (long)a0;
  if (*(char *)(v2 + 0xd0b0))
    return *(unsigned long *)(v2 * 8 + 0xd0a0);
  *(char *)(v2 + 0xd0b0) = 1;
  v1 = lseek(*(unsigned int *)(v2 * 4 + 0xd220),*(unsigned long *)(v2 * 8 + 0xd0c0),1);
  *(unsigned long *)(v2 * 8 + 0xd0a0) = v1;
  return v1;
}

// Function: sub_3bd0 @ 0x3bd0
void sub_3bd0(unsigned char *a0,unsigned char a1)
{
  unsigned char *v1; // rbp
  
  if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)a1 * 2) & 0x40)) {
    if ((char)a1 < '\0') {
      a1 += 0x80;
      v1 = &a0[2];
      a0[0] = 0x4d;
      a0[1] = 0x2d;
      a0 = v1;
    }
    if (0x20 <= a1) { // branch-flip
      if (a1 == 0x7f) {
        *a0 = 0x5e;
        a1 = 0x3f;
        a0 = &a0[1];
      }
    }
    else {
      *a0 = 0x5e;
      a1 += 0x40;
      a0 = &a0[1];
    }
  }
  *a0 = a1;
  a0[1] = 0;
}

// Function: sub_3c60 @ 0x3c60
void sub_3c60(int a0)
{
  if ((dat_d0b4) && (dat_d0b4 != a0))
    sub_3a80("options -l and -s are incompatible",0); // no-return
  dat_d0b4 = a0;
}

// Function: sub_3c90 @ 0x3c90
void sub_3c90(void)
{
  char *v1; // rax
  
  if (*(unsigned char *)stdout & 0x20) {
    error(2,0,"%s",dcgettext(NULL,"write failed",5));
    return;
  }
  if (!fclose(stdout))
    return;
  v1 = dcgettext(NULL,"standard output",5);
  error(2,*__errno_location(),"%s",v1);
}

// Function: sub_3d10 @ 0x3d10
void sub_3d10(void)
{
  return;
}

// Function: sub_3d20 @ 0x3d20
void sub_3d20(int a0)
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
  (*dat_d260)();
  if (a0) // branch-flip
    v6 = dat_d258;
  else {
    v6 = dat_d250;
  }
  v1 = strlen(dat_d248);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_d248,v1);
    write(2,": C",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_d248,v1,0x1002);
    *(unsigned short *)v3 = s_984c._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_d020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_3e40 @ 0x3e40
unsigned long sub_3e40(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_3d20(0xb); // no-return
}

// Function: sub_3e60 @ 0x3e60
void sub_3e60(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_d240 == 0) & 0xb);
  sub_3d20(a0); // no-return
}

// Function: sub_3e90 @ 0x3e90
unsigned long sub_3e90(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_3d10;
  dat_d260 = a0;
  dat_d258 = dcgettext(NULL,"program error",5);
  dat_d250 = dcgettext(NULL,"stack overflow",5);
  dat_d248 = sub_3f40();
  if (!sub_4d00(sub_3e60,0xd280,0x10000)) {
    dat_d240 = sub_4c80(sub_3e40);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_3f40 @ 0x3f40
unsigned long sub_3f40(void)
{
  return __progname;
}

// Function: sub_3f50 @ 0x3f50
bool sub_3f50(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_48b0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_3fe0 @ 0x3fe0
char * sub_3fe0(unsigned long a0,void *a1)
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

// Function: sub_4080 @ 0x4080
void sub_4080(char *a0)
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
    dat_1d280 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_4120 @ 0x4120
void sub_4120(struct_2 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x9e60) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_4161;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_70e0(&a0->field_0x24,v2,sub_5660(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
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
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (a0->field_0x24)
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0->field_0x20 = '\x01';
  if (mbsinit((mbstate_t *)&a0->field_0x1[3])) {
    a0->field_0x0 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
label_4161:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_4290 @ 0x4290
unsigned long sub_4290(char *a0,char *a1)
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
  
  v3 = (char *)sub_56d0(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_7a80(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_43c1:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = '\0';
          sub_4120(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_43c1;
        v8 = (unsigned char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),iswalnum(v14) == 0);
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v17 = v3;
      v7[0] = 0;
      v15 = 0;
      while( true ) {
        v10 = '\0';
        v16 = 0;
        sub_4120(v7);
        if ((v19) && (!v20)) break;
        if (!v10)
          sub_4120(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      if (!v10)
        sub_4120(v6);
      if (((((v13) && (!v14)) || (!v13)) || (!iswalnum(v14))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = '\0';
      sub_4120(v6);
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

// Function: sub_4520 @ 0x4520
char * sub_4520(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_4290(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_63b0(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_45b0 @ 0x45b0
char * sub_45b0(char *a0,char *a1)
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
  void *v8; // rax
  unsigned long *v9; // rbx
  
  v3 = dcgettext(NULL,a0,5);
  v4 = (char *)sub_71e0();
  v6 = v3;
  if (sub_7170(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_6910(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_63b0(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)(v5 + (long)v8);
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_6910(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_47b3;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_461a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_47b3:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_461a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_4290(v3,a0)) {
      a0 = v7;
label_463f:
      if (((!v11) || (!sub_4290(v3,v11))) && ((!a1 || (!sub_4290(v3,a1))))) goto label_46bc;
      if (!v7) goto label_466d;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_461a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_4290(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_466d;
      goto label_463f;
    }
    if (!v2)
      return a0;
    if (sub_4290(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_46bc:
    v5 = strlen(v3);
    v6 = (char *)sub_63b0(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_466d:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_48b0 @ 0x48b0
unsigned long sub_48b0(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_4960 @ 0x4960
void sub_4960(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_4970 @ 0x4970
void sub_4970(unsigned long a0,void *a1,void *a2)
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
  if (dat_1d288) { // branch-flip
    v4 = (*dat_1d288)(v1,0);
    if (v4)
      return;
    if (dat_1d2a0) goto label_49be;
  }
  else {
    if (!dat_1d2a0) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_49be:
    v3 = dat_1d2a8;
    v2 = *(unsigned long *)((long)a2 + 0xa0);
    if (dat_1d2a8) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_5240(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_4aa5;
        }
        else if ((*v9)(v1,&v7)) {
label_4aa5:
          (*dat_1d2a0)(dat_1d298 <= v2 && v2 <= dat_1d298 + dat_1d290,a2);
        }
      }
    }
  }
  if ((dat_1d288) && (v4 = (*dat_1d288)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_4b00 @ 0x4b00
void sub_4b00(void)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  int v3; // stack - 0x20
  
  v1 = sub_4970;
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
  v3 = (-(unsigned int)(dat_1d2a0 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,(sigaction *)&v1,NULL);
}

// Function: sub_4c80 @ 0x4c80
unsigned long sub_4c80(unsigned long a0)
{
  dat_1d288 = a0;
  sub_4b00();
  return 0;
}

// Function: sub_4ca0 @ 0x4ca0
void sub_4ca0(void)
{
  dat_1d288 = 0;
  if (dat_1d2a0)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_4cd0 @ 0x4cd0
unsigned long sub_4cd0(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_4d00 @ 0x4d00
long sub_4d00(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x58
  char v4 [4];
  unsigned int v5; // stack - 0x50
  unsigned int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x48
  
  if (!dat_1d2a8) {
    v1 = sub_5240(v4,&v3);
    if (0 <= v1) { // branch-flip
      v2 = CONCAT44(v6,v5) + -1;
      dat_1d2a8 = v2;
    }
    else {
      v2 = dat_1d2a8;
    }
    if (!v2) {
      v2 = 0xffffffff;
      return v2;
    }
  }
  v5 = 0;
  dat_1d290 = a2;
  dat_1d298 = a1;
  dat_1d2a0 = a0;
  v3 = a1;
  v7 = a2;
  v1 = sigaltstack(&v3,NULL);
  if (v1 <= -1) {
    v2 = 0xffffffff;
    return v2;
  }
  sub_4b00();
  v2 = 0;
  return v2;
}

// Function: sub_4de0 @ 0x4de0
void sub_4de0(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_1d2a0 = 0;
  if (dat_1d288) // branch-flip
    sub_4b00();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_4e70 @ 0x4e70
bool sub_4e70(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_4e90 @ 0x4e90
unsigned long sub_4e90(struct_1 *a0,long *a1)
{
  unsigned char v1;
  long v2;
  long v3; // rcx
  int v4; // esi
  
  v4 = 0;
  v3 = 0;
  while (v2 = a0->field_0x0, v2 != a0->field_0x8) {
    v1 = *(unsigned char *)(a0->field_0x18 + v2);
    if (10 <= (unsigned int)v1 - 0x30) { // branch-flip
      if (6 <= (unsigned int)v1 - 0x41) { // branch-flip
        if (5 < (unsigned int)v1 - 0x61) goto label_4f10;
        v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x57);
      }
      else {
        v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x37);
      }
    }
    else {
      v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x30);
    }
    v4 += 1;
    a0->field_0x0 = v2 + 1;
  }
  a0->field_0x10 = 1;
label_4f10:
  if (v4) {
    *a1 = v3;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_4f20 @ 0x4f20
unsigned long sub_4f20(unsigned long a0,long *a1) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  long v4; // rax
  unsigned long v5; // rax
  long v6;
  void *v7;
  unsigned long v8;
  unsigned long v9; // stack - 0x58
  
  v1 = dat_1d2b0;
  v3 = a0 * 2 - *a1;
  if (a0 < v3)
    return 0;
  v4 = (v3 / dat_1d2b0) * dat_1d2b0;
  v5 = (((unsigned long)(*a1 - 1U) / dat_1d2b0 + 1) * dat_1d2b0 - v4) / dat_1d2b0;
  v3 = 1;
  if (v5 <= 1)
    return 1;
  do {
    v9 = v3;
    v3 = v9 * 2;
  } while (v3 < v5);
  v9 &= 0x7fffffffffffffff;
  while( true ) {
    v2 = v9;
    v7 = (void *)(v4 + v9 * v1);
    if (v9 < v5) {
      v8 = v9;
      do {
        if (0 <= mincore(v7,v1,&v6))
          return 0;
        v8 += v3 & 0xfffffffffffffffe;
        v7 = (void *)((long)v7 + v9 * v1 * 2);
      } while (v8 < v5);
    }
    if (v9 < 2) break;
    v9 >>= 1;
    v3 = v2;
  }
  return 1;
}

// Function: sub_5060 @ 0x5060
void * sub_5060(void *a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  unsigned long v4; // rsi
  unsigned long v5; // rsi
  void *v6; // r14
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_1d2b0;
    if (!a0)
      return NULL;
    if ((unsigned long)a0 / dat_1d2b0 < v2)
      v2 = (unsigned long)a0 / dat_1d2b0;
    v4 = dat_1d2b0 * v2;
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

// Function: sub_5140 @ 0x5140
void * sub_5140(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  void *v5; // rdi
  
  v1 = dat_1d2b0;
  v5 = (void *)(a0 + dat_1d2b0);
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

// Function: sub_5240 @ 0x5240
unsigned long sub_5240(char *a0,unsigned long *a1)
{
  char *v1;
  char *v10;
  unsigned long v11; // r12
  unsigned long v12;
  char *v13; // stack - 0x98
  char *v14; // stack - 0x90
  long v15; // stack - 0x88
  char *v16; // stack - 0x70
  char *v17; // stack - 0x68
  unsigned long v18; // stack - 0x60
  char *v19; // stack - 0x58
  char *v2;
  char *v20; // stack - 0x50
  char *v3;
  int v4; // eax
  int v5; // eax
  long v6;
  void *v7; // rax
  char *v8;
  char v9 [8];
  
  v4 = open("/proc/self/maps",0);
  if (0 <= v4) {
    v16 = v9;
    v11 = 0;
    v15 = 0;
    v12 = 1;
    v17 = NULL;
    v19 = NULL;
    v20 = NULL;
    do {
      do {
        if (0x104a <= v12) { // branch-flip
          v5 = (int)read(v4,v16,v12);
          if (0 <= v5) {
            if (!v5) goto label_5468;
            if (v12 < (unsigned long)(long)(v5 + 0x1049)) goto label_5300;
            v6 = (long)v5;
            while( true ) {
              v5 = (int)read(v4,&v16[v6],v12 - v6);
              if (v5 <= -1) break;
              if (v12 - v6 < (unsigned long)(long)(v5 + 0x1049)) goto label_5300;
              if (!v5) {
                close(v4);
                v10 = NULL;
                goto label_54d0;
              }
              v6 = v5 + v6;
            }
          }
          if (*__errno_location() != 4) {
label_5468:
            close(v4);
            goto label_53a0;
          }
        }
        else {
label_5300:
          if (v11) { // branch-flip
            v12 *= 2;
            if (!v12) goto label_5468;
            if (v17)
              munmap(v17,v18);
          }
          else {
            v11 = (unsigned long)getpagesize();
            for (v12 = v11; v12 <= 0x1049; v12 = v12 * 2) {
            }
          }
          v17 = (char *)mmap(0,v12,3,0x22,0xffffffff,0);
          if (v17 == (char *)0xffffffffffffffff) {
            close(v4);
            goto label_53b4;
          }
          v20 = &v17[v12];
          v19 = v17;
          v16 = v17;
          v18 = v12;
        }
      } while (0 <= (long)lseek(v4,0,0));
      close(v4);
      v4 = open("/proc/self/maps",0);
    } while (0 <= v4);
label_53a0:
    if (v17)
      munmap(v17,v18);
  }
label_53b4:
  v12 = dat_1d2b0;
  if (!dat_1d2b0) {
    v12 = (unsigned long)getpagesize();
    dat_1d2b0 = v12;
  }
  v6 = ((unsigned long)a0 / v12) * v12;
  *a1 = sub_5060(v6);
  a1[1] = sub_5140(v6);
  v7 = sub_4f20;
label_55b3:
  a1[2] = v7;
  return 0;
label_54d0:
  if (((((int)sub_4e90() < 0) || (v15 == v6)) || (v2 = &v16[v15], v15 = v15 + 1, *v2 != '-')) || ((int)sub_4e90() < 0)) goto label_53a0;
  do {
    if (v6 == v15) break;
    v2 = &v16[v15];
    v15 += 1;
  } while (*v2 != '\n');
  v8 = v14;
  if (v13 <= v19) { // branch-flip
    v1 = &v14[-1];
    if (v1 < &v20[-1]) goto label_5580;
    v3 = v10;
    if ((v13 < v19) && ((v3 = v19, v13 <= a0 && (a0 <= &v19[-1])))) {
      v14 = v19;
      goto label_558e;
    }
    v10 = v3;
    if (v1 <= &v20[-1]) // branch-flip
      v8 = v10;
    else if ((v20 <= a0) && (a0 <= v1)) {
      v13 = v20;
      goto label_558e;
    }
  }
  else {
label_5580:
    if ((v13 <= a0) && (a0 <= &v14[-1])) {
label_558e:
      *a1 = v13;
      a1[1] = v14;
      a1[3] = v10;
      if (v17)
        munmap(v17,v18);
      v7 = sub_4e70;
      goto label_55b3;
    }
  }
  v10 = v8;
  goto label_54d0;
}

// Function: sub_5660 @ 0x5660
void * sub_5660(void *a0,unsigned long a1)
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

// Function: sub_56a0 @ 0x56a0
void sub_56a0(void)
{
  __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
}

// Function: sub_56d0 @ 0x56d0
unsigned char * sub_56d0(char *a0,int a1)
{
  unsigned char v1;
  int v10; // r15d
  unsigned char *v11; // stack - 0x90
  char v12; // stack - 0x74
  unsigned char *v13; // stack - 0x70
  long v14; // stack - 0x68
  char v15; // stack - 0x60
  long v2;
  bool v3;
  unsigned char *v4; // rax
  long *v5; // rax
  mbstate_t v6; // stack - 0x7c
  unsigned int v7 [7]; // stack - 0x5c
  unsigned char *v8;
  unsigned char *v9;
  
  v4 = (unsigned char *)strdup(a0);
  if (!v4)
    sub_6830(); // no-return
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    if (a1) {
      v13 = v4;
      v3 = 0;
      v9 = &v4[strlen((char *)v4)];
      v6 = 0;
      v8 = v4;
      while( true ) {
        v12 = 0;
        if (v8 >= v9) break;
        v12 = 0;
        if (v3) {
label_5b24:
          v14 = sub_70e0(v7,v8,(long)v9 - (long)v8,&v6);
          if (v14 != -1) { // branch-flip
            if (v14 != -2) {
              if (!v14) {
                v14 = 1;
                if (*v13) {
                  v14 = 1; // return-dupe
                  __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next"); // no-return
                }
                if (v7[0]) {
                  v14 = 1;
                  sub_56a0(); // no-return, return-dupe
                }
              }
              v15 = '\x01';
              if (mbsinit(&v6))
                v3 = 0;
              goto label_5ae5;
            }
            v14 = (long)v9 - (long)v13;
            v15 = '\0';
            v12 = 1;
          }
          else {
            v14 = 1;
            v15 = '\0';
            v12 = 1;
          }
label_58c3:
          v8 = v13;
          break;
        }
        if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0x9e60) >> (*v8 & 0x1f) & 1)) {
          if (mbsinit(&v6)) {
            v3 = 1;
            goto label_5b24;
          }
          __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // return-dupe, no-return
        }
        v14 = 1;
        v7[0] = (unsigned int)(char)*v8;
        v15 = '\x01';
label_5ae5:
        v12 = 1;
        if (!iswspace(v7[0])) goto label_58c3;
        v8 = &v13[v14];
        v13 = v8;
      }
      memmove(v4,v8,strlen((char *)v8) + 1);
      if (a1 == 1)
        return v4;
    }
    v13 = v4;
    v10 = 0;
    v3 = 0;
    v9 = &v4[strlen((char *)v4)];
    v6 = 0;
    v8 = v4;
    if (v4 < v9) {
      do {
        v12 = 0;
        if (v3) {
label_583a:
          v14 = sub_70e0(v7,v8,(long)v9 - (long)v8,&v6);
          if (v14 != -1) { // branch-flip
            if (v14 != -2) { // branch-flip
              if (!v14) {
                v14 = 1;
                if (*v13) {
                  v14 = 1;
                  __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next");
                }
                if (v7[0]) {
                  v14 = 1;
                  sub_56a0();
                }
              }
              v15 = '\x01';
              if (mbsinit(&v6))
                v3 = 0;
            }
            else {
              v14 = (long)v9 - (long)v13;
              v15 = '\0';
            }
          }
          else {
            v14 = 1;
            v15 = '\0';
          }
        }
        else {
          if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0x9e60) >> (*v8 & 0x1f) & 1)) {
            if (!mbsinit(&v6))
              __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next");
            v3 = 1;
            goto label_583a;
          }
          v14 = 1;
          v7[0] = (unsigned int)(char)*v8;
          v15 = '\x01';
        }
        v12 = 1;
        if (v10) { // branch-flip
          if (v10 != 1) { // branch-flip
            if ((v10 != 2) || (!v15)) goto label_576b;
            if (!iswspace(v7[0]))
              v10 = 1;
          }
          else if ((((v15) && (iswspace(v7[0]))) && (v15)) && (iswspace(v7[0]))) {
            v11 = v13;
            v10 = 2;
          }
        }
        else if (v15) { // branch-flip
          if (!iswspace(v7[0])) {
            if (!v15) goto label_576b;
            v10 = 1;
            iswspace(v7[0]);
          }
        }
        else {
label_576b:
          v10 = 1;
        }
        v8 = &v13[v14];
        v13 = v8;
      } while (v8 < v9);
      if (v10 == 2)
        *v11 = 0;
    }
  }
  else {
    if (a1) {
      v1 = *v4;
      v9 = v4;
      if (v1) {
        v5 = __ctype_b_loc();
        do {
          if (!(*(unsigned char *)(*v5 + 1 + (unsigned long)v1 * 2) & 0x20)) break;
          v1 = v9[1];
          v9 = &v9[1];
        } while (v1);
      }
      memmove(v4,v9,strlen((char *)v9) + 1);
      if (a1 == 1)
        return v4;
    }
    v9 = &v4[strlen((char *)v4) - 1];
    if (v4 <= v9) {
      v2 = *(long *)__ctype_b_loc();
      do {
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)*v9 * 2) & 0x20))
          return v4;
        *v9 = 0;
        v9 = &v9[-1];
      } while (v4 <= v9);
    }
  }
  return v4;
}

// Function: sub_5c70 @ 0x5c70
long sub_5c70(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_5d9d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_5dd6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x9ca8) + 0x9ca8;
  switch(a5) {
    case 0:
      goto label_5dd6;
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
      goto label_5d9d;
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

// Function: sub_60f0 @ 0x60f0
void sub_60f0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_5c70(); // tail-call
}

// Function: sub_6110 @ 0x6110
void sub_6110(FILE *a0,char *a1,char *a2,char *a3,struct_3 *a4)
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
  sub_5c70(a0,a1,a2,a3);
}

// Function: sub_61b0 @ 0x61b0
void sub_61b0(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_5c70(a0,a1,a2);
}

// Function: sub_6290 @ 0x6290
void sub_6290(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_6340 @ 0x6340
void sub_6340(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_6830(); // no-return
}

// Function: sub_6370 @ 0x6370
void sub_6370(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_6830(); // no-return
}

// Function: sub_63b0 @ 0x63b0
void sub_63b0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_6830(); // no-return
}

// Function: sub_63d0 @ 0x63d0
void sub_63d0(void)
{
  sub_63b0(); // tail-call
}

// Function: sub_63e0 @ 0x63e0
void sub_63e0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_6830(); // no-return
}

// Function: sub_6400 @ 0x6400
void sub_6400(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_6830(); // no-return
}

// Function: sub_6440 @ 0x6440
void sub_6440(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_6830(); // no-return
}

// Function: sub_6470 @ 0x6470
void sub_6470(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_6830(); // no-return
}

// Function: sub_64b0 @ 0x64b0
void sub_64b0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_6830(); // no-return
}

// Function: sub_64f0 @ 0x64f0
void sub_64f0(void *a0,unsigned long *a1)
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
  sub_6830(); // no-return
}

// Function: sub_6580 @ 0x6580
void sub_6580(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_6830(); // no-return
}

// Function: sub_6610 @ 0x6610
void sub_6610(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_66ce;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_66ce:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_6830(); // no-return
}

// Function: sub_6710 @ 0x6710
void sub_6710(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_6830(); // no-return
}

// Function: sub_6730 @ 0x6730
void sub_6730(unsigned long a0)
{
  sub_6710(a0,1); // tail-call
}

// Function: sub_6740 @ 0x6740
void sub_6740(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_6830(); // no-return
}

// Function: sub_6760 @ 0x6760
void sub_6760(unsigned long a0)
{
  sub_6740(a0,1); // tail-call
}

// Function: sub_6770 @ 0x6770
void sub_6770(char *a0,unsigned long a1)
{
  memcpy((void *)sub_63b0(a1),a0,a1); // tail-call
}

// Function: sub_67a0 @ 0x67a0
void sub_67a0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_63e0(a1),a0,a1); // tail-call
}

// Function: sub_67d0 @ 0x67d0
void sub_67d0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_63e0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_6810 @ 0x6810
void sub_6810(char *a0)
{
  sub_6770(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_6830 @ 0x6830
void sub_6830(void)
{
  error(dat_d020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_6870 @ 0x6870
void sub_6870(void)
{
  int v1; // eax
  
  v1 = sub_8150();
  if (!v1)
    return;
  error(dat_d020,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_68b0 @ 0x68b0
int sub_68b0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_8200(a0,a1,a2,a3,a4);
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_6830(); // no-return
  }
  return v1;
}

// Function: sub_68e0 @ 0x68e0
long sub_68e0(void)
{
  long v1; // rax
  
  v1 = sub_8480();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_6830(); // no-return
  }
  return v1;
}

// Function: sub_6910 @ 0x6910
long sub_6910(char *a0,long a1,long a2)
{
  long v1; // rax
  
  v1 = sub_86d0(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_6830(); // no-return
  }
  return v1;
}

// Function: sub_6940 @ 0x6940
unsigned int sub_6940(unsigned char *a0,unsigned long *a1,unsigned int a2,long *a3,char *a4)
{
  unsigned char v1;
  char v10 [16];
  char v11 [16];
  char v12 [16];
  char v13 [16];
  char v14 [16];
  char v15 [16];
  char v16 [16];
  char v17 [16];
  char v18 [16];
  int *v19; // rax
  unsigned char v2;
  long v20;
  long v21; // rax
  unsigned long v22; // rax
  unsigned long v23; // rcx
  long v24;
  int v25; // esi
  int v26; // r8d
  unsigned int v27; // r10d
  unsigned int v28;
  unsigned char *v3;
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x54,"xstrtoimax"); // no-return
  if (!a1)
    a1 = &v24;
  v19 = __errno_location();
  *v19 = 0;
  v20 = strtoimax(a0,a1,a2);
  v3 = (unsigned char *)*a1;
  if (v3 != a0) { // branch-flip
    if (*v19) { // branch-flip
      if (*v19 != 0x22)
        return 4;
      v28 = 1;
    }
    else {
      v28 = 0;
    }
    if ((!a4) || (v1 = *v3, !v1)) goto label_69cb;
    if (!strchr(a4,(int)(char)v1)) goto label_69d3;
  }
  else {
    if (!a4)
      return 4;
    v1 = *v3;
    if (!v1)
      return 4;
    v28 = 0;
    v20 = 1;
    if (!strchr(a4,(int)(char)v1))
      return 4;
  }
  v26 = 1;
  v23 = 0x400;
  if (((unsigned char)((unsigned int)v1 - 0x45) <= 0x2f) && (0x814400308945U >> ((unsigned long)((unsigned int)v1 - 0x45) & 0x3f) & 1)) {
    v23 = 0x400;
    v26 = 1;
    if (strchr(a4,0x30)) {
      v2 = v3[1];
      if (v2 != 0x44) { // branch-flip
        if (v2 != 0x69) { // branch-flip
          v26 = (v2 == 0x42) + 1;
          if (v2 == 0x42)
            v23 = 1000;
        }
        else {
          v26 = (v3[2] == 0x42) + 1 + (unsigned int)(v3[2] == 0x42);
        }
      }
      else {
        v26 = 2;
        v23 = 1000;
      }
    }
  }
  switch(v1) {
    case 0x42:
      if (v20 < -0x20000000000000) goto label_6ca0;
      if (0x20000000000000 <= v20) { // branch-flip
label_6f70:
        v20 = 0x7fffffffffffffff;
        v28 = 1;
      }
      else {
        v20 <<= 10;
      }
      break;
    default:
label_69d3:
      *a3 = v20;
      return v28 | 2;
    case 0x45:
      v25 = 6;
      v27 = 0;
      v13._8_8_ = 0;
      v13._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      do {
        if (SUB168((v13._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0) <= v20) { // branch-flip
          v14._8_8_ = 0;
          v14._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
          if (v20 <= SUB168((v14._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0)) // branch-flip
            v20 *= v23;
          else {
            v20 = 0x7fffffffffffffff;
            v27 = 1;
          }
        }
        else {
          v20 = -0x8000000000000000;
          v27 = 1;
        }
        v25 -= 1;
      } while (v25);
      goto label_6b40;
    case 0x47:
    case 0x67:
      v25 = 3;
      v27 = 0;
      v7._8_8_ = 0;
      v7._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      do {
        if (SUB168((v7._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0) <= v20) { // branch-flip
          v8._8_8_ = 0;
          v8._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
          if (v20 <= SUB168((v8._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0)) // branch-flip
            v20 *= v23;
          else {
            v20 = 0x7fffffffffffffff;
            v27 = 1;
          }
        }
        else {
          v20 = -0x8000000000000000;
          v27 = 1;
        }
        v25 -= 1;
      } while (v25);
      goto label_6b40;
    case 0x4b:
    case 0x6b:
      v6._8_8_ = 0;
      v6._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      if (v20 < SUB168((v6._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0)) goto label_6ca0;
      v22 = 0x7fffffffffffffff / v23;
label_6be8:
      if ((long)v22 < v20) goto label_6f70;
      v20 *= v23;
      break;
    case 0x4d:
    case 0x6d:
      v9._8_8_ = 0;
      v9._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      v21 = SUB168((v9._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0);
      if (v21 <= v20) {
        v10._8_8_ = 0;
        v10._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
        v22 = SUB168((v10._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0);
        if ((long)v22 < v20) goto label_6f70;
        v20 *= v23;
        if (v21 <= v20) goto label_6be8;
      }
label_6ca0:
      v20 = -0x8000000000000000;
      v28 = 1;
      break;
    case 0x50:
      v25 = 5;
      v27 = 0;
      v11._8_8_ = 0;
      v11._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      do {
        if (SUB168((v11._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0) <= v20) { // branch-flip
          v12._8_8_ = 0;
          v12._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
          if (v20 <= SUB168((v12._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0)) // branch-flip
            v20 *= v23;
          else {
            v20 = 0x7fffffffffffffff;
            v27 = 1;
          }
        }
        else {
          v20 = -0x8000000000000000;
          v27 = 1;
        }
        v25 -= 1;
      } while (v25);
      goto label_6b40;
    case 0x54:
    case 0x74:
      v25 = 4;
      v27 = 0;
      v4._8_8_ = 0;
      v4._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      do {
        if (SUB168((v4._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0) <= v20) { // branch-flip
          v5._8_8_ = 0;
          v5._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
          if (v20 <= SUB168((v5._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0)) // branch-flip
            v20 *= v23;
          else {
            v20 = 0x7fffffffffffffff;
            v27 = 1;
          }
        }
        else {
          v20 = -0x8000000000000000;
          v27 = 1;
        }
        v25 -= 1;
      } while (v25);
      goto label_6b40;
    case 0x59:
      v25 = 8;
      v27 = 0;
      v17._8_8_ = 0;
      v17._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      do {
        if (SUB168((v17._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0) <= v20) { // branch-flip
          v18._8_8_ = 0;
          v18._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
          if (v20 <= SUB168((v18._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0)) // branch-flip
            v20 *= v23;
          else {
            v20 = 0x7fffffffffffffff;
            v27 = 1;
          }
        }
        else {
          v20 = -0x8000000000000000;
          v27 = 1;
        }
        v25 -= 1;
      } while (v25);
      goto label_6b40;
    case 0x5a:
      v25 = 7;
      v27 = 0;
      v15._8_8_ = 0;
      v15._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
      do {
        if (SUB168((v15._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816((long)v23),0) <= v20) { // branch-flip
          v16._8_8_ = 0;
          v16._0_8_ = SUB168(SEXT816(0x7fffffffffffffff),8);
          if (v20 <= SUB168((v16._0_16_ << 0x40 | ZEXT816(0x7fffffffffffffff)) / SEXT816((long)v23),0)) // branch-flip
            v20 *= v23;
          else {
            v20 = 0x7fffffffffffffff;
            v27 = 1;
          }
        }
        else {
          v20 = -0x8000000000000000;
          v27 = 1;
        }
        v25 -= 1;
      } while (v25);
label_6b40:
      v28 |= v27;
      break;
    case 0x62:
      if (v20 < -0x40000000000000) goto label_6ca0;
      if (0x3fffffffffffff < v20) goto label_6f70;
      v20 <<= 9;
      break;
    case 99:
      break;
    case 0x77:
      if (v20 < -0x4000000000000000) goto label_6ca0;
      if (0x3fffffffffffffff < v20) goto label_6f70;
      v20 *= 2;
    
  }
  *a1 = &v3[v26];
  if (v3[v26])
    v28 |= 2;
label_69cb:
  *a3 = v20;
  return v28;
}

// Function: sub_6fc0 @ 0x6fc0
long sub_6fc0(int a0,char *a1,char *a2)
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // r14
  char *v4;
  
  v3 = 0x7fffffffffffffff;
  a2 = &a1[(long)a2];
  v4 = a1;
  do {
    while( true ) {
      v1 = (long)a2 - (long)v4;
      if (v3 < (unsigned long)((long)a2 - (long)v4))
        v1 = v3;
      v2 = read(a0,v4,v1);
      if (0 < v2) break;
      if (!v2) {
        return (long)v4 - (long)a1;
      }
      if ((*__errno_location() != 0x16) || (v1 < 0x80000000))
        return -1;
      v3 = 0x7fffffff;
      if (a2 <= v4) {
        return (long)v4 - (long)a1;
      }
    }
    v4 = &v4[v2];
  } while (v4 < a2);
  return (long)v4 - (long)a1;
}

// Function: sub_7070 @ 0x7070
void * sub_7070(void *a0,void *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  void *v3;
  void *v4; // rdx
  void *v5;
  
  if (a0) {
    if (a1) {
      v3 = a0;
      v5 = a1;
      while (v4 = (void *)((unsigned long)v3 % (unsigned long)v5), v4) {
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
  v3 = _DT_INIT;
  if (a1)
    v3 = a1;
  return v3;
}

// Function: sub_70e0 @ 0x70e0
unsigned long sub_70e0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_3f50(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_7170 @ 0x7170
int sub_7170(long a0,long a1)
{
  unsigned int v1;
  unsigned char v2;
  long v3; // rdx
  unsigned char v4;
  unsigned int v5;
  
  if (a0 == a1)
    return 0;
  v3 = 0;
  do {
    v4 = *(unsigned char *)(a0 + v3);
    v1 = (unsigned int)v4;
    v2 = *(unsigned char *)(a1 + v3);
    v5 = (unsigned int)v2;
    if (0x1a <= v1 - 0x41) { // branch-flip
      if (v5 - 0x41 <= 0x19) {
label_718d:
        v5 += 0x20;
        v2 += 0x20;
      }
      if (!v1) break;
    }
    else {
      v1 += 0x20;
      v4 += 0x20;
      if (v5 - 0x41 <= 0x19) goto label_718d;
    }
    v3 += 1;
  } while (v4 == v2);
  return v1 - v5;
}

// Function: sub_71e0 @ 0x71e0
char * sub_71e0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_7220 @ 0x7220
int sub_7220(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_7250 @ 0x7250
void sub_7250(struct_4 *a0,struct_5 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_5 *v3;
  
  v3 = (struct_5 *)a1->field_0x0;
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

// Function: sub_72b0 @ 0x72b0
unsigned int sub_72b0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x9e60) >> (a0 & 0x1f) & 1;
}

// Function: sub_72d0 @ 0x72d0
unsigned long sub_72d0(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_8a00(v9);
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
              goto label_73c8;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_73c8:
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
      *(unsigned long *)&v8[-8] = 0x741e;
      sub_8a50(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_7470 @ 0x7470
void sub_7470(struct_2 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x9e60) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_74b1;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_70e0(&a0->field_0x24,v2,sub_5660(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
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
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (a0->field_0x24)
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0->field_0x20 = '\x01';
  if (mbsinit((mbstate_t *)&a0->field_0x1[3])) {
    a0->field_0x0 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
label_74b1:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_75e0 @ 0x75e0
char sub_75e0(void *a0,unsigned char *a1,unsigned long *a2)
{
  char v1;
  unsigned char v10 [32];
  char v11 [4]; // stack - 0x88
  char v12 [4]; // stack - 0xc8
  void **v13;
  char *v14; // rsp
  void **v15; // rsp
  void **v16;
  unsigned char *v17;
  unsigned long v18;
  unsigned long *v19;
  void *v2;
  unsigned long *v20; // r13
  long v21;
  void *v22;
  unsigned long *v23; // stack - 0xf0
  void *v24; // stack - 0xe8
  char v25; // stack - 0xd9
  void *v26; // stack - 0xd8
  char *v27; // stack - 0xd0
  unsigned long v28; // stack - 0xc4
  char v29; // stack - 0xbc
  char v3 [16];
  void *v30; // stack - 0xb8
  long v31; // stack - 0xb0
  char v32; // stack - 0xa8
  int v33; // stack - 0xa4
  unsigned long v34; // stack - 0x84
  char v35; // stack - 0x7c
  unsigned char *v36; // stack - 0x78
  unsigned long v37; // stack - 0x70
  char v38; // stack - 0x68
  int v39; // stack - 0x64
  unsigned long *v4;
  void *v5;
  char v6; // al
  unsigned int v7; // eax
  unsigned long *v8;
  void *v9; // stack - 0xf8
  
  v15 = &v9;
  v13 = &v9;
  v16 = &v9;
  v9 = a0;
  v23 = a2;
  v8 = (void *)sub_8a80(a1);
  v24 = v8;
  v3 = ZEXT816(v8);
  v8 = SUB168(ZEXT816(0x38) * v3,0);
  if ((0 <= (long)v8) && (!SUB168(ZEXT816(0x38) * v3,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v8 = (unsigned long *)sub_8a00(v18);
      v19 = v8;
      v5 = v24;
    }
    else {
      v16 = &v9;
      while (v15 != (void **)((long)&v9 - (v18 + 0x36 & 0xfffffffffffff000))) {
        v13 = (char *)((long)v16 + -0x1000);
        v14 = (char *)((long)v16 + -0x1000);
        *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
        v15 = (char *)((long)v16 + -0x1000);
        v16 = (void **)v14;
      }
      v7 = (unsigned int)(v18 + 0x36) & 0xff0;
      v8 = (unsigned long)(unsigned long)v7;
      v16 = (char *)((long)v13 + -(long)v8);
      if (v8)
        *(unsigned long *)((long)v13 + -8) = *(unsigned long *)((long)v13 + -8);
      v19 = (unsigned long *)(((unsigned long)((long)v13 + -(long)v8 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v5 = v24;
    }
    if (v19) {
      v8 = &v19[3];
      v11[0] = 0;
      v27 = v11;
      v34 = 0;
      v24 = v5;
      v36 = a1;
      while( true ) {
        v35 = 0;
        *(unsigned long *)((long)v16 + -8) = 0x776e;
        sub_7470(v27);
        v25 = v38;
        if ((v38) && (!v39)) break;
        if (v36 != v10) // branch-flip
          v8[-3] = v36;
        else {
          *(unsigned long *)((long)v16 + -8) = 0x779b;
          memcpy(v8,v10,v37);
          v8[-3] = v8;
        }
        v8[-2] = v37;
        *(char *)&v8[-1] = v38;
        if (v38)
          *(int *)((long)v8 + -4) = v39;
        v36 = &v36[v37];
        v8 = &v8[6];
      }
      v21 = 0;
      v19[(long)v5 * 6 + 1] = 1;
      v26 = (void *)0x2;
      v4 = v19;
      if ((void *)0x3 <= v24) {
label_77e0:
        v20 = &v4[6];
        v1 = *(char *)&v4[8];
        do {
          v8 = &v19[v21 * 6];
          if ((v1) && (*(char *)&v8[2])) {
            if (*(int *)((long)v4 + 0x44) == *(int *)((long)v8 + 0x14)) {
label_7836:
              v21 += 1;
              v8 = (long)((long)v26 - v21);
              v19[(long)((long)v5 * 6 + (long)v26)] = v8;
              goto label_784b;
            }
          }
          else {
            v18 = v4[7];
            if (v18 == v8[1]) {
              v22 = (void *)*v8;
              v2 = (void *)*v20;
              *(unsigned long *)((long)v16 + -8) = 0x7832;
              v7 = memcmp(v2,v22,v18);
              if (!v7) goto label_7836;
            }
          }
          if (!v21) goto label_7a40;
          v21 -= v19[(long)v5 * 6 + v21];
        } while( true );
      }
label_786b:
      v11[0] = 0;
      v22 = NULL;
      v12[0] = 0;
      *v23 = 0;
      v28 = 0;
      v29 = '\0';
      v34 = 0;
      v35 = '\0';
      v30 = v9;
      v36 = v9;
      v6 = 0;
      v1 = v6;
label_78c0:
      v6 = v1;
      if (!v6) goto label_790f;
      if (!v38) goto label_7921;
label_78ca:
      if (v39) {
        v8 = &v19[(long)v22 * 6];
        if (!*(char *)&v8[2]) goto label_792c;
        if (*(int *)((long)v8 + 0x14) != v39) goto label_7936;
        v17 = v36;
        v18 = v37;
label_78f3:
        v36 = (void *)((long)v17 + v18);
        v35 = '\0';
        v22 = (void *)((long)v22 + 1);
        if (v24 != v22) {
label_790f:
          do {
            *(unsigned long *)((long)v16 + -8) = 0x791b;
            sub_7470(v27);
            if (v38) goto label_78ca;
label_7921:
            v8 = &v19[(long)v22 * 6];
label_792c:
            v18 = v37;
            if (v8[1] == v37) {
              v2 = (void *)*v8;
              v26 = v36;
              *(unsigned long *)((long)v16 + -8) = 0x79a6;
              v7 = memcmp(v2,v36,v37);
              v17 = v26;
              if (!v7) goto label_78f3;
            }
label_7936:
            if (v22) {
              v21 = v19[(long)((long)v5 * 6 + (long)v22)];
              v8 = (void *)((long)v22 - v21);
              v1 = v35;
              v26 = v8;
              if (!v21) goto label_78c0;
              v22 = v8;
              if (!v29) goto label_7a15;
              goto label_79ee;
            }
            if (!v29) {
              *(unsigned long *)((long)v16 + -8) = 0x7950;
              sub_7470(v12);
            }
            if ((v32) && (!v33)) goto label_7a6e;
            v29 = '\0';
            v30 = (void *)((long)v30 + v31);
            v35 = '\0';
            v36 = (void *)((long)v36 + v37);
          } while( true );
        }
        *v23 = (unsigned long)v30;
      }
      *(unsigned long *)((long)v16 + -8) = 0x7a69;
      sub_8a50(v19);
      v6 = v25;
      return v6;
    }
  }
  v6 = '\0';
  return v6;
label_7a40:
  v19[(long)((long)v5 * 6 + (long)v26)] = v26;
label_784b:
  v26 = (void *)((long)v26 + 1);
  v4 = v20;
  if (v24 == v26) goto label_786b;
  goto label_77e0;
label_79ee:
  v26 = v22;
  if ((v32) && (!v33)) {
label_7a6e:
    *(unsigned long *)((long)v16 + -8) = 0x7a73; // no-return
    abort();
  }
  v29 = '\0';
  v30 = (void *)((long)v30 + v31);
  v21 -= 1;
  if (!v21) goto label_7a30;
label_7a15:
  *(unsigned long *)((long)v16 + -8) = 0x7a1d;
  sub_7470(v12);
  v22 = v26;
  goto label_79ee;
label_7a30:
  v1 = v35;
  goto label_78c0;
}

// Function: sub_7a80 @ 0x7a80
char * sub_7a80(char *a0,char *a1) // early-return, return-dupe
{
  char *v1;
  char *v10; // stack - 0x190
  char v11 [4]; // stack - 0x188
  char v12 [4]; // stack - 0x88
  unsigned int v13; // stack - 0x108
  unsigned long v14; // stack - 0xc8
  char v15 [4]; // stack - 0x148
  unsigned long v16;
  long v17;
  unsigned long v18; // stack - 0x1e0
  unsigned char v19; // stack - 0x1d1
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
  char v43 [5]; // stack - 0xc0
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
    sub_7470(v11);
    if ((!v25) || (v26)) {
      v19 = 1;
      v15[0] = 0;
      v27 = 0;
      v28 = '\0';
      v13 &= 0xffffff00;
      v34 = 0;
      v33 = 0;
      v18 = 0;
      v20 = 0;
      v8 = 0;
      v29 = a1;
      v36 = a0;
      do {
        v35 &= 0xffffff00;
        sub_7470(&v13);
        if ((char)v38) { // branch-flip
          if (!v39)
            return NULL;
          v4 = 10 <= v18 & v19;
          if (v4) {
            if (v18 * 5 <= v8) { // branch-flip
label_8000:
              v17 = v8 - v20;
              if (v17) { // branch-flip
                if (!v28) goto label_804d;
                while ((!v31 || (v32))) {
                  v28 = '\0';
                  v29 = &v29[v30];
                  v17 -= 1;
                  if (!v17) goto label_8080;
label_804d:
                  sub_7470(v15);
                }
                if (v28) goto label_8110;
label_8080:
                sub_7470(v15);
              }
              else if (!v28) goto label_8080;
              v19 = v31;
              if (v31) { // branch-flip
                if (!v32) {
label_8110:
                  v19 = sub_75e0(a0,a1,&v10);
                  if (v19)
                    return v10;
                }
              }
              else {
                v19 = 1;
              }
              v20 = v8;
              if (!(char)v38) goto label_7dd6;
            }
            else {
              v19 = v4;
            }
          }
          v9 = v8 + 1;
          if (!v25) goto label_7dd6;
          if (v39 == v26) goto label_7e16;
label_7d68:
          v6 = v36;
          v16 = v37;
        }
        else {
          v4 = 10 <= v18 & v19;
          if (v4) { // branch-flip
            if (v18 * 5 <= v8) goto label_8000;
            v19 = v4;
          }
          else {
          }
label_7dd6:
          v16 = v37;
          v6 = v36;
          v9 = v8 + 1;
          if (v37 == v24) {
            if (!memcmp(v36,v23,v37)) {
label_7e16:
              v9 = v8 + 1;
              v14 = CONCAT44(v33,v13);
              v12[0] = 0;
              v47 = CONCAT44(v39,v38);
              v52 = 0;
              v46 = v37;
              v45 = &v36[v37];
              v44 = (undefined3)(v35 >> 8);
              v43[4] = 0;
              v43._0_4_ = v34;
              v51 = 0;
              v48 = v40;
              v49 = v41;
              v50 = v42;
              v53 = a1;
              sub_7470(v12);
              if ((v55) && (!v56))
                abort(); // no-return
              v53 = &v53[v54];
              do {
                v9 += 1;
                v52 = 0;
                sub_7470(v12);
                if ((v55) && (!v56))
                  return v36;
                if (!v43[4])
                  sub_7470(&v14);
                v16 = v54;
                v1 = v53;
                v8 = v46;
                v6 = v45;
                if ((char)v47) { // branch-flip
                  if (!v47._4_4_)
                    return NULL;
                  if (!v55) goto label_7f8e;
                  if (v47._4_4_ != v56) goto label_7d68;
                }
                else {
label_7f8e:
                  if ((v46 != v54) || (memcmp(v45,v53,v46))) goto label_7d68;
                }
                v45 = &v6[v8];
                v43[4] = 0;
                v53 = &v1[v16];
              } while( true );
            }
          }
        }
        v18 += 1;
        v36 = &v6[v16];
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
      v16 = 0;
      v9 = 0;
      v8 = 0;
      v5 = 1;
      v6 = a1;
      do {
        v16 += 1;
        v7 = v9 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v17 = v7 - v9;
            if (!a0[v17])
              return NULL;
            v7 += 1;
            if (a0[v17] != v3) break;
            v3 = a1[v7 - v9];
            if (!v3)
              return a0;
          }
        }
        v9 = v7;
        if (!*v1)
          return NULL;
        v4 = 10 <= v16 & (unsigned char)v5;
        if (v4) {
          if (v16 * 5 <= v9) { // branch-flip
            if (v6) {
              v6 = &v6[strnlen(v6,v9 - v8)];
              v8 = v9;
              if (*v6) goto label_7bad;
            }
            v5 = sub_72d0(v1,a1,strlen(a1),&v10);
            if ((char)v5)
              return v10;
            v6 = NULL;
          }
          else {
label_7bad:
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

// Function: sub_8150 @ 0x8150
int sub_8150(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_8790(v2,1);
    if (0 <= v1) { // branch-flip
label_8183:
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
        goto label_8183;
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

// Function: sub_8200 @ 0x8200
unsigned long sub_8200(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
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
label_83a2:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_844e:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_844e;
      goto label_83a2;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_8480 @ 0x8480
char * sub_8480(char *a0,unsigned long a1) // return-dupe
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
    if (*v4 == 0x16) goto label_861d;
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
label_861d:
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

// Function: sub_86d0 @ 0x86d0
char * sub_86d0(char *a0,long a1,long a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_7170(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_8480(a0,v3);
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

// Function: sub_8790 @ 0x8790
int sub_8790(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_1d2b8) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_1d2b8 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_1d2b8 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_1d2b8 != -1)
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

// Function: sub_8a00 @ 0x8a00
long sub_8a00(unsigned long a0)
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

// Function: sub_8a50 @ 0x8a50
void sub_8a50(void *a0)
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1))); // tail-call
}

// Function: sub_8a80 @ 0x8a80
unsigned long sub_8a80(unsigned char *a0)
{
  bool v1;
  char v10; // stack - 0x58
  unsigned char *v2;
  unsigned long v3; // rax
  mbstate_t v4; // stack - 0x74
  int v5 [9]; // stack - 0x54
  unsigned long v6; // r12
  char v7; // stack - 0x6c
  unsigned char *v8; // stack - 0x68
  unsigned long v9; // stack - 0x60
  
  if (__ctype_get_mb_cur_max() <= 1)
    return strlen((char *)a0); // tail-call
  v6 = 0;
  v1 = 0;
  v4 = 0;
  v8 = a0;
  do {
    v7 = 0;
    if (v1) {
label_8b5c:
      v3 = __ctype_get_mb_cur_max();
      v2 = v8;
      v9 = sub_70e0(v5,v2,sub_5660(v8,v3),&v4);
      if (v9 != 0xffffffffffffffff) { // branch-flip
        if (v9 != 0xfffffffffffffffe) {
          if (!v9) {
            v9 = 1;
            if (*v8)
              __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
            if (v5[0])
              __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
          }
          v10 = 1;
          if (mbsinit(&v4))
            v1 = 0;
          goto label_8b38;
        }
        v9 = strlen((char *)v8);
        v10 = 0;
      }
      else {
        v9 = 1;
        v10 = 0;
      }
    }
    else {
      if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0x9e60) >> (*v8 & 0x1f) & 1)) {
        if (!mbsinit(&v4))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_8b5c;
      }
      v9 = 1;
      v5[0] = (int)(char)*v8;
      v10 = 1;
label_8b38:
      if (!v5[0])
        return v6;
    }
    v8 = &v8[v9];
    v6 += 1;
  } while( true );
}

// Function: _DT_FINI @ 0x8ccc
void _DT_FINI(void)
{
  return;
}
