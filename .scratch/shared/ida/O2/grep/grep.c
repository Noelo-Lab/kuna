// Function: main @ 0x4ff0
long long main(int a1, char **a2, char **a3)
{
  char *p_s; // r12
  char *v4; // rbp
  char v5; // r14
  int v6; // ebx
  int v7; // edx
  char *v8; // rax
  long long v9; // rdi
  int v10; // ebx
  long long v11; // rdi
  char *v12; // rbp
  const char *v13; // rbp
  char *v14; // rax
  FILE *v15; // rbp
  long long v16; // rsi
  long long v17; // rsi
  int *v18; // rax
  long long v19; // rdi
  long long v20; // rdi
  char *v21; // rax
  const char *v22; // rbp
  char *v23; // r14
  long long i; // r12
  long long v25; // rdx
  long long v26; // rax
  size_t v27; // rax
  char *v28; // rbx
  long long v29; // r14
  int *v30; // rax
  int flags; // esi
  char *v32; // r14
  long long v33; // rbp
  size_t v34; // rbx
  long long v35; // rax
  int v36; // eax
  bool v37; // zf
  int v38; // eax
  char *v39; // rbx
  char v40; // bp
  int *v42; // rax
  long long v43; // rdx
  int v44; // eax
  unsigned char v45; // si
  int v46; // ecx
  long long v47; // rsi
  int v48; // eax
  long long v49; // rax
  char *v50; // r15
  long long v51; // rbx
  char *v52; // rbp
  void *v53; // r12
  char v54; // cl
  long long v55; // rax
  signed long long v56; // r13
  char *v57; // rsi
  char *v58; // rax
  int v59; // ebx
  int v60; // eax
  unsigned long long v61; // rcx
  unsigned long long v62; // rax
  char **v63; // rbx
  const char *v64; // rbp
  int v65; // r12d
  int v66; // eax
  int v67; // eax
  int v68; // eax
  long long v69; // rax
  long long v70; // rdx
  char *v71; // rax
  size_t v72; // rax
  char v73; // al
  char *v74; // r13
  char *v75; // r14
  long long v76; // rdi
  int v77; // eax
  long long v78; // rdx
  char *v79; // r15
  char *v80; // rbp
  wchar_t v81; // esi
  char v82; // al
  char v83; // cl
  unsigned char v84; // cl
  char *v85; // rax
  char *v86; // r13
  char *j; // rdx
  char *v88; // rax
  long long v89; // rax
  const char *v90; // r12
  const char *v91; // r13
  char *v92; // r15
  const char *v93; // rbx
  char v94; // r14
  char *v95; // rax
  char **v96; // rbp
  const char *v97; // rdi
  char *v98; // rax
  void (*v99)(void); // rax
  int v100; // eax
  char *v101; // rax
  long long v102; // rax
  long long *v103; // rdi
  long long v104; // rcx
  struct stat *p_buf; // rsi
  long long v106; // rbx
  long long v107; // rax
  char *v108; // rax
  long long k; // rbx
  long long v110; // rax
  long long v111; // rax
  char *v112; // rax
  char v113; // [rsp+0h] [rbp-208h]
  int streamb; // [rsp+10h] [rbp-1F8h]
  FILE *stream; // [rsp+10h] [rbp-1F8h]
  FILE *streama; // [rsp+10h] [rbp-1F8h]
  int streamc; // [rsp+10h] [rbp-1F8h]
  int v119; // [rsp+18h] [rbp-1F0h]
  long long v120; // [rsp+18h] [rbp-1F0h]
  int v121; // [rsp+18h] [rbp-1F0h]
  int v123; // [rsp+24h] [rbp-1E4h]
  char *src; // [rsp+28h] [rbp-1E0h]
  long long *v125; // [rsp+30h] [rbp-1D8h]
  int v126; // [rsp+38h] [rbp-1D0h]
  int v127; // [rsp+3Ch] [rbp-1CCh]
  char *v128; // [rsp+40h] [rbp-1C8h]
  long long v129; // [rsp+40h] [rbp-1C8h]
  long long v130; // [rsp+48h] [rbp-1C0h]
  long long v131; // [rsp+58h] [rbp-1B0h] BYREF
  long long v132; // [rsp+60h] [rbp-1A8h] BYREF
  long long v133; // [rsp+68h] [rbp-1A0h] BYREF
  char v134[8]; // [rsp+70h] [rbp-198h] BYREF
  mbstate_t ps; // [rsp+78h] [rbp-190h] BYREF
  struct stat buf; // [rsp+80h] [rbp-188h] BYREF
  struct stat v137; // [rsp+110h] [rbp-F8h] BYREF
  char s; // [rsp+1A0h] [rbp-68h] BYREF
  char v139[20]; // [rsp+1A1h] [rbp-67h] BYREF
  char v140; // [rsp+1B5h] [rbp-53h] BYREF
  char v141; // [rsp+1B8h] [rbp-50h] BYREF
  unsigned long long v142; // [rsp+1C8h] [rbp-40h]

  v142 = __readfsqword(0x28u);
  byte_2B684 = 10;
  v131 = 0;
  v132 = 0;
  status = 2;
  dword_2B804 = -1;
  qword_2B7E0 = 0x7FFFFFFFFFFFFFFFLL;
  qword_2B7F8 = -1;
  qword_2B7F0 = -1;
  v133 = -1;
  byte_2B8FD = 0;
  setlocale(6, "");
  bindtextdomain("grep", "/usr/local/share/locale");
  textdomain("grep");
  sub_16210(byte_2B180);
  sub_20120(&loc_7A00);
  sub_F6A0(0);
  qword_2B8C8 = (void *)sub_1A2D0(0, 0, sub_7850, sub_78A0, 0);
  if ( !qword_2B8C8 )
    sub_1F410();
  v127 = 0;
  v125 = &v132;
  p_s = &s;
  v126 = 0;
  v123 = -1;
  src = 0;
  while ( 1 )
  {
    v4 = p_s;
    v5 = 0;
    v119 = optind;
    v6 = optind;
    while ( 1 )
    {
      v7 = getopt_long(a1, a2, "0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz", &longopts, 0);
      if ( (unsigned int)(v7 - 48) > 9 )
        break;
      if ( dword_2B010 == v6 && v5 )
      {
        v8 = &v4[-(s == 48)];
        if ( v8 == &v140 )
        {
          *(int *)v8 = 3026478;
          v4 = &v141;
          goto LABEL_10;
        }
      }
      else
      {
        v8 = p_s;
      }
      dword_2B010 = v6;
      v4 = v8 + 1;
      v6 = optind;
      v5 = 1;
      *v8 = v7;
    }
    if ( v4 == p_s )
      goto LABEL_11;
LABEL_10:
    *v4 = 0;
    streamb = v7;
    sub_7D10(p_s, &v133);
    v7 = streamb;
LABEL_11:
    if ( v7 == -1 )
      break;
    switch ( v7 )
    {
      case 0:
      case 85:
        continue;
      case 65:
        sub_7D10(optarg, &qword_2B7F0);
        continue;
      case 66:
        sub_7D10(optarg, &qword_2B7F8);
        continue;
      case 67:
        sub_7D10(optarg, &v133);
        continue;
      case 68:
        v15 = (FILE *)optarg;
        if ( strcmp(optarg, "read") )
          goto LABEL_117;
        dword_2B884 = 1;
        continue;
      case 69:
        v123 = sub_7950("egrep");
        continue;
      case 70:
        v123 = sub_7950("fgrep");
        continue;
      case 71:
        v123 = sub_7950("grep");
        continue;
      case 72:
        v127 = 1;
        continue;
      case 73:
        goto LABEL_24;
      case 76:
        dword_2B80C = 2;
        continue;
      case 80:
        v123 = sub_7950("perl");
        continue;
      case 82:
        dword_2B014 = 778;
        goto LABEL_20;
      case 84:
        byte_2B8FC = 1;
        continue;
      case 86:
        byte_2B905 = 1;
        continue;
      case 88:
        v123 = sub_7950(optarg);
        continue;
      case 90:
        dword_2B804 = 0;
        continue;
      case 97:
        goto LABEL_22;
      case 98:
        byte_2B800 = 1;
        continue;
      case 99:
        byte_2B7E8 = 1;
        continue;
      case 100:
        v36 = dword_22788[sub_F3F0("--directories", optarg, off_29FA0, dword_22788, 4, off_2B078, 1)];
        v37 = v36 == 3;
        dword_2B018 = v36;
        v38 = v119;
        if ( !v37 )
          v38 = v126;
        v126 = v38;
        continue;
      case 101:
        v32 = optarg;
        v33 = v131;
        v34 = strlen(optarg);
        if ( (long long)(v34 + v131 - v132 + 1) > 0 )
        {
          v35 = sub_1F1F0(src);
          v32 = optarg;
          src = (char *)v35;
          qword_2B8D8 = v35;
        }
        *(char *)mempcpy(&src[v131], v32, v34) = 10;
        v131 = sub_7A20(src, v33, v34 + v33 + 1, "");
        continue;
      case 102:
        v22 = optarg;
        if ( !strcmp(optarg, "-") )
        {
          v15 = (FILE *)stdin;
LABEL_75:
          v23 = src;
          v128 = p_s;
          v120 = v131;
          for ( i = v131; ; i += v27 )
          {
            v25 = v132;
            if ( i - v132 + 2 > 0 )
            {
              v26 = sub_1F1F0(v23);
              v25 = v132;
              qword_2B8D8 = v26;
              v23 = (char *)v26;
            }
            v27 = fread_unlocked(&v23[i], 1u, v25 - (i + 1), v15);
            if ( !v27 )
              break;
          }
          v130 = i + 1;
          v28 = &v23[i];
          src = v23;
          v29 = i;
          p_s = v128;
          v30 = __errno_location();
          flags = *v30;
          if ( (v15->_flags & 0x20) != 0 )
            goto LABEL_81;
          if ( (FILE *)stdin != v15 )
          {
            stream = (FILE *)v30;
            if ( !fclose(v15) )
              goto LABEL_82;
            flags = stream->_flags;
LABEL_81:
            if ( !flags )
              goto LABEL_82;
            error(2, flags, "%s", optarg);
LABEL_307:
            sub_A170(0);
          }
          clearerr_unlocked(v15);
LABEL_82:
          if ( v29 != v131 && src[v29 - 1] != 10 )
          {
            *v28 = 10;
            v29 = v130;
          }
          v131 = sub_7A20(src, v120, v29, optarg);
          continue;
        }
        v15 = fopen(v22, "r");
        if ( v15 )
          goto LABEL_75;
        v42 = __errno_location();
        error(2, *v42, "%s", optarg);
LABEL_117:
        if ( strcmp((const char *)v15, "skip") )
        {
          v108 = dcgettext(0, "unknown devices method", 5);
          error(2, 0, v108);
          goto LABEL_296;
        }
        dword_2B884 = 2;
        break;
      case 104:
        v127 = -1;
        continue;
      case 105:
      case 121:
        byte_2B687 = 1;
        continue;
      case 108:
        goto LABEL_72;
      case 109:
        if ( (unsigned int)sub_1F450(optarg) <= 1 )
          continue;
        v21 = dcgettext(0, "invalid max count", 5);
        error(2, 0, v21);
LABEL_72:
        dword_2B80C = 1;
        continue;
      case 110:
        byte_2B801 = 1;
        continue;
      case 111:
        byte_2B8FD = 1;
        continue;
      case 113:
        goto LABEL_58;
      case 114:
LABEL_20:
        dword_2B018 = 3;
        v126 = v119;
        continue;
      case 115:
        byte_2B904 = 1;
        continue;
      case 117:
        v14 = dcgettext(0, "warning: --unix-byte-offsets (-u) is obsolete", 5);
        error(0, 0, v14);
        continue;
      case 118:
        byte_2B802 = 1;
        continue;
      case 119:
        sub_E570();
        byte_2B686 = 1;
        continue;
      case 120:
        byte_2B685 = 1;
        continue;
      case 122:
        byte_2B684 = 0;
        continue;
      case 128:
        v13 = optarg;
        if ( !strcmp(optarg, "binary") )
        {
          dword_2B810 = 0;
        }
        else if ( !strcmp(v13, "text") )
        {
LABEL_22:
          dword_2B810 = 1;
        }
        else
        {
          if ( strcmp(v13, "without-match") )
          {
            v112 = dcgettext(0, "unknown binary-files type", 5);
            error(2, 0, v112);
          }
LABEL_24:
          dword_2B810 = 2;
        }
        continue;
      case 129:
        v12 = optarg;
        if ( !optarg )
          goto LABEL_177;
        if ( (unsigned int)sub_F750(optarg, "always")
          && (unsigned int)sub_F750(v12, "yes")
          && (unsigned int)sub_F750(v12, "force") )
        {
          if ( (unsigned int)sub_F750(v12, "never")
            && (unsigned int)sub_F750(v12, "no")
            && (unsigned int)sub_F750(v12, "none") )
          {
            if ( (unsigned int)sub_F750(v12, "auto")
              && (unsigned int)sub_F750(v12, "tty")
              && (unsigned int)sub_F750(v12, "if-tty") )
            {
              dword_2B908 = 1;
            }
            else
            {
LABEL_177:
              dword_2B900 = 2;
            }
          }
          else
          {
            dword_2B900 = 0;
          }
        }
        else
        {
          dword_2B900 = 1;
        }
        continue;
      case 130:
        sub_16480(optarg);
        v19 = qword_2B8A0;
        if ( !qword_2B8A0 )
        {
          qword_2B8A0 = sub_169D0();
          v19 = qword_2B8A0;
        }
        sub_16D40(v19, optarg, 1342177280);
        v20 = qword_2B8A8;
        if ( !qword_2B8A8 )
        {
          qword_2B8A8 = sub_169D0();
          v20 = qword_2B8A8;
        }
        sub_16D40(v20, optarg, 0x10000000);
        continue;
      case 131:
      case 134:
        v9 = qword_2B8B0;
        if ( !qword_2B8B0 )
        {
          streamc = v7;
          v102 = sub_169D0();
          v7 = streamc;
          qword_2B8B0 = v102;
          v9 = v102;
        }
        v10 = (v7 == 134) << 29;
        sub_16D40(v9, optarg, v10 | 0x50000000u);
        v11 = qword_2B8B8;
        if ( !qword_2B8B8 )
        {
          qword_2B8B8 = sub_169D0();
          v11 = qword_2B8B8;
        }
        sub_16D40(v11, optarg, v10 | 0x10000000u);
        continue;
      case 132:
        v16 = qword_2B8B0;
        if ( !qword_2B8B0 )
        {
          qword_2B8B0 = sub_169D0();
          v16 = qword_2B8B0;
        }
        if ( (unsigned int)sub_17290(sub_16D40, v16, optarg, 1342177280, 10) )
          goto LABEL_57;
        v17 = qword_2B8B8;
        if ( !qword_2B8B8 )
        {
          qword_2B8B8 = sub_169D0();
          v17 = qword_2B8B8;
        }
        if ( (unsigned int)sub_17290(sub_16D40, v17, optarg, 0x10000000, 10) )
        {
LABEL_57:
          v18 = __errno_location();
          error(2, *v18, "%s", optarg);
LABEL_58:
          byte_2B7A1 = 1;
          status = 0;
        }
        continue;
      case 133:
        off_2B070 = optarg;
        continue;
      case 135:
        byte_2B7D8 = 1;
        continue;
      case 136:
        qword_2B7D0 = (long long)optarg;
        continue;
      case 137:
        byte_2B687 = 0;
        continue;
      default:
        goto LABEL_270;
    }
  }
  v39 = p_s;
  if ( byte_2B905 )
  {
    sub_19950();
    sub_1ED40(stdout, v113);
    v101 = dcgettext(
             0,
             "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",
             5);
    puts(v101);
    return 0;
  }
  if ( dword_2B908 )
    goto LABEL_307;
  if ( src )
  {
    if ( !v131 )
    {
      byte_2B686 = 0;
      byte_2B685 = 0;
      byte_2B802 ^= 1u;
      v131 = 1;
      *src = 10;
    }
  }
  else
  {
    v69 = optind;
    if ( optind >= a1 )
LABEL_270:
      sub_A170(2);
    ++optind;
    v70 = 0;
    v71 = a2[v69];
    if ( v123 != 2 && *v71 == 92 )
      v70 = v71[1] == 45;
    src = (char *)sub_1F3F0(&v71[v70]);
    qword_2B8D8 = (long long)src;
    v72 = strlen(src);
    src[v72] = 10;
    v131 = sub_7A20(src, 0, v72 + 1, "");
  }
  v129 = v131;
  v125 = (long long *)--v131;
  sub_1A490(qword_2B8C8);
  v40 = byte_2B7A1;
  if ( byte_2B7A1 )
  {
    v40 = 0;
  }
  else
  {
    if ( !fstat(1, &buf) )
    {
      v68 = buf.st_mode & 0xF000;
      if ( v68 == 0x8000 )
      {
        v103 = &qword_2B920;
        v104 = 36;
        p_buf = &buf;
        while ( v104 )
        {
          *(int *)v103 = p_buf->st_dev;
          p_buf = (struct stat *)((char *)p_buf + 4);
          v103 = (long long *)((char *)v103 + 4);
          --v104;
        }
      }
      else if ( v68 == 0x2000 )
      {
        if ( !stat("/dev/null", &v137) && buf.st_ino == v137.st_ino && buf.st_dev == v137.st_dev )
          byte_2B7A0 = 1;
        else
          v40 = 1;
      }
    }
    if ( !byte_2B7A0 )
    {
      if ( !dword_2B80C )
        goto LABEL_108;
      goto LABEL_107;
    }
  }
  dword_2B80C = 0;
LABEL_107:
  byte_2B7E8 = 0;
  byte_2B7A2 = 1;
LABEL_108:
  byte_2B803 = byte_2B7A2 | byte_2B7E8;
  if ( qword_2B7F0 < 0 )
    qword_2B7F0 = v133;
  if ( qword_2B7F8 < 0 )
    qword_2B7F8 = v133;
  if ( (!qword_2B7E0 || !v125 && byte_2B802 && !byte_2B685 && !byte_2B686) && dword_2B80C != 2 )
    return 1;
  if ( dword_2B900 == 2 )
  {
    v100 = v40 && (unsigned int)sub_1FA50() && isatty(1) != 0;
    dword_2B900 = v100;
  }
  sub_1FA90();
  if ( !dword_2B900 )
    goto LABEL_122;
  v85 = getenv("GREP_COLOR");
  v86 = v85;
  if ( v85 && *v85 )
  {
    for ( j = v85; *j == 59 || (unsigned int)(*j - 48) <= 9; ++j )
    {
      if ( !j[1] )
      {
        off_2B060[0] = v85;
        off_2B068[0] = v85;
        break;
      }
    }
  }
  v88 = getenv("GREP_COLORS");
  if ( !v88 || !*v88 )
    goto LABEL_246;
  v89 = sub_1F3F0(v88);
  streama = (FILE *)v86;
  v90 = (const char *)(v89 + 1);
  v91 = (const char *)v89;
  v92 = v39;
  v93 = 0;
  while ( 1 )
  {
    v94 = *(v90 - 1);
    if ( v94 == 58 || !v94 )
      break;
    if ( v94 == 61 )
    {
      if ( v91 == v90 - 1 || v93 )
        goto LABEL_245;
      *((char *)v90 - 1) = 0;
      v93 = v90;
    }
    else if ( v93 && v94 != 59 && (unsigned int)(v94 - 48) > 9 )
    {
      goto LABEL_245;
    }
LABEL_259:
    ++v90;
  }
  *((char *)v90 - 1) = 0;
  v96 = off_2A620;
  v97 = "mt";
  do
  {
    if ( !strcmp(v97, v91) )
      break;
    v97 = v96[3];
    v96 += 3;
  }
  while ( v97 );
  v98 = v96[1];
  if ( v98 && v93 )
    *(long long *)v98 = v93;
  v99 = (void (*)(void))v96[2];
  if ( v99 )
    v99();
  if ( v94 )
  {
    v91 = v90;
    v93 = 0;
    goto LABEL_259;
  }
LABEL_245:
  v86 = (char *)streama;
  v39 = v92;
LABEL_246:
  if ( off_2B068[0] == v86 || off_2B060[0] == v86 )
  {
    v95 = dcgettext(0, "warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'", 5);
    error(0, 0, v95, v86, v86);
  }
LABEL_122:
  v43 = 1;
  v44 = 1;
  v45 = 0;
  do
  {
    while ( byte_2B183[v43] == 1 || ((unsigned char)v43 & v45) != 0 )
    {
      if ( ++v43 == 256 )
        goto LABEL_128;
    }
    do
    {
      v46 = v44;
      v44 *= 2;
    }
    while ( v44 <= (int)v43 );
    ++v43;
    v45 |= v46;
    v44 = v46;
  }
  while ( v43 != 256 );
LABEL_128:
  v47 = 0x101010101010101LL * v45;
  v48 = 0;
  if ( v123 >= 0 )
    v48 = v123;
  qword_2B868 = v47;
  v121 = v48;
  if ( v123 > 2 )
    goto LABEL_160;
  if ( byte_2B687 )
  {
    v74 = (char *)&unk_2B284;
    v75 = byte_2B6A0;
    do
    {
      v76 = *(unsigned int *)v74;
      if ( (int)v76 != -1 )
      {
        v77 = sub_16380(v76, &v137);
        if ( v77 <= 0 )
        {
LABEL_213:
          v82 = 1;
        }
        else
        {
          v78 = 4LL * v77;
          v79 = (char *)&v137 + v78;
          v80 = (char *)&buf.__unused[2] + v78 + -4 * (unsigned int)(v77 - 1) + 4;
          while ( 1 )
          {
            v81 = *((int *)v79 - 1);
            ps = 0;
            if ( wcrtomb(v39, v81, &ps) != 1 )
              break;
            v79 -= 4;
            if ( v80 == v79 )
              goto LABEL_213;
          }
          v82 = -1;
        }
        *v75 = v82;
      }
      v74 += 4;
      ++v75;
    }
    while ( &byte_2B684 != v74 );
  }
  if ( v123 == 2 )
  {
    if ( !byte_2B180[0] )
    {
      v73 = 0;
      if ( qword_2B8D0 == 1 )
        v73 = byte_2B686;
      goto LABEL_201;
    }
    v106 = 0;
    v137.st_dev = 0;
    while ( (long long)v125 > v106 )
    {
      v107 = byte_2B180[(unsigned char)src[v106] + 3];
      if ( (char)v107 != 0xFE
        || (v107 = sub_1AC90(&src[v106], (char *)v125 - v106, &v137), (unsigned long long)v107 > 0x10) )
      {
        if ( v107 < 0 )
        {
LABEL_294:
          v73 = 1;
LABEL_201:
          v121 = 2;
          if ( (v73 & 1) != 0 )
          {
            sub_A3E0(&qword_2B8D8, &v131);
            v121 = 0;
            src = (char *)qword_2B8D8;
          }
          goto LABEL_160;
        }
      }
      v106 += v107;
    }
    if ( byte_2B687 )
    {
LABEL_296:
      v137.st_dev = 0;
      for ( k = 0; (long long)v125 > k; k += v110 )
      {
        v111 = (unsigned char)src[k];
        if ( *(int *)&byte_2B180[4 * v111 + 260] == -1 )
          v110 = sub_7D80(&src[k], (char *)v125 - k, &v137);
        else
          v110 = byte_2B6A0[v111];
        if ( v110 < 0 )
          goto LABEL_294;
      }
    }
    v73 = 0;
    goto LABEL_201;
  }
  if ( qword_2B8D0 <= 1 )
    goto LABEL_160;
  v49 = sub_1EF30(v129);
  v50 = src;
  v51 = (long long)v125;
  v137.st_dev = 0;
  v52 = (char *)v49;
  v53 = (void *)v49;
  while ( 2 )
  {
    if ( v51 )
    {
      v54 = *v50;
      if ( *v50 == 91 )
        goto LABEL_159;
      if ( v54 <= 91 )
      {
        if ( v54 <= 46 )
        {
          if ( v54 <= 35 )
            goto LABEL_142;
          if ( ((1LL << v54) & 0x441000000000LL) != 0 )
            goto LABEL_159;
          if ( ((1LL << v54) & 0x90000000000LL) == 0 )
            goto LABEL_142;
LABEL_215:
          if ( v123 == 1 )
            goto LABEL_159;
          goto LABEL_142;
        }
        if ( v54 == 63 )
          goto LABEL_215;
LABEL_142:
        if ( byte_2B687 )
        {
          v55 = (unsigned char)*v50;
          if ( *(int *)&byte_2B180[4 * v55 + 260] == -1 )
            v56 = sub_7D80(v50, v51, &v137);
          else
            v56 = byte_2B6A0[v55];
        }
        else
        {
          v56 = sub_E540(v50, v51, &v137);
        }
        if ( v56 < 0 )
          goto LABEL_159;
        v57 = v50;
        v50 += v56;
        v51 -= v56;
        v53 = mempcpy(v53, v57, v56);
        continue;
      }
      if ( v54 != 92 )
      {
        if ( v54 == 94 )
          goto LABEL_159;
        if ( (unsigned char)(v54 - 123) > 1u )
          goto LABEL_142;
        goto LABEL_215;
      }
      if ( v51 <= 1 )
        goto LABEL_142;
      v83 = v50[1];
      if ( v83 <= 60 )
      {
        if ( v83 <= 9 )
          goto LABEL_223;
        if ( ((1LL << v83) & 0x13FE008000000400LL) != 0 )
          goto LABEL_159;
        if ( ((1LL << v83) & 0xB0000000000LL) == 0 )
          goto LABEL_223;
      }
      else
      {
        v84 = v83 - 62;
        if ( v84 > 0x3Eu )
          goto LABEL_223;
        if ( ((1LL << v84) & 0x220001402200011LL) != 0 )
          goto LABEL_159;
        if ( ((1LL << v84) & 0x6000000000000002LL) == 0 )
        {
LABEL_223:
          ++v50;
          --v51;
          goto LABEL_142;
        }
      }
      if ( v123 != 1 )
      {
        v121 = 0;
        goto LABEL_159;
      }
      goto LABEL_223;
    }
    break;
  }
  v121 = 2;
  if ( v125 != (long long *)((char *)v53 - v52) )
  {
    v131 = (char *)v53 - v52;
    *(char *)mempcpy(src, v52, (char *)v53 - v52) = 10;
  }
LABEL_159:
  free(v52);
LABEL_160:
  v58 = &aGrep_0[32 * v121];
  qword_2B878 = (long long (*)(long long, long long, long long, long long, long long))*((long long *)v58 + 3);
  qword_2B870 = (*((long long (**)(char *, long long, long long, bool))v58 + 2))(
                  src,
                  v131,
                  *((int *)v58 + 3),
                  (dword_2B900 | (unsigned char)byte_2B8FD) != 0);
  s = 0;
  v139[0] = byte_2B684;
  v139[1] = 0;
  byte_2B820 = (qword_2B878(qword_2B870, v139, 1, v134, 0) == 0) == byte_2B802;
  v59 = a1 - optind;
  if ( v127 || v59 > 1 )
    v60 = v127 >= 0;
  else
    v60 = -(dword_2B018 == 3);
  dword_2B808 = v60;
  v61 = sysconf(30);
  if ( v61 - 1 > 0x3FFFFFFFFFFFFFFALL )
    abort();
  qword_2B838 = v61;
  v62 = 98304;
  if ( 0x18000 % v61 )
    v62 = v61 + 98304 - 0x18000 % v61;
  nbytes = v61 + v62 + 8;
  dest = (void *)sub_1EF30(v61 + v62 + 8);
  if ( (dword_2B014 & 2) != 0 && !dword_2B884 )
    dword_2B884 = 1;
  if ( v59 <= 0 )
  {
    if ( dword_2B018 == 3 && v126 > 0 )
    {
      byte_2B88A = 1;
      v63 = (char **)&off_29F90;
    }
    else
    {
      v63 = &off_29F80;
    }
  }
  else
  {
    v63 = &a2[optind];
  }
  v64 = *v63;
  v65 = 1;
  do
  {
    while ( 1 )
    {
      ++v63;
      if ( strcmp(v64, "-") )
        break;
      domainname = (char *)qword_2B7D0;
      v66 = sub_9130(0, 1);
      v64 = *v63;
      v65 &= v66;
      if ( !*v63 )
        goto LABEL_175;
    }
    domainname = (char *)v64;
    v67 = sub_A0F0(4294967196LL, v64, 1, 1);
    v64 = *v63;
    v65 &= v67;
  }
  while ( *v63 );
LABEL_175:
  if ( byte_2B889 )
    return 2;
  else
    return (unsigned char)v65;
}



// Function: regex_compile @ 0x6840
long long regex_compile(
        long long a1,
        const char *a2,
        size_t a3,
        long long a4,
        long long a5,
        reg_syntax_t a6,
        unsigned int a7)
{
  const char *v11; // r12
  __m128i *v13; // rbx
  const char *v14; // rcx
  ptrdiff_t v15; // [rsp+18h] [rbp-90h] BYREF
  struct re_pattern_buffer buffer; // [rsp+20h] [rbp-88h] BYREF
  unsigned long long v17; // [rsp+68h] [rbp-40h]

  v17 = __readfsqword(0x28u);
  buffer.buffer = 0;
  buffer.allocated = 0;
  if ( !byte_2B687 && !(char)a7 )
  {
    buffer.translate = 0;
    buffer.fastmap = (char *)sub_1EF30(256);
LABEL_8:
    re_set_syntax(a6);
    v11 = re_compile_pattern(a2, a3, &buffer);
    if ( !v11 )
    {
      v13 = (__m128i *)(*(long long *)(a1 + 16) + (a4 << 6));
      *v13 = _mm_load_si128((const __m128i *)&buffer);
      v13[1] = _mm_load_si128((const __m128i *)&buffer.used);
      v13[2] = _mm_load_si128((const __m128i *)&buffer.fastmap);
      v13[3] = _mm_load_si128((const __m128i *)&buffer.re_nsub);
      return 1;
    }
    goto LABEL_5;
  }
  buffer.fastmap = 0;
  buffer.translate = 0;
  if ( !(char)a7 )
    goto LABEL_8;
  re_set_syntax(a6 | 0x2000000);
  v11 = re_compile_pattern(a2, a3, &buffer);
  if ( !v11 )
  {
    regfree(&buffer);
    return a7;
  }
LABEL_5:
  free(buffer.fastmap);
  if ( a5 == -1 || (v14 = (const char *)sub_9080(a5, &v15), !*v14) )
  {
    error(0, 0, "%s", v11);
    return 0;
  }
  else
  {
    error(0, 0, "%s:%td: %s", v14, v15, v11);
    return 0;
  }
}



// Function: dfaerror @ 0x6a10
long long dfaerror(const char *a1)
{
  error(2, 0, "%s", a1);
  return sub_6A40();
}



// Function: dfawarn @ 0x6a40
void dfawarn(long long a1)
{
  char *v1; // rax

  v1 = dcgettext(0, "warning: %s", 5);
  error(0, 0, v1, a1);
}



// Function: GEAcompile @ 0x6a70
long long GEAcompile(char *a1, size_t a2, reg_syntax_t a3, char a4)
{
  long long v5; // r12
  long long v6; // rax
  bool v7; // zf
  char *v8; // rbp
  char v9; // bl
  long long v10; // rax
  int v11; // r13d
  long long v12; // r14
  char *v13; // rbx
  char *i; // rdi
  char *v15; // rax
  char v16; // bl
  char v17; // al
  char *v18; // rbp
  char *v19; // r13
  long long v20; // rax
  long long v21; // r14
  size_t v22; // rax
  char *v24; // rax
  size_t v25; // rax
  size_t v26; // rbx
  const char *v27; // rsi
  char *v28; // rax
  size_t v29; // rbx
  char *v30; // rax
  char v31; // cl
  void *v32; // r14
  long long v33; // rdi
  char *v34; // rax
  size_t v35; // rax
  size_t v36; // rax
  size_t v37; // rax
  unsigned int v38; // [rsp+8h] [rbp-B0h]
  size_t v39; // [rsp+8h] [rbp-B0h]
  char v40; // [rsp+10h] [rbp-A8h]
  size_t v41; // [rsp+10h] [rbp-A8h]
  char *v42; // [rsp+18h] [rbp-A0h]
  char v43; // [rsp+18h] [rbp-A0h]
  char *src; // [rsp+28h] [rbp-90h]
  size_t v45; // [rsp+30h] [rbp-88h]
  char *ptr; // [rsp+38h] [rbp-80h]
  unsigned long long v47; // [rsp+40h] [rbp-78h]
  char *s; // [rsp+48h] [rbp-70h]
  size_t n; // [rsp+50h] [rbp-68h]

  s = a1;
  v5 = sub_1F2F0(1, 72);
  v6 = sub_15A90();
  v7 = byte_2B687 == 0;
  *(long long *)(v5 + 8) = v6;
  if ( !v7 )
    a3 |= 0x400000uLL;
  sub_15FA0(v6, byte_2B180, a3, a3 & 0x10 | (2 * (unsigned int)(byte_2B684 == 0)) | 0x2C);
  v8 = a1;
  v47 = (unsigned long long)&a1[a2];
  v9 = byte_2B182 | byte_2B180[0] ^ 1;
  v10 = sub_1EF10(64);
  v40 = 0;
  *(long long *)(v5 + 24) = 0;
  *(long long *)(v5 + 16) = v10 + 64;
  v11 = v9 == 0 ? 128 : 92;
  v45 = 0;
  ptr = 0;
  src = a1;
  v12 = 0;
  do
  {
    v42 = (char *)rawmemchr(v8, 10);
    if ( v42 - v8 - 1 >= 0 )
    {
      v13 = v42 - 1;
      for ( i = v8; ; i = v15 + 1 )
      {
        v15 = (char *)memchr(i, 92, v13 - i);
        if ( !v15 )
          break;
        if ( (unsigned char)(v15[1] - 49) <= 8u )
        {
          if ( v8 <= src )
          {
            v38 = 0;
            v16 = 1;
          }
          else
          {
            v29 = v8 - src;
            if ( (long long)(v8 - src + v45) > 0 )
              ptr = (char *)sub_1F1F0(ptr);
            memcpy(&ptr[v45], src, v29);
            v45 += v29;
            v16 = 1;
            v38 = 0;
          }
          goto LABEL_12;
        }
        if ( v15[1] == v11 && ++v15 == v13 )
          break;
      }
    }
    v38 = 1;
    v16 = 0;
LABEL_12:
    if ( *(long long *)(v5 + 24) - 1LL + 2 > 0 )
      *(long long *)(v5 + 16) = sub_1F1F0((void *)(*(long long *)(v5 + 16) - 64LL)) + 64;
    re_set_syntax(a3);
    v7 = (unsigned char)sub_6840(v5, v8, v42 - v8, *(long long *)(v5 + 24), v12, a3, v38) == 0;
    v17 = v40;
    if ( v7 )
      v17 = 1;
    v8 = v42 + 1;
    ++v12;
    v40 = v17;
    if ( v16 )
    {
      ++*(long long *)(v5 + 24);
      src = v42 + 1;
    }
  }
  while ( v47 >= (unsigned long long)v8 );
  if ( v17 )
    exit(2);
  if ( (unsigned long long)src <= v47 )
  {
    if ( src > s )
    {
      ptr = (char *)sub_1EFA0(ptr, v45 + v47 - (long long)src);
      memcpy(&ptr[v45], src, v47 - (long long)src);
      v45 += v47 - (long long)src;
    }
    else
    {
      v45 = a2;
      ptr = s;
    }
  }
  if ( byte_2B686 || (v18 = 0, byte_2B685) )
  {
    v24 = (char *)sub_1EF30(a2 + 45);
    v18 = v24;
    if ( byte_2B685 )
    {
      if ( (a3 & 0x2000) != 0 )
      {
        strcpy(v24, "^(");
        v25 = strlen(v24);
        v26 = a2 + v25;
        memcpy(&v18[v25], s, a2);
        v27 = ")$";
      }
      else
      {
        *(int *)v24 = 2645086;
        v36 = strlen(v24);
        v26 = a2 + v36;
        memcpy(&v18[v36], s, a2);
        v27 = "\\)$";
      }
    }
    else if ( (a3 & 0x2000) != 0 )
    {
      *(__m128i *)v24 = _mm_load_si128((const __m128i *)&xmmword_21060);
      strcpy(v24 + 16, ")(");
      v37 = strlen(v24);
      v26 = a2 + v37;
      memcpy(&v18[v37], s, a2);
      v27 = ")([^[:alnum:]_]|$)";
    }
    else
    {
      *(__m128i *)v24 = _mm_load_si128((const __m128i *)&xmmword_21080);
      strcpy(v24 + 16, "_]\\)\\(");
      v35 = strlen(v24);
      v26 = a2 + v35;
      memcpy(&v18[v35], s, a2);
      v27 = "\\)\\([^[:alnum:]_]\\|$\\)";
    }
    v28 = strcpy(&v18[v26], v27);
    s = v18;
    n = v26 + strlen(v28);
    sub_14CF0(v18, n, *(long long *)(v5 + 8));
  }
  else
  {
    sub_14CF0(s, a2, *(long long *)(v5 + 8));
  }
  v19 = (char *)sub_15020(*(long long *)(v5 + 8));
  if ( v19 )
  {
    v20 = sub_E5C0(0);
    v7 = *v19 == 0;
    *(long long *)v5 = v20;
    v21 = v20;
    if ( v7 )
    {
      v22 = strlen(v19 + 3);
      sub_C9D0(v21, v19 + 3, v22);
    }
    else
    {
      ++*(long long *)(v5 + 56);
      v41 = strlen(v19 + 3);
      v39 = (unsigned char)v19[1] + v41 + (unsigned char)v19[2];
      v30 = (char *)sub_1EF30(v39);
      v31 = byte_2B684;
      v32 = v30;
      *v30 = byte_2B684;
      v33 = (unsigned char)v19[1];
      v43 = v31;
      *(char *)(v5 + 64) |= v33;
      v34 = memcpy(&v30[v33], v19 + 3, v41);
      if ( v19[2] )
        v34[v41] = v43;
      sub_C9D0(*(long long *)v5, v32, v39);
      free(v32);
    }
    sub_CEC0(*(struct obstack **)v5);
    j_free(v19);
  }
  sub_15AA0(0, 0, *(long long *)(v5 + 8), 1);
  if ( ptr )
  {
    if ( a4 || !(unsigned char)sub_14E30(*(long long *)(v5 + 8)) )
    {
      *(long long *)(v5 + 16) -= 64LL;
      ++*(long long *)(v5 + 24);
      if ( !(unsigned char)sub_6840(v5, ptr, v45, 0, -1, a3, 0) )
        abort();
    }
    if ( s != ptr )
      free(ptr);
  }
  free(v18);
  return v5;
}



// Function: EGexecute @ 0x7110
long long EGexecute(long long *a1, char *a2, long long a3, signed long long *a4, char *a5)
{
  long long *v5; // r14
  char *v6; // rbx
  long long v7; // rdi
  long long v8; // rdi
  char *v9; // rbx
  long long v10; // rax
  long long v11; // r12
  char *v12; // r13
  char *v13; // rax
  char *v14; // r11
  long long v15; // rbp
  char *v16; // r15
  long long v17; // rax
  char *v18; // rax
  long long v19; // rax
  const void *v20; // r12
  char *v21; // rax
  char *v22; // rsi
  long long v23; // rax
  const void *v24; // rbp
  struct re_registers *v25; // r12
  long long *v26; // rax
  const char *v27; // r14
  long long v28; // rbx
  int v29; // esi
  long long *v30; // r15
  int v31; // r13d
  const char *v32; // rbx
  const char *v33; // rbp
  long long v34; // r14
  long long v35; // rdi
  int v36; // eax
  long long v37; // rbp
  long long v38; // rdi
  int v39; // eax
  const char *v40; // r9
  long long v41; // rcx
  bool v42; // zf
  long long v44; // rdi
  int v45; // eax
  char *v46; // rax
  int c[2]; // [rsp+0h] [rbp-F8h]
  int ca[2]; // [rsp+0h] [rbp-F8h]
  char *string; // [rsp+18h] [rbp-E0h]
  long long v51; // [rsp+20h] [rbp-D8h]
  char *v52; // [rsp+28h] [rbp-D0h]
  long long v53; // [rsp+30h] [rbp-C8h]
  long long v54; // [rsp+38h] [rbp-C0h]
  char *v55; // [rsp+40h] [rbp-B8h]
  char *v56; // [rsp+48h] [rbp-B0h]
  char *v57; // [rsp+50h] [rbp-A8h]
  signed long long v58; // [rsp+58h] [rbp-A0h]
  long long v60; // [rsp+68h] [rbp-90h]
  char v61; // [rsp+76h] [rbp-82h]
  char v62; // [rsp+77h] [rbp-81h]
  char v64; // [rsp+8Fh] [rbp-69h] BYREF
  char *v65; // [rsp+90h] [rbp-68h] BYREF
  long long v66; // [rsp+98h] [rbp-60h] BYREF
  long long v67[11]; // [rsp+A0h] [rbp-58h] BYREF

  v5 = a1;
  v6 = &a2[a3];
  v7 = a1[1];
  v67[3] = __readfsqword(0x28u);
  v61 = byte_2B684;
  v60 = sub_14E10(v7);
  v65 = a2;
  v62 = sub_14E20(v7);
  v57 = v6;
  if ( a2 >= v6 )
    return -1;
  string = a2;
  while ( 1 )
  {
    while ( 1 )
    {
      if ( a5 )
      {
        v16 = string;
        v56 = a5;
        string = v57;
        goto LABEL_28;
      }
      v8 = *v5;
      v64 = 0;
      v66 = 0;
      v9 = v57;
      if ( v8 )
        break;
LABEL_15:
      if ( !v60 )
      {
        v22 = string;
        string = v9;
        v16 = v22;
        goto LABEL_21;
      }
      v19 = sub_14E00(v60, string, v9, 0, &v66, 0);
      v20 = (const void *)v19;
      if ( v19 && v9 != (char *)v19 )
      {
        if ( v66 )
          string = (char *)memrchr(a2, v61, v19 - (long long)a2) + 1;
        v21 = (char *)rawmemchr(v20, v61);
        v22 = string;
        v66 = 0;
        string = v21 + 1;
        v16 = v22;
        goto LABEL_21;
      }
      string = v9;
      if ( v9 >= v57 )
        return -1;
    }
    v10 = sub_E3E0(v8, &string[-*((unsigned char *)v5 + 64)], *((unsigned char *)v5 + 64) + v57 - string, v67, 1);
    v11 = v10;
    if ( v10 < 0 )
      return v11;
    v12 = &string[v10];
    v13 = (char *)memrchr(a2, v61, &string[v10] - a2);
    v14 = v13 + 1;
    if ( !v13 )
      v14 = a2;
    v15 = v5[7];
    v16 = v14;
    if ( v62 == 1 && v67[0] >= v15 )
    {
      v17 = v12 - v14;
      if ( v12 - v14 < 16 )
        v17 = 16;
      if ( v17 < v11 >> 2 )
      {
        string = v14;
        v9 = (char *)rawmemchr(v12, v61) + 1;
      }
      else if ( v17 >= (v57 - string) >> 2 )
      {
        string = v14;
        v9 = v57;
      }
      else
      {
        v18 = (char *)rawmemchr(&string[4 * v17], v61);
        string = v16;
        v9 = v18 + 1;
      }
      goto LABEL_15;
    }
    *(long long *)ca = v67[0];
    v46 = (char *)rawmemchr(v12, v61) + 1;
    string = v46;
    if ( *(long long *)ca >= v15 )
    {
      string = v16;
      v9 = v46;
      goto LABEL_15;
    }
    if ( byte_2B180[0] != 1 || byte_2B182 )
      goto LABEL_84;
    if ( v65 < v16 )
      v65 = v16;
    if ( !sub_E640(&v65, 0, v12, v57) )
    {
LABEL_84:
      v58 = string - v16;
      goto LABEL_64;
    }
    v22 = v65;
LABEL_21:
    v23 = sub_14E00(v5[1], v22, string, 0, &v66, &v64);
    v24 = (const void *)v23;
    if ( v23 )
    {
      if ( string != (char *)v23 )
        break;
    }
LABEL_51:
    if ( string >= v57 )
      return -1;
  }
  if ( v66 )
    v16 = (char *)memrchr(a2, v61, v23 - (long long)a2) + 1;
  string = (char *)rawmemchr(v24, v61) + 1;
  if ( !v64 )
    goto LABEL_84;
  v56 = v16;
LABEL_28:
  v58 = string - v16;
  if ( string - v16 > 0x80000000LL )
    goto LABEL_83;
  if ( v5[3] <= 0 )
    goto LABEL_51;
  v25 = (struct re_registers *)(v5 + 4);
  v51 = 0;
  *(long long *)c = string;
  v52 = string - 1;
  v26 = v5;
  v27 = v16;
  v55 = (char *)(string - v56);
  v28 = 0;
  v29 = (int)v56 - (int)v16;
  v30 = v26;
  v31 = v58 - 1;
  do
  {
    while ( 1 )
    {
      v37 = v28 << 6;
      v38 = (v28 << 6) + v30[2];
      *(char *)(v38 + 56) = *(char *)(v38 + 56) & 0x3F | ((byte_2B684 == 10) << 7);
      v39 = re_search((struct re_pattern_buffer *)v38, v27, v31, v29, (int)string - (int)v56 - 1, v25);
      if ( v39 < -1 )
        goto LABEL_83;
      if ( v39 == -1 )
        goto LABEL_42;
      v40 = &v27[v39];
      if ( *(long long *)c < (unsigned long long)v40 )
        goto LABEL_42;
      v41 = *(int *)v30[6] - v39;
      if ( !a5 )
        break;
      if ( !byte_2B686 )
        goto LABEL_54;
      if ( !byte_2B685 )
        goto LABEL_33;
      if ( v55 - 1 == (char *)v41 )
      {
        v41 = string - v56;
        v40 = v56;
        v42 = *(long long *)c == (long long)v56;
        if ( *(long long *)c <= (unsigned long long)v56 )
        {
LABEL_55:
          if ( v42 )
          {
            if ( v51 >= v41 )
              v41 = v51;
            v51 = v41;
          }
          goto LABEL_42;
        }
        goto LABEL_61;
      }
LABEL_42:
      if ( v30[3] <= ++v28 )
        goto LABEL_50;
    }
    if ( !byte_2B685 )
    {
      if ( !byte_2B686 )
      {
LABEL_63:
        v16 = (char *)v27;
        goto LABEL_64;
      }
LABEL_33:
      v53 = v28;
      v32 = &v27[v39];
      v54 = v37;
      v33 = v27;
      v34 = *(int *)v30[6] - v39;
      while ( sub_E820(&v32[v34], v52) || sub_E830(v33, v32, v52) )
      {
        if ( v34 <= 0 )
          goto LABEL_37;
        v44 = v30[2] + v54;
        *(char *)(v44 + 56) |= 0x40u;
        v45 = re_match(
                (struct re_pattern_buffer *)v44,
                v33,
                (int)v32 + v34 - 1 - (int)v56,
                (int)v32 - (int)v33,
                v25);
        if ( v45 < -1 )
          goto LABEL_83;
        v34 = v45;
        if ( v45 <= 0 )
        {
LABEL_37:
          if ( v52 == v32 )
            goto LABEL_41;
          v35 = v30[2] + v54;
          *(char *)(v35 + 56) &= ~0x40u;
          v36 = re_search(
                  (struct re_pattern_buffer *)v35,
                  v33,
                  v31,
                  (int)v32 + 1 - (int)v33,
                  (int)string - ((int)v32 + 1) - 1,
                  v25);
          if ( v36 < 0 )
          {
            v28 = v53;
            v27 = v33;
            if ( v36 != -1 )
LABEL_83:
              sub_1F410();
            goto LABEL_42;
          }
          v34 = *(int *)v30[6] - v36;
          v32 = &v33[v36];
        }
        if ( *(long long *)c < (unsigned long long)v32 )
        {
LABEL_41:
          v28 = v53;
          v27 = v33;
          goto LABEL_42;
        }
      }
      v41 = v34;
      v40 = v32;
      v27 = v33;
      v28 = v53;
      if ( !a5 )
        goto LABEL_63;
LABEL_54:
      v42 = *(long long *)c == (long long)v40;
      if ( *(long long *)c <= (unsigned long long)v40 )
        goto LABEL_55;
LABEL_61:
      v51 = v41;
      *(long long *)c = v40;
      goto LABEL_42;
    }
    if ( (char *)v41 == v55 - 1 )
      goto LABEL_63;
    ++v28;
  }
  while ( v30[3] > v28 );
LABEL_50:
  v5 = v30;
  if ( (unsigned long long)string <= *(long long *)c )
    goto LABEL_51;
  v16 = *(char **)c;
  v58 = v51;
LABEL_64:
  *a4 = v58;
  return v16 - a2;
}



// Function: hash_pattern @ 0x7850
unsigned long long hash_pattern(long long a1, unsigned long long a2)
{
  unsigned char *v2; // rcx
  unsigned long long v3; // rax
  long long i; // rdx

  v2 = (unsigned char *)(qword_2B8D8 + a1 - 1);
  v3 = 0x3938A0565D85CB03LL;
  for ( i = *v2; (char)i != 10; i = *v2 )
  {
    ++v2;
    v3 = i ^ (33 * v3);
  }
  return v3 % a2;
}



// Function: compare_patterns @ 0x78a0
long long compare_patterns(long long a1, long long a2)
{
  char *v2; // rcx
  char *v3; // rsi
  char v4; // dl
  long long v5; // rax

  v2 = (char *)(qword_2B8D8 + a1 - 1);
  v3 = (char *)(qword_2B8D8 + a2 - 1);
  v4 = *v2;
  if ( *v2 != *v3 )
    return 0;
  v5 = 1;
  while ( v4 != 10 )
  {
    v4 = v2[v5++];
    if ( v4 != v3[v5 - 1] )
      return 0;
  }
  return 1;
}



// Function: color_cap_mt_fct @ 0x78f0
char *color_cap_mt_fct()
{
  off_2B060[0] = off_2B068[0];
  return off_2B068[0];
}



// Function: color_cap_rv_fct @ 0x7910
void color_cap_rv_fct()
{
  dword_2B900 = -1;
}



// Function: color_cap_ne_fct @ 0x7920
const char *color_cap_ne_fct()
{
  off_2B028 = "\x1B[%sm";
  off_2B020[0] = "\x1B[m";
  return "\x1B[m";
}



// Function: setmatcher @ 0x7950
long long setmatcher(char *s1, int a2)
{
  long long v2; // rbx
  unsigned int v3; // r12d
  char *v4; // rax
  long long result; // rax
  char *v6; // rax

  v2 = 0;
  while ( 1 )
  {
    v3 = v2;
    if ( !strcmp(s1, &aGrep_0[32 * v2]) )
      break;
    if ( ++v2 == 7 )
    {
      v4 = dcgettext(0, "invalid matcher %s", 5);
      error(2, 0, v4, s1);
      break;
    }
  }
  if ( a2 < 0 || (int)v2 == a2 )
    return v3;
  v6 = dcgettext(0, "conflicting matchers specified", 5);
  error(2, 0, v6);
  result = (unsigned int)errnum;
  if ( !errnum )
    return sub_F7E0();
  return result;
}



// Function: update_patterns @ 0x7a20
char *update_patterns(long long a1, long long a2, long long a3, long long a4)
{
  char *v4; // r13
  char *v5; // rbx
  int v6; // r14d
  long long v7; // rbp
  long long v8; // rax
  char *v9; // r15
  const void *v10; // rsi
  int v11; // eax
  long long v12; // rax
  char *v13; // rdi
  long long *v14; // rdx
  int v17; // [rsp+Ch] [rbp-4Ch]
  unsigned long long v18; // [rsp+10h] [rbp-48h]

  v4 = (char *)(a1 + a2);
  v18 = a1 + a3;
  if ( a1 + a2 >= (unsigned long long)(a1 + a3) )
    return (char *)a2;
  v5 = (char *)(a1 + a2);
  v6 = 0;
  v7 = 1;
  do
  {
    v10 = v4;
    v4 = (char *)rawmemchr(v4, 10) + 1;
    memmove(v5, v10, v4 - (char *)v10);
    v9 = &v5[-a1];
    v17 = v6;
    v11 = sub_1A700(qword_2B8C8, &v5[-a1 + 1], 0);
    v6 = v11;
    if ( v11 )
    {
      if ( v11 < 0 )
        sub_1F410();
      v5 += v4 - (char *)v10;
      if ( v17 )
      {
        v8 = qword_2B8D0;
      }
      else
      {
        v12 = qword_2B8E0;
        v13 = (char *)qword_2B8F0;
        if ( qword_2B8E0 == qword_2B8E8 )
        {
          qword_2B8F0 = (void *)sub_1F1F0(qword_2B8F0);
          v13 = (char *)qword_2B8F0;
          v12 = qword_2B8E0;
        }
        qword_2B8E0 = v12 + 1;
        v14 = (long long *)&v13[24 * v12];
        v8 = qword_2B8D0;
        v14[1] = a4;
        *v14 = v8;
        v14[2] = v7;
      }
      v9 = &v5[-a1];
      qword_2B8D0 = v8 + 1;
    }
    ++v7;
  }
  while ( (unsigned long long)v4 < v18 );
  return v9;
}



// Function: printf_errno @ 0x7b80
unsigned long long printf_errno(long long a1, ...)
{
  gcc_va_list va; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v3; // [rsp+18h] [rbp-C0h]

  va_start(va, a1);
  v3 = __readfsqword(0x28u);
  if ( (int)((long long (*)(FILE *, long long, long long, __va_list_tag *))__vfprintf_chk)(stdout, 1, a1, va) < 0 )
    errnum = *__errno_location();
  return v3 - __readfsqword(0x28u);
}



// Function: nlscan @ 0x7c90
void nlscan(unsigned long long a1)
{
  const void *v2; // rdi
  long long v3; // r13
  unsigned int v4; // r12d
  long long v5; // rbx
  char *v6; // rax
  bool v7; // of

  v2 = (const void *)qword_2B7C0;
  v3 = qword_2B818;
  if ( qword_2B7C0 >= a1 )
    goto LABEL_6;
  v4 = byte_2B684;
  v5 = 0;
  do
  {
    v6 = (char *)memchr(v2, v4, a1 - (long long)v2);
    if ( !v6 )
      break;
    v2 = v6 + 1;
    ++v5;
  }
  while ( a1 > (unsigned long long)(v6 + 1) );
  v7 = __OFADD__(v3, v5);
  v3 += v5;
  if ( v7 )
  {
    sub_7C60();
    sub_7D10(v2, v4);
  }
  else
  {
LABEL_6:
    qword_2B818 = v3;
    qword_2B7C0 = a1;
  }
}



// Function: print_sep @ 0x7ec0
long long print_sep(char a1)
{
  long long result; // rax

  if ( dword_2B900 && *off_2B040[0] )
    sub_1FAA0(off_2B028);
  sub_7E70((unsigned int)a1);
  result = (unsigned int)dword_2B900;
  if ( dword_2B900 )
  {
    result = (long long)off_2B040[0];
    if ( *off_2B040[0] )
      return sub_1FAC0(off_2B020[0]);
  }
  return result;
}



// Function: print_offset @ 0x7f20
long long print_offset(long long a1, char *a2)
{
  long long v2; // rdx
  long long result; // rax

  v2 = a1;
  if ( dword_2B900 && *a2 )
  {
    sub_1FAA0(off_2B028);
    v2 = a1;
  }
  sub_7B80((long long)"%*ld", dword_2B8F8, v2);
  result = (unsigned int)dword_2B900;
  if ( dword_2B900 )
  {
    if ( *a2 )
      return sub_1FAC0(off_2B020[0]);
  }
  return result;
}



// Function: print_filename @ 0x7fa0
long long print_filename()
{
  const char *v0; // rdi
  long long result; // rax

  if ( dword_2B900 && *off_2B058[0] )
  {
    sub_1FAA0(off_2B028);
    v0 = domainname;
    if ( domainname )
      goto LABEL_4;
  }
  else
  {
    v0 = domainname;
    if ( domainname )
      goto LABEL_4;
  }
  domainname = dcgettext(v0, "(standard input)", 5);
  v0 = domainname;
LABEL_4:
  if ( fputs_unlocked(v0, stdout) < 0 )
    errnum = *__errno_location();
  result = (unsigned int)dword_2B900;
  if ( dword_2B900 )
  {
    result = (long long)off_2B058[0];
    if ( *off_2B058[0] )
      return sub_1FAC0(off_2B020[0]);
  }
  return result;
}



// Function: print_line_head @ 0x8050
long long print_line_head(unsigned long long a1, long long a2, unsigned long long a3, char a4)
{
  long long v6; // rdi
  unsigned long long v7; // rbx
  long long result; // rax
  char *v9; // r15
  long long v10; // rdx
  char *v11; // r13
  long long v12; // rax
  bool v14; // [rsp+0h] [rbp-58h]
  char v15; // [rsp+Fh] [rbp-49h]
  long long v16[9]; // [rsp+10h] [rbp-48h] BYREF

  v16[1] = __readfsqword(0x28u);
  if ( dword_2B810 != 1 )
  {
    v9 = (char *)(a1 + a2);
    v10 = qword_2B868;
    v15 = *(char *)(a1 + a2);
    if ( qword_2B868 )
    {
      *v9 = -1;
      v11 = (char *)a1;
      v16[0] = 0;
      while ( 1 )
      {
        if ( ((unsigned char)v11 & 7) != 0 )
        {
          if ( ((unsigned char)v10 & *v11) != 0 )
            goto LABEL_23;
          ++v11;
        }
        else
        {
          while ( (*(long long *)v11 & v10) == 0 )
            v11 += 8;
          while ( ((unsigned char)v10 & *v11) == 0 )
            ++v11;
LABEL_23:
          if ( v9 <= v11 )
          {
            *v9 = v15;
            break;
          }
          v12 = sub_1AC90(v11, v9 - v11, v16);
          if ( v12 < 0 )
          {
            byte_2B888 = 1;
            *v9 = v15;
            return 0;
          }
          v10 = qword_2B868;
          v11 += v12;
        }
      }
    }
  }
  if ( dword_2B808 )
  {
    sub_7FA0();
    if ( dword_2B804 )
      sub_7EC0(a4);
    else
      sub_7E70(0);
  }
  if ( byte_2B801 )
  {
    v6 = qword_2B818;
    if ( qword_2B7C0 < a3 )
    {
      sub_7C90(a1);
      v6 = qword_2B818 + 1;
      if ( __OFADD__(1, qword_2B818) )
        goto LABEL_33;
      ++qword_2B818;
      qword_2B7C0 = a3;
    }
    sub_7F20(v6, off_2B050[0]);
    sub_7EC0(a4);
  }
  if ( !byte_2B800 )
    goto LABEL_11;
  v7 = a1 - (long long)qword_2B848;
  v6 = qword_2B7C8 + v7;
  if ( __OFADD__(qword_2B7C8, v7) )
  {
LABEL_33:
    sub_7C60(v6);
    return sub_82A0();
  }
  sub_7F20(v6, off_2B048[0]);
  sub_7EC0(a4);
LABEL_11:
  result = (unsigned char)byte_2B8FC;
  if ( !byte_2B8FC )
    return 1;
  if ( a2 != 0 && (dword_2B808 | (unsigned char)(byte_2B800 | byte_2B801)) != 0 )
  {
    v14 = a2 != 0 && (dword_2B808 | (unsigned char)(byte_2B800 | byte_2B801)) != 0;
    sub_7E70(9);
    return v14;
  }
  return result;
}



// Function: prline @ 0x82a0
long long prline(unsigned long long a1, char *a2, char a3)
{
  unsigned long long v4; // rbp
  char v6; // al
  char v7; // di
  char v8; // cl
  int v9; // ebp
  char *v11; // r9
  char *v12; // rdx
  char v13; // al
  char *v14; // r12
  long long v15; // rax
  char v16; // r8
  unsigned long long v17; // r13
  unsigned long long v18; // r12
  long long i; // rax
  long long v20; // rax
  unsigned long long v21; // r15
  unsigned long long v22; // rsi
  unsigned long long v23; // rdi
  char *v24; // rax
  char *v25; // [rsp+10h] [rbp-58h]
  char *v26; // [rsp+18h] [rbp-50h]
  long long v27; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v28; // [rsp+28h] [rbp-40h]

  v4 = a1;
  v28 = __readfsqword(0x28u);
  v6 = byte_2B8FD;
  if ( !byte_2B8FD )
  {
    v16 = sub_8050(a1, (long long)&a2[-a1 - 1], (unsigned long long)a2, a3);
    v6 = byte_2B8FD;
    if ( !v16 )
      return v28 - __readfsqword(0x28u);
  }
  v7 = a3 == 58;
  v8 = v6 & (byte_2B802 ^ (a3 == 58));
  if ( dword_2B900 )
  {
    v11 = off_2B068[0];
    v12 = (char *)off_2B038;
    if ( v7 == ((unsigned char)byte_2B802 & (dword_2B900 < 0)) )
      v12 = (char *)off_2B030;
    if ( a3 != 58 )
      v11 = off_2B060[0];
    v26 = v12;
    v25 = v11;
    if ( !v8 )
    {
      if ( *v12 )
      {
        if ( v7 == byte_2B802 )
        {
LABEL_23:
          if ( v6 )
            goto LABEL_7;
          v13 = *v26;
LABEL_68:
          if ( !v13 )
          {
LABEL_69:
            v6 = byte_2B8FD;
            goto LABEL_4;
          }
LABEL_27:
          v14 = a2;
          if ( (unsigned long long)a2 <= v4 )
            goto LABEL_31;
          v15 = 0;
          if ( *(a2 - 1) == byte_2B684 )
          {
            v14 = a2 - 1;
            if ( (unsigned long long)(a2 - 1) <= v4 )
              goto LABEL_31;
            v15 = 1;
          }
          if ( a2[~v15] == 13 )
            v14 = &a2[-v15 - 1];
LABEL_31:
          if ( (long long)&v14[-v4] > 0 )
          {
            sub_1FAA0(off_2B028);
            sub_7E40(v4, &v14[-v4]);
            if ( *v26 )
              sub_1FAC0(off_2B020[0]);
            v6 = byte_2B8FD;
            v4 = (unsigned long long)v14;
            goto LABEL_4;
          }
          goto LABEL_69;
        }
        if ( !*v11 )
          goto LABEL_27;
      }
      else
      {
        if ( !*v11 )
          goto LABEL_4;
        if ( v7 == byte_2B802 )
          goto LABEL_23;
      }
      if ( (unsigned long long)a2 <= v4 )
      {
LABEL_21:
        if ( !v4 )
          return v28 - __readfsqword(0x28u);
        v6 = byte_2B8FD;
        goto LABEL_23;
      }
      goto LABEL_39;
    }
LABEL_38:
    if ( (unsigned long long)a2 <= v4 )
    {
LABEL_70:
      v4 = (unsigned long long)a2;
      goto LABEL_21;
    }
LABEL_39:
    v17 = v4;
    v18 = 0;
    for ( i = qword_2B878(qword_2B870, v4, &a2[-v4], &v27, v4);
          i >= 0;
          i = qword_2B878(qword_2B870, v4, &a2[-v4], &v27, v17) )
    {
      v21 = v4 + i;
      if ( a2 == (char *)(v4 + i) )
        break;
      if ( v27 )
      {
        if ( byte_2B8FD )
        {
          if ( !(unsigned char)sub_8050(v4 + i, v27, (unsigned long long)a2, byte_2B802 == 0 ? 58 : 45) )
            return v28 - __readfsqword(0x28u);
        }
        else
        {
          if ( *v26 )
            sub_1FAA0(off_2B028);
          if ( !v18 )
            v18 = v17;
          v22 = v21 - v18;
          v23 = v18;
          v18 = 0;
          sub_7E40(v23, v22);
        }
        if ( dword_2B900 && *v25 )
          sub_1FAA0(off_2B028);
        sub_7E40(v21, v27);
        if ( dword_2B900 && *v25 )
          sub_1FAC0(off_2B020[0]);
        if ( byte_2B8FD )
          sub_7E70((unsigned int)byte_2B684);
        v20 = v27;
      }
      else
      {
        v20 = 1;
        v27 = 1;
        if ( !v18 )
          v18 = v17;
      }
      v17 = v21 + v20;
      if ( (unsigned long long)a2 <= v21 + v20 )
        break;
    }
    if ( !byte_2B8FD )
    {
      v4 = v17;
      if ( v18 )
      {
        v4 = v18;
        v13 = *v26;
        goto LABEL_68;
      }
      goto LABEL_21;
    }
    goto LABEL_70;
  }
  if ( v8 )
  {
    v25 = 0;
    v26 = 0;
    goto LABEL_38;
  }
LABEL_4:
  if ( v4 < (unsigned long long)a2 && v6 != 1 )
    sub_7E40(v4, &a2[-v4]);
LABEL_7:
  if ( byte_2B7D8 && fflush_unlocked(stdout) )
  {
    v9 = *__errno_location();
    errnum = v9;
  }
  else
  {
    v9 = errnum;
  }
  if ( !v9 )
  {
    s = a2;
    return v28 - __readfsqword(0x28u);
  }
  v24 = dcgettext(0, "write error", 5);
  error(2, v9, v24);
  return sub_8710();
}



// Function: prpending @ 0x8710
void prpending(unsigned long long a1)
{
  char *v1; // rax
  void *v2; // rbp

  if ( !s )
    s = qword_2B848;
  for ( ; qword_2B7A8 > 0; --qword_2B7A8 )
  {
    v2 = s;
    if ( (unsigned long long)s >= a1 )
      break;
    v1 = (char *)rawmemchr(s, byte_2B684);
    sub_82A0((unsigned long long)v2, v1 + 1, 45);
  }
}



// Function: grepbuf @ 0x8790
long long grepbuf(unsigned long long s, unsigned long long a2)
{
  unsigned long long v2; // r13
  long long v3; // rax
  char *v4; // r15
  int v5; // ebx
  char *v6; // r14
  char *v7; // rcx
  long long v8; // rax
  unsigned long long v9; // rcx
  long long v10; // rax
  char v11; // r12
  long long v12; // rdx
  long long v13; // r15
  long long v14; // rbp
  char *v15; // rax
  long long v16; // rax
  unsigned long long v18; // rdi
  const char *v19; // rdi
  int v20; // edi
  char *v21; // [rsp+8h] [rbp-70h]
  char *v22; // [rsp+18h] [rbp-60h]
  long long v23; // [rsp+28h] [rbp-50h]
  long long v24[9]; // [rsp+30h] [rbp-48h] BYREF

  v24[1] = __readfsqword(0x28u);
  v23 = qword_2B7B0;
  if ( s >= a2 )
    return 0;
  v2 = s;
  while ( 1 )
  {
    v3 = qword_2B878(qword_2B870, v2, a2 - v2, v24, 0);
    if ( v3 < 0 )
    {
      if ( !byte_2B802 )
      {
LABEL_72:
        v13 = qword_2B7B0;
        return v23 - v13;
      }
      v24[0] = 0;
      if ( a2 <= v2 )
        return v23 - qword_2B7B0;
      v21 = (char *)a2;
      v9 = a2;
LABEL_21:
      v5 = byte_2B684;
      v6 = (char *)v2;
      if ( !byte_2B803 )
      {
        v4 = (char *)v2;
        v2 = v9;
        goto LABEL_8;
      }
LABEL_22:
      v10 = qword_2B7B0;
      v11 = byte_2B802;
      v2 = v9;
      v12 = qword_2B7B0;
      if ( byte_2B802 )
      {
LABEL_23:
        v13 = v12;
        v14 = 0;
        if ( v21 > v6 )
        {
          while ( v14 < v13 )
          {
            v15 = (char *)rawmemchr(v6, v5) + 1;
            if ( v11 )
            {
              ++v14;
              if ( v21 <= v15 )
                goto LABEL_30;
            }
            else
            {
              v22 = v15;
              sub_82A0((unsigned long long)v6, v15, 58);
              ++v14;
              v15 = v22;
              v11 = byte_2B803;
              v13 = qword_2B7B0;
              if ( v21 <= v22 )
              {
LABEL_30:
                v13 -= v14;
                v6 = v15;
                goto LABEL_31;
              }
            }
            v6 = v15;
          }
          v13 -= v14;
        }
        goto LABEL_31;
      }
      goto LABEL_45;
    }
    v4 = (char *)(v2 + v3);
    v21 = (char *)(v2 + v3 + v24[0]);
    if ( !byte_2B802 )
      break;
    if ( (unsigned long long)v4 > v2 )
    {
      v9 = v2 + v3 + v24[0];
      v21 = (char *)(v2 + v3);
      goto LABEL_21;
    }
    v2 += v3 + v24[0];
LABEL_36:
    if ( a2 <= v2 )
      return v23 - qword_2B7B0;
  }
  if ( (char *)a2 == v4 )
    goto LABEL_72;
  v5 = byte_2B684;
  if ( byte_2B803 )
  {
    v10 = qword_2B7B0;
    v2 = (unsigned long long)v21;
LABEL_45:
    v13 = v10 - 1;
LABEL_46:
    v16 = 0;
    offset = qword_2B830 + v21 - (char *)qword_2B840;
    goto LABEL_34;
  }
  v2 += v3 + v24[0];
LABEL_8:
  if ( qword_2B7A8 > 0 )
  {
    sub_8710((unsigned long long)v4);
    if ( byte_2B803 )
    {
      v6 = v4;
      v9 = v2;
      goto LABEL_22;
    }
  }
  v6 = v4;
  v7 = (char *)::s;
  if ( !::s )
    v7 = qword_2B848;
  v8 = 0;
  if ( qword_2B7F8 > 0 )
  {
    do
    {
      if ( v7 < v6 )
      {
        do
        {
          if ( (char)v5 == *(--v6 - 1) )
            break;
          --v6;
        }
        while ( (char)v5 != *(v6 - 1) );
      }
      ++v8;
    }
    while ( v8 != qword_2B7F8 );
    if ( byte_2B688 )
    {
      if ( ::s != v6 )
      {
        v19 = off_2B070;
        if ( off_2B070 )
        {
LABEL_57:
          if ( dword_2B900 && *off_2B040[0] )
          {
            sub_1FAA0(off_2B028);
            v19 = off_2B070;
          }
          if ( fputs_unlocked(v19, stdout) < 0 )
            errnum = *__errno_location();
          if ( dword_2B900 && *off_2B040[0] )
          {
            sub_1FAC0(off_2B020[0]);
            sub_7E70(10);
          }
          else
          {
            sub_7E70(10);
          }
        }
      }
    }
    while ( v6 < v4 )
    {
      v18 = (unsigned long long)v6;
      v6 = (char *)rawmemchr(v6, (char)v5) + 1;
      sub_82A0(v18, v6, 45);
    }
  }
  else
  {
    if ( (qword_2B7F0 & qword_2B7F8) >= 0 && ::s != v4 )
    {
      if ( byte_2B688 )
      {
        v19 = off_2B070;
        if ( off_2B070 )
        {
          v6 = v4;
          goto LABEL_57;
        }
      }
    }
    v6 = v4;
  }
  v11 = byte_2B803;
  if ( byte_2B802 )
  {
    v12 = qword_2B7B0;
    goto LABEL_23;
  }
  if ( byte_2B803 )
  {
    v13 = qword_2B7B0 - 1;
    goto LABEL_46;
  }
  v6 = v21;
  sub_82A0((unsigned long long)v4, v21, 58);
  v11 = byte_2B803;
  v13 = qword_2B7B0 - 1;
LABEL_31:
  v16 = 0;
  offset = qword_2B830 + v6 - (char *)qword_2B840;
  if ( !v11 )
  {
    v16 = qword_2B7F0;
    if ( qword_2B7F0 < 0 )
      v16 = 0;
  }
LABEL_34:
  qword_2B7A8 = v16;
  byte_2B688 = 1;
  qword_2B7B0 = v13;
  if ( v13 && !byte_2B7A2 )
    goto LABEL_36;
  if ( byte_2B7A1 )
  {
    v20 = 0;
    if ( byte_2B889 )
      v20 = status;
    exit(v20);
  }
  return v23 - v13;
}



// Function: suppressible_error @ 0x8cb0
void suppressible_error(int errnum)
{
  const char *v1; // rcx

  if ( byte_2B904 )
  {
    byte_2B889 = 1;
  }
  else
  {
    v1 = domainname;
    if ( !domainname )
    {
      domainname = dcgettext(0, "(standard input)", 5);
      v1 = domainname;
    }
    error(0, errnum, "%s", v1);
    byte_2B889 = 1;
  }
}



// Function: fillbuf @ 0x8d20
long long fillbuf(long long a1, long long a2)
{
  char *v3; // rbp
  size_t v4; // rax
  long long v5; // r14
  unsigned long long v6; // r13
  long long v7; // r9
  long long v8; // r15
  char *v9; // rax
  int v10; // r15d
  size_t v11; // rbp
  long long v12; // rax
  long long v13; // rcx
  char *v14; // rsi
  __off_t v15; // r14
  long long result; // rax
  char *v17; // rdx
  long long v18; // rbx
  __off_t v19; // rax
  char *v20; // rsi
  unsigned long long v21; // rdx
  char *v22; // rax
  char *v23; // rax
  size_t n; // [rsp+8h] [rbp-40h]
  char *na; // [rsp+8h] [rbp-40h]

  v3 = (char *)dest;
  v4 = nbytes;
  v5 = qword_2B838;
  v6 = (unsigned long long)qword_2B840;
  if ( (char *)dest + nbytes - (char *)qword_2B840 < qword_2B838 + 8 )
  {
    v7 = a1 + 1;
    v8 = ~a1;
    if ( (long long)(qword_2B838 + 8 + qword_2B838 + a1 - nbytes) <= 0 )
    {
      v20 = (char *)qword_2B840 + v8;
      v21 = ((unsigned long long)dest + v7) % qword_2B838;
      if ( v21 )
      {
        v6 = (unsigned long long)dest + qword_2B838 + a1 + 1 - v21;
        memmove((void *)(v6 + v8), v20, a1 + 1);
        v4 = nbytes;
      }
      else
      {
        na = (char *)dest + v7;
        memmove(dest, v20, a1 + 1);
        v4 = nbytes;
        v6 = (unsigned long long)na;
      }
    }
    else
    {
      n = a1 + 1;
      v9 = (char *)sub_1F1F0(0);
      v5 = qword_2B838;
      v3 = v9;
      v6 = (unsigned long long)&v9[a1 + 1];
      if ( v6 % qword_2B838 )
      {
        v6 = (unsigned long long)&v9[qword_2B838 + a1 + 1 - v6 % qword_2B838];
        memmove((void *)(v6 + v8), (char *)qword_2B840 + v8, n);
      }
      else
      {
        memmove(v9, (char *)qword_2B840 + v8, n);
      }
      free(dest);
      dest = v3;
      v4 = nbytes;
    }
  }
  v10 = fd;
  qword_2B848 = (char *)(v6 - a1);
  v11 = (size_t)&v3[v4 - 8 - v6 - (long long)&v3[v4 - 8 - v6] % v5];
  while ( 1 )
  {
    v12 = sub_1DA00(v10, (void *)v6, v11);
    v13 = v12;
    if ( v12 == -1 )
    {
      v14 = (char *)v6;
      result = 0;
LABEL_14:
      *(long long *)v14 = 0;
      qword_2B840 = v14;
      return result;
    }
    v14 = (char *)(v6 + v12);
    v15 = v12 + qword_2B830;
    qword_2B830 += v12;
    result = (unsigned char)byte_2B821 ^ 1u;
    LOBYTE(result) = (v13 == 0) | byte_2B821 ^ 1;
    if ( (char)result )
      goto LABEL_14;
    if ( v6 < (unsigned long long)v14 )
    {
      v17 = (char *)v6;
      while ( !*v17 )
      {
        if ( v14 == ++v17 )
          goto LABEL_15;
      }
      result = 1;
      goto LABEL_14;
    }
LABEL_15:
    v18 = qword_2B818 + v13;
    if ( __OFADD__(qword_2B818, v13) )
      break;
    qword_2B818 += v13;
    v10 = fd;
    if ( !byte_2B880 )
    {
      v19 = lseek(fd, v15, 3);
      if ( v19 >= 0
        || *__errno_location() == 6
        && (*(int *)(a2 + 24) & 0xF000) == 0x8000
        && v15 < *(long long *)(a2 + 48)
        && (v19 = lseek(v10, 0, 2), v19 >= 0) )
      {
        if ( __OFADD__(v18, v19 - v15) )
          goto LABEL_32;
        qword_2B818 = v18 + v19 - v15;
        qword_2B830 = v19;
      }
      else
      {
        byte_2B880 = 1;
      }
    }
  }
  v22 = dcgettext(0, "input is too large to count", 5);
  error(2, 0, v22);
LABEL_32:
  v23 = dcgettext(0, "input is too large to count", 5);
  error(2, 0, v23);
  return sub_9080(2, 0);
}



// Function: pattern_file_name @ 0x9080
long long pattern_file_name(long long a1, long long *a2)
{
  long long *v2; // rcx
  long long v3; // rax
  long long v4; // rdx
  long long *v6; // rcx

  v2 = qword_2B8F0;
  if ( qword_2B8E0 <= 1 )
  {
LABEL_6:
    *a2 = v2[2] + a1 - *v2;
    return v2[1];
  }
  else
  {
    v3 = 24;
    v4 = 1;
    do
    {
      if ( *(long long *)((char *)qword_2B8F0 + v3) > a1 )
      {
        v2 = (char *)qword_2B8F0 + v3 - 24;
        goto LABEL_6;
      }
      ++v4;
      v3 += 24;
    }
    while ( v4 != qword_2B8E0 );
    v6 = (char *)qword_2B8F0 + 24 * v4 - 24;
    *a2 = v6[2] + a1 - *v6;
    return v6[1];
  }
}



// Function: grepdesc @ 0x9130
long long grepdesc(int a1, unsigned char a2)
{
  int v2; // ebp
  char v3; // bl
  char *v4; // rsi
  unsigned long long v5; // rdx
  char v6; // bp
  signed long long v7; // r12
  char v8; // r15
  char *v9; // r13
  const char *v10; // r8
  unsigned long long v11; // r14
  size_t v12; // rax
  __off_t v13; // rsi
  int v14; // edx
  char *v15; // r15
  size_t v16; // rax
  int v17; // edx
  int *v18; // rax
  int v19; // edx
  long long *v20; // rax
  long long v21; // rdi
  int v22; // eax
  int v23; // edx
  char v24; // r12
  char *v25; // rax
  unsigned long long v26; // rdi
  char *v27; // r15
  long long v28; // rax
  long long v29; // rax
  signed long long v30; // rax
  bool v31; // of
  long long v32; // rax
  long long v33; // rdi
  int *v34; // rax
  __off_t v35; // rax
  __off_t v36; // rax
  __off_t v37; // r15
  int *v38; // rax
  __off_t st_size; // rcx
  unsigned int v41; // ebp
  int *v42; // rbp
  long long v43; // rax
  long long v44; // r12
  short v45; // ax
  unsigned char v46; // bl
  char *v47; // rdx
  int v48; // edx
  unsigned int v49; // r15d
  long long v50; // rax
  int v51; // edi
  bool v52; // zf
  int *v53; // rax
  char *v54; // r12
  char *v55; // rax
  int v56; // eax
  int v57; // ecx
  int v58; // r8d
  int v59; // edx
  char *v60; // r12
  char *v61; // rax
  int *v62; // rax
  int *v63; // rbx
  long long v64; // rdi
  int *v65; // rax
  long long v66; // r12
  char *v67; // rax
  char *v68; // rax
  ssize_t v69; // rax
  int *v70; // rax
  ssize_t v71; // rax
  int *v72; // rax
  char *v73; // [rsp+0h] [rbp-1B8h]
  long long v74; // [rsp+8h] [rbp-1B0h]
  long long v75; // [rsp+10h] [rbp-1A8h]
  int offset; // [rsp+20h] [rbp-198h]
  const char *v77; // [rsp+28h] [rbp-190h]
  const char *v78; // [rsp+28h] [rbp-190h]
  __off_t v79; // [rsp+30h] [rbp-188h]
  char v81; // [rsp+3Dh] [rbp-17Bh]
  char v82; // [rsp+3Eh] [rbp-17Ah]
  char v83; // [rsp+3Fh] [rbp-179h]
  long long v84[2]; // [rsp+40h] [rbp-178h] BYREF
  struct stat buf; // [rsp+50h] [rbp-168h] BYREF
  struct stat v86; // [rsp+E0h] [rbp-D8h] BYREF
  unsigned long long v87; // [rsp+178h] [rbp-40h]

  v87 = __readfsqword(0x28u);
  if ( fstat(a1, &buf) )
  {
    v38 = __errno_location();
    LODWORD(v11) = 1;
    sub_8CB0(*v38);
    goto LABEL_104;
  }
  v2 = 0;
  if ( !a1 )
  {
    if ( dword_2B808 < 0 )
      dword_2B808 = (buf.st_mode & 0xF000) == 0x4000;
    goto LABEL_5;
  }
  if ( dword_2B884 == 2 )
  {
    v19 = buf.st_mode & 0xF000;
    if ( v19 == 49152 || (buf.st_mode & 0xB000) == 0x2000 || v19 == 4096 )
      goto LABEL_34;
  }
  else if ( !dword_2B884 && a2 != 1 )
  {
    v17 = buf.st_mode & 0xF000;
    if ( v17 == 49152 || (buf.st_mode & 0xB000) == 0x2000 || v17 == 4096 )
      goto LABEL_34;
    goto LABEL_44;
  }
  if ( !a2 )
    goto LABEL_44;
  if ( (buf.st_mode & 0xF000) != 0x4000 )
  {
    v20 = &qword_2B8B0;
LABEL_42:
    v21 = v20[1];
    if ( v21 && (unsigned char)sub_16B90(v21, domainname) )
      goto LABEL_34;
    goto LABEL_44;
  }
  if ( dword_2B018 == 4 )
  {
LABEL_34:
    LODWORD(v11) = 1;
    goto LABEL_35;
  }
  v20 = &qword_2B8A0;
  if ( !byte_2B88A )
    goto LABEL_42;
LABEL_44:
  if ( dword_2B808 < 0 )
  {
    dword_2B808 = (buf.st_mode & 0xF000) == 0x4000;
    v22 = dword_2B018;
    if ( dword_2B018 != 3 )
      goto LABEL_46;
    goto LABEL_120;
  }
  v22 = dword_2B018;
  if ( dword_2B018 == 3 )
  {
LABEL_120:
    if ( (buf.st_mode & 0xF000) != 0x4000 )
      goto LABEL_47;
    v41 = dword_2B014 & ~(a2 ^ 1);
    if ( close(a1) )
    {
      v70 = __errno_location();
      sub_8CB0(*v70);
    }
    v84[1] = 0;
    v84[0] = domainname;
    v42 = (int *)sub_189C0(v84, v41, 0);
    if ( !v42 )
      sub_1F410();
    LODWORD(v11) = 1;
    while ( 1 )
    {
      while ( 1 )
      {
        do
        {
LABEL_125:
          v43 = sub_18FD0(v42);
          v44 = v43;
          if ( !v43 )
          {
            v62 = __errno_location();
            v63 = v62;
            if ( *v62 )
              sub_8CB0(*v62);
            if ( (unsigned int)sub_18DE0(v42) )
              sub_8CB0(*v63);
            return (unsigned int)v11;
          }
          v45 = *(short *)(v43 + 104);
          v46 = a2 & (*(long long *)(v44 + 88) == 0);
        }
        while ( v45 == 6 );
        if ( !v46 )
          break;
LABEL_128:
        v47 = *(char **)(v44 + 56);
        domainname = v47;
        if ( byte_2B88A && v47[1] )
          domainname = v47 + 2;
        v48 = v42[18];
        v49 = 1;
        if ( (v48 & 2) == 0 )
          v49 = (unsigned char)(v46 & v48);
        switch ( v45 )
        {
          case 1:
            if ( dword_2B018 == 3 )
              goto LABEL_125;
            sub_19720(v42, v44, 4);
            goto LABEL_182;
          case 2:
            if ( !byte_2B904 )
            {
              v60 = domainname;
              v61 = dcgettext(0, "%s: warning: recursive directory loop", 5);
              error(0, 0, v61, v60);
            }
            goto LABEL_125;
          case 3:
          case 11:
            if ( dword_2B884 != 2 && (v46 || dword_2B884) )
              goto LABEL_182;
            v56 = *(int *)(v44 + 136);
            if ( !v56
              && (v57 = v49 << 8,
                  BYTE1(v57) = v49 ^ 1,
                  v58 = fstatat(v42[11], *(const char **)(v44 + 48), &v86, v57),
                  LOWORD(v56) = v86.st_mode,
                  v58) )
            {
              v65 = __errno_location();
              sub_8CB0(*v65);
            }
            else
            {
              v59 = v56 & 0xF000;
              if ( v59 != 49152 && (v56 & 0xB000) != 0x2000 && v59 != 4096 )
LABEL_182:
                LODWORD(v11) = sub_A0F0((unsigned int)v42[11], *(long long *)(v44 + 48), v49, v46) & v11;
            }
            break;
          case 4:
          case 7:
          case 10:
            sub_8CB0(*(int *)(v44 + 64));
            goto LABEL_125;
          case 8:
          case 13:
            goto LABEL_182;
          case 12:
          case 14:
            goto LABEL_125;
          default:
            abort();
        }
      }
      if ( (unsigned short)(v45 - 1) <= 1u || (v64 = qword_2B8B0, v45 == 4) )
      {
        if ( dword_2B018 == 4 )
          goto LABEL_198;
        v64 = qword_2B8A0;
      }
      if ( !v64 )
        goto LABEL_128;
      if ( !(unsigned char)sub_16B90(v64, v44 + 256) )
      {
        v45 = *(short *)(v44 + 104);
        goto LABEL_128;
      }
LABEL_198:
      sub_19720(v42, v44, 4);
    }
  }
LABEL_46:
  if ( v22 == 4 && (buf.st_mode & 0xF000) == 0x4000 )
    goto LABEL_34;
LABEL_47:
  if ( dword_2B884 == 2 || !dword_2B884 && !a2 )
  {
    v23 = buf.st_mode & 0xF000;
    if ( v23 == 49152 || (buf.st_mode & 0xB000) == 0x2000 || v23 == 4096 )
      goto LABEL_34;
  }
LABEL_5:
  v82 = byte_2B803;
  if ( !byte_2B803
    && !dword_2B80C
    && qword_2B7E0 > 1
    && (buf.st_mode & 0xF000) == 0x8000
    && buf.st_ino == qword_2B928
    && buf.st_dev == qword_2B920 )
  {
    if ( !byte_2B904 )
    {
      v66 = sub_90F0();
      v67 = dcgettext(0, "%s: input file is also the output", 5);
      error(0, 0, v67, v66);
    }
    byte_2B889 = 1;
    LODWORD(v11) = 1;
    goto LABEL_104;
  }
  v3 = byte_2B684;
  v4 = (char *)dest + 1;
  v83 = byte_2B7A2;
  v5 = ((unsigned long long)dest + 1) % qword_2B838;
  if ( v5 )
    v4 = (char *)dest + qword_2B838 + 1 - v5;
  qword_2B840 = v4;
  qword_2B848 = v4;
  *(v4 - 1) = byte_2B684;
  fd = a1;
  if ( a1 )
  {
    qword_2B830 = 0;
    byte_2B881 = 0;
    byte_2B880 = 0;
  }
  else
  {
    qword_2B830 = lseek(0, 0, 1);
    byte_2B881 = qword_2B830 < 0;
    byte_2B880 = qword_2B830 < 0;
    if ( qword_2B830 < 0 )
    {
      v51 = *__errno_location();
      if ( v51 != 29 )
      {
        sub_8CB0(v51);
        LODWORD(v11) = 1;
        v81 = 0;
        v75 = 0;
        goto LABEL_80;
      }
      qword_2B830 = 0;
    }
  }
  qword_2B7C8 = 0;
  s = 0;
  *((long long *)&xmmword_2B7A8 + 1) = qword_2B7E0;
  qword_2B818 = 0;
  ::offset = 0;
  *(long long *)&xmmword_2B7A8 = 0;
  byte_2B821 = byte_2B820 & (v3 == 0);
  byte_2B888 = 0;
  v81 = sub_8D20(0, (long long)&buf);
  if ( !v81 )
  {
    v53 = __errno_location();
    LODWORD(v11) = 1;
    sub_8CB0(*v53);
    v75 = 0;
    goto LABEL_80;
  }
  dword_2B8F8 = 0;
  if ( byte_2B8FC )
  {
    st_size = 0x7FFFFFFFFFFFFFFFLL;
    if ( (buf.st_mode & 0xF000) == 0x8000 )
    {
      st_size = buf.st_size;
      if ( byte_2B801 )
        st_size = (buf.st_size != 0x7FFFFFFFFFFFFFFFLL) + buf.st_size;
    }
    do
    {
      ++v2;
      st_size /= 10LL;
    }
    while ( st_size );
    dword_2B8F8 = v2;
  }
  v74 = -1;
  v73 = 0;
  v75 = 0;
  v6 = 0;
  v7 = 0;
  v8 = v81;
  while ( 1 )
  {
    v9 = (char *)qword_2B840;
    v10 = qword_2B848;
    v11 = (unsigned long long)v74 >> 63;
    LOBYTE(v11) = v3 != 0 && v74 < 0;
    if ( !(char)v11 )
      goto LABEL_21;
    offset = dword_2B810;
    if ( dword_2B810 == 1 )
      goto LABEL_21;
    *(char *)qword_2B840 = 0;
    v77 = v10;
    v79 = v9 - v10;
    v12 = strlen(v10);
    v13 = v79;
    v10 = v77;
    v14 = offset;
    if ( v79 == v12 )
    {
      if ( !v8 )
        goto LABEL_21;
      if ( byte_2B881 )
        goto LABEL_21;
      if ( (buf.st_mode & 0xF000) != 0x8000 )
        goto LABEL_21;
      if ( v79 >= buf.st_size )
        goto LABEL_21;
      if ( !a1 )
      {
        v35 = lseek(0, 0, 1);
        v10 = v77;
        v13 = v35;
        if ( v35 < 0 )
          goto LABEL_21;
      }
      v78 = v10;
      v36 = lseek(a1, v13, 4);
      v10 = v78;
      v37 = v36;
      if ( v36 < 0 )
        goto LABEL_21;
      if ( lseek(a1, v13, 0) < 0 )
      {
        v72 = __errno_location();
        sub_8CB0(*v72);
      }
      if ( v37 >= buf.st_size )
      {
        v10 = qword_2B848;
        v9 = (char *)qword_2B840;
        goto LABEL_21;
      }
      v14 = dword_2B810;
    }
    if ( v14 == 2 )
    {
      v81 = 0;
      v75 = 0;
      goto LABEL_80;
    }
    if ( !byte_2B7E8 )
    {
      byte_2B803 = 1;
      byte_2B7A2 = 1;
    }
    v10 = qword_2B848;
    v6 = v3;
    v9 = (char *)qword_2B840;
    byte_2B821 = byte_2B820;
    v74 = v75;
LABEL_21:
    qword_2B7C0 = (long long)v10;
    if ( s )
      s = (void *)v10;
    v11 = (unsigned long long)&v10[v7];
    if ( &v10[v7] == v9 )
      break;
    if ( v6 )
    {
      v15 = (char *)&v10[v7];
      while ( 1 )
      {
        *v9 = 0;
        v16 = strlen(v15);
        *v9 = v6;
        v15 += v16;
        if ( v15 == v9 )
          break;
        do
          *v15++ = v6;
        while ( !*v15 );
      }
    }
    v24 = *(char *)(v11 - 1);
    *(char *)(v11 - 1) = v3;
    v25 = (char *)memrchr((const void *)(v11 - 1), v3, (size_t)&v9[-v11 + 1]);
    *(char *)(v11 - 1) = v24;
    v26 = v11 - (long long)v73;
    v27 = v25 + 1;
    if ( (char *)v11 == v25 + 1 )
    {
      v27 = (char *)(v11 - (long long)v73);
      v73 = &v9[-v26];
      goto LABEL_62;
    }
    v73 = (char *)(v9 - v27);
    if ( (unsigned long long)v27 > v26 )
    {
      if ( !*((long long *)&xmmword_2B7A8 + 1) )
      {
        if ( !(long long)xmmword_2B7A8 )
          goto LABEL_91;
        goto LABEL_57;
      }
      v75 += sub_8790(v26, (unsigned long long)v27);
      if ( (long long)xmmword_2B7A8 )
      {
LABEL_57:
        sub_8710((unsigned long long)v27);
        if ( xmmword_2B7A8 == 0 )
          goto LABEL_91;
      }
      else if ( !*((long long *)&xmmword_2B7A8 + 1) )
      {
        goto LABEL_91;
      }
      if ( byte_2B7A2 )
      {
        v28 = 0;
        if ( v74 >= 0 )
          v28 = v74;
        if ( v28 < v75 )
        {
LABEL_91:
          v81 = 0;
          goto LABEL_78;
        }
      }
    }
LABEL_62:
    if ( qword_2B7F8 > 0 )
    {
      v29 = 0;
      do
      {
        if ( v27 <= qword_2B848 )
          break;
        if ( v27 == s )
          goto LABEL_71;
        ++v29;
        do
          --v27;
        while ( v3 != *(v27 - 1) );
      }
      while ( v29 != qword_2B7F8 );
    }
    if ( v27 != s )
      s = 0;
LABEL_71:
    v7 = v9 - v27;
    if ( byte_2B800 )
    {
      v30 = (char *)qword_2B840 - qword_2B848 - v7;
      v31 = __OFADD__(qword_2B7C8, v30);
      v32 = qword_2B7C8 + v30;
      if ( v31 )
      {
        v68 = dcgettext(0, "input is too large to count", 5);
        error(2, 0, v68);
        goto LABEL_203;
      }
      qword_2B7C8 = v32;
    }
    if ( byte_2B801 )
      sub_7C90((unsigned long long)v27);
    v33 = v9 - v27;
    v8 = 0;
    if ( !(unsigned char)sub_8D20(v33, (long long)&buf) )
    {
      v34 = __errno_location();
      sub_8CB0(*v34);
      v81 = 0;
      goto LABEL_78;
    }
  }
  if ( v73 )
  {
    v52 = *((long long *)&xmmword_2B7A8 + 1) == 0;
    *v9 = v3;
    qword_2B840 = v9 + 1;
    if ( !v52 )
      v75 += sub_8790((unsigned long long)&v10[v7 - (long long)v73], (unsigned long long)(v9 + 1));
    if ( (long long)xmmword_2B7A8 )
      sub_8710((unsigned long long)qword_2B840);
  }
LABEL_78:
  byte_2B7A2 = v83;
  LOBYTE(v11) = v75 == 0;
  byte_2B803 = v82;
  if ( !dword_2B810 && !v82 && (byte_2B888 || v74 >= 0 && v74 < v75) )
  {
    v54 = domainname;
    if ( !domainname )
    {
      domainname = dcgettext(0, "(standard input)", 5);
      v54 = domainname;
    }
    v55 = dcgettext(0, "%s: binary file matches", 5);
    error(0, 0, v55, v54);
  }
LABEL_80:
  if ( byte_2B7E8 )
  {
    if ( dword_2B808 )
    {
      sub_7FA0();
      if ( dword_2B804 )
        sub_7EC0(58);
      else
        sub_7E70(0);
    }
    sub_7B80((long long)"%ld\n", v75);
    if ( byte_2B7D8 && fflush_unlocked(stdout) )
      errnum = *__errno_location();
  }
  if ( !dword_2B80C )
  {
    if ( !a1 )
    {
      if ( *((long long *)&xmmword_2B7A8 + 1) )
      {
        if ( v81 || !byte_2B881 && (lseek(0, 0, 2) >= 0 || *__errno_location() == 22) )
          return (unsigned int)v11;
        if ( (buf.st_mode & 0xF000) != 0x1000 )
          goto LABEL_143;
LABEL_203:
        if ( !byte_2B7A0 )
          goto LABEL_143;
        v69 = splice(0, 0, 1, 0, 0x18000u, 1u);
        if ( v69 >= 0 )
        {
          if ( !v69 )
            return (unsigned int)v11;
          do
            v71 = splice(0, 0, 1, 0, 0x18000u, 1u);
          while ( v71 > 0 );
          if ( !v71 )
            return (unsigned int)v11;
        }
        else if ( *__errno_location() == 22 )
        {
LABEL_143:
          do
          {
            v50 = sub_1DA00(0, dest, nbytes);
            if ( !v50 )
              return (unsigned int)v11;
          }
          while ( v50 != -1 );
        }
      }
      else if ( qword_2B830 == ::offset || byte_2B881 || lseek(0, ::offset, 0) >= 0 )
      {
        return (unsigned int)v11;
      }
LABEL_36:
      v18 = __errno_location();
      sub_8CB0(*v18);
      return (unsigned int)v11;
    }
LABEL_35:
    if ( !close(a1) )
      return (unsigned int)v11;
    goto LABEL_36;
  }
  if ( dword_2B80C == (v75 == 0) + 1 )
  {
    sub_7FA0();
    sub_7E70(dword_2B804 & 0xA);
    if ( byte_2B7D8 )
    {
      if ( fflush_unlocked(stdout) )
        errnum = *__errno_location();
    }
  }
LABEL_104:
  if ( a1 )
    goto LABEL_35;
  return (unsigned int)v11;
}



// Function: grepfile @ 0xa0f0
char grepfile(int a1, int a2, char a3, unsigned char a4, int a5, int a6)
{
  int v8; // edx
  int v9; // edi
  char result; // al
  int v11; // edi

  v8 = a3 == 0 ? 131328 : 256;
  if ( dword_2B884 == 2 || !dword_2B884 && a4 != 1 )
    BYTE1(v8) |= 8u;
  v9 = sub_1B5D0(a1, a2, v8, a4, a5, a6);
  if ( v9 >= 0 )
    return sub_9130(v9, a4);
  v11 = *__errno_location();
  if ( a3 || (result = v11 == 31 || v11 == 40) == 0 )
  {
    sub_8CB0(v11);
    return 1;
  }
  return result;
}



// Function: usage @ 0xa170
void usage(int status)
{
  long long v1; // r12
  char *v2; // rax
  long long v3; // r12
  char *v4; // rax
  char *v5; // rax
  long long v6; // rdx
  long long v7; // r12
  char *v8; // rax
  char *v9; // rax
  long long v10; // rdx
  char *v11; // rax
  long long v12; // rdx
  char *v13; // rax
  long long v14; // rdx
  char *v15; // rax
  long long v16; // rdx
  char *v17; // rax
  long long v18; // rdx
  char *v19; // rax
  long long v20; // rdx
  char *v21; // rax
  long long v22; // rdx
  char *v23; // rax
  long long v24; // rdx
  char *v25; // rax
  long long v26; // rdx
  char *v27; // rax
  long long v28; // rdx
  char *v29; // rax
  long long v30; // rdx

  v1 = sub_19950();
  v2 = dcgettext(0, "Usage: %s [OPTION]... PATTERNS [FILE]...\n", 5);
  if ( status )
  {
    __fprintf_chk(stderr, 1, v2, v1);
    v3 = sub_19950();
    v4 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v4, v3);
  }
  else
  {
    __printf_chk(1, v2, v1);
    v5 = dcgettext(0, "Search for PATTERNS in each FILE.\n", 5);
    __printf_chk(1, v5, v6);
    v7 = sub_19950();
    v8 = dcgettext(
           0,
           "Example: %s -i 'hello world' menu.h main.c\n"
           "PATTERNS can contain multiple patterns separated by newlines.\n"
           "\n"
           "Pattern selection and interpretation:\n",
           5);
    __printf_chk(1, v8, v7);
    v9 = dcgettext(
           0,
           "  -E, --extended-regexp     PATTERNS are extended regular expressions\n"
           "  -F, --fixed-strings       PATTERNS are strings\n"
           "  -G, --basic-regexp        PATTERNS are basic regular expressions\n"
           "  -P, --perl-regexp         PATTERNS are Perl regular expressions\n",
           5);
    __printf_chk(1, v9, v10);
    v11 = dcgettext(
            0,
            "  -e, --regexp=PATTERNS     use PATTERNS for matching\n"
            "  -f, --file=FILE           take PATTERNS from FILE\n"
            "  -i, --ignore-case         ignore case distinctions in patterns and data\n"
            "      --no-ignore-case      do not ignore case distinctions (default)\n"
            "  -w, --word-regexp         match only whole words\n"
            "  -x, --line-regexp         match only whole lines\n"
            "  -z, --null-data           a data line ends in 0 byte, not newline\n",
            5);
    __printf_chk(1, v11, v12);
    v13 = dcgettext(
            0,
            "\n"
            "Miscellaneous:\n"
            "  -s, --no-messages         suppress error messages\n"
            "  -v, --invert-match        select non-matching lines\n"
            "  -V, --version             display version information and exit\n"
            "      --help                display this help text and exit\n",
            5);
    __printf_chk(1, v13, v14);
    v15 = dcgettext(
            0,
            "\n"
            "Output control:\n"
            "  -m, --max-count=NUM       stop after NUM selected lines\n"
            "  -b, --byte-offset         print the byte offset with output lines\n"
            "  -n, --line-number         print line number with output lines\n"
            "      --line-buffered       flush output on every line\n"
            "  -H, --with-filename       print file name with output lines\n"
            "  -h, --no-filename         suppress the file name prefix on output\n"
            "      --label=LABEL         use LABEL as the standard input file name prefix\n",
            5);
    __printf_chk(1, v15, v16);
    v17 = dcgettext(
            0,
            "  -o, --only-matching       show only nonempty parts of lines that match\n"
            "  -q, --quiet, --silent     suppress all normal output\n"
            "      --binary-files=TYPE   assume that binary files are TYPE;\n"
            "                            TYPE is 'binary', 'text', or 'without-match'\n"
            "  -a, --text                equivalent to --binary-files=text\n",
            5);
    __printf_chk(1, v17, v18);
    v19 = dcgettext(
            0,
            "  -I                        equivalent to --binary-files=without-match\n"
            "  -d, --directories=ACTION  how to handle directories;\n"
            "                            ACTION is 'read', 'recurse', or 'skip'\n"
            "  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n"
            "                            ACTION is 'read' or 'skip'\n"
            "  -r, --recursive           like --directories=recurse\n"
            "  -R, --dereference-recursive  likewise, but follow all symlinks\n",
            5);
    __printf_chk(1, v19, v20);
    v21 = dcgettext(
            0,
            "      --include=GLOB        search only files that match GLOB (a file pattern)\n"
            "      --exclude=GLOB        skip files that match GLOB\n"
            "      --exclude-from=FILE   skip files that match any file pattern from FILE\n"
            "      --exclude-dir=GLOB    skip directories that match GLOB\n",
            5);
    __printf_chk(1, v21, v22);
    v23 = dcgettext(
            0,
            "  -L, --files-without-match  print only names of FILEs with no selected lines\n"
            "  -l, --files-with-matches  print only names of FILEs with selected lines\n"
            "  -c, --count               print only a count of selected lines per FILE\n"
            "  -T, --initial-tab         make tabs line up (if needed)\n"
            "  -Z, --null                print 0 byte after FILE name\n",
            5);
    __printf_chk(1, v23, v24);
    v25 = dcgettext(
            0,
            "\n"
            "Context control:\n"
            "  -B, --before-context=NUM  print NUM lines of leading context\n"
            "  -A, --after-context=NUM   print NUM lines of trailing context\n"
            "  -C, --context=NUM         print NUM lines of output context\n",
            5);
    __printf_chk(1, v25, v26);
    v27 = dcgettext(
            0,
            "  -NUM                      same as --context=NUM\n"
            "      --group-separator=SEP  print SEP on line between matches with context\n"
            "      --no-group-separator  do not print separator for matches with context\n"
            "      --color[=WHEN],\n"
            "      --colour[=WHEN]       use markers to highlight the matching strings;\n"
            "                            WHEN is 'always', 'never', or 'auto'\n"
            "  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n"
            "\n",
            5);
    __printf_chk(1, v27, v28);
    v29 = dcgettext(
            0,
            "When FILE is '-', read standard input.  With no FILE, read '.' if\n"
            "recursive, '-' otherwise.  With fewer than two FILEs, assume -h.\n"
            "Exit status is 0 if any line is selected, 1 otherwise;\n"
            "if any error occurs and -q is not given, the exit status is 2.\n",
            5);
    __printf_chk(1, v29, v30);
    sub_1EE20();
  }
  exit(status);
}



// Function: fgrep_to_grep_pattern @ 0xa3e0
unsigned long long fgrep_to_grep_pattern(void **a1, size_t *a2)
{
  size_t v2; // r13
  char *v3; // r12
  long long v4; // rdi
  long long v5; // rax
  char *v6; // r15
  size_t v7; // r14
  size_t v8; // r14
  size_t v9; // rbp
  char *v10; // rax
  long long v11; // rax
  int v12; // eax
  long long v13; // rcx
  long long v15; // [rsp+8h] [rbp-60h]
  long long v17; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v18; // [rsp+28h] [rbp-40h]

  v2 = *a2;
  v3 = *a1;
  v4 = *a2 + 1;
  v18 = __readfsqword(0x28u);
  v17 = 0;
  v5 = sub_1F050(v4, 2);
  v15 = v5;
  if ( !v2 )
  {
    v6 = (char *)v5;
    goto LABEL_15;
  }
  v6 = (char *)v5;
  do
  {
    while ( 1 )
    {
      v11 = byte_2B180[(unsigned char)*v3 + 3];
      if ( (char)v11 == 0xFE )
        v11 = sub_1AC90(v3, v2, &v17);
      if ( v11 != -1 )
        break;
      v17 = 0;
LABEL_10:
      v12 = (unsigned char)*v3;
      if ( (unsigned char)(v12 - 36) <= 0x3Au )
      {
        v13 = 0x580000000000441LL;
        if ( _bittest64(&v13, (unsigned int)(v12 - 36)) )
        {
          *v6 = 92;
          LOBYTE(v12) = *v3;
          ++v6;
        }
      }
      --v2;
      *v6++ = v12;
      ++v3;
      if ( !v2 )
        goto LABEL_14;
    }
    if ( v11 == 1 )
      goto LABEL_10;
    v7 = 0;
    if ( v11 != -2 )
    {
      v8 = v2;
      v2 = v11;
      v7 = v8 - v11;
    }
    v9 = v2;
    v10 = mempcpy(v6, v3, v2);
    v2 = v7;
    v3 += v9;
    v6 = v10;
  }
  while ( v7 );
LABEL_14:
  v2 = (size_t)&v6[-v15];
LABEL_15:
  *v6 = 10;
  free(*a1);
  *a1 = (void *)v15;
  *a2 = v2;
  return v18 - __readfsqword(0x28u);
}



// Function: Fcompile @ 0xa570
struct obstack **Fcompile(struct obstack *s, struct obstack *a2)
{
  struct obstack *v2; // r15
  char *v3; // r14
  struct obstack *v4; // rbp
  struct obstack *v5; // r12
  size_t v6; // r13
  struct obstack *v7; // rsi
  char *v8; // rbx
  char v9; // cl
  char *v10; // rax
  long long v11; // rbx
  struct obstack **result; // rax
  char v13; // [rsp+7h] [rbp-61h]
  signed long long v15; // [rsp+20h] [rbp-48h]

  v2 = s;
  v3 = (char *)s + (long long)a2;
  v4 = 0;
  v15 = 0;
  v5 = (struct obstack *)sub_E5C0(1);
  do
  {
    v8 = (char *)rawmemchr(v2, 10);
    v6 = v8 - (char *)v2;
    if ( byte_2B685 )
    {
      v9 = byte_2B684;
      if ( v2 > s && byte_2B684 == 10 )
      {
        v2 = (struct obstack *)((char *)v2 - 1);
      }
      else
      {
        if ( (long long)(v6 + 1) >= v15 )
        {
          free(v4);
          v15 = v8 - (char *)v2;
          v10 = (char *)sub_1F1F0(0);
          v9 = byte_2B684;
          v4 = (struct obstack *)v10;
          *v10 = byte_2B684;
        }
        v13 = v9;
        memcpy((char *)&v4->chunk_size + 1, v2, v6);
        v2 = v4;
        *((char *)&v4->chunk_size + v6 + 1) = v13;
      }
      v6 += 2LL;
    }
    v7 = v2;
    v2 = (struct obstack *)(v8 + 1);
    sub_C9D0(v5, v7, v6);
  }
  while ( v8 + 1 <= v3 );
  free(v4);
  v11 = sub_CEB0(v5);
  sub_CEC0(v5);
  result = (struct obstack **)sub_1EF10(40);
  *result = v5;
  result[2] = s;
  result[1] = (struct obstack *)v11;
  result[3] = a2;
  result[4] = 0;
  return result;
}



// Function: Fexecute @ 0xa710
long long Fexecute(long long *a1, char *a2, long long a3, signed long long *a4, char *a5)
{
  char *v5; // rbp
  void *v6; // r12
  long long v7; // rax
  char *v8; // rbx
  long long v9; // rax
  char *v10; // rbx
  char *v11; // r13
  void *v12; // r15
  char *v13; // rax
  char *v14; // r15
  char *v15; // r10
  long long result; // rax
  char *v17; // rax
  char *v18; // r13
  char *v19; // r15
  int v20; // r12d
  char *v21; // rax
  char *v22; // rdx
  unsigned int v24; // [rsp+Ch] [rbp-BCh]
  char v26; // [rsp+31h] [rbp-97h]
  bool v27; // [rsp+32h] [rbp-96h]
  char v28; // [rsp+33h] [rbp-95h]
  char v29; // [rsp+34h] [rbp-94h]
  char *v30; // [rsp+38h] [rbp-90h]
  void *s; // [rsp+40h] [rbp-88h] BYREF
  long long v32; // [rsp+48h] [rbp-80h] BYREF
  char v33[16]; // [rsp+50h] [rbp-78h] BYREF
  long long v34; // [rsp+60h] [rbp-68h]
  char v35[16]; // [rsp+70h] [rbp-58h] BYREF
  char *v36; // [rsp+80h] [rbp-48h]
  unsigned long long v37; // [rsp+88h] [rbp-40h]

  v5 = &a2[a3];
  v6 = (void *)*a1;
  v37 = __readfsqword(0x28u);
  v28 = byte_2B684;
  v7 = (unsigned char)byte_2B685;
  v27 = a5 != 0;
  v26 = ((byte_2B685 | byte_2B182) ^ 1) & byte_2B180[0];
  v29 = a5 != 0;
  v8 = a2;
  if ( a5 )
    v8 = a5;
  v24 = ((unsigned char)byte_2B685
       ^ 1)
      & (unsigned char)((a5 != 0) | byte_2B686 | ((byte_2B685 | byte_2B182) ^ 1) & byte_2B180[0]);
  s = v8;
  if ( v5 >= v8 )
  {
    while ( 1 )
    {
      v9 = sub_E3E0(v6, &v8[-v7], v7 + v5 - v8, v33, v24);
      if ( v9 < 0 )
        return -1;
      v10 = &v8[v9];
      v32 = 0;
      v11 = (char *)(v34 - 2 * (unsigned char)byte_2B685);
      if ( !v26 || !sub_E640(&s, &v32, v10, v5) )
        break;
      v8 = (char *)s;
      if ( v5 < s )
        return -1;
LABEL_28:
      v7 = (unsigned char)byte_2B685;
    }
    if ( byte_2B686 != 1 && v27 )
    {
      v19 = v11;
      goto LABEL_36;
    }
    if ( byte_2B685 )
    {
      v19 = &v11[a5 == 0];
      goto LABEL_36;
    }
    if ( !byte_2B686 )
    {
      v18 = &v11[(long long)v10];
LABEL_39:
      v20 = v28;
      if ( v5 > v18 )
        v5 = (char *)rawmemchr(v18, v28) + 1;
LABEL_41:
      v21 = memrchr(a2, v20, v10 - a2);
      if ( v21 )
      {
        v22 = v21 + 1;
        result = v21 + 1 - a2;
      }
      else
      {
        v22 = a2;
        result = 0;
      }
      v19 = (char *)(v5 - v22);
      goto LABEL_37;
    }
    if ( v32 )
    {
      if ( v32 > 0 )
      {
        if ( !sub_E820(&v10[-v32], v5) )
        {
LABEL_18:
          while ( 1 )
          {
            v14 = &v11[(long long)v10];
            if ( !sub_E820(&v11[(long long)v10], v5) )
              break;
            if ( !a5 && !byte_2B180[0] )
            {
              if ( !a1[4] )
              {
                sub_A3E0((void **)a1 + 2, (size_t *)a1 + 3);
                a1[4] = sub_6A70((char *)a1[2], a1[3], 0x10A46u, v29);
              }
              v15 = v5;
              if ( v5 > v14 )
                v15 = (char *)rawmemchr(v14, v28) + 1;
              v30 = v15;
              if ( sub_7110((long long *)a1[4], v10, v15 - v10, a4, 0) < 0 )
              {
                v10 = v30 - 1;
                goto LABEL_27;
              }
              v20 = v28;
              v5 = v30;
              goto LABEL_41;
            }
            if ( !v11 || sub_E3E0(v6, v10, v11 - 1, v35, 1) )
              goto LABEL_27;
            v11 = v36;
          }
          v17 = v11;
          v18 = &v11[(long long)v10];
          v19 = v17;
          if ( !a5 )
            goto LABEL_39;
LABEL_36:
          result = v10 - a2;
LABEL_37:
          *a4 = (signed long long)v19;
          return result;
        }
        goto LABEL_27;
      }
      v12 = s;
    }
    else
    {
      v12 = s;
      v13 = (char *)memrchr(s, v28, v10 - (char *)s);
      if ( v13 )
      {
        v12 = v13 + 1;
        s = v13 + 1;
      }
    }
    if ( !sub_E830(v12, v10, v5) )
      goto LABEL_18;
LABEL_27:
    s = &v10[sub_E810(v10, v5)];
    v8 = (char *)s + 1;
    if ( v5 < (char *)s + 1 )
      return -1;
    goto LABEL_28;
  }
  return -1;
}



// Function: treefails @ 0xab10
void treefails(long long a1, long long a2, long long a3, unsigned char a4)
{
  unsigned int v4; // r14d
  long long v8; // rbx
  long long *v9; // rax
  unsigned char v10; // di
  long long *v11; // rcx
  long long *v12; // rdx
  long long *v13; // rax

  if ( a1 )
  {
    v4 = a4;
    v8 = a2;
    treefails(*(long long *)a1, a2, a3, a4);
    treefails(*(long long *)(a1 + 8), a2, a3, v4);
    if ( a2 )
    {
      while ( 1 )
      {
        v9 = *(long long **)(v8 + 8);
        if ( v9 )
          break;
LABEL_13:
        v8 = *(long long *)(v8 + 32);
        if ( !v8 )
          goto LABEL_14;
      }
      v10 = *(char *)(a1 + 24);
      while ( v10 != *((char *)v9 + 24) )
      {
        v11 = (long long *)*v9;
        if ( v10 >= *((char *)v9 + 24) )
          v11 = (long long *)v9[1];
        v9 = v11;
        if ( !v11 )
          goto LABEL_13;
      }
      v12 = *(long long **)(a1 + 16);
      v13 = (long long *)v9[2];
      v12[4] = v13;
      if ( !a4 && *v13 && !*v12 )
        *v12 = -1;
    }
    else
    {
LABEL_14:
      *(long long *)(*(long long *)(a1 + 16) + 32LL) = a3;
    }
  }
}



// Function: hasevery @ 0xabd0
long long hasevery(long long *a1, long long a2)
{
  long long *v2; // rbx
  long long result; // rax
  unsigned char v4; // cl
  long long *v5; // rdx

  if ( !a2 )
    return 1;
  v2 = a1;
  result = hasevery(a1, *(long long *)a2);
  if ( (char)result )
  {
    result = hasevery(a1, *(long long *)(a2 + 8));
    if ( (char)result )
    {
      if ( a1 )
      {
        v4 = *(char *)(a2 + 24);
        while ( v4 != *((char *)v2 + 24) )
        {
          v5 = (long long *)*v2;
          if ( v4 >= *((char *)v2 + 24) )
            v5 = (long long *)v2[1];
          v2 = v5;
          if ( !v5 )
            return 0;
        }
      }
      else
      {
        return 0;
      }
    }
  }
  return result;
}



// Function: memchr_kwset @ 0xac50
char *memchr_kwset(char *a1, long long a2, long long a3)
{
  int v3; // r8d
  char *v4; // rcx
  char *result; // rax
  int v6; // esi
  size_t v7; // rcx
  unsigned long long v8; // rdx
  char v9; // r9
  unsigned long long v10; // rsi
  long long v11; // rax
  char *v12; // rdx

  v3 = *(int *)(a3 + 2444);
  v4 = &a1[a2];
  if ( v3 >= 0 )
  {
    while ( a1 < v4 )
    {
      if ( *(long long *)(a3 + 8LL * (unsigned char)*a1 + 368) )
        return a1;
      if ( ((unsigned char)++a1 & 0xF) == 0 )
      {
        v6 = *(int *)(a3 + 2440);
        v7 = v4 - a1;
        v8 = v7;
        if ( (char)v6 == (char)v3 )
          return memchr(a1, (unsigned char)v6, v7);
        v9 = v6;
        if ( v7 )
        {
          while ( ((unsigned char)a1 & 7) != 0 )
          {
            if ( *a1 == (char)v3 || *a1 == (char)v6 )
              return a1;
            ++a1;
            if ( !--v8 )
              break;
          }
        }
        v10 = (unsigned char)v6
            | ((unsigned char)v6 << 8)
            | (unsigned long long)((long long)((unsigned char)v6 | ((unsigned char)v6 << 8)) << 16);
        if ( v8 <= 7 )
        {
LABEL_32:
          if ( !v8 )
            return 0;
        }
        else
        {
          while ( 1 )
          {
            v11 = ((long long)((unsigned char)v3 | ((unsigned char)v3 << 8)) << 16)
                | (unsigned char)v3
                | ((unsigned char)v3 << 8);
            if ( ((~((v10 | (v10 << 32)) ^ *(long long *)a1)
                 & (((v10 | (v10 << 32)) ^ *(long long *)a1) - 0x101010101010101LL)
                 | (((v11 | (v11 << 32)) ^ *(long long *)a1) - 0x101010101010101LL)
                 & ~((v11 | (v11 << 32)) ^ *(long long *)a1))
                & 0x8080808080808080LL) != 0 )
              break;
            v8 -= 8LL;
            a1 += 8;
            if ( v8 <= 7 )
              goto LABEL_32;
          }
        }
        v12 = &a1[v8];
        result = a1;
        while ( *result != v9 && *result != (char)v3 )
        {
          if ( ++result == v12 )
            return 0;
        }
        return result;
      }
    }
    return 0;
  }
  if ( a1 >= v4 )
    return 0;
  while ( 1 )
  {
    result = *(char **)(a3 + 8LL * (unsigned char)*a1 + 368);
    if ( result )
      break;
    if ( v4 == ++a1 )
      return result;
  }
  return a1;
}



// Function: acexec @ 0xaf60
long long acexec(long long a1, unsigned char *a2, long long a3, long long *a4, int a5)
{
  unsigned char *v6; // r13
  long long v9; // rbx
  long long *v10; // rdi
  long long v11; // rax
  int v12; // r15d
  unsigned char *v13; // r10
  char *v14; // rax
  long long v15; // rax
  unsigned char *v16; // r9
  long long v17; // rcx
  long long v18; // rax
  long long v19; // rdx
  long long result; // rax
  long long *v21; // r10
  long long v22; // rdx
  unsigned char *v23; // rax
  long long v24; // rsi
  long long v25; // rdx
  long long v26; // rcx
  long long v27; // rcx
  long long v28; // rax
  long long *v29; // rdx
  int v30; // [rsp+0h] [rbp-3Ch]

  a5 = (unsigned char)a5;
  if ( *(long long *)(a1 + 104) > a3 )
    return -1;
  v6 = &a2[a3];
  v9 = *(long long *)(a1 + 2432);
  v10 = *(long long **)(a1 + 96);
  v11 = *v10;
  if ( *v10 )
  {
    v16 = a2;
  }
  else
  {
    v12 = *(int *)(a1 + 2440);
    v13 = a2;
    while ( 2 )
    {
      if ( v12 < 0 )
      {
        while ( 1 )
        {
          v28 = *v13++;
          if ( v9 )
            v28 = *(unsigned char *)(v9 + v28);
          v10 = *(long long **)(a1 + 8 * v28 + 368);
          if ( v10 )
            break;
          if ( v6 <= v13 )
            return -1;
        }
      }
      else
      {
        v30 = a5;
        v14 = sub_AC50(v13, v6 - v13, a1);
        if ( !v14 )
          return -1;
        v13 = v14 + 1;
        a5 = v30;
        v15 = (unsigned char)*v14;
        if ( v9 )
          v15 = *(unsigned char *)(v9 + v15);
        v10 = *(long long **)(a1 + 8 * v15 + 368);
      }
      v11 = *v10;
      if ( !*v10 )
      {
        if ( v6 > v13 )
        {
LABEL_11:
          v16 = v13++;
          v17 = *(v13 - 1);
          if ( v9 )
            v17 = *(unsigned char *)(v9 + v17);
          while ( 2 )
          {
            v18 = v10[1];
            do
            {
              if ( *(char *)(v18 + 24) == (char)v17 )
              {
                v10 = *(long long **)(v18 + 16);
                goto LABEL_19;
              }
              v19 = *(long long *)v18;
              if ( *(char *)(v18 + 24) <= (unsigned char)v17 )
                v19 = *(long long *)(v18 + 8);
              v18 = v19;
            }
            while ( v19 );
            v10 = (long long *)v10[4];
            if ( v10 )
            {
              v11 = *v10;
              if ( !*v10 )
                continue;
              goto LABEL_23;
            }
            break;
          }
          v10 = *(long long **)(a1 + 8 * v17 + 368);
          if ( !v10 )
          {
            if ( v6 > v13 )
              continue;
            return -1;
          }
LABEL_19:
          v11 = *v10;
          if ( *v10 )
            break;
          if ( v6 != v13 )
            goto LABEL_11;
        }
        return -1;
      }
      break;
    }
    v16 = v13;
  }
LABEL_23:
  v21 = v10;
  if ( v11 < 0 )
  {
    do
      v21 = (long long *)v21[4];
    while ( *v21 < 0 );
  }
  v22 = v21[5];
  v23 = &v16[-v22];
  if ( (char)a5 && v16 < v6 )
  {
    do
    {
      v24 = *v16;
      if ( v9 )
        LOBYTE(v24) = *(char *)(v9 + v24);
      while ( 1 )
      {
        v25 = v10[1];
        if ( v25 )
          break;
LABEL_44:
        v10 = (long long *)v10[4];
        v22 = v21[5];
        if ( !v10 || v10[5] < v22 )
          goto LABEL_38;
      }
      while ( *(char *)(v25 + 24) != (char)v24 )
      {
        v26 = *(long long *)v25;
        if ( *(char *)(v25 + 24) <= (unsigned char)v24 )
          v26 = *(long long *)(v25 + 8);
        v25 = v26;
        if ( !v26 )
          goto LABEL_44;
      }
      v10 = *(long long **)(v25 + 16);
      ++v16;
      if ( *v10 )
      {
        v29 = *(long long **)(v25 + 16);
        if ( *v10 < 0 )
        {
          do
            v29 = (long long *)v29[4];
          while ( *v29 < 0 );
        }
        if ( &v16[-v29[5]] <= v23 )
        {
          v23 = &v16[-v29[5]];
          v21 = v29;
        }
      }
    }
    while ( v6 != v16 );
    v22 = v21[5];
  }
LABEL_38:
  v27 = *v21;
  result = v23 - a2;
  a4[2] = v22;
  a4[1] = result;
  *a4 = v27 >> 1;
  return result;
}



// Function: bmexec_trans @ 0xaf70
long long bmexec_trans(long long a1, char *a2, long long a3)
{
  signed long long v3; // r8
  long long v4; // rbx
  long long v5; // r12
  char v6; // r14
  char v7; // r13
  signed long long v8; // rbp
  unsigned long long v9; // rdx
  char *v10; // r15
  signed long long v11; // r11
  long long v12; // rax
  signed long long v13; // r9
  long long i; // r10
  long long v15; // rsi
  long long v16; // rax
  long long v17; // rcx
  long long v18; // rax
  long long v19; // rdi
  long long v20; // rcx
  long long v21; // rsi
  long long v22; // r9
  long long j; // r9
  long long v24; // rcx
  long long v25; // rsi
  char *v27; // r15
  long long v28; // rcx
  unsigned long long v29; // rax
  long long v30; // rdi
  char *v31; // rax
  char *v32; // rsi
  char *v33; // rsi
  long long v34; // rdi
  char *v35; // rsi
  char *v36; // rsi
  long long v37; // rdi
  char *v38; // rax
  unsigned long long v39; // rax
  char *v40; // rax
  signed long long v41; // r10
  long long k; // r11
  long long v43; // rdi
  long long v44; // rdx
  long long v45; // rsi
  char v46; // di
  long long v47; // rdx
  long long v48; // r9
  long long v49; // rsi
  long long v50; // rdi
  long long v51; // r10
  long long m; // r10
  long long v53; // rsi
  long long v54; // rdi
  char *v55; // rax
  long long v56; // [rsp+8h] [rbp-70h]
  long long v59; // [rsp+20h] [rbp-58h]
  signed long long v60; // [rsp+30h] [rbp-48h]

  v3 = *(long long *)(a1 + 104);
  if ( v3 )
  {
    v4 = *(long long *)(a1 + 2432);
    v5 = a1;
    if ( a3 < v3 )
      return -1;
    if ( v3 == 1 )
    {
      v55 = sub_AC50(a2, a3, a1);
      if ( v55 )
        return v55 - a2;
      return -1;
    }
    v6 = *(char *)(a1 + 2440);
    v56 = a1 + 112;
    v7 = *(char *)(a1 + 2448);
    v8 = v3 + *(long long *)(a1 + 2416);
    v9 = (unsigned long long)&a2[v3];
    if ( !is_mul_ok(0xCu, v3) || 12 * v3 >= a3 )
    {
      v10 = &a2[a3];
      goto LABEL_7;
    }
    v10 = &a2[a3];
    if ( v9 > (unsigned long long)&a2[a3 - 11 * v3] )
    {
LABEL_7:
      v11 = *(unsigned char *)(v5 + *(unsigned char *)(v9 - 1) + 112);
LABEL_8:
      while ( v11 <= (long long)&v10[-v9] )
      {
        v9 += v11;
        v12 = *(unsigned char *)(v9 - 1);
        v11 = *(unsigned char *)(v56 + v12);
        if ( !*(char *)(v56 + v12) )
        {
          v13 = v3;
          for ( i = 0; ; i = v16 - 1 )
          {
            v18 = *(unsigned char *)(v9 - 2);
            if ( v4 )
              LOBYTE(v18) = *(char *)(v4 + v18);
            if ( v7 == (char)v18 )
              break;
            v15 = 0;
            v16 = 2;
LABEL_12:
            v13 = *(long long *)(*(long long *)(v5 + 2424) + v15);
            v9 += v13;
            if ( v9 > (unsigned long long)v10 )
              goto LABEL_8;
            v17 = *(unsigned char *)(v9 - 1);
            if ( v4 )
              LOBYTE(v17) = *(char *)(v4 + v17);
            if ( v6 != (char)v17 )
              goto LABEL_8;
          }
          v19 = -3;
          v16 = 2;
          while ( ++v16 <= v13 )
          {
            v20 = *(unsigned char *)(v9 + v19);
            v21 = *(unsigned char *)(v8 + v19);
            if ( v4 )
            {
              LOBYTE(v20) = *(char *)(v4 + v20);
              LOBYTE(v21) = *(char *)(v4 + v21);
            }
            --v19;
            if ( (char)v21 != (char)v20 )
              goto LABEL_32;
          }
          v22 = i + v13;
          v16 = v22 + 1;
          if ( v3 >= v22 + 1 )
          {
            for ( j = ~v22; ; --j )
            {
              v24 = *(unsigned char *)(v9 + j);
              v25 = *(unsigned char *)(v8 + j);
              if ( v4 )
              {
                LOBYTE(v24) = *(char *)(v4 + v24);
                LOBYTE(v25) = *(char *)(v4 + v25);
              }
              if ( (char)v25 != (char)v24 )
                break;
              if ( v3 < ++v16 )
                return v9 - v3 - (long long)a2;
            }
            if ( v16 <= v3 )
            {
LABEL_32:
              v15 = 8 * v16 - 16;
              goto LABEL_12;
            }
          }
          return v9 - v3 - (long long)a2;
        }
      }
      return -1;
    }
    v27 = &a2[a3 - 11 * v3];
    v28 = a1;
    while ( 1 )
    {
      v29 = v9 + *(unsigned char *)(v56 + *(unsigned char *)(v9 - 1));
      v30 = *(unsigned char *)(v56 + *(unsigned char *)(v29 - 1));
      v31 = (char *)(v30 + v29);
      if ( !(char)v30 )
        break;
      v32 = &v31[*(unsigned char *)(v56 + (unsigned char)*(v31 - 1))];
      v33 = &v32[*(unsigned char *)(v56 + (unsigned char)*(v32 - 1))];
      v34 = *(unsigned char *)(v56 + (unsigned char)*(v33 - 1));
      v31 = &v33[v34];
      if ( !(char)v34 )
        break;
      v35 = &v31[*(unsigned char *)(v56 + (unsigned char)*(v31 - 1))];
      v36 = &v35[*(unsigned char *)(v56 + (unsigned char)*(v35 - 1))];
      v37 = *(unsigned char *)(v56 + (unsigned char)*(v36 - 1));
      v31 = &v36[v37];
      if ( !(char)v37 )
        break;
      v38 = &v31[*(unsigned char *)(v56 + (unsigned char)*(v31 - 1))];
      v39 = (unsigned long long)&v38[*(unsigned char *)(v56 + (unsigned char)*(v38 - 1))];
      if ( (long long)(v39 - v9) <= 127 )
      {
        v60 = v3;
        v59 = v28;
        v40 = sub_AC50((char *)(v39 - 1), (long long)&a2[a3 + 1 - v39], v28);
        if ( !v40 )
          return -1;
        v31 = v40 + 1;
        v28 = v59;
        v3 = v60;
        if ( v31 >= v27 )
        {
          v10 = &a2[a3];
          v5 = v59;
          v9 = (unsigned long long)v31;
          goto LABEL_7;
        }
        break;
      }
      v9 = v39;
LABEL_69:
      if ( (unsigned long long)v27 < v9 )
      {
        v5 = v28;
        v31 = (char *)v9;
LABEL_67:
        v9 = (unsigned long long)v31;
        v10 = &a2[a3];
        goto LABEL_7;
      }
    }
    v41 = v3;
    for ( k = 0; ; k = v44 - 1 )
    {
      v47 = (unsigned char)*(v31 - 2);
      if ( v4 )
        LOBYTE(v47) = *(char *)(v4 + v47);
      if ( v7 == (char)v47 )
        break;
      v43 = 0;
      v44 = 2;
LABEL_45:
      v41 = *(long long *)(*(long long *)(v28 + 2424) + v43);
      v31 += v41;
      v45 = (unsigned char)*(v31 - 1);
      if ( v27 < v31 )
      {
        v5 = v28;
        goto LABEL_67;
      }
      v46 = *(v31 - 1);
      if ( v4 )
        v46 = *(char *)(v4 + (unsigned char)v45);
      if ( v6 != v46 )
      {
        v9 = (unsigned long long)&v31[*(unsigned char *)(v56 + v45)];
        goto LABEL_69;
      }
    }
    v48 = -3;
    v44 = 2;
    while ( ++v44 <= v41 )
    {
      v49 = (unsigned char)v31[v48];
      v50 = *(unsigned char *)(v8 + v48);
      if ( v4 )
      {
        LOBYTE(v49) = *(char *)(v4 + v49);
        LOBYTE(v50) = *(char *)(v4 + v50);
      }
      --v48;
      if ( (char)v50 != (char)v49 )
        goto LABEL_65;
    }
    v51 = k + v41;
    v44 = v51 + 1;
    if ( v3 >= v51 + 1 )
    {
      for ( m = ~v51; ; --m )
      {
        v53 = (unsigned char)v31[m];
        v54 = *(unsigned char *)(v8 + m);
        if ( v4 )
        {
          LOBYTE(v53) = *(char *)(v4 + v53);
          LOBYTE(v54) = *(char *)(v4 + v54);
        }
        if ( (char)v54 != (char)v53 )
          break;
        if ( v3 < ++v44 )
          return &v31[-v3] - a2;
      }
      if ( v3 >= v44 )
      {
LABEL_65:
        v43 = 8 * v44 - 16;
        goto LABEL_45;
      }
    }
    return &v31[-v3] - a2;
  }
  return v3;
}



// Function: bmexec @ 0xb3f0
long long bmexec(long long a1, char *a2, long long a3, long long *a4)
{
  long long result; // rax
  long long v6; // rdx

  result = sub_AF70(a1, a2, a3);
  v6 = *(long long *)(a1 + 104);
  *a4 = 0;
  a4[1] = result;
  a4[2] = v6;
  return result;
}



// Function: kwsalloc @ 0xc880
long long kwsalloc(long long a1)
{
  long long v1; // r12
  long long v2; // rcx
  long long v3; // rax
  long long *v4; // rdx
  long long v5; // rax
  long long v6; // rsi
  long long v7; // rax

  v1 = sub_1EF10(2464);
  obstack_begin(v1, 0, 0, sub_1EF10, &free);
  v2 = *(long long *)(v1 + 32);
  v3 = *(long long *)(v1 + 24);
  *(long long *)(v1 + 88) = 0;
  if ( (unsigned long long)(v2 - v3) <= 0x3F )
  {
    obstack_newchunk(v1, 64);
    v4 = *(long long **)(v1 + 16);
    v2 = *(long long *)(v1 + 32);
    v5 = *(long long *)(v1 + 24) + 64LL;
    if ( (long long *)v5 != v4 )
      goto LABEL_3;
  }
  else
  {
    v4 = *(long long **)(v1 + 16);
    v5 = v3 + 64;
    if ( (long long *)v5 != v4 )
      goto LABEL_3;
  }
  *(char *)(v1 + 80) |= 2u;
LABEL_3:
  v6 = *(long long *)(v1 + 48);
  *(long long *)(v1 + 96) = v4;
  v7 = ~v6 & (v6 + v5);
  if ( v7 - *(long long *)(v1 + 8) <= (unsigned long long)(v2 - *(long long *)(v1 + 8)) )
    v2 = v7;
  *(long long *)(v1 + 24) = v2;
  *(long long *)(v1 + 16) = v2;
  *v4 = 0;
  v4[1] = 0;
  v4[2] = 0;
  v4[3] = 0;
  v4[4] = 0;
  v4[5] = 0;
  v4[6] = 0;
  *(long long *)(v1 + 104) = 0x7FFFFFFFFFFFFFFFLL;
  *(long long *)(v1 + 2432) = a1;
  *(long long *)(v1 + 2456) = sub_AF60;
  *(long long *)(v1 + 2416) = 0;
  return v1;
}



// Function: kwsincr @ 0xc9d0
unsigned long long kwsincr(long long *a1, unsigned char *a2, long long a3)
{
  long long *v3; // r15
  unsigned char *v4; // r12
  long long (*v5)(long long, char *, long long, long long *); // r11
  long long *v6; // r14
  long long v7; // r13
  long long v8; // r15
  long long *v9; // rdx
  long long v10; // rbp
  long long *v11; // rax
  long long v12; // rbx
  unsigned char v13; // cl
  long long v14; // rsi
  long long v15; // rsi
  long long v16; // rax
  long long v17; // r8
  long long v18; // rax
  long long v19; // r9
  long long *v20; // rax
  long long v21; // rdi
  long long v22; // rax
  long long *v23; // rcx
  long long v24; // rax
  bool v25; // cc
  int v26; // edi
  long long v27; // rax
  long long *v28; // rax
  long long *v29; // rsi
  char v30; // al
  char v31; // al
  int v32; // eax
  long long *v33; // rax
  long long v34; // rdi
  long long v36; // rax
  long long v37; // rax
  int v39; // eax
  long long v40; // rdi
  long long v41; // rbx
  long long *v42; // rsi
  char *v43; // rdi
  long long v44; // r8
  long long v45; // r9
  long long *v46; // rdi
  long long v47; // r8
  long long v48; // r9
  unsigned long long v49; // [rsp+0h] [rbp-F8h]
  long long *v50; // [rsp+0h] [rbp-F8h]
  long long *v51; // [rsp+0h] [rbp-F8h]
  long long v52; // [rsp+8h] [rbp-F0h]
  long long (*v53)(long long, char *, long long, long long *); // [rsp+8h] [rbp-F0h]
  long long v54; // [rsp+10h] [rbp-E8h]
  long long (*v55)(long long, char *, long long, long long *); // [rsp+18h] [rbp-E0h]
  int v56[12]; // [rsp+20h] [rbp-D8h]
  long long v57[13]; // [rsp+50h] [rbp-A8h]
  unsigned long long v58; // [rsp+B8h] [rbp-40h]

  v3 = a1;
  v4 = a2;
  v5 = (long long (*)(long long, char *, long long, long long *))a1[307];
  v6 = (long long *)a1[12];
  v58 = __readfsqword(0x28u);
  if ( v5 == sub_B3F0 )
    v4 = &a2[a3];
  if ( a3 )
  {
    v7 = a3 - 1;
    v8 = a1[304];
    v9 = a1;
    while ( 1 )
    {
      if ( v5 == sub_B3F0 )
        v10 = *--v4;
      else
        v10 = *v4++;
      if ( v8 )
        LOBYTE(v10) = *(char *)(v8 + v10);
      v11 = (long long *)v6[1];
      v12 = 1;
      v56[0] = 0;
      v57[0] = v6 + 1;
      if ( v11 )
      {
        while ( 1 )
        {
          v13 = *((char *)v11 + 24);
          if ( v13 == (char)v10 )
            break;
          v57[v12] = v11;
          v14 = v12 + 1;
          if ( v13 > (unsigned char)v10 )
          {
            v11 = (long long *)*v11;
            v56[v14 - 1] = 0;
            if ( !v11 )
              goto LABEL_16;
          }
          else
          {
            v11 = (long long *)v11[1];
            v56[v14 - 1] = 1;
            if ( !v11 )
              goto LABEL_16;
          }
          ++v12;
        }
        v6 = (long long *)v11[2];
        goto LABEL_38;
      }
      v12 = 0;
LABEL_16:
      v15 = v9[4];
      v16 = v9[3];
      if ( (unsigned long long)(v15 - v16) <= 0x1F )
      {
        v53 = v5;
        v51 = v9;
        obstack_newchunk(v9, 32);
        v9 = v51;
        v5 = v53;
        v17 = v51[2];
        v15 = v51[4];
        v18 = v51[3] + 32LL;
        if ( v18 != v17 )
          goto LABEL_18;
      }
      else
      {
        v17 = v9[2];
        v18 = v16 + 32;
        if ( v18 != v17 )
          goto LABEL_18;
      }
      *((char *)v9 + 80) |= 2u;
LABEL_18:
      v19 = v9[6];
      v20 = (long long *)(~v19 & (v19 + v18));
      v52 = ~v19;
      v21 = v9[1];
      v49 = v15 - v21;
      if ( (unsigned long long)v20 - v21 > v15 - v21 )
      {
        v9[3] = v15;
        v9[2] = v15;
        *(long long *)v17 = 0;
        *(long long *)(v17 + 8) = 0;
LABEL_20:
        v55 = v5;
        v54 = v17;
        v50 = v9;
        obstack_newchunk(v9, 64);
        v9 = v50;
        v17 = v54;
        v5 = v55;
        v19 = v50[6];
        v15 = v50[4];
        v22 = v50[3] + 64LL;
        v52 = ~v19;
        v21 = v50[1];
        v49 = v15 - v21;
        v23 = (long long *)v9[2];
        if ( (long long *)v22 == v23 )
          goto LABEL_51;
        goto LABEL_21;
      }
      v9[3] = v20;
      v9[2] = v20;
      *(long long *)v17 = 0;
      *(long long *)(v17 + 8) = 0;
      if ( (unsigned long long)(v15 - (long long)v20) <= 0x3F )
        goto LABEL_20;
      v23 = v20;
      v22 = (long long)(v20 + 8);
      if ( (long long *)v22 == v23 )
LABEL_51:
        *((char *)v9 + 80) |= 2u;
LABEL_21:
      v24 = v52 & (v19 + v22);
      v25 = v24 - v21 <= v49;
      v26 = v56[v12];
      if ( v25 )
        v15 = v24;
      v9[3] = v15;
      v27 = v6[5];
      v9[2] = v15;
      *(long long *)(v17 + 16) = v23;
      v23[5] = v27 + 1;
      v28 = (long long *)v57[v12];
      *v23 = 0;
      v23[1] = 0;
      v23[2] = v6;
      v23[3] = 0;
      v23[4] = 0;
      v23[6] = 0;
      *(char *)(v17 + 24) = v10;
      *(char *)(v17 + 25) = 0;
      if ( v26 )
        v28[1] = v17;
      else
        *v28 = v17;
      if ( v12 )
      {
        while ( 1 )
        {
          v29 = (long long *)v57[v12];
          v30 = *((char *)v29 + 25);
          if ( v30 )
            break;
          *((char *)v29 + 25) = v26 == 0 ? -1 : 1;
          if ( !--v12 )
            goto LABEL_48;
          v26 = v56[v12];
        }
        if ( v26 )
        {
          if ( v26 == 1 )
          {
            v31 = v30 + 1;
            *((char *)v29 + 25) = v31;
            if ( v31 )
            {
LABEL_32:
              if ( v31 == -2 )
              {
                v39 = v56[v12 + 1];
                if ( v39 )
                {
                  if ( v39 != 1 )
LABEL_64:
                    abort();
                  v46 = (long long *)*v29;
                  v33 = *(long long **)(*v29 + 8);
                  v47 = v33[1];
                  v48 = *v33;
                  *v33 = *v29;
                  v46[1] = v48;
                  v33[1] = (long long)v29;
                  *v29 = v47;
                  *((char *)v46 + 25) = -(*((char *)v33 + 25) == 1);
                  *((char *)v29 + 25) = *((char *)v33 + 25) == 0xFF;
                  *((char *)v33 + 25) = 0;
                }
                else
                {
                  v33 = (long long *)*v29;
                  v40 = *(long long *)(*v29 + 8);
                  v33[1] = (long long)v29;
                  *((char *)v29 + 25) = 0;
                  *v29 = v40;
                  *((char *)v33 + 25) = 0;
                }
              }
              else
              {
                if ( v31 != 2 )
                  goto LABEL_64;
                v32 = v56[v12 + 1];
                if ( v32 )
                {
                  if ( v32 != 1 )
                    goto LABEL_64;
                  v33 = (long long *)v29[1];
                  v34 = *v33;
                  *v33 = (long long)v29;
                  *((char *)v29 + 25) = 0;
                  v29[1] = v34;
                  *((char *)v33 + 25) = 0;
                }
                else
                {
                  v43 = (char *)v29[1];
                  v33 = *(long long **)v43;
                  v44 = *(long long *)(*(long long *)v43 + 8LL);
                  v45 = **(long long **)v43;
                  *v33 = (long long)v29;
                  v29[1] = v45;
                  v33[1] = (long long)v43;
                  *(long long *)v43 = v44;
                  *((char *)v29 + 25) = -(*((char *)v33 + 25) == 1);
                  v43[25] = *((char *)v33 + 25) == 0xFF;
                  *((char *)v33 + 25) = 0;
                }
              }
              v41 = v12 - 1;
              v42 = (long long *)v57[v41];
              if ( v56[v41] )
                v42[1] = v33;
              else
                *v42 = v33;
              v6 = v23;
              goto LABEL_38;
            }
          }
        }
        else
        {
          v31 = v30 - 1;
          *((char *)v29 + 25) = v31;
          if ( v31 )
            goto LABEL_32;
        }
      }
LABEL_48:
      v6 = v23;
LABEL_38:
      if ( v7-- == 0 )
      {
        v3 = v9;
        break;
      }
    }
  }
  v36 = v3[11];
  if ( !*v6 )
    *v6 = 2 * v36 + 1;
  v3[11] = v36 + 1;
  v37 = v6[5];
  if ( v37 < v3[13] )
    v3[13] = v37;
  return v58 - __readfsqword(0x28u);
}



// Function: kwswords @ 0xceb0
long long kwswords(long long a1)
{
  return *(long long *)(a1 + 88);
}



// Function: kwsprep @ 0xcec0
unsigned long long kwsprep(struct obstack *obstack)
{
  struct _obstack_chunk *(*chunkfun)(void *, long long); // rbx
  char *p_next_free; // r12
  long long chunk_size; // rax
  unsigned char v5; // r15
  char *object_base; // rax
  long long v7; // rsi
  struct _obstack_chunk *chunk; // r8
  long long *v9; // rcx
  long long *v10; // r10
  long long *v11; // rbx
  long long v12; // r9
  long long **v13; // r11
  long long v14; // rdi
  struct _obstack_chunk *v15; // rdx
  long long *v16; // rbx
  struct _obstack_chunk *v17; // rdx
  long long v18; // rdi
  struct _obstack_chunk *v19; // rax
  long long *v20; // r10
  struct _obstack_chunk *v21; // rdx
  long long *v22; // rbx
  long long v23; // rdi
  struct _obstack_chunk *v24; // rdx
  long long *v25; // rbx
  struct _obstack_chunk *v26; // rax
  long long v27; // rdi
  struct _obstack_chunk *v28; // rax
  long long *v29; // rcx
  struct _obstack_chunk *v30; // rdx
  long long *v31; // r10
  long long *v32; // rbx
  long long v33; // rdi
  struct _obstack_chunk *v34; // rdx
  long long *v35; // rbx
  struct _obstack_chunk *v36; // rdx
  long long v37; // rdi
  struct _obstack_chunk *v38; // rax
  long long *v39; // r10
  struct _obstack_chunk *v40; // rax
  long long *v41; // rbx
  long long v42; // rdi
  struct _obstack_chunk *v43; // rdx
  long long *v44; // rbx
  struct _obstack_chunk *v45; // rdx
  long long v46; // rdi
  struct _obstack_chunk *v47; // rax
  long long *v48; // r10
  long long v49; // rbx
  long long *v50; // rcx
  long long *v51; // r13
  long long v52; // rdi
  struct _obstack_chunk *v53; // rdx
  long long *v54; // r13
  struct _obstack_chunk *v55; // rdx
  long long v56; // rdi
  struct _obstack_chunk *v57; // rax
  long long *v58; // rcx
  struct _obstack_chunk *v59; // rdx
  long long *v60; // r13
  long long v61; // rdi
  struct _obstack_chunk *v62; // rdx
  long long *v63; // r13
  struct _obstack_chunk *v64; // rax
  long long v65; // rdi
  struct _obstack_chunk *v66; // rax
  long long v67; // rcx
  struct _obstack_chunk *v68; // rax
  long long *v69; // r13
  long long *v70; // rdx
  long long v71; // rdi
  struct _obstack_chunk *v72; // rsi
  long long *v73; // rdx
  long long v74; // rdi
  struct _obstack_chunk *v75; // rdx
  struct _obstack_chunk *v76; // rax
  long long *v77; // r13
  struct _obstack_chunk *v78; // rdx
  long long *v79; // rdx
  long long v80; // rdi
  struct _obstack_chunk *v81; // rsi
  long long *v82; // rdx
  struct _obstack_chunk *v83; // rsi
  long long v84; // rdi
  long long v85; // rdx
  long long v86; // r13
  struct _obstack_chunk *v87; // rax
  long long *v88; // r10
  long long v89; // rbx
  char *v90; // rax
  long long *v91; // rax
  char *v92; // rax
  char *v93; // rax
  long long v94; // rbx
  char *v95; // rax
  long long *v96; // rax
  char *v97; // rax
  char *v98; // rax
  char *v99; // rax
  long long *v100; // r10
  long long v101; // rbx
  char *v102; // rax
  long long *v103; // rax
  char *v104; // rax
  char *v105; // rax
  long long v106; // rbx
  char *v107; // rax
  long long *v108; // rax
  char *v109; // rax
  char *v110; // rax
  char *v111; // rax
  char *v112; // rax
  long long *v113; // r11
  char *v114; // rax
  long long *v115; // rax
  char *v116; // rax
  char *v117; // rax
  long long v118; // r11
  char *v119; // rax
  long long *v120; // rax
  char *v121; // rax
  char *v122; // rax
  char *v123; // rax
  long long *v124; // r11
  long long v125; // rbx
  char *v126; // rax
  long long *v127; // rax
  char *v128; // rax
  char *v129; // rax
  long long v130; // rbx
  char *v131; // rax
  long long *v132; // rax
  char *v133; // rax
  char *v134; // rax
  char *v135; // rax
  char *v136; // rax
  char *v137; // rax
  long long *v138; // r8
  long long v139; // r9
  long long v140; // r10
  char *v141; // rax
  char *v142; // rax
  long long v143; // rbx
  long long v144; // r8
  long long v145; // r11
  long long v146; // rax
  long long v147; // rax
  char **p_object_base; // r8
  long long v149; // r8
  int v150; // r9d
  int v151; // r10d
  long long v152; // rax
  int v153; // esi
  long long v154; // rcx
  long long v155; // rdx
  int v156; // edi
  struct _obstack_chunk *v157; // rdx
  long long v158; // rax
  struct _obstack_chunk *v160; // r9
  struct _obstack_chunk **v161; // r11
  struct _obstack_chunk *i; // rcx
  struct _obstack_chunk *prev; // r8
  char *limit; // r10
  long long v165; // rdi
  struct _obstack_chunk *v166; // rdx
  struct _obstack_chunk *v167; // r10
  struct _obstack_chunk *v168; // rdx
  struct _obstack_chunk *v169; // rdi
  struct _obstack_chunk *v170; // rax
  char *chunk_limit; // rcx
  char *next_free; // rax
  char *v173; // r8
  char *v174; // r14
  long long tempint; // rdx
  char *v176; // rax
  long long v177; // rsi
  char *v178; // rax
  long long v179; // rax
  long long *v180; // rax
  char *v181; // rdx
  unsigned char *v182; // rsi
  void *v183; // r14
  char *v184; // rax
  long long v185; // rsi
  long long *j; // rax
  long long v187; // rdx
  long long v188; // rcx
  char *v189; // rdx
  char *v190; // rax
  char *v191; // rsi
  char *v192; // r13
  long long v193; // rcx
  char *v194; // rax
  long long v195; // rdi
  char *v196; // rax
  struct _obstack_chunk *v197; // rdx
  char *v198; // rax
  char *v200; // rcx
  char *v201; // rdx
  char *v202; // rax
  unsigned long long v203; // r13
  char *v204; // rdi
  char *v205; // rax
  long long v206; // rsi
  char *v207; // rax
  struct _obstack_chunk *v208; // rax
  char *v209; // rdx
  long long v210; // rax
  long long v211; // rcx
  long long v212; // rax
  long long v213; // [rsp+8h] [rbp-970h]
  long long *v214; // [rsp+10h] [rbp-968h]
  long long *v215; // [rsp+10h] [rbp-968h]
  long long *v216; // [rsp+10h] [rbp-968h]
  long long *v217; // [rsp+10h] [rbp-968h]
  long long *v218; // [rsp+10h] [rbp-968h]
  long long *v219; // [rsp+10h] [rbp-968h]
  long long *v220; // [rsp+10h] [rbp-968h]
  long long *v221; // [rsp+10h] [rbp-968h]
  long long *v222; // [rsp+10h] [rbp-968h]
  long long v223; // [rsp+10h] [rbp-968h]
  long long *v224; // [rsp+10h] [rbp-968h]
  long long v225; // [rsp+10h] [rbp-968h]
  long long *v226; // [rsp+10h] [rbp-968h]
  long long v227; // [rsp+10h] [rbp-968h]
  long long *v228; // [rsp+10h] [rbp-968h]
  long long v229; // [rsp+10h] [rbp-968h]
  long long *v230; // [rsp+10h] [rbp-968h]
  long long v231; // [rsp+10h] [rbp-968h]
  long long *v232; // [rsp+10h] [rbp-968h]
  long long v233; // [rsp+10h] [rbp-968h]
  long long *v234; // [rsp+10h] [rbp-968h]
  long long v235; // [rsp+10h] [rbp-968h]
  long long *v236; // [rsp+10h] [rbp-968h]
  long long v237; // [rsp+10h] [rbp-968h]
  struct _obstack_chunk *(*v238)(void *, long long); // [rsp+18h] [rbp-960h]
  struct _obstack_chunk *v239; // [rsp+28h] [rbp-950h] BYREF
  char v240; // [rsp+30h] [rbp-948h] BYREF
  char v241; // [rsp+830h] [rbp-148h] BYREF
  unsigned long long v242; // [rsp+938h] [rbp-40h]

  chunkfun = obstack[27].chunkfun;
  v242 = __readfsqword(0x28u);
  p_next_free = &v241;
  if ( !chunkfun )
    p_next_free = (char *)&obstack[1].next_free;
  chunk_size = obstack[1].chunk_size;
  v213 = chunk_size;
  v5 = chunk_size == 1;
  if ( chunk_size != 1 )
  {
    object_base = obstack[1].object_base;
    if ( (long long)object_base > 255 )
      LOBYTE(object_base) = -1;
    *(long long *)p_next_free = 0x101010101010101LL * (unsigned char)object_base;
    v7 = (__PAIR128__(0x101010101010101LL, 0x101010101010101LL) * (unsigned char)object_base) >> 64;
    *((long long *)p_next_free + 1) = v7;
    *((long long *)p_next_free + 3) = v7;
    *((long long *)p_next_free + 2) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 5) = v7;
    *((long long *)p_next_free + 7) = v7;
    *((long long *)p_next_free + 9) = v7;
    *((long long *)p_next_free + 4) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 11) = v7;
    *((long long *)p_next_free + 13) = v7;
    *((long long *)p_next_free + 15) = v7;
    *((long long *)p_next_free + 6) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 17) = v7;
    *((long long *)p_next_free + 19) = v7;
    *((long long *)p_next_free + 21) = v7;
    *((long long *)p_next_free + 8) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 23) = v7;
    *((long long *)p_next_free + 25) = v7;
    *((long long *)p_next_free + 27) = v7;
    *((long long *)p_next_free + 10) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 29) = v7;
    *((long long *)p_next_free + 12) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 14) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 16) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 18) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 20) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 22) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 24) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 26) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 28) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 30) = 0x101010101010101LL * (unsigned char)object_base;
    *((long long *)p_next_free + 31) = v7;
    chunk = obstack[1].chunk;
    v239 = chunk;
    if ( !chunk )
      goto LABEL_239;
    goto LABEL_7;
  }
  v160 = obstack[1].chunk;
  v161 = &v239;
  v239 = v160;
  for ( i = v160; i; i = (struct _obstack_chunk *)i[1].limit )
  {
    prev = i->prev;
    if ( prev )
    {
      limit = prev->limit;
      if ( prev->limit )
      {
        if ( *(long long *)limit )
          sub_B420(*(long long *)limit, v161);
        v165 = *((long long *)limit + 1);
        if ( v165 )
          sub_B420(v165, v161);
        v166 = v239;
        v239 = (struct _obstack_chunk *)*((long long *)limit + 2);
        v166[1].limit = (char *)v239;
      }
      v167 = prev->prev;
      v168 = v239;
      if ( v167 )
      {
        if ( v167->limit )
          sub_B420(v167->limit, v161);
        v169 = v167->prev;
        if ( v169 )
          sub_B420(v169, v161);
        v168 = *(struct _obstack_chunk **)v167->contents;
        v239[1].limit = (char *)v168;
      }
      v170 = *(struct _obstack_chunk **)prev->contents;
      v168[1].limit = (char *)v170;
      v239 = v170;
    }
  }
  chunk_limit = obstack->chunk_limit;
  next_free = obstack->next_free;
  v173 = obstack[1].object_base;
  v174 = v173;
  if ( (unsigned long long)v173 > chunk_limit - next_free )
  {
    obstack_newchunk(obstack, obstack[1].object_base);
    tempint = (long long)obstack->object_base;
    chunk_limit = obstack->chunk_limit;
    v160 = obstack[1].chunk;
    v176 = &v174[(unsigned long long)obstack->next_free];
    v173 = obstack[1].object_base;
    if ( v176 != (char *)tempint )
      goto LABEL_281;
  }
  else
  {
    tempint = (long long)obstack->object_base;
    v176 = &next_free[(long long)v173];
    if ( v176 != (char *)tempint )
      goto LABEL_281;
  }
  *((char *)obstack + 80) |= 2u;
LABEL_281:
  v177 = *(long long *)&obstack->alignment_mask;
  obstack[27].temp.tempint = tempint;
  v178 = (char *)(~v177 & (unsigned long long)&v176[v177]);
  if ( (char *)(v178 - (char *)obstack->chunk) <= (char *)(chunk_limit - (char *)obstack->chunk) )
    chunk_limit = v178;
  obstack->next_free = chunk_limit;
  obstack->object_base = chunk_limit;
  if ( (long long)v173 > 0 )
  {
    v179 = 0;
    while ( 1 )
    {
      *(char *)(tempint + v179++) = v160->prev[1].limit;
      v160 = (struct _obstack_chunk *)v160[1].limit;
      if ( (long long)obstack[1].object_base <= v179 )
        break;
      tempint = obstack[27].temp.tempint;
    }
  }
  v180 = (long long *)sub_C880((long long)obstack[27].chunkfun);
  v181 = obstack[1].object_base;
  v182 = (unsigned char *)obstack[27].temp.tempint;
  v183 = v180;
  v180[307] = sub_B3F0;
  sub_C9D0(v180, v182, (long long)v181);
  obstack_free(obstack, 0);
  qmemcpy(obstack, v183, 0x9A0u);
  free(v183);
  v184 = obstack[1].object_base;
  if ( (long long)v184 > 255 )
    LOBYTE(v184) = -1;
  *(long long *)p_next_free = 0x101010101010101LL * (unsigned char)v184;
  v185 = (__PAIR128__(0x101010101010101LL, 0x101010101010101LL) * (unsigned char)v184) >> 64;
  *((long long *)p_next_free + 1) = v185;
  *((long long *)p_next_free + 3) = v185;
  *((long long *)p_next_free + 2) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 5) = v185;
  *((long long *)p_next_free + 7) = v185;
  *((long long *)p_next_free + 9) = v185;
  *((long long *)p_next_free + 4) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 11) = v185;
  *((long long *)p_next_free + 13) = v185;
  *((long long *)p_next_free + 15) = v185;
  *((long long *)p_next_free + 6) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 17) = v185;
  *((long long *)p_next_free + 19) = v185;
  *((long long *)p_next_free + 21) = v185;
  *((long long *)p_next_free + 8) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 23) = v185;
  *((long long *)p_next_free + 25) = v185;
  *((long long *)p_next_free + 27) = v185;
  *((long long *)p_next_free + 10) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 29) = v185;
  *((long long *)p_next_free + 12) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 14) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 16) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 18) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 20) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 22) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 24) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 26) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 28) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 30) = 0x101010101010101LL * (unsigned char)v184;
  *((long long *)p_next_free + 31) = v185;
  chunk = obstack[1].chunk;
  v239 = chunk;
  if ( !chunk )
  {
    v140 = 0;
    goto LABEL_291;
  }
LABEL_7:
  v238 = chunkfun;
  do
  {
    while ( 1 )
    {
      v12 = (long long)chunk->prev;
      if ( !v12 )
        goto LABEL_223;
      v13 = *(long long ***)v12;
      if ( *(long long *)v12 )
      {
        v9 = *v13;
        if ( *v13 )
        {
          v10 = (long long *)*v9;
          if ( *v9 )
          {
            v11 = (long long *)*v10;
            if ( *v10 )
            {
              if ( *v11 )
                sub_B420(*v11, &v239);
              v14 = v11[1];
              if ( v14 )
                sub_B420(v14, &v239);
              v15 = v239;
              v239 = (struct _obstack_chunk *)v11[2];
              v15[1].limit = (char *)v239;
            }
            v16 = (long long *)v10[1];
            v17 = v239;
            if ( v16 )
            {
              if ( *v16 )
                sub_B420(*v16, &v239);
              v18 = v16[1];
              if ( v18 )
                sub_B420(v18, &v239);
              v17 = (struct _obstack_chunk *)v16[2];
              v239[1].limit = (char *)v17;
            }
            v19 = (struct _obstack_chunk *)v10[2];
            v17[1].limit = (char *)v19;
            v239 = v19;
          }
          v20 = (long long *)v9[1];
          v21 = v239;
          if ( v20 )
          {
            v22 = (long long *)*v20;
            if ( *v20 )
            {
              if ( *v22 )
                sub_B420(*v22, &v239);
              v23 = v22[1];
              if ( v23 )
                sub_B420(v23, &v239);
              v24 = v239;
              v239 = (struct _obstack_chunk *)v22[2];
              v24[1].limit = (char *)v239;
            }
            v25 = (long long *)v20[1];
            v26 = v239;
            if ( v25 )
            {
              if ( *v25 )
                sub_B420(*v25, &v239);
              v27 = v25[1];
              if ( v27 )
                sub_B420(v27, &v239);
              v26 = (struct _obstack_chunk *)v25[2];
              v239[1].limit = (char *)v26;
            }
            v21 = (struct _obstack_chunk *)v20[2];
            v26[1].limit = (char *)v21;
          }
          v28 = (struct _obstack_chunk *)v9[2];
          v21[1].limit = (char *)v28;
          v239 = v28;
        }
        v29 = v13[1];
        v30 = v239;
        if ( v29 )
        {
          v31 = (long long *)*v29;
          if ( *v29 )
          {
            v32 = (long long *)*v31;
            if ( *v31 )
            {
              if ( *v32 )
                sub_B420(*v32, &v239);
              v33 = v32[1];
              if ( v33 )
                sub_B420(v33, &v239);
              v34 = v239;
              v239 = (struct _obstack_chunk *)v32[2];
              v34[1].limit = (char *)v239;
            }
            v35 = (long long *)v31[1];
            v36 = v239;
            if ( v35 )
            {
              if ( *v35 )
                sub_B420(*v35, &v239);
              v37 = v35[1];
              if ( v37 )
                sub_B420(v37, &v239);
              v36 = (struct _obstack_chunk *)v35[2];
              v239[1].limit = (char *)v36;
            }
            v38 = (struct _obstack_chunk *)v31[2];
            v36[1].limit = (char *)v38;
            v239 = v38;
          }
          v39 = (long long *)v29[1];
          v40 = v239;
          if ( v39 )
          {
            v41 = (long long *)*v39;
            if ( *v39 )
            {
              if ( *v41 )
                sub_B420(*v41, &v239);
              v42 = v41[1];
              if ( v42 )
                sub_B420(v42, &v239);
              v43 = v239;
              v239 = (struct _obstack_chunk *)v41[2];
              v43[1].limit = (char *)v239;
            }
            v44 = (long long *)v39[1];
            v45 = v239;
            if ( v44 )
            {
              if ( *v44 )
                sub_B420(*v44, &v239);
              v46 = v44[1];
              if ( v46 )
                sub_B420(v46, &v239);
              v45 = (struct _obstack_chunk *)v44[2];
              v239[1].limit = (char *)v45;
            }
            v40 = (struct _obstack_chunk *)v39[2];
            v45[1].limit = (char *)v40;
          }
          v30 = (struct _obstack_chunk *)v29[2];
          v40[1].limit = (char *)v30;
        }
        v47 = (struct _obstack_chunk *)v13[2];
        v48 = *(long long **)(v12 + 8);
        v30[1].limit = (char *)v47;
        v239 = v47;
        if ( v48 )
        {
LABEL_69:
          v49 = *v48;
          if ( *v48 )
          {
            v50 = *(long long **)v49;
            if ( *(long long *)v49 )
            {
              v51 = (long long *)*v50;
              if ( *v50 )
              {
                if ( *v51 )
                  sub_B420(*v51, &v239);
                v52 = v51[1];
                if ( v52 )
                  sub_B420(v52, &v239);
                v53 = v239;
                v239 = (struct _obstack_chunk *)v51[2];
                v53[1].limit = (char *)v239;
              }
              v54 = (long long *)v50[1];
              v55 = v239;
              if ( v54 )
              {
                if ( *v54 )
                  sub_B420(*v54, &v239);
                v56 = v54[1];
                if ( v56 )
                  sub_B420(v56, &v239);
                v55 = (struct _obstack_chunk *)v54[2];
                v239[1].limit = (char *)v55;
              }
              v57 = (struct _obstack_chunk *)v50[2];
              v55[1].limit = (char *)v57;
              v239 = v57;
            }
            v58 = *(long long **)(v49 + 8);
            v59 = v239;
            if ( v58 )
            {
              v60 = (long long *)*v58;
              if ( *v58 )
              {
                if ( *v60 )
                  sub_B420(*v60, &v239);
                v61 = v60[1];
                if ( v61 )
                  sub_B420(v61, &v239);
                v62 = v239;
                v239 = (struct _obstack_chunk *)v60[2];
                v62[1].limit = (char *)v239;
              }
              v63 = (long long *)v58[1];
              v64 = v239;
              if ( v63 )
              {
                if ( *v63 )
                  sub_B420(*v63, &v239);
                v65 = v63[1];
                if ( v65 )
                  sub_B420(v65, &v239);
                v64 = (struct _obstack_chunk *)v63[2];
                v239[1].limit = (char *)v64;
              }
              v59 = (struct _obstack_chunk *)v58[2];
              v64[1].limit = (char *)v59;
            }
            v66 = *(struct _obstack_chunk **)(v49 + 16);
            v59[1].limit = (char *)v66;
            v239 = v66;
          }
          v67 = v48[1];
          v68 = v239;
          if ( v67 )
          {
            v69 = *(long long **)v67;
            if ( *(long long *)v67 )
            {
              v70 = (long long *)*v69;
              if ( *v69 )
              {
                if ( *v70 )
                {
                  v214 = (long long *)*v69;
                  sub_B420(*v70, &v239);
                  v70 = v214;
                }
                v71 = v70[1];
                if ( v71 )
                {
                  v215 = v70;
                  sub_B420(v71, &v239);
                  v70 = v215;
                }
                v72 = v239;
                v239 = (struct _obstack_chunk *)v70[2];
                v72[1].limit = (char *)v239;
              }
              v73 = (long long *)v69[1];
              if ( v73 )
              {
                if ( *v73 )
                {
                  v216 = (long long *)v69[1];
                  sub_B420(*v73, &v239);
                  v73 = v216;
                }
                v74 = v73[1];
                if ( v74 )
                {
                  v217 = v73;
                  sub_B420(v74, &v239);
                  v73 = v217;
                }
                v75 = (struct _obstack_chunk *)v73[2];
                v239[1].limit = (char *)v75;
              }
              else
              {
                v75 = v239;
              }
              v76 = (struct _obstack_chunk *)v69[2];
              v75[1].limit = (char *)v76;
              v239 = v76;
            }
            v77 = *(long long **)(v67 + 8);
            v78 = v239;
            if ( v77 )
            {
              v79 = (long long *)*v77;
              if ( *v77 )
              {
                if ( *v79 )
                {
                  v218 = (long long *)*v77;
                  sub_B420(*v79, &v239);
                  v79 = v218;
                }
                v80 = v79[1];
                if ( v80 )
                {
                  v219 = v79;
                  sub_B420(v80, &v239);
                  v79 = v219;
                }
                v81 = v239;
                v239 = (struct _obstack_chunk *)v79[2];
                v81[1].limit = (char *)v239;
              }
              v82 = (long long *)v77[1];
              v83 = v239;
              if ( v82 )
              {
                if ( *v82 )
                {
                  v220 = (long long *)v77[1];
                  sub_B420(*v82, &v239);
                  v82 = v220;
                }
                v84 = v82[1];
                if ( v84 )
                {
                  v221 = v82;
                  sub_B420(v84, &v239);
                  v82 = v221;
                }
                v83 = (struct _obstack_chunk *)v82[2];
                v239[1].limit = (char *)v83;
              }
              v78 = (struct _obstack_chunk *)v77[2];
              v83[1].limit = (char *)v78;
            }
            v68 = *(struct _obstack_chunk **)(v67 + 16);
            v78[1].limit = (char *)v68;
          }
          v85 = v48[2];
          v86 = *(long long *)chunk[1].contents;
          v68[1].limit = (char *)v85;
          v87 = *(struct _obstack_chunk **)(v12 + 16);
          *(long long *)(v85 + 24) = v87;
          v239 = v87;
          if ( !v13 )
          {
LABEL_176:
            if ( v49 )
            {
              v113 = *(long long **)v49;
              if ( *(long long *)v49 )
              {
                if ( *v113 )
                {
                  v230 = (long long *)*v113;
                  sub_C100(*(long long *)*v113, v86, p_next_free);
                  sub_C100(v230[1], v86, p_next_free);
                  v114 = &p_next_free[*((unsigned char *)v230 + 24)];
                  if ( (unsigned char)*v114 > v86 )
                    *v114 = v86;
                }
                v115 = (long long *)v113[1];
                if ( v115 )
                {
                  v231 = v113[1];
                  sub_C100(*v115, v86, p_next_free);
                  sub_C100(*(long long *)(v231 + 8), v86, p_next_free);
                  v116 = &p_next_free[*(unsigned char *)(v231 + 24)];
                  if ( (unsigned char)*v116 > v86 )
                    *v116 = v86;
                }
                v117 = &p_next_free[*((unsigned char *)v113 + 24)];
                if ( (unsigned char)*v117 > v86 )
                  *v117 = v86;
              }
              v118 = *(long long *)(v49 + 8);
              if ( v118 )
              {
                if ( *(long long *)v118 )
                {
                  v232 = *(long long **)v118;
                  sub_C100(**(long long **)v118, v86, p_next_free);
                  sub_C100(v232[1], v86, p_next_free);
                  v119 = &p_next_free[*((unsigned char *)v232 + 24)];
                  if ( (unsigned char)*v119 > v86 )
                    *v119 = v86;
                }
                v120 = *(long long **)(v118 + 8);
                if ( v120 )
                {
                  v233 = *(long long *)(v118 + 8);
                  sub_C100(*v120, v86, p_next_free);
                  sub_C100(*(long long *)(v233 + 8), v86, p_next_free);
                  v121 = &p_next_free[*(unsigned char *)(v233 + 24)];
                  if ( (unsigned char)*v121 > v86 )
                    *v121 = v86;
                }
                v122 = &p_next_free[*(unsigned char *)(v118 + 24)];
                if ( (unsigned char)*v122 > v86 )
                  *v122 = v86;
              }
              v123 = &p_next_free[*(unsigned char *)(v49 + 24)];
              if ( (unsigned char)*v123 > v86 )
                *v123 = v86;
            }
            v124 = (long long *)v48[1];
            if ( v124 )
            {
              v125 = *v124;
              if ( *v124 )
              {
                if ( *(long long *)v125 )
                {
                  v234 = *(long long **)v125;
                  sub_C100(**(long long **)v125, v86, p_next_free);
                  sub_C100(v234[1], v86, p_next_free);
                  v126 = &p_next_free[*((unsigned char *)v234 + 24)];
                  if ( (unsigned char)*v126 > v86 )
                    *v126 = v86;
                }
                v127 = *(long long **)(v125 + 8);
                if ( v127 )
                {
                  v235 = *(long long *)(v125 + 8);
                  sub_C100(*v127, v86, p_next_free);
                  sub_C100(*(long long *)(v235 + 8), v86, p_next_free);
                  v128 = &p_next_free[*(unsigned char *)(v235 + 24)];
                  if ( (unsigned char)*v128 > v86 )
                    *v128 = v86;
                }
                v129 = &p_next_free[*(unsigned char *)(v125 + 24)];
                if ( (unsigned char)*v129 > v86 )
                  *v129 = v86;
              }
              v130 = v124[1];
              if ( v130 )
              {
                if ( *(long long *)v130 )
                {
                  v236 = *(long long **)v130;
                  sub_C100(**(long long **)v130, v86, p_next_free);
                  sub_C100(v236[1], v86, p_next_free);
                  v131 = &p_next_free[*((unsigned char *)v236 + 24)];
                  if ( (unsigned char)*v131 > v86 )
                    *v131 = v86;
                }
                v132 = *(long long **)(v130 + 8);
                if ( v132 )
                {
                  v237 = *(long long *)(v130 + 8);
                  sub_C100(*v132, v86, p_next_free);
                  sub_C100(*(long long *)(v237 + 8), v86, p_next_free);
                  v133 = &p_next_free[*(unsigned char *)(v237 + 24)];
                  if ( (unsigned char)*v133 > v86 )
                    *v133 = v86;
                }
                v134 = &p_next_free[*(unsigned char *)(v130 + 24)];
                if ( (unsigned char)*v134 > v86 )
                  *v134 = v86;
              }
              v135 = &p_next_free[*((unsigned char *)v124 + 24)];
              if ( (unsigned char)*v135 > v86 )
                *v135 = v86;
            }
            v136 = &p_next_free[*((unsigned char *)v48 + 24)];
            if ( (unsigned char)*v136 > v86 )
              *v136 = v86;
            goto LABEL_220;
          }
        }
        else
        {
          v157 = *(struct _obstack_chunk **)(v12 + 16);
          v86 = *(long long *)chunk[1].contents;
          v47[1].limit = (char *)v157;
          v239 = v157;
        }
        v88 = *v13;
        if ( *v13 )
        {
          v89 = *v88;
          if ( *v88 )
          {
            if ( *(long long *)v89 )
            {
              v222 = *(long long **)v89;
              sub_C100(**(long long **)v89, v86, p_next_free);
              sub_C100(v222[1], v86, p_next_free);
              v90 = &p_next_free[*((unsigned char *)v222 + 24)];
              if ( (unsigned char)*v90 > v86 )
                *v90 = v86;
            }
            v91 = *(long long **)(v89 + 8);
            if ( v91 )
            {
              v223 = *(long long *)(v89 + 8);
              sub_C100(*v91, v86, p_next_free);
              sub_C100(*(long long *)(v223 + 8), v86, p_next_free);
              v92 = &p_next_free[*(unsigned char *)(v223 + 24)];
              if ( (unsigned char)*v92 > v86 )
                *v92 = v86;
            }
            v93 = &p_next_free[*(unsigned char *)(v89 + 24)];
            if ( (unsigned char)*v93 > v86 )
              *v93 = v86;
          }
          v94 = v88[1];
          if ( v94 )
          {
            if ( *(long long *)v94 )
            {
              v224 = *(long long **)v94;
              sub_C100(**(long long **)v94, v86, p_next_free);
              sub_C100(v224[1], v86, p_next_free);
              v95 = &p_next_free[*((unsigned char *)v224 + 24)];
              if ( (unsigned char)*v95 > v86 )
                *v95 = v86;
            }
            v96 = *(long long **)(v94 + 8);
            if ( v96 )
            {
              v225 = *(long long *)(v94 + 8);
              sub_C100(*v96, v86, p_next_free);
              sub_C100(*(long long *)(v225 + 8), v86, p_next_free);
              v97 = &p_next_free[*(unsigned char *)(v225 + 24)];
              if ( (unsigned char)*v97 > v86 )
                *v97 = v86;
            }
            v98 = &p_next_free[*(unsigned char *)(v94 + 24)];
            if ( (unsigned char)*v98 > v86 )
              *v98 = v86;
          }
          v99 = &p_next_free[*((unsigned char *)v88 + 24)];
          if ( (unsigned char)*v99 > v86 )
            *v99 = v86;
        }
        v100 = v13[1];
        if ( v100 )
        {
          v101 = *v100;
          if ( *v100 )
          {
            if ( *(long long *)v101 )
            {
              v226 = *(long long **)v101;
              sub_C100(**(long long **)v101, v86, p_next_free);
              sub_C100(v226[1], v86, p_next_free);
              v102 = &p_next_free[*((unsigned char *)v226 + 24)];
              if ( (unsigned char)*v102 > v86 )
                *v102 = v86;
            }
            v103 = *(long long **)(v101 + 8);
            if ( v103 )
            {
              v227 = *(long long *)(v101 + 8);
              sub_C100(*v103, v86, p_next_free);
              sub_C100(*(long long *)(v227 + 8), v86, p_next_free);
              v104 = &p_next_free[*(unsigned char *)(v227 + 24)];
              if ( (unsigned char)*v104 > v86 )
                *v104 = v86;
            }
            v105 = &p_next_free[*(unsigned char *)(v101 + 24)];
            if ( (unsigned char)*v105 > v86 )
              *v105 = v86;
          }
          v106 = v100[1];
          if ( v106 )
          {
            if ( *(long long *)v106 )
            {
              v228 = *(long long **)v106;
              sub_C100(**(long long **)v106, v86, p_next_free);
              sub_C100(v228[1], v86, p_next_free);
              v107 = &p_next_free[*((unsigned char *)v228 + 24)];
              if ( (unsigned char)*v107 > v86 )
                *v107 = v86;
            }
            v108 = *(long long **)(v106 + 8);
            if ( v108 )
            {
              v229 = *(long long *)(v106 + 8);
              sub_C100(*v108, v86, p_next_free);
              sub_C100(*(long long *)(v229 + 8), v86, p_next_free);
              v109 = &p_next_free[*(unsigned char *)(v229 + 24)];
              if ( (unsigned char)*v109 > v86 )
                *v109 = v86;
            }
            v110 = &p_next_free[*(unsigned char *)(v106 + 24)];
            if ( (unsigned char)*v110 > v86 )
              *v110 = v86;
          }
          v111 = &p_next_free[*((unsigned char *)v100 + 24)];
          if ( (unsigned char)*v111 > v86 )
            *v111 = v86;
        }
        v112 = &p_next_free[*((unsigned char *)v13 + 24)];
        if ( (unsigned char)*v112 > v86 )
          *v112 = v86;
        v48 = *(long long **)(v12 + 8);
        if ( !v48 )
          goto LABEL_220;
        v49 = *v48;
        goto LABEL_176;
      }
      v48 = *(long long **)(v12 + 8);
      if ( v48 )
        goto LABEL_69;
      v141 = *(char **)(v12 + 16);
      v86 = *(long long *)chunk[1].contents;
      v239[1].limit = v141;
      v239 = (struct _obstack_chunk *)v141;
LABEL_220:
      v137 = &p_next_free[*(unsigned char *)(v12 + 24)];
      if ( (unsigned char)*v137 > v86 )
        *v137 = v86;
      v12 = (long long)chunk->prev;
LABEL_223:
      sub_AB10(v12, (long long)chunk[1].prev, (long long)obstack[1].chunk, v5);
      if ( v213 == 1 )
      {
        v142 = obstack[1].object_base;
        v143 = v138[4];
        v138[6] = v142;
        v138[7] = v142;
        if ( v143 )
          break;
      }
      chunk = (struct _obstack_chunk *)v138[3];
      if ( !chunk )
        goto LABEL_238;
    }
    do
    {
      if ( !(unsigned char)sub_ABD0(*(long long **)(v143 + 8), v139) )
      {
        v146 = *(long long *)(v144 + 40) - *(long long *)(v143 + 40);
        if ( v146 < *(long long *)(v143 + 48) )
          *(long long *)(v143 + 48) = v146;
      }
      if ( v145 )
      {
        v147 = *(long long *)(v144 + 40) - *(long long *)(v143 + 40);
        if ( *(long long *)(v143 + 56) > v147 )
          *(long long *)(v143 + 56) = v147;
      }
      v143 = *(long long *)(v143 + 32);
    }
    while ( v143 );
    chunk = *(struct _obstack_chunk **)(v144 + 24);
  }
  while ( chunk );
LABEL_238:
  chunkfun = v238;
  if ( v213 != 1 )
    goto LABEL_239;
LABEL_291:
  for ( j = *(long long **)(v140 + 24); j; j = (long long *)j[3] )
  {
    v187 = j[7];
    v188 = *(long long *)(j[2] + 56LL);
    if ( v187 > v188 )
    {
      j[7] = v188;
      v187 = v188;
    }
    if ( j[6] > v187 )
      j[6] = v187;
  }
LABEL_239:
  p_object_base = (char **)&v240;
  if ( !chunkfun )
    p_object_base = &obstack[4].object_base;
  *p_object_base = 0;
  p_object_base[255] = 0;
  memset(
    (void *)((unsigned long long)(p_object_base + 1) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)p_object_base - (((int)p_object_base + 8) & 0xFFFFFFF8) + 2048) >> 3));
  sub_BA10(obstack[1].chunk->prev, p_object_base);
  v152 = 0;
  v153 = -2;
  while ( 2 )
  {
    while ( 2 )
    {
      v156 = v152;
      if ( chunkfun )
      {
        v154 = *((unsigned char *)chunkfun + v152);
        v155 = *(long long *)(v149 + 8 * v154);
        *((long long *)&obstack[4].object_base + v152) = v155;
      }
      else
      {
        v155 = *((long long *)&obstack[4].object_base + v152);
        LODWORD(v154) = v152;
      }
      if ( !v155 )
      {
LABEL_249:
        if ( ++v152 == 256 )
          goto LABEL_254;
        continue;
      }
      break;
    }
    if ( v153 != -2 )
    {
      if ( (int)v154 == v153 )
      {
        if ( v153 != v150 )
          v156 = v151;
        v150 = v156;
      }
      else if ( (int)v154 == (int)v152 && v153 == v150 )
      {
        v150 = v152;
      }
      else
      {
        v153 = -1;
      }
      goto LABEL_249;
    }
    ++v152;
    v150 = v156;
    v153 = v154;
    if ( v152 != 256 )
      continue;
    break;
  }
LABEL_254:
  LODWORD(obstack[27].freefun) = v153;
  HIDWORD(obstack[27].freefun) = v150;
  if ( v213 != 1 )
    goto LABEL_255;
  v189 = obstack->chunk_limit;
  v190 = obstack->next_free;
  v191 = obstack[1].object_base;
  v192 = v191;
  if ( (unsigned long long)v191 > v189 - v190 )
  {
    obstack_newchunk(obstack, v191);
    v193 = (long long)obstack->object_base;
    v189 = obstack->chunk_limit;
    v191 = obstack[1].object_base;
    v194 = &v192[(unsigned long long)obstack->next_free];
    if ( v194 != (char *)v193 )
      goto LABEL_300;
LABEL_320:
    *((char *)obstack + 80) |= 2u;
    goto LABEL_300;
  }
  v193 = (long long)obstack->object_base;
  v194 = &v190[(long long)v191];
  if ( v194 == (char *)v193 )
    goto LABEL_320;
LABEL_300:
  v195 = *(long long *)&obstack->alignment_mask;
  obstack[27].temp.tempint = v193;
  v196 = (char *)(~v195 & (unsigned long long)&v194[v195]);
  if ( (char *)(v196 - (char *)obstack->chunk) <= (char *)(v189 - (char *)obstack->chunk) )
    v189 = v196;
  obstack->next_free = v189;
  obstack->object_base = v189;
  v197 = obstack[1].chunk;
  if ( (long long)v191 > 0 )
  {
    v198 = v191 - 1;
    while ( 1 )
    {
      v198[v193] = (char)v197->prev[1].limit;
      v197 = (struct _obstack_chunk *)v197[1].limit;
      if ( v198-- == 0 )
        break;
      v193 = obstack[27].temp.tempint;
    }
    v200 = obstack[1].object_base;
    if ( (long long)v200 > 1 )
    {
      v201 = obstack->chunk_limit;
      v202 = obstack->next_free;
      v203 = 8LL * (long long)v200 - 8;
      if ( v203 > v201 - v202 )
      {
        obstack_newchunk(obstack, 8LL * (long long)v200 - 8);
        v202 = obstack->next_free;
        v201 = obstack->chunk_limit;
        v200 = obstack[1].object_base;
      }
      v204 = obstack->object_base;
      v205 = &v202[v203];
      if ( v205 == v204 )
        *((char *)obstack + 80) |= 2u;
      v206 = *(long long *)&obstack->alignment_mask;
      *(long long *)&obstack[27].alignment_mask = v204;
      v207 = (char *)(~v206 & (unsigned long long)&v205[v206]);
      if ( (char *)(v207 - (char *)obstack->chunk) <= (char *)(v201 - (char *)obstack->chunk) )
        v201 = v207;
      v208 = obstack[1].chunk;
      obstack->next_free = v201;
      obstack->object_base = v201;
      v209 = v208[1].limit;
      if ( (long long)v200 > 1 )
      {
        v210 = 0;
        do
        {
          v211 = *((long long *)v209 + 6);
          v209 = (char *)*((long long *)v209 + 3);
          *(long long *)&v204[8 * v210] = v211;
          v200 = obstack[1].object_base;
          ++v210;
        }
        while ( (long long)(v200 - 1) > v210 );
      }
      v212 = (unsigned char)v200[obstack[27].temp.tempint - 2];
      if ( chunkfun )
        LOBYTE(v212) = *((char *)chunkfun + v212);
      LOBYTE(obstack[27].extra_arg) = v212;
    }
  }
LABEL_255:
  v158 = 0;
  if ( chunkfun )
  {
    do
    {
      *((char *)&obstack[1].next_free + v158) = p_next_free[*((unsigned char *)chunkfun + v158)];
      ++v158;
    }
    while ( v158 != 256 );
  }
  return v242 - __readfsqword(0x28u);
}



// Function: kwsexec @ 0xe3e0
long long kwsexec(long long a1, long long a2, long long a3, long long a4, unsigned char a5)
{
  return (*(long long (**)(long long, long long, long long, long long, long long))(a1 + 2456))(a1, a2, a3, a4, a5);
}



// Function: wordchars_count @ 0xe420
long long wordchars_count(long long a1, long long a2, char a3)
{
  long long v3; // r14
  long long v5; // rbx
  long long v6; // rax
  long long v7; // rax
  long long v8; // rdx
  int v9; // eax
  long long v11; // [rsp+8h] [rbp-60h]
  wint_t wc; // [rsp+1Ch] [rbp-4Ch] BYREF
  long long v13[9]; // [rsp+20h] [rbp-48h] BYREF

  v3 = 0;
  v5 = a2 - a1;
  v13[1] = __readfsqword(0x28u);
  v13[0] = 0;
  while ( v5 > v3 )
  {
    v6 = *(unsigned char *)(a1 + v3);
    if ( byte_2B9C0[v6] )
    {
      ++v3;
    }
    else
    {
      if ( byte_2B180[v6 + 3] != -2 )
        return v3;
      wc = 0;
      v7 = sub_1ACC0(&wc, a1 + v3, v5 - v3, v13);
      v8 = v7;
      if ( wc != 95 )
      {
        v11 = v7;
        v9 = iswalnum(wc);
        v8 = v11;
        if ( !v9 )
          return v3;
      }
      v3 += v8 + (v8 == 0);
    }
    if ( !a3 )
      return v3;
  }
  return v3;
}



// Function: wordinit @ 0xe570
bool wordinit()
{
  bool *v0; // rbp
  wint_t *v1; // rbx
  bool result; // al

  v0 = byte_2B9C0;
  v1 = (wint_t *)&unk_2B284;
  do
  {
    result = 1;
    if ( *v1 != 95 )
      result = iswalnum(*v1) != 0;
    ++v1;
    *v0++ = result;
  }
  while ( v1 != (wint_t *)&byte_2B684 );
  return result;
}



// Function: kwsinit @ 0xe5c0
long long kwsinit(char a1)
{
  long long v3; // rdi
  const int_t *v4; // rcx
  long long i; // rax

  if ( !byte_2B687 )
    return sub_C880(0);
  if ( __ctype_get_mb_cur_max() == 1 || (v3 = 0, a1) )
  {
    v3 = sub_1EF30(256);
    v4 = *__ctype_toupper_loc();
    for ( i = 0; i != 256; ++i )
      *(char *)(v3 + i) = v4[i];
  }
  return sub_C880(v3);
}



// Function: mb_goback @ 0xe640
signed long long mb_goback(unsigned char **a1, long long *a2, unsigned char *a3, long long a4)
{
  unsigned char *v5; // rbx
  unsigned char *v7; // rax
  long long v8; // rdx
  unsigned long long v9; // rax
  char v11; // al
  long long v12; // rdx
  unsigned char *v13; // r14
  long long v14; // rdx
  long long v15[9]; // [rsp+0h] [rbp-48h] BYREF

  v5 = *a1;
  v15[1] = __readfsqword(0x28u);
  if ( v5 >= a3 )
    return a3 - v5;
  if ( !byte_2B182 )
  {
    v15[0] = 0;
    while ( 1 )
    {
      v8 = byte_2B180[*v5 + 3];
      if ( (char)v8 == 0xFE && (v9 = sub_1AC90(v5, a4 - (long long)v5, v15), v8 = v9, v9 <= 0x10) || (v9 = v8, v8 >= 0) )
      {
        v7 = &v5[v9];
        if ( a3 <= v7 )
          goto LABEL_9;
      }
      else
      {
        v15[0] = 0;
        v8 = 1;
        v7 = v5 + 1;
        if ( a3 <= v5 + 1 )
        {
LABEL_9:
          if ( a2 )
            *a2 = v8;
          goto LABEL_11;
        }
      }
      v5 = v7;
    }
  }
  if ( (*a3 & 0xC0) != 0x80 )
    goto LABEL_14;
  v11 = *(a3 - 1);
  if ( (v11 & 0xC0) == 0x80 )
  {
    v11 = *(a3 - 2);
    if ( (v11 & 0xC0) == 0x80 )
    {
      v11 = *(a3 - 3);
      if ( (v11 & 0xC0) == 0x80 )
        goto LABEL_14;
      v12 = 3;
    }
    else
    {
      v12 = 2;
    }
  }
  else
  {
    v12 = 1;
  }
  if ( !((int)(unsigned char)~v11 >> (7 - v12)) )
  {
    v15[0] = 0;
    v13 = &a3[-v12];
    v14 = sub_1AC90(v13, a4 - (long long)v13, v15);
    if ( (unsigned long long)v14 <= 0x10 || (v7 = a3, v14 >= 0) )
    {
      v7 = &v13[v14];
      v5 = v13;
    }
LABEL_11:
    *a1 = v7;
    if ( v7 == a3 )
      return 0;
    return a3 - v5;
  }
LABEL_14:
  *a1 = a3;
  return 0;
}



// Function: wordchars_size @ 0xe810
long long wordchars_size(long long a1, long long a2)
{
  return sub_E420(a1, a2, 1);
}



// Function: wordchar_prev @ 0xe830
long long wordchar_prev(unsigned char *a1, unsigned char *a2, long long a3)
{
  long long v4; // rdx
  signed long long v6; // rax
  unsigned char *v7[5]; // [rsp+0h] [rbp-28h] BYREF

  v7[1] = (unsigned char *)__readfsqword(0x28u);
  if ( a1 == a2 )
    return 0;
  v4 = *(a2 - 1);
  if ( !byte_2B180[0] || ((unsigned char)byte_2B182 & (unsigned char)~((unsigned char)v4 >> 7)) != 0 )
    return (unsigned char)byte_2B9C0[v4];
  v7[0] = a1;
  v6 = sub_E640(v7, 0, a2 - 1, a3);
  return sub_E420((long long)&a2[-v6 - 1], a3, 0);
}



// Function: private_malloc @ 0xe900
long long private_malloc(long long a1)
{
  if ( a1 < 0 )
    sub_1F410(a1);
  return sub_1EF30(a1);
}



// Function: jit_exec @ 0xe920
long long jit_exec(long long *a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v9; // eax
  unsigned int matched; // r15d
  long long v11; // r9
  long long v13; // r15
  long long v14; // rdi
  long long v15; // rax
  long long v16; // rdx
  long long v17; // rdi
  long long v18; // rax
  long long v19; // [rsp+8h] [rbp-50h]
  int v20; // [rsp+14h] [rbp-44h] BYREF
  unsigned long long v21; // [rsp+18h] [rbp-40h]

  v21 = __readfsqword(0x28u);
  while ( 1 )
  {
    while ( 1 )
    {
      matched = pcre2_match_8(a1[1], a2, a3, a4, a5, a1[3], a1[2]);
      if ( matched != -46 )
        break;
      v11 = a1[5];
      if ( v11 > 0x3FFFFFFFFFFFFFFFLL )
        return matched;
      v13 = 2 * v11;
      v14 = a1[4];
      v19 = a1[5];
      a1[5] = 2 * v11;
      ((void (*)(long long, long long))pcre2_jit_stack_free_8)(v14, a2);
      v15 = pcre2_jit_stack_create_8(v19, v13, *a1);
      a1[4] = v15;
      v16 = v15;
      if ( !v15 )
        sub_1F410(v19);
      v17 = a1[2];
      if ( !v17 )
      {
        v18 = pcre2_match_context_create_8(*a1);
        v16 = a1[4];
        a1[2] = v18;
        v17 = v18;
      }
      pcre2_jit_stack_assign_8(v17, 0, v16);
    }
    if ( matched != -53 )
      break;
    pcre2_config_8(7, &v20);
    v9 = v20 < 0;
    v20 *= 2;
    if ( v9 )
      break;
    if ( !a1[2] )
      a1[2] = pcre2_match_context_create_8(*a1);
    pcre2_set_depth_limit_8();
  }
  return matched;
}



// Function: Pcompile @ 0xea80
long long Pcompile(char *src, size_t n)
{
  char *v2; // r14
  unsigned int v3; // r13d
  long long v4; // r12
  unsigned int v5; // r13d
  long long v6; // rax
  long long v7; // rbp
  long long v8; // r15
  void *v9; // r10
  long long v10; // rax
  long long v11; // rax
  long long v12; // rdi
  long long v13; // rax
  long long v14; // rdi
  unsigned int v15; // r13d
  unsigned long long v16; // rax
  unsigned long long v17; // rdx
  short *v19; // rax
  char *v20; // rax
  char *v21; // rax
  char *v22; // rax
  size_t v23; // [rsp+0h] [rbp-168h]
  void *ptr; // [rsp+8h] [rbp-160h]
  char *ptra; // [rsp+8h] [rbp-160h]
  unsigned int v26; // [rsp+14h] [rbp-154h] BYREF
  char v27[8]; // [rsp+18h] [rbp-150h] BYREF
  char v28[264]; // [rsp+20h] [rbp-148h] BYREF
  unsigned long long v29; // [rsp+128h] [rbp-40h]

  v2 = src;
  v29 = __readfsqword(0x28u);
  v23 = n;
  v3 = byte_2B687 == 0 ? 0xFFFFFFF8 : 0;
  v4 = sub_1EF30(56);
  v5 = v3 + 24;
  v6 = pcre2_general_context_create_8(sub_E900, j_free_0, 0);
  *(long long *)v4 = v6;
  v7 = v6;
  v8 = pcre2_compile_context_create_8(v6);
  if ( byte_2B180[0] )
  {
    if ( !byte_2B182 )
    {
LABEL_14:
      v21 = dcgettext(0, "-P supports only unibyte and UTF-8 locales", 5);
      error(2, 0, v21);
      goto LABEL_15;
    }
    v5 |= 0x4080000u;
  }
  if ( &src[n] != rawmemchr(src, 10) )
    goto LABEL_16;
  if ( byte_2B685 )
  {
    pcre2_set_compile_extra_options_8(v8, 8);
    v9 = 0;
  }
  else
  {
    v9 = 0;
    if ( byte_2B686 )
    {
      v23 = n + 17;
      ptra = (char *)sub_1EF30(n + 17);
      qmemcpy(ptra, "(?<!\\w)(?:", 10);
      v19 = mempcpy(ptra + 10, src, n);
      v9 = ptra;
      *(int *)v19 = 557787177;
      v2 = ptra;
      v19[2] = 30556;
      *((char *)v19 + 6) = 41;
    }
  }
  ptr = v9;
  v10 = pcre2_maketables_8(v7);
  pcre2_set_character_tables_8(v8, v10);
  v11 = pcre2_compile_8(v2, v23, v5, &v26, v27, v8);
  *(long long *)(v4 + 8) = v11;
  if ( !v11 )
  {
LABEL_15:
    pcre2_get_error_message_8(v26, v28, 256);
    error(2, 0, "%s", v28);
LABEL_16:
    v22 = dcgettext(0, "the -P option only supports a single pattern", 5);
    error(2, 0, v22);
  }
  free(ptr);
  pcre2_compile_context_free_8(v8);
  v12 = *(long long *)(v4 + 8);
  *(long long *)(v4 + 16) = 0;
  v13 = pcre2_match_data_create_from_pattern_8(v12, v7);
  v14 = *(long long *)(v4 + 8);
  *(long long *)(v4 + 24) = v13;
  v26 = pcre2_jit_compile_8(v14, 1);
  v15 = v26;
  v16 = v26 + 48;
  if ( (unsigned int)v16 > 0x30 || (v17 = 0xFFFEFFFFFFFFFFF6LL, _bittest64((const long long *)&v17, v16)) )
  {
    v20 = dcgettext(0, "JIT internal error: %d", 5);
    error(2, 0, v20, v15);
    goto LABEL_14;
  }
  *(long long *)(v4 + 32) = 0;
  *(long long *)(v4 + 40) = 0x8000;
  *(int *)(v4 + 48) = sub_E920((long long *)v4, (long long)"", 0, 0, 1u);
  *(int *)(v4 + 52) = sub_E920((long long *)v4, (long long)"", 0, 0, 0);
  return v4;
}



// Function: Pexecute @ 0xeda0
long long Pexecute(long long *a1, char *a2, long long a3, long long *a4, char *a5)
{
  char *v6; // rbx
  long long v7; // rdi
  bool v8; // bp
  char *v9; // r14
  long long v10; // rax
  long long v11; // r15
  long long v12; // rcx
  int v13; // r8d
  char *v14; // r10
  long long v16; // r12
  char *v17; // rax
  long long v18; // r12
  char *v19; // rax
  long long v20; // r12
  char *v21; // rax
  long long v22; // r12
  char *v23; // rax
  long long v24; // r12
  char *v25; // rax
  long long v26; // r12
  char *v27; // rax
  long long v28; // r12
  char *v29; // rax
  long long v30; // [rsp+0h] [rbp-68h]
  unsigned int v31; // [rsp+0h] [rbp-68h]
  long long *ovector_pointer_8; // [rsp+18h] [rbp-50h]

  v6 = a2;
  v7 = a1[3];
  if ( a5 )
    v6 = a5;
  v8 = *(v6 - 1) == byte_2B684;
  v30 = (long long)a2;
  ovector_pointer_8 = (long long *)pcre2_get_ovector_pointer_8(v7);
  while ( 1 )
  {
    v9 = (char *)rawmemchr(v6, byte_2B684);
    if ( byte_2B180[(unsigned char)*v6 + 3] == -1 )
    {
      do
        v10 = (unsigned char)*++v6;
      while ( byte_2B180[v10 + 3] == -1 );
      v11 = (long long)v6;
      v12 = 0;
      v8 = 0;
    }
    else
    {
      v11 = v30;
      v12 = (long long)&v6[-v30];
    }
    if ( v9 == v6 )
      break;
    v13 = sub_E920(a1, v11, (long long)&v9[-v11], v12, !v8);
    if ( v13 != -1 )
      goto LABEL_12;
LABEL_9:
    v6 = v9 + 1;
    v8 = 1;
    if ( v9 + 1 >= &a2[a3] )
      return -1;
    v30 = (long long)(v9 + 1);
  }
  v13 = *((int *)a1 + v8 + 12);
  ovector_pointer_8[1] = v12;
  *ovector_pointer_8 = v12;
  if ( v13 == -1 )
    goto LABEL_9;
LABEL_12:
  if ( v13 <= 0 )
  {
    if ( v13 < -45 )
      goto LABEL_20;
    while ( 1 )
    {
      v31 = v13;
      v16 = sub_90F0();
      v17 = dcgettext(0, "%s: internal PCRE error: %d", 5);
      error(2, 0, v17, v16, v31);
LABEL_20:
      if ( v13 >= -63 )
      {
        switch ( v13 )
        {
          case -63:
            goto LABEL_27;
          case -53:
            goto LABEL_26;
          case -52:
            goto LABEL_25;
          case -48:
            goto LABEL_24;
          case -47:
            goto LABEL_23;
          case -46:
            v18 = sub_90F0();
            v19 = dcgettext(0, "%s: exhausted PCRE JIT stack", 5);
            error(2, 0, v19, v18);
LABEL_23:
            v20 = sub_90F0();
            v21 = dcgettext(0, "%s: exceeded PCRE's backtracking limit", 5);
            error(2, 0, v21, v20);
LABEL_24:
            v22 = sub_90F0();
            v23 = dcgettext(0, "%s: memory exhausted", 5);
            error(2, 0, v23, v22);
LABEL_25:
            v24 = sub_90F0();
            v25 = dcgettext(0, "%s: PCRE detected recurse loop", 5);
            error(2, 0, v25, v24);
LABEL_26:
            v26 = sub_90F0();
            v27 = dcgettext(0, "%s: exceeded PCRE's nested backtracking limit", 5);
            error(2, 0, v27, v26);
LABEL_27:
            v28 = sub_90F0();
            v29 = dcgettext(0, "%s: exceeded PCRE's heap limit", 5);
            error(2, 0, v29, v28);
            sub_A170(1);
          default:
            continue;
        }
      }
    }
  }
  v14 = v9 + 1;
  if ( a5 )
  {
    v14 = (char *)(v11 + ovector_pointer_8[1]);
    v30 = v11 + *ovector_pointer_8;
  }
  *a4 = &v14[-v30];
  return v30 - (long long)a2;
}


