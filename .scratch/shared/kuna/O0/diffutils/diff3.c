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
  void *v18; // stack - 0xf8
  void *v19; // stack - 0xf0
  bool v2;
  unsigned int *v20; // stack - 0xe8
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
        v18 = (void *)sub_43e5(v1,v17,v9,&v8);
        v19 = (void *)sub_43e5(v16[v6[3]],v17,v9,&v10);
        sub_2b58(v19);
        sub_2b58(v18);
        v20 = (unsigned int *)sub_3696(v19,v18);
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
long sub_3696(void *a0,void *a1)
{
  void *v1; // rax
  long *v10; // stack - 0x68
  void *v11; // stack - 0x60
  void *v12; // stack - 0x58
  void *v13; // stack - 0x50
  long v2; // stack - 0x80
  void *v3 [3]; // stack - 0x28
  long v4 [4]; // stack - 0x48
  unsigned int v5; // stack - 0x8c
  unsigned int v6; // stack - 0x88
  unsigned int v7; // stack - 0x84
  long v8; // stack - 0x78
  void *v9; // stack - 0x70
  
  v2 = 0;
  v10 = &v2;
  v3[0] = a0;
  v3[1] = a1;
  v11 = (void *)0xfd80;
  while( true ) {
    if ((!v3[0]) && (!v3[1]))
      return v2;
    v4[3] = 0;
    v4[2] = 0;
    v4[1] = 0;
    v4[0] = 0;
    if (v3[0]) { // branch-flip
      if (v3[1]) // branch-flip
        v6 = (unsigned int)(*(long *)((long)v3[1] + 0x10) < *(long *)((long)v3[0] + 0x10));
      else {
        v6 = 0;
      }
    }
    else {
      v6 = 1;
    }
    v5 = v6;
    v12 = v3[(int)v6];
    v8 = *(long *)((long)v12 + 0x18);
    v4[(long)(int)v6 + 2] = (long)v12;
    v4[(int)v6] = v4[(long)(int)v6 + 2];
    v3[(int)v6] = *(void **)((long)v12 + 0x40);
    *(unsigned long *)(v4[(long)(int)v6 + 2] + 0x40) = 0;
    v7 = v6 ^ 1;
    v9 = v3[(int)v7];
    while ((v9 && (*(long *)((long)v9 + 0x10) <= v8 + 1))) {
      if (v4[(int)v7]) // branch-flip
        *(void **)(v4[(long)(int)v7 + 2] + 0x40) = v9;
      else {
        v4[(int)v7] = (long)v9;
      }
      v4[(long)(int)v7 + 2] = (long)v9;
      v3[(int)v7] = *(void **)((long)v3[(int)v7] + 0x40);
      *(unsigned long *)((long)v9 + 0x40) = 0;
      if (v8 < *(long *)((long)v9 + 0x18)) {
        v5 ^= 1;
        v8 = *(long *)((long)v9 + 0x18);
      }
      v7 = v5 ^ 1;
      v9 = v3[(int)v7];
    }
    v1 = (void *)sub_393d(v4,&v4[2],v6,v5,v11);
    v13 = v1;
    if (!v1) break;
    *v10 = (long)v1;
    v10 = (long *)((long)v1 + 0x68);
    v11 = v1;
  }
  sub_5fcf("internal error: screwup in format of diff blocks"); // no-return
}


// Function: sub_393d @ 0x393d
unsigned int * sub_393d(long *a0,long a1,int a2,int a3,void *a4)
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
      v14[v15] = (v13 - *(long *)((long)a4 + 0x30)) + *(long *)((long)a4 + (long)v15 * 0x10 + 0x10);
      v14[(long)v15 + 2] = (v1 - *(long *)((long)a4 + 0x30)) + *(long *)((long)a4 + (long)v15 * 0x10 + 0x10);
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
char sub_54de(FILE *a0,void *a1,int *a2,long a3,char *a4,char *a5,char *a6) // ternary
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
void * sub_5f95(void *a0)
{
  void *v1; // rbx
  void *v2;
  
  v2 = NULL;
  while (v1 = a0, v1) {
    a0 = *(void **)((long)v1 + 0x68);
    *(void **)((long)v1 + 0x68) = v2;
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

