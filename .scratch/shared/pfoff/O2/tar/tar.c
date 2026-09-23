// Function: main @ 0xa9f0
int4 main(int4 argc,char **argv,char **envp) // ternary
{
  int8 *v1;
  int4 v10; // eax
  char *v11;
  int8 v12; // rax
  char *v13;
  unsigned long v14;
  uint4 v15;
  char **v16;
  char **v17;
  int4 v18 [2]; // stack - 0x1a8
  int4 *v19; // stack - 0x168
  char **v2;
  int4 v20 [2]; // stack - 0x188
  int8 v21; // stack - 0x138
  char v22 [4];
  char v23; // stack - 0x43
  char **v24;
  unsigned long *v25;
  char **v26;
  uint8 v27; // rdx
  int8 v28;
  char **v29; // stack - 0x1c0
  char **v3;
  unsigned long v30; // stack - 0x1a0
  unsigned long v31; // stack - 0x198
  unsigned long v32; // stack - 0x190
  char *v33; // stack - 0x180
  unsigned long v34; // stack - 0x178
  unsigned long v35; // stack - 0x170
  unsigned long *v36; // stack - 0x160
  unsigned short v37; // stack - 0x158
  char v38; // stack - 0x156
  char *v39; // stack - 0x150
  unsigned long *v4;
  char *v40; // stack - 0x148
  int8 *v41; // stack - 0x130
  unsigned long v42; // stack - 0x128
  char v43; // stack - 0x42
  char v44; // stack - 0x41
  int8 v45; // stack - 0x40
  unsigned short v5;
  int8 *v6;
  uint4 v7;
  int4 *v8;
  char v9;
  
  v27 = (uint8)argc;
  v45 = *(int8 *)(v28 + 0x28);
  sub_c740();
  sub_4af40(*argv);
  setlocale(6,"");
  bindtextdomain("tar","/usr/local/share/locale");
  textdomain("tar");
  dat_81e08 = 2;
  dat_82d50 = sub_10c20;
  dat_82d58 = 0;
  sub_4c760(NULL,7);
  sub_40160(dcgettext(NULL,"stdout",5));
  if (sub_4e3a0()) {
    if (dat_82d50)
      (*dat_82d50)();
    v11 = dcgettext(NULL,"failed to assert availability of the standard file descriptors",5);
    v13 = "%s";
    error(0,0,v13,v11); // return-dupe
    sub_16fd0(); // no-return
  }
  dat_82c18 = 10;
  dat_82838 = (int8 *)sub_4ecc0(0x50);
  dat_82830 = 0;
  signal(0x11,0);
  v19 = v18;
  v37 = 0;
  v18[0] = 1;
  v30 = 0;
  v31 = 0;
  v32 = 0;
  v36 = NULL;
  v38 = '\0';
  v39 = NULL;
  v40 = NULL;
  sub_3f7c0("tar",0x7fb00);
  v39 = getenv("SIMPLE_BACKUP_SUFFIX");
  v11 = getenv("POSIXLY_CORRECT");
  dat_82ba8 = 0;
  dat_82ba4 = 0;
  dat_82a38 = v11 != NULL;
  dat_82ba0 = 0x14;
  dat_82b98 = 0x2800;
  dat_82b60 = sub_40920();
  dat_827e8 = 1;
  dat_82a7c = 0;
  dat_82b08 = 0xffffffffffffffff;
  dat_82b00 = 0x8000000000000000;
  dat_82ae0 = 0x8000000000000000;
  dat_82ae8 = -1;
  dat_82ad4 = 8;
  dat_82a84 = 1;
  dat_82a80 = 0;
  dat_827e4 = 0;
  dat_82abc = 0xffffffff;
  dat_82ac0 = 0;
  dat_82b54 = 0xffffffff;
  dat_82b58 = 0;
  dat_82b20 = 1;
  dat_82b24 = -1;
  dat_827fc = 0xffffffff;
  if ((2 <= argc) && (v11 = argv[1], *v11 != '-')) {
    v23 = 0x2d;
    v26 = &argv[2];
    v44 = 0;
    argc = (int4)strlen(v11) + argc;
    v29 = (char **)sub_4ecc0((int8)argc * 8);
    v11 = argv[1];
    v17 = &v29[1];
    *v29 = *argv;
    v9 = *v11;
    argv = &argv[v27];
    if (v9) {
      v16 = v17;
      do {
        while( true ) {
          v17 = &v16[1];
          v43 = v9;
          *v16 = (char *)sub_4ee30(&v23);
          v12 = sub_2d3c0(dat_7fb20,(int4)*v11);
          if (v12) break;
          if (dat_7fb40) {
            v25 = dat_7fb40;
            for (v4 = (unsigned long *)*dat_7fb40; v4; v4 = (unsigned long *)*v4) {
              v12 = sub_2d3c0((int8 *)*v4);
              if (v12) goto label_b2c8;
              v4 = &v25[4];
              v25 = &v25[4];
            }
          }
label_b35e:
          v9 = v11[1];
          v11 = &v11[1];
          v16 = v17;
          if (!v9) goto label_b36d;
        }
label_b2c8:
        if (!*(int8 *)(v12 + 0x10)) goto label_b35e;
        if (argv <= v26) {
          if (dat_82d50)
            (*dat_82d50)();
          v9 = *v11;
          error(0,0,dcgettext(NULL,"Old option \'%c\' requires an argument.",5),v9);
          sub_2dea0(2); // no-return
        }
        v13 = *v26;
        v11 = &v11[1];
        v17 = &v16[2];
        v26 = &v26[1];
        v16[1] = v13;
        v9 = *v11;
        v16 = v17;
      } while (v9);
    }
label_b36d:
    if (v26 < argv) {
      v16 = v26;
      v24 = v17;
      do {
        v3 = &v24[1];
        v2 = &v16[1];
        *v24 = *v16;
        v16 = v2;
        v24 = v3;
      } while (v2 < argv);
      v17 = (char **)((int8)v17 + ((int8)argv + (0xffffffffffffffffU - (int8)v26) & 0xfffffffffffffff8U) + 8);
    }
    *v17 = NULL;
    v27 = (uint8)(uint4)(argc - 1);
    argv = v29;
  }
  v11 = getenv("TAR_OPTIONS");
  v33 = "TAR_OPTIONS";
  v20[0] = 0;
  v34 = 0;
  v35 = 0;
  if (v11) {
    v42 = 1;
    v10 = sub_375a0(v11,&v21,0x2000e46);
    v8 = v19;
    if (v10) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = (char *)sub_358f0(&v21);
      v13 = dcgettext(NULL,"cannot split TAR_OPTIONS: %s",5);
      error(0,0,v13,v11);
      sub_16fd0();
    }
    if (v21) {
      v19 = v20;
      v10 = (int4)v21 + (int4)v42;
      *v41 = dat_82dd0;
      if (argp_parse((void *)0x7fb20,v10,v41,0x28,v22,&v19))
        abort(); // no-return
      if (dat_82630) {
        v19 = v8;
        if (dat_82d50)
          (*dat_82d50)();
        v11 = v33;
        error(0,0,dcgettext(NULL,"non-option arguments in %s",5),v11);
        sub_2dea0(2); // no-return
      }
      v21 = 0;
    }
    v19 = v8;
    sub_35800(&v21);
  }
  if (argp_parse((void *)0x7fb20,v27 & 0xffffffff,argv,8,v20,&v19))
    exit(2); // no-return
  v1 = dat_82838;
  v5 = v37;
  if ((char)v37) {
    if (dat_82ba8 != 3) // branch-flip
      dat_82aa8 = 0xffffffff;
    else {
      sub_2e430("v7");
      v1 = dat_82838;
      v5 = v37;
    }
  }
  while( true ) {
    dat_82838 = v1;
    v37 = v5;
    if (v20[0] >= (int4)v27) break;
    sub_28130(argv[v20[0]]);
    v20[0] += 1;
    v1 = dat_82838;
    v5 = v37;
  }
  if (!dat_82ba4) {
    v37._1_1_ = (char)((uint2)v5 >> 8);
    dat_82ba4 = (v37._1_1_) ? 4 : 6; // branch-flip
  }
  if (((((dat_82a40) && (dat_82ba8 == 3)) || (dat_82b50)) || ((dat_82b10 || (dat_82a88)))) && (!(1 << ((uint1)dat_82ba4 & 0x1f) & 0x54U))) {
    if (dat_82d50)
      (*dat_82d50)();
    v11 = "GNU features wanted on incompatible archive format";
    goto label_bae2;
  }
  if (dat_82b38) {
    if (!dat_82630) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "--occurrence is meaningless without a file list";
      goto label_bae2;
    }
    v27 = (uint8)dat_82ba8;
    if (!(*(uint1 *)(v27 * 4 + 0x69a60) & 0x10)) {
      if ((dat_82bc8) && (*dat_82bc8 == 1))
        sub_2e2f0("--occurrence",(char *)sub_2dd30(v27)); // no-return
      dat_82b38 = 0;
    }
  }
  if (!dat_82830) {
    dat_82830 = 1;
    *v1 = (int8)getenv("TAPE");
    if (!*dat_82838)
      *dat_82838 = 0x6b716;
  }
  v8 = dat_82bd0;
  if ((2 <= dat_82830) && (!dat_82b10)) {
    if (dat_82d50)
      (*dat_82d50)();
    v11 = "Multiple archive files require \'-M\' option";
    goto label_bae2;
  }
  if (dat_82b28) { // branch-flip
    if (0 <= dat_82b08) {
      v10 = sub_2dca0(dat_82bd0,dat_82bd8);
      if (v10)
        sub_2e2f0("--listed-incremental","--newer"); // no-return
      if (*v8 == 1) {
        dat_82b28 = 0;
        goto label_aef1;
      }
      dat_82b00 = 0;
    }
  }
  else {
label_aef1:
    if (dat_82b24 != -1) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"--level is meaningless without --listed-incremental",5));
    }
  }
  v11 = dat_82a40;
  if ((dat_82a40) && ((dat_82ba4 & 0xfffffffb) == 2)) {
    v27 = (uint8)(-(uint4)(dat_82b10 == '\0') & 0x12) + 0x51;
    if (v27 < strlen(dat_82a40)) {
      if (dat_82d50) {
        (*dat_82d50)();
        v11 = dat_82a40;
      }
      v14 = sub_4ce40(v11);
      error(0,0,dcngettext(NULL,"%s: Volume label is too long (limit is %lu byte)","%s: Volume label is too long (limit is %lu bytes)",v27,5),v14,v27);
      sub_2dea0(2); // no-return
    }
  }
  v8 = dat_82bf8;
  if (dat_82a50) { // branch-flip
    if (dat_82b10) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "Cannot verify multi-volume archives";
      goto label_bae2;
    }
    if (dat_82b70) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "Cannot verify compressed archives";
      goto label_bae2;
    }
    v27 = (uint8)dat_82ba8;
    if (!(*(uint1 *)(v27 * 4 + 0x69a60) & 2)) {
      if ((dat_82be0) && (*dat_82be0 == 1))
        sub_2e2f0("--verify",(char *)sub_2dd30(v27)); // no-return
      dat_82a50 = '\0';
    }
  }
  else if (dat_82b70) {
    if (dat_82b10) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "Cannot use multi-volume compressed archives";
      goto label_bae2;
    }
    if (*(uint1 *)((uint8)dat_82ba8 * 4 + 0x69a60) & 4) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "Cannot update compressed archives";
      goto label_bae2;
    }
    if (dat_82ba8 == 2) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "Cannot concatenate compressed archives";
      goto label_bae2;
    }
  }
  if ((dat_82af0 == 2) && (dat_82ae8 <= -1)) {
    if (dat_82d50)
      (*dat_82d50)();
    v11 = "--clamp-mtime needs a date specified using --mtime";
    goto label_bae2;
  }
  if ((v37._1_1_) && (dat_82ba4 != 4)) {
    if (!(*(uint1 *)((uint8)dat_82ba8 * 4 + 0x69a60) & 1)) {
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "--pax-option can be used only on POSIX archives";
      goto label_bae2;
    }
    if (1 <= dat_82a9c) { // branch-flip
label_b01b:
      if (dat_82aa0 <= 0) goto label_b41f;
    }
    else {
      if (dat_82aa0 <= 0) goto label_b41f;
label_b3ed:
      if (!(*(uint1 *)((uint8)dat_82ba8 * 4 + 0x69a60) & 1)) {
        if (dat_82d50)
          (*dat_82d50)();
        v11 = "--selinux can be used only on POSIX archives";
        goto label_bae2;
      }
    }
label_b028:
    if (dat_82a78) goto label_b06c;
label_b031:
    if ((dat_82aac) && (!(*(uint1 *)((uint8)dat_82ba8 * 4 + 0x69a60) & 1))) {
label_b050:
      if ((dat_82bf0) && (*dat_82bf0 == 1))
        sub_2e2f0("--same-order",(char *)sub_2dd30(dat_82ba8)); // no-return
      dat_82aac = '\0';
    }
  }
  else {
    if (1 <= dat_82a9c) { // branch-flip
      if (dat_82ba4 != 4) {
        if (!(*(uint1 *)((uint8)dat_82ba8 * 4 + 0x69a60) & 1)) {
          if (dat_82d50)
            (*dat_82d50)();
          v11 = "--acls can be used only on POSIX archives";
          goto label_bae2;
        }
        goto label_b01b;
      }
      if (dat_82aa0 <= 0) goto label_b41f;
    }
    else if (1 <= dat_82aa0) { // branch-flip
      if (dat_82ba4 != 4) goto label_b3ed;
    }
    else {
label_b41f:
      if ((1 <= dat_82a98) && (dat_82ba4 != 4)) {
        if (!(*(uint1 *)((uint8)dat_82ba8 * 4 + 0x69a60) & 1)) {
          if (dat_82d50)
            (*dat_82d50)();
          v11 = "--xattrs can be used only on POSIX archives";
          goto label_bae2;
        }
        goto label_b028;
      }
    }
    if (!dat_82a78) goto label_b031;
    v27 = (uint8)dat_82ba8;
    if (!(*(uint1 *)(v27 * 4 + 0x69a60) & 1)) {
      if ((dat_82be8) && (*dat_82be8 == 1))
        sub_2e2f0("--starting-file",(char *)sub_2dd30(v27)); // no-return
      dat_82a78 = '\0';
      if (!dat_82aac) goto label_b06c;
      goto label_b050;
    }
  }
label_b06c:
  if (dat_82ad0) {
    if (dat_82b92) {
      v10 = sub_2dca0(dat_82bf8,dat_82c00);
      if (v10)
        sub_2e2f0("--one-top-level","--absolute-names"); // no-return
      if (*v8 != 1) {
        dat_82ad0 = '\0';
        goto label_b0b8;
      }
      dat_82b92 = '\0';
    }
    if (!dat_82ac8) {
      v11 = (char *)sub_40250((char *)*dat_82838);
      dat_82ac8 = sub_2b8b0(v11);
      free(v11);
      if (!dat_82ac8) {
        if (dat_82d50)
          (*dat_82d50)();
        v11 = "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR";
        goto label_bae2;
      }
    }
  }
label_b0b8:
  v8 = dat_82bf0;
  v7 = dat_82ba8;
  if (dat_82aba)
    dat_82b34 = 3;
  if (dat_82b69) { // branch-flip
    dat_82808 = 0x80900;
    if (dat_82b88 != 2) { // branch-flip
      dat_82804 = 0x90900;
      dat_82800 = 0;
    }
    else {
      v15 = 0x40000;
label_b7a6:
      dat_82808 |= v15;
      dat_82800 = (dat_82b69 ^ 1) << 8;
      dat_82804 = dat_82808 | 0x10000;
    }
  }
  else {
    if (dat_82b88 != 2) {
      dat_82808 = 0xa0900;
      v15 = 0;
      goto label_b7a6;
    }
    dat_82808 = 0xe0900;
    dat_82804 = 0xf0900;
    dat_82800 = 0x100;
  }
  if (dat_82ba8 != 9) { // branch-flip
    if (dat_82b91)
      dat_82a54 = 2;
  }
  else if (!dat_82630)
    dat_82a54 += 1;
  if ((dat_82a70 != dat_63748) && (dat_82a70 < (float8)dat_82b98)) {
    if (dat_82d50)
      (*dat_82d50)();
    v11 = "Volume length cannot be less than record size";
label_bae2:
    error(0,0,dcgettext(NULL,v11,5));
    sub_2dea0(2); // no-return
  }
  if ((dat_82aac) && (dat_82b28)) {
    v10 = sub_2dca0(dat_82bf0,dat_82bd0);
    if (v10)
      sub_2e2f0("--preserve-order","--listed-incremental"); // no-return
    if (*v8 != 1) // branch-flip
      dat_82aac = '\0';
    else {
      dat_82b28 = 0;
    }
  }
  switch(v7) {
    case 1:
    case 2:
    case 8:
      v1 = &dat_82838[dat_82830];
      dat_82828 = dat_82838;
      v6 = dat_82838;
      while (v6 < v1) {
        v11 = (char *)*v6;
        dat_82828 = v6;
        if (!strcmp(v11,"-")) {
          if (dat_82d50)
            (*dat_82d50)();
          v11 = "Options \'-Aru\' are incompatible with \'-f -\'";
          goto label_bae2;
        }
        dat_82828 = &v6[1];
        v6 = dat_82828;
      }
      break;
    case 3:
      if (!dat_82630) {
        if (dat_82d50)
          (*dat_82d50)();
        v11 = "Cowardly refusing to create an empty archive";
        goto label_bae2;
      }
      if ((v38) && (dat_82830)) {
        v11 = (char *)*dat_82838;
        if (strcmp(v11,"-"))
          sub_2b880(v11,dat_82b70);
      }
      break;
    case 5:
    case 6:
    case 7:
    case 9:
      v1 = &dat_82838[dat_82830];
      dat_82828 = dat_82838;
      v6 = dat_82838;
      while (v6 < v1) {
        v11 = (char *)*v6;
        dat_82828 = v6;
        if (!strcmp(v11,"-"))
          sub_2df30(0x66f36);
        dat_82828 = &v6[1];
        v6 = dat_82828;
      }
    
  }
  if (dat_82820) { // branch-flip
    dat_81f28 = fopen(dat_82820,"w");
    if (!dat_81f28)
      sub_32ec0(dat_82820); // no-return
  }
  else if (dat_82a6a) // branch-flip
    dat_81f28 = stderr;
  else {
    dat_81f28 = stdout;
  }
  dat_82828 = dat_82838;
  if (v39)
    dat_82db8 = sub_4ee30(v39);
  if ((dat_82b84) && (((dat_82b80 = sub_40040("--backup",v40), !dat_82b80 || (dat_82a6a)) || (dat_82a60))))
    dat_82b84 = '\0';
  sub_10ba0();
  v4 = v36;
  while (v4) {
    v25 = (unsigned long *)*v4;
    if (dat_82a54) { // branch-flip
      v13 = (char *)sub_23240(v4[1],v4[2],1);
      v11 = (char *)v4[4];
      if (strcmp(v11,v13)) {
        if (dat_82d50) {
          (*dat_82d50)();
          v11 = (char *)v4[4];
        }
        v14 = v4[3];
        error(0,0,dcgettext(NULL,"Option %s: Treating date \'%s\' as %s",5),v14,v11,v13);
        v11 = (char *)v4[4]; // crossjump-dupe
      }
    }
    else {
      v11 = (char *)v4[4];
    }
    free(v11);
    free(v4);
    v4 = v25;
  }
  sub_28180();
  if (dat_82a48)
    sub_dbf0();
  switch(dat_82ba8) {
    case 0:
      if (dat_82d50)
        (*dat_82d50)();
      v11 = "You must specify one of the \'-Acdtrux\', \'--delete\' or \'--test-label\' options";
      goto label_bae2;
    case 1:
    case 2:
    case 8:
      sub_31350();
      break;
    case 3:
      sub_16020();
      break;
    case 4:
      sub_16780();
      break;
    case 5:
      sub_110c0();
      sub_23e80(sub_11680);
      break;
    case 6:
      sub_1a0b0();
      sub_23e80(sub_1a1e0);
      sub_1a080();
      break;
    case 7:
      sub_23e80(sub_24580);
      break;
    case 9:
      sub_24680();
    
  }
  sub_10fa0();
  if (dat_82a69)
    sub_cc30();
  if (dat_82c20)
    sub_148b0();
  if (dat_82a48)
    sub_dcf0();
  free(dat_82838);
  sub_320a0();
  sub_281e0();
  if (dat_82d58 == 2)
    error(0,0,dcgettext(NULL,"Exiting with failure status due to previous errors",5));
  if (dat_81f28 != stdout) { // branch-flip
    if (((*(uint1 *)stderr & 0x20) || (fclose(stderr))) && (dat_82d58 <= 1))
      dat_82d58 = 2;
  }
  else {
    sub_40180();
  }
  if (v45 != *(int8 *)(v28 + 0x28))
    __stack_chk_fail(); // no-return
  return dat_82d58;
}


// Function: sub_bf60 @ 0xbf60
int4 sub_bf60(float8 a0,FILE *a1,char *a2)
{
  float8 v1;
  int4 v2;
  char *v3; // rax
  char v4 [656];
  char v5 [664];
  uint8 v6; // rdi
  char *v7; // rax
  float8 v8; // xmm2_qa
  
  v2 = 0;
  if ((a2) && (v2 = 0, *a2))
    v2 = __fprintf_chk(a1,1,"%s: ",dcgettext(NULL,a2,5));
  v1 = dat_63758;
  if ((dat_81f78 <= dat_63748) || (v8 = a0 / dat_81f78, dat_63750 <= v8))
    v7 = "?";
  else {
    if (dat_63758 <= v8)
      v6 = (int8)(v8 - dat_63758) ^ 0x8000000000000000;
    else {
      v6 = (uint8)v8;
    }
    v7 = (char *)sub_44440(v6,v5,0x1b0,1,1);
  }
  if (v1 <= a0)
    v6 = (int8)(a0 - v1) ^ 0x8000000000000000;
  else {
    v6 = (uint8)a0;
  }
  v3 = (char *)sub_44440(v6,v4,0x1b0,1,1);
  return __fprintf_chk(a1,1,"%.0f (%s, %s/s)",a0,v3,v7) + v2;
}


// Function: sub_c100 @ 0xc100
void sub_c100(void)
{
  int8 v1;
  int8 v2; // rbp
  
  v2 = (int8)dat_82178;
  v1 = *(int8 *)(v2 * 8 + 0x82180);
  if (!v1) {
    *(unsigned long *)(v2 * 8 + 0x82180) = sub_265a0((int8 *)(v2 * 8 + 0x82190),dat_82b98);
    v1 = *(int8 *)((int8)dat_82178 * 8 + 0x82180);
  }
  dat_81f58 = v1;
  dat_81f48 = v1;
  dat_81f50 = v1 + (int8)dat_82ba0 * 0x200;
  return;
}


// Function: sub_c180 @ 0xc180
uint8 sub_c180(void)
{
  int8 *v1;
  int8 v2;
  uint8 v3;
  uint8 v4;
  uint8 v5;
  int8 *v6;
  int8 v7;
  int8 *v8;
  
  sub_10cf0(1);
  if ((dat_82a70 != dat_63748) && (dat_82a70 <= dat_821a0)) {
    *__errno_location() = 0x1c;
    return 0;
  }
  v4 = dat_82b98;
  if (!dat_82a30)
    v4 = sub_2c140();
  if (!v4)
    return 0;
  if (!dat_82b10)
    return v4;
  v6 = dat_82148;
  if (dat_82138)
    return v4;
  while( true ) {
    if (!v6)
      return v4;
    v1 = (int8 *)*v6;
    if ((!v1) || (v4 < (uint8)(v1[1] << 9))) break;
    v6 = v1;
  }
  v2 = v6[1];
  v5 = v4 + v2 * -0x200;
  v3 = v6[4];
  v7 = (v5 >> 9) + v6[5];
  if (v3 <= v5)
    v5 = v3;
  v6[5] = v7;
  v6[4] = v3 - v5;
  v7 += v2;
  if (v3 - v5) {
    v7 = v2;
    v1 = v6;
  }
  v6 = dat_82148;
  if ((v1 != dat_82148) && (v8 = dat_82148, dat_82148)) {
    do {
      v6 = (int8 *)*v8;
      free((void *)v8[2]);
      free(v8);
      if (!v6) break;
      v8 = v6;
    } while (v1 != v6);
  }
  if (!v6)
    dat_82140 = 0;
  for (; v1; v1 = (int8 *)*v1) {
    v1[1] = v1[1] - v7;
    v1[5] = 0;
  }
  dat_82148 = v6;
  return v4;
}


// Function: sub_c400 @ 0xc400
void sub_c400(int8 a0) // return-dupe
{
  unsigned long *v1;
  int8 v2; // rcx
  void *v3 [14]; // stack - 0x1d8
  void **v4;
  unsigned int v5; // stack - 0x168
  uint4 v6; // stack - 0x164
  uint4 v7; // stack - 0x160
  unsigned long v8; // stack - 0x150
  unsigned long v9; // stack - 0xc0
  
  if (dat_82ba4 != 4)
    return;
  v2 = 0x37;
  v4 = v3;
  while (v2) {
    v1 = &v4[1];
    *v4 = NULL;
    v2 -= 1;
    v4 = (void **)v1;
  }
  v3[0] = *(void **)(a0 + 0x10);
  v5 = 0x81a4;
  v3[1] = v3[0];
  v6 = getuid();
  v7 = getgid();
  v3[0] = (void *)sub_1cdf0(v3,"%d/GNUFileParts/%f.%n",(int8)dat_7f014);
  v3[1] = v3[0];
  v8 = *(unsigned long *)(a0 + 0x20);
  v9 = v8;
  v1 = (unsigned long *)sub_13570(v3);
  if (v1) {
    sub_13230((uint1 *)sub_13200(0,v3,v1));
    free(v3[0]);
    return;
  }
  abort(); // no-return
}


// Function: sub_c660 @ 0xc660
void sub_c660(void)
{
  sub_1dd60((struct_5 *)0x820e8); // tail-call
  return;
}


// Function: sub_c670 @ 0xc670
void sub_c670(struct_48 *a0)
{
  if (!dat_82b10)
    return;
  sub_c500(a0->field_0x0,a0->field_0x88,a0->field_0x88); // tail-call
  return;
}


// Function: sub_c6a0 @ 0xc6a0
void sub_c6a0(void) // return-dupe
{
  int8 *v1;
  int8 *v2;
  
  if (!dat_82b10)
    return;
  v2 = dat_82148;
  if (!dat_82148) {
    dat_82148 = NULL;
    dat_82140 = 0;
    return;
  }
  do {
    v1 = (int8 *)*v2;
    free((void *)v2[2]);
    free(v2);
    v2 = v1;
  } while (v1);
  dat_82140 = 0;
  dat_82148 = NULL;
  return;
}


// Function: sub_c710 @ 0xc710
void sub_c710(unsigned long a0) // return-dupe
{
  if (!dat_82148)
    return;
  *(unsigned long *)(dat_82148 + 0x20) = a0;
  return;
}


// Function: sub_c730 @ 0xc730
void sub_c730(void)
{
  dat_82164 = 0;
  return;
}


// Function: sub_c740 @ 0xc740
void sub_c740(void)
{
  sub_43220((timespec *)0x82a20);
  dat_82a10 = dat_82a20;
  dat_82a18 = dat_82a28;
  dat_82a00 = dat_82a20;
  dat_82a08 = dat_82a28;
  return;
}


// Function: sub_c770 @ 0xc770
float8 sub_c770(void)
{
  int8 v1; // stack - 0x28
  int8 v2; // stack - 0x20
  
  sub_43220(&v1);
  dat_81f78 = (float8)(v2 - dat_82a08) / dat_63760 + (float8)(v1 - dat_82a00) + dat_81f78;
  sub_43220((timespec *)0x82a00);
  return dat_81f78;
}


// Function: sub_c810 @ 0xc810
int8 sub_c810(int4 *a0)
{
  int4 v1; // eax
  int4 *v2; // rcx
  int4 v3;
  int8 v4; // rsi
  
  if ((!dat_82b70) && (dat_81f70)) {
    *a0 = 0;
    if (dat_7f020) { // branch-flip
      v2 = (int4 *)0x7f038;
      v1 = 0;
      v3 = dat_7f020;
      do {
        v4 = (int8)v1;
        v1 += 1;
        if (dat_81f70 == v3) {
          *a0 = v1;
          return *(int8 *)(v4 * 0x18 + 0x7f028);
        }
        v3 = *v2;
        v2 = &v2[6];
      } while (v3);
    }
    else {
      v1 = 0;
    }
    *a0 = v1;
    return 0;
  }
  return dat_82b70;
}


// Function: sub_c890 @ 0xc890
unsigned long sub_c890(uint4 *a0)
{
  int4 v1;
  int4 v2; // eax
  uint4 v3;
  uint8 v4;
  int4 *v5;
  
  if (dat_82b70)
    return 0;
  v3 = *a0;
  v4 = (uint8)(int4)v3;
  v5 = (int4 *)(v4 * 0x18 + 0x7f020);
  v1 = *v5;
  while( true ) {
    if (!v1) {
      *a0 = v3;
      return 0;
    }
    v2 = (int4)v4;
    v3 = v2 + 1;
    v4 = (uint8)v3;
    if (dat_81f70 == v1) break;
    v1 = v5[6];
    v5 = &v5[6];
  }
  *a0 = v3;
  return *(unsigned long *)((int8)v2 * 0x18 + 0x7f028);
}


// Function: sub_c900 @ 0xc900
int4 sub_c900(FILE *a0,unsigned long *a1,uint8 a2,uint4 a3)
{
  char *v1;
  int4 v2;
  int4 v3; // eax
  uint8 v4; // rax
  char *v5;
  char v6 [24];
  uint8 v7;
  float8 v8; // xmm0_qa
  
  if (8 <= dat_82ba8) { // branch-flip
    if (dat_82ba8 != 8)
      abort(); // no-return, return-dupe
  }
  else {
    if (4 < dat_82ba8) {
      v5 = (char *)*a1;
      v2 = sub_bf60(a0,dcgettext(NULL,v5,5));
      goto label_cac6;
    }
    if (dat_82ba8 == 4) {
      v4 = dat_81f38 * dat_82b98;
      v7 = a2;
      if ((int8)v4 < 0)
        v7 = v4 >> 1 | (uint8)((uint4)v4 & 1);
      v2 = sub_bf60(a0,*a1,v7);
      v1 = *(char **)&a0->field_0x28;
      if (*(char **)&a0->field_0x30 <= v1) // branch-flip
        __overflow(a0,(uint4)a2 & 0xff);
      else {
        *(char **)&a0->field_0x28 = &v1[1];
        *v1 = (char)a2;
      }
      v3 = sub_bf60(a0,a1[1]);
      v1 = *(char **)&a0->field_0x28;
      if (*(char **)&a0->field_0x30 <= v1) // branch-flip
        __overflow(a0,(uint4)a2 & 0xff);
      else {
        *(char **)&a0->field_0x28 = &v1[1];
        *v1 = (char)a2;
      }
      v5 = (char *)a1[2];
      v2 = v2 + v3 + 2;
      if ((v5) && (*v5)) {
        v5 = dcgettext(NULL,v5,5);
        v2 += __fprintf_chk(a0,1,"%s: ",v5);
      }
      v8 = (float8)(uint8)((dat_81f38 - dat_822d8) * dat_82b98) - (dat_821a8 + dat_821a0);
      if (dat_63758 <= v8)
        v7 = (int8)(v8 - dat_63758) ^ 0x8000000000000000;
      else {
        v7 = (uint8)v8;
      }
      v5 = (char *)sub_45370(v7,v6);
      v2 += __fprintf_chk(a0,1,"%s",v5);
      goto label_cac6;
    }
    if (!dat_82ba8) {
      abort();
    }
  }
  v2 = sub_bf60(a0,a1[1]);
label_cac6:
  if (a3) {
    v1 = *(char **)&a0->field_0x28;
    if (*(char **)&a0->field_0x30 <= v1) // branch-flip
      __overflow(a0,a3 & 0xff);
    else {
      *(char **)&a0->field_0x28 = &v1[1];
      *v1 = (char)a3;
    }
    v2 += 1;
  }
  return v2;
}


// Function: sub_cc30 @ 0xcc30
int4 sub_cc30(void)
{
  int4 v1; // eax
  
  v1 = sub_c900(stderr,(unsigned long *)0x7d140,10,10); // tail-call
  return v1;
}


// Function: sub_cc80 @ 0xcc80
void sub_cc80(void) // return-dupe
{
  if (!dat_82161)
    return;
  dat_81f40 = 1;
  dat_81f48 = dat_81f58;
  dat_81f50 = dat_81f58 + (int8)dat_82ba0 * 0x200;
  dat_82161 = 0;
  return;
}


// Function: sub_cd20 @ 0xcd20
void sub_cd20(int4 a0)
{
  if (!close(a0))
    return;
  sub_32d60(dcgettext(NULL,"(pipe)",5)); // tail-call
  return;
}


// Function: sub_cd60 @ 0xcd60
void sub_cd60(void)
{
  int4 v1;
  char *v2; // rsi
  
  sub_32f00((char *)*dat_82828);
  v1 = dat_82164;
  if (dat_82170) { // branch-flip
    dat_82164 += 1;
    if (v1 <= 10)
      return;
    if (dat_82d50)
      (*dat_82d50)();
    v2 = "Too many errors, quitting";
  }
  else {
    if (dat_82d50)
      (*dat_82d50)();
    v2 = "At beginning of tape, quitting now";
  }
  error(0,0,dcgettext(NULL,v2,5));
  sub_16fd0(); // no-return
}


// Function: sub_cdf0 @ 0xcdf0
void sub_cdf0(uint8 a0)
{
  stat v1; // stack - 0xc8
  uint8 v2;
  void *v3;
  
  v3 = (void *)(dat_81f58 + a0);
  v2 = dat_82b98 - a0;
  if ((((v2) && (dat_81b82 & 0x40)) && (!(v2 & 0x1ff) && !dat_82170)) && (a0)) {
    if (fstat(dat_82a34,&v1)) // branch-flip
      sub_26460((char *)*dat_82828);
    else if ((v1._24_4_ & 0xb000) == 0x2000) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcngettext(NULL,"Record size = %lu block","Record size = %lu blocks",a0 >> 9,5),a0 >> 9);
    }
  }
  while( true ) {
    if (v2 & 0x1ff) { // branch-flip
      if (!a0) break;
    }
    else if (((!v2) || (!a0)) || (!dat_82160)) break;
    while( true ) {
      if (0x40000000 <= dat_82a34) // branch-flip
        a0 = sub_34460(dat_82a34 + -0x40000000,v3,v2);
      else {
        a0 = sub_4d570(dat_82a34,v3,v2);
      }
      if (a0 != 0xffffffffffffffff) break;
      sub_cd60();
    }
    if (!a0) break;
    if (!dat_82160) {
      v2 = dat_82b98 - v2;
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcngettext(NULL,"Unaligned block (%lu byte) in archive","Unaligned block (%lu bytes) in archive",v2,5),v2);
      sub_16fd0(); // no-return
    }
    v2 -= a0;
    v3 = (void *)((int8)v3 + a0);
  }
  dat_81f38 += 1;
  dat_81f50 = (dat_82b98 - v2 & 0xfffffffffffffe00) + dat_81f58;
  return;
}


// Function: sub_d000 @ 0xd000
void sub_d000(int8 a0) // return-dupe
{
  char v1;
  int8 v2; // rax
  uint8 v3; // rax
  int8 v4; // rdx
  unsigned short v5 [2]; // stack - 0x18
  void *v6;
  int8 v7; // r12
  unsigned int v8; // stack - 0x14
  
  if (dat_81f40) { // branch-flip
    v4 = (int8)dat_82ba0;
    v2 = (int8)dat_81f50 - (int8)dat_81f58 >> 9;
    v6 = dat_81f58;
  }
  else {
    if (!dat_82cf8) {
      dat_81f48 = dat_81f58;
      dat_82170 += (int8)dat_81f50 - (int8)dat_81f58 >> 9;
      dat_81f50 = (void *)((int8)dat_81f58 + (int8)dat_82ba0 * 0x200);
      (*dat_82150)(a0); // return-dupe, jump-as-call
      return;
    }
    dat_81f40 = 1;
    dat_82cf8 = '\0';
    v5[0] = 4;
    v8 = 1;
    if (0x40000000 <= dat_82a34) { // branch-flip
      sub_348b0(dat_82a34 + -0x40000000,0x40086d01,v5);
      v1 = dat_3;
    }
    else {
      v1 = (char)((uint4)ioctl(dat_82a34,0x40086d01,v5) >> 0x18);
    }
    if (v1 <= '\xff') {
      if (*__errno_location() == 5) {
        if (0x40000000 <= dat_82a34) { // branch-flip
          sub_348b0(dat_82a34 + -0x40000000,0x40086d01,v5);
          v1 = dat_3;
        }
        else {
          v1 = (char)((uint4)ioctl(dat_82a34,0x40086d01,v5) >> 0x18);
        }
        if ('\0' <= v1) goto label_d24d;
      }
      if (0x40000000 <= dat_82a34) // branch-flip
        v2 = sub_34720(dat_82a34 + -0x40000000,0,1);
      else {
        v2 = lseek(dat_82a34,0,1);
      }
      v7 = v2 - ((int8)dat_81f50 - (int8)dat_81f58);
      if (v7 <= -1)
        v7 = 0;
      if (0x40000000 <= dat_82a34) // branch-flip
        v3 = sub_34720(dat_82a34 + -0x40000000,v7,0);
      else {
        v3 = lseek(dat_82a34,v7,0);
      }
      if (v7 != v3) {
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"Cannot backspace archive file; it may be unreadable without -i",5));
        if (dat_81f58 != dat_82cf0)
          memset(dat_81f58,0,(int8)dat_82cf0 - (int8)dat_81f58);
      }
    }
label_d24d:
    v4 = (int8)dat_82ba0;
    v2 = (int8)dat_81f50 - (int8)dat_81f58 >> 9;
    if (v2 < v4) {
      memset(dat_81f50,0,(v4 - v2) * 0x200);
      dat_81f50 = (void *)((int8)dat_82ba0 * 0x200 + (int8)dat_81f58);
      return;
    }
    v6 = dat_81f58;
  }
  dat_82170 += v2;
  dat_81f50 = (void *)(v4 * 0x200 + (int8)v6);
  if (dat_81f40 == 1) {
    v2 = (int8)dat_81f48 - (int8)v6;
    dat_81f48 = v6;
    (*dat_82158)(v2); // jump-as-call
    return;
  }
  if (dat_81f40 == 2) {
    dat_81f48 = v6;
    abort(); // no-return
  }
  a0 = (int8)dat_81f48;
  dat_81f48 = v6;
  if (dat_81f40)
    return;
  (*dat_82150)(a0);
  return;
}


// Function: sub_d530 @ 0xd530
void sub_d530(void)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3; // rax
  char v4 [24];
  
  v1 = (char *)sub_45370((int8)dat_7f014,v4);
  v2 = strlen(dat_82a40);
  v3 = (char *)sub_4ecc0(v2 + 9 + strlen(v1));
  __sprintf_chk(v3,1,0xffffffffffffffff,"%s %s %s",dat_82a40,"Volume",v1);
  if (dat_82ba4 != 4) // branch-flip
    sub_d3c0(v3);
  else {
    sub_1da70("GNU.volume.label",0x81f80,v3);
  }
  free(v3);
  return;
}


// Function: sub_d600 @ 0xd600
uint8 sub_d600(char *a0)
{
  char v1;
  uint8 v2;
  uint4 *v3;
  int8 v4;
  void *v5;
  void *v6;
  bool v7; // al
  
  v1 = dat_82160;
  dat_82160 = 1;
  if (!a0)
    a0 = &v4;
  dat_81f50 = dat_81f58;
  v6 = dat_81f58;
  if (dat_81f58 != dat_81f48) { // branch-flip
label_d657:
    v7 = dat_81f48 == NULL;
    v5 = dat_81f58;
  }
  else {
    if (dat_82161) {
      *a0 = 1;
      v5 = dat_81f58;
      dat_82160 = v1;
      goto label_d6b1;
    }
    sub_d000();
    v6 = dat_81f50;
    if (dat_81f48 != dat_81f50) goto label_d657;
    dat_82161 = '\x01';
    v7 = 1;
    v5 = dat_81f58;
  }
  *a0 = v7;
  dat_82160 = v1;
  if (v5 != v6) {
    if ((!strcmp((char *)((int8)v5 + 0x101),"ustar")) || (!strcmp((char *)((int8)v5 + 0x101),"ustar  "))) {
      v2 = sub_22480(v5,1);
      if ((int4)v2 == 1)
        return v2;
      v5 = dat_81f58;
    }
  }
label_d6b1:
  v6 = (void *)0x630b4;
  v2 = 2;
  v3 = (uint4 *)0x7d190;
  while( true ) {
    if (!memcmp(v5,v6,v2))
      return (uint8)*v3;
    if (&v3[6] == (uint4 *)0x7d250) break;
    v2 = *(uint8 *)&v3[8];
    v6 = *(void **)&v3[10];
    v3 = &v3[6];
  }
  return 0;
}


// Function: sub_d780 @ 0xd780
uint8 sub_d780(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  int8 v3; // rcx
  int8 v4; // rdx
  int8 v5; // r8
  
  v1 = dat_82170;
  v3 = dat_81f48 - dat_81f58 >> 9;
  v4 = dat_82ba0 - v3;
  if ((a0 <= v4 * 0x200) || (v2 = a0 + v4 * -0x200, v2 < dat_82b98))
    return 0;
  v2 = (v2 / dat_82b98) * dat_82b98;
  if (0x40000000 <= dat_82a34) // branch-flip
    v2 = sub_34720(dat_82a34 + -0x40000000,v2,1);
  else {
    v2 = lseek(dat_82a34,v2,1);
  }
  if ((int8)v2 <= -1)
    return v2;
  if (v2 % dat_82b98) {
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"rmtlseek not stopped at a record boundary",5));
    sub_16fd0(); // no-return
  }
  v5 = ((int8)v2 >> 9) - (v3 + v1);
  dat_82170 = ((int8)v2 >> 9) - (int8)dat_82ba0;
  dat_81f38 += v5 / (int8)dat_82ba0;
  dat_81f48 = dat_81f50;
  return v5;
}


// Function: sub_d8d0 @ 0xd8d0
void sub_d8d0(void)
{
  unsigned long *v1;
  unsigned long *v2;
  int4 v3; // eax
  
  if ((dat_82cf8) || (dat_81f40 == 1)) {
    do {
      sub_d000();
    } while (dat_81f58 < dat_81f48);
  }
  sub_c770();
  if (dat_82a50)
    sub_11f60();
  if (0x40000000 <= dat_82a34) // branch-flip
    v3 = sub_34350(dat_82a34 + -0x40000000);
  else {
    v3 = close(dat_82a34);
  }
  if (v3)
    sub_32d60((char *)*dat_82828);
  sub_2bee0(dat_82168,(uint1)dat_82161);
  sub_2fc00((unsigned long *)0x82840);
  free(dat_82190);
  free(dat_82198);
  v2 = dat_82148;
  while (v2) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[2]);
    free(v2);
    v2 = v1;
  }
  dat_82148 = NULL;
  dat_82140 = 0;
  return;
}


// Function: sub_da10 @ 0xda10
void sub_da10(void)
{
  int4 v1; // eax
  
  sub_33510();
  if (0x40000000 <= dat_82a34) // branch-flip
    v1 = sub_34350(dat_82a34 + -0x40000000);
  else {
    v1 = close(dat_82a34);
  }
  if (v1)
    sub_32d60((char *)*dat_82828);
  sub_2bee0(dat_82168,0);
  sub_16fd0(); // no-return
}


// Function: sub_da70 @ 0xda70
void sub_da70(unsigned long a0)
{
  int4 v1;
  int4 *v2; // rax
  
  if (dat_82a69) {
    v2 = __errno_location();
    v1 = *v2;
    sub_cc30();
    *v2 = v1;
  }
  sub_da10(*dat_82828,a0,dat_82b98); // no-return
}


// Function: sub_dac0 @ 0xdac0
void sub_dac0(void)
{
  int8 v1; // rax
  
  v1 = sub_c180();
  if (dat_82b98 == v1) {
    dat_81f30 += 1;
    dat_821a0 = (float8)v1 + dat_821a0;
    return;
  }
  sub_da70(v1); // no-return
}


// Function: sub_dbf0 @ 0xdbf0
void sub_dbf0(void) // return-dupe
{
  int4 v1; // eax
  FILE *v2; // rax
  char *v3; // rax
  
  v2 = fopen(dat_82a48,"r");
  if (!v2) {
    if (*__errno_location() == 2)
      return;
    sub_32ea0(dat_82a48); // tail-call
    return;
  }
  if ((__isoc99_fscanf(v2,"%d",(int4 *)0x7f010) == 1) && (0 <= dat_7f010)) {
    if (*(uint1 *)v2 & 0x20) { // branch-flip
      sub_32f00(dat_82a48);
      v1 = fclose(v2);
    }
    else {
      v1 = fclose(v2);
    }
    if (!v1)
      return;
    sub_32d60(dat_82a48); // tail-call
    return;
  }
  if (dat_82d50)
    (*dat_82d50)();
  v3 = (char *)sub_4ce40(dat_82a48);
  error(0,0,dcgettext(NULL,"%s: contains invalid volume number",5),v3);
  sub_16fd0(); // no-return
}


// Function: sub_dcf0 @ 0xdcf0
void sub_dcf0(void)
{
  int4 v1; // eax
  FILE *v2; // rax
  
  v2 = fopen(dat_82a48,"w");
  if (!v2) {
    sub_32ea0(dat_82a48); // tail-call
    return;
  }
  __fprintf_chk(v2,1,"%d\n",dat_7f010);
  if (*(uint1 *)v2 & 0x20) { // branch-flip
    sub_334f0(dat_82a48);
    v1 = fclose(v2);
  }
  else {
    v1 = fclose(v2);
  }
  if (!v1)
    return;
  sub_32d60(dat_82a48); // tail-call
  return;
}


// Function: sub_e570 @ 0xe570
void sub_e570(int8 a0)
{
  unsigned long *v1;
  int8 v10;
  uint8 v11; // rax
  uint4 v12; // edx
  int8 *v13;
  int8 v14; // stack - 0x48
  uint8 v15; // rbp
  unsigned long *v16;
  unsigned long *v17;
  void *v18; // r12
  void *v19;
  unsigned long *v2;
  unsigned long *v20;
  uint4 v3;
  int8 *v4;
  int8 *v5;
  int8 v6;
  unsigned int v7;
  uint8 v8; // rax
  char *v9;
  
  dat_82158 = sub_dac0;
  v8 = sub_c180();
  if ((v8 != dat_82b98) && (!dat_82b10))
    sub_da70(v8); // return-dupe, no-return
  if (v8)
    dat_81f30 += 1;
  dat_821a0 = (float8)(int8)v8 + dat_821a0;
  v13 = dat_82148;
  if (v8 != dat_82b98) {
    while (((v13 && (v4 = (int8 *)*v13, v4)) && ((uint8)(v4[1] << 9) <= v8))) {
      v13 = v4;
    }
    if (v8 & 0x1ff) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"write did not end on a block boundary",5));
      dat_82d58 = 2;
      sub_da70(v8);
    }
    if ((int8)v8 <= -1) {
      v3 = *__errno_location();
      if ((2 <= (uint4)(v3 - 5U)) && (v3 != 0x1c)) {
        sub_da70(v8);
      }
    }
    if (!sub_dd80(1)) {
      dat_82158 = sub_e570; // return-dupe
      return;
    }
    sub_2fc00((unsigned long *)0x81f80);
    dat_7f010 += 1;
    if (dat_7f010 < 0)
      sub_c310(); // no-return
    v15 = a0 - v8;
    dat_821a8 = dat_821a8 + dat_821a0;
    dat_821a0 = 0.0;
    dat_7f014 += 1;
    v18 = (void *)(dat_81f58 + v8);
    dat_82178 = (uint4)(dat_82178 == 0);
    sub_c100();
    dat_82138 = 1;
    if (dat_82a40)
      sub_d530();
    if (v13) { // branch-flip
      if (dat_82ba4 != 4) { // branch-flip
        v20 = dat_81f48;
        if (((dat_81f48 == dat_81f50) && (v20 = NULL, !dat_82161)) && (sub_d000(), v20 = dat_81f48, dat_81f48 == dat_81f50)) {
          dat_82161 = '\x01';
          v20 = NULL;
        }
        v9 = (char *)v13[2];
        v8 = strlen(v9);
        if (0x65 <= v8) {
          if (dat_82d50) {
            (*dat_82d50)();
            v9 = (char *)v13[2];
          }
          v9 = (char *)sub_4ce40(v9);
          error(0,0,dcgettext(NULL,"%s: file name too long to be stored in a GNU multivolume header, truncated",5),v9);
          v8 = 100;
        }
        *v20 = 0;
        v20[0x3f] = 0;
        v16 = (unsigned long *)((uint8)&v20[1] & 0xfffffffffffffff8);
        v11 = (uint8)(((int4)v20 - (int4)v16) + 0x200U >> 3);
        while (v11) {
          v17 = &v16[1];
          *v16 = 0;
          v11 -= 1;
          v16 = v17;
        }
        v12 = (uint4)v8;
        v11 = v8 & 0xffffffff;
        v16 = (unsigned long *)v13[2];
        if (8 <= v12) { // branch-flip
          v17 = (unsigned long *)((uint8)&v20[1] & 0xfffffffffffffff8);
          *v20 = *v16;
          *(unsigned long *)((int8)v20 + ((v8 & 0xffffffff) - 8)) = *(unsigned long *)((int8)v16 + ((v8 & 0xffffffff) - 8));
          v10 = (int8)v20 - (int8)v17;
          v8 = (uint8)((int4)v10 + v12 >> 3);
          v16 = (unsigned long *)((int8)v16 - v10);
          while (v8) {
            v2 = &v17[1];
            v1 = &v16[1];
            *v17 = *v16;
            v8 -= 1;
            v16 = v1;
            v17 = v2;
          }
        }
        else if (v8 & 4) { // branch-flip
          *(unsigned int *)v20 = *(unsigned int *)v16;
          *(unsigned int *)((int8)v20 + (v11 - 4)) = *(unsigned int *)((int8)v16 + (v11 - 4));
        }
        else if ((v12) && (*(char *)v20 = *(char *)v16, v8 & 2))
          *(unsigned short *)((int8)v20 + (v11 - 2)) = *(unsigned short *)((int8)v16 + (v11 - 2));
        *(char *)((int8)v20 + 0x9c) = 0x4d;
        sub_12f80(v13[4],(char *)((int8)v20 + 0x7c),0xc);
        sub_12f80(v13[3] - v13[4],(char *)((int8)v20 + 0x171),0xc);
        v7 = dat_82a54;
        dat_82a54 = 0;
        sub_13320(0x82840,v20,-1);
        v16 = dat_81f48;
        if (dat_81f48 <= v20) {
          v16 = (unsigned long *)((int8)dat_81f48 + ((int8)v20 - (int8)dat_81f48 & 0xfffffffffffffe00U) + 0x200);
          dat_81f48 = v16;
        }
        v20 = dat_81f50;
        dat_82a54 = v7;
        if (dat_81f50 < v16) {
          abort(); // no-return, return-dupe
        }
      }
      else {
        v14 = v13[3] - v13[4];
        sub_1da70("GNU.volume.filename",0x81f80,v13[2]);
        sub_1da70("GNU.volume.size",0x81f80,&v13[4]);
        sub_1da70("GNU.volume.offset",0x81f80,&v14);
        v20 = dat_81f50;
      }
    }
    else {
      v20 = dat_81f50;
    }
    v16 = dat_81f48;
    if (((dat_81f48 == v20) && (v16 = NULL, !dat_82161)) && (sub_d000(), v16 = dat_81f48, dat_81f48 == dat_81f50)) {
      dat_82161 = '\x01';
      v16 = NULL;
    }
    sub_13200(1,0x81f80,v16);
    sub_2fc00((unsigned long *)0x81f80);
    if (v13)
      sub_c400(v13);
    v10 = dat_81f58;
    v4 = dat_82148;
    v20 = dat_81f48;
    if (((dat_81f48 == dat_81f50) && (v20 = NULL, !dat_82161)) && (sub_d000(), v10 = dat_81f58, v4 = dat_82148, v20 = dat_81f48, dat_81f48 == dat_81f50)) {
      dat_82161 = '\x01';
      v20 = NULL;
    }
    while ((v6 = dat_81f58, v4 && (v13 != v4))) {
      v5 = (int8 *)*v4;
      dat_81f58 = v10;
      free((void *)v4[2]);
      free(v4);
      v10 = dat_81f58;
      v4 = v5;
      dat_81f58 = v6;
    }
    if (!v4)
      dat_82140 = 0;
    for (; v13; v13 = (int8 *)*v13) {
      v13[1] = v13[1] + ((int8)v20 - dat_81f58 >> 9);
      v13[5] = 0;
    }
    dat_82138 = 0;
    v8 = (int8)dat_81f50 - (int8)v20;
    v19 = v18;
    dat_81f58 = v10;
    dat_82148 = v4;
    if (v8 < v15) {
      do {
        v18 = (void *)((int8)v19 + v8);
        memcpy(v20,v19,v8);
        v15 -= v8;
        v20 = (unsigned long *)((v8 - 1 & 0xfffffffffffffe00) + (int8)v20);
        v16 = dat_81f48;
        if (dat_81f48 <= v20) {
          v16 = (unsigned long *)((int8)dat_81f48 + ((int8)v20 - (int8)dat_81f48 & 0xfffffffffffffe00U) + 0x200);
          dat_81f48 = v16;
        }
        if (dat_81f50 < v16) {
          abort();
        }
        v17 = dat_81f50;
        v20 = dat_81f48;
        if (((dat_81f50 == v16) && (v20 = NULL, !dat_82161)) && (sub_d000(), v17 = dat_81f50, v20 = dat_81f48, dat_81f48 == dat_81f50)) {
          dat_82161 = '\x01';
          v20 = NULL;
        }
        v8 = (int8)v17 - (int8)v20;
        v19 = v18;
      } while (v8 < v15);
    }
    memcpy(v20,v18,v15);
    memset((void *)((int8)v20 + v15),0,v8 - v15);
    v20 = (unsigned long *)((int8)v20 + (v15 - 1 & 0xfffffffffffffe00));
    v16 = dat_81f48;
    if (dat_81f48 <= v20) {
      v16 = (unsigned long *)((int8)dat_81f48 + ((int8)v20 - (int8)dat_81f48 & 0xfffffffffffffe00U) + 0x200);
      dat_81f48 = v16;
    }
    if (dat_81f50 < v16)
      abort();
    if (((dat_81f50 == v16) && (!dat_82161)) && (sub_d000(), dat_81f48 == dat_81f50))
      dat_82161 = '\x01';
  }
  dat_82158 = sub_e570;
  return;
}


// Function: sub_f3f0 @ 0xf3f0
void * sub_f3f0(uint1 *a0)
{
  uint8 v1; // rax
  int8 *v2; // rax
  void *v3; // rax
  uint1 *v4;
  
  v1 = strlen((char *)a0);
  if ((v1) && (v4 = &a0[v1 - 1], a0 < v4)) {
    v2 = __ctype_b_loc();
    do {
      if (!(*(uint1 *)(*v2 + 1 + (uint8)*v4 * 2) & 8)) {
        if (v4 <= a0)
          return NULL;
        v4 = &v4[-7];
        if ((a0 < v4) && (!memcmp(v4," Volume ",8))) {
          v1 = (int8)v4 - (int8)a0;
          v3 = (void *)sub_4ecc0(v1 + 1);
          memcpy(v3,a0,v1);
          *(char *)((int8)v3 + v1) = 0;
          return v3;
        }
        return NULL;
      }
      v4 = &v4[-1];
    } while (a0 != v4);
  }
  return NULL;
}


// Function: sub_f4b0 @ 0xf4b0
void sub_f4b0(void)
{
  int8 v1;
  int4 v2; // eax
  char *v3;
  char *v4; // rax
  char v5 [360];
  char v6 [80];
  
  if (dat_81f18) {
label_f4de:
    v3 = dat_81f18;
    if (!fnmatch(dat_82a40,dat_81f18,0))
      return;
    if ((dat_82b10) && (v3 = (char *)sub_f3f0(v3), v3)) {
      v2 = fnmatch(v3,dat_82a40,0);
      free(v3);
      if (!v2)
        return;
    }
    if (dat_82d50)
      (*dat_82d50)();
    v3 = (char *)sub_4d2b0(1,dat_82a40);
    v4 = (char *)sub_4d2b0(0,dat_81f18);
    error(0,0,dcgettext(NULL,"Volume %s does not match %s",5),v4,v3);
    sub_16fd0(); // no-return
  }
  if (dat_81f48 == dat_81f50) {
    if (dat_82161) goto label_f625;
    sub_d000();
    if (dat_81f48 == dat_81f50) {
      dat_82161 = '\x01';
      goto label_f625;
    }
  }
  v1 = dat_81f48;
  if (dat_81f48) {
    if (*(char *)(dat_81f48 + 0x9c) != 'V') { // branch-flip
      if (*(char *)(dat_81f48 + 0x9c) == 'g') {
        sub_2fb40(v5);
        sub_1db80(v6,v1,sub_231e0((uint1 *)(v1 + 0x7c),0xc));
        sub_1d8a0(v5);
        sub_2fc00(v5);
      }
    }
    else {
      sub_25160((unsigned long *)0x81f18,dat_81f48,100);
    }
    if (dat_81f18) goto label_f4de;
  }
label_f625:
  if (dat_82d50)
    (*dat_82d50)();
  v3 = (char *)sub_4d2d0(dat_82a40);
  error(0,0,dcgettext(NULL,"Archive not labeled to match %s",5),v3);
  sub_16fd0(); // no-return
}


// Function: sub_f6f0 @ 0xf6f0
void sub_f6f0(void)
{
  (*dat_82158)(dat_82b98); // jump-as-call
  return;
}


// Function: sub_f710 @ 0xf710
void sub_f710(int4 a0)
{
  uint4 v1;
  char *v10;
  bool v11; // zf
  uint4 v2; // eax
  uint8 v3; // rax
  int4 *v4; // rax
  uint4 *v5; // rdx
  char v6; // stack - 0x41
  char *v7;
  int4 v8;
  unsigned long *v9;
  
  dat_82150 = sub_ec80;
  dat_82158 = sub_e570;
  if (dat_82b98) { // branch-flip
    if (dat_82830) { // branch-flip
      sub_2fc00((unsigned long *)0x82840);
      dat_82178 = 0;
      sub_c100();
      v8 = 0;
      if (a0 != 2)
        v8 = a0;
      v7 = (char *)*dat_82838;
      dat_81f40 = v8;
      if ((strcmp(v7,"-")) || (!isatty((uint4)(v8 != 0)))) {
        v7 = dat_82b70;
        v9 = dat_82838;
        dat_82160 = dat_82ab9;
        dat_81f38 = 0;
        if (dat_82b70) { // branch-flip
          if (a0 != 1) { // branch-flip
            if (a0 == 2)
              abort(); // no-return
            if (!a0) {
              dat_82168 = sub_2c570();
              dat_82160 = 1;
              dat_81f50 = dat_81f58;
            }
          }
          else {
            dat_82168 = sub_2c180();
            if ((!dat_82820) && (v7 = (char *)*dat_82838, !strcmp(v7,"-")))
              dat_81f28 = stderr;
          }
label_f7f6:
          if (dat_82a34 <= -1) {
label_fb60:
            v4 = __errno_location();
            v8 = *v4;
            goto label_fb6b;
          }
          if (0x40000000 <= dat_82a34) { // branch-flip
            sub_2be10();
            sub_2bdc0();
          }
          else {
            if (!sub_2bd50()) goto label_fb60;
            sub_2be10();
            sub_2bdc0();
          }
          if (a0) goto label_f821;
          if (((dat_81f48 == dat_81f50) && (!dat_82161)) && (sub_d000(), dat_81f48 == dat_81f50))
            dat_82161 = '\x01';
        }
        else {
          v10 = (char *)*dat_82838;
          if (strcmp(v10,"-")) {
            if (a0 != 1) {
              if (a0 != 2) {
                if (a0) goto label_f7f6;
                if (dat_82d70) { // branch-flip
label_fad1:
                  dat_82a34 = open(v10,0,0x1b6);
                }
                else {
                  dat_82d78 = strchr(v10,0x3a);
                  v10 = (char *)*v9;
                  if (((!dat_82d78) || (dat_82d78 <= v10)) || (v3 = (int8)dat_82d78 - (int8)v10, memchr(v10,0x2f,v3))) goto label_fad1;
                  dat_82a34 = sub_33a80(v10,0,0x40000000,dat_82ab0);
                }
                if (dat_82a34 == -1) goto label_fb60;
                if (dat_82b10) { // branch-flip
label_fb39:
                  dat_81f38 = 0;
                  dat_81f50 = dat_81f58;
                }
                else {
                  v8 = dat_82a34;
                  if (dat_82b70) {
label_fb04:
                    if (0x40000000 <= v8) // branch-flip
                      sub_34350(v8 + -0x40000000);
                    else {
                      close(v8);
                    }
                    dat_82161 = '\0';
                    dat_82168 = sub_2c570();
                    dat_82160 = 1;
                    goto label_fb39;
                  }
                  v8 = sub_d600(&v6);
                  if (v8) { // branch-flip
                    if (v8 != 1) {
                      dat_81f70 = v8;
label_ff1b:
                      v8 = dat_82a34;
                      goto label_fb04;
                    }
                    if (v6) {
                      if (dat_82d50)
                        (*dat_82d50)();
                      error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
                      dat_82d58 = 2;
                    }
                  }
                  else {
                    if (v6) {
                      if (dat_82d50)
                        (*dat_82d50)();
                      error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
                      dat_82d58 = 2;
                    }
                    sub_2b880((char *)*dat_82838,0);
                    if (dat_82b70) goto label_ff1b;
                  }
                }
                if (dat_82a34 <= -1) goto label_fb60;
                sub_c350();
                goto label_f7f6;
              }
              if (dat_82d70) { // branch-flip
label_fd47:
                dat_82a34 = open(v10,0x42,0x1b6);
              }
              else {
                dat_82d78 = strchr(v10,0x3a);
                v10 = (char *)*v9;
                if (((!dat_82d78) || (dat_82d78 <= v10)) || (v3 = (int8)dat_82d78 - (int8)v10, memchr(v10,0x2f,v3))) goto label_fd47;
                dat_82a34 = sub_33a80(v10,0x42,0x40000000,dat_82ab0);
              }
              if (2 <= (uint4)sub_d600(NULL)) {
                if (dat_82d50)
                  (*dat_82d50)();
                v7 = "Cannot update compressed archives";
                goto label_fd8b;
              }
              goto label_f7f6;
            }
            v11 = dat_82b84 != '\0';
            if (v11) {
              sub_25c10(v10,1);
              v10 = (char *)*dat_82838;
              v9 = dat_82838;
            }
            if (dat_82a50) { // branch-flip
              if (!dat_82d70) {
                dat_82d78 = strchr(v10,0x3a);
                v10 = (char *)*v9;
                if (((dat_82d78) && (v10 < dat_82d78)) && (v3 = (int8)dat_82d78 - (int8)v10, !memchr(v10,0x2f,v3))) {
                  dat_82a34 = sub_33a80(v10,0x42,0x40000000,dat_82ab0);
                  goto label_fcb9;
                }
              }
              dat_82a34 = open(v10,0x42,0x1b6);
            }
            else {
              if (!dat_82d70) {
                dat_82d78 = strchr(v10,0x3a);
                v10 = (char *)*v9;
                if (((dat_82d78) && (v10 < dat_82d78)) && (v3 = (int8)dat_82d78 - (int8)v10, !memchr(v10,0x2f,v3))) {
                  dat_82a34 = sub_33a80(v10,0x41,0x40000000,dat_82ab0);
                  goto label_fcb9;
                }
              }
              dat_82a34 = creat(v10,0x1b6);
            }
label_fcb9:
            if (0 <= dat_82a34) {
              if (0x40000000 <= dat_82a34) {
                sub_2be10();
                sub_2bdc0();
                goto label_f83d;
              }
              if (sub_2bd50()) goto label_fc06;
            }
            v4 = __errno_location();
            v8 = *v4;
            if (v11)
              sub_25ea0();
label_fb6b:
            *v4 = v8;
            sub_32ec0((char *)*dat_82838); // no-return
          }
          dat_82160 = 1;
          if (dat_82a50) {
            if (dat_82d50)
              (*dat_82d50)();
            v7 = "Cannot verify stdin/stdout archive";
            goto label_fd8b;
          }
          if (a0 != 1) { // branch-flip
            if (a0 != 2) {
              if (!a0) {
                dat_82a34 = 0;
                v2 = sub_d600(&v6);
                if (2 <= v2) {
                  if (dat_82d50)
                    (*dat_82d50)();
                  v5 = (uint4 *)0x7f020;
                  v8 = 0;
                  do {
                    v1 = *v5;
                    if (!v1) {
label_10069:
                      error(0,0,dcgettext(NULL,"Archive is compressed. Use %s option",5),v7);
                      sub_16fd0(); // no-return
                    }
                    v5 = &v5[6];
                    if (v2 == v1) {
                      v7 = *(char **)((int8)v8 * 0x18 + 0x7f030);
                      goto label_10069;
                    }
                    v8 += 1;
                  } while( true );
                }
                if (v6) {
                  if (dat_82d50)
                    (*dat_82d50)();
                  error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
                  dat_82d58 = 2;
                }
              }
              goto label_f7f6;
            }
            dat_82a34 = 0;
            dat_81f20 = 1;
            dat_81f50 = dat_81f58;
          }
          else {
            dat_82a34 = 1;
          }
          if (!dat_82820)
            dat_81f28 = stderr;
          if (!sub_2bd50()) goto label_fb60;
label_fc06:
          sub_2be10();
          sub_2bdc0();
label_f821:
          if (2 <= (uint4)(a0 - 1U)) {
            sub_43220((timespec *)0x82a10); // return-dupe
            dat_82a00 = dat_82a10;
            dat_82a08 = dat_82a18;
            return;
          }
          dat_81f30 = 0;
          if (a0 == 1) {
label_f83d:
            dat_81f30 = 0;
            if (dat_82a40) {
              if (dat_82b10) // branch-flip
                sub_d530();
              else if (dat_82ba4 != 4) // branch-flip
                sub_d3c0(dat_82a40);
              else {
                sub_1da70("GNU.volume.label",0x81f80,dat_82a40);
              }
            }
            sub_43220((timespec *)0x82a10);
            dat_82a00 = dat_82a10;
            dat_82a08 = dat_82a18;
            return;
          }
        }
        if (dat_82a40)
          sub_f4b0();
        sub_43220((timespec *)0x82a10);
        dat_82a00 = dat_82a10;
        dat_82a08 = dat_82a18;
        return;
      }
      if (dat_82d50)
        (*dat_82d50)();
      if (!v8) {
        v7 = dcgettext(NULL,"Refusing to read archive contents from terminal (missing -f option?)",5);
        error(0,0,v7); // return-dupe
        sub_16fd0(); // no-return
      }
      v7 = "Refusing to write archive contents to terminal (missing -f option?)";
    }
    else {
      if (dat_82d50)
        (*dat_82d50)();
      v7 = "No archive name given";
    }
  }
  else {
    if (dat_82d50)
      (*dat_82d50)();
    v7 = "Invalid value for record_size";
  }
label_fd8b:
  v7 = dcgettext(NULL,v7,5);
  error(0,0,v7);
  sub_16fd0();
}


// Function: sub_100a0 @ 0x100a0
char * sub_100a0(char *a0)
{
  char v1;
  char *v2; // rax
  uint8 v3; // rax
  
  v2 = (char *)sub_4ee30(a0);
  v1 = *v2;
  if ((v1 == '\"') || (v1 == '\'')) {
    v3 = strlen(v2);
    if (v2[v3 - 1] == v1) {
      memmove(v2,&v2[1],v3 - 2);
      v2[v3 - 2] = '\0';
      sub_251b0(v2);
      return v2;
    }
  }
  sub_251b0(v2);
  return v2;
}


// Function: sub_10110 @ 0x10110
uint8 sub_10110(FILE *a0) // early-return
{
  int4 v1; // eax
  char *v2; // rax
  int8 v3; // rax
  char v4 [2];
  uint2 v5; // stack - 0x16
  
  v1 = fileno(a0);
  if (((ioctl(v1,0x5413,v4)) || (v3 = (uint8)v5, !v5)) && ((v2 = getenv("COLUMNS"), !v2 || (v3 = strtol(v2,NULL,10), v3 <= 0))))
    return 0x50;
  return v3;
}


// Function: sub_101a0 @ 0x101a0
uint8 sub_101a0(FILE *a0,uint8 a1,uint1 *a2,char a3,unsigned int a4)
{
  uint1 v1;
  timeval v10; // stack - 0x178
  int8 v11; // stack - 0x148
  unsigned long v12 [4];
  unsigned long *v13;
  int4 v14; // r8d
  uint1 *v15;
  char *v16; // stack - 0x1a0
  char *v17; // stack - 0x198
  char *v18; // stack - 0x180
  int8 v19; // stack - 0x140
  char *v2;
  unsigned long v20; // stack - 0x110
  uint1 *v3;
  tm *v4; // rax
  int8 v5; // rax
  unsigned long v6; // rax
  uint8 v7; // rax
  char *v8;
  char v9 [24];
  
  if (a3) { // branch-flip
    v16 = dcgettext(NULL,"write",5);
    v17 = (char *)sub_45370(a4,v9);
    if (!a2)
      a2 = (uint1 *)dcgettext(NULL,"Write checkpoint %u",5);
  }
  else {
    v16 = dcgettext(NULL,"read",5);
    v17 = (char *)sub_45370(a4,v9);
    if (!a2)
      a2 = (uint1 *)dcgettext(NULL,"Read checkpoint %u",5);
  }
  v1 = *a2;
  do {
    if (!v1) {
      fflush_unlocked(a0);
      return a1 & 0xffffffff;
    }
    if (v1 != 0x25) {
      v15 = *(uint1 **)&a0->field_0x28;
      if (*(uint1 **)&a0->field_0x30 <= v15) // branch-flip
        __overflow(a0,(uint4)v1);
      else {
        *(uint1 **)&a0->field_0x28 = &v15[1];
        *v15 = v1;
      }
      a1 += 1;
      v15 = a2;
      if (*a2 == 0xd) {
        dat_821d8 = 1;
        a1 = 0;
      }
      goto label_1025c;
    }
    v1 = a2[1];
    v15 = &a2[1];
    if (v1 == 0x7b) {
      v18 = strchr((char *)&a2[2],0x7d);
      if (v18) {
        v7 = (int8)v18 - (int8)v15;
        v8 = dat_821c8;
        if (dat_821c0 < v7) {
          dat_821c0 = v7;
          v8 = (char *)sub_4ece0(dat_821c8,v7);
          dat_821c8 = v8;
        }
        memcpy(v8,&a2[2],v7 - 1);
        v8 = dat_821c8;
        v15 = (uint1 *)&v18[1];
        dat_821c8[v7 - 1] = '\0';
        if (dat_821c8) {
          if (v18[1] == '*') {
            v7 = strtol(dat_821c8,NULL,10);
            goto label_1060c;
          }
          switch(v18[1]) {
            case 0x54:
              sub_c770();
              v20 = 0x6379c;
              v5 = sub_375a0(v8,&v11,0x4644);
              if ((int4)v5) { // branch-flip
                if (dat_82d50)
                  (*dat_82d50)();
                v6 = sub_358f0(&v11);
                error(0,0,dcgettext(NULL,"cannot split string \'%s\': %s",5),v8,v6);
                v13 = (unsigned long *)0x7f140;
                dat_82d58 = 2;
              }
              else {
                if (v11) {
                  v5 = 0;
                  v13 = v12;
                  do {
                    v13[v5] = *(unsigned long *)(v19 + v5 * 8);
                    v5 += 1;
                  } while (v5 != v11);
                  if ((int4)v5 == 3) goto label_105d9;
                }
                v13 = v12;
                v5 = (int8)(int4)v5;
                do {
                  v13[v5] = 0;
                  v5 += 1;
                } while ((int4)v5 != 3);
              }
label_105d9:
              a1 += (int8)(int4)sub_c900(a0,v13,0x2c,0);
              sub_35800(&v11);
              goto label_1025c;
            case 99:
              goto label_103d0;
            case 100:
              goto label_103f8;
            case 0x73:
              goto label_10398;
            case 0x74:
              goto label_104f7;
            case 0x75:
              goto label_103c0;
            
          }
        }
      }
label_10330:
      v2 = *(char **)&a0->field_0x28;
      if (*(char **)&a0->field_0x30 <= v2) // branch-flip
        __overflow(a0,0x25);
      else {
        *(char **)&a0->field_0x28 = &v2[1];
        *v2 = 0x25;
      }
      v1 = *v15;
      v3 = *(uint1 **)&a0->field_0x28;
      if (*(uint1 **)&a0->field_0x30 <= v3) // branch-flip
        __overflow(a0,(uint4)v1);
      else {
        *(uint1 **)&a0->field_0x28 = &v3[1];
        *v3 = v1;
      }
      a1 += 2;
      goto label_1025c;
    }
    if (v1 == 0x2a) {
      v7 = sub_10110(a0);
label_1060c:
      if (a1 < v7) {
        do {
          v2 = *(char **)&a0->field_0x28;
          if (*(char **)&a0->field_0x30 <= v2) // branch-flip
            __overflow(a0,0x20);
          else {
            *(char **)&a0->field_0x28 = &v2[1];
            *v2 = 0x20;
          }
          a1 += 1;
        } while (a1 != v7);
      }
      goto label_1025c;
    }
    switch(v1) {
      case 0x54:
        sub_c770();
        a1 += (int8)(int4)sub_c900(a0,(unsigned long *)0x7f140,0x2c,0);
        break;
      default:
        goto label_10330;
      case 99:
label_103d0:
        a1 += (int8)(int4)sub_101a0(a0,a1,"%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r",(uint1)a3,a4);
        break;
      case 100:
label_103f8:
        sub_c770();
        a1 += (int8)__fprintf_chk(a0,1,"%.0f");
        break;
      case 0x73:
label_10398:
        v8 = v16;
        goto label_103a0;
      case 0x74:
        v8 = "%c";
label_104f7:
        gettimeofday(&v10,NULL);
        v4 = localtime(&v10);
        v14 = v10._8_4_ * 1000;
        a1 += sub_43180(a0,v8,v4,0,v14);
        break;
      case 0x75:
label_103c0:
        v8 = v17;
label_103a0:
        fputs_unlocked(v8,a0);
        a1 += strlen(v8);
      
    }
label_1025c:
    v1 = v15[1];
    a2 = &v15[1];
  } while( true );
}


// Function: sub_10ba0 @ 0x10ba0
void sub_10ba0(void) // return-dupe
{
  if (!dat_82260) {
    if (!dat_82b78)
      return;
    if (dat_82270)
      return;
    sub_10710("echo");
  }
  if (dat_82260 != 1)
    return;
  sigprocmask(0,(sigset_t *)0x821e0,NULL);
  if (dat_82b78) {
    dat_82260 = 2;
    return;
  }
  dat_82260 = 2;
  dat_82b78 = 10;
  return;
}


// Function: sub_10c20 @ 0x10c20
void sub_10c20(void) // return-dupe x2
{
  char *v1;
  int8 v2;
  int8 *v3;
  FILE *v4;
  
  v3 = dat_82270;
  if (!dat_82270)
    return;
  do {
    while (((v4 = dat_821d0, (int4)v3[1] != 3 || (!dat_821d0)) || (!dat_821d8))) {
      v3 = (int8 *)*v3;
      if (!v3)
        return;
    }
    for (v2 = sub_10110(dat_821d0); v2; v2 = v2 + -1) {
      v1 = *(char **)&v4->field_0x28;
      if (*(char **)&v4->field_0x30 <= v1) { // branch-flip
        __overflow(v4,0x20);
        v4 = dat_821d0;
      }
      else {
        *(char **)&v4->field_0x28 = &v1[1];
        *v1 = 0x20;
      }
    }
    v1 = *(char **)&v4->field_0x28;
    if (*(char **)&v4->field_0x30 <= v1) { // branch-flip
      __overflow(v4,0xd);
      v4 = dat_821d0;
    }
    else {
      *(char **)&v4->field_0x28 = &v1[1];
      *v1 = 0xd;
    }
    fflush_unlocked(v4);
    v3 = (int8 *)*v3;
  } while (v3);
  return;
}


// Function: sub_10cf0 @ 0x10cf0
void sub_10cf0(char a0)
{
  unsigned long *v1;
  char *v2;
  int4 v3; // eax
  FILE *v4;
  int8 v5;
  
  if ((dat_82b78) && (dat_82278 = dat_82278 + 1, v1 = dat_82270, v4 = dat_821d0, !(dat_82278 % dat_82b78))) {
    while (dat_821d0 = v4, v1) {
      switch(*(unsigned int *)&v1[1]) {
        case 0:
          v2 = *(char **)&dat_81f28->field_0x28;
          if (*(char **)&dat_81f28->field_0x30 <= v2) // branch-flip
            __overflow(dat_81f28,0x2e);
          else {
            *(char **)&dat_81f28->field_0x28 = &v2[1];
            *v2 = 0x2e;
          }
          fflush_unlocked(dat_81f28);
          break;
        case 1:
          if ((v4) || (v4 = fopen("/dev/tty","w"), dat_821d0 = v4, v4)) {
            v2 = *(char **)&v4->field_0x28;
            if (*(char **)&v4->field_0x30 <= v2) // branch-flip
              __overflow(v4,7);
            else {
              *(char **)&v4->field_0x28 = &v2[1];
              *v2 = 7;
            }
            fflush_unlocked(dat_821d0);
          }
          break;
        case 2:
          v3 = __fprintf_chk(stderr,1,"%s: ",dat_82dd0);
          sub_101a0(stderr,(int8)v3,v1[2],(uint1)a0,dat_82278);
          v2 = *(char **)&stderr->field_0x28;
          if (*(char **)&stderr->field_0x30 <= v2) // branch-flip
            __overflow(stderr,10);
          else {
            *(char **)&stderr->field_0x28 = &v2[1];
            *v2 = 10;
          }
          break;
        case 3:
          if ((v4) || (v4 = fopen("/dev/tty","w"), dat_821d0 = v4, v4))
            sub_101a0(v4,0,v1[2],(uint1)a0,dat_82278);
          break;
        case 4:
          sleep(*(uint4 *)&v1[2]);
          break;
        case 5:
          sub_2d260(v1[2],*dat_82828,dat_82278);
          break;
        case 6:
          sub_c770();
          sub_cc30();
          break;
        case 7:
          sigwait((void *)0x821e0,&v5);
        
      }
      v1 = (unsigned long *)*v1;
      v4 = dat_821d0;
    }
  }
  return;
}


// Function: sub_10fa0 @ 0x10fa0
void sub_10fa0(void)
{
  if (!dat_82b78)
    return;
  sub_10c20();
  if (dat_821d0) {
    fclose(dat_821d0); // tail-call
    return;
  }
  return;
}


// Function: sub_10fe0 @ 0x10fe0
unsigned long sub_10fe0(void)
{
  return 1;
}


// Function: sub_11120 @ 0x11120
void sub_11120(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,int8 a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char *v1;
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
  if (a9) {
    v1 = *(char **)(a8 + 8);
    __fprintf_chk(dat_81f28,1,"%s: ",(char *)sub_250f0(1,v1));
    v6 = &Stack0000000000000008;
    v3 = 0x10;
    v7 = v4;
    v5 = 0x30;
    __vfprintf_chk(dat_81f28,1,a9,&v3);
    __fprintf_chk(dat_81f28,1,"\n");
  }
  sub_2fd00(1);
  return;
}


// Function: sub_11250 @ 0x11250
unsigned long sub_11250(uint8 a0,void *a1) // return-dupe
{
  uint8 v1; // rax
  
  v1 = sub_25fe0(dat_82290,dat_82288,a0);
  if (a0 == v1) {
    if (!memcmp(a1,dat_82288,a0))
      return 1;
    sub_11120(0x82840,dcgettext(NULL,"Contents differ",5));
    return 0;
  }
  if (v1 != 0xffffffffffffffff) {
    sub_11120(0x82840,dcngettext(NULL,"Could only read %lu of %lu byte","Could only read %lu of %lu bytes",a0,5),v1,a0);
    return 0;
  }
  sub_32f00(dat_82848);
  sub_11120(0x82840,0);
  return 0;
}


// Function: sub_11340 @ 0x11340
void sub_11340(void) // return-dupe
{
  uint8 v1; // rax
  int8 v2; // rax
  char *v3; // rax
  char v4 [1032];
  
  v3 = v4;
  v1 = strlen(dat_82858);
  if (0x400 <= v1)
    v3 = (char *)sub_4ecc0(v1 + 1);
  v2 = readlinkat(dat_7f234,dat_82848,v3,v1 + 1);
  if (0 <= v2) { // branch-flip
    if ((v1 != v2) || (memcmp(dat_82858,v3,v1)))
      sub_11120(0x82840,dcgettext(NULL,"Symlink differs",5));
  }
  else {
    if (*__errno_location() != 2) // branch-flip
      sub_33140(dat_82848);
    else {
      sub_33160(dat_82848);
    }
    sub_11120(0x82840,0);
  }
  if (v3 == v4)
    return;
  free(v3);
  return;
}


// Function: sub_11460 @ 0x11460
void sub_11460(void) // return-dupe
{
  unsigned long v1;
  int4 v2; // eax
  char v3 [24];
  char *v4; // rsi
  uint4 v5; // stack - 0xa0
  
  v1 = dat_82848;
  v2 = sub_25fc0(dat_82848,v3);
  if (v2) {
    if (*__errno_location() != 2) // branch-flip
      sub_33410(v1);
    else {
      sub_33430(v1);
    }
    sub_11120(0x82840,0);
    return;
  }
  if ((v5 & 0xf000) != 0x4000) // branch-flip
    v4 = "File type differs";
  else {
    if (!((v5 ^ dat_828b0) & 0xfff))
      return;
    v4 = "Mode differs";
  }
  sub_11120(0x82840,dcgettext(NULL,v4,5));
  return;
}


// Function: sub_11560 @ 0x11560
void sub_11560(void)
{
  unsigned long v1;
  int4 v2;
  unsigned long v3;
  char v4 [144];
  char v5 [152];
  
  v1 = dat_82848;
  v2 = sub_25fc0(dat_82848,v4);
  v3 = dat_82858;
  if (v2) { // branch-flip
    v2 = *__errno_location();
    v3 = v1;
  }
  else {
    v2 = sub_25fc0(dat_82858,v5);
    if (!v2) {
      if (!sub_2c0f0(v4,v5)) {
        v3 = sub_250f0(0,dat_82858);
        sub_11120(0x82840,dcgettext(NULL,"Not linked to %s",5),v3);
        return;
      }
      return;
    }
    v2 = *__errno_location();
  }
  if (v2 != 2) // branch-flip
    sub_33410(v3);
  else {
    sub_33430(v3);
  }
  sub_11120(0x82840,0);
  return;
}


// Function: sub_11680 @ 0x11680
void sub_11680(void) // return-dupe x2
{
  uint1 v1;
  char *v10;
  uint4 v11; // stack - 0xb0
  int8 v12; // stack - 0xa0
  int8 v13; // stack - 0x98
  unsigned long v14; // stack - 0x80
  unsigned long v15; // stack - 0x78
  int8 v16; // stack - 0x70
  int8 v17; // stack - 0x68
  int4 v2; // eax
  char *v3;
  int8 v4; // rax
  int8 v5; // rax
  uint4 v6; // ecx
  code *v7; // rdx
  char v8;
  char v9 [24];
  
  sub_ccd0(dat_82408);
  if (dat_82a54) {
    if (dat_82280)
      __fprintf_chk(dat_81f28,1,dcgettext(NULL,"Verify ",5));
    sub_23bc0((struct_9 *)0x82840,dat_82408,-1);
  }
  v3 = dat_82848;
  v8 = *(char *)(dat_82408 + 0x9c);
  switch(v8) {
    case 0:
    case 0x30:
    case 0x37:
    case 0x53:
      goto label_1176d;
    default:
      if (dat_82d50) {
        (*dat_82d50)();
        v8 = *(char *)(dat_82408 + 0x9c);
      }
      v3 = (char *)sub_4ce40(dat_82848);
      error(0,0,dcgettext(NULL,"%s: Unknown file type \'%c\', diffed as normal file",5),v3,v8);
      dat_82d58 = 2;
label_1176d:
      v3 = dat_82848;
      if (dat_82850) {
        sub_11460(); // return-dupe, tail-call
        return;
      }
      v2 = sub_25fc0(dat_82848,v9);
      if (v2) {
        if (*__errno_location() != 2) // branch-flip
          sub_33410(v3);
        else {
          sub_33430(v3);
        }
        sub_11120(0x82840,0); // return-dupe
        sub_24660();
        return;
      }
      if ((v11 & 0xf000) != 0x8000) { // branch-flip
label_117e5:
        v3 = "File type differs";
      }
      else {
        if ((v11 ^ dat_828b0) & 0xfff)
          sub_11120(0x82840,dcgettext(NULL,"Mode differs",5));
        if (!sub_2c0d0(v9,0x82898))
          sub_11120(0x82840,dcgettext(NULL,"Uid differs",5));
        if (!sub_2c0e0(v9,0x82898))
          sub_11120(0x82840,dcgettext(NULL,"Gid differs",5));
        v2 = sub_2fcc0(v16,v17,dat_82938,dat_82940);
        if (v2)
          sub_11120(0x82840,dcgettext(NULL,"Mod time differs",5));
        if ((*(char *)(dat_82408 + 0x9c) == 'S') || (v13 == dat_828c8)) {
          dat_82290 = __openat_2(dat_7f234,v3,dat_82808);
          if (dat_82290 < 0) {
            sub_32ea0(v3);
            sub_24660();
            sub_11120(0x82840,0);
            return;
          }
          if (dat_82960) // branch-flip
            sub_2b690(dat_82290,0x82840);
          else {
            sub_10ff0(sub_11250);
          }
          if (((dat_82b88 == 1) && (v13)) && (v2 = sub_26100(dat_82290,dat_7f234,v3,v14,v15), v2))
            sub_334b0(v3);
          if (!close(dat_82290))
            return;
          sub_32d60(v3);
          return;
        }
label_11cd0:
        v3 = "Size differs";
      }
      sub_11120(0x82840,dcgettext(NULL,v3,5));
      sub_24660();
      return;
    case 0x31:
      sub_11560(); // tail-call
      return;
    case 0x32:
      sub_11340(); // tail-call
      return;
    case 0x33:
    case 0x34:
    case 0x36:
      v2 = sub_25fc0(dat_82848,v9);
      if (v2) {
        if (*__errno_location() != 2) // branch-flip
          sub_33410(v3);
        else {
          sub_33430(v3);
        }
        sub_11120(0x82840,0);
        return;
      }
      v6 = v11 & 0xf000;
      if (*(char *)(dat_82408 + 0x9c) != '3') { // branch-flip
        if (*(char *)(dat_82408 + 0x9c) == '4') {
          if (v6 != 0x6000) {
            sub_11120(0x82840,dcgettext(NULL,"File type differs",5)); // return-dupe
            return;
          }
          goto label_11cf4;
        }
        if (v6 != 0x1000) {
          sub_11120(0x82840,dcgettext(NULL,"File type differs",5));
          return;
        }
      }
      else {
        if (v6 != 0x2000) {
          sub_11120(0x82840,dcgettext(NULL,"File type differs",5));
          return;
        }
label_11cf4:
        v3 = "Device number differs";
        if (dat_828c0 != v12) {
          sub_11120(0x82840,dcgettext(NULL,v3,5)); // return-dupe
          return;
        }
      }
      if (!((v11 ^ dat_828b0) & 0xfff))
        return;
      v3 = "Mode differs";
      sub_11120(0x82840,dcgettext(NULL,v3,5));
      return;
    case 0x35:
    case 0x44:
      if (!sub_20fc0(0x82840)) {
        sub_11460();
        return;
      }
      if (dat_829e8) { // branch-flip
label_1195f:
        v3 = (char *)sub_1fed0(sub_1f810((unsigned long *)0x82840));
        if (v3) {
          v10 = dat_829d0;
          do {
            v1 = *v10 + 0xbc;
            v5 = 1L << (v1 & 0x3f);
            do {
              do {
                if (!*v10) {
label_11d50:
                  if (!*v3) {
                    sub_11460();
                    return;
                  }
                  goto label_11d5b;
                }
              } while (0x15 < v1);
              if (v5 & 0x114000) goto label_11d50;
              if (v5 & 0x200400) {
                v8 = *v3;
                if (!strchr("YN",(int4)v8)) goto label_11d5b;
                v2 = strcmp(&v10[1],&v3[1]);
                goto label_11d9f;
              }
            } while (!(v5 & 1));
            v2 = strcmp(v10,v3);
label_11d9f:
            if (v2) goto label_11d5b;
            v4 = strlen(v10) + 1;
            v10 = &v10[v4];
            v3 = &v3[v4];
          } while( true );
        }
        sub_10ff0(sub_10fe0);
      }
      else {
        v2 = sub_14950(dat_829d8,dat_82840,dat_82808);
        if (0 <= v2) { // branch-flip
          if (!fstat(v2,(stat *)0x82898)) {
            dat_829e8 = v2;
            goto label_1195f;
          }
          close(v2);
          v7 = sub_26460;
        }
        else {
          v7 = sub_26370;
        }
        sub_26490(dat_82840,0,v7);
      }
      sub_11460();
      return;
    case 0x4d:
      if (dat_82850) {
        sub_11460();
        return;
      }
      v2 = sub_25fc0(dat_82848,v9);
      if (v2) {
        if (*__errno_location() != 2) // branch-flip
          sub_33410(v3);
        else {
          sub_33430(v3);
        }
        sub_11120(0x82840,0);
        return;
      }
      if ((v11 & 0xf000) == 0x8000) {
        v5 = sub_231e0((uint1 *)(dat_82408 + 0x171),0xc);
        if (((0 <= v5) && (!SCARRY8(v5,dat_828c8))) && (v13 == v5 + dat_828c8)) {
          v2 = __openat_2(dat_7f234,dat_82848,dat_82808);
          if (0 <= v2) {
            if (0 <= (int8)lseek(v2,v5,0)) // branch-flip
              sub_10ff0(sub_11250);
            else {
              sub_33200(dat_82848,v5);
              sub_11120(0x82840,0);
            }
            if (!close(v2))
              return;
            sub_32d60(dat_82848);
            return;
          }
          sub_32ea0(dat_82848);
          sub_11120(0x82840,0);
          sub_24660();
          return;
        }
        goto label_11cd0;
      }
      goto label_117e5;
    case 0x56:
      return;
    
  }
label_11d5b:
  sub_11120(0x82840,dcgettext(NULL,"Contents differ",5));
  sub_11460();
  return;
}


// Function: sub_11f60 @ 0x11f60
void sub_11f60(void) // return-dupe
{
  int4 v1;
  int8 v2; // rax
  char *v3; // rax
  unsigned short v4 [2]; // stack - 0x60
  char v5 [24];
  uint4 v6; // r12d
  unsigned int v7; // stack - 0x5c
  
  if (sub_335f0()) { // branch-flip
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"Archive contains file names with leading prefixes removed.",5));
    if (sub_30e80()) goto label_12110;
label_1213f:
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"Verification may fail to locate original files.",5));
  }
  else if (sub_30e80()) {
label_12110:
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"Archive contains transformed file names.",5));
    goto label_1213f;
  }
  sub_1f6e0();
  if ((!dat_82288) && (dat_82288 = sub_265a0(v4,dat_82b98), dat_82b28))
    sub_203b0();
  fsync(dat_82a34);
  ioctl(dat_82a34,0x24b);
  v7 = 1;
  v4[0] = 2;
  if (0x40000000 <= dat_82a34) // branch-flip
    v1 = sub_348b0(dat_82a34 + -0x40000000,0x40086d01,v4);
  else {
    v1 = ioctl(dat_82a34,0x40086d01,v4);
  }
  if (0 <= v1) goto label_12044;
  if (*__errno_location() != 5) { // branch-flip
    if (0x40000000 <= dat_82a34) goto label_1202c;
label_122d0:
    v2 = lseek(dat_82a34,0,0);
  }
  else {
    if (0x40000000 <= dat_82a34) // branch-flip
      v1 = sub_348b0(dat_82a34 + -0x40000000,0x40086d01,v4);
    else {
      v1 = ioctl(dat_82a34,0x40086d01,v4);
    }
    if (0 <= v1) goto label_12044;
    if (dat_82a34 <= 0x3fffffff) goto label_122d0;
label_1202c:
    v2 = sub_34720(dat_82a34 + -0x40000000,0,0);
  }
  if (v2) {
    sub_332b0((char *)*dat_82838);
    return;
  }
label_12044:
  dat_82280 = 1;
  dat_81f40 = 0;
  sub_f6e0();
  while( true ) {
    v1 = sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0);
    if (v1 == 5) {
      v6 = 0;
      do {
        v6 += 1;
        sub_ccd0(dat_82408);
        v1 = sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0);
      } while (v1 == 5);
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcngettext(NULL,"VERIFY FAILURE: %d invalid header detected","VERIFY FAILURE: %d invalid headers detected",(int8)(int4)v6,5),(uint8)v6);
      dat_82d58 = 2;
    }
    if (v1 == 4) break;
    if (v1 != 3) { // branch-flip
      sub_22c60(dat_82408,0x82840,(int4 *)0x82400,1);
      sub_11680();
      sub_2fc00((unsigned long *)0x82840);
    }
    else {
      sub_ccd0(dat_82408);
      if (!dat_82b51) {
        if (sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0) == 3) {
          dat_81f40 = 1;
          dat_82280 = 0;
          return;
        }
        if (dat_81b80 & 1) {
          if (dat_82d50)
            (*dat_82d50)();
          v3 = (char *)sub_45370(sub_cc60(),v5);
          error(0,0,dcgettext(NULL,"A lone zero block at %s",5),v3);
        }
      }
    }
  }
  dat_82280 = 0;
  dat_81f40 = 1;
  return;
}


// Function: sub_12380 @ 0x12380
uint8 sub_12380(struct_46 *a0,uint8 a1)
{
  return (a0->field_0x0 ^ a0->field_0x8) % a1;
}


// Function: sub_123a0 @ 0x123a0
unsigned long sub_123a0(struct_46 *a0,struct_46 *a1)
{
  uint8 v1; // rax
  
  v1 = a0->field_0x0 ^ a1->field_0x0 | a0->field_0x8 ^ a1->field_0x8;
  return CONCAT71((undefined7)(v1 >> 8),v1 == 0);
}


// Function: sub_123c0 @ 0x123c0
unsigned long sub_123c0(int4 a0,int4 a1,uint8 a2,int8 a3,code *a4,char *a5,int8 a6,char *a7) // early-return
{
  char *v1;
  uint4 v10; // stack - 0xbc
  char v11 [23];
  int8 v12;
  uint8 v13;
  uint8 v14; // r15
  char *v15;
  char *v2;
  int8 v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  char *v6;
  char v7 [32];
  char v8 [32];
  char v9 [32];
  
  v1 = a7;
  if (a1) { // branch-flip
    if (0x40 <= (uint8)((a6 + -1) * 8)) { // branch-flip
      v13 = 0xffffffffffffffff;
      v14 = 0x8000000000000000;
      v2 = (char *)sub_45370(0xffffffffffffffff,v7);
    }
    else {
      v14 = 1L << ((uint1)((int4)(a6 + -1) << 3) & 0x3f);
      v13 = v14 - 1;
      v2 = (char *)sub_45370(v13,v7);
    }
    v3 = sub_45370(v14,v8);
    *(char *)(v3 + -1) = 0x2d;
    v15 = (char *)(v3 + -1);
  }
  else if (0x40 <= (uint8)(a6 * 3 - 3U)) { // branch-flip
    v13 = 0xffffffffffffffff;
    v2 = (char *)sub_45370(0xffffffffffffffff,v7);
    v15 = "0";
  }
  else {
    v15 = "0";
    v13 = (1L << ((char)a6 * '\x03' - 3U & 0x3fU)) - 1;
    v2 = (char *)sub_45370(v13,v7);
  }
  if (a0) { // branch-flip
    v3 = sub_45370(-a2,v9);
    *(char *)(v3 + -1) = 0x2d;
    v6 = (char *)(v3 + -1);
  }
  else {
    v6 = (char *)sub_45370(a2,&v12);
  }
  if (a4) {
    v13 &= (*a4)(&v10);
    v10 = dat_82ba4 == 6 & v10;
    if (v10)
      v13 = -v13;
    v4 = (char *)sub_45370(v13,v11);
    if (v10) {
      v4[-1] = '-';
      v4 = &v4[-1];
    }
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"value %s out of %s range %s..%s; substituting %s",5),v6,v1,v15,v2,v4);
    v5 = sub_12680(v10,v13,a3,0,a5,a6,v1);
    return v5;
  }
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"value %s out of %s range %s..%s",5),v6,v1,v15,v2);
  dat_82d58 = 2;
  return 0;
}


// Function: sub_12990 @ 0x12990
uint8 sub_12990(int8 a0) // return-dupe
{
  int4 *v1; // rax
  int8 v2;
  int8 v3; // rax
  uint8 v4; // r12
  
  v1 = __errno_location();
  v4 = CONCAT71((undefined7)((uint8)v1 >> 8),*v1 == 0x18 && a0) & 0xffffffff;
  if (*v1 != 0x18 || !a0)
    return v4;
  if (!*(int8 *)(a0 + 0x198))
    return 0;
  v2 = *(int8 *)(*(int8 *)(a0 + 0x198) + 0x198);
  do {
    if (!v2)
      return 0;
    while (v3 = v2, v2 = *(int8 *)(v3 + 0x198), 1 <= *(int4 *)(v3 + 0x1a8)) {
      if ((!v2) || (*(int4 *)(v2 + 0x1a8) <= 0)) {
        sub_2fb80(v3);
        return v4;
      }
    }
  } while( true );
}


// Function: sub_12a20 @ 0x12a20
void sub_12a20(char *a0)
{
  char *v1; // rax
  
  if (!(dat_81b80 & 0x20)) {
    if (dat_82b52)
      return;
    sub_2fd00(2); // tail-call
    return;
  }
  if (dat_82d50)
    (*dat_82d50)();
  v1 = (char *)sub_4ce40(a0);
  error(0,0,dcgettext(NULL,"%s: Unknown file type; file ignored",5),v1);
  if (dat_82b52)
    return;
  sub_2fd00(2); // tail-call
  return;
}


// Function: sub_12ac0 @ 0x12ac0
void sub_12ac0(int8 *a0)
{
  uint8 v1;
  char *v2;
  int8 v3; // rax
  char *v4;
  uint8 v5;
  int8 v6;
  
  v2 = (char *)*a0;
  v5 = strlen(v2);
  do {
    if (!v5) {
      v6 = 1;
      v4 = v2;
label_12afc:
      if (*v4 != '/') {
        v3 = sub_4ece0(v2,v5 + 2);
        *a0 = v3;
        v4 = (char *)(v3 + v5);
      }
      *v4 = '/';
      *(char *)(*a0 + v6) = 0;
      return;
    }
    v1 = v5 - 1;
    if (v2[v5 - 1] != '/') {
      v4 = &v2[v5];
      v6 = v5 + 1;
      goto label_12afc;
    }
    v5 = v1;
  } while( true );
}


// Function: sub_12da0 @ 0x12da0
unsigned long * sub_12da0(int8 a0)
{
  unsigned long *v1;
  char v2;
  int8 v3;
  unsigned long *v4; // rax
  int8 v5; // rax
  uint8 v6; // rcx
  unsigned long *v7;
  
  v4 = (unsigned long *)sub_d360();
  *v4 = 0;
  v4[0x3f] = 0;
  v5 = 0;
  v6 = (uint8)(((int4)v4 - (int4)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
  v7 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
  while (v6) {
    v1 = &v7[1];
    *v7 = 0;
    v6 -= 1;
    v7 = v1;
  }
  v3 = *(int8 *)(a0 + 8);
  do {
    v2 = *(char *)(v3 + v5);
    *(char *)((int8)v4 + v5) = v2;
    if (!v2) break;
    v5 += 1;
  } while (v5 != 100);
  if (dat_82ba4 == 2)
    *(char *)((int8)v4 + 99) = 0;
  return v4;
}


// Function: sub_12e10 @ 0x12e10
void sub_12e10(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long *v1; // rax
  uint8 v2; // rax
  
  v1 = (unsigned long *)sub_4ecc0(0x28);
  *v1 = a0;
  *(unsigned int *)&v1[2] = a1;
  v1[3] = a2;
  v1[4] = dat_822d0;
  v2 = strlen(a0);
  dat_822d0 = v1;
  v1[1] = v2;
  return;
}


// Function: sub_12f10 @ 0x12f10
unsigned long sub_12f10(int4 a0) // early-return
{
  int4 v1; // eax
  char v2 [56];
  
  if (read(a0,v2,0x2b) != 0x2b)
    return 0;
  v1 = memcmp(v2,"Signature: 8a477f597d28d172789f06886806bc55",0x2b);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_12fe0 @ 0x12fe0
void sub_12fe0(void)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  void *v3; // rax
  uint8 v4; // rcx
  unsigned long *v5;
  
  v2 = (unsigned long *)sub_d360();
  *v2 = 0;
  v2[0x3f] = 0;
  v4 = (uint8)(((int4)v2 - (int4)(unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
  v5 = (unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8);
  while (v4) {
    v1 = &v5[1];
    *v5 = 0;
    v4 -= 1;
    v5 = v1;
  }
  sub_ccd0(v2);
  v3 = (void *)sub_d360();
  memset(v3,0,sub_cd10(v3));
  sub_ccd0(v3); // tail-call
  return;
}


// Function: sub_13050 @ 0x13050
unsigned long * sub_13050(int8 a0,uint8 a1,uint8 a2)
{
  unsigned long *v1;
  unsigned long *v10;
  char v2;
  unsigned long *v3; // rax
  int8 v4; // rax
  char *v5;
  char *v6; // rax
  char *v7;
  uint8 v8;
  uint8 v9; // rcx
  
  v3 = (unsigned long *)sub_d360();
  *v3 = 0;
  v3[0x3f] = 0;
  v4 = 0;
  v8 = (uint8)(((int4)v3 - (int4)(unsigned long *)((uint8)&v3[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
  v10 = (unsigned long *)((uint8)&v3[1] & 0xfffffffffffffff8);
  while (v8) {
    v1 = &v10[1];
    *v10 = 0;
    v8 -= 1;
    v10 = v1;
  }
  do {
    v2 = *(char *)(a0 + v4);
    *(char *)((int8)v3 + v4) = v2;
    if (!v2) break;
    v4 += 1;
  } while (v4 != 100);
  if (dat_82ba4 == 2)
    *(char *)((int8)v3 + 99) = 0;
  sub_12680(a1 >> 0x3f,a1,8,0,(char *)((int8)v3 + 0x7c),0xc,"off_t");
  v8 = 0x1ffffffff;
  if (a2 <= 0x1ffffffff)
    v8 = a2;
  if ((int8)a2 <= -1)
    v8 = 0;
  sub_12680(0,v8,8,0,&v3[0x11],0xc,"time_t");
  if (((uint4)(dat_82ba4 - 3U) < 2) || (v8 = 0x81a4, dat_82ba4 == 6))
    v8 = 0x1a4;
  *(char *)((int8)v3 + 0x6b) = 0;
  v5 = (char *)((int8)v3 + 0x6a);
  do {
    v6 = &v5[-1];
    v9 = v8 >> 3;
    *v5 = ((uint1)v8 & 7) + 0x30;
    v5 = v6;
    v8 = v9;
  } while ((char *)((int8)v3 + 99) != v6);
  *(char *)((int8)v3 + 0x73) = 0;
  v7 = (char *)((int8)v3 + 0x72);
  do {
    *v7 = 0x30;
    v7 = &v7[-1];
  } while (v7 != (char *)((int8)v3 + 0x6b));
  *(char *)((int8)v3 + 0x7b) = 0;
  v7 = (char *)((int8)v3 + 0x7a);
  do {
    *v7 = 0x30;
    v7 = &v7[-1];
  } while (v7 != (char *)((int8)v3 + 0x73));
  *(unsigned int *)((int8)v3 + 0x101) = 0x61747375;
  *(unsigned short *)((int8)v3 + 0x105) = 0x72;
  *(unsigned short *)((int8)v3 + 0x107) = 0x3030;
  return v3;
}


// Function: sub_13230 @ 0x13230
void sub_13230(uint1 *a0)
{
  uint1 v1;
  uint1 *v2;
  uint1 *v3; // rax
  int4 v4; // edx
  uint8 v5;
  uint8 v6; // r10
  
  v4 = 0;
  a0[0x94] = 0x20;
  a0[0x95] = 0x20;
  a0[0x96] = 0x20;
  a0[0x97] = 0x20;
  a0[0x98] = 0x20;
  a0[0x99] = 0x20;
  a0[0x9a] = 0x20;
  a0[0x9b] = 0x20;
  v2 = a0;
  do {
    v1 = *v2;
    v2 = &v2[1];
    v4 += (uint4)v1;
  } while (v2 != &a0[0x200]);
  v5 = (uint8)v4;
  if (0x40000 <= v4) {
    if ((dat_82ba4 & 0xfffffffb) == 2) {
      a0[0x94] = 0x80;
      v2 = &a0[0x9a];
      do {
        *v2 = (uint1)v5;
        v2 = &v2[-1];
        v5 >>= 8;
      } while (v2 != &a0[0x94]);
      sub_ccd0(a0); // tail-call
      return;
    }
    sub_123c0(0,0,v5,8,NULL,&a0[0x94],7);
    sub_ccd0(a0); // tail-call
    return;
  }
  a0[0x9a] = 0;
  v2 = &a0[0x99];
  do {
    v3 = &v2[-1];
    v6 = v5 >> 3;
    *v2 = ((uint1)v5 & 7) + 0x30;
    v2 = v3;
    v5 = v6;
  } while (&a0[0x93] != v3);
  sub_ccd0(a0); // tail-call
  return;
}


// Function: sub_13320 @ 0x13320
void sub_13320(int8 a0,int8 a1,int8 a2)
{
  uint4 v1; // eax
  
  if (((dat_82a54) && (*(uint1 *)(a1 + 0x9c) != 0x4b)) && ((v1 = (uint4)*(uint1 *)(a1 + 0x9c) - 0x4c, 0x2d <= (uint1)v1 || (!(0x100008000001U >> ((uint8)v1 & 0x3f) & 1))))) {
    dat_82400 = dat_82ba4;
    sub_23bc0(a0,a1,a2);
  }
  if ((!*(int8 *)(a0 + 0x178)) && (*(int8 *)(a0 + 0x168))) {
    sub_13230((uint1 *)sub_12c70(0,a0,a1)); // tail-call
    return;
  }
  sub_13230(a1); // tail-call
  return;
}


// Function: sub_133d0 @ 0x133d0
void sub_133d0(int8 a0,char *a1,char a2)
{
  char v1;
  uint8 v2;
  int8 v3; // rax
  int8 v4; // rax
  void *v5; // rax
  uint8 v6; // rax
  uint8 v7;
  char *v8;
  
  v2 = strlen(a1);
  v7 = v2 + 1;
  v3 = sub_13050("././@LongLink",v7,0);
  if (!dat_82ad2) {
    if (!dat_822a8) {
      sub_27eb0(0,(unsigned long *)0x822a8);
      sub_27f40(0,(unsigned long *)0x822a0);
    }
    v4 = 0;
    do {
      v1 = *(char *)(dat_822a8 + v4);
      *(char *)(v3 + 0x109 + v4) = v1;
      if (!v1) break;
      v4 += 1;
    } while (v4 != 0x20);
    *(char *)(v3 + 0x128) = 0;
    v4 = 0;
    do {
      v1 = *(char *)(dat_822a0 + v4);
      *(char *)(v3 + 0x129 + v4) = v1;
      if (!v1) break;
      v4 += 1;
    } while (v4 != 0x20);
    *(char *)(v3 + 0x148) = 0;
  }
  *(char *)(v3 + 0x9c) = a2;
  *(unsigned long *)(v3 + 0x101) = 0x20207261747375;
  sub_13320(a0,v3,-1);
  v5 = (void *)sub_d360();
  v6 = sub_cd10(v5);
  v8 = a1;
  if (v6 < v7) {
    do {
      a1 = &v8[v6];
      memcpy(v5,v8,v6);
      v7 -= v6;
      sub_ccd0((int8)v5 + (v6 - 1 & 0xfffffffffffffe00));
      v5 = (void *)sub_d360();
      v6 = sub_cd10(v5);
      v8 = a1;
    } while (v6 < v7);
    v2 = v7 - 1;
  }
  memcpy(v5,a1,v7);
  memset((void *)((int8)v5 + v7),0,v6 - v7);
  sub_ccd0((int8)v5 + (v2 & 0xfffffffffffffe00)); // tail-call
  return;
}


// Function: sub_13570 @ 0x13570
char * sub_13570(int8 a0)
{
  char *v1;
  int8 v10; // rax
  unsigned long v11; // rax
  uint8 v12; // rax
  char *v13;
  int8 v14; // stack - 0x58
  int8 v15; // stack - 0x50
  uint8 v16; // stack - 0x48
  uint4 v17;
  uint4 v18; // edi
  unsigned long *v19;
  unsigned long *v2;
  unsigned long v20; // r8
  uint4 v21; // r13d
  char *v22; // stack - 0x60
  int8 v23; // stack - 0x40
  char v3;
  uint4 v4;
  uint8 v5;
  char *v6; // rax
  uint8 v7;
  char *v8;
  int8 v9;
  
  v8 = *(char **)(a0 + 8);
  v14 = 0;
  v15 = 0;
  if (dat_82ba4 != 4) { // branch-flip
label_135b7:
    v4 = dat_82ba4;
    v5 = strlen(v8);
    if (v4 != 2) { // branch-flip
      if (0x65 <= v5) {
        switch(v4) {
          default:
            goto label_13e25;
          case 1:
            if (dat_82d50) {
              (*dat_82d50)();
              v8 = *(char **)(a0 + 8);
            }
            v11 = sub_4ce40(v8);
            v8 = dcgettext(NULL,"%s: file name is too long (max %d); not dumped",5);
            v20 = 99;
            error(0,0,v8,v11,v20); // return-dupe
            dat_82d58 = 2;
            return NULL;
          case 3:
          case 5:
            if (0x101 <= v5) {
              if (dat_82d50)
                (*dat_82d50)();
              v11 = sub_4ce40(v8);
              v8 = dcgettext(NULL,"%s: file name is too long (max %d); not dumped",5);
              v20 = 0x100;
              error(0,0,v8,v11,v20);
              dat_82d58 = 2;
              return NULL;
            }
            if (0x9d <= v5) { // branch-flip
              v3 = v8[0x9b];
              v7 = 0x9b;
            }
            else {
              v7 = v5 - 1;
              if (v8[v5 - 1] != '/') goto label_13d8f;
              v3 = v8[v5 - 2];
              v7 = v5 - 2;
            }
            while (v3 != '/') {
label_13d8f:
              v7 -= 1;
              if (!v7) goto label_13d95;
              v3 = v8[v7];
            }
            v5 = (v5 - 1) - v7;
            if (100 <= v5 - 1) {
label_13d95:
              if (dat_82d50)
                (*dat_82d50)();
              v11 = sub_4ce40(v8);
              error(0,0,dcgettext(NULL,"%s: file name is too long (cannot be split); not dumped",5),v11);
              dat_82d58 = 2;
              return NULL;
            }
            v6 = (char *)sub_d360();
            v6[0] = '\0';
            v6[1] = '\0';
            v6[2] = '\0';
            v6[3] = '\0';
            v6[4] = '\0';
            v6[5] = '\0';
            v6[6] = '\0';
            v6[7] = '\0';
            v6[0x1f8] = '\0';
            v6[0x1f9] = '\0';
            v6[0x1fa] = '\0';
            v6[0x1fb] = '\0';
            v6[0x1fc] = '\0';
            v6[0x1fd] = '\0';
            v6[0x1fe] = '\0';
            v6[0x1ff] = '\0';
            v19 = (unsigned long *)((uint8)&v6[8] & 0xfffffffffffffff8);
            v12 = (uint8)(((int4)v6 - (int4)v19) + 0x200U >> 3);
            while (v12) {
              v2 = &v19[1];
              *v19 = 0;
              v12 -= 1;
              v19 = v2;
            }
            v13 = &v6[0x159];
            v4 = (uint4)v7;
            v12 = v7 & 0xffffffff;
            if (8 <= v4) { // branch-flip
              *(unsigned long *)&v6[0x159] = *(unsigned long *)v8;
              *(unsigned long *)&v6[(v7 & 0xffffffff) + 0x151] = *(unsigned long *)&v8[(v7 & 0xffffffff) - 8];
              v9 = (int8)v13 - (int8)(unsigned long *)((uint8)&v6[0x161] & 0xfffffffffffffff8);
              v12 = (uint8)(v4 + (int4)v9 >> 3);
              v13 = &v8[-v9];
              v19 = (unsigned long *)((uint8)&v6[0x161] & 0xfffffffffffffff8);
              while (v12) {
                v2 = &v19[1];
                v1 = &v13[8];
                *v19 = *(unsigned long *)v13;
                v12 -= 1;
                v13 = v1;
                v19 = v2;
              }
            }
            else if (v7 & 4) { // branch-flip
              *(unsigned int *)v13 = *(unsigned int *)v8;
              *(unsigned int *)&v6[v12 + 0x155] = *(unsigned int *)&v8[v12 - 4];
            }
            else if ((v4) && (*v13 = *v8, v7 & 2))
              *(unsigned short *)&v6[v12 + 0x157] = *(unsigned short *)&v8[v12 - 2];
            v8 = &v8[v7 + 1];
            if (8 <= v5) { // branch-flip
              v19 = (unsigned long *)((uint8)&v6[8] & 0xfffffffffffffff8);
              *(unsigned long *)v6 = *(unsigned long *)v8;
              *(unsigned long *)&v6[v5 - 8] = *(unsigned long *)&v8[v5 - 8];
              v9 = (int8)v6 - (int8)v19;
              v5 = v5 + v9 >> 3;
              v8 = &v8[-v9];
              while (v5) {
                v2 = &v19[1];
                v13 = &v8[8];
                *v19 = *(unsigned long *)v8;
                v5 -= 1;
                v8 = v13;
                v19 = v2;
              }
            }
            else if (v5 & 4) { // branch-flip
              *(unsigned int *)v6 = *(unsigned int *)v8;
              *(unsigned int *)&v6[v5 - 4] = *(unsigned int *)&v8[v5 - 4];
            }
            else if ((v5) && (*v6 = *v8, v5 & 2))
              *(unsigned short *)&v6[v5 - 2] = *(unsigned short *)&v8[v5 - 2];
            goto label_13605;
          case 4:
            sub_1da70("path",a0,0);
            break;
          case 6:
            goto label_13d38;
          
        }
      }
    }
    else if (100 <= v5) {
label_13d38:
      sub_133d0(a0,v8,0x4c);
    }
    v6 = (char *)sub_12da0(a0);
  }
  else {
    if (sub_31a80(v8)) {
      v8 = *(char **)(a0 + 8);
      goto label_135b7;
    }
    sub_1da70("path",a0,0);
    v6 = (char *)sub_12da0(a0);
  }
label_13605:
  if (!v6)
    return NULL;
  sub_24e10(*(unsigned int *)(a0 + 0x74),(int4 *)(a0 + 0x74),&v14);
  v22 = (char *)0x1362e;
  sub_24ee0(*(unsigned int *)(a0 + 0x78),(int4 *)(a0 + 0x78),&v15);
  if (dat_82b18) { // branch-flip
    v17 = *(uint4 *)(a0 + 0x70);
    v22 = (char *)0x13663;
    v4 = sub_45ce0(v17,(v17 & 0xf000) == 0x4000,dat_82b14,dat_82b18,NULL);
    v4 = v17 & 0xfffff000 | v4;
    *(uint4 *)(a0 + 0x70) = v4;
  }
  else {
    v4 = *(uint4 *)(a0 + 0x70);
  }
  if ((dat_82ba4 & 0xfffffffd) != 1) { // branch-flip
    if ((2 <= dat_82ba4 - 3) && (dat_82ba4 != 6)) {
      v5 = (uint8)v4;
      if (0x200000 <= v4) {
        if (dat_82ba4 != 2) { // branch-flip
          v22 = "mode_t";
          sub_123c0(0,0,v5,4,NULL,&v6[100],8);
        }
        else {
          v6[100] = '\x80';
          v8 = &v6[0x6b];
          do {
            *v8 = (char)v5;
            v8 = &v8[-1];
            v5 >>= 8;
          } while (&v6[100] != v8);
        }
        goto label_136b9;
      }
    }
    else {
      v5 = (uint8)(v4 & 0xfff);
    }
    v6[0x6b] = '\0';
    v8 = &v6[0x6a];
    do {
      v13 = &v8[-1];
      v7 = v5 >> 3;
      *v8 = ((uint1)v5 & 7) + 0x30;
      v8 = v13;
      v5 = v7;
    } while (&v6[99] != v13);
  }
  else {
    v6[0x6b] = '\0';
    v5 = (uint8)(v4 & 0xfff);
    v8 = &v6[0x6a];
    do {
      v13 = &v8[-1];
      v7 = v5 >> 3;
      *v8 = ((uint1)v5 & 7) + 0x30;
      v5 = v7;
      v8 = v13;
    } while (&v6[99] != v13);
  }
label_136b9:
  v4 = *(uint4 *)(a0 + 0x74);
  v5 = (uint8)v4;
  if ((dat_82ba4 != 4) || (v4 <= 0x1fffff)) {
    if (v4 <= 0x1fffff) goto label_13b83;
    if ((dat_82ba4 & 0xfffffffb) != 2) { // branch-flip
      v22 = "uid_t";
      if (!sub_123c0(0,0,v5,4,NULL,&v6[0x6c],8))
        return NULL;
    }
    else {
      v6[0x6c] = '\x80';
      v8 = &v6[0x73];
      do {
        *v8 = (char)v5;
        v8 = &v8[-1];
        v5 >>= 8;
      } while (&v6[0x6c] != v8);
    }
  }
  else {
    v22 = (char *)0x13b81;
    sub_1da70("uid",a0,0);
    v5 = 0;
label_13b83:
    v6[0x73] = '\0';
    v8 = &v6[0x72];
    do {
      v13 = &v8[-1];
      v7 = v5 >> 3;
      *v8 = ((uint1)v5 & 7) + 0x30;
      v8 = v13;
      v5 = v7;
    } while (&v6[0x6b] != v13);
  }
  v4 = *(uint4 *)(a0 + 0x78);
  if ((0x200000 <= v4) && (dat_82ba4 == 4)) {
    v22 = (char *)0x13bc9;
    sub_1da70("gid",a0,0);
    v5 = 0;
label_13bcb:
    v6[0x7b] = '\0';
    v8 = &v6[0x7a];
    do {
      v13 = &v8[-1];
      v7 = v5 >> 3;
      *v8 = ((uint1)v5 & 7) + 0x30;
      v8 = v13;
      v5 = v7;
    } while (&v6[0x73] != v13);
label_13761:
    v5 = *(uint8 *)(a0 + 0x88);
    if (dat_82ba4 != 4) goto label_13775;
label_13b42:
    v4 = 0;
    if (v5 >> 0x21) {
      v22 = (char *)0x13b62;
      sub_1da70("size",a0,0);
      v5 = 0;
    }
  }
  else {
    v5 = (uint8)v4;
    if (v4 <= 0x1fffff) goto label_13bcb;
    if ((dat_82ba4 & 0xfffffffb) == 2) {
      v6[0x74] = '\x80';
      v8 = &v6[0x7b];
      do {
        *v8 = (char)v5;
        v8 = &v8[-1];
        v5 >>= 8;
      } while (&v6[0x74] != v8);
      goto label_13761;
    }
    v22 = "gid_t";
    if (!sub_123c0(0,0,v5,4,NULL,&v6[0x74],8))
      return NULL;
    v5 = *(uint8 *)(a0 + 0x88);
    if (dat_82ba4 == 4) goto label_13b42;
label_13775:
    v4 = (uint4)(v5 >> 0x3f);
  }
  if (!sub_12680(v4,v5,8,0,&v6[0x7c],0xc,"off_t"))
    return NULL;
  if (dat_82af0 != 1) { // branch-flip
    if (dat_82af0 != 2) { // branch-flip
      if (!dat_82af0) {
        v16 = *(uint8 *)(a0 + 0xf8);
        v23 = *(int8 *)(a0 + 0x100);
      }
    }
    else if (1 <= (int4)(((uint4)(dat_82ae8 < *(int8 *)(a0 + 0x100)) - (uint4)(*(int8 *)(a0 + 0x100) < dat_82ae8)) + ((uint4)((int8)dat_82ae0 < *(int8 *)(a0 + 0xf8)) - (uint4)(*(int8 *)(a0 + 0xf8) < (int8)dat_82ae0)) * 2)) { // branch-flip
      v16 = dat_82ae0;
      v23 = dat_82ae8;
    }
    else {
      v16 = *(uint8 *)(a0 + 0xf8);
      v23 = *(int8 *)(a0 + 0x100);
    }
  }
  else {
    v16 = dat_82ae0;
    v23 = dat_82ae8;
  }
  v5 = v16;
  if (dat_82ba4 != 4) // branch-flip
    v7 = v16 >> 0x3f;
  else if ((v16 >> 0x21) || (v7 = 0, v23)) {
    v22 = (char *)0x13e5b;
    sub_1da70("mtime",a0,&v16);
    v7 = 0;
    v5 = v16;
    if (v16 >> 0x21) {
      v16 = 0;
      v5 = 0;
    }
  }
  v8 = "time_t";
  if (!sub_12680(v7,v5,8,0,&v6[0x88],0xc))
    return NULL;
  if ((*(uint4 *)(a0 + 0x70) & 0xb000) != 0x2000) { // branch-flip
    if ((dat_82ba4 & 0xfffffffb) != 2) {
      if (!sub_12680(0,0,4,0,&v6[0x149],8,"major_t"))
        return NULL;
      v3 = sub_12680(0,0,4,0,&v6[0x151],8,"minor_t");
      goto label_13d23;
    }
  }
  else {
    v5 = *(uint8 *)(a0 + 0x80);
    v17 = (uint4)(v5 >> 0x20);
    v4 = v17 & 0xfffff000 | (uint4)(v5 >> 8) & 0xfff;
    v18 = (uint4)((v5 >> 0x14) << 8);
    v21 = v18 | (uint4)(v5 & 0xff);
    if ((0x200000 <= v4) && (dat_82ba4 == 4)) {
      sub_1da70("devmajor",a0,0,v5 & 0xff,v8,v22);
      v17 = 0;
      v5 = 0;
    }
    else {
      v5 = (uint8)(int4)v4;
      v17 >>= 0x1f;
    }
    if (!sub_12680(v17,v5,4,0,&v6[0x149],8,"major_t"))
      return NULL;
    if ((dat_82ba4 != 4) || (v21 <= 0x1fffff)) {
      v5 = (uint8)(int4)v21;
      v18 >>= 0x1f;
    }
    else {
      sub_1da70("devminor",a0,0);
      v18 = 0;
      v5 = 0;
    }
    v3 = sub_12680(v18,v5,4,0,&v6[0x151],8,"minor_t");
label_13d23:
    if (!v3)
      return NULL;
  }
  if (dat_82ba4 != 4) { // branch-flip
    v4 = dat_82ba4;
    if ((dat_82b50) && ((dat_82ba4 & 0xfffffffb) == 2)) {
      sub_12680(*(uint8 *)(a0 + 0xe8) >> 0x3f,*(uint8 *)(a0 + 0xe8),8,0,&v6[0x159],0xc,"time_t");
      sub_12680(*(uint8 *)(a0 + 0x108) >> 0x3f,*(uint8 *)(a0 + 0x108),8,0,&v6[0x165],0xc,"time_t");
      v4 = dat_82ba4;
    }
  }
  else {
    sub_1da70("atime",a0,0);
    sub_1da70("ctime",a0,0);
    v4 = dat_82ba4;
  }
  if (v4 == 1) {
    v6[0x9c] = '\0';
    return v6;
  }
  v6[0x9c] = '0';
  if (5 <= v4) { // branch-flip
    if (v4 != 6) {
label_13e25:
      abort(); // no-return, return-dupe
    }
  }
  else {
    if (3 <= v4) {
      builtin_strncpy(&v6[0x101],"ustar",6);
      v6[0x107] = '0';
      v6[0x108] = '0';
      goto label_138b1;
    }
    if (v4 != 2) {
      abort();
    }
  }
  builtin_strncpy(&v6[0x101],"ustar  ",8);
label_138b1:
  if (!dat_82ad2) {
    if (v14) // branch-flip
      *(unsigned long *)(a0 + 0x20) = sub_4ee30(v14);
    else {
      sub_27eb0(*(unsigned int *)(a0 + 0x74),(unsigned long *)(a0 + 0x20));
    }
    if (v15) // branch-flip
      *(unsigned long *)(a0 + 0x28) = sub_4ee30(v15);
    else {
      sub_27f40(*(unsigned int *)(a0 + 0x78),(unsigned long *)(a0 + 0x28));
    }
    v8 = *(char **)(a0 + 0x20);
    if (dat_82ba4 == 4) {
      if ((0x21 <= strlen(v8)) || (!sub_31a80(v8))) {
        sub_1da70("uname",a0,0);
        v8 = *(char **)(a0 + 0x20);
      }
      else {
        v8 = *(char **)(a0 + 0x20);
      }
    }
    v9 = 0;
    do {
      v3 = v8[v9];
      v6[v9 + 0x109] = v3;
      if (!v3) break;
      v9 += 1;
    } while (v9 != 0x20);
    v6[0x128] = '\0';
    if (dat_82ba4 == 4) {
      v8 = *(char **)(a0 + 0x28);
      if ((0x21 <= strlen(v8)) || (!sub_31a80(v8)))
        sub_1da70("gname",a0,0);
    }
    v9 = *(int8 *)(a0 + 0x28);
    v10 = 0;
    do {
      v3 = *(char *)(v9 + v10);
      v6[v10 + 0x129] = v3;
      if (!v3) break;
      v10 += 1;
    } while (v10 != 0x20);
    v6[0x148] = '\0';
    v4 = dat_82ba4;
  }
  if (v4 != 4)
    return v6;
  if (1 <= dat_82a9c) {
    if (*(int8 *)(a0 + 0x38))
      sub_1da70("SCHILY.acl.access",a0,0);
    if (*(int8 *)(a0 + 0x48))
      sub_1da70("SCHILY.acl.default",a0,0);
  }
  if ((1 <= dat_82aa0) && (*(int8 *)(a0 + 0x30)))
    sub_1da70("RHT.security.selinux",a0,0);
  if (1 <= dat_82a98) {
    v5 = 0;
    v16 = 0;
    v9 = *(int8 *)(a0 + 0x160);
    if (*(int8 *)(a0 + 0x158)) {
      do {
        sub_1da70(*(char **)(v9 + v5 * 0x18),a0,&v16);
        v5 = v16 + 1;
        v16 = v5;
      } while (v5 < *(uint8 *)(a0 + 0x158));
      return v6;
    }
    return v6;
  }
  return v6;
}


// Function: sub_14390 @ 0x14390
void sub_14390(int8 a0)
{
  uint8 v1; // rax
  char *v2; // rax
  
  if (dat_82ba4 <= 6) {
    v1 = 1L << ((uint1)dat_82ba4 & 0x3f);
    if (v1 & 0x2a) {
      if (dat_82d50)
        (*dat_82d50)();
      v2 = (char *)sub_4ce40(*(char **)(a0 + 0x18));
      error(0,0,dcgettext(NULL,"%s: link name is too long; not dumped",5),v2);
      dat_82d58 = 2;
      return;
    }
    if (v1 & 0x44) {
      sub_133d0(a0,*(char **)(a0 + 0x18),0x4b); // tail-call
      return;
    }
    if (dat_82ba4 == 4) {
      sub_1da70("linkpath",a0,0); // tail-call
      return;
    }
  }
  abort(); // no-return
}


// Function: sub_14450 @ 0x14450
unsigned long sub_14450(int4 a0,struct_48 *a1) // return-dupe
{
  unsigned long *v1;
  uint8 v10; // rcx
  uint8 v11;
  char v12 [24];
  unsigned long *v13;
  uint4 v2; // eax
  int8 v3; // rax
  int8 v4;
  uint8 v5;
  uint8 v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long *v9;
  
  v11 = a1->field_0x88;
  v3 = sub_cc60();
  v4 = sub_13570(a1);
  if (!v4)
    return 2;
  sub_13320(a1,v4,v3);
  sub_c640(a1->field_0x8,a1->field_0x88,a1->field_0x88);
label_144b8:
  do {
    v10 = v11;
    if ((int8)v10 <= 0)
      return 0;
    v3 = sub_d360();
    v5 = sub_cd10(v3);
    if (v5 <= v10) { // branch-flip
      if (a0 <= 0) {
        sub_ccd0(v3 + (v5 - 1 & 0xfffffffffffffe00));
        v11 = v10 - v5;
        goto label_144b8;
      }
    }
    else {
      if (v10 & 0x1ff) {
        v2 = 0x200 - ((uint4)v10 & 0x1ff);
        v13 = (unsigned long *)(v3 + v10);
        if (8 <= v2) { // branch-flip
          *v13 = 0;
          *(unsigned long *)((int8)v13 + ((uint8)v2 - 8)) = 0;
          v11 = (uint8)(v2 + ((int4)v13 - (int4)(unsigned long *)((uint8)&v13[1] & 0xfffffffffffffff8)) >> 3);
          v13 = (unsigned long *)((uint8)&v13[1] & 0xfffffffffffffff8);
          while (v11) {
            v9 = &v13[1];
            *v13 = 0;
            v11 -= 1;
            v13 = v9;
          }
        }
        else if (v2 & 4) { // branch-flip
          *(unsigned int *)v13 = 0;
          *(unsigned int *)((int8)v13 + ((uint8)v2 - 4)) = 0;
        }
        else if ((v2) && (*(char *)v13 = 0, v2 & 2))
          *(unsigned short *)((int8)v13 + ((uint8)v2 - 2)) = 0;
      }
      if (a0 <= 0) {
        sub_ccd0(v3 + (v10 - 1 & 0xfffffffffffffe00));
        return 0;
      }
      v5 = v10;
    }
    v6 = sub_25fe0(a0,v3,v5);
    if (v6 == 0xffffffffffffffff) {
      sub_263a0(a1->field_0x0,a1->field_0x88 - v10,v5);
      do {
        v9 = (unsigned long *)sub_d360();
        v10 -= 0x200;
        *v9 = 0;
        v9[0x3f] = 0;
        v11 = (uint8)(((int4)v9 - (int4)(unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
        v13 = (unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8);
        while (v11) {
          v1 = &v13[1];
          *v13 = 0;
          v11 -= 1;
          v13 = v1;
        }
        sub_ccd0(v9);
      } while (0 < (int8)v10);
      return 1;
    }
    v11 = v10 - v6;
    sub_ccd0((v5 - 1 & 0xfffffffffffffe00) + v3);
    if (v6 != v5) {
      memset((void *)(v3 + v6),0,v5 - v6);
      if (dat_81b80 & 0x80) {
        if (dat_82d50)
          (*dat_82d50)();
        v7 = sub_45370(v11,v12);
        v8 = sub_4ce40(a1->field_0x0);
        error(0,0,dcngettext(NULL,"%s: File shrank by %s byte; padding with zeros","%s: File shrank by %s bytes; padding with zeros",v11,5),v8,v7);
      }
      if (!dat_82b52)
        sub_2fd00(1);
      v3 = v10 - v5;
      if (v3 <= 0)
        return 1;
      v11 = v3 - 1;
      v4 = v3 + -0x200;
      do {
        v9 = (unsigned long *)sub_d360();
        v3 -= 0x200;
        *v9 = 0;
        v9[0x3f] = 0;
        v10 = (uint8)(((int4)v9 - (int4)(unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
        v13 = (unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8);
        while (v10) {
          v1 = &v13[1];
          *v13 = 0;
          v10 -= 1;
          v13 = v1;
        }
        sub_ccd0(v9);
      } while (v3 != v4 - (v11 & 0xfffffffffffffe00));
      return 1;
    }
  } while( true );
}


// Function: sub_148b0 @ 0x148b0
void sub_148b0(void) // return-dupe
{
  int8 v1; // rax
  unsigned long v2; // rax
  
  if (!dat_822c0)
    return;
  v1 = sub_439d0(dat_822c0);
  if (!v1)
    return;
  do {
    if (*(int8 *)(v1 + 0x10)) {
      if (dat_82d50)
        (*dat_82d50)();
      v2 = sub_4d2d0((char *)(v1 + 0x18));
      error(0,0,dcgettext(NULL,"Missing links to %s.",5),v2);
    }
    v1 = sub_43a20(dat_822c0,v1);
  } while (v1);
  return;
}


// Function: sub_14950 @ 0x14950
int4 sub_14950(int8 a0,char *a1,int4 a2)
{
  int4 v1;
  
  if (!dat_82298) {
    dat_82298 = '\x01';
    strerror(2);
    dcgettext(NULL,"",5);
  }
  while( true ) {
    v1 = dat_7f234;
    if (a0)
      v1 = *(int4 *)(a0 + 0x1a8);
    v1 = __openat_2(v1,a1,a2);
    if (0 <= v1) break;
    if (!sub_12990(a0))
      return v1;
  }
  return v1;
}


// Function: sub_149e0 @ 0x149e0
unsigned int sub_149e0(int8 a0,unsigned long *a1) // return-dupe
{
  int8 *v1;
  code *v2;
  int4 v3; // eax
  uint8 v4; // rax
  unsigned long *v5;
  
  v5 = dat_822d0;
  if (!dat_822d0)
    return 0;
  do {
    v4 = sub_14950(a0,(char *)*v5,dat_82808);
    v3 = (int4)v4;
    if (0 <= v3) {
      v2 = (code *)v5[3];
      if ((!v2) || ((*v2)(v4 & 0xffffffff))) {
        close(v3);
        if (a1)
          *a1 = *v5;
        return *(unsigned int *)&v5[2];
      }
      close(v3);
    }
    v1 = &v5[4];
    v5 = (unsigned long *)*v1;
    if (!(unsigned long *)*v1)
      return 0;
  } while( true );
}


// Function: sub_14a80 @ 0x14a80
void sub_14a80(int8 a0)
{
  unsigned long *v1;
  int4 v2;
  int4 v3;
  stat v4; // stack - 0xb8
  
  v1 = *(unsigned long **)(a0 + 0x198);
  if ((v1) && (!*(int4 *)&v1[0x35])) {
    v2 = __openat_2(*(int4 *)(a0 + 0x1a8),"..",dat_82804);
    if (0 <= v2) { // branch-flip
      if (((fstat(v2,&v4)) || (v1[0xc] != v4._8_8_)) || (v1[0xb] != v4._0_8_)) {
        close(v2);
        *(unsigned int *)&v1[0x35] = 2;
        return;
      }
    }
    else {
      v3 = *__errno_location();
      v2 = -v3;
      if ((0 < v3) && (v3 = __openat_2(dat_7f234,(char *)*v1,dat_82804), 0 <= v3)) {
        if ((fstat(v2,&v4)) || ((v1[0xc] != v4._8_8_ || (v1[0xb] != v4._0_8_))))
          close(v3);
        else {
          v2 = v3;
        }
      }
    }
    *(int4 *)&v1[0x35] = v2;
  }
  return;
}


// Function: sub_14c50 @ 0x14c50
void sub_14c50(struct_53 *a0,char *a1,char *a2)
{
  uint8 v1;
  int4 v10;
  DIR *v11; // rax
  int8 v12;
  int8 v13; // rax
  char *v14;
  int8 v15;
  unsigned long v16;
  int8 v17;
  void *v18; // rax
  char *v19; // rax
  unsigned long *v2;
  uint8 v20;
  uint8 v21;
  code *v22; // rdx
  int8 v23;
  int8 v24;
  char v25;
  unsigned long v26; // stack - 0xe8
  stat v27; // stack - 0xd8
  char *v28; // stack - 0xe0
  unsigned long *v29;
  int8 v3;
  bool v30; // zf
  bool v31;
  uint4 v32; // stack - 0x128
  char *v33; // stack - 0x108
  uint8 v34; // stack - 0xf8
  unsigned long v4;
  int8 v5;
  bool v6;
  uint4 v7; // eax
  int4 v8;
  int4 v9; // eax
  
  v10 = dat_7f234;
  v15 = a0->field_0x198;
  v30 = v15 == 0;
  if (v30) {
    if (dat_82b40) { // branch-flip
      if (!sub_2df50("add",a2))
        return;
      sub_25120(a0,a2);
      sub_25120(&a0->field_0x8,(char *)sub_33640(a2,0,(uint1)dat_82b92));
      sub_30e70(&a0->field_0x8,1);
      v6 = 0;
    }
    else {
      sub_25120(a0,a2);
      sub_25120(&a0->field_0x8,(char *)sub_33640(a2,0,(uint1)dat_82b92));
      sub_30e70(&a0->field_0x8,1);
      v6 = 0;
    }
  }
  else {
    v10 = *(int4 *)(v15 + 0x1a8);
    if ((dat_82b40) && (!sub_2df50("add",a2)))
      return;
    sub_25120(a0,a2);
    sub_25120(&a0->field_0x8,(char *)sub_33640(a2,0,(uint1)dat_82b92));
    sub_30e70(&a0->field_0x8,1);
    if (v10 <= -1) {
      v22 = sub_26370;
      *__errno_location() = -v10;
      sub_26490(a2,v30,v22); // return-dupe, tail-call
      return;
    }
    v6 = 1;
  }
  if (fstatat(v10,a1,(stat *)&a0->field_0x58,dat_82800)) {
    v22 = sub_26460; // return-dupe
    sub_26490(a2,v30,v22);
    return;
  }
  v7 = a0->field_0x70 & 0xf000;
  if (v7 != 0x4000) { // branch-flip
    v8 = 0;
    if (v7 == 0x8000) {
      if (dat_82a30) { // branch-flip
        if ((dat_82a69) && (dat_82a88)) {
          v20 = a0->field_0x88;
          v21 = v20;
          if ((int8)v20 <= -1)
            v21 = v20 + 0x1ff;
          v23 = (int8)v21 >> 9;
          if (v20 & 0x1ff) {
            if (0x3ff <= v20 + 0x1ff)
              v23 += 1;
          }
          if (a0->field_0x98 < v23) goto label_14e5e;
        }
      }
      else if ((a0->field_0x88) || ((a0->field_0x70 & 0x124) != 0x124)) goto label_14e5e;
    }
  }
  else {
label_14e5e:
    v8 = sub_14950(v15,a1,dat_82808);
    if (v8 < 0) {
      v22 = sub_26370;
      sub_26490(a2,v30,v22);
      return;
    }
    a0->field_0x1a8 = v8;
    if (fstat(v8,(stat *)&a0->field_0x58)) {
      v22 = sub_26460;
      sub_26490(a2,v30,v22);
      return;
    }
  }
  v23 = a0->field_0x88;
  v24 = a0->field_0xc8;
  v17 = a0->field_0xb0;
  a0->field_0x118 = v23;
  a0->field_0xe8 = a0->field_0xa0;
  v12 = a0->field_0xb8;
  a0->field_0xf0 = a0->field_0xa8;
  v3 = a0->field_0xc0;
  a0->field_0xf8 = v17;
  a0->field_0x100 = v12;
  a0->field_0x108 = v3;
  a0->field_0x110 = v24;
  if (((((dat_82b50 != '\x01') || (v30)) && ((a0->field_0x70 & 0xf000) != 0x4000)) && ((int4)(((uint4)(dat_82b08 < v12) - (uint4)(v12 < dat_82b08)) + ((uint4)(dat_82b00 < v17) - (uint4)(v17 < dat_82b00)) * 2) <= -1)) && ((!dat_82b8c || ((int4)(((uint4)(dat_82b08 < v24) - (uint4)(v24 < dat_82b08)) + ((uint4)(dat_82b00 < v3) - (uint4)(v3 < dat_82b00)) * 2) <= -1)))) {
    if (dat_82b50)
      return;
    if (!dat_82a54)
      return;
    if (!(dat_81b81 & 1))
      return;
    if (dat_82d50)
      (*dat_82d50)();
    v16 = sub_4ce40(a2);
    v14 = "%s: file is unchanged; not dumped";
    error(0,0,dcgettext(NULL,v14,5),v16); // return-dupe, tail-call
    return;
  }
  if (sub_2bd80(a0)) {
    if (!(dat_81b81 & 4))
      return;
    if (dat_82d50)
      (*dat_82d50)();
    v16 = sub_4ce40(a2);
    v14 = "%s: file is the archive; not dumped";
    error(0,0,dcgettext(NULL,v14,5),v16);
    return;
  }
  v7 = a0->field_0x70 & 0xf000;
  v32 = a0->field_0x70 & 0xf000;
  if (v7 != 0x4000) { // branch-flip
    if ((dat_822c0) && ((a0->field_0x68 > dat_822c8 || (dat_82ab8)))) {
      v27._8_8_ = a0->field_0x60;
      v27._0_8_ = a0->field_0x58;
      v15 = sub_43960(dat_822c0,&v27);
      if (v15) {
        v14 = (char *)sub_33640((char *)(v15 + 0x18),1,(uint1)dat_82b92);
        if (*(int8 *)(v15 + 0x10))
          *(int8 *)(v15 + 0x10) = *(int8 *)(v15 + 0x10) + -1;
        v15 = sub_cc60();
        sub_25120(&a0->field_0x18,v14);
        v31 = dat_82ba4 != 2;
        if ((uint8)v31 + 99 < strlen(v14))
          sub_14390(a0);
        a0->field_0x88 = 0;
        v17 = sub_13570(a0);
        if (v17) {
          v23 = 0;
          do {
            v25 = v14[v23];
            *(char *)(v17 + 0x9d + v23) = v25;
            if (!v25) break;
            v23 += 1;
          } while (v23 != 100);
          *(char *)(v17 + 0x9c) = 0x31;
          sub_13320(a0,v17,v15);
          if (dat_82ab8) {
            sub_31150(a0->field_0x0,0);
            return;
          }
          return;
        }
      }
      v7 = a0->field_0x70 & 0xf000;
    }
    if (v7 != 0x8000) {
      if (v7 == 0xa000) {
        v14 = (char *)sub_3a040(v10,a1,a0->field_0x88);
        a0->field_0x18 = v14;
        if (v14) {
          sub_30e70(&a0->field_0x18,4);
          v30 = dat_82ba4 != 2;
          v14 = a0->field_0x18;
          if ((uint8)v30 + 99 < strlen(v14))
            sub_14390(a0);
          sub_32430(v10,a1,a0,0);
          sub_32100(v10,a1,a0,0);
          v15 = sub_cc60();
          a0->field_0x88 = 0;
          v23 = sub_13570(a0);
          if (!v23)
            return;
          v14 = a0->field_0x18;
          v24 = 0;
          do {
            v25 = v14[v24];
            *(char *)(v23 + 0x9d + v24) = v25;
            if (!v25) break;
            v24 += 1;
          } while (v24 != 100);
          *(char *)(v23 + 0x9c) = 0x32;
          sub_13320(a0,v23,v15);
          if (dat_82ab8)
            sub_31150(a2,0);
          sub_12b40(a0); // tail-call
          return;
        }
        if (*__errno_location() == 0xc)
          sub_17020(); // no-return
        v22 = sub_263d0;
        sub_26490(a2,v30,v22);
        return;
      }
      if (v7 != 0x2000) { // branch-flip
        if (v7 != 0x6000) { // branch-flip
          if (v7 != 0x1000) {
            if (v7 == 0xc000) {
              if (!(dat_81b80 & 0x20))
                return;
              if (dat_82d50)
                (*dat_82d50)();
              v16 = sub_4ce40(a2);
              v14 = "%s: socket ignored";
              error(0,0,dcgettext(NULL,v14,5),v16);
              return;
            }
            sub_12a20(a2); // return-dupe, tail-call
            return;
          }
          v25 = '6';
          sub_31ed0(v10,a1,a0,0,1);
          sub_32430(v10,a1,a0,0);
          sub_32100(v10,a1,a0,0);
        }
        else {
          v25 = '4';
          sub_31ed0(v10,a1,a0,0,1);
          sub_32430(v10,a1,a0,0);
          sub_32100(v10,a1,a0,0);
        }
      }
      else {
        v25 = '3';
        sub_31ed0(v10,a1,a0,0,1);
        sub_32430(v10,a1,a0,0);
        sub_32100(v10,a1,a0,0);
      }
      if (dat_82ba4 != 1) {
        v15 = sub_cc60();
        a0->field_0x88 = 0;
        v23 = sub_13570(a0);
        if (!v23)
          return;
        *(char *)(v23 + 0x9c) = v25;
        if (v25 != '6') {
          v7 = (uint4)((uint8)a0->field_0x80 >> 0x20);
          sub_12680(v7 >> 0x1f,(int8)(int4)(v7 & 0xfffff000 | (uint4)((uint8)a0->field_0x80 >> 8) & 0xfff),4,0,(char *)(v23 + 0x149),8,"major_t");
          v7 = (uint4)(((uint8)a0->field_0x80 >> 0x14) << 8);
          sub_12680(v7 >> 0x1f,(int8)(int4)(v7 | (uint4)a0->field_0x80 & 0xff),4,0,(char *)(v23 + 0x151),8,"minor_t");
        }
        sub_13320(a0,v23,v15);
        if (dat_82ab8) {
          sub_31150(a2,0); // tail-call
          return;
        }
        return;
      }
      sub_12a20(a2);
      return;
    }
    sub_31ed0(v10,a1,a0,0,1);
    sub_32430(v10,a1,a0,v8);
    sub_32100(v10,a1,a0,v8);
    if ((v8) && (dat_82a88)) {
      v20 = a0->field_0x88;
      v21 = v20;
      if ((int8)v20 <= -1)
        v21 = v20 + 0x1ff;
      v15 = (int8)v21 >> 9;
      if (v20 & 0x1ff) {
        if (0x3ff <= v20 + 0x1ff)
          v15 += 1;
      }
      if (v15 <= a0->field_0x98) goto label_15555;
      v7 = sub_2af80(v8,a0);
      if (v7 == 3) {
        v7 = sub_14450(v8,a0);
        goto label_1556c;
      }
      if (2 <= v7) {
        sub_2fb80(a0); // return-dupe
        return;
      }
    }
    else {
label_15555:
      v7 = sub_14450(v8,a0);
label_1556c:
      if (2 <= v7) {
        if (v7 == 3)
          abort(); // no-return
        sub_2fb80(a0);
        return;
      }
    }
    sub_12b40(a0);
    if (v7) {
      sub_2fb80(a0);
      return;
    }
label_1558b:
    if (v8) // branch-flip
      v9 = fstat(v8,&v27);
    else {
      if ((v10 <= -1) && (v6)) {
        *__errno_location() = -v10;
        goto label_151a8;
      }
      v9 = fstatat(v10,a1,&v27,dat_82800);
    }
    if (!v9) {
      if (((((uint4)(v24 < v27._112_8_) - (uint4)(v27._112_8_ < v24)) + ((uint4)(v3 < v27._104_8_) - (uint4)(v27._104_8_ < v3)) * 2) && ((dat_82ab8 != '\x01' || (v32 != 0x4000)))) || (v23 < v27._48_8_)) {
        if (dat_81b80 & 0x10) {
          if (dat_82d50)
            (*dat_82d50)();
          v14 = (char *)sub_4ce40(a2);
          error(0,0,dcgettext(NULL,"%s: file changed as we read it",5),v14);
        }
        sub_2fd00(1);
      }
      else if ((((dat_82b88 == 1) && (v8)) && ((v23 || (v32 == 0x4000)))) && (v16 = a0->field_0xe8, v4 = a0->field_0xf0, sub_26100(v8,v10,a1,v16,v4)))
        sub_334b0(a2);
      if (sub_2fb80(a0)) {
        if (dat_82ab8) {
          sub_31150(a2,v32 == 0x4000);
          return;
        }
        return;
      }
      return;
    }
  }
  else {
    sub_31ed0(v10,a1,a0,0,0);
    sub_32430(v10,a1,a0,v8);
    sub_32100(v10,a1,a0,v8);
    sub_12ac0(a0);
    sub_12ac0(&a0->field_0x8);
    if (sub_149e0(a0,&v26) == 3) {
      v14 = dcgettext(NULL,"directory not dumped",5);
      sub_12e70(a0->field_0x0,v26,v14); // tail-call
      return;
    }
    do {
      v11 = fdopendir(a0->field_0x1a8);
      a0->field_0x1a0 = v11;
      if (v11) {
        v20 = (uint8)dat_827e4;
        v33 = (char *)sub_4d730(v11);
        if (v33) {
          v17 = a0->field_0x198;
          v12 = sub_cc60();
          a0->field_0x88 = 0;
          v13 = sub_13570(a0);
          if (!v13) goto label_15153;
          sub_17440(a0);
          if (dat_82b50) { // branch-flip
            v5 = *(int8 *)(dat_82620 + 0x38);
            if (dat_82ba4 != 4) { // branch-flip
              *(char *)(v13 + 0x9c) = 0x44;
              if (v5) {
                v17 = sub_cc60();
                v14 = (char *)sub_1ff00(*(int8 *)(dat_82620 + 0x38));
                v21 = sub_1f690(v14);
                sub_12680(v21 >> 0x3f,v21,8,0,(char *)(v13 + 0x7c),0xc,"off_t",v20);
                sub_13320(a0,v13,v17);
                sub_c640(a0->field_0x8,v21,v21);
                if ((int8)v21 > 0) goto label_15cf7;
                goto label_15153;
              }
            }
            else {
              *(char *)(v13 + 0x9c) = 0x35;
              if (v5) {
                sub_1da70("GNU.dumpdir",a0,sub_1ff00(v5));
                sub_13320(a0,v13,v12);
                goto label_15153;
              }
            }
          }
          else {
            *(char *)(v13 + 0x9c) = 0x35;
            sub_13320(a0,v13,v12);
          }
          if (dat_82ad4) {
            if (((dat_82ad1) && (v17)) && (*(int8 *)(a0->field_0x198 + 0x58) != a0->field_0x58)) {
              if ((dat_82a54) && (dat_81b82 & 4)) {
                if (dat_82d50)
                  (*dat_82d50)();
                v14 = (char *)sub_4ce40(a0->field_0x0);
                error(0,0,dcgettext(NULL,"%s: file is on a different filesystem; not dumped",5),v14);
              }
            }
            else {
              v10 = sub_149e0(a0,&v28);
              if (v10 != 1) { // branch-flip
                if (v10 != 2) { // branch-flip
                  if (!v10) {
                    v19 = (char *)sub_4ee30(a0->field_0x0);
                    v20 = strlen(v19);
                    v34 = v20;
                    v14 = v33;
                    while (v21 = strlen(v14), v21) {
                      v1 = v20 + v21;
                      if (v34 < v1) {
                        v19 = (char *)sub_4ece0(v19,v1 + 1);
                        v34 = v1;
                      }
                      strcpy(&v19[v20],v14);
                      if (!sub_176d0(v19,a0))
                        sub_14bb0(a0,v14,v19);
                      v14 = &v14[v21 + 1];
                    }
                    free(v19);
                  }
                }
                else {
                  v14 = dcgettext(NULL,"contents not dumped",5);
                  sub_12e70(a0->field_0x0,v28,v14);
                }
              }
              else {
                v14 = dcgettext(NULL,"contents not dumped",5);
                sub_12e70(a0->field_0x0,v28,v14);
                v20 = strlen(a0->field_0x0);
                v14 = (char *)sub_4ecc0(v20 + 1 + strlen(v28));
                strcpy(v14,a0->field_0x0);
                strcat(v14,v28);
                sub_14bb0(a0,v28,v14);
                free(v14);
              }
            }
          }
          goto label_15153;
        }
        break;
      }
    } while (sub_12990(a0));
    sub_26400(a0->field_0x0);
    v31 = 0;
label_1516b:
    v8 = a0->field_0x1a8;
    v10 = dat_7f234;
    if (v15)
      v10 = *(int4 *)(v15 + 0x1a8);
    if (!v31) {
      sub_2fb80(a0);
      return;
    }
    if (0 <= v8) goto label_1558b;
    *__errno_location() = -v8;
  }
label_151a8:
  sub_26490(a2,v30,sub_26460);
  sub_2fb80(a0);
  return;
label_15cf7:
  v18 = (void *)sub_d360();
  v20 = sub_cd10(v18);
  if (v20 <= v21) goto label_15d5e;
  if (v21 & 0x1ff) {
    v7 = 0x200 - ((uint4)v21 & 0x1ff);
    v29 = (unsigned long *)((int8)v18 + v21);
    if (8 <= v7) { // branch-flip
      *v29 = 0;
      *(unsigned long *)((int8)v29 + ((uint8)v7 - 8)) = 0;
      v20 = (uint8)(v7 + ((int4)v29 - (int4)(unsigned long *)((uint8)&v29[1] & 0xfffffffffffffff8)) >> 3);
      v29 = (unsigned long *)((uint8)&v29[1] & 0xfffffffffffffff8);
      while (v20) {
        v2 = &v29[1];
        *v29 = 0;
        v20 -= 1;
        v29 = v2;
      }
    }
    else if (v7 & 4) { // branch-flip
      *(unsigned int *)v29 = 0;
      *(unsigned int *)((int8)v29 + ((uint8)v7 - 4)) = 0;
    }
    else if ((v7) && (*(char *)v29 = 0, v7 & 2))
      *(unsigned short *)((int8)v29 + ((uint8)v7 - 2)) = 0;
  }
  memcpy(v18,v14,v21);
  sub_ccd0((int8)v18 + (v21 - 1 & 0xfffffffffffffe00));
label_15153:
  sub_14a80(a0);
  free(v33);
  v31 = 1;
  goto label_1516b;
label_15d5e:
  v21 -= v20;
  memcpy(v18,v14,v20);
  v14 = &v14[v20];
  sub_ccd0((int8)v18 + (v20 - 1 & 0xfffffffffffffe00));
  if ((int8)v21 <= 0) goto label_15153;
  goto label_15cf7;
}


// Function: sub_16020 @ 0x16020
void sub_16020(void)
{
  char v1;
  uint8 v10; // stack - 0x3c0
  stat v11 [2];
  uint8 v12; // stack - 0x3e0
  int4 v13; // stack - 0x210
  int8 *v14; // stack - 0x60
  char *v2;
  int4 v3; // eax
  int8 v4; // rax
  char *v5;
  uint8 v6; // rax
  char *v7;
  char v8 [408];
  int8 v9 [11]; // stack - 0x3b8
  
  dat_822c8 = 0;
  if (dat_82630 != 2)
    dat_822c8 = (uint8)(dat_82b69 ^ 1);
  sub_f710(1);
  sub_c660();
  if (dat_82b50) { // branch-flip
    v10 = 0;
    sub_28cf0();
    while (v4 = sub_29260(), v4) {
      v5 = *(char **)(v4 + 0x10);
      if (!sub_176d0(v5,0)) {
        v5 = *(char **)(v4 + 0x10);
        sub_2fb40(v8);
        v14 = NULL;
        sub_14c50(v8,v5,v5);
        sub_2fc00(v8);
      }
    }
    sub_292e0();
    v5 = NULL;
    while (v4 = sub_29260(), v4) {
      v7 = *(char **)(v4 + 0x10);
      if (!sub_176d0(v7,0)) {
        v7 = *(char **)(v4 + 0x10);
        v12 = strlen(v7);
        if (v10 <= v12) {
          do {
            v5 = (char *)sub_4ed20(v5,&v10);
          } while (v10 <= v12);
          v7 = *(char **)(v4 + 0x10);
        }
        memcpy(v5,v7,v12);
        if (v5[v12 - 1] != '/') {
          v6 = v12 + 1;
          v5[v12] = '/';
          v12 = v6;
        }
        sub_2fb40(v9);
        v7 = (char *)sub_1fed0(*(int8 *)(v4 + 0x38));
        if (v7) {
          for (; v1 = *v7, v1; v7 = &v7[v6 + 1]) {
            v6 = strlen(v7);
            if (v1 == 'Y') {
              if (!v9[0]) {
                v3 = __openat_2(dat_7f234,*(char **)(v4 + 0x10),dat_82804);
                if (v3 < 0) {
                  sub_26490(*(char **)(v4 + 0x10),*(int8 *)(v4 + 0x40) == 0,sub_26370);
                  break;
                }
                v13 = v3;
                if (fstat(v3,v11)) {
                  sub_26490(*(char **)(v4 + 0x10),*(int8 *)(v4 + 0x40) == 0,sub_26460);
                  break;
                }
                v2 = *(char **)(v4 + 0x10);
                v9[0] = sub_4ee30(v2);
              }
              while (v10 < v12 + v6) {
                v5 = (char *)sub_4ed20(v5,&v10);
              }
              strcpy(&v5[v12],&v7[1]);
              sub_2fb40(v8);
              v14 = v9;
              sub_14c50(v8,&v7[1],v5);
              if (dat_82b28)
                sub_1f780(v9);
              sub_2fc00(v8);
            }
          }
        }
        sub_2fc00(v9);
      }
    }
    free(v5);
  }
  else {
    while (v5 = (char *)sub_281f0(1), v5) {
      if (!sub_176d0(v5,0)) {
        sub_2fb40(v8);
        v14 = NULL;
        sub_14c50(v8,v5,v5);
        sub_2fc00(v8);
      }
    }
  }
  sub_12fe0();
  sub_d8d0();
  sub_31110();
  if (dat_82b28) {
    sub_20e40(); // tail-call
    return;
  }
  return;
}


// Function: sub_16620 @ 0x16620
void sub_16620(int4 a0)
{
  unsigned long v1;
  int8 v2; // rdi
  
  v1 = dat_81f58;
  dat_81f58 = dat_822e8;
  if (dat_822e0) { // branch-flip
    dat_82a34 = 1;
    sub_f6f0();
    dat_82a34 = 0;
  }
  else {
    v2 = (dat_822d8 + dat_81f30) - dat_81f38;
    if (v2)
      sub_16410(v2);
    sub_f6f0();
  }
  dat_81f58 = v1;
  if (((a0) && (!dat_822e0)) && (v2 = dat_81f38 - (dat_822d8 + dat_81f30), v2))
    sub_16410(v2);
  dat_822e4 = 0;
  return;
}


// Function: sub_166d0 @ 0x166d0
void sub_166d0(int8 a0,int8 a1)
{
  unsigned long *v1;
  unsigned long *v2;
  int8 v3; // rcx
  unsigned long *v4;
  uint8 v5; // rcx
  int8 v6; // rbx
  unsigned long *v7;
  
  if (!a1)
    return;
  v6 = 0;
  do {
    while( true ) {
      v3 = (int8)dat_822e4;
      v7 = (unsigned long *)(v6 * 0x200 + a0);
      dat_822e4 += 1;
      v4 = (unsigned long *)(v3 * 0x200 + dat_822e8);
      *v4 = *v7;
      v4[0x3f] = v7[0x3f];
      v3 = (int8)v4 - (int8)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      v5 = (uint8)((int4)v3 + 0x200U >> 3);
      v7 = (unsigned long *)((int8)v7 - v3);
      v4 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      while (v5) {
        v2 = &v4[1];
        v1 = &v7[1];
        *v4 = *v7;
        v5 -= 1;
        v7 = v1;
        v4 = v2;
      }
      if (dat_822e4 == dat_82ba0) break;
      v6 += 1;
      if (a1 == v6)
        return;
    }
    v6 += 1;
    sub_16620(1);
  } while (a1 != v6);
  return;
}


// Function: sub_16fd0 @ 0x16fd0
void sub_16fd0(void)
{
  if (dat_822f0)
    (*dat_822f0)();
  error(2,0,dcgettext(NULL,"Error is not recoverable: exiting now",5));
  return;
}


// Function: sub_17020 @ 0x17020
void sub_17020(void)
{
  error(0,0,"%s",dcgettext(NULL,"memory exhausted",5));
  sub_16fd0(); // no-return
}


// Function: sub_17060 @ 0x17060
void sub_17060(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x822f8;
  *a0 = 0x8000000;
  return;
}


// Function: sub_17080 @ 0x17080
void sub_17080(struct_65 *a0,char *a1,uint4 a2)
{
  char v1;
  int8 *v2; // rax
  
  v2 = __ctype_b_loc();
  while (v1 = *a1, *(uint1 *)(*v2 + 1 + (int8)v1 * 2) & 0x20) {
    a1 = &a1[1];
  }
  if ((v1) && (v1 != '#')) {
    if (v1 == '!') {
      if (a1[1] != '!') { // branch-flip
        a1 = &a1[1];
        a2 |= 0x20000000;
      }
      else {
        a1 = &a1[2];
      }
    }
    if (!strncmp(a1,"RE:",3)) {
      a1 = &a1[3];
      a2 = a2 & 0xefffffff | 0x8000000;
    }
    sub_40c90(a0,a1,a2); // tail-call
    return;
  }
  return;
}


// Function: sub_17130 @ 0x17130
void sub_17130(struct_65 *a0,char *a1,unsigned int a2)
{
  char v1;
  int8 *v2; // rax
  
  v2 = __ctype_b_loc();
  while( true ) {
    v1 = *a1;
    if (!(*(uint1 *)(*v2 + 1 + (int8)v1 * 2) & 0x20)) break;
    a1 = &a1[1];
  }
  if ((v1) && (v1 != '#')) {
    if (v1 == '\\')
      a1 = &a1[a1[1] == '#'];
    sub_40c90(a0,a1,a2); // tail-call
    return;
  }
  return;
}


// Function: sub_171b0 @ 0x171b0
void sub_171b0(unsigned long a0,char *a1,uint4 a2,uint4 *a3)
{
  char *v1;
  char v2;
  uint1 v3;
  int8 v4;
  uint4 v5;
  uint8 v6; // rax
  char *v7;
  
  v4 = *(int8 *)__ctype_b_loc();
  while (v2 = *a1, *(uint1 *)(v4 + 1 + (int8)v2 * 2) & 0x20) {
    a1 = &a1[1];
  }
  if ((v2) && (v2 != '#')) {
    if (strncmp(a1,"syntax:",7)) {
      v6 = strlen(a1);
      v7 = a1;
      if (a1[v6 - 1] == '/') {
        a2 |= 0x4000008;
        v7 = (char *)sub_4ecc0(v6);
        memcpy(v7,a1,v6 - 1);
        v7[v6 - 1] = '\0';
        sub_40820(a0,v7);
      }
      v5 = a2 & 0xf7ffffff;
      if (*a3 == 0x8000000)
        v5 = a2 & 0xefffffff;
      sub_40c90(a0,v7,*a3 | v5); // tail-call
      return;
    }
    v7 = &a1[7];
    v3 = *(uint1 *)(v4 + 1 + (int8)a1[7] * 2);
    while (v3 & 0x20) {
      v1 = &v7[1];
      v7 = &v7[1];
      v3 = *(uint1 *)(v4 + 1 + (int8)*v1 * 2);
    }
    if (strcmp(v7,"regexp")) { // branch-flip
      if (!strcmp(v7,"glob"))
        *a3 = 0x10000000;
    }
    else {
      *a3 = 0x8000000;
    }
  }
  return;
}


// Function: sub_17330 @ 0x17330
void sub_17330(struct_65 *a0,char *a1,unsigned int a2) // return-dupe
{
  int8 v1;
  uint8 v2; // rbx
  uint8 v3; // stack - 0x118
  int8 v4; // stack - 0x110
  
  if (sub_375a0(a1,&v3,0x844))
    return;
  if (v3) {
    v2 = 0;
    do {
      v1 = v2 * 8;
      v2 += 1;
      sub_40c90(a0,*(char **)(v4 + v1),a2);
    } while (v2 < v3);
  }
  sub_35800(&v3);
  return;
}


// Function: sub_173d0 @ 0x173d0
void sub_173d0(char *a0,unsigned int a1)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_4ecc0(strlen(a0) + 0x10);
  *v1 = 0;
  *(unsigned int *)&v1[1] = a1;
  strcpy((char *)((int8)v1 + 0xc),a0);
  if (dat_82300) {
    *dat_82300 = v1;
    dat_82300 = v1;
    return;
  }
  dat_82308 = v1;
  dat_82300 = v1;
  return;
}


// Function: sub_17440 @ 0x17440
void sub_17440(int8 a0)
{
  char *v1;
  unsigned int v10;
  int8 *v11;
  int8 v12;
  unsigned long *v13;
  unsigned long *v14; // stack - 0x40
  code *v2;
  int8 v3;
  int4 v4;
  FILE *v5; // rax
  int8 v6; // rax
  unsigned long v7;
  unsigned long *v8; // rax
  char *v9;
  
  if (*(int8 *)(a0 + 0x1b0))
    return;
  v14 = NULL;
  if (dat_82308) {
    v11 = dat_82308;
    v13 = NULL;
    do {
      while( true ) {
        v1 = (char *)((int8)v11 + 0xc);
        v4 = *(int4 *)(a0 + 0x1a8);
        if (!faccessat(v4,v1,0,0)) break;
label_17490:
        v11 = (int8 *)*v11;
        if (!v11) goto label_175af;
      }
      v4 = sub_14950(a0,v1,0);
      if (v4 == -1) {
        sub_32ea0(v1);
        goto label_17490;
      }
      v5 = fdopen(v4,"r");
      if (!v5) {
        if (dat_82d50)
          (*dat_82d50)();
        v9 = dcgettext(NULL,"%s: fdopen failed",5);
        error(0,*__errno_location(),v9,v1);
        dat_82d58 = 2;
        close(v4);
        goto label_17490;
      }
      v6 = sub_40920();
      v12 = 0x7f160;
      v9 = dat_7f160;
      while ((v9 && (strcmp(v9,v1)))) {
        v9 = *(char **)(v12 + 0x28);
        v12 += 0x28;
      }
      v7 = *(unsigned long *)(v12 + 0x20);
      if (*(code **)(v12 + 0x18)) {
        v7 = (**(code **)(v12 + 0x18))(v7);
        *(unsigned long *)(v12 + 0x20) = v7;
      }
      v2 = *(code **)(v12 + 0x10);
      if (sub_41000(v2,v6,v5,0x50000001,10,v7)) {
        v4 = *__errno_location();
        if (dat_82d50)
          (*dat_82d50)();
        error(0,v4,"%s",(char *)sub_4ce40(v1));
        sub_16fd0(); // no-return
      }
      fclose(v5);
      v8 = (unsigned long *)sub_4ecc0(0x20);
      v3 = v11[1];
      v8[3] = v6;
      v10 = 0;
      if ((int4)v3)
        v10 = *(unsigned int *)(v12 + 8);
      *(unsigned int *)&v8[2] = v10;
      v8[1] = v13;
      *v8 = 0;
      if (!v13) {
        v14 = v8;
        v13 = v8;
        goto label_17490;
      }
      v11 = (int8 *)*v11;
      *v13 = v8;
      v13 = v8;
    } while (v11);
  }
label_175af:
  *(unsigned long **)(a0 + 0x1b0) = v14;
  return;
}


// Function: sub_17680 @ 0x17680
void sub_17680(int8 a0)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)(a0 + 0x1b0);
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_40930((struct_14 *)v2[3]);
    free(v2);
    v2 = v1;
  }
  *(unsigned long *)(a0 + 0x1b0) = 0;
  return;
}


// Function: sub_176d0 @ 0x176d0
char sub_176d0(char *a0,int8 a1)
{
  char v1;
  unsigned long *v2;
  char v3; // al
  char *v4; // rax
  char *v5;
  uint4 v6; // r12d
  char *v7;
  char v8; // stack - 0x39
  
  v3 = sub_40ae0(dat_82b60,a0);
  v8 = v3;
  if ((!v3) && (a1)) {
    v6 = 0;
    v5 = NULL;
    v7 = NULL;
    do {
      for (v2 = *(unsigned long **)(a1 + 0x1b0); v2; v2 = (unsigned long *)*v2) {
        if (!(v6 & *(uint4 *)&v2[2])) {
          v3 = sub_40ae0((int8 *)v2[3],a0);
          if (!v3) {
            if (!v7) {
              v1 = *a0;
              v7 = a0;
              while ((v1 == '.' && (v7[1] == '/'))) {
                v7 = &v7[2];
                v1 = *v7;
              }
            }
            v3 = sub_40ae0((int8 *)v2[3],v7);
            if (!v3) {
              if (v5) // branch-flip
                v3 = sub_40ae0((int8 *)v2[3],v5);
              else {
                v4 = (char *)sub_40250(a0);
                v5 = v4;
                v3 = sub_40ae0((int8 *)v2[3],v4);
              }
              if (!v3) goto label_17740;
            }
          }
          v8 = '\x01';
          free(v5); // return-dupe
          v3 = v8;
          return v3;
        }
label_17740:
      }
      a1 = *(int8 *)(a1 + 0x198);
      v6 = 2;
    } while (a1);
    free(v5);
  }
  v3 = v8;
  return v3;
}


// Function: sub_17810 @ 0x17810
void sub_17810(void)
{
  int8 *v1; // rbx
  int8 v2;
  
  if (dat_7f160) {
    v1 = (int8 *)0x7f160;
    v2 = dat_7f160;
    do {
      v1 = &v1[5];
      sub_173d0(v2,0);
      v2 = *v1;
    } while (v2);
    return;
  }
  return;
}


// Function: sub_17850 @ 0x17850
int8 * sub_17850(char *a0) // return-dupe x2
{
  stat v1; // stack - 0xb8
  int8 *v2;
  
  v2 = dat_82318;
  if (!dat_82318)
    return NULL;
  if (!fstatat(dat_7f234,a0,&v1,0x100)) {
    while ((v2[1] != v1._0_8_ || (v2[2] != v1._8_8_))) {
      v2 = (int8 *)*v2;
      if (!v2)
        return NULL;
    }
    return v2;
  }
  if (*__errno_location() == 2)
    return NULL;
  sub_33410(a0);
  return NULL;
}


// Function: sub_17910 @ 0x17910
void sub_17910(char *a0,int8 a1,int8 a2)
{
  char *v1; // rax
  char *v2; // rax
  int8 v3; // stack - 0x68
  char v4 [40];
  int8 v5; // r14
  uint8 v6; // r15
  int8 v7; // stack - 0x60
  
  if (0 <= a1) { // branch-flip
    if (((int4)(((uint4)(a2 < dat_82a18) - (uint4)(dat_82a18 < a2)) + ((uint4)(a1 < dat_82a10) - (uint4)(dat_82a10 < a1)) * 2) < 0) && (sub_43220(&v3), (int4)(((uint4)(a2 < v7) - (uint4)(v7 < a2)) + ((uint4)(a1 < v3) - (uint4)(v3 < a1)) * 2) <= -1)) {
      v6 = a1 - v3;
      v5 = a2 - v7;
      if (v5 < 0) {
        v5 += 1000000000;
        v6 -= 1;
      }
      if (dat_81b81 & 0x80) {
        if (dat_82d50)
          (*dat_82d50)();
        v1 = (char *)sub_257a0(v6,v5,v4);
        v2 = (char *)sub_23240(a1,a2,1);
        error(0,0,dcgettext(NULL,"%s: time stamp %s is %s s in the future",5),a0,v2,v1);
      }
    }
  }
  else if (dat_81b81 & 0x80) {
    if (dat_82d50)
      (*dat_82d50)();
    v1 = (char *)sub_23240(a1,a2,1);
    error(0,0,dcgettext(NULL,"%s: implausibly old time stamp %s",5),a0,v1); // tail-call
    return;
  }
  return;
}


// Function: sub_17ae0 @ 0x17ae0
void sub_17ae0(unsigned long *a0)
{
  char *v1;
  int8 v2; // stack - 0xb8
  unsigned long v3; // stack - 0xb0
  
  do {
    *(char *)&a0[0xb] = 1;
    v1 = (char *)a0[0x14];
    if (sub_25fc0(v1,&v2)) { // branch-flip
      sub_33410((char *)a0[0x14]);
      a0 = (unsigned long *)*a0;
    }
    else {
      a0[1] = v2;
      a0[2] = v3;
      a0 = (unsigned long *)*a0;
    }
  } while ((a0) && (!*(char *)&a0[0xb]));
  return;
}


// Function: sub_17b80 @ 0x17b80
void sub_17b80(char *a0,int8 a1,unsigned int a2,unsigned int a3,unsigned int a4,unsigned int a5)
{
  unsigned int v1;
  unsigned long *v10;
  unsigned long v11;
  stat v12; // stack - 0xd8
  void *v13;
  int4 v2;
  char *v3;
  int8 v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  char v8; // al
  uint8 v9; // rax
  
  v9 = strlen(a0);
  for (v10 = dat_82320; v10; v10 = (unsigned long *)*v10) {
    v3 = (char *)v10[0x14];
    if (!strcmp(v3,a0)) {
      if (*(char *)&v10[10]) {
        v2 = *(int4 *)((int8)v10 + 0x54);
        if (fstatat(dat_7f234,v3,&v12,v2)) // branch-flip
          sub_33410((char *)v10[0x14]);
        else {
          v10[1] = v12._0_8_;
          v10[2] = v12._8_8_;
        }
      }
      *(unsigned int *)&v10[3] = a4;
      if (a1) goto label_17c16;
      goto label_17d58;
    }
  }
  v10 = (unsigned long *)sub_4ecc0(0xa8);
  *v10 = dat_82320;
  v10[0x13] = v9;
  dat_82320 = v10;
  v11 = sub_4ee30(a0);
  *(char *)&v10[0xb] = 0;
  v10[0x14] = v11;
  if (!a1) {
    *(unsigned int *)&v10[3] = a4;
label_17d58:
    *(unsigned int *)&v10[9] = a2;
    *(char *)&v10[10] = 1;
    *(unsigned int *)((int8)v10 + 0x4c) = a3;
    *(unsigned int *)((int8)v10 + 0x54) = a5;
    *(unsigned int *)((int8)v10 + 0x5c) = dat_82580;
    v10[0xc] = 0;
    v10[0xd] = 0;
    v10[0xe] = 0;
    v10[0xf] = 0;
    v10[0x10] = 0;
    v10[0x12] = 0;
    v10[0x11] = 0;
    v8 = sub_25b20(a0);
    goto label_17dc1;
  }
  v10[1] = *(unsigned long *)(a1 + 0x58);
  v10[2] = *(unsigned long *)(a1 + 0x60);
  *(unsigned int *)&v10[3] = a4;
label_17c16:
  v3 = *(char **)(a1 + 0x30);
  v11 = *(unsigned long *)(a1 + 0xe8);
  v5 = *(unsigned long *)(a1 + 0xf0);
  v6 = *(unsigned long *)(a1 + 0xf8);
  v7 = *(unsigned long *)(a1 + 0x100);
  *(unsigned int *)((int8)v10 + 0x1c) = *(unsigned int *)(a1 + 0x74);
  v1 = *(unsigned int *)(a1 + 0x78);
  *(unsigned int *)&v10[9] = a2;
  *(unsigned int *)&v10[4] = v1;
  *(char *)&v10[10] = 0;
  *(unsigned int *)((int8)v10 + 0x4c) = a3;
  *(unsigned int *)((int8)v10 + 0x54) = a5;
  *(unsigned int *)((int8)v10 + 0x5c) = dat_82580;
  v10[0xc] = 0;
  v10[5] = v11;
  v10[6] = v5;
  v10[7] = v6;
  v10[8] = v7;
  sub_25120(&v10[0xc],v3);
  if (*(void **)(a1 + 0x38)) { // branch-flip
    v11 = sub_4edf0(*(void **)(a1 + 0x38),*(int8 *)(a1 + 0x40) + 1);
    v13 = *(void **)(a1 + 0x48);
    v10[0xd] = v11;
    v10[0xe] = *(unsigned long *)(a1 + 0x40);
    if (v13) goto label_17cb1;
label_17d3e:
    v10[0xf] = 0;
    v11 = 0;
  }
  else {
    v13 = *(void **)(a1 + 0x48);
    v10[0xd] = 0;
    v10[0xe] = 0;
    if (!v13) goto label_17d3e;
label_17cb1:
    v4 = *(int8 *)(a1 + 0x50);
    v10[0xf] = sub_4edf0(v13,v4 + 1);
    v11 = *(unsigned long *)(a1 + 0x50);
  }
  v10[0x10] = v11;
  sub_1d820(a1,&v10[0x12],&v10[0x11]);
  v8 = sub_25b20(a0);
label_17dc1:
  if (!v8)
    return;
  sub_17ae0(v10); // tail-call
  return;
}


// Function: sub_17ec0 @ 0x17ec0
int4 sub_17ec0(char *a0,char *a1) // return-dupe
{
  int4 v1;
  char v2;
  int4 v3; // eax
  int4 *v4; // rax
  char *v5;
  char *v6;
  stat v7;
  uint4 v8; // r12d
  uint4 v9;
  
  v5 = a0;
  if (!*a0)
    return 0;
label_17f00:
  do {
    v2 = v5[1];
    v6 = v5;
    if (!v2)
      return 0;
    while( true ) {
      v5 = &v6[1];
      if ((((a0 == v5) || (v2 != '/')) || (*v6 == '/')) || ((*v6 == '.' && (((v5 == &a0[1] || (v6[-1] == '/')) || ((v6[-1] == '.' && ((v5 == &a0[2] || (v6[-2] == '/')))))))))) goto label_17f00;
      *v5 = '\0';
      v8 = ~dat_8232c & 0x1ff;
      v9 = v8;
      if (!dat_82330)
        v9 = v8 | 0xc0;
      if (!mkdirat(dat_7f234,a0,v9)) break;
      v4 = __errno_location();
      v1 = *v4;
      if ((v1 != 0x11) && (v3 = fstatat(dat_7f234,a0,&v7,0), v3)) {
        *v4 = v1;
        sub_32e40(a0);
        *v5 = '/';
        return v3;
      }
      *v5 = '/';
      v2 = v6[2];
      v6 = v5;
      if (!v2)
        return 0;
    }
    sub_17b80(a0,0,~dat_82328 & v9,0x1ff,v8,0x100);
    sub_23c30(a0,(int4)v5 - (int4)a0,v8);
    *a1 = 1;
    *v5 = '/';
  } while( true );
}


// Function: sub_18100 @ 0x18100
unsigned long sub_18100(uint8 a0,char *a1,uint8 a2,unsigned long a3,int4 a4) // early-return
{
  int4 *v1; // rax
  int4 v2;
  
  if (!sub_18090(a0,a1,a2,a3))
    return 0;
  v1 = __errno_location();
  v2 = *v1;
  if (v2 != 1) {
    if (((int4)a3) && (a4 != 0x32)) {
      if ((v2 == 0x5f) || (v2 == 0x26)) {
        if (!sub_18090(a0 & 0xffffffff,a1,a2 & 0xffffffff,0))
          return 0;
        v2 = *v1;
      }
      if (!v2)
        return 0;
    }
    else {
      if (!v2)
        return 0;
      if (a4 == 0x32) {
        if (v2 == 0x26)
          return 0;
        if (v2 == 0x5f)
          return 0;
      }
    }
  }
  *v1 = v2;
  return 0xffffffff;
}


// Function: sub_181b0 @ 0x181b0
void sub_181b0(char *a0,int8 a1,int4 a2,uint4 a3,uint4 a4,char a5,unsigned int a6,int4 a7)
{
  int4 v1;
  int4 v2;
  stat v3; // stack - 0xd8
  uint4 v4;
  uint4 v5;
  char v6; // r11b
  int4 v7; // r14d
  uint4 v8; // stack - 0xec
  
  v1 = a7;
  v7 = (int4)a5;
  v8 = ~a4;
  v6 = (char)a6;
  if (dat_82a68) { // branch-flip
    if (1 <= dat_82aa8) { // branch-flip
      v5 = 0x1ff;
      if (v6) goto label_1821d;
label_1837b:
      v5 = *(uint4 *)(a1 + 0x74);
      v4 = *(uint4 *)(a1 + 0x78);
      if (0 <= a2) { // branch-flip
        if (!fchown(a2,v5,v4)) goto label_18505;
        v2 = *__errno_location();
        if ((v2 == 0x26) || (v2 == 0x5f)) goto label_184d8;
label_183d7:
        if (a5 == '2') {
          v2 = *__errno_location();
          if ((v2 == 0x26) || (v2 == 0x5f)) goto label_183fe;
        }
        sub_32ce0(a0,v5,v4);
      }
      else {
label_184d8:
        if (fchownat(dat_7f234,a0,v5,v4,v1)) goto label_183d7;
label_18505:
        if ((v8 | a3) & 0x49) {
          a4 &= ~(a3 & 0xc00);
          v8 = ~a4;
        }
      }
label_183fe:
      v5 = 0xfff;
      if (dat_82aa4 <= 0)
        v5 = 0x1ff;
    }
    else {
label_18420:
      if ((dat_82aa4 <= 0) || (v5 = 0xfff, v6))
        v5 = 0x1ff;
    }
  }
  else {
    v5 = 0x1ff;
    if (!v6) {
      if (dat_82b50) { // branch-flip
        v3._0_8_ = *(unsigned long *)(a1 + 0xe8);
        v3._8_8_ = *(unsigned long *)(a1 + 0xf0);
      }
      else {
        v3._8_8_ = 0x3ffffffe;
      }
      v3._16_8_ = *(unsigned long *)(a1 + 0xf8);
      v3._24_8_ = *(unsigned long *)(a1 + 0x100);
      v2 = sub_412d0(a2,dat_7f234,a0,&v3,a7);
      if (v2) { // branch-flip
        if (a5 == '2') {
          v2 = *__errno_location();
          if ((v2 == 0x5f) || (v2 == 0x26)) goto label_1836d;
        }
        sub_334b0(a0);
      }
      else {
        if (dat_82b50)
          sub_17910(a0,v3._0_8_,v3._8_8_);
        sub_17910(a0,v3._16_8_,v3._24_8_);
      }
label_1836d:
      if (dat_82aa8 > 0) goto label_1837b;
      goto label_18420;
    }
  }
label_1821d:
  v4 = ~dat_82328 & *(uint4 *)(a1 + 0x70);
  if ((v4 ^ a3 | v8) & v5) {
    if (~(a4 | v5) & 0xfff) {
      if (0 <= a2) // branch-flip
        v2 = fstat(a2,&v3);
      else {
        v2 = fstatat(dat_7f234,a0,&v3,v1);
      }
      a3 = v3._24_4_;
      if (v2) {
        sub_33410(a0);
        goto label_1825e;
      }
    }
    v5 = (v4 ^ a3 & 0xfff) & v5;
    if (v5) {
      v5 = a3 & 0xfff ^ v5;
      if (sub_18100(a2,a0,v5,v1,v7))
        sub_32c30(a0,v5);
    }
  }
label_1825e:
  sub_32570(a1,a0,v7,1);
  sub_31f50(a1,a0,v7);
  sub_324c0(a1,a0,v7); // tail-call
  return;
}


// Function: sub_185e0 @ 0x185e0
void sub_185e0(void)
{
  int8 *v1;
  int8 *v10;
  int8 v11; // stack - 0x1c8
  int8 v12; // stack - 0x1c0
  int8 v13; // stack - 0x1b8
  int8 v14; // stack - 0x1b0
  int8 v15; // stack - 0x1a8
  unsigned int v16; // stack - 0x188
  unsigned int v17; // stack - 0x184
  unsigned int v18; // stack - 0x180
  int8 v19; // stack - 0x110
  unsigned long *v2;
  int8 v20; // stack - 0x108
  int8 v21; // stack - 0x100
  int8 v22; // stack - 0xf8
  int8 v23; // stack - 0xa0
  int8 v24; // stack - 0x98
  int8 *v3;
  int4 v4;
  unsigned long *v5;
  int8 *v6;
  stat v7; // stack - 0x288
  char v8 [48];
  int8 *v9;
  
  v9 = dat_82318;
  if (!dat_82318) {
    dat_82318 = NULL;
    return;
  }
  do {
    v10 = NULL;
    v3 = &v9[0x14];
    sub_26300(*(unsigned int *)&v9[0xb]);
    v6 = (int8 *)v9[0xc];
    if (v6) {
      do {
        while( true ) {
          v1 = &v6[1];
          if ((((!fstatat(dat_7f234,(char *)v1,&v7,0x100)) && (v7._0_8_ == v9[1])) && (v7._8_8_ == v9[2])) && (v4 = (int4)~(uint4)((uint8)v9[3] >> 0x20) >> 0x1f, !(CONCAT31((undefined3)((uint4)-v4 >> 8),v9[4] < -1) + ((int4)~(uint4)((uint8)v9[4] >> 0x20) >> 0x1f) + ((uint4)(v9[3] < -1) + v4) * 2))) break;
label_18640:
          v6 = (int8 *)*v6;
          if (!v6) goto label_18730;
        }
        if (unlinkat(dat_7f234,(char *)v1,0)) {
          sub_33490(v1);
          goto label_18640;
        }
        if ((v10) && (!linkat(dat_7f234,(char *)v10,dat_7f234,(char *)v1,0))) goto label_18640;
        if ((char)v9[5]) {
          if (symlinkat((char *)v3,dat_7f234,(char *)v1)) // branch-flip
            sub_33370(v3,v1);
          else {
            v16 = *(unsigned int *)((int8)v9 + 0x2c);
            v19 = v9[7];
            v20 = v9[8];
            v21 = v9[9];
            v22 = v9[10];
            v17 = *(unsigned int *)&v9[6];
            v18 = *(unsigned int *)((int8)v9 + 0x34);
            v11 = v9[0xd];
            v12 = v9[0xe];
            v13 = v9[0xf];
            v14 = v9[0x10];
            v15 = v9[0x11];
            v24 = v9[0x13];
            v23 = v9[0x12];
            sub_181b0(v1,v8,0xffffffff,0,0,0x32,0,0x100);
            v10 = v1;
          }
          goto label_18640;
        }
        if (!linkat(dat_7f234,(char *)v3,dat_7f234,(char *)v1,0)) goto label_18640;
        sub_32dc0(v3,v1);
        v6 = (int8 *)*v6;
      } while (v6);
label_18730:
      v5 = (unsigned long *)v9[0xc];
      while (v5) {
        v2 = (unsigned long *)*v5;
        free(v5);
        v5 = v2;
      }
    }
    sub_1d580((unsigned long *)v9[0x13],v9[0x12]);
    free((void *)v9[0xd]);
    v3 = (int8 *)*v9;
    free(v9);
    if (!v3) {
      dat_82318 = NULL;
      return;
    }
    v9 = v3;
  } while( true );
}


// Function: sub_18980 @ 0x18980
unsigned long sub_18980(char *a0,bool a1,char *a2) // return-dupe
{
  int4 v1;
  uint4 *v2; // rax
  char v3 [152];
  
  if (*a2)
    return 0;
  v2 = (uint4 *)__errno_location();
  v1 = *v2;
  if (v1 != 0x11) { // branch-flip
    if (0x12 <= v1) {
      if ((((v1 != 0x28) && ((v1 & 0xffffffbf) != 0x1f)) || (!a1)) || ((dat_82b34 != 2 || (dat_82b69)))) goto label_189f0;
      if (strchr(a0,0x2f)) {
        if (sub_25fc0(a0,v3)) goto label_189f0;
        goto label_18a42;
      }
      goto label_18a80;
    }
    if (v1 != 2) goto label_189f0;
  }
  else {
label_18a42:
    if (dat_82b34 == 5) {
      if (dat_81b82 & 0x10) {
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"%s: skipping existing file",5),a0);
        return 2;
      }
      return 2;
    }
    if (6 <= dat_82b34) { // branch-flip
      if ((dat_82b34 == 6) && (!sub_188c0(a0))) goto label_18a80;
    }
    else if (3 <= dat_82b34) { // branch-flip
      if (dat_82b34 == 4)
        return 0;
    }
    else {
label_18a80:
      if (1 <= (int4)sub_26a40(a0,0))
        return 1;
    }
  }
  if ((!sub_17ec0(a0,a2)) && (*a2))
    return 1;
label_189f0:
  *v2 = v1;
  return 0;
}


// Function: sub_18b30 @ 0x18b30
unsigned int sub_18b30(char *a0,char a1)
{
  int4 v1; // eax
  uint4 v2;
  char v3; // stack - 0x31
  
  v3 = 0;
  v2 = dat_828b0;
  if (1 <= dat_82aa8)
    v2 = dat_828b0 & 0xffffffc0;
  while( true ) {
    if (!mkfifoat(dat_7f234,a0,v2 & 0x1ff)) {
      sub_181b0(a0,0x82840,0xffffffff,~dat_82328 & v2 & 0x1ff,0x1ff,(int4)a1,0,0x100);
      return 0;
    }
    v1 = sub_18980(a0,0,&v3);
    if (!v1) break;
    if (v1 == 2)
      return 0;
  }
  sub_32e60(a0);
  return 0xffffffff;
}


// Function: sub_18c10 @ 0x18c10
unsigned int sub_18c10(char *a0,char a1)
{
  int4 v1; // eax
  uint4 v2;
  char v3; // stack - 0x31
  
  v3 = 0;
  v2 = dat_828b0;
  if (1 <= dat_82aa8)
    v2 = dat_828b0 & 0xffffffc0;
  while( true ) {
    v1 = mknodat(dat_7f234,a0,v2 & 0x61ff,dat_828c0);
    if (!v1) {
      sub_181b0(a0,0x82840,0xffffffff,~dat_82328 & v2 & 0x61ff,0x1ff,(int4)a1,0,0x100);
      return 0;
    }
    v1 = sub_18980(a0,0,&v3);
    if (!v1) break;
    if (v1 == 2)
      return 0;
  }
  sub_32e80(a0);
  return 0xffffffff;
}


// Function: sub_18d00 @ 0x18d00
unsigned int sub_18d00(char *a0,char a1,char *a2,unsigned long *a3)
{
  unsigned long v1;
  uint8 v2;
  char *v3;
  int4 v4; // eax
  unsigned long *v5;
  unsigned long *v6; // rax
  stat v7; // stack - 0xc8
  
  do {
    v4 = openat(dat_7f234,a0,0xc1,0);
    if (0 <= v4) {
      if (fstat(v4,&v7)) {
        sub_33410(a0);
        close(v4);
        return 0xffffffff;
      }
      if (!close(v4)) {
        v5 = (unsigned long *)sub_4ecc0(strlen(dat_82858) + 0xa1);
        if (a3) { // branch-flip
          v1 = *a3;
          *a3 = v5;
          *v5 = v1;
        }
        else {
          *v5 = dat_82318;
          dat_82318 = v5;
        }
        v5[3] = 0xffffffffffffffff;
        v5[4] = 0xffffffffffffffff;
        v5[1] = v7._0_8_;
        *(char *)&v5[5] = a1;
        v5[2] = v7._8_8_;
        if (a1) {
          *(unsigned int *)((int8)v5 + 0x2c) = dat_828b0;
          v5[7] = dat_82928;
          v5[8] = dat_82930;
          v5[6] = dat_828b4;
          v5[9] = dat_82938;
          v5[10] = dat_82940;
        }
        *(unsigned int *)&v5[0xb] = dat_82580;
        v6 = (unsigned long *)sub_4ecc0(strlen(a0) + 9);
        v5[0xc] = v6;
        *v6 = 0;
        strcpy((char *)&v6[1],a0);
        v5[0xd] = 0;
        sub_25120(&v5[0xd],dat_82870);
        v5[0xe] = 0;
        v5[0xf] = 0;
        v5[0x10] = 0;
        v5[0x11] = 0;
        sub_1d820(0x82840,&v5[0x13],&v5[0x12]);
        strcpy((char *)&v5[0x14],dat_82858);
        v5 = dat_82320;
        do {
          if (!v5)
            return 0;
          if (!*(char *)&v5[0xb]) {
            v2 = v5[0x13];
            v3 = (char *)v5[0x14];
            if (((!strncmp(a0,v3,v2)) && (a0[v2] == '/')) && ((char *)sub_400d0(a0) == &a0[v2 + 1])) {
              sub_17ae0(v5);
              return 0;
            }
          }
          v5 = (unsigned long *)*v5;
        } while( true );
      }
      sub_32d60(a0);
      return 0xffffffff;
    }
    if ((*__errno_location() == 0x11) && (sub_17850(a0)))
      return 0;
    v4 = sub_18980(a0,0,a2);
    if (!v4) {
      sub_32ea0(a0);
      return 0xffffffff;
    }
  } while (v4 != 2);
  return 0;
}


// Function: sub_18fd0 @ 0x18fd0
unsigned int sub_18fd0(char *a0)
{
  char *v1;
  char v10; // stack - 0x159
  stat v11; // stack - 0xc8
  char v2; // al
  int4 v3;
  int4 v4; // eax
  unsigned long *v5; // rax
  int4 *v6; // rax
  int8 *v7; // rax
  int8 *v8;
  stat v9; // stack - 0x158
  
  v1 = dat_82858;
  v10 = 0;
  if ((!dat_82b92) && (v2 = sub_29420(dat_82858), v2))
    return sub_18d00(a0,0,&v10,NULL);
  v5 = (unsigned long *)sub_17850(v1);
  if (v5)
    return sub_18d00(a0,0,&v10,v5);
  v6 = __errno_location();
  while( true ) {
    v4 = linkat(dat_7f234,v1,dat_7f234,a0,0);
    v8 = dat_82318;
    v3 = *v6;
    if (!v4) {
      if ((dat_82318) && (!fstatat(dat_7f234,v1,&v9,0x100))) {
        do {
          if (((int4)v8[0xb] == dat_82580) && (((v8[1] == v9._0_8_ && (v8[2] == v9._8_8_)) && (!((-(uint4)(v8[4] < -1) - ((int4)~(uint4)((uint8)v8[4] >> 0x20) >> 0x1f)) + (-(uint4)(v8[3] < -1) - ((int4)~(uint4)((uint8)v8[3] >> 0x20) >> 0x1f)) * 2))))) {
            v7 = (int8 *)sub_4ecc0(strlen(a0) + 9);
            strcpy((char *)&v7[1],a0);
            *v7 = v8[0xc];
            v8[0xc] = (int8)v7;
            return 0;
          }
          v8 = (int8 *)*v8;
        } while (v8);
      }
      return 0;
    }
    if ((v3 == 0x11) && (!strcmp(v1,a0)))
      return 0;
    if (((!fstatat(dat_7f234,v1,&v9,0x100)) && (!fstatat(dat_7f234,a0,&v11,0x100))) && ((v9._0_8_ == v11._0_8_ && (v9._8_8_ == v11._8_8_)))) break;
    *v6 = v3;
    v3 = sub_18980(a0,0,&v10);
    if (v3 != 1) {
      if (v3 == 2)
        return 0;
      if ((dat_82b50) && (*v6 == 0x11))
        return 0;
      sub_32dc0(v1,a0);
      return 1;
    }
  }
  return 0;
}


// Function: sub_19270 @ 0x19270
unsigned long sub_19270(char *a0) // early-return
{
  char v1; // al
  int4 v2; // eax
  unsigned long v3; // rax
  char v4; // stack - 0x21
  char *v5;
  
  v4 = 0;
  if ((!dat_82b92) && ((*dat_82858 == '/' || (v1 = sub_29420(dat_82858), v1)))) {
    v3 = sub_18d00(a0,1,&v4,NULL);
    return v3;
  }
  v5 = dat_82858;
  while( true ) {
    if (!symlinkat(v5,dat_7f234,a0)) {
      sub_181b0(a0,0x82840,0xffffffff,0,0,0x32,0,0x100);
      return 0;
    }
    v2 = sub_18980(a0,0,&v4);
    if (!v2) break;
    if (v2 == 2)
      return 0;
    v5 = dat_82858;
  }
  sub_33370(dat_82858,a0);
  return 0xffffffff;
}


// Function: sub_19380 @ 0x19380
int4 sub_19380(char *a0,int4 a1)
{
  int4 v1;
  stat v10; // stack - 0xd8
  uint4 v11; // ebp
  uint4 v12;
  char v13; // stack - 0xf4
  uint4 v14; // stack - 0xf0
  unsigned int v15; // stack - 0xec
  bool v2;
  int4 v3;
  int8 v4; // rax
  uint8 v5;
  uint8 v6; // rax
  int4 *v7; // rax
  uint8 v8; // stack - 0xe0
  char v9; // stack - 0xe1
  
  v9 = 0;
  v11 = dat_828b0 & 0x1c0;
  if (dat_82aa8 <= 0)
    v11 = dat_828b0 & 0x1ff;
  v13 = (char)a1;
  if (dat_82a6a) { // branch-flip
    v15 = 0;
    v3 = 1;
    v14 = 0;
  }
  else {
    if (!dat_82a60) {
      v8 &= 0xffffffffffffff00;
      if ((1 <= dat_82a98) && (dat_82998)) {
        v12 = ~dat_82328 & dat_828b0;
        do {
          v3 = mknodat(dat_7f234,a0,v12 & 0x1ff,0);
          if (!v3) {
            v2 = 1;
            sub_32570(0x82840,a0,(int4)v13,0);
            goto label_195a8;
          }
          v3 = sub_18980(a0,0,&v8);
          if (!v3) {
            sub_24660();
            sub_32ea0(a0);
            return 1;
          }
        } while (v3 != 2);
      }
      v2 = 0;
label_195a8:
      do {
        v1 = dat_82b34;
        v12 = 0x809c1;
        if (dat_82b34 == 2)
          v12 = (-(uint4)(dat_82b69 == '\0') & 0x20000) + 0x80b41;
        if (v2)
          v12 &= 0xffffff7f;
        if (((a1 == 0x37) && (!dat_82310)) && (dat_82310 = 1, dat_81b80 & 8)) {
          if (dat_82d50)
            (*dat_82d50)();
          error(0,0,dcgettext(NULL,"Extracting contiguous files as regular files",5));
        }
        v3 = openat(dat_7f234,a0,v12,(uint8)v11);
        if (0 <= v3) {
          if (v1 != 2) {
            v15 = 0x1ff;
            v14 = ~dat_82328 & v11;
            goto label_19421;
          }
          if (fstat(v3,&v10)) { // branch-flip
            v7 = __errno_location();
            v1 = *v7;
            close(v3);
            *v7 = v1;
          }
          else {
            v14 = v10._24_4_;
            if ((v10._24_4_ & 0xf000) == 0x8000) {
              v15 = 0xffffffff;
              goto label_19421;
            }
            close(v3);
            *__errno_location() = 0x11;
          }
        }
        v3 = sub_18980(a0,1,&v9);
        if (v3 != 1) {
          sub_24660();
          if (v3 == 2)
            return 0;
          sub_32ea0(a0);
          return 1;
        }
      } while( true );
    }
    v3 = sub_2c8b0(a0,0x66,0x82840);
    v15 = 0;
    v14 = 0;
    if (v3 < 0) {
      sub_24660();
      return 0;
    }
  }
label_19421:
  sub_c670((struct_48 *)0x82840);
  if (dat_82960) { // branch-flip
    sub_2b460(v3,0x82840,&v8);
    v5 = v8;
  }
  else {
    v8 = dat_828c8;
    v5 = dat_828c8;
    while (0 < (int8)v5) {
      sub_c710(v5);
      v4 = sub_d360();
      if (!v4) {
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
        dat_82d58 = 2;
        v5 = v8;
        break;
      }
      v5 = sub_cd10(v4);
      if (v8 < v5)
        v5 = v8;
      *__errno_location() = 0;
      v6 = sub_26070(v3,v4,v5);
      v8 -= v5;
      sub_ccd0(v4 + -1 + v5);
      if (v5 != v6) {
        if (!dat_82a60)
          sub_33510(a0,v6,v5);
        v5 = v8;
        break;
      }
      v5 = v8;
    }
  }
  sub_23d40(v5);
  sub_c6a0();
  if (dat_82a6a)
    return 0;
  if (dat_82a60) // branch-flip
    v3 = close(v3);
  else {
    sub_181b0(a0,0x82840,v3,v14,v15,(int4)v13,0,(uint8)(dat_82b34 != 2) << 8);
    v3 = close(v3);
  }
  if (v3 <= -1)
    sub_32d60(a0);
  if (dat_82a60) {
    sub_2ce00();
    return v3;
  }
  return v3;
}


// Function: sub_198d0 @ 0x198d0
int4 sub_198d0(char *a0,int4 a1)
{
  int8 *v1;
  int8 v10; // stack - 0x168
  char v11; // stack - 0x169
  stat v12; // stack - 0xd8
  unsigned int *v13;
  unsigned long v14; // r9
  uint4 v15;
  uint4 v16;
  unsigned long v17; // r15
  int8 v18; // stack - 0x160
  uint4 v19; // stack - 0x150
  unsigned int *v2;
  char v3;
  int4 v4;
  int4 v5; // eax
  int4 *v6; // rax
  char *v7;
  int8 v8; // rcx
  int8 *v9;
  
  v11 = '\0';
  if ((dat_82ad1) && (!dat_827f0)) {
    if (fstatat(dat_7f234,".",&v12,0)) // branch-flip
      sub_26460(".");
    else {
      dat_827f0 = v12._0_8_;
    }
  }
  if (dat_82b50) // branch-flip
    sub_21100(a0);
  else if (a1 == 0x44)
    sub_24660();
  v15 = 0x1c0;
  if ((dat_82aa8 <= 0) && (v15 = 0x1c0, dat_82aa4 <= 0))
    v15 = 0x1ff;
  v15 = dat_828b0 & v15;
  if (!dat_82330)
    v15 |= 0xc0;
  v17 = 0;
  v16 = 0;
  do {
    v4 = mkdirat(dat_7f234,a0,v15);
    if (!v4) goto label_19c10;
    v6 = __errno_location();
    v3 = v11;
    if (*v6 == 0x11) {
      if (v11) { // branch-flip
        if (dat_82b30) {
label_19a74:
          if ((((!fstatat(dat_7f234,a0,&v12,0x100)) && ((v12._24_4_ & 0xf000) == 0xa000)) && (!fstatat(dat_7f234,a0,&v12,0))) && ((v12._24_4_ & 0xf000) == 0x4000)) {
            *v6 = 0x11;
            return 0;
          }
          *v6 = 0x11;
        }
label_199be:
        if (!sub_25fc0(a0,&v10)) {
          if ((v19 & 0xf000) == 0x4000) {
            if (v3) {
              if (dat_82320) {
                v9 = dat_82320;
                do {
                  v4 = *(int4 *)((int8)v9 + 0x54);
                  v7 = (char *)v9[0x14];
                  if (fstatat(dat_7f234,v7,&v12,v4)) {
                    sub_33410((char *)v9[0x14]);
                    return 0;
                  }
                  if ((v12._0_8_ == v10) && (v12._8_8_ == v18)) {
                    v9[1] = dat_82898;
                    v9[2] = dat_828a0;
                    *(uint4 *)&v9[3] = dat_828b0;
                    *(unsigned int *)((int8)v9 + 0x1c) = dat_828b4;
                    v9[5] = dat_82928;
                    v9[6] = dat_82930;
                    *(unsigned int *)&v9[4] = dat_828b8;
                    v9[7] = dat_82938;
                    v9[8] = dat_82940;
                    *(unsigned int *)&v9[9] = v12._24_4_;
                    *(unsigned int *)((int8)v9 + 0x4c) = 0xffffffff;
                    *(char *)&v9[10] = 0;
                    return 0;
                  }
                  v9 = (int8 *)*v9;
                } while (v9);
              }
              if (dat_82d50)
                (*dat_82d50)();
              v7 = (char *)sub_4ce40(a0);
              error(0,0,dcgettext(NULL,"%s: Unexpected inconsistency when making directory",5),v7);
              dat_82d58 = 2;
              return 0;
            }
            if (dat_82b34 == 1) {
              v15 = 0x1c0;
              if ((dat_82aa8 <= 0) && (v15 = 0x1c0, dat_82aa4 <= 0))
                v15 = 0x1ff;
              v15 = v15 & v19 | -(uint4)(dat_82330 == '\0') & 0xc0;
              v4 = sub_18100(0xffffffff,a0,v15,0x100,0x35);
              if (!v4) {
                v8 = 0x24;
                v9 = &v10;
                v13 = (unsigned int *)0x82898;
                while (v8) {
                  v2 = &v13[1];
                  v1 = (int8 *)((int8)v9 + 4);
                  *v13 = *(unsigned int *)v9;
                  v8 -= 1;
                  v9 = v1;
                  v13 = v2;
                }
label_19c10:
                v14 = 0x100;
                v17 = 0x1ff;
                v19 = ~dat_82328 & v15;
                sub_17b80(a0,0x82840,v19,v17,dat_828b0,v14); // return-dupe
                return v4;
              }
              sub_32c30(a0,v15);
            }
            v17 = 0xffffffff;
            v15 = dat_82b34;
            goto label_19a19;
          }
          v17 = 0xffffffff;
          v16 = v19;
        }
        *v6 = 0x11;
      }
      else {
        if (dat_82b30) goto label_19a74;
        if (dat_82b34 <= 2) goto label_199be;
      }
    }
    v5 = sub_18980(a0,0,&v11);
    if (!v5) {
      if (*v6 != 0x11) {
        sub_32e40(a0);
        return 1;
      }
      goto label_19a13;
    }
    if (v5 != 1) {
label_19a13:
      v19 = v16;
      v15 = dat_82b34;
label_19a19:
      if (v15 & 0xfffffffd)
        return v4;
      v14 = 0;
      sub_17b80(a0,0x82840,v19,v17,dat_828b0,v14);
      return v4;
    }
  } while( true );
}


// Function: sub_19da0 @ 0x19da0
void sub_19da0(char *a0,uint1 a1) // return-dupe
{
  int4 v1;
  undefined4 v10;
  unsigned int v11;
  unsigned long v12; // stack - 0x1c8
  unsigned long v13; // stack - 0x1c0
  unsigned long v14; // stack - 0x1b8
  unsigned long v15; // stack - 0x1b0
  unsigned long v16; // stack - 0x1a8
  unsigned int v17; // stack - 0x188
  unsigned int v18; // stack - 0x184
  unsigned int v19; // stack - 0x180
  uint8 v2;
  unsigned long v20; // stack - 0x110
  unsigned long v21; // stack - 0x108
  unsigned long v22; // stack - 0x100
  unsigned long v23; // stack - 0xf8
  unsigned long v24; // stack - 0xa0
  unsigned long v25; // stack - 0x98
  void *v3;
  unsigned long *v4;
  uint8 v5; // rax
  char *v6;
  uint1 v7; // bl
  char v8 [48];
  stat v9; // stack - 0x288
  
  v5 = strlen(a0);
  if (!dat_82320)
    return;
  v7 = 0;
  do {
    v4 = dat_82320;
    v10 = *(unsigned int *)&dat_82320[9];
    v11 = *(unsigned int *)((int8)dat_82320 + 0x4c);
    v7 |= *(uint1 *)&dat_82320[0xb];
    if (a1 < *(uint1 *)&dat_82320[0xb])
      return;
    v2 = dat_82320[0x13];
    if ((((v2 < v5) && (a0[v2])) && ((a0[v2] == '/' || (a0[v2 - 1] == '/')))) && (v3 = (void *)dat_82320[0x14], !memcmp(a0,v3,v2)))
      return;
    sub_26300(*(unsigned int *)((int8)v4 + 0x5c));
    if (v7) { // branch-flip
      v1 = *(int4 *)((int8)v4 + 0x54);
      v6 = (char *)v4[0x14];
      if (fstatat(dat_7f234,v6,&v9,v1)) // branch-flip
        sub_33410((char *)v4[0x14]);
      else {
        if ((v9._0_8_ == v4[1]) && (v9._8_8_ == v4[2])) {
          v11 = 0xffffffff;
          v10 = v9._24_4_;
          goto label_19df0;
        }
        if (dat_82d50)
          (*dat_82d50)();
        v6 = (char *)sub_4ce40((char *)v4[0x14]);
        error(0,0,dcgettext(NULL,"%s: Directory renamed before its status could be extracted",5),v6);
        dat_82d58 = 2;
      }
    }
    else {
label_19df0:
      v17 = *(unsigned int *)&v4[3];
      v20 = v4[5];
      v21 = v4[6];
      v22 = v4[7];
      v23 = v4[8];
      v18 = *(unsigned int *)((int8)v4 + 0x1c);
      v19 = *(unsigned int *)&v4[4];
      v12 = v4[0xc];
      v13 = v4[0xd];
      v14 = v4[0xe];
      v15 = v4[0xf];
      v16 = v4[0x10];
      v25 = v4[0x12];
      v24 = v4[0x11];
      sub_181b0((char *)v4[0x14],v8,0xffffffff,v10,v11,0x35,(uint1)*(char *)&v4[10],*(unsigned int *)((int8)v4 + 0x54));
    }
    dat_82320 = (unsigned long *)*v4;
    free((void *)v4[0x14]);
    sub_1d580((unsigned long *)v4[0x12],v4[0x11]);
    free((void *)v4[0xc]);
    free((void *)v4[0xd]);
    free((void *)v4[0xf]);
    free(v4);
    if (!dat_82320)
      return;
  } while( true );
}


// Function: sub_1a080 @ 0x1a080
void sub_1a080(void)
{
  sub_19da0("",0);
  sub_185e0();
  sub_19da0("",1); // tail-call
  return;
}


// Function: sub_1a0b0 @ 0x1a0b0
void sub_1a0b0(void)
{
  uint4 v1; // eax
  
  dat_82330 = geteuid() == 0;
  dat_82aa4 += (uint4)(uint1)dat_82330;
  dat_82aa8 += (uint4)(uint1)dat_82330;
  v1 = umask(0);
  dat_8232c = v1;
  if (1 <= dat_82aa4) {
    dat_82328 = 0;
    return;
  }
  umask(v1);
  dat_82328 = v1;
  return;
}


// Function: sub_1a110 @ 0x1a110
void sub_1a110(char *a0)
{
  int8 *v1;
  char *v2;
  int4 v3;
  int8 *v4; // rbp
  int8 *v5;
  
  v3 = dat_82580;
  if (!dat_82320)
    return;
  v1 = dat_82320;
  v5 = NULL;
  while( true ) {
    v4 = v1;
    v1 = (int8 *)*v4;
    if (*(int4 *)((int8)v4 + 0x5c) == v3) {
      v2 = (char *)v4[0x14];
      if (!strcmp(v2,a0)) {
        free(v2);
        sub_1d580((unsigned long *)v4[0x12],v4[0x11]);
        free((void *)v4[0xc]);
        free((void *)v4[0xd]);
        free((void *)v4[0xf]);
        free(v4);
        if (v5) {
          *v5 = (int8)v1;
          return;
        }
        dat_82320 = v1;
        return;
      }
    }
    if (!v1) break;
    v5 = v4;
  }
  return;
}


// Function: sub_1a1e0 @ 0x1a1e0
void sub_1a1e0(void) // return-dupe
{
  unsigned int v1;
  char v2;
  int4 v3;
  int4 v4; // eax
  char *v5;
  char *v6; // rax
  unsigned long v7; // rax
  code *v8; // rbx
  
  dat_822f0 = sub_1a080;
  sub_ccd0(dat_82408);
  if ((!dat_82b92) && (v2 = sub_29420(dat_82840), v2)) {
    if (dat_82d50)
      (*dat_82d50)();
    v5 = (char *)sub_4ce40(dat_82840);
    error(0,0,dcgettext(NULL,"%s: Member name contains \'..\'",5),v5);
    dat_82d58 = 2;
    sub_24660(); // return-dupe, tail-call
    return;
  }
  if ((!*dat_82848) || ((dat_82b40 && (!sub_2df50("extract"))))) {
    sub_24660();
    return;
  }
  v1 = dat_82580;
  if (dat_82a54) {
    sub_23bc0((struct_9 *)0x82840,dat_82408,-1);
    v1 = dat_82580;
  }
  dat_82580 = v1;
  if (!dat_827e0) {
    sub_19da0(dat_82848,0);
    sub_26300(v1);
  }
  if ((dat_82b84) && (v2 = sub_25c10(dat_82848,0), !v2)) {
    v3 = *__errno_location();
    if (dat_82d50)
      (*dat_82d50)();
    v5 = (char *)sub_4ce40(dat_82848);
    error(0,v3,dcgettext(NULL,"%s: Was unable to backup this file",5),v5);
    dat_82d58 = 2;
    sub_24660();
    return;
  }
  v2 = sub_2b3a0(0x82840);
  v5 = dat_82848;
  if (v2) {
    v3 = 0x53;
    goto label_1a28c;
  }
  v2 = *(char *)(dat_82408 + 0x9c);
  v3 = (int4)v2;
  switch(v2) {
    case 0:
    case 0x30:
    case 0x37:
      if (dat_82850) {
        v8 = sub_198d0;
        break;
      }
      goto label_1a28c;
    default:
      if (dat_81b82 & 1) {
        if (dat_82d50)
          (*dat_82d50)();
        v6 = (char *)sub_4ce40(v5);
        error(0,0,dcgettext(NULL,"%s: Unknown file type \'%c\', extracted as normal file",5),v6,v2);
      }
      goto label_1a28c;
    case 0x31:
      v3 = 0x31;
      v8 = sub_18fd0;
      break;
    case 0x32:
      v3 = 0x32;
      v8 = sub_19270;
      break;
    case 0x33:
      dat_828b0 |= 0x2000;
      v3 = 0x33;
      v8 = sub_18c10;
      break;
    case 0x34:
      dat_828b0 |= 0x6000;
      v3 = 0x34;
      v8 = sub_18c10;
      break;
    case 0x35:
    case 0x44:
      v8 = sub_198d0;
      if (dat_829c8)
        dat_827e0 = '\x01';
      break;
    case 0x36:
      v3 = 0x36;
      v8 = sub_18b30;
      break;
    case 0x4b:
    case 0x4c:
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Unexpected long name header",5));
      dat_82d58 = 2;
      goto label_1a320;
    case 0x4d:
      if (dat_82d50)
        (*dat_82d50)(dat_82848);
      v7 = sub_4ce40(dat_82848);
      error(0,0,dcgettext(NULL,"%s: Cannot extract -- file is continued from another volume",5),v7);
      dat_82d58 = 2;
      goto label_1a320;
    case 0x53:
      v3 = 0x53;
label_1a28c:
      if ((!dat_82a6a) && (!dat_82a60)) {
        v8 = sub_19380;
        goto label_1a2ae;
      }
      v8 = sub_19380;
      goto label_1a4be;
    case 0x56:
      goto label_1a320;
    
  }
  if (!dat_82a6a) {
    if (!dat_82a60) {
label_1a2ae:
      if (dat_82b34 != 3) { // branch-flip
        if ((dat_82b34 == 6) && (sub_188c0(v5,NULL))) {
          if (dat_81b81 & 8) {
            if (dat_82d50)
              (*dat_82d50)();
            v5 = (char *)sub_4d2d0(v5);
            error(0,0,dcgettext(NULL,"Current %s is newer or same age",5),v5);
          }
          goto label_1a320;
        }
      }
      else {
        v4 = sub_26a40(v5,(uint1)dat_82aba);
        if ((!v4) && (*__errno_location() & 0xfffffffdU)) {
          sub_33490(v5);
          goto label_1a320;
        }
      }
label_1a4be:
      v3 = (*v8)(dat_82848,v3);
      if (!v3)
        return;
      goto label_1a325;
    }
    if (v8 == sub_19380) goto label_1a4be;
  }
label_1a320:
  sub_24660();
label_1a325:
  if (dat_82b84) {
    sub_25ea0(); // tail-call
    return;
  }
  return;
}


// Function: sub_1a700 @ 0x1a700
unsigned long sub_1a700(char *a0,char *a1) // return-dupe x3
{
  unsigned long *v1;
  int4 v2; // eax
  int4 *v3; // rax
  char *v4;
  char *v5; // rax
  int8 v6;
  int4 v7;
  
  v2 = renameat(dat_7f234,a0,dat_7f234,a1);
  v7 = dat_82580;
  v1 = dat_82320;
  if (!v2) {
    do {
      if (!v1)
        return 1;
      if (*(int4 *)((int8)v1 + 0x5c) == v7) {
        v4 = (char *)v1[0x14];
        if (!strcmp(v4,a0)) {
          free(v4);
          v1[0x14] = sub_4ee30(a1);
          v1[0x13] = strlen(a1);
          return 1;
        }
      }
      v1 = (unsigned long *)*v1;
    } while( true );
  }
  v3 = __errno_location();
  v7 = *v3;
  if ((v7 == 2) && (!sub_17ec0(a1,&v6))) {
    if (!renameat(dat_7f234,a0,dat_7f234,a1))
      return 1;
    v7 = *v3;
  }
  if (dat_82d50)
    (*dat_82d50)();
  v4 = (char *)sub_4d2b0(1,a1);
  v5 = (char *)sub_4d2b0(0,a0);
  error(0,v7,dcgettext(NULL,"Cannot rename %s to %s",5),v5,v4);
  dat_82d58 = 2;
  return 0;
}


// Function: sub_1a8a0 @ 0x1a8a0
void sub_1a8a0(void)
{
  return;
}


// Function: sub_1a8b0 @ 0x1a8b0
void sub_1a8b0(void)
{
  return;
}


// Function: sub_1a8c0 @ 0x1a8c0
void sub_1a8c0(unsigned long *a0,unsigned long *a1,char *a2) // return-dupe
{
  char *v1; // stack - 0x28
  char v2 [16];
  
  v2 = sub_25930(a2,&v1,0);
  if ((0 <= SUB168(v2,8)) && (!*v1)) {
    *a1 = SUB168(v2,0);
    sub_25120(a0,a2);
    return;
  }
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"Time stamp is out of allowed range",5));
  dat_82d58 = 2;
  return;
}


// Function: sub_1a970 @ 0x1a970
void sub_1a970(int8 a0,unsigned long a1,void *a2,uint8 a3)
{
  void *v1; // rax
  
  v1 = (void *)sub_4ecc0(a3);
  *(void **)(a0 + 400) = v1;
  memcpy(v1,a2,a3); // tail-call
  return;
}


// Function: sub_1a9b0 @ 0x1a9b0
void sub_1a9b0(struct_4 *a0,void *a1,uint8 a2)
{
  obstack *v1;
  void *v2;
  
  v1 = a0->field_0x0;
  v2 = *(void **)&v1->field_0x18;
  if ((uint8)(*(int8 *)&v1->field_0x20 - (int8)v2) < a2) {
    _obstack_newchunk(v1,a2);
    v2 = *(void **)&v1->field_0x18;
  }
  memcpy(v2,a1,a2);
  *(uint8 *)&v1->field_0x18 = *(int8 *)&v1->field_0x18 + a2;
  a0->field_0x8 = a0->field_0x8 + a2;
  return;
}


// Function: sub_1aa20 @ 0x1aa20
void sub_1aa20(int8 *a0,int8 *a1,char *a2,void *a3,int8 a4)
{
  void *v1;
  int8 v2;
  uint8 v3; // rbx
  int8 v4; // rbx
  
  v2 = *a1;
  v1 = (void *)*a0;
  *a1 = v2 + 1;
  v3 = (v2 + 1) * 0x18;
  v4 = v3 - 0x18;
  v2 = sub_4ece0(v1,v3);
  *a0 = v2;
  *(unsigned long *)(v2 + v4) = sub_4ee30(a2);
  v2 = *a0;
  *(unsigned long *)(v2 + v4 + 8) = sub_4edf0(a3,a4 + 1);
  *(int8 *)(*a0 + 0x10 + v4) = a4;
  return;
}


// Function: sub_1aaa0 @ 0x1aaa0
void sub_1aaa0(int8 a0,unsigned long a1,void *a2,int8 a3)
{
  unsigned long v1; // rax
  
  v1 = sub_4edf0(a2,a3 + 1);
  *(int8 *)(a0 + 0x50) = a3;
  *(unsigned long *)(a0 + 0x48) = v1;
  return;
}


// Function: sub_1aad0 @ 0x1aad0
void sub_1aad0(int8 a0,unsigned long a1,void *a2,int8 a3)
{
  unsigned long v1; // rax
  
  v1 = sub_4edf0(a2,a3 + 1);
  *(int8 *)(a0 + 0x40) = a3;
  *(unsigned long *)(a0 + 0x38) = v1;
  return;
}


// Function: sub_1ab00 @ 0x1ab00
int8 sub_1ab00(char *a0)
{
  int4 v1; // eax
  char *v2;
  int8 v3; // r12
  
  v3 = 0x7d260;
  v2 = "atime";
  while( true ) {
    if (*(char *)(v3 + 0x1c)) // branch-flip
      v1 = strncmp(v2,a0,strlen(v2));
    else {
      v1 = strcmp(v2,a0);
    }
    if (!v1) break;
    v2 = *(char **)(v3 + 0x20);
    v3 += 0x20;
    if (!v2)
      return 0;
  }
  return v3;
}


// Function: sub_1ab80 @ 0x1ab80
void sub_1ab80(int8 *a0,unsigned long a1)
{
  char *v1;
  struct_10 *v2; // rax
  uint8 v3; // rax
  
  if (!a0)
    return;
  do {
    v2 = (struct_10 *)sub_1ab00((char *)a0[1]);
    if (v2) {
      v1 = (char *)a0[2];
      v3 = strlen(v1);
      (*v2->field_0x10)(a1,v2->field_0x0,v1,v3);
    }
    a0 = (int8 *)*a0;
  } while (a0);
  return;
}


// Function: sub_1abf0 @ 0x1abf0
void sub_1abf0(char *a0,char *a1,uint8 a2,uint8 a3)
{
  char *v1; // rax
  char *v2; // rax
  char v3 [32];
  char v4 [24];
  
  v1 = (char *)sub_45230(a2,v3);
  v2 = (char *)sub_45370(a3,v4);
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"Extended header %s=%s is out of range %s..%s",5),a0,a1,v1,v2);
  dat_82d58 = 2;
  return;
}


// Function: sub_1aca0 @ 0x1aca0
unsigned long sub_1aca0(struct_74 *a0,char *a1,char *a2) // return-dupe x2
{
  char *v1; // stack - 0x28
  char v2 [16];
  
  v2 = sub_25930(a1,&v1,1);
  if (0 <= SUB168(v2,8)) {
    a0->field_0x8 = SUB168(v2,8);
    a0->field_0x0 = SUB168(v2,0);
    return 1;
  }
  if ((a1 < v1) && (!*v1)) {
    sub_1abf0(a2,a1,0x8000000000000000,0x7fffffffffffffff);
    return 0;
  }
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"Malformed extended header: invalid %s=%s",5),a2,a1);
  dat_82d58 = 2;
  return 0;
}


// Function: sub_1ad90 @ 0x1ad90
void sub_1ad90(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1aca0(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0xf8) = v1;
  *(unsigned long *)(a0 + 0x100) = v2;
  return;
}


// Function: sub_1adf0 @ 0x1adf0
void sub_1adf0(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1aca0(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0x108) = v1;
  *(unsigned long *)(a0 + 0x110) = v2;
  return;
}


// Function: sub_1ae50 @ 0x1ae50
void sub_1ae50(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1aca0(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0xe8) = v1;
  *(unsigned long *)(a0 + 0xf0) = v2;
  return;
}


// Function: sub_1aeb0 @ 0x1aeb0
void sub_1aeb0(int8 a0,char *a1,char *a2)
{
  uint8 v1;
  unsigned long v10;
  bool v2;
  bool v3;
  int4 *v4; // rax
  unsigned long v5;
  unsigned long *v6; // rdx
  char *v7; // stack - 0x48
  char *v8; // rsi
  char v9;
  
  *(unsigned long *)(a0 + 0x130) = 0;
  if ((uint4)((int4)*a2 - 0x30U) <= 9) {
    v4 = __errno_location();
    v2 = 1;
    do {
      *v4 = 0;
      v5 = strtoimax(a2,&v7,10);
      if (v2) {
        if (*v4 == 0x22) {
          sub_1abf0(a1,a2,0,0x7fffffffffffffff); // return-dupe
          return;
        }
      }
      else {
        if (*v4 == 0x22) {
          sub_1abf0(a1,a2,0,0x7fffffffffffffff);
          return;
        }
        v1 = *(uint8 *)(a0 + 0x130);
        if (*(uint8 *)(a0 + 0x138) <= v1) {
          if (dat_82d50)
            (*dat_82d50)();
          v8 = "Malformed extended header: excess %s=%s";
          error(0,0,dcgettext(NULL,v8,5),a1,a2); // return-dupe
          dat_82d58 = 2;
          return;
        }
        v6 = (unsigned long *)(v1 * 0x10 + *(int8 *)(a0 + 0x140));
        *(uint8 *)(a0 + 0x130) = v1 + 1;
        *v6 = v10;
        v6[1] = v5;
        v5 = v10;
      }
      v3 = (bool)(v2 ^ 1);
      v9 = *v7;
      if (!v9) {
        if (v2) {
          if (dat_82d50)
            (*dat_82d50)();
          error(0,0,dcgettext(NULL,"Malformed extended header: invalid %s: odd number of values",5),a1);
          dat_82d58 = 2;
          return;
        }
        return;
      }
      if (v9 != ',') {
        if (dat_82d50) {
          (*dat_82d50)();
          v9 = *v7;
        }
        error(0,0,dcgettext(NULL,"Malformed extended header: invalid %s: unexpected delimiter %c",5),a1,v9);
        dat_82d58 = 2;
        return;
      }
      a2 = &v7[1];
      v10 = v5;
      v2 = v3;
    } while ((uint4)((int4)v7[1] - 0x30U) <= 9);
  }
  if (dat_82d50)
    (*dat_82d50)();
  v8 = "Malformed extended header: invalid %s=%s";
  error(0,0,dcgettext(NULL,v8,5),a1,a2);
  dat_82d58 = 2;
  return;
}


// Function: sub_1b1f0 @ 0x1b1f0
void sub_1b1f0(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b100(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x78) = v1[0];
  return;
}


// Function: sub_1b250 @ 0x1b250
void sub_1b250(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b100(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x74) = v1[0];
  return;
}


// Function: sub_1b610 @ 0x1b610
void sub_1b610(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b100(&v1,a2,0x7fffffffffffffff,a1))
    return;
  *(unsigned long *)(a0 + 0x88) = v1;
  return;
}


// Function: sub_1b670 @ 0x1b670
void sub_1b670(unsigned long a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b100(&v1,a2,0xffffffffffffffff,a1))
    return;
  dat_81f08 = v1;
  return;
}


// Function: sub_1b6d0 @ 0x1b6d0
void sub_1b6d0(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b100(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x128) = v1[0];
  return;
}


// Function: sub_1b730 @ 0x1b730
void sub_1b730(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b100(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x124) = v1[0];
  return;
}


// Function: sub_1b790 @ 0x1b790
void sub_1b790(unsigned long a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b100(&v1,a2,0xffffffffffffffff,a1))
    return;
  dat_81f00 = v1;
  return;
}


// Function: sub_1b7f0 @ 0x1b7f0
void sub_1b7f0(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b100(&v1,a2,0x7fffffffffffffff,a1))
    return;
  *(char *)(a0 + 0x150) = 1;
  *(unsigned long *)(a0 + 0x148) = v1;
  return;
}


// Function: sub_1b860 @ 0x1b860
void sub_1b860(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x18
  
  if (!sub_1b100(&v2,a2,0xffffffffffffffff,a1))
    return;
  *(unsigned long *)(a0 + 0x138) = v2;
  v1 = sub_4edb0(v2,0x10);
  *(unsigned long *)(a0 + 0x130) = 0;
  *(unsigned long *)(a0 + 0x140) = v1;
  return;
}


// Function: sub_1bab0 @ 0x1bab0
void sub_1bab0(unsigned long *a0,char *a1,char *a2,unsigned long a3)
{
  unsigned long v1;
  int8 v2; // rax
  unsigned long *v3; // rax
  unsigned long v4;
  
  v2 = sub_1ab00(a1);
  if ((v2) && (*(uint1 *)(v2 + 0x18) & 2)) {
    (**(code **)(v2 + 0x10))(a0,a1,a2,a3); // jump-as-call
    return;
  }
  v3 = (unsigned long *)sub_4ecc0(0x18);
  v3[1] = sub_4ee30(a1);
  v4 = 0;
  if (a2)
    v4 = sub_4ee30(a2);
  v1 = *a0;
  v3[2] = v4;
  *a0 = v3;
  *v3 = v1;
  return;
}


// Function: sub_1bb40 @ 0x1bb40
void sub_1bb40(struct_4 *a0,char *a1,void *a2,uint8 a3)
{
  obstack *v1;
  char v10;
  char *v11; // r15
  bool v12; // zf
  char *v2;
  int8 v3; // rax
  char *v4;
  char v5 [20];
  char v6 [4];
  uint8 v7;
  uint8 v8; // rax
  char *v9;
  
  if (dat_82340) { // branch-flip
    *dat_82340 = '\0';
    v10 = *a1;
    v9 = dat_82340;
  }
  else {
    dat_82338 = 0x100;
    v9 = (char *)sub_4ecc0(0x100);
    v10 = *a1;
  }
  v2 = v9;
  dat_82340 = v9;
  if (v10) {
    v8 = 0;
    do {
      v7 = v8 + 2;
      if (dat_82338 <= v7) { // branch-flip
        v2 = (char *)sub_4ed20(v2,(uint8 *)0x82338);
        v9 = &v2[v8];
        dat_82340 = v2;
        if (v10 != '%') goto label_1bba2;
label_1bbed:
        builtin_strncpy(v9,"%25",4);
      }
      else {
        v9 = &v2[v8];
        if (v10 == '%') goto label_1bbed;
label_1bba2:
        if (v10 != '=') { // branch-flip
          *v9 = v10;
          v7 = v8;
        }
        else {
          builtin_strncpy(v9,"%3D",4);
        }
      }
      v10 = a1[1];
      a1 = &a1[1];
      v8 = v7 + 1;
    } while (v10);
    v9 = &v2[v8];
  }
  *v9 = '\0';
  v8 = strlen(v2);
  v4 = NULL;
  do {
    v3 = sub_45370(&v4[v8 + 3 + a3],v5);
    v11 = &v6[-v3];
    v12 = v4 != v11;
    v4 = v11;
  } while (v12);
  sub_1a9b0(a0,v3,v11);
  v1 = a0->field_0x0;
  v4 = *(char **)&v1->field_0x18;
  if (*(char **)&v1->field_0x20 == v4) {
    _obstack_newchunk(v1,1);
    v4 = *(char **)&v1->field_0x18;
  }
  *(char **)&v1->field_0x18 = &v4[1];
  *v4 = 0x20;
  a0->field_0x8 = a0->field_0x8 + 1;
  sub_1a9b0(a0,v2,v8);
  v1 = a0->field_0x0;
  v4 = *(char **)&v1->field_0x18;
  if (*(char **)&v1->field_0x20 == v4) {
    _obstack_newchunk(v1,1);
    v4 = *(char **)&v1->field_0x18;
  }
  *(char **)&v1->field_0x18 = &v4[1];
  *v4 = 0x3d;
  a0->field_0x8 = a0->field_0x8 + 1;
  sub_1a9b0(a0,a2,a3);
  v1 = a0->field_0x0;
  v4 = *(char **)&v1->field_0x18;
  if (*(char **)&v1->field_0x20 == v4) {
    _obstack_newchunk(v1,1);
    v4 = *(char **)&v1->field_0x18;
  }
  *(char **)&v1->field_0x18 = &v4[1];
  *v4 = 10;
  a0->field_0x8 = a0->field_0x8 + 1;
  return;
}


// Function: sub_1bdb0 @ 0x1bdb0
void sub_1bdb0(char *a0,char *a1,struct_4 *a2)
{
  char *v1;
  char *v2; // stack - 0x38
  
  v1 = v2;
  if (!sub_31920(1,a0,&v2)) {
    v1 = (char *)sub_4ee30(a0);
    v2 = v1;
  }
  sub_1bb40(a2,a1,v1,strlen(v1));
  free(v2);
  return;
}


// Function: sub_1be50 @ 0x1be50
void sub_1be50(int8 a0,char *a1,struct_4 *a2)
{
  sub_1bdb0(*(char **)(a0 + 0x30),a1,a2); // tail-call
  return;
}


// Function: sub_1bf20 @ 0x1bf20
void sub_1bf20(unsigned long a0,char *a1,struct_4 *a2,char *a3)
{
  sub_1bb40(a2,a1,a3,sub_1f690(a3)); // tail-call
  return;
}


// Function: sub_1bf50 @ 0x1bf50
void sub_1bf50(int8 a0,char *a1,struct_4 *a2,struct_46 *a3)
{
  char *v1; // rax
  char v2 [40];
  uint8 v3;
  uint8 v4;
  
  if (a3) { // branch-flip
    v4 = a3->field_0x0;
    v3 = a3->field_0x8;
  }
  else {
    v4 = *(uint8 *)(a0 + 0xf8);
    v3 = *(uint8 *)(a0 + 0x100);
  }
  v1 = (char *)sub_257a0(v4,v3,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1bfe0 @ 0x1bfe0
void sub_1bfe0(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [40];
  
  v1 = (char *)sub_257a0(*(uint8 *)(a0 + 0x108),*(unsigned long *)(a0 + 0x110),v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c060 @ 0x1c060
void sub_1c060(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [40];
  
  v1 = (char *)sub_257a0(*(uint8 *)(a0 + 0xe8),*(unsigned long *)(a0 + 0xf0),v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c0e0 @ 0x1c0e0
void sub_1c0e0(int8 a0,char *a1,struct_4 *a2,int8 *a3)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(uint8 *)(*a3 * 0x10 + *(int8 *)(a0 + 0x140)),0,0xffffffffffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c170 @ 0x1c170
void sub_1c170(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(unsigned int *)(a0 + 0x124),0,0xffffffffffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c1f0 @ 0x1c1f0
void sub_1c1f0(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(unsigned int *)(a0 + 0x128),0,0xffffffffffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c370 @ 0x1c370
void sub_1c370(unsigned long a0,char *a1,struct_4 *a2,uint8 *a3)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*a3,0,0xffffffffffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c3f0 @ 0x1c3f0
void sub_1c3f0(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(unsigned int *)(a0 + 0x74),0,0xffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c4e0 @ 0x1c4e0
void sub_1c4e0(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(uint8 *)(a0 + 0x130),0,0xffffffffffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c560 @ 0x1c560
void sub_1c560(int8 a0,char *a1,struct_4 *a2)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(unsigned int *)(a0 + 0x78),0,0xffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c5d0 @ 0x1c5d0
void sub_1c5d0(int8 a0,char *a1,struct_4 *a2,int8 *a3)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_255c0(*(uint8 *)(*a3 * 0x10 + *(int8 *)(a0 + 0x140) + 8),0,0xffffffffffffffff,v2);
  sub_1bb40(a2,a1,v1,strlen(v1));
  return;
}


// Function: sub_1c660 @ 0x1c660
void sub_1c660(unsigned long a0,unsigned long a1,char *a2)
{
  if (dat_81f10) {
    free(dat_81f10);
    dat_81f10 = NULL;
  }
  if (sub_31920(0,a2,(unsigned long *)0x81f10))
    return;
  sub_25120((unsigned long *)0x81f10,a2); // tail-call
  return;
}


// Function: sub_1c6d0 @ 0x1c6d0
void sub_1c6d0(unsigned long a0,unsigned long a1,char *a2)
{
  if (dat_81f18) {
    free(dat_81f18);
    dat_81f18 = NULL;
  }
  if (sub_31920(0,a2,(unsigned long *)0x81f18))
    return;
  sub_25120((unsigned long *)0x81f18,a2); // tail-call
  return;
}


// Function: sub_1c740 @ 0x1c740
void sub_1c740(int8 a0,unsigned long a1,char *a2)
{
  if (*(void **)(a0 + 0x18)) {
    free(*(void **)(a0 + 0x18));
    *(unsigned long *)(a0 + 0x18) = 0;
  }
  if (sub_31920(0,a2,(unsigned long *)(a0 + 0x18)))
    return;
  sub_25120((unsigned long *)(a0 + 0x18),a2); // tail-call
  return;
}


// Function: sub_1c790 @ 0x1c790
void sub_1c790(int8 a0,unsigned long a1,char *a2)
{
  if (*(void **)(a0 + 0x30)) {
    free(*(void **)(a0 + 0x30));
    *(unsigned long *)(a0 + 0x30) = 0;
  }
  if (sub_31920(0,a2,(unsigned long *)(a0 + 0x30)))
    return;
  sub_25120((unsigned long *)(a0 + 0x30),a2); // tail-call
  return;
}


// Function: sub_1c7e0 @ 0x1c7e0
void sub_1c7e0(int8 a0,unsigned long a1,char *a2)
{
  if (*(void **)(a0 + 0x20)) {
    free(*(void **)(a0 + 0x20));
    *(unsigned long *)(a0 + 0x20) = 0;
  }
  if (sub_31920(0,a2,(unsigned long *)(a0 + 0x20)))
    return;
  sub_25120((unsigned long *)(a0 + 0x20),a2); // tail-call
  return;
}


// Function: sub_1c830 @ 0x1c830
void sub_1c830(int8 a0,unsigned long a1,char *a2)
{
  if (*(void **)(a0 + 0x28)) {
    free(*(void **)(a0 + 0x28));
    *(unsigned long *)(a0 + 0x28) = 0;
  }
  if (sub_31920(0,a2,(unsigned long *)(a0 + 0x28)))
    return;
  sub_25120((unsigned long *)(a0 + 0x28),a2); // tail-call
  return;
}


// Function: sub_1c880 @ 0x1c880
void sub_1c880(struct_75 *a0,unsigned long a1,char *a2)
{
  char *v1;
  
  a0->field_0x151 = 1;
  if (a0->field_0x0) {
    free(a0->field_0x0);
    a0->field_0x0 = NULL;
  }
  if (!sub_31920(0,a2,a0))
    sub_25120(a0,a2);
  if (a0->field_0x8) {
    free(a0->field_0x8);
    a0->field_0x8 = NULL;
  }
  if (!sub_31920(0,a2,&a0->field_0x8))
    sub_25120(&a0->field_0x8,a2);
  v1 = a0->field_0x8;
  a0->field_0x10 = sub_403c0(v1);
  return;
}


// Function: sub_1c920 @ 0x1c920
void sub_1c920(struct_76 *a0,unsigned long a1,char *a2)
{
  char *v1;
  
  if (a0->field_0x151)
    return;
  if (a0->field_0x0) {
    free(a0->field_0x0);
    a0->field_0x0 = NULL;
  }
  if (!sub_31920(0,a2,a0))
    sub_25120(a0,a2);
  if (a0->field_0x8) {
    free(a0->field_0x8);
    a0->field_0x8 = NULL;
  }
  if (!sub_31920(0,a2,&a0->field_0x8))
    sub_25120(&a0->field_0x8,a2);
  v1 = a0->field_0x8;
  a0->field_0x10 = sub_403c0(v1);
  return;
}


// Function: sub_1ca20 @ 0x1ca20
void sub_1ca20(char *a0) // return-dupe
{
  uint1 v1;
  uint1 *v10;
  uint1 *v2; // rax
  uint1 *v3; // rax
  int8 *v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // rax
  int8 v7; // rdx
  char *v8;
  uint1 v9;
  
  v2 = (uint1 *)strtok(a0,",");
  if (!v2)
    return;
  do {
    v3 = (uint1 *)strchr((char *)v2,0x3d);
    if (!v3) {
      if (dat_82d50)
        (*dat_82d50)();
      v8 = dcgettext(NULL,"Keyword %s is unknown or not yet implemented",5);
      error(0,0,v8,v2); // return-dupe
      sub_2dea0(2); // no-return
    }
    if (v3 == v2) {
      if (dat_82d50)
        (*dat_82d50)();
      v2 = (uint1 *)sub_4d2d0(v2);
      v8 = "Malformed pax option: %s";
label_1cbfc:
      v8 = dcgettext(NULL,v8,5);
      error(0,0,v8,v2);
      sub_2dea0(2);
    }
    v1 = v3[-1];
    v10 = v3;
    if (v1 == 0x3a)
      v10 = &v3[-1];
    if (v2 < v10) {
      v4 = __ctype_b_loc();
      do {
        if (!(*(uint1 *)(*v4 + 1 + (uint8)*v10 * 2) & 0x20)) break;
        v10 = &v10[-1];
      } while (v10 != v2);
    }
    *v10 = 0;
    v10 = &v3[1];
    v9 = v3[1];
    if (v9) {
      v4 = __ctype_b_loc();
      do {
        if (!(*(uint1 *)(*v4 + 1 + (uint8)v9 * 2) & 0x20)) break;
        v9 = v10[1];
        v10 = &v10[1];
      } while (v9);
    }
    if (strcmp((char *)v2,"delete")) { // branch-flip
      if (strcmp((char *)v2,"exthdr.name")) { // branch-flip
        if (strcmp((char *)v2,"globexthdr.name")) { // branch-flip
          if (strcmp((char *)v2,"exthdr.mtime")) { // branch-flip
            if (strcmp((char *)v2,"globexthdr.mtime")) { // branch-flip
              v8 = "atime";
              v7 = 0x7d260;
              do {
                if (((!*(char *)(v7 + 0x1c)) && (*(uint1 *)(v7 + 0x18) & 1)) && (!strcmp(v8,(char *)v2))) {
                  if (dat_82d50)
                    (*dat_82d50)();
                  v8 = dcgettext(NULL,"Keyword %s cannot be overridden",5);
                  error(0,0,v8,v2);
                  sub_2dea0(2);
                }
                v8 = *(char **)(v7 + 0x20);
                v7 += 0x20;
              } while (v8);
              if (v1 == 0x3a) {
                v6 = (unsigned long *)sub_4ecc0(0x18);
                v6[1] = sub_4ee30(v2);
                v6[2] = sub_4ee30(v10);
                *v6 = dat_82380;
                dat_82380 = v6;
              }
              else {
                v6 = (unsigned long *)sub_4ecc0(0x18);
                v6[1] = sub_4ee30(v2);
                v6[2] = sub_4ee30(v10);
                *v6 = dat_82388;
                dat_82388 = v6;
              }
            }
            else {
              sub_1a8c0((unsigned long *)0x82350,(unsigned long *)0x82348,v10);
            }
          }
          else {
            sub_1a8c0((unsigned long *)0x82368,(unsigned long *)0x82360,v10);
          }
        }
        else {
          sub_25120((unsigned long *)0x82358,v10);
        }
      }
      else {
        sub_25120((unsigned long *)0x82370,v10);
      }
    }
    else {
      v8 = "atime";
      v7 = 0x7d260;
      do {
        if (((!*(char *)(v7 + 0x1c)) && (*(uint1 *)(v7 + 0x18) & 1)) && (!fnmatch((char *)v10,v8,0))) {
          if (dat_82d50)
            (*dat_82d50)();
          v2 = (uint1 *)sub_4d2d0(v10);
          v8 = "Pattern %s cannot be used";
          goto label_1cbfc;
        }
        v8 = *(char **)(v7 + 0x20);
        v7 += 0x20;
      } while (v8);
      v6 = (unsigned long *)sub_4ecc0(0x18);
      v5 = sub_4ee30(v10);
      v6[2] = 0;
      v6[1] = v5;
      *v6 = dat_82390;
      dat_82390 = v6;
    }
    v2 = (uint1 *)strtok(NULL,",");
    if (!v2)
      return;
  } while( true );
}


// Function: sub_1cdf0 @ 0x1cdf0
char * sub_1cdf0(unsigned long *a0,char *a1,uint8 a2)
{
  char *v1;
  char v10 [20];
  char v11 [12];
  char v12 [4];
  char *v13;
  char *v14;
  char *v15; // stack - 0xa8
  char *v16; // stack - 0xa0
  char v2;
  char v3;
  char *v4; // rax
  char *v5;
  char *v6;
  char *v7; // rax
  char *v8; // rax
  char v9 [20];
  
  v3 = *a1;
  if (v3) { // branch-flip
    v7 = NULL;
    v15 = NULL;
    v4 = NULL;
    v8 = NULL;
    v14 = NULL;
    v16 = NULL;
    v13 = a1;
    do {
      while( true ) {
        v2 = v13[1];
        v5 = &v13[1];
        if ((v3 == '%') && (v2)) break;
        v14 = &v14[1];
        v13 = v5;
        v3 = v2;
label_1ce5e:
        if (!v3) goto label_1cef8;
      }
      if (v2 == 'f') {
        if (a0) { // branch-flip
          v7 = (char *)sub_400d0((char *)*a0);
          v5 = &v13[2];
          v14 = &v14[strlen(v7)];
          v1 = &v13[2];
          v13 = v5;
          v3 = *v1;
        }
        else {
label_1d158:
          v5 = &v13[2];
          v13 = &v13[2];
          v3 = *v5;
        }
        goto label_1ce5e;
      }
      if ('f' < v2) {
        if (v2 != 'n') { // branch-flip
          if (v2 != 'p') goto label_1d0aa;
          v5 = (char *)sub_45370((int8)getpid(),v10);
          v15 = v5;
          v6 = v11;
        }
        else {
          v5 = (char *)sub_45370(a2,v9);
          v16 = v5;
          v6 = v12;
        }
        v1 = &v13[2];
        v14 = &v6[(int8)v14 - (int8)v5];
        v5 = &v13[2];
        v13 = v1;
        v3 = *v5;
        goto label_1ce5e;
      }
      if (v2 == '%') {
        v5 = &v13[2];
        v14 = &v14[1];
        v13 = &v13[2];
        v3 = *v5;
        goto label_1ce5e;
      }
      if (v2 != 'd') {
label_1d0aa:
        v5 = &v13[2];
        v14 = &v14[2];
        v13 = &v13[2];
        v3 = *v5;
        goto label_1ce5e;
      }
      if (!a0) goto label_1d158;
      if (!v8)
        v8 = (char *)sub_40230((char *)*a0);
      v4 = (char *)sub_33640(v8,0,(uint1)dat_82b92);
      v14 = &v14[strlen(v4)];
      v3 = v13[2];
      v13 = &v13[2];
    } while (v3);
label_1cef8:
    v14 = &v14[1];
  }
  else {
    v15 = NULL;
    v7 = NULL;
    v4 = NULL;
    v8 = NULL;
    v16 = NULL;
    v14 = (char *)0x1;
  }
  v5 = (char *)sub_4ecc0(v14);
  v3 = *a1;
  v13 = v5;
  if (v3) {
label_1cf30:
    v2 = a1[1];
    if (v3 != '%') { // branch-flip
      *v13 = v3;
      a1 = &a1[1];
      v13 = &v13[1];
      v3 = v2;
    }
    else {
      if (v2 != 'f') { // branch-flip
        if (v2 <= 'f') {
          if (v2 != '%') {
            if (v2 == 'd') {
              if (v4)
                v13 = stpcpy(v13,v4);
              goto label_1cf6d;
            }
            *v13 = '%';
            if (v2) {
label_1d0c0:
              v13[1] = v2;
              v1 = &a1[2];
              v13 = &v13[2];
              a1 = &a1[2];
              v3 = *v1;
              goto label_1cf78;
            }
            v13 = &v13[1];
            goto label_1cf80;
          }
          *v13 = '%';
          v1 = &a1[2];
          v13 = &v13[1];
          a1 = &a1[2];
          v3 = *v1;
          goto label_1cf78;
        }
        if (v2 != 'n') { // branch-flip
          if (v2 != 'p') {
            *v13 = '%';
            goto label_1d0c0;
          }
          v13 = stpcpy(v13,v15);
        }
        else {
          v13 = stpcpy(v13,v16);
        }
label_1d03d:
        v3 = a1[2];
      }
      else {
        if (v7) {
          v13 = stpcpy(v13,v7);
          goto label_1d03d;
        }
label_1cf6d:
        v3 = a1[2];
      }
      a1 = &a1[2];
    }
label_1cf78:
    if (!v3) goto label_1cf80;
    goto label_1cf30;
  }
  free(v8);
  goto label_1cf9f;
label_1cf80:
  free(v8);
  if (v5 < v13) {
    do {
      if (v13[-1] != '/') break;
      v13 = &v13[-1];
    } while (v5 != v13);
  }
label_1cf9f:
  *v13 = '\0';
  return v5;
}


// Function: sub_1d1d0 @ 0x1d1d0
void sub_1d1d0(struct_6 *a0)
{
  if (dat_82370) {
    sub_1cdf0(a0,dat_82370,0); // tail-call
    return;
  }
  sub_25120((unsigned long *)0x82370,*(char **)((uint8)dat_82a38 * 8 + 0x7d640));
  sub_1cdf0(a0,dat_82370,0); // tail-call
  return;
}


// Function: sub_1d220 @ 0x1d220
void sub_1d220(void)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3;
  
  v3 = dat_82358;
  if (!dat_82358) {
    v3 = *(char **)((uint8)dat_82a38 * 8 + 0x7d650);
    v1 = getenv("TMPDIR");
    if (!v1)
      v1 = "/tmp";
    v2 = strlen(v1);
    dat_82358 = (char *)sub_4ecc0(v2 + 1 + strlen(v3));
    v3 = strcat(strcpy(dat_82358,v1),v3);
  }
  sub_1cdf0(NULL,v3,dat_82398 + 1); // tail-call
  return;
}


// Function: sub_1d2d0 @ 0x1d2d0
void sub_1d2d0(uint1 a0,int8 a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  unsigned long *v1;
  int8 v10; // rbx
  unsigned long *v11;
  unsigned long *v12;
  unsigned long *v2;
  uint4 v3; // eax
  uint1 *v4; // rax
  unsigned long *v5;
  void *v6; // rax
  uint8 v7; // rax
  int8 v8; // rcx
  uint8 v9;
  
  v9 = a3[1];
  if (a0 != 0x67) { // branch-flip
    if ((a0 == 0x78) && (dat_82368))
      a2 = dat_82360;
  }
  else if (dat_82350)
    a2 = dat_82348;
  v4 = (uint1 *)sub_13050(a1,v9,a2);
  v3 = (uint4)v9;
  v4[0x9c] = a0;
  sub_13230(v4);
  v10 = a3[2] + v9;
  while( true ) {
    v5 = (unsigned long *)sub_d360();
    v12 = (unsigned long *)(v10 - v9);
    if (v9 == (v3 & 0x1ff)) break;
    v9 -= 0x200;
    v8 = (int8)v5 - (int8)(unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    *v5 = *v12;
    v5[0x3f] = v12[0x3f];
    v7 = (uint8)((int4)v8 + 0x200U >> 3);
    v12 = (unsigned long *)((int8)v12 - v8);
    v11 = (unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    while (v7) {
      v2 = &v11[1];
      v1 = &v12[1];
      *v11 = *v12;
      v7 -= 1;
      v12 = v1;
      v11 = v2;
    }
    sub_ccd0(v5);
    if (!v9) goto label_1d39c;
  }
  v6 = memcpy(v5,v12,v9);
  v7 = 0x200 - v9;
  v5 = (unsigned long *)(v9 + (int8)v6);
  v3 = (uint4)v7;
  if (8 <= v3) { // branch-flip
    *v5 = 0;
    *(unsigned long *)((int8)v5 + ((v7 & 0xffffffff) - 8)) = 0;
    v9 = (uint8)(v3 + ((int4)v5 - (int4)(unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8)) >> 3);
    v5 = (unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    while (v9) {
      v12 = &v5[1];
      *v5 = 0;
      v9 -= 1;
      v5 = v12;
    }
  }
  else if (v7 & 4) { // branch-flip
    *(unsigned int *)v5 = 0;
    *(unsigned int *)((int8)v5 + ((v7 & 0xffffffff) - 4)) = 0;
  }
  else if ((v3) && (*(char *)v5 = 0, v7 & 2))
    *(unsigned short *)((int8)v5 + ((v7 & 0xffffffff) - 2)) = 0;
  sub_ccd0(v6);
label_1d39c:
  if ((obstack *)*a3) { // branch-flip
    _obstack_free((obstack *)*a3,NULL);
    free((void *)*a3);
    *a3 = 0;
  }
  else {
    free((void *)a3[2]);
  }
  a3[2] = 0;
  a3[1] = 0;
  if (a0 != 0x67)
    return;
  dat_82398 += 1;
  return;
}


// Function: sub_1d4e0 @ 0x1d4e0
void sub_1d4e0(void)
{
  if (!dat_82388)
    return;
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"can\'t update global extended header record",5));
  sub_2dea0(2); // no-return
}


// Function: sub_1d530 @ 0x1d530
void sub_1d530(int8 a0)
{
  *(unsigned long *)(a0 + 0x160) = 0;
  *(unsigned long *)(a0 + 0x158) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  *(unsigned long *)(a0 + 0x40) = 0;
  *(unsigned long *)(a0 + 0x48) = 0;
  *(unsigned long *)(a0 + 0x50) = 0;
  *(unsigned long *)(a0 + 0x30) = 0;
  return;
}


// Function: sub_1d580 @ 0x1d580
void sub_1d580(unsigned long *a0,int8 a1)
{
  unsigned long *v1;
  unsigned long *v2; // rbx
  int8 v3; // rbp
  
  if (a1) {
    v3 = 0;
    v1 = a0;
    do {
      v3 += 1;
      v2 = &v1[3];
      free((void *)*v1);
      free((void *)v1[1]);
      v1 = v2;
    } while (a1 != v3);
  }
  free(a0); // tail-call
  return;
}


// Function: sub_1d5d0 @ 0x1d5d0
void sub_1d5d0(int8 a0,char *a1,void *a2,int8 a3)
{
  uint8 v1; // rax
  char *v2; // rax
  
  v1 = strlen(a1);
  v2 = (char *)sub_4ecc0(v1 + 1 + strlen("SCHILY.xattr."));
  builtin_strncpy(v2,"SCHILY.xattr.",0xd);
  strcpy(&v2[0xd],a1);
  sub_1aa20((int8 *)(a0 + 0x160),(int8 *)(a0 + 0x158),v2,a2,a3);
  free(v2); // tail-call
  return;
}


// Function: sub_1d660 @ 0x1d660
void sub_1d660(int8 a0,char *a1,void *a2,int8 a3)
{
  char v1;
  char v10 [8];
  char *v11;
  char *v12; // rsp
  char *v13; // rsp
  char *v14; // rsp
  char *v15;
  char *v16; // rsp
  int8 v2;
  char *v3;
  char *v4;
  uint8 v5; // rax
  char *v6;
  char *v7; // rcx
  uint8 v8; // rdx
  char *v9; // rbx
  
  v11 = v10;
  v5 = strlen(a1);
  v13 = v10;
  v15 = v10;
  while (v13 != &v10[-(v5 + 0x18 & 0xfffffffffffff000)]) {
    v11 = &v15[-0x1000];
    v12 = &v15[-0x1000];
    *(unsigned long *)&v15[-8] = *(unsigned long *)&v15[-8];
    v13 = &v15[-0x1000];
    v15 = v12;
  }
  v8 = (uint8)((uint4)(v5 + 0x18) & 0xff0);
  v2 = -v8;
  v14 = &v11[v2];
  v15 = &v11[v2];
  if (v8)
    *(unsigned long *)&v11[-8] = *(unsigned long *)&v11[-8];
  v9 = (char *)((uint8)&v11[v2 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v11[v2 + -8] = 0x1d6f6;
  memcpy(v9,a1,v5 + 1);
  v4 = &v11[v2];
  while (v14 != &v11[v2 - (a3 + 0x18U & 0xfffffffffffff000)]) {
    v15 = &v4[-0x1000];
    v16 = &v4[-0x1000];
    *(unsigned long *)&v4[-8] = *(unsigned long *)&v4[-8];
    v14 = &v4[-0x1000];
    v4 = v16;
  }
  v5 = (uint8)((uint4)(a3 + 0x18U) & 0xff0);
  v2 = -v5;
  if (v5)
    *(unsigned long *)&v15[-8] = *(unsigned long *)&v15[-8];
  *(unsigned long *)&v15[v2 + -8] = 0x1d756;
  memcpy((void *)((uint8)&v15[v2 + 0xf] & 0xfffffffffffffff0),a2,a3 + 1);
  v6 = v9;
  v3 = v9;
label_1d76f:
  do {
    while( true ) {
      v7 = &v3[1];
      v1 = *v6;
      if (v1 == '%') break;
      *v3 = v1;
      if (!v1) {
        *(unsigned long *)&v15[v2 + -8] = 0x1d7cc;
        v5 = strlen("SCHILY.xattr.");
        *(unsigned long *)&v15[v2 + -8] = 0x1d7de;
        sub_1d5d0(a0,&v9[v5],(void *)((uint8)&v15[v2 + 0xf] & 0xfffffffffffffff0),a3);
        return;
      }
label_1d767:
      v6 = &v6[1];
      v3 = v7;
    }
    if (v6[1] != '3') {
      if ((v6[1] != '2') || (v6[2] != '5')) {
label_1d78b:
        *v3 = '%';
        goto label_1d767;
      }
      *v3 = '%';
      v6 = &v6[3];
      v3 = v7;
      goto label_1d76f;
    }
    if (v6[2] != 'D') goto label_1d78b;
    *v3 = '=';
    v6 = &v6[3];
    v3 = v7;
  } while( true );
}


// Function: sub_1d820 @ 0x1d820
void sub_1d820(int8 a0,int8 *a1,int8 *a2)
{
  unsigned long *v1;
  uint8 v2;
  uint8 v3; // rbx
  
  *a1 = 0;
  *a2 = 0;
  if (*(int8 *)(a0 + 0x158)) {
    v2 = 0;
    do {
      v3 = v2 + 1;
      v1 = (unsigned long *)(*(int8 *)(a0 + 0x160) + v2 * 0x18);
      sub_1aa20(a1,a2,(char *)*v1,(void *)v1[1],v1[2]);
      v2 = v3;
    } while (v3 < *(uint8 *)(a0 + 0x158));
    return;
  }
  return;
}


// Function: sub_1d8a0 @ 0x1d8a0
void sub_1d8a0(int8 a0) // return-dupe
{
  int8 v1; // stack - 0x28
  int8 v2;
  uint8 v3;
  
  sub_1ab80(dat_82388,a0);
  sub_1ab80(dat_82378,a0);
  v3 = *(uint8 *)(a0 + 0x170);
  if (v3) {
    v2 = *(int8 *)(a0 + 0x178);
    v1 = v2 + 0x200;
    while (sub_1b390(v3,v2,&v1,sub_1b9b0,a0)) {
      v3 = *(uint8 *)(a0 + 0x170);
      v2 = *(int8 *)(a0 + 0x178);
    }
  }
  sub_1ab80(dat_82380,a0);
  *(unsigned long *)(a0 + 0x118) = *(unsigned long *)(a0 + 0x88);
  if (!*(char *)(a0 + 0x150))
    return;
  *(unsigned long *)(a0 + 0x88) = *(unsigned long *)(a0 + 0x148);
  return;
}


// Function: sub_1d980 @ 0x1d980
void sub_1d980(int8 a0) // return-dupe
{
  int8 *v1;
  int8 *v2;
  int8 v3; // stack - 0x38
  int8 v4;
  uint8 v5;
  
  v5 = *(uint8 *)(a0 + 8);
  if (!v5)
    return;
  v4 = *(int8 *)(a0 + 0x10);
  v3 = v4 + 0x200;
  v2 = dat_82378;
  if (dat_82378) {
    do {
      v1 = (int8 *)*v2;
      free((void *)v2[1]);
      free((void *)v2[2]);
      free(v2);
      v2 = v1;
    } while (v1);
    v5 = *(uint8 *)(a0 + 8);
    v4 = *(int8 *)(a0 + 0x10);
  }
  dat_82378 = NULL;
  while (sub_1b390(v5,v4,&v3,sub_1bab0,0x82378)) {
    v5 = *(uint8 *)(a0 + 8);
    v4 = *(int8 *)(a0 + 0x10);
  }
  return;
}


// Function: sub_1da70 @ 0x1da70
void sub_1da70(char *a0,int8 a1,unsigned long a2)
{
  unsigned long *v1;
  char *v2;
  int8 v3; // rax
  obstack *v4; // rax
  
  if (*(int8 *)(a1 + 0x178))
    return;
  v3 = sub_1ab00(a0);
  if ((v3) && (v1 = dat_82390, *(int8 *)(v3 + 8))) {
    for (; v1; v1 = (unsigned long *)*v1) {
      v2 = (char *)v1[1];
      if (!fnmatch(v2,a0,0))
        return;
    }
    v1 = dat_82380;
    if (!*(int8 *)(a1 + 0x168)) {
      v4 = (obstack *)sub_4ecc0(0x58);
      *(obstack **)(a1 + 0x168) = v4;
      _obstack_begin(v4,0,0,sub_4ecc0,dat_7efc8);
      v1 = dat_82380;
    }
    while( true ) {
      if (!v1) {
        (**(code **)(v3 + 8))(a1,a0,a1 + 0x168,a2); // jump-as-call
        return;
      }
      v2 = (char *)v1[1];
      if (!strcmp(v2,a0)) break;
      v1 = (unsigned long *)*v1;
    }
  }
  return;
}


// Function: sub_1db80 @ 0x1db80
void sub_1db80(int8 a0,unsigned long *a1,int8 a2)
{
  unsigned long *v1;
  uint8 v10;
  int8 v11; // r14
  unsigned long *v2;
  int8 v3; // rax
  unsigned long *v4;
  int8 v5;
  uint8 v6; // rcx
  uint4 v7; // ebp
  uint8 v8;
  unsigned long *v9;
  
  v5 = 0;
  if (0 <= a2)
    v5 = a2;
  v11 = 0;
  v10 = v5 + 0x200;
  *(uint8 *)(a0 + 8) = v10;
  v3 = sub_4ecc0(v5 + 0x201);
  *(int8 *)(a0 + 0x10) = v3;
  *(char *)(v3 + 0x200 + v5) = 0;
  do {
    v8 = 0x200;
    if (v10 <= 0x200)
      v8 = v10;
    if (!a1) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      sub_16fd0(); // no-return
    }
    v4 = (unsigned long *)(*(int8 *)(a0 + 0x10) + v11);
    v7 = (uint4)v8;
    if (8 <= v7) { // branch-flip
      *v4 = *a1;
      *(unsigned long *)((int8)v4 + ((v8 & 0xffffffff) - 8)) = *(unsigned long *)((int8)a1 + ((v8 & 0xffffffff) - 8));
      v5 = (int8)v4 - (int8)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      v6 = (uint8)((int4)v5 + v7 >> 3);
      v9 = (unsigned long *)((int8)a1 - v5);
      v4 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      while (v6) {
        v2 = &v4[1];
        v1 = &v9[1];
        *v4 = *v9;
        v6 -= 1;
        v9 = v1;
        v4 = v2;
      }
    }
    else if (v8 & 4) { // branch-flip
      *(unsigned int *)v4 = *(unsigned int *)a1;
      *(unsigned int *)((int8)v4 + ((v8 & 0xffffffff) - 4)) = *(unsigned int *)((int8)a1 + ((v8 & 0xffffffff) - 4));
    }
    else if ((v7) && (*(char *)v4 = *(char *)a1, v8 & 2))
      *(unsigned short *)((int8)v4 + ((v8 & 0xffffffff) - 2)) = *(unsigned short *)((int8)a1 + ((v8 & 0xffffffff) - 2));
    v10 -= v8;
    v11 += v8;
    sub_ccd0(a1);
    a1 = (unsigned long *)sub_d360();
  } while (1 <= (int8)v10);
  return;
}


// Function: sub_1dcd0 @ 0x1dcd0
void sub_1dcd0(struct_5 *a0)
{
  unsigned long *v1;
  int8 v2;
  int8 v3;
  uint8 v4;
  uint8 v5; // rdx
  
  for (v1 = dat_82380; v1; v1 = (unsigned long *)*v1) {
    sub_1bdb0((char *)v1[2],(char *)v1[1],a0);
  }
  v2 = a0->field_0x0;
  v3 = *(int8 *)(v2 + 0x10);
  if (*(int8 *)(v2 + 0x18) == v3)
    *(uint1 *)(v2 + 0x50) = *(uint1 *)(v2 + 0x50) | 2;
  v5 = *(int8 *)(v2 + 0x18) + *(uint8 *)(v2 + 0x30) & ~*(uint8 *)(v2 + 0x30);
  v4 = *(uint8 *)(v2 + 0x20);
  if (v5 - *(int8 *)(v2 + 8) <= *(uint8 *)(v2 + 0x20) - *(int8 *)(v2 + 8))
    v4 = v5;
  *(uint8 *)(v2 + 0x18) = v4;
  *(uint8 *)(v2 + 0x10) = v4;
  a0->field_0x10 = v3;
  return;
}


// Function: sub_1dd60 @ 0x1dd60
void sub_1dd60(struct_5 *a0)
{
  void *v1; // rax
  obstack *v2; // rax
  int8 *v3;
  
  if (dat_82388) {
    v3 = dat_82388;
    if (!a0->field_0x0) {
      v2 = (obstack *)sub_4ecc0(0x58);
      a0->field_0x0 = (int8)v2;
      _obstack_begin(v2,0,0,sub_4ecc0,dat_7efc8);
      v3 = dat_82388;
      if (!dat_82388) goto label_1dd98;
    }
    do {
      sub_1bdb0((char *)v3[2],(char *)v3[1],a0);
      v3 = (int8 *)*v3;
    } while (v3);
  }
label_1dd98:
  if (!a0->field_0x0)
    return;
  sub_1dcd0(a0);
  v1 = (void *)sub_1d220();
  sub_1d2d0(0x67,v1,dat_82a20,a0);
  free(v1); // tail-call
  return;
}


// Function: sub_1de70 @ 0x1de70
void sub_1de70(int8 a0)
{
  *(unsigned long *)(a0 + 0x18) = 0;
  return;
}


// Function: sub_1de80 @ 0x1de80
void sub_1de80(struct_77 *a0,char *a1)
{
  uint8 v1; // rax
  obstack *v2; // rax
  
  if (a0->field_0x10)
    return;
  if (!a0->field_0x0) {
    v2 = (obstack *)sub_4ecc0(0x58);
    a0->field_0x0 = (int8)v2;
    _obstack_begin(v2,0,0,sub_4ecc0,dat_7efc8);
  }
  v1 = strlen(a1);
  a0->field_0x18 = a0->field_0x18 + v1;
  sub_1a9b0(a0,a1,strlen(a1)); // tail-call
  return;
}


// Function: sub_1df00 @ 0x1df00
unsigned long sub_1df00(int8 *a0,char *a1) // return-dupe
{
  uint8 v1; // rax
  char *v10; // rbp
  obstack *v11;
  bool v12; // zf
  char *v2; // rax
  int8 v3;
  char *v4;
  obstack *v5;
  char *v6;
  char *v7;
  char v8 [20];
  char v9 [4];
  
  if (a0[2])
    return 0;
  if (!*a0) {
    v5 = (obstack *)sub_4ecc0(0x58);
    *a0 = (int8)v5;
    _obstack_begin(v5,0,0,sub_4ecc0,dat_7efc8);
  }
  v1 = strlen(a1);
  v3 = a0[3];
  v7 = NULL;
  do {
    v2 = (char *)sub_45370(&v7[v3 + 3 + v1],v8);
    v12 = v7 != &v9[-(int8)v2];
    v7 = &v9[-(int8)v2];
  } while (v12);
  v7 = &v9[strlen(a1) + -(int8)v2 + 2];
  v5 = (obstack *)*a0;
  v6 = *(char **)&v5->field_0x20;
  v3 = *(int8 *)&v5->field_0x18;
  v11 = v5;
  if (&v6[-v3] < v7) {
    _obstack_newchunk(v5,(uint8)v7);
    v11 = (obstack *)*a0;
    v3 = *(int8 *)&v5->field_0x18;
    v6 = *(char **)&v11->field_0x20;
  }
  *(char **)&v5->field_0x18 = &v7[v3];
  v4 = *(char **)&v11->field_0x18;
  a0[1] = (int8)&v7[a0[1]];
  if (v4 == v6) {
    _obstack_newchunk(v11,1);
    v4 = *(char **)&v11->field_0x18;
  }
  *(char **)&v11->field_0x18 = &v4[1];
  *v4 = 10;
  a0[1] = a0[1] + 1;
  v1 = ~a0[3];
  v10 = (char *)((v1 - (int8)v7) + *(int8 *)(*a0 + 0x18));
  memmove((void *)(*(int8 *)(*a0 + 0x18) + v1),v10,a0[3]);
  v2 = stpcpy(v10,v2);
  *v2 = ' ';
  *stpcpy(&v2[1],a1) = '=';
  return 1;
}


// Function: sub_1e0d0 @ 0x1e0d0
uint8 sub_1e0d0(int8 a0,uint8 a1)
{
  return (uint8)(*(int8 *)(a0 + 0x18) + *(int8 *)(a0 + 0x20)) % a1;
}


// Function: sub_1e0f0 @ 0x1e0f0
unsigned long sub_1e0f0(int8 a0,int8 a1)
{
  if (*(int8 *)(a0 + 0x18) != *(int8 *)(a1 + 0x18))
    return 0;
  return CONCAT71((undefined7)((uint8)*(int8 *)(a1 + 0x20) >> 8),*(int8 *)(a0 + 0x20) == *(int8 *)(a1 + 0x20));
}


// Function: sub_1e190 @ 0x1e190
struct_36 * sub_1e190(char *a0,unsigned long a1)
{
  uint8 v1;
  struct_36 *v2; // rax
  void *v3; // rax
  uint8 v4;
  
  v1 = strlen(a0);
  v2 = (struct_36 *)sub_4ecc0(0x60);
  v2->field_0x0 = 0;
  v2->field_0x30 = 0;
  v2->field_0x28 = 0;
  v2->field_0x40 = 0;
  v2->field_0x3c = 0;
  if ((2 <= v1) && (v4 = v1 - 1, a0[v1 - 1] == '/')) {
  }
  else {
    v4 = v1;
    v1 += 1;
  }
  v3 = (void *)sub_4ecc0(v1);
  v2->field_0x58 = v3;
  v3 = memcpy(v3,a0,v4);
  v2->field_0x50 = a1;
  *(char *)((int8)v3 + v4) = 0;
  v2->field_0x48 = 0;
  return v2;
}


// Function: sub_1e240 @ 0x1e240
unsigned long sub_1e240(char *a0)
{
  void *v1; // rax
  unsigned long v2;
  
  v2 = 0;
  if (dat_823b0) {
    v1 = (void *)sub_1e190(a0,sub_26700(dat_82580,a0));
    v2 = sub_43960(dat_823b0,v1);
    free(*(void **)((int8)v1 + 0x50));
    free(*(void **)((int8)v1 + 0x58));
    free(v1);
  }
  return v2;
}


// Function: sub_1e2c0 @ 0x1e2c0
unsigned int sub_1e2c0(int8 a0,int8 a1)
{
  uint4 v1; // eax
  
  v1 = strcmp(*(char **)(a0 + 0x50),*(char **)(a1 + 0x50));
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_1e2e0 @ 0x1e2e0
unsigned long * sub_1e2e0(char *a0,char *a1)
{
  int8 v1;
  int8 v10; // stack - 0x50
  unsigned long *v2; // rax
  unsigned long *v3;
  int8 v4; // rbx
  uint8 v5;
  uint8 v6;
  uint8 v7; // r12
  char v8;
  char *v9;
  
  v8 = *a0;
  if (v8) { // branch-flip
    v10 = 0;
    v7 = 1;
    v4 = 0;
    v9 = a0;
    do {
      v1 = strlen(v9) + 1;
      v7 += v1;
      if (a1) { // branch-flip
        if (strchr(a1,(int4)v8)) goto label_1e345;
      }
      else {
label_1e345:
        v10 += 1;
      }
      v9 = &v9[v1];
      v4 += 1;
      v8 = *v9;
    } while (v8);
    v5 = v7 + 0x20;
    v6 = v10 + 1;
  }
  else {
    v10 = 0;
    v6 = 1;
    v5 = 0x21;
    v4 = 0;
    v7 = 1;
  }
  v2 = (unsigned long *)sub_4ecc0(v5);
  *v2 = &v2[4];
  memcpy(&v2[4],a0,v7);
  v2[1] = v4;
  v2[2] = v10;
  v3 = (unsigned long *)sub_4edb0(v6,8);
  v9 = (char *)*v2;
  v2[3] = v3;
  v8 = *v9;
  if (v8) {
    v4 = 0;
    do {
      if (a1) { // branch-flip
        if (strchr(a1,(int4)v8)) goto label_1e3d2;
      }
      else {
label_1e3d2:
        v3[v4] = &v9[1];
        v4 += 1;
      }
      v9 = &v9[strlen(v9) + 1];
      v8 = *v9;
    } while (v8);
    v3 = &v3[v4];
  }
  *v3 = 0;
  return v2;
}


// Function: sub_1e440 @ 0x1e440
int8 sub_1e440(char *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,bool a5,unsigned int a6,char *a7)
{
  unsigned int v1;
  char *v2;
  int8 v3; // rax
  unsigned long v4;
  int8 v5;
  
  v2 = a7;
  v1 = a6;
  v3 = sub_1e190(a0,sub_26700(dat_82580,a0));
  if (dat_823b8) // branch-flip
    *dat_823b8 = v3;
  else {
    dat_823c0 = v3;
  }
  *(unsigned long *)(v3 + 8) = a1;
  *(unsigned long *)(v3 + 0x10) = a2;
  *(unsigned long *)(v3 + 0x18) = a3;
  *(unsigned long *)(v3 + 0x20) = a4;
  *(unsigned int *)(v3 + 0x38) = 1;
  if (a5)
    *(uint4 *)(v3 + 0x3c) = *(uint4 *)(v3 + 0x3c) | 2;
  if ((char)v1)
    *(uint4 *)(v3 + 0x3c) = *(uint4 *)(v3 + 0x3c) | 4;
  v4 = 0;
  dat_823b8 = (int8 *)v3;
  if (v2)
    v4 = sub_1e2e0(v2,"YND");
  *(unsigned long *)(v3 + 0x28) = v4;
  v5 = dat_823b0;
  if ((((dat_823b0) || (v5 = sub_43bf0(0,0,sub_1e5b0,sub_1e2c0,0), dat_823b0 = v5, v5)) && (sub_44240(v5,v3))) && (((v5 = dat_823a8, dat_823a8 || (v5 = sub_43bf0(0,0,sub_1e0d0,sub_1e0f0,0), dat_823a8 = v5, v5)) && (sub_44240(v5,v3)))))
    return v3;
  sub_17020(); // no-return
}


// Function: sub_1e5b0 @ 0x1e5b0
uint8 sub_1e5b0(int8 a0,uint8 a1)
{
  uint1 *v1;
  
  v1 = *(uint1 **)(a0 + 0x50);
  return sub_43b80(v1,a1); // tail-call
}


// Function: sub_1e5c0 @ 0x1e5c0
void sub_1e5c0(obstack *a0,char *a1,char *a2)
{
  uint8 v1;
  char *v2;
  void *v3;
  
  if (*a1) { // branch-flip
    a1 = (char *)sub_33640(a1,0,(uint1)dat_82b92);
    v2 = *(char **)&a0->field_0x18;
    if (*(char **)&a0->field_0x20 == v2) goto label_1e6aa;
  }
  else {
    v2 = *(char **)&a0->field_0x18;
    if (*(char **)&a0->field_0x20 == v2) {
label_1e6aa:
      _obstack_newchunk(a0,1);
      v2 = *(char **)&a0->field_0x18;
    }
  }
  *(char **)&a0->field_0x18 = &v2[1];
  *v2 = 0x52;
  v1 = strlen(a1);
  v3 = *(void **)&a0->field_0x18;
  v1 += 1;
  if ((uint8)(*(int8 *)&a0->field_0x20 - (int8)v3) < v1) {
    _obstack_newchunk(a0,v1);
    v3 = *(void **)&a0->field_0x18;
  }
  memcpy(v3,a1,v1);
  v2 = (char *)(*(int8 *)&a0->field_0x18 + v1);
  *(char **)&a0->field_0x18 = v2;
  if (*a2) { // branch-flip
    a2 = (char *)sub_33640(a2,0,(uint1)dat_82b92);
    v2 = *(char **)&a0->field_0x18;
    if (*(char **)&a0->field_0x20 != v2) goto label_1e640;
  }
  else if (*(char **)&a0->field_0x20 != v2) goto label_1e640;
  _obstack_newchunk(a0,1);
  v2 = *(char **)&a0->field_0x18;
label_1e640:
  *(char **)&a0->field_0x18 = &v2[1];
  *v2 = 0x54;
  v1 = strlen(a2);
  v3 = *(void **)&a0->field_0x18;
  v1 += 1;
  if ((uint8)(*(int8 *)&a0->field_0x20 - (int8)v3) < v1) {
    _obstack_newchunk(a0,v1);
    v3 = *(void **)&a0->field_0x18;
  }
  memcpy(v3,a2,v1);
  *(uint8 *)&a0->field_0x18 = *(int8 *)&a0->field_0x18 + v1;
  return;
}


// Function: sub_1e730 @ 0x1e730
uint4 sub_1e730(FILE *a0,obstack *a1,int8 *a2)
{
  uint1 *v1;
  char *v2;
  uint4 v3;
  int8 v4;
  
  v1 = *(uint1 **)&a0->field_0x8;
  if (*(uint1 **)&a0->field_0x10 <= v1) // branch-flip
    v3 = __uflow(a0);
  else {
    *(uint1 **)&a0->field_0x8 = &v1[1];
    v3 = (uint4)*v1;
  }
  v4 = 0;
  while (2 <= v3 + 1) {
    v2 = *(char **)&a1->field_0x18;
    if (*(char **)&a1->field_0x20 == v2) {
      _obstack_newchunk(a1,1);
      v2 = *(char **)&a1->field_0x18;
    }
    *(char **)&a1->field_0x18 = &v2[1];
    *v2 = (char)v3;
    v1 = *(uint1 **)&a0->field_0x8;
    if (*(uint1 **)&a0->field_0x10 <= v1) // branch-flip
      v3 = __uflow(a0);
    else {
      *(uint1 **)&a0->field_0x8 = &v1[1];
      v3 = (uint4)*v1;
    }
    v4 += 1;
  }
  v2 = *(char **)&a1->field_0x18;
  if (*(char **)&a1->field_0x20 == v2) {
    _obstack_newchunk(a1,1);
    v2 = *(char **)&a1->field_0x18;
  }
  *(char **)&a1->field_0x18 = &v2[1];
  *v2 = 0;
  *a2 = v4;
  return v3;
}


// Function: sub_1e820 @ 0x1e820
unsigned long sub_1e820(FILE *a0,char *a1,uint8 a2,uint8 a3,unsigned long *a4) // early-return
{
  uint4 v1; // eax
  int8 v10; // rbx
  char v11 [32]; // stack - 0xb8
  char v12 [32];
  char v13 [24];
  char v14 [32];
  bool v15; // zf
  char *v16;
  uint1 *v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char *v6;
  char *v7; // rax
  char v8; // cl
  int4 v9;
  
  v2 = *(uint1 **)&a0->field_0x8;
  if (*(uint1 **)&a0->field_0x10 <= v2) { // branch-flip
    v1 = __uflow(a0);
    v2 = (uint1 *)CONCAT44(dat_4,v1);
  }
  else {
    *(uint1 **)&a0->field_0x8 = &v2[1];
    v1 = (uint4)*v2;
  }
  v15 = v1 == 0x2d;
  v10 = 0;
  do {
    v9 = (int4)v10;
    v8 = (char)v1;
    if (!v9 && v15) {
      v2 = *(uint1 **)&a0->field_0x8;
      v11[0] = v8;
      if (*(uint1 **)&a0->field_0x10 <= v2) goto label_1e8cc;
label_1e8a5:
      *(uint1 **)&a0->field_0x8 = &v2[1];
      v1 = (uint4)*v2;
    }
    else {
      if (10 <= v1 - 0x30) {
        v11[v9] = '\0';
        if ((int4)v1 < 0) {
          if (*(uint1 *)a0 & 0x20)
            sub_33090(dat_82b28); // no-return
          if (!v9)
            return CONCAT71((undefined7)((uint8)v2 >> 8),!v9 && v15);
          if (dat_82d50)
            (*dat_82d50)();
          v6 = dcgettext(NULL,"Unexpected EOF in snapshot file",5);
          error(0,0,"%s: %s",(char *)sub_4ce40(dat_82b28),v6);
          sub_16fd0(); // no-return
        }
        if (!v1) {
          *a4 = sub_255e0(v11,0,a2);
          v9 = *__errno_location();
          if (!v9)
            return 1;
          if (v9 != 0x22) {
            if (dat_82d50)
              (*dat_82d50)();
            v6 = (char *)sub_452d0(ftello(a0),v12);
            v16 = (char *)sub_4ce40(dat_82b28);
            error(0,v9,dcgettext(NULL,"%s: byte %s: %s %s",5),v16,v6,a1,v11);
            sub_16fd0(); // no-return
          }
          if (dat_82d50)
            (*dat_82d50)();
          v6 = (char *)sub_45370(a3,v13);
          v16 = (char *)sub_45230(a2,v14);
          v5 = (char *)sub_452d0(ftello(a0),v12);
          v7 = (char *)sub_4ce40(dat_82b28);
          error(0,0x22,dcgettext(NULL,"%s: byte %s: (valid range %s..%s)\n\t%s %s",5),v7,v5,v16,v6,a1,v11);
          sub_16fd0(); // no-return
        }
        if (dat_82d50)
          (*dat_82d50)();
        v3 = sub_452d0(ftello(a0),v12);
        v4 = sub_4ce40(dat_82b28);
        v5 = dcgettext(NULL,"%s: byte %s: %s %s followed by invalid byte 0x%02x",5);
        v6 = (char *)(uint8)v1;
        v16 = v11;
        error(0,0,v5,v4,v3,a1,v16,v6); // return-dupe
        sub_16fd0(); // no-return
      }
      v11[v10] = v8;
      v6 = v11;
      if (v10 == 0x14) {
        if (dat_82d50)
          (*dat_82d50)();
        v3 = sub_452d0(ftello(a0),v12);
        v4 = sub_4ce40(dat_82b28);
        v5 = dcgettext(NULL,"%s: byte %s: %s %.*s... too long",5);
        v16 = (char *)0x15;
        error(0,0,v5,v4,v3,a1,v16,v6);
        sub_16fd0();
      }
      v2 = *(uint1 **)&a0->field_0x8;
      if (v2 < *(uint1 **)&a0->field_0x10) goto label_1e8a5;
label_1e8cc:
      v1 = __uflow(a0);
      v2 = (uint1 *)CONCAT44(dat_4,v1);
    }
    v10 += 1;
  } while( true );
}


// Function: sub_1ebb0 @ 0x1ebb0
void sub_1ebb0(FILE *a0,struct_40 *a1)
{
  char *v1; // rax
  unsigned long v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  
  if ((sub_1e820(a0,"sec",0x8000000000000000,0x7fffffffffffffff,&v2)) && (sub_1e820(a0,"nsec",0,999999999,&v3))) {
    a1->field_0x0 = v2;
    a1->field_0x8 = v3;
    return;
  }
  if (dat_82d50)
    (*dat_82d50)();
  v1 = dcgettext(NULL,"Unexpected EOF in snapshot file",5);
  error(0,0,"%s: %s",(char *)sub_4ce40(dat_82b28),v1);
  sub_16fd0(); // no-return
}


// Function: sub_1ec90 @ 0x1ec90
char * sub_1ec90(int8 a0,char *a1)
{
  int8 v1;
  char *v2;
  int4 v3; // eax
  uint8 v4; // rbx
  uint8 v5;
  uint8 v6;
  
  if (!a0)
    return NULL;
  v5 = 0;
  v1 = *(int8 *)(a0 + 0x18);
  v6 = *(uint8 *)(a0 + 0x10);
  do {
    if (v6 <= v5)
      return NULL;
    while( true ) {
      v4 = v6 + v5 >> 1;
      v2 = *(char **)(v1 + v4 * 8);
      v3 = strcmp(a1,v2);
      if (v3 < 0) break;
      if (!v3)
        return &v2[-1];
      v5 = v4 + 1;
      if (v6 <= v5)
        return NULL;
    }
    v6 = v4;
  } while( true );
}


// Function: sub_1ed10 @ 0x1ed10
unsigned long sub_1ed10(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  unsigned long v2;
  
  v2 = 0;
  if (dat_823a8) {
    v1 = (void *)sub_1e190("",0);
    *(unsigned long *)((int8)v1 + 0x18) = a0;
    *(unsigned long *)((int8)v1 + 0x20) = a1;
    v2 = sub_43960(dat_823a8,v1);
    free(*(void **)((int8)v1 + 0x50));
    free(*(void **)((int8)v1 + 0x58));
    free(v1);
  }
  return v2;
}


// Function: sub_1ed80 @ 0x1ed80
int8 sub_1ed80(char *a0,int8 a1,uint4 a2,char *a3)
{
  unsigned long *v1;
  char *v10; // rax
  uint8 v11; // rax
  uint8 v12;
  unsigned long v13; // stack - 0x48
  void *v2;
  unsigned long v3;
  bool v4;
  uint4 v5;
  int4 v6;
  int8 v7; // rax
  int8 v8;
  char *v9;
  
  v12 = *(uint8 *)(a1 + 0x58);
  v7 = sub_1e240(a0);
  if (v7) { // branch-flip
    if (*(uint1 *)(v7 + 0x3c) & 1) { // branch-flip
      if (a2 & 0x20) {
        sub_25120((unsigned long *)(v7 + 0x58),a0);
        goto label_1edde;
      }
    }
    else {
label_1edde:
      v9 = *(char **)(v7 + 0x58);
      if (!strcmp(v9,a0)) {
        if (((dat_82b20) && ((!(*(uint1 *)(v7 + 0x3c) & 2) || (0 <= (int8)v12)))) && (v8 = *(int8 *)(a1 + 0x58), *(int8 *)(v7 + 0x18) != v8)) {
label_1ee2f:
          v8 = sub_1ed10(v8,*(int8 *)(a1 + 0x60));
          if (v8) { // branch-flip
            v9 = *(char **)(v8 + 0x58);
            if (strcmp(v9,a0)) {
              if (dat_81b81 & 0x20) {
                if (dat_82d50) {
                  (*dat_82d50)();
                  v9 = *(char **)(v8 + 0x58);
                }
                v9 = (char *)sub_4d2b0(1,v9);
                v10 = (char *)sub_4ce40(a0);
                error(0,0,dcgettext(NULL,"%s: Directory has been renamed from %s",5),v10,v9);
                v9 = *(char **)(v8 + 0x58);
              }
              *(int8 *)(v7 + 0x40) = v8;
              *(uint4 *)(v7 + 0x3c) = *(uint4 *)(v7 + 0x3c) | 0x10;
              *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) & 0xffffffef;
              v11 = strlen(v9);
              strlen(a0);
              for (v1 = dat_823c0; v1; v1 = (unsigned long *)*v1) {
                sub_25500(&v1[0xb],v9,v11,a0);
              }
            }
            *(unsigned int *)(v7 + 0x38) = 1;
            v4 = 0;
          }
          else {
            v3 = *(unsigned long *)(a1 + 0x58);
            *(unsigned int *)(v7 + 0x38) = 2;
            v4 = 1;
            *(unsigned long *)(v7 + 0x18) = v3;
            *(unsigned long *)(v7 + 0x20) = *(unsigned long *)(a1 + 0x60);
          }
          v5 = *(uint4 *)(v7 + 0x3c);
          if ((int8)v12 <= -1)
            v5 = *(uint4 *)(v7 + 0x3c) | 2;
        }
        else {
          if (*(int8 *)(v7 + 0x20) != *(int8 *)(a1 + 0x60)) {
            v8 = *(int8 *)(a1 + 0x58);
            goto label_1ee2f;
          }
          *(unsigned int *)(v7 + 0x38) = 1;
          v5 = *(uint4 *)(v7 + 0x3c);
          v4 = 0;
        }
        *(uint4 *)(v7 + 0x3c) = v5 | 4;
        goto label_1ef85;
      }
    }
    *a3 = 0x4e;
  }
  else {
    v8 = sub_1ed10(*(unsigned long *)(a1 + 0x58),*(unsigned long *)(a1 + 0x60));
    v7 = sub_1e440(a0,*(unsigned long *)(a1 + 0xb0),*(unsigned long *)(a1 + 0xb8),*(unsigned long *)(a1 + 0x58),*(unsigned long *)(a1 + 0x60),v12 >> 0x3f);
    if (v8) { // branch-flip
      v9 = *(char **)(v8 + 0x58);
      if (strcmp(v9,a0)) {
        if (dat_81b81 & 0x20) {
          if (dat_82d50) {
            (*dat_82d50)();
            v9 = *(char **)(v8 + 0x58);
          }
          v9 = (char *)sub_4d2b0(1,v9);
          v10 = (char *)sub_4ce40(a0);
          error(0,0,dcgettext(NULL,"%s: Directory has been renamed from %s",5),v10,v9);
          v9 = *(char **)(v8 + 0x58);
        }
        *(int8 *)(v7 + 0x40) = v8;
        *(uint4 *)(v7 + 0x3c) = *(uint4 *)(v7 + 0x3c) | 0x10;
        *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) & 0xffffffef;
        v12 = strlen(v9);
        strlen(a0);
        for (v1 = dat_823c0; v1; v1 = (unsigned long *)*v1) {
          sub_25500(&v1[0xb],v9,v12,a0);
        }
      }
      *(unsigned int *)(v7 + 0x38) = 1;
      v4 = 0;
    }
    else {
      *(uint4 *)(v7 + 0x3c) = *(uint4 *)(v7 + 0x3c) | 8;
      if (dat_81b81 & 0x10) {
        if (dat_82d50)
          (*dat_82d50)();
        v9 = (char *)sub_4ce40(a0);
        error(0,0,dcgettext(NULL,"%s: Directory is new",5),v9);
      }
      v6 = 2;
      if (((!dat_82b28) && (0 <= (int4)(((uint4)(dat_82b08 < *(int8 *)(a1 + 0xb8)) - (uint4)(*(int8 *)(a1 + 0xb8) < dat_82b08)) + ((uint4)(dat_82b00 < *(int8 *)(a1 + 0xb0)) - (uint4)(*(int8 *)(a1 + 0xb0) < dat_82b00)) * 2))) && (v6 = 1, dat_82b8c))
        v6 = 1 - ((int4)(((uint4)(dat_82b08 < *(int8 *)(a1 + 200)) - (uint4)(*(int8 *)(a1 + 200) < dat_82b08)) + ((uint4)(dat_82b00 < *(int8 *)(a1 + 0xc0)) - (uint4)(*(int8 *)(a1 + 0xc0) < dat_82b00)) * 2) >> 0x1f);
      *(int4 *)(v7 + 0x38) = v6;
      v4 = 0;
    }
label_1ef85:
    if (((dat_82ad1) && (*(int8 *)(a1 + 0x198))) && (*(int8 *)(a1 + 0x58) != *(int8 *)(*(int8 *)(a1 + 0x198) + 0x58))) {
      if (dat_81b82 & 4) {
        if (dat_82d50)
          (*dat_82d50)();
        v9 = (char *)sub_4ce40(*(char **)(v7 + 0x58));
        error(0,0,dcgettext(NULL,"%s: directory is on a different filesystem; not dumped",5),v9);
      }
      v2 = *(void **)(v7 + 0x28);
      *(unsigned int *)(v7 + 0x38) = 0;
      if (!v2) {
        *(uint4 *)(v7 + 0x3c) = *(uint4 *)(v7 + 0x3c) | 1;
        return v7;
      }
      free(*(void **)((int8)v2 + 0x18));
      free(v2);
      *(unsigned long *)(v7 + 0x28) = 0;
      v6 = *(int4 *)(v7 + 0x38);
    }
    else {
      if ((a2 & 0x10) && (*(uint4 *)(v7 + 0x38) = a2 & 3, !(a2 & 3)))
        *a3 = 0x4e;
      if ((v4) && (dat_81b81 & 0x20)) {
        if (dat_82d50)
          (*dat_82d50)();
        v9 = (char *)sub_4ce40(a0);
        error(0,0,dcgettext(NULL,"%s: Directory has been renamed",5),v9);
      }
      v6 = *(int4 *)(v7 + 0x38);
    }
    *(uint4 *)(v7 + 0x3c) = *(uint4 *)(v7 + 0x3c) | 1;
    if (v6) {
      v6 = sub_149e0(a1,&v13);
      if (v6 != 2) { // branch-flip
        if (v6 != 3) { // branch-flip
          if (v6 == 1) {
            sub_12e70(a0,v13,dcgettext(NULL,"contents not dumped",5));
            *(unsigned int *)(v7 + 0x38) = 0;
            *(unsigned long *)(v7 + 0x48) = v13;
          }
        }
        else {
          sub_12e70(a0,v13,dcgettext(NULL,"directory not dumped",5));
          *a3 = 0x4e;
          *(unsigned int *)(v7 + 0x38) = 0;
        }
      }
      else {
        sub_12e70(a0,v13,dcgettext(NULL,"contents not dumped",5));
        *(unsigned long *)(v7 + 0x48) = v13;
      }
    }
  }
  return v7;
}


// Function: sub_1f3f0 @ 0x1f3f0
uint4 sub_1f3f0(int8 a0,FILE *a1)
{
  uint1 *v1;
  uint1 v10;
  uint1 *v11; // r12
  uint1 *v2;
  int8 *v3;
  int8 v4;
  char *v5;
  uint4 v6; // eax
  char *v7;
  struct_35 *v8; // rax
  char v9 [24];
  
  if (*(uint4 *)(a0 + 0x3c) & 4) {
    v6 = *(uint4 *)(a0 + 0x3c) & 2;
    v11 = (uint1 *)0x66c5b;
    v10 = 0x31 - (v6 == 0);
    if (!v6)
      v11 = (uint1 *)0x6af34;
    v1 = &v11[1];
    do {
      v2 = *(uint1 **)&a1->field_0x28;
      if (*(uint1 **)&a1->field_0x30 <= v2) { // branch-flip
        if (__overflow(a1,(uint4)v10) == -1) goto label_1f463;
      }
      else {
        *(uint1 **)&a1->field_0x28 = &v2[1];
        *v2 = v10;
      }
      if (v1 == v11) goto label_1f463;
      v11 = &v11[1];
      v10 = *v11;
    } while( true );
  }
  return (*(uint4 *)a1 >> 5 ^ 1) & 1; // return-dupe
label_1f463:
  v7 = (char *)sub_255c0(*(uint8 *)(a0 + 8),0x8000000000000000,0x7fffffffffffffff,v9);
  fwrite_unlocked(v7,strlen(v7) + 1,1,a1);
  v7 = (char *)sub_45230(*(uint8 *)(a0 + 0x10),v9);
  fwrite_unlocked(v7,strlen(v7) + 1,1,a1);
  v7 = (char *)sub_255c0(*(uint8 *)(a0 + 0x18),0,0xffffffffffffffff,v9);
  fwrite_unlocked(v7,strlen(v7) + 1,1,a1);
  v7 = (char *)sub_255c0(*(uint8 *)(a0 + 0x20),0,0xffffffffffffffff,v9);
  fwrite_unlocked(v7,strlen(v7) + 1,1,a1);
  v7 = *(char **)(a0 + 0x58);
  fwrite_unlocked(v7,strlen(v7) + 1,1,a1);
  v3 = *(int8 **)(a0 + 0x28);
  if (v3) {
    v8 = (struct_35 *)sub_4ecc0(0x18);
    v4 = v3[2];
    v8->field_0x0 = v3;
    v8->field_0x8 = 0;
    v8->field_0x10 = 0;
    if (v4) {
      v4 = *(int8 *)v3[3];
      v8->field_0x10 = 1;
      v7 = (char *)(v4 + -1);
      do {
        fwrite_unlocked(v7,strlen(v7) + 1,1,a1);
        v7 = (char *)sub_1e120(v8);
      } while (v7);
    }
    free(v8);
  }
  v5 = *(char **)&a1->field_0x28;
  if (*(char **)&a1->field_0x30 <= v5) { // branch-flip
    if (__overflow(a1,0) == -1) {
      return (*(uint4 *)a1 >> 5 ^ 1) & 1;
    }
  }
  else {
    *(char **)&a1->field_0x28 = &v5[1];
    *v5 = 0;
  }
  v5 = *(char **)&a1->field_0x28;
  if (*(char **)&a1->field_0x30 <= v5) // branch-flip
    __overflow(a1,0);
  else {
    *(char **)&a1->field_0x28 = &v5[1];
    *v5 = 0;
  }
  return (*(uint4 *)a1 >> 5 ^ 1) & 1;
}


// Function: sub_1f6e0 @ 0x1f6e0
void sub_1f6e0(void) // return-dupe
{
  int8 *v1;
  int8 *v2;
  
  if (dat_823b0)
    sub_43cf0(dat_823b0);
  if (dat_823a8)
    sub_43cf0(dat_823a8);
  v2 = dat_823c0;
  if (!dat_823c0) {
    dat_823b8 = 0;
    dat_823c0 = NULL;
    return;
  }
  do {
    v1 = (int8 *)*v2;
    free((void *)v2[10]);
    free((void *)v2[0xb]);
    free(v2);
    v2 = v1;
  } while (v1);
  dat_823b8 = 0;
  dat_823c0 = NULL;
  return;
}


// Function: sub_1f770 @ 0x1f770
void sub_1f770(int8 a0,void *a1,uint8 a2,char *a3,uint8 a4)
{
  sub_25500((unsigned long *)(a0 + 0x58),a1,a2,a3,a4); // tail-call
  return;
}


// Function: sub_1f780 @ 0x1f780
void sub_1f780(struct_49 *a0) // return-dupe x2
{
  int4 v1;
  int8 v2; // rax
  stat v3; // stack - 0xb8
  
  v2 = sub_1e240(a0->field_0x0);
  if (!v2)
    return;
  v1 = a0->field_0x1a8;
  if (!fstat(v1,&v3)) {
    *(unsigned long *)(v2 + 0x10) = v3._96_8_;
    *(unsigned long *)(v2 + 8) = v3._88_8_;
    return;
  }
  sub_26460(*(char **)(v2 + 0x58));
  return;
}


// Function: sub_1f810 @ 0x1f810
int8 sub_1f810(unsigned long *a0)
{
  char v1;
  char *v10;
  uint8 v11; // rax
  struct_35 *v12; // rax
  char *v13;
  unsigned int v14;
  code *v15; // rdx
  unsigned long v16; // rdx
  int8 v17;
  char v18 [88];
  stat v19 [2]; // stack - 0x1a0
  int8 v2;
  int8 v20;
  unsigned long *v21;
  uint8 v22; // stack - 0x238
  unsigned long *v23; // stack - 0x230
  char *v24; // stack - 0x228
  int8 v25; // stack - 0x220
  char *v26; // stack - 0x210
  unsigned long *v27; // stack - 0x60
  int4 v28; // stack - 0x50
  int8 *v3;
  uint1 v4;
  int4 v5;
  char *v6;
  int8 v7;
  unsigned long *v8; // rax
  uint8 v9; // rax
  
  v10 = (char *)*a0;
  v24 = (char *)sub_14860(a0);
  v2 = a0[0xb];
  v7 = a0[0x33];
  if (!v24) {
    sub_331a0(v10);
    sub_17440(a0);
    v6 = (char *)sub_4ee30(v10);
    sub_253a0(v6);
    v16 = 0;
    if (!v7)
      v16 = 0x20;
    v7 = sub_1ed80(v6,a0,v16,&v17);
    free(v6);
    v8 = (unsigned long *)sub_265f0(v10);
    sub_26660(v8); // return-dupe
    free(v24);
    return v7;
  }
  sub_17440(a0);
  v6 = (char *)sub_4ee30(v10);
  sub_253a0(v6);
  v16 = 0;
  if (!v7)
    v16 = 0x20;
  v7 = sub_1ed80(v6,a0,v16,&v17);
  free(v6);
  v8 = (unsigned long *)sub_265f0(v10);
  if (!*(int4 *)(v7 + 0x38)) {
    if (*(char **)(v7 + 0x48)) {
      v9 = strlen(*(char **)(v7 + 0x48));
      v10 = (char *)sub_4ecc0(v9 + 3);
      *v10 = 'Y';
      memcpy(&v10[1],*(void **)(v7 + 0x48),v9 + 1);
      v10[v9 + 2] = '\0';
      *(unsigned long *)(v7 + 0x30) = *(unsigned long *)(v7 + 0x28);
      *(unsigned long *)(v7 + 0x28) = sub_1e2e0(v10,NULL);
      free(v10);
    }
    sub_26660(v8);
    free(v24);
    return v7;
  }
  v25 = 0;
  if (*(int4 *)(v7 + 0x38) != 2) {
    v20 = *(int8 *)(v7 + 0x40);
    if (v20) { // branch-flip
      v25 = *(int8 *)(v20 + 0x30);
      if (!v25)
        v25 = *(int8 *)(v20 + 0x28);
    }
    else {
      v25 = *(int8 *)(v7 + 0x28);
    }
  }
  if (*v24) { // branch-flip
    v9 = 0;
    v20 = 0;
    v10 = v24;
    do {
      v9 += 1;
      v11 = strlen(v10);
      v10 = &v10[v11 + 1];
      v20 = v11 + 2 + v20;
    } while (*v10);
    v22 = v20 + 1;
    v23 = (unsigned long *)sub_4edb0(v9,8);
    if (*v24) {
label_1f9e0:
      v10 = v24;
      v21 = v23;
      do {
        *v21 = v10;
        v21 = &v21[1];
        v10 = &v10[strlen(v10) + 1];
      } while (*v10);
    }
    v11 = 0;
    qsort(v23,v9,8,sub_1e2b0);
    v26 = (char *)sub_4ecc0(v22);
    v10 = v26;
    if (v9) {
      do {
        v6 = (char *)v23[v11];
        if (sub_1ec90(v25,v6)) // branch-flip
          v4 = -(*(int8 *)(v7 + 0x48) == 0) & 0xd7;
        else {
          v4 = -(*(int8 *)(v7 + 0x48) == 0) & 0x10;
        }
        *v10 = v4 + 0x49;
        v13 = &v10[1];
        do {
          v1 = *v6;
          v6 = &v6[1];
          v10 = &v13[1];
          *v13 = v1;
          v13 = v10;
        } while (v1);
        v11 += 1;
      } while (v9 != v11);
    }
  }
  else {
    v9 = 0;
    v23 = (unsigned long *)sub_4edb0(0,8);
    v22 = 1;
    if (*v24) goto label_1f9e0;
    qsort(v23,0,8,sub_1e2b0);
    v10 = (char *)sub_4ecc0(1);
    v26 = v10;
  }
  *v10 = '\0';
  *(unsigned long *)(v7 + 0x30) = *(unsigned long *)(v7 + 0x28);
  *(unsigned long *)(v7 + 0x28) = sub_1e2e0(v26,NULL);
  free(v26);
  free(v23);
  v3 = *(int8 **)(v7 + 0x28);
  v12 = (struct_35 *)sub_4ecc0(0x18);
  v10 = (char *)*v3;
  v12->field_0x0 = v3;
  v12->field_0x8 = 1;
  v12->field_0x10 = 0;
  if (*v10) {
    v12->field_0x10 = strlen(v10) + 1;
    do {
      while( true ) {
        v6 = &v10[1];
        v13 = (char *)sub_26680(v8,v6);
        if ((*v10 != 'I') && (!sub_176d0(v13,a0))) break;
        *v10 = 'N';
        v10 = (char *)sub_1e120(v12);
        if (!v10) goto label_1fbe8;
      }
      v5 = *(int4 *)&a0[0x35];
      sub_2fb40(v18);
      if (0 <= v5) { // branch-flip
        if (fstatat(v5,v6,v19,dat_82800)) {
label_1fcab:
          v15 = sub_26460;
          goto label_1fbb6;
        }
        if ((v19[0]._24_4_ & 0xf000) == 0x4000) {
          v5 = sub_14950(a0,v6,dat_82808);
          if (v5 <= -1) {
            v15 = sub_26370;
            goto label_1fbb6;
          }
          v28 = v5;
          if (!fstat(v5,v19)) {
            if ((v19[0]._24_4_ & 0xf000) != 0x4000) goto label_1fcca;
            if (dat_82ad4) { // branch-flip
              v14 = 0;
              if (*(int4 *)(v7 + 0x38) == 2)
                v14 = 0x12;
            }
            else {
              v14 = 0x10;
            }
            *v10 = 'D';
            v27 = a0;
            sub_1ed80(v13,v18,v14,v10);
            sub_14a80(v18);
            goto label_1fbc5;
          }
          goto label_1fcab;
        }
label_1fcca:
        if ((dat_82ad1) && (v19[0]._0_8_ != v2)) {
label_1fcea:
          *v10 = 'N';
        }
        else if (*v10 != 'Y') {
          if (((int4)(((uint4)(dat_82b08 < (int8)v19[0]._96_8_) - (uint4)((int8)v19[0]._96_8_ < dat_82b08)) + ((uint4)(dat_82b00 < (int8)v19[0]._88_8_) - (uint4)((int8)v19[0]._88_8_ < dat_82b00)) * 2) < 0) && ((!dat_82b8c || ((int4)(((uint4)(dat_82b08 < (int8)v19[0]._112_8_) - (uint4)((int8)v19[0]._112_8_ < dat_82b08)) + ((uint4)(dat_82b00 < (int8)v19[0]._104_8_) - (uint4)((int8)v19[0]._104_8_ < dat_82b00)) * 2) <= -1)))) goto label_1fcea;
          *v10 = 'Y';
        }
      }
      else {
        v15 = sub_26370;
        *__errno_location() = -v5;
label_1fbb6:
        sub_26490(v13,0,v15);
        *v10 = 'N';
      }
label_1fbc5:
      sub_2fc00(v18);
      v10 = (char *)sub_1e120(v12);
    } while (v10);
  }
label_1fbe8:
  free(v12);
  sub_26660(v8);
  free(v24);
  return v7;
}


// Function: sub_1ff00 @ 0x1ff00
int8 sub_1ff00(int8 a0) // early-return x3
{
  int8 v1;
  
  if (!a0)
    return 0x64eec;
  if (!*(int8 **)(a0 + 0x28))
    return 0x64eec;
  v1 = **(int8 **)(a0 + 0x28);
  if (v1)
    return v1;
  return 0x64eec;
}


// Function: sub_1ff30 @ 0x1ff30
void sub_1ff30(int8 a0)
{
  uint8 v1;
  char v10 [24]; // stack - 0x98
  uint8 v11; // r12
  char *v12;
  char *v13; // stack - 0x80
  char *v14; // stack - 0x78
  uint8 v15; // stack - 0x68
  uint1 v16; // stack - 0x48
  unsigned long *v2;
  void *v3;
  undefined8 v4;
  int8 v5; // rax
  unsigned long *v6;
  char *v7;
  char *v8;
  unsigned long *v9;
  
  if (!dat_823c0)
    return;
  _obstack_begin((obstack *)v10,0,0,sub_4ecc0,dat_7efc8);
  v12 = v13;
  if (!a0) {
    v11 = 0;
    v9 = dat_823c0;
    if (!dat_823c0) {
      _obstack_free((obstack *)v10,NULL); // return-dupe
      return;
    }
label_20018:
    if (*(uint1 *)((int8)v9 + 0x3c) & 0x10) {
      v6 = v9;
label_20028:
      v2 = (unsigned long *)v6[8];
      if (v2 != v9) goto label_20038;
      v7 = (char *)sub_40230((char *)v9[0xb]);
      if (v14 == v13)
        _obstack_newchunk((obstack *)v10,1);
      v12 = &v13[1];
      *v13 = 0x58;
      v13 = v12;
      v1 = strlen(v7) + 1;
      if ((uint8)((int8)v14 - (int8)v13) < v1)
        _obstack_newchunk((obstack *)v10,v1);
      memcpy(v13,v7,v1);
      v13 = &v13[v1];
      sub_1e5c0((obstack *)v10,(char *)v9[0xb],"");
      for (v2 = v9; v2 != v6; v2 = (unsigned long *)v2[8]) {
        sub_1e5c0((obstack *)v10,*(char **)(v2[8] + 0x58),(char *)v2[0xb]);
      }
      sub_1e5c0((obstack *)v10,"",(char *)v6[0xb]);
      free(v7);
      goto label_20010;
    }
    goto label_20010;
  }
  if (*(unsigned long **)(a0 + 0x28)) { // branch-flip
    v7 = (char *)**(unsigned long **)(a0 + 0x28);
    v11 = 0;
    if (v7) {
      if (*v7) {
        v8 = v7;
        do {
          v5 = strlen(v8) + 1;
          v8 = &v8[v5];
          v11 += v5;
        } while (*v8);
        if ((uint8)((int8)v14 - (int8)v12) < v11) {
          _obstack_newchunk((obstack *)v10,v11);
          v12 = v13;
        }
      }
      memcpy(v12,v7,v11);
      v13 = &v13[v11];
    }
  }
  else {
    v11 = 0;
  }
  v9 = dat_823c0;
  if (dat_823c0) goto label_20018;
label_20084:
  if ((int8)v13 - (int8)v10._16_8_ != v11) {
    if (v13 == v14)
      _obstack_newchunk((obstack *)v10,1);
    v12 = &v13[1];
    *v13 = 0;
    v3 = *(void **)(a0 + 0x28);
    v13 = v12;
    free(*(void **)((int8)v3 + 0x18));
    free(v3);
    v4 = v10._16_8_;
    if (v13 == v10._16_8_)
      v16 |= 2;
    v10._16_8_ = v14;
    if ((uint8)((int8)(char *)((uint8)&v13[v15] & ~v15) - v10._8_8_) <= (uint8)((int8)v14 - v10._8_8_))
      v10._16_8_ = (char *)((uint8)&v13[v15] & ~v15);
    v13 = v10._16_8_;
    *(unsigned long *)(a0 + 0x28) = sub_1e2e0(v4,"YND");
  }
  _obstack_free((obstack *)v10,NULL);
  return;
label_20038:
  v6 = v2;
  if (!v2) goto label_2003d;
  goto label_20028;
label_2003d:
  v6 = v9;
  do {
    if (!v6[8]) {
      v9 = (unsigned long *)*v9;
      goto label_20070;
    }
    sub_1e5c0((obstack *)v10,*(char **)(v6[8] + 0x58),(char *)v6[0xb]);
    v6 = (unsigned long *)v6[8];
  } while (v6);
label_20010:
  v9 = (unsigned long *)*v9;
label_20070:
  if (!v9) goto label_20078;
  goto label_20018;
label_20078:
  if (!a0) {
    _obstack_free((obstack *)v10,NULL);
    return;
  }
  goto label_20084;
}


// Function: sub_202a0 @ 0x202a0
void sub_202a0(void)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  int8 *v3;
  char v4 [32];
  char v5 [24];
  int8 v6;
  uint8 v7;
  
  __printf_chk(1,"This tar\'s snapshot file field ranges are\n");
  __printf_chk(1,"   (%-15s => [ %s, %s ]):\n\n","field name","min","max");
  v6 = 0;
  v7 = 1;
  v3 = (int8 *)0x7d660;
  while( true ) {
    v1 = sub_255c0(v7,v6,v7,v5);
    v2 = sub_255c0(v3[1],v3[1],v3[2],v4);
    __printf_chk(1,"    %-15s => [ %s, %s ],\n",*v3,v2,v1);
    if (!v3[3]) break;
    v7 = v3[5];
    v6 = v3[4];
    v3 = &v3[3];
  }
  __printf_chk(1,"\n");
  return;
}


// Function: sub_203b0 @ 0x203b0
void sub_203b0(void)
{
  uint1 *v1;
  undefined8 v10;
  unsigned long v11; // rax
  int4 *v12; // rax
  uint8 v13; // rax
  char *v14;
  char *v15; // rax
  char *v16;
  unsigned long v17; // stack - 0xe0
  char *v18; // stack - 0xe8
  char v19 [16]; // stack - 0xb8
  int8 v2;
  int8 v20; // stack - 0xd8
  char *v21; // stack - 0xd0
  int8 v22; // stack - 0xc8
  char v23 [24];
  undefined8 v24;
  bool v25; // zf
  char v26 [16];
  uint8 v27; // stack - 0x100
  int8 v28; // stack - 0xf8
  char **v29; // stack - 0xf0
  void *v3;
  unsigned long v30; // stack - 0xc0
  void *v31; // stack - 0xa8
  void *v32; // stack - 0xa0
  void *v33; // stack - 0x98
  uint8 v34; // stack - 0x88
  uint1 v35; // stack - 0x68
  char v4;
  int4 v5; // eax
  uint4 v6; // eax
  int8 v7;
  void *v8;
  void *v9; // rax
  
  v18 = NULL;
  v17 = 0;
  v5 = open(dat_82b28,(-(uint4)(dat_82b24 == 0) & 0x200) + 0x42,0x1b6);
  if (v5 < 0) {
    sub_32ea0(dat_82b28);
    return;
  }
  dat_823a0 = fdopen(v5,"r+");
  if (!dat_823a0) {
    sub_32ea0(dat_82b28);
    close(v5);
    return;
  }
  sub_29260();
  sub_292e0();
  v7 = __getdelim(&v18,&v17,10,dat_823a0);
  v14 = v18;
  if (v7 <= 0) goto label_2079e;
  if (strncmp(v18,"GNU tar",7)) { // branch-flip
label_20b1c:
    v22 = 0;
    v21 = NULL;
    v14 = strdup(v14);
    v21 = v14;
    v22 = strlen(v14) + 1;
    v7 = 1;
    v26 = sub_25930(v14,v19,0);
    dat_82b08 = SUB168(v26,8);
    dat_82b00 = SUB168(v26,0);
    if ((int8)dat_82b08 < 0) {
label_20dbf:
      if (dat_82d50)
        (*dat_82d50)();
      v14 = dcgettext(NULL,"Invalid time stamp",5);
      v16 = (char *)sub_4ce40(dat_82b28);
      error(0,*__errno_location(),"%s:%ld: %s",v16,v7,v14);
      sub_16fd0(); // no-return
    }
    v27 = 0;
    v28 = 1;
  }
  else {
    v16 = &v14[8];
    if (v14[7] != '-') {
      if (dat_82d50)
        (*dat_82d50)();
      error(1,0,dcgettext(NULL,"Bad incremental file format",5));
      return;
    }
    v4 = v14[8];
    while (v4 != '-') {
      if (!v4) {
        if (dat_82d50)
          (*dat_82d50)();
        error(1,0,dcgettext(NULL,"Bad incremental file format",5));
        return;
      }
      v14 = &v16[1];
      v16 = &v16[1];
      v4 = *v14;
    }
    v27 = strtoumax(&v16[1],0,10);
    if (2 <= v27) {
      if (v27 != 2) {
        if (dat_82d50)
          (*dat_82d50)();
        error(1,0,dcgettext(NULL,"Unsupported incremental format version: %lu",5),v27);
        return;
      }
      _obstack_begin((obstack *)v19,0,0,sub_4ecc0,dat_7efc8);
      sub_1ebb0(dat_823a0,(struct_40 *)0x82b00);
      while (v4 = sub_1e820(dat_823a0,"nfs",0,1,&v20), v4) {
        v25 = v20 != 0;
        sub_1ebb0(dat_823a0,&v22);
        v4 = sub_1e820(dat_823a0,"dev",0,0xffffffffffffffff,&v20);
        v7 = v20;
        if (((!v4) || (v4 = sub_1e820(dat_823a0,"ino",0,0xffffffffffffffff,&v20), v2 = v20, !v4)) || (v5 = sub_1e730(dat_823a0,(obstack *)v19,&v21), v3 = v31, v5)) {
          if (dat_82d50)
            (*dat_82d50)();
          v14 = dcgettext(NULL,"Unexpected EOF in snapshot file",5);
          error(0,0,"%s: %s",(char *)sub_4ce40(dat_82b28),v14);
          sub_16fd0(); // no-return
        }
        if (v31 == v32)
          v35 |= 2;
        v8 = (void *)((int8)v32 + v34 & ~v34);
        v31 = v33;
        if ((uint8)((int8)v8 - v19._8_8_) <= (uint8)((int8)v33 - v19._8_8_))
          v31 = v8;
        v32 = v31;
        do {
          v5 = sub_1e730(dat_823a0,(obstack *)v19,&v21);
          if (v5) break;
        } while ((char *)0x2 <= v21);
        v1 = *(uint1 **)&dat_823a0->field_0x8;
        if (*(uint1 **)&dat_823a0->field_0x10 <= v1) // branch-flip
          v6 = __uflow(dat_823a0);
        else {
          *(uint1 **)&dat_823a0->field_0x8 = &v1[1];
          v6 = (uint4)*v1;
        }
        v8 = v31;
        if (v6) {
          if (dat_82d50)
            (*dat_82d50)();
          v14 = dcgettext(NULL,"Missing record terminator",5);
          v16 = (char *)sub_452d0(ftello(dat_823a0),v23);
          v15 = (char *)sub_4ce40(dat_82b28);
          error(0,0,dcgettext(NULL,"%s: byte %s: %s",5),v15,v16,v14);
          sub_16fd0(); // no-return
        }
        if (v31 == v32)
          v35 |= 2;
        v9 = (void *)((int8)v32 + v34 & ~v34);
        v31 = v33;
        if ((uint8)((int8)v9 - v19._8_8_) <= (uint8)((int8)v33 - v19._8_8_))
          v31 = v9;
        v32 = v31;
        sub_1e440(v3,v22,v30,v7,v2,v25,0,v8);
        if ((v8 <= (uint8)v19._8_8_) || (v33 <= v8))
          _obstack_free((obstack *)v19,v8);
        else {
          v31 = v8;
          v32 = v8;
        }
      }
      goto label_2079e;
    }
    v21 = NULL;
    v22 = 0;
    v14 = v18;
    if (v27 != 1) goto label_20b1c;
    if (__getdelim(&v21,&v22,10,dat_823a0) <= 0) {
      sub_32f00(dat_82b28);
      free(v21);
      goto label_2079e;
    }
    v26 = sub_25930(v21,v19,0);
    v24 = v19._0_8_;
    dat_82b08 = SUB168(v26,8);
    dat_82b00 = SUB168(v26,0);
    if ((int8)dat_82b08 < 0) {
      v7 = 2;
      goto label_20dbf;
    }
    v28 = 2;
    if (*(char *)v19._0_8_) {
      v12 = __errno_location();
      *v12 = 0;
      v13 = strtoumax((char *)(v24 + 1),v19,10);
      if (!*v12) {
        if (1000000000 <= v13) // branch-flip
          *v12 = 0x22;
        else if ((char *)(v24 + 1) != (char *)v19._0_8_) {
          v28 = 2;
          dat_82b08 = v13;
          goto label_2092c;
        }
      }
      if (dat_82d50)
        (*dat_82d50)();
      v14 = dcgettext(NULL,"Invalid time stamp",5);
      v16 = (char *)sub_4ce40(dat_82b28);
      error(0,*v12,"%s:%ld: %s",v16,2,v14);
      dat_82d58 = 2;
      dat_82b00 = 0x8000000000000000;
      dat_82b08 = 0xffffffffffffffff;
      v28 = 2;
    }
  }
label_2092c:
  while( true ) {
    v29 = &v21;
    v5 = (int4)__getdelim(v29,&v22,10,dat_823a0);
    if (v5 <= 0) break;
    v4 = *v21;
    v28 += 1;
    v14 = &v21[v4 == '+'];
    if (v21[(int8)v5 + -1] == '\n')
      v21[(int8)v5 + -1] = '\0';
    v12 = __errno_location();
    v13 = 0;
    v24 = 0;
    if (v27 == 1) {
      v26 = sub_25930(v14,v19,0);
      v10 = v19._0_8_;
      v24 = SUB168(v26,0);
      if ((SUB168(v26,8) < 0) || (*(char *)v19._0_8_ != ' ')) {
        if (dat_82d50)
          (*dat_82d50)();
        v14 = "Invalid modification time";
        goto label_20a14;
      }
      *v12 = 0;
      v13 = strtoumax(v19._0_8_,v19,10);
      if (1000000000 <= v13) { // branch-flip
        if (!*v12)
          *v12 = 0x22;
      }
      else if (((!*v12) && (v10 != v19._0_8_)) && (v14 = (char *)v19._0_8_, *(char *)v19._0_8_ == ' ')) goto label_20898;
      if (dat_82d50)
        (*dat_82d50)();
      v14 = "Invalid modification time (nanoseconds)";
label_20a14:
      v14 = dcgettext(NULL,v14,5);
      v16 = (char *)sub_4ce40(dat_82b28);
      error(0,*v12,"%s:%ld: %s",v16,v28,v14);
      sub_16fd0(); // no-return
    }
label_20898:
    v10 = sub_255e0(v14,v19,0,0xffffffffffffffff);
    if ((*v12) || (*(char *)v19._0_8_ != ' ')) {
      if (dat_82d50)
        (*dat_82d50)();
      v14 = "Invalid device number";
      goto label_20a14;
    }
    v11 = sub_255e0(v19._0_8_,v19,0,0xffffffffffffffff);
    if ((*v12) || (*(char *)v19._0_8_ != ' ')) {
      if (dat_82d50)
        (*dat_82d50)();
      v14 = "Invalid inode number";
      goto label_20a14;
    }
    v14 = (char *)(v19._0_8_ + 1);
    sub_251b0(v14);
    sub_1e440(v14,v24,v13,v10,v11,v4 == '+',0,NULL);
  }
  free(v21);
label_2079e:
  if (*(uint1 *)dat_823a0 & 0x20)
    sub_32f00(dat_82b28);
  free(v18);
  return;
}


// Function: sub_20e40 @ 0x20e40
void sub_20e40(void) // return-dupe x2
{
  FILE *v1;
  int4 v2; // eax
  char *v3; // rax
  char v4 [24];
  
  v1 = dat_823a0;
  if (!dat_823a0)
    return;
  if (fseeko(dat_823a0,0,0))
    sub_331e0(dat_82b28);
  v2 = fileno(v1);
  if (sub_2c110(v2))
    sub_33450(dat_82b28);
  __fprintf_chk(v1,1,"%s-%s-%d\n","GNU tar","1.34",2);
  v3 = (char *)sub_45230(dat_82a20,v4);
  fwrite_unlocked(v3,strlen(v3) + 1,1,v1);
  v3 = (char *)sub_45370(dat_82a28,v4);
  fwrite_unlocked(v3,strlen(v3) + 1,1,v1);
  if ((*(uint1 *)v1 & 0x20) || ((dat_823b0 && (sub_43b00(dat_823b0,sub_1f3f0,v1), *(uint1 *)v1 & 0x20))))
    sub_334f0(dat_82b28);
  if (!fclose(v1))
    return;
  sub_32d60(dat_82b28);
  return;
}


// Function: sub_20fc0 @ 0x20fc0
uint8 sub_20fc0(int8 a0)
{
  uint1 v1;
  uint8 v2;
  void *v3;
  uint8 v4; // rax
  uint8 v5;
  void *v6;
  void *v7; // stack - 0x40
  
  v1 = *(uint1 *)(a0 + 0x188);
  if (!v1)
    return 0;
  if (!*(int8 *)(a0 + 400)) {
    v5 = *(uint8 *)(a0 + 0x88);
    v7 = (void *)sub_4ecc0(v5);
    sub_ccd0(dat_82408);
    sub_c670(a0);
    v6 = v7;
    v3 = v7;
    while (v5) {
      sub_c710(v5);
      v3 = (void *)sub_d360();
      if (!v3) {
        if (dat_82d50)
          (*dat_82d50)();
        error(1,0,dcgettext(NULL,"Unexpected EOF in archive",5));
        return v4;
      }
      v2 = sub_cd10(v3);
      if (v5 < v2)
        v2 = v5;
      memcpy(v6,v3,v2);
      v6 = (void *)((int8)v6 + v2);
      sub_ccd0((int8)v3 + (v2 - 1));
      v5 -= v2;
      v3 = v7;
    }
    sub_c6a0();
    *(char *)(a0 + 0x189) = 1;
    *(void **)(a0 + 400) = v3;
    v1 = *(uint1 *)(a0 + 0x188);
  }
  return (uint8)v1;
}


// Function: sub_21100 @ 0x21100
void sub_21100(char *a0)
{
  uint1 *v1;
  uint1 *v10;
  uint1 *v11;
  uint1 *v12;
  char *v13; // rax
  uint1 v14;
  char *v15;
  int8 v16 [3]; // stack - 0xd8
  uint1 *v17;
  uint1 v18; // r13b
  uint1 v19;
  int4 v2;
  uint4 v20; // stack - 0xc0
  bool v3;
  char *v4; // rax
  struct_63 *v5; // rax
  char *v6; // rax
  char *v7;
  unsigned long v8; // rax
  uint8 v9; // rax
  
  if ((!sub_20fc0(0x82840)) || (v4 = (char *)sub_26970(a0,0), v17 = dat_829d0, !v4)) {
    sub_24660(); // return-dupe, tail-call
    return;
  }
  v19 = *dat_829d0;
  if (v19) { // branch-flip
    v3 = 0;
    v18 = 0;
    v10 = dat_829d0;
    v14 = v19;
    do {
      if (v18) { // branch-flip
        if (v14 != v18) {
          if (dat_82d50)
            (*dat_82d50)();
          error(0,0,dcgettext(NULL,"Malformed dumpdir: expected \'%c\' but found %#3o",5),v18,(uint4)v14);
          dat_82d58 = 2;
          sub_24660();
          return;
        }
        if (v14 != 0x54) goto label_21204;
        if (v18 != 0x54) goto label_217f2;
        if ((!v10[1]) && (!v3)) {
          if (dat_82d50)
            (*dat_82d50)();
          v4 = "Malformed dumpdir: empty name in \'T\'";
          goto label_217ce;
        }
        v18 = 0;
      }
      else {
        if (v14 == 0x54) {
label_217f2:
          if (dat_82d50)
            (*dat_82d50)();
          v4 = "Malformed dumpdir: \'T\' not preceded by \'R\'";
label_217ce:
          error(0,0,dcgettext(NULL,v4,5));
          dat_82d58 = 2;
          sub_24660();
          return;
        }
label_21204:
        if (v14 != 0x58) { // branch-flip
          if (v14 == 0x52) {
            if (v10[1]) // branch-flip
              v18 = 0x54;
            else {
              if (!v3) {
                if (dat_82d50)
                  (*dat_82d50)();
                v4 = "Malformed dumpdir: empty name in \'R\'";
                goto label_217ce;
              }
              v18 = 0x54;
              v3 = 0;
            }
          }
        }
        else {
          if (v3) {
            if (dat_82d50)
              (*dat_82d50)();
            v4 = "Malformed dumpdir: \'X\' duplicated";
            goto label_217ce;
          }
          v3 = 1;
        }
      }
      v10 = &v10[strlen((char *)v10) + 1];
      v14 = *v10;
    } while (v14);
    if (v18) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Malformed dumpdir: expected \'%c\' but found end of data",5),v18);
      dat_82d58 = 2;
      sub_24660();
      return;
    }
    if ((v3) && (dat_81b80 & 2)) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Malformed dumpdir: \'X\' never used",5));
      v19 = *dat_829d0;
      v17 = dat_829d0;
      if (!v19) goto label_212b3;
    }
    v10 = NULL;
    do {
      if (v19 != 0x58) { // branch-flip
        v9 = strlen((char *)v17);
        if (v19 == 0x52) {
          v1 = &v17[v9 + 1];
          v12 = &v17[1];
          v11 = &v1[1];
          if (v17[1])
            v12 = (uint1 *)sub_33640(v12,0,(uint1)dat_82b92);
          if (v1[1])
            v11 = (uint1 *)sub_33640(v11,0,(uint1)dat_82b92);
          if (*v12) { // branch-flip
            if (!*v11)
              v11 = v10;
          }
          else {
            v12 = v10;
          }
          if (!sub_1a700(v12,v11)) {
            free(v10);
            free(v4);
            sub_24660();
            return;
          }
          v17 = v1;
        }
      }
      else {
        v9 = strlen((char *)&v17[1]);
        v10 = (uint1 *)sub_4ece0(v10,v9 + 0xc);
        memcpy(v10,&v17[1],v9);
        v12 = &v10[v9 + 1];
        v10[v9] = 0x2f;
        v12[0] = 0x74;
        v12[1] = 0x61;
        v12[2] = 0x72;
        v12[3] = 0x2e;
        v12[4] = 0x58;
        v12[5] = 0x58;
        v12[6] = 0x58;
        v12[7] = 0x58;
        v12[8] = 0x58;
        v12[9] = 0x58;
        v12[10] = 0;
        if (!mkdtemp((char *)v10)) {
          if (dat_82d50)
            (*dat_82d50)();
          v8 = sub_4d2d0(v10);
          v6 = dcgettext(NULL,"Cannot create temporary directory using template %s",5);
          error(0,*__errno_location(),v6,v8);
          dat_82d58 = 2;
          free(v10);
          free(v4);
          sub_24660();
          return;
        }
      }
      v17 = &v17[strlen((char *)v17) + 1];
      v19 = *v17;
    } while (v19);
  }
  else {
label_212b3:
    v10 = NULL;
  }
  v6 = NULL;
  free(v10);
  v5 = (struct_63 *)sub_1e2e0(dat_829d0,"YND");
  if (*v4) {
    v15 = v4;
    do {
      free(v6);
      v6 = (char *)sub_29320(a0,v15);
      if (sub_25fc0(v6,v16)) { // branch-flip
        if (*__errno_location() == 2) goto label_2134f;
        sub_26460(v6);
        if (dat_82d50)
          (*dat_82d50)();
        v8 = sub_4ce40(v6);
        v7 = "%s: Not purging directory: unable to stat";
label_21337:
        error(0,0,dcgettext(NULL,v7,5),v8);
      }
      else {
        v7 = (char *)sub_1ec90(v5,v15);
        if (v7) { // branch-flip
          if (*v7 != 'D') { // branch-flip
            if ((*v7 == 'Y') && ((v20 & 0xf000) == 0x4000)) goto label_213c0;
          }
          else if ((v20 & 0xf000) != 0x4000) goto label_213c0;
        }
        else {
label_213c0:
          if ((dat_82ad1) && (v16[0] != dat_827f0)) {
            if (dat_82d50)
              (*dat_82d50)();
            v8 = sub_4ce40(v6);
            v7 = "%s: directory is on a different device: not purging";
            goto label_21337;
          }
          if ((!dat_82b40) || (sub_2df50("delete",v6))) {
            if (dat_82a54) {
              v13 = (char *)sub_4d2d0(v6);
              v7 = dat_82dd0;
              __fprintf_chk(dat_81f28,1,dcgettext(NULL,"%s: Deleting %s\n",5),v7,v13);
            }
            if (!sub_26a40(v6,1)) {
              v2 = *__errno_location();
              if (dat_82d50)
                (*dat_82d50)();
              v7 = (char *)sub_4ce40(v6);
              error(0,v2,dcgettext(NULL,"%s: Cannot remove",5),v7);
              dat_82d58 = 2;
            }
          }
        }
      }
label_2134f:
      v15 = &v15[strlen(v15) + 1];
    } while (*v15);
  }
  free(v6);
  free(v5->field_0x18);
  free(v5);
  free(v4); // tail-call
  return;
}


// Function: sub_21850 @ 0x21850
void sub_21850(uint1 *a0,int8 a1)
{
  uint1 v1;
  uint1 *v2;
  char *v3;
  bool v4;
  uint1 *v5; // rbx
  
  if (!a1)
    return;
  v4 = 0;
  while( true ) {
    a1 -= 1;
    v5 = &a0[1];
    v1 = *a0;
    if (v1) { // branch-flip
      if ((0x16 <= (uint1)((int4)(char)v1 - 0x44U)) || (!(0x314401UL >> ((uint8)(uint4)((int4)(char)v1 - 0x44U) & 0x3f) & 1))) {
        v2 = *(uint1 **)&dat_81f28->field_0x28;
        if (*(uint1 **)&dat_81f28->field_0x30 <= v2) // branch-flip
          __overflow(dat_81f28,(uint4)v1);
        else {
          *(uint1 **)&dat_81f28->field_0x28 = &v2[1];
          *v2 = v1;
        }
      }
      else {
        __fprintf_chk(dat_81f28,1,"%c");
        if (!v4)
          __fprintf_chk(dat_81f28,1," ");
        v4 = 1;
      }
    }
    else {
      v3 = *(char **)&dat_81f28->field_0x28;
      if (*(char **)&dat_81f28->field_0x30 <= v3) // branch-flip
        __overflow(dat_81f28,10);
      else {
        *(char **)&dat_81f28->field_0x28 = &v3[1];
        *v3 = 10;
      }
      v4 = 0;
    }
    if (!a1) break;
    a0 = v5;
  }
  return;
}


// Function: sub_21980 @ 0x21980
char * sub_21980(char *a0,int4 *a1)
{
  int4 v1;
  char *v2;
  char *v3; // rax
  
  v1 = *a1;
  if (v1 != 2) { // branch-flip
    if (v1 == 4) {
      v3 = a0;
      return v3;
    }
    v2 = a0;
    if (v1 == 1) {
      v3 = (char *)sub_33640(a0,0,(uint1)dat_82b92);
      v2 = v3;
    }
  }
  else {
    v3 = (char *)sub_33640(a0,1,(uint1)dat_82b92);
    v2 = v3;
  }
  if (!dat_82a90) {
    v3 = v2;
    return v3;
  }
  v3 = (uint8)sub_293b0(v2);
  if (v3 == (char *)0xffffffffffffffff)
    v3 = (uint8)strlen(v2);
  v3 = &v2[(int8)v3];
  return v3;
}


// Function: sub_21a20 @ 0x21a20
uint1 * sub_21a20(uint1 *a0,uint8 a1,char *a2,int8 a3,uint1 *a4,unsigned int a5,unsigned int a6) // early-return x5
{
  uint1 v1;
  uint1 *v10;
  uint8 v11; // rax
  uint1 v12;
  uint1 *v13;
  char v14 [8];
  char v15 [31];
  char v16 [999];
  char v17 [33];
  int8 v18;
  char *v19;
  char v2;
  char *v20; // rsp
  char *v21; // rsp
  uint1 *v22;
  uint1 *v23;
  uint1 *v24;
  void *v25; // r14
  uint1 *v26;
  bool v27; // zf
  bool v28;
  uint1 *v29; // stack - 0x490
  char *v3;
  uint1 *v30; // stack - 0x488
  uint1 *v31; // stack - 0x480
  unsigned int v32; // stack - 0x474
  uint1 *v33; // stack - 0x470
  uint4 v4; // eax
  uint1 *v5;
  int8 v6;
  char *v7; // rax
  char *v8; // rax
  char *v9;
  
  v13 = &a0[a1];
  v21 = v14;
  v19 = v14;
  v30 = (uint1 *)CONCAT44(v30._4_4_,a5);
  v32 = a6;
  v31 = a4;
  if (v13 != &a0[*a0 == 0]) {
    v33 = &a0[*a0 == 0];
    v5 = __ctype_b_loc();
    v10 = v33;
    do {
      v12 = *v10;
      if (!(*(uint1 *)(*(int8 *)v5 + 1 + (uint8)v12 * 2) & 0x20)) {
        v33 = (uint1 *)-a3;
        if ((uint4)((int4)(char)v12 - 0x30U) <= 7) {
          v22 = &v10[1];
          v24 = (uint1 *)(int8)((int4)(char)v12 - 0x30U);
          if (v13 != v22) {
            v28 = 0;
            v26 = v22;
            goto label_21b23;
          }
          v28 = 0;
          v26 = v13;
          goto label_21e48;
        }
        if (!(char)v30) {
          if (!(v12 - 0x2b & 0xfd)) {
            if ((!(char)v32) && (!dat_82450)) {
              dat_82450 = '\x01';
              v29 = v10;
              v30 = v5;
              if (dat_82d50)
                (*dat_82d50)();
              error(0,0,dcgettext(NULL,"Archive contains obsolescent base-64 headers",5));
              v12 = *v29;
              v10 = v29;
              v5 = v30;
            }
            v10 = &v10[1];
            v28 = v12 == 0x2d;
            v24 = NULL;
            if (v13 == v10) {
              if (v12 != 0x2d)
                return (uint1 *)0;
              return NULL;
            }
            goto label_21d3f;
          }
          if ((v12 == 0x80) || (v12 == 0xff)) {
            v10 = &v10[1];
            v4 = v12 & 0x40;
            v24 = (uint1 *)(int8)(int4)((v12 & 0x3f) - v4);
            goto label_22082;
          }
        }
        if (!v12)
          return NULL;
        goto label_21c23;
      }
      v10 = &v10[1];
    } while (v13 != v10);
  }
  if (!a2)
    return (uint1 *)0xffffffffffffffff;
  if ((char)v32 == '\x01')
    return (uint1 *)0xffffffffffffffff;
  if (dat_82d50)
    (*dat_82d50)();
  v9 = "Blanks in header where numeric %s value expected";
  error(0,0,dcgettext(NULL,v9,5),a2); // return-dupe
  dat_82d58 = 2;
  return (uint1 *)0xffffffffffffffff;
  while( true ) {
    v27 = (uint1 *)((uint8)v24 & 0x1fffffffffffffff) != v24;
    v24 = (uint1 *)((int8)(int4)v4 + (int8)v24 * 8);
    v26 = &v26[1];
    v28 = (bool)(v28 | v27);
    if (v13 == v26) break;
label_21b23:
    v1 = *v26;
    v11 = (uint8)(uint4)(int4)(char)v1;
    v4 = (int4)(char)v1 - 0x30;
    if (8 <= v4) {
      if ((v31 < v24) || (v28)) goto label_21e5a;
      if (v13 == v26)
        return v24;
      if (!v1)
        return v24;
      goto label_21b59;
    }
  }
label_21e48:
  if ((v24 <= v31) && (!v28))
    return v24;
label_21e5a:
  if ('2' <= (char)v12) {
    if (!a2) {
      if (v28)
        return (uint1 *)0xffffffffffffffff;
      goto label_21fcf;
    }
    v23 = (uint1 *)(int8)(7 - (int4)(char)(v12 - 0x30 | 4));
    if (v13 != v22) { // branch-flip
      v12 = 0;
      do {
        v1 = *v22;
        if (8 <= (uint4)((int4)(char)v1 - 0x30U)) {
          v26 = v22;
          goto label_21ed6;
        }
        v22 = &v22[1];
        v12 |= (uint1 *)((uint8)v23 & 0x1fffffffffffffff) != v23;
        v23 = (uint1 *)((int8)(7 - ((int4)(char)v1 - 0x30U)) + (int8)v23 * 8);
      } while (v13 != v22);
      v26 = v13;
    }
    else {
      v12 = 0;
      v26 = v13;
    }
label_21ed6:
    v24 = &v23[1];
    if ((bool)(v24 == NULL | v12)) goto label_22261;
    v28 = 0;
    if (v33 < v24) goto label_21fc6;
    if (!(char)v32) {
      v30 = v10;
      v31 = v23;
      v33 = v5;
      if (dat_82d50)
        (*dat_82d50)();
      v9 = dcgettext(NULL,"Archive octal value %.*s is out of %s range; assuming two\'s complement",5);
      error(0,0,v9,(int4)v26 - (int4)v30,(char *)v30,a2);
      v23 = v31;
      v5 = v33;
    }
    if ((v13 == v26) || (!*v26))
      return (uint1 *)~(uint8)v23;
    if (*(uint1 *)(*(int8 *)v5 + 1 + (uint8)*v26 * 2) & 0x20) {
      return (uint1 *)-(int8)v24; // return-dupe
    }
label_21c2c:
    if (!dat_82448) {
      dat_82448 = sub_4c700(NULL);
      sub_4c760(dat_82448,8);
    }
    for (; (a0 != v13 && (!v13[-1])); v13 = &v13[-1]) {
    }
    sub_4c820(&v18,1000,a0,(int8)v13 - (int8)a0,dat_82448);
    if ((char)v32)
      return (uint1 *)0xffffffffffffffff;
    if (dat_82d50)
      (*dat_82d50)();
    v9 = dcgettext(NULL,"Archive contains %.*s where numeric %s value expected",5);
    v11 = 1000;
    v13 = &v18;
    error(0,0,v9,v11,v13,a2); // return-dupe
    dat_82d58 = 2;
    return (uint1 *)0xffffffffffffffff;
  }
label_21fc6:
  if (v28) {
label_22261:
    if (!a2)
      return (uint1 *)0xffffffffffffffff;
    if ((char)v32 == '\x01')
      return (uint1 *)0xffffffffffffffff;
    v33 = v10;
    if (dat_82d50)
      (*dat_82d50)();
    v9 = dcgettext(NULL,"Archive octal value %.*s is out of %s range",5);
    v11 = (uint8)(uint4)((int4)v26 - (int4)v33);
    v13 = v33;
    error(0,0,v9,v11,v13,a2);
    dat_82d58 = 2;
    return (uint1 *)0xffffffffffffffff;
  }
label_21fcf:
  if ((v13 != v26) && (v11 = (uint8)*v26, *v26)) {
label_21b59:
    if (!(*(uint1 *)(*(int8 *)v5 + 1 + (v11 & 0xff) * 2) & 0x20)) {
label_21c23:
      if (!a2)
        return (uint1 *)0xffffffffffffffff;
      goto label_21c2c;
    }
  }
  goto label_21b70;
  while ((uint1 *)((uint8)v24 & 0xffffffffffffff | (uint8)-v4 << 0x32) == v24) {
label_22082:
    v1 = *v10;
    v10 = &v10[1];
    v24 = (uint1 *)((int8)v24 * 0x100 + (uint8)v1);
    if (v13 == v10) {
      v28 = (v12 & 0x40) != 0;
      if (v12 & 0x40)
        v24 = (uint1 *)-(int8)v24;
      goto label_21d65;
    }
  }
  if (!a2)
    return (uint1 *)0xffffffffffffffff;
  if ((char)v32 == '\x01')
    return (uint1 *)0xffffffffffffffff;
  if (dat_82d50)
    (*dat_82d50)();
  v9 = "Archive base-256 value is out of %s range";
  error(0,0,dcgettext(NULL,v9,5),a2);
  dat_82d58 = 2;
  return (uint1 *)0xffffffffffffffff;
  while( true ) {
    if ((uint1 *)((uint8)v24 & 0x3ffffffffffffff) != v24) {
      v3 = v14;
      while (v21 != &v14[-(a1 + 0x18 & 0xfffffffffffff000)]) {
        v19 = &v3[-0x1000];
        v20 = &v3[-0x1000];
        *(unsigned long *)&v3[-8] = *(unsigned long *)&v3[-8];
        v21 = &v3[-0x1000];
        v3 = v20;
      }
      v11 = (uint8)((uint4)(a1 + 0x18) & 0xff0);
      v6 = -v11;
      if (v11)
        *(unsigned long *)&v19[-8] = *(unsigned long *)&v19[-8];
      v25 = (void *)((uint8)&v19[v6 + 0xf] & 0xfffffffffffffff0);
      *(unsigned long *)&v19[v6 + -8] = 0x221a6;
      memcpy(v25,a0,a1);
      *(char *)((int8)v25 + a1) = 0;
      if ((char)v32 != '\x01') {
        if (a2) {
          if (dat_82d50) {
            *(unsigned long *)&v19[v6 + -8] = 0x221cf;
            (*dat_82d50)();
          }
          *(unsigned long *)&v19[v6 + -8] = 0x221d7;
          v9 = (char *)sub_4d2d0(v25);
          *(unsigned long *)&v19[v6 + -8] = 0x221ed;
          v8 = dcgettext(NULL,"Archive signed base-64 string %s is out of %s range",5);
          *(unsigned long *)&v19[v6 + -8] = 0x22201;
          error(0,0,v8,v9,a2);
          dat_82d58 = 2;
          return (uint1 *)0xffffffffffffffff;
        }
        return (uint1 *)0xffffffffffffffff;
      }
      return (uint1 *)0xffffffffffffffff;
    }
    v10 = &v10[1];
    v24 = (uint1 *)((int8)v24 << 6 | (int8)v2);
    if (v13 == v10) break;
label_21d3f:
    v12 = *v10;
    v2 = *(char *)((uint8)v12 + 0x82460);
    if ('@' <= v2) {
      if (((v10 != v13) && (v12)) && (!(*(uint1 *)(*(int8 *)v5 + 1 + (uint8)v12 * 2) & 0x20))) goto label_21c23;
      break;
    }
  }
label_21d65:
  if (v28) {
    if (v24 <= v33)
      return (uint1 *)-(int8)v24;
    if ((char)v32 == '\x01')
      return (uint1 *)0xffffffffffffffff;
    if (!a2)
      return (uint1 *)0xffffffffffffffff;
    v8 = (char *)sub_45370(v33,v15);
    v6 = sub_45370(v24,v16);
    *(char *)(v6 + -1) = 0x2d;
    v9 = (char *)(v6 + -1);
    goto label_21dbe;
  }
label_21b70:
  if (v24 <= v31)
    return v24;
  if (!a2)
    return (uint1 *)0xffffffffffffffff;
  if ((char)v32 == '\x01')
    return (uint1 *)0xffffffffffffffff;
  v8 = (char *)sub_45370(v33,v15);
  v9 = (char *)sub_45370(v24,v16);
label_21dbe:
  if (v33) {
    v8[-1] = '-';
    v8 = &v8[-1];
  }
  if (dat_82d50)
    (*dat_82d50)();
  v7 = (char *)sub_45370(v31,v17);
  error(0,0,dcgettext(NULL,"Archive value %s is out of %s range %s..%s",5),v9,a2,v8,v7);
  dat_82d58 = 2;
  return (uint1 *)0xffffffffffffffff;
}


// Function: sub_22480 @ 0x22480
undefined16 sub_22480(uint1 *a0,uint1 a1)
{
  uint1 v1;
  int4 v2; // eax
  char v3 [16];
  char v4 [16];
  char v5 [16];
  int4 v6; // ecx
  uint1 *v7;
  int4 v8; // esi
  unsigned long v9; // r8
  
  v8 = 0;
  v6 = 0;
  v7 = a0;
  do {
    v4._8_8_ = &v7[1];
    v6 += (uint4)*v7;
    v8 += (char)*v7;
    v7 = v4._8_8_;
  } while (&a0[0x200] != v4._8_8_);
  if (!v6) {
    v4._0_8_ = 3;
    return v4._0_16_;
  }
  v7 = &a0[0x9b];
  do {
    v1 = *v7;
    v7 = &v7[-1];
    v6 -= (uint4)v1;
    v8 -= (char)v1;
  } while (&a0[0x93] != v7);
  v5._9_7_ = 0;
  v5[8] = a1;
  v2 = sub_21a20(&a0[0x94],8,NULL,0,(uint1 *)0x7fffffff,1);
  if (0 <= v2) {
    if ((v6 + 0x100 == v2) || (v9 = 5, v8 + 0x100 == v2))
      v9 = 1;
    v3[8] = a1;
    v3._0_8_ = v9;
    v3._9_7_ = 0;
    return v3._0_16_;
  }
  v5._0_8_ = 5;
  return v5._0_16_;
}


// Function: sub_22570 @ 0x22570
int4 sub_22570(unsigned long *a0,unsigned long *a1,int4 a2)
{
  uint1 *v1;
  char *v10;
  uint8 v11;
  int8 v12; // rax
  uint8 v13;
  char v14 [16]; // stack - 0x168
  unsigned long v15; // stack - 0x148
  unsigned long *v16;
  unsigned long v17;
  unsigned long *v18; // stack - 0x190
  unsigned long *v19; // stack - 0x180
  unsigned long *v2;
  uint8 v20; // stack - 0x178
  uint8 v21; // stack - 0x170
  char v22 [16]; // stack - 0x158
  unsigned long v23; // stack - 0x140
  unsigned long v24; // stack - 0x138
  unsigned long v25; // stack - 0x130
  unsigned long v26; // stack - 0x128
  unsigned long v27; // stack - 0x120
  unsigned long v28; // stack - 0x118
  unsigned long v29; // stack - 0x110
  unsigned long *v3;
  unsigned long v30; // stack - 0x108
  unsigned long v31; // stack - 0x100
  unsigned long v32; // stack - 0xf8
  unsigned long v33; // stack - 0xf0
  uint8 v34; // stack - 0xe8
  unsigned long v35; // stack - 0xe0
  unsigned long v36; // stack - 0xd8
  unsigned long v37; // stack - 0xd0
  unsigned long v38; // stack - 0xc8
  unsigned long v39; // stack - 0xc0
  char v4;
  unsigned long v40; // stack - 0xb8
  unsigned short v41; // stack - 0xb0
  char v42; // stack - 0xae
  char v43; // stack - 0xad
  unsigned int v5;
  int4 v6; // eax
  unsigned long *v7;
  unsigned long *v8;
  void *v9; // rax
  
  v20 = 0;
  v21 = 0;
  v18 = NULL;
  v19 = NULL;
label_225e8:
  do {
    v7 = (unsigned long *)sub_d360();
    *a0 = v7;
    if (!v7) {
      v6 = 4;
      free(v19); // return-dupe
      free(v18);
      return v6;
    }
    v6 = sub_22480(v7,0);
    if (v6 != 1) {
      free(v19);
      free(v18);
      return v6;
    }
    if (*(char *)((int8)v7 + 0x9c) == '1') {
      a1[0x11] = 0;
label_22638:
      free(dat_823f8);
      v8 = &v19[0x40];
      if (!v19) {
        if (*(char *)((int8)v7 + 0x159)) { // branch-flip
          v10 = (char *)&v15;
          if (!strcmp((char *)((int8)v7 + 0x101),"ustar")) {
            v15 = *(unsigned long *)((int8)v7 + 0x159);
            v23 = *(unsigned long *)((int8)v7 + 0x161);
            v40 = *(unsigned long *)((int8)v7 + 0x1e9);
            v43 = 0;
            v24 = *(unsigned long *)((int8)v7 + 0x169);
            v25 = *(unsigned long *)((int8)v7 + 0x171);
            v26 = *(unsigned long *)((int8)v7 + 0x179);
            v27 = *(unsigned long *)((int8)v7 + 0x181);
            v28 = *(unsigned long *)((int8)v7 + 0x189);
            v29 = *(unsigned long *)((int8)v7 + 0x191);
            v30 = *(unsigned long *)((int8)v7 + 0x199);
            v31 = *(unsigned long *)((int8)v7 + 0x1a1);
            v32 = *(unsigned long *)((int8)v7 + 0x1a9);
            v33 = *(unsigned long *)((int8)v7 + 0x1b1);
            v42 = *(char *)((int8)v7 + 499);
            v34 = *(uint8 *)((int8)v7 + 0x1b9);
            v35 = *(unsigned long *)((int8)v7 + 0x1c1);
            v41 = *(unsigned short *)((int8)v7 + 0x1f1);
            v36 = *(unsigned long *)((int8)v7 + 0x1c9);
            v37 = *(unsigned long *)((int8)v7 + 0x1d1);
            v38 = *(unsigned long *)((int8)v7 + 0x1d9);
            v39 = *(unsigned long *)((int8)v7 + 0x1e1);
            v11 = strlen(v10);
            v10[v11] = '/';
            v10 = (char *)((int8)&v15 + v11 + 1);
          }
        }
        else {
          v10 = (char *)&v15;
        }
        v8 = &v15;
        v21 = 0;
        v17 = v7[1];
        *(unsigned long *)v10 = *v7;
        *(unsigned long *)&v10[8] = v17;
        v17 = v7[3];
        *(unsigned long *)&v10[0x10] = v7[2];
        *(unsigned long *)&v10[0x18] = v17;
        v17 = v7[5];
        *(unsigned long *)&v10[0x20] = v7[4];
        *(unsigned long *)&v10[0x28] = v17;
        v17 = v7[7];
        *(unsigned long *)&v10[0x30] = v7[6];
        *(unsigned long *)&v10[0x38] = v17;
        v17 = v7[9];
        *(unsigned long *)&v10[0x40] = v7[8];
        *(unsigned long *)&v10[0x48] = v17;
        v17 = v7[0xb];
        *(unsigned long *)&v10[0x50] = v7[10];
        *(unsigned long *)&v10[0x58] = v17;
        v5 = *(unsigned int *)&v7[0xc];
        v10[100] = '\0';
        *(unsigned int *)&v10[0x60] = v5;
      }
      dat_823f8 = v19;
      dat_823e8 = v21;
      sub_25120(a1,v8);
      sub_25120(&a1[1],v8);
      v10 = (char *)a1[1];
      *(char *)&a1[2] = sub_403c0(v10);
      free(dat_823f0);
      if (v18) // branch-flip
        v8 = &v18[0x40];
      else {
        v15 = *(unsigned long *)((int8)v7 + 0x9d);
        v23 = *(unsigned long *)((int8)v7 + 0xa5);
        v8 = &v15;
        v24 = *(unsigned long *)((int8)v7 + 0xad);
        v25 = *(unsigned long *)((int8)v7 + 0xb5);
        v26 = *(unsigned long *)((int8)v7 + 0xbd);
        v27 = *(unsigned long *)((int8)v7 + 0xc5);
        v20 = 0;
        v28 = *(unsigned long *)((int8)v7 + 0xcd);
        v29 = *(unsigned long *)((int8)v7 + 0xd5);
        v34 = CONCAT44((int4)(v34 >> 0x20),*(unsigned int *)((int8)v7 + 0xfd)) & 0xffffff00ffffffff;
        v30 = *(unsigned long *)((int8)v7 + 0xdd);
        v31 = *(unsigned long *)((int8)v7 + 0xe5);
        v32 = *(unsigned long *)((int8)v7 + 0xed);
        v33 = *(unsigned long *)((int8)v7 + 0xf5);
      }
      dat_823f0 = v18;
      dat_823e0 = v20;
      sub_25120(&a1[3],v8);
      v18 = NULL;
      v19 = NULL;
      free(v19);
      free(v18);
      return v6;
    }
    v1 = (uint1 *)((int8)v7 + 0x7c);
    v17 = 0;
    v11 = sub_21a20(v1,0xc,"off_t",0,(uint1 *)0x7fffffffffffffff,0);
    a1[0x11] = v11;
    if ((int8)v11 < 0) {
      v6 = 5;
      free(v19);
      free(v18);
      return v6;
    }
    v4 = *(char *)((int8)v7 + 0x9c);
    if (((uint1)(v4 + 0xb5U) < 2) || (v4 == 'x')) {
      if (a2 == 1) goto label_228a8;
      if (2 <= (uint1)(v4 + 0xb5U)) { // branch-flip
label_227ac:
        sub_1db80(&a1[0x2d],v7,sub_21a20(v1,0xc,"off_t",0,(uint1 *)0x7fffffffffffffff,0,0));
      }
      else {
        v13 = v11 + 0x200;
        if ((v11 & 0x1ff) && (v13 = (v11 + 0x400) - (uint8)((uint4)v11 & 0x1ff), v13 < v11))
          sub_17020(v17); // no-return
        v8 = (unsigned long *)sub_4ecc0(v13 + 1);
        if (*(char *)((int8)v7 + 0x9c) != 'L') { // branch-flip
          free(v18);
          v18 = v8;
          v20 = v13 >> 9;
        }
        else {
          free(v19);
          v19 = v8;
          v21 = v13 >> 9;
        }
        sub_ccd0(v7);
        v16 = &v8[0x40];
        v12 = (int8)v8 - (int8)(unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
        *v8 = *v7;
        v8[0x3f] = v7[0x3f];
        v11 = (uint8)((int4)v12 + 0x200U >> 3);
        v7 = (unsigned long *)((int8)v7 - v12);
        v8 = (unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
        while (v11) {
          v3 = &v8[1];
          v2 = &v7[1];
          *v8 = *v7;
          v11 -= 1;
          v7 = v2;
          v8 = v3;
        }
        for (v13 = v13 - 0x200; v13; v13 = v13 - v11) {
          v9 = (void *)sub_d360();
          if (!v9) {
            if (dat_82d50)
              (*dat_82d50)();
            error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
            dat_82d58 = 2;
            break;
          }
          v11 = sub_cd10(v9);
          if (v13 <= v11)
            v11 = v13;
          memcpy(v16,v9,v11);
          v16 = (unsigned long *)((int8)v16 + v11);
          sub_ccd0((int8)v9 + (v11 - 1));
        }
        *(char *)v16 = 0;
      }
      goto label_225e8;
    }
    if ((v4 != 'g') && (v4 != 'X')) goto label_22638;
    if (a2 == 1) goto label_228a8;
    if (v4 == 'X') goto label_227ac;
    v8 = dat_82560;
    if (!dat_82560) {
      v8 = (unsigned long *)sub_4ecc0(0x200);
      dat_82560 = v8;
    }
    *v8 = *v7;
    v8[0x3f] = v7[0x3f];
    v12 = (int8)v8 - (int8)(unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
    v11 = (uint8)((int4)v12 + 0x200U >> 3);
    v16 = (unsigned long *)((int8)v7 - v12);
    v8 = (unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
    while (v11) {
      v3 = &v8[1];
      v2 = &v16[1];
      *v8 = *v16;
      v11 -= 1;
      v16 = v2;
      v8 = v3;
    }
    v14._0_16_ = 0;
    v22._0_16_ = 0;
    sub_1db80(v14,v7,sub_21a20(v1,0xc,"off_t",0,(uint1 *)0x7fffffffffffffff,0,0));
    sub_1d980(v14);
    sub_1de20(v14);
    if (a2 == 2) {
label_228a8:
      v6 = 2;
      free(v19);
      free(v18);
      return v6;
    }
  } while( true );
}


// Function: sub_22c60 @ 0x22c60
void sub_22c60(int8 a0,int8 a1,int4 *a2,int4 a3) // return-dupe
{
  char v1;
  uint4 v2; // eax
  uint8 v3; // rax
  unsigned long v4; // rax
  char *v5;
  uint1 *v6;
  int4 v7; // r12d
  
  v3 = sub_21a20((uint1 *)(a0 + 100),8,"mode_t",-0x8000000000000000,(uint1 *)0xffffffffffffffff,0,0);
  if (strcmp((char *)(a0 + 0x101),"ustar")) { // branch-flip
    v7 = 1;
    if (!strcmp((char *)(a0 + 0x101),"ustar  "))
      v7 = (-(uint4)((v3 & 0xfffffffffffff000) == 0) & 4) + 2;
  }
  else if ((((*(char *)(a0 + 0x1db)) || (8 <= (uint1)(*(char *)(a0 + 0x1dc) - 0x30U))) || (*(char *)(a0 + 0x1e7) != ' ')) || ((8 <= (uint1)(*(char *)(a0 + 0x1e8) - 0x30U) || (v7 = 5, *(char *)(a0 + 499) != ' '))))
    v7 = 4 - (uint4)(*(int8 *)(a1 + 0x170) == 0);
  *a2 = v7;
  *(uint4 *)(a1 + 0x70) = (uint4)v3 & 0xfff;
  v4 = sub_21a20((uint1 *)(a0 + 0x88),0xc,"time_t",-0x8000000000000000,(uint1 *)0x7fffffffffffffff,0,0);
  v5 = NULL;
  v1 = *(char *)(a0 + 0x109);
  *(unsigned long *)(a1 + 0xf8) = v4;
  *(unsigned long *)(a1 + 0x100) = 0;
  if (v1)
    v5 = (char *)(a0 + 0x109);
  sub_25160((unsigned long *)(a1 + 0x20),v5,0x20);
  v5 = NULL;
  if (*(char *)(a0 + 0x129))
    v5 = (char *)(a0 + 0x129);
  sub_25160((unsigned long *)(a1 + 0x28),v5,0x20);
  sub_1d530(a1);
  if (v7 != 2) { // branch-flip
    if (v7 != 5) { // branch-flip
      *(unsigned long *)(a1 + 0x108) = dat_82a20;
      *(unsigned long *)(a1 + 0x110) = dat_82a28;
      *(unsigned long *)(a1 + 0xe8) = dat_82a20;
      *(unsigned long *)(a1 + 0xf0) = dat_82a28;
      if (v7 == 1) {
        v3 = 0;
        *(unsigned int *)(a1 + 0x74) = sub_21a20((uint1 *)(a0 + 0x6c),8,"uid_t",0,(uint1 *)0xffffffff,0,0);
        *(unsigned int *)(a1 + 0x78) = sub_21a20((uint1 *)(a0 + 0x74),8,"gid_t",0,(uint1 *)0xffffffff,0,v3 & 0xffffffff00000000);
        v3 = 0;
        goto label_22e6e;
      }
    }
    else {
      v3 = 0;
      v6 = (uint1 *)(a0 + 0x1e8);
      *(unsigned long *)(a1 + 0xe8) = sub_21a20((uint1 *)(a0 + 0x1dc),0xc,"time_t",-0x8000000000000000,(uint1 *)0x7fffffffffffffff,0,0);
label_23142:
      v4 = sub_21a20(v6,0xc,"time_t",-0x8000000000000000,(uint1 *)0x7fffffffffffffff,0,v3 & 0xffffffff00000000);
      *(unsigned long *)(a1 + 0x110) = 0;
      *(unsigned long *)(a1 + 0x108) = v4;
      *(unsigned long *)(a1 + 0xf0) = 0;
    }
  }
  else {
    if (dat_82b50) {
      v3 = 0;
      v6 = (uint1 *)(a0 + 0x165);
      *(unsigned long *)(a1 + 0xe8) = sub_21a20((uint1 *)(a0 + 0x159),0xc,"time_t",-0x8000000000000000,(uint1 *)0x7fffffffffffffff,0,0);
      goto label_23142;
    }
    *(unsigned long *)(a1 + 0x108) = dat_82a20;
    *(unsigned long *)(a1 + 0x110) = dat_82a28;
    *(unsigned long *)(a1 + 0xe8) = dat_82a20;
    *(unsigned long *)(a1 + 0xf0) = dat_82a28;
  }
  if (a3) {
    if (((dat_82ad2) || (!*(char *)(a0 + 0x109))) || (!sub_27fd0((char *)(a0 + 0x109),(unsigned int *)(a1 + 0x74))))
      *(unsigned int *)(a1 + 0x74) = sub_21a20((uint1 *)(a0 + 0x6c),8,"uid_t",0,(uint1 *)0xffffffff,0,0);
    if (((dat_82ad2) || (!*(char *)(a0 + 0x129))) || (!sub_28080((char *)(a0 + 0x129),(unsigned int *)(a1 + 0x78))))
      *(unsigned int *)(a1 + 0x78) = sub_21a20((uint1 *)(a0 + 0x74),8,"gid_t",0,(uint1 *)0xffffffff,0,0);
  }
  v3 = 0;
  if ((uint4)((int4)*(char *)(a0 + 0x9c) - 0x33U) < 2) {
    v3 = sub_21a20((uint1 *)(a0 + 0x151),8,"minor_t",-0x80000000,(uint1 *)0x7fffffff,0,0);
    v2 = sub_21a20((uint1 *)(a0 + 0x149),8,"major_t",-0x80000000,(uint1 *)0x7fffffff,0);
    v3 = ((uint8)v2 & 0xfffff000) << 0x20 | (v3 & 0xffffff00) << 0xc | v3 & 0xff | (uint8)((v2 & 0xfff) << 8);
  }
label_22e6e:
  *(uint8 *)(a1 + 0x80) = v3;
  sub_1d8a0(a1);
  if (sub_2b3a0(a1)) {
    sub_2b400(a1);
    *(char *)(a1 + 0x120) = 1;
    return;
  }
  *(char *)(a1 + 0x120) = 0;
  if ((((dat_82400 & 0xfffffffb) != 2) || (*(char *)(dat_82408 + 0x9c) != 'D')) && (!*(int8 *)(a1 + 400)))
    return;
  *(char *)(a1 + 0x188) = 1;
  return;
}


// Function: sub_23240 @ 0x23240
int8 sub_23240(int8 a0,int4 a1,bool a2) // early-return x2
{
  tm *v1;
  uint8 v2;
  uint8 v3;
  uint8 v4;
  int8 v5; // rax
  int8 v6; // stack - 0x28
  
  v6 = a0;
  if ((a1) && (a0 <= -1)) {
    v6 = a0 + 1;
    a1 = 1000000000 - a1;
  }
  if (dat_82b91) // branch-flip
    v1 = gmtime(&v6);
  else {
    v1 = localtime(&v6);
  }
  if (v1) {
    if (!a2) {
      strftime((char *)0x82420,0x25,"%Y-%m-%d %H:%M",v1);
      return 0x82420;
    }
    strftime((char *)0x82420,0x25,"%Y-%m-%d %H:%M:%S",v1);
    sub_256a0(a1,(char *)(strlen((char *)0x82420) + 0x82420));
    return 0x82420;
  }
  if (0 <= a0) // branch-flip
    v2 = sub_45370(v6,0x82426);
  else {
    v5 = sub_45370(-v6,0x82426);
    v2 = v5 - 1;
    *(char *)(v5 + -1) = 0x2d;
  }
  v3 = 0x82441;
  if (!a2)
    v3 = 0x82434;
  for (v4 = v2; v3 < v4; v4 = v4 - 1) {
    *(char *)(v4 - 1) = 0x20;
  }
  v5 = v3 - v2;
  if (v2 < v3)
    v5 = 0;
  if (a2)
    sub_256a0(a1,(char *)0x8243a);
  return v2 + v5;
}


// Function: sub_233e0 @ 0x233e0
void sub_233e0(struct_9 *a0,int8 a1,int8 a2)
{
  char *v1;
  char v10 [2];
  char v11 [32];
  char v12 [32];
  char v13 [32];
  char v14; // stack - 0xe4
  char v15; // stack - 0xe6
  int4 v16;
  char *v17;
  char *v18; // stack - 0x110
  char *v19; // stack - 0xf0
  FILE *v2;
  char v20; // stack - 0xe5
  int4 v3; // eax
  char *v4;
  char *v5;
  uint8 v6;
  unsigned long v7;
  char v8 [56];
  char v9 [9];
  
  if (dat_827e1) { // branch-flip
    v17 = a0->field_0x8;
    if (!v17)
      v17 = a0->field_0x0;
  }
  else {
    v17 = a0->field_0x0;
    if (!v17)
      v17 = a0->field_0x8;
  }
  if (dat_82b7c) {
    if (a2 < 0)
      a2 = sub_cc60();
    v4 = (char *)sub_45370((a2 - dat_823e0) - dat_823e8,v8);
    __fprintf_chk(dat_81f28,1,dcgettext(NULL,"block %s: ",5),v4);
  }
  v2 = dat_81f28;
  if (dat_82a54 <= 1) {
    fputs_unlocked((char *)sub_4ca70(v17),v2);
    if ((dat_827e1) && (a0->field_0x10)) {
      v1 = *(char **)&dat_81f28->field_0x28;
      if (*(char **)&dat_81f28->field_0x30 <= v1) // branch-flip
        __overflow(dat_81f28,0x2f);
      else {
        *(char **)&dat_81f28->field_0x28 = &v1[1];
        *v1 = 0x2f;
      }
    }
label_237af:
    v1 = *(char **)&dat_81f28->field_0x28;
    if (*(char **)&dat_81f28->field_0x30 <= v1) // branch-flip
      __overflow(dat_81f28,10);
    else {
      *(char **)&dat_81f28->field_0x28 = &v1[1];
      *v1 = 10;
    }
    fflush_unlocked(dat_81f28); // return-dupe
    sub_32830(a0);
    return;
  }
  v14 = '?';
  switch(*(char *)(a1 + 0x9c)) {
    case 0:
    case 0x30:
    case 0x53:
      v14 = (-(a0->field_0x10 == '\0') & 0xc9U) + 100;
      break;
    case 0x31:
      v14 = 'h';
      break;
    case 0x32:
      v14 = 'l';
      break;
    case 0x33:
      v14 = 'c';
      break;
    case 0x34:
      v14 = 'b';
      break;
    case 0x35:
    case 0x44:
      v14 = 'd';
      break;
    case 0x36:
      v14 = 'p';
      break;
    case 0x37:
      v14 = 'C';
      break;
    case 0x4b:
    case 0x4c:
      v14 = 'L';
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Unexpected long name header",5));
      dat_82d58 = 2;
      break;
    case 0x4d:
      v14 = 'M';
      break;
    case 0x56:
      dat_82451 = 1;
      v14 = 'V';
    
  }
  sub_329f0(a0->field_0x70,v9);
  sub_326f0(a0,v10);
  v18 = (char *)sub_23240(a0->field_0xf8,a0->field_0x100,(uint1)dat_82b90);
  v16 = (int4)strlen(v18);
  if (dat_7f228 < v16)
    dat_7f228 = v16;
  v4 = a0->field_0x20;
  if ((((!v4) || (!*v4)) || (dat_82400 == 1)) || (dat_82ad2))
    v4 = (char *)sub_45370(a0->field_0x74,v11);
  v5 = a0->field_0x28;
  if (((!v5) || (!*v5)) || ((dat_82400 == 1 || (dat_82ad2))))
    v5 = (char *)sub_45370(a0->field_0x78,v12);
  if (2 <= (uint4)((int4)*(char *)(a1 + 0x9c) - 0x33U)) { // branch-flip
    v6 = a0->field_0x88;
    __strcpy_chk(v8,(char *)sub_45370(v6,v13),0x2a);
  }
  else {
    v7 = a0->field_0x80;
    __strcpy_chk(v8,(char *)sub_45370((uint4)((uint8)v7 >> 0x20) & 0xfffff000 | (uint4)((uint8)v7 >> 8) & 0xfff,v13),0x2a);
    __strcat_chk(v8,",",0x2a);
    v6 = a0->field_0x80;
    __strcat_chk(v8,(char *)sub_45370((uint4)((v6 >> 0x14) << 8) | (uint4)v6 & 0xff,v13),0x2a);
  }
  v19 = v13;
  v16 = (int4)strlen(v8);
  v6 = strlen(v4);
  v3 = (int4)v6 + 2 + (int4)strlen(v5) + v16;
  if (v3 <= dat_7f22c) // branch-flip
    v16 = (dat_7f22c - v3) + v16;
  else {
    dat_7f22c = v3;
  }
  __fprintf_chk(dat_81f28,1,"%s %s/%s %*s %-*s",&v14,v4,v5,v16,v8,dat_7f228,v18);
  __fprintf_chk(dat_81f28,1," %s",(char *)sub_4ca70(v17));
  if ((dat_827e1) && (a0->field_0x10)) {
    v1 = *(char **)&dat_81f28->field_0x28;
    if (*(char **)&dat_81f28->field_0x30 <= v1) // branch-flip
      __overflow(dat_81f28,0x2f);
    else {
      *(char **)&dat_81f28->field_0x28 = &v1[1];
      *v1 = 0x2f;
    }
  }
  switch(*(char *)(a1 + 0x9c)) {
    case 0:
    case 0x30:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x44:
    case 0x53:
      goto label_237af;
    default:
      v20 = 0;
      v15 = *(char *)(a1 + 0x9c);
      v7 = sub_4d2d0(&v15);
      v17 = " unknown file type %s\n";
      goto label_2385a;
    case 0x31:
      v7 = sub_4ca70(a0->field_0x18);
      v17 = " link to %s\n";
label_2385a:
      __fprintf_chk(dat_81f28,1,dcgettext(NULL,v17,5),v7);
      fflush_unlocked(dat_81f28);
      sub_32830(a0);
      return;
    case 0x32:
      v17 = a0->field_0x18;
      __fprintf_chk(dat_81f28,1," -> %s\n",sub_4ca70(v17));
      fflush_unlocked(dat_81f28);
      sub_32830(a0);
      return;
    case 0x4b:
      v17 = "--Long Link--\n";
      break;
    case 0x4c:
      v17 = "--Long Name--\n";
      break;
    case 0x4d:
      v6 = sub_21a20((uint1 *)(a1 + 0x171),0xc,"uintmax_t",0,(uint1 *)0xffffffffffffffff,0,0);
      __strcpy_chk(v8,(char *)sub_45370(v6,v19),0x2a);
      __fprintf_chk(dat_81f28,1,dcgettext(NULL,"--Continued at byte %s--\n",5),v8);
      fflush_unlocked(dat_81f28);
      sub_32830(a0);
      return;
    case 0x56:
      v17 = "--Volume Header--\n";
    
  }
  __fprintf_chk(dat_81f28,1,dcgettext(NULL,v17,5));
  fflush_unlocked(dat_81f28);
  sub_32830(a0);
  return;
}


// Function: sub_23ae0 @ 0x23ae0
void sub_23ae0(void)
{
  unsigned long *v1;
  char v10; // stack - 0x18c
  int8 v2; // rcx
  unsigned long v3 [17];
  char v4 [8];
  char v5 [440];
  char v6 [4];
  unsigned long *v7;
  unsigned long v8; // stack - 0x1a0
  unsigned int v9; // stack - 0x198
  
  v2 = 0x40;
  v7 = v3;
  while (v2) {
    v1 = &v7[1];
    *v7 = 0;
    v2 -= 1;
    v7 = v1;
  }
  v10 = 0x56;
  if (dat_82560) {
    v8 = *(unsigned long *)(dat_82560 + 0x88);
    v9 = *(unsigned int *)(dat_82560 + 0x90);
  }
  sub_2fb40(v4);
  sub_25120(v5,".");
  sub_22c60(v3,v4,v6,0);
  sub_25120(v5,dat_81f18);
  sub_233e0(v4,v3,0);
  sub_2fc00(v4);
  return;
}


// Function: sub_23c30 @ 0x23c30
void sub_23c30(char *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  char v3 [10];
  char v4; // stack - 0x43
  char v5 [24];
  
  if (dat_82a54 <= 1)
    return;
  v4 = 'd';
  sub_329f0(a2,v3);
  if (dat_82b7c) {
    v1 = (char *)sub_45370(sub_cc60(),v5);
    __fprintf_chk(dat_81f28,1,dcgettext(NULL,"block %s: ",5),v1);
  }
  v1 = (char *)sub_4ca70(a0);
  v2 = dcgettext(NULL,"Creating directory:",5);
  __fprintf_chk(dat_81f28,1,"%s %*s %s\n",&v4,dat_7f22c + 1 + dat_7f228,v2,v1);
  return;
}


// Function: sub_23d40 @ 0x23d40
void sub_23d40(int8 a0) // return-dupe
{
  uint8 v1;
  uint8 v2; // rax
  int8 v3;
  
  if (dat_827f8) {
    v3 = sub_d780(a0);
    if (0 <= v3) // branch-flip
      a0 += v3 * -0x200;
    else {
      dat_827f8 = '\0';
    }
  }
  sub_c710(a0);
  if (a0 <= 0)
    return;
  v1 = a0 - 1;
  v3 = a0 + -0x200;
  do {
    v2 = sub_d360();
    if (!v2) {
      if (dat_82d50)
        (*dat_82d50)(0);
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      sub_16fd0(); // no-return
    }
    sub_ccd0(v2);
    a0 -= 0x200;
    sub_c710(a0);
  } while (a0 != v3 - (v1 & 0xfffffffffffffe00));
  return;
}


// Function: sub_23e80 @ 0x23e80
void sub_23e80(code *a0)
{
  char v1;
  int8 v2;
  unsigned long v3; // rax
  int8 v4; // rdx
  char v5 [24];
  int4 v6; // edi
  
  v4 = 0x41;
  dat_82460 = s_65520._64_8_;
  dat_82468 = s_65520._72_8_;
  dat_82470 = s_65520._64_8_;
  dat_82478 = s_65520._72_8_;
  dat_82480 = s_65520._64_8_;
  dat_82488 = s_65520._72_8_;
  dat_82490 = s_65520._64_8_;
  dat_82498 = s_65520._72_8_;
  dat_824a0 = s_65520._64_8_;
  dat_824a8 = s_65520._72_8_;
  dat_824b0 = s_65520._64_8_;
  dat_824b8 = s_65520._72_8_;
  dat_824c0 = s_65520._64_8_;
  dat_824c8 = s_65520._72_8_;
  dat_824d0 = s_65520._64_8_;
  dat_824d8 = s_65520._72_8_;
  dat_824e0 = s_65520._64_8_;
  dat_824e8 = s_65520._72_8_;
  dat_824f0 = s_65520._64_8_;
  dat_824f8 = s_65520._72_8_;
  dat_82500 = s_65520._64_8_;
  dat_82508 = s_65520._72_8_;
  dat_82510 = s_65520._64_8_;
  dat_82518 = s_65520._72_8_;
  dat_82520 = s_65520._64_8_;
  dat_82528 = s_65520._72_8_;
  dat_82530 = s_65520._64_8_;
  dat_82538 = s_65520._72_8_;
  dat_82540 = s_65520._64_8_;
  dat_82548 = s_65520._72_8_;
  dat_82550 = s_65520._64_8_;
  dat_82558 = s_65520._72_8_;
  v2 = 0;
  while( true ) {
    *(char *)(v4 + 0x82460) = (char)v2;
    if (v2 + 1 == 0x40) break;
    v4 = (int8)"BCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s"[v2];
    v2 += 1;
  }
  sub_282f0();
  v6 = 0;
  sub_f710(0);
label_23f7c:
  sub_2fc00((unsigned long *)0x82840);
  switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
    case 0:
    case 2:
label_24510:
      abort(); // no-return
    case 1:
label_2406e:
label_24075:
      sub_22c60(dat_82408,0x82840,(int4 *)0x82400,1);
      v1 = sub_28880(dat_82848);
      if (v1) {
        if (0 <= dat_82b08) {
          dat_82938 = sub_21a20((uint1 *)(dat_82408 + 0x88),0xc,"time_t",-0x8000000000000000,(uint1 *)0x7fffffffffffffff,0,0);
          dat_82940 = 0;
          if ((int4)((-(uint4)(0 < dat_82b08) - (int4)(dat_82b08 >> 0x3f)) + ((uint4)(dat_82b00 < dat_82938) - (uint4)(dat_82938 < dat_82b00)) * 2) < 0) goto label_240a0;
        }
        v1 = sub_176d0(dat_82848,dat_829d8);
        if (v1) goto label_240a0;
        v6 = (int4)*(char *)(dat_82408 + 0x9c);
        if (v6 == 0x56) goto label_242f7;
label_24503:
        sub_222e0(v6,0x82840);
label_242f7:
        (*a0)();
        if (!sub_289e0(0x82840)) goto label_2430d;
        break;
      }
label_240a0:
      v1 = *(char *)(dat_82408 + 0x9c);
      if (v1 == 'M') {
        v6 = 0x4d;
        goto label_24503;
      }
      if (v1 == 'V') goto label_242f7;
      if ((v1 == '5') && (dat_82a89)) {
        if (dat_82d50)
          (*dat_82d50)();
        v3 = sub_4ce40(dat_82848);
        error(0,0,dcgettext(NULL,"%s: Omitting",5),v3);
      }
      if (!dat_829c9) {
        sub_23e00();
        if (sub_289e0(0x82840)) break;
        sub_2fc00((unsigned long *)0x82840);
        switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
          case 0:
          case 2:
            goto label_24510;
          case 1:
            goto label_24075;
          case 3:
            goto label_24441;
          case 4:
            goto label_23fa2;
          case 5:
            goto label_2444c;
          default:
            goto label_23faf;
          
        }
      }
      if (sub_289e0(0x82840)) break;
      sub_2fc00((unsigned long *)0x82840);
      switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
        case 0:
        case 2:
          goto label_24510;
        case 1:
          goto label_24075;
        case 3:
          goto label_24441;
        case 4:
          goto label_23fa2;
        case 5:
          goto label_2444c;
        default:
          break;
        
      }
    case 3:
      goto label_23fe1;
    case 4:
label_23fa2:
      if (dat_82b7c) {
        v3 = sub_45370(sub_cc60(),v5);
        __fprintf_chk(dat_81f28,1,dcgettext(NULL,"block %s: ** End of File **\n",5),v3);
      }
      break;
    case 5:
      sub_ccd0(dat_82408);
      if (v6) {
        if (v6 == 1) goto label_2415c;
        goto label_241f4;
      }
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
      dat_82d58 = 2;
      goto label_2415c;
    
  }
  sub_d8d0(); // return-dupe
  sub_28a70(); // tail-call
  return;
label_2430d:
  sub_2fc00((unsigned long *)0x82840);
  switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
    case 0:
    case 2:
      goto label_24510;
    case 1:
      goto label_24075;
    case 3:
label_24441:
      v6 = 1;
      goto label_23fe1;
    case 4:
      goto label_23fa2;
    case 5:
      goto label_2444c;
    default:
      goto label_23faf;
    
  }
label_2444c:
  sub_ccd0(dat_82408);
label_2415c:
  if (dat_82b7c) {
    v3 = sub_45370((sub_cc60() - dat_823e0) - dat_823e8,v5);
    __fprintf_chk(dat_81f28,1,dcgettext(NULL,"block %s: ",5),v3);
  }
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"Skipping to next header",5));
  dat_82d58 = 2;
  if (sub_289e0(0x82840)) {
    sub_d8d0();
    sub_28a70();
    return;
  }
  sub_2fc00((unsigned long *)0x82840);
  switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
    case 0:
    case 2:
      goto label_24510;
    case 1:
      goto label_2406e;
    case 3:
      goto label_23fdb;
    case 4:
      goto label_23fa2;
    case 5:
      sub_ccd0(dat_82408);
label_241f4:
      do {
        if (sub_289e0(0x82840)) {
          sub_d8d0();
          sub_28a70();
          return;
        }
        sub_2fc00((unsigned long *)0x82840);
        switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
          case 0:
          case 2:
            goto label_24510;
          case 1:
            goto label_2406e;
          case 3:
            goto label_23fdb;
          case 4:
            goto label_23fa2;
          case 5:
            sub_ccd0(dat_82408);
            if (sub_289e0(0x82840)) {
              sub_d8d0();
              sub_28a70();
              return;
            }
            sub_2fc00((unsigned long *)0x82840);
            switch(sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0)) {
              case 0:
              case 2:
                goto label_24510;
              case 1:
                goto label_2406e;
              case 3:
                goto label_23fdb;
              case 4:
                goto label_23fa2;
              case 5:
                sub_ccd0(dat_82408);
                break;
              default:
                goto label_23faf;
              
            }
            break;
          default:
            goto label_23faf;
          
        }
      } while( true );
    default:
      goto label_23faf;
    
  }
label_23fdb:
  v6 = 5;
label_23fe1:
  if (dat_82b7c) {
    v3 = sub_45370(sub_cc60(),v5);
    __fprintf_chk(dat_81f28,1,dcgettext(NULL,"block %s: ** Block of NULs **\n",5),v3);
  }
  sub_ccd0(dat_82408);
  if (!dat_82b51) {
    if ((sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0) != 3) && (dat_81b80 & 1)) {
      if (dat_82d50)
        (*dat_82d50)();
      v3 = sub_45370(sub_cc60(),v5);
      error(0,0,dcgettext(NULL,"A lone zero block at %s",5),v3);
    }
    sub_d8d0();
    sub_28a70();
    return;
  }
  if (sub_289e0(0x82840)) {
label_23faf:
    sub_d8d0();
    sub_28a70();
    return;
  }
  goto label_23f7c;
}


// Function: sub_24580 @ 0x24580
void sub_24580(void)
{
  unsigned long v1;
  int8 v2; // rax
  
  v2 = sub_cc60();
  v1 = dat_82408;
  if (dat_82a54) {
    if (((dat_82400 == 4) && (!dat_82451)) && (dat_81f18)) {
      sub_23ae0();
      dat_82451 = '\x01';
    }
    sub_233e0((struct_9 *)0x82840,v1,v2);
    if ((dat_82b50) && (3 <= dat_82a54)) {
      if (sub_20fc0(0x82840)) {
        v2 = sub_1f690(dat_829d0);
        sub_21850(dat_829d0,v2);
      }
    }
  }
  if (dat_829c9)
    return;
  sub_23e00(); // tail-call
  return;
}


// Function: sub_24680 @ 0x24680
void sub_24680(void)
{
  char v1; // al
  int8 v2;
  char *v3; // rax
  int8 v4; // rdx
  
  v4 = 0x41;
  dat_82460 = s_65520._64_8_;
  dat_82468 = s_65520._72_8_;
  dat_82470 = s_65520._64_8_;
  dat_82478 = s_65520._72_8_;
  dat_82480 = s_65520._64_8_;
  dat_82488 = s_65520._72_8_;
  dat_82490 = s_65520._64_8_;
  dat_82498 = s_65520._72_8_;
  dat_824a0 = s_65520._64_8_;
  dat_824a8 = s_65520._72_8_;
  dat_824b0 = s_65520._64_8_;
  dat_824b8 = s_65520._72_8_;
  dat_824c0 = s_65520._64_8_;
  dat_824c8 = s_65520._72_8_;
  dat_824d0 = s_65520._64_8_;
  dat_824d8 = s_65520._72_8_;
  dat_824e0 = s_65520._64_8_;
  dat_824e8 = s_65520._72_8_;
  dat_824f0 = s_65520._64_8_;
  dat_824f8 = s_65520._72_8_;
  dat_82500 = s_65520._64_8_;
  dat_82508 = s_65520._72_8_;
  dat_82510 = s_65520._64_8_;
  dat_82518 = s_65520._72_8_;
  dat_82520 = s_65520._64_8_;
  dat_82528 = s_65520._72_8_;
  dat_82530 = s_65520._64_8_;
  dat_82538 = s_65520._72_8_;
  dat_82540 = s_65520._64_8_;
  dat_82548 = s_65520._72_8_;
  dat_82550 = s_65520._64_8_;
  dat_82558 = s_65520._72_8_;
  v2 = 0;
  while( true ) {
    *(char *)(v4 + 0x82460) = (char)v2;
    if (v2 + 1 == 0x40) break;
    v4 = (int8)"BCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s"[v2];
    v2 += 1;
  }
  sub_282f0();
  sub_f710(0);
  if (sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0) == 1) {
    sub_22c60(dat_82408,0x82840,(int4 *)0x82400,0);
    if (*(char *)(dat_82408 + 0x9c) == 'V')
      sub_25160((unsigned long *)0x81f18,dat_82408,100);
    if (dat_81f18) {
      if (dat_82a54)
        sub_23ae0();
      v1 = sub_28880(dat_81f18);
      if ((!v1) && (dat_82b10)) {
        v3 = (char *)sub_f3f0(dat_81f18);
        sub_28880(v3);
        free(v3);
      }
    }
  }
  sub_d8d0();
  sub_28c00(); // tail-call
  return;
}


// Function: sub_247f0 @ 0x247f0
uint8 sub_247f0(uint8 *a0,uint8 a1)
{
  return *a0 % a1;
}


// Function: sub_24800 @ 0x24800
unsigned long sub_24800(int8 *a0,int8 *a1)
{
  return CONCAT71((undefined7)((uint8)*a1 >> 8),*a0 == *a1);
}


// Function: sub_24810 @ 0x24810
uint8 sub_24810(char *a0)
{
  passwd *v1; // rax
  
  v1 = getpwnam(a0);
  if (v1)
    return (uint8)*(uint4 *)&v1->field_0x10;
  return 0xffffffffffffffff;
}


// Function: sub_24840 @ 0x24840
uint8 sub_24840(char *a0)
{
  group *v1; // rax
  
  v1 = getgrnam(a0);
  if (v1)
    return (uint8)*(uint4 *)&v1->field_0x10;
  return 0xffffffffffffffff;
}


// Function: sub_24de0 @ 0x24de0
void sub_24de0(char *a0)
{
  sub_24950((int8 *)0x82570,a0,sub_24810,"UID"); // tail-call
  return;
}


// Function: sub_24e10 @ 0x24e10
bool sub_24e10(uint8 a0,int4 *a1,int8 *a2) // early-return
{
  int8 v1;
  int8 v2; // rax
  uint8 v3 [3]; // stack - 0x38
  
  if (dat_82570) {
    v3[0] = a0 & 0xffffffff;
    v2 = sub_43960(dat_82570,v3);
    if (v2) {
      v1 = *(int8 *)(v2 + 0x10);
      *a1 = (int4)*(unsigned long *)(v2 + 8);
      *a2 = v1;
      return 0;
    }
  }
  if (dat_82abc != -1)
    *a1 = dat_82abc;
  if (!dat_82ac0)
    return dat_82abc == -1;
  *a2 = dat_82ac0;
  return 0;
}


// Function: sub_24eb0 @ 0x24eb0
void sub_24eb0(char *a0)
{
  sub_24950((int8 *)0x82568,a0,sub_24840,"GID"); // tail-call
  return;
}


// Function: sub_24ee0 @ 0x24ee0
bool sub_24ee0(uint8 a0,int4 *a1,int8 *a2) // early-return
{
  int8 v1;
  int8 v2; // rax
  uint8 v3 [3]; // stack - 0x38
  
  if (dat_82568) {
    v3[0] = a0 & 0xffffffff;
    v2 = sub_43960(dat_82568,v3);
    if (v2) {
      v1 = *(int8 *)(v2 + 0x10);
      *a1 = (int4)*(unsigned long *)(v2 + 8);
      *a2 = v1;
      return 0;
    }
  }
  if (dat_82b54 != -1)
    *a1 = dat_82b54;
  if (!dat_82b58)
    return dat_82b54 == -1;
  *a2 = dat_82b58;
  return 0;
}


// Function: sub_250f0 @ 0x250f0
void sub_250f0(unsigned int a0,char *a1)
{
  sub_4cf60(a0,sub_4c740(NULL),a1); // tail-call
  return;
}


// Function: sub_25160 @ 0x25160
void sub_25160(unsigned long *a0,char *a1,uint8 a2) // return-dupe
{
  uint8 v1; // rax
  void *v2; // rax
  
  free((void *)*a0);
  if (!a1)
    return;
  v1 = strnlen(a1,a2);
  v2 = memcpy((void *)sub_4ecc0(v1 + 1),a1,v1);
  *(char *)((int8)v2 + v1) = 0;
  *a0 = v2;
  return;
}


// Function: sub_251b0 @ 0x251b0
unsigned long sub_251b0(char *a0)
{
  char *v1;
  char v2;
  char *v3;
  char *v4;
  unsigned long v5; // r8
  
  v2 = *a0;
  if (!v2)
    return 1;
  v5 = 1;
  v3 = a0;
  do {
    while( true ) {
      v4 = &v3[1];
      v1 = &a0[1];
      if (v2 != '\\') break;
      switch(v3[1]) {
        case 0x30:
        case 0x31:
        case 0x32:
        case 0x33:
        case 0x34:
        case 0x35:
        case 0x36:
        case 0x37:
          v2 = v3[1] + '\xd0';
          if (8 <= (uint1)(v3[2] - 0x30U)) { // branch-flip
            *a0 = v2;
            v4 = &v3[2];
            v2 = *v4;
          }
          else {
            v2 = v3[2] + '\xd0' + v2 * '\b';
            if (8 <= (uint1)(v3[3] - 0x30U)) { // branch-flip
              *a0 = v2;
              v4 = &v3[3];
              v2 = *v4;
            }
            else {
              v4 = &v3[4];
              *a0 = v3[3] + '\xd0' + v2 * '\b';
              v2 = *v4;
            }
          }
          break;
        default:
          *a0 = '\\';
          if (!v3[1]) {
            v5 = 0;
            goto label_25230;
          }
          a0[1] = v3[1];
          v4 = &v3[2];
          v2 = *v4;
          v1 = &a0[2];
          v5 = 0;
          break;
        case 0x3f:
          *a0 = '\x7f';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x5c:
          *a0 = '\\';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x61:
          *a0 = '\a';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x62:
          *a0 = '\b';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x66:
          *a0 = '\f';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x6e:
          *a0 = '\n';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x72:
          *a0 = '\r';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x74:
          *a0 = '\t';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x76:
          *a0 = '\v';
          v4 = &v3[2];
          v2 = *v4;
        
      }
label_251e9:
      a0 = v1;
      v3 = v4;
      if (!v2) goto label_25230;
    }
    if (a0 != v3) {
      *a0 = v2;
      v2 = v3[1];
      goto label_251e9;
    }
    v2 = a0[1];
    a0 = v1;
    v3 = v4;
  } while (v2);
label_25230:
  if (v4 != v1)
    *v1 = '\0';
  return v5;
}


// Function: sub_253a0 @ 0x253a0
char * sub_253a0(char *a0)
{
  uint8 v1; // rax
  char *v2; // rax
  char *v3;
  
  if ((a0) && (*a0)) {
    v1 = strlen(a0);
    v3 = &a0[v1 - 1];
    if (a0 < &a0[v1 - 1]) {
      do {
        if (*v3 != '/')
          return a0;
        v2 = &v3[-1];
        *v3 = '\0';
        v3 = v2;
      } while (a0 != v2);
    }
  }
  return a0;
}


// Function: sub_253e0 @ 0x253e0
void sub_253e0(char *a0) // return-dupe
{
  char *v1;
  char *v2;
  char v3;
  char v4;
  char *v5;
  
  v3 = *a0;
  v2 = a0;
  v1 = a0;
  v5 = a0;
  if (v3 == '.') {
    do {
      v1 = v2;
      if (v2[1] != '/') break;
      v3 = v2[2];
      v1 = &v2[2];
      if (v3 == '/') {
        v1 = &v2[3];
        v3 = v2[3];
        while (v3 == '/') {
          v2 = &v1[1];
          v1 = &v1[1];
          v3 = *v2;
        }
      }
      v5 = &v5[v3 == '\0'];
      *v5 = v3;
      v2 = v1;
    } while (v3 == '.');
    v3 = *v1;
  }
  *v5 = v3;
  while (v2 = &v5[1], v5 = v2, v4 = v3, v3) {
    while( true ) {
      v1 = &v1[1];
      v3 = *v1;
      if ((v4 == '/') && (v1[v3 == '.'] == '/')) break;
      v2 = &v5[1];
      *v5 = v3;
      v5 = v2;
      v4 = v3;
      if (!v3) goto label_254c0;
    }
    do {
      v1 = &v1[(uint8)(v3 == '.') + 1];
      v3 = *v1;
    } while (v1[v3 == '.'] == '/');
    *v5 = v3;
  }
label_254c0:
  if ((int8)v2 - (int8)a0 <= 2)
    return;
  if (v2[-2] != '.') { // branch-flip
    if (v2[-2] != '/') goto label_254e0;
  }
  else if ((v2[-3] != '/') || (v2 = &v2[-1], (int8)v2 - (int8)a0 <= 2)) goto label_254e0;
  v2 = &v2[-1];
label_254e0:
  v2[-1] = '\0';
  return;
}


// Function: sub_25500 @ 0x25500
void sub_25500(unsigned long *a0,void *a1,uint8 a2,char *a3,uint8 a4)
{
  uint8 v1; // rax
  char *v2;
  char *v3;
  
  v2 = (char *)*a0;
  v1 = strlen(v2);
  if (a2 < v1) {
    if ((!memcmp(v2,a1,a2)) && (v3 = &v2[a2], *v3 == '/')) {
      if (a2 < a4) {
        v2 = (char *)sub_4ece0(v2,(v1 - a2) + 1 + a4);
        *a0 = v2;
        v3 = &v2[a2];
      }
      memmove(&v2[a4],v3,(v1 - a2) + 1);
      memcpy(v2,a3,a4); // tail-call
      return;
    }
  }
  return;
}


// Function: sub_255c0 @ 0x255c0
char * sub_255c0(uint8 a0,unsigned long a1,uint8 a2,int8 a3)
{
  if (a0 <= a2)
    return sub_45370(a0,a3); // tail-call
  return sub_45230(a0,a3); // tail-call
}


// Function: sub_255e0 @ 0x255e0
uint8 sub_255e0(char *a0,unsigned long a1,uint8 a2,uint8 a3) // early-return
{
  int4 *v1; // rax
  uint8 v2;
  uint8 v3; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  if (0 <= (int8)a3) { // branch-flip
    if ((uint4)((int4)a0[*a0 == '-'] - 0x30U) <= 9) {
      v3 = strtoimax(a0,a1,10);
      if ((((int8)v3 < (int8)a2) || (v2 = v3, (int8)a3 < (int8)v3)) && (*v1 = 0x22, v2 = a3, (int8)v3 < (int8)a2))
        v2 = a2;
      return v2;
    }
  }
  else if ((uint4)((int4)*a0 - 0x30U) <= 9) {
    v3 = strtoumax(a0,a1,10);
    if (v3 <= a3)
      return v3;
    *v1 = 0x22;
    return a3;
  }
  *v1 = 0x16;
  return 0;
}


// Function: sub_257a0 @ 0x257a0
int8 sub_257a0(uint8 a0,uint4 a1,int8 a2)
{
  int8 v1;
  int8 v2; // rax
  int8 v3; // rcx
  int4 v4;
  int4 v5; // esi
  
  v1 = a2 + 1;
  if (1000000000 <= a1) { // branch-flip
    if (0 <= (int8)a0) {
      v1 = sub_45370(a0,v1);
      goto label_258e0;
    }
    a1 = 0;
label_257de:
    v1 = sub_45370(-a0,v1);
    *(char *)(v1 + -1) = 0x2d;
    v1 -= 1;
  }
  else {
    if ((a1) && ((int8)a0 <= -1)) {
      a0 += 1;
      a1 = 1000000000 - a1;
      goto label_257de;
    }
    if ((int8)a0 < 0) goto label_257de;
    v1 = sub_45370(a0,v1);
  }
  if (a1) {
    v2 = (int8)(int4)a1;
    *(char *)(a2 + 0x15) = 0x2e;
    if ((int4)a1 % 10) { // branch-flip
      *(char *)(a2 + 0x1f) = 0;
      v3 = 8;
      *(char *)(a2 + 0x1e) = (char)((int4)a1 % 10) + '0';
    }
    else {
      v4 = 9;
      do {
        v5 = v4;
        v4 = v5 + -1;
        a1 = (int4)(v2 * 0x66666667 >> 0x22) - ((int4)a1 >> 0x1f);
        v2 = (int8)(int4)a1;
      } while (!((int4)a1 % 10));
      v5 -= 2;
      v3 = (int8)v5;
      *(char *)(a2 + 0x16 + (int8)v4) = 0;
      *(char *)(a2 + 0x16 + v3) = (char)((int4)a1 % 10) + '0';
      if (!v5)
        return v1;
    }
    do {
      a1 = (int4)(v2 * 0x66666667 >> 0x22) - ((int4)a1 >> 0x1f);
      v2 = (int8)(int4)a1;
      *(char *)(a2 + 0x15 + v3) = (char)a1 + (char)((int4)a1 / 10) * '\xf6' + '0';
      v3 -= 1;
    } while ((int4)v3);
    return v1;
  }
label_258e0:
  *(char *)(a2 + 0x15) = 0;
  return v1;
}


// Function: sub_25930 @ 0x25930
undefined16 sub_25930(char *a0,unsigned long *a1,bool a2)
{
  char v1;
  char *v10;
  uint4 v11; // edx
  uint4 v12; // r9d
  int4 v13;
  int4 v2; // eax
  int4 v3;
  int4 *v4; // rax
  char v5 [16];
  int8 v6;
  char v7 [16];
  char v8 [16];
  int8 v9; // rdx
  
  v1 = *a0;
  v4 = __errno_location();
  if (10 <= (uint4)((int4)a0[v1 == '-'] - 0x30U)) {
    *v4 = 0x16;
    *a1 = a0;
    v5._8_8_ = 0xffffffffffffffff;
    v5._0_8_ = 0x8000000000000000;
    return v5._0_16_;
  }
  *v4 = 0;
  if (v1 != '-') { // branch-flip
    v6 = strtoumax();
    if (0 <= v6) {
      v10 = (char *)*a1;
      v13 = *v4;
      goto label_25ab4;
    }
    *v4 = 0x22;
    v10 = (char *)*a1;
    if ((!a2) || (*v10 != '.')) {
      *a1 = v10;
      v8._8_8_ = 0xffffffffffffffff;
      v8._0_8_ = 0x8000000000000000;
      return v8._0_16_;
    }
    v13 = 0x22;
    v6 = -0x8000000000000000;
  }
  else {
    v6 = strtoimax(a0,a1,10);
    v10 = (char *)*a1;
    v13 = *v4;
label_25ab4:
    v3 = 0;
    if ((!a2) || (v3 = 0, *v10 != '.')) goto label_25a43;
  }
  v11 = (int4)v10[1];
  v10 = &v10[1];
  if (10 <= (uint4)(v11 - 0x30U)) { // branch-flip
    v3 = 0;
    v12 = 0;
    v2 = 0;
label_25a30:
    do {
      v2 += 1;
      v3 *= 10;
    } while (v2 != 9);
  }
  else {
    v12 = 0;
    v2 = 0;
    v3 = 0;
    do {
      if (9 <= v2) // branch-flip
        v12 |= (char)v11 != '0';
      else {
        v2 += 1;
        v3 = v11 + -0x30 + v3 * 10;
      }
      v11 = (int4)v10[1];
      v10 = &v10[1];
    } while ((uint4)(v11 - 0x30U) <= 9);
    if (v2 <= 8) goto label_25a30;
  }
  if (v1 == '-') {
    v11 = v3 + v12;
    v3 = 0;
    if (v11) {
      if (v6 != -0x8000000000000000) { // branch-flip
        v6 -= 1;
        v3 = 1000000000 - v11;
      }
      else {
        v3 = -1;
      }
    }
  }
label_25a43:
  v9 = (int8)v3;
  *a1 = v10;
  if (v13 == 0x22)
    v9 = -1;
  v7._8_8_ = v9;
  v7._0_8_ = v6;
  return v7._0_16_;
}


// Function: sub_25c10 @ 0x25c10
unsigned long sub_25c10(char *a0,bool a1)
{
  int4 v1;
  char *v2;
  uint8 v3; // rax
  char *v4; // rax
  stat v5; // stack - 0xc8
  
  free(dat_82600);
  v2 = NULL;
  if (a0)
    v2 = (char *)sub_4ee30(a0);
  dat_82600 = v2;
  free(dat_825f8);
  dat_825f8 = NULL;
  if ((((!a1) || (dat_82d70)) || (dat_82d78 = strchr(a0,0x3a), !dat_82d78)) || ((dat_82d78 <= a0 || (v3 = (int8)dat_82d78 - (int8)a0, memchr(a0,0x2f,v3))))) {
    if (fstatat(dat_7f234,a0,&v5,dat_82800)) { // branch-flip
      if (*__errno_location() != 2) {
        sub_33410(a0);
        return 0;
      }
    }
    else if (((v5._24_4_ & 0xf000) != 0x4000) && ((!a1 || ((v5._24_4_ & 0xb000) != 0x2000)))) {
      v2 = (char *)sub_3ffd0(dat_7f234,a0,dat_82b80);
      dat_825f8 = v2;
      if (!v2)
        sub_17020(); // no-return
      if (renameat(dat_7f234,dat_82600,dat_7f234,v2)) {
        v1 = *__errno_location();
        if (dat_82d50) {
          (*dat_82d50)();
          v2 = dat_825f8;
        }
        v2 = (char *)sub_4d2b0(1,v2);
        v4 = (char *)sub_4ce40(dat_82600);
        error(0,v1,dcgettext(NULL,"%s: Cannot rename to %s",5),v4,v2);
        dat_82d58 = 2;
        free(dat_825f8);
        dat_825f8 = NULL;
        return 0;
      }
      if (dat_82a54) {
        v2 = (char *)sub_4d2b0(1,v2);
        v4 = (char *)sub_4d2b0(0,dat_82600);
        __fprintf_chk(dat_81f28,1,dcgettext(NULL,"Renaming %s to %s\n",5),v4,v2);
      }
    }
  }
  return 1;
}


// Function: sub_25ea0 @ 0x25ea0
void sub_25ea0(void)
{
  int4 v1;
  char *v2; // rax
  char *v3; // rax
  
  if (dat_825f8) {
    if (renameat(dat_7f234,dat_825f8,dat_7f234,dat_82600)) {
      v1 = *__errno_location();
      if (dat_82d50)
        (*dat_82d50)();
      v2 = (char *)sub_4d2b0(1,dat_82600);
      v3 = (char *)sub_4ce40(dat_825f8);
      error(0,v1,dcgettext(NULL,"%s: Cannot rename to %s",5),v3,v2);
      dat_82d58 = 2;
    }
    if (dat_82a54) {
      v2 = (char *)sub_4d2b0(1,dat_82600);
      v3 = (char *)sub_4d2b0(0,dat_825f8);
      __fprintf_chk(dat_81f28,1,dcgettext(NULL,"Renaming %s back to %s\n",5),v3,v2);
    }
    free(dat_825f8);
    dat_825f8 = NULL;
    return;
  }
  return;
}


// Function: sub_25fe0 @ 0x25fe0
int8 sub_25fe0(uint8 a0,void *a1,uint8 a2)
{
  int8 v1; // rax
  uint8 v2; // rax
  
  v1 = sub_4d570(a0,a1,a2);
  if (v1 == -1) {
    if (*__errno_location() == 0xb) {
      v2 = sub_4f850(a0 & 0xffffffff,3);
      if ((0 <= (int4)(uint4)v2) && (v2 & 0x800)) {
        if (sub_4f850(a0 & 0xffffffff,4,(uint4)v2 & 0xfffff7ff) != -1)
          return sub_4d570(a0 & 0xffffffff,a1,a2); // tail-call
      }
    }
  }
  return v1;
}


// Function: sub_26070 @ 0x26070
uint8 sub_26070(uint8 a0,void *a1,uint8 a2)
{
  uint8 v1; // rax
  uint8 v2; // rax
  
  v1 = sub_431a0(a0,a1,a2);
  if (v1 < a2) {
    if (*__errno_location() == 0xb) {
      v2 = sub_4f850(a0 & 0xffffffff,3);
      if ((0 <= (int4)(uint4)v2) && (v2 & 0x800)) {
        if (sub_4f850(a0 & 0xffffffff,4,(uint4)v2 & 0xfffff7ff) != -1)
          v1 += sub_431a0(a0 & 0xffffffff,(void *)((int8)a1 + v1),a2 - v1);
      }
    }
  }
  return v1;
}


// Function: sub_26100 @ 0x26100
void sub_26100(void)
{
  sub_412d0();
  return;
}


// Function: sub_26160 @ 0x26160
int4 sub_26160(void) // early-return
{
  if (dat_825e8)
    return (int4)dat_825e8 + -1;
  return 0;
}


// Function: sub_26180 @ 0x26180
uint8 sub_26180(char *a0)
{
  char *v1;
  int8 *v2;
  char v3;
  uint8 v4;
  
  v4 = dat_825e8;
  if (dat_825e8 != dat_825e0) goto label_2619b;
  if (dat_825e0) { // branch-flip
    if (dat_825f0) { // branch-flip
      if (0x38e38e38e38e38d < dat_825e0)
        sub_17020(); // no-return, return-dupe
label_26244:
      dat_825e0 = dat_825e0 + 1 + (dat_825e0 >> 1);
    }
    else if ((SUB168(ZEXT816(0x18) * ZEXT816(dat_825e0),0) < 0) || (SUB168(ZEXT816(0x18) * ZEXT816(dat_825e0),8))) {
      sub_17020();
    }
  }
  else {
    dat_825e0 = 2;
    if (dat_825f0) goto label_26244;
  }
  dat_825f0 = (unsigned long *)sub_4ece0(dat_825f0,dat_825e0 * 0x18);
  v4 = dat_825e8;
  if (!dat_825e8) {
    dat_825f0[1] = 0;
    *dat_825f0 = ".";
    v4 = 1;
    *(unsigned int *)&dat_825f0[2] = 0xffffff9c;
    dat_825e8 = 1;
  }
label_2619b:
  if (*a0 == '.') {
    do {
      while( true ) {
        if (a0[1] != '/') {
          v3 = a0[1];
          goto label_262b9;
        }
        v1 = &a0[2];
        a0 = &a0[2];
        if (*v1 == '/') break;
        if (*v1 != '.') goto label_26207;
      }
      do {
        v1 = &a0[1];
        a0 = &a0[1];
      } while (*v1 == '/');
    } while (*v1 == '.');
label_26207:
    v3 = *a0;
label_262b9:
    if (!v3)
      return (uint8)(uint4)((int4)v4 - 1);
  }
  v2 = &dat_825f0[v4 * 3];
  *v2 = (int8)a0;
  v2[1] = 0;
  *(unsigned int *)&v2[2] = 0;
  dat_825e8 = v4 + 1;
  return v4;
}


// Function: sub_26320 @ 0x26320
unsigned long sub_26320(void)
{
  return *(unsigned long *)(dat_825f0 + (int8)dat_82580 * 0x18);
}


// Function: sub_26370 @ 0x26370
void sub_26370(char *a0)
{
  if (!dat_82b52) {
    sub_32ea0(a0); // tail-call
    return;
  }
  if (!(dat_81b82 & 0x80))
    return;
  sub_32ee0(a0); // tail-call
  return;
}


// Function: sub_263a0 @ 0x263a0
void sub_263a0(char *a0,uint8 a1,uint8 a2)
{
  if (!dat_82b52) {
    sub_32f20(a0,a1,a2); // tail-call
    return;
  }
  if (!(dat_81b82 & 0x80))
    return;
  sub_32fe0(a0,a1,a2); // tail-call
  return;
}


// Function: sub_263d0 @ 0x263d0
void sub_263d0(char *a0)
{
  if (!dat_82b52) {
    sub_33140(a0); // tail-call
    return;
  }
  if (!(dat_81b82 & 0x80))
    return;
  sub_33160(a0); // tail-call
  return;
}


// Function: sub_26400 @ 0x26400
void sub_26400(char *a0)
{
  if (!dat_82b52) {
    sub_331a0(a0); // tail-call
    return;
  }
  if (!(dat_81b82 & 0x80))
    return;
  sub_331c0(a0); // tail-call
  return;
}


// Function: sub_26430 @ 0x26430
void sub_26430(char *a0,uint8 a1)
{
  if (!dat_82b52) {
    sub_33200(a0,a1); // tail-call
    return;
  }
  if (!(dat_81b82 & 0x80))
    return;
  sub_332d0(a0,a1); // tail-call
  return;
}


// Function: sub_26460 @ 0x26460
void sub_26460(char *a0)
{
  if (!dat_82b52) {
    sub_33410(a0); // tail-call
    return;
  }
  if (!(dat_81b82 & 0x80))
    return;
  sub_33430(a0); // tail-call
  return;
}


// Function: sub_26490 @ 0x26490
void sub_26490(char *a0,bool a1,code *a2)
{
  char *v1; // rax
  
  if (!a1) {
    if (*__errno_location() == 2) {
      if (dat_81b80 & 0x40) {
        if (dat_82d50)
          (*dat_82d50)();
        v1 = (char *)sub_4ce40(a0);
        error(0,0,dcgettext(NULL,"%s: File removed before we read it",5),v1);
      }
      sub_2fd00(1); // tail-call
      return;
    }
  }
  (*a2)(a0); // jump-as-call
  return;
}


// Function: sub_26520 @ 0x26520
void sub_26520(void)
{
  if (fork() != -1)
    return;
  sub_32b70("fork",dcgettext(NULL,"child process",5)); // no-return
}


// Function: sub_26560 @ 0x26560
void sub_26560(int4 *a0)
{
  if (0 <= pipe(a0))
    return;
  sub_32b70("pipe",dcgettext(NULL,"interprocess channel",5)); // no-return
}


// Function: sub_265a0 @ 0x265a0
int8 sub_265a0(int8 *a0,uint8 a1)
{
  uint8 v1;
  int8 v2; // rax
  uint8 v3; // rbx
  
  v3 = (uint8)getpagesize();
  if (!CARRY8(a1,v3)) {
    v2 = sub_4ecc0();
    v1 = v2 + -1 + v3;
    *a0 = v2;
    return v1 - v1 % v3;
  }
  sub_17020(a1 + v3); // no-return
}


// Function: sub_265f0 @ 0x265f0
unsigned long * sub_265f0(char *a0)
{
  char v1;
  unsigned long *v2; // rax
  char *v3; // rax
  uint8 v4;
  
  v2 = (unsigned long *)sub_4ecc0(0x18);
  v4 = strlen(a0) + 2;
  v2[1] = v4;
  v3 = (char *)sub_4ecc0(v4);
  *v2 = v3;
  strcpy(v3,a0);
  v4 = strlen(v3);
  v1 = v3[v4 - 1];
  v2[2] = v4;
  if (v1 != '/') {
    v3[v4] = '/';
    v2[2] = v4 + 1;
  }
  return v2;
}


// Function: sub_26660 @ 0x26660
void sub_26660(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
  return;
}


// Function: sub_26680 @ 0x26680
void * sub_26680(unsigned long *a0,char *a1)
{
  uint8 v1;
  int8 v2;
  void *v3; // rax
  void *v4;
  
  v3 = (uint8)strlen(a1);
  v1 = (uint8)v3;
  if ((uint8)a0[1] <= (uint8)(a0[2] + 1 + (int8)v3)) { // branch-flip
    v3 = (void *)*a0;
    do {
      v3 = (void *)sub_4ed20(v3,&a0[1]);
      *a0 = v3;
      v2 = a0[2];
      v4 = v3;
    } while ((uint8)a0[1] <= a0[2] + 1 + v1);
  }
  else {
    v4 = (void *)*a0;
    v2 = a0[2];
  }
  v3 = (void *)v2;
  strcpy((char *)((int8)v4 + (int8)v3),a1);
  v3 = (void *)*a0;
  return v3;
}


// Function: sub_26700 @ 0x26700
char * sub_26700(int4 a0,char *a1)
{
  int8 v1;
  int8 v10; // rax
  int8 v11;
  int4 v12; // edx
  int8 v13;
  unsigned long *v2;
  int4 v3;
  int4 v4;
  uint8 v5; // rax
  char *v6;
  char *v7;
  int8 *v8; // rax
  int8 *v9;
  
  v3 = dat_82580;
  if (*a1 != '/') { // branch-flip
    if (dat_825f0) { // branch-flip
      v1 = (int8)a0 * 0x18;
      v7 = *(char **)(dat_825f0 + 8 + v1);
      if (!v7) {
        v9 = (int8 *)(dat_825f0 + -0x10 + v1);
        v4 = a0;
        if (0 <= a0) { // branch-flip
          do {
            v12 = v4;
            if (!v12) goto label_2683c;
            v8 = &v9[-3];
            v13 = *v9;
            v9 = v8;
            v4 = v12 + -1;
          } while (!v13);
        }
        else {
          v12 = a0 + 1;
        }
        if (v12 <= a0) {
label_2683c:
          v4 = dat_82580;
          do {
            while( true ) {
              if (v12 != v4)
                sub_24f80(v12);
              v13 = dat_825f0;
              if (v12) break;
              v11 = sub_4ee70();
              *(int8 *)(v13 + 8) = v11;
              if (!v11) {
                sub_32b70("getcwd","."); // return-dupe, no-return
              }
              v12 = 1;
              v4 = dat_82580;
              if (a0 <= 0) goto label_2692a;
            }
            v11 = (int8)v12;
            v12 += 1;
            v13 = v11 * 0x18;
            v2 = (unsigned long *)(dat_825f0 + v13);
            v6 = (char *)*v2;
            if (*v6 != '/') { // branch-flip
              v9 = (int8 *)sub_265f0(*(char **)(dat_825f0 + -0x10 + v13));
              v6 = *(char **)(dat_825f0 + v11 * 0x18);
              if (*(char *)(*v9 + -1 + v9[2]) != '/') {
                sub_26680(v9,(char *)0x7f230);
                v9[2] = v9[2] + 1;
              }
              sub_26680(v9,v6);
              v5 = strlen(v6);
              v11 = *v9;
              v10 = v5 + v9[2];
              v9[2] = v10;
              v13 += dat_825f0;
              if (*(char *)(v11 + -1 + v10) == '/')
                *(char *)(v11 + v10) = 0;
              free(v9);
              *(int8 *)(v13 + 8) = v11;
            }
            else {
              v2[1] = sub_4ee30(v6);
            }
            v4 = dat_82580;
          } while (v12 <= a0);
label_2692a:
          if (v3 != dat_82580)
            sub_24f80(v3);
          v7 = *(char **)(dat_825f0 + 8 + v1);
        }
      }
    }
    else {
      v7 = dat_82588;
      if (!dat_82588) {
        v7 = (char *)sub_4ee70();
        dat_82588 = v7;
        if (!v7)
          sub_32b70("getcwd",".");
      }
    }
    v5 = strlen(v7);
    v6 = (char *)sub_4ecc0(v5 + 2 + strlen(a1));
    strcpy(v6,v7);
    v6[v5] = '/';
    strcpy(&v6[v5 + 1],a1);
  }
  else {
    v6 = (char *)sub_4ee30(a1);
  }
  sub_253e0(v6);
  return v6;
}


// Function: sub_26970 @ 0x26970
int8 sub_26970(char *a0,int4 a1) // return-dupe x2
{
  int4 v1; // eax
  DIR *v2; // rax
  int8 v3; // rax
  
  v1 = __openat_2(dat_7f234,a0,dat_82808 | 0x10000);
  if (v1 <= -1) {
    if ((!a1) && (*__errno_location() == 2))
      return 0;
    sub_32ea0(a0);
    return 0;
  }
  v2 = fdopendir(v1);
  if (!v2) {
    sub_331a0(a0);
    if (!close(v1))
      return 0;
    sub_331a0(a0);
    return 0;
  }
  v3 = sub_4d730(v2,dat_827e4);
  if (!v3)
    sub_331a0(a0);
  if (closedir(v2)) {
    sub_331a0(a0);
    return v3;
  }
  return v3;
}


// Function: sub_26a40 @ 0x26a40
uint4 sub_26a40(char *a0,int4 a1) // return-dupe
{
  char *v1;
  uint4 v2;
  int4 v3; // eax
  uint4 *v4; // rax
  char *v5; // rax
  uint8 v6;
  void *v7; // rax
  
  if (!unlinkat(dat_7f234,a0,0))
    return 1;
  v4 = (uint4 *)__errno_location();
  if ((*v4 != 1) && (*v4 != 0x15))
    return 0;
  if (!sub_25ba0(a0))
    return 1;
  if (*v4 <= 0x27) {
    if (!(0x8000020001U >> ((uint8)*v4 & 0x3f) & 1))
      return 0;
    if (a1 == 1) {
      v5 = (char *)sub_26970(a0,0);
      if (!v5)
        return 0;
      v6 = strlen(v5);
      v1 = v5;
      while( true ) {
        if (!v6) {
          free(v5);
          return (uint4)(sub_25ba0(a0) == 0);
        }
        v7 = (void *)sub_29320(a0,v1);
        v3 = sub_26a40(v7,1);
        v2 = *v4;
        free(v7);
        if (!v3) break;
        v1 = &v1[v6 + 1];
        v6 = strlen(v1);
      }
      free(v5);
      *v4 = v2;
      return 0;
    }
    return -(uint4)(a1 == 2);
  }
  return 0;
}


// Function: sub_26b90 @ 0x26b90
unsigned long ** sub_26b90(unsigned long *a0,int4 a1,code *a2)
{
  unsigned long v1;
  int4 v10; // r8d
  unsigned long **v11;
  unsigned long **v2;
  unsigned long **v3;
  unsigned long *v4;
  int4 v5;
  unsigned long **v6; // rax
  unsigned long **v7;
  unsigned long *v8; // stack - 0x38
  unsigned long **v9;
  
  if (a1 != 1) {
    if (a1 != 2) {
      v10 = (a1 + 1) / 2;
      v4 = a0;
      v5 = v10;
      while (v5 = v5 + -1, v5) {
        v4 = (unsigned long *)*v4;
      }
      v1 = *v4;
      *v4 = 0;
      v6 = (unsigned long **)sub_26b90(a0,v10,a2);
      v9 = v6;
      v6 = (unsigned long **)sub_26b90(v1,a1 / 2,a2);
      v7 = v6;
      if ((v9) && (v6)) {
        v11 = &v8;
        do {
          v5 = (*a2)(v9,v7);
          if (0 <= v5) { // branch-flip
            v2 = (unsigned long **)*v7;
            *v11 = v7;
            v11 = v7;
          }
          else {
            v3 = (unsigned long **)*v9;
            *v11 = v9;
            v2 = v7;
            v11 = v9;
            v9 = v3;
          }
          v7 = v2;
          if (!v9) break;
        } while (v7);
      }
      else {
        v11 = &v8;
      }
      v6 = v11;
      if (!v9)
        v9 = v7;
      *v6 = v9;
      v6 = (unsigned long **)v8;
      return v6;
    }
    v5 = (*a2)(a0,*a0);
    if (0 < v5) {
      v6 = (unsigned long *)*a0;
      *v6 = a0;
      *a0 = 0;
      return v6;
    }
  }
  v6 = (unsigned long **)a0;
  return v6;
}


// Function: sub_26d00 @ 0x26d00
void sub_26d00(void)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_4ecc0(0x38);
  if (!dat_826b0) {
    *v2 = 0;
    v2[1] = 0;
    *(unsigned int *)&v2[2] = 3;
    dat_826b0 = v2;
    v2 = (unsigned long *)sub_4ecc0(0x38);
  }
  v1 = (unsigned long *)dat_826b0[1];
  v2[1] = v1;
  if (v1)
    *v1 = v2;
  *v2 = dat_826b0;
  dat_826b0[1] = v2;
  return;
}


// Function: sub_26d70 @ 0x26d70
unsigned int sub_26d70(int8 a0,int8 a1)
{
  uint4 v1; // eax
  
  v1 = strcmp(*(char **)(a0 + 0x58),*(char **)(a1 + 0x58));
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_26d90 @ 0x26d90
void sub_26d90(int8 a0,int8 a1)
{
  strcmp(*(char **)(a0 + 0x10),*(char **)(a1 + 0x10)); // tail-call
  return;
}


// Function: sub_26db0 @ 0x26db0
unsigned int sub_26db0(char *a0,int8 a1) // return-dupe
{
  int8 *v1;
  int8 *v10; // stack - 0x110
  int8 v11; // stack - 0x108
  int8 v2; // rax
  char *v3; // rax
  char v4;
  unsigned int v5 [2]; // stack - 0x138
  int4 v6; // stack - 0x118
  unsigned long v7; // stack - 0x130
  unsigned long v8; // stack - 0x128
  unsigned int v9; // stack - 0x114
  
  v4 = *a0;
  if (!v4)
    return 1;
  v1 = __ctype_b_loc();
  while (*(uint1 *)(*v1 + 1 + (int8)v4 * 2) & 0x20) {
    v4 = a0[1];
    a0 = &a0[1];
    if (!v4)
      return 1;
  }
  if (v4 != '-')
    return 1;
  v11 = 1;
  if (sub_375a0(a0,&v6,0x2000e46)) {
    if (dat_82d50)
      (*dat_82d50)();
    v3 = (char *)sub_358f0(&v6);
    error(0,0,dcgettext(NULL,"cannot split string \'%s\': %s",5),a0,v3);
    sub_16fd0(); // no-return
  }
  *v10 = dat_82dd0;
  v7 = *(unsigned long *)(a1 + 0x18);
  v5[0] = 2;
  v8 = *(unsigned long *)(a1 + 0x20);
  sub_2fac0((int4)v11 + v6,v10,v5);
  v2 = CONCAT44(v9,v6) + v11;
  if (v2) {
    v1 = &v10[v2];
    do {
      *v10 = 0;
      v10 = &v10[1];
    } while (v1 != v10);
  }
  sub_35800(&v6);
  return 0;
}


// Function: sub_26f30 @ 0x26f30
unsigned int sub_26f30(char *a0)
{
  if (!dat_826fc) {
    if (sub_40850(a0,0)) {
      dat_82634 = 1;
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Pattern matching characters used in file names",5));
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning",5));
    }
    return dat_82634;
  }
  return dat_82634;
}


// Function: sub_26fd0 @ 0x26fd0
uint8 sub_26fd0(int8 a0,uint8 a1)
{
  uint1 *v1;
  
  v1 = *(uint1 **)(a0 + 0x58);
  return sub_43b80(v1,a1); // tail-call
}


// Function: sub_270f0 @ 0x270f0
void sub_270f0(void) // return-dupe
{
  int4 v1;
  unsigned long *v2;
  int8 *v3;
  
  v3 = dat_826b0;
  dat_826b0 = (int8)*dat_826b0;
  if (dat_826b0)
    *(unsigned long *)((int8)dat_826b0 + 8) = 0;
  v1 = (int4)v3[2];
  if ((v1 != 4) && (v1 != 1)) {
    if (v1 != 3) {
      while (v2 = dat_82628, dat_82628) {
        dat_82628 = (unsigned long *)dat_82628[1];
        free(v2);
      }
      free(v3); // tail-call
      return;
    }
    free(v3); // tail-call
    return;
  }
  if ((dat_82ba8 != 3) && (dat_82ba8 != 8))
    return;
  *v3 = 0;
  v3[1] = (int8)dat_82628;
  if (!dat_82628) {
    dat_82628 = v3;
    return;
  }
  *dat_82628 = (int8)v3;
  dat_82628 = v3;
  return;
}


// Function: sub_271a0 @ 0x271a0
unsigned long sub_271a0(int4 a0,unsigned long a1,int8 a2) // return-dupe
{
  int4 *v1;
  uint8 v2;
  char *v3;
  int8 v4; // rax
  int8 *v5; // rax
  
  if (a0 == 0x100) {
    v4 = sub_26d00();
    *(unsigned int *)(v4 + 0x10) = 0;
    *(unsigned long *)(v4 + 0x18) = a1;
    if (!dat_82630) {
      dat_82630 = 1;
      return 0;
    }
    if (dat_82630 != 1)
      return 0;
    dat_82630 = 2;
    return 0;
  }
  if (0x101 <= a0) { // branch-flip
    if (a0 == 0x1000005) {
      v1 = (int4 *)**(unsigned long **)(a2 + 0x28);
      if (*v1 != 2)
        return 7;
      v2 = *(uint8 *)&v1[4];
      v3 = *(char **)&v1[2];
      error(0,0,dcgettext(NULL,"%s:%lu: unrecognized option",5),v3,v2);
      sub_2fd00(2);
      return 7;
    }
  }
  else {
    if (a0 == 0x43) {
      v4 = sub_26d00();
      *(unsigned long *)(v4 + 0x18) = a1;
      *(unsigned int *)(v4 + 0x10) = 1;
      return 0;
    }
    if (a0 == 0x54) {
      v4 = sub_26d00();
      *(unsigned int *)(v4 + 0x10) = 2;
      *(unsigned long *)(v4 + 0x20) = 0;
      *(unsigned long *)(v4 + 0x30) = 0;
      *(unsigned long *)(v4 + 0x18) = a1;
      dat_82630 = 2;
      return 0;
    }
  }
  for (v5 = (int8 *)0x7f280; ((*v5 || ((int4)v5[1])) || (v5[4])); v5 = &v5[6]) {
    if (a0 == (int4)v5[1]) {
      v4 = sub_26d00();
      *(unsigned long *)(v4 + 0x20) = a1;
      *(unsigned int *)(v4 + 0x10) = 4;
      *(int4 *)(v4 + 0x18) = a0;
      return 0;
    }
  }
  return 7;
}


// Function: sub_27340 @ 0x27340
int8 * sub_27340(int4 a0) // return-dupe
{
  uint1 *v1;
  char *v10;
  unsigned long v11; // rax
  unsigned long *v12;
  FILE *v13;
  uint1 *v14;
  stat v15; // stack - 0xd8
  uint8 v16; // rax
  uint1 *v17;
  char *v18;
  int8 *v19;
  uint4 v2;
  int8 *v20;
  uint4 v21;
  bool v22;
  bool v23;
  uint4 v3;
  unsigned int v4; // eax
  int4 v5;
  char *v6; // rax
  unsigned long v7;
  int8 *v8;
  int8 v9;
  
  v19 = dat_826b0;
label_27376:
  if (v19) goto label_2737f;
  goto label_27877;
label_2737f:
  switch(*(unsigned int *)&v19[2]) {
    case 0:
      goto label_273e0;
    case 1:
      if (!a0) goto label_273e0;
      v18 = (char *)sub_4ee30((char *)v19[3]);
      sub_26300(sub_26180(v18));
      sub_270f0();
      v19 = dat_826b0;
      goto label_27376;
    case 2:
      goto label_274f0;
    case 3:
label_273c8:
      sub_270f0();
      v19 = dat_826b0;
      goto label_27376;
    case 4:
      v18 = (char *)v19[4];
      v21 = *(uint4 *)&v19[3];
      switch(v21) {
        case 0x58:
          v5 = sub_41210(sub_40c90,dat_82b60,v18,(uint4)(dat_826fc != 1) << 0x1c | dat_826f8 | dat_82ad4,10);
          if (v5) {
            v5 = *__errno_location();
            if (dat_82d50)
              (*dat_82d50)();
            error(0,v5,"%s",sub_4ce40(v18));
            sub_16fd0(); // no-return
          }
          break;
        default:
          if (dat_82d50)
            (*dat_82d50)();
          error(0,0,"unhandled positional option %d",(uint8)v21);
          sub_16fd0(); // no-return
        case 0x101:
          v9 = 0x657df;
          v19 = (int8 *)0x7d708;
          do {
            v8 = &v19[1];
            sub_40c90(dat_82b60,v9,0x10000000);
            v9 = *v19;
            v19 = v8;
          } while (v9);
          break;
        case 0x102:
          sub_12e10("CACHEDIR.TAG",1,sub_12f10);
          break;
        case 0x103:
          sub_12e10("CACHEDIR.TAG",2,sub_12f10);
          break;
        case 0x104:
          sub_12e10("CACHEDIR.TAG",3,sub_12f10);
          break;
        case 0x105:
          sub_40c90(dat_82b60,v18,dat_826f8 | dat_82ad4 | (uint4)(dat_826fc != 1) << 0x1c);
          break;
        case 0x106:
          sub_173d0(v18,2);
          break;
        case 0x107:
          sub_173d0(v18,1);
          break;
        case 0x108:
          sub_12e10(v18,1,0);
          break;
        case 0x109:
          sub_12e10(v18,2,0);
          break;
        case 0x10a:
          sub_12e10(v18,3,0);
          break;
        case 0x10b:
          v9 = 0x657e3;
          v19 = (int8 *)0x7d728;
          do {
            v8 = &v19[1];
            sub_40c90(dat_82b60,v9,0);
            v9 = *v19;
            v19 = v8;
          } while (v9);
          break;
        case 0x10c:
          sub_17810();
          break;
        case 0x10d:
          dat_826f8 |= 0x10;
          break;
        case 0x10e:
          dat_826f8 &= 0xffffffef;
          break;
        case 0x10f:
          dat_826f8 |= 0x40000000;
          break;
        case 0x110:
          dat_826f8 &= 0xbfffffff;
          dat_7f238 = 0;
          break;
        case 0x111:
          dat_82ad4 = 8;
          break;
        case 0x112:
          dat_82ad4 = 0;
          break;
        case 0x113:
          dat_827e8 = '\x01';
          break;
        case 0x114:
          dat_827e8 = '\0';
          break;
        case 0x115:
          dat_82700 = 0;
          break;
        case 0x116:
          dat_826f8 |= 1;
          break;
        case 0x117:
          dat_826fc = 1;
          break;
        case 0x118:
          dat_7f23c = '\0';
          dat_82700 = 1;
          break;
        case 0x119:
          dat_7f23c = '\n';
          dat_82700 = 0;
          break;
        case 0x11a:
          dat_82700 = 1;
          break;
        case 0x11b:
          dat_826f8 &= 0xfffffffe;
          break;
        case 0x11c:
          dat_826fc = 2;
        
      }
      goto label_273c8;
    default:
      goto label_27376;
    
  }
label_274f0:
  v13 = (FILE *)v19[6];
  if (v13) {
    v21 = *(uint4 *)&v19[5];
    goto label_27503;
  }
  v18 = (char *)v19[3];
  if (strcmp(v18,"-")) {
    if (stat(v18,&v15))
      sub_333f0(v18); // no-return
    v8 = v19;
    do {
      if (((int4)v8[2] == 2) && (v8[6])) {
        v10 = (char *)v8[3];
        v12 = dat_82698;
        goto label_27a2e;
      }
      v8 = (int8 *)*v8;
    } while (v8);
    v10 = dcgettext(NULL,"command line",5);
    v12 = dat_82698;
label_27a2e:
    do {
      if (!v12) goto label_27ae0;
      if ((v12[1] == v15._8_8_) && (v12[2] == v15._0_8_)) {
        v4 = sub_4c780(0,0x3a,1);
        if (dat_82d50)
          (*dat_82d50)();
        v7 = v12[3];
        v11 = sub_4ca40(0,v18);
        error(0,0,dcgettext(NULL,"%s: file list requested from %s already read from %s",5),v11,v10,v7);
        dat_82d58 = 2;
        sub_4c780(0,0x3a,v4);
        sub_270f0();
        v19 = dat_826b0;
        goto label_27376;
      }
      v12 = (unsigned long *)*v12;
    } while( true );
  }
  sub_2df30(0x657e7);
  v19[6] = (int8)stdin;
  v13 = stdin;
  goto label_277a0;
label_27ae0:
  v12 = (unsigned long *)sub_4ecc0(0x20);
  v18 = (char *)v19[3];
  v12[3] = v10;
  *v12 = dat_82698;
  v12[1] = v15._8_8_;
  v12[2] = v15._0_8_;
  dat_82698 = v12;
  v13 = fopen(v18,"r");
  v19[6] = (int8)v13;
  if (!v13)
    sub_32ec0((char *)v19[3]); // no-return
label_277a0:
  v21 = (uint4)dat_7f23c;
  *(uint4 *)&v19[5] = v21;
  *(char *)((int8)v19 + 0x2c) = dat_82700;
label_27503:
  v9 = v19[4];
  v17 = *(uint1 **)&v13->field_0x10;
  v14 = *(uint1 **)&v13->field_0x8;
label_27510:
  do {
    v9 += 1;
    v19[4] = v9;
    if (v17 <= v14) { // branch-flip
      v3 = __uflow(v13);
      v23 = v3 != 0xffffffff;
      if ((v21 != v3) && (v23)) goto label_27537;
      v22 = 1;
      v16 = 0;
    }
    else {
      v1 = &v14[1];
      *(uint1 **)&v13->field_0x8 = v1;
      v3 = (uint4)*v14;
      if (v21 == *v14) {
        v14 = v1;
        goto label_27510;
      }
label_27537:
      v16 = 0;
      do {
        if (v16 != dat_826a0) // branch-flip
          dat_826a8[v16] = (char)v3;
        else {
          dat_826a8 = (char *)sub_4ed20(dat_826a8,(uint8 *)0x826a0);
          dat_826a8[v16] = (char)v3;
        }
        if (!v3) {
          if (dat_81b81 & 2) {
            if (dat_82d50)
              (*dat_82d50)();
            v18 = (char *)v19[3];
            error(0,0,"%s: file name read contains nul character",sub_4ce40(v18));
          }
          *(unsigned int *)&v19[5] = 0;
          v18 = dat_826a8;
          goto label_27601;
        }
        v16 += 1;
        v17 = *(uint1 **)&v13->field_0x8;
        if (*(uint1 **)&v13->field_0x10 <= v17) { // branch-flip
          v3 = __uflow(v13);
          v22 = v3 != 0xffffffff;
          v23 = v22;
          v2 = v3;
        }
        else {
          *(uint1 **)&v13->field_0x8 = &v17[1];
          v3 = (uint4)*v17;
          v22 = 1;
          v23 = 1;
          v2 = (uint4)*v17;
        }
      } while ((v21 != v2) && (v23 = v22, v22));
      v22 = v16 == 0;
      v23 = (bool)(v23 & v22);
    }
    if (!v23) goto label_276d8;
    v13 = (FILE *)v19[6];
    v21 = *(uint4 *)&v19[5];
    v9 = v19[4];
    v17 = *(uint1 **)&v13->field_0x10;
    v14 = *(uint1 **)&v13->field_0x8;
  } while( true );
label_276d8:
  v18 = dat_826a8;
  if (dat_826a0 == v16) {
    v18 = (char *)sub_4ed20(dat_826a8,(uint8 *)0x826a0);
    dat_826a8 = v18;
  }
  v18[v16] = '\0';
  v16 = strlen(v18);
  v10 = &v18[v16 - 1];
  if (v18 < &v18[v16 - 1]) {
    do {
      if (*v10 != '/') break;
      v6 = &v10[-1];
      *v10 = '\0';
      v10 = v6;
    } while (v6 != v18);
  }
  if ((v3 == 0xffffffff) && (v22)) {
    v18 = (char *)v19[3];
    if (strcmp(v18,"-"))
      fclose((FILE *)v19[6]);
    v19[6] = 0;
    sub_270f0();
    v19 = dat_826b0;
    goto label_27376;
  }
label_27601:
  if (*(char *)((int8)v19 + 0x2c)) {
    dat_82670 = 0;
    dat_82678 = v18;
    return (int8 *)0x82660;
  }
  if (dat_827e8) {
    sub_251b0(v18);
    v18 = dat_826a8;
  }
  if (sub_26db0(v18,v19)) {
    dat_82670 = 0;
    dat_82678 = dat_826a8;
    return (int8 *)0x82660;
  }
  v19 = dat_826b0;
  if (dat_826b0) {
    v8 = (int8 *)dat_826b0[1];
    if ((int8 *)dat_826b0[1]) {
      do {
        dat_826b0 = v8;
        v8 = (int8 *)dat_826b0[1];
      } while ((int8 *)dat_826b0[1]);
      v19 = dat_826b0;
    }
    goto label_27376;
  }
label_27877:
  if (!dat_82628)
    return v19;
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly.",5));
  v8 = dat_82628;
  do {
    v20 = v8;
    v8 = (int8 *)v20[1];
  } while ((int8 *)v20[1]);
  do {
    dat_82d58 = 2;
    if ((int4)v20[2] != 1) { // branch-flip
      if ((int4)v20[2] == 4) {
        v18 = (char *)v20[4];
        if (!v18) {
          if (dat_82d50)
            (*dat_82d50)();
          v8 = (int8 *)0x7f280;
          while( true ) {
            v9 = *v8;
            if ((((!v9) && (!(int4)v8[1])) && (!v8[4])) || ((int4)v20[3] == (int4)v8[1])) break;
            v8 = &v8[6];
          }
          v18 = "--%s has no effect";
          goto label_279ea;
        }
        if (dat_82d50) {
          (*dat_82d50)();
          v18 = (char *)v20[4];
        }
        v7 = sub_4d2d0(v18);
        v8 = (int8 *)0x7f280;
        while( true ) {
          v9 = *v8;
          if ((((!v9) && (!(int4)v8[1])) && (!v8[4])) || ((int4)v20[3] == (int4)v8[1])) break;
          v8 = &v8[6];
        }
        error(0,0,dcgettext(NULL,"--%s %s has no effect",5),v9,v7);
      }
    }
    else {
      if (dat_82d50)
        (*dat_82d50)();
      v9 = sub_4d2d0((char *)v20[3]);
      v18 = "-C %s has no effect";
label_279ea:
      error(0,0,dcgettext(NULL,v18,5),v9);
    }
    dat_82d58 = 2;
    v20 = (int8 *)*v20;
    if (!v20) {
      while (v8 = dat_82628, dat_82628) {
        dat_82628 = (int8 *)dat_82628[1];
        free(v8);
      }
      return v19;
    }
  } while( true );
label_273e0:
  v18 = (char *)v19[3];
  v16 = strlen(v18);
  while (dat_826a0 <= v16) {
    dat_826a8 = (char *)sub_4ed20(dat_826a8,(uint8 *)0x826a0);
  }
  v10 = strcpy(dat_826a8,v18);
  v16 = strlen(v10);
  v18 = &v10[v16 - 1];
  if (v10 < &v10[v16 - 1]) {
    do {
      if (*v18 != '/') break;
      v6 = &v18[-1];
      *v18 = '\0';
      v18 = v6;
    } while (v6 != v10);
  }
  if (dat_827e8) {
    sub_251b0(v10);
    v10 = dat_826a8;
  }
  dat_82670 = *(unsigned int *)&v19[2];
  dat_82678 = v10;
  sub_270f0();
  return (int8 *)0x82660;
}


// Function: sub_27eb0 @ 0x27eb0
void sub_27eb0(uint4 a0,unsigned long *a1)
{
  passwd *v1; // rax
  
  if ((a0) && (dat_826cc == a0)) {
label_27f26:
    *a1 = sub_4ee30("");
    return;
  }
  if ((!dat_826f0) || (dat_826e4 != a0)) {
    v1 = getpwuid(a0);
    if (!v1) {
      dat_826cc = a0;
      goto label_27f26;
    }
    dat_826e4 = a0;
    sub_25120((unsigned long *)0x826f0,*(char **)v1);
  }
  *a1 = sub_4ee30(dat_826f0);
  return;
}


// Function: sub_27f40 @ 0x27f40
void sub_27f40(uint4 a0,unsigned long *a1)
{
  group *v1; // rax
  
  if ((a0) && (dat_826c8 == a0)) {
label_27fb6:
    *a1 = sub_4ee30("");
    return;
  }
  if ((!dat_826e8) || (dat_826e0 != a0)) {
    v1 = getgrgid(a0);
    if (!v1) {
      dat_826c8 = a0;
      goto label_27fb6;
    }
    dat_826e0 = a0;
    sub_25120((unsigned long *)0x826e8,*(char **)v1);
  }
  *a1 = sub_4ee30(dat_826e8);
  return;
}


// Function: sub_27fd0 @ 0x27fd0
unsigned long sub_27fd0(char *a0,unsigned int *a1)
{
  int4 v1; // eax
  passwd *v2;
  
  if (dat_826d8) {
    v1 = strcmp(a0,dat_826d8);
    if (!v1)
      return CONCAT44(dat_4,v1);
  }
  if ((dat_826f0) && (*a0 == *dat_826f0)) {
    if (!strcmp(a0,dat_826f0)) goto label_2802e;
    v2 = getpwnam(a0);
  }
  else {
    v2 = getpwnam(a0);
  }
  if (!v2) {
    sub_25120((unsigned long *)0x826d8,a0);
    return 0;
  }
  dat_826e4 = *(unsigned int *)&v2->field_0x10;
  sub_25120((unsigned long *)0x826f0,*(char **)v2);
label_2802e:
  *a1 = dat_826e4;
  return 1;
}


// Function: sub_28080 @ 0x28080
unsigned long sub_28080(char *a0,unsigned int *a1)
{
  int4 v1; // eax
  group *v2;
  
  if (dat_826d0) {
    v1 = strcmp(a0,dat_826d0);
    if (!v1)
      return CONCAT44(dat_4,v1);
  }
  if ((dat_826e8) && (*a0 == *dat_826e8)) {
    if (!strcmp(a0,dat_826e8)) goto label_280de;
    v2 = getgrnam(a0);
  }
  else {
    v2 = getgrnam(a0);
  }
  if (!v2) {
    sub_25120((unsigned long *)0x826d0,a0);
    return 0;
  }
  dat_826e0 = *(unsigned int *)&v2->field_0x10;
  sub_25120((unsigned long *)0x826e8,a0);
label_280de:
  *a1 = dat_826e0;
  return 1;
}


// Function: sub_28180 @ 0x28180
void sub_28180(void)
{
  int8 v1;
  
  dat_826a8 = sub_4ecc0(0x66);
  dat_826a0 = 100;
  if ((dat_826b0) && (v1 = *(int8 *)(dat_826b0 + 8), *(int8 *)(dat_826b0 + 8))) {
    do {
      dat_826b0 = v1;
      v1 = *(int8 *)(dat_826b0 + 8);
    } while (*(int8 *)(dat_826b0 + 8));
  }
  return;
}


// Function: sub_281e0 @ 0x281e0
void sub_281e0(void)
{
  free(dat_826a8); // tail-call
  return;
}


// Function: sub_28210 @ 0x28210
unsigned long * sub_28210(char *a0,unsigned int a1,char a2,unsigned long a3)
{
  unsigned long *v1; // rax
  char *v2; // rax
  uint8 v3; // rax
  
  v1 = (unsigned long *)sub_4ed80(0x60);
  if (!a0)
    a0 = "";
  v2 = (char *)sub_4ee30(a0);
  v1[2] = v2;
  v3 = strlen(v2);
  *(unsigned int *)&v1[5] = a1;
  v1[3] = v3;
  v1[1] = dat_826b8;
  v1[8] = a3;
  *(char *)((int8)v1 + 0x24) = a2;
  *v1 = 0;
  v1[6] = 0;
  *(uint4 *)&v1[4] = dat_7f238 | dat_826f8 | dat_82ad4 | (uint4)(dat_826fc == 2) << 0x1c;
  v1[7] = 0;
  if (dat_826b8) // branch-flip
    *dat_826b8 = v1;
  else {
    dat_826c0 = v1;
  }
  dat_826b8 = v1;
  return v1;
}


// Function: sub_282f0 @ 0x282f0
void sub_282f0(void) // return-dupe
{
  void *v1;
  int4 v2;
  int8 v3; // rax
  unsigned long *v4; // rax
  uint8 v5; // rax
  int4 v6;
  char *v7;
  
  v2 = 0;
  v6 = v2;
  if (dat_82aac) { // branch-flip
    while (v3 = sub_27340(0), v1 = dat_82638, v3) {
      if (*(int4 *)(v3 + 0x10) != 1) {
        if (dat_82638) {
          free(*(void **)((int8)dat_82638 + 0x10));
          free(*(void **)((int8)v1 + 0x58));
          free(v1);
        }
        v7 = *(char **)(v3 + 0x18);
        v4 = (unsigned long *)sub_4ed80(0x60);
        if (!v7)
          v7 = "";
        v7 = (char *)sub_4ee30(v7);
        v4[2] = v7;
        v5 = strlen(v7);
        *v4 = 0;
        v4[3] = v5;
        v4[6] = 0;
        *(int4 *)&v4[5] = dat_82640;
        v4[7] = 0;
        v4[8] = 0;
        *(char *)((int8)v4 + 0x24) = 1;
        dat_82638 = v4;
        dat_826b8 = v4;
        *(uint4 *)&v4[4] = dat_7f238 | dat_826f8 | dat_82ad4 | (uint4)(dat_826fc == 2) << 0x1c;
        dat_826c0 = v4;
        return;
      }
      v7 = *(char **)(v3 + 0x18);
      dat_82640 = sub_26180((char *)sub_4ee30(v7));
    }
    v2 = dat_82640;
    if (!dat_82640)
      return;
  }
  else {
    while (v3 = sub_27340(0), v3) {
      v7 = *(char **)(v3 + 0x18);
      if (*(int4 *)(v3 + 0x10) != 1) { // branch-flip
        sub_28210(v7,v2,1,0);
        v6 = v2;
      }
      else {
        v2 = sub_26180((char *)sub_4ee30(v7));
      }
    }
    if (v6 == v2)
      return;
  }
  sub_28210(NULL,v2,0,0); // tail-call
  return;
}


// Function: sub_28480 @ 0x28480
void sub_28480(int8 a0,int8 a1) // return-dupe
{
  unsigned int v1;
  char v10;
  uint8 v11; // stack - 0x238
  int8 v12; // stack - 0x230
  int8 v13; // stack - 0x218
  int8 v14; // stack - 0x60
  int4 v15; // stack - 0x50
  int4 v2;
  int8 v3; // rax
  char *v4;
  char *v5; // rax
  uint8 v6;
  unsigned long v7 [11]; // stack - 0x1f8
  uint8 v8; // stack - 0x200
  stat v9 [2];
  
  v3 = sub_1f810(a0);
  *(int8 *)(a1 + 0x38) = v3;
  v4 = (char *)sub_1fed0(v3);
  if (!v4)
    return;
  v6 = *(uint8 *)(a1 + 0x18);
  v8 = v6 + 0x66;
  if (v6 <= 99)
    v8 = 0x66;
  v5 = (char *)sub_4ecc0();
  v1 = *(unsigned int *)(a1 + 0x28);
  strcpy(v5,*(char **)(a1 + 0x10));
  v11 = v6;
  if (v5[v6 - 1] != '/') {
    v11 = v6 + 1;
    v5[v6] = '/';
    v5[v6 + 1] = '\0';
  }
  v12 = 0;
  v10 = *v4;
  if (v10) {
    v13 = 0;
    do {
      v6 = strlen(v4);
      if (v10 == 'D') {
        if (v8 < v11 + v6) {
          do {
            v5 = (char *)sub_4ed20(v5,&v8);
          } while (v8 < v11 + v6);
        }
        strcpy(&v5[v11],&v4[1]);
        v3 = sub_28210(v5,v1,0,a1);
        if (v12) // branch-flip
          *(int8 *)(v13 + 0x50) = v3;
        else {
          v12 = v3;
        }
        sub_2fb40(v7);
        v2 = *(int4 *)(a0 + 0x1a8);
        v14 = a0;
        if (0 <= v2) { // branch-flip
          v2 = sub_14950(a0,&v4[1],dat_82808 | 0x10000);
          if (v2 < 0) goto label_28645;
          v15 = v2;
          if (fstat(v2,v9)) // branch-flip
            sub_26460(v5);
          else {
            v7[0] = sub_4ee30(v5);
            sub_28480(v7,v3);
            sub_14a80(v7);
          }
        }
        else {
          *__errno_location() = -v2;
label_28645:
          sub_26370(v5);
        }
        sub_2fc00(v7);
        v13 = v3;
      }
      v4 = &v4[v6 + 1];
      v10 = *v4;
    } while (v10);
  }
  free(v5);
  *(int8 *)(a1 + 0x48) = v12;
  return;
}


// Function: sub_286f0 @ 0x286f0
void sub_286f0(char *a0)
{
  int8 *v1;
  unsigned long *v2; // rax
  char *v3; // rax
  uint8 v4; // rax
  int8 *v5;
  unsigned long *v6;
  
  v2 = (unsigned long *)sub_4ed80(0x60);
  if (!a0)
    a0 = "";
  v3 = (char *)sub_4ee30(a0);
  v2[2] = v3;
  v4 = strlen(v3);
  v1 = dat_826c0;
  v2[3] = v4;
  v6 = dat_826b8;
  if (dat_82a78) {
    v6 = (unsigned long *)dat_826c0[1];
    v5 = (int8 *)*dat_826c0;
    if (v6) { // branch-flip
      *v6 = v5;
      v5 = (int8)*dat_826c0;
    }
    else {
      dat_826c0 = v5;
    }
    if (v5) { // branch-flip
      v5[1] = (int8)v6;
      v6 = dat_826b8;
    }
    else {
      dat_826b8 = v6;
    }
    free((void *)v1[2]);
    free((void *)v1[0xb]);
    free(v1);
  }
  v1 = dat_826c0;
  v2[1] = 0;
  dat_826c0 = v2;
  *v2 = v1;
  if (!v6)
    dat_826b8 = v2;
  v2[6] = 0;
  *(unsigned int *)&v2[5] = 0;
  v2[7] = 0;
  v2[8] = 0;
  *(char *)((int8)v2 + 0x24) = 1;
  *(uint4 *)&v2[4] = dat_7f238 | dat_826f8 | dat_82ad4 | (uint4)(dat_826fc == 2) << 0x1c;
  dat_82a78 = 1;
  return;
}


// Function: sub_28880 @ 0x28880
uint8 sub_28880(char *a0) // early-return
{
  unsigned int v1;
  uint8 v2; // rax
  unsigned long *v3;
  char *v4;
  
  while( true ) {
    if (!dat_826c0) {
      v2 = 1;
      return v2;
    }
    v4 = (char *)dat_826c0[2];
    v3 = dat_826c0;
    if (!*v4) {
      sub_26300(*(unsigned int *)&dat_826c0[5]);
      v2 = 1;
      dat_826c0 = NULL;
      dat_826b8 = 0;
      return v2;
    }
    while (v1 = *(unsigned int *)&v3[4], !sub_40a20(v4,a0,v1)) {
      v3 = (unsigned long *)*v3;
      while( true ) {
        if (!v3) {
          if ((dat_82a78) && (!dat_826c0))
            dat_82a78 = '\0';
          goto label_288df;
        }
        v4 = (char *)v3[2];
        if (*v4) break;
        v3 = (unsigned long *)*v3;
      }
    }
    if (!dat_82a78) break;
    if (dat_826c0 == v3) {
      dat_82a78 = '\0';
      break;
    }
label_288df:
    if (((!dat_82aac) || (!dat_826c0[6])) || (sub_282f0(), dat_826c0[6])) {
      v2 = 0;
      return v2;
    }
  }
  if (((a0[v3[3]] != '/') || (!dat_82ad4)) || (!v3[6])) {
    v2 = v3[6] + 1;
    v3[6] = v2;
  }
  sub_26300(*(unsigned int *)&v3[5]);
  if (dat_82b38) // branch-flip
    v2 = CONCAT71((undefined7)((uint8)dat_82b38 >> 8),v3[6] == dat_82b38);
  else {
    v2 = (uint8)(v3[6] != 0);
  }
  return v2;
}


// Function: sub_289e0 @ 0x289e0
unsigned long sub_289e0(int8 a0) // return-dupe
{
  char *v1;
  unsigned long *v2;
  uint8 v3;
  uint8 v4; // rax
  
  v3 = dat_82b38;
  v1 = *(char **)(a0 + 8);
  if (!v1)
    return 0;
  if (!dat_82b38)
    return 0;
  if (*(char *)(a0 + 0x10))
    return 0;
  v4 = strlen(v1);
  v2 = dat_826c0;
  while( true ) {
    if (!v2)
      return 1;
    if ((*(char *)v2[2]) && ((uint8)v2[6] < v3)) break;
    if (((uint8)v2[3] <= v4) && (v1[v2[3]] == '/'))
      return 0;
    v2 = (unsigned long *)*v2;
  }
  return 0;
}


// Function: sub_28a70 @ 0x28a70
void sub_28a70(void) // return-dupe
{
  char v1;
  unsigned long *v2;
  char *v3;
  unsigned long v4; // rax
  int8 v5; // rax
  
  v2 = dat_826c0;
label_28a96:
  do {
    if (!v2) {
      dat_826c0 = NULL;
      dat_826b8 = 0;
      if (!dat_82aac) {
        dat_826c0 = NULL;
        dat_826b8 = 0;
        return;
      }
      while( true ) {
        v5 = sub_27340(1);
        if (!v5)
          return;
        v3 = *(char **)(v5 + 0x18);
        if (!v3) break;
        sub_26f30(v3);
        if (dat_82d50)
          (*dat_82d50)();
        v4 = sub_4ce40(v3);
        error(0,0,dcgettext(NULL,"%s: Not found in archive",5),v4);
        dat_82d58 = 2;
      }
      return;
    }
    if (dat_82b38) { // branch-flip
      if ((uint8)v2[6] < dat_82b38) {
        v3 = (char *)v2[2];
        v1 = *v3;
        goto label_28ad2;
      }
    }
    else if (!v2[6]) {
      v3 = (char *)v2[2];
      v1 = *v3;
label_28ad2:
      if (v1) {
        sub_26f30(v3);
        if (dat_82d50)
          (*dat_82d50)();
        v4 = sub_4ce40((char *)v2[2]);
        if (v2[6]) // branch-flip
          v3 = dcgettext(NULL,"%s: Required occurrence not found in archive",5);
        else {
          v3 = dcgettext(NULL,"%s: Not found in archive",5);
        }
        error(0,0,v3,v4);
        v2 = (unsigned long *)*v2;
        dat_82d58 = 2;
        goto label_28a96;
      }
    }
    v2 = (unsigned long *)*v2;
  } while( true );
}


// Function: sub_28c00 @ 0x28c00
void sub_28c00(void) // return-dupe
{
  char *v1;
  int8 *v2;
  int8 v3; // rax
  
  v2 = dat_826c0;
  if (!dat_826c0)
    return;
  do {
    if (dat_82b38) { // branch-flip
      if (dat_82b38 <= (uint8)v2[6])
        return;
    }
    else if (v2[6])
      return;
    v2 = (int8 *)*v2;
  } while (v2);
  if (dat_82a54)
    error(0,0,dcgettext(NULL,"Archive label mismatch",5));
  sub_2fd00(1);
  v2 = dat_826c0;
  while ((v2 && (v1 = (char *)v2[2], !sub_26f30(v1)))) {
    v2 = (int8 *)*v2;
  }
  dat_826c0 = NULL;
  dat_826b8 = 0;
  if (!dat_82aac) {
    dat_826c0 = NULL;
    dat_826b8 = 0;
    return;
  }
  while( true ) {
    v3 = sub_27340(1);
    if (!v3)
      return;
    v1 = *(char **)(v3 + 0x18);
    if (!v1) break;
    if (sub_26f30(v1))
      return;
  }
  return;
}


// Function: sub_28cf0 @ 0x28cf0
void sub_28cf0(void) // return-dupe
{
  unsigned int v1;
  int4 v10;
  unsigned long v11; // rax
  struct_62 *v12; // rax
  int8 *v13;
  int8 *v14; // rdx
  unsigned long v15 [11]; // stack - 0x1f8
  stat v16 [2]; // stack - 0x1a0
  char *v17;
  unsigned long *v18;
  int4 v19; // stack - 0x50
  unsigned long *v2;
  unsigned long *v3;
  unsigned long *v4;
  int8 *v5;
  int8 *v6;
  int8 *v7;
  unsigned long *v8;
  int4 v9;
  
  sub_282f0();
  if (dat_826c0) { // branch-flip
    if (dat_82b28) {
label_28d37:
      v9 = sub_26160();
      if (v9) {
        if (v9 != 1) {
          if (dat_82d50)
            (*dat_82d50)();
          v17 = "Only one -C option is allowed with --listed-incremental";
label_29189:
          error(0,0,dcgettext(NULL,v17,5));
          sub_2dea0(2); // no-return
        }
        if (!(int4)dat_826c0[5]) {
          if (dat_82d50)
            (*dat_82d50)();
          v17 = "Using -C option inside file list is not allowed with --listed-incremental";
          goto label_29189;
        }
      }
      sub_203b0();
      goto label_28d60;
    }
  }
  else {
    sub_28210(".",0,0,0);
    if (dat_82b28) goto label_28d37;
label_28d60:
    if (!dat_826c0) {
      v9 = 0;
      goto label_28e07;
    }
  }
  v13 = dat_826c0;
  v10 = 0;
  do {
    while ((((v9 = v10, v13[6] || (v13[7])) || (*(uint1 *)((int8)v13 + 0x23) & 0x10)) || (sub_26300(*(unsigned int *)&v13[5]), !*(char *)v13[2]))) {
label_28d80:
      v13 = (int8 *)*v13;
      v10 = v9 + 1;
      if (!v13) goto label_28e00;
    }
    sub_2fb40(v15);
    v17 = (char *)v13[2];
    if (sub_25fc0(v17,v16)) {
      sub_26460((char *)v13[2]);
      goto label_28d80;
    }
    if ((v16[0]._24_4_ & 0xf000) == 0x4000) {
      v10 = __openat_2(dat_7f234,(char *)v13[2],dat_82808 | 0x10000);
      if (0 <= v10) { // branch-flip
        v19 = v10;
        v10 = fstat(v10,v16);
        if (v10) // branch-flip
          sub_26460((char *)v13[2]);
        else {
          v15[0] = sub_4ee30((char *)v13[2]);
          v13[6] = v13[6] + 1;
          sub_28480(v15,v13);
        }
      }
      else {
        sub_26370((char *)v13[2]);
      }
    }
    sub_2fc00(v15);
    v13 = (int8 *)*v13;
    v10 = v9 + 1;
  } while (v13);
label_28e00:
  v9 += 1;
label_28e07:
  dat_826c0 = (unsigned long *)sub_26b90(dat_826c0,v9,sub_26d90);
  v8 = dat_826c0;
  v18 = NULL;
  while (v4 = v8, v4) {
    v4[1] = v18;
    v8 = (unsigned long *)*v4;
    v18 = v4;
  }
  v9 = 0;
  v18 = NULL;
  v11 = sub_43bf0(0,0,sub_26fd0,sub_26d70,0);
  v8 = dat_826c0;
  do {
    while( true ) {
      if (!v8) {
        dat_826b8 = v18;
        sub_43da0(v11);
        dat_826c0 = (int8 *)sub_26b90(dat_826c0,v9,sub_26fe0);
        if (!dat_826c0) {
          dat_826c0 = NULL;
          return;
        }
        v13 = NULL;
        v7 = dat_826c0;
        do {
          v14 = v7;
          v14[1] = (int8)v13;
          v13 = v14;
          v7 = (int8 *)*v14;
          v5 = dat_826c0;
          v6 = dat_82b28;
        } while ((int8 *)*v14);
        while( true ) {
          if (!v6)
            return;
          if (*(char *)v5[2]) break;
          v5 = (int8 *)*v5;
          v6 = v5;
        }
        sub_1ff30(v5[7]); // tail-call
        return;
      }
      v4 = (unsigned long *)*v8;
      v17 = (char *)v8[2];
      v1 = *(unsigned int *)&v8[5];
      v8[0xb] = sub_26700(v1,v17);
      if ((v18) && (v12 = (struct_62 *)sub_43960(v11,v8), v12)) break;
label_28f17:
      v8[6] = 0;
      if (!sub_44240(v11,v8))
        sub_17020(); // no-return
      v9 += 1;
      v18 = v8;
      v8 = v4;
    }
    if (!v8[8]) {
      if (v12->field_0x48)
        sub_27040(v12->field_0x48,(char *)v8[2],v8[3]);
      sub_442a0(v11,v8);
      v18 = v12->field_0x8;
      v2 = v12->field_0x0;
      if (v18) // branch-flip
        *v18 = v2;
      else {
        dat_826c0 = v2;
      }
      if (v2) // branch-flip
        v2[1] = v18;
      else {
        dat_826b8 = v18;
      }
      v9 -= 1;
      free(v12->field_0x10);
      free(v12->field_0x58);
      free(v12);
      goto label_28f17;
    }
    if (v8[9])
      sub_27040(v8[9],v12->field_0x10,v12->field_0x18);
    v2 = (unsigned long *)v8[1];
    v3 = (unsigned long *)*v8;
    if (v2) // branch-flip
      *v2 = v3;
    else {
      dat_826c0 = v3;
    }
    if (v3) // branch-flip
      v3[1] = v2;
    else {
      dat_826b8 = v2;
    }
    free((void *)v8[2]);
    free((void *)v8[0xb]);
    free(v8);
    v8 = v4;
  } while( true );
}


// Function: sub_291d0 @ 0x291d0
int8 * sub_291d0(char *a0)
{
  unsigned int v1;
  char *v2;
  int8 *v3;
  
  while( true ) {
    v3 = dat_826c0;
    if (!dat_826c0)
      return NULL;
    do {
      v2 = (char *)v3[2];
      if ((*v2) && (v1 = *(unsigned int *)&v3[4], sub_40a20(v2,a0,v1)))
        return v3;
      v3 = (int8 *)*v3;
    } while (v3);
    if (!dat_82aac)
      return NULL;
    if (!dat_826c0) break;
    if (!dat_826c0[6])
      return NULL;
    sub_282f0();
    if (dat_826c0[6])
      return NULL;
  }
  return NULL;
}


// Function: sub_29260 @ 0x29260
int8 * sub_29260(void) // return-dupe
{
  int8 *v1;
  
  v1 = dat_82620;
  if ((!dat_82620) && (dat_82620 = dat_826c0, v1 = dat_826c0, !dat_826c0))
    return NULL;
  while ((v1[6] || (!*(char *)v1[2]))) {
    v1 = (int8 *)*v1;
    dat_82620 = v1;
    if (!v1)
      return NULL;
  }
  v1[6] = 1;
  sub_26300(*(unsigned int *)&v1[5]);
  return dat_82620;
}


// Function: sub_292e0 @ 0x292e0
void sub_292e0(void) // return-dupe
{
  int8 *v1;
  
  dat_82620 = 0;
  v1 = dat_826c0;
  if (!dat_826c0)
    return;
  do {
    v1[6] = 0;
    v1 = (int8 *)*v1;
  } while (v1);
  dat_82620 = 0;
  return;
}


// Function: sub_29320 @ 0x29320
void * sub_29320(char *a0,char *a1)
{
  uint8 v1;
  uint8 v2; // rax
  void *v3; // rax
  uint8 v4;
  
  v2 = strlen(a0);
  v1 = strlen(a1) + 1;
  v4 = 0;
  if ((v2) && (v4 = v2, a0[v2 - 1] != '/'))
    v4 = v2 + 1;
  v3 = (void *)sub_4ecc0(v1 + v4);
  memcpy(v3,a0,v2);
  *(char *)((int8)v3 + v2) = 0x2f;
  memcpy((void *)((int8)v3 + v4),a1,v1);
  return v3;
}


// Function: sub_293b0 @ 0x293b0
int8 sub_293b0(char *a0,int8 a1)
{
  char *v1;
  char v2;
  char *v3;
  
  v2 = *a0;
  v3 = a0;
  while (v2 == '/') {
    v1 = &v3[1];
    v3 = &v3[1];
    v2 = *v1;
  }
  while( true ) {
    while( true ) {
      if (!v2)
        return -1;
      v3 = &v3[1];
      if (v2 == '/') break;
      v2 = *v3;
    }
    a1 -= 1;
    if (!a1) break;
    for (; v2 = *v3, v2 == '/'; v3 = &v3[1]) {
    }
  }
  return (int8)v3 - (int8)a0;
}


// Function: sub_29420 @ 0x29420
unsigned long sub_29420(char *a0)
{
  char v1;
  unsigned long v2; // rax
  char v3;
  
  while (((v3 = *a0, v3 != '.' || (a0[1] != '.')) || (v1 = a0[2], v2 = CONCAT71((undefined7)((uint8)v2 >> 8),v1 == '/' || !v1), v1 != '/' && v1))) {
    while (a0 = &a0[1], v3 != '/') {
      if (!v3)
        return 0;
      v3 = *a0;
    }
  }
  return v2;
}


// Function: sub_29490 @ 0x29490
unsigned int sub_29490(int8 a0) // early-return
{
  uint4 v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x18) + 0x130))
    return 1;
  v1 = *(int4 *)(*(int8 *)(a0 + 0x18) + 0x124);
  return CONCAT31((undefined3)(v1 >> 8),v1 != 0);
}


// Function: sub_294c0 @ 0x294c0
uint8 sub_294c0(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  uint8 v3;
  
  v2 = sub_231e0((uint1 *)(dat_82408 + 0x1c4),0xc);
  v1 = *(int8 *)(a0 + 0x18);
  *(unsigned long *)(v1 + 0x118) = *(unsigned long *)(v1 + 0x88);
  v3 = 0;
  if (0 <= (int8)v2)
    v3 = v2;
  *(uint8 *)(v1 + 0x88) = v3;
  return ~v2 >> 0x3f;
}


// Function: sub_29510 @ 0x29510
uint8 sub_29510(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  uint8 v3;
  
  v2 = sub_231e0((uint1 *)(dat_82408 + 0x1e3),0xc);
  v1 = *(int8 *)(a0 + 0x18);
  *(unsigned long *)(v1 + 0x118) = *(unsigned long *)(v1 + 0x88);
  v3 = 0;
  if (0 <= (int8)v2)
    v3 = v2;
  *(uint8 *)(v1 + 0x88) = v3;
  return ~v2 >> 0x3f;
}


// Function: sub_29560 @ 0x29560
void sub_29560(int8 a0,struct_40 *a1)
{
  unsigned long *v1;
  uint8 v2;
  unsigned long v3;
  void *v4;
  uint8 v5;
  int8 v6; // rsi
  
  v4 = *(void **)(a0 + 0x140);
  v2 = *(uint8 *)(a0 + 0x130);
  v5 = *(uint8 *)(a0 + 0x138);
  if (v5 == v2) {
    if (v4) { // branch-flip
      if (0x555555555555554 < v5)
        sub_17020(); // no-return, return-dupe
      v5 = v5 + 1 + (v5 >> 1);
      v6 = v5 * 0x10;
    }
    else if (v5) { // branch-flip
      v6 = v5 << 4;
      if ((v6 < 0) || (v5 >> 0x3c)) {
        sub_17020();
      }
    }
    else {
      v6 = 0x80;
      v5 = 8;
    }
    *(uint8 *)(a0 + 0x138) = v5;
    v4 = (void *)sub_4ece0(v4,v6);
    *(void **)(a0 + 0x140) = v4;
  }
  v3 = a1->field_0x8;
  v1 = (unsigned long *)((int8)v4 + v2 * 0x10);
  *v1 = a1->field_0x0;
  v1[1] = v3;
  *(uint8 *)(a0 + 0x130) = v2 + 1;
  return;
}


// Function: sub_29630 @ 0x29630
unsigned int sub_29630(int8 a0)
{
  char *v1;
  uint8 v10; // rax
  uint8 v11;
  int8 v12;
  char *v13;
  char *v14;
  char *v15;
  char *v16;
  uint8 v17; // stack - 0x60
  char v18 [24];
  uint8 *v19;
  char v2;
  int8 v20;
  uint8 *v21;
  uint8 *v22; // r15
  int8 v23; // stack - 0x80
  unsigned long v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned int v6; // eax
  int8 v7;
  int8 v8;
  uint8 v9;
  
  v20 = *(int8 *)(a0 + 0x18);
  *(int4 *)(v20 + 0x128) = dat_82a80;
  *(int4 *)(v20 + 0x124) = dat_82a84;
  if (dat_82a84) { // branch-flip
    v23 = sub_cc60();
    v20 = *(int8 *)(a0 + 0x18);
    v3 = *(unsigned long *)(v20 + 8);
    v9 = *(uint8 *)(v20 + 0x130);
    v19 = *(uint8 **)(v20 + 0x140);
    v9 = strlen((char *)sub_45370(v9,v18));
    v7 = *(int8 *)(a0 + 0x18);
    v20 = v9 + 1;
    if (*(int8 *)(v7 + 0x130)) {
      v9 = 0;
      v21 = v19;
      do {
        v11 = *v21;
        v9 += 1;
        v22 = &v21[2];
        v10 = strlen((char *)sub_45370(v11,v18));
        v11 = v21[1];
        v11 = strlen((char *)sub_45370(v11,v18));
        v7 = *(int8 *)(a0 + 0x18);
        v20 = v10 + 2 + v20 + v11;
        v21 = v22;
      } while (v9 < *(uint8 *)(v7 + 0x130));
    }
    v9 = v20 + 0x3fe;
    if (0 <= (int8)(v20 + 0x1ffU))
      v9 = v20 + 0x1ffU;
    *(int8 *)(v7 + 0x118) = *(int8 *)(v7 + 0x118) + (v9 & 0xfffffffffffffe00);
    *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + (v9 & 0xfffffffffffffe00);
    sub_1da70("GNU.sparse.major",v7,0);
    sub_1da70("GNU.sparse.minor",*(int8 *)(a0 + 0x18),0);
    sub_1da70("GNU.sparse.name",*(int8 *)(a0 + 0x18),0);
    sub_1da70("GNU.sparse.realsize",*(int8 *)(a0 + 0x18),0);
    v5 = *(unsigned long **)(a0 + 0x18);
    v5[1] = sub_1cdf0(v5,"%d/GNUSparseFile.%p/%f",0);
    v20 = *(int8 *)(a0 + 0x18);
    v15 = *(char **)(v20 + 8);
    if (0x65 <= strlen(v15)) {
      v15[100] = '\0';
      v20 = *(int8 *)(a0 + 0x18);
    }
    v4 = *(unsigned long *)(v20 + 0x88);
    *(unsigned long *)(v20 + 0x88) = *(unsigned long *)(v20 + 0x118);
    v12 = sub_13570(v20);
    v7 = *(int8 *)(a0 + 0x18);
    *(unsigned long *)(v20 + 0x88) = v4;
    sub_13320(v7,v12,v23);
    free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
    *(unsigned long *)(*(int8 *)(a0 + 0x18) + 8) = v3;
    v13 = (char *)sub_d360();
    v14 = (char *)sub_45370(*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130),v18);
    v16 = &v13[0x200];
    v2 = *v14;
    v15 = v13;
    while (v2) {
      if (v16 == v15) {
        sub_ccd0(v13);
        v15 = (char *)sub_d360();
        v16 = &v15[0x200];
        v13 = v15;
      }
      v2 = *v14;
      v14 = &v14[1];
      v1 = &v15[1];
      *v15 = v2;
      v2 = *v14;
      v15 = v1;
    }
    if (v16 == v15) {
      sub_ccd0(v13);
      v15 = (char *)sub_d360();
      v13 = v15;
    }
    *v15 = '\n';
    v15 = &v15[1];
    if (*(int8 *)(*(int8 *)(a0 + 0x18) + 0x130)) {
      v9 = 0;
      do {
        v14 = (char *)sub_45370(*v19,v18);
        v16 = &v13[0x200];
        v2 = *v14;
        while (v2) {
          if (v15 == v16) {
            sub_ccd0(v13);
            v15 = (char *)sub_d360();
            v16 = &v15[0x200];
            v13 = v15;
          }
          v2 = *v14;
          v14 = &v14[1];
          v1 = &v15[1];
          *v15 = v2;
          v2 = *v14;
          v15 = v1;
        }
        if (v16 == v15) {
          sub_ccd0(v13);
          v15 = (char *)sub_d360();
          v16 = &v15[0x200];
          v13 = v15;
        }
        *v15 = '\n';
        v14 = (char *)sub_45370(v19[1],v18);
        v2 = *v14;
        while (v15 = &v15[1], v2) {
          if (v15 == v16) {
            sub_ccd0(v13);
            v15 = (char *)sub_d360();
            v16 = &v15[0x200];
            v13 = v15;
          }
          v2 = *v14;
          v14 = &v14[1];
          *v15 = v2;
          v2 = *v14;
        }
        if (v16 == v15) {
          sub_ccd0(v13);
          v15 = (char *)sub_d360();
          v13 = v15;
        }
        *v15 = '\n';
        v15 = &v15[1];
        v9 += 1;
        v19 = &v19[2];
      } while (v9 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130));
    }
    v6 = 1;
    memset(v15,0,(uint8)&v13[0x200 - (int8)v15]);
    sub_ccd0(v13);
  }
  else {
    v7 = sub_cc60();
    v20 = *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140);
    sub_1da70("GNU.sparse.size",*(int8 *)(a0 + 0x18),0);
    sub_1da70("GNU.sparse.numblocks",*(int8 *)(a0 + 0x18),0);
    if ((sub_1c9c0("GNU.sparse.map")) || (!dat_82a80)) {
      v17 = 0;
      v20 = *(int8 *)(a0 + 0x18);
      if (*(int8 *)(v20 + 0x130)) {
        do {
          sub_1da70("GNU.sparse.offset",v20,&v17);
          sub_1da70("GNU.sparse.numbytes",*(int8 *)(a0 + 0x18),&v17);
          v20 = *(int8 *)(a0 + 0x18);
          v17 += 1;
        } while (v17 < *(uint8 *)(v20 + 0x130));
      }
      v3 = *(unsigned long *)(v20 + 0x88);
      v6 = 1;
      *(unsigned long *)(v20 + 0x88) = *(unsigned long *)(v20 + 0x118);
      v8 = sub_13570(v20);
      v12 = *(int8 *)(a0 + 0x18);
      *(unsigned long *)(v20 + 0x88) = v3;
      sub_13320(v12,v8,v7);
    }
    else {
      sub_1da70("GNU.sparse.name",*(int8 *)(a0 + 0x18),0);
      v5 = *(unsigned long **)(a0 + 0x18);
      v12 = v5[1];
      v5[1] = sub_1cdf0(v5,"%d/GNUSparseFile.%p/%f",0);
      sub_1de70(*(int8 *)(a0 + 0x18) + 0x168);
      v8 = *(int8 *)(a0 + 0x18);
      v17 = 0;
      v9 = 0;
      if (*(int8 *)(v8 + 0x130)) {
        while( true ) {
          v15 = (char *)sub_45370(*(uint8 *)(v20 + v9 * 0x10),v18);
          sub_1de80((struct_77 *)(*(int8 *)(a0 + 0x18) + 0x168),v15);
          sub_1de80((struct_77 *)(*(int8 *)(a0 + 0x18) + 0x168),",");
          v15 = (char *)sub_45370(*(uint8 *)(v20 + 8 + v17 * 0x10),v18);
          sub_1de80((struct_77 *)(*(int8 *)(a0 + 0x18) + 0x168),v15);
          v8 = *(int8 *)(a0 + 0x18);
          v17 += 1;
          if (*(uint8 *)(v8 + 0x130) <= v17) break;
          v9 = v17;
          if (v17) {
            sub_1de80((struct_77 *)(v8 + 0x168),",");
            v9 = v17;
          }
        }
      }
      v6 = sub_1df00((int8 *)(v8 + 0x168),"GNU.sparse.map");
      if ((char)v6) { // branch-flip
        v20 = *(int8 *)(a0 + 0x18);
        v3 = *(unsigned long *)(v20 + 0x88);
        *(unsigned long *)(v20 + 0x88) = *(unsigned long *)(v20 + 0x118);
        v8 = sub_13570(v20);
        *(unsigned long *)(v20 + 0x88) = v3;
        sub_13320(*(int8 *)(a0 + 0x18),v8,v7);
        if (v12) {
          free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
          *(int8 *)(*(int8 *)(a0 + 0x18) + 8) = v12;
        }
      }
      else {
        free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
        *(int8 *)(*(int8 *)(a0 + 0x18) + 8) = v12;
      }
    }
  }
  return v6;
}


// Function: sub_29e30 @ 0x29e30
unsigned long sub_29e30(int8 a0) // return-dupe x3
{
  uint1 *v1;
  char v2;
  char *v3;
  int4 v4; // eax
  int8 v5;
  int8 v6;
  uint8 v7; // rbp
  uint1 *v8;
  
  v5 = dat_82408;
  v2 = *(char *)(dat_82408 + 0x159);
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
  if ((!v2) && (*(char *)(dat_82408 + 0x16e))) {
    v8 = (uint1 *)(dat_82408 + 0x164);
    v1 = (uint1 *)(dat_82408 + 0x1c4);
    do {
      if (!v8[0xc])
        return 1;
      v4 = sub_29d90(a0,v8);
      if (v4) goto label_29ee9;
      v8 = &v8[0x18];
    } while (v1 != v8);
    if (!*(char *)(v5 + 0x163))
      return 1;
  }
  do {
    v5 = sub_d360();
    if (!v5) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      dat_82d58 = 2;
      return 0;
    }
    v7 = 0;
    sub_ccd0(v5);
    v6 = v5;
    do {
      if (!*(char *)(v6 + 0xc)) {
        *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + 0x200;
        return 1;
      }
      v7 += 1;
      v4 = sub_29d90(a0,v6);
      v6 += 0x18;
    } while ((v7 <= 0x14) && (!v4));
    *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + 0x200;
  } while ((*(char *)(v5 + 0x1f8)) && (!v4));
label_29ee9:
  if (v4 != 2)
    return 1;
  if (dat_82d50)
    (*dat_82d50)();
  v3 = (char *)**(unsigned long **)(a0 + 0x18);
  error(0,0,dcgettext(NULL,"%s: invalid sparse archive member",5),v3);
  dat_82d58 = 2;
  return 0;
}


// Function: sub_29fd0 @ 0x29fd0
unsigned long sub_29fd0(int8 a0) // return-dupe
{
  uint1 *v1;
  char *v2;
  int4 v3; // eax
  int8 v4;
  uint1 *v5;
  int8 v6;
  uint8 v7; // rbp
  
  v4 = dat_82408;
  v5 = (uint1 *)(dat_82408 + 0x182);
  v1 = (uint1 *)(dat_82408 + 0x1e2);
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
  do {
    if (!v5[0xc])
      return 1;
    v3 = sub_29d90(a0,v5);
    if (v3) goto label_2a096;
    v5 = &v5[0x18];
  } while (v5 != v1);
  if (!*(char *)(v4 + 0x1e2))
    return 1;
  do {
    v4 = sub_d360();
    if (!v4) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      dat_82d58 = 2;
      return 0;
    }
    v7 = 0;
    sub_ccd0(v4);
    v6 = v4;
    do {
      if (!*(char *)(v6 + 0xc))
        return 1;
      v7 += 1;
      v3 = sub_29d90(a0,v6);
      v6 += 0x18;
    } while ((v7 <= 0x14) && (!v3));
  } while ((*(char *)(v4 + 0x1f8)) && (!v3));
label_2a096:
  if (v3 != 2)
    return 1;
  if (dat_82d50)
    (*dat_82d50)();
  v2 = (char *)**(unsigned long **)(a0 + 0x18);
  error(0,0,dcgettext(NULL,"%s: invalid sparse archive member",5),v2);
  dat_82d58 = 2;
  return 0;
}


// Function: sub_2a650 @ 0x2a650
uint4 sub_2a650(struct_78 *a0,int8 a1)
{
  uint1 v1;
  unsigned int v2;
  uint4 v3; // eax
  int8 v4; // rax
  uint8 v5; // rax
  uint8 v6;
  int8 v7;
  
  v3 = sub_2a5b0(a0,*(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + a1 * 0x10));
  if ((char)v3) {
    v7 = *(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + 8 + a1 * 0x10);
    if (v7) { // branch-flip
      while (0 < v7) {
        v6 = 0x200;
        if (v7 <= 0x200)
          v6 = v7;
        v4 = sub_d360();
        if (!v4) {
          if (dat_82d50)
            (*dat_82d50)();
          error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
          dat_82d58 = 2;
          return 0;
        }
        sub_ccd0(v4);
        a0->field_0x10 = a0->field_0x10 + 0x200;
        v5 = sub_26070(a0->field_0x0,v4,v6);
        v7 -= v5;
        sub_c710(*(int8 *)(a0->field_0x18 + 0x118) - a0->field_0x10);
        a0->field_0x8 = a0->field_0x8 + v5;
        if (v6 != v5) {
          sub_33510(*(char **)a0->field_0x18,v5,v6);
          return 0;
        }
      }
    }
    else {
      v1 = a0->field_0x4;
      if ((v1) && (v2 = a0->field_0x0, sub_2c110(v2))) {
        v3 = (uint4)v1;
        sub_33470(*(char **)a0->field_0x18);
      }
    }
  }
  return v3;
}


// Function: sub_2a7c0 @ 0x2a7c0
char sub_2a7c0(struct_79 *a0,int8 a1) // return-dupe
{
  int4 v1;
  uint8 *v10; // rsi
  uint8 v11;
  int8 *v12; // r15
  char v13; // stack - 0xf1
  char v2; // al
  int8 v3; // rax
  int8 v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  int8 v7;
  stat v8; // stack - 0xe8
  char v9 [24];
  
  a1 *= 0x10;
  v10 = (uint8 *)(a1 + *(int8 *)(a0->field_0x18 + 0x140));
  v7 = v10[1];
  v2 = sub_2a5b0(a0,*v10);
  v13 = v2;
  if (v2) {
    while (0 < v7) {
      v11 = 0x200;
      if (v7 <= 0x200)
        v11 = v7;
      v3 = sub_d360();
      v4 = sub_4d570(a0->field_0x0,v3,v11);
      if (v4 == -1) {
        v12 = (int8 *)(a1 + ((unsigned long *)a0->field_0x18)[0x28]);
        sub_263a0(*(char **)(unsigned long *)a0->field_0x18,(v12[1] + *v12) - v7,v11);
        return '\0';
      }
      if (!v4) {
        v1 = a0->field_0x0;
        if (fstat(v1,&v8)) { // branch-flip
          v12 = (int8 *)(a1 + *(int8 *)(a0->field_0x18 + 0x140));
          v7 = (v7 + *(int8 *)(a0->field_0x18 + 0x88)) - (v12[1] + *v12);
        }
        else {
          v7 = *(int8 *)(a0->field_0x18 + 0x88) - v8._48_8_;
        }
        if (dat_81b80 & 0x80) {
          if (dat_82d50)
            (*dat_82d50)();
          v5 = sub_45370(v7,v9);
          v6 = sub_4ce40(*(char **)a0->field_0x18);
          error(0,0,dcngettext(NULL,"%s: File shrank by %s byte; padding with zeros","%s: File shrank by %s bytes; padding with zeros",v7,5),v6,v5);
        }
        if (dat_82b52)
          return '\0';
        sub_2fd00(1);
        return '\0';
      }
      memset((void *)(v3 + v4),0,0x200 - v4);
      a0->field_0x10 = a0->field_0x10 + v4;
      sub_ccd0(v3);
      v7 -= v4;
      v13 = v2;
    }
  }
  return v13;
}


// Function: sub_2a9d0 @ 0x2a9d0
unsigned int sub_2a9d0(struct_37 *a0,uint8 a1,int8 a2)
{
  unsigned int v1; // eax
  int8 v2; // rax
  unsigned long v3; // rax
  char *v4;
  char *v5; // rdx
  char v6 [32];
  char v7 [520];
  int8 v8; // r13
  
  v1 = sub_2a5b0(a0,a1);
  if ((char)v1) {
    for (; (int8)a1 < a2; a1 = a1 + v2) {
      v8 = a2 - a1;
      if (0x201 <= v8)
        v8 = 0x200;
      v2 = sub_4d570(a0->field_0x0,v7,v8);
      if (v2 == -1) {
        sub_263a0((char *)*a0->field_0x18,a1,v8);
        return 0;
      }
      if (!v2) {
        v4 = dcgettext(NULL,"Size differs",5);
        sub_11120(a0->field_0x18,v4);
        return 0;
      }
      v4 = v7;
      do {
        v5 = &v4[1];
        if (*v4) {
          v3 = sub_452d0(a1,v6);
          v4 = dcgettext(NULL,"File fragment at %s is not a hole",5);
          sub_11120(a0->field_0x18,v4,v3);
          return 0;
        }
        v4 = v5;
      } while (&v7[v2] != v5);
    }
  }
  return v1;
}


// Function: sub_2ab20 @ 0x2ab20
char sub_2ab20(struct_38 *a0,int8 a1)
{
  uint8 v1; // rax
  void *v2; // rax
  char *v3; // rax
  char v4 [520];
  int8 v5;
  uint8 v6;
  int8 *v7; // r15
  char v8; // stack - 0x251
  
  a1 *= 0x10;
  v8 = sub_2a5b0(a0,*(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + a1));
  if (v8) {
    v5 = *(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + 8 + a1);
    sub_c710(*(int8 *)(a0->field_0x18 + 0x118) - a0->field_0x10);
    do {
      if (v5 <= 0)
        return v8;
      v6 = 0x200;
      if (v5 <= 0x200)
        v6 = v5;
      v2 = (void *)sub_d360();
      if (!v2) {
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
        dat_82d58 = 2;
        v8 = '\0'; // return-dupe
        return v8;
      }
      sub_ccd0(v2);
      a0->field_0x10 = a0->field_0x10 + 0x200;
      v1 = sub_4d570(a0->field_0x0,v4,v6);
      if (v1 == 0xffffffffffffffff) {
        v7 = (int8 *)(a1 + ((unsigned long *)a0->field_0x18)[0x28]);
        sub_263a0(*(char **)(unsigned long *)a0->field_0x18,(v7[1] + *v7) - v5,v6);
        v8 = '\0';
        return v8;
      }
      if (!v1) {
        sub_11120(0x82840,dcgettext(NULL,"Size differs",5));
        v8 = '\0';
        return v8;
      }
      v5 -= v1;
      sub_c710(*(int8 *)(a0->field_0x18 + 0x118) - a0->field_0x10);
    } while (!memcmp(v2,v4,v1));
    v3 = dcgettext(NULL,"Contents differ",5);
    sub_11120(a0->field_0x18,v3);
    v8 = '\0';
  }
  return v8;
}


// Function: sub_2ad50 @ 0x2ad50
unsigned long * sub_2ad50(int8 a0) // return-dupe
{
  char *v1;
  unsigned long *v10;
  int8 v11; // r15
  int8 v12;
  bool v13; // al
  int8 v2;
  int8 v3;
  unsigned long *v4;
  unsigned long *v5; // rax
  uint8 v6; // rcx
  char *v7;
  unsigned long *v8;
  uint8 v9;
  
  v5 = (int8)sub_cc60();
  v2 = (int8)v5;
  v5 = (int8)sub_13570(*(int8 *)(a0 + 0x18));
  v3 = (int8)v5;
  *(char *)((int8)v5 + 0x9c) = 0x53;
  v12 = *(int8 *)(a0 + 0x18);
  if (5 <= *(uint8 *)(v12 + 0x130))
    *(char *)((int8)v5 + 0x1e2) = 1;
  v7 = (char *)((int8)v5 + 0x182);
  sub_12f80(*(uint8 *)(v12 + 0x88),(char *)((int8)v5 + 0x1e3),0xc);
  sub_12f80(*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x118),(char *)((int8)v5 + 0x7c),0xc);
  v12 = *(int8 *)(a0 + 0x18);
  if (*(int8 *)(v12 + 0x130)) { // branch-flip
    v9 = 0;
    do {
      v11 = v9 * 0x10;
      v9 += 1;
      sub_12f80(*(uint8 *)(*(int8 *)(v12 + 0x140) + v11),v7,0xc);
      v1 = &v7[0xc];
      v7 = &v7[0x18];
      sub_12f80(*(uint8 *)(*(int8 *)(*(int8 *)(a0 + 0x18) + 0x140) + 8 + v11),v1,0xc);
      v12 = *(int8 *)(a0 + 0x18);
      v13 = v9 < *(uint8 *)(v12 + 0x130);
      if (v9 == 4) break;
    } while (v13);
  }
  else {
    v13 = 0;
    v9 = 0;
  }
  *(bool *)(v3 + 0x1e2) = v13;
  sub_13320(v12,v3,v2);
  if (*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130) <= v9) {
    v5 = (unsigned long *)0x1;
    return v5;
  }
  do {
    v5 = (unsigned long *)sub_d360();
    v4 = v5;
    *v5 = 0;
    v5[0x3f] = 0;
    v6 = (uint8)(((int4)v5 - (int4)(unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
    v10 = (unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    while (v6) {
      v8 = &v10[1];
      *v10 = 0;
      v6 -= 1;
      v10 = v8;
    }
    if (v9 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130)) {
      v10 = &v5[0x3f];
      v12 = *(int8 *)(a0 + 0x18);
      v8 = v5;
      do {
        v5 = (unsigned long *)v12;
        v12 = v9 * 0x10;
        v9 += 1;
        sub_12f80(*(uint8 *)(*(int8 *)((int8)v5 + 0x140) + v12),v8,0xc);
        v7 = (char *)((int8)v8 + 0xc);
        v8 = &v8[3];
        sub_12f80(*(uint8 *)(*(int8 *)(*(int8 *)(a0 + 0x18) + 0x140) + 8 + v12),v7,0xc);
        if (*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130) <= v9) goto label_2af2d;
        v12 = *(int8 *)(a0 + 0x18);
      } while (v10 != v8);
      *(char *)&v4[0x3f] = 1;
    }
label_2af2d:
    sub_ccd0(v4);
    if (*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130) <= v9) {
      v5 = (unsigned long *)0x1;
      return v5;
    }
  } while( true );
}


// Function: sub_2af80 @ 0x2af80
uint1 sub_2af80(int4 a0,int8 a1)
{
  int4 v1;
  int4 v10; // stack - 0x278
  int8 v11; // stack - 0x288
  char v12 [520];
  int8 v13;
  uint1 v14; // stack - 0x291
  int8 v15; // stack - 0x280
  char v16; // stack - 0x274
  int8 v17; // stack - 0x268
  int8 v18; // stack - 0x260
  int8 v19; // stack - 0x258
  char v2; // al
  char *v3; // rax
  char *v4;
  int8 v5; // rax
  int8 v6; // rax
  code *v7;
  uint8 v8;
  uint8 v9; // rbx
  
  v14 = sub_29d10(&v10);
  if (!v14)
    return 3;
  v18 = a1;
  v16 = 1;
  v15 = 0;
  v10 = a0;
  if (!*(int8 *)(a1 + 0x98)) {
    *(unsigned long *)(a1 + 0x118) = 0;
label_2b2ea:
    v15 = 0;
    v11 = *(int8 *)(a1 + 0x88);
    sub_29560(a1,&v11);
    v13 = v19;
    goto label_2b280;
  }
  if (dat_82a7c != 1) { // branch-flip
    if (!(dat_82a7c & 0xfffffffd)) {
      v6 = 0;
      v11 = 0;
      v13 = 0;
      while( true ) {
        *(int8 *)(a1 + 0x118) = v6;
        v6 = lseek(a0,v13,3);
        if (v6 == -1) break;
        v5 = lseek(a0,v6,4);
        if ((!v13 && !v6) && (v5 == *(int8 *)(a1 + 0x88))) {
          lseek(a0,0,0);
          a1 = v18;
          v1 = v10;
          goto label_2b069;
        }
        v11 = v6;
        v15 = v5 - v6;
        sub_29560(a1,&v11);
        v6 = v15 + *(int8 *)(a1 + 0x118);
        v13 = v5;
      }
      if (*__errno_location() == 6) goto label_2b2ea;
      a1 = v18;
      v1 = v10;
      goto label_2b069;
    }
  }
  else {
    v1 = a0;
label_2b069:
    v11 = 0;
    v15 = 0;
    *(unsigned long *)(a1 + 0x118) = 0;
    v7 = *(code **)(v19 + 0x30);
    if ((!v7) || ((*v7)(&v10,0,0))) {
      v13 = 0;
      while ((v6 = sub_25fe0(v1,v12,0x200), v6 && (v6 != -1))) {
        v4 = v12;
        do {
          v3 = &v4[1];
          if (*v4) {
            if (!v15)
              v11 = v13;
            v15 += v6;
            *(int8 *)(a1 + 0x118) = *(int8 *)(a1 + 0x118) + v6;
            v7 = *(code **)(v19 + 0x30);
            if (!v7) goto label_2b10f;
            v4 = v12;
            goto label_2b1f3;
          }
          v4 = v3;
        } while (v3 != &v12[v6]);
        if (v15) {
          sub_29560(a1,&v11);
          v15 = 0;
          v7 = *(code **)(v19 + 0x30);
          v4 = NULL;
          if (v7) {
label_2b1f3:
            if (!(*v7)(&v10,1,v4)) goto label_2b205;
          }
        }
label_2b10f:
        v13 += v6;
      }
      if (!v15)
        v11 = v13;
      sub_29560(a1,&v11);
      *(int8 *)(a1 + 0x118) = *(int8 *)(a1 + 0x118) + v6;
      v7 = *(code **)(v19 + 0x30);
      v13 = v19;
      if (v7) {
        if (!(*v7)(&v10,2,0)) goto label_2b205;
        v13 = v19;
      }
label_2b280:
      if (*(int8 *)(v13 + 0x38)) { // branch-flip
        if (*(code **)(v13 + 0x18))
          (**(code **)(v13 + 0x18))(&v10);
        v13 = v18;
        if (0 <= a0) {
          sub_c640(*(char **)(v18 + 8),*(unsigned long *)(v18 + 0x88),*(int8 *)(v18 + 0x118) - v17);
          v8 = 0;
          do {
            v13 = v18;
            if (*(uint8 *)(v18 + 0x130) <= v8) goto label_2b2aa;
            a1 = v18;
            if (!*(code **)(v19 + 0x38)) goto label_2b002;
            v9 = v8 + 1;
            v2 = (**(code **)(v19 + 0x38))(&v10,v8);
            v8 = v9;
          } while (v2);
          goto label_2b205;
        }
      }
      else {
        v13 = v18;
      }
label_2b2aa:
      sub_147d0(*(int8 *)(v13 + 0x118) - v17);
      v7 = *(code **)(v19 + 8);
      if (!v7)
        return 0;
      return (*v7)(&v10) & v14 ^ 1; // return-dupe
    }
label_2b205:
    a1 = v18;
  }
label_2b002:
  sub_147d0(*(int8 *)(a1 + 0x118) - v17);
  v14 = 0;
  v7 = *(code **)(v19 + 8);
  if (!v7)
    return 1;
  return (*v7)(&v10) & v14 ^ 1;
}


// Function: sub_2b3a0 @ 0x2b3a0
unsigned long sub_2b3a0(unsigned long a0) // early-return
{
  code *v1;
  char v2 [24];
  unsigned long v3; // stack - 0x30
  int8 v4; // stack - 0x28
  
  if ((sub_29d10(v2)) && (v1 = *(code **)(v4 + 0x10), v1)) {
    v3 = a0;
    return (*v1)();
  }
  return 0;
}


// Function: sub_2b400 @ 0x2b400
void sub_2b400(unsigned long a0)
{
  char v1 [24];
  unsigned long v2; // stack - 0x30
  int8 v3; // stack - 0x28
  
  if ((sub_29d10(v1)) && (*(code **)(v3 + 0x20))) {
    v2 = a0;
    (**(code **)(v3 + 0x20))();
  }
  return;
}


// Function: sub_2b460 @ 0x2b460
uint4 sub_2b460(uint4 a0,int8 a1,int8 *a2)
{
  uint4 v1; // eax
  int8 v10; // stack - 0x58
  int8 v11; // stack - 0x50
  int8 v12; // stack - 0x48
  int8 v2;
  code *v3;
  int8 v4;
  uint8 v5;
  uint8 v6; // rbx
  unsigned int v7; // stack - 0x68
  char v8; // stack - 0x64
  unsigned long v9; // stack - 0x60
  
  v5 = (uint8)a0;
  v1 = sub_29d10(&v7);
  if (!(char)v1)
    return 3;
  v11 = a1;
  v7 = (unsigned int)v5;
  v2 = lseek(v5 & 0xffffffff,0,0);
  v9 = 0;
  v8 = v2 == 0;
  v3 = *(code **)(v12 + 0x28);
  if ((v3) && (!(*v3)(&v7))) {
label_2b570:
    v2 = v12;
    v4 = v11;
label_2b580:
    v3 = *(code **)(v2 + 8);
    v1 = 0;
    *a2 = *(int8 *)(v4 + 0x118) - v10;
    if (!v3)
      return 1;
  }
  else {
    v5 = 0;
    v2 = v12;
    if (*(int8 *)(v11 + 0x130)) {
      do {
        v3 = *(code **)(v2 + 0x40);
        v4 = v11;
        if (!v3) goto label_2b580;
        v6 = v5 + 1;
        if (!(*v3)(&v7,v5)) goto label_2b570;
        v2 = v12;
        v5 = v6;
      } while (v6 < *(uint8 *)(v11 + 0x130));
    }
    v3 = *(code **)(v12 + 8);
    *a2 = *(int8 *)(v11 + 0x118) - v10;
    if (!v3)
      return 0;
  }
  return ((*v3)(&v7) & v1 ^ 1) & 0xff;
}


// Function: sub_2b5b0 @ 0x2b5b0
uint4 sub_2b5b0(int8 a0)
{
  uint4 v1; // eax
  code *v2;
  unsigned int v3 [4]; // stack - 0x58
  int8 v4; // stack - 0x48
  int8 v5; // stack - 0x40
  int8 v6; // stack - 0x38
  
  v1 = sub_29d10(v3);
  if (!(char)v1)
    return 3;
  v3[0] = 0xffffffff;
  v5 = a0;
  if (*(code **)(v6 + 0x28)) { // branch-flip
    v1 = (**(code **)(v6 + 0x28))();
    sub_23d40(*(int8 *)(v5 + 0x118) - v4);
    v2 = *(code **)(v6 + 8);
    if (!v2) {
      return (v1 ^ 1) & 0xff;
    }
  }
  else {
    sub_23d40(*(int8 *)(a0 + 0x118) - v4);
    v2 = *(code **)(v6 + 8);
    if (!v2)
      return 0;
  }
  if (!(*v2)(v3))
    return 1;
  return (v1 ^ 1) & 0xff;
}


// Function: sub_2b690 @ 0x2b690
unsigned int sub_2b690(unsigned int a0,int8 a1)
{
  int8 v1;
  int8 v10; // stack - 0x58
  int8 v11; // stack - 0x50
  int8 v12; // stack - 0x48
  char v2; // al
  unsigned int v3; // eax
  int8 *v4; // rbx
  unsigned int v5; // stack - 0x68
  uint8 v6;
  uint8 v7; // rbp
  uint8 v8; // rsi
  char v9; // stack - 0x64
  
  v3 = sub_29d10(&v5);
  if (!(char)v3)
    return v3;
  v11 = a1;
  v9 = 1;
  v5 = a0;
  if (*(code **)(v12 + 0x28)) { // branch-flip
    v2 = (**(code **)(v12 + 0x28))();
    sub_c670(a1);
    if (!v2) goto label_2b734;
  }
  else {
    sub_c670(a1);
  }
  v8 = 0;
  v6 = 0;
  do {
    if (*(uint8 *)(v11 + 0x130) <= v6) goto label_2b748;
    v1 = *(int8 *)(*(int8 *)(v11 + 0x140) + v6 * 0x10);
    if (!sub_2a9d0(&v5,v8,v1)) break;
    v7 = v6 + 1;
    v2 = sub_2ab20(&v5,v6);
    v4 = (int8 *)(v6 * 0x10 + *(int8 *)(v11 + 0x140));
    v8 = v4[1] + *v4;
    v6 = v7;
  } while (v2);
label_2b734:
  v3 = 0;
  sub_23d40(*(int8 *)(v11 + 0x118) - v10);
label_2b748:
  sub_c6a0();
  if (*(code **)(v12 + 8))
    (**(code **)(v12 + 8))(&v5);
  return v3;
}


// Function: sub_2b7d0 @ 0x2b7d0
int8 sub_2b7d0(char *a0,int8 *a1) // return-dupe x2
{
  char *v1; // rax
  uint8 v2; // rax
  int8 v3; // rbx
  void *v4;
  
  v1 = strrchr(a0,0x2e);
  if (!v1)
    return 0;
  v2 = strlen(&v1[1]);
  if (!dat_7f920)
    return 0;
  v3 = 0x7f920;
  v4 = dat_7f920;
  while ((*(uint8 *)(v3 + 8) != v2 || (memcmp(v4,&v1[1],v2)))) {
    v4 = *(void **)(v3 + 0x18);
    v3 += 0x18;
    if (!v4)
      return 0;
  }
  if (!a1)
    return v3;
  *a1 = (strlen(a0) - v2) + -1;
  return v3;
}


// Function: sub_2b880 @ 0x2b880
void sub_2b880(char *a0,int8 a1) // return-dupe
{
  int8 v1; // rax
  
  v1 = sub_2b7d0(a0,NULL);
  if (v1)
    a1 = *(int8 *)(v1 + 0x10);
  if (!a1)
    return;
  dat_82b70 = a1;
  return;
}


// Function: sub_2b8b0 @ 0x2b8b0
void * sub_2b8b0(char *a0)
{
  unsigned long *v1; // rax
  void *v2;
  uint8 v3; // stack - 0x38
  
  v1 = (unsigned long *)sub_2b7d0(a0,&v3);
  v2 = NULL;
  if (v1) {
    if (5 <= v3) { // branch-flip
      if ((!strncmp(&a0[v3 - 4],".tar",4)) && (*(char *)*v1 != 't'))
        v3 -= 4;
    }
    else if (!v3)
      return NULL;
    v2 = (void *)sub_4ecc0(v3 + 1);
    memcpy(v2,a0,v3);
    *(char *)((int8)v2 + v3) = 0;
  }
  return v2;
}


// Function: sub_2b970 @ 0x2b970
void sub_2b970(char *a0)
{
  char *v1; // stack - 0x38
  int8 v2; // fs_offset
  unsigned long v3; // stack - 0x30
  char *v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x10
  
  v6 = *(unsigned long *)(v2 + 0x28);
  v1 = "/bin/sh";
  v3 = 0x66997;
  v5 = 0;
  v4 = a0;
  execv("/bin/sh",&v1);
  sub_32da0(a0); // no-return
}


// Function: sub_2b9c0 @ 0x2b9c0
void sub_2b9c0(int4 a0)
{
  uint4 v1; // stack - 0x24
  uint4 v2; // edi
  int8 v3; // fs_offset
  unsigned long v4; // stack - 0x20
  
  v4 = *(unsigned long *)(v3 + 0x28);
  do {
    if (waitpid(a0,(int4 *)&v1,0) != -1) goto label_2b9fb;
  } while (*__errno_location() == 4);
  sub_334d0(dat_82b70);
label_2b9fb:
  v2 = v1 >> 8 & 0xff;
  if ('\x02' <= (char)(((uint1)v1 & 0x7f) + 1)) {
    raise(v1 & 0x7f);
    v2 = 0;
  }
  exit(v2); // no-return
}


// Function: sub_2ba30 @ 0x2ba30
void sub_2ba30(void)
{
  char *v1;
  unsigned long v10; // stack - 0xb8
  unsigned long v11; // stack - 0x40
  char *v2; // rax
  char *v3; // rax
  unsigned long v4; // rdx
  char v5 [4];
  int8 v6; // stack - 0x128
  int8 v7; // fs_offset
  char **v8; // stack - 0x120
  int8 v9; // stack - 0x118
  
  v11 = *(unsigned long *)(v7 + 0x28);
  v9 = 1;
  v10 = __environ;
  v1 = (char *)sub_c810(v5);
  if (!v1) {
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"unable to run decompression program",5));
    sub_16fd0(); // no-return
  }
  v4 = 0x2080e06;
  while( true ) {
    if (sub_375a0(v1,&v6,v4)) {
      if (dat_82d50)
        (*dat_82d50)();
      v3 = (char *)sub_358f0(&v6);
      error(0,0,dcgettext(NULL,"cannot split string \'%s\': %s",5),v1,v3);
      sub_16fd0(); // no-return
    }
    memmove(v8,&v8[v9],v6 * 8);
    v8[v6] = "-d";
    execvp(*v8,v8);
    v8[v6] = NULL;
    v3 = (char *)sub_c890(v5);
    if (!v3) break;
    if (dat_81b82 & 8) {
      if (dat_82d50)
        (*dat_82d50)();
      v2 = dcgettext(NULL,"cannot run %s",5);
      error(0,*__errno_location(),v2,v1);
      if (dat_81b82 & 8) {
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"trying %s",5),v3);
      }
    }
    v4 = 0x2080e0e;
    v1 = v3;
  }
  sub_32da0(v1); // no-return
}


// Function: sub_2bd50 @ 0x2bd50
unsigned int sub_2bd50(void)
{
  uint4 v1; // eax
  
  v1 = fstat(dat_82a34,(stat *)0x82740);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_2bd80 @ 0x2bd80
bool sub_2bd80(struct_53 *a0)
{
  if ((dat_82818) && (dat_82818 == a0->field_0x58))
    return a0->field_0x60 == dat_82810;
  return 0;
}


// Function: sub_2bdc0 @ 0x2bdc0
void sub_2bdc0(void)
{
  if ((dat_82a34 <= 0x3fffffff) && ((dat_82758 & 0xf000) == 0x8000)) {
    dat_82818 = dat_82740;
    dat_82810 = dat_82748;
    return;
  }
  dat_82818 = 0;
  return;
}


// Function: sub_2be10 @ 0x2be10
void sub_2be10(void) // return-dupe x2
{
  char *v1;
  stat v2; // stack - 0xa8
  
  v1 = (char *)*dat_82838;
  if (!strcmp(v1,"/dev/null")) {
    dat_82a30 = 1;
    return;
  }
  if ((((dat_82a34 <= 0x3fffffff) && ((dat_82758 & 0xf000) == 0x2000)) && (!stat("/dev/null",&v2))) && (dat_82740 == v2._0_8_)) {
    dat_82a30 = dat_82748 == v2._8_8_;
    return;
  }
  dat_82a30 = 0;
  return;
}


// Function: sub_2bee0 @ 0x2bee0
void sub_2bee0(int4 a0,bool a1) // return-dupe x2
{
  uint1 v1;
  char *v2; // rax
  uint8 v3; // rcx
  uint4 v4; // stack - 0x24
  
  if (!a0)
    return;
  do {
    if (waitpid(a0,(int4 *)&v4,0) != -1) {
      v1 = (uint1)v4;
      goto label_2bf3d;
    }
  } while (*__errno_location() == 4);
  sub_334d0(dat_82b70);
  v1 = (uint1)v4;
label_2bf3d:
  if ('\x02' <= (char)((v1 & 0x7f) + 1)) { // branch-flip
    if (((v4 & 0x7f) == 0xd) && (!a1))
      return;
    if (dat_82d50)
      (*dat_82d50)();
    v2 = dcgettext(NULL,"Child died with signal %d",5);
    v3 = (uint8)(v4 & 0x7f);
  }
  else {
    if (!(char)(v4 >> 8))
      return;
    if (dat_82d50)
      (*dat_82d50)();
    v3 = (uint8)(v4 >> 8) & 0xff;
    v2 = dcgettext(NULL,"Child returned status %d",5);
  }
  error(0,0,v2,v3);
  sub_16fd0(); // no-return
}


// Function: sub_2c010 @ 0x2c010
void sub_2c010(void)
{
  int4 v1; // eax
  char *v2; // rax
  int8 v3;
  
  v2 = getenv("SHELL");
  if (!v2)
    v2 = "/bin/sh";
  v1 = sub_26520();
  if (v1) {
    do {
      if (waitpid(v1,&v3,0) != -1)
        return;
    } while (*__errno_location() == 4);
    sub_334d0(v2);
    return;
  }
  execlp(v2,"-sh",0x669ed,0);
  sub_32da0(v2); // no-return
}


// Function: sub_2c0d0 @ 0x2c0d0
unsigned int sub_2c0d0(int8 a0,int8 a1)
{
  return CONCAT31((undefined3)((uint4)*(int4 *)(a1 + 0x1c) >> 8),*(int4 *)(a0 + 0x1c) == *(int4 *)(a1 + 0x1c));
}


// Function: sub_2c0e0 @ 0x2c0e0
unsigned int sub_2c0e0(int8 a0,int8 a1)
{
  return CONCAT31((undefined3)((uint4)*(int4 *)(a1 + 0x20) >> 8),*(int4 *)(a0 + 0x20) == *(int4 *)(a1 + 0x20));
}


// Function: sub_2c0f0 @ 0x2c0f0
unsigned long sub_2c0f0(struct_18 *a0,struct_18 *a1)
{
  if (a1->field_0x0 != a0->field_0x0)
    return 0;
  return CONCAT71((undefined7)((uint8)a0->field_0x8 >> 8),a1->field_0x8 == a0->field_0x8);
}


// Function: sub_2c110 @ 0x2c110
unsigned long sub_2c110(uint8 a0)
{
  int8 v1; // rax
  
  v1 = lseek(a0,0,1);
  if (0 <= v1)
    return ftruncate(a0 & 0xffffffff,v1); // tail-call
  return 0xffffffff;
}


// Function: sub_2c140 @ 0x2c140
uint8 sub_2c140(void)
{
  uint8 v1; // rax
  
  if (0x40000000 <= dat_82a34) {
    v1 = sub_345c0(dat_82a34 + -0x40000000); // tail-call
    return v1;
  }
  v1 = sub_431a0(dat_82a34,dat_81f58,dat_82b98); // tail-call
  return v1;
}


// Function: sub_2c180 @ 0x2c180
int4 sub_2c180(void)
{
  unsigned long *v1;
  int8 v10;
  unsigned int v11; // stack - 0x2c
  int4 v12; // stack - 0x24
  int4 v2;
  int8 v3; // rax
  uint8 v4;
  int4 *v5; // rax
  uint8 v6; // rbx
  int4 v7; // stack - 0x30
  int4 v8; // stack - 0x28
  char *v9;
  
  signal(0xd,1);
  sub_26560(&v7);
  v2 = sub_26520();
  if (0 < v2) {
    dat_82a34 = v11;
    sub_cd20(v7);
    return v2;
  }
  sub_4af40(dcgettext(NULL,"tar (child)",5));
  signal(0xd,0);
  if (v7)
    sub_2bc90(v7,0);
  sub_cd20(v11);
  v1 = dat_82838;
  if (dat_82d70) { // branch-flip
    v9 = (char *)*dat_82838;
label_2c237:
    if (sub_2bc20(v9)) {
      if (dat_82b84)
        sub_25c10((char *)*dat_82838,1);
      v9 = (char *)*dat_82838;
      if (strcmp(v9,"-")) {
        dat_82a34 = creat(v9,0x1b6);
        if (dat_82a34 < 0) {
          v5 = __errno_location();
          v2 = *v5;
          if (dat_82b84)
            sub_25ea0();
          *v5 = v2;
          sub_32ec0((char *)*dat_82838); // no-return
        }
        if (dat_82a34 != 1)
          sub_2bc90(dat_82a34,1);
      }
      sub_2b970(dat_82b70); // return-dupe, no-return
    }
  }
  else {
    dat_82d78 = strchr((char *)*dat_82838,0x3a);
    v9 = (char *)*v1;
    if (((!dat_82d78) || (dat_82d78 <= v9)) || (v4 = (int8)dat_82d78 - (int8)v9, memchr(v9,0x2f,v4))) goto label_2c237;
  }
  sub_26560(&v8);
  v2 = sub_26520();
  if (!v2) {
    sub_4af40(dcgettext(NULL,"tar (grandchild)",5));
    if (v12 != 1)
      sub_2bc90(v12,1);
    sub_cd20(v8);
    sub_2b970(dat_82b70);
  }
  if (v8)
    sub_2bc90(v8,0);
  sub_cd20(v12);
  v1 = dat_82838;
  v9 = (char *)*dat_82838;
  if (!strcmp(v9,"-")) {
    dat_82a34 = 1;
    goto label_2c364;
  }
  if (dat_82d70) { // branch-flip
label_2c4bd:
    dat_82a34 = creat(v9,0x1b6);
  }
  else {
    dat_82d78 = strchr(v9,0x3a);
    v9 = (char *)*v1;
    if (((!dat_82d78) || (dat_82d78 <= v9)) || (v4 = (int8)dat_82d78 - (int8)v9, memchr(v9,0x2f,v4))) goto label_2c4bd;
    dat_82a34 = sub_33a80(v9,0x41,0x40000000,dat_82ab0);
  }
  if (dat_82a34 <= -1)
    sub_32ec0((char *)*dat_82838); // no-return
label_2c364:
  v4 = dat_82b98;
  do {
    if (!v4)
      sub_2b9c0(v2); // return-dupe, no-return
    v6 = 0;
    v10 = dat_81f58;
    do {
      v3 = sub_4d570(0,v10,v4 - v6);
      if (v3 == -1)
        sub_33090(dat_82b70); // no-return
      if (!v3) {
        if (!v6) {
          sub_2b9c0(v2);
        }
        memset((void *)(dat_81f58 + v6),0,dat_82b98 - v6);
        v4 = sub_2c140();
        if (dat_82b98 != v4) {
          sub_da70(v4); // no-return, return-dupe
        }
        sub_2b9c0(v2);
      }
      v6 += v3;
      v10 += v3;
      v4 = dat_82b98;
    } while (v6 < dat_82b98);
    v4 = sub_2c140();
    if (dat_82b98 != v4)
      sub_da70(v4);
  } while( true );
}


// Function: sub_2c570 @ 0x2c570
int4 sub_2c570(void)
{
  unsigned long *v1;
  int4 v10; // stack - 0x34
  int4 v2; // eax
  uint8 v3;
  unsigned int v4; // stack - 0x40
  int4 v5; // stack - 0x38
  uint8 v6;
  char *v7;
  int8 v8;
  int4 v9; // stack - 0x3c
  
  sub_26560(&v4);
  v2 = sub_26520();
  if (0 < v2) {
    dat_82a34 = v4;
    sub_cd20(v9);
    return v2;
  }
  sub_4af40(dcgettext(NULL,"tar (child)",5));
  signal(0xd,0);
  if (v9 != 1)
    sub_2bc90(v9,1);
  sub_cd20(v4);
  v1 = dat_82838;
  v7 = (char *)*dat_82838;
  if (strcmp(v7,"-")) {
    if (!dat_82d70) {
      dat_82d78 = strchr(v7,0x3a);
      v7 = (char *)*v1;
      if (((dat_82d78) && (v7 < dat_82d78)) && (v3 = (int8)dat_82d78 - (int8)v7, !memchr(v7,0x2f,v3))) goto label_2c658;
    }
    if (sub_2bc20(v7)) {
      dat_82a34 = open((char *)*dat_82838,0,0x1b6);
      if (0 <= dat_82a34) {
        if (dat_82a34)
          sub_2bc90(dat_82a34,0);
        sub_2ba30(); // no-return, return-dupe
      }
      sub_32ec0((char *)*dat_82838); // return-dupe, no-return
    }
  }
label_2c658:
  sub_26560(&v5);
  v2 = sub_26520();
  if (!v2) {
    sub_4af40(dcgettext(NULL,"tar (grandchild)",5));
    if (v5)
      sub_2bc90(v5,0);
    sub_cd20(v10);
    sub_2ba30();
  }
  if (v10 != 1)
    sub_2bc90(v10,1);
  sub_cd20(v5);
  v1 = dat_82838;
  v7 = (char *)*dat_82838;
  if (!strcmp(v7,"-")) {
    dat_82a34 = 0;
    goto label_2c6b8;
  }
  if (dat_82d70) { // branch-flip
label_2c7f4:
    dat_82a34 = open(v7,0,0x1b6);
  }
  else {
    dat_82d78 = strchr(v7,0x3a);
    v7 = (char *)*v1;
    if (((!dat_82d78) || (dat_82d78 <= v7)) || (v3 = (int8)dat_82d78 - (int8)v7, memchr(v7,0x2f,v3))) goto label_2c7f4;
    dat_82a34 = sub_33a80(v7,0,0x40000000,dat_82ab0);
  }
  if (dat_82a34 <= -1)
    sub_32ec0((char *)*dat_82838);
label_2c6b8:
  while( true ) {
    sub_c730();
    while( true ) {
      if (0x40000000 <= dat_82a34) { // branch-flip
        v3 = sub_34460(dat_82a34 + -0x40000000,dat_81f58,dat_82b98);
        v8 = dat_81f58;
      }
      else {
        v3 = sub_4d570(dat_82a34,dat_81f58,dat_82b98);
        v8 = dat_81f58;
      }
      dat_81f58 = v8;
      if (v3 != 0xffffffffffffffff) break;
      sub_cd60();
    }
    if (!v3) break;
    do {
      v6 = 0x200;
      if (v3 <= 0x200)
        v6 = v3;
      if (sub_431a0(1,v8,v6) != v6)
        sub_334f0(dat_82b70);
      v8 += v6;
      v3 -= v6;
    } while (v3);
  }
  sub_cd20(1);
  sub_2b9c0(v2); // no-return
}


// Function: sub_2c8b0 @ 0x2c8b0
unsigned int sub_2c8b0(char *a0,char a1,int8 a2)
{
  char *v1;
  char v10 [32];
  char v11 [32];
  char v12 [32];
  char v13 [32];
  int4 v14;
  unsigned int v15; // stack - 0x1a0
  int4 v2; // stack - 0x1a4
  char v3 [32];
  char v4 [34]; // stack - 0x19a
  char v5 [32];
  char v6 [40];
  char v7 [32];
  char v8 [32];
  char v9 [32];
  
  sub_26560(&v2);
  dat_82728 = signal(0xd,1);
  dat_82730 = sub_26520();
  if (dat_82730) {
    sub_cd20(v2);
    return v15;
  }
  if (v2)
    sub_2bc90(v2,0);
  sub_cd20(v15);
  if (setenv("TAR_VERSION","1.34",1)) {
    sub_17020(); // no-return, return-dupe
  }
  v1 = (char *)*dat_82828;
  if (v1) { // branch-flip
    if (setenv("TAR_ARCHIVE",v1,1)) {
      sub_17020();
    }
  }
  else {
    unsetenv("TAR_ARCHIVE");
  }
  v1 = (char *)sub_45370(((int8)dat_82828 - dat_82838 >> 3) + 1,v3);
  if (setenv("TAR_VOLUME",v1,1)) {
    sub_17020();
  }
  v1 = (char *)sub_45370((int8)dat_82ba0,&v4[2]);
  if (setenv("TAR_BLOCKING_FACTOR",v1,1)) {
    sub_17020();
  }
  v14 = dat_82400;
  if (!dat_82400)
    v14 = dat_82ba4;
  v1 = (char *)sub_2dce0(v14);
  if (v1) { // branch-flip
    if (setenv("TAR_FORMAT",v1,1)) {
      sub_17020();
    }
  }
  else {
    unsetenv("TAR_FORMAT");
  }
  v4[1] = 0;
  v4[0] = a1;
  if (setenv("TAR_FILETYPE",v4,1)) {
    sub_17020();
  }
  __snprintf_chk(v5,0x18,1,0x18,"0%lo",(uint8)*(uint4 *)(a2 + 0x70));
  if (setenv("TAR_MODE",v5,1)) {
    sub_17020();
  }
  if (a0) { // branch-flip
    if (setenv("TAR_FILENAME",a0,1)) {
      sub_17020();
    }
  }
  else {
    unsetenv("TAR_FILENAME");
  }
  v1 = *(char **)(a2 + 8);
  if (v1) { // branch-flip
    if (setenv("TAR_REALNAME",v1,1)) {
      sub_17020();
    }
  }
  else {
    unsetenv("TAR_REALNAME");
  }
  v1 = *(char **)(a2 + 0x20);
  if (v1) { // branch-flip
    if (setenv("TAR_UNAME",v1,1)) {
      sub_17020();
    }
  }
  else {
    unsetenv("TAR_UNAME");
  }
  v1 = *(char **)(a2 + 0x28);
  if (v1) { // branch-flip
    if (setenv("TAR_GNAME",v1,1)) {
      sub_17020();
    }
  }
  else {
    unsetenv("TAR_GNAME");
  }
  v1 = (char *)sub_257a0(*(uint8 *)(a2 + 0xe8),*(unsigned long *)(a2 + 0xf0),v6);
  if (setenv("TAR_ATIME",v1,1)) {
    sub_17020();
  }
  v1 = (char *)sub_257a0(*(uint8 *)(a2 + 0xf8),*(unsigned long *)(a2 + 0x100),v7);
  if (setenv("TAR_MTIME",v1,1)) {
    sub_17020();
  }
  v1 = (char *)sub_257a0(*(uint8 *)(a2 + 0x108),*(unsigned long *)(a2 + 0x110),v8);
  if (setenv("TAR_CTIME",v1,1)) {
    sub_17020();
  }
  v1 = (char *)sub_45370(*(uint8 *)(a2 + 0x88),v9);
  if (setenv("TAR_SIZE",v1,1)) {
    sub_17020();
  }
  v1 = (char *)sub_45370(*(unsigned int *)(a2 + 0x74),v10);
  if (setenv("TAR_UID",v1,1)) {
    sub_17020();
  }
  v1 = (char *)sub_45370(*(unsigned int *)(a2 + 0x78),v11);
  if (setenv("TAR_GID",v1,1)) {
    sub_17020();
  }
  if (a1 != 'h') { // branch-flip
    if ('i' <= a1) { // branch-flip
      if (a1 == 'l') goto label_2cd62;
    }
    else if ((uint1)(a1 + 0x9eU) <= 1) {
      v1 = (char *)sub_45370((uint4)((*(uint8 *)(a2 + 0x80) >> 0x14) << 8) | (uint4)*(uint8 *)(a2 + 0x80) & 0xff,v12);
      if (setenv("TAR_MINOR",v1,1)) {
        sub_17020();
      }
      v1 = (char *)sub_45370((uint4)((uint8)*(unsigned long *)(a2 + 0x80) >> 0x20) & 0xfffff000 | (uint4)((uint8)*(unsigned long *)(a2 + 0x80) >> 8) & 0xfff,v13);
      if (setenv("TAR_MAJOR",v1,1)) {
        sub_17020();
      }
      goto label_2cd14;
    }
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
  }
  else {
label_2cd62:
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    v1 = *(char **)(a2 + 0x18);
    if (v1) {
      if (setenv("TAR_LINKNAME",v1,1))
        sub_17020();
      sub_2b970(dat_82a60); // return-dupe, no-return
    }
  }
label_2cd14:
  unsetenv("TAR_LINKNAME");
  sub_2b970(dat_82a60);
}


// Function: sub_2ce00 @ 0x2ce00
void sub_2ce00(void) // return-dupe
{
  uint4 v1;
  uint4 v2; // stack - 0x24
  int4 v3;
  uint8 v4; // r12
  
  v3 = dat_82730;
  if (dat_82730 <= -1)
    return;
  signal(0xd,dat_82728);
  while( true ) {
    if (waitpid(v3,(int4 *)&v2,0) != -1) {
      v1 = v2 & 0x7f;
      if (v1) { // branch-flip
        if ('\x02' <= (char)((char)v1 + '\x01')) { // branch-flip
          if (dat_82d50) {
            (*dat_82d50)();
            v1 = v2 & 0x7f;
          }
          v4 = (uint8)dat_82730;
          error(0,0,dcgettext(NULL,"%lu: Child terminated on signal %d",5),v4,v1);
        }
        else {
          if (dat_82d50)
            (*dat_82d50)();
          v4 = (uint8)dat_82730;
          error(0,0,dcgettext(NULL,"%lu: Child terminated on unknown reason",5),v4);
          dat_82d58 = 2;
        }
      }
      else if ((!dat_82a59) && (v1 = v2 >> 8, (char)(v2 >> 8))) {
        if (dat_82d50) {
          (*dat_82d50)();
          v1 = v2 >> 8;
        }
        v4 = (uint8)dat_82730;
        error(0,0,dcgettext(NULL,"%lu: Child returned status %d",5),v4,v1 & 0xff);
        dat_82d58 = 2;
      }
      dat_82730 = 0xffffffff;
      return;
    }
    if (*__errno_location() != 4) break;
    v3 = dat_82730;
  }
  dat_82730 = 0xffffffff;
  sub_334d0(dat_82a60);
  return;
}


// Function: sub_2cfc0 @ 0x2cfc0
uint4 sub_2cfc0(unsigned long *a0,int4 a1)
{
  int4 v1;
  int4 v10; // stack - 0x4c
  FILE *v2; // rax
  int4 v3; // stack - 0x50
  char *v4; // stack - 0x60
  unsigned long v5; // stack - 0x58
  uint4 v6; // stack - 0x64
  char v7 [24];
  uint8 v8; // rdi
  int4 v9; // eax
  
  sub_26560(&v3);
  dat_82720 = signal(0xd,1);
  v1 = sub_26520();
  if (!v1) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",(char *)*a0,1);
    setenv("TAR_VOLUME",(char *)sub_45370((int8)a1,v7),1);
    v8 = (uint8)dat_82ba0;
    setenv("TAR_BLOCKING_FACTOR",(char *)sub_45370(v8,v7),1);
    setenv("TAR_SUBCOMMAND",(char *)sub_2dd30(dat_82ba8),1);
    v1 = dat_82400;
    if (!dat_82400)
      v1 = dat_82ba4;
    setenv("TAR_FORMAT",(char *)sub_2dce0(v1),1);
    setenv("TAR_FD",(char *)sub_45370((int8)v10,v7),1);
    sub_cd20(v3);
    sub_2b970(dat_82b48); // no-return
  }
  v4 = NULL;
  v5 = 0;
  sub_cd20(v10);
  v2 = fdopen(v3,"r");
  v9 = (int4)__getdelim(&v4,&v5,10,v2);
  fclose(v2);
  if ((1 <= v9) && (v4[(int8)v9 + -1] == '\n')) {
    v4[(int8)v9 + -1] = '\0';
    v9 -= 1;
  }
  while (waitpid(v1,(int4 *)&v6,0) == -1) {
    if (*__errno_location() != 4) {
      signal(0xd,dat_82720);
      sub_334d0(dat_82b48);
      return 0xffffffff;
    }
  }
  signal(0xd,dat_82720);
  if (v6 & 0x7f) {
    free(v4);
    return 0xffffffff;
  }
  if ((!(char)(v6 >> 8)) && (1 <= v9)) {
    *a0 = v4;
    return 0;
  }
  free(v4);
  return v6 >> 8 & 0xff;
}


// Function: sub_2d260 @ 0x2d260
void sub_2d260(char *a0,char *a1,int4 a2)
{
  int4 v1;
  int8 v2;
  char v3 [24];
  uint8 v4; // rdi
  
  v1 = sub_26520();
  if (!v1) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",a1,1);
    setenv("TAR_CHECKPOINT",(char *)sub_45370((int8)a2,v3),1);
    v4 = (uint8)dat_82ba0;
    setenv("TAR_BLOCKING_FACTOR",(char *)sub_45370(v4,v3),1);
    setenv("TAR_SUBCOMMAND",(char *)sub_2dd30(dat_82ba8),1);
    v1 = dat_82400;
    if (!dat_82400)
      v1 = dat_82ba4;
    setenv("TAR_FORMAT",(char *)sub_2dce0(v1),1);
    sub_2b970(a0); // no-return
  }
  do {
    if (waitpid(v1,&v2,0) != -1)
      return;
  } while (*__errno_location() == 4);
  sub_334d0(a0);
  return;
}


// Function: sub_2d3c0 @ 0x2d3c0
int8 * sub_2d3c0(int8 *a0,int4 a1)
{
  for (; (((*a0 || ((int4)a0[1])) || (a0[2])) || (((int4)a0[3] || (a0[4])))); a0 = &a0[6]) {
    if (a1 == (int4)a0[1])
      return a0;
  }
  return NULL;
}


// Function: sub_2d400 @ 0x2d400
void sub_2d400(void)
{
  int8 v1; // rax
  int8 v2;
  char *v3;
  
  v3 = "oldgnu";
  v2 = 0x7da00;
  while (v1 = v2 + 0x10, v3) {
    if (*(int4 *)(v2 + 0x18) == 6) {
      sub_4f3c0("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",v3,0x6b716,0x14,"escape","/usr/local/libexec/rmt","/usr/bin/rsh"); // return-dupe
      return;
    }
    v3 = *(char **)(v2 + 0x20);
    v2 = v1;
  }
  v3 = "unknown?";
  sub_4f3c0("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",v3,0x6b716,0x14,"escape","/usr/local/libexec/rmt","/usr/bin/rsh");
  return;
}


// Function: sub_2d480 @ 0x2d480
unsigned long sub_2d480(uint4 a0,struct_80 *a1)
{
  unsigned int v1;
  unsigned long v2;
  unsigned int *v3; // rax
  char *v4;
  uint8 v5;
  
  v5 = 0x20;
  v4 = a1->field_0x8;
  if (v4)
    v5 = strlen(v4) + 0x21;
  v3 = (unsigned int *)sub_4ecc0(v5);
  v4 = NULL;
  if (a1->field_0x8)
    v4 = strcpy((char *)&v3[8],a1->field_0x8);
  v1 = a1->field_0x0;
  *(char **)&v3[2] = v4;
  *v3 = v1;
  *(unsigned long *)&v3[4] = a1->field_0x10;
  v2 = *(unsigned long *)((uint8)a0 * 8 + 0x82bc0);
  *(unsigned int **)((uint8)a0 * 8 + 0x82bc0) = v3;
  *(unsigned long *)&v3[6] = v2;
  return v2;
}


// Function: sub_2d500 @ 0x2d500
void sub_2d500(unsigned int a0)
{
  sub_c770();
  sub_cc30();
  signal(a0,sub_2d500); // tail-call
  return;
}


// Function: sub_2d670 @ 0x2d670
unsigned long sub_2d670(int4 a0,unsigned long a1)
{
  uint1 v1;
  char *v10; // rsi
  int8 *v11; // r12
  int8 *v12;
  unsigned short *v13; // stack - 0x80
  char *v14; // stack - 0x78
  uint8 v15; // stack - 0x68
  uint1 v16; // stack - 0x48
  char *v2;
  uint8 v3; // rax
  uint8 v4; // rax
  char *v5; // rax
  undefined8 v6;
  char *v7;
  unsigned short *v8;
  char v9 [24]; // stack - 0x98
  
  if (a0 != 0x94) { // branch-flip
    switch(a0) { // branch-flip
      default:
        return a1;
      case 0x4a:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "xz";
        break;
      case 0x5a:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "compress";
        break;
      case 0x6a:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "bzip2";
        break;
      case 0x7a:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "gzip";
        break;
      case 0x95:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "lzma";
        break;
      case 0x96:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "lzop";
        break;
      case 0xcf:
        v2 = dcgettext(NULL,"filter the archive through %s",5);
        v10 = "zstd";
        break;
      case 0x2000004:
        _obstack_begin((obstack *)v9,0,0,sub_4ecc0,dat_7efc8);
        v2 = dcgettext(NULL,"Valid arguments for the --quoting-style option are:",5);
        v3 = strlen(v2);
        if ((uint8)((int8)v14 - (int8)v13) < v3)
          _obstack_newchunk((obstack *)v9,v3);
        memcpy(v13,v2,v3);
        v8 = (unsigned short *)((int8)v13 + v3);
        if ((uint8)((int8)v14 - (int8)v8) < 2) {
          _obstack_newchunk((obstack *)v9,2);
          v8 = v13;
        }
        *v8 = 0xa0a;
        v8 = &v13[1];
        v3 = strlen("  ");
        v2 = "literal";
        v12 = (int8 *)0x7e6e8;
        while( true ) {
          if ((uint8)((int8)v14 - (int8)v8) < v3) {
            _obstack_newchunk((obstack *)v9,v3);
            v8 = v13;
          }
          memcpy(v8,(void *)0x6aab9,v3);
          v8 = (char *)((int8)v13 + v3);
          v4 = strlen(v2);
          if ((uint8)((int8)v14 - (int8)v8) < v4) {
            _obstack_newchunk((obstack *)v9,v4);
            v8 = v13;
          }
          memcpy(v8,v2,v4);
          v13 = (char *)((int8)v13 + v4);
          if (v13 == (unsigned short *)v14)
            _obstack_newchunk((obstack *)v9,1);
          v8 = (unsigned short *)((int8)v13 + 1);
          v11 = &v12[1];
          *(char *)v13 = 10;
          v2 = (char *)*v12;
          if (!v2) break;
          v12 = v11;
        }
        v2 = dcgettext(NULL,"\n*This* tar defaults to:\n",5);
        v3 = strlen(v2);
        if ((uint8)((int8)v14 - (int8)v13) < v3)
          _obstack_newchunk((obstack *)v9,v3);
        memcpy(v13,v2,v3);
        v7 = (char *)((int8)v13 + v3);
        v2 = (char *)sub_2d400();
        v3 = strlen(v2);
        if ((uint8)((int8)v14 - (int8)v13) < v3)
          _obstack_newchunk((obstack *)v9,v3);
        memcpy(v13,v2,v3);
        v13 = (char *)((int8)v13 + v3);
        if (v13 == (unsigned short *)v14)
          _obstack_newchunk((obstack *)v9,1);
        v7 = (char *)((int8)v13 + 1);
        *(char *)v13 = 10;
        v13 = (unsigned short *)v7;
        if (v14 == v7)
          _obstack_newchunk((obstack *)v9,1);
        v6 = v9._16_8_;
        *(char *)v13 = 0;
        if ((char *)((int8)v13 + 1) == v9._16_8_)
          v1 = v16 | 2;
        v5 = (char *)((uint8)&((char *)((int8)v13 + 1))[v15] & ~v15);
        v7 = v14;
        if ((uint8)((int8)v5 - v9._8_8_) <= (uint8)((int8)v14 - v9._8_8_))
          v7 = v5;
        v9._16_8_ = v7;
        v6 = sub_4ee30(v6);
        _obstack_free((obstack *)v9,NULL);
        return v6;
      
    }
  }
  else {
    v2 = dcgettext(NULL,"filter the archive through %s",5);
    v10 = "lzip";
  }
  v6 = sub_4f3c0(v2,v10); // tail-call
  return v6;
}


// Function: sub_2dd30 @ 0x2dd30
char * sub_2dd30(unsigned int a0)
{
  switch(a0) {
    case 0:
      return "unknown?";
    case 1:
      return "-r";
    case 2:
      return "-A";
    case 3:
      return "-c";
    case 4:
      return "-D";
    case 5:
      return "-d";
    case 6:
      return "-x";
    case 7:
      return "-t";
    case 8:
      return "-u";
    case 9:
      return "--test-label";
    default:
      abort(); // no-return
    
  }
}


// Function: sub_2ddf0 @ 0x2ddf0
int4 sub_2ddf0(char *a0)
{
  int4 v1; // eax
  unsigned long *v2;
  char *v3;
  
  v2 = (unsigned long *)0x7d940;
  v1 = strncmp(a0,"SIG",3);
  v3 = &a0[3];
  if (v1)
    v3 = a0;
  v1 = strcmp("USR1",v3);
  while( true ) {
    if (!v1) {
      v1 = *(unsigned int *)&v2[1];
      return v1;
    }
    v2 = &v2[2];
    if (v2 == (unsigned long *)0x7d990) break;
    v1 = strcmp((char *)*v2,v3);
  }
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"Unknown signal name: %s",5),a0);
  sub_16fd0(); // no-return
}


// Function: sub_2dea0 @ 0x2dea0
void sub_2dea0(int4 a0)
{
  argp_help(0x7fb20,stderr,4,dat_82dd0);
  sub_40180();
  exit(a0); // no-return
}


// Function: sub_2df50 @ 0x2df50
bool sub_2df50(char *a0,char *a1)
{
  char *v1;
  int4 v2;
  int4 v3; // eax
  unsigned long v4; // stack - 0x28
  char *v5; // stack - 0x30
  bool v6;
  
  if (!dat_82bb0) {
    if ((dat_82a34) && (!dat_82c10)) {
      dat_82c10 = 0x66c79;
      dat_82bb0 = stdin;
    }
    else {
      dat_82bb0 = fopen("/dev/tty","r");
      if (!dat_82bb0)
        sub_32ec0("/dev/tty"); // no-return
    }
  }
  __fprintf_chk(dat_81f28,1,"%s %s?",a0,(char *)sub_4d2d0(a1));
  fflush_unlocked(dat_81f28);
  if (dat_82bac) // branch-flip
    v6 = 0;
  else {
    v5 = NULL;
    v4 = 0;
    if (0 <= __getdelim(&v5,&v4,10,dat_82bb0)) { // branch-flip
      v3 = rpmatch(v5);
      v2 = dat_82bac;
      v6 = 0 < v3;
      free(v5);
      if (!v2)
        return v6;
    }
    else {
      dat_82bac = 1;
      free(v5);
      v6 = 0;
    }
  }
  v1 = *(char **)&dat_81f28->field_0x28;
  if (*(char **)&dat_81f28->field_0x30 <= v1) // branch-flip
    __overflow(dat_81f28,10);
  else {
    *(char **)&dat_81f28->field_0x28 = &v1[1];
    *v1 = 10;
  }
  fflush_unlocked(dat_81f28);
  return v6;
}


// Function: sub_2e120 @ 0x2e120
unsigned long sub_2e120(int8 a0,unsigned long a1,char *a2,struct_82 *a3) // early-return x2
{
  unsigned long v1;
  int8 v2;
  int4 v3; // eax
  char *v4; // rax
  char *v5; // rax
  unsigned long *v6; // rax
  stat v7; // stack - 0xc8
  
  if (2 <= (uint1)(*a2 - 0x2eU)) {
    if (!sub_4aed0(a3,a2,NULL)) {
      if (dat_82d50)
        (*dat_82d50)();
      v4 = (char *)sub_4d2d0(a2);
      v5 = (char *)sub_23240(a3->field_0x0,a3->field_0x8,0);
      error(0,0,dcgettext(NULL,"Substituting %s for unknown date format %s",5),v5,v4);
      a3->field_0x8 = 0;
      return 1;
    }
    v6 = (unsigned long *)sub_4ecc0(0x28);
    v2 = a3->field_0x0;
    v1 = a3->field_0x8;
    v6[3] = a1;
    v6[1] = v2;
    v6[2] = v1;
    v6[4] = sub_4ee30(a2);
    v1 = *(unsigned long *)(a0 + 8);
    *(unsigned long **)(a0 + 8) = v6;
    *v6 = v1;
    return 0;
  }
  v3 = stat(a2,&v7);
  if (!v3) {
    a3->field_0x0 = v7._88_8_;
    a3->field_0x8 = v7._96_8_;
    return CONCAT44(dat_4,v3);
  }
  sub_33410(a2);
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"Date sample file not found",5));
  sub_2dea0(2); // no-return
}


// Function: sub_2e2f0 @ 0x2e2f0
void sub_2e2f0(char *a0,char *a1)
{
  if (dat_82d50)
    (*dat_82d50)();
  error(0,0,dcgettext(NULL,"\'%s\' cannot be used with \'%s\'",5),a0,a1);
  sub_2dea0(2); // no-return
}


// Function: sub_2e340 @ 0x2e340
void sub_2e340(uint4 a0,struct_80 *a1)
{
  char *v1;
  char *v2;
  struct_51 *v3; // rax
  
  v3 = (struct_51 *)sub_2d480(9,a1);
  if ((v3) && (a1->field_0x0 == v3->field_0x0)) {
    if ((a1->field_0x0 != 1) && (v1 = v3->field_0x8, v2 = a1->field_0x8, strcmp(v2,v1))) {
      dat_82b34 = a0;
      return;
    }
    if (dat_82b34 != a0)
      sub_2e2f0(*(char **)((int8)(int4)a0 * 8 + 0x7d900),*(char **)((uint8)dat_82b34 * 8 + 0x7d900)); // no-return
  }
  dat_82b34 = a0;
  return;
}


// Function: sub_2e4c0 @ 0x2e4c0
int4 sub_2e4c0(int4 a0,char *a1,struct_85 *a2) // switch-return x114, return-dupe x3
{
  uint1 v1;
  uint8 v10; // rdx
  uint8 v11;
  char v12 [24]; // stack - 0xc8
  uint8 v13; // stack - 0xd8
  char v14 [40];
  uint8 v15; // rax
  char *v16;
  unsigned long *v17;
  void *v18; // stack - 0xf8
  unsigned long v19; // stack - 0xd0
  int4 *v2;
  char *v20; // stack - 0xb0
  char *v21; // stack - 0xa8
  uint8 v22; // stack - 0x98
  uint1 v23; // stack - 0x78
  undefined8 v3;
  char v4;
  int8 v5;
  void *v6; // rax
  int8 *v7;
  char *v8; // rax
  int8 v9; // rdx
  
  v17 = a2->field_0x28;
  if (0xcf < a0) {
    if (a0 != 0x1000003) {
      if (a0 != 0x1000005)
        return 7;
      v2 = (int4 *)*v17;
      if (*v2 != 2) { // branch-flip
        if (!*v2) {
          v16 = *(char **)&v2[2];
          error(0,0,dcgettext(NULL,"error parsing %s",5),v16);
        }
      }
      else {
        v11 = *(uint8 *)&v2[4];
        v16 = *(char **)&v2[2];
        error(0,0,dcgettext(NULL,"%s:%lu: location of the error",5),v16,v11);
      }
      exit(0x40); // no-return
    }
    v7 = *(int8 **)(a2->field_0x0 + 0x20);
    if (v7) {
      if (*v7) {
        v9 = 0x20;
        v5 = 0;
        while( true ) {
          *(unsigned long **)(a2->field_0x30 + v5) = v17;
          v5 += 8;
          v7 = (int8 *)(*(int8 *)(a2->field_0x0 + 0x20) + v9);
          v9 += 0x20;
          if (!*v7) break;
          v17 = a2->field_0x28;
        }
        return 0;
      }
      return 0;
    }
    return 0;
  }
  if (a0 < 0)
    return 7;
  switch(a0) {
    case 0:
      sub_28130(a1);
      return a0;
    default:
      return 7;
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
      argp_error(a2,dcgettext(NULL,"Options \'-[0-7][lmh]\' not supported by *this* tar",5));
      exit(0x40); // no-return
    case 0x41:
      sub_2e2a0(2);
      return 0;
    case 0x42:
      dat_82ab9 = 1;
      return 0;
    case 0x46:
      dat_82b10 = 1;
      dat_82b48 = a1;
      return 0;
    case 0x47:
      dat_82b50 = 1;
      return 0;
    case 0x48:
      sub_2e430(a1);
      return 0;
    case 0x49:
      sub_2e3b0(a1,(struct_80 *)*v17);
      return 0;
    case 0x4a:
      sub_2e3b0("xz",(struct_80 *)*v17);
      return 0;
    case 0x4b:
      sub_2d480(5,(struct_80 *)*v17);
      sub_286f0(a1);
      return 0;
    case 0x4c:
      if (!sub_4eec0(a1,v12,10,&v13,"bBcGgkKMmPTtw")) {
        if ((a1 < (uint8)v12._0_8_) && (v4 = *(char *)(v12._0_8_ + -1), !strchr("bBcGgkKMmPTtw",(int4)v4))) {
          dat_82a70 = (float8)v13 * dat_69d60;
          dat_82b10 = 1;
          return 0;
        }
        if (0 > (int8)v13) {
          dat_82a70 = (float8)v13;
          dat_82b10 = 1;
          return 0;
        }
        dat_82a70 = (float8)(int8)v13;
        dat_82b10 = 1;
        return 0;
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid tape length";
      break;
    case 0x4d:
      dat_82b10 = 1;
      return 0;
    case 0x4e:
      dat_82b8c = 1;
      if (dat_82b08 <= -1) {
        v16 = "--after-date";
        sub_2e120(v17,v16,a1,(struct_82 *)0x82b00); // return-dupe
        sub_2d480(3,(struct_80 *)*v17);
        return 0;
      }
      goto label_2f8e5;
    case 0x4f:
      dat_82a6a = 1;
      return 0;
    case 0x50:
      sub_2d480(8,(struct_80 *)*v17);
      dat_82b92 = 1;
      return 0;
    case 0x52:
      dat_82b7c = 1;
      return 0;
    case 0x53:
      dat_82a88 = 1;
      return 0;
    case 0x55:
      sub_2e340(3,(struct_80 *)*v17);
      return 0;
    case 0x56:
      dat_82a40 = a1;
      return 0;
    case 0x57:
      sub_2d480(4,(struct_80 *)*v17);
      dat_82a50 = 1;
      return 0;
    case 0x5a:
      sub_2e3b0("compress",(struct_80 *)*v17);
      return 0;
    case 0x61:
      *(char *)((int8)v17 + 0x12) = 1;
      return 0;
    case 0x62:
      if ((((!sub_4eec0(a1,NULL,10,v12,"")) && (dat_82ba0 = (int4)v12._0_8_, 1 <= dat_82ba0)) && ((char *)(int8)dat_82ba0 == (char *)v12._0_8_)) && (dat_82b98 = (int8)(v12._0_8_ << 9), (char *)v12._0_8_ == (char *)(v12._0_8_ & 0x7fffffffffffff)))
        return 0;
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid blocking factor";
      break;
    case 99:
      sub_2e2a0(3);
      return 0;
    case 100:
      sub_2e2a0(5);
      return 0;
    case 0x66:
      v5 = dat_82838;
      if (dat_82830 == dat_82c18) {
        if (dat_82838) { // branch-flip
          if (0xaaaaaaaaaaaaaa9 < dat_82c18)
            sub_17020(); // no-return, return-dupe
          dat_82c18 = dat_82c18 + 1 + (dat_82c18 >> 1);
        }
        else if (dat_82c18) { // branch-flip
          if ((SUB168(ZEXT816(8) * ZEXT816(dat_82c18),0) < 0) || (SUB168(ZEXT816(8) * ZEXT816(dat_82c18),8))) {
            sub_17020();
          }
        }
        else {
          dat_82c18 = 0x10;
        }
        v5 = sub_4ece0(dat_82838,dat_82c18 * 8);
        dat_82838 = v5;
      }
      *(char **)(v5 + dat_82830 * 8) = a1;
      dat_82830 += 1;
      return 0;
    case 0x67:
      sub_2d480(2,(struct_80 *)*v17);
      dat_82b28 = a1;
      dat_82b50 = 1;
      dat_82b8c = 1;
      return 0;
    case 0x68:
      dat_82b69 = 1;
      return 0;
    case 0x69:
      dat_82b51 = 1;
      return 0;
    case 0x6a:
      sub_2e3b0("bzip2",(struct_80 *)*v17);
      return 0;
    case 0x6b:
      sub_2e340(4,(struct_80 *)*v17);
      return 0;
    case 0x6c:
      dat_82c20 = 1;
      return 0;
    case 0x6d:
      dat_82a68 = 1;
      return 0;
    case 0x6e:
      dat_827fc = 1;
      return 0;
    case 0x6f:
      *(char *)&v17[2] = 1;
      return 0;
    case 0x70:
      dat_82aa4 = 1;
      return 0;
    case 0x72:
      sub_2e2a0(1);
      return 0;
    case 0x73:
      sub_2d480(6,(struct_80 *)*v17);
      dat_82aac = 1;
      return 0;
    case 0x74:
      sub_2e2a0(7);
      dat_82a54 += 1;
      return 0;
    case 0x75:
      sub_2e2a0(8);
      return 0;
    case 0x76:
      dat_82a54 += 1;
      dat_81b80 |= 0x583000;
      return 0;
    case 0x77:
      dat_82b40 = 1;
      return 0;
    case 0x78:
      sub_2e2a0(6);
      return 0;
    case 0x7a:
      sub_2e3b0("gzip",(struct_80 *)*v17);
      return 0;
    case 0x80:
      sub_2e430("posix");
      dat_82a9c = 1;
      return 0;
    case 0x81:
      if (a1) {
        v5 = sub_3a4e0("--atime-preserve",a1,(int8 *)0x7d9e0,(void *)0x69a90,4,dat_81dd8);
        dat_82b88 = *(unsigned int *)(v5 * 4 + 0x69a90);
        return 0;
      }
      dat_82b88 = 1;
      return 0;
    case 0x82:
      if (a1) {
        v17[4] = a1;
        dat_82b84 = 1;
        return 0;
      }
      dat_82b84 = 1;
      return 0;
    case 0x83:
      dat_82b20 = 1;
      return 0;
    case 0x84:
      if (!a1) {
        dat_82b78 = 10;
        return 0;
      }
      if (*a1 == '.') {
        a1 = &a1[1];
        sub_10710(".");
      }
      dat_82b78 = (unsigned int)strtoul(a1,(char **)v12,0);
      if (!*(char *)v12._0_8_)
        return 0;
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "--checkpoint value is not an integer";
      goto label_2f9a2;
    case 0x85:
      sub_10710(a1);
      return 0;
    case 0x86:
      dat_82af0 = 2;
      return 0;
    case 0x87:
      dat_827e0 = 1;
      return 0;
    case 0x88:
      dat_82b68 = 1;
      return 0;
    case 0x89:
      sub_2e2a0(4);
      return 0;
    case 0x8a:
      dat_82d70 = 1;
      return 0;
    case 0x8b:
      dat_82b90 = 1;
      return 0;
    case 0x8c:
      v5 = sub_2d520(a1,(int8 *)0x82b58);
      if (v5 != -1) {
        dat_82b54 = (int4)v5;
        return 0;
      }
      dat_82b54 = 0xffffffff;
      if (dat_82b58) {
        sub_28080(dat_82b58,(unsigned int *)0x82b54);
        return 0;
      }
      dat_82b54 = 0xffffffff;
      return 0;
    case 0x8d:
      sub_24eb0(a1);
      return 0;
    case 0x8e:
      dat_82a59 = 1;
      return 0;
    case 0x8f:
      dat_82b52 = 1;
      return 0;
    case 0x90:
      dat_82820 = a1;
      return 0;
    case 0x91:
      dat_82b30 = 1;
      return 0;
    case 0x92:
      sub_2e340(6,(struct_80 *)*v17);
      return 0;
    case 0x93:
      dat_82b24 = (unsigned int)strtoul(a1,(char **)v12,10);
      if (!*(char *)v12._0_8_)
        return 0;
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid incremental level value";
      goto label_2f8ff;
    case 0x94:
      sub_2e3b0("lzip",(struct_80 *)*v17);
      return 0;
    case 0x95:
      sub_2e3b0("lzma",(struct_80 *)*v17);
      return 0;
    case 0x96:
      sub_2e3b0("lzop",(struct_80 *)*v17);
      return 0;
    case 0x97:
      dat_82b18 = sub_458b0(a1);
      if (dat_82b18) {
        dat_82b14 = umask(0);
        umask(dat_82b14);
        return 0;
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid mode given on option";
label_2f9a2:
      error(0,0,dcgettext(NULL,v16,5));
      sub_16fd0(); // no-return
    case 0x98:
      sub_2e120(v17,"--mtime",a1,(struct_82 *)0x82ae0);
      if (!dat_82af0) {
        dat_82af0 = 1;
        return 0;
      }
      return 0;
    case 0x99:
      if (dat_82b08 <= -1) {
        v16 = "--newer-mtime";
        sub_2e120(v17,v16,a1,(struct_82 *)0x82b00);
        sub_2d480(3,(struct_80 *)*v17);
        return 0;
      }
label_2f8e5:
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "More than one threshold date";
      goto label_2f8ff;
    case 0x9a:
      dat_82a9c = 0xffffffff;
      return 0;
    case 0x9b:
      *(char *)((int8)v17 + 0x12) = 0;
      return 0;
    case 0x9c:
      dat_82b20 = 0;
      return 0;
    case 0x9d:
      dat_827e0 = 0;
      return 0;
    case 0x9e:
      dat_82a59 = 0;
      return 0;
    case 0x9f:
      sub_2e340(1,(struct_80 *)*v17);
      return 0;
    case 0xa0:
      for (; *a1; a1 = &a1[1]) {
        sub_4c780(0,(int4)*a1,0);
      }
      return 0;
    case 0xa1:
      dat_82aa8 = 0xffffffff;
      return 0;
    case 0xa2:
      dat_82aa4 = 0xffffffff;
      return 0;
    case 0xa3:
      dat_827fc = 0;
      return 0;
    case 0xa4:
      dat_82aa0 = 0xffffffff;
      return 0;
    case 0xa5:
      dat_82a98 = 0xffffffff;
      return 0;
    case 0xa6:
      dat_82ad2 = 1;
      return 0;
    case 0xa7:
      sub_2d480(1,(struct_80 *)*v17);
      if (!a1) {
        dat_82b38 = (char *)0x1;
        return 0;
      }
      if (!sub_4eec0(a1,NULL,10,v12,"")) {
        dat_82b38 = (char *)v12._0_8_;
        return 0;
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = dcgettext(NULL,"Invalid number",5);
      a1 = (char *)sub_4ce40(a1);
      v8 = "%s: %s";
      error(0,0,v8,a1,v16); // return-dupe
      sub_16fd0(); // no-return
    case 0xa8:
      sub_2e430("v7");
      return 0;
    case 0xa9:
      dat_82ad1 = 1;
      return 0;
    case 0xaa:
      sub_2d480(7,(struct_80 *)*v17);
      dat_82ac8 = a1;
      dat_82ad0 = 1;
      return 0;
    case 0xab:
      sub_2e340(0,(struct_80 *)*v17);
      return 0;
    case 0xac:
      sub_2e340(2,(struct_80 *)*v17);
      return 0;
    case 0xad:
      v5 = sub_2d520(a1,(int8 *)0x82ac0);
      if (v5 != -1) {
        dat_82abc = (int4)v5;
        return 0;
      }
      dat_82abc = 0xffffffff;
      if (dat_82ac0) {
        sub_27fd0(dat_82ac0,(unsigned int *)0x82abc);
        return 0;
      }
      dat_82abc = 0xffffffff;
      return 0;
    case 0xae:
      sub_24de0(a1);
      return 0;
    case 0xaf:
      _obstack_begin((obstack *)v12,0,0,sub_4ecc0,dat_7efc8);
      while( true ) {
        v16 = v20;
        if (!*a1) break;
        v15 = (int8)v21 - (int8)v20;
        v11 = strcspn(a1,",");
        v18 = memchr(a1,0x3d,v11);
        if (v18) { // branch-flip
          v10 = (int8)v18 + (1U - (int8)a1);
          if (v15 < v10) {
            _obstack_newchunk((obstack *)v12,v10);
            v16 = v20;
          }
          memcpy(v16,a1,v10);
          v16 = &v20[v10];
          v10 = v11 - v10;
          v1 = *(uint1 *)((int8)v18 + 1);
          v18 = (void *)((int8)v18 + 1);
          v20 = v16;
          if (v1) {
            v7 = __ctype_b_loc();
            do {
              if (!(*(uint1 *)(*v7 + 1 + (uint8)v1 * 2) & 0x20)) {
                if ((v1 == 0x7b) && (*(char *)((int8)v18 + (v10 - 1)) == '}')) {
                  v16 = (char *)sub_4ecc0(v10);
                  memcpy(v16,(void *)((int8)v18 + 1),v10 - 2);
                  v16[v10 - 2] = '\0';
                  if (sub_2e120(v17,"--pax-option",v16,&v13)) { // branch-flip
                    if ((uint8)((int8)v21 - (int8)v20) < v10)
                      _obstack_newchunk((obstack *)v12,v10);
                    memcpy(v20,v18,v10);
                    v20 = &v20[v10];
                  }
                  else {
                    v8 = (char *)sub_257a0(v13,v19,v14);
                    v15 = strlen(v8);
                    if ((uint8)((int8)v21 - (int8)v20) < v15)
                      _obstack_newchunk((obstack *)v12,v15);
                    memcpy(v20,v8,v15);
                    v20 = &v20[v15];
                  }
                  free(v16);
                  v16 = v20;
                  goto label_2ece9;
                }
                break;
              }
              v1 = *(uint1 *)((int8)v18 + 1);
              v18 = (void *)((int8)v18 + 1);
              v10 -= 1;
            } while (v1);
          }
          if ((uint8)((int8)v21 - (int8)v16) < v10) {
            _obstack_newchunk((obstack *)v12,v10);
            v16 = v20;
          }
          memcpy(v16,v18,v10);
          v20 = &v20[v10];
          v16 = v20;
        }
        else {
          if (v15 < v11) {
            _obstack_newchunk((obstack *)v12,v11);
            v16 = v20;
          }
          memcpy(v16,a1,v11);
          v20 = &v20[v11];
          v16 = v20;
        }
label_2ece9:
        a1 = &a1[v11];
        v4 = *a1;
        if (!v4) break;
        if (v21 == v16) {
          _obstack_newchunk((obstack *)v12,1);
          v4 = *a1;
          v16 = v20;
        }
        v20 = &v16[1];
        a1 = &a1[1];
        *v16 = v4;
      }
      if (v16 == v21) {
        _obstack_newchunk((obstack *)v12,1);
        v16 = v20;
      }
      v3 = v12._16_8_;
      *v16 = '\0';
      if (v12._16_8_ == &v16[1])
        v23 |= 2;
      v16 = (char *)((uint8)&(&v16[1])[v22] & ~v22);
      v12._16_8_ = v21;
      if ((uint8)((int8)v16 - v12._8_8_) <= (uint8)((int8)v21 - v12._8_8_))
        v12._16_8_ = v16;
      v20 = v12._16_8_;
      v16 = (char *)sub_4ee30(v3);
      _obstack_free((obstack *)v12,NULL);
      *(char *)((int8)v17 + 0x11) = 1;
      sub_1ca20(v16);
      free(v16);
      return 0;
    case 0xb0:
      sub_2e430("posix");
      return 0;
    case 0xb1:
      for (; *a1; a1 = &a1[1]) {
        sub_4c780(0,(int4)*a1,1);
      }
      return 0;
    case 0xb2:
      v16 = "literal";
      v11 = 0;
      do {
        if (!strcmp(a1,v16)) {
          sub_4c760(NULL,v11 & 0xffffffff);
          return 0;
        }
        v11 += 1;
        v16 = *(char **)(v11 * 8 + 0x7e6e0);
      } while (v16);
      if (dat_82d50)
        (*dat_82d50)();
      v16 = dat_82dd0;
      v8 = dcgettext(NULL,"Unknown quoting style \'%s\'. Try \'%s --quoting-style=help\' to get a list.",5);
      error(0,0,v8,a1,v16);
      sub_16fd0();
    case 0xb3:
      if (!sub_4eec0(a1,NULL,10,v12,"bBcGgkKMmPTtw")) {
        dat_82b98 = (char *)v12._0_8_;
        if (!(v12._0_8_ & 0x1ff)) {
          dat_82ba0 = (int4)((uint8)v12._0_8_ >> 9);
          return 0;
        }
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"Record size must be a multiple of %d.",5),0x200);
        sub_2dea0(2); // no-return
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid record size";
      break;
    case 0xb4:
      dat_82aba = 1;
      return 0;
    case 0xb5:
      dat_82ab8 = 1;
      return 0;
    case 0xb6:
      dat_82a58 = 1;
      return 0;
    case 0xb7:
      dat_81c60 = a1;
      return 0;
    case 0xb8:
      dat_82ab0 = a1;
      return 0;
    case 0xb9:
      dat_82aa8 = 1;
      return 0;
    case 0xba:
      sub_2e430("posix");
      dat_82aa0 = 1;
      return 0;
    case 0xbb:
      v6 = (void *)sub_2d400();
      __printf_chk(1,"%s\n",v6);
      sub_40180();
      free(v6);
      exit(0); // no-return
    case 0xbc:
      dat_82a89 = 1;
      return 0;
    case 0xbd:
      sub_202a0();
      sub_40180();
      exit(0); // no-return
    case 0xbe:
      dat_827e1 = 1;
      return 0;
    case 0xbf:
      sub_2e340(5,(struct_80 *)*v17);
      return 0;
    case 0xc0:
      v5 = sub_3a4e0("--sort",a1,(int8 *)0x7d9c0,(void *)0x7fae8,4,dat_81dd8);
      dat_827e4 = *(unsigned int *)(v5 * 4 + 0x7fae8);
      return 0;
    case 0xc1:
      v5 = sub_3a4e0("--hole-detection",a1,(int8 *)0x7d990,(void *)0x69a88,4,dat_81dd8);
      dat_82a7c = *(unsigned int *)(v5 * 4 + 0x69a88);
      dat_82a88 = 1;
      return 0;
    case 0xc2:
      dat_82a88 = 1;
      dat_82a84 = (unsigned int)strtoul(a1,(char **)v12,10);
      if (!*(char *)v12._0_8_)
        return 0;
      if (*(char *)v12._0_8_ == '.') {
        v16 = (char *)(v12._0_8_ + 1);
        dat_82a80 = (unsigned int)strtoul(v16,(char **)v12,10);
        if (!*(char *)v12._0_8_)
          return 0;
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid sparse version value";
      goto label_2f8ff;
    case 0xc3:
      if (!sub_4eec0(a1,NULL,10,v12,"")) {
        dat_82a90 = (char *)v12._0_8_;
        return 0;
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Invalid number of elements";
      break;
    case 0xc4:
      v17[3] = a1;
      dat_82b84 = 1;
      return 0;
    case 0xc5:
      sub_2e2a0(9);
      return 0;
    case 0xc6:
      if (a1) {
        signal(sub_2ddf0(a1),sub_2d500);
        return 0;
      }
      dat_82a69 = 1;
      return 0;
    case 199:
      if (!dat_82a60) {
        dat_82a60 = a1;
        return 0;
      }
      if (dat_82d50)
        (*dat_82d50)();
      v16 = "Only one --to-command option allowed";
label_2f8ff:
      error(0,0,dcgettext(NULL,v16,5));
      sub_2dea0(2); // no-return
    case 200:
      sub_30870(a1);
      return 0;
    case 0xc9:
      dat_82b91 = 1;
      return 0;
    case 0xca:
      dat_82a48 = a1;
      return 0;
    case 0xcb:
      sub_31ab0(a1);
      return 0;
    case 0xcc:
      sub_2e430("posix");
      dat_82a98 = 1;
      return 0;
    case 0xcd:
    case 0xce:
      sub_2e430("posix");
      dat_82a98 = 1;
      sub_31fd0(a1,a0 == 0xce);
      return 0;
    case 0xcf:
      sub_2e3b0("zstd",(struct_80 *)*v17);
      return 0;
    
  }
  v16 = dcgettext(NULL,v16,5);
  error(0,0,"%s: %s",sub_4ce40(a1),v16);
  sub_2dea0(2); // no-return
}


// Function: sub_2fac0 @ 0x2fac0
void sub_2fac0(unsigned int a0,int8 *a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x38
  unsigned long v2; // stack - 0x30
  unsigned short v3; // stack - 0x28
  char v4; // stack - 0x26
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  v3 = 0;
  v2 = 0;
  v4 = 0;
  v5 = 0;
  v6 = 0;
  v1 = a2;
  argp_parse((void *)0x7f240,a0,a1,0x2a,NULL,&v1);
  return;
}


// Function: sub_2fb40 @ 0x2fb40
void sub_2fb40(unsigned long *a0)
{
  unsigned long *v1;
  uint8 v2; // rcx
  unsigned long *v3;
  
  *a0 = 0;
  a0[0x36] = 0;
  v2 = (uint8)(((int4)a0 - (int4)(unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8)) + 0x1b8U >> 3);
  v3 = (unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8);
  while (v2) {
    v1 = &v3[1];
    *v3 = 0;
    v2 -= 1;
    v3 = v1;
  }
  return;
}


// Function: sub_2fb80 @ 0x2fb80
bool sub_2fb80(struct_15 *a0)
{
  int4 v1; // eax
  
  if (a0->field_0x1a0) // branch-flip
    v1 = closedir(a0->field_0x1a0);
  else {
    if ((int4)a0->field_0x1a8 <= 0) {
      a0->field_0x1a8 = 0;
      return 1;
    }
    v1 = close(a0->field_0x1a8);
  }
  a0->field_0x1a8 = 0;
  a0->field_0x1a0 = NULL;
  if (v1)
    sub_26340(a0->field_0x0);
  return v1 == 0;
}


// Function: sub_2fc00 @ 0x2fc00
void sub_2fc00(unsigned long *a0)
{
  unsigned long *v1;
  uint8 v2; // rcx
  unsigned long *v3;
  
  sub_2fb80(a0);
  sub_1d580((unsigned long *)a0[0x2c],a0[0x2b]);
  free((void *)*a0);
  free((void *)a0[1]);
  free((void *)a0[3]);
  free((void *)a0[4]);
  free((void *)a0[5]);
  free((void *)a0[6]);
  free((void *)a0[7]);
  free((void *)a0[9]);
  free((void *)a0[0x28]);
  free((void *)a0[0x32]);
  sub_1de20(&a0[0x2d]);
  sub_17680(a0);
  *a0 = 0;
  a0[0x36] = 0;
  v2 = (uint8)(((int4)a0 - (int4)(unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8)) + 0x1b8U >> 3);
  v3 = (unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8);
  while (v2) {
    v1 = &v3[1];
    *v3 = 0;
    v2 -= 1;
    v3 = v1;
  }
  return;
}


// Function: sub_2fcc0 @ 0x2fcc0
int4 sub_2fcc0(int8 a0,int8 a1,int8 a2,int8 a3)
{
  int4 v1; // r8d
  
  v1 = 0;
  if (dat_82400 == 4)
    v1 = (uint4)(a3 < a1) - (uint4)(a1 < a3);
  return v1 + ((uint4)(a2 < a0) - (uint4)(a0 < a2)) * 2;
}


// Function: sub_2fd20 @ 0x2fd20
unsigned long sub_2fd20(uint4 *a0,unsigned int a1) // return-dupe
{
  switch(a1) {
    case 0x48:
      *a0 = *a0 & 0xfffffffd;
      return 0;
    default:
      return 1;
    case 0x52:
      *a0 = *a0 & 0xfffffffe;
      return 0;
    case 0x53:
      *a0 = *a0 & 0xfffffffb;
      return 0;
    case 0x68:
      *a0 = *a0 | 2;
      return 0;
    case 0x72:
      *a0 = *a0 | 1;
      return 0;
    case 0x73:
      *a0 = *a0 | 4;
      return 0;
    
  }
}


// Function: sub_2fdb0 @ 0x2fdb0
uint1 * sub_2fdb0(int4 a0,void *a1,uint8 a2)
{
  int8 *v1; // rax
  uint1 *v2;
  uint1 *v3;
  uint1 *v4; // rdx
  
  v2 = dat_82cb8;
  if (dat_82c40 < a2) {
    dat_82c40 = a2;
    v2 = (uint1 *)sub_4ece0(dat_82cb8,a2);
    dat_82cb8 = v2;
  }
  memcpy(v2,a1,a2);
  if (a0 != 3) { // branch-flip
    switch(a0) { // branch-flip
      case 1:
        v1 = __ctype_toupper_loc();
        *v2 = (uint1)*(unsigned int *)(*v1 + (uint8)*v2 * 4);
        return v2;
      case 2:
        v1 = __ctype_tolower_loc();
        *v2 = (uint1)*(unsigned int *)(*v1 + (uint8)*v2 * 4);
        break;
      case 4:
        if (v2 < &v2[a2]) {
          v1 = __ctype_tolower_loc();
          v3 = v2;
          do {
            v4 = &v3[1];
            *v3 = (uint1)*(unsigned int *)(*v1 + (uint8)*v3 * 4);
            v3 = v4;
          } while (v4 != &v2[a2]);
          return v2;
        }
      
    }
  }
  else if (v2 < &v2[a2]) {
    v1 = __ctype_toupper_loc();
    v3 = v2;
    do {
      v4 = &v3[1];
      *v3 = (uint1)*(unsigned int *)(*v1 + (uint8)*v3 * 4);
      v3 = v4;
    } while (v4 != &v2[a2]);
    return v2;
  }
  return v2;
}


// Function: sub_2fef0 @ 0x2fef0
void sub_2fef0(int8 a0,char a1)
{
  struct_83 *v1; // rax
  char *v2; // rax
  
  v1 = (struct_83 *)sub_4ecc0(0x20);
  v1->field_0x0 = 0;
  if (*(unsigned long **)(a0 + 0x60)) // branch-flip
    **(unsigned long **)(a0 + 0x60) = v1;
  else {
    *(struct_83 **)(a0 + 0x58) = v1;
  }
  *(int8 *)(a0 + 0x68) = *(int8 *)(a0 + 0x68) + 1;
  *(struct_83 **)(a0 + 0x60) = v1;
  v1->field_0x8 = 0;
  v2 = (char *)sub_4ecc0(2);
  v1->field_0x18 = 1;
  *v2 = a1;
  v2[1] = 0;
  v1->field_0x10 = v2;
  return;
}


// Function: sub_2ff60 @ 0x2ff60
void sub_2ff60(int8 a0,void *a1,int8 a2)
{
  struct_84 *v1; // rax
  void *v2; // rax
  uint8 v3; // rbx
  
  v3 = a2 - (int8)a1;
  if (!v3)
    return;
  v1 = (struct_84 *)sub_4ecc0(0x20);
  v1->field_0x0 = 0;
  if (*(unsigned long **)(a0 + 0x60)) // branch-flip
    **(unsigned long **)(a0 + 0x60) = v1;
  else {
    *(struct_84 **)(a0 + 0x58) = v1;
  }
  *(int8 *)(a0 + 0x68) = *(int8 *)(a0 + 0x68) + 1;
  *(struct_84 **)(a0 + 0x60) = v1;
  v1->field_0x8 = 0;
  v2 = (void *)sub_4ecc0(v3 + 1);
  v1->field_0x10 = v2;
  v2 = memcpy(v2,a1,v3);
  v1->field_0x18 = v3;
  *(char *)((int8)v2 + v3) = 0;
  return;
}


// Function: sub_30000 @ 0x30000
char * sub_30000(char *a0)
{
  unsigned long *v1;
  int8 v10; // rdx
  char v11;
  char *v12; // stack - 0x250
  char v13; // stack - 0x248
  char v14 [518];
  char *v15; // stack - 0x258
  char v16;
  char v17;
  unsigned long v18; // rsi
  char *v19;
  uint4 v2;
  char *v20;
  int4 v21;
  uint8 v22;
  int4 v23; // r13d
  int4 v24;
  char v25; // stack - 0x247
  uint8 v3;
  int4 v4;
  int8 v5;
  char *v6;
  int4 v7; // eax
  char *v8; // rax
  uint4 *v9; // rcx
  
  v8 = (int8)sub_4ed80(0x70);
  v5 = (int8)v8;
  if (dat_82cc0) // branch-flip
    *dat_82cc0 = (int8)v8;
  else {
    dat_82cc8 = (int8)v8;
  }
  if (*a0 != 's') { // branch-flip
    dat_82cc0 = (int8 *)v8;
    v7 = strncmp(a0,"flags=",6);
    if (!v7) {
      v11 = a0[6];
      a0 = &a0[6];
      dat_81b60 = 0;
      if (!v11) {
        v8 = a0;
        dat_81b60 = 0;
        return v8;
      }
      v9 = (uint4 *)0x81b60;
      while( true ) {
        if (v11 == ';') {
          v8 = &a0[1];
          return v8;
        }
        v7 = sub_2fd20(v9);
        if (v7) break;
        v11 = a0[1];
        a0 = &a0[1];
        if (!v11) {
          v8 = a0;
          return v8;
        }
      }
      if (dat_82d50)
        (*dat_82d50)();
      v11 = *a0;
      v19 = "Unknown transform flag: %c";
label_30774:
      v8 = dcgettext(NULL,v19,5);
      error(0,0,v8,(uint8)(uint4)(int4)v11);
      sub_2dea0(2); // no-return
    }
  }
  else {
    v11 = a0[1];
    v7 = (int4)v11;
    v4 = v7;
    dat_82cc0 = (int8 *)v5;
    if (v11) {
      v22 = 2;
      v16 = a0[2];
      v3 = 2;
      while (v16) {
        v21 = (int4)v22;
        v23 = v21 + 1;
        if (v11 == v16) {
          v10 = (int8)v23;
          v19 = &a0[v10];
          v11 = *v19;
          v24 = v23;
          if (v11) goto label_30192;
          goto label_307bb;
        }
        if (v16 == '\\')
          v23 = (v21 + 2) - (uint4)(a0[v3 + 1] == '\0');
        v22 = (uint8)v23;
        v16 = a0[v22];
        v3 = v22;
      }
    }
  }
label_307bb:
  if (dat_82d50)
    (*dat_82d50)();
  v19 = "Invalid transform expression";
label_307d5:
  v8 = dcgettext(NULL,v19,5);
  error(0,0,v8);
  sub_2dea0(2); // no-return
  while( true ) {
    if (v11 == '\\')
      v24 = (v24 + 1) - (uint4)(a0[v10 + 1] == '\0');
    v24 += 1;
    v10 = (int8)v24;
    v11 = a0[v10];
    if (!v11) break;
label_30192:
    if (v11 == v16) {
      v20 = &a0[v10 + 1];
      *(unsigned int *)(v5 + 8) = 0;
      v16 = *v20;
      *(unsigned int *)(v5 + 0xc) = dat_81b60;
      v2 = 0;
      v12 = v20;
      v17 = v16;
      if ((!v16) || (v16 == ';')) goto label_30250;
      goto label_30213;
    }
  }
  goto label_307bb;
  while( true ) {
    v17 = '\0';
    v12 = v20;
    if (!v16) goto label_30250;
    v21 = (int4)v22;
    if (v16 == ';') break;
label_30213:
    v21 = (int4)v22;
    v8 = v12;
    if (v16 != 'i') { // branch-flip
      if ('i' < v16) {
        if (v16 != 'x') goto label_301e2;
        v2 |= 1;
        v20 = &v12[1];
        goto label_301fb;
      }
      if (v16 <= '9') {
        if ('/' < v16) {
          v8 = (uint8)strtoul(v20,&v12,0);
          v22 &= 0xffffffff;
          *(int4 *)(v5 + 0x10) = v7;
          v8 = &v12[-1];
          v20 = v12;
          goto label_301fb;
        }
label_301e2:
        v7 = sub_2fd20((uint4 *)(v5 + 0xc));
        if (!v7) {
          v8 = v12;
          v20 = &v12[1];
          goto label_301fb;
        }
        if (dat_82d50)
          (*dat_82d50)();
        v19 = "Unknown flag in transform expression: %c";
        v11 = *v12;
        goto label_30774;
      }
      if (v16 != 'g') goto label_301e2;
      *(unsigned int *)(v5 + 8) = 1;
      v20 = &v12[1];
      v16 = v12[1];
    }
    else {
      v2 |= 2;
      v20 = &v12[1];
label_301fb:
      v21 = (int4)v22;
      v16 = v8[1];
    }
  }
label_30700:
  v12 = &v20[1];
label_3025a:
  v8 = (char *)sub_4ecc0((int8)(v21 + -1));
  v20 = v8;
  memcpy(v8,&a0[2],(int8)(v21 + -2));
  v8[v3 - 2] = '\0';
  v7 = sub_5dc90((struct_72 *)(v5 + 0x18),v8,v2);
  if (v7) {
    sub_5ddb0(v7,(struct_72 *)(v5 + 0x18),&v13,0x200);
    if (dat_82d50)
      (*dat_82d50)();
    v8 = dcgettext(NULL,"Invalid transform expression: %s",5);
    error(0,0,v8,&v13);
    sub_2dea0(2); // no-return
  }
  if ((*v20 == '^') || ((v21 != 2 && (v20[v3 - 3] == '$'))))
    *(unsigned int *)(v5 + 8) = 0;
  v24 -= v23;
  free(v20);
  v8 = (char *)sub_4ecc0((int8)(v24 + 1));
  v6 = v8;
  memcpy(v8,v19,(int8)v24);
  v15 = v8;
  v8[v24] = '\0';
  v16 = *v8;
  v19 = v8;
  v20 = v15;
  do {
    while( true ) {
      if (!v16) {
        v15 = v20;
        sub_2ff60(v5,v19,v8);
        free(v6);
        v8 = v12;
        return v8;
      }
      if (v16 == '\\') break;
      if (v16 != '&') // branch-flip
        v8 = &v8[1];
      else {
        sub_2ff60(v5,v19,v8);
        v8 = (unsigned long *)sub_4ecc0(0x20);
        v1 = *(unsigned long **)(v5 + 0x60);
        *(unsigned long *)v8 = 0;
        if (v1) // branch-flip
          *v1 = v8;
        else {
          *(char **)(v5 + 0x58) = v8;
        }
        *(char **)(v5 + 0x60) = v8;
        *(unsigned int *)((int8)v8 + 8) = 1;
        *(unsigned long *)((int8)v8 + 0x10) = 0;
        *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
        v8 = &v15[1];
        v19 = v8;
      }
      v16 = *v8;
      v15 = v8;
      v20 = v8;
    }
    sub_2ff60(v5,v19,v8);
    v19 = &v15[1];
    v16 = v15[1];
    v15 = v19;
    switch(v16) {
      case 0x26:
        v18 = 0x26;
        break;
      default:
        if (v16 != v11) { // branch-flip
          v13 = '\\';
          v25 = v16;
          sub_2ff60(v5,&v13,v14);
        }
        else {
          sub_2fef0(v5,v4);
        }
        v15 = &v15[1];
        v8 = v15;
        goto label_30480;
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
        v8 = (uint8)strtoul(v19,&v15,10);
        v22 = (uint8)v8;
        if (v8 <= *(uint8 *)(v5 + 0x48)) {
          v8 = (unsigned long *)sub_4ecc0(0x20);
          v1 = *(unsigned long **)(v5 + 0x60);
          *(unsigned long *)v8 = 0;
          if (v1) // branch-flip
            *v1 = v8;
          else {
            *(char **)(v5 + 0x58) = v8;
          }
          *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
          *(char **)(v5 + 0x60) = v8;
          *(unsigned int *)((int8)v8 + 8) = 1;
          *(uint8 *)((int8)v8 + 0x10) = v22;
          v8 = v15;
          goto label_30480;
        }
        if (dat_82d50)
          (*dat_82d50)();
        v19 = "Invalid transform replacement: back reference out of range";
        goto label_307d5;
      case 0x45:
        v8 = (unsigned long *)sub_4ecc0(0x20);
        v1 = *(unsigned long **)(v5 + 0x60);
        *(unsigned long *)v8 = 0;
        if (v1) // branch-flip
          *v1 = v8;
        else {
          *(char **)(v5 + 0x58) = v8;
        }
        *(char **)(v5 + 0x60) = v8;
        *(unsigned int *)((int8)v8 + 8) = 2;
        *(unsigned int *)((int8)v8 + 0x10) = 0;
        *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
        v15 = &v15[1];
        v8 = v15;
        goto label_30480;
      case 0x4c:
        v8 = (unsigned long *)sub_4ecc0(0x20);
        v1 = *(unsigned long **)(v5 + 0x60);
        *(unsigned long *)v8 = 0;
        if (v1) // branch-flip
          *v1 = v8;
        else {
          *(char **)(v5 + 0x58) = v8;
        }
        *(char **)(v5 + 0x60) = v8;
        *(unsigned int *)((int8)v8 + 8) = 2;
        *(unsigned int *)((int8)v8 + 0x10) = 4;
        *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
        v15 = &v15[1];
        v8 = v15;
        goto label_30480;
      case 0x55:
        v8 = (unsigned long *)sub_4ecc0(0x20);
        v1 = *(unsigned long **)(v5 + 0x60);
        *(unsigned long *)v8 = 0;
        if (v1) // branch-flip
          *v1 = v8;
        else {
          *(char **)(v5 + 0x58) = v8;
        }
        *(char **)(v5 + 0x60) = v8;
        *(unsigned int *)((int8)v8 + 8) = 2;
        *(unsigned int *)((int8)v8 + 0x10) = 3;
        *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
        v15 = &v15[1];
        v8 = v15;
        goto label_30480;
      case 0x5c:
        v18 = 0x5c;
        break;
      case 0x61:
        v18 = 7;
        break;
      case 0x62:
        v18 = 8;
        break;
      case 0x66:
        v18 = 0xc;
        break;
      case 0x6c:
        v8 = (unsigned long *)sub_4ecc0(0x20);
        v1 = *(unsigned long **)(v5 + 0x60);
        *(unsigned long *)v8 = 0;
        if (v1) // branch-flip
          *v1 = v8;
        else {
          *(char **)(v5 + 0x58) = v8;
        }
        *(char **)(v5 + 0x60) = v8;
        *(unsigned int *)((int8)v8 + 8) = 2;
        *(unsigned int *)((int8)v8 + 0x10) = 2;
        *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
        v15 = &v15[1];
        v8 = v15;
        goto label_30480;
      case 0x6e:
        v18 = 10;
        break;
      case 0x72:
        v18 = 0xd;
        break;
      case 0x74:
        v18 = 9;
        break;
      case 0x75:
        v8 = (unsigned long *)sub_4ecc0(0x20);
        v1 = *(unsigned long **)(v5 + 0x60);
        *(unsigned long *)v8 = 0;
        if (v1) // branch-flip
          *v1 = v8;
        else {
          *(char **)(v5 + 0x58) = v8;
        }
        *(char **)(v5 + 0x60) = v8;
        *(unsigned int *)((int8)v8 + 8) = 2;
        *(unsigned int *)((int8)v8 + 0x10) = 1;
        *(int8 *)(v5 + 0x68) = *(int8 *)(v5 + 0x68) + 1;
        v15 = &v15[1];
        v8 = v15;
        goto label_30480;
      case 0x76:
        v18 = 0xb;
      
    }
    sub_2fef0(v5,v18);
    v15 = &v15[1];
    v8 = v15;
label_30480:
    v16 = *v8;
    v19 = v8;
    v20 = v15;
  } while( true );
label_30250:
  if (v17 != ';') goto label_3025a;
  goto label_30700;
}


// Function: sub_30870 @ 0x30870
void sub_30870(char *a0)
{
  if (!*a0)
    return;
  do {
    a0 = (char *)sub_30000(a0);
  } while (*a0);
  return;
}


// Function: sub_308a0 @ 0x308a0
bool sub_308a0(unsigned long *a0,uint4 a1,code *a2,unsigned long a3) // return-dupe
{
  int4 v1;
  uint4 v10;
  char *v11;
  bool v12; // r12b
  int8 *v13;
  int8 *v14;
  char *v15;
  uint8 v16; // stack - 0x78
  uint4 v17; // stack - 0x58
  uint4 v2;
  int8 v3;
  uint8 *v4; // rax
  uint8 v5;
  uint8 v6;
  char *v7;
  uint8 v8;
  uint4 v9;
  
  v7 = (char *)*a0;
  if (!dat_82c48) {
    _obstack_begin((obstack *)0x82c60,0,0,sub_4ecc0,dat_7efc8);
    dat_82c48 = '\x01';
  }
  if (dat_82cc8) {
    v12 = 0;
    v13 = dat_82cc8;
    do {
      while (!(a1 & *(uint4 *)((int8)v13 + 0xc))) {
        v13 = (int8 *)*v13;
        if (!v13) goto label_30b0c;
      }
      v4 = (uint8 *)sub_4ecc0((v13[9] + 1) * 0x10);
      if (*v7) {
        v17 = 0;
        v9 = 0;
        v16 = 0;
        do {
          v3 = v13[9];
          if (sub_5de90((struct_34 *)&v13[3],v7,v3 + 1,v4,0)) { // branch-flip
            v5 = strlen(v7);
            if ((uint8)((int8)dat_82c80 - (int8)dat_82c78) < v5)
              _obstack_newchunk((obstack *)0x82c60,v5);
            memcpy(dat_82c78,v7,v5);
            dat_82c78 = &dat_82c78[v5];
label_30a33:
            v15 = &v7[v5];
            if (!(int4)v13[1]) {
              v6 = strlen(v15);
              if ((uint8)((int8)dat_82c80 - (int8)dat_82c78) < v6)
                _obstack_newchunk((obstack *)0x82c60,v6);
              memcpy(dat_82c78,v15,v6);
              dat_82c78 = &dat_82c78[v6];
              if (dat_82c80 == dat_82c78) goto label_30d94;
              goto label_30a8c;
            }
          }
          else {
            v6 = *v4;
            v5 = v4[1];
            if (v6) {
              if ((uint8)((int8)dat_82c80 - (int8)dat_82c78) < v6)
                _obstack_newchunk((obstack *)0x82c60,v6);
              memcpy(dat_82c78,v7,v6);
              dat_82c78 = &dat_82c78[v6];
            }
            v16 += 1;
            if ((!*(uint4 *)&v13[2]) || (*(uint4 *)&v13[2] <= v16)) {
              v14 = (int8 *)v13[0xb];
              if (v14) {
                v10 = v9;
                do {
                  while( true ) {
                    v2 = v17;
                    v1 = (int4)v14[1];
                    v9 = v10;
                    if (v1 != 1) break;
                    v6 = v4[v14[2] * 2];
                    if ((v6 != 0xffffffffffffffff) && (v8 = (&v4[v14[2] * 2])[1], v8 != 0xffffffffffffffff)) {
                      v8 -= v6;
                      v15 = &v7[v6];
                      if (v10) {
                        v15 = (char *)sub_2fdb0(v10,v15,v8);
                        if (v10 - 1 <= 1) {
                          v17 = 0;
                          v10 = v2;
                        }
                      }
                      v11 = dat_82c78;
                      if ((uint8)((int8)dat_82c80 - (int8)dat_82c78) < v8) {
label_30c72:
                        _obstack_newchunk((obstack *)0x82c60,v8);
                        v11 = dat_82c78;
                      }
                      memcpy(v11,v15,v8); // crossjump-dupe
                      dat_82c78 = &dat_82c78[v8];
                      v9 = v10;
                    }
label_30b98:
                    v14 = (int8 *)*v14;
                    v10 = v9;
                    if (!v14) goto label_30a33;
                  }
                  if (v1 != 2) {
                    if (!v1) {
                      if (v10) { // branch-flip
                        v15 = (char *)sub_2fdb0(v10,(void *)v14[2],v14[3]);
                        if (v10 - 1 <= 1) {
                          v17 = 0;
                          v10 = v2;
                        }
                      }
                      else {
                        v15 = (char *)v14[2];
                      }
                      v8 = v14[3];
                      v11 = dat_82c78;
                      if ((uint8)((int8)dat_82c80 - (int8)dat_82c78) < v8) goto label_30c72;
                      memcpy(v11,v15,v8);
                      dat_82c78 = &dat_82c78[v8];
                      v9 = v10;
                    }
                    goto label_30b98;
                  }
                  v2 = *(uint4 *)&v14[2];
                  v9 = v2;
                  if (v2 > 2) {
                    v9 = v10;
                    if (v2 - 3 <= 1)
                      v9 = v2;
                    goto label_30b98;
                  }
                  if (v2) {
                    if ((v17) && (2 <= v17 - 3)) {
                    }
                    else {
                      v17 = v10;
                    }
                    goto label_30b98;
                  }
                  v14 = (int8 *)*v14;
                  v9 = 0;
                  v10 = 0;
                } while (v14);
              }
              goto label_30a33;
            }
            if ((uint8)((int8)dat_82c80 - (int8)dat_82c78) < v5)
              _obstack_newchunk((obstack *)0x82c60,v5);
            v15 = &v7[v5];
            memcpy(dat_82c78,v7,v5);
            dat_82c78 = &dat_82c78[v5];
          }
          v7 = v15;
        } while (*v15);
      }
      if (dat_82c80 == dat_82c78) {
label_30d94:
        _obstack_newchunk((obstack *)0x82c60,1);
      }
label_30a8c:
      v7 = &dat_82c78[1];
      *dat_82c78 = '\0';
      dat_82c78 = v7;
      free(v4);
      if (dat_82c70 == dat_82c78)
        dat_82cb0 |= 2;
      v12 = 1;
      v7 = &dat_82c78[dat_82c90];
      dat_82c78 = dat_82c80;
      if ((uint8)((int8)(char *)((uint8)v7 & ~dat_82c90) - (int8)dat_82c68) <= (uint8)((int8)dat_82c80 - (int8)dat_82c68))
        dat_82c78 = (char *)((uint8)v7 & ~dat_82c90);
      v13 = (int8 *)*v13;
      v7 = dat_82c70;
      dat_82c70 = dat_82c78;
    } while (v13);
label_30b0c:
    if (v12) {
      v15 = v7;
      if (a2)
        v15 = (char *)(*a2)(v7,a3);
      sub_25120(a0,v15);
      if ((dat_82c68 < v7) && (v7 < dat_82c80)) {
        dat_82c70 = v7;
        dat_82c78 = v7;
        return v12;
      }
      _obstack_free((obstack *)0x82c60,v7);
      return v12;
    }
  }
  if (a2) {
    *a0 = 0;
    sub_25120(a0,(char *)(*a2)(v7,a3));
    free(v7);
    return 1;
  }
  return 0;
}


// Function: sub_30e70 @ 0x30e70
bool sub_30e70(char **a0,uint4 a1)
{
  return sub_308a0(a0,a1,NULL,0); // tail-call
}


// Function: sub_30e80 @ 0x30e80
bool sub_30e80(void)
{
  return dat_82cc8 != 0;
}


// Function: sub_30e90 @ 0x30e90
void sub_30e90(bool a0)
{
  int4 v1;
  int8 *v2;
  unsigned int v3;
  char *v4;
  int4 *v5; // rax
  int8 *v6;
  int8 *v7; // r15
  
  v3 = dat_82580;
  v6 = NULL;
  v2 = dat_82ce8;
  if (dat_82ce8) {
    do {
      while ((v7 = v2, v2 = (int8 *)*v7, !a0 && (dat_81f30 <= (uint8)v7[4]))) {
label_30ed0:
        v6 = v7;
label_30ed3:
        if (!v2) goto label_30fa4;
      }
      sub_26300(*(unsigned int *)&v7[1]);
      if ((char)v7[3]) {
        v4 = (char *)v7[2];
        if ((!(int4)v7[1]) || ((*v4 && (strcmp(v4,"."))))) {
          if (unlinkat(dat_7f234,v4,0x200)) {
            v1 = *__errno_location();
            if ((v1 == 0x11) || (v1 == 0x27)) goto label_30ed0;
            if (v1 != 2) {
              sub_33180(v4);
              v4 = (char *)v7[2];
              goto label_30f70;
            }
          }
          goto label_310d8;
        }
        goto label_30ed0;
      }
      v4 = (char *)v7[2];
      if (unlinkat(dat_7f234,v4,0)) { // branch-flip
        v5 = __errno_location();
        v4 = (char *)v7[2];
        if (*v5 != 2) {
          sub_33490(v4);
          v4 = (char *)v7[2]; // crossjump-dupe
        }
      }
      else {
label_310d8:
        v4 = (char *)v7[2];
      }
label_30f70:
      free(v4);
      dat_82cd8 -= 1;
      *v7 = (int8)dat_82cd0;
      dat_82cd0 = v7;
      if (!v6) {
        dat_82ce8 = v2;
        goto label_30ed3;
      }
      *v6 = (int8)v2;
    } while (v2);
label_30fa4:
    if (dat_82ce8) {
      v2 = dat_82ce8;
      if (a0) {
        do {
          v6 = v2;
          v2 = (int8 *)*v6;
          sub_26300(*(unsigned int *)&v6[1]);
          v4 = (char *)v6[2];
          if ((((int4)v6[1]) && ((char)v6[3])) && ((!*v4 || (!strcmp(v4,"."))))) {
            v4 = (char *)sub_26320();
            sub_26300((int4)v6[1] + -1);
          }
          if ((unlinkat(dat_7f234,v4,0x200)) && (*__errno_location() != 2))
            sub_33180(v4);
          free((void *)v6[2]);
          dat_82cd8 -= 1;
          *v6 = (int8)dat_82cd0;
          dat_82cd0 = v6;
        } while (v2);
        dat_82ce0 = 0;
        dat_82ce8 = NULL;
      }
      sub_26300(v3); // return-dupe, tail-call
      return;
    }
  }
  dat_82ce0 = 0;
  sub_26300(v3);
  return;
}


// Function: sub_31110 @ 0x31110
void sub_31110(void) // return-dupe
{
  int8 *v1;
  int8 *v2;
  
  sub_30e90(1);
  v2 = dat_82cd0;
  if (!dat_82cd0)
    return;
  do {
    v1 = (int8 *)*v2;
    free(v2);
    v2 = v1;
    dat_82cd0 = v1;
  } while (v1);
  return;
}


// Function: sub_31150 @ 0x31150
void sub_31150(char *a0,char a1)
{
  unsigned long *v1;
  unsigned long *v2;
  unsigned long *v3;
  char *v4;
  unsigned long *v5;
  unsigned long *v6; // rbx
  unsigned long *v7;
  
  v5 = dat_82cd0;
  if ((dat_82ce8) && ((uint8)dat_82ce8[4] < dat_81f30)) {
    sub_30e90(0);
    v5 = dat_82cd0;
  }
  if (v5) { // branch-flip
    dat_82cd0 = (unsigned long *)*v5;
    *v5 = 0;
  }
  else {
    dat_82cd0 = v5;
    v5 = (unsigned long *)sub_4ecc0(0x28);
  }
  *v5 = 0;
  *(unsigned int *)&v5[1] = dat_82580;
  v4 = (char *)sub_4ee30(a0);
  v5[2] = v4;
  sub_253e0(v4);
  *(char *)&v5[3] = a1;
  v5[4] = dat_81f30;
  v2 = v5;
  if ((a1) && ((v4 = (char *)v5[2], !*v4 || (!strcmp(v4,"."))))) {
    v1 = dat_82ce8;
    if (dat_82ce8) {
      v3 = dat_82ce8;
      v7 = NULL;
      while( true ) {
        v6 = v3;
        if ((*(char *)&v6[3]) && (((v4 = (char *)v6[2], !*v4 || (!strcmp(v4,"."))) && (*(int4 *)&v6[1] < *(int4 *)&v5[1])))) {
          if (!v7) {
            *v5 = v1;
            dat_82ce8 = v5;
            dat_82cd8 += 1; // return-dupe
            return;
          }
          *v5 = *v7;
          *v7 = v5;
          v2 = dat_82ce8;
          v1 = (unsigned long *)*v5;
          goto label_3130c;
        }
        if (!(unsigned long *)*v6) break;
        v3 = (unsigned long *)*v6;
        v7 = v6;
      }
    }
    if (dat_82ce0) { // branch-flip
      *v5 = *dat_82ce0;
      *dat_82ce0 = v5;
      v1 = (unsigned long *)*v5;
      v2 = dat_82ce8;
    }
    else {
      *v5 = v1;
    }
  }
  else if (dat_82ce0) { // branch-flip
    *v5 = *dat_82ce0;
    *dat_82ce0 = v5;
    v2 = dat_82ce8;
    v1 = (unsigned long *)*v5;
  }
  else {
    *v5 = dat_82ce8;
    v1 = dat_82ce8;
  }
label_3130c:
  dat_82ce8 = v2;
  if (!v1)
    dat_82ce0 = v5;
  dat_82cd8 += 1;
  return;
}


// Function: sub_31350 @ 0x31350
void sub_31350(void)
{
  unsigned long *v1;
  unsigned long v10; // rax
  uint8 v11; // rcx
  stat v12; // stack - 0xe8
  char v13 [24];
  uint4 v14;
  uint8 v15;
  char *v16;
  char *v17; // stack - 0xf8
  unsigned int v2;
  uint4 v3; // eax
  int4 v4; // eax
  int8 v5;
  int8 v6; // rax
  uint8 v7;
  unsigned long *v8;
  unsigned long v9; // rax
  
  sub_282f0();
  sub_f710(2);
  sub_1d4e0();
  v14 = 0;
  do {
    v3 = sub_22570((unsigned long *)0x82408,(unsigned long *)0x82840,0);
    switch(v3) {
      case 0:
      case 2:
        abort(); // no-return, return-dupe
      case 1:
        sub_22c60(dat_82408,0x82840,(int4 *)0x82400,0);
        sub_22460((int4)*(char *)(dat_82408 + 0x9c),0x82840);
        dat_82ba4 = dat_82400;
        if ((dat_82ba8 == 8) && (v5 = sub_291d0(dat_82848), v5)) {
          sub_26300(*(unsigned int *)(v5 + 0x28));
          v4 = sub_25fc0(dat_82848,&v12);
          if (!v4) {
            if ((v12._24_4_ & 0xf000) != 0x4000) { // branch-flip
              v4 = sub_2fcc0(v12._88_8_,v12._96_8_,dat_82938,dat_82940);
              if (v4 <= 0)
                sub_28840(v5);
            }
            else {
              v17 = (char *)sub_26970(*(char **)(v5 + 0x10),1);
              if (v17) {
                v8 = (unsigned long *)sub_265f0(*(char **)(v5 + 0x10));
                if (*v17) {
                  v16 = v17;
                  do {
                    v2 = *(unsigned int *)(v5 + 0x28);
                    sub_28210((char *)sub_26680(v8,v16),v2,0,0);
                    v16 = &v16[strlen(v16) + 1];
                  } while (*v16);
                }
                sub_26660(v8);
                free(v17);
                sub_28840(v5);
              }
            }
          }
        }
        sub_24660();
        sub_2fc00((unsigned long *)0x82840);
        break;
      case 3:
        dat_81f48 = dat_82408;
        sub_2fc00((unsigned long *)0x82840);
        goto label_313e8;
      case 4:
        sub_2fc00((unsigned long *)0x82840);
label_313e8:
        sub_cc80();
        dat_82cf8 = 1;
        dat_82cf0 = dat_81f48;
label_31418:
        do {
          v5 = sub_29260();
          while( true ) {
            if (!v5) {
              sub_12fe0();
              sub_d8d0();
              sub_31110();
              sub_28a70(); // tail-call
              return;
            }
            v16 = *(char **)(v5 + 0x10);
            if ((sub_176d0(v16,0)) || ((dat_82b40 && (!sub_2df50("add",v16))))) goto label_31418;
            if (dat_82ba8 == 2) break;
            sub_14bb0(NULL,v16,v16);
            v5 = sub_29260();
          }
          v4 = openat(dat_7f234,v16,0);
          if (0 <= v4) { // branch-flip
            v15 = v12._48_8_;
            if (fstat(v4,&v12)) // branch-flip
              sub_33410(v16);
            else {
              for (; 0 < (int8)v15; v15 = v15 - v6) {
                v5 = sub_d360();
                v7 = sub_cd10(v5);
                if (v15 < v7) {
                  v7 = v15;
                  if (v15 & 0x1ff) {
                    v14 = 0x200 - ((uint4)v15 & 0x1ff);
                    v8 = (unsigned long *)(v5 + v15);
                    if (8 <= v14) { // branch-flip
                      *v8 = 0;
                      *(unsigned long *)((int8)v8 + ((uint8)v14 - 8)) = 0;
                      v11 = (uint8)(v14 + ((int4)v8 - (int4)(unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8)) >> 3);
                      v8 = (unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
                      while (v11) {
                        v1 = &v8[1];
                        *v8 = 0;
                        v11 -= 1;
                        v8 = v1;
                      }
                    }
                    else if (v14 & 4) { // branch-flip
                      *(unsigned int *)v8 = 0;
                      *(unsigned int *)((int8)v8 + ((uint8)v14 - 4)) = 0;
                    }
                    else if ((v14) && (*(char *)v8 = 0, v14 & 2))
                      *(unsigned short *)((int8)v8 + ((uint8)v14 - 2)) = 0;
                  }
                }
                v6 = sub_4d570(v4,v5,v7);
                if (v6 == -1)
                  sub_330b0(v16,v12._48_8_ - v15,v7); // no-return
                if (!v6) {
                  if (dat_82d50)
                    (*dat_82d50)();
                  v9 = sub_45370(v15,v13);
                  v10 = sub_4ce40(v16);
                  error(0,0,dcngettext(NULL,"%s: File shrank by %s byte","%s: File shrank by %s bytes",v15,5),v10,v9);
                  sub_16fd0(); // no-return
                }
                sub_ccd0(v5 + (v6 - 1U & 0xfffffffffffffe00U));
              }
            }
            if (close(v4))
              sub_32d60(v16);
          }
          else {
            sub_32ea0(v16);
          }
        } while( true );
      case 5:
        sub_ccd0(dat_82408);
        if (v14 != 1) {
          if (2 <= v14) { // branch-flip
            if (v14 != 3) {
              if (v14 != 4) goto label_315cc;
              abort();
            }
          }
          else {
            if (dat_82d50)
              (*dat_82d50)();
            error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
          }
        }
        if (dat_82d50)
          (*dat_82d50)();
        error(0,0,dcgettext(NULL,"Skipping to next header",5));
        dat_82d58 = 2;
        sub_2fc00((unsigned long *)0x82840);
        break;
      default:
label_315cc:
        sub_2fc00((unsigned long *)0x82840);
      
    }
    v14 = v3;
  } while( true );
}


// Function: sub_31920 @ 0x31920
unsigned long sub_31920(uint1 a0,char *a1,unsigned long *a2) // return-dupe
{
  char *v1; // rax
  char *v2; // stack - 0x48
  int8 v3; // stack - 0x40
  char *v4; // stack - 0x50
  int8 v5; // stack - 0x38
  int8 v6;
  
  v6 = *(int8 *)((uint8)a0 * 8 + 0x81b70);
  if (v6 != -1) { // branch-flip
    if (!v6) {
label_319ce:
      *a2 = sub_4ee30(a1);
      return 1;
    }
  }
  else {
    if (a0) // branch-flip
      dat_81b78 = iconv_open("UTF-8",sub_453d0());
    else {
      dat_81b70 = iconv_open(sub_453d0(),"UTF-8");
    }
    v6 = *(int8 *)((uint8)a0 * 8 + 0x81b70);
    if (!v6) goto label_319ce;
    if (v6 == -1)
      return 0;
  }
  v3 = strlen(a1) + 1;
  v5 = v3 * 0x10 + 1;
  v1 = (char *)sub_4ecc0();
  v2 = v1;
  v4 = a1;
  if (!iconv(v6,&v4,&v3,&v2,&v5)) {
    *v2 = 0;
    *a2 = v1;
    return 1;
  }
  free(v1);
  return 0;
}


// Function: sub_31a80 @ 0x31a80
unsigned long sub_31a80(char *a0)
{
  while( true ) {
    if (!*a0)
      return 1;
    if (*a0 < '\0') break;
    a0 = &a0[1];
  }
  return 0;
}


// Function: sub_31ab0 @ 0x31ab0
void sub_31ab0(char *a0)
{
  unsigned long v1;
  uint8 v2; // rax
  
  if (!strcmp(a0,"none")) {
    dat_81b80 = 0;
    return;
  }
  v2 = strlen(a0);
  v1 = dat_81dd8;
  if ((3 <= v2) && (!memcmp(a0,(void *)0x6a08a,3))) {
    dat_81b80 = ~*(uint4 *)(sub_3a4e0("--warning",&a0[3],(int8 *)0x7da80,(void *)0x81ba0,4,v1) * 4 + 0x81ba0) & dat_81b80;
    return;
  }
  dat_81b80 |= *(uint4 *)(sub_3a4e0("--warning",a0,(int8 *)0x7da80,(void *)0x81ba0,4,v1) * 4 + 0x81ba0);
  return;
}


// Function: sub_31e00 @ 0x31e00
unsigned long sub_31e00(char *a0,bool a1)
{
  char *v1;
  uint8 v2; // rbx
  
  if (dat_82d28) { // branch-flip
    if (dat_82d30) {
      v2 = 0;
      do {
        v1 = *(char **)(dat_82d20 + v2 * 8);
        if (!fnmatch(v1,a0,0)) goto label_31e78;
        v2 += 1;
      } while (v2 < dat_82d30);
    }
  }
  else if ((a1) || (!strncmp(a0,"user.",5))) {
label_31e78:
    if ((dat_82d40) && (dat_82d48)) {
      v2 = 0;
      do {
        v1 = *(char **)(dat_82d38 + v2 * 8);
        if (!fnmatch(v1,a0,0))
          return 1;
        v2 += 1;
      } while (v2 < dat_82d48);
    }
    return 0;
  }
  return 1;
}


// Function: sub_31ed0 @ 0x31ed0
void sub_31ed0(void)
{
  if (dat_82a9c <= 0)
    return;
  if (!dat_82d14) {
    if (dat_82d50)
      (*dat_82d50)();
    error(0,0,dcgettext(NULL,"POSIX ACL support is not available",5));
    dat_82d14 = 1;
    return;
  }
  dat_82d14 = 1;
  return;
}


// Function: sub_31f50 @ 0x31f50
void sub_31f50(unsigned long a0,unsigned long a1,char a2)
{
  if ((1 <= dat_82a9c) && (a2 != '2')) {
    if (!dat_82d10) {
      if (dat_82d50)
        (*dat_82d50)();
      error(0,0,dcgettext(NULL,"POSIX ACL support is not available",5));
      dat_82d10 = 1;
      return;
    }
    dat_82d10 = 1;
    return;
  }
  return;
}


// Function: sub_31fd0 @ 0x31fd0
void sub_31fd0(unsigned long a0,bool a1)
{
  uint8 v1;
  void *v2;
  uint8 v3;
  unsigned long *v4;
  
  v4 = (unsigned long *)0x82d20;
  if (!a1)
    v4 = (unsigned long *)0x82d38;
  v1 = v4[2];
  v3 = v4[1];
  v2 = (void *)*v4;
  if (v1 != v3) goto label_31fff;
  if (v3) { // branch-flip
    if (v2) { // branch-flip
      if (0xaaaaaaaaaaaaaa9 < v3)
        sub_17020(); // no-return, return-dupe
label_32031:
      v3 = v3 + 1 + (v3 >> 1);
    }
    else if ((SUB168(ZEXT816(8) * ZEXT816(v3),0) < 0) || (SUB168(ZEXT816(8) * ZEXT816(v3),8))) {
      sub_17020();
    }
  }
  else {
    v4[1] = 4;
    v3 = 4;
    if (v2) goto label_32031;
  }
  v4[1] = v3;
  v2 = (void *)sub_4ece0(v2,v3 * 8);
  *v4 = v2;
  v1 = v4[2];
label_31fff:
  v4[2] = v1 + 1;
  *(unsigned long *)((int8)v2 + v1 * 8) = a0;
  return;
}


// Function: sub_320a0 @ 0x320a0
void sub_320a0(void)
{
  if (dat_82d28) {
    free(dat_82d20);
    if (!dat_82d40)
      return;
    free(dat_82d38); // tail-call
    return;
  }
  if (!dat_82d40)
    return;
  free(dat_82d38); // tail-call
  return;
}


// Function: sub_32100 @ 0x32100
void sub_32100(unsigned int a0,char *a1,int8 a2,int4 a3) // return-dupe x2
{
  int4 v1;
  char *v2;
  int8 v3; // rax
  char *v4; // rax
  void *v5;
  int8 v6;
  uint8 v7;
  uint8 v8; // stack - 0x58
  
  if (dat_82a98 <= 0)
    return;
  v7 = dat_81c10;
  v2 = dat_82d08;
  if (!dat_82d08) {
    if (dat_81c10) { // branch-flip
      if ((int8)dat_81c10 < 0) {
        sub_17020(); // no-return, return-dupe
      }
    }
    else {
      dat_81c10 = 0x80;
    }
    v2 = (char *)sub_4ece0(NULL);
    v7 = dat_81c10;
    dat_82d08 = v2;
  }
  while (!a3) {
    v3 = sub_39dd0(a0,a1,v2,v7);
    if (v3 != -1) {
      v2 = dat_82d08;
      goto label_321e3;
    }
label_32176:
    if (*__errno_location() != 0x22) {
      v2 = "llistxattrat";
      if (a3)
        v2 = "flistxattr";
      sub_32bd0(v2,a1); // tail-call
      return;
    }
    if (dat_82d08) { // branch-flip
      if (0x5555555555555553 < dat_81c10) {
        sub_17020();
      }
      dat_81c10 = (dat_81c10 >> 1) + 1 + dat_81c10;
    }
    else if (dat_81c10) { // branch-flip
      if ((int8)dat_81c10 <= -1) {
        sub_17020();
      }
    }
    else {
      dat_81c10 = 0x80;
    }
    v2 = (char *)sub_4ece0(dat_82d08);
    v7 = dat_81c10;
    dat_82d08 = v2;
  }
  v3 = flistxattr(a3,v2,v7);
  if (v3 == -1) goto label_32176;
label_321e3:
  if (!dat_82d00) {
    if (dat_81c08) { // branch-flip
      if ((int8)dat_81c08 < 0)
        sub_17020();
    }
    else {
      dat_81c08 = 0x80;
    }
    dat_82d00 = (void *)sub_4ece0(NULL);
  }
  if (v3 <= 0)
    return;
  v4 = "fgetxattr";
  if (!a3)
    v4 = "lgetxattrat";
  do {
    v8 = strlen(v2);
    v5 = dat_82d00;
    while( true ) {
      if (a3) // branch-flip
        v6 = fgetxattr(a3,v2,v5,dat_81c08);
      else {
        v6 = sub_398d0(a0,a1,v2,v5);
      }
      if (v6 != -1) {
        if (!sub_31e00(v2,1))
          sub_1d5d0(a2,v2,dat_82d00,v6);
        goto label_322e3;
      }
      v1 = *__errno_location();
      if (v1 != 0x22) break;
      if (dat_82d00) { // branch-flip
        if (0x5555555555555553 < dat_81c08) {
          sub_17020();
        }
        dat_81c08 = (dat_81c08 >> 1) + 1 + dat_81c08;
      }
      else if (dat_81c08) { // branch-flip
        if ((int8)dat_81c08 <= -1) {
          sub_17020();
        }
      }
      else {
        dat_81c08 = 0x80;
      }
      v5 = (void *)sub_4ece0(dat_82d00);
      dat_82d00 = v5;
    }
    if (v1 != 0x3d)
      sub_32bd0(v4,a1);
label_322e3:
    v2 = &v2[v8 + 1];
    v3 = (v3 - v8) + -1;
    if (v3 <= 0)
      return;
  } while( true );
}


// Function: sub_32430 @ 0x32430
void sub_32430(int4 a0,char *a1,struct_53 *a2,int4 a3)
{
  int4 v1;
  char *v2; // rdi
  
  if (dat_82aa0 <= 0)
    return;
  if (a3) // branch-flip
    v1 = sub_4fbc0(a3,&a2->field_0x20[0x10]);
  else {
    v1 = sub_4dd40(a0,a1);
  }
  if (v1 == -1) {
    v1 = *__errno_location();
    if ((v1 != 0x5f) && (v1 != 0x3d)) {
      v2 = "fgetfilecon";
      if (!a3)
        v2 = "lgetfileconat";
      sub_32bd0(v2,a1); // tail-call
      return;
    }
  }
  return;
}


// Function: sub_324c0 @ 0x324c0
void sub_324c0(int8 a0,char *a1,char a2)
{
  int4 v1; // eax
  char *v2; // rax
  char *v3;
  
  if ((dat_82aa0 <= 0) || (v3 = *(char **)(a0 + 0x30), !v3))
    return;
  if (a2 != '2') { // branch-flip
    v1 = sub_4df60(dat_7f234,a1,v3);
    v3 = "setfileconat";
  }
  else {
    v1 = sub_4e180(dat_7f234,a1,v3);
    v3 = "lsetfileconat";
  }
  if ((v1 == -1) && (dat_81b82 & 0x20)) {
    if (dat_82d50)
      (*dat_82d50)();
    v2 = dcgettext(NULL,"%s: Cannot set SELinux context for file \'%s\'",5);
    error(0,*__errno_location(),v2,v3,a1); // tail-call
    return;
  }
  return;
}


// Function: sub_32570 @ 0x32570
void sub_32570(int8 a0,char *a1,char a2,uint4 a3)
{
  void *v1;
  int4 v2; // eax
  char *v3; // rax
  uint8 v4; // rbx
  int8 v5;
  char *v6; // r12
  char *v7; // r14
  
  if ((1 <= dat_82a98) && (*(int8 *)(a0 + 0x158))) {
    v4 = 0;
    do {
      v5 = *(int8 *)(*(int8 *)(a0 + 0x160) + v4 * 0x18);
      v7 = (char *)(v5 + strlen("SCHILY.xattr."));
      if (a2 != '0') { // branch-flip
label_32605:
        if (!sub_31e00(v7,0)) {
          v5 = v4 * 0x18 + *(int8 *)(a0 + 0x160);
          v1 = *(void **)(v5 + 8);
          if (v1) {
            if (a2 != '2') { // branch-flip
              v2 = sub_39140();
              v6 = "setxattrat";
            }
            else {
              v2 = sub_393c0(dat_7f234,a1,v7,v1,*(uint8 *)(v5 + 0x10),0);
              v6 = "lsetxattr";
            }
            if ((v2 == -1) && (dat_81b82 & 0x20)) {
              if (dat_82d50)
                (*dat_82d50)();
              v3 = dcgettext(NULL,"%s: Cannot set \'%s\' extended attribute for file \'%s\'",5);
              error(0,*__errno_location(),v3,v6,v7,a1);
            }
          }
        }
      }
      else if ((strcmp(v7,"security.capability") != 0) != a3) goto label_32605;
      v4 += 1;
    } while (v4 < *(uint8 *)(a0 + 0x158));
  }
  return;
}


// Function: sub_326f0 @ 0x326f0
void sub_326f0(int8 a0,unsigned short *a1) // return-dupe x3
{
  int8 v1;
  uint8 v2; // rax
  uint8 v3; // r13
  
  if (dat_82a54 <= 1) {
    *(char *)a1 = 0;
    return;
  }
  if (((1 <= dat_82a98) || (0 < dat_82aa0)) || (0 < dat_82a9c)) {
    *a1 = 0x20;
    if ((1 <= dat_82a98) && (*(int8 *)(a0 + 0x158))) {
      v3 = 0;
      do {
        v2 = strlen("SCHILY.xattr.");
        v1 = *(int8 *)(*(int8 *)(a0 + 0x160) + v3 * 0x18);
        if (!sub_31e00((char *)(v2 + v1),0)) {
          *(char *)a1 = 0x2a;
          if (dat_82aa0 <= 0) goto label_327ab;
          goto label_327a0;
        }
        v3 += 1;
      } while (v3 < *(uint8 *)(a0 + 0x158));
    }
    if (dat_82aa0 <= 0) {
      if (dat_82a9c <= 0)
        return;
      goto label_327b5;
    }
label_327a0:
    if (*(int8 *)(a0 + 0x30))
      *(char *)a1 = 0x2e;
  }
label_327ab:
  if (dat_82a9c <= 0)
    return;
label_327b5:
  if ((!*(int8 *)(a0 + 0x40)) && (!*(int8 *)(a0 + 0x50)))
    return;
  *(char *)a1 = 0x2b;
  return;
}


// Function: sub_32830 @ 0x32830
void sub_32830(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  char *v3; // rax
  uint8 v4;
  
  if (dat_82a54 <= 2)
    return;
  if ((1 <= dat_82aa0) && (*(char **)(a0 + 0x30)))
    __fprintf_chk(dat_81f28,1,"  s: %s\n",*(char **)(a0 + 0x30));
  if ((1 <= dat_82a9c) && ((*(int8 *)(a0 + 0x40) || (*(int8 *)(a0 + 0x50))))) {
    __fprintf_chk(dat_81f28,1,"  a: ");
    sub_31b90("",*(char **)(a0 + 0x38),*(uint8 *)(a0 + 0x40));
    v4 = *(uint8 *)(a0 + 0x50);
    if ((*(int8 *)(a0 + 0x40)) && (v4)) {
      __fprintf_chk(dat_81f28,1,",");
      v4 = *(uint8 *)(a0 + 0x50);
    }
    sub_31b90("default:",*(char **)(a0 + 0x48),v4);
    __fprintf_chk(dat_81f28,1,"\n");
  }
  if ((1 <= dat_82a98) && (*(int8 *)(a0 + 0x158))) {
    v4 = 0;
    do {
      while( true ) {
        v2 = strlen("SCHILY.xattr.");
        v1 = v4 * 0x18;
        v3 = (char *)(v2 + *(int8 *)(*(int8 *)(a0 + 0x160) + v4 * 0x18));
        if (sub_31e00(v3,0)) break;
        v4 += 1;
        __fprintf_chk(dat_81f28,1,"  x: %lu %s\n",*(unsigned long *)(*(int8 *)(a0 + 0x160) + 0x10 + v1),v3);
        if (*(uint8 *)(a0 + 0x158) <= v4)
          return;
      }
      v4 += 1;
    } while (v4 < *(uint8 *)(a0 + 0x158));
  }
  return;
}

