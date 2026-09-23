// Function: main @ 0x4050
int main(int argc,char **argv,char **envp)
{
  bool v1;
  unsigned int v10; // stack - 0x98
  char *v11; // stack - 0xc8
  char *v12; // stack - 0xc0
  char *v13; // stack - 0xb8
  char *v14; // stack - 0xb0
  char v15; // stack - 0xa2
  char v16; // stack - 0xa1
  int v17; // stack - 0x94
  int v18; // stack - 0x90
  int v19; // stack - 0x8c
  int v2; // eax
  unsigned long v20; // stack - 0x88
  unsigned short v21; // stack - 0x80
  unsigned int v22; // stack - 0x7c
  char v23; // stack - 0x78
  long v24; // stack - 0x70
  char v25; // stack - 0x68
  char v26; // stack - 0x67
  char v27; // stack - 0x66
  char v28; // stack - 0x65
  char v29; // stack - 0x64
  int v3; // eax
  char v30; // stack - 0x63
  char v31; // stack - 0x62
  char v32; // stack - 0x61
  char v33; // stack - 0x60
  char v34; // stack - 0x5f
  char v35; // stack - 0x5e
  char v36; // stack - 0x5d
  char v37; // stack - 0x5c
  char v38; // stack - 0x5b
  char v39; // stack - 0x5a
  int v4; // eax
  int v40; // stack - 0x54
  unsigned long v41; // stack - 0x50
  unsigned long v42; // stack - 0x48
  int v5; // eax
  int v6; // eax
  char *v7;
  unsigned long v8; // rax
  long v9; // rdx
  
  sub_12680(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_19d70(sub_e2b0);
  dat_2315a = 0 < is_selinux_enabled();
  sub_7670(&v10);
  v23 = 0;
  v40 = 1;
  v18 = 4;
  v19 = 2;
  v22 = 0;
  v17 = 1;
  v20 = 0x100000000;
  v21 = 0;
  v24 = 0;
  v32 = 0;
  v31 = 0;
  v30 = 0;
  v29 = '\0';
  v28 = '\0';
  v27 = 0;
  v26 = 1;
  v25 = 0;
  v36 = 0;
  v35 = '\0';
  v34 = 0;
  v33 = '\0';
  v37 = 0;
  v38 = 0;
  v7 = getenv("POSIXLY_CORRECT");
  v15 = 0;
  v41 = 0;
  v39 = v7 != NULL;
  v42 = 0;
  v14 = NULL;
  v11 = NULL;
  v16 = 0;
  v12 = NULL;
  v13 = NULL;
  v1 = 0;
label_41b8:
  v2 = getopt_long(argc,argv,"abdfHilLnprst:uvxPRS:TZ",(option *)0x224a0,NULL);
  if (v2 != -1) {
    if (0x88 < v2) {
      sub_54f0(1); // return-dupe, no-return
    }
    if (v2 <= 0x47) {
      if (v2 == -0x83) {
        sub_16a20(stdout,"cp","GNU coreutils",dat_23020,"Torbjorn Granlund","David MacKenzie","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_54f0(0); // no-return
      sub_54f0(1);
    }
    switch(v2) {
      case 0x48:
        v17 = 3;
        break;
      default:
        goto label_4799;
      case 0x4c:
        v17 = 4;
        break;
      case 0x50:
        v17 = 2;
        break;
      case 0x52:
      case 0x72:
        v33 = '\x01';
        break;
      case 0x53:
        v13 = optarg;
        v1 = 1;
        break;
      case 0x54:
        v15 = 1;
        break;
      case 0x5a:
        if (dat_2315a) { // branch-flip
          if (optarg) // branch-flip
            v14 = optarg;
          else {
            v24 = selabel_open(0,0,0);
            if (!v24) {
              v7 = dcgettext(NULL,"warning: ignoring --context",5);
              error(0,*__errno_location(),v7);
            }
          }
        }
        else if (optarg)
          error(0,0,dcgettext(NULL,"warning: ignoring --context; it requires an SELinux-enabled kernel",5));
        break;
      case 0x61:
        v25 = 1;
        v17 = 2;
        v22._0_2_ = CONCAT11(1,(char)v22);
        v22 = CONCAT22(0x101,(unsigned short)v22);
        v27 = 1;
        if (dat_2315a)
          v28 = '\x01';
        v30 = 1;
        v33 = '\x01';
        v32 = 1;
        break;
      case 0x62:
        v1 = 1;
        v7 = optarg;
        if (!optarg)
          v7 = v12;
        v12 = v7;
        break;
      case 100:
        v25 = 1;
        v17 = 2;
        break;
      case 0x66:
        v20._0_7_ = CONCAT16(1,(undefined6)v20);
        break;
      case 0x69:
        v18 = 3;
        break;
      case 0x6c:
        v20 = CONCAT17(1,(undefined7)v20);
        break;
      case 0x6e:
        v18 = 2;
        break;
      case 0x70:
label_4210:
        v22._0_2_ = CONCAT11(1,(char)v22);
        v22 = CONCAT22(0x101,(unsigned short)v22);
        v27 = 1;
        break;
      case 0x73:
        v35 = '\x01';
        break;
      case 0x74:
        if (v11) {
          error(1,0,dcgettext(NULL,"multiple target directories specified",5));
          return v5;
        }
        v11 = optarg;
        break;
      case 0x75:
        v36 = 1;
        break;
      case 0x76:
        v37 = 1;
        break;
      case 0x78:
        v22 = CONCAT31(v22._1_3_,1);
        break;
      case 0x80:
        v26 = 0;
        break;
      case 0x81:
        v16 = 1;
        break;
      case 0x82:
        sub_49a0(optarg,&v10,0);
        break;
      case 0x83:
        dat_23159 = 1;
        break;
      case 0x84:
        if (optarg) {
          sub_49a0(optarg,&v10,1);
          v27 = 1;
          break;
        }
        goto label_4210;
      case 0x85:
        if (optarg) // branch-flip
          v40 = *(int *)(sub_cfc0("--reflink",optarg,(long *)0x22860,(void *)0x1a460,4,dat_23028,1) * 4 + 0x1a460);
        else {
          v40 = 2;
        }
        break;
      case 0x86:
        v19 = *(int *)(sub_cfc0("--sparse",optarg,(long *)0x22880,(void *)0x1a470,4,dat_23028,1) * 4 + 0x1a470);
        break;
      case 0x87:
        dat_23158 = 1;
        break;
      case 0x88:
        goto label_4250;
      
    }
    goto label_41b8;
  }
  if ((v20._7_1_) && (v35)) {
    v7 = "cannot make both hard and symbolic links";
    goto label_4784;
  }
  if (v18 != 2) { // branch-flip
    if (!v1) goto label_46a9;
    if ((v40 == 2) && (v19 != 2)) goto label_4778;
    v7 = dcgettext(NULL,"backup type",5);
    v10 = sub_d8d0(v7,v12);
  }
  else {
    v36 = 0;
    if (v1) {
      v7 = "options --backup and --no-clobber are mutually exclusive";
      goto label_4784;
    }
label_46a9:
    v10 = 0;
    if ((v40 == 2) && (v10 = 0, v19 != 2)) {
label_4778:
      v7 = "--reflink can be used only with --sparse=auto";
label_4784:
      error(0,0,dcgettext(NULL,v7,5));
label_4799:
      sub_54f0(1);
    }
  }
  sub_d0f0(v13);
  if (v17 != 1) { // branch-flip
label_459f:
    if (v33) goto label_4715;
  }
  else {
    if ((!v33) || (v20._7_1_)) {
      v17 = 4;
      goto label_459f;
    }
    v17 = 2;
label_4715:
    v20._0_5_ = CONCAT14(v16,(unsigned int)v20);
  }
  if (v14 || v24) { // branch-flip
    if (v29) { // branch-flip
      if (v28) {
        error(1,0,dcgettext(NULL,"cannot set target context and preserve it",5));
        return v4;
      }
      if (!dat_2315a) {
        error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5)); // return-dupe
        return v3;
      }
    }
    else {
      v28 = '\0';
    }
    if ((v14) && (setfscreatecon(v14) < 0)) {
      v8 = sub_14b30(v14);
      v7 = dcgettext(NULL,"failed to set default file creation context to %s",5);
      error(1,*__errno_location(),v7,v8);
      return v6;
    }
  }
  else if ((v29) && (!dat_2315a)) {
    error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5));
    return v3;
  }
  sub_bdb0();
  v9 = (long)optind;
  argc -= optind;
  return (sub_5a50(argc,&argv[v9],v11,(unsigned char)v15,&v10) ^ 1) & 0xff;
label_4250:
  v20._0_6_ = CONCAT15(1,(undefined5)v20);
  goto label_41b8;
}


// Function: sub_49a0 @ 0x49a0
void sub_49a0(char *a0,unsigned long a1,bool a2)
{
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  long v4; // rax
  
  v1 = (char *)sub_17130(a0);
  v2 = "--no-preserve";
  if (a2)
    v2 = "--preserve";
  v3 = strchr(v1,0x2c);
  if (v3)
    *v3 = '\0';
  v4 = sub_cfc0(v2,v1,(long *)0x22460,(void *)0x1a440,4,dat_23028);
  if (*(unsigned int *)(v4 * 4 + 0x1a440) <= 6) {
    (*(void *)((long)*(int *)((unsigned long)*(unsigned int *)(v4 * 4 + 0x1a440) * 4 + 0x1a320) + 0x1a320))(); // jump-as-call
    return;
  }
  abort(); // no-return
}


// Function: sub_4b10 @ 0x4b10
unsigned long sub_4b10(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,void *a6) // return-dupe x2
{
  unsigned int v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  void *v12;
  char *v13;
  unsigned long *v14;
  unsigned long v15; // rax
  unsigned int v16; // ecx
  unsigned long v17; // rdx
  stat *v18; // stack - 0x1a8
  stat v19; // stack - 0x168
  long v2;
  stat v20; // stack - 0xd8
  stat **v21;
  char *v22; // rsp
  stat **v23; // rsp
  char *v24; // rsp
  char *v25;
  char *v26; // rsp
  char *v27;
  int v28;
  unsigned int v29; // r13d
  long v3;
  char *v30; // stack - 0x1a0
  unsigned long *v31; // stack - 0x198
  long *v32; // stack - 0x190
  stat *v33; // stack - 0x188
  char *v34; // stack - 0x180
  char *v35; // stack - 0x178
  void *v36; // stack - 0x170
  stat **v4;
  char *v5;
  void *v6;
  char v7;
  int v8;
  unsigned int v9;
  
  v6 = a6;
  v23 = &v18;
  v21 = &v18;
  v30 = a3;
  v32 = a4;
  v34 = a5;
  v10 = sub_e4a0(a0);
  *v32 = 0;
  if (v10 <= a1)
    return 1;
  v11 = strlen(a0);
  v4 = &v18;
  while (v23 != (stat **)((long)&v18 - (v11 + 0x18 & 0xfffffffffffff000))) {
    v21 = (char *)((long)v4 + -0x1000);
    v22 = (char *)((long)v4 + -0x1000);
    *(unsigned long *)((long)v4 + -8) = *(unsigned long *)((long)v4 + -8);
    v23 = (char *)((long)v4 + -0x1000);
    v4 = (stat **)v22;
  }
  v17 = (unsigned long)((unsigned int)(v11 + 0x18) & 0xff0);
  v3 = -v17;
  v24 = (char *)((long)v21 + v3);
  v25 = (char *)((long)v21 + v3);
  if (v17)
    *(unsigned long *)((long)v21 + -8) = *(unsigned long *)((long)v21 + -8);
  *(unsigned long *)((long)v21 + v3 + -8) = 0x4c05;
  v36 = memcpy((void *)((unsigned long)((long)v21 + v3 + 0xf) & 0xfffffffffffffff0),a0,v11 + 1);
  v5 = (char *)((long)v21 + v3);
  while (v24 != (char *)((long)v21 + (v3 - (v10 + 0x18 & 0xfffffffffffff000)))) {
    v25 = &v5[-0x1000];
    v26 = &v5[-0x1000];
    *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
    v24 = &v5[-0x1000];
    v5 = v26;
  }
  v11 = (unsigned long)((unsigned int)(v10 + 0x18) & 0xff0);
  v3 = -v11;
  if (v11)
    *(unsigned long *)&v25[-8] = *(unsigned long *)&v25[-8];
  v12 = (void *)((unsigned long)&v25[v3 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v25[v3 + -8] = 0x4c6f;
  memcpy(v12,v36,v10);
  v27 = (char *)((long)v12 + a1);
  *(char *)((long)v12 + v10) = 0;
  v7 = *v27;
  while (v7 == '/') {
    v27 = &v27[1];
    v7 = *v27;
  }
  v33 = &v19;
  *(unsigned long *)&v25[v3 + -8] = 0x4ca4;
  if (!fstatat(a2,v27,v33,0)) {
    if ((v19._24_4_ & 0xf000) == 0x4000) {
      *v34 = '\0';
      return 1;
    }
label_4b75:
    *(unsigned long *)&v25[v3 + -8] = 0x5136;
    v27 = (char *)sub_14440(4,v12);
    *(unsigned long *)&v25[v3 + -8] = 0x514c;
    v13 = dcgettext(NULL,"%s exists but is not a directory",5);
    *(unsigned long *)&v25[v3 + -8] = 0x515d;
    error(0,0,v13,v27);
    return 0;
  }
  v35 = (char *)(a1 + (long)v36);
  v27 = v35;
  if (*v35 == '/') {
    do {
      v27 = &v27[1];
    } while (*v27 == '/');
  }
  v18 = &v20;
  v13 = v27;
  do {
    *(unsigned long *)&v25[v3 + -8] = 0x4cfa;
    v13 = strchr(v13,0x2f);
    if (!v13)
      return 1;
    *v13 = '\0';
    *(unsigned long *)&v25[v3 + -8] = 0x4d1d;
    v8 = fstatat(a2,v27,v33,0);
    if ((v8) || (*(unsigned int *)((long)v6 + 0x1c) & 0xffffff00)) {
      *(unsigned long *)&v25[v3 + -8] = 0x4df3;
      if (stat(v35,v18)) { // branch-flip
        *(unsigned long *)&v25[v3 + -8] = 0x4e00;
        v28 = *__errno_location();
        if (v28) {
label_4b75:
          v36 = (void *)CONCAT44(v36._4_4_,v28);
          *(unsigned long *)&v25[v3 + -8] = 0x5054;
          v27 = (char *)sub_14440(4,v35);
          *(unsigned long *)&v25[v3 + -8] = 0x506a;
          v13 = dcgettext(NULL,"failed to get attributes of %s",5);
          *(unsigned long *)&v25[v3 + -8] = 0x507f;
          error(0,(int)v36,v13,v27);
          return 0;
        }
      }
      else if ((v20._24_4_ & 0xf000) != 0x4000) {
        v28 = 0x14;
        goto label_4b75;
      }
      *(unsigned long *)&v25[v3 + -8] = 0x4e16;
      v14 = (unsigned long *)sub_16c50(0xa8);
      *v14 = v20._0_8_;
      v14[1] = v20._8_8_;
      v14[2] = v20._16_8_;
      v14[3] = CONCAT44(v20._28_4_,v20._24_4_);
      v14[4] = v20._32_8_;
      v14[5] = v20._40_8_;
      v14[6] = v20._48_8_;
      v14[7] = v20._56_8_;
      v14[8] = v20._64_8_;
      v14[9] = v20._72_8_;
      v14[10] = v20._80_8_;
      v14[0xb] = v20._88_8_;
      v14[0xc] = v20._96_8_;
      v14[0xd] = v20._104_8_;
      v14[0xe] = v20._112_8_;
      v14[0xf] = v20._120_8_;
      v14[0x10] = v20._128_8_;
      v14[0x11] = v20._136_8_;
      v14[0x13] = (long)v13 - (long)v36;
      v2 = *v32;
      *(char *)&v14[0x12] = 0;
      v14[0x14] = v2;
      *v32 = (long)v14;
      if (!v8) goto label_4d37;
      v1 = *(unsigned int *)&v14[3];
      v31 = v14;
      *(unsigned long *)&v25[v3 + -8] = 0x4ed7;
      v7 = sub_72a0(v35,v36,v1,1,v6);
      if (!v7)
        return 0;
      *v34 = '\x01';
      v9 = *(unsigned int *)&v31[3];
      if (*(char *)((long)v6 + 0x1d)) { // branch-flip
        v29 = v9 & 0x3f;
        v16 = ~v29;
      }
      else if (*(char *)((long)v6 + 0x1e)) { // branch-flip
        v29 = v9 & 0x12;
        v16 = ~v29;
      }
      else {
        v16 = 0xffffffff;
        v29 = 0;
      }
      if (*(char *)((long)v6 + 0x20))
        v9 = 0x1ff;
      *(unsigned long *)&v25[v3 + -8] = 0x4f34;
      if (mkdirat(a2,v27,v9 & v16 & 0xfff)) {
        *(unsigned long *)&v25[v3 + -8] = 0x51d4;
        v15 = sub_14440(4,v36);
        v27 = "cannot make directory %s";
        goto label_4b75;
      }
      if (v30) {
        *(unsigned long *)&v25[v3 + -8] = 0x4f70;
        __printf_chk(1,v30,v35,v36);
      }
      *(unsigned long *)&v25[v3 + -8] = 0x4f95;
      if (fstatat(a2,v27,v33,0x100)) {
        *(unsigned long *)&v25[v3 + -8] = 0x5181;
        v15 = sub_14440(4,v36);
        v27 = "failed to get attributes of %s";
        goto label_4b75;
      }
      v9 = v19._24_4_;
      if (!*(char *)((long)v6 + 0x1e)) {
        if (~v19._24_4_ & v29) { // branch-flip
          *(unsigned long *)&v25[v3 + -8] = 0x50e1;
          v29 &= ~sub_bbd0();
          v14 = v31;
          v9 = v19._24_4_;
          if (!(~v19._24_4_ & v29)) goto label_4fbf;
        }
        else {
label_4fbf:
          v14 = v31;
          v9 = v19._24_4_;
          if ((v19._24_4_ & 0x1c0) == 0x1c0) goto label_4fde;
        }
        *(char *)&v14[0x12] = 1;
        *(unsigned int *)&v14[3] = v29 | v9;
      }
label_4fde:
      if ((v9 | 0x1c0) != v9) {
        *(unsigned long *)&v25[v3 + -8] = 0x4ffa;
        if (fchmodat(a2,v27,v9 | 0x1c0,0x100)) {
          *(unsigned long *)&v25[v3 + -8] = 0x51f6;
          v15 = sub_14440(4,v36);
          v27 = "setting permissions for %s";
label_4b75:
          *(unsigned long *)&v25[v3 + -8] = 0x5197;
          v27 = dcgettext(NULL,v27,5);
          *(unsigned long *)&v25[v3 + -8] = 0x519f;
          v8 = *__errno_location();
          *(unsigned long *)&v25[v3 + -8] = 0x51b0;
          error(0,v8,v27,v15);
          return 0;
        }
      }
      if (!*v34) goto label_4d79;
    }
    else {
label_4d37:
      *(unsigned long *)&v25[v3 + -8] = 0x4d51;
      v7 = sub_72a0(v35,v36,0,0,v6);
      if (!v7)
        return 0;
      if ((v19._24_4_ & 0xf000) != 0x4000) {
        v12 = v36;
        goto label_4b75;
      }
      *v34 = '\0';
label_4d79:
      if ((*(long *)((long)v6 + 0x28)) || (*(char *)((long)v6 + 0x33))) {
        *(unsigned long *)&v25[v3 + -8] = 0x4d96;
        v7 = sub_7510(v36,0,v6);
        if ((!v7) && (*(char *)((long)v6 + 0x34)))
          return 0;
      }
    }
    v7 = v13[1];
    *v13 = '/';
    v13 = &v13[1];
    while (v7 == '/') {
      v13 = &v13[1];
      v7 = *v13;
    }
  } while( true );
}


// Function: sub_5210 @ 0x5210
unsigned long sub_5210(char *a0,int a1,unsigned long a2,void *a3,void *a4) // return-dupe
{
  char v1;
  unsigned long v10; // rax
  char *v11;
  unsigned long v12; // rdx
  unsigned long *v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long *v15;
  char *v16; // rsp
  unsigned long *v17; // rsp
  unsigned long v18; // stack - 0x70
  unsigned long v19; // stack - 0x60
  int v2;
  unsigned long v20; // stack - 0x58
  unsigned long v21; // stack - 0x50
  unsigned int v3;
  unsigned int v4;
  unsigned int v5;
  long v6;
  unsigned long *v7;
  unsigned long v8; // rax
  char *v9; // rax
  
  v15 = &v13;
  v18 = a2;
  v8 = strlen(a0);
  v17 = &v13;
  v7 = &v13;
  while (v17 != (unsigned long *)((long)&v13 - (v8 + 0x18 & 0xfffffffffffff000))) {
    v15 = (char *)((long)v7 + -0x1000);
    v16 = (char *)((long)v7 + -0x1000);
    *(unsigned long *)((long)v7 + -8) = *(unsigned long *)((long)v7 + -8);
    v17 = (char *)((long)v7 + -0x1000);
    v7 = (unsigned long *)v16;
  }
  v12 = (unsigned long)((unsigned int)(v8 + 0x18) & 0xff0);
  v6 = -v12;
  if (v12)
    *(unsigned long *)((long)v15 + -8) = *(unsigned long *)((long)v15 + -8);
  *(unsigned long *)((long)v15 + v6 + -8) = 0x52a0;
  v9 = memcpy((void *)((unsigned long)((long)v15 + v6 + 0xf) & 0xfffffffffffffff0),a0,v8 + 1);
  v11 = &v9[v18 - (long)a0];
  if (!a3)
    return 1;
  v13 = &v14;
  do {
    v1 = *(char *)((long)a4 + 0x1f);
    v9[*(long *)((long)a3 + 0x98)] = '\0';
    if (v1) {
      v14 = *(unsigned long *)((long)a3 + 0x48);
      v19 = *(unsigned long *)((long)a3 + 0x50);
      v20 = *(unsigned long *)((long)a3 + 0x58);
      v21 = *(unsigned long *)((long)a3 + 0x60);
      *(unsigned long *)((long)v15 + v6 + -8) = 0x534e;
      if (utimensat(a1,v11,v13,0)) {
        *(unsigned long *)((long)v15 + v6 + -8) = 0x5363;
        v10 = sub_14440(4,v9);
        *(unsigned long *)((long)v15 + v6 + -8) = 0x5379;
        v11 = dcgettext(NULL,"failed to preserve times for %s",5);
        *(unsigned long *)((long)v15 + v6 + -8) = 0x5381;
        v2 = *__errno_location();
        *(unsigned long *)((long)v15 + v6 + -8) = 0x5392;
        error(0,v2,v11,v10);
        return 0;
      }
    }
    if (*(char *)((long)a4 + 0x1d)) { // branch-flip
      v5 = *(unsigned int *)((long)a3 + 0x20);
      v3 = *(unsigned int *)((long)a3 + 0x1c);
      *(unsigned long *)((long)v15 + v6 + -8) = 0x53b7;
      if (!fchownat(a1,v11,v3,v5,0x100)) goto label_52d3;
      *(unsigned long *)((long)v15 + v6 + -8) = 0x53c7;
      if (!sub_76c0(a4)) {
        v18 &= 0xffffffffffffff00;
        *(unsigned long *)((long)v15 + v6 + -8) = 0x54da;
        v10 = sub_14440(4,v9);
        v11 = "failed to preserve ownership for %s";
        goto label_546b;
      }
      v5 = *(unsigned int *)((long)a3 + 0x20);
      *(unsigned long *)((long)v15 + v6 + -8) = 0x53e8;
      fchownat(a1,v11,0xffffffff,v5,0x100);
      v1 = *(char *)((long)a4 + 0x1e);
    }
    else {
label_52d3:
      v1 = *(char *)((long)a4 + 0x1e);
    }
    if (v1) { // branch-flip
      v4 = *(unsigned int *)((long)a3 + 0x18);
      *(unsigned long *)((long)v15 + v6 + -8) = 0x5419;
      if (sub_c830(v11,0xffffffff,v9,0xffffffff,v4))
        return 0;
    }
    else if (*(char *)((long)a3 + 0x90)) {
      v5 = *(unsigned int *)((long)a3 + 0x18);
      v18 &= 0xffffffffffffff00;
      *(unsigned long *)((long)v15 + v6 + -8) = 0x5447;
      if (fchmodat(a1,v11,v5,0x100)) {
        *(unsigned long *)((long)v15 + v6 + -8) = 0x545c;
        v10 = sub_14440(4,v9);
        v11 = "failed to preserve permissions for %s";
label_546b:
        *(unsigned long *)((long)v15 + v6 + -8) = 0x5472;
        v11 = dcgettext(NULL,v11,5);
        *(unsigned long *)((long)v15 + v6 + -8) = 0x547a;
        v2 = *__errno_location();
        *(unsigned long *)((long)v15 + v6 + -8) = 0x548b;
        error(0,v2,v11,v10);
        return v18 & 0xff;
      }
    }
    v9[*(long *)((long)a3 + 0x98)] = '/';
    a3 = *(void **)((long)a3 + 0xa0);
    if (!a3)
      return 1;
  } while( true );
}


// Function: sub_54f0 @ 0x54f0
void sub_54f0(int a0)
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
  
  v6 = dat_235a0;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",5),v6,v6,v6);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don\'t copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --no-preserve=ATTR_LIST  don\'t preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe backup suffix is \'~\', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n",5),v1);
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
      if (!strcmp("cp",v6)) break;
      v6 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v6);
    v6 = *(char **)((long)v4 + 8);
    if (!v6)
      v6 = "cp";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "cp";
    if (!strcmp("cp","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "cp")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}


// Function: sub_5a50 @ 0x5a50
unsigned long sub_5a50(int a0,unsigned long *a1,long a2,bool a3,int *a4)
{
  int *v1;
  int v10;
  unsigned int v11; // eax
  char *v12;
  unsigned long v13; // rax
  void *v14; // rax
  unsigned long v15; // rax
  unsigned long v16; // rax
  unsigned long v17; // rax
  char *v18;
  unsigned long v19; // rax
  int *v2;
  char *v20; // rcx
  unsigned long v21; // rdx
  long v22;
  unsigned char *v23; // stack - 0x128
  stat v24; // stack - 0xd8
  char *v25; // stack - 0xe0
  long v26;
  unsigned char v27; // stack - 0xea
  void *v28; // stack - 0xe8
  char *v29;
  void *v3;
  char *v30; // rsp
  unsigned char **v31;
  char *v32; // rsp
  int *v33;
  int *v34;
  long v35;
  unsigned int v36; // eax
  char **v37; // stack - 0x120
  char *v38; // stack - 0x118
  long v39; // stack - 0x110
  long v4;
  int *v40; // stack - 0x108
  int v41; // stack - 0x100
  int v42; // stack - 0xfc
  char *v5;
  void *v6;
  unsigned char v7;
  char v8; // al
  char v9;
  
  v31 = &v23;
  v27 = 0;
  v39 = a2; // branch-flip
  v40 = a4;
  v42 = a0;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (a0 == 1) {
      v17 = sub_14440(4,(char *)*a1);
      v18 = "missing destination file operand after %s";
label_6026:
      error(0,0,dcgettext(NULL,v18,5),v17);
      sub_54f0(1); // return-dupe, no-return
    }
    v18 = "missing file operand";
  }
  else {
    v24._24_4_ = 0;
    if (a3) { // branch-flip
      if (a2) {
        error(1,0,dcgettext(NULL,"cannot combine --target-directory (-t) and --no-target-directory (-T)",5));
        return v16;
      }
      if (2 < a0) {
        v17 = sub_14440(4,(char *)a1[2]);
        v18 = "extra operand %s";
        goto label_6026;
      }
    }
    else {
      if (a2) {
        v41 = sub_156a0(a2,&v24);
        if (v41 == -1) {
          v17 = sub_14440(4,v39);
          v18 = dcgettext(NULL,"target directory %s",5);
          error(1,*__errno_location(),v18,v17);
          return v19;
        }
        if (v42 != 1) {
label_5eef:
          v33 = v40;
          sub_75f0(v40);
          sub_7630(v33);
        }
label_5aeb:
        v37 = &v25;
        v22 = 0;
        v36 = 1;
        v38 = &v26;
        v23 = &v27;
        do {
          v18 = (char *)a1[v22];
          v25 = NULL;
          if (dat_23158) { // branch-flip
            *(unsigned long *)((long)v31 + -8) = 0x5d70;
            sub_e5a0(v18);
            if (!dat_23159) goto label_5b4d;
label_5d80:
            *(unsigned long *)((long)v31 + -8) = 0x5d85;
            v13 = strlen(v18);
            v20 = (char *)((long)v31 + -(v13 + 0x18 & 0xfffffffffffff000));
            v29 = (char *)v31;
            v5 = (char *)v31;
            while (v31 != (unsigned char **)v20) {
              v29 = &v5[-0x1000];
              v32 = &v5[-0x1000];
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
              v31 = (char *)&v5[-0x1000];
              v5 = v32;
            }
            v21 = (unsigned long)((unsigned int)(v13 + 0x18) & 0xff0);
            v4 = -v21;
            v31 = (char *)&v29[v4];
            if (v21)
              *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
            *(unsigned long *)&v29[v4 + -8] = 0x5de2;
            v12 = memcpy((void *)((unsigned long)&v29[v4 + 0xf] & 0xfffffffffffffff0),v18,v13 + 1);
            *(unsigned long *)&v29[v4 + -8] = 0x5ded;
            sub_e5a0(v12);
            *(unsigned long *)&v29[v4 + -8] = 0x5e03;
            v35 = v39;
            v14 = (void *)sub_ea10(v39,v12,v37);
            v12 = NULL;
            v10 = v40[0xf];
            *(long *)&v29[v4 + -8] = v35;
            if ((char)v10)
              v12 = "%s -> %s\n";
            *(int **)&v29[v4 + -0x10] = v40;
            *(unsigned long *)&v29[v4 + -0x18] = 0x5e46;
            v8 = sub_4b10(v14,(long)v25 - (long)v14,v41,v12,&v28,v23);
            v9 = *v25;
            while (v9 == '/') {
              v25 = &v25[1];
              v9 = *v25;
            }
            if (v8) goto label_5bf4;
            v36 = 0;
            v31 = (unsigned char **)&v29[v4];
            v9 = dat_23159;
label_5e83:
            v6 = v28;
            if (v9) {
              while (v6) {
                v3 = *(void **)((long)v6 + 0xa0);
                *(unsigned long *)((long)v31 + -8) = 0x5eb6;
                v28 = v3;
                free(v6);
                v6 = v3;
              }
            }
          }
          else {
            if (dat_23159) goto label_5d80;
label_5b4d:
            *(unsigned long *)((long)v31 + -8) = 0x5b52;
            v12 = (char *)sub_d970(v18);
            *(unsigned long *)((long)v31 + -8) = 0x5b5d;
            v13 = strlen(v12);
            v20 = (char *)((long)v31 + -(v13 + 0x18 & 0xfffffffffffff000));
            v29 = (char *)v31;
            v5 = (char *)v31;
            while (v31 != (unsigned char **)v20) {
              v29 = &v5[-0x1000];
              v30 = &v5[-0x1000];
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
              v31 = (char *)&v5[-0x1000];
              v5 = v30;
            }
            v21 = (unsigned long)((unsigned int)(v13 + 0x18) & 0xff0);
            v4 = -v21;
            v31 = (char *)&v29[v4];
            if (v21)
              *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
            *(unsigned long *)&v29[v4 + -8] = 0x5bba;
            v12 = memcpy((void *)((unsigned long)&v29[v4 + 0xf] & 0xfffffffffffffff0),v12,v13 + 1);
            *(unsigned long *)&v29[v4 + -8] = 0x5bc5;
            sub_e5a0(v12);
            *(unsigned long *)&v29[v4 + -8] = 0x5bd4;
            v10 = strcmp(v12,"..");
            *(unsigned long *)&v29[v4 + -8] = 0x5bf1;
            v14 = (void *)sub_ea10(v39,&v12[v10 == 0],v37);
label_5bf4:
            *(unsigned long *)((long)v31 + -8) = 0;
            *(char **)((long)v31 + -0x10) = v38;
            *(unsigned long *)((long)v31 + -0x18) = 0x5c23;
            v11 = sub_ba80(v18,v14,v41,v25,v27,v40);
            v36 &= v11;
            if (dat_23159) {
              *(unsigned long *)((long)v31 + -8) = 0x5ee3;
              v11 = sub_5210(v14,v41,v25,v28,v40);
              v36 &= v11;
              v9 = dat_23159;
              goto label_5e83;
            }
          }
          v22 += 1;
          *(unsigned long *)((long)v31 + -8) = 0x5c41;
          free(v14);
        } while ((int)v22 < v42);
        return (unsigned long)v36; // return-dupe
      }
      v22 = a1[(long)a0 + -1];
      v39 = v22;
      v41 = sub_156a0(v22,&v24);
      if (v41 != -1) { // branch-flip
        v42 = a0 + -1;
        if (v22) {
          if (1 < v42) goto label_5eef;
          v36 = 1;
          if (v42 != 1) {
            return (unsigned long)v36;
          }
          goto label_5aeb;
        }
      }
      else {
        v10 = *__errno_location();
        if (v10 == 2)
          v27 = 1;
        if (3 <= v42) {
          v18 = (char *)sub_14440(4,v39);
          error(1,v10,dcgettext(NULL,"target %s",5),v18);
          return v15;
        }
      }
    }
    v7 = v27;
    v18 = (char *)*a1;
    v12 = (char *)a1[1];
    if (!dat_23159) {
      if (((((*(char *)((long)v40 + 0x16)) && (*v40)) && (!strcmp(v18,v12))) && ((!v7 && ((v24._24_4_ || (!stat(v12,&v24))))))) && (v33 = v40, (v24._24_4_ & 0xf000) == 0x8000)) {
        v12 = (char *)sub_d850(0xffffff9c,v12,*v40);
        v22 = 0x16;
        v34 = (int *)0x23100;
        while (v22) {
          v2 = &v34[1];
          v1 = &v33[1];
          *v34 = *v33;
          v22 -= 1;
          v33 = v1;
          v34 = v2;
        }
        dat_23100 = 0;
        v40 = (int *)0x23100;
      }
      v36 = sub_ba80(v18,v12,0xffffff9c,v12,-(unsigned int)v7,v40,&v25,NULL);
      return (unsigned long)v36;
    }
    v18 = "with --parents, the destination must be a directory";
  }
  error(0,0,dcgettext(NULL,v18,5));
  sub_54f0(1);
}


// Function: sub_60f0 @ 0x60f0
void sub_60f0(void)
{
  return;
}


// Function: sub_6100 @ 0x6100
void sub_6100(void)
{
  sub_14440(4); // tail-call
}


// Function: sub_6110 @ 0x6110
void sub_6110(void *a0,long a1,char *a2)
{
  unsigned long v1;
  void *v2; // rax
  unsigned long v3; // rsi
  
  v3 = a1 - (long)a0;
  v1 = strlen(a2) + 1;
  v2 = (void *)sub_16c70(v1 + v3);
  memcpy((void *)((long)v2 + v3),a2,v1);
  memcpy(v2,a0,v3); // tail-call
}


// Function: sub_6170 @ 0x6170
unsigned long sub_6170(unsigned int a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if ((!dat_23560) && (dat_23560 = calloc(dat_23018,1), !dat_23560)) {
    dat_23018 = 0x400;
    dat_23560 = (void *)0x23160;
  }
  if (!a1)
    return 1;
  while( true ) {
    v2 = dat_23018;
    if (a1 <= dat_23018)
      v2 = a1;
    v1 = sub_111c0(a0,dat_23560,v2);
    if (v1 != v2) break;
    a1 -= v1;
    if (!a1)
      return 1;
  }
  return 0;
}


// Function: sub_6220 @ 0x6220
unsigned long sub_6220(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,void *a4)
{
  char v1;
  void *v10; // stack - 0x18
  int v2; // eax
  unsigned long v3; // rax
  undefined7 v4; // rax
  void *v5; // stack - 0x28
  void *v6; // r8
  void **v7;
  bool v8; // zf
  void *v9; // stack - 0x20
  
  v7 = &v5;
  v1 = *(char *)((long)a4 + 0x33);
  if ((*(char *)((long)a4 + 0x31)) && (!*(char *)((long)a4 + 0x36))) {
    if (*(char *)((long)a4 + 0x37)) {
      v6 = sub_6720;
      v7 = NULL;
      if (!v1) {
        v6 = NULL;
        v7 = NULL;
        if (*(long *)((long)a4 + 0x28)) {
          v6 = sub_6720;
          v7 = NULL;
        }
      }
      goto label_6282;
    }
    v6 = sub_6720;
    v5 = sub_6360;
    if ((!v1) && (!*(long *)((long)a4 + 0x28)))
      v6 = NULL;
  }
  else {
    v6 = sub_6720;
    v5 = sub_6440;
    if (!v1) {
      v6 = sub_6720;
      if (!*(long *)((long)a4 + 0x28))
        v6 = NULL;
      v5 = sub_6440;
    }
  }
  v9 = sub_6100;
  v10 = sub_60f0;
label_6282:
  v2 = (unsigned int)a1;
  v2 |= a3;
  if (0 <= v2) { // branch-flip
    v3 = attr_copy_fd(a0,a1,a2);
    v8 = v2 == 0;
    v4 = (undefined7)((unsigned long)v3 >> 8);
    v3 = CONCAT71(v4,v8);
  }
  else {
    v3 = attr_copy_file(a0,a2,v6,v7);
    v8 = v2 == 0;
    v4 = (undefined7)((unsigned long)v3 >> 8);
    v3 = CONCAT71(v4,v8);
  }
  return v3;
}


// Function: sub_6360 @ 0x6360
void sub_6360(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,int *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  int v1;
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
  char v2; // al
  unsigned int v3; // stack - 0xd8
  char v4 [16];
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v2) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  v1 = *__errno_location();
  if ((v1 != 0x5f) && (v1 != 0x3d)) {
    v6 = &Stack0000000000000008;
    v7 = v4;
    v3 = 0x10;
    v5 = 0x30;
    sub_163b0(0,v1,a9,&v3);
  }
}


// Function: sub_6440 @ 0x6440
void sub_6440(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,int *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  int *v2; // rax
  unsigned int v3; // stack - 0xd8
  char v4 [16];
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
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
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  v2 = __errno_location();
  v6 = &Stack0000000000000008;
  v3 = 0x10;
  v7 = v4;
  v5 = 0x30;
  sub_163b0(0,*v2,a9,&v3);
}


// Function: sub_6510 @ 0x6510
void sub_6510(char *a0,char *a1,char *a2)
{
  char *v1;
  char *v2; // rax
  
  v2 = (char *)sub_14310(1,4,a1);
  __printf_chk(1,"%s -> %s",(char *)sub_14310(0,4,a0),v2);
  if (a2) {
    v2 = (char *)sub_14440(4,a2);
    __printf_chk(1,dcgettext(NULL," (backup: %s)",5),v2);
  }
  v1 = *(char **)&stdout->field_0x28;
  if (v1 < *(char **)&stdout->field_0x30) {
    *(char **)&stdout->field_0x28 = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}


// Function: sub_65d0 @ 0x65d0
unsigned int sub_65d0(char *a0,unsigned int a1,char *a2,char *a3,unsigned int a4,unsigned long a5,char a6,unsigned int a7,unsigned char a8)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  unsigned char v5; // r12b
  unsigned int v6;
  unsigned long v7; // stack - 0x50
  
  v6 = a7;
  v1 = sub_be90(a1,a2,a4,a5,(unsigned long)a8 << 10,(unsigned char)a6);
  if (1 <= v1) { // branch-flip
    v4 = NULL;
    if (!a0) {
      v4 = (char *)sub_6110(a3,a5,a2,v7);
      a0 = v4;
    }
    v2 = (char *)sub_14310(1,4,a0);
    v3 = (char *)sub_14310(0,4,a3);
    v6 = 0;
    error(0,v1,dcgettext(NULL,"cannot create hard link %s to %s",5),v3,v2);
    free(v4);
  }
  else {
    v5 = (unsigned char)v6 & (unsigned char)((unsigned int)v1 >> 0x1f);
    v6 = CONCAT31((undefined3)((unsigned int)v6 >> 8),v5);
    if (v5) { // branch-flip
      v4 = (char *)sub_14440(4,a3,0xffffffffffffffff);
      __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v4);
    }
    else {
      v6 = 1;
    }
  }
  return v6;
}


// Function: sub_67a0 @ 0x67a0
void sub_67a0(void *a0,char *a1,int a2,char *a3,struct_27 *a4)
{
  unsigned int v1;
  char *v2;
  char *v3; // rax
  unsigned long v4; // rax
  long v5;
  char v6 [9];
  char v7; // stack - 0x32
  
  if ((((a4->field_0x18 & 0xf000) != 0xa000) && (!sub_16bb0())) && (faccessat(a2,a3,2,0x200))) {
    sub_e880(a4->field_0x18,&v5);
    v1 = a4->field_0x18;
    v7 = 0;
    v4 = sub_14440(4,a1);
    v2 = dat_235a0;
    if ((*(char *)((long)a0 + 0x18)) || (*(unsigned int *)((long)a0 + 0x14) & 0xffff00))
      v3 = dcgettext(NULL,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    else {
      v3 = dcgettext(NULL,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    __fprintf_chk(stderr,1,v3,v2,v4,(unsigned long)(v1 & 0xfff),v6);
  }
  else {
    v3 = (char *)sub_14440(4,a1);
    v2 = dat_235a0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: overwrite %s? ",5),v2,v3);
  }
  sub_17440(); // tail-call
}


// Function: sub_6920 @ 0x6920
unsigned long sub_6920(unsigned long a0,char *a1,bool a2,long a3)
{
  int v1;
  long v2; // rax
  int *v3; // rax
  char *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // r12
  
  v2 = lseek(a0,a3,1);
  if (v2 < 0) {
    v6 = sub_14440(4,a1);
    v4 = dcgettext(NULL,"cannot lseek %s",5);
    error(0,*__errno_location(),v4,v6);
    return 0;
  }
  if (a2) {
    if ((int)fallocate(a0 & 0xffffffff,3,v2 - a3,a3) <= -1) {
      v3 = __errno_location();
      v1 = *v3;
      v7 = CONCAT71((undefined7)((unsigned long)a3 >> 8),v1 == 0x5f || v1 == 0x26);
      if (v1 != 0x5f && v1 != 0x26) {
        v4 = (char *)sub_14440(4,a1);
        v5 = dcgettext(NULL,"error deallocating %s",5);
        error(0,*v3,v5,v4);
        return v7 & 0xffffffff; // return-dupe
      }
    }
  }
  v7 = 1;
  return v7 & 0xffffffff;
}


// Function: sub_6a30 @ 0x6a30
unsigned long sub_6a30(int a0,int a1,unsigned long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,char *a7,char *a8,unsigned long a9,long *a10,unsigned char *a11) // return-dupe
{
  bool v1;
  long v10; // rax
  long v11;
  int *v12; // rax
  char *v13;
  char *v14;
  char *v15;
  unsigned long v16; // rax
  bool v17;
  bool v18;
  unsigned long v19;
  unsigned int v2;
  unsigned long v20;
  unsigned long v21;
  char *v22;
  unsigned long v23;
  char *v24;
  bool v25;
  unsigned long v26;
  unsigned long v27; // stack - 0x98
  long *v3;
  unsigned char *v4;
  long *v5;
  unsigned char *v6;
  bool v7;
  int v8;
  unsigned int v9; // eax
  
  v6 = a11;
  v5 = a10;
  v13 = a8;
  v14 = a7;
  *a11 = 0;
  v27 = a9;
  *a10 = 0;
  if ((a4) || (!(char)a6)) {
    if (!a9)
      return 1;
label_6b28:
    v23 = a3;
    if (a4)
      v23 = a4;
    v26 = 0;
    v25 = 0;
    do {
      v15 = (char *)*a2;
      v2 = a6;
      v24 = a7;
      v22 = a8;
      v11 = a9;
      v3 = a10;
      v4 = a11;
      while( true ) {
        a6 = v2;
        a7 = v24;
        a8 = v22;
        a9 = v11;
        a10 = v3;
        a11 = v4;
        if (!v15) {
          v15 = (char *)sub_16bf0((long)getpagesize(),a3);
          *a2 = v15;
        }
        v11 = a3;
        if (v27 <= a3)
          v11 = v27;
        v11 = read(a0,v15,v11);
        if (v11 <= -1) break;
        if (!v11) {
label_7023:
          if (v25)
            return sub_6920(a1,v13,(unsigned char)a5,v26); // tail-call
          return 1;
        }
        *v5 = *v5 + v11;
        v19 = v23;
        v21 = v11;
        v24 = v15;
        v17 = v25;
label_6c20:
        do {
          v20 = v19;
          if (v21 < v19)
            v20 = v21;
          v25 = v20 && a4;
          if (v20 && a4) {
            v19 = v20;
            v22 = v15;
            do {
              if (*v22) {
                v18 = v17;
                v7 = 0;
                v1 = v25;
                goto label_6c89;
              }
              v22 = &v22[1];
              v19 -= 1;
              if (!v19) {
                v18 = (bool)((v17 ^ 1U) & v26 != 0);
                goto label_6d90;
              }
            } while (v19 & 0xf);
            v8 = memcmp(v15,v22,v19);
            v18 = (bool)(v8 == 0 ^ v17);
            v7 = v8 == 0;
            v1 = v8 != 0;
label_6c89:
            v25 = v7;
            v18 = (bool)(v26 != 0 & v18);
            if ((v20 != v21) || (!v1)) {
label_6d90:
              if (!v18) goto label_6d99;
              v1 = 0;
            }
            else {
              if (!v18) {
                v25 = 0;
                goto label_6d32;
              }
              v1 = 1;
              v25 = 0;
            }
label_6d46:
            if (v17) { // branch-flip
              v9 = sub_6920(a1,v13,(unsigned char)a5,v26);
              if (!(char)v9)
                return (unsigned long)v9;
            }
            else if (v26 != sub_111c0(a1,v24,v26)) {
              v16 = sub_14440(4,v13);
              v14 = dcgettext(NULL,"error writing %s",5);
              error(0,*__errno_location(),v14,v16);
              return 0;
            }
            v24 = v15;
            v26 = v20;
            v17 = v25;
            if (v1) {
              if (v20) { // branch-flip
                if (v18) {
                  v19 = 0;
                  goto label_6c20;
                }
              }
              else {
                if (v18) {
                  v26 = 0;
                  break;
                }
                v21 = 0;
              }
              v21 -= v20;
              v26 = 0;
            }
            else {
              v21 -= v20;
            }
          }
          else {
            if (((v20 == v21) && (v17 != 1)) || (v25 = v17, !v20)) {
label_6d32:
              v26 += v20;
              v18 = 0;
              v1 = 1;
              goto label_6d46;
            }
label_6d99:
            v26 += v20;
            if (v26 + 0x8000000000000000 < v20) {
              v14 = (char *)sub_14440(4,v14);
              error(0,0,dcgettext(NULL,"overflow reading %s",5),v14);
              return 0;
            }
            v21 -= v20;
            v17 = v25;
          }
          v15 = &v15[v20];
          v19 = v20;
        } while (v21);
        v27 -= v11;
        *v6 = v25;
        if (!v27) goto label_7023;
        v15 = (char *)*a2;
        v2 = a6;
        v24 = a7;
        v22 = a8;
        v11 = a9;
        v3 = a10;
        v4 = a11;
      }
      v12 = __errno_location();
      if (*v12 != 4) {
        v14 = (char *)sub_14440(4,v14);
        v13 = dcgettext(NULL,"error reading %s",5);
        error(0,*v12,v13,v14);
        return 0;
      }
    } while( true );
  }
  if (!a9)
    return 1;
  do {
    while( true ) {
      v23 = 0x7fffffffc0000000;
      if (v27 <= 0x7fffffffc0000000)
        v23 = v27;
      v10 = copy_file_range(a0,NULL,a1,NULL,v23,0);
      if (!v10) {
        if (*a10)
          return 1;
        goto label_6b28;
      }
      if (v10 < 0) break;
      *a10 = *a10 + v10;
      v27 -= v10;
      if (!v27)
        return 1;
    }
    v12 = __errno_location();
    v8 = *v12;
    if (v8 == 0x26) goto label_6b28;
    if (0x1a < v8) {
      if (v8 == 0x5f) goto label_6b28;
      break;
    }
    if (1 <= v8) {
      if (1L << ((unsigned char)v8 & 0x3f) & 0x4440200U) goto label_6b28;
      if (v8 == 1) {
        if (!*a10) goto label_6b28;
        break;
      }
    }
  } while (v8 == 4);
  v13 = (char *)sub_14310(1,4,a8);
  v14 = (char *)sub_14310(0,4,v14);
  v15 = dcgettext(NULL,"error copying %s to %s",5);
  error(0,*v12,v15,v14,v13);
  return 0;
}


// Function: sub_72a0 @ 0x72a0
unsigned char sub_72a0(char *a0,char *a1,unsigned long a2,unsigned char a3,void *a4)
{
  unsigned char v1;
  long v2;
  int *v3; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6; // stack - 0x38
  
  v1 = *(unsigned char *)((long)a4 + 0x33);
  if (!v1) {
    v2 = *(long *)((long)a4 + 0x28);
    a3 = v2 != 0 & a3;
    if ((a3) && ((int)sub_c450(v2,a1,a2) <= -1)) {
      v3 = __errno_location();
      if ((*v3 != 0x5f) && (*v3 != 0x3d)) {
        v4 = (char *)sub_14440(4,a1);
        v5 = dcgettext(NULL,"failed to set default file creation context for %s",5);
        error(0,*v3,v5,v4);
        return a3;
      }
    }
    return 1;
  }
  if ((*(char *)((long)a4 + 0x31)) && (!*(char *)((long)a4 + 0x34))) {
    if (*(char *)((long)a4 + 0x37)) { // branch-flip
      if ((int)sub_15590(a0,&v6) <= -1) goto label_732e;
      if (0 <= setfscreatecon(v6)) {
        freecon(v6); // return-dupe
        return v1;
      }
    }
    else {
      if ((int)sub_15590(a0,&v6) <= -1) {
        v3 = __errno_location();
        if ((*v3 == 0x5f) || (*v3 == 0x3d)) goto label_732e;
        goto label_72f8;
      }
      if (0 <= setfscreatecon(v6)) {
        freecon(v6);
        return v1;
      }
      v3 = __errno_location();
      if ((*v3 != 0x5f) && (*v3 != 0x3d)) goto label_74ba;
    }
  }
  else {
    if ((int)sub_15590(a0,&v6) <= -1) {
      v3 = __errno_location();
label_72f8:
      v4 = (char *)sub_14440(4,a0);
      v5 = dcgettext(NULL,"failed to get security context of %s",5);
      error(0,*v3,v5,v4);
label_732e:
      if (!*(char *)((long)a4 + 0x34))
        return v1;
      return 0;
    }
    if (0 <= setfscreatecon(v6)) {
      freecon(v6);
      return v1;
    }
    v3 = __errno_location();
label_74ba:
    v4 = (char *)sub_14b30(v6);
    v5 = dcgettext(NULL,"failed to set default file creation context to %s",5);
    error(0,*v3,v5,v4);
  }
  if (*(char *)((long)a4 + 0x34)) {
    freecon(v6);
    return 0;
  }
  freecon(v6);
  return v1;
}


// Function: sub_7510 @ 0x7510
unsigned long sub_7510(char *a0,char a1,void *a2) // return-dupe
{
  long v1;
  unsigned int v2; // eax
  char *v3; // rax
  char *v4; // rax
  int *v5; // rax
  
  v1 = *(long *)((long)a2 + 0x28);
  if ((*(char *)((long)a2 + 0x31)) && (!*(char *)((long)a2 + 0x34))) {
    if (*(char *)((long)a2 + 0x37))
      return sub_c6f0(v1,a0,(unsigned char)a1); // tail-call
    v2 = sub_c6f0(v1,a0,(unsigned char)a1);
    if ((char)v2)
      return 1;
    v5 = __errno_location();
    if (*v5 == 0x5f)
      return (unsigned long)v2;
    if (*v5 == 0x3d)
      return (unsigned long)v2;
  }
  else {
    if (sub_c6f0(v1,a0,(unsigned char)a1))
      return 1;
    v5 = __errno_location();
  }
  v3 = (char *)sub_14310(0,4,a0);
  v4 = dcgettext(NULL,"failed to set the security context of %s",5);
  error(0,*v5,v4,v3);
  return 0;
}


// Function: sub_75f0 @ 0x75f0
void sub_75f0(struct_19 *a0)
{
  long v1; // rax
  
  v1 = sub_11bb0(0x3d,NULL,sub_12460,sub_12430,sub_124d0);
  a0->field_0x48 = v1;
  if (v1)
    return;
  sub_17150(); // no-return
}


// Function: sub_7630 @ 0x7630
void sub_7630(struct_20 *a0)
{
  long v1; // rax
  
  v1 = sub_11bb0(0x3d,NULL,sub_12410,sub_12430,sub_124d0);
  a0->field_0x50 = v1;
  if (v1)
    return;
  sub_17150(); // no-return
}


// Function: sub_7670 @ 0x7670
void sub_7670(unsigned long *a0)
{
  unsigned long *v1;
  unsigned int v2; // eax
  unsigned long v3; // rcx
  unsigned long *v4;
  
  *a0 = 0;
  a0[10] = 0;
  v3 = (unsigned long)(((int)a0 - (int)(unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8)) + 0x58U >> 3);
  v4 = (unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8);
  while (v3) {
    v1 = &v4[1];
    *v4 = 0;
    v3 -= 1;
    v4 = v1;
  }
  v2 = geteuid();
  *(unsigned int *)&a0[8] = 0xffffffff;
  *(bool *)((long)a0 + 0x1b) = v2 == 0;
  *(bool *)((long)a0 + 0x1a) = v2 == 0;
}


// Function: sub_76f0 @ 0x76f0
unsigned long sub_76f0(stat *a0,stat *a1,int a2,stat *a3,unsigned int a4,void **a5,unsigned long *a6,unsigned int *a7,unsigned int a8,void **a9,unsigned char *a10,char *a11)
{
  unsigned long v1;
  char v10;
  undefined8 v100; // stack - 0x228
  unsigned long v101; // stack - 0xa0
  long v102; // stack - 0x98
  undefined8 v103; // stack - 0x90
  undefined8 v104; // stack - 0x88
  undefined8 v105; // stack - 0x80
  undefined8 v106; // stack - 0x78
  undefined8 v107; // stack - 0x70
  undefined8 v108; // stack - 0x68
  undefined8 v109; // stack - 0x60
  unsigned char v11;
  undefined8 v110; // stack - 0x58
  undefined8 v111; // stack - 0x50
  char v12;
  int v13;
  unsigned int v14;
  unsigned int v15;
  char *v16;
  unsigned int *v17; // rax
  char *v18;
  unsigned long v19;
  unsigned long v2;
  char *v20; // rax
  unsigned long *v21;
  void *v22; // rax
  int *v23; // rax
  unsigned long v24;
  char *v25; // rax
  void **v26;
  stat *v27;
  unsigned int v28;
  undefined4 v29;
  unsigned long v3;
  void *v30;
  char v31 [8];
  char v32 [80]; // stack - 0x288
  char v33 [144]; // stack - 0x1f8
  char v34 [144]; // stack - 0x168
  char v35 [56]; // stack - 0xd8
  void *v36; // stack - 0x310
  char v37 [2]; // stack - 0x312
  long v38; // stack - 0x308
  void *v39; // stack - 0x2a8
  unsigned long v4;
  long v40;
  char *v41;
  char *v42;
  char *v43;
  char *v44;
  char *v45; // rsp
  char *v46; // rsp
  stat *v47;
  struct_6 *v48;
  unsigned long v49;
  unsigned long v5;
  char v50;
  unsigned long v51;
  unsigned long v52; // rbx
  unsigned int *v53;
  long v54;
  unsigned char v55;
  bool v56;
  char *v57; // stack - 0x3e0
  unsigned int v58; // stack - 0x3d8
  unsigned int v59; // stack - 0x3d4
  unsigned long v6;
  void **v60; // stack - 0x3d0
  unsigned int v61; // stack - 0x3c8
  unsigned int v62; // stack - 0x3c4
  unsigned int *v63; // stack - 0x3c0
  void **v64; // stack - 0x3b8
  char *v65; // stack - 0x3b0
  unsigned long *v66; // stack - 0x3a8
  unsigned int *v67; // stack - 0x3a0
  char *v68; // stack - 0x398
  void **v69; // stack - 0x390
  unsigned long v7;
  char *v70; // stack - 0x388
  stat *v71; // stack - 0x380
  void **v72; // stack - 0x378
  unsigned char *v73; // stack - 0x370
  unsigned long v74; // stack - 0x368
  unsigned long *v75; // stack - 0x360
  void **v76; // stack - 0x358
  stat *v77; // stack - 0x350
  char *v78; // stack - 0x348
  stat *v79; // stack - 0x340
  unsigned long v8;
  stat *v80; // stack - 0x338
  int v81; // stack - 0x330
  unsigned int v82; // stack - 0x32c
  stat *v83; // stack - 0x328
  stat *v84; // stack - 0x320
  unsigned long v85; // stack - 0x300
  unsigned long v86; // stack - 0x2f8
  unsigned long v87; // stack - 0x2f0
  unsigned long v88; // stack - 0x2e8
  unsigned long v89; // stack - 0x2e0
  unsigned char *v9;
  unsigned long v90; // stack - 0x2d8
  unsigned long v91; // stack - 0x2d0
  unsigned long v92; // stack - 0x2c8
  unsigned long v93; // stack - 0x2c0
  unsigned long v94; // stack - 0x2b8
  long v95; // stack - 0x2a0
  undefined8 v96; // stack - 0x298
  undefined8 v97; // stack - 0x290
  long v98; // stack - 0x238
  undefined8 v99; // stack - 0x230
  
  v53 = a7;
  v43 = v31;
  v44 = v31;
  v46 = v31;
  v41 = v31;
  v80 = a1;
  v79 = a3;
  v75 = a6;
  v81 = a2;
  v72 = a5;
  v83 = (stat *)CONCAT44(v83._4_4_,a8);
  v76 = a9;
  v28 = a7[0x10];
  v73 = a10;
  v78 = a11;
  v74 = CONCAT71(v74._1_7_,(char)a8);
  *a10 = 0;
  v84 = (stat *)CONCAT71(v84._1_7_,(char)a7[6]);
  if ((char)a7[6]) {
    if ((int)v28 < 0) {
      if (!sub_14b50(0xffffff9c,a0,a2,a3,1)) {
        a4 = 1;
        *v78 = 1;
        goto label_79ae;
      }
      v28 = *__errno_location();
    }
    v56 = v28 == 0;
    v84 = (stat *)CONCAT71(v84._1_7_,v56);
    a4 = (unsigned int)v56;
    *v78 = v56;
    if (!v28) goto label_79ae;
label_779d:
    if ((v28 != 0x11) || (v53[2] != 2)) {
      v47 = a0;
      v27 = a0;
      v13 = -100;
      v82 = a4;
      goto label_77b2;
    }
label_7803:
    if (((char)v83) && (v48 = *(struct_6 **)&v53[0x14], v48)) {
      v27 = (stat *)v32;
      if (((v82 & 0xf000) != 0x4000) && (!*v53)) {
        v71 = (stat *)CONCAT44(v71._4_4_,a4);
        v77 = (stat *)v32;
        v15 = sub_e820(v48,a0,(stat *)v32);
        v51 = (unsigned long)v15;
        if ((char)v15) {
          v19 = sub_14440(4,a0);
          v16 = "warning: source file %s specified more than once";
          v16 = dcgettext(NULL,v16,5); // return-dupe
          error(0,0,v16,v19); // return-dupe
          return v51 & 0xffffffff; // return-dupe
        }
        v48 = *(struct_6 **)&v53[0x14];
        v27 = v77;
        a4 = (unsigned int)v71;
      }
      v77 = (stat *)CONCAT44(v77._4_4_,a4);
      sub_e790(v48,a0,v27);
      a4 = (unsigned int)v77;
    }
    if (v53[1] == 4) {
      v71 = (stat *)CONCAT44(v71._4_4_,1);
label_787c:
      v77 = NULL;
      v55 = 0;
      v44 = v31;
      if (1 <= (int)a4) goto label_7892;
label_7b88:
      if ((v28 == 0x11) && (v53[2] == 2)) {
        v55 = 0;
        v10 = 0;
label_8af3:
        if (!*(char *)((long)v53 + 0x3b)) {
          if ((char)v53[6]) goto label_910b;
          if ((v82 & 0xf000) != 0x4000) {
label_8c1d:
            if ((v53[2] == 2) || ((v53[2] == 3 && (v12 = sub_67a0(v53,v80,v81,v79,v33), !v12)))) {
              v51 = 1; // return-dupe
              return v51 & 0xffffffff;
            }
          }
          goto label_8c33;
        }
        if ((v82 & 0xf000) != 0x4000) { // branch-flip
          v56 = 0;
          if ((*(char *)((long)v53 + 0x1f)) && (v56 = 1, (char)v53[6]))
            v56 = (void *)v33._0_8_ != v32._0_8_;
          v13 = sub_15b50(v81,v79,v33,v32,v56);
          if (v13 < 0) {
            if (!(char)v53[6]) goto label_8c1d;
            goto label_910b;
          }
          if (v78)
            *v78 = 1;
          v16 = (char *)sub_bd30(v79,v32._8_8_,v32._0_8_);
          if ((v16) && (v10 = sub_65d0(NULL,v81,v16,v80,v81,v79,1,(unsigned char)*(char *)&v53[0xf],(unsigned long)v71 & 0xffffffff), !v10)) {
            if (*(char *)((long)v53 + 0x33)) {
              v44 = v31;
              if (setfscreatecon(NULL)) goto label_8bf7;
            }
            v51 = 0; // return-dupe
            return v51 & 0xffffffff;
          }
        }
        else {
          if (!(char)v53[6]) goto label_8c33;
label_910b:
          v28 = v53[2];
          if (v28 != 2) {
            if (v28 != 3) {
              if ((v28 != 4) || (!*(char *)((long)v53 + 0x3d))) {
label_8c33:
                if (v10) {
                  v51 = 1;
                  return v51 & 0xffffffff;
                }
                if ((v33._24_4_ & 0xf000) != 0x4000) goto label_8c56;
                if ((v82 & 0xf000) != 0x4000) {
label_9818:
                  if ((char)v53[6]) {
                    v51 = (unsigned long)*v53;
                    v28 = v33._24_4_;
                    if (*v53) goto label_982f;
                  }
                  v16 = (char *)sub_14440(4,v80);
                  v51 = 0;
                  error(0,0,dcgettext(NULL,"cannot overwrite directory %s with non-directory",5),v16);
                  return v51 & 0xffffffff;
                }
label_8cc9:
                v51 = (unsigned long)*v53;
label_8ccc:
                if ((char)v53[6]) { // branch-flip
                  if ((v32._24_4_ & 0xf000) == 0x4000) {
                    v28 = v33._24_4_;
                    goto label_a0b6;
                  }
label_9e4c:
                  if ((int)v51) goto label_9847;
label_8d80:
                  v77 = NULL;
                  v28 = 0x11;
                  v44 = v31;
                }
                else {
                  v11 = 0;
                  if ((int)v51) goto label_9e1b;
label_8ce2:
                  v28 = 0x11;
                  v77 = NULL;
                  v11 = (v33._24_4_ & 0xf000) == 0x4000 | v11;
                  v51 = CONCAT71((undefined7)(v51 >> 8),v11);
                  v44 = v31;
                  if (!v11) {
                    if (!*(char *)((long)v53 + 0x15)) {
                      v44 = v31;
                      if (!*(char *)((long)v53 + 0x31)) goto label_7892;
                      if ((!(char)v53[0xc]) || ((unsigned long)v33._16_8_ <= 1)) {
                        v28 = 0x11;
                        v77 = NULL;
                        v44 = v31;
                        if ((v53[1] != 2) || (v44 = v31, (v32._24_4_ & 0xf000) == 0x8000)) goto label_7892;
                      }
                    }
                    v77 = NULL;
                    if ((unlinkat(v81,(char *)v79,0)) && (v23 = __errno_location(), *v23 != 2)) {
                      v16 = (char *)sub_14440(4,v80);
                      v18 = dcgettext(NULL,"cannot remove %s",5);
                      error(0,*v23,v18,v16);
                      return v51 & 0xffffffff;
                    }
                    v84 = (stat *)CONCAT71(v84._1_7_,(char)v53[0xf]);
                    if (!(char)v53[0xf]) {
                      v84 = (stat *)CONCAT71(v84._1_7_,1);
                      goto label_8d80;
                    }
                    v28 = 0x11;
                    v16 = (char *)sub_14440(4,v80);
                    __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v16);
                    v77 = NULL;
                    v44 = v31;
                  }
                }
              }
              else {
                if ((v33._24_4_ & 0xf000) != 0xa000) {
                  if ((!sub_16bb0()) && (faccessat(v81,(char *)v79,2,0x200))) goto label_917c;
                  goto label_8c33;
                }
                if (v10) {
                  v51 = 1;
                  return v51 & 0xffffffff;
                }
label_8c56:
                v15 = v82 & 0xf000;
                v28 = v33._24_4_;
                if (v15 != 0x4000) { // branch-flip
                  v51 = (unsigned long)*v53;
                  if (!(char)v83) goto label_8ccc;
label_8c7d:
                  if ((int)v51 != 3) {
                    v10 = sub_e820(*(struct_6 **)&v53[0x12],v79,v33);
                    if (v10) {
                      v19 = sub_14310(1,4,a0);
                      v24 = sub_14310(0,4,v80);
                      v16 = "will not overwrite just-created %s with %s";
                      error(0,0,dcgettext(NULL,v16,5),v24,v19); // return-dupe
                      v51 = 0;
                      return v51 & 0xffffffff;
                    }
                    if ((v15 == 0x4000) || ((v33._24_4_ & 0xf000) != 0x4000)) goto label_8cc9;
                    goto label_9818;
                  }
                  if ((char)v53[6]) goto label_982f;
label_9e1b:
                  v13 = (int)v51;
                  v16 = (char *)sub_d970(a0);
                  if (*v16 == '.') {
                    v11 = 0;
                    goto label_985e;
                  }
label_9883:
                  if ((v33._24_4_ & 0xf000) == 0x4000) goto label_8d80;
                }
                else {
                  if ((!(char)v53[6]) || (v51 = (unsigned long)*v53, !*v53)) {
                    v19 = sub_14310(1,4,a0);
                    v24 = sub_14310(0,4,v80);
                    v16 = "cannot overwrite non-directory %s with directory %s";
                    error(0,0,dcgettext(NULL,v16,5),v24,v19);
                    v51 = 0;
                    return v51 & 0xffffffff;
                  }
                  if ((char)v83) goto label_8c7d;
label_982f:
                  if ((v32._24_4_ & 0xf000) == 0x4000) {
label_a0b6:
                    if ((v28 & 0xf000) == 0x4000) goto label_9e4c;
                    if (!(int)v51) {
                      v19 = sub_147c0(0,3,v80);
                      v24 = sub_147c0(0,3,a0);
                      v16 = "cannot move directory onto non-directory: %s -> %s";
                      error(0,0,dcgettext(NULL,v16,5),v24,v19);
                      v51 = 0;
                      return v51 & 0xffffffff;
                    }
                  }
label_9847:
                  v13 = (int)v51;
                  v16 = (char *)sub_d970(a0);
                  v11 = 1;
                  if (*v16 == '.') {
label_985e:
                    v13 = (int)v51;
                    if ((!v16[(unsigned long)(v16[1] == '.') + 1]) || (v16[(unsigned long)(v16[1] == '.') + 1] == '/')) goto label_8ce2;
                    if (!v11) goto label_9883;
                  }
                }
                if (v13 != 3) {
                  v84 = (stat *)strlen(v16);
                  v70 = (char *)sub_d970(v79);
                  v77 = (stat *)strlen(v70);
                  v18 = dat_23580;
                  v51 = strlen(dat_23580);
                  if (v84 == (stat *)&(&v77->field_0x0)[v51]) {
                    v84 = v77;
                    v13 = memcmp(v16,v70,(unsigned long)v77);
                    if (!v13) {
                      v13 = strcmp(&v16[(long)v84],v18);
                      v27 = v79;
                      if (!v13) {
                        v16 = (char *)sub_6110(v27,&(&v27->field_0x0)[strlen((char *)v79)],v18);
                        v13 = fstatat(v81,v16,(stat *)v35,0);
                        free(v16);
                        if (((!v13) && (v32._8_8_ == v35._8_8_)) && (v32._0_8_ == v35._0_8_)) {
                          if ((char)v53[6]) // branch-flip
                            v16 = dcgettext(NULL,"backing up %s might destroy source;  %s not moved",5);
                          else {
                            v16 = dcgettext(NULL,"backing up %s might destroy source;  %s not copied",5);
                          }
                          v19 = sub_14310(1,4,a0);
                          v51 = 0;
                          error(0,0,v16,sub_14310(0,4,v80),v19);
                          return v51 & 0xffffffff;
                        }
                      }
                    }
                  }
                }
                v27 = v79;
                v16 = (char *)sub_d840(v81,v79,*v53);
                if (v16) { // branch-flip
                  v52 = (long)v27 - (long)v80;
                  v51 = strlen(v16);
                  v84 = (unsigned long)(v51 + 1);
                  v51 = v52 + 0x18 + v51;
                  v44 = v31;
                  while (v46 != &v31[-(v51 & 0xfffffffffffff000)]) {
                    v41 = &v44[-0x1000];
                    v45 = &v44[-0x1000];
                    *(unsigned long *)&v44[-8] = *(unsigned long *)&v44[-8];
                    v46 = &v44[-0x1000];
                    v44 = v45;
                  }
                  v51 = (unsigned long)((unsigned int)v51 & 0xff0);
                  v54 = -v51;
                  if (v51)
                    *(unsigned long *)&v41[-8] = *(unsigned long *)&v41[-8];
                  v77 = (stat *)((unsigned long)&v41[v54 + 0xf] & 0xfffffffffffffff0);
                  v28 = 0x11;
                  *(unsigned long *)&v41[v54 + -8] = 0x9a35;
                  v30 = mempcpy(v77,v80,v52);
                  *(unsigned long *)&v41[v54 + -8] = 0x9a47;
                  memcpy(v30,v16,(unsigned long)v84);
                  *(unsigned long *)&v41[v54 + -8] = 0x9a4f;
                  free(v16);
                  v84 = (stat *)CONCAT71(v84._1_7_,1);
                  v44 = &v41[v54];
                }
                else {
                  v23 = __errno_location();
                  if (*v23 != 2) {
                    v16 = (char *)sub_14440(4,v80);
                    v18 = dcgettext(NULL,"cannot backup %s",5);
                    v51 = 0;
                    error(0,*v23,v18,v16);
                    return v51 & 0xffffffff;
                  }
                  v84 = (stat *)CONCAT71(v84._1_7_,1);
                  v28 = 0x11;
                  v77 = NULL;
                  v44 = v31;
                }
              }
label_7892:
              if ((char)v83) goto label_7ad0;
              goto label_789f;
            }
label_917c:
            v12 = sub_67a0(v53,v80,v81,v79,v33);
            if (v12) goto label_8c33;
          }
label_91b0:
          if (v78)
            *v78 = 1;
        }
        v51 = 1;
        return v51 & 0xffffffff;
      }
      v15 = v82 & 0xf000;
      if ((((v15 != 0x8000) && (v11 = (v15 == 0xa000 || v15 == 0x4000) | (unsigned char)v53[5] ^ 1, v11)) || (v11 = (unsigned char)v53[6], v11)) || ((v11 = *(unsigned char *)((long)v53 + 0x3a), v11 || (v11 = *(unsigned char *)((long)v53 + 0x17), v11)))) {
label_7bca:
        v13 = 0x100;
label_7bd0:
        if (fstatat(v81,(char *)v79,(stat *)v33,v13)) {
          v23 = __errno_location();
          if (*v23 != 0x28) { // branch-flip
            if (*v23 == 2) goto label_8460;
          }
          else if (*(char *)((long)v53 + 0x16)) goto label_8467;
          v16 = (char *)sub_14440(4,v80);
          v18 = dcgettext(NULL,"cannot stat %s",5);
          v51 = 0;
          error(0,*v23,v18,v16);
          return v51 & 0xffffffff;
        }
      }
      else {
        if (*v53) {
          v11 = 1;
          goto label_7bca;
        }
        v11 = *(unsigned char *)((long)v53 + 0x15);
        if (v11) goto label_7bca;
        v13 = 0;
        if (!a4) goto label_7bd0;
label_8460:
        v84 = (stat *)CONCAT71(v84._1_7_,1);
label_8467:
        v77 = NULL;
        v55 = 0;
        v44 = v31;
        v11 = 0;
        if (v28 != 0x11) goto label_7892;
      }
      v55 = v11;
      if (v53[2] == 2) goto label_8af0;
      if ((v32._8_8_ == v33._8_8_) && (v32._0_8_ == (void *)v33._0_8_)) {
        v10 = *(char *)((long)v53 + 0x17);
        if (!v10) {
          if (v53[1] == 2) {
            v56 = 1;
            v29 = 1;
            goto label_7c23;
          }
          v27 = (stat *)v34;
          if (!fstatat(v81,(char *)v79,v27,0x100)) {
            v77 = (stat *)v35;
            v13 = lstat((char *)a0,v77);
            if (!v13) {
              v56 = v35._8_8_ == v34._8_8_ && v35._0_8_ == v34._0_8_;
              v47 = v77;
              if ((((v35._24_4_ & 0xf000) != 0xa000) || ((v34._24_4_ & 0xf000) != 0xa000)) || (!*(char *)((long)v53 + 0x15))) goto label_7c47;
            }
          }
        }
        goto label_8af3;
      }
      if (v53[1] != 2) goto label_8af0;
      v56 = 0;
      v29 = 0;
label_7c23:
      v27 = (stat *)v33;
      v47 = (stat *)v32;
      if (((v32._24_4_ & 0xf000) != 0xa000) || ((v33._24_4_ & 0xf000) != 0xa000)) {
label_7c47:
        if (*v53) { // branch-flip
          if (!v56) {
            if (((((char)v53[6]) || (v53[1] == 2)) || ((*(unsigned int *)&v47->field_0x18 & 0xf000) != 0xa000)) || ((*(unsigned int *)&v27->field_0x18 & 0xf000) == 0xa000)) goto label_8af0;
            goto label_7c99;
          }
          v10 = 0;
          v11 = sub_14e20(0xffffff9c,a0,v81,v79);
          v11 ^= 1;
label_a173:
          if (v11) goto label_8af3;
        }
        else {
          v10 = (char)v53[6];
          if ((v10) || (*(char *)((long)v53 + 0x15))) {
            if ((*(unsigned int *)&v27->field_0x18 & 0xf000) == 0xa000) goto label_8af0;
            if ((v56) && (2 <= *(unsigned long *)&v27->field_0x10)) {
              v77 = v47;
              v10 = sub_14e20(0xffffff9c,a0,v81,v79);
              if (v10) {
                v47 = v77;
                if ((*(unsigned int *)&v77->field_0x18 & 0xf000) != 0xa000) goto label_9bb4;
                goto label_9bc9;
              }
              v11 = (unsigned char)v53[6] ^ 1;
              v10 = 0;
              goto label_a173;
            }
            if ((*(unsigned int *)&v47->field_0x18 & 0xf000) != 0xa000) goto label_9e99;
label_9bce:
            if (((v10) && ((v32._24_4_ & 0xf000) == 0xa000)) && (2 <= *(unsigned long *)&v27->field_0x10)) {
              v77 = v47;
              v16 = canonicalize_file_name((char *)a0);
              v47 = v77;
              if (v16) {
                v11 = sub_14e20(0xffffff9c,v16,v81,v79);
                v10 = 0;
                v11 ^= 1;
                free(v16);
                goto label_a173;
              }
            }
          }
          else if ((*(unsigned int *)&v47->field_0x18 & 0xf000) != 0xa000) {
label_9bb4:
            if ((*(unsigned int *)&v27->field_0x18 & 0xf000) != 0xa000) {
label_9e99:
              if ((*(long *)&v47->field_0x8 != *(long *)&v27->field_0x8) || (*(long *)v47 != *(long *)v27)) goto label_8af0;
              v10 = *(char *)((long)v53 + 0x17);
              if (v10) goto label_8af3;
            }
label_9bc9:
            v10 = (char)v53[6];
            goto label_9bce;
          }
          if ((*(char *)((long)v53 + 0x3a)) && ((*(unsigned int *)&v27->field_0x18 & 0xf000) == 0xa000)) {
label_8af0:
            v10 = 0;
            goto label_8af3;
          }
          if (v53[1] == 2) {
            if ((*(unsigned int *)&v47->field_0x18 & 0xf000) != 0xa000) { // branch-flip
              v35._0_8_ = *(void **)v47;
              v35._8_8_ = *(long *)&v47->field_0x8;
              v35._16_8_ = *(unsigned long *)&v47->field_0x10;
              v35._24_8_ = *(unsigned long *)&v47->field_0x18;
              v35._32_8_ = *(unsigned long *)&v47->field_0x20;
              v35._40_8_ = *(unsigned long *)&v47->field_0x28;
              v35._48_8_ = *(void ***)&v47->field_0x30;
              v101 = *(unsigned long *)&v47->field_0x38;
              v102 = *(long *)&v47->field_0x40;
              v103 = *(unsigned long *)&v47->field_0x48;
              v104 = *(unsigned long *)&v47->field_0x50;
              v105 = *(unsigned long *)&v47->field_0x58;
              v106 = *(unsigned long *)&v47->field_0x60;
              v107 = *(unsigned long *)&v47->field_0x68;
              v108 = *(unsigned long *)&v47->field_0x70;
              v109 = *(unsigned long *)&v47->field_0x78;
              v110 = *(unsigned long *)&v47->field_0x80;
              v111 = *(unsigned long *)&v47->field_0x88;
            }
            else if (stat((char *)a0,(stat *)v35)) goto label_8af0;
            if ((*(unsigned int *)&v27->field_0x18 & 0xf000) != 0xa000) { // branch-flip
              v34._0_8_ = *(void **)v27;
              v34._8_8_ = *(long *)&v27->field_0x8;
              v34._16_8_ = *(unsigned long *)&v27->field_0x10;
              v34._24_8_ = *(unsigned long *)&v27->field_0x18;
              v34._32_8_ = *(unsigned long *)&v27->field_0x20;
              v34._40_8_ = *(unsigned long *)&v27->field_0x28;
              v34._48_8_ = *(unsigned long *)&v27->field_0x30;
              v34._56_8_ = *(void ***)&v27->field_0x38;
              v34._64_8_ = *(unsigned long *)&v27->field_0x40;
              v34._72_8_ = *(unsigned long *)&v27->field_0x48;
              v34._80_8_ = *(unsigned long *)&v27->field_0x50;
              v34._88_8_ = *(unsigned long *)&v27->field_0x58;
              v34._96_8_ = *(unsigned long *)&v27->field_0x60;
              v34._104_8_ = *(unsigned long *)&v27->field_0x68;
              v34._112_8_ = *(unsigned long *)&v27->field_0x70;
              v34._120_8_ = *(unsigned long *)&v27->field_0x78;
              v34._128_8_ = *(unsigned long *)&v27->field_0x80;
              v34._136_8_ = *(unsigned long *)&v27->field_0x88;
            }
            else if (fstatat(v81,(char *)v79,(stat *)v34,0)) goto label_8af0;
            if ((v35._8_8_ != v34._8_8_) || (v35._0_8_ != v34._0_8_)) goto label_8af0;
            if (*(char *)((long)v53 + 0x17)) {
              v10 = (*(unsigned int *)&v27->field_0x18 & 0xf000) != 0xa000;
              goto label_8af3;
            }
          }
        }
      }
      else {
        v77 = (stat *)CONCAT44(v77._4_4_,v29);
        v10 = sub_14e20(0xffffff9c,a0,v81,v79);
        if (!v10) {
          v10 = '\0';
          if ((!*v53) && ((int)v77)) {
            v10 = 1;
            v11 = (unsigned char)v53[6] ^ 1;
            goto label_a173;
          }
          goto label_8af3;
        }
      }
label_7c99:
      v19 = sub_14310(1,4,v80);
      v24 = sub_14310(0,4,a0);
      v16 = "%s and %s are the same file";
      error(0,0,dcgettext(NULL,v16,5),v24,v19);
      v51 = 0;
      return v51 & 0xffffffff;
    }
    if ((v53[1] != 3) || (!(char)v83)) {
      v71 = (stat *)((unsigned long)v71 & 0xffffffff00000000);
      goto label_787c;
    }
    v71 = (stat *)CONCAT44(v71._4_4_,1);
    if ((int)a4 <= 0) goto label_7b88;
    v77 = NULL;
    v55 = 0;
label_7ad0:
    if ((!*(long *)&v53[0x12]) || ((char)v53[6])) {
label_789f:
      if (((char)v53[0xf]) && (!(char)v53[6])) {
label_8258:
        if ((v82 & 0xf000) != 0x4000) {
          *(unsigned long *)&v44[-8] = 0x8284;
          sub_6510(a0,v80,v77);
        }
      }
      if (v28) goto label_78ba;
      v41 = v44;
      if ((char)v53[6]) goto label_7d27;
label_7d98:
      v52 = (unsigned long)v84 & 0xff;
      v16 = NULL;
label_7da3:
      v28 = v82;
      if (*(char *)((long)v53 + 0x39))
        v28 = v53[4];
      v44 = v41;
      if (*(char *)((long)v53 + 0x1d)) {
        *(unsigned long *)&v41[-8] = 0x7e4b;
        v10 = sub_72a0(a0,v80,v82,v52,v53);
        if (!v10) {
          v51 = 0;
          return v51 & 0xffffffff;
        }
        v70 = (char *)(CONCAT44(v70._4_4_,v28) & 0xffffffff0000003f);
        v78 = (char *)(CONCAT44(v78._4_4_,v82) & 0xffffffff0000f000);
        v21 = v75;
        if ((v82 & 0xf000) == 0x4000) goto label_8356;
label_7e7a:
        v15 = (unsigned int)*(unsigned char *)((long)v53 + 0x3a);
        if (!*(unsigned char *)((long)v53 + 0x3a)) {
          if (!*(char *)((long)v53 + 0x17)) {
            if (((unsigned int)v78 != 0x8000) && (!((unsigned int)v78 != 0xa000 & (unsigned char)v53[5]))) {
              v76 = (void **)((unsigned long)v76._1_7_ << 8);
              if ((unsigned int)v78 == 0x1000) {
                v28 = ~(unsigned int)v70 & v82;
                *(unsigned long *)&v41[-8] = 0xa2d6;
                v13 = mknodat(v81,v79,v28,0);
                v10 = (char)v76;
                if (v13) {
                  *(unsigned long *)&v41[-8] = 0xa2fd;
                  v13 = mkfifoat(v81,(char *)v79,v28 & 0xffffefff);
                  v10 = (char)v76;
                  if (v13) {
                    *(unsigned long *)&v41[-8] = 0xa31e;
                    v19 = sub_14440(4,v80);
                    v18 = "cannot create fifo %s";
                    goto label_8638;
                  }
                }
                goto label_7f17;
              }
              if (((unsigned int)v78 & 0xffffbfff) == 0x2000 || (unsigned int)v78 == 0xc000) {
                *(unsigned long *)&v41[-8] = 0x8601;
                v13 = mknodat(v81,v79,~(unsigned int)v70 & v82,v32._40_8_);
                v10 = (char)v76;
                if (v13) {
                  *(unsigned long *)&v41[-8] = 0x8622;
                  v19 = sub_14440(4,v80);
                  v18 = "cannot create special file %s";
                  goto label_8638;
                }
                goto label_7f17;
              }
              if ((unsigned int)v78 == 0xa000) {
                *(unsigned long *)&v41[-8] = 0x9f44;
                v18 = (char *)sub_c960(a0,v32._48_8_);
                if (!v18) {
                  *(unsigned long *)&v41[-8] = 0xae5a;
                  v19 = sub_14440(4,a0);
                  v18 = "cannot read symbolic link %s";
                  goto label_8638;
                }
                v50 = *(char *)((long)v53 + 0x16);
                *(unsigned long *)&v41[-8] = 0x9f70;
                v13 = sub_c000(v18,v81,v79,(unsigned char)v50,0xffffffff);
                if (v13 <= 0) {
                  *(unsigned long *)&v41[-8] = 0xa90a;
                  free(v18);
label_a90a:
                  if (*(char *)((long)v53 + 0x33)) {
                    *(unsigned long *)&v41[-8] = 0xae40;
                    if (setfscreatecon(NULL)) goto label_8bf7;
                  }
                  v15 = 1;
                  v10 = '\0';
                  if (*(char *)((long)v53 + 0x1d)) {
                    v76 = (void **)CONCAT71(v76._1_7_,*(char *)((long)v53 + 0x1d));
                    *(unsigned long *)&v41[-8] = 0xa954;
                    v13 = fchownat(v81,(char *)v79,v32._28_4_,v32._32_4_,0x100);
                    if (v13) { // branch-flip
                      *(unsigned long *)&v41[-8] = 0xa96c;
                      v15 = sub_76c0(v53);
                      v10 = '\0';
                      if (!(char)v15) {
                        *(unsigned long *)&v41[-8] = 0xa98d;
                        v18 = dcgettext(NULL,"failed to preserve ownership for %s",5);
                        *(unsigned long *)&v41[-8] = 0xa995;
                        v13 = *__errno_location();
                        *(unsigned long *)&v41[-8] = 0xa9aa;
                        error(0,v13,v18,v80);
                        if (*(char *)((long)v53 + 0x32)) goto label_8658;
                        v15 = (unsigned int)(unsigned char)v76;
                        v10 = '\0';
                      }
                    }
                    else {
                      v15 = (unsigned int)(unsigned char)v76;
                      v10 = '\0';
                    }
                  }
                  goto label_7f17;
                }
                if (((*(char *)((long)v53 + 0x3b) == '\x01') && (!(char)v84)) && ((v33._24_4_ & 0xf000) == 0xa000)) {
                  v76 = (void **)v33._48_8_;
                  *(unsigned long *)&v41[-8] = 0xafb5;
                  if (v76 == (void **)strlen(v18)) {
                    *(unsigned long *)&v41[-8] = 0xafd7;
                    v26 = (void **)sub_caf0(v81,v79);
                    if (v26) {
                      v76 = v26;
                      *(unsigned long *)&v41[-8] = 0xaff2;
                      if (!strcmp((char *)v26,v18)) {
                        *(unsigned long *)&v41[-8] = 0xb006;
                        free(v76);
                        *(unsigned long *)&v41[-8] = 0xb00e;
                        free(v18);
                        goto label_a90a;
                      }
                      *(unsigned long *)&v41[-8] = 0xba71;
                      free(v76);
                    }
                  }
                }
                *(unsigned long *)&v41[-8] = 0x9f97;
                free(v18);
                *(unsigned long *)&v41[-8] = 0x9fa8;
                v18 = (char *)sub_14440(4,v80);
                *(unsigned long *)&v41[-8] = 0x9fbe;
                v20 = dcgettext(NULL,"cannot create symbolic link %s",5);
                *(unsigned long *)&v41[-8] = 0x9fd0;
                error(0,v13,v20,v18);
                goto label_8658;
              }
              *(unsigned long *)&v41[-8] = 0xa42d;
              v19 = sub_14440(4,a0);
              v18 = "%s has unknown file type";
label_8394:
              *(unsigned long *)&v41[-8] = 0x839b;
              v18 = dcgettext(NULL,v18,5);
              *(unsigned long *)&v41[-8] = 0x83ac;
              error(0,0,v18,v19);
              goto label_8658;
            }
            v15 = v53[1];
            v36 = NULL;
            v73 = (unsigned char *)CONCAT44(v73._4_4_,v32._24_4_);
            v76 = (void **)CONCAT71(v76._1_7_,*(char *)((long)v53 + 0x31));
            v75 = (unsigned long *)CONCAT71(v75._1_7_,*(char *)((long)v53 + 0x35));
            *(unsigned long *)&v41[-8] = 0x946d;
            v15 = sub_e620(a0,(unsigned long)(v15 == 2) << 0x11);
            v51 = (unsigned long)v15;
            if ((int)v15 < 0) {
              *(unsigned long *)&v41[-8] = 0xa25e;
              v19 = sub_14440(4,a0);
              v18 = "cannot open %s for reading";
              goto label_8638;
            }
            *(unsigned long *)&v41[-8] = 0x9486;
            if (fstat(v15,(stat *)v35)) { // branch-flip
              *(unsigned long *)&v41[-8] = 0xa4a8;
              v76 = (void **)sub_14440(4,a0);
              *(unsigned long *)&v41[-8] = 0xa4c2;
              v18 = dcgettext(NULL,"cannot fstat %s",5);
              *(unsigned long *)&v41[-8] = 0xa4ca;
              v13 = *__errno_location();
              v26 = v76;
label_a000:
              v30 = NULL;
              *(unsigned long *)&v41[-8] = 0xa00b;
              error(0,v13,v18,v26);
              v50 = 0;
            }
            else {
              if ((v32._8_8_ != v35._8_8_) || (v32._0_8_ != v35._0_8_)) {
                *(unsigned long *)&v41[-8] = 0x9fe2;
                v26 = (void **)sub_14440(4,a0);
                *(unsigned long *)&v41[-8] = 0x9ff8;
                v18 = dcgettext(NULL,"skipping file %s, as it was replaced while being copied",5);
                v13 = 0;
                goto label_a000;
              }
              v65 = (char *)(CONCAT44(v65._4_4_,v28) & 0xffffffff000001ff);
              if ((char)v84) goto label_a500;
              *(unsigned long *)&v41[-8] = 0x94f8;
              v28 = sub_125a0(v81,v79,(-(unsigned int)((char)v76 == '\0') & 0xfffffe00) + 0x201);
              v52 = (unsigned long)v28;
              *(unsigned long *)&v41[-8] = 0x94ff;
              v26 = (void **)__errno_location();
              v13 = *(int *)v26;
              if (0 <= (int)v28) {
                if ((!*(long *)&v53[10]) && (!*(char *)((long)v53 + 0x33))) {
label_953b:
                  v58 = 0;
                  v63 = (unsigned int *)((unsigned long)v63 & 0xffffffff00000000);
label_954f:
                  v28 = (unsigned int)v52;
                  if ((char)v76) {
                    if (v53[0x11]) {
                      *(unsigned long *)&v41[-8] = 0xaeee;
                      if (!ioctl(v28,0x40049409,v51)) goto label_a64a;
                      if (v53[0x11] == 2) {
                        *(unsigned long *)&v41[-8] = 0xaf13;
                        v73 = (unsigned char *)sub_14310(1,4,a0);
                        *(unsigned long *)&v41[-8] = 0xaf2d;
                        v75 = (unsigned long *)sub_14310(0,4,v80);
                        *(unsigned long *)&v41[-8] = 0xaf47;
                        v76 = (void **)dcgettext(NULL,"failed to clone %s from %s",5);
                        *(unsigned long *)&v41[-8] = 0xaf53;
                        v13 = *__errno_location();
                        *(unsigned long *)&v41[-8] = 0xaf73;
                        error(0,v13,(char *)v76,v75,v73);
                        v55 = 0;
                        goto label_96e5;
                      }
                    }
label_9568:
                    *(unsigned long *)&v41[-8] = 0x9576;
                    if (fstat(v28,(stat *)v34)) {
                      *(unsigned long *)&v41[-8] = 0xb1d5;
                      v18 = "cannot fstat %s";
                      v75 = (unsigned long *)sub_14440(4,v80);
                      goto label_b133;
                    }
                    v72 = (void **)((unsigned long)v72 & 0xffffffff00000000);
                    if (v34._24_4_ != ((unsigned int)v63 | v34._24_4_)) {
                      *(unsigned long *)&v41[-8] = 0x959d;
                      v13 = fchmod(v28,(unsigned int)v63 | v34._24_4_);
                      v29 = v72._0_4_;
                      if (!v13)
                        v29 = v63._0_4_;
                      v63 = (unsigned int *)CONCAT44(v63._4_4_,v29);
                    }
                    if (!(char)v76) goto label_ac58;
                    v60 = v34._56_8_;
                    v69 = v34._56_8_;
                    if ((void **)0x1ffffffffffe0001 <= (void **)&((undefined8)v34._56_8_)[-0x4000]) {
                      v69 = (void **)0x20000;
                      v60 = (void **)0x200;
                      if ((char *)((long)v34._56_8_ + -1) <= (char *)0x1fffffffffffffff)
                        v60 = v34._56_8_;
                    }
                    if ((v35._24_4_ & 0xf000) != 0x8000) {
                      v13 = 1;
label_ab10:
                      if (((v34._24_4_ & 0xf000) != 0x8000) || ((v53[3] != 3 && ((v13 == 1 || (v53[3] != 2)))))) {
                        v72 = (void **)CONCAT44(v72._4_4_,v13);
                        *(unsigned long *)&v41[-8] = 0xab3e;
                        sub_e5e0(v51,0,0,2);
                        v49 = v101;
                        if (0x1ffffffffffe0001 <= v101 - 0x20000)
                          v49 = 0x20000;
                        *(unsigned long *)&v41[-8] = 0xab78;
                        v26 = (void **)sub_da00(v49,v69,0x7fffffffffffffff);
                        if (((v35._24_4_ & 0xf000) == 0x8000) && (v35._48_8_ < v69))
                          v69 = (void **)((long)v35._48_8_ + 1);
                        v18 = &((char *)((long)v26 + -1))[(long)v69];
                        v69 = v26;
                        if (1 <= (long)&v18[-((unsigned long)v18 % (unsigned long)v26)])
                          v69 = (void **)&v18[-((unsigned long)v18 % (unsigned long)v26)];
                        v59 = (unsigned int)(v53[0x11] != 0);
                        if ((int)v72 == 3) {
                          v61 = 1;
label_b1f7:
                          v37[0] = 0;
                          v72 = v35._48_8_;
                          if (0 <= (long)v68) {
                            v64 = NULL;
                            v57 = NULL;
                            v66 = (unsigned long *)CONCAT71(v66._1_7_,v76._0_1_);
                            v54 = 0;
                            v67 = v53;
                            v18 = v68;
                            v68 = v16;
                            v71 = a0;
                            do {
                              v20 = v18;
                              *(unsigned long *)&v41[-8] = 0xb266;
                              v26 = (void **)lseek(v51,v20,4);
                              if ((long)v26 <= -1) {
                                *(unsigned long *)&v41[-8] = 0xb277;
                                if (*__errno_location() != 6) goto label_b50c;
                                v26 = v72;
                                if ((long)v20 < (long)v72) goto label_b293;
                                *(unsigned long *)&v41[-8] = 0xb443;
                                v26 = (void **)lseek(v51,0,2);
                                if ((long)v26 < 0) goto label_b50c;
                                if ((long)v20 < (long)v26) {
                                  v72 = v26;
                                  goto label_b293;
                                }
label_b500:
                                v16 = v68;
                                v53 = v67;
                                a0 = v71;
                                v72 = v26;
                                goto label_b3ff;
                              }
                              if ((long)v72 < (long)v26)
                                v72 = v26;
label_b293:
                              *(unsigned long *)&v41[-8] = 0xb2a0;
                              if ((long)lseek(v51,v20,0) < 0) {
label_b50c:
                                a0 = v71;
                                goto label_b521;
                              }
                              if (&v20[-v54 - (long)v57]) {
                                if (v61 != 1) {
                                  *(unsigned long *)&v41[-8] = 0xb2e1;
                                  v10 = sub_6920(v52,v80,v61 == 3);
                                  v66 = (unsigned long *)CONCAT71(v66._1_7_,v10);
                                  if (v10) {
                                    v54 = (long)v26 - (long)v20;
                                    v26 = v60;
                                    goto label_b2fc;
                                  }
label_b575:
                                  v16 = v68;
                                  v53 = v67;
                                  a0 = v71;
                                  goto label_b56d;
                                }
                                *(unsigned long *)&v41[-8] = 0xb46e;
                                v10 = sub_6170(v52,&v20[-v54 - (long)v57]);
                                a0 = v71;
                                v16 = v68;
                                v53 = v67;
                                if (v10) {
                                  v54 = (long)v26 - (long)v20;
                                  v66 = (unsigned long *)((unsigned long)v66 & 0xffffffffffffff00);
                                  v26 = NULL;
                                  goto label_b2fc;
                                }
                                *(unsigned long *)&v41[-8] = 0xb982;
                                v18 = "%s: write failed";
                                v72 = (void **)sub_147c0(0,3,v80);
                                goto label_b541;
                              }
                              v54 = (long)v26 - (long)v20;
                              v26 = NULL;
                              if (v61 != 1)
                                v26 = v60;
                              v66 = (unsigned long *)((unsigned long)v66 & 0xffffffffffffff00);
label_b2fc:
                              a0 = v71;
                              *(char **)&v41[-8] = &v37[1];
                              *(unsigned long **)&v41[-0x10] = &v38;
                              *(long *)&v41[-0x18] = v54;
                              *(stat **)&v41[-0x20] = v80;
                              *(stat **)&v41[-0x28] = v71;
                              *(unsigned long *)&v41[-0x30] = (unsigned long)v59;
                              *(unsigned long *)&v41[-0x38] = 0xb342;
                              v10 = sub_6a30(v51,v52,&v36,v69,v26,1);
                              if (!v10) goto label_b575;
                              v26 = (void **)&v20[v38];
                              v55 = (unsigned char)v66;
                              if (v38)
                                v55 = v37[1];
                              v66 = (unsigned long *)CONCAT71(v66._1_7_,v55);
                              v64 = v26;
                              if (v38 < v54) goto label_b500;
                              *(unsigned long *)&v41[-8] = 0xb391;
                              v18 = (char *)lseek(v51,v26,3);
                              if ((long)v18 < 0) goto label_b3e3;
                              v57 = v20;
                            } while( true );
                          }
                          v64 = NULL;
                          v56 = 0 < (long)v35._48_8_;
                          goto label_b5a2;
                        }
                        v60 = NULL;
                        v14 = v53[3];
                      }
                      else {
                        v72 = (void **)CONCAT44(v72._4_4_,v13);
                        *(unsigned long *)&v41[-8] = 0xb0b5;
                        sub_e5e0(v51,0,0,2);
                        if ((int)v72 == 3) {
                          v59 = (unsigned int)(v53[0x11] != 0);
                          v61 = v53[3];
                          goto label_b1f7;
                        }
                        v14 = v53[3];
                        v59 = (unsigned int)(v53[0x11] != 0);
                      }
                      v37[0] = 0;
                      *(char **)&v41[-8] = v37;
                      *(unsigned long **)&v41[-0x10] = &v38;
                      *(unsigned long *)&v41[-0x18] = 0xffffffffffffffff;
                      *(stat **)&v41[-0x20] = v80;
                      *(stat **)&v41[-0x28] = a0;
                      *(unsigned long *)&v41[-0x30] = (unsigned long)v59;
                      *(unsigned long *)&v41[-0x38] = 0xac38;
                      v10 = sub_6a30(v51,v52,&v36,v69,v60,v14 == 3);
                      goto label_ac42;
                    }
                    v13 = 1;
                    if ((long)v35._48_8_ / 0x200 <= v102) goto label_ab10;
                    *(unsigned long *)&v41[-8] = 0x9662;
                    v68 = (char *)lseek(v51,0,3);
                    if (0 <= (long)v68) {
label_b4d9:
                      v13 = 3;
                      goto label_ab10;
                    }
                    *(unsigned long *)&v41[-8] = 0x9677;
                    v26 = (void **)__errno_location();
                    v13 = *(int *)v26;
                    v72 = v26;
                    if (v13 == 6) goto label_b4d9;
                    if ((v13 == 0x16) || (v13 == 0x5f)) {
                      v13 = 2;
                      goto label_ab10;
                    }
                    *(unsigned long *)&v41[-8] = 0x96ab;
                    v76 = (void **)sub_14440(4,a0);
                    *(unsigned long *)&v41[-8] = 0x96c5;
                    v18 = dcgettext(NULL,"cannot lseek %s",5);
                    v13 = *(int *)v72;
                    *(unsigned long *)&v41[-8] = 0x96e2;
                    error(0,v13,v18,(char *)v76);
                    goto label_96e2;
                  }
label_a64a:
                  v14 = *(unsigned char *)((long)v53 + 0x1d) | (unsigned int)v63;
                  v76 = (void **)CONCAT44(v76._4_4_,v14);
                  if (v14) {
                    v76 = (void **)((unsigned long)v76 & 0xffffffffffffff00);
                    goto label_9568;
                  }
                  v34._24_8_ &= 0xffffffff00000000;
                  if (*(char *)((long)v53 + 0x1f)) goto label_a676;
                  goto label_a74a;
                }
                *(unsigned long *)&v41[-8] = 0x9529;
                v10 = sub_7510(v80,0,v53);
                if ((v10) || (!(char)v53[0xd])) goto label_953b;
                v84 = (stat *)((unsigned long)v84 & 0xffffffffffffff00);
                v55 = 0;
                goto label_96e5;
              }
              if (v13 != 2) {
                v10 = '\0';
                if (*(char *)((long)v53 + 0x16)) {
                  v72 = v26;
                  *(unsigned long *)&v41[-8] = 0xad87;
                  if (unlinkat(v81,(char *)v79,0)) { // branch-flip
                    if (*(int *)v72 != 2) {
                      v76 = v72;
                      *(unsigned long *)&v41[-8] = 0xb6b1;
                      v26 = (void **)sub_14440(4,v80);
                      *(unsigned long *)&v41[-8] = 0xb6c7;
                      v18 = dcgettext(NULL,"cannot remove %s",5);
                      v13 = *(int *)v76;
                      goto label_a000;
                    }
                  }
                  else if ((char)v53[0xf]) {
                    *(unsigned long *)&v41[-8] = 0xb7dc;
                    v18 = (char *)sub_14440(4,v80);
                    *(unsigned long *)&v41[-8] = 0xb7f2;
                    v20 = dcgettext(NULL,"removed %s\n",5);
                    *(unsigned long *)&v41[-8] = 0xb804;
                    __printf_chk(1,v20,v18);
                  }
                  goto label_ada1;
                }
label_a58d:
                v84 = (stat *)CONCAT71(v84._1_7_,v10);
label_a593:
                v76 = (void **)CONCAT44(v76._4_4_,v13);
                *(unsigned long *)&v41[-8] = 0xa5ab;
                v26 = (void **)sub_14440(4,v80);
                *(unsigned long *)&v41[-8] = 0xa5c1;
                v18 = dcgettext(NULL,"cannot create regular file %s",5);
                v13 = (int)v76;
                goto label_a000;
              }
label_ada1:
              if (!*(long *)&v53[10]) {
label_a500:
                v28 = ~(unsigned int)v70 & (unsigned int)v65;
                if (((char)v75) && (!*(char *)((long)v53 + 0x1b)))
                  v28 |= 0x80;
                v71 = (stat *)CONCAT44(v71._4_4_,v28);
                *(unsigned long *)&v41[-8] = 0xa548;
                v28 = sub_125a0(v81,v79,0xc1,v28);
                v52 = (unsigned long)v28;
                *(unsigned long *)&v41[-8] = 0xa54f;
                v72 = (void **)__errno_location();
                v55 = *(int *)v72 == 0x11 & (unsigned char)(v28 >> 0x1f);
                v84 = (stat *)CONCAT71(v84._1_7_,v55);
                v28 = (unsigned int)v71;
                v13 = *(int *)v72;
                if (v55) {
                  v10 = (char)v53[6];
                  v13 = 0x11;
                  if (v10) goto label_a58d;
                  v69 = (void **)CONCAT44(v69._4_4_,0x11);
                  *(unsigned long *)&v41[-8] = 0xaa39;
                  v54 = readlinkat(v81,(char *)v79,&v40,1);
                  v28 = (unsigned int)v71;
                  v13 = (int)v69;
                  if (0 <= v54) {
                    if (!*(char *)((long)v53 + 0x3e)) {
                      v76 = (void **)((unsigned long)v76 & 0xffffffffffffff00);
                      *(unsigned long *)&v41[-8] = 0xb873;
                      v18 = (char *)sub_14440(4,v80);
                      *(unsigned long *)&v41[-8] = 0xb889;
                      v20 = dcgettext(NULL,"not writing through dangling symlink %s",5);
                      v30 = NULL;
                      *(unsigned long *)&v41[-8] = 0xb89c;
                      error(0,0,v20,v18);
                      v50 = SUB81(v76,0);
                      goto label_9710;
                    }
                    v84 = (stat *)CONCAT44(v84._4_4_,(unsigned int)v71);
                    *(unsigned long *)&v41[-8] = 0xaa7d;
                    v28 = sub_125a0(v81,v79,0x41,(unsigned long)v71 & 0xffffffff);
                    v52 = (unsigned long)v28;
                    v13 = *(int *)v72;
                    v28 = (unsigned int)v84;
                  }
                }
                v27 = v80;
                v55 = v13 == 0x15 & (unsigned char)(v52 >> 0x1f);
                v84 = (stat *)CONCAT71(v84._1_7_,v55);
                if (v55) { // branch-flip
                  v13 = 0x15;
                  if (*(char *)v80) {
                    *(unsigned long *)&v41[-8] = 0xaac4;
                    v51 = strlen((char *)v80);
                    v13 = (*(char *)((long)v27 + (v51 - 1)) != '/') + 0x14;
                  }
                }
                else {
                  v63 = (unsigned int *)CONCAT44(v63._4_4_,~(unsigned int)v65 & v28);
                  if (0 <= (int)v52) {
                    v58 = (unsigned int)v70;
                    v84 = (stat *)CONCAT71(v84._1_7_,1);
                    goto label_954f;
                  }
                  v84 = (stat *)CONCAT71(v84._1_7_,1);
                }
                goto label_a593;
              }
              *(unsigned long *)&v41[-8] = 0xadc9;
              v10 = sub_72a0(a0,v80,(unsigned long)v65 & 0xffffffff,1,v53);
              if (v10) goto label_a500;
              v84 = (stat *)((unsigned long)v84 & 0xffffffffffffff00);
              v30 = NULL;
              v50 = 0;
            }
            goto label_9710;
          }
          v51 = 1;
          if (!*(char *)((long)v53 + 0x16))
            v51 = (unsigned long)(v53[2] == 3);
          *(unsigned long *)&v41[-0x10] = (unsigned long)v71 & 0xffffffff;
          *(unsigned long *)&v41[-0x18] = 0;
          *(unsigned long *)&v41[-0x20] = v51;
          *(unsigned long *)&v41[-0x28] = 0x8ea9;
          v12 = sub_65d0(a0,0xffffff9c,a0,v80,v81,v79);
          v10 = '\0';
          goto label_8eb2;
        }
        if (*(char *)a0 != '/') {
          *(unsigned long *)&v41[-8] = 0x7e9a;
          v18 = (char *)sub_e480(v79);
          if (v81 != -100) { // branch-flip
label_7ece:
            *(unsigned long *)&v41[-8] = 0x7eda;
            if (!stat(".",(stat *)v34)) {
              *(unsigned long *)&v41[-8] = 0x9aff;
              if ((!fstatat(v81,v18,(stat *)v35,0)) && ((v34._8_8_ != v35._8_8_ || (v34._0_8_ != v35._0_8_)))) {
                *(unsigned long *)&v41[-8] = 0x9b23;
                free(v18);
                *(unsigned long *)&v41[-8] = 0x9b36;
                v19 = sub_147c0(0,3,v80);
                v18 = "%s: can make relative symbolic links only in current directory";
                goto label_8394;
              }
            }
          }
          else {
            v76 = (void **)0x1a14b;
            *(unsigned long *)&v41[-8] = 0x7ec3;
            if (strcmp(".",v18)) goto label_7ece;
          }
          *(unsigned long *)&v41[-8] = 0x7eea;
          free(v18);
        }
        v50 = *(char *)((long)v53 + 0x16);
        *(unsigned long *)&v41[-8] = 0x7f0a;
        v13 = sub_c000(a0,v81,v79,(unsigned char)v50,0xffffffff);
        v10 = '\0';
        if (1 <= v13) {
          *(unsigned long *)&v41[-8] = 0x9772;
          v18 = (char *)sub_14310(1,4,a0);
          *(unsigned long *)&v41[-8] = 0x9788;
          v20 = (char *)sub_14310(0,4,v80);
          *(unsigned long *)&v41[-8] = 0x979e;
          v25 = dcgettext(NULL,"cannot create symbolic link %s to %s",5);
          *(unsigned long *)&v41[-8] = 0x97b2;
          error(0,v13,v25,v20,v18);
          goto label_8658;
        }
        goto label_7f17;
      }
      v78 = (char *)(CONCAT44(v78._4_4_,v82) & 0xffffffff0000f000);
      if ((v82 & 0xf000) == 0x4000) {
        *(unsigned long *)&v41[-8] = 0x832b;
        v10 = sub_72a0(a0,v80,v82,v52,v53);
        if (!v10) {
          v51 = 0;
          return v51 & 0xffffffff;
        }
        v70 = (char *)(CONCAT44(v70._4_4_,v28) & 0xffffffff00000012);
        v21 = v75;
label_8356:
        for (; v21; v21 = (unsigned long *)*v21) {
          if ((v21[1] == v32._8_8_) && ((void *)v21[2] == v32._0_8_)) {
            *(unsigned long *)&v41[-8] = 0x8385;
            v19 = sub_14440(4,a0);
            v18 = "cannot copy cyclic symbolic link %s";
            goto label_8394;
          }
        }
        v42 = &v41[-0x20];
        v43 = &v41[-0x20];
        v44 = &v41[-0x20];
        *(unsigned long *)&v41[-8] = *(unsigned long *)&v41[-8];
        v21 = (unsigned long *)((unsigned long)&v41[-0x11] & 0xfffffffffffffff0);
        *v21 = v75;
        v21[1] = v32._8_8_;
        v21[2] = v32._0_8_;
        v66 = v21;
        if (((char)v84) || ((v33._24_4_ & 0xf000) != 0x4000)) {
          v15 = ~(unsigned int)v70;
          *(unsigned long *)&v41[-0x28] = 0x8794;
          if (mkdirat(v81,(char *)v79,v15 & v28 & 0xfff)) { // branch-flip
            *(unsigned long *)&v41[-0x28] = 0x97e9;
            v19 = sub_14440(4,v80);
            v18 = "cannot create directory %s";
          }
          else {
            *(unsigned long *)&v41[-0x28] = 0x87ba;
            v13 = fstatat(v81,(char *)v79,(stat *)v33,0x100);
            v29 = v33._24_4_;
            if (!v13) {
              v64 = (void **)((unsigned long)v64 & 0xffffffffffffff00);
              if ((v33._24_4_ & 0x1c0) != 0x1c0) {
                *(unsigned long *)&v41[-0x28] = 0x87fc;
                v13 = fchmodat(v81,(char *)v79,v33._24_4_ | 0x1c0,0x100);
                v62 = v29;
                v64 = (void **)CONCAT71(v64._1_7_,1);
                if (v13) {
                  *(unsigned long *)&v41[-0x28] = 0xa4ec;
                  v19 = sub_14440(4,v80);
                  v18 = "setting permissions for %s";
                  v44 = &v41[-0x20];
                  goto label_8638;
                }
              }
              if (!*(char *)v76) {
                *(unsigned long *)&v41[-0x28] = 0x9f16;
                sub_bd30(v79,v33._8_8_,v33._0_8_);
                *(char *)v76 = '\x01';
              }
              if ((char)v53[0xf]) {
                if ((char)v53[6]) { // branch-flip
                  *(unsigned long *)&v41[-0x28] = 0x8844;
                  v16 = (char *)sub_14440(4,v80);
                  *(unsigned long *)&v41[-0x28] = 0x885a;
                  v18 = dcgettext(NULL,"created directory %s\n",5);
                  *(unsigned long *)&v41[-0x28] = 0x886c;
                  __printf_chk(1,v18,v16);
                }
                else {
                  *(unsigned long *)&v41[-0x28] = 0xa199;
                  sub_6510(a0,v80,NULL);
                }
              }
              goto label_8870;
            }
            *(unsigned long *)&v41[-0x28] = 0xa034;
            v19 = sub_14440(4,v80);
            v18 = "cannot stat %s";
            v44 = &v41[-0x20];
          }
label_8638:
          *(unsigned long *)&v44[-8] = 0x863f;
          v18 = dcgettext(NULL,v18,5);
          *(unsigned long *)&v44[-8] = 0x8647;
          v13 = *__errno_location();
          *(unsigned long *)&v44[-8] = 0x8658;
          error(0,v13,v18,v19);
          goto label_8658;
        }
        if ((*(long *)&v53[10]) || (*(char *)((long)v53 + 0x33))) {
          *(unsigned long *)&v41[-0x28] = 0x9a8c;
          v10 = sub_7510(v80,0,v53);
          if (!v10) {
            v44 = &v41[-0x20];
            if ((char)v53[0xd]) goto label_8658;
          }
        }
        v64 = (void **)((unsigned long)v64 & 0xffffffffffffff00);
        v70 = (char *)((unsigned long)v70 & 0xffffffff00000000);
label_8870:
        v55 = v72 != NULL & (unsigned char)v53[7];
        v51 = CONCAT71((undefined7)((unsigned long)v16 >> 8),v55);
        if ((!v55) || (*v72 == v32._0_8_)) {
          v54 = *(long *)v53;
          v24 = *(unsigned long *)&v53[2];
          v1 = *(unsigned long *)&v53[4];
          v2 = *(unsigned long *)&v53[6];
          v3 = *(unsigned long *)&v53[8];
          v4 = *(unsigned long *)&v53[10];
          v5 = *(unsigned long *)&v53[0xc];
          v6 = *(unsigned long *)&v53[0xe];
          v7 = *(unsigned long *)&v53[0x10];
          v8 = *(unsigned long *)&v53[0x12];
          v19 = *(unsigned long *)&v53[0x14];
          *(unsigned long *)&v41[-0x28] = 0x88e0;
          v38 = v54;
          v85 = v24;
          v86 = v1;
          v87 = v2;
          v88 = v3;
          v89 = v4;
          v90 = v5;
          v91 = v6;
          v92 = v7;
          v93 = v8;
          v94 = v19;
          v16 = (char *)sub_15300(a0,2);
          v9 = v73;
          v65 = v16;
          if (v16) { // branch-flip
            if (v53[1] == 3)
              v38 = CONCAT44(2,(unsigned int)v38);
            if (*v16) { // branch-flip
              v74 = (long)v79 - (long)v80;
              v73 = v32;
              v69 = &v36;
              v72 = (void **)v37;
              v71 = (stat *)&v37[1];
              v68 = (char *)&v38;
              v63 = v53;
              v75 = (unsigned long *)((unsigned long)v75._1_7_ << 8);
              v67 = (unsigned int *)CONCAT44(v67._4_4_,(int)v52);
              v78 = (char *)CONCAT71(v78._1_7_,1);
              v77 = a0;
              do {
                *(unsigned long *)&v41[-0x28] = 0x89dc;
                v30 = (void *)sub_ea10(v77,v16,NULL);
                *(unsigned long *)&v41[-0x28] = 0x89f0;
                v22 = (void *)sub_ea10(v80,v16,NULL);
                v55 = *(unsigned char *)v76;
                *(void ***)&v41[-0x28] = v69;
                *(void ***)&v41[-0x30] = v72;
                *(stat **)&v41[-0x38] = v71;
                *(unsigned long *)&v41[-0x40] = 0;
                *(char **)&v41[-0x48] = v68;
                *(unsigned long **)&v41[-0x50] = v66;
                *(unsigned long *)&v41[-0x58] = 0x8a4d;
                v37[1] = v55;
                v11 = sub_76f0(v30,v22,v81,(long)v22 + v74,(unsigned long)v67 & 0xffffffff,v73);
                v55 = v37[0];
                *v9 = *v9 | v37[0];
                v78 = (char *)CONCAT71(v78._1_7_,(unsigned char)v78 & v11);
                *(unsigned long *)&v41[-0x28] = 0x8a6a;
                free(v22);
                *(unsigned long *)&v41[-0x28] = 0x8a72;
                free(v30);
                if (v55) break;
                v75 = (unsigned long *)CONCAT71(v75._1_7_,(unsigned char)v75 | v37[1]);
                *(unsigned long *)&v41[-0x28] = 0x89bd;
                v16 = &v16[strlen(v16) + 1];
              } while (*v16);
              v51 = (unsigned long)v78 & 0xff;
              v53 = v63;
              a0 = v77;
            }
            else {
              v75 = (unsigned long *)((unsigned long)v75._1_7_ << 8);
              v51 = 1;
            }
            *(unsigned long *)&v41[-0x28] = 0x8a9d;
            free(v65);
            *(unsigned char *)v76 = (char)v75;
          }
          else {
            *(unsigned long *)&v41[-0x28] = 0xb9a7;
            v19 = sub_14440(4,a0);
            *(unsigned long *)&v41[-0x28] = 0xb9bd;
            v16 = dcgettext(NULL,"cannot access %s",5);
            *(unsigned long *)&v41[-0x28] = 0xb9c5;
            v13 = *__errno_location();
            v51 = 0;
            *(unsigned long *)&v41[-0x28] = 0xb9d9;
            error(0,v13,v16,v19);
          }
        }
        if ((char)v83) {
          v10 = '\0';
          v15 = 0;
          v55 = 0;
          v74 = CONCAT71(v74._1_7_,v64._0_1_);
          goto label_7f59;
        }
        v15 = 0;
        v74 = CONCAT71(v74._1_7_,v64._0_1_);
        goto label_7fd8;
      }
      *(unsigned long *)&v41[-8] = 0x7dfe;
      v10 = sub_72a0(a0,v80,v82,v52,v53);
      v70 = (char *)((unsigned long)v70 & 0xffffffff00000000);
      if (v10) goto label_7e7a;
      goto label_7cf0;
    }
    if (!*v53) {
      v27 = (stat *)v33;
      if (!v55) {
        v27 = (stat *)v35;
        *(unsigned long *)&v44[-8] = 0x91f1;
        if (fstatat(v81,(char *)v79,v27,0x100)) goto label_789f;
      }
      if ((*(unsigned int *)&v27->field_0x18 & 0xf000) == 0xa000) {
        v48 = *(struct_6 **)&v53[0x12];
        *(unsigned long *)&v44[-8] = 0x7b26;
        v10 = sub_e820(v48,v79,v27);
        if (v10) {
          *(unsigned long *)&v44[-8] = 0x7b44;
          v16 = (char *)sub_14310(1,4,v80);
          *(unsigned long *)&v44[-8] = 0x7b56;
          v18 = (char *)sub_14310(0,4,a0);
          *(unsigned long *)&v44[-8] = 0x7b6c;
          v20 = dcgettext(NULL,"will not copy %s through just-created symlink %s",5);
          *(unsigned long *)&v44[-8] = 0x7b80;
          error(0,0,v20,v18,v16);
          v51 = 0;
          return v51 & 0xffffffff;
        }
      }
      goto label_789f;
    }
    if ((char)v53[0xf]) goto label_8258;
    v41 = v44;
    if (!v28) goto label_7d98;
label_78ba:
    v41 = v44;
    if ((!(char)v53[0xe]) || ((v82 & 0xf000) != 0x4000)) {
      v10 = (char)v53[6];
      if (!v10) {
        if (!(char)v53[0xc]) goto label_7d98;
        if (!*(char *)((long)v53 + 0x17)) goto label_820e;
        v16 = NULL;
        goto label_8246;
      }
      if (v32._16_8_ == 1) {
        *(unsigned long *)&v44[-8] = 0x9343;
        v16 = (char *)sub_bcd0(v32._8_8_,v32._0_8_);
label_8f3d:
        if (v16) {
          if ((v82 & 0xf000) == 0x4000) goto label_790d;
          *(unsigned long *)&v44[-0x10] = (unsigned long)v71 & 0xffffffff;
          *(unsigned long *)&v44[-0x18] = (unsigned long)(unsigned char)v53[0xf];
          *(unsigned long *)&v44[-0x20] = 1;
          *(unsigned long *)&v44[-0x28] = 0x8f90;
          v10 = sub_65d0(NULL,v81,v16,v80,v81,v79);
          if (v10) {
            v51 = 1;
            return v51 & 0xffffffff;
          }
          goto label_8f9c;
        }
        goto label_7970;
      }
      if (((char)v53[0xc]) && (!*(char *)((long)v53 + 0x17))) {
label_820e:
        if ((2 <= (unsigned long)v32._16_8_) || ((((char)v83 && (v53[1] == 3)) || (v16 = NULL, v53[1] == 4)))) {
          *(unsigned long *)&v44[-8] = 0x8f3a;
          v16 = (char *)sub_bd30(v79,v32._8_8_,v32._0_8_);
          goto label_8f3d;
        }
        goto label_823e;
      }
      v16 = NULL;
label_7a20:
      if (v28 == 0x11) {
        *(unsigned long *)&v44[-8] = 0x8dba;
        if (renameat(-100,(char *)a0,v81,(char *)v79)) {
          *(unsigned long *)&v44[-8] = 0x8dc7;
          v28 = *__errno_location();
          if (v28) goto label_7a2a;
        }
label_7d27:
        if ((char)v53[0xf]) {
          *(unsigned long *)&v44[-8] = 0x92a3;
          v16 = dcgettext(NULL,"renamed ",5);
          *(unsigned long *)&v44[-8] = 0x92b2;
          __printf_chk(1,v16);
          *(unsigned long *)&v44[-8] = 0x92c8;
          sub_6510(a0,v80,v77);
        }
        if (*(long *)&v53[10]) {
          *(unsigned long *)&v44[-8] = 0x7d4d;
          sub_7510(v80,1,v53);
        }
        if (v78)
          *v78 = 1;
        if (((char)v83) && (!*(char *)((long)v53 + 0x3f))) {
          v30 = *(void **)&v53[0x12];
          *(unsigned long *)&v44[-8] = 0x921f;
          sub_e790(v30,v79,v32);
          v51 = (unsigned long)v83 & 0xff;
          return v51 & 0xffffffff;
        }
        v51 = 1;
        return v51 & 0xffffffff;
      }
label_7a2a:
      if (v28 == 0x16) {
        *(unsigned long *)&v44[-8] = 0x93ae;
        v16 = (char *)sub_14310(1,4,dat_23568);
        *(unsigned long *)&v44[-8] = 0x93c4;
        v18 = (char *)sub_14310(0,4,dat_23570);
        *(unsigned long *)&v44[-8] = 0x93da;
        v20 = dcgettext(NULL,"cannot move %s to a subdirectory of itself, %s",5);
        v51 = 1;
        *(unsigned long *)&v44[-8] = 0x93f4;
        error(0,0,v20,v18,v16);
        *v73 = 1;
        return v51 & 0xffffffff;
      }
      if (v28 != 0x12) {
        *(unsigned long *)&v44[-8] = 0x8e06;
        v19 = sub_14310(1,4,v80);
        *(unsigned long *)&v44[-8] = 0x8e18;
        v24 = sub_14310(0,4,a0);
        *(unsigned long *)&v44[-8] = 0x8e2e;
        v16 = dcgettext(NULL,"cannot move %s to %s",5);
label_8e3a:
        *(unsigned long *)&v44[-8] = 0x8e43;
        error(0,v28,v16,v24,v19);
        *(unsigned long *)&v44[-8] = 0x8e56;
        sub_bc60(v32._8_8_,v32._0_8_);
        v51 = 0;
        return v51 & 0xffffffff;
      }
      v28 = v82 & 0xf000;
      *(unsigned long *)&v44[-8] = 0x7a6a;
      if (unlinkat(v81,(char *)v79,(unsigned int)(v28 == 0x4000) << 9)) {
        *(unsigned long *)&v44[-8] = 0x7a73;
        v17 = (unsigned int *)__errno_location();
        if (*v17 != 2) {
          *(unsigned long *)&v44[-8] = 0x9246;
          v19 = sub_14310(1,4,v80);
          *(unsigned long *)&v44[-8] = 0x9258;
          v24 = sub_14310(0,4,a0);
          *(unsigned long *)&v44[-8] = 0x926e;
          v16 = dcgettext(NULL,"inter-device move failed: %s to %s; unable to remove target",5);
          v28 = *v17;
          goto label_8e3a;
        }
      }
      v55 = v28 != 0x4000 & (unsigned char)v53[0xf];
      v84 = (stat *)CONCAT71(v84._1_7_,v55);
      if (v55) { // branch-flip
        v52 = 1;
        *(unsigned long *)&v44[-8] = 0x9369;
        v18 = dcgettext(NULL,"copied ",5);
        *(unsigned long *)&v44[-8] = 0x9378;
        __printf_chk(1,v18);
        *(unsigned long *)&v44[-8] = 0x938e;
        sub_6510(a0,v80,v77);
      }
      else {
        v84 = (stat *)CONCAT71(v84._1_7_,1);
        v52 = 1;
      }
      goto label_7da3;
    }
    if ((char)v83) { // branch-flip
      *(unsigned long *)&v44[-8] = 0x7905;
      v16 = (char *)sub_bd30(v79,v32._8_8_,v32._0_8_);
    }
    else {
      *(unsigned long *)&v44[-8] = 0x8f18;
      v16 = (char *)sub_bcd0(v32._8_8_,v32._0_8_);
    }
    if (!v16) {
label_7970:
      v10 = (char)v53[6];
label_823e:
      if (v10) goto label_7a20;
label_8246:
      v52 = (unsigned long)v84 & 0xff;
      goto label_7da3;
    }
label_790d:
    *(unsigned long *)&v44[-8] = 0x7923;
    v10 = sub_14e20(0xffffff9c,a0,v81,v16);
    if (v10) { // branch-flip
      *(unsigned long *)&v44[-8] = 0x9dc1;
      v16 = (char *)sub_14310(1,4,dat_23568);
      *(unsigned long *)&v44[-8] = 0x9dd7;
      v18 = (char *)sub_14310(0,4,dat_23570);
      *(unsigned long *)&v44[-8] = 0x9ded;
      v20 = dcgettext(NULL,"cannot copy a directory, %s, into itself, %s",5);
      *(unsigned long *)&v44[-8] = 0x9e01;
      error(0,0,v20,v18,v16);
      *v73 = 1;
    }
    else {
      *(unsigned long *)&v44[-8] = 0x7942;
      v10 = sub_14e20(v81,v79,v81,v16);
      v27 = v80;
      if (v10) {
        *(unsigned long *)&v44[-8] = 0xa3e9;
        v16 = (char *)sub_14440(4,dat_23570);
        *(unsigned long *)&v44[-8] = 0xa3ff;
        v18 = dcgettext(NULL,"warning: source directory %s specified more than once",5);
        *(unsigned long *)&v44[-8] = 42000;
        error(0,0,v18,v16);
        if ((char)v53[6]) goto label_91b0;
        v51 = 1;
        return v51 & 0xffffffff;
      }
      if ((v53[1] == 4) || ((v53[1] == 3 && ((char)v83)))) goto label_7970;
      *(unsigned long *)&v44[-8] = 0xa1b7;
      v16 = (char *)sub_6110(v80,v79,v16);
      *(unsigned long *)&v44[-8] = 0xa1cc;
      v18 = (char *)sub_14310(1,4,v16);
      *(unsigned long *)&v44[-8] = 0xa1de;
      v20 = (char *)sub_14310(0,4,v27);
      *(unsigned long *)&v44[-8] = 0xa1f4;
      v25 = dcgettext(NULL,"will not create hard link %s to directory %s",5);
      *(unsigned long *)&v44[-8] = 0xa208;
      error(0,0,v25,v20,v18);
      *(unsigned long *)&v44[-8] = 0xa210;
      free(v16);
    }
label_8f9c:
    if (*(char *)((long)v53 + 0x33)) {
      *(unsigned long *)&v44[-8] = 0x8fae;
      if (setfscreatecon(NULL)) goto label_8bf7;
    }
    goto label_866c;
  }
  v84 = (stat *)CONCAT71(v84._1_7_,0 < (int)a4);
  if (v28) goto label_779d;
label_79ae:
  if (*(char *)((long)v53 + 0x3f)) {
    v28 = 0;
    goto label_7803;
  }
  v28 = 0;
  v47 = v79;
  v27 = v80;
  v13 = v81;
  v82 = a4;
label_77b2:
  v13 = fstatat(v13,(char *)v47,(stat *)v32,(unsigned int)(v53[1] == 2) << 8);
  a4 = v82;
  if (!v13) {
    v82 = v32._24_4_;
    if (((v32._24_4_ & 0xf000) != 0x4000) || ((char)v53[0xe])) goto label_7803;
    v19 = sub_14440(4,a0);
    v51 = 0;
    if (*(char *)((long)v53 + 0x19)) {
      v16 = "omitting directory %s";
      v16 = dcgettext(NULL,v16,5);
      error(0,0,v16,v19);
      return v51 & 0xffffffff;
    }
    v16 = dcgettext(NULL,"-r not specified; omitting directory %s",5);
    error(0,0,v16,v19);
    return v51 & 0xffffffff;
  }
label_8528:
  *(unsigned long *)&v43[-8] = 0x8532;
  v19 = sub_14440(4,v27);
  v16 = "cannot stat %s";
  v44 = v43;
label_8541:
  *(unsigned long *)&v44[-8] = 0x8548;
  v16 = dcgettext(NULL,v16,5);
  *(unsigned long *)&v44[-8] = 0x8550;
  v13 = *__errno_location();
  *(unsigned long *)&v44[-8] = 0x8561;
  error(0,v13,v16,v19);
  goto label_7cf0;
label_b3e3:
  *(unsigned long *)&v41[-8] = 0xb3e8;
  v16 = v68;
  v53 = v67;
  if (*__errno_location() != 6) { // branch-flip
label_b521:
    v16 = v68;
    v53 = v67;
    *(unsigned long *)&v41[-8] = 0xb52e;
    v18 = "cannot lseek %s";
    v72 = (void **)sub_14440(4,a0);
label_b541:
    *(unsigned long *)&v41[-8] = 0xb548;
    v76 = (void **)dcgettext(NULL,v18,5);
    *(unsigned long *)&v41[-8] = 0xb554;
    v13 = *__errno_location();
    *(unsigned long *)&v41[-8] = 0xb56d;
    error(0,v13,(char *)v76,v72);
label_b56d:
    v10 = '\0';
  }
  else {
label_b3ff:
    v56 = (long)v64 < (long)v72;
    if (((long)v64 < (long)v72) || ((char)v66)) {
label_b5a2:
      if (v61 != 1) { // branch-flip
        v71 = (stat *)CONCAT71(v71._1_7_,v56);
        *(unsigned long *)&v41[-8] = 0xb5c3;
        v13 = ftruncate(v52,v72);
        if (v13) {
label_b908:
          *(unsigned long *)&v41[-8] = 0xb919;
          v18 = "failed to extend %s";
          v72 = (void **)sub_14440(4,v80);
          goto label_b541;
        }
        if ((v61 == 3) && ((char)v71)) {
          *(unsigned long *)&v41[-8] = 0xb604;
          v13 = fallocate(v52,3,v64,(long)v72 - (long)v64);
          if (v13 <= -1) {
            *(unsigned long *)&v41[-8] = 0xb611;
            v27 = (stat *)__errno_location();
            v13 = *(int *)v27;
            if (v13 != 0x5f && v13 != 0x26) {
              v72 = (void **)CONCAT71(v72._1_7_,v13 == 0x5f || v13 == 0x26);
              v71 = v27;
              *(unsigned long *)&v41[-8] = 0xb64b;
              v76 = (void **)sub_14440(4,v80);
              *(unsigned long *)&v41[-8] = 0xb665;
              v18 = dcgettext(NULL,"error deallocating %s",5);
              v13 = *(int *)v71;
              *(unsigned long *)&v41[-8] = 0xb682;
              error(0,v13,v18,(char *)v76);
              v10 = (char)v72;
              goto label_ac42;
            }
          }
        }
      }
      else {
        v49 = (long)v72 - (long)v64;
        *(unsigned long *)&v41[-8] = 0xb900;
        if (!sub_6170(v52,v49)) goto label_b908;
      }
    }
    v10 = (char)v76;
  }
label_ac42:
  if (v10 != '\x01') { // branch-flip
label_96e2:
    v55 = 0;
  }
  else {
    if (v37[0]) {
      *(unsigned long *)&v41[-8] = 0xb107;
      v13 = ftruncate(v52,v38);
      if (v13 <= -1) {
        *(unsigned long *)&v41[-8] = 0xb120;
        v18 = "failed to extend %s";
        v75 = (unsigned long *)sub_14440(4,v80);
label_b133:
        *(unsigned long *)&v41[-8] = 0xb13a;
        v76 = (void **)dcgettext(NULL,v18,5);
        *(unsigned long *)&v41[-8] = 0xb146;
        v13 = *__errno_location();
        *(unsigned long *)&v41[-8] = 0xb15f;
        error(0,v13,(char *)v76,v75);
        goto label_96e2;
      }
    }
label_ac58:
    v76 = (void **)CONCAT44(v76._4_4_,v63._0_4_);
    if (*(char *)((long)v53 + 0x1f)) {
label_a676:
      v39 = v32._72_8_;
      v95 = v98;
      v96 = v99;
      v97 = v100;
      *(unsigned long *)&v41[-8] = 0xa6cc;
      v13 = sub_e6a0(v52,v81,v79,&v39,0);
      if (v13) {
        *(unsigned long *)&v41[-8] = 0xb175;
        v71 = (stat *)sub_14440(4,v80);
        *(unsigned long *)&v41[-8] = 0xb18f;
        v72 = (void **)dcgettext(NULL,"preserving times for %s",5);
        *(unsigned long *)&v41[-8] = 0xb19b;
        v13 = *__errno_location();
        *(unsigned long *)&v41[-8] = 0xb1b4;
        error(0,v13,(char *)v72,v71);
        if (*(char *)((long)v53 + 0x32)) goto label_96e2;
      }
    }
    if (*(char *)((long)v53 + 0x1d)) {
      if ((v32._28_4_ != v34._28_4_) || (v32._32_4_ != v34._32_4_)) {
        *(char **)&v41[-8] = v34;
        *(unsigned long *)&v41[-0x10] = (unsigned long)v84 & 0xff;
        *(unsigned long *)&v41[-0x18] = ZEXT48(v32._32_4_);
        *(unsigned long *)&v41[-0x20] = ZEXT48(v32._28_4_);
        *(unsigned long *)&v41[-0x28] = 0xa72f;
        v13 = sub_7080(v53,v80,v81,v79,v52,v32._24_4_);
        if (v13 == -1) goto label_96e2;
        if (!v13)
          v73 = (unsigned char *)((unsigned long)v73 & 0xfffffffffffff1ff);
      }
    }
label_a74a:
    if ((char)v75) { // branch-flip
      *(unsigned long *)&v41[-8] = 0xaec7;
      v10 = sub_6220(a0,v51,v80,v52,v53);
      if (v10) goto label_a757;
      v55 = *(unsigned char *)((long)v53 + 0x36) ^ 1;
    }
    else {
label_a757:
      v55 = 1;
    }
    if (*(unsigned long *)&v53[6] & 0xff0000000000ff) { // branch-flip
      v76 = (void **)CONCAT71(v76._1_7_,v55);
      *(unsigned long *)&v41[-8] = 0xae90;
      v13 = sub_c830(a0,v51,v80,v52,(unsigned long)v73 & 0xffffffff);
      v55 = (unsigned char)v76;
      if ((v13) && (*(char *)((long)v53 + 0x32)))
        v55 = 0;
    }
    else if (*(char *)((long)v53 + 0x39)) { // branch-flip
      v14 = v53[4];
      v76 = (void **)CONCAT71(v76._1_7_,v55);
      *(unsigned long *)&v41[-8] = 0xb041;
      v13 = sub_c8e0(v80,v52,v14);
      v55 = (unsigned char)v76;
      if (v13)
        v55 = 0;
    }
    else {
      v11 = (unsigned char)v84 & (unsigned char)v53[8];
      v75 = (unsigned long *)CONCAT71(v75._1_7_,v11);
      if (v11) { // branch-flip
        if (dat_23010 == 0xffffffff) {
          v84 = (stat *)CONCAT71(v84._1_7_,v55);
          *(unsigned long *)&v41[-8] = 0xb817;
          dat_23010 = umask(0);
          *(unsigned long *)&v41[-8] = 0xb824;
          umask(dat_23010);
          v55 = (unsigned char)v84;
        }
        v76 = (void **)CONCAT71(v76._1_7_,v55);
        *(unsigned long *)&v41[-8] = 0xb7a5;
        v13 = sub_c8e0(v80,v52,~dat_23010 & 0x1b6);
        v84 = (stat *)CONCAT71(v84._1_7_,v75._0_1_);
        v55 = (unsigned char)v76;
        if (v13)
          v55 = 0;
      }
      else if ((int)v76 || v58) {
        if (dat_23010 != 0xffffffff) { // branch-flip
          v14 = dat_23010;
          if (~dat_23010 & v58 || (int)v76) {
label_a7ca:
            v76 = (void **)CONCAT71(v76._1_7_,v55);
            *(unsigned long *)&v41[-8] = 0xa7e2;
            v13 = fchmod(v28,(unsigned int)v65 & ~v14);
            v55 = (unsigned char)v76;
            if (v13) {
              v73 = (unsigned char *)CONCAT71(v73._1_7_,(unsigned char)v76);
              *(unsigned long *)&v41[-8] = 0xa80a;
              v75 = (unsigned long *)sub_14440(4,v80);
              *(unsigned long *)&v41[-8] = 0xa824;
              v76 = (void **)dcgettext(NULL,"preserving permissions for %s",5);
              *(unsigned long *)&v41[-8] = 0xa830;
              v13 = *__errno_location();
              *(unsigned long *)&v41[-8] = 0xa849;
              error(0,v13,(char *)v76,v75);
              v55 = (unsigned char)v73;
              if (*(char *)((long)v53 + 0x32))
                v55 = 0;
            }
          }
        }
        else {
          v73 = (unsigned char *)CONCAT71(v73._1_7_,v55);
          *(unsigned long *)&v41[-8] = 0xb9ec;
          dat_23010 = umask(0);
          v75 = (unsigned long *)CONCAT44(v75._4_4_,dat_23010);
          *(unsigned long *)&v41[-8] = 0xb9ff;
          umask(dat_23010);
          v55 = (unsigned char)v73;
          if (~(unsigned int)v75 & v58 || (int)v76) {
            v14 = dat_23010;
            v55 = (unsigned char)v73;
            if ((unsigned int)v75 == 0xffffffff) {
              v76 = (void **)CONCAT71(v76._1_7_,(unsigned char)v73);
              *(unsigned long *)&v41[-8] = 0xba40;
              dat_23010 = umask(0);
              *(unsigned long *)&v41[-8] = 0xba4d;
              umask(dat_23010);
              v14 = dat_23010;
              v55 = (unsigned char)v76;
            }
            goto label_a7ca;
          }
        }
      }
    }
  }
label_96e5:
  v76 = (void **)CONCAT71(v76._1_7_,v55);
  *(unsigned long *)&v41[-8] = 0x96f3;
  if (0 <= close(v28)) { // branch-flip
    v50 = SUB81(v76,0);
    v30 = v36;
  }
  else {
    *(unsigned long *)&v41[-8] = 0xadf3;
    v76 = (void **)sub_14440(4,v80);
    *(unsigned long *)&v41[-8] = 0xae0d;
    v18 = dcgettext(NULL,"failed to close %s",5);
    *(unsigned long *)&v41[-8] = 0xae15;
    v13 = *__errno_location();
    *(unsigned long *)&v41[-8] = 0xae2a;
    error(0,v13,v18,v76);
    v50 = 0;
    v30 = v36;
  }
label_9710:
  v76 = (void **)CONCAT71(v76._1_7_,v50);
  *(unsigned long *)&v41[-8] = 0x971f;
  if (0 <= close(v15)) { // branch-flip
    v15 = 0;
    *(unsigned long *)&v41[-8] = 0x9741;
    free(v30);
    v10 = (char)v76;
    v12 = (char)v76;
label_8eb2:
    if (!v12) goto label_8658;
label_7f17:
    v55 = (int)v78 != 0x4000;
    if (!(char)v84) {
      v11 = ((unsigned char)v53[5] ^ 1) & v55;
      if (v11) {
        if ((*(long *)&v53[10]) || (*(char *)((long)v53 + 0x33))) {
          v78 = (char *)CONCAT71(v78._1_7_,v10);
          v76 = (void **)CONCAT71(v76._1_7_,v11);
          *(unsigned long *)&v41[-8] = 0x8fe9;
          v55 = sub_7510(v80,0,v53);
          v10 = (char)v78;
          if (!v55) {
            v84 = (stat *)CONCAT71(v84._1_7_,(char)v53[0xd]);
            if ((char)v53[0xd]) goto label_8658;
            v55 = (unsigned char)v76;
          }
        }
        else {
          v55 = v11;
        }
      }
    }
    v51 = 1;
    v43 = v41;
    if ((char)v83) {
      v74 &= 0xffffffffffffff00;
      v51 = (unsigned long)v83 & 0xffffffff;
      v42 = v41;
label_7f59:
      v43 = v42;
      if (*(long *)&v53[0x12]) {
        v78 = (char *)CONCAT71(v78._1_7_,v10);
        *(unsigned long *)&v42[-8] = 0x7f8c;
        v83 = (stat *)v35;
        v13 = fstatat(v81,(char *)v79,(stat *)v35,0x100);
        v27 = v83;
        v10 = (char)v78;
        if (!v13) {
          v30 = *(void **)&v53[0x12];
          v83 = (stat *)CONCAT71(v83._1_7_,(char)v78);
          *(unsigned long *)&v42[-8] = 0x7fb6;
          sub_e790(v30,v79,v27);
          v10 = (char)v83;
        }
      }
    }
    if (((*(char *)((long)v53 + 0x17)) && (v55)) || (v10)) {
      return v51 & 0xffffffff;
    }
label_7fd8:
    if (*(char *)((long)v53 + 0x1f)) {
      v35._0_8_ = v32._72_8_;
      v35._8_8_ = v98;
      v35._16_8_ = v99;
      v35._24_8_ = v100;
      *(unsigned long *)&v43[-8] = 0x803b;
      if (utimensat(v81,(char *)v79,v35,(v15 & 0xff) << 8)) {
        *(unsigned long *)&v43[-8] = 0x8050;
        v83 = (stat *)sub_14440(4,v80);
        *(unsigned long *)&v43[-8] = 0x806a;
        v16 = dcgettext(NULL,"preserving times for %s",5);
        *(unsigned long *)&v43[-8] = 0x8072;
        v13 = *__errno_location();
        *(unsigned long *)&v43[-8] = 0x8087;
        error(0,v13,v16,v83);
        if (*(char *)((long)v53 + 0x32)) {
          v51 = 0;
          return v51 & 0xffffffff;
        }
      }
    }
    if ((char)v15) { // branch-flip
      if (!*(char *)((long)v53 + 0x35)) {
        return v51 & 0xffffffff;
      }
      *(unsigned long *)&v43[-8] = 0x8ef7;
      if (sub_6220(a0,0xffffffff,v80,0xffffffff,v53)) {
        return v51 & 0xffffffff;
      }
      v10 = *(char *)((long)v53 + 0x36);
    }
    else {
      if (*(char *)((long)v53 + 0x1d)) {
        if ((((char)v84) || (v33._28_4_ != v32._28_4_)) || (v33._32_4_ != v32._32_4_)) {
          *(char **)&v43[-8] = v33;
          *(unsigned long *)&v43[-0x10] = (unsigned long)v84 & 0xff;
          *(unsigned long *)&v43[-0x18] = ZEXT48(v32._32_4_);
          *(unsigned long *)&v43[-0x20] = ZEXT48(v32._28_4_);
          *(unsigned long *)&v43[-0x28] = 0x8104;
          v13 = sub_7080(v53,v80,v81,v79,0xffffffff,v32._24_4_);
          if (v13 == -1) {
            v51 = 0;
            return v51 & 0xffffffff;
          }
          if (!v13)
            v82 &= 0xfffff1ff;
        }
      }
      if (*(char *)((long)v53 + 0x35)) {
        *(unsigned long *)&v43[-8] = 0xa094;
        v10 = sub_6220(a0,0xffffffff,v80,0xffffffff,v53);
        if ((!v10) && (*(char *)((long)v53 + 0x36))) {
          v51 = 0;
          return v51 & 0xffffffff;
        }
      }
      if (!(*(unsigned long *)&v53[6] & 0xff0000000000ff)) {
        if (*(char *)((long)v53 + 0x39)) // branch-flip
          v28 = v53[4];
        else {
          if (!((unsigned char)v84 & (unsigned char)v53[8])) {
            if ((int)v70) { // branch-flip
              v28 = dat_23010;
              if (dat_23010 == 0xffffffff) {
                *(unsigned long *)&v43[-8] = 0xad45;
                v28 = umask(0);
                *(unsigned long *)&v43[-8] = 0xad54;
                dat_23010 = v28;
                umask(v28);
              }
              v28 = (unsigned int)v70 & ~v28;
              v70 = (char *)CONCAT44(v70._4_4_,v28);
              if (!v28) goto label_8169;
              if ((char)v74 != '\x01') {
                if ((char)v84) {
                  *(unsigned long *)&v43[-8] = 0x8513;
                  if (fstatat(v81,(char *)v79,(stat *)v33,0x100)) {
                    v27 = v80;
                    goto label_8528;
                  }
                }
                v62 = v33._24_4_;
                if (!((unsigned int)v70 & ~v33._24_4_)) {
                  return v51 & 0xffffffff;
                }
              }
            }
            else {
label_8169:
              if (!(char)v74) {
                return v51 & 0xffffffff;
              }
            }
            v28 = v62 | (unsigned int)v70;
            *(unsigned long *)&v43[-8] = 0x8199;
            if (!fchmodat(v81,(char *)v79,v28,0x100)) {
              return v51 & 0xffffffff;
            }
            *(unsigned long *)&v43[-8] = 0x81b2;
            v19 = sub_14440(4,v80);
            *(unsigned long *)&v43[-8] = 0x81c8;
            v16 = dcgettext(NULL,"preserving permissions for %s",5);
            *(unsigned long *)&v43[-8] = 0x81d0;
            v13 = *__errno_location();
            *(unsigned long *)&v43[-8] = 0x81e1;
            error(0,v13,v16,v19);
            goto label_81e1;
          }
          v28 = 0x1ff;
          if ((v82 & 0x7000) != 0x4000)
            v28 = 0x1b6;
          v15 = dat_23010;
          if (dat_23010 == 0xffffffff) {
            *(unsigned long *)&v43[-8] = 0xad2a;
            v15 = umask(0);
            *(unsigned long *)&v43[-8] = 0xad39;
            dat_23010 = v15;
            umask(v15);
          }
          v28 = ~v15 & v28;
        }
        *(unsigned long *)&v43[-8] = 0x9eef;
        if (!sub_c8e0(v80,0xffffffff,v28)) {
          return v51 & 0xffffffff;
        }
        v51 = 0;
        return v51 & 0xffffffff;
      }
      *(unsigned long *)&v43[-8] = 0x9b70;
      if (!sub_c830(a0,0xffffffff,v80,0xffffffff,v82)) {
        return v51 & 0xffffffff;
      }
label_81e1:
      v10 = *(char *)((long)v53 + 0x32);
    }
    if (!v10) {
      return v51 & 0xffffffff;
    }
  }
  else {
    *(unsigned long *)&v41[-8] = 0xa45f;
    v19 = sub_14440(4,a0);
    *(unsigned long *)&v41[-8] = 0xa475;
    v18 = dcgettext(NULL,"failed to close %s",5);
    *(unsigned long *)&v41[-8] = 0xa47d;
    v13 = *__errno_location();
    *(unsigned long *)&v41[-8] = 0xa48e;
    error(0,v13,v18,v19);
    *(unsigned long *)&v41[-8] = 0xa496;
    free(v30);
label_8658:
    if (*(char *)((long)v53 + 0x33)) {
      *(unsigned long *)&v44[-8] = 0x92d7;
      if (setfscreatecon(NULL)) {
label_8bf7:
        *(unsigned long *)&v44[-8] = 0x8bfc; // no-return
        sub_6770();
      }
    }
    if (!v16) {
      *(unsigned long *)&v44[-8] = 0x9323;
      sub_bc60(v32._8_8_,v32._0_8_);
    }
label_866c:
    if (v77) {
      v16 = (char *)(((long)v79 - (long)v80) + (long)v77);
      *(unsigned long *)&v44[-8] = 0x869d;
      if (renameat(v81,v16,v81,(char *)v79)) {
        *(unsigned long *)&v44[-8] = 0x92f9;
        v19 = sub_14440(4,v80);
        v16 = "cannot un-backup %s";
        goto label_8541;
      }
      if ((char)v53[0xf]) {
        *(unsigned long *)&v44[-8] = 0x86c6;
        v16 = (char *)sub_14310(1,4,v80);
        *(unsigned long *)&v44[-8] = 0x86dc;
        v18 = (char *)sub_14310(0,4,v77);
        *(unsigned long *)&v44[-8] = 0x86f2;
        v20 = dcgettext(NULL,"%s -> %s (unbackup)\n",5);
        *(unsigned long *)&v44[-8] = 0x8707;
        __printf_chk(1,v20,v18,v16);
      }
    }
  }
label_7cf0:
  v51 = 0;
  return v51 & 0xffffffff;
}


// Function: sub_ba80 @ 0xba80
void sub_ba80(unsigned long a0,unsigned long a1,unsigned int a2,stat *a3,unsigned long a4,struct_30 *a5,unsigned char *a6,char *a7)
{
  char v1; // stack - 0x11
  
  if (4 <= a5->field_0x0)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (3 <= a5->field_0xc - 1)
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (2 < a5->field_0x44)
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((a5->field_0x17) && (a5->field_0x3a))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a5->field_0xc != 2) && (a5->field_0x44 == 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  v1 = 0;
  dat_23568 = a1;
  dat_23570 = a0;
  sub_76f0(a0,a1,a2,a3,a4,NULL,NULL,a5,1,&v1,a6,a7);
}


// Function: sub_bc10 @ 0xbc10
unsigned long sub_bc10(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_bc20 @ 0xbc20
unsigned long sub_bc20(struct_12 *a0,struct_12 *a1)
{
  if (a0->field_0x0 != a1->field_0x0)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x8 >> 8),a0->field_0x8 == a1->field_0x8);
}


// Function: sub_bc60 @ 0xbc60
void sub_bc60(unsigned long a0,unsigned long a1) // return-dupe
{
  struct_28 *v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = (struct_28 *)sub_12270(dat_23578,&v2);
  if (!v1)
    return;
  free(v1->field_0x10);
  free(v1);
}


// Function: sub_bcd0 @ 0xbcd0
unsigned long sub_bcd0(unsigned long a0,unsigned long a1) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  
  v2 = a0;
  v3 = a1;
  v1 = sub_11920(dat_23578,&v2);
  if (!v1)
    return 0;
  return *(unsigned long *)(v1 + 0x10);
}


// Function: sub_bd30 @ 0xbd30
unsigned long sub_bd30(char *a0,unsigned long a1,unsigned long a2) // early-return
{
  struct_15 *v1; // rax
  unsigned long v2; // rax
  struct_15 *v3; // rax
  
  v1 = (struct_15 *)sub_16c50(0x18);
  v2 = sub_17130(a0);
  *(unsigned long *)v1->field_0x0 = a1;
  v1->field_0x10 = v2;
  *(unsigned long *)((long)v1->field_0x0 + 8) = a2;
  v3 = (struct_15 *)sub_12210(dat_23578,v1);
  if (!v3)
    sub_17150(); // no-return
  if (v1 == v3)
    return 0;
  free((void *)v1->field_0x10);
  free(v1);
  return v3->field_0x10;
}


// Function: sub_bdb0 @ 0xbdb0
void sub_bdb0(void)
{
  dat_23578 = sub_11bb0(0x67,NULL,sub_bc10,sub_bc20,sub_bc40);
  if (dat_23578)
    return;
  sub_17150(); // no-return
}


// Function: sub_bdf0 @ 0xbdf0
void sub_bdf0(char *a0,struct_24 *a1)
{
  linkat(a1->field_0x0,a1->field_0x8,a1->field_0x10,a0,a1->field_0x14); // tail-call
}


// Function: sub_be10 @ 0xbe10
char * sub_be10(void *a0,char *a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3; // rax
  
  v2 = sub_d970(a0) - (long)a0;
  v1 = v2 + 9;
  if ((0x101 <= v1) && (a1 = malloc(v1), !a1))
    return NULL;
  v3 = mempcpy(a1,a0,v2);
  *(unsigned long *)v3 = s_1c458._0_8_;
  v3[8] = s_1c458[8];
  return a1;
}


// Function: sub_be70 @ 0xbe70
void sub_be70(char *a0,struct_25 *a1)
{
  symlinkat(a1->field_0x0,a1->field_0x8,a0); // tail-call
}


// Function: sub_be90 @ 0xbe90
int sub_be90(int a0,char *a1,int a2,char *a3,int a4,char a5,int a6)
{
  char *v1;
  int v2; // eax
  int *v3; // rax
  char v4 [264];
  int v5 [2]; // stack - 0x168
  int v6;
  char *v7; // stack - 0x160
  int v8; // stack - 0x158
  int v9; // stack - 0x154
  
  v2 = a6;
  if (a6 < 0) {
    v2 = linkat(a0,a1,a2,a3,a4);
    if (!v2) {
      v2 = 0;
      return v2;
    }
    v3 = __errno_location();
    v2 = *v3;
  }
  if ((a5 == '\x01') && (v2 == 0x11)) {
    v3 = (char *)sub_be10(a3,v4);
    v1 = (char *)v3;
    if (v3) { // branch-flip
      v5[0] = a0;
      v7 = a1;
      v8 = a2;
      v9 = a4;
      v2 = sub_157b0(v3,0,v5,sub_bdf0,6);
      if (v2) { // branch-flip
        v3 = __errno_location();
        v6 = *v3;
      }
      else {
        v2 = renameat(a2,v1,a2,a3);
        v6 = -1;
        if (v2) {
          v3 = __errno_location();
          v6 = *v3;
        }
        unlinkat(a2,v1,0);
      }
      v2 = v6;
      if (v1 != v4)
        free(v1);
    }
    else {
      v3 = __errno_location();
      v2 = *v3;
    }
  }
  return v2;
}


// Function: sub_c000 @ 0xc000
int sub_c000(char *a0,int a1,char *a2,char a3,int a4)
{
  char *v1; // rax
  char *v2; // stack - 0x158
  char v3 [264];
  int v4; // stack - 0x150
  
  if (a4 < 0) {
    if (!symlinkat(a0,a1,a2))
      return 0;
    a4 = *__errno_location();
  }
  if ((a3 == '\x01') && (a4 == 0x11)) {
    v1 = (char *)sub_be10(a2,v3);
    if (v1) { // branch-flip
      v2 = a0;
      v4 = a1;
      if (sub_157b0(v1,0,&v2,sub_be70,6)) // branch-flip
        a4 = *__errno_location();
      else {
        a4 = -1;
        if (renameat(a1,v1,a1,a2)) {
          a4 = *__errno_location();
          unlinkat(a1,v1,0);
        }
      }
      if (v1 != v3)
        free(v1);
    }
    else {
      a4 = *__errno_location();
    }
  }
  return a4;
}


// Function: sub_c140 @ 0xc140
int sub_c140(long a0,char *a1)
{
  int v1;
  stat v10; // stack - 0xd8
  char *v11; // stack - 0xe0
  int v2;
  int v3;
  int *v4; // rax
  long v5; // rax
  long v6; // rax
  long v7; // rax
  char *v8; // rax
  char *v9; // stack - 0xe8
  
  v9 = NULL;
  v11 = NULL;
  if (!a0) {
    if (getfscreatecon(&v11) < 0)
      return -1;
    v4 = __errno_location();
    if (v11) {
      v2 = lsetfilecon(a1,v11);
      v1 = *v4;
      freecon(v11);
      *v4 = v1;
      return v2;
    }
    *v4 = 0x3d;
    return -1;
  }
  v1 = open(a1,0x20000);
  v4 = __errno_location();
  if (v1 == -1) {
    v2 = *v4;
    if (v2 != 0x28) { // branch-flip
      v6 = 0;
      v5 = 0;
    }
    else {
      if (0 <= lstat(a1,&v10)) goto label_c1bd;
      v2 = *v4;
      v6 = 0;
      v5 = 0;
    }
label_c28b:
    v3 = v1;
    goto label_c28e;
  }
  if (0 <= fstat(v1,&v10)) { // branch-flip
label_c1bd:
    if (0 <= (int)selabel_lookup(a0,&v9,a1,v10._24_4_)) { // branch-flip
      v5 = context_new(v9);
      if (v5) { // branch-flip
        if (v1 != -1) {
          if ((int)sub_15610(v1) < 0) {
            v6 = 0;
            v3 = -1;
            goto label_c3ab;
          }
          v6 = context_new(v11);
          if (v6) goto label_c221;
          v2 = *v4;
          v3 = -1;
          goto label_c24f;
        }
        if ((int)sub_15590(a1,&v11) < 0) {
          v2 = *v4;
          v6 = 0;
          goto label_c28b;
        }
        v6 = context_new(v11);
        if (!v6) {
          v2 = *v4;
          v3 = -1;
          goto label_c28e;
        }
label_c221:
        v7 = context_type_get(v5);
        if (((v7) && (!context_type_set(v6,v7))) && (v8 = (char *)context_str(v6), v8)) {
          if (v1 == -1) {
            v1 = lsetfilecon(a1,v8);
            v2 = *v4;
            goto label_c28b;
          }
          v3 = fsetfilecon(v1,v8);
          goto label_c3ab;
        }
        v2 = *v4;
      }
      else {
        v2 = *v4;
        v6 = 0;
      }
    }
    else {
      v2 = *v4;
      v6 = 0;
      v5 = 0;
      if (v2 == 2) {
        *v4 = 0x3d;
        v2 = 0x3d;
      }
    }
    v3 = -1;
    if (v1 == -1) goto label_c28e;
  }
  else {
    v6 = 0;
    v5 = 0;
    v3 = -1;
label_c3ab:
    v2 = *v4;
  }
label_c24f:
  close(v1);
label_c28e:
  context_free(v5);
  context_free(v6);
  freecon(v9);
  freecon(v11);
  *v4 = v2;
  return v3;
}


// Function: sub_c450 @ 0xc450
int sub_c450(unsigned long a0,char *a1,unsigned int a2)
{
  short v1; // ax
  char *v10; // stack - 0x48
  char *v11; // stack - 0x58
  char *v12;
  int v13;
  int v2; // eax
  int *v3; // rax
  char *v4; // rax
  long v5;
  long v6; // rax
  long v7; // rax
  char *v8; // stack - 0x60
  char *v9; // stack - 0x50
  
  v8 = NULL;
  v11 = NULL;
  v3 = __errno_location();
  if (*a1 != '/') { // branch-flip
    a1 = (char *)sub_da80(a1,2);
    if (!a1) {
      v13 = *v3;
      v12 = NULL;
      v6 = 0;
      v2 = -1;
      v5 = 0;
      goto label_c5df;
    }
    v12 = a1;
  }
  else {
    v12 = NULL;
  }
  if (0 <= (int)selabel_lookup(a0,&v8,a1,a2)) { // branch-flip
    v9 = NULL;
    v10 = NULL;
    v4 = (char *)sub_e480(a1);
    if (((getcon(&v9) <= -1) || ((int)sub_15510(v4,&v10) <= -1)) || (v1 = mode_to_security_class(a2), !v1)) {
      v13 = *v3;
      v6 = 0;
      v5 = 0;
      free(v4);
      v2 = -1;
      freecon(v9);
      freecon(v10);
      *v3 = v13;
      goto label_c5df;
    }
    v2 = security_compute_create(v9,v10,(unsigned short)v1,&v11);
    v13 = *v3;
    free(v4);
    freecon(v9);
    freecon(v10);
    *v3 = v13;
    if (0 <= v2) {
      v5 = context_new(v8);
      if (v5) { // branch-flip
        v6 = context_new(v11);
        if (((v6) && (v7 = context_type_get(v5), v7)) && ((!context_type_set(v6,v7) && (v4 = (char *)context_str(v6), v4)))) {
          v2 = setfscreatecon(v4);
          v13 = *v3;
        }
        else {
          v13 = *v3;
          v2 = -1;
        }
      }
      else {
        v13 = *v3;
        v6 = 0;
        v2 = -1;
      }
      goto label_c5df;
    }
  }
  else {
    v13 = *v3;
    if (v13 == 2) {
      *v3 = 0x3d;
      v13 = 0x3d;
      v6 = 0;
      v5 = 0;
      v2 = -1;
      goto label_c5df;
    }
  }
  v6 = 0;
  v5 = 0;
  v2 = -1;
label_c5df:
  context_free(v5);
  context_free(v6);
  freecon(v8);
  freecon(v11);
  free(v12);
  *v3 = v13;
  return v2;
}


// Function: sub_c6f0 @ 0xc6f0
unsigned long sub_c6f0(long a0,char *a1,bool a2)
{
  char *v1;
  int v10;
  unsigned long v11; // stack - 0x40
  int v2; // eax
  int *v3;
  struct_11 *v4; // rax
  long v5; // rax
  char *v6;
  char *v7; // stack - 0x48
  unsigned long v8; // r12
  undefined7 v9; // r12
  
  v6 = NULL;
  v9 = (undefined7)((unsigned long)a0 >> 8);
  if (*a1 != '/') { // branch-flip
    v6 = (char *)sub_da80(a1,2);
    if (!v6) {
      v8 = 0;
      return v8 & 0xffffffff; // return-dupe
    }
    v3 = __errno_location();
    v7 = v6;
  }
  else {
    v3 = __errno_location();
    v7 = a1;
  }
  if (a2) {
    v11 = 0;
    v10 = 0;
    v4 = (struct_11 *)sub_17190(&v7,0x10,0);
    do {
      v5 = sub_10840(v4);
      while( true ) {
        if (!v5) {
          if (*v3)
            v10 = *v3;
          if (sub_10650(v4))
            v10 = *v3;
          free(v6);
          v8 = CONCAT71(v9,v10 == 0);
          return v8 & 0xffffffff;
        }
        v1 = v4->field_0x20;
        if (0 <= (int)sub_c140(a0,v1)) break;
        v10 = *v3;
        v5 = sub_10840(v4);
      }
    } while( true );
  }
  v2 = sub_c140(a0,v7);
  v10 = *v3;
  v8 = CONCAT71(v9,v2 != -1);
  free(v6);
  *v3 = v10;
  return v8 & 0xffffffff;
}


// Function: sub_e480 @ 0xe480
void sub_e480(char *a0)
{
  if (sub_e4e0(a0))
    return;
  sub_17150(); // no-return
}


// Function: sub_11240 @ 0x11240
unsigned long sub_11240(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}


// Function: sub_11260 @ 0x11260
bool sub_11260(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_11270 @ 0x11270
unsigned long sub_11270(struct_0 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x1c620)
    return 1;
  v1 = v2[2];
  if ((((dat_1c634 < v1) && (v1 < dat_1c638)) && (dat_1c63c < v2[3])) && (dat_1c650 <= *v2)) {
    v3 = *v2 + dat_1c634;
    if (((v3 < v2[1]) && (v2[1] <= dat_1c640)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x1c620;
  return 0;
}


// Function: sub_11300 @ 0x11300
unsigned long sub_11300(struct_1 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + a0->field_0x0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*a0->field_0x38)(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*a0->field_0x38)(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    v4 = *v5;
  }
  if (a3) {
    v1 = (long *)v5[1];
    if (v1) {
      v2 = v1[1];
      *v5 = *v1;
      v5[1] = v2;
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}


// Function: sub_11550 @ 0x11550
unsigned long sub_11550(struct_2 *a0,struct_33 *a1,bool a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = a1->field_0x0;
  if (a1->field_0x8 <= v6)
    return 1;
label_11586:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0->field_0x10;
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*a0->field_0x30)(v8,v4);
          v4 = a0->field_0x10;
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + a0->field_0x0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0->field_0x48;
            a0->field_0x18 = a0->field_0x18 + 1;
            *v7 = 0;
            v7[1] = v8;
            a0->field_0x48 = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*a0->field_0x30)(v8,a0->field_0x10);
        if ((unsigned long)a0->field_0x10 <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + a0->field_0x0);
        if (*v7) { // branch-flip
          v5 = (long *)a0->field_0x48;
          if (v5) // branch-flip
            a0->field_0x48 = v5[1];
          else {
            v5 = malloc(0x10);
            if (!v5)
              return 0;
          }
          v1 = v7[1];
          *v5 = v8;
          v5[1] = v1;
          v7[1] = (long)v5;
        }
        else {
          *v7 = v8;
          a0->field_0x18 = a0->field_0x18 + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1->field_0x18 = a1->field_0x18 + -1;
        if (a1->field_0x8 <= v6)
          return 1;
        goto label_11586;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}


// Function: sub_117c0 @ 0x117c0
void sub_117c0(unsigned long *a0,FILE *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long *v4;
  unsigned long v5;
  unsigned long v6;
  
  v6 = 0;
  v1 = a0[2];
  v2 = a0[3];
  for (v4 = (long *)*a0; v4 < (long *)a0[1]; v4 = &v4[2]) {
    while (!*v4) {
      v4 = &v4[2];
      if ((long *)a0[1] <= v4) goto label_1182d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_1182d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_1c650) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_11920 @ 0x11920
long sub_11920(struct_6 *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = a0->field_0x38;
      if ((*v1)(a1)) break;
      v3 = (long *)v3[1];
      if (!v3)
        return 0;
      v4 = *v3;
    }
    return *v3;
  }
  return 0;
}


// Function: sub_119e0 @ 0x119e0
long sub_119e0(struct_34 *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = v3;
  do {
    v1 = *v4;
    v4 = (long *)v4[1];
    if (v1 == a1) {
      if (v4)
        return *v4;
      break;
    }
  } while (v4);
  do {
    v3 = &v3[2];
    if (a0->field_0x8 <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}


// Function: sub_11ac0 @ 0x11ac0
long sub_11ac0(struct_33 *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = a0->field_0x0;
  if (a0->field_0x8 <= v4) // branch-flip
    v3 = 0;
  else {
    v3 = 0;
    do {
      v2 = *v4;
      if (v2) {
        v1 = v4;
        while( true ) {
          if (!(*a1)(v2,a2))
            return v3;
          v1 = (long *)v1[1];
          v3 += 1;
          if (!v1) break;
          v2 = *v1;
        }
      }
      v4 = &v4[2];
    } while (v4 < a0->field_0x8);
  }
  return v3;
}


// Function: sub_11bb0 @ 0x11bb0
unsigned long * sub_11bb0(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_11240;
  if (!a3)
    a3 = sub_11260;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x1c620;
  v1[5] = a1;
  if (sub_11270(v1)) {
    v2 = sub_11410(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
    v1[2] = v2;
    if (v2) {
      v3 = calloc(v2,0x10);
      *v1 = v3;
      if (v3) {
        v1[6] = a2;
        v1[3] = 0;
        v1[7] = a3;
        v1[8] = a4;
        v1[1] = (void *)((long)v3 + v1[2] * 0x10);
        v1[4] = 0;
        v1[9] = 0;
        return v1;
      }
    }
  }
  free(v1);
  return NULL;
}


// Function: sub_11cc0 @ 0x11cc0
void sub_11cc0(struct_36 *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = a0->field_0x0;
  if (a0->field_0x0 < a0->field_0x8) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if (a0->field_0x8 <= v5) goto label_11d5b;
      }
      v3 = a0->field_0x40;
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = a0->field_0x40;
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0->field_0x48;
          *v4 = 0;
          v4[1] = v2;
          a0->field_0x48 = v4;
          if (!v1) break;
          v4 = v1;
        }
      }
      if (v3)
        (*v3)(*v5);
      *v5 = 0;
      v6 = &v5[2];
      v5[1] = 0;
      v5 = v6;
    } while (v6 < a0->field_0x8);
  }
label_11d5b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_11d70 @ 0x11d70
void sub_11d70(struct_7 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_11e1c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11ddf;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0->field_0x40)(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = a0->field_0x8;
      v4 = &v4[2];
    } while (v4 < v3);
label_11ddf:
    v4 = a0->field_0x0;
  }
  if (v4 < v3) {
    do {
      v2 = (void *)v4[1];
      while (v2) {
        v1 = *(void **)((long)v2 + 8);
        free(v2);
        v2 = v1;
      }
      v4 = &v4[2];
    } while (v4 < a0->field_0x8);
  }
label_11e1c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_11e60 @ 0x11e60
unsigned int sub_11e60(struct_4 *a0,unsigned long a1) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v10; // stack - 0x48
  unsigned long v11; // stack - 0x40
  void *v2; // stack - 0x88
  void *v3; // stack - 0x80
  unsigned long v4; // stack - 0x78
  unsigned long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  v4 = sub_11410(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
  if (!v4)
    return 0;
  if (a0->field_0x10 == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0->field_0x28;
  v6 = 0;
  v8 = a0->field_0x30;
  v9 = a0->field_0x38;
  v10 = a0->field_0x40;
  v11 = a0->field_0x48;
  v1 = sub_11550(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_11550(a0,&v2,1)) && (sub_11550(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free(a0->field_0x0);
  a0->field_0x0 = v2;
  a0->field_0x8 = v3;
  a0->field_0x10 = v4;
  a0->field_0x18 = v5;
  a0->field_0x48 = v11;
  return v1;
}


// Function: sub_11fe0 @ 0x11fe0
long * sub_11fe0(void *a0,long a1,long *a2) // return-dupe, ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4;
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (!a1) {
    abort(); // no-return, return-dupe
  }
  v2 = (long)sub_11300(a0,a1,&v3,0);
  if (v2) {
    if (!a2) {
      v2 = NULL;
      return v2;
    }
    *a2 = (long)v2;
    v2 = NULL;
    return v2;
  }
  v4 = *(unsigned long *)((long)a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)((long)a0 + 0x10);
    if ((long)v2 > -1) goto label_1206f;
label_120f6:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)((long)a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_120f6;
label_1206f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
    sub_11270(a0);
    v1 = *(long *)((long)a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_1c644 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_1c648 <= v5) ? (long)(v5 - dat_1c648) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_11e60(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_11300(a0,a1,&v3,0);
      if (v2)
        abort();
    }
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)((long)a0 + 0x48);
  if (v2) // branch-flip
    *(long *)((long)a0 + 0x48) = v2[1];
  else {
    v2 = malloc(0x10);
    if (!v2) {
      v2 = (long *)0xffffffff;
      return v2;
    }
  }
  v1 = v3[1];
  *v2 = a1;
  v2[1] = v1;
  v3[1] = (long)v2;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}


// Function: sub_12210 @ 0x12210
unsigned long sub_12210(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_11fe0(a0,a1,&v3);
  if (v1 != -1) {
    v4 = v1 == 0;
    v2 = a1;
    if (v4)
      v2 = v3;
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_12270 @ 0x12270
long sub_12270(void *a0,long a1) // ternary x2
{
  unsigned long v1;
  float *v2;
  void *v3;
  void *v4;
  long v5; // rax
  unsigned long v6; // rax
  long *v7; // stack - 0x28
  float v8; // xmm4_da
  float v9;
  
  v5 = sub_11300(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)((long)a0 + 0x18) - 1;
    *(unsigned long *)((long)a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)((long)a0 + 0x10) : *(unsigned long *)((long)a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)((long)a0 + 0x28) * v8) {
      sub_11270(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1c648 <= v8) ? (long)(v8 - dat_1c648) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11e60(a0,v6)) {
          v4 = *(void **)((long)a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)((long)a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

