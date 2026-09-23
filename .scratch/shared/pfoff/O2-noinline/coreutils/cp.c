// Function: main @ 0x40b0
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
  char v20; // stack - 0x84
  char v21; // stack - 0x83
  char v22; // stack - 0x82
  char v23; // stack - 0x81
  char v24; // stack - 0x7c
  char v25; // stack - 0x7b
  unsigned short v26; // stack - 0x7a
  long v27; // stack - 0x70
  char v28; // stack - 0x68
  char v29; // stack - 0x67
  int v3; // eax
  char v30; // stack - 0x66
  char v31; // stack - 0x65
  char v32; // stack - 0x64
  char v33; // stack - 0x63
  unsigned short v34; // stack - 0x61
  char v35; // stack - 0x5e
  char v36; // stack - 0x5d
  char v37; // stack - 0x5c
  int v38; // stack - 0x54
  int v4; // eax
  int v5; // eax
  int v6; // eax
  char *v7;
  unsigned long v8; // rax
  long v9; // rdx
  
  sub_127f0(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_1a150(sub_e560);
  dat_2515a = 0 < is_selinux_enabled();
  sub_49f0(&v10);
  v15 = 0;
  v14 = NULL;
  v11 = NULL;
  v16 = 0;
  v12 = NULL;
  v13 = NULL;
  v1 = 0;
label_4188:
  v2 = getopt_long(argc,argv,"abdfHilLnprst:uvxPRS:TZ",(option *)0x244a0,NULL);
  if (v2 != -1) {
    if (0x88 < v2) {
      sub_57e0(1); // return-dupe, no-return
    }
    if (v2 <= 0x47) {
      if (v2 == -0x83) {
        sub_16cc0(stdout,"cp","GNU coreutils",dat_25020,"Torbjorn Granlund","David MacKenzie","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_57e0(0); // no-return
      sub_57e0(1);
    }
    switch(v2) {
      case 0x48:
        v17 = 3;
        break;
      default:
        goto label_4769;
      case 0x4c:
        v17 = 4;
        break;
      case 0x50:
        v17 = 2;
        break;
      case 0x52:
      case 0x72:
        v34 = CONCAT11(1,(char)v34);
        break;
      case 0x53:
        v13 = optarg;
        v1 = 1;
        break;
      case 0x54:
        v15 = 1;
        break;
      case 0x5a:
        if (dat_2515a) { // branch-flip
          if (optarg) // branch-flip
            v14 = optarg;
          else {
            v27 = selabel_open(0,0,0);
            if (!v27) {
              v7 = dcgettext(NULL,"warning: ignoring --context",5);
              error(0,*__errno_location(),v7);
            }
          }
        }
        else if (optarg)
          error(0,0,dcgettext(NULL,"warning: ignoring --context; it requires an SELinux-enabled kernel",5));
        break;
      case 0x61:
        v28 = 1;
        v17 = 2;
        v25 = 1;
        v26 = 0x101;
        v30 = 1;
        if (dat_2515a)
          v31 = '\x01';
        v33 = 1;
        v34 = 0x101;
        break;
      case 0x62:
        v1 = 1;
        v7 = optarg;
        if (!optarg)
          v7 = v12;
        v12 = v7;
        break;
      case 100:
        v28 = 1;
        v17 = 2;
        break;
      case 0x66:
        v22 = 1;
        break;
      case 0x69:
        v18 = 3;
        break;
      case 0x6c:
        v23 = '\x01';
        break;
      case 0x6e:
        v18 = 2;
        break;
      case 0x70:
label_41e0:
        v25 = 1;
        v26 = 0x101;
        v30 = 1;
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
        v24 = 1;
        break;
      case 0x80:
        v29 = 0;
        break;
      case 0x81:
        v16 = 1;
        break;
      case 0x82:
        sub_4a80(optarg,&v10,0);
        break;
      case 0x83:
        dat_25159 = 1;
        break;
      case 0x84:
        if (optarg) {
          sub_4a80(optarg,&v10,1);
          v30 = 1;
          break;
        }
        goto label_41e0;
      case 0x85:
        if (optarg) // branch-flip
          v38 = *(int *)(sub_d220("--reflink",optarg,(long *)0x24860,(void *)0x1cad0,4,dat_25028,1) * 4 + 0x1cad0);
        else {
          v38 = 2;
        }
        break;
      case 0x86:
        v19 = *(int *)(sub_d220("--sparse",optarg,(long *)0x24880,(void *)0x1cae0,4,dat_25028,1) * 4 + 0x1cae0);
        break;
      case 0x87:
        dat_25158 = 1;
        break;
      case 0x88:
        goto label_4220;
      
    }
    goto label_4188;
  }
  if ((v23) && (v35)) {
    v7 = "cannot make both hard and symbolic links";
    goto label_4754;
  }
  if (v18 != 2) { // branch-flip
    if (!v1) goto label_4679;
    if ((v38 == 2) && (v19 != 2)) goto label_4748;
    v7 = dcgettext(NULL,"backup type",5);
    v10 = sub_db20(v7,v12);
  }
  else {
    v36 = 0;
    if (v1) {
      v7 = "options --backup and --no-clobber are mutually exclusive";
      goto label_4754;
    }
label_4679:
    v10 = 0;
    if ((v38 == 2) && (v10 = 0, v19 != 2)) {
label_4748:
      v7 = "--reflink can be used only with --sparse=auto";
label_4754:
      error(0,0,dcgettext(NULL,v7,5));
label_4769:
      sub_57e0(1);
    }
  }
  sub_d750(v13);
  if (v17 != 1) { // branch-flip
label_456f:
    if (v34._1_1_) goto label_46e5;
  }
  else {
    if ((!v34._1_1_) || (v23)) {
      v17 = 4;
      goto label_456f;
    }
    v17 = 2;
label_46e5:
    v20 = v16;
  }
  if (v14 || v27) { // branch-flip
    if (v32) { // branch-flip
      if (v31) {
        error(1,0,dcgettext(NULL,"cannot set target context and preserve it",5));
        return v4;
      }
      if (!dat_2515a) {
        error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5)); // return-dupe
        return v3;
      }
    }
    else {
      v31 = '\0';
    }
    if ((v14) && (setfscreatecon(v14) < 0)) {
      v8 = sub_14e00(v14);
      v7 = dcgettext(NULL,"failed to set default file creation context to %s",5);
      error(1,*__errno_location(),v7,v8);
      return v6;
    }
  }
  else if ((v32) && (!dat_2515a)) {
    error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5));
    return v3;
  }
  sub_bfe0();
  v9 = (long)optind;
  argc -= optind;
  return (sub_5ae0(argc,&argv[v9],v11,(unsigned char)v15,&v10) ^ 1) & 0xff;
label_4220:
  v21 = 1;
  goto label_4188;
}


// Function: sub_49f0 @ 0x49f0
void sub_49f0(unsigned long *a0)
{
  char *v1; // rax
  
  sub_83b0(a0);
  *(unsigned int *)((long)a0 + 0x44) = 1;
  a0[1] = 0x200000004;
  a0[2] = 0x100000000;
  *(unsigned int *)((long)a0 + 0x1c) = 0;
  *(char *)&a0[4] = 0;
  *(unsigned int *)((long)a0 + 4) = 1;
  *(unsigned short *)&a0[3] = 0;
  a0[5] = 0;
  a0[6] = 0x100;
  *(unsigned int *)&a0[7] = 0;
  *(unsigned short *)((long)a0 + 0x3c) = 0;
  v1 = getenv("POSIXLY_CORRECT");
  a0[9] = 0;
  a0[10] = 0;
  *(bool *)((long)a0 + 0x3e) = v1 != NULL;
}


// Function: sub_4a80 @ 0x4a80
void sub_4a80(char *a0,unsigned long a1,bool a2)
{
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  long v4; // rax
  
  v1 = (char *)sub_17230(a0);
  v2 = "--no-preserve";
  if (a2)
    v2 = "--preserve";
  v3 = strchr(v1,0x2c);
  if (v3)
    *v3 = '\0';
  v4 = sub_d220(v2,v1,(long *)0x24460,(void *)0x1cab0,4,dat_25028);
  if (*(unsigned int *)(v4 * 4 + 0x1cab0) <= 6) {
    (*(void *)((long)*(int *)((unsigned long)*(unsigned int *)(v4 * 4 + 0x1cab0) * 4 + 0x1c990) + 0x1c990))(); // jump-as-call
    return;
  }
  abort(); // no-return
}


// Function: sub_4bf0 @ 0x4bf0
unsigned long sub_4bf0(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,long a6) // return-dupe x2
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
  long v6;
  char v7;
  int v8;
  unsigned int v9;
  
  v6 = a6;
  v23 = &v18;
  v21 = &v18;
  v30 = a3;
  v32 = a4;
  v34 = a5;
  v10 = sub_e750(a0);
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
  *(unsigned long *)((long)v21 + v3 + -8) = 0x4ce5;
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
  *(unsigned long *)&v25[v3 + -8] = 0x4d4f;
  memcpy(v12,v36,v10);
  v27 = (char *)((long)v12 + a1);
  *(char *)((long)v12 + v10) = 0;
  v7 = *v27;
  while (v7 == '/') {
    v27 = &v27[1];
    v7 = *v27;
  }
  v33 = &v19;
  *(unsigned long *)&v25[v3 + -8] = 0x4d84;
  if (!fstatat(a2,v27,v33,0)) {
    if ((v19._24_4_ & 0xf000) == 0x4000) {
      *v34 = '\0';
      return 1;
    }
label_4c55:
    *(unsigned long *)&v25[v3 + -8] = 0x5206;
    v27 = (char *)sub_14b40(4,v12);
    *(unsigned long *)&v25[v3 + -8] = 0x521c;
    v13 = dcgettext(NULL,"%s exists but is not a directory",5);
    *(unsigned long *)&v25[v3 + -8] = 0x522d;
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
    *(unsigned long *)&v25[v3 + -8] = 0x4dda;
    v13 = strchr(v13,0x2f);
    if (!v13)
      return 1;
    *v13 = '\0';
    *(unsigned long *)&v25[v3 + -8] = 0x4dfd;
    v8 = fstatat(a2,v27,v33,0);
    if ((v8) || (*(unsigned int *)(v6 + 0x1c) & 0xffffff00)) {
      *(unsigned long *)&v25[v3 + -8] = 0x4ed3;
      if (stat(v35,v18)) { // branch-flip
        *(unsigned long *)&v25[v3 + -8] = 0x4ee0;
        v28 = *__errno_location();
        if (v28) {
label_4c55:
          v36 = (void *)CONCAT44(v36._4_4_,v28);
          *(unsigned long *)&v25[v3 + -8] = 0x512c;
          v27 = (char *)sub_14b40(4,v35);
          *(unsigned long *)&v25[v3 + -8] = 0x5142;
          v13 = dcgettext(NULL,"failed to get attributes of %s",5);
          *(unsigned long *)&v25[v3 + -8] = 0x5157;
          error(0,(int)v36,v13,v27);
          return 0;
        }
      }
      else if ((v20._24_4_ & 0xf000) != 0x4000) {
        v28 = 0x14;
        goto label_4c55;
      }
      *(unsigned long *)&v25[v3 + -8] = 0x4ef6;
      v14 = (unsigned long *)sub_16e90(0xa8);
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
      if (!v8) goto label_4e17;
      v1 = *(unsigned int *)&v14[3];
      v31 = v14;
      *(unsigned long *)&v25[v3 + -8] = 0x4fb7;
      v7 = sub_7ff0(v35,v36,v1,1,v6);
      if (!v7)
        return 0;
      *v34 = '\x01';
      v9 = *(unsigned int *)&v31[3];
      if (*(char *)(v6 + 0x1d)) { // branch-flip
        v29 = v9 & 0x3f;
        v16 = ~v29;
      }
      else if (*(char *)(v6 + 0x1e)) { // branch-flip
        v29 = v9 & 0x12;
        v16 = ~v29;
      }
      else {
        v16 = 0xffffffff;
        v29 = 0;
      }
      if (*(char *)(v6 + 0x20))
        v9 = 0x1ff;
      *(unsigned long *)&v25[v3 + -8] = 0x5014;
      if (mkdirat(a2,v27,v9 & v16 & 0xfff)) {
        *(unsigned long *)&v25[v3 + -8] = 0x52a4;
        v15 = sub_14b40(4,v36);
        v27 = "cannot make directory %s";
        goto label_4c55;
      }
      if (v30) {
        *(unsigned long *)&v25[v3 + -8] = 0x5050;
        __printf_chk(1,v30,v35,v36);
      }
      *(unsigned long *)&v25[v3 + -8] = 0x5075;
      if (fstatat(a2,v27,v33,0x100)) {
        *(unsigned long *)&v25[v3 + -8] = 0x5251;
        v15 = sub_14b40(4,v36);
        v27 = "failed to get attributes of %s";
        goto label_4c55;
      }
      v9 = v19._24_4_;
      if (!*(char *)(v6 + 0x1e)) {
        if (~v19._24_4_ & v29) { // branch-flip
          *(unsigned long *)&v25[v3 + -8] = 0x51b1;
          v29 &= ~sub_8640();
          v14 = v31;
          v9 = v19._24_4_;
          if (!(~v19._24_4_ & v29)) goto label_509f;
        }
        else {
label_509f:
          v14 = v31;
          v9 = v19._24_4_;
          if ((v19._24_4_ & 0x1c0) == 0x1c0) goto label_50be;
        }
        *(char *)&v14[0x12] = 1;
        *(unsigned int *)&v14[3] = v29 | v9;
      }
label_50be:
      if ((v9 | 0x1c0) != v9) {
        *(unsigned long *)&v25[v3 + -8] = 0x50d5;
        if (sub_e520(a2,v27)) {
          *(unsigned long *)&v25[v3 + -8] = 0x52c6;
          v15 = sub_14b40(4,v36);
          v27 = "setting permissions for %s";
label_4c55:
          *(unsigned long *)&v25[v3 + -8] = 0x5267;
          v27 = dcgettext(NULL,v27,5);
          *(unsigned long *)&v25[v3 + -8] = 0x526f;
          v8 = *__errno_location();
          *(unsigned long *)&v25[v3 + -8] = 0x5280;
          error(0,v8,v27,v15);
          return 0;
        }
      }
      if (!*v34) goto label_4e59;
    }
    else {
label_4e17:
      *(unsigned long *)&v25[v3 + -8] = 0x4e31;
      v7 = sub_7ff0(v35,v36,0,0,v6);
      if (!v7)
        return 0;
      if ((v19._24_4_ & 0xf000) != 0x4000) {
        v12 = v36;
        goto label_4c55;
      }
      *v34 = '\0';
label_4e59:
      if ((*(long *)(v6 + 0x28)) || (*(char *)(v6 + 0x33))) {
        *(unsigned long *)&v25[v3 + -8] = 0x4e76;
        v7 = sub_8250(v36,0,v6);
        if ((!v7) && (*(char *)(v6 + 0x34)))
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


// Function: sub_52e0 @ 0x52e0
unsigned long sub_52e0(char *a0,int a1,unsigned long a2,long a3,long a4) // return-dupe
{
  char v1;
  char *v10;
  unsigned long v11; // rdx
  unsigned long *v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long *v14;
  char *v15; // rsp
  unsigned long *v16; // rsp
  char v17 [16];
  unsigned long v18; // stack - 0x70
  unsigned long v19; // stack - 0x60
  int v2;
  unsigned long v20; // stack - 0x58
  unsigned long v21; // stack - 0x50
  unsigned int v3;
  unsigned int v4;
  long v5;
  unsigned long *v6;
  unsigned long v7; // rax
  char *v8; // rax
  unsigned long v9; // rax
  
  v14 = &v12;
  v18 = a2;
  v7 = strlen(a0);
  v16 = &v12;
  v6 = &v12;
  while (v16 != (unsigned long *)((long)&v12 - (v7 + 0x18 & 0xfffffffffffff000))) {
    v14 = (char *)((long)v6 + -0x1000);
    v15 = (char *)((long)v6 + -0x1000);
    *(unsigned long *)((long)v6 + -8) = *(unsigned long *)((long)v6 + -8);
    v16 = (char *)((long)v6 + -0x1000);
    v6 = (unsigned long *)v15;
  }
  v11 = (unsigned long)((unsigned int)(v7 + 0x18) & 0xff0);
  v5 = -v11;
  if (v11)
    *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
  *(unsigned long *)((long)v14 + v5 + -8) = 0x5370;
  v8 = memcpy((void *)((unsigned long)((long)v14 + v5 + 0xf) & 0xfffffffffffffff0),a0,v7 + 1);
  v10 = &v8[v18 - (long)a0];
  if (!a3)
    return 1;
  v12 = &v13;
  do {
    v1 = *(char *)(a4 + 0x1f);
    v8[*(long *)(a3 + 0x98)] = '\0';
    if (v1) {
      *(unsigned long *)((long)v14 + v5 + -8) = 0x53f5;
      v17 = sub_15960(a3);
      v19 = SUB168(v17,8);
      v13 = SUB168(v17,0);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5405;
      v17 = sub_15980(a3);
      v21 = SUB168(v17,8);
      v20 = SUB168(v17,0);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x541e;
      if (utimensat(a1,v10,v12,0)) {
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5433;
        v9 = sub_14b40(4,v8);
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5449;
        v10 = dcgettext(NULL,"failed to preserve times for %s",5);
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5451;
        v2 = *__errno_location();
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5462;
        error(0,v2,v10,v9);
        return 0;
      }
    }
    if (*(char *)(a4 + 0x1d)) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x20);
      v3 = *(unsigned int *)(a3 + 0x1c);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5481;
      if (!sub_e540(a1,v10,v3,v4)) goto label_53a3;
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5491;
      if (!sub_8400(a4)) {
        v18 &= 0xffffffffffffff00;
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5592;
        v9 = sub_14b40(4,v8);
        v10 = "failed to preserve ownership for %s";
        goto label_5526;
      }
      v4 = *(unsigned int *)(a3 + 0x20);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x54ac;
      sub_e540(a1,v10,0xffffffff,v4);
      v1 = *(char *)(a4 + 0x1e);
    }
    else {
label_53a3:
      v1 = *(char *)(a4 + 0x1e);
    }
    if (v1) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x18);
      *(unsigned long *)((long)v14 + v5 + -8) = 0x54d9;
      if (sub_caa0(v10,0xffffffff,v8,0xffffffff,v4))
        return 0;
    }
    else if (*(char *)(a3 + 0x90)) {
      v4 = *(unsigned int *)(a3 + 0x18);
      v18 &= 0xffffffffffffff00;
      *(unsigned long *)((long)v14 + v5 + -8) = 0x5502;
      if (sub_e520(a1,v10,v4)) {
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5517;
        v9 = sub_14b40(4,v8);
        v10 = "failed to preserve permissions for %s";
label_5526:
        *(unsigned long *)((long)v14 + v5 + -8) = 0x552d;
        v10 = dcgettext(NULL,v10,5);
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5535;
        v2 = *__errno_location();
        *(unsigned long *)((long)v14 + v5 + -8) = 0x5546;
        error(0,v2,v10,v9);
        return v18 & 0xff;
      }
    }
    v8[*(long *)(a3 + 0x98)] = '/';
    a3 = *(long *)(a3 + 0xa0);
    if (!a3)
      return 1;
  } while( true );
}


// Function: sub_57e0 @ 0x57e0
void sub_57e0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_255a0;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",5),v2,v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n",5),v1);
    sub_4970();
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
    sub_49a0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n",5),v1);
    sub_55b0();
  }
  exit(a0); // no-return
}


// Function: sub_5ae0 @ 0x5ae0
unsigned long sub_5ae0(int a0,unsigned long *a1,long a2,bool a3,int *a4)
{
  int *v1;
  unsigned int v10; // eax
  int v11;
  unsigned int v12; // eax
  char *v13;
  unsigned long v14; // rax
  void *v15; // rax
  unsigned long v16; // rax
  unsigned long v17; // rax
  unsigned long v18; // rax
  char *v19;
  int *v2;
  unsigned long v20; // rax
  char *v21; // rcx
  unsigned long v22; // rdx
  long v23;
  unsigned char *v24; // stack - 0x128
  stat v25; // stack - 0xd8
  char *v26; // stack - 0xe0
  long v27;
  unsigned char v28; // stack - 0xea
  void *v29; // stack - 0xe8
  void *v3;
  char *v30;
  char *v31; // rsp
  unsigned char **v32;
  char *v33; // rsp
  int *v34;
  int *v35;
  long v36;
  unsigned int v37; // eax
  char **v38; // stack - 0x120
  char *v39; // stack - 0x118
  long v4;
  long v40; // stack - 0x110
  int *v41; // stack - 0x108
  unsigned int v42; // stack - 0x100
  int v43; // stack - 0xfc
  char *v5;
  void *v6;
  unsigned char v7;
  char v8; // al
  char v9;
  
  v32 = &v24;
  v28 = 0;
  v40 = a2; // branch-flip
  v41 = a4;
  v43 = a0;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (a0 == 1) {
      v18 = sub_14b40(4,(char *)*a1);
      v19 = "missing destination file operand after %s";
label_60c1:
      error(0,0,dcgettext(NULL,v19,5),v18);
      sub_57e0(1); // return-dupe, no-return
    }
    v19 = "missing file operand";
  }
  else {
    v25._24_4_ = 0;
    if (a3) { // branch-flip
      if (a2) {
        error(1,0,dcgettext(NULL,"cannot combine --target-directory (-t) and --no-target-directory (-T)",5));
        return v17;
      }
      if (2 < a0) {
        v18 = sub_14b40(4,(char *)a1[2]);
        v19 = "extra operand %s";
        goto label_60c1;
      }
    }
    else {
      if (a2) {
        v10 = sub_15a20(a2,&v25);
        v42 = v10;
        if (!sub_15a10(v10)) {
          v18 = sub_14b40(4,v40);
          v19 = dcgettext(NULL,"target directory %s",5);
          error(1,*__errno_location(),v19,v18);
          return v20;
        }
        if (v43 != 1) {
label_5f87:
          v34 = v41;
          sub_8330(v41);
          sub_8370(v34);
        }
label_5b81:
        v38 = &v26;
        v23 = 0;
        v37 = 1;
        v39 = &v27;
        v24 = &v28;
        do {
          v19 = (char *)a1[v23];
          v26 = NULL;
          if (dat_25158) { // branch-flip
            *(unsigned long *)((long)v32 + -8) = 0x5e08;
            sub_e800(v19);
            if (!dat_25159) goto label_5be5;
label_5e18:
            *(unsigned long *)((long)v32 + -8) = 0x5e1d;
            v14 = strlen(v19);
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v5 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v5[-0x1000];
              v33 = &v5[-0x1000];
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
              v32 = (char *)&v5[-0x1000];
              v5 = v33;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v4 = -v22;
            v32 = (char *)&v30[v4];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v4 + -8] = 0x5e7a;
            v13 = memcpy((void *)((unsigned long)&v30[v4 + 0xf] & 0xfffffffffffffff0),v19,v14 + 1);
            *(unsigned long *)&v30[v4 + -8] = 0x5e85;
            sub_e800(v13);
            *(unsigned long *)&v30[v4 + -8] = 0x5e9b;
            v36 = v40;
            v15 = (void *)sub_ec90(v40,v13,v38);
            v13 = NULL;
            v11 = v41[0xf];
            *(long *)&v30[v4 + -8] = v36;
            if ((char)v11)
              v13 = "%s -> %s\n";
            *(int **)&v30[v4 + -0x10] = v41;
            *(unsigned long *)&v30[v4 + -0x18] = 0x5ede;
            v8 = sub_4bf0(v15,(long)v26 - (long)v15,v42,v13,&v29,v24);
            v9 = *v26;
            while (v9 == '/') {
              v26 = &v26[1];
              v9 = *v26;
            }
            if (v8) goto label_5c8c;
            v37 = 0;
            v32 = (unsigned char **)&v30[v4];
            v9 = dat_25159;
label_5f1b:
            v6 = v29;
            if (v9) {
              while (v6) {
                v3 = *(void **)((long)v6 + 0xa0);
                *(unsigned long *)((long)v32 + -8) = 0x5f4e;
                v29 = v3;
                free(v6);
                v6 = v3;
              }
            }
          }
          else {
            if (dat_25159) goto label_5e18;
label_5be5:
            *(unsigned long *)((long)v32 + -8) = 0x5bea;
            v13 = (char *)sub_db60(v19);
            *(unsigned long *)((long)v32 + -8) = 0x5bf5;
            v14 = strlen(v13);
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v5 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v5[-0x1000];
              v31 = &v5[-0x1000];
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
              v32 = (char *)&v5[-0x1000];
              v5 = v31;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v4 = -v22;
            v32 = (char *)&v30[v4];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v4 + -8] = 0x5c52;
            v13 = memcpy((void *)((unsigned long)&v30[v4 + 0xf] & 0xfffffffffffffff0),v13,v14 + 1);
            *(unsigned long *)&v30[v4 + -8] = 0x5c5d;
            sub_e800(v13);
            *(unsigned long *)&v30[v4 + -8] = 0x5c6c;
            v11 = strcmp(v13,"..");
            *(unsigned long *)&v30[v4 + -8] = 0x5c89;
            v15 = (void *)sub_ec90(v40,&v13[v11 == 0],v38);
label_5c8c:
            *(unsigned long *)((long)v32 + -8) = 0;
            *(char **)((long)v32 + -0x10) = v39;
            *(unsigned long *)((long)v32 + -0x18) = 0x5cbb;
            v12 = sub_bae0(v19,v15,v42,v26,v28,v41);
            v37 &= v12;
            if (dat_25159) {
              *(unsigned long *)((long)v32 + -8) = 0x5f7b;
              v12 = sub_52e0(v15,v42,v26,v29,v41);
              v37 &= v12;
              v9 = dat_25159;
              goto label_5f1b;
            }
          }
          v23 += 1;
          *(unsigned long *)((long)v32 + -8) = 0x5cd9;
          free(v15);
        } while ((int)v23 < v43);
        return (unsigned long)v37; // return-dupe
      }
      v23 = a1[(long)a0 + -1];
      v40 = v23;
      v42 = sub_15a20(v23,&v25);
      v37 = sub_15a10(v42);
      if ((char)v37) { // branch-flip
        v43 = a0 + -1;
        if (v23) {
          if (1 < v43) goto label_5f87;
          if (v43 != 1) {
            return (unsigned long)v37;
          }
          goto label_5b81;
        }
      }
      else {
        v11 = *__errno_location();
        if (v11 == 2)
          v28 = 1;
        if (3 <= v43) {
          v19 = (char *)sub_14b40(4,v40);
          error(1,v11,dcgettext(NULL,"target %s",5),v19);
          return v16;
        }
      }
    }
    v7 = v28;
    v19 = (char *)*a1;
    v13 = (char *)a1[1];
    if (!dat_25159) {
      if (((((*(char *)((long)v41 + 0x16)) && (*v41)) && (!strcmp(v19,v13))) && ((!v7 && ((v25._24_4_ || (!stat(v13,&v25))))))) && (v34 = v41, (v25._24_4_ & 0xf000) == 0x8000)) {
        v13 = (char *)sub_daa0(0xffffff9c,v13,*v41);
        v23 = 0x16;
        v35 = (int *)0x25100;
        while (v23) {
          v2 = &v35[1];
          v1 = &v34[1];
          *v35 = *v34;
          v23 -= 1;
          v34 = v1;
          v35 = v2;
        }
        dat_25100 = 0;
        v41 = (int *)0x25100;
      }
      v37 = sub_bae0(v19,v13,0xffffff9c,v13,-(unsigned int)v7,v41,&v26,NULL);
      return (unsigned long)v37;
    }
    v19 = "with --parents, the destination must be a directory";
  }
  error(0,0,dcgettext(NULL,v19,5));
  sub_57e0(1);
}


// Function: sub_61e0 @ 0x61e0
unsigned long sub_61e0(struct_4 *a0,long *a1) // return-dupe
{
  if (!a1)
    return 0;
  while ((a1[1] != a0->field_0x8 || (a1[2] != a0->field_0x0))) {
    a1 = (long *)*a1;
    if (!a1)
      return 0;
  }
  return 1;
}


// Function: sub_6220 @ 0x6220
void sub_6220(void)
{
  return;
}


// Function: sub_6230 @ 0x6230
int * sub_6230(long a0)
{
  int v1;
  unsigned long v2;
  bool v3; // al
  unsigned int v4; // eax
  int *v5; // rax
  undefined7 v6; // rax
  
  v5 = __errno_location();
  v1 = *v5;
  v2 = (unsigned long)v5 >> 8;
  v6 = (undefined7)v2;
  v3 = v1 == 1 || v1 == 0x16;
  if (v1 == 1 || v1 == 0x16) {
    v4 = (unsigned int)*(unsigned char *)(a0 + 0x1b);
    v4 ^= 1;
    v5 = (unsigned long)(unsigned long)v4;
  }
  return v5;
}


// Function: sub_6260 @ 0x6260
void sub_6260(void)
{
  sub_14b40(4); // tail-call
}


// Function: sub_6270 @ 0x6270
unsigned long sub_6270(struct_17 *a0)
{
  if (4 <= a0->field_0x0)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (3 <= a0->field_0xc - 1)
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (2 < a0->field_0x44)
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((a0->field_0x17) && (a0->field_0x3a))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a0->field_0x44 == 2) && (a0->field_0xc != 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  return 1;
}


// Function: sub_6370 @ 0x6370
unsigned long sub_6370(int a0,char *a1,unsigned int a2)
{
  int v1; // eax
  
  if ((a2 & 0xf000) == 0xa000)
    return 1;
  if (sub_16e20())
    return 1;
  v1 = faccessat(a0,a1,2,0x200);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_63e0 @ 0x63e0
void sub_63e0(long a0,char *a1,unsigned int a2,char *a3,long a4)
{
  unsigned int v1;
  unsigned int v2;
  char *v3;
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  char v7 [9];
  char v8; // stack - 0x32
  
  v1 = *(unsigned int *)(a4 + 0x18);
  if (sub_6370(a2,a3,v1)) { // branch-flip
    v5 = (char *)sub_14b40(4,a1);
    v3 = dat_255a0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: overwrite %s? ",5),v3,v5);
  }
  else {
    sub_eb60(*(unsigned int *)(a4 + 0x18),&v6);
    v2 = *(unsigned int *)(a4 + 0x18);
    v8 = 0;
    v4 = sub_14b40(4,a1);
    v3 = dat_255a0;
    if ((*(char *)(a0 + 0x18)) || (*(unsigned int *)(a0 + 0x14) & 0xffff00))
      v5 = dcgettext(NULL,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    else {
      v5 = dcgettext(NULL,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    __fprintf_chk(stderr,1,v5,v3,v4,(unsigned long)(v2 & 0xfff),v7);
  }
  sub_17540(); // tail-call
}


// Function: sub_6530 @ 0x6530
unsigned int sub_6530(long a0,char *a1,unsigned int a2,char *a3,long a4) // early-return
{
  int v1;
  unsigned int v2;
  
  if (!*(unsigned char *)(a0 + 0x18))
    __assert_fail("x->move_mode","src/copy.c",0x710,"abandon_move"); // no-return
  v1 = *(int *)(a0 + 8);
  if (v1 == 2)
    return (unsigned int)*(unsigned char *)(a0 + 0x18);
  if ((v1 != 3) && (((v1 != 4 || (!*(char *)(a0 + 0x3d))) || (v2 = *(unsigned int *)(a4 + 0x18), sub_6370(a2,a3,v2)))))
    return 0;
  return sub_63e0(a0,a1,a2,a3,a4) ^ 1;
}


// Function: sub_65e0 @ 0x65e0
void sub_65e0(void *a0,long a1,char *a2)
{
  unsigned long v1;
  void *v2; // rax
  unsigned long v3; // rsi
  
  v3 = a1 - (long)a0;
  v1 = strlen(a2) + 1;
  v2 = (void *)sub_16eb0(v1 + v3);
  memcpy((void *)((long)v2 + v3),a2,v1);
  memcpy(v2,a0,v3); // tail-call
}


// Function: sub_6640 @ 0x6640
bool sub_6640(char *a0,struct_4 *a1,int a2,char *a3)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  char *v5;
  stat v6; // stack - 0xd8
  bool v7;
  
  v7 = 0;
  v2 = strlen(a0);
  v3 = (char *)sub_db60(a3);
  v4 = strlen(v3);
  v5 = dat_25580;
  if (((strlen(dat_25580) + v4 == v2) && (!memcmp(a0,v3,v4))) && (!strcmp(&a0[v4],v5))) {
    v5 = (char *)sub_65e0(a3,&a3[strlen(a3)],v5);
    v1 = fstatat(a2,v5,&v6,0);
    free(v5);
    v7 = 0;
    if ((!v1) && (a1->field_0x8 == v6._8_8_))
      v7 = a1->field_0x0 == v6._0_8_;
  }
  return v7;
}


// Function: sub_6770 @ 0x6770
void sub_6770(int a0,unsigned long a1)
{
  ioctl(a0,0x40049409,a1 & 0xffffffff); // tail-call
}


// Function: sub_6780 @ 0x6780
int sub_6780(unsigned long a0,long a1,long *a2)
{
  int v1;
  long v2;
  long v3;
  int v4; // r12d
  
  v4 = 1;
  if ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000) {
    v3 = *(long *)(a1 + 0x30);
    v2 = v3 + 0x1ff;
    if (0 <= v3)
      v2 = v3;
    if (*(long *)(a1 + 0x40) < v2 >> 9) {
      v4 = 3;
      v3 = lseek(a0,0,3);
      *a2 = v3;
      if (v3 <= -1) {
        v1 = *__errno_location();
        if ((v1 != 6) && (v4 = 2, v1 != 0x16))
          return (unsigned int)(unsigned char)sub_61c0(v1) * 2;
      }
    }
  }
  return v4;
}


// Function: sub_6820 @ 0x6820
unsigned long sub_6820(unsigned int a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if ((!dat_25560) && (dat_25560 = calloc(dat_25018,1), !dat_25560)) {
    dat_25018 = 0x400;
    dat_25560 = (void *)0x25160;
  }
  if (!a1)
    return 1;
  while( true ) {
    v2 = dat_25018;
    if (a1 <= dat_25018)
      v2 = a1;
    v1 = sub_11320(a0,dat_25560,v2);
    if (v1 != v2) break;
    a1 -= v1;
    if (!a1)
      return 1;
  }
  return 0;
}


// Function: sub_68d0 @ 0x68d0
int sub_68d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1; // al
  int v2; // eax
  int v3; // edx
  
  v2 = fallocate(a0,3,a1,a2);
  if (v2 < 0) {
    v1 = sub_61c0(*__errno_location());
    if ((v3 == 0x26) || (v1))
      return 0;
  }
  return v2;
}


// Function: sub_6910 @ 0x6910
unsigned long sub_6910(unsigned long a0,char *a1,bool a2,long a3)
{
  long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = lseek(a0,a3,1);
  if (v1 < 0) {
    v2 = sub_14b40(4,a1);
    v3 = "cannot lseek %s";
label_69a4:
    v3 = dcgettext(NULL,v3,5);
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  if (a2) {
    if ((int)sub_68d0(a0 & 0xffffffff,v1 - a3,a3) <= -1) {
      v2 = sub_14b40(4,a1);
      v3 = "error deallocating %s";
      goto label_69a4;
    }
  }
  return 1;
}


// Function: sub_6a40 @ 0x6a40
int * sub_6a40(int a0,unsigned long a1,long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,char *a7,char *a8,unsigned long a9,long *a10,char *a11) // return-dupe x2
{
  bool v1;
  char *v10;
  char *v11;
  char *v12;
  unsigned long v13; // rax
  unsigned long v14;
  int *v15; // rdx
  unsigned long v16;
  int *v17;
  int *v18;
  int v19; // esi
  long *v2;
  unsigned char v20;
  char *v21;
  unsigned char v22; // r13b
  unsigned long v23;
  unsigned long v24; // stack - 0xb0
  unsigned long v25; // stack - 0x98
  char *v3;
  unsigned int v4;
  unsigned int v5; // eax
  long v6; // rax
  long v7;
  int *v8; // rax
  unsigned long v9;
  
  v3 = a11;
  v2 = a10;
  v10 = a8;
  v11 = a7;
  v19 = (int)a1;
  *a11 = 0;
  v25 = a9;
  *a10 = 0;
  if ((!a4) && ((char)a6)) {
    if (!a9)
      return (int *)0x1;
    v17 = (int *)(a1 & 0xffffffff);
    do {
      while( true ) {
        v14 = 0x7fffffffc0000000;
        if (v25 <= 0x7fffffffc0000000)
          v14 = v25;
        v6 = copy_file_range(a0,NULL,v19,NULL,v14,0);
        if (!v6) {
          if (*v2)
            return (int *)0x1;
          goto label_6b30;
        }
        if (v6 < 0) break;
        *v2 = *v2 + v6;
        v25 -= v6;
        if (!v25)
          return (int *)0x1;
      }
      v4 = *__errno_location();
      if ((v4 == 0x26) || (v5 = sub_61c0(v4), (char)v5)) goto label_6b30;
      if (v4 <= 0x1a) {
        if (1L << ((unsigned char)v4 & 0x3f) & 0x4440200U) goto label_6b30;
        if (v4 == 1) {
          v17 = v15;
          if (!*v2) goto label_6b30;
          goto label_6bda;
        }
      }
      if (v4 != 4) {
label_6bda:
        v10 = (char *)sub_14a70(1,4,v10);
        v11 = (char *)sub_14a70(0,4,v11);
        v12 = dcgettext(NULL,"error copying %s to %s",5);
        error(0,*v15,v12,v11,v10);
        return (int *)(unsigned long)v5;
      }
    } while( true );
  }
  if (!a9)
    return (int *)0x1;
label_6b30:
  v14 = a3;
  if (a4)
    v14 = a4;
  v23 = 0;
  v18 = NULL;
  do {
    while( true ) {
      v12 = (char *)*a2;
      if (!v12) {
        v12 = (char *)sub_16e60((long)getpagesize(),a3);
        *a2 = (long)v12;
      }
      v7 = v25;
      if (a3 <= v25)
        v7 = a3;
      v7 = read(a0,v12,v7);
      if (0 <= v7) break;
      v8 = __errno_location();
      if (*v8 != 4) {
        v11 = (char *)sub_14b40(4,v11);
        v10 = dcgettext(NULL,"error reading %s",5);
        error(0,*v8,v10,v11);
        return NULL;
      }
    }
    if (!v7) {
label_6fc8:
      if ((char)v18)
        return (int *)sub_6910(v19,v10,(unsigned char)a5,v23); // tail-call
      return (int *)0x1;
    }
    *v2 = *v2 + v7;
    v9 = v14;
    v21 = v12;
    v24 = v7;
label_6c20:
    do {
      v16 = v9;
      if (v24 < v9)
        v16 = v24;
      v22 = (unsigned char)v18;
      if (v16 && a4) {
        v9 = sub_69e0(v12,v16);
        v17 = (int *)(v9 & 0xffffffff);
        v20 = ((unsigned char)v9 ^ v22) & v23 != 0;
        if (((char)v17 == '\x01') || (v16 != v24)) {
          if (!v20) goto label_6d71;
          v1 = 0;
        }
        else {
          if (!v20) {
            v17 = NULL;
            goto label_6d13;
          }
          v1 = 1;
          v17 = NULL;
        }
label_6d2c:
        if (v22) { // branch-flip
          v4 = sub_6910(v19,v10,(unsigned char)a5,v23);
          if (!(char)v4)
            return (int *)(unsigned long)v4;
        }
        else if (v23 != sub_11320(v19,v21,v23)) {
          v13 = sub_14b40(4,v10);
          v11 = dcgettext(NULL,"error writing %s",5);
          error(0,*__errno_location(),v11,v13);
          return v18;
        }
        v21 = v12;
        v23 = v16;
        if (v1) {
          if (v16) { // branch-flip
            if (v20) {
              v18 = (int *)((unsigned long)v17 & 0xffffffff);
              v9 = 0;
              goto label_6c20;
            }
          }
          else {
            if (v20) {
              v23 = 0;
              break;
            }
            v24 = 0;
          }
          v24 -= v16;
          v18 = (int *)((unsigned long)v17 & 0xffffffff);
          v23 = 0;
        }
        else {
          v24 -= v16;
          v18 = (int *)((unsigned long)v17 & 0xffffffff);
        }
      }
      else {
        if (((v22 != 1) && (v17 = (int *)CONCAT71((undefined7)((unsigned long)v17 >> 8),v16 && a4), v16 == v24)) || (v17 = v18, !v16)) {
label_6d13:
          v23 += v16;
          v1 = 1;
          v20 = 0;
          goto label_6d2c;
        }
label_6d71:
        v23 += v16;
        if (v23 + 0x8000000000000000 < v16) {
          v11 = (char *)sub_14b40(4,v11);
          error(0,0,dcgettext(NULL,"overflow reading %s",5),v11);
          return NULL;
        }
        v24 -= v16;
        v18 = v17;
      }
      v12 = &v12[v16];
      v9 = v16;
    } while (v24);
    v25 -= v7;
    *v3 = (char)v17;
    if (!v25) {
      v18 = v17;
      goto label_6fc8;
    }
    v18 = (int *)((unsigned long)v17 & 0xffffffff);
  } while( true );
}


// Function: sub_7020 @ 0x7020
void sub_7020(int a0,unsigned int a1,char *a2,unsigned int a3)
{
  if (a0 <= -1) {
    sub_e520(a1,a2,a3); // tail-call
    return;
  }
  fchmod(a0,a3); // tail-call
}


// Function: sub_7040 @ 0x7040
unsigned long sub_7040(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,long a4)
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
  v1 = *(char *)(a4 + 0x33);
  if ((*(char *)(a4 + 0x31)) && (!*(char *)(a4 + 0x36))) {
    if (*(char *)(a4 + 0x37)) {
      v6 = sub_7330;
      v7 = NULL;
      if (!v1) {
        v6 = NULL;
        v7 = NULL;
        if (*(long *)(a4 + 0x28)) {
          v6 = sub_7330;
          v7 = NULL;
        }
      }
      goto label_70a2;
    }
    v6 = sub_7330;
    v5 = sub_7180;
    if ((!v1) && (!*(long *)(a4 + 0x28)))
      v6 = NULL;
  }
  else {
    v6 = sub_7330;
    v5 = sub_7260;
    if (!v1) {
      v6 = sub_7330;
      if (!*(long *)(a4 + 0x28))
        v6 = NULL;
      v5 = sub_7260;
    }
  }
  v9 = sub_6260;
  v10 = sub_6220;
label_70a2:
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


// Function: sub_7180 @ 0x7180
void sub_7180(void) // return-dupe
{
  int v1;
  
  v1 = *__errno_location();
  if (sub_61d0(v1))
    return;
  sub_16760(0);
}


// Function: sub_7260 @ 0x7260
void sub_7260(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,int *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  sub_16760(0,*v2,a9,&v3);
}


// Function: sub_7330 @ 0x7330
int sub_7330(char *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = strncmp(a0,"security.selinux",0x10);
  if (!v1)
    return v1;
  return (unsigned long)(attr_copy_check_permissions(a0,a1) != 0);
}


// Function: sub_7380 @ 0x7380
void sub_7380(void)
{
  char *v1; // rax
  
  if (!setfscreatecon(NULL))
    return;
  v1 = dcgettext(NULL,"failed to restore the default file creation context",5);
  error(1,*__errno_location(),v1);
}


// Function: sub_73c0 @ 0x73c0
void sub_73c0(char *a0,char *a1,char *a2)
{
  char *v1; // rax
  
  v1 = (char *)sub_14a70(1,4,a1);
  __printf_chk(1,"%s -> %s",(char *)sub_14a70(0,4,a0),v1);
  if (a2) {
    v1 = (char *)sub_14b40(4,a2);
    __printf_chk(1,dcgettext(NULL," (backup: %s)",5),v1);
  }
  putchar_unlocked(10); // tail-call
}


// Function: sub_7460 @ 0x7460
unsigned int sub_7460(char *a0,unsigned int a1,char *a2,char *a3,unsigned int a4,unsigned long a5,char a6,unsigned int a7,unsigned char a8)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  unsigned char v5; // r12b
  unsigned int v6;
  unsigned long v7; // stack - 0x50
  
  v6 = a7;
  v1 = sub_c0c0(a1,a2,a4,a5,(unsigned long)a8 << 10,(unsigned char)a6);
  if (1 <= v1) { // branch-flip
    v4 = NULL;
    if (!a0) {
      v4 = (char *)sub_65e0(a3,a5,a2,v7);
      a0 = v4;
    }
    v2 = (char *)sub_14a70(1,4,a0);
    v3 = (char *)sub_14a70(0,4,a3);
    v6 = 0;
    error(0,v1,dcgettext(NULL,"cannot create hard link %s to %s",5),v3,v2);
    free(v4);
  }
  else {
    v5 = (unsigned char)v6 & (unsigned char)((unsigned int)v1 >> 0x1f);
    v6 = CONCAT31((undefined3)((unsigned int)v6 >> 8),v5);
    if (v5) { // branch-flip
      v4 = (char *)sub_14b40(4,a3,0xffffffffffffffff);
      __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v4);
    }
    else {
      v6 = 1;
    }
  }
  return v6;
}


// Function: sub_7ff0 @ 0x7ff0
unsigned char sub_7ff0(char *a0,char *a1,unsigned long a2,unsigned char a3,long a4)
{
  unsigned char v1;
  int v2;
  long v3;
  int *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x38
  
  v1 = *(unsigned char *)(a4 + 0x33);
  if (!v1) {
    v3 = *(long *)(a4 + 0x28);
    a3 = v3 != 0 & a3;
    if ((a3) && ((int)sub_c760(v3,a1,a2) <= -1)) {
      v4 = __errno_location();
      v2 = *v4;
      if (!sub_61d0(v2)) {
        v5 = (char *)sub_14b40(4,a1);
        v6 = dcgettext(NULL,"failed to set default file creation context for %s",5);
        error(0,*v4,v6,v5);
        return a3;
      }
    }
    return 1;
  }
  if ((*(char *)(a4 + 0x31)) && (!*(char *)(a4 + 0x34))) {
    if (*(char *)(a4 + 0x37)) { // branch-flip
      if ((int)sub_158e0(a0,&v7) <= -1) goto label_807e;
      if (0 <= setfscreatecon(v7)) {
        freecon(v7); // return-dupe
        return v1;
      }
    }
    else {
      if ((int)sub_158e0(a0,&v7) <= -1) {
        v4 = __errno_location();
        v2 = *v4;
        if (sub_61d0(v2)) goto label_807e;
        goto label_8048;
      }
      if (0 <= setfscreatecon(v7)) {
        freecon(v7);
        return v1;
      }
      v4 = __errno_location();
      v2 = *v4;
      if (!sub_61d0(v2)) goto label_81fd;
    }
  }
  else {
    if ((int)sub_158e0(a0,&v7) <= -1) {
      v4 = __errno_location();
label_8048:
      v5 = (char *)sub_14b40(4,a0);
      v6 = dcgettext(NULL,"failed to get security context of %s",5);
      error(0,*v4,v6,v5);
label_807e:
      if (!*(char *)(a4 + 0x34))
        return v1;
      return 0;
    }
    if (0 <= setfscreatecon(v7)) {
      freecon(v7);
      return v1;
    }
    v4 = __errno_location();
label_81fd:
    v5 = (char *)sub_14e00(v7);
    v6 = dcgettext(NULL,"failed to set default file creation context to %s",5);
    error(0,*v4,v6,v5);
  }
  if (*(char *)(a4 + 0x34)) {
    freecon(v7);
    return 0;
  }
  freecon(v7);
  return v1;
}


// Function: sub_8250 @ 0x8250
unsigned long sub_8250(char *a0,char a1,long a2) // return-dupe
{
  int v1;
  long v2;
  unsigned int v3; // eax
  char *v4; // rax
  char *v5; // rax
  int *v6; // rax
  
  v2 = *(long *)(a2 + 0x28);
  if ((*(char *)(a2 + 0x31)) && (!*(char *)(a2 + 0x34))) {
    if (*(char *)(a2 + 0x37))
      return sub_c960(v2,a0,(unsigned char)a1); // tail-call
    v3 = sub_c960(v2,a0,(unsigned char)a1);
    if ((char)v3)
      return 1;
    v6 = __errno_location();
    v1 = *v6;
    if (sub_61d0(v1))
      return (unsigned long)v3;
  }
  else {
    if (sub_c960(v2,a0,(unsigned char)a1))
      return 1;
    v6 = __errno_location();
  }
  v4 = (char *)sub_14a70(0,4,a0);
  v5 = dcgettext(NULL,"failed to set the security context of %s",5);
  error(0,*v6,v5,v4);
  return 0;
}


// Function: sub_8330 @ 0x8330
void sub_8330(long a0)
{
  long v1; // rax
  
  v1 = sub_11cd0(0x3d,0,sub_12540,sub_12510,sub_125b0);
  *(long *)(a0 + 0x48) = v1;
  if (v1)
    return;
  sub_17250(); // no-return
}


// Function: sub_8370 @ 0x8370
void sub_8370(long a0)
{
  long v1; // rax
  
  v1 = sub_11cd0(0x3d,0,sub_124f0,sub_12510,sub_125b0);
  *(long *)(a0 + 0x50) = v1;
  if (v1)
    return;
  sub_17250(); // no-return
}


// Function: sub_83b0 @ 0x83b0
void sub_83b0(unsigned long *a0)
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


// Function: sub_8400 @ 0x8400
int * sub_8400(long a0)
{
  int v1;
  unsigned long v2;
  bool v3; // al
  unsigned int v4; // eax
  int *v5; // rax
  undefined7 v6; // rax
  
  v5 = __errno_location();
  v1 = *v5;
  v2 = (unsigned long)v5 >> 8;
  v6 = (undefined7)v2;
  v3 = v1 == 1 || v1 == 0x16;
  if (v1 == 1 || v1 == 0x16) {
    v4 = (unsigned int)*(unsigned char *)(a0 + 0x1a);
    v4 ^= 1;
    v5 = (unsigned long)(unsigned long)v4;
  }
  return v5;
}


// Function: sub_8640 @ 0x8640
unsigned int sub_8640(void)
{
  unsigned int v1; // eax
  
  if (dat_25010 != 0xffffffff)
    return dat_25010;
  v1 = umask(0);
  dat_25010 = v1;
  umask(v1);
  return v1;
}


// Function: sub_8680 @ 0x8680
unsigned int sub_8680(char *a0,char *a1,int a2,char *a3,long a4,unsigned int a5,unsigned int a6,char *a7,long *a8)
{
  char v1;
  unsigned long v10;
  char *v11;
  int *v12; // rax
  unsigned long v13;
  unsigned long v14; // rax
  unsigned long v15; // rax
  char *v16; // rax
  char *v17; // rax
  long v18; // rax
  unsigned int v19;
  char v2;
  stat v20; // stack - 0xd8
  stat v21; // stack - 0x168
  char v22; // stack - 0x1a1
  unsigned long v23; // stack - 0x1a0
  unsigned long v24; // stack - 0x190
  unsigned long v25; // stack - 0x188
  long v26;
  bool v27; // al
  bool v28; // r9b
  char v29 [16];
  long *v3;
  long v30; // stack - 0x1d8
  unsigned int v31; // stack - 0x1c4
  int v32; // stack - 0x1b8
  unsigned int v33; // stack - 0x1b0
  long v34; // stack - 0x198
  unsigned long v35; // stack - 0x180
  unsigned long v36; // stack - 0x178
  unsigned long v37; // stack - 0x170
  char v4;
  unsigned char v5; // al
  int v6; // eax
  unsigned int v7;
  int v8;
  unsigned int v9;
  
  v3 = a8;
  v11 = a7;
  v31 = *(unsigned int *)&a8[3];
  v23 = 0;
  v4 = *(char *)(a4 + 0x31);
  v1 = *(char *)(a4 + 0x35);
  v6 = sub_e880(a0,(unsigned long)(*(int *)(a4 + 4) == 2) << 0x11);
  if (v6 < 0) {
    v10 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"cannot open %s for reading",5);
    error(0,*__errno_location(),v11,v10);
    return 0;
  }
  if (fstat(v6,&v20)) { // branch-flip
    v10 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"cannot fstat %s",5);
    v8 = *__errno_location();
label_8779:
    v7 = 0;
    error(0,v8,v11,v10);
    v10 = 0;
  }
  else {
    if ((v3[1] != v20._8_8_) || (*v3 != v20._0_8_)) {
      v10 = sub_14b40(4,a0);
      v11 = dcgettext(NULL,"skipping file %s, as it was replaced while being copied",5);
      v8 = 0;
      goto label_8779;
    }
    v2 = *v11;
    v12 = __errno_location();
    if (v2) {
label_8cdb:
      v9 = ~a6 & a5;
      if ((v1) && (!*(char *)(a4 + 0x1b)))
        v9 |= 0x80;
      v32 = sub_12710(a2,a3,0xc1,v9);
      v8 = *v12;
      v5 = -(char)(v32 >> 0x1f);
      if ((v8 != 0x11) || (!v5)) {
        v5 &= v8 == 0x15;
label_8da9:
        if (v5) { // branch-flip
          v8 = 0x15;
          if (*a1)
            v8 = (a1[strlen(a1) - 1] != '/') + 0x14;
        }
        else {
          v9 = ~a5 & v9;
          if (0 <= v32) goto label_88bc;
        }
      }
      else {
        v8 = 0x11;
        if (!*(char *)(a4 + 0x18)) {
          v18 = readlinkat(a2,a3,&v26,1);
          v32 = 0x11;
          v8 = v32;
          if (0 <= v18) {
            if (!*(char *)(a4 + 0x3e)) {
              v11 = (char *)sub_14b40(4,a1);
              error(0,0,dcgettext(NULL,"not writing through dangling symlink %s",5),v11);
              v7 = 0;
label_92ad:
              v10 = 0;
              goto label_8788;
            }
            v32 = sub_12710(a2,a3,0x41,v9);
            v8 = *v12;
            v5 = v8 == 0x15 & (unsigned char)((unsigned int)v32 >> 0x1f);
            goto label_8da9;
          }
        }
      }
label_8d49:
      v10 = sub_14b40(4,a1);
      v11 = dcgettext(NULL,"cannot create regular file %s",5);
      goto label_8779;
    }
    v32 = sub_12710(a2,a3,(-(unsigned int)(v4 == '\0') & 0xfffffe00) + 0x201);
    v8 = *v12;
    if (0 <= v32) { // branch-flip
      if ((*(long *)(a4 + 0x28)) || (*(char *)(a4 + 0x33))) {
        v7 = sub_8250(a1,0,a4);
        if (((char)v7) || (!*(char *)(a4 + 0x34))) goto label_88a0;
      }
      else {
label_88a0:
        if (*v11) goto label_8cdb;
        a6 = 0;
        v9 = 0;
label_88bc:
        if (v4) { // branch-flip
          if (*(int *)(a4 + 0x44)) {
            if (!sub_6770(v32,v6)) goto label_8de0;
            if (*(int *)(a4 + 0x44) == 2) {
              v7 = 0;
              v11 = (char *)sub_14a70(1,4,a0);
              v17 = (char *)sub_14a70(0,4,a1);
              v16 = dcgettext(NULL,"failed to clone %s from %s",5);
              error(0,*v12,v16,v17,v11);
              goto label_8ba8;
            }
          }
label_88d2:
          v33 = fstat(v32,&v21);
          if (v33) {
            v7 = 0;
            v11 = (char *)sub_14b40(4,a1);
            v17 = dcgettext(NULL,"cannot fstat %s",5);
            error(0,*v12,v17,v11);
            goto label_8ba8;
          }
          if (v21._24_4_ != (v21._24_4_ | v9)) {
            v8 = sub_7020(v32,a2,a3,v21._24_4_ | v9);
            if (v8)
              v9 = v33;
          }
          if (v4) {
            v13 = sub_7fd0(v21._56_8_,&v20);
            v30 = 0x200;
            if ((unsigned long)(v21._56_8_ - 1U) <= 0x1fffffffffffffff)
              v30 = v21._56_8_;
            v8 = sub_6780(v6);
            if (v8) { // branch-flip
              if (((v21._24_4_ & 0xf000) != 0x8000) || ((*(int *)(a4 + 0xc) != 3 && ((*(int *)(a4 + 0xc) != 2 || (v8 == 1)))))) {
                sub_e840(v6,0,0,2);
                v14 = sub_dbf0(sub_7fd0(v20._56_8_),v13,0x7fffffffffffffff);
                if (((v20._24_4_ & 0xf000) == 0x8000) && ((unsigned long)v20._48_8_ < v13))
                  v13 = v20._48_8_ + 1;
                v13 = (v13 - 1) + v14;
                v13 -= v13 % v14;
                v27 = *(int *)(a4 + 0x44) != 0;
                if (0 < (long)v13) {
                  v19 = 1;
                  if (v8 == 3) goto label_91e4;
                  v30 = 0;
                  v28 = *(int *)(a4 + 0xc) == 3;
                  goto label_8a67;
                }
                v13 = v14;
                if (v8 != 3) {
                  v30 = 0;
                  v28 = *(int *)(a4 + 0xc) == 3;
                  goto label_8a67;
                }
                v19 = 1;
label_91e4:
                v22 = '\0';
                v4 = sub_7b50(v6,v32,&v23,v13,v30,v34,v20._48_8_,v19,v27,a0,a1);
              }
              else {
                sub_e840(v6,0,0,2);
                if (v8 == 3) {
                  v27 = *(int *)(a4 + 0x44) != 0;
                  v19 = *(unsigned int *)(a4 + 0xc);
                  goto label_91e4;
                }
                v27 = *(int *)(a4 + 0x44) != 0;
                v28 = *(int *)(a4 + 0xc) == 3;
label_8a67:
                v22 = '\0';
                v4 = sub_6a40(v6,v32,&v23,v13,v30,v28,v27,a0,a1,0xffffffffffffffff,&v24,&v22);
              }
              if (v4 != '\x01') goto label_8eb0;
              if ((!v22) || (0 <= (int)ftruncate(v32,v24))) goto label_8ab7;
              v10 = sub_14b40(4,a1);
              v11 = "failed to extend %s";
            }
            else {
              v10 = sub_14b40(4,a0);
              v11 = "cannot lseek %s";
            }
            v11 = dcgettext(NULL,v11,5);
            error(0,*v12,v11,v10);
label_8eb0:
            v7 = 0;
            goto label_8ba8;
          }
label_8ab7:
          if (*(char *)(a4 + 0x1f)) goto label_8e06;
label_8ad0:
          if (*(char *)(a4 + 0x1d)) {
            if ((*(int *)((long)v3 + 0x1c) != v21._28_4_) || (v21._32_4_ != (int)v3[4])) {
              v7 = 0;
              v8 = sub_8430(a4,a1,a2,a3,v32,*(unsigned int *)&v3[3],*(int *)((long)v3 + 0x1c),(int)v3[4],(unsigned char)*v11,&v21);
              if (v8 == -1) goto label_8ba8;
              v7 = v31 & 0xfffff1ff;
              if (v8)
                v7 = v31;
              v31 = v7;
            }
          }
        }
        else {
label_8de0:
          if (*(char *)(a4 + 0x1d) || v9) {
            v4 = '\0';
            goto label_88d2;
          }
          v21._24_4_ = 0;
          v9 = 0;
          if (*(char *)(a4 + 0x1f)) {
label_8e06:
            v29 = sub_15960(v3);
            v35 = SUB168(v29,8);
            v25 = SUB168(v29,0);
            v29 = sub_15980(v3);
            v37 = SUB168(v29,8);
            v36 = SUB168(v29,0);
            if (sub_e900(v32,a2,a3,&v25,0)) {
              v17 = (char *)sub_14b40(4,a1);
              v16 = dcgettext(NULL,"preserving times for %s",5);
              error(0,*v12,v16,v17);
              if (*(char *)(a4 + 0x32)) goto label_8eb0;
            }
            goto label_8ad0;
          }
        }
        if ((v1) && (!sub_7040(a0,v6,a1,v32,a4)))
          v7 = *(unsigned char *)(a4 + 0x36) ^ 1;
        else {
          v7 = 1;
        }
        if (*(unsigned long *)(a4 + 0x18) & 0xff0000000000ff) { // branch-flip
          if ((sub_caa0(a0,v6,a1,v32,v31)) && (*(char *)(a4 + 0x32)))
            v7 = 0;
        }
        else if (*(char *)(a4 + 0x39)) { // branch-flip
          v19 = *(unsigned int *)(a4 + 0x10);
          if (sub_cb50(a1,v32,v19))
            v7 = 0;
        }
        else if ((*(char *)(a4 + 0x20)) && (*v11)) {
          v9 = sub_8640();
          if (sub_cb50(a1,v32,~v9 & 0x1b6))
            v7 = 0;
        }
        else if ((v9 || a6) && (~sub_8640() & a6 || v9)) {
          v9 = sub_8640();
          if (sub_7020(v32,a2,a3,a5 & ~v9)) {
            v11 = (char *)sub_14b40(4,a1);
            v17 = dcgettext(NULL,"preserving permissions for %s",5);
            error(0,*v12,v17,v11);
            if (*(char *)(a4 + 0x32))
              v7 = 0;
          }
        }
      }
label_8ba8:
      v8 = close(v32);
      v10 = v23;
      if (0 <= v8) {
        v6 = close(v6);
        goto label_8bc6;
      }
      v7 = 0;
      v11 = (char *)sub_14b40(4,a1);
      v17 = dcgettext(NULL,"failed to close %s",5);
      error(0,*v12,v17,v11);
      v10 = v23;
    }
    else {
      if (v8 == 2) goto label_8ca8;
      if (!*(char *)(a4 + 0x16)) {
        if (*v11) goto label_8cdb;
        goto label_8d49;
      }
      if (!unlinkat(a2,a3,0)) {
        if (*(char *)(a4 + 0x3c)) {
          v17 = (char *)sub_14b40(4,a1);
          __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v17);
        }
label_8ca8:
        if ((!*(long *)(a4 + 0x28)) || (v7 = sub_7ff0(a0,a1,a5,1,a4), (char)v7)) {
          *v11 = '\x01';
          goto label_8cdb;
        }
        goto label_92ad;
      }
      if (*v12 == 2) goto label_8ca8;
      v11 = (char *)sub_14b40(4,a1);
      v17 = dcgettext(NULL,"cannot remove %s",5);
      v10 = 0;
      error(0,*v12,v17,v11);
      v7 = 0;
    }
  }
label_8788:
  v6 = close(v6);
label_8bc6:
  if (v6 <= -1) {
    v15 = sub_14b40(4,a0);
    v11 = dcgettext(NULL,"failed to close %s",5);
    v7 = 0;
    error(0,*__errno_location(),v11,v15);
  }
  sub_cbd0(v10);
  return v7;
}


// Function: sub_9470 @ 0x9470
unsigned long sub_9470(char *a0,char *a1,int a2,char *a3,unsigned int a4,long *a5,long *a6,int *a7,unsigned int a8,char *a9,char *a10,long *a11)
{
  int v1;
  unsigned long *v10; // rax
  char *v11; // rax
  char *v12; // rax
  unsigned long v13;
  unsigned long v14; // rax
  void *v15; // rax
  char *v16; // rax
  long *v17;
  unsigned int v18;
  stat *v19;
  int *v2;
  char v20 [4];
  long v21; // stack - 0x288
  char v22 [144]; // stack - 0x1f8
  char v23; // stack - 0x289
  stat v24; // stack - 0x168
  stat v25; // stack - 0xd8
  bool v26; // stack - 0x28a
  char *v27;
  char *v28;
  char *v29;
  char v3;
  char *v30;
  char *v31;
  char *v32; // rsp
  char *v33; // rsp
  long v34;
  char v35;
  undefined4 v36;
  int v37;
  unsigned long v38; // r14
  unsigned char v39;
  unsigned char v4; // al
  unsigned int v40;
  char *v41;
  bool v42; // r8b
  unsigned char v43;
  char v44 [16];
  unsigned int v45; // stack - 0x304
  long v46; // stack - 0x300
  unsigned int v47; // stack - 0x2f8
  unsigned char v48; // stack - 0x2f1
  long *v49; // stack - 0x2f0
  char v5; // al
  char *v50; // stack - 0x2e8
  char *v51; // stack - 0x2e0
  unsigned long *v52; // stack - 0x2d8
  long *v53; // stack - 0x2d0
  long *v54; // stack - 0x2c8
  long *v55; // stack - 0x2c0
  unsigned long v56; // stack - 0x2b8
  char *v57; // stack - 0x2b0
  char *v58; // stack - 0x2a8
  int v59; // stack - 0x2a0
  int v6;
  unsigned int v60; // stack - 0x29c
  unsigned long v61; // stack - 0x280
  unsigned long v62; // stack - 0x278
  unsigned int v63; // stack - 0x270
  unsigned int v64; // stack - 0x26c
  unsigned int v65; // stack - 0x268
  unsigned long v66; // stack - 0x260
  unsigned long v67; // stack - 0x258
  int *v7; // rax
  char *v8;
  unsigned long v9; // rax
  
  v2 = a7;
  v28 = v20;
  v27 = v20;
  v33 = v20;
  v29 = v20;
  v30 = v20;
  v31 = v20;
  v53 = a6;
  v51 = a9;
  v56 = CONCAT44(v56._4_4_,a8);
  v50 = a10;
  v37 = a7[0x10];
  v55 = a11;
  v52 = (unsigned long *)CONCAT71(v52._1_7_,(char)a8);
  *a10 = 0;
  v26 = 0 < (int)a4;
  v57 = a1;
  v49 = a5;
  v58 = a3;
  v59 = a2;
  if (!(char)a7[6]) {
    if (!v37) goto label_9531;
label_9560:
    if ((v37 == 0x11) && (v2[2] == 2)) {
      if (!(char)v56) goto label_9767;
      goto label_95c4;
    }
    v6 = -100;
    v8 = a0;
    v41 = a0;
label_9575:
    v1 = v2[1];
    if (!sub_6360(v6,v8,&v21,(unsigned long)(v1 == 2) << 8)) {
      v60 = v63;
      if (((v63 & 0xf000) != 0x4000) || (v41 = NULL, (char)v2[0xe])) goto label_95b7;
      v9 = sub_14b40(4,a0);
      v41 = NULL;
      if (*(char *)((long)v2 + 0x19)) {
        v8 = "omitting directory %s";
        v8 = dcgettext(NULL,v8,5); // return-dupe
        error(0,0,v8,v9); // return-dupe
        return (unsigned long)v41 & 0xffffffff; // return-dupe
      }
      v8 = dcgettext(NULL,"-r not specified; omitting directory %s",5);
      error(0,0,v8,v9);
      return (unsigned long)v41 & 0xffffffff;
    }
label_a440:
    *(unsigned long *)&v28[-8] = 0xa44a;
    v9 = sub_14b40(4,v41);
    v8 = "cannot stat %s";
label_a459:
    v41 = NULL;
    *(unsigned long *)&v28[-8] = 0xa463;
    v8 = dcgettext(NULL,v8,5);
    *(unsigned long *)&v28[-8] = 0xa46b;
    v37 = *__errno_location();
    *(unsigned long *)&v28[-8] = 0xa47c;
    error(0,v37,v8,v9);
    return (unsigned long)v41 & 0xffffffff;
  }
  if (v37 < 0) {
    if (!sub_14e40(0xffffff9c,a0,a2,a3,1)) {
      v26 = 1;
      a4 = 1;
      *(char *)v55 = '\x01';
      goto label_9531;
    }
    v37 = *__errno_location();
  }
  v26 = v37 == 0;
  *(bool *)v55 = v26;
  a4 = (unsigned int)v26;
  if (v37) goto label_9560;
label_9531:
  if (!*(char *)((long)v2 + 0x3f)) {
    v37 = 0;
    v8 = v58;
    v41 = v57;
    v6 = v59;
    goto label_9575;
  }
  v37 = 0;
label_95b7:
  if (!(char)v56) {
label_9767:
    v6 = v2[1];
    v48 = sub_7fb0(v6,0);
    if ((int)a4 > 0) {
      v54 = NULL;
      goto label_9668;
    }
label_9858:
    if ((v37 == 0x11) && (v2[2] == 2)) {
      v23 = '\0';
      v3 = '\0';
      goto label_9935;
    }
    v18 = v60 & 0xf000;
    if ((((v18 == 0x8000) || (v39 = (v18 == 0xa000 || v18 == 0x4000) | *(unsigned char *)&v2[5] ^ 1, !v39)) && (v39 = *(unsigned char *)&v2[6], !v39)) && ((v39 = *(unsigned char *)((long)v2 + 0x3a), !v39 && (v39 = *(unsigned char *)((long)v2 + 0x17), !v39)))) {
      if (*v2) {
        v39 = 1;
        v36 = 0x100;
        goto label_98a2;
      }
      v39 = *(unsigned char *)((long)v2 + 0x15);
      if (v39) goto label_989c;
      v36 = 0;
      if (!a4) goto label_98a2;
label_a36b:
      v26 = 1;
label_a372:
      v54 = NULL;
      v3 = 0;
      v54._0_1_ = '\0';
      v27 = v20;
      if (v37 == 0x11) {
        v37 = v2[2];
        goto label_98db;
      }
label_a38a:
      if ((char)v56) goto label_9627;
      goto label_9668;
    }
label_989c:
    v36 = 0x100;
label_98a2:
    v41 = v22;
    v54 = (long *)CONCAT71(v54._1_7_,v39);
    v6 = sub_6360(v59,v58,v41,v36);
    if (v6) {
      v7 = __errno_location();
      if (*v7 != 0x28) { // branch-flip
        if (*v7 == 2) goto label_a36b;
      }
      else if (*(char *)((long)v2 + 0x16)) goto label_a372;
      v41 = NULL;
      v8 = (char *)sub_14b40(4,v57);
      v11 = dcgettext(NULL,"cannot stat %s",5);
      error(0,*v7,v11,v8);
      return (unsigned long)v41 & 0xffffffff;
    }
    v37 = v2[2];
label_98db:
    v23 = '\0';
    v3 = (char)v54;
    if (v37 != 2) {
      v8 = &v23;
      v18 = sub_75b0(a0,&v21,v59,v58,v22,v2);
      v41 = (char *)(unsigned long)v18;
      v3 = (char)v54;
      if ((char)v18) goto label_9935;
      v9 = sub_14a70(1,4,v57,v8);
      v14 = sub_14a70(0,4,a0);
      v8 = "%s and %s are the same file";
      goto label_abe0;
    }
label_9935:
    if (!*(char *)((long)v2 + 0x3b)) {
      if ((char)v2[6]) goto label_aa71;
      if ((v60 & 0xf000) == 0x4000) goto label_a641;
label_a62c:
      if (v2[2] != 2) {
        if (v2[2] == 3) {
          v54 = (long *)CONCAT71(v54._1_7_,v3);
          v5 = sub_63e0(v2,v57,v59,v58,v22);
          v3 = (char)v54;
          if (!v5) {
            v41 = (char *)0x1; // return-dupe
            return (unsigned long)v41 & 0xffffffff;
          }
        }
        goto label_a641;
      }
      v41 = (char *)0x1;
      return (unsigned long)v41 & 0xffffffff;
    }
    if ((v60 & 0xf000) == 0x4000) {
      if (!(char)v2[6]) goto label_a641;
label_aa71:
      v54 = (long *)CONCAT71(v54._1_7_,v3);
      v18 = sub_6530(v2,v57,v59,v58,v22);
      v41 = (char *)(unsigned long)v18;
      v3 = (char)v54;
      if (!(char)v18) {
label_a641:
        if (v23) {
          v41 = (char *)0x1;
          return (unsigned long)v41 & 0xffffffff;
        }
        v41 = NULL;
        if ((v22._24_4_ & 0xf000) != 0x4000) { // branch-flip
          if ((v60 & 0xf000) != 0x4000) { // branch-flip
            v37 = *v2;
            if (!(char)v56) goto label_a6f6;
label_a68f:
            v54 = (long *)(CONCAT44(v54._4_4_,v60) & 0xffffffff0000f000);
            if (v37 != 3) {
              v46 = CONCAT71(v46._1_7_,v3);
              v3 = sub_ea90(*(long *)&v2[0x12],v58,v22);
              if (v3) {
                v9 = sub_14a70(1,4,a0);
                v14 = sub_14a70(0,4,v57);
                v8 = "will not overwrite just-created %s with %s";
                v31 = v20;
                goto label_abe0;
              }
              v3 = (char)v46;
              if (((int)v54 == 0x4000) || ((v22._24_4_ & 0xf000) != 0x4000)) goto label_a6f4;
              goto label_ae9c;
            }
            v37 = 3;
            v6 = 3;
            if ((char)v2[6]) goto label_b0d1;
label_b25f:
            v18 = v22._24_4_;
            v54 = (long *)CONCAT44(v54._4_4_,v6);
            v46 = CONCAT71(v46._1_7_,v3);
            v5 = sub_6190((char *)sub_db60(a0));
            v3 = (char)v46;
            if (!v5) {
              v37 = (int)v54;
              if ((v18 & 0xf000) == 0x4000) goto label_a798;
              goto label_aef9;
            }
label_a70f:
            if (((v18 & 0xf000) == 0x4000) || ((!*(char *)((long)v2 + 0x15) && ((!*(char *)((long)v2 + 0x31) || (((!(char)v2[0xc] || ((unsigned long)v22._16_8_ <= 1)) && ((v2[1] != 2 || ((v63 & 0xf000) == 0x8000)))))))))) goto label_a798;
            v54 = (long *)CONCAT71(v54._1_7_,v3);
            v37 = unlinkat(v59,v58,0);
            if (v37) {
              v7 = __errno_location();
              if (*v7 != 2) {
                v9 = sub_14b40(4,v57);
                v8 = "cannot remove %s";
                v8 = dcgettext(NULL,v8,5); // return-dupe
                error(0,*v7,v8,v9);
                return (unsigned long)v41 & 0xffffffff;
              }
            }
            v26 = 1;
            v3 = (char)v54;
            if ((char)v2[0xf]) {
              v46 = CONCAT71(v46._1_7_,(char)v54);
              v8 = (char *)sub_14b40(4,v57);
              __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v8);
              v54 = NULL;
              v3 = (char)v46;
              goto label_a7a3;
            }
          }
          else {
            if ((!(char)v2[6]) || (v37 = *v2, !v37)) {
              v9 = sub_14a70(1,4,a0);
              v14 = sub_14a70(0,4,v57);
              v8 = "cannot overwrite non-directory %s with directory %s";
              v31 = v20;
              goto label_abe0;
            }
            if ((char)v56) goto label_a68f;
label_b0d1:
            if ((v63 & 0xf000) == 0x4000) {
              v18 = v22._24_4_;
label_b487:
              if ((v18 & 0xf000) != 0x4000) {
                if (!v37) {
                  v9 = sub_14c40(0,3,v57);
                  v14 = sub_14c40(0,3,a0);
                  v8 = "cannot move directory onto non-directory: %s -> %s";
                  v31 = v20;
                  goto label_abe0;
                }
                goto label_aec6;
              }
            }
            if (v37) goto label_aec6;
          }
label_a798:
          v54 = NULL;
          v30 = v20;
        }
        else {
          if ((v60 & 0xf000) == 0x4000) {
label_a6f4:
            v37 = *v2;
label_a6f6:
            if (!(char)v2[6]) {
              v18 = v22._24_4_;
              v6 = v37;
              if (v37) goto label_b25f;
              goto label_a70f;
            }
            goto label_b0d1;
          }
label_ae9c:
          if ((!(char)v2[6]) || (v37 = *v2, !v37)) {
            v9 = sub_14b40(4,v57);
            v8 = "cannot overwrite directory %s with non-directory";
            v8 = dcgettext(NULL,v8,5);
            error(0,0,v8,v9);
            return (unsigned long)v41 & 0xffffffff;
          }
          v18 = v22._24_4_;
          if ((v63 & 0xf000) == 0x4000) goto label_b487;
label_aec6:
          v54 = (long *)CONCAT71(v54._1_7_,v3);
          v46 = CONCAT44(v46._4_4_,v37);
          v5 = sub_6190((char *)sub_db60(a0));
          v3 = (char)v54;
          if (v5) goto label_a798;
          v37 = (int)v46;
label_aef9:
          if (v37 != 3) {
            v54 = (long *)CONCAT71(v54._1_7_,v3);
            v5 = sub_6640();
            v3 = (char)v54;
            if (v5) {
              if ((char)v2[6]) // branch-flip
                v8 = dcgettext(NULL,"backing up %s might destroy source;  %s not moved",5);
              else {
                v8 = dcgettext(NULL,"backing up %s might destroy source;  %s not copied",5);
              }
              v9 = sub_14a70(1,4,a0);
              error(0,0,v8,sub_14a70(0,4,v57),v9);
              return (unsigned long)v41 & 0xffffffff;
            }
          }
          v8 = v58;
          v54 = (unsigned long)CONCAT71(v54._1_7_,v3);
          v11 = (char *)sub_da90(v59,v58,*v2);
          if (v11) { // branch-flip
            v38 = (long)v8 - (long)v57;
            v13 = strlen(v11);
            v41 = (char *)(v13 + 1);
            v13 = v38 + 0x18 + v13;
            v30 = v20;
            while (v33 != &v20[-(v13 & 0xfffffffffffff000)]) {
              v29 = &v30[-0x1000];
              v32 = &v30[-0x1000];
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
              v33 = &v30[-0x1000];
              v30 = v32;
            }
            v13 = (unsigned long)((unsigned int)v13 & 0xff0);
            v34 = -v13;
            if (v13)
              *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
            v46 = CONCAT71(v46._1_7_,(char)v54);
            v54 = (long *)((unsigned long)&v29[v34 + 0xf] & 0xfffffffffffffff0);
            *(unsigned long *)&v29[v34 + -8] = 0xafea;
            v15 = mempcpy(v54,v57,v38);
            *(unsigned long *)&v29[v34 + -8] = 0xaff8;
            memcpy(v15,v11,(unsigned long)v41);
            *(unsigned long *)&v29[v34 + -8] = 0xb000;
            free(v11);
            v30 = &v29[v34];
            v3 = (char)v46;
          }
          else {
            v46 = CONCAT71(v46._1_7_,(char)v54);
            v7 = __errno_location();
            v54 = NULL;
            v30 = v20;
            v3 = (char)v46;
            if (*v7 != 2) {
              v9 = sub_14b40(4,v57);
              v8 = "cannot backup %s";
              v8 = dcgettext(NULL,v8,5);
              error(0,*v7,v8,v9);
              return (unsigned long)v41 & 0xffffffff;
            }
          }
          v26 = 1;
        }
label_a7a3:
        v37 = 0x11;
        v27 = v30;
        goto label_a38a;
      }
      if (!v55) {
        v41 = (char *)0x1;
        return (unsigned long)v41 & 0xffffffff;
      }
      goto label_aaba;
    }
    v42 = 0;
    if ((*(char *)((long)v2 + 0x1f)) && (v42 = 1, (char)v2[6]))
      v42 = v22._0_8_ != v21;
    v54 = (long *)CONCAT71(v54._1_7_,v3);
    v37 = sub_15e70(v59,v58,v22,&v21,v42);
    v8 = v58;
    if (v37 < 0) {
      v3 = (char)v54;
      if (!(char)v2[6]) goto label_a62c;
      goto label_aa71;
    }
    if (v55)
      *(char *)v55 = '\x01';
    v11 = (char *)sub_bf60(v58,v61,v21);
    if ((!v11) || (v3 = sub_7460(NULL,v59,v11,v57,v59,v8,1,(unsigned char)*(char *)&v2[0xf],v48), v3)) {
      v41 = (char *)0x1;
      return (unsigned long)v41 & 0xffffffff;
    }
    if (*(char *)((long)v2 + 0x33))
      sub_7380();
    v41 = NULL; // return-dupe
    return (unsigned long)v41 & 0xffffffff;
  }
label_95c4:
  v34 = *(long *)&v2[0x14];
  if (v34) {
    v17 = &v21;
    if (((v60 & 0xf000) == 0x4000) || (*v2)) {
label_95f2:
      sub_ea00(v34,a0,v17);
      goto label_95fd;
    }
    v54 = &v21;
    v18 = sub_ea90(v34,a0,&v21);
    v41 = (char *)(unsigned long)v18;
    if (!(char)v18) {
      v34 = *(long *)&v2[0x14];
      v17 = v54;
      goto label_95f2;
    }
    v9 = sub_14b40(4,a0);
    v8 = "warning: source file %s specified more than once";
    v8 = dcgettext(NULL,v8,5);
    error(0,0,v8,v9);
    return (unsigned long)v41 & 0xffffffff;
  }
label_95fd:
  v6 = v2[1];
  v48 = sub_7fb0(v6,1);
  if ((int)a4 <= 0) goto label_9858;
  v54 = NULL;
  v3 = 0;
  v27 = v20;
label_9627:
  if ((*(long *)&v2[0x12]) && (v41 = NULL, !(char)v2[6])) {
    if (!*v2) {
      v19 = (stat *)v22;
      if (!v3) {
        v19 = &v25;
        *(unsigned long *)&v27[-8] = 0xaae9;
        if (fstatat(v59,v58,v19,0x100)) goto label_9668;
      }
      if ((*(unsigned int *)&v19->field_0x18 & 0xf000) == 0xa000) {
        v34 = *(long *)&v2[0x12];
        *(unsigned long *)&v27[-8] = 0xaba0;
        v3 = sub_ea90(v34,v58,v19);
        if (v3) {
          *(unsigned long *)&v27[-8] = 0xabbe;
          v9 = sub_14a70(1,4,v57);
          *(unsigned long *)&v27[-8] = 0xabd0;
          v14 = sub_14a70(0,4,a0);
          v8 = "will not copy %s through just-created symlink %s";
          v41 = NULL;
          v31 = v27;
label_abe0:
          *(unsigned long *)&v31[-8] = 0xabe7;
          v8 = dcgettext(NULL,v8,5);
          *(unsigned long *)&v31[-8] = 0xabfb;
          error(0,0,v8,v14,v9);
          return (unsigned long)v41 & 0xffffffff;
        }
      }
      goto label_9668;
    }
    if ((char)v2[0xf]) goto label_9f88;
    if (!v37) goto label_9ad8;
label_9681:
    if ((!(char)v2[0xe]) || ((v60 & 0xf000) != 0x4000)) {
      v3 = (char)v2[6];
      if (v3) {
        if (v62 == 1) {
          *(unsigned long *)&v27[-8] = 0xaca3;
          v46 = sub_bf00(v61,v21);
label_a961:
          if (v46) {
            if ((v60 & 0xf000) == 0x4000) goto label_96d5;
            *(unsigned long *)&v27[-0x10] = (unsigned long)v48;
            *(unsigned long *)&v27[-0x18] = (unsigned long)*(unsigned char *)&v2[0xf];
            *(unsigned long *)&v27[-0x20] = 1;
            *(unsigned long *)&v27[-0x28] = 0xa9bd;
            v3 = sub_7460(NULL,v59,v46,v57,v59,v58);
            if (v3) {
              v41 = (char *)0x1;
              return (unsigned long)v41 & 0xffffffff;
            }
            goto label_a9c9;
          }
          goto label_9740;
        }
        if (((char)v2[0xc]) && (!*(char *)((long)v2 + 0x17))) goto label_9f34;
        v46 = 0;
label_97c5:
        if (v37 != 0x11) { // branch-flip
label_97cf:
          if (v37 != 0x16) {
            if (v37 != 0x12) {
              *(unsigned long *)&v27[-8] = 0xa816;
              v9 = sub_14a70(1,4,v57);
              *(unsigned long *)&v27[-8] = 0xa828;
              v14 = sub_14a70(0,4,a0);
              *(unsigned long *)&v27[-8] = 0xa83e;
              v8 = dcgettext(NULL,"cannot move %s to %s",5);
label_a84a:
              *(unsigned long *)&v27[-8] = 0xa853;
              error(0,v37,v8,v14,v9);
              *(unsigned long *)&v27[-8] = 0xa866;
              sub_bea0(v61,v21);
              v41 = NULL;
              return (unsigned long)v41 & 0xffffffff;
            }
            v18 = v60 & 0xf000;
            *(unsigned long *)&v27[-8] = 0x9812;
            if (unlinkat(v59,v58,(unsigned int)(v18 == 0x4000) << 9)) {
              *(unsigned long *)&v27[-8] = 0x981b;
              v7 = __errno_location();
              if (*v7 != 2) {
                *(unsigned long *)&v27[-8] = 0xab3e;
                v9 = sub_14a70(1,4,v57);
                *(unsigned long *)&v27[-8] = 0xab50;
                v14 = sub_14a70(0,4,a0);
                *(unsigned long *)&v27[-8] = 0xab66;
                v8 = dcgettext(NULL,"inter-device move failed: %s to %s; unable to remove target",5);
                v37 = *v7;
                goto label_a84a;
              }
            }
            if (((char)v2[0xf]) && (v18 != 0x4000)) {
              *(unsigned long *)&v27[-8] = 0xaa33;
              v8 = dcgettext(NULL,"copied ",5);
              *(unsigned long *)&v27[-8] = 0xaa42;
              __printf_chk(1,v8);
              *(unsigned long *)&v27[-8] = 0xaa58;
              sub_73c0(a0,v57,v54);
            }
            v26 = 1;
            v43 = 1;
            v39 = 1;
            goto label_9aee;
          }
          *(unsigned long *)&v27[-8] = 0xacc6;
          v8 = (char *)sub_14a70(1,4,dat_25568);
          *(unsigned long *)&v27[-8] = 0xacdc;
          v11 = (char *)sub_14a70(0,4,dat_25570);
          *(unsigned long *)&v27[-8] = 0xacf2;
          v12 = dcgettext(NULL,"cannot move %s to a subdirectory of itself, %s",5);
          *(unsigned long *)&v27[-8] = 0xad06;
          error(0,0,v12,v11,v8);
          *v50 = 1;
        }
        else {
          *(unsigned long *)&v27[-8] = 0xa7ca;
          if (renameat(-100,a0,v59,v58)) {
            *(unsigned long *)&v27[-8] = 0xa7d7;
            v37 = *__errno_location();
            if (v37) goto label_97cf;
          }
label_9a66:
          if ((char)v2[0xf]) {
            *(unsigned long *)&v27[-8] = 0xac23;
            v8 = dcgettext(NULL,"renamed ",5);
            *(unsigned long *)&v27[-8] = 0xac32;
            __printf_chk(1,v8);
            *(unsigned long *)&v27[-8] = 0xac48;
            sub_73c0(a0,v57,v54);
          }
          if (*(long *)&v2[10]) {
            *(unsigned long *)&v27[-8] = 0x9a8b;
            sub_8250(v57,1,v2);
          }
          if (v55)
            *(char *)v55 = '\x01';
          if (((char)v56) && (!*(char *)((long)v2 + 0x3f))) {
            v34 = *(long *)&v2[0x12];
            *(unsigned long *)&v27[-8] = 0xab17;
            sub_ea00(v34,v58,&v21);
            v41 = (char *)(v56 & 0xff);
            return (unsigned long)v41 & 0xffffffff;
          }
        }
        v41 = (char *)0x1;
        return (unsigned long)v41 & 0xffffffff;
      }
      if (!(char)v2[0xc]) goto label_9ad8;
      if (!*(char *)((long)v2 + 0x17)) {
label_9f34:
        if ((2 <= v62) || ((((char)v56 && (v2[1] == 3)) || (v46 = 0, v2[1] == 4)))) {
          *(unsigned long *)&v27[-8] = 0xa95a;
          v46 = sub_bf60(v58,v61,v21);
          goto label_a961;
        }
        goto label_9f6b;
      }
      v46 = 0;
label_9f73:
      v43 = v26;
      v39 = v26;
      goto label_9aee;
    }
    if ((char)v56) { // branch-flip
      *(unsigned long *)&v27[-8] = 0xa92f;
      v46 = sub_bf60(v58,v61,v21);
    }
    else {
      *(unsigned long *)&v27[-8] = 0x96c4;
      v46 = sub_bf00(v61,v21);
    }
    if (!v46) {
label_9740:
      v3 = (char)v2[6];
label_9f6b:
      if (v3) goto label_97c5;
      goto label_9f73;
    }
label_96d5:
    *(unsigned long *)&v27[-8] = 0x96ef;
    v3 = sub_15130(0xffffff9c,a0,v59,v46);
    if (v3) { // branch-flip
      *(unsigned long *)&v27[-8] = 0xb206;
      v8 = (char *)sub_14a70(1,4,dat_25568);
      *(unsigned long *)&v27[-8] = 0xb21c;
      v11 = (char *)sub_14a70(0,4,dat_25570);
      *(unsigned long *)&v27[-8] = 0xb232;
      v12 = dcgettext(NULL,"cannot copy a directory, %s, into itself, %s",5);
      *(unsigned long *)&v27[-8] = 0xb246;
      error(0,0,v12,v11,v8);
      *v50 = 1;
    }
    else {
      *(unsigned long *)&v27[-8] = 0x9712;
      v3 = sub_15130(v59,v58,v59,v46);
      v8 = v57;
      if (v3) {
        *(unsigned long *)&v27[-8] = 0xb6de;
        v8 = (char *)sub_14b40(4,dat_25570);
        *(unsigned long *)&v27[-8] = 0xb6f4;
        v11 = dcgettext(NULL,"warning: source directory %s specified more than once",5);
        *(unsigned long *)&v27[-8] = 0xb705;
        error(0,0,v11,v8);
        v39 = v55 != NULL & *(unsigned char *)&v2[6];
        v41 = (char *)CONCAT71((undefined7)((unsigned long)v41 >> 8),v39);
        if (v39) {
label_aaba:
          *(char *)v55 = '\x01';
          return (unsigned long)v41 & 0xffffffff;
        }
        v41 = (char *)0x1;
        return (unsigned long)v41 & 0xffffffff;
      }
      if ((v2[1] == 4) || ((v2[1] == 3 && ((char)v56)))) goto label_9740;
      *(unsigned long *)&v27[-8] = 0xb51e;
      v11 = (char *)sub_65e0(v57,v58,v46);
      *(unsigned long *)&v27[-8] = 0xb533;
      v12 = (char *)sub_14a70(1,4,v11);
      *(unsigned long *)&v27[-8] = 0xb545;
      v8 = (char *)sub_14a70(0,4,v8);
      *(unsigned long *)&v27[-8] = 0xb55b;
      v16 = dcgettext(NULL,"will not create hard link %s to directory %s",5);
      *(unsigned long *)&v27[-8] = 0xb56f;
      error(0,0,v16,v8,v12);
      *(unsigned long *)&v27[-8] = 0xb577;
      free(v11);
    }
label_a9c9:
    v28 = v27;
    if (*(char *)((long)v2 + 0x33)) {
label_a9d3:
      *(unsigned long *)&v27[-8] = 0xa9d8;
      sub_7380();
label_a562:
      v28 = v27;
      if (!v46) {
        *(unsigned long *)&v27[-8] = 0xac8b;
        sub_bea0(v61,v21);
      }
    }
    if (v54) {
      v8 = (char *)((long)v54 + ((long)v58 - (long)v57));
      *(unsigned long *)&v28[-8] = 0xa5a1;
      if (renameat(v59,v8,v59,v58)) {
        *(unsigned long *)&v28[-8] = 0xac61;
        v9 = sub_14b40(4,v57);
        v8 = "cannot un-backup %s";
        goto label_a459;
      }
      if ((char)v2[0xf]) {
        *(unsigned long *)&v28[-8] = 0xa5c9;
        v8 = (char *)sub_14a70(1,4,v57);
        *(unsigned long *)&v28[-8] = 0xa5df;
        v11 = (char *)sub_14a70(0,4,v54);
        *(unsigned long *)&v28[-8] = 0xa5f5;
        v12 = dcgettext(NULL,"%s -> %s (unbackup)\n",5);
        *(unsigned long *)&v28[-8] = 0xa60a;
        __printf_chk(1,v12,v11,v8);
      }
    }
  }
  else {
label_9668:
    if (((char)v2[0xf]) && (!(char)v2[6])) {
label_9f88:
      if ((v60 & 0xf000) != 0x4000) {
        *(unsigned long *)&v27[-8] = 0x9fb4;
        sub_73c0(a0,v57,v54);
      }
    }
    if (v37) goto label_9681;
    if ((char)v2[6]) goto label_9a66;
label_9ad8:
    v46 = 0;
    v43 = v26;
    v39 = v26;
label_9aee:
    v40 = v60;
    v18 = v60;
    if (*(char *)((long)v2 + 0x39))
      v18 = v2[4];
    v55 = (long *)(CONCAT44(v55._4_4_,v18) & 0xffffffff00000fff);
    v29 = v27;
    if (*(char *)((long)v2 + 0x1d)) { // branch-flip
      *(unsigned long *)&v27[-8] = 0x9b8f;
      v3 = sub_7ff0(a0,v57,v60,v43,v2);
      if (v3) {
        v47 = (unsigned int)v55 & 0x3f;
        if ((v40 & 0xf000) == 0x4000) {
label_a06a:
          v8 = (char *)(unsigned long)(v40 & 0xf000);
          v17 = &v21;
          *(unsigned long *)&v27[-8] = 0xa080;
          v48 = sub_61e0(v17,v53);
          if (v48) { // branch-flip
            *(unsigned long *)&v27[-8] = 0xae5d;
            v9 = sub_14b40(4,a0);
            v8 = "cannot copy cyclic symbolic link %s";
label_ae6c:
            *(unsigned long *)&v27[-8] = 0xae73;
            v8 = dcgettext(NULL,v8,5);
            *(unsigned long *)&v27[-8] = 0xae84;
            error(0,0,v8,v9);
          }
          else {
            v30 = &v27[-0x20];
            v28 = &v27[-0x20];
            v29 = &v27[-0x20];
            *(unsigned long *)&v27[-8] = *(unsigned long *)&v27[-8];
            v10 = (unsigned long *)((unsigned long)&v27[-0x11] & 0xfffffffffffffff0);
            *v10 = v53;
            v10[1] = v61;
            v10[2] = v21;
            v52 = v10;
            if ((!v39) && ((v22._24_4_ & 0xf000) == 0x4000)) {
              if ((*(long *)&v2[10]) || (*(char *)((long)v2 + 0x33))) {
                v55 = v17;
                *(unsigned long *)&v27[-0x28] = 0xb053;
                v3 = sub_8250(v57,0,v2);
                if (!v3) {
                  v29 = &v27[-0x20];
                  if ((char)v2[0xd]) goto label_a558;
                }
              }
              v47 = 0;
              v35 = 0;
label_a240:
              v4 = v49 != NULL & *(unsigned char *)&v2[7];
              v41 = (char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),v4);
              if ((!v4) || (*v49 == v21)) {
                *(char **)&v27[-0x28] = v50;
                *(char **)&v27[-0x30] = v51;
                *(int **)&v27[-0x38] = v2;
                *(unsigned long **)&v27[-0x40] = v52;
                v55 = (long *)CONCAT71(v55._1_7_,v35);
                *(unsigned long *)&v27[-0x48] = 0xa291;
                v18 = sub_bbc0(a0,v57,v59,v58,v43);
                v41 = (char *)(unsigned long)v18;
                v35 = (char)v55;
              }
              if ((char)v56) {
                v53 = (long *)((unsigned long)v53._1_7_ << 8);
                v43 = 0;
                v52 = (unsigned long *)CONCAT71(v52._1_7_,v35);
                goto label_9c9d;
              }
              v53 = (long *)((unsigned long)v53._1_7_ << 8);
              v52 = (unsigned long *)CONCAT71(v52._1_7_,v35);
              goto label_9d14;
            }
            v53 = v17;
            v18 = ~v47 & (unsigned int)v55;
            *(unsigned long *)&v27[-0x28] = 0xa12c;
            if (mkdirat(v59,v58,v18)) { // branch-flip
              *(unsigned long *)&v27[-0x28] = 0xae39;
              v9 = sub_14b40(4,v57);
              v8 = "cannot create directory %s";
            }
            else {
              v55 = v53;
              *(unsigned long *)&v27[-0x28] = 0xa160;
              v37 = fstatat(v59,v58,(stat *)v22,0x100);
              v36 = v22._24_4_;
              if (!v37) {
                v8 = (char *)(unsigned long)(unsigned int)v22._24_4_;
                v35 = 0;
                if ((v22._24_4_ & 0x1c0) != 0x1c0) {
                  *(unsigned long *)&v27[-0x28] = 0xa1aa;
                  v37 = sub_e520(v59,v58,v22._24_4_ | 0x1c0);
                  v45 = v36;
                  v35 = 1;
                  if (v37) {
                    *(unsigned long *)&v27[-0x28] = 0xb76a;
                    v9 = sub_14b40(4,v57);
                    v8 = "setting permissions for %s";
                    v29 = &v27[-0x20];
                    goto label_a538;
                  }
                }
                v17 = v55;
                if (!*v51) {
                  v54 = v55;
                  v55 = (long *)CONCAT71(v55._1_7_,v35);
                  *(unsigned long *)&v27[-0x28] = 0xb305;
                  sub_bf60(v58,v22._8_8_,v22._0_8_);
                  *v51 = '\x01';
                  v17 = v54;
                  v35 = (char)v55;
                }
                if ((char)v2[0xf]) {
                  v55 = (long *)CONCAT71(v55._1_7_,v35);
                  v54 = v17;
                  if ((char)v2[6]) { // branch-flip
                    *(unsigned long *)&v27[-0x28] = 0xa205;
                    v8 = (char *)sub_14b40(4,v57);
                    *(unsigned long *)&v27[-0x28] = 0xa21b;
                    v11 = dcgettext(NULL,"created directory %s\n",5);
                    *(unsigned long *)&v27[-0x28] = 0xa22d;
                    __printf_chk(1,v11,v8);
                    v35 = (char)v55;
                  }
                  else {
                    *(unsigned long *)&v27[-0x28] = 0xb4ed;
                    sub_73c0(a0,v57,NULL);
                    v35 = (char)v55;
                  }
                }
                goto label_a240;
              }
              *(unsigned long *)&v27[-0x28] = 0xb3f0;
              v9 = sub_14b40(4,v57);
              v8 = "cannot stat %s";
              v29 = &v27[-0x20];
            }
label_a538:
            *(unsigned long *)&v29[-8] = 0xa53f;
            v8 = dcgettext(NULL,v8,5);
            *(unsigned long *)&v29[-8] = 0xa547;
            v37 = *__errno_location();
            *(unsigned long *)&v29[-8] = 0xa558;
            error(0,v37,v8,v9);
          }
label_a558:
          v27 = v29;
          if (*(char *)((long)v2 + 0x33)) goto label_a9d3;
          goto label_a562;
        }
label_9bba:
        v40 &= 0xf000;
        v53 = (long *)CONCAT71(v53._1_7_,*(char *)((long)v2 + 0x3a));
        if (*(char *)((long)v2 + 0x3a)) { // branch-flip
          if (*a0 != '/') {
            *(unsigned long *)&v27[-8] = 0x9bdf;
            v8 = (char *)sub_e730(v58);
            if (v59 != -100) { // branch-flip
label_9c13:
              *(unsigned long *)&v27[-8] = 0x9c1f;
              if (!stat(".",&v24)) {
                *(unsigned long *)&v27[-8] = 0xb10f;
                if ((!fstatat(v59,v8,&v25,0)) && ((v24._8_8_ != v25._8_8_ || (v24._0_8_ != v25._0_8_)))) {
                  *(unsigned long *)&v27[-8] = 0xb133;
                  free(v8);
                  *(unsigned long *)&v27[-8] = 0xb146;
                  v9 = sub_14c40(0,3,v57);
                  v8 = "%s: can make relative symbolic links only in current directory";
                  goto label_ae6c;
                }
              }
            }
            else {
              v55 = (long *)0x1c7d1;
              *(unsigned long *)&v27[-8] = 0x9c08;
              if (strcmp(".",v8)) goto label_9c13;
            }
            *(unsigned long *)&v27[-8] = 0x9c2f;
            free(v8);
          }
          v35 = *(char *)((long)v2 + 0x16);
          *(unsigned long *)&v27[-8] = 0x9c4e;
          v37 = sub_c230(a0,v59,v58,(unsigned char)v35,0xffffffff);
          v48 = 0;
          if (v37 > 0) {
            *(unsigned long *)&v27[-8] = 0xada2;
            v8 = (char *)sub_14a70(1,4,a0);
            *(unsigned long *)&v27[-8] = 0xadb8;
            v11 = (char *)sub_14a70(0,4,v57);
            *(unsigned long *)&v27[-8] = 0xadce;
            v12 = dcgettext(NULL,"cannot create symbolic link %s to %s",5);
            *(unsigned long *)&v27[-8] = 0xade3;
            error(0,v37,v12,v11,v8);
            goto label_a558;
          }
        }
        else if (*(char *)((long)v2 + 0x17)) { // branch-flip
          v13 = 1;
          if (!*(char *)((long)v2 + 0x16))
            v13 = (unsigned long)(v2[2] == 3);
          *(unsigned long *)&v27[-0x10] = (unsigned long)v48;
          *(unsigned long *)&v27[-0x18] = 0;
          *(unsigned long *)&v27[-0x20] = v13;
          *(unsigned long *)&v27[-0x28] = 0xa8b8;
          v3 = sub_7460(a0,0xffffff9c,a0,v57,v59,v58);
          v48 = 0;
          if (!v3) goto label_a558;
        }
        else if ((v40 != 0x8000) && (v43 = v40 != 0xa000 & *(unsigned char *)&v2[5], v48 = v43, !v43)) {
          if (v40 != 0x1000) { // branch-flip
            if ((v40 >> 8 & 0xbf) != 0x20 && v40 != 0xc000) { // branch-flip
              if (v40 != 0xa000) {
                *(unsigned long *)&v27[-8] = 0xb734;
                v9 = sub_14b40(4,a0);
                v8 = "%s has unknown file type";
                goto label_ae6c;
              }
              *(unsigned long *)&v27[-8] = 0xb33f;
              v8 = (char *)sub_cbf0(a0,v67);
              if (!v8) {
                *(unsigned long *)&v27[-8] = 0xb990;
                v9 = sub_14b40(4,a0);
                v8 = "cannot read symbolic link %s";
                goto label_a538;
              }
              v35 = *(char *)((long)v2 + 0x16);
              *(unsigned long *)&v27[-8] = 0xb36a;
              v37 = sub_c230(v8,v59,v58,(unsigned char)v35,0xffffffff);
              v55 = (long *)CONCAT44(v55._4_4_,v37);
              if (1 <= v37) {
                if (((*(char *)((long)v2 + 0x3b) == '\x01') && (!v39)) && ((v22._24_4_ & 0xf000) == 0xa000)) {
                  v53 = (long *)v22._48_8_;
                  *(unsigned long *)&v27[-8] = 0xb9d0;
                  if (v53 == (long *)strlen(v8)) {
                    *(unsigned long *)&v27[-8] = 0xb9f2;
                    v17 = (long *)sub_cd80(v59,v58);
                    if (v17) {
                      v53 = v17;
                      *(unsigned long *)&v27[-8] = 0xba0d;
                      if (!strcmp((char *)v17,v8)) {
                        *(unsigned long *)&v27[-8] = 0xba1d;
                        free(v53);
                        *(unsigned long *)&v27[-8] = 0xba25;
                        free(v8);
                        goto label_b8b9;
                      }
                      *(unsigned long *)&v27[-8] = 0xba87;
                      free(v53);
                    }
                  }
                }
                *(unsigned long *)&v27[-8] = 0xb38f;
                free(v8);
                *(unsigned long *)&v27[-8] = 0xb3a0;
                v8 = (char *)sub_14b40(4,v57);
                *(unsigned long *)&v27[-8] = 0xb3b6;
                v11 = dcgettext(NULL,"cannot create symbolic link %s",5);
                *(unsigned long *)&v27[-8] = 0xb3cb;
                error(0,(int)v55,v11,v8);
                goto label_a558;
              }
              *(unsigned long *)&v27[-8] = 0xb8b9;
              free(v8);
label_b8b9:
              if (*(char *)((long)v2 + 0x33)) {
                *(unsigned long *)&v27[-8] = 0xb97e;
                sub_7380();
              }
              v43 = *(unsigned char *)((long)v2 + 0x1d);
              v53 = (long *)CONCAT71(v53._1_7_,1);
              v48 = v43;
              if (v43) {
                *(unsigned long *)&v27[-8] = 0xb8fa;
                v37 = sub_e540(v59,v58,v64,v65);
                if (v37) {
                  *(unsigned long *)&v27[-8] = 0xb90a;
                  v3 = sub_8400(v2);
                  v53 = (long *)CONCAT71(v53._1_7_,v3);
                  if (v3) {
                    v48 = 0;
                    goto label_9c60;
                  }
                  *(unsigned long *)&v27[-8] = 0xba3d;
                  v8 = dcgettext(NULL,"failed to preserve ownership for %s",5);
                  *(unsigned long *)&v27[-8] = 0xba45;
                  v37 = *__errno_location();
                  *(unsigned long *)&v27[-8] = 0xba5a;
                  error(0,v37,v8,v57);
                  if (*(char *)((long)v2 + 0x32)) goto label_a558;
                }
                v43 = v48;
                v48 = 0;
                v53 = (long *)CONCAT71(v53._1_7_,v43);
              }
            }
            else {
              *(unsigned long *)&v27[-8] = 0xa50c;
              v37 = mknodat(v59,v58,~v47 & v60,v66);
              if (v37) {
                *(unsigned long *)&v27[-8] = 0xa525;
                v9 = sub_14b40(4,v57);
                v8 = "cannot create special file %s";
                goto label_a538;
              }
            }
          }
          else {
            v18 = ~v47 & v60;
            *(unsigned long *)&v27[-8] = 0xb605;
            v37 = mknodat(v59,v58,v18,0);
            if (v37) {
              *(unsigned long *)&v27[-8] = 0xb625;
              if (mkfifoat(v59,v58,v18 & 0xffffefff)) {
                *(unsigned long *)&v27[-8] = 0xb63e;
                v9 = sub_14b40(4,v57);
                v8 = "cannot create fifo %s";
                goto label_a538;
              }
            }
          }
        }
        else {
          *(long **)&v27[-0x10] = &v21;
          *(bool **)&v27[-0x18] = &v26;
          *(unsigned long *)&v27[-0x20] = (unsigned long)v47;
          *(unsigned long *)&v27[-0x28] = 0xad60;
          v48 = sub_8680(a0,v57,v59,v58,v2,(unsigned int)v55 & 0x1ff);
          if (!v48) goto label_a558;
          v53 = (long *)((unsigned long)v53 & 0xffffffffffffff00);
          v39 = v26;
        }
label_9c60:
        v43 = v40 != 0x4000;
        if (!v39) {
          v4 = (*(unsigned char *)&v2[5] ^ 1) & v43;
          if (v4) {
            if ((*(long *)&v2[10]) || (*(char *)((long)v2 + 0x33))) {
              *(unsigned long *)&v27[-8] = 0xa9fc;
              v43 = sub_8250(v57,0,v2);
              if (v43) goto label_9c83;
              v39 = 0;
              if ((char)v2[0xd]) goto label_a558;
            }
            v43 = v4;
          }
        }
label_9c83:
        v41 = (char *)0x1;
        v28 = v27;
        if ((char)v56) {
          v52 = (unsigned long *)((unsigned long)v52 & 0xffffffffffffff00);
          v41 = (char *)(v56 & 0xffffffff);
          v30 = v27;
label_9c9d:
          v28 = v30;
          if (*(long *)&v2[0x12]) {
            v56 = CONCAT71(v56._1_7_,v43);
            *(unsigned long *)&v30[-8] = 0x9ccc;
            v37 = fstatat(v59,v58,&v25,0x100);
            v43 = (unsigned char)v56;
            if (!v37) {
              v34 = *(long *)&v2[0x12];
              *(unsigned long *)&v30[-8] = 0x9ceb;
              sub_ea00(v34,v58,&v25);
              v43 = (unsigned char)v56;
            }
          }
        }
        if (((*(char *)((long)v2 + 0x17)) && (v43)) || (v48)) {
          return (unsigned long)v41 & 0xffffffff;
        }
label_9d14:
        if (*(char *)((long)v2 + 0x1f)) {
          *(unsigned long *)&v28[-8] = 0x9d2d;
          v44 = sub_15960(&v21);
          *(unsigned long *)&v28[-8] = 0x9d43;
          v25._0_8_ = SUB168(v44,0);
          v25._8_8_ = SUB168(v44,8);
          v44 = sub_15980(&v21);
          v18 = (unsigned int)(unsigned char)v53;
          *(unsigned long *)&v28[-8] = 0x9d74;
          v25._16_8_ = SUB168(v44,0);
          v25._24_8_ = SUB168(v44,8);
          if (utimensat(v59,v58,&v25,v18 << 8)) {
            *(unsigned long *)&v28[-8] = 0x9d89;
            v56 = sub_14b40(4,v57);
            *(unsigned long *)&v28[-8] = 0x9da3;
            v8 = dcgettext(NULL,"preserving times for %s",5);
            *(unsigned long *)&v28[-8] = 0x9dab;
            v37 = *__errno_location();
            *(unsigned long *)&v28[-8] = 0x9dc0;
            error(0,v37,v8,v56);
            if (*(char *)((long)v2 + 0x32)) {
              v41 = NULL;
              return (unsigned long)v41 & 0xffffffff;
            }
          }
        }
        if ((char)v53) { // branch-flip
          if (!*(char *)((long)v2 + 0x35)) {
            return (unsigned long)v41 & 0xffffffff;
          }
          *(unsigned long *)&v28[-8] = 0xa906;
          if (sub_7040(a0,0xffffffff,v57,0xffffffff,v2)) {
            return (unsigned long)v41 & 0xffffffff;
          }
          v3 = *(char *)((long)v2 + 0x36);
        }
        else {
          if (*(char *)((long)v2 + 0x1d)) {
            if (((v39) || (v22._28_4_ != v64)) || (v22._32_4_ != v65)) {
              *(char **)&v28[-8] = v22;
              *(unsigned long *)&v28[-0x10] = (unsigned long)v39;
              *(unsigned long *)&v28[-0x18] = (unsigned long)v65;
              *(unsigned long *)&v28[-0x20] = (unsigned long)v64;
              *(unsigned long *)&v28[-0x28] = 0x9e38;
              v37 = sub_8430(v2,v57,v59,v58,0xffffffff,v63);
              if (v37 == -1) {
                v41 = NULL;
                return (unsigned long)v41 & 0xffffffff;
              }
              if (!v37)
                v60 &= 0xfffff1ff;
            }
          }
          if (*(char *)((long)v2 + 0x35)) {
            *(unsigned long *)&v28[-8] = 0xb45f;
            v3 = sub_7040(a0,0xffffffff,v57,0xffffffff,v2);
            if ((!v3) && (*(char *)((long)v2 + 0x36))) {
              v41 = NULL;
              return (unsigned long)v41 & 0xffffffff;
            }
          }
          if (!(*(unsigned long *)&v2[6] & 0xff0000000000ff)) {
            if (*(char *)((long)v2 + 0x39)) // branch-flip
              v18 = v2[4];
            else {
              if (!(v39 & *(unsigned char *)&v2[8])) {
                if (v47) { // branch-flip
                  *(unsigned long *)&v28[-8] = 0xb65d;
                  v47 &= ~sub_8640();
                  if (!v47) goto label_9e9a;
                  if ((char)v52 != '\x01') {
                    if (v39) {
                      *(unsigned long *)&v28[-8] = 0xa42f;
                      if (fstatat(v59,v58,(stat *)v22,0x100)) {
                        v41 = v57;
                        goto label_a440;
                      }
                    }
                    v45 = v22._24_4_;
                    if (!(v47 & ~v22._24_4_)) {
                      return (unsigned long)v41 & 0xffffffff;
                    }
                  }
                }
                else {
label_9e9a:
                  if (!(char)v52) {
                    return (unsigned long)v41 & 0xffffffff;
                  }
                }
                *(unsigned long *)&v28[-8] = 0x9ec5;
                v37 = sub_e520(v59,v58,v45 | v47);
                if (!v37) {
                  return (unsigned long)v41 & 0xffffffff;
                }
                *(unsigned long *)&v28[-8] = 0x9ede;
                v9 = sub_14b40(4,v57);
                *(unsigned long *)&v28[-8] = 0x9ef4;
                v8 = dcgettext(NULL,"preserving permissions for %s",5);
                *(unsigned long *)&v28[-8] = 0x9efc;
                v37 = *__errno_location();
                *(unsigned long *)&v28[-8] = 0x9f0d;
                error(0,v37,v8,v9);
                goto label_9f0d;
              }
              v18 = 0x1ff;
              if ((v60 & 0x7000) != 0x4000)
                v18 = 0x1b6;
              *(unsigned long *)&v28[-8] = 0xb6c2;
              v18 = ~sub_8640() & v18;
            }
            *(unsigned long *)&v28[-8] = 0xb2d0;
            if (!sub_cb50(v57,0xffffffff,v18)) {
              return (unsigned long)v41 & 0xffffffff;
            }
            v41 = NULL;
            return (unsigned long)v41 & 0xffffffff;
          }
          *(unsigned long *)&v28[-8] = 0xb1c0;
          if (!sub_caa0(a0,0xffffffff,v57,0xffffffff,v60)) {
            return (unsigned long)v41 & 0xffffffff;
          }
label_9f0d:
          v3 = *(char *)((long)v2 + 0x32);
        }
        if (!v3) {
          return (unsigned long)v41 & 0xffffffff;
        }
      }
    }
    else if ((v60 & 0xf000) != 0x4000) { // branch-flip
      *(unsigned long *)&v27[-8] = 0x9b4f;
      v3 = sub_7ff0(a0,v57,v60,v43,v2);
      v47 = 0;
      if (v3) goto label_9bba;
    }
    else {
      *(unsigned long *)&v27[-8] = 0xa053;
      v3 = sub_7ff0(a0,v57,v60,v43,v2);
      if (v3) {
        v47 = (unsigned int)v55 & 0x12;
        goto label_a06a;
      }
    }
  }
  v41 = NULL;
  return (unsigned long)v41 & 0xffffffff;
}


// Function: sub_bae0 @ 0xbae0
void sub_bae0(unsigned long a0,unsigned long a1,unsigned int a2,char *a3,unsigned int a4,struct_17 *a5,char *a6,long *a7)
{
  char *v1;
  long *v2;
  char v3; // stack - 0x41
  
  v2 = a7;
  v1 = a6;
  if (sub_6270(a5)) {
    v3 = 0;
    dat_25568 = a1;
    dat_25570 = a0;
    sub_9470(a0,a1,a2,a3,a4,NULL,NULL,a5,1,&v3,v1,v2);
    return;
  }
  __assert_fail("valid_options (options)","src/copy.c",0xc2a,"copy"); // no-return
}


// Function: sub_bbc0 @ 0xbbc0
unsigned char sub_bbc0(char *a0,long a1,unsigned int a2,long a3,char a4,unsigned long a5,unsigned long a6,unsigned long *a7,unsigned char *a8,unsigned char *a9)
{
  unsigned long *v1;
  long v10;
  unsigned char v11; // stack - 0x9b
  unsigned char v12; // stack - 0x9a
  unsigned long v13; // stack - 0x98
  char *v14;
  unsigned char v15; // stack - 0xfa
  unsigned char v16; // stack - 0xf9
  unsigned long v17; // stack - 0x90
  unsigned long v18; // stack - 0x88
  unsigned long v19; // stack - 0x80
  unsigned char *v2;
  unsigned long v20; // stack - 0x78
  unsigned long v21; // stack - 0x70
  unsigned long v22; // stack - 0x68
  unsigned long v23; // stack - 0x60
  unsigned long v24; // stack - 0x58
  unsigned long v25; // stack - 0x50
  unsigned long v26; // stack - 0x48
  unsigned char *v3;
  unsigned char v4;
  unsigned char v5; // al
  char *v6; // rax
  void *v7; // rax
  void *v8; // rax
  unsigned long v9;
  
  v3 = a9;
  v2 = a8;
  v1 = a7;
  v9 = a6;
  v13 = *a7;
  v17 = a7[1];
  v18 = a7[2];
  v19 = a7[3];
  v20 = a7[4];
  v21 = a7[5];
  v22 = a7[6];
  v23 = a7[7];
  v24 = a7[8];
  v25 = a7[9];
  v26 = a7[10];
  v6 = (char *)sub_15610(a0,2);
  if (v6) { // branch-flip
    if (*(int *)((long)v1 + 4) == 3)
      v13 = CONCAT44(2,(unsigned int)v13);
    if (*v6) { // branch-flip
      v16 = 0;
      v15 = 1;
      v14 = v6;
      do {
        v7 = (void *)sub_ec90(a0,v14,NULL);
        v8 = (void *)sub_ec90(a1,v14,NULL);
        v12 = *v2;
        v5 = sub_9470(v7,v8,a2,(long)v8 + (a3 - a1),(unsigned char)a4,a5,v9,&v13,0,&v12,&v11,&v10);
        v4 = v11;
        v15 &= v5;
        *v3 = *v3 | v11;
        free(v8);
        free(v7);
        if (v4) break;
        v16 |= v12;
        v14 = &v14[strlen(v14) + 1];
      } while (*v14);
    }
    else {
      v16 = 0;
      v15 = 1;
    }
    free(v6);
    *v2 = v16;
  }
  else {
    v9 = sub_14b40(4,a0);
    v6 = dcgettext(NULL,"cannot access %s",5);
    error(0,*__errno_location(),v6,v9);
    v15 = 0;
  }
  return v15;
}


// Function: sub_be50 @ 0xbe50
unsigned long sub_be50(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_be60 @ 0xbe60
unsigned long sub_be60(struct_4 *a0,struct_4 *a1)
{
  if (a0->field_0x0 != a1->field_0x0)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x8 >> 8),a0->field_0x8 == a1->field_0x8);
}


// Function: sub_be80 @ 0xbe80
void sub_be80(void *a0)
{
  free(*(void **)((long)a0 + 0x10));
  free(a0); // tail-call
}


// Function: sub_bea0 @ 0xbea0
void sub_bea0(unsigned long a0,unsigned long a1) // return-dupe
{
  void *v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = (void *)sub_12350(dat_25578,&v2);
  if (!v1)
    return;
  sub_be80(v1);
}


// Function: sub_bf00 @ 0xbf00
unsigned long sub_bf00(unsigned long a0,unsigned long a1) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  
  v2 = a0;
  v3 = a1;
  v1 = sub_11a70(dat_25578,&v2);
  if (!v1)
    return 0;
  return *(unsigned long *)(v1 + 0x10);
}


// Function: sub_bf60 @ 0xbf60
unsigned long sub_bf60(char *a0,unsigned long a1,unsigned long a2) // early-return
{
  struct_13 *v1; // rax
  unsigned long v2; // rax
  struct_13 *v3; // rax
  
  v1 = (struct_13 *)sub_16e90(0x18);
  v2 = sub_17230(a0);
  *(unsigned long *)v1->field_0x0 = a1;
  v1->field_0x10 = v2;
  *(unsigned long *)((long)v1->field_0x0 + 8) = a2;
  v3 = (struct_13 *)sub_122f0(dat_25578,v1);
  if (!v3)
    sub_17250(); // no-return
  if (v1 == v3)
    return 0;
  sub_be80(v1);
  return v3->field_0x10;
}


// Function: sub_bfe0 @ 0xbfe0
void sub_bfe0(void)
{
  dat_25578 = sub_11cd0(0x67,0,sub_be50,sub_be60,sub_be80);
  if (dat_25578)
    return;
  sub_17250(); // no-return
}


// Function: sub_c020 @ 0xc020
void sub_c020(char *a0,struct_21 *a1)
{
  linkat(a1->field_0x0,a1->field_0x8,a1->field_0x10,a0,a1->field_0x14); // tail-call
}


// Function: sub_c040 @ 0xc040
char * sub_c040(void *a0,char *a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3; // rax
  
  v2 = sub_db60(a0) - (long)a0;
  v1 = v2 + 9;
  if ((0x101 <= v1) && (a1 = malloc(v1), !a1))
    return NULL;
  v3 = mempcpy(a1,a0,v2);
  *(unsigned long *)v3 = s_1d490._0_8_;
  v3[8] = s_1d490[8];
  return a1;
}


// Function: sub_c0a0 @ 0xc0a0
void sub_c0a0(char *a0,struct_23 *a1)
{
  symlinkat(a1->field_0x0,a1->field_0x8,a0); // tail-call
}


// Function: sub_c0c0 @ 0xc0c0
int sub_c0c0(int a0,char *a1,int a2,char *a3,int a4,char a5,int a6)
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
    v3 = (char *)sub_c040(a3,v4);
    v1 = (char *)v3;
    if (v3) { // branch-flip
      v5[0] = a0;
      v7 = a1;
      v8 = a2;
      v9 = a4;
      v2 = sub_15ba0(v3,0,v5,sub_c020,6);
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


// Function: sub_c230 @ 0xc230
int sub_c230(char *a0,int a1,char *a2,char a3,int a4)
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
    v1 = (char *)sub_c040(a2,v3);
    if (v1) { // branch-flip
      v2 = a0;
      v4 = a1;
      if (sub_15ba0(v1,0,&v2,sub_c0a0,6)) // branch-flip
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


// Function: sub_c370 @ 0xc370
unsigned int sub_c370(unsigned long a0,unsigned int a1,char **a2)
{
  int v1;
  short v2; // ax
  unsigned int v3; // eax
  char *v4; // rax
  int *v5; // rax
  char *v6; // stack - 0x40
  char *v7; // stack - 0x38
  
  v6 = NULL;
  v7 = NULL;
  v4 = (char *)sub_e730();
  if (((0 <= getcon(&v6)) && (0 <= (int)sub_158c0(v4,&v7))) && (v2 = mode_to_security_class(a1), v2))
    v3 = security_compute_create(v6,v7,(unsigned short)v2,a2);
  else {
    v3 = 0xffffffff;
  }
  v5 = __errno_location();
  v1 = *v5;
  free(v4);
  freecon(v6);
  freecon(v7);
  *v5 = v1;
  return v3;
}


// Function: sub_c450 @ 0xc450
int sub_c450(long a0,char *a1)
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
      if (0 <= lstat(a1,&v10)) goto label_c4cd;
      v2 = *v4;
      v6 = 0;
      v5 = 0;
    }
label_c59b:
    v3 = v1;
    goto label_c59e;
  }
  if (0 <= fstat(v1,&v10)) { // branch-flip
label_c4cd:
    if (0 <= (int)selabel_lookup(a0,&v9,a1,v10._24_4_)) { // branch-flip
      v5 = context_new(v9);
      if (v5) { // branch-flip
        if (v1 != -1) {
          if ((int)sub_15900(v1) < 0) {
            v6 = 0;
            v3 = -1;
            goto label_c6bb;
          }
          v6 = context_new(v11);
          if (v6) goto label_c531;
          v2 = *v4;
          v3 = -1;
          goto label_c55f;
        }
        if ((int)sub_158e0(a1,&v11) < 0) {
          v2 = *v4;
          v6 = 0;
          goto label_c59b;
        }
        v6 = context_new(v11);
        if (!v6) {
          v2 = *v4;
          v3 = -1;
          goto label_c59e;
        }
label_c531:
        v7 = context_type_get(v5);
        if (((v7) && (!context_type_set(v6,v7))) && (v8 = (char *)context_str(v6), v8)) {
          if (v1 == -1) {
            v1 = lsetfilecon(a1,v8);
            v2 = *v4;
            goto label_c59b;
          }
          v3 = fsetfilecon(v1,v8);
          goto label_c6bb;
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
    if (v1 == -1) goto label_c59e;
  }
  else {
    v6 = 0;
    v5 = 0;
    v3 = -1;
label_c6bb:
    v2 = *v4;
  }
label_c55f:
  close(v1);
label_c59e:
  context_free(v5);
  context_free(v6);
  freecon(v9);
  freecon(v11);
  *v4 = v2;
  return v3;
}


// Function: sub_c760 @ 0xc760
int sub_c760(unsigned long a0,char *a1,unsigned int a2)
{
  int v1; // eax
  int v10;
  int *v2; // rax
  long v3;
  long v4; // rax
  long v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x50
  char *v8; // stack - 0x48
  char *v9;
  
  v7 = NULL;
  v8 = NULL;
  v2 = __errno_location();
  if (*a1 != '/') { // branch-flip
    a1 = (char *)sub_e4c0(a1,2);
    if (!a1) {
      v10 = *v2;
      v9 = NULL;
      v4 = 0;
      v1 = -1;
      v3 = 0;
      goto label_c869;
    }
    v9 = a1;
  }
  else {
    v9 = NULL;
  }
  if (0 <= (int)selabel_lookup(a0,&v7,a1,a2)) { // branch-flip
    if (0 <= (int)sub_c370(a1,a2,&v8)) { // branch-flip
      v3 = context_new(v7);
      if (v3) { // branch-flip
        v4 = context_new(v8);
        if ((((v4) && (v5 = context_type_get(v3), v5)) && (!context_type_set(v4,v5))) && (v6 = (char *)context_str(v4), v6)) {
          v1 = setfscreatecon(v6);
          v10 = *v2;
        }
        else {
          v10 = *v2;
          v1 = -1;
        }
      }
      else {
        v10 = *v2;
        v4 = 0;
        v1 = -1;
      }
    }
    else {
      v10 = *v2;
      v4 = 0;
      v3 = 0;
      v1 = -1;
    }
  }
  else {
    v10 = *v2;
    if (v10 != 2) { // branch-flip
      v4 = 0;
      v3 = 0;
      v1 = -1;
    }
    else {
      *v2 = 0x3d;
      v10 = 0x3d;
      v4 = 0;
      v3 = 0;
      v1 = -1;
    }
  }
label_c869:
  context_free(v3);
  context_free(v4);
  freecon(v7);
  freecon(v8);
  free(v9);
  *v2 = v10;
  return v1;
}


// Function: sub_c960 @ 0xc960
unsigned long sub_c960(long a0,char *a1,bool a2)
{
  char *v1;
  int v10;
  unsigned long v11; // stack - 0x40
  int v2; // eax
  int *v3;
  struct_10 *v4; // rax
  long v5; // rax
  char *v6;
  char *v7; // stack - 0x48
  unsigned long v8; // r12
  undefined7 v9; // r12
  
  v6 = NULL;
  v9 = (undefined7)((unsigned long)a0 >> 8);
  if (*a1 != '/') { // branch-flip
    v6 = (char *)sub_e4c0(a1,2);
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
    v4 = (struct_10 *)sub_17290(&v7,0x10,0);
    do {
      v5 = sub_10b30(v4);
      while( true ) {
        if (!v5) {
          if (*v3)
            v10 = *v3;
          if (sub_10940(v4))
            v10 = *v3;
          free(v6);
          v8 = CONCAT71(v9,v10 == 0);
          return v8 & 0xffffffff;
        }
        v1 = v4->field_0x20;
        if (0 <= (int)sub_c450(a0,v1)) break;
        v10 = *v3;
        v5 = sub_10b30(v4);
      }
    } while( true );
  }
  v2 = sub_c450(a0,v7);
  v10 = *v3;
  v8 = CONCAT71(v9,v2 != -1);
  free(v6);
  *v3 = v10;
  return v8 & 0xffffffff;
}


// Function: sub_e730 @ 0xe730
void sub_e730(char *a0)
{
  if (sub_e790(a0))
    return;
  sub_17250(); // no-return
}


// Function: sub_113a0 @ 0x113a0
unsigned long sub_113a0(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2;
  unsigned long v3; // rdx
  unsigned long v4; // rsi
  long v5; // r8
  
  v1 = SUB168(ZEXT816(a0) * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v3 = a0 % 3;
  if ((10 <= a0) && (v3)) {
    v5 = 0x10;
    v4 = 9;
    v2 = 3;
    do {
      v2 += 2;
      v4 += v5;
      v1 = a0 / v2;
      v3 = a0 % v2;
      if (a0 <= v4) break;
      v5 += 8;
    } while (v3);
  }
  v2 = v1 >> 8;
  v1 = CONCAT71((undefined7)v2,v3 != 0);
  return v1;
}


// Function: sub_11410 @ 0x11410
unsigned long sub_11410(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_113a0(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}


// Function: sub_11450 @ 0x11450
unsigned long sub_11450(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_17720(a0,3) % a1;
}


// Function: sub_11470 @ 0x11470
bool sub_11470(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_11480 @ 0x11480
unsigned long sub_11480(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1d660)
    return 1;
  v1 = v2[2];
  if ((((dat_1d674 < v1) && (v1 < dat_1d678)) && (dat_1d67c < v2[3])) && (dat_1d690 <= *v2)) {
    v3 = *v2 + dat_1d674;
    if (((v3 < v2[1]) && (v2[1] <= dat_1d680)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1d660;
  return 0;
}


// Function: sub_11510 @ 0x11510
void sub_11510(long a0,struct_1 *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  *(struct_1 **)(a0 + 0x48) = a1;
}


// Function: sub_11530 @ 0x11530
long sub_11530(struct_2 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}


// Function: sub_11560 @ 0x11560
long sub_11560(long a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_11530(a0,a1);
  *a2 = v4;
  v5 = *v4;
  if (!v5)
    return 0;
  if (v5 != a1) {
    v1 = *(void **)(a0 + 0x38);
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = *(void **)(a0 + 0x38), (*v1)(a1))) break;
        v4 = (long *)v4[1];
      }
      v5 = *(long *)v4[1];
      if (!a3)
        return v5;
      v4[1] = ((long *)v4[1])[1];
      sub_11510(a0);
      return v5;
    }
    v5 = *v4;
  }
  if (a3) {
    v2 = (long *)v4[1];
    if (v2) {
      v3 = v2[1];
      *v4 = *v2;
      v4[1] = v3;
      sub_11510(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}


// Function: sub_11660 @ 0x11660
void sub_11660(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_11690 @ 0x11690
unsigned long sub_11690(long a0,struct_28 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_4 *v3; // rax
  struct_4 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_116b8:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_4 *)sub_11530(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_11510(a0);
          v6 = v1;
          if (!v1) goto label_11728;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_11728:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_116b8;
    v3 = (struct_4 *)sub_11530(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_4 *)sub_11660(a0);
      if (!v4)
        return 0;
      v2 = v3->field_0x8;
      v4->field_0x0 = v7;
      v4->field_0x8 = v2;
      v3->field_0x8 = (long)v4;
    }
    else {
      v3->field_0x0 = v7;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1->field_0x18 = a1->field_0x18 + -1;
    if (a1->field_0x8 <= v5)
      return 1;
  } while( true );
}


// Function: sub_11850 @ 0x11850
unsigned long sub_11850(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_11860 @ 0x11860
unsigned long sub_11860(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_11870 @ 0x11870
unsigned long sub_11870(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_11880 @ 0x11880
unsigned long sub_11880(struct_28 *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = a0->field_0x0;
  v4 = 0;
  do {
    if (a0->field_0x8 <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if (a0->field_0x8 <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}


// Function: sub_11950 @ 0x11950
void sub_11950(long a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_11870(a0);
  v2 = sub_11850();
  v3 = sub_11860();
  v4 = sub_11880();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_1d690) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_11a70 @ 0x11a70
long sub_11a70(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_11530(a0,a1);
  v3 = *v2;
  if (!v3)
    return 0;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}


// Function: sub_11b20 @ 0x11b20
long sub_11b20(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_11530(a0,a1);
  v3 = v2;
  do {
    v1 = *v3;
    v3 = (long *)v3[1];
    if (v1 == a1) {
      if (v3)
        return *v3;
      break;
    }
  } while (v3);
  do {
    v2 = &v2[2];
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}


// Function: sub_11be0 @ 0x11be0
long sub_11be0(struct_28 *a0,void *a1,unsigned long a2)
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


// Function: sub_11cd0 @ 0x11cd0
unsigned long * sub_11cd0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_11450;
  if (!a3)
    a3 = sub_11470;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x1d660;
  v1[5] = a1;
  if (sub_11480(v1)) {
    v2 = sub_117c0(*(float *)(a1 + 8),a0,(unsigned char)*(char *)(a1 + 0x10));
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


// Function: sub_11de0 @ 0x11de0
void sub_11de0(struct_30 *a0)
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
        if (a0->field_0x8 <= v5) goto label_11e7b;
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
label_11e7b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_11e90 @ 0x11e90
void sub_11e90(struct_7 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_11f3c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11eff;
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
label_11eff:
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
label_11f3c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_11f80 @ 0x11f80
unsigned int sub_11f80(struct_5 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_117c0(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_11690(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_11690(a0,&v2,1)) && (sub_11690(a0,&v2,0))) {
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


// Function: sub_12100 @ 0x12100
unsigned long sub_12100(long a0,long a1,long *a2) // ternary
{
  long v1;
  struct_4 *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_11560(a0,a1,&v3,0);
    if (v1) {
      if (a2) {
        *a2 = v1;
        return 0;
      }
      return 0;
    }
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_11480(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_1d684 <= v5)
          return 0xffffffff;
        v4 = (dat_1d688 <= v5) ? (long)(v5 - dat_1d688) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_11f80(a0,v4))
          return 0xffffffff;
        if (sub_11560(a0,a1,&v3,0)) {
          abort(); // no-return, return-dupe
        }
      }
    }
    if (!*v3) {
      *v3 = a1;
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (struct_4 *)sub_11660(a0);
    if (!v2)
      return 0xffffffff;
    v2->field_0x0 = a1;
    v2->field_0x8 = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}


// Function: sub_122f0 @ 0x122f0
unsigned long sub_122f0(long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_12100(a0,a1,&v3);
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


// Function: sub_12350 @ 0x12350
long sub_12350(long a0,long a1) // ternary x2
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
  
  v5 = sub_11560(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_11480(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1d688 <= v8) ? (long)(v8 - dat_1d688) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11f80(a0,v6)) {
          v4 = *(void **)(a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)(a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

