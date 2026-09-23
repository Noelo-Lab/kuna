// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_cfd0)
    return;
  (*dat_cfd0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_ce18)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2370
void __cxa_finalize(void)
{
  (*dat_cfe0)(); // jump-as-call
}

// Function: free @ 0x2380
void free(void *a0)
{
  (*dat_ce20)(); // jump-as-call
}

// Function: abort @ 0x2390
void abort(void)
{
  (*dat_ce28)(); // jump-as-call
}

// Function: __errno_location @ 0x23a0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_ce30)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x23b0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_ce38)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x23c0
void _exit(int a0)
{
  (*dat_ce40)(); // jump-as-call
}

// Function: __fpending @ 0x23d0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce48)(); // jump-as-call
  return v1;
}

// Function: getc_unlocked @ 0x23e0
int getc_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ce50)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x23f0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ce58)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x2400
void clearerr_unlocked(FILE *a0)
{
  (*dat_ce60)(); // jump-as-call
}

// Function: textdomain @ 0x2410
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce68)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2420
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ce70)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2430
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce78)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2440
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce80)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2450
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce88)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2460
void __stack_chk_fail(void)
{
  (*dat_ce90)(); // jump-as-call
}

// Function: getopt_long @ 0x2470
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ce98)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2480
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_cea0)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x2490
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cea8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x24a0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ceb0)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x24b0
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_ceb8)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x24c0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cec0)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x24d0
void lseek(void)
{
  (*dat_cec8)(); // jump-as-call
}

// Function: __assert_fail @ 0x24e0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_ced0)(); // jump-as-call
}

// Function: fputs @ 0x24f0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_ced8)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x2500
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cee0)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x2510
void posix_fadvise(void)
{
  (*dat_cee8)(); // jump-as-call
}

// Function: memcmp @ 0x2520
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_cef0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2530
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_cef8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2540
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf00)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2550
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf08)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2560
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_cf10)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x2570
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf18)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2580
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf20)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x2590
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf28)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x25a0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf30)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x25b0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf38)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x25c0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf40)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x25d0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf48)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x25e0
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf50)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x25f0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_cf58)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2600
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf60)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2610
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf68)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x2620
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf70)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2630
void error(int a0,int a1,char *a2,...)
{
  (*dat_cf78)(); // jump-as-call
}

// Function: fseeko @ 0x2640
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_cf80)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x2650
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_cf88)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x2660
void strtoumax(void)
{
  (*dat_cf90)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x2670
void __cxa_atexit(void)
{
  (*dat_cf98)(); // jump-as-call
}

// Function: exit @ 0x2680
void exit(int a0)
{
  (*dat_cfa0)(); // jump-as-call
}

// Function: mbsinit @ 0x2690
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_cfa8)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x26a0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cfb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x26b0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cfb8)(); // jump-as-call
  return v1;
}

// Function: sub_26c0 @ 0x26c0
void sub_26c0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_cfc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_26f0 @ 0x26f0
void sub_26f0(void)
{
  return;
}

// Function: _FINI_0 @ 0x2760
void _FINI_0(void)
{
  if (!dat_d0e8) {
    if (dat_cfe0)
      __cxa_finalize(dat_d008);
    sub_26f0();
    dat_d0e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x27a0
void _INIT_0(void)
{
  return;
}

// Function: sub_27a9 @ 0x27a9
void sub_27a9(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nWith no FILE, or when FILE is -, read standard input.\n"),v1);
}

// Function: sub_27da @ 0x27da
void sub_27da(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nMandatory arguments to long options are mandatory for short options too.\n"),v1);
}

// Function: sub_280b @ 0x280b
void sub_280b(char *a0) // ternary
{
  FILE *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  char *v14; // stack - 0x58
  char *v15; // stack - 0x50
  char *v16; // stack - 0x48
  char *v17; // stack - 0x40
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x30
  int v2; // eax
  char *v3;
  char *v4; // stack - 0x98
  char *v5; // stack - 0xb8
  char **v6; // stack - 0xb0
  char *v7; // stack - 0x90
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v4 = "[";
  v7 = "test invocation";
  v8 = "coreutils";
  v9 = "Multi-call invocation";
  v10 = "sha224sum";
  v11 = "sha2 utilities";
  v12 = "sha256sum";
  v13 = "sha2 utilities";
  v14 = "sha384sum";
  v15 = "sha2 utilities";
  v16 = "sha512sum";
  v17 = "sha2 utilities";
  v18 = 0;
  v19 = 0;
  v6 = &v4;
  while ((*v6 && (v3 = *v6, strcmp(a0,v3)))) {
    v6 = &v6[2];
  }
  v5 = a0;
  if (v6[1])
    v5 = v6[1];
  printf(gettext("\n%s online help: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(gettext("Report any translation bugs to <https://translationproject.org/team/>\n"),v1);
  if (strcmp(a0,"[")) // branch-flip
    v3 = a0;
  else {
    v3 = "test";
  }
  printf(gettext("Full documentation <%s%s>\n"),"https://www.gnu.org/software/coreutils/",v3);
  v3 = (v5 != a0) ? "" : " invocation"; // branch-flip
  printf(gettext("or available locally via: info \'(coreutils) %s%s\'\n"),v5,v3);
}

// Function: sub_2aa2 @ 0x2aa2
void sub_2aa2(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_18158;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [-WIDTH] [OPTION]... [FILE]...\n"),v2);
    v1 = stdout;
    fputs_unlocked(gettext("Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n"),v1);
    sub_27a9();
    sub_27da();
    v1 = stdout;
    fputs_unlocked(gettext("  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_280b("fmt");
  }
  exit(a0); // no-return
}

// Function: main @ 0x2bde
int main(int argc,char **argv,char **envp)
{
  bool v1;
  unsigned long v10; // rax
  char **v11; // stack - 0x58
  int v12; // stack - 0x4c
  bool v13; // stack - 0x3e
  char *v14; // stack - 0x38
  char *v15; // stack - 0x30
  char v2;
  char v3;
  char v4;
  unsigned char v5; // al
  int v6; // eax
  int v7; // eax
  char *v8;
  FILE *v9; // rax
  
  v13 = 1;
  v14 = NULL;
  v15 = NULL;
  sub_498f(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8540(sub_4625);
  dat_d103 = 0;
  dat_d102 = 0;
  dat_d101 = 0;
  dat_d100 = 0;
  dat_d110 = 0x4b;
  dat_d108 = 0x91c3;
  dat_d114 = 0;
  dat_d118 = 0;
  dat_d11c = 0;
  v2 = 0;
  v3 = 0;
  v4 = 0;
  v11 = argv;
  v12 = argc;
  if (((2 <= argc) && (v2 = dat_d100, v3 = dat_d101, v4 = dat_d102, *argv[1] == '-')) && ((unsigned int)((int)argv[1][1] - 0x30U) <= 9)) {
    v14 = &argv[1][1];
    argv[1] = *argv;
    v11 = &argv[1];
    v12 = argc + -1;
  }
  while( true ) {
    dat_d102 = v4;
    dat_d101 = v3;
    dat_d100 = v2;
    v6 = getopt_long(v12,v11,"0123456789cstuw:p:g:",(option *)0xca80,NULL);
    if (v6 == -1) {
      if (v14)
        dat_d110 = sub_7afa(v14,0,0x9c4,"",gettext("invalid width"),0);
      if (v15) { // branch-flip
        v8 = gettext("invalid width");
        dat_d120 = sub_7afa(v15,0,(long)dat_d110,"",v8,0);
        if (!v14)
          dat_d110 = dat_d120 + 10;
      }
      else {
        dat_d120 = (dat_d110 * 0xbb) / 200;
      }
      v1 = 0;
      if (v12 != optind) { // branch-flip
        for (; optind < v12; optind = optind + 1) {
          v8 = v11[optind];
          if (strcmp(v8,"-")) { // branch-flip
            v9 = fopen(v8,"r");
            if (v9) // branch-flip
              v13 = (sub_31d1(v9,v8) & v13) != 0;
            else {
              v10 = sub_6589(4,v8);
              v8 = gettext("cannot open %s for reading");
              error(0,*__errno_location(),v8,v10);
              v13 = 0;
            }
          }
          else {
            v5 = sub_31d1(stdin,v8);
            v13 = (v5 & v13) != 0;
            v1 = 1;
          }
        }
      }
      else {
        v1 = 1;
        v13 = sub_31d1(stdin,"-");
      }
      if ((v1) && (v6 = sub_4791(stdin), v6)) {
        v8 = gettext("closing standard input");
        error(1,*__errno_location(),"%s",v8);
        return v7;
      }
      return (int)(v13 ^ 1);
    }
    if (0x77 < v6) break;
    if (v6 <= 0x62) {
      if (v6 == -0x83) {
        sub_7168(stdout,"fmt","GNU coreutils",dat_d010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v6 == -0x82)
        sub_2aa2(0); // no-return
      break;
    }
    v2 = dat_d100;
    v3 = dat_d101;
    v4 = dat_d102;
    switch(v6) {
      case 99:
        dat_d100 = 1;
        v2 = dat_d100;
        break;
      default:
        goto label_2d94;
      case 0x67:
        v15 = optarg;
        break;
      case 0x70:
        sub_312d(optarg);
        v2 = dat_d100;
        v3 = dat_d101;
        v4 = dat_d102;
        break;
      case 0x73:
        dat_d102 = 1;
        v4 = dat_d102;
        break;
      case 0x74:
        dat_d101 = 1;
        v3 = dat_d101;
        break;
      case 0x75:
        dat_d103 = 1;
        break;
      case 0x77:
        v14 = optarg;
      
    }
  }
label_2d94:
  if ((unsigned int)(v6 - 0x30U) <= 9)
    error(0,0,gettext("invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead"),(char)v6);
  sub_2aa2(1); // no-return
}

// Function: sub_312d @ 0x312d
void sub_312d(char *a0)
{
  char *v1; // stack - 0x20
  char *v2; // stack - 0x10
  
  dat_d118 = 0;
  for (v1 = a0; *v1 == ' '; v1 = &v1[1]) {
    dat_d118 += 1;
  }
  dat_d108 = v1;
  dat_d114 = (int)strlen(v1);
  for (v2 = &v1[dat_d114]; (v1 < v2 && (v2[-1] == ' ')); v2 = &v2[-1]) {
  }
  *v2 = '\0';
  dat_d11c = (int)v2 - (int)v1;
}

// Function: sub_31d1 @ 0x31d1
unsigned int sub_31d1(FILE *a0,char *a1) // ternary
{
  unsigned long v1; // rax
  char *v2; // rax
  unsigned int v3; // stack - 0x1c
  
  sub_4752(a0,2);
  dat_18128 = 0;
  dat_18134 = 0;
  dat_18138 = sub_3978(a0);
  while( true ) {
    if (!sub_33b7(a0)) break;
    sub_3db9();
    sub_411d(dat_18120);
  }
  v3 = (ferror_unlocked(a0)) ? 0 : 0xffffffff; // branch-flip
  if (a0 != stdin) { // branch-flip
    if ((sub_4791(a0)) && ((int)v3 <= -1))
      v3 = *__errno_location();
  }
  else {
    clearerr_unlocked(a0);
  }
  if (0 <= (int)v3) {
    v1 = sub_671b(0,3,a1);
    if (v3) // branch-flip
      v2 = "%s";
    else {
      v2 = gettext("read error");
    }
    error(0,v3,v2,v1);
  }
  return v3 >> 0x1f;
}

// Function: sub_32fb @ 0x32fb
void sub_32fb(bool a0) // return-dupe x4, return-dupe x2
{
  if (dat_d102) {
    dat_18134 = dat_18130;
    return;
  }
  if (dat_d100) {
    if (a0) {
      dat_18134 = dat_d124;
      return;
    }
    dat_18134 = dat_18130;
    return;
  }
  if (!dat_d101) {
    dat_18134 = dat_18130;
    return;
  }
  if ((a0) && (dat_d124 != dat_18130)) {
    dat_18134 = dat_d124;
    return;
  }
  if (dat_18134 != dat_18130)
    return;
  if (!dat_18130) {
    dat_18134 = 3;
    return;
  }
  dat_18134 = 0;
}

// Function: sub_33b7 @ 0x33b7
unsigned long sub_33b7(FILE *a0) // return-dupe
{
  int v1; // stack - 0xc
  
  dat_18140 = 0;
  v1 = dat_18138;
  while ((((v1 == 10 || (v1 == -1)) || (dat_1813c < dat_d118)) || (dat_d124 < dat_1813c + dat_d114))) {
    if (sub_3630(a0,v1) == -1) {
      dat_18138 = 0xffffffff;
      return 0;
    }
    putchar_unlocked(10);
    v1 = sub_3978(a0);
  }
  dat_1812c = dat_1813c;
  dat_18130 = dat_d124;
  dat_e4c8 = 0xd140;
  dat_18120 = 0xe4e0;
  v1 = sub_3793(a0,v1);
  sub_32fb((unsigned char)sub_3743(v1));
  if (!dat_d102) {
    if (dat_d100) { // branch-flip
      if (sub_3743(v1)) {
        do {
          v1 = sub_3793(a0,v1);
          if (!sub_3743(v1)) break;
        } while (dat_d124 == dat_18134);
      }
    }
    else if (dat_d101) { // branch-flip
      if ((sub_3743(v1)) && (dat_d124 != dat_18130)) {
        do {
          v1 = sub_3793(a0,v1);
          if (!sub_3743(v1)) break;
        } while (dat_d124 == dat_18134);
      }
    }
    else {
      while ((sub_3743(v1) && (dat_d124 == dat_18134))) {
        v1 = sub_3793(a0,v1);
      }
    }
  }
  if (0xe4e1 <= dat_18120) {
    *(unsigned char *)(dat_18120 - 0x18) = *(unsigned char *)(dat_18120 - 0x18) | 8;
    *(unsigned char *)(dat_18120 - 0x18) = *(unsigned char *)(dat_18120 - 0x18) & 0xfd | (*(unsigned char *)(dat_18120 - 0x18) >> 3 & 1) * '\x02';
    dat_18138 = v1;
    return 1;
  }
  __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph"); // no-return
}

// Function: sub_3630 @ 0x3630
int sub_3630(FILE *a0,int a1)
{
  char *v1;
  int v2; // stack - 0x24
  char *v3; // stack - 0x10
  
  dat_d128 = 0;
  v2 = a1;
  if ((dat_1813c < dat_d124) || ((a1 != 10 && (a1 != -1)))) {
    sub_42aa(dat_1813c);
    v3 = dat_d108;
    while ((dat_d128 != dat_d124 && (*v3))) {
      v1 = &v3[1];
      putchar_unlocked((int)*v3);
      dat_d128 += 1;
      v3 = v1;
    }
    if ((a1 != -1) && (a1 != 10))
      sub_42aa(dat_d124 - dat_d128);
    if ((a1 == -1) && (dat_1813c + dat_d11c <= dat_d124))
      putchar_unlocked(10);
  }
  while ((v2 != 10 && (v2 != -1))) {
    putchar_unlocked(v2);
    v2 = getc_unlocked(a0);
  }
  return v2;
}

// Function: sub_3743 @ 0x3743
unsigned int sub_3743(int a0) // return-dupe
{
  if ((((dat_1813c == dat_1812c) && (dat_1813c + dat_d114 <= dat_d124)) && (a0 != 10)) && (a0 != -1))
    return 1;
  return 0;
}

// Function: sub_3793 @ 0x3793
void sub_3793(FILE *a0,int a1) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_3793
{
  char *v1;
  int v2;
  char v3; // al
  unsigned int v4; // edx
  int v5; // stack - 0x34
  
  v5 = a1;
  do {
    *dat_18120 = (long)dat_e4c8;
    do {
      if (dat_e4c8 == (char *)0xe4c8) {
        sub_32fb(1);
        sub_3bd2();
      }
      v1 = &dat_e4c8[1];
      *dat_e4c8 = (char)v5;
      dat_e4c8 = v1;
      v5 = getc_unlocked(a0);
      if (v5 == -1) break;
    } while (sub_4512(v5) != '\x01');
    *(int *)&dat_18120[1] = (int)dat_e4c8 - (int)*dat_18120;
    dat_d124 += (int)dat_18120[1];
    sub_3aba(dat_18120);
    v2 = dat_d124;
    v5 = sub_3a4c(a0,v5);
    *(int *)((long)dat_18120 + 0xc) = dat_d124 - v2;
    v3 = !((v5 != -1) && ((!(*(unsigned char *)&dat_18120[2] & 2) || ((v5 != 10 && (*(int *)((long)dat_18120 + 0xc) <= 1))))));
    *(unsigned char *)&dat_18120[2] = *(unsigned char *)&dat_18120[2] & 0xf7 | v3 * '\b';
    if (((v5 == 10) || (v5 == -1)) || (dat_d103)) {
      v4 = (*(unsigned char *)&dat_18120[2] & 8) ? 2 : 1; // branch-flip
      *(unsigned int *)((long)dat_18120 + 0xc) = v4;
    }
    if (dat_18120 == (long *)0x180d0) {
      sub_32fb(1);
      sub_3bd2();
    }
    dat_18120 = &dat_18120[5];
    if ((v5 == 10) || (v5 == -1)) {
      sub_3978(a0);
      return;
    }
  } while( true );
}

// Function: sub_3978 @ 0x3978
unsigned int sub_3978(FILE *a0)
{
  unsigned int v1; // stack - 0x14
  unsigned char *v2; // stack - 0x10
  
  dat_d124 = 0;
  v1 = sub_3a4c(a0,getc_unlocked(a0));
  if (dat_d11c) { // branch-flip
    dat_1813c = dat_d124;
    for (v2 = dat_d108; *v2; v2 = &v2[1]) {
      if (v1 != *v2)
        return v1;
      dat_d124 += 1;
      v1 = getc_unlocked(a0);
    }
    v1 = sub_3a4c(a0,v1);
  }
  else {
    dat_1813c = dat_d118;
    if (dat_d124 <= dat_d118)
      dat_1813c = dat_d124;
  }
  return v1;
}

// Function: sub_3a4c @ 0x3a4c
int sub_3a4c(FILE *a0,int a1)
{
  int v1; // stack - 0x14
  
  v1 = a1;
  do {
    if (v1 != 0x20) { // branch-flip
      if (v1 != 9)
        return v1;
      dat_18128 = 1;
      if (dat_d124 <= -1)
        dat_d124 += 7;
      dat_d124 = ((dat_d124 >> 3) + 1) * 8;
    }
    else {
      dat_d124 += 1;
    }
    v1 = getc_unlocked(a0);
  } while( true );
}

// Function: sub_3aba @ 0x3aba
void sub_3aba(struct_0 *a0)
{
  unsigned char v1;
  unsigned char v2;
  unsigned char *v3;
  unsigned char *v4; // stack - 0x18
  
  v3 = a0->field_0x0;
  v4 = &v3[(long)a0->field_0x8 + -1];
  v1 = *v4;
  v2 = *v3;
  a0->field_0x10 = a0->field_0x10 & 0xfe | strchr("([\'`\"",(int)(char)v2) != NULL;
  a0->field_0x10 = a0->field_0x10 & 0xfb | ((*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 4) != 0) * '\x04';
  for (; v3 < v4; v4 = &v4[-1]) {
    v1 = *v4;
    if (!strchr(")]\'\"",(int)(char)v1)) break;
  }
  v1 = *v4;
  a0->field_0x10 = a0->field_0x10 & 0xfd | (strchr(".?!",(int)(char)v1) != NULL) * '\x02';
}

// Function: sub_3bd2 @ 0x3bd2
void sub_3bd2(void) // return-dupe
{
  int v1; // eax
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  long v4; // stack - 0x10
  
  if (dat_18120 == (long *)0xe4e0) {
    fwrite_unlocked((void *)0xd140,1,dat_e4c8 - 0xd140,stdout);
    dat_e4c8 = 0xd140;
    return;
  }
  sub_3db9();
  v2 = dat_18120;
  v4 = 0x7fffffffffffffff;
  for (v3 = dat_e500; v3 != dat_18120; v3 = (long *)v3[4]) {
    if (v3[3] - *(long *)(v3[4] + 0x18) < v4) {
      v2 = v3;
      v4 = v3[3] - *(long *)(v3[4] + 0x18);
    }
    if (v4 <= 0x7ffffffffffffff6)
      v4 += 9;
  }
  sub_411d(v2);
  memmove((void *)0xd140,(void *)*v2,dat_e4c8 - *v2);
  v1 = (int)*v2 + -0xd140;
  dat_e4c8 -= v1;
  for (v3 = v2; v3 <= dat_18120; v3 = &v3[5]) {
    *v3 = *v3 - (long)v1;
  }
  memmove((void *)0xe4e0,v2,(long)dat_18120 + (0x28U - (long)v2));
  dat_18120 = (long *)((long)dat_18120 - (long)&v2[-0x1c9c]);
}

// Function: sub_3db9 @ 0x3db9
void sub_3db9(void)
{
  unsigned long v1;
  unsigned int v2;
  unsigned long v3;
  int v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  long v7; // stack - 0x18
  long v8; // stack - 0x10
  
  *(unsigned long *)(dat_18120 + 0x18) = 0;
  v2 = *(unsigned int *)(dat_18120 + 8);
  *(int *)(dat_18120 + 8) = dat_d110;
  v3 = dat_18120;
  while (v5 = v3 - 0x28, 0xe4e0 <= v5) {
    v8 = 0x7fffffffffffffff;
    if (v5 != 0xe4e0) // branch-flip
      v4 = dat_18134;
    else {
      v4 = dat_18130;
    }
    v6 = v5;
    v4 += *(int *)(v3 - 0x20);
    do {
      v1 = v6 + 0x28;
      v7 = sub_406b(v1,v4);
      v7 = *(long *)(v6 + 0x40) + v7;
      if ((v5 == 0xe4e0) && (1 <= dat_18140))
        v7 += ((long)((v4 - dat_18140) * 10) * (long)((v4 - dat_18140) * 10)) / 2;
      if (v7 < v8) {
        v8 = v7;
        *(unsigned long *)(v3 - 8) = v1;
        *(int *)(v3 - 0x14) = v4;
      }
    } while ((v1 != dat_18120) && (v4 = v4 + *(int *)(v6 + 0x30) + *(int *)(v6 + 0xc), v6 = v1, v4 < dat_d110));
    *(long *)(v3 - 0x10) = v8 + sub_3f67(v5);
    v3 = v5;
  }
  *(unsigned int *)(dat_18120 + 8) = v2;
}

// Function: sub_3f67 @ 0x3f67
long sub_3f67(unsigned long a0)
{
  char v1 [16];
  char v2 [16];
  long v3; // stack - 0x10
  
  v3 = 0x1324;
  if (0xe4e1 <= a0) {
    if (*(unsigned char *)(a0 - 0x18) & 2) { // branch-flip
      if (*(unsigned char *)(a0 - 0x18) & 8) // branch-flip
        v3 = 0x960;
      else {
        v3 = 0x59164;
      }
    }
    else if (*(unsigned char *)(a0 - 0x18) & 4) // branch-flip
      v3 = 0xce4;
    else if ((0xe509 <= a0) && (*(unsigned char *)(a0 - 0x40) & 8)) {
      v1._8_8_ = 0;
      v1._0_8_ = SUB168(SEXT816(40000),8);
      v3 = SUB168((v1._0_16_ << 0x40 | ZEXT816(40000)) / SEXT816((long)(*(int *)(a0 - 0x20) + 2)),0) + 0x1324;
    }
  }
  if (*(unsigned char *)(a0 + 0x10) & 1) // branch-flip
    v3 -= 0x640;
  else if (*(unsigned char *)(a0 + 0x10) & 8) {
    v2._8_8_ = 0;
    v2._0_8_ = SUB168(SEXT816(0x57e4),8);
    v3 += SUB168((v2._0_16_ << 0x40 | ZEXT816(0x57e4)) / SEXT816((long)(*(int *)(a0 + 8) + 2)),0);
  }
  return v3;
}

// Function: sub_406b @ 0x406b
long sub_406b(void *a0,int a1) // early-return
{
  long v1; // stack - 0x10
  
  if (a0 == dat_18120)
    return 0;
  v1 = (long)((dat_d120 - a1) * 10) * (long)((dat_d120 - a1) * 10);
  if (*(void **)((long)a0 + 0x20) != dat_18120) {
    a1 -= *(int *)((long)a0 + 0x14);
    v1 += ((long)(a1 * 10) * (long)(a1 * 10)) / 2;
  }
  return v1;
}

// Function: sub_411d @ 0x411d
void sub_411d(long a0)
{
  long v1; // stack - 0x10
  
  sub_417f((void *)0xe4e0,dat_18130);
  for (v1 = dat_e500; v1 != a0; v1 = *(long *)(v1 + 0x20)) {
    sub_417f(v1,dat_18134);
  }
}

// Function: sub_417f @ 0x417f
void sub_417f(void *a0,int a1)
{
  long v1;
  void *v2; // stack - 0x20
  
  dat_d128 = 0;
  sub_42aa(dat_1812c);
  fputs_unlocked(dat_d108,stdout);
  dat_d128 = dat_d11c + dat_d128;
  sub_42aa(a1 - dat_d128);
  v1 = *(long *)((long)a0 + 0x20);
  for (v2 = a0; v2 != (void *)(v1 + -0x28); v2 = (void *)((long)v2 + 0x28)) {
    sub_4248(v2);
    sub_42aa(*(unsigned int *)((long)v2 + 0xc));
  }
  sub_4248(v2);
  dat_18140 = dat_d128;
  putchar_unlocked(10);
}

// Function: sub_4248 @ 0x4248
void sub_4248(struct_1 *a0)
{
  char *v1;
  int v2; // stack - 0x14
  char *v3; // stack - 0x10
  
  v2 = a0->field_0x8;
  v3 = a0->field_0x0;
  while (v2) {
    v1 = &v3[1];
    putchar_unlocked((int)*v3);
    v2 -= 1;
    v3 = v1;
  }
  dat_d128 += a0->field_0x8;
}

// Function: sub_42aa @ 0x42aa
void sub_42aa(int a0)
{
  int v1;
  
  a0 += dat_d128;
  if (dat_18128) {
    v1 = a0;
    if (a0 <= -1)
      v1 = a0 + 7;
    v1 = (v1 >> 3) << 3;
    if (dat_d128 + 1 < v1) {
      for (; dat_d128 < v1; dat_d128 = ((dat_d128 >> 3) + 1) * 8) {
        putchar_unlocked(9);
        if (dat_d128 <= -1)
          dat_d128 += 7;
      }
    }
  }
  for (; dat_d128 < a0; dat_d128 = dat_d128 + 1) {
    putchar_unlocked(0x20);
  }
}

// Function: sub_4352 @ 0x4352
unsigned long sub_4352(int a0) // return-dupe
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

// Function: sub_4390 @ 0x4390
unsigned long sub_4390(int a0) // return-dupe
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

// Function: sub_43c2 @ 0x43c2
bool sub_43c2(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_43e3 @ 0x43e3
unsigned int sub_43e3(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_440b @ 0x440b
unsigned long sub_440b(int a0) // return-dupe
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

// Function: sub_4438 @ 0x4438
bool sub_4438(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_445c @ 0x445c
bool sub_445c(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_4480 @ 0x4480
bool sub_4480(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_44a4 @ 0x44a4
bool sub_44a4(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_44c8 @ 0x44c8
unsigned long sub_44c8(int a0) // return-dupe, return-dupe x3
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

// Function: sub_4512 @ 0x4512
unsigned long sub_4512(int a0) // return-dupe
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

// Function: sub_453f @ 0x453f
bool sub_453f(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_4563 @ 0x4563
unsigned long sub_4563(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_45ab @ 0x45ab
int sub_45ab(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_45ce @ 0x45ce
int sub_45ce(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_45f1 @ 0x45f1
void sub_45f1(unsigned long a0)
{
  dat_18148 = a0;
}

// Function: sub_460b @ 0x460b
void sub_460b(char a0)
{
  dat_18150 = a0;
}

// Function: sub_4625 @ 0x4625
void sub_4625(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_8102(stdout);
  if (v1) {
    if (dat_18150 == '\x01') {
      if (*__errno_location() == 0x20) goto label_46f6;
    }
    v2 = gettext("write error");
    if (dat_18148) { // branch-flip
      v3 = (char *)sub_66ca(dat_18148);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_d018); // no-return
  }
label_46f6:
  v1 = sub_8102(stderr);
  if (v1)
    _exit(dat_d018); // no-return
}

// Function: sub_471d @ 0x471d
void sub_471d(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  posix_fadvise(a0,a1,a2,a3);
}

// Function: sub_4752 @ 0x4752
void sub_4752(FILE *a0,unsigned int a1) // return-dupe
{
  if (!a0)
    return;
  sub_471d(fileno(a0),0,0,a1);
}

// Function: sub_4791 @ 0x4791
unsigned long sub_4791(FILE *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_481f;
  }
  if (sub_4882(a0))
    v2 = *__errno_location();
label_481f:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_484a @ 0x484a
void sub_484a(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_48d1(a0,0,1);
}

// Function: sub_4882 @ 0x4882
void sub_4882(FILE *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_484a(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_48d1 @ 0x48d1
unsigned long sub_48d1(FILE *a0,long a1,int a2) // early-return x2
{
  long v1; // rax
  
  if (((*(long *)&a0->field_0x10 == *(long *)&a0->field_0x8) && (*(long *)&a0->field_0x28 == *(long *)&a0->field_0x20)) && (!*(long *)&a0->field_0x48)) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 == -1)
      return 0xffffffff;
    *(unsigned int *)a0 = *(unsigned int *)a0 & 0xffffffef;
    *(long *)&a0->field_0x90 = v1;
    return 0;
  }
  return CONCAT44(dat_4,fseeko(a0,a1,a2));
}

// Function: sub_498f @ 0x498f
void sub_498f(char *a0)
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
  dat_18158 = v2;
  program_invocation_name = v2;
}

// Function: sub_4a7a @ 0x4a7a
unsigned long sub_4a7a(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x18160;
  v2 = sub_7888(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_4ac9 @ 0x4ac9
unsigned int sub_4ac9(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x18160;
  return *a0;
}

// Function: sub_4aed @ 0x4aed
void sub_4aed(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x18160;
  *a0 = a1;
}

// Function: sub_4b18 @ 0x4b18
unsigned int sub_4b18(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x18160;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_4ba8 @ 0x4ba8
unsigned int sub_4ba8(void *a0,unsigned int a1)
{
  unsigned int v1;
  void *v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = (void *)0x18160;
  v1 = *(unsigned int *)((long)v2 + 4);
  *(unsigned int *)((long)v2 + 4) = a1;
  return v1;
}

// Function: sub_4be2 @ 0x4be2
void sub_4be2(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x18160;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_4c44 @ 0x4c44
unsigned long * sub_4c44(unsigned long *a0,unsigned int a1)
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

// Function: sub_4d09 @ 0x4d09
char * sub_4d09(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_8278();
  if (!sub_8088(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_8088(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x985e;
  return (char *)0x9862;
}

// Function: sub_4dce @ 0x4dce
unsigned long sub_4dce(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_4e88:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_4fdc:
      v14 = 1;
      goto label_4fe0;
    case 2:
label_4fef:
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
      goto label_4fdc;
    case 4:
label_4fe0:
      if (!v14)
        v2 = 1;
      goto label_4fef;
    case 5:
label_4eca:
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
      goto label_4eca;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_4d09("`",v19);
        v17 = (char *)sub_4d09("\'",v19);
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
label_5c8e:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_5cc6;
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
      if (v14) goto label_5e0a;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_55ad:
    if (!v24) {
label_55b8:
      v16 = 1;
label_55bc:
      if (v19 != 2) goto label_5ad3;
label_5395:
      if (v14) goto label_5e0a;
    }
    goto label_5ad3;
  }
  if (0x7e < v22) {
label_5699:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_5713:
      v31 = sub_82c0(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_57c1;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_5870;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_5870:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_5ad3;
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
        if (v14) goto label_5e0a;
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
    goto label_5bf9;
  }
  if (v22 == 0x7d) {
label_557b:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_5ad3;
    goto label_55ad;
  }
  if (0x7d < v22) goto label_5699;
  if (v22 == 0x7c) goto label_55bc;
  if (0x7c < v22) goto label_5699;
  if (v22 == 0x7b) goto label_557b;
  if (0x7b < v22) goto label_5699;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_5699;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_5690;
    if (v11 & 0xa4000000) goto label_55bc;
    if (!(v11 & 0x8000000)) goto label_5699;
    if (v19 == 2) {
      if (!v14) goto label_5bf9;
      goto label_5e0a;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_5551;
    goto label_5bf9;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_5ad3;
        goto label_5c89;
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
        goto label_5ad3;
      }
      goto label_5e0a;
    default:
      goto label_5699;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_5551;
    case 10:
      v23 = 0x6e;
      goto label_5551;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_5551:
      if ((v19 == 2) && (v14)) goto label_5e0a;
      break;
    case 0x20:
      goto label_55b8;
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
      goto label_55bc;
    case 0x23:
      goto label_55ad;
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
label_5690:
      v16 = 1;
      goto label_5ad3;
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
          goto label_5ad3;
        }
        goto label_5e0a;
      }
      goto label_5ad3;
    case 0x3f:
      if (v19 == 2) goto label_5395;
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
          goto label_5ad3;
        }
        goto label_5e0a;
      }
      goto label_5ad3;
    
  }
  if (v2) {
    v22 = v23;
label_5b3e:
    if (v14) goto label_5e0a;
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
label_5ad3:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_5b3e;
  }
label_5bf9:
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
label_5c89:
  v24 += 1;
  goto label_5c8e;
label_57c1:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_5e0a;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_5870;
  goto label_5713;
label_5cc6:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_5e0a:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_4dce(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_4dce(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_4e88;
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

// Function: sub_5e8f @ 0x5e8f
unsigned long sub_5e8f(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x18160;
  v1 = *__errno_location();
  v2 = sub_4dce(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_5f37 @ 0x5f37
void sub_5f37(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_5f6d(a0,a1,NULL,a2);
}

// Function: sub_5f6d @ 0x5f6d
long sub_5f6d(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x18160;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_4dce(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_7394(v3 + 1U);
  sub_4dce(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_6099 @ 0x6099
void sub_6099(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_d090;
  for (v2 = 1; v2 < dat_d020; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x181a0) {
    free(*(void **)((long)v1 + 8));
    dat_d080 = 0x100;
    dat_d088 = 0x181a0;
  }
  if (v1 == (void *)0xd080) {
    dat_d020 = 1;
    return;
  }
  free(v1);
  dat_d020 = 1;
  dat_d090 = (void *)0xd080;
}

// Function: sub_6158 @ 0x6158
void * sub_6158(int a0,char *a1,unsigned long a2,struct_2 *a3)
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
  v6 = dat_d090;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_d020 <= a0) {
      v5 = dat_d090 == (unsigned long *)0xd080;
      v4 = (long)dat_d020;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_d090;
      }
      dat_d090 = (unsigned long *)sub_7624(v3,&v4,(long)((a0 - dat_d020) + 1),0x7fffffff,0x10);
      v6 = dat_d090;
      if (v5) {
        *dat_d090 = dat_d080;
        dat_d090[1] = dat_d088;
      }
      memset(&dat_d090[(long)dat_d020 * 2],0,(v4 - dat_d020) * 0x10);
      dat_d020 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_4dce(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x181a0)
        free(v7);
      v7 = (void *)sub_7394(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_4dce(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_6407 @ 0x6407
void sub_6407(unsigned int a0,char *a1)
{
  sub_6158(a0,a1,0xffffffffffffffff,(struct_2 *)0x18160);
}

// Function: sub_643b @ 0x643b
void sub_643b(unsigned int a0,char *a1,unsigned long a2)
{
  sub_6158(a0,a1,a2,(struct_2 *)0x18160);
}

// Function: sub_646d @ 0x646d
void sub_646d(char *a0)
{
  sub_6407(0,a0);
}

// Function: sub_6490 @ 0x6490
void sub_6490(char *a0,unsigned long a1)
{
  sub_643b(0,a0,a1);
}

// Function: sub_64bb @ 0x64bb
void sub_64bb(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_4c44(v1,a1);
  sub_6158(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_6523 @ 0x6523
void sub_6523(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_4c44(v1,a1);
  sub_6158(a0,a2,a3,v1);
}

// Function: sub_6589 @ 0x6589
void sub_6589(unsigned int a0,char *a1)
{
  sub_64bb(0,a0,a1);
}

// Function: sub_65b1 @ 0x65b1
void sub_65b1(unsigned int a0,char *a1,unsigned long a2)
{
  sub_6523(0,a0,a1,a2);
}

// Function: sub_65e1 @ 0x65e1
void sub_65e1(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_18160;
  v2 = dat_18168;
  v3 = dat_18170;
  v4 = dat_18178;
  v5 = dat_18180;
  v6 = dat_18188;
  v7 = dat_18190;
  sub_4b18(&v1,(int)a2,1);
  sub_6158(0,a0,a1,&v1);
}

// Function: sub_669c @ 0x669c
void sub_669c(char *a0,char a1)
{
  sub_65e1(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_66ca @ 0x66ca
void sub_66ca(char *a0)
{
  sub_669c(a0,0x3a);
}

// Function: sub_66ed @ 0x66ed
void sub_66ed(char *a0,unsigned long a1)
{
  sub_65e1(a0,a1,0x3a);
}

// Function: sub_671b @ 0x671b
void sub_671b(unsigned int a0,unsigned int a1,char *a2)
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
  sub_4c44(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_4b18(&v2,0x3a,1);
  sub_6158(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_67dd @ 0x67dd
void sub_67dd(unsigned int a0,long a1,long a2,char *a3)
{
  sub_6817(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_6817 @ 0x6817
void sub_6817(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_18160;
  v2 = dat_18168;
  v3 = dat_18170;
  v4 = dat_18178;
  v5 = dat_18180;
  v6 = dat_18188;
  v7 = dat_18190;
  sub_4be2(&v1,a1,a2);
  sub_6158(a0,a3,a4,&v1);
}

// Function: sub_68d5 @ 0x68d5
void sub_68d5(long a0,long a1,char *a2)
{
  sub_67dd(0,a0,a1,a2);
}

// Function: sub_6908 @ 0x6908
void sub_6908(long a0,long a1,char *a2,unsigned long a3)
{
  sub_6817(0,a0,a1,a2,a3);
}

// Function: sub_6946 @ 0x6946
void sub_6946(unsigned int a0,char *a1,unsigned long a2)
{
  sub_6158(a0,a1,a2,(struct_2 *)0xd040);
}

// Function: sub_6978 @ 0x6978
void sub_6978(char *a0,unsigned long a1)
{
  sub_6946(0,a0,a1);
}

// Function: sub_69a3 @ 0x69a3
void sub_69a3(unsigned int a0,char *a1)
{
  sub_6946(a0,a1,0xffffffffffffffff);
}

// Function: sub_69d0 @ 0x69d0
void sub_69d0(char *a0)
{
  sub_69a3(0,a0);
}

// Function: sub_69f3 @ 0x69f3
void sub_69f3(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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
  fprintf(a0,"Copyright %s %d Free Software Foundation, Inc.",gettext("(C)"),0x7e6);
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

// Function: sub_6fef @ 0x6fef
void sub_6fef(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_69f3(a0,a1,a2,a3,a4,v1);
}

// Function: sub_7062 @ 0x7062
void sub_7062(FILE *a0,char *a1,char *a2,char *a3,struct_3 *a4)
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
  sub_69f3(a0,a1,a2,a3,v2,v3);
}

// Function: sub_7168 @ 0x7168
void sub_7168(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_7062(a8,a9,a10,a11,&v3);
}

// Function: sub_724c @ 0x724c
void sub_724c(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-coreutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_72f5 @ 0x72f5
void sub_72f5(void *a0,unsigned long a1,unsigned long a2)
{
  sub_742e(a0,a1,a2);
}

// Function: sub_7326 @ 0x7326
long sub_7326(long a0)
{
  if (!a0)
    sub_7980(); // no-return
  return a0;
}

// Function: sub_7348 @ 0x7348
void sub_7348(unsigned long a0)
{
  sub_7326(malloc(a0));
}

// Function: sub_736e @ 0x736e
void sub_736e(unsigned long a0)
{
  sub_7326(sub_81ab(a0));
}

// Function: sub_7394 @ 0x7394
void sub_7394(unsigned long a0)
{
  sub_7348(a0);
}

// Function: sub_73b2 @ 0x73b2
void * sub_73b2(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_7980(); // no-return
  return v1;
}

// Function: sub_73fd @ 0x73fd
void sub_73fd(void *a0,unsigned long a1)
{
  sub_7326(sub_81c9(a0,a1));
}

// Function: sub_742e @ 0x742e
void * sub_742e(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_7980(); // no-return
  return v1;
}

// Function: sub_7488 @ 0x7488
void sub_7488(void *a0,unsigned long a1,unsigned long a2)
{
  sub_7326(sub_8229(a0,a1,a2));
}

// Function: sub_74c1 @ 0x74c1
void sub_74c1(unsigned long a0,unsigned long a1)
{
  sub_742e(NULL,a0,a1);
}

// Function: sub_74ec @ 0x74ec
void sub_74ec(unsigned long a0,unsigned long a1)
{
  sub_7488(NULL,a0,a1);
}

// Function: sub_7517 @ 0x7517
void sub_7517(void *a0,unsigned long *a1)
{
  sub_7545(a0,a1,1);
}

// Function: sub_7545 @ 0x7545
unsigned long sub_7545(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_7980(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_742e(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_7624 @ 0x7624
unsigned long sub_7624(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_7980(); // no-return
  v3 = sub_73b2(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_77e0 @ 0x77e0
void sub_77e0(unsigned long a0)
{
  sub_7826(a0,1);
}

// Function: sub_7803 @ 0x7803
void sub_7803(unsigned long a0)
{
  sub_7857(a0,1);
}

// Function: sub_7826 @ 0x7826
void sub_7826(unsigned long a0,unsigned long a1)
{
  sub_7326(calloc(a0,a1));
}

// Function: sub_7857 @ 0x7857
void sub_7857(unsigned long a0,unsigned long a1)
{
  sub_7326(sub_8200(a0,a1));
}

// Function: sub_7888 @ 0x7888
void sub_7888(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7348(a1),a0,a1);
}

// Function: sub_78c0 @ 0x78c0
void sub_78c0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_736e(a1),a0,a1);
}

// Function: sub_7900 @ 0x7900
void sub_7900(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_736e(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_794f @ 0x794f
void sub_794f(char *a0)
{
  sub_7888(a0,strlen(a0) + 1);
}

// Function: sub_7980 @ 0x7980
void sub_7980(void)
{
  error(dat_d018,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_79c0 @ 0x79c0
unsigned long sub_79c0(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,char *a4,char *a5,int a6)
{
  int v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x28
  int v4;
  int v5; // stack - 0x2c
  
  v5 = sub_7c2a(a0,NULL,a1,&v3,a4);
  if (v5) { // branch-flip
    if (v5 != 1) { // branch-flip
      if (v5 == 3)
        *__errno_location() = 0;
    }
    else {
      *__errno_location() = 0x4b;
    }
  }
  else if ((v3 < a2) || (a3 < v3)) {
    v5 = 1;
    if (0x40000000 <= v3) // branch-flip
      *__errno_location() = 0x4b;
    else {
      *__errno_location() = 0x22;
    }
  }
  if (v5) {
    v2 = (char *)sub_69d0(a0);
    if (*__errno_location() != 0x16) // branch-flip
      v4 = *__errno_location();
    else {
      v4 = 0;
    }
    if (a6) // branch-flip
      v1 = a6;
    else {
      v1 = 1;
    }
    error(v1,v4,"%s: %s",a5,v2);
  }
  return v3;
}

// Function: sub_7afa @ 0x7afa
void sub_7afa(unsigned char *a0,unsigned long a1,unsigned long a2,char *a3,char *a4,unsigned int a5)
{
  sub_79c0(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_7b53 @ 0x7b53
bool sub_7b53(unsigned long *a0,int a1)
{
  char v1 [16];
  bool v2;
  
  v2 = 0;
  if (((long)a1 <= -1) && (*a0))
    v2 = 1;
  v1 = ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0);
  if (SUB168(v1,8))
    v2 = 1;
  if (!v2)
    *a0 = SUB168(v1,0);
  else {
    *a0 = 0xffffffffffffffff;
  }
  return v2;
}

// Function: sub_7be5 @ 0x7be5
unsigned int sub_7be5(unsigned long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_7b53(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_7c2a @ 0x7c2a
unsigned int sub_7c2a(unsigned char *a0,long *a1,int a2,unsigned long *a3,char *a4) // early-return x2
{
  char v1;
  long *v10; // stack - 0x18
  long v2;
  unsigned long v3; // stack - 0x28
  unsigned char v4; // stack - 0x41
  unsigned int v5; // stack - 0x40
  unsigned int v6; // stack - 0x3c
  int v7; // stack - 0x38
  unsigned int v8; // stack - 0x34
  unsigned char *v9; // stack - 0x20
  
  v5 = 0;
  if ((a2 < 0) || (0x25 <= a2))
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v2;
  v10 = a1;
  *__errno_location() = 0;
  v4 = *a0;
  v9 = a0;
  while (*(unsigned short *)((unsigned long)v4 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
    v9 = &v9[1];
    v4 = *v9;
  }
  if (v4 == 0x2d)
    return 4;
  v3 = strtoumax(a0,v10,a2);
  if (a0 != (unsigned char *)*v10) { // branch-flip
    if (*__errno_location()) {
      if (*__errno_location() != 0x22)
        return 4;
      v5 = 1;
    }
  }
  else {
    if (((!a4) || (!*(char *)*v10)) || (v1 = *(char *)*v10, !strchr(a4,(int)v1)))
      return 4;
    v3 = 1;
  }
  if (!a4) {
    *a3 = v3;
    return v5;
  }
  if (*(char *)*v10) {
    v6 = 0x400;
    v7 = 1;
    v1 = *(char *)*v10;
    if (!strchr(a4,(int)v1)) {
      *a3 = v3;
      return v5 | 2;
    }
    if ((((unsigned int)((int)*(char *)*v10 - 0x45U) <= 0x2f) && (0x814400308945U >> ((unsigned char)((int)*(char *)*v10 - 0x45U) & 0x3f) & 1)) && (strchr(a4,0x30))) {
      v1 = *(char *)(*v10 + 1);
      if (v1 != 'i') { // branch-flip
        if ((v1 <= 'i') && ((v1 == 'B' || (v1 == 'D')))) {
          v6 = 1000;
          v7 = 2;
        }
      }
      else if (*(char *)(*v10 + 2) == 'B')
        v7 = 3;
    }
    switch(*(char *)*v10) {
      case 0x42:
        v8 = sub_7b53(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_7be5(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_7be5(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_7be5(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_7be5(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_7be5(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_7be5(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_7be5(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_7be5(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_7b53(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_7b53(&v3,2);
      
    }
    v5 |= v8;
    *v10 = *v10 + (long)v7;
    if (*(char *)*v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_8088 @ 0x8088
int sub_8088(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_45ab((unsigned char)*a0);
    v2 = sub_45ab((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_8102 @ 0x8102
unsigned long sub_8102(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_4791(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_8191 @ 0x8191
unsigned long sub_8191(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_81ab @ 0x81ab
void sub_81ab(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_81c9 @ 0x81c9
void sub_81c9(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_8200 @ 0x8200
void sub_8200(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_8229 @ 0x8229
void sub_8229(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_8278 @ 0x8278
char * sub_8278(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_82c0 @ 0x82c0
unsigned long sub_82c0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_836a(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_836a @ 0x836a
unsigned int sub_836a(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_84f4(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_8414 @ 0x8414
char * sub_8414(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_843c @ 0x843c
unsigned long sub_843c(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_8414(a0);
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

// Function: sub_84f4 @ 0x84f4
void sub_84f4(unsigned int a0,char *a1,unsigned long a2)
{
  sub_843c(a0,a1,a2);
}

// Function: sub_8522 @ 0x8522
void sub_8522(unsigned int a0)
{
  sub_8414(a0);
}

// Function: sub_8540 @ 0x8540
void sub_8540(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_d008); // tail-call
}

// Function: _DT_FINI @ 0x8554
void _DT_FINI(void)
{
  return;
}

