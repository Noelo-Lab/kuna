// Function: main @ 0x3f80
long long main(int a1, char **a2, char **a3)
{
  char **v3; // r12
  long long v4; // rbx
  struct lconv *v5; // rax
  char *decimal_point; // rcx
  char *thousands_sep; // rdx
  int *v8; // r15
  int i; // ebp
  int v10; // r13d
  int *v11; // rbp
  long long v12; // r15
  int v13; // r13d
  const char *v14; // r14
  long long v15; // rax
  long long v16; // rcx
  long long *v17; // rbp
  long long v18; // rax
  char *v19; // rdx
  int v20; // edx
  char *v21; // rax
  long long v22; // rax
  long long v23; // rdx
  long long v24; // r8
  long long v25; // rdx
  char v26; // cl
  char v27; // al
  int v28; // eax
  long long v29; // rdx
  int v30; // eax
  unsigned long long v31; // rax
  char *v32; // rax
  long long v33; // rdx
  long long v34; // rcx
  long long v35; // r14
  char *v36; // rax
  char *v37; // rax
  char *v38; // rdx
  int v39; // eax
  char *v40; // rdx
  int v41; // edi
  int v42; // eax
  const char *v43; // rsi
  int v44; // eax
  long long v45; // rdx
  long long v46; // rdi
  const char *v47; // rdi
  char *v48; // rdx
  int v49; // eax
  long long v50; // rax
  const char *v51; // rdi
  char *v52; // rdx
  int v53; // eax
  long long v54; // rdx
  char *v55; // rax
  char *v56; // r8
  long long v57; // rcx
  char *v58; // rdi
  long long v59; // rdx
  bool v60; // zf
  char *v61; // rax
  long long v62; // rdx
  char *v63; // r8
  long long v64; // rcx
  char *v65; // rdx
  long long v66; // rcx
  char *v68; // rdi
  char *v69; // rax
  unsigned char v70; // al
  unsigned long long v71; // r8
  long long v72; // rax
  char *v73; // rax
  int v74; // r9d
  long long v75; // rdx
  char *v76; // rax
  char *v77; // r8
  char *v78; // rdi
  int v79; // r9d
  long long v80; // rax
  long long *v81; // rcx
  long long v82; // rdi
  long long v83; // rdx
  char *v84; // rax
  long long v85; // rax
  long long v86; // rax
  long long v87; // rax
  char *v88; // r8
  char *v89; // rdi
  long long v90; // rax
  long long v91; // r12
  char *v92; // rax
  char *v93; // rax
  char *v94; // rax
  char *v95; // r8
  char *v96; // rsi
  char *v97; // rax
  char *v98; // rax
  char *v99; // r8
  unsigned int v100; // ecx
  char *v101; // rdi
  long long v102; // rax
  char *v103; // rax
  long long v104; // rdx
  long long v105; // rax
  long long v106; // rax
  struct sigaction *v107; // rsi
  long long v108; // r12
  char *v109; // rax
  long long v110; // r12
  char *v111; // rax
  long long v112; // r12
  char *v113; // rax
  long long v114; // r12
  char *v115; // rax
  long long v116; // r12
  char *v117; // rax
  char *v118; // rax
  long long v119; // rax
  char *v120; // rax
  long long v121; // r12
  char *v122; // rax
  char *v123; // rax
  char *v124; // rax
  char *v125; // rax
  struct sigaction *act; // [rsp+0h] [rbp-2F8h]
  char *s1; // [rsp+8h] [rbp-2F0h]
  char *s1a; // [rsp+8h] [rbp-2F0h]
  char *v129; // [rsp+18h] [rbp-2E0h]
  char *v130; // [rsp+18h] [rbp-2E0h]
  unsigned long long v131; // [rsp+20h] [rbp-2D8h]
  char *v132; // [rsp+28h] [rbp-2D0h]
  long long *v133; // [rsp+28h] [rbp-2D0h]
  char *v134; // [rsp+28h] [rbp-2D0h]
  char *v135; // [rsp+30h] [rbp-2C8h]
  char *v136; // [rsp+38h] [rbp-2C0h]
  int v137; // [rsp+40h] [rbp-2B8h]
  unsigned int v138; // [rsp+40h] [rbp-2B8h]
  unsigned int v139; // [rsp+40h] [rbp-2B8h]
  unsigned int v140; // [rsp+40h] [rbp-2B8h]
  int v141; // [rsp+40h] [rbp-2B8h]
  unsigned int v142; // [rsp+40h] [rbp-2B8h]
  unsigned int v143; // [rsp+40h] [rbp-2B8h]
  int v144; // [rsp+40h] [rbp-2B8h]
  char *v145; // [rsp+40h] [rbp-2B8h]
  char *v146; // [rsp+40h] [rbp-2B8h]
  char *v147; // [rsp+40h] [rbp-2B8h]
  char v148; // [rsp+4Ah] [rbp-2AEh]
  char v149; // [rsp+4Bh] [rbp-2ADh]
  int v150; // [rsp+4Ch] [rbp-2ACh]
  unsigned int v151; // [rsp+4Ch] [rbp-2ACh]
  unsigned int v152; // [rsp+4Ch] [rbp-2ACh]
  unsigned int v153; // [rsp+4Ch] [rbp-2ACh]
  char v154[80]; // [rsp+50h] [rbp-2A8h] BYREF
  long long v155[6]; // [rsp+A0h] [rbp-258h] BYREF
  char v156; // [rsp+D0h] [rbp-228h]
  char v157; // [rsp+D1h] [rbp-227h]
  char v158; // [rsp+D2h] [rbp-226h]
  unsigned char v159; // [rsp+D3h] [rbp-225h]
  char v160; // [rsp+D4h] [rbp-224h]
  char v161; // [rsp+D5h] [rbp-223h]
  char v162; // [rsp+D6h] [rbp-222h]
  char v163; // [rsp+D7h] [rbp-221h]
  char v164; // [rsp+D8h] [rbp-220h]
  int longind[2]; // [rsp+F0h] [rbp-208h] BYREF
  long long *v166; // [rsp+F8h] [rbp-200h]
  struct sigaction oact; // [rsp+210h] [rbp-E8h] BYREF
  char v168[2]; // [rsp+2B6h] [rbp-42h] BYREF
  unsigned long long v169; // [rsp+2B8h] [rbp-40h]

  v3 = a2;
  LODWORD(v4) = a1;
  v169 = __readfsqword(0x28u);
  v136 = getenv("POSIXLY_CORRECT");
  v148 = (unsigned int)sub_EE70() - 200112 > 0x2B8;
  sub_EF00(*a2);
  v135 = setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_8F60(3);
  byte_1E861 = ((long long (*)(void))sub_CFB0)();
  byte_1E860 = sub_CFB0(2);
  v5 = localeconv();
  decimal_point = v5->decimal_point;
  byte_1E868 = *v5->decimal_point;
  if ( !byte_1E868 || decimal_point[1] )
    byte_1E868 = 46;
  thousands_sep = v5->thousands_sep;
  dword_1E864 = *thousands_sep;
  if ( dword_1E864 )
  {
    if ( !thousands_sep[1] )
      goto LABEL_5;
LABEL_63:
    byte_1E862 = 1;
  }
  dword_1E864 = 128;
LABEL_5:
  byte_1E418 = 0;
  sub_5B80();
  v8 = (int *)&unk_15324;
  sigemptyset(&newmask);
  for ( i = 14; ; i = *v8++ )
  {
    sigaction(i, 0, &oact);
    if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&newmask, i);
    if ( v8 == (int *)&unk_1534C )
      break;
  }
  oact.sa_flags = 0;
  oact.sa_handler = (__sighandler_t)sub_5B40;
  v10 = 14;
  *(__m128i *)oact.sa_mask.__val = _mm_load_si128((const __m128i *)&newmask);
  *(__m128i *)&oact.sa_mask.__val[2] = _mm_load_si128((const __m128i *)&newmask.__val[2]);
  v11 = (int *)&unk_15324;
  *(__m128i *)&oact.sa_mask.__val[4] = _mm_load_si128((const __m128i *)&newmask.__val[4]);
  *(__m128i *)&oact.sa_mask.__val[6] = _mm_load_si128((const __m128i *)&newmask.__val[6]);
  *(__m128i *)&oact.sa_mask.__val[8] = _mm_load_si128((const __m128i *)&newmask.__val[8]);
  *(__m128i *)&oact.sa_mask.__val[10] = _mm_load_si128((const __m128i *)&newmask.__val[10]);
  *(__m128i *)&oact.sa_mask.__val[12] = _mm_load_si128((const __m128i *)&newmask.__val[12]);
  *(__m128i *)&oact.sa_mask.__val[14] = _mm_load_si128((const __m128i *)&newmask.__val[14]);
  while ( !sigismember(&newmask, v10) )
  {
    if ( v11 == (int *)&unk_1534C )
      goto LABEL_15;
LABEL_12:
    v10 = *v11++;
  }
  sigaction(v10, &oact, 0);
  if ( v11 != (int *)&unk_1534C )
    goto LABEL_12;
LABEL_15:
  v12 = 0;
  v13 = 0;
  signal(17, 0);
  v14 = "-bcCdfghik:mMno:rRsS:t:T:uVy:z";
  sub_14980(sub_5AD0);
  sub_5CF0(v155, 8);
  v155[0] = -1;
  v15 = sub_12FD0((int)v4);
  act = 0;
  LODWORD(v16) = 0;
  s1 = 0;
  v17 = (long long *)v15;
  v131 = 0;
  v129 = 0;
  v149 = 0;
  while ( 1 )
  {
    while ( 1 )
    {
      longind[0] = -1;
      if ( (int)v16 == -1 )
        goto LABEL_25;
      if ( !v12 || !v136 )
        goto LABEL_29;
      v18 = optind;
      if ( v148 == 1 && !(char)v13 )
      {
        if ( optind == (int)v4 )
        {
          if ( act )
            goto LABEL_160;
          v13 = 0;
          goto LABEL_45;
        }
        v19 = v3[optind];
        if ( *v19 == 45 && v19[1] == 111 && (v19[2] || optind + 1 != (int)v4) )
        {
LABEL_29:
          LODWORD(v22) = getopt_long(v4, v3, v14, &longopts, longind);
          LODWORD(v16) = v22;
          if ( (int)v22 != -1 )
          {
            if ( (int)v22 > 135 )
              goto LABEL_161;
            if ( (int)v22 > 0 )
            {
              switch ( (int)v22 )
              {
                case 1:
                  v65 = optarg;
                  if ( *optarg != 43 )
                    goto LABEL_119;
                  if ( optind == (int)v4 )
                  {
                    v74 = 0;
                  }
                  else
                  {
                    v73 = v3[optind];
                    v74 = 0;
                    if ( *v73 == 45 )
                      v74 = (unsigned int)(v73[1] - 48) <= 9;
                  }
                  v148 |= v74 && v136 == 0;
                  if ( !v148 )
                    goto LABEL_119;
                  v150 = v16;
                  v141 = v74;
                  v132 = (char *)sub_5CF0(v154, v3);
                  v76 = (char *)sub_5D20(v75 + 1, v132, 0);
                  v77 = v132;
                  LODWORD(v16) = v150;
                  v78 = v76;
                  if ( !v76 )
                  {
                    if ( !*(long long *)v132 && !*((long long *)v132 + 1) )
                      *(long long *)v132 = -1;
LABEL_147:
                    v65 = optarg;
LABEL_119:
                    v17[v12++] = (long long)v65;
                    continue;
                  }
                  v79 = v141;
                  if ( *v76 != 46 )
                  {
                    if ( *(long long *)v132 || *((long long *)v132 + 1) )
                      goto LABEL_164;
LABEL_145:
                    *(long long *)v77 = -1;
                    goto LABEL_146;
                  }
                  v106 = sub_5D20(v76 + 1, v132 + 8, 0);
                  v77 = v132;
                  v79 = v141;
                  LODWORD(v16) = v150;
                  v78 = (char *)v106;
                  if ( !*(long long *)v132 && !*((long long *)v132 + 1) )
                    goto LABEL_145;
LABEL_146:
                  if ( !v78 )
                    goto LABEL_147;
LABEL_164:
                  v151 = v16;
                  v144 = v79;
                  v132 = v77;
                  v94 = (char *)sub_57C0(v78, v77, 0);
                  v95 = v132;
                  LODWORD(v16) = v151;
                  if ( *v94 )
                    goto LABEL_147;
                  if ( !v144 )
                    goto LABEL_172;
                  v96 = v132 + 16;
                  v145 = v132;
                  v97 = v3[optind++];
                  v132 = v97;
                  v98 = (char *)sub_5D20(v97 + 1, v96, "invalid number after '-'");
                  v99 = v145;
                  v100 = v151;
                  v101 = v98;
                  if ( !v98 )
                    __assert_fail("s", "src/sort.c", 0x1161u, "main");
                  if ( *v98 != 46 )
                    goto LABEL_168;
                  goto LABEL_199;
                case 67:
                case 99:
                  goto LABEL_37;
                case 77:
                case 82:
                case 86:
                case 98:
                case 100:
                case 102:
                case 103:
                case 104:
                case 105:
                case 110:
                case 114:
                  goto LABEL_34;
                case 83:
                  LODWORD(v132) = v22;
                  sub_89B0((unsigned int)longind[0], optarg);
                  LODWORD(v16) = (int)v132;
                  continue;
                case 84:
                  LODWORD(v132) = v22;
                  sub_61E0(optarg);
                  LODWORD(v16) = (int)v132;
                  continue;
                case 107:
                  v139 = v22;
                  v133 = (long long *)sub_5CF0(v154, v3);
                  v55 = (char *)sub_5D20(optarg, v133, v54);
                  v56 = (char *)v133;
                  v57 = v139;
                  v58 = v55;
                  v59 = *v133 - 1;
                  v60 = *v133 == 0;
                  *v133 = v59;
                  if ( v60 )
                    goto LABEL_208;
                  if ( *v55 != 46
                    || (v85 = sub_5D20(v55 + 1, v133 + 1, "invalid number after '.'"),
                        v56 = (char *)v133,
                        LODWORD(v57) = v139,
                        v58 = (char *)v85,
                        v86 = v133[1],
                        v133[1] = v86 - 1,
                        v86) )
                  {
                    if ( !*(long long *)v56 && !*((long long *)v56 + 1) )
                      *(long long *)v56 = -1;
                    v140 = v57;
                    v132 = v56;
                    v61 = (char *)sub_57C0(v58, v56, 0);
                    v63 = v132;
                    v64 = v140;
                    if ( *v61 != 44 )
                    {
                      *((long long *)v132 + 2) = -1;
                      *((long long *)v132 + 3) = 0;
                      goto LABEL_114;
                    }
                  }
                  else
                  {
                    v61 = (char *)sub_5A10(optarg, "character offset is zero", -1, v139);
                  }
                  v142 = v64;
                  v134 = v63;
                  v87 = sub_5D20(v61 + 1, v63 + 16, "invalid number after ','");
                  v88 = v134;
                  v57 = v142;
                  v89 = (char *)v87;
                  v90 = *((long long *)v134 + 2);
                  v59 = v90 - 1;
                  *((long long *)v134 + 2) = v90 - 1;
                  if ( !v90 )
                    goto LABEL_208;
                  if ( *v89 == 46 )
                  {
                    v105 = sub_5D20(v89 + 1, v134 + 24, "invalid number after '.'");
                    LODWORD(v57) = v142;
                    v88 = v134;
                    v89 = (char *)v105;
                  }
                  v143 = v57;
                  v132 = v88;
                  v61 = (char *)sub_57C0(v89, v88, 1);
                  v64 = v143;
                  v63 = v132;
LABEL_114:
                  if ( *v61 )
                    goto LABEL_204;
                  LODWORD(v132) = v64;
                  sub_60C0(v63);
                  LODWORD(v16) = (int)v132;
                  continue;
                case 109:
                  v149 = 1;
                  continue;
                case 111:
                  v47 = s1;
                  v48 = optarg;
                  if ( s1 )
                  {
                    LODWORD(v132) = v22;
                    s1a = optarg;
                    v49 = strcmp(v47, optarg);
                    v48 = s1a;
                    LODWORD(v16) = (int)v132;
                    if ( v49 )
                      goto LABEL_206;
                  }
                  s1 = v48;
                  continue;
                case 115:
                  byte_1E41A = 1;
                  continue;
                case 116:
                  v42 = *optarg;
                  if ( !*optarg )
                    goto LABEL_202;
                  if ( !optarg[1] )
                    goto LABEL_93;
                  v43 = "\\0";
                  v138 = v16;
                  v132 = optarg;
                  v44 = strcmp(optarg, "\\0");
                  v46 = (long long)v132;
                  v16 = v138;
                  if ( v44 )
                    goto LABEL_203;
                  v42 = 0;
LABEL_93:
                  if ( dword_1E028 != 128 && dword_1E028 != v42 )
                    goto LABEL_205;
                  dword_1E028 = v42;
                  continue;
                case 117:
                  byte_1E419 = 1;
                  continue;
                case 121:
                  v40 = optarg;
                  if ( v3[optind - 1] == optarg )
                  {
                    while ( 1 )
                    {
                      v41 = *v40;
                      if ( (unsigned int)(v41 - 48) > 9 )
                        break;
                      ++v40;
                    }
                    optind = ((char)v41 == 0) + optind - 1;
                  }
                  continue;
                case 122:
                  byte_1E038 = 0;
                  continue;
                case 128:
                  LODWORD(v16) = 99;
                  if ( optarg )
                    LODWORD(v16) = aGhmnrvccc[sub_C4B0("--check", optarg, off_1D580, "CCc", 1, off_1E128, 1, v23) + 6];
LABEL_37:
                  if ( (char)v13 && v13 != (int)v16 )
                    goto LABEL_195;
                  v13 = (char)v16;
                  continue;
                case 129:
                  v38 = optarg;
                  if ( !arg
                    || (v137 = v22, v132 = optarg, v39 = strcmp(arg, optarg), v38 = v132, LODWORD(v16) = v137, !v39) )
                  {
                    arg = v38;
                    continue;
                  }
                  v118 = dcgettext(0, "multiple compress programs specified", 5);
                  error(2, 0, v118);
LABEL_195:
                  sub_5940("cC");
                  goto LABEL_196;
                case 130:
                  byte_1E400 = 1;
                  continue;
                case 131:
                  act = (struct sigaction *)optarg;
                  continue;
                case 132:
                  LODWORD(v132) = v22;
                  sub_8B60((unsigned int)longind[0], optarg);
                  LODWORD(v16) = (int)v132;
                  continue;
                case 133:
                  v51 = v129;
                  v52 = optarg;
                  if ( v129 )
                  {
                    LODWORD(v132) = v22;
                    v130 = optarg;
                    v53 = strcmp(v51, optarg);
                    v52 = v130;
                    LODWORD(v16) = (int)v132;
                    if ( v53 )
                      goto LABEL_207;
                  }
                  v129 = v52;
                  continue;
                case 134:
                  LODWORD(v16) = aGhmnrvccc[sub_C4B0("--sort", optarg, off_1D540, "ghMnRVCCc", 1, off_1E128, 1, v24)];
LABEL_34:
                  LODWORD(v132) = v16;
                  v168[0] = v16;
                  v168[1] = 0;
                  sub_57C0(v168, v155, 2);
                  LODWORD(v16) = (int)v132;
                  continue;
                case 135:
                  LODWORD(v132) = v22;
                  v50 = sub_8900((unsigned int)longind[0], optarg);
                  LODWORD(v16) = (int)v132;
                  v131 = v50;
                  continue;
                default:
                  goto LABEL_161;
              }
            }
            if ( (int)v22 == -131 )
            {
              sub_12D40(
                (int)stdout,
                (unsigned int)"sort",
                (unsigned int)"GNU coreutils",
                (int)off_1E120,
                (unsigned int)"Mike Haertel",
                (unsigned int)"Paul Eggert",
                0,
                v22);
              exit(0);
            }
            if ( (int)v22 == -130 )
              sub_BE50(0);
LABEL_161:
            sub_BE50(2);
          }
LABEL_25:
          v18 = optind;
        }
      }
      if ( (int)v18 >= (int)v4 )
        break;
      v20 = v18 + 1;
      v21 = v3[v18];
      optind = v20;
      v17[v12++] = (long long)v21;
    }
    if ( act )
    {
      if ( v12 )
      {
LABEL_160:
        v91 = sub_11170(4, *v17);
        v92 = dcgettext(0, "extra operand %s", 5);
        error(0, 0, v92, v91);
        v93 = dcgettext(0, "file operands cannot be combined with --files0-from", 5);
        __fprintf_chk(stderr, 1, "%s\n", v93);
        goto LABEL_161;
      }
      v14 = (const char *)longind;
      v3 = (char **)sub_6010(act, "r");
      sub_11EE0(longind);
      v107 = act;
      if ( !(unsigned char)sub_11F90((FILE *)v3) )
        goto LABEL_193;
      sub_6040((FILE *)v3);
      if ( !*(long long *)longind )
        goto LABEL_192;
      free(v17);
      v4 = *(long long *)longind;
      v17 = v166;
      if ( *(long long *)longind )
      {
        v3 = (char **)"-";
        while ( 1 )
        {
          v14 = (const char *)v17[v12];
          if ( !strcmp(v14, "-") )
            break;
          ++v12;
          if ( !*v14 )
            goto LABEL_190;
          if ( v4 == v12 )
            goto LABEL_45;
        }
        v108 = sub_11170(4, v14);
        v109 = dcgettext(0, "when reading file names from stdin, no file name of %s allowed", 5);
        error(2, 0, v109, v108);
LABEL_190:
        v110 = sub_11270(0, 3, act);
        v111 = dcgettext(0, "%s:%lu: invalid zero-length file name", 5);
        error(2, 0, v111, v110, v12);
        goto LABEL_191;
      }
    }
LABEL_45:
    v25 = qword_1E410;
    if ( qword_1E410 )
    {
      LODWORD(v4) = 0;
      do
      {
        while ( (unsigned char)sub_5680(v25) && !*(char *)(v29 + 55) )
        {
          v26 = v163;
          *(long long *)(v29 + 32) = v155[4];
          *(long long *)(v29 + 40) = v155[5];
          *(char *)(v29 + 48) = v156;
          *(char *)(v29 + 49) = v157;
          *(char *)(v29 + 54) = v162;
          *(char *)(v29 + 50) = v158;
          *(char *)(v29 + 52) = v160;
          *(char *)(v29 + 53) = v161;
          v27 = v164;
          *(char *)(v29 + 55) = v26;
          *(char *)(v29 + 56) = v27;
          v28 = v159;
          *(char *)(v29 + 51) = v159;
          v25 = *(long long *)(v29 + 64);
          LODWORD(v4) = v28 | v4;
          if ( !v25 )
            goto LABEL_51;
        }
        v30 = *(unsigned char *)(v29 + 51);
        v25 = *(long long *)(v29 + 64);
        LODWORD(v4) = v30 | v4;
      }
      while ( v25 );
LABEL_51:
      v3 = 0;
LABEL_52:
      sub_5970();
      if ( !byte_1E400 )
        goto LABEL_53;
      goto LABEL_73;
    }
    if ( !(unsigned char)sub_5680(v155) )
    {
      v3 = (char **)(&dword_0 + 1);
      sub_60C0(v155);
      LODWORD(v4) = v159;
      goto LABEL_52;
    }
    sub_5970();
    if ( !byte_1E400 )
    {
      byte_1E41B = v163;
      goto LABEL_55;
    }
    LODWORD(v4) = 0;
    v3 = 0;
LABEL_73:
    if ( !(char)v13 )
      break;
LABEL_198:
    v101 = aXDebug;
    aXDebug[0] = v13;
    sub_5940(aXDebug);
LABEL_199:
    v153 = v100;
    v147 = v99;
    v119 = sub_5D20(v101 + 1, v99 + 24, "invalid number after '.'");
    v100 = v153;
    v99 = v147;
    v101 = (char *)v119;
LABEL_168:
    if ( !*((long long *)v99 + 3) )
    {
      v102 = *((long long *)v99 + 2);
      if ( v102 )
        *((long long *)v99 + 2) = v102 - 1;
    }
    v152 = v100;
    v146 = v99;
    v103 = (char *)sub_57C0(v101, v99, 1);
    v95 = v146;
    LODWORD(v16) = v152;
    if ( *v103 )
    {
      sub_5A10(v132, "stray character in field spec", v104, v152);
LABEL_202:
      v120 = dcgettext(0, "empty tab", 5);
      v43 = 0;
      v46 = 2;
      error(2, 0, v120);
LABEL_203:
      v121 = sub_11430(v46, v43, v45, v16);
      v122 = dcgettext(0, "multi-character tab %s", 5);
      error(2, 0, v122, v121);
LABEL_204:
      sub_5A10(optarg, "stray character in field spec", v62, v64);
LABEL_205:
      v123 = dcgettext(0, "incompatible tabs", 5);
      error(2, 0, v123);
LABEL_206:
      v124 = dcgettext(0, "multiple output files specified", 5);
      error(2, 0, v124);
LABEL_207:
      v125 = dcgettext(0, "multiple random sources specified", 5);
      error(2, 0, v125);
LABEL_208:
      sub_5A10(optarg, "field number is zero", v59, v57);
      start();
    }
LABEL_172:
    v95[57] = 1;
    LODWORD(v132) = v16;
    sub_60C0(v95);
    LODWORD(v16) = (int)v132;
  }
  if ( s1 )
  {
LABEL_196:
    if ( !(char)v13 )
      v13 = 111;
    goto LABEL_198;
  }
  if ( !v135 || !setlocale(3, "") )
  {
    v37 = dcgettext(0, "failed to set locale", 5);
    error(0, 0, "%s", v37);
  }
  if ( byte_1E861 )
  {
    v32 = setlocale(3, 0);
    v35 = sub_11430(v32, 0, v33, v34);
    v36 = dcgettext(0, "text ordering performed using %s sorting rules", 5);
    error(0, 0, v36, v35);
  }
  else
  {
    v69 = dcgettext(0, "text ordering performed using simple byte comparison", 5);
    error(0, 0, "%s", v69);
  }
  sub_7D30(v155, (unsigned char)v3);
LABEL_53:
  byte_1E41B = v163;
  if ( (char)v4 )
    sub_6110(v129);
LABEL_55:
  if ( !qword_1E428 )
  {
    v68 = getenv("TMPDIR");
    if ( !v68 )
      v68 = "/tmp";
    sub_61E0(v68);
  }
  if ( !v12 )
  {
    v12 = 1;
    free(v17);
    v17 = (long long *)sub_12EB0(8);
    *v17 = (long long)"-";
  }
  if ( qword_1E438 )
  {
    v31 = 34LL * (unsigned int)dword_1E024;
    if ( v31 < qword_1E438 )
      v31 = qword_1E438;
    qword_1E438 = v31;
  }
  if ( (char)v13 )
  {
    if ( v12 != 1 )
    {
LABEL_191:
      v112 = sub_11170(4, v17[1]);
      v113 = dcgettext(0, "extra operand %s not allowed with -%c", 5);
      error(2, 0, v113, v112, (unsigned int)(char)v13);
LABEL_192:
      v114 = sub_11170(4, act);
      v115 = dcgettext(0, "no input from %s", 5);
      v107 = 0;
      error(2, 0, v115, v114);
LABEL_193:
      v116 = sub_11170(4, v107);
      v117 = dcgettext(0, "cannot read file names from %s", 5);
      error(2, 0, v117, v116);
    }
    if ( !s1 )
    {
      v70 = sub_AAC0(*v17, (unsigned int)(char)v13);
      exit(v70 ^ 1);
    }
    LOBYTE(off_1E010) = v13;
    sub_5940(&off_1E010);
    goto LABEL_63;
  }
  sub_6C00(v17, v12);
  sub_6CA0(s1);
  if ( v149 )
  {
    v80 = sub_13150(v12, 16);
    v81 = &v17[v12];
    v82 = v80;
    do
    {
      v83 = *v17++;
      v80 += 16;
      *(long long *)(v80 - 16) = v83;
    }
    while ( v17 != v81 );
    sub_B720(v82, 0, v12, s1);
  }
  else
  {
    if ( !v131 )
    {
      v71 = sub_E9D0(2);
      v72 = 8;
      if ( v71 <= 8 )
        v72 = v71;
      v131 = v72;
    }
    v66 = v131;
    if ( v131 > 0xFFFFFFFFFFFFFFLL )
      v66 = 0xFFFFFFFFFFFFFFLL;
    sub_BAC0(v17, v12, s1, v66);
  }
  if ( byte_1E418 && (unsigned int)sub_C900(stdin) == -1 )
  {
    v84 = dcgettext(0, "close failed", 5);
    sub_5E10(v84, "-");
  }
  return 0;
}



// Function: proctab_hasher @ 0x5400
unsigned long long proctab_hasher(long long a1, unsigned long long a2)
{
  return *(int *)(a1 + 8) % a2;
}



// Function: proctab_comparator @ 0x5420
bool proctab_comparator(long long a1, long long a2)
{
  return *(int *)(a1 + 8) == *(int *)(a2 + 8);
}



// Function: traverse_raw_number @ 0x5430
long long traverse_raw_number(char **a1)
{
  char *v1; // rax
  char *v2; // rsi
  unsigned int v3; // edx
  unsigned int v4; // r8d
  char v5; // r9
  int v6; // edx
  char *v8; // rdx
  unsigned int v9; // eax
  unsigned int v10; // ecx
  char *v11; // rsi

  v1 = *a1;
  v2 = *a1 + 1;
  v3 = **a1;
  if ( v3 - 48 > 9 )
  {
    v4 = 0;
  }
  else
  {
    v4 = 0;
    do
    {
      v5 = 1;
      if ( (char)v4 < (char)v3 )
        v4 = v3;
      v6 = v1[1];
      v1 += 2;
      if ( v6 != dword_1E864 )
      {
        v1 = v2;
        v5 = 0;
      }
      v2 = v1 + 1;
      v3 = *v1;
    }
    while ( v3 - 48 <= 9 );
    if ( v5 )
    {
      v11 = v1 - 1;
      *a1 = v11;
      return v4;
    }
  }
  if ( byte_1E868 == (char)v3 )
  {
    v8 = v2 + 1;
    v9 = *v2;
    if ( v9 - 48 <= 9 )
    {
      do
      {
        v2 = v8;
        if ( (char)v4 < (char)v9 )
          v4 = v9;
        v10 = *v8++;
        v9 = v10;
      }
      while ( v10 - 48 <= 9 );
    }
  }
  else
  {
    --v2;
  }
  *a1 = v2;
  return v4;
}



// Function: find_unit_order @ 0x54f0
long long find_unit_order(char *a1)
{
  char v1; // r11
  char v2; // r8
  char v3; // r11
  long long result; // rax
  char *v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v1 = *a1;
  v5[1] = (char *)__readfsqword(0x28u);
  v5[0] = &a1[v1 == 45];
  v2 = sub_5430(v5);
  result = 0;
  if ( v2 > 48 )
  {
    result = (unsigned int)byte_15360[(unsigned char)*v5[0]];
    if ( v3 == 45 )
      return (unsigned int)-byte_15360[(unsigned char)*v5[0]];
  }
  return result;
}



// Function: getmonth @ 0x5570
long long getmonth(char *a1, long long *a2)
{
  char *v3; // rdx
  unsigned char v5; // r8
  char *v6; // r9
  unsigned long long v7; // rbx
  unsigned long long v8; // rbp
  char *v9; // r10
  unsigned long long v10; // r11
  long long v11; // rdx
  unsigned char v12; // al
  char *i; // rsi
  long long v14; // rdx
  long long v15; // rdx
  unsigned char v16; // r14
  unsigned char v17; // al
  long long v18; // rcx

  v3 = &unk_1E740;
  while ( 1 )
  {
    v5 = sub_53E0((unsigned int)*a1, a2, v3);
    if ( !v3[v5] )
      break;
    a1 = v6 + 1;
  }
  v7 = 12;
  v8 = 0;
  v9 = (char *)&unk_1E440;
  while ( 1 )
  {
    v10 = (v8 + v7) >> 1;
    v11 = (unsigned char)*(&off_1E060)[2 * v10];
    if ( !(char)v11 )
      break;
    v12 = v5;
    for ( i = v6; ; v12 = sub_53E0((unsigned int)*i, i, v14) )
    {
      v16 = sub_53E0((unsigned int)v9[v12], i, v11);
      v17 = sub_53E0((unsigned int)(char)v15, i, v15);
      if ( v16 < v17 )
      {
        v7 = v10;
        goto LABEL_12;
      }
      if ( v16 > v17 )
        break;
      v14 = *(unsigned char *)(v18 + 1);
      ++i;
      if ( !(char)v14 )
        goto LABEL_15;
    }
    v8 = v10 + 1;
LABEL_12:
    if ( v8 >= v7 )
      return 0;
  }
  i = v6;
LABEL_15:
  if ( a2 )
    *a2 = i;
  return LODWORD((&off_1E060)[2 * v10 + 1]);
}



// Function: key_numeric @ 0x5660
bool key_numeric(long long a1)
{
  return (*(long long *)(a1 + 48) & 0xFFFF00FF0000LL) != 0;
}



// Function: default_key_compare @ 0x5680
char default_key_compare(long long a1)
{
  char result; // al

  result = 0;
  if ( !*(long long *)(a1 + 32) && !*(long long *)(a1 + 40) )
  {
    result = *(char *)(a1 + 48);
    if ( result )
      return 0;
    if ( *(char *)(a1 + 49) )
      return result;
    result = sub_5660(a1);
    if ( result )
      return 0;
    if ( !*(char *)(a1 + 54) && !*(char *)(a1 + 56) )
      return *(char *)(a1 + 51) ^ 1;
  }
  return result;
}



// Function: key_to_opts @ 0x56d0
char *key_to_opts(long long a1, char *a2)
{
  char *result; // rax

  if ( *(short *)(a1 + 48) )
    *a2++ = 98;
  if ( *(_UNKNOWN **)(a1 + 32) == &unk_1E540 )
    *a2++ = 100;
  result = a2;
  if ( *(long long *)(a1 + 40) )
  {
    *a2 = 102;
    result = a2 + 1;
  }
  if ( *(char *)(a1 + 52) )
    *result++ = 103;
  if ( *(char *)(a1 + 53) )
    *result++ = 104;
  if ( *(_UNKNOWN **)(a1 + 32) == &unk_1E640 )
    *result++ = 105;
  if ( *(char *)(a1 + 54) )
    *result++ = 77;
  if ( *(char *)(a1 + 50) )
    *result++ = 110;
  if ( *(char *)(a1 + 51) )
    *result++ = 82;
  if ( *(char *)(a1 + 55) )
    *result++ = 114;
  if ( *(char *)(a1 + 56) )
    *result++ = 86;
  *result = 0;
  return result;
}



// Function: compare_nodes @ 0x5780
long long compare_nodes(long long a1, long long a2)
{
  unsigned int v2; // eax

  v2 = *(int *)(a2 + 80);
  if ( *(int *)(a1 + 80) == v2 )
    return *(long long *)(a1 + 40) + *(long long *)(a1 + 48) < *(long long *)(a2 + 40) + *(long long *)(a2 + 48);
  else
    return *(int *)(a1 + 80) < v2;
}



// Function: set_ordering @ 0x57c0
char *set_ordering(char *a1, long long a2, int a3)
{
  char v3; // al
  char *v4; // r8
  unsigned int v6; // edx
  unsigned int v7; // edi

  v3 = *a1;
  v4 = a1;
  if ( *a1 )
  {
    v6 = a3 - 1;
    v7 = a3 & 0xFFFFFFFD;
    while ( 2 )
    {
      switch ( v3 )
      {
        case 'M':
          *(char *)(a2 + 54) = 1;
          goto LABEL_5;
        case 'R':
          *(char *)(a2 + 51) = 1;
          goto LABEL_5;
        case 'V':
          *(char *)(a2 + 56) = 1;
          goto LABEL_5;
        case 'b':
          if ( !v7 )
            *(char *)(a2 + 48) = 1;
          if ( v6 <= 1 )
            *(char *)(a2 + 49) = 1;
          goto LABEL_5;
        case 'd':
          *(long long *)(a2 + 32) = &unk_1E540;
          goto LABEL_5;
        case 'f':
          *(long long *)(a2 + 40) = &unk_1E440;
          goto LABEL_5;
        case 'g':
          *(char *)(a2 + 52) = 1;
          goto LABEL_5;
        case 'h':
          *(char *)(a2 + 53) = 1;
          goto LABEL_5;
        case 'i':
          if ( !*(long long *)(a2 + 32) )
            *(long long *)(a2 + 32) = &unk_1E640;
          goto LABEL_5;
        case 'n':
          *(char *)(a2 + 50) = 1;
          goto LABEL_5;
        case 'r':
          *(char *)(a2 + 55) = 1;
LABEL_5:
          v3 = *++v4;
          if ( !v3 )
            return v4;
          continue;
        default:
          return v4;
      }
    }
  }
  return v4;
}



// Function: struct_month_cmp @ 0x5930
int struct_month_cmp(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: incompatible_options @ 0x5940
long long incompatible_options(long long a1)
{
  char *v1; // rax

  v1 = dcgettext(0, "options '-%s' are incompatible", 5);
  error(2, 0, v1, a1);
  return sub_5970();
}



// Function: check_ordering_compatibility @ 0x5970
unsigned long long check_ordering_compatibility()
{
  long long v0; // rdi
  long long v2; // r8
  char v3[40]; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long i; // [rsp+28h] [rbp-10h]

  v0 = qword_1E410;
  for ( i = __readfsqword(0x28u); v0; v0 = *(long long *)(v0 + 64) )
  {
    if ( (unsigned char)((*(long long *)(v0 + 32) != 0) | *(char *)(v0 + 51) | *(char *)(v0 + 56))
       + *(unsigned char *)(v0 + 54)
       + *(unsigned char *)(v0 + 53)
       + *(unsigned char *)(v0 + 52)
       + *(unsigned char *)(v0 + 50) > 1 )
    {
      *(char *)(v0 + 55) = 0;
      *(short *)(v0 + 48) = 0;
      sub_56D0(v0, v3);
      sub_5940(v2);
    }
  }
  return i - __readfsqword(0x28u);
}



// Function: badfieldspec @ 0x5a10
long long badfieldspec(long long a1, const char *a2, long long a3, long long a4)
{
  long long v4; // rbx
  char *v5; // r12
  char *v6; // rax

  v4 = sub_11430(a1, a2, a3, a4);
  v5 = dcgettext(0, a2, 5);
  v6 = dcgettext(0, "%s: invalid field specification %s", 5);
  error(2, 0, v6, v5, v4);
  return sub_5A60();
}



// Function: cs_enter @ 0x5a60
int cs_enter(long long a1)
{
  int result; // eax

  result = pthread_sigmask(0, &newmask, (__sigset_t *)(a1 + 8));
  *(char *)a1 = result == 0;
  return result;
}



// Function: cs_leave @ 0x5a80
int cs_leave(long long a1)
{
  int result; // eax

  if ( *(char *)a1 )
    return pthread_sigmask(2, (const __sigset_t *)(a1 + 8), 0);
  return result;
}



// Function: cleanup @ 0x5aa0
int cleanup()
{
  const char *v0; // rbx
  int result; // eax

  v0 = (const char *)ptr;
  if ( ptr )
  {
    do
    {
      result = unlink(v0 + 13);
      v0 = *(const char **)v0;
    }
    while ( v0 );
  }
  ptr = 0;
  return result;
}



// Function: exit_cleanup @ 0x5ad0
long long exit_cleanup()
{
  long long v1[19]; // [rsp+0h] [rbp-98h] BYREF

  v1[17] = __readfsqword(0x28u);
  if ( ptr )
  {
    sub_5A60((long long)v1);
    sub_5AA0();
    sub_5A80((long long)v1);
  }
  return sub_C5C0();
}



// Function: sighandler @ 0x5b40
int sighandler(int sig)
{
  sub_5AA0();
  signal(sig, 0);
  return raise(sig);
}



// Function: inittables @ 0x5b80
void inittables()
{
  long long v0; // rbx
  const unsigned short **v1; // r12
  unsigned short v2; // dx
  char v3; // al
  char **v4; // r14
  long long i; // r13
  char *v6; // r15
  size_t v7; // rbx
  long long v8; // rax
  long long v9; // rcx
  char *v10; // rsi
  char *v11; // rdx
  unsigned char v12; // al
  long long v13; // rdx
  long long v14; // r8

  v0 = 0;
  v1 = __ctype_b_loc();
  do
  {
    byte_1E740[v0] = sub_5B60((unsigned int)v0);
    v2 = (*v1)[v0];
    byte_1E640[v0] = ((v2 >> 14) ^ 1) & 1;
    v3 = 0;
    if ( (v2 & 8) == 0 )
      v3 = sub_5B60((unsigned int)v0) ^ 1;
    byte_1E540[v0] = v3;
    byte_1E440[v0] = toupper(v0);
    ++v0;
  }
  while ( v0 != 256 );
  if ( byte_1E860 )
  {
    v4 = &off_1E060;
    for ( i = 1; i != 13; ++i )
    {
      v6 = nl_langinfo((int)i + 131085);
      v7 = strlen(v6);
      v8 = sub_12EB0(v7 + 1);
      *((int *)v4 + 2) = i;
      *v4 = (char *)v8;
      v9 = v8;
      if ( v7 )
      {
        v10 = &v6[v7];
        v11 = v6;
        v7 = 0;
        do
        {
          v12 = sub_53E0((unsigned int)*v11, v10, v11);
          if ( (*(char *)(v14 + 2LL * v12) & 1) == 0 )
            *(char *)(v9 + v7++) = byte_1E440[v12];
          v11 = (char *)(v13 + 1);
        }
        while ( v10 != v11 );
      }
      *(char *)(v9 + v7) = 0;
      v4 += 2;
    }
    qsort(&off_1E060, 0xCu, 0x10u, (__compar_fn_t)compar);
  }
}



// Function: key_init @ 0x5cf0
long long key_init(long long a1)
{
  *(long long *)(a1 + 64) = 0;
  *(_OWORD *)(a1 + 16) = 0;
  *(_OWORD *)a1 = 0;
  *(long long *)(a1 + 16) = -1;
  *(_OWORD *)(a1 + 32) = 0;
  *(_OWORD *)(a1 + 48) = 0;
  return a1;
}



// Function: parse_field_count @ 0x5d20
long long parse_field_count(char *a1, long long *a2, const char *a3)
{
  long long v4; // rdx
  long long v5; // rcx
  long long v7; // rbx
  char *v8; // r12
  char *v9; // rax
  long long v10[6]; // [rsp+8h] [rbp-30h] BYREF

  v10[2] = __readfsqword(0x28u);
  switch ( (unsigned int)sub_13650(a1) )
  {
    case 0u:
    case 2u:
      *a2 = v10[1];
      return v10[0];
    case 1u:
    case 3u:
      *a2 = -1;
      return v10[0];
    case 4u:
      if ( a3 )
      {
        v7 = sub_11430(a1, v10, v4, v5);
        v8 = dcgettext(0, a3, 5);
        v9 = dcgettext(0, "%s: invalid count at start of %s", 5);
        error(2, 0, v9, v8, v7);
        sub_5E10(2, 0);
      }
      return 0;
    default:
      return v10[0];
  }
}



// Function: sort_die @ 0x5e10
void sort_die(const char *a1, char *a2)
{
  char *v2; // rdx
  const char *v3; // rbx
  int *v4; // rax

  v2 = a2;
  if ( !a2 )
    goto LABEL_3;
  while ( 1 )
  {
    v3 = (const char *)sub_11270(0, 3, v2);
    v4 = __errno_location();
    error(2, *v4, "%s: %s", a1, v3);
LABEL_3:
    v2 = dcgettext(0, "standard output", 5);
  }
}



// Function: get_outstatus @ 0x5e70
struct stat *get_outstatus()
{
  long long v0; // r8
  long long v2; // r8
  int *v3; // rax

  if ( !dword_1E2F0 )
  {
    if ( fstat(1, &buf) )
    {
      v3 = __errno_location();
      v2 = 0;
      dword_1E2F0 = *v3;
      if ( dword_1E2F0 >= 0 )
        return (struct stat *)v2;
    }
    else
    {
      dword_1E2F0 = -1;
    }
    return &buf;
  }
  v0 = 0;
  if ( dword_1E2F0 < 0 )
    return &buf;
  return (struct stat *)v0;
}



// Function: stream_open @ 0x5ee0
FILE *stream_open(char *file, char *modes)
{
  const char *v3; // r12
  FILE *v4; // r13
  struct stat *v5; // rax
  long long v6; // r13
  char *v7; // rax
  int v8; // edi

  if ( *modes != 114 )
  {
    if ( *modes != 119 )
      __assert_fail("!\"unexpected mode passed to stream_open\"", "src/sort.c", 0x3D5u, "stream_open");
    if ( !file )
      return stdout;
    if ( !ftruncate((int)&dword_0 + 1, 0) )
      return stdout;
    v3 = (const char *)(unsigned int)*__errno_location();
    v5 = sub_5E70();
    if ( v5 )
    {
      if ( (v5->st_mode & 0xF000) != 0x8000 )
        return stdout;
    }
    v6 = sub_11270(0, 3, file);
    v7 = dcgettext(0, "%s: error truncating", 5);
    error(2, (int)v3, v7, v6);
    goto LABEL_12;
  }
  v3 = modes;
  if ( strcmp(file, "-") )
  {
LABEL_12:
    v4 = 0;
    v8 = open(file, 0x80000);
    if ( v8 >= 0 )
      v4 = fdopen(v8, v3);
    goto LABEL_8;
  }
  byte_1E418 = 1;
  v4 = stdin;
LABEL_8:
  sub_C8D0(v4, 2);
  return v4;
}



// Function: xfopen @ 0x6010
FILE *xfopen(char *a1, char *a2)
{
  FILE *result; // rax
  char *v3; // rax

  result = sub_5EE0(a1, a2);
  if ( !result )
  {
    v3 = dcgettext(0, "open failed", 5);
    sub_5E10(v3, a1);
  }
  return result;
}



// Function: xfclose @ 0x6040
void xfclose(FILE *stream, char *a2)
{
  int v3; // eax
  const char *v4; // rsi
  char *v5; // rax

  v3 = fileno(stream);
  if ( !v3 )
  {
    clearerr_unlocked(stream);
    return;
  }
  if ( v3 != 1 )
  {
    if ( !(unsigned int)sub_C900(stream) )
      return;
    v4 = "close failed";
LABEL_8:
    v5 = dcgettext(0, v4, 5);
    sub_5E10(v5, a2);
  }
  if ( fflush_unlocked(stream) )
  {
    v4 = "fflush failed";
    goto LABEL_8;
  }
}



// Function: insertkey @ 0x60c0
long long insertkey(void *a1)
{
  long long result; // rax
  long long v2; // rdx
  long long *v3; // rcx
  long long v4; // rcx

  result = sub_131B0(a1, 0x48u);
  v2 = qword_1E410;
  v3 = &qword_1E410;
  if ( qword_1E410 )
  {
    do
    {
      v4 = v2;
      v2 = *(long long *)(v2 + 64);
    }
    while ( v2 );
    v3 = (long long *)(v4 + 64);
  }
  *v3 = result;
  *(long long *)(result + 64) = 0;
  return result;
}



// Function: random_md5_state_init @ 0x6110
unsigned long long random_md5_state_init(char *a1)
{
  char *v1; // r12
  long long v2; // rax
  void *v3; // rbp
  const char *v5; // rsi
  char *v6; // rax
  char v7[24]; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v8; // [rsp+18h] [rbp-20h]

  v1 = a1;
  v8 = __readfsqword(0x28u);
  v2 = sub_11710(a1, 16);
  if ( !v2 )
  {
    if ( !a1 )
      v1 = "getrandom";
    v5 = "open failed";
LABEL_7:
    v6 = dcgettext(0, v5, 5);
    sub_5E10(v6, v1);
  }
  v3 = (void *)v2;
  sub_11830(v2, v7, 16);
  if ( (unsigned int)sub_11850(v3) )
  {
    v5 = "close failed";
    goto LABEL_7;
  }
  j__MD5_Init(&xmmword_1E300);
  sub_C690(v7, 16, &xmmword_1E300);
  return v8 - __readfsqword(0x28u);
}



// Function: add_temp_dir @ 0x61e0
long long add_temp_dir(long long a1)
{
  long long v1; // rdx
  long long v3; // rdi
  long long v5; // rax

  v1 = qword_1E428;
  v3 = qword_1E430;
  if ( qword_1E428 == qword_1E420 )
  {
    v5 = sub_12FF0(qword_1E430, &qword_1E420, 8);
    v1 = qword_1E428;
    qword_1E430 = v5;
    v3 = v5;
  }
  *(long long *)(v3 + 8 * v1) = a1;
  qword_1E428 = v1 + 1;
  return v1 + 1;
}



// Function: initbuf @ 0x6240
void *initbuf(long long a1, long long a2, unsigned long long a3)
{
  size_t v4; // rbp
  void *result; // rax

  while ( 1 )
  {
    v4 = (a3 & 0xFFFFFFFFFFFFFFE0LL) + 32;
    result = malloc(v4);
    *(long long *)a1 = result;
    if ( result )
      break;
    a3 = v4 >> 1;
    if ( a2 + 1 >= a3 )
      sub_13270();
  }
  *(long long *)(a1 + 40) = a2;
  *(long long *)(a1 + 24) = v4;
  *(long long *)(a1 + 16) = 0;
  *(long long *)(a1 + 32) = 0;
  *(long long *)(a1 + 8) = 0;
  *(char *)(a1 + 48) = 0;
  return result;
}



// Function: debug_width @ 0x62b0
long long debug_width(char *a1, char *a2)
{
  char *v2; // rbx
  long long result; // rax

  v2 = a1;
  result = (int)sub_E580(a1, a2 - a1, 0);
  if ( a2 > a1 )
  {
    do
      result += *v2++ == 9;
    while ( a2 != v2 );
  }
  return result;
}



// Function: mark_key @ 0x62f0
int mark_key(long long a1, long long a2)
{
  long long v2; // rbp
  long long v3; // rbx
  char *v6; // rax

  v2 = a2;
  if ( a1 )
  {
    v3 = a1 - 1;
    do
      putchar_unlocked(32);
    while ( v3-- != 0 );
  }
  if ( a2 )
  {
    do
    {
      putchar_unlocked(95);
      --v2;
    }
    while ( v2 );
    return putchar_unlocked(10);
  }
  else
  {
    v6 = dcgettext(0, "^ no match for key\n", 5);
    return __printf_chk(1, v6);
  }
}



// Function: nan_compare @ 0x6370
int nan_compare(const char *a1, const char *a2)
{
  long double v3; // [rsp+0h] [rbp-38h] BYREF
  long double s2; // [rsp+10h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  v3 = strtold(a1, 0);
  s2 = strtold(a2, 0);
  return memcmp(&v3, &s2, 0x10u);
}



// Function: general_numcompare @ 0x63d0
long long general_numcompare(const char *a1, const char *a2)
{
  long double v2; // fst7
  long long result; // rax
  long double v4; // [rsp+0h] [rbp-48h]
  char *endptr; // [rsp+18h] [rbp-30h] BYREF
  char *v6[5]; // [rsp+20h] [rbp-28h] BYREF

  v6[1] = (char *)__readfsqword(0x28u);
  v4 = strtold(a1, &endptr);
  v2 = strtold(a2, v6);
  if ( endptr == a1 )
    return (unsigned int)-(v6[0] != a2);
  if ( v6[0] == a2 )
    return 1;
  if ( v2 > v4 )
    return 0xFFFFFFFFLL;
  result = 1;
  if ( v4 <= v2 )
  {
    result = 0;
    if ( v4 != v2 )
      return 0xFFFFFFFFLL;
  }
  return result;
}



// Function: numcompare @ 0x64d0
long long numcompare(char *a1, char *a2)
{
  char *v3; // rdx
  unsigned char v4; // al
  long long v5; // r8
  unsigned char v6; // al
  long long v7; // r8

  v3 = byte_1E740;
  while ( 1 )
  {
    v4 = sub_53E0((unsigned int)*a1, a2, v3);
    if ( !v3[v4] )
      break;
    a1 = (char *)(v5 + 1);
  }
  while ( 1 )
  {
    v6 = sub_53E0((unsigned int)*a2, a2, v3);
    if ( !v3[v6] )
      break;
    ++a2;
  }
  return sub_12790(v7, a2, (unsigned int)byte_1E868, (unsigned int)dword_1E864);
}



// Function: human_numcompare @ 0x6530
long long human_numcompare(char *a1, char *a2)
{
  char *v2; // rdx
  char *v3; // r12
  unsigned char v5; // al
  unsigned char v6; // al
  int v7; // ebx
  long long result; // rax

  v2 = byte_1E740;
  v3 = a2;
  while ( 1 )
  {
    v5 = sub_53E0((unsigned int)*a1, a2, v2);
    if ( !v2[v5] )
      break;
    ++a1;
  }
  while ( 1 )
  {
    v6 = sub_53E0((unsigned int)*v3, a2, v2);
    if ( !v2[v6] )
      break;
    ++v3;
  }
  v7 = sub_54F0(a1);
  result = v7 - (unsigned int)sub_54F0(v3);
  if ( !(int)result )
    return sub_12790(a1, v3, (unsigned int)byte_1E868, (unsigned int)dword_1E864);
  return result;
}



// Function: xstrxfrm @ 0x65c0
size_t xstrxfrm(char *dest, char *src, size_t n)
{
  int *v4; // rax
  int *v5; // rbx
  size_t result; // rax
  char *v7; // rax
  char *v8; // rax
  long long v9; // r12
  char *v10; // rax
  char *v11; // rdx

  v4 = __errno_location();
  *v4 = 0;
  v5 = v4;
  result = strxfrm(dest, src, n);
  if ( *v5 )
  {
    v7 = dcgettext(0, "string transformation failed", 5);
    error(0, *v5, v7);
    v8 = dcgettext(0, "set LC_ALL='C' to work around the problem", 5);
    error(0, 0, v8);
    v9 = sub_110A0(0, 8, src);
    v10 = dcgettext(0, "the untransformed string was %s", 5);
    error(2, 0, v10, v9);
    return sub_6680((char *)&dword_0 + 2, 0, v11);
  }
  return result;
}



// Function: compare_random @ 0x6680
long long compare_random(char *src, size_t n, char *a3, size_t a4)
{
  char *v4; // r14
  char *v5; // rbp
  size_t v6; // rbx
  size_t v7; // r13
  char *v8; // r15
  size_t v9; // r12
  size_t v10; // rdx
  char *v11; // rax
  size_t v12; // rax
  size_t v13; // rdx
  char *v14; // rdi
  unsigned long long v15; // r13
  char *v16; // rax
  char *v17; // rax
  char *v18; // r12
  int v19; // eax
  size_t v21; // rdx
  char *s1; // [rsp+0h] [rbp-1118h]
  char *v23; // [rsp+8h] [rbp-1110h]
  char *s2; // [rsp+10h] [rbp-1108h]
  char *v25; // [rsp+18h] [rbp-1100h]
  size_t na; // [rsp+20h] [rbp-10F8h]
  unsigned int v27; // [rsp+2Ch] [rbp-10ECh]
  size_t size; // [rsp+30h] [rbp-10E8h]
  char v29[16]; // [rsp+50h] [rbp-10C8h] BYREF
  char v30[16]; // [rsp+60h] [rbp-10B8h] BYREF
  _OWORD v31[5]; // [rsp+70h] [rbp-10A8h] BYREF
  long long v32; // [rsp+C0h] [rbp-1058h]
  int v33; // [rsp+C8h] [rbp-1050h]
  __m128i si128; // [rsp+CCh] [rbp-104Ch] BYREF
  __m128i v35; // [rsp+DCh] [rbp-103Ch]
  __m128i v36; // [rsp+ECh] [rbp-102Ch]
  __m128i v37; // [rsp+FCh] [rbp-101Ch]
  __m128i v38; // [rsp+10Ch] [rbp-100Ch]
  long long v39; // [rsp+11Ch] [rbp-FFCh]
  int v40; // [rsp+124h] [rbp-FF4h]
  char v41[4008]; // [rsp+130h] [rbp-FE8h] BYREF
  unsigned long long v42; // [rsp+10D8h] [rbp-40h]

  v42 = __readfsqword(0x28u);
  v4 = a3;
  v5 = src;
  v6 = n;
  v7 = a4;
  v40 = dword_1E358;
  v39 = qword_1E350;
  v32 = qword_1E350;
  v33 = dword_1E358;
  si128 = _mm_load_si128((const __m128i *)&xmmword_1E300);
  v35 = _mm_load_si128((const __m128i *)&xmmword_1E310);
  v36 = _mm_load_si128((const __m128i *)&xmmword_1E320);
  v37 = _mm_load_si128((const __m128i *)&xmmword_1E330);
  v38 = _mm_load_si128((const __m128i *)&xmmword_1E340);
  v31[0] = si128;
  v31[1] = v35;
  v31[2] = v36;
  v31[3] = v37;
  v31[4] = v38;
  if ( byte_1E861 )
  {
    v8 = 0;
    v9 = 4000;
    v23 = &src[n];
    v25 = &a3[a4];
    size = 3 * (a4 + n) + 2;
    s1 = v41;
    v27 = 0;
LABEL_3:
    if ( v9 < size )
    {
      while ( 1 )
      {
        v9 = (3 * v9) >> 1;
        if ( v9 < size )
          v9 = size;
        free(v8);
        v11 = (char *)malloc(v9);
        v8 = v11;
        if ( !v11 )
        {
          v9 = 4000;
          s1 = v41;
          goto LABEL_4;
        }
        s1 = v11;
        if ( v5 >= v23 )
          goto LABEL_5;
LABEL_20:
        v12 = sub_65C0(s1, v5, v9);
        v6 = v12 + 1;
        if ( v4 < v25 )
          break;
        na = 0;
        v15 = v12 + 1;
        if ( v9 < v6 )
          goto LABEL_24;
LABEL_45:
        if ( v9 < v15 )
          goto LABEL_24;
        s2 = &s1[v6];
        if ( v5 >= v23 )
          goto LABEL_7;
        v5 += strlen(v5) + 1;
        if ( v4 < v25 )
          goto LABEL_8;
LABEL_9:
        if ( v5 >= v23 && v4 >= v25 )
          goto LABEL_52;
        sub_C690(s1, v6, v31);
        sub_C690(s2, na, &si128);
        if ( v27 )
          goto LABEL_3;
        v10 = na;
        if ( v6 <= na )
          v10 = v6;
        v27 = memcmp(s1, s2, v10);
        if ( v27 )
          goto LABEL_3;
        v27 = (v6 > na) - (v6 < na);
        if ( v9 >= size )
          goto LABEL_4;
      }
      if ( v9 >= v6 )
      {
        v13 = v9 - v6;
        v14 = &s1[v6];
      }
      else
      {
        v13 = 0;
        v14 = 0;
      }
LABEL_23:
      na = sub_65C0(v14, v4, v13) + 1;
      v15 = na + v6;
      if ( v9 >= v6 )
        goto LABEL_45;
LABEL_24:
      if ( v15 <= 0x5555555555555554LL )
        v15 = (3 * v15) >> 1;
      free(v8);
      v16 = (char *)sub_12EB0(v15);
      v8 = v16;
      if ( v5 >= v23 )
      {
        v17 = &v16[v6];
        s2 = v17;
        if ( v4 >= v25 )
          goto LABEL_28;
        v9 = v15;
        strxfrm(v17, v4, na);
        s1 = v8;
LABEL_8:
        v4 += strlen(v4) + 1;
        goto LABEL_9;
      }
      strxfrm(v16, v5, v6);
      s2 = &v8[v6];
      if ( v4 < v25 )
      {
        v9 = v15;
        strxfrm(s2, v4, na);
        s1 = v8;
        v5 += strlen(v5) + 1;
        goto LABEL_8;
      }
LABEL_28:
      if ( v5 < v23 )
      {
        v9 = v15;
        s1 = v8;
        v5 += strlen(v5) + 1;
        goto LABEL_9;
      }
      v7 = na;
      v18 = v8;
    }
    else
    {
LABEL_4:
      if ( v5 < v23 )
        goto LABEL_20;
LABEL_5:
      if ( v4 < v25 )
      {
        v14 = s1;
        v13 = v9;
        v6 = 0;
        goto LABEL_23;
      }
      na = 0;
      v6 = 0;
      s2 = s1;
LABEL_7:
      if ( v4 < v25 )
        goto LABEL_8;
LABEL_52:
      v18 = v8;
      v7 = na;
      v8 = s1;
    }
    sub_C690(v8, v6, v31);
    sub_C6C0(v31, v29);
    sub_C690(s2, v7, &si128);
    sub_C6C0(&si128, v30);
    v19 = memcmp(v29, v30, 0x10u);
    if ( v19 )
      goto LABEL_43;
    if ( v27 )
      goto LABEL_32;
    v4 = s2;
    v5 = v8;
  }
  else
  {
    sub_C690(src, n, v31);
    sub_C6C0(v31, v29);
    sub_C690(v4, v7, &si128);
    sub_C6C0(&si128, v30);
    v18 = 0;
    v27 = memcmp(v29, v30, 0x10u);
    if ( v27 )
      goto LABEL_32;
  }
  v21 = v7;
  if ( v6 <= v7 )
    v21 = v6;
  v27 = memcmp(v5, v4, v21);
  if ( !v27 )
  {
    v19 = (v6 > v7) - (v6 < v7);
LABEL_43:
    v27 = v19;
  }
LABEL_32:
  free(v18);
  return v27;
}



// Function: check_inputs @ 0x6c00
void check_inputs(const char **a1, long long a2)
{
  long long i; // rbp
  const char *v4; // r12
  char *v5; // rbp
  char *v6; // rax

  if ( a2 )
  {
    for ( i = 0; i != a2; ++i )
    {
      v4 = *a1;
      if ( strcmp(*a1, "-") )
      {
        if ( euidaccess(v4, 4) )
        {
          v5 = (char *)*a1;
          v6 = dcgettext(0, "cannot read", 5);
          sub_5E10(v6, v5);
        }
      }
      ++a1;
    }
  }
}



// Function: move_fd @ 0x6c80
int move_fd(int fd, int a2)
{
  int result; // eax

  if ( fd != a2 )
  {
    dup2(fd, a2);
    return close(fd);
  }
  return result;
}



// Function: check_output @ 0x6ca0
void check_output(char *a1)
{
  int v2; // edi
  char *v3; // rax

  if ( a1 )
  {
    v2 = open(a1, 524353, 438);
    if ( v2 < 0 )
    {
      v3 = dcgettext(0, "open failed", 5);
      sub_5E10(v3, a1);
    }
    sub_6C80(v2, 1);
  }
}



// Function: delete_proc @ 0x6cf0
long long delete_proc(int a1)
{
  long long v1; // rax
  int v3[6]; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v4; // [rsp+18h] [rbp-10h]

  v4 = __readfsqword(0x28u);
  v3[2] = a1;
  v1 = sub_DFF0(qword_1E360, v3);
  if ( !v1 )
    return 0;
  *(char *)(v1 + 12) = 2;
  return 1;
}



// Function: reap @ 0x6d50
long long reap(__pid_t pid)
{
  int v1; // edx
  bool v3; // zf
  __pid_t v4; // edi
  __pid_t v5; // eax
  unsigned int v6; // r12d
  long long v8; // r13
  char *v9; // r12
  int *v10; // rax
  long long v11; // r12
  char *v12; // rax
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v14; // [rsp+8h] [rbp-20h]

  v1 = 0;
  v14 = __readfsqword(0x28u);
  v3 = pid == 0;
  v4 = -1;
  if ( !v3 )
    v4 = pid;
  LOBYTE(v1) = v3;
  v5 = waitpid(v4, &stat_loc, v1);
  if ( v5 < 0 )
  {
    v8 = sub_11170(4, arg);
    v9 = dcgettext(0, "waiting for %s [-d]", 5);
    v10 = __errno_location();
    error(2, *v10, v9, v8);
  }
  else
  {
    v6 = v5;
    if ( !v5 || pid <= 0 && !(unsigned char)sub_6CF0(v5) )
      return v6;
    if ( !(stat_loc & 0x7F | BYTE1(stat_loc)) )
    {
      --dword_1E35C;
      return v6;
    }
  }
  v11 = sub_11170(4, arg);
  v12 = dcgettext(0, "%s [-d] terminated abnormally", 5);
  error(2, 0, v12, v11);
  return sub_6E60(2);
}



// Function: wait_proc @ 0x6e60
long long wait_proc(__pid_t pid)
{
  long long result; // rax

  result = sub_6CF0(pid);
  if ( (char)result )
    return sub_6D50(pid);
  return result;
}



// Function: zaptemp @ 0x6e80
unsigned long long zaptemp(char *name)
{
  long long *v1; // rbx
  char *i; // rbp
  long long v3; // r14
  int v4; // r13d
  int v5; // eax
  long long v7; // r12
  char *v8; // rax
  int errnum; // [rsp+Ch] [rbp-CCh]
  char v10[136]; // [rsp+10h] [rbp-C8h] BYREF
  unsigned long long v11; // [rsp+98h] [rbp-40h]

  v1 = (long long *)&ptr;
  v11 = __readfsqword(0x28u);
  for ( i = (char *)ptr; name != i + 13; i = *(char **)i )
    v1 = (long long *)i;
  if ( i[12] == 1 )
    sub_6E60(*((int *)i + 2));
  v3 = *(long long *)i;
  sub_5A60((long long)v10);
  v4 = unlink(name);
  v5 = *__errno_location();
  *v1 = v3;
  errnum = v5;
  sub_5A80((long long)v10);
  if ( v4 )
  {
    v7 = sub_11270(0, 3, name);
    v8 = dcgettext(0, "warning: cannot remove: %s", 5);
    error(0, errnum, v8, v7);
    if ( v3 )
      goto LABEL_7;
LABEL_9:
    off_1E040 = v1;
    goto LABEL_7;
  }
  if ( !v3 )
    goto LABEL_9;
LABEL_7:
  free(i);
  return v11 - __readfsqword(0x28u);
}



// Function: reap_exited @ 0x6fb0
long long reap_exited()
{
  long long result; // rax

  result = (unsigned int)dword_1E35C;
  if ( dword_1E35C > 0 )
  {
    do
      result = sub_6D50(0);
    while ( (int)result && dword_1E35C > 0 );
  }
  return result;
}



// Function: reap_some @ 0x6ff0
long long reap_some()
{
  sub_6D50(-1);
  return sub_6FB0();
}



// Function: reap_all @ 0x7010
long long reap_all()
{
  long long result; // rax

  for ( ; dword_1E35C > 0; result = (unsigned int)dword_1E35C )
    sub_6D50(-1);
  return result;
}



// Function: pipe_fork @ 0x7040
long long pipe_fork(int *a1, long long a2)
{
  long long v2; // rbx
  int *v3; // rbp
  void *v4; // r14
  __pid_t v5; // eax
  int v6; // r15d
  unsigned int v7; // r12d
  double v10; // [rsp+0h] [rbp-D8h]
  char v11[136]; // [rsp+10h] [rbp-C8h] BYREF
  unsigned long long v12; // [rsp+98h] [rbp-40h]

  v12 = __readfsqword(0x28u);
  if ( (int)sub_EC90(a1) < 0 )
  {
    return (unsigned int)-1;
  }
  else
  {
    if ( dword_1E024 + 1 < (unsigned int)dword_1E35C )
      sub_6FF0(a1, 0x80000);
    v2 = a2 - 1;
    v3 = __errno_location();
    v10 = 0.25;
    while ( 1 )
    {
      sub_5A60((long long)v11);
      v4 = ptr;
      ptr = 0;
      v5 = fork();
      v6 = *v3;
      v7 = v5;
      if ( !v5 )
      {
        sub_5A80((long long)v11);
        *v3 = v6;
        close(0);
        close(1);
        return v7;
      }
      ptr = v4;
      sub_5A80((long long)v11);
      *v3 = v6;
      if ( (v7 & 0x80000000) == 0 )
        break;
      if ( v6 != 11 )
        goto LABEL_12;
      sub_13440(v11, 0x80000, v10);
      v10 = v10 + v10;
      sub_6FB0();
      if ( v2-- == 0 )
      {
        v6 = *v3;
LABEL_12:
        close(*a1);
        close(a1[1]);
        *v3 = v6;
        return v7;
      }
    }
    ++dword_1E35C;
  }
  return v7;
}



// Function: async_safe_die @ 0x71d0
void async_safe_die(unsigned int a1, const char *a2)
{
  size_t v2; // rax
  const char *v3; // rbp
  size_t v4; // rax
  char v5[12]; // [rsp+Ch] [rbp-2Ch] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-20h]

  v6 = __readfsqword(0x28u);
  v2 = strlen(a2);
  write(2, a2, v2);
  if ( a1 )
  {
    v3 = (const char *)sub_E440(a1, v5);
    write(2, ": errno ", 8u);
    v4 = strlen(v3);
    write(2, v3, v4);
  }
  write(2, "\n", 1u);
  _exit(2);
}



// Function: register_proc @ 0x7270
long long register_proc(long long a1)
{
  long long v2; // rdi
  long long result; // rax

  v2 = qword_1E360;
  if ( !qword_1E360 && (qword_1E360 = sub_D970(47, 0, sub_5400, sub_5420, 0), (v2 = qword_1E360) == 0)
    || (*(char *)(a1 + 12) = 1, (result = sub_DF90(v2, a1)) == 0) )
  {
    sub_13270(v2);
  }
  return result;
}



// Function: open_temp @ 0x72d0
long long open_temp(long long a1)
{
  int v1; // r12d
  int v2; // eax
  FILE *v3; // r12
  int *v5; // rbp
  int v6; // edi
  int *v7; // rax
  int v8; // ebx
  int *v9; // rbp
  int *v10; // rax
  long long v11; // r12
  char *v12; // rax
  int v13; // [rsp+0h] [rbp-28h] BYREF
  int fd; // [rsp+4h] [rbp-24h]
  unsigned long long v15; // [rsp+8h] [rbp-20h]

  v15 = __readfsqword(0x28u);
  if ( *(char *)(a1 + 12) == 1 )
    sub_6E60(*(int *)(a1 + 8));
  v1 = open((const char *)(a1 + 13), 0);
  if ( v1 < 0 )
    return 0;
  v2 = sub_7040(&v13, 9);
  if ( v2 != -1 )
  {
    if ( !v2 )
    {
      close(v13);
      sub_6C80(v1, 0);
      sub_6C80(fd, 1);
      execlp(arg, arg, "-d", 0);
      v10 = __errno_location();
      sub_71D0(*v10, "couldn't execute compress program (with -d)");
    }
    *(int *)(a1 + 8) = v2;
    sub_7270(a1);
    close(v1);
    close(fd);
    v3 = fdopen(v13, "r");
    if ( !v3 )
    {
      v7 = __errno_location();
      v8 = *v7;
      v9 = v7;
      close(v13);
      *v9 = v8;
    }
    return (long long)v3;
  }
  v5 = __errno_location();
  if ( *v5 == 24 )
  {
    v6 = v1;
    v3 = 0;
    close(v6);
    *v5 = 24;
    return (long long)v3;
  }
  v11 = sub_11170(4, arg);
  v12 = dcgettext(0, "couldn't create process for %s -d", 5);
  error(2, *v5, v12, v11);
  return sub_7470();
}



// Function: open_input_files @ 0x7470
long long open_input_files(long long a1, long long a2, long long *a3)
{
  long long v5; // rax
  long long v6; // rbp
  long long v7; // r13
  FILE *v8; // rax
  long long v9; // rdi
  long long v10; // rax

  v5 = sub_12FD0(a2);
  *a3 = v5;
  if ( a2 )
  {
    v6 = v5;
    v7 = 0;
    while ( 1 )
    {
      v9 = *(long long *)(a1 + 8);
      if ( v9 && *(char *)(v9 + 12) )
      {
        v10 = sub_72D0(v9);
        *(long long *)(v6 + 8 * v7) = v10;
        if ( !v10 )
          return v7;
      }
      else
      {
        v8 = sub_5EE0(*(char **)a1, "r");
        *(long long *)(v6 + 8 * v7) = v8;
        if ( !v8 )
          return v7;
      }
      ++v7;
      a1 += 16;
      if ( a2 == v7 )
        return v7;
    }
  }
  return 0;
}



// Function: create_temp_file @ 0x7510
long long create_temp_file(int *a1, char a2)
{
  const char *v2; // r14
  size_t v3; // rbx
  long long v4; // rax
  long long v5; // r15
  long long *v6; // r12
  long long v7; // rax
  int v8; // r15d
  int *v9; // rax
  int *v10; // rbx
  long long *v11; // rax
  int v12; // r13d
  long long *v14; // rdi
  long long v15; // r12
  char *v16; // rax
  int v18; // [rsp+Ch] [rbp-CCh]
  char v19[136]; // [rsp+10h] [rbp-C8h] BYREF
  unsigned long long v20; // [rsp+98h] [rbp-40h]

  v20 = __readfsqword(0x28u);
  v2 = *(const char **)(qword_1E430 + 8 * qword_1E248);
  v3 = strlen(v2);
  v4 = sub_12EB0((v3 + 32) & 0xFFFFFFFFFFFFFFF8LL);
  v5 = v4 + 13;
  v6 = (long long *)v4;
  memcpy((void *)(v4 + 13), v2, v3);
  *(long long *)((char *)v6 + v3 + 13) = 0x58585874726F732FLL;
  *(int *)(v5 + v3 + 8) = 5789784;
  v7 = qword_1E248;
  *v6 = 0;
  qword_1E248 = v7 + 1;
  if ( v7 + 1 == qword_1E428 )
    qword_1E248 = 0;
  sub_5A60((long long)v19);
  v8 = sub_12280(v5, 0x80000);
  v9 = __errno_location();
  v10 = v9;
  if ( v8 >= 0 )
  {
    v11 = off_1E040;
    v12 = *v10;
    off_1E040 = v6;
    *v11 = (long long)v6;
    sub_5A80((long long)v19);
    *v10 = v12;
LABEL_5:
    *a1 = v8;
    return (long long)v6;
  }
  v18 = *v9;
  sub_5A80((long long)v19);
  *v10 = v18;
  if ( a2 == 1 && v18 == 24 )
  {
    v14 = v6;
    v6 = 0;
    free(v14);
    goto LABEL_5;
  }
  v15 = sub_11170(4, v2);
  v16 = dcgettext(0, "cannot create temporary file in %s", 5);
  error(2, *v10, v16, v15);
  return sub_76B0();
}



// Function: maybe_create_temp @ 0x76b0
long long maybe_create_temp(FILE **a1, char a2)
{
  long long v2; // rax
  long long v3; // r12
  bool v4; // zf
  int v5; // eax
  FILE *v6; // rax
  char *v8; // rax
  int *v9; // rax
  int fd; // [rsp+Ch] [rbp-2Ch] BYREF
  int v11; // [rsp+10h] [rbp-28h] BYREF
  int v12; // [rsp+14h] [rbp-24h]
  unsigned long long v13; // [rsp+18h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  v2 = sub_7510(&fd, a2);
  v3 = v2;
  if ( v2 )
  {
    v4 = arg == 0;
    *(char *)(v2 + 12) = 0;
    if ( !v4 )
    {
      v5 = sub_7040(&v11, 4);
      *(int *)(v3 + 8) = v5;
      if ( v5 > 0 )
      {
        close(fd);
        close(v11);
        fd = v12;
        sub_7270(v3);
      }
      else if ( !v5 )
      {
        close(v12);
        sub_6C80(fd, 1);
        sub_6C80(v11, 0);
        execlp(arg, arg, 0);
        v9 = __errno_location();
        sub_71D0(*v9, "couldn't execute compress program");
      }
    }
    v6 = fdopen(fd, "w");
    *a1 = v6;
    if ( !v6 )
    {
      v8 = dcgettext(0, "couldn't create temporary file", 5);
      sub_5E10(v8, (char *)(v3 + 13));
    }
  }
  return v3;
}



// Function: create_temp @ 0x77e0
long long create_temp(FILE **a1)
{
  return sub_76B0(a1, 0);
}



// Function: default_sort_size @ 0x77f0
unsigned long long default_sort_size()
{
  rlim_t rlim_cur; // rbx
  rlim_t v1; // rbx
  double v2; // xmm0_8
  double v3; // xmm2_8
  double v4; // xmm0_8
  double v5; // xmm1_8
  double v6; // xmm0_8
  unsigned long long result; // rax
  double v8; // [rsp+8h] [rbp-40h]
  struct rlimit rlimits; // [rsp+10h] [rbp-38h] BYREF
  unsigned long long v10; // [rsp+28h] [rbp-20h]

  rlim_cur = -1;
  v10 = __readfsqword(0x28u);
  if ( !getrlimit(RLIMIT_DATA, &rlimits) )
    rlim_cur = rlimits.rlim_cur;
  if ( !getrlimit(RLIMIT_AS, &rlimits) && rlim_cur > rlimits.rlim_cur )
    rlim_cur = rlimits.rlim_cur;
  v1 = rlim_cur >> 1;
  if ( !getrlimit(__RLIMIT_RSS, &rlimits) && v1 > (rlimits.rlim_cur & 0xFFFFFFFFFFFFFFF0LL) - (rlimits.rlim_cur >> 4) )
    v1 = (rlimits.rlim_cur & 0xFFFFFFFFFFFFFFF0LL) - (rlimits.rlim_cur >> 4);
  v8 = sub_EB50();
  v2 = sub_EA60();
  v3 = 0.125 * v2;
  v4 = v2 * 0.75;
  v5 = fmax(v8, v3);
  if ( (double)(int)v1 > v4 )
  {
    if ( v4 < 9.223372036854776e18 )
    {
      v1 = (unsigned int)(int)v4;
      goto LABEL_13;
    }
    v1 = (unsigned int)(int)(v4 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  }
  if ( (v1 & 0x8000000000000000LL) == 0LL )
  {
LABEL_13:
    v6 = (double)(int)v1;
    goto LABEL_14;
  }
  v6 = (double)(int)(v1 & 1 | (v1 >> 1)) + (double)(int)(v1 & 1 | (v1 >> 1));
LABEL_14:
  if ( v6 > v5 )
  {
    if ( v5 < 9.223372036854776e18 )
      v1 = (unsigned int)(int)v5;
    else
      v1 = (unsigned int)(int)(v5 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  }
  result = 34LL * (unsigned int)dword_1E024;
  if ( result < v1 )
    return v1;
  return result;
}



// Function: init_node @ 0x7970
long long init_node(long long a1, long long a2, long long a3, unsigned long long a4, long long a5, char a6)
{
  long long v6; // r13
  long long v7; // rbp
  long long v8; // r12
  unsigned long long v9; // rbx
  long long v10; // rcx
  long long v11; // rdx
  long long v12; // r14
  long long v13; // rsi
  int v14; // eax
  long long v16; // rax

  while ( 1 )
  {
    v6 = a5;
    v7 = a2;
    v8 = a3 - 32 * a5;
    v9 = a4;
    if ( a6 )
    {
      v10 = *(long long *)(a1 + 40) >> 1;
      v11 = *(long long *)(a1 + 40) - v10;
      v12 = v8 - 32 * v10;
      v13 = a1 + 16;
    }
    else
    {
      v10 = *(long long *)(a1 + 48) >> 1;
      v11 = *(long long *)(a1 + 48) - v10;
      v12 = v8 - 32 * v10;
      v13 = a1 + 24;
    }
    *(long long *)(v7 + 56) = a1;
    v14 = *(int *)(a1 + 80);
    *(long long *)(v7 + 32) = v13;
    *(long long *)(v7 + 16) = v8;
    *(long long *)v7 = v8;
    *(long long *)(v7 + 24) = v12;
    *(long long *)(v7 + 8) = v12;
    *(long long *)(v7 + 40) = v10;
    *(long long *)(v7 + 48) = v11;
    *(int *)(v7 + 80) = v14 + 1;
    *(char *)(v7 + 84) = 0;
    pthread_mutex_init((pthread_mutex_t *)(v7 + 88), 0);
    if ( v9 <= 1 )
      break;
    *(long long *)(v7 + 64) = v7 + 128;
    v16 = init_node(v7, v7 + 128, v8, v9 >> 1, v6, 1);
    a5 = v6;
    a4 = v9 - (v9 >> 1);
    a3 = v12;
    *(long long *)(v7 + 72) = v16;
    a1 = v7;
    a2 = v16;
    a6 = 0;
  }
  *(long long *)(v7 + 64) = 0;
  *(long long *)(v7 + 72) = 0;
  return v7 + 128;
}



// Function: merge_tree_init @ 0x7aa0
long long merge_tree_init(unsigned long long a1, long long a2, long long a3)
{
  long long v4; // rax
  long long v5; // r12

  v4 = sub_12EB0(a1 << 8);
  *(long long *)(v4 + 48) = a2;
  v5 = v4;
  *(long long *)(v4 + 40) = a2;
  *(long long *)(v4 + 24) = 0;
  *(long long *)(v4 + 16) = 0;
  *(long long *)(v4 + 8) = 0;
  *(long long *)v4 = 0;
  *(long long *)(v4 + 32) = 0;
  *(long long *)(v4 + 56) = 0;
  *(int *)(v4 + 80) = 0;
  *(char *)(v4 + 84) = 0;
  pthread_mutex_init((pthread_mutex_t *)(v4 + 88), 0);
  sub_7970(v5, v5 + 128, a3, a1, a2, 0);
  return v5;
}



// Function: queue_init @ 0x7b40
int queue_init(long long a1, long long a2)
{
  *(long long *)a1 = sub_E300(sub_5780, 2 * a2);
  pthread_mutex_init((pthread_mutex_t *)(a1 + 8), 0);
  return pthread_cond_init((pthread_cond_t *)(a1 + 48), 0);
}



// Function: lock_node @ 0x7b70
int lock_node(long long a1)
{
  return pthread_mutex_lock((pthread_mutex_t *)(a1 + 88));
}



// Function: queue_insert @ 0x7b80
int queue_insert(long long a1, long long a2)
{
  pthread_mutex_lock((pthread_mutex_t *)(a1 + 8));
  sub_E390(*(long long *)a1, a2);
  *(char *)(a2 + 84) = 1;
  pthread_cond_signal((pthread_cond_t *)(a1 + 48));
  return pthread_mutex_unlock((pthread_mutex_t *)(a1 + 8));
}



// Function: queue_check_insert @ 0x7bc0
void queue_check_insert(long long a1, long long a2)
{
  long long v2; // rdx
  long long v3; // rax

  if ( !*(char *)(a2 + 84) )
  {
    v2 = *(long long *)(a2 + 8);
    v3 = *(long long *)(a2 + 24);
    if ( *(long long *)a2 != *(long long *)(a2 + 16) )
    {
      if ( v2 == v3 && *(long long *)(a2 + 48) )
        return;
LABEL_4:
      sub_7B80(a1, a2);
      return;
    }
    if ( v2 != v3 && !*(long long *)(a2 + 40) )
      goto LABEL_4;
  }
}



// Function: unlock_node @ 0x7c00
int unlock_node(long long a1)
{
  return pthread_mutex_unlock((pthread_mutex_t *)(a1 + 88));
}



// Function: queue_check_insert_parent @ 0x7c10
int queue_check_insert_parent(long long a1, long long a2)
{
  long long v2; // rax

  if ( *(int *)(a2 + 80) > 1u )
  {
    sub_7B70(*(long long *)(a2 + 56));
    sub_7BC0(a1, *(long long *)(a2 + 56));
    LODWORD(v2) = sub_7C00(*(long long *)(a2 + 56));
  }
  else
  {
    v2 = *(long long *)(a2 + 40) + *(long long *)(a2 + 48);
    if ( !v2 )
      LODWORD(v2) = sub_7B80(a1, *(long long *)(a2 + 56));
  }
  return v2;
}



// Function: queue_pop @ 0x7c70
long long queue_pop(long long a1)
{
  pthread_mutex_t *v1; // rbp
  long long v2; // r12

  v1 = (pthread_mutex_t *)(a1 + 8);
  pthread_mutex_lock((pthread_mutex_t *)(a1 + 8));
  while ( 1 )
  {
    v2 = sub_E400(*(long long *)a1);
    if ( v2 )
      break;
    pthread_cond_wait((pthread_cond_t *)(a1 + 48), v1);
  }
  pthread_mutex_unlock(v1);
  sub_7B70(v2);
  *(char *)(v2 + 84) = 0;
  return v2;
}



// Function: merge_tree_destroy @ 0x7cd0
void merge_tree_destroy(long long a1, char *a2)
{
  pthread_mutex_t *v2; // rbx
  long long v3; // rbp
  pthread_mutex_t *v4; // rdi

  if ( 2 * a1 )
  {
    v2 = (pthread_mutex_t *)(a2 + 88);
    v3 = (long long)&a2[256 * a1 + 88];
    do
    {
      v4 = v2;
      v2 = (pthread_mutex_t *)((char *)v2 + 128);
      pthread_mutex_destroy(v4);
    }
    while ( v2 != (pthread_mutex_t *)v3 );
  }
  free(a2);
}



// Function: queue_destroy @ 0x7d10
int queue_destroy(long long a1)
{
  sub_E370(*(void **)a1);
  pthread_cond_destroy((pthread_cond_t *)(a1 + 48));
  return pthread_mutex_destroy((pthread_mutex_t *)(a1 + 8));
}



// Function: key_warnings @ 0x7d30
void key_warnings(const __m128i *a1, char a2)
{
  long long v2; // rax
  long long v3; // r13
  int v4; // r14d
  int v5; // r12d
  int v6; // ebp
  int v7; // r15d
  int v8; // edx
  long long v9; // rdx
  int v10; // eax
  long long v11; // rsi
  int v12; // eax
  long long v13; // rcx
  int v14; // eax
  bool v15; // al
  char v16; // dl
  bool v17; // di
  char v18; // al
  long long v19; // rdi
  long long v20; // rbx
  long long v21; // rax
  long long v22; // rax
  const char *v23; // rax
  const char *v24; // rax
  long long v25; // rbx
  char *v26; // rax
  char *v27; // rax
  char v28; // dl
  bool v29; // bl
  int v30; // eax
  unsigned long long v31; // rax
  char *v32; // rax
  bool v33; // al
  char v34; // al
  int v35; // eax
  char v36; // bl
  const unsigned short **v37; // rax
  unsigned char v38; // al
  long long *v39; // rdx
  const unsigned short **v40; // rax
  unsigned char v41; // al
  long long *v42; // rdx
  long long v43; // r12
  char *v44; // rax
  char *v45; // rax
  char v46; // al
  char v47; // bl
  char *v48; // rax
  size_t v49; // rax
  unsigned long long v50; // rax
  const char *v51; // rdx
  int v52; // r8d
  char *v53; // rax
  char *v54; // rdx
  char *v55; // r12
  long long v56; // rbx
  char *v57; // rax
  size_t v58; // rax
  unsigned long long v59; // rax
  const char *v60; // rdx
  int v61; // r8d
  char *v62; // rax
  size_t v63; // rax
  unsigned long long v64; // rax
  const char *v65; // rdx
  int v66; // r8d
  char *v67; // rax
  long long v68; // rax
  const char *v69; // rsi
  long long v70; // r12
  char *v71; // rax
  long long v72; // r12
  char *v73; // rax
  long long v74; // rax
  char v75; // [rsp+Eh] [rbp-14Ah]
  char v76; // [rsp+Fh] [rbp-149h]
  long long v77; // [rsp+10h] [rbp-148h]
  char *v78; // [rsp+18h] [rbp-140h]
  long long v79; // [rsp+18h] [rbp-140h]
  long long v80; // [rsp+20h] [rbp-138h]
  long long v81; // [rsp+20h] [rbp-138h]
  long long v82; // [rsp+28h] [rbp-130h]
  unsigned char v83; // [rsp+38h] [rbp-120h]
  unsigned char v84; // [rsp+39h] [rbp-11Fh]
  char v85; // [rsp+3Ah] [rbp-11Eh]
  bool v87; // [rsp+3Ch] [rbp-11Ch]
  char v88; // [rsp+3Dh] [rbp-11Bh]
  bool v89; // [rsp+3Eh] [rbp-11Ah]
  char v90; // [rsp+3Fh] [rbp-119h]
  _OWORD v91[2]; // [rsp+40h] [rbp-118h] BYREF
  __m128i v92; // [rsp+60h] [rbp-F8h]
  __m128i v93; // [rsp+70h] [rbp-E8h]
  long long v94; // [rsp+80h] [rbp-D8h]
  char v95[32]; // [rsp+90h] [rbp-C8h] BYREF
  char v96; // [rsp+B0h] [rbp-A8h] BYREF
  char v97[47]; // [rsp+B1h] [rbp-A7h] BYREF
  char s[56]; // [rsp+E0h] [rbp-78h] BYREF
  unsigned long long v99; // [rsp+118h] [rbp-40h]

  v99 = __readfsqword(0x28u);
  v2 = a1[4].m128i_i64[0];
  v91[0] = _mm_loadu_si128(a1);
  v91[1] = _mm_loadu_si128(a1 + 1);
  v94 = v2;
  v92 = _mm_loadu_si128(a1 + 2);
  v93 = _mm_loadu_si128(a1 + 3);
  if ( !qword_1E410 )
    goto LABEL_53;
  v90 = 0;
  v3 = qword_1E410;
  v89 = 0;
  v4 = v93.m128i_u8[0];
  v75 = v93.m128i_i8[1];
  v5 = v93.m128i_u8[4];
  v6 = v93.m128i_u8[8];
  v88 = 0;
  v76 = v93.m128i_i8[6];
  v87 = 0;
  v7 = v93.m128i_u8[3];
  v83 = v93.m128i_u8[2];
  v77 = 1;
  v84 = v93.m128i_u8[5];
  v85 = v93.m128i_i8[7];
  do
  {
    v15 = sub_5660(v3);
    if ( v15 )
    {
      v16 = *(char *)(v3 + 52);
      v17 = v87;
      if ( !v16 )
        v17 = v15;
      v18 = v88;
      if ( v16 )
        v18 = *(char *)(v3 + 52);
      v87 = v17;
      v88 = v18;
    }
    if ( *(char *)(v3 + 57) )
    {
      v19 = *(long long *)v3;
      v20 = *(long long *)v3 + 1LL;
      v80 = *(long long *)(v3 + 16);
      if ( *(long long *)v3 == -1 )
      {
        v20 = 1;
        v19 = 0;
      }
      v21 = sub_E520(v19, v95);
      v96 = 43;
      v82 = __stpcpy_chk(v97, v21, 45);
      v22 = sub_E520(v20, v95);
      strcpy(s, "-k ");
      v78 = (char *)__stpcpy_chk(&s[3], v22, 44);
      if ( *(long long *)(v3 + 16) != -1 )
      {
        v81 = v80 + 1;
        v23 = (const char *)sub_E520(v81, v95);
        *(short *)v82 = 11552;
        strcpy((char *)(v82 + 2), v23);
        v24 = (const char *)sub_E520(v81 + (*(long long *)(v3 + 24) == -1), v95);
        *v78 = 44;
        strcpy(v78 + 1, v24);
      }
      v25 = sub_11420(1, s);
      v79 = sub_11420(0, &v96);
      v26 = dcgettext(0, "obsolescent key %s used; consider %s instead", 5);
      error(0, 0, v26, v79, v25);
    }
    if ( *(long long *)v3 != -1 && *(long long *)v3 > *(long long *)(v3 + 16) )
    {
      v27 = dcgettext(0, "key %lu has zero width and will be ignored", 5);
      error(0, 0, v27, v77);
    }
    v29 = sub_5660(v3);
    v30 = 1;
    if ( !v29 )
      v30 = *(unsigned char *)(v3 + 54);
    if ( *(long long *)(v3 + 16) || !*(long long *)(v3 + 24) )
    {
      if ( !v28 )
      {
        if ( dword_1E028 == 128
          && (!*(char *)(v3 + 48) && (!v30 || *(long long *)(v3 + 8)) || !*(char *)(v3 + 49) && *(long long *)(v3 + 24)) )
        {
          v48 = dcgettext(0, "leading blanks are significant in key %lu; consider also specifying 'b'", 5);
          error(0, 0, v48, v77);
          v29 = sub_5660(v3);
        }
        goto LABEL_34;
      }
    }
    else if ( !v28 )
    {
      goto LABEL_6;
    }
    if ( a2 )
      goto LABEL_6;
LABEL_34:
    if ( !v29 )
      goto LABEL_6;
    v31 = *(long long *)v3 + 1LL;
    if ( *(long long *)v3 == -1 )
      v31 = 1;
    if ( *(long long *)(v3 + 16) < v31 )
    {
LABEL_6:
      v8 = *(unsigned char *)(v3 + 52);
      goto LABEL_7;
    }
    v32 = dcgettext(0, "key %lu is numeric and spans multiple fields", 5);
    error(0, 0, v32, v77);
    v8 = *(unsigned char *)(v3 + 52);
    v33 = v89;
    if ( !(char)v8 )
      v33 = v29;
    v89 = v33;
    v34 = v90;
    if ( (char)v8 )
      v34 = *(char *)(v3 + 52);
    v90 = v34;
LABEL_7:
    if ( v92.m128i_i64[0] && v92.m128i_i64[0] == *(long long *)(v3 + 32) )
      v92.m128i_i64[0] = 0;
    if ( v92.m128i_i64[1] && v92.m128i_i64[1] == *(long long *)(v3 + 40) )
      v92.m128i_i64[1] = 0;
    v9 = v8 ^ 1u;
    v5 &= v9;
    v10 = *(unsigned char *)(v3 + 48) ^ 1;
    v93.m128i_i8[4] = v5;
    v4 &= v10;
    LOBYTE(v10) = *(char *)(v3 + 49);
    v93.m128i_i8[0] = v4;
    v75 &= v10 ^ 1;
    v76 &= *(char *)(v3 + 54) ^ 1;
    LOBYTE(v10) = *(char *)(v3 + 50);
    v93.m128i_i8[1] = v75;
    v83 &= (unsigned char)v10 ^ 1;
    LOBYTE(v10) = *(char *)(v3 + 53);
    v93.m128i_i8[6] = v76;
    v11 = v83;
    v84 &= (unsigned char)v10 ^ 1;
    v12 = *(unsigned char *)(v3 + 51);
    v13 = v84;
    v93.m128i_i8[2] = v83;
    v7 &= v12 ^ 1;
    v14 = *(unsigned char *)(v3 + 56);
    v93.m128i_i8[5] = v84;
    v93.m128i_i8[3] = v7;
    v6 &= v14 ^ 1;
    LOBYTE(v14) = *(char *)(v3 + 55);
    v93.m128i_i8[8] = v6;
    v85 &= v14 ^ 1;
    v93.m128i_i8[7] = v85;
    v3 = *(long long *)(v3 + 64);
    ++v77;
  }
  while ( v3 );
  if ( !v89 )
  {
    if ( v90 )
      goto LABEL_87;
    if ( v87 || v88 )
    {
LABEL_80:
      s[1] = 0;
      v55 = (char *)"";
      s[0] = byte_1E868;
      v56 = sub_11430(s, v11, v9, v13);
      if ( byte_1E868 != dword_1E028 )
        v55 = dcgettext(0, "note ", 5);
      v57 = dcgettext(0, "%snumbers use %s as a decimal point in this locale", 5);
      error(0, 0, v57, v55, v56);
      goto LABEL_50;
    }
    goto LABEL_53;
  }
  v35 = dword_1E028;
  v36 = dword_1E864;
  if ( dword_1E028 != 128 )
  {
    if ( dword_1E028 == dword_1E864 )
      goto LABEL_97;
    v89 = 0;
LABEL_73:
    v9 = (unsigned int)byte_1E868;
    if ( (int)v9 != v35 )
    {
      if ( v35 == 45 )
      {
        strcpy(s, "-");
        v68 = sub_11430(s, v11, v9, v13);
        v69 = "field separator %s is treated as a minus sign in numbers";
        v70 = v68;
      }
      else
      {
        if ( v35 != 43 || !v90 )
          goto LABEL_77;
        strcpy(s, "+");
        v74 = sub_11430(s, v11, v9, v13);
        v69 = "field separator %s is treated as a plus sign in numbers";
        v70 = v74;
      }
      v71 = dcgettext(0, v69, 5);
      v11 = 0;
      error(0, 0, v71, v70);
      goto LABEL_77;
    }
LABEL_49:
    s[1] = 0;
    s[0] = byte_1E868;
    v43 = sub_11430(s, v11, v9, v13);
    v44 = dcgettext(0, "field separator %s is treated as a decimal point in numbers", 5);
    error(0, 0, v44, v43);
    goto LABEL_50;
  }
  if ( dword_1E864 == 128 )
  {
    v89 = 0;
    goto LABEL_99;
  }
  v37 = __ctype_b_loc();
  v38 = sub_53E0((unsigned int)v36, v83, v37);
  v9 = *v39;
  if ( (*(char *)(v9 + 2LL * v38) & 1) == 0 )
  {
    v89 = 0;
    goto LABEL_48;
  }
LABEL_97:
  s[0] = v36;
  s[1] = 0;
  v72 = sub_11430(s, v83, v9, v13);
  v73 = dcgettext(0, "field separator %s is treated as a group separator in numbers", 5);
  v11 = 0;
  error(0, 0, v73, v72);
LABEL_87:
  v35 = dword_1E028;
  if ( dword_1E028 != 128 )
    goto LABEL_73;
LABEL_99:
  if ( dword_1E864 != 128 )
  {
LABEL_48:
    v40 = __ctype_b_loc();
    v41 = sub_53E0((unsigned int)byte_1E868, v11, v40);
    v9 = *v42;
    if ( (*(char *)(v9 + 2LL * v41) & 1) != 0 )
      goto LABEL_49;
  }
LABEL_77:
  if ( v87 || v88 )
  {
    if ( !v89 )
      goto LABEL_80;
LABEL_50:
    if ( v87 && byte_1E862 )
    {
      v45 = dcgettext(0, "the multi-byte number group separator in this locale is not supported", 5);
      error(0, 0, v45);
    }
  }
LABEL_53:
  v46 = sub_5680((long long)v91);
  v47 = v93.m128i_i8[7];
  if ( !v46 )
  {
    if ( !byte_1E41A && !byte_1E419 )
      v93.m128i_i8[7] = 0;
    sub_56D0((long long)v91, s);
    v49 = strlen(s);
    v50 = sub_53F0(v49, "option '-%s' is ignored", "options '-%s' are ignored");
    v53 = dcngettext(0, "option '-%s' is ignored", v51, v50, v52);
    error(0, 0, v53, s);
    v93.m128i_i8[7] = v47;
    if ( !v47 )
      return;
    goto LABEL_64;
  }
  if ( !v93.m128i_i8[7] )
    return;
  if ( !byte_1E41A )
  {
    if ( !byte_1E419 || !qword_1E410 )
    {
LABEL_65:
      if ( !byte_1E419 )
      {
        if ( qword_1E410 )
        {
          v54 = dcgettext(0, "option '-r' only applies to last-resort comparison", 5);
          error(0, 0, v54);
        }
      }
      return;
    }
    sub_56D0((long long)v91, s);
    v63 = strlen(s);
    v64 = sub_53F0(v63, "option '-%s' is ignored", "options '-%s' are ignored");
    v67 = dcngettext(0, "option '-%s' is ignored", v65, v64, v66);
    error(0, 0, v67, s);
    v93.m128i_i8[7] = v47;
LABEL_64:
    if ( byte_1E41A )
      return;
    goto LABEL_65;
  }
  if ( qword_1E410 )
  {
    sub_56D0((long long)v91, s);
    v58 = strlen(s);
    v59 = sub_53F0(v58, "option '-%s' is ignored", "options '-%s' are ignored");
    v62 = dcngettext(0, "option '-%s' is ignored", v60, v59, v61);
    error(0, 0, v62, s);
    v93.m128i_i8[7] = 1;
    goto LABEL_64;
  }
}



// Function: debug_line @ 0x9400
long long debug_line(long long *a1)
{
  long long v1; // rbx
  long long v3; // [rsp+0h] [rbp-18h]

  v1 = qword_1E410;
  while ( 1 )
  {
    sub_9230(*a1, a1[1], v1);
    if ( !v1 )
      break;
    v1 = *(long long *)(v1 + 64);
    if ( !v1 )
    {
      if ( !byte_1E419 && !byte_1E41A )
        return sub_9230(*a1, a1[1], 0);
      return v3;
    }
  }
  return v3;
}



// Function: write_line @ 0x9460
size_t write_line(const void **a1, FILE *a2, char *a3)
{
  char *v4; // rbp
  char *v5; // r15
  char *v6; // rbx
  size_t v7; // rdx
  const void *v8; // rdi
  size_t result; // rax
  int v11; // eax
  int v12; // edi
  char *v13; // rax
  char *v14; // rax

  v4 = (char *)*a1;
  v5 = (char *)a1[1];
  v6 = &v5[(long long)*a1];
  if ( !a3 && byte_1E400 )
  {
    while ( v4 < v6 )
    {
      v11 = *v4++;
      v12 = 62;
      if ( (char)v11 != 9 )
      {
        v12 = 10;
        if ( v6 != v4 )
          v12 = v11;
      }
      if ( fputc_unlocked(v12, a2) == -1 )
      {
        v13 = dcgettext(0, "write failed", 5);
        sub_5E10(v13, 0);
      }
    }
    return sub_9400(a1);
  }
  else
  {
    v7 = (size_t)a1[1];
    v8 = *a1;
    *(v6 - 1) = byte_1E038;
    result = fwrite_unlocked(v8, 1u, v7, a2);
    if ( (char *)result != v5 )
    {
      v14 = dcgettext(0, "write failed", 5);
      sub_5E10(v14, a3);
    }
    *(v6 - 1) = 0;
  }
  return result;
}



// Function: keycompare @ 0x9550
int keycompare(char **a1, char *a2)
{
  char *v2; // r14
  char *v3; // r15
  long long v4; // rbp
  unsigned long long v5; // r12
  char *v6; // r11
  char *v7; // rcx
  long long v8; // rbx
  size_t v9; // r13
  size_t v10; // r12
  long long v11; // rdx
  unsigned char v12; // al
  long long v13; // r9
  long long v14; // rdx
  long long v15; // rdx
  long long v16; // r9
  unsigned char v17; // al
  unsigned char v18; // r8
  unsigned char v19; // al
  int result; // eax
  unsigned char v21; // al
  char v22; // al
  char v23; // al
  char *v24; // r13
  char *v25; // rdx
  unsigned char v26; // al
  long long v27; // rcx
  unsigned long long v28; // rdi
  char *v29; // rcx
  long long v30; // rdx
  long long v31; // r8
  char *v32; // r10
  long long v33; // rdi
  unsigned char v34; // al
  long long v35; // r8
  long long v36; // rdx
  long long v37; // r8
  char *v38; // r9
  long long v39; // rdi
  unsigned char v40; // al
  long long v41; // r8
  char *v42; // rax
  int v43; // ebx
  size_t v44; // rdx
  bool v45; // cf
  long long v46; // rdi
  unsigned char v47; // al
  long long v48; // rdx
  unsigned int v49; // eax
  unsigned char v50; // al
  long long v51; // rdx
  unsigned char v52; // r8
  unsigned char v53; // dl
  int v54; // eax
  char *v55; // [rsp+8h] [rbp-1020h]
  int v56; // [rsp+8h] [rbp-1020h]
  char **v57; // [rsp+10h] [rbp-1018h]
  unsigned long long v58; // [rsp+18h] [rbp-1010h]
  char v59; // [rsp+26h] [rbp-1002h]
  char v60; // [rsp+27h] [rbp-1001h]
  void *ptr; // [rsp+30h] [rbp-FF8h]
  char v63[4008]; // [rsp+40h] [rbp-FE8h] BYREF
  unsigned long long v64; // [rsp+FE8h] [rbp-40h]

  v2 = a1[2];
  v3 = (char *)*((long long *)a2 + 2);
  v4 = qword_1E410;
  v5 = (unsigned long long)a1[3];
  v57 = (char **)a2;
  v6 = (char *)*((long long *)a2 + 3);
  v64 = __readfsqword(0x28u);
  while ( 1 )
  {
    v7 = (char *)v5;
    v8 = *(long long *)(v4 + 32);
    if ( (unsigned long long)v2 >= v5 )
      v7 = v2;
    if ( v3 >= v6 )
      v6 = v3;
    v55 = *(char **)(v4 + 40);
    v9 = v7 - v2;
    v10 = v6 - v3;
    if ( byte_1E861 || sub_5660(v4) || (*(long long *)(v4 + 48) & 0xFF0000FF000000LL) != 0 || *(char *)(v4 + 56) )
    {
      v58 = v8 | (unsigned long long)v55;
      if ( !(v8 | (unsigned long long)v55) )
      {
        v22 = *v7;
        *v7 = 0;
        v59 = v22;
        v23 = *v6;
        *v6 = 0;
        v60 = v23;
        goto LABEL_37;
      }
      v28 = v9 + v10 + 2;
      v29 = v63;
      ptr = 0;
      if ( v28 > 0xFA0 )
      {
        v29 = (char *)sub_12EB0(v28);
        ptr = v29;
        a2 = &v29[v9 + 1];
        if ( v9 )
          goto LABEL_66;
      }
      else
      {
        a2 = &v63[v9 + 1];
        if ( v9 )
        {
LABEL_66:
          v9 = 0;
          while ( 1 )
          {
            v30 = (unsigned char)*v2;
            v33 = (unsigned int)(char)v30;
            if ( !v8 )
              goto LABEL_72;
            if ( !*(char *)(v8 + (unsigned char)sub_53E0(v33, a2, v30)) )
              break;
LABEL_70:
            if ( v32 == ++v2 )
            {
              v29[v9] = 0;
              if ( v10 )
                goto LABEL_74;
LABEL_90:
              v42 = a2;
LABEL_82:
              *v42 = 0;
              v3 = a2;
              v2 = v29;
LABEL_37:
              if ( *(char *)(v4 + 50) )
              {
                a2 = v3;
                result = sub_64D0(v2, v3);
                goto LABEL_39;
              }
              if ( *(char *)(v4 + 52) )
              {
                a2 = v3;
                result = sub_63D0(v2, v3);
              }
              else
              {
                if ( *(char *)(v4 + 53) )
                {
                  a2 = v3;
                  result = sub_6530(v2, v3);
                  if ( v58 )
                    goto LABEL_50;
LABEL_40:
                  v2[v9] = v59;
                  v3[v10] = v60;
                  goto LABEL_41;
                }
                if ( *(char *)(v4 + 54) )
                {
                  a2 = 0;
                  v43 = sub_5570(v2, 0);
                  result = v43 - sub_5570(v3, 0);
                }
                else if ( *(char *)(v4 + 51) )
                {
                  a2 = (char *)v9;
                  result = sub_6680(v2, v9, v3, v10);
                }
                else if ( *(char *)(v4 + 56) )
                {
                  a2 = (char *)v9;
                  result = sub_CD70(v2, v9, v3, v10);
                }
                else if ( v9 )
                {
                  result = 1;
                  if ( v10 )
                  {
                    a2 = (char *)(v9 + 1);
                    result = sub_133E0(v2, v9 + 1, v3, v10 + 1);
                  }
                }
                else
                {
                  result = -(v10 != 0);
                }
              }
LABEL_39:
              if ( !v58 )
                goto LABEL_40;
LABEL_50:
              v56 = result;
              free(ptr);
              result = v56;
              if ( !v56 )
                goto LABEL_42;
LABEL_51:
              if ( *(char *)(v4 + 55) )
                return -result;
              return result;
            }
          }
          if ( v31 )
          {
LABEL_72:
            v34 = sub_53E0(v33, a2, v30);
            LOBYTE(v30) = *(char *)(v35 + v34);
          }
          v29[v9++] = v30;
          goto LABEL_70;
        }
      }
      *v29 = 0;
      if ( !v10 )
        goto LABEL_90;
LABEL_74:
      v10 = 0;
      while ( 1 )
      {
        v36 = (unsigned char)*v3;
        v39 = (unsigned int)(char)v36;
        if ( !v8 )
          goto LABEL_80;
        if ( !*(char *)(v8 + (unsigned char)sub_53E0(v39, a2, v36)) )
          break;
LABEL_78:
        if ( v38 == ++v3 )
        {
          v42 = &a2[v10];
          goto LABEL_82;
        }
      }
      if ( v37 )
      {
LABEL_80:
        v40 = sub_53E0(v39, a2, v36);
        LOBYTE(v36) = *(char *)(v41 + v40);
      }
      a2[v10++] = v36;
      goto LABEL_78;
    }
    if ( !v8 )
    {
      if ( !v9 )
      {
        result = -(v10 != 0);
        goto LABEL_41;
      }
      if ( v10 )
      {
        a2 = v55;
        if ( v55 )
        {
          while ( v2 < v7 && v3 < v6 )
          {
            v46 = (unsigned int)*v2;
            ++v3;
            ++v2;
            v47 = sub_53E0(v46, v55, v11);
            v49 = sub_53E0((unsigned int)v55[v47], v55, v48);
            v50 = sub_53E0((unsigned int)*(v3 - 1), v55, v49);
            v52 = sub_53E0((unsigned int)v55[v50], v55, v51);
            v54 = v53;
            v11 = v52;
            result = v54 - v52;
            if ( result )
              goto LABEL_51;
          }
        }
        else
        {
          v44 = v10;
          a2 = v3;
          if ( v9 <= v10 )
            v44 = v9;
          result = memcmp(v2, v3, v44);
          if ( result )
            goto LABEL_51;
        }
        if ( v9 < v10 )
        {
          result = -1;
          goto LABEL_51;
        }
        if ( v9 == v10 )
          goto LABEL_42;
      }
      result = 1;
      goto LABEL_51;
    }
    a2 = v2;
    if ( v55 )
      break;
    while ( 1 )
    {
      if ( v2 < v7 && *(char *)(v8 + (unsigned char)sub_53E0((unsigned int)*v2, a2, v11)) )
        goto LABEL_27;
      if ( v3 >= v6 )
        break;
      while ( 1 )
      {
        v21 = sub_53E0((unsigned int)*v3, a2, v11);
        v11 = v21;
        if ( !*(char *)(v8 + v21) )
          break;
        if ( v6 == ++v3 )
          goto LABEL_117;
      }
      if ( v7 <= v2 || v6 <= v3 )
        break;
      v19 = sub_53E0((unsigned int)*v2, a2, v21);
      result = v19 - (int)v11;
      if ( result )
        goto LABEL_51;
      ++v3;
LABEL_27:
      ++v2;
    }
LABEL_117:
    v45 = v2 < v7;
LABEL_109:
    result = v45 - (v6 > v3);
LABEL_41:
    if ( result )
      goto LABEL_51;
LABEL_42:
    v4 = *(long long *)(v4 + 64);
    if ( !v4 )
      return 0;
    v2 = *a1;
    v24 = a1[1];
    if ( *(long long *)(v4 + 16) == -1 )
    {
      v5 = (unsigned long long)&v24[(long long)v2 - 1];
      v25 = *v57;
      v6 = &v57[1][(long long)*v57 - 1];
    }
    else
    {
      v5 = sub_8F70(*a1, a1[1], v4);
      a2 = v57[1];
      v6 = (char *)sub_8F70(*v57, a2, v4);
    }
    if ( *(long long *)v4 == -1 )
    {
      v3 = *v57;
      if ( *(char *)(v4 + 48) )
      {
        if ( v5 > (unsigned long long)v2 )
        {
          do
          {
            v26 = sub_53E0((unsigned int)*v2, a2, v25);
            if ( !*(char *)(v27 + v26) )
              break;
            ++v2;
          }
          while ( (char *)v5 != v2 );
        }
        if ( v6 > v3 )
        {
          do
          {
            a2 = byte_1E740;
            if ( !byte_1E740[(unsigned char)sub_53E0((unsigned int)*v3, byte_1E740, v25)] )
              break;
            ++v3;
          }
          while ( v6 != v3 );
        }
      }
    }
    else
    {
      v2 = (char *)sub_90D0(v2, v24, v4);
      a2 = v57[1];
      v3 = (char *)sub_90D0(*v57, a2, v4);
    }
  }
  while ( a2 < v7 && *(char *)(v8 + (unsigned char)sub_53E0((unsigned int)*a2, a2, v11)) )
LABEL_22:
    ++a2;
  if ( v3 >= v6 )
    goto LABEL_108;
  while ( 1 )
  {
    v11 = (unsigned char)sub_53E0((unsigned int)*v3, a2, v11);
    if ( !*(char *)(v8 + v11) )
      break;
    if ( v6 == ++v3 )
      goto LABEL_108;
  }
  if ( v7 <= a2 || v6 <= v3 )
  {
LABEL_108:
    v45 = a2 < v7;
    goto LABEL_109;
  }
  v12 = sub_53E0((unsigned int)*a2, a2, v11);
  sub_53E0((unsigned int)*(char *)(v13 + v12), a2, v14);
  v17 = sub_53E0((unsigned int)*(char *)(v16 + v15), a2, v15);
  v11 = v18 - (unsigned int)v17;
  if ( v18 == v17 )
  {
    ++v3;
    goto LABEL_22;
  }
  result = v18 - v17;
  if ( *(char *)(v4 + 55) )
    return -result;
  return result;
}



// Function: compare @ 0x9c70
int compare(long long a1, char *a2)
{
  int result; // eax
  long long v5; // r12
  long long v6; // r14
  unsigned long long v7; // r13
  unsigned long long v8; // r15
  const void *v9; // rsi
  char *v10; // rdi
  size_t v11; // rdx

  if ( !qword_1E410 || (result = sub_9550((char **)a1, a2)) == 0 && !byte_1E419 && !byte_1E41A )
  {
    v5 = *(long long *)(a1 + 8);
    v6 = *((long long *)a2 + 1);
    v7 = v6 - 1;
    v8 = v5 - 1;
    if ( v5 == 1 )
    {
      result = -(v6 != 1);
    }
    else
    {
      result = 1;
      if ( v6 != 1 )
      {
        v9 = *(const void **)a2;
        v10 = *(char **)a1;
        if ( byte_1E861 )
        {
          result = sub_133E0(v10, *(long long *)(a1 + 8), *(long long *)a2, *((long long *)a2 + 1));
        }
        else
        {
          v11 = v6 - 1;
          if ( v8 <= v7 )
            v11 = v5 - 1;
          result = memcmp(v10, v9, v11);
          if ( !result )
          {
            result = -1;
            if ( v8 >= v7 )
              result = v5 != v6;
          }
        }
      }
    }
    if ( byte_1E41B )
      return -result;
  }
  return result;
}



// Function: mergelines @ 0x9d40
int mergelines(__m128i *a1, unsigned long long a2, long long a3)
{
  long long v3; // r15
  long long v4; // r13
  unsigned long long v5; // r12
  __m128i *v6; // rbx
  char *m128i_i8; // r14
  char *i; // rbp
  long long v9; // rax
  bool v10; // zf
  __m128i v11; // xmm3
  __m128i v12; // xmm5
  unsigned long long v14; // [rsp+8h] [rbp-40h]

  v3 = a3;
  v4 = a3 - 32;
  v5 = a2 >> 1;
  v6 = a1;
  v14 = a2 - (a2 >> 1);
  m128i_i8 = a1[-2 * (a2 >> 1)].m128i_i8;
  for ( i = m128i_i8 - 32; ; i -= 32 )
  {
    while ( 1 )
    {
      v6 -= 2;
      LODWORD(v9) = sub_9C70(v4, i);
      if ( (int)v9 > 0 )
        break;
      v11 = _mm_loadu_si128((const __m128i *)(v3 - 16));
      *v6 = _mm_loadu_si128((const __m128i *)(v3 - 32));
      v6[1] = v11;
      if ( !--v5 )
        return v9;
      v3 = v4;
      v4 -= 32;
    }
    v10 = v14-- == 1;
    *v6 = _mm_loadu_si128((const __m128i *)m128i_i8 - 2);
    v6[1] = _mm_loadu_si128((const __m128i *)m128i_i8 - 1);
    if ( v10 )
      break;
    m128i_i8 = i;
  }
  v9 = -32;
  do
  {
    v12 = _mm_loadu_si128((const __m128i *)(v3 + v9 + 16));
    v6[(unsigned long long)v9 / 0x10] = _mm_loadu_si128((const __m128i *)(v3 + v9));
    v6[(unsigned long long)v9 / 0x10 + 1] = v12;
    v9 -= 32;
    --v5;
  }
  while ( v5 );
  return v9;
}



// Function: sequential_sort @ 0x9e20
int sequential_sort(long long a1, unsigned long long a2, __m128i *a3, char a4)
{
  __m128i *v4; // r13
  long long v5; // r12
  unsigned long long v8; // r14
  unsigned long long v9; // rsi
  unsigned long long v10; // rdi
  __m128i v11; // xmm3
  __m128i *v12; // rax
  const __m128i *v13; // rax
  __m128i v14; // xmm1
  __m128i v15; // xmm0
  __m128i v16; // xmm4
  __m128i v17; // xmm5
  int v18; // edx
  const __m128i *v19; // rdx
  __m128i v20; // xmm7
  __m128i v21; // xmm4
  __m128i v22; // xmm5

  v4 = a3;
  v5 = a1;
  if ( a2 == 2 )
  {
    LODWORD(v13) = sub_9C70(a1 - 32, (char *)(a1 - 64));
    if ( a4 )
    {
      v18 = (int)v13 > 0;
      v13 = (const __m128i *)(a1 + 32LL * (v18 - 2));
      v19 = (const __m128i *)(a1 + 32LL * ~v18);
      v20 = _mm_loadu_si128(v13 + 1);
      v21 = _mm_loadu_si128(v19);
      v22 = _mm_loadu_si128(v19 + 1);
      v4[-4] = _mm_loadu_si128(v13);
      v4[-2] = v21;
      v4[-1] = v22;
      v4[-3] = v20;
    }
    else if ( (int)v13 > 0 )
    {
      v14 = _mm_loadu_si128((const __m128i *)(a1 - 32));
      v15 = _mm_loadu_si128((const __m128i *)(a1 - 16));
      v16 = _mm_loadu_si128((const __m128i *)(a1 - 64));
      v17 = _mm_loadu_si128((const __m128i *)(a1 - 48));
      v4[-2] = v14;
      v4[-1] = v15;
      *(__m128i *)(a1 - 32) = v16;
      *(__m128i *)(a1 - 16) = v17;
      *(__m128i *)(a1 - 64) = v14;
      *(__m128i *)(a1 - 48) = v15;
    }
  }
  else
  {
    v8 = a2 >> 1;
    v9 = a2 - (a2 >> 1);
    v10 = a1 - 32 * v8;
    if ( a4 )
    {
      sequential_sort(v10, v9, &a3[-2 * v8], 1);
      if ( a2 > 3 )
        sequential_sort(v5, v8, v4, 0);
    }
    else
    {
      sequential_sort(v10, v9, a3, 0);
      if ( a2 > 3 )
      {
        sequential_sort(v5, v8, v4, 1);
        v12 = v4;
      }
      else
      {
        v11 = _mm_loadu_si128((const __m128i *)(v5 - 16));
        v12 = v4;
        v4[-2] = _mm_loadu_si128((const __m128i *)(v5 - 32));
        v4[-1] = v11;
      }
      v4 = (__m128i *)v5;
      v5 = (long long)v12;
    }
    LODWORD(v13) = sub_9D40(v4, a2, v5);
  }
  return (int)v13;
}



// Function: write_unique @ 0x9fb0
int write_unique(long long a1, FILE *a2, char *a3)
{
  int result; // eax
  char *v4; // [rsp+8h] [rbp-10h]

  if ( !byte_1E419 )
    return sub_9460((const void **)a1, a2, a3);
  if ( !(long long)xmmword_1E840 || (v4 = a3, result = sub_9C70(a1, (char *)&xmmword_1E840), a3 = v4, result) )
  {
    xmmword_1E840 = (__int128)_mm_loadu_si128((const __m128i *)a1);
    xmmword_1E850 = (__int128)_mm_loadu_si128((const __m128i *)(a1 + 16));
    return sub_9460((const void **)a1, a2, a3);
  }
  return result;
}



// Function: mergelines_node @ 0xa020
long long mergelines_node(long long a1, unsigned long long a2, FILE *a3, char *a4)
{
  unsigned int v7; // eax
  long long v8; // rbp
  long long v9; // rdx
  long long v10; // r13
  const __m128i *v11; // rcx
  long long v12; // rsi
  __m128i **v13; // r10
  const __m128i *v14; // rdx
  long long result; // rax
  long long v16; // r11
  long long v17; // r8
  __m128i *v18; // r14
  long long v19; // rax
  __m128i v20; // xmm0
  __m128i v21; // xmm1
  long long v22; // rdx
  __m128i v23; // xmm2
  __m128i v24; // xmm3
  long long v25; // rdi
  long long v26; // r9
  long long v27; // rax
  long long v28; // rdi
  long long v29; // rdi
  long long v30; // rcx
  long long v31; // r15
  bool v32; // cf
  long long v33; // r8
  long long v34; // r8
  const __m128i *v35; // rdx
  __m128i v36; // xmm4
  long long v37; // r8
  const __m128i *v38; // r8
  long long v39; // rcx
  const __m128i *v40; // rdx
  __m128i v41; // xmm6
  const __m128i *v42; // rsi
  long long v43; // r15
  long long v44; // [rsp+8h] [rbp-40h]

  v7 = *(int *)(a1 + 80);
  v8 = *(long long *)a1;
  v9 = *(long long *)(a1 + 8);
  v10 = *(long long *)a1;
  v44 = v9;
  v11 = *(const __m128i **)(a1 + 16);
  v12 = (a2 >> (2 * (unsigned char)v7 + 2)) + 1;
  if ( v7 > 1 )
  {
    v13 = *(__m128i ***)(a1 + 32);
    v14 = *(const __m128i **)(a1 + 8);
    result = *(long long *)a1;
    v16 = 0;
    v17 = 0;
    v18 = *v13;
    if ( (const __m128i *)v8 == v11 )
    {
LABEL_11:
      v25 = *(long long *)(a1 + 48);
      v26 = *(long long *)(a1 + 40);
      if ( v16 != v25 )
      {
LABEL_12:
        if ( v17 == v26 )
        {
          v38 = *(const __m128i **)(a1 + 24);
          if ( v38 != v14 )
          {
            v39 = v12 - 1;
            if ( v12 )
            {
              v40 = v14 - 2;
              do
              {
                v41 = _mm_loadu_si128(v40);
                v18 -= 2;
                *(long long *)(a1 + 8) = v40;
                v42 = v40;
                *v18 = v41;
                v18[1] = _mm_loadu_si128(v40 + 1);
                if ( v40 == v38 )
                {
                  v16 = (v44 - (long long)v40) >> 5;
                  goto LABEL_13;
                }
                --v39;
                v40 -= 2;
              }
              while ( v39 != -1 );
              v16 = (v44 - (long long)v42) >> 5;
            }
          }
        }
      }
    }
    else
    {
      while ( 1 )
      {
        if ( *(const __m128i **)(a1 + 24) == v14 )
        {
          v13 = *(__m128i ***)(a1 + 32);
          v33 = v10 - result;
          result = *(long long *)a1;
          v17 = v33 >> 5;
          v16 = (v44 - (long long)v14) >> 5;
          goto LABEL_38;
        }
        if ( !v12 )
          break;
        v18 -= 2;
        if ( sub_9C70(result - 32, v14[-2].m128i_i8) > 0 )
        {
          v19 = *(long long *)(a1 + 8);
          v11 = *(const __m128i **)(a1 + 16);
          v20 = _mm_loadu_si128((const __m128i *)(v19 - 32));
          v14 = (const __m128i *)(v19 - 32);
          *(long long *)(a1 + 8) = v19 - 32;
          *v18 = v20;
          v21 = _mm_loadu_si128((const __m128i *)(v19 - 16));
          result = *(long long *)a1;
          v18[1] = v21;
          if ( v11 == (const __m128i *)result )
            goto LABEL_10;
        }
        else
        {
          v22 = *(long long *)a1;
          v11 = *(const __m128i **)(a1 + 16);
          v23 = _mm_loadu_si128((const __m128i *)(*(long long *)a1 - 32LL));
          result = *(long long *)a1 - 32LL;
          *(long long *)a1 = result;
          *v18 = v23;
          v24 = _mm_loadu_si128((const __m128i *)(v22 - 16));
          v14 = *(const __m128i **)(a1 + 8);
          v18[1] = v24;
          if ( v11 == (const __m128i *)result )
          {
LABEL_10:
            v13 = *(__m128i ***)(a1 + 32);
            --v12;
            v17 = (v10 - result) >> 5;
            v16 = (v44 - (long long)v14) >> 5;
            goto LABEL_11;
          }
        }
        --v12;
      }
      v13 = *(__m128i ***)(a1 + 32);
      v12 = -1;
      v37 = v10 - result;
      result = *(long long *)a1;
      v17 = v37 >> 5;
      v16 = (v44 - (long long)v14) >> 5;
LABEL_38:
      v25 = *(long long *)(a1 + 48);
      v26 = *(long long *)(a1 + 40);
      if ( v25 != v16 )
        goto LABEL_12;
      if ( v11 != (const __m128i *)result )
      {
        v34 = v12 - 1;
        if ( v12 )
        {
          v35 = (const __m128i *)(result - 32);
          do
          {
            v36 = _mm_loadu_si128(v35);
            v18 -= 2;
            *(long long *)a1 = v35;
            result = (long long)v35;
            *v18 = v36;
            v18[1] = _mm_loadu_si128(v35 + 1);
            if ( v35 == v11 )
              break;
            --v34;
            v35 -= 2;
          }
          while ( v34 != -1 );
        }
      }
    }
LABEL_13:
    *v13 = v18;
    goto LABEL_14;
  }
  v25 = 0;
  v27 = 0;
  if ( (const __m128i *)v8 == v11 )
  {
LABEL_24:
    v26 = *(long long *)(a1 + 40);
    v30 = v26;
    if ( *(long long *)(a1 + 48) == v25 )
    {
      result = v10;
      v16 = v25;
      goto LABEL_14;
    }
    goto LABEL_29;
  }
  while ( *(long long *)(a1 + 24) != v9 )
  {
    if ( !v12 )
    {
      v12 = -1;
      break;
    }
    if ( sub_9C70(v10 - 32, (char *)(v9 - 32)) > 0 )
    {
      v28 = *(long long *)(a1 + 8) - 32LL;
      *(long long *)(a1 + 8) = v28;
      sub_9FB0(v28, a3, a4);
      v10 = *(long long *)a1;
      v9 = *(long long *)(a1 + 8);
      if ( *(long long *)a1 == *(long long *)(a1 + 16) )
        goto LABEL_23;
    }
    else
    {
      v29 = *(long long *)a1 - 32LL;
      *(long long *)a1 = v29;
      sub_9FB0(v29, a3, a4);
      v10 = *(long long *)a1;
      v9 = *(long long *)(a1 + 8);
      if ( *(long long *)a1 == *(long long *)(a1 + 16) )
      {
LABEL_23:
        --v12;
        v27 = (v8 - v10) >> 5;
        v25 = (v44 - v9) >> 5;
        goto LABEL_24;
      }
    }
    --v12;
  }
  v25 = (v44 - v9) >> 5;
  if ( *(long long *)(a1 + 48) != v25 )
  {
    v30 = *(long long *)(a1 + 40);
    v27 = (v8 - v10) >> 5;
LABEL_29:
    v26 = v30;
    if ( v30 == v27 && v9 != *(long long *)(a1 + 24) && (v43 = v12 - 1, v12) )
    {
      do
      {
        *(long long *)(a1 + 8) = v9 - 32;
        sub_9FB0(v9 - 32, a3, a4);
        v9 = *(long long *)(a1 + 8);
        if ( v9 == *(long long *)(a1 + 24) )
          break;
        v32 = v43-- == 0;
      }
      while ( !v32 );
      result = *(long long *)a1;
      v26 = *(long long *)(a1 + 40);
      v25 = *(long long *)(a1 + 48);
      v16 = (v44 - v9) >> 5;
    }
    else
    {
      result = v10;
      v25 = *(long long *)(a1 + 48);
      v16 = (v44 - *(long long *)(a1 + 8)) >> 5;
    }
    goto LABEL_14;
  }
  result = v10;
  if ( *(long long *)(a1 + 16) != v10 && (v31 = v12 - 1, v12) )
  {
    do
    {
      *(long long *)a1 = result - 32;
      sub_9FB0(result - 32, a3, a4);
      result = *(long long *)a1;
      if ( *(long long *)a1 == *(long long *)(a1 + 16) )
        break;
      v32 = v31-- == 0;
    }
    while ( !v32 );
    v26 = *(long long *)(a1 + 40);
    v25 = *(long long *)(a1 + 48);
    v16 = (v44 - *(long long *)(a1 + 8)) >> 5;
  }
  else
  {
    v26 = *(long long *)(a1 + 40);
    v16 = (v44 - v9) >> 5;
  }
LABEL_14:
  *(long long *)(a1 + 48) = v25 - v16;
  *(long long *)(a1 + 40) = v26 - ((v8 - result) >> 5);
  return result;
}



// Function: merge_loop @ 0xa460
int merge_loop(long long a1, unsigned long long a2, FILE *a3, char *a4)
{
  long long v6; // rbp

  while ( 1 )
  {
    v6 = sub_7C70(a1);
    if ( !*(int *)(v6 + 80) )
      break;
    sub_A020(v6, a2, a3, a4);
    sub_7BC0(a1, v6);
    sub_7C10(a1, v6);
    sub_7C00(v6);
  }
  sub_7C00(v6);
  return sub_7B80(a1, v6);
}



// Function: sortlines @ 0xa4e0
unsigned long long sortlines(
        long long a1,
        unsigned long long a2,
        unsigned long long a3,
        long long *a4,
        long long a5,
        FILE *a6,
        char *a7)
{
  unsigned long long v10; // r9
  unsigned long long v11; // r12
  long long v12; // rax
  __m128i *v13; // r11
  long long v14; // r10
  long long v15; // rax
  int v17; // eax
  long long v18; // [rsp+8h] [rbp-B0h]
  long long v19; // [rsp+8h] [rbp-B0h]
  long long v20; // [rsp+8h] [rbp-B0h]
  long long v21; // [rsp+10h] [rbp-A8h]
  unsigned long long v22; // [rsp+18h] [rbp-A0h]
  long long v23; // [rsp+18h] [rbp-A0h]
  long long v24; // [rsp+18h] [rbp-A0h]
  long long v25; // [rsp+20h] [rbp-98h]
  long long v26; // [rsp+28h] [rbp-90h]
  pthread_t newthread; // [rsp+38h] [rbp-80h] BYREF
  long long arg[7]; // [rsp+40h] [rbp-78h] BYREF
  unsigned long long v29; // [rsp+78h] [rbp-40h]

  v10 = a4[5];
  v11 = a4[6];
  v29 = __readfsqword(0x28u);
  v12 = a4[8];
  arg[0] = a1;
  arg[1] = a2 >> 1;
  arg[3] = v12;
  arg[2] = a3;
  arg[4] = a5;
  arg[5] = a6;
  arg[6] = a7;
  if ( v10 + v11 > 0x1FFFF && a2 > 1 )
  {
    v24 = a5;
    v17 = pthread_create(&newthread, 0, start_routine, arg);
    a5 = v24;
    if ( !v17 )
    {
      sortlines(a1 - 32 * *((int *)a4 + 10), a2 - (a2 >> 1), a3, a4[9], v24, (int)a6, (long long)a7);
      pthread_join(newthread, 0);
      return v29 - __readfsqword(0x28u);
    }
    v10 = a4[5];
    v11 = a4[6];
  }
  v13 = (__m128i *)(a1 - 32 * a3);
  v14 = a1 - 32 * v10;
  v15 = -32LL * v10;
  if ( v11 > 1 )
  {
    v26 = a5;
    v25 = -32LL * v10;
    v22 = v10;
    v18 = a1 - 32 * v10;
    sub_9E20(v18, v11, &v13[-2 * (v10 >> 1)], 0);
    a5 = v26;
    v15 = v25;
    v10 = v22;
    v13 = (__m128i *)(a1 - 32 * a3);
    v14 = v18;
  }
  if ( v10 > 1 )
  {
    v23 = a5;
    v21 = v14;
    v19 = v15;
    sub_9E20(a1, v10, v13, 0);
    a5 = v23;
    v14 = v21;
    v15 = v19;
  }
  *a4 = a1;
  a4[1] = v14;
  a4[2] = v14;
  a4[3] = v15 - 32 * v11 + a1;
  v20 = a5;
  sub_7B80(a5, (long long)a4);
  sub_A460(v20, a3, a6, a7);
  return v29 - __readfsqword(0x28u);
}



// Function: sortlines_thread @ 0xa710
void *sortlines_thread(void *a1)
{
  sub_A4E0(
    *(long long *)a1,
    *((long long *)a1 + 1),
    *((long long *)a1 + 2),
    *((long long **)a1 + 3),
    *((long long *)a1 + 4),
    *((FILE **)a1 + 5),
    *((char **)a1 + 6));
  return 0;
}



// Function: fillbuf @ 0xa750
long long fillbuf(char **a1, FILE *a2, char *a3)
{
  long long v3; // rbp
  long long v4; // r12
  char *v5; // r9
  size_t v6; // rdx
  char *v7; // rdx
  unsigned long long v8; // r12
  long long v9; // rsi
  char *v10; // r13
  char *v11; // r15
  char *v12; // r14
  long long v13; // rax
  long long v14; // rdx
  char **v15; // rbx
  size_t v16; // r13
  size_t v17; // rax
  char *v18; // r11
  char **v19; // r15
  unsigned long long v20; // r13
  char *v21; // rax
  long long v22; // rdx
  unsigned long long v23; // rsi
  bool v24; // zf
  unsigned char v26; // al
  long long v27; // rcx
  char *v28; // r13
  char *v29; // rdi
  char *v30; // rdx
  long long v31; // rax
  unsigned long long v32; // rdx
  long long v33; // rcx
  char *v34; // rax
  char *v35; // [rsp+0h] [rbp-88h]
  char *v36; // [rsp+8h] [rbp-80h]
  unsigned long long v37; // [rsp+10h] [rbp-78h]
  unsigned long long v38; // [rsp+10h] [rbp-78h]
  signed long long v39; // [rsp+10h] [rbp-78h]
  unsigned long long v40; // [rsp+18h] [rbp-70h]
  char v43; // [rsp+37h] [rbp-51h]
  long long v45[9]; // [rsp+40h] [rbp-48h] BYREF

  v3 = qword_1E410;
  v4 = qword_1E030;
  v45[1] = __readfsqword(0x28u);
  v43 = byte_1E038;
  v36 = a1[5];
  if ( *((char *)a1 + 48) )
    return 0;
  v5 = a1[1];
  v6 = (size_t)a1[4];
  if ( v5 == (char *)v6 )
  {
    v7 = a1[2];
  }
  else
  {
    memmove(*a1, &v5[(long long)*a1 - v6], v6);
    v5 = a1[4];
    a1[2] = 0;
    v7 = 0;
    a1[1] = v5;
  }
  v8 = v4 - 34;
  v9 = (long long)a1[3];
  v10 = *a1;
  v40 = (unsigned long long)(v36 + 1);
  while ( 1 )
  {
    v11 = &v5[(long long)v10];
    v12 = v10;
    v13 = sub_A740(v10, v9, v7);
    v15 = (char **)(v13 - 32 * v14);
    v37 = v13 - v14 * (long long)v36 - (long long)v11;
    if ( v14 )
      v12 = &v15[1][(long long)*v15];
    if ( v37 <= v40 )
    {
LABEL_34:
      v28 = v12;
    }
    else
    {
      while ( 1 )
      {
        v16 = (v37 - 1) / v40;
        v17 = fread_unlocked(v11, 1u, v16, a2);
        v38 = v37 - v17;
        v35 = &v11[v17];
        if ( v16 != v17 )
        {
          if ( ferror_unlocked(a2) )
          {
            v34 = dcgettext(0, "read failed", 5);
            sub_5E10(v34, a3);
          }
          if ( feof_unlocked(a2) )
          {
            *((char *)a1 + 48) = 1;
            if ( *a1 == v35 )
              return 0;
            if ( v12 != v35 && *(v35 - 1) != v43 )
              *v35++ = v43;
          }
        }
        v18 = v11;
        v19 = v15;
        v20 = v38;
        while ( 1 )
        {
          v21 = (char *)memchr(v18, v43, v35 - v18);
          if ( !v21 )
            break;
          v18 = v21 + 1;
          v19 -= 4;
          *v21 = 0;
          *v19 = v12;
          v23 = v21 + 1 - v12;
          v19[1] = (char *)v23;
          if ( v8 < v23 )
            v8 = v21 + 1 - v12;
          v20 -= (unsigned long long)v36;
          if ( v3 )
          {
            if ( *(long long *)(v3 + 16) != -1 )
            {
              v39 = v21 + 1 - v12;
              v21 = (char *)sub_8F70(v12, v23, v3);
              v23 = v39;
            }
            v24 = *(long long *)v3 == -1;
            v19[3] = v21;
            if ( v24 )
            {
              if ( *(char *)(v3 + 48) )
              {
                while ( 1 )
                {
                  v26 = sub_53E0((unsigned int)*v12, v23, v22);
                  if ( !*(char *)(v27 + v26) )
                    break;
                  ++v12;
                }
              }
              v19[2] = v12;
            }
            else
            {
              v19[2] = (char *)sub_90D0(v12, v23, v3);
            }
          }
          v12 = v18;
        }
        v37 = v20;
        v15 = v19;
        if ( *((char *)a1 + 48) )
          break;
        v11 = v35;
        if ( v20 <= v40 )
          goto LABEL_34;
      }
      v11 = v35;
      v28 = v12;
    }
    v29 = *a1;
    v30 = a1[3];
    a1[1] = (char *)(v11 - *a1);
    v31 = (sub_A740(v29, v30, v30) - (long long)v15) >> 5;
    *(long long *)(v33 + 16) = v31;
    if ( v31 )
      break;
    v45[0] = v32 >> 5;
    v10 = (char *)sub_12FF0(v29, v45, 32);
    v9 = 32LL * v45[0];
    v5 = a1[1];
    v7 = a1[2];
    *a1 = v10;
    a1[3] = (char *)v9;
  }
  qword_1E030 = v8 + 34;
  *(long long *)(v33 + 32) = v11 - v28;
  return 1;
}



// Function: check @ 0xaac0
long long check(char *a1, char a2)
{
  FILE *v3; // rax
  unsigned long long v4; // rdx
  int v5; // ebx
  size_t v6; // r12
  char v7; // al
  long long v8; // rdx
  unsigned long long v9; // rbp
  unsigned long long v10; // r14
  long long v11; // r15
  size_t v12; // rdx
  char *v13; // rdi
  char *v14; // rax
  char *v15; // rdi
  void *v16; // rax
  unsigned int v17; // r12d
  const void **v19; // r15
  long long v20; // rax
  long long v21; // r14
  long long v22; // rbx
  char *v23; // rax
  char *v24; // rax
  long long v25; // [rsp+8h] [rbp-100h]
  FILE *stream; // [rsp+10h] [rbp-F8h]
  char *ptr; // [rsp+18h] [rbp-F0h]
  long long v28; // [rsp+28h] [rbp-E0h]
  long long v29; // [rsp+30h] [rbp-D8h]
  long long v30; // [rsp+38h] [rbp-D0h]
  void *dest[4]; // [rsp+50h] [rbp-B8h] BYREF
  char *v32[2]; // [rsp+70h] [rbp-98h] BYREF
  long long v33; // [rsp+80h] [rbp-88h]
  long long v34; // [rsp+88h] [rbp-80h]
  char v35[24]; // [rsp+B0h] [rbp-58h] BYREF
  unsigned long long v36; // [rsp+C8h] [rbp-40h]

  v36 = __readfsqword(0x28u);
  v3 = sub_6010(a1, "r");
  v4 = qword_1E438;
  stream = v3;
  v30 = qword_1E410;
  v5 = (unsigned char)byte_1E419 ^ 1;
  if ( qword_1E438 < (unsigned long long)qword_1E030 )
    v4 = qword_1E030;
  v6 = 0;
  sub_6240((long long)v32, 32, v4);
  dest[0] = 0;
  v29 = 0;
  while ( 1 )
  {
    v7 = sub_A750(v32, stream, a1);
    ptr = v32[0];
    if ( !v7 )
    {
      v17 = 1;
      goto LABEL_20;
    }
    v25 = sub_A740(v32[0], v34, v8);
    v28 = v33;
    v9 = v25 - 32 * v33;
    if ( v6 )
    {
      if ( v5 <= sub_9C70((long long)dest, (char *)(v25 - 32)) )
        break;
    }
    v10 = v25 - 32;
    while ( 1 )
    {
      v11 = v10;
      if ( v9 >= v10 )
        break;
      v10 -= 32LL;
      if ( v5 <= sub_9C70(v11, (char *)v10) )
        goto LABEL_18;
    }
    v12 = *(long long *)(v10 + 8);
    v29 += v28;
    if ( v12 > v6 )
    {
      while ( 1 )
      {
        v6 *= 2LL;
        if ( !v6 )
          break;
        if ( v12 <= v6 )
          goto LABEL_15;
      }
      v6 = *(long long *)(v10 + 8);
LABEL_15:
      free(dest[0]);
      v16 = (void *)sub_12EB0(v6);
      v12 = *(long long *)(v10 + 8);
      dest[0] = v16;
    }
    v13 = (char *)memcpy(dest[0], *(const void **)v10, v12);
    dest[1] = *(void **)(v10 + 8);
    if ( v30 )
    {
      v14 = &v13[*(long long *)(v10 + 16)];
      v15 = &v13[*(long long *)(v10 + 24) - *(long long *)v10];
      dest[2] = &v14[-*(long long *)v10];
      dest[3] = v15;
    }
  }
  v11 = v25;
LABEL_18:
  v17 = 0;
  if ( a2 == 99 )
  {
    v19 = (const void **)(v11 - 32);
    v20 = sub_E520(v29 + ((v25 - (long long)v19) >> 5), v35);
    v21 = qword_1E888;
    v22 = v20;
    v23 = dcgettext(0, "%s: %s:%s: disorder: ", 5);
    __fprintf_chk(stderr, 1, v23, v21, a1, v22);
    v24 = dcgettext(0, "standard error", 5);
    sub_9460(v19, stderr, v24);
  }
LABEL_20:
  sub_6040(stream, a1);
  free(ptr);
  free(dest[0]);
  return v17;
}



// Function: mergefps @ 0xad90
void mergefps(long long a1, unsigned long long a2, unsigned long long a3, FILE *a4, char *a5, FILE **a6)
{
  long long *v8; // r12
  unsigned long long *v9; // rbp
  unsigned long long v10; // rbx
  long long v11; // rbp
  char **v12; // r12
  long long v13; // rdx
  char **v14; // r13
  long long v15; // rdx
  long long v16; // rax
  unsigned long long v17; // rax
  unsigned long long v18; // r14
  unsigned long long *v19; // rbx
  unsigned long long *v20; // r13
  unsigned long long v21; // rax
  unsigned long long v22; // r13
  unsigned long long v23; // rcx
  unsigned long long v24; // rbx
  long long v25; // rax
  long long v26; // rdx
  long long v27; // rdx
  unsigned long long v28; // r13
  unsigned long long v29; // rax
  unsigned long long v30; // r14
  unsigned long long v31; // rax
  unsigned long long v32; // r13
  unsigned long long v33; // r15
  unsigned long long *v34; // rbx
  int v35; // eax
  unsigned long long *v36; // rax
  unsigned long long v37; // rdx
  void *v38; // r14
  void *v39; // rdi
  const void *v40; // rsi
  char *v41; // rdi
  unsigned long long v42; // rcx
  unsigned long long *v43; // rax
  unsigned long long v44; // rax
  unsigned long long v45; // rdx
  long long v46; // rax
  const __m128i *v47; // rdx
  __m128i v48; // xmm0
  __m128i v49; // xmm1
  __m128i v50; // xmm2
  long long v51; // r9
  __m128i v52; // xmm3
  long long v53; // r9
  long long v54; // r9
  unsigned long long *v55; // rax
  unsigned long long v56; // rdx
  FILE *v57; // rax
  unsigned long long v58; // rax
  void *v59; // rax
  long long *v60; // [rsp+0h] [rbp-D8h]
  long long *v61; // [rsp+8h] [rbp-D0h]
  unsigned long long v62; // [rsp+8h] [rbp-D0h]
  unsigned long long v63; // [rsp+10h] [rbp-C8h]
  long long *v64; // [rsp+18h] [rbp-C0h]
  char **v65; // [rsp+20h] [rbp-B8h]
  unsigned long long *v66; // [rsp+28h] [rbp-B0h]
  void **v67; // [rsp+28h] [rbp-B0h]
  void *v73; // [rsp+58h] [rbp-80h]
  long long v74; // [rsp+68h] [rbp-70h]
  void *dest[13]; // [rsp+70h] [rbp-68h] BYREF

  v63 = a3;
  dest[5] = (void *)__readfsqword(0x28u);
  v65 = (char **)sub_12FD0(a3);
  v8 = (long long *)sub_12FD0(a3);
  v64 = (long long *)sub_12FD0(a3);
  dest[0] = 0;
  v9 = (unsigned long long *)sub_12FD0(a3);
  v74 = qword_1E410;
  if ( !a3 )
    goto LABEL_60;
  v61 = v8;
  v10 = 0;
  v66 = v9;
  while ( 1 )
  {
    while ( 1 )
    {
      v11 = 8 * v10;
      v12 = &v65[7 * v10];
      v13 = qword_1E030;
      if ( qword_1E438 / v63 >= qword_1E030 )
        v13 = qword_1E438 / v63;
      v14 = (char **)(a1 + 16 * v10);
      sub_6240((long long)&v65[7 * v10], 32, v13);
      if ( !(unsigned char)sub_A750(v12, a6[v10], *v14) )
        break;
      v16 = sub_A740(*v12, v12[3], v15);
      v61[v10] = v16 - 32;
      v64[v10++] = v16 - 32LL * (long long)v12[2];
      if ( v10 >= v63 )
      {
        v8 = v61;
        v9 = v66;
        goto LABEL_8;
      }
    }
    sub_6040(a6[v10], *v14);
    if ( a2 > v10 )
    {
      --a2;
      sub_6E80(*v14);
    }
    free(*v12);
    if ( v63 - 1 <= v10 )
      break;
    do
    {
      v57 = a6[(unsigned long long)v11 / 8 + 1];
      *(__m128i *)(a1 + 2 * v11) = _mm_loadu_si128((const __m128i *)(a1 + 2 * v11 + 16));
      a6[(unsigned long long)v11 / 8] = v57;
      v11 += 8;
    }
    while ( 8 * v63 - 8 != v11 );
    --v63;
  }
  v8 = v61;
  v9 = v66;
  if ( v63 == 1 )
    goto LABEL_60;
  --v63;
LABEL_8:
  v17 = 0;
  do
  {
    v9[v17] = v17;
    ++v17;
  }
  while ( v17 != v63 );
  if ( v63 != 1 )
  {
    v18 = 1;
    do
    {
      while ( 1 )
      {
        v19 = &v9[v18];
        v20 = v19 - 1;
        if ( sub_9C70(v8[*(v19 - 1)], (char *)v8[*v19]) <= 0 )
          break;
        v21 = *v20;
        v18 = 1;
        *v20 = *v19;
        *v19 = v21;
      }
      ++v18;
    }
    while ( v18 < v63 );
  }
  v73 = 0;
  v67 = 0;
  while ( 1 )
  {
    while ( 1 )
    {
      while ( 1 )
      {
        v22 = v8[*v9];
        if ( byte_1E419 )
        {
          if ( v67 )
          {
            if ( !sub_9C70((long long)v67, (char *)v8[*v9]) )
              goto LABEL_19;
            sub_9460((const void **)dest, a4, a5);
          }
          v38 = *(void **)(v22 + 8);
          v39 = dest[0];
          if ( v38 > v73 )
          {
            v58 = (unsigned long long)v73;
            while ( v58 )
            {
              v58 *= 2LL;
              if ( (unsigned long long)v38 <= v58 )
              {
                v38 = (void *)v58;
                break;
              }
            }
            free(dest[0]);
            v59 = (void *)sub_12EB0(v38);
            v73 = v38;
            v38 = *(void **)(v22 + 8);
            dest[0] = v59;
            v39 = v59;
          }
          v40 = *(const void **)v22;
          dest[1] = v38;
          v41 = (char *)memcpy(v39, v40, (size_t)v38);
          v67 = dest;
          if ( v74 )
          {
            dest[2] = &v41[*(long long *)(v22 + 16) - *(long long *)v22];
            dest[3] = &v41[*(long long *)(v22 + 24) - *(long long *)v22];
          }
        }
        else
        {
          sub_9460((const void **)v22, a4, a5);
        }
LABEL_19:
        v23 = *v9;
        v62 = *v9;
        if ( v64[*v9] >= v22 )
          break;
        v29 = *v9;
        v8[v23] = v22 - 32;
        v28 = v63;
        if ( v63 != 1 )
          goto LABEL_22;
LABEL_33:
        *v9 = v29;
      }
      if ( !(unsigned char)sub_A750(&v65[8 * v23 - v62], a6[v62], *(char **)(a1 + 16 * v62)) )
        break;
      v24 = *v9;
      v62 = *v9;
      v25 = sub_A740(v65[7 * *v9], v65[7 * *v9 + 3], &v65[7 * *v9]);
      v27 = *(long long *)(v26 + 16);
      v28 = v63;
      v8[v24] = v25 - 32;
      v64[v24] = v25 - 32 * v27;
      v29 = v62;
      if ( v63 == 1 )
        goto LABEL_33;
LABEL_22:
      v30 = 1;
      v60 = &v8[v29];
      v31 = v28;
      v32 = 1;
      v33 = v31;
      do
      {
        while ( 1 )
        {
          v34 = &v9[v30];
          v35 = sub_9C70(*v60, (char *)v8[*v34]);
          if ( v35 < 0 || !v35 && *v34 > v62 )
            break;
          v32 = v30 + 1;
          v30 = (v30 + 1 + v33) >> 1;
          if ( v32 >= v33 )
            goto LABEL_28;
        }
        v33 = v30;
        v30 = (v32 + v30) >> 1;
      }
      while ( v32 < v33 );
LABEL_28:
      if ( v32 != 1 )
      {
        v36 = v9;
        do
        {
          v37 = v36[1];
          *v36++ = v37;
        }
        while ( v36 != &v9[v32 - 1] );
      }
      v9[v32 - 1] = v62;
    }
    v42 = *v9;
    v43 = v9 + 1;
    if ( v63 != 1 )
    {
      do
      {
        if ( *v43 > v42 )
        {
          --*v43;
          v42 = *v9;
        }
        ++v43;
      }
      while ( &v9[v63] != v43 );
    }
    sub_6040(a6[v42], *(char **)(a1 + 16 * v42));
    v44 = *v9;
    if ( *v9 < a2 )
    {
      --a2;
      sub_6E80(*(char **)(a1 + 16 * v44));
      v44 = *v9;
    }
    free(v65[7 * v44]);
    v45 = *v9;
    if ( v63 - 1 > *v9 )
    {
      v46 = 8 * v45;
      v47 = (const __m128i *)&v65[7 * v45];
      do
      {
        v48 = _mm_loadu_si128((const __m128i *)(a1 + 2 * v46 + 16));
        v47 = (const __m128i *)((char *)v47 + 56);
        v49 = _mm_loadu_si128(v47);
        v50 = _mm_loadu_si128(v47 + 1);
        a6[(unsigned long long)v46 / 8] = a6[(unsigned long long)v46 / 8 + 1];
        v51 = v47[3].m128i_i64[0];
        v52 = _mm_loadu_si128(v47 + 2);
        *(__m128i *)(a1 + 2 * v46) = v48;
        v47[-1].m128i_i64[1] = v51;
        v53 = v8[(unsigned long long)v46 / 8 + 1];
        *(__m128i *)((char *)v47 - 56) = v49;
        v8[(unsigned long long)v46 / 8] = v53;
        v54 = v64[(unsigned long long)v46 / 8 + 1];
        *(__m128i *)((char *)v47 - 40) = v50;
        v64[(unsigned long long)v46 / 8] = v54;
        v46 += 8;
        *(__m128i *)((char *)v47 - 24) = v52;
      }
      while ( 8 * v63 - 8 != v46 );
    }
    if ( v63 == 1 )
      break;
    v55 = v9;
    do
    {
      v56 = v55[1];
      *v55++ = v56;
    }
    while ( &v9[v63 - 1] != v55 );
    --v63;
  }
  if ( v67 && byte_1E419 )
  {
    sub_9460((const void **)dest, a4, a5);
    free(dest[0]);
  }
LABEL_60:
  sub_6040(a4, a5);
  free(a6);
  free(v65);
  free(v9);
  free(v64);
  free(v8);
}



// Function: mergefiles @ 0xb500
unsigned long long mergefiles(long long a1, unsigned long long a2, unsigned long long a3, FILE *a4, char *a5)
{
  unsigned long long v8; // rax
  unsigned long long v9; // r12
  char *v11; // rbp
  char *v12; // rax
  FILE **v13[9]; // [rsp+0h] [rbp-48h] BYREF

  v13[1] = (FILE **)__readfsqword(0x28u);
  v8 = sub_7470(a1, a3, (long long *)v13);
  v9 = v8;
  if ( a3 > v8 && v8 <= 1 )
  {
    v11 = *(char **)(a1 + 16 * v8);
    v12 = dcgettext(0, "open failed", 5);
    sub_5E10(v12, v11);
  }
  sub_AD90(a1, a2, v8, a4, a5, v13[0]);
  return v9;
}



// Function: avoid_trashing_input @ 0xb5b0
unsigned long long avoid_trashing_input(long long a1, unsigned long long a2, unsigned long long a3, const char *a4)
{
  const char *v5; // r12
  const char **v6; // rbx
  struct stat *v7; // r15
  const char *v8; // r15
  int v9; // ebp
  const char *v10; // rbp
  long long v12; // rax
  const char **v13; // [rsp+0h] [rbp-F8h]
  FILE *v14; // [rsp+18h] [rbp-E0h] BYREF
  struct stat v15; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v16; // [rsp+B8h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  if ( a2 < a3 )
  {
    v5 = 0;
    v6 = (const char **)(a1 + 16 * a2);
    v13 = (const char **)(a1 + 16 * a3);
    do
    {
      while ( 1 )
      {
        v8 = *v6;
        v9 = strcmp(*v6, "-");
        if ( a4 )
        {
          if ( !strcmp(a4, v8) )
            break;
        }
        v7 = sub_5E70();
        if ( !v7 )
          return v16 - __readfsqword(0x28u);
        if ( v9 )
        {
          if ( stat(*v6, &v15) || v15.st_ino != v7->st_ino )
            goto LABEL_7;
          goto LABEL_14;
        }
LABEL_5:
        if ( fstat(0, &v15) || v15.st_ino != v7->st_ino )
          goto LABEL_7;
LABEL_14:
        if ( v15.st_dev == v7->st_dev )
          goto LABEL_15;
LABEL_7:
        v6 += 2;
        if ( v6 == v13 )
          return v16 - __readfsqword(0x28u);
      }
      if ( !v9 )
      {
        v7 = sub_5E70();
        if ( !v7 )
          return v16 - __readfsqword(0x28u);
        goto LABEL_5;
      }
LABEL_15:
      v10 = v5 + 13;
      if ( !v5 )
      {
        v12 = sub_77E0(&v14);
        v10 = (const char *)(v12 + 13);
        v5 = (const char *)v12;
        sub_B500((long long)v6, 0, 1u, v14, (char *)(v12 + 13));
      }
      *v6 = v10;
      v6 += 2;
      *(v6 - 1) = v5;
    }
    while ( v6 != v13 );
  }
  return v16 - __readfsqword(0x28u);
}



// Function: merge @ 0xb720
unsigned long long merge(long long *a1, unsigned long long a2, unsigned long long a3, char *a4)
{
  unsigned long long v6; // rsi
  unsigned long long v7; // r13
  long long v8; // rbx
  long long v9; // rax
  unsigned long long v10; // rsi
  long long v11; // r15
  unsigned long long v12; // rax
  unsigned long long v13; // rdx
  long long v14; // rcx
  unsigned long long v15; // rdi
  long long *v16; // r10
  unsigned long long v17; // rdx
  long long *v18; // r15
  unsigned long long v19; // rax
  unsigned long long v20; // r13
  unsigned long long v21; // rax
  unsigned long long v22; // rbx
  unsigned long long v23; // r15
  char **v24; // rbp
  char **v25; // r12
  long long v26; // rax
  long long v27; // rbp
  bool v28; // cc
  unsigned long long v29; // rbx
  unsigned long long v30; // r13
  size_t v31; // rdx
  FILE *v32; // rax
  char *v33; // rax
  long long v34; // rax
  unsigned long long v35; // rdx
  unsigned long long v36; // rsi
  unsigned long long v37; // rax
  unsigned long long v38; // rdx
  char *v40; // rbp
  char *v41; // rax
  long long v42; // [rsp+8h] [rbp-80h]
  unsigned long long j; // [rsp+8h] [rbp-80h]
  long long v44; // [rsp+8h] [rbp-80h]
  unsigned long long v45; // [rsp+8h] [rbp-80h]
  long long v46; // [rsp+8h] [rbp-80h]
  unsigned long long v47; // [rsp+10h] [rbp-78h]
  long long v48; // [rsp+10h] [rbp-78h]
  long long v49; // [rsp+18h] [rbp-70h]
  long long *v51; // [rsp+28h] [rbp-60h]
  FILE **v52; // [rsp+38h] [rbp-50h] BYREF
  FILE *v53; // [rsp+40h] [rbp-48h] BYREF
  unsigned long long i; // [rsp+48h] [rbp-40h]

  v6 = (unsigned int)dword_1E024;
  v47 = a3;
  for ( i = __readfsqword(0x28u); (unsigned int)dword_1E024 < v47; v47 += v7 - v8 )
  {
    v7 = 0;
    v8 = 0;
    if ( v6 > v47 )
    {
      v15 = v47;
      v19 = v6;
      v16 = a1;
      v18 = a1;
      v17 = 0;
    }
    else
    {
      do
      {
        v9 = sub_77E0(&v53);
        v10 = a2;
        v11 = v9;
        v42 = v9 + 13;
        if ( (unsigned int)dword_1E024 <= a2 )
          v10 = (unsigned int)dword_1E024;
        v12 = sub_B500((long long)&a1[2 * v8], v10, (unsigned int)dword_1E024, v53, (char *)(v9 + 13));
        v13 = a2;
        v6 = (unsigned int)dword_1E024;
        if ( v12 <= a2 )
          v13 = v12;
        v8 += v12;
        v14 = 2 * v7++;
        v15 = v47 - v8;
        a1[v14] = v42;
        a1[v14 + 1] = v11;
        a2 -= v13;
      }
      while ( v6 <= v47 - v8 );
      v16 = &a1[v14 + 2];
      v17 = v7 % v6;
      v18 = &a1[2 * v8];
      v19 = v6 - v7 % v6;
    }
    if ( v15 > v19 )
    {
      v51 = v16;
      v45 = v15 + v17 + 1 - v6;
      v34 = sub_77E0(&v53);
      v35 = v45;
      v36 = a2;
      v49 = v34;
      v28 = v45 <= a2;
      v46 = v34 + 13;
      if ( v28 )
        v36 = v35;
      v37 = sub_B500((long long)v18, v36, v35, v53, (char *)(v34 + 13));
      v38 = a2;
      if ( v37 <= a2 )
        v38 = v37;
      ++v7;
      v8 += v37;
      *v51 = v46;
      v51[1] = v49;
      a2 -= v38;
      v18 = &a1[2 * v8];
      v16 = &a1[2 * v7];
    }
    a2 += v7;
    memmove(v16, v18, 16 * (v47 - v8));
    v6 = (unsigned int)dword_1E024;
  }
  v20 = v47;
  sub_B5B0((long long)a1, a2, v47, a4);
  for ( j = a2; ; j = v27 - v29 + 1 )
  {
    v21 = sub_7470((long long)a1, v20, (long long *)&v52);
    v22 = v21;
    if ( v20 != v21 )
    {
      if ( v21 <= 2 )
      {
        v40 = (char *)a1[2 * v21];
        v41 = dcgettext(0, "open failed", 5);
        sub_5E10(v41, v40);
      }
      goto LABEL_14;
    }
    v32 = sub_5EE0(a4, "w");
    if ( v32 )
      break;
    if ( *__errno_location() != 24 || v20 <= 2 )
    {
      v33 = dcgettext(0, "open failed", 5);
      sub_5E10(v33, a4);
    }
LABEL_14:
    v23 = v22 - 1;
    v24 = (char **)&a1[2 * v22 - 2];
    while ( 1 )
    {
      v25 = v24;
      sub_6040(v52[v23], *v24);
      v24 -= 2;
      v26 = sub_76B0(&v53, v23 > 2);
      if ( v26 )
        break;
      --v23;
    }
    v27 = j;
    v48 = v26;
    v28 = j <= v23;
    v29 = j;
    v44 = v26 + 13;
    if ( !v28 )
      v29 = v23;
    v30 = v20 - v23;
    sub_AD90((long long)a1, v29, v23, v53, (char *)(v26 + 13), v52);
    v31 = 16 * v30;
    v20 = v30 + 1;
    *a1 = v44;
    a1[1] = v48;
    memmove(a1 + 2, v25, v31);
  }
  sub_AD90((long long)a1, j, v20, v32, a4, v52);
  return i - __readfsqword(0x28u);
}



// Function: sort @ 0xbac0
long long sort(char **a1, unsigned long long a2, char *a3, unsigned long long a4)
{
  long long v5; // rbx
  char *v6; // r15
  FILE *v7; // rsi
  char i; // al
  long long v9; // rdx
  char v10; // bl
  unsigned long long v11; // r12
  long long v12; // rbp
  char *v13; // r14
  long long v15; // rdx
  unsigned long long v16; // rax
  unsigned long long v17; // rax
  long long *v18; // rbx
  long long *v19; // rbp
  long long *j; // rdx
  char *v21; // rax
  unsigned long long v22; // [rsp+0h] [rbp-138h]
  long long v23; // [rsp+10h] [rbp-128h]
  unsigned long long v24; // [rsp+20h] [rbp-118h]
  long long v25; // [rsp+28h] [rbp-110h]
  FILE *v28; // [rsp+40h] [rbp-F8h] BYREF
  FILE *stream; // [rsp+48h] [rbp-F0h] BYREF
  void *ptr; // [rsp+50h] [rbp-E8h] BYREF
  long long v31; // [rsp+58h] [rbp-E0h]
  unsigned long long v32; // [rsp+60h] [rbp-D8h]
  long long v33; // [rsp+68h] [rbp-D0h]
  long long v34; // [rsp+70h] [rbp-C8h]
  char v35; // [rsp+80h] [rbp-B8h]
  char v36[104]; // [rsp+90h] [rbp-A8h] BYREF
  unsigned long long v37; // [rsp+F8h] [rbp-40h]

  v24 = a2;
  v37 = __readfsqword(0x28u);
  v33 = 0;
  if ( !a2 )
    goto LABEL_28;
  v22 = 0;
  v5 = 0;
  while ( 2 )
  {
    v6 = *a1;
    v25 = 48;
    v28 = sub_6010(*a1, "r");
    v7 = v28;
    if ( a4 > 1 )
    {
      v15 = 1;
      v16 = 1;
      do
      {
        v16 *= 2LL;
        ++v15;
      }
      while ( a4 > v16 );
      v25 = 32 * v15;
      if ( v5 )
        goto LABEL_5;
      goto LABEL_26;
    }
    if ( !v5 )
    {
LABEL_26:
      v17 = sub_8700(&v28, a1, v24, v25);
      sub_6240((long long)&ptr, v25, v17);
      v7 = v28;
    }
LABEL_5:
    --v24;
    v35 = 0;
    for ( i = sub_A750((char **)&ptr, v7, v6); i; i = sub_A750((char **)&ptr, v28, v6) )
    {
      v10 = v35;
      v11 = v32;
      if ( !v35 )
        goto LABEL_13;
      if ( v24 )
      {
        v9 = v31;
        if ( v25 + 1 < v33 - v31 - v32 * v25 )
        {
          v34 = v31;
          sub_6040(v28, v6);
          goto LABEL_20;
        }
LABEL_13:
        *(long long *)&xmmword_1E840 = 0;
        v12 = sub_A740(ptr, v33, v9);
LABEL_14:
        ++v22;
        v10 = 0;
        v13 = (char *)(sub_77E0(&stream) + 13);
        goto LABEL_15;
      }
      *(long long *)&xmmword_1E840 = 0;
      v12 = sub_A740(ptr, v33, v9);
      if ( v34 | v22 )
        goto LABEL_14;
      sub_6040(v28, v6);
      v13 = a3;
      v22 = 0;
      stream = sub_6010(a3, "w");
LABEL_15:
      if ( v11 > 1 )
      {
        sub_7B40((long long)v36, a4);
        v23 = sub_7AA0(a4, v11, v12);
        sub_A4E0(v12, a4, v11, (long long *)(v23 + 128), (long long)v36, stream, v13);
        sub_7CD0(a4, (char *)v23);
        sub_7D10((long long)v36);
      }
      else
      {
        sub_9FB0(v12 - 32, stream, v13);
      }
      sub_6040(stream, v13);
      if ( v10 )
      {
        free(ptr);
        return sub_7010();
      }
    }
    sub_6040(v28, v6);
    if ( v24 )
    {
LABEL_20:
      ++a1;
      v5 = v33;
      continue;
    }
    break;
  }
  v24 = v22;
LABEL_28:
  free(ptr);
  v18 = ::ptr;
  v19 = (long long *)sub_12FD0(v24);
  for ( j = v19; v18; *(j - 2) = (long long)v21 )
  {
    j[1] = (long long)v18;
    v21 = (char *)v18 + 13;
    v18 = (long long *)*v18;
    j += 2;
  }
  sub_B720(v19, v24, v24, a3);
  free(v19);
  return sub_7010();
}



// Function: usage @ 0xbe50
void usage(int status)
{
  long long v2; // r12
  char *v3; // rax
  char *v4; // rax
  FILE *v5; // r12
  char *v6; // rax
  FILE *v7; // r12
  char *v8; // rax
  FILE *v9; // r12
  char *v10; // rax
  FILE *v11; // r12
  char *v12; // rax
  FILE *v13; // r12
  char *v14; // rax
  FILE *v15; // r12
  char *v16; // rax
  FILE *v17; // r12
  char *v18; // rax
  FILE *v19; // r12
  char *v20; // rax
  FILE *v21; // r12
  char *v22; // rax
  FILE *v23; // r12
  char *v24; // rax
  FILE *v25; // r12
  char *v26; // rax
  FILE *v27; // r12
  char *v28; // rax
  FILE *v29; // r12
  char *v30; // rax
  char *v31; // rax
  FILE *v32; // r12
  char *v33; // rax
  FILE *v34; // r12
  char *v35; // rax
  FILE *v36; // r12
  char *v37; // rax
  FILE *v38; // r12
  char *v39; // rax
  FILE *v40; // rsi
  char *v41; // rdi

  v2 = qword_1E888;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5);
    __printf_chk(1, v4);
    v5 = stdout;
    v6 = dcgettext(0, "Write sorted concatenation of all FILE(s) to standard output.\n", 5);
    fputs_unlocked(v6, v5);
    sub_58D0();
    sub_5900();
    v7 = stdout;
    v8 = dcgettext(0, "Ordering options:\n\n", 5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "  -b, --ignore-leading-blanks  ignore leading blanks\n"
            "  -d, --dictionary-order      consider only blanks and alphanumeric characters\n"
            "  -f, --ignore-case           fold lower case to upper case characters\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(
            0,
            "  -g, --general-numeric-sort  compare according to general numerical value\n"
            "  -i, --ignore-nonprinting    consider only printable characters\n"
            "  -M, --month-sort            compare (unknown) < 'JAN' < ... < 'DEC'\n",
            5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(0, "  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n", 5);
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = dcgettext(
            0,
            "  -n, --numeric-sort          compare according to string numerical value\n"
            "  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n"
            "      --random-source=FILE    get random bytes from FILE\n"
            "  -r, --reverse               reverse the result of comparisons\n",
            5);
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = dcgettext(
            0,
            "      --sort=WORD             sort according to WORD:\n"
            "                                general-numeric -g, human-numeric -h, month -M,\n"
            "                                numeric -n, random -R, version -V\n"
            "  -V, --version-sort          natural sort of (version) numbers within text\n"
            "\n",
            5);
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = dcgettext(0, "Other options:\n\n", 5);
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = dcgettext(
            0,
            "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n"
            "                            for more use temp files\n",
            5);
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = dcgettext(
            0,
            "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n"
            "  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n"
            "      --compress-program=PROG  compress temporaries with PROG;\n"
            "                              decompress them with PROG -d\n",
            5);
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = dcgettext(
            0,
            "      --debug               annotate the part of the line used to sort,\n"
            "                              and warn about questionable usage to stderr\n"
            "      --files0-from=F       read input from the files specified by\n"
            "                            NUL-terminated names in file F;\n"
            "                            If F is - then read names from standard input\n",
            5);
    fputs_unlocked(v26, v25);
    v27 = stdout;
    v28 = dcgettext(
            0,
            "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n"
            "  -m, --merge               merge already sorted files; do not sort\n",
            5);
    fputs_unlocked(v28, v27);
    v29 = stdout;
    v30 = dcgettext(
            0,
            "  -o, --output=FILE         write result to FILE instead of standard output\n"
            "  -s, --stable              stabilize sort by disabling last-resort comparison\n"
            "  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n",
            5);
    fputs_unlocked(v30, v29);
    v31 = dcgettext(
            0,
            "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n"
            "  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n"
            "                              multiple options specify multiple directories\n"
            "      --parallel=N          change the number of sorts run concurrently to N\n"
            "  -u, --unique              with -c, check for strict ordering;\n"
            "                              without -c, output only the first of an equal run\n",
            5);
    __printf_chk(1, v31);
    v32 = stdout;
    v33 = dcgettext(0, "  -z, --zero-terminated     line delimiter is NUL, not newline\n", 5);
    fputs_unlocked(v33, v32);
    v34 = stdout;
    v35 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v35, v34);
    v36 = stdout;
    v37 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v37, v36);
    v38 = stdout;
    v39 = dcgettext(
            0,
            "\n"
            "KEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\n"
            "field number and C a character position in the field; both are origin 1, and\n"
            "the stop position defaults to the line's end.  If neither -t nor -b is in\n"
            "effect, characters in a field are counted from the beginning of the preceding\n"
            "whitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\n"
            "which override global ordering options for that key.  If no key is given, use\n"
            "the entire line as the key.  Use --debug to diagnose incorrect key usage.\n"
            "\n"
            "SIZE may be followed by the following multiplicative suffixes:\n",
            5);
    fputs_unlocked(v39, v38);
    v40 = stdout;
    v41 = dcgettext(
            0,
            "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n"
            "\n"
            "*** WARNING ***\n"
            "The locale specified by the environment affects sort order.\n"
            "Set LC_ALL=C to get the traditional sort order that uses\n"
            "native byte values.\n",
            5);
    fputs_unlocked(v41, v40);
    sub_8D30();
  }
  exit(status);
}



// Function: is_prime @ 0xd040
bool is_prime(unsigned long long a1)
{
  unsigned long long v1; // rdx
  long long v2; // r8
  unsigned long long v3; // rsi
  unsigned long long v4; // rcx

  v1 = a1 - (a1 / 3 + (((0xAAAAAAAAAAAAAAABLL * (unsigned __int128)a1) >> 64) & 0xFFFFFFFFFFFFFFFELL));
  if ( a1 > 9 && v1 )
  {
    v2 = 16;
    v3 = 9;
    v4 = 3;
    do
    {
      v4 += 2LL;
      v3 += v2;
      v1 = a1 % v4;
      if ( a1 <= v3 )
        break;
      v2 += 8;
    }
    while ( v1 );
  }
  return v1 != 0;
}



// Function: next_prime @ 0xd0b0
unsigned long long next_prime(unsigned long long a1)
{
  long long v1; // rax
  unsigned long long v2; // r9

  v1 = 10;
  if ( a1 >= 0xA )
    v1 = a1;
  v2 = v1 | 1;
  if ( (v1 | 1) != 0xFFFFFFFFFFFFFFFFLL )
  {
    do
    {
      if ( sub_D040(v2) )
        break;
      v2 += 2LL;
    }
    while ( v2 != -1 );
  }
  return v2;
}



// Function: raw_hasher @ 0xd0f0
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_13A80(a1, 3) % a2;
}



// Function: raw_comparator @ 0xd110
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0xd120
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_17200 )
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
    *(long long *)(a1 + 40) = &unk_17200;
    return 0;
  }
  return result;
}



// Function: free_entry @ 0xd1b0
long long free_entry(long long a1, long long *a2)
{
  long long result; // rax

  result = *(long long *)(a1 + 72);
  *a2 = 0;
  a2[1] = result;
  *(long long *)(a1 + 72) = a2;
  return result;
}



// Function: safe_hasher @ 0xd1d0
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v2; // rax

  v2 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( *(long long *)(a1 + 16) <= v2 )
    abort();
  return *(long long *)a1 + 16 * v2;
}



// Function: hash_find_entry @ 0xd200
long long hash_find_entry(long long a1, long long a2, __m128i **a3, char a4)
{
  __m128i *v6; // rax
  __m128i *v7; // rbx
  long long result; // rax
  __m128i *v9; // rsi
  long long *v10; // rax
  long long *v11; // rsi
  long long v12; // [rsp+8h] [rbp-30h]
  long long v13; // [rsp+8h] [rbp-30h]

  v6 = (__m128i *)sub_D1D0(a1, a2);
  *a3 = v6;
  if ( !v6->m128i_i64[0] )
    return 0;
  v7 = v6;
  if ( v6->m128i_i64[0] != a2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(a2) )
    {
      result = v7->m128i_i64[0];
      goto LABEL_5;
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
          v13 = *v11;
          v7->m128i_i64[1] = v11[1];
          sub_D1B0(a1, v11);
          return v13;
        }
        return result;
      }
      v7 = (__m128i *)v7->m128i_i64[1];
    }
    return 0;
  }
  result = v6->m128i_i64[0];
LABEL_5:
  if ( a4 )
  {
    v9 = (__m128i *)v7->m128i_i64[1];
    if ( v9 )
    {
      v12 = result;
      *v7 = _mm_loadu_si128(v9);
      sub_D1B0(a1, v9);
      return v12;
    }
    else
    {
      v7->m128i_i64[0] = 0;
    }
  }
  return result;
}



// Function: allocate_entry @ 0xd300
long long *allocate_entry(long long a1)
{
  long long *result; // rax

  result = *(long long **)(a1 + 72);
  if ( !result )
    return malloc(0x10u);
  *(long long *)(a1 + 72) = result[1];
  return result;
}



// Function: transfer_entries @ 0xd330
long long transfer_entries(long long a1, long long a2, char a3)
{
  long long *v5; // rbp
  long long v6; // r15
  long long *v7; // r14
  long long v8; // r15
  long long *v9; // rax
  long long *v10; // rsi
  long long *v11; // rax
  long long *v12; // r14
  long long *v13; // rax
  long long v14; // rdx

  v5 = *(long long **)a2;
  if ( *(long long *)a2 >= *(long long *)(a2 + 8) )
    return 1;
  while ( 1 )
  {
    while ( 1 )
    {
      v6 = *v5;
      if ( *v5 )
      {
        v7 = (long long *)v5[1];
        if ( v7 )
        {
          do
          {
            while ( 1 )
            {
              v8 = *v7;
              v9 = (long long *)sub_D1D0(a1, *v7);
              v10 = v7;
              v7 = (long long *)v7[1];
              if ( !*v9 )
                break;
              v10[1] = v9[1];
              v9[1] = v10;
              if ( !v7 )
                goto LABEL_10;
            }
            *v9 = v8;
            ++*(long long *)(a1 + 24);
            sub_D1B0(a1, v10);
          }
          while ( v7 );
LABEL_10:
          v6 = *v5;
        }
        v5[1] = 0;
        if ( !a3 )
          break;
      }
      v5 += 2;
      if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
        return 1;
    }
    v11 = (long long *)sub_D1D0(a1, v6);
    v12 = v11;
    if ( !*v11 )
    {
      *v11 = v6;
      ++*(long long *)(a1 + 24);
      goto LABEL_15;
    }
    v13 = sub_D300(a1);
    if ( !v13 )
      return 0;
    v14 = v12[1];
    *v13 = v6;
    v13[1] = v14;
    v12[1] = v13;
LABEL_15:
    *v5 = 0;
    v5 += 2;
    --*(long long *)(a2 + 24);
    if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
      return 1;
  }
}



// Function: hash_get_n_buckets @ 0xd4f0
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0xd500
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0xd510
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0xd520
unsigned long long hash_get_max_bucket_length(long long a1)
{
  long long *v1; // rcx
  unsigned long long v2; // rsi
  unsigned long long v3; // r8
  long long v4; // rax
  unsigned long long i; // rdx

  v1 = *(long long **)a1;
  v2 = *(long long *)(a1 + 8);
  v3 = 0;
  if ( *(long long *)a1 < v2 )
  {
    do
    {
      while ( !*v1 )
      {
        v1 += 2;
        if ( (unsigned long long)v1 >= v2 )
          return v3;
      }
      v4 = v1[1];
      for ( i = 1; v4; ++i )
        v4 = *(long long *)(v4 + 8);
      if ( v3 < i )
        v3 = i;
      v1 += 2;
    }
    while ( (unsigned long long)v1 < v2 );
  }
  return v3;
}



// Function: hash_print_statistics @ 0xd5f0
long long hash_print_statistics(long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // r13
  unsigned long long v4; // r12
  long long v5; // r9
  double v6; // xmm0_8
  double v7; // xmm1_8

  sub_D510(a1);
  v2 = sub_D4F0(a1);
  v3 = sub_D500(a1);
  v4 = sub_D520(a1);
  __fprintf_chk(a2, 1, "# entries:         %lu\n", v5);
  __fprintf_chk(a2, 1, "# buckets:         %lu\n", v2);
  if ( v3 < 0 )
  {
    v6 = ((double)(int)(v3 & 1 | ((unsigned long long)v3 >> 1)) + (double)(int)(v3 & 1 | ((unsigned long long)v3 >> 1)))
       * 100.0;
    if ( v2 >= 0 )
      goto LABEL_3;
LABEL_6:
    v7 = (double)(int)(v2 & 1 | ((unsigned long long)v2 >> 1)) + (double)(int)(v2 & 1 | ((unsigned long long)v2 >> 1));
    goto LABEL_4;
  }
  v6 = (double)(int)v3 * 100.0;
  if ( v2 < 0 )
    goto LABEL_6;
LABEL_3:
  v7 = (double)(int)v2;
LABEL_4:
  __fprintf_chk(a2, 1, "# buckets used:    %lu (%.2f%%)\n", v3, v6 / v7);
  return __fprintf_chk(a2, 1, "max bucket length: %lu\n", v4);
}



// Function: hash_lookup @ 0xd710
long long hash_lookup(long long a1, long long a2)
{
  long long v2; // rbp
  long long *v3; // rax
  long long v4; // rsi
  long long *v5; // rbx

  v2 = a2;
  v3 = (long long *)sub_D1D0(a1, a2);
  v4 = *v3;
  if ( !*v3 )
    return 0;
  v5 = v3;
  while ( v4 != v2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(v2) )
      return *v5;
    v5 = (long long *)v5[1];
    if ( !v5 )
      return 0;
    v4 = *v5;
  }
  return v2;
}



// Function: hash_get_next @ 0xd7c0
long long hash_get_next(long long a1, long long a2)
{
  long long *v2; // rax
  long long *v3; // rdx
  long long v4; // rcx
  long long v5; // r8

  v2 = (long long *)sub_D1D0(a1, a2);
  v3 = v2;
  while ( 1 )
  {
    v4 = *v3;
    v3 = (long long *)v3[1];
    if ( v4 == a2 )
      break;
    if ( !v3 )
      goto LABEL_7;
  }
  if ( v3 )
    return *v3;
LABEL_7:
  while ( 1 )
  {
    v2 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v2 )
      break;
    v5 = *v2;
    if ( *v2 )
      return v5;
  }
  return 0;
}



// Function: hash_do_for_each @ 0xd880
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



// Function: hash_initialize @ 0xd970
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
    v7 = sub_D0F0;
  if ( !a4 )
    v8 = sub_D110;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_17200;
    v9[5] = v5;
    if ( (unsigned char)sub_D120((long long)v9)
      && (v10 = sub_D460(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
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



// Function: hash_clear @ 0xda80
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



// Function: hash_free @ 0xdb30
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



// Function: hash_rehash @ 0xdc20
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
  v2 = sub_D460(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
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
  v6 = sub_D330((long long)&ptr, a1, 0);
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
    if ( !(unsigned char)sub_D330(a1, (long long)&ptr, 1) || !(unsigned char)sub_D330(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0xdda0
long long hash_insert_if_absent(long long *a1, long long a2, long long *a3)
{
  long long v5; // rax
  unsigned int v6; // r8d
  long long v8; // rax
  float v9; // xmm5_4
  long long v10; // rax
  float v11; // xmm4_4
  long long *v12; // rax
  __m128i *v13; // rdx
  float v14; // xmm4_4
  long long v15; // rax
  float v16; // xmm0_4
  float v17; // xmm5_4
  float v18; // xmm4_4
  long long v19; // rdx
  long long v20; // rdx
  long long v21; // rsi
  __m128i *v22[7]; // [rsp+0h] [rbp-38h] BYREF

  v22[1] = (__m128i *)__readfsqword(0x28u);
  if ( !a2 )
LABEL_26:
    abort();
  v5 = sub_D200((long long)a1, a2, v22, 0);
  if ( !v5 )
  {
    v8 = a1[3];
    if ( v8 < 0 )
    {
      v20 = a1[3] & 1LL | (a1[3] >> 1);
      v9 = (float)(int)v20 + (float)(int)v20;
    }
    else
    {
      v9 = (float)(int)v8;
    }
    v10 = a1[2];
    if ( v10 < 0 )
    {
      v19 = a1[2] & 1LL | (a1[2] >> 1);
      v11 = (float)(int)v19 + (float)(int)v19;
    }
    else
    {
      v11 = (float)(int)v10;
    }
    if ( v9 > (float)(*(float *)(a1[5] + 8LL) * v11) )
    {
      sub_D120((long long)a1);
      v15 = a1[5];
      v16 = *(float *)(v15 + 8);
      if ( v17 > (float)(v14 * v16) )
      {
        v18 = v14 * *(float *)(v15 + 12);
        if ( !*(char *)(v15 + 16) )
          v18 = v18 * v16;
        if ( v18 >= 1.8446744e19 )
          return (unsigned int)-1;
        v21 = v18 >= 9.223372e18
            ? (unsigned int)(int)(float)(v18 - 9.223372e18) ^ 0x8000000000000000LL
            : (unsigned int)(int)v18;
        if ( !(unsigned char)sub_DC20((long long)a1, v21) )
          return (unsigned int)-1;
        if ( sub_D200((long long)a1, a2, v22, 0) )
          goto LABEL_26;
      }
    }
    if ( !*(long long *)v22[0] )
    {
      *(long long *)v22[0] = a2;
      v6 = 1;
      ++a1[4];
      ++a1[3];
      return v6;
    }
    v12 = sub_D300((long long)a1);
    if ( v12 )
    {
      v13 = v22[0];
      *v12 = a2;
      v6 = 1;
      v12[1] = v13->m128i_i64[1];
      v13->m128i_i64[1] = (long long)v12;
      ++a1[4];
      return v6;
    }
    return (unsigned int)-1;
  }
  v6 = 0;
  if ( a3 )
    *a3 = v5;
  return v6;
}



// Function: hash_insert @ 0xdf90
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_DDA0(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0xdff0
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
  v3 = sub_D200((long long)a1, a2, v18, 1);
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
    sub_D120((long long)a1);
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
      if ( !(unsigned char)sub_DC20((long long)a1, v14) )
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



// Function: parse_omp_threads @ 0xe7d0
unsigned long long parse_omp_threads(char *nptr)
{
  const char *v1; // r12
  char v2; // di
  unsigned long long v3; // r13
  char v5; // r12
  char *v6; // rbp
  char *v7[5]; // [rsp+0h] [rbp-28h] BYREF

  v7[1] = (char *)__readfsqword(0x28u);
  if ( !nptr )
    return 0;
  v1 = nptr;
  v2 = *nptr;
  if ( v2 )
  {
    while ( (unsigned char)sub_13C00((unsigned int)v2) )
    {
      v2 = *++v1;
      if ( !v2 )
        goto LABEL_8;
    }
    v3 = 0;
    if ( !(unsigned char)sub_13B70((unsigned int)v2) )
      return v3;
  }
  else
  {
LABEL_8:
    v3 = 0;
    if ( !(unsigned char)sub_13B70(0) )
      return v3;
  }
  v7[0] = 0;
  v3 = strtoul(v1, v7, 10);
  if ( !v7[0] )
    return 0;
  v5 = *v7[0];
  if ( *v7[0] )
  {
    v6 = v7[0] + 1;
    while ( (unsigned char)sub_13C00((unsigned int)v5) )
    {
      v7[0] = v6;
      v5 = *v6++;
      if ( !v5 )
        return v3;
    }
    if ( v5 != 44 )
      return 0;
  }
  return v3;
}



// Function: num_processors_via_affinity_mask @ 0xe8d0
long long num_processors_via_affinity_mask()
{
  long long result; // rax
  cpu_set_t _0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars88; // [rsp+88h] [rbp+88h]

  vars88 = __readfsqword(0x28u);
  if ( sched_getaffinity(0, 0x80u, &_0) )
    return 0;
  result = __sched_cpucount(0x80u, &_0);
  if ( !(int)result )
    return 0;
  return result;
}



// Function: num_processors_ignoring_omp @ 0xe940
long long num_processors_ignoring_omp(int a1)
{
  long long v1; // rbx
  long long v2; // r12
  unsigned long long v4; // rax

  if ( a1 == 1 )
  {
    v2 = sub_E8D0();
    if ( v2 )
      return v2;
    v2 = sysconf(84);
    if ( v2 > 0 )
      return v2;
    return 1;
  }
  v1 = sysconf(83);
  v2 = v1;
  if ( (unsigned long long)(v1 - 1) <= 1 )
  {
    v4 = sub_E8D0();
    if ( v1 >= v4 )
      return v2;
    v1 = v4;
  }
  if ( v1 <= 0 )
    return 1;
  return v1;
}



// Function: num_processors @ 0xe9d0
unsigned long long num_processors(int a1)
{
  unsigned long long v1; // rbx
  unsigned long long result; // rax
  char *v3; // rax
  unsigned long long v4; // rbp
  char *v5; // rax

  v1 = -1;
  if ( a1 != 2 )
    goto LABEL_2;
  v3 = getenv("OMP_NUM_THREADS");
  v4 = sub_E7D0(v3);
  v5 = getenv("OMP_THREAD_LIMIT");
  a1 = 1;
  v1 = sub_E7D0(v5);
  if ( !v1 )
    v1 = -1;
  if ( v4 )
  {
    result = v4;
    if ( v1 <= v4 )
      return v1;
  }
  else
  {
LABEL_2:
    result = sub_E940(a1);
    if ( v1 <= result )
      return v1;
  }
  return result;
}


