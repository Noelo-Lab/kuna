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

