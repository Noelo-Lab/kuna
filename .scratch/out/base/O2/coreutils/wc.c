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
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_cde8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x24c0
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_cdf0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x24d0
void iswcntrl(void)
{
  (*dat_cdf8)(); // jump-as-call
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
int fclose(void *a0)
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
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ce40)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2570
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
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
int __overflow(void *a0,int a1)
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
int fputs(char *a0,void *a1)
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
void fgetc(void)
{
  (*dat_ce88)(); // jump-as-call
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
int fputs_unlocked(char *a0,void *a1)
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
int feof(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ced8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x26a0
int stat(char *a0,void *a1)
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
int fileno(void *a0)
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
int fflush(void *a0)
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
int __freading(void *a0)
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
int setvbuf(void *a0,char *a1,int a2,unsigned long a3)
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
void btowc(void)
{
  (*dat_cf40)(); // jump-as-call
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
void fseeko(void)
{
  (*dat_cf58)(); // jump-as-call
}

// Function: fopen @ 0x27a0
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf60)(); // jump-as-call
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
void sysinfo(void)
{
  (*dat_cf78)(); // jump-as-call
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
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_cf90)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2810
int mbsinit(void *a0)
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
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_cfa8)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x2840
void getdelim(void)
{
  (*dat_cfb0)(); // jump-as-call
}

// Function: __ctype_b_loc @ 0x2850
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cfb8)(); // jump-as-call
  return v1;
}

// Function: sub_2860 @ 0x2860
unsigned long sub_2860(int a0,char **a1)
{
  bool v1;
  char *v10;
  unsigned long v11; // rax
  int *v12;
  unsigned long v13; // rax
  unsigned long v14; // rcx
  long v15;
  unsigned long v16;
  char v17 [24];
  int v18; // stack - 0x1fc
  unsigned long v19; // stack - 0x1f8
  unsigned char v2; // al
  unsigned long v20;
  long v21; // fs_offset
  double v22;
  long v23; // stack - 0x230
  int *v24; // stack - 0x220
  unsigned char v25; // stack - 0x218
  char v26; // stack - 0x20d
  char **v27; // stack - 0x1f0
  unsigned int v28; // stack - 0xc0
  long v29; // stack - 0xa8
  unsigned char v3;
  long v30; // stack - 0x40
  int v4; // eax
  long v5; // rax
  char *v6;
  void *v7;
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v30 = *(long *)(v21 + 0x28);
  sub_4d10(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_88e0(sub_46f0);
  dat_d120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  dat_d131 = 0;
  dat_d132 = 0;
  dat_d118 = getenv("POSIXLY_CORRECT") != NULL;
  dat_d133 = 0;
  dat_d134 = 0;
  dat_d130 = 0;
  dat_d138 = 0;
  dat_d140 = 0;
  dat_d148 = 0;
  dat_d150 = 0;
  dat_d158 = 0;
  v10 = NULL;
  while( true ) {
    v9 = 0;
    v4 = getopt_long(a0,a1,"clLmw",(void *)0xca20,NULL);
    if (v4 == -1) break;
    if (0x81 < v4) {
      sub_4230(1); // return-dupe, no-return
    }
    if (v4 <= 0x4b) {
      if (v4 == -0x83) {
        sub_7a40(stdout,"wc","GNU coreutils",dat_d018,"Paul Rubin","David MacKenzie",0,v9);
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
        v10 = optarg;
      
    }
  }
  if ((((!dat_d134) && (!dat_d133)) && (!dat_d132)) && ((!dat_d131 && (!dat_d130)))) {
    dat_d131 = 1;
    dat_d133 = 1;
    dat_d134 = 1;
  }
  if (v10) { // branch-flip
    if (optind < a0) {
      v9 = sub_69f0(4,a1[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v9);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_4230(1);
    }
    if (strcmp(v10,"-")) { // branch-flip
      v7 = fopen(v10,"r");
      if (!v7) {
        v9 = sub_69f0(4,v10);
        v10 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v10,v9);
        return v8;
      }
    }
    else {
      v7 = stdin;
    }
    v4 = fileno(v7);
    if ((fstat(v4,v17)) || ((v28 & 0xf000) != 0x8000)) {
label_2ad5:
      v5 = sub_45b0(v7);
      if (!v5) {
        sub_8110(); // no-return, return-dupe
      }
      v26 = '\0';
      v24 = (int *)sub_7d50(1,0x98);
    }
    else {
      v22 = dat_99b8;
      if ((double)sub_4bd0() * dat_99c0 <= dat_99b8)
        v22 = dat_99c0 * (double)sub_4bd0();
      if (v22 < (double)v29) goto label_2ad5;
      sub_71f0(&v19);
      v26 = sub_72a0(v7,&v19);
      if ((!v26) || (sub_47e0(v7))) {
        v9 = sub_69f0(4,v10);
        error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v9);
        return v13;
      }
      v5 = sub_4580(v27);
      if (!v5) {
        sub_8110();
      }
      v16 = 1;
      if (v19)
        v16 = v19;
      v24 = (int *)sub_7d50(v16,0x98);
      v16 = v19;
      a1 = v27;
      if (v19) goto label_3024;
    }
    v16 = 0;
    *v24 = 1;
    dat_d12c = 1;
  }
  else {
    if (a0 <= optind) { // branch-flip
      v16 = 1;
      a1 = (char **)0xd110;
    }
    else {
      a1 = &a1[optind];
      v16 = (unsigned long)(a0 - optind);
    }
    v5 = sub_4580(a1);
    if (!v5) {
      sub_8110();
    }
    v26 = '\0';
    v24 = (int *)sub_7d50(v16,0x98);
label_3024:
    if ((v16 != 1) || ((unsigned int)dat_d134 + (unsigned int)dat_d133 + (unsigned int)dat_d132 + (unsigned int)dat_d131 + (unsigned int)dat_d130 != 1)) {
      v20 = 0;
      v12 = &v24[2];
      do {
        v6 = a1[v20];
        if ((v6) && (strcmp(v6,"-")))
          v4 = stat(v6,v12);
        else {
          v4 = fstat(0,v12);
        }
        v20 += 1;
        v12[-2] = v4;
        v12 = &v12[0x26];
      } while (v20 < v16);
      dat_d12c = 1;
      if (*v24 <= 0) {
        v4 = 1;
        v20 = 0;
        v14 = 0;
        v12 = v24;
        do {
          if (!*v12) {
            if ((v12[8] & 0xf000U) != 0x8000) // branch-flip
              v4 = 7;
            else {
              v20 += *(long *)&v12[0xe];
            }
          }
          v14 += 1;
          v12 = &v12[0x26];
        } while (v14 < v16);
        dat_d12c = 1;
        if (10 <= v20) {
          do {
            dat_d12c += 1;
            v14 = v20 / 10;
            v1 = 100 <= v20;
            v20 = v14;
          } while (v1);
        }
        if (dat_d12c <= v4)
          dat_d12c = v4;
      }
    }
    else {
      *v24 = 1;
      dat_d12c = 1;
    }
  }
  v23 = 0;
  v3 = 1;
  while (v6 = (char *)sub_45f0(v5,&v18), v6) {
    if (v10) { // branch-flip
      if (strcmp(v10,"-")) { // branch-flip
        if (*v6) goto label_2c10;
      }
      else if (strcmp(v6,"-")) { // branch-flip
        if (*v6) {
          if (v16) {
            v15 = v23;
            goto label_2c1b;
          }
          v12 = v24;
          goto label_2b6c;
        }
      }
      else {
        sub_69f0(4,v6);
        error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5));
        if (*v6) goto label_2de1;
      }
      v9 = sub_4690(v5);
      v6 = dcgettext(NULL,"invalid zero-length file name",5);
      error(0,0,"%s:%lu: %s",sub_6d70(0,3,v10),v9,v6);
      v3 = 0;
    }
    else if (*v6) { // branch-flip
label_2c10:
      v15 = 0;
      if (v16)
        v15 = v23;
label_2c1b:
      v12 = (int *)(v15 + (long)v24);
      if (strcmp(v6,"-")) { // branch-flip
label_2b6c:
        v4 = open(v6,0);
        if (v4 != -1) {
          v25 = sub_3820(v4,v6,v12,0);
          if (!close(v4)) goto label_2bb5;
        }
        sub_6d70(0,3,v6);
        error(0,*__errno_location(),"%s");
        v25 = 0;
      }
      else {
        dat_d128 = '\x01';
        v25 = sub_3820(0,v6,(int *)(v15 + (long)v24),0xffffffffffffffff);
      }
label_2bb5:
      v3 &= v25;
    }
    else {
      dcgettext(NULL,"invalid zero-length file name",5);
      error(0,0,"%s");
label_2de1:
      v3 = 0;
    }
    if (!v16)
      *v24 = 1;
    v23 += 0x98;
  }
  if (v18 != 3) {
    if (v18 != 4) { // branch-flip
      if (v18 != 2)
        __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
      v2 = v10 == NULL & v3;
      if (v2) {
        v3 = v2;
        if (!sub_4690(v5)) {
          dat_d128 = '\x01';
          v3 = sub_3820(0,0,v24,0xffffffffffffffff);
        }
      }
    }
    else {
      v9 = sub_6d70(0,3,v10);
      v10 = dcgettext(NULL,"%s: read error",5);
      error(0,*__errno_location(),v10,v9);
      v3 = 0;
    }
    if (v26)
      sub_7270(&v19);
    if (2 <= (unsigned long)sub_4690(v5))
      sub_3430(dat_d158,dat_d150,dat_d148,dat_d140,dat_d138,dcgettext(NULL,"total",5));
    sub_46b0(v5);
    free(v24);
    if ((dat_d128) && (close(0))) {
      error(1,*__errno_location(),"-");
      return v11;
    }
    if (v30 != *(long *)(v21 + 0x28))
      __stack_chk_fail(); // no-return
    return (unsigned long)(v3 ^ 1);
  }
  sub_8110();
}

// Function: sub_3340 @ 0x3340
void sub_3340(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_cfc8)(sub_2860,v2,&Stack0000000000000008,0,0,a2,v1);
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
  v1 = *(char **)((long)stdout + 0x28);
  if (v1 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10);
}

// Function: sub_3640 @ 0x3640
unsigned long sub_3640(unsigned long a0,unsigned int a1,long *a2,long *a3)
{
  char *v1;
  long v10; // stack - 0x4068
  bool v2;
  unsigned long v3; // rax
  char *v4; // rax
  char *v5;
  unsigned long v6; // rax
  long v7;
  char v8 [16392];
  long v9;
  
  v2 = 0;
  v10 = 0;
  v9 = 0;
  while( true ) {
    v3 = sub_7490(a1,v8,0x4000);
    if (!v3) {
      *a3 = v10;
      *a2 = v9;
      return 1;
    }
    if (v3 == 0xffffffffffffffff) break;
    v10 += v3;
    v1 = &v8[v3];
    if (v2) {
      *v1 = '\n';
      v7 = v9;
      v5 = v8;
      while (v5 = rawmemchr(v5,10), v5 < v1) {
        v5 = &v5[1];
        v7 += 1;
      }
      v2 = (unsigned long)(v7 - v9) <= v3 / 0xf;
      v9 = v7;
    }
    else if (v1 != v8) { // branch-flip
      v5 = v8;
      v7 = v9;
      do {
        v4 = &v5[1];
        v7 += (unsigned long)(*v5 == '\n');
        v5 = v4;
      } while (v1 != v4);
      v2 = (unsigned long)(v7 - v9) <= v3 / 0xf;
      v9 = v7;
    }
    else {
      v2 = 1;
    }
  }
  v6 = sub_6d70(0,3,a0);
  error(0,*__errno_location(),"%s",v6);
  return 0;
}

// Function: sub_3800 @ 0x3800
unsigned long sub_3800(unsigned long a0,unsigned long a1,long a2,long a3)
{
  if ((a2) && (a3))
    return sub_3640(); // tail-call
  return 0;
}

// Function: sub_3820 @ 0x3820
unsigned int sub_3820(int a0,char *a1,int *a2,long a3) // ternary
{
  unsigned short v1;
  unsigned int v10; // stack - 0x4064
  long v11; // stack - 0x4060
  unsigned long v12;
  char *v13; // rsi
  unsigned char v14;
  long v15;
  unsigned long v16;
  unsigned char *v17;
  unsigned int v18;
  unsigned long v19; // r13
  unsigned long v2;
  unsigned char *v20;
  unsigned int v21;
  bool v22; // r9b
  long v23; // stack - 0x40a0
  unsigned long v24; // stack - 0x4090
  unsigned char v25; // stack - 0x4079
  char *v26; // stack - 0x4070
  unsigned char v3; // al
  unsigned int v4; // eax
  void *v5;
  unsigned long v6;
  unsigned long v7;
  unsigned char v8 [16392];
  unsigned long v9 [2]; // stack - 0x4058
  
  v26 = a1;
  if (!a1) {
    v5 = (char *)dcgettext(NULL,"standard input",5);
    v26 = (char *)v5;
  }
  v9[0] = 0;
  v11 = 0;
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  v14 = dat_d131; // branch-flip
  if (2 <= v5) {
    v3 = dat_d132;
    v25 = v3;
  }
  else {
    v25 = 0;
    if (!dat_d131)
      v14 = dat_d132;
  }
  if ((dat_d133) || (dat_d130))
    sub_47a0(a0,0,0,2);
  else {
    if ((v14 == 1) && (!v25)) {
      if (dat_d134) {
        sub_47a0(a0,0,0,2);
label_40e0:
        if (dat_d134) {
label_40f0:
          if (*(int *)cpuid_basic_info(0)) { // branch-flip
            if (!(*(unsigned int *)(cpuid_Version_info(1) + 0xc) & 0x8000000)) {
label_4120:
              v13 = "avx2 support not detected";
              if (dat_d160) goto label_417a;
              goto label_4135;
            }
            if (*(unsigned int *)cpuid_basic_info(0) <= 6) goto label_4165;
            if (!(*(unsigned int *)(cpuid_Extended_Feature_Enumeration_info(7) + 4) & 0x20)) goto label_4120;
            if (dat_d160) {
              v5 = (char *)dcgettext(NULL,"using avx2 hardware support",5);
              error(0,0,"%s",v5);
            }
            v5 = sub_86a0;
            dat_d010 = sub_86a0;
          }
          else {
label_4165:
            if (dat_d160) {
              v13 = "failed to get cpuid";
label_417a:
              v5 = (char *)dcgettext(NULL,v13,5);
              error(0,0,"%s",v5);
            }
label_4135:
            v5 = dat_d010;
          }
          v12 = 0;
          v16 = 0;
          v23 = 0;
          v4 = (*v5)(v26,a0,&v11,v9);
          v7 = v9[0];
          v21 = v4;
          goto label_39f8;
        }
      }
      v4 = *a2;
      if (0 < (int)v4) {
        v4 = fstat(a0,&a2[2]);
        *a2 = v4;
      }
      v16 = dat_d120;
      if (((!v4) && (v4 = a2[8] & 0xd000, v4 == 0x8000)) && (v7 = *(unsigned long *)&a2[0xe], 0 <= (long)v7)) {
        if (a3 != -1) { // branch-flip
          if (v7 % dat_d120) {
label_40b7:
            v12 = 0;
            v16 = 0;
            v23 = 0;
            v9[0] = v7;
            v21 = 1;
            goto label_39f8;
          }
          v14 = 1;
          v16 = 0;
        }
        else {
          v5 = (unsigned long)lseek(a0,0,1);
          if (v7 % v16) {
            v7 = (v5 <= v7) ? v7 - (long)v5 : 0; // branch-flip
            goto label_40b7;
          }
          v14 = (unsigned char)~(unsigned char)((unsigned long)v5 >> 0x38) >> 7;
          v16 = (unsigned long)v5;
        }
        v5 = (unsigned long)(*(long *)&a2[0x10] + 1);
        v12 = 0x201;
        if ((unsigned long)(*(long *)&a2[0x10] - 1U) <= 0x1fffffffffffffff)
          v12 = (unsigned long)v5;
        v15 = v7 - v7 % v12;
        if ((((long)v16 < v15) && (v14)) && (v5 = (long)lseek(a0,v15,1), 0 <= (long)v5))
          v9[0] = v15 - v16;
      }
      sub_47a0(a0,0,0,2);
      while (v5 = (long)sub_7490(a0,v8,0x4000), v5) {
        if (v5 == (void *)0xffffffffffffffff) {
          v12 = 0;
          v23 = 0;
          v5 = (unsigned long)sub_6d70(0,3,v26);
          v2 = v5;
          v5 = (int *)__errno_location();
          v16 = 0;
          error(0,*(int *)v5,"%s",v2);
          v7 = v9[0];
          v21 = 0;
          goto label_39f8;
        }
        v9[0] += (long)v5;
      }
      v12 = 0;
      v23 = 0;
      v16 = 0;
      v7 = v9[0];
      v21 = 1;
      goto label_39f8;
    }
    sub_47a0(a0,0,0,2);
    if (v25 != 1) {
      if (v14) goto label_40e0;
      if (!v25) goto label_40f0;
    }
  }
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v5) { // branch-flip
    v19 = 0;
    v22 = 0;
    v12 = 0;
    v9[1] = 0;
    v7 = 0;
    v16 = 0;
    v24 = 0;
    v23 = 0;
label_3ae0:
    v5 = (long)sub_7490(a0,&v8[v19],0x4000 - v19);
    if (v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v5 = (unsigned long)sub_6d70(0,3,v26);
        v2 = v5;
        v5 = (int *)__errno_location();
        v21 = 0;
        error(0,*(int *)v5,"%s",v2);
        goto label_3f68;
      }
      v9[0] += (long)v5;
      v19 += (long)v5;
      v17 = v8;
      do {
        v6 = v9[1];
        if (v22) {
label_3c30:
          v5 = (long)sub_4a50(&v10,v17,v19,&v9[1]);
          v15 = (long)v5;
          if (v5 != (void *)0xfffffffffffffffe) {
            if (v5 != (void *)0xffffffffffffffff) {
              v4 = mbsinit(&v9[1]);
              v22 = v4 == 0;
              if (v15) { // branch-flip
                switch(v10) {
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
                    v18 = v10;
                    break;
                  case 0x20:
                    goto label_3c06;
                  
                }
              }
              else {
                v10 = 0;
                v15 = 1;
                v18 = 0;
              }
              v4 = iswprint(v18);
              if (v4) {
                if (dat_d130) {
                  v4 = wcwidth(v10);
                  v6 = (long)(int)v4 + v12;
                  if (1 <= (int)v4)
                    v12 = v6;
                }
                v4 = iswspace(v10);
                if (v4) goto label_3be9;
                if (dat_d118) // branch-flip
                  v7 = (unsigned long)dat_d118;
                else {
                  if ((((v10 == 0xa0) || (v10 == 0x2007)) || (v10 == 0x202f)) || (v10 == 0x2060)) goto label_3be9;
                  v7 = 1;
                }
              }
              goto label_3ba8;
            }
            v17 = &v17[1];
            v15 = -1;
            v22 = 1;
            goto label_3bb2;
          }
          v9[1] = v6;
          if (v19) {
            if (v19 == 0x4000) {
              v17 = &v17[1];
              v19 = 0x3fff;
            }
            __memmove_chk(v8,v17,v19,0x4001);
          }
          v22 = 1;
          break;
        }
        v14 = *v17;
        v3 = v14 >> 5;
        v5 = (unsigned long)(unsigned long)v3;
        v4 = *(unsigned int *)((long)v5 * 4 + 0x99e0) >> (v14 & 0x1f);
        v18 = v4 & 1;
        if (!v18) goto label_3c30;
        v4 = (unsigned int)(char)v14;
        v10 = v4;
        switch(v14) {
          case 9:
            v15 = 1;
label_3c26:
            v12 = (v12 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v15 = 1;
label_3c16:
            v11 += 1;
            goto label_3bd6;
          case 0xb:
label_3bf8:
            v15 = 1;
            break;
          case 0xc:
          case 0xd:
            v15 = 1;
label_3bd6:
            v6 = v12;
            if (v12 <= v24)
              v6 = v24;
            v12 = 0;
            v24 = v6;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v14 * 2);
            if (v1 & 0x4000) { // branch-flip
              v12 += 1;
              if (v1 & 0x2000) goto label_3bf8;
              v7 = (unsigned long)v18;
              v15 = 1;
            }
            else {
              v15 = 1;
            }
            goto label_3ba8;
          case 0x20:
            v15 = 1;
label_3c06:
            v12 += 1;
          
        }
label_3be9:
        v23 += v7;
        v7 = 0;
label_3ba8:
        v17 = &v17[v15];
        v15 = -v15;
        v16 += 1;
label_3bb2:
        v19 += v15;
      } while (v19);
      goto label_3ae0;
    }
    v21 = 1;
label_3f68:
    if (v12 <= v24)
      v12 = v24;
    v23 = v7 + v23;
    v7 = v9[0];
  }
  else {
    v12 = 0;
    v19 = 0;
    v7 = 0;
    v16 = v12;
    while (v5 = (long)sub_7490(a0,v8,0x4000), v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v21 = 0;
        v5 = (unsigned long)sub_6d70(0,3,v26);
        v2 = v5;
        v5 = (int *)__errno_location();
        error(0,*(int *)v5,"%s",v2);
        goto label_3fc8;
      }
      v9[0] += (long)v5;
      v17 = &v8[(long)v5];
      v20 = v8;
      do {
        v14 = *v20;
        v20 = &v20[1];
        switch(v14) {
          case 9:
            v19 = (v19 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v11 += 1;
            goto label_3ee6;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_3ee6:
            if (v12 < v19)
              v12 = v19;
            v19 = 0;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v14 * 2);
            if (v1 & 0x4000) {
              v19 += 1;
              if (v1 & 0x2000) goto label_3ef0;
              v4 = btowc((unsigned long)v14);
              if (dat_d118) // branch-flip
                v7 = (unsigned long)dat_d118;
              else {
                if (((v4 == 0xa0) || (v4 == 0x2007)) || ((v4 == 0x202f || (v4 == 0x2060)))) goto label_3ef0;
                v7 = 1;
              }
            }
            goto label_3ed0;
          case 0x20:
            v19 += 1;
          
        }
label_3ef0:
        v16 += v7;
        v7 = 0;
label_3ed0:
      } while (v17 != v20);
    }
    v21 = 1;
label_3fc8:
    if (v12 < v19)
      v12 = v19;
    v23 = v7 + v16;
    v16 = 0;
    v7 = v9[0];
  }
label_39f8:
  if (v25 < dat_d132)
    v16 = v7;
  sub_3430(v11,v23,v16,v7,v12,a1);
  dat_d150 += v23;
  dat_d158 += v11;
  dat_d148 += v16;
  dat_d140 += v9[0];
  if (dat_d138 < v12)
    dat_d138 = v12;
  v4 = v21;
  return v4;
}

// Function: sub_4230 @ 0x4230
void sub_4230(int a0)
{
  void *v1;
  char *v10; // stack - 0xb0
  char *v11; // stack - 0x88
  char *v12; // stack - 0x80
  char *v13; // stack - 0x78
  char *v14; // stack - 0x70
  char *v15; // stack - 0x68
  char *v16; // stack - 0x60
  unsigned long v17; // stack - 0x58
  unsigned long v18; // stack - 0x50
  unsigned long v19; // stack - 0x40
  unsigned long v2;
  char *v20 [4]; // stack - 0xa8
  int v3; // eax
  char *v4; // rax
  char **v5; // rbx
  char *v6; // stack - 0xb8
  char *v7;
  char *v8; // r13
  long v9; // fs_offset
  
  v2 = dat_d178;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
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
    v6 = "[";
    v7 = "[";
    v10 = "test invocation";
    v20[0] = "coreutils";
    v20[1] = "Multi-call invocation";
    v11 = "sha256sum";
    v20[2] = "sha224sum";
    v13 = "sha384sum";
    v20[3] = "sha2 utilities";
    v12 = "sha2 utilities";
    v14 = "sha2 utilities";
    v15 = "sha512sum";
    v16 = "sha2 utilities";
    v17 = 0;
    v18 = 0;
    do {
      if (!strcmp("wc",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "wc";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "wc";
    if (!strcmp("wc","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "wc")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}

// Function: sub_4580 @ 0x4580
void sub_4580(unsigned long a0) // return-dupe
{
  unsigned long *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  *v1 = 0;
  v1[4] = a0;
  v1[5] = a0;
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
long sub_45f0(long *a0,int *a1) // early-return x2
{
  long *v1;
  long v2;
  void *v3;
  
  if (!*a0) {
    v1 = (long *)a0[5];
    v2 = *v1;
    if (!v2) {
      *a1 = 2;
      return 0;
    }
    *a1 = 1;
    a0[5] = (long)&v1[1];
    return v2;
  }
  if (0 <= (long)getdelim(&a0[2],&a0[3],0)) {
    *a1 = 1;
    a0[1] = a0[1] + 1;
    return a0[2];
  }
  v3 = (void *)*a0;
  *a1 = (-(unsigned int)(feof(v3) == 0) & 2) + 2;
  return 0;
}

// Function: sub_4690 @ 0x4690
long sub_4690(long *a0)
{
  if (*a0)
    return a0[1];
  return a0[5] - a0[4] >> 3;
}

// Function: sub_46b0 @ 0x46b0
void sub_46b0(long *a0)
{
  if (*a0)
    free((void *)a0[2]);
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
  unsigned long v4; // rax
  
  v1 = sub_8150(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_d168) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_d170) { // branch-flip
        v4 = sub_6c50(dat_d170);
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
void sub_47b0(void *a0,unsigned int a1)
{
  if (a0) {
    posix_fadvise(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_47e0 @ 0x47e0
unsigned long sub_47e0(void *a0) // early-return
{
  int v1;
  int *v2; // rax
  
  if (0 <= fileno(a0)) {
    if (__freading(a0)) {
      v1 = fileno(a0);
      if (lseek(v1,0,1) == -1) {
        return fclose(a0); // tail-call, return-dupe
      }
    }
    if (sub_4870(a0)) {
      v2 = __errno_location();
      v1 = *v2;
      if (!v1)
        return CONCAT44(dat_4,fclose(a0));
      *v2 = v1;
      return 0xffffffff;
    }
  }
  return fclose(a0);
}

// Function: sub_4870 @ 0x4870
void sub_4870(unsigned int *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*a0 & 0x100)) {
      sub_48c0(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_48c0 @ 0x48c0
unsigned long sub_48c0(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  long v1; // rax
  
  if (((*(long *)&a0[4] == *(long *)&a0[2]) && (*(long *)&a0[10] == *(long *)&a0[8])) && (!*(long *)&a0[0x12])) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 != -1) {
      *a0 = *a0 & 0xffffffef;
      *(long *)&a0[0x24] = v1;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko(a0); // tail-call
}

// Function: sub_4940 @ 0x4940
char * sub_4940(unsigned long a0,long a1)
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

// Function: sub_49a0 @ 0x49a0
int sub_49a0(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_49d0 @ 0x49d0
void sub_49d0(long *a0,long *a1) // return-dupe
{
  long v1;
  long *v2;
  
  v2 = (long *)*a1;
  if (v2 == &a1[3])
    v2 = memcpy(&a0[3],&a1[3],a1[1]);
  v1 = a1[1];
  *a0 = (long)v2;
  a0[1] = v1;
  v1 = a1[2];
  *(char *)&a0[2] = (char)v1;
  if (!(char)v1)
    return;
  *(unsigned int *)((long)a0 + 0x14) = *(unsigned int *)((long)a1 + 0x14);
}

// Function: sub_4a30 @ 0x4a30
unsigned int sub_4a30(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x99e0) >> (a0 & 0x1f) & 1;
}

// Function: sub_4a50 @ 0x4a50
unsigned long sub_4a50(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_81c0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
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
  unsigned long v15; // stack - 0x48
  unsigned int v16; // stack - 0x4c
  unsigned char v17;
  unsigned int v18;
  unsigned long v19;
  bool v2;
  unsigned char v20;
  bool v21;
  unsigned long v22;
  bool v23;
  unsigned long v24;
  bool v25;
  bool v26;
  bool v27;
  bool v28;
  bool v29;
  bool v3;
  long v30; // stack - 0xe0
  unsigned long v31; // stack - 0xd8
  char *v32; // stack - 0xb0
  unsigned long v33; // stack - 0xa8
  unsigned long v34; // stack - 0xa0
  unsigned int v35; // stack - 0x80
  char *v36; // stack - 0x70
  char *v37; // stack - 0x68
  bool v4;
  bool v5;
  long v6;
  long v7;
  unsigned char v8;
  unsigned int v9; // eax
  
  v30 = a6;
  v37 = a7;
  v36 = a8;
  v35 = a5;
label_4f00:
  v10 = (unsigned long)__ctype_get_mb_cur_max();
  v34 = (unsigned long)v10;
  v18 = v35 & 2;
  switch(a4) {
    case 0:
      v26 = 0;
      v2 = 0;
      v13 = 0;
      v31 = 0;
      v32 = NULL;
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
      v26 = 1;
      v2 = 0;
      v13 = 0;
      v31 = 1;
      v32 = "\'";
      break;
    case 3:
      v2 = 1;
label_5360:
      v26 = 1;
      v13 = 0;
      a4 = 2;
      v31 = 1;
      v32 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_54c1;
      }
      goto label_53a6;
    case 5:
      if (!v18) goto label_5301;
      v26 = 1;
      v2 = 1;
      v13 = 0;
      v31 = 1;
      v32 = "\"";
      break;
    case 6:
      v26 = 1;
      v13 = 0;
      v2 = 1;
      v31 = 1;
      a4 = 5;
      v32 = "\"";
      break;
    case 7:
      v26 = 0;
      v2 = 1;
      v13 = 0;
      v31 = 0;
      v32 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v10 = (char *)dcgettext(NULL,"`",5);
        v37 = (char *)v10;
        if (v10 == (long *)"`") {
          v10 = (char *)sub_4db0("`",a4);
          v37 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v36 = (char *)v10;
        if (v10 == (long *)"\'") {
          v10 = (char *)sub_4db0("\'",a4);
          v36 = (char *)v10;
        }
      }
      v13 = 0;
      if (!v18) {
        v11 = *v37;
        while (v11) {
          if (v13 < a1)
            a0[v13] = v11;
          v13 += 1;
          v11 = v37[v13];
        }
      }
      v26 = v18 != 0;
      v2 = 1;
      v10 = (unsigned long)strlen(v36);
      v32 = v36;
      v31 = (unsigned long)v10;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v21 = 0;
  v33 = 0;
  v14 = a1;
  v3 = 0;
label_4feb:
  do {
label_4ffe:
    v24 = 0;
    a1 = v14;
label_5008:
    v14 = v33;
    v27 = a3 != v24;
    if (a3 == 0xffffffffffffffff)
      v27 = a2[v24] != '\0';
    if (v27) {
      v12 = (unsigned char *)&a2[v24];
      v23 = (bool)(a4 != 2 & v2);
      v27 = 1;
      v14 = v13;
      v28 = v2;
      v29 = v2;
      if (v23) {
        v25 = 1;
        if (v31) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v31)) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          if ((a3 < v24 + v31) || (v9 = memcmp(v12,v32,v31), v9)) goto label_5970;
          if (!v26) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v22 = v19;
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
            v5 = v23;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_5243;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_5128;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_5a14;
              goto label_5958;
            }
            if (v8 == 0x7d) goto label_5ae0;
            if ((char)v8 <= '}') {
              v22 = 0x7b;
              if (v8 == 0x7b) goto label_593a;
              v22 = 0x7c;
              if (v8 == 0x7c) goto label_5958;
              goto label_5243;
            }
            if (v8 != 0x7e) goto label_5243;
label_5a94:
            if (!v24) {
              v8 = a4 == 2;
              v22 = 0x7e;
              v27 = v3;
              goto label_52b9;
            }
            v22 = 0x7e;
            goto label_5aa2;
          }
          goto label_522c;
        }
label_5970:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v22 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v23 = 0;
                if (v8 == 0x7e) goto label_5a94;
                goto label_5240;
              }
              v22 = 0x7b;
              v23 = 0;
              if (v8 != 0x7b) {
                v22 = 0x7c;
                v23 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_5243;
                goto label_5958;
              }
            }
            else {
              v22 = 0x7d;
              v23 = 0;
            }
label_593a:
            if (a3 == 0xffffffffffffffff) goto label_5aef;
label_5948:
            v5 = v23;
            if (a3 == 1) goto label_55d5;
label_5958:
            v8 = a4 == 2;
            v27 = v3;
            v25 = 0;
            v23 = v5;
            goto label_52b9;
          }
          if (v8 == 0x40) goto label_5240;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v23 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_5128;
          v23 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_5958;
label_5a14:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v26)) && (v31)) goto label_5a3a;
            v17 = 0x5c;
            goto label_5462;
          }
label_5243:
          v17 = (unsigned char)v19;
          if (v34 != 1) goto label_57c3;
label_5250:
          v10 = __ctype_b_loc();
          v22 = 1;
          v27 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v28 = (bool)(v27 & v2);
          v27 = !v27;
          goto label_529e;
        }
        switch(v8) {
          case 0:
            goto label_555e;
          default:
label_5240:
            v23 = 0;
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
            v23 = 0;
label_55f3:
            v22 = 9;
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
            if (v26) {
              v2 = 1;
              goto label_548f;
            }
            v25 = 0;
            goto label_51ae;
          case 0xd:
label_545d:
            v17 = 0x72;
            break;
          case 0x20:
            v23 = 0;
label_562d:
            v22 = 0x20;
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
            v23 = 0;
            v27 = 0;
            goto label_51f9;
          case 0x23:
            v22 = 0x23;
            v23 = 0;
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
            v23 = 0;
label_5128:
            v8 = 0;
            goto label_5138;
          case 0x27:
            v23 = 0;
            goto label_5442;
          case 0x3f:
            v23 = 0;
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
            v22 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_5948;
label_5aef:
            v5 = v23;
            if (a2[1]) goto label_5958;
            goto label_55d5;
          }
          if ((char)v17 <= '}') {
            v22 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_51f6;
              goto label_5243;
            }
            goto label_593a;
          }
          v22 = 0x7e;
          if (v17 == 0x7e) goto label_55d5;
          if (v34 == 1) goto label_5250;
label_57c3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v22 = 0;
          do {
            v19 = v24 + v22;
            v10 = (long)sub_4a50(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v25 = v27;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_5d2b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_60af;
              goto label_60a5;
            }
            if (((a4 == 2) && (v26)) && (v10 != (long *)0x1)) {
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
              v27 = 0;
            v22 += v7;
            v9 = mbsinit(&v15);
            v25 = v27;
          } while (!v9);
          v28 = (bool)((v25 ^ 1U) & v2);
          goto label_5d2b;
        }
        if (v17 == 0x40) goto label_5243;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_52b0;
        if ((unsigned long)v10 & 0xa4000000) goto label_51f6;
        if (v17 != 0x5c) goto label_5243;
        if (a4 == 2) {
          if (!v26) goto label_5a3a;
          goto label_5223;
        }
        if (!(bool)(v2 & v26 & v31 != 0)) {
          v22 = 0x5c;
          v17 = 0x5c;
          v23 = 0;
          goto label_5600;
        }
label_5a3a:
        v24 += 1;
        v25 = 0;
        v17 = 0x5c;
        v8 = v21;
        goto label_5a50;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v23 = 0;
            v22 = 0;
            if (v35 & 1) goto label_56d2;
            goto label_560b;
          }
label_555e:
          if (!v26) {
            v23 = 0;
label_556c:
            v28 = a4 == 2;
            v8 = v21 ^ 1;
            v8 &= v28;
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
                v25 = 0;
                v17 = 0x30;
                v21 = (bool)v8;
                goto label_52e3;
              }
label_5e75:
              a0[v14] = 0x5c;
              v13 = v14;
              v21 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v21;
              goto label_5e75;
            }
            v14 = v13 + 1;
            if (a4 != 2) {
              v22 = 0x30;
              if ((v24 + 1 < a3) && (v8 = a2[v24 + 1] - 0x30, v8 <= 9)) {
                if (v14 < a1)
                  a0[v14] = 0x30;
                if (v13 + 2 < a1)
                  a0[v13 + 2] = 0x30;
                v14 = v13 + 3;
                v22 = 0x30;
              }
label_5eac:
              v17 = (unsigned char)v22;
              v25 = 0;
              v13 = v14;
              v8 = v27;
              if (!v2 || v28) goto label_52e3;
              goto label_5138;
            }
            v17 = 0x30;
            v25 = 0;
            v13 = v14;
            v8 = v27;
            goto label_52e3;
          }
          if (a4 != 2) goto label_522c;
          goto label_549b;
        default:
          goto label_5243;
        case 7:
          v22 = 7;
          v17 = 0x61;
          break;
        case 8:
          v22 = 8;
          v17 = 0x62;
          break;
        case 9:
          v22 = 9;
          v17 = 0x74;
          goto label_568f;
        case 10:
          v22 = 10;
          v17 = 0x6e;
          goto label_568f;
        case 0xb:
          v22 = 0xb;
          v17 = 0x76;
          break;
        case 0xc:
          v22 = 0xc;
          v17 = 0x66;
          break;
        case 0xd:
          v22 = 0xd;
          v17 = 0x72;
label_568f:
          v8 = a4 == 2 & v26;
          v23 = 0;
          if (!(bool)v8) goto label_5600;
label_5211:
          a4 = 2;
label_5223:
          if (!v29) goto label_522c;
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
          v27 = 0;
label_51f9:
          v22 = v19;
          v25 = v27;
          if ((a4 == 2) && (v26)) goto label_5211;
          goto label_52b7;
        case 0x23:
label_55d0:
          v22 = 0x23;
label_55d5:
          if (!v24) {
            v19 = v22;
            goto label_51f9;
          }
label_5aa2:
          v8 = a4 == 2;
          v27 = v3;
          v25 = 0;
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
          v25 = v27;
          if (a4 != 2) {
            v8 = 0;
            v22 = 0x27;
            goto label_52b9;
          }
          if (!v26) {
            if (a1) { // branch-flip
              v14 = 0;
              v22 = a1;
              if (v33) goto label_5ecb;
            }
            else {
label_5ecb:
              if (v13 < a1)
                a0[v13] = 0x27;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x5c;
              v14 = a1;
              v22 = v33;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x27;
            }
            v13 += 3;
            v8 = 0;
            v17 = 0x27;
            a1 = v14;
            v33 = v22;
            v21 = 0;
            v3 = v27;
            goto label_52e3;
          }
          goto label_5223;
        case 0x3f:
label_5401:
          if (a4 == 2) {
            if (v26) goto label_5223;
            v8 = 0;
            v17 = 0x3f;
            v25 = 0;
            goto label_52e3;
          }
          if ((((a4 == 5) && (v35 & 4)) && (v19 = v24 + 2, v19 < a3)) && (a2[v24 + 1] == '?')) {
            v8 = a2[v19];
            v22 = (unsigned long)v8;
            if (v8 <= 0x3e) {
              v10 = (unsigned long)(0x7000a38200000000 >> (v8 & 0x3f));
              v1 = (unsigned long)v10 & 1;
              v8 = (unsigned char)v10 & 1;
              if (!v1) {
                v22 = 0x3f;
                v27 = v3;
                v25 = 0;
                goto label_52b9;
              }
              if (v26) goto label_522c;
              if (v13 < a1)
                a0[v13] = 0x3f;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x22;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x22;
              if (v13 + 3 < a1)
                a0[v13 + 3] = 0x3f;
              v14 = v13 + 4;
              v28 = 0;
              v27 = 0;
              v24 = v19;
              goto label_5eac;
            }
          }
          v8 = 0;
          v22 = 0x3f;
          v27 = v3;
          v25 = 0;
          goto label_52b9;
        
      }
label_5600:
      if (!v2) {
label_560b:
        v17 = (unsigned char)v22;
        v25 = 0;
        v8 = 0;
        v27 = v3;
        if (!v26) goto label_52e0;
        goto label_5138;
      }
label_5462:
      v25 = 0;
      goto label_5473;
    }
    if (!(bool)(v13 == 0 & v26 & a4 == 2)) {
      v8 = a4 == 2 & (v26 ^ 1U);
      v26 = (bool)(v26 ^ 1U);
      if ((!(bool)v8) || (v26 = (bool)v8, !v3)) {
label_616b:
        v10 = (long *)v13;
        if (((v32) && (v26)) && (v11 = *v32, v11)) {
          do {
            if (v10 < a1)
              a0[(long)v10] = v11;
            v10 = (unsigned long)((long)v10 + 1);
            v11 = v32[(long)v10 - v13];
          } while (v11);
        }
        if (v10 < a1)
          a0[(long)v10] = 0;
        return v10;
      }
      if (!v4) {
        v8 = !a1 && v33;
        v26 = v3;
        if (a1 || !v33) goto label_616b;
        v24 = v33;
        goto label_5bfb;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v31 = 1;
      v34 = (unsigned long)v10;
      v32 = "\"";
      if (!(v35 & 2)) goto label_621b;
      v3 = 0;
      v21 = 0;
      v33 = 0;
      v2 = v4;
      v26 = v4;
      goto label_4ffe;
    }
label_58fd:
    if (v2) {
label_549b:
      v2 = 1;
      v35 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 0;
      v34 = (unsigned long)v10;
    }
    else {
      v2 = 0;
      v35 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 0;
      v34 = (unsigned long)v10;
    }
label_54c1:
    v26 = 0;
    a4 = 2;
    v13 = 1;
    v32 = "\'";
    v31 = 1;
    v21 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v8 = 0;
      v24 = 0;
      v33 = a1;
label_5bfb:
      v13 = 1;
      *a0 = 0x27;
      a4 = 2;
      v26 = 0;
      v31 = 1;
      v32 = "\'";
      v14 = v33;
      v33 = v24;
      v3 = (bool)v8;
    }
    else {
      v33 = 0;
      v14 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v22 += 1;
    v10 = (unsigned long)(v24 + v22);
    if (a3 <= v10) break;
label_60a5:
    if (!a2[(long)v10]) break;
  }
label_60af:
  v25 = 0;
label_5d2b:
  v19 = (unsigned long)v17;
  v27 = v25;
  if (2 <= v22) {
label_5d35:
    v20 = 0;
    v22 += v24;
    v14 = v24;
    do {
      v17 = (unsigned char)v19;
      if (v28) {
        v29 = a4 == 2;
        if (v26) goto label_5223;
        v8 = v29 & (v21 ^ 1U);
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x24;
          if (v13 + 2 < a1)
            a0[v13 + 2] = 0x27;
          v13 += 3;
          v21 = 1;
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
        v24 = v14 + 1;
        v13 += 3;
        v17 = (v17 & 7) + 0x30;
        if (v22 <= v24) goto label_51c1;
        v20 = 1;
      }
      else {
        v8 = v20 ^ 1;
        v8 &= v21;
        if (v23) {
          if (v13 < a1)
            a0[v13] = 0x5c;
          v13 += 1;
        }
        v24 = v14 + 1;
        if (v22 <= v24) {
          goto label_5a50;
        }
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x27;
          v13 += 2;
          v23 = 0;
          v21 = 0;
        }
        else {
          v23 = 0;
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
  if (v28) {
    v25 = 0;
    v28 = v2;
    goto label_5d35;
  }
label_52b0:
  v22 = v19;
  v25 = v27;
label_52b7:
  v8 = a4 == 2;
  v27 = v3;
label_52b9:
  v17 = (unsigned char)v22;
  v28 = (bool)v8 == 0;
  v3 = v27;
  v8 = 0;
  if ((v28 && v2) || (v8 = 0, v26)) {
label_5138:
    v17 = (unsigned char)v22;
    v13 = v14;
    if (!v30) goto label_52e3;
    if (!(*(unsigned int *)(v30 + (v22 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_52e3;
  }
  else {
label_52e0:
    v8 = 0;
    v3 = v27;
label_52e3:
    if (!v23) {
      v8 ^= 1;
      v24 += 1;
      v8 &= v21;
label_5a50:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v21 = 0;
        v13 += 2;
      }
      goto label_51c1;
    }
  }
label_5473:
  if (v26) {
label_548f:
    if ((bool)(v2 & a4 == 2)) goto label_549b;
label_522c:
    v35 &= 0xfffffffd;
    v30 = 0;
    goto label_4f00;
  }
  v8 = v21 ^ 1;
  v8 &= a4 == 2;
  if ((bool)v8) {
    if (v13 < a1)
      a0[v13] = 0x27;
    if (v13 + 1 < a1)
      a0[v13 + 1] = 0x24;
    if (v13 + 2 < a1)
      a0[v13 + 2] = 0x27;
    v13 += 3;
    v21 = 1;
  }
label_51ae:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v24 += 1;
label_51c1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v25)
    v4 = 0;
  goto label_5008;
label_56d2:
  v24 += 1;
  goto label_5008;
label_621b:
  a1 = v33;
label_5301:
  v26 = 0;
  a4 = 5;
  v13 = 1;
  v32 = "\"";
  v2 = 1;
  v31 = 1;
  v21 = 0;
  v3 = 0;
  v4 = 1;
  v33 = 0;
  v14 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_4feb;
}

// Function: sub_6320 @ 0x6320
void * sub_6320(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
{
  unsigned long *v1;
  long v10; // stack - 0x48
  int *v11; // stack - 0x68
  int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned long v5;
  unsigned long *v6;
  unsigned long v7;
  void *v8;
  long v9; // rdx
  
  v11 = __errno_location();
  v2 = *v11;
  if (a0 <= 0x7ffffffe) {
    v6 = dat_d080;
    if (dat_d078 <= (int)a0) {
      v10 = (long)dat_d078;
      v9 = (long)(int)((a0 - dat_d078) + 1);
      if (dat_d080 != (unsigned long *)0xd090) // branch-flip
        v6 = (unsigned long *)sub_7ef0(dat_d080,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_7ef0(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_d090;
        v6[1] = dat_d098;
      }
      dat_d080 = v6;
      memset(&v6[(long)dat_d078 * 2],0,(v10 - dat_d078) * 0x10);
      dat_d078 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_4e90(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0xd180)
        free(v8);
      v8 = (void *)sub_7c50(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_4e90(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_6510 @ 0x6510
void sub_6510(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0xd280;
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
unsigned int sub_65d0(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0xd280;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
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
void sub_6630(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0xd280;
  v2 = __errno_location();
  v1 = *v2;
  sub_4e90(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_66b0 @ 0x66b0
unsigned long sub_66b0(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0xd280;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_4e90(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_7c50(v3 + 1);
  sub_4e90(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_67a0 @ 0x67a0
void sub_67a0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_66b0(a0,a1,0,a2); // tail-call
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
void sub_6850(unsigned long a0,unsigned long a1)
{
  sub_6320(a0,a1,0xffffffffffffffff,0xd280); // tail-call
}

// Function: sub_6870 @ 0x6870
void sub_6870(void)
{
  sub_6320(); // tail-call
}

// Function: sub_6880 @ 0x6880
void sub_6880(unsigned long a0)
{
  sub_6320(0,a0,0xffffffffffffffff,0xd280); // tail-call
}

// Function: sub_68a0 @ 0x68a0
void sub_68a0(unsigned long a0,unsigned long a1)
{
  sub_6320(0,a0,a1,0xd280); // tail-call
}

// Function: sub_68c0 @ 0x68c0
void sub_68c0(unsigned long a0,int a1,unsigned long a2)
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
void sub_6960(unsigned long a0,int a1,unsigned long a2,unsigned long a3)
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
void sub_69f0(int a0,unsigned long a1)
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
void sub_6a80(int a0,unsigned long a1,unsigned long a2)
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
void sub_6b10(unsigned long a0,unsigned long a1,unsigned char a2)
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
void sub_6bb0(unsigned long a0,unsigned char a1)
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
void sub_6c50(unsigned long a0)
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
void sub_6ce0(unsigned long a0,unsigned long a1)
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
void sub_6d70(unsigned long a0,int a1,unsigned long a2)
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
void sub_6e10(unsigned long a0,long a1,long a2,unsigned long a3)
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
void sub_6eb0(unsigned int a0,long a1,long a2,unsigned long a3,unsigned long a4)
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
void sub_6f50(long a0,long a1,unsigned long a2)
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
void sub_6ff0(long a0,long a1,unsigned long a2,unsigned long a3)
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
void sub_7090(void)
{
  sub_6320(); // tail-call
}

// Function: sub_70a0 @ 0x70a0
void sub_70a0(unsigned long a0,unsigned long a1)
{
  sub_6320(0,a0,a1,0xd040); // tail-call
}

// Function: sub_70c0 @ 0x70c0
void sub_70c0(unsigned long a0,unsigned long a1)
{
  sub_6320(a0,a1,0xffffffffffffffff,0xd040); // tail-call
}

// Function: sub_70e0 @ 0x70e0
void sub_70e0(unsigned long a0)
{
  sub_6320(0,a0,0xffffffffffffffff,0xd040); // tail-call
}

// Function: sub_7100 @ 0x7100
void sub_7100(long *a0)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  unsigned long v4;
  long *v5;
  
  v1 = a0[6];
  v2 = a0[5];
  if (v1 == v2)
    *(unsigned char *)&a0[0xd] = *(unsigned char *)&a0[0xd] | 2;
  v3 = v1 + a0[9] & ~a0[9];
  v4 = a0[7];
  if (v3 - a0[4] <= (unsigned long)(a0[7] - a0[4]))
    v4 = v3;
  a0[6] = v4;
  a0[5] = v4;
  v5 = (long *)a0[0x11];
  if ((unsigned long)(a0[0x12] - (long)v5) <= 7) {
    _obstack_newchunk(&a0[0xe],8);
    v5 = (long *)a0[0x11];
  }
  *v5 = v2;
  v5 = (long *)a0[0x1c];
  a0[0x11] = a0[0x11] + 8;
  if ((unsigned long)(a0[0x1d] - (long)v5) <= 7) {
    _obstack_newchunk(&a0[0x19],8);
    v5 = (long *)a0[0x1c];
  }
  *v5 = (v1 - v2) + -1;
  a0[0x1c] = a0[0x1c] + 8;
  *a0 = *a0 + 1;
}

// Function: sub_71f0 @ 0x71f0
void sub_71f0(unsigned long *a0)
{
  unsigned long v1;
  unsigned long v2;
  
  v2 = dat_cfe0;
  v1 = dat_cfc0;
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin(&a0[3],0,0,dat_cfe0,dat_cfc0);
  _obstack_begin(&a0[0xe],0,0,v2,v1);
  _obstack_begin(&a0[0x19],0,0,v2,v1); // tail-call
}

// Function: sub_7270 @ 0x7270
void sub_7270(long a0)
{
  _obstack_free(a0 + 0x18,0);
  _obstack_free(a0 + 0x70,0);
  _obstack_free(a0 + 200,0); // tail-call
}

// Function: sub_72a0 @ 0x72a0
unsigned long sub_72a0(void *a0,long a1)
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
    v2 = *(char **)(a1 + 0x30);
    if (v1 == -1) {
      if (v2 != *(char **)(a1 + 0x28)) {
        if (v2 == *(char **)(a1 + 0x38)) {
          _obstack_newchunk(a1 + 0x18,1);
          v2 = *(char **)(a1 + 0x30);
        }
        *(char **)(a1 + 0x30) = &v2[1];
        *v2 = 0;
        sub_7100(a1);
      }
      v5 = *(unsigned long **)(a1 + 0x88);
      if ((unsigned long)(*(long *)(a1 + 0x90) - (long)v5) <= 7) {
        _obstack_newchunk(a1 + 0x70,8);
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
    if (*(char **)(a1 + 0x38) != v2) goto label_72c0;
    _obstack_newchunk(a1 + 0x18,1);
    v2 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v2[1];
    *v2 = (char)v1;
  } while (v1);
  goto label_7310;
label_72c0:
  *(char **)(a1 + 0x30) = &v2[1];
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
long sub_7500(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
void sub_79a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  unsigned int v1;
  long *v2;
  long v3;
  long v4 [11];
  long v5; // r9
  
  v5 = 0;
  do {
    v1 = *a4;
    if (0x30 <= v1) { // branch-flip
      v2 = *(long **)&a4[2];
      *(long **)&a4[2] = &v2[1];
      v3 = *v2;
      v4[v5] = v3;
    }
    else {
      *a4 = v1 + 8;
      v3 = *(long *)((unsigned long)v1 + *(long *)&a4[4]);
      v4[v5] = v3;
    }
  } while ((v3) && (v5 = v5 + 1, v5 != 10));
  sub_7500(a0,a1,a2,a3);
}

// Function: sub_7a40 @ 0x7a40
void sub_7a40(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  unsigned long v3;
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
  if (SEXT816((long)v3) != SEXT816(v2) * SEXT816(a4)) // branch-flip
    v4 = 0x7fffffffffffffff;
  else {
    if (0x80 <= (long)v3) goto label_7fae;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_7fae:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816((long)v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
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
unsigned long sub_8150(unsigned int *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *a0;
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
long * sub_82e0(long *a0,long a1,long a2) // ternary
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
  a0[6] = v3;
  if (!a1)
    a1 = 0xfe0;
  *a0 = a1;
  v1 = (*(unsigned char *)&a0[10] & 1) ? (long *)(*(void *)a0[7])(a0[9]) : (long *)(*(void *)a0[7])(a1); // branch-flip
  a0[1] = (long)v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0[2] = v2;
    a0[3] = v2;
    v3 = *a0;
    *v1 = v3 + (long)v1;
    a0[4] = v3 + (long)v1;
    v1[1] = 0;
    v1 = (long *)0x1;
    *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] & 0xf9;
    return v1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_begin @ 0x8380
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  sub_82e0(); // tail-call
}

// Function: _obstack_begin_1 @ 0x83a0
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  sub_82e0(); // tail-call
}

// Function: _obstack_newchunk @ 0x83c0
void _obstack_newchunk(unsigned long *a0,unsigned long a1) // ternary
{
  unsigned long v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4; // rax
  unsigned long v5;
  unsigned long v6; // r13
  void *v7; // r14
  
  v6 = a0[3] - a0[2];
  v2 = a0[1];
  v5 = a1 + v6 + a0[6];
  v1 = v5 + 100 + (v6 >> 3);
  if (v5 < *a0)
    v5 = *a0;
  if (v5 <= v1)
    v5 = v1;
  if ((!CARRY8(a1,v6)) && (!CARRY8(a1 + v6,a0[6]))) {
    v4 = (a0[10] & 1) ? (unsigned long *)(*(void *)a0[7])(a0[9]) : (unsigned long *)(*(void *)a0[7])(v5); // branch-flip
    if (v4) {
      a0[1] = (unsigned long)v4;
      v3 = (void *)a0[2];
      v4[1] = v2;
      a0[4] = (long)v4 + v5;
      *v4 = (long)v4 + v5;
      v7 = (void *)((long)v4 + a0[6] + 0x10 & ~a0[6]);
      memcpy(v7,v3,v6);
      v1 = a0[10];
      if ((!(v1 & 2)) && (a0[2] == (~a0[6] & v2 + 0x10 + a0[6]))) {
        v4[1] = *(unsigned long *)(v2 + 8);
        if (v1 & 1) // branch-flip
          (*(void *)a0[8])(a0[9],v2);
        else {
          (*(void *)a0[8])(v2);
        }
      }
      a0[2] = (unsigned long)v7;
      a0[3] = (long)v7 + v6;
      *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x84f0
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

// Function: _obstack_free @ 0x8530
void _obstack_free(long a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)(a0 + 8);
  if (*(unsigned long **)(a0 + 8)) {
    while( true ) {
      if ((v2 < a1) && (v1 = (unsigned long *)*v2, a1 <= v1)) {
        *(unsigned long **)(a0 + 0x18) = a1;
        *(unsigned long **)(a0 + 0x10) = a1;
        *(unsigned long **)(a0 + 0x20) = v1;
        *(unsigned long **)(a0 + 8) = v2;
        return;
      }
      v1 = (unsigned long *)v2[1];
      if (*(unsigned char *)(a0 + 0x50) & 1) { // branch-flip
        (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48));
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
      else {
        (**(void **)(a0 + 0x40))(v2);
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
      if (!v1) break;
      v2 = v1;
    }
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x85b0
long _obstack_memory_used(long a0)
{
  long *v1;
  long v2;
  
  v2 = 0;
  for (v1 = *(long **)(a0 + 8); v1; v1 = (long *)v1[1]) {
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

// Function: sub_8690 @ 0x8690
void sub_8690(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_86a0 @ 0x86a0
unsigned long sub_86a0(unsigned long a0,unsigned int a1,long *a2,long *a3)
{
  char v1 [32];
  int v10; // r8d
  int v11; // r9d
  bool v12; // r13b
  unsigned long v13; // r13
  unsigned long v14; // r13
  long v15;
  char v16 [32];
  char v17 [32];
  char v18 [64]; // zmm3
  unsigned long v2; // rax
  char (*v3)[32];
  unsigned long v4; // rax
  char (*v5)[32];
  long v6; // rbx
  char v7 [64];
  char v8 [16280];
  int v9; // r8d
  
  v12 = !a2 || !a3;
  v14 = CONCAT71((undefined7)((unsigned long)v13 >> 8),v12);
  if (v12)
    v14 = 0;
  else {
    v6 = 0;
    v15 = 0;
    while (v2 = sub_7490(a1,(char (*)[32])v7,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = sub_6d70(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v14 & 0xffffffff; // return-dupe
      }
      v6 += v2;
      if (0x40 <= v2) { // branch-flip
        v18 = ZEXT1664(0);
        v17 = SUB6432(v18,0);
        v5 = (char (*)[32])v7;
        do {
          v16 = vpcmpeqb_avx2(dat_a1c0,*v5);
          v1 = vpcmpeqb_avx2(dat_a1c0,v5[1]);
          v5 = &v5[2];
          v17 = vpsubb_avx2(v17,v16);
          v16 = vpsubb_avx2(SUB6432(v18,0),v1);
          v18 = ZEXT3264(v16);
          v3 = (char (*)[32])&v8[v2 - 0x40 & 0xffffffffffffffc0];
        } while (v5 != (char (*)[32])&v8[v2 - 0x40 & 0xffffffffffffffc0]);
      }
      else {
        v17 = SUB6432(ZEXT1664(0),0);
        v3 = (char (*)[32])v7;
        v16 = v17;
      }
      v17 = vpsadbw_avx2(v17,SUB6432(ZEXT1664(0),0));
      v16 = vpsadbw_avx2(v16,SUB6432(ZEXT1664(0),0));
      v9 = vpextrw_avx(SUB3216(v17,0),0);
      v10 = vpextrw_avx(SUB3216(v17,0x10),0);
      v11 = vpextrw_avx(SUB3216(v16,0),0);
      v15 += (long)(vpextrw_avx(SUB3216(v17,0),4) + v9 + v10 + vpextrw_avx(SUB3216(v17,0x10),4)) + (long)(vpextrw_avx(SUB3216(v16,0),4) + v11 + vpextrw_avx(SUB3216(v16,0x10),0) + vpextrw_avx(SUB3216(v16,0x10),4));
      for (; (char (*)[32])&(*(char (*)[32])v7)[v2] != v3; v3 = (char (*)[32])&(*v3)[1]) {
        v15 += (unsigned long)((*v3)[0] == '\n');
      }
    }
    v14 = 1;
    *a2 = v15;
    *a3 = v6;
  }
  return v14 & 0xffffffff;
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
