// Function: main @ 0x2900
long long main(int a1, char **a2, char **a3)
{
  const char *v3; // r13
  char **v5; // rbx
  char *v6; // rdi
  int v7; // eax
  char *v8; // rax
  long long v9; // r12
  char *v10; // rax
  const char **v11; // rbx
  long long v12; // rcx
  long long v13; // r8
  long long v14; // r9
  char *v15; // rax
  long long v16; // rcx
  long long v17; // r8
  long long v18; // r9
  char *v19; // rax
  long long v20; // rcx
  long long v21; // r8
  long long v22; // r9
  FILE *v23; // rbp
  char *v24; // rax
  const char *v25; // rsi
  char *v26; // rax
  long long v27; // rcx
  long long v28; // r8
  long long v29; // r9
  char *v30; // rbx
  char *v31; // r12
  char *v32; // rax
  long long v33; // r9
  const char *v34; // rdx
  const char *v35; // rcx
  const char *v36; // r8
  FILE *v38; // rdi
  int v39; // eax
  long long v40; // rsi
  long long v41; // rcx
  char *v42; // rcx
  char *v43; // rax
  char *v44; // rax
  unsigned long long v45; // r13
  char *v46; // rbp
  long long i; // rbx
  long long v48; // r12
  char *v49; // r14
  int v50; // edi
  int v51; // eax
  unsigned long long v52; // rbx
  long long v53; // rax
  long long v54; // r12
  unsigned long long v55; // rax
  size_t v56; // r12
  char *v57; // rsi
  int v58; // eax
  unsigned int v59; // edx
  const char *v60; // r10
  char *v61; // rax
  char *v62; // rdi
  long long *j; // rcx
  long long v64; // rax
  char v65; // r8
  char *v66; // r11
  size_t v67; // r11
  size_t v68; // r15
  size_t v69; // rbx
  const char *v70; // r15
  unsigned int v71; // r13d
  long long v72; // rax
  long long v73; // rbx
  long long v74; // r12
  long long v75; // rax
  int v76; // ebp
  long long v77; // rdi
  char *v78; // rdi
  char *v79; // rbx
  long long k; // rbp
  char *v81; // rax
  long long v82; // rdx
  long long v83; // r15
  long long v84; // rax
  int *v85; // rax
  long long v86; // rax
  char *v87; // rax
  const char *v88; // rdx
  const char *v89; // rcx
  const char *v90; // r8
  const char *v91; // r9
  const char *v92; // rbp
  char *v93; // rcx
  long long v94; // rdx
  unsigned char v95; // r12
  char *v96; // r12
  long long v97; // r15
  int v98; // eax
  long long v99; // rbx
  long long v100; // rbx
  long long v101; // rbx
  long long v102; // rbx
  int *v103; // rax
  long long v104; // rbp
  long long v105; // rax
  long long v106; // rbx
  long long v107; // rbp
  long long v108; // r8
  long long v109; // rax
  long long v110; // rdx
  long long v111; // rax
  char v112; // al
  long long v113; // r12
  char *v114; // rdx
  char *v115; // rax
  char *v116; // rax
  int *v117; // rax
  int *v118; // rax
  const char *v119; // [rsp+8h] [rbp-210h]
  unsigned char v120; // [rsp+8h] [rbp-210h]
  long long v121; // [rsp+10h] [rbp-208h]
  int v122; // [rsp+18h] [rbp-200h]
  long long v123; // [rsp+18h] [rbp-200h]
  char *v124; // [rsp+20h] [rbp-1F8h]
  unsigned int v125; // [rsp+2Ch] [rbp-1ECh]
  long long v126; // [rsp+30h] [rbp-1E8h]
  char **v127; // [rsp+38h] [rbp-1E0h]
  char v128; // [rsp+38h] [rbp-1E0h]
  const char *v129; // [rsp+40h] [rbp-1D8h]
  char *v130; // [rsp+40h] [rbp-1D8h]
  char **v131; // [rsp+48h] [rbp-1D0h]
  long long v132; // [rsp+50h] [rbp-1C8h]
  bool v133; // [rsp+5Bh] [rbp-1BDh]
  int v134; // [rsp+5Ch] [rbp-1BCh]
  long long v135; // [rsp+68h] [rbp-1B0h] BYREF
  struct stat buf; // [rsp+70h] [rbp-1A8h] BYREF
  struct stat v137; // [rsp+100h] [rbp-118h] BYREF
  char v138[5]; // [rsp+196h] [rbp-82h] BYREF
  char v139[5]; // [rsp+19Bh] [rbp-7Dh] BYREF
  char v140[32]; // [rsp+1A0h] [rbp-78h] BYREF
  char v141[24]; // [rsp+1C0h] [rbp-58h] BYREF
  unsigned long long v142; // [rsp+1D8h] [rbp-40h]

  v3 = "bci:ln:sv";
  v5 = a2;
  v6 = *a2;
  v142 = __readfsqword(0x28u);
  status = 2;
  sub_4080(v6);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_3E90(0);
  sub_6870();
  v119 = (const char *)&v135;
  while ( 2 )
  {
    v7 = getopt_long(a1, a2, "bci:ln:sv", &longopts, 0);
    if ( v7 != -1 )
    {
      switch ( v7 )
      {
        case 98:
        case 99:
          byte_D0B2 = 1;
          continue;
        case 105:
          sub_3AE0(0, &optarg, 58);
          v8 = optarg++;
          if ( *v8 == 58 )
          {
            sub_3AE0(1, &optarg, 0);
          }
          else if ( qword_D0C8 < qword_D0C0[0] )
          {
            qword_D0C8 = qword_D0C0[0];
          }
          continue;
        case 108:
          sub_3C60(1);
          continue;
        case 110:
          if ( (unsigned int)sub_6940(optarg) || v135 < 0 )
            sub_3A80("invalid --bytes value '%s'");
          if ( qword_D010 < 0 || v135 <= qword_D010 )
            qword_D010 = v135;
          continue;
        case 115:
          sub_3C60(3);
          continue;
        case 118:
          sub_4520("David MacKenzie");
          sub_45B0("Torbjorn Granlund", aTorbj);
          v38 = stdout;
          sub_61B0(stdout, 0);
          sub_3C90(v38, "cmp");
          return 0;
        case 128:
          v9 = qword_1D280;
          v10 = dcgettext(0, "Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n", 5);
          v11 = (const char **)off_CA00;
          __printf_chk(1, v10, v9, v12, v13, v14);
          v15 = dcgettext(0, "Compare two files byte by byte.", 5);
          __printf_chk(1, "%s\n", v15, v16, v17, v18);
          v19 = dcgettext(
                  0,
                  "The optional SKIP1 and SKIP2 specify the number of bytes to skip\n"
                  "at the beginning of each file (zero by default).",
                  5);
          __printf_chk(1, "\n%s\n\n", v19, v20, v21, v22);
          v23 = stdout;
          v24 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
          fputs_unlocked(v24, v23);
          v25 = "-b, --print-bytes          print differing bytes";
          do
          {
            ++v11;
            v26 = dcgettext(0, v25, 5);
            __printf_chk(1, "  %s\n", v26, v27, v28, v29);
            v25 = *v11;
          }
          while ( *v11 );
          v30 = dcgettext(0, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
          v31 = dcgettext(0, "If a FILE is '-' or missing, read standard input.", 5);
          v32 = dcgettext(
                  0,
                  "SKIP values may be followed by the following multiplicative suffixes:\n"
                  "kB 1000, K 1024, MB 1,000,000, M 1,048,576,\n"
                  "GB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",
                  5);
          __printf_chk(1, "\n%s\n\n%s\n%s\n", v32, v31, v30, v33);
          sub_6290();
          sub_3C90(1, "\n%s\n\n%s\n%s\n", v34, v35, v36);
          return 0;
        default:
          v40 = 0;
          v39 = sub_3A80(0);
          goto LABEL_73;
      }
    }
    break;
  }
  v39 = optind;
  if ( optind == a1 )
  {
    sub_3A80("missing operand after '%s'");
    goto LABEL_175;
  }
  v40 = optind;
  v41 = (long long)v5[v40];
  ++optind;
  qword_D230[0] = v41;
  v42 = (char *)"-";
  if ( v39 + 1 < a1 )
  {
LABEL_73:
    v42 = v5[v40 + 1];
    optind = v39 + 2;
  }
  s2 = v42;
  if ( a1 > optind )
  {
    v43 = v5[optind++];
    v135 = (long long)v43;
    sub_3AE0(0, &v135, 0);
    if ( optind < a1 )
    {
      v44 = v5[optind++];
      v135 = (long long)v44;
      sub_3AE0(1, &v135, 0);
      if ( optind < a1 )
      {
        sub_3A80("extra operand '%s'");
LABEL_187:
        v117 = __errno_location();
        error(2, *v117, "%s", (const char *)qword_D230[0]);
LABEL_188:
        v118 = __errno_location();
        error(2, *v118, "%s", s2);
        start();
      }
    }
  }
  v45 = 0;
  v46 = (char *)"-";
  i = (long long)&unk_D100;
LABEL_27:
  v48 = (int)v45;
  if ( !v45 || qword_D0C0[0] != qword_D0C8 || strcmp((const char *)qword_D230[0], s2) )
  {
    v49 = (char *)qword_D230[v45];
    v50 = strcmp(v49, v46);
    if ( !v50 )
    {
      *(&fd + v45) = 0;
      goto LABEL_31;
    }
    while ( 1 )
    {
      v50 = open(v49, 0, 0);
      *(&fd + v45) = v50;
      if ( v50 < 0 )
      {
        if ( dword_D0B4 == 3 )
          exit(2);
LABEL_156:
        v103 = __errno_location();
        error(2, *v103, "%s", (const char *)qword_D230[v48]);
LABEL_157:
        v104 = qword_D130;
        v105 = sub_3B80(0);
        v106 = qword_D1C0;
        v107 = v104 - v105;
        v108 = sub_3B80(1);
        v109 = 0;
        if ( v107 >= 0 )
          v109 = v107;
        v110 = v109;
        v111 = v106 - v108;
        if ( v106 - v108 < 0 )
          v111 = 0;
        if ( v111 != v110 )
        {
          if ( qword_D010 < 0 )
            goto LABEL_166;
          if ( v111 > v110 )
            v111 = v110;
          if ( qword_D010 > v111 )
LABEL_166:
            exit(1);
        }
        goto LABEL_39;
      }
LABEL_31:
      if ( fstat(v50, (struct stat *)(i + 144 * v45)) )
        goto LABEL_156;
      if ( v45 != 1 )
      {
        v45 = 1;
        goto LABEL_27;
      }
      v134 = 0;
      if ( qword_D108 == qword_D198 && *(long long *)i == qword_D190 )
        goto LABEL_191;
      v51 = dword_D118 & 0xF000;
      if ( v51 == 24576 )
      {
        if ( (dword_D1A8 & 0xF000) != 0x6000 )
          goto LABEL_92;
      }
      else
      {
        if ( v51 != 0x2000 )
          goto LABEL_36;
        if ( (dword_D1A8 & 0xF000) != 0x2000 )
        {
LABEL_92:
          if ( dword_D0B4 == 3 )
            goto LABEL_39;
LABEL_93:
          if ( !fstat(1, &buf) && !stat("/dev/null", &v137) )
          {
            if ( buf.st_ino == v137.st_ino && buf.st_dev == v137.st_dev )
            {
LABEL_149:
              dword_D0B4 = 2;
              goto LABEL_94;
            }
            v98 = buf.st_mode & 0xF000;
            if ( v98 == 24576 )
            {
              if ( (v137.st_mode & 0xF000) == 0x6000 )
              {
LABEL_148:
                if ( buf.st_rdev == v137.st_rdev )
                  goto LABEL_149;
              }
            }
            else if ( v98 == 0x2000 && (v137.st_mode & 0xF000) == 0x2000 )
            {
              goto LABEL_148;
            }
          }
LABEL_94:
          if ( dword_D0B4 != 3 )
            goto LABEL_39;
          goto LABEL_37;
        }
      }
      if ( qword_D128 == qword_D1B8 )
      {
LABEL_191:
        if ( dword_D118 == dword_D1A8
          && qword_D110 == qword_D1A0
          && dword_D11C == dword_D1AC
          && dword_D120 == dword_D1B0
          && qword_D130 == qword_D1C0
          && qword_D158 == qword_D1E8
          && qword_D168 == qword_D1F8 )
        {
          v73 = sub_3B80(0);
          if ( v73 == sub_3B80(1) )
            return 0;
        }
      }
LABEL_36:
      if ( dword_D0B4 != 3 )
        goto LABEL_93;
LABEL_37:
      if ( (dword_D118 & 0xF000) == 0x8000 && (dword_D1A8 & 0xF000) == 0x8000 )
        goto LABEL_157;
LABEL_39:
      qword_D0D0 = sub_7070(qword_D138, qword_D1C8, 0x7FFFFFFFFFFFFFF7LL);
      v52 = (qword_D0D0 + 15) & 0xFFFFFFFFFFFFFFF8LL;
      v53 = sub_63B0(16 * ((unsigned long long)(qword_D0D0 + 15) >> 3));
      v49 = (char *)(v53 + v52);
      qword_D0E0 = v53;
      v46 = (char *)v53;
      qword_D0E8 = v53 + v52;
      v121 = qword_D010;
      if ( dword_D0B4 == 1 )
      {
        v74 = 0x7FFFFFFFFFFFFFFFLL;
        if ( qword_D010 >= 0 )
          v74 = qword_D010;
        if ( (dword_D118 & 0xF000) == 0x8000 )
        {
          v101 = qword_D130;
          v102 = v101 - sub_3B80(0);
          if ( v74 > v102 )
            v74 = v102;
        }
        if ( (dword_D1A8 & 0xF000) == 0x8000 )
        {
          v99 = qword_D1C0;
          v100 = v99 - sub_3B80(1);
          if ( v74 > v100 )
            v74 = v100;
        }
        v125 = 1;
        v75 = v74;
        while ( 1 )
        {
          v75 /= 10;
          if ( !v75 )
            break;
          ++v125;
        }
      }
      for ( i = 0; ; i = 1 )
      {
        v48 = (int)i;
        v45 = qword_D0C0[i];
        if ( v45 )
        {
          if ( sub_3B80((unsigned int)i) == -1 )
          {
            while ( 1 )
            {
              v83 = qword_D0D0;
              if ( v45 <= qword_D0D0 )
                v83 = v45;
              v84 = sub_6FC0(*(&fd + i), v46);
              if ( v83 != v84 )
                break;
              v45 -= v83;
              if ( !v45 )
                goto LABEL_42;
            }
            if ( v84 == -1 )
              break;
          }
        }
LABEL_42:
        if ( i == 1 )
        {
          v122 = 0;
          v119 = (char *)(&dword_0 + 1);
          v126 = 1;
          v133 = 1;
          while ( 1 )
          {
            v54 = qword_D0D0;
            if ( v121 >= 0 )
            {
              if ( qword_D0D0 > (unsigned long long)v121 )
                v54 = v121;
              v121 -= v54;
            }
            v5 = (char **)sub_6FC0(fd, v46);
            if ( v5 == (char **)-1LL )
              goto LABEL_187;
            v55 = sub_6FC0(dword_D224, v49);
            v3 = (const char *)v55;
            if ( v55 == -1 )
              goto LABEL_188;
            v56 = v55;
            v57 = v49;
            if ( (unsigned long long)v5 <= v55 )
              v56 = (size_t)v5;
            v58 = memcmp(v46, v49, v56);
            v59 = dword_D0B4;
            v60 = v119;
            LOBYTE(v57) = dword_D0B4 == 0;
            if ( v58 )
            {
              v61 = (char *)v5 + (long long)v49;
              if ( v5 < (char **)v3 || (*v61 = 85, v5 == (char **)v3) )
                v3[(long long)v46] = 121;
              *((char *)v5 + (long long)v46) = ~*v61;
              v3[(long long)v49] = ~v3[(long long)v46];
              memset((char *)v5 + (long long)v46 + 1, 0, ~(char)v5 & 7);
              memset((void *)&v3[(long long)v49 + 1], 0, ~(char)v3 & 7);
              v62 = v49;
              for ( j = v46; *j == *(long long *)v62; ++j )
                v62 += 8;
              v64 = 0;
              do
              {
                v65 = *((char *)j + v64);
                v66 = (char *)j + v64++;
              }
              while ( v65 == v62[v64 - 1] );
              v67 = v66 - v46;
              v119 += v67;
              v68 = v67;
              if ( v67 && (char)v57 )
                goto LABEL_109;
              if ( v56 > v67 )
              {
                if ( v59 == 2 )
                {
                  if ( v5 != (char **)v3 )
                    goto LABEL_115;
                  v122 = 1;
LABEL_99:
                  v76 = v122 != 0;
                  while ( 1 )
                  {
LABEL_100:
                    if ( !close(fd) )
                    {
                      v77 = (unsigned int)dword_D224;
                      if ( !close(dword_D224) )
                      {
                        if ( v76 && (unsigned int)dword_D0B4 <= 1 )
                          sub_3C90(v77, v57);
                        exit(v76);
                      }
LABEL_175:
                      v134 = 1;
                    }
                    v57 = (char *)(unsigned int)*__errno_location();
                    error(2, (int)v57, "%s", (const char *)qword_D230[v134]);
LABEL_177:
                    if ( dword_D0B4 == 3 )
                      goto LABEL_115;
                    v113 = qword_D230[v5 > (char **)v3];
                    if ( v119 != (char *)&dword_0 + 1 )
                      break;
                    v76 = 1;
                    v115 = dcgettext(0, "cmp: EOF on %s which is empty\n", 5);
                    v57 = (char *)(&dword_0 + 1);
                    __fprintf_chk(stderr, 1, v115, v113);
                  }
                  v5 = (char **)sub_3FE0(v119 - 1, v140);
                  if ( dword_D0B4 )
                  {
                    v116 = dcgettext(0, "cmp: EOF on %s after byte %s\n", 5);
                    v57 = (char *)(&dword_0 + 1);
                    __fprintf_chk(stderr, 1, v116, v113);
                  }
                  else
                  {
                    sub_3FE0(v126 - v133, v141);
                    if ( v133 )
                      v114 = dcgettext(0, "cmp: EOF on %s after byte %s, line %s\n", 5);
                    else
                      v114 = dcgettext(0, "cmp: EOF on %s after byte %s, in line %s\n", 5);
                    v57 = (char *)(&dword_0 + 1);
                    __fprintf_chk(stderr, 1, v114, v113);
                  }
LABEL_115:
                  v76 = 1;
                  goto LABEL_100;
                }
                if ( v59 > 2 )
                {
                  if ( v59 == 3 )
                    goto LABEL_115;
                }
                else
                {
                  if ( !v59 )
                    goto LABEL_135;
                  v127 = v5;
                  v69 = v67;
                  v70 = v60;
                  v129 = v3;
                  do
                  {
                    v71 = (unsigned char)v49[v69];
                    v120 = v46[v69];
                    if ( v120 != (char)v71 )
                    {
                      v72 = sub_3FE0(&v70[v69], v141);
                      if ( byte_D0B2 )
                      {
                        v132 = v72;
                        sub_3BD0(v139, v120);
                        sub_3BD0(v140, v71);
                        v57 = "%*s %3o %-4s %3o %s\n";
                        __printf_chk(1, "%*s %3o %-4s %3o %s\n", v125, v132, v120, v139);
                      }
                      else
                      {
                        v57 = "%*s %3o %3o\n";
                        __printf_chk(1, "%*s %3o %3o\n", v125, v72, v120, v71);
                      }
                    }
                    ++v69;
                  }
                  while ( v56 != v69 );
                  v5 = v127;
                  v3 = v129;
                  v122 = -1;
                  v119 = &v70[v56];
                }
              }
            }
            else
            {
              v119 += v56;
              if ( v56 && !dword_D0B4 )
              {
                v68 = v56;
LABEL_109:
                v130 = v46;
                v78 = v46;
                v131 = v5;
                v79 = &v46[v68];
                v124 = &v46[v68];
                v128 = v46[v68];
                v46[v68] = 10;
                for ( k = 0; ; ++k )
                {
                  v57 = byte_9 + 1;
                  v81 = (char *)rawmemchr(v78, 10);
                  if ( v79 == v81 )
                    break;
                  v78 = v81 + 1;
                }
                v82 = k;
                v46 = v130;
                v126 += v82;
                *v124 = v128;
                v5 = v131;
                if ( v56 > v68 )
                {
LABEL_135:
                  v5 = (char **)sub_3FE0(v119, v140);
                  v86 = sub_3FE0(v126, v141);
                  if ( byte_D0B2 )
                  {
                    v95 = v49[v68];
                    v123 = v86;
                    sub_3BD0(v138, (unsigned char)v46[v68]);
                    v119 = v139;
                    sub_3BD0(v139, v95);
                    v96 = s2;
                    v97 = qword_D230[0];
                    v57 = dcgettext(0, "%s %s differ: byte %s, line %s is %3o %s %3o %s\n", 5);
                    __printf_chk(1, v57, v97, v96, v5, v123);
                  }
                  else
                  {
                    v119 = (const char *)v86;
                    v87 = dcgettext(0, "%s %s differ: byte %s, line %s\n", 5);
                    v91 = v119;
                    v92 = v87;
                    if ( v87 != "%s %s differ: byte %s, line %s\n"
                      || (v112 = sub_3F50(5, "%s %s differ: byte %s, line %s\n", v88, v89, v90, v119), v91 = v119, v112) )
                    {
                      v93 = s2;
                      v94 = qword_D230[0];
                    }
                    else
                    {
                      v93 = s2;
                      v94 = qword_D230[0];
                      v92 = "%s %s differ: char %s, line %s\n";
                    }
                    v57 = (char *)v92;
                    __printf_chk(1, v92, v94, v93, v5, v91);
                  }
                  goto LABEL_115;
                }
                v133 = v130[v68 - 1] == 10;
              }
            }
            if ( v5 != (char **)v3 )
              goto LABEL_177;
            if ( v5 != (char **)qword_D0D0 )
              goto LABEL_99;
          }
        }
      }
      v85 = __errno_location();
      error(2, *v85, "%s", (const char *)qword_D230[(int)i]);
    }
  }
  return 0;
}



// Function: try_help @ 0x3a80
long long try_help(char *msgid, long long a2)
{
  char *v2; // rax
  long long v3; // r12
  char *v4; // rax
  long long v5; // rdx

  if ( msgid )
  {
    v2 = dcgettext(0, msgid, 5);
    error(0, 0, v2, a2);
  }
  v3 = qword_1D280;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_3AE0(2, 0, v5);
}



// Function: specify_ignore_initial @ 0x3ae0
unsigned long long specify_ignore_initial(int a1, char **a2, char a3)
{
  char *v4; // r13
  int v5; // eax
  long long v7; // [rsp+0h] [rbp-38h]
  unsigned long long v8; // [rsp+8h] [rbp-30h]

  v4 = *a2;
  v8 = __readfsqword(0x28u);
  v5 = sub_6940(*a2);
  if ( v5 && (v5 != 2 || **a2 != a3) || v7 < 0 )
    sub_3A80("invalid --ignore-initial value '%s'", (long long)v4);
  if ( v7 > qword_D0C0[a1] )
    qword_D0C0[a1] = v7;
  return v8 - __readfsqword(0x28u);
}



// Function: file_position @ 0x3b80
__off_t file_position(int a1)
{
  __off_t result; // rax

  if ( byte_D0B0[a1] )
    return qword_D0A0[a1];
  byte_D0B0[a1] = 1;
  result = lseek(*(&fd + a1), qword_D0C0[a1], 1);
  qword_D0A0[a1] = result;
  return result;
}



// Function: sprintc @ 0x3bd0
long long sprintc(short *a1, unsigned int a2)
{
  unsigned int v2; // r12d
  char *v3; // rbp
  const unsigned short **v4; // r8
  long long result; // rax

  v2 = a2;
  v3 = a1;
  v4 = __ctype_b_loc();
  result = a2;
  if ( ((*v4)[(unsigned char)a2] & 0x4000) == 0 )
  {
    if ( (a2 & 0x80u) != 0 )
    {
      v2 = a2 - 128;
      v3 = a1 + 1;
      *a1 = 11597;
      if ( (unsigned char)(a2 + 0x80) <= 0x1Fu )
        goto LABEL_4;
    }
    else if ( (unsigned char)a2 <= 0x1Fu )
    {
LABEL_4:
      *v3 = 94;
      result = v2 + 64;
      ++v3;
      goto LABEL_5;
    }
    result = v2;
    if ( (char)v2 == 127 )
    {
      *v3 = 94;
      result = 63;
      ++v3;
    }
  }
LABEL_5:
  *v3 = result;
  v3[1] = 0;
  return result;
}


