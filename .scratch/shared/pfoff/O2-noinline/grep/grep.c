// Function: main @ 0x5090
int main(int argc,char **argv,char **envp)
{
  char *v1;
  int v10; // eax
  int v11; // eax
  int v12; // eax
  FILE *v13;
  int *v14; // rax
  unsigned long *v15; // rax
  char *v16;
  long v17;
  long v18; // stack - 0x178
  long v19; // stack - 0x180
  unsigned int *v2;
  long v20; // stack - 0x188
  char v21 [8];
  char v22; // stack - 0x42
  stat v23; // stack - 0x168
  stat v24; // stack - 0xd8
  unsigned int v25; // eax
  stat *v26;
  unsigned int *v27;
  long v28; // r11
  int v29;
  unsigned long v3;
  unsigned long v30;
  long v31;
  char *v32;
  char *v33; // stack - 0x1c8
  int v34; // stack - 0x1c0
  int v35; // stack - 0x1bc
  unsigned int v36; // stack - 0x1a8
  int v37; // stack - 0x1a4
  char v38; // stack - 0x43
  char v39; // stack - 0x41
  char v4 [16];
  bool v5;
  char v6;
  int v7; // eax
  unsigned int v8; // eax
  int v9; // eax
  
  dat_2a684 = 10;
  v20 = 0;
  v19 = 0;
  dat_2a080 = 2;
  dat_2a804 = 0xffffffff;
  dat_2a7e0 = 0x7fffffffffffffff;
  dat_2a7f8 = -1;
  dat_2a7f0 = -1;
  v18 = -1;
  dat_2a8fd = '\0';
  setlocale(6,"");
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  sub_145d0(0x2a180);
  sub_1e1d0(sub_7a80);
  sub_dd40(NULL);
  dat_2a8c8 = sub_18320(0,0,sub_73d0,sub_7420,0);
  if (!dat_2a8c8)
    sub_1d4b0(); // no-return
  v36 = 0;
  v35 = 0;
  v34 = -1;
  v33 = NULL;
label_51d0:
  do {
    v29 = optind;
    v7 = sub_7aa0(argc,argv,&v18);
    v16 = optarg;
    if (v7 == -1) {
      if (dat_2a905) {
        sub_1cf80(stdout,sub_179e0(),"GNU grep","3.8",0);
        puts(dcgettext(NULL,"Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",5));
        return 0;
      }
      if (dat_2a908)
        sub_a440(0); // no-return
      if (v33) { // branch-flip
        if (!v20) {
          dat_2a686 = 0;
          dat_2a685 = '\0';
          dat_2a802 ^= 1;
          v20 = 1;
          *v33 = '\n';
        }
      }
      else {
        v31 = (long)optind;
        if (argc <= optind) {
          sub_a440(2); // no-return, return-dupe
        }
        optind += 1;
        v16 = argv[v31];
        v30 = 0;
        if ((v34 != 2) && (*v16 == '\\'))
          v30 = (unsigned long)(v16[1] == '-');
        v33 = (char *)sub_1d490(&v16[v30]);
        dat_2a8d8 = v33;
        v30 = strlen(v33);
        v33[v30] = '\n';
        v20 = sub_7bd0(v33,0,v30 + 1,"");
      }
      v31 = v20 + -1;
      v20 = v31;
      sub_184e0(dat_2a8c8);
      if (!dat_2a7a1) {
        v5 = 0;
        if (!fstat(1,&v23)) {
          if ((v23._24_4_ & 0xf000) != 0x8000) { // branch-flip
            if ((v23._24_4_ & 0xf000) == 0x2000) {
              if (((stat("/dev/null",&v24)) || (v23._8_8_ != v24._8_8_)) || (v23._0_8_ != v24._0_8_))
                v5 = 1;
              else {
                dat_2a7a0 = '\x01';
              }
            }
          }
          else {
            v17 = 0x24;
            v26 = &v23;
            v27 = (unsigned int *)0x2a920;
            while (v17) {
              v2 = &v27[1];
              v1 = &v26->field_0x4;
              *v27 = *(unsigned int *)v26;
              v17 -= 1;
              v26 = (stat *)v1;
              v27 = v2;
            }
          }
        }
        if (dat_2a7a0) { // branch-flip
label_5952:
          dat_2a80c = 0;
label_595c:
          dat_2a7e8 = 0;
          dat_2a7a2 = 1;
        }
        else if (dat_2a80c) goto label_595c;
        dat_2a803 = dat_2a7e8 | dat_2a7a2;
        if (dat_2a7f0 < 0)
          dat_2a7f0 = v18;
        if (dat_2a7f8 < 0)
          dat_2a7f8 = v18;
        if (((!dat_2a7e0) || (((!v31 && (dat_2a802)) && ((!dat_2a685 && (!dat_2a686)))))) && (dat_2a80c != 2)) {
          v25 = 1;
          return v25; // return-dupe
        }
        if (dat_2a900 == 2) {
          if ((!v5) || (!sub_1d920()))
            dat_2a900 = 0;
          else {
            dat_2a900 = (unsigned int)(isatty(1) != 0);
          }
        }
        sub_1d960();
        if (dat_2a900) {
          v16 = getenv("GREP_COLOR");
          if (v16) {
            v6 = *v16;
            if (v6) {
              v32 = v16;
              while( true ) {
                if (v6 != ';') {
                  if (!sub_da90((int)v6)) goto label_5feb;
                }
                v6 = v32[1];
                if (!v6) break;
                v32 = &v32[1];
              }
              dat_2a060 = v16;
              dat_2a068 = v16;
            }
          }
label_5feb:
          sub_7970();
          if ((dat_2a068 == v16) || (dat_2a060 == v16))
            error(0,0,dcgettext(NULL,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5),v16,v16);
        }
        sub_7520();
        v30 = 0;
        if (0 <= v34)
          v30 = (long)v34;
        if (v34 <= 2) {
          if (dat_2a687)
            sub_7d30();
          if (v34 != 2) { // branch-flip
            if (2 <= dat_2a8d0)
              v30 = (unsigned long)(int)sub_7fb0(v30 & 0xffffffff,v33,&v20);
          }
          else {
            if (dat_2a180) { // branch-flip
              v25 = 1;
              if ((!sub_7730(v33,v31)) && (v25 = 0, dat_2a687))
                v25 = (sub_7f20(v33,v31) ^ 1) & 0xff;
            }
            else {
              v25 = 0;
              if (dat_2a8d0 == 1)
                v25 = (unsigned int)dat_2a686;
            }
            v30 = 2;
            if (v25 & 1) {
              v30 = 0;
              sub_a6b0(0x2a8d8,&v20);
              v33 = dat_2a8d8;
            }
          }
        }
        v31 = v30 * 0x20;
        dat_2a878 = *(void **)(v31 + 0x29798);
        dat_2a870 = (**(void **)(v31 + 0x29790))(v33,v20,(long)*(int *)(v31 + 0x2978c),dat_2a8fd || dat_2a900);
        v22 = dat_2a684;
        v38 = 0;
        v39 = 0;
        v31 = (*dat_2a878)(dat_2a870,&v22,1,v21,0);
        dat_2a820 = (v31 == 0) == (bool)dat_2a802;
        argc -= optind;
        if ((v36) || (2 <= argc))
          dat_2a808 = ~v36 >> 0x1f;
        else {
          dat_2a808 = -(unsigned int)(dat_2a018 == 3);
        }
        v30 = sysconf(0x1e);
        if (0x3ffffffffffffffa >= v30 - 1) {
          v4._8_8_ = 0;
          v4._0_8_ = v30;
          v31 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v4._0_16_,0);
          dat_2a858 = 0x18000;
          if (v31)
            dat_2a858 = (v30 + 0x18000) - v31;
          dat_2a858 = v30 + 8 + dat_2a858;
          dat_2a838 = v30;
          dat_2a860 = sub_1d110();
          if ((dat_2a014 & 2) && (!dat_2a884))
            dat_2a884 = 1;
          if (1 <= argc) { // branch-flip
            v25 = 1;
            argv = &argv[optind];
            v16 = *argv;
            do {
              v8 = sub_a3f0(v16);
              v16 = argv[1];
              argv = &argv[1];
              v25 &= v8;
            } while (v16);
          }
          else {
            if ((dat_2a018 != 3) || (v35 <= 0))
              v15 = (unsigned long *)0x28f60;
            else {
              dat_2a88a = 1;
              v15 = (unsigned long *)0x28f70;
            }
            v3 = *v15;
            v25 = (unsigned int)(unsigned char)sub_a3f0(v3);
          }
          if (dat_2a889)
            v25 = 2;
          return v25;
        }
        abort(); // no-return, return-dupe
      }
      v5 = 0;
      goto label_5952;
    }
    switch(v7) {
      case 0:
      case 0x55:
        goto label_51d0;
      default:
        sub_a440(2);
      case 0x41:
        sub_7900(optarg,(long *)0x2a7f0);
        goto label_51d0;
      case 0x42:
        sub_7900(optarg,(long *)0x2a7f8);
        goto label_51d0;
      case 0x43:
        sub_7900(optarg,&v18);
        goto label_51d0;
      case 0x44:
        if (strcmp(optarg,"read")) { // branch-flip
          if (strcmp(v16,"skip")) {
            error(2,0,dcgettext(NULL,"unknown devices method",5));
            return v12;
          }
          dat_2a884 = 2;
        }
        else {
          dat_2a884 = 1;
        }
        goto label_51d0;
      case 0x45:
        v34 = sub_77b0("egrep",v34);
        goto label_51d0;
      case 0x46:
        v34 = sub_77b0("fgrep",v34);
        goto label_51d0;
      case 0x47:
        v34 = sub_77b0("grep",v34);
        goto label_51d0;
      case 0x48:
        v36 = 1;
        goto label_51d0;
      case 0x49:
label_52c3:
        dat_2a810 = 2;
        goto label_51d0;
      case 0x4c:
        dat_2a80c = 2;
        goto label_51d0;
      case 0x50:
        v34 = sub_77b0("perl",v34);
        goto label_51d0;
      case 0x52:
        dat_2a014 = 0x30a;
        goto label_5276;
      case 0x54:
        dat_2a8fc = 1;
        goto label_51d0;
      case 0x56:
        dat_2a905 = '\x01';
        goto label_51d0;
      case 0x58:
        v34 = sub_77b0(optarg,v34);
        goto label_51d0;
      case 0x5a:
        dat_2a804 = 0;
        goto label_51d0;
      case 0x61:
label_529d:
        dat_2a810 = 1;
        goto label_51d0;
      case 0x62:
        dat_2a800 = 1;
        goto label_51d0;
      case 99:
        goto label_54f0;
      case 100:
        dat_2a018 = *(int *)(sub_d900("--directories",optarg,0x28f80,0x20788,4,dat_2a078,1) * 4 + 0x20788);
        if (dat_2a018 != 3)
          v29 = v35;
        v35 = v29;
        goto label_51d0;
      case 0x65:
        v30 = strlen(optarg);
        v17 = v20;
        v31 = v30 + 1 + (v20 - v19);
        if (1 <= v31) {
          dat_2a8d8 = (char *)sub_1d2b0(v33,&v19,v31,-1,1);
          v33 = dat_2a8d8;
          v16 = optarg;
        }
        *(char *)mempcpy(&v33[v17],v16,v30) = 10;
        v20 = sub_7bd0(v33,v17,v30 + 1 + v17,"");
        goto label_51d0;
      case 0x66:
        v29 = strcmp(optarg,"-");
        v13 = stdin;
        if ((v29) && (v13 = fopen(v16,"r"), !v13)) {
          error(2,*__errno_location(),"%s",optarg);
          return v10;
        }
        v31 = v20;
        v17 = v20;
        while( true ) {
          v28 = (v17 - v19) + 2;
          if (1 <= v28) {
            v33 = (char *)sub_1d2b0(v33,&v19,v28,-1,1);
            dat_2a8d8 = v33;
          }
          v30 = fread_unlocked(&v33[v17],1,v19 - (v17 + 1),v13);
          if (!v30) break;
          v17 += v30;
        }
        v14 = __errno_location();
        v37 = *v14;
        if (ferror_unlocked(v13)) { // branch-flip
label_5751:
          if (v37) {
            error(2,v37,"%s",optarg);
            return v11;
          }
        }
        else if (stdin != v13) { // branch-flip
          if (fclose(v13)) {
            v37 = *v14;
            goto label_5751;
          }
        }
        else {
          clearerr_unlocked(v13);
        }
        if ((v17 != v31) && (v33[v17 + -1] != '\n')) {
          v33[v17] = '\n';
          v17 += 1;
        }
        v20 = sub_7bd0(v33,v31,v17,optarg);
        goto label_51d0;
      case 0x68:
        v36 = 0xffffffff;
        goto label_51d0;
      case 0x69:
      case 0x79:
        dat_2a687 = '\x01';
        goto label_51d0;
      case 0x6c:
        dat_2a80c = 1;
        goto label_51d0;
      case 0x6d:
        v25 = sub_1d560(optarg,NULL,10,(unsigned long *)0x2a7e0,"");
        if (2 <= v25) {
          error(2,0,dcgettext(NULL,"invalid max count",5));
label_54f0:
          dat_2a7e8 = 1;
        }
        goto label_51d0;
      case 0x6e:
        dat_2a801 = 1;
        goto label_51d0;
      case 0x6f:
        dat_2a8fd = '\x01';
        goto label_51d0;
      case 0x71:
        dat_2a7a1 = '\x01';
        dat_2a080 = 0;
        goto label_51d0;
      case 0x72:
label_5276:
        dat_2a018 = 3;
        v35 = v29;
        goto label_51d0;
      case 0x73:
        dat_2a904 = 1;
        goto label_51d0;
      case 0x75:
        error(0,0,dcgettext(NULL,"warning: --unix-byte-offsets (-u) is obsolete",5));
        goto label_51d0;
      case 0x76:
        dat_2a802 = 1;
        goto label_51d0;
      case 0x77:
        sub_ca30();
        dat_2a686 = 1;
        goto label_51d0;
      case 0x78:
        dat_2a685 = '\x01';
        goto label_51d0;
      case 0x7a:
        dat_2a684 = 0;
        goto label_51d0;
      case 0x80:
        if (!strcmp(optarg,"binary")) {
          dat_2a810 = 0;
          goto label_51d0;
        }
        if (!strcmp(v16,"text")) goto label_529d;
        if (strcmp(v16,"without-match")) {
          error(2,0,dcgettext(NULL,"unknown binary-files type",5));
          abort();
        }
        goto label_52c3;
      case 0x81:
        if (optarg) {
          v29 = sub_ddf0(optarg,"always");
          if (((!v29) || (!sub_ddf0(v16,"yes"))) || (!sub_ddf0(v16,"force"))) {
            dat_2a900 = 1;
            goto label_51d0;
          }
          if (((!sub_ddf0(v16,"never")) || (!sub_ddf0(v16,"no"))) || (!sub_ddf0(v16,"none"))) {
            dat_2a900 = 0;
            goto label_51d0;
          }
          if (((sub_ddf0(v16,"auto")) && (sub_ddf0(v16,"tty"))) && (sub_ddf0(v16,"if-tty"))) {
            dat_2a908 = 1;
            goto label_51d0;
          }
        }
        dat_2a900 = 2;
        goto label_51d0;
      case 0x82:
        sub_147b0(optarg);
        v31 = dat_2a8a0;
        if (!dat_2a8a0) {
          v31 = sub_14d20();
          dat_2a8a0 = v31;
        }
        sub_14fb0(v31,optarg,sub_76d0(0));
        v31 = dat_2a8a8;
        if (!dat_2a8a8) {
          v31 = sub_14d20();
          dat_2a8a8 = v31;
        }
        sub_14fb0(v31,optarg,sub_76d0(1));
        break;
      case 0x83:
      case 0x86:
        v31 = 0;
        while( true ) {
          v17 = *(long *)(v31 * 8 + 0x2a8b0);
          if (!v17) {
            v17 = sub_14d20(v31);
            *(long *)(v31 * 8 + 0x2a8b0) = v17;
          }
          sub_14fb0(v17,optarg,sub_76d0(v31) | (unsigned int)(v7 == 0x86) << 0x1d);
          if (v31 == 1) break;
          v31 = 1;
        }
        break;
      case 0x84:
        v31 = 0;
        while( true ) {
          if (!*(long *)(v31 * 8 + 0x2a8b0))
            *(unsigned long *)(v31 * 8 + 0x2a8b0) = sub_14d20(v31);
          sub_76d0(v31);
          if (sub_15440(sub_14fb0)) {
            error(2,*__errno_location(),"%s",optarg);
            return v9;
          }
          if (v31 == 1) break;
          v31 = 1;
        }
        break;
      case 0x85:
        dat_2a070 = optarg;
        break;
      case 0x87:
        dat_2a7d8 = 1;
        break;
      case 0x88:
        dat_2a7d0 = optarg;
        break;
      case 0x89:
        dat_2a687 = '\0';
      
    }
  } while( true );
}


// Function: sub_62e0 @ 0x62e0
unsigned long sub_62e0(void *a0,long a1,bool a2) // early-return x2
{
  void *v1;
  
  v1 = (void *)((long)a0 + a1 + -1);
  if (a1 + -1 <= -1)
    return 0;
  while( true ) {
    a0 = memchr(a0,0x5c,(long)v1 - (long)a0);
    if (!a0)
      return 0;
    if ((unsigned char)(*(char *)((long)a0 + 1) - 0x31U) <= 8) break;
    if (((int)*(char *)((long)a0 + 1) == (-(unsigned int)(a2 == 0) & 0x24) + 0x5c) && (a0 = (void *)((long)a0 + 1), v1 == a0))
      return 0;
    a0 = (void *)((long)a0 + 1);
  }
  return 1;
}


// Function: sub_6350 @ 0x6350
void sub_6350(struct_10 *a0)
{
  unsigned char *v1;
  char v2;
  unsigned char v3;
  unsigned char v4;
  char *v5; // rax
  void *v6; // rax
  unsigned long v7; // rax
  unsigned char *v8; // rax
  long v9; // r13
  
  v5 = (char *)sub_13730(a0->field_0x8);
  if (v5) {
    v6 = (void *)sub_ca70(0);
    v2 = *v5;
    v1 = (unsigned char *)&v5[3];
    a0->field_0x0 = v6;
    if (v2) { // branch-flip
      a0->field_0x38 = a0->field_0x38 + 1;
      v7 = strlen((char *)v1);
      v9 = (unsigned char)v5[1] + v7 + (unsigned long)(unsigned char)v5[2];
      v8 = (unsigned char *)sub_1d110(v9);
      v4 = dat_2a684;
      *v8 = dat_2a684;
      v3 = v5[1];
      a0->field_0x40 = a0->field_0x40 | v3;
      v6 = memcpy(&v8[v3],v1,v7);
      if (v5[2])
        *(unsigned char *)((long)v6 + v7) = v4;
      sub_bab0(a0->field_0x0,v8,v9);
      free(v8);
    }
    else {
      sub_bab0(v6,v1,strlen((char *)v1));
    }
    sub_bfa0(a0->field_0x0);
    sub_13ef0(v5); // tail-call
    return;
  }
}


// Function: sub_6460 @ 0x6460
unsigned int sub_6460(long a0,char *a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe
{
  unsigned int v1;
  undefined8 v10; // stack - 0x50
  char *v2; // rax
  char *v3; // rax
  unsigned long *v4; // rbx
  char v5 [32]; // stack - 0x88
  long v6; // stack - 0x90
  void *v7; // stack - 0x68
  undefined8 v8; // stack - 0x60
  undefined8 v9; // stack - 0x58
  
  v1 = a6;
  v5._0_8_ = 0;
  v5._8_8_ = 0;
  if ((dat_2a687) || ((char)a6)) {
    v7 = NULL;
    v8 = 0;
    if ((char)a6) {
      re_set_syntax(a5 | 0x2000000);
      v2 = re_compile_pattern(a1,a2,(re_pattern_buffer *)v5);
      if (!v2) {
        regfree((re_pattern_buffer *)v5);
        return v1;
      }
      goto label_6507;
    }
  }
  else {
    v7 = (void *)sub_1d110(0x100);
  }
  v8 = 0;
  re_set_syntax(a5);
  v2 = re_compile_pattern(a1,a2,(re_pattern_buffer *)v5);
  if (!v2) {
    v4 = (unsigned long *)(a3 * 0x40 + *(long *)(a0 + 0x10));
    *v4 = v5._0_8_;
    v4[1] = v5._8_8_;
    v4[2] = v5._16_8_;
    v4[3] = v5._24_8_;
    v4[4] = v7;
    v4[5] = v8;
    v4[6] = v9;
    v4[7] = v10;
    return 1;
  }
label_6507:
  free(v7);
  if ((a4 != -1) && (v3 = (char *)sub_89e0(a4,&v6), *v3)) {
    error(0,0,"%s:%td: %s",v3,v6,v2);
    return 0;
  }
  error(0,0,"%s",v2);
  return 0;
}


// Function: sub_6630 @ 0x6630
void sub_6630(char *a0)
{
  error(2,0,"%s",a0);
}


// Function: sub_6660 @ 0x6660
void sub_6660(char *a0)
{
  error(0,0,dcgettext(NULL,"warning: %s",5),a0); // tail-call
}


// Function: sub_6690 @ 0x6690
long sub_6690(char *a0,unsigned long a1,unsigned long a2,bool a3)
{
  bool v1;
  long v10; // stack - 0x50
  long v11; // stack - 0x48
  unsigned long v12; // r9
  char *v13;
  unsigned long v14;
  long v15; // r13
  void *v16; // stack - 0xa8
  unsigned long v17; // stack - 0xa0
  char *v18; // stack - 0x98
  char *v19; // stack - 0x90
  unsigned char v2; // al
  char *v20; // stack - 0x78
  unsigned long v21; // stack - 0x70
  unsigned int v3;
  long v4; // rax
  unsigned long *v5; // rax
  char *v6; // rax
  long v7;
  long v8;
  unsigned char v9; // bl
  
  v4 = sub_1d390(1,0x48);
  v5 = (unsigned long *)sub_13f00();
  *(unsigned long **)(v4 + 8) = v5;
  if (dat_2a687)
    a2 |= 0x400000;
  sub_142e0(v5,(unsigned char *)0x2a180,a2,(unsigned int)(dat_2a684 == '\0') * 2 | (unsigned int)a2 & 0x10 | 0x2c);
  v2 = dat_2a180 ^ 1 | dat_2a182;
  v6 = &a0[a1];
  v7 = sub_1d0f0(0x40);
  *(unsigned long *)(v4 + 0x18) = 0;
  v10 = 1;
  *(long *)(v4 + 0x10) = v7 + 0x40;
  v11 = 0;
  v16 = rawmemchr(a0,10);
  v15 = 0;
  v14 = (long)v16 - (long)a0;
  v3 = sub_62e0(a0,v14,v2);
  v7 = 0;
  v1 = 0;
  v17 = 0;
  v19 = NULL;
  v8 = 1;
  v13 = a0;
  v18 = a0;
label_6829:
  v7 = (v7 - v8) + 2;
  if (1 <= v7) {
    v8 = *(long *)(v4 + 0x10);
    *(long *)(v4 + 0x10) = sub_1d2b0((void *)(v8 + -0x40),&v10,v7,-1,0x40) + 0x40;
  }
  re_set_syntax(a2);
  v7 = *(long *)(v4 + 0x18);
  if (!sub_6460(v4,v13,v14,v7,v15,a2,(v3 ^ 1) & 0xff))
    v1 = 1;
  v13 = (char *)((long)v16 + 1);
  v15 += 1;
  if ((char)v3) {
    *(long *)(v4 + 0x18) = *(long *)(v4 + 0x18) + 1;
    if (v6 < v13) goto label_6976;
    v16 = rawmemchr(v13,10);
    v14 = (long)v16 - (long)v13;
    v3 = sub_62e0(v13,v14,v2);
    v18 = v13;
label_681f:
    v7 = *(long *)(v4 + 0x18);
    v8 = v10;
    goto label_6829;
  }
  if (v6 >= v13) {
    v16 = rawmemchr(v13,10);
    v14 = (long)v16 - (long)v13;
    v3 = sub_62e0(v13,v14,v2);
    v9 = v18 < v13 & (unsigned char)v3;
    if (v9) { // branch-flip
      v12 = (long)v13 - (long)v18;
      v7 = (v17 - v11) + v12;
      if (1 <= v7)
        v19 = (char *)sub_1d2b0(v19,&v11,v7,-1,1);
      memcpy(&v19[v17],v18,v12);
      v17 += v12;
      v3 = (unsigned int)v9;
    }
    else {
    }
    goto label_681f;
  }
  v13 = v18;
label_6976:
  if (v1)
    exit(2); // no-return
  if (v13 <= v6) {
    if (v13 <= a0) { // branch-flip
      v17 = a1;
      v19 = a0;
    }
    else {
      v14 = v17 + ((long)v6 - (long)v13);
      v19 = (char *)sub_1d180(v19,v14);
      memcpy(&v19[v17],v13,(long)v6 - (long)v13);
      v17 = v14;
    }
  }
  if (!dat_2a686) {
    v13 = NULL;
    v20 = a0;
    v21 = a1;
    if (!dat_2a685) goto label_69c3;
  }
  v13 = (char *)sub_1d110(a1 + 0x2d);
  if (dat_2a685) { // branch-flip
    if (a2 & 0x2000) { // branch-flip
      *(unsigned short *)v13 = dat_1f097;
      v13[2] = dat_1f099;
      v14 = strlen(v13);
      v7 = v14 + a1;
      memcpy(&v13[v14],a0,a1);
      v6 = ")$";
    }
    else {
      *(unsigned int *)v13 = dat_1f09a;
      v14 = strlen(v13);
      v7 = v14 + a1;
      memcpy(&v13[v14],a0,a1);
      v6 = "\\)$";
    }
  }
  else if (a2 & 0x2000) { // branch-flip
    *(unsigned long *)v13 = s_1f060._0_8_;
    *(unsigned long *)&v13[8] = s_1f060._8_8_;
    *(unsigned short *)&v13[0x10] = s_1f060._16_2_;
    v13[0x12] = s_1f060[0x12];
    v14 = strlen(v13);
    v7 = v14 + a1;
    memcpy(&v13[v14],a0,a1);
    v6 = ")([^[:alnum:]_]|$)";
  }
  else {
    *(unsigned long *)v13 = s_1f080._0_8_;
    *(unsigned long *)&v13[8] = s_1f080._8_8_;
    *(unsigned int *)&v13[0x10] = s_1f080._16_4_;
    *(unsigned short *)&v13[0x14] = s_1f080._20_2_;
    v13[0x16] = s_1f080[0x16];
    v14 = strlen(v13);
    v7 = v14 + a1;
    memcpy(&v13[v14],a0,a1);
    v6 = "\\)\\([^[:alnum:]_]\\|$\\)";
  }
  v21 = strlen(strcpy(&v13[v7],v6));
  v20 = v13;
  v21 += v7;
label_69c3:
  sub_13340(v20,v21,*(long *)(v4 + 8));
  sub_6350(v4);
  sub_14250(0,0,*(long *)(v4 + 8),1);
  if (v19) {
    if ((a3) || (v7 = *(long *)(v4 + 8), !sub_13480(v7))) {
      *(long *)(v4 + 0x10) = *(long *)(v4 + 0x10) + -0x40;
      *(long *)(v4 + 0x18) = *(long *)(v4 + 0x18) + 1;
      if (!sub_6460(v4,v19,v17,0,-1,a2,0))
        abort(); // no-return
    }
    if (v20 != v19)
      free(v19);
  }
  free(v13);
  return v4;
}


// Function: sub_6c90 @ 0x6c90
long sub_6c90(struct_14 *a0,char *a1,long a2,long *a3,char *a4) // return-dupe
{
  char *v1;
  char *v10;
  char *v11;
  int v12;
  long v13;
  char *v14; // rbx
  long v15 [3]; // stack - 0x58
  long v16; // stack - 0x60
  char v17; // stack - 0x69
  char *v18; // stack - 0x68
  long v19;
  char *v2;
  int v20; // esi
  re_pattern_buffer *v21; // rdi
  char *v22;
  bool v23; // zf
  char *v24; // stack - 0xf8
  char *v25; // stack - 0xe0
  long v26; // stack - 0xd8
  char *v27; // stack - 0xb0
  long v28; // stack - 0xa0
  char v3;
  char v4; // al
  int v5;
  long v6; // rax
  long v7;
  void *v8; // rax
  long v9;
  
  v3 = dat_2a684;
  v14 = &a1[a2];
  v7 = a0->field_0x8;
  v6 = sub_13460(v7);
  v4 = sub_13470(v7);
  if (v14 <= a1)
    return -1;
  v25 = a1;
  v18 = a1;
  do {
    while (a4) {
      v10 = v25;
      v25 = v14;
      v27 = a4;
label_6f59:
      v28 = (long)v25 - (long)v10;
      if (0x80000000 < v28)
        sub_1d4b0(); // no-return, return-dupe
      if (1 <= a0->field_0x18) {
        v1 = a0->field_0x20;
        v26 = 0;
        v24 = v25;
        v11 = &v25[-1];
        v9 = (long)v25 - (long)v27;
        v7 = 0;
        v20 = (int)v10;
        do {
          while( true ) {
            v19 = v7 * 0x40;
            v21 = (re_pattern_buffer *)(a0->field_0x10 + v19);
            v21->field_0x38 = (dat_2a684 == '\n') << 7 | v21->field_0x38 & 0x3f;
            v5 = re_search(v21,v10,(int)v28 + -1,(int)v27 - v20,(int)v9 + -1,v1);
            if (v5 < -1) {
              sub_1d4b0();
            }
            if (v5 != -1) break;
label_70a8:
            v7 += 1;
            if (a0->field_0x18 <= v7) goto label_7150;
          }
          v22 = &v10[v5];
          if (v24 < v22) goto label_70a8;
          v13 = (long)(*a0->field_0x30 - v5);
          if (a4) {
            if (dat_2a686) {
              if (!dat_2a685) goto label_700d;
              v23 = v9 + -1 != v13;
              v13 = v9;
              v22 = v27;
              if (v23) goto label_70a8;
            }
label_719a:
            if (v24 <= v22) { // branch-flip
              if (v24 == v22) {
                if (v13 <= v26)
                  v13 = v26;
                v26 = v13;
              }
            }
            else {
              v24 = v22;
              v26 = v13;
            }
            goto label_70a8;
          }
          if (!dat_2a685) {
            if (dat_2a686) {
label_700d:
              do {
                if ((!sub_cc90(&v22[v13],v11)) && (!sub_cca0(v10,v22,v11))) {
                  if (a4) goto label_719a;
                  goto label_7210;
                }
                v5 = (int)v22; // branch-flip
                if (1 <= v13) {
                  v21 = (re_pattern_buffer *)(v19 + a0->field_0x10);
                  v2 = &v21->field_0x38;
                  *v2 = *v2 | 0x40;
                  v12 = re_match(v21,v10,(v5 + -1 + (int)v13) - (int)v27,v5 - v20,v1);
                  if (v12 < -1) {
                    sub_1d4b0();
                  }
                  if (v12 <= 0) goto label_7042;
                }
                else {
label_7042:
                  if (v11 == v22) break;
                  v21 = (re_pattern_buffer *)(v19 + a0->field_0x10);
                  v2 = &v21->field_0x38;
                  *v2 = *v2 & 0xbf;
                  v5 = re_search(v21,v10,(int)v28 + -1,(v5 + 1) - v20,((int)v25 - (v5 + 1)) + -1,v1);
                  if (v5 < 0) {
                    if (v5 != -1) {
                      sub_1d4b0();
                    }
                    goto label_70a8;
                  }
                  v12 = *a0->field_0x30 - v5;
                  v22 = &v10[v5];
                }
                v13 = (long)v12;
              } while (v22 <= v24);
              goto label_70a8;
            }
            goto label_7210;
          }
          if (v13 == v9 + -1) goto label_7210;
          v7 += 1;
        } while (v7 < a0->field_0x18);
label_7150:
        if (v24 < v25) {
          v28 = v26;
          v10 = v24;
          goto label_7210;
        }
      }
label_7162:
      if (v14 <= v25)
        return 0xffffffffffffffff;
    }
    v17 = '\0';
    v16 = 0;
    v11 = v14;
    if (a0->field_0x0) {
      v7 = sub_c870(a0->field_0x0,(long)v25 - (unsigned long)a0->field_0x40,(unsigned long)a0->field_0x40 + ((long)v14 - (long)v25),v15,1);
      if (v7 <= -1)
        return v7;
      v22 = &v25[v7];
      v20 = (int)v3;
      v8 = memrchr(a1,v20,(long)v22 - (long)a1);
      v9 = v15[0];
      v10 = (char *)((long)v8 + 1);
      if (!v8)
        v10 = a1;
      v19 = a0->field_0x38;
      if ((v4 == '\x01') && (v19 <= v15[0])) {
        v9 = (long)v22 - (long)v10;
        if (v9 <= 0xf)
          v9 = 0x10;
        if (v7 >> 2 <= v9) { // branch-flip
          if ((long)v14 - (long)v25 >> 2 <= v9) // branch-flip
            v25 = v10;
          else {
            v11 = (char *)((long)rawmemchr(&v25[v9 * 4],v20) + 1);
            v25 = v10;
          }
        }
        else {
          v11 = (char *)((long)rawmemchr(v22,v20) + 1);
          v25 = v10;
        }
        goto label_6e20;
      }
      v11 = (char *)((long)rawmemchr(v22,v20) + 1);
      if (v19 <= v9) {
        v25 = v10;
        goto label_6e20;
      }
      v25 = v11;
      if ((dat_2a180 == '\x01') && (!dat_2a182)) {
        if (v18 < v10)
          v18 = v10;
        if (!sub_caf0(&v18,NULL,v22,v14)) goto label_73ab;
        v22 = v18;
        goto label_6eb5;
      }
label_73ab:
      v28 = (long)v25 - (long)v10;
label_7210:
      *a3 = v28;
      return (long)v10 - (long)a1;
    }
label_6e20:
    if (!v6) {
      v22 = v25;
      v10 = v25;
      v25 = v11;
label_6eb5:
      v11 = (char *)sub_13450(a0->field_0x8,v22,v25,0,&v16,&v17);
      if ((!v11) || (v25 == v11)) goto label_7162;
      if (v16)
        v10 = (char *)((long)memrchr(a1,(int)v3,(long)v11 - (long)a1) + 1);
      v25 = (char *)((long)rawmemchr(v11,(int)v3) + 1);
      if (v17) {
        v27 = v10;
        goto label_6f59;
      }
      goto label_73ab;
    }
    v10 = (char *)sub_13450(v6,v25,v11,0,&v16,0);
    if ((v10) && (v11 != v10)) {
      if (v16)
        v25 = (char *)((long)memrchr(a1,(int)v3,(long)v10 - (long)a1) + 1);
      v8 = rawmemchr(v10,(int)v3);
      v16 = 0;
      v22 = v25;
      v10 = v25;
      v25 = (char *)((long)v8 + 1);
      goto label_6eb5;
    }
    v25 = v11;
    if (v14 <= v11)
      return 0xffffffffffffffff;
  } while( true );
}


// Function: sub_73d0 @ 0x73d0
unsigned long sub_73d0(long a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  unsigned char *v3;
  
  v3 = (unsigned char *)(dat_2a8d8 + -1 + a0);
  v2 = 0x3938a0565d85cb03;
  v1 = *v3;
  while (v1 != 10) {
    v3 = &v3[1];
    v2 = v2 * 0x21 ^ (unsigned long)v1;
    v1 = *v3;
  }
  return v2 % a1;
}


// Function: sub_7420 @ 0x7420
unsigned long sub_7420(long a0,long a1) // return-dupe
{
  char *v1;
  char *v2;
  char *v3;
  long v4; // rax
  long v5;
  char v6;
  
  v1 = (char *)(dat_2a8d8 + -1 + a0);
  v2 = (char *)(dat_2a8d8 + -1 + a1);
  v6 = *v1;
  if (v6 != *v2)
    return 0;
  v5 = 1;
  do {
    if (v6 == '\n')
      return 1;
    v6 = v1[v5];
    v4 = v5 + 1;
    v3 = &v2[v5];
    v5 = v4;
  } while (v6 == *v3);
  return 0;
}


// Function: sub_7470 @ 0x7470
void sub_7470(void)
{
  dat_2a060 = dat_2a068;
}


// Function: sub_7490 @ 0x7490
void sub_7490(void)
{
  dat_2a900 = 0xffffffff;
}


// Function: sub_74a0 @ 0x74a0
void sub_74a0(void)
{
  dat_2a028 = 0x1f09e;
  dat_2a020 = 0x1f0a4;
}


// Function: sub_74d0 @ 0x74d0
unsigned int sub_74d0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  unsigned long v1; // rax
  unsigned int v2; // edx
  
  v2 = a0 & 0xf000;
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),(a0 & 0xb000) == 0x2000) | CONCAT31((undefined3)((unsigned int)a3 >> 8),v2 == 0xc000) | CONCAT31((undefined3)(v2 >> 8),v2 == 0x1000);
}


// Function: sub_7500 @ 0x7500
unsigned char sub_7500(unsigned char a0) // early-return
{
  if (dat_2a884 == 2)
    return 1;
  return dat_2a884 == 0 & (a0 ^ 1);
}


// Function: sub_7520 @ 0x7520
void sub_7520(void)
{
  unsigned int v1;
  int v2; // eax
  long v3; // rdx
  unsigned int v4; // esi
  
  v3 = 1;
  v2 = 1;
  v4 = 0;
  do {
    while ((*(char *)(v3 + 0x2a183) == '\x01' || (v1 = v2, v4 & 0xff & (unsigned int)v3))) {
      v3 += 1;
      if (v3 == 0x100) {
        dat_2a868 = (unsigned long)(v4 & 0xff) * 0x101010101010101;
        return;
      }
    }
    do {
      v2 = v1;
      v1 = v2 * 2;
    } while ((int)(v2 * 2) <= (int)(unsigned int)v3);
    v3 += 1;
    v4 |= v2;
  } while (v3 != 0x100);
  dat_2a868 = (unsigned long)(v4 & 0xff) * 0x101010101010101;
  return;
}


// Function: sub_7590 @ 0x7590
unsigned long * sub_7590(unsigned long *a0)
{
  char *v1;
  unsigned long v2;
  unsigned long v3;
  unsigned char v4; // al
  
  v3 = dat_2a868;
  while( true ) {
    if (!((unsigned long)a0 & 7)) {
      if (v3 & *a0) // branch-flip
        v4 = sub_c9d0((int)(char)*a0);
      else {
        do {
          a0 = &a0[1];
        } while (!(v3 & *a0));
        v4 = sub_c9d0((int)(char)*a0);
      }
      while (!(v4 & v3)) {
        v1 = (char *)((long)a0 + 1);
        a0 = (unsigned long *)((long)a0 + 1);
        v4 = sub_c9d0((int)*v1);
      }
      return a0;
    }
    v2 = *a0;
    if ((unsigned char)sub_c9d0((int)(char)v2) & v3) break;
    a0 = (unsigned long *)((long)a0 + 1);
  }
  return a0;
}


// Function: sub_7630 @ 0x7630
unsigned long sub_7630(unsigned long *a0,long a1) // return-dupe
{
  char *v1;
  long v2; // rax
  char *v3; // rax
  unsigned long v4; // stack - 0x28
  
  if (!dat_2a868)
    return 0;
  v1 = (char *)((long)a0 + a1);
  v4 = 0;
  *v1 = 0xff;
  while( true ) {
    v3 = (char *)sub_7590(a0);
    if (v1 <= v3)
      return 0;
    v2 = sub_c9e0(v3,(long)v1 - (long)v3,&v4);
    if (v2 < 0) break;
    a0 = (unsigned long *)&v3[v2];
  }
  return 1;
}


// Function: sub_76d0 @ 0x76d0
int sub_76d0(bool a0)
{
  return (-(unsigned int)(a0 == 0) & 0x40000000) + 0x10000000;
}


// Function: sub_76f0 @ 0x76f0
unsigned long sub_76f0(char *a0,long a1) // return-dupe
{
  char *v1; // rsi
  
  v1 = &a0[a1];
  if (v1 <= a0)
    return 1;
  do {
    if (*a0)
      return 0;
    a0 = &a0[1];
  } while (a0 != v1);
  return 1;
}


// Function: sub_7720 @ 0x7720
unsigned int sub_7720(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x28) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 0x1f);
}


// Function: sub_7730 @ 0x7730
unsigned long sub_7730(char *a0,long a1) // return-dupe
{
  long v1; // rax
  long v2; // rbx
  unsigned long v3; // stack - 0x38
  
  v3 = 0;
  if (a1 <= 0)
    return 0;
  v2 = 0;
  do {
    v1 = sub_c9f0(&a0[v2],a1 - v2,&v3);
    if (v1 <= -1)
      return 1;
    v2 += v1;
  } while (v2 < a1);
  return 0;
}


// Function: sub_77b0 @ 0x77b0
char * sub_77b0(char *a0,int a1)
{
  char *v1; // rax
  unsigned long v2; // rbx
  
  v2 = 0;
  while( true ) {
    if (!strcmp(a0,(char *)(v2 * 0x20 + 0x29780))) break;
    v2 += 1;
    if (v2 == 7) {
      v1 = dcgettext(NULL,"invalid matcher %s",5);
      error(2,0,v1,a0);
      return v1;
    }
  }
  if ((0 <= a1) && ((int)v2 != a1)) {
    v1 = dcgettext(NULL,"conflicting matchers specified",5);
    error(2,0,v1);
    return v1;
  }
  v1 = (char *)(v2 & 0xffffffff);
  return v1;
}


// Function: sub_7860 @ 0x7860
void sub_7860(long a0,long a1)
{
  if (!SCARRY8(a0,a1))
    return;
  error(2,0,dcgettext(NULL,"input is too large to count",5));
}


// Function: sub_7890 @ 0x7890
unsigned long sub_7890(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  a0[a1] = '\0';
  v1 = strlen(a0);
  return CONCAT71((undefined7)(v1 >> 8),a1 != v1);
}


// Function: sub_78b0 @ 0x78b0
void sub_78b0(char *a0,char *a1,char a2)
{
  unsigned long v1; // rax
  char *v2;
  
  if (a2) {
    while( true ) {
      *a1 = '\0';
      v1 = strlen(a0);
      *a1 = a2;
      v2 = &a0[v1];
      if (a1 == &a0[v1]) break;
      do {
        a0 = &v2[1];
        *v2 = a2;
        v2 = a0;
      } while (!*a0);
    }
    return;
  }
}


// Function: sub_7900 @ 0x7900
void sub_7900(unsigned char *a0,long *a1)
{
  if (((unsigned int)sub_1d560(a0,NULL,10,a1,"") <= 1) && (0 <= *a1))
    return;
  error(2,0,"%s: %s",(char *)a0,dcgettext(NULL,"invalid context length argument",5));
}


// Function: sub_7970 @ 0x7970
void sub_7970(void)
{
  char v1;
  char *v2;
  char *v3;
  long v4; // rbx
  char *v5; // r12
  char *v6;
  
  v2 = getenv("GREP_COLORS");
  if ((v2) && (*v2)) {
    v3 = (char *)sub_1d490(v2);
    v6 = NULL;
    v2 = v3;
label_79dc:
    do {
      while( true ) {
        v5 = &v2[1];
        v1 = *v2;
        if ((v1 != ':') && (v1)) break;
        *v2 = '\0';
        v4 = 0x29600;
        v2 = "mt";
        do {
          if (!strcmp(v2,v3)) break;
          v2 = *(char **)(v4 + 0x18);
          v4 += 0x18;
        } while (v2);
        if ((v6) && (*(unsigned long **)(v4 + 8)))
          **(unsigned long **)(v4 + 8) = v6;
        if (*(void **)(v4 + 0x10))
          (**(void **)(v4 + 0x10))();
        if (!v1)
          return;
        v6 = NULL;
        v3 = v5;
        v2 = v5;
      }
      if (v1 == '=') {
        if (v3 == v2)
          return;
        if (v6)
          return;
        *v2 = '\0';
        v6 = v5;
        v2 = v5;
        goto label_79dc;
      }
      v2 = v5;
    } while (((!v6) || (v1 == ';')) || (sub_da90((int)v1)));
  }
}


// Function: sub_7a80 @ 0x7a80
void sub_7a80(void)
{
  if (dat_2a8c0)
    return;
  sub_de70(); // tail-call
}


// Function: sub_7aa0 @ 0x7aa0
int sub_7aa0(int a0,char **a1,long *a2)
{
  int v1;
  char v2; // al
  int v3; // eax
  char *v4;
  char v5 [21]; // stack - 0x68
  char v6 [3];
  bool v7; // stack - 0x81
  char *v8; // stack - 0x80
  
  v7 = 0;
  v8 = v5;
  do {
    v1 = optind;
    v3 = getopt_long(a0,a1,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(option *)0x28fa0,NULL);
    v2 = sub_da90(v3);
    if (!v2) {
      if (v8 != v5) {
label_7b75:
        *v8 = '\0';
        sub_7900(v5,a2);
      }
      return v3;
    }
    if ((dat_2a010 != v1) || (!v7))
      v4 = v5;
    else {
      v4 = &v8[-(unsigned long)(v5[0] == '0')];
      if (&v8[-(unsigned long)(v5[0] == '0')] == v6) {
        builtin_strncpy(&v8[-(unsigned long)(v5[0] == '0')],"...",4);
        v8 = &v6[3];
        goto label_7b75;
      }
    }
    v8 = &v4[1];
    *v4 = (char)v3;
    v7 = (bool)v2;
    dat_2a010 = v1;
  } while( true );
}


// Function: sub_7bd0 @ 0x7bd0
long sub_7bd0(long a0,long a1,long a2,long a3)
{
  void *v1;
  long v2;
  long *v3;
  int v4; // eax
  long v5;
  void *v6;
  long v7; // rbp
  void *v8;
  int v9;
  
  v6 = (void *)(a0 + a1);
  if (v6 < (void *)(a2 + a0)) {
    v7 = 1;
    v8 = v6;
    v9 = 0;
    do {
      v1 = (void *)((long)rawmemchr(v8,10) + 1);
      memmove(v6,v8,(long)v1 - (long)v8);
      a1 = (long)v6 - a0;
      v4 = sub_18750(dat_2a8c8,a1 + 1,NULL);
      if (v4) {
        if (v4 < 0)
          sub_1d4b0(); // no-return
        v6 = (void *)((long)v6 + ((long)v1 - (long)v8));
        if (!v9) {
          v5 = dat_2a8f0;
          if (dat_2a8e0 == dat_2a8e8) {
            v5 = sub_1d2b0(dat_2a8f0,(long *)0x2a8e8,1,-1,0x18);
            dat_2a8f0 = v5;
          }
          v2 = dat_2a8e0 + 1;
          v3 = (long *)(v5 + dat_2a8e0 * 0x18);
          v3[1] = a3;
          *v3 = dat_2a8d0;
          v3[2] = v7;
          dat_2a8e0 = v2;
        }
        dat_2a8d0 += 1;
        a1 = (long)v6 - a0;
      }
      v7 += 1;
      v8 = v1;
      v9 = v4;
    } while (v1 < (void *)(a2 + a0));
  }
  return a1;
}


// Function: sub_7d30 @ 0x7d30
void sub_7d30(void)
{
  int v1;
  char v2; // al
  int v3; // eax
  char v4 [128];
  mbstate_t v5; // stack - 0xe0
  char v6 [24];
  int *v7; // r13
  char *v8; // r14
  char *v9;
  
  v8 = (char *)0x2a6a0;
  v7 = (int *)0x2a284;
  do {
    if (*v7 != -1) {
      v3 = sub_146b0(*v7,v4);
      if (1 <= v3) {
        v9 = &v4[(long)v3 * 4];
        do {
          v1 = *(int *)&v9[-4];
          v5 = 0;
          if (wcrtomb(v6,v1,&v5) != 1) {
            v2 = 0xff;
            goto label_7dfc;
          }
          v9 = &v9[-4];
        } while (v9 != &v4[(unsigned long)(unsigned int)(v3 - 1) * -4 + (long)v3 * 4 + -4]);
      }
      v2 = 1;
label_7dfc:
      *v8 = v2;
    }
    v7 = &v7[1];
    v8 = &v8[1];
    if (v7 == (int *)0x2a684)
      return;
  } while( true );
}


// Function: sub_7e40 @ 0x7e40
unsigned long sub_7e40(unsigned char *a0,unsigned long a1,mbstate_t *a2)
{
  long *v1; // rax
  unsigned int v2; // stack - 0xbc
  char v3 [136];
  unsigned long v4;
  unsigned long v5; // rax
  
  if (*(int *)((unsigned long)*a0 * 4 + 0x2a284) != -1) // branch-flip
    v5 = (unsigned long)*(char *)((unsigned long)*a0 + 0x2a6a0);
  else {
    v5 = sub_18c40(&v2,a0,a1,a2);
    if ((v5 <= 0x10) && (!sub_146b0(v2,v3))) {
      v4 = v5;
      do {
        v4 -= 1;
        if ((long)v4 <= 0)
          return v5;
        v1 = __ctype_toupper_loc();
      } while ((unsigned int)a0[v4] == *(unsigned int *)(*v1 + (unsigned long)a0[v4] * 4));
    }
    v5 = 0xffffffffffffffff;
  }
  return v5;
}


// Function: sub_7f20 @ 0x7f20
unsigned long sub_7f20(char *a0,long a1) // return-dupe
{
  int v1; // eax
  unsigned long v2; // stack - 0x38
  long v3; // r13
  
  v2 = 0;
  if (a1 <= 0)
    return 1;
  v3 = 0;
  do {
    v1 = sub_7e40(&a0[v3],a1 - v3,&v2);
    if (v1 <= -1)
      return 0;
    v3 += v1;
  } while (v3 < a1);
  return 1;
}


// Function: sub_7fb0 @ 0x7fb0
int sub_7fb0(int a0,unsigned char *a1,unsigned long *a2)
{
  unsigned char v1;
  void *v2; // rax
  unsigned long v3;
  void *v4;
  unsigned long v5; // stack - 0x48
  long v6;
  unsigned char *v7; // r15
  unsigned char *v8;
  
  v6 = *a2;
  v2 = (void *)sub_1d110(v6 + 1);
  v5 = 0;
  v4 = v2;
  v8 = a1;
  while (v6) {
    v1 = *v8;
    if (v1 == 0x5b) {
      free(v2); // return-dupe
      return a0;
    }
    if ('\\' <= (char)v1) { // branch-flip
      if (v1 != 0x5c) { // branch-flip
        if (v1 == 0x5e) {
          free(v2);
          return a0;
        }
        if ((unsigned char)(v1 + 0x85) <= 1) goto label_80e9;
      }
      else if (2 <= v6) {
        v1 = v8[1];
        if ('=' <= (char)v1) { // branch-flip
          if ((unsigned char)(v1 - 0x3e) <= 0x3e) {
            v3 = 1L << (v1 - 0x3e & 0x3f);
            if (v3 & 0x220001402200011) {
              free(v2);
              return a0;
            }
            v3 &= 0x6000000000000002;
label_81a9:
            if ((v3) && (!a0)) {
              free(v2);
              return a0;
            }
          }
        }
        else if ('\n' <= (char)v1) {
          v3 = 1L << (v1 & 0x3f);
          if (!(v3 & 0x13fe008000000400)) {
            v3 &= 0xb0000000000;
            goto label_81a9;
          }
          free(v2);
          return a0;
        }
        v8 = &v8[1];
        v6 -= 1;
      }
    }
    else if ('/' <= (char)v1) { // branch-flip
      if (v1 == 0x3f) {
label_80e9:
        if (a0) {
          free(v2);
          return a0;
        }
      }
    }
    else if ('$' <= (char)v1) {
      v3 = 1L << (v1 & 0x3f);
      if (v3 & 0x441000000000) {
        free(v2);
        return a0;
      }
      if (v3 & 0x90000000000) goto label_80e9;
    }
    if (dat_2a687) // branch-flip
      v3 = sub_7e40(v8,v6,&v5);
    else {
      v3 = sub_c9f0(v8,v6,&v5);
    }
    if ((long)v3 < 0) {
      free(v2);
      return a0;
    }
    v7 = &v8[v3];
    v4 = mempcpy(v4,v8,v3);
    v6 -= v3;
    v8 = v7;
  }
  a0 = 2;
  v6 = (long)v4 - (long)v2;
  if (*a2 != v6) {
    *a2 = v6;
    *(char *)mempcpy(a1,v2,v6) = 10;
  }
  free(v2);
  return a0;
}


// Function: sub_81e0 @ 0x81e0
unsigned long sub_81e0(char *a0,unsigned char a1,unsigned int a2)
{
  long *v1;
  unsigned long v2; // rax
  
  if ((char)a2) { // branch-flip
    if (dat_2a018 == 4)
      return (unsigned long)a2;
    if ((a1) && (dat_2a88a))
      return 0;
    v1 = *(long **)((unsigned long)a1 * 8 + 0x2a8a0);
  }
  else {
    v1 = *(long **)((unsigned long)a1 * 8 + 0x2a8b0);
  }
  if (!v1)
    return 0;
  v2 = sub_14ef0(v1,a0); // tail-call
  return v2;
}


// Function: sub_8250 @ 0x8250
void sub_8250(void *a0)
{
  void *v1;
  long v2; // rbp
  int v3; // r12d
  
  if (a0 <= dat_2a7c0) // branch-flip
    v2 = 0;
  else {
    v3 = (int)dat_2a684;
    v2 = 0;
    v1 = dat_2a7c0;
    while( true ) {
      v1 = memchr(v1,v3,(long)a0 - (long)v1);
      if (!v1) break;
      v1 = (void *)((long)v1 + 1);
      v2 += 1;
      if (a0 <= v1) break;
    }
  }
  dat_2a818 = sub_7860(dat_2a818,v2);
  dat_2a7c0 = a0;
}


// Function: sub_82c0 @ 0x82c0
void sub_82c0(char *a0)
{
  if (!*a0)
    return;
  sub_1d970(dat_2a028,a0); // tail-call
}


// Function: sub_82e0 @ 0x82e0
void sub_82e0(char *a0)
{
  if (!dat_2a900)
    return;
  sub_82c0(a0); // tail-call
}


// Function: sub_8300 @ 0x8300
void sub_8300(char *a0) // return-dupe
{
  if (0 <= fputs_unlocked(a0,stdout))
    return;
  dat_2a8c0 = *__errno_location();
}


// Function: sub_8330 @ 0x8330
void sub_8330(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  char v1; // al
  unsigned long v10; // stack - 0x98
  unsigned long v11; // stack - 0x90
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long v15; // stack - 0x58
  unsigned long v16; // stack - 0x48
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x18
  unsigned int v2; // stack - 0xd8
  char v3 [8];
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xb0
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v1) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v5 = &Stack0000000000000008;
  v2 = 8;
  v6 = v3;
  v4 = 0x30;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  if (0 <= __vfprintf_chk(stdout,1,a8,&v2))
    return;
  dat_2a8c0 = *__errno_location();
}


// Function: sub_8410 @ 0x8410
void sub_8410(int a0) // return-dupe
{
  if (0 <= putchar_unlocked(a0))
    return;
  dat_2a8c0 = *__errno_location();
}


// Function: sub_8430 @ 0x8430
void sub_8430(void) // return-dupe
{
  if (!fflush_unlocked(stdout))
    return;
  dat_2a8c0 = *__errno_location();
}


// Function: sub_8560 @ 0x8560
unsigned char sub_8560(unsigned long a0,long a1)
{
  unsigned long v1;
  long v10; // rdx
  unsigned long *v11;
  long v12;
  long v13;
  long v14;
  long v15;
  unsigned int v16;
  unsigned int v2;
  unsigned char v3; // al
  char v4; // al
  unsigned long v5; // rax
  long v6;
  long v7; // rax
  void *v8;
  unsigned long v9; // rax
  
  v8 = dat_2a860;
  v15 = dat_2a838;
  v12 = dat_2a838 + 8;
  v13 = dat_2a858;
  v11 = dat_2a840;
  if ((long)dat_2a860 + (dat_2a858 - (long)dat_2a840) < v12) {
    v1 = a0 + 1;
    v5 = ~a0;
    v13 = ((dat_2a838 + a0) - dat_2a858) + v12;
    if (1 <= v13) { // branch-flip
      v16 = *(unsigned int *)(a1 + 0x18);
      v14 = -1;
      v6 = a1;
      if ((((sub_8550(v16)) && (v6 = *(long *)(v6 + 0x30) - dat_2a830, 0 <= v6)) && (v7 = v12 + a0 + v6, !SCARRY8(v12 + a0,v6))) && (v14 = v10 + v13, v10 + v13 < v7))
        v14 = v7;
      v8 = (void *)sub_1d2b0(NULL,(long *)0x2a858,v13,v14,1);
      v15 = dat_2a838;
      v11 = (unsigned long *)((long)v8 + v1);
      if ((unsigned long)v11 % dat_2a838) { // branch-flip
        v11 = (unsigned long *)(((dat_2a838 + 1 + a0) - (unsigned long)v11 % dat_2a838) + (long)v8);
        memmove((void *)(v5 + (long)v11),(void *)((long)dat_2a840 + v5),v1);
      }
      else {
        memmove(v8,(void *)(v5 + (long)dat_2a840),v1);
      }
      free(dat_2a860);
      v13 = dat_2a858;
      dat_2a860 = v8;
    }
    else {
      v11 = (unsigned long *)((long)dat_2a860 + v1);
      if ((unsigned long)v11 % dat_2a838) { // branch-flip
        v11 = (unsigned long *)(((dat_2a838 + 1 + a0) - (unsigned long)v11 % dat_2a838) + (long)dat_2a860);
        memmove((void *)(v5 + (long)v11),(void *)((long)dat_2a840 + v5),v1);
        v13 = dat_2a858;
      }
      else {
        memmove(dat_2a860,(void *)((long)dat_2a840 + v5),v1);
        v13 = dat_2a858;
      }
    }
  }
  dat_2a848 = (long)v11 - a0;
  v12 = (long)v8 + ((v13 + -8) - (long)v11);
  do {
    v16 = dat_2a850;
label_86eb:
    v13 = sub_1baf0(v16,v11,v12 - v12 % v15);
    if (v13 == -1) {
      v3 = 0;
label_8704:
      *v11 = 0;
      dat_2a840 = v11;
      return v3;
    }
    dat_2a830 += v13;
    v3 = dat_2a821 ^ 1 | v13 == 0;
    if (v3) {
      v11 = (unsigned long *)((long)v11 + v13);
      goto label_8704;
    }
    v4 = sub_76f0(v11,v13);
    if (!v4) {
      v11 = (unsigned long *)((long)v11 + v13);
      v3 = 1;
      goto label_8704;
    }
    v9 = sub_7860(dat_2a818,v13);
    v16 = dat_2a850;
    v13 = dat_2a830;
    dat_2a818 = v9;
  } while (dat_2a880);
  v6 = lseek(dat_2a850,dat_2a830,3);
  if ((0 <= v6) || (((*__errno_location() == 6 && (v2 = *(unsigned int *)(a1 + 0x18), sub_8550(v2))) && ((v13 < *(long *)(a1 + 0x30) && (v6 = lseek(v16,0,2), 0 <= v6)))))) {
    dat_2a818 = sub_7860(v9,v6 - v13);
    v16 = dat_2a850;
    dat_2a830 = v6;
  }
  else {
    dat_2a880 = '\x01';
  }
  goto label_86eb;
}


// Function: sub_88b0 @ 0x88b0
unsigned long sub_88b0(unsigned long a0,unsigned long a1,char *a2)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v3 = a1;
  if (a1 > a0) {
    v2 = 0;
    if (*(char *)(a1 - 1) != dat_2a684) // branch-flip
      v1 = *(char *)(a1 - 1);
    else {
      v3 = a1 - 1;
      if (a0 >= v3) goto label_88e8;
      v2 = 1;
      v1 = *(char *)(a1 - 2);
    }
    if (v1 == '\r')
      v3 = a1 - (v2 + 1);
  }
label_88e8:
  if (1 <= (long)(v3 - a0)) {
    sub_82c0(a2);
    sub_8460(a0,v3 - a0);
    sub_8890((unsigned char)*a2);
    a0 = v3;
  }
  return a0;
}


// Function: sub_8950 @ 0x8950
void sub_8950(char *a0)
{
  if (!dat_2a900)
    return;
  sub_8890((unsigned char)*a0); // tail-call
}


// Function: sub_8970 @ 0x8970
void sub_8970(unsigned long a0,char *a1)
{
  sub_82e0(a1);
  sub_8330("%*ld",dat_2a8f8,a0);
  sub_8950(a1); // tail-call
}


// Function: sub_89b0 @ 0x89b0
void sub_89b0(char a0)
{
  sub_82e0(dat_2a040);
  sub_8410((int)a0);
  sub_8950(dat_2a040); // tail-call
}


// Function: sub_89e0 @ 0x89e0
long sub_89e0(long a0,long *a1)
{
  long v1; // rax
  long *v2;
  long v3; // rdx
  
  v2 = dat_2a8f0;
  if (2 <= dat_2a8e0) {
    v1 = 0x18;
    v3 = 1;
    while (*(long *)((long)dat_2a8f0 + v1) <= a0) {
      v3 += 1;
      v1 += 0x18;
      if (v3 == dat_2a8e0) {
        v2 = &dat_2a8f0[v3 * 3 + -3];
        *a1 = (a0 - *v2) + v2[2];
        return v2[1];
      }
    }
    v2 = (long *)((long)dat_2a8f0 + v1 + -0x18);
  }
  *a1 = (a0 - *v2) + v2[2];
  return v2[1];
}


// Function: sub_8a50 @ 0x8a50
void sub_8a50(void)
{
  if (dat_2a890)
    return;
  dat_2a890 = dcgettext(NULL,"(standard input)",5);
}


// Function: sub_8a90 @ 0x8a90
void sub_8a90(void)
{
  sub_82e0(dat_2a058);
  sub_8300((char *)sub_8a50());
  sub_8950(dat_2a058); // tail-call
}


// Function: sub_8ac0 @ 0x8ac0
char sub_8ac0(unsigned long *a0,long a1,unsigned long a2,char a3) // early-return
{
  char v1;
  bool v2;
  bool v3;
  char v4;
  unsigned long v5;
  
  if (dat_2a810 != 1) {
    v1 = *(char *)((long)a0 + a1);
    v4 = sub_7630(a0,a1);
    *(char *)((long)a0 + a1) = v1;
    if (v4) {
      dat_2a888 = 1;
      return '\0';
    }
  }
  if (dat_2a808) {
    sub_8a90();
    if (dat_2a804) // branch-flip
      sub_89b0((int)a3);
    else {
      sub_8410(0);
    }
  }
  if (dat_2a801) {
    v5 = dat_2a818;
    if (dat_2a7c0 < a2) {
      sub_8250(a0);
      v5 = sub_7860(dat_2a818,1);
      dat_2a7c0 = a2;
      dat_2a818 = v5;
    }
    sub_8970(v5,dat_2a050);
    sub_89b0((int)a3);
  }
  if (dat_2a800) {
    v5 = sub_7860(dat_2a7c8,(long)a0 - dat_2a848);
    sub_8970(v5,dat_2a048);
    sub_89b0((int)a3);
  }
  if (!dat_2a8fc)
    return '\x01';
  v2 = dat_2a801 || dat_2a800;
  v3 = dat_2a808 != 0;
  v4 = dat_2a8fc;
  if ((v2 || v3) && a1) {
    sub_8410(9);
    v4 = (v2 || v3) && a1;
  }
  return v4;
}


// Function: sub_8c60 @ 0x8c60
unsigned long * sub_8c60(unsigned long *a0,unsigned long *a1,char *a2,char *a3)
{
  unsigned long *v1;
  char v2; // al
  long v3; // rax
  long v4; // stack - 0x48
  unsigned long *v5;
  unsigned long *v6;
  unsigned long *v7;
  
  if (a1 <= a0) { // branch-flip
    if (!dat_2a8fd)
      return a0;
  }
  else {
    v5 = NULL;
    v7 = a0;
    do {
      while( true ) {
        v3 = (*dat_2a878)(dat_2a870,a0,(long)a1 - (long)a0,&v4,v7);
        if ((v3 < 0) || (v1 = (unsigned long *)((long)a0 + v3), a1 == v1)) goto label_8d78;
        if (!v4) break;
        if (dat_2a8fd) { // branch-flip
          v2 = sub_8ac0(v1,v4,a1,(-(dat_2a802 == '\0') & 0xdU) + 0x2d);
          if (!v2)
            return NULL;
        }
        else {
          sub_82c0(a2);
          v6 = v5;
          if (!v5)
            v6 = v7;
          v5 = NULL;
          sub_8460(v6,(long)v1 - (long)v6);
        }
        sub_82e0(a3);
        sub_8460(v1,v4);
        sub_8950(a3);
        if (dat_2a8fd)
          sub_8410((int)dat_2a684);
        v7 = (unsigned long *)((long)v1 + v4);
        if (a1 <= v7) goto label_8d78;
      }
      v4 = 1;
      if (!v5)
        v5 = v7;
      v7 = (unsigned long *)((long)v1 + 1);
    } while (v7 < a1);
label_8d78:
    if (!dat_2a8fd) {
      if (!v5)
        return v7;
      return v5;
    }
  }
  return a1;
}


// Function: sub_8e20 @ 0x8e20
void sub_8e20(unsigned long a0,unsigned long a1,char a2)
{
  int v1;
  unsigned char v2;
  char *v3;
  unsigned char v4;
  char *v5;
  bool v6; // zf
  
  v4 = dat_2a8fd;
  if ((!dat_2a8fd) && (v4 = dat_2a8fd, !sub_8ac0(a0,(a1 - a0) + -1,a1,(int)a2)))
    return;
  v6 = a2 == ':';
  v2 = (v6 ^ dat_2a802) & dat_2a8fd;
  if (dat_2a900) { // branch-flip
    v5 = dat_2a038;
    if (v6 == (bool)((unsigned char)((unsigned int)dat_2a900 >> 0x1f) & dat_2a802))
      v5 = dat_2a030;
    v3 = dat_2a068;
    if (a2 != ':')
      v3 = dat_2a060;
    if (v2) goto label_8f00;
    v2 = dat_2a8fd;
    if (*v5) { // branch-flip
      if (v6 == (bool)dat_2a802) goto label_8f20;
      if (*v3) goto label_8f00;
    }
    else {
      if (!*v3) goto label_8e6c;
      if (v6 != (bool)dat_2a802) goto label_8f00;
label_8f20:
      if (v2) goto label_8e88;
      if (!*v5) {
        v4 = 0;
        goto label_8e6c;
      }
    }
    a0 = sub_88b0(a0,a1,v5);
    v4 = dat_2a8fd;
  }
  else if (v2) {
    v3 = NULL;
    v5 = NULL;
label_8f00:
    a0 = sub_8c60(a0,a1,v5,v3);
    if (!a0)
      return;
    v2 = dat_2a8fd;
    goto label_8f20;
  }
label_8e6c:
  if ((a0 < a1) && (v4 != 1))
    sub_8460(a0,a1 - a0);
label_8e88:
  if (dat_2a7d8)
    sub_8430();
  v1 = dat_2a8c0;
  if (!dat_2a8c0) {
    dat_2a7b8 = a1;
    return;
  }
  error(2,v1,dcgettext(NULL,"write error",5));
}


// Function: sub_8ff0 @ 0x8ff0
void sub_8ff0(void *a0) // return-dupe x2
{
  void *v1;
  int v2; // esi
  
  if (!dat_2a7b8)
    dat_2a7b8 = dat_2a848;
  if (dat_2a7a8 <= 0)
    return;
  do {
    v1 = dat_2a7b8;
    if (a0 <= dat_2a7b8)
      return;
    v2 = (int)dat_2a684;
    sub_8e20(v1,(long)rawmemchr(dat_2a7b8,v2) + 1,0x2d);
    dat_2a7a8 -= 1;
  } while (1 <= dat_2a7a8);
}


// Function: sub_9070 @ 0x9070
void sub_9070(void *a0,void *a1)
{
  char *v1;
  unsigned long v2; // rax
  void *v3;
  void *v4;
  void *v5;
  int v6; // r12d
  long v7;
  long v8; // r14
  char v9;
  
  v9 = dat_2a684;
  v6 = (int)dat_2a684;
  v4 = a0;
  if ((dat_2a803) || ((1 <= (long)dat_2a7a8 && (sub_8ff0(a0), dat_2a803)))) {
    if (!dat_2a802) {
      dat_2a688 = 1; // return-dupe
      dat_2a7a8 = 0;
      dat_2a7b0 -= 1;
      dat_2a828 = (long)a1 + (dat_2a830 - dat_2a840);
      return;
    }
    v9 = dat_2a802;
  }
  else {
    v3 = dat_2a7b8;
    if (!dat_2a7b8)
      v3 = dat_2a848;
    v2 = 0;
    if (1 <= (long)dat_2a7f8) { // branch-flip
      do {
        while (v5 = v4, v4 > v3) {
          do {
            v4 = (void *)((long)v5 - 1);
            v1 = (char *)((long)v5 - 2);
            v5 = v4;
          } while (*v1 != v9);
          v2 += 1;
          if (v2 == dat_2a7f8) goto label_9219;
        }
        v2 += 1;
      } while (v2 != dat_2a7f8);
label_9219:
      if (((dat_2a688) && (dat_2a7b8 != v4)) && (dat_2a070)) {
label_9107:
        sub_82e0(dat_2a040);
        sub_8300(dat_2a070);
        sub_8950(dat_2a040);
        sub_8410(10);
      }
      if (v4 < a0) {
        do {
          v3 = (void *)((long)rawmemchr(v4,(int)v9) + 1);
          sub_8e20(v4,v3,0x2d);
          v4 = v3;
        } while (v3 < a0);
      }
    }
    else if ((((0 <= (long)(dat_2a7f8 & dat_2a7f0)) && (dat_2a7b8 != a0)) && (dat_2a688)) && (dat_2a070)) goto label_9107;
    v9 = dat_2a803;
    if (!dat_2a802) {
      if (dat_2a803) {
        dat_2a688 = 1;
        dat_2a7a8 = 0;
        dat_2a7b0 -= 1;
        dat_2a828 = (long)a1 + (dat_2a830 - dat_2a840);
        return;
      }
      sub_8e20(a0,a1,0x3a);
      dat_2a7b0 -= 1;
      v4 = a1;
      v9 = dat_2a803;
      goto label_91bc;
    }
  }
  v8 = 0;
  v7 = dat_2a7b0;
  if (v4 < a1) {
    for (; v8 < v7; v8 = v8 + 1) {
      v3 = (void *)((long)rawmemchr(v4,v6) + 1);
      if (!v9) {
        sub_8e20(v4,v3,0x3a);
        v7 = dat_2a7b0;
        v9 = dat_2a803;
      }
      v4 = v3;
      if (a1 <= v3) {
        dat_2a7b0 = v7 - (v8 + 1);
        goto label_91bc;
      }
    }
    dat_2a7b0 = v7 - v8;
  }
label_91bc:
  dat_2a7a8 = 0;
  if (!v9) {
    dat_2a7a8 = dat_2a7f0;
    if ((long)dat_2a7f0 <= -1)
      dat_2a7a8 = 0;
  }
  dat_2a688 = 1;
  dat_2a828 = (long)v4 + (dat_2a830 - dat_2a840);
}


// Function: sub_9350 @ 0x9350
long sub_9350(unsigned long a0,unsigned long a1) // early-return
{
  unsigned long v1;
  long v2;
  long v3; // rax
  long v4; // stack - 0x38
  unsigned long v5;
  unsigned long v6;
  int v7;
  
  v2 = dat_2a7b0;
  if (a1 <= a0)
    return 0;
  do {
    v3 = (*dat_2a878)(dat_2a870,a0,a1 - a0,&v4,0);
    if (0 <= v3) { // branch-flip
      v1 = a0 + v3;
      v5 = v4 + v1;
      if (!dat_2a802) {
        if (a1 == v1) {
          return v2 - dat_2a7b0; // return-dupe
        }
        v6 = v5;
        a0 = v1;
        goto label_93a9;
      }
      if (a0 < v1) {
        v6 = v1;
        goto label_93a9;
      }
    }
    else {
      if (!dat_2a802) {
        return v2 - dat_2a7b0;
      }
      v4 = 0;
      if (a1 <= a0) break;
      v6 = a1;
      v5 = a1;
label_93a9:
      sub_9070(a0,v6);
      if ((!dat_2a7b0) || (dat_2a7a2)) {
        if (dat_2a7a1) {
          v7 = 0;
          if (dat_2a889)
            v7 = dat_2a080;
          exit(v7); // no-return
        }
        return v2 - dat_2a7b0;
      }
    }
    a0 = v5;
  } while (v5 < a1);
  return v2 - dat_2a7b0;
}


// Function: sub_94a0 @ 0x94a0
void sub_94a0(int a0)
{
  if (dat_2a904) {
    dat_2a889 = 1;
    return;
  }
  error(0,a0,"%s",(char *)sub_8a50());
  dat_2a889 = 1;
}


// Function: sub_94e0 @ 0x94e0
unsigned long sub_94e0(long a0,unsigned long a1,long a2)
{
  unsigned int v1;
  long v2; // rax
  long v3; // rax
  long v4; // rdx
  
  if (dat_2a881)
    return 0;
  v1 = *(unsigned int *)(a2 + 0x18);
  if ((((sub_8550(v1)) && (a0 < *(long *)(v4 + 0x30))) && (((int)a1 || (a0 = lseek(0,0,1), 0 <= a0)))) && (v2 = lseek(a1 & 0xffffffff,a0,4), 0 <= v2)) {
    v3 = lseek(a1 & 0xffffffff,a0,0);
    if (v3 < 0)
      v3 = sub_94a0(*__errno_location());
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),v2 < *(long *)(a2 + 0x30));
  }
  return 0;
}


// Function: sub_95a0 @ 0x95a0
void sub_95a0(int a0,long a1,bool a2)
{
  unsigned int v1;
  
  if (a0)
    return;
  if (dat_2a7b0) { // branch-flip
    if ((!a2) && (((dat_2a881 || (((long)lseek(0,0,2) <= -1 && (*__errno_location() != 0x16)))) && (v1 = *(unsigned int *)(a1 + 0x18), !sub_8490(v1))))) {
      sub_94a0(*__errno_location()); // return-dupe, tail-call
      return;
    }
  }
  else if (((dat_2a830 != dat_2a828) && (!dat_2a881)) && ((long)lseek(0,dat_2a828,0) <= -1)) {
    sub_94a0(*__errno_location());
    return;
  }
}


// Function: sub_9710 @ 0x9710
long sub_9710(unsigned long a0,long a1,char *a2) // return-dupe
{
  unsigned int v1;
  unsigned long v10; // rdx
  int v11;
  unsigned long v12; // rbp
  char *v13;
  char *v14;
  char *v15;
  unsigned long v16; // r14
  long v17; // stack - 0x70
  long v18; // stack - 0x68
  char v19; // stack - 0x59
  char v2;
  long v20; // stack - 0x58
  char v3;
  char v4;
  char v5;
  unsigned int v6; // eax
  void *v7; // rax
  long v8;
  char *v9;
  
  v4 = dat_2a803;
  v3 = dat_2a7a2;
  v2 = dat_2a684;
  if (!sub_9630(a0))
    return 0;
  dat_2a7c8 = 0;
  dat_2a7b8 = NULL;
  dat_2a7b0 = dat_2a7e0;
  dat_2a821 = v2 == '\0' & dat_2a820;
  dat_2a818 = 0;
  dat_2a828 = 0;
  dat_2a7a8 = 0;
  dat_2a888 = '\0';
  v6 = sub_8560(0,a1);
  v10 = (unsigned long)v6;
  if (!(char)v6) {
    sub_94a0(*__errno_location());
    return 0;
  }
  dat_2a8f8 = 0;
  if (dat_2a8fc) {
    v8 = 0x7fffffffffffffff;
    v1 = *(unsigned int *)(a1 + 0x18);
    if ((sub_8550(v1)) && (v8 = *(long *)(a1 + 0x30), dat_2a801))
      v8 += (unsigned long)(v8 != 0x7fffffffffffffff);
    dat_2a8f8 = 0;
    do {
      dat_2a8f8 += 1;
      v8 /= 10;
    } while (v8);
  }
  v19 = '\0';
  v10 &= 0xffffffff;
  v16 = 0;
  v18 = -1;
  v20 = 0;
  v17 = 0;
  do {
    v13 = dat_2a848;
    v9 = dat_2a840;
    v11 = dat_2a810;
    if (v18 <= -1) {
      if (v2) {
        if (dat_2a810 != 1) {
          v12 = (long)dat_2a840 - (long)dat_2a848;
          v5 = sub_7890(dat_2a848,v12);
          if (v5) { // branch-flip
label_9b0d:
            if (v11 == 2)
              return 0;
            if (!dat_2a7e8) {
              dat_2a803 = '\x01';
              dat_2a7a2 = '\x01';
            }
            dat_2a821 = dat_2a820;
            v18 = v20;
            v13 = dat_2a848;
            v9 = dat_2a840;
            v19 = v2;
          }
          else if ((char)v10) {
            if (sub_94e0(v12,(int)a0,a1)) {
              v11 = dat_2a810;
              goto label_9b0d;
            }
            v13 = dat_2a848;
            v9 = dat_2a840;
          }
        }
      }
    }
    if (dat_2a7b8)
      dat_2a7b8 = v13;
    v14 = &v13[v16];
    dat_2a7c0 = v13;
    if (v14 == v9) {
      *a2 = 1;
      if (v17) {
        dat_2a840 = &v9[1];
        *v9 = v2;
        if (dat_2a7b0)
          v20 += sub_9350(&v13[v16 - v17]);
        if (dat_2a7a8)
          sub_8ff0(dat_2a840);
      }
      goto label_99f0;
    }
    sub_78b0(v14,v9,(int)v19);
    v5 = v14[-1];
    v14[-1] = v2;
    v7 = memrchr(&v14[-1],(int)v2,(unsigned long)&v9[1 - (long)v14]);
    v14[-1] = v5;
    v15 = &v14[-v17];
    v13 = (char *)((long)v7 + 1);
    if (v14 != v13) { // branch-flip
      v17 = (long)v9 - (long)v13;
      if (v15 < v13) {
        if (dat_2a7b0) { // branch-flip
          v20 += sub_9350(v15,v13);
          if (dat_2a7a8) goto label_991a;
          if (!dat_2a7b0) goto label_99f0;
        }
        else {
          if (!dat_2a7a8) goto label_99f0;
label_991a:
          sub_8ff0(v13);
          if (!dat_2a7b0 && !dat_2a7a8) goto label_99f0;
        }
        if (dat_2a7a2) {
          v8 = 0;
          if (0 <= v18)
            v8 = v18;
          if (v8 < v20) goto label_99f0;
        }
      }
    }
    else {
      v17 = (long)v9 - (long)v15;
      v13 = v15;
    }
    if (1 <= dat_2a7f8) {
      v8 = 0;
      do {
        if (v13 <= dat_2a848) break;
        if (v13 == dat_2a7b8) goto label_99af;
        v8 += 1;
        v14 = v13;
        do {
          v13 = &v14[-1];
          v15 = &v14[-2];
          v14 = v13;
        } while (*v15 != v2);
      } while (v8 != dat_2a7f8);
    }
    if (v13 != dat_2a7b8)
      dat_2a7b8 = NULL;
label_99af:
    v16 = (long)v9 - (long)v13;
    if (dat_2a800)
      dat_2a7c8 = sub_7860(dat_2a7c8,&dat_2a840[-v16 - (long)dat_2a848]);
    if (dat_2a801)
      sub_8250(v13);
    v10 = 0;
  } while (sub_8560(v16,a1));
  sub_94a0(*__errno_location());
label_99f0:
  if (dat_2a810) {
    dat_2a7a2 = v3;
    dat_2a803 = v4;
    return v20;
  }
  if (v4) {
    dat_2a7a2 = v3;
    dat_2a803 = v4;
    return v20;
  }
  if (!dat_2a888) {
    if (v18 <= -1) {
      dat_2a7a2 = v3;
      dat_2a803 = v4;
      return v20;
    }
    if (v20 <= v18) {
      dat_2a7a2 = v3;
      dat_2a803 = v4;
      return v20;
    }
  }
  dat_2a7a2 = v3;
  dat_2a803 = v4;
  v9 = (char *)sub_8a50();
  error(0,0,dcgettext(NULL,"%s: binary file matches",5),v9);
  return v20;
}


// Function: sub_9c70 @ 0x9c70
unsigned long sub_9c70(int a0,unsigned char a1)
{
  char v1; // al
  unsigned long v10; // r13
  unsigned long v11; // r13
  unsigned long v12; // stack - 0xd0
  long v2; // rax
  long v3; // rax
  int *v4; // rax
  char *v5; // rax
  stat v6; // stack - 0xc8
  char v7; // stack - 0xd9
  unsigned long v8; // stack - 0xd8
  unsigned int v9; // r12d
  
  v7 = 0;
  if (fstat(a0,&v6)) { // branch-flip
    v11 = 1;
    sub_94a0(*__errno_location());
label_9d70:
    if (!a0) {
      return v11 & 0xffffffff; // return-dupe
    }
  }
  else {
    if (!a0) {
      if ((int)dat_2a808 < 0)
        dat_2a808 = (unsigned int)((v6._24_4_ & 0xf000) == 0x4000);
label_9cca:
      if ((((dat_2a803) || (dat_2a80c)) || (dat_2a7e0 <= 1)) || ((((v6._24_4_ & 0xf000) != 0x8000 || (v6._8_8_ != dat_2a928)) || (v6._0_8_ != dat_2a920)))) {
        v2 = sub_9710(a0,&v6,&v7);
        if (dat_2a7e8) {
          if (dat_2a808) {
            sub_8a90();
            if (dat_2a804) // branch-flip
              sub_89b0(0x3a);
            else {
              sub_8410(0);
            }
          }
          sub_8330("%ld\n",v2);
          if (dat_2a7d8)
            sub_8430();
        }
        v11 = CONCAT71((undefined7)((unsigned long)v10 >> 8),v2 == 0);
        if (dat_2a80c) { // branch-flip
          if (dat_2a80c == (v2 == 0) + 1) {
            sub_8a90();
            sub_8410(dat_2a804 & 10);
            if (dat_2a7d8)
              sub_8430();
          }
        }
        else {
          sub_95a0(a0,&v6,(unsigned char)v7);
        }
      }
      else {
        if (!dat_2a904) {
          v5 = (char *)sub_8a50();
          error(0,0,dcgettext(NULL,"%s: input file is also the output",5),v5);
        }
        dat_2a889 = 1;
        v11 = 1;
      }
      goto label_9d70;
    }
    if (((!sub_7500(a1)) || (v1 = sub_74d0(v6._24_4_), !v1)) && ((!a1 || (v1 = sub_81e0(dat_2a890,1,(v6._24_4_ & 0xf000) == 0x4000), !v1)))) {
      if (0 <= (int)dat_2a808) { // branch-flip
        if (dat_2a018 == 3) goto label_9ed9;
label_9e01:
        if ((dat_2a018 == 4) && ((v6._24_4_ & 0xf000) == 0x4000)) goto label_9e36;
      }
      else {
        dat_2a808 = (unsigned int)((v6._24_4_ & 0xf000) == 0x4000);
        if (dat_2a018 != 3) goto label_9e01;
label_9ed9:
        if ((v6._24_4_ & 0xf000) == 0x4000) {
          v9 = ~(unsigned int)(a1 ^ 1) & dat_2a014;
          if (close(a0))
            sub_94a0(*__errno_location());
          v12 = 0;
          v8 = dat_2a890;
          v2 = sub_16c90(&v8,v9,0);
          if (!v2)
            sub_1d4b0(); // no-return
          v11 = 1;
          while (v3 = sub_171f0(v2), v3) {
            v11 = (unsigned long)((unsigned int)v11 & sub_a180(v2,v3,a1));
          }
          v4 = __errno_location();
          if (*v4)
            sub_94a0(*v4);
          if (sub_17000(v2))
            sub_94a0(*v4);
          return v11 & 0xffffffff;
        }
      }
      if (((dat_2a884 != 2) && ((dat_2a884 || (a1)))) || (v1 = sub_74d0(v6._24_4_), !v1)) goto label_9cca;
    }
label_9e36:
    v11 = 1;
  }
  if (close(a0))
    sub_94a0(*__errno_location());
  return v11 & 0xffffffff;
}


// Function: sub_a0f0 @ 0xa0f0
unsigned long sub_a0f0(unsigned int a0,unsigned long a1,char a2,char a3)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // r8
  
  v3 = (unsigned long)a0;
  sub_7500((unsigned char)a3);
  if (a2) { // branch-flip
    v1 = sub_19570(v3 & 0xffffffff);
    if (v1 < 0) {
      sub_94a0(*__errno_location());
      return 1;
    }
  }
  else {
    v1 = sub_19570(v3 & 0xffffffff);
    if (v1 <= -1) {
      v2 = sub_7720(*__errno_location());
      if ((char)v2)
        return v2;
      sub_94a0();
      return 1;
    }
  }
  return sub_9c70(v1,(unsigned char)a3); // tail-call
}


// Function: sub_a180 @ 0xa180
unsigned long sub_a180(long a0,long a1,unsigned int a2) // return-dupe x3
{
  short v1;
  int v2;
  char *v3;
  unsigned int v4; // eax
  unsigned long v5; // rax
  unsigned int v6; // eax
  stat v7; // stack - 0xd8
  
  v1 = *(short *)(a1 + 0x68);
  a2 = *(long *)(a1 + 0x58) == 0 & a2;
  if (v1 == 6)
    return 1;
  if (!(char)a2) {
    v6 = sub_81e0((char *)(a1 + 0x100),0,(unsigned short)(v1 - 1U) < 2 || v1 == 4);
    if ((char)v6) {
      sub_17830(a0,a1,4);
      return (unsigned long)v6;
    }
    v1 = *(short *)(a1 + 0x68);
  }
  dat_2a890 = *(char **)(a1 + 0x38);
  if ((dat_2a88a) && (dat_2a890[1]))
    dat_2a890 = &dat_2a890[2];
  v3 = dat_2a890;
  v6 = 1;
  if (!(*(unsigned int *)(a0 + 0x48) & 2))
    v6 = *(unsigned int *)(a0 + 0x48) & a2;
  switch(v1) {
    default:
      abort(); // no-return
    case 1:
      if (dat_2a018 == 3)
        return 1;
      sub_17830(a0,a1,4);
      break;
    case 2:
      if (dat_2a904)
        return 1;
      error(0,0,dcgettext(NULL,"%s: warning: recursive directory loop",5),v3);
      return 1;
    case 3:
    case 0xb:
      v4 = sub_7500(a2);
      if ((char)v4) {
        v2 = *(int *)(a1 + 0x88);
        if (!*(int *)(a1 + 0x88)) {
          v3 = *(char **)(a1 + 0x30);
          v2 = *(int *)(a0 + 0x2c);
          if (fstatat(v2,v3,&v7,v6 << 8 ^ 0x100)) {
            sub_94a0(*__errno_location());
            return (unsigned long)v4;
          }
          v2 = v7._24_4_;
        }
        if (sub_74d0(v2))
          return (unsigned long)v4;
      }
      break;
    case 4:
    case 7:
    case 10:
      sub_94a0(*(unsigned int *)(a1 + 0x40));
      return 1;
    case 8:
    case 0xd:
      break;
    case 0xc:
    case 0xe:
      return 1;
    
  }
  v5 = sub_a0f0(*(unsigned int *)(a0 + 0x2c),*(unsigned long *)(a1 + 0x30),v6,a2); // tail-call
  return v5;
}


// Function: sub_a3f0 @ 0xa3f0
void sub_a3f0(char *a0)
{
  if (!strcmp(a0,"-")) {
    dat_2a890 = (char *)dat_2a7d0;
    sub_9c70(0,1); // tail-call
    return;
  }
  dat_2a890 = a0;
  sub_a0f0(0xffffff9c,a0,1,1); // tail-call
}


// Function: sub_a440 @ 0xa440
void sub_a440(int a0)
{
  char *v1; // rax
  
  v1 = (char *)sub_179e0();
  if (a0) { // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    v1 = (char *)sub_179e0();
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v1);
  }
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"Search for PATTERNS in each FILE.\n",5));
    v1 = (char *)sub_179e0();
    __printf_chk(1,dcgettext(NULL,"Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n",5));
    __printf_chk(1,dcgettext(NULL,"  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n",5));
    __printf_chk(1,dcgettext(NULL,"\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n",5));
    __printf_chk(1,dcgettext(NULL,"\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n",5));
    __printf_chk(1,dcgettext(NULL,"  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n",5));
    __printf_chk(1,dcgettext(NULL,"  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n",5));
    __printf_chk(1,dcgettext(NULL,"      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n",5));
    __printf_chk(1,dcgettext(NULL,"  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n",5));
    __printf_chk(1,dcgettext(NULL,"\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n",5));
    __printf_chk(1,dcgettext(NULL,"  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n",5));
    __printf_chk(1,dcgettext(NULL,"When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n",5));
    sub_1d030();
  }
  exit(a0); // no-return
}


// Function: sub_a6b0 @ 0xa6b0
void sub_a6b0(unsigned long *a0,unsigned long *a1)
{
  unsigned char v1;
  unsigned char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x48
  unsigned char *v5;
  unsigned long v6; // r14
  unsigned long v7;
  unsigned char *v8; // stack - 0x60
  
  v7 = *a1;
  v5 = (unsigned char *)*a0;
  v4 = 0;
  v2 = (unsigned char *)sub_1d210(v7 + 1,2);
  v8 = v2;
  if (v7) { // branch-flip
    do {
      while (v3 = sub_c9f0(v5,v7,&v4), v3 == 0xffffffffffffffff) {
        v4 = 0;
label_a767:
        v1 = *v5;
        if (((unsigned char)((unsigned int)v1 - 0x24) <= 0x3a) && (0x580000000000441U >> ((unsigned long)((unsigned int)v1 - 0x24) & 0x3f) & 1)) {
          *v8 = 0x5c;
          v1 = *v5;
          v8 = &v8[1];
        }
        v7 -= 1;
        *v8 = v1;
        v8 = &v8[1];
        v5 = &v5[1];
        if (!v7) goto label_a7ab;
      }
      if (v3 == 1) goto label_a767;
      v6 = 0;
      if (v3 != 0xfffffffffffffffe) {
        v6 = v7 - v3;
        v7 = v3;
      }
      v8 = mempcpy(v8,v5,v7);
      v5 = &v5[v7];
      v7 = v6;
    } while (v6);
label_a7ab:
    v7 = (long)v8 - (long)v2;
  }
  else {
    v7 = 0;
  }
  *v8 = 10;
  free((void *)*a0);
  *a0 = v2;
  *a1 = v7;
}


// Function: sub_a820 @ 0xa820
void sub_a820(char *a0,long a1)
{
  char v1;
  unsigned long v2; // rax
  void *v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // rax
  unsigned long v7; // stack - 0x48
  unsigned long v8; // r13
  char *v9;
  
  v4 = NULL;
  v7 = 0;
  v2 = sub_ca70(1);
  v9 = a0;
  do {
    v3 = rawmemchr(v9,10);
    v8 = (long)v3 - (long)v9;
    if (dat_2a685) {
      if ((v9 <= a0) || (dat_2a684 != '\n')) {
        if ((long)v7 <= (long)(v8 + 1)) {
          free(v4);
          v7 = v8;
          v4 = (char *)sub_1d2b0(NULL,&v7,2,-1,1);
          *v4 = dat_2a684;
        }
        v1 = dat_2a684;
        memcpy(&v4[1],v9,v8);
        v4[v8 + 1] = v1;
        v9 = v4;
      }
      else {
        v9 = &v9[-1];
      }
      v8 += 2;
    }
    sub_bab0(v2,v9,v8);
    v9 = (char *)((long)v3 + 1U);
  } while ((char *)((long)v3 + 1U) <= &a0[a1]);
  free(v4);
  v5 = sub_bf90(v2);
  sub_bfa0(v2);
  v6 = (unsigned long *)sub_1d0f0(0x28);
  *v6 = v2;
  v6[2] = a0;
  v6[1] = v5;
  v6[3] = a1;
  v6[4] = 0;
}


// Function: sub_a9c0 @ 0xa9c0
long sub_a9c0(struct_29 *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
{
  long v1;
  void *v10;
  unsigned char v11; // cl
  unsigned char v12; // cl
  unsigned int v13; // edx
  char v14 [16];
  char v15 [16];
  long v16; // stack - 0x80
  void *v17; // stack - 0x88
  void *v18;
  long v19;
  struct_14 *v2;
  long v20; // stack - 0x68
  long v21; // stack - 0x48
  unsigned long v3;
  char *v4;
  char v5;
  unsigned int v6; // eax
  long v7;
  void *v8;
  long v9; // rax
  
  v5 = dat_2a684;
  v18 = (void *)((long)a1 + a2);
  v1 = a0->field_0x0;
  v6 = (unsigned int)dat_2a685;
  v11 = dat_2a180 & ((dat_2a182 | dat_2a685) ^ 1);
  v12 = v11 | dat_2a686;
  v13 = v6 ^ 1;
  v10 = a1;
  if (a4)
    v10 = a4;
  if (v18 < v10)
    return -1;
  v17 = v10;
  while( true ) {
    v7 = sub_c870(v1,(long)v10 - (unsigned long)v6,(long)v18 + ((unsigned long)v6 - (long)v10),v14,(v12 | a4 != NULL) & v13);
    if (v7 <= -1)
      return -1;
    v7 = (long)v10 + v7;
    v16 = 0;
    v19 = v20 - (int)((unsigned int)dat_2a685 * 2);
    if ((!v11) || (!sub_caf0(&v17,&v16,v7,v18))) break;
    v10 = v17;
    if (v18 < v17)
      return -1;
label_ac3c:
    v6 = (unsigned int)dat_2a685;
  }
  v10 = v17;
  if ((dat_2a686 == 1) || (!a4)) {
    if (!dat_2a685) {
      if (!dat_2a686) {
        v10 = (void *)(v7 + v19);
label_acfd:
        if (v10 < v18)
          v18 = (void *)((long)rawmemchr(v10,(int)v5) + 1);
label_ad17:
        v10 = memrchr(a1,(int)v5,v7 - (long)a1);
        if (v10) { // branch-flip
          v7 = ((long)v10 + 1) - (long)a1;
          a1 = (void *)((long)v10 + 1);
        }
        else {
          v7 = 0;
        }
        v19 = (long)v18 - (long)a1;
        goto label_acc8;
      }
      if (v16) { // branch-flip
        if (v16 <= 0) goto label_ab44;
        v9 = sub_cc90(v7 - v16,v18);
      }
      else {
        v8 = memrchr(v17,(int)v5,v7 - (long)v17);
        if (v8) {
          v10 = (void *)((long)v8 + 1);
          v17 = v10;
        }
label_ab44:
        v9 = sub_cca0(v10,v7,v18);
      }
      if (!v9) {
        while( true ) {
          v10 = (void *)(v7 + v19);
          if (!sub_cc90(v10,v18)) {
            if (!a4) goto label_acfd;
            goto label_acc0;
          }
          if ((!a4) && (!dat_2a180)) break;
          if ((!v19) || (sub_c870(v1,v7,v19 + -1,v15,1))) goto label_ac20;
          v19 = v21;
        }
        if (!a0->field_0x20) {
          sub_a6b0(&a0->field_0x10,&a0->field_0x18);
          v3 = a0->field_0x18;
          v4 = a0->field_0x10;
          a0->field_0x20 = sub_6690(v4,v3,0x10a46,0);
        }
        v8 = v18;
        if (v10 < v18)
          v8 = (void *)((long)rawmemchr(v10,(int)v5) + 1);
        v2 = (struct_14 *)a0->field_0x20;
        if (0 <= (long)sub_6c90(v2,v7,(long)v8 - v7,a3,NULL)) {
          v18 = v8;
          goto label_ad17;
        }
        v7 = (long)v8 - 1;
      }
label_ac20:
      v17 = (void *)(v7 + sub_cc80(v7,v18));
      v10 = (void *)((long)v17 + 1);
      if (v18 < (void *)((long)v17 + 1))
        return -1;
      goto label_ac3c;
    }
    v19 += (unsigned long)(a4 == NULL);
  }
label_acc0:
  v7 -= (long)a1;
label_acc8:
  *a3 = v19;
  return v7;
}


// Function: sub_adc0 @ 0xadc0
unsigned int sub_adc0(char a0)
{
  return sub_c9d0((int)a0); // tail-call
}


// Function: sub_add0 @ 0xadd0
unsigned int sub_add0(long a0,unsigned int a1)
{
  if (a0)
    return (unsigned int)*(unsigned char *)(a0 + (unsigned long)(unsigned char)sub_adc0((int)(char)a1));
  return a1;
}


// Function: sub_ae00 @ 0xae00
void sub_ae00(unsigned long *a0,long *a1)
{
  long v1;
  
  if (a0) {
    sub_ae00(*a0,a1);
    sub_ae00(a0[1],a1);
    v1 = a0[2];
    *(long *)(*a1 + 0x18) = v1;
    *a1 = v1;
    return;
  }
}


// Function: sub_ae50 @ 0xae50
void sub_ae50(struct_5 *a0,long a1,long a2,char a3)
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  long *v4;
  long *v5;
  unsigned long *v6;
  
  if (!a0)
    return;
  sub_ae50(a0->field_0x0,a1,a2,(unsigned char)a3);
  sub_ae50(a0->field_0x8,a1,a2,(unsigned char)a3);
  do {
    if (!a1) {
      a0->field_0x10[4] = a2;
      return;
    }
    v6 = *(unsigned long **)(a1 + 8);
    if (v6) {
      do {
        v2 = (unsigned char *)&v6[3];
        if (a0->field_0x18 == *v2) {
          v4 = a0->field_0x10;
          v5 = (long *)v6[2];
          v4[4] = (long)v5;
          if (a3)
            return;
          if (!*v5)
            return;
          if (*v4)
            return;
          *v4 = -1;
          return;
        }
        v3 = (unsigned long *)*v6;
        v1 = &v6[1];
        v6 = v3;
        if (*v2 <= a0->field_0x18)
          v6 = (unsigned long *)*v1;
      } while (v6);
    }
    a1 = *(long *)(a1 + 0x20);
  } while( true );
}


// Function: sub_af10 @ 0xaf10
void sub_af10(struct_5 *a0,long a1,long a2) // return-dupe
{
  unsigned char *v1; // rbx
  
  if (!a0)
    return;
  sub_af10(a0->field_0x0,a1,a2);
  sub_af10(a0->field_0x8,a1,a2);
  v1 = (unsigned char *)(a2 + (unsigned long)a0->field_0x18);
  if ((long)(unsigned long)*v1 <= a1)
    return;
  *v1 = (unsigned char)a1;
}


// Function: sub_af60 @ 0xaf60
unsigned long sub_af60(unsigned long *a0,struct_5 *a1) // return-dupe
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  unsigned long v4; // rax
  
  if (!a1)
    return 1;
  v4 = sub_af60(a0,a1->field_0x0);
  if (((char)v4) && (v4 = sub_af60(a0,a1->field_0x8), (char)v4)) {
    if (!a0)
      return 0;
    do {
      v2 = (unsigned char *)&a0[3];
      if (a1->field_0x18 == *v2)
        return v4;
      v3 = (unsigned long *)*a0;
      v1 = &a0[1];
      a0 = v3;
      if (*v2 <= a1->field_0x18)
        a0 = (unsigned long *)*v1;
    } while (a0);
    return 0;
  }
  return v4;
}


// Function: sub_afe0 @ 0xafe0
void sub_afe0(struct_6 *a0,long a1)
{
  if (a0) {
    sub_afe0(a0->field_0x0,a1);
    sub_afe0(a0->field_0x8,a1);
    *(unsigned long *)(a1 + (unsigned long)a0->field_0x18 * 8) = a0->field_0x10;
    return;
  }
}


// Function: sub_b030 @ 0xb030
unsigned long sub_b030(unsigned long *a0,unsigned long a1,long a2,long a3,long a4,char a5,char a6,long a7,long a8)
{
  char v1;
  char v2;
  unsigned char v3; // al
  unsigned long v4; // rax
  long v5;
  unsigned long v6;
  long v7;
  long v8; // stack - 0x58
  
  v2 = a6;
  v6 = *a0;
  v8 = 0;
  v5 = a3;
  do {
    v3 = sub_add0(a4,(int)*(char *)(v6 - 2));
    if (v3 != v2) { // branch-flip
      v5 = 0;
      v7 = 2;
    }
    else {
      v7 = 2;
      do {
        v7 += 1;
        if (v5 < v7) {
          v7 = v5 + 1 + v8;
          goto label_b124;
        }
        v3 = sub_add0(a4,(int)*(char *)(v6 + -v7));
        v1 = v3;
        v3 = sub_add0(a4,(int)*(char *)(a2 + -v7));
      } while (v1 == v3);
label_b167:
      v5 = v7 * 8 + -0x10;
    }
    v5 = *(long *)(*(long *)(a8 + 0x978) + v5);
    v6 += v5;
    if (a1 < v6) goto label_b197;
    v1 = *(char *)(v6 - 1);
    v3 = sub_add0(a4,(int)v1);
    if (v3 != a5) {
      if (a7) {
        v3 = sub_adc0((int)v1);
        v4 = (unsigned long)v3;
        v4 = (unsigned long)*(unsigned char *)(a7 + v4);
        v6 += v4;
      }
label_b197:
      v4 = 0;
label_b199:
      *a0 = v6;
      return v4;
    }
    v8 = v7 + -1;
  } while( true );
label_b124:
  if (a3 < v7) {
    v6 -= a3;
    v4 = 1;
    goto label_b199;
  }
  v3 = sub_add0(a4,(int)*(char *)(v6 + -v7));
  v1 = v3;
  v3 = sub_add0(a4,(int)*(char *)(a2 + -v7));
  if (v1 != v3) goto label_b167;
  v7 += 1;
  goto label_b124;
}


// Function: sub_b1c0 @ 0xb1c0
char * sub_b1c0(char *a0,long a1,long a2) // return-dupe x2
{
  char *v1;
  char v2;
  int v3;
  unsigned int v4;
  
  v1 = &a0[a1];
  v3 = *(int *)(a2 + 0x98c);
  if (0 <= v3) { // branch-flip
    while( true ) {
      if (v1 <= a0)
        return NULL;
      v2 = *a0;
      if (*(long *)(a2 + 0x170 + (unsigned long)(unsigned char)sub_adc0((int)v2) * 8)) break;
      a0 = &a0[1];
      if (!((unsigned long)a0 & 0xf)) {
        v4 = *(unsigned int *)(a2 + 0x988);
        return (char *)sub_190f0(a0,v4,v3,(long)v1 - (long)a0); // tail-call
      }
    }
  }
  else {
    if (v1 <= a0)
      return NULL;
    while (v2 = *a0, !*(long *)(a2 + 0x170 + (unsigned long)(unsigned char)sub_adc0((int)v2) * 8)) {
      a0 = &a0[1];
      if (v1 == a0)
        return NULL;
    }
  }
  return a0;
}


// Function: sub_b290 @ 0xb290
long sub_b290(long a0,char *a1,long a2,long *a3,bool a4) // return-dupe x5
{
  unsigned char *v1;
  unsigned long *v10;
  long v11;
  char *v12; // rbp
  long *v13;
  char *v14;
  unsigned long v15;
  long *v16;
  char v2;
  int v3;
  long v4;
  unsigned long *v5;
  unsigned long *v6;
  long *v7;
  unsigned char v8; // al
  char *v9;
  
  if (a2 < *(long *)(a0 + 0x68))
    return -1;
  v13 = *(long **)(a0 + 0x60);
  v4 = *(long *)(a0 + 0x980);
  v11 = *v13;
  v12 = &a1[a2];
  v16 = v13;
  v9 = a1;
  if (!v11) {
    v3 = *(int *)(a0 + 0x988);
    v14 = a1;
label_b2e6:
    v9 = v14;
    if (0 <= v3) { // branch-flip
      v9 = (char *)sub_b1c0(v14,(long)v12 - (long)v14,a0);
      if (!v9)
        return -1;
      v2 = *v9;
      v9 = &v9[1];
      v13 = *(long **)(a0 + 0x170 + (unsigned long)(unsigned char)sub_add0(v4,(int)v2) * 8);
    }
    else {
      while( true ) {
        v2 = *v9;
        v9 = &v9[1];
        v13 = *(long **)(a0 + 0x170 + (unsigned long)(unsigned char)sub_add0(v4,(int)v2) * 8);
        if (v13) break;
        if (v12 <= v9)
          return -1;
      }
    }
    v11 = *v13;
    v16 = v13;
    if (!v11) {
      if (v12 <= v9)
        return -1;
      do {
        v14 = &v9[1];
        v8 = sub_add0(v4,(int)*v9);
        v10 = (unsigned long *)v13[1];
        while (v1 = (unsigned char *)&v10[3], *v1 != v8) {
          v5 = (unsigned long *)*v10;
          v6 = &v10[1];
          v10 = v5;
          if (*v1 <= v8)
            v10 = (unsigned long *)*v6;
          if (!v10) {
            v13 = (long *)v13[4];
            if (!v13) {
              v13 = *(long **)(a0 + 0x170 + (unsigned long)v8 * 8);
              if (v13) goto label_b375;
              if (v12 <= v14)
                return -1;
              goto label_b2e6;
            }
            v11 = *v13;
            v16 = v13;
            if (v11) goto label_b3a7;
            v10 = (unsigned long *)v13[1];
          }
        }
        v13 = (long *)v10[2];
label_b375:
        v11 = *v13;
        v16 = v13;
        v9 = v14;
        if (v11) break;
        if (v12 == v14)
          return -1;
      } while( true );
    }
  }
label_b3a7:
  while (v11 <= -1) {
    v13 = (long *)v13[4];
    v11 = *v13;
  }
  v11 = v13[5];
  v15 = (long)v9 - v11;
  if ((!a4) || (v12 <= v9)) {
label_b434:
    v4 = *v13;
    a3[2] = v11;
    *a3 = v4 >> 1;
    a3[1] = v15 - (long)a1;
    return v15 - (long)a1;
  }
  do {
    v8 = sub_add0(v4,(int)*v9);
label_b3e8:
    v10 = (unsigned long *)v16[1];
label_b3ef:
    if (!v10) goto label_b490;
    v1 = (unsigned char *)&v10[3];
    if (*v1 != v8) {
      v6 = (unsigned long *)*v10;
      v5 = &v10[1];
      v10 = v6;
      if (*v1 <= v8)
        v10 = (unsigned long *)*v5;
      goto label_b3ef;
    }
    v16 = (long *)v10[2];
    v9 = &v9[1];
    v11 = *v16;
    v7 = v16;
    if (v11) {
      while (v11 <= -1) {
        v11 = *(long *)v7[4];
        v7 = (long *)v7[4];
      }
      if ((unsigned long)((long)v9 - v7[5]) <= v15) {
        v13 = v7;
        v15 = (long)v9 - v7[5];
      }
    }
    if (v12 == v9) {
      v11 = v13[5];
      goto label_b434;
    }
  } while( true );
label_b490:
  v16 = (long *)v16[4];
  v11 = v13[5];
  if ((!v16) || (v16[5] < v11)) goto label_b434;
  goto label_b3e8;
}


// Function: sub_b530 @ 0xb530
long sub_b530(long a0,char *a1,long a2,long *a3)
{
  return sub_b290(a0,a1,a2,a3); // tail-call
}


// Function: sub_b540 @ 0xb540
long sub_b540(long a0,long a1,long a2) // return-dupe x2
{
  long v1;
  unsigned long v10;
  unsigned long v11; // stack - 0x48
  unsigned long v12; // rbp
  unsigned long v13;
  char v2;
  char v3;
  char v4;
  unsigned char v5;
  long v6;
  long v7; // rax
  long v8;
  long v9;
  
  v8 = *(long *)(a0 + 0x68);
  if (!v8)
    return 0;
  v6 = *(long *)(a0 + 0x980);
  if (a2 < v8)
    return -1;
  if (v8 == 1) {
    v8 = sub_b1c0(a1,a2,a0);
    if (!v8)
      return -1;
    return v8 - a1;
  }
  v1 = a0 + 0x70;
  v13 = a1 + v8;
  v7 = *(long *)(a0 + 0x970) + v8;
  v2 = *(char *)(a0 + 0x988);
  v3 = *(char *)(a0 + 0x990);
  v11 = v13;
  if ((a2 <= v8 * 0xc) || (SEXT816(v8 * 0xc) != SEXT816(v8) * SEXT816(0xc))) {
label_b5f3:
    v12 = a2 + a1;
    v4 = *(char *)(v13 - 1);
    v10 = (unsigned long)*(unsigned char *)(a0 + 0x70 + (unsigned long)(unsigned char)sub_adc0((int)v4));
    do {
      v11 = v13;
      if ((long)(v12 - v11) < (long)v10)
        return -1;
      while( true ) {
        v13 = v11 + v10;
        v4 = *(char *)(v13 - 1);
        v11 = v13;
        v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v10 = (unsigned long)v5;
        if (v5) break;
        if (sub_b030(&v11,v12,v7,v8,v6,(int)v2,(int)v3,0,a0)) {
          return v11 - a1;
        }
        if ((long)(v12 - v11) <= -1)
          return -1;
      }
    } while( true );
  }
  v12 = a2 + v8 * -0xb + a1;
label_b6df:
  do {
    if (v12 < v13) goto label_b5f3;
    v4 = *(char *)(v13 - 1);
    v9 = v13 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
    v4 = *(char *)(v9 + -1);
    v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
    v10 = v9 + (unsigned long)v5;
    v11 = v10;
    if (v5) {
      v4 = *(char *)(v10 - 1);
      v9 = v10 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
      v4 = *(char *)(v9 + -1);
      v9 += (unsigned long)*(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
      v4 = *(char *)(v9 + -1);
      v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
      v10 = v9 + (unsigned long)v5;
      v11 = v10;
      if (v5) {
        v4 = *(char *)(v10 - 1);
        v9 = v10 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v4 = *(char *)(v9 + -1);
        v9 += (unsigned long)*(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v4 = *(char *)(v9 + -1);
        v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v10 = v9 + (unsigned long)v5;
        v11 = v10;
        if (v5) {
          v4 = *(char *)(v10 - 1);
          v9 = v10 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
          v4 = *(char *)(v9 + -1);
          v11 = v9 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
          v9 = v11 - v13;
          v13 = v11;
          if (0x80 <= v9) goto label_b6df;
          v9 = sub_b1c0(v11 - 1,(a1 + a2) - (v11 - 1),a0);
          if (!v9)
            return -1;
          v13 = v9 + 1;
          v11 = v13;
          if (v12 <= v13) goto label_b5f3;
        }
      }
    }
    v13 = v11;
    if (sub_b030(&v11,v12,v7,v8,v6,(int)v2,(int)v3,v1,a0))
      return v11 - a1;
  } while( true );
}


// Function: sub_b930 @ 0xb930
void sub_b930(long a0,long a1,long a2,unsigned long *a3)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = sub_b540(a0,a1,a2);
  v1 = *(unsigned long *)(a0 + 0x68);
  *a3 = 0;
  a3[1] = v2;
  a3[2] = v1;
}


// Function: sub_b960 @ 0xb960
void * sub_b960(unsigned long a0)
{
  void *v1; // rax
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  
  v1 = (void *)sub_1d0f0(0x9a0);
  _obstack_begin(v1,0,0,sub_1d0f0,dat_29fb8);
  v4 = *(unsigned long *)((long)v1 + 0x20);
  *(unsigned long *)((long)v1 + 0x58) = 0;
  if (0x40 <= v4 - *(long *)((long)v1 + 0x18)) { // branch-flip
    v5 = *(unsigned long **)((long)v1 + 0x10);
    v2 = (unsigned long *)(*(long *)((long)v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_b9ca;
  }
  else {
    _obstack_newchunk(v1,0x40);
    v5 = *(unsigned long **)((long)v1 + 0x10);
    v4 = *(unsigned long *)((long)v1 + 0x20);
    v2 = (unsigned long *)(*(long *)((long)v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_b9ca;
  }
  *(unsigned char *)((long)v1 + 0x50) = *(unsigned char *)((long)v1 + 0x50) | 2;
label_b9ca:
  *(unsigned long **)((long)v1 + 0x60) = v5;
  v3 = (long)v2 + *(unsigned long *)((long)v1 + 0x30) & ~*(unsigned long *)((long)v1 + 0x30);
  if (v3 - *(long *)((long)v1 + 8) <= v4 - *(long *)((long)v1 + 8))
    v4 = v3;
  *(unsigned long *)((long)v1 + 0x18) = v4;
  *(unsigned long *)((long)v1 + 0x10) = v4;
  *v5 = 0;
  v5[1] = 0;
  v5[2] = 0;
  v5[3] = 0;
  v5[4] = 0;
  v5[5] = 0;
  v5[6] = 0;
  *(unsigned long *)((long)v1 + 0x68) = 0x7fffffffffffffff;
  *(unsigned long *)((long)v1 + 0x980) = a0;
  *(void **)((long)v1 + 0x998) = sub_b530;
  *(unsigned long *)((long)v1 + 0x970) = 0;
  return v1;
}


// Function: sub_bab0 @ 0xbab0
void sub_bab0(void *a0,unsigned char *a1,long a2) // return-dupe
{
  unsigned char v1;
  long *v10;
  long *v11;
  long v12;
  int v13 [13]; // stack - 0xdc
  long *v14 [13]; // stack - 0xa8
  unsigned char v15;
  unsigned long v16;
  int v17;
  long v18;
  unsigned long *v19;
  void *v2;
  unsigned long v20;
  unsigned long v21; // stack - 0xf8
  unsigned long v22; // stack - 0xf0
  long v3;
  long *v4;
  long v5;
  long v6;
  long *v7;
  char v8;
  unsigned long *v9; // rax
  
  v2 = *(void **)((long)a0 + 0x998);
  v3 = *(long *)((long)a0 + 0x980);
  v7 = *(long **)((long)a0 + 0x60);
  if (v2 == sub_b930)
    a1 = &a1[a2];
label_bb05:
  do {
    while( true ) {
      if (!a2) {
        v3 = *(long *)((long)a0 + 0x58);
        if (!*v7)
          *v7 = v3 * 2 + 1;
        *(long *)((long)a0 + 0x58) = v3 + 1;
        if (*(long *)((long)a0 + 0x68) <= v7[5])
          return;
        *(long *)((long)a0 + 0x68) = v7[5];
        return;
      }
      a2 -= 1;
      if (v2 != sub_b930) { // branch-flip
        v15 = *a1;
        a1 = &a1[1];
      }
      else {
        v15 = a1[-1];
        a1 = &a1[-1];
      }
      if (v3)
        v15 = *(unsigned char *)(v3 + (unsigned long)v15);
      v10 = (long *)v7[1];
      v13[1] = 0;
      v14[0] = &v7[1];
      v12 = 1;
      if (!v10) break;
      while (v1 = *(unsigned char *)&v10[3], v1 != v15) {
        v14[v12] = v10;
        v18 = v12 + 1;
        if (v15 <= v1) { // branch-flip
          v10 = (long *)*v10;
          v13[v18] = 0;
        }
        else {
          v10 = (long *)v10[1];
          v13[v18] = 1;
        }
        if (!v10) goto label_bb9c;
        v12 = v18;
      }
      v7 = (long *)v10[2];
    }
    v12 = 0;
label_bb9c:
    v16 = *(unsigned long *)((long)a0 + 0x20);
    if (0x20 <= v16 - *(long *)((long)a0 + 0x18)) { // branch-flip
      v19 = *(unsigned long **)((long)a0 + 0x10);
      v9 = (unsigned long *)(*(long *)((long)a0 + 0x18) + 0x20);
      if (v9 == v19) {
label_bea7:
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      }
    }
    else {
      _obstack_newchunk(a0,0x20);
      v19 = *(unsigned long **)((long)a0 + 0x10);
      v16 = *(unsigned long *)((long)a0 + 0x20);
      v9 = (unsigned long *)(*(long *)((long)a0 + 0x18) + 0x20);
      if (v9 == v19) goto label_bea7;
    }
    v20 = *(unsigned long *)((long)a0 + 0x30);
    v22 = ~v20;
    v10 = (long *)((long)v9 + v20 & v22);
    v18 = *(long *)((long)a0 + 8);
    v21 = v16 - v18;
    if ((unsigned long)((long)v10 - v18) <= v21) { // branch-flip
      *(long **)((long)a0 + 0x18) = v10;
      *(long **)((long)a0 + 0x10) = v10;
      *v19 = 0;
      v19[1] = 0;
      if (v16 - (long)v10 <= 0x3f) goto label_bc0f;
      v11 = &v10[8];
      if (v11 == v10) goto label_be4f;
    }
    else {
      *(unsigned long *)((long)a0 + 0x18) = v16;
      *(unsigned long *)((long)a0 + 0x10) = v16;
      *v19 = 0;
      v19[1] = 0;
label_bc0f:
      _obstack_newchunk(a0,0x40);
      v20 = *(unsigned long *)((long)a0 + 0x30);
      v16 = *(unsigned long *)((long)a0 + 0x20);
      v22 = ~v20;
      v11 = (long *)(*(long *)((long)a0 + 0x18) + 0x40);
      v18 = *(long *)((long)a0 + 8);
      v21 = v16 - v18;
      v10 = *(long **)((long)a0 + 0x10);
      if (v11 == v10) {
label_be4f:
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      }
    }
    v22 = (long)v11 + v20 & v22;
    v17 = v13[v12 + 1];
    if (v22 - v18 <= v21)
      v16 = v22;
    *(unsigned long *)((long)a0 + 0x18) = v16;
    v18 = v7[5];
    *(unsigned long *)((long)a0 + 0x10) = v16;
    v19[2] = v10;
    v10[5] = v18 + 1;
    v11 = v14[v12];
    *v10 = 0;
    v10[1] = 0;
    v10[2] = (long)v7;
    v10[3] = 0;
    v10[4] = 0;
    v10[6] = 0;
    *(unsigned char *)&v19[3] = v15;
    *(char *)((long)v19 + 0x19) = 0;
    if (v17) // branch-flip
      v11[1] = (long)v19;
    else {
      *v11 = (long)v19;
    }
    v7 = v10;
  } while (!v12);
  while( true ) {
    v10 = v14[v12];
    v8 = *(char *)((long)v10 + 0x19);
    if (v8) break;
    *(unsigned char *)((long)v10 + 0x19) = -(v17 == 0) | 1;
    if (!(v12 + -1)) goto label_bb05;
    v17 = v13[v12];
    v12 -= 1;
  }
  if (v17) { // branch-flip
    if (v17 != 1) goto label_bb05;
    v8 += '\x01';
    *(char *)((long)v10 + 0x19) = v8;
  }
  else {
    v8 += '\xff';
    *(char *)((long)v10 + 0x19) = v8;
  }
  if (v8) {
    if (v8 != '\xfe') { // branch-flip
      if (v8 != '\x02')
        abort(); // no-return, return-dupe
      if (v13[v12 + 2]) { // branch-flip
        if (v13[v12 + 2] != 1) {
          abort();
        }
        v11 = (long *)v10[1];
        v18 = *v11;
        *v11 = (long)v10;
        *(char *)((long)v10 + 0x19) = 0;
        v10[1] = v18;
        *(char *)((long)v11 + 0x19) = 0;
      }
      else {
        v4 = (long *)v10[1];
        v11 = (long *)*v4;
        v18 = v11[1];
        v5 = *v11;
        *v11 = (long)v10;
        v10[1] = v5;
        v11[1] = (long)v4;
        *v4 = v18;
        *(char *)((long)v10 + 0x19) = -(*(char *)((long)v11 + 0x19) == '\x01');
        *(bool *)((long)v4 + 0x19) = *(char *)((long)v11 + 0x19) == '\xff';
        *(char *)((long)v11 + 0x19) = 0;
      }
    }
    else if (v13[v12 + 2]) { // branch-flip
      if (v13[v12 + 2] != 1) {
        abort();
      }
      v18 = *v10;
      v11 = *(long **)(v18 + 8);
      v5 = v11[1];
      v6 = *v11;
      *v11 = v18;
      *(long *)(v18 + 8) = v6;
      v11[1] = (long)v10;
      *v10 = v5;
      *(char *)(v18 + 0x19) = -(*(char *)((long)v11 + 0x19) == '\x01');
      *(bool *)((long)v10 + 0x19) = *(char *)((long)v11 + 0x19) == '\xff';
      *(char *)((long)v11 + 0x19) = 0;
    }
    else {
      v11 = (long *)*v10;
      v18 = v11[1];
      v11[1] = (long)v10;
      *(char *)((long)v10 + 0x19) = 0;
      *v10 = v18;
      *(char *)((long)v11 + 0x19) = 0;
    }
    if (v13[v12]) // branch-flip
      v14[v12 + -1][1] = (long)v11;
    else {
      *v14[v12 + -1] = (long)v11;
    }
  }
  goto label_bb05;
}


// Function: sub_bf90 @ 0xbf90
unsigned long sub_bf90(long a0)
{
  return *(unsigned long *)(a0 + 0x58);
}


// Function: sub_bfa0 @ 0xbfa0
void sub_bfa0(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned long v10;
  unsigned long v11;
  long v12;
  unsigned long *v13;
  long *v14;
  unsigned long v15;
  unsigned long v16; // rdx
  unsigned long v17; // rdx
  unsigned long v18 [33];
  long *v19; // stack - 0x950
  unsigned long *v2;
  unsigned long v20 [256];
  long v21;
  unsigned long *v22;
  long *v23;
  struct_5 *v24;
  long **v25;
  unsigned long *v26;
  long v27;
  unsigned int v28; // r13d
  unsigned int v29;
  long v3;
  unsigned int v30;
  unsigned int v31; // stack - 0x96c
  unsigned char *v4;
  char v5;
  unsigned char v6; // al
  unsigned long v7;
  long v8;
  unsigned long *v9;
  
  v3 = a0[0x130];
  v26 = v18;
  if (!v3)
    v26 = &a0[0xe];
  v27 = a0[0xb];
  if (v27 == 1) {
    v25 = &v19;
    v14 = (long *)a0[0xc];
    v23 = v14;
    v19 = v14;
    for (; v14; v14 = (long *)v14[3]) {
      sub_ae00((unsigned long *)v14[1],v25);
    }
    v7 = a0[4];
    v10 = a0[0xd];
    if (v10 <= v7 - a0[3]) { // branch-flip
      v21 = a0[2];
      v12 = a0[3] + v10;
      if (v12 == v21) {
label_c855:
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      }
    }
    else {
      _obstack_newchunk((obstack *)a0,v10);
      v21 = a0[2];
      v7 = a0[4];
      v23 = (long *)a0[0xc];
      v12 = a0[3] + v10;
      v10 = a0[0xd];
      if (v12 == v21) goto label_c855;
    }
    a0[0x12e] = v21;
    v11 = v12 + a0[6] & ~a0[6];
    if (v11 - a0[1] <= v7 - a0[1])
      v7 = v11;
    a0[3] = v7;
    a0[2] = v7;
    if (1 <= (long)v10) {
      v12 = 0;
      while( true ) {
        *(char *)(v21 + v12) = *(char *)(v23[1] + 0x18);
        v12 += 1;
        v23 = (long *)v23[3];
        if ((long)a0[0xd] <= v12) break;
        v21 = a0[0x12e];
      }
    }
    v13 = (unsigned long *)sub_b960(a0[0x130]);
    v21 = a0[0xd];
    v4 = (unsigned char *)a0[0x12e];
    v13[0x133] = sub_b930;
    sub_bab0(v13,v4,v21);
    _obstack_free((obstack *)a0,NULL);
    v21 = 0x134;
    v9 = v13;
    v22 = a0;
    while (v21) {
      v2 = &v22[1];
      v1 = &v9[1];
      *v22 = *v9;
      v21 -= 1;
      v9 = v1;
      v22 = v2;
    }
    free(v13);
    v7 = a0[0xd];
    if (0x100 <= (long)v7)
      v7 = 0xff;
    v7 &= 0xff;
    *v26 = SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),0);
    v21 = v7 * 0x101010101010101 + SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),8);
    v26[1] = v21;
    v26[3] = v21;
    v26[2] = v7 * 0x101010101010101;
    v26[5] = v21;
    v26[7] = v21;
    v26[9] = v21;
    v26[4] = v7 * 0x101010101010101;
    v26[0xb] = v21;
    v26[0xd] = v21;
    v26[0xf] = v21;
    v26[6] = v7 * 0x101010101010101;
    v26[0x11] = v21;
    v26[0x13] = v21;
    v26[0x15] = v21;
    v26[8] = v7 * 0x101010101010101;
    v26[0x17] = v21;
    v26[0x19] = v21;
    v26[0x1b] = v21;
    v26[10] = v7 * 0x101010101010101;
    v26[0x1d] = v21;
    v26[0xc] = v7 * 0x101010101010101;
    v26[0xe] = v7 * 0x101010101010101;
    v26[0x10] = v7 * 0x101010101010101;
    v26[0x12] = v7 * 0x101010101010101;
    v26[0x14] = v7 * 0x101010101010101;
    v26[0x16] = v7 * 0x101010101010101;
    v26[0x18] = v7 * 0x101010101010101;
    v26[0x1a] = v7 * 0x101010101010101;
    v26[0x1c] = v7 * 0x101010101010101;
    v15 = SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),8);
    v26[0x1e] = SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),0);
    v26[0x1f] = v21;
    v14 = (long *)a0[0xc];
    if (v14) goto label_c164;
    v21 = 0;
    v19 = v14;
label_c3d0:
    while (v21 = *(long *)(v21 + 0x18), v21) {
      while( true ) {
        v12 = *(long *)(*(long *)(v21 + 0x10) + 0x38);
        v8 = *(long *)(v21 + 0x38);
        if (v12 < *(long *)(v21 + 0x38)) {
          *(long *)(v21 + 0x38) = v12;
          v8 = v12;
        }
        if (*(long *)(v21 + 0x30) <= v8) break;
        *(long *)(v21 + 0x30) = v8;
        v21 = *(long *)(v21 + 0x18);
        if (!v21) goto label_c233;
      }
    }
  }
  else {
    v7 = a0[0xd];
    if (0x100 <= (long)v7)
      v7 = 0xff;
    v7 &= 0xff;
    *v26 = SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),0);
    v21 = v7 * 0x101010101010101 + SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),8);
    v26[1] = v21;
    v26[3] = v21;
    v26[2] = v7 * 0x101010101010101;
    v26[5] = v21;
    v26[7] = v21;
    v26[9] = v21;
    v26[4] = v7 * 0x101010101010101;
    v26[0xb] = v21;
    v26[0xd] = v21;
    v26[0xf] = v21;
    v26[6] = v7 * 0x101010101010101;
    v26[0x11] = v21;
    v26[0x13] = v21;
    v26[0x15] = v21;
    v26[8] = v7 * 0x101010101010101;
    v26[0x17] = v21;
    v26[0x19] = v21;
    v26[0x1b] = v21;
    v26[10] = v7 * 0x101010101010101;
    v26[0x1d] = v21;
    v26[0xc] = v7 * 0x101010101010101;
    v26[0xe] = v7 * 0x101010101010101;
    v26[0x10] = v7 * 0x101010101010101;
    v26[0x12] = v7 * 0x101010101010101;
    v26[0x14] = v7 * 0x101010101010101;
    v26[0x16] = v7 * 0x101010101010101;
    v26[0x18] = v7 * 0x101010101010101;
    v26[0x1a] = v7 * 0x101010101010101;
    v26[0x1c] = v7 * 0x101010101010101;
    v15 = SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),8);
    v26[0x1e] = SUB168(ZEXT816(v7) * ZEXT816(0x101010101010101),0);
    v26[0x1f] = v21;
    v14 = (long *)a0[0xc];
    v19 = v14;
    if (v14) {
label_c164:
      v25 = &v19;
      v19 = v14;
      do {
        while( true ) {
          v24 = (struct_5 *)v14[1];
          sub_ae00(v24,v25,v15,v27 == 1);
          sub_af10(v24,v14[5],v26);
          v24 = (struct_5 *)v14[1];
          sub_ae50(v24,v14[4],a0[0xc]);
          if (v27 == 1) break;
label_c170:
          v14 = (long *)v14[3];
          v15 = v16;
          if (!v14) goto label_c227;
        }
        v21 = a0[0xd];
        v12 = v14[4];
        v14[6] = v21;
        v14[7] = v21;
        if (!v12) goto label_c170;
        v21 = *v14;
        do {
          v5 = sub_af60(*(unsigned long **)(v12 + 8),v24);
          if ((!v5) && (v8 = v14[5] - *(long *)(v12 + 0x28), v8 < *(long *)(v12 + 0x30)))
            *(long *)(v12 + 0x30) = v8;
          if ((v21) && (v8 = v14[5] - *(long *)(v12 + 0x28), v8 < *(long *)(v12 + 0x38)))
            *(long *)(v12 + 0x38) = v8;
          v12 = *(long *)(v12 + 0x20);
        } while (v12);
        v14 = (long *)v14[3];
        v15 = v17;
      } while (v14);
label_c227:
      if (v27 == 1) {
        v21 = a0[0xc];
        goto label_c3d0;
      }
    }
  }
label_c233:
  v9 = &a0[0x2e];
  if (v3)
    v9 = v20;
  v21 = 0;
  *v9 = 0;
  v9[0xff] = 0;
  v7 = (unsigned long)(((int)v9 - (int)(unsigned long *)((unsigned long)&v9[1] & 0xfffffffffffffff8)) + 0x800U >> 3);
  v22 = (unsigned long *)((unsigned long)&v9[1] & 0xfffffffffffffff8);
  while (v7) {
    v13 = &v22[1];
    *v22 = 0;
    v7 -= 1;
    v22 = v13;
  }
  sub_afe0(*(struct_6 **)(a0[0xc] + 8),v9);
  v31 = 0xffffffff;
  v29 = 0xfffffffe;
  do {
    while( true ) {
      v28 = (unsigned int)v21;
      if (v3) { // branch-flip
        v6 = sub_adc0((int)*(char *)(v3 + v21));
        v30 = (unsigned int)v6;
        v12 = v9[v6];
        a0[v21 + 0x2e] = v12;
      }
      else {
        v12 = a0[v21 + 0x2e];
        v30 = v28;
      }
      if (v12) break;
label_c2f0:
      v30 = v29;
      v21 += 1;
      v29 = v30;
      if (v21 == 0x100) goto label_c32d;
    }
    if (v29 != 0xfffffffe) {
      if (v30 != v29) { // branch-flip
        if ((v30 != v28) || (v29 != v31))
          v29 = 0xffffffff;
        else {
          v31 = v28;
        }
      }
      else {
        if (v29 != v31)
          v28 = 0xffffffff;
        v31 = v28;
      }
      goto label_c2f0;
    }
    v21 += 1;
    v29 = v30;
    v31 = v28;
  } while (v21 != 0x100);
label_c32d:
  *(unsigned int *)&a0[0x131] = v30;
  *(unsigned int *)((long)a0 + 0x98c) = v31;
  if (v27 != 1) goto label_c34a;
  v7 = a0[4];
  v10 = a0[0xd];
  if (v10 <= v7 - a0[3]) { // branch-flip
    v27 = a0[2];
    v21 = a0[3] + v10;
    v11 = v10;
    if (v21 == v27) {
label_c821:
      *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
    }
  }
  else {
    _obstack_newchunk((obstack *)a0,v10);
    v27 = a0[2];
    v7 = a0[4];
    v11 = a0[0xd];
    v21 = a0[3] + v10;
    if (v21 == v27) goto label_c821;
  }
  a0[0x12e] = v27;
  v10 = v21 + a0[6] & ~a0[6];
  if (v10 - a0[1] <= v7 - a0[1])
    v7 = v10;
  a0[3] = v7;
  a0[2] = v7;
  v21 = a0[0xc];
  if (1 <= (long)v11) {
    while( true ) {
      v11 -= 1;
      *(char *)(v27 + v11) = *(char *)(*(long *)(v21 + 8) + 0x18);
      v21 = *(long *)(v21 + 0x18);
      if (!v11) break;
      v27 = a0[0x12e];
    }
    v27 = a0[0xd];
    if (2 <= v27) {
      v10 = a0[4];
      v21 = a0[3];
      v7 = v27 * 8 - 8;
      if (v10 - v21 < v7) {
        _obstack_newchunk((obstack *)a0,v7);
        v21 = a0[3];
        v10 = a0[4];
        v27 = a0[0xd];
      }
      v12 = a0[2];
      if (v21 + v7 == v12)
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      a0[0x12f] = v12;
      v7 = v21 + v7 + a0[6] & ~a0[6];
      if (v7 - a0[1] <= v10 - a0[1])
        v10 = v7;
      a0[3] = v10;
      a0[2] = v10;
      v21 = *(long *)(a0[0xc] + 0x18);
      if (2 <= v27) {
        v8 = 0;
        do {
          v9 = (unsigned long *)(v21 + 0x30);
          v21 = *(long *)(v21 + 0x18);
          *(unsigned long *)(v12 + v8 * 8) = *v9;
          v27 = a0[0xd];
          v8 += 1;
        } while (v8 < v27 + -1);
      }
      v5 = *(char *)(a0[0x12e] + -2 + v27);
      *(char *)&a0[0x132] = sub_add0(v3,(int)v5);
    }
  }
label_c34a:
  v27 = 0;
  if (v3) {
    do {
      v5 = *(char *)(v3 + v27);
      *(char *)((long)a0 + v27 + 0x70) = *(char *)((long)v26 + (unsigned long)(unsigned char)sub_adc0((int)v5));
      v27 += 1;
    } while (v27 != 0x100);
    return;
  }
}


// Function: sub_c870 @ 0xc870
void sub_c870(long a0)
{
  (**(void **)(a0 + 0x998))(); // jump-as-call
}


// Function: sub_c8b0 @ 0xc8b0
unsigned long sub_c8b0(unsigned int a0)
{
  int v1; // eax
  
  if (a0 == 0x5f)
    return 1;
  v1 = iswalnum(a0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 != 0);
}


// Function: sub_c8e0 @ 0xc8e0
long sub_c8e0(long a0,long a1,bool a2)
{
  unsigned long v1; // rax
  long v2; // rax
  unsigned long v3; // stack - 0x48
  unsigned int v4; // stack - 0x4c
  long v5; // r12
  
  v5 = 0;
  v3 = 0;
  do {
    while( true ) {
      if (a1 - a0 <= v5)
        return v5;
      v1 = (unsigned long)(unsigned char)*(char *)(a0 + v5);
      if (!*(char *)(v1 + 0x2a9c0)) break;
      v5 += 1;
      if (!a2)
        return v5;
    }
    if (*(char *)(v1 + 0x2a183) != '\xfe')
      return v5;
    v4 = 0;
    v2 = sub_18c40(&v4,(char *)(a0 + v5),(a1 - a0) - v5,&v3);
    if (!sub_c8b0(v4))
      return v5;
    v5 = v5 + v2 + (unsigned long)(v2 == 0);
  } while (a2);
  return v5;
}


// Function: sub_ca30 @ 0xca30
void sub_ca30(void)
{
  unsigned int v1;
  unsigned int *v2; // rbx
  char *v3;
  char *v4; // rbp
  
  v2 = (unsigned int *)0x2a284;
  v3 = (char *)0x2a9c0;
  do {
    v1 = *v2;
    v2 = &v2[1];
    v4 = &v3[1];
    *v3 = sub_c8b0(v1);
    v3 = v4;
  } while (v2 != (unsigned int *)0x2a684);
}


// Function: sub_ca70 @ 0xca70
void sub_ca70(bool a0)
{
  long v1; // rax
  long v2; // rbx
  
  if (dat_2a687) {
    if ((__ctype_get_mb_cur_max() == 1) || (a0)) {
      v2 = 0;
      v1 = sub_1d110(0x100);
      do {
        *(char *)(v1 + v2) = (char)toupper((int)v2);
        v2 += 1;
      } while (v2 != 0x100);
      sub_b960(v1); // tail-call
      return;
    }
  }
  sub_b960(0); // tail-call
}


// Function: sub_caf0 @ 0xcaf0
long sub_caf0(unsigned long *a0,long *a1,char *a2,long a3) // early-return, return-dupe
{
  int v1; // eax
  long v2;
  char *v3;
  unsigned long v4; // stack - 0x48
  char *v5; // r12
  char *v6;
  
  v6 = (char *)*a0;
  if (v6 < a2) {
    if (dat_2a182) { // branch-flip
      if (((int)*a2 & 0xc0U) != 0x80) {
label_cb95:
        *a0 = a2;
        return 0;
      }
      v1 = (unsigned int)a2[-1];
      if ((v1 & 0xc0) != 0x80) // branch-flip
        v2 = 1;
      else {
        v1 = (unsigned int)a2[-2];
        if ((v1 & 0xc0) != 0x80) // branch-flip
          v2 = 2;
        else {
          v1 = (unsigned int)a2[-3];
          if ((v1 & 0xc0) == 0x80) goto label_cb95;
          v2 = 3;
        }
      }
      if ((int)(~v1 & 0xff) >> (7U - (char)v2 & 0x1fU)) goto label_cb95;
      v4 = 0;
      v5 = &a2[-v2];
      v2 = sub_c9e0(v5,a3 - (long)v5,&v4);
      v3 = a2;
      if (0 <= v2) {
        v3 = &v5[v2];
        v6 = v5;
      }
    }
    else {
      v4 = 0;
      while( true ) {
        v2 = sub_c9f0(v6,a3 - (long)v6,&v4);
        if (0 <= v2) // branch-flip
          v3 = &v6[v2];
        else {
          v4 = 0;
          v2 = 1;
          v3 = &v6[1];
        }
        if (a2 <= v3) break;
        v6 = v3;
      }
      if (a1)
        *a1 = v2;
    }
    *a0 = v3;
    if (v3 == a2)
      return 0;
  }
  return (long)a2 - (long)v6;
}


// Function: sub_cc80 @ 0xcc80
long sub_cc80(long a0,long a1)
{
  return sub_c8e0(a0,a1,1); // tail-call
}


// Function: sub_cc90 @ 0xcc90
long sub_cc90(long a0,long a1)
{
  return sub_c8e0(a0,a1,0); // tail-call
}


// Function: sub_cca0 @ 0xcca0
unsigned long sub_cca0(long a0,long a1,long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x28
  
  if (a0 == a1)
    return 0;
  if ((dat_2a180) && (!(~(*(unsigned char *)(a1 + -1) >> 7) & dat_2a182))) {
    v2 = a0;
    v1 = sub_cc90((a1 + -1) - sub_caf0(&v2,NULL,(char *)(a1 + -1),a2),a2); // tail-call
    return v1;
  }
  return (unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)(a1 + -1) + 0x2a9c0);
}


// Function: sub_cd70 @ 0xcd70
void sub_cd70(long a0)
{
  if (0 <= a0) {
    sub_1d110(); // tail-call
    return;
  }
  sub_1d4b0(); // no-return
}


// Function: sub_cd90 @ 0xcd90
int sub_cd90(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int a4)
{
  int v1;
  long v2;
  long v3;
  int v4; // stack - 0x44
  bool v5; // sf
  unsigned long v6; // stack - 0x60
  
  while( true ) {
    while (v1 = pcre2_match_8(a0[1],a1,a2,a3,a4,a0[3],a0[2]), v1 == -0x2e) {
      v2 = a0[5];
      if (0x4000000000000000 <= v2)
        return -0x2e;
      a0[5] = v2 * 2;
      pcre2_jit_stack_free_8(a0[4]);
      v2 = pcre2_jit_stack_create_8(v2,v2 * 2,*a0);
      a0[4] = v2;
      if (!v2)
        sub_1d4b0(); // no-return
      v3 = a0[2];
      if (!v3) {
        v3 = pcre2_match_context_create_8(*a0);
        v2 = a0[4];
        a0[2] = v3;
      }
      v6 = 0xcec2;
      pcre2_jit_stack_assign_8(v3,0,v2);
    }
    if (v1 != -0x35)
      return v1;
    pcre2_config_8(7,&v4,v6);
    v5 = v4 <= -1;
    v4 *= 2;
    if (v5) break;
    v2 = a0[2];
    v1 = v4;
    if (!v2) {
      v2 = pcre2_match_context_create_8(*a0);
      a0[2] = v2;
      v1 = v4;
    }
    v6 = 0xcdf8;
    pcre2_set_depth_limit_8(v2,v1);
  }
  return 0xffffffcb;
}


// Function: sub_cf00 @ 0xcf00
unsigned long * sub_cf00(char *a0,unsigned long a1)
{
  unsigned int v1; // eax
  unsigned long *v10; // rax
  int v11; // stack - 0x154
  char v12 [8];
  char v13 [264];
  unsigned int v14; // r13d
  bool v15; // cf
  unsigned long v16; // stack - 0x168
  char *v17; // stack - 0x160
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  char *v6; // rax
  unsigned long *v7; // rax
  unsigned long *v8; // rax
  unsigned long *v9; // rax
  
  v15 = dat_2a687 == '\0';
  v2 = (unsigned long *)sub_1d110(0x38);
  v14 = (-(unsigned int)v15 & 0xfffffff8) + 0x18;
  v3 = pcre2_general_context_create_8(sub_cd70,sub_cd60,0);
  *v2 = v3;
  v4 = pcre2_compile_context_create_8(v3);
  if (dat_2a180) {
    if (!dat_2a182) {
      error(2,0,dcgettext(NULL,"-P supports only unibyte and UTF-8 locales",5));
      return v8;
    }
    v14 |= 0x4080000;
  }
  if (&a0[a1] != rawmemchr(a0,10)) {
    error(2,0,dcgettext(NULL,"the -P option only supports a single pattern",5));
    return v10;
  }
  v16 = a1;
  if (dat_2a685) { // branch-flip
    pcre2_set_compile_extra_options_8(v4,8);
    v17 = NULL;
  }
  else {
    v17 = NULL;
    if (dat_2a686) {
      v16 = a1 + 0x11;
      v17 = (char *)sub_1d110(v16);
      *(unsigned long *)v17 = s_20990._0_8_;
      *(unsigned short *)&v17[8] = s_20990._8_2_;
      v6 = mempcpy(&v17[10],a0,a1);
      *(unsigned int *)v6 = s_20988._0_4_;
      *(unsigned short *)&v6[4] = s_20988._4_2_;
      v6[6] = s_20988[6];
      a0 = v17;
    }
  }
  pcre2_set_character_tables_8(v4,pcre2_maketables_8(v3));
  v5 = pcre2_compile_8(a0,v16,v14,&v11,v12,v4);
  v2[1] = v5;
  if (!v5) {
    pcre2_get_error_message_8(v11,v13,0x100);
    error(2,0,"%s",v13);
    return v9;
  }
  free(v17);
  pcre2_compile_context_free_8(v4);
  v2[2] = 0;
  v3 = pcre2_match_data_create_from_pattern_8(v2[1],v3);
  v2[3] = v3;
  v1 = pcre2_jit_compile_8(v2[1],1);
  v11 = v1;
  if ((v1 + 0x30U <= 0x30) && (!(0xfffefffffffffff6U >> ((unsigned long)(v1 + 0x30U) & 0x3f) & 1))) {
    v2[4] = 0;
    v2[5] = 0x8000;
    *(unsigned int *)&v2[6] = sub_cd90(v2,"",0,0,1);
    *(unsigned int *)((long)v2 + 0x34) = sub_cd90(v2,"",0,0,0);
    return v2;
  }
  error(2,0,dcgettext(NULL,"JIT internal error: %d",5),v1);
  return v7;
}


// Function: sub_d220 @ 0xd220
long sub_d220(unsigned long *a0,char *a1,long a2,long *a3,char *a4)
{
  char v1;
  char *v10;
  unsigned int v11; // r13d
  unsigned long v12; // r14
  char *v13; // stack - 0x60
  int v2;
  long *v3; // rax
  char *v4;
  long v5; // rax
  char *v6;
  long v7; // rax
  unsigned long v8; // rax
  long v9; // rbp
  
  v6 = a1;
  if (a4)
    v6 = a4;
  v12 = (unsigned long)(v6[-1] == dat_2a684);
  v3 = (long *)pcre2_get_ovector_pointer_8(a0[3]);
  v13 = a1;
  do {
    v4 = rawmemchr(v6,(int)dat_2a684);
    v10 = v13;
    while( true ) {
      while (v1 = *v6, *(char *)((unsigned long)(unsigned char)sub_c9d0((int)v1) + 0x2a183) == '\xff') {
        v6 = &v6[1];
        v12 = 0;
        v10 = v6;
      }
      v9 = (long)v6 - (long)v10;
      if (v6 == v4) break;
      v11 = (unsigned int)v12 ^ 1;
      v2 = sub_cd90(a0,v10,(long)v4 - (long)v10,v9,v11);
      if (!sub_cef0()) goto label_d41b;
      v5 = pcre2_get_startchar_8(a0[3]);
      if (v9 <= v5) { // branch-flip
        if (v5) // branch-flip
          v2 = sub_cd90(a0,v10,v5,v9,v11 | 0x40000002);
        else {
          v2 = *(int *)((long)a0 + v12 * 4 + 0x30);
          *v3 = 0;
          v3[1] = 0;
        }
        if (v2 != -1) goto label_d39e;
        v6 = &v10[v5 + 1];
        v12 = 0;
        v10 = v6;
      }
      else {
        v10 = &v10[v5 + 1];
      }
    }
    v2 = *(int *)((long)a0 + v12 * 4 + 0x30);
    v3[1] = v9;
    *v3 = v9;
label_d41b:
    if (v2 != -1) {
label_d39e:
      if (1 <= v2) {
        v4 = &v4[1];
        if (a4) {
          v13 = &v10[*v3];
          v4 = &v10[v3[1]];
        }
        *a3 = (long)v4 - (long)v13;
        return (long)v13 - (long)a1;
      }
      if ((v2 < -0x2d) && (-0x3f <= v2)) {
        switch(v2) {
          case 0xffffffc1:
            v8 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exceeded PCRE\'s heap limit",5),v8);
            goto label_d4ff;
          case 0xffffffcc:
label_d58c:
            v8 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: PCRE detected recurse loop",5),v8);
            goto label_d5bb;
          case 0xffffffcb:
label_d5bb:
            v8 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exceeded PCRE\'s nested backtracking limit",5),v8);
          case 0xffffffd0:
label_d55d:
            v8 = sub_8a50(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
            error(2,0,dcgettext(NULL,"%s: memory exhausted",5),v8);
            goto label_d58c;
          case 0xffffffd1:
label_d52e:
            v8 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exceeded PCRE\'s backtracking limit",5),v8);
            goto label_d55d;
          case 0xffffffd2:
label_d4ff:
            v8 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exhausted PCRE JIT stack",5),v8);
            goto label_d52e;
          
        }
      }
      v6 = (char *)sub_8a50();
      error(2,0,dcgettext(NULL,"%s: internal PCRE error: %d",5),v6,v2);
      return v7;
    }
    v12 = 1;
    v6 = &v4[1];
    if (&a1[a2] <= v6)
      return -1;
    v13 = v6;
  } while( true );
}

