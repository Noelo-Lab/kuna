// Function: main @ 0x3c50
long long main(int a1, char **a2, char **a3)
{
  char *v4; // rdi
  char *v5; // rax
  int v6; // eax
  int v7; // eax
  char *v8; // rax
  unsigned int v9; // eax
  char *v10; // rax
  unsigned int v11; // eax
  int *v12; // rax
  const char *v13; // r13
  int v14; // eax
  FILE *v15; // rdi
  long long v16; // rax
  long long v17; // r13
  bool v18; // al
  void *v19; // r15
  const char *v20; // rax
  const char *v21; // rbx
  int v22; // r14d
  long long v23; // rax
  long long v24; // r12
  long long v25; // rax
  int v26; // ebp
  int v27; // ebx
  char *v28; // rax
  long long v29; // r13
  char *v30; // rax
  int v31; // eax
  long long v32; // r13
  long long v33; // r11
  long long v34; // rdx
  long long v35; // r10
  unsigned long long v36; // r12
  long long v37; // rax
  unsigned long long v38; // rdx
  long long *v39; // rax
  unsigned int v40; // ebp
  long long *v41; // rax
  long long v42; // rdx
  bool v43; // zf
  bool v44; // sf
  bool v45; // of
  int v46; // ecx
  long long v47; // rax
  unsigned long long v48; // rdx
  bool v49; // al
  long long v50; // r12
  char *v51; // rax
  long long v52; // rbx
  char *v53; // rbp
  const char *v54; // rax
  char *v55; // rax
  long long v56; // rax
  long long v57; // r13
  char *v58; // rax
  long long v59; // r14
  char *v60; // r12
  int *v61; // rax
  long long result; // rax
  const __m128i *v63; // rax
  long long v64; // rdx
  long long v65; // rcx
  bool v66; // si
  int v67; // edi
  long long v68; // rsi
  __m128i *v69; // rdx
  long long v70; // rdi
  long long v71; // rcx
  long long v72; // rsi
  long long v73; // rcx
  int *v74; // rbx
  char *v75; // rax
  char *v76; // rax
  char **v77; // rdi
  bool v78; // cf
  long long v79; // rdx
  long long v80; // rsi
  long long v81; // rax
  char *v82; // rax
  int v83; // eax
  char *v84; // rax
  long long v85; // r12
  char *v86; // rax
  long long v87; // r13
  char *v88; // rax
  const char *v89; // rax
  long long v90; // rbx
  long long v91; // r12
  char *v92; // rax
  char *v93; // rax
  long long i; // r13
  long long v95; // rbp
  char *v96; // rax
  long long v97; // r12
  char *v98; // rax
  char *v99; // rax
  long long v100; // r12
  char *v101; // rax
  char *v102; // rax
  long long v103; // r13
  char *v104; // r12
  int *v105; // rax
  unsigned char v106; // [rsp+8h] [rbp-160h]
  const char *v107; // [rsp+8h] [rbp-160h]
  long long v108; // [rsp+8h] [rbp-160h]
  char *s1; // [rsp+10h] [rbp-158h]
  int v110; // [rsp+18h] [rbp-150h]
  long long v111; // [rsp+18h] [rbp-150h]
  int ptr; // [rsp+20h] [rbp-148h]
  long long *ptra; // [rsp+20h] [rbp-148h]
  char v114; // [rsp+28h] [rbp-140h]
  long long v115; // [rsp+30h] [rbp-138h]
  long long v116; // [rsp+30h] [rbp-138h]
  long long v117; // [rsp+30h] [rbp-138h]
  long long v118; // [rsp+38h] [rbp-130h]
  long long v119; // [rsp+38h] [rbp-130h]
  long long v120; // [rsp+38h] [rbp-130h]
  char v121; // [rsp+43h] [rbp-125h]
  unsigned char v122; // [rsp+43h] [rbp-125h]
  unsigned int v123; // [rsp+44h] [rbp-124h]
  int longind; // [rsp+5Ch] [rbp-10Ch] BYREF
  long long v125; // [rsp+60h] [rbp-108h] BYREF
  long long v126; // [rsp+68h] [rbp-100h]
  __m128i v127; // [rsp+70h] [rbp-F8h]
  long long v128[2]; // [rsp+80h] [rbp-E8h] BYREF
  struct stat v129; // [rsp+90h] [rbp-D8h] BYREF
  unsigned long long v130; // [rsp+128h] [rbp-40h]

  v4 = *a2;
  v130 = __readfsqword(0x28u);
  v128[1] = 0;
  v128[0] = ".";
  sub_EB20(v4);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_13BE0(sub_5F80);
  qword_1B140 = sub_6800();
  v5 = getenv("DU_BLOCK_SIZE");
  sub_D520(v5);
  v114 = 0;
  v110 = 16;
  ptr = 8;
  s1 = 0;
  v106 = 1;
  v121 = 0;
  while ( 1 )
  {
    longind = -1;
    v6 = getopt_long(a1, a2, "0abd:chHklmst:xB:DLPSX:", &longopts, &longind);
    if ( v6 == -1 )
      break;
    if ( v6 > 135 )
      goto LABEL_8;
    if ( v6 <= 47 )
    {
      if ( v6 == -131 )
      {
        sub_11AB0(stdout, (char)"Paul Eggert");
        exit(0);
      }
      if ( v6 == -130 )
        sub_5540(0);
LABEL_8:
      v106 = 0;
    }
    else
    {
      switch ( v6 )
      {
        case 48:
          byte_1B182 = 1;
          continue;
        case 66:
          goto LABEL_23;
        case 68:
        case 72:
          v110 = 17;
          continue;
        case 76:
          v110 = 2;
          continue;
        case 80:
          v110 = 16;
          continue;
        case 83:
          byte_1B180 = 1;
          continue;
        case 88:
          if ( (unsigned int)sub_70C0(sub_6B70, qword_1B140, optarg, 0x10000000, 10) )
          {
            v107 = (const char *)sub_10B80(0, 3, optarg);
            v12 = __errno_location();
            error(0, *v12, "%s", v107);
            v106 = 0;
          }
          continue;
        case 97:
          byte_1B186 = 1;
          continue;
        case 98:
          byte_1B185 = 1;
          dword_1B170 = 0;
          qword_1B148 = 1;
          continue;
        case 99:
          byte_1B181 = 1;
          continue;
        case 100:
          if ( (unsigned int)sub_12910(optarg) )
          {
            v108 = sub_10EF0(optarg);
            v28 = dcgettext(0, "invalid maximum depth %s", 5);
            error(0, 0, v28, v108);
            v106 = 0;
          }
          else
          {
            v121 = 1;
            qword_1B010 = v125;
          }
          continue;
        case 104:
          dword_1B170 = 176;
          qword_1B148 = 1;
          continue;
        case 107:
          dword_1B170 = 0;
          qword_1B148 = 1024;
          continue;
        case 108:
          byte_1B184 = 1;
          continue;
        case 109:
          dword_1B170 = 0;
          qword_1B148 = 0x100000;
          continue;
        case 115:
          v114 = 1;
          continue;
        case 116:
          v9 = sub_12250(optarg);
          if ( v9 )
            sub_12850(v9, (unsigned int)longind, 116, &longopts, optarg);
          if ( qword_1B178 || *optarg != 45 )
            continue;
          v10 = dcgettext(0, "invalid --threshold argument '-0'", 5);
          error(1, 0, v10);
LABEL_23:
          v11 = sub_D520(optarg);
          if ( v11 )
            sub_12850(v11, (unsigned int)longind, 66, &longopts, optarg);
          return result;
        case 120:
          ptr = 72;
          continue;
        case 128:
          byte_1B185 = 1;
          continue;
        case 129:
          sub_6B70(qword_1B140, optarg, 0x10000000);
          continue;
        case 130:
          s1 = optarg;
          continue;
        case 131:
          dword_1B170 = 144;
          qword_1B148 = 1;
          continue;
        case 133:
          byte_1B16C = 1;
          v7 = 0;
          if ( optarg )
            v7 = dword_156F0[sub_5CE0("--time", optarg, off_1A6C0, dword_156F0, 4, off_1B020, 1)];
          dword_1B168 = v7;
          v8 = getenv("TZ");
          qword_1B150 = sub_10FE0(v8);
          continue;
        case 134:
          ::s1 = optarg;
          continue;
        case 135:
          byte_1B16D = 1;
          continue;
        default:
          goto LABEL_8;
      }
    }
  }
  if ( !v106 )
    goto LABEL_229;
  if ( byte_1B186 )
  {
    if ( !v114 )
      goto LABEL_45;
    v99 = dcgettext(0, "cannot both summarize and show all entries", 5);
    error(0, 0, v99);
LABEL_229:
    sub_5540(1);
  }
  if ( v121 )
  {
    if ( v114 )
    {
      v29 = qword_1B010;
      if ( qword_1B010
        || (v75 = dcgettext(0, "warning: summarizing is the same as using --max-depth=0", 5),
            error(0, 0, v75),
            (v29 = qword_1B010) != 0) )
      {
        v30 = dcgettext(0, "warning: summarizing conflicts with --max-depth=%lu", 5);
        error(0, 0, v30, v29);
        sub_5540(1);
      }
LABEL_177:
      qword_1B010 = 0;
    }
  }
  else if ( v114 )
  {
    goto LABEL_177;
  }
LABEL_45:
  if ( byte_1B16D )
  {
    if ( byte_1B185 )
    {
      v93 = dcgettext(0, "warning: options --apparent-size and -b are ineffective with --inodes", 5);
      error(0, 0, v93);
    }
    qword_1B148 = 1;
  }
  if ( byte_1B16C )
  {
    if ( ::s1 )
    {
      v13 = ::s1;
      goto LABEL_52;
    }
    v82 = getenv("TIME_STYLE");
    ::s1 = v82;
    v13 = v82;
    if ( !v82 || !strcmp(v82, "locale") )
    {
      v13 = "long-iso";
      ::s1 = "long-iso";
      goto LABEL_188;
    }
    if ( *v13 == 43 )
    {
      v96 = strchr(v13, 10);
      if ( v96 )
      {
        *v96 = 0;
        goto LABEL_52;
      }
      goto LABEL_53;
    }
    while ( 1 )
    {
      v13 = ::s1;
      if ( strncmp(::s1, "posix-", 6u) )
        break;
      ::s1 = (char *)(v13 + 6);
    }
LABEL_52:
    if ( *v13 == 43 )
    {
LABEL_53:
      qword_1B158 = (long long)(v13 + 1);
    }
    else
    {
LABEL_188:
      v83 = dword_156E0[sub_5CE0("time style", v13, off_1A6A0, dword_156E0, 4, off_1B020, 1)];
      switch ( v83 )
      {
        case 1:
          qword_1B158 = (long long)"%Y-%m-%d %H:%M";
          break;
        case 2:
          qword_1B158 = (long long)"%Y-%m-%d";
          break;
        case 0:
          qword_1B158 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
          break;
      }
    }
  }
  if ( s1 )
  {
    if ( a1 > optind )
    {
      v100 = sub_10EF0(a2[optind]);
      v101 = dcgettext(0, "extra operand %s", 5);
      error(0, 0, v101, v100);
      v102 = dcgettext(0, "file operands cannot be combined with --files0-from", 5);
      __fprintf_chk(stderr, 1, "%s\n", v102);
      sub_5540(1);
    }
    v14 = strcmp(s1, "-");
    v15 = stdin;
    if ( v14 )
    {
      v16 = sub_8E20(s1, (char *)"r", stdin);
      v15 = stdin;
      if ( !v16 )
      {
        v103 = sub_10800(4, s1);
        v104 = dcgettext(0, "cannot open %s for reading", 5);
        v105 = __errno_location();
        error(1, *v105, v104, v103);
LABEL_232:
        __assert_fail("e == ent", "src/du.c", 0x20Fu, "process_file");
      }
    }
    v17 = sub_5E40(v15);
    v18 = v106;
  }
  else
  {
    v77 = (char **)v128;
    if ( a1 > optind )
      v77 = &a2[optind];
    v17 = sub_5E10(v77);
    v18 = v110 == 2 || optind + 1 < a1;
  }
  byte_1B183 = v18;
  if ( !v17 )
    goto LABEL_222;
  qword_1B198 = (void *)sub_6130();
  if ( !qword_1B198 )
    goto LABEL_222;
  if ( byte_1B184 || !byte_1B183 )
    ptr |= 0x100u;
  v19 = (void *)v17;
  v123 = v110 | ptr;
  v122 = v106;
LABEL_65:
  while ( 2 )
  {
    v20 = (const char *)sub_5E80(v19, &longind);
    v21 = v20;
    if ( v20 )
    {
      if ( s1 )
      {
        if ( !strcmp(s1, "-") && !strcmp(v21, "-") )
        {
          v50 = sub_10800(4, v21);
          v51 = dcgettext(0, "when reading file names from stdin, no file name of %s allowed", 5);
          error(0, 0, v51, v50);
          if ( !*v21 )
            goto LABEL_134;
        }
        else
        {
          if ( *v21 )
            goto LABEL_70;
LABEL_134:
          v52 = sub_5F20(v19);
          v53 = dcgettext(0, "invalid zero-length file name", 5);
          v54 = (const char *)sub_10B80(0, 3, s1);
          error(0, 0, "%s:%lu: %s", v54, v52, v53);
        }
        v122 = 0;
        continue;
      }
      if ( !*v20 )
      {
        v55 = dcgettext(0, "invalid zero-length file name", 5);
        error(0, 0, "%s", v55);
        v122 = 0;
        continue;
      }
LABEL_70:
      qword_1B110[0] = v21;
      v22 = v106;
      ptra = (long long *)sub_121C0(qword_1B110, v123, 0);
      while ( 1 )
      {
LABEL_71:
        v23 = sub_ADD0(ptra);
        v24 = v23;
        if ( !v23 )
        {
          v74 = __errno_location();
          if ( *v74 )
          {
            LOBYTE(v22) = 0;
            v85 = sub_10B80(0, 3, ptra[4]);
            v86 = dcgettext(0, "fts_read failed: %s", 5);
            error(0, *v74, v86, v85);
          }
          qword_1B188 = 0;
          if ( (unsigned int)sub_ABE0(ptra) )
          {
            LOBYTE(v22) = 0;
            v84 = dcgettext(0, "fts_close failed", 5);
            error(0, *v74, v84);
          }
          v122 &= v22;
          goto LABEL_65;
        }
        v25 = *(long long *)(v23 + 56);
        v26 = *(unsigned short *)(v24 + 104);
        v111 = v25;
        if ( v26 == 4 )
          break;
        v27 = v106;
        if ( v26 == 6 )
          goto LABEL_98;
        if ( (unsigned char)sub_69C0(qword_1B140, v25) )
          goto LABEL_237;
        if ( v26 == 11 )
        {
          v27 = (int)ptra;
          sub_B520(ptra, v24, 1);
          if ( v24 != sub_ADD0(ptra) )
            goto LABEL_232;
          v26 = *(unsigned short *)(v24 + 104);
        }
        LOBYTE(v27) = v26 == 13 || v26 == 10;
        if ( (char)v27 )
        {
          v22 = 0;
          v87 = sub_10800(4, v111);
          v88 = dcgettext(0, "cannot access %s", 5);
          error(0, *(int *)(v24 + 64), v88, v87);
          continue;
        }
        if ( (ptra[9] & 0x40) != 0 && *(long long *)(v24 + 88) > 0 && ptra[3] != *(long long *)(v24 + 112) )
          goto LABEL_237;
        if ( !byte_1B184 && (byte_1B183 || (*(int *)(v24 + 136) & 0xF000) != 0x4000 && *(long long *)(v24 + 128) > 1u) )
        {
          v31 = sub_61D0(qword_1B198, *(long long *)(v24 + 112), *(long long *)(v24 + 120));
          if ( v31 < 0 )
            goto LABEL_222;
          if ( !v31 )
          {
LABEL_237:
            if ( v26 == 1 )
            {
              sub_B520(ptra, v24, 4);
              if ( v24 != sub_ADD0(ptra) )
                __assert_fail("e == ent", "src/du.c", 0x230u, "process_file");
            }
            continue;
          }
        }
        if ( v26 != 2 )
        {
          if ( v26 == 7 )
          {
            v89 = (const char *)sub_10B80(0, 3, v111);
            error(0, *(int *)(v24 + 64), "%s", v89);
          }
          else
          {
            if ( v26 == 1 )
              continue;
            v27 = v106;
          }
          goto LABEL_98;
        }
        if ( (unsigned char)sub_12210(ptra, v24) )
        {
          v90 = *(long long *)v24;
          if ( !qword_1B190 )
          {
            qword_1B190 = (void *)sub_6130();
            if ( !qword_1B190 )
              goto LABEL_222;
            for ( i = sub_DF90(0); i; i = v95 )
            {
              if ( (*(char *)(i + 40) & 3) == 0
                && !stat(*(const char **)(i + 8), &v129)
                && (int)sub_61D0(qword_1B190, v129.st_dev, v129.st_ino) < 0 )
              {
                goto LABEL_222;
              }
              v95 = *(long long *)(i + 48);
              sub_E9F0((void *)i);
            }
          }
          if ( v24 == v90 )
          {
LABEL_207:
            v22 = 0;
            v91 = sub_10B80(0, 3, v111);
            v92 = dcgettext(
                    0,
                    "WARNING: Circular directory structure.\n"
                    "This almost certainly means that you have a corrupted file system.\n"
                    "NOTIFY YOUR SYSTEM MANAGER.\n"
                    "The following directory is part of the cycle:\n"
                    "  %s\n",
                    5);
            error(0, 0, v92, v91);
          }
          else
          {
            while ( (int)sub_6260(qword_1B190, *(long long *)(v24 + 112), *(long long *)(v24 + 120)) <= 0 )
            {
              v24 = *(long long *)(v24 + 8);
              if ( !v24 || v90 == v24 )
                goto LABEL_207;
            }
          }
        }
      }
      v27 = 0;
      v57 = sub_10800(4, v25);
      v58 = dcgettext(0, "cannot read directory %s", 5);
      error(0, *(int *)(v24 + 64), v58, v57);
LABEL_98:
      if ( dword_1B168 )
      {
        if ( dword_1B168 == 2 )
        {
          v32 = *(long long *)(v24 + 184);
          v33 = *(long long *)(v24 + 192);
        }
        else
        {
          v32 = *(long long *)(v24 + 216);
          v33 = *(long long *)(v24 + 224);
        }
      }
      else
      {
        v32 = *(long long *)(v24 + 200);
        v33 = *(long long *)(v24 + 208);
      }
      if ( byte_1B185 )
      {
        v34 = 0;
        if ( *(long long *)(v24 + 160) >= 0 )
          v34 = *(long long *)(v24 + 160);
        v35 = v34;
      }
      else
      {
        v35 = *(long long *)(v24 + 176) << 9;
      }
      v36 = *(long long *)(v24 + 88);
      v125 = v35;
      v126 = 1;
      v127.m128i_i64[0] = v32;
      v127.m128i_i64[1] = v33;
      if ( qword_1B108 )
      {
        v37 = qword_1B188;
        if ( v36 != qword_1B188 )
        {
          if ( v36 <= qword_1B188 )
          {
            if ( v36 != qword_1B188 - 1 )
              __assert_fail("level == prev_level - 1", "src/du.c", 0x27Eu, "process_file");
            v63 = (const __m128i *)(qword_1B100 + (qword_1B188 << 6));
            v64 = v63->m128i_i64[0] + v35;
            if ( __CFADD__(v63->m128i_i64[0], v35) )
              v64 = -1;
            v65 = v63->m128i_i64[1] + 1;
            v66 = v32 < v63[1].m128i_i64[0];
            v67 = v32 > v63[1].m128i_i64[0];
            v125 = v64;
            v126 = v65;
            if ( (v33 > v63[1].m128i_i64[1]) - (v33 < v63[1].m128i_i64[1]) + 2 * (v67 - v66) < 0 )
              v127 = _mm_loadu_si128(v63 + 1);
            if ( !byte_1B180 )
            {
              v78 = __CFADD__(v63[2].m128i_i64[0], v64);
              v79 = v63[2].m128i_i64[0] + v64;
              if ( v78 )
                v79 = -1;
              v80 = v63[3].m128i_i64[0];
              v126 = v63[2].m128i_i64[1] + v65;
              v125 = v79;
              if ( (v127.m128i_i64[1] > v63[3].m128i_i64[1])
                 - (v127.m128i_i64[1] < v63[3].m128i_i64[1])
                 + 2 * ((v127.m128i_i64[0] > v80) - (v127.m128i_i64[0] < v80)) < 0 )
                v127 = _mm_loadu_si128(v63 + 3);
            }
            v68 = -1;
            v69 = (__m128i *)(qword_1B100 + (v36 << 6));
            v70 = v69[3].m128i_i64[0];
            if ( !__CFADD__(v69[2].m128i_i64[0], v63->m128i_i64[0]) )
              v68 = v69[2].m128i_i64[0] + v63->m128i_i64[0];
            v71 = v69[3].m128i_i64[1];
            v119 = v68;
            v69[2].m128i_i64[0] = v68;
            v72 = v69[2].m128i_i64[1] + v63->m128i_i64[1];
            v45 = __OFSUB__(v70, v63[1].m128i_i64[0]);
            v43 = v70 == v63[1].m128i_i64[0];
            v44 = v70 - v63[1].m128i_i64[0] < 0;
            v116 = v71;
            v69[2].m128i_i64[1] = v72;
            if ( (v71 > v63[1].m128i_i64[1])
               - (v71 < v63[1].m128i_i64[1])
               + 2 * (!(v44 ^ v45 | v43) - (v44 ^ (unsigned char)v45)) < 0 )
            {
              v70 = v63[1].m128i_i64[0];
              v69[3] = _mm_loadu_si128(v63 + 1);
              v116 = v69[3].m128i_i64[1];
            }
            v73 = v63[2].m128i_i64[0] + v119;
            if ( __CFADD__(v63[2].m128i_i64[0], v119) )
              v73 = -1;
            v69[2].m128i_i64[1] = v63[2].m128i_i64[1] + v72;
            v45 = __OFSUB__(v70, v63[3].m128i_i64[0]);
            v43 = v70 == v63[3].m128i_i64[0];
            v44 = v70 - v63[3].m128i_i64[0] < 0;
            v69[2].m128i_i64[0] = v73;
            if ( (v116 > v63[3].m128i_i64[1])
               - (v116 < v63[3].m128i_i64[1])
               + 2 * (!(v44 ^ v45 | v43) - (v44 ^ (unsigned char)v45)) < 0 )
              v69[3] = _mm_loadu_si128(v63 + 3);
          }
          else
          {
            if ( v36 >= qword_1B108 )
            {
              v120 = v35;
              v117 = v33;
              v81 = sub_11D40(qword_1B100, v36, 128);
              v35 = v120;
              v33 = v117;
              qword_1B100 = v81;
              qword_1B108 = 2 * v36;
              v37 = qword_1B188;
            }
            v38 = v37 + 1;
            if ( v36 >= v37 + 1 )
            {
              v39 = (long long *)(qword_1B100 + (v37 << 6));
              do
              {
                ++v38;
                v39[8] = 0;
                v39 += 8;
                v39[1] = 0;
                v39[2] = 0x8000000000000000LL;
                v39[3] = -1;
                v39[4] = 0;
                v39[5] = 0;
                v39[6] = 0x8000000000000000LL;
                v39[7] = -1;
              }
              while ( v36 >= v38 );
            }
          }
        }
      }
      else
      {
        v118 = v35;
        v115 = v33;
        qword_1B108 = v36 + 10;
        v56 = sub_12060(v36 + 10, 64);
        v33 = v115;
        v35 = v118;
        qword_1B100 = v56;
      }
      v40 = v26 & 0xFFFFFFFD;
      qword_1B188 = v36;
      if ( !byte_1B180 || v40 != 4 )
      {
        v41 = (long long *)(qword_1B100 + (v36 << 6));
        v42 = *v41 + v35;
        if ( __CFADD__(*v41, v35) )
          v42 = -1;
        ++v41[1];
        v45 = __OFSUB__(v32, v41[2]);
        v43 = v32 == v41[2];
        v44 = v32 - v41[2] < 0;
        v46 = v32 < v41[2];
        *v41 = v42;
        if ( (v33 < v41[3]) - (v33 > v41[3]) + 2 * (v46 - !(v44 ^ v45 | v43)) < 0 )
        {
          v41[2] = v32;
          v41[3] = v33;
        }
      }
      v47 = qword_1B120 + v35;
      if ( __CFADD__(qword_1B120, v35) )
        v47 = -1;
      ++qword_1B128;
      qword_1B120 = v47;
      if ( (v33 < qword_1B138) - (v33 > qword_1B138) + 2 * ((v32 < qword_1B130) - (v32 > qword_1B130)) < 0 )
      {
        qword_1B130 = v32;
        qword_1B138 = v33;
      }
      if ( (v40 == 4 || byte_1B186) && v36 <= qword_1B010 || !v36 )
      {
        v48 = v126;
        if ( !byte_1B16D )
          v48 = v125;
        v49 = v48 >= qword_1B178;
        if ( qword_1B178 < 0 )
          v49 = -qword_1B178 >= v48;
        v22 &= v27;
        if ( v49 )
          sub_53D0(&v125, v111);
      }
      else
      {
        v22 &= v27;
      }
      goto LABEL_71;
    }
    break;
  }
  switch ( longind )
  {
    case 3:
LABEL_222:
      sub_12180();
    case 4:
      v59 = sub_10B80(0, 3, s1);
      v60 = dcgettext(0, "%s: read error", 5);
      v61 = __errno_location();
      error(0, *v61, v60, v59);
      v122 = 0;
      break;
    case 2:
      break;
    default:
      __assert_fail("!\"unexpected error code from argv_iter\"", "src/du.c", 0x439u, "main");
  }
  sub_5F40(v19);
  sub_61A0(qword_1B198);
  if ( qword_1B190 )
    sub_61A0(qword_1B190);
  if ( s1 && ((stdin->_flags & 0x20) != 0 || (unsigned int)sub_71A0(stdin)) && v122 )
  {
    v97 = sub_10800(4, s1);
    v98 = dcgettext(0, "error reading %s", 5);
    error(1, 0, v98, v97);
  }
  if ( byte_1B181 )
  {
    v76 = dcgettext(0, "total", 5);
    sub_53D0(&qword_1B120, v76);
  }
  return v122 ^ 1u;
}



// Function: print_only_size @ 0x5340
unsigned long long print_only_size(long long a1)
{
  FILE *v1; // rbp
  char *v2; // rdi
  long long v4; // [rsp+0h] [rbp-2A8h] BYREF
  unsigned long long v5; // [rsp+298h] [rbp-10h]

  v1 = stdout;
  v5 = __readfsqword(0x28u);
  if ( a1 == -1 )
    v2 = dcgettext(0, "Infinity", 5);
  else
    v2 = (char *)sub_C920(a1, &v4, (unsigned int)dword_1B170, 1, qword_1B148);
  fputs_unlocked(v2, v1);
  return v5 - __readfsqword(0x28u);
}



// Function: print_size @ 0x53d0
int print_size(const __m128i *a1, const char *a2)
{
  long long v3; // rdi
  char *IO_write_ptr; // rax
  long long v5; // r14
  long long v6; // rbp
  const char *v8; // rbp
  long long v9; // r13
  char *v10; // rax
  __m128i v11; // [rsp+0h] [rbp-98h] BYREF
  char v12[64]; // [rsp+10h] [rbp-88h] BYREF
  char v13[24]; // [rsp+50h] [rbp-48h] BYREF
  unsigned long long v14; // [rsp+68h] [rbp-30h]

  v3 = a1->m128i_i64[1];
  v14 = __readfsqword(0x28u);
  if ( !byte_1B16D )
    v3 = a1->m128i_i64[0];
  sub_5340(v3);
  if ( byte_1B16C )
  {
    IO_write_ptr = stdout->_IO_write_ptr;
    if ( IO_write_ptr >= stdout->_IO_write_end )
    {
      __overflow(stdout, 9);
    }
    else
    {
      stdout->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 9;
    }
    v5 = qword_1B150;
    v6 = qword_1B158;
    v11 = _mm_loadu_si128(a1 + 1);
    if ( sub_112F0(qword_1B150, &v11, v12) )
    {
      sub_8E00(stdout, v6, v12, v5, v11.m128i_u32[2]);
    }
    else
    {
      v8 = (const char *)sub_D920(v11.m128i_i64[0], v13);
      v9 = sub_10EF0(v8);
      v10 = dcgettext(0, "time %s is out of range", 5);
      error(0, 0, v10, v9);
      fputs_unlocked(v8, stdout);
    }
  }
  __printf_chk(1, "\t%s%c", a2, byte_1B182 == 0 ? 0xA : 0);
  return fflush_unlocked(stdout);
}



// Function: usage @ 0x5540
void usage(int status)
{
  long long v2; // r13
  char *v3; // rax
  const char **v4; // rbx
  char *v5; // rax
  FILE *v6; // rbp
  char *v7; // rax
  FILE *v8; // rbp
  char *v9; // rax
  FILE *v10; // rbp
  char *v11; // rax
  FILE *v12; // rbp
  char *v13; // rax
  FILE *v14; // rbp
  char *v15; // rax
  FILE *v16; // rbp
  char *v17; // rax
  FILE *v18; // rbp
  char *v19; // rax
  FILE *v20; // rbp
  char *v21; // rax
  FILE *v22; // rbp
  char *v23; // rax
  FILE *v24; // rbp
  char *v25; // rax
  FILE *v26; // rbp
  char *v27; // rax
  char *v28; // rax
  FILE *v29; // rbp
  char *v30; // rax
  const char *v31; // rsi
  const char *v32; // r14
  char *v33; // rax
  long long v34; // r8
  long long v35; // r9
  char *v36; // rdi
  const char *v37; // r13
  char *v38; // rax
  const char *v39; // rcx
  char *v40; // r13
  char *v41; // rax
  FILE *v42; // rbx
  char *v43; // rax
  const char *v44; // [rsp+0h] [rbp-B8h] BYREF
  const char *v45; // [rsp+8h] [rbp-B0h]
  char *v46; // [rsp+10h] [rbp-A8h]
  const char *v47; // [rsp+18h] [rbp-A0h]
  const char *v48; // [rsp+20h] [rbp-98h]
  const char *v49; // [rsp+28h] [rbp-90h]
  const char *v50; // [rsp+30h] [rbp-88h]
  const char *v51; // [rsp+38h] [rbp-80h]
  const char *v52; // [rsp+40h] [rbp-78h]
  const char *v53; // [rsp+48h] [rbp-70h]
  const char *v54; // [rsp+50h] [rbp-68h]
  const char *v55; // [rsp+58h] [rbp-60h]
  long long v56; // [rsp+60h] [rbp-58h]
  long long v57; // [rsp+68h] [rbp-50h]
  unsigned long long v58; // [rsp+78h] [rbp-40h]

  v2 = qword_1B1B0;
  v58 = __readfsqword(0x28u);
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = &v44;
    v5 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5);
    __printf_chk(1, v5, v2, v2);
    v6 = stdout;
    v7 = dcgettext(0, "Summarize device usage of the set of FILEs, recursively for directories.\n", 5);
    fputs_unlocked(v7, v6);
    v8 = stdout;
    v9 = dcgettext(0, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = dcgettext(
            0,
            "  -0, --null            end each output line with NUL, not newline\n"
            "  -a, --all             write counts for all files, not just directories\n"
            "      --apparent-size   print apparent sizes rather than device usage; although\n"
            "                          the apparent size is usually smaller, it may be\n"
            "                          larger due to holes in ('sparse') files, internal\n"
            "                          fragmentation, indirect blocks, and the like\n",
            5);
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = dcgettext(
            0,
            "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n"
            "                           '-BM' prints sizes in units of 1,048,576 bytes;\n"
            "                           see SIZE format below\n"
            "  -b, --bytes           equivalent to '--apparent-size --block-size=1'\n"
            "  -c, --total           produce a grand total\n"
            "  -D, --dereference-args  dereference only symlinks that are listed on the\n"
            "                          command line\n"
            "  -d, --max-depth=N     print the total for a directory (or file, with --all)\n"
            "                          only if it is N or fewer levels below the command\n"
            "                          line argument;  --max-depth=0 is the same as\n"
            "                          --summarize\n",
            5);
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = dcgettext(
            0,
            "      --files0-from=F   summarize device usage of the\n"
            "                          NUL-terminated file names specified in file F;\n"
            "                          if F is -, then read names from standard input\n"
            "  -H                    equivalent to --dereference-args (-D)\n"
            "  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n"
            "      --inodes          list inode usage information instead of block usage\n",
            5);
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = dcgettext(
            0,
            "  -k                    like --block-size=1K\n"
            "  -L, --dereference     dereference all symbolic links\n"
            "  -l, --count-links     count sizes many times if hard linked\n"
            "  -m                    like --block-size=1M\n",
            5);
    fputs_unlocked(v17, v16);
    v18 = stdout;
    v19 = dcgettext(
            0,
            "  -P, --no-dereference  don't follow any symbolic links (this is the default)\n"
            "  -S, --separate-dirs   for directories do not include size of subdirectories\n"
            "      --si              like -h, but use powers of 1000 not 1024\n"
            "  -s, --summarize       display only a total for each argument\n",
            5);
    fputs_unlocked(v19, v18);
    v20 = stdout;
    v21 = dcgettext(
            0,
            "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n"
            "                          or entries greater than SIZE if negative\n"
            "      --time            show time of the last modification of any file in the\n"
            "                          directory, or any of its subdirectories\n"
            "      --time=WORD       show time as WORD instead of modification time:\n"
            "                          atime, access, use, ctime or status\n"
            "      --time-style=STYLE  show times using STYLE, which can be:\n"
            "                            full-iso, long-iso, iso, or +FORMAT;\n"
            "                            FORMAT is interpreted like in 'date'\n",
            5);
    fputs_unlocked(v21, v20);
    v22 = stdout;
    v23 = dcgettext(
            0,
            "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n"
            "      --exclude=PATTERN    exclude files that match PATTERN\n"
            "  -x, --one-file-system    skip directories on different file systems\n",
            5);
    fputs_unlocked(v23, v22);
    v24 = stdout;
    v25 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v25, v24);
    v26 = stdout;
    v27 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v27, v26);
    v28 = dcgettext(
            0,
            "\n"
            "Display values are in units of the first available SIZE from --block-size,\n"
            "and the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\n"
            "Otherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n",
            5);
    __printf_chk(1, v28, "DU");
    v29 = stdout;
    v30 = dcgettext(
            0,
            "\n"
            "The SIZE argument is an integer and optional unit (example: 10K is 10*1024).\n"
            "Units are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\n"
            "Binary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",
            5);
    fputs_unlocked(v30, v29);
    v44 = "[";
    v31 = "[";
    v45 = "test invocation";
    v46 = "coreutils";
    v47 = "Multi-call invocation";
    v50 = "sha256sum";
    v48 = "sha224sum";
    v52 = "sha384sum";
    v49 = "sha2 utilities";
    v51 = "sha2 utilities";
    v53 = "sha2 utilities";
    v54 = "sha512sum";
    v55 = "sha2 utilities";
    v56 = 0;
    v57 = 0;
    do
    {
      if ( !strcmp("du", v31) )
        break;
      v31 = v4[2];
      v4 += 2;
    }
    while ( v31 );
    v32 = v4[1];
    if ( !v32 )
      v32 = "du";
    v33 = dcgettext(0, "\n%s online help: <%s>\n", 5);
    __printf_chk(
      1,
      v33,
      "GNU coreutils",
      "https://www.gnu.org/software/coreutils/",
      v34,
      v35,
      v44,
      v45,
      v46,
      v47,
      v48,
      v49,
      v50,
      v51,
      v52,
      v53,
      v54,
      v55,
      v56,
      v57);
    v36 = setlocale(5, 0);
    if ( v36 && strncmp(v36, "en_", 3u) )
    {
      v42 = stdout;
      v43 = dcgettext(0, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
      fputs_unlocked(v43, v42);
    }
    v37 = "du";
    if ( !strcmp("du", "[") )
      v37 = "test";
    v38 = dcgettext(0, "Full documentation <%s%s>\n", 5);
    v39 = v37;
    v40 = " invocation";
    __printf_chk(1, v38, "https://www.gnu.org/software/coreutils/", v39);
    if ( v32 != "du" )
      v40 = (char *)"";
    v41 = dcgettext(0, "or available locally via: info '(coreutils) %s%s'\n", 5);
    __printf_chk(1, v41, v32, v40);
  }
  exit(status);
}



// Function: raw_hasher @ 0xb750
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return __ROR8__(a1, 3) % a2;
}



// Function: raw_comparator @ 0xb770
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0xb780
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_15AA0 )
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
    *(long long *)(a1 + 40) = &unk_15AA0;
    return 0;
  }
  return result;
}



// Function: hash_find_entry @ 0xb810
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



// Function: transfer_entries @ 0xba60
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



// Function: hash_print_statistics @ 0xbcd0
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



// Function: hash_lookup @ 0xbe30
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



// Function: hash_get_next @ 0xbef0
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



// Function: hash_do_for_each @ 0xbfd0
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



// Function: hash_string @ 0xc050
unsigned long long hash_string(unsigned char *a1, unsigned long long a2)
{
  long long v2; // rcx
  unsigned long long i; // rdx
  unsigned long long v4; // rax

  v2 = *a1;
  for ( i = 0; (char)v2; i = v4 % a2 )
  {
    ++a1;
    v4 = v2 + 31 * i;
    v2 = *a1;
  }
  return i;
}



// Function: hash_initialize @ 0xc0c0
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
    v7 = sub_B750;
  if ( !a4 )
    v8 = sub_B770;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_15AA0;
    v9[5] = v5;
    if ( (unsigned char)sub_B780((long long)v9)
      && (v10 = sub_B920(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
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



// Function: hash_clear @ 0xc1d0
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



// Function: hash_free @ 0xc280
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



// Function: hash_rehash @ 0xc370
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
  v2 = sub_B920(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
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
  v6 = sub_BA60((long long)&ptr, a1, 0);
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
    if ( !(unsigned char)sub_BA60(a1, (long long)&ptr, 1) || !(unsigned char)sub_BA60(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0xc4f0
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
  v5 = sub_B810((long long)a1, a2, v22, 0);
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
          sub_B780((long long)a1);
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
            if ( !(unsigned char)sub_C370((long long)a1, v21) )
              return (unsigned int)-1;
            if ( sub_B810((long long)a1, a2, v22, 0) )
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



// Function: hash_insert @ 0xc720
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_C4F0(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0xc780
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
  v3 = sub_B810((long long)a1, a2, v18, 1);
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
    sub_B780((long long)a1);
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
      if ( !(unsigned char)sub_C370((long long)a1, v14) )
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


