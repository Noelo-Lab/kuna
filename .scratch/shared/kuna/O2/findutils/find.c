// Function: main @ 0x7240
int main(int argc,char **argv,char **envp)
{
  int v1; // eax
  int v2; // eax
  int v3; // eax
  void *v4; // rax
  char *v5;
  
  v5 = *argv;
  if (!v5)
    v5 = "find";
  sub_21e40(v5);
  sub_da20();
  dat_38618 = 0;
  dat_38604 = 0;
  dat_38608 = 0;
  dat_385f8 = 0xffffff9c;
  if (sub_16220())
    sub_15fd0();
  dat_38610 = sub_ea90("w");
  if (dat_38610) {
    sub_e630((struct_49 *)0x38620);
    setlocale(6,"");
    bindtextdomain("findutils","/usr/local/share/locale");
    textdomain("findutils");
    if (!sub_27a00(sub_17a40)) {
      v1 = sub_e180(argc,argv);
      if (dat_38660 & 2)
        dat_38670 = sub_dec0;
      if (dat_38660 & 0x80)
        __fprintf_chk(stderr,1,"cur_day_start = %s",ctime((void *)0x38648));
      v4 = (void *)sub_c9f0(argc,argv,v1);
      if (sub_8070(argc - v1,&argv[v1])) {
        sub_9e00(v4);
        sub_dab0();
      }
      return dat_38604;
    }
    v5 = dcgettext(NULL,"The atexit library function failed",5);
    error(1,*__errno_location(),v5);
    return v2;
  }
  v5 = dcgettext(NULL,"Failed to initialize shared-file hash table",5);
  error(1,*__errno_location(),v5);
  return v3;
}


// Function: sub_7510 @ 0x7510
char * sub_7510(int a0)
{
  switch(a0) {
    default:
      __sprintf_chk((char *)0x38598,1,0xe,"[%d]",a0);
      return (char *)0x38598;
    case 1:
      return "FTS_D";
    case 2:
      return "FTS_DC";
    case 3:
      return "FTS_DEFAULT";
    case 4:
      return "FTS_DNR";
    case 5:
      return "FTS_DOT";
    case 6:
      return "FTS_DP";
    case 7:
      return "FTS_ERR";
    case 8:
      return "FTS_F";
    case 9:
      return "FTS_INIT";
    case 10:
      return "FTS_NS";
    case 0xb:
      return "FTS_NSOK";
    case 0xc:
      return "FTS_SL";
    case 0xd:
      return "FTS_SLNONE";
    case 0xe:
      return "FTS_W";
    
  }
}


// Function: sub_7670 @ 0x7670
void sub_7670(struct_4 *a0,struct_3 *a1) // return-dupe
{
  char v1;
  char *v10;
  unsigned int v11; // edx
  long v12;
  unsigned int v13;
  long v14; // stack - 0x168
  unsigned int v15; // stack - 0x16c
  char v16 [152];
  long v17; // stack - 0x160
  long v18; // stack - 0x158
  long v19; // stack - 0x150
  unsigned char v2;
  long v20; // stack - 0x148
  long v21; // stack - 0x140
  long v22; // stack - 0x138
  long v23; // stack - 0x130
  long v24; // stack - 0x128
  long v25; // stack - 0x120
  long v26; // stack - 0x118
  long v27; // stack - 0x110
  long v28; // stack - 0x108
  long v29; // stack - 0x100
  unsigned char v3;
  long v30; // stack - 0xf8
  long v31; // stack - 0xf0
  long v32; // stack - 0xe8
  long v33; // stack - 0xe0
  short v4;
  int v5;
  struct_2 *v6; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9;
  
  if (dat_38660 & 4) {
    v9 = (char *)sub_239f0(2,dat_3868c,(char *)a1->field_0x30);
    v10 = (char *)sub_239f0(1,dat_3868c,a1->field_0x38);
    v5 = dat_38014;
    v12 = a1->field_0x58;
    v7 = (char *)sub_7510(a1->field_0x68);
    v8 = (char *)sub_239f0(0,dat_3868c,a1->field_0x38);
    __fprintf_chk(stderr,1,"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",v8,v7,(int)v12,v5,v10,v9);
  }
  v4 = a1->field_0x68;
  if (v4 != 6) { // branch-flip
    v12 = a1->field_0x58;
    if ((v12 <= dat_38014) && (v13 = dat_38018, v12)) {
label_76e0:
      v5 = a0->field_0x2c;
      if (v13 & 0x200) goto label_76ea;
label_770b:
      v12 = a1->field_0x58;
      v4 = a1->field_0x68;
    }
    else if (dat_38018 & 0x200) goto label_7908;
label_7713:
    v17 = a1->field_0x78;
    dat_38014 = (unsigned int)v12;
    if (v4 == 7) {
      sub_e8c0(a1->field_0x40,a1->field_0x38); // return-dupe
      return;
    }
    if (v4 != 4) { // branch-flip
      if (v4 == 2) {
        if ((a1->field_0x88 & 0xf000) != 0xa000) {
          v12 = *(long *)(a1->field_0x0 + 0x48);
          v9 = *(char **)(a1->field_0x0 + 0x38);
          if (v12) { // branch-flip
            v10 = &v9[v12];
            v1 = *v10;
            *v10 = '\0';
            v9 = (char *)sub_239f0(1,dat_3868c,v9);
            *v10 = v1;
          }
          else {
            v9 = (char *)sub_239f0(1,dat_3868c,"");
          }
          v10 = (char *)sub_e8a0(0,a1->field_0x38);
          error(0,0,dcgettext(NULL,"File system loop detected; %s is part of the same file system loop as %s.",5),v10,v9);
          dat_38604 = 1;
          return;
        }
        v9 = (char *)sub_e8a0(0,a1->field_0x38);
        error(0,0,dcgettext(NULL,"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.",5),v9);
        dat_38604 = 1;
        return;
      }
      if (v4 != 0xd) { // branch-flip
        if (v4 == 10) {
          if (!v12) {
            sub_e8c0(a1->field_0x40,a1->field_0x38);
            return;
          }
          v5 = (*dat_38670)(a1->field_0x30,v16);
          if ((v5) && (*__errno_location() == 0x28)) {
            sub_e8c0(0x28,a1->field_0x38); // return-dupe
            return;
          }
          sub_e8c0(a1->field_0x40,a1->field_0x38);
          v4 = a1->field_0x68;
        }
      }
      else {
        v5 = (*dat_38670)(a1->field_0x30,v16);
        if ((v5) && (*__errno_location() == 0x28)) {
          sub_e8c0(0x28,a1->field_0x38);
          return;
        }
        v4 = a1->field_0x68;
      }
    }
    else {
      sub_e8c0(a1->field_0x40,a1->field_0x38);
      if (dat_38620)
        return;
      v4 = a1->field_0x68;
    }
    if (2 <= (unsigned short)(v4 - 10U)) goto label_7761;
    if (dat_385e4)
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting"); // no-return
    if ((v4 != 0xb) && (dat_385e8))
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,"consider_visiting"); // no-return
    v15 = dat_385e8;
    v13 = dat_385e8;
label_7a35:
    dat_385e0 = (unsigned int)a1->field_0x58;
    if (v13) goto label_7808;
    v4 = a1->field_0x68;
label_7a4b:
    if (((unsigned short)(v4 - 1U) < 2) || (v4 == 6)) goto label_7847;
    v5 = 0;
    v13 = 0;
    if (0 <= dat_38624) goto label_7862;
label_7882:
    v11 = (unsigned int)CONCAT71(dat_38661,dat_38660);
    if (a1->field_0x58 < (long)dat_38628) goto label_7934;
    if (dat_38660 & 4) goto label_7bf5;
  }
  else {
    if (dat_38018 & 0x200) {
label_7908:
      if (0 <= dat_38010) {
        close(dat_38010);
        dat_38010 = -1;
        v13 = dat_38018;
        goto label_76e0;
      }
      v5 = a0->field_0x2c;
label_76ea:
      if ((v5 != -100) && (v5 < 0))
        sub_7640(); // no-return
      dat_385f8 = v5;
      if (dat_38010 < 0) {
        if (v5 == -100) {
          dat_38010 = -100;
          v12 = a1->field_0x58;
          v4 = a1->field_0x68;
          goto label_7713;
        }
        dat_38010 = sub_17a00();
      }
      goto label_770b;
    }
    dat_38014 = (unsigned int)a1->field_0x58;
label_7761:
    v14 = a1->field_0x70;
    v17 = a1->field_0x78;
    v18 = a1->field_0x80;
    v19 = a1->field_0x88;
    dat_385e4 = 0x101;
    v20 = a1->field_0x90;
    v21 = a1->field_0x98;
    v22 = a1->field_0xa0;
    v23 = a1->field_0xa8;
    v24 = a1->field_0xb0;
    v25 = a1->field_0xb8;
    v26 = a1->field_0xc0;
    v27 = a1->field_0xc8;
    v28 = a1->field_0xd0;
    v29 = a1->field_0xd8;
    v30 = a1->field_0xe0;
    v31 = a1->field_0xe8;
    v32 = a1->field_0xf0;
    v33 = a1->field_0xf8;
    v15 = (unsigned int)v19;
    dat_385e8 = (unsigned int)v19;
    if (!(unsigned int)v19) {
      v9 = (char *)sub_239f0(0,dat_3868c,a1->field_0x38);
      error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v9);
      v13 = v15;
      goto label_7a35;
    }
    dat_385e0 = (unsigned int)a1->field_0x58;
label_7808:
    v9 = a1->field_0x38;
    if (!sub_dfe0(&v15,v9,&a1[1],&v14,0))
      return;
    v4 = a1->field_0x68;
    if ((v15 & 0xf000) != 0x4000) goto label_7a4b;
label_7847:
    if (v4 == 0xb) {
      sub_1a750(a0,a1,1);
      return;
    }
    v5 = 1;
    if (0 <= dat_38624) { // branch-flip
label_7862:
      if (a1->field_0x58 < (long)dat_38624) goto label_786c;
      sub_1a750(a0,a1,4);
      v13 = (unsigned int)((long)dat_38624 < a1->field_0x58);
      v4 = a1->field_0x68;
    }
    else {
label_786c:
      v13 = 0;
    }
    if (v4 == 1) {
      if (!dat_38620) goto label_792d;
      goto label_7882;
    }
    if ((v4 != 6) || (!dat_38620)) goto label_7882;
label_792d:
    v11 = (unsigned int)CONCAT71(dat_38661,dat_38660);
label_7934:
    if (!(v11 & 4)) goto label_793d;
    v13 = 1;
label_7bf5:
    v3 = dat_385e5;
    v2 = dat_385e4;
    v9 = (char *)sub_7510((unsigned short)v4);
    v10 = (char *)sub_239f0(0,dat_3868c,a1->field_0x38);
    __fprintf_chk(stderr,1,"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",v10,v9,v5,v13,(unsigned int)v2,(unsigned int)v3);
    v4 = a1->field_0x68;
  }
  if (!v13) {
    dat_385f0 = a1->field_0x30;
    dat_385e4 = CONCAT11(dat_385e5,2 <= (unsigned short)(v4 - 10U));
    dat_385f8 = a0->field_0x2c;
    v6 = (struct_2 *)sub_b580();
    sub_e7b0(a1->field_0x38,&v14,v6);
    if (dat_38600)
      sub_1a750(a0,a1,4);
  }
label_793d:
  if (a1->field_0x68 == 6) {
    dat_38600 = 0;
    return;
  }
}


// Function: sub_7e10 @ 0x7e10
unsigned long sub_7e10(char *a0) // return-dupe
{
  int v1;
  unsigned long v10; // stack - 0x40
  struct_16 *v2; // rax
  int *v3; // rax
  long v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x48
  unsigned int v8;
  int v9; // r13d
  
  dat_385fc = (unsigned int)strlen(a0);
  if ((dat_38018 & 0x200) && (dat_385f8 = 0xffffff9c, dat_38010 < 0))
    dat_38010 = -100;
  v10 = 0;
  if (dat_38668 != 1) { // branch-flip
    if (dat_38668 != 2) { // branch-flip
      v8 = dat_38018;
      if (!dat_38668) {
        dat_38018 |= 0x10;
        v8 = dat_38018;
      }
    }
    else {
      dat_38018 |= 0x11;
      v8 = dat_38018;
    }
  }
  else {
    dat_38018 |= 3;
    v8 = dat_38018;
  }
  if (dat_3862d) {
    dat_38018 = v8 | 0x40;
    v8 = dat_38018;
  }
  v9 = -0x80000000;
  v7 = a0;
  v2 = (struct_16 *)sub_199d0(&v7,v8,0);
  v3 = __errno_location();
  if (!v2) {
    v5 = (char *)sub_e8a0(0,a0);
    v6 = dcgettext(NULL,"cannot search %s",5);
    error(0,*v3,v6,v5);
    dat_38604 = 1;
    return 1;
  }
  while( true ) {
    *v3 = 0;
    v4 = sub_19ff0(v2);
    if (!v4) break;
    v1 = (int)*(unsigned long *)(v4 + 0x58);
    if ((dat_38608) && (v9 != v1)) {
      sub_d9f0();
      v1 = *(int *)(v4 + 0x58);
    }
    v9 = v1;
    dat_385e8 = *(int *)(v4 + 0x88);
    dat_38618 = 0;
    dat_385e4 = 0;
    dat_385e5 = dat_385e8 != 0;
    sub_7670(v2);
  }
  if (*v3) { // branch-flip
    a0 = (char *)sub_e8a0(0,a0);
    v5 = "failed to read file names from file system at or below %s";
  }
  else {
    if (!sub_19e00(v2))
      return 1;
    v5 = dcgettext(NULL,"failed to restore working directory after searching %s",5);
  }
  error(0,*v3,v5,a0);
  dat_38604 = 1;
  return 0;
}


// Function: sub_8070 @ 0x8070
unsigned long sub_8070(int a0,unsigned long *a1)
{
  FILE *v1;
  unsigned long v10; // rax
  int v11; // stack - 0x16c
  stat v12; // stack - 0x168
  unsigned short v13; // stack - 0x42
  stat v14; // stack - 0xd8
  unsigned long v15; // rax
  FILE *v16; // stack - 0x180
  int v2; // eax
  char *v3;
  struct_28 *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  if ((1 <= a0) && (v3 = (char *)*a1, !sub_e120(v3,1))) {
    v16 = dat_38690;
    if (dat_38690) {
      v3 = (char *)sub_e8a0(0,(char *)*a1);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v3);
      error(1,0,"%s",dcgettext(NULL,"file operands cannot be combined with -files0-from",5));
      return v10;
    }
    v3 = NULL;
    v4 = (struct_28 *)sub_17670(a1);
  }
  else {
    v1 = dat_38690;
    if (!dat_38690) {
      v13 = 0x2e;
      return (unsigned long)(unsigned int)sub_7e10(&v13);
    }
    if (strcmp((char *)dat_38690,"-")) { // branch-flip
      v3 = (char *)sub_e8a0(0,v1);
      v16 = fopen((char *)dat_38690,"r");
      if (!v16) {
        v5 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v5,v3);
        return v8;
      }
      v2 = fileno(v16);
      if (v2 < 0)
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints"); // no-return
      if ((((dat_38698) && (!fstat(v2,&v12))) && (!fstat(0,&v14))) && ((v12._8_8_ == v14._8_8_ && (v12._0_8_ == v14._0_8_)))) {
        error(1,0,"%s: %s\n",dcgettext(NULL,"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir",5),v3);
        return v6;
      }
      sub_17990(v2,1);
    }
    else {
      if (dat_38698) {
        error(1,0,"%s\n",dcgettext(NULL,"option -files0-from reading from standard input cannot be combined with -ok, -okdir",5));
        return v9;
      }
      v3 = (char *)sub_e8a0(0,dcgettext(NULL,"(standard input)",5));
      v16 = stdin;
    }
    v4 = (struct_28 *)sub_176a0(v16);
  }
  if (!v4)
    sub_25f50(); // no-return, return-dupe
  v15 = 1;
  do {
    while( true ) {
      v5 = (char *)sub_176e0(v4,&v11);
      if (!v5) {
        if (v11 == 3) {
          sub_25f50();
        }
        if (v11 == 4) {
          v5 = dcgettext(NULL,"%s: read error",5);
          error(0,*__errno_location(),v5,v3);
          dat_38604 = 1;
          sub_177a0(v4);
          return 0;
        }
        if (v11 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,"process_all_startpoints"); // no-return
        goto label_8176;
      }
      if (*v5) break;
      if (dat_38690) { // branch-flip
        v15 = sub_17780(v4);
        error(0,0,"%s:%lu: %s",v3,v15,dcgettext(NULL,"invalid zero-length file name",5));
      }
      else {
        error(0,2,"%s",(char *)sub_e8a0(0,v5));
      }
      dat_38604 = 1;
      v15 = 0;
    }
    if ((!dat_38690) && (sub_e120(v5,1))) goto label_8176;
    dat_385fc = (unsigned int)strlen(v5);
  } while (sub_7e10(v5));
  v15 = 0;
label_8176:
  sub_177a0(v4);
  if ((((char)v15) && (dat_38690)) && ((ferror(v16) || (sub_17d40(v16))))) {
    error(1,0,dcgettext(NULL,"error reading %s",5),v3);
    return v7;
  }
  return v15;
}


// Function: sub_8530 @ 0x8530
unsigned long sub_8530(unsigned int *a0)
{
  *a0 = dat_38018;
  return 1;
}


// Function: sub_8550 @ 0x8550
void sub_8550(char *a0,char *a1)
{
  char *v1; // rax
  
  sub_180c0(stdout);
  v1 = dcgettext(NULL,"< %s ... %s > ? ",5);
  if (0 <= __fprintf_chk(stderr,1,v1,a0,a1)) {
    sub_180c0(stderr);
    sub_268c0(); // tail-call
    return;
  }
  v1 = dcgettext(NULL,"Failed to write prompt for -ok",5);
  error(1,*__errno_location(),v1);
}


// Function: sub_8870 @ 0x8870
void sub_8870(FILE *a0,long *a1)
{
  if (!a1)
    return;
  while( true ) {
    if (((void *)*a1 != sub_8940) && ((void *)*a1 != sub_8990)) break;
    if (a1[0x22]) goto label_88be;
    a1 = (long *)a1[0x23];
    if (!a1)
      return;
  }
  if ((!a1[0x22]) && (!a1[0x23])) {
    sub_86e0(a0,a1); // tail-call
    return;
  }
label_88be:
  __fprintf_chk(a0,1,"%s"," ( ");
  sub_86e0(a0,a1);
  __fprintf_chk(a0,1,"%s"," ) "); // tail-call
  return;
}


// Function: sub_89e0 @ 0x89e0
unsigned long sub_89e0(long a0,long a1,void *a2,int a3) // early-return x3
{
  int v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  double v6; // xmm0_qa
  double v7; // xmm0_qa
  
  v1 = *(int *)((long)a2 + 0x3c);
  if (v1 == 1) {
    v2 = *(long *)((long)a2 + 0x48);
    if ((a0 == *(long *)((long)a2 + 0x40)) && (v2 == a1))
      return 0;
    v3 = difftime(a0,*(long *)((long)a2 + 0x40));
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),(double)(a1 - v2) * dat_28950 + v4 < 0.0);
  }
  if (v1 == 2) {
    difftime(a0,*(unsigned long *)((long)a2 + 0x40));
    v7 = (double)(a1 - *(long *)((long)a2 + 0x48)) * dat_28950 + v6;
    if (v7 <= dat_28958)
      return 0;
    return (unsigned long)(v7 <= (double)a3);
  }
  if (!v1) {
    v2 = *(long *)((long)a2 + 0x48);
    if ((a0 == *(long *)((long)a2 + 0x40)) && (a1 == v2))
      return 0;
    v3 = difftime(a0,*(long *)((long)a2 + 0x40));
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(a1 - v2) * dat_28950 + v5);
  }
  __assert_fail("0","pred.c",0x8a,"pred_timewindow"); // no-return
}


// Function: sub_8b10 @ 0x8b10
unsigned long sub_8b10(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x50);
  v2 = *(long *)((long)a1 + 0x48);
  return sub_89e0(v2,v1,a2,0x3c); // tail-call
}


// Function: sub_8b30 @ 0x8b30
unsigned long sub_8b30(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer"); // no-return
  v1 = *(long *)((long)a1 + 0x50);
  v2 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a1 + 0x48) == *(long *)((long)a2 + 0x40)) && (v1 == v2))
    return 0;
  v3 = difftime();
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(v1 - v2) * dat_28950 + v4);
}


// Function: sub_8bc0 @ 0x8bc0
unsigned long sub_8bc0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x50);
  v2 = *(long *)((long)a1 + 0x48);
  return sub_89e0(v2,v1,a2,0x15180); // tail-call
}


// Function: sub_8bf0 @ 0x8bf0
unsigned long sub_8bf0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a1 + 0x68);
  return sub_89e0(v2,v1,a2,0x3c); // tail-call
}


// Function: sub_8c10 @ 0x8c10
unsigned long sub_8c10(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer"); // no-return
  v1 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a1 + 0x68) == *(long *)((long)a2 + 0x40)) && (v1 == v2))
    return 0;
  v3 = difftime();
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(v1 - v2) * dat_28950 + v4);
}


// Function: sub_8ca0 @ 0x8ca0
unsigned long sub_8ca0(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  if (*(struct_2 **)((long)a2 + 0x110))
    sub_e7b0(a0,a1,*(struct_2 **)((long)a2 + 0x110));
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_e7b0(a0,a1,v1); // tail-call
}


// Function: sub_8ce0 @ 0x8ce0
unsigned long sub_8ce0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a1 + 0x68);
  return sub_89e0(v2,v1,a2,0x15180); // tail-call
}


// Function: sub_8d00 @ 0x8d00
char sub_8d00(char *a0,void *a1) // return-dupe
{
  int *v1; // rax
  char *v2;
  char *v3; // rax
  int v4; // r13d
  
  v2 = dat_385f0;
  if (!strcmp(dat_385f0,"."))
    return '\x01';
  v4 = 0;
  if (dat_385e4)
    v4 = (unsigned int)((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x4000) << 9;
  if (!unlinkat(dat_385f8,v2,v4))
    return '\x01';
  v1 = __errno_location();
  if (*v1 != 2) { // branch-flip
    if (((*v1 == 0x15) && (!v4)) && (!unlinkat(dat_385f8,dat_385f0,0x200)))
      return '\x01';
  }
  else if (dat_3862e) {
    *v1 = 0;
    return dat_3862e;
  }
  v2 = (char *)sub_e8a0(0,a0);
  v3 = dcgettext(NULL,"cannot delete %s",5);
  error(0,*v1,v3,v2);
  dat_38604 = 1;
  return '\0';
}


// Function: sub_8e20 @ 0x8e20
bool sub_8e20(char *a0,void *a1)
{
  unsigned int v1; // eax
  int v2; // eax
  int *v3; // rax
  DIR *v4; // rax
  dirent *v5; // rax
  char *v6; // rax
  bool v7; // r12b
  
  v1 = *(unsigned int *)((long)a1 + 0x18) & 0xf000;
  if (v1 != 0x4000) {
    if (v1 != 0x8000)
      return 0;
    return *(long *)((long)a1 + 0x30) == 0;
  }
  v3 = __errno_location();
  *v3 = 0;
  v2 = sub_1f080(dat_385f8,dat_385f0,0x90900);
  if (0 <= v2) {
    v4 = fdopendir(v2);
    if (!v4) {
      v6 = (char *)sub_e8a0(0,a0);
      error(0,*v3,"%s",v6);
      dat_38604 = 1;
      close(v2);
      return 0;
    }
    *v3 = 0;
    do {
      v5 = readdir(v4);
      if (!v5) {
        v7 = 1;
        goto label_8ed3;
      }
    } while ((v5->field_0x13 == '.') && ((!v5->field_0x14 || (*(short *)&v5->field_0x14 == 0x2e))));
    v7 = 0;
label_8ed3:
    if (*v3) {
      v6 = (char *)sub_e8a0(0,a0);
      error(0,*v3,"%s",v6);
      dat_38604 = 1;
      closedir(v4);
      return 0;
    }
    if (!closedir(v4))
      return v7;
  }
  v6 = (char *)sub_e8a0(0,a0);
  error(0,*v3,"%s",v6);
  dat_38604 = 1;
  return 0;
}


// Function: sub_8fc0 @ 0x8fc0
void sub_8fc0(void)
{
  sub_a040(dat_385f0); // tail-call
}


// Function: sub_8fd0 @ 0x8fd0
unsigned long sub_8fd0(void)
{
  return 0;
}


// Function: sub_9090 @ 0x9090
unsigned int sub_9090(unsigned long a0,long *a1,void *a2)
{
  char *v1;
  unsigned int v2; // eax
  
  v1 = *(char **)((long)a2 + 0x38);
  v2 = strcmp((char *)sub_108e0(a1,a0),v1);
  return CONCAT31((undefined3)(v2 >> 8),v2 == 0);
}


// Function: sub_90c0 @ 0x90c0
unsigned long sub_90c0(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x20) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < (unsigned long)*(unsigned int *)((long)a1 + 0x20));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x20) == *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
}


// Function: sub_9110 @ 0x9110
unsigned int sub_9110(unsigned long a0,void *a1,void *a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)((long)a1 + 0x20) >> 8),*(int *)((long)a2 + 0x38) == *(int *)((long)a1 + 0x20));
}


// Function: sub_9120 @ 0x9120
unsigned long sub_9120(char *a0,void *a1,void *a2)
{
  if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0xa000)
    return 0;
  return sub_8660(a0,a2,1); // tail-call
}


// Function: sub_9150 @ 0x9150
int sub_9150(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  unsigned int v4; // eax
  char *v5;
  
  v5 = *(char **)((long)a2 + 0x38);
  v5 = (char *)sub_17b90(a0);
  v3 = v5;
  sub_17d00(v5);
  v4 = fnmatch(v5,v5,0x10);
  v2 = v4;
  free(v3);
  v1 = v4 >> 8;
  v4 = CONCAT31((undefined3)v1,v2 == 0);
  return v4;
}


// Function: sub_9190 @ 0x9190
unsigned long sub_9190(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)((long)a1 + 8) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < *(unsigned long *)((long)a1 + 8));
  }
  return CONCAT71(v2,*(long *)((long)a1 + 8) == *(long *)((long)a2 + 0x40)) & 0xffffffff;
}


// Function: sub_91e0 @ 0x91e0
unsigned int sub_91e0(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1; // eax
  
  v1 = fnmatch(*(char **)((long)a2 + 0x38),a0,0x10);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_9210 @ 0x9210
unsigned long sub_9210(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)((long)a1 + 0x10) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < *(unsigned long *)((long)a1 + 0x10));
  }
  return CONCAT71(v2,*(long *)((long)a1 + 0x10) == *(long *)((long)a2 + 0x40)) & 0xffffffff;
}


// Function: sub_9260 @ 0x9260
unsigned long sub_9260(char *a0,void *a1,void *a2)
{
  if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0xa000)
    return 0;
  return sub_8660(a0,a2,0); // tail-call
}


// Function: sub_9290 @ 0x9290
unsigned long sub_9290(unsigned char *a0,void *a1,void *a2)
{
  sub_165e0(a0,dat_385f8,dat_385f0,a1,dat_38638,dat_3865c,(unsigned char)*(char *)((long)a2 + 0x28),*(FILE **)((long)a2 + 0x40));
  return 1;
}


// Function: sub_92d0 @ 0x92d0
unsigned long sub_92d0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x60);
  v2 = *(long *)((long)a1 + 0x58);
  return sub_89e0(v2,v1,a2,0x3c); // tail-call
}


// Function: sub_92f0 @ 0x92f0
unsigned long sub_92f0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x60);
  v2 = *(long *)((long)a1 + 0x58);
  return sub_89e0(v2,v1,a2,0x15180); // tail-call
}


// Function: sub_9310 @ 0x9310
int sub_9310(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  unsigned int v4; // eax
  char *v5;
  
  v5 = *(char **)((long)a2 + 0x38);
  v5 = (char *)sub_17b90(a0);
  v3 = v5;
  sub_17d00(v5);
  v4 = fnmatch(v5,v5,0);
  v2 = v4;
  free(v3);
  v1 = v4 >> 8;
  v4 = CONCAT31((undefined3)v1,v2 == 0);
  return v4;
}


// Function: sub_9350 @ 0x9350
unsigned int sub_9350(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_e7b0(a0,a1,v1) ^ 1;
}


// Function: sub_9370 @ 0x9370
unsigned long sub_9370(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer"); // no-return
  v1 = *(long *)((long)a1 + 0x60);
  v2 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a1 + 0x58) == *(long *)((long)a2 + 0x40)) && (v1 == v2))
    return 0;
  v3 = difftime();
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(v1 - v2) * dat_28950 + v4);
}


// Function: sub_9400 @ 0x9400
unsigned long sub_9400(char *a0,void *a1,void *a2) // early-return
{
  long v1;
  unsigned long v2; // rax
  long v3;
  long v4;
  double v5; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY"); // no-return
  switch(*(unsigned int *)((long)a2 + 0x38)) {
    case 0:
      v4 = *(long *)((long)a1 + 0x48);
      v3 = *(long *)((long)a1 + 0x50);
      break;
    case 1:
      v2 = sub_e8a0(0,a0);
      error(0,0,dcgettext(NULL,"WARNING: cannot determine birth time of file %s",5),v2);
      return 0;
    case 2:
      v4 = *(long *)((long)a1 + 0x68);
      v3 = *(long *)((long)a1 + 0x70);
      break;
    case 3:
      v4 = *(long *)((long)a1 + 0x58);
      v3 = *(long *)((long)a1 + 0x60);
      break;
    case 4:
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY"); // no-return
    default:
      __assert_fail("collected","pred.c",0x29b,"pred_newerXY"); // no-return
    
  }
  v1 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a2 + 0x40) == v4) && (v3 == v1))
    return 0;
  v2 = difftime();
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),0.0 <= (double)(v3 - v1) * dat_28950 + v5);
}


// Function: sub_9540 @ 0x9540
unsigned long sub_9540(unsigned long a0,void *a1)
{
  group *v1; // rax
  
  v1 = getgrgid(*(unsigned int *)((long)a1 + 0x20));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}


// Function: sub_9560 @ 0x9560
unsigned long sub_9560(unsigned long a0,void *a1)
{
  passwd *v1; // rax
  
  v1 = getpwuid(*(unsigned int *)((long)a1 + 0x1c));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}


// Function: sub_9580 @ 0x9580
unsigned long sub_9580(char *a0,unsigned long a1,struct_34 *a2)
{
  char *v1;
  
  v1 = *(char **)a2->field_0xe8;
  if (!sub_8550(v1,a0))
    return 0;
  return sub_a040(a0,a1,a2); // tail-call
}


// Function: sub_95d0 @ 0x95d0
unsigned long sub_95d0(char *a0,unsigned long a1,struct_34 *a2)
{
  char *v1;
  unsigned long v2; // rax
  
  v1 = *(char **)a2->field_0xe8;
  if (!sub_8550(v1,a0))
    return 0;
  v2 = sub_a040(dat_385f0,a1,a2); // tail-call
  return v2;
}


// Function: sub_9640 @ 0x9640
unsigned int sub_9640(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1; // eax
  
  v1 = fnmatch(*(char **)((long)a2 + 0x38),a0,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_9660 @ 0x9660
unsigned long sub_9660(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  unsigned int v1;
  unsigned int v2;
  int v3;
  undefined7 v4; // r8
  
  v1 = *(unsigned int *)((long)a1 + 0x18);
  v2 = *(unsigned int *)((long)a2 + (unsigned long)((v1 & 0xf000) == 0x4000) * 4 + 0x3c);
  v3 = *(int *)((long)a2 + 0x38);
  if (v3 == 1) {
    if (!v2)
      return 1;
    return (unsigned long)((v1 & v2) != 0);
  }
  v4 = (undefined7)((unsigned long)a4 >> 8);
  if (v3 != 2) {
    if (v3)
      abort(); // no-return
    return CONCAT71(v4,(v1 & v2) == v2) & 0xffffffff;
  }
  return CONCAT71(v4,(v1 & 0xfff) == v2) & 0xffffffff;
}


// Function: sub_96e0 @ 0x96e0
unsigned int sub_96e0(void)
{
  unsigned int v1; // eax
  
  v1 = faccessat(dat_385f8,dat_385f0,1,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_9710 @ 0x9710
unsigned int sub_9710(void)
{
  unsigned int v1; // eax
  
  v1 = faccessat(dat_385f8,dat_385f0,4,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_9740 @ 0x9740
unsigned int sub_9740(void)
{
  unsigned int v1; // eax
  
  v1 = faccessat(dat_385f8,dat_385f0,2,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_97e0 @ 0x97e0
unsigned long sub_97e0(char *a0,unsigned long a1,void *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  v2 = strlen(a0);
  v1 = re_match(*(re_pattern_buffer **)((long)a2 + 0x38),a0,(int)v2,0,NULL);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == (int)v2);
}


// Function: sub_9820 @ 0x9820
unsigned long sub_9820(unsigned long a0,void *a1,void *a2)
{
  int v1;
  unsigned long v2; // rax
  undefined7 v3; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (*(long *)((long)a1 + 0x30) / (long)*(int *)((long)a2 + 0x3c) + 1) - (unsigned long)(*(long *)((long)a1 + 0x30) % (long)*(int *)((long)a2 + 0x3c) == 0);
  v3 = (undefined7)((unsigned long)a1 >> 8);
  if (v1 == 1)
    return CONCAT71(v3,v2 < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < v2);
  }
  return CONCAT71(v3,*(unsigned long *)((long)a2 + 0x40) == v2) & 0xffffffff;
}


// Function: sub_9890 @ 0x9890
unsigned long sub_9890(char *a0,struct_12 *a1,void *a2)
{
  unsigned long v1; // rax
  
  if (((!a1->field_0x8) || (a1->field_0x8 == *(long *)((long)a2 + 0x38))) && (v1 = sub_d820(a0,dat_385f0,a1), !(int)v1)) {
    if (a1->field_0x8 != *(long *)((long)a2 + 0x38))
      return v1;
    return CONCAT71((undefined7)((unsigned long)*(long *)((long)a2 + 0x40) >> 8),a1->field_0x0 == *(long *)((long)a2 + 0x40));
  }
  return 0;
}


// Function: sub_9900 @ 0x9900
char sub_9900(unsigned long a0,void *a1,long a2)
{
  unsigned int v1;
  long v2; // rcx
  char v3;
  
  if (!dat_385e5)
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type"); // no-return
  v3 = 0;
  if (dat_385e8) {
    v1 = dat_385e8;
    if (dat_385e4)
      v1 = *(unsigned int *)((long)a1 + 0x18);
    v1 &= 0xf000;
    if (v1 != 0x6000) { // branch-flip
      if (0x6001 <= v1) { // branch-flip
        if (v1 != 0xa000) { // branch-flip
          v2 = 6;
          if (v1 != 0xc000) {
            if (v1 != 0x8000)
              return 0;
            v2 = 3;
          }
        }
        else {
          v2 = 4;
        }
      }
      else if (v1 != 0x2000) { // branch-flip
        v2 = 2;
        if ((v1 != 0x4000) && (v2 = 5, v1 != 0x1000))
          return 0;
      }
      else {
        v2 = 1;
      }
    }
    else {
      v2 = 0;
    }
    v3 = *(char *)(a2 + 0x38 + v2);
  }
  return v3;
}


// Function: sub_99e0 @ 0x99e0
unsigned long sub_99e0(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x1c) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < (unsigned long)*(unsigned int *)((long)a1 + 0x1c));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x1c) == *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
}


// Function: sub_9a30 @ 0x9a30
unsigned long sub_9a30(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  long v4;
  long v5;
  double v6; // xmm0_qa
  
  v1 = *(long *)((long)a1 + 0x50);
  v4 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a1 + 0x48);
  v5 = *(long *)((long)a1 + 0x68);
  if (v1 != v4 || v2 != v5) { // branch-flip
    difftime(v2,v5);
    if ((double)(v1 - v4) * dat_28950 + v6 < 0.0)
      return 0;
    v5 -= v2;
    v4 -= v1;
    if (v4 <= -1) {
      v4 += 1000000000;
      v5 -= 1;
    }
  }
  else {
    v4 = 0;
    v5 = 0;
  }
  v3 = sub_89e0(v5,v4,a2,0x15180); // tail-call
  return v3;
}


// Function: sub_9ae0 @ 0x9ae0
unsigned int sub_9ae0(unsigned long a0,void *a1,void *a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)((long)a1 + 0x1c) >> 8),*(int *)((long)a2 + 0x38) == *(int *)((long)a1 + 0x1c));
}


// Function: sub_9af0 @ 0x9af0
int * sub_9af0(char *a0,void *a1,long a2) // early-return, ternary
{
  char v1;
  int *v2;
  char v3; // al
  int v4; // eax
  int *v5; // rax
  void *v6; // rbx
  char v7 [152];
  
  v3 = sub_dfb0();
  v6 = (v3) ? sub_de70 : sub_ddb0; // branch-flip
  sub_d810(v7);
  v4 = (*v6)(dat_385f0,v7);
  if (v4) { // branch-flip
    v3 = sub_dfb0();
    v1 = v3;
    v5 = __errno_location();
    v2 = v5;
    if ((!v1) || (*v5 != 2)) {
      v5 = (char *)sub_e8a0(0,a0);
      error(0,*v2,"%s",(char *)v5);
      v5 = NULL;
      dat_38604 = 1;
      return v5;
    }
    v5 = (unsigned long)sub_9900(a0,a1,a2);
  }
  else {
    v5 = (unsigned long)sub_9900(a0,v7,a2);
  }
  return v5;
}


// Function: sub_9c00 @ 0x9c00
unsigned long sub_9c00(char *a0,unsigned long a1,void *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3;
  char *v4; // stack - 0x38
  unsigned long v5; // r12
  unsigned long v6; // r12
  
  v1 = (*dat_38680)(dat_385f8,dat_385f0,&v4);
  if (0 <= v1) { // branch-flip
    v3 = *(char **)((long)a2 + 0x38);
    v6 = CONCAT71((undefined7)((unsigned long)v5 >> 8),fnmatch(v3,v4,0) == 0);
    freecon(v4);
  }
  else {
    v2 = sub_e8a0(0,a0);
    v3 = dcgettext(NULL,"getfilecon failed: %s",5);
    v6 = 0;
    error(0,*__errno_location(),v3,v2);
  }
  return v6 & 0xffffffff;
}


// Function: sub_9cd0 @ 0x9cd0
void sub_9cd0(FILE *a0,void *a1)
{
  char *v1;
  int v2; // eax
  char *v3;
  int v4; // edx
  char v5 [264];
  
  do {
    if (!a1) {
      __fprintf_chk(a0,1,"\n");
      return;
    }
    v3 = NULL;
    if (*(char **)((long)a1 + 8)) {
      __strcpy_chk(v5,*(char **)((long)a1 + 8),0x100);
      v2 = (int)strlen(v5);
      v4 = v2 + -1;
      if (0 <= v4) { // branch-flip
        v1 = &v5[v4];
        do {
          v3 = v1;
          if ((*v3 != ' ') && (*v3 != '\t')) {
            v3 = &v5[v4 + 1];
            break;
          }
          v4 -= 1;
          v1 = &v3[-1];
        } while (v4 != -1);
      }
      else {
        v3 = &v5[v2];
      }
      *v3 = '\0';
      v3 = v5;
    }
    __fprintf_chk(a0,1,"[%s] ",v3);
    a1 = *(void **)((long)a1 + 0x108);
  } while( true );
}


// Function: sub_9e00 @ 0x9e00
void sub_9e00(void *a0)
{
  if (!(dat_38660 & 0x40))
    return;
  __fprintf_chk(stderr,1,"Predicate success rates after completion:\n");
  if (a0)
    sub_86e0(stderr,a0);
  __fprintf_chk(stderr,1,"\n"); // tail-call
}


// Function: sub_9e60 @ 0x9e60
void sub_9e60(long *a0)
{
  void *v1;
  int *v2;
  
  if (!a0)
    return;
  do {
    v1 = (void *)*a0;
    if (!v1)
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check"); // no-return
    v2 = (int *)a0[0x26];
    if (!v2)
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check"); // no-return
    if ((v1 != *(void **)&v2[6]) && (*(void **)&v2[6]))
      __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check"); // no-return
    switch(*v2) {
      case 0:
      case 2:
        if (!*v2)
          __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check"); // no-return
        __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,"pred_sanity_check"); // no-return
      case 1:
      case 3:
      case 4:
      case 5:
        if (*(char *)((long)a0 + 0x19))
          __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check"); // no-return
        if ((char)a0[3])
          __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check"); // no-return
        break;
      case 6:
        if (!(char)a0[3])
          __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check"); // no-return
        if (((v1 != sub_8600) && (v1 != sub_85e0)) && (!*(char *)((long)a0 + 0x19)))
          __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check"); // no-return
      
    }
    a0 = (long *)a0[0x21];
    if (!a0)
      return;
  } while( true );
}


// Function: sub_a040 @ 0xa040
unsigned long sub_a040(char *a0,unsigned long a1,struct_34 *a2)
{
  struct_7 *v1;
  long v10; // rbx
  char v11;
  bool v12; // zf
  char *v13; // stack - 0x50
  unsigned char v14; // stack - 0x41
  char *v15; // stack - 0x40
  struct_6 *v2;
  unsigned int v3;
  unsigned long v4; // rax
  char *v5; // rax
  char *v6;
  unsigned int *v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v14 = sub_e880(a2->field_0x0);
  v6 = dat_385f0;
  if (v14) { // branch-flip
    if (!a2->field_0xd0) {
      v5 = strchr(dat_385f0,0x2f);
      v3 = dat_385f8;
      if (v5) { // branch-flip
        v6 = (char *)sub_17c40(v6);
        v3 = dat_385f8;
        v7 = (unsigned int *)sub_25a50(0x10);
        a2->field_0xf8 = (long)v7;
        *(unsigned long *)&v7[2] = 0;
        *v7 = sub_1f080(v3,v6,0);
        if (*(int *)a2->field_0xf8 < 0) {
          free(v6);
label_a33f:
          v8 = sub_e8a0(0,a0);
          v6 = dcgettext(NULL,"Failed to save working directory in order to run a command on %s",5);
          error(1,*__errno_location(),v6,v8);
          return v9;
        }
        sub_17990(*(int *)a2->field_0xf8,1);
        free(v6);
        v6 = dat_385f0;
      }
      else {
        v7 = (unsigned int *)sub_25a50(0x10);
        a2->field_0xf8 = (long)v7;
        *(unsigned long *)&v7[2] = 0;
        *v7 = sub_1f080(v3,".",0);
        if (*(int *)a2->field_0xf8 < 0) goto label_a33f;
        sub_17990(*(int *)a2->field_0xf8,1);
        v6 = dat_385f0;
      }
    }
    a0 = (char *)sub_17b90(v6);
    v15 = a0;
    v11 = (*a0 != '/') * '\x02';
    v13 = NULL;
    if (*a0 != '/')
      v13 = "./";
  }
  else {
    if (a2->field_0xf8 != dat_385c0)
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec"); // no-return
    v13 = NULL;
    v11 = '\0';
    v15 = NULL;
  }
  v4 = (unsigned long)a2->field_0x38;
  v2 = (struct_6 *)&a2->field_0x39[0x5f];
  v1 = (struct_7 *)&a2->field_0x39[7];
  if (a2->field_0x38) { // branch-flip
    sub_14fb0(v1,v2,a0,strlen(a0) + 1,v13,(unsigned char)v11,0);
    if (a2->field_0xd0)
      dat_38608 = 1;
    if (!v15) {
      return v4 & 0xffffffff; // return-dupe
    }
    if (!v14)
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec"); // return-dupe, no-return
  }
  else {
    v10 = 0;
    if (1 <= a2->field_0xf0) {
      do {
        v4 = strlen(a0);
        v6 = *(char **)(a2->field_0xe8 + v10 * 8);
        v10 += 1;
        sub_15270(v1,v2,v6,strlen(v6),v13,(unsigned char)v11,a0,v4,0);
      } while ((int)v10 < a2->field_0xf0);
    }
    sub_15430(v1,v2);
    v12 = !(a2->field_0x100 >> 8 & 0xff) && !(a2->field_0x100 & 0x7f);
    v4 = CONCAT71((undefined7)((unsigned long)v10 >> 8),v12);
    if (!v14) {
      if (v15) {
        __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
      }
      return v4 & 0xffffffff;
    }
    sub_242c0((struct_10 *)a2->field_0xf8);
    if (!v15) {
      return v4 & 0xffffffff;
    }
    v14 = v12;
  }
  free(v15);
  v4 = (unsigned long)v14;
  return v4 & 0xffffffff;
}


// Function: sub_a3a0 @ 0xa3a0
int * sub_a3a0(unsigned long a0,struct_53 *a1,unsigned long a2,char **a3) // return-dupe
{
  long v1;
  unsigned long v10; // stack - 0x3a
  char *v11;
  char *v12; // r12
  unsigned short v13; // stack - 0x32
  struct_9 *v2;
  int v3;
  int *v4;
  char v5; // al
  int v6; // eax
  int *v7; // rax
  unsigned int v8;
  unsigned long v9; // rbx
  
  if (dat_38660 & 0x20) {
    v9 = 0;
    __fprintf_chk(stderr,1,"DebugExec: launching process (argc=%lu):",a1->field_0x60 - 1);
    if (a1->field_0x60 != 1) {
      do {
        v1 = v9 * 8;
        v9 += 1;
        v7 = (unsigned long)sub_e8a0(0,*(char **)(a1->field_0x68 + v1));
        __fprintf_chk(stderr,1," %s",v7);
        v7 = (unsigned long)(a1->field_0x60 - 1);
      } while (v9 < v7);
    }
    __fprintf_chk(stderr,1,"\n");
  }
  sub_180c0(stdout);
  sub_180c0(stderr);
  if (dat_3801c) {
    dat_3801c = 0;
    signal(0x11,0);
  }
  v6 = fork();
  v3 = v6;
  if (v6 == -1) {
    v7 = (char *)dcgettext(NULL,"cannot fork",5);
    v11 = (char *)v7;
    v7 = __errno_location();
    error(1,*v7,v11);
    return v7;
  }
  if (v6) {
    while (v6 = waitpid(v3,(int *)&a1->field_0xc8,0), v6 == -1) {
      v7 = __errno_location();
      v4 = v7;
      if (*v7 != 4) {
        v7 = (char *)sub_e8a0(0,*a3);
        v11 = (char *)v7;
        v7 = (char *)dcgettext(NULL,"error waiting for %s",5);
        error(0,*v4,(char *)v7,v11);
        dat_38604 = 1;
        v7 = NULL;
        return v7;
      }
    }
    v8 = a1->field_0xc8;
    v5 = ((unsigned char)v8 & 0x7f) + 1;
    if ('\x02' <= v5) { // branch-flip
      v7 = (char *)sub_239f0(0,dat_3868c,*a3);
      v11 = (char *)v7;
      v7 = (char *)dcgettext(NULL,"%s terminated by signal %d",5);
      error(0,0,(char *)v7,v11,v8 & 0x7f);
    }
    else {
      v8 = v8 >> 8 & 0xff;
      if (dat_38660 & 0x20)
        __fprintf_chk(stderr,1,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)v3,v8);
      if (!v8) {
        v7 = (int *)0x1;
        return v7;
      }
    }
    if (!a1->field_0x0) {
      v7 = (int *)0x1;
      return v7;
    }
    v7 = (int *)0x1;
    dat_38604 = 1;
    return v7;
  }
  v2 = a1->field_0xc0;
  if (!v2)
    __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch"); // no-return
  v7 = __errno_location();
  v4 = v7;
  if (a1->field_0xbc) { // branch-flip
    v10 = 0x6c756e2f7665642f;
    v13 = 0x6c;
    v6 = close(0);
    if (0 <= v6) {
      v6 = sub_18040(&v10,0);
      if (0 <= v6) goto label_a437;
      v7 = (unsigned long)sub_e8a0(0,&v10);
      error(0,*v4,"%s",v7);
      v6 = sub_24290(v2);
      goto label_a7ad;
    }
    v7 = (char *)dcgettext(NULL,"Cannot close standard input",5);
    error(0,*v4,(char *)v7);
    v6 = sub_24290(v2);
    if (!v6) {
      _exit(1); // return-dupe, no-return
    }
  }
  else {
label_a437:
    v6 = sub_24290(v2);
label_a7ad:
    if (!v6) {
      v5 = sub_16220();
      if (v5)
        sub_16240();
      v5 = sub_15970(a3);
      if (v5) // branch-flip
        *v4 = 7;
      else {
        execvp(*a3,a3);
      }
      v7 = (unsigned long)sub_e8a0(0,*a3);
      error(0,*v4,"%s",v7);
      _exit(1);
    }
  }
  if (0 <= v2->field_0x0) { // branch-flip
label_a614:
    v11 = "";
    v12 = "";
  }
  else {
    v11 = v2->field_0x8;
    v12 = ": ";
    if (!v11) goto label_a614;
  }
  v7 = (char *)dcgettext(NULL,"Failed to change directory%s%s",5);
  error(0,*v4,(char *)v7,v12,v11);
  _exit(1);
}


// Function: sub_a840 @ 0xa840
void sub_a840(void *a0,char a1)
{
  long v1;
  
  while( true ) {
    if ((a1) && (*(int *)((long)a0 + 0x10) != 3))
      __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization"); // no-return
    v1 = *(long *)((long)a0 + 0x110);
    if (v1) {
      if (*(int *)(v1 + 0x10) != 3)
        __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization"); // no-return
      sub_a840(v1,0);
    }
    a0 = *(void **)((long)a0 + 0x118);
    if (!a0) break;
    a1 = 0;
  }
}


// Function: sub_a8c0 @ 0xa8c0
char * sub_a8c0(long *a0,int a1,void *a2)
{
  short v1;
  long v10;
  int v2;
  unsigned int v3;
  unsigned long v4;
  char *v5;
  char *v6; // rax
  long v7; // rdx
  long v8;
  long *v9;
  
  v10 = *a0;
  if (!v10) {
    v6 = dcgettext(NULL,"invalid expression",5);
    error(1,0,v6);
    return v6;
  }
  v9 = a0;
  switch(*(unsigned int *)(v10 + 0x10)) {
    case 0:
label_ab90:
      v6 = dcgettext(NULL,"invalid expression",5);
      error(1,0,v6);
label_abb4:
      v6 = dcgettext(NULL,"oops -- invalid expression type!",5);
      error(1,0,v6);
      return v6;
    case 1:
      v8 = *(long *)(v10 + 0x108);
      *a0 = v8;
      break;
    case 2:
label_a960:
      *v9 = *(long *)(v10 + 0x108);
      v6 = (unsigned long)sub_a8c0();
      *(char **)(v10 + 0x118) = v6;
      v8 = *a0;
      break;
    case 3:
      goto label_ab64;
    case 4:
      v8 = *(long *)(v10 + 0x108);
      if ((!v8) || (*(char *)(v8 + 0x29))) {
        v5 = *(char **)(v10 + 8);
        v6 = dcgettext(NULL,"invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'",5);
        error(1,0,v6,v5);
        return v6;
      }
      v2 = *(int *)(v8 + 0x10);
      *a0 = v8;
      if (v2 != 5) {
label_aa00:
        v6 = (long)sub_a8c0();
        v10 = *a0;
        if ((!v10) || (*(int *)(v10 + 0x10) != 5)) {
          v6 = dcgettext(NULL,"invalid expression; I was expecting to find a \')\' somewhere but did not see one.",5);
          error(1,0,v6);
          return v6;
        }
        v8 = *(long *)(v10 + 0x108);
        *a0 = v8;
        v10 = (long)v6;
        goto label_a983;
      }
      if (*(char *)(v10 + 0x29)) {
        v10 = *(long *)(v8 + 8);
        v6 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        v9 = (long *)0x1;
        error(1,0,v6,v10);
        goto label_a960;
      }
      goto label_ab40;
    case 5:
      if (!a2) {
        v5 = *(char **)(v10 + 8);
        v6 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        error(1,0,v6,v5);
        return v6;
      }
      if (2 <= (unsigned int)(*(int *)((long)a2 + 0x10) - 2U)) { // branch-flip
        if (!*(char *)(v10 + 0x29)) {
          v6 = dcgettext(NULL,"invalid expression; you have too many \')\'",5);
          error(1,0,v6);
          goto label_aa00;
        }
      }
      else if (!*(char *)(v10 + 0x29)) {
        v5 = *(char **)((long)a2 + 8);
        v6 = dcgettext(NULL,"expected an expression between \'%s\' and \')\'",5);
        error(1,0,v6,v5);
        return v6;
      }
      v10 = *(long *)((long)a2 + 8);
      v6 = dcgettext(NULL,"expected an expression after \'%s\'",5);
      error(1,0,v6,v10);
      v8 = v7;
      goto label_aa70;
    default:
      goto label_abb4;
    
  }
label_a983:
  if ((!v8) || (a1 = (int)(short)a1, *(int *)(v8 + 0x14) <= a1)) {
    v6 = (char *)v10;
    return v6;
  }
label_aa70:
  v3 = *(unsigned int *)(v8 + 0x10);
  if (v3 != 5) {
    while (v3 != 4) {
      if (4 < v3) goto label_ab00;
      if (v3 != 3) break;
      *(long *)(v8 + 0x110) = v10;
      v10 = *a0;
      v1 = *(short *)(v10 + 0x14);
      *a0 = *(long *)(v10 + 0x108);
      v6 = (unsigned long)sub_a8c0(a0,(int)v1);
      *(char **)(v10 + 0x118) = v6;
      v8 = *a0;
      if (!v8) {
        v6 = (char *)v10;
        return v6;
      }
      if (*(int *)(v8 + 0x14) <= a1) {
        v6 = (char *)v10;
        return v6;
      }
      v3 = *(unsigned int *)(v8 + 0x10);
    }
    v6 = dcgettext(NULL,"invalid expression",5);
    error(1,0,v6);
label_ab00:
    if (v3 != 5) {
      v6 = dcgettext(NULL,"oops -- invalid expression type (%d)!",5);
      error(1,0,v6,v3);
      return v6;
    }
    if (v10) {
      v6 = (char *)v10;
      return v6;
    }
    v10 = 0;
  }
  v6 = dcgettext(NULL,"invalid expression",5);
  error(1,0,v6);
label_ab40:
  v6 = dcgettext(NULL,"invalid expression; empty parentheses are not allowed.",5);
  error(1,0,v6);
label_ab64:
  v4 = *(unsigned long *)(v10 + 8);
  v6 = dcgettext(NULL,"invalid expression; you have used a binary operator \'%s\' with nothing before it.",5);
  error(1,0,v6,v4);
  goto label_ab90;
}


// Function: sub_ad30 @ 0xad30
void sub_ad30(struct_42 *a0,int a1,unsigned long *a2)
{
  float v1;
  struct_41 *v2; // rax
  float v3;
  
  v2 = (struct_41 *)sub_25e50(0x138);
  v2->field_0x10 = 3;
  v2->field_0x14 = a1;
  v2->field_0x20 = 0;
  if (a1 != 2) { // branch-flip
    if (a1 != 3) {
      if (a1 != 1) {
        v2->field_0x118 = a0;
        *a2 = v2;
        return;
      }
      *a2 = v2;
      v2->field_0x8 = ",";
      v2->field_0x0 = sub_8ca0;
      *(unsigned int *)v2->field_0x24 = 0x3f800000;
      v2->field_0x118 = a0;
      return;
    }
    v1 = a0->field_0x24;
    v2->field_0x8 = "-a";
    v2->field_0x0 = sub_8990;
  }
  else {
    v1 = a0->field_0x24;
    v2->field_0x8 = "-o";
    v2->field_0x0 = sub_8940;
  }
  if (v1 <= dat_29578) {
    v3 = 0.0;
    if (0.0 <= v1)
      v3 = v1;
  }
  else {
    v3 = dat_29578;
  }
  v2->field_0x118 = a0;
  *(float *)v2->field_0x24 = v3;
  *a2 = v2;
}


// Function: sub_ae30 @ 0xae30
void sub_ae30(struct_39 *a0) // return-dupe
{
  char *v1;
  unsigned long v10;
  unsigned long *v11; // r14
  void *v12; // stack - 0x68
  unsigned int v13; // stack - 0x5c
  void *v2;
  void *v3;
  int v4; // eax
  unsigned int v5;
  unsigned long v6; // rcx
  void *v7; // stack - 0x48
  void *v8; // stack - 0x50
  unsigned long v9;
  
  if (!a0)
    return;
  sub_ae30(a0->field_0x118);
  sub_ae30(a0->field_0x110);
  v13 = 3;
  if ((!a0->field_0x1a) && (v13 = 1, !a0->field_0x1c))
    v13 = (unsigned int)a0->field_0x1b + (unsigned int)a0->field_0x1b;
  v12 = a0->field_0x0;
  if ((v12 != sub_8fb0) && (v12 != sub_8fc0)) {
    if (v12 != sub_ff90) { // branch-flip
      if (!dat_386b0) {
        v11 = (unsigned long *)0x380e0;
        qsort(0x380e0,0x3b,0x10,sub_acd0);
        do {
          v2 = (void *)v11[2];
          v3 = (void *)*v11;
          v8 = v2;
          v7 = v3;
          if (memcmp(&v8,&v7,8) < 0) {
            error(1,0,"failed to sort the costlookup array");
            return;
          }
          v8 = v3;
          v7 = v2;
          if (0 < memcmp(&v8,&v7,8))
            __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted"); // no-return
          v11 = &v11[2];
        } while (v11 != (unsigned long *)0x38480);
        dat_386b0 = 1;
        v12 = a0->field_0x0;
      }
      v10 = 0;
      v9 = 0x3b;
      do {
        while( true ) {
          v6 = v10 + v9 >> 1;
          v8 = v12;
          v7 = *(void **)(v6 * 0x10 + 0x380e0);
          v4 = memcmp(&v8,&v7,8);
          if (0 <= v4) break;
          v9 = v6;
          if (v6 <= v10) goto label_af40;
        }
        if (!v4) {
          v5 = *(unsigned int *)(v6 * 0x10 + 0x380e8);
          goto label_afd3;
        }
        v10 = v6 + 1;
      } while (v10 < v9);
label_af40:
      v1 = a0->field_0x8;
      error(0,0,dcgettext(NULL,"warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug",5),v1);
      v5 = 10;
    }
    else {
      v5 = a0->field_0x20;
    }
label_afd3:
    if (v5 <= v13)
      v5 = v13;
  }
  else {
    v5 = 8 - (unsigned int)a0->field_0x38;
  }
  a0->field_0x20 = v5;
}


// Function: sub_b4b0 @ 0xb4b0
unsigned long sub_b4b0(char *a0,unsigned char a1)
{
  char *v1;
  int v2; // eax
  unsigned long v3; // rbx
  
  if (!dat_386b8) {
    v2 = fnmatch(a0,".",(unsigned int)a1 << 4);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
  }
  v3 = 0;
  do {
    v1 = *(char **)(dat_386c0 + v3 * 8);
    if (!fnmatch(a0,v1,(unsigned int)a1 << 4))
      return 1;
    v3 += 1;
  } while (v3 < dat_386b8);
  return 0;
}


// Function: sub_b580 @ 0xb580
unsigned long sub_b580(void)
{
  return dat_386d0;
}


// Function: sub_b590 @ 0xb590
float sub_b590(struct_40 *a0)
{
  unsigned int v1;
  long v2;
  long v3;
  void *v4;
  float v5;
  float v6;
  float v7;
  
  if (!a0)
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates"); // no-return
  v2 = a0->field_0x118;
  if (v2)
    sub_b590(v2);
  v3 = a0->field_0x110;
  if (v3)
    sub_b590(v3);
  v1 = a0->field_0x10;
  if (v1 == 5)
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates"); // no-return
  if (v1 == 4)
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates"); // no-return
  if (v1 == 2) {
    if (a0->field_0x0 != sub_9350)
      __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates"); // no-return
    if (v3)
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates"); // no-return
    v6 = dat_29578 - *(float *)(v2 + 0x24);
    a0->field_0x24 = v6;
    return v6;
  }
  if (v1 <= 2) {
    if (v1) { // branch-flip
      if (v2)
        __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates"); // no-return
      if (v3)
        __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates"); // no-return
    }
    else {
      if (v2)
        __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates"); // no-return
      if (v3)
        __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates"); // no-return
    }
    return a0->field_0x24;
  }
  if (v1 != 3)
    __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates"); // no-return
  v4 = a0->field_0x0;
  v6 = dat_29578;
  if (v4 != sub_8990) { // branch-flip
    if (v4 == sub_8ca0) goto label_b678;
    if (v4 != sub_8940)
      __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates"); // no-return
    if (v2) // branch-flip
      v7 = *(float *)(v2 + 0x24);
    else {
      v7 = dat_29578;
    }
    v5 = dat_29578;
    if (v3)
      v5 = *(float *)(v3 + 0x24);
    v5 = v5 + v7;
  }
  else {
    if (v2) // branch-flip
      v5 = *(float *)(v2 + 0x24);
    else {
      v5 = dat_29578;
    }
    if (v3)
      v5 = v5 * *(float *)(v3 + 0x24);
  }
  if (v5 <= dat_29578) {
    v6 = 0.0;
    if (0.0 <= v5)
      v6 = v5;
  }
label_b678:
  a0->field_0x24 = v6;
  return v6;
}


// Function: sub_b8a0 @ 0xb8a0
void sub_b8a0(int *a0)
{
  long v1; // rax
  
  if (!*a0)
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred"); // no-return
  if (*a0 != 2) {
    v1 = sub_25e50(0x138);
    if (dat_386d8) // branch-flip
      *(long *)(dat_386c8 + 0x108) = v1;
    else {
      dat_386d8 = v1;
    }
    *(int **)(v1 + 0x130) = a0;
    *(unsigned short *)(v1 + 0x1a) = 0x101;
    *(unsigned long *)(v1 + 0x10) = 0;
    *(char **)(v1 + 0x30) = "ThisShouldBeSetToSomethingElse";
    *(char *)(v1 + 0x28) = dat_3862f;
    *(unsigned long *)(v1 + 0x20) = 0x3f8000000000000a;
    *(unsigned long *)(v1 + 0x128) = 0;
    *(unsigned long *)(v1 + 0x120) = 0;
    dat_386c8 = v1;
    return;
  }
  __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred"); // no-return
}


// Function: sub_b9b0 @ 0xb9b0
void sub_b9b0(int *a0,unsigned long a1)
{
  long v1;
  struct_44 *v2; // rax
  
  v1 = dat_386a0;
  if (!dat_386a0) {
    v1 = sub_14bc0("and");
    dat_386a0 = v1;
    if (!v1)
      __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op"); // no-return
  }
  if (dat_386c8) {
    if ((*(unsigned int *)(dat_386c8 + 0x10) & 0xfffffffb) != 1) { // branch-flip
      if (!*(unsigned int *)(dat_386c8 + 0x10)) {
        error(1,0,dcgettext(NULL,"oops -- invalid default insertion of and!",5));
        return;
      }
    }
    else {
      v2 = (struct_44 *)sub_b8a0(v1);
      v2->field_0x8 = "-a";
      v2->field_0x0 = sub_8990;
      v2->field_0x30 = 0;
      v2->field_0x38 = 0;
      v2->field_0x10 = 0x300000003;
      v2->field_0x18 = 0;
      v2->field_0x1c = 0;
    }
  }
  v1 = sub_b8a0(a0);
  *(unsigned long *)(v1 + 0x30) = a1;
  *(int **)(v1 + 0x130) = a0;
}


// Function: sub_bac0 @ 0xbac0
void sub_bac0(FILE *a0,void *a1,int a2)
{
  char v1;
  char *v10;
  int v11; // ecx
  short v2; // ax
  unsigned int v3; // eax
  long v4; // rax
  long v5;
  unsigned long v6; // rax
  char *v7;
  short v8;
  int *v9; // rdx
  
  if (!a1)
    return;
  do {
    v11 = 0;
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v11 += 1;
      } while (v11 != a2);
    }
    __fprintf_chk(a0,1,"pred=[");
    if (*(char **)((long)a1 + 0x30)) // branch-flip
      __fprintf_chk(a0,1,"%s %s",*(char **)((long)a1 + 8),*(char **)((long)a1 + 0x30));
    else {
      __fprintf_chk(a0,1,"%s",*(char **)((long)a1 + 8));
    }
    v10 = "no";
    v2 = (short)*(unsigned int *)((long)a1 + 0x14);
    if (v2) {
      v8 = 1;
      v11 = 0;
      v5 = 0x36100;
      while( true ) {
        v11 += 1;
        v4 = v5 + 0x10;
        if ((v2 == v8) || (v8 == -1)) break;
        v8 = *(short *)(v5 + 0x20);
        v5 = v4;
      }
      v10 = *(char **)((long)v11 * 0x10 + 0x36108);
    }
    v7 = "no";
    v2 = (short)*(unsigned int *)((long)a1 + 0x10);
    if (v2) {
      v8 = 1;
      v11 = 0;
      v5 = 0x36080;
      while( true ) {
        v11 += 1;
        v4 = v5 + 0x10;
        if ((v2 == v8) || (v8 == -1)) break;
        v8 = *(short *)(v5 + 0x20);
        v5 = v4;
      }
      v7 = *(char **)((long)v11 * 0x10 + 0x36088);
    }
    __fprintf_chk(a0,1,"] type=%s prec=%s",v7,v10);
    v10 = "";
    v9 = (int *)0x38020;
    if (!*(char *)((long)a1 + 0x18))
      v10 = "no ";
    v6 = 0;
    do {
      if (*(int *)((long)a1 + 0x20) == *v9) {
        v7 = *(char **)(v6 * 0x10 + 0x38028);
        goto label_bc5b;
      }
      v3 = (int)v6 + 1;
      v6 = (unsigned long)v3;
      v9 = &v9[4];
    } while (v3 != 0xb);
    v7 = "unknown";
label_bc5b:
    __fprintf_chk(a0,1," cost=%s est_success_rate=%#.4g %sside effects ",v7,(double)*(float *)((long)a1 + 0x24),v10);
    if (*(unsigned long *)((long)a1 + 0x18) & 0xffffff0000) {
      __fprintf_chk(a0,1,"Needs ");
      if (*(char *)((long)a1 + 0x1a)) { // branch-flip
        __fprintf_chk(a0,1,"stat");
        if (*(char *)((long)a1 + 0x1c)) { // branch-flip
          v10 = ",";
label_bdc5:
          __fprintf_chk(a0,1,"%sinode",v10);
          v1 = *(char *)((long)a1 + 0x1b);
        }
        else {
          v1 = *(char *)((long)a1 + 0x1b);
        }
        if (!v1) goto label_bc89;
        v10 = ",";
      }
      else {
        if (*(char *)((long)a1 + 0x1c)) {
          v10 = "";
          goto label_bdc5;
        }
        if (!*(char *)((long)a1 + 0x1b)) goto label_bc89;
        v10 = "";
      }
      __fprintf_chk(a0,1,"%stype",v10);
    }
label_bc89:
    v11 = 0;
    __fprintf_chk(a0,1,"\n");
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v11 += 1;
      } while (v11 != a2);
    }
    if (*(long *)((long)a1 + 0x110)) { // branch-flip
      __fprintf_chk(a0,1,"left:\n");
      sub_bac0(a0,*(unsigned long *)((long)a1 + 0x110),a2 + 1);
    }
    else {
      if (!*(long *)((long)a1 + 0x118)) {
        v10 = "no children.\n";
        __fprintf_chk(a0,1,v10); // return-dupe, tail-call
        return;
      }
      __fprintf_chk(a0,1,"no left.\n");
    }
    v11 = 0;
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v11 += 1;
      } while (v11 != a2);
    }
    if (!*(long *)((long)a1 + 0x118)) {
      v10 = "no right.\n";
      __fprintf_chk(a0,1,v10);
      return;
    }
    a2 += 1;
    __fprintf_chk(a0,1,"right:\n");
    a1 = *(void **)((long)a1 + 0x118);
    if (!a1)
      return;
  } while( true );
}


// Function: sub_bef0 @ 0xbef0
void sub_bef0(struct_12 *a0,long *a1) // return-dupe
{
  float v1;
  long v10;
  float v2;
  unsigned int v3;
  unsigned int v4;
  long v5;
  long v6;
  long v7;
  long v8;
  long v9;
  
  v10 = a0->field_0x0;
  if (!v10)
    return;
  if (dat_38660 & 8) { // branch-flip
    __fprintf_chk(stderr,1,"%s:\n","predlist before merge sort");
    sub_bac0(stderr,(void *)a0->field_0x0,2);
    v10 = a0->field_0x0;
    sub_b590(v10);
    if (!v10) {
      v8 = 0;
      v9 = 0;
      goto label_bfd3;
    }
  }
  else {
    sub_b590(v10);
  }
  v8 = 0;
  v9 = 0;
  do {
    v6 = *(long *)(v10 + 0x110);
    a0->field_0x0 = v6;
    *(unsigned long *)(v10 + 0x110) = 0;
    if (v8) {
      v3 = *(unsigned int *)(*(long *)(v10 + 0x118) + 0x20);
      v7 = v8;
      do {
        v4 = *(unsigned int *)(*(long *)(v7 + 0x118) + 0x20);
        if (v4 != v3) {
          if (v4 < v3) goto label_bfb1;
label_bf50:
          v5 = *(long *)(v7 + 0x110);
          *(long *)(v10 + 0x110) = v5;
          if (!v5)
            v9 = v10;
          *(long *)(v7 + 0x110) = v10;
          goto label_bf6f;
        }
        v1 = *(float *)(*(long *)(v7 + 0x118) + 0x24);
        v2 = *(float *)(*(long *)(v10 + 0x118) + 0x24);
        if (v1 == v2) goto label_bf50;
        if (*(int *)(v7 + 0x14) != 2) { // branch-flip
          if (v2 <= v1) goto label_bf50;
        }
        else if (v1 < v2) goto label_bf50;
label_bfb1:
        v7 = *(long *)(v7 + 0x110);
      } while (v7);
    }
    *(long *)(v10 + 0x110) = v8;
    v8 = v10;
    if (!v9)
      v9 = v10;
label_bf6f:
    v10 = v6;
  } while (v6);
label_bfd3:
  if (dat_38660 & 8) {
    __fprintf_chk(stderr,1,"%s:\n","predlist after merge sort");
    sub_bac0(stderr,v8,2);
  }
  sub_b590(v8);
  *(long *)(v9 + 0x110) = *a1;
  *a1 = v8;
  a0->field_0x8 = 0;
  a0->field_0x0 = 0;
}


// Function: sub_c110 @ 0xc110
unsigned long sub_c110(long *a0)
{
  int v1;
  long v10;
  char *v11;
  int *v12; // rdx
  int v13;
  long v14; // stack - 0x118
  long v15 [23];
  long v16; // stack - 0x108
  unsigned int v17;
  long v18;
  unsigned long v19;
  void *v2;
  long v20; // stack - 0x110
  long v21; // stack - 0x100
  char *v3;
  unsigned int v4;
  long *v5;
  long *v6; // rax
  unsigned long v7; // rax
  long v8;
  unsigned long v9; // rax
  
  v8 = *a0;
  v14 = 0;
  v20 = 0;
  v16 = 0;
  v21 = 0;
  if (!v8)
    return 0;
  v5 = v15;
  do {
    v5[1] = 0;
    v6 = &v5[2];
    *v5 = 0;
    v5 = v6;
  } while (v6 != &v15[0x16]);
  v10 = v8;
  v18 = *(long *)(v8 + 0x110);
  if (*(long *)(v8 + 0x110)) { // branch-flip
    for (; *(long *)(v18 + 0x110); v18 = *(long *)(v18 + 0x110)) {
      v10 = v18;
    }
    v17 = *(unsigned int *)(v10 + 0x14);
    v5 = (long *)(v10 + 0x110);
  }
  else {
    v17 = 3;
    v5 = a0;
    v18 = v8;
  }
  if (*(int *)(v18 + 0x10) != 3) { // branch-flip
    sub_ad30(v18,v17,v5);
    if (dat_38660 & 9) goto label_c448;
  }
  else {
    if (!(dat_38660 & 9)) goto label_c1e6;
label_c448:
    __fprintf_chk(stderr,1,"Normalized Eval Tree:\n");
    sub_bac0(stderr,(void *)*a0,0);
  }
  v8 = *a0;
  if (!v8) {
    v19 = 0;
    sub_c0b0(v15,0xb,&v16,&v14,a0); // return-dupe
    return v19;
  }
label_c1e6:
  v1 = *(int *)(v8 + 0x10);
  v13 = 0;
  if (v1 == 3)
    v13 = *(int *)(v8 + 0x14);
  v19 = 0;
  do {
    if ((v1 == 3) && (*(int *)(v8 + 0x14) != v13))
      v8 = sub_ad30(v8,v13,a0);
    v5 = *(long **)(v8 + 0x118);
    v4 = *(unsigned int *)&v5[2];
    v2 = (void *)*v5;
    if (v4 != 2) { // branch-flip
      if (2 < v4) {
        if (v4 != 3) {
          error(1,0,dcgettext(NULL,"oops -- invalid expression type!",5));
          return v9;
        }
        *(char *)&v5[3] = sub_c110(v8 + 0x118);
        v5 = *(long **)(v8 + 0x118);
        goto label_c3c0;
      }
      if (v13 == 1) goto label_c3c0;
      if ((char)v5[3]) goto label_c3c7;
      if ((((v2 != sub_9310 && v2 != sub_9640) && v2 != sub_9150) && (v2 != sub_91e0)) && ((((v2 == sub_9350 || v2 == sub_8990) || !dat_38688) || v2 == sub_8ca0 || ((v2 == sub_8940 || ((int)v5[4])))))) {
        if (v2 == sub_97e0) {
          *a0 = *(long *)(v8 + 0x110);
          *(long *)(v8 + 0x110) = v14;
          v14 = v8;
          if (!v20)
            v20 = v8;
          goto label_c3ec;
        }
        if ((2 <= dat_38688) && (((v4 = *(unsigned int *)&v5[4], v4 - 1 < 2 && (!*(char *)((long)v5 + 0x1a))) || (dat_38688 != 2)))) {
          if (dat_38660 & 8) {
            __fprintf_chk(stderr,1,"-O%d: categorising predicate ",(unsigned int)dat_38688);
            v11 = *(char **)(*(long *)(v8 + 0x118) + 0x30);
            v3 = *(char **)(*(long *)(v8 + 0x118) + 8);
            if (v11) // branch-flip
              __fprintf_chk(stderr,1,"%s %s",v3,v11);
            else {
              __fprintf_chk(stderr,1,"%s",v3);
            }
            v12 = (int *)0x38020;
            v7 = 0;
            do {
              if (*(int *)(*(long *)(v8 + 0x118) + 0x20) == *v12) {
                v11 = *(char **)(v7 * 0x10 + 0x38028);
                goto label_c617;
              }
              v4 = (int)v7 + 1;
              v7 = (unsigned long)v4;
              v12 = &v12[4];
            } while (v4 != 0xb);
            v11 = "unknown";
label_c617:
            __fprintf_chk(stderr,1," by cost (%s)\n",v11);
            v4 = *(unsigned int *)(*(long *)(v8 + 0x118) + 0x20);
          }
          v7 = (unsigned long)v4;
          v5 = &v15[v7 * 2];
          *a0 = *(long *)(v8 + 0x110);
          *(long *)(v8 + 0x110) = *v5;
          *v5 = v8;
          if (!v15[v7 * 2 + 1])
            v15[v7 * 2 + 1] = v8;
          goto label_c3ec;
        }
        goto label_c3e5;
      }
      if (dat_38660 & 8) {
        __fprintf_chk(stderr,1,"-O%d: promoting cheap predicate ",(unsigned int)dat_38688);
        v11 = *(char **)(*(long *)(v8 + 0x118) + 0x30);
        v3 = *(char **)(*(long *)(v8 + 0x118) + 8);
        if (v11) // branch-flip
          __fprintf_chk(stderr,1,"%s %s",v3,v11);
        else {
          __fprintf_chk(stderr,1,"%s",v3);
        }
        __fprintf_chk(stderr,1," into name_list\n");
      }
      *a0 = *(long *)(v8 + 0x110);
      *(long *)(v8 + 0x110) = v16;
      v16 = v8;
      if (!v21)
        v21 = v8;
    }
    else {
      *(char *)&v5[3] = sub_c110(&v5[0x23]);
      v5 = *(long **)(v8 + 0x118);
label_c3c0:
      if ((char)v5[3]) {
label_c3c7:
        v19 = 1;
        sub_c0b0(v15,0xb,&v16,&v14,a0);
      }
label_c3e5:
      a0 = (long *)(v8 + 0x110);
    }
label_c3ec:
    v8 = *a0;
    if (!v8) {
      sub_c0b0(v15,0xb,&v16,&v14,a0);
      return v19;
    }
    v1 = *(int *)(v8 + 0x10);
  } while( true );
}


// Function: sub_c690 @ 0xc690
long sub_c690(struct_43 *a0)
{
  float v1;
  char *v10;
  unsigned int v11;
  void *v12; // rdx
  unsigned int v13;
  void *v14;
  unsigned long v15;
  unsigned int v16;
  long v17;
  long v18;
  bool v19; // zf
  float v2;
  int v3;
  void *v4;
  void *v5;
  void *v6;
  char v7; // al
  unsigned int v8; // eax
  long v9; // rax
  
  if (!a0) {
    v9 = 0;
    return v9;
  }
  do {
    v11 = (unsigned int)dat_38660 & 8;
    v10 = "Not a binary operation";
    if (a0->field_0x10 == 3) {
      v9 = a0->field_0x110;
      v15 = dat_38660;
      do {
        v11 = (unsigned int)v15;
        if ((!v9) || (v18 = a0->field_0x118, !v18)) {
          v10 = "Doesn\'t have two arms";
          v11 &= 8;
          goto label_c6d8;
        }
        v17 = *(long *)(v9 + 0x118);
        if (!v17) {
          v10 = "Left arm has no child on RHS";
          v11 &= 8;
          goto label_c6d8;
        }
        if (*(char *)(v17 + 0x18)) {
label_c774:
          v11 &= 8;
          v10 = "Left subtree has side-effects";
          goto label_c6d8;
        }
        v7 = sub_b430(*(void **)(v17 + 0x110));
        v11 = (unsigned int)v15;
        if (v7) goto label_c774;
        v14 = *(void **)(v17 + 0x118);
        v7 = sub_b430(v14);
        v11 = (unsigned int)v15;
        if (v7) goto label_c774;
        if (*(char *)(v18 + 0x18)) {
label_c7ed:
          v11 &= 8;
          v10 = "Right subtree has side-effects";
          goto label_c6d8;
        }
        v4 = *(void **)(v18 + 0x110);
        v7 = sub_b430(v4);
        v11 = (unsigned int)v15;
        if (v7) goto label_c7ed;
        v5 = *(void **)(v18 + 0x118);
        v7 = sub_b430(v5);
        v11 = (unsigned int)v15;
        if (v7) goto label_c7ed;
        v16 = 0;
        if (v12) {
          v8 = sub_b0c0(v12);
          v16 = v8;
        }
        v8 = 0;
        if (v14)
          v8 = sub_b0c0(v14);
        if (v16 < *(unsigned int *)(v17 + 0x20))
          v16 = *(unsigned int *)(v17 + 0x20);
        if (v16 < v8)
          v16 = v8;
        v13 = 0;
        if (v4) {
          v8 = sub_b0c0(v4);
          v13 = v8;
        }
        v8 = 0;
        if (v5)
          v8 = sub_b0c0(v5);
        if (v13 < *(unsigned int *)(v18 + 0x20))
          v13 = *(unsigned int *)(v18 + 0x20);
        if (v13 < v8)
          v13 = v8;
        v11 = (unsigned int)v15 & 8;
        if ((int)v16 < (int)v13) {
          v10 = "efficient as-is";
          goto label_c6d8;
        }
        if (v16 == v13) {
          v1 = *(float *)(v17 + 0x24);
          v2 = *(float *)(v18 + 0x24);
          if (v15 & 8) { // branch-flip
            __fprintf_chk(stderr,1,"Success rates: l=%f, r=%f\n");
            v6 = a0->field_0x0;
            v11 = (unsigned int)dat_38660 & 8;
            v15 = dat_38660;
          }
          else {
            v6 = a0->field_0x0;
          }
          if (v6 != sub_8940) { // branch-flip
            if (v6 != sub_8990) {
              v10 = "Not \'AND\' or \'OR\'";
              goto label_c6d8;
            }
            if (v2 <= v1) {
              v10 = "Operation is AND; right success rate <= left";
              goto label_c6d8;
            }
          }
          else if (v1 <= v2) {
            v10 = "Operation is OR; right success rate >= left";
            goto label_c6d8;
          }
        }
        v19 = v11 != 0;
        v11 = 0;
        if (v19) {
          __fprintf_chk(stderr,1,"Performing arm swap on:\n");
          sub_bac0(stderr,a0,0);
          v11 = (unsigned int)dat_38660 & 8;
          v15 = dat_38660;
        }
        v9 = a0->field_0x110;
        v3 = a0->field_0x10;
        v18 = *(long *)(v9 + 0x118);
        *(long *)(v9 + 0x118) = a0->field_0x118;
        a0->field_0x118 = v18;
      } while (v3 == 3);
      v10 = "Not a binary operation";
    }
label_c6d8:
    if (v11) {
      __fprintf_chk(stderr,1,"Not an arm swap candidate (%s):\n",v10);
      sub_bac0(stderr,a0,0);
    }
    v7 = sub_c690(a0->field_0x110);
    if ((!v7) && (v7 = sub_c690(a0->field_0x118), !v7)) {
      v9 = 0;
      return v9;
    }
  } while( true );
}


// Function: sub_c9f0 @ 0xc9f0
int * sub_c9f0(int a0,long a1,int a2)
{
  long *v1;
  long *v10; // stack - 0x48
  unsigned long v11;
  int v12;
  bool v13; // zf
  long v14; // stack - 0x70
  char *v2;
  char *v3;
  int *v4;
  int *v5;
  char v6; // al
  int *v7; // rax
  int v8 [4]; // stack - 0x5c
  int v9; // stack - 0x4c
  
  v7 = (long)(long)a2;
  dat_386d8 = NULL;
  dat_386c0 = a1 + (long)v7 * 8;
  v8[0] = a0;
  v9 = a2;
  if (a2 < a0) {
    while (v6 = sub_e120(*(char **)(a1 + (long)v7 * 8),1), !v6) {
      dat_386b8 += 1;
      v9 += 1;
      if (v8[0] <= v9) break;
      v7 = (long)(long)v9;
    }
  }
  v7 = (int *)sub_14bc0("(");
  v4 = v7;
  v7 = (long)sub_14bc0(")");
  v14 = (long)v7;
  v7 = (int *)sub_14bc0("print");
  v5 = v7;
  if (!v4)
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree"); // no-return
  if (!v14)
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree"); // no-return
  if (!v7)
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree"); // no-return
  sub_12c60(v4,a1,v8);
  *(char **)(dat_386c8 + 8) = "(";
  *(char *)((long)dat_386d8 + 0x29) = 1;
  sub_14ba0(a1,v8[0]);
  sub_9e60(dat_386c8);
  if (v9 < v8[0]) {
    do {
      v7 = (long)(long)v9;
      dat_38618 = 0;
      v6 = sub_e120(*(char **)(a1 + (long)v7 * 8),0);
      v13 = v6 == '\0';
      v7 = (long)(long)v9;
      v2 = *(char **)(a1 + (long)v7 * 8);
      if (v13) {
        v7 = (char *)dcgettext(NULL,"paths must precede expression: `%s\'",5);
        error(0,0,(char *)v7,v2);
        v7 = (long)(long)v9;
        v2 = *(char **)(a1 + (long)v7 * 8);
        if (!access(v2,0)) {
          v2 = *(char **)(dat_386c8 + 8);
          v7 = (char *)dcgettext(NULL,"possible unquoted pattern after predicate `%s\'?",5);
          error(0,0,(char *)v7,v2);
        }
        exit(1); // no-return
      }
      v7 = (int *)sub_14bc0(v2);
      v4 = v7;
      if (!v7) {
        v7 = (char *)dcgettext(NULL,"unknown predicate `%s\'",5);
        error(1,0,(char *)v7,v2);
        return v7;
      }
      v12 = v9;
      if (*v7 != 4) {
        v12 = v9 + 1;
        v9 = v12;
      }
      v6 = (**(void **)&v7[4])(v7,a1,&v9);
      if (!v6) {
        v3 = *(char **)(a1 + (long)v9 * 8);
        if (!v3) {
          v7 = (char *)dcgettext(NULL,"missing argument to `%s\'",5);
          error(1,0,(char *)v7,v2);
          return v7;
        }
        if ((*v4 == 4) && (v9 == v12)) {
          v7 = (char *)dcgettext(NULL,"invalid predicate `%s\'",5);
          error(1,0,(char *)v7,v2);
          return v7;
        }
        v7 = (char *)dcgettext(NULL,"invalid argument `%s\' to `%s\'",5);
        error(1,0,(char *)v7,v3,v2);
        return v7;
      }
      v7 = NULL;
      *(char **)(dat_386c8 + 8) = v2;
      if (v9 != v12)
        v7 = *(int **)(a1 + (long)v12 * 8);
      *(int **)(dat_386c8 + 0x30) = v7;
      sub_9e60(dat_386c8);
      sub_9e60(dat_386d8);
    } while (v9 < v8[0]);
  }
  sub_14bb0(a1,v8[0],dat_386c8,dat_386d8);
  v1 = dat_386d8;
  if ((void *)dat_386d8[0x21]) { // branch-flip
    v6 = sub_e0e0((void *)dat_386d8[0x21]);
    if (v6) { // branch-flip
      sub_12a60(v14,a1,v8);
      *(char **)(dat_386c8 + 8) = ")";
      *(char *)(dat_386c8 + 0x29) = 1;
      sub_9e60(dat_386c8);
      sub_11e20(v5,a1,v8);
      *(char **)(dat_386c8 + 8) = "-print";
      *(char *)(dat_386c8 + 0x29) = 1;
      sub_9e60(dat_386c8);
      sub_9e60(dat_386d8);
    }
    else {
      v1 = &dat_386d8[0x21];
      v10 = dat_386d8;
      dat_386d8 = (long *)*v1;
      sub_9e60((long *)*v1);
      free(v10);
    }
  }
  else {
    v10 = dat_386d8;
    dat_386c8 = 0;
    dat_386d8 = NULL;
    free(v1);
    sub_11e20(v5,a1,v8);
    *(char **)(dat_386c8 + 8) = "-print";
    sub_9e60(dat_386c8);
    sub_9e60(dat_386d8);
  }
  if (dat_38660 & 9) {
    __fprintf_chk(stderr,1,"Predicate List:\n");
    sub_9cd0(stderr,dat_386d8);
  }
  sub_14a70(dat_386d8);
  sub_9e60(dat_386d8);
  v10 = dat_386d8;
  v7 = (unsigned long)sub_a8c0(&v10,0,NULL);
  dat_386d0 = v7;
  sub_b590(v7);
  if (!v10) {
    if (dat_38660 & 9) {
      __fprintf_chk(stderr,1,"Eval Tree:\n");
      sub_bac0(stderr,dat_386d0,0);
      v7 = (int *)dat_386d0;
    }
    sub_ae30(v7);
    sub_c110((long *)0x386d0);
    v11 = dat_386d0;
    sub_a840(dat_386d0,1);
    sub_c690(v11);
    v11 = dat_386d0;
    sub_a840(dat_386d0,1);
    if (dat_38660 & 9) {
      __fprintf_chk(stderr,1,"Optimized Eval Tree:\n");
      sub_bac0(stderr,dat_386d0,0);
      __fprintf_chk(stderr,1,"Optimized command line:\n");
      sub_9de0(stderr,dat_386d0);
      __fprintf_chk(stderr,1,"\n");
      v11 = dat_386d0;
    }
    v7 = (int *)v11;
    return v7;
  }
  if ((void *)*v10 == sub_8be0) {
    v7 = (char *)dcgettext(NULL,"you have too many \')\'",5);
    error(1,0,(char *)v7);
    return v7;
  }
  v2 = (char *)v10[1];
  if (v2) {
    v7 = (char *)dcgettext(NULL,"unexpected extra predicate \'%s\'",5);
    error(1,0,(char *)v7,v2);
    return v7;
  }
  v7 = (char *)dcgettext(NULL,"unexpected extra predicate",5);
  error(1,0,(char *)v7);
  return v7;
}


// Function: sub_d130 @ 0xd130
void sub_d130(long *a0) // return-dupe
{
  if (!a0)
    return;
  do {
    while( true ) {
      if (!dat_38608)
        __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs"); // no-return
      sub_d130(a0[0x22]);
      if (((((void *)*a0 == sub_8fc0) || ((void *)*a0 == sub_95d0)) && ((char)a0[7])) && ((int)a0[0x1a])) break;
label_d180:
      a0 = (long *)a0[0x23];
      if (!a0)
        return;
    }
    sub_159f0((struct_9 *)a0[0x1f],sub_d220,&a0[7]);
    if ((struct_10 *)a0[0x1f] == dat_385c0) goto label_d180;
    sub_242c0((struct_10 *)a0[0x1f]);
    free((void *)a0[0x1f]);
    a0[0x1f] = 0;
    a0 = (long *)a0[0x23];
    if (!a0)
      return;
  } while( true );
}


// Function: sub_d220 @ 0xd220
unsigned long sub_d220(long a0)
{
  sub_15430((void *)(a0 + 8),(struct_5 *)(a0 + 0x60));
  return 0;
}


// Function: sub_d240 @ 0xd240
void sub_d240(long *a0)
{
  if (!a0)
    return;
  do {
    sub_d240(a0[0x22]);
    if ((((void *)*a0 == sub_8fb0) && ((char)a0[7])) && ((int)a0[0x1a]))
      sub_15430(&a0[8],(struct_5 *)&a0[0x13]);
    a0 = (long *)a0[0x23];
  } while (a0);
}


// Function: sub_d520 @ 0xd520
void sub_d520(int *a0)
{
  long v1;
  long v2;
  struct_45 *v3; // rax
  
  v1 = *(long *)&a0[6];
  if (v1) {
    v3 = (struct_45 *)sub_b9b0(a0,0);
    v2 = *(long *)&a0[2];
    v3->field_0x0 = v1;
    v3->field_0x8 = v2;
    v3->field_0x38 = 0;
    v3->field_0x10 = 1;
    return;
  }
  sub_d2b0(); // no-return
}


// Function: sub_d570 @ 0xd570
void sub_d570(int a0)
{
  FILE *v1;
  char *v2;
  long v3;
  unsigned long *v4; // rbx
  
  v2 = dat_38ed0;
  if (a0) {
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
    exit(a0); // no-return
  }
  v4 = (unsigned long *)0x36180;
  __fprintf_chk(stdout,1,dcgettext(NULL,"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n",5),v2);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nOther common options:\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"      --help                   display this help and exit\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"      --version                output version information and exit\n\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"Valid arguments for -D:\n",5),v1);
  v3 = 0;
  do {
    if (v3) { // branch-flip
      __fprintf_chk(stdout,1,"%s%s",", ",*v4);
      v1 = stdout;
      if (v3 + 1 == 9) {
        fputs(dcgettext(NULL,"\nUse \'-D help\' for a description of the options, or see find(1)\n\n",5),v1);
        sub_175f0(stdout,dat_38ed0);
        exit(0); // no-return
      }
    }
    else {
      __fprintf_chk(stdout,1,"%s%s","",*v4);
    }
    v4 = &v4[3];
    v3 += 1;
  } while( true );
}


// Function: sub_d820 @ 0xd820
unsigned long sub_d820(char *a0,char *a1,struct_1 *a2) // return-dupe x3
{
  int v1;
  char *v2; // rax
  int *v3; // rax
  
  if (dat_385e4) {
    dat_385e4 = 0x101;
    dat_385e8 = a2->field_0x18;
    return 0;
  }
  v1 = (*dat_38670)(a1,a2);
  if (!v1) {
    if (a2->field_0x18) {
      dat_385e8 = a2->field_0x18;
      dat_385e4 = 0x101;
      return 0;
    }
    v2 = (char *)sub_239f0(0,dat_3868c,a1);
    error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v2);
    dat_385e4 = 0x101;
    dat_385e8 = a2->field_0x18;
    dat_38604 = 1;
    return 0;
  }
  v3 = __errno_location();
  v1 = *v3;
  if ((dat_3862e) && (v1 == 2))
    return 0xffffffff;
  if (dat_38618) {
    dat_38618 = 1;
    return 0xffffffff;
  }
  error(0,v1,"%s",(char *)sub_239f0(0,dat_3868c,a0));
  dat_38604 = 1;
  dat_38618 = 1;
  return 0xffffffff;
}


// Function: sub_d920 @ 0xd920
unsigned long sub_d920(void) // return-dupe
{
  int v1; // eax
  char v2 [130];
  char v3 [262];
  float v4; // xmm1_da
  
  if (uname(v2))
    return 1;
  v4 = (float)(double)strtod(v3,0);
  v1 = strcmp("Linux",v2);
  if (v1) {
    v1 = strcmp("FreeBSD",v2);
    if (v1)
      return 1;
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),(float)s_2a2d3._1_4_ <= v4);
  }
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),dat_2a2d0 <= v4);
}


// Function: sub_da20 @ 0xda20
void sub_da20(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  char *v4;
  
  dat_385c0 = (int *)sub_25a50(0x10);
  v1 = sub_24230(dat_385c0);
  if (!v1)
    return;
  v3 = "";
  v4 = "";
  if (*dat_385c0 < 0) {
    v3 = *(char **)&dat_385c0[2];
    v4 = ": ";
    if (!v3) {
      v3 = "";
      v4 = v3;
    }
  }
  v2 = dcgettext(NULL,"Failed to save initial working directory%s%s",5);
  error(1,*__errno_location(),v2,v4,v3);
}


// Function: sub_dab0 @ 0xdab0
void sub_dab0(void)
{
  void *v1;
  int v2;
  long *v3;
  char *v4; // rax
  char *v5;
  long *v6;
  char *v7;
  
  v3 = (long *)sub_b580();
  if (v3) { // branch-flip
    v6 = v3;
    do {
      if ((void *)v6[0x22])
        sub_d2e0((void *)v6[0x22],sub_d240);
      sub_d240(v6);
      v6 = (long *)v6[0x23];
    } while (v6);
    if (dat_38608) {
      sub_d130((long *)sub_b580());
      dat_38608 = '\0';
    }
    sub_eb20(dat_38610);
    do {
      if ((void *)v3[0x22])
        sub_d2e0((void *)v3[0x22],sub_d0f0);
      v1 = (void *)*v3;
      if (((v1 == sub_ff90 || v1 == sub_9020) || v1 == sub_8fe0) || (v1 == sub_9060))
        v3[8] = 0;
      v3 = (long *)v3[0x23];
    } while (v3);
  }
  else {
    sub_eb20(dat_38610);
  }
  v2 = sub_24290(dat_385c0);
  if (!v2) {
    sub_242c0(dat_385c0);
    free(dat_385c0);
    dat_385c0 = NULL;
    if (sub_16220()) {
      sub_16240();
      sub_161f0();
    }
    v2 = sub_180c0(stdout);
    if (v2 != -1)
      return;
    v2 = *__errno_location();
    error(0,v2,"%s",(char *)sub_239f0(0,dat_3868c,"standard output"));
    dat_38604 = 1;
    return;
  }
  v5 = "";
  v7 = "";
  if (*dat_385c0 < 0) {
    v5 = *(char **)&dat_385c0[2];
    v7 = ": ";
    if (!v5) {
      v5 = "";
      v7 = v5;
    }
  }
  v4 = dcgettext(NULL,"Failed to restore initial working directory%s%s",5);
  error(0,*__errno_location(),v4,v7,v5);
  _exit(1); // no-return
}


// Function: sub_dcd0 @ 0xdcd0
unsigned long sub_dcd0(char *a0,stat *a1)
{
  unsigned int v1; // eax
  int v2;
  
  if ((dat_385f8 <= -1) && (dat_385f8 != -100))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat"); // no-return
  if (dat_385e0) {
    v2 = fstatat(dat_385f8,a0,a1,0x100); // tail-call
    return v2;
  }
  v1 = fstatat(dat_385f8,a0,a1,0);
  if (v1) {
    v2 = *__errno_location();
    if ((v2 == 2) || (v2 == 0x14)) {
      if (dat_38660 & 2)
        __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
      v2 = fstatat(dat_385f8,a0,a1,0x100); // tail-call
      return v2;
    }
  }
  return (unsigned long)v1;
}


// Function: sub_dec0 @ 0xdec0
void sub_dec0(char *a0,stat *a1)
{
  dat_386e0 += 1;
  __fprintf_chk(stderr,1,"debug_stat (%s)\n",a0);
  if (dat_38668 == 1) {
    sub_ddb0(a0,a1); // tail-call
    return;
  }
  if (dat_38668 == 2) {
    sub_dcd0(a0,a1); // tail-call
    return;
  }
  if (dat_38668)
    __assert_fail("0","util.c",0x2a5,"debug_stat"); // no-return
  if ((dat_385f8 <= -1) && (dat_385f8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  fstatat(dat_385f8,a0,a1,0x100); // tail-call
}


// Function: sub_dfe0 @ 0xdfe0
unsigned int sub_dfe0(unsigned int *a0,char *a1,char *a2,struct_1 *a3,unsigned int a4) // return-dupe
{
  unsigned int v1;
  
  v1 = *a0;
  if (v1) {
    if (((v1 & 0xf000) == 0xa000) && ((dat_38668 == 1 || ((dat_38668 == 2 && (!dat_385e0)))))) {
      if (sub_d820(a1,a2,a3))
        return 0;
      dat_385e8 = a3->field_0x18;
      *a0 = dat_385e8;
      dat_385e4 = CONCAT11(1,dat_385e4);
      return 1;
    }
    dat_385e4 = CONCAT11(1,dat_385e4);
    a3->field_0x18 = v1;
    dat_385e8 = v1;
    return 1;
  }
  if ((char)a4) {
    dat_385e8 = 0;
    dat_385e4 = 0;
    return a4;
  }
  if (sub_d820(a1,a2,a3))
    return 0;
  dat_385e8 = a3->field_0x18;
  *a0 = dat_385e8;
  dat_385e4 = CONCAT11(1,dat_385e4);
  return 1;
}


// Function: sub_e0e0 @ 0xe0e0
unsigned long sub_e0e0(void *a0)
{
  while( true ) {
    if (!a0)
      return 1;
    if (*(char *)((long)a0 + 0x19)) break;
    a0 = *(void **)((long)a0 + 0x108);
  }
  return 0;
}


// Function: sub_e120 @ 0xe120
unsigned long sub_e120(char *a0,unsigned int a1) // return-dupe
{
  undefined7 v1; // rax
  
  v1 = (undefined7)((unsigned long)((long)*(int *)((unsigned long)(unsigned char)(*a0 - 0x21) * 4 + 0x2a220) + 0x2a220) >> 8);
  switch(*a0) {
    case 0x21:
    case 0x28:
      return CONCAT71(v1,a0[1] == '\0');
    default:
      return 0;
    case 0x29:
    case 0x2c:
      goto label_e150;
    case 0x2d:
      return CONCAT71(v1,a0[1] != '\0');
    
  }
label_e150:
  if (!a0[1])
    return (unsigned long)(a1 ^ 1);
  return 0;
}


// Function: sub_e180 @ 0xe180
unsigned long sub_e180(int a0,long a1)
{
  unsigned char v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  char v14 [2]; // stack - 0x42
  char *v15; // stack - 0x50
  char *v16;
  char *v17;
  unsigned int v18; // r12d
  unsigned long *v19; // r12
  int v2;
  FILE *v3;
  long v4; // rax
  char *v5;
  int *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v18 = 1;
  if (2 <= a0) {
    do {
      v4 = (long)(int)v18;
      v17 = *(char **)(a1 + v4 * 8);
      if (strcmp("-H",v17)) { // branch-flip
        if (strcmp("-L",v17)) { // branch-flip
          if (strcmp("-P",v17)) { // branch-flip
            if (!strcmp("--",v17)) {
              v18 += 1;
              break;
            }
            if (strcmp("-D",v17)) { // branch-flip
              if (strncmp("-O",v17,2)) break;
              v5 = &v17[2];
              v1 = v17[2];
              if (!v1) {
                error(1,0,dcgettext(NULL,"The -O option must be immediately followed by a decimal integer",5));
                return v13;
              }
              if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v1 * 2) & 8)) {
                error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
                return v12;
              }
              v6 = __errno_location();
              v2 = *v6;
              *v6 = 0;
              v7 = strtoul(v5,&v15,10);
              if (v7) { // branch-flip
                if (*v15) {
                  error(1,0,dcgettext(NULL,"Invalid optimisation level %s",5),v5); // return-dupe
                  return v10;
                }
                if (v7 == 0xffffffffffffffff) {
                  if (*v6) {
                    v17 = dcgettext(NULL,"Invalid optimisation level %s",5);
                    error(1,*v6,v17,v5);
                    return v9;
                  }
                  error(1,0,dcgettext(NULL,"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",5),v7); // return-dupe
                  return v8;
                }
                if (0x10000 <= v7) {
                  error(1,0,dcgettext(NULL,"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",5),v7);
                  return v8;
                }
              }
              else {
                if (v5 == v15) {
                  error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
                  return v11;
                }
                if (*v15) {
                  error(1,0,dcgettext(NULL,"Invalid optimisation level %s",5),v5);
                  return v10;
                }
              }
              dat_38688 = (unsigned short)v7;
              *v6 = v2;
            }
            else {
              v18 += 1;
              if (a0 <= (int)v18) {
                v17 = "Missing argument after the -D option.";
label_e4bc:
                error(0,0,dcgettext(NULL,v17,5));
                sub_d570(1); // no-return
              }
              v15 = NULL;
              v14[0] = ',';
              v14[1] = '\0';
              v17 = *(char **)(a1 + 8 + v4 * 8);
              v5 = strtok_r(v17,v14,&v15);
              if (!v5) {
                v17 = "Empty argument to the -D option.";
                goto label_e4bc;
              }
              do {
                v16 = "exec";
                v4 = 0;
                v19 = (unsigned long *)0x36198;
                while (strcmp(v16,v5)) {
                  v4 += 1;
                  if (v4 == 9) {
                    v5 = (char *)sub_239f0(0,dat_3868c,v17);
                    error(0,0,dcgettext(NULL,"Ignoring unrecognised debug flag %s",5),v5);
                    goto label_e31a;
                  }
                  v16 = (char *)*v19;
                  v19 = &v19[3];
                }
                dat_38660 |= (long)*(int *)(v4 * 0x18 + 0x36188);
label_e31a:
                v5 = strtok_r(NULL,v14,&v15);
                v3 = stdout;
              } while (v5);
              if (dat_38660 & 0x10) {
                fputs(dcgettext(NULL,"Valid arguments for -D:\n",5),v3);
                v5 = "exec";
                v17 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
                for (v19 = (unsigned long *)0x36198; __fprintf_chk(stdout,1,"%-10s %s\n",v5,v17), v19 != (unsigned long *)0x36258; v19 = &v19[3]) {
                  v17 = (char *)v19[2];
                  v5 = (char *)*v19;
                }
                exit(0); // no-return
              }
            }
          }
          else {
            sub_14af0(0);
          }
        }
        else {
          sub_14af0(1);
        }
      }
      else {
        sub_14af0(2);
      }
      v18 += 1;
    } while ((int)v18 < a0);
  }
  return (unsigned long)v18;
}


// Function: sub_e630 @ 0xe630
void sub_e630(struct_49 *a0)
{
  char v1; // al
  int v2; // eax
  long v3; // rdx
  timeval v4; // stack - 0x28
  
  a0->field_0x11 = getenv("POSIXLY_CORRECT") != NULL;
  v1 = sub_d920();
  a0->field_0x5c = 0;
  a0->field_0x58 = v1;
  v2 = isatty(0);
  a0->field_0xf = 0;
  a0->field_0x10 = v2 != 0;
  if (a0->field_0x11)
    a0->field_0x10 = 0;
  *(unsigned long *)&a0->field_0x2[2] = 0xffffffffffffffff;
  a0->field_0x0 = 1;
  if (gettimeofday(&v4,NULL)) { // branch-flip
    v4._0_8_ = time(0);
    if (v4._0_8_ == -1)
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now"); // no-return
    v3 = 0;
  }
  else {
    v3 = v4._8_8_ * 1000;
  }
  a0->field_0x18 = v4._0_8_;
  a0->field_0x28 = v4._0_8_ + -0x15180;
  *(unsigned short *)a0->field_0xd = 0;
  a0->field_0x20 = v3;
  a0->field_0x30 = v3;
  a0->field_0x38 = 0;
  a0->field_0x3c = (-(unsigned int)(a0->field_0x11 == '\0') & 0x200) + 0x200;
  a0->field_0x40 = 0;
  a0->field_0x68 = 2;
  if (getenv("FIND_BLOCK_SIZE")) {
    error(1,0,dcgettext(NULL,"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable",5));
    return;
  }
  a0->field_0xc = 0;
  sub_14af0(0);
  a0->field_0x6c = 8;
  a0->field_0x70 = 0;
  a0->field_0x78 = 0;
}


// Function: sub_e7b0 @ 0xe7b0
unsigned long sub_e7b0(char *a0,void *a1,struct_2 *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  a2->field_0x120 = a2->field_0x120 + 1;
  if ((((!(a2->field_0x18 & 0xffffff0000)) || ((((!*(char *)((long)&a2->field_0x18 + 2) || (dat_385e4)) && ((!*(char *)((long)&a2->field_0x18 + 3) || (dat_385e5)))) && ((!*(char *)((long)&a2->field_0x18 + 4) || (((*(long *)((long)a1 + 8) && (dat_385e5)) && ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0x4000)))))))) || (v1 = sub_d820(a0,dat_385f0,a1), !v1)) && (v2 = (*a2->field_0x0)(a0,a1,a2), (char)v2)) {
    a2->field_0x128 = a2->field_0x128 + 1;
    return v2;
  }
  return 0;
}


// Function: sub_e880 @ 0xe880
unsigned int sub_e880(void *a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == sub_95d0) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == sub_8fc0);
}


// Function: sub_e920 @ 0xe920
void sub_e920(int a0,char *a1)
{
  if (!dat_38618) {
    error(1,a0,"%s",(char *)sub_239f0(0,dat_3868c,a1));
    return;
  }
  dat_38618 = 1;
  abort(); // no-return
}


// Function: sub_e9b0 @ 0xe9b0
void sub_e9b0(int a0,char *a1)
{
  dat_38618 = 0;
  error(1,a0,"%s",(char *)sub_239f0(0,dat_3868c,a1));
}


// Function: sub_ea00 @ 0xea00
unsigned long sub_ea00(struct_12 *a0,struct_12 *a1)
{
  if (a0->field_0x8 != a1->field_0x8)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x0 >> 8),a0->field_0x0 == a1->field_0x0);
}


// Function: sub_ea30 @ 0xea30
unsigned long sub_ea30(struct_50 *a0,unsigned long a1)
{
  return (a0->field_0x0 ^ a0->field_0x8) % a1;
}


// Function: sub_ea90 @ 0xea90
unsigned long * sub_ea90(char *a0)
{
  unsigned long *v1; // rax
  char *v2; // rax
  long v3; // rax
  unsigned long *v4;
  
  v1 = malloc(0x10);
  v4 = v1;
  if (v1) {
    v2 = strdup(a0);
    *v1 = v2;
    if (v2) { // branch-flip
      v3 = sub_1b2f0(0xb,NULL,sub_ea30,sub_ea00,sub_ea50);
      v1[1] = v3;
      if (!v3) {
        free((void *)*v1);
        v4 = NULL;
        free(v1);
      }
    }
    else {
      v4 = NULL;
      free(v1);
    }
  }
  return v4;
}


// Function: sub_eb20 @ 0xeb20
void sub_eb20(struct_35 *a0)
{
  free(a0->field_0x0);
  sub_1b4b0(a0->field_0x8); // tail-call
}


// Function: sub_eb40 @ 0xeb40
unsigned long sub_eb40(struct_46 *a0,char *a1)
{
  struct_24 *v1;
  int v2;
  unsigned long *v3; // rax
  char *v4; // rax
  FILE *v5; // rax
  long v6; // rax
  int *v7; // rax
  stat v8; // stack - 0xc8
  unsigned long v9;
  
  v3 = malloc(0x20);
  if (v3) { // branch-flip
    v4 = strdup(a1);
    v3[2] = v4;
    if (v4) { // branch-flip
      v5 = (FILE *)sub_182a0(a1,a0->field_0x0);
      v3[3] = v5;
      if (v5) { // branch-flip
        v2 = fileno(v5);
        if (v2 < 0)
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen"); // no-return
        sub_17990(v2,1);
        if (0 <= fstat(v2,&v8)) { // branch-flip
          v1 = a0->field_0x8;
          *v3 = v8._0_8_;
          v3[1] = v8._8_8_;
          v6 = sub_1b060(v1,v3);
          if (v6) { // branch-flip
            sub_ea50(v3);
            v9 = *(unsigned long *)(v6 + 0x18);
          }
          else {
            v1 = a0->field_0x8;
            if (sub_1b950(v1,v3)) // branch-flip
              v9 = v3[3];
            else {
              v7 = __errno_location();
              v2 = *v7;
              sub_ea50(v3);
              *v7 = v2;
              v9 = 0;
            }
          }
        }
        else {
          v9 = 0;
          sub_ea50(v3);
        }
      }
      else {
        sub_ea50(v3);
        v9 = 0;
      }
    }
    else {
      free(v3);
      v9 = 0;
    }
  }
  else {
    v9 = 0;
  }
  return v9;
}


// Function: sub_ece0 @ 0xece0
void sub_ece0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,void *a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  FILE *v1;
  unsigned long v10; // stack - 0xa0
  unsigned long v11; // stack - 0x98
  unsigned long v12; // stack - 0x90
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x28
  char v2; // al
  unsigned long v20; // stack - 0x18
  int *v3; // rax
  unsigned int v4; // stack - 0xd8
  char v5 [16];
  unsigned int v6; // stack - 0xd4
  char *v7; // stack - 0xd0
  char *v8; // stack - 0xc8
  unsigned long v9; // stack - 0xa8
  
  if (v2) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v1 = *(FILE **)((long)a8 + 8);
  v7 = &Stack0000000000000008;
  v4 = 0x10;
  v8 = v5;
  v6 = 0x30;
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  if (0 <= __vfprintf_chk(v1,1,a9,&v4))
    return;
  v3 = __errno_location();
  sub_e970(*v3,*(char **)((long)a8 + 0x10));
}


// Function: sub_edc0 @ 0xedc0
unsigned char * sub_edc0(long a0,long a1,int a2)
{
  unsigned int *v1;
  unsigned long v10; // rax
  tm *v11; // rax
  char *v12; // rax
  unsigned long v13; // rax
  char *v14;
  unsigned int v15; // edx
  unsigned int v16; // stack - 0x68
  long v17; // stack - 0xc8
  char v18 [8]; // stack - 0x74
  tm v19; // stack - 0xb8
  unsigned long v2;
  unsigned int v20; // stack - 0x6c
  unsigned char v21;
  long v22;
  long v23; // stack - 0xf0
  long v24; // stack - 0xe0
  long v25; // stack - 0xc0
  unsigned int v26; // stack - 0x64
  unsigned char v3;
  long v4;
  bool v5;
  bool v6;
  int v7;
  unsigned int v8;
  unsigned char *v9;
  
  v17 = a0;
  v25 = a1;
  if (a2 != 0x2b) { // branch-flip
    v18[1] = (char)a2;
    v18[0] = 0x25;
    v18[2] = 0;
    if ((0x19 <= (unsigned int)(a2 - 0x40U)) || (!(0x1180001UL >> ((unsigned long)(unsigned int)(a2 - 0x40U) & 0x3f) & 1))) {
      v16 &= 0xffffff00;
      v6 = 0;
      v5 = 0;
      v8 = 0;
    }
    else {
      v6 = 1;
      v5 = 1;
      v8 = __snprintf_chk((char *)&v16,0x20,1,0x20,".%09ld0",a1);
    }
    if (a2 == 0x40) goto label_ee40;
  }
  else {
    v18._0_8_ = 0x64252d6d252d5925;
    v20 = 0x54252b;
    v8 = __snprintf_chk((char *)&v16,0x20,1,0x20,".%09ld0",a1);
    v6 = 1;
  }
  v5 = v6;
  v11 = localtime(&v17);
  if (v11) {
    v10 = (unsigned long)(int)v8;
    v12 = (char *)sub_25a50(strlen(v18) + 2);
    *v12 = '_';
    memcpy(&v12[1],v18,strlen(v18) + 1);
    v19._8_8_ = *(unsigned long *)&v11->field_0x8;
    v19._16_8_ = *(unsigned long *)&v11->field_0x10;
    v19._24_8_ = *(unsigned long *)&v11->field_0x18;
    v19._0_4_ = SUB84(*(unsigned long *)v11,0);
    v19._32_8_ = *(unsigned long *)&v11->field_0x20;
    v19._40_8_ = *(unsigned long *)&v11->field_0x28;
    v19._48_8_ = *(unsigned long *)&v11->field_0x30;
    v7 = v19._0_4_ + 0xb;
    if (0xb <= (int)v19._0_4_)
      v7 = v19._0_4_ + -0xb;
    v19._4_4_ = SUB84((unsigned long)*(unsigned long *)v11 >> 0x20,0);
    v19._0_4_ = v7;
    v14 = (char *)dat_38b08;
    if (!dat_38b08) {
      dat_38b00 = 1;
      v14 = (char *)sub_25a50(1);
      dat_38b08 = (unsigned char *)v14;
    }
    while ((v13 = strftime(v14,dat_38b00,v12,v11), !v13 || (dat_38b00 <= v13))) {
      v14 = (char *)sub_25ca0(v14,(unsigned long *)0x38b00,1);
      dat_38b08 = (unsigned char *)v14;
    }
    v2 = v10 + 1 + v13;
    dat_38b08 = (unsigned char *)sub_25aa0(v14,v2);
    dat_38b00 = v2;
    v14 = (char *)sub_25a50(v2);
    strftime(v14,dat_38b00,v12,&v19);
    v9 = dat_38b08;
    v21 = *dat_38b08;
    if (v21) {
      v23 = 0;
      v22 = 0;
      v24 = 0;
      v6 = 0;
      do {
        v3 = v14[v22];
        if (!v3) {
          free(v12); // return-dupe
          free(v14);
          return &v9[1];
        }
        if (v3 != v21) {
          v4 = *(long *)__ctype_b_loc();
          if ((!(*(unsigned char *)(v4 + 1 + (unsigned long)v21 * 2) & 8)) || (!(*(unsigned char *)(v4 + 1 + (unsigned long)v3 * 2) & 8))) {
            free(v12);
            free(v14);
            return &v9[1];
          }
          if (v6) {
            if (v23 != v22 - v24) {
              free(v12);
              free(v14);
              return &v9[1];
            }
            v23 += 1;
          }
          else {
            v23 = 1;
            v6 = 1;
            v24 = v22;
          }
        }
        v22 += 1;
        v21 = v9[v22];
      } while (v21);
      if ((!v14[v22]) && (v23 == 2)) {
        v24 += 2;
        v1 = (unsigned int *)&v9[v24];
        if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)(unsigned char)*v1 * 2) & 8)) {
          if (v2 != v13 + 1 + v10)
            __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,"do_time_format"); // no-return
          memmove(&v9[v10 + v24],v1,(v13 + 1) - v24);
          if (8 <= v8) { // branch-flip
            *(unsigned long *)v1 = CONCAT44(v26,v16);
            *(unsigned long *)((long)v1 + ((unsigned long)v8 - 8)) = *(unsigned long *)&v18[(unsigned long)v8 + 4];
            v22 = (long)v1 - ((unsigned long)&v1[2] & 0xfffffffffffffff8);
            v8 = v8 + (int)v22 & 0xfffffff8;
            if (8 <= v8) {
              v15 = 0;
              do {
                v10 = (unsigned long)v15;
                v15 += 8;
                *(unsigned long *)(((unsigned long)&v1[2] & 0xfffffffffffffff8) + v10) = *(unsigned long *)((long)&v16 + (v10 - v22));
              } while (v15 < v8);
            }
          }
          else if (v10 & 4) { // branch-flip
            *v1 = v16;
            *(unsigned int *)((long)v1 + ((unsigned long)v8 - 4)) = *(unsigned int *)((long)&v20 + (unsigned long)v8);
          }
          else if (v8) {
            *(unsigned char *)v1 = (unsigned char)v16;
            if (v10 & 2)
              *(unsigned short *)((long)v1 + ((unsigned long)v8 - 2)) = *(unsigned short *)((long)&v20 + (unsigned long)v8 + 2);
          }
        }
      }
    }
    free(v12);
    free(v14);
    return &v9[1];
  }
label_ee40:
  v22 = -v17;
  if (-v17 <= -1)
    v22 = v17;
  v9 = (unsigned char *)sub_1bb50(v22,(char *)0x38b21,0,1,1);
  if ((unsigned char *)0x38b21 <= v9) {
    if ((unsigned char *)0x38e37 <= v9)
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date"); // no-return
    if (v17 <= -1) {
      v9[-1] = 0x2d;
      v9 = &v9[-1];
    }
    if (v5) {
      v10 = strlen((char *)v9);
      if ((unsigned char *)0x317 <= &v9[v10 - 0x38b20])
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date"); // no-return
      v10 = 0x316 - (long)&v9[v10 - 0x38b20];
      if (v10 <= strlen((char *)&v16)) {
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)(int)v8,v10,(char *)&v16);
        if (v10 <= strlen((char *)&v16))
          __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date"); // no-return
      }
      strcat((char *)v9,(char *)&v16);
    }
    return v9;
  }
  __assert_fail("p > buf","print.c",0x2b9,"format_date"); // no-return
}


// Function: sub_f400 @ 0xf400
unsigned long sub_f400(unsigned long a0,long a1) // early-return
{
  int v1;
  tm *v10; // rax
  unsigned long v11; // stack - 0x18
  long v12; // stack - 0x10
  int v2;
  int v3;
  int v4;
  int v5;
  int v6;
  int v7;
  char *v8;
  char *v9;
  
  v11 = a0;
  v12 = a1;
  v10 = localtime(&v11);
  if (!v10)
    return sub_edc0(v11,v12,0x40);
  v1 = *(int *)&v10->field_0x18;
  if (v1 < 0)
    __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format"); // no-return
  if (6 < v1)
    __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format"); // no-return
  v2 = *(int *)&v10->field_0x10;
  if (v2 < 0)
    __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format"); // no-return
  if (0xb < v2)
    __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format"); // no-return
  v3 = *(int *)&v10->field_0x8;
  if (v3 < 0)
    __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format"); // no-return
  if (0x17 < v3)
    __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format"); // no-return
  v4 = *(int *)&v10->field_0x4;
  if (v4 <= 0x3b) {
    v5 = *(int *)v10;
    if (0x3d < v5)
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format"); // no-return
    v6 = *(int *)&v10->field_0x14;
    v7 = *(int *)&v10->field_0xc;
    v8 = *(char **)((long)v2 * 8 + 0x36260);
    v9 = *(char **)((long)v1 * 8 + 0x362c0);
    if (__snprintf_chk((char *)0x38700,0x400,1,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",v9,v8,v7,v3,v4,v5,v12,v6 + 0x76c) <= 0x3ff)
      return 0x38700;
    __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format"); // no-return
  }
  __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format"); // no-return
}


// Function: sub_f620 @ 0xf620
void sub_f620(void *a0,struct_37 *a1,char *a2,struct_36 *a3) // return-dupe
{
  FILE *v1;
  char v10 [10];
  char *v11; // stack - 0x360
  stat v12; // stack - 0x358
  void *v13;
  unsigned long v14; // r8
  unsigned long v15;
  char v16; // stack - 0x2be
  unsigned int v2;
  int v3;
  int *v4; // rax
  passwd *v5; // rax
  char *v6;
  unsigned long v7; // rax
  unsigned long v8; // rcx
  char *v9;
  
  v3 = a1->field_0x0;
  if (v3 == 1) {
    v3 = a1->field_0x10;
    v1 = *(FILE **)((long)a0 + 8);
    v13 = a1->field_0x8;
    if (fwrite(v13,1,(long)v3,v1) < (unsigned long)(long)v3) {
      v4 = __errno_location();
      sub_e970(*v4,*(char **)((long)a0 + 0x10));
    }
    v1 = *(FILE **)((long)a0 + 8);
    if (!sub_180c0(v1))
      return;
    v4 = __errno_location(); // return-dupe
    sub_e970(*v4,*(char **)((long)a0 + 0x10)); // tail-call
    return;
  }
  if (v3 != 2) {
    if (v3)
      return;
label_f688:
    v3 = a1->field_0x10;
    v1 = *(FILE **)((long)a0 + 8);
    v13 = a1->field_0x8;
    if ((unsigned long)(long)v3 <= fwrite(v13,1,(long)v3,v1))
      return;
    v4 = __errno_location();
    sub_e970(*v4,*(char **)((long)a0 + 0x10));
    return;
  }
  switch(a1->field_0x4) {
    case 0:
      error(1,0,dcgettext(NULL,"error: %s at end of format string",5),"%");
      goto label_f65e;
    default:
      return;
    case 0x25:
      goto label_f688;
    case 0x44:
      v15 = a3->field_0x0;
      v14 = 1;
      v8 = 1;
      break;
    case 0x46:
      v9 = (char *)sub_108e0(a3,a2);
      goto label_f7c4;
    case 0x47:
      goto label_f79c;
    case 0x48:
      v9 = (char *)sub_25a50((long)(dat_385fc + 1));
      v15 = (unsigned long)dat_385fc;
      memcpy(v9,a2,v15);
      v9[v15] = '\0';
      sub_ece0(a0,a1->field_0x8,v9);
      free(v9); // return-dupe, tail-call
      return;
    case 0x4d:
      sub_18290(a3,v10);
      v16 = 0;
      sub_ece0(a0,a1->field_0x8,v10);
      return;
    case 0x50:
      v9 = "";
      if (1 <= (int)dat_385e0)
        v9 = &(&a2[dat_385fc])[a2[dat_385fc] == '/'];
label_f7c4:
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,v9);
label_fa78:
      if (0 <= v3)
        return;
      v4 = __errno_location();
      sub_e970(*v4,*(char **)((long)a0 + 0x10));
      return;
    case 0x53:
      sub_ece0(a0,a1->field_0x8); // tail-call
      return;
    case 0x55:
      goto label_f752;
    case 0x59:
      v2 = a3->field_0x18 & 0xf000;
      if (v2 == 0xa000) {
        if (fstatat(dat_385f8,dat_385f0,&v12,0)) {
          v4 = __errno_location();
          v13 = a1->field_0x8;
          v3 = *v4;
          if ((v3 != 2) && (v3 != 0x14)) {
            if (v3 != 0x28) {
              sub_ece0(a0,v13,"?");
              v7 = sub_e8a0(0,a2);
              error(0,*v4,"%s",v7);
              return;
            }
            sub_ece0(a0,v13,"L");
            return;
          }
          sub_ece0(a0,v13,"N");
          return;
        }
        v9 = "f";
        v2 = v12._24_4_ & 0xf000;
        if (((v2 != 0x8000) && (v9 = "d", v2 != 0x4000)) && ((v9 = "l", v2 != 0xa000 && ((((v9 = "s", v2 != 0xc000 && (v9 = "b", v2 != 0x6000)) && (v9 = "c", v2 != 0x2000)) && (v9 = "U", v2 == 0x1000))))))
          v9 = "p";
        sub_ece0(a0,a1->field_0x8,v9); // return-dupe
        return;
      }
      v9 = "f";
      if ((v2 != 0x8000) && (v9 = "d", v2 != 0x4000)) goto label_f96d;
      goto label_f9c1;
    case 0x5a:
      v3 = (*dat_38680)(dat_385f8,dat_385f0,&v11);
      if (0 <= v3) {
        sub_ece0(a0,a1->field_0x8,v11);
        freecon(v11);
        return;
      }
label_f65e:
      sub_ece0(a0);
      v7 = sub_e8a0(0,a2);
      v9 = dcgettext(NULL,"getfilecon failed: %s",5);
      error(0,*__errno_location(),v9,v7);
      dat_38604 = 1;
      return;
    case 0x61:
      v9 = (char *)sub_f400(a3->field_0x48,a3->field_0x50);
      goto label_f9c1;
    case 0x62:
      v15 = a3->field_0x40;
      v14 = 0x200;
      v8 = 0x200;
      break;
    case 99:
      v9 = (char *)sub_f400(a3->field_0x68,a3->field_0x70);
      goto label_f9c1;
    case 100:
      v13 = a1->field_0x8;
      v2 = dat_385e0;
      sub_ece0(a0,v13,v2); // return-dupe, tail-call
      return;
    case 0x66:
      v9 = (char *)sub_17b90(a2);
label_fbce:
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,v9);
      goto label_fb12;
    case 0x67:
      v5 = (passwd *)getgrgid(a3->field_0x20);
      if (v5) {
label_f9ef:
        *(char *)((long)a1->field_0x8 + (long)a1->field_0x10) = 0x73;
        v13 = a1->field_0x8;
        v9 = *(char **)v5;
        sub_ece0(a0,v13,v9); // return-dupe, tail-call
        return;
      }
      goto label_f79c;
    case 0x68:
      v9 = (char *)sub_25f30(a2);
      for (v6 = &v9[strlen(v9) - 1]; v9 <= v6; v6 = &v6[-1]) {
        if (*v6 != '/') {
          if ((v9 < v6) && (v6[1] == '/'))
            v6[1] = '\0';
          break;
        }
      }
      v6 = strrchr(v9,0x2f);
      if (v6) {
        *v6 = '\0';
        goto label_fbce;
      }
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,".");
      goto label_fb12;
    case 0x69:
      v15 = a3->field_0x8;
      v14 = 1;
      v8 = 1;
      break;
    case 0x6b:
      v15 = a3->field_0x40;
      v14 = 0x400;
      v8 = 0x200;
      break;
    case 0x6c:
      if ((a3->field_0x18 & 0xf000) == 0xa000) {
        v9 = (char *)sub_17650(dat_385f8,dat_385f0);
        if (v9) goto label_fbce;
        sub_e8c0(*__errno_location(),a2);
        dat_38604 = 1;
      }
      v9 = NULL;
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,"");
label_fb12:
      if (v3 <= -1) {
        v4 = __errno_location();
        sub_e970(*v4,*(char **)((long)a0 + 0x10));
      }
      free(v9);
      return;
    case 0x6d:
      v13 = a1->field_0x8;
      v2 = a3->field_0x18 & 0xfff;
      sub_ece0(a0,v13,v2);
      return;
    case 0x6e:
      v15 = a3->field_0x10;
      v14 = 1;
      v8 = 1;
      break;
    case 0x70:
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,a2);
      goto label_fa78;
    case 0x73:
      v15 = a3->field_0x30;
      v14 = 1;
      v8 = 1;
      break;
    case 0x74:
      v9 = (char *)sub_f400(a3->field_0x58,a3->field_0x60);
      goto label_f9c1;
    case 0x75:
      v5 = getpwuid(a3->field_0x1c);
      if (v5) goto label_f9ef;
      v15 = (unsigned long)a3->field_0x1c; // return-dupe
      v14 = 1;
      v8 = 1;
      v9 = (char *)sub_1bb50(v15,v10,0,v8,v14);
      sub_ece0(a0,a1->field_0x8,v9);
      return;
    case 0x79:
      v9 = "f";
      v2 = a3->field_0x18 & 0xf000;
      if (((v2 != 0x8000) && (v9 = "d", v2 != 0x4000)) && (v9 = "l", v2 != 0xa000)) {
label_f96d:
        v9 = "s";
        if ((v2 != 0xc000) && (((v9 = "b", v2 != 0x6000 && (v9 = "c", v2 != 0x2000)) && (v9 = "U", v2 == 0x1000))))
          v9 = "p";
      }
label_f9c1:
      v13 = a1->field_0x8;
      sub_ece0(a0,v13,v9);
      return;
    
  }
label_f770:
  v9 = (char *)sub_1bb50(v15,v10,0,v8,v14);
  sub_ece0(a0,a1->field_0x8,v9);
  return;
label_f752:
  v15 = (unsigned long)a3->field_0x1c;
  v14 = 1;
  v8 = 1;
  goto label_f770;
label_f79c:
  v15 = (unsigned long)a3->field_0x20;
  v14 = 1;
  v8 = 1;
  goto label_f770;
}


// Function: sub_ff90 @ 0xff90
unsigned long sub_ff90(char *a0,struct_36 *a1,void *a2) // return-dupe
{
  char v1;
  unsigned long v2; // rax
  int *v3;
  unsigned long v4;
  unsigned long v5;
  
  v3 = *(int **)((long)a2 + 0x38);
  if (!v3)
    return 1;
  a2 = (void *)((long)a2 + 0x38);
  do {
    while ((*v3 != 2 || (!*(char *)((long)v3 + 5)))) {
      sub_f620(a2,v3,a0,a1);
      v3 = *(int **)&v3[6];
      if (!v3)
        return 1;
    }
    v1 = (char)v3[1];
    if (v1 != 'C') { // branch-flip
      if ('C' < v1) {
        if (v1 != 'T')
          __assert_fail("0","print.c",0x518,"pred_fprintf"); // no-return, return-dupe
        v4 = a1->field_0x60;
        v5 = a1->field_0x58;
        goto label_ffe6;
      }
      if (v1 == 'A') {
        v4 = a1->field_0x50;
        v5 = a1->field_0x48;
        goto label_ffe6;
      }
      if (v1 != 'B') {
        __assert_fail("0","print.c",0x518,"pred_fprintf");
      }
      v5 = 0xffffffffffffffff;
      v4 = 0xffffffffffffffff;
      if (*(char *)((long)v3 + 5) == '@') goto label_ffe6;
      sub_ece0(a2,*(unsigned long *)&v3[2],"");
    }
    else {
      v4 = a1->field_0x70;
      v5 = a1->field_0x68;
label_ffe6:
      v2 = sub_edc0(v5,v4);
      sub_ece0(a2,*(unsigned long *)&v3[2],v2);
    }
    v3 = *(int **)&v3[6];
    if (!v3)
      return 1;
  } while( true );
}


// Function: sub_100b0 @ 0x100b0
long sub_100b0(long *a0,char *a1,unsigned int a2,unsigned int a3,char a4,char a5,void *a6)
{
  long v1;
  unsigned int v2; // eax
  unsigned int *v3; // rax
  char *v4; // rax
  unsigned long v5; // r14
  char v6; // stack - 0x39
  
  if (a4 == '{')
    __assert_fail("format_char != \'{\'","print.c",0x4e,"make_segment"); // no-return
  if (a4 == '[')
    __assert_fail("format_char != \'[\'","print.c",0x4f,"make_segment"); // no-return
  if (a4 == '(')
    __assert_fail("format_char != \'(\'","print.c",0x50,"make_segment"); // no-return
  v5 = (unsigned long)(int)a2;
  v3 = (unsigned int *)sub_25a50(0x20);
  *a0 = (long)v3;
  *v3 = a3;
  *(char *)&v3[1] = a4;
  *(char *)((long)v3 + 5) = a5;
  v3[6] = 0;
  v3[7] = 0;
  v3[4] = a2;
  v4 = (char *)sub_25a50(v5 + 2);
  *(char **)&v3[2] = v4;
  v4 = &strncpy(v4,a1,v5)[v5];
  if (a3 < 2) {
    if (a4)
      __assert_fail("0 == format_char","print.c",0x61,"make_segment"); // no-return
    if (!a5) {
      v1 = *a0;
      *v4 = '\0';
      return v1 + 0x18;
    }
    __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment"); // no-return
  }
  if (a3 != 2)
    __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment"); // no-return
  switch(a4) {
    case 0x25:
    case 100:
      v6 = a4;
      goto label_101f9;
    default:
      v2 = 0;
      break;
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x46:
    case 0x47:
    case 0x4d:
    case 0x54:
    case 0x55:
    case 0x61:
    case 0x62:
    case 99:
    case 0x67:
    case 0x6b:
    case 0x6e:
    case 0x73:
    case 0x74:
    case 0x75:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x48:
    case 0x50:
    case 0x66:
    case 0x68:
    case 0x70:
      v6 = 's';
label_101f9:
      *v4 = v6;
      v4[1] = '\0';
      return *a0 + 0x18; // return-dupe
    case 0x53:
      *v4 = 'g';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x59:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 2;
      break;
    case 0x5a:
      *v4 = 's';
      v2 = 5;
      v4 = &v4[1];
      break;
    case 0x69:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1c) = 1;
      v2 = 1;
      break;
    case 0x6c:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 4;
      break;
    case 0x6d:
      *v4 = 'o';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x79:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1b) = 1;
      v2 = 2;
    
  }
  *v4 = '\0';
  if (*(unsigned int *)((long)a6 + 0x20) < v2)
    *(unsigned int *)((long)a6 + 0x20) = v2;
  return *a0 + 0x18;
}


// Function: sub_10380 @ 0x10380
unsigned long sub_10380(unsigned long *a0,struct_55 *a1,unsigned char *a2) // return-dupe x2
{
  unsigned char v1;
  int v10;
  unsigned char *v11;
  int v12; // r13d
  unsigned char *v13;
  unsigned char *v14;
  long v15; // r15
  unsigned long v16; // rbp
  long *v17; // stack - 0x48
  unsigned long v2;
  void *v3; // rax
  unsigned char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned char v7;
  char *v8; // rsi
  long v9;
  
  v3 = (void *)sub_d490(a1,sub_ff90,a2);
  v2 = a0[1];
  *(unsigned long *)((long)v3 + 0x38) = *a0;
  *(unsigned long *)((long)v3 + 0x40) = v2;
  v2 = a0[3];
  *(unsigned long *)((long)v3 + 0x48) = a0[2];
  *(unsigned long *)((long)v3 + 0x50) = v2;
  v2 = a0[4];
  v17 = (long *)((long)v3 + 0x38);
  *(unsigned int *)((long)v3 + 0x18) = 0x101;
  *(unsigned long *)((long)v3 + 0x58) = v2;
  *(unsigned int *)((long)v3 + 0x20) = 0;
  *(unsigned long *)((long)v3 + 0x38) = 0;
  v7 = *a2;
  if (!v7)
    return 1;
  v13 = a2;
  do {
    while( true ) {
      v1 = a2[1];
      v16 = (unsigned long)v1;
      v12 = (int)v13;
      if (v7 != 0x5c) break;
      v10 = (int)a2;
      if (v1 == 99) {
        sub_100b0(v17,v13,v10 - v12,1,0,0,v3);
        if (!*(char *)((long)v3 + 0x1a))
          return 1;
        if (3 <= *(unsigned int *)((long)v3 + 0x20))
          return 1;
        *(unsigned int *)((long)v3 + 0x20) = 3;
        return 1;
      }
      v4 = &a2[1];
      if (v1) { // branch-flip
        if (8 <= (unsigned char)(v1 - 0x30)) { // branch-flip
          if ((0x1b <= (unsigned char)(v1 + 0xa4)) || (!"\\"[(unsigned char)(v1 + 0xa4)])) {
            v11 = &a2[2];
            error(0,0,dcgettext(NULL,"warning: unrecognized escape `\\%c\'",5),v1);
            a2 = v4;
            goto label_1040f;
          }
          *a2 = "\\"[(unsigned char)(v1 + 0xa4)];
          v11 = &a2[2];
          a2 = v4;
        }
        else {
          v7 = 0;
          v9 = 0;
          while( true ) {
            if (8 <= (unsigned char)((char)v16 - 0x30U)) break;
            v11 = &v4[1];
            v7 = (char)v16 + '\xd0' + v7 * '\b';
            v14 = v4;
            if (v9 + 1 == 3) goto label_10740;
            v16 = (unsigned long)*v11;
            v4 = v11;
            v9 += 1;
          }
          v14 = &a2[v9];
          v11 = v4;
label_10740:
          *a2 = v7;
          a2 = v14;
        }
      }
      else {
        error(0,0,dcgettext(NULL,"warning: escape `\\\' followed by nothing at all",5));
        v11 = v4;
      }
      v17 = (long *)sub_100b0(v17,v13,(v10 - v12) + 1,0,0,0,v3);
      v7 = a2[1];
      a2 = v11;
      v13 = v11;
      if (!v7) goto label_104b7;
    }
    if (v7 != 0x25) // branch-flip
      v11 = &a2[1];
    else {
      if (!v1) {
        error(1,0,dcgettext(NULL,"error: %s at end of format string",5),(char *)a2);
        return v5;
      }
      if (v1 != 0x25) { // branch-flip
        v9 = 0;
        while( true ) {
          v15 = v9 + 1;
          if ((!(char)v16) || (!strchr("-+ #",(int)(char)v16))) break;
          v16 = (unsigned long)a2[v9 + 2];
          v9 = v15;
        }
        v9 = *(long *)__ctype_b_loc();
        while (v7 = (unsigned char)v16, *(unsigned char *)(v9 + 1 + v16 * 2) & 8) {
          v15 += 1;
          v16 = (unsigned long)a2[v15];
        }
        v4 = &a2[v15];
        if (v7 == 0x2e) {
          v4 = &a2[v15 + 1];
          v7 = *v4;
          if (*(unsigned char *)(v9 + 1 + (unsigned long)v7 * 2) & 8) {
            v11 = &a2[v15 + 2];
            do {
              v4 = v11;
              v7 = *v4;
              v11 = &v4[1];
            } while (*(unsigned char *)(v9 + 1 + (unsigned long)v7 * 2) & 8);
          }
        }
        v16 = (unsigned long)(unsigned int)(int)(char)v7;
        a2 = v4;
        if (strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)(char)v7)) { // branch-flip
          if (v7) {
            v10 = 0;
            goto label_1068d;
          }
          v16 = 0;
label_1056a:
          if (strchr("{[(",(int)v16)) {
            error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(char)v16); // return-dupe
            return v6;
          }
          v4 = &a2[1];
          v8 = "warning: unrecognized format directive `%%%c\'";
        }
        else {
label_10610:
          if (!strchr("ABCT",(int)v16)) goto label_1056a;
          v10 = (int)(char)a2[1];
          v4 = &a2[1];
          if (a2[1]) goto label_1068d;
          if (strchr("{[(",(int)v16)) {
            error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(char)v16);
            return v6;
          }
          v8 = "warning: format directive `%%%c\' should be followed by another character";
        }
        error(0,0,dcgettext(NULL,v8,5),v16);
        v17 = (long *)sub_100b0(v17,v13,(int)v4 - v12,0,0,0,v3);
        v11 = v4;
      }
      else {
        v4 = &a2[1];
        v16 = 0x25;
        a2 = v4;
        if (!strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",0x25)) goto label_10610;
        v16 = 0x25;
        v10 = 0;
label_1068d:
        v11 = &v4[1];
        v17 = (long *)sub_100b0(v17,v13,(int)a2 - v12,2,v16,v10,v3);
        a2 = v4;
      }
      v13 = v11;
    }
label_1040f:
    v7 = a2[1];
    a2 = v11;
  } while (v7);
label_104b7:
  if (v11 <= v13)
    return 1;
  sub_100b0(v17,v13,(int)v11 - (int)v13,0,0,0,v3);
  return 1;
}


// Function: sub_10870 @ 0x10870
long sub_10870(char a0)
{
  long v1;
  long v2;
  
  if (dat_38e40) {
    if ((dat_38e38 == '\x01') || (v2 = dat_38e40, !a0))
      return dat_38e40;
    do {
      v1 = *(long *)(v2 + 0x30);
      sub_1efb0(v2);
      v2 = v1;
    } while (v1);
    dat_38e40 = 0;
  }
  dat_38e40 = sub_1e550((unsigned char)a0);
  dat_38e38 = a0;
  return dat_38e40;
}


// Function: sub_108e0 @ 0x108e0
void sub_108e0(long *a0) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3;
  long v4;
  long v5 [19]; // stack - 0xc8
  long v6;
  
  if (dat_38e58) {
    if ((dat_38e50) && (*a0 == dat_38e48))
      return;
    free(dat_38e58);
  }
  dat_38e48 = *a0;
  v3 = sub_10870(1);
  if (!v3) {
    error(1,0,dcgettext(NULL,"Cannot read mounted file system list",5));
    return;
  }
  v6 = 0;
  do {
    v1 = *(char **)(v3 + 0x18);
    if (strcmp(v1,"ignore")) {
      v4 = *(long *)(v3 + 0x20);
      if (*(long *)(v3 + 0x20) == -1) {
        sub_d810(v5);
        v2 = (*dat_38670)(*(unsigned long *)(v3 + 8),v5);
        if (v2) goto label_10970;
        *(long *)(v3 + 0x20) = v5[0];
        v4 = v5[0];
      }
      if (*a0 == v4)
        v6 = v3;
    }
label_10970:
    v3 = *(long *)(v3 + 0x30);
    if (!v3) {
      if (v6) {
        v1 = *(char **)(v6 + 0x18);
        dat_38e58 = (void *)sub_25f30(v1);
        dat_38e50 = 1;
        return;
      }
      dat_38e50 = 0;
      dat_38e58 = (unsigned long)sub_25f30(dcgettext(NULL,"unknown",5));
      return;
    }
  } while( true );
}


// Function: sub_10a50 @ 0x10a50
unsigned long sub_10a50(char *a0)
{
  char *v1;
  long v2;
  
  if (strcmp("afs",a0)) {
    v2 = sub_10870(0);
    if (v2) {
      while( true ) {
        v1 = *(char **)(v2 + 0x18);
        if (!strcmp(a0,v1)) break;
        v2 = *(long *)(v2 + 0x30);
        if (!v2)
          return 0;
      }
    }
  }
  return 1;
}


// Function: sub_10ac0 @ 0x10ac0
void * sub_10ac0(long *a0)
{
  long v1;
  void *v10;
  long v11; // stack - 0x100
  unsigned long v2;
  int v3; // eax
  void *v4; // rax
  unsigned long v5; // stack - 0xe0
  long v6 [19]; // stack - 0xd8
  long v7;
  long v8;
  void *v9;
  
  v5 = 0;
  v4 = (long)sub_1e550(0);
  v7 = (long)v4;
  if (v4) { // branch-flip
    v11 = 0;
    v8 = (long)v4;
    v9 = NULL;
    do {
      while( true ) {
        v1 = v11 + 1;
        v2 = v1 * 8;
        v4 = (void *)sub_15be0(v9,v2,&v5);
        v10 = v4;
        if (v4) break;
        v10 = NULL;
        free(v9);
        v8 = *(long *)(v8 + 0x30);
        v9 = v10;
        if (!v8) goto label_10b90;
      }
      v4 = *(void **)(v8 + 0x20);
      if (v4 != (void *)0xffffffffffffffff) { // branch-flip
label_10b36:
        *(void **)((long)v10 + (v2 - 8)) = v4;
        v11 = v1;
      }
      else {
        sub_d810(v6);
        v3 = (*dat_38670)(*(unsigned long *)(v8 + 8),v6);
        if (!v3) {
          *(long *)(v8 + 0x20) = v6[0];
          v4 = (void *)v6[0];
          goto label_10b36;
        }
      }
      v8 = *(long *)(v8 + 0x30);
      v9 = v10;
    } while (v8);
label_10b90:
    do {
      v8 = *(long *)(v7 + 0x30);
      sub_1efb0(v7);
      v7 = v8;
    } while (v8);
    if (v10)
      *a0 = v11;
  }
  else {
    v10 = NULL;
  }
  v4 = v10;
  return v4;
}


// Function: sub_10c30 @ 0x10c30
bool sub_10c30(unsigned long a0,long a1,int *a2)
{
  long v1;
  
  if (a1) {
    v1 = *(long *)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      dat_38690 = v1;
    }
    return v1 != 0;
  }
  return 0;
}


// Function: sub_10c80 @ 0x10c80
void sub_10c80(void)
{
  char *v1; // rdx
  unsigned int v2; // stack - 0x14
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  sub_16370("find");
  __printf_chk(1,dcgettext(NULL,"Features enabled: ",5));
  __printf_chk(1,"D_TYPE ");
  v1 = "enabled";
  if (!dat_38678)
    v1 = "disabled";
  __printf_chk(1,"O_NOFOLLOW(%s) ",v1);
  __printf_chk(1,"LEAF_OPTIMISATION ");
  if (1 <= is_selinux_enabled())
    __printf_chk(1,"SELINUX ");
  v2 = 0;
  if (sub_8530(&v2)) {
    __printf_chk(1,"FTS(");
    if (v2 & 0x200)
      __printf_chk(1,"FTS_CWDFD");
    __printf_chk(1,") ");
  }
  __printf_chk(1,"CBO(level=%d) ",(unsigned int)dat_38688);
  __printf_chk(1,"\n");
  exit(0); // no-return
}


// Function: sub_10e20 @ 0x10e20
unsigned long sub_10e20(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  *(unsigned int *)(v1 + 0x18) = 1;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_10e50 @ 0x10e50
unsigned long sub_10e50(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  if (!dat_38620)
    *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned short *)(v1 + 0x18) = 1;
  return 1;
}


// Function: sub_10e80 @ 0x10e80
unsigned long sub_10e80(int *a0)
{
  *(unsigned int *)(sub_d520(a0) + 0x24) = 0x3a83126f;
  return 1;
}


// Function: sub_10ea0 @ 0x10ea0
unsigned long sub_10ea0(int *a0)
{
  struct_56 *v1; // rax
  unsigned int v2;
  
  v1 = (struct_56 *)sub_d520(a0);
  v1->field_0x18 = 0;
  v2 = dat_2bc38;
  if (v1->field_0x0 != sub_96e0)
    v2 = dat_2bc3c;
  v1->field_0x24 = v2;
  return 1;
}


// Function: sub_10ee0 @ 0x10ee0
unsigned long sub_10ee0(int *a0)
{
  *(unsigned int *)(sub_d520(a0) + 0x24) = 0x3c23d70a;
  return 1;
}


// Function: sub_10f00 @ 0x10f00
unsigned long sub_10f00(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  dat_38620 = 0;
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_10f30 @ 0x10f30
char * sub_10f30(long a0,int *a1,struct_55 *a2,void *a3) // return-dupe x2
{
  char v1;
  char *v2;
  long v3;
  char *v4; // rax
  char *v5; // r12
  float v6;
  
  v5 = "-xtype";
  if (a3 != sub_9af0)
    v5 = "-type";
  if (!a0) {
    v4 = NULL;
    return v4;
  }
  v2 = *(char **)(a0 + (long)*a1 * 8);
  if (!v2) {
    v4 = NULL;
    return v4;
  }
  *a1 = *a1 + 1;
  if (*v2) {
    v4 = (long)sub_d490(a2,a3,v2);
    v3 = (long)v4;
    *(unsigned int *)((long)v4 + 0x24) = 0;
    *(bool *)((long)v4 + 0x1a) = a3 == sub_9af0;
    *(bool *)((long)v4 + 0x1b) = a3 != sub_9af0;
    v1 = *v2;
    if (!v1) {
      v4 = (char *)0x1;
      return v4;
    }
    do {
      if (0x30 <= (unsigned char)(v1 + 0xbcU)) {
label_110d2:
        v4 = dcgettext(NULL,"Unknown argument to %s: %c",5);
        error(1,0,v4,v5,v1);
        return v4;
      }
      switch(v1 + 0xbcU) {
        case 0:
          v4 = dcgettext(NULL,"%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.",5);
          error(1,0,v4,v5,0x44);
          goto label_1113d;
        default:
          goto label_110d2;
        case 0x1e:
          v4 = NULL;
          v6 = dat_2bc44;
          break;
        case 0x1f:
          v4 = (char *)0x1;
          v6 = dat_2bc48;
          break;
        case 0x20:
          v4 = (char *)0x2;
          v6 = dat_2bc4c;
          break;
        case 0x22:
          v4 = (char *)0x3;
          v6 = dat_2bc50;
          break;
        case 0x28:
          v4 = (char *)0x4;
          v6 = dat_2bc54;
          break;
        case 0x2c:
          v4 = (char *)0x5;
          v6 = dat_2bc58;
          break;
        case 0x2f:
          v4 = (char *)0x6;
          v6 = dat_2bc5c;
        
      }
      if (*(char *)(v3 + 0x38 + (long)v4)) {
label_1113d:
        v4 = dcgettext(NULL,"Duplicate file type \'%c\' in the argument list to %s.",5);
        error(1,0,v4,(unsigned long)(unsigned int)(int)v1,v5);
label_11167:
        v4 = dcgettext(NULL,"Must separate multiple arguments to %s using: \',\'",5);
        error(1,0,v4,v5);
        break;
      }
      *(float *)(v3 + 0x24) = v6 + *(float *)(v3 + 0x24);
      *(char *)(v3 + 0x38 + (long)v4) = 1;
      if (!v2[1]) {
        v4 = (char *)0x1;
        return v4;
      }
      if (v2[1] != ',') goto label_11167;
      v1 = v2[2];
      v2 = &v2[2];
    } while (v1);
    v4 = dcgettext(NULL,"Last file type in list argument to %s is missing, i.e., list is ending on: \',\'",5);
    error(1,0,v4,v5);
  }
  v4 = dcgettext(NULL,"Arguments to %s should contain at least one letter",5);
  error(1,0,v4,v5);
  return v4;
}


// Function: sub_11200 @ 0x11200
void sub_11200(struct_55 *a0,long a1,int *a2)
{
  sub_10f30(a1,a2,a0,sub_9900); // tail-call
}


// Function: sub_11220 @ 0x11220
char * sub_11220(void)
{
  int v1; // eax
  char *v2; // rax
  
  if (dat_38e60) {
    v2 = (char *)0x1;
    return v2;
  }
  v1 = fnmatch("foo","foo",0);
  if (!v1) {
    v1 = fnmatch("Foo","foo",0);
    if (v1) {
      v1 = fnmatch("Foo","foo",0x10);
      if (!v1) {
        dat_38e60 = 1;
        v2 = (char *)0x1;
        return v2;
      }
    }
  }
  v2 = dcgettext(NULL,"sanity check of the fnmatch() library function failed.",5);
  error(1,0,v2);
  return v2;
}


// Function: sub_112c0 @ 0x112c0
unsigned long sub_112c0(struct_55 *a0,long a1,int *a2,char *a3,void *a4) // return-dupe
{
  char *v1;
  long v2; // rax
  char *v3; // rax
  
  sub_11220();
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_d490(a0,a4,v1);
  *(unsigned short *)(v2 + 0x1a) = 0;
  *(char **)(v2 + 0x38) = v1;
  if (strpbrk(v1,"*?[")) // branch-flip
    *(unsigned int *)(v2 + 0x24) = dat_2bc60;
  else {
    *(unsigned int *)(v2 + 0x24) = dat_2bc64;
  }
  if ((((!dat_38631) && (v3 = strrchr(v1,0x2f), v3)) && (!v3[1])) && (!sub_b4b0(v1,a4 == sub_91e0))) {
    error(0,0,dcgettext(NULL,"warning: -%s %s will not match anything because it ends with /.",5),a3,v1);
    *(unsigned int *)(v2 + 0x24) = 0x322bcc77;
  }
  return 1;
}


// Function: sub_113f0 @ 0x113f0
unsigned long sub_113f0(struct_55 *a0,long a1,int *a2)
{
  return sub_112c0(a0,a1,a2); // tail-call
}


// Function: sub_115c0 @ 0x115c0
unsigned long sub_115c0(int *a0)
{
  *(unsigned int *)(sub_d4d0(a0,0) + 0x24) = 0x38d1b717;
  return 1;
}


// Function: sub_115e0 @ 0x115e0
unsigned long sub_115e0(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  long v2; // rax
  unsigned int v3;
  
  sub_11220();
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_d4d0(a0,v1);
  *(char **)(v2 + 0x38) = v1;
  v3 = dat_2bc6c;
  if (!strpbrk(v1,"*?["))
    v3 = dat_2bc70;
  *(unsigned int *)(v2 + 0x24) = v3;
  return 1;
}


// Function: sub_11690 @ 0x11690
unsigned long sub_11690(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  long v2; // rax
  unsigned int v3;
  
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_d4d0(a0,v1);
  *(char **)(v2 + 0x38) = v1;
  v3 = dat_2bc6c;
  if (!strpbrk(v1,"*?["))
    v3 = dat_2bc70;
  *(unsigned int *)(v2 + 0x24) = v3;
  return 1;
}


// Function: sub_11720 @ 0x11720
char * sub_11720(int *a0,long a1,int *a2)
{
  unsigned char *v1;
  unsigned long v10;
  unsigned int v11;
  unsigned char v2;
  unsigned char *v3;
  int v4; // eax
  char *v5; // rax
  unsigned char v6;
  long v7; // stack - 0x48
  char v8 [14];
  unsigned char v9 [2]; // stack - 0x32
  
  if ((!a1) || (v5 = (long)(long)*a2, v3 = *(unsigned char **)(a1 + (long)v5 * 8), !v3)) {
    v5 = NULL;
    return v5;
  }
  v5 = (unsigned long)strlen((char *)v3);
  if (!v4) {
    v5 = dcgettext(NULL,"invalid null argument to -size",5);
    error(1,0,v5);
    return v5;
  }
  v5 = (long)(long)v4;
  v1 = &v3[(long)v5 + -1];
  v6 = *v1;
  if (v6 != 0x62) { // branch-flip
    if ('b' < (char)v6) {
      if (v6 != 0x6b) { // branch-flip
        if (v6 != 0x77) { // branch-flip
          if (v6 != 99) {
            v5 = dcgettext(NULL,"invalid -size type `%c\'",5); // return-dupe
            error(1,0,v5,v6);
            return v5;
          }
          *v1 = 0;
          v10 = 1;
        }
        else {
          *v1 = 0;
          v10 = 2;
        }
      }
      else {
        *v1 = 0;
        v10 = 0x400;
      }
      goto label_117a9;
    }
    if (v6 == 0x47) {
      *v1 = 0;
      v10 = 0x40000000;
      goto label_117a9;
    }
    if ('H' <= (char)v6) {
      if (v6 != 0x4d) {
        v5 = dcgettext(NULL,"invalid -size type `%c\'",5);
        error(1,0,v5,v6);
        return v5;
      }
      *v1 = 0;
      v10 = 0x100000;
      goto label_117a9;
    }
    if (9 < (unsigned char)(v6 - 0x30)) {
      v5 = dcgettext(NULL,"invalid -size type `%c\'",5);
      error(1,0,v5,v6);
      return v5;
    }
    v2 = *v3;
    v10 = 0x200;
    v6 = 0;
  }
  else {
    *v1 = 0;
    v10 = 0x200;
label_117a9:
    v2 = *v3;
  }
  v11 = (unsigned int)v10;
  if (v2 != 0x2b) { // branch-flip
    if (v2 != 0x2d) { // branch-flip
      v4 = sub_26490(v3,v8,10,&v7,"");
      if (v4) {
        v9[1] = 0; // return-dupe
        v9[0] = v6;
        v5 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
        error(1,0,v5,(char *)v3,(char *)v9);
        return v5;
      }
      v5 = (long)sub_d4d0(a0,v3);
      *(unsigned int *)((long)v5 + 0x38) = 2;
      *(unsigned int *)((long)v5 + 0x3c) = v11;
      v11 = dat_2bc40;
    }
    else {
      v4 = sub_26490(&v3[1],v8,10,&v7,"");
      if (v4) {
        v9[1] = 0;
        v9[0] = v6;
        v5 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
        error(1,0,v5,(char *)v3,(char *)v9);
        return v5;
      }
      v5 = (long)sub_d4d0(a0,v3);
      *(unsigned int *)((long)v5 + 0x3c) = v11;
      *(unsigned int *)((long)v5 + 0x38) = 1;
      v11 = dat_2bc3c;
      if ((unsigned long)(v10 * v7) <= 0x5000)
        v11 = dat_2bc64;
    }
  }
  else {
    v4 = sub_26490(&v3[1],v8,10,&v7,"");
    if (v4) {
      v9[1] = 0;
      v9[0] = v6;
      v5 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
      error(1,0,v5,(char *)v3,(char *)v9);
      return v5;
    }
    v5 = (long)sub_d4d0(a0,v3);
    *(unsigned int *)((long)v5 + 0x3c) = v11;
    *(unsigned int *)((long)v5 + 0x38) = 0;
    v11 = dat_2bc64;
    if ((unsigned long)(v10 * v7) <= 0x5000)
      v11 = dat_2bc3c;
  }
  *(long *)((long)v5 + 0x40) = v7;
  *(unsigned short *)((long)v5 + 0x1a) = 1;
  *(unsigned int *)((long)v5 + 0x24) = v11;
  *a2 = *a2 + 1;
  v5 = (char *)0x1;
  return v5;
}


// Function: sub_11a40 @ 0x11a40
unsigned long sub_11a40(int *a0,long a1,int *a2)
{
  char *v1;
  int v2;
  int v3; // eax
  long v4; // rax
  long v5; // stack - 0x168
  stat v6; // stack - 0xd8
  unsigned int v7;
  long v8; // stack - 0x160
  unsigned int v9; // stack - 0x150
  
  sub_d810(&v5);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,&v5);
  if (v2)
    sub_e920(*__errno_location(),v1); // return-dupe, no-return
  sub_d810(&v6);
  v7 = 0;
  if (!dat_38668) {
    if (dat_38678) // branch-flip
      v7 = 0x20000;
    else if ((v9 & 0xf000) == 0xa000) {
      v2 = -2;
      goto label_11b07;
    }
  }
  v2 = sub_160b0(v1,v7);
  if (0 <= v2) {
    if ((fstat(v2,&v6)) || (v3 = (*dat_38670)(v1,&v5), v3)) {
      sub_e920(*__errno_location(),v1);
    }
    if ((dat_38668) || (dat_38678)) {
      v5 = v6._0_8_;
      v8 = v6._8_8_;
    }
    else if ((((v9 & 0xf000) == 0xa000) || (v5 != v6._0_8_)) || (v8 != v6._8_8_)) {
      close(v2);
      v2 = -1;
    }
  }
label_11b07:
  v4 = sub_d4d0(a0,v1);
  *(int *)(v4 + 0x48) = v2;
  *(long *)(v4 + 0x38) = v8;
  *(unsigned int *)(v4 + 0x24) = 0x3c23d70a;
  *(long *)(v4 + 0x40) = v5;
  *(unsigned short *)(v4 + 0x1a) = 1;
  return 1;
}


// Function: sub_11c90 @ 0x11c90
void sub_11c90(char *a0,struct_47 *a1)
{
  int v1; // eax
  FILE *v2;
  char *v3; // rax
  bool v4; // al
  
  a1->field_0x0 = 0;
  a1->field_0x20 = sub_23640(NULL);
  if (strcmp(a0,"/dev/stderr")) { // branch-flip
    if (strcmp(a0,"/dev/stdout")) { // branch-flip
      v2 = (FILE *)sub_eb40(dat_38610,a0);
      a1->field_0x10 = a0;
      a1->field_0x8 = v2;
      if (!v2)
        sub_e9b0(*__errno_location(),a0); // no-return
    }
    else {
      a1->field_0x8 = stdout;
      v3 = dcgettext(NULL,"standard output",5);
      v2 = a1->field_0x8;
      a1->field_0x10 = v3;
    }
  }
  else {
    a1->field_0x8 = stderr;
    v3 = dcgettext(NULL,"standard error",5);
    v2 = a1->field_0x8;
    a1->field_0x10 = v3;
  }
  v1 = fileno(v2);
  v4 = 0;
  if (v1 != -1)
    v4 = isatty(v1) != 0;
  a1->field_0x18 = v4;
}


// Function: sub_11d80 @ 0x11d80
unsigned long sub_11d80(struct_55 *a0,long a1,int *a2) // early-return x2
{
  int v1;
  unsigned char *v2;
  unsigned long v3; // rax
  char v4 [40];
  
  if (!a1)
    return 0;
  v1 = *a2;
  v2 = *(unsigned char **)(a1 + (long)v1 * 8);
  if (!v2)
    return 0;
  *a2 = v1 + 1;
  sub_11c90("/dev/stdout",v4);
  v3 = sub_10380(v4,a0,v2);
  if (!(char)v3)
    *a2 = v1;
  return v3;
}


// Function: sub_11e20 @ 0x11e20
unsigned long sub_11e20(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  *(unsigned int *)(v1 + 0x18) = 0x101;
  sub_11c90("/dev/stdout",(struct_47 *)(v1 + 0x38));
  return 1;
}


// Function: sub_11e50 @ 0x11e50
bool sub_11e50(int *a0,long a1,int *a2)
{
  char *v1;
  long v2; // rax
  
  if (a1) {
    v1 = *(char **)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      v2 = sub_d4d0(a0,v1);
      sub_11c90(v1,(struct_47 *)(v2 + 0x38));
      *(unsigned int *)(v2 + 0x18) = 0x101;
      *(unsigned int *)(v2 + 0x24) = 0x3f800000;
    }
    return v1 != NULL;
  }
  return 0;
}


// Function: sub_11ec0 @ 0x11ec0
bool sub_11ec0(int *a0,long a1,int *a2)
{
  char *v1;
  long v2; // rax
  
  if (a1) {
    v1 = *(char **)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      v2 = sub_d520(a0);
      sub_11c90(v1,(struct_47 *)(v2 + 0x38));
      *(unsigned int *)(v2 + 0x24) = 0x3f800000;
      *(unsigned short *)(v2 + 0x18) = 0x101;
    }
    return v1 != NULL;
  }
  return 0;
}


// Function: sub_11f30 @ 0x11f30
unsigned long sub_11f30(struct_55 *a0,long a1,int *a2)
{
  char *v1;
  unsigned char *v2;
  unsigned long v3; // rax
  char v4 [40];
  int v5;
  
  v5 = *a2;
  if (a1) {
    v1 = *(char **)(a1 + (long)v5 * 8);
    if ((v1) && (v2 = *(unsigned char **)(a1 + 8 + (long)v5 * 8), v2)) {
      *a2 = v5 + 2;
      sub_11c90(v1,v4);
      v5 = *a2;
      v3 = sub_10380(v4,a0,v2);
      if ((char)v3)
        return v3;
    }
  }
  *a2 = v5;
  return 0;
}


// Function: sub_11fd0 @ 0x11fd0
bool sub_11fd0(int *a0,long a1,int *a2)
{
  char *v1;
  long v2; // rax
  
  if (a1) {
    v1 = *(char **)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      v2 = sub_d4d0(a0,v1);
      sub_11c90(v1,(struct_47 *)(v2 + 0x38));
      *(unsigned int *)(v2 + 0x18) = 0x101;
      *(unsigned int *)(v2 + 0x24) = 0x3f800000;
    }
    return v1 != NULL;
  }
  return 0;
}


// Function: sub_12040 @ 0x12040
char * sub_12040(int *a0,long a1,int *a2)
{
  unsigned char *v1;
  int v2;
  char *v3;
  int v4; // eax
  char *v5; // rax
  unsigned long v6; // rbx
  unsigned char *v7;
  unsigned int v8; // r14d
  unsigned int v9; // stack - 0x40
  
  if (!a1) {
    v5 = NULL;
    return v5;
  }
  v1 = *(unsigned char **)(a1 + (long)*a2 * 8);
  if (v1) {
    v4 = *a2 + 1;
    *a2 = v4;
    if (*v1 != 0x2d) { // branch-flip
      if (*v1 != 0x2f) { // branch-flip
        v8 = 2;
        v9 = dat_2bc40;
        v7 = v1;
      }
      else {
        v9 = dat_2bc74;
        v7 = &v1[1];
        v8 = 1;
      }
    }
    else {
      v7 = &v1[1];
      v8 = 0;
      v9 = dat_2bc38;
    }
    v5 = (char *)sub_1def0(v7);
    v3 = v5;
    if ((v5) && ((*v1 != 0x2b || (8 <= (unsigned char)(v1[1] - 0x30))))) {
      v4 = sub_1e310(0,0,0,v3,NULL);
      v2 = v4;
      v4 = sub_1e310(0,1,0,v3,NULL);
      v6 = CONCAT44(v4,v2);
      free(v3);
      if ((*v1 == 0x2f) && (!v4 && !v2)) {
        v8 = 0;
        v5 = dcgettext(NULL,"warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.",5);
        error(0,0,v5,(char *)v1);
        v9 = dat_2bc78;
      }
      v5 = (long)sub_d4d0(a0,v1);
      *(unsigned int *)((long)v5 + 0x38) = v8;
      *(unsigned long *)((long)v5 + 0x3c) = v6;
      *(unsigned int *)((long)v5 + 0x24) = v9;
      v5 = (char *)0x1;
      return v5;
    }
    v5 = (char *)sub_239f0(0,dat_3868c,v1);
    v3 = v5;
    v5 = dcgettext(NULL,"invalid mode %s",5);
    error(1,0,v5,v3);
    return v5;
  }
  v5 = NULL;
  return v5;
}


// Function: sub_12230 @ 0x12230
unsigned long sub_12230(char *a0,void *a1,long a2,int *a3)
{
  struct_57 *v1;
  unsigned long v10; // rax
  long v11;
  char *v12; // rax
  char *v13; // rax
  unsigned long v14; // rax
  long v15; // rax
  unsigned long v16; // rax
  unsigned long v17; // rax
  unsigned long v18; // rax
  unsigned long v19; // rax
  struct_6 *v2;
  long *v20;
  unsigned long v21; // stack - 0x60
  long v22; // stack - 0x68
  char v23 [24];
  int v24;
  int v25; // ebp
  long *v26;
  long *v27;
  char *v28; // stack - 0xb8
  char v3;
  int v4;
  void *v5;
  bool v6;
  void *v7;
  void *v8; // rax
  char *v9;
  
  v5 = *(void **)((long)a1 + 0x18);
  if ((!a2) || (!*(long *)(a2 + (long)*a3 * 8)))
    return 0;
  v8 = (void *)sub_d490(a1,v5,"(some -exec* arguments)");
  *(unsigned int *)((long)v8 + 0x18) = 0x101;
  *(unsigned long *)((long)v8 + 0xf8) = 0;
  if (v5 != sub_9580 && v5 != sub_95d0) {
    *(char *)((long)v8 + 0xf4) = 0;
    v7 = sub_8fc0;
label_123d6:
    if (v5 != v7) {
      if (!dat_385c0)
        __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok"); // no-return
      *(long *)((long)v8 + 0xf8) = dat_385c0;
      goto label_123f8;
    }
  }
  else {
    dat_38698 = 1;
    *(char *)((long)v8 + 0xf4) = 1;
    if (v5 != sub_8fc0) {
      v7 = sub_95d0;
      goto label_123d6;
    }
  }
  dat_3862e = 0;
  v9 = getenv("PATH");
  if (v9) {
    sub_17570(v9,":",1,&v22,&v21);
    do {
      if (!v21) {
        error(1,0,dcgettext(NULL,"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)",5),a0); // return-dupe
        return v10;
      }
      v3 = v9[v22];
      if ((v21 == 1) && (v3 == '.')) {
        error(1,0,dcgettext(NULL,"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)",5),a0);
        return v10;
      }
      if (v3 != '/') {
        v12 = strndup(&v9[v22],v21);
        if (!v12)
          v12 = &v9[v22];
        v9 = (char *)sub_e8a0(0,v12);
        error(1,0,dcgettext(NULL,"The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH",5),v9,a0);
        return v17;
      }
    } while (sub_17570(v9,":",0,&v22,&v21));
  }
label_123f8:
  *(char *)((long)v8 + 0x38) = 0;
  v4 = *a3;
  v11 = (long)v4 * 8;
  v27 = (long *)(a2 + v11);
  v9 = (char *)*v27;
  v24 = v4;
  if (v9) {
    v26 = (long *)(a2 + 8 + v11);
    v28 = NULL;
    v25 = 0;
    v6 = 0;
    v20 = v27;
    do {
      if (*v9 != ';') { // branch-flip
        if ((((*v9 == '+') && (v5 != sub_9580 && v5 != sub_95d0)) && (!v9[1])) && (v6)) {
          *(char *)((long)v8 + 0x38) = 1;
label_124de:
          if ((v4 != v24) && (*v20)) {
            if (*(char *)((long)v8 + 0x38)) {
              v9 = "dir";
              if (v5 != sub_8fc0)
                v9 = "";
              if (1 < v25) {
                error(1,0,dcgettext(NULL,"Only one instance of {} is supported with -exec%s ... +",5),v9);
                return v19;
              }
              if (strlen(v28) != 2) {
                __snprintf_chk(v23,0x13,1,0x13,"-exec%s ... {} +",v9);
                v9 = (char *)sub_239f0(2,dat_3868c,v28);
                v12 = (char *)sub_239f0(1,dat_3868c,"{}");
                v13 = (char *)sub_239f0(0,dat_3868c,v23);
                error(1,0,dcgettext(NULL,"In %s the %s must appear by itself, but you specified %s",5),v13,v12,v9);
                return v14;
              }
            }
            v1 = (struct_57 *)((long)v8 + 0x40);
            if ((unsigned int)(sub_156e0(v1,0x800) - 1) < 2) {
              error(1,0,dcgettext(NULL,"The environment is too large for exec().",5));
              return v18;
            }
            sub_15860(v1);
            v25 = v24 - v4;
            v2 = (struct_6 *)((long)v8 + 0x98);
            *(void **)((long)v8 + 0x80) = sub_a3a0;
            if (*(char *)((long)v8 + 0x38)) { // branch-flip
              *(unsigned long *)((long)v8 + 0xe8) = 0;
              *(unsigned long *)((long)v8 + 0x70) = 0;
              *(long *)((long)v8 + 0x78) = (long)(v25 + -1);
              *(unsigned long *)((long)v8 + 0x68) = 0;
              *(unsigned long *)((long)v8 + 0x88) = 0;
              *(unsigned long *)((long)v8 + 0x90) = 0;
              sub_15890(v1,v2,(long)v8 + 0x38);
              if (v4 < v24 + -1) {
                do {
                  v9 = (char *)*v27;
                  v27 = &v27[1];
                  sub_14fb0(v1,v2,v9,strlen(v9) + 1,NULL,0,1);
                } while (v27 != (long *)(a2 + 8 + ((unsigned long)(unsigned int)((v24 - v4) - 2) + (long)v4) * 8));
              }
            }
            else {
              *(int *)((long)v8 + 0xf0) = v25;
              *(char **)((long)v8 + 0x70) = "{}";
              *(unsigned long *)((long)v8 + 0x68) = strlen("{}");
              *(unsigned long *)((long)v8 + 0x88) = 0;
              *(unsigned long *)((long)v8 + 0x90) = 0;
              *(unsigned long *)((long)v8 + 0xe8) = sub_25a50((long)v25 << 3);
              sub_15890(v1,v2,(long)v8 + 0x38);
              v4 = *(int *)((long)v8 + 0xf0);
              if (1 <= v4) {
                v11 = *(long *)((long)v8 + 0xe8);
                v15 = 0;
                do {
                  *(long *)(v11 + v15 * 8) = v27[v15];
                  v15 += 1;
                } while (v15 != v4);
              }
            }
            if (*v20) {
              *a3 = v24 + 1;
              return 1;
            }
            *a3 = v24;
            return 1;
          }
          goto label_1260a;
        }
      }
      else if (!v9[1]) goto label_124de;
      if (sub_1d5d0(v9,"{}")) { // branch-flip
        v25 += 1;
        if ((!v24) && ((v5 == sub_8fc0 || (v5 == sub_95d0)))) {
          error(1,0,dcgettext(NULL,"You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.",5));
          return v16;
        }
        v6 = 1;
        v28 = v9;
      }
      else {
        v6 = 0;
      }
      v9 = (char *)*v26;
      v24 += 1;
      v26 = &v26[1];
      v20 = &v20[1];
    } while (v9);
  }
label_1260a:
  *a3 = v24;
  free(v8);
  return 0;
}


// Function: sub_12920 @ 0x12920
void sub_12920(void *a0,long a1,int *a2)
{
  sub_12230("-okdir",a0,a1,a2); // tail-call
}


// Function: sub_129a0 @ 0x129a0
unsigned long sub_129a0(void)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b990();
  v1->field_0x10 = 0x200000003;
  v1->field_0x0 = sub_8940;
  v1->field_0x1a = 0;
  return 1;
}


// Function: sub_129e0 @ 0x129e0
unsigned long sub_129e0(void)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b990();
  v1->field_0x10 = 0x300000003;
  v1->field_0x0 = sub_8990;
  v1->field_0x1a = 0;
  return 1;
}


// Function: sub_12a20 @ 0x12a20
unsigned long sub_12a20(void)
{
  struct_59 *v1; // rax
  
  v1 = (struct_59 *)sub_b990();
  v1->field_0x10 = 0x100000003;
  v1->field_0x0 = sub_8ca0;
  v1->field_0x1a = 0;
  v1->field_0x24 = 0x3f800000;
  return 1;
}


// Function: sub_12a60 @ 0x12a60
unsigned long sub_12a60(void)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b990();
  v1->field_0x10 = 5;
  v1->field_0x0 = sub_8be0;
  v1->field_0x1a = 0;
  return 1;
}


// Function: sub_12a90 @ 0x12a90
unsigned long sub_12a90(int *a0,long a1,int *a2)
{
  int v1; // eax
  unsigned int v2;
  group *v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7; // rax
  char *v8;
  unsigned long v9; // rax
  
  if (!a1)
    return 0;
  v8 = *(char **)(a1 + (long)*a2 * 8);
  if (!v8)
    return 0;
  *a2 = *a2 + 1;
  v3 = getgrnam(v8);
  endgrent();
  if (v3) // branch-flip
    v2 = *(unsigned int *)&v3->field_0x10;
  else {
    v1 = (int)strspn(v8,"0123456789");
    if (!v1) {
      if (*v8) {
        v8 = (char *)sub_239f0(0,dat_3868c,v8);
        error(1,0,dcgettext(NULL,"%s is not the name of an existing group",5),v8);
        return v5;
      }
      error(1,0,dcgettext(NULL,"argument to -group is empty, but should be a group name",5));
      return v6;
    }
    if (v8[v1]) {
      v7 = (char *)sub_239f0(1,dat_3868c,&v8[v1]);
      v8 = (char *)sub_239f0(0,dat_3868c,v8);
      error(1,0,dcgettext(NULL,"%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s",5),v8,v7);
      return v9;
    }
    v2 = sub_17410(v8);
  }
  v4 = sub_d4d0(a0,v8);
  *(unsigned int *)(v4 + 0x38) = v2;
  v2 = dat_2bc68;
  if (100 <= *(unsigned long *)(v4 + 0x40))
    v2 = dat_2bc38;
  *(unsigned int *)(v4 + 0x24) = v2;
  return 1;
}


// Function: sub_12c20 @ 0x12c20
unsigned long sub_12c20(int *a0)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b9b0(a0,0);
  v1->field_0x10 = 0x400000002;
  v1->field_0x0 = sub_9350;
  v1->field_0x1a = 0;
  return 1;
}


// Function: sub_12c60 @ 0x12c60
unsigned long sub_12c60(int *a0)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b9b0(a0,0);
  v1->field_0x10 = 4;
  v1->field_0x0 = sub_9630;
  v1->field_0x1a = 0;
  return 1;
}


// Function: sub_12dc0 @ 0x12dc0
unsigned long sub_12dc0(unsigned long a0,char *a1,char **a2)
{
  int v1; // eax
  
  v1 = sub_242e0(a0,a1,a2);
  if (!v1)
    return 0;
  return sub_12d30(a0 & 0xffffffff,a1,a2,v1); // tail-call
}


// Function: sub_12fc0 @ 0x12fc0
unsigned int sub_12fc0(int *a0,long a1,int *a2)
{
  int v1;
  char *v2;
  unsigned int v3; // eax
  long v4; // rax
  unsigned int v5;
  
  if (!a1)
    return 0;
  v1 = *a2;
  v3 = 0;
  v2 = *(char **)(a1 + (long)v1 * 8);
  if (v2) {
    *a2 = v1 + 1;
    sub_11220();
    v3 = sub_12f00("-name","-wholename",v2);
    if ((char)v3) {
      v4 = sub_d4d0(a0,v2);
      *(unsigned short *)(v4 + 0x1a) = 0;
      *(char **)(v4 + 0x38) = v2;
      v5 = dat_2bc60;
      if (!strpbrk(v2,"*?["))
        v5 = dat_2bc64;
      *(unsigned int *)(v4 + 0x24) = v5;
      return v3;
    }
    *a2 = v1;
  }
  return v3;
}


// Function: sub_13090 @ 0x13090
unsigned int sub_13090(int *a0,long a1,int *a2) // return-dupe x2
{
  char *v1;
  unsigned int v2; // eax
  long v3; // rax
  unsigned int v4;
  
  sub_11220();
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_12f00("-iname","-iwholename",v1);
  if (!(char)v2)
    return 0;
  v3 = sub_d4d0(a0,v1);
  *(unsigned short *)(v3 + 0x1a) = 0;
  *(char **)(v3 + 0x38) = v1;
  v4 = dat_2bc60;
  if (!strpbrk(v1,"*?["))
    v4 = dat_2bc64;
  *(unsigned int *)(v3 + 0x24) = v4;
  return v2;
}


// Function: sub_13420 @ 0x13420
unsigned long sub_13420(unsigned long a0,long a1,int *a2) // return-dupe
{
  char *v1;
  unsigned long v2; // rax
  
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  dat_3867c = sub_17200(v1);
  v2 = sub_133b0(); // tail-call
  return v2;
}


// Function: sub_13460 @ 0x13460
char * sub_13460(unsigned long a0,long a1,int *a2,int *a3)
{
  char *v1;
  char *v2;
  int v3;
  char *v4; // rax
  unsigned int v5;
  
  v3 = *a2;
  v1 = *(char **)(a1 + -8 + (long)v3 * 8);
  if ((a1) && (v2 = *(char **)(a1 + (long)v3 * 8), v2)) {
    v3 += 1;
    *a2 = v3;
    v4 = (unsigned long)strspn(v2,"0123456789");
    v5 = dat_3868c;
    if ((1 <= v3) && (v4 = (long)(long)v3, !v2[(long)v4])) {
      v3 = sub_17410(v2,dat_3868c);
      *a3 = v3;
      if (0 <= v3) {
        v4 = (unsigned long)sub_133b0(); // tail-call
        return v4;
      }
      v5 = dat_3868c;
    }
    v4 = (char *)sub_239f0(0,v5,v2);
    v2 = v4;
    v4 = dcgettext(NULL,"Expected a positive decimal integer argument to %s, but got %s",5);
    error(1,0,v4,v1,v2);
    return v4;
  }
  v4 = NULL;
  return v4;
}


// Function: sub_13520 @ 0x13520
void sub_13520(void)
{
  sub_13460(); // tail-call
}


// Function: sub_13620 @ 0x13620
unsigned long sub_13620(int *a0,long a1,int *a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  char v1;
  char v10 [6]; // stack - 0x46
  char v11 [72];
  bool v12; // r9b
  unsigned long v13; // r9
  float v14; // xmm0_da
  unsigned int v15;
  long v16; // stack - 0x90
  long v17; // stack - 0x88
  long v18; // stack - 0x80
  long v19; // stack - 0x78
  char v2;
  long v20; // stack - 0x70
  long v21; // stack - 0x68
  void *v3;
  int v4;
  long *v5; // rax
  unsigned long v6; // rax
  char *v7;
  unsigned long v8; // rax
  int *v9; // rax
  
  if (a1) {
    v4 = *a2;
    v7 = *(char **)(a1 + (long)v4 * 8);
    if ((v7) && (strlen(v7) == 8)) {
      builtin_strncpy(v10,"aBcmt",6);
      if (strncmp("-newer",v7,6))
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY"); // no-return
      v1 = v7[6];
      v2 = v7[7];
      v12 = v1 == 'B' || v2 == 'B';
      v13 = CONCAT71((undefined7)((unsigned long)a5 >> 8),v12);
      if (v12) {
        error(0,0,dcgettext(NULL,"This system does not provide a way to find the birth time of a file.",5));
        v13 = 0;
      }
      else if (v1 != 't') {
        v13 = 0;
        if (strchr(v10,(int)v1)) {
          v13 = 0;
          if (strchr(v10,(int)v2)) {
            if (!*(long *)(a1 + 8 + (long)v4 * 8)) {
              v7 = (char *)sub_239f0(0,dat_3868c,v7);
              error(1,0,dcgettext(NULL,"The %s test needs an argument",5),v7);
              return v6;
            }
            *a2 = v4 + 1;
            v5 = (long *)sub_d4d0(a0);
            if (v1 != 'c') { // branch-flip
              if (v1 != 'm') { // branch-flip
                if (v1 != 'a') {
                  if (strchr(v10,(int)v1))
                    __assert_fail("0","parser.c",0x64c,"parse_newerXY"); // no-return
                  __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY"); // no-return
                }
                v15 = 0;
              }
              else {
                v15 = 3;
              }
            }
            else {
              v15 = 2;
            }
            *(unsigned int *)&v5[7] = v15;
            if (v2 != 't') { // branch-flip
              sub_d810(v11);
              v4 = (*dat_38670)(*(unsigned long *)(a1 + (long)*a2 * 8),v11);
              if (v4) {
                v9 = __errno_location();
                sub_e920(*v9,*(char **)(a1 + (long)*a2 * 8)); // no-return
              }
              if (v2 != 'c') { // branch-flip
                if ('d' <= v2) { // branch-flip
                  if (v2 != 'm')
                    __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime"); // no-return, return-dupe
                  v16 = v18;
                  v17 = v19;
                }
                else if (v2 != 'a') {
                  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime");
                }
              }
              else {
                v16 = v20;
                v17 = v21;
              }
              v5[8] = v16;
              v5[9] = v17;
            }
            else {
              v7 = *(char **)(a1 + (long)*a2 * 8);
              if (!sub_21de0((struct_12 *)&v5[8],v7,(long *)0x38638)) {
                v7 = (char *)sub_239f0(0,dat_3868c,*(char **)(a1 + (long)*a2 * 8));
                error(1,0,dcgettext(NULL,"I cannot figure out how to interpret %s as a date or time",5),v7);
                return v8;
              }
              v16 = v5[8];
            }
            *(unsigned int *)((long)v5 + 0x3c) = 0;
            v14 = (float)(int)((dat_38648 - v16) / 0x15180);
            v15 = dat_2bc40;
            if (((dat_2bc64 <= v14) && (v15 = dat_2bc74, dat_29578 <= v14)) && (v14 <= dat_2bc80))
              v15 = dat_2bc7c;
            v3 = (void *)*v5;
            *(unsigned int *)((long)v5 + 0x24) = v15;
            *a2 = *a2 + 1;
            if (!v3)
              __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY"); // no-return
            if (v3 != sub_9400)
              __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"); // no-return
            v13 = (unsigned long)*(unsigned char *)((long)v5 + 0x1a);
            if (!*(unsigned char *)((long)v5 + 0x1a))
              __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY"); // no-return
          }
        }
      }
      return v13 & 0xffffffff; // return-dupe
    }
  }
  v13 = 0;
  return v13 & 0xffffffff;
}


// Function: sub_13b00 @ 0x13b00
unsigned long sub_13b00(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  sub_11c90("/dev/stdout",(struct_47 *)(v1 + 0x38));
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  *(unsigned short *)(v1 + 0x18) = 0x101;
  return 1;
}


// Function: sub_13b40 @ 0x13b40
unsigned long sub_13b40(int *a0)
{
  long v1; // rax
  
  v1 = sub_d4d0(a0,0);
  sub_11c90("/dev/stdout",(struct_47 *)(v1 + 0x38));
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_13b80 @ 0x13b80
void sub_13b80(void)
{
  dat_3862c = 1;
  dat_38680 = sub_12dc0;
  dat_38668 = 1;
  if (!(dat_38660 & 2)) {
    dat_38670 = sub_ddb0;
    sub_133b0(); // tail-call
    return;
  }
  dat_38670 = sub_dec0;
  sub_133b0(); // tail-call
}


// Function: sub_13be0 @ 0x13be0
unsigned long sub_13be0(double a0,char *a1,void *a2,long a3,long a4,char *a5)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // rax
  long v4; // rdx
  double v5; // stack - 0x50
  double v6; // stack - 0x48
  double v7; // xmm0_qa
  
  if (*a1 != '+') { // branch-flip
    if (*a1 != '-') // branch-flip
      *(unsigned int *)((long)a2 + 4) = 2;
    else {
      *(unsigned int *)((long)a2 + 4) = 0;
      a1 = &a1[1];
    }
  }
  else {
    *(unsigned int *)((long)a2 + 4) = 1;
    a1 = &a1[1];
  }
  v1 = sub_25f90(a1,NULL,&v5,dat_37fe8);
  if ((char)v1) {
    v7 = (double)modf(a0 * v5,&v6) * dat_2bc88;
    if (dat_2bc88 <= v7)
      __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp"); // no-return
    v2 = (long)((double)a3 - v6);
    *(long *)((long)a2 + 8) = v2;
    if (a3 < v2 != v6 < 0.0) {
      error(1,0,a5,a1);
      return v3;
    }
    v4 = (long)((double)a4 - v7);
    if (v7 <= (double)a4)
      *(long *)((long)a2 + 0x10) = v4;
    else {
      *(long *)((long)a2 + 0x10) = v4 + 1000000000;
      *(long *)((long)a2 + 8) = v2 + -1;
    }
  }
  return (unsigned long)v1;
}


// Function: sub_13d80 @ 0x13d80
char * sub_13d80(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  unsigned long v4; // stack - 0x38
  unsigned long v5; // r12
  float v6; // xmm0_da
  unsigned int v7;
  long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  if (!a1) {
    v3 = NULL;
    return v3;
  }
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1) {
    v3 = NULL;
    return v3;
  }
  v2 = *a2 + 1;
  *a2 = v2;
  v2 = sub_13be0(v1,&v4,0,0,"arithmetic overflow while converting %s days to a number of seconds");
  v5 = (unsigned long)v2;
  if (!(char)v2) {
    v3 = dcgettext(NULL,"Invalid argument %s to -used",5);
    error(1,0,v3,v1);
    return v3;
  }
  v3 = (long)sub_d4d0(a0,v1);
  *(unsigned long *)((long)v3 + 0x38) = v4;
  *(long *)((long)v3 + 0x40) = v8;
  *(unsigned long *)((long)v3 + 0x48) = v9;
  v6 = (float)(v8 / 0x15180);
  v7 = dat_2bc40;
  if (((dat_2bc64 <= v6) && (v7 = dat_2bc74, dat_29578 <= v6)) && (v6 <= dat_2bc80))
    v7 = dat_2bc7c;
  *(unsigned int *)((long)v3 + 0x24) = v7;
  v3 = (char *)v5;
  return v3;
}


// Function: sub_13ed0 @ 0x13ed0
long sub_13ed0(long a0,int *a1,int *a2) // return-dupe
{
  unsigned char *v1;
  long v2; // rax
  char *v3; // rcx
  int v4; // ebx
  unsigned long v5; // stack - 0x30
  char v6 [8];
  unsigned char *v7;
  char *v8; // r8
  
  if (!a0)
    return 0;
  v1 = *(unsigned char **)(a0 + (long)*a1 * 8);
  if (v1) {
    *a1 = *a1 + 1;
    if (*v1 != 0x2b) { // branch-flip
      if (*v1 != 0x2d) { // branch-flip
        v4 = 2;
        v7 = v1;
      }
      else {
        v7 = &v1[1];
        v4 = 1;
      }
    }
    else {
      v7 = &v1[1];
      v4 = 0;
    }
    if (sub_26490(v7,v6,10,&v5,""))
      return 0;
    v2 = sub_d4d0(a2,v1);
    *(int *)(v2 + 0x38) = v4;
    *(unsigned long *)(v2 + 0x40) = v5;
    if (dat_38660 & 1) {
      __fprintf_chk(stderr,1,"inserting %s\n",*(char **)(v2 + 8));
      v8 = " >";
      v3 = "gt";
      if (v4) {
        v8 = " =";
        if (v4 == 1)
          v8 = " <";
        v3 = "eq";
        if (v4 == 1)
          v3 = "lt";
      }
      __fprintf_chk(stderr,1,"    type: %s    %s  ",v3,v8);
      __fprintf_chk(stderr,1,"%lu\n",*(unsigned long *)(v2 + 0x40));
      return v2;
    }
    return v2;
  }
  return 0;
}


// Function: sub_14060 @ 0x14060
unsigned long sub_14060(int *a0,long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_13ed0(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(long *)(v1 + 0x40) != 1) {
    if (*(long *)(v1 + 0x40) != 2) {
      *(unsigned int *)(v1 + 0x24) = 0x3a83126f;
      return 1;
    }
    *(unsigned int *)(v1 + 0x24) = 0x3c23d70a;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = 0x3f7d70a4;
  return 1;
}


// Function: sub_140d0 @ 0x140d0
unsigned long sub_140d0(int *a0,long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_13ed0(a1,a2,a0);
  if (v1) {
    *(unsigned int *)(v1 + 0x24) = 0x358637bd;
    *(unsigned short *)(v1 + 0x1a) = 0;
    *(char *)(v1 + 0x1c) = 1;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}


// Function: sub_141e0 @ 0x141e0
unsigned long sub_141e0(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3; // rax
  char v4 [88];
  float v5; // xmm0_da
  unsigned int v6;
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  
  sub_d810(v4);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,v4);
  if (v2)
    sub_e920(*__errno_location(),v1); // no-return
  v3 = sub_d4d0(a0,v1);
  *(unsigned long *)(v3 + 0x48) = v8;
  *(long *)(v3 + 0x40) = v7;
  *(unsigned long *)(v3 + 0x38) = 3;
  v5 = (float)(int)((dat_38648 - v7) / 0x15180);
  v6 = dat_2bc40;
  if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
    v6 = dat_2bc7c;
  *(unsigned int *)(v3 + 0x24) = v6;
  return 1;
}


// Function: sub_14340 @ 0x14340
unsigned long sub_14340(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3; // rax
  char v4 [88];
  float v5; // xmm0_da
  unsigned int v6;
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  
  sub_d810(v4);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,v4);
  if (v2)
    sub_e920(*__errno_location(),v1); // no-return
  v3 = sub_d4d0(a0,v1);
  *(unsigned long *)(v3 + 0x48) = v8;
  *(long *)(v3 + 0x40) = v7;
  *(unsigned long *)(v3 + 0x38) = 0;
  v5 = (float)(int)((dat_38648 - v7) / 0x15180);
  v6 = dat_2bc40;
  if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
    v6 = dat_2bc7c;
  *(unsigned int *)(v3 + 0x24) = v6;
  return 1;
}


// Function: sub_144a0 @ 0x144a0
unsigned long sub_144a0(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3; // rax
  char v4 [88];
  float v5; // xmm0_da
  unsigned int v6;
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  
  sub_d810(v4);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,v4);
  if (v2)
    sub_e920(*__errno_location(),v1); // no-return
  v3 = sub_d4d0(a0,v1);
  *(unsigned long *)(v3 + 0x48) = v8;
  *(long *)(v3 + 0x40) = v7;
  *(unsigned long *)(v3 + 0x38) = 2;
  v5 = (float)(int)((dat_38648 - v7) / 0x15180);
  v6 = dat_2bc40;
  if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
    v6 = dat_2bc7c;
  *(unsigned int *)(v3 + 0x24) = v6;
  return 1;
}


// Function: sub_14600 @ 0x14600
unsigned int sub_14600(int *a0,long a1,int *a2,unsigned int a3)
{
  int v1;
  unsigned int v2; // eax
  long v3;
  unsigned int v4; // stack - 0x48
  float v5; // xmm0_da
  unsigned int v6;
  unsigned int v7; // stack - 0x44
  long v8; // stack - 0x40
  unsigned long v9; // stack - 0x38
  
  if (a1) { // branch-flip
    v1 = *a2;
    v2 = 0;
    v3 = *(long *)(a1 + (long)v1 * 8);
    if (v3) {
      *a2 = v1 + 1;
      v4 = a3;
      v2 = sub_13be0(v3,&v4,dat_38648 + 0x15180,dat_38650,"arithmetic overflow while converting %s minutes to a number of seconds");
      if ((char)v2) { // branch-flip
        v3 = sub_d4d0(a0,v3);
        *(unsigned long *)(v3 + 0x38) = CONCAT44(v7,v4);
        *(long *)(v3 + 0x40) = v8;
        *(unsigned long *)(v3 + 0x48) = v9;
        v5 = (float)(int)((dat_38648 - v8) / 0x15180);
        v6 = dat_2bc40;
        if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
          v6 = dat_2bc7c;
        *(unsigned int *)(v3 + 0x24) = v6;
      }
      else {
        *a2 = v1;
      }
    }
  }
  else {
    v2 = 0;
  }
  return v2;
}


// Function: sub_14750 @ 0x14750
unsigned int sub_14750(int *a0,long a1,int *a2)
{
  return sub_14600(a0,a1,a2); // tail-call
}


// Function: sub_14780 @ 0x14780
unsigned int sub_14780(int *a0,long a1,int *a2)
{
  int v1;
  int v10; // stack - 0x54
  long v11; // stack - 0x50
  unsigned long v12; // stack - 0x48
  unsigned int v2; // eax
  char *v3; // rax
  long v4;
  unsigned int v5; // stack - 0x58
  unsigned long v6; // stack - 0x60
  char *v7;
  float v8; // xmm0_da
  unsigned int v9;
  
  v3 = dcgettext(NULL,"arithmetic overflow while converting %s days to a number of seconds",5);
  if (a1) { // branch-flip
    v1 = *a2;
    v2 = 0;
    v7 = *(char **)(a1 + (long)v1 * 8);
    if (v7) {
      *a2 = v1 + 1;
      v4 = dat_38648;
      if (*v7 == '-')
        v4 = dat_38648 + 0x1517f;
      v2 = sub_13be0(v7,&v5,v4,dat_38650,v3);
      if ((char)v2) { // branch-flip
        v4 = sub_d4d0(a0,v7);
        *(unsigned long *)(v4 + 0x38) = CONCAT44(v10,v5);
        *(long *)(v4 + 0x40) = v11;
        *(unsigned long *)(v4 + 0x48) = v12;
        v8 = (float)(int)((dat_38648 - v11) / 0x15180);
        v9 = dat_2bc40;
        if (((dat_2bc64 <= v8) && (v9 = dat_2bc74, dat_29578 <= v8)) && (v8 <= dat_2bc80))
          v9 = dat_2bc7c;
        *(unsigned int *)(v4 + 0x24) = v9;
        if (dat_38660 & 1) {
          __fprintf_chk(stderr,1,"inserting %s\n",*(char **)(v4 + 8));
          if (v10) { // branch-flip
            if (v10 != 1) { // branch-flip
              v7 = " ?";
              v3 = "?";
              if (v10 == 2) {
                __fprintf_chk(stderr,1,"    type: %s    %s  ","eq",">=");
                v6 = *(unsigned long *)(v4 + 0x40);
                v3 = ctime(&v6);
                __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v4 + 0x40),v3);
                v6 = *(long *)(v4 + 0x40) + 0x15180;
                __fprintf_chk(stderr,1,"                 <  %lu %s",v6,ctime(&v6));
                return v2;
              }
            }
            else {
              v7 = " <";
              v3 = "lt";
            }
          }
          else {
            v7 = " >";
            v3 = "gt";
          }
          __fprintf_chk(stderr,1,"    type: %s    %s  ",v3,v7);
          v6 = *(unsigned long *)(v4 + 0x40);
          v3 = ctime(&v6);
          __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v4 + 0x40),v3);
        }
      }
      else {
        *a2 = v1;
      }
    }
  }
  else {
    v2 = 0;
  }
  return v2;
}


// Function: sub_14a70 @ 0x14a70
void sub_14a70(long *a0)
{
  void *v1;
  unsigned int v2; // eax
  
  if (a0) {
    v2 = 0;
    do {
      while( true ) {
        v1 = (void *)*a0;
        if (v1 != sub_8d00) break;
        a0 = (long *)a0[0x21];
        v2 |= 1;
        if (!a0) goto label_14aba;
      }
      a0 = (long *)a0[0x21];
      if (v1 == sub_8600)
        v2 |= 2;
    } while (a0);
label_14aba:
    if ((v2 == 3) && (!dat_38621)) {
      error(1,0,dcgettext(NULL,"The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.",5));
      return;
    }
  }
}


// Function: sub_14ba0 @ 0x14ba0
void sub_14ba0(void)
{
  dat_38e70 = 0;
}


// Function: sub_14bb0 @ 0x14bb0
void sub_14bb0(void)
{
  return;
}


// Function: sub_14bc0 @ 0x14bc0
unsigned long sub_14bc0(char *a0)
{
  char v1;
  long *v2; // rbx
  int v3; // ebp
  int *v4;
  char *v5;
  
  if (!strncmp("-newer",a0,6)) {
    v4 = (int *)0x37840;
    if (strlen(a0) == 8) {
      return sub_12ca0(a0,v4); // return-dupe, tail-call
    }
  }
  v1 = *a0;
  v2 = (long *)0x36d88;
  v5 = "!";
  v3 = 0;
  while( true ) {
    if (!strcmp(v5,&a0[v1 == '-'])) break;
    v5 = (char *)*v2;
    v2 = &v2[4];
    v3 += 1;
    if (!v5)
      return 0;
  }
  v4 = (int *)((long)v3 * 0x20 + 0x36d60);
  return sub_12ca0(a0,v4);
}


// Function: sub_14c70 @ 0x14c70
unsigned long sub_14c70(int *a0,long a1,int *a2) // return-dupe x3
{
  char *v1;
  long v2; // rax
  char *v3; // rax
  stat v4;
  undefined4 v5;
  
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  if ((2 <= dat_38688) && (!sub_10a50(v1))) {
    if (dat_38660 & 8)
      __fprintf_chk(stderr,1,"-fstype %s can never succeed, substituting -false\n",v1);
    v2 = sub_d520((int *)sub_14bc0("false"));
    *(unsigned int *)(v2 + 0x18) = 0;
    *(unsigned int *)(v2 + 0x24) = 0;
    return 1;
  }
  v2 = sub_d4d0(a0,v1);
  *(char **)(v2 + 0x38) = v1;
  v5 = dat_29578;
  if (!stat("/",&v4)) {
    v3 = (char *)sub_108e0(&v4,"/");
    v5 = s_2bc83._1_4_;
    if (strcmp(v1,v3))
      v5 = dat_2bc74;
  }
  *(unsigned int *)(v2 + 0x24) = v5;
  return 1;
}


// Function: sub_14df0 @ 0x14df0
unsigned long sub_14df0(void)
{
  long v1; // rax
  
  v1 = sub_d520((int *)sub_14bc0("false"));
  *(unsigned int *)(v1 + 0x18) = 0;
  *(unsigned int *)(v1 + 0x24) = 0;
  return 1;
}

