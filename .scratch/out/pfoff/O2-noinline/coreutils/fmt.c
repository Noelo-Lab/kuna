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

// Function: dcgettext @ 0x2440
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce80)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2450
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce88)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2460
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce90)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2470
void __stack_chk_fail(void)
{
  (*dat_ce98)(); // jump-as-call
}

// Function: getopt_long @ 0x2480
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_cea0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2490
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_cea8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x24a0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ceb0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x24b0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ceb8)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x24c0
void lseek(void)
{
  (*dat_cec0)(); // jump-as-call
}

// Function: __assert_fail @ 0x24d0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_cec8)(); // jump-as-call
}

// Function: fputs @ 0x24e0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_ced0)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x24f0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ced8)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x2500
void posix_fadvise(void)
{
  (*dat_cee0)(); // jump-as-call
}

// Function: memcmp @ 0x2510
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_cee8)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2520
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_cef0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2530
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cef8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2540
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf00)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2550
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_cf08)(); // jump-as-call
  return v1;
}

// Function: __memmove_chk @ 0x2560
void * __memmove_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf10)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2570
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf18)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x2580
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf20)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x2590
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf28)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x25a0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf30)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x25b0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf38)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x25c0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf40)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x25d0
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf48)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x25e0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_cf50)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x25f0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf58)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2600
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf60)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x2610
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf68)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2620
void error(int a0,int a1,char *a2,...)
{
  (*dat_cf70)(); // jump-as-call
}

// Function: fseeko @ 0x2630
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_cf78)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x2640
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_cf80)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x2650
void strtoumax(void)
{
  (*dat_cf88)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x2660
void __cxa_atexit(void)
{
  (*dat_cf90)(); // jump-as-call
}

// Function: exit @ 0x2670
void exit(int a0)
{
  (*dat_cf98)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2680
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_cfa0)(); // jump-as-call
  return v1;
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
undefined16 sub_26c0(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x26e0
int main(int argc,char **argv,char **envp)
{
  char *v1;
  char *v10; // stack - 0x48
  bool v2;
  int v3;
  unsigned int v4; // eax
  int v5; // eax
  FILE *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned int v9; // eax
  
  v7 = NULL;
  sub_4350(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_7c40(sub_40f0);
  dat_18130 = 0;
  dat_18131 = 0;
  dat_18132 = 0;
  dat_18133 = 0;
  dat_18120 = 0x4b;
  dat_18128 = "";
  dat_1811c = 0;
  dat_18118 = 0;
  dat_18114 = 0;
  if (((2 <= argc) && (v1 = argv[1], *v1 == '-')) && ((unsigned int)((int)v1[1] - 0x30U) <= 9)) {
    v7 = &v1[1];
    v1 = *argv;
    argc -= 1;
    argv = &argv[1];
    *argv = v1;
  }
  v10 = NULL;
  while (v3 = getopt_long(argc,argv,"0123456789cstuw:p:g:",(option *)0xca80,NULL), v3 != -1) {
    if (0x77 < v3) {
label_2a9d:
      if ((unsigned int)(v3 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),(char)v3);
      sub_3dc0(1); // no-return
    }
    if (v3 <= 0x62) {
      if (v3 == -0x83) {
        sub_6dd0(stdout,"fmt","GNU coreutils",dat_d010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v3 == -0x82)
        sub_3dc0(0); // no-return
      goto label_2a9d;
    }
    switch(v3) {
      case 99:
        dat_18133 = 1;
        break;
      default:
        goto label_2a9d;
      case 0x67:
        v10 = optarg;
        break;
      case 0x70:
        sub_2fd0(optarg);
        break;
      case 0x73:
        dat_18131 = 1;
        break;
      case 0x74:
        dat_18132 = 1;
        break;
      case 0x75:
        dat_18130 = 1;
        break;
      case 0x77:
        v7 = optarg;
      
    }
  }
  if (v7) { // branch-flip
    v3 = sub_7450(v7,0,0x9c4,"",dcgettext(NULL,"invalid width",5),0);
    dat_18120 = v3;
    if (v10) // branch-flip
      dat_18110 = sub_7450(v10,0,(long)v3,"",dcgettext(NULL,"invalid width",5),0);
    else {
label_29f3:
      dat_18110 = (dat_18120 * 0xbb) / 200;
    }
  }
  else {
    if (!v10) goto label_29f3;
    dat_18110 = sub_7450(v10,0,0x4b,"",dcgettext(NULL,"invalid width",5),0);
    dat_18120 = dat_18110 + 10;
  }
  if (optind != argc) { // branch-flip
    v2 = 0;
    v9 = 1;
    v3 = optind;
    if (argc <= optind) {
      return (v9 ^ 1) & 0xff; // return-dupe
    }
    do {
      v7 = argv[v3];
      if (strcmp(v7,"-")) { // branch-flip
        v6 = fopen(v7,"r");
        if (v6) // branch-flip
          v9 &= sub_3a30(v6,v7);
        else {
          v8 = sub_65c0(4,v7);
          v7 = dcgettext(NULL,"cannot open %s for reading",5);
          v9 = 0;
          error(0,*__errno_location(),v7,v8);
        }
      }
      else {
        v4 = sub_3a30(stdin,v7);
        v2 = 1;
        v9 &= v4;
      }
      v3 = optind + 1;
      optind = v3;
    } while (v3 < argc);
    if (!v2) {
      return (v9 ^ 1) & 0xff;
    }
  }
  else {
    v9 = sub_3a30(stdin,"-");
  }
  v3 = sub_41e0(stdin);
  if (v3) {
    v7 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v7);
    return v5;
  }
  return (v9 ^ 1) & 0xff;
}

// Function: sub_2b90 @ 0x2b90
void sub_2b90(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_cfc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_2bc0 @ 0x2bc0
void sub_2bc0(void)
{
  return;
}

// Function: _FINI_0 @ 0x2c30
void _FINI_0(void)
{
  if (!dat_d0c8) {
    if (dat_cfe0)
      __cxa_finalize(dat_d008);
    sub_2bc0();
    dat_d0c8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x2c70
void _INIT_0(void)
{
  return;
}

// Function: sub_2c80 @ 0x2c80
void sub_2c80(bool a0) // return-dupe x2
{
  if (dat_18131) {
    dat_d0ec = dat_d0f0;
    return;
  }
  if (dat_18133) {
    if (a0) {
      dat_d0ec = dat_1810c;
      return;
    }
    dat_d0ec = dat_d0f0;
    return;
  }
  if (!dat_18132) {
    dat_d0ec = dat_d0f0;
    return;
  }
  if ((a0) && (dat_1810c != dat_d0f0)) {
    dat_d0ec = dat_1810c;
    return;
  }
  if (dat_d0ec != dat_d0f0)
    return;
  dat_d0ec = -(unsigned int)(dat_d0f0 == 0) & 3;
}

// Function: sub_2d10 @ 0x2d10
bool sub_2d10(int a0)
{
  if ((dat_d0e4 == dat_d0f4) && (dat_d0e4 + dat_1811c <= dat_1810c))
    return a0 != 10 && a0 != -1;
  return 0;
}

// Function: sub_2d50 @ 0x2d50
long sub_2d50(unsigned long a0)
{
  unsigned char v1;
  char v2 [16];
  char v3 [16];
  long v4;
  
  v4 = 0x1324;
  if (0xd121 <= a0) {
    v1 = *(unsigned char *)(a0 - 0x18);
    if (v1 & 2) // branch-flip
      v4 = (unsigned long)(-(unsigned int)((v1 & 8) == 0) & 0x58804) + 0x960;
    else {
      v4 = 0xce4;
      if (((!(v1 & 4)) && (v4 = 0x1324, 0xd149 <= a0)) && (*(unsigned char *)(a0 - 0x40) & 8)) {
        v3._8_8_ = 0;
        v3._0_8_ = SUB168(SEXT816(40000),8);
        v4 = SUB168((v3._0_16_ << 0x40 | ZEXT816(40000)) / SEXT816((long)(*(int *)(a0 - 0x20) + 2)),0) + 0x1324;
      }
    }
  }
  if (*(unsigned char *)(a0 + 0x10) & 1) // branch-flip
    v4 -= 0x640;
  else if (*(unsigned char *)(a0 + 0x10) & 8) {
    v2._8_8_ = 0;
    v2._0_8_ = SUB168(SEXT816(0x57e4),8);
    return v4 + SUB168((v2._0_16_ << 0x40 | ZEXT816(0x57e4)) / SEXT816((long)(*(int *)(a0 + 8) + 2)),0);
  }
  return v4;
}

// Function: sub_2e00 @ 0x2e00
long sub_2e00(long a0,int a1) // early-return
{
  long v1; // rax
  long v2; // rdx
  
  if (dat_d100 == a0)
    return 0;
  v1 = (long)((dat_18110 - a1) * 10);
  v1 *= v1;
  if (dat_d100 == *(long *)(a0 + 0x20))
    return v1;
  v2 = (long)((a1 - *(int *)(a0 + 0x14)) * 10);
  return v1 + (v2 * v2 >> 1);
}

// Function: sub_2e50 @ 0x2e50
void sub_2e50(void)
{
  unsigned int v1;
  long v10;
  unsigned long v11; // r10
  unsigned long v12;
  unsigned long v13;
  int v2;
  int v3;
  int v4;
  int v5;
  long v6; // rax
  unsigned long v7;
  long v8; // rsi
  int v9;
  
  v5 = dat_18120;
  v4 = dat_d0f0;
  v3 = dat_d0ec;
  v2 = dat_d0e0;
  v11 = 0xd120;
  v1 = *(unsigned int *)(dat_d100 + 8);
  v13 = dat_d100 - 0x28;
  *(unsigned long *)(dat_d100 + 0x18) = 0;
  *(int *)(dat_d100 + 8) = dat_18120;
  v12 = dat_d100;
  if (0xd120 <= v13) {
    do {
      v10 = 0x7fffffffffffffff;
      v9 = v3;
      if (v13 == v11)
        v9 = v4;
      v9 += *(int *)(v13 + 8);
      v7 = v13;
      do {
        v7 += 0x28;
        v6 = sub_2e00(v7,v9);
        v6 += *(long *)(v7 + 0x18);
        if ((v13 == v11) && (1 <= v2)) {
          v8 = (long)((v9 - v2) * 10);
          v6 += v8 * v8 >> 1;
        }
        if (v6 < v10) {
          *(unsigned long *)(v13 + 0x20) = v7;
          *(int *)(v13 + 0x14) = v9;
          v10 = v6;
        }
      } while ((v12 != v7) && (v9 = v9 + *(int *)(v7 + 8) + *(int *)(v7 - 0x1c), v9 < v5));
      v7 = v13 - 0x28;
      *(long *)(v13 + 0x18) = sub_2d50(v13) + v10;
      v13 = v7;
    } while (v11 <= v7);
  }
  *(unsigned int *)(v12 + 8) = v1;
}

// Function: sub_2f70 @ 0x2f70
void sub_2f70(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1); // tail-call
}

// Function: sub_2fa0 @ 0x2fa0
void sub_2fa0(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_2fd0 @ 0x2fd0
void sub_2fd0(char *a0)
{
  char *v1; // rax
  int v2; // edi
  
  dat_18118 = 0;
  if (*a0 == ' ') {
    v2 = (int)a0;
    do {
      dat_18118 = (1U - v2) + (int)a0;
      a0 = &a0[1];
    } while (*a0 == ' ');
  }
  dat_18128 = a0;
  dat_1811c = (int)strlen(a0);
  v1 = &a0[dat_1811c];
  if (a0 < v1) {
    do {
      if (v1[-1] != ' ') break;
      v1 = &v1[-1];
    } while (v1 != a0);
  }
  *v1 = '\0';
  dat_18114 = (int)v1 - (int)a0;
}

// Function: sub_3050 @ 0x3050
void sub_3050(FILE *a0,int a1)
{
  int v1;
  
  do {
    if (a1 != 0x20) { // branch-flip
      if (a1 != 9)
        return;
      dat_d0f8 = 1;
      v1 = dat_1810c + 7;
      if (0 <= dat_1810c)
        v1 = dat_1810c;
      dat_1810c = (v1 >> 3) * 8 + 8;
    }
    else {
      dat_1810c += 1;
    }
    a1 = getc_unlocked(a0);
  } while( true );
}

// Function: sub_30b0 @ 0x30b0
void sub_30b0(FILE *a0) // return-dupe
{
  unsigned char v1;
  unsigned int v2;
  unsigned char *v3;
  
  dat_1810c = 0;
  v2 = sub_3050(a0,getc_unlocked(a0));
  if (!dat_18114) {
    if (dat_1810c <= dat_18118) {
      dat_d0e4 = dat_1810c;
      return;
    }
    dat_d0e4 = dat_18118;
    return;
  }
  v1 = *dat_18128;
  dat_d0e4 = dat_1810c;
  v3 = dat_18128;
  while( true ) {
    if (!v1) {
      sub_3050(a0,v2); // tail-call
      return;
    }
    if (v1 != v2) break;
    v3 = &v3[1];
    dat_1810c += 1;
    v2 = getc_unlocked(a0);
    v1 = *v3;
  }
}

// Function: sub_3150 @ 0x3150
void sub_3150(int a0) // return-dupe
{
  int v1;
  int v2;
  int v3; // eax
  int v4;
  
  v1 = dat_18108 + a0;
  v4 = dat_18108;
  if (dat_d0f8) {
    v2 = v1 + 7;
    if (0 <= v1)
      v2 = v1;
    if ((dat_18108 + 1 < (int)(v2 & 0xfffffff8)) && (dat_18108 < (int)(v2 & 0xfffffff8))) {
      do {
        putchar_unlocked(9);
        v4 = dat_18108 + 7;
        if (0 <= dat_18108)
          v4 = dat_18108;
        v3 = (v4 >> 3) + 1;
        v4 = v3 * 8;
        dat_18108 = v4;
      } while (v3 < v2 >> 3);
    }
  }
  if (v1 <= v4)
    return;
  do {
    putchar_unlocked(0x20);
    dat_18108 += 1;
  } while (dat_18108 < v1);
}

// Function: sub_31f0 @ 0x31f0
void sub_31f0(struct_0 *a0)
{
  char v1;
  char *v2;
  int v3;
  char *v4; // rbp
  
  v2 = a0->field_0x0;
  v3 = 0;
  if (a0->field_0x8) {
    v4 = &v2[(unsigned int)a0->field_0x8];
    do {
      v1 = *v2;
      v2 = &v2[1];
      putchar_unlocked((int)v1);
    } while (v4 != v2);
    v3 = a0->field_0x8;
  }
  dat_18108 += v3;
}

// Function: sub_3230 @ 0x3230
void sub_3230(long a0,int a1)
{
  long v1;
  
  dat_18108 = 0;
  sub_3150(dat_d0f4);
  fputs_unlocked(dat_18128,stdout);
  dat_18108 = dat_18114 + dat_18108;
  sub_3150(a1 - dat_18108);
  v1 = *(long *)(a0 + 0x20);
  for (; a0 != v1 + -0x28; a0 = a0 + 0x28) {
    sub_31f0(a0);
    sub_3150(*(unsigned int *)(a0 + 0xc));
  }
  sub_31f0(a0);
  dat_d0e0 = dat_18108;
  putchar_unlocked(10); // tail-call
}

// Function: sub_32d0 @ 0x32d0
void sub_32d0(long a0) // return-dupe
{
  long *v1;
  long v2;
  
  sub_3230(0xd120,dat_d0f0);
  v2 = dat_d140;
  if (dat_d140 == a0)
    return;
  do {
    sub_3230(v2,dat_d0ec);
    v1 = (long *)(v2 + 0x20);
    v2 = *v1;
  } while (a0 != *v1);
}

// Function: sub_3320 @ 0x3320
int sub_3320(FILE *a0,int a1)
{
  int v1; // eax
  int v2;
  int v3; // edx
  char *v4;
  
  dat_18108 = 0;
  if (dat_1810c <= dat_d0e4) { // branch-flip
    if (a1 == 10 || a1 == -1) {
      v1 = a1;
      dat_18108 = 0;
      return v1;
    }
    sub_3150(dat_d0e4);
    v3 = 0;
    v4 = dat_18128;
    v1 = dat_18108;
    v2 = dat_1810c;
    if (dat_18108 == dat_1810c) goto label_33e4;
label_33a2:
    do {
      if (!*v4) goto label_33ac;
      putchar_unlocked((int)*v4);
      v4 = &v4[1];
      v1 = dat_18108 + 1;
      v2 = dat_1810c;
      dat_18108 = v1;
    } while (v1 != dat_1810c);
  }
  else {
    sub_3150(dat_d0e4);
    v4 = dat_18128;
    v1 = dat_18108;
    v2 = dat_1810c;
    if (dat_18108 != dat_1810c) goto label_33a2;
  }
label_33ac:
  if (a1 == 10 || a1 == -1) {
    if (a1 != -1) {
      v1 = a1;
      return v1;
    }
    v1 = dat_18114 + dat_d0e4;
    if (dat_1810c < v1) {
      v1 = -1;
      return v1;
    }
    putchar_unlocked(10);
    v1 = -1;
    return v1;
  }
  v3 = v2 - v1;
label_33e4:
  sub_3150(v3);
  do {
    putchar_unlocked(a1);
    v1 = getc_unlocked(a0);
    if (v1 == 10) {
      v1 = 10;
      return v1;
    }
    a1 = v1;
  } while (v1 != -1);
  v1 = -1;
  return v1;
}

// Function: sub_3460 @ 0x3460
void sub_3460(struct_1 *a0)
{
  char *v1;
  char *v2; // rax
  long *v3; // rax
  char *v4;
  int v5; // ebp
  
  v1 = a0->field_0x0;
  v4 = &v1[(long)a0->field_0x8 + -1];
  v2 = strchr("([\'`\"",(int)*v1);
  v3 = __ctype_b_loc();
  a0->field_0x10 = *(unsigned char *)(*v3 + (unsigned long)(unsigned char)*v4 * 2) & 4 | v2 != NULL | a0->field_0x10 & 0xfa;
  if (v4 <= v1) // branch-flip
    v5 = (int)*v4;
  else {
    while( true ) {
      v5 = (int)*v4;
      if (!strchr(")]\'\"",v5)) break;
      if (v1 == &v4[-1]) {
        v5 = (int)v4[-1];
        break;
      }
      v4 = &v4[-1];
    }
  }
  a0->field_0x10 = a0->field_0x10 & 0xfd | (strchr(".?!",v5) != NULL) * '\x02';
}

// Function: sub_3530 @ 0x3530
void sub_3530(void)
{
  long *v1;
  long v2;
  void *v3; // rax
  long v4; // rax
  long *v5;
  long v6;
  long *v7;
  long *v8;
  
  v5 = dat_d100;
  if (dat_d100 == (long *)0xd120) {
    fwrite_unlocked((void *)0x16d80,1,dat_16d60 - 0x16d80,stdout);
    dat_16d60 = 0x16d80;
    return;
  }
  sub_2e50();
  v8 = v5;
  if (v5 != dat_d140) {
    v2 = 0x7fffffffffffffff;
    v7 = dat_d140;
    v6 = dat_d140[3];
    do {
      v1 = (long *)v7[4];
      v6 -= v1[3];
      if (v6 < v2) {
        v2 = v6;
        v8 = v7;
      }
      if (v2 <= 0x7ffffffffffffff6)
        v2 += 9;
      v7 = v1;
      v6 = v1[3];
    } while (v5 != v1);
  }
  sub_32d0(v8);
  v6 = dat_16d60;
  v3 = __memmove_chk((void *)0x16d80,(void *)*v8,dat_16d60 - *v8,5000);
  v2 = *v8;
  v4 = (long)((int)v2 - (int)v3);
  dat_16d60 = v6 - v4;
  if (v8 <= dat_d100) {
    v5 = v8;
    while( true ) {
      v7 = &v5[5];
      *v5 = v2 - v4;
      if (dat_d100 < v7) break;
      v2 = *v7;
      v5 = v7;
    }
  }
  v2 = (long)dat_d100 - (long)&v8[-0x1a24];
  __memmove_chk((void *)0xd120,v8,(long)dat_d100 + (0x28U - (long)v8),40000);
  dat_d100 = (long *)v2;
}

// Function: sub_3690 @ 0x3690
void sub_3690(FILE *a0,int a1)
{
  bool v1; // dl
  int v2; // edx
  long *v3;
  
  v3 = dat_d100;
label_36b8:
  do {
    *v3 = (long)dat_16d60;
    do {
      if (dat_16d60 == (char *)0x18108) {
        sub_2c80(1);
        sub_3530();
      }
      *dat_16d60 = (char)a1;
      dat_16d60 = &dat_16d60[1];
      a1 = getc_unlocked(a0);
    } while ((a1 != -1) && (!sub_4030(a1)));
    v2 = (int)dat_16d60 - (int)*dat_d100;
    *(int *)&dat_d100[1] = v2;
    v2 = dat_1810c + v2;
    dat_1810c = v2;
    sub_3460(dat_d100);
    a1 = sub_3050(a0,a1);
    v2 = dat_1810c - v2;
    *(int *)((long)dat_d100 + 0xc) = v2;
    if (a1 != -1) { // branch-flip
      if (*(unsigned char *)&dat_d100[2] & 2) { // branch-flip
        v1 = 1 < v2 || a1 == 10;
        if (1 < v2 || a1 == 10) goto label_3791;
        *(unsigned char *)&dat_d100[2] = *(unsigned char *)&dat_d100[2] & 0xf7;
      }
      else {
        v1 = 0;
label_3791:
        *(unsigned char *)&dat_d100[2] = *(unsigned char *)&dat_d100[2] & 0xf7 | v1 * '\b';
        if (a1 == 10) goto label_37d4;
      }
      if (dat_18130) goto label_37d4;
      if (dat_d100 != (long *)0x16d10) {
        v3 = &dat_d100[5];
        dat_d100 = v3;
        goto label_36b8;
      }
label_3820:
      sub_2c80(1);
      sub_3530();
    }
    else {
      *(unsigned char *)&dat_d100[2] = *(unsigned char *)&dat_d100[2] | 8;
label_37d4:
      *(unsigned int *)((long)dat_d100 + 0xc) = ((*(unsigned char *)&dat_d100[2] & 8) != 0) + 1;
      if (dat_d100 == (long *)0x16d10) goto label_3820;
    }
    v3 = &dat_d100[5];
    dat_d100 = v3;
    if ((a1 == 10) || (a1 == -1)) {
      sub_30b0(a0); // tail-call
      return;
    }
  } while( true );
}

// Function: sub_3850 @ 0x3850
unsigned long sub_3850(FILE *a0)
{
  char v1; // al
  int v2;
  unsigned int v3; // eax
  unsigned int v4; // eax
  unsigned long v5; // rsi
  
  dat_d0e0 = 0;
  v2 = dat_d0e8;
  while ((((v2 == 10 || (v2 == -1)) || (dat_d0e4 < dat_18118)) || (dat_1810c < dat_1811c + dat_d0e4))) {
    v2 = sub_3320(a0);
    if (v2 == -1) {
      dat_d0e8 = v2;
      return 0;
    }
    putchar_unlocked(10);
    v2 = sub_30b0(a0);
  }
  dat_d0f4 = dat_d0e4;
  dat_d0f0 = dat_1810c;
  dat_16d60 = 0x16d80;
  dat_d100 = 0xd120;
  v3 = sub_3690(a0);
  v5 = (unsigned long)v3;
  sub_2c80((unsigned char)sub_2d10(v3));
  v4 = (unsigned int)v5;
  if (!dat_18131) {
    v1 = sub_2d10(v5 & 0xffffffff);
    v4 = (unsigned int)v5;
    if (dat_18133) { // branch-flip
      if (v1) {
        do {
          v4 = sub_3690(a0);
          v1 = sub_2d10(v4);
          if (!v1) break;
        } while (dat_1810c == dat_d0ec);
      }
    }
    else if (dat_18132) { // branch-flip
      if ((v1) && (dat_1810c != dat_d0f0)) {
        do {
          v4 = sub_3690(a0);
          v1 = sub_2d10(v4);
          if (!v1) break;
        } while (dat_1810c == dat_d0ec);
      }
    }
    else {
      while( true ) {
        v4 = (unsigned int)v5;
        if ((!sub_2d10(v5 & 0xffffffff)) || (dat_1810c != dat_d0ec)) break;
        v5 = (unsigned long)(unsigned int)sub_3690(a0);
      }
    }
  }
  if (dat_d100 <= 0xd120)
    __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph"); // no-return
  *(unsigned char *)(dat_d100 - 0x18) = *(unsigned char *)(dat_d100 - 0x18) | 10;
  dat_d0e8 = v4;
  return 1;
}

// Function: sub_3a30 @ 0x3a30
unsigned int sub_3a30(FILE *a0,char *a1)
{
  int v1;
  unsigned int v2; // eax
  char *v3; // rax
  
  sub_41b0(a0,2);
  dat_d0f8 = 0;
  dat_d0ec = 0;
  dat_d0e8 = sub_30b0(a0);
  while (v2 = sub_3850(a0), (char)v2) {
    sub_2e50();
    sub_32d0(dat_d100);
  }
  if (ferror_unlocked(a0)) { // branch-flip
    if (a0 != stdin) // branch-flip
      sub_41e0(a0);
    else {
      clearerr_unlocked(a0);
    }
    v3 = (char *)sub_66c0(0,3,a1);
  }
  else {
    if (a0 == stdin) {
      clearerr_unlocked(a0);
      return 1;
    }
    if (!sub_41e0(a0))
      return 1;
    v1 = *__errno_location();
    if (v1 < 0)
      return 1;
    v3 = (char *)sub_66c0(0,3,a1);
    if (v1) {
      error(0,v1,"%s",v3);
      return v2;
    }
  }
  error(0,0,dcgettext(NULL,"read error",5),v3);
  return v2;
}

// Function: sub_3b90 @ 0x3b90
void sub_3b90(void)
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
    if (!strcmp("fmt",v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!v6)
    v6 = "fmt";
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "fmt";
  if (!strcmp("fmt","["))
    v3 = "test";
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == "fmt")
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_3dc0 @ 0x3dc0
void sub_3dc0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_18148;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5),v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n",5),v1);
    sub_2f70();
    sub_2fa0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_3b90();
  }
  exit(a0); // no-return
}

// Function: sub_3f00 @ 0x3f00
unsigned long sub_3f00(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_3f30 @ 0x3f30
bool sub_3f30(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_3f50 @ 0x3f50
bool sub_3f50(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_3f60 @ 0x3f60
unsigned int sub_3f60(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_3f80 @ 0x3f80
unsigned long sub_3f80(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_3fa0 @ 0x3fa0
bool sub_3fa0(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_3fb0 @ 0x3fb0
bool sub_3fb0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_3fc0 @ 0x3fc0
bool sub_3fc0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_3fd0 @ 0x3fd0
bool sub_3fd0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_3fe0 @ 0x3fe0
unsigned long sub_3fe0(int a0)
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

// Function: sub_4030 @ 0x4030
bool sub_4030(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_4050 @ 0x4050
bool sub_4050(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_4060 @ 0x4060
unsigned int sub_4060(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_4090 @ 0x4090
int sub_4090(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_40b0 @ 0x40b0
int sub_40b0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_40d0 @ 0x40d0
void sub_40d0(unsigned long a0)
{
  dat_18140 = a0;
}

// Function: sub_40e0 @ 0x40e0
void sub_40e0(char a0)
{
  dat_18138 = a0;
}

// Function: sub_40f0 @ 0x40f0
void sub_40f0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_7920(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_18138) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_18140) { // branch-flip
        v4 = (char *)sub_66a0(dat_18140);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_d018); // return-dupe, no-return
    }
  }
  v1 = sub_7920(stderr);
  if (!v1)
    return;
  _exit(dat_d018);
}

// Function: sub_41a0 @ 0x41a0
void sub_41a0(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_41b0 @ 0x41b0
void sub_41b0(FILE *a0,unsigned int a1)
{
  if (a0) {
    sub_41a0(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_41e0 @ 0x41e0
unsigned long sub_41e0(FILE *a0) // early-return
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
    if (sub_4290(a0)) {
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

// Function: sub_4270 @ 0x4270
void sub_4270(FILE *a0)
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_42d0(a0,0,1); // tail-call
}

// Function: sub_4290 @ 0x4290
void sub_4290(FILE *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_4270(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_42d0 @ 0x42d0
int sub_42d0(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_4350 @ 0x4350
void sub_4350(char *a0)
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
    dat_18148 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_43f0 @ 0x43f0
struct_2 * sub_43f0(struct_2 *a0,int a1)
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

// Function: sub_4440 @ 0x4440
bool sub_4440(long a0,long a1)
{
  return sub_78c0((char *)(a0 + 9),(char *)(a1 + 9)) == 0;
}

// Function: sub_4460 @ 0x4460
unsigned long sub_4460(long a0,long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_4050((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_4440(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_44c0 @ 0x44c0
unsigned long sub_44c0(long a0,long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_4050((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_4460(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_4540 @ 0x4540
unsigned long sub_4540(long a0,long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_4050((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_44c0(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_45b0 @ 0x45b0
unsigned long sub_45b0(long a0,long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_4050((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_4540(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_4650 @ 0x4650
unsigned long sub_4650(long a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_4050((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_45b0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_46f0 @ 0x46f0
unsigned long sub_46f0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_4050((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_4650(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_47b0 @ 0x47b0
unsigned long sub_47b0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_4050((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_46f0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_4870 @ 0x4870
unsigned long sub_4870(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_4050((int)(char)a2)) // branch-flip
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
      return sub_47b0(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_4950 @ 0x4950
unsigned long sub_4950(unsigned char *a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_4050((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_4870(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_4a10 @ 0x4a10
char * sub_4a10(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned char *v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = (unsigned char *)sub_7a00();
    if (sub_4950(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_4950(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x8798 : (char *)0x879d; // branch-flip
    }
  }
  return v1;
}

// Function: sub_4b40 @ 0x4b40
long * sub_4b40(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_4bb0:
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
label_5036:
      v2 = 0;
      goto label_4ff0;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_5151;
      }
      v23 = 1;
      v2 = 0;
      v12 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v2 = 1;
label_4ff0:
      v23 = 1;
      v12 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_5151;
      }
      goto label_5036;
    case 5:
      if (!v17) goto label_4f91;
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
        v9 = (char *)sub_4a10("`",a4);
        v35 = (char *)v9;
        v9 = (char *)sub_4a10("\'",a4);
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
label_4c73:
  do {
label_4c86:
    v22 = 0;
    a1 = v13;
label_4c90:
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
          if ((a3 < v22 + v29) || (v8 = memcmp(v11,v30,v29), v8)) goto label_5600;
          if (!v23) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v20 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_51fc;
                default:
                  goto label_4ed3;
                case 7:
                  goto label_51c0;
                case 8:
                  goto label_51a5;
                case 9:
                  goto label_5283;
                case 10:
                  goto label_519b;
                case 0xb:
                  goto label_52b0;
                case 0xc:
                  goto label_51e7;
                case 0xd:
                  goto label_50ed;
                case 0x20:
                  goto label_52bd;
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
                  goto label_4e86;
                case 0x23:
                  goto label_5260;
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
                  goto label_4db8;
                case 0x27:
                  goto label_50d2;
                case 0x3f:
                  goto label_5091;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_4ed3;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_4db8;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_56a4;
              goto label_55e8;
            }
            if (v7 == 0x7d) goto label_5770;
            if ((char)v7 <= '}') {
              v20 = 0x7b;
              if (v7 == 0x7b) goto label_55ca;
              v20 = 0x7c;
              if (v7 == 0x7c) goto label_55e8;
              goto label_4ed3;
            }
            if (v7 != 0x7e) goto label_4ed3;
label_5724:
            if (!v22) {
              v7 = a4 == 2;
              v20 = 0x7e;
              v25 = v3;
              goto label_4f49;
            }
            v20 = 0x7e;
            goto label_5732;
          }
          goto label_4ebc;
        }
label_5600:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v20 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v21 = 0;
                if (v7 == 0x7e) goto label_5724;
                goto label_4ed0;
              }
              v20 = 0x7b;
              v24 = 0;
              if (v7 != 0x7b) {
                v20 = 0x7c;
                v24 = 0;
                v21 = 0;
                if (v7 == 0x7c) goto label_55e8;
                goto label_4ed3;
              }
            }
            else {
              v20 = 0x7d;
              v24 = 0;
            }
label_55ca:
            if (a3 == 0xffffffffffffffff) goto label_577f;
label_55d8:
            v21 = v24;
            if (a3 == 1) goto label_5265;
          }
          else {
            if (v7 == 0x40) goto label_4ed0;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v21 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_4db8;
            v24 = 0;
            v21 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_56a4:
              if ((char)v18 != '\\') goto label_4ed3;
              if (((bool)(v2 & v23)) && (v29)) goto label_56ca;
              v16 = 0x5c;
              goto label_50f2;
            }
          }
label_55e8:
          v7 = a4 == 2;
          v25 = v3;
          v24 = 0;
          goto label_4f49;
        }
        switch(v7) {
          case 0:
            goto label_51ee;
          default:
label_4ed0:
            v24 = 0;
label_4ed3:
            v16 = (unsigned char)v18;
            v21 = v24;
            if (v32 != 1) goto label_5453;
label_4ee0:
            v9 = __ctype_b_loc();
            v20 = 1;
            v25 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v26 = (bool)(v25 & v2);
            v25 = !v25;
            v21 = v24;
            goto label_4f2e;
          case 7:
label_51c0:
            v16 = 0x61;
            if (!v23) goto label_51b8;
            goto label_511f;
          case 8:
label_51a5:
            v16 = 0x62;
            goto label_51aa;
          case 9:
            v24 = 0;
label_5283:
            v20 = 9;
            v16 = 0x74;
            goto label_5290;
          case 10:
label_519b:
            v16 = 0x6e;
            break;
          case 0xb:
label_52b0:
            v16 = 0x76;
            break;
          case 0xc:
label_51e7:
            v16 = 0x66;
label_51aa:
            if (v23) goto label_511f;
label_51b8:
            v24 = 0;
            goto label_4e3e;
          case 0xd:
label_50ed:
            v16 = 0x72;
            break;
          case 0x20:
            v21 = 0;
label_52bd:
            v20 = 0x20;
            goto label_4db8;
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
            goto label_4e89;
          case 0x23:
            v20 = 0x23;
            v24 = 0;
            goto label_5265;
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
label_4db8:
            v7 = 0;
            goto label_4dc8;
          case 0x27:
            v21 = 0;
            goto label_50d2;
          case 0x3f:
            v21 = 0;
            goto label_5091;
          
        }
        goto label_50f2;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_5770:
            v20 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_55d8;
label_577f:
            v21 = v24;
            if (a2[1]) goto label_55e8;
            goto label_5265;
          }
          if ((char)v16 <= '}') {
            v20 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_4e86;
              goto label_4ed3;
            }
            goto label_55ca;
          }
          v20 = 0x7e;
          if (v16 == 0x7e) goto label_5265;
          if (v32 == 1) goto label_4ee0;
label_5453:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v20 = 0;
          do {
            v18 = v22 + v20;
            v9 = (long)sub_7a40(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v24 = v25;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_59bb;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_5d35;
              goto label_5d3f;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_558d;
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
          goto label_59bb;
        }
        if (v16 == 0x40) goto label_4ed3;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_4f40;
        if ((unsigned long)v9 & 0xa4000000) goto label_4e86;
        if (v16 != 0x5c) goto label_4ed3;
        if (a4 == 2) {
          if (!v23) goto label_56ca;
          goto label_4eb3;
        }
        if (!(bool)(v2 & v23 & v29 != 0)) {
          v20 = 0x5c;
          v16 = 0x5c;
          v24 = 0;
          goto label_5290;
        }
label_56ca:
        v22 += 1;
        v24 = 0;
        v16 = 0x5c;
        v7 = v19;
        goto label_56e0;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v21 = 0;
            v20 = 0;
            if (v33 & 1) goto label_5362;
            goto label_529b;
          }
label_51ee:
          if (!v23) {
            v21 = 0;
label_51fc:
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
                goto label_4f73;
              }
label_5b05:
              a0[v13] = 0x5c;
              v12 = v13;
              v19 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v19;
              goto label_5b05;
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
label_5b3c:
              v16 = (unsigned char)v20;
              v24 = 0;
              v12 = v13;
              v7 = v25;
              if (!v2 || v26) goto label_4f73;
              goto label_4dc8;
            }
            v16 = 0x30;
            v24 = 0;
            v12 = v13;
            v7 = v25;
            goto label_4f73;
          }
          if (a4 != 2) goto label_4ebc;
          goto label_512b;
        default:
          goto label_4ed3;
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
          goto label_531f;
        case 10:
          v20 = 10;
          v16 = 0x6e;
          goto label_531f;
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
label_531f:
          v7 = a4 == 2 & v23;
          v24 = 0;
          if (!(bool)v7) goto label_5290;
label_4ea1:
          a4 = 2;
label_4eb3:
          if (!v27) goto label_4ebc;
          goto label_512b;
        case 0x20:
          v18 = 0x20;
          goto label_4e89;
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
label_4e86:
          v25 = 0;
          goto label_4e89;
        case 0x23:
label_5260:
          v20 = 0x23;
label_5265:
          v21 = v24;
          if (v22) {
label_5732:
            v7 = a4 == 2;
            v25 = v3;
            v24 = 0;
            goto label_4f49;
          }
          v18 = v20;
label_4e89:
          v20 = v18;
          v21 = v24;
          v24 = v25;
          if ((a4 == 2) && (v23)) goto label_4ea1;
          goto label_4f47;
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
          goto label_4f40;
        case 0x27:
label_50d2:
          v24 = v25;
          if (a4 != 2) {
            v7 = 0;
            v20 = 0x27;
            goto label_4f49;
          }
          if (!v23) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v31) goto label_5b5b;
            }
            else {
label_5b5b:
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
            goto label_4f73;
          }
          goto label_4eb3;
        case 0x3f:
label_5091:
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
                  goto label_4f49;
                }
                if (v23) goto label_4ebc;
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
                goto label_5b3c;
              }
            }
            v7 = 0;
            v20 = 0x3f;
            v25 = v3;
            v24 = 0;
            goto label_4f49;
          }
          if (v23) goto label_4eb3;
          v7 = 0;
          v16 = 0x3f;
          v24 = 0;
          goto label_4f73;
        
      }
label_5290:
      v21 = v24;
      if (!v2) {
label_529b:
        v16 = (unsigned char)v20;
        v24 = 0;
        v7 = 0;
        v25 = v3;
        if (!v23) goto label_4f70;
        goto label_4dc8;
      }
label_50f2:
      v24 = 0;
      goto label_5103;
    }
    if (!(bool)(v12 == 0 & v23 & a4 == 2)) {
      v7 = a4 == 2 & (v23 ^ 1U);
      v23 = (bool)(v23 ^ 1U);
      if ((!(bool)v7) || (v23 = (bool)v7, !v3)) {
label_5dbf:
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
        if (a1 || !v31) goto label_5dbf;
        v22 = v31;
        goto label_588b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 1;
      v32 = (unsigned long)v9;
      v30 = "\"";
      if (!(v33 & 2)) goto label_5e6f;
      v3 = 0;
      v19 = 0;
      v31 = 0;
      v2 = v4;
      v23 = v4;
      goto label_4c86;
    }
label_558d:
    if (v2) {
label_512b:
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
label_5151:
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
label_588b:
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
label_5d35:
    if (!a2[(long)v9]) break;
  }
label_5d3f:
  v24 = 0;
label_59bb:
  v18 = (unsigned long)v16;
  v25 = v24;
  if (2 <= v20) {
label_59c5:
    v25 = 0;
    v20 += v22;
    v13 = v22;
    do {
      v16 = (unsigned char)v18;
      if (v26) {
        v27 = a4 == 2;
        if (v23) goto label_4eb3;
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
        if (v20 <= v22) goto label_4e51;
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
          goto label_56e0;
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
label_4f2e:
  if (v26) {
    v24 = 0;
    v26 = v2;
    goto label_59c5;
  }
label_4f40:
  v20 = v18;
  v24 = v25;
label_4f47:
  v7 = a4 == 2;
  v25 = v3;
label_4f49:
  v16 = (unsigned char)v20;
  v26 = (bool)v7 == 0;
  v3 = v25;
  v7 = 0;
  if ((v26 && v2) || (v7 = 0, v23)) {
label_4dc8:
    v16 = (unsigned char)v20;
    v12 = v13;
    if (!v28) goto label_4f73;
    if (!(*(unsigned int *)(v28 + (v20 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_4f73;
  }
  else {
label_4f70:
    v7 = 0;
    v3 = v25;
label_4f73:
    if (!v21) {
      v7 ^= 1;
      v22 += 1;
      v7 &= v19;
label_56e0:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v19 = 0;
        v12 += 2;
      }
      goto label_4e51;
    }
  }
label_5103:
  if (v23) {
    v23 = v2;
label_511f:
    if ((bool)(v23 & a4 == 2)) goto label_512b;
label_4ebc:
    v33 &= 0xfffffffd;
    v28 = 0;
    goto label_4bb0;
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
label_4e3e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v22 += 1;
label_4e51:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v24)
    v4 = 0;
  goto label_4c90;
label_5362:
  v22 += 1;
  goto label_4c90;
label_5e6f:
  a1 = v31;
label_4f91:
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
  goto label_4c73;
}

// Function: sub_5f70 @ 0x5f70
char * sub_5f70(unsigned int a0,char *a1,unsigned long a2,struct_3 *a3)
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
    v5 = dat_d060;
    if (dat_d058 <= (int)a0) {
      v10 = (long)dat_d058;
      v8 = (long)(int)((a0 - dat_d058) + 1);
      if (dat_d060 != (unsigned long *)0xd070) // branch-flip
        v5 = (unsigned long *)sub_7100(dat_d060,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_7100(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_d070;
        v5[1] = dat_d078;
      }
      dat_d060 = v5;
      memset(&v5[(long)dat_d058 * 2],0,(v10 - dat_d058) * 0x10);
      dat_d058 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_4b40(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x18160)
        free(v7);
      v7 = (char *)sub_6f80(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_4b40(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_6160 @ 0x6160
void sub_6160(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x18260;
  sub_7240(a0,0x38);
  *v2 = v1;
}

// Function: sub_61a0 @ 0x61a0
unsigned int sub_61a0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x18260;
  return *a0;
}

// Function: sub_61c0 @ 0x61c0
void sub_61c0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x18260;
  *a0 = a1;
}

// Function: sub_61e0 @ 0x61e0
unsigned int sub_61e0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x18260;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_6220 @ 0x6220
unsigned int sub_6220(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x18260;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_6240 @ 0x6240
void sub_6240(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x18260;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_6280 @ 0x6280
void sub_6280(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x18260;
  v2 = __errno_location();
  v1 = *v2;
  sub_4b40(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_6300 @ 0x6300
char * sub_6300(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x18260;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_4b40(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_6f80(v3 + 1U);
  sub_4b40(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_63f0 @ 0x63f0
char * sub_63f0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_6300(a0,a1,NULL,a2); // tail-call
}

// Function: sub_6400 @ 0x6400
void sub_6400(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_d060;
  if (2 <= dat_d058) {
    v4 = (unsigned long *)((long)dat_d060 + 0x18);
    v1 = (unsigned long *)((long)dat_d060 + (unsigned long)(unsigned int)(dat_d058 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x18160) {
    free(*(void **)((long)v3 + 8));
    dat_d078 = 0x18160;
    dat_d070 = 0x100;
  }
  if (v3 == (void *)0xd070) {
    dat_d058 = 1;
    return;
  }
  free(v3);
  dat_d058 = 1;
  dat_d060 = (void *)0xd070;
}

// Function: sub_64a0 @ 0x64a0
char * sub_64a0(unsigned int a0,char *a1)
{
  return sub_5f70(a0,a1,0xffffffffffffffff,(struct_3 *)0x18260); // tail-call
}

// Function: sub_64c0 @ 0x64c0
char * sub_64c0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_5f70(a0,a1,a2); // tail-call
}

// Function: sub_64d0 @ 0x64d0
char * sub_64d0(char *a0)
{
  return sub_64a0(0,a0); // tail-call
}

// Function: sub_64e0 @ 0x64e0
char * sub_64e0(char *a0,unsigned long a1)
{
  return sub_64c0(0,a0,a1); // tail-call
}

// Function: sub_64f0 @ 0x64f0
void sub_64f0(unsigned int a0,unsigned long a1,char *a2)
{
  char v1 [56];
  
  sub_43f0(v1);
  sub_5f70(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_6550 @ 0x6550
void sub_6550(unsigned int a0,unsigned long a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_43f0(v1);
  sub_5f70(a0,a2,a3,v1);
}

// Function: sub_65c0 @ 0x65c0
void sub_65c0(unsigned int a0,char *a1)
{
  sub_64f0(0,a0,a1); // tail-call
}

// Function: sub_65d0 @ 0x65d0
void sub_65d0(unsigned int a0,char *a1,unsigned long a2)
{
  sub_6550(0,a0,a1,a2); // tail-call
}

// Function: sub_65f0 @ 0x65f0
void sub_65f0(char *a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_18260;
  v3 = dat_18268;
  v4 = dat_18270;
  v5 = dat_18278;
  v8 = dat_18290;
  v6 = dat_18280;
  v7 = dat_18288;
  sub_61e0(&v1,(int)a2,1,a3,a0,a1);
  sub_5f70(0,a0,a1,v2);
}

// Function: sub_6680 @ 0x6680
void sub_6680(char *a0,char a1)
{
  sub_65f0(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_66a0 @ 0x66a0
void sub_66a0(char *a0)
{
  sub_6680(a0,0x3a); // tail-call
}

// Function: sub_66b0 @ 0x66b0
void sub_66b0(char *a0,unsigned long a1)
{
  sub_65f0(a0,a1,0x3a); // tail-call
}

// Function: sub_66c0 @ 0x66c0
void sub_66c0(unsigned int a0,unsigned long a1,char *a2)
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
  
  sub_43f0(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_61e0(v3,0x3a,1);
  sub_5f70(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_6760 @ 0x6760
void sub_6760(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_18260;
  v2 = dat_18268;
  v7 = dat_18290;
  v3 = dat_18270;
  v4 = dat_18278;
  v5 = dat_18280;
  v6 = dat_18288;
  sub_6240(&v1,a1,a2);
  sub_5f70(a0,a3,a4,&v1);
}

// Function: sub_6800 @ 0x6800
void sub_6800(unsigned int a0)
{
  sub_6760(a0); // tail-call
}

// Function: sub_6810 @ 0x6810
void sub_6810(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_6800(0,a0,a1,a2); // tail-call
}

// Function: sub_6830 @ 0x6830
void sub_6830(long a0,long a1,char *a2,unsigned long a3)
{
  sub_6760(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_6850 @ 0x6850
char * sub_6850(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_5f70(a0,a1,a2); // tail-call
}

// Function: sub_6860 @ 0x6860
char * sub_6860(char *a0,unsigned long a1)
{
  return sub_6850(0,a0,a1); // tail-call
}

// Function: sub_6870 @ 0x6870
char * sub_6870(unsigned int a0,char *a1)
{
  return sub_6850(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_6880 @ 0x6880
char * sub_6880(char *a0)
{
  return sub_6870(0,a0); // tail-call
}

// Function: sub_6890 @ 0x6890
long sub_6890(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_69bd:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_69f6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x8e88) + 0x8e88;
  switch(a5) {
    case 0:
      goto label_69f6;
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
      goto label_69bd;
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

// Function: sub_6d10 @ 0x6d10
void sub_6d10(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_6890(); // tail-call
}

// Function: sub_6d30 @ 0x6d30
void sub_6d30(FILE *a0,char *a1,char *a2,char *a3,struct_4 *a4)
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
  sub_6890(a0,a1,a2,a3);
}

// Function: sub_6dd0 @ 0x6dd0
void sub_6dd0(void)
{
  sub_6d30();
}

// Function: sub_6e80 @ 0x6e80
void sub_6e80(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_6f30 @ 0x6f30
long sub_6f30(long a0)
{
  if (a0)
    return a0;
  sub_7300(); // no-return
}

// Function: sub_6f40 @ 0x6f40
void sub_6f40(unsigned long a0)
{
  sub_6f30(malloc(a0)); // tail-call
}

// Function: sub_6f60 @ 0x6f60
void sub_6f60(void)
{
  sub_6f30(sub_7990()); // tail-call
}

// Function: sub_6f80 @ 0x6f80
void sub_6f80(void)
{
  sub_6f40(); // tail-call
}

// Function: sub_6f90 @ 0x6f90
void sub_6f90(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_7300(); // no-return
}

// Function: sub_6fd0 @ 0x6fd0
void sub_6fd0(void)
{
  sub_6f30(sub_79a0()); // tail-call
}

// Function: sub_6ff0 @ 0x6ff0
void sub_6ff0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_7300(); // no-return
}

// Function: sub_7030 @ 0x7030
void sub_7030(void *a0,unsigned long a1,unsigned long a2)
{
  sub_6ff0(a0,a1,a2); // tail-call
}

// Function: sub_7040 @ 0x7040
void sub_7040(void)
{
  sub_6f30(sub_79d0()); // tail-call
}

// Function: sub_7060 @ 0x7060
void sub_7060(unsigned long a0,unsigned long a1)
{
  sub_6ff0(NULL,a0,a1); // tail-call
}

// Function: sub_7070 @ 0x7070
void sub_7070(unsigned long a0,unsigned long a1)
{
  sub_7040(0,a0,a1); // tail-call
}

// Function: sub_7080 @ 0x7080
void sub_7080(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_7300(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_6ff0(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_70f0 @ 0x70f0
void sub_70f0(void *a0,unsigned long *a1)
{
  sub_7080(a0,a1,1); // tail-call
}

// Function: sub_7100 @ 0x7100
void sub_7100(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_71a6;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_71a6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_7300(a0,v3); // no-return
  sub_6f90(a0,v3);
  *a1 = v2;
}

// Function: sub_71e0 @ 0x71e0
void sub_71e0(unsigned long a0,unsigned long a1)
{
  sub_6f30(calloc(a0,a1)); // tail-call
}

// Function: sub_7200 @ 0x7200
void sub_7200(unsigned long a0)
{
  sub_71e0(a0,1); // tail-call
}

// Function: sub_7210 @ 0x7210
void sub_7210(void)
{
  sub_6f30(sub_79c0()); // tail-call
}

// Function: sub_7230 @ 0x7230
void sub_7230(unsigned long a0)
{
  sub_7210(a0,1); // tail-call
}

// Function: sub_7240 @ 0x7240
void sub_7240(void *a0,unsigned long a1)
{
  memcpy((void *)sub_6f40(a1),a0,a1); // tail-call
}

// Function: sub_7270 @ 0x7270
void sub_7270(void *a0,unsigned long a1)
{
  memcpy((void *)sub_6f60(a1),a0,a1); // tail-call
}

// Function: sub_72a0 @ 0x72a0
void sub_72a0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_6f60(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_72e0 @ 0x72e0
void sub_72e0(char *a0)
{
  sub_7240(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_7300 @ 0x7300
void sub_7300(void)
{
  error(dat_d018,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_7340 @ 0x7340
unsigned long sub_7340(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5,int a6)
{
  int v1;
  char *v2; // rax
  int *v3; // rax
  unsigned long v4; // stack - 0x48
  
  v1 = sub_74e0(a0,NULL,a1,&v4);
  if (v1) { // branch-flip
    v3 = __errno_location();
    if (v1 != 1) {
      if (v1 == 3)
        *v3 = 0;
      goto label_73e8;
    }
  }
  else {
    if ((a2 <= v4) && (v4 <= a3))
      return v4;
    v3 = __errno_location();
    if (v4 <= 0x3fffffff) {
      *v3 = 0x22;
      goto label_73e8;
    }
  }
  *v3 = 0x4b;
label_73e8:
  v2 = (char *)sub_6880(a0);
  v1 = *v3;
  if (v1 == 0x16) goto label_7420;
  do {
    if (!a6)
      a6 = 1;
    error(a6,v1,"%s: %s",a5,v2);
label_7420:
    v1 = 0;
  } while( true );
}

// Function: sub_7450 @ 0x7450
void sub_7450(unsigned char *a0,unsigned long a1,unsigned long a2,char *a3,char *a4,unsigned long a5)
{
  sub_7340(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_7480 @ 0x7480
bool sub_7480(unsigned long *a0,int a1)
{
  unsigned long v1; // rax
  bool v2; // cf
  
  v1 = SUB168(ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0),0);
  v2 = SUB168(ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0),8) != 0;
  if (v2)
    v1 = 0xffffffffffffffff;
  *a0 = v1;
  return v2;
}

// Function: sub_74b0 @ 0x74b0
unsigned int sub_74b0(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_7480();
  } while (a2);
  return v1;
}

// Function: sub_74e0 @ 0x74e0
unsigned int sub_74e0(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned long v10; // r8
  unsigned int v11;
  unsigned int v2; // eax
  int *v3; // rax
  long *v4; // rax
  unsigned char *v5;
  unsigned char v6;
  long v7;
  unsigned long v8; // stack - 0x48
  int v9; // ecx
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v7;
  v3 = __errno_location();
  *v3 = 0;
  v6 = *a0;
  v4 = __ctype_b_loc();
  v5 = a0;
  while (*(unsigned char *)(*v4 + 1 + (unsigned long)v6 * 2) & 0x20) {
    v6 = v5[1];
    v5 = &v5[1];
  }
  if (v6 == 0x2d)
    return 4;
  v8 = strtoumax(a0,a1,a2);
  v5 = (unsigned char *)*a1;
  if (v5 != a0) { // branch-flip
    if (*v3) { // branch-flip
      v11 = 1;
      if (*v3 != 0x22)
        return 4;
    }
    else {
      v11 = 0;
    }
    if (!a4) {
      *a3 = v8;
      return v11;
    }
    v6 = *v5;
    if (!v6) goto label_75a2;
    v9 = (int)(char)v6;
  }
  else {
    if ((!a4) || (v6 = *a0, !v6))
      return 4;
    v9 = (int)(char)v6;
    if (!strchr(a4,v9))
      return 4;
    v8 = 1;
    v11 = 0;
  }
  if (!strchr(a4,v9)) {
label_75a9:
    *a3 = v8;
    return v11 | 2;
  }
  v2 = (unsigned int)v6 - 0x45;
  v9 = 1;
  v10 = 0x400;
  if (((unsigned char)v2 <= 0x2f) && (0x814400308945U >> ((unsigned long)v2 & 0x3f) & 1)) {
    v10 = 0x400;
    v9 = 1;
    if (strchr(a4,0x30)) {
      v1 = v5[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v9 = (v1 == 0x42) + 1;
          v10 = 0x400;
          if (v1 == 0x42)
            v10 = 1000;
        }
        else {
          v9 = (v5[2] == 0x42) + 1 + (unsigned int)(v5[2] == 0x42);
        }
      }
      else {
        v9 = 2;
        v10 = 1000;
      }
    }
  }
  switch((unsigned int)v6 - 0x42 & 0xff) {
    case 0:
      v11 |= sub_7480(&v8,0x400);
      break;
    default:
      goto label_75a9;
    case 3:
      v11 |= sub_74b0(&v8,v10,6);
      break;
    case 5:
    case 0x25:
      v11 |= sub_74b0(&v8,v10,3);
      break;
    case 9:
    case 0x29:
      v11 |= sub_74b0(&v8,v10,1);
      break;
    case 0xb:
    case 0x2b:
      v11 |= sub_74b0(&v8,v10,2);
      break;
    case 0xe:
      v11 |= sub_74b0(&v8,v10,5);
      break;
    case 0x12:
    case 0x32:
      v11 |= sub_74b0(&v8,v10,4);
      break;
    case 0x17:
      v11 |= sub_74b0(&v8,v10,8);
      break;
    case 0x18:
      v11 |= sub_74b0(&v8,v10,7);
      break;
    case 0x20:
      v11 |= sub_7480(&v8,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v11 |= sub_7480(&v8,2);
    
  }
  *a1 = &v5[v9];
  if (v5[v9])
    v11 |= 2;
label_75a2:
  *a3 = v8;
  return v11;
}

// Function: sub_78c0 @ 0x78c0
int sub_78c0(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_4090((unsigned char)*a0);
    v1 = sub_4090((unsigned char)*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_7920 @ 0x7920
unsigned long sub_7920(FILE *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_41e0(a0);
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

// Function: sub_7990 @ 0x7990
void sub_7990(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_79a0 @ 0x79a0
void sub_79a0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_79c0 @ 0x79c0
void sub_79c0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_79d0 @ 0x79d0
void sub_79d0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_7a00 @ 0x7a00
char * sub_7a00(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_7a40 @ 0x7a40
unsigned long sub_7a40(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_7ad0(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_7ad0 @ 0x7ad0
bool sub_7ad0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_7c20(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_7b60 @ 0x7b60
void sub_7b60(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_7b70 @ 0x7b70
unsigned long sub_7b70(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_7b60();
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

// Function: sub_7c20 @ 0x7c20
unsigned long sub_7c20(void)
{
  return sub_7b70(); // tail-call
}

// Function: sub_7c30 @ 0x7c30
void sub_7c30(void)
{
  sub_7b60(); // tail-call
}

// Function: sub_7c40 @ 0x7c40
void sub_7c40(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_d008); // tail-call
}

// Function: _DT_FINI @ 0x7c54
void _DT_FINI(void)
{
  return;
}
