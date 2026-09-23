// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_bfe8)
    return;
  (*dat_bfe8)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_be38)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2360
void __cxa_finalize(void)
{
  (*dat_bff8)(); // jump-as-call
}

// Function: __uflow @ 0x2370
int __uflow(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_be40)(); // jump-as-call
  return v1;
}

// Function: free @ 0x2380
void free(void *a0)
{
  (*dat_be48)(); // jump-as-call
}

// Function: abort @ 0x2390
void abort(void)
{
  (*dat_be50)(); // jump-as-call
}

// Function: __errno_location @ 0x23a0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_be58)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x23b0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_be60)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x23c0
void _exit(int a0)
{
  (*dat_be68)(); // jump-as-call
}

// Function: __fpending @ 0x23d0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_be70)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x23e0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_be78)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x23f0
void clearerr_unlocked(FILE *a0)
{
  (*dat_be80)(); // jump-as-call
}

// Function: textdomain @ 0x2400
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_be88)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2410
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_be90)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2420
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_be98)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x2430
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_bea0)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2440
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_bea8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2450
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_beb0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2460
void __stack_chk_fail(void)
{
  (*dat_beb8)(); // jump-as-call
}

// Function: getopt_long @ 0x2470
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_bec0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2480
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_bec8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2490
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_bed0)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x24a0
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_bed8)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x24b0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_bee0)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x24c0
void lseek(void)
{
  (*dat_bee8)(); // jump-as-call
}

// Function: __assert_fail @ 0x24d0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_bef0)(); // jump-as-call
}

// Function: fputs @ 0x24e0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_bef8)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x24f0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bf00)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x2500
void posix_fadvise(void)
{
  (*dat_bf08)(); // jump-as-call
}

// Function: memcmp @ 0x2510
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_bf10)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2520
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_bf18)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2530
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bf20)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2540
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_bf28)(); // jump-as-call
  return v1;
}

// Function: __memmove_chk @ 0x2550
void * __memmove_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bf30)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2560
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bf38)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x2570
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_bf40)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x2580
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bf48)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x2590
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_bf50)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x25a0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_bf58)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x25b0
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_bf60)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x25c0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_bf68)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x25d0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bf70)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x25e0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_bf78)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x25f0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_bf80)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2600
void error(int a0,int a1,char *a2,...)
{
  (*dat_bf88)(); // jump-as-call
}

// Function: fseeko @ 0x2610
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_bf90)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x2620
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_bf98)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x2630
void strtoumax(void)
{
  (*dat_bfa0)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x2640
void __cxa_atexit(void)
{
  (*dat_bfa8)(); // jump-as-call
}

// Function: exit @ 0x2650
void exit(int a0)
{
  (*dat_bfb0)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2660
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_bfb8)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2670
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_bfc0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2680
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_bfc8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2690
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_bfd0)(); // jump-as-call
  return v1;
}

// Function: main @ 0x26a0
int main(int argc,char **argv,char **envp)
{
  bool v1;
  char **v10;
  char *v11; // stack - 0x48
  char *v12; // stack - 0x40
  int v2;
  unsigned int v3; // eax
  int v4; // eax
  char *v5; // rax
  FILE *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned int v9; // eax
  
  sub_4350(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_7b90(sub_4100);
  dat_17130 = 0;
  dat_17131 = 0;
  dat_17132 = 0;
  dat_17133 = 0;
  dat_17120 = 0x4b;
  dat_17128 = "";
  dat_1711c = 0;
  dat_17118 = 0;
  dat_17114 = 0;
  v12 = NULL;
  v10 = argv;
  if (((2 <= argc) && (v7 = argv[1], *v7 == '-')) && ((unsigned int)((int)v7[1] - 0x30U) <= 9)) {
    v12 = &v7[1];
    v10 = &argv[1];
    argc -= 1;
    *v10 = *argv;
  }
  v11 = NULL;
  while (v2 = getopt_long(argc,v10,"0123456789cstuw:p:g:",(option *)0xbaa0,NULL), v2 != -1) {
    if (0x77 < v2) {
label_2acd:
      if ((unsigned int)(v2 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),(char)v2);
      sub_3d70(1); // no-return
    }
    if (v2 <= 0x62) {
      if (v2 == -0x83) {
        sub_6c80(stdout,"fmt","GNU coreutils",dat_c010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_3d70(0); // no-return
      goto label_2acd;
    }
    switch(v2) {
      case 99:
        dat_17133 = 1;
        break;
      default:
        goto label_2acd;
      case 0x67:
        v11 = optarg;
        break;
      case 0x70:
        dat_17118 = 0;
        v7 = optarg;
        if (*optarg == ' ') {
          do {
            dat_17118 = (1U - (int)optarg) + (int)v7;
            v7 = &v7[1];
          } while (*v7 == ' ');
        }
        dat_17128 = v7;
        dat_1711c = (int)strlen(v7);
        v5 = &v7[dat_1711c];
        if (v7 < v5) {
          do {
            if (v5[-1] != ' ') break;
            v5 = &v5[-1];
          } while (v7 != v5);
        }
        *v5 = '\0';
        dat_17114 = (int)v5 - (int)v7;
        break;
      case 0x73:
        dat_17131 = 1;
        break;
      case 0x74:
        dat_17132 = 1;
        break;
      case 0x75:
        dat_17130 = 1;
        break;
      case 0x77:
        v12 = optarg;
      
    }
  }
  if (v12) { // branch-flip
    v2 = sub_74a0(v12,0,0x9c4,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = v2;
    if (v11) // branch-flip
      dat_17110 = sub_74a0(v11,0,(long)v2,"",dcgettext(NULL,"invalid width",5),0);
    else {
label_2a19:
      dat_17110 = (dat_17120 * 0xbb) / 200;
    }
  }
  else {
    if (!v11) goto label_2a19;
    dat_17110 = sub_74a0(v11,0,0x4b,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = dat_17110 + 10;
  }
  if (optind != argc) { // branch-flip
    v1 = 0;
    v9 = 1;
    v2 = optind;
    if (argc <= optind) {
      return (v9 ^ 1) & 0xff; // return-dupe
    }
    do {
      v7 = v10[v2];
      if (strcmp(v7,"-")) { // branch-flip
        v6 = fopen(v7,"r");
        if (v6) // branch-flip
          v9 &= sub_3700(v6,v7);
        else {
          v8 = sub_6030(4,v7);
          v7 = dcgettext(NULL,"cannot open %s for reading",5);
          v9 = 0;
          error(0,*__errno_location(),v7,v8);
        }
      }
      else {
        v3 = sub_3700(stdin,v7);
        v1 = 1;
        v9 &= v3;
      }
      v2 = optind + 1;
      optind = v2;
    } while (v2 < argc);
    if (!v1) {
      return (v9 ^ 1) & 0xff;
    }
  }
  else {
    v9 = sub_3700(stdin,"-");
  }
  v2 = sub_41f0(stdin);
  if (v2) {
    v7 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v7);
    return v4;
  }
  return (v9 ^ 1) & 0xff;
}

// Function: sub_2bc0 @ 0x2bc0
void sub_2bc0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_bfd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_2bf0 @ 0x2bf0
void sub_2bf0(void)
{
  return;
}

// Function: _FINI_0 @ 0x2c60
void _FINI_0(void)
{
  if (!dat_c0c8) {
    if (dat_bff8)
      __cxa_finalize(dat_c008);
    sub_2bf0();
    dat_c0c8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x2ca0
void _INIT_0(void)
{
  return;
}

// Function: sub_2cb0 @ 0x2cb0
void sub_2cb0(void)
{
  unsigned char v1;
  unsigned long v10;
  long v11;
  long v12; // r14
  int v13;
  unsigned int v2;
  int v3;
  char v4 [16];
  char v5 [16];
  unsigned long v6;
  unsigned long v7;
  long v8;
  unsigned long v9; // rdx
  
  v2 = *(unsigned int *)(dat_c100 + 8);
  *(unsigned long *)(dat_c100 + 0x18) = 0;
  *(int *)(dat_c100 + 8) = dat_17120;
  v7 = dat_c100;
  while (v6 = v7 - 0x28, 0xc120 <= v6) {
    v3 = *(int *)(v7 - 0x20);
    v13 = dat_c0ec;
    if (v6 == 0xc120)
      v13 = dat_c0f0;
    v11 = 0x7fffffffffffffff;
    v13 += v3;
    v10 = v6;
    do {
      v9 = v10 + 0x28;
      if (dat_c100 != v9) { // branch-flip
        v8 = (long)((dat_17110 - v13) * 10);
        v8 *= v8;
        if (dat_c100 != *(unsigned long *)(v10 + 0x48)) {
          v12 = (long)((v13 - *(int *)(v10 + 0x3c)) * 10);
          v8 += v12 * v12 >> 1;
        }
      }
      else {
        v8 = 0;
      }
      v8 += *(long *)(v10 + 0x40);
      if ((v6 == 0xc120) && (1 <= dat_c0e0)) {
        v12 = (long)((v13 - dat_c0e0) * 10);
        v8 += v12 * v12 >> 1;
      }
      if (v8 < v11) {
        *(unsigned long *)(v7 - 8) = v9;
        *(int *)(v7 - 0x14) = v13;
        v11 = v8;
      }
    } while ((dat_c100 != v9) && (v13 = v13 + *(int *)(v10 + 0x30) + *(int *)(v10 + 0xc), v10 = v9, v13 < dat_17120));
    v8 = 0x1324;
    if (0xc121 <= v6) {
      v1 = *(unsigned char *)(v7 - 0x40);
      if (v1 & 2) // branch-flip
        v8 = (unsigned long)(-(unsigned int)((v1 & 8) == 0) & 0x58804) + 0x960;
      else {
        v8 = 0xce4;
        if (((!(v1 & 4)) && (v8 = 0x1324, 0xc149 <= v6)) && (*(unsigned char *)(v7 - 0x68) & 8)) {
          v5._8_8_ = 0;
          v5._0_8_ = SUB168(SEXT816(40000),8);
          v8 = SUB168((v5._0_16_ << 0x40 | ZEXT816(40000)) / SEXT816((long)(*(int *)(v7 - 0x48) + 2)),0) + 0x1324;
        }
      }
    }
    if (*(unsigned char *)(v7 - 0x18) & 1) // branch-flip
      v8 -= 0x640;
    else if (*(unsigned char *)(v7 - 0x18) & 8) {
      v4._8_8_ = 0;
      v4._0_8_ = SUB168(SEXT816(0x57e4),8);
      v8 += SUB168((v4._0_16_ << 0x40 | ZEXT816(0x57e4)) / SEXT816((long)(v3 + 2)),0);
    }
    *(long *)(v7 - 0x10) = v11 + v8;
    v7 = v6;
  }
  *(unsigned int *)(dat_c100 + 8) = v2;
}

// Function: sub_2ea0 @ 0x2ea0
void sub_2ea0(FILE *a0,unsigned int a1)
{
  int v1;
  unsigned char *v2;
  
  do {
    while (a1 != 0x20) {
      if (a1 != 9)
        return;
      dat_c0f8 = 1;
      v1 = dat_1710c + 7;
      if (0 <= dat_1710c)
        v1 = dat_1710c;
      dat_1710c = (v1 >> 3) * 8 + 8;
      v2 = *(unsigned char **)&a0->field_0x8;
      if (v2 < *(unsigned char **)&a0->field_0x10) goto label_2ee4;
label_2f0d:
      a1 = __uflow(a0);
    }
    dat_1710c += 1;
    v2 = *(unsigned char **)&a0->field_0x8;
    if (*(unsigned char **)&a0->field_0x10 <= v2) goto label_2f0d;
label_2ee4:
    *(unsigned char **)&a0->field_0x8 = &v2[1];
    a1 = (unsigned int)*v2;
  } while( true );
}

// Function: sub_2f30 @ 0x2f30
void sub_2f30(FILE *a0) // return-dupe
{
  unsigned char v1;
  unsigned char *v2;
  unsigned int v3;
  unsigned int v4; // eax
  unsigned long v5;
  unsigned char *v6;
  
  v6 = *(unsigned char **)&a0->field_0x8;
  dat_1710c = 0;
  if (*(unsigned char **)&a0->field_0x10 <= v6) // branch-flip
    v4 = __uflow(a0);
  else {
    *(unsigned char **)&a0->field_0x8 = &v6[1];
    v4 = (unsigned int)*v6;
  }
  v5 = sub_2ea0(a0,v4);
  v3 = (unsigned int)v5;
  if (!dat_17114) {
    if (dat_1710c <= dat_17118) {
      dat_c0e4 = dat_1710c;
      return;
    }
    dat_c0e4 = dat_17118;
    return;
  }
  v1 = *dat_17128;
  dat_c0e4 = dat_1710c;
  v6 = dat_17128;
  while( true ) {
    if (!v1) {
      sub_2ea0(a0,v3); // tail-call
      return;
    }
    if ((unsigned int)v1 != (unsigned int)v5) break;
    dat_1710c += 1;
    v2 = *(unsigned char **)&a0->field_0x8;
    if (*(unsigned char **)&a0->field_0x10 <= v2) // branch-flip
      v5 = (unsigned long)(unsigned int)__uflow(a0);
    else {
      *(unsigned char **)&a0->field_0x8 = &v2[1];
      v5 = (unsigned long)*v2;
    }
    v3 = (unsigned int)v5;
    v1 = v6[1];
    v6 = &v6[1];
  }
}

// Function: sub_3000 @ 0x3000
void sub_3000(struct_0 *a0)
{
  unsigned char v1;
  unsigned int v2;
  unsigned char *v3;
  unsigned char *v4;
  unsigned char *v5; // rbx
  int v6;
  
  v2 = a0->field_0x8;
  v3 = a0->field_0x0;
  v6 = 0;
  if (v2) {
    v4 = v3;
    do {
      v5 = &v4[1];
      v1 = *v4;
      v4 = *(unsigned char **)&stdout->field_0x28;
      if (*(unsigned char **)&stdout->field_0x30 <= v4) // branch-flip
        __overflow(stdout,(unsigned int)v1);
      else {
        *(unsigned char **)&stdout->field_0x28 = &v4[1];
        *v4 = v1;
      }
      v4 = v5;
    } while (v5 != &v3[v2]);
    v6 = a0->field_0x8;
  }
  dat_17108 += v6;
}

// Function: sub_3060 @ 0x3060
void sub_3060(int a0)
{
  int v1;
  char *v2;
  int v3;
  int v4;
  int v5; // eax
  
  v1 = dat_17108 + a0;
  v4 = dat_17108;
  if (dat_c0f8) {
    v3 = v1 + 7;
    if (0 <= v1)
      v3 = v1;
    if (((int)(v3 & 0xfffffff8) <= dat_17108 + 1) || ((int)(v3 & 0xfffffff8) <= dat_17108)) goto label_30df;
    do {
      v2 = *(char **)&stdout->field_0x28;
      if (*(char **)&stdout->field_0x30 <= v2) // branch-flip
        __overflow(stdout,9);
      else {
        *(char **)&stdout->field_0x28 = &v2[1];
        *v2 = 9;
      }
      v4 = dat_17108 + 7;
      if (0 <= dat_17108)
        v4 = dat_17108;
      v5 = (v4 >> 3) + 1;
      dat_17108 = v5 * 8;
      v4 = dat_17108;
    } while (v5 < v3 >> 3);
  }
label_30df:
  while (v4 < v1) {
    v2 = *(char **)&stdout->field_0x28;
    if (*(char **)&stdout->field_0x30 <= v2) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)&stdout->field_0x28 = &v2[1];
      *v2 = 0x20;
    }
    dat_17108 += 1;
    v4 = dat_17108;
  }
}

// Function: sub_3140 @ 0x3140
void sub_3140(long a0,int a1)
{
  long v1;
  char *v2;
  
  dat_17108 = 0;
  sub_3060(dat_c0f4);
  fputs_unlocked(dat_17128,stdout);
  dat_17108 = dat_17114 + dat_17108;
  sub_3060(a1 - dat_17108);
  v1 = *(long *)(a0 + 0x20);
  for (; a0 != v1 + -0x28; a0 = a0 + 0x28) {
    sub_3000(a0);
    sub_3060(*(unsigned int *)(a0 + 0xc));
  }
  sub_3000(a0);
  dat_c0e0 = dat_17108;
  v2 = *(char **)&stdout->field_0x28;
  if (v2 < *(char **)&stdout->field_0x30) {
    *(char **)&stdout->field_0x28 = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}

// Function: sub_3200 @ 0x3200
void sub_3200(void)
{
  long *v1;
  long v2;
  void *v3; // rax
  long v4; // rax
  long *v5;
  long v6;
  long *v7;
  long *v8;
  
  v5 = dat_c100;
  if (dat_c100 == (long *)0xc120) {
    fwrite_unlocked((void *)0x15d80,1,dat_15d60 - 0x15d80,stdout);
    dat_15d60 = 0x15d80;
    return;
  }
  sub_2cb0();
  v8 = v5;
  if (v5 != dat_c140) {
    v2 = 0x7fffffffffffffff;
    v7 = dat_c140;
    v6 = dat_c140[3];
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
  sub_3140(0xc120,dat_c0f0);
  v5 = dat_c140;
  v2 = dat_15d60;
  while (dat_15d60 = v2, v8 != v5) {
    sub_3140(v5,dat_c0ec);
    v5 = (long *)v5[4];
    v2 = dat_15d60;
  }
  v3 = __memmove_chk((void *)0x15d80,(void *)*v8,v2 - *v8,5000);
  v6 = *v8;
  v4 = (long)((int)v6 - (int)v3);
  dat_15d60 = v2 - v4;
  v5 = v8;
  if (v8 <= dat_c100) {
    while( true ) {
      v7 = &v5[5];
      *v5 = v6 - v4;
      if (dat_c100 < v7) break;
      v6 = *v7;
      v5 = v7;
    }
  }
  v2 = (long)dat_c100 - (long)&v8[-0x1824];
  __memmove_chk((void *)0xc120,v8,(long)dat_c100 + (0x28U - (long)v8),40000);
  dat_c100 = (long *)v2;
}

// Function: sub_3390 @ 0x3390
void sub_3390(bool a0) // return-dupe x2
{
  if (dat_17131) {
    dat_c0ec = dat_c0f0;
    return;
  }
  if (dat_17133) {
    if (a0) {
      dat_c0ec = dat_1710c;
      return;
    }
    dat_c0ec = dat_c0f0;
    return;
  }
  if (!dat_17132) {
    dat_c0ec = dat_c0f0;
    return;
  }
  if ((a0) && (dat_c0f0 != dat_1710c)) {
    dat_c0ec = dat_1710c;
    return;
  }
  if (dat_c0f0 != dat_c0ec)
    return;
  dat_c0ec = -(unsigned int)(dat_c0f0 == 0) & 3;
}

// Function: sub_3420 @ 0x3420
void sub_3420(FILE *a0,unsigned int a1)
{
  unsigned char *v1;
  long *v10;
  char *v11;
  char *v2;
  char *v3;
  bool v4; // al
  int v5; // eax
  long *v6; // rax
  char *v7;
  char *v8; // rax
  int v9; // eax
  
  v6 = __ctype_b_loc();
  v10 = dat_c100;
label_3450:
  do {
    *v10 = (long)dat_15d60;
    v7 = dat_15d60;
    v3 = dat_15d60;
    do {
      while( true ) {
        dat_15d60 = v7;
        if (dat_15d60 == (char *)0x17108) {
          dat_15d60 = v3;
          sub_3390(1);
          sub_3200();
        }
        v7 = &dat_15d60[1];
        *dat_15d60 = (char)a1;
        v1 = *(unsigned char **)&a0->field_0x8;
        dat_15d60 = v7;
        if (*(unsigned char **)&a0->field_0x10 <= v1) { // branch-flip
          a1 = __uflow(a0);
          v7 = dat_15d60;
          if (a1 == 0xffffffff) goto label_34a7;
        }
        else {
          *(unsigned char **)&a0->field_0x8 = &v1[1];
          a1 = (unsigned int)*v1;
        }
        v3 = dat_15d60;
        if ((int)a1 <= 0xd) break;
        if (a1 == 0x20) goto label_34a7;
      }
    } while ((int)a1 <= 8);
label_34a7:
    v10 = dat_c100;
    v2 = (char *)*dat_c100;
    v5 = (int)v7 - (int)v2;
    *(int *)&dat_c100[1] = v5;
    v9 = dat_1710c + v5;
    v11 = &v2[(long)v5 + -1];
    dat_1710c = v9;
    v8 = strchr("([\'`\"",(int)*v2);
    *(unsigned char *)&v10[2] = *(unsigned char *)(*v6 + (unsigned long)(unsigned char)*v11 * 2) & 4 | v8 != NULL | *(unsigned char *)&v10[2] & 0xfa;
    if (v11 <= v2) // branch-flip
      v5 = (int)*v11;
    else {
      while( true ) {
        v5 = (int)*v11;
        if (!strchr(")]\'\"",v5)) break;
        if (v2 == &v11[-1]) {
          v5 = (int)v11[-1];
          break;
        }
        v11 = &v11[-1];
      }
    }
    *(unsigned char *)&v10[2] = *(unsigned char *)&v10[2] & 0xfd | (strchr(".?!",v5) != NULL) * '\x02';
    a1 = sub_2ea0(a0,a1);
    v9 = dat_1710c - v9;
    *(int *)((long)dat_c100 + 0xc) = v9;
    if (a1 != 0xffffffff) { // branch-flip
      if (*(unsigned char *)&dat_c100[2] & 2) { // branch-flip
        v4 = 1 < v9 || a1 == 10;
        if (1 < v9 || a1 == 10) goto label_35b7;
        *(unsigned char *)&dat_c100[2] = *(unsigned char *)&dat_c100[2] & 0xf7;
      }
      else {
        v4 = 0;
label_35b7:
        *(unsigned char *)&dat_c100[2] = *(unsigned char *)&dat_c100[2] & 0xf7 | v4 * '\b';
        if (a1 == 10) goto label_363c;
      }
      if (dat_17130) goto label_363c;
      if (dat_c100 != (long *)0x15d10) {
        v10 = &dat_c100[5];
        dat_c100 = v10;
        goto label_3450;
      }
label_36d0:
      sub_3390(1);
      sub_3200();
    }
    else {
      *(unsigned char *)&dat_c100[2] = *(unsigned char *)&dat_c100[2] | 8;
label_363c:
      *(unsigned int *)((long)dat_c100 + 0xc) = ((*(unsigned char *)&dat_c100[2] & 8) != 0) + 1;
      if (dat_c100 == (long *)0x15d10) goto label_36d0;
    }
    v10 = &dat_c100[5];
    dat_c100 = v10;
    if ((a1 == 10) || (a1 == 0xffffffff)) {
      sub_2f30(a0); // tail-call
      return;
    }
  } while( true );
}

// Function: sub_3700 @ 0x3700
unsigned long sub_3700(FILE *a0,char *a1) // return-dupe
{
  unsigned char v1;
  unsigned long v10;
  int v11;
  unsigned long v12;
  unsigned char *v13;
  bool v14;
  unsigned char *v2;
  char *v3;
  unsigned int v4;
  int v5;
  unsigned int v6; // eax
  unsigned long v7; // rax
  char *v8; // rax
  int v9; // ecx
  
  sub_41c0(a0,2);
  dat_c0f8 = 0;
  dat_c0ec = 0;
  v4 = sub_2f30(a0);
  dat_c0e8 = v4;
label_3746:
  dat_c0e0 = 0;
  do {
    if ((v4 != 10) && (v4 != 0xffffffff)) {
      if ((dat_17118 <= (int)dat_c0e4) && ((int)(dat_1711c + dat_c0e4) <= dat_1710c)) goto label_3a07;
      dat_17108 = 0;
      if (dat_1710c <= (int)dat_c0e4) {
        sub_3060(dat_c0e4);
        v11 = dat_1710c;
        if (dat_17108 != dat_1710c) {
label_393a:
          v14 = 1;
          v13 = dat_17128;
          v5 = dat_17108;
          goto label_37df;
        }
        sub_3060(0);
label_3981:
        do {
          v3 = *(char **)&stdout->field_0x28;
          if (*(char **)&stdout->field_0x30 <= v3) // branch-flip
            __overflow(stdout,v4 & 0xff);
          else {
            *(char **)&stdout->field_0x28 = &v3[1];
            *v3 = (char)v4;
          }
          v13 = *(unsigned char **)&a0->field_0x8;
          if (*(unsigned char **)&a0->field_0x10 <= v13) { // branch-flip
            v4 = __uflow(a0);
            v14 = v4 != 0xffffffff;
          }
          else {
            *(unsigned char **)&a0->field_0x8 = &v13[1];
            v4 = (unsigned int)*v13;
            v14 = 1;
          }
        } while ((v4 != 10) && (v14));
        goto label_38db;
      }
      v14 = 1;
label_379e:
      dat_17108 = 0;
      sub_3060(dat_c0e4);
      v13 = dat_17128;
      v5 = dat_17108;
      v11 = dat_1710c;
      if (dat_1710c != dat_17108) {
label_37df:
        do {
          v1 = *v13;
          if (!v1) break;
          v13 = &v13[1];
          v2 = *(unsigned char **)&stdout->field_0x28;
          if (*(unsigned char **)&stdout->field_0x30 <= v2) { // branch-flip
            __overflow(stdout,(unsigned int)v1);
            v11 = dat_1710c;
          }
          else {
            *(unsigned char **)&stdout->field_0x28 = &v2[1];
            *v2 = v1;
          }
          dat_17108 += 1;
          v5 = dat_17108;
        } while (dat_17108 != v11);
      }
      if (v14) {
        sub_3060(v11 - v5);
label_3955:
        if (v4 != 0xffffffff) goto label_3981;
label_3822:
        if ((int)(dat_17114 + dat_c0e4) <= dat_1710c) {
          v3 = *(char **)&stdout->field_0x28;
          if (*(char **)&stdout->field_0x30 <= v3) // branch-flip
            __overflow(stdout,10);
          else {
            *(char **)&stdout->field_0x28 = &v3[1];
            *v3 = 10;
          }
        }
        goto label_383a;
      }
      if (v4 == 0xffffffff) goto label_3822;
    }
    else {
      dat_17108 = 0;
      v14 = v4 != 10 && v4 != 0xffffffff;
      if ((int)dat_c0e4 < dat_1710c) goto label_379e;
      if (v14) {
        sub_3060(dat_c0e4);
        v11 = dat_1710c;
        if (dat_17108 != dat_1710c) goto label_393a;
        sub_3060(0);
        goto label_3955;
      }
label_38db:
      if (v4 == 0xffffffff) {
label_383a:
        dat_c0e8 = 0xffffffff;
        if (*(unsigned char *)a0 & 0x20) { // branch-flip
          if (a0 != stdin) // branch-flip
            sub_41f0(a0);
          else {
            clearerr_unlocked(a0);
          }
          v7 = sub_63b0(0,3,a1);
        }
        else {
          if (a0 == stdin) {
            clearerr_unlocked(a0);
            return 1;
          }
          if (!sub_41f0(a0))
            return 1;
          v11 = *__errno_location();
          if (v11 < 0)
            return 1;
          v7 = sub_63b0(0,3,a1);
          if (v11) {
            v8 = "%s";
            error(0,v11,v8,v7); // return-dupe
            return 0;
          }
        }
        v11 = 0;
        v8 = dcgettext(NULL,"read error",5);
        error(0,v11,v8,v7);
        return 0;
      }
    }
    v3 = *(char **)&stdout->field_0x28;
    if (*(char **)&stdout->field_0x30 <= v3) // branch-flip
      __overflow(stdout,10);
    else {
      *(char **)&stdout->field_0x28 = &v3[1];
      *v3 = 10;
    }
    v4 = sub_2f30(a0);
  } while( true );
label_3a07:
  dat_c0f4 = dat_c0e4;
  dat_c0f0 = dat_1710c;
  dat_15d60 = 0x15d80;
  dat_c100 = 0xc120;
  v4 = sub_3420(a0,v4);
  v10 = (unsigned long)dat_c0e4;
  v14 = 0;
  v12 = (unsigned long)v4;
  if ((dat_c0e4 == dat_c0f4) && ((int)(dat_1711c + dat_c0e4) <= dat_1710c))
    v14 = v4 != 0xffffffff && v4 != 10;
  v4 = dat_c0f4;
  sub_3390(v14);
  v6 = (unsigned int)v12;
  if (!dat_17131) {
    v9 = (unsigned int)v10;
    if (dat_17133) { // branch-flip
      if (((v9 == v4) && ((int)(v9 + dat_1711c) <= dat_1710c)) && ((v6 != 10 && (v6 != 0xffffffff)))) {
        while (((v6 = sub_3420(a0), dat_c0e4 == dat_c0f4 && ((int)(dat_c0e4 + dat_1711c) <= dat_1710c)) && (v6 != 10))) {
          if ((v6 == 0xffffffff) || (dat_c0ec != dat_1710c)) break;
        }
      }
    }
    else if (dat_17132) { // branch-flip
      if (((v9 == v4) && ((int)(v9 + dat_1711c) <= dat_1710c)) && ((v6 != 10 && ((v6 != 0xffffffff && (dat_c0f0 != dat_1710c)))))) {
        while (v6 = sub_3420(a0), dat_c0e4 == dat_c0f4) {
          if ((((dat_1710c < (int)(dat_c0e4 + dat_1711c)) || (v6 == 10)) || (v6 == 0xffffffff)) || (dat_c0ec != dat_1710c)) break;
        }
      }
    }
    else if (v9 == v4) {
      do {
        v6 = (unsigned int)v12;
        if ((((dat_1710c < (int)v10 + dat_1711c) || (v6 == 10)) || (v6 == 0xffffffff)) || (dat_c0ec != dat_1710c)) break;
        v6 = sub_3420(a0);
        v10 = (unsigned long)dat_c0e4;
        v12 = (unsigned long)v6;
      } while (dat_c0e4 == dat_c0f4);
    }
  }
  v10 = dat_c100;
  if (dat_c100 <= 0xc120)
    __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph"); // no-return
  *(unsigned char *)(dat_c100 - 0x18) = *(unsigned char *)(dat_c100 - 0x18) | 10;
  dat_c0e8 = v6;
  sub_2cb0();
  sub_3140(0xc120,dat_c0f0);
  v4 = dat_c0e8;
  v12 = dat_c140;
  while (dat_c0e8 = v4, v12 != v10) {
    sub_3140(v12,dat_c0ec);
    v12 = *(unsigned long *)(v12 + 0x20);
    v4 = dat_c0e8;
  }
  goto label_3746;
}

// Function: sub_3d70 @ 0x3d70
void sub_3d70(int a0)
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
  
  v6 = dat_17148;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5),v6);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n",5),v1);
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
      if (!strcmp("fmt",v6)) break;
      v6 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v6);
    v6 = *(char **)((long)v4 + 8);
    if (!v6)
      v6 = "fmt";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "fmt";
    if (!strcmp("fmt","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "fmt")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}

// Function: sub_40e0 @ 0x40e0
void sub_40e0(unsigned long a0)
{
  dat_17140 = a0;
}

// Function: sub_40f0 @ 0x40f0
void sub_40f0(char a0)
{
  dat_17138 = a0;
}

// Function: sub_4100 @ 0x4100
void sub_4100(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_7900(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_17138) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_17140) { // branch-flip
        v4 = (char *)sub_6290(dat_17140);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_c018); // return-dupe, no-return
    }
  }
  v1 = sub_7900(stderr);
  if (!v1)
    return;
  _exit(dat_c018);
}

// Function: sub_41b0 @ 0x41b0
void sub_41b0(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_41c0 @ 0x41c0
void sub_41c0(FILE *a0,unsigned int a1)
{
  if (a0) {
    posix_fadvise(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_41f0 @ 0x41f0
unsigned long sub_41f0(FILE *a0) // early-return
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
    if (sub_4280(a0)) {
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

// Function: sub_4280 @ 0x4280
void sub_4280(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_42d0(a0,0,1);
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
    dat_17148 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_43f0 @ 0x43f0
char * sub_43f0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_7970();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x879d;
      return (char *)0x8798;
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

// Function: sub_44d0 @ 0x44d0
long * sub_44d0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_4540:
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
label_49e6:
      v2 = 0;
      goto label_49a0;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_4b01;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_49a0:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_4b01;
      }
      goto label_49e6;
    case 5:
      if (!v18) goto label_4941;
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
          v10 = (char *)sub_43f0("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)"\'") {
          v10 = (char *)sub_43f0("\'",a4);
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
label_462b:
  do {
label_463e:
    v23 = 0;
    a1 = v14;
label_4648:
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
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_4fb0;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_4bac;
                default:
                  goto label_4883;
                case 7:
                  goto label_4b97;
                case 8:
                  goto label_4b66;
                case 9:
                  goto label_4c33;
                case 10:
                  goto label_4b8d;
                case 0xb:
                  goto label_4c60;
                case 0xc:
                  goto label_4b4b;
                case 0xd:
                  goto label_4a9d;
                case 0x20:
                  goto label_4c6d;
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
                  goto label_4836;
                case 0x23:
                  goto label_4c10;
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
                  goto label_4768;
                case 0x27:
                  goto label_4a82;
                case 0x3f:
                  goto label_4a41;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_4883;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_4768;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_5054;
              goto label_4f98;
            }
            if (v8 == 0x7d) goto label_5120;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_4f7a;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_4f98;
              goto label_4883;
            }
            if (v8 != 0x7e) goto label_4883;
label_50d4:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_48f9;
            }
            v21 = 0x7e;
            goto label_50e2;
          }
          goto label_486c;
        }
label_4fb0:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_50d4;
                goto label_4880;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_4883;
                goto label_4f98;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_4f7a:
            if (a3 == 0xffffffffffffffff) goto label_512f;
label_4f88:
            v5 = v22;
            if (a3 == 1) goto label_4c15;
label_4f98:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_48f9;
          }
          if (v8 == 0x40) goto label_4880;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_4768;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_4f98;
label_5054:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_507a;
            v17 = 0x5c;
            goto label_4aa2;
          }
label_4883:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_4e03;
label_4890:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_48de;
        }
        switch(v8) {
          case 0:
            goto label_4b9e;
          default:
label_4880:
            v22 = 0;
            goto label_4883;
          case 7:
label_4b97:
            v17 = 0x61;
            goto label_4b77;
          case 8:
label_4b66:
            v17 = 0x62;
            goto label_4b77;
          case 9:
            v22 = 0;
label_4c33:
            v21 = 9;
            v17 = 0x74;
            goto label_4c40;
          case 10:
label_4b8d:
            v17 = 0x6e;
            break;
          case 0xb:
label_4c60:
            v17 = 0x76;
            break;
          case 0xc:
label_4b4b:
            v17 = 0x66;
label_4b77:
            if (v25) {
              v2 = 1;
              goto label_4acf;
            }
            v24 = 0;
            goto label_47ee;
          case 0xd:
label_4a9d:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_4c6d:
            v21 = 0x20;
            goto label_4768;
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
            goto label_4839;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_4c15;
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
label_4768:
            v8 = 0;
            goto label_4778;
          case 0x27:
            v22 = 0;
            goto label_4a82;
          case 0x3f:
            v22 = 0;
            goto label_4a41;
          
        }
        goto label_4aa2;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_5120:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_4f88;
label_512f:
            v5 = v22;
            if (a2[1]) goto label_4f98;
            goto label_4c15;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_4836;
              goto label_4883;
            }
            goto label_4f7a;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_4c15;
          if (v33 == 1) goto label_4890;
label_4e03:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_79b0(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_536b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_56ef;
              goto label_56e5;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_4f3d;
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
          goto label_536b;
        }
        if (v17 == 0x40) goto label_4883;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_48f0;
        if ((unsigned long)v10 & 0xa4000000) goto label_4836;
        if (v17 != 0x5c) goto label_4883;
        if (a4 == 2) {
          if (!v25) goto label_507a;
          goto label_4863;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_4c40;
        }
label_507a:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_5090;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_4d12;
            goto label_4c4b;
          }
label_4b9e:
          if (!v25) {
            v22 = 0;
label_4bac:
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
                goto label_4923;
              }
label_54b5:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_54b5;
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
label_54ec:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_4923;
              goto label_4778;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_4923;
          }
          if (a4 != 2) goto label_486c;
          goto label_4adb;
        default:
          goto label_4883;
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
          goto label_4ccf;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_4ccf;
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
label_4ccf:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_4c40;
label_4851:
          a4 = 2;
label_4863:
          if (!v28) goto label_486c;
          goto label_4adb;
        case 0x20:
          v19 = 0x20;
          goto label_4839;
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
label_4836:
          v26 = 0;
label_4839:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_4851;
          goto label_48f7;
        case 0x23:
label_4c10:
          v21 = 0x23;
label_4c15:
          if (!v23) {
            v19 = v21;
            goto label_4839;
          }
label_50e2:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_48f9;
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
          goto label_48f0;
        case 0x27:
label_4a82:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_48f9;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_550b;
            }
            else {
label_550b:
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
            goto label_4923;
          }
          goto label_4863;
        case 0x3f:
label_4a41:
          if (a4 == 2) {
            if (v25) goto label_4863;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_4923;
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
                goto label_48f9;
              }
              if (v25) goto label_486c;
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
              goto label_54ec;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_48f9;
        
      }
label_4c40:
      if (!v2) {
label_4c4b:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_4920;
        goto label_4778;
      }
label_4aa2:
      v24 = 0;
      goto label_4ab3;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_57ab:
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
        if (a1 || !v32) goto label_57ab;
        v23 = v32;
        goto label_523b;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_585b;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_463e;
    }
label_4f3d:
    if (v2) {
label_4adb:
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
label_4b01:
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
label_523b:
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
label_56e5:
    if (!a2[(long)v10]) break;
  }
label_56ef:
  v24 = 0;
label_536b:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_5375:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_4863;
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
        if (v21 <= v23) goto label_4801;
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
          goto label_5090;
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
label_48de:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_5375;
  }
label_48f0:
  v21 = v19;
  v24 = v26;
label_48f7:
  v8 = a4 == 2;
  v26 = v3;
label_48f9:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_4778:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_4923;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_4923;
  }
  else {
label_4920:
    v8 = 0;
    v3 = v26;
label_4923:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_5090:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_4801;
    }
  }
label_4ab3:
  if (v25) {
label_4acf:
    if ((bool)(v2 & a4 == 2)) goto label_4adb;
label_486c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_4540;
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
label_47ee:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_4801:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_4648;
label_4d12:
  v23 += 1;
  goto label_4648;
label_585b:
  a1 = v32;
label_4941:
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
  goto label_462b;
}

// Function: sub_5960 @ 0x5960
char * sub_5960(unsigned int a0,char *a1,unsigned long a2,struct_1 *a3)
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
    v5 = dat_c060;
    if (dat_c058 <= (int)a0) {
      v10 = (long)dat_c058;
      v8 = (long)(int)((a0 - dat_c058) + 1);
      if (dat_c060 != (unsigned long *)0xc070) // branch-flip
        v5 = (unsigned long *)sub_7130(dat_c060,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_7130(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_c070;
        v5[1] = dat_c078;
      }
      dat_c060 = v5;
      memset(&v5[(long)dat_c058 * 2],0,(v10 - dat_c058) * 0x10);
      dat_c058 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_44d0(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x17160)
        free(v7);
      v7 = (char *)sub_6e90(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_44d0(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_5b50 @ 0x5b50
void sub_5b50(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x17260;
  sub_7290(a0,0x38);
  *v2 = v1;
}

// Function: sub_5b90 @ 0x5b90
unsigned int sub_5b90(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x17260;
  return *a0;
}

// Function: sub_5bb0 @ 0x5bb0
void sub_5bb0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x17260;
  *a0 = a1;
}

// Function: sub_5bd0 @ 0x5bd0
unsigned int sub_5bd0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x17260;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_5c10 @ 0x5c10
unsigned int sub_5c10(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x17260;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_5c30 @ 0x5c30
void sub_5c30(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x17260;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_5c70 @ 0x5c70
void sub_5c70(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x17260;
  v2 = __errno_location();
  v1 = *v2;
  sub_44d0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_5cf0 @ 0x5cf0
char * sub_5cf0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x17260;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_44d0(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_6e90(v3 + 1U);
  sub_44d0(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_5de0 @ 0x5de0
char * sub_5de0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_5cf0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_5df0 @ 0x5df0
void sub_5df0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_c060;
  if (2 <= dat_c058) {
    v4 = (unsigned long *)((long)dat_c060 + 0x18);
    v1 = (unsigned long *)((long)dat_c060 + (unsigned long)(unsigned int)(dat_c058 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x17160) {
    free(*(void **)((long)v3 + 8));
    dat_c078 = 0x17160;
    dat_c070 = 0x100;
  }
  if (v3 == (void *)0xc070) {
    dat_c058 = 1;
    return;
  }
  free(v3);
  dat_c058 = 1;
  dat_c060 = (void *)0xc070;
}

// Function: sub_5e90 @ 0x5e90
char * sub_5e90(unsigned int a0,char *a1)
{
  return sub_5960(a0,a1,0xffffffffffffffff,(struct_1 *)0x17260); // tail-call
}

// Function: sub_5eb0 @ 0x5eb0
char * sub_5eb0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_5960(a0,a1,a2); // tail-call
}

// Function: sub_5ec0 @ 0x5ec0
char * sub_5ec0(char *a0)
{
  return sub_5960(0,a0,0xffffffffffffffff,(struct_1 *)0x17260); // tail-call
}

// Function: sub_5ee0 @ 0x5ee0
char * sub_5ee0(char *a0,unsigned long a1)
{
  return sub_5960(0,a0,a1,(struct_1 *)0x17260); // tail-call
}

// Function: sub_5f00 @ 0x5f00
void sub_5f00(unsigned int a0,int a1,char *a2)
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
    sub_5960(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_5fa0 @ 0x5fa0
void sub_5fa0(unsigned int a0,int a1,char *a2,unsigned long a3)
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
    sub_5960(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6030 @ 0x6030
void sub_6030(int a0,char *a1)
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
    sub_5960(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_60c0 @ 0x60c0
void sub_60c0(int a0,char *a1,unsigned long a2)
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
    sub_5960(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6150 @ 0x6150
void sub_6150(char *a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_17260;
  v4 = dat_17268;
  v9 = dat_17290;
  v5 = dat_17270;
  v6 = dat_17278;
  v7 = dat_17280;
  v8 = dat_17288;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_5960(0,a0,a1,&v3);
}

// Function: sub_61f0 @ 0x61f0
void sub_61f0(char *a0,unsigned char a1)
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
  
  v3 = dat_17260;
  v4 = dat_17268;
  v9 = dat_17290;
  v5 = dat_17270;
  v6 = dat_17278;
  v7 = dat_17280;
  v8 = dat_17288;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_5960(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_6290 @ 0x6290
void sub_6290(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_17260;
  v3 = (unsigned int)((unsigned long)dat_17268 >> 0x20);
  v8 = dat_17290;
  v4 = dat_17270;
  v5 = dat_17278;
  v6 = dat_17280;
  v7 = dat_17288;
  v2 = (unsigned int)dat_17268;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_5960(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_6320 @ 0x6320
void sub_6320(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_17260;
  v3 = (unsigned int)((unsigned long)dat_17268 >> 0x20);
  v8 = dat_17290;
  v4 = dat_17270;
  v5 = dat_17278;
  v6 = dat_17280;
  v7 = dat_17288;
  v2 = (unsigned int)dat_17268;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_5960(0,a0,a1,&v1);
}

// Function: sub_63b0 @ 0x63b0
void sub_63b0(unsigned int a0,int a1,char *a2)
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
    sub_5960(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6450 @ 0x6450
void sub_6450(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_17268;
  v4 = dat_17270;
  v5 = dat_17278;
  v8 = dat_17290;
  v2 = (unsigned int)((unsigned long)dat_17260 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_17280;
  v7 = dat_17288;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_5960(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_64f0 @ 0x64f0
void sub_64f0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_17268;
  v4 = dat_17270;
  v5 = dat_17278;
  v8 = dat_17290;
  v2 = (unsigned int)((unsigned long)dat_17260 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_17280;
  v7 = dat_17288;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_5960(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6590 @ 0x6590
void sub_6590(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_17268;
  v4 = dat_17270;
  v5 = dat_17278;
  v8 = dat_17290;
  v2 = (unsigned int)((unsigned long)dat_17260 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_17280;
  v7 = dat_17288;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_5960(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_6630 @ 0x6630
void sub_6630(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_17268;
  v4 = dat_17270;
  v5 = dat_17278;
  v8 = dat_17290;
  v2 = (unsigned int)((unsigned long)dat_17260 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_17280;
  v7 = dat_17288;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_5960(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_66d0 @ 0x66d0
char * sub_66d0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_5960(a0,a1,a2); // tail-call
}

// Function: sub_66e0 @ 0x66e0
char * sub_66e0(char *a0,unsigned long a1)
{
  return sub_5960(0,a0,a1,(struct_1 *)0xc020); // tail-call
}

// Function: sub_6700 @ 0x6700
char * sub_6700(unsigned int a0,char *a1)
{
  return sub_5960(a0,a1,0xffffffffffffffff,(struct_1 *)0xc020); // tail-call
}

// Function: sub_6720 @ 0x6720
char * sub_6720(char *a0)
{
  return sub_5960(0,a0,0xffffffffffffffff,(struct_1 *)0xc020); // tail-call
}

// Function: sub_6740 @ 0x6740
long sub_6740(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_686d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_68a6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x8e68) + 0x8e68;
  switch(a5) {
    case 0:
      goto label_68a6;
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
      goto label_686d;
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

// Function: sub_6bc0 @ 0x6bc0
void sub_6bc0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_6740(); // tail-call
}

// Function: sub_6be0 @ 0x6be0
void sub_6be0(FILE *a0,char *a1,char *a2,char *a3,struct_2 *a4)
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
  sub_6740(a0,a1,a2,a3);
}

// Function: sub_6c80 @ 0x6c80
void sub_6c80(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_6740(a0,a1,a2);
}

// Function: sub_6d60 @ 0x6d60
void sub_6d60(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_6e10 @ 0x6e10
void sub_6e10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_7350(); // no-return
}

// Function: sub_6e50 @ 0x6e50
void sub_6e50(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_7350(); // no-return
}

// Function: sub_6e70 @ 0x6e70
void sub_6e70(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_7350(); // no-return
}

// Function: sub_6e90 @ 0x6e90
void sub_6e90(void)
{
  sub_6e50(); // tail-call
}

// Function: sub_6ea0 @ 0x6ea0
void sub_6ea0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_7350(); // no-return
}

// Function: sub_6ee0 @ 0x6ee0
void sub_6ee0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_7350(); // no-return
}

// Function: sub_6f10 @ 0x6f10
void sub_6f10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_7350(); // no-return
}

// Function: sub_6f50 @ 0x6f50
void sub_6f50(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_7350(); // no-return
}

// Function: sub_6f90 @ 0x6f90
void sub_6f90(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_7350(); // no-return
}

// Function: sub_6fc0 @ 0x6fc0
void sub_6fc0(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_7350(); // no-return
}

// Function: sub_7010 @ 0x7010
void sub_7010(void *a0,unsigned long *a1)
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
  sub_7350(); // no-return
}

// Function: sub_70a0 @ 0x70a0
void sub_70a0(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_7350(); // no-return
}

// Function: sub_7130 @ 0x7130
void sub_7130(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_71ee;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_71ee:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_7350(); // no-return
}

// Function: sub_7230 @ 0x7230
void sub_7230(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_7350(); // no-return
}

// Function: sub_7250 @ 0x7250
void sub_7250(unsigned long a0)
{
  sub_7230(a0,1); // tail-call
}

// Function: sub_7260 @ 0x7260
void sub_7260(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_7350(); // no-return
}

// Function: sub_7280 @ 0x7280
void sub_7280(unsigned long a0)
{
  sub_7260(a0,1); // tail-call
}

// Function: sub_7290 @ 0x7290
void sub_7290(void *a0,unsigned long a1)
{
  memcpy((void *)sub_6e50(a1),a0,a1); // tail-call
}

// Function: sub_72c0 @ 0x72c0
void sub_72c0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_6e70(a1),a0,a1); // tail-call
}

// Function: sub_72f0 @ 0x72f0
void sub_72f0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_6e70(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_7330 @ 0x7330
void sub_7330(char *a0)
{
  sub_7290(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_7350 @ 0x7350
void sub_7350(void)
{
  error(dat_c018,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_7390 @ 0x7390
unsigned long sub_7390(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5,int a6)
{
  int v1;
  char *v2; // rax
  int *v3; // rax
  unsigned long v4; // stack - 0x48
  
  v1 = sub_74d0(a0,NULL,a1,&v4);
  if (v1) { // branch-flip
    v3 = __errno_location();
    if (v1 != 1) {
      if (v1 == 3)
        *v3 = 0;
      goto label_7438;
    }
  }
  else {
    if ((a2 <= v4) && (v4 <= a3))
      return v4;
    v3 = __errno_location();
    if (v4 <= 0x3fffffff) {
      *v3 = 0x22;
      goto label_7438;
    }
  }
  *v3 = 0x4b;
label_7438:
  v2 = (char *)sub_6720(a0);
  v1 = *v3;
  if (v1 == 0x16) goto label_7470;
  do {
    if (!a6)
      a6 = 1;
    error(a6,v1,"%s: %s",a5,v2);
label_7470:
    v1 = 0;
  } while( true );
}

// Function: sub_74a0 @ 0x74a0
void sub_74a0(unsigned char *a0,unsigned long a1,unsigned long a2,char *a3,char *a4,unsigned long a5)
{
  sub_7390(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_74d0 @ 0x74d0
unsigned int sub_74d0(unsigned char *a0,long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned long v10;
  long v11;
  int v12; // esi
  int v13; // r9d
  unsigned int v14;
  char v2 [16];
  unsigned int v3; // eax
  int *v4; // rax
  long *v5; // rax
  unsigned char *v6;
  unsigned long v7;
  unsigned long v8; // rcx
  unsigned char v9;
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v11;
  v4 = __errno_location();
  *v4 = 0;
  v9 = *a0;
  v5 = __ctype_b_loc();
  v6 = a0;
  while (*(unsigned char *)(*v5 + 1 + (unsigned long)v9 * 2) & 0x20) {
    v9 = v6[1];
    v6 = &v6[1];
  }
  if (v9 == 0x2d)
    return 4;
  v7 = strtoumax(a0,a1,a2);
  v6 = (unsigned char *)*a1;
  if (v6 != a0) { // branch-flip
    if (*v4) { // branch-flip
      v14 = 1;
      if (*v4 != 0x22)
        return 4;
    }
    else {
      v14 = 0;
    }
    if (!a4) goto label_758d;
    v9 = *v6;
    if (!v9) goto label_758d;
    v10 = v7;
    if (!strchr(a4,(int)(char)v9)) goto label_7595;
  }
  else {
    if ((!a4) || (v9 = *a0, !v9))
      return 4;
    v14 = 0;
    v10 = 1;
    if (!strchr(a4,(int)(char)v9))
      return 4;
  }
  v3 = (unsigned int)v9 - 0x45;
  v13 = 1;
  v8 = 0x400;
  if (((unsigned char)v3 <= 0x2f) && (0x814400308945U >> ((unsigned long)v3 & 0x3f) & 1)) {
    v8 = 0x400;
    v13 = 1;
    if (strchr(a4,0x30)) {
      v1 = v6[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v13 = (v1 == 0x42) + 1;
          if (v1 == 0x42)
            v8 = 1000;
        }
        else {
          v13 = (v6[2] == 0x42) + 1 + (unsigned int)(v6[2] == 0x42);
        }
      }
      else {
        v13 = 2;
        v8 = 1000;
      }
    }
  }
  v7 = v10;
  switch((unsigned int)v9 - 0x42 & 0xff) {
    case 0:
      v7 = v10 << 10;
      if (v10 >> 0x36) {
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      break;
    default:
label_7595:
      *a3 = v10;
      return v14 | 2;
    case 3:
      v12 = 6;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_7794;
    case 5:
    case 0x25:
      if ((SUB168(ZEXT816(v10) * ZEXT816(v8),8)) || (v2 = ZEXT816(SUB168(ZEXT816(v10) * ZEXT816(v8),0)) * ZEXT816(v8), v7 = SUB168(v2,0), SUB168(v2,8))) goto label_7759;
label_7727:
      if (SUB168(ZEXT816(v7) * ZEXT816(v8),8)) { // branch-flip
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      else {
        v7 = SUB168(ZEXT816(v7) * ZEXT816(v8),0);
      }
      break;
    case 9:
    case 0x29:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_7759;
      break;
    case 0xb:
    case 0x2b:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (!SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_7727;
label_7759:
      v14 = 1;
      v7 = 0xffffffffffffffff;
      break;
    case 0xe:
      v12 = 5;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_7794;
    case 0x12:
    case 0x32:
      v12 = 4;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_7794;
    case 0x17:
      v12 = 8;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_7794;
    case 0x18:
      v12 = 7;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
label_7794:
      v14 |= v3;
      v7 = v10;
      break;
    case 0x20:
      if (v10 >> 0x37) { // branch-flip
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      else {
        v7 = v10 << 9;
      }
      break;
    case 0x21:
      break;
    case 0x35:
      if (0 <= (long)v10) // branch-flip
        v7 = v10 * 2;
      else {
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
    
  }
  *a1 = (long)&v6[v13];
  if (v6[v13])
    v14 |= 2;
label_758d:
  *a3 = v7;
  return v14;
}

// Function: sub_7900 @ 0x7900
unsigned long sub_7900(FILE *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *(unsigned int *)a0;
  v3 = sub_41f0(a0);
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

// Function: sub_7970 @ 0x7970
char * sub_7970(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_79b0 @ 0x79b0
unsigned long sub_79b0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_7a40(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_7a40 @ 0x7a40
bool sub_7a40(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_7ad0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_7ad0 @ 0x7ad0
unsigned long sub_7ad0(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_7b80 @ 0x7b80
void sub_7b80(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_7b90 @ 0x7b90
void sub_7b90(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_c008); // tail-call
}

// Function: _DT_FINI @ 0x7ba4
void _DT_FINI(void)
{
  return;
}
