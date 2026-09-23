// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_dfe0)
    return;
  (*dat_dfe0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_ddb8)(); // jump-as-call
}

// Function: free @ 0x2440
void free(void *a0)
{
  (*dat_dfc8)(); // jump-as-call
}

// Function: malloc @ 0x2450
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_dfe8)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x2460
void __cxa_finalize(void)
{
  (*dat_dff8)(); // jump-as-call
}

// Function: getenv @ 0x2470
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ddc0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x2480
void abort(void)
{
  (*dat_ddc8)(); // jump-as-call
}

// Function: __errno_location @ 0x2490
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_ddd0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x24a0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_ddd8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x24b0
void _exit(int a0)
{
  (*dat_dde0)(); // jump-as-call
}

// Function: __fpending @ 0x24c0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_dde8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x24d0
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ddf0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x24e0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_ddf8)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x24f0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_de00)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2500
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_de08)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2510
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_de10)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2520
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_de18)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x2530
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_de20)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2540
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_de28)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2550
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_de30)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2560
void __stack_chk_fail(void)
{
  (*dat_de38)(); // jump-as-call
}

// Function: getopt_long @ 0x2570
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_de40)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2580
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_de48)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2590
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_de50)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x25a0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_de58)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x25b0
void lseek(void)
{
  (*dat_de60)(); // jump-as-call
}

// Function: __assert_fail @ 0x25c0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_de68)(); // jump-as-call
}

// Function: fputs @ 0x25d0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_de70)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x25e0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_de78)(); // jump-as-call
  return v1;
}

// Function: fgetc @ 0x25f0
int fgetc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_de80)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2600
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_de88)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x2610
void posix_fadvise(void)
{
  (*dat_de90)(); // jump-as-call
}

// Function: read @ 0x2620
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_de98)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x2630
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_dea0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2640
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_dea8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x2650
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_deb0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2660
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_deb8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2670
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_dec0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2680
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_dec8)(); // jump-as-call
  return v1;
}

// Function: __memmove_chk @ 0x2690
void * __memmove_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ded0)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x26a0
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ded8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x26b0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_dee0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x26c0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_dee8)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x26d0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_def0)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x26e0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_def8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x26f0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_df00)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x2700
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_df08)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x2710
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_df10)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x2720
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_df18)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2730
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_df20)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2740
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_df28)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x2750
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_df30)(); // jump-as-call
  return v1;
}

// Function: setvbuf @ 0x2760
int setvbuf(FILE *a0,char *a1,int a2,unsigned long a3)
{
  int v1; // eax
  
  v1 = (*dat_df38)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x2770
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_df40)(); // jump-as-call
  return v1;
}

// Function: btowc @ 0x2780
unsigned int btowc(int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_df48)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2790
void error(int a0,int a1,char *a2,...)
{
  (*dat_df50)(); // jump-as-call
}

// Function: open @ 0x27a0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_df58)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x27b0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_df60)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x27c0
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_df68)(); // jump-as-call
  return v1;
}

// Function: sysconf @ 0x27d0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_df70)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x27e0
void __cxa_atexit(void)
{
  (*dat_df78)(); // jump-as-call
}

// Function: sysinfo @ 0x27f0
int sysinfo(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_df80)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x2800
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_df88)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x2810
void exit(int a0)
{
  (*dat_df90)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2820
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_df98)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2830
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_dfa0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2840
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_dfa8)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x2850
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_dfb0)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x2860
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_dfb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2870
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_dfc0)(); // jump-as-call
  return v1;
}

// Function: sub_2880 @ 0x2880
undefined16 sub_2880(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x28a0
int main(int argc,char **argv,char **envp)
{
  unsigned char v1; // al
  char *v10; // rax
  FILE *v11;
  char *v12; // rax
  unsigned long v13; // rax
  unsigned long v14; // rax
  int *v15;
  stat v16; // stack - 0xd8
  int v17; // stack - 0x1fc
  long v18; // stack - 0x1f8
  int *v19;
  unsigned char v2;
  long v20; // fs_offset
  double v21;
  long v22; // stack - 0x228
  bool v23; // stack - 0x209
  char **v24; // stack - 0x1f0
  long v25; // stack - 0x40
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  char *v7;
  struct_4 *v8; // rax
  int *v9; // rax
  
  v25 = *(long *)(v20 + 0x28);
  sub_4ea0(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8cd0(sub_4870);
  dat_e120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  v7 = getenv("POSIXLY_CORRECT");
  dat_e131 = '\0';
  dat_e132 = '\0';
  dat_e118 = v7 != NULL;
  dat_e133 = '\0';
  dat_e134 = '\0';
  dat_e130 = '\0';
  dat_e138 = 0;
  dat_e140 = 0;
  dat_e148 = 0;
  dat_e150 = 0;
  dat_e158 = 0;
  v7 = NULL;
  while( true ) {
    v3 = getopt_long(argc,argv,"clLmw",(option *)0xda20,NULL);
    if (v3 == -1) break;
    if (0x81 < v3) {
      sub_45c0(1); // return-dupe, no-return
    }
    if (v3 <= 0x4b) {
      if (v3 == -0x83) {
        sub_7d20(stdout,"wc","GNU coreutils",dat_e018,"Paul Rubin","David MacKenzie",0,CONCAT44(dat_4,v3));
        exit(0); // no-return
      }
      if (v3 == -0x82)
        sub_45c0(0); // no-return
      sub_45c0(1);
    }
    switch(v3) {
      case 0x4c:
        dat_e130 = '\x01';
        break;
      default:
        goto label_3048;
      case 99:
        dat_e131 = '\x01';
        break;
      case 0x6c:
        dat_e134 = '\x01';
        break;
      case 0x6d:
        dat_e132 = '\x01';
        break;
      case 0x77:
        dat_e133 = '\x01';
        break;
      case 0x80:
        dat_e160 = 1;
        break;
      case 0x81:
        v7 = optarg;
      
    }
  }
  if ((((!dat_e134) && (!dat_e133)) && (!dat_e132)) && ((!dat_e131 && (!dat_e130)))) {
    dat_e131 = '\x01';
    dat_e133 = '\x01';
    dat_e134 = '\x01';
  }
  if (v7) { // branch-flip
    if (optind < argc) {
      v7 = (char *)sub_7110(4,argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v7);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
label_3048:
      sub_45c0(1);
    }
    if (strcmp(v7,"-")) { // branch-flip
      v11 = fopen(v7,"r");
      if (!v11) {
        v14 = sub_7110(4,v7);
        v7 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v7,v14);
        return v4;
      }
    }
    else {
      v11 = stdin;
    }
    v3 = fileno(v11);
    if ((!fstat(v3,&v16)) && ((v16._24_4_ & 0xf000) == 0x8000)) {
      v21 = dat_99c0;
      if ((double)sub_4d60() * dat_99c8 <= dat_99c0)
        v21 = dat_99c8 * (double)sub_4d60();
      if ((double)(long)v16._48_8_ <= v21) {
        sub_74d0(&v18);
        v23 = sub_7580(v11,&v18);
        if ((!v23) || (sub_4960(v11))) {
          v7 = (char *)sub_7110(4,v7);
          error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v7);
          return v6;
        }
        v22 = v18;
        v8 = (struct_4 *)sub_4700(v24);
        argv = v24;
        goto label_2b30;
      }
    }
    v8 = (struct_4 *)sub_4730(v11);
    v23 = 0;
    v22 = 0;
    argv = NULL;
  }
  else {
    if (argc <= optind) { // branch-flip
      v22 = 1;
      argv = (char **)0xe110;
    }
    else {
      v22 = (long)(argc - optind);
      argv = &argv[optind];
    }
    v8 = (struct_4 *)sub_4700(argv);
    v23 = 0;
  }
label_2b30:
  if (v8) {
    v9 = (int *)sub_32f0(v22,argv);
    dat_e12c = sub_3230(v22,v9);
    v2 = 1;
    v15 = v9;
    while (v10 = (char *)sub_4770(v8,&v17), v10) {
      if (v7) { // branch-flip
        if ((strcmp(v7,"-")) || (strcmp(v10,"-"))) {
          if (*v10) goto label_2ba5;
        }
        else {
          v12 = (char *)sub_7110(4,v10);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v12);
          if (*v10) goto label_2c21;
        }
        v13 = sub_4810(v8);
        v10 = dcgettext(NULL,"invalid zero-length file name",5);
        error(0,0,"%s:%lu: %s",(char *)sub_7210(0,3,v7),v13,v10);
        v2 = 0;
      }
      else if (*v10) { // branch-flip
label_2ba5:
        v19 = v9;
        if (v22)
          v19 = v15;
        v2 &= sub_4500(v10,v19);
      }
      else {
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
label_2c21:
        v2 = 0;
      }
      if (!v22)
        *v9 = 1;
      v15 = &v15[0x26];
    }
    if (v17 != 3) {
      if (v17 != 4) { // branch-flip
        if (v17 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
        v1 = v7 == NULL & v2;
        if ((v1) && (v2 = v1, !sub_4810(v8)))
          v2 = sub_4500(NULL,v9);
      }
      else {
        v14 = sub_7210(0,3,v7);
        v7 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v7,v14);
        v2 = 0;
      }
      if (v23)
        sub_7550(&v18);
      if (2 <= (unsigned long)sub_4810(v8))
        sub_35d0(dat_e158,dat_e150,dat_e148,dat_e140,dat_e138,dcgettext(NULL,"total",5));
      sub_4830(v8);
      free(v9);
      if ((dat_e128) && (close(0))) {
        error(1,*__errno_location(),"-");
        return v5;
      }
      if (v25 == *(long *)(v20 + 0x28))
        return (int)(v2 ^ 1);
      __stack_chk_fail(); // no-return
    }
  }
  sub_8250(); // no-return
}

// Function: sub_30f0 @ 0x30f0
void sub_30f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_dfd0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_3120 @ 0x3120
void sub_3120(void)
{
  return;
}

// Function: _FINI_0 @ 0x3190
void _FINI_0(void)
{
  if (!dat_e108) {
    if (dat_dff8)
      __cxa_finalize(dat_e008);
    sub_3120();
    dat_e108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x31d0
void _INIT_0(void)
{
  return;
}

// Function: sub_31e0 @ 0x31e0
unsigned int sub_31e0(unsigned int a0)
{
  return a0;
}

// Function: sub_31f0 @ 0x31f0
bool sub_31f0(int a0) // return-dupe
{
  if (dat_e118)
    return 0;
  if ((a0 != 0xa0) && (a0 != 0x2007))
    return a0 == 0x202f || a0 == 0x2060;
  return 1;
}

// Function: sub_3230 @ 0x3230
int sub_3230(long a0,int *a1)
{
  bool v1;
  int v2; // ecx
  unsigned long v3;
  unsigned long v4; // rdx
  int *v5; // rdi
  int v6;
  
  if ((a0) && (*a1 <= 0)) {
    v3 = 0;
    v6 = 1;
    v5 = &a1[a0 * 0x26];
    do {
      if (!*a1) {
        if ((a1[8] & 0xf000U) != 0x8000) // branch-flip
          v6 = 7;
        else {
          v3 += *(long *)&a1[0xe];
        }
      }
      a1 = &a1[0x26];
    } while (a1 != v5);
    v2 = 1;
    if (10 <= v3) {
      do {
        v2 += 1;
        v4 = v3 / 10;
        v1 = 100 <= v3;
        v3 = v4;
      } while (v1);
    }
    if (v6 <= v2)
      v6 = v2;
    return v6;
  }
  return 1;
}

// Function: sub_32c0 @ 0x32c0
void sub_32c0(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1); // tail-call
}

// Function: sub_32f0 @ 0x32f0
unsigned int * sub_32f0(unsigned long a0,long a1)
{
  char *v1;
  int v2; // eax
  unsigned int *v3; // rax
  unsigned long v4;
  unsigned int *v5;
  
  v4 = 1;
  if (a0)
    v4 = a0;
  v3 = (unsigned int *)sub_7fb0(v4,0x98);
  if ((a0) && ((a0 != 1 || ((unsigned int)dat_e134 + (unsigned int)dat_e133 + (unsigned int)dat_e132 + (unsigned int)dat_e131 + (unsigned int)dat_e130 != 1)))) {
    v4 = 0;
    v5 = &v3[2];
    do {
      v1 = *(char **)(a1 + v4 * 8);
      if (v1) { // branch-flip
        if (!strcmp(v1,"-")) goto label_3348;
        v2 = stat(v1,(stat *)v5);
      }
      else {
label_3348:
        v2 = fstat(0,(stat *)v5);
      }
      v4 += 1;
      v5[-2] = v2;
      v5 = &v5[0x26];
    } while (a0 != v4);
  }
  else {
    *v3 = 1;
  }
  return v3;
}

// Function: sub_33e0 @ 0x33e0
char sub_33e0(char *a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  bool v1;
  unsigned long v2; // rax
  char *v3; // rax
  char *v4;
  char *v5;
  unsigned long v6;
  char v7 [16392];
  unsigned long v8;
  long v9; // stack - 0x4070
  
  if (!a2 || !a3)
    return 0;
  v1 = 0;
  v9 = 0;
  v8 = 0;
  while( true ) {
    v2 = sub_7770(a1,v7,0x4000);
    if (!v2) {
      *a3 = v9;
      *a2 = v8;
      return 1;
    }
    if (v2 == 0xffffffffffffffff) break;
    v9 += v2;
    v5 = &v7[v2];
    if (v1) {
      *v5 = '\n';
      v6 = v8;
      v4 = v7;
      while (v4 = rawmemchr(v4,10), v4 < v5) {
        v4 = &v4[1];
        v6 += 1;
      }
      v1 = (unsigned long)(v6 - v8) <= v2 / 0xf;
      v8 = v6;
    }
    else if (v5 != v7) { // branch-flip
      v4 = v7;
      v6 = v8;
      do {
        v3 = &v4[1];
        v6 += (unsigned long)(*v4 == '\n');
        v4 = v3;
      } while (v5 != v3);
      v1 = (unsigned long)(v6 - v8) <= v2 / 0xf;
      v8 = v6;
    }
    else {
      v1 = 1;
    }
  }
  v5 = (char *)sub_7210(0,3,a0);
  error(0,*__errno_location(),"%s",v5);
  return 0;
}

// Function: sub_35b0 @ 0x35b0
void sub_35b0(int a0)
{
  sub_31f0(btowc(a0)); // tail-call
}

// Function: sub_35d0 @ 0x35d0
void sub_35d0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  unsigned long v1; // rax
  char v2 [24];
  char *v3;
  char *v4;
  
  v3 = "%*s";
  if (dat_e134) {
    v1 = sub_4ad0(a0,v2);
    v3 = " %*s";
    __printf_chk(1,"%*s",(unsigned long)dat_e12c,v1);
  }
  v4 = v3;
  if (dat_e133) {
    v1 = sub_4ad0(a1,v2);
    v4 = " %*s";
    __printf_chk(1,v3,(unsigned long)dat_e12c,v1);
  }
  v3 = v4;
  if (dat_e132) {
    v1 = sub_4ad0(a2,v2);
    v3 = " %*s";
    __printf_chk(1,v4,(unsigned long)dat_e12c,v1);
  }
  v4 = v3;
  if (dat_e131) {
    v1 = sub_4ad0(a3,v2);
    v4 = " %*s";
    __printf_chk(1,v3,(unsigned long)dat_e12c,v1);
  }
  if (dat_e130) {
    v1 = sub_4ad0(a4,v2);
    __printf_chk(1,v4,(unsigned long)dat_e12c,v1);
  }
  if (a5) {
    if (strchr(a5,10))
      a5 = (char *)sub_7210(0,3,a5);
    __printf_chk(1," %s",a5);
  }
  putchar_unlocked(10);
}

// Function: sub_37a0 @ 0x37a0
unsigned int sub_37a0(void)
{
  return *(unsigned int *)cpuid_basic_info(0);
}

// Function: sub_37b0 @ 0x37b0
bool sub_37b0(unsigned int *a0,unsigned int *a1,unsigned int *a2,unsigned int *a3)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4;
  bool v5; // zf
  
  v5 = sub_37a0() != 0;
  if (v5) {
    v1 = (unsigned int *)cpuid_Version_info(1);
    v2 = v1[1];
    v3 = v1[2];
    v4 = v1[3];
    *a0 = *v1;
    *a1 = v2;
    *a2 = v4;
    *a3 = v3;
  }
  return v5;
}

// Function: sub_37f0 @ 0x37f0
unsigned long sub_37f0(unsigned int *a0,unsigned int *a1,unsigned int *a2,unsigned int *a3)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned long v5; // r8
  
  v5 = 0;
  if (7 <= (unsigned int)sub_37a0()) {
    v1 = (unsigned int *)cpuid_Extended_Feature_Enumeration_info(7);
    v2 = v1[1];
    v3 = v1[2];
    v4 = v1[3];
    *a0 = *v1;
    v5 = 1;
    *a1 = v2;
    *a2 = v4;
    *a3 = v3;
  }
  return v5 & 0xffffffff;
}

// Function: sub_3830 @ 0x3830
char sub_3830(void)
{
  char v1;
  int v2; // eax
  unsigned int v3 [2]; // stack - 0x38
  unsigned int v4; // stack - 0x3c
  unsigned int v5; // stack - 0x40
  char *v6; // rsi
  unsigned int *v7;
  
  v7 = &v5;
  v5 = 0;
  v4 = 0;
  v3[0] = 0;
  v3[1] = 0;
  v2 = sub_37b0(v7,&v4,v3,&v3[1]);
  v1 = dat_e160;
  if (v2) { // branch-flip
    if (v3[0] & 0x8000000) {
      v3[1] = 0;
      v3[0] = 0;
      v4 = 0;
      v5 = 0;
      if (!sub_37f0(v7,&v4,v3,&v3[1])) goto label_38d8;
      if (v4 & 0x20) {
        if (v1) {
          error(0,0,"%s",dcgettext(NULL,"using avx2 hardware support",5));
          return v1;
        }
        return '\x01';
      }
    }
    if (!v1)
      return '\0';
    v6 = "avx2 support not detected";
  }
  else {
label_38d8:
    if (!v1)
      return '\0';
    v6 = "failed to get cpuid";
  }
  error(0,0,"%s",dcgettext(NULL,v6,5));
  return '\0';
}

// Function: sub_39b0 @ 0x39b0
void sub_39b0(void)
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
    if (!strcmp("wc",v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!v6)
    v6 = "wc";
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "wc";
  if (!strcmp("wc","["))
    v3 = "test";
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == "wc")
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_3be0 @ 0x3be0
bool sub_3be0(unsigned int a0)
{
  return (a0 & 0xd000) == 0x8000;
}

// Function: sub_3bf0 @ 0x3bf0
int sub_3bf0(int a0,char *a1,struct_2 *a2,long a3) // ternary
{
  char v1;
  char v10 [16392];
  mbstate_t v11; // stack - 0x4050
  unsigned int v12; // stack - 0x4064
  unsigned long v13; // stack - 0x4058
  long v14; // stack - 0x4060
  long *v15;
  unsigned long v16;
  unsigned int v17;
  unsigned int v18;
  char *v19;
  unsigned short v2;
  char *v20;
  unsigned char v21;
  unsigned long v22;
  long v23;
  bool v24;
  unsigned long v25; // stack - 0x40a8
  long v26; // stack - 0x4098
  unsigned long v27; // stack - 0x4090
  unsigned char v28; // stack - 0x4079
  char *v29; // stack - 0x4070
  unsigned long v3;
  mbstate_t v4;
  unsigned char v5; // al
  int v6; // eax
  void *v7;
  unsigned long v8;
  unsigned long v9;
  
  v29 = a1;
  if (!a1) {
    v7 = (char *)dcgettext(NULL,"standard input",5);
    v29 = (char *)v7;
  }
  v13 = 0;
  v14 = 0;
  v7 = (unsigned long)__ctype_get_mb_cur_max();
  v21 = dat_e131; // branch-flip
  if (2 <= v7) {
    v5 = dat_e132;
    v28 = v5;
  }
  else {
    v28 = 0;
    if (!dat_e131)
      v21 = dat_e132;
  }
  if ((dat_e133) || (dat_e130))
    sub_4920(a0,0,0,2);
  else {
    if ((v21 == 1) && (!v28)) {
      if (dat_e134) {
        sub_4920(a0,0,0,2);
label_446e:
        if (dat_e134) {
label_4480:
          v5 = sub_3830();
          if (v5) { // branch-flip
            v7 = sub_8a90;
            dat_e010 = sub_8a90;
          }
          else {
            v7 = dat_e010;
          }
          v25 = 0;
          v22 = 0;
          v9 = 0;
          v6 = (*v7)(v29,a0,&v14,&v13);
          v8 = v13;
          v18 = v6;
          goto label_3db8;
        }
      }
      v6 = a2->field_0x0;
      if (0 < v6) {
        v6 = fstat(a0,(stat *)&a2->field_0x4[4]);
        a2->field_0x0 = v6;
      }
      if (!v6) {
        v6 = sub_3be0(a2->field_0x20);
        v18 = v6;
        v22 = dat_e120;
        v21 = (unsigned char)v6;
        if ((v21) && (v8 = a2->field_0x38, 0 <= (long)v8)) {
          if (a3 != -1) { // branch-flip
            v22 = 0;
            if (v8 % dat_e120) {
label_442f:
              v25 = 0;
              v22 = 0;
              v9 = 0;
              v13 = v8;
              goto label_3db8;
            }
          }
          else {
            v7 = (unsigned long)lseek(a0,0,1);
            if (v8 % v22) {
              v8 = (v7 <= v8) ? v8 - (long)v7 : 0; // branch-flip
              goto label_442f;
            }
            v21 = (unsigned char)~(unsigned char)((unsigned long)v7 >> 0x38) >> 7;
            v22 = (unsigned long)v7;
          }
          v7 = (unsigned long)(a2->field_0x40 + 1);
          v9 = 0x201;
          if ((unsigned long)(a2->field_0x40 - 1U) <= 0x1fffffffffffffff)
            v9 = (unsigned long)v7;
          v23 = v8 - v8 % v9;
          if ((((long)v22 < v23) && (v21)) && (v7 = (long)lseek(a0,v23,1), 0 <= (long)v7))
            v13 = v23 - v22;
        }
      }
      sub_4920(a0,0,0,2);
      while (v7 = (long)sub_7770(a0,v10,0x4000), v7) {
        if (v7 == (void *)0xffffffffffffffff) {
          v25 = 0;
          v9 = 0;
          v7 = (unsigned long)sub_7210(0,3,v29);
          v3 = v7;
          v7 = (int *)__errno_location();
          v22 = 0;
          error(0,*(int *)v7,"%s",v3);
          v8 = v13;
          v18 = 0;
          goto label_3db8;
        }
        v13 += (long)v7;
      }
      v25 = 0;
      v9 = 0;
      v22 = 0;
      v8 = v13;
      v18 = 1;
      goto label_3db8;
    }
    sub_4920(a0,0,0,2);
    if (v28 != 1) {
      if (v21) goto label_446e;
      if (!v28) goto label_4480;
    }
  }
  v7 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v7) { // branch-flip
    v8 = 0;
    v24 = 0;
    v9 = 0;
    v11 = 0;
    v22 = 0;
    v25 = 0;
    v27 = 0;
    v26 = 0;
label_3ea8:
    v7 = (long)sub_7770(a0,&v10[v8],0x4000 - v8);
    if (v7) { // branch-flip
      if (v7 != (void *)0xffffffffffffffff) {
        v13 += (long)v7;
        v8 += (long)v7;
        v19 = v10;
        if (v24) goto label_3f9c;
label_3ef0:
        v1 = *v19;
        v17 = (unsigned int)v1;
        v5 = sub_4bc0(v17);
        v21 = v5;
        if (!v5) goto label_3f9c;
        v12 = v17;
        switch(v1) {
          case 9:
            v23 = 1;
label_40b5:
            v7 = (unsigned long)(v25 & 0xfffffffffffffff8);
            v7 = (unsigned long)((long)v7 + 8);
            v25 = (unsigned long)v7;
label_4072:
            v26 += v9;
            v9 = 0;
label_3f80:
            do {
              v19 = &v19[v23];
              v8 -= v23;
              v22 += 1;
              while( true ) {
                if (!v8) goto label_3ea8;
                if (!v24) goto label_3ef0;
label_3f9c:
                v4 = v11;
                v7 = (long)sub_4be0(&v12,v19,v8,&v11);
                v23 = (long)v7;
                if (v7 == (void *)0xfffffffffffffffe) {
                  v11 = v4;
                  if (v8) {
                    if (v8 == 0x4000) {
                      v19 = &v19[1];
                      v8 = 0x3fff;
                    }
                    __memmove_chk(v10,v19,v8,0x4001);
                  }
                  v24 = 1;
                  goto label_3ea8;
                }
                if (v7 != (void *)0xffffffffffffffff) break;
                v19 = &v19[1];
                v8 -= 1;
                v24 = 1;
              }
              v6 = mbsinit(&v11);
              v24 = v6 == 0;
              if (v23) { // branch-flip
                switch(v12) {
                  case 9:
                    goto label_40b5;
                  case 10:
                    goto label_40a5;
                  case 0xb:
                    goto label_4072;
                  case 0xc:
                  case 0xd:
                    goto label_4055;
                  default:
                    v17 = v12;
                    break;
                  case 0x20:
                    goto label_4095;
                  
                }
              }
              else {
                v12 = 0;
                v23 = 1;
                v17 = 0;
              }
              v6 = iswprint(v17);
              if (v6) {
                if (dat_e130) {
                  v6 = wcwidth(v12);
                  v16 = (long)v6 + v25;
                  if (v6 <= 0)
                    v16 = v25;
                  v25 = v16;
                }
                v6 = iswspace(v12);
                if ((v6) || (v6 = sub_31f0(v12), v6)) goto label_4072;
                v9 = 1;
              }
            } while( true );
          case 10:
            v23 = 1;
            goto label_40a5;
          case 0xb:
label_4080:
            v23 = 1;
            goto label_4072;
          case 0xc:
          case 0xd:
            v23 = 1;
            goto label_4055;
          case 0x20:
            v23 = 1;
            goto label_4095;
          
        }
        v7 = __ctype_b_loc();
        v15 = (long *)v7;
        v5 = sub_31e0(v17);
        v7 = (unsigned long)(unsigned long)v5;
        v2 = *(unsigned short *)(*v15 + (long)v7 * 2);
        if (v2 & 0x4000) {
          v25 += 1;
          if (v2 & 0x2000) goto label_4080;
          v9 = (unsigned long)v21;
        }
        v23 = 1;
        goto label_3f80;
      }
      v7 = (unsigned long)sub_7210(0,3,v29);
      v3 = v7;
      v7 = (int *)__errno_location();
      v18 = 0;
      error(0,*(int *)v7,"%s",v3);
    }
    else {
      v18 = 1;
    }
    if (v25 <= v27)
      v25 = v27;
    v9 += v26;
    v8 = v13;
  }
  else {
    v22 = 0;
    v25 = 0;
    v23 = 0;
    v9 = v25;
    while (v7 = (long)sub_7770(a0,v10,0x4000), v7) {
      if (v7 == (void *)0xffffffffffffffff) {
        v7 = (unsigned long)sub_7210(0,3,v29);
        v3 = v7;
        v7 = (int *)__errno_location();
        v18 = 0;
        error(0,*(int *)v7,"%s",v3);
        goto label_4347;
      }
      v13 += (long)v7;
      v19 = &v10[(long)v7];
      v20 = v10;
      do {
        v1 = *v20;
        v20 = &v20[1];
        switch(v1) {
          case 9:
            v22 = (v22 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v14 += 1;
            goto label_4266;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_4266:
            if (v25 < v22)
              v25 = v22;
            v22 = 0;
            break;
          default:
            v7 = __ctype_b_loc();
            v15 = (long *)v7;
            v5 = sub_31e0((int)v1);
            v2 = *(unsigned short *)(*v15 + (unsigned long)v5 * 2);
            if (v2 & 0x4000) {
              v22 += 1;
              if ((v2 & 0x2000) || (v6 = sub_35b0(v5), v6)) goto label_426f;
              v23 = 1;
            }
            goto label_4250;
          case 0x20:
            v22 += 1;
          
        }
label_426f:
        v9 += v23;
        v23 = 0;
label_4250:
      } while (v19 != v20);
    }
    v18 = 1;
label_4347:
    if (v25 < v22)
      v25 = v22;
    v9 = v23 + v9;
    v22 = 0;
    v8 = v13;
  }
label_3db8:
  if (v28 < dat_e132)
    v22 = v8;
  sub_35d0(v14,v9,v22,v8,v25,a1);
  dat_e150 += v9;
  dat_e158 += v14;
  dat_e148 += v22;
  dat_e140 += v13;
  if (dat_e138 < v25)
    dat_e138 = v25;
  v6 = v18;
  return v6;
label_4095:
  v25 += 1;
  goto label_4072;
label_40a5:
  v14 += 1;
  goto label_4055;
label_4055:
  v7 = (void *)v25;
  v25 = 0;
  if (v7 <= v27)
    v7 = (void *)v27;
  v27 = (unsigned long)v7;
  goto label_4072;
}

// Function: sub_4500 @ 0x4500
unsigned long sub_4500(char *a0,struct_2 *a1)
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  
  if (a0) {
    if (strcmp(a0,"-")) {
      v1 = open(a0,0);
      if (v1 != -1) {
        v2 = sub_3bf0(v1,a0,a1,0);
        if (!close(v1))
          return (unsigned long)v2;
      }
      v4 = (char *)sub_7210(0,3,a0);
      error(0,*__errno_location(),"%s",v4);
      return 0;
    }
  }
  dat_e128 = 1;
  v3 = sub_3bf0(0,a0,a1,-1); // tail-call
  return v3;
}

// Function: sub_45c0 @ 0x45c0
void sub_45c0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_e178;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n",5),v1);
    sub_32c0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_39b0();
  }
  exit(a0); // no-return
}

// Function: sub_4700 @ 0x4700
void sub_4700(char **a0) // return-dupe
{
  struct_12 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
}

// Function: sub_4730 @ 0x4730
void sub_4730(unsigned long a0) // return-dupe
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

// Function: sub_4770 @ 0x4770
long sub_4770(struct_4 *a0,int *a1) // early-return x2
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

// Function: sub_4810 @ 0x4810
long sub_4810(struct_5 *a0)
{
  if (a0->field_0x0)
    return a0->field_0x8;
  return a0->field_0x28 - a0->field_0x20 >> 3;
}

// Function: sub_4830 @ 0x4830
void sub_4830(struct_6 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_4850 @ 0x4850
void sub_4850(unsigned long a0)
{
  dat_e170 = a0;
}

// Function: sub_4860 @ 0x4860
void sub_4860(char a0)
{
  dat_e168 = a0;
}

// Function: sub_4870 @ 0x4870
void sub_4870(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_84c0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_e168) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_e170) { // branch-flip
        v4 = (char *)sub_71f0(dat_e170);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_e020); // return-dupe, no-return
    }
  }
  v1 = sub_84c0(stderr);
  if (!v1)
    return;
  _exit(dat_e020);
}

// Function: sub_4920 @ 0x4920
void sub_4920(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_4930 @ 0x4930
void sub_4930(FILE *a0,unsigned int a1)
{
  if (a0) {
    sub_4920(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_4960 @ 0x4960
unsigned long sub_4960(FILE *a0) // early-return
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
    if (sub_4a10(a0)) {
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

// Function: sub_49f0 @ 0x49f0
void sub_49f0(FILE *a0)
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_4a50(a0,0,1); // tail-call
}

// Function: sub_4a10 @ 0x4a10
void sub_4a10(FILE *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_49f0(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_4a50 @ 0x4a50
int sub_4a50(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_4ad0 @ 0x4ad0
char * sub_4ad0(unsigned long a0,long a1)
{
  bool v1;
  char *v2;
  
  *(char *)(a1 + 0x14) = 0;
  v2 = (char *)(a1 + 0x14);
  do {
    v2 = &v2[-1];
    *v2 = (char)a0 + (char)(a0 / 10) * '\xf6' + '0';
    v1 = 10 <= a0;
    a0 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_4b30 @ 0x4b30
int sub_4b30(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_4b60 @ 0x4b60
void sub_4b60(struct_10 *a0,struct_11 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_11 *v3;
  
  v3 = (struct_11 *)a1->field_0x0;
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

// Function: sub_4bc0 @ 0x4bc0
unsigned int sub_4bc0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x9a00) >> (a0 & 0x1f) & 1;
}

// Function: sub_4be0 @ 0x4be0
unsigned long sub_4be0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_8530(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_4c70 @ 0x4c70
double sub_4c70(void)
{
  long v1; // rax
  char v2 [32];
  double v3;
  unsigned long v4; // stack - 0x68
  unsigned int v5; // stack - 0x20
  
  v3 = (double)sysconf(0x55);
  v1 = sysconf(0x1e);
  if ((v3 < 0.0) || ((double)v1 < 0.0)) {
    v3 = dat_9a20;
    if (!sysinfo(v2))
      v3 = (double)v4 * (double)v5;
  }
  else {
    v3 = v3 * (double)v1;
  }
  return v3;
}

// Function: sub_4d60 @ 0x4d60
double sub_4d60(void)
{
  long v1; // rax
  char v2 [40];
  double v3; // xmm0_qa
  double v4; // xmm0_qa
  unsigned long v5; // stack - 0x60
  unsigned long v6; // stack - 0x50
  unsigned int v7; // stack - 0x20
  
  v3 = (double)sysconf(0x56);
  v1 = sysconf(0x1e);
  if ((0.0 <= v3) && (0.0 <= (double)v1))
    return v3 * (double)v1;
  if (!sysinfo(v2))
    return ((double)v5 + (double)v6) * (double)v7;
  sub_4c70();
  return v4 * dat_9a28;
}

// Function: sub_4ea0 @ 0x4ea0
void sub_4ea0(char *a0)
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
    dat_e178 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_4f40 @ 0x4f40
struct_0 * sub_4f40(struct_0 *a0,int a1)
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

// Function: sub_4f90 @ 0x4f90
bool sub_4f90(long a0,long a1)
{
  return sub_8460((char *)(a0 + 9),(char *)(a1 + 9)) == 0;
}

// Function: sub_4fb0 @ 0x4fb0
unsigned long sub_4fb0(long a0,long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_83e0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_4f90(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_5010 @ 0x5010
unsigned long sub_5010(long a0,long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_83e0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_4fb0(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_5090 @ 0x5090
unsigned long sub_5090(long a0,long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_83e0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_5010(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_5100 @ 0x5100
unsigned long sub_5100(long a0,long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_83e0((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_5090(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_51a0 @ 0x51a0
unsigned long sub_51a0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_83e0((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_5100(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_5240 @ 0x5240
unsigned long sub_5240(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_83e0((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_51a0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_5300 @ 0x5300
unsigned long sub_5300(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_83e0((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_5240(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_53c0 @ 0x53c0
unsigned long sub_53c0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_83e0((int)(char)a2)) // branch-flip
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
      return sub_5300(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_54a0 @ 0x54a0
unsigned long sub_54a0(unsigned char *a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_83e0((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_53c0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_5560 @ 0x5560
char * sub_5560(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned char *v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = (unsigned char *)sub_8630();
    if (sub_54a0(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_54a0(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x9a78 : (char *)0x9a7f; // branch-flip
    }
  }
  return v1;
}

// Function: sub_5690 @ 0x5690
long * sub_5690(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_5700:
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
label_5b86:
      v2 = 0;
      goto label_5b40;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_5ca1;
      }
      v23 = 1;
      v2 = 0;
      v12 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v2 = 1;
label_5b40:
      v23 = 1;
      v12 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_5ca1;
      }
      goto label_5b86;
    case 5:
      if (!v17) goto label_5ae1;
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
        v9 = (char *)sub_5560("`",a4);
        v35 = (char *)v9;
        v9 = (char *)sub_5560("\'",a4);
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
label_57c3:
  do {
label_57d6:
    v22 = 0;
    a1 = v13;
label_57e0:
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
          if ((a3 < v22 + v29) || (v8 = memcmp(v11,v30,v29), v8)) goto label_6150;
          if (!v23) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v20 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_5d4c;
                default:
                  goto label_5a23;
                case 7:
                  goto label_5d10;
                case 8:
                  goto label_5cf5;
                case 9:
                  goto label_5dd3;
                case 10:
                  goto label_5ceb;
                case 0xb:
                  goto label_5e00;
                case 0xc:
                  goto label_5d37;
                case 0xd:
                  goto label_5c3d;
                case 0x20:
                  goto label_5e0d;
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
                  goto label_59d6;
                case 0x23:
                  goto label_5db0;
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
                  goto label_5908;
                case 0x27:
                  goto label_5c22;
                case 0x3f:
                  goto label_5be1;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_5a23;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_5908;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_61f4;
              goto label_6138;
            }
            if (v7 == 0x7d) goto label_62c0;
            if ((char)v7 <= '}') {
              v20 = 0x7b;
              if (v7 == 0x7b) goto label_611a;
              v20 = 0x7c;
              if (v7 == 0x7c) goto label_6138;
              goto label_5a23;
            }
            if (v7 != 0x7e) goto label_5a23;
label_6274:
            if (!v22) {
              v7 = a4 == 2;
              v20 = 0x7e;
              v25 = v3;
              goto label_5a99;
            }
            v20 = 0x7e;
            goto label_6282;
          }
          goto label_5a0c;
        }
label_6150:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v20 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v21 = 0;
                if (v7 == 0x7e) goto label_6274;
                goto label_5a20;
              }
              v20 = 0x7b;
              v24 = 0;
              if (v7 != 0x7b) {
                v20 = 0x7c;
                v24 = 0;
                v21 = 0;
                if (v7 == 0x7c) goto label_6138;
                goto label_5a23;
              }
            }
            else {
              v20 = 0x7d;
              v24 = 0;
            }
label_611a:
            if (a3 == 0xffffffffffffffff) goto label_62cf;
label_6128:
            v21 = v24;
            if (a3 == 1) goto label_5db5;
          }
          else {
            if (v7 == 0x40) goto label_5a20;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v21 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_5908;
            v24 = 0;
            v21 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_61f4:
              if ((char)v18 != '\\') goto label_5a23;
              if (((bool)(v2 & v23)) && (v29)) goto label_621a;
              v16 = 0x5c;
              goto label_5c42;
            }
          }
label_6138:
          v7 = a4 == 2;
          v25 = v3;
          v24 = 0;
          goto label_5a99;
        }
        switch(v7) {
          case 0:
            goto label_5d3e;
          default:
label_5a20:
            v24 = 0;
label_5a23:
            v16 = (unsigned char)v18;
            v21 = v24;
            if (v32 != 1) goto label_5fa3;
label_5a30:
            v9 = __ctype_b_loc();
            v20 = 1;
            v25 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v26 = (bool)(v25 & v2);
            v25 = !v25;
            v21 = v24;
            goto label_5a7e;
          case 7:
label_5d10:
            v16 = 0x61;
            if (!v23) goto label_5d08;
            goto label_5c6f;
          case 8:
label_5cf5:
            v16 = 0x62;
            goto label_5cfa;
          case 9:
            v24 = 0;
label_5dd3:
            v20 = 9;
            v16 = 0x74;
            goto label_5de0;
          case 10:
label_5ceb:
            v16 = 0x6e;
            break;
          case 0xb:
label_5e00:
            v16 = 0x76;
            break;
          case 0xc:
label_5d37:
            v16 = 0x66;
label_5cfa:
            if (v23) goto label_5c6f;
label_5d08:
            v24 = 0;
            goto label_598e;
          case 0xd:
label_5c3d:
            v16 = 0x72;
            break;
          case 0x20:
            v21 = 0;
label_5e0d:
            v20 = 0x20;
            goto label_5908;
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
            goto label_59d9;
          case 0x23:
            v20 = 0x23;
            v24 = 0;
            goto label_5db5;
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
label_5908:
            v7 = 0;
            goto label_5918;
          case 0x27:
            v21 = 0;
            goto label_5c22;
          case 0x3f:
            v21 = 0;
            goto label_5be1;
          
        }
        goto label_5c42;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_62c0:
            v20 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_6128;
label_62cf:
            v21 = v24;
            if (a2[1]) goto label_6138;
            goto label_5db5;
          }
          if ((char)v16 <= '}') {
            v20 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_59d6;
              goto label_5a23;
            }
            goto label_611a;
          }
          v20 = 0x7e;
          if (v16 == 0x7e) goto label_5db5;
          if (v32 == 1) goto label_5a30;
label_5fa3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v20 = 0;
          do {
            v18 = v22 + v20;
            v9 = (long)sub_4be0(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v24 = v25;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_650b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_6885;
              goto label_688f;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_60dd;
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
          goto label_650b;
        }
        if (v16 == 0x40) goto label_5a23;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_5a90;
        if ((unsigned long)v9 & 0xa4000000) goto label_59d6;
        if (v16 != 0x5c) goto label_5a23;
        if (a4 == 2) {
          if (!v23) goto label_621a;
          goto label_5a03;
        }
        if (!(bool)(v2 & v23 & v29 != 0)) {
          v20 = 0x5c;
          v16 = 0x5c;
          v24 = 0;
          goto label_5de0;
        }
label_621a:
        v22 += 1;
        v24 = 0;
        v16 = 0x5c;
        v7 = v19;
        goto label_6230;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v21 = 0;
            v20 = 0;
            if (v33 & 1) goto label_5eb2;
            goto label_5deb;
          }
label_5d3e:
          if (!v23) {
            v21 = 0;
label_5d4c:
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
                goto label_5ac3;
              }
label_6655:
              a0[v13] = 0x5c;
              v12 = v13;
              v19 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v19;
              goto label_6655;
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
label_668c:
              v16 = (unsigned char)v20;
              v24 = 0;
              v12 = v13;
              v7 = v25;
              if (!v2 || v26) goto label_5ac3;
              goto label_5918;
            }
            v16 = 0x30;
            v24 = 0;
            v12 = v13;
            v7 = v25;
            goto label_5ac3;
          }
          if (a4 != 2) goto label_5a0c;
          goto label_5c7b;
        default:
          goto label_5a23;
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
          goto label_5e6f;
        case 10:
          v20 = 10;
          v16 = 0x6e;
          goto label_5e6f;
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
label_5e6f:
          v7 = a4 == 2 & v23;
          v24 = 0;
          if (!(bool)v7) goto label_5de0;
label_59f1:
          a4 = 2;
label_5a03:
          if (!v27) goto label_5a0c;
          goto label_5c7b;
        case 0x20:
          v18 = 0x20;
          goto label_59d9;
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
label_59d6:
          v25 = 0;
          goto label_59d9;
        case 0x23:
label_5db0:
          v20 = 0x23;
label_5db5:
          v21 = v24;
          if (v22) {
label_6282:
            v7 = a4 == 2;
            v25 = v3;
            v24 = 0;
            goto label_5a99;
          }
          v18 = v20;
label_59d9:
          v20 = v18;
          v21 = v24;
          v24 = v25;
          if ((a4 == 2) && (v23)) goto label_59f1;
          goto label_5a97;
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
          goto label_5a90;
        case 0x27:
label_5c22:
          v24 = v25;
          if (a4 != 2) {
            v7 = 0;
            v20 = 0x27;
            goto label_5a99;
          }
          if (!v23) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v31) goto label_66ab;
            }
            else {
label_66ab:
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
            goto label_5ac3;
          }
          goto label_5a03;
        case 0x3f:
label_5be1:
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
                  goto label_5a99;
                }
                if (v23) goto label_5a0c;
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
                goto label_668c;
              }
            }
            v7 = 0;
            v20 = 0x3f;
            v25 = v3;
            v24 = 0;
            goto label_5a99;
          }
          if (v23) goto label_5a03;
          v7 = 0;
          v16 = 0x3f;
          v24 = 0;
          goto label_5ac3;
        
      }
label_5de0:
      v21 = v24;
      if (!v2) {
label_5deb:
        v16 = (unsigned char)v20;
        v24 = 0;
        v7 = 0;
        v25 = v3;
        if (!v23) goto label_5ac0;
        goto label_5918;
      }
label_5c42:
      v24 = 0;
      goto label_5c53;
    }
    if (!(bool)(v12 == 0 & v23 & a4 == 2)) {
      v7 = a4 == 2 & (v23 ^ 1U);
      v23 = (bool)(v23 ^ 1U);
      if ((!(bool)v7) || (v23 = (bool)v7, !v3)) {
label_690f:
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
        if (a1 || !v31) goto label_690f;
        v22 = v31;
        goto label_63db;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 1;
      v32 = (unsigned long)v9;
      v30 = "\"";
      if (!(v33 & 2)) goto label_69bf;
      v3 = 0;
      v19 = 0;
      v31 = 0;
      v2 = v4;
      v23 = v4;
      goto label_57d6;
    }
label_60dd:
    if (v2) {
label_5c7b:
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
label_5ca1:
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
label_63db:
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
label_6885:
    if (!a2[(long)v9]) break;
  }
label_688f:
  v24 = 0;
label_650b:
  v18 = (unsigned long)v16;
  v25 = v24;
  if (2 <= v20) {
label_6515:
    v25 = 0;
    v20 += v22;
    v13 = v22;
    do {
      v16 = (unsigned char)v18;
      if (v26) {
        v27 = a4 == 2;
        if (v23) goto label_5a03;
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
        if (v20 <= v22) goto label_59a1;
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
          goto label_6230;
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
label_5a7e:
  if (v26) {
    v24 = 0;
    v26 = v2;
    goto label_6515;
  }
label_5a90:
  v20 = v18;
  v24 = v25;
label_5a97:
  v7 = a4 == 2;
  v25 = v3;
label_5a99:
  v16 = (unsigned char)v20;
  v26 = (bool)v7 == 0;
  v3 = v25;
  v7 = 0;
  if ((v26 && v2) || (v7 = 0, v23)) {
label_5918:
    v16 = (unsigned char)v20;
    v12 = v13;
    if (!v28) goto label_5ac3;
    if (!(*(unsigned int *)(v28 + (v20 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_5ac3;
  }
  else {
label_5ac0:
    v7 = 0;
    v3 = v25;
label_5ac3:
    if (!v21) {
      v7 ^= 1;
      v22 += 1;
      v7 &= v19;
label_6230:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v19 = 0;
        v12 += 2;
      }
      goto label_59a1;
    }
  }
label_5c53:
  if (v23) {
    v23 = v2;
label_5c6f:
    if ((bool)(v23 & a4 == 2)) goto label_5c7b;
label_5a0c:
    v33 &= 0xfffffffd;
    v28 = 0;
    goto label_5700;
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
label_598e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v22 += 1;
label_59a1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v24)
    v4 = 0;
  goto label_57e0;
label_5eb2:
  v22 += 1;
  goto label_57e0;
label_69bf:
  a1 = v31;
label_5ae1:
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
  goto label_57c3;
}

// Function: sub_6ac0 @ 0x6ac0
char * sub_6ac0(unsigned int a0,char *a1,unsigned long a2,struct_1 *a3)
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
    v5 = dat_e080;
    if (dat_e078 <= (int)a0) {
      v10 = (long)dat_e078;
      v8 = (long)(int)((a0 - dat_e078) + 1);
      if (dat_e080 != (unsigned long *)0xe090) // branch-flip
        v5 = (unsigned long *)sub_8050(dat_e080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_8050(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_e090;
        v5[1] = dat_e098;
      }
      dat_e080 = v5;
      memset(&v5[(long)dat_e078 * 2],0,(v10 - dat_e078) * 0x10);
      dat_e078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_5690(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0xe180)
        free(v7);
      v7 = (char *)sub_7ed0(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_5690(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_6cb0 @ 0x6cb0
void sub_6cb0(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0xe280;
  sub_8190(a0,0x38);
  *v2 = v1;
}

// Function: sub_6cf0 @ 0x6cf0
unsigned int sub_6cf0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0xe280;
  return *a0;
}

// Function: sub_6d10 @ 0x6d10
void sub_6d10(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0xe280;
  *a0 = a1;
}

// Function: sub_6d30 @ 0x6d30
unsigned int sub_6d30(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0xe280;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_6d70 @ 0x6d70
unsigned int sub_6d70(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0xe280;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_6d90 @ 0x6d90
void sub_6d90(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0xe280;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_6dd0 @ 0x6dd0
void sub_6dd0(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0xe280;
  v2 = __errno_location();
  v1 = *v2;
  sub_5690(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_6e50 @ 0x6e50
char * sub_6e50(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0xe280;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_5690(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_7ed0(v3 + 1U);
  sub_5690(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_6f40 @ 0x6f40
char * sub_6f40(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_6e50(a0,a1,NULL,a2); // tail-call
}

// Function: sub_6f50 @ 0x6f50
void sub_6f50(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_e080;
  if (2 <= dat_e078) {
    v4 = (unsigned long *)((long)dat_e080 + 0x18);
    v1 = (unsigned long *)((long)dat_e080 + (unsigned long)(unsigned int)(dat_e078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0xe180) {
    free(*(void **)((long)v3 + 8));
    dat_e098 = 0xe180;
    dat_e090 = 0x100;
  }
  if (v3 == (void *)0xe090) {
    dat_e078 = 1;
    return;
  }
  free(v3);
  dat_e078 = 1;
  dat_e080 = (void *)0xe090;
}

// Function: sub_6ff0 @ 0x6ff0
char * sub_6ff0(unsigned int a0,char *a1)
{
  return sub_6ac0(a0,a1,0xffffffffffffffff,(struct_1 *)0xe280); // tail-call
}

// Function: sub_7010 @ 0x7010
char * sub_7010(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_6ac0(a0,a1,a2); // tail-call
}

// Function: sub_7020 @ 0x7020
char * sub_7020(char *a0)
{
  return sub_6ff0(0,a0); // tail-call
}

// Function: sub_7030 @ 0x7030
char * sub_7030(char *a0,unsigned long a1)
{
  return sub_7010(0,a0,a1); // tail-call
}

// Function: sub_7040 @ 0x7040
void sub_7040(unsigned int a0,unsigned long a1,char *a2)
{
  char v1 [56];
  
  sub_4f40(v1);
  sub_6ac0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_70a0 @ 0x70a0
void sub_70a0(unsigned int a0,unsigned long a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_4f40(v1);
  sub_6ac0(a0,a2,a3,v1);
}

// Function: sub_7110 @ 0x7110
void sub_7110(unsigned int a0,char *a1)
{
  sub_7040(0,a0,a1); // tail-call
}

// Function: sub_7120 @ 0x7120
void sub_7120(unsigned int a0,char *a1,unsigned long a2)
{
  sub_70a0(0,a0,a1,a2); // tail-call
}

// Function: sub_7140 @ 0x7140
void sub_7140(char *a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_e280;
  v3 = dat_e288;
  v4 = dat_e290;
  v5 = dat_e298;
  v8 = dat_e2b0;
  v6 = dat_e2a0;
  v7 = dat_e2a8;
  sub_6d30(&v1,(int)a2,1,a3,a0,a1);
  sub_6ac0(0,a0,a1,v2);
}

// Function: sub_71d0 @ 0x71d0
void sub_71d0(char *a0,char a1)
{
  sub_7140(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_71f0 @ 0x71f0
void sub_71f0(char *a0)
{
  sub_71d0(a0,0x3a); // tail-call
}

// Function: sub_7200 @ 0x7200
void sub_7200(char *a0,unsigned long a1)
{
  sub_7140(a0,a1,0x3a); // tail-call
}

// Function: sub_7210 @ 0x7210
void sub_7210(unsigned int a0,unsigned long a1,char *a2)
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
  
  sub_4f40(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_6d30(v3,0x3a,1);
  sub_6ac0(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_72b0 @ 0x72b0
void sub_72b0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_e280;
  v2 = dat_e288;
  v7 = dat_e2b0;
  v3 = dat_e290;
  v4 = dat_e298;
  v5 = dat_e2a0;
  v6 = dat_e2a8;
  sub_6d90(&v1,a1,a2);
  sub_6ac0(a0,a3,a4,&v1);
}

// Function: sub_7350 @ 0x7350
void sub_7350(unsigned int a0)
{
  sub_72b0(a0); // tail-call
}

// Function: sub_7360 @ 0x7360
void sub_7360(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_7350(0,a0,a1,a2); // tail-call
}

// Function: sub_7380 @ 0x7380
void sub_7380(long a0,long a1,char *a2,unsigned long a3)
{
  sub_72b0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_73a0 @ 0x73a0
char * sub_73a0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_6ac0(a0,a1,a2); // tail-call
}

// Function: sub_73b0 @ 0x73b0
char * sub_73b0(char *a0,unsigned long a1)
{
  return sub_73a0(0,a0,a1); // tail-call
}

// Function: sub_73c0 @ 0x73c0
char * sub_73c0(unsigned int a0,char *a1)
{
  return sub_73a0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_73d0 @ 0x73d0
char * sub_73d0(char *a0)
{
  return sub_73c0(0,a0); // tail-call
}

// Function: sub_73e0 @ 0x73e0
void sub_73e0(struct_8 *a0)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  unsigned long v4;
  long *v5;
  
  v1 = a0->field_0x30;
  v2 = a0->field_0x28;
  if (v1 == v2)
    a0->field_0x68 = a0->field_0x68 | 2;
  v3 = v1 + a0->field_0x48 & ~a0->field_0x48;
  v4 = a0->field_0x38;
  if (v3 - a0->field_0x20 <= a0->field_0x38 - a0->field_0x20)
    v4 = v3;
  a0->field_0x30 = v4;
  a0->field_0x28 = v4;
  v5 = a0->field_0x88;
  if ((unsigned long)(a0->field_0x90 - (long)v5) <= 7) {
    _obstack_newchunk((obstack *)&a0->field_0x69[7],8);
    v5 = a0->field_0x88;
  }
  *v5 = v2;
  v5 = a0->field_0xe0;
  a0->field_0x88 = &a0->field_0x88[1];
  if ((unsigned long)(a0->field_0xe8 - (long)v5) <= 7) {
    _obstack_newchunk((obstack *)&a0->field_0x98[0x30],8);
    v5 = a0->field_0xe0;
  }
  *v5 = (v1 - v2) + -1;
  a0->field_0xe0 = &a0->field_0xe0[1];
  a0->field_0x0 = a0->field_0x0 + 1;
}

// Function: sub_74d0 @ 0x74d0
void sub_74d0(unsigned long *a0)
{
  void *v1;
  void *v2;
  
  v2 = dat_dfe8;
  v1 = dat_dfc8;
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin((obstack *)&a0[3],0,0,dat_dfe8,dat_dfc8);
  _obstack_begin((obstack *)&a0[0xe],0,0,v2,v1);
  _obstack_begin((obstack *)&a0[0x19],0,0,v2,v1); // tail-call
}

// Function: sub_7550 @ 0x7550
void sub_7550(long a0)
{
  _obstack_free((obstack *)(a0 + 0x18),NULL);
  _obstack_free((obstack *)(a0 + 0x70),NULL);
  _obstack_free((obstack *)(a0 + 200),NULL); // tail-call
}

// Function: sub_7580 @ 0x7580
unsigned long sub_7580(FILE *a0,long a1)
{
  int v1; // eax
  char *v2;
  long v3; // rax
  unsigned long v4; // rax
  unsigned long *v5;
  unsigned long v6;
  
label_75ae:
  do {
    v1 = fgetc(a0);
    v2 = *(char **)(a1 + 0x30);
    if (v1 == -1) {
      if (v2 != *(char **)(a1 + 0x28)) {
        if (v2 == *(char **)(a1 + 0x38)) {
          _obstack_newchunk((obstack *)(a1 + 0x18),1);
          v2 = *(char **)(a1 + 0x30);
        }
        *(char **)(a1 + 0x30) = &v2[1];
        *v2 = 0;
        sub_73e0(a1);
      }
      v5 = *(unsigned long **)(a1 + 0x88);
      if ((unsigned long)(*(long *)(a1 + 0x90) - (long)v5) <= 7) {
        _obstack_newchunk((obstack *)(a1 + 0x70),8);
        v5 = *(unsigned long **)(a1 + 0x88);
      }
      *v5 = 0;
      v3 = *(long *)(a1 + 0x88) + 8;
      if (v3 == *(long *)(a1 + 0x80))
        *(unsigned char *)(a1 + 0xc0) = *(unsigned char *)(a1 + 0xc0) | 2;
      *(long *)(a1 + 8) = *(long *)(a1 + 0x80);
      v4 = v3 + *(unsigned long *)(a1 + 0xa0) & ~*(unsigned long *)(a1 + 0xa0);
      v6 = *(unsigned long *)(a1 + 0x90);
      if (v4 - *(long *)(a1 + 0x78) <= *(unsigned long *)(a1 + 0x90) - *(long *)(a1 + 0x78))
        v6 = v4;
      *(unsigned long *)(a1 + 0x88) = v6;
      *(unsigned long *)(a1 + 0x80) = v6;
      if (*(long *)(a1 + 0xe0) == *(long *)(a1 + 0xd8))
        *(unsigned char *)(a1 + 0x118) = *(unsigned char *)(a1 + 0x118) | 2;
      *(long *)(a1 + 0x10) = *(long *)(a1 + 0xd8);
      v4 = *(long *)(a1 + 0xe0) + *(unsigned long *)(a1 + 0xf8) & ~*(unsigned long *)(a1 + 0xf8);
      v6 = *(unsigned long *)(a1 + 0xe8);
      if (v4 - *(long *)(a1 + 0xd0) <= *(unsigned long *)(a1 + 0xe8) - *(long *)(a1 + 0xd0))
        v6 = v4;
      *(unsigned long *)(a1 + 0xe0) = v6;
      *(unsigned long *)(a1 + 0xd8) = v6;
      v1 = ferror(a0);
      return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
    }
    if (*(char **)(a1 + 0x38) != v2) goto label_75a0;
    _obstack_newchunk((obstack *)(a1 + 0x18),1);
    v2 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v2[1];
    *v2 = (char)v1;
  } while (v1);
  goto label_75f0;
label_75a0:
  *(char **)(a1 + 0x30) = &v2[1];
  *v2 = (char)v1;
  if (!v1) {
label_75f0:
    sub_73e0(a1);
  }
  goto label_75ae;
}

// Function: sub_7770 @ 0x7770
long sub_7770(int a0,void *a1,unsigned long a2)
{
  int v1;
  long v2; // rax
  
  while( true ) {
    do {
      v2 = read(a0,a1,a2);
      if (0 <= v2)
        return v2;
      v1 = *__errno_location();
    } while (v1 == 4);
    if (a2 <= 0x7ff00000) break;
    if (v1 != 0x16)
      return v2;
    a2 = 0x7ff00000;
  }
  return v2;
}

// Function: sub_77e0 @ 0x77e0
long sub_77e0(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_790d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_7946:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xa168) + 0xa168;
  switch(a5) {
    case 0:
      goto label_7946;
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
      goto label_790d;
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

// Function: sub_7c60 @ 0x7c60
void sub_7c60(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_77e0(); // tail-call
}

// Function: sub_7c80 @ 0x7c80
void sub_7c80(FILE *a0,char *a1,char *a2,char *a3,struct_9 *a4)
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
  sub_77e0(a0,a1,a2,a3);
}

// Function: sub_7d20 @ 0x7d20
void sub_7d20(void)
{
  sub_7c80();
}

// Function: sub_7dd0 @ 0x7dd0
void sub_7dd0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_7e80 @ 0x7e80
long sub_7e80(long a0)
{
  if (a0)
    return a0;
  sub_8250(); // no-return
}

// Function: sub_7e90 @ 0x7e90
void sub_7e90(unsigned long a0)
{
  sub_7e80(malloc(a0)); // tail-call
}

// Function: sub_7eb0 @ 0x7eb0
void sub_7eb0(void)
{
  sub_7e80(sub_85c0()); // tail-call
}

// Function: sub_7ed0 @ 0x7ed0
void sub_7ed0(void)
{
  sub_7e90(); // tail-call
}

// Function: sub_7ee0 @ 0x7ee0
void sub_7ee0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_8250(); // no-return
}

// Function: sub_7f20 @ 0x7f20
void sub_7f20(void)
{
  sub_7e80(sub_85d0()); // tail-call
}

// Function: sub_7f40 @ 0x7f40
void sub_7f40(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_8250(); // no-return
}

// Function: sub_7f80 @ 0x7f80
void sub_7f80(void *a0,unsigned long a1,unsigned long a2)
{
  sub_7f40(a0,a1,a2); // tail-call
}

// Function: sub_7f90 @ 0x7f90
void sub_7f90(void)
{
  sub_7e80(sub_8600()); // tail-call
}

// Function: sub_7fb0 @ 0x7fb0
void sub_7fb0(unsigned long a0,unsigned long a1)
{
  sub_7f40(NULL,a0,a1); // tail-call
}

// Function: sub_7fc0 @ 0x7fc0
void sub_7fc0(unsigned long a0,unsigned long a1)
{
  sub_7f90(0,a0,a1); // tail-call
}

// Function: sub_7fd0 @ 0x7fd0
void sub_7fd0(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_8250(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_7f40(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_8040 @ 0x8040
void sub_8040(void *a0,unsigned long *a1)
{
  sub_7fd0(a0,a1,1); // tail-call
}

// Function: sub_8050 @ 0x8050
void sub_8050(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_80f6;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_80f6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_8250(a0,v3); // no-return
  sub_7ee0(a0,v3);
  *a1 = v2;
}

// Function: sub_8130 @ 0x8130
void sub_8130(unsigned long a0,unsigned long a1)
{
  sub_7e80(calloc(a0,a1)); // tail-call
}

// Function: sub_8150 @ 0x8150
void sub_8150(unsigned long a0)
{
  sub_8130(a0,1); // tail-call
}

// Function: sub_8160 @ 0x8160
void sub_8160(void)
{
  sub_7e80(sub_85f0()); // tail-call
}

// Function: sub_8180 @ 0x8180
void sub_8180(unsigned long a0)
{
  sub_8160(a0,1); // tail-call
}

// Function: sub_8190 @ 0x8190
void sub_8190(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7e90(a1),a0,a1); // tail-call
}

// Function: sub_81c0 @ 0x81c0
void sub_81c0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7eb0(a1),a0,a1); // tail-call
}

// Function: sub_81f0 @ 0x81f0
void sub_81f0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_7eb0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_8230 @ 0x8230
void sub_8230(char *a0)
{
  sub_8190(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_8250 @ 0x8250
void sub_8250(void)
{
  error(dat_e020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_8290 @ 0x8290
unsigned long sub_8290(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_82c0 @ 0x82c0
bool sub_82c0(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_82e0 @ 0x82e0
bool sub_82e0(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_82f0 @ 0x82f0
unsigned int sub_82f0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_8310 @ 0x8310
unsigned long sub_8310(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_8330 @ 0x8330
bool sub_8330(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_8340 @ 0x8340
bool sub_8340(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_8350 @ 0x8350
bool sub_8350(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_8360 @ 0x8360
bool sub_8360(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_8370 @ 0x8370
unsigned long sub_8370(int a0)
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

// Function: sub_83c0 @ 0x83c0
bool sub_83c0(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_83e0 @ 0x83e0
bool sub_83e0(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_83f0 @ 0x83f0
unsigned int sub_83f0(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_8420 @ 0x8420
int sub_8420(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_8440 @ 0x8440
int sub_8440(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_8460 @ 0x8460
int sub_8460(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_8420((unsigned char)*a0);
    v1 = sub_8420((unsigned char)*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_84c0 @ 0x84c0
unsigned long sub_84c0(FILE *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_4960(a0);
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

// Function: sub_8530 @ 0x8530
bool sub_8530(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_8a70(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_85c0 @ 0x85c0
void sub_85c0(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_85d0 @ 0x85d0
void sub_85d0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_85f0 @ 0x85f0
void sub_85f0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_8600 @ 0x8600
void sub_8600(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_8630 @ 0x8630
char * sub_8630(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_8670 @ 0x8670
void sub_8670(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x38))(a1); // jump-as-call
}

// Function: sub_8690 @ 0x8690
void sub_8690(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x40))(a1); // jump-as-call
}

// Function: sub_86b0 @ 0x86b0
unsigned long sub_86b0(struct_7 *a0,long a1,long a2)
{
  long *v1; // rax
  unsigned long v2; // rdx
  long v3;
  
  if (a2) // branch-flip
    v3 = a2 + -1;
  else {
    v3 = 0xf;
    a2 = 0x10;
  }
  a0->field_0x30 = v3;
  if (!a1)
    a1 = 0xfe0;
  a0->field_0x0 = a1;
  v1 = (long *)sub_8670(a0);
  a0->field_0x8 = v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0->field_0x10 = v2;
    a0->field_0x18 = v2;
    v3 = a0->field_0x0 + (long)v1;
    *v1 = v3;
    a0->field_0x20 = v3;
    v1[1] = 0;
    a0->field_0x50 = a0->field_0x50 & 0xf9;
    return 1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_8740 @ 0x8740
void sub_8740(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_e020); // no-return
}

// Function: _obstack_begin @ 0x8790
int _obstack_begin(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4)
{
  a0->field_0x50 = a0->field_0x50 & 0xfe;
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  return sub_86b0(a0,a1,a2); // tail-call
}

// Function: _obstack_begin_1 @ 0x87b0
int _obstack_begin_1(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4,void *a5)
{
  a0->field_0x50 = a0->field_0x50 | 1;
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  *(void **)&a0->field_0x48 = a5;
  return sub_86b0(a0,a1,a2); // tail-call
}

// Function: _obstack_newchunk @ 0x87d0
void _obstack_newchunk(obstack *a0,unsigned long a1)
{
  unsigned long v1;
  long v2;
  void *v3;
  long *v4; // rax
  unsigned long v5;
  unsigned long v6; // r13
  void *v7; // r14
  
  v6 = *(long *)&a0->field_0x18 - *(long *)&a0->field_0x10;
  v2 = *(long *)&a0->field_0x8;
  v5 = a1 + v6 + *(long *)&a0->field_0x30;
  v1 = v5 + 100 + (v6 >> 3);
  if (v5 < *(unsigned long *)a0)
    v5 = *(unsigned long *)a0;
  if (v5 <= v1)
    v5 = v1;
  if ((!CARRY8(a1,v6)) && (!CARRY8(a1 + v6,*(unsigned long *)&a0->field_0x30))) {
    v4 = (long *)sub_8670(a0);
    if (v4) {
      *(long **)&a0->field_0x8 = v4;
      v3 = *(void **)&a0->field_0x10;
      v4[1] = v2;
      *(unsigned long *)&a0->field_0x20 = (long)v4 + v5;
      *v4 = (long)v4 + v5;
      v7 = (void *)((long)v4 + *(unsigned long *)&a0->field_0x30 + 0x10 & ~*(unsigned long *)&a0->field_0x30);
      memcpy(v7,v3,v6);
      if ((!(a0->field_0x50 & 2)) && (*(unsigned long *)&a0->field_0x10 == (~*(unsigned long *)&a0->field_0x30 & v2 + 0x10 + *(unsigned long *)&a0->field_0x30))) {
        v4[1] = *(long *)(v2 + 8);
        sub_8690(a0,v2);
      }
      *(void **)&a0->field_0x10 = v7;
      *(unsigned long *)&a0->field_0x18 = (long)v7 + v6;
      a0->field_0x50 = a0->field_0x50 & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x88d0
unsigned long _obstack_allocated_p(long a0,unsigned long *a1)
{
  unsigned long *v1;
  
  v1 = *(unsigned long **)(a0 + 8);
  if (!v1)
    return 0;
  while ((a1 <= v1 || ((unsigned long *)*v1 < a1))) {
    v1 = (unsigned long *)v1[1];
    if (!v1)
      return 0;
  }
  return 1;
}

// Function: _obstack_free @ 0x8910
void _obstack_free(obstack *a0,void *a1)
{
  void *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)&a0->field_0x8;
  if (v2) {
    do {
      if ((v2 < a1) && (v1 = (void *)*v2, a1 <= v1)) {
        *(void **)&a0->field_0x18 = a1;
        *(void **)&a0->field_0x10 = a1;
        *(void **)&a0->field_0x20 = v1;
        *(unsigned long **)&a0->field_0x8 = v2;
        return;
      }
      v2 = (unsigned long *)v2[1];
      sub_8690(a0);
      a0->field_0x50 = a0->field_0x50 | 2;
    } while (v2);
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x8980
unsigned long _obstack_memory_used(obstack *a0)
{
  long *v1;
  unsigned long v2;
  
  v2 = 0;
  for (v1 = *(long **)&a0->field_0x8; v1; v1 = (long *)v1[1]) {
    v2 += *v1 - (long)v1;
  }
  return v2;
}

// Function: sub_89b0 @ 0x89b0
void sub_89b0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_89c0 @ 0x89c0
unsigned long sub_89c0(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_89b0();
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

// Function: sub_8a70 @ 0x8a70
unsigned long sub_8a70(void)
{
  return sub_89c0(); // tail-call
}

// Function: sub_8a80 @ 0x8a80
void sub_8a80(void)
{
  sub_89b0(); // tail-call
}

// Function: sub_8a90 @ 0x8a90
unsigned long sub_8a90(char *a0,unsigned int a1,long *a2,long *a3)
{
  char v1 [32];
  int v10; // r8d
  int v11; // r8d
  int v12; // r8d
  int v13; // r8d
  int v14; // r9d
  int v15; // r9d
  bool v16; // r13b
  unsigned long v17; // r13
  unsigned long v18; // r13
  long v19;
  unsigned long v2; // rax
  char v20 [32];
  char v21 [32];
  char v22 [64]; // zmm3
  char (*v3)[32];
  char *v4; // rax
  int v5; // edx
  char (*v6)[32];
  long v7; // rbx
  char v8 [64];
  char v9 [16280];
  
  v16 = !a2 || !a3;
  v18 = CONCAT71((undefined7)((unsigned long)v17 >> 8),v16);
  if (v16)
    v18 = 0;
  else {
    v7 = 0;
    v19 = 0;
    while (v2 = sub_7770(a1,(char (*)[32])v8,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = (char *)sub_7210(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v18 & 0xffffffff; // return-dupe
      }
      v7 += v2;
      if (0x40 <= v2) { // branch-flip
        v22 = ZEXT1664(0);
        v21 = SUB6432(v22,0);
        v6 = (char (*)[32])v8;
        do {
          v20 = vpcmpeqb_avx2(dat_a200,*v6);
          v1 = vpcmpeqb_avx2(dat_a200,v6[1]);
          v6 = &v6[2];
          v21 = vpsubb_avx2(v21,v20);
          v20 = vpsubb_avx2(SUB6432(v22,0),v1);
          v22 = ZEXT3264(v20);
          v3 = (char (*)[32])&v9[v2 - 0x40 & 0xffffffffffffffc0];
        } while (v6 != (char (*)[32])&v9[v2 - 0x40 & 0xffffffffffffffc0]);
      }
      else {
        v21 = SUB6432(ZEXT1664(0),0);
        v3 = (char (*)[32])v8;
        v20 = v21;
      }
      v21 = vpsadbw_avx2(v21,SUB6432(ZEXT1664(0),0));
      v20 = vpsadbw_avx2(v20,SUB6432(ZEXT1664(0),0));
      v10 = vpextrw_avx(SUB3216(v21,0),0);
      v5 = vpextrw_avx(SUB3216(v21,0),4);
      v11 = vpextrw_avx(SUB3216(v21,0x10),0);
      v14 = vpextrw_avx(SUB3216(v20,0),0);
      v12 = vpextrw_avx(SUB3216(v21,0x10),4);
      v13 = vpextrw_avx(SUB3216(v20,0),4);
      v15 = vpextrw_avx(SUB3216(v20,0x10),0);
      v19 += (long)(v5 + v10 + v11 + v12) + (long)(v13 + v14 + v15 + vpextrw_avx(SUB3216(v20,0x10),4));
      for (; (char (*)[32])&(*(char (*)[32])v8)[v2] != v3; v3 = (char (*)[32])&(*v3)[1]) {
        v19 += (unsigned long)((*v3)[0] == '\n');
      }
    }
    v18 = 1;
    *a2 = v19;
    *a3 = v7;
  }
  return v18 & 0xffffffff;
}

// Function: sub_8cd0 @ 0x8cd0
void sub_8cd0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_e008); // tail-call
}

// Function: _DT_FINI @ 0x8ce4
void _DT_FINI(void)
{
  return;
}
