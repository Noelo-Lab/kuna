// Function: main @ 0x4d10
long long main(int a1, char **a2, char **a3)
{
  char *const *v4; // rbx
  char *v5; // rdi
  int v6; // eax
  long long v7; // r10
  char *v8; // rsi
  long long v9; // rdx
  char v10; // al
  long long *v11; // rax
  long long v12; // rdx
  char *v13; // rsi
  long long v14; // rdx
  char v15; // al
  unsigned int v16; // eax
  char *v17; // rax
  int v18; // eax
  int v19; // eax
  char *v20; // rsi
  unsigned int v21; // eax
  char v22; // al
  int v23; // ebp
  long long v24; // rax
  char *v25; // rbp
  char i; // al
  int v27; // eax
  long long v28; // rbp
  int v29; // eax
  unsigned long long v30; // rdi
  char v31; // al
  long long v32; // rax
  int v33; // r12d
  char *v34; // rdi
  const char *v35; // rsi
  void *v36; // rbp
  void *v37; // rdi
  long long v38; // rax
  long long v39; // rdx
  long long v40; // rax
  void *v41; // rdi
  long long v42; // rax
  char *j; // r12
  const char **v44; // rbp
  long long v45; // rdi
  char **v46; // rsi
  long long v47; // rax
  int v48; // eax
  int k; // ebx
  void *v50; // rbp
  char v52; // al
  char *v53; // rdi
  unsigned long long v54; // rax
  long long v55; // rdx
  long long v56; // rcx
  const char *v57; // rax
  char *v58; // rax
  char *v59; // rax
  char *v60; // rax
  const char *v61; // rbp
  char *const *v62; // r12
  const char *v63; // rbx
  int v64; // eax
  char *v65; // rax
  char *v66; // rbp
  int v67; // eax
  char *IO_write_ptr; // rax
  char *v69; // r12
  char v70; // al
  char v71; // al
  bool v72; // zf
  long long v73; // rbx
  const char *v74; // rsi
  long long *v75; // rcx
  long long v76; // r12
  char *v77; // rax
  char *v78; // rax
  void *v79; // r12
  void *v80; // rdi
  const char *v81; // r12
  char *v82; // rax
  char *v83; // rbp
  char *v84; // r13
  char *v85; // rax
  char *v86; // rbp
  long long v87; // r12
  char *v88; // rax
  char *v89; // rax
  char *v90; // rax
  char *v91; // rbp
  char *v92; // r12
  long long v93; // rax
  long long *v94; // r13
  void *v95; // rax
  FILE *v96; // r12
  char *v97; // rax
  const char *v98; // rcx
  FILE *v99; // rbp
  char *v100; // rax
  long long v101; // r12
  char *v102; // rax
  long long v103; // r12
  char *v104; // rax
  char *v105; // rax
  long long v106; // r12
  char *v107; // rax
  long long v108; // [rsp-8h] [rbp-A0h]
  int v109; // [rsp+0h] [rbp-98h]
  int v110; // [rsp+0h] [rbp-98h]
  char *const *v111; // [rsp+0h] [rbp-98h]
  int v112; // [rsp+8h] [rbp-90h]
  int v113; // [rsp+Ch] [rbp-8Ch]
  long long v114; // [rsp+10h] [rbp-88h]
  const char *s1; // [rsp+18h] [rbp-80h]
  long long v116; // [rsp+20h] [rbp-78h]
  int v117; // [rsp+28h] [rbp-70h]
  char v118; // [rsp+2Fh] [rbp-69h]
  char *v119; // [rsp+38h] [rbp-60h] BYREF
  int longind[2]; // [rsp+40h] [rbp-58h] BYREF
  long long v121; // [rsp+48h] [rbp-50h]
  char v122[3]; // [rsp+55h] [rbp-43h] BYREF
  unsigned long long v123; // [rsp+58h] [rbp-40h]

  v4 = a2;
  v5 = *a2;
  v123 = __readfsqword(0x28u);
  sub_150F0(v5);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  status = 2;
  sub_19D40(sub_F770);
  byte_262B8 = 1;
  dword_26210 = 0;
  ptr = 0;
  tp.tv_sec = 0x8000000000000000LL;
  tp.tv_nsec = -1;
  v114 = -1;
  v116 = -1;
  v113 = -1;
  v112 = -1;
  v117 = -1;
  v109 = -1;
  v118 = 0;
  s1 = 0;
  while ( 2 )
  {
    longind[0] = -1;
    v6 = getopt_long(a1, v4, "abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1", &longopts, longind);
    if ( v6 != -1 )
    {
      switch ( v6 )
      {
        case -131:
          sub_183C0(stdout, 0);
          exit(0);
        case -130:
          sub_DF00(0);
        case 49:
          v109 = v109 != 0;
          continue;
        case 65:
          dword_262F0 = 1;
          continue;
        case 66:
          sub_6BE0("*~");
          sub_6BE0(".*~");
          continue;
        case 67:
          v109 = 2;
          continue;
        case 68:
          byte_26318 = 1;
          continue;
        case 70:
          v20 = optarg;
          if ( !optarg
            || (v108 = v7,
                v21 = dword_1A6E0[sub_ECF0("--classify", optarg, &off_241E0, dword_1A6E0, 4, off_251F0, 1)],
                v21 == 1)
            || v21 == 2 && (unsigned char)sub_6D10("--classify", v20, v108) )
          {
            dword_26314 = 3;
          }
          continue;
        case 71:
          byte_25028 = 0;
          continue;
        case 72:
          dword_262F8 = 2;
          continue;
        case 73:
          sub_6BE0(optarg);
          continue;
        case 76:
          dword_262F8 = 4;
          continue;
        case 78:
          v112 = 0;
          continue;
        case 81:
          v112 = 5;
          continue;
        case 82:
          byte_262F6 = 1;
          continue;
        case 83:
          v113 = 3;
          continue;
        case 84:
          v19 = (unsigned int)dcgettext(0, "invalid tab size", 5);
          v116 = sub_18AD0((int)optarg, 0, 0, -1, (int)"", v19, 2);
          continue;
        case 85:
          v113 = 6;
          continue;
        case 88:
          v113 = 1;
          continue;
        case 90:
          byte_26365 = 1;
          continue;
        case 97:
          dword_262F0 = 2;
          continue;
        case 98:
          v112 = 7;
          continue;
        case 99:
          dword_26334 = 1;
          continue;
        case 100:
          byte_262F5 = 1;
          continue;
        case 102:
          v18 = -1;
          dword_262F0 = 2;
          byte_26312 = 0;
          byte_26311 = 0;
          if ( v109 )
            v18 = v109;
          byte_2632C = 0;
          v113 = 6;
          v109 = v18;
          continue;
        case 103:
          byte_25029 = 0;
          v109 = 0;
          continue;
        case 104:
          dword_26328 = 176;
          dword_2631C = 176;
          qword_26320 = 1;
          qword_25020 = 1;
          continue;
        case 105:
          byte_262FC = 1;
          continue;
        case 107:
          v118 = 1;
          continue;
        case 108:
          v109 = 0;
          continue;
        case 109:
          v109 = 4;
          continue;
        case 110:
          byte_2632D = 1;
          v109 = 0;
          continue;
        case 111:
          byte_25028 = 0;
          v109 = 0;
          continue;
        case 112:
          dword_26314 = 1;
          continue;
        case 113:
          v117 = 1;
          continue;
        case 114:
          byte_2632F = 1;
          continue;
        case 115:
          byte_2632C = 1;
          continue;
        case 116:
          v113 = 5;
          continue;
        case 117:
          dword_26334 = 2;
          continue;
        case 118:
          goto LABEL_32;
        case 119:
          v114 = sub_6C10(optarg);
          if ( v114 >= 0 )
            continue;
          v4 = (char *const *)sub_174C0(optarg);
          v17 = dcgettext(0, "invalid line width", 5);
          error(2, 0, "%s: %s", v17, (const char *)v4);
LABEL_32:
          v113 = 4;
          continue;
        case 120:
          v109 = 3;
          continue;
        case 128:
          byte_2632E = 1;
          continue;
        case 129:
          v16 = sub_12150(optarg);
          if ( v16 )
            sub_18D20(v16, (unsigned int)longind[0], 0, &longopts, optarg);
          dword_2631C = dword_26328;
          qword_25020 = qword_26320;
          continue;
        case 130:
          v13 = optarg;
          if ( !optarg
            || (v14 = dword_1A6E0[sub_ECF0("--color", optarg, &off_241E0, dword_1A6E0, 4, off_251F0, 1)],
                (int)v14 == 1) )
          {
            v15 = 1;
          }
          else
          {
            v15 = 0;
            if ( (int)v14 == 2 )
              v15 = sub_6D10("--color", v13, v14);
          }
          byte_26312 = v15 & 1;
          continue;
        case 131:
          dword_262F8 = 3;
          continue;
        case 132:
          dword_26314 = 2;
          continue;
        case 133:
          v109 = dword_1A750[sub_ECF0("--format", optarg, off_242C0, dword_1A750, 4, off_251F0, 1)];
          continue;
        case 134:
          v109 = 0;
          s1 = "full-iso";
          continue;
        case 135:
          byte_262F4 = 1;
          continue;
        case 136:
          v11 = (long long *)sub_18590(16);
          *v11 = optarg;
          v12 = qword_262E0;
          qword_262E0 = (long long)v11;
          v11[1] = v12;
          continue;
        case 137:
          v8 = optarg;
          if ( !optarg
            || (v9 = dword_1A6E0[sub_ECF0("--hyperlink", optarg, &off_241E0, dword_1A6E0, 4, off_251F0, 1)],
                (int)v9 == 1) )
          {
            v10 = 1;
          }
          else
          {
            v10 = 0;
            if ( (int)v9 == 2 )
              v10 = sub_6D10("--hyperlink", v8, v9);
          }
          byte_26311 = v10 & 1;
          continue;
        case 138:
          dword_26314 = dword_1A770[sub_ECF0("--indicator-style", optarg, off_24980, dword_1A770, 4, off_251F0, 1)];
          continue;
        case 139:
          v112 = dword_1E9C0[sub_ECF0("--quoting-style", optarg, off_24A20, dword_1E9C0, 4, off_251F0, 1)];
          continue;
        case 140:
          goto LABEL_5;
        case 141:
          dword_26328 = 144;
          dword_2631C = 144;
          qword_26320 = 1;
          qword_25020 = 1;
          continue;
        case 142:
          v113 = dword_1A730[sub_ECF0("--sort", optarg, &off_24280, dword_1A730, 4, off_251F0, 1)];
          continue;
        case 143:
          dword_26334 = dword_1A710[sub_ECF0("--time", optarg, off_24240, dword_1A710, 4, off_251F0, 1)];
          continue;
        case 144:
          s1 = optarg;
          continue;
        case 145:
          byte_25019 = 0;
          byte_26312 = 0;
          v112 = 0;
          v109 = v109 != 0;
LABEL_5:
          v117 = 0;
          continue;
        default:
          goto LABEL_289;
      }
    }
    break;
  }
  if ( !qword_26320 )
  {
    v53 = getenv("LS_BLOCK_SIZE");
    sub_12150(v53);
    if ( v53 || getenv("BLOCK_SIZE") )
    {
      dword_2631C = dword_26328;
      qword_25020 = qword_26320;
    }
    if ( v118 )
    {
      dword_26328 = 0;
      qword_26320 = 1024;
    }
  }
  if ( v109 >= 0 )
  {
    dword_26338 = v109;
    if ( (unsigned int)(v109 - 2) <= 2 )
      goto LABEL_81;
    goto LABEL_156;
  }
  if ( dword_251E0 == 1 )
  {
    if ( !(unsigned char)((long long (*)(void))sub_6D10)() )
    {
      v48 = 1;
      goto LABEL_155;
    }
LABEL_268:
    dword_26338 = 2;
    goto LABEL_81;
  }
  if ( dword_251E0 == 2 )
    goto LABEL_268;
  v48 = 0;
LABEL_155:
  dword_26338 = v48;
LABEL_156:
  if ( !byte_26312 )
  {
    if ( v114 != -1 )
      goto LABEL_82;
    goto LABEL_158;
  }
LABEL_81:
  if ( v114 != -1 )
    goto LABEL_82;
  if ( !(unsigned char)((long long (*)(void))sub_6D10)() || ioctl(1, 0x5413u, longind) < 0 || !HIWORD(longind[0]) )
  {
    v85 = getenv("COLUMNS");
    v86 = v85;
    if ( v85 && *v85 )
    {
      v114 = sub_6C10(v85);
      if ( v114 >= 0 )
        goto LABEL_82;
      v87 = sub_174C0(v86);
      v88 = dcgettext(0, "ignoring invalid width in environment variable COLUMNS: %s", 5);
      error(0, 0, v88, v87);
    }
LABEL_158:
    v114 = 80;
    goto LABEL_82;
  }
  v114 = HIWORD(longind[0]);
LABEL_82:
  qword_262B0 = v114;
  qword_26200 = v114 / 3uLL
              + (v114 != v114 / 3uLL
                       + (((0xAAAAAAAAAAAAAAABLL * (unsigned __int128)(unsigned long long)v114) >> 64)
                        & 0xFFFFFFFFFFFFFFFELL));
  if ( (unsigned int)(dword_26338 - 2) > 2 )
    goto LABEL_83;
  v42 = v116;
  if ( v116 >= 0 )
    goto LABEL_138;
  qword_262C0 = 8;
  v90 = getenv("TABSIZE");
  v91 = v90;
  if ( v90 )
  {
    if ( !(unsigned int)sub_18DE0(v90) )
    {
      v42 = *(long long *)longind;
LABEL_138:
      qword_262C0 = v42;
      goto LABEL_83;
    }
    v101 = sub_174C0(v91);
    v102 = dcgettext(0, "ignoring invalid tab size in environment variable TABSIZE: %s", 5);
    error(0, 0, v102, v101);
  }
LABEL_83:
  v22 = v117 & 1;
  if ( v117 == -1 )
  {
    v52 = 0;
    if ( dword_251E0 == 1 )
      v52 = ((long long (*)(void))sub_6D10)();
    v22 = v52 & 1;
  }
  byte_262D8 = v22;
  if ( v112 >= 0 )
    goto LABEL_85;
  v65 = getenv("QUOTING_STYLE");
  v66 = v65;
  if ( v65 )
  {
    v67 = sub_E9C0(v65);
    if ( v67 < 0 )
    {
      v106 = sub_174C0(v66);
      v107 = dcgettext(0, "ignoring invalid value of environment variable QUOTING_STYLE: %s", 5);
      error(0, 0, v107, v106);
    }
    else
    {
      v112 = dword_1E9C0[v67];
      if ( v112 >= 0 )
      {
LABEL_85:
        sub_16950(0, (unsigned int)v112);
        goto LABEL_86;
      }
    }
  }
  v112 = 7;
  if ( dword_251E0 != 1 )
    goto LABEL_85;
  if ( (unsigned char)((long long (*)(void))sub_6D10)() )
  {
    v112 = 3;
    goto LABEL_85;
  }
LABEL_86:
  v23 = sub_16930(0);
  if ( (!dword_26338 || (unsigned int)(dword_26338 - 2) <= 1 && qword_262B0) && (v23 == 3 || v23 == 6 || v23 == 1) )
  {
    byte_263A8 = 1;
    qword_262D0 = sub_168F0(0);
  }
  else
  {
    byte_263A8 = 0;
    v24 = sub_168F0(0);
    qword_262D0 = v24;
    if ( v23 == 7 )
      sub_16970(v24, 32, 1);
  }
  if ( (unsigned int)dword_26314 > 1 )
  {
    v25 = &asc_1BD23[dword_26314 - 2];
    for ( i = *v25; *v25; i = *v25 )
    {
      ++v25;
      sub_16970(qword_262D0, (unsigned int)i, 1);
    }
  }
  qword_262C8 = sub_168F0(0);
  sub_16970(qword_262C8, 58, 1);
  byte_26318 &= (dword_26338 == 0) & (byte_26311 ^ 1);
  if ( (unsigned char)byte_26318 > byte_25019 )
    goto LABEL_306;
  v27 = v113;
  if ( v113 < 0 )
  {
    if ( !dword_26338 )
    {
      dword_26330 = 0;
      goto LABEL_141;
    }
    if ( (unsigned int)(dword_26334 - 1) <= 2 )
    {
      dword_26330 = 5;
      goto LABEL_98;
    }
    v27 = 0;
  }
  dword_26330 = v27;
  if ( !dword_26338 )
  {
LABEL_141:
    if ( !s1 )
    {
      s1 = getenv("TIME_STYLE");
      if ( !s1 )
        s1 = "locale";
    }
    for ( j = (char *)s1; !strncmp(j, "posix-", 6u); j += 6 )
    {
      if ( !(unsigned char)sub_10260(2) )
        goto LABEL_98;
    }
    if ( *j != 43 )
    {
      v44 = (const char **)off_249C0;
      v45 = (long long)j;
      v46 = off_249C0;
      v47 = sub_E9C0(j);
      if ( v47 < 0 )
      {
        sub_EB30("time style", j, v47);
        v96 = stderr;
        v97 = dcgettext(0, "Valid arguments are:\n", 5);
        fputs_unlocked(v97, v96);
        v98 = "full-iso";
        do
        {
          ++v44;
          __fprintf_chk(stderr, 1, "  - [posix-]%s\n", v98);
          v98 = *v44;
        }
        while ( *v44 );
        v99 = stderr;
        v100 = dcgettext(0, "  - +FORMAT (e.g., +%H:%M) for a 'date'-style format\n", 5);
        fputs_unlocked(v100, v99);
LABEL_289:
        sub_DF00(2);
      }
      if ( v47 == 2 )
      {
        off_25040 = "%Y-%m-%d ";
        off_25048 = "%m-%d %H:%M";
      }
      else if ( v47 > 2 )
      {
        if ( v47 == 3 )
        {
          v45 = 2;
          if ( (unsigned char)sub_10260(2) )
          {
            v89 = dcgettext(0, off_25040, 2);
            v46 = (char **)off_25048;
            v45 = 0;
            off_25040 = v89;
            off_25048 = dcgettext(0, off_25048, 2);
          }
        }
      }
      else if ( v47 )
      {
        off_25048 = "%Y-%m-%d %H:%M";
        off_25040 = "%Y-%m-%d %H:%M";
      }
      else
      {
        off_25048 = "%Y-%m-%d %H:%M:%S.%N %z";
        off_25040 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      goto LABEL_258;
    }
    v81 = j + 1;
    v46 = (char **)(byte_9 + 1);
    v45 = (long long)v81;
    v82 = strchr(v81, 10);
    v83 = v82;
    if ( !v82 )
    {
      v84 = (char *)v81;
      goto LABEL_257;
    }
    v84 = v82 + 1;
    v46 = (char **)(byte_9 + 1);
    v45 = (long long)(v82 + 1);
    if ( !strchr(v82 + 1, 10) )
    {
      *v83 = 0;
LABEL_257:
      off_25040 = (char *)v81;
      off_25048 = v84;
LABEL_258:
      sub_6D40(v45, v46);
      goto LABEL_98;
    }
    v103 = sub_174C0(v81);
    v104 = dcgettext(0, "invalid time style format %s", 5);
    error(2, 0, v104, v103);
LABEL_306:
    v105 = dcgettext(0, "--dired and --zero are incompatible", 5);
    error(2, 0, v105);
LABEL_307:
    __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)", "src/ls.c", 0x41Du, "dev_ino_pop");
  }
LABEL_98:
  v28 = optind;
  if ( !byte_26312 )
    goto LABEL_99;
  v58 = getenv("LS_COLORS");
  v119 = v58;
  if ( v58 && *v58 )
  {
    strcpy(v122, "??");
    v111 = v4;
    qword_26300 = (void *)sub_18A70(v58);
    *(long long *)longind = qword_26300;
    while ( 1 )
    {
LABEL_232:
      while ( 1 )
      {
        v69 = v119;
        v70 = *v119;
        if ( *v119 != 42 )
          break;
        v92 = v119 + 1;
        v93 = sub_18590(40);
        v119 = v92;
        v94 = (long long *)v93;
        v95 = qword_26308;
        qword_26308 = v94;
        v94[4] = v95;
        v94[1] = *(long long *)longind;
        if ( (unsigned char)sub_6960(longind, &v119, 1, v94) )
        {
          v72 = *v119++ == 61;
          if ( v72 )
          {
            v94[3] = *(long long *)longind;
            if ( (unsigned char)sub_6960(longind, &v119, 0, v94 + 2) )
              continue;
          }
        }
        goto LABEL_280;
      }
      if ( v70 != 58 )
        break;
      ++v119;
    }
    if ( !v70 )
    {
      v4 = v111;
      goto LABEL_245;
    }
    v122[0] = *v119;
    v71 = *++v119;
    if ( !v71 || (v122[1] = v71, v72 = v69[2] == 61, v119 = v69 + 3, !v72) )
    {
LABEL_280:
      v4 = v111;
      goto LABEL_242;
    }
    v73 = 0;
    v74 = "lc";
    while ( strcmp(v122, v74) )
    {
      v74 = off_248A0[++v73];
      if ( !v74 )
        goto LABEL_241;
    }
    v75 = &qword_25060[2 * (int)v73];
    v75[1] = *(long long *)longind;
    if ( (unsigned char)sub_6960(longind, &v119, 0, v75) )
      goto LABEL_232;
LABEL_241:
    v4 = v111;
    v76 = sub_174C0(v122);
    v77 = dcgettext(0, "unrecognized prefix: %s", 5);
    error(0, 0, v77, v76);
LABEL_242:
    v78 = dcgettext(0, "unparsable value for LS_COLORS environment variable", 5);
    error(0, 0, v78);
    free(qword_26300);
    v79 = qword_26308;
    while ( v79 )
    {
      v80 = v79;
      v79 = (void *)*((long long *)v79 + 4);
      free(v80);
    }
    byte_26312 = 0;
LABEL_245:
    if ( qword_250D0 == 6 && !strncmp(off_250D8, "target", 6u) )
      byte_26390 = 1;
  }
  else
  {
    v59 = getenv("COLORTERM");
    if ( !v59 || !*v59 )
    {
      v60 = getenv("TERM");
      if ( v60 && *v60 )
      {
        v110 = v28;
        v61 = "# Configuration file for dircolors, a utility to help you set the";
        v62 = v4;
        v63 = v60;
        do
        {
          if ( !strncmp(v61, "TERM ", 5u) && !fnmatch(v61 + 5, v63, 0) )
          {
            v28 = v110;
            v4 = v62;
            goto LABEL_246;
          }
          v61 += strlen(v61) + 1;
        }
        while ( (unsigned long long)(v61 - "# Configuration file for dircolors, a utility to help you set the") <= 0x12C7 );
        v28 = v110;
        v4 = v62;
      }
      byte_26312 = 0;
    }
  }
LABEL_246:
  if ( !byte_26312 )
  {
LABEL_99:
    if ( !byte_262F4 )
      goto LABEL_101;
    goto LABEL_100;
  }
  qword_262C0 = 0;
  if ( byte_262F4
    || (unsigned char)sub_6C80(13)
    || (unsigned char)sub_6C80(14) && byte_26390
    || (unsigned char)sub_6C80(12) && !dword_26338 )
  {
LABEL_100:
    byte_262FD = 1;
  }
LABEL_101:
  if ( !dword_262F8 )
  {
    v29 = 1;
    if ( !byte_262F5 && dword_26314 != 3 )
      v29 = dword_26338 == 0 ? 1 : 3;
    dword_262F8 = v29;
  }
  if ( byte_262F6 )
  {
    qword_263C8 = (void *)sub_10C60(30, 0, sub_6910, sub_6920, j_free);
    if ( !qword_263C8 )
      sub_18A90();
    obstack_begin(&unk_260E0, 0, 0, &malloc, &free);
  }
  v30 = (unsigned long long)getenv("TZ");
  qword_262A8 = sub_17950((void *)v30);
  if ( ((dword_26330 - 3) & 0xFFFFFFFD) == 0 || !dword_26338 || byte_26365 || byte_2632C )
  {
    byte_262A1 = 1;
    v31 = 0;
  }
  else
  {
    byte_262A1 = 0;
    v31 = 1;
    if ( !byte_262F6 && !byte_26312 && !dword_26314 )
      v31 = byte_262F4;
  }
  byte_262A0 = v31 & 1;
  if ( byte_26318 )
  {
    obstack_begin(&unk_261A0, 0, 0, &malloc, &free);
    v30 = (unsigned long long)&unk_26140;
    obstack_begin(&unk_26140, 0, 0, &malloc, &free);
  }
  if ( byte_26311 )
  {
    v54 = 0;
    while ( 1 )
    {
      v56 = (unsigned int)v54;
      if ( v54 <= 0x5A )
      {
        v55 = 1;
        if ( (int)v54 > 64 )
          goto LABEL_197;
        v30 = (unsigned int)(v54 - 48);
        if ( (unsigned int)v30 <= 9 )
          goto LABEL_197;
        v56 = (unsigned int)(v54 - 45);
        if ( (unsigned int)v56 > 1 )
          goto LABEL_195;
LABEL_201:
        v55 = 1;
        byte_25FE0[v54++] |= 1u;
        if ( v54 == 256 )
        {
LABEL_202:
          v57 = (const char *)sub_18C10(v30, byte_25FE0, v55, v56);
          if ( !v57 )
            v57 = "";
          qword_26388 = (long long)v57;
          break;
        }
      }
      else
      {
        v30 = (unsigned int)(v54 - 97);
        v55 = 1;
        if ( (unsigned int)v30 <= 0x19 )
          goto LABEL_197;
        v56 = (unsigned int)(v54 - 45);
        if ( (unsigned int)v56 <= 1 )
          goto LABEL_201;
LABEL_195:
        if ( (int)v54 == 126 )
          goto LABEL_201;
        LOBYTE(v55) = (int)v54 == 95;
LABEL_197:
        byte_25FE0[v54++] |= v55;
        if ( v54 == 256 )
          goto LABEL_202;
      }
    }
  }
  qword_263B8 = 100;
  v32 = sub_186D0(100, 208);
  v33 = a1 - v28;
  qword_263B0 = 0;
  qword_263C0 = v32;
  sub_8730();
  if ( a1 - (int)v28 <= 0 )
  {
    if ( byte_262F5 )
    {
      v35 = (char *)(&dword_0 + 3);
      v34 = ".";
      sub_96F0(".");
    }
    else
    {
      v35 = 0;
      v34 = ".";
      sub_6FB0(".", 0);
    }
    v36 = ptr;
    if ( !qword_263B0 )
      goto LABEL_216;
  }
  else
  {
    do
    {
      v34 = v4[v28];
      v35 = 0;
      ++v28;
      sub_96F0(v34);
    }
    while ( a1 > (int)v28 );
    if ( !qword_263B0 )
      goto LABEL_117;
  }
  sub_8E30(v34, v35);
  if ( !byte_262F5 )
  {
    v35 = (char *)(&dword_0 + 1);
    sub_9510(0);
  }
  if ( !qword_263B0 )
  {
LABEL_117:
    v36 = ptr;
    if ( v33 > 1 )
      goto LABEL_121;
LABEL_216:
    if ( v36 )
    {
      if ( !*((long long *)v36 + 3) )
        byte_262B8 = 0;
      goto LABEL_122;
    }
    goto LABEL_168;
  }
  sub_D420();
  if ( ptr )
  {
    ++qword_261F8;
    IO_write_ptr = stdout->_IO_write_ptr;
    if ( IO_write_ptr >= stdout->_IO_write_end )
    {
      v35 = byte_9 + 1;
      __overflow(stdout, 10);
      v36 = ptr;
      goto LABEL_121;
    }
    v36 = ptr;
    stdout->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = 10;
    do
    {
LABEL_122:
      v37 = *(void **)v36;
      ptr = (void *)*((long long *)v36 + 3);
      if ( !qword_263C8 || v37 )
      {
        v35 = (const char *)*((long long *)v36 + 1);
        sub_D820(v37);
        free(*(void **)v36);
        free(*((void **)v36 + 1));
        free(v36);
        byte_262B8 = 1;
      }
      else
      {
        v38 = qword_260F8;
        if ( (unsigned long long)(qword_260F8 - qword_260F0) <= 0xF )
          goto LABEL_307;
        v35 = (const char *)longind;
        qword_260F8 -= 16;
        v39 = *(long long *)(v38 - 16);
        v40 = *(long long *)(v38 - 8);
        *(long long *)longind = v39;
        v121 = v40;
        v41 = (void *)sub_11320(qword_263C8, longind);
        if ( !v41 )
          __assert_fail("found", "src/ls.c", 0x70Du, "main");
        free(v41);
        free(*(void **)v36);
        free(*((void **)v36 + 1));
        free(v36);
      }
      v36 = ptr;
LABEL_121:
      ;
    }
    while ( v36 );
  }
LABEL_168:
  if ( byte_26312 && byte_26310 )
  {
    if ( qword_25060[0] != 2
      || (v35 = "\x1B[", memcmp(::s1, "\x1B[", 2u))
      || qword_25070 != 1
      || *(char *)off_25078 != 109 )
    {
      sub_7880(qword_25060, v35);
      sub_7880(&qword_25070, v35);
    }
    fflush_unlocked(stdout);
    sub_76B0(0);
    for ( k = dword_26214; k; --k )
      raise(19);
    if ( sig )
      raise(sig);
  }
  if ( byte_26318 )
  {
    sub_7450("//DIRED//", &unk_261A0);
    sub_7450("//SUBDIRED//", &unk_26140);
    v64 = sub_16930(qword_262D0);
    v35 = "//DIRED-OPTIONS// --quoting-style=%s\n";
    __printf_chk(1, "//DIRED-OPTIONS// --quoting-style=%s\n", off_24A20[v64]);
  }
  v50 = qword_263C8;
  if ( qword_263C8 )
  {
    if ( sub_10790(qword_263C8, v35) )
      __assert_fail("hash_get_n_entries (active_dir_set) == 0", "src/ls.c", 0x741u, "main");
    sub_10E20(v50);
  }
  return (unsigned int)dword_26210;
}



// Function: dev_ino_hash @ 0x6910
unsigned long long dev_ino_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: dev_ino_compare @ 0x6920
bool dev_ino_compare(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( *a1 == *a2 )
    return a1[1] == a2[1];
  return result;
}



// Function: sighandler @ 0x6940
long long sighandler(int a1)
{
  long long result; // rax

  result = (unsigned int)sig;
  if ( !sig )
    sig = a1;
  return result;
}



// Function: get_funky_string @ 0x6960
long long get_funky_string(long long *a1, unsigned char **a2, unsigned int a3, long long *a4)
{
  long long result; // rax
  char v7; // r12
  unsigned char *v9; // rcx
  long long v10; // rbx
  long long v11; // rdi
  long long v12; // r8
  long long v13; // rsi
  long long v14; // r9
  bool v15; // cc
  char v16; // dl
  unsigned char *v17; // r13
  unsigned char v18; // dl
  unsigned char v19; // r8
  int v20; // r8d
  unsigned char *v21; // r9

  result = a3;
  v7 = a3;
  v9 = *a2;
  v10 = *a1;
  v11 = 1;
  LOBYTE(a3) = **a2;
  v12 = 0;
  v13 = v10 + 1;
  v14 = v10;
  v15 = (char)a3 <= 92;
  if ( (char)a3 == 92 )
    goto LABEL_8;
LABEL_2:
  if ( v15 )
  {
    if ( (char)a3 == 61 )
    {
      if ( v7 )
        goto LABEL_13;
    }
    else if ( (char)a3 <= 61 && (!(char)a3 || (char)a3 == 58) )
    {
      result = 1;
      goto LABEL_13;
    }
LABEL_5:
    ++v9;
    goto LABEL_6;
  }
  if ( (char)a3 != 94 )
    goto LABEL_5;
  v18 = v9[1];
  if ( (unsigned char)(v18 - 64) <= 0x3Eu )
  {
    v9 += 2;
    ++v11;
    *(char *)(++v13 - 2) = v18 & 0x1F;
    goto LABEL_7;
  }
  ++v9;
  if ( v18 == 63 )
  {
    ++v11;
    *(char *)(++v13 - 2) = 127;
    while ( 1 )
    {
LABEL_7:
      LOBYTE(a3) = *v9;
      v14 = v13 - 1;
      v12 = v11 - 1;
      v15 = (char)*v9 <= 92;
      if ( *v9 != 92 )
        goto LABEL_2;
LABEL_8:
      v16 = v9[1];
      v17 = v9 + 2;
      if ( !v16 )
        break;
      switch ( v16 )
      {
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
          v9 += 2;
          LOBYTE(a3) = v16 - 48;
          v19 = *v17;
          if ( (unsigned char)(*v17 - 48) > 7u )
            goto LABEL_6;
          do
          {
            ++v9;
            LOBYTE(a3) = v19 + 8 * a3 - 48;
            v19 = *v9;
          }
          while ( (unsigned char)(*v9 - 48) <= 7u );
          *(char *)(v13 - 1) = a3;
          ++v11;
          ++v13;
          continue;
        case '?':
          v16 = 127;
          v9 += 2;
          goto LABEL_19;
        case 'X':
        case 'x':
          v20 = v9[2];
          v21 = v9 + 3;
          a3 = 0;
          v9 += 2;
          if ( (char)v20 <= 70 )
            goto LABEL_29;
          break;
        case '_':
          v16 = 32;
          v9 += 2;
          goto LABEL_19;
        case 'a':
          v16 = 7;
          goto LABEL_18;
        case 'b':
          v16 = 8;
          goto LABEL_18;
        case 'e':
          v16 = 27;
          v9 += 2;
          goto LABEL_19;
        case 'f':
          v16 = 12;
          v9 += 2;
          goto LABEL_19;
        case 'n':
          v16 = 10;
          v9 += 2;
          goto LABEL_19;
        case 'r':
          v16 = 13;
          v9 += 2;
          goto LABEL_19;
        case 't':
          v16 = 9;
          v9 += 2;
          goto LABEL_19;
        case 'v':
          v16 = 11;
          goto LABEL_18;
        default:
LABEL_18:
          v9 += 2;
LABEL_19:
          *(char *)(v13 - 1) = v16;
          ++v11;
          ++v13;
          continue;
      }
      while ( (unsigned char)(v20 - 97) <= 5u )
      {
        a3 = v20 + 16 * a3 - 87;
        while ( 1 )
        {
          v20 = *v21++;
          v9 = v21 - 1;
          if ( (char)v20 > 70 )
            break;
LABEL_29:
          if ( (char)v20 > 64 )
          {
            a3 = v20 + 16 * a3 - 55;
          }
          else
          {
            if ( (unsigned char)(v20 - 48) > 9u )
              goto LABEL_6;
            a3 = v20 + 16 * a3 - 48;
          }
        }
      }
LABEL_6:
      *(char *)(v13 - 1) = a3;
      ++v11;
      ++v13;
    }
    v9 += 2;
    result = 0;
  }
  else
  {
    result = 0;
  }
LABEL_13:
  *a1 = v14;
  *a2 = v9;
  *a4 = v12;
  return result;
}



// Function: add_ignore_pattern @ 0x6be0
long long *add_ignore_pattern(long long a1)
{
  long long *result; // rax
  long long v2; // rdx

  result = (long long *)sub_18590(16);
  v2 = qword_262E8;
  *result = a1;
  result[1] = v2;
  qword_262E8 = (long long)result;
  return result;
}



// Function: decode_line_length @ 0x6c10
long long decode_line_length(char *a1)
{
  int v1; // eax
  long long result; // rax
  long long v3; // [rsp+0h] [rbp-18h]

  v1 = sub_18DE0(a1);
  if ( v1 )
    return -(long long)(v1 != 1);
  result = v3;
  if ( v3 < 0 )
    return 0;
  return result;
}



// Function: is_colored @ 0x6c80
long long is_colored(unsigned int a1)
{
  long long *v1; // rax
  long long v2; // rdx
  const char *v3; // rdi
  unsigned int v5; // r8d
  unsigned int v6; // r8d

  v1 = &qword_25060[2 * a1];
  v2 = *v1;
  if ( !*v1 )
    return 0;
  v3 = (const char *)v1[1];
  if ( v2 == 1 )
  {
    LOBYTE(v5) = strncmp(v3, "0", 1u) != 0;
    return v5;
  }
  else if ( v2 == 2 )
  {
    LOBYTE(v6) = strncmp(v3, "00", 2u) != 0;
    return v6;
  }
  else
  {
    return 1;
  }
}



// Function: stdout_isatty @ 0x6d10
long long stdout_isatty()
{
  char v0; // al

  v0 = byte_25018;
  if ( byte_25018 < 0 )
  {
    v0 = isatty(1);
    byte_25018 = v0;
  }
  return v0 & 1;
}



// Function: abformat_init @ 0x6d40
unsigned long long abformat_init()
{
  long long v0; // rsi
  char *v1; // rax
  char i; // dl
  char v3; // cl
  char *v4; // rbx
  void *v5; // r12
  unsigned long long v6; // rbp
  char *v7; // r13
  const char *v9; // rbx
  const char *v10; // rbp
  long long v11; // r14
  char *v12; // r15
  unsigned int v13; // eax
  long long v14; // [rsp+10h] [rbp-678h]
  long long v15[2]; // [rsp+30h] [rbp-658h]
  char v16[1536]; // [rsp+40h] [rbp-648h] BYREF
  char v17; // [rsp+640h] [rbp-48h] BYREF
  unsigned long long v18; // [rsp+648h] [rbp-40h]

  v0 = 0;
  v18 = __readfsqword(0x28u);
  do
  {
    v1 = (&off_25040)[v0];
    for ( i = *v1; i; ++v1 )
    {
      v3 = v1[1];
      if ( i == 37 )
      {
        if ( v3 == 37 )
        {
          i = v1[2];
          ++v1;
          continue;
        }
        if ( v3 == 98 )
          goto LABEL_9;
      }
      i = v1[1];
    }
    v1 = 0;
LABEL_9:
    v15[v0++] = v1;
  }
  while ( v0 != 2 );
  if ( v15[0] || v15[1] )
  {
    v4 = v16;
    v5 = &unk_2000E;
    v6 = 0;
    while ( 1 )
    {
      v7 = nl_langinfo((nl_item)v5);
      if ( strchr(v7, 37) )
        break;
      __ctype_b_loc();
      if ( (unsigned long long)sub_12810(v7) > 0x7F )
        break;
      if ( v6 < 0xC )
        v6 = 12;
      LODWORD(v5) = (int)v5 + 1;
      v4 += 128;
      if ( v4 == &v17 )
      {
        v14 = 0;
LABEL_23:
        v9 = v16;
        v10 = (&off_25040)[v14];
        v11 = v15[v14];
        v12 = (char *)&unk_253E0 + 1536 * v14;
        while ( 1 )
        {
          if ( v11 )
          {
            if ( v11 - (long long)v10 > 128 )
              return v18 - __readfsqword(0x28u);
            v13 = __snprintf_chk(v12, 128, 1, -1, "%.*s%s%s", (int)v11 - (int)v10, v10, v9, (const char *)(v11 + 2));
          }
          else
          {
            v13 = snprintf(v12, 0x80u, "%s", v10);
          }
          if ( v13 > 0x7F )
            return v18 - __readfsqword(0x28u);
          v9 += 128;
          v12 += 128;
          if ( v9 == &v17 )
          {
            if ( v14 == 1 )
            {
              byte_253C8 = 1;
              return v18 - __readfsqword(0x28u);
            }
            v14 = 1;
            goto LABEL_23;
          }
        }
      }
    }
  }
  return v18 - __readfsqword(0x28u);
}



// Function: do_statx @ 0x7050
long long do_statx(long long a1, long long a2, long long a3, unsigned int a4, unsigned int a5)
{
  short v5; // bp
  long long result; // rax
  long long v8; // rcx
  long long v9; // rdx
  long long v10; // rcx
  long long v11; // rsi
  long long v12; // rdx
  long long v13; // rsi
  long long v14; // rdx
  long long v15; // rsi
  long long v16; // rdx
  long long v17; // rsi
  long long v18; // rdx
  long long v19; // rdi
  char v20[4]; // [rsp+0h] [rbp-128h] BYREF
  unsigned int v21; // [rsp+4h] [rbp-124h]
  unsigned int v22; // [rsp+10h] [rbp-118h]
  long long v23; // [rsp+14h] [rbp-114h]
  unsigned short v24; // [rsp+1Ch] [rbp-10Ch]
  long long v25; // [rsp+20h] [rbp-108h]
  long long v26; // [rsp+28h] [rbp-100h]
  long long v27; // [rsp+30h] [rbp-F8h]
  long long v28; // [rsp+40h] [rbp-E8h]
  unsigned int v29; // [rsp+48h] [rbp-E0h]
  long long v30; // [rsp+50h] [rbp-D8h]
  unsigned int v31; // [rsp+58h] [rbp-D0h]
  long long v32; // [rsp+60h] [rbp-C8h]
  unsigned int v33; // [rsp+68h] [rbp-C0h]
  long long v34; // [rsp+70h] [rbp-B8h]
  unsigned int v35; // [rsp+78h] [rbp-B0h]
  unsigned int v36; // [rsp+80h] [rbp-A8h]
  unsigned int v37; // [rsp+84h] [rbp-A4h]
  unsigned int v38; // [rsp+88h] [rbp-A0h]
  unsigned int v39; // [rsp+8Ch] [rbp-9Ch]
  unsigned long long v40; // [rsp+108h] [rbp-20h]

  BYTE1(a4) |= 8u;
  v5 = a5;
  v40 = __readfsqword(0x28u);
  result = statx(a1, a2, a4, a5, v20);
  if ( (int)result >= 0 )
  {
    v8 = v22;
    *(long long *)a3 = (unsigned char)v39
                  | (v38 << 8) & 0xFFF00
                  | ((unsigned long long)v38 << 32) & 0xFFFFF00000000000LL
                  | ((unsigned long long)v39 << 12) & 0xFFFFFF00000LL;
    v9 = v25;
    *(long long *)(a3 + 16) = v8;
    v10 = v36;
    *(long long *)(a3 + 8) = v9;
    *(int *)(a3 + 24) = v24;
    *(long long *)(a3 + 28) = v23;
    v11 = v21;
    *(long long *)(a3 + 40) = (unsigned char)v37
                         | ((int)v10 << 8) & 0xFFF00
                         | (v10 << 32) & 0xFFFFF00000000000LL
                         | ((unsigned long long)v37 << 12) & 0xFFFFFF00000LL;
    v12 = v26;
    *(long long *)(a3 + 56) = v11;
    v13 = v29;
    *(long long *)(a3 + 48) = v12;
    v14 = v27;
    *(long long *)(a3 + 80) = v13;
    v15 = v35;
    *(long long *)(a3 + 64) = v14;
    v16 = v28;
    *(long long *)(a3 + 96) = v15;
    v17 = v33;
    *(long long *)(a3 + 72) = v16;
    v18 = v34;
    *(long long *)(a3 + 112) = v17;
    *(long long *)(a3 + 88) = v18;
    *(long long *)(a3 + 104) = v32;
    if ( (v5 & 0x800) != 0 )
    {
      if ( (v20[1] & 8) != 0 )
      {
        v19 = v31;
        *(long long *)(a3 + 88) = v30;
        *(long long *)(a3 + 96) = v19;
      }
      else
      {
        *(long long *)(a3 + 96) = -1;
        *(long long *)(a3 + 88) = -1;
      }
    }
  }
  return result;
}



// Function: file_escape @ 0x72a0
long long file_escape(const char *a1, char a2)
{
  unsigned char *v2; // rbx
  size_t v3; // rax
  long long v4; // r13
  unsigned char v5; // al
  unsigned char *v6; // rbp
  unsigned char *v7; // rdi

  v2 = (unsigned char *)a1;
  v3 = strlen(a1);
  v4 = sub_186D0(3, v3 + 1);
  v5 = *a1;
  v6 = (unsigned char *)v4;
  if ( *a1 )
  {
    while ( 1 )
    {
      ++v2;
      if ( v5 == 47 )
      {
        if ( a2 )
          break;
      }
      if ( byte_25FE0[v5] )
      {
        *v6++ = v5;
LABEL_4:
        v5 = *v2;
        if ( !*v2 )
          goto LABEL_9;
      }
      else
      {
        v7 = v6;
        v6 += 3;
        __sprintf_chk(v7, 1, -1, "%%%02x", v5);
        v5 = *v2;
        if ( !*v2 )
          goto LABEL_9;
      }
    }
    *v6++ = 47;
    goto LABEL_4;
  }
LABEL_9:
  *v6 = 0;
  return v4;
}



// Function: indent @ 0x7360
int indent(unsigned long long a1, unsigned long long a2)
{
  unsigned long long v3; // rbx
  unsigned long long v4; // rax
  unsigned long long v5; // rcx
  unsigned long long v6; // rbp
  char *IO_write_ptr; // rsi
  char *IO_write_end; // r8

  if ( a1 < a2 )
  {
    v3 = a1;
    do
    {
      while ( 1 )
      {
        v5 = qword_262C0;
        v6 = v3 + 1;
        IO_write_ptr = stdout->_IO_write_ptr;
        IO_write_end = stdout->_IO_write_end;
        if ( !qword_262C0 || a2 / qword_262C0 <= v6 / qword_262C0 )
          break;
        if ( IO_write_ptr >= IO_write_end )
        {
          __overflow(stdout, 9);
          v5 = qword_262C0;
        }
        else
        {
          stdout->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = 9;
        }
        v4 = v3 / v5;
        v3 = v3 + v5 - v3 % v5;
        if ( v3 >= a2 )
          return v4;
      }
      if ( IO_write_ptr >= IO_write_end )
      {
        LODWORD(v4) = __overflow(stdout, 32);
      }
      else
      {
        LODWORD(v4) = (int)IO_write_ptr + 1;
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = 32;
      }
      ++v3;
    }
    while ( v6 < a2 );
  }
  return v4;
}



// Function: dired_outbuf @ 0x7430
size_t dired_outbuf(const void *a1, size_t a2)
{
  qword_261F8 += a2;
  return fwrite_unlocked(a1, 1u, a2, stdout);
}



// Function: dired_dump_obstack @ 0x7450
int dired_dump_obstack(const char *a1, long long a2)
{
  char *IO_write_ptr; // rax
  char *v3; // rbx
  signed long long v4; // rbp
  unsigned long long v5; // rax
  unsigned long long v6; // rdx
  char *v7; // rbp
  long long v8; // rdx

  IO_write_ptr = *(char **)(a2 + 24);
  v3 = *(char **)(a2 + 16);
  v4 = IO_write_ptr - v3;
  if ( (unsigned long long)(IO_write_ptr - v3) > 7 )
  {
    if ( IO_write_ptr == v3 )
      *(char *)(a2 + 80) |= 2u;
    v5 = ~*(long long *)(a2 + 48) & (unsigned long long)&IO_write_ptr[*(long long *)(a2 + 48)];
    v6 = *(long long *)(a2 + 32);
    if ( v5 - *(long long *)(a2 + 8) <= v6 - *(long long *)(a2 + 8) )
      v6 = v5;
    v7 = &v3[v4 & 0xFFFFFFFFFFFFFFF8LL];
    *(long long *)(a2 + 24) = v6;
    *(long long *)(a2 + 16) = v6;
    fputs_unlocked(a1, stdout);
    do
    {
      v8 = *(long long *)v3;
      v3 += 8;
      __printf_chk(1, " %ld", v8);
    }
    while ( v3 != v7 );
    IO_write_ptr = stdout->_IO_write_ptr;
    if ( IO_write_ptr >= stdout->_IO_write_end )
    {
      LODWORD(IO_write_ptr) = __overflow(stdout, 10);
    }
    else
    {
      stdout->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 10;
    }
  }
  return (int)IO_write_ptr;
}



// Function: file_failure @ 0x7540
void file_failure(char a1, const char *a2, long long a3)
{
  long long v3; // r13
  int *v4; // rax

  v3 = sub_16DD0(4, a3);
  v4 = __errno_location();
  error(0, *v4, a2, v3);
  if ( a1 )
  {
    dword_26210 = 2;
  }
  else if ( !dword_26210 )
  {
    dword_26210 = 1;
  }
}



// Function: put_indicator @ 0x7880
size_t put_indicator(long long a1, long long a2)
{
  if ( !byte_26310 )
  {
    byte_26310 = 1;
    if ( tcgetpgrp(1) >= 0 )
      sub_76B0(1);
    if ( qword_25088 )
    {
      put_indicator(&unk_25080, a2);
    }
    else
    {
      put_indicator(qword_25060, a2);
      put_indicator(&unk_25090, a2);
      put_indicator(&qword_25070, a2);
    }
  }
  return fwrite_unlocked(*(const void **)(a1 + 8), *(long long *)a1, 1u, stdout);
}



// Function: quote_name_width @ 0x7d70
long long quote_name_width(void *a1, long long a2, unsigned int a3)
{
  unsigned char v4; // [rsp+Fh] [rbp-2039h] BYREF
  void *ptr; // [rsp+10h] [rbp-2038h] BYREF
  long long v6; // [rsp+18h] [rbp-2030h] BYREF
  char v7[24]; // [rsp+20h] [rbp-2028h] BYREF
  unsigned long long v8; // [rsp+2028h] [rbp-20h]

  v8 = __readfsqword(0x28u);
  ptr = v7;
  sub_7920(&ptr, a1, a2, a3, &v6, &v4);
  if ( ptr != v7 && ptr != a1 )
    free(ptr);
  return v6 + v4;
}



// Function: length_of_file_name_and_frills @ 0x7e10
long long length_of_file_name_and_frills(long long a1, long long a2, long long a3, long long a4)
{
  long long v5; // rbx
  long long v6; // rax
  long long v7; // rax
  long long v8; // r8
  char v9; // al
  long long v10; // r8
  const char *v12; // rdi
  long long v13; // rax
  const char *v14; // rax
  long long v15[87]; // [rsp+0h] [rbp-2B8h] BYREF

  v15[83] = __readfsqword(0x28u);
  if ( !byte_262FC )
  {
    v5 = 0;
    goto LABEL_12;
  }
  if ( dword_26338 == 4 )
  {
    v14 = (const char *)sub_12720(*(long long *)(a1 + 32), v15);
    v5 = strlen(v14) + 1;
LABEL_12:
    if ( !byte_2632C )
    {
LABEL_6:
      if ( !byte_26365 )
        goto LABEL_7;
      if ( dword_26338 == 4 )
      {
        v13 = strlen(*(const char **)(a1 + 176)) + 1;
LABEL_19:
        v5 += v13;
        v7 = *(long long *)(a1 + 200);
        if ( v7 )
          goto LABEL_8;
        goto LABEL_20;
      }
LABEL_18:
      v13 = dword_26354 + 1LL;
      goto LABEL_19;
    }
    if ( dword_26338 == 4 )
    {
      v12 = "?";
      if ( *(char *)(a1 + 184) )
        v12 = (const char *)sub_11550(*(long long *)(a1 + 88), v15, (unsigned int)dword_26328, 512, qword_26320);
      v6 = strlen(v12) + 1;
      goto LABEL_5;
    }
LABEL_4:
    v6 = dword_2635C + 1LL;
LABEL_5:
    v5 += v6;
    goto LABEL_6;
  }
  v5 = dword_26360 + 1LL;
  if ( byte_2632C )
    goto LABEL_4;
  if ( byte_26365 )
    goto LABEL_18;
LABEL_7:
  v7 = *(long long *)(a1 + 200);
  if ( v7 )
    goto LABEL_8;
LABEL_20:
  v7 = sub_7D70(*(void **)a1, qword_262D0, *(int *)(a1 + 196));
LABEL_8:
  v8 = v5 + v7;
  if ( dword_26314 )
  {
    v9 = sub_75B0(*(unsigned char *)(a1 + 184), *(unsigned int *)(a1 + 48), *(unsigned int *)(a1 + 168), a4, v8);
    return v10 - ((v9 == 0) - 1LL);
  }
  return v8;
}



// Function: calculate_columns @ 0x7ff0
unsigned long long calculate_columns(char a1, long long a2, long long a3, unsigned long long a4)
{
  long long v4; // rsi
  unsigned long long v5; // r12
  long long v7; // rdi
  unsigned long long v8; // rbx
  long long v9; // rax
  long long v10; // rdi
  long long v11; // r9
  long long v12; // r8
  unsigned long long v13; // rdi
  long long *v14; // rax
  unsigned long long v15; // rbx
  long long v16; // rax
  unsigned long long v17; // r10
  long long v18; // r9
  unsigned long long v19; // r11
  unsigned long long v20; // r8
  unsigned long long v21; // rax
  unsigned long long v22; // rdi
  long long v23; // rax
  char *v24; // rax

  v4 = qword_26200;
  v5 = qword_263B0;
  if ( !qword_26200 )
  {
    if ( qword_263B0 > (unsigned long long)qword_252E0 )
      goto LABEL_5;
    goto LABEL_37;
  }
  if ( qword_26200 < (unsigned long long)qword_263B0 )
  {
    if ( qword_26200 <= (unsigned long long)qword_252E0 )
    {
      v11 = qword_263B0;
      v5 = qword_26200;
      goto LABEL_12;
    }
    v5 = qword_26200;
    if ( (unsigned long long)qword_26200 >> 1 > qword_26200 )
      goto LABEL_5;
LABEL_29:
    v7 = qword_26208;
    v23 = sub_18650(qword_26208, qword_26200, 24);
    v8 = qword_26200;
    qword_26208 = v23;
    goto LABEL_6;
  }
  if ( qword_263B0 <= (unsigned long long)qword_252E0 )
  {
LABEL_37:
    v11 = qword_263B0;
    if ( qword_263B0 )
      goto LABEL_12;
LABEL_38:
    if ( !v11 )
      return v5;
    goto LABEL_17;
  }
  if ( (unsigned long long)qword_26200 >> 1 <= qword_263B0 )
    goto LABEL_29;
LABEL_5:
  v7 = qword_26208;
  v4 = v5;
  v8 = 2 * v5;
  qword_26208 = sub_18650(qword_26208, v5, 48);
LABEL_6:
  if ( __CFADD__(v8, qword_252E0 + 1) || !is_mul_ok(v8 + qword_252E0 + 1, v8 - qword_252E0) )
    sub_18A90(v7, v4, ((v8 + qword_252E0 + 1) * (unsigned __int128)(v8 - qword_252E0)) >> 64 != 0);
  v4 = 8;
  v9 = sub_186D0(((v8 + qword_252E0 + 1) * (v8 - qword_252E0)) >> 1, 8);
  a3 = qword_252E0;
  if ( v8 > qword_252E0 )
  {
    v10 = qword_26208;
    a3 = 8 * qword_252E0 + 8;
    v4 = 8 * v8 + 8;
    do
    {
      a4 = 3 * a3;
      *(long long *)(v10 + 3 * a3 - 8) = v9;
      v9 += a3;
      a3 += 8;
    }
    while ( v4 != a3 );
  }
  qword_252E0 = v8;
  v11 = qword_263B0;
  if ( !v5 )
    goto LABEL_38;
LABEL_12:
  v12 = qword_26208;
  v4 = 3;
  v13 = 0;
  do
  {
    v14 = *(long long **)(v12 + 8 * v4 - 8);
    *(char *)(v12 + 8 * v4 - 24) = 1;
    *(long long *)(v12 + 8 * v4 - 16) = v4;
    a4 = (unsigned long long)&v14[v13];
    do
    {
      a3 = (long long)v14;
      *v14++ = 3;
    }
    while ( a4 != a3 );
    ++v13;
    v4 += 3;
  }
  while ( v13 < v5 );
  if ( v11 )
  {
LABEL_17:
    v15 = 0;
    do
    {
      v16 = sub_7E10(*((long long *)qword_263A0 + v15), v4, a3, a4);
      v17 = qword_263B0;
      v18 = v16;
      if ( v5 )
      {
        v19 = qword_262B0;
        v4 = qword_26208;
        a4 = 0;
        do
        {
          v22 = a4++;
          if ( *(char *)v4 )
          {
            v20 = a1 ? v15 / ((v17 + a4 - 1) / a4) : v15 % a4;
            a3 = *(long long *)(v4 + 16) + 8 * v20;
            v21 = v18 + 2LL * (v20 != v22);
            if ( *(long long *)a3 < v21 )
            {
              *(long long *)(v4 + 8) += v21 - *(long long *)a3;
              *(long long *)a3 = v21;
              *(char *)v4 = *(long long *)(v4 + 8) < v19;
            }
          }
          v4 += 24;
        }
        while ( v5 != a4 );
      }
      ++v15;
    }
    while ( v15 < v17 );
  }
  if ( v5 > 1 )
  {
    v24 = (char *)(qword_26208 + 24 * v5 - 24);
    do
    {
      if ( *v24 )
        break;
      --v5;
      v24 -= 24;
    }
    while ( v5 != 1 );
  }
  return v5;
}



// Function: process_signals @ 0x84f0
unsigned long long process_signals(long long a1, sigset_t *a2)
{
  int v2; // r14d
  sigset_t v4; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v5; // [rsp+88h] [rbp-30h]

  v5 = __readfsqword(0x28u);
  while ( sig || dword_26214 )
  {
    if ( byte_26310 )
    {
      sub_7880((long long)qword_25060, (long long)a2);
      sub_7880((long long)&qword_25070, (long long)a2);
    }
    fflush_unlocked(stdout);
    sigprocmask(0, &set, &v4);
    v2 = sig;
    if ( dword_26214 )
    {
      v2 = 19;
      --dword_26214;
    }
    else
    {
      signal(sig, 0);
    }
    raise(v2);
    a2 = &v4;
    sigprocmask(2, &v4, 0);
  }
  return v5 - __readfsqword(0x28u);
}



// Function: format_user_width @ 0x85f0
long long format_user_width(__uid_t a1)
{
  long long result; // rax
  long long v3; // rdi

  if ( byte_2632D )
    return __snprintf_chk(0, 0, 1, -1, "%lu", a1);
  v3 = sub_12340(a1);
  if ( !v3 )
    return __snprintf_chk(0, 0, 1, -1, "%lu", a1);
  result = sub_12EC0(v3, 0);
  if ( (int)result < 0 )
    return 0;
  return result;
}



// Function: calc_req_mask @ 0x8640
long long calc_req_mask()
{
  int v0; // eax
  long long result; // rax
  int v2; // esi
  int v3; // eax

  v0 = -(byte_262FC == 0);
  LOBYTE(v0) = 0;
  result = (unsigned int)(v0 + 258);
  if ( byte_2632C )
    BYTE1(result) |= 4u;
  if ( !dword_26338 )
  {
    if ( (unsigned int)dword_26334 <= 3 )
    {
      v2 = dword_1A640[dword_26334];
      v3 = v2 | result;
      if ( byte_25029 || byte_2632E )
        result = v3 | 0x20Cu;
      else
        result = v3 | 0x204u;
      if ( byte_25028 )
        result = (unsigned int)result | 0x10;
      switch ( dword_26330 )
      {
        case 0:
        case 1:
        case 2:
        case 4:
        case 6:
          return result;
        case 3:
          goto LABEL_12;
        case 5:
          return v2 | (unsigned int)result;
        default:
          break;
      }
    }
LABEL_17:
    abort();
  }
  switch ( dword_26330 )
  {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      return result;
    case 3:
LABEL_12:
      BYTE1(result) |= 2u;
      return result;
    case 5:
      if ( (unsigned int)dword_26334 > 3 )
        goto LABEL_17;
      v2 = dword_1A640[dword_26334];
      return v2 | (unsigned int)result;
    default:
      goto LABEL_17;
  }
}



// Function: clear_files @ 0x8730
void clear_files()
{
  unsigned long long v0; // rbp
  void **v1; // rbx

  if ( qword_263B0 )
  {
    v0 = 0;
    do
    {
      v1 = (void **)*((long long *)qword_263A0 + v0);
      free(*v1);
      free(v1[1]);
      free(v1[2]);
      if ( v1[22] != &unk_2502A )
        freecon();
      ++v0;
    }
    while ( qword_263B0 > v0 );
  }
  byte_263A9 = 0;
  qword_263B0 = 0;
  byte_26364 = 0;
  dword_26360 = 0;
  dword_2635C = 0;
  dword_26358 = 0;
  dword_26350 = 0;
  dword_2634C = 0;
  dword_26348 = 0;
  dword_26354 = 0;
  dword_26344 = 0;
  dword_26340 = 0;
  dword_2633C = 0;
}



// Function: strcmp_width @ 0x8980
int strcmp_width(long long a1, long long a2)
{
  long long v2; // rax
  long long v3; // rdx
  int v4; // ebx
  int result; // eax

  v2 = *(long long *)(a1 + 200);
  if ( !v2 )
    LODWORD(v2) = sub_7D70(*(void **)a1, qword_262D0, *(int *)(a1 + 196));
  v3 = *(long long *)(a2 + 200);
  v4 = v2;
  if ( !v3 )
    LODWORD(v3) = sub_7D70(*(void **)a2, qword_262D0, *(int *)(a2 + 196));
  result = v4 - v3;
  if ( v4 == (int)v3 )
    return strcmp(*(const char **)a1, *(const char **)a2);
  return result;
}



// Function: xstrcoll_atime @ 0x8a00
int xstrcoll_atime(long long a1, long long a2)
{
  int result; // eax
  const char *v3; // rbp
  const char *v4; // r12

  result = (*(long long *)(a1 + 104) < *(long long *)(a2 + 104))
         - (*(long long *)(a1 + 104) > *(long long *)(a2 + 104))
         + 2 * ((*(long long *)(a1 + 96) < *(long long *)(a2 + 96)) - (*(long long *)(a1 + 96) > *(long long *)(a2 + 96)));
  if ( !result )
  {
    v3 = *(const char **)a1;
    v4 = *(const char **)a2;
    *__errno_location() = 0;
    return strcoll(v3, v4);
  }
  return result;
}



// Function: rev_xstrcoll_ctime @ 0x8ae0
int rev_xstrcoll_ctime(long long a1, long long a2)
{
  int result; // eax
  const char *v3; // rbp
  const char *v4; // r12

  result = (*(long long *)(a2 + 136) < *(long long *)(a1 + 136))
         - (*(long long *)(a2 + 136) > *(long long *)(a1 + 136))
         + 2 * ((*(long long *)(a2 + 128) < *(long long *)(a1 + 128)) - (*(long long *)(a2 + 128) > *(long long *)(a1 + 128)));
  if ( !result )
  {
    v3 = *(const char **)a2;
    v4 = *(const char **)a1;
    *__errno_location() = 0;
    return strcoll(v3, v4);
  }
  return result;
}



// Function: rev_xstrcoll_df_ctime @ 0x8b60
int rev_xstrcoll_df_ctime(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8AE0(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8AE0(a1, a2);
}



// Function: rev_xstrcoll_btime @ 0x8bf0
int rev_xstrcoll_btime(long long a1, long long a2)
{
  int result; // eax
  const char *v3; // rbp
  const char *v4; // r12

  result = (*(long long *)(a2 + 120) < *(long long *)(a1 + 120))
         - (*(long long *)(a2 + 120) > *(long long *)(a1 + 120))
         + 2 * ((*(long long *)(a2 + 112) < *(long long *)(a1 + 112)) - (*(long long *)(a2 + 112) > *(long long *)(a1 + 112)));
  if ( !result )
  {
    v3 = *(const char **)a2;
    v4 = *(const char **)a1;
    *__errno_location() = 0;
    return strcoll(v3, v4);
  }
  return result;
}



// Function: xstrcoll_btime @ 0x8cd0
int xstrcoll_btime(long long a1, long long a2)
{
  int result; // eax
  const char *v3; // rbp
  const char *v4; // r12

  result = (*(long long *)(a1 + 120) < *(long long *)(a2 + 120))
         - (*(long long *)(a1 + 120) > *(long long *)(a2 + 120))
         + 2 * ((*(long long *)(a1 + 112) < *(long long *)(a2 + 112)) - (*(long long *)(a1 + 112) > *(long long *)(a2 + 112)));
  if ( !result )
  {
    v3 = *(const char **)a1;
    v4 = *(const char **)a2;
    *__errno_location() = 0;
    return strcoll(v3, v4);
  }
  return result;
}



// Function: xstrcoll_ctime @ 0x8d40
int xstrcoll_ctime(long long a1, long long a2)
{
  int result; // eax
  const char *v3; // rbp
  const char *v4; // r12

  result = (*(long long *)(a1 + 136) < *(long long *)(a2 + 136))
         - (*(long long *)(a1 + 136) > *(long long *)(a2 + 136))
         + 2 * ((*(long long *)(a1 + 128) < *(long long *)(a2 + 128)) - (*(long long *)(a1 + 128) > *(long long *)(a2 + 128)));
  if ( !result )
  {
    v3 = *(const char **)a1;
    v4 = *(const char **)a2;
    *__errno_location() = 0;
    return strcoll(v3, v4);
  }
  return result;
}



// Function: rev_xstrcoll_atime @ 0x8dc0
int rev_xstrcoll_atime(long long a1, long long a2)
{
  int result; // eax
  const char *v3; // rbp
  const char *v4; // r12

  result = (*(long long *)(a2 + 104) < *(long long *)(a1 + 104))
         - (*(long long *)(a2 + 104) > *(long long *)(a1 + 104))
         + 2 * ((*(long long *)(a2 + 96) < *(long long *)(a1 + 96)) - (*(long long *)(a2 + 96) > *(long long *)(a1 + 96)));
  if ( !result )
  {
    v3 = *(const char **)a2;
    v4 = *(const char **)a1;
    *__errno_location() = 0;
    return strcoll(v3, v4);
  }
  return result;
}



// Function: sort_files @ 0x8e30
long long sort_files()
{
  long long v0; // rbp
  void *v1; // rax
  long long *v2; // rax
  long long v3; // rdx
  long long *v4; // rcx
  long long result; // rax
  int v6; // eax
  long long v7; // rcx
  long long v8; // r8
  void *v9; // rdi
  long long v10; // rdx
  long long *v11; // rax
  long long *v12; // rsi
  long long v13; // rbx
  bool v14; // cf
  long long v15; // rbp
  long long v16; // rax

  v0 = qword_263B0;
  if ( qword_263B0 + ((unsigned long long)qword_263B0 >> 1) > qword_26398 )
  {
    free(qword_263A0);
    v1 = (void *)sub_186D0(v0, 24);
    v0 = qword_263B0;
    qword_263A0 = v1;
    qword_26398 = 3 * qword_263B0;
  }
  if ( v0 )
  {
    v2 = (long long *)qword_263A0;
    v3 = qword_263C0;
    v4 = (long long *)((char *)qword_263A0 + 8 * v0);
    do
    {
      *v2++ = v3;
      v3 += 208;
    }
    while ( v2 != v4 );
    result = (unsigned int)dword_26330;
    if ( dword_26330 == 2 )
    {
LABEL_22:
      v13 = 0;
      do
      {
        v15 = *((long long *)qword_263A0 + v13);
        v16 = *(long long *)(v15 + 200);
        if ( !v16 )
          v16 = sub_7D70(*(void **)v15, qword_262D0, *(int *)(v15 + 196));
        v14 = ++v13 < (unsigned long long)qword_263B0;
        *(long long *)(v15 + 200) = v16;
      }
      while ( v14 );
      result = (unsigned int)dword_26330;
LABEL_8:
      if ( (int)result == 6 )
        return result;
      goto LABEL_9;
    }
LABEL_7:
    if ( !qword_262B0 || (unsigned int)(dword_26338 - 2) > 1 || !v0 )
      goto LABEL_8;
    goto LABEL_22;
  }
  result = (unsigned int)dword_26330;
  if ( dword_26330 != 2 )
    goto LABEL_7;
LABEL_9:
  v6 = _setjmp(env);
  v7 = (unsigned int)dword_26330;
  if ( v6 )
  {
    if ( dword_26330 == 4 )
      __assert_fail("sort_type != sort_version", "src/ls.c", 0x1008u, "sort_files");
    v8 = qword_263B0;
    v9 = qword_263A0;
    if ( qword_263B0 )
    {
      v10 = qword_263C0;
      v11 = (long long *)qword_263A0;
      v12 = (long long *)((char *)qword_263A0 + 8 * qword_263B0);
      do
      {
        *v11++ = v10;
        v10 += 208;
      }
      while ( v12 != v11 );
    }
    v6 = 1;
  }
  else
  {
    v8 = qword_263B0;
    v9 = qword_263A0;
  }
  if ( (int)v7 == 5 )
    v7 = (unsigned int)(dword_26334 + 5);
  return sub_131B0(
           v9,
           v8,
           *(&off_23FA0[8 * v7] + 4 * v6 + 2 * (unsigned char)byte_2632F + (unsigned char)byte_262F4));
}



// Function: rev_xstrcoll_width @ 0x91a0
int rev_xstrcoll_width(long long a1, long long a2)
{
  long long v2; // rax
  long long v3; // rdx
  int v4; // ebx
  int result; // eax
  int *v6; // rax
  const char *v7; // r12
  const char *v8; // rbp

  v2 = *(long long *)(a2 + 200);
  if ( !v2 )
    LODWORD(v2) = sub_7D70(*(void **)a2, qword_262D0, *(int *)(a2 + 196));
  v3 = *(long long *)(a1 + 200);
  v4 = v2;
  if ( !v3 )
    LODWORD(v3) = sub_7D70(*(void **)a1, qword_262D0, *(int *)(a1 + 196));
  result = v4 - v3;
  if ( v4 == (int)v3 )
  {
    v6 = __errno_location();
    v7 = *(const char **)a2;
    v8 = *(const char **)a1;
    *v6 = 0;
    return strcoll(v7, v8);
  }
  return result;
}



// Function: xstrcoll_width @ 0x9230
int xstrcoll_width(long long a1, long long a2)
{
  long long v2; // rax
  long long v3; // rdx
  int v4; // ebx
  int result; // eax
  int *v6; // rax
  const char *v7; // r12
  const char *v8; // rbp

  v2 = *(long long *)(a1 + 200);
  if ( !v2 )
    LODWORD(v2) = sub_7D70(*(void **)a1, qword_262D0, *(int *)(a1 + 196));
  v3 = *(long long *)(a2 + 200);
  v4 = v2;
  if ( !v3 )
    LODWORD(v3) = sub_7D70(*(void **)a2, qword_262D0, *(int *)(a2 + 196));
  result = v4 - v3;
  if ( v4 == (int)v3 )
  {
    v6 = __errno_location();
    v7 = *(const char **)a1;
    v8 = *(const char **)a2;
    *v6 = 0;
    return strcoll(v7, v8);
  }
  return result;
}



// Function: xstrcoll_extension @ 0x9370
int xstrcoll_extension(const char **a1, const char **a2)
{
  const char *v2; // r13
  char *v3; // rax
  const char *v4; // r14
  const char *v5; // rbp
  char *v6; // r12
  int result; // eax

  v2 = *a1;
  v3 = strrchr(*a1, 46);
  v4 = *a2;
  v5 = v3;
  v6 = strrchr(*a2, 46);
  if ( !v6 )
    v6 = (char *)"";
  if ( !v5 )
    v5 = "";
  *__errno_location() = 0;
  result = strcoll(v5, v6);
  if ( !result )
    return strcoll(v2, v4);
  return result;
}



// Function: rev_xstrcoll_extension @ 0x9400
int rev_xstrcoll_extension(const char **a1, const char **a2)
{
  const char *v2; // r13
  char *v3; // rax
  const char *v4; // r14
  const char *v5; // rbp
  char *v6; // r12
  int result; // eax

  v2 = *a2;
  v3 = strrchr(*a2, 46);
  v4 = *a1;
  v5 = v3;
  v6 = strrchr(*a1, 46);
  if ( !v6 )
    v6 = (char *)"";
  if ( !v5 )
    v5 = "";
  *__errno_location() = 0;
  result = strcoll(v5, v6);
  if ( !result )
    return strcoll(v2, v4);
  return result;
}



// Function: rev_strcmp_width @ 0x9490
int rev_strcmp_width(long long a1, long long a2)
{
  long long v2; // rax
  long long v3; // rdx
  int v4; // ebx
  int result; // eax

  v2 = *(long long *)(a2 + 200);
  if ( !v2 )
    LODWORD(v2) = sub_7D70(*(void **)a2, qword_262D0, *(int *)(a2 + 196));
  v3 = *(long long *)(a1 + 200);
  v4 = v2;
  if ( !v3 )
    LODWORD(v3) = sub_7D70(*(void **)a1, qword_262D0, *(int *)(a1 + 196));
  result = v4 - v3;
  if ( v4 == (int)v3 )
    return strcmp(*(const char **)a2, *(const char **)a1);
  return result;
}



// Function: extract_dirs_from_files @ 0x9510
void extract_dirs_from_files(void *src)
{
  long long *v1; // rbx
  long long v2; // rax
  void *v3; // rax
  long long v4; // rbx
  bool v5; // cf
  long long v6; // rbp
  int v7; // eax
  char *v8; // r15
  char *v9; // rax
  void *v10; // r15
  long long *v11; // rsi
  char *v12; // rdi
  char *v13; // rax
  long long v14; // rdx
  bool v15; // zf
  char v16; // al

  if ( src && qword_263C8 )
  {
    v1 = (long long *)sub_18590(32);
    v2 = sub_18A70(src);
    *v1 = 0;
    v1[1] = v2;
    v3 = ptr;
    *((char *)v1 + 16) = 0;
    v1[3] = v3;
    ptr = v1;
  }
  v4 = qword_263B0 - 1;
  if ( !qword_263B0 )
    goto LABEL_24;
  do
  {
    while ( 1 )
    {
      v6 = *((long long *)qword_263A0 + v4);
      v7 = *(int *)(v6 + 168);
      if ( v7 != 3 && v7 != 9 )
        goto LABEL_6;
      v8 = *(char **)v6;
      if ( !src )
        goto LABEL_20;
      v9 = (char *)sub_EE20(*(long long *)v6);
      if ( *v9 == 46 )
      {
        v16 = v9[(v9[1] == 46) + 1];
        if ( !v16 || v16 == 47 )
          goto LABEL_6;
      }
      if ( *v8 == 47 )
      {
LABEL_20:
        sub_6FB0(v8, *(void **)(v6 + 8));
      }
      else
      {
        v10 = (void *)sub_FBB0(src, v8, 0);
        sub_6FB0(v10, *(void **)(v6 + 8));
        free(v10);
      }
      if ( *(int *)(v6 + 168) == 9 )
      {
        free(*(void **)v6);
        free(*(void **)(v6 + 8));
        free(*(void **)(v6 + 16));
        if ( *(_UNKNOWN **)(v6 + 176) != &unk_2502A )
          break;
      }
LABEL_6:
      v5 = v4-- == 0;
      if ( v5 )
        goto LABEL_16;
    }
    freecon();
    v5 = v4-- == 0;
  }
  while ( !v5 );
LABEL_16:
  if ( !qword_263B0 )
  {
LABEL_24:
    v14 = 0;
    goto LABEL_19;
  }
  v11 = qword_263A0;
  v12 = (char *)qword_263A0 + 8 * qword_263B0;
  v13 = (char *)qword_263A0;
  v14 = 0;
  do
  {
    v15 = *(int *)(*(long long *)v13 + 168LL) == 9;
    v11[v14] = *(long long *)v13;
    v13 += 8;
    v14 += !v15;
  }
  while ( v12 != v13 );
LABEL_19:
  qword_263B0 = v14;
}



// Function: rev_strcmp_df_size @ 0xa660
int rev_strcmp_df_size(long long a1, long long a2)
{
  int v3; // esi
  bool v4; // al
  int result; // eax
  int v6; // eax
  long long v7; // rax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v7 = *(long long *)(a1 + 72);
  if ( *(long long *)(a2 + 72) > v7 )
    return -1;
  if ( *(long long *)(a2 + 72) == v7 )
    return strcmp(*(const char **)a2, *(const char **)a1);
  else
    return 1;
}



// Function: rev_strcmp_df_ctime @ 0xa710
int rev_strcmp_df_ctime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a2 + 136) < *(long long *)(a1 + 136))
         - (*(long long *)(a2 + 136) > *(long long *)(a1 + 136))
         + 2 * ((*(long long *)(a2 + 128) < *(long long *)(a1 + 128)) - (*(long long *)(a2 + 128) > *(long long *)(a1 + 128)));
  if ( !result )
    return strcmp(*(const char **)a2, *(const char **)a1);
  return result;
}



// Function: rev_strcmp_df_atime @ 0xa7f0
int rev_strcmp_df_atime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a2 + 104) < *(long long *)(a1 + 104))
         - (*(long long *)(a2 + 104) > *(long long *)(a1 + 104))
         + 2 * ((*(long long *)(a2 + 96) < *(long long *)(a1 + 96)) - (*(long long *)(a2 + 96) > *(long long *)(a1 + 96)));
  if ( !result )
    return strcmp(*(const char **)a2, *(const char **)a1);
  return result;
}



// Function: strcmp_df_name @ 0xa8c0
int strcmp_df_name(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return strcmp(*(const char **)a1, *(const char **)a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return strcmp(*(const char **)a1, *(const char **)a2);
}



// Function: rev_xstrcoll_df_name @ 0xa960
int rev_xstrcoll_df_name(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  const char *v6; // r12
  const char *v7; // rbp

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 9 || *(int *)(a1 + 168) == 3;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = *(const char **)a1;
  v7 = *(const char **)a2;
  *__errno_location() = 0;
  return strcoll(v7, v6);
}



// Function: rev_xstrcoll_df_version @ 0xaa20
int rev_xstrcoll_df_version(long long *a1, long long *a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  const char *v6; // rbp
  const char *v7; // r12

  v2 = *((int *)a2 + 42);
  v3 = *((int *)a1 + 42) == 3 || *((int *)a1 + 42) == 9;
  if ( v2 != 3 && v2 != 9 && (*((int *)a2 + 43) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*((int *)a1 + 43) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = (const char *)*a2;
  v7 = (const char *)*a1;
  result = sub_101E0(*a2, *a1);
  if ( !result )
    return strcmp(v6, v7);
  return result;
}



// Function: xstrcoll_df_version @ 0xaaf0
int xstrcoll_df_version(long long *a1, long long *a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  const char *v6; // rbp
  const char *v7; // r12

  v2 = *((int *)a2 + 42);
  v3 = *((int *)a1 + 42) == 3 || *((int *)a1 + 42) == 9;
  if ( v2 != 3 && v2 != 9 && (*((int *)a2 + 43) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*((int *)a1 + 43) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = (const char *)*a1;
  v7 = (const char *)*a2;
  result = sub_101E0(*a1, *a2);
  if ( !result )
    return strcmp(v6, v7);
  return result;
}



// Function: xstrcoll_df_name @ 0xabc0
int xstrcoll_df_name(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  const char *v6; // r12
  const char *v7; // rbp

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 9 || *(int *)(a1 + 168) == 3;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = *(const char **)a2;
  v7 = *(const char **)a1;
  *__errno_location() = 0;
  return strcoll(v7, v6);
}



// Function: xstrcoll_df_btime @ 0xac80
int xstrcoll_df_btime(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8CD0(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8CD0(a1, a2);
}



// Function: xstrcoll_df_ctime @ 0xad10
int xstrcoll_df_ctime(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8D40(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8D40(a1, a2);
}



// Function: rev_strcmp_df_width @ 0xada0
int rev_strcmp_df_width(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_9490(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_9490(a1, a2);
}



// Function: rev_strcmp_df_btime @ 0xae30
int rev_strcmp_df_btime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a2 + 120) < *(long long *)(a1 + 120))
         - (*(long long *)(a2 + 120) > *(long long *)(a1 + 120))
         + 2 * ((*(long long *)(a2 + 112) < *(long long *)(a1 + 112)) - (*(long long *)(a2 + 112) > *(long long *)(a1 + 112)));
  if ( !result )
    return strcmp(*(const char **)a2, *(const char **)a1);
  return result;
}



// Function: strcmp_df_btime @ 0xaf00
int strcmp_df_btime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a1 + 120) < *(long long *)(a2 + 120))
         - (*(long long *)(a1 + 120) > *(long long *)(a2 + 120))
         + 2 * ((*(long long *)(a1 + 112) < *(long long *)(a2 + 112)) - (*(long long *)(a1 + 112) > *(long long *)(a2 + 112)));
  if ( !result )
    return strcmp(*(const char **)a1, *(const char **)a2);
  return result;
}



// Function: rev_xstrcoll_df_btime @ 0xafd0
int rev_xstrcoll_df_btime(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8BF0(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8BF0(a1, a2);
}



// Function: xstrcoll_df_width @ 0xb060
int xstrcoll_df_width(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_9230(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_9230(a1, a2);
}



// Function: strcmp_df_extension @ 0xb0f0
long long strcmp_df_extension(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  long long result; // rax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return 0xFFFFFFFFLL;
    v5 = 0;
LABEL_8:
    result = v5 - (unsigned int)((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( (int)result )
      return result;
    return sub_71F0(a1, a2, &strcmp);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_71F0(a1, a2, &strcmp);
}



// Function: rev_xstrcoll_df_extension @ 0xb190
int rev_xstrcoll_df_extension(const char **a1, const char **a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *((int *)a2 + 42);
  v3 = *((int *)a1 + 42) == 3 || *((int *)a1 + 42) == 9;
  if ( v2 != 3 && v2 != 9 && (*((int *)a2 + 43) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*((int *)a1 + 43) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_9400(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_9400(a1, a2);
}



// Function: strcmp_df_size @ 0xb220
int strcmp_df_size(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  long long v6; // rax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = *(long long *)(a2 + 72);
  if ( *(long long *)(a1 + 72) > v6 )
    return -1;
  if ( *(long long *)(a1 + 72) == v6 )
    return strcmp(*(const char **)a1, *(const char **)a2);
  else
    return 1;
}



// Function: rev_xstrcoll_df_size @ 0xb2d0
int rev_xstrcoll_df_size(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  long long v6; // rax
  const char *v7; // rbp
  const char *v8; // r12

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 9 || *(int *)(a1 + 168) == 3;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = *(long long *)(a1 + 72);
  if ( *(long long *)(a2 + 72) > v6 )
    return -1;
  if ( *(long long *)(a2 + 72) != v6 )
    return 1;
  v7 = *(const char **)a2;
  v8 = *(const char **)a1;
  *__errno_location() = 0;
  return strcoll(v7, v8);
}



// Function: strcmp_df_width @ 0xb3a0
int strcmp_df_width(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8980(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8980(a1, a2);
}



// Function: rev_xstrcoll_df_width @ 0xb430
int rev_xstrcoll_df_width(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_91A0(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_91A0(a1, a2);
}



// Function: xstrcoll_df_extension @ 0xb4c0
int xstrcoll_df_extension(const char **a1, const char **a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *((int *)a2 + 42);
  v3 = *((int *)a1 + 42) == 3 || *((int *)a1 + 42) == 9;
  if ( v2 != 3 && v2 != 9 && (*((int *)a2 + 43) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*((int *)a1 + 43) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_9370(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_9370(a1, a2);
}



// Function: rev_strcmp_df_name @ 0xb550
int rev_strcmp_df_name(long long a1, long long a2)
{
  int v3; // esi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return strcmp(*(const char **)a2, *(const char **)a1);
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
  return strcmp(*(const char **)a2, *(const char **)a1);
}



// Function: xstrcoll_df_size @ 0xb5f0
int xstrcoll_df_size(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax
  long long v6; // rax
  const char *v7; // rbp
  const char *v8; // r12

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 9 || *(int *)(a1 + 168) == 3;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_9;
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
LABEL_9:
  v6 = *(long long *)(a2 + 72);
  if ( *(long long *)(a1 + 72) > v6 )
    return -1;
  if ( *(long long *)(a1 + 72) != v6 )
    return 1;
  v7 = *(const char **)a1;
  v8 = *(const char **)a2;
  *__errno_location() = 0;
  return strcoll(v7, v8);
}



// Function: strcmp_df_atime @ 0xb6c0
int strcmp_df_atime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a1 + 104) < *(long long *)(a2 + 104))
         - (*(long long *)(a1 + 104) > *(long long *)(a2 + 104))
         + 2 * ((*(long long *)(a1 + 96) < *(long long *)(a2 + 96)) - (*(long long *)(a1 + 96) > *(long long *)(a2 + 96)));
  if ( !result )
    return strcmp(*(const char **)a1, *(const char **)a2);
  return result;
}



// Function: rev_xstrcoll_df_atime @ 0xb790
int rev_xstrcoll_df_atime(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8DC0(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8DC0(a1, a2);
}



// Function: rev_strcmp_df_extension @ 0xb820
long long rev_strcmp_df_extension(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  long long result; // rax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return 0xFFFFFFFFLL;
    v5 = 0;
LABEL_8:
    result = v5 - (unsigned int)((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( (int)result )
      return result;
    return sub_71F0(a2, a1, &strcmp);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_71F0(a2, a1, &strcmp);
}



// Function: rev_strcmp_df_mtime @ 0xb8c0
int rev_strcmp_df_mtime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a2 + 120) < *(long long *)(a1 + 120))
         - (*(long long *)(a2 + 120) > *(long long *)(a1 + 120))
         + 2 * ((*(long long *)(a2 + 112) < *(long long *)(a1 + 112)) - (*(long long *)(a2 + 112) > *(long long *)(a1 + 112)));
  if ( !result )
    return strcmp(*(const char **)a2, *(const char **)a1);
  return result;
}



// Function: strcmp_df_mtime @ 0xb990
int strcmp_df_mtime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a1 + 120) < *(long long *)(a2 + 120))
         - (*(long long *)(a1 + 120) > *(long long *)(a2 + 120))
         + 2 * ((*(long long *)(a1 + 112) < *(long long *)(a2 + 112)) - (*(long long *)(a1 + 112) > *(long long *)(a2 + 112)));
  if ( !result )
    return strcmp(*(const char **)a1, *(const char **)a2);
  return result;
}



// Function: xstrcoll_df_atime @ 0xbaf0
int xstrcoll_df_atime(long long a1, long long a2)
{
  int v2; // ecx
  bool v3; // al
  int result; // eax
  int v5; // eax

  v2 = *(int *)(a2 + 168);
  v3 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v2 != 3 && v2 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v3 )
      return -1;
    v5 = 0;
LABEL_8:
    result = v5 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    return sub_8A00(a1, a2);
  }
  if ( !v3 )
  {
    v5 = 1;
    goto LABEL_8;
  }
  return sub_8A00(a1, a2);
}



// Function: strcmp_df_ctime @ 0xbc10
int strcmp_df_ctime(long long a1, long long a2)
{
  int v3; // edi
  bool v4; // al
  int result; // eax
  int v6; // eax

  v3 = *(int *)(a2 + 168);
  v4 = *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
  if ( v3 != 3 && v3 != 9 && (*(int *)(a2 + 172) & 0xF000) != 0x4000 )
  {
    if ( v4 )
      return -1;
    v6 = 0;
LABEL_8:
    result = v6 - ((*(int *)(a1 + 172) & 0xF000) == 0x4000);
    if ( result )
      return result;
    goto LABEL_10;
  }
  if ( !v4 )
  {
    v6 = 1;
    goto LABEL_8;
  }
LABEL_10:
  result = (*(long long *)(a1 + 136) < *(long long *)(a2 + 136))
         - (*(long long *)(a1 + 136) > *(long long *)(a2 + 136))
         + 2 * ((*(long long *)(a1 + 128) < *(long long *)(a2 + 128)) - (*(long long *)(a1 + 128) > *(long long *)(a2 + 128)));
  if ( !result )
    return strcmp(*(const char **)a1, *(const char **)a2);
  return result;
}



// Function: quote_name @ 0xbcf0
long long quote_name(void *a1, long long a2, unsigned int a3, long long a4, char a5, long long a6, const char *a7)
{
  long long v9; // rcx
  long long v12; // rsi
  long long v13; // rbp
  char v14; // al
  long long *v15; // r8
  char v16; // r13
  size_t v17; // r10
  long long v18; // r9
  void *v19; // r15
  char *v20; // rax
  char *v21; // rcx
  void *v22; // r9
  size_t v23; // r10
  long long *v24; // rdx
  long long *v25; // rdx
  unsigned char v26; // dl
  char *v27; // rax
  unsigned char v29; // dl
  char *v30; // rax
  char *IO_write_ptr; // rax
  void *ptr; // [rsp+8h] [rbp-2070h]
  void *ptra; // [rsp+8h] [rbp-2070h]
  long long v34; // [rsp+10h] [rbp-2068h]
  size_t v35; // [rsp+10h] [rbp-2068h]
  size_t n; // [rsp+18h] [rbp-2060h]
  unsigned char v37; // [rsp+27h] [rbp-2051h] BYREF
  void *v38; // [rsp+28h] [rbp-2050h] BYREF
  char v39[24]; // [rsp+30h] [rbp-2048h] BYREF
  unsigned long long v40; // [rsp+2038h] [rbp-40h]

  v9 = a3;
  v40 = __readfsqword(0x28u);
  v12 = (long long)a1;
  v38 = v39;
  v13 = sub_7920(&v38, a1, a2, v9, 0, &v37);
  if ( v37 && a5 )
  {
    ++qword_261F8;
    IO_write_ptr = stdout->_IO_write_ptr;
    if ( IO_write_ptr >= stdout->_IO_write_end )
    {
      v12 = 32;
      __overflow(stdout, 32);
    }
    else
    {
      stdout->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 32;
    }
  }
  if ( a4 )
  {
    v14 = sub_6C80(4u);
    v15 = qword_25060;
    if ( v14 )
    {
      sub_7880((long long)qword_25060, v12);
      sub_7880((long long)&qword_25070, v12);
      v15 = &qword_25070 - 2;
    }
    sub_7880((long long)v15, v12);
    sub_7880(a4, v12);
    sub_7880((long long)&qword_25070, v12);
  }
  if ( a7 )
  {
    v16 = byte_263A8;
    if ( byte_263A8 && (v16 = byte_263A9) != 0 )
    {
      if ( v37 )
      {
        v17 = v13;
        v18 = 0;
        v16 = 0;
      }
      else
      {
        v17 = v13 - 2;
        v29 = *(char *)v38;
        v30 = stdout->_IO_write_ptr;
        if ( v30 >= stdout->_IO_write_end )
        {
          __overflow(stdout, v29);
          v17 = v13 - 2;
          v18 = 1;
        }
        else
        {
          v18 = 1;
          stdout->_IO_write_ptr = v30 + 1;
          *v30 = v29;
        }
      }
    }
    else
    {
      v17 = v13;
      v18 = 0;
    }
    n = v17;
    v34 = v18;
    v19 = (void *)sub_72A0((const char *)qword_26388, 0);
    v20 = (char *)sub_72A0(a7, 1);
    v21 = (char *)"";
    if ( *v20 != 47 )
      v21 = "/";
    ptr = v20;
    __printf_chk(1, "\x1B]8;;file://%s%s%s\a", v19, v21);
    free(v19);
    free(ptr);
    v22 = (void *)v34;
    v23 = n;
    if ( a6 )
      goto LABEL_13;
LABEL_34:
    fwrite_unlocked((char *)v38 + (long long)v22, 1u, v23, stdout);
    qword_261F8 += v13;
    goto LABEL_21;
  }
  v23 = v13;
  v22 = 0;
  v16 = 0;
  if ( !a6 )
    goto LABEL_34;
LABEL_13:
  if ( byte_26318 )
  {
    v24 = *(long long **)(a6 + 24);
    if ( *(long long *)(a6 + 32) - (long long)v24 <= 7u )
    {
      v35 = v23;
      ptra = v22;
      obstack_newchunk(a6, 8);
      v24 = *(long long **)(a6 + 24);
      v23 = v35;
      v22 = ptra;
    }
    *v24 = qword_261F8;
    *(long long *)(a6 + 24) += 8LL;
  }
  fwrite_unlocked((char *)v38 + (long long)v22, 1u, v23, stdout);
  qword_261F8 += v13;
  if ( byte_26318 )
  {
    v25 = *(long long **)(a6 + 24);
    if ( *(long long *)(a6 + 32) - (long long)v25 <= 7u )
    {
      obstack_newchunk(a6, 8);
      v25 = *(long long **)(a6 + 24);
    }
    *v25 = qword_261F8;
    *(long long *)(a6 + 24) += 8LL;
  }
LABEL_21:
  if ( a7 )
  {
    fputs_unlocked("\x1B]8;;\a", stdout);
    if ( v16 )
    {
      v26 = *((char *)v38 + v13 - 1);
      v27 = stdout->_IO_write_ptr;
      if ( v27 >= stdout->_IO_write_end )
      {
        __overflow(stdout, v26);
      }
      else
      {
        stdout->_IO_write_ptr = v27 + 1;
        *v27 = v26;
      }
    }
  }
  if ( v38 != a1 && v38 != v39 )
    free(v38);
  return v13 + v37;
}



// Function: print_name_with_quoting @ 0xc0e0
long long print_name_with_quoting(char **a1, char a2, long long a3, unsigned long long a4)
{
  char *v7; // r13
  sigset_t *v8; // rsi
  long long v9; // r12
  int v11; // ebx
  int v12; // eax
  char v13; // cl
  long long v14; // rdx
  unsigned long long v15; // rcx
  char *v16; // rcx
  sigset_t *v17; // rsi
  int v18; // edx
  size_t v19; // rax
  long long *v20; // rbx
  size_t v21; // rcx
  int v22; // eax
  char v23; // al
  long long v24; // rcx
  char v25; // al
  size_t v26; // [rsp+8h] [rbp-40h]

  if ( !a2 )
  {
    v7 = *a1;
    if ( !byte_26312 )
      goto LABEL_3;
    v12 = *((unsigned char *)a1 + 185);
    if ( byte_26390 && (char)v12 )
      v11 = *((int *)a1 + 43);
    else
      v11 = *((int *)a1 + 12);
    goto LABEL_11;
  }
  v7 = a1[1];
  if ( !byte_26312 )
  {
LABEL_3:
    v8 = (sigset_t *)qword_262D0;
    v9 = sub_BCF0(v7, qword_262D0, *((int *)a1 + 49), 0, a2 ^ 1u, a3, a1[2]);
    sub_84F0((long long)v7, v8);
    return v9;
  }
  v11 = *((int *)a1 + 43);
  if ( !*((char *)a1 + 185) )
  {
    if ( (unsigned char)sub_6C80(0xCu) )
    {
      v15 = 192;
      goto LABEL_17;
    }
    v12 = -1;
LABEL_11:
    v13 = *((char *)a1 + 184);
    if ( !v13 )
      goto LABEL_12;
    goto LABEL_29;
  }
  v13 = *((char *)a1 + 184);
  v12 = 0;
  if ( !v13 )
  {
LABEL_12:
    v14 = dword_1A6A0[*((unsigned int *)a1 + 42)];
    v13 = (int)v14 == 7;
    if ( (int)v14 != 5 )
    {
LABEL_13:
      if ( v12 || !v13 )
      {
        v15 = 16 * v14;
      }
      else
      {
        v15 = 208;
        if ( !byte_26390 )
          v15 = (-(long long)((unsigned char)sub_6C80(0xDu) == 0) & 0xFFFFFFFFFFFFFFA0LL) + 208;
      }
      goto LABEL_17;
    }
    goto LABEL_39;
  }
LABEL_29:
  v18 = v11 & 0xF000;
  if ( v18 != 0x8000 )
  {
    if ( v18 == 0x4000 )
    {
      if ( (v11 & 0x202) != 0x202 || (v25 = sub_6C80(0x14u), v15 = 320, !v25) )
      {
        if ( (v11 & 2) == 0 || (v23 = sub_6C80(0x13u), v15 = 304, !v23) )
        {
          v15 = 96;
          if ( (v11 & 0x200) != 0 )
          {
            v24 = -(long long)((unsigned char)sub_6C80(0x12u) == 0);
            LOBYTE(v24) = v24 & 0x40;
            v15 = v24 + 288;
          }
        }
      }
      goto LABEL_17;
    }
    if ( v18 != 40960 )
    {
      v15 = 128;
      if ( v18 != 4096 )
      {
        v15 = 144;
        if ( v18 != 49152 )
        {
          v15 = 160;
          if ( v18 != 24576 )
          {
            v15 = 176;
            if ( v18 != 0x2000 )
              v15 = 208;
          }
        }
      }
      goto LABEL_17;
    }
    v14 = 7;
    goto LABEL_13;
  }
  if ( (v11 & 0x800) != 0 && (unsigned char)sub_6C80(0x10u) )
  {
    v15 = 256;
    goto LABEL_17;
  }
  if ( (v11 & 0x400) != 0 && (unsigned char)sub_6C80(0x11u) )
  {
    v15 = 272;
    goto LABEL_17;
  }
  if ( (unsigned char)sub_6C80(0x15u) && *((char *)a1 + 192) )
  {
    v15 = 336;
    goto LABEL_17;
  }
  if ( (v11 & 0x49) != 0 && (unsigned char)sub_6C80(0xEu) )
  {
    v15 = 224;
    goto LABEL_17;
  }
  if ( (unsigned long long)a1[5] > 1 && (unsigned char)sub_6C80(0x16u) )
  {
    v15 = 352;
    goto LABEL_17;
  }
LABEL_39:
  v19 = strlen(v7);
  v20 = qword_26308;
  v21 = v19;
  if ( !qword_26308 )
  {
LABEL_43:
    v15 = 80;
LABEL_17:
    v16 = (char *)qword_25060 + v15;
    goto LABEL_18;
  }
  while ( 1 )
  {
    if ( v21 >= *v20 )
    {
      v26 = v21;
      v22 = sub_EEB0(&v7[v21 - *v20], v20[1]);
      v21 = v26;
      if ( !v22 )
        break;
    }
    v20 = (long long *)v20[4];
    if ( !v20 )
      goto LABEL_43;
  }
  v16 = (char *)(v20 + 2);
LABEL_18:
  if ( !*((long long *)v16 + 1) )
  {
    if ( !(unsigned char)sub_6C80(4u) )
      goto LABEL_3;
    v16 = 0;
  }
  v17 = (sigset_t *)qword_262D0;
  v9 = sub_BCF0(v7, qword_262D0, *((int *)a1 + 49), (long long)v16, a2 ^ 1u, a3, a1[2]);
  sub_84F0((long long)v7, v17);
  if ( qword_25088 )
  {
    sub_7880((long long)&unk_25080, (long long)v17);
  }
  else
  {
    sub_7880((long long)qword_25060, (long long)v17);
    sub_7880((long long)&unk_25090, (long long)v17);
    sub_7880((long long)&qword_25070, (long long)v17);
  }
  if ( qword_262B0 && a4 / qword_262B0 != (v9 + a4 - 1) / qword_262B0 )
    sub_7880((long long)&unk_251D0, a4 / qword_262B0);
  return v9;
}



// Function: print_with_separator @ 0xc7c0
int print_with_separator(char a1, long long a2, long long a3, long long a4)
{
  unsigned long long v4; // r13
  unsigned long long i; // rbx
  unsigned long long v6; // rbp
  long long v7; // r12
  long long v8; // rax
  unsigned char v9; // cl
  char v10; // dl
  char *IO_write_ptr; // rax
  char *v12; // rax
  char v13; // dl
  char *v14; // rax
  char v16; // [rsp+Bh] [rbp-3Dh]
  unsigned char v17; // [rsp+Ch] [rbp-3Ch]

  if ( qword_263B0 )
  {
    v4 = 0;
    for ( i = 0; qword_263B0 > i; ++i )
    {
      v7 = *((long long *)qword_263A0 + i);
      if ( qword_262B0 )
      {
        v8 = sub_7E10(*((long long *)qword_263A0 + i), a2, a3, a4);
        if ( i )
        {
          v6 = v4 + 2;
          if ( !qword_262B0 || qword_262B0 > v8 + v4 + 2 && -3 - v8 >= v4 )
          {
            v4 += v8 + 2;
            goto LABEL_11;
          }
          v9 = byte_25019;
          v4 = v8;
          v6 = 0;
          v10 = byte_25019;
LABEL_12:
          IO_write_ptr = stdout->_IO_write_ptr;
          if ( IO_write_ptr >= stdout->_IO_write_end )
          {
            v17 = v9;
            v16 = v10;
            __overflow(stdout, a1);
            v10 = v16;
            v9 = v17;
          }
          else
          {
            stdout->_IO_write_ptr = IO_write_ptr + 1;
            *IO_write_ptr = a1;
          }
          v12 = stdout->_IO_write_ptr;
          if ( v12 >= stdout->_IO_write_end )
          {
            __overflow(stdout, v9);
          }
          else
          {
            stdout->_IO_write_ptr = v12 + 1;
            *v12 = v10;
          }
          goto LABEL_4;
        }
        v6 = v4;
        v4 += v8;
      }
      else
      {
        v6 = v4;
        if ( i )
        {
          v6 = v4 + 2;
          v4 += 2LL;
LABEL_11:
          v9 = 32;
          v10 = 32;
          goto LABEL_12;
        }
      }
LABEL_4:
      a2 = v6;
      sub_C590(v7, v6);
    }
  }
  v13 = byte_25019;
  v14 = stdout->_IO_write_ptr;
  if ( v14 >= stdout->_IO_write_end )
  {
    LODWORD(v14) = __overflow(stdout, (unsigned char)byte_25019);
  }
  else
  {
    stdout->_IO_write_ptr = v14 + 1;
    *v14 = v13;
  }
  return (int)v14;
}



// Function: print_long_format @ 0xc980
unsigned long long print_long_format(long long a1)
{
  int v2; // eax
  bool v3; // cc
  __syscall_slong_t v4; // rax
  bool v5; // r14
  char *v6; // rbx
  const char *v7; // r15
  int v8; // ebp
  int v9; // eax
  char *v10; // rax
  char v11; // al
  const char *v12; // rax
  char *v13; // rbx
  const char *v14; // r15
  int v15; // ebp
  int v16; // ebp
  char *v17; // rbp
  char v18; // al
  const char *v19; // rbx
  int v20; // r8d
  char *v21; // rbx
  size_t v22; // rbx
  long long v23; // rax
  long long v24; // rcx
  long long v25; // r8
  long long v26; // rdx
  long long v27; // rbp
  __syscall_slong_t v29; // rdx
  int v30; // eax
  __syscall_slong_t v31; // rax
  size_t v32; // rax
  long long v33; // rcx
  long long v34; // r8
  unsigned char v35; // al
  char *IO_write_ptr; // rdx
  size_t v37; // rax
  const char *v38; // r9
  long long v39; // rdi
  char *v40; // rdi
  char *v41; // rdi
  char *v42; // rdi
  __time_t v43; // rdx
  __time_t tv_sec; // rcx
  int v45; // r8d
  int v46; // eax
  int v47; // esi
  int v48; // eax
  char *v49; // rdx
  long long v50; // rax
  int v51; // ebp
  unsigned long long v52; // rax
  unsigned int v53; // edi
  const char *v54; // r15
  const char *v55; // r9
  int v56; // eax
  char *v57; // rdx
  long long v58; // rax
  int v59; // [rsp+4h] [rbp-1354h]
  long long v60; // [rsp+18h] [rbp-1340h] BYREF
  __time_t v61; // [rsp+20h] [rbp-1338h] BYREF
  __syscall_slong_t v62; // [rsp+28h] [rbp-1330h]
  char v63[16]; // [rsp+30h] [rbp-1328h] BYREF
  int v64; // [rsp+40h] [rbp-1318h]
  char v65[16]; // [rsp+70h] [rbp-12E8h] BYREF
  int v66; // [rsp+80h] [rbp-12D8h]
  char v67[12]; // [rsp+B4h] [rbp-12A4h] BYREF
  char v68[32]; // [rsp+C0h] [rbp-1298h] BYREF
  char v69[584]; // [rsp+E0h] [rbp-1278h] BYREF
  char v70[3656]; // [rsp+4D0h] [rbp-E88h] BYREF
  unsigned long long v71; // [rsp+1318h] [rbp-40h]

  v71 = __readfsqword(0x28u);
  if ( *(char *)(a1 + 184) )
  {
    sub_FBA0(a1 + 24, v67);
  }
  else
  {
    v67[0] = aPcdbLswd[*(unsigned int *)(a1 + 168)];
    memset(&v67[1], 63, 10);
  }
  if ( !byte_26364 )
  {
    v67[10] = 0;
    goto LABEL_5;
  }
  v30 = *(int *)(a1 + 188);
  if ( v30 == 1 )
  {
    v67[10] = 46;
LABEL_5:
    v2 = dword_26334;
    v3 = (unsigned int)dword_26334 <= 2;
    if ( dword_26334 != 2 )
      goto LABEL_6;
LABEL_49:
    v31 = *(long long *)(a1 + 104);
    v5 = 1;
    v61 = *(long long *)(a1 + 96);
    v62 = v31;
    goto LABEL_10;
  }
  if ( v30 != 2 )
    goto LABEL_5;
  v2 = dword_26334;
  v67[10] = 43;
  v3 = (unsigned int)dword_26334 <= 2;
  if ( dword_26334 == 2 )
    goto LABEL_49;
LABEL_6:
  if ( v3 )
  {
    if ( v2 )
    {
      v4 = *(long long *)(a1 + 136);
      v5 = 1;
      v61 = *(long long *)(a1 + 128);
    }
    else
    {
      v4 = *(long long *)(a1 + 120);
      v5 = 1;
      v61 = *(long long *)(a1 + 112);
    }
    v62 = v4;
  }
  else
  {
    if ( v2 != 3 )
      abort();
    v29 = *(long long *)(a1 + 120);
    v61 = *(long long *)(a1 + 112);
    v62 = v29;
    v5 = (v29 & v61) != -1;
  }
LABEL_10:
  v6 = v70;
  if ( byte_262FC )
  {
    v38 = "?";
    if ( *(char *)(a1 + 184) )
    {
      v39 = *(long long *)(a1 + 32);
      if ( v39 )
        v38 = (const char *)sub_12720(v39, v69);
    }
    v6 = &v70[(int)__sprintf_chk(v70, 1, 3643, "%*s ", dword_26360, v38)];
  }
  if ( byte_2632C )
  {
    v7 = "?";
    if ( *(char *)(a1 + 184) )
      v7 = (const char *)sub_11550(*(long long *)(a1 + 88), v69, (unsigned int)dword_26328, 512, qword_26320);
    v8 = dword_2635C;
    v9 = v8 - sub_12EC0(v7, 0);
    if ( v9 <= 0 )
      goto LABEL_72;
    v10 = &v6[v9];
    do
      *v6++ = 32;
    while ( v6 != v10 );
    while ( 1 )
    {
      ++v7;
      v6 = v10 + 1;
      v11 = *(v7 - 1);
      *(v6 - 1) = v11;
      if ( !v11 )
        break;
LABEL_72:
      v10 = v6;
    }
    *(v6 - 1) = 32;
  }
  v12 = "?";
  if ( *(char *)(a1 + 184) )
    v12 = (const char *)sub_12720(*(long long *)(a1 + 40), v69);
  v13 = &v6[(int)__sprintf_chk(v6, 1, -1, "%s %*s ", v67, dword_26358, v12)];
  if ( byte_26318 )
  {
    v37 = strlen("  ");
    sub_7430("  ", v37);
  }
  if ( byte_25029 || byte_25028 || byte_2632E || byte_26365 )
  {
    sub_7430(v70, v13 - v70);
    if ( byte_25029 )
    {
      v42 = "?";
      if ( *(char *)(a1 + 184) )
      {
        v42 = 0;
        if ( !byte_2632D )
          v42 = (char *)sub_12340(*(int *)(a1 + 52));
      }
      sub_92C0(v42);
    }
    if ( byte_25028 )
    {
      v40 = "?";
      if ( *(char *)(a1 + 184) )
      {
        v40 = 0;
        if ( !byte_2632D )
          v40 = (char *)sub_124E0(*(int *)(a1 + 56));
      }
      sub_92C0(v40);
      if ( !byte_2632E )
      {
LABEL_28:
        v13 = v70;
        if ( !byte_26365 )
          goto LABEL_29;
        goto LABEL_69;
      }
    }
    else if ( !byte_2632E )
    {
      goto LABEL_28;
    }
    v41 = "?";
    if ( *(char *)(a1 + 184) )
    {
      v41 = 0;
      if ( !byte_2632D )
        v41 = (char *)sub_12340(*(int *)(a1 + 52));
    }
    sub_92C0(v41);
    v13 = v70;
    if ( byte_26365 )
LABEL_69:
      sub_92C0(*(char **)(a1 + 176));
  }
LABEL_29:
  if ( !*(char *)(a1 + 184) )
  {
    v14 = "?";
LABEL_31:
    v15 = dword_2633C;
    v16 = v15 - sub_12EC0(v14, 0);
    if ( v16 <= 0 )
    {
      v17 = v13;
    }
    else
    {
      v17 = &v13[v16];
      do
        *v13++ = 32;
      while ( v13 != v17 );
    }
    do
    {
      v18 = *v14++;
      *v17++ = v18;
    }
    while ( v18 );
    *(v17 - 1) = 32;
    goto LABEL_36;
  }
  if ( (*(int *)(a1 + 48) & 0xB000) != 0x2000 )
  {
    v14 = (const char *)sub_11550(*(long long *)(a1 + 72), v69, (unsigned int)dword_2631C, 1, qword_25020);
    goto LABEL_31;
  }
  v51 = dword_2633C - (dword_26344 + dword_26340 + 2);
  v52 = *(long long *)(a1 + 64);
  v53 = v52 >> 12;
  LOBYTE(v53) = 0;
  v54 = (const char *)sub_12720((unsigned char)v52 | v53, v69);
  v59 = dword_26340;
  v55 = (const char *)sub_12720(
                        (*(long long *)(a1 + 64) >> 8) & 0xFFF | (unsigned int)HIDWORD(*(long long *)(a1 + 64)) & 0xFFFFF000,
                        v68);
  v56 = 0;
  if ( v51 >= 0 )
    v56 = v51;
  v17 = &v13[(int)__sprintf_chk(v13, 1, -1, "%*s, %*s ", dword_26344 + v56, v55, v59, v54)];
LABEL_36:
  *v17 = 1;
  if ( !*(char *)(a1 + 184) || !v5 )
    goto LABEL_38;
  if ( sub_17C60(qword_262A8, &v61, v63) )
  {
    v43 = v61;
    tv_sec = tp.tv_sec;
    v45 = tp.tv_nsec > v62;
    v46 = tp.tv_nsec < v62;
    v47 = v45 - v46;
    if ( v45 - v46 + 2 * ((tp.tv_sec > v61) - (tp.tv_sec < v61)) < 0 )
    {
      sub_10200(&tp);
      v45 = v62 < tp.tv_nsec;
      v43 = v61;
      tv_sec = tp.tv_sec;
      v46 = v62 > tp.tv_nsec;
      v47 = v45 - v46;
    }
    v48 = (v47 + 2 * ((tv_sec - 15778476 > v43) - (tv_sec - 15778476 < v43)))
        & (v46 - v45 + 2 * ((tv_sec < v43) - (tv_sec > v43)));
    v49 = byte_253C8
        ? (char *)&unk_253E0 + 128 * (v64 + (unsigned long long)((v48 >> 31) & 0xC))
        : (&off_25040)[(unsigned int)v48 >> 31];
    v50 = sub_14D80(v17, 1001, v49, v63, qword_262A8);
    if ( v50 )
    {
      v17 += v50;
LABEL_90:
      *v17 = 32;
      v21 = v17 + 1;
      goto LABEL_41;
    }
  }
  if ( !*v17 )
    goto LABEL_90;
  if ( *(char *)(a1 + 184) )
  {
    v19 = (const char *)sub_12680(v61, v68);
    goto LABEL_39;
  }
LABEL_38:
  v19 = "?";
LABEL_39:
  v20 = dword_25014;
  if ( dword_25014 < 0 )
  {
    v60 = 0;
    if ( !sub_17C60(qword_262A8, &v60, v65) )
      goto LABEL_101;
    v57 = off_25040;
    if ( byte_253C8 )
      v57 = (char *)&unk_253E0 + 128 * (long long)v66;
    v58 = sub_14D80(v69, 1001, v57, v65, qword_262A8);
    if ( v58 )
    {
      dword_25014 = sub_12CA0(v69, v58, 0);
      v20 = dword_25014;
    }
    else
    {
LABEL_101:
      v20 = dword_25014;
    }
    if ( v20 < 0 )
    {
      dword_25014 = 0;
      v20 = 0;
    }
  }
  v21 = &v17[(int)__sprintf_chk(v17, 1, -1, "%*s ", v20, v19)];
LABEL_41:
  v22 = v21 - v70;
  sub_7430(v70, v22);
  v23 = sub_C0E0((char **)a1, 0, (long long)&unk_261A0, v22);
  v26 = *(unsigned int *)(a1 + 168);
  v27 = v23;
  if ( (int)v26 == 6 )
  {
    if ( *(long long *)(a1 + 8) )
    {
      v32 = strlen(" -> ");
      sub_7430(" -> ", v32);
      sub_C0E0((char **)a1, 1, 0, v22 + v27 + 4);
      if ( dword_26314 )
      {
        v35 = sub_75B0(1, *(unsigned int *)(a1 + 172), 0, v33, v34);
        if ( v35 )
          goto LABEL_59;
      }
    }
  }
  else if ( dword_26314 )
  {
    v35 = sub_75B0(*(unsigned char *)(a1 + 184), *(unsigned int *)(a1 + 48), v26, v24, v25);
    if ( v35 )
    {
LABEL_59:
      ++qword_261F8;
      IO_write_ptr = stdout->_IO_write_ptr;
      if ( IO_write_ptr >= stdout->_IO_write_end )
      {
        __overflow(stdout, v35);
      }
      else
      {
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = v35;
      }
    }
  }
  return v71 - __readfsqword(0x28u);
}



// Function: print_current_files @ 0xd420
int print_current_files(long long a1, unsigned long long a2, long long a3, unsigned long long a4)
{
  int v4; // eax
  int v5; // kr00_4
  char *IO_write_ptr; // rax
  long long v7; // r14
  long long v8; // rbp
  long long v9; // rcx
  long long v10; // r15
  long long v11; // r12
  unsigned long long v12; // r13
  unsigned long long v13; // rbx
  char v14; // dl
  char *v15; // rax
  long long v16; // r12
  long long v17; // rdx
  long long v18; // rcx
  unsigned long long v19; // rbp
  unsigned long long v20; // rdi
  char v21; // di
  unsigned long long i; // rbx
  unsigned long long j; // rbx
  char v24; // si
  unsigned long long v25; // rcx
  long long v26; // r15
  unsigned long long v27; // rdx
  unsigned long long v28; // r12
  long long v29; // r13
  unsigned long long v30; // r14
  unsigned long long v31; // rdi
  long long v32; // rax
  long long v33; // rbx
  long long v34; // rbp
  char *v35; // rax
  char v36; // dl
  unsigned long long v38; // [rsp+0h] [rbp-50h]
  long long v39; // [rsp+0h] [rbp-50h]
  char *v40; // [rsp+8h] [rbp-48h]
  unsigned long long v41; // [rsp+10h] [rbp-40h]

  v5 = v4;
  LODWORD(IO_write_ptr) = dword_26338;
  switch ( dword_26338 )
  {
    case 0:
      for ( i = 0; qword_263B0 > i; ++i )
      {
        if ( byte_26312 && (unsigned char)sub_6C80(4u) )
        {
          sub_7880((long long)qword_25060, a2);
          sub_7880((long long)&unk_250A0, a2);
          sub_7880((long long)&unk_250A0 - 48, a2);
        }
        sub_C980(*((long long *)qword_263A0 + i));
        ++qword_261F8;
        a2 = (unsigned char)byte_25019;
        IO_write_ptr = stdout->_IO_write_ptr;
        if ( IO_write_ptr >= stdout->_IO_write_end )
        {
          LODWORD(IO_write_ptr) = __overflow(stdout, (unsigned char)byte_25019);
        }
        else
        {
          stdout->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = a2;
        }
      }
      break;
    case 1:
      for ( j = 0; qword_263B0 > j; ++j )
      {
        sub_C590(*((long long *)qword_263A0 + j), 0);
        v24 = byte_25019;
        IO_write_ptr = stdout->_IO_write_ptr;
        if ( IO_write_ptr >= stdout->_IO_write_end )
        {
          LODWORD(IO_write_ptr) = __overflow(stdout, (unsigned char)byte_25019);
        }
        else
        {
          stdout->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = v24;
        }
      }
      break;
    case 2:
      if ( !qword_262B0 )
        goto LABEL_34;
      v41 = 0;
      v25 = sub_7FF0(1, a2, a3, a4);
      v26 = qword_26208 + 24 * v25 - 24;
      v27 = qword_263B0 % v25;
      IO_write_ptr = (char *)(qword_263B0 / v25 - ((qword_263B0 % v25 == 0) - 1LL));
      v40 = IO_write_ptr;
      if ( IO_write_ptr )
      {
        do
        {
          v28 = v41;
          v29 = 0;
          v30 = 0;
          while ( 1 )
          {
            v39 = *((long long *)qword_263A0 + v28);
            v32 = sub_7E10(v39, a2, v27, v25);
            a2 = v30;
            v33 = v32;
            v34 = *(long long *)(*(long long *)(v26 + 16) + v29);
            v29 += 8;
            sub_C590(v39, v30);
            v28 += (unsigned long long)v40;
            if ( v28 >= qword_263B0 )
              break;
            v31 = v33 + v30;
            a2 = v30 + v34;
            v30 = a2;
            sub_7360(v31, a2);
          }
          v27 = (unsigned char)byte_25019;
          v35 = stdout->_IO_write_ptr;
          if ( v35 >= stdout->_IO_write_end )
          {
            a2 = (unsigned char)byte_25019;
            __overflow(stdout, (unsigned char)byte_25019);
          }
          else
          {
            v25 = (unsigned long long)(v35 + 1);
            stdout->_IO_write_ptr = v35 + 1;
            *v35 = v27;
          }
          LODWORD(IO_write_ptr) = ++v41;
        }
        while ( v40 != (char *)v41 );
      }
      break;
    case 3:
      if ( !qword_262B0 )
      {
LABEL_34:
        v21 = 32;
        goto LABEL_11;
      }
      v38 = sub_7FF0(0, a2, a3, a4);
      v7 = qword_26208 + 24 * v38 - 24;
      v8 = *(long long *)qword_263A0;
      v10 = sub_7E10(*(long long *)qword_263A0, a2, 3 * v38, v9);
      v11 = **(long long **)(v7 + 16);
      sub_C590(v8, 0);
      if ( (unsigned long long)qword_263B0 > 1 )
      {
        v12 = 0;
        v13 = 1;
        do
        {
          v19 = v13 % v38;
          if ( v13 % v38 )
          {
            v20 = v12 + v10;
            v12 += v11;
            sub_7360(v20, v12);
          }
          else
          {
            v14 = byte_25019;
            v15 = stdout->_IO_write_ptr;
            v12 = 0;
            if ( v15 >= stdout->_IO_write_end )
            {
              __overflow(stdout, (unsigned char)byte_25019);
            }
            else
            {
              stdout->_IO_write_ptr = v15 + 1;
              *v15 = v14;
            }
          }
          v16 = *((long long *)qword_263A0 + v13++);
          sub_C590(v16, v12);
          v10 = sub_7E10(v16, v12, v17, v18);
          v11 = *(long long *)(*(long long *)(v7 + 16) + 8 * v19);
        }
        while ( v13 < qword_263B0 );
      }
      v36 = byte_25019;
      IO_write_ptr = stdout->_IO_write_ptr;
      if ( IO_write_ptr >= stdout->_IO_write_end )
      {
        LODWORD(IO_write_ptr) = __overflow(stdout, (unsigned char)byte_25019);
      }
      else
      {
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = v36;
      }
      break;
    case 4:
      v21 = 44;
LABEL_11:
      LODWORD(IO_write_ptr) = sub_C7C0(v21, a2, a3, a4);
      break;
    default:
      LODWORD(IO_write_ptr) = v5;
      break;
  }
  return (int)IO_write_ptr;
}



// Function: print_dir @ 0xd820
void print_dir(char *src, unsigned long long a2, unsigned char a3)
{
  void *v3; // r14
  int *v6; // rax
  int *v7; // rbp
  DIR *v8; // rax
  DIR *v9; // r12
  int v10; // eax
  long long v11; // r15
  long long *v12; // rax
  void *v13; // rdi
  long long v14; // rax
  long long v15; // rdx
  long long v16; // r13
  char *v17; // rax
  long long *v18; // rax
  char *v19; // r15
  size_t v20; // rax
  const char *v21; // rdi
  struct dirent *v22; // rax
  struct dirent *v23; // rbx
  char *d_name; // r14
  long long v25; // r15
  char *IO_write_ptr; // rax
  bool v27; // zf
  size_t v28; // rax
  char *v29; // rax
  char *v30; // rax
  long long v31; // rdi
  long long v32; // rdx
  unsigned long long v33; // rcx
  const char *v34; // rbx
  size_t v35; // rax
  char v36; // dl
  const char *v37; // r12
  char *v38; // rax
  char *v39; // rbx
  char *v40; // rbp
  size_t v41; // rax
  long long v42; // r15
  long long v43; // rdx
  unsigned long long v44; // rcx
  size_t v45; // rax
  char *v46; // rsi
  unsigned long long v47; // [rsp-8h] [rbp-390h]
  long long v48; // [rsp+8h] [rbp-380h]
  unsigned int v49; // [rsp+8h] [rbp-380h]
  void *ptr; // [rsp+10h] [rbp-378h]
  char *ptra; // [rsp+10h] [rbp-378h]
  long long v53[18]; // [rsp+20h] [rbp-368h] BYREF
  char v54[663]; // [rsp+B1h] [rbp-2D7h] BYREF
  unsigned long long v55; // [rsp+348h] [rbp-40h]

  v3 = (void *)a2;
  v55 = __readfsqword(0x28u);
  v6 = __errno_location();
  *v6 = 0;
  v7 = v6;
  v8 = opendir(src);
  if ( !v8 )
  {
    v46 = dcgettext(0, "cannot open directory %s", 5);
    sub_7540(a3, v46, (long long)src);
    return;
  }
  v9 = v8;
  if ( !qword_263C8 )
    goto LABEL_12;
  v10 = dirfd(v8);
  if ( v10 < 0 )
  {
    if ( (int)sub_7050(4294967196LL, (long long)src, (long long)v53, 0, 0x100u) >= 0 )
      goto LABEL_5;
LABEL_37:
    v30 = dcgettext(0, "cannot determine device and inode of %s", 5);
    sub_7540(a3, v30, (long long)src);
    closedir(v9);
    return;
  }
  if ( (int)sub_7050((unsigned int)v10, (long long)"", (long long)v53, 0x1000u, 0x100u) < 0 )
    goto LABEL_37;
LABEL_5:
  v11 = v53[0];
  v48 = v53[1];
  v12 = (long long *)sub_18590(16);
  v13 = qword_263C8;
  v12[1] = v11;
  a2 = (unsigned long long)v12;
  *v12 = v48;
  ptr = v12;
  v14 = sub_112C0(v13, v12);
  if ( !v14 )
    sub_18A90(v13, a2, v15);
  if ( ptr != (void *)v14 )
  {
    free(ptr);
    v16 = sub_17150(0, 3, src);
    v17 = dcgettext(0, "%s: not listing already-listed directory", 5);
    error(0, 0, v17, v16);
    closedir(v9);
    dword_26210 = 2;
    return;
  }
  v18 = (long long *)qword_260F8;
  if ( (unsigned long long)(qword_26100 - qword_260F8) <= 0xF )
  {
    a2 = 16;
    obstack_newchunk(&unk_260E0, 16);
    v18 = (long long *)qword_260F8;
  }
  qword_260F8 = (long long)(v18 + 2);
  v18[1] = v11;
  *v18 = v48;
LABEL_12:
  sub_8730();
  if ( byte_262F6 )
  {
    if ( byte_25010 )
      goto LABEL_14;
LABEL_31:
    ++qword_261F8;
    IO_write_ptr = stdout->_IO_write_ptr;
    if ( IO_write_ptr < stdout->_IO_write_end )
    {
      v27 = byte_26318 == 0;
      byte_25010 = 0;
      stdout->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 10;
      if ( v27 )
      {
LABEL_15:
        v19 = 0;
        if ( !byte_26311 )
        {
LABEL_16:
          if ( !v3 )
            v3 = src;
          sub_BCF0(v3, qword_262C8, 0xFFFFFFFF, 0, 1, (long long)&unk_26140, v19);
          free(v19);
          v20 = strlen(":\n");
          sub_7430(":\n", v20);
          a2 = v47;
          goto LABEL_19;
        }
LABEL_34:
        v19 = (char *)sub_EF30(src, 2);
        if ( !v19 )
        {
          v29 = dcgettext(0, "error canonicalizing %s", 5);
          sub_7540(a3, v29, (long long)src);
        }
        goto LABEL_16;
      }
LABEL_33:
      v28 = strlen("  ");
      v19 = 0;
      sub_7430("  ", v28);
      if ( !byte_26311 )
        goto LABEL_16;
      goto LABEL_34;
    }
    __overflow(stdout, 10);
LABEL_14:
    byte_25010 = 0;
    if ( !byte_26318 )
      goto LABEL_15;
    goto LABEL_33;
  }
  if ( byte_262B8 )
  {
    if ( byte_25010 )
      goto LABEL_14;
    goto LABEL_31;
  }
LABEL_19:
  ptra = 0;
  v49 = a3;
  while ( 1 )
  {
    *v7 = 0;
    v21 = (const char *)v9;
    v22 = readdir(v9);
    v23 = v22;
    if ( !v22 )
      break;
    d_name = v22->d_name;
    if ( dword_262F0 == 2 )
      goto LABEL_53;
    if ( v22->d_name[0] == 46 )
    {
      if ( !dword_262F0 || !v22->d_name[(v22->d_name[1] == 46) + 1] )
        goto LABEL_28;
LABEL_53:
      v42 = qword_262E8;
      if ( qword_262E8 )
      {
        while ( 1 )
        {
          v21 = *(const char **)v42;
          a2 = (unsigned long long)d_name;
          if ( !fnmatch(*(const char **)v42, d_name, 4) )
            break;
          v42 = *(long long *)(v42 + 8);
          if ( !v42 )
            goto LABEL_58;
        }
      }
      else
      {
LABEL_58:
        a2 = 0;
        if ( (unsigned char)(v23->d_type - 1) <= 0xDu )
          a2 = dword_1A600[(unsigned char)(v23->d_type - 1)];
        v21 = d_name;
        ptra += sub_96F0(d_name);
        if ( dword_26338 == 1 && dword_26330 == 6 && !byte_2632C && !byte_262F6 )
        {
          sub_8E30();
          sub_D420((long long)d_name, a2, v43, v44);
          sub_8730();
        }
      }
      goto LABEL_28;
    }
    if ( dword_262F0 )
      goto LABEL_53;
    v25 = qword_262E0;
    if ( !qword_262E0 )
      goto LABEL_53;
    while ( 1 )
    {
      v21 = *(const char **)v25;
      a2 = (unsigned long long)d_name;
      if ( !fnmatch(*(const char **)v25, d_name, 4) )
        break;
      v25 = *(long long *)(v25 + 8);
      if ( !v25 )
        goto LABEL_53;
    }
LABEL_28:
    sub_84F0((long long)v21, (sigset_t *)a2);
  }
  if ( *v7 )
  {
    v21 = (const char *)v49;
    a2 = (unsigned long long)dcgettext(0, "reading directory %s", 5);
    sub_7540(v49, (const char *)a2, (long long)src);
    if ( *v7 == 75 )
      goto LABEL_28;
  }
  v31 = (long long)v9;
  if ( closedir(v9) )
  {
    v31 = a3;
    a2 = (unsigned long long)dcgettext(0, "closing directory %s", 5);
    sub_7540(a3, (const char *)a2, (long long)src);
  }
  sub_8E30();
  if ( byte_262F6 )
  {
    a2 = 0;
    v31 = (long long)src;
    sub_9510(src);
  }
  if ( !dword_26338 || byte_2632C )
  {
    v34 = (const char *)sub_11550(ptra, v54, (unsigned int)dword_26328, 512, qword_26320);
    v35 = strlen(v34);
    v36 = byte_25019;
    *((char *)v34 - 1) = 32;
    v37 = v34 - 1;
    v38 = (char *)&v34[v35];
    v27 = byte_26318 == 0;
    *v38 = v36;
    v39 = v38 + 1;
    if ( !v27 )
    {
      v45 = strlen("  ");
      sub_7430("  ", v45);
    }
    v40 = dcgettext(0, "total", 5);
    v41 = strlen(v40);
    sub_7430(v40, v41);
    a2 = v39 - v37;
    v31 = (long long)v37;
    sub_7430(v37, v39 - v37);
  }
  if ( qword_263B0 )
    sub_D420(v31, a2, v32, v33);
}



// Function: usage @ 0xdf00
void usage(int status)
{
  long long v2; // r13
  char *v3; // rax
  char *v4; // rax
  FILE *v5; // rbp
  char *v6; // rax
  FILE *v7; // rbp
  char *v8; // rax
  FILE *v9; // rbp
  char *v10; // rax
  FILE *v11; // rbp
  char *v12; // rax
  FILE *v13; // rbp
  char *v14; // rax
  FILE *v15; // rbp
  char *v16; // rax
  FILE *v17; // rbp
  char *v18; // rax
  FILE *v19; // rbp
  char *v20; // rax
  FILE *v21; // rbp
  char *v22; // rax
  FILE *v23; // rbp
  char *v24; // rax
  FILE *v25; // rbp
  char *v26; // rax
  FILE *v27; // rbp
  char *v28; // rax
  FILE *v29; // rbp
  char *v30; // rax
  FILE *v31; // rbp
  char *v32; // rax
  FILE *v33; // rbp
  char *v34; // rax
  FILE *v35; // rbp
  char *v36; // rax
  FILE *v37; // rbp
  char *v38; // rax
  FILE *v39; // rbp
  char *v40; // rax
  FILE *v41; // rbp
  char *v42; // rax
  FILE *v43; // rbp
  char *v44; // rax
  FILE *v45; // rbp
  char *v46; // rax
  FILE *v47; // rbp
  char *v48; // rax
  FILE *v49; // rbp
  char *v50; // rax
  FILE *v51; // rbp
  char *v52; // rax
  FILE *v53; // rbp
  char *v54; // rax
  FILE *v55; // rbp
  char *v56; // rax
  FILE *v57; // rbp
  char *v58; // rax
  FILE *v59; // rbp
  char *v60; // rax
  FILE *v61; // rbp
  char *v62; // rax
  FILE *v63; // rbp
  char *v64; // rax
  FILE *v65; // rbp
  char *v66; // rax
  FILE *v67; // rbp
  char *v68; // rax
  FILE *v69; // rbp
  char *v70; // rax
  FILE *v71; // rbp
  char *v72; // rax
  FILE *v73; // rbp
  char *v74; // rax
  FILE *v75; // rbp
  char *v76; // rax
  FILE *v77; // rbp
  char *v78; // rax
  FILE *v79; // rbp
  char *v80; // rax
  FILE *v81; // rbp
  char *v82; // rax
  FILE *v83; // rbp
  char *v84; // rax
  FILE *v85; // rbp
  char *v86; // rax
  FILE *v87; // rbp
  char *v88; // rax
  FILE *v89; // rbp
  char *v90; // rax
  FILE *v91; // rbp
  char *v92; // rax
  FILE *v93; // rbp
  char *v94; // rax
  FILE *v95; // rbp
  char *v96; // rax
  FILE *v97; // rsi
  const char *v98; // rbp
  char **v99; // rbx
  const char *v100; // rsi
  char *v101; // r14
  char *v102; // rax
  long long v103; // r8
  long long v104; // r9
  char *v105; // rdi
  const char *v106; // r13
  char *v107; // rax
  const char *v108; // rcx
  char *v109; // r13
  char *v110; // rax
  FILE *v111; // rbx
  char *v112; // rax
  char *v113; // [rsp+0h] [rbp-B8h] BYREF
  const char *v114; // [rsp+8h] [rbp-B0h]
  char *v115; // [rsp+10h] [rbp-A8h]
  const char *v116; // [rsp+18h] [rbp-A0h]
  const char *v117; // [rsp+20h] [rbp-98h]
  const char *v118; // [rsp+28h] [rbp-90h]
  const char *v119; // [rsp+30h] [rbp-88h]
  const char *v120; // [rsp+38h] [rbp-80h]
  const char *v121; // [rsp+40h] [rbp-78h]
  const char *v122; // [rsp+48h] [rbp-70h]
  const char *v123; // [rsp+50h] [rbp-68h]
  const char *v124; // [rsp+58h] [rbp-60h]
  long long v125; // [rsp+60h] [rbp-58h]
  long long v126; // [rsp+68h] [rbp-50h]
  unsigned long long v127; // [rsp+78h] [rbp-40h]

  v2 = qword_26400;
  v127 = __readfsqword(0x28u);
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n", 5);
    __printf_chk(1, v4, v2);
    v5 = stdout;
    v6 = dcgettext(
           0,
           "List information about the FILEs (the current directory by default).\n"
           "Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n",
           5);
    fputs_unlocked(v6, v5);
    v7 = stdout;
    v8 = dcgettext(0, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "  -a, --all                  do not ignore entries starting with .\n"
            "  -A, --almost-all           do not list implied . and ..\n"
            "      --author               with -l, print the author of each file\n"
            "  -b, --escape               print C-style escapes for nongraphic characters\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(
            0,
            "      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n"
            "                             e.g., '--block-size=M'; see SIZE format below\n"
            "\n",
            5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(0, "  -B, --ignore-backups       do not list implied entries ending with ~\n", 5);
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = dcgettext(
            0,
            "  -c                         with -lt: sort by, and show, ctime (time of last\n"
            "                             modification of file status information);\n"
            "                             with -l: show ctime and sort by name;\n"
            "                             otherwise: sort by ctime, newest first\n"
            "\n",
            5);
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = dcgettext(
            0,
            "  -C                         list entries by columns\n"
            "      --color[=WHEN]         color the output WHEN; more info below\n"
            "  -d, --directory            list directories themselves, not their contents\n"
            "  -D, --dired                generate output designed for Emacs' dired mode\n",
            5);
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = dcgettext(
            0,
            "  -f                         list all entries in directory order\n"
            "  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n"
            "      --file-type            likewise, except do not append '*'\n",
            5);
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = dcgettext(
            0,
            "      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n"
            "                             single-column -1, verbose -l, vertical -C\n"
            "\n",
            5);
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = dcgettext(0, "      --full-time            like -l --time-style=full-iso\n", 5);
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = dcgettext(0, "  -g                         like -l, but do not list owner\n", 5);
    fputs_unlocked(v26, v25);
    v27 = stdout;
    v28 = dcgettext(
            0,
            "      --group-directories-first\n"
            "                             group directories before files;\n"
            "                             can be augmented with a --sort option, but any\n"
            "                             use of --sort=none (-U) disables grouping\n"
            "\n",
            5);
    fputs_unlocked(v28, v27);
    v29 = stdout;
    v30 = dcgettext(0, "  -G, --no-group             in a long listing, don't print group names\n", 5);
    fputs_unlocked(v30, v29);
    v31 = stdout;
    v32 = dcgettext(
            0,
            "  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n"
            "      --si                   likewise, but use powers of 1000 not 1024\n",
            5);
    fputs_unlocked(v32, v31);
    v33 = stdout;
    v34 = dcgettext(
            0,
            "  -H, --dereference-command-line\n"
            "                             follow symbolic links listed on the command line\n",
            5);
    fputs_unlocked(v34, v33);
    v35 = stdout;
    v36 = dcgettext(
            0,
            "      --dereference-command-line-symlink-to-dir\n"
            "                             follow each command line symbolic link\n"
            "                             that points to a directory\n"
            "\n",
            5);
    fputs_unlocked(v36, v35);
    v37 = stdout;
    v38 = dcgettext(
            0,
            "      --hide=PATTERN         do not list implied entries matching shell PATTERN\n"
            "                             (overridden by -a or -A)\n"
            "\n",
            5);
    fputs_unlocked(v38, v37);
    v39 = stdout;
    v40 = dcgettext(0, "      --hyperlink[=WHEN]     hyperlink file names WHEN\n", 5);
    fputs_unlocked(v40, v39);
    v41 = stdout;
    v42 = dcgettext(
            0,
            "      --indicator-style=WORD\n"
            "                             append indicator with style WORD to entry names:\n"
            "                             none (default), slash (-p),\n"
            "                             file-type (--file-type), classify (-F)\n"
            "\n",
            5);
    fputs_unlocked(v42, v41);
    v43 = stdout;
    v44 = dcgettext(
            0,
            "  -i, --inode                print the index number of each file\n"
            "  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n",
            5);
    fputs_unlocked(v44, v43);
    v45 = stdout;
    v46 = dcgettext(
            0,
            "  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n"
            "                             used only with -s and per directory totals\n"
            "\n",
            5);
    fputs_unlocked(v46, v45);
    v47 = stdout;
    v48 = dcgettext(0, "  -l                         use a long listing format\n", 5);
    fputs_unlocked(v48, v47);
    v49 = stdout;
    v50 = dcgettext(
            0,
            "  -L, --dereference          when showing file information for a symbolic\n"
            "                             link, show information for the file the link\n"
            "                             references rather than for the link itself\n"
            "\n",
            5);
    fputs_unlocked(v50, v49);
    v51 = stdout;
    v52 = dcgettext(0, "  -m                         fill width with a comma separated list of entries\n", 5);
    fputs_unlocked(v52, v51);
    v53 = stdout;
    v54 = dcgettext(
            0,
            "  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n"
            "  -N, --literal              print entry names without quoting\n"
            "  -o                         like -l, but do not list group information\n"
            "  -p, --indicator-style=slash\n"
            "                             append / indicator to directories\n",
            5);
    fputs_unlocked(v54, v53);
    v55 = stdout;
    v56 = dcgettext(0, "  -q, --hide-control-chars   print ? instead of nongraphic characters\n", 5);
    fputs_unlocked(v56, v55);
    v57 = stdout;
    v58 = dcgettext(
            0,
            "      --show-control-chars   show nongraphic characters as-is (the default,\n"
            "                             unless program is 'ls' and output is a terminal)\n"
            "\n",
            5);
    fputs_unlocked(v58, v57);
    v59 = stdout;
    v60 = dcgettext(0, "  -Q, --quote-name           enclose entry names in double quotes\n", 5);
    fputs_unlocked(v60, v59);
    v61 = stdout;
    v62 = dcgettext(
            0,
            "      --quoting-style=WORD   use quoting style WORD for entry names:\n"
            "                             literal, locale, shell, shell-always,\n"
            "                             shell-escape, shell-escape-always, c, escape\n"
            "                             (overrides QUOTING_STYLE environment variable)\n"
            "\n",
            5);
    fputs_unlocked(v62, v61);
    v63 = stdout;
    v64 = dcgettext(
            0,
            "  -r, --reverse              reverse order while sorting\n"
            "  -R, --recursive            list subdirectories recursively\n"
            "  -s, --size                 print the allocated size of each file, in blocks\n",
            5);
    fputs_unlocked(v64, v63);
    v65 = stdout;
    v66 = dcgettext(0, "  -S                         sort by file size, largest first\n", 5);
    fputs_unlocked(v66, v65);
    v67 = stdout;
    v68 = dcgettext(
            0,
            "      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n"
            "                             time (-t), version (-v), extension (-X), width\n"
            "\n",
            5);
    fputs_unlocked(v68, v67);
    v69 = stdout;
    v70 = dcgettext(
            0,
            "      --time=WORD            change the default of using modification times;\n"
            "                               access time (-u): atime, access, use;\n"
            "                               change time (-c): ctime, status;\n"
            "                               birth time: birth, creation;\n"
            "                             with -l, WORD determines which time to show;\n"
            "                             with --sort=time, sort by WORD (newest first)\n"
            "\n",
            5);
    fputs_unlocked(v70, v69);
    v71 = stdout;
    v72 = dcgettext(
            0,
            "      --time-style=TIME_STYLE\n"
            "                             time/date format with -l; see TIME_STYLE below\n",
            5);
    fputs_unlocked(v72, v71);
    v73 = stdout;
    v74 = dcgettext(
            0,
            "  -t                         sort by time, newest first; see --time\n"
            "  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n",
            5);
    fputs_unlocked(v74, v73);
    v75 = stdout;
    v76 = dcgettext(
            0,
            "  -u                         with -lt: sort by, and show, access time;\n"
            "                             with -l: show access time and sort by name;\n"
            "                             otherwise: sort by access time, newest first\n"
            "\n",
            5);
    fputs_unlocked(v76, v75);
    v77 = stdout;
    v78 = dcgettext(0, "  -U                         do not sort; list entries in directory order\n", 5);
    fputs_unlocked(v78, v77);
    v79 = stdout;
    v80 = dcgettext(0, "  -v                         natural sort of (version) numbers within text\n", 5);
    fputs_unlocked(v80, v79);
    v81 = stdout;
    v82 = dcgettext(
            0,
            "  -w, --width=COLS           set output width to COLS.  0 means no limit\n"
            "  -x                         list entries by lines instead of by columns\n"
            "  -X                         sort alphabetically by entry extension\n"
            "  -Z, --context              print any security context of each file\n"
            "      --zero                 end each output line with NUL, not newline\n"
            "  -1                         list one file per line\n",
            5);
    fputs_unlocked(v82, v81);
    v83 = stdout;
    v84 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v84, v83);
    v85 = stdout;
    v86 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v86, v85);
    v87 = stdout;
    v88 = dcgettext(
            0,
            "\n"
            "The SIZE argument is an integer and optional unit (example: 10K is 10*1024).\n"
            "Units are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\n"
            "Binary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",
            5);
    fputs_unlocked(v88, v87);
    v89 = stdout;
    v90 = dcgettext(
            0,
            "\n"
            "The TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\n"
            "FORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\n"
            "then FORMAT1 applies to non-recent files and FORMAT2 to recent files.\n"
            "TIME_STYLE prefixed with 'posix-' takes effect only outside the POSIX locale.\n"
            "Also the TIME_STYLE environment variable sets the default style to use.\n",
            5);
    fputs_unlocked(v90, v89);
    v91 = stdout;
    v92 = dcgettext(0, "\nThe WHEN argument defaults to 'always' and can also be 'auto' or 'never'.\n", 5);
    fputs_unlocked(v92, v91);
    v93 = stdout;
    v94 = dcgettext(
            0,
            "\n"
            "Using color to distinguish file types is disabled both by default and\n"
            "with --color=never.  With --color=auto, ls emits color codes only when\n"
            "standard output is connected to a terminal.  The LS_COLORS environment\n"
            "variable can change the settings.  Use the dircolors(1) command to set it.\n",
            5);
    fputs_unlocked(v94, v93);
    v95 = stdout;
    v96 = dcgettext(
            0,
            "\n"
            "Exit status:\n"
            " 0  if OK,\n"
            " 1  if minor problems (e.g., cannot access subdirectory),\n"
            " 2  if serious trouble (e.g., cannot access command-line argument).\n",
            5);
    v97 = v95;
    v98 = "ls";
    fputs_unlocked(v96, v97);
    if ( dword_251E0 != 1 )
    {
      v98 = "dir";
      if ( dword_251E0 != 2 )
        v98 = "vdir";
    }
    v99 = &v113;
    v125 = 0;
    v114 = "test invocation";
    v115 = "coreutils";
    v100 = "[";
    v116 = "Multi-call invocation";
    v119 = "sha256sum";
    v117 = "sha224sum";
    v121 = "sha384sum";
    v113 = "[";
    v118 = "sha2 utilities";
    v120 = "sha2 utilities";
    v122 = "sha2 utilities";
    v123 = "sha512sum";
    v124 = "sha2 utilities";
    v126 = 0;
    do
    {
      if ( !strcmp(v98, v100) )
        break;
      v100 = v99[2];
      v99 += 2;
    }
    while ( v100 );
    v101 = v99[1];
    if ( !v101 )
      v101 = (char *)v98;
    v102 = dcgettext(0, "\n%s online help: <%s>\n", 5);
    __printf_chk(
      1,
      v102,
      "GNU coreutils",
      "https://www.gnu.org/software/coreutils/",
      v103,
      v104,
      v113,
      v114,
      v115,
      v116,
      v117,
      v118,
      v119,
      v120,
      v121,
      v122,
      v123,
      v124,
      v125,
      v126);
    v105 = setlocale(5, 0);
    if ( v105 && strncmp(v105, "en_", 3u) )
    {
      v111 = stdout;
      v112 = dcgettext(0, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
      fputs_unlocked(v112, v111);
    }
    v106 = "test";
    if ( strcmp(v98, "[") )
      v106 = v98;
    v107 = dcgettext(0, "Full documentation <%s%s>\n", 5);
    v108 = v106;
    v109 = " invocation";
    __printf_chk(1, v107, "https://www.gnu.org/software/coreutils/", v108);
    if ( v98 != v101 )
      v109 = (char *)"";
    v110 = dcgettext(0, "or available locally via: info '(coreutils) %s%s'\n", 5);
    __printf_chk(1, v110, v101, v109);
  }
  exit(status);
}



// Function: raw_hasher @ 0x102f0
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return __ROR8__(a1, 3) % a2;
}



// Function: raw_comparator @ 0x10310
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0x10320
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_1E310 )
    return 1;
  v2 = *(float *)(v1 + 8);
  if ( v2 <= 0.1
    || v2 >= 0.89999998
    || *(float *)(v1 + 12) <= 1.1
    || *(float *)v1 < 0.0
    || (v3 = *(float *)v1 + 0.1, v4 = *(float *)(v1 + 4), v4 <= v3)
    || v4 > 1.0
    || (result = 1, v2 <= v3) )
  {
    *(long long *)(a1 + 40) = &unk_1E310;
    return 0;
  }
  return result;
}



// Function: hash_find_entry @ 0x103b0
long long hash_find_entry(long long a1, long long a2, __m128i **a3, char a4)
{
  unsigned long long v6; // rax
  __m128i *v7; // rbx
  long long result; // rax
  const __m128i *v9; // rdx
  long long *v10; // rax
  long long *v11; // rdx

  v6 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v6 >= *(long long *)(a1 + 16) )
    abort();
  v7 = (__m128i *)(*(long long *)a1 + 16 * v6);
  *a3 = v7;
  if ( !v7->m128i_i64[0] )
    return 0;
  if ( v7->m128i_i64[0] != a2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(a2) )
    {
      result = v7->m128i_i64[0];
      goto LABEL_6;
    }
    while ( 1 )
    {
      v10 = (long long *)v7->m128i_i64[1];
      if ( !v10 )
        break;
      if ( *v10 == a2 || (*(unsigned char (**)(long long))(a1 + 56))(a2) )
      {
        v11 = (long long *)v7->m128i_i64[1];
        result = *v11;
        if ( a4 )
        {
          v7->m128i_i64[1] = v11[1];
          *v11 = 0;
          v11[1] = *(long long *)(a1 + 72);
          *(long long *)(a1 + 72) = v11;
        }
        return result;
      }
      v7 = (__m128i *)v7->m128i_i64[1];
    }
    return 0;
  }
  result = v7->m128i_i64[0];
LABEL_6:
  if ( a4 )
  {
    v9 = (const __m128i *)v7->m128i_i64[1];
    if ( v9 )
    {
      *v7 = _mm_loadu_si128(v9);
      v9->m128i_i64[0] = 0;
      v9->m128i_i64[1] = *(long long *)(a1 + 72);
      *(long long *)(a1 + 72) = v9;
    }
    else
    {
      v7->m128i_i64[0] = 0;
    }
  }
  return result;
}



// Function: transfer_entries @ 0x10600
long long transfer_entries(long long a1, long long a2, char a3)
{
  long long *v5; // rbx
  long long v6; // r15
  long long *v7; // r13
  unsigned long long v8; // rsi
  long long v9; // r15
  unsigned long long v10; // rax
  long long *v11; // rax
  long long *v12; // rdx
  long long v13; // rax
  unsigned long long v14; // rax
  long long *v15; // r13
  long long *v16; // rax
  long long v17; // rdx

  v5 = *(long long **)a2;
  if ( *(long long *)a2 >= *(long long *)(a2 + 8) )
    return 1;
  while ( 2 )
  {
    while ( 1 )
    {
      v6 = *v5;
      if ( *v5 )
        break;
LABEL_3:
      v5 += 2;
      if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
        return 1;
    }
    v7 = (long long *)v5[1];
    if ( !v7 )
      goto LABEL_13;
    v8 = *(long long *)(a1 + 16);
    while ( 1 )
    {
      v9 = *v7;
      v10 = (*(long long (**)(long long, unsigned long long))(a1 + 48))(*v7, v8);
      v8 = *(long long *)(a1 + 16);
      if ( v10 >= v8 )
        goto LABEL_24;
      v11 = (long long *)(*(long long *)a1 + 16 * v10);
      v12 = (long long *)v7[1];
      if ( !*v11 )
        break;
      v7[1] = v11[1];
      v11[1] = v7;
      if ( !v12 )
        goto LABEL_12;
LABEL_8:
      v7 = v12;
    }
    *v11 = v9;
    v13 = *(long long *)(a1 + 72);
    ++*(long long *)(a1 + 24);
    *v7 = 0;
    v7[1] = v13;
    *(long long *)(a1 + 72) = v7;
    if ( v12 )
      goto LABEL_8;
LABEL_12:
    v6 = *v5;
LABEL_13:
    v5[1] = 0;
    if ( a3 )
      goto LABEL_3;
    v14 = (*(long long (**)(long long, long long))(a1 + 48))(v6, *(long long *)(a1 + 16));
    if ( v14 >= *(long long *)(a1 + 16) )
LABEL_24:
      abort();
    v15 = (long long *)(*(long long *)a1 + 16 * v14);
    if ( !*v15 )
    {
      *v15 = v6;
      ++*(long long *)(a1 + 24);
LABEL_19:
      *v5 = 0;
      v5 += 2;
      --*(long long *)(a2 + 24);
      if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
        return 1;
      continue;
    }
    break;
  }
  v16 = *(long long **)(a1 + 72);
  if ( v16 )
  {
    *(long long *)(a1 + 72) = v16[1];
    goto LABEL_18;
  }
  v16 = malloc(0x10u);
  if ( v16 )
  {
LABEL_18:
    v17 = v15[1];
    *v16 = v6;
    v16[1] = v17;
    v15[1] = v16;
    goto LABEL_19;
  }
  return 0;
}



// Function: hash_print_statistics @ 0x10870
long long hash_print_statistics(long long a1, long long a2)
{
  unsigned long long v2; // r12
  long long *v4; // rcx
  unsigned long long v5; // rsi
  long long v6; // rbx
  long long v7; // r13
  long long v8; // rax
  unsigned long long i; // rdx
  double v10; // xmm0_8
  double v11; // xmm1_8

  v2 = 0;
  v4 = *(long long **)a1;
  v5 = *(long long *)(a1 + 8);
  v6 = *(long long *)(a1 + 16);
  v7 = *(long long *)(a1 + 24);
  if ( *(long long *)a1 < v5 )
  {
    do
    {
      while ( !*v4 )
      {
        v4 += 2;
        if ( (unsigned long long)v4 >= v5 )
          goto LABEL_10;
      }
      v8 = v4[1];
      for ( i = 1; v8; ++i )
        v8 = *(long long *)(v8 + 8);
      if ( v2 < i )
        v2 = i;
      v4 += 2;
    }
    while ( (unsigned long long)v4 < v5 );
  }
LABEL_10:
  __fprintf_chk(a2, 1, "# entries:         %lu\n", *(long long *)(a1 + 32));
  __fprintf_chk(a2, 1, "# buckets:         %lu\n", v6);
  if ( v7 < 0 )
  {
    v10 = ((double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1)) + (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1)))
        * 100.0;
    if ( v6 >= 0 )
      goto LABEL_12;
LABEL_15:
    v11 = (double)(int)(v6 & 1 | ((unsigned long long)v6 >> 1)) + (double)(int)(v6 & 1 | ((unsigned long long)v6 >> 1));
    goto LABEL_13;
  }
  v10 = (double)(int)v7 * 100.0;
  if ( v6 < 0 )
    goto LABEL_15;
LABEL_12:
  v11 = (double)(int)v6;
LABEL_13:
  __fprintf_chk(a2, 1, "# buckets used:    %lu (%.2f%%)\n", v7, v10 / v11);
  return __fprintf_chk(a2, 1, "max bucket length: %lu\n", v2);
}



// Function: hash_lookup @ 0x109d0
long long hash_lookup(long long a1, long long a2)
{
  long long v2; // r12
  unsigned long long v3; // rax
  long long *v4; // rbx
  long long v5; // rsi

  v2 = a2;
  v3 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v3 >= *(long long *)(a1 + 16) )
    abort();
  v4 = (long long *)(*(long long *)a1 + 16 * v3);
  v5 = *v4;
  if ( !*v4 )
    return 0;
  while ( v5 != v2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(v2) )
      return *v4;
    v4 = (long long *)v4[1];
    if ( !v4 )
      return 0;
    v5 = *v4;
  }
  return v2;
}



// Function: hash_get_next @ 0x10a90
long long hash_get_next(long long a1, long long a2)
{
  unsigned long long v2; // rax
  long long *v3; // rax
  long long *v4; // rdx
  long long v5; // rcx
  long long v6; // r8

  v2 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v2 >= *(long long *)(a1 + 16) )
    abort();
  v3 = (long long *)(*(long long *)a1 + 16 * v2);
  v4 = v3;
  while ( 1 )
  {
    v5 = *v4;
    v4 = (long long *)v4[1];
    if ( v5 == a2 )
      break;
    if ( !v4 )
      goto LABEL_8;
  }
  if ( v4 )
    return *v4;
LABEL_8:
  while ( 1 )
  {
    v3 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v3 )
      break;
    v6 = *v3;
    if ( *v3 )
      return v6;
  }
  return 0;
}



// Function: hash_do_for_each @ 0x10b70
long long hash_do_for_each(long long a1, unsigned char (*a2)(long long, long long), long long a3)
{
  long long *v3; // r14
  long long v6; // r12
  long long v7; // rdi
  long long *v9; // rbx

  v3 = *(long long **)a1;
  if ( *(long long *)a1 >= *(long long *)(a1 + 8) )
    return 0;
  v6 = 0;
  while ( 1 )
  {
    v7 = *v3;
    if ( *v3 )
      break;
LABEL_4:
    v3 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v3 )
      return v6;
  }
  v9 = v3;
  while ( a2(v7, a3) )
  {
    v9 = (long long *)v9[1];
    ++v6;
    if ( !v9 )
      goto LABEL_4;
    v7 = *v9;
  }
  return v6;
}



// Function: hash_initialize @ 0x10c60
long long *hash_initialize(
        long long a1,
        float *a2,
        unsigned long long (*a3)(long long a1, unsigned long long a2),
        bool (*a4)(long long a1, long long a2),
        long long a5)
{
  float *v5; // r15
  unsigned long long (*v7)(long long, unsigned long long); // rbp
  bool (*v8)(long long, long long); // rbx
  long long *v9; // r12
  size_t v10; // rax
  char *v11; // rax
  long long v12; // rdx
  void *v14; // rdi

  v5 = a2;
  v7 = a3;
  v8 = a4;
  if ( !a3 )
    v7 = sub_102F0;
  if ( !a4 )
    v8 = sub_10310;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_1E310;
    v9[5] = v5;
    if ( (unsigned char)sub_10320((long long)v9)
      && (v10 = sub_104C0(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
      && (v11 = (char *)calloc(v10, 0x10u), (*v9 = v11) != 0) )
    {
      v12 = v9[2];
      v9[6] = v7;
      v9[3] = 0;
      v9[7] = v8;
      v9[8] = a5;
      v9[1] = &v11[16 * v12];
      v9[4] = 0;
      v9[9] = 0;
    }
    else
    {
      v14 = v9;
      v9 = 0;
      free(v14);
    }
  }
  return v9;
}



// Function: hash_clear @ 0x10d70
void hash_clear(long long *a1)
{
  long long *v1; // r12
  long long *v2; // rbx
  void (*v3)(long long); // rdx
  long long *v4; // rax
  long long v5; // rcx

  v1 = (long long *)*a1;
  if ( *a1 < a1[1] )
  {
    do
    {
      while ( !*v1 )
      {
        v1 += 2;
        if ( a1[1] <= (unsigned long long)v1 )
          goto LABEL_14;
      }
      v2 = (long long *)v1[1];
      v3 = (void (*)(long long))a1[8];
      if ( v2 )
      {
        while ( 1 )
        {
          if ( v3 )
          {
            v3(*v2);
            v3 = (void (*)(long long))a1[8];
          }
          v4 = (long long *)v2[1];
          v5 = a1[9];
          *v2 = 0;
          v2[1] = v5;
          a1[9] = v2;
          if ( !v4 )
            break;
          v2 = v4;
        }
      }
      if ( v3 )
        v3(*v1);
      *v1 = 0;
      v1 += 2;
      *(v1 - 1) = 0;
    }
    while ( a1[1] > (unsigned long long)v1 );
  }
LABEL_14:
  a1[3] = 0;
  a1[4] = 0;
}



// Function: hash_free @ 0x10e20
void hash_free(long long *ptr)
{
  long long *v2; // r13
  unsigned long long v3; // rax
  long long *v4; // rbp
  long long *v5; // rbx
  long long *v6; // rbx
  void *v7; // rdi
  long long *v8; // rbx
  void *v9; // rdi

  v2 = (long long *)*ptr;
  v3 = ptr[1];
  v4 = (long long *)*ptr;
  if ( ptr[8] && ptr[4] )
  {
    if ( (unsigned long long)v2 >= v3 )
      goto LABEL_15;
    do
    {
      while ( !*v2 )
      {
        v2 += 2;
        if ( v3 <= (unsigned long long)v2 )
          goto LABEL_10;
      }
      v5 = v2;
      do
      {
        ((void (*)(void))ptr[8])();
        v5 = (long long *)v5[1];
      }
      while ( v5 );
      v3 = ptr[1];
      v2 += 2;
    }
    while ( v3 > (unsigned long long)v2 );
LABEL_10:
    v4 = (long long *)*ptr;
  }
  if ( v3 > (unsigned long long)v4 )
  {
    do
    {
      v6 = (long long *)v4[1];
      while ( v6 )
      {
        v7 = v6;
        v6 = (long long *)v6[1];
        free(v7);
      }
      v4 += 2;
    }
    while ( ptr[1] > (unsigned long long)v4 );
  }
LABEL_15:
  v8 = (long long *)ptr[9];
  while ( v8 )
  {
    v9 = v8;
    v8 = (long long *)v8[1];
    free(v9);
  }
  free((void *)*ptr);
  free(ptr);
}



// Function: hash_rehash @ 0x10f10
long long hash_rehash(long long a1, long long a2)
{
  size_t v2; // rax
  size_t v3; // rbx
  char *v4; // rax
  long long v5; // rax
  unsigned int v6; // r12d
  void *ptr; // [rsp+0h] [rbp-88h] BYREF
  char *v9; // [rsp+8h] [rbp-80h]
  size_t v10; // [rsp+10h] [rbp-78h]
  long long v11; // [rsp+18h] [rbp-70h]
  long long v12; // [rsp+20h] [rbp-68h]
  long long v13; // [rsp+28h] [rbp-60h]
  long long v14; // [rsp+30h] [rbp-58h]
  long long v15; // [rsp+38h] [rbp-50h]
  long long v16; // [rsp+40h] [rbp-48h]
  long long v17; // [rsp+48h] [rbp-40h]
  unsigned long long v18; // [rsp+58h] [rbp-30h]

  v18 = __readfsqword(0x28u);
  v2 = sub_104C0(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
  if ( !v2 )
    return 0;
  v3 = v2;
  if ( *(long long *)(a1 + 16) == v2 )
    return 1;
  v4 = (char *)calloc(v2, 0x10u);
  ptr = v4;
  if ( !v4 )
    return 0;
  v10 = v3;
  v11 = 0;
  v9 = &v4[16 * v3];
  v5 = *(long long *)(a1 + 40);
  v12 = 0;
  v13 = v5;
  v14 = *(long long *)(a1 + 48);
  v15 = *(long long *)(a1 + 56);
  v16 = *(long long *)(a1 + 64);
  v17 = *(long long *)(a1 + 72);
  v6 = sub_10600((long long)&ptr, a1, 0);
  if ( (char)v6 )
  {
    free(*(void **)a1);
    *(long long *)a1 = ptr;
    *(long long *)(a1 + 8) = v9;
    *(long long *)(a1 + 16) = v10;
    *(long long *)(a1 + 24) = v11;
    *(long long *)(a1 + 72) = v17;
  }
  else
  {
    *(long long *)(a1 + 72) = v17;
    if ( !(unsigned char)sub_10600(a1, (long long)&ptr, 1) || !(unsigned char)sub_10600(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0x11090
long long hash_insert_if_absent(long long *a1, long long a2, long long *a3)
{
  long long v5; // rax
  unsigned int v6; // r8d
  long long v8; // rax
  float v9; // xmm5_4
  long long v10; // rax
  float v11; // xmm4_4
  __m128i *v12; // rdx
  long long *v13; // rax
  long long v14; // rcx
  long long v15; // rdx
  float v16; // xmm4_4
  long long v17; // rax
  float v18; // xmm0_4
  float v19; // xmm5_4
  float v20; // xmm4_4
  long long v21; // rsi
  __m128i *v22[7]; // [rsp+0h] [rbp-38h] BYREF

  v22[1] = (__m128i *)__readfsqword(0x28u);
  if ( !a2 )
LABEL_28:
    abort();
  v5 = sub_103B0((long long)a1, a2, v22, 0);
  if ( !v5 )
  {
    v8 = a1[3];
    if ( v8 < 0 )
    {
      v15 = a1[3] & 1LL | (a1[3] >> 1);
      v10 = a1[2];
      v9 = (float)(int)v15 + (float)(int)v15;
      if ( v10 >= 0 )
        goto LABEL_8;
    }
    else
    {
      v9 = (float)(int)v8;
      v10 = a1[2];
      if ( v10 >= 0 )
      {
LABEL_8:
        v11 = (float)(int)v10;
LABEL_9:
        if ( v9 > (float)(*(float *)(a1[5] + 8LL) * v11) )
        {
          sub_10320((long long)a1);
          v17 = a1[5];
          v18 = *(float *)(v17 + 8);
          if ( v19 > (float)(v16 * v18) )
          {
            v20 = v16 * *(float *)(v17 + 12);
            if ( !*(char *)(v17 + 16) )
              v20 = v20 * v18;
            if ( v20 >= 1.8446744e19 )
              return (unsigned int)-1;
            v21 = v20 >= 9.223372e18
                ? (unsigned int)(int)(float)(v20 - 9.223372e18) ^ 0x8000000000000000LL
                : (unsigned int)(int)v20;
            if ( !(unsigned char)sub_10F10((long long)a1, v21) )
              return (unsigned int)-1;
            if ( sub_103B0((long long)a1, a2, v22, 0) )
              goto LABEL_28;
          }
        }
        v12 = v22[0];
        if ( !*(long long *)v22[0] )
        {
          *(long long *)v22[0] = a2;
          v6 = 1;
          ++a1[4];
          ++a1[3];
          return v6;
        }
        v13 = (long long *)a1[9];
        if ( v13 )
        {
          a1[9] = v13[1];
LABEL_13:
          v14 = v12->m128i_i64[1];
          *v13 = a2;
          v6 = 1;
          v13[1] = v14;
          v12->m128i_i64[1] = (long long)v13;
          ++a1[4];
          return v6;
        }
        v13 = malloc(0x10u);
        if ( v13 )
        {
          v12 = v22[0];
          goto LABEL_13;
        }
        return (unsigned int)-1;
      }
    }
    v11 = (float)(v10 & 1 | (unsigned int)((unsigned long long)v10 >> 1))
        + (float)(v10 & 1 | (unsigned int)((unsigned long long)v10 >> 1));
    goto LABEL_9;
  }
  v6 = 0;
  if ( a3 )
    *a3 = v5;
  return v6;
}



// Function: hash_insert @ 0x112c0
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_11090(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0x11320
long long hash_remove(long long *a1, long long a2)
{
  long long v3; // r12
  __m128i *v4; // rax
  long long v6; // rax
  float v7; // xmm5_4
  long long v8; // rax
  float v9; // xmm4_4
  float v10; // xmm4_4
  long long v11; // rax
  float v12; // xmm5_4
  float v13; // xmm4_4
  unsigned long long v14; // rsi
  long long *v15; // rbp
  void *v16; // rdi
  unsigned long long v17; // rdx
  __m128i *v18[5]; // [rsp+0h] [rbp-28h] BYREF

  v18[1] = (__m128i *)__readfsqword(0x28u);
  v3 = sub_103B0((long long)a1, a2, v18, 1);
  if ( !v3 )
    return v3;
  v4 = v18[0];
  --a1[4];
  if ( v4->m128i_i64[0] )
    return v3;
  v6 = a1[3] - 1LL;
  a1[3] = v6;
  if ( v6 < 0 )
  {
    v17 = v6 & 1 | ((unsigned long long)v6 >> 1);
    v8 = a1[2];
    v7 = (float)(int)v17 + (float)(int)v17;
    if ( v8 >= 0 )
      goto LABEL_6;
LABEL_18:
    v9 = (float)(v8 & 1 | (unsigned int)((unsigned long long)v8 >> 1))
       + (float)(v8 & 1 | (unsigned int)((unsigned long long)v8 >> 1));
    goto LABEL_7;
  }
  v7 = (float)(int)v6;
  v8 = a1[2];
  if ( v8 < 0 )
    goto LABEL_18;
LABEL_6:
  v9 = (float)(int)v8;
LABEL_7:
  if ( (float)(*(float *)a1[5] * v9) > v7 )
  {
    sub_10320((long long)a1);
    v11 = a1[5];
    if ( (float)(*(float *)v11 * v10) > v12 )
    {
      v13 = v10 * *(float *)(v11 + 4);
      if ( !*(char *)(v11 + 16) )
        v13 = v13 * *(float *)(v11 + 8);
      if ( v13 >= 9.223372e18 )
        v14 = (unsigned int)(int)(float)(v13 - 9.223372e18) ^ 0x8000000000000000LL;
      else
        v14 = (unsigned int)(int)v13;
      if ( !(unsigned char)sub_10F10((long long)a1, v14) )
      {
        v15 = (long long *)a1[9];
        while ( v15 )
        {
          v16 = v15;
          v15 = (long long *)v15[1];
          free(v16);
        }
        a1[9] = 0;
      }
    }
  }
  return v3;
}


