// Function: sub_4050 @ 0x4050
unsigned long sub_4050(int a0,char **a1)
{
  bool v1;
  unsigned int v10; // stack - 0x98
  long v11; // stack - 0xc8
  long v12; // stack - 0xc0
  long v13; // stack - 0xb8
  long v14; // stack - 0xb0
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
  char *v3; // rax
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
  unsigned long v4; // rax
  int v40; // stack - 0x54
  unsigned long v41; // stack - 0x50
  unsigned long v42; // stack - 0x48
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  long v9;
  
  sub_12680(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_19d70(sub_e2b0);
  dat_2315a = 0 < (int)is_selinux_enabled();
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
  v15 = 0;
  v41 = 0;
  v39 = getenv("POSIXLY_CORRECT") != NULL;
  v42 = 0;
  v14 = 0;
  v11 = 0;
  v16 = 0;
  v12 = 0;
  v13 = 0;
  v1 = 0;
label_41b8:
  v2 = getopt_long(a0,a1,"abdfHilLnprst:uvxPRS:TZ",(void *)0x224a0,NULL);
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
              v3 = dcgettext(NULL,"warning: ignoring --context",5);
              error(0,*__errno_location(),v3);
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
        v9 = optarg;
        if (!optarg)
          v9 = v12;
        v12 = v9;
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
          return v6;
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
        if (optarg) { // branch-flip
          v9 = sub_cfc0("--reflink",optarg,0x22860,0x1a460,4,dat_23028,1);
          v40 = *(int *)(v9 * 4 + 0x1a460);
        }
        else {
          v40 = 2;
        }
        break;
      case 0x86:
        v9 = sub_cfc0("--sparse",optarg,0x22880,0x1a470,4,dat_23028,1);
        v19 = *(int *)(v9 * 4 + 0x1a470);
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
    v3 = "cannot make both hard and symbolic links";
    goto label_4784;
  }
  if (v18 != 2) { // branch-flip
    if (!v1) goto label_46a9;
    if ((v40 == 2) && (v19 != 2)) goto label_4778;
    v3 = dcgettext(NULL,"backup type",5);
    v10 = sub_d8d0(v3,v12);
  }
  else {
    v36 = 0;
    if (v1) {
      v3 = "options --backup and --no-clobber are mutually exclusive";
      goto label_4784;
    }
label_46a9:
    v10 = 0;
    if ((v40 == 2) && (v10 = 0, v19 != 2)) {
label_4778:
      v3 = "--reflink can be used only with --sparse=auto";
label_4784:
      error(0,0,dcgettext(NULL,v3,5));
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
        return v5;
      }
      if (!dat_2315a) {
        error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5)); // return-dupe
        return v4;
      }
    }
    else {
      v28 = '\0';
    }
    if ((v14) && ((int)setfscreatecon(v14) < 0)) {
      v7 = sub_14b30(v14);
      v3 = dcgettext(NULL,"failed to set default file creation context to %s",5);
      error(1,*__errno_location(),v3,v7);
      return v8;
    }
  }
  else if ((v29) && (!dat_2315a)) {
    error(1,0,dcgettext(NULL,"cannot preserve security context without an SELinux-enabled kernel",5));
    return v4;
  }
  sub_bdb0();
  v9 = (long)optind;
  a0 -= optind;
  return (unsigned long)((sub_5a50(a0,&a1[v9],v11,v15,&v10) ^ 1) & 0xff);
label_4250:
  v20._0_6_ = CONCAT15(1,(undefined5)v20);
  goto label_41b8;
}


// Function: sub_49a0 @ 0x49a0
void sub_49a0(unsigned long a0,unsigned long a1,char a2)
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
  v4 = sub_cfc0(v2,v1,0x22460,0x1a440,4,dat_23028);
  if (*(unsigned int *)(v4 * 4 + 0x1a440) <= 6) {
    (*(void *)((long)*(int *)((unsigned long)*(unsigned int *)(v4 * 4 + 0x1a440) * 4 + 0x1a320) + 0x1a320))(); // jump-as-call
    return;
  }
  abort(); // no-return
}


// Function: sub_4b10 @ 0x4b10
unsigned long sub_4b10(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,long a6) // return-dupe x2
{
  unsigned int v1;
  int v10;
  unsigned int v11;
  unsigned long v12; // rax
  unsigned long v13; // rax
  void *v14;
  char *v15;
  unsigned long *v16;
  unsigned long v17; // rax
  unsigned int v18; // ecx
  unsigned long v19; // rdx
  long v2;
  unsigned long *v20; // stack - 0x1a8
  char v21 [24];
  unsigned long v22; // stack - 0xd8
  char *v23; // rsp
  unsigned long *v24; // rsp
  char *v25; // rsp
  char *v26;
  char *v27; // rsp
  char *v28;
  unsigned int v29; // r13d
  char v3;
  char *v30; // stack - 0x1a0
  unsigned long *v31; // stack - 0x198
  long *v32; // stack - 0x190
  char *v33; // stack - 0x188
  char *v34; // stack - 0x180
  char *v35; // stack - 0x178
  void *v36; // stack - 0x170
  unsigned int v37; // stack - 0x150
  unsigned long v38; // stack - 0xd0
  unsigned long v39; // stack - 0xc8
  long v4;
  unsigned int v40; // stack - 0xc0
  unsigned int v41; // stack - 0xbc
  unsigned long v42; // stack - 0xb8
  unsigned long v43; // stack - 0xb0
  unsigned long v44; // stack - 0xa8
  unsigned long v45; // stack - 0xa0
  unsigned long v46; // stack - 0x98
  unsigned long v47; // stack - 0x90
  unsigned long v48; // stack - 0x88
  unsigned long v49; // stack - 0x80
  unsigned long *v5;
  unsigned long v50; // stack - 0x78
  unsigned long v51; // stack - 0x70
  unsigned long v52; // stack - 0x68
  unsigned long v53; // stack - 0x60
  unsigned long v54; // stack - 0x58
  unsigned long v55; // stack - 0x50
  char *v6;
  long v7;
  char v8;
  int v9;
  
  v7 = a6;
  v24 = &v20;
  v16 = &v20;
  v30 = a3;
  v32 = a4;
  v34 = a5;
  v12 = sub_e4a0(a0);
  *v32 = 0;
  if (v12 <= a1)
    return 1;
  v13 = strlen(a0);
  v5 = &v20;
  while (v24 != (unsigned long *)((long)&v20 - (v13 + 0x18 & 0xfffffffffffff000))) {
    v16 = (char *)((long)v5 + -0x1000);
    v23 = (char *)((long)v5 + -0x1000);
    *(unsigned long *)((long)v5 + -8) = *(unsigned long *)((long)v5 + -8);
    v24 = (char *)((long)v5 + -0x1000);
    v5 = (unsigned long *)v23;
  }
  v19 = (unsigned long)((unsigned int)(v13 + 0x18) & 0xff0);
  v4 = -v19;
  v25 = (char *)((long)v16 + v4);
  v26 = (char *)((long)v16 + v4);
  if (v19)
    *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
  *(unsigned long *)((long)v16 + v4 + -8) = 0x4c05;
  v36 = memcpy((void *)((unsigned long)((long)v16 + v4 + 0xf) & 0xfffffffffffffff0),a0,v13 + 1,*(char *)((long)v16 + v4 + -8));
  v6 = (char *)((long)v16 + v4);
  while (v25 != (char *)((long)v16 + (v4 - (v12 + 0x18 & 0xfffffffffffff000)))) {
    v26 = &v6[-0x1000];
    v27 = &v6[-0x1000];
    *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
    v25 = &v6[-0x1000];
    v6 = v27;
  }
  v13 = (unsigned long)((unsigned int)(v12 + 0x18) & 0xff0);
  v4 = -v13;
  if (v13)
    *(unsigned long *)&v26[-8] = *(unsigned long *)&v26[-8];
  v14 = (void *)((unsigned long)&v26[v4 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v26[v4 + -8] = 0x4c6f;
  memcpy(v14,v36,v12,v26[v4 + -8]);
  v28 = (char *)((long)v14 + a1);
  *(char *)((long)v14 + v12) = 0;
  v8 = *v28;
  while (v8 == '/') {
    v28 = &v28[1];
    v8 = *v28;
  }
  v33 = v21;
  *(unsigned long *)&v26[v4 + -8] = 0x4ca4;
  v3 = v26[v4 + -8];
  if (!fstatat(a2,v28,v21,0,v3)) {
    if ((v37 & 0xf000) == 0x4000) {
      *v34 = '\0';
      return 1;
    }
label_4b75:
    *(unsigned long *)&v26[v4 + -8] = 0x5136;
    v17 = sub_14440(4,v14);
    *(unsigned long *)&v26[v4 + -8] = 0x514c;
    v28 = dcgettext(NULL,"%s exists but is not a directory",5,v26[v4 + -8]);
    *(unsigned long *)&v26[v4 + -8] = 0x515d;
    error(0,0,v28,v17);
    return 0;
  }
  v28 = (char *)(a1 + (long)v36);
  v35 = v28;
  if (*v28 != '/') // branch-flip
    v28 = v35;
  else {
    do {
      v28 = &v28[1];
    } while (*v28 == '/');
  }
  v20 = &v22;
  v15 = v28;
  do {
    *(unsigned long *)&v26[v4 + -8] = 0x4cfa;
    v15 = strchr(v15,0x2f,v26[v4 + -8]);
    if (!v15)
      return 1;
    *v15 = '\0';
    *(unsigned long *)&v26[v4 + -8] = 0x4d1d;
    v9 = fstatat(a2,v28,v33,0,v26[v4 + -8]);
    if ((v9) || (*(unsigned int *)(v7 + 0x1c) & 0xffffff00)) {
      *(unsigned long *)&v26[v4 + -8] = 0x4df3;
      v10 = stat(v35,v20,v26[v4 + -8]);
      if (v10) { // branch-flip
        *(unsigned long *)&v26[v4 + -8] = 0x4e00;
        v3 = v26[v4 + -8];
        v10 = *__errno_location(v3);
        if (v10) {
label_4b75:
          v36 = (void *)CONCAT44(v36._4_4_,v10);
          *(unsigned long *)&v26[v4 + -8] = 0x5054;
          v17 = sub_14440(4,v35);
          *(unsigned long *)&v26[v4 + -8] = 0x506a;
          v28 = dcgettext(NULL,"failed to get attributes of %s",5,v26[v4 + -8]);
          *(unsigned long *)&v26[v4 + -8] = 0x507f;
          error(0,(int)v36,v28,v17);
          return 0;
        }
      }
      else if ((v40 & 0xf000) != 0x4000) {
        v10 = 0x14;
        goto label_4b75;
      }
      *(unsigned long *)&v26[v4 + -8] = 0x4e16;
      v16 = (unsigned long *)sub_16c50(0xa8);
      *v16 = v22;
      v16[1] = v38;
      v16[2] = v39;
      v16[3] = CONCAT44(v41,v40);
      v16[4] = v42;
      v16[5] = v43;
      v16[6] = v44;
      v16[7] = v45;
      v16[8] = v46;
      v16[9] = v47;
      v16[10] = v48;
      v16[0xb] = v49;
      v16[0xc] = v50;
      v16[0xd] = v51;
      v16[0xe] = v52;
      v16[0xf] = v53;
      v16[0x10] = v54;
      v16[0x11] = v55;
      v16[0x13] = (long)v15 - (long)v36;
      v2 = *v32;
      *(char *)&v16[0x12] = 0;
      v16[0x14] = v2;
      *v32 = (long)v16;
      if (!v9) goto label_4d37;
      v1 = *(unsigned int *)&v16[3];
      v31 = v16;
      *(unsigned long *)&v26[v4 + -8] = 0x4ed7;
      v8 = sub_72a0(v35,v36,v1,1,v7);
      if (!v8)
        return 0;
      *v34 = '\x01';
      v11 = *(unsigned int *)&v31[3];
      if (*(char *)(v7 + 0x1d)) { // branch-flip
        v29 = v11 & 0x3f;
        v18 = ~v29;
      }
      else if (*(char *)(v7 + 0x1e)) { // branch-flip
        v29 = v11 & 0x12;
        v18 = ~v29;
      }
      else {
        v18 = 0xffffffff;
        v29 = 0;
      }
      if (*(char *)(v7 + 0x20))
        v11 = 0x1ff;
      *(unsigned long *)&v26[v4 + -8] = 0x4f34;
      if (mkdirat(a2,v28,v11 & v18 & 0xfff)) {
        *(unsigned long *)&v26[v4 + -8] = 0x51d4;
        v17 = sub_14440(4,v36);
        v28 = "cannot make directory %s";
        goto label_4b75;
      }
      if (v30) {
        *(unsigned long *)&v26[v4 + -8] = 0x4f70;
        __printf_chk(1,v30,v35,v36);
      }
      *(unsigned long *)&v26[v4 + -8] = 0x4f95;
      v9 = fstatat(a2,v28,v33,0x100,v26[v4 + -8]);
      if (v9) {
        *(unsigned long *)&v26[v4 + -8] = 0x5181;
        v17 = sub_14440(4,v36);
        v28 = "failed to get attributes of %s";
        goto label_4b75;
      }
      v11 = v37;
      if (!*(char *)(v7 + 0x1e)) {
        if (~v37 & v29) { // branch-flip
          *(unsigned long *)&v26[v4 + -8] = 0x50e1;
          v29 &= ~sub_bbd0();
          v16 = v31;
          v11 = v37;
          if (!(~v37 & v29)) goto label_4fbf;
        }
        else {
label_4fbf:
          v16 = v31;
          v11 = v37;
          if ((v37 & 0x1c0) == 0x1c0) goto label_4fde;
        }
        *(char *)&v16[0x12] = 1;
        *(unsigned int *)&v16[3] = v29 | v11;
      }
label_4fde:
      if ((v11 | 0x1c0) != v11) {
        *(unsigned long *)&v26[v4 + -8] = 0x4ffa;
        if (fchmodat(a2,v28,v11 | 0x1c0,0x100)) {
          *(unsigned long *)&v26[v4 + -8] = 0x51f6;
          v17 = sub_14440(4,v36);
          v28 = "setting permissions for %s";
label_4b75:
          *(unsigned long *)&v26[v4 + -8] = 0x5197;
          v28 = dcgettext(NULL,v28,5,v26[v4 + -8]);
          *(unsigned long *)&v26[v4 + -8] = 0x519f;
          v3 = v26[v4 + -8];
          v9 = *__errno_location(v3);
          *(unsigned long *)&v26[v4 + -8] = 0x51b0;
          error(0,v9,v28,v17);
          return 0;
        }
      }
      if (!*v34) goto label_4d79;
    }
    else {
label_4d37:
      *(unsigned long *)&v26[v4 + -8] = 0x4d51;
      v8 = sub_72a0(v35,v36,0,0,v7);
      if (!v8)
        return 0;
      if ((v37 & 0xf000) != 0x4000) {
        v14 = v36;
        goto label_4b75;
      }
      *v34 = '\0';
label_4d79:
      if ((*(long *)(v7 + 0x28)) || (*(char *)(v7 + 0x33))) {
        *(unsigned long *)&v26[v4 + -8] = 0x4d96;
        v8 = sub_7510(v36,0,v7);
        if ((!v8) && (*(char *)(v7 + 0x34)))
          return 0;
      }
    }
    v8 = v15[1];
    *v15 = '/';
    v15 = &v15[1];
    while (v8 == '/') {
      v15 = &v15[1];
      v8 = *v15;
    }
  } while( true );
}


// Function: sub_5210 @ 0x5210
unsigned long sub_5210(char *a0,unsigned int a1,unsigned long a2,long a3,long a4) // return-dupe
{
  long v1;
  void *v10; // rax
  unsigned long v11; // rax
  char *v12; // rax
  unsigned long v13; // rdx
  unsigned long *v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long *v16;
  char *v17; // rsp
  unsigned long *v18; // rsp
  unsigned long v19; // stack - 0x70
  char v2;
  unsigned long v20; // stack - 0x60
  unsigned long v21; // stack - 0x58
  unsigned long v22; // stack - 0x50
  unsigned int v3;
  unsigned int v4;
  char v5;
  long v6;
  unsigned long *v7;
  int v8;
  unsigned long v9; // rax
  
  v16 = &v14;
  v19 = a2;
  v9 = strlen(a0);
  v18 = &v14;
  v7 = &v14;
  while (v18 != (unsigned long *)((long)&v14 - (v9 + 0x18 & 0xfffffffffffff000))) {
    v16 = (char *)((long)v7 + -0x1000);
    v17 = (char *)((long)v7 + -0x1000);
    *(unsigned long *)((long)v7 + -8) = *(unsigned long *)((long)v7 + -8);
    v18 = (char *)((long)v7 + -0x1000);
    v7 = (unsigned long *)v17;
  }
  v13 = (unsigned long)((unsigned int)(v9 + 0x18) & 0xff0);
  v6 = -v13;
  if (v13)
    *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
  *(unsigned long *)((long)v16 + v6 + -8) = 0x52a0;
  v10 = memcpy((void *)((unsigned long)((long)v16 + v6 + 0xf) & 0xfffffffffffffff0),a0,v9 + 1,*(char *)((long)v16 + v6 + -8));
  v1 = (long)v10 + (v19 - (long)a0);
  if (!a3)
    return 1;
  v14 = &v15;
  do {
    v2 = *(char *)(a4 + 0x1f);
    *(char *)((long)v10 + *(long *)(a3 + 0x98)) = 0;
    if (v2) {
      v15 = *(unsigned long *)(a3 + 0x48);
      v20 = *(unsigned long *)(a3 + 0x50);
      v21 = *(unsigned long *)(a3 + 0x58);
      v22 = *(unsigned long *)(a3 + 0x60);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x534e;
      v8 = utimensat(a1,v1,v14,0);
      if (v8) {
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5363;
        v11 = sub_14440(4,v10);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5379;
        v12 = dcgettext(NULL,"failed to preserve times for %s",5,*(char *)((long)v16 + v6 + -8));
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5381;
        v5 = *(char *)((long)v16 + v6 + -8);
        v8 = *__errno_location(v5);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5392;
        error(0,v8,v12,v11);
        return 0;
      }
    }
    if (*(char *)(a4 + 0x1d)) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x20);
      v3 = *(unsigned int *)(a3 + 0x1c);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53b7;
      if (!fchownat(a1,v1,v3,v4,0x100)) goto label_52d3;
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53c7;
      if (!sub_76c0(a4)) {
        v19 &= 0xffffffffffffff00;
        *(unsigned long *)((long)v16 + v6 + -8) = 0x54da;
        v11 = sub_14440(4,v10);
        v12 = "failed to preserve ownership for %s";
        goto label_546b;
      }
      v4 = *(unsigned int *)(a3 + 0x20);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x53e8;
      fchownat(a1,v1,0xffffffff,v4,0x100);
      v2 = *(char *)(a4 + 0x1e);
    }
    else {
label_52d3:
      v2 = *(char *)(a4 + 0x1e);
    }
    if (v2) { // branch-flip
      v4 = *(unsigned int *)(a3 + 0x18);
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5419;
      if (sub_c830(v1,0xffffffff,v10,0xffffffff,v4))
        return 0;
    }
    else if (*(char *)(a3 + 0x90)) {
      v4 = *(unsigned int *)(a3 + 0x18);
      v19 &= 0xffffffffffffff00;
      *(unsigned long *)((long)v16 + v6 + -8) = 0x5447;
      if (fchmodat(a1,v1,v4,0x100)) {
        *(unsigned long *)((long)v16 + v6 + -8) = 0x545c;
        v11 = sub_14440(4,v10);
        v12 = "failed to preserve permissions for %s";
label_546b:
        *(unsigned long *)((long)v16 + v6 + -8) = 0x5472;
        v12 = dcgettext(NULL,v12,5,*(char *)((long)v16 + v6 + -8));
        *(unsigned long *)((long)v16 + v6 + -8) = 0x547a;
        v5 = *(char *)((long)v16 + v6 + -8);
        v8 = *__errno_location(v5);
        *(unsigned long *)((long)v16 + v6 + -8) = 0x548b;
        error(0,v8,v12,v11);
        return v19 & 0xff;
      }
    }
    *(char *)((long)v10 + *(long *)(a3 + 0x98)) = 0x2f;
    a3 = *(long *)(a3 + 0xa0);
    if (!a3)
      return 1;
  } while( true );
}


// Function: sub_54f0 @ 0x54f0
void sub_54f0(int a0)
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
  
  v2 = dat_235a0;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",5),v2,v2,v2);
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
      if (!strcmp("cp",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "cp";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "cp";
    if (!strcmp("cp","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "cp")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}


// Function: sub_5a50 @ 0x5a50
unsigned long sub_5a50(int a0,unsigned long *a1,long a2,char a3,int *a4)
{
  int *v1;
  char v10; // al
  char v11;
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
  char v25 [24];
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
  char v4;
  long v40; // stack - 0x110
  int *v41; // stack - 0x108
  int v42; // stack - 0x100
  int v43; // stack - 0xfc
  unsigned int v44; // stack - 0xc0
  long v5;
  char *v6;
  void *v7;
  int v8;
  unsigned char v9;
  
  v32 = &v24;
  v28 = 0;
  v40 = a2; // branch-flip
  v41 = a4;
  v43 = a0;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (a0 == 1) {
      v18 = sub_14440(4,*a1);
      v19 = "missing destination file operand after %s";
label_6026:
      error(0,0,dcgettext(NULL,v19,5),v18);
      sub_54f0(1); // return-dupe, no-return
    }
    v19 = "missing file operand";
  }
  else {
    v44 = 0;
    if (a3) { // branch-flip
      if (a2) {
        error(1,0,dcgettext(NULL,"cannot combine --target-directory (-t) and --no-target-directory (-T)",5));
        return v17;
      }
      if (2 < a0) {
        v18 = sub_14440(4,a1[2]);
        v19 = "extra operand %s";
        goto label_6026;
      }
    }
    else {
      if (a2) {
        v42 = sub_156a0(a2,v25);
        if (v42 == -1) {
          v18 = sub_14440(4,v40);
          v19 = dcgettext(NULL,"target directory %s",5);
          error(1,*__errno_location(),v19,v18);
          return v20;
        }
        if (v43 != 1) {
label_5eef:
          v34 = v41;
          sub_75f0(v41);
          sub_7630(v34);
        }
label_5aeb:
        v23 = 0;
        v37 = 1;
        v38 = &v26;
        v39 = &v27;
        v24 = &v28;
        do {
          v19 = (char *)a1[v23];
          v26 = NULL;
          if (dat_23158) { // branch-flip
            *(unsigned long *)((long)v32 + -8) = 0x5d70;
            sub_e5a0(v19);
            if (!dat_23159) goto label_5b4d;
label_5d80:
            *(unsigned long *)((long)v32 + -8) = 0x5d85;
            v14 = strlen(v19,*(char *)((long)v32 + -8));
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v6 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v6[-0x1000];
              v33 = &v6[-0x1000];
              *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
              v32 = (char *)&v6[-0x1000];
              v6 = v33;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v5 = -v22;
            v32 = (char *)&v30[v5];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v5 + -8] = 0x5de2;
            v15 = memcpy((void *)((unsigned long)&v30[v5 + 0xf] & 0xfffffffffffffff0),v19,v14 + 1,v30[v5 + -8]);
            *(unsigned long *)&v30[v5 + -8] = 0x5ded;
            sub_e5a0(v15);
            *(unsigned long *)&v30[v5 + -8] = 0x5e03;
            v36 = v40;
            v15 = (void *)sub_ea10(v40,v15,v38);
            v13 = NULL;
            v8 = v41[0xf];
            *(long *)&v30[v5 + -8] = v36;
            if ((char)v8)
              v13 = "%s -> %s\n";
            *(int **)&v30[v5 + -0x10] = v41;
            *(unsigned long *)&v30[v5 + -0x18] = 0x5e46;
            v10 = sub_4b10(v15,(long)v26 - (long)v15,v42,v13,&v29,v24);
            v11 = *v26;
            v13 = v26;
            while (v11 == '/') {
              v13 = &v13[1];
              v26 = v13;
              v11 = *v13;
            }
            if (v10) goto label_5bf4;
            v37 = 0;
            v32 = (unsigned char **)&v30[v5];
            v11 = dat_23159;
label_5e83:
            v7 = v29;
            if (v11) {
              while (v7) {
                v3 = *(void **)((long)v7 + 0xa0);
                v29 = v3;
                *(unsigned long *)((long)v32 + -8) = 0x5eb6;
                free(v7,*(char *)((long)v32 + -8));
                v7 = v3;
              }
            }
          }
          else {
            if (dat_23159) goto label_5d80;
label_5b4d:
            *(unsigned long *)((long)v32 + -8) = 0x5b52;
            v13 = (char *)sub_d970(v19);
            *(unsigned long *)((long)v32 + -8) = 0x5b5d;
            v14 = strlen(v13,*(char *)((long)v32 + -8));
            v21 = (char *)((long)v32 + -(v14 + 0x18 & 0xfffffffffffff000));
            v30 = (char *)v32;
            v6 = (char *)v32;
            while (v32 != (unsigned char **)v21) {
              v30 = &v6[-0x1000];
              v31 = &v6[-0x1000];
              *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
              v32 = (char *)&v6[-0x1000];
              v6 = v31;
            }
            v22 = (unsigned long)((unsigned int)(v14 + 0x18) & 0xff0);
            v5 = -v22;
            v32 = (char *)&v30[v5];
            if (v22)
              *(unsigned long *)&v30[-8] = *(unsigned long *)&v30[-8];
            *(unsigned long *)&v30[v5 + -8] = 0x5bba;
            v13 = memcpy((void *)((unsigned long)&v30[v5 + 0xf] & 0xfffffffffffffff0),v13,v14 + 1,v30[v5 + -8]);
            *(unsigned long *)&v30[v5 + -8] = 0x5bc5;
            sub_e5a0(v13);
            *(unsigned long *)&v30[v5 + -8] = 0x5bd4;
            v4 = v30[v5 + -8];
            *(unsigned long *)&v30[v5 + -8] = 0x5bf1;
            v15 = (void *)sub_ea10(v40,&v13[strcmp(v13,"..",v4) == 0],v38);
label_5bf4:
            *(unsigned long *)((long)v32 + -8) = 0;
            *(char **)((long)v32 + -0x10) = v39;
            *(unsigned long *)((long)v32 + -0x18) = 0x5c23;
            v12 = sub_ba80(v19,v15,v42,v26,v28,v41);
            v37 &= v12;
            if (dat_23159) {
              *(unsigned long *)((long)v32 + -8) = 0x5ee3;
              v12 = sub_5210(v15,v42,v26,v29,v41);
              v37 &= v12;
              v11 = dat_23159;
              goto label_5e83;
            }
          }
          v23 += 1;
          *(unsigned long *)((long)v32 + -8) = 0x5c41;
          free(v15,*(char *)((long)v32 + -8));
        } while ((int)v23 < v43);
        return (unsigned long)v37; // return-dupe
      }
      v23 = a1[(long)a0 + -1];
      v40 = v23;
      v42 = sub_156a0(v23,v25);
      if (v42 != -1) { // branch-flip
        v43 = a0 + -1;
        if (v23) {
          if (1 < v43) goto label_5eef;
          v37 = 1;
          if (v43 != 1) {
            return (unsigned long)v37;
          }
          goto label_5aeb;
        }
      }
      else {
        v8 = *__errno_location();
        if (v8 == 2)
          v28 = 1;
        if (3 <= v43) {
          v18 = sub_14440(4,v40);
          error(1,v8,dcgettext(NULL,"target %s",5),v18);
          return v16;
        }
      }
    }
    v9 = v28;
    v19 = (char *)*a1;
    v13 = (char *)a1[1];
    if (!dat_23159) {
      if (((((*(char *)((long)v41 + 0x16)) && (*v41)) && (!strcmp(v19,v13))) && ((!v9 && ((v44 || (!stat(v13,v25))))))) && (v34 = v41, (v44 & 0xf000) == 0x8000)) {
        v13 = (char *)sub_d850(0xffffff9c,v13,*v41);
        v23 = 0x16;
        v35 = (int *)0x23100;
        while (v23) {
          v2 = &v35[1];
          v1 = &v34[1];
          *v35 = *v34;
          v23 -= 1;
          v34 = v1;
          v35 = v2;
        }
        dat_23100 = 0;
        v41 = (int *)0x23100;
      }
      v37 = sub_ba80(v19,v13,0xffffff9c,v13,-(unsigned int)v9,v41,&v26,0);
      return (unsigned long)v37;
    }
    v19 = "with --parents, the destination must be a directory";
  }
  error(0,0,dcgettext(NULL,v19,5));
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
unsigned long sub_6220(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,long a4)
{
  char v1;
  void *v10; // stack - 0x18
  unsigned int v2; // eax
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
      v6 = sub_6720;
      v7 = NULL;
      if (!v1) {
        v6 = NULL;
        v7 = NULL;
        if (*(long *)(a4 + 0x28)) {
          v6 = sub_6720;
          v7 = NULL;
        }
      }
      goto label_6282;
    }
    v6 = sub_6720;
    v5 = sub_6360;
    if ((!v1) && (!*(long *)(a4 + 0x28)))
      v6 = NULL;
  }
  else {
    v6 = sub_6720;
    v5 = sub_6440;
    if (!v1) {
      v6 = sub_6720;
      if (!*(long *)(a4 + 0x28))
        v6 = NULL;
      v5 = sub_6440;
    }
  }
  v9 = sub_6100;
  v10 = sub_60f0;
label_6282:
  v2 = (unsigned int)a1;
  v2 |= a3;
  if (0 <= (int)v2) { // branch-flip
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
void sub_6360(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
void sub_6440(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x90
  unsigned long v11; // stack - 0x88
  unsigned long v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x38
  unsigned long v17; // stack - 0x28
  unsigned long v18; // stack - 0x18
  unsigned int v2; // stack - 0xd8
  char v3 [16];
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xa8
  unsigned long v8; // stack - 0xa0
  unsigned long v9; // stack - 0x98
  
  if (v1) {
    v11 = a0;
    v12 = a1;
    v13 = a2;
    v14 = a3;
    v15 = a4;
    v16 = a5;
    v17 = a6;
    v18 = a7;
  }
  v7 = a10;
  v8 = a11;
  v9 = a12;
  v10 = a13;
  v5 = &Stack0000000000000008;
  v2 = 0x10;
  v6 = v3;
  v4 = 0x30;
  sub_163b0(0,*__errno_location(),a9,&v2);
}


// Function: sub_6510 @ 0x6510
void sub_6510(unsigned long a0,unsigned long a1,long a2)
{
  char *v1;
  unsigned long v2; // rax
  
  v2 = sub_14310(1,4,a1);
  __printf_chk(1,"%s -> %s",sub_14310(0,4,a0),v2);
  if (a2) {
    v2 = sub_14440(4,a2);
    __printf_chk(1,dcgettext(NULL," (backup: %s)",5),v2);
  }
  v1 = *(char **)((long)stdout + 0x28);
  if (v1 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}


// Function: sub_65d0 @ 0x65d0
unsigned int sub_65d0(void *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned int a4,unsigned long a5,char a6,unsigned int a7,unsigned char a8)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned char v5; // r12b
  unsigned int v6;
  unsigned long v7; // stack - 0x50
  
  v6 = a7;
  v1 = sub_be90(a1,a2,a4,a5,(unsigned long)a8 << 10,a6);
  if (1 <= v1) { // branch-flip
    v4 = NULL;
    if (!a0) {
      v4 = (void *)sub_6110(a3,a5,a2,v7);
      a0 = v4;
    }
    v3 = sub_14310(1,4,a0);
    v2 = sub_14310(0,4,a3);
    v6 = 0;
    error(0,v1,dcgettext(NULL,"cannot create hard link %s to %s",5),v2,v3);
    free(v4);
  }
  else {
    v5 = (unsigned char)v6 & (unsigned char)((unsigned int)v1 >> 0x1f);
    v6 = CONCAT31((undefined3)((unsigned int)v6 >> 8),v5);
    if (v5) { // branch-flip
      v3 = sub_14440(4,a3,0xffffffffffffffff);
      __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v3);
    }
    else {
      v6 = 1;
    }
  }
  return v6;
}


// Function: sub_67a0 @ 0x67a0
void sub_67a0(long a0,unsigned long a1,unsigned int a2,unsigned long a3,long a4)
{
  unsigned int v1;
  unsigned long v2;
  unsigned long v3; // rax
  char *v4; // rax
  long v5;
  char v6 [9];
  char v7; // stack - 0x32
  
  if ((((*(unsigned int *)(a4 + 0x18) & 0xf000) != 0xa000) && (!sub_16bb0())) && (faccessat(a2,a3,2,0x200))) {
    sub_e880(*(unsigned int *)(a4 + 0x18),&v5);
    v1 = *(unsigned int *)(a4 + 0x18);
    v7 = 0;
    v3 = sub_14440(4,a1);
    v2 = dat_235a0;
    if ((*(char *)(a0 + 0x18)) || (*(unsigned int *)(a0 + 0x14) & 0xffff00))
      v4 = dcgettext(NULL,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    else {
      v4 = dcgettext(NULL,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    __fprintf_chk(stderr,1,v4,v2,v3,(unsigned long)(v1 & 0xfff),v6);
  }
  else {
    v3 = sub_14440(4,a1);
    v2 = dat_235a0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: overwrite %s? ",5),v2,v3);
  }
  sub_17440(); // tail-call
}


// Function: sub_6920 @ 0x6920
unsigned long sub_6920(unsigned long a0,unsigned long a1,char a2,long a3)
{
  int v1;
  long v2; // rax
  int *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6; // r12
  
  v2 = lseek(a0,a3,1);
  if (v2 < 0) {
    v4 = sub_14440(4,a1);
    v5 = dcgettext(NULL,"cannot lseek %s",5);
    error(0,*__errno_location(),v5,v4);
    return 0;
  }
  if (a2) {
    if ((int)fallocate(a0 & 0xffffffff,3,v2 - a3,a3) <= -1) {
      v3 = __errno_location();
      v1 = *v3;
      v6 = CONCAT71((undefined7)((unsigned long)a3 >> 8),v1 == 0x5f || v1 == 0x26);
      if (v1 != 0x5f && v1 != 0x26) {
        v4 = sub_14440(4,a1);
        v5 = dcgettext(NULL,"error deallocating %s",5);
        error(0,*v3,v5,v4);
        return v6 & 0xffffffff; // return-dupe
      }
    }
  }
  v6 = 1;
  return v6 & 0xffffffff;
}


// Function: sub_6a30 @ 0x6a30
unsigned long sub_6a30(int a0,unsigned int a1,unsigned long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,unsigned long a7,unsigned long a8,unsigned long a9,long *a10,unsigned char *a11) // return-dupe
{
  bool v1;
  int v10;
  unsigned int v11; // eax
  long v12; // rax
  unsigned long v13;
  int *v14; // rax
  unsigned long v15;
  unsigned long v16;
  char *v17;
  bool v18;
  bool v19;
  unsigned int v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22;
  char *v23;
  unsigned long v24;
  unsigned char v25;
  char *v26;
  bool v27;
  unsigned long v28;
  unsigned long v29; // stack - 0x98
  unsigned long v3;
  unsigned long v4;
  long *v5;
  unsigned char *v6;
  long *v7;
  unsigned char *v8;
  bool v9;
  
  v8 = a11;
  v7 = a10;
  v15 = a8;
  v16 = a7;
  *a11 = 0;
  v29 = a9;
  *a10 = 0;
  if ((a4) || (!(char)a6)) {
    if (!a9)
      return 1;
label_6b28:
    v24 = a3;
    if (a4)
      v24 = a4;
    v28 = 0;
    v27 = 0;
    do {
      v17 = (char *)*a2;
      v2 = a6;
      v3 = a7;
      v4 = a8;
      v13 = a9;
      v5 = a10;
      v6 = a11;
      while( true ) {
        a6 = v2;
        a7 = v3;
        a8 = v4;
        a9 = v13;
        a10 = v5;
        a11 = v6;
        if (!v17) {
          v17 = (char *)sub_16bf0((long)getpagesize(),a3);
          *a2 = v17;
        }
        v13 = a3;
        if (v29 <= a3)
          v13 = v29;
        v13 = read(a0,v17,v13);
        if ((long)v13 <= -1) break;
        if (!v13) {
label_7023:
          if (v27)
            return sub_6920(a1,v15,a5,v28); // tail-call
          return 1;
        }
        *v7 = *v7 + v13;
        v20 = v24;
        v22 = v13;
        v26 = v17;
        v18 = v27;
label_6c20:
        do {
          v21 = v20;
          if (v22 < v20)
            v21 = v22;
          v27 = v21 && a4;
          if (v21 && a4) {
            v20 = v21;
            v23 = v17;
            do {
              if (*v23) {
                v19 = v18;
                v9 = 0;
                v1 = v27;
                goto label_6c89;
              }
              v23 = &v23[1];
              v20 -= 1;
              if (!v20) {
                v25 = (v18 ^ 1U) & v28 != 0;
                goto label_6d90;
              }
            } while (v20 & 0xf);
            v10 = memcmp(v17,v23,v20);
            v19 = (bool)(v10 == 0 ^ v18);
            v9 = v10 == 0;
            v1 = v10 != 0;
label_6c89:
            v27 = v9;
            v25 = v28 != 0 & v19;
            if ((v21 != v22) || (!v1)) {
label_6d90:
              if (!v25) goto label_6d99;
              v1 = 0;
            }
            else {
              if (!v25) {
                v27 = 0;
                goto label_6d32;
              }
              v1 = 1;
              v27 = 0;
            }
label_6d46:
            if (v18) { // branch-flip
              v11 = sub_6920(a1,v15,a5,v28);
              if (!(char)v11)
                return (unsigned long)v11;
            }
            else if (v28 != sub_111c0(a1,v26,v28)) {
              v16 = sub_14440(4,v15);
              v17 = dcgettext(NULL,"error writing %s",5);
              error(0,*__errno_location(),v17,v16);
              return 0;
            }
            v26 = v17;
            v28 = v21;
            v18 = v27;
            if (v1) {
              if (v21) { // branch-flip
                if (v25) {
                  v20 = 0;
                  goto label_6c20;
                }
              }
              else {
                if (v25) {
                  v28 = 0;
                  break;
                }
                v22 = 0;
              }
              v22 -= v21;
              v28 = 0;
            }
            else {
              v22 -= v21;
            }
          }
          else {
            if (((v21 == v22) && (v18 != 1)) || (v27 = v18, !v21)) {
label_6d32:
              v28 += v21;
              v25 = 0;
              v1 = 1;
              goto label_6d46;
            }
label_6d99:
            v28 += v21;
            if (v28 + 0x8000000000000000 < v21) {
              v16 = sub_14440(4,v16);
              error(0,0,dcgettext(NULL,"overflow reading %s",5),v16);
              return 0;
            }
            v22 -= v21;
            v18 = v27;
          }
          v17 = &v17[v21];
          v20 = v21;
        } while (v22);
        v29 -= v13;
        *v8 = v27;
        if (!v29) goto label_7023;
        v17 = (char *)*a2;
        v2 = a6;
        v3 = a7;
        v4 = a8;
        v13 = a9;
        v5 = a10;
        v6 = a11;
      }
      v14 = __errno_location();
      if (*v14 != 4) {
        v16 = sub_14440(4,v16);
        v17 = dcgettext(NULL,"error reading %s",5);
        error(0,*v14,v17,v16);
        return 0;
      }
    } while( true );
  }
  if (!a9)
    return 1;
  do {
    while( true ) {
      v24 = 0x7fffffffc0000000;
      if (v29 <= 0x7fffffffc0000000)
        v24 = v29;
      v12 = copy_file_range(a0,0,a1,0,v24,0);
      if (!v12) {
        if (*v7)
          return 1;
        goto label_6b28;
      }
      if (v12 < 0) break;
      *v7 = *v7 + v12;
      v29 -= v12;
      if (!v29)
        return 1;
    }
    v14 = __errno_location();
    v10 = *v14;
    if (v10 == 0x26) goto label_6b28;
    if (0x1a < v10) {
      if (v10 == 0x5f) goto label_6b28;
      break;
    }
    if (1 <= v10) {
      if (1L << ((unsigned char)v10 & 0x3f) & 0x4440200U) goto label_6b28;
      if (v10 == 1) {
        if (!*v7) goto label_6b28;
        break;
      }
    }
  } while (v10 == 4);
  v15 = sub_14310(1,4,v15);
  v16 = sub_14310(0,4,v16);
  v17 = dcgettext(NULL,"error copying %s to %s",5);
  error(0,*v14,v17,v16,v15);
  return 0;
}


// Function: sub_72a0 @ 0x72a0
unsigned char sub_72a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned char a3,long a4)
{
  unsigned char v1;
  long v2;
  int v3; // eax
  int *v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7; // stack - 0x38
  
  v1 = *(unsigned char *)(a4 + 0x33);
  if (!v1) {
    v2 = *(long *)(a4 + 0x28);
    a3 = v2 != 0 & a3;
    if ((a3) && ((int)sub_c450(v2,a1,a2) <= -1)) {
      v4 = __errno_location();
      if ((*v4 != 0x5f) && (*v4 != 0x3d)) {
        v5 = sub_14440(4,a1);
        v6 = dcgettext(NULL,"failed to set default file creation context for %s",5);
        error(0,*v4,v6,v5);
        return a3;
      }
    }
    return 1;
  }
  if ((*(char *)(a4 + 0x31)) && (!*(char *)(a4 + 0x34))) {
    if (*(char *)(a4 + 0x37)) { // branch-flip
      if ((int)sub_15590(a0,&v7) <= -1) goto label_732e;
      v3 = setfscreatecon(v7);
      if (0 <= v3) {
        freecon(v7); // return-dupe
        return v1;
      }
    }
    else {
      if ((int)sub_15590(a0,&v7) <= -1) {
        v4 = __errno_location();
        if ((*v4 == 0x5f) || (*v4 == 0x3d)) goto label_732e;
        goto label_72f8;
      }
      v3 = setfscreatecon(v7);
      if (0 <= v3) {
        freecon(v7);
        return v1;
      }
      v4 = __errno_location();
      if ((*v4 != 0x5f) && (*v4 != 0x3d)) goto label_74ba;
    }
  }
  else {
    if ((int)sub_15590(a0,&v7) <= -1) {
      v4 = __errno_location();
label_72f8:
      v5 = sub_14440(4,a0);
      v6 = dcgettext(NULL,"failed to get security context of %s",5);
      error(0,*v4,v6,v5);
label_732e:
      if (!*(char *)(a4 + 0x34))
        return v1;
      return 0;
    }
    v3 = setfscreatecon(v7);
    if (0 <= v3) {
      freecon(v7);
      return v1;
    }
    v4 = __errno_location();
label_74ba:
    v5 = sub_14b30(v7);
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


// Function: sub_7510 @ 0x7510
unsigned long sub_7510(unsigned long a0,char a1,long a2) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v2;
  char *v3; // rax
  int *v4; // rax
  
  v2 = *(unsigned long *)(a2 + 0x28);
  if ((*(char *)(a2 + 0x31)) && (!*(char *)(a2 + 0x34))) {
    if (*(char *)(a2 + 0x37))
      return sub_c6f0(v2,a0,a1); // tail-call
    v1 = sub_c6f0(v2,a0,a1);
    if ((char)v1)
      return 1;
    v4 = __errno_location();
    if (*v4 == 0x5f)
      return (unsigned long)v1;
    if (*v4 == 0x3d)
      return (unsigned long)v1;
  }
  else {
    if (sub_c6f0(v2,a0,a1))
      return 1;
    v4 = __errno_location();
  }
  v2 = sub_14310(0,4,a0);
  v3 = dcgettext(NULL,"failed to set the security context of %s",5);
  error(0,*v4,v3,v2);
  return 0;
}


// Function: sub_75f0 @ 0x75f0
void sub_75f0(long a0)
{
  long v1; // rax
  
  v1 = sub_11bb0(0x3d,0,sub_12460,sub_12430,sub_124d0);
  *(long *)(a0 + 0x48) = v1;
  if (v1)
    return;
  sub_17150(); // no-return
}


// Function: sub_7630 @ 0x7630
void sub_7630(long a0)
{
  long v1; // rax
  
  v1 = sub_11bb0(0x3d,0,sub_12410,sub_12430,sub_124d0);
  *(long *)(a0 + 0x50) = v1;
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
unsigned long sub_76f0(void **a0,void **a1,int a2,void **a3,unsigned int a4,void **a5,unsigned long *a6,unsigned int *a7,unsigned int a8,void **a9,void **a10,char *a11)
{
  char *v1;
  unsigned int *v10; // rax
  unsigned int v100; // stack - 0x1dc
  unsigned int v101; // stack - 0x1d8
  void **v102; // stack - 0x1c8
  void *v103; // stack - 0x160
  void *v104; // stack - 0x158
  void *v105; // stack - 0x150
  void *v106; // stack - 0x148
  void *v107; // stack - 0x140
  void *v108; // stack - 0x138
  void **v109; // stack - 0x130
  char *v11;
  void *v110; // stack - 0x128
  void *v111; // stack - 0x120
  void *v112; // stack - 0x118
  void *v113; // stack - 0x110
  void *v114; // stack - 0x108
  void *v115; // stack - 0x100
  void *v116; // stack - 0xf8
  void *v117; // stack - 0xf0
  void *v118; // stack - 0xe8
  void *v119; // stack - 0xe0
  unsigned long v12;
  void *v120; // stack - 0xd0
  void *v121; // stack - 0xc8
  void *v122; // stack - 0xc0
  void *v123; // stack - 0xb8
  void *v124; // stack - 0xb0
  void **v125; // stack - 0xa8
  void *v126; // stack - 0xa0
  void *v127; // stack - 0x98
  void *v128; // stack - 0x90
  void *v129; // stack - 0x88
  unsigned long v13; // rax
  void *v130; // stack - 0x80
  void *v131; // stack - 0x78
  void *v132; // stack - 0x70
  void *v133; // stack - 0x68
  void *v134; // stack - 0x60
  void *v135; // stack - 0x58
  void *v136; // stack - 0x50
  void *v14; // rax
  int *v15; // rax
  void **v16;
  long v17;
  unsigned int v18;
  char v19 [8];
  char v2;
  void *v20; // stack - 0x288
  void *v21; // stack - 0x1f8
  void *v22; // stack - 0x168
  void *v23; // stack - 0xd8
  void *v24; // stack - 0x310
  char v25 [2]; // stack - 0x312
  long v26; // stack - 0x308
  void *v27; // stack - 0x2a8
  long v28;
  char *v29;
  unsigned char v3;
  char *v30;
  char *v31;
  char *v32;
  char *v33; // rsp
  char *v34; // rsp
  void **v35;
  void *v36;
  char v37;
  unsigned long v38;
  unsigned long *v39;
  char v4;
  unsigned long v40; // rax
  unsigned long v41; // r13
  unsigned int *v42;
  unsigned char v43;
  bool v44;
  unsigned long *v45; // stack - 0x3e0
  unsigned int v46; // stack - 0x3d8
  unsigned int v47; // stack - 0x3d4
  void **v48; // stack - 0x3d0
  unsigned int v49; // stack - 0x3c8
  int v5;
  unsigned int v50; // stack - 0x3c4
  unsigned int *v51; // stack - 0x3c0
  void **v52; // stack - 0x3b8
  char *v53; // stack - 0x3b0
  unsigned long *v54; // stack - 0x3a8
  unsigned int *v55; // stack - 0x3a0
  unsigned long *v56; // stack - 0x398
  void **v57; // stack - 0x390
  char *v58; // stack - 0x388
  void **v59; // stack - 0x380
  unsigned int v6;
  void **v60; // stack - 0x378
  void **v61; // stack - 0x370
  unsigned long v62; // stack - 0x368
  unsigned long *v63; // stack - 0x360
  void **v64; // stack - 0x358
  void **v65; // stack - 0x350
  char *v66; // stack - 0x348
  void **v67; // stack - 0x340
  void **v68; // stack - 0x338
  int v69; // stack - 0x330
  unsigned int v7;
  unsigned int v70; // stack - 0x32c
  void **v71; // stack - 0x328
  void **v72; // stack - 0x320
  unsigned long v73; // stack - 0x300
  unsigned long v74; // stack - 0x2f8
  unsigned long v75; // stack - 0x2f0
  unsigned long v76; // stack - 0x2e8
  unsigned long v77; // stack - 0x2e0
  unsigned long v78; // stack - 0x2d8
  unsigned long v79; // stack - 0x2d0
  unsigned int v8;
  unsigned long v80; // stack - 0x2c8
  unsigned long v81; // stack - 0x2c0
  unsigned long v82; // stack - 0x2b8
  void *v83; // stack - 0x2a0
  void *v84; // stack - 0x298
  void *v85; // stack - 0x290
  void *v86; // stack - 0x280
  unsigned long v87; // stack - 0x278
  unsigned int v88; // stack - 0x270
  unsigned int v89; // stack - 0x26c
  unsigned long *v9;
  unsigned int v90; // stack - 0x268
  unsigned long v91; // stack - 0x260
  unsigned long v92; // stack - 0x258
  void *v93; // stack - 0x240
  void *v94; // stack - 0x238
  void *v95; // stack - 0x230
  void *v96; // stack - 0x228
  void *v97; // stack - 0x1f0
  unsigned long v98; // stack - 0x1e8
  unsigned int v99; // stack - 0x1e0
  
  v42 = a7;
  v31 = v19;
  v32 = v19;
  v34 = v19;
  v29 = v19;
  v68 = a1;
  v67 = a3;
  v63 = a6;
  v69 = a2;
  v60 = a5;
  v71 = (void **)CONCAT44(v71._4_4_,a8);
  v64 = a9;
  v8 = a7[0x10];
  v61 = a10;
  v66 = a11;
  v62 = CONCAT71(v62._1_7_,(char)a8);
  *(unsigned char *)a10 = 0;
  v72 = (void **)CONCAT71(v72._1_7_,(char)a7[6]);
  if ((char)a7[6]) {
    if ((int)v8 < 0) {
      if (!sub_14b50(0xffffff9c,a0,a2,a3,1)) {
        a4 = 1;
        *v66 = 1;
        goto label_79ae;
      }
      v8 = *__errno_location();
    }
    v44 = v8 == 0;
    v72 = (void **)CONCAT71(v72._1_7_,v44);
    a4 = (unsigned int)v44;
    *v66 = v44;
    if (!v8) goto label_79ae;
label_779d:
    if ((v8 != 0x11) || (v42[2] != 2)) {
      v35 = a0;
      v16 = a0;
      v5 = -100;
      v70 = a4;
      goto label_77b2;
    }
label_7803:
    if (((char)v71) && (v17 = *(long *)&v42[0x14], v17)) {
      v16 = &v20;
      if (((v70 & 0xf000) != 0x4000) && (!*v42)) {
        v59 = (void **)CONCAT44(v59._4_4_,a4);
        v65 = &v20;
        v7 = sub_e820(v17,a0,&v20);
        v38 = (unsigned long)v7;
        if ((char)v7) {
          v12 = sub_14440(4,a0);
          v11 = "warning: source file %s specified more than once";
          v11 = dcgettext(NULL,v11,5); // return-dupe
          error(0,0,v11,v12); // return-dupe
          return v38 & 0xffffffff; // return-dupe
        }
        v17 = *(long *)&v42[0x14];
        v16 = v65;
        a4 = (unsigned int)v59;
      }
      v65 = (void **)CONCAT44(v65._4_4_,a4);
      sub_e790(v17,a0,v16);
      a4 = (unsigned int)v65;
    }
    if (v42[1] == 4) {
      v59 = (void **)CONCAT44(v59._4_4_,1);
label_787c:
      v65 = NULL;
      v43 = 0;
      v32 = v19;
      if (1 <= (int)a4) goto label_7892;
label_7b88:
      if ((v8 == 0x11) && (v42[2] == 2)) {
        v43 = 0;
        v2 = '\0';
label_8af3:
        if (!*(char *)((long)v42 + 0x3b)) {
          if ((char)v42[6]) goto label_910b;
          if ((v70 & 0xf000) != 0x4000) {
label_8c1d:
            if ((v42[2] == 2) || ((v42[2] == 3 && (v4 = sub_67a0(v42,v68,v69,v67,&v21), !v4)))) {
              v38 = 1; // return-dupe
              return v38 & 0xffffffff;
            }
          }
          goto label_8c33;
        }
        if ((v70 & 0xf000) != 0x4000) { // branch-flip
          v44 = 0;
          if ((*(char *)((long)v42 + 0x1f)) && (v44 = 1, (char)v42[6]))
            v44 = v21 != v20;
          v5 = sub_15b50(v69,v67,&v21,&v20,v44);
          if (v5 < 0) {
            if (!(char)v42[6]) goto label_8c1d;
            goto label_910b;
          }
          if (v66)
            *v66 = 1;
          v17 = sub_bd30(v67,v86,v20);
          if ((v17) && (v2 = sub_65d0(0,v69,v17,v68,v69,v67,1,(char)v42[0xf],(unsigned long)v59 & 0xffffffff), !v2)) {
            if (*(char *)((long)v42 + 0x33)) {
              v32 = v19;
              if (setfscreatecon(0)) goto label_8bf7;
            }
            v38 = 0; // return-dupe
            return v38 & 0xffffffff;
          }
        }
        else {
          if (!(char)v42[6]) goto label_8c33;
label_910b:
          v8 = v42[2];
          if (v8 != 2) {
            if (v8 != 3) {
              if ((v8 != 4) || (!*(char *)((long)v42 + 0x3d))) {
label_8c33:
                if (v2) {
                  v38 = 1;
                  return v38 & 0xffffffff;
                }
                if ((v99 & 0xf000) != 0x4000) goto label_8c56;
                if ((v70 & 0xf000) != 0x4000) {
label_9818:
                  if ((char)v42[6]) {
                    v38 = (unsigned long)*v42;
                    v8 = v99;
                    if (*v42) goto label_982f;
                  }
                  v12 = sub_14440(4,v68);
                  v38 = 0;
                  error(0,0,dcgettext(NULL,"cannot overwrite directory %s with non-directory",5),v12);
                  return v38 & 0xffffffff;
                }
label_8cc9:
                v38 = (unsigned long)*v42;
label_8ccc:
                if ((char)v42[6]) { // branch-flip
                  if ((v88 & 0xf000) == 0x4000) {
                    v8 = v99;
                    goto label_a0b6;
                  }
label_9e4c:
                  if ((int)v38) goto label_9847;
label_8d80:
                  v65 = NULL;
                  v8 = 0x11;
                  v32 = v19;
                }
                else {
                  v3 = 0;
                  if ((int)v38) goto label_9e1b;
label_8ce2:
                  v8 = 0x11;
                  v65 = NULL;
                  v3 = (v99 & 0xf000) == 0x4000 | v3;
                  v38 = CONCAT71((undefined7)(v38 >> 8),v3);
                  v32 = v19;
                  if (!v3) {
                    if (!*(char *)((long)v42 + 0x15)) {
                      v32 = v19;
                      if (!*(char *)((long)v42 + 0x31)) goto label_7892;
                      if ((!(char)v42[0xc]) || (v98 <= 1)) {
                        v8 = 0x11;
                        v65 = NULL;
                        v32 = v19;
                        if ((v42[1] != 2) || (v32 = v19, (v88 & 0xf000) == 0x8000)) goto label_7892;
                      }
                    }
                    v65 = NULL;
                    v5 = unlinkat(v69,(char *)v67,0);
                    if ((v5) && (v15 = __errno_location(), *v15 != 2)) {
                      v12 = sub_14440(4,v68);
                      v11 = dcgettext(NULL,"cannot remove %s",5);
                      error(0,*v15,v11,v12);
                      return v38 & 0xffffffff;
                    }
                    v72 = (void **)CONCAT71(v72._1_7_,(char)v42[0xf]);
                    if (!(char)v42[0xf]) {
                      v72 = (void **)CONCAT71(v72._1_7_,1);
                      goto label_8d80;
                    }
                    v8 = 0x11;
                    v12 = sub_14440(4,v68);
                    __printf_chk(1,dcgettext(NULL,"removed %s\n",5),v12);
                    v65 = NULL;
                    v32 = v19;
                  }
                }
              }
              else {
                if ((v99 & 0xf000) != 0xa000) {
                  if ((!sub_16bb0()) && (v5 = faccessat(v69,v67,2,0x200), v5)) goto label_917c;
                  goto label_8c33;
                }
                if (v2) {
                  v38 = 1;
                  return v38 & 0xffffffff;
                }
label_8c56:
                v7 = v70 & 0xf000;
                v8 = v99;
                if (v7 != 0x4000) { // branch-flip
                  v38 = (unsigned long)*v42;
                  if (!(char)v71) goto label_8ccc;
label_8c7d:
                  if ((int)v38 != 3) {
                    v2 = sub_e820(*(unsigned long *)&v42[0x12],v67,&v21);
                    if (v2) {
                      v12 = sub_14310(1,4,a0);
                      v13 = sub_14310(0,4,v68);
                      v11 = "will not overwrite just-created %s with %s";
                      error(0,0,dcgettext(NULL,v11,5),v13,v12); // return-dupe
                      v38 = 0;
                      return v38 & 0xffffffff;
                    }
                    if ((v7 == 0x4000) || ((v99 & 0xf000) != 0x4000)) goto label_8cc9;
                    goto label_9818;
                  }
                  if ((char)v42[6]) goto label_982f;
label_9e1b:
                  v5 = (int)v38;
                  v11 = (char *)sub_d970(a0);
                  if (*v11 == '.') {
                    v3 = 0;
                    goto label_985e;
                  }
label_9883:
                  if ((v99 & 0xf000) == 0x4000) goto label_8d80;
                }
                else {
                  if ((!(char)v42[6]) || (v38 = (unsigned long)*v42, !*v42)) {
                    v12 = sub_14310(1,4,a0);
                    v13 = sub_14310(0,4,v68);
                    v11 = "cannot overwrite non-directory %s with directory %s";
                    error(0,0,dcgettext(NULL,v11,5),v13,v12);
                    v38 = 0;
                    return v38 & 0xffffffff;
                  }
                  if ((char)v71) goto label_8c7d;
label_982f:
                  if ((v88 & 0xf000) == 0x4000) {
label_a0b6:
                    if ((v8 & 0xf000) == 0x4000) goto label_9e4c;
                    if (!(int)v38) {
                      v12 = sub_147c0(0,3,v68);
                      v13 = sub_147c0(0,3,a0);
                      v11 = "cannot move directory onto non-directory: %s -> %s";
                      error(0,0,dcgettext(NULL,v11,5),v13,v12);
                      v38 = 0;
                      return v38 & 0xffffffff;
                    }
                  }
label_9847:
                  v5 = (int)v38;
                  v11 = (char *)sub_d970(a0);
                  v3 = 1;
                  if (*v11 == '.') {
label_985e:
                    v5 = (int)v38;
                    if ((!v11[(unsigned long)(v11[1] == '.') + 1]) || (v11[(unsigned long)(v11[1] == '.') + 1] == '/')) goto label_8ce2;
                    if (!v3) goto label_9883;
                  }
                }
                if (v5 != 3) {
                  v72 = (void **)strlen(v11);
                  v58 = (char *)sub_d970(v67);
                  v65 = (void **)strlen(v58);
                  v1 = dat_23580;
                  v38 = strlen(dat_23580);
                  if (v72 == (void **)((long)v65 + v38)) {
                    v72 = v65;
                    v5 = memcmp(v11,v58,(unsigned long)v65);
                    if (!v5) {
                      v5 = strcmp(&v11[(long)v72],v1);
                      v16 = v67;
                      if (!v5) {
                        v38 = strlen((char *)v67);
                        v11 = (char *)sub_6110(v16,(long)v16 + v38,v1);
                        v5 = fstatat(v69,v11,&v23,0);
                        free(v11);
                        if (((!v5) && (v86 == v120)) && (v20 == v23)) {
                          if ((char)v42[6]) // branch-flip
                            v11 = dcgettext(NULL,"backing up %s might destroy source;  %s not moved",5);
                          else {
                            v11 = dcgettext(NULL,"backing up %s might destroy source;  %s not copied",5);
                          }
                          v12 = sub_14310(1,4,a0);
                          v13 = sub_14310(0,4,v68);
                          v38 = 0;
                          error(0,0,v11,v13,v12);
                          return v38 & 0xffffffff;
                        }
                      }
                    }
                  }
                }
                v16 = v67;
                v11 = (char *)sub_d840(v69,v67,*v42);
                if (v11) { // branch-flip
                  v41 = (long)v16 - (long)v68;
                  v40 = strlen(v11);
                  v38 = v41 + 0x18 + v40;
                  v32 = v19;
                  while (v34 != &v19[-(v38 & 0xfffffffffffff000)]) {
                    v29 = &v32[-0x1000];
                    v33 = &v32[-0x1000];
                    *(unsigned long *)&v32[-8] = *(unsigned long *)&v32[-8];
                    v34 = &v32[-0x1000];
                    v32 = v33;
                  }
                  v38 = (unsigned long)((unsigned int)v38 & 0xff0);
                  v17 = -v38;
                  if (v38)
                    *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
                  v72 = (void **)(v40 + 1);
                  v8 = 0x11;
                  v65 = (void **)((unsigned long)&v29[v17 + 0xf] & 0xfffffffffffffff0);
                  *(unsigned long *)&v29[v17 + -8] = 0x9a35;
                  v36 = mempcpy((void **)((unsigned long)&v29[v17 + 0xf] & 0xfffffffffffffff0),v68,v41,v29[v17 + -8]);
                  *(unsigned long *)&v29[v17 + -8] = 0x9a47;
                  memcpy(v36,v11,(unsigned long)v72,v29[v17 + -8]);
                  *(unsigned long *)&v29[v17 + -8] = 0x9a4f;
                  free(v11,v29[v17 + -8]);
                  v72 = (void **)CONCAT71(v72._1_7_,1);
                  v32 = &v29[v17];
                }
                else {
                  v15 = __errno_location();
                  if (*v15 != 2) {
                    v12 = sub_14440(4,v68);
                    v11 = dcgettext(NULL,"cannot backup %s",5);
                    v38 = 0;
                    error(0,*v15,v11,v12);
                    return v38 & 0xffffffff;
                  }
                  v72 = (void **)CONCAT71(v72._1_7_,1);
                  v8 = 0x11;
                  v65 = NULL;
                  v32 = v19;
                }
              }
label_7892:
              if ((char)v71) goto label_7ad0;
              goto label_789f;
            }
label_917c:
            v4 = sub_67a0(v42,v68,v69,v67,&v21);
            if (v4) goto label_8c33;
          }
label_91b0:
          if (v66)
            *v66 = 1;
        }
        v38 = 1;
        return v38 & 0xffffffff;
      }
      v7 = v70 & 0xf000;
      if ((((v7 != 0x8000) && (v3 = (v7 == 0xa000 || v7 == 0x4000) | (unsigned char)v42[5] ^ 1, v3)) || (v3 = (unsigned char)v42[6], v3)) || ((v3 = *(unsigned char *)((long)v42 + 0x3a), v3 || (v3 = *(unsigned char *)((long)v42 + 0x17), v3)))) {
label_7bca:
        v5 = 0x100;
label_7bd0:
        v5 = fstatat(v69,(char *)v67,&v21,v5);
        if (v5) {
          v15 = __errno_location();
          if (*v15 != 0x28) { // branch-flip
            if (*v15 == 2) goto label_8460;
          }
          else if (*(char *)((long)v42 + 0x16)) goto label_8467;
          v12 = sub_14440(4,v68);
          v11 = dcgettext(NULL,"cannot stat %s",5);
          v38 = 0;
          error(0,*v15,v11,v12);
          return v38 & 0xffffffff;
        }
      }
      else {
        if (*v42) {
          v3 = 1;
          goto label_7bca;
        }
        v3 = *(unsigned char *)((long)v42 + 0x15);
        if (v3) goto label_7bca;
        v5 = 0;
        if (!a4) goto label_7bd0;
label_8460:
        v72 = (void **)CONCAT71(v72._1_7_,1);
label_8467:
        v65 = NULL;
        v43 = 0;
        v32 = v19;
        v3 = 0;
        if (v8 != 0x11) goto label_7892;
      }
      v43 = v3;
      if (v42[2] == 2) goto label_8af0;
      if ((v86 == v97) && (v20 == v21)) {
        v2 = *(char *)((long)v42 + 0x17);
        if (!v2) {
          if (v42[1] == 2) {
            v44 = 1;
            v18 = 1;
            goto label_7c23;
          }
          v16 = &v22;
          v5 = fstatat(v69,(char *)v67,v16,0x100);
          if (!v5) {
            v65 = &v23;
            v5 = lstat((char *)a0,v65);
            if (!v5) {
              v44 = v120 == v103 && v23 == v22;
              v35 = v65;
              if (((((unsigned int)v122 & 0xf000) != 0xa000) || (((unsigned int)v105 & 0xf000) != 0xa000)) || (!*(char *)((long)v42 + 0x15))) goto label_7c47;
            }
          }
        }
        goto label_8af3;
      }
      if (v42[1] != 2) goto label_8af0;
      v44 = 0;
      v18 = 0;
label_7c23:
      v16 = &v21;
      v35 = &v20;
      if (((v88 & 0xf000) != 0xa000) || ((v99 & 0xf000) != 0xa000)) {
label_7c47:
        if (*v42) { // branch-flip
          if (!v44) {
            if (((((char)v42[6]) || (v42[1] == 2)) || (((unsigned long)v35[3] & 0xf000) != 0xa000)) || (((unsigned long)v16[3] & 0xf000) == 0xa000)) goto label_8af0;
            goto label_7c99;
          }
          v2 = '\0';
          v3 = sub_14e20(0xffffff9c,a0,v69,v67);
          v3 ^= 1;
label_a173:
          if (v3) goto label_8af3;
        }
        else {
          v2 = (char)v42[6];
          if ((v2) || (*(char *)((long)v42 + 0x15))) {
            if (((unsigned long)v16[3] & 0xf000) == 0xa000) goto label_8af0;
            if ((v44) && ((void *)0x2 <= v16[2])) {
              v65 = v35;
              v2 = sub_14e20(0xffffff9c,a0,v69,v67);
              if (v2) {
                v35 = v65;
                if (((unsigned long)v65[3] & 0xf000) != 0xa000) goto label_9bb4;
                goto label_9bc9;
              }
              v3 = (unsigned char)v42[6] ^ 1;
              v2 = '\0';
              goto label_a173;
            }
            if (((unsigned long)v35[3] & 0xf000) != 0xa000) goto label_9e99;
label_9bce:
            if (((v2) && ((v88 & 0xf000) == 0xa000)) && ((void *)0x2 <= v16[2])) {
              v65 = v35;
              v11 = canonicalize_file_name((char *)a0);
              v35 = v65;
              if (v11) {
                v3 = sub_14e20(0xffffff9c,v11,v69,v67);
                v2 = '\0';
                v3 ^= 1;
                free(v11);
                goto label_a173;
              }
            }
          }
          else if (((unsigned long)v35[3] & 0xf000) != 0xa000) {
label_9bb4:
            if (((unsigned long)v16[3] & 0xf000) != 0xa000) {
label_9e99:
              if ((v35[1] != v16[1]) || (*v35 != *v16)) goto label_8af0;
              v2 = *(char *)((long)v42 + 0x17);
              if (v2) goto label_8af3;
            }
label_9bc9:
            v2 = (char)v42[6];
            goto label_9bce;
          }
          if ((*(char *)((long)v42 + 0x3a)) && (((unsigned long)v16[3] & 0xf000) == 0xa000)) {
label_8af0:
            v2 = '\0';
            goto label_8af3;
          }
          if (v42[1] == 2) {
            if (((unsigned long)v35[3] & 0xf000) != 0xa000) { // branch-flip
              v23 = *v35;
              v120 = v35[1];
              v121 = v35[2];
              v122 = v35[3];
              v123 = v35[4];
              v124 = v35[5];
              v125 = v35[6];
              v126 = v35[7];
              v127 = v35[8];
              v128 = v35[9];
              v129 = v35[10];
              v130 = v35[0xb];
              v131 = v35[0xc];
              v132 = v35[0xd];
              v133 = v35[0xe];
              v134 = v35[0xf];
              v135 = v35[0x10];
              v136 = v35[0x11];
            }
            else if (stat((char *)a0,&v23)) goto label_8af0;
            if (((unsigned long)v16[3] & 0xf000) != 0xa000) { // branch-flip
              v22 = *v16;
              v103 = v16[1];
              v104 = v16[2];
              v105 = v16[3];
              v106 = v16[4];
              v107 = v16[5];
              v108 = v16[6];
              v109 = v16[7];
              v110 = v16[8];
              v111 = v16[9];
              v112 = v16[10];
              v113 = v16[0xb];
              v114 = v16[0xc];
              v115 = v16[0xd];
              v116 = v16[0xe];
              v117 = v16[0xf];
              v118 = v16[0x10];
              v119 = v16[0x11];
            }
            else {
              v5 = fstatat(v69,(char *)v67,&v22,0);
              if (v5) goto label_8af0;
            }
            if ((v120 != v103) || (v23 != v22)) goto label_8af0;
            if (*(char *)((long)v42 + 0x17)) {
              v2 = ((unsigned long)v16[3] & 0xf000) != 0xa000;
              goto label_8af3;
            }
          }
        }
      }
      else {
        v65 = (void **)CONCAT44(v65._4_4_,v18);
        v2 = sub_14e20(0xffffff9c,a0,v69,v67);
        if (!v2) {
          v2 = '\0';
          if ((!*v42) && ((int)v65)) {
            v2 = '\x01';
            v3 = (unsigned char)v42[6] ^ 1;
            goto label_a173;
          }
          goto label_8af3;
        }
      }
label_7c99:
      v12 = sub_14310(1,4,v68);
      v13 = sub_14310(0,4,a0);
      v11 = "%s and %s are the same file";
      error(0,0,dcgettext(NULL,v11,5),v13,v12);
      v38 = 0;
      return v38 & 0xffffffff;
    }
    if ((v42[1] != 3) || (!(char)v71)) {
      v59 = (void **)((unsigned long)v59 & 0xffffffff00000000);
      goto label_787c;
    }
    v59 = (void **)CONCAT44(v59._4_4_,1);
    if ((int)a4 <= 0) goto label_7b88;
    v65 = NULL;
    v43 = 0;
label_7ad0:
    if ((!*(long *)&v42[0x12]) || ((char)v42[6])) {
label_789f:
      if (((char)v42[0xf]) && (!(char)v42[6])) {
label_8258:
        if ((v70 & 0xf000) != 0x4000) {
          *(unsigned long *)&v32[-8] = 0x8284;
          sub_6510(a0,v68,v65);
        }
      }
      if (v8) goto label_78ba;
      v29 = v32;
      if ((char)v42[6]) goto label_7d27;
label_7d98:
      v40 = (unsigned long)v72 & 0xff;
      v9 = NULL;
label_7da3:
      v8 = v70;
      if (*(char *)((long)v42 + 0x39))
        v8 = v42[4];
      v32 = v29;
      if (*(char *)((long)v42 + 0x1d)) {
        *(unsigned long *)&v29[-8] = 0x7e4b;
        v2 = sub_72a0(a0,v68,v70,v40,v42);
        if (!v2) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
        v58 = (char *)(CONCAT44(v58._4_4_,v8) & 0xffffffff0000003f);
        v66 = (char *)(CONCAT44(v66._4_4_,v70) & 0xffffffff0000f000);
        v39 = v63;
        if ((v70 & 0xf000) == 0x4000) goto label_8356;
label_7e7a:
        v7 = (unsigned int)*(unsigned char *)((long)v42 + 0x3a);
        if (!*(unsigned char *)((long)v42 + 0x3a)) {
          if (!*(char *)((long)v42 + 0x17)) {
            if (((unsigned int)v66 != 0x8000) && (!((unsigned int)v66 != 0xa000 & (unsigned char)v42[5]))) {
              v64 = (void **)((unsigned long)v64._1_7_ << 8);
              if ((unsigned int)v66 == 0x1000) {
                v8 = ~(unsigned int)v58 & v70;
                *(unsigned long *)&v29[-8] = 0xa2d6;
                v5 = mknodat(v69,v67,v8,0);
                v2 = (char)v64;
                if (v5) {
                  *(unsigned long *)&v29[-8] = 0xa2fd;
                  v5 = mkfifoat(v69,v67,v8 & 0xffffefff);
                  v2 = (char)v64;
                  if (v5) {
                    *(unsigned long *)&v29[-8] = 0xa31e;
                    v12 = sub_14440(4,v68);
                    v11 = "cannot create fifo %s";
                    goto label_8638;
                  }
                }
                goto label_7f17;
              }
              if (((unsigned int)v66 & 0xffffbfff) == 0x2000 || (unsigned int)v66 == 0xc000) {
                *(unsigned long *)&v29[-8] = 0x8601;
                v5 = mknodat(v69,v67,~(unsigned int)v58 & v70,v91);
                v2 = (char)v64;
                if (v5) {
                  *(unsigned long *)&v29[-8] = 0x8622;
                  v12 = sub_14440(4,v68);
                  v11 = "cannot create special file %s";
                  goto label_8638;
                }
                goto label_7f17;
              }
              if ((unsigned int)v66 == 0xa000) {
                *(unsigned long *)&v29[-8] = 0x9f44;
                v11 = (char *)sub_c960(a0,v92);
                if (!v11) {
                  *(unsigned long *)&v29[-8] = 0xae5a;
                  v12 = sub_14440(4,a0);
                  v11 = "cannot read symbolic link %s";
                  goto label_8638;
                }
                v37 = *(char *)((long)v42 + 0x16);
                *(unsigned long *)&v29[-8] = 0x9f70;
                v5 = sub_c000(v11,v69,v67,v37,0xffffffff);
                if (v5 <= 0) {
                  *(unsigned long *)&v29[-8] = 0xa90a;
                  free(v11,v29[-8]);
label_a90a:
                  if (*(char *)((long)v42 + 0x33)) {
                    *(unsigned long *)&v29[-8] = 0xae40;
                    if (setfscreatecon(0)) goto label_8bf7;
                  }
                  v7 = 1;
                  v2 = '\0';
                  if (*(char *)((long)v42 + 0x1d)) {
                    v64 = (void **)CONCAT71(v64._1_7_,*(char *)((long)v42 + 0x1d));
                    *(unsigned long *)&v29[-8] = 0xa954;
                    v5 = fchownat(v69,v67,v89,v90,0x100);
                    if (v5) { // branch-flip
                      *(unsigned long *)&v29[-8] = 0xa96c;
                      v7 = sub_76c0(v42);
                      v2 = '\0';
                      if (!(char)v7) {
                        *(unsigned long *)&v29[-8] = 0xa98d;
                        v11 = dcgettext(NULL,"failed to preserve ownership for %s",5,v29[-8]);
                        *(unsigned long *)&v29[-8] = 0xa995;
                        v37 = v29[-8];
                        v5 = *__errno_location(v37);
                        *(unsigned long *)&v29[-8] = 0xa9aa;
                        error(0,v5,v11,v68);
                        if (*(char *)((long)v42 + 0x32)) goto label_8658;
                        v7 = (unsigned int)(unsigned char)v64;
                        v2 = '\0';
                      }
                    }
                    else {
                      v7 = (unsigned int)(unsigned char)v64;
                      v2 = '\0';
                    }
                  }
                  goto label_7f17;
                }
                if (((*(char *)((long)v42 + 0x3b) == '\x01') && (!(char)v72)) && ((v99 & 0xf000) == 0xa000)) {
                  v64 = v102;
                  *(unsigned long *)&v29[-8] = 0xafb5;
                  v37 = v29[-8];
                  if (v64 == (void **)strlen(v11,v37)) {
                    *(unsigned long *)&v29[-8] = 0xafd7;
                    v16 = (void **)sub_caf0(v69,v67);
                    if (v16) {
                      v64 = v16;
                      *(unsigned long *)&v29[-8] = 0xaff2;
                      v37 = v29[-8];
                      if (!strcmp((char *)v16,v11,v37)) {
                        *(unsigned long *)&v29[-8] = 0xb006;
                        free(v64,v29[-8]);
                        *(unsigned long *)&v29[-8] = 0xb00e;
                        free(v11,v29[-8]);
                        goto label_a90a;
                      }
                      *(unsigned long *)&v29[-8] = 0xba71;
                      free(v64,v29[-8]);
                    }
                  }
                }
                *(unsigned long *)&v29[-8] = 0x9f97;
                free(v11,v29[-8]);
                *(unsigned long *)&v29[-8] = 0x9fa8;
                v12 = sub_14440(4,v68);
                *(unsigned long *)&v29[-8] = 0x9fbe;
                v11 = dcgettext(NULL,"cannot create symbolic link %s",5,v29[-8]);
                *(unsigned long *)&v29[-8] = 0x9fd0;
                error(0,v5,v11,v12);
                goto label_8658;
              }
              *(unsigned long *)&v29[-8] = 0xa42d;
              v12 = sub_14440(4,a0);
              v11 = "%s has unknown file type";
label_8394:
              *(unsigned long *)&v29[-8] = 0x839b;
              v11 = dcgettext(NULL,v11,5,v29[-8]);
              *(unsigned long *)&v29[-8] = 0x83ac;
              error(0,0,v11,v12);
              goto label_8658;
            }
            v7 = v42[1];
            v24 = NULL;
            v61 = (void **)CONCAT44(v61._4_4_,v88);
            v64 = (void **)CONCAT71(v64._1_7_,*(char *)((long)v42 + 0x31));
            v63 = (unsigned long *)CONCAT71(v63._1_7_,*(char *)((long)v42 + 0x35));
            *(unsigned long *)&v29[-8] = 0x946d;
            v7 = sub_e620(a0,(unsigned long)(v7 == 2) << 0x11);
            v38 = (unsigned long)v7;
            if ((int)v7 < 0) {
              *(unsigned long *)&v29[-8] = 0xa25e;
              v12 = sub_14440(4,a0);
              v11 = "cannot open %s for reading";
              goto label_8638;
            }
            *(unsigned long *)&v29[-8] = 0x9486;
            v37 = v29[-8];
            if (fstat(v7,&v23,v37)) { // branch-flip
              *(unsigned long *)&v29[-8] = 0xa4a8;
              v64 = (void **)sub_14440(4,a0);
              *(unsigned long *)&v29[-8] = 0xa4c2;
              v11 = dcgettext(NULL,"cannot fstat %s",5,v29[-8]);
              *(unsigned long *)&v29[-8] = 0xa4ca;
              v37 = v29[-8];
              v5 = *__errno_location(v37);
              v16 = v64;
label_a000:
              v36 = NULL;
              *(unsigned long *)&v29[-8] = 0xa00b;
              error(0,v5,v11,v16);
              v37 = 0;
            }
            else {
              if ((v86 != v120) || (v20 != v23)) {
                *(unsigned long *)&v29[-8] = 0x9fe2;
                v16 = (void **)sub_14440(4,a0);
                *(unsigned long *)&v29[-8] = 0x9ff8;
                v11 = dcgettext(NULL,"skipping file %s, as it was replaced while being copied",5,v29[-8]);
                v5 = 0;
                goto label_a000;
              }
              v53 = (char *)(CONCAT44(v53._4_4_,v8) & 0xffffffff000001ff);
              if ((char)v72) goto label_a500;
              *(unsigned long *)&v29[-8] = 0x94f8;
              v8 = sub_125a0(v69,v67,(-(unsigned int)((char)v64 == '\0') & 0xfffffe00) + 0x201);
              v40 = (unsigned long)v8;
              *(unsigned long *)&v29[-8] = 0x94ff;
              v16 = (void **)__errno_location(v29[-8]);
              v5 = *(int *)v16;
              if (0 <= (int)v8) {
                if ((!*(long *)&v42[10]) && (!*(char *)((long)v42 + 0x33))) {
label_953b:
                  v46 = 0;
                  v51 = (unsigned int *)((unsigned long)v51 & 0xffffffff00000000);
label_954f:
                  v8 = (unsigned int)v40;
                  if ((char)v64) {
                    if (v42[0x11]) {
                      *(unsigned long *)&v29[-8] = 0xaeee;
                      if (!ioctl(v8,0x40049409,v38)) goto label_a64a;
                      if (v42[0x11] == 2) {
                        *(unsigned long *)&v29[-8] = 0xaf13;
                        v61 = (void **)sub_14310(1,4,a0);
                        *(unsigned long *)&v29[-8] = 0xaf2d;
                        v63 = (unsigned long *)sub_14310(0,4,v68);
                        *(unsigned long *)&v29[-8] = 0xaf47;
                        v37 = v29[-8];
                        v64 = (void **)dcgettext(NULL,"failed to clone %s from %s",5,v37);
                        *(unsigned long *)&v29[-8] = 0xaf53;
                        v37 = v29[-8];
                        v5 = *__errno_location(v37);
                        *(unsigned long *)&v29[-8] = 0xaf73;
                        error(0,v5,(char *)v64,v63,v61);
                        v43 = 0;
                        goto label_96e5;
                      }
                    }
label_9568:
                    *(unsigned long *)&v29[-8] = 0x9576;
                    v37 = v29[-8];
                    if (fstat(v8,&v22,v37)) {
                      *(unsigned long *)&v29[-8] = 0xb1d5;
                      v63 = (unsigned long *)sub_14440(4,v68);
                      v11 = "cannot fstat %s";
                      goto label_b133;
                    }
                    v60 = (void **)((unsigned long)v60 & 0xffffffff00000000);
                    if ((unsigned int)v105 != ((unsigned int)v51 | (unsigned int)v105)) {
                      *(unsigned long *)&v29[-8] = 0x959d;
                      v5 = fchmod(v8,(unsigned int)v51 | (unsigned int)v105,v29[-8]);
                      v18 = v60._0_4_;
                      if (!v5)
                        v18 = v51._0_4_;
                      v51 = (unsigned int *)CONCAT44(v51._4_4_,v18);
                    }
                    if (!(char)v64) goto label_ac58;
                    v48 = v109;
                    v57 = v109;
                    if ((void **)0x1ffffffffffe0001 <= &v109[-0x4000]) {
                      v57 = (void **)0x20000;
                      v48 = (void **)0x200;
                      if ((char *)((long)v109 + -1) <= (char *)0x1fffffffffffffff)
                        v48 = v109;
                    }
                    if (((unsigned int)v122 & 0xf000) != 0x8000) {
                      v5 = 1;
label_ab10:
                      if ((((unsigned int)v105 & 0xf000) != 0x8000) || ((v42[3] != 3 && ((v5 == 1 || (v42[3] != 2)))))) {
                        v60 = (void **)CONCAT44(v60._4_4_,v5);
                        *(unsigned long *)&v29[-8] = 0xab3e;
                        sub_e5e0(v38,0,0,2);
                        v36 = v126;
                        if (0x1ffffffffffe0001 <= (unsigned long)((long)v126 - 0x20000U))
                          v36 = (void *)0x20000;
                        *(unsigned long *)&v29[-8] = 0xab78;
                        v16 = (void **)sub_da00(v36,v57,0x7fffffffffffffff);
                        if ((((unsigned int)v122 & 0xf000) == 0x8000) && (v125 < v57))
                          v57 = (void **)((long)v125 + 1);
                        v25[0] = 0;
                        v11 = &((char *)((long)v16 + -1))[(long)v57];
                        if (1 <= (long)&v11[-((unsigned long)v11 % (unsigned long)v16)])
                          v16 = (void **)&v11[-((unsigned long)v11 % (unsigned long)v16)];
                        v57 = v16;
                        v47 = (unsigned int)(v42[0x11] != 0);
                        if ((int)v60 == 3) {
                          v49 = 1;
label_b1f7:
                          v39 = v56;
                          v25[0] = 0;
                          v60 = v125;
                          if (0 <= (long)v56) {
                            v59 = a0;
                            v52 = NULL;
                            v45 = NULL;
                            v54 = (unsigned long *)CONCAT71(v54._1_7_,v64._0_1_);
                            v17 = 0;
                            v56 = v9;
                            v55 = v42;
                            do {
                              *(unsigned long *)&v29[-8] = 0xb266;
                              v16 = (void **)lseek(v38,v39,4);
                              if ((long)v16 <= -1) {
                                *(unsigned long *)&v29[-8] = 0xb277;
                                v37 = v29[-8];
                                if (*__errno_location(v37) != 6) goto label_b50c;
                                v16 = v60;
                                if ((long)v39 < (long)v60) goto label_b293;
                                *(unsigned long *)&v29[-8] = 0xb443;
                                v16 = (void **)lseek(v38,0,2);
                                if ((long)v16 < 0) goto label_b50c;
                                if ((long)v39 < (long)v16) {
                                  v60 = v16;
                                  goto label_b293;
                                }
label_b500:
                                v60 = v16;
                                v9 = v56;
                                v42 = v55;
                                a0 = v59;
                                goto label_b3ff;
                              }
                              if ((long)v60 < (long)v16)
                                v60 = v16;
label_b293:
                              *(unsigned long *)&v29[-8] = 0xb2a0;
                              if ((long)lseek(v38,v39,0) < 0) {
label_b50c:
                                a0 = v59;
                                goto label_b521;
                              }
                              v17 = (long)v39 + (-v17 - (long)v45);
                              if (v17) {
                                if (v49 != 1) {
                                  *(unsigned long *)&v29[-8] = 0xb2e1;
                                  v2 = sub_6920(v40,v68,v49 == 3);
                                  v54 = (unsigned long *)CONCAT71(v54._1_7_,v2);
                                  if (v2) {
                                    v17 = (long)v16 - (long)v39;
                                    v16 = v48;
                                    goto label_b2fc;
                                  }
label_b575:
                                  v9 = v56;
                                  v42 = v55;
                                  a0 = v59;
                                  goto label_b56d;
                                }
                                *(unsigned long *)&v29[-8] = 0xb46e;
                                v2 = sub_6170(v40,v17);
                                a0 = v59;
                                v9 = v56;
                                v42 = v55;
                                if (v2) {
                                  v17 = (long)v16 - (long)v39;
                                  v54 = (unsigned long *)((unsigned long)v54 & 0xffffffffffffff00);
                                  v16 = NULL;
                                  goto label_b2fc;
                                }
                                *(unsigned long *)&v29[-8] = 0xb982;
                                v60 = (void **)sub_147c0(0,3,v68);
                                v11 = "%s: write failed";
                                goto label_b541;
                              }
                              v17 = (long)v16 - (long)v39;
                              v16 = NULL;
                              if (v49 != 1)
                                v16 = v48;
                              v54 = (unsigned long *)((unsigned long)v54 & 0xffffffffffffff00);
label_b2fc:
                              a0 = v59;
                              *(char **)&v29[-8] = &v25[1];
                              *(unsigned long **)&v29[-0x10] = &v26;
                              *(long *)&v29[-0x18] = v17;
                              *(void ***)&v29[-0x20] = v68;
                              *(void ***)&v29[-0x28] = v59;
                              *(unsigned long *)&v29[-0x30] = (unsigned long)v47;
                              *(unsigned long *)&v29[-0x38] = 0xb342;
                              v2 = sub_6a30(v38,v40,&v24,v57,v16,1);
                              if (!v2) goto label_b575;
                              v16 = (void **)(v26 + (long)v39);
                              v43 = (unsigned char)v54;
                              if (v26)
                                v43 = v25[1];
                              v52 = v16;
                              v54 = (unsigned long *)CONCAT71(v54._1_7_,v43);
                              if (v26 < v17) goto label_b500;
                              *(unsigned long *)&v29[-8] = 0xb391;
                              v9 = (unsigned long *)lseek(v38,v16,3);
                              if ((long)v9 < 0) goto label_b3e3;
                              v45 = v39;
                              v39 = v9;
                            } while( true );
                          }
                          v52 = NULL;
                          v44 = 0 < (long)v125;
                          goto label_b5a2;
                        }
                        v48 = NULL;
                        v6 = v42[3];
                      }
                      else {
                        v60 = (void **)CONCAT44(v60._4_4_,v5);
                        *(unsigned long *)&v29[-8] = 0xb0b5;
                        sub_e5e0(v38,0,0,2);
                        v25[0] = 0;
                        if ((int)v60 == 3) {
                          v47 = (unsigned int)(v42[0x11] != 0);
                          v49 = v42[3];
                          goto label_b1f7;
                        }
                        v6 = v42[3];
                        v47 = (unsigned int)(v42[0x11] != 0);
                      }
                      v25[0] = 0;
                      *(char **)&v29[-8] = v25;
                      *(unsigned long **)&v29[-0x10] = &v26;
                      *(unsigned long *)&v29[-0x18] = 0xffffffffffffffff;
                      *(void ***)&v29[-0x20] = v68;
                      *(void ***)&v29[-0x28] = a0;
                      *(unsigned long *)&v29[-0x30] = (unsigned long)v47;
                      *(unsigned long *)&v29[-0x38] = 0xac38;
                      v2 = sub_6a30(v38,v40,&v24,v57,v48,v6 == 3);
                      goto label_ac42;
                    }
                    v5 = 1;
                    if ((long)v125 / 0x200 <= (long)v127) goto label_ab10;
                    *(unsigned long *)&v29[-8] = 0x9662;
                    v56 = (unsigned long *)lseek(v38,0,3);
                    if (0 <= (long)v56) {
label_b4d9:
                      v5 = 3;
                      goto label_ab10;
                    }
                    *(unsigned long *)&v29[-8] = 0x9677;
                    v60 = (void **)__errno_location(v29[-8]);
                    v5 = *(int *)v60;
                    if (v5 == 6) goto label_b4d9;
                    if ((v5 == 0x16) || (v5 == 0x5f)) {
                      v5 = 2;
                      goto label_ab10;
                    }
                    *(unsigned long *)&v29[-8] = 0x96ab;
                    v64 = (void **)sub_14440(4,a0);
                    *(unsigned long *)&v29[-8] = 0x96c5;
                    v11 = dcgettext(NULL,"cannot lseek %s",5,v29[-8]);
                    v5 = *(int *)v60;
                    *(unsigned long *)&v29[-8] = 0x96e2;
                    error(0,v5,v11,v64);
                    goto label_96e2;
                  }
label_a64a:
                  v6 = *(unsigned char *)((long)v42 + 0x1d) | (unsigned int)v51;
                  v64 = (void **)CONCAT44(v64._4_4_,v6);
                  if (v6) {
                    v64 = (void **)((unsigned long)v64 & 0xffffffffffffff00);
                    goto label_9568;
                  }
                  v105 = (void *)((unsigned long)v105 & 0xffffffff00000000);
                  if (*(char *)((long)v42 + 0x1f)) goto label_a676;
                  goto label_a74a;
                }
                *(unsigned long *)&v29[-8] = 0x9529;
                v2 = sub_7510(v68,0,v42);
                if ((v2) || (!(char)v42[0xd])) goto label_953b;
                v72 = (void **)((unsigned long)v72 & 0xffffffffffffff00);
                v43 = 0;
                goto label_96e5;
              }
              if (v5 != 2) {
                v2 = '\0';
                if (*(char *)((long)v42 + 0x16)) {
                  v60 = v16;
                  *(unsigned long *)&v29[-8] = 0xad87;
                  v5 = unlinkat(v69,(char *)v67,0,v29[-8]);
                  if (v5) { // branch-flip
                    if (*(int *)v60 != 2) {
                      v64 = v60;
                      *(unsigned long *)&v29[-8] = 0xb6b1;
                      v16 = (void **)sub_14440(4,v68);
                      *(unsigned long *)&v29[-8] = 0xb6c7;
                      v11 = dcgettext(NULL,"cannot remove %s",5,v29[-8]);
                      v5 = *(int *)v64;
                      goto label_a000;
                    }
                  }
                  else if ((char)v42[0xf]) {
                    *(unsigned long *)&v29[-8] = 0xb7dc;
                    v12 = sub_14440(4,v68);
                    *(unsigned long *)&v29[-8] = 0xb7f2;
                    v11 = dcgettext(NULL,"removed %s\n",5,v29[-8]);
                    *(unsigned long *)&v29[-8] = 0xb804;
                    __printf_chk(1,v11,v12);
                  }
                  goto label_ada1;
                }
label_a58d:
                v72 = (void **)CONCAT71(v72._1_7_,v2);
label_a593:
                v64 = (void **)CONCAT44(v64._4_4_,v5);
                *(unsigned long *)&v29[-8] = 0xa5ab;
                v16 = (void **)sub_14440(4,v68);
                *(unsigned long *)&v29[-8] = 0xa5c1;
                v11 = dcgettext(NULL,"cannot create regular file %s",5,v29[-8]);
                v5 = (int)v64;
                goto label_a000;
              }
label_ada1:
              if (!*(long *)&v42[10]) {
label_a500:
                v8 = ~(unsigned int)v58 & (unsigned int)v53;
                if (((char)v63) && (!*(char *)((long)v42 + 0x1b)))
                  v8 |= 0x80;
                v59 = (void **)CONCAT44(v59._4_4_,v8);
                *(unsigned long *)&v29[-8] = 0xa548;
                v8 = sub_125a0(v69,v67,0xc1,v8);
                v40 = (unsigned long)v8;
                *(unsigned long *)&v29[-8] = 0xa54f;
                v60 = (void **)__errno_location(v29[-8]);
                v5 = *(int *)v60;
                v43 = v5 == 0x11 & (unsigned char)(v8 >> 0x1f);
                v72 = (void **)CONCAT71(v72._1_7_,v43);
                v8 = (unsigned int)v59;
                if (v43) {
                  v2 = (char)v42[6];
                  v5 = 0x11;
                  if (v2) goto label_a58d;
                  v57 = (void **)CONCAT44(v57._4_4_,0x11);
                  *(unsigned long *)&v29[-8] = 0xaa39;
                  v17 = readlinkat(v69,v67,&v28,1);
                  v5 = (int)v57;
                  v8 = (unsigned int)v59;
                  if (0 <= v17) {
                    if (!*(char *)((long)v42 + 0x3e)) {
                      v64 = (void **)((unsigned long)v64 & 0xffffffffffffff00);
                      *(unsigned long *)&v29[-8] = 0xb873;
                      v12 = sub_14440(4,v68);
                      *(unsigned long *)&v29[-8] = 0xb889;
                      v11 = dcgettext(NULL,"not writing through dangling symlink %s",5,v29[-8]);
                      v36 = NULL;
                      *(unsigned long *)&v29[-8] = 0xb89c;
                      error(0,0,v11,v12);
                      v37 = SUB81(v64,0);
                      goto label_9710;
                    }
                    v72 = (void **)CONCAT44(v72._4_4_,(unsigned int)v59);
                    *(unsigned long *)&v29[-8] = 0xaa7d;
                    v8 = sub_125a0(v69,v67,0x41,(unsigned long)v59 & 0xffffffff);
                    v40 = (unsigned long)v8;
                    v5 = *(int *)v60;
                    v8 = (unsigned int)v72;
                  }
                }
                v16 = v68;
                v43 = v5 == 0x15 & (unsigned char)(v40 >> 0x1f);
                v72 = (void **)CONCAT71(v72._1_7_,v43);
                if (v43) { // branch-flip
                  v5 = 0x15;
                  if (*(char *)v68) {
                    *(unsigned long *)&v29[-8] = 0xaac4;
                    v38 = strlen((char *)v68,v29[-8]);
                    v5 = (*(char *)((long)v16 + (v38 - 1)) != '/') + 0x14;
                  }
                }
                else {
                  v51 = (unsigned int *)CONCAT44(v51._4_4_,~(unsigned int)v53 & v8);
                  if (0 <= (int)v40) {
                    v72 = (void **)CONCAT71(v72._1_7_,1);
                    v46 = (unsigned int)v58;
                    goto label_954f;
                  }
                  v72 = (void **)CONCAT71(v72._1_7_,1);
                }
                goto label_a593;
              }
              *(unsigned long *)&v29[-8] = 0xadc9;
              v2 = sub_72a0(a0,v68,(unsigned long)v53 & 0xffffffff,1,v42);
              if (v2) goto label_a500;
              v72 = (void **)((unsigned long)v72 & 0xffffffffffffff00);
              v36 = NULL;
              v37 = 0;
            }
            goto label_9710;
          }
          v38 = 1;
          if (!*(char *)((long)v42 + 0x16))
            v38 = (unsigned long)(v42[2] == 3);
          *(unsigned long *)&v29[-0x10] = (unsigned long)v59 & 0xffffffff;
          *(unsigned long *)&v29[-0x18] = 0;
          *(unsigned long *)&v29[-0x20] = v38;
          *(unsigned long *)&v29[-0x28] = 0x8ea9;
          v4 = sub_65d0(a0,0xffffff9c,a0,v68,v69,v67);
          v2 = '\0';
          goto label_8eb2;
        }
        if (*(char *)a0 != '/') {
          *(unsigned long *)&v29[-8] = 0x7e9a;
          v11 = (char *)sub_e480(v67);
          if (v69 != -100) { // branch-flip
label_7ece:
            *(unsigned long *)&v29[-8] = 0x7eda;
            v37 = v29[-8];
            if (!stat(".",&v22,v37)) {
              *(unsigned long *)&v29[-8] = 0x9aff;
              v5 = fstatat(v69,v11,&v23,0,v29[-8]);
              if ((!v5) && ((v103 != v120 || (v22 != v23)))) {
                *(unsigned long *)&v29[-8] = 0x9b23;
                free(v11,v29[-8]);
                *(unsigned long *)&v29[-8] = 0x9b36;
                v12 = sub_147c0(0,3,v68);
                v11 = "%s: can make relative symbolic links only in current directory";
                goto label_8394;
              }
            }
          }
          else {
            v64 = (void **)0x1a14b;
            *(unsigned long *)&v29[-8] = 0x7ec3;
            v37 = v29[-8];
            if (strcmp(".",v11,v37)) goto label_7ece;
          }
          *(unsigned long *)&v29[-8] = 0x7eea;
          free(v11,v29[-8]);
        }
        v37 = *(char *)((long)v42 + 0x16);
        *(unsigned long *)&v29[-8] = 0x7f0a;
        v5 = sub_c000(a0,v69,v67,v37,0xffffffff);
        v2 = '\0';
        if (1 <= v5) {
          *(unsigned long *)&v29[-8] = 0x9772;
          v12 = sub_14310(1,4,a0);
          *(unsigned long *)&v29[-8] = 0x9788;
          v13 = sub_14310(0,4,v68);
          *(unsigned long *)&v29[-8] = 0x979e;
          v11 = dcgettext(NULL,"cannot create symbolic link %s to %s",5,v29[-8]);
          *(unsigned long *)&v29[-8] = 0x97b2;
          error(0,v5,v11,v13,v12);
          goto label_8658;
        }
        goto label_7f17;
      }
      v66 = (char *)(CONCAT44(v66._4_4_,v70) & 0xffffffff0000f000);
      if ((v70 & 0xf000) == 0x4000) {
        *(unsigned long *)&v29[-8] = 0x832b;
        v2 = sub_72a0(a0,v68,v70,v40,v42);
        if (!v2) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
        v58 = (char *)(CONCAT44(v58._4_4_,v8) & 0xffffffff00000012);
        v39 = v63;
label_8356:
        for (; v39; v39 = (unsigned long *)*v39) {
          if (((void *)v39[1] == v86) && ((void *)v39[2] == v20)) {
            *(unsigned long *)&v29[-8] = 0x8385;
            v12 = sub_14440(4,a0);
            v11 = "cannot copy cyclic symbolic link %s";
            goto label_8394;
          }
        }
        v30 = &v29[-0x20];
        v31 = &v29[-0x20];
        v32 = &v29[-0x20];
        *(unsigned long *)&v29[-8] = *(unsigned long *)&v29[-8];
        v54 = (unsigned long *)((unsigned long)&v29[-0x11] & 0xfffffffffffffff0);
        *v54 = v63;
        v54[1] = v86;
        v54[2] = v20;
        if (((char)v72) || ((v99 & 0xf000) != 0x4000)) {
          *(unsigned long *)&v29[-0x28] = 0x8794;
          v5 = mkdirat(v69,v67,~(unsigned int)v58 & v8 & 0xfff);
          if (v5) { // branch-flip
            *(unsigned long *)&v29[-0x28] = 0x97e9;
            v12 = sub_14440(4,v68);
            v11 = "cannot create directory %s";
          }
          else {
            *(unsigned long *)&v29[-0x28] = 0x87ba;
            v5 = fstatat(v69,(char *)v67,&v21,0x100,v29[-0x28]);
            v8 = v99;
            if (!v5) {
              v52 = (void **)((unsigned long)v52 & 0xffffffffffffff00);
              if ((v99 & 0x1c0) != 0x1c0) {
                *(unsigned long *)&v29[-0x28] = 0x87fc;
                v5 = fchmodat(v69,v67,v99 | 0x1c0,0x100);
                v50 = v8;
                v52 = (void **)CONCAT71(v52._1_7_,1);
                if (v5) {
                  *(unsigned long *)&v29[-0x28] = 0xa4ec;
                  v12 = sub_14440(4,v68);
                  v11 = "setting permissions for %s";
                  v32 = &v29[-0x20];
                  goto label_8638;
                }
              }
              if (!*(char *)v64) {
                *(unsigned long *)&v29[-0x28] = 0x9f16;
                sub_bd30(v67,v97,v21);
                *(char *)v64 = '\x01';
              }
              if ((char)v42[0xf]) {
                if ((char)v42[6]) { // branch-flip
                  *(unsigned long *)&v29[-0x28] = 0x8844;
                  v9 = (unsigned long *)sub_14440(4,v68);
                  *(unsigned long *)&v29[-0x28] = 0x885a;
                  v11 = dcgettext(NULL,"created directory %s\n",5,v29[-0x28]);
                  *(unsigned long *)&v29[-0x28] = 0x886c;
                  __printf_chk(1,v11,v9);
                }
                else {
                  *(unsigned long *)&v29[-0x28] = 0xa199;
                  sub_6510(a0,v68,0);
                }
              }
              goto label_8870;
            }
            *(unsigned long *)&v29[-0x28] = 0xa034;
            v12 = sub_14440(4,v68);
            v11 = "cannot stat %s";
            v32 = &v29[-0x20];
          }
label_8638:
          *(unsigned long *)&v32[-8] = 0x863f;
          v11 = dcgettext(NULL,v11,5,v32[-8]);
          *(unsigned long *)&v32[-8] = 0x8647;
          v37 = v32[-8];
          v5 = *__errno_location(v37);
          *(unsigned long *)&v32[-8] = 0x8658;
          error(0,v5,v11,v12);
          goto label_8658;
        }
        if ((*(long *)&v42[10]) || (*(char *)((long)v42 + 0x33))) {
          *(unsigned long *)&v29[-0x28] = 0x9a8c;
          v2 = sub_7510(v68,0,v42);
          if ((!v2) && (v32 = &v29[-0x20], (char)v42[0xd])) goto label_8658;
        }
        v52 = (void **)((unsigned long)v52 & 0xffffffffffffff00);
        v58 = (char *)((unsigned long)v58 & 0xffffffff00000000);
label_8870:
        v43 = v60 != NULL & (unsigned char)v42[7];
        v38 = CONCAT71((undefined7)((unsigned long)v9 >> 8),v43);
        if ((!v43) || (*v60 == v20)) {
          v26 = *(long *)v42;
          v73 = *(unsigned long *)&v42[2];
          v74 = *(unsigned long *)&v42[4];
          v75 = *(unsigned long *)&v42[6];
          v82 = *(unsigned long *)&v42[0x14];
          v76 = *(unsigned long *)&v42[8];
          v77 = *(unsigned long *)&v42[10];
          v78 = *(unsigned long *)&v42[0xc];
          v79 = *(unsigned long *)&v42[0xe];
          v80 = *(unsigned long *)&v42[0x10];
          v81 = *(unsigned long *)&v42[0x12];
          *(unsigned long *)&v29[-0x28] = 0x88e0;
          v11 = (char *)sub_15300(a0,2);
          v16 = v61;
          v53 = v11;
          if (v11) { // branch-flip
            if (v42[1] == 3)
              v26 = CONCAT44(2,(unsigned int)v26);
            if (*v11) { // branch-flip
              v62 = (long)v67 - (long)v68;
              v57 = &v24;
              v60 = (void **)v25;
              v59 = (void **)&v25[1];
              v51 = v42;
              v63 = (unsigned long *)((unsigned long)v63._1_7_ << 8);
              v56 = &v26;
              v55 = (unsigned int *)CONCAT44(v55._4_4_,(int)v40);
              v66 = (char *)CONCAT71(v66._1_7_,1);
              v65 = a0;
              v61 = &v20;
              do {
                *(unsigned long *)&v29[-0x28] = 0x89dc;
                v36 = (void *)sub_ea10(v65,v11,0);
                *(unsigned long *)&v29[-0x28] = 0x89f0;
                v14 = (void *)sub_ea10(v68,v11,0);
                v43 = *(unsigned char *)v64;
                *(void ***)&v29[-0x28] = v57;
                *(void ***)&v29[-0x30] = v60;
                v25[1] = v43;
                *(void ***)&v29[-0x38] = v59;
                *(unsigned long *)&v29[-0x40] = 0;
                *(unsigned long **)&v29[-0x48] = v56;
                *(unsigned long **)&v29[-0x50] = v54;
                *(unsigned long *)&v29[-0x58] = 0x8a4d;
                v3 = sub_76f0(v36,v14,v69,(long)v14 + v62,(unsigned long)v55 & 0xffffffff,v61);
                v43 = v25[0];
                *(unsigned char *)v16 = *(unsigned char *)v16 | v25[0];
                v66 = (char *)CONCAT71(v66._1_7_,(unsigned char)v66 & v3);
                *(unsigned long *)&v29[-0x28] = 0x8a6a;
                free(v14,v29[-0x28]);
                *(unsigned long *)&v29[-0x28] = 0x8a72;
                free(v36,v29[-0x28]);
                if (v43) break;
                v63 = (unsigned long *)CONCAT71(v63._1_7_,(unsigned char)v63 | v25[1]);
                *(unsigned long *)&v29[-0x28] = 0x89bd;
                v37 = v29[-0x28];
                v11 = &v11[strlen(v11,v37) + 1];
              } while (*v11);
              v38 = (unsigned long)v66 & 0xff;
              v42 = v51;
              a0 = v65;
            }
            else {
              v63 = (unsigned long *)((unsigned long)v63._1_7_ << 8);
              v38 = 1;
            }
            *(unsigned long *)&v29[-0x28] = 0x8a9d;
            free(v53,v29[-0x28]);
            *(unsigned char *)v64 = (char)v63;
          }
          else {
            *(unsigned long *)&v29[-0x28] = 0xb9a7;
            v12 = sub_14440(4,a0);
            *(unsigned long *)&v29[-0x28] = 0xb9bd;
            v11 = dcgettext(NULL,"cannot access %s",5,v29[-0x28]);
            *(unsigned long *)&v29[-0x28] = 0xb9c5;
            v37 = v29[-0x28];
            v5 = *__errno_location(v37);
            v38 = 0;
            *(unsigned long *)&v29[-0x28] = 0xb9d9;
            error(0,v5,v11,v12);
          }
        }
        if ((char)v71) {
          v2 = '\0';
          v7 = 0;
          v43 = 0;
          v62 = CONCAT71(v62._1_7_,v52._0_1_);
          goto label_7f59;
        }
        v7 = 0;
        v62 = CONCAT71(v62._1_7_,v52._0_1_);
        goto label_7fd8;
      }
      *(unsigned long *)&v29[-8] = 0x7dfe;
      v2 = sub_72a0(a0,v68,v70,v40,v42);
      v58 = (char *)((unsigned long)v58 & 0xffffffff00000000);
      if (v2) goto label_7e7a;
      goto label_7cf0;
    }
    if (!*v42) {
      v16 = &v21;
      if (!v43) {
        v16 = &v23;
        *(unsigned long *)&v32[-8] = 0x91f1;
        v5 = fstatat(v69,(char *)v67,v16,0x100,v32[-8]);
        if (v5) goto label_789f;
      }
      if (((unsigned long)v16[3] & 0xf000) == 0xa000) {
        v12 = *(unsigned long *)&v42[0x12];
        *(unsigned long *)&v32[-8] = 0x7b26;
        v2 = sub_e820(v12,v67,v16);
        if (v2) {
          *(unsigned long *)&v32[-8] = 0x7b44;
          v12 = sub_14310(1,4,v68);
          *(unsigned long *)&v32[-8] = 0x7b56;
          v13 = sub_14310(0,4,a0);
          *(unsigned long *)&v32[-8] = 0x7b6c;
          v11 = dcgettext(NULL,"will not copy %s through just-created symlink %s",5,v32[-8]);
          *(unsigned long *)&v32[-8] = 0x7b80;
          error(0,0,v11,v13,v12);
          v38 = 0;
          return v38 & 0xffffffff;
        }
      }
      goto label_789f;
    }
    if ((char)v42[0xf]) goto label_8258;
    v29 = v32;
    if (!v8) goto label_7d98;
label_78ba:
    v29 = v32;
    if ((!(char)v42[0xe]) || ((v70 & 0xf000) != 0x4000)) {
      v2 = (char)v42[6];
      if (!v2) {
        if (!(char)v42[0xc]) goto label_7d98;
        if (!*(char *)((long)v42 + 0x17)) goto label_820e;
        v9 = NULL;
        goto label_8246;
      }
      if (v87 == 1) {
        *(unsigned long *)&v32[-8] = 0x9343;
        v9 = (unsigned long *)sub_bcd0(v86,v20);
label_8f3d:
        if (v9) {
          if ((v70 & 0xf000) == 0x4000) goto label_790d;
          *(unsigned long *)&v32[-0x10] = (unsigned long)v59 & 0xffffffff;
          *(unsigned long *)&v32[-0x18] = (unsigned long)(unsigned char)v42[0xf];
          *(unsigned long *)&v32[-0x20] = 1;
          *(unsigned long *)&v32[-0x28] = 0x8f90;
          v2 = sub_65d0(0,v69,v9,v68,v69,v67);
          if (v2) {
            v38 = 1;
            return v38 & 0xffffffff;
          }
          goto label_8f9c;
        }
        goto label_7970;
      }
      if (((char)v42[0xc]) && (!*(char *)((long)v42 + 0x17))) {
label_820e:
        if ((2 <= v87) || ((((char)v71 && (v42[1] == 3)) || (v9 = NULL, v42[1] == 4)))) {
          *(unsigned long *)&v32[-8] = 0x8f3a;
          v9 = (unsigned long *)sub_bd30(v67,v86,v20);
          goto label_8f3d;
        }
        goto label_823e;
      }
      v9 = NULL;
label_7a20:
      if (v8 == 0x11) {
        *(unsigned long *)&v32[-8] = 0x8dba;
        v5 = renameat(0xffffff9c,a0,v69,v67);
        if (v5) {
          *(unsigned long *)&v32[-8] = 0x8dc7;
          v37 = v32[-8];
          v8 = *__errno_location(v37);
          if (v8) goto label_7a2a;
        }
label_7d27:
        if ((char)v42[0xf]) {
          *(unsigned long *)&v32[-8] = 0x92a3;
          v11 = dcgettext(NULL,"renamed ",5,v32[-8]);
          *(unsigned long *)&v32[-8] = 0x92b2;
          __printf_chk(1,v11);
          *(unsigned long *)&v32[-8] = 0x92c8;
          sub_6510(a0,v68,v65);
        }
        if (*(long *)&v42[10]) {
          *(unsigned long *)&v32[-8] = 0x7d4d;
          sub_7510(v68,1,v42);
        }
        if (v66)
          *v66 = 1;
        if (((char)v71) && (!*(char *)((long)v42 + 0x3f))) {
          v12 = *(unsigned long *)&v42[0x12];
          *(unsigned long *)&v32[-8] = 0x921f;
          sub_e790(v12,v67,&v20);
          v38 = (unsigned long)v71 & 0xff;
          return v38 & 0xffffffff;
        }
        v38 = 1;
        return v38 & 0xffffffff;
      }
label_7a2a:
      if (v8 == 0x16) {
        *(unsigned long *)&v32[-8] = 0x93ae;
        v12 = sub_14310(1,4,dat_23568);
        *(unsigned long *)&v32[-8] = 0x93c4;
        v13 = sub_14310(0,4,dat_23570);
        *(unsigned long *)&v32[-8] = 0x93da;
        v11 = dcgettext(NULL,"cannot move %s to a subdirectory of itself, %s",5,v32[-8]);
        v38 = 1;
        *(unsigned long *)&v32[-8] = 0x93f4;
        error(0,0,v11,v13,v12);
        *(unsigned char *)v61 = 1;
        return v38 & 0xffffffff;
      }
      if (v8 != 0x12) {
        *(unsigned long *)&v32[-8] = 0x8e06;
        v12 = sub_14310(1,4,v68);
        *(unsigned long *)&v32[-8] = 0x8e18;
        v13 = sub_14310(0,4,a0);
        *(unsigned long *)&v32[-8] = 0x8e2e;
        v11 = dcgettext(NULL,"cannot move %s to %s",5,v32[-8]);
label_8e3a:
        *(unsigned long *)&v32[-8] = 0x8e43;
        error(0,v8,v11,v13,v12);
        *(unsigned long *)&v32[-8] = 0x8e56;
        sub_bc60(v86,v20);
        v38 = 0;
        return v38 & 0xffffffff;
      }
      v8 = v70 & 0xf000;
      *(unsigned long *)&v32[-8] = 0x7a6a;
      v5 = unlinkat(v69,(char *)v67,(unsigned int)(v8 == 0x4000) << 9,v32[-8]);
      if (v5) {
        *(unsigned long *)&v32[-8] = 0x7a73;
        v10 = (unsigned int *)__errno_location(v32[-8]);
        if (*v10 != 2) {
          *(unsigned long *)&v32[-8] = 0x9246;
          v12 = sub_14310(1,4,v68);
          *(unsigned long *)&v32[-8] = 0x9258;
          v13 = sub_14310(0,4,a0);
          *(unsigned long *)&v32[-8] = 0x926e;
          v11 = dcgettext(NULL,"inter-device move failed: %s to %s; unable to remove target",5,v32[-8]);
          v8 = *v10;
          goto label_8e3a;
        }
      }
      v43 = v8 != 0x4000 & (unsigned char)v42[0xf];
      v72 = (void **)CONCAT71(v72._1_7_,v43);
      if (v43) { // branch-flip
        v40 = 1;
        *(unsigned long *)&v32[-8] = 0x9369;
        v11 = dcgettext(NULL,"copied ",5,v32[-8]);
        *(unsigned long *)&v32[-8] = 0x9378;
        __printf_chk(1,v11);
        *(unsigned long *)&v32[-8] = 0x938e;
        sub_6510(a0,v68,v65);
      }
      else {
        v72 = (void **)CONCAT71(v72._1_7_,1);
        v40 = 1;
      }
      goto label_7da3;
    }
    if ((char)v71) { // branch-flip
      *(unsigned long *)&v32[-8] = 0x7905;
      v9 = (unsigned long *)sub_bd30(v67,v86,v20);
    }
    else {
      *(unsigned long *)&v32[-8] = 0x8f18;
      v9 = (unsigned long *)sub_bcd0(v86,v20);
    }
    if (!v9) {
label_7970:
      v2 = (char)v42[6];
label_823e:
      if (v2) goto label_7a20;
label_8246:
      v40 = (unsigned long)v72 & 0xff;
      goto label_7da3;
    }
label_790d:
    *(unsigned long *)&v32[-8] = 0x7923;
    v2 = sub_14e20(0xffffff9c,a0,v69,v9);
    if (v2) { // branch-flip
      *(unsigned long *)&v32[-8] = 0x9dc1;
      v12 = sub_14310(1,4,dat_23568);
      *(unsigned long *)&v32[-8] = 0x9dd7;
      v13 = sub_14310(0,4,dat_23570);
      *(unsigned long *)&v32[-8] = 0x9ded;
      v11 = dcgettext(NULL,"cannot copy a directory, %s, into itself, %s",5,v32[-8]);
      *(unsigned long *)&v32[-8] = 0x9e01;
      error(0,0,v11,v13,v12);
      *(unsigned char *)v61 = 1;
    }
    else {
      *(unsigned long *)&v32[-8] = 0x7942;
      v2 = sub_14e20(v69,v67,v69,v9);
      v16 = v68;
      if (v2) {
        *(unsigned long *)&v32[-8] = 0xa3e9;
        v12 = sub_14440(4,dat_23570);
        *(unsigned long *)&v32[-8] = 0xa3ff;
        v11 = dcgettext(NULL,"warning: source directory %s specified more than once",5,v32[-8]);
        *(unsigned long *)&v32[-8] = 42000;
        error(0,0,v11,v12);
        if ((char)v42[6]) goto label_91b0;
        v38 = 1;
        return v38 & 0xffffffff;
      }
      if ((v42[1] == 4) || ((v42[1] == 3 && ((char)v71)))) goto label_7970;
      *(unsigned long *)&v32[-8] = 0xa1b7;
      v36 = (void *)sub_6110(v68,v67,v9);
      *(unsigned long *)&v32[-8] = 0xa1cc;
      v12 = sub_14310(1,4,v36);
      *(unsigned long *)&v32[-8] = 0xa1de;
      v13 = sub_14310(0,4,v16);
      *(unsigned long *)&v32[-8] = 0xa1f4;
      v11 = dcgettext(NULL,"will not create hard link %s to directory %s",5,v32[-8]);
      *(unsigned long *)&v32[-8] = 0xa208;
      error(0,0,v11,v13,v12);
      *(unsigned long *)&v32[-8] = 0xa210;
      free(v36,v32[-8]);
    }
label_8f9c:
    if (*(char *)((long)v42 + 0x33)) {
      *(unsigned long *)&v32[-8] = 0x8fae;
      if (setfscreatecon(0)) goto label_8bf7;
    }
    goto label_866c;
  }
  v72 = (void **)CONCAT71(v72._1_7_,0 < (int)a4);
  if (v8) goto label_779d;
label_79ae:
  if (*(char *)((long)v42 + 0x3f)) {
    v8 = 0;
    goto label_7803;
  }
  v8 = 0;
  v35 = v67;
  v16 = v68;
  v5 = v69;
  v70 = a4;
label_77b2:
  v5 = fstatat(v5,(char *)v35,&v20,(unsigned int)(v42[1] == 2) << 8);
  a4 = v70;
  if (!v5) {
    v70 = v88;
    if (((v88 & 0xf000) != 0x4000) || ((char)v42[0xe])) goto label_7803;
    v12 = sub_14440(4,a0);
    v38 = 0;
    if (*(char *)((long)v42 + 0x19)) {
      v11 = "omitting directory %s";
      v11 = dcgettext(NULL,v11,5);
      error(0,0,v11,v12);
      return v38 & 0xffffffff;
    }
    v11 = dcgettext(NULL,"-r not specified; omitting directory %s",5);
    error(0,0,v11,v12);
    return v38 & 0xffffffff;
  }
label_8528:
  *(unsigned long *)&v31[-8] = 0x8532;
  v12 = sub_14440(4,v16);
  v11 = "cannot stat %s";
  v32 = v31;
label_8541:
  *(unsigned long *)&v32[-8] = 0x8548;
  v11 = dcgettext(NULL,v11,5,v32[-8]);
  *(unsigned long *)&v32[-8] = 0x8550;
  v37 = v32[-8];
  v5 = *__errno_location(v37);
  *(unsigned long *)&v32[-8] = 0x8561;
  error(0,v5,v11,v12);
  goto label_7cf0;
label_b3e3:
  *(unsigned long *)&v29[-8] = 0xb3e8;
  v37 = v29[-8];
  v9 = v56;
  v42 = v55;
  if (*__errno_location(v37) != 6) { // branch-flip
label_b521:
    v9 = v56;
    v42 = v55;
    *(unsigned long *)&v29[-8] = 0xb52e;
    v11 = "cannot lseek %s";
    v60 = (void **)sub_14440(4,a0);
label_b541:
    *(unsigned long *)&v29[-8] = 0xb548;
    v37 = v29[-8];
    v64 = (void **)dcgettext(NULL,v11,5,v37);
    *(unsigned long *)&v29[-8] = 0xb554;
    v37 = v29[-8];
    v5 = *__errno_location(v37);
    *(unsigned long *)&v29[-8] = 0xb56d;
    error(0,v5,(char *)v64,v60);
label_b56d:
    v2 = '\0';
  }
  else {
label_b3ff:
    v44 = (long)v52 < (long)v60;
    if (((long)v52 < (long)v60) || ((char)v54)) {
label_b5a2:
      if (v49 != 1) { // branch-flip
        v59 = (void **)CONCAT71(v59._1_7_,v44);
        *(unsigned long *)&v29[-8] = 0xb5c3;
        v5 = ftruncate(v40,v60);
        if (v5) {
label_b908:
          *(unsigned long *)&v29[-8] = 0xb919;
          v60 = (void **)sub_14440(4,v68);
          v11 = "failed to extend %s";
          goto label_b541;
        }
        if ((v49 == 3) && ((char)v59)) {
          *(unsigned long *)&v29[-8] = 0xb604;
          v5 = fallocate(v40,3,v52,(long)v60 - (long)v52);
          if (v5 <= -1) {
            *(unsigned long *)&v29[-8] = 0xb611;
            v16 = (void **)__errno_location(v29[-8]);
            v5 = *(int *)v16;
            if (v5 != 0x5f && v5 != 0x26) {
              v60 = (void **)CONCAT71(v60._1_7_,v5 == 0x5f || v5 == 0x26);
              v59 = v16;
              *(unsigned long *)&v29[-8] = 0xb64b;
              v64 = (void **)sub_14440(4,v68);
              *(unsigned long *)&v29[-8] = 0xb665;
              v11 = dcgettext(NULL,"error deallocating %s",5,v29[-8]);
              v5 = *(int *)v59;
              *(unsigned long *)&v29[-8] = 0xb682;
              error(0,v5,v11,v64);
              v2 = (char)v60;
              goto label_ac42;
            }
          }
        }
      }
      else {
        v17 = (long)v60 - (long)v52;
        *(unsigned long *)&v29[-8] = 0xb900;
        if (!sub_6170(v40,v17)) goto label_b908;
      }
    }
    v2 = (char)v64;
  }
label_ac42:
  if (v2 != '\x01') { // branch-flip
label_96e2:
    v43 = 0;
  }
  else {
    if (v25[0]) {
      *(unsigned long *)&v29[-8] = 0xb107;
      v5 = ftruncate(v40,v26);
      if (v5 <= -1) {
        *(unsigned long *)&v29[-8] = 0xb120;
        v63 = (unsigned long *)sub_14440(4,v68);
        v11 = "failed to extend %s";
label_b133:
        *(unsigned long *)&v29[-8] = 0xb13a;
        v37 = v29[-8];
        v64 = (void **)dcgettext(NULL,v11,5,v37);
        *(unsigned long *)&v29[-8] = 0xb146;
        v37 = v29[-8];
        v5 = *__errno_location(v37);
        *(unsigned long *)&v29[-8] = 0xb15f;
        error(0,v5,(char *)v64,v63);
        goto label_96e2;
      }
    }
label_ac58:
    v64 = (void **)CONCAT44(v64._4_4_,v51._0_4_);
    if (*(char *)((long)v42 + 0x1f)) {
label_a676:
      v27 = v93;
      v83 = v94;
      v84 = v95;
      v85 = v96;
      *(unsigned long *)&v29[-8] = 0xa6cc;
      v5 = sub_e6a0(v40,v69,v67,&v27,0);
      if (v5) {
        *(unsigned long *)&v29[-8] = 0xb175;
        v59 = (void **)sub_14440(4,v68);
        *(unsigned long *)&v29[-8] = 0xb18f;
        v37 = v29[-8];
        v60 = (void **)dcgettext(NULL,"preserving times for %s",5,v37);
        *(unsigned long *)&v29[-8] = 0xb19b;
        v37 = v29[-8];
        v5 = *__errno_location(v37);
        *(unsigned long *)&v29[-8] = 0xb1b4;
        error(0,v5,(char *)v60,v59);
        if (*(char *)((long)v42 + 0x32)) goto label_96e2;
      }
    }
    if (*(char *)((long)v42 + 0x1d)) {
      if ((v89 != v105._4_4_) || (v90 != (unsigned int)v106)) {
        *(void ***)&v29[-8] = &v22;
        *(unsigned long *)&v29[-0x10] = (unsigned long)v72 & 0xff;
        *(unsigned long *)&v29[-0x18] = (unsigned long)v90;
        *(unsigned long *)&v29[-0x20] = (unsigned long)v89;
        *(unsigned long *)&v29[-0x28] = 0xa72f;
        v5 = sub_7080(v42,v68,v69,v67,v40,v88);
        if (v5 == -1) goto label_96e2;
        if (!v5)
          v61 = (void **)((unsigned long)v61 & 0xfffffffffffff1ff);
      }
    }
label_a74a:
    if ((char)v63) { // branch-flip
      *(unsigned long *)&v29[-8] = 0xaec7;
      v2 = sub_6220(a0,v38,v68,v40,v42);
      if (v2) goto label_a757;
      v43 = *(unsigned char *)((long)v42 + 0x36) ^ 1;
    }
    else {
label_a757:
      v43 = 1;
    }
    if (*(unsigned long *)&v42[6] & 0xff0000000000ff) { // branch-flip
      v64 = (void **)CONCAT71(v64._1_7_,v43);
      *(unsigned long *)&v29[-8] = 0xae90;
      v5 = sub_c830(a0,v38,v68,v40,(unsigned long)v61 & 0xffffffff);
      v43 = (unsigned char)v64;
      if ((v5) && (*(char *)((long)v42 + 0x32)))
        v43 = 0;
    }
    else if (*(char *)((long)v42 + 0x39)) { // branch-flip
      v6 = v42[4];
      v64 = (void **)CONCAT71(v64._1_7_,v43);
      *(unsigned long *)&v29[-8] = 0xb041;
      v5 = sub_c8e0(v68,v40,v6);
      v43 = (unsigned char)v64;
      if (v5)
        v43 = 0;
    }
    else {
      v3 = (unsigned char)v72 & (unsigned char)v42[8];
      v63 = (unsigned long *)CONCAT71(v63._1_7_,v3);
      if (v3) { // branch-flip
        if (dat_23010 == 0xffffffff) {
          v72 = (void **)CONCAT71(v72._1_7_,v43);
          *(unsigned long *)&v29[-8] = 0xb817;
          dat_23010 = umask(0,v29[-8]);
          *(unsigned long *)&v29[-8] = 0xb824;
          umask(dat_23010,v29[-8]);
          v43 = (unsigned char)v72;
        }
        v64 = (void **)CONCAT71(v64._1_7_,v43);
        *(unsigned long *)&v29[-8] = 0xb7a5;
        v5 = sub_c8e0(v68,v40,~dat_23010 & 0x1b6);
        v43 = (unsigned char)v64;
        v72 = (void **)CONCAT71(v72._1_7_,v63._0_1_);
        if (v5)
          v43 = 0;
      }
      else if ((int)v64 || v46) {
        if (dat_23010 != 0xffffffff) { // branch-flip
          v6 = dat_23010;
          if (~dat_23010 & v46 || (int)v64) {
label_a7ca:
            v64 = (void **)CONCAT71(v64._1_7_,v43);
            *(unsigned long *)&v29[-8] = 0xa7e2;
            v5 = fchmod(v8,(unsigned int)v53 & ~v6,v29[-8]);
            v43 = (unsigned char)v64;
            if (v5) {
              v61 = (void **)CONCAT71(v61._1_7_,(unsigned char)v64);
              *(unsigned long *)&v29[-8] = 0xa80a;
              v63 = (unsigned long *)sub_14440(4,v68);
              *(unsigned long *)&v29[-8] = 0xa824;
              v37 = v29[-8];
              v64 = (void **)dcgettext(NULL,"preserving permissions for %s",5,v37);
              *(unsigned long *)&v29[-8] = 0xa830;
              v37 = v29[-8];
              v5 = *__errno_location(v37);
              *(unsigned long *)&v29[-8] = 0xa849;
              error(0,v5,(char *)v64,v63);
              v43 = (unsigned char)v61;
              if (*(char *)((long)v42 + 0x32))
                v43 = 0;
            }
          }
        }
        else {
          v61 = (void **)CONCAT71(v61._1_7_,v43);
          *(unsigned long *)&v29[-8] = 0xb9ec;
          dat_23010 = umask(0,v29[-8]);
          v63 = (unsigned long *)CONCAT44(v63._4_4_,dat_23010);
          *(unsigned long *)&v29[-8] = 0xb9ff;
          umask(dat_23010,v29[-8]);
          v43 = (unsigned char)v61;
          if (~(unsigned int)v63 & v46 || (int)v64) {
            v6 = dat_23010;
            if ((unsigned int)v63 == 0xffffffff) {
              v64 = (void **)CONCAT71(v64._1_7_,(unsigned char)v61);
              *(unsigned long *)&v29[-8] = 0xba40;
              dat_23010 = umask(0,v29[-8]);
              *(unsigned long *)&v29[-8] = 0xba4d;
              umask(dat_23010,v29[-8]);
              v43 = (unsigned char)v64;
              v6 = dat_23010;
            }
            goto label_a7ca;
          }
        }
      }
    }
  }
label_96e5:
  v64 = (void **)CONCAT71(v64._1_7_,v43);
  *(unsigned long *)&v29[-8] = 0x96f3;
  v37 = v29[-8];
  if (0 <= close(v8,v37)) { // branch-flip
    v37 = SUB81(v64,0);
    v36 = v24;
  }
  else {
    *(unsigned long *)&v29[-8] = 0xadf3;
    v64 = (void **)sub_14440(4,v68);
    *(unsigned long *)&v29[-8] = 0xae0d;
    v11 = dcgettext(NULL,"failed to close %s",5,v29[-8]);
    *(unsigned long *)&v29[-8] = 0xae15;
    v37 = v29[-8];
    v5 = *__errno_location(v37);
    *(unsigned long *)&v29[-8] = 0xae2a;
    error(0,v5,v11,v64);
    v37 = 0;
    v36 = v24;
  }
label_9710:
  v64 = (void **)CONCAT71(v64._1_7_,v37);
  *(unsigned long *)&v29[-8] = 0x971f;
  v37 = v29[-8];
  if (0 <= close(v7,v37)) { // branch-flip
    v7 = 0;
    *(unsigned long *)&v29[-8] = 0x9741;
    free(v36,v29[-8]);
    v2 = (char)v64;
    v4 = (char)v64;
label_8eb2:
    if (!v4) goto label_8658;
label_7f17:
    v43 = (int)v66 != 0x4000;
    if (!(char)v72) {
      v3 = ((unsigned char)v42[5] ^ 1) & v43;
      if (v3) {
        if ((*(long *)&v42[10]) || (*(char *)((long)v42 + 0x33))) {
          v66 = (char *)CONCAT71(v66._1_7_,v2);
          v64 = (void **)CONCAT71(v64._1_7_,v3);
          *(unsigned long *)&v29[-8] = 0x8fe9;
          v43 = sub_7510(v68,0,v42);
          v2 = (char)v66;
          if (!v43) {
            v72 = (void **)CONCAT71(v72._1_7_,(char)v42[0xd]);
            if ((char)v42[0xd]) goto label_8658;
            v43 = (unsigned char)v64;
          }
        }
        else {
          v43 = v3;
        }
      }
    }
    v38 = 1;
    v31 = v29;
    if ((char)v71) {
      v62 &= 0xffffffffffffff00;
      v38 = (unsigned long)v71 & 0xffffffff;
      v30 = v29;
label_7f59:
      v31 = v30;
      if (*(long *)&v42[0x12]) {
        v66 = (char *)CONCAT71(v66._1_7_,v2);
        v71 = &v23;
        *(unsigned long *)&v30[-8] = 0x7f8c;
        v5 = fstatat(v69,(char *)v67,&v23,0x100,v30[-8]);
        v16 = v71;
        v2 = (char)v66;
        if (!v5) {
          v12 = *(unsigned long *)&v42[0x12];
          v71 = (void **)CONCAT71(v71._1_7_,(char)v66);
          *(unsigned long *)&v30[-8] = 0x7fb6;
          sub_e790(v12,v67,v16);
          v2 = (char)v71;
        }
      }
    }
    if (((*(char *)((long)v42 + 0x17)) && (v43)) || (v2)) {
      return v38 & 0xffffffff;
    }
label_7fd8:
    if (*(char *)((long)v42 + 0x1f)) {
      v23 = v93;
      v120 = v94;
      v121 = v95;
      v122 = v96;
      *(unsigned long *)&v31[-8] = 0x803b;
      v5 = utimensat(v69,v67,&v23,(v7 & 0xff) << 8);
      if (v5) {
        *(unsigned long *)&v31[-8] = 0x8050;
        v71 = (void **)sub_14440(4,v68);
        *(unsigned long *)&v31[-8] = 0x806a;
        v11 = dcgettext(NULL,"preserving times for %s",5,v31[-8]);
        *(unsigned long *)&v31[-8] = 0x8072;
        v37 = v31[-8];
        v5 = *__errno_location(v37);
        *(unsigned long *)&v31[-8] = 0x8087;
        error(0,v5,v11,v71);
        if (*(char *)((long)v42 + 0x32)) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
      }
    }
    if ((char)v7) { // branch-flip
      if (!*(char *)((long)v42 + 0x35)) {
        return v38 & 0xffffffff;
      }
      *(unsigned long *)&v31[-8] = 0x8ef7;
      if (sub_6220(a0,0xffffffff,v68,0xffffffff,v42)) {
        return v38 & 0xffffffff;
      }
      v2 = *(char *)((long)v42 + 0x36);
    }
    else {
      if (*(char *)((long)v42 + 0x1d)) {
        if ((((char)v72) || (v100 != v89)) || (v101 != v90)) {
          *(void ***)&v31[-8] = &v21;
          *(unsigned long *)&v31[-0x10] = (unsigned long)v72 & 0xff;
          *(unsigned long *)&v31[-0x18] = (unsigned long)v90;
          *(unsigned long *)&v31[-0x20] = (unsigned long)v89;
          *(unsigned long *)&v31[-0x28] = 0x8104;
          v5 = sub_7080(v42,v68,v69,v67,0xffffffff,v88);
          if (v5 == -1) {
            v38 = 0;
            return v38 & 0xffffffff;
          }
          if (!v5)
            v70 &= 0xfffff1ff;
        }
      }
      if (*(char *)((long)v42 + 0x35)) {
        *(unsigned long *)&v31[-8] = 0xa094;
        v2 = sub_6220(a0,0xffffffff,v68,0xffffffff,v42);
        if ((!v2) && (*(char *)((long)v42 + 0x36))) {
          v38 = 0;
          return v38 & 0xffffffff;
        }
      }
      if (!(*(unsigned long *)&v42[6] & 0xff0000000000ff)) {
        if (*(char *)((long)v42 + 0x39)) // branch-flip
          v8 = v42[4];
        else {
          if (!((unsigned char)v72 & (unsigned char)v42[8])) {
            if ((int)v58) { // branch-flip
              v8 = dat_23010;
              if (dat_23010 == 0xffffffff) {
                *(unsigned long *)&v31[-8] = 0xad45;
                v8 = umask(0,v31[-8]);
                *(unsigned long *)&v31[-8] = 0xad54;
                dat_23010 = v8;
                umask(v8,v31[-8]);
              }
              v8 = (unsigned int)v58 & ~v8;
              v58 = (char *)CONCAT44(v58._4_4_,v8);
              if (!v8) goto label_8169;
              if ((char)v62 != '\x01') {
                if ((char)v72) {
                  *(unsigned long *)&v31[-8] = 0x8513;
                  v5 = fstatat(v69,(char *)v67,&v21,0x100,v31[-8]);
                  if (v5) {
                    v16 = v68;
                    goto label_8528;
                  }
                }
                v50 = v99;
                if (!((unsigned int)v58 & ~v99)) {
                  return v38 & 0xffffffff;
                }
              }
            }
            else {
label_8169:
              if (!(char)v62) {
                return v38 & 0xffffffff;
              }
            }
            *(unsigned long *)&v31[-8] = 0x8199;
            v5 = fchmodat(v69,v67,v50 | (unsigned int)v58,0x100);
            if (!v5) {
              return v38 & 0xffffffff;
            }
            *(unsigned long *)&v31[-8] = 0x81b2;
            v12 = sub_14440(4,v68);
            *(unsigned long *)&v31[-8] = 0x81c8;
            v11 = dcgettext(NULL,"preserving permissions for %s",5,v31[-8]);
            *(unsigned long *)&v31[-8] = 0x81d0;
            v37 = v31[-8];
            v5 = *__errno_location(v37);
            *(unsigned long *)&v31[-8] = 0x81e1;
            error(0,v5,v11,v12);
            goto label_81e1;
          }
          v8 = 0x1ff;
          if ((v70 & 0x7000) != 0x4000)
            v8 = 0x1b6;
          v7 = dat_23010;
          if (dat_23010 == 0xffffffff) {
            *(unsigned long *)&v31[-8] = 0xad2a;
            v7 = umask(0,v31[-8]);
            *(unsigned long *)&v31[-8] = 0xad39;
            dat_23010 = v7;
            umask(v7,v31[-8]);
          }
          v8 = ~v7 & v8;
        }
        *(unsigned long *)&v31[-8] = 0x9eef;
        if (!sub_c8e0(v68,0xffffffff,v8)) {
          return v38 & 0xffffffff;
        }
        v38 = 0;
        return v38 & 0xffffffff;
      }
      *(unsigned long *)&v31[-8] = 0x9b70;
      if (!sub_c830(a0,0xffffffff,v68,0xffffffff,v70)) {
        return v38 & 0xffffffff;
      }
label_81e1:
      v2 = *(char *)((long)v42 + 0x32);
    }
    if (!v2) {
      return v38 & 0xffffffff;
    }
  }
  else {
    *(unsigned long *)&v29[-8] = 0xa45f;
    v12 = sub_14440(4,a0);
    *(unsigned long *)&v29[-8] = 0xa475;
    v11 = dcgettext(NULL,"failed to close %s",5,v29[-8]);
    *(unsigned long *)&v29[-8] = 0xa47d;
    v37 = v29[-8];
    v5 = *__errno_location(v37);
    *(unsigned long *)&v29[-8] = 0xa48e;
    error(0,v5,v11,v12);
    *(unsigned long *)&v29[-8] = 0xa496;
    free(v36,v29[-8]);
label_8658:
    if (*(char *)((long)v42 + 0x33)) {
      *(unsigned long *)&v32[-8] = 0x92d7;
      if (setfscreatecon(0)) {
label_8bf7:
        *(unsigned long *)&v32[-8] = 0x8bfc; // no-return
        sub_6770();
      }
    }
    if (!v9) {
      *(unsigned long *)&v32[-8] = 0x9323;
      sub_bc60(v86,v20);
    }
label_866c:
    if (v65) {
      *(unsigned long *)&v32[-8] = 0x869d;
      v5 = renameat(v69,(long)v65 + ((long)v67 - (long)v68));
      if (v5) {
        *(unsigned long *)&v32[-8] = 0x92f9;
        v12 = sub_14440(4,v68);
        v11 = "cannot un-backup %s";
        goto label_8541;
      }
      if ((char)v42[0xf]) {
        *(unsigned long *)&v32[-8] = 0x86c6;
        v12 = sub_14310(1,4,v68);
        *(unsigned long *)&v32[-8] = 0x86dc;
        v13 = sub_14310(0,4,v65);
        *(unsigned long *)&v32[-8] = 0x86f2;
        v11 = dcgettext(NULL,"%s -> %s (unbackup)\n",5,v32[-8]);
        *(unsigned long *)&v32[-8] = 0x8707;
        __printf_chk(1,v11,v13,v12);
      }
    }
  }
  v38 = 0;
  return v38 & 0xffffffff;
}


// Function: sub_ba80 @ 0xba80
void sub_ba80(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long a3,unsigned long a4,unsigned int *a5,unsigned long a6,unsigned long a7)
{
  char v1; // stack - 0x11
  
  if (4 <= *a5)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (3 <= a5[3] - 1)
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (2 < a5[0x11])
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((*(char *)((long)a5 + 0x17)) && (*(char *)((long)a5 + 0x3a)))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a5[3] != 2) && (a5[0x11] == 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  v1 = 0;
  dat_23568 = a1;
  dat_23570 = a0;
  sub_76f0(a0,a1,a2,a3,a4,0,0,a5,1,&v1,a6,a7);
}


// Function: sub_bc10 @ 0xbc10
unsigned long sub_bc10(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_bc20 @ 0xbc20
unsigned long sub_bc20(long *a0,long *a1)
{
  if (*a0 != *a1)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1[1] >> 8),a0[1] == a1[1]);
}


// Function: sub_bc60 @ 0xbc60
void sub_bc60(unsigned long a0,unsigned long a1) // return-dupe
{
  void *v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = (void *)sub_12270(dat_23578,&v2);
  if (!v1)
    return;
  free(*(void **)((long)v1 + 0x10));
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
unsigned long sub_bd30(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  unsigned long *v3; // rax
  
  v1 = (unsigned long *)sub_16c50(0x18);
  v2 = sub_17130(a0);
  *v1 = a1;
  v1[2] = v2;
  v1[1] = a2;
  v3 = (unsigned long *)sub_12210(dat_23578,v1);
  if (!v3)
    sub_17150(); // no-return
  if (v1 == v3)
    return 0;
  free((void *)v1[2]);
  free(v1);
  return v3[2];
}


// Function: sub_bdb0 @ 0xbdb0
void sub_bdb0(void)
{
  dat_23578 = sub_11bb0(0x67,0,sub_bc10,sub_bc20,sub_bc40);
  if (dat_23578)
    return;
  sub_17150(); // no-return
}


// Function: sub_bdf0 @ 0xbdf0
void sub_bdf0(unsigned long a0,unsigned int *a1)
{
  linkat(*a1,*(unsigned long *)&a1[2],a1[4],a0,a1[5]); // tail-call
}


// Function: sub_be10 @ 0xbe10
void * sub_be10(void *a0,void *a1)
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
void sub_be70(unsigned long a0,unsigned long *a1)
{
  symlinkat(*a1,*(unsigned int *)&a1[1],a0); // tail-call
}


// Function: sub_be90 @ 0xbe90
int sub_be90(unsigned long a0,unsigned long a1,int a2,unsigned long a3,unsigned int a4,char a5,int a6)
{
  char *v1;
  int v2; // eax
  int *v3; // rax
  char v4 [264];
  unsigned int v5 [2]; // stack - 0x168
  int v6;
  unsigned long v7; // stack - 0x160
  int v8; // stack - 0x158
  unsigned int v9; // stack - 0x154
  
  v2 = a6;
  if (a6 < 0) {
    v2 = linkat(a0,a1,a2);
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
      v5[0] = (unsigned int)a0;
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
int sub_c000(unsigned long a0,int a1,unsigned long a2,char a3,int a4)
{
  char *v1; // rax
  unsigned long v2; // stack - 0x158
  char v3 [264];
  int v4; // stack - 0x150
  
  if (a4 < 0) {
    if (!symlinkat())
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
  long v10; // stack - 0xe0
  unsigned int v11; // stack - 0xc0
  int v2;
  int v3;
  int *v4; // rax
  long v5; // rax
  long v6; // rax
  long v7; // rax
  unsigned long v8; // stack - 0xe8
  char v9 [24];
  
  v8 = 0;
  v10 = 0;
  if (!a0) {
    if ((int)getfscreatecon(&v10) < 0)
      return -1;
    v4 = __errno_location();
    if (v10) {
      v2 = lsetfilecon(a1,v10);
      v1 = *v4;
      freecon(v10);
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
      if (0 <= lstat(a1,v9)) goto label_c1bd;
      v2 = *v4;
      v6 = 0;
      v5 = 0;
    }
label_c28b:
    v3 = v1;
    goto label_c28e;
  }
  if (0 <= fstat(v1,v9)) { // branch-flip
label_c1bd:
    if (0 <= (int)selabel_lookup(a0,&v8,a1,v11)) { // branch-flip
      v5 = context_new(v8);
      if (v5) { // branch-flip
        if (v1 != -1) {
          if ((int)sub_15610(v1) < 0) {
            v6 = 0;
            v3 = -1;
            goto label_c3ab;
          }
          v6 = context_new(v10);
          if (v6) goto label_c221;
          v2 = *v4;
          v3 = -1;
          goto label_c24f;
        }
        if ((int)sub_15590(a1,&v10) < 0) {
          v2 = *v4;
          v6 = 0;
          goto label_c28b;
        }
        v6 = context_new(v10);
        if (!v6) {
          v2 = *v4;
          v3 = -1;
          goto label_c28e;
        }
label_c221:
        v7 = context_type_get(v5);
        if (((v7) && (!context_type_set(v6,v7))) && (v7 = context_str(v6), v7)) {
          if (v1 == -1) {
            v1 = lsetfilecon(a1,v7);
            v2 = *v4;
            goto label_c28b;
          }
          v3 = fsetfilecon(v1);
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
  freecon(v8);
  freecon(v10);
  *v4 = v2;
  return v3;
}


// Function: sub_c450 @ 0xc450
unsigned int sub_c450(unsigned long a0,char *a1,unsigned int a2)
{
  short v1; // ax
  unsigned long v10; // stack - 0x50
  unsigned long v11; // stack - 0x48
  unsigned long v12; // stack - 0x58
  char *v13;
  int v14;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  void *v5; // rax
  long v6;
  long v7; // rax
  long v8; // rax
  unsigned long v9; // stack - 0x60
  
  v9 = 0;
  v12 = 0;
  v4 = __errno_location();
  if (*a1 != '/') { // branch-flip
    a1 = (char *)sub_da80(a1,2);
    if (!a1) {
      v14 = *v4;
      v13 = NULL;
      v7 = 0;
      v3 = 0xffffffff;
      v6 = 0;
      goto label_c5df;
    }
    v13 = a1;
  }
  else {
    v13 = NULL;
  }
  if (0 <= (int)selabel_lookup(a0,&v9,a1,a2)) { // branch-flip
    v10 = 0;
    v11 = 0;
    v5 = (void *)sub_e480(a1);
    if ((((int)getcon(&v10) <= -1) || ((int)sub_15510(v5,&v11) <= -1)) || (v1 = mode_to_security_class(a2), !v1)) {
      v14 = *v4;
      v7 = 0;
      v6 = 0;
      free(v5);
      v3 = 0xffffffff;
      freecon(v10);
      freecon(v11);
      *v4 = v14;
      goto label_c5df;
    }
    v2 = security_compute_create(v10,v11,v1,&v12);
    v14 = *v4;
    free(v5);
    freecon(v10);
    freecon(v11);
    *v4 = v14;
    if (0 <= v2) {
      v6 = context_new(v9);
      if (v6) { // branch-flip
        v7 = context_new(v12);
        if (((v7) && (v8 = context_type_get(v6), v8)) && ((!context_type_set(v7,v8) && (v8 = context_str(v7), v8)))) {
          v3 = setfscreatecon(v8);
          v14 = *v4;
        }
        else {
          v14 = *v4;
          v3 = 0xffffffff;
        }
      }
      else {
        v14 = *v4;
        v7 = 0;
        v3 = 0xffffffff;
      }
      goto label_c5df;
    }
  }
  else {
    v14 = *v4;
    if (v14 == 2) {
      *v4 = 0x3d;
      v14 = 0x3d;
      v7 = 0;
      v6 = 0;
      v3 = 0xffffffff;
      goto label_c5df;
    }
  }
  v7 = 0;
  v6 = 0;
  v3 = 0xffffffff;
label_c5df:
  context_free(v6);
  context_free(v7);
  freecon(v9);
  freecon(v12);
  free(v13);
  *v4 = v14;
  return v3;
}


// Function: sub_c6f0 @ 0xc6f0
unsigned long sub_c6f0(unsigned long a0,char *a1,char a2)
{
  unsigned long v1;
  int v10;
  unsigned long v11; // stack - 0x40
  int v2; // eax
  int *v3;
  long v4; // rax
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
    v4 = sub_17190(&v7,0x10,0);
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
        v1 = *(unsigned long *)(v4 + 0x20);
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
void sub_e480(void)
{
  if (sub_e4e0())
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
unsigned long sub_11270(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1c620)
    return 1;
  v1 = v2[2];
  if ((((dat_1c634 < v1) && (v1 < dat_1c638)) && (dat_1c63c < v2[3])) && (dat_1c650 <= *v2)) {
    v3 = *v2 + dat_1c634;
    if (((v3 < v2[1]) && (v2[1] <= dat_1c640)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1c620;
  return 0;
}


// Function: sub_11300 @ 0x11300
unsigned long sub_11300(long *a0,long a1,unsigned long *a2,char a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + *a0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*(void *)a0[7])(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*(void *)a0[7])(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0[9];
      a0[9] = (long)v1;
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
      v1[1] = a0[9];
      a0[9] = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}


// Function: sub_11550 @ 0x11550
unsigned long sub_11550(long *a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = (long *)*a1;
  if ((long *)a1[1] <= v6)
    return 1;
label_11586:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0[2];
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*(void *)a0[6])(v8,v4);
          v4 = a0[2];
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + *a0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0[9];
            a0[3] = a0[3] + 1;
            *v7 = 0;
            v7[1] = v8;
            a0[9] = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*(void *)a0[6])(v8,a0[2]);
        if ((unsigned long)a0[2] <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + *a0);
        if (*v7) { // branch-flip
          v5 = (long *)a0[9];
          if (v5) // branch-flip
            a0[9] = v5[1];
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
          a0[3] = a0[3] + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1[3] = a1[3] + -1;
        if ((long *)a1[1] <= v6)
          return 1;
        goto label_11586;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}


// Function: sub_117c0 @ 0x117c0
void sub_117c0(unsigned long *a0,void *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_11920 @ 0x11920
long sub_11920(long *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + *a0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = (void *)a0[7];
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
long sub_119e0(long *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + *a0);
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
    if ((long *)a0[1] <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}


// Function: sub_11ac0 @ 0x11ac0
long sub_11ac0(unsigned long *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = (long *)*a0;
  if ((long *)a0[1] <= v4) // branch-flip
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
    } while (v4 < (long *)a0[1]);
  }
  return v3;
}


// Function: sub_11bb0 @ 0x11bb0
unsigned long * sub_11bb0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
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
    a1 = 0x1c620;
  v1[5] = a1;
  if (sub_11270(v1)) {
    v2 = sub_11410(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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
void sub_11cc0(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = (long *)*a0;
  if ((long *)*a0 < (long *)a0[1]) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if ((long *)a0[1] <= v5) goto label_11d5b;
      }
      v3 = (void *)a0[8];
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = (void *)a0[8];
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0[9];
          *v4 = 0;
          v4[1] = v2;
          a0[9] = v4;
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
    } while (v6 < (long *)a0[1]);
  }
label_11d5b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_11d70 @ 0x11d70
void sub_11d70(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_11e1c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11ddf;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0[8])(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = (long *)a0[1];
      v4 = &v4[2];
    } while (v4 < v3);
label_11ddf:
    v4 = (long *)*a0;
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
    } while (v4 < (long *)a0[1]);
  }
label_11e1c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_11e60 @ 0x11e60
unsigned int sub_11e60(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_11410(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
  if (!v4)
    return 0;
  if (a0[2] == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0[5];
  v6 = 0;
  v8 = a0[6];
  v9 = a0[7];
  v10 = a0[8];
  v11 = a0[9];
  v1 = sub_11550(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_11550(a0,&v2,1)) && (sub_11550(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free((void *)*a0);
  *a0 = v2;
  a0[1] = v3;
  a0[2] = v4;
  a0[3] = v5;
  a0[9] = v11;
  return v1;
}


// Function: sub_11fe0 @ 0x11fe0
long * sub_11fe0(long a0,long a1,long *a2) // return-dupe, ternary
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
  v4 = *(unsigned long *)(a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)(a0 + 0x10);
    if ((long)v2 > -1) goto label_1206f;
label_120f6:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_120f6;
label_1206f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_11270(a0);
    v1 = *(long *)(a0 + 0x28);
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
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)(a0 + 0x48);
  if (v2) // branch-flip
    *(long *)(a0 + 0x48) = v2[1];
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}


// Function: sub_12210 @ 0x12210
unsigned long sub_12210(unsigned long a0,unsigned long a1) // early-return
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
long sub_12270(long a0,unsigned long a1) // ternary x2
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_11270(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1c648 <= v8) ? (long)(v8 - dat_1c648) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11e60(a0,v6)) {
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

