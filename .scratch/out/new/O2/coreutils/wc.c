// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_cfd8)
    return;
  (*dat_cfd8)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_cdb8)(); // jump-as-call
}

// Function: free @ 0x2430
void free(void *a0)
{
  (*dat_cfc0)(); // jump-as-call
}

// Function: malloc @ 0x2440
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cfe0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x2450
void __cxa_finalize(void)
{
  (*dat_cff0)(); // jump-as-call
}

// Function: getenv @ 0x2460
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cdc0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x2470
void abort(void)
{
  (*dat_cdc8)(); // jump-as-call
}

// Function: __errno_location @ 0x2480
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_cdd0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2490
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_cdd8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x24a0
void _exit(int a0)
{
  (*dat_cde0)(); // jump-as-call
}

// Function: __fpending @ 0x24b0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_cde8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x24c0
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cdf0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x24d0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cdf8)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x24e0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ce00)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x24f0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce08)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2500
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ce10)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2510
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce18)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x2520
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce20)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2530
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce28)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2540
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce30)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2550
void __stack_chk_fail(void)
{
  (*dat_ce38)(); // jump-as-call
}

// Function: getopt_long @ 0x2560
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ce40)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2570
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce48)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2580
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce50)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x2590
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_ce58)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x25a0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce60)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x25b0
void lseek(void)
{
  (*dat_ce68)(); // jump-as-call
}

// Function: __assert_fail @ 0x25c0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_ce70)(); // jump-as-call
}

// Function: fputs @ 0x25d0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_ce78)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x25e0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ce80)(); // jump-as-call
  return v1;
}

// Function: fgetc @ 0x25f0
int fgetc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ce88)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2600
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ce90)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x2610
void posix_fadvise(void)
{
  (*dat_ce98)(); // jump-as-call
}

// Function: read @ 0x2620
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_cea0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x2630
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_cea8)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2640
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_ceb0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x2650
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ceb8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2660
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cec0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2670
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_cec8)(); // jump-as-call
  return v1;
}

// Function: __memmove_chk @ 0x2680
void * __memmove_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ced0)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x2690
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ced8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x26a0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_cee0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x26b0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cee8)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x26c0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cef0)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x26d0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cef8)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x26e0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf00)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x26f0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf08)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x2700
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf10)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2710
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf18)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2720
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf20)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x2730
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf28)(); // jump-as-call
  return v1;
}

// Function: setvbuf @ 0x2740
int setvbuf(FILE *a0,char *a1,int a2,unsigned long a3)
{
  int v1; // eax
  
  v1 = (*dat_cf30)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x2750
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf38)(); // jump-as-call
  return v1;
}

// Function: btowc @ 0x2760
unsigned int btowc(int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_cf40)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2770
void error(int a0,int a1,char *a2,...)
{
  (*dat_cf48)(); // jump-as-call
}

// Function: open @ 0x2780
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf50)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x2790
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_cf58)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x27a0
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_cf60)(); // jump-as-call
  return v1;
}

// Function: sysconf @ 0x27b0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_cf68)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x27c0
void __cxa_atexit(void)
{
  (*dat_cf70)(); // jump-as-call
}

// Function: sysinfo @ 0x27d0
int sysinfo(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf78)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x27e0
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_cf80)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x27f0
void exit(int a0)
{
  (*dat_cf88)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2800
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_cf90)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2810
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf98)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2820
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cfa0)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x2830
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_cfa8)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x2840
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_cfb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2850
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cfb8)(); // jump-as-call
  return v1;
}

// Function: main @ 0x2860
int main(int argc,char **argv,char **envp)
{
  bool v1;
  char *v10;
  FILE *v11;
  char *v12; // rax
  unsigned long v13;
  unsigned long v14; // rax
  int *v15;
  unsigned long v16; // rcx
  long v17;
  unsigned long v18;
  stat v19; // stack - 0xd8
  unsigned char v2; // al
  int v20; // stack - 0x1fc
  unsigned long v21; // stack - 0x1f8
  long v22; // fs_offset
  double v23;
  long v24; // stack - 0x230
  int *v25; // stack - 0x220
  unsigned char v26; // stack - 0x218
  bool v27; // stack - 0x20d
  char **v28; // stack - 0x1f0
  long v29; // stack - 0x40
  unsigned char v3;
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  char *v8;
  struct_3 *v9; // rax
  
  v29 = *(long *)(v22 + 0x28);
  sub_4d10(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_88e0(sub_46f0);
  dat_d120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  v8 = getenv("POSIXLY_CORRECT");
  dat_d131 = 0;
  dat_d132 = 0;
  dat_d118 = v8 != NULL;
  dat_d133 = 0;
  dat_d134 = 0;
  dat_d130 = 0;
  dat_d138 = 0;
  dat_d140 = 0;
  dat_d148 = 0;
  dat_d150 = 0;
  dat_d158 = 0;
  v8 = NULL;
  while( true ) {
    v14 = 0;
    v4 = getopt_long(argc,argv,"clLmw",(option *)0xca20,NULL);
    if (v4 == -1) break;
    if (0x81 < v4) {
      sub_4230(1); // return-dupe, no-return
    }
    if (v4 <= 0x4b) {
      if (v4 == -0x83) {
        sub_7a40(stdout,"wc","GNU coreutils",dat_d018,"Paul Rubin","David MacKenzie",0,v14);
        exit(0); // no-return
      }
      if (v4 == -0x82)
        sub_4230(0); // no-return
      sub_4230(1);
    }
    switch(v4) {
      case 0x4c:
        dat_d130 = 1;
        break;
      default:
        goto label_3299;
      case 99:
        dat_d131 = 1;
        break;
      case 0x6c:
        dat_d134 = 1;
        break;
      case 0x6d:
        dat_d132 = 1;
        break;
      case 0x77:
        dat_d133 = 1;
        break;
      case 0x80:
        dat_d160 = 1;
        break;
      case 0x81:
        v8 = optarg;
      
    }
  }
  if ((((!dat_d134) && (!dat_d133)) && (!dat_d132)) && ((!dat_d131 && (!dat_d130)))) {
    dat_d131 = 1;
    dat_d133 = 1;
    dat_d134 = 1;
  }
  if (v8) { // branch-flip
    if (optind < argc) {
      v8 = (char *)sub_69f0(4,argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v8);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
label_3299:
      sub_4230(1);
    }
    if (strcmp(v8,"-")) { // branch-flip
      v11 = fopen(v8,"r");
      if (!v11) {
        v14 = sub_69f0(4,v8);
        v8 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v8,v14);
        return v5;
      }
    }
    else {
      v11 = stdin;
    }
    v4 = fileno(v11);
    if ((fstat(v4,&v19)) || ((v19._24_4_ & 0xf000) != 0x8000)) {
label_2ad5:
      v9 = (struct_3 *)sub_45b0(v11);
      if (!v9) {
        sub_8110(); // no-return, return-dupe
      }
      v25 = (int *)sub_7d50(1,0x98);
      v27 = 0;
    }
    else {
      v23 = dat_99b8;
      if ((double)sub_4bd0() * dat_99c0 <= dat_99b8)
        v23 = dat_99c0 * (double)sub_4bd0();
      if (v23 < (double)(long)v19._48_8_) goto label_2ad5;
      sub_71f0(&v21);
      v27 = sub_72a0(v11,&v21);
      if ((!v27) || (sub_47e0(v11))) {
        v8 = (char *)sub_69f0(4,v8);
        error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v8);
        return v7;
      }
      v9 = (struct_3 *)sub_4580(v28);
      if (!v9) {
        sub_8110();
      }
      v18 = 1;
      if (v21)
        v18 = v21;
      v25 = (int *)sub_7d50(v18,0x98);
      v18 = v21;
      argv = v28;
      if (v21) goto label_3024;
    }
    v18 = 0;
    *v25 = 1;
    dat_d12c = 1;
  }
  else {
    if (argc <= optind) { // branch-flip
      v18 = 1;
      argv = (char **)0xd110;
    }
    else {
      argv = &argv[optind];
      v18 = (unsigned long)(argc - optind);
    }
    v9 = (struct_3 *)sub_4580(argv);
    if (!v9) {
      sub_8110();
    }
    v25 = (int *)sub_7d50(v18,0x98);
    v27 = 0;
label_3024:
    if ((v18 != 1) || ((unsigned int)dat_d134 + (unsigned int)dat_d133 + (unsigned int)dat_d132 + (unsigned int)dat_d131 + (unsigned int)dat_d130 != 1)) {
      v13 = 0;
      v15 = &v25[2];
      do {
        v10 = argv[v13];
        if ((v10) && (strcmp(v10,"-")))
          v4 = stat(v10,(stat *)v15);
        else {
          v4 = fstat(0,(stat *)v15);
        }
        v13 += 1;
        v15[-2] = v4;
        v15 = &v15[0x26];
      } while (v13 < v18);
      dat_d12c = 1;
      if (*v25 <= 0) {
        v4 = 1;
        v13 = 0;
        v16 = 0;
        v15 = v25;
        do {
          if (!*v15) {
            if ((v15[8] & 0xf000U) != 0x8000) // branch-flip
              v4 = 7;
            else {
              v13 += *(long *)&v15[0xe];
            }
          }
          v16 += 1;
          v15 = &v15[0x26];
        } while (v16 < v18);
        dat_d12c = 1;
        if (10 <= v13) {
          do {
            dat_d12c += 1;
            v16 = v13 / 10;
            v1 = 100 <= v13;
            v13 = v16;
          } while (v1);
        }
        if (dat_d12c <= v4)
          dat_d12c = v4;
      }
    }
    else {
      *v25 = 1;
      dat_d12c = 1;
    }
  }
  v24 = 0;
  v3 = 1;
  while (v10 = (char *)sub_45f0(v9,&v20), v10) {
    if (v8) { // branch-flip
      if (strcmp(v8,"-")) { // branch-flip
        if (*v10) goto label_2c10;
      }
      else if (strcmp(v10,"-")) { // branch-flip
        if (*v10) {
          if (v18) {
            v17 = v24;
            goto label_2c1b;
          }
          v15 = v25;
          goto label_2b6c;
        }
      }
      else {
        v12 = (char *)sub_69f0(4,v10);
        error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v12);
        if (*v10) goto label_2de1;
      }
      v13 = sub_4690(v9);
      v10 = dcgettext(NULL,"invalid zero-length file name",5);
      error(0,0,"%s:%lu: %s",(char *)sub_6d70(0,3,v8),v13,v10);
      v3 = 0;
    }
    else if (*v10) { // branch-flip
label_2c10:
      v17 = 0;
      if (v18)
        v17 = v24;
label_2c1b:
      v15 = (int *)(v17 + (long)v25);
      if (strcmp(v10,"-")) { // branch-flip
label_2b6c:
        v4 = open(v10,0);
        if (v4 != -1) {
          v26 = sub_3820(v4,v10,v15,0);
          v4 = close(v4);
          if (!v4) goto label_2bb5;
        }
        v10 = (char *)sub_6d70(0,3,v10);
        error(0,*__errno_location(),"%s",v10);
        v26 = 0;
      }
      else {
        dat_d128 = '\x01';
        v26 = sub_3820(0,v10,(int *)(v17 + (long)v25),-1);
      }
label_2bb5:
      v3 &= v26;
    }
    else {
      error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
label_2de1:
      v3 = 0;
    }
    if (!v18)
      *v25 = 1;
    v24 += 0x98;
  }
  if (v20 != 3) {
    if (v20 != 4) { // branch-flip
      if (v20 != 2)
        __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
      v2 = v8 == NULL & v3;
      if (v2) {
        v3 = v2;
        if (!sub_4690(v9)) {
          dat_d128 = '\x01';
          v3 = sub_3820(0,NULL,v25,-1);
        }
      }
    }
    else {
      v14 = sub_6d70(0,3,v8);
      v8 = dcgettext(NULL,"%s: read error",5);
      error(0,*__errno_location(),v8,v14);
      v3 = 0;
    }
    if (v27)
      sub_7270(&v21);
    if (2 <= (unsigned long)sub_4690(v9))
      sub_3430(dat_d158,dat_d150,dat_d148,dat_d140,dat_d138,dcgettext(NULL,"total",5));
    sub_46b0(v9);
    free(v25);
    if ((dat_d128) && (close(0))) {
      error(1,*__errno_location(),"-");
      return v6;
    }
    if (v29 != *(long *)(v22 + 0x28))
      __stack_chk_fail(); // no-return
    return (int)(v3 ^ 1);
  }
  sub_8110();
}

// Function: sub_3340 @ 0x3340
void sub_3340(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_cfc8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_3370 @ 0x3370
void sub_3370(void)
{
  return;
}

// Function: _FINI_0 @ 0x33e0
void _FINI_0(void)
{
  if (!dat_d108) {
    if (dat_cff0)
      __cxa_finalize(dat_d008);
    sub_3370();
    dat_d108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x3420
void _INIT_0(void)
{
  return;
}

// Function: sub_3430 @ 0x3430
void sub_3430(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5) // return-dupe
{
  char *v1;
  unsigned long v2; // rax
  char v3 [24];
  char *v4;
  char *v5;
  
  v4 = "%*s";
  if (dat_d134) {
    v2 = sub_4940(a0,v3);
    v4 = " %*s";
    __printf_chk(1,"%*s",(unsigned long)dat_d12c,v2);
  }
  v5 = v4;
  if (dat_d133) {
    v2 = sub_4940(a1,v3);
    v5 = " %*s";
    __printf_chk(1,v4,(unsigned long)dat_d12c,v2);
  }
  v4 = v5;
  if (dat_d132) {
    v2 = sub_4940(a2,v3);
    v4 = " %*s";
    __printf_chk(1,v5,(unsigned long)dat_d12c,v2);
  }
  v5 = v4;
  if (dat_d131) {
    v2 = sub_4940(a3,v3);
    v5 = " %*s";
    __printf_chk(1,v4,(unsigned long)dat_d12c,v2);
  }
  if (dat_d130) {
    v2 = sub_4940(a4,v3);
    __printf_chk(1,v5,(unsigned long)dat_d12c,v2);
  }
  if (a5) {
    if (strchr(a5,10))
      a5 = (char *)sub_6d70(0,3,a5);
    __printf_chk(1," %s",a5);
  }
  v1 = *(char **)&stdout->field_0x28;
  if (v1 < *(char **)&stdout->field_0x30) {
    *(char **)&stdout->field_0x28 = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10);
}

// Function: sub_3640 @ 0x3640
unsigned long sub_3640(char *a0,unsigned int a1,long *a2,long *a3)
{
  bool v1;
  unsigned long v2; // rax
  char *v3; // rax
  char *v4;
  char *v5;
  unsigned long v6;
  char v7 [16392];
  unsigned long v8;
  long v9; // stack - 0x4068
  
  v1 = 0;
  v9 = 0;
  v8 = 0;
  while( true ) {
    v2 = sub_7490(a1,v7,0x4000);
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
  v5 = (char *)sub_6d70(0,3,a0);
  error(0,*__errno_location(),"%s",v5);
  return 0;
}

// Function: sub_3800 @ 0x3800
unsigned long sub_3800(char *a0,unsigned int a1,long *a2,long *a3)
{
  if ((a2) && (a3))
    return sub_3640(a0,a1,a2,a3); // tail-call
  return 0;
}

// Function: sub_3820 @ 0x3820
unsigned int sub_3820(int a0,char *a1,struct_1 *a2,long a3) // ternary
{
  unsigned short v1;
  mbstate_t v10; // stack - 0x4050
  unsigned int v11; // stack - 0x4064
  unsigned long v12; // stack - 0x4058
  long v13; // stack - 0x4060
  unsigned long v14;
  char *v15;
  unsigned char v16;
  long v17;
  unsigned long v18;
  unsigned char *v19;
  mbstate_t v2;
  unsigned int v20;
  unsigned char *v21;
  unsigned int v22;
  bool v23; // sil
  long v24; // stack - 0x40a0
  unsigned long v25; // stack - 0x4090
  unsigned char v26; // stack - 0x4079
  char *v27; // stack - 0x4070
  unsigned char v3; // al
  unsigned int v4; // eax
  void *v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8; // rbx
  unsigned char v9 [16392];
  
  v27 = a1;
  if (!a1) {
    v5 = (char *)dcgettext(NULL,"standard input",5);
    v27 = (char *)v5;
  }
  v12 = 0;
  v13 = 0;
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  v16 = dat_d131; // branch-flip
  if (2 <= v5) {
    v3 = dat_d132;
    v26 = v3;
  }
  else {
    v26 = 0;
    if (!dat_d131)
      v16 = dat_d132;
  }
  if ((dat_d133) || (dat_d130))
    sub_47a0(a0,0,0,2);
  else {
    if ((v16 == 1) && (!v26)) {
      if (dat_d134) {
        sub_47a0(a0,0,0,2);
label_40e0:
        if (dat_d134) {
label_40f0:
          if (*(int *)cpuid_basic_info(0)) { // branch-flip
            if (!(*(unsigned int *)(cpuid_Version_info(1) + 0xc) & 0x8000000)) {
label_4120:
              v15 = "avx2 support not detected";
              if (dat_d160) goto label_417a;
              goto label_4135;
            }
            if (*(unsigned int *)cpuid_basic_info(0) <= 6) goto label_4165;
            if (!(*(unsigned int *)(cpuid_Extended_Feature_Enumeration_info(7) + 4) & 0x20)) goto label_4120;
            if (dat_d160) {
              v5 = (char *)dcgettext(NULL,"using avx2 hardware support",5);
              error(0,0,"%s",(char *)v5);
            }
            v5 = sub_86a0;
            dat_d010 = sub_86a0;
          }
          else {
label_4165:
            if (dat_d160) {
              v15 = "failed to get cpuid";
label_417a:
              v5 = (char *)dcgettext(NULL,v15,5);
              error(0,0,"%s",(char *)v5);
            }
label_4135:
            v5 = dat_d010;
          }
          v14 = 0;
          v18 = 0;
          v8 = 0;
          v4 = (*v5)(v27,a0,&v13,&v12);
          v7 = v12;
          v22 = v4;
          goto label_39f8;
        }
      }
      v4 = a2->field_0x0;
      if (0 < (int)v4) {
        v4 = fstat(a0,(stat *)&a2->field_0x4[4]);
        a2->field_0x0 = v4;
      }
      v18 = dat_d120;
      if (((!v4) && (v4 = a2->field_0x20 & 0xd000, v4 == 0x8000)) && (v7 = a2->field_0x38, 0 <= (long)v7)) {
        if (a3 != -1) { // branch-flip
          if (v7 % dat_d120) {
label_40b7:
            v14 = 0;
            v18 = 0;
            v8 = 0;
            v12 = v7;
            v22 = 1;
            goto label_39f8;
          }
          v23 = 1;
          v18 = 0;
        }
        else {
          v5 = (unsigned long)lseek(a0,0,1);
          if (v7 % v18) {
            v7 = (v5 <= v7) ? v7 - (long)v5 : 0; // branch-flip
            goto label_40b7;
          }
          v23 = (bool)((unsigned char)~(unsigned char)((unsigned long)v5 >> 0x38) >> 7);
          v18 = (unsigned long)v5;
        }
        v5 = (unsigned long)(a2->field_0x40 + 1);
        v14 = 0x201;
        if ((unsigned long)(a2->field_0x40 - 1U) <= 0x1fffffffffffffff)
          v14 = (unsigned long)v5;
        v17 = v7 - v7 % v14;
        if ((((long)v18 < v17) && (v23)) && (v5 = (long)lseek(a0,v17,1), 0 <= (long)v5))
          v12 = v17 - v18;
      }
      sub_47a0(a0,0,0,2);
      while (v5 = (long)sub_7490(a0,v9,0x4000), v5) {
        if (v5 == (void *)0xffffffffffffffff) {
          v14 = 0;
          v8 = 0;
          v5 = (char *)sub_6d70(0,3,v27);
          v15 = (char *)v5;
          v5 = (int *)__errno_location();
          v18 = 0;
          error(0,*(int *)v5,"%s",v15);
          v7 = v12;
          v22 = 0;
          goto label_39f8;
        }
        v12 += (long)v5;
      }
      v14 = 0;
      v8 = 0;
      v18 = 0;
      v7 = v12;
      v22 = 1;
      goto label_39f8;
    }
    sub_47a0(a0,0,0,2);
    if (v26 != 1) {
      if (v16) goto label_40e0;
      if (!v26) goto label_40f0;
    }
  }
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v5) { // branch-flip
    v7 = 0;
    v23 = 0;
    v14 = 0;
    v10 = 0;
    v8 = 0;
    v18 = 0;
    v25 = 0;
    v24 = 0;
label_3ae0:
    v5 = (long)sub_7490(a0,&v9[v7],0x4000 - v7);
    if (v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v5 = (char *)sub_6d70(0,3,v27);
        v15 = (char *)v5;
        v5 = (int *)__errno_location();
        v22 = 0;
        error(0,*(int *)v5,"%s",v15);
        goto label_3f68;
      }
      v12 += (long)v5;
      v7 += (long)v5;
      v19 = v9;
      do {
        v2 = v10;
        if (v23) {
label_3c30:
          v5 = (long)sub_4a50(&v11,v19,v7,&v10);
          v17 = (long)v5;
          if (v5 != (void *)0xfffffffffffffffe) {
            if (v5 != (void *)0xffffffffffffffff) {
              v4 = mbsinit(&v10);
              v23 = v4 == 0;
              if (v17) { // branch-flip
                switch(v11) {
                  case 9:
                    goto label_3c26;
                  case 10:
                    goto label_3c16;
                  case 0xb:
                    goto label_3be9;
                  case 0xc:
                  case 0xd:
                    goto label_3bd6;
                  default:
                    v20 = v11;
                    break;
                  case 0x20:
                    goto label_3c06;
                  
                }
              }
              else {
                v11 = 0;
                v17 = 1;
                v20 = 0;
              }
              v4 = iswprint(v20);
              if (v4) {
                if (dat_d130) {
                  v4 = wcwidth(v11);
                  v6 = (long)(int)v4 + v14;
                  if (1 <= (int)v4)
                    v14 = v6;
                }
                v4 = iswspace(v11);
                if (v4) goto label_3be9;
                if (dat_d118) // branch-flip
                  v8 = (unsigned long)dat_d118;
                else {
                  if ((((v11 == 0xa0) || (v11 == 0x2007)) || (v11 == 0x202f)) || (v11 == 0x2060)) goto label_3be9;
                  v8 = 1;
                }
              }
              goto label_3ba8;
            }
            v19 = &v19[1];
            v17 = -1;
            v23 = 1;
            goto label_3bb2;
          }
          v10 = v2;
          if (v7) {
            if (v7 == 0x4000) {
              v19 = &v19[1];
              v7 = 0x3fff;
            }
            __memmove_chk(v9,v19,v7,0x4001);
          }
          v23 = 1;
          break;
        }
        v16 = *v19;
        v3 = v16 >> 5;
        v5 = (unsigned long)(unsigned long)v3;
        v4 = *(unsigned int *)((long)v5 * 4 + 0x99e0) >> (v16 & 0x1f);
        v20 = v4 & 1;
        if (!v20) goto label_3c30;
        v4 = (unsigned int)(char)v16;
        v11 = v4;
        switch(v16) {
          case 9:
            v17 = 1;
label_3c26:
            v14 = (v14 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v17 = 1;
label_3c16:
            v13 += 1;
            goto label_3bd6;
          case 0xb:
label_3bf8:
            v17 = 1;
            break;
          case 0xc:
          case 0xd:
            v17 = 1;
label_3bd6:
            v6 = v14;
            if (v14 <= v25)
              v6 = v25;
            v14 = 0;
            v25 = v6;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v16 * 2);
            if (v1 & 0x4000) { // branch-flip
              v14 += 1;
              if (v1 & 0x2000) goto label_3bf8;
              v8 = (unsigned long)v20;
              v17 = 1;
            }
            else {
              v17 = 1;
            }
            goto label_3ba8;
          case 0x20:
            v17 = 1;
label_3c06:
            v14 += 1;
          
        }
label_3be9:
        v24 += v8;
        v8 = 0;
label_3ba8:
        v19 = &v19[v17];
        v17 = -v17;
        v18 += 1;
label_3bb2:
        v7 += v17;
      } while (v7);
      goto label_3ae0;
    }
    v22 = 1;
label_3f68:
    if (v14 <= v25)
      v14 = v25;
    v8 += v24;
    v7 = v12;
  }
  else {
    v14 = 0;
    v7 = 0;
    v8 = 0;
    v18 = v14;
    while (v5 = (long)sub_7490(a0,v9,0x4000), v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v22 = 0;
        v5 = (char *)sub_6d70(0,3,v27);
        v15 = (char *)v5;
        v5 = (int *)__errno_location();
        error(0,*(int *)v5,"%s",v15);
        goto label_3fc8;
      }
      v12 += (long)v5;
      v19 = &v9[(long)v5];
      v21 = v9;
      do {
        v16 = *v21;
        v21 = &v21[1];
        switch(v16) {
          case 9:
            v7 = (v7 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v13 += 1;
            goto label_3ee6;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_3ee6:
            if (v14 < v7)
              v14 = v7;
            v7 = 0;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v16 * 2);
            if (v1 & 0x4000) {
              v7 += 1;
              if (v1 & 0x2000) goto label_3ef0;
              v4 = btowc((unsigned int)v16);
              if (dat_d118) // branch-flip
                v8 = (unsigned long)dat_d118;
              else {
                if (((v4 == 0xa0) || (v4 == 0x2007)) || ((v4 == 0x202f || (v4 == 0x2060)))) goto label_3ef0;
                v8 = 1;
              }
            }
            goto label_3ed0;
          case 0x20:
            v7 += 1;
          
        }
label_3ef0:
        v18 += v8;
        v8 = 0;
label_3ed0:
      } while (v19 != v21);
    }
    v22 = 1;
label_3fc8:
    if (v14 < v7)
      v14 = v7;
    v8 += v18;
    v18 = 0;
    v7 = v12;
  }
label_39f8:
  if (v26 < dat_d132)
    v18 = v7;
  sub_3430(v13,v8,v18,v7,v14,a1);
  dat_d150 += v8;
  dat_d158 += v13;
  dat_d148 += v18;
  dat_d140 += v12;
  if (dat_d138 < v14)
    dat_d138 = v14;
  v4 = v22;
  return v4;
}

// Function: sub_4230 @ 0x4230
void sub_4230(int a0)
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
  
  v6 = dat_d178;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v6,v6);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
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
      if (!strcmp("wc",v6)) break;
      v6 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v6);
    v6 = *(char **)((long)v4 + 8);
    if (!v6)
      v6 = "wc";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "wc";
    if (!strcmp("wc","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "wc")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}

// Function: sub_4580 @ 0x4580
void sub_4580(char **a0) // return-dupe
{
  struct_11 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
}

// Function: sub_45b0 @ 0x45b0
void sub_45b0(unsigned long a0) // return-dupe
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

// Function: sub_45f0 @ 0x45f0
long sub_45f0(struct_3 *a0,int *a1) // early-return x2
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

// Function: sub_4690 @ 0x4690
long sub_4690(struct_4 *a0)
{
  if (a0->field_0x0)
    return a0->field_0x8;
  return a0->field_0x28 - a0->field_0x20 >> 3;
}

// Function: sub_46b0 @ 0x46b0
void sub_46b0(struct_5 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_46d0 @ 0x46d0
void sub_46d0(unsigned long a0)
{
  dat_d170 = a0;
}

// Function: sub_46e0 @ 0x46e0
void sub_46e0(char a0)
{
  dat_d168 = a0;
}

// Function: sub_46f0 @ 0x46f0
void sub_46f0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_8150(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_d168) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_d170) { // branch-flip
        v4 = (char *)sub_6c50(dat_d170);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_d020); // return-dupe, no-return
    }
  }
  v1 = sub_8150(stderr);
  if (!v1)
    return;
  _exit(dat_d020);
}

// Function: sub_47a0 @ 0x47a0
void sub_47a0(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_47b0 @ 0x47b0
void sub_47b0(FILE *a0,unsigned int a1)
{
  if (a0) {
    posix_fadvise(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_47e0 @ 0x47e0
unsigned long sub_47e0(FILE *a0) // early-return
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
    if (sub_4870(a0)) {
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

// Function: sub_4870 @ 0x4870
void sub_4870(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_48c0(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_48c0 @ 0x48c0
int sub_48c0(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_4940 @ 0x4940
char * sub_4940(unsigned long a0,void *a1)
{
  bool v1;
  char *v2;
  
  *(char *)((long)a1 + 0x14) = 0;
  v2 = (char *)((long)a1 + 0x14);
  do {
    v2 = &v2[-1];
    *v2 = (char)a0 + (char)(a0 / 10) * '\xf6' + '0';
    v1 = 10 <= a0;
    a0 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_49a0 @ 0x49a0
int sub_49a0(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_49d0 @ 0x49d0
void sub_49d0(struct_8 *a0,struct_9 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_9 *v3;
  
  v3 = (struct_9 *)a1->field_0x0;
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

// Function: sub_4a30 @ 0x4a30
unsigned int sub_4a30(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x99e0) >> (a0 & 0x1f) & 1;
}

// Function: sub_4a50 @ 0x4a50
unsigned long sub_4a50(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_81c0(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_4ae0 @ 0x4ae0
double sub_4ae0(void)
{
  long v1; // rax
  char v2 [32];
  double v3;
  unsigned long v4; // stack - 0x68
  unsigned int v5; // stack - 0x20
  
  v3 = (double)sysconf(0x55);
  v1 = sysconf(0x1e);
  if ((v3 < 0.0) || ((double)v1 < 0.0)) {
    v3 = dat_9a00;
    if (!sysinfo(v2))
      v3 = (double)v4 * (double)v5;
  }
  else {
    v3 = v3 * (double)v1;
  }
  return v3;
}

// Function: sub_4bd0 @ 0x4bd0
double sub_4bd0(void)
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
  sub_4ae0();
  return v4 * dat_9a08;
}

// Function: sub_4d10 @ 0x4d10
void sub_4d10(char *a0)
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
    dat_d178 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_4db0 @ 0x4db0
char * sub_4db0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_8250();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x9a5f;
      return (char *)0x9a58;
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

// Function: sub_4e90 @ 0x4e90
long * sub_4e90(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_4f00:
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
label_53a6:
      v2 = 0;
      goto label_5360;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_54c1;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_5360:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_54c1;
      }
      goto label_53a6;
    case 5:
      if (!v18) goto label_5301;
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
          v10 = (char *)sub_4db0("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)"\'") {
          v10 = (char *)sub_4db0("\'",a4);
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
label_4feb:
  do {
label_4ffe:
    v23 = 0;
    a1 = v14;
label_5008:
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
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_5970;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_556c;
                default:
                  goto label_5243;
                case 7:
                  goto label_5557;
                case 8:
                  goto label_5526;
                case 9:
                  goto label_55f3;
                case 10:
                  goto label_554d;
                case 0xb:
                  goto label_5620;
                case 0xc:
                  goto label_550b;
                case 0xd:
                  goto label_545d;
                case 0x20:
                  goto label_562d;
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
                  goto label_51f6;
                case 0x23:
                  goto label_55d0;
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
                  goto label_5128;
                case 0x27:
                  goto label_5442;
                case 0x3f:
                  goto label_5401;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_5243;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_5128;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_5a14;
              goto label_5958;
            }
            if (v8 == 0x7d) goto label_5ae0;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_593a;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_5958;
              goto label_5243;
            }
            if (v8 != 0x7e) goto label_5243;
label_5a94:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_52b9;
            }
            v21 = 0x7e;
            goto label_5aa2;
          }
          goto label_522c;
        }
label_5970:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_5a94;
                goto label_5240;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_5243;
                goto label_5958;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_593a:
            if (a3 == 0xffffffffffffffff) goto label_5aef;
label_5948:
            v5 = v22;
            if (a3 == 1) goto label_55d5;
label_5958:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_52b9;
          }
          if (v8 == 0x40) goto label_5240;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_5128;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_5958;
label_5a14:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_5a3a;
            v17 = 0x5c;
            goto label_5462;
          }
label_5243:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_57c3;
label_5250:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_529e;
        }
        switch(v8) {
          case 0:
            goto label_555e;
          default:
label_5240:
            v22 = 0;
            goto label_5243;
          case 7:
label_5557:
            v17 = 0x61;
            goto label_5537;
          case 8:
label_5526:
            v17 = 0x62;
            goto label_5537;
          case 9:
            v22 = 0;
label_55f3:
            v21 = 9;
            v17 = 0x74;
            goto label_5600;
          case 10:
label_554d:
            v17 = 0x6e;
            break;
          case 0xb:
label_5620:
            v17 = 0x76;
            break;
          case 0xc:
label_550b:
            v17 = 0x66;
label_5537:
            if (v25) {
              v2 = 1;
              goto label_548f;
            }
            v24 = 0;
            goto label_51ae;
          case 0xd:
label_545d:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_562d:
            v21 = 0x20;
            goto label_5128;
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
            goto label_51f9;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_55d5;
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
label_5128:
            v8 = 0;
            goto label_5138;
          case 0x27:
            v22 = 0;
            goto label_5442;
          case 0x3f:
            v22 = 0;
            goto label_5401;
          
        }
        goto label_5462;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_5ae0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_5948;
label_5aef:
            v5 = v22;
            if (a2[1]) goto label_5958;
            goto label_55d5;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_51f6;
              goto label_5243;
            }
            goto label_593a;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_55d5;
          if (v33 == 1) goto label_5250;
label_57c3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_4a50(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_5d2b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_60af;
              goto label_60a5;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_58fd;
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
          goto label_5d2b;
        }
        if (v17 == 0x40) goto label_5243;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_52b0;
        if ((unsigned long)v10 & 0xa4000000) goto label_51f6;
        if (v17 != 0x5c) goto label_5243;
        if (a4 == 2) {
          if (!v25) goto label_5a3a;
          goto label_5223;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_5600;
        }
label_5a3a:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_5a50;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_56d2;
            goto label_560b;
          }
label_555e:
          if (!v25) {
            v22 = 0;
label_556c:
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
                goto label_52e3;
              }
label_5e75:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_5e75;
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
label_5eac:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_52e3;
              goto label_5138;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_52e3;
          }
          if (a4 != 2) goto label_522c;
          goto label_549b;
        default:
          goto label_5243;
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
          goto label_568f;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_568f;
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
label_568f:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_5600;
label_5211:
          a4 = 2;
label_5223:
          if (!v28) goto label_522c;
          goto label_549b;
        case 0x20:
          v19 = 0x20;
          goto label_51f9;
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
label_51f6:
          v26 = 0;
label_51f9:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_5211;
          goto label_52b7;
        case 0x23:
label_55d0:
          v21 = 0x23;
label_55d5:
          if (!v23) {
            v19 = v21;
            goto label_51f9;
          }
label_5aa2:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_52b9;
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
          goto label_52b0;
        case 0x27:
label_5442:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_52b9;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_5ecb;
            }
            else {
label_5ecb:
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
            goto label_52e3;
          }
          goto label_5223;
        case 0x3f:
label_5401:
          if (a4 == 2) {
            if (v25) goto label_5223;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_52e3;
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
                goto label_52b9;
              }
              if (v25) goto label_522c;
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
              goto label_5eac;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_52b9;
        
      }
label_5600:
      if (!v2) {
label_560b:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_52e0;
        goto label_5138;
      }
label_5462:
      v24 = 0;
      goto label_5473;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_616b:
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
        if (a1 || !v32) goto label_616b;
        v23 = v32;
        goto label_5bfb;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_621b;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_4ffe;
    }
label_58fd:
    if (v2) {
label_549b:
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
label_54c1:
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
label_5bfb:
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
label_60a5:
    if (!a2[(long)v10]) break;
  }
label_60af:
  v24 = 0;
label_5d2b:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_5d35:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_5223;
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
        if (v21 <= v23) goto label_51c1;
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
          goto label_5a50;
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
label_529e:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_5d35;
  }
label_52b0:
  v21 = v19;
  v24 = v26;
label_52b7:
  v8 = a4 == 2;
  v26 = v3;
label_52b9:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_5138:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_52e3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_52e3;
  }
  else {
label_52e0:
    v8 = 0;
    v3 = v26;
label_52e3:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_5a50:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_51c1;
    }
  }
label_5473:
  if (v25) {
label_548f:
    if ((bool)(v2 & a4 == 2)) goto label_549b;
label_522c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_4f00;
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
label_51ae:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_51c1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_5008;
label_56d2:
  v23 += 1;
  goto label_5008;
label_621b:
  a1 = v32;
label_5301:
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
  goto label_4feb;
}

// Function: sub_6320 @ 0x6320
char * sub_6320(unsigned int a0,char *a1,unsigned long a2,struct_0 *a3)
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
    v5 = dat_d080;
    if (dat_d078 <= (int)a0) {
      v10 = (long)dat_d078;
      v8 = (long)(int)((a0 - dat_d078) + 1);
      if (dat_d080 != (unsigned long *)0xd090) // branch-flip
        v5 = (unsigned long *)sub_7ef0(dat_d080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_7ef0(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_d090;
        v5[1] = dat_d098;
      }
      dat_d080 = v5;
      memset(&v5[(long)dat_d078 * 2],0,(v10 - dat_d078) * 0x10);
      dat_d078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_4e90(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0xd180)
        free(v7);
      v7 = (char *)sub_7c50(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_4e90(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_6510 @ 0x6510
void sub_6510(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0xd280;
  sub_8050(a0,0x38);
  *v2 = v1;
}

// Function: sub_6550 @ 0x6550
unsigned int sub_6550(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0xd280;
  return *a0;
}

// Function: sub_6570 @ 0x6570
void sub_6570(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0xd280;
  *a0 = a1;
}

// Function: sub_6590 @ 0x6590
unsigned int sub_6590(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0xd280;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_65d0 @ 0x65d0
unsigned int sub_65d0(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0xd280;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_65f0 @ 0x65f0
void sub_65f0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0xd280;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_6630 @ 0x6630
void sub_6630(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0xd280;
  v2 = __errno_location();
  v1 = *v2;
  sub_4e90(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_66b0 @ 0x66b0
char * sub_66b0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0xd280;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_4e90(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_7c50(v3 + 1U);
  sub_4e90(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_67a0 @ 0x67a0
char * sub_67a0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_66b0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_67b0 @ 0x67b0
void sub_67b0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_d080;
  if (2 <= dat_d078) {
    v4 = (unsigned long *)((long)dat_d080 + 0x18);
    v1 = (unsigned long *)((long)dat_d080 + (unsigned long)(unsigned int)(dat_d078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0xd180) {
    free(*(void **)((long)v3 + 8));
    dat_d098 = 0xd180;
    dat_d090 = 0x100;
  }
  if (v3 == (void *)0xd090) {
    dat_d078 = 1;
    return;
  }
  free(v3);
  dat_d078 = 1;
  dat_d080 = (void *)0xd090;
}

// Function: sub_6850 @ 0x6850
char * sub_6850(unsigned int a0,char *a1)
{
  return sub_6320(a0,a1,0xffffffffffffffff,(struct_0 *)0xd280); // tail-call
}

// Function: sub_6870 @ 0x6870
char * sub_6870(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_6320(a0,a1,a2); // tail-call
}

// Function: sub_6880 @ 0x6880
char * sub_6880(char *a0)
{
  return sub_6320(0,a0,0xffffffffffffffff,(struct_0 *)0xd280); // tail-call
}

// Function: sub_68a0 @ 0x68a0
char * sub_68a0(char *a0,unsigned long a1)
{
  return sub_6320(0,a0,a1,(struct_0 *)0xd280); // tail-call
}

// Function: sub_68c0 @ 0x68c0
void sub_68c0(unsigned int a0,int a1,char *a2)
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
    sub_6320(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6960 @ 0x6960
void sub_6960(unsigned int a0,int a1,char *a2,unsigned long a3)
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
    sub_6320(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_69f0 @ 0x69f0
void sub_69f0(int a0,char *a1)
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
    sub_6320(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6a80 @ 0x6a80
void sub_6a80(int a0,char *a1,unsigned long a2)
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
    sub_6320(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6b10 @ 0x6b10
void sub_6b10(char *a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_d280;
  v4 = dat_d288;
  v9 = dat_d2b0;
  v5 = dat_d290;
  v6 = dat_d298;
  v7 = dat_d2a0;
  v8 = dat_d2a8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_6320(0,a0,a1,&v3);
}

// Function: sub_6bb0 @ 0x6bb0
void sub_6bb0(char *a0,unsigned char a1)
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
  
  v3 = dat_d280;
  v4 = dat_d288;
  v9 = dat_d2b0;
  v5 = dat_d290;
  v6 = dat_d298;
  v7 = dat_d2a0;
  v8 = dat_d2a8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_6320(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_6c50 @ 0x6c50
void sub_6c50(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_d280;
  v3 = (unsigned int)((unsigned long)dat_d288 >> 0x20);
  v8 = dat_d2b0;
  v4 = dat_d290;
  v5 = dat_d298;
  v6 = dat_d2a0;
  v7 = dat_d2a8;
  v2 = (unsigned int)dat_d288;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_6320(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_6ce0 @ 0x6ce0
void sub_6ce0(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_d280;
  v3 = (unsigned int)((unsigned long)dat_d288 >> 0x20);
  v8 = dat_d2b0;
  v4 = dat_d290;
  v5 = dat_d298;
  v6 = dat_d2a0;
  v7 = dat_d2a8;
  v2 = (unsigned int)dat_d288;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_6320(0,a0,a1,&v1);
}

// Function: sub_6d70 @ 0x6d70
void sub_6d70(unsigned int a0,int a1,char *a2)
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
    sub_6320(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6e10 @ 0x6e10
void sub_6e10(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_d288;
  v4 = dat_d290;
  v5 = dat_d298;
  v8 = dat_d2b0;
  v2 = (unsigned int)((unsigned long)dat_d280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_d2a0;
  v7 = dat_d2a8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_6320(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6eb0 @ 0x6eb0
void sub_6eb0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_d288;
  v4 = dat_d290;
  v5 = dat_d298;
  v8 = dat_d2b0;
  v2 = (unsigned int)((unsigned long)dat_d280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_d2a0;
  v7 = dat_d2a8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_6320(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6f50 @ 0x6f50
void sub_6f50(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_d288;
  v4 = dat_d290;
  v5 = dat_d298;
  v8 = dat_d2b0;
  v2 = (unsigned int)((unsigned long)dat_d280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_d2a0;
  v7 = dat_d2a8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_6320(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6ff0 @ 0x6ff0
void sub_6ff0(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_d288;
  v4 = dat_d290;
  v5 = dat_d298;
  v8 = dat_d2b0;
  v2 = (unsigned int)((unsigned long)dat_d280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_d2a0;
  v7 = dat_d2a8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_6320(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_7090 @ 0x7090
char * sub_7090(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_6320(a0,a1,a2); // tail-call
}

// Function: sub_70a0 @ 0x70a0
char * sub_70a0(char *a0,unsigned long a1)
{
  return sub_6320(0,a0,a1,(struct_0 *)0xd040); // tail-call
}

// Function: sub_70c0 @ 0x70c0
char * sub_70c0(unsigned int a0,char *a1)
{
  return sub_6320(a0,a1,0xffffffffffffffff,(struct_0 *)0xd040); // tail-call
}

// Function: sub_70e0 @ 0x70e0
char * sub_70e0(char *a0)
{
  return sub_6320(0,a0,0xffffffffffffffff,(struct_0 *)0xd040); // tail-call
}

// Function: sub_7100 @ 0x7100
void sub_7100(struct_7 *a0)
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

// Function: sub_71f0 @ 0x71f0
void sub_71f0(unsigned long *a0)
{
  void *v1;
  void *v2;
  
  v2 = dat_cfe0;
  v1 = dat_cfc0;
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin((obstack *)&a0[3],0,0,dat_cfe0,dat_cfc0);
  _obstack_begin((obstack *)&a0[0xe],0,0,v2,v1);
  _obstack_begin((obstack *)&a0[0x19],0,0,v2,v1); // tail-call
}

// Function: sub_7270 @ 0x7270
void sub_7270(long a0)
{
  _obstack_free((obstack *)(a0 + 0x18),NULL);
  _obstack_free((obstack *)(a0 + 0x70),NULL);
  _obstack_free((obstack *)(a0 + 200),NULL); // tail-call
}

// Function: sub_72a0 @ 0x72a0
unsigned long sub_72a0(FILE *a0,void *a1)
{
  int v1; // eax
  char *v2;
  long v3; // rax
  unsigned long v4; // rax
  unsigned long *v5;
  unsigned long v6;
  
label_72ce:
  do {
    v1 = fgetc(a0);
    v2 = *(char **)((long)a1 + 0x30);
    if (v1 == -1) {
      if (v2 != *(char **)((long)a1 + 0x28)) {
        if (v2 == *(char **)((long)a1 + 0x38)) {
          _obstack_newchunk((obstack *)((long)a1 + 0x18),1);
          v2 = *(char **)((long)a1 + 0x30);
        }
        *(char **)((long)a1 + 0x30) = &v2[1];
        *v2 = 0;
        sub_7100(a1);
      }
      v5 = *(unsigned long **)((long)a1 + 0x88);
      if ((unsigned long)(*(long *)((long)a1 + 0x90) - (long)v5) <= 7) {
        _obstack_newchunk((obstack *)((long)a1 + 0x70),8);
        v5 = *(unsigned long **)((long)a1 + 0x88);
      }
      *v5 = 0;
      v3 = *(long *)((long)a1 + 0x88) + 8;
      if (v3 == *(long *)((long)a1 + 0x80))
        *(unsigned char *)((long)a1 + 0xc0) = *(unsigned char *)((long)a1 + 0xc0) | 2;
      *(long *)((long)a1 + 8) = *(long *)((long)a1 + 0x80);
      v4 = v3 + *(unsigned long *)((long)a1 + 0xa0) & ~*(unsigned long *)((long)a1 + 0xa0);
      v6 = *(unsigned long *)((long)a1 + 0x90);
      if (v4 - *(long *)((long)a1 + 0x78) <= *(unsigned long *)((long)a1 + 0x90) - *(long *)((long)a1 + 0x78))
        v6 = v4;
      *(unsigned long *)((long)a1 + 0x88) = v6;
      *(unsigned long *)((long)a1 + 0x80) = v6;
      if (*(long *)((long)a1 + 0xe0) == *(long *)((long)a1 + 0xd8))
        *(unsigned char *)((long)a1 + 0x118) = *(unsigned char *)((long)a1 + 0x118) | 2;
      *(long *)((long)a1 + 0x10) = *(long *)((long)a1 + 0xd8);
      v4 = *(long *)((long)a1 + 0xe0) + *(unsigned long *)((long)a1 + 0xf8) & ~*(unsigned long *)((long)a1 + 0xf8);
      v6 = *(unsigned long *)((long)a1 + 0xe8);
      if (v4 - *(long *)((long)a1 + 0xd0) <= *(unsigned long *)((long)a1 + 0xe8) - *(long *)((long)a1 + 0xd0))
        v6 = v4;
      *(unsigned long *)((long)a1 + 0xe0) = v6;
      *(unsigned long *)((long)a1 + 0xd8) = v6;
      v1 = ferror(a0);
      return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
    }
    if (*(char **)((long)a1 + 0x38) != v2) goto label_72c0;
    _obstack_newchunk((obstack *)((long)a1 + 0x18),1);
    v2 = *(char **)((long)a1 + 0x30);
    *(char **)((long)a1 + 0x30) = &v2[1];
    *v2 = (char)v1;
  } while (v1);
  goto label_7310;
label_72c0:
  *(char **)((long)a1 + 0x30) = &v2[1];
  *v2 = (char)v1;
  if (!v1) {
label_7310:
    sub_7100(a1);
  }
  goto label_72ce;
}

// Function: sub_7490 @ 0x7490
long sub_7490(int a0,void *a1,unsigned long a2)
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

// Function: sub_7500 @ 0x7500
long sub_7500(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_762d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_7666:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xa128) + 0xa128;
  switch(a5) {
    case 0:
      goto label_7666;
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
      goto label_762d;
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

// Function: sub_7980 @ 0x7980
void sub_7980(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_7500(); // tail-call
}

// Function: sub_79a0 @ 0x79a0
void sub_79a0(FILE *a0,char *a1,char *a2,char *a3,struct_10 *a4)
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
  sub_7500(a0,a1,a2,a3);
}

// Function: sub_7a40 @ 0x7a40
void sub_7a40(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_7500(a0,a1,a2);
}

// Function: sub_7b20 @ 0x7b20
void sub_7b20(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_7bd0 @ 0x7bd0
void sub_7bd0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_8110(); // no-return
}

// Function: sub_7c10 @ 0x7c10
void sub_7c10(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_8110(); // no-return
}

// Function: sub_7c30 @ 0x7c30
void sub_7c30(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_8110(); // no-return
}

// Function: sub_7c50 @ 0x7c50
void sub_7c50(void)
{
  sub_7c10(); // tail-call
}

// Function: sub_7c60 @ 0x7c60
void sub_7c60(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_8110(); // no-return
}

// Function: sub_7ca0 @ 0x7ca0
void sub_7ca0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_8110(); // no-return
}

// Function: sub_7cd0 @ 0x7cd0
void sub_7cd0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_8110(); // no-return
}

// Function: sub_7d10 @ 0x7d10
void sub_7d10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_8110(); // no-return
}

// Function: sub_7d50 @ 0x7d50
void sub_7d50(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_8110(); // no-return
}

// Function: sub_7d80 @ 0x7d80
void sub_7d80(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_8110(); // no-return
}

// Function: sub_7dd0 @ 0x7dd0
void sub_7dd0(void *a0,unsigned long *a1)
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
  sub_8110(); // no-return
}

// Function: sub_7e60 @ 0x7e60
void sub_7e60(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_8110(); // no-return
}

// Function: sub_7ef0 @ 0x7ef0
void sub_7ef0(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_7fae;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_7fae:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_8110(); // no-return
}

// Function: sub_7ff0 @ 0x7ff0
void sub_7ff0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_8110(); // no-return
}

// Function: sub_8010 @ 0x8010
void sub_8010(unsigned long a0)
{
  sub_7ff0(a0,1); // tail-call
}

// Function: sub_8020 @ 0x8020
void sub_8020(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_8110(); // no-return
}

// Function: sub_8040 @ 0x8040
void sub_8040(unsigned long a0)
{
  sub_8020(a0,1); // tail-call
}

// Function: sub_8050 @ 0x8050
void sub_8050(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7c10(a1),a0,a1); // tail-call
}

// Function: sub_8080 @ 0x8080
void sub_8080(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7c30(a1),a0,a1); // tail-call
}

// Function: sub_80b0 @ 0x80b0
void sub_80b0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_7c30(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_80f0 @ 0x80f0
void sub_80f0(char *a0)
{
  sub_8050(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_8110 @ 0x8110
void sub_8110(void)
{
  error(dat_d020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_8150 @ 0x8150
unsigned long sub_8150(FILE *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *(unsigned int *)a0;
  v3 = sub_47e0(a0);
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

// Function: sub_81c0 @ 0x81c0
bool sub_81c0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_85e0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_8250 @ 0x8250
char * sub_8250(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_8290 @ 0x8290
void sub_8290(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_d020); // no-return
}

// Function: sub_82e0 @ 0x82e0
long * sub_82e0(struct_6 *a0,long a1,long a2) // ternary
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
  v1 = (a0->field_0x50 & 1) ? (long *)(*a0->field_0x38)(a0->field_0x48) : (long *)(*a0->field_0x38)(a1); // branch-flip
  a0->field_0x8 = v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0->field_0x10 = v2;
    a0->field_0x18 = v2;
    v3 = a0->field_0x0 + (long)v1;
    *v1 = v3;
    a0->field_0x20 = v3;
    v1[1] = 0;
    v1 = (long *)0x1;
    a0->field_0x50 = a0->field_0x50 & 0xf9;
    return v1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_begin @ 0x8380
int _obstack_begin(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4)
{
  a0->field_0x50 = a0->field_0x50 & 0xfe;
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  return sub_82e0(a0,a1,a2); // tail-call
}

// Function: _obstack_begin_1 @ 0x83a0
int _obstack_begin_1(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4,void *a5)
{
  a0->field_0x50 = a0->field_0x50 | 1;
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  *(void **)&a0->field_0x48 = a5;
  return sub_82e0(a0,a1,a2); // tail-call
}

// Function: _obstack_newchunk @ 0x83c0
void _obstack_newchunk(obstack *a0,unsigned long a1) // ternary
{
  unsigned long v1;
  unsigned char v2;
  long v3;
  void *v4;
  long *v5; // rax
  unsigned long v6;
  unsigned long v7; // r13
  void *v8; // r14
  
  v7 = *(long *)&a0->field_0x18 - *(long *)&a0->field_0x10;
  v3 = *(long *)&a0->field_0x8;
  v6 = a1 + v7 + *(long *)&a0->field_0x30;
  v1 = v6 + 100 + (v7 >> 3);
  if (v6 < *(unsigned long *)a0)
    v6 = *(unsigned long *)a0;
  if (v6 <= v1)
    v6 = v1;
  if ((!CARRY8(a1,v7)) && (!CARRY8(a1 + v7,*(unsigned long *)&a0->field_0x30))) {
    v5 = (a0->field_0x50 & 1) ? (long *)(**(void **)&a0->field_0x38)(*(unsigned long *)&a0->field_0x48) : (long *)(**(void **)&a0->field_0x38)(v6); // branch-flip
    if (v5) {
      *(long **)&a0->field_0x8 = v5;
      v4 = *(void **)&a0->field_0x10;
      v5[1] = v3;
      *(unsigned long *)&a0->field_0x20 = (long)v5 + v6;
      *v5 = (long)v5 + v6;
      v8 = (void *)((long)v5 + *(unsigned long *)&a0->field_0x30 + 0x10 & ~*(unsigned long *)&a0->field_0x30);
      memcpy(v8,v4,v7);
      v2 = a0->field_0x50;
      if ((!(v2 & 2)) && (*(unsigned long *)&a0->field_0x10 == (~*(unsigned long *)&a0->field_0x30 & v3 + 0x10 + *(unsigned long *)&a0->field_0x30))) {
        v5[1] = *(long *)(v3 + 8);
        if (v2 & 1) // branch-flip
          (**(void **)&a0->field_0x40)(*(unsigned long *)&a0->field_0x48,v3);
        else {
          (**(void **)&a0->field_0x40)(v3);
        }
      }
      *(void **)&a0->field_0x10 = v8;
      *(unsigned long *)&a0->field_0x18 = (long)v8 + v7;
      a0->field_0x50 = a0->field_0x50 & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x84f0
unsigned long _obstack_allocated_p(void *a0,unsigned long *a1)
{
  unsigned long *v1;
  
  v1 = *(unsigned long **)((long)a0 + 8);
  if (!v1)
    return 0;
  while ((a1 <= v1 || ((unsigned long *)*v1 < a1))) {
    v1 = (unsigned long *)v1[1];
    if (!v1)
      return 0;
  }
  return 1;
}

// Function: _obstack_free @ 0x8530
void _obstack_free(obstack *a0,void *a1) // return-dupe
{
  void *v1;
  unsigned long *v2;
  unsigned long *v3;
  
  v3 = *(unsigned long **)&a0->field_0x8;
  if (*(unsigned long **)&a0->field_0x8) {
    while( true ) {
      if ((v3 < a1) && (v1 = (void *)*v3, a1 <= v1)) {
        *(void **)&a0->field_0x18 = a1;
        *(void **)&a0->field_0x10 = a1;
        *(void **)&a0->field_0x20 = v1;
        *(unsigned long **)&a0->field_0x8 = v3;
        return;
      }
      v2 = (unsigned long *)v3[1];
      if (a0->field_0x50 & 1) { // branch-flip
        (**(void **)&a0->field_0x40)(*(unsigned long *)&a0->field_0x48);
        a0->field_0x50 = a0->field_0x50 | 2;
      }
      else {
        (**(void **)&a0->field_0x40)(v3);
        a0->field_0x50 = a0->field_0x50 | 2;
      }
      if (!v2) break;
      v3 = v2;
    }
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x85b0
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

// Function: sub_85e0 @ 0x85e0
unsigned long sub_85e0(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_8690 @ 0x8690
void sub_8690(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_86a0 @ 0x86a0
unsigned long sub_86a0(char *a0,unsigned int a1,long *a2,long *a3)
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
    while (v2 = sub_7490(a1,(char (*)[32])v8,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = (char *)sub_6d70(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v18 & 0xffffffff; // return-dupe
      }
      v7 += v2;
      if (0x40 <= v2) { // branch-flip
        v22 = ZEXT1664(0);
        v21 = SUB6432(v22,0);
        v6 = (char (*)[32])v8;
        do {
          v20 = vpcmpeqb_avx2(dat_a1c0,*v6);
          v1 = vpcmpeqb_avx2(dat_a1c0,v6[1]);
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

// Function: sub_88e0 @ 0x88e0
void sub_88e0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_d008); // tail-call
}

// Function: _DT_FINI @ 0x88f4
void _DT_FINI(void)
{
  return;
}
