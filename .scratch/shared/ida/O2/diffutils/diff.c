// Function: main @ 0x4f20
long long main(int a1, char **a2, char **a3)
{
  long long v3; // r14
  const struct option *v4; // r13
  long long v5; // rbp
  char *const *v6; // rbx
  char *v7; // rdi
  const char *v8; // r12
  long long v9; // rax
  unsigned int v10; // r10d
  int v11; // eax
  intmax_t v12; // rax
  long long v13; // rdx
  long long v14; // r14
  unsigned int v15; // r10d
  long long *v16; // rbx
  long long *v17; // rdi
  intmax_t v18; // rax
  long long v19; // rdx
  long long v20; // r12
  char *v21; // rax
  char *v22; // rax
  FILE *v23; // rbp
  char *v24; // rax
  FILE *v25; // rsi
  char **v26; // rbp
  const char *v27; // rsi
  FILE *v28; // rdi
  char *IO_write_ptr; // rax
  char *i; // r14
  char *v31; // rbx
  int v32; // edx
  char *v33; // rax
  size_t v34; // rax
  long long v35; // r14
  const char *v36; // rbx
  char *v37; // rbp
  char *v38; // rdi
  intmax_t v39; // rax
  const char *v40; // r14
  int v41; // eax
  intmax_t v42; // rax
  char v43; // bp
  FILE *v44; // rdi
  long long v46; // rdi
  const char *v47; // rsi
  unsigned long long v48; // rdi
  unsigned long long v49; // rcx
  unsigned long long v50; // rdx
  unsigned long long v51; // rdi
  unsigned long long v52; // rax
  int v53; // eax
  char v54; // al
  bool v55; // dl
  char **v56; // r14
  long long v57; // r15
  long long v58; // r13
  char *v59; // rdi
  char *v60; // rax
  long long v61; // r14
  const char *v62; // rdi
  const char *v63; // rax
  const char *v64; // rdi
  bool v65; // al
  int v66; // eax
  int j; // r12d
  int v68; // eax
  long long v69; // rax
  char *v70; // rdi
  int v71; // r8d
  const char *v72; // rdx
  const char *v73; // rdx
  int v74; // eax
  long long v75; // rax
  unsigned long long v76; // [rsp+0h] [rbp-88h]
  long long v77; // [rsp+8h] [rbp-80h]
  char v78; // [rsp+14h] [rbp-74h]
  unsigned int v79; // [rsp+14h] [rbp-74h]
  char v80; // [rsp+18h] [rbp-70h]
  unsigned int v81; // [rsp+18h] [rbp-70h]
  unsigned int v82; // [rsp+18h] [rbp-70h]
  unsigned int v83; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v84; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v85; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v86; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v87; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v88; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v89; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v90; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v91; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v92; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v93; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v94; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v95; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v96; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v97; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v98; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v99; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v100; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v101; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v102; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v103; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v104; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v105; // [rsp+1Ch] [rbp-6Ch]
  unsigned int v106; // [rsp+1Ch] [rbp-6Ch]
  char *const *v107; // [rsp+20h] [rbp-68h]
  int v108; // [rsp+20h] [rbp-68h]
  char *const *v109; // [rsp+28h] [rbp-60h]
  const struct option *v110; // [rsp+30h] [rbp-58h] BYREF
  long long v111; // [rsp+38h] [rbp-50h] BYREF
  char *endptr[9]; // [rsp+40h] [rbp-48h] BYREF

  v3 = 0xFFFFFFFFLL;
  v4 = &longopts;
  v5 = a1;
  v6 = a2;
  v7 = *a2;
  endptr[1] = (char *)__readfsqword(0x28u);
  v110 = 0;
  v111 = 0;
  status = 2;
  sub_14480(v7);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  v8 = "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ";
  textdomain("diffutils");
  sub_FD50(0);
  qword_26600 = (long long)&buffer;
  qword_265C0 = (long long)&stru_264C0;
  re_set_syntax(0x50A46u);
  v9 = sub_10310();
  byte_26180 = 0;
  qword_26410 = v9;
  sub_16F50();
  v78 = 0;
  v76 = 0;
  v80 = 0;
  v77 = -1;
  while ( 1 )
  {
    while ( 1 )
    {
      v10 = getopt_long(v5, v6, v8, v4, 0);
      if ( v10 == -1 )
        break;
      switch ( v10 )
      {
        case 0u:
        case 0x68u:
        case 0x80u:
        case 0x85u:
          goto LABEL_5;
        case 0x30u:
        case 0x31u:
        case 0x32u:
        case 0x33u:
        case 0x34u:
        case 0x35u:
        case 0x36u:
        case 0x37u:
        case 0x38u:
        case 0x39u:
          if ( (unsigned int)(v3 - 48) > 9 )
          {
            v3 = v10;
            v77 = (int)(v10 - 48);
          }
          else
          {
            v3 = v10;
            if ( v77 - ((int)v10 <= 51) > 0x666666666666665LL )
              v77 = 0x3FFFFFFFFFFFFFFFLL;
            else
              v77 = 10 * v77 + (int)(v10 - 48);
          }
          continue;
        case 0x42u:
          byte_26554 = 1;
          v3 = v10;
          continue;
        case 0x43u:
        case 0x55u:
          if ( optarg )
          {
            v81 = v10;
            v12 = strtoimax(optarg, endptr, 10);
            if ( *endptr[0] )
              goto LABEL_221;
            v10 = v81;
            if ( v12 < 0 )
              goto LABEL_221;
            v13 = 0x3FFFFFFFFFFFFFFFLL;
            if ( v12 <= 0x3FFFFFFFFFFFFFFFLL )
              v13 = v12;
            v14 = v13;
          }
          else
          {
            v14 = 3;
          }
          v82 = v10;
          sub_8F60((unsigned int)(v10 == 85) + 2);
          v15 = v82;
          if ( qword_26570 >= v14 )
          {
            v80 = 1;
            v3 = v15;
          }
          else
          {
            qword_26570 = v14;
            v3 = v82;
            v80 = 1;
          }
          break;
        case 0x44u:
          v95 = v10;
          sub_8F60(7);
          v34 = strlen(optarg);
          v35 = sub_16A40(7 * v34 + 97);
          __sprintf_chk(
            v35,
            1,
            -1,
            "%%=%c#ifndef %s\n"
            "%%<#endif /* ! %s */\n"
            "%c#ifdef %s\n"
            "%%>#endif /* %s */\n"
            "%c#ifndef %s\n"
            "%%<#else /* %s */\n"
            "%%>#endif /* %s */\n",
            0,
            optarg,
            optarg,
            0,
            optarg,
            optarg,
            0,
            optarg,
            optarg,
            optarg);
          v109 = v6;
          v36 = (const char *)v35;
          v3 = v95;
          v108 = v5;
          v37 = (char *)&s1;
          do
          {
            v38 = v37;
            v37 += 8;
            sub_8F00(v38, v36, "-D");
            v36 += strlen(v36) + 1;
          }
          while ( &byte_26480 != v37 );
          v5 = v108;
          v6 = v109;
          continue;
        case 0x45u:
          v3 = v10;
          if ( (unsigned int)dword_26558 <= 3 )
            dword_26558 |= 1u;
          continue;
        case 0x46u:
          v87 = v10;
          sub_8D60(&unk_265E0, optarg);
          v3 = v87;
          continue;
        case 0x48u:
          byte_26418 = 1;
          v3 = v10;
          continue;
        case 0x49u:
          v86 = v10;
          sub_8D60(&qword_265A0, optarg);
          v3 = v86;
          continue;
        case 0x4Cu:
          if ( !qword_26540 )
            goto LABEL_178;
          if ( !qword_26548 )
          {
            v3 = v10;
            qword_26548 = (long long)optarg;
            continue;
          }
          sub_E7B0("too many file label options");
LABEL_219:
          sub_8E40("invalid width '%s'");
          goto LABEL_220;
        case 0x4Eu:
          byte_26586 = 1;
          v3 = v10;
          continue;
        case 0x50u:
          byte_26585 = 1;
          v3 = v10;
          continue;
        case 0x53u:
          v100 = v10;
          sub_8F00(&qword_26488, optarg, "-S");
          v3 = v100;
          continue;
        case 0x54u:
          byte_26492 = 1;
          v3 = v10;
          continue;
        case 0x57u:
          v101 = v10;
          v42 = strtoimax(optarg, endptr, 10);
          if ( v42 <= 0 )
            goto LABEL_219;
          v10 = v101;
          if ( *endptr[0] )
            goto LABEL_219;
          if ( v42 == v76 )
            goto LABEL_5;
          if ( v76 )
            goto LABEL_223;
          v76 = v42;
          v3 = v101;
          continue;
        case 0x58u:
          v83 = v10;
          v11 = sub_10BD0(sub_10680, qword_26410, optarg, byte_26551 == 0 ? 0x10000000 : 268435472, 10);
          v10 = v83;
          if ( !v11 )
            goto LABEL_5;
          goto LABEL_224;
        case 0x5Au:
          v3 = v10;
          if ( (unsigned int)dword_26558 <= 3 )
            dword_26558 |= 2u;
          continue;
        case 0x61u:
          byte_26568 = 1;
          v3 = v10;
          continue;
        case 0x62u:
          v3 = v10;
          if ( (unsigned int)dword_26558 <= 3 )
            dword_26558 = 4;
          continue;
        case 0x63u:
          v102 = v10;
          v46 = 2;
          goto LABEL_89;
        case 0x64u:
          byte_26408 = 1;
          v3 = v10;
          continue;
        case 0x65u:
          v106 = v10;
          sub_8F60(4);
          v3 = v106;
          continue;
        case 0x66u:
          v105 = v10;
          sub_8F60(5);
          v3 = v105;
          continue;
        case 0x69u:
          byte_26552 = 1;
          v3 = v10;
          continue;
        case 0x6Cu:
          if ( path[0] )
          {
            v104 = v10;
            byte_26480 = 1;
            signal(17, 0);
            v3 = v104;
            continue;
          }
          sub_8E40("pagination not supported on this host");
LABEL_223:
          sub_E7B0("conflicting width options");
LABEL_224:
          sub_E770(optarg);
LABEL_225:
          sub_8E40("invalid horizon length '%s'");
          goto LABEL_226;
        case 0x6Eu:
          v103 = v10;
          sub_8F60(6);
          v3 = v103;
          continue;
        case 0x70u:
          v79 = v10;
          sub_8D60(&unk_265E0, "^[[:alpha:]$_]");
          v3 = v79;
          v78 = 1;
          continue;
        case 0x71u:
          byte_264A1 = 1;
          v3 = v10;
          continue;
        case 0x72u:
          byte_26608 = 1;
          v3 = v10;
          continue;
        case 0x73u:
          byte_26584 = 1;
          v3 = v10;
          continue;
        case 0x74u:
          byte_264A0 = 1;
          v3 = v10;
          continue;
        case 0x75u:
          v102 = v10;
          v46 = 3;
LABEL_89:
          sub_8F60(v46);
          v3 = v102;
          if ( qword_26570 <= 2 )
            qword_26570 = 3;
          continue;
        case 0x76u:
          sub_14920("Len Tower");
          sub_14920("Richard Stallman");
          v43 = sub_14920("David Hayes");
          sub_14920("Mike Haertel");
          sub_14920("Paul Eggert");
          v44 = stdout;
          sub_16840(stdout, v43);
          sub_9DF0(v44, "diff");
          return 0;
        case 0x77u:
          dword_26558 = 5;
          v3 = v10;
          continue;
        case 0x78u:
          v94 = v10;
          sub_10680(qword_26410, optarg, byte_26551 == 0 ? 0x10000000 : 268435472);
          v3 = v94;
          continue;
        case 0x79u:
          v93 = v10;
          sub_8F60(8);
          v3 = v93;
          continue;
        case 0x81u:
          v92 = v10;
          sub_8F00(&v110, optarg, "--from-file");
          v3 = v92;
          continue;
        case 0x82u:
          v20 = qword_36860;
          v21 = dcgettext(0, "Usage: %s [OPTION]... FILES\n", 5);
          __printf_chk(1, v21, v20);
          v22 = dcgettext(0, "Compare FILES line by line.", 5);
          __printf_chk(1, "%s\n\n", v22);
          v23 = stdout;
          v24 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
          v25 = v23;
          v26 = off_24F40;
          fputs_unlocked(v24, v25);
          v27 = "    --normal                  output a normal diff (the default)";
          do
          {
            while ( !*v27 )
            {
              v28 = stdout;
              IO_write_ptr = stdout->_IO_write_ptr;
              if ( IO_write_ptr >= stdout->_IO_write_end )
              {
                __overflow(stdout, 10);
              }
              else
              {
                stdout->_IO_write_ptr = IO_write_ptr + 1;
                *IO_write_ptr = 10;
              }
              v27 = v26[1];
              ++v26;
              if ( !v27 )
                goto LABEL_51;
            }
            for ( i = dcgettext(0, v27, 5); ; i = v31 )
            {
              v33 = strchr(i, 10);
              if ( !v33 )
                break;
              v31 = v33 + 1;
              v32 = (int)v33 + 1 - (int)i;
              if ( v32 > 4095 )
                __assert_fail("msglen < 4096", "diff.c", 0x3F8u, "usage");
              __printf_chk(1, "  %.*s", v32, i);
            }
            v28 = (FILE *)(&dword_0 + 1);
            ++v26;
            __printf_chk(1, &aS[2 * (*i != 32 && *i != 45)], i);
            v27 = *v26;
          }
          while ( *v26 );
LABEL_51:
          sub_16920();
          sub_9DF0(v28, v27);
          return 0;
        case 0x83u:
          v91 = v10;
          v18 = strtoimax(optarg, endptr, 10);
          if ( *endptr[0] || v18 < 0 )
            goto LABEL_225;
          v19 = qword_26560;
          v3 = v91;
          if ( qword_26560 < v18 )
            v19 = v18;
          qword_26560 = v19;
          continue;
        case 0x84u:
          byte_26551 = 1;
          v3 = v10;
          continue;
        case 0x86u:
          byte_26439 = 1;
          v3 = v10;
          continue;
        case 0x87u:
          v90 = v10;
          sub_8F60(7);
          v107 = v6;
          v16 = qword_26440;
          v3 = v90;
          do
          {
            v17 = v16++;
            sub_8F00(v17, optarg, "--line-format");
          }
          while ( &unk_26458 != (_UNKNOWN *)v16 );
          v6 = v107;
          continue;
        case 0x88u:
          byte_26550 = 1;
          v3 = v10;
          continue;
        case 0x89u:
          byte_26551 = 0;
          v3 = v10;
          continue;
        case 0x8Au:
          v89 = v10;
          sub_8F60(1);
          v3 = v89;
          continue;
        case 0x8Bu:
          v88 = v10;
          sub_8F60(8);
          byte_2643A = 1;
          v3 = v88;
          continue;
        case 0x8Cu:
          byte_26490 = 1;
          v3 = v10;
          continue;
        case 0x8Du:
          byte_26491 = 1;
          v3 = v10;
          continue;
        case 0x8Eu:
          byte_26438 = 1;
          v3 = v10;
          continue;
        case 0x8Fu:
          v97 = v10;
          v39 = strtoimax(optarg, endptr, 10);
          if ( v39 > 0 )
          {
            v10 = v97;
            if ( !*endptr[0] )
            {
              if ( v39 == qword_26498 )
              {
LABEL_5:
                v3 = v10;
                continue;
              }
              if ( !qword_26498 )
              {
                qword_26498 = v39;
                v3 = v97;
                continue;
              }
LABEL_226:
              sub_E7B0("conflicting tabsize options");
              start();
            }
          }
          v47 = optarg;
          v62 = "invalid tabsize '%s'";
          v69 = sub_8E40("invalid tabsize '%s'");
LABEL_211:
          j = 0;
          while ( (int)v69 < (int)v5 )
          {
            v47 = v6[v69];
            v62 = 0;
            v74 = sub_8F90(0, v47, v3);
            if ( j < v74 )
              j = v74;
            v69 = ++optind;
          }
          goto LABEL_173;
        case 0x90u:
          v96 = v10;
          sub_8F00(&v111, optarg, "--to-file");
          v3 = v96;
          continue;
        case 0x91u:
        case 0x92u:
        case 0x93u:
          v85 = v10;
          sub_8F60(7);
          v3 = v85 - 145;
          sub_8F00(&qword_26440[(int)v3], optarg, &aUnchangedLineF[24 * (int)v3]);
          continue;
        case 0x94u:
        case 0x95u:
        case 0x96u:
        case 0x97u:
          v84 = v10;
          sub_8F60(7);
          v3 = v84 - 148;
          sub_8F00(&(&s1)[(int)v3], optarg, &aUnchangedGroup[25 * (int)v3]);
          continue;
        case 0x98u:
          v40 = optarg;
          if ( optarg && (v99 = v10, v41 = strcmp(optarg, "auto"), v10 = v99, v41) )
          {
            if ( !strcmp(v40, "always") )
            {
              dword_2657C = 2;
              v3 = v99;
              continue;
            }
            if ( strcmp(v40, "never") )
            {
LABEL_220:
              sub_8E40("invalid color '%s'");
LABEL_221:
              sub_8E40("invalid context length '%s'");
            }
            dword_2657C = 0;
            v3 = v99;
          }
          else
          {
            dword_2657C = 1;
            v3 = v10;
          }
          break;
        case 0x99u:
          v98 = v10;
          sub_E7F0(optarg);
          v3 = v98;
          continue;
        case 0x9Au:
          byte_26180 = 1;
          v3 = v10;
          continue;
        default:
          sub_8E40(0);
          goto LABEL_191;
      }
    }
    if ( dword_2657C == 1 )
    {
      v70 = getenv("TERM");
      if ( v70 )
      {
        if ( !strcmp(v70, "dumb") )
          dword_2657C = 0;
      }
    }
    if ( dword_26580 )
      goto LABEL_143;
    if ( !v78 )
      break;
    sub_8F60(2);
    if ( v77 >= 0 )
    {
      if ( dword_26580 != 2 )
        goto LABEL_114;
LABEL_191:
      if ( (unsigned char)sub_10F10(2) )
LABEL_114:
        qword_26400 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
      else
        qword_26400 = (long long)"%a %b %e %T %Y";
LABEL_115:
      v47 = (const char *)qword_26570;
      if ( (unsigned int)(dword_26580 - 2) <= 1 && (v77 > qword_26570 || v77 != qword_26570 && !v80) )
      {
        qword_26570 = v77;
        v47 = (const char *)v77;
      }
      goto LABEL_116;
    }
    qword_26570 = 3;
    if ( dword_26580 == 2 )
    {
      if ( !(unsigned char)sub_10F10(2) )
      {
        qword_26400 = (long long)"%a %b %e %T %Y";
LABEL_146:
        v47 = (const char *)qword_26570;
        goto LABEL_116;
      }
      v47 = (const char *)qword_26570;
      qword_26400 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
    }
    else
    {
      v47 = (char *)(&dword_0 + 3);
      qword_26400 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
    }
LABEL_116:
    if ( !qword_26498 )
      qword_26498 = 8;
    if ( !v76 )
    {
      if ( byte_264A0 )
      {
        v76 = 67;
        v52 = 63;
        goto LABEL_128;
      }
      v76 = 130;
LABEL_120:
      v48 = qword_26498;
      goto LABEL_121;
    }
    v48 = 1;
    if ( !byte_264A0 )
      goto LABEL_120;
LABEL_121:
    v49 = ((unsigned char)v76 & (unsigned char)(v48 + 3) & 1) + ((v48 + 3) >> 1) + (v76 >> 1);
    v50 = v49 % v48;
    v51 = v49 - v49 % v48;
    if ( v51 <= 3 || v51 >= v76 )
    {
      v52 = 0;
    }
    else
    {
      v52 = v50 - v49 + v76;
      if ( v52 > v51 - 3 )
        v52 = v51 - 3;
      if ( !v52 )
        v51 = v76;
      v76 = v51;
    }
LABEL_128:
    qword_26430 = v52;
    qword_26428 = v76;
    if ( qword_26560 < (long long)v47 )
      qword_26560 = (long long)v47;
    sub_8EA0(&unk_265E0);
    sub_8EA0(&qword_265A0);
    v53 = dword_26580;
    if ( dword_26580 != 7 )
    {
LABEL_131:
      v54 = byte_26438 | (v53 != 8);
      goto LABEL_132;
    }
    if ( !qword_26440[0] )
      qword_26440[0] = (long long)"%l\n";
    if ( !qword_26448 )
      qword_26448 = (long long)"%l\n";
    if ( !qword_26450 )
      qword_26450 = (long long)"%l\n";
    if ( !qword_26468 )
    {
      v72 = "%<";
      if ( qword_26478 )
        v72 = (const char *)qword_26478;
      qword_26468 = (long long)v72;
    }
    if ( !qword_26470 )
    {
      v73 = "%>";
      if ( qword_26478 )
        v73 = (const char *)qword_26478;
      qword_26470 = (long long)v73;
    }
    if ( s1 )
    {
      if ( qword_26478 )
        goto LABEL_167;
    }
    else
    {
      v47 = "%=";
      s1 = "%=";
      v64 = "%=";
      if ( qword_26478 )
      {
LABEL_184:
        v71 = strcmp(v64, "%=");
        v65 = 0;
        if ( !v71 )
          v65 = *(char *)qword_26440[0] == 0;
        goto LABEL_168;
      }
    }
    v47 = (const char *)qword_26470;
    qword_26478 = sub_FA30(qword_26468, qword_26470, "");
    v53 = dword_26580;
    if ( dword_26580 != 7 )
      goto LABEL_131;
LABEL_167:
    v64 = s1;
    v65 = 1;
    v47 = "%=";
    if ( *s1 )
      goto LABEL_184;
LABEL_168:
    v54 = v65;
LABEL_132:
    byte_26578 = v54;
    v55 = 1;
    if ( !qword_265A0 )
      v55 = dword_26558 != 0;
    v8 = (const char *)(v6 + 1);
    byte_26553 = byte_264A1 & ~(v55 | byte_26490 | byte_26552 | byte_26554);
    byte_26553 &= 1u;
    if ( optind > 1 )
    {
      v56 = (char **)(v6 + 1);
      v57 = 1;
      v58 = (long long)&v6[(unsigned int)(optind - 2) + 2];
      do
      {
        v59 = *v56++;
        v57 += sub_14D70(v59) + 1;
      }
      while ( (char **)v58 != v56 );
      v60 = (char *)sub_16A40(v57);
      v61 = (long long)v60;
      do
      {
        *v60 = 32;
        v47 = *(const char **)v8;
        v62 = v60 + 1;
        v8 += 8;
        v60 = (char *)sub_14DE0(v60 + 1, v47);
      }
      while ( (const char *)v58 != v8 );
      goto LABEL_139;
    }
    while ( 1 )
    {
      v62 = (char *)(&dword_0 + 1);
      v60 = (char *)sub_16A40(1);
      v61 = (long long)v60;
LABEL_139:
      v4 = v110;
      *v60 = 0;
      qword_26420 = v61;
      v3 = v111;
      if ( v4 )
      {
        if ( !v111 )
        {
          v66 = optind;
          for ( j = 0; v66 < (int)v5; optind = v66 )
          {
            v62 = 0;
            v47 = (const char *)v4;
            v68 = sub_8F90(0, v4, v6[v66]);
            if ( j < v68 )
              j = v68;
            v66 = optind + 1;
          }
LABEL_173:
          sub_E670();
          sub_9DF0(v62, v47);
          exit(j);
        }
        sub_E7B0("--from-file and --to-file both specified");
        goto LABEL_142;
      }
      v69 = optind;
      if ( v111 )
        goto LABEL_211;
      if ( (int)v5 - optind == 2 )
      {
        v75 = optind + 1LL;
        v62 = 0;
        v47 = v6[v75 - 1];
        j = sub_8F90(0, v47, v6[v75]);
        goto LABEL_173;
      }
      if ( (int)v5 - optind > 1 )
        break;
      v47 = v6[v5 - 1];
      sub_8E40("missing operand after '%s'");
    }
    sub_8E40("extra operand '%s'");
LABEL_178:
    v3 = v10;
    qword_26540 = (long long)optarg;
  }
LABEL_142:
  sub_8F60(1);
LABEL_143:
  if ( dword_26580 != 2 || (unsigned char)sub_10F10(2) )
    v63 = "%Y-%m-%d %H:%M:%S.%N %z";
  else
    v63 = "%a %b %e %T %Y";
  qword_26400 = (long long)v63;
  if ( v77 < 0 )
    goto LABEL_146;
  goto LABEL_115;
}



// Function: diff_2_files @ 0x6e20
long long diff_2_files(long long a1)
{
  long long v1; // rbp
  long long v2; // rax
  int v3; // ecx
  int v4; // edx
  void *v5; // r14
  long long v6; // rax
  long long v7; // rbx
  long long v8; // rax
  long long v9; // rdi
  long long v10; // r13
  const void *v11; // r14
  unsigned int v12; // r12d
  long long v13; // rbx
  void *v14; // r13
  long long v16; // rax
  long long v17; // rdx
  long long v18; // rax
  long long v19; // rax
  long long v20; // rbx
  long long v21; // rdi
  long long v22; // rax
  long long v23; // rax
  long long v24; // rdx
  long long v25; // rax
  long long *v26; // r12
  long long v27; // rbx
  long long v28; // rcx
  long long v29; // rax
  long long v30; // rdx
  long long v31; // rax
  long long v32; // rcx
  long long i; // rdx
  long long v34; // rax
  char **v35; // r15
  long long v36; // rax
  long long *v37; // rcx
  long long *v38; // r10
  char **v39; // r9
  char *v40; // rdi
  unsigned long long v41; // rax
  char *v42; // r8
  long long v43; // r11
  unsigned long long v44; // rdx
  unsigned long long v45; // rsi
  long long v46; // rdx
  long long v47; // r13
  unsigned long long v48; // r13
  char *v49; // rdx
  long long v50; // r11
  long long j; // rax
  char *v52; // r9
  char v53; // cl
  long long *v54; // rax
  char *m; // rsi
  long long v56; // r8
  long long v57; // rcx
  long long v58; // rdx
  long long v59; // r10
  long long v60; // rcx
  long long k; // rsi
  char v62; // r8
  long long v63; // r8
  char *v64; // rcx
  long long v65; // rcx
  long long v66; // rbx
  long long v67; // rcx
  long long v68; // r10
  long long v69; // rsi
  char *v70; // r14
  bool v71; // cc
  char *v72; // rax
  char *v73; // rbx
  long long v74; // rsi
  char v75; // r10
  char *v76; // rax
  char *v77; // r8
  long long v78; // rsi
  long long v79; // rbx
  long long v80; // rax
  long long v81; // r10
  long long v82; // rdx
  long long v83; // rax
  long long v84; // rbx
  long long v85; // rsi
  long long v86; // rcx
  long long v87; // r11
  long long v88; // rdi
  long long v89; // r10
  long long v90; // r8
  bool v91; // zf
  long long v92; // rax
  long long v93; // rdx
  long long v94; // r13
  long long v95; // r9
  long long v96; // r9
  char *v97; // r9
  long long v98; // rsi
  long long v99; // r8
  long long v100; // rbx
  long long v101; // r12
  char *fastmap; // r15
  unsigned char v103; // dl
  char *v104; // rbp
  char **v105; // rax
  char *v106; // rdi
  char **v107; // rbx
  int v108; // eax
  char *v109; // rdi
  long long v110; // rax
  long long v111; // r13
  long long v112; // rax
  long long v113; // r14
  long long v114; // r14
  char *v115; // rax
  long long v116; // rsi
  long long v117; // rdi
  long long v118; // r14
  long long v119; // r13
  char *v120; // rdx
  long long v121; // rcx
  long long v122; // r9
  unsigned char v123; // al
  char *v124; // r15
  char *v125; // rbx
  long long v126; // rbx
  long long v127; // r12
  long long v128; // rbp
  long long v129; // r15
  char *v130; // rax
  char *v131; // rax
  const char *v132; // rcx
  const void *v133; // [rsp+8h] [rbp-F0h]
  long long v134; // [rsp+8h] [rbp-F0h]
  long long v135; // [rsp+8h] [rbp-F0h]
  char **v136; // [rsp+10h] [rbp-E8h]
  long long v137; // [rsp+10h] [rbp-E8h]
  long long v138; // [rsp+10h] [rbp-E8h]
  char *v139; // [rsp+10h] [rbp-E8h]
  char *ptr; // [rsp+18h] [rbp-E0h]
  long long v141; // [rsp+20h] [rbp-D8h]
  long long v142; // [rsp+28h] [rbp-D0h]
  long long v143; // [rsp+38h] [rbp-C0h]
  char v144[8]; // [rsp+40h] [rbp-B8h] BYREF
  char v145[8]; // [rsp+48h] [rbp-B0h] BYREF
  char v146[8]; // [rsp+50h] [rbp-A8h] BYREF
  char v147[8]; // [rsp+58h] [rbp-A0h] BYREF
  long long v148[2]; // [rsp+60h] [rbp-98h] BYREF
  long long v149; // [rsp+70h] [rbp-88h]
  long long v150; // [rsp+78h] [rbp-80h]
  char v151; // [rsp+80h] [rbp-78h]
  long long v152; // [rsp+88h] [rbp-70h]
  char v153; // [rsp+90h] [rbp-68h] BYREF
  char *v154; // [rsp+98h] [rbp-60h]
  long long v155[11]; // [rsp+A0h] [rbp-58h] BYREF

  v1 = a1;
  v155[3] = __readfsqword(0x28u);
  if ( (unsigned char)sub_BBC0(a1, (unsigned char)byte_26553) )
  {
    v2 = *(long long *)(a1 + 368);
    v3 = *(int *)a1;
    v4 = *(int *)(a1 + 304);
    if ( *(long long *)(a1 + 64) <= 0LL
      || *(long long *)(a1 + 64) == v2
      || v2 <= 0
      || v3 >= 0 && (*(int *)(a1 + 40) & 0xF000) != 0x8000
      || v4 >= 0 && (*(int *)(a1 + 344) & 0xF000) != 0x8000 )
    {
      v5 = *(void **)(a1 + 160);
      if ( v3 == v4 )
      {
        v14 = *(void **)(a1 + 464);
        v12 = 0;
        goto LABEL_16;
      }
      v6 = sub_17190(*(long long *)(a1 + 72), *(long long *)(a1 + 376), 0x7FFFFFFFFFFFFFFELL);
      v7 = sub_17190(8, v6, 0x7FFFFFFFFFFFFFFELL);
      v8 = sub_16A90(v5, v7);
      v9 = *(long long *)(a1 + 464);
      v10 = v7;
      *(long long *)(v1 + 160) = v8;
      *(long long *)(v1 + 464) = sub_16A90(v9, v7);
      while ( 1 )
      {
        if ( *(int *)v1 >= 0 )
          sub_B8A0(v1, v10 - *(long long *)(v1 + 176));
        if ( *(int *)(v1 + 304) >= 0 )
          sub_B8A0(v1 + 304, v10 - *(long long *)(v1 + 480));
        v13 = *(long long *)(v1 + 176);
        if ( v13 != *(long long *)(v1 + 480) )
          break;
        v11 = *(const void **)(v1 + 464);
        v133 = *(const void **)(v1 + 160);
        v12 = memcmp(v133, v11, *(long long *)(v1 + 176));
        if ( v12 )
          break;
        if ( v13 != v10 )
        {
          v14 = (void *)v11;
          v5 = (void *)v133;
          goto LABEL_16;
        }
        *(long long *)(v1 + 480) = 0;
        *(long long *)(v1 + 176) = 0;
      }
    }
    v12 = 1;
    sub_6D90(1, v1);
    v5 = *(void **)(v1 + 160);
    v14 = *(void **)(v1 + 464);
    goto LABEL_16;
  }
  ptr = (char *)sub_FAB0(*(long long *)(a1 + 200) + *(long long *)(a1 + 504) + 4LL);
  *(long long *)(a1 + 280) = ptr + 1;
  v16 = *(long long *)(a1 + 200);
  v17 = (long long)&ptr[v16 + 3];
  v18 = *(long long *)(a1 + 504) + v16;
  *(long long *)(a1 + 584) = v17;
  v19 = sub_16A40(16 * v18);
  v20 = *(long long *)(a1 + 200);
  v21 = *(long long *)(a1 + 296);
  *(long long *)(v1 + 256) = v19;
  v22 = 8 * v20 + v19;
  *(long long *)(v1 + 264) = v22;
  v23 = 8 * v20 + v22;
  v24 = *(long long *)(v1 + 504);
  *(long long *)(v1 + 560) = v23;
  *(long long *)(v1 + 568) = v23 + 8 * v24;
  v25 = sub_FAB0(16 * v21);
  v71 = *(long long *)(v1 + 200) <= 0LL;
  v26 = (long long *)v25;
  v155[0] = v25;
  v27 = v25 + 8LL * *(long long *)(v1 + 296);
  v155[1] = v27;
  if ( !v71 )
  {
    v28 = *(long long *)(v1 + 248);
    v29 = 0;
    do
    {
      v30 = *(long long *)(v28 + 8 * v29++);
      ++v26[v30];
    }
    while ( v29 < *(long long *)(v1 + 200) );
  }
  v31 = *(long long *)(v1 + 504);
  if ( v31 > 0 )
  {
    v32 = *(long long *)(v1 + 552);
    for ( i = 0; i < v31; ++i )
    {
      v34 = *(long long *)(v32 + 8 * i);
      ++*(long long *)(v27 + 8 * v34);
      v31 = *(long long *)(v1 + 504);
    }
  }
  v35 = (char **)&v153;
  v36 = sub_FAB0(*(long long *)(v1 + 200) + v31);
  v37 = (long long *)(v1 + 248);
  v38 = v155;
  v39 = (char **)&v153;
  v40 = (char *)v36;
  v41 = *(long long *)(v1 + 200);
  v42 = v40;
  v154 = &v40[v41];
  while ( 1 )
  {
    v43 = *v37;
    v44 = v41 >> 8;
    if ( v41 >> 8 )
    {
      v45 = 5;
      do
      {
        v45 *= 2LL;
        v44 >>= 2;
      }
      while ( v44 );
LABEL_29:
      v46 = 0;
      while ( 1 )
      {
        v47 = *(long long *)(v43 + 8 * v46);
        if ( !v47 )
          goto LABEL_31;
        v48 = *(long long *)(v27 + 8 * v47);
        if ( v48 )
        {
          if ( v48 <= v45 )
            goto LABEL_31;
          v42[v46++] = 2;
          if ( v46 == v41 )
            goto LABEL_36;
        }
        else
        {
          v42[v46] = 1;
LABEL_31:
          if ( ++v46 == v41 )
            goto LABEL_36;
        }
      }
    }
    if ( v41 )
    {
      v45 = 5;
      goto LABEL_29;
    }
LABEL_36:
    v37 += 38;
    ++v39;
    if ( (long long *)(v1 + 856) == v37 )
      break;
    v27 = *v38;
    v41 = *(v37 - 6);
    --v38;
    v42 = *v39;
  }
  v136 = (char **)&v153;
  v49 = v40;
  v134 = 0;
  while ( 1 )
  {
    v50 = *(long long *)(v1 + v134 + 200);
    for ( j = 0; v50 > j; ++j )
    {
      while ( 1 )
      {
        v52 = &v49[j];
        v53 = v49[j];
        if ( v53 != 2 )
          break;
        ++j;
        *v52 = 0;
        if ( v50 <= j )
          goto LABEL_44;
      }
      if ( !v53 )
        continue;
      v60 = j;
      for ( k = 0; ; k += v62 == 2 )
      {
        if ( v50 > ++v60 )
        {
          v62 = v49[v60];
          if ( v62 )
            continue;
        }
        break;
      }
      if ( j < v60 )
      {
        while ( v49[v60 - 1] == 2 )
        {
          --v60;
          --k;
          v49[v60] = 0;
          if ( j == v60 )
          {
            v63 = 0;
            if ( k > 0 )
              goto LABEL_41;
            goto LABEL_101;
          }
        }
        v63 = v60 - j;
        if ( v60 - j < 4 * k )
        {
          if ( j < v60 )
          {
            v64 = &v49[v60 - 1];
            do
            {
              if ( *v64 == 2 )
                *v64 = 0;
              --v64;
            }
            while ( &v49[j - 1] != v64 );
          }
          continue;
        }
      }
      else
      {
        v63 = v60 - j;
        if ( v60 - j < 4 * k )
          continue;
      }
      v65 = v63 >> 4;
      if ( v63 >> 4 <= 0 )
      {
LABEL_101:
        v67 = j + v63 - 1;
        if ( v63 <= 0 )
          goto LABEL_98;
        v66 = 1;
      }
      else
      {
        v66 = 1;
        do
        {
          v66 *= 2;
          v65 >>= 2;
        }
        while ( v65 );
        v67 = j + v63 - 1;
      }
      v68 = 0;
      v69 = 0;
      do
      {
        while ( 1 )
        {
          v70 = &v49[j + v69];
          if ( *v70 != 2 )
          {
            v68 = 0;
            goto LABEL_77;
          }
          v71 = v68 <= v66;
          if ( v68 == v66 )
          {
            v69 -= ++v68;
            goto LABEL_77;
          }
          ++v68;
          if ( !v71 )
            break;
LABEL_77:
          if ( ++v69 >= v63 )
            goto LABEL_82;
        }
        ++v69;
        *v70 = 0;
      }
      while ( v69 < v63 );
LABEL_82:
      v72 = &v49[j];
      v73 = &v52[v63];
      v74 = 0;
      while ( 1 )
      {
        v75 = *v72;
        if ( v72 - v52 > 7 && v75 == 1 )
          break;
        if ( v75 == 2 )
        {
          *v72++ = 0;
          v74 = 0;
          if ( v73 == v72 )
            break;
        }
        else
        {
          if ( v75 )
          {
            if ( ++v74 == 3 )
              break;
          }
          else
          {
            v74 = 0;
          }
          if ( v73 == ++v72 )
            break;
        }
      }
      v76 = &v49[v67];
      v77 = &v49[v67 - v63];
      v78 = 0;
      while ( &v49[v67] - v76 <= 7 || *v76 != 1 )
      {
        if ( *v76 == 2 )
        {
          *v76-- = 0;
          v78 = 0;
          if ( v77 == v76 )
            break;
        }
        else
        {
          if ( *v76 )
          {
            if ( ++v78 == 3 )
              break;
          }
          else
          {
            v78 = 0;
          }
          if ( v77 == --v76 )
            break;
        }
      }
LABEL_98:
      j = v67;
LABEL_41:
      ;
    }
LABEL_44:
    v134 += 304;
    ++v136;
    if ( v134 == 608 )
      break;
    v49 = *v136;
  }
  v54 = (long long *)(v1 + 200);
  for ( m = v40; ; m = *v35 )
  {
    v56 = *v54;
    v57 = 0;
    if ( *v54 > 0 )
    {
      v58 = 0;
      do
      {
        while ( !byte_26408 && m[v58] )
        {
          *(char *)(v54[10] + v58++) = 1;
          if ( v56 == v58 )
            goto LABEL_52;
        }
        v59 = 8 * v57;
        *(long long *)(v54[7] + 8 * v57++) = *(long long *)(v54[6] + 8 * v58);
        *(long long *)(v54[8] + v59) = v58++;
      }
      while ( v56 != v58 );
    }
LABEL_52:
    v54[9] = v57;
    v54 += 38;
    ++v35;
    if ( (long long *)(v1 + 808) == v54 )
      break;
  }
  free(v40);
  free(v26);
  v79 = *(long long *)(v1 + 272) + *(long long *)(v1 + 576) + 3LL;
  v148[0] = *(long long *)(v1 + 256);
  v148[1] = *(long long *)(v1 + 560);
  v80 = sub_16A40(16 * v79);
  v81 = *(long long *)(v1 + 576);
  v82 = 8 * v81 + 8;
  v149 = v80 + v82;
  v150 = v82 + 8 * v79 + v80;
  v151 = byte_26418;
  if ( v79 )
  {
    v83 = 1;
    do
    {
      v83 *= 2;
      v79 >>= 2;
    }
    while ( v79 );
    if ( v83 < 4096 )
      v83 = 4096;
  }
  else
  {
    v83 = 4096;
  }
  v152 = v83;
  v84 = 0;
  qmemcpy(&unk_261A0, (const void *)v1, 0x130u);
  qmemcpy(&unk_262D0, (const void *)(v1 + 304), 0x130u);
  sub_6360(0, *(long long *)(v1 + 272), 0, v81, (unsigned char)byte_26408, v148);
  free((void *)(v149 + -8 - 8LL * *(long long *)(v1 + 576)));
  do
  {
    v85 = *(long long *)(v1 + v84 + 280);
    v86 = 0;
    v87 = 0;
    v88 = *(long long *)(v1 + v84 + 248);
    v89 = *(long long *)(v1 + v84 + 200);
    v90 = *(long long *)(v1 - v84 + 584);
    while ( 1 )
    {
LABEL_112:
      v91 = v89 == v87;
      if ( v89 <= v87 )
        goto LABEL_143;
LABEL_113:
      if ( *(char *)(v85 + v87) )
        break;
      do
        ++v86;
      while ( *(char *)(v90 + v86 - 1) );
      ++v87;
    }
    do
    {
      v92 = v87;
      do
        ++v92;
      while ( *(char *)(v85 + v92) );
      while ( *(char *)(v90 + v86) )
        ++v86;
      v93 = v87;
      do
      {
        v94 = v92 - v93;
        while ( v93 )
        {
          v95 = v93;
          if ( *(long long *)(v88 + 8 * v93 - 8) != *(long long *)(v88 + 8 * v92 - 8) )
            break;
          --v93;
          --v92;
          *(char *)(v85 + v93) = 1;
          *(char *)(v85 + v92) = 0;
          if ( *(char *)(v85 + v95 - 2) )
          {
            do
              --v93;
            while ( *(char *)(v85 + v93 - 1) );
          }
          do
            --v86;
          while ( *(char *)(v90 + v86) );
        }
        v87 = v92;
        if ( !*(char *)(v90 + v86 - 1) )
          v87 = v89;
        if ( v92 != v89 )
        {
          while ( 1 )
          {
            v96 = v92;
            if ( *(long long *)(v88 + 8 * v93) != *(long long *)(v88 + 8 * v92) )
              goto LABEL_136;
            ++v93;
            ++v92;
            *(char *)(v85 + v93 - 1) = 0;
            *(char *)(v85 + v96) = 1;
            if ( *(char *)(v85 + v92) )
              break;
            while ( 1 )
            {
              ++v86;
              if ( !*(char *)(v90 + v86) )
                break;
LABEL_133:
              v87 = v92;
            }
LABEL_135:
            if ( v92 == v89 )
              goto LABEL_136;
          }
          do
            ++v92;
          while ( *(char *)(v85 + v92) );
          ++v86;
          if ( !*(char *)(v90 + v86) )
            goto LABEL_135;
          goto LABEL_133;
        }
LABEL_136:
        ;
      }
      while ( v94 != v92 - v93 );
      if ( v92 <= v87 )
      {
        v87 = v92;
        goto LABEL_112;
      }
      v97 = (char *)(v85 + v92 - 1);
      do
      {
        v97[v93 - v92] = 1;
        *v97 = 0;
        do
          --v86;
        while ( *(char *)(v90 + v86) );
        --v97;
      }
      while ( (char *)(v85 - 1 + v87) != v97 );
      v91 = v89 == v87;
      if ( v89 > v87 )
        goto LABEL_113;
LABEL_143:
      ;
    }
    while ( !v91 );
    v84 += 304;
  }
  while ( v84 != 608 );
  v98 = *(long long *)(v1 + 280);
  v99 = *(long long *)(v1 + 584);
  v100 = *(long long *)(v1 + 200);
  v101 = *(long long *)(v1 + 504);
  if ( dword_26580 == 4 )
  {
    if ( v100 <= 0 && v101 <= 0 )
      goto LABEL_204;
    v135 = v1;
    v118 = 0;
    v119 = 0;
    v120 = 0;
    v121 = *(long long *)(v1 + 200);
    v122 = *(long long *)(v1 + 504);
    do
    {
      v123 = *(char *)(v99 + v118);
      if ( v123 | *(char *)(v98 + v119) )
      {
        v126 = v119;
        if ( *(char *)(v98 + v119) )
        {
          do
            ++v126;
          while ( *(char *)(v98 + v126) );
          v127 = v126 - v119;
        }
        else
        {
          v127 = 0;
        }
        v128 = v118;
        if ( v123 )
        {
          do
            ++v128;
          while ( *(char *)(v99 + v128) );
          v129 = v128 - v118;
        }
        else
        {
          v129 = 0;
        }
        v143 = v99;
        v142 = v122;
        v141 = v121;
        v139 = v120;
        v130 = (char *)sub_16A40(48);
        v99 = v143;
        *((long long *)v130 + 3) = v119;
        v119 = v126;
        *((long long *)v130 + 4) = v118;
        v122 = v142;
        v118 = v128;
        *(long long *)v130 = v139;
        v121 = v141;
        v120 = v130;
        *((long long *)v130 + 1) = v129;
        *((long long *)v130 + 2) = v127;
      }
      ++v119;
      ++v118;
    }
    while ( v119 < v121 || v118 < v122 );
    v1 = v135;
    fastmap = v120;
LABEL_151:
    if ( !byte_26554 && !stru_264C0.fastmap )
      goto LABEL_220;
    if ( !fastmap )
      goto LABEL_206;
    v137 = v1;
    v104 = fastmap;
    do
    {
      v105 = (char **)sub_F060(v104);
      v106 = v104;
      v104 = *v105;
      *v105 = 0;
      v107 = v105;
      v108 = sub_F670(v106, v144, v145, v146, v147);
      *v107 = v104;
    }
    while ( v104 && !v108 );
    v1 = v137;
    v12 = v108 != 0;
  }
  else
  {
    if ( (v101 & v100) >= 0 )
    {
      fastmap = 0;
      do
      {
        v103 = *(char *)(v99 + v101 - 1);
        if ( v103 | *(char *)(v98 + v100 - 1) )
        {
          if ( *(char *)(v98 + v100 - 1) )
          {
            v110 = v100;
            do
              --v110;
            while ( *(char *)(v98 + v110 - 1) );
            v111 = v100 - v110;
            v100 = v110;
          }
          else
          {
            v111 = 0;
          }
          if ( v103 )
          {
            v112 = v101;
            do
              --v112;
            while ( *(char *)(v99 + v112 - 1) );
            v113 = v101;
            v101 = v112;
            v114 = v113 - v112;
          }
          else
          {
            v114 = 0;
          }
          v138 = v99;
          v115 = (char *)sub_16A40(48);
          v99 = v138;
          *(long long *)v115 = fastmap;
          fastmap = v115;
          *((long long *)v115 + 3) = v100;
          *((long long *)v115 + 4) = v101;
          *((long long *)v115 + 1) = v114;
          *((long long *)v115 + 2) = v111;
        }
        --v101;
        --v100;
      }
      while ( v100 >= 0 || v101 >= 0 );
      goto LABEL_151;
    }
LABEL_204:
    if ( byte_26554 || (fastmap = stru_264C0.fastmap) != 0 )
    {
LABEL_206:
      fastmap = 0;
      v12 = 0;
      if ( !byte_264A1 )
      {
LABEL_207:
        v12 = 0;
        if ( !byte_26578 )
          goto LABEL_179;
      }
      goto LABEL_161;
    }
LABEL_220:
    v12 = fastmap != 0;
  }
  if ( byte_264A1 )
  {
    if ( v12 )
      sub_6D90(1, v1);
  }
  else
  {
    if ( !v12 )
      goto LABEL_207;
LABEL_179:
    v116 = qword_26548;
    if ( !qword_26548 )
      v116 = *(long long *)(v1 + 312);
    v117 = qword_26540;
    if ( !qword_26540 )
      v117 = *(long long *)(v1 + 8);
    sub_E800(v117, v116, *(long long *)(v1 + 608) != 0);
    switch ( dword_26580 )
    {
      case 1:
        sub_CFF0(fastmap);
        break;
      case 2:
        sub_8C10(fastmap, 0);
        break;
      case 3:
        sub_8C10(fastmap, 1);
        break;
      case 4:
        sub_AC20(fastmap);
        break;
      case 5:
        sub_AC40(fastmap);
        break;
      case 6:
        sub_AC60(fastmap);
        break;
      case 7:
        sub_B6C0(fastmap);
        break;
      case 8:
        sub_DA50(fastmap);
        break;
      default:
        abort();
    }
    sub_EAD0();
  }
LABEL_161:
  free(*(void **)(v1 + 256));
  free(ptr);
  free(*(void **)(v1 + 248));
  free((void *)(*(long long *)(v1 + 184) + 8LL * *(long long *)(v1 + 192)));
  free(*(void **)(v1 + 552));
  free((void *)(*(long long *)(v1 + 488) + 8LL * *(long long *)(v1 + 496)));
  while ( fastmap )
  {
    v109 = fastmap;
    fastmap = *(char **)fastmap;
    free(v109);
  }
  if ( (unsigned int)(dword_26580 - 4) <= 1 )
  {
    v124 = (char *)&qword_26540;
    v125 = (char *)(v1 + 288);
    do
    {
      if ( *v125 )
      {
        v131 = dcgettext(0, "No newline at end of file", 5);
        v132 = *(const char **)v124;
        if ( !*(long long *)v124 )
          v132 = (const char *)*((long long *)v125 - 35);
        error(0, 0, "%s: %s\n", v132, v131);
        v12 = 2;
      }
      v124 += 8;
      v125 += 304;
    }
    while ( &byte_26550 != v124 );
  }
  v5 = *(void **)(v1 + 160);
  v14 = *(void **)(v1 + 464);
LABEL_16:
  if ( v5 != v14 )
  {
    free(v5);
    v14 = *(void **)(v1 + 464);
  }
  free(v14);
  return v12;
}



// Function: print_context_label @ 0x7cf0
int print_context_label(const char *a1, long long a2, const char *a3, const char *a4)
{
  char *IO_write_ptr; // rax
  long long v7; // rax
  struct tm *v8; // rax
  long long v9; // r14
  char v11[56]; // [rsp+0h] [rbp-68h] BYREF
  unsigned long long v12; // [rsp+38h] [rbp-30h]

  v12 = __readfsqword(0x28u);
  sub_F510(0);
  if ( a4 )
  {
    __fprintf_chk(stream, 1, "%s %s", a1, a4);
  }
  else
  {
    v8 = localtime((const time_t *)(a2 + 104));
    v9 = *(long long *)(a2 + 112);
    if ( !v8 || !sub_14460(v11, 43, qword_26400, v8, 0, (unsigned int)v9) )
      __sprintf_chk(v11, 1, 43, "%ld.%.9d", *(long long *)(a2 + 104), v9);
    __fprintf_chk(stream, 1, "%s %s\t%s", a1, a3, v11);
  }
  sub_F510(3);
  IO_write_ptr = stream->_IO_write_ptr;
  if ( IO_write_ptr >= stream->_IO_write_end )
  {
    LODWORD(v7) = __overflow(stream, 10);
  }
  else
  {
    stream->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = 10;
    return v12 - __readfsqword(0x28u);
  }
  return v7;
}



// Function: find_function @ 0x7e50
long long find_function(long long a1, long long a2)
{
  long long v2; // rbp
  long long v3; // rbx
  long long v4; // r12
  unsigned long long v5; // r8
  long long v6; // r12

  v2 = qword_26178;
  v3 = a2;
  qword_26178 = a2;
  while ( v2 <= --v3 )
  {
    v4 = *(long long *)(a1 + 8 * v3);
    v5 = *(long long *)(a1 + 8 * v3 + 8) + ~v4;
    if ( v5 > 0x7FFFFFFF )
      LODWORD(v5) = 0x7FFFFFFF;
    if ( re_search(&buffer, *(const char **)(a1 + 8 * v3), v5, 0, v5, 0) >= 0 )
    {
      qword_26170 = v3;
      return v4;
    }
  }
  v6 = 0;
  if ( qword_26170 != 0x7FFFFFFFFFFFFFFFLL )
    return *(long long *)(a1 + 8 * qword_26170);
  return v6;
}



// Function: print_context_function @ 0x7f00
size_t print_context_function(FILE *stream, _IO_FILE *a2)
{
  char *IO_write_ptr; // rax
  _IO_FILE *v5; // rdx
  int i; // ecx
  char flags; // al
  _IO_FILE *v8; // rdi
  char *v9; // rsi
  int v10; // edx
  char v11; // al
  char *v12; // rsi
  char v13; // al

  IO_write_ptr = stream->_IO_write_ptr;
  if ( stream->_IO_write_end <= IO_write_ptr )
  {
    __overflow(stream, 32);
  }
  else
  {
    stream->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = 32;
  }
  v5 = a2;
  for ( i = 0; ; ++i )
  {
    flags = v5->_flags;
    v8 = v5;
    if ( SLOBYTE(v5->_flags) <= 13 )
      break;
    if ( flags != 32 )
      goto LABEL_6;
    v5 = (_IO_FILE *)((char *)v5 + 1);
LABEL_19:
    ;
  }
  if ( flags > 8 )
  {
    v5 = (_IO_FILE *)((char *)v5 + 1);
    if ( flags != 10 )
      goto LABEL_19;
  }
LABEL_6:
  v9 = (char *)&a2->_flags + i + 1;
  v10 = i;
  if ( flags != 10 )
  {
    do
    {
      if ( ++v10 > i + 39 )
        break;
      v11 = *v9++;
    }
    while ( v11 != 10 );
  }
  if ( i < v10 )
  {
    v12 = (char *)a2 + v10 - 1;
    do
    {
      v13 = *v12;
      if ( *v12 > 13 )
      {
        if ( v13 != 32 )
          return fwrite_unlocked(v8, 1u, v10 - i, stream);
      }
      else if ( v13 <= 8 )
      {
        return fwrite_unlocked(v8, 1u, v10 - i, stream);
      }
      --v10;
      --v12;
    }
    while ( i != v10 );
  }
  return fwrite_unlocked(v8, 1u, v10 - i, stream);
}



// Function: print_context_number_range @ 0x7fe0
unsigned long long print_context_number_range(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+8h] [rbp-20h] BYREF
  long long v5; // [rsp+10h] [rbp-18h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  sub_F5E0(a1, a2, a3, &v4, &v5);
  if ( v5 > v4 )
    __fprintf_chk(stream, 1, "%ld,%ld", v4, v5);
  else
    __fprintf_chk(stream, 1, "%ld", v5);
  return v6 - __readfsqword(0x28u);
}



// Function: pr_context_hunk @ 0x8070
unsigned long long pr_context_hunk(long long *a1)
{
  long long *v1; // rbx
  int v2; // eax
  char v3; // r12
  long long v4; // rsi
  long long v5; // rdx
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // rdx
  long long v9; // rdi
  _IO_FILE *v10; // r13
  long long v11; // rdx
  long long v12; // rcx
  _IO_FILE *v13; // rbp
  char *IO_write_ptr; // rax
  char *v15; // rax
  long long v16; // r15
  long long *v17; // r14
  long long v18; // rax
  const char *v19; // rdi
  char *v20; // rax
  long long i; // r15
  long long v22; // rax
  const char *v23; // rdi
  char *v25; // rax
  char *v26; // rax
  long long v27; // [rsp+8h] [rbp-60h] BYREF
  long long v28; // [rsp+10h] [rbp-58h] BYREF
  long long v29; // [rsp+18h] [rbp-50h] BYREF
  long long v30; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v31; // [rsp+28h] [rbp-40h]

  v1 = a1;
  v31 = __readfsqword(0x28u);
  v2 = sub_F670(a1, &v27, &v28, &v29, &v30);
  if ( v2 )
  {
    v3 = v2;
    v4 = v27 - qword_26570;
    if ( v27 - qword_26570 < -qword_26288 )
      v4 = -qword_26288;
    v5 = v29 - qword_26570;
    v27 = v4;
    if ( v29 - qword_26570 < -qword_26288 )
      v5 = -qword_26288;
    v29 = v5;
    v6 = qword_26270 - 1;
    v7 = v30;
    if ( qword_26270 - qword_26570 > v28 )
      v6 = qword_26570 + v28;
    v28 = v6;
    v8 = qword_263A0 - 1;
    if ( qword_263A0 - qword_26570 > v30 )
      v8 = v30 + qword_26570;
    v30 = v8;
    if ( buffer.fastmap )
    {
      v9 = qword_26258;
      v10 = (_IO_FILE *)sub_7E50(qword_26258, v4);
      sub_E830(v9, v4, v11, v12);
      v13 = stream;
      fputs_unlocked("***************", stream);
      if ( v10 )
        sub_7F00(v13, v10);
    }
    else
    {
      sub_E830(qword_263A0 - qword_26570, v4, v8, v7);
      v13 = stream;
      fputs_unlocked("***************", stream);
    }
    IO_write_ptr = v13->_IO_write_ptr;
    if ( IO_write_ptr >= v13->_IO_write_end )
    {
      __overflow(v13, 10);
    }
    else
    {
      v13->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 10;
    }
    sub_F510(4);
    fputs_unlocked("*** ", v13);
    sub_7FE0((long long)&unk_261A0, v27, v28);
    fputs_unlocked(" ****", v13);
    sub_F510(3);
    v15 = v13->_IO_write_ptr;
    if ( v15 >= v13->_IO_write_end )
    {
      __overflow(v13, 10);
    }
    else
    {
      v13->_IO_write_ptr = v15 + 1;
      *v15 = 10;
    }
    if ( (v3 & 1) != 0 )
    {
      v16 = v27;
      if ( v27 <= v28 )
      {
        v17 = v1;
        do
        {
          sub_F510(2);
          if ( !v17 )
            goto LABEL_45;
          while ( 1 )
          {
            v18 = v17[3];
            if ( v18 + v17[2] > v16 )
              break;
            v17 = (long long *)*v17;
            if ( !v17 )
              goto LABEL_45;
          }
          if ( v18 <= v16 )
          {
            v19 = "-";
            if ( (long long)v17[1] > 0 )
              v19 = "!";
          }
          else
          {
LABEL_45:
            v19 = " ";
          }
          sub_F330(v19, 8 * v16 + qword_26258, 1);
          sub_F510(3);
          if ( *(char *)(*(long long *)(qword_26258 + 8 * v16 + 8) - 1LL) == 10 )
          {
            v25 = v13->_IO_write_ptr;
            if ( v25 >= v13->_IO_write_end )
            {
              __overflow(v13, 10);
            }
            else
            {
              v13->_IO_write_ptr = v25 + 1;
              *v25 = 10;
            }
          }
          ++v16;
        }
        while ( v28 >= v16 );
      }
    }
    sub_F510(4);
    fputs_unlocked("--- ", v13);
    sub_7FE0((long long)&unk_262D0, v29, v30);
    fputs_unlocked(" ----", v13);
    sub_F510(3);
    v20 = v13->_IO_write_ptr;
    if ( v20 >= v13->_IO_write_end )
    {
      __overflow(v13, 10);
    }
    else
    {
      v13->_IO_write_ptr = v20 + 1;
      *v20 = 10;
    }
    if ( (v3 & 2) != 0 )
    {
      for ( i = v29; v30 >= i; ++i )
      {
        sub_F510(1);
        if ( !v1 )
          goto LABEL_44;
        while ( 1 )
        {
          v22 = v1[4];
          if ( v22 + v1[1] > i )
            break;
          v1 = (long long *)*v1;
          if ( !v1 )
            goto LABEL_44;
        }
        if ( v22 <= i )
        {
          v23 = "+";
          if ( (long long)v1[2] > 0 )
            v23 = "!";
        }
        else
        {
LABEL_44:
          v23 = " ";
        }
        sub_F330(v23, 8 * i + qword_26388, 1);
        sub_F510(3);
        if ( *(char *)(*(long long *)(qword_26388 + 8 * i + 8) - 1LL) == 10 )
        {
          v26 = v13->_IO_write_ptr;
          if ( v26 >= v13->_IO_write_end )
          {
            __overflow(v13, 10);
          }
          else
          {
            v13->_IO_write_ptr = v26 + 1;
            *v26 = 10;
          }
        }
      }
    }
  }
  return v31 - __readfsqword(0x28u);
}



// Function: print_unidiff_number_range @ 0x84f0
unsigned long long print_unidiff_number_range(long long a1, long long a2, long long a3)
{
  const char *v3; // rdx
  long long v5; // [rsp+8h] [rbp-20h] BYREF
  long long v6; // [rsp+10h] [rbp-18h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-10h]

  v7 = __readfsqword(0x28u);
  sub_F5E0(a1, a2, a3, &v5, &v6);
  if ( v6 > v5 )
  {
    __fprintf_chk(stream, 1, "%ld,%ld", v5, v6 - v5 + 1);
  }
  else
  {
    v3 = "%ld,0";
    if ( v6 >= v5 )
      v3 = "%ld";
    __fprintf_chk(stream, 1, v3);
  }
  return v7 - __readfsqword(0x28u);
}



// Function: pr_unidiff_hunk @ 0x8590
unsigned long long pr_unidiff_hunk(long long *a1)
{
  long long *v1; // r14
  long long v2; // rsi
  long long v3; // rdx
  long long v4; // rdx
  long long v5; // rcx
  long long v6; // rdx
  long long v7; // rdi
  _IO_FILE *v8; // rbp
  long long v9; // rdx
  long long v10; // rcx
  _IO_FILE *v11; // r15
  char *IO_write_ptr; // rax
  long long v13; // r13
  long long v14; // rbx
  char **v15; // rbp
  char *v16; // rax
  int v17; // esi
  long long v19; // rbx
  long long v20; // r12
  long long v21; // rbp
  char *v22; // rax
  long long v23; // rbp
  char *v24; // rax
  char *v25; // rax
  long long v26; // rbx
  long long v27; // r12
  long long v28; // rbp
  char *v29; // rax
  long long v30; // rbp
  char *v31; // rax
  char *v32; // rax
  long long v33; // [rsp+0h] [rbp-78h]
  long long v34; // [rsp+8h] [rbp-70h]
  long long v35; // [rsp+8h] [rbp-70h]
  long long v36; // [rsp+18h] [rbp-60h] BYREF
  long long v37; // [rsp+20h] [rbp-58h] BYREF
  long long v38; // [rsp+28h] [rbp-50h] BYREF
  long long v39; // [rsp+30h] [rbp-48h] BYREF
  unsigned long long v40; // [rsp+38h] [rbp-40h]

  v1 = a1;
  v40 = __readfsqword(0x28u);
  if ( (unsigned int)sub_F670(a1, &v36, &v37, &v38, &v39) )
  {
    v2 = v36 - qword_26570;
    if ( v36 - qword_26570 < -qword_26288 )
      v2 = -qword_26288;
    v3 = v38 - qword_26570;
    v36 = v2;
    if ( v38 - qword_26570 < -qword_26288 )
      v3 = -qword_26288;
    v38 = v3;
    v4 = qword_26270 - 1;
    v5 = v39;
    if ( qword_26270 - qword_26570 > v37 )
      v4 = qword_26570 + v37;
    v37 = v4;
    v6 = qword_263A0 - 1;
    if ( qword_263A0 - qword_26570 > v39 )
      v6 = v39 + qword_26570;
    v39 = v6;
    if ( buffer.fastmap )
    {
      v7 = qword_26258;
      v8 = (_IO_FILE *)sub_7E50(qword_26258, v2);
      sub_E830(v7, v2, v9, v10);
      v11 = stream;
      sub_F510(4);
      fputs_unlocked("@@ -", v11);
      sub_84F0((long long)&unk_261A0, v36, v37);
      fputs_unlocked(" +", v11);
      sub_84F0((long long)&unk_262D0, v38, v39);
      fputs_unlocked(" @@", v11);
      sub_F510(3);
      if ( v8 )
        sub_7F00(v11, v8);
    }
    else
    {
      sub_E830(qword_263A0 - qword_26570, v2, v6, v5);
      v11 = stream;
      sub_F510(4);
      fputs_unlocked("@@ -", v11);
      sub_84F0((long long)&unk_261A0, v36, v37);
      fputs_unlocked(" +", v11);
      sub_84F0((long long)&unk_262D0, v38, v39);
      fputs_unlocked(" @@", v11);
      sub_F510(3);
    }
    IO_write_ptr = v11->_IO_write_ptr;
    if ( IO_write_ptr >= v11->_IO_write_end )
    {
      __overflow(v11, 10);
    }
    else
    {
      v11->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 10;
    }
    v13 = v36;
    v33 = v38;
    while ( 1 )
    {
      if ( v37 < v13 && v39 < v33 )
        return v40 - __readfsqword(0x28u);
      while ( !v1 || v1[3] > v13 )
      {
        v14 = v13 + 1;
        v15 = (char **)(qword_26258 + 8 * v13);
        if ( !byte_26491 || **v15 != 10 )
        {
          v16 = v11->_IO_write_ptr;
          v17 = byte_26492 == 0 ? 32 : 9;
          if ( v16 >= v11->_IO_write_end )
          {
            __overflow(v11, v17);
          }
          else
          {
            v11->_IO_write_ptr = v16 + 1;
            *v16 = v17;
          }
        }
        ++v13;
        sub_F500(0, v15);
        ++v33;
        if ( v37 < v14 && v39 < v33 )
          return v40 - __readfsqword(0x28u);
      }
      v19 = v1[2];
      v34 = v19 - 1;
      if ( !v19 )
        goto LABEL_42;
      v20 = 8 * v13;
      do
      {
        while ( 1 )
        {
          v21 = qword_26258;
          sub_F510(2);
          v22 = v11->_IO_write_ptr;
          v23 = v20 + v21;
          if ( v22 >= v11->_IO_write_end )
          {
            __overflow(v11, 45);
          }
          else
          {
            v11->_IO_write_ptr = v22 + 1;
            *v22 = 45;
          }
          if ( byte_26492 && (!byte_26491 || **(char **)v23 != 10) )
          {
            v24 = v11->_IO_write_ptr;
            if ( v24 >= v11->_IO_write_end )
            {
              __overflow(v11, 9);
            }
            else
            {
              v11->_IO_write_ptr = v24 + 1;
              *v24 = 9;
            }
          }
          sub_F330(0, v23, 1);
          sub_F510(3);
          if ( *(char *)(*(long long *)(v23 + 8) - 1LL) != 10 )
            goto LABEL_30;
          v25 = v11->_IO_write_ptr;
          if ( v25 < v11->_IO_write_end )
            break;
          __overflow(v11, 10);
LABEL_30:
          v20 += 8;
          if ( !--v19 )
            goto LABEL_41;
        }
        v20 += 8;
        v11->_IO_write_ptr = v25 + 1;
        *v25 = 10;
        --v19;
      }
      while ( v19 );
LABEL_41:
      v13 += v34 + 1;
LABEL_42:
      v26 = v1[1];
      v35 = v26 - 1;
      if ( !v26 )
        goto LABEL_56;
      v27 = 8 * v33;
      while ( 2 )
      {
        while ( 2 )
        {
          v28 = qword_26388;
          sub_F510(1);
          v29 = v11->_IO_write_ptr;
          v30 = v27 + v28;
          if ( v29 >= v11->_IO_write_end )
          {
            __overflow(v11, 43);
          }
          else
          {
            v11->_IO_write_ptr = v29 + 1;
            *v29 = 43;
          }
          if ( byte_26492 && (!byte_26491 || **(char **)v30 != 10) )
          {
            v31 = v11->_IO_write_ptr;
            if ( v31 >= v11->_IO_write_end )
            {
              __overflow(v11, 9);
            }
            else
            {
              v11->_IO_write_ptr = v31 + 1;
              *v31 = 9;
            }
          }
          sub_F330(0, v30, 1);
          sub_F510(3);
          if ( *(char *)(*(long long *)(v30 + 8) - 1LL) != 10 )
            goto LABEL_44;
          v32 = v11->_IO_write_ptr;
          if ( v32 >= v11->_IO_write_end )
          {
            __overflow(v11, 10);
LABEL_44:
            v27 += 8;
            if ( !--v26 )
              goto LABEL_55;
            continue;
          }
          break;
        }
        v27 += 8;
        v11->_IO_write_ptr = v32 + 1;
        *v32 = 10;
        if ( --v26 )
          continue;
        break;
      }
LABEL_55:
      v33 += v35 + 1;
LABEL_56:
      v1 = (long long *)*v1;
    }
  }
  return v40 - __readfsqword(0x28u);
}



// Function: find_hunk @ 0x8b10
long long *find_hunk(long long *a1)
{
  long long v1; // rdx
  long long v2; // rax
  long long v3; // r9
  long long v4; // rcx
  long long *v5; // r8

  v1 = a1[3];
  v2 = a1[4];
  while ( 1 )
  {
    v5 = a1;
    a1 = (long long *)*a1;
    if ( !a1 )
      break;
    v3 = 2 * qword_26570 + 1;
    if ( *((char *)a1 + 40) )
      v3 = qword_26570;
    v4 = a1[3] - (v5[2] + v1);
    if ( v4 != a1[4] - (v5[1] + v2) )
      abort();
    if ( v4 >= v3 )
      break;
    v2 = a1[4];
    v1 = a1[3];
  }
  return v5;
}



// Function: print_context_header @ 0x8b80
int print_context_header(long long a1, const char **a2, char a3)
{
  long long v4; // rbp
  const char *v5; // rdx

  v4 = a1 + 304;
  v5 = *a2;
  if ( a3 )
  {
    sub_7CF0("---", a1, v5, (const char *)qword_26540);
    return sub_7CF0("+++", v4, a2[1], (const char *)qword_26548);
  }
  else
  {
    sub_7CF0("***", a1, v5, (const char *)qword_26540);
    return sub_7CF0("---", v4, a2[1], (const char *)qword_26548);
  }
}



// Function: print_context_script @ 0x8c10
long long print_context_script(long long *a1, char a2)
{
  long long *v2; // rbx
  long long *v3; // r14
  int v4; // eax
  unsigned long long (*v5)(long long *); // rdx
  long long *i; // rax
  long long v8; // rax
  char v9[8]; // [rsp+18h] [rbp-60h] BYREF
  char v10[8]; // [rsp+20h] [rbp-58h] BYREF
  char v11[8]; // [rsp+28h] [rbp-50h] BYREF
  char v12[8]; // [rsp+30h] [rbp-48h] BYREF
  unsigned long long v13; // [rsp+38h] [rbp-40h]

  v13 = __readfsqword(0x28u);
  if ( byte_26554 || stru_264C0.fastmap )
  {
    if ( a1 )
    {
      v2 = a1;
      do
      {
        v3 = v2;
        v2 = (long long *)*v2;
        *v3 = 0;
        v4 = sub_F670(v3, v9, v10, v11, v12);
        *v3 = v2;
        *((char *)v3 + 40) = v4 == 0;
      }
      while ( v2 );
    }
  }
  else
  {
    for ( i = a1; i; i = *(long long **)v8 )
    {
      *((char *)i + 40) = 0;
      v8 = *i;
      if ( !v8 )
        break;
      *(char *)(v8 + 40) = 0;
    }
  }
  qword_26178 = -qword_26288;
  qword_26170 = 0x7FFFFFFFFFFFFFFFLL;
  if ( a2 )
    v5 = sub_8590;
  else
    v5 = sub_8070;
  return sub_F080(a1, sub_8B10, v5);
}



// Function: add_regexp @ 0x8d60
void add_regexp(long long a1, const char *a2)
{
  size_t v3; // rbp
  const char *v4; // rax
  long long v5; // r14
  long long v6; // r13
  unsigned long long v7; // rsi
  bool v8; // al
  char v9; // dl
  long long v10; // rax
  unsigned long long v11; // rdx
  long long v12; // rdx

  v3 = strlen(a2);
  v4 = re_compile_pattern(a2, v3, *(struct re_pattern_buffer **)(a1 + 32));
  if ( v4 )
  {
    error(2, 0, "%s: %s", a2, v4);
  }
  else
  {
    v5 = *(long long *)a1;
    v6 = *(long long *)(a1 + 8);
    v7 = *(long long *)(a1 + 16);
    v8 = *(long long *)a1 != 0;
    v9 = 2 * v8;
    *(char *)(a1 + 24) = v8;
    v10 = v5;
    v11 = v3 + v6 + (v9 & 2);
    *(long long *)(a1 + 8) = v11;
    if ( v11 >= v7 )
    {
      if ( !v7 )
        v7 = 1;
      do
        v7 *= 2LL;
      while ( v11 >= v7 );
      *(long long *)(a1 + 16) = v7;
      v10 = sub_16A90(v5, v7);
      *(long long *)a1 = v10;
    }
    if ( v5 )
    {
      v12 = v6 + 1;
      *(char *)(v10 + v6) = 92;
      v6 += 2;
      *(char *)(v10 + v12) = 124;
    }
    memcpy((void *)(v10 + v6), a2, v3 + 1);
  }
}



// Function: try_help @ 0x8e40
long long try_help(char *msgid, long long a2)
{
  char *v2; // rax
  long long v3; // r12
  char *v4; // rax

  if ( msgid )
  {
    v2 = dcgettext(0, msgid, 5);
    error(0, 0, v2, a2);
  }
  v3 = qword_36860;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_8EA0(2);
}



// Function: compare_files @ 0x8f90
long long compare_files(long long a1, const char *a2, long long a3, long long a4, int *a5)
{
  long long v5; // r13
  const char *v6; // r12
  long long v7; // rbx
  bool v8; // dl
  __m128i *v9; // rbp
  char *v10; // r14
  long long v11; // rax
  long long v12; // rdi
  struct timespec *v13; // r15
  int i; // r13d
  __m128i si128; // xmm1
  __m128i v16; // xmm2
  __m128i v17; // xmm3
  __m128i v18; // xmm4
  __m128i v19; // xmm5
  __m128i v20; // xmm6
  __m128i v21; // xmm7
  __m128i v22; // xmm0
  char v23; // di
  __m128i *v24; // rax
  __m128i *v25; // rsi
  unsigned int v26; // r15d
  char v27; // r9
  const char *v28; // rsi
  int v29; // edx
  int *v30; // rax
  long long v31; // rdi
  int *v32; // rbx
  long long v33; // rcx
  long long v34; // rcx
  long long v35; // rdx
  const char *tv_sec; // rdi
  struct stat *p_tv_nsec; // rsi
  bool v39; // al
  int v40; // r14d
  long long v41; // rax
  struct stat *v42; // rsi
  bool v43; // al
  long long v44; // rdx
  long long v45; // rcx
  long long v46; // rdx
  short v47; // di
  int v48; // eax
  long long v49; // r9
  long long v50; // rcx
  int v51; // edx
  unsigned int v52; // edi
  bool v53; // di
  long long v54; // rdx
  long long v55; // rsi
  int *v56; // rax
  long long v57; // rdx
  long long v58; // rcx
  int v59; // ebx
  int v60; // edi
  int v61; // edi
  long long v62; // rdx
  long long v63; // rcx
  __off_t v64; // rax
  long long tv_nsec; // rax
  long long v66; // rdx
  long long v67; // rcx
  const char *v68; // rax
  char *v69; // r12
  int v70; // eax
  long long v71; // rdx
  long long v72; // rcx
  int v73; // eax
  long long v74; // rdx
  long long v75; // rdx
  long long v76; // rcx
  const char *v77; // rdx
  const char *v78; // rsi
  char *v79; // rax
  void *ptr; // [rsp+8h] [rbp-2C0h]
  char *file; // [rsp+10h] [rbp-2B8h]
  unsigned int fileb; // [rsp+10h] [rbp-2B8h]
  unsigned int filea; // [rsp+10h] [rbp-2B8h]
  char *v84; // [rsp+18h] [rbp-2B0h]
  __m128i v85[38]; // [rsp+20h] [rbp-2A8h] BYREF
  long long v86; // [rsp+280h] [rbp-48h]
  unsigned long long v87; // [rsp+288h] [rbp-40h]

  v5 = a3;
  v6 = a2;
  v7 = a1;
  v87 = __readfsqword(0x28u);
  v8 = a3 == 0;
  if ( a2 && !v8 )
  {
    v9 = v85;
    v86 = a1;
    memset(v85, 0, sizeof(v85));
    v85[0].m128i_i32[0] = -2;
    goto LABEL_10;
  }
  if ( byte_26585 == 1 && !v8 )
  {
    v9 = v85;
    v86 = a1;
    memset(v85, 0, sizeof(v85));
    if ( a2 )
    {
      v85[0].m128i_i32[0] = -2;
      v85[19].m128i_i32[0] = -2;
      goto LABEL_7;
    }
    v85[0].m128i_i32[0] = -1;
    goto LABEL_82;
  }
  if ( !byte_26586 )
  {
    v26 = 1;
    if ( !a2 )
      v6 = (const char *)v5;
    sub_E660("Only in %s: %s\n", *(long long *)(a1 + 304LL * (a2 == 0) + 8), v6);
    return v26;
  }
  v9 = v85;
  v86 = a1;
  memset(v85, 0, sizeof(v85));
  if ( a2 )
  {
    v85[0].m128i_i32[0] = -2;
    if ( !v5 )
    {
      v85[19].m128i_i32[0] = -1;
      v5 = (long long)a2;
      goto LABEL_11;
    }
LABEL_10:
    v85[19].m128i_i32[0] = -2;
    goto LABEL_11;
  }
  v85[0].m128i_i32[0] = -1;
  if ( !v5 )
  {
    v85[19].m128i_i32[0] = -1;
LABEL_80:
    v6 = 0;
LABEL_11:
    if ( !a1 )
      goto LABEL_8;
LABEL_12:
    v11 = sub_10D80(*(long long *)(a1 + 8), v6, 0);
    v12 = *(long long *)(a1 + 312);
    v85[0].m128i_i64[1] = v11;
    v10 = (char *)v11;
    ptr = (void *)sub_10D80(v12, v5, 0);
    v5 = (long long)ptr;
    goto LABEL_13;
  }
LABEL_82:
  v85[19].m128i_i32[0] = -2;
  if ( !v5 )
    goto LABEL_80;
LABEL_7:
  v6 = (const char *)v5;
  if ( a1 )
    goto LABEL_12;
LABEL_8:
  v85[0].m128i_i64[1] = (long long)v6;
  v10 = 0;
  ptr = 0;
LABEL_13:
  v85[19].m128i_i64[1] = v5;
  v13 = (struct timespec *)&v85[6].m128i_u64[1];
  for ( i = 0; ; i = 1 )
  {
    if ( LODWORD(v13[-7].tv_nsec) == -1 )
      goto LABEL_18;
    if ( i && !strcmp((const char *)v85[19].m128i_i64[1], (const char *)v85[0].m128i_i64[1]) )
    {
      si128 = _mm_load_si128(&v85[2]);
      v16 = _mm_load_si128(&v85[3]);
      v17 = _mm_load_si128(&v85[4]);
      v18 = _mm_load_si128(&v85[5]);
      v85[20] = _mm_load_si128(&v85[1]);
      v19 = _mm_load_si128(&v85[6]);
      v85[19].m128i_i32[0] = v85[0].m128i_i32[0];
      v20 = _mm_load_si128(&v85[7]);
      v85[21] = si128;
      v21 = _mm_load_si128(&v85[8]);
      v22 = _mm_load_si128(&v85[9]);
      v85[22] = v16;
      v85[23] = v17;
      v85[24] = v18;
      v85[25] = v19;
      v85[26] = v20;
      v85[27] = v21;
      v85[28] = v22;
      goto LABEL_18;
    }
    tv_sec = (const char *)v13[-6].tv_sec;
    p_tv_nsec = (struct stat *)&v13[-6].tv_nsec;
    if ( !strcmp(tv_sec, "-") )
    {
      LODWORD(v13[-7].tv_nsec) = 0;
      isatty(0);
      if ( fstat(0, (struct stat *)&v13[-6].tv_nsec) )
        goto LABEL_46;
      if ( (v13[-4].tv_sec & 0xF000) == 0x8000 )
      {
        v64 = lseek(0, 0, 1);
        if ( v64 < 0 )
        {
          LODWORD(v13[-7].tv_nsec) = -3 - *__errno_location();
        }
        else
        {
          v13[-3].tv_nsec -= v64;
          tv_nsec = v13[-3].tv_nsec;
          if ( tv_nsec < 0 )
            tv_nsec = 0;
          v13[-3].tv_nsec = tv_nsec;
        }
      }
      sub_10EB0(v13);
    }
    else
    {
      if ( byte_26550 )
      {
        if ( !lstat(tv_sec, p_tv_nsec) )
          goto LABEL_18;
LABEL_46:
        a5 = __errno_location();
        LODWORD(v13[-7].tv_nsec) = -3 - *a5;
        goto LABEL_18;
      }
      if ( stat(tv_sec, p_tv_nsec) )
        goto LABEL_46;
    }
LABEL_18:
    v13 += 19;
    if ( i == 1 )
      break;
  }
  v23 = byte_26586;
  v24 = v85;
  v25 = v85;
  v26 = 0;
  v27 = byte_26585;
  while ( 1 )
  {
    ++v26;
    if ( !v23 )
    {
      if ( v26 != 1 )
        goto LABEL_22;
      if ( !v27 )
        goto LABEL_41;
    }
    if ( v24->m128i_i32[0] == -2 )
      break;
    if ( (v24->m128i_i32[0] == -12 || v24->m128i_i32[0] == -5) && !v7 && ((v25[19].m128i_i32[0] + 2) & 0xFFFFFFFD) == 0 )
      goto LABEL_40;
LABEL_22:
    if ( v26 == 2 )
      goto LABEL_23;
LABEL_41:
    v24 += 19;
    v25 -= 19;
  }
  v51 = v24[2].m128i_i32[2];
  if ( (v51 & 0xF000) != 0x8000 || (v51 & 0x1FF) != 0 || v24[4].m128i_i64[0] )
    goto LABEL_22;
LABEL_40:
  v24->m128i_i32[0] = -1;
  if ( v26 != 2 )
    goto LABEL_41;
LABEL_23:
  v28 = (const char *)v85[0].m128i_u32[0];
  if ( v85[0].m128i_i32[0] == -1 )
  {
    memset(&v85[1], 0, 0x90u);
    v85[2].m128i_i32[2] = v85[21].m128i_i32[2];
    if ( v85[19].m128i_i32[0] != -1 )
      goto LABEL_48;
    memset(&v85[20], 0, 0x90u);
    v85[21].m128i_i32[2] = v85[2].m128i_i32[2];
LABEL_49:
    if ( !v7 )
    {
      v39 = (v85[2].m128i_i16[4] & 0xF000) == 0x4000;
      if ( v39 != ((v85[21].m128i_i16[4] & 0xF000) == 0x4000) )
      {
        v40 = (v85[2].m128i_i16[4] & 0xF000) != 0x4000;
        v84 = (char *)v85[19 * v39].m128i_i64[1];
        v41 = sub_FB40(v84);
        file = (char *)(304LL * v40);
        v10 = (char *)sub_A660(*(long long *)((char *)&v85[0].m128i_i64[1] + (long long)file), v41);
        *(long long *)((char *)&v85[0].m128i_i64[1] + (long long)file) = (long long)v10;
        if ( !strcmp(v84, "-") )
        {
          sub_E7B0("cannot compare '-' to a directory");
          goto LABEL_184;
        }
        v42 = (struct stat *)&v85[1].m128i_i8[(long long)file];
        if ( byte_26550 )
          v43 = lstat(v10, v42) != 0;
        else
          v43 = stat(v10, v42) != 0;
        v28 = (const char *)v85[0].m128i_u32[0];
        if ( v43 )
        {
          sub_E490(v10, v85[0].m128i_u32[0], v44, v45);
          goto LABEL_28;
        }
      }
    }
    v46 = v85[19].m128i_u32[0];
    if ( (int)v28 == -1 )
    {
      if ( v85[19].m128i_i32[0] == -1 )
        goto LABEL_93;
      v47 = v85[21].m128i_i16[4];
      v50 = 0;
      v48 = v85[2].m128i_i16[4] & 0xF000;
LABEL_86:
      v52 = v47 & 0xF000;
      v49 = v52;
      v53 = v52 == 0x4000;
      if ( v48 == 0x4000 )
      {
        if ( v53 )
        {
          if ( dword_26580 != 7 )
          {
            if ( v7 && !byte_26608 )
            {
              sub_E660(
                "Common subdirectories: %s and %s\n",
                (const char *)v85[0].m128i_i64[1],
                (const char *)v85[19].m128i_i64[1]);
              goto LABEL_93;
            }
            goto LABEL_91;
          }
LABEL_184:
          sub_E7B0("-D option not supported with directories");
          goto LABEL_185;
        }
      }
      else if ( !v53 )
      {
        goto LABEL_63;
      }
      if ( (int)v28 == -1 )
      {
LABEL_119:
        if ( !byte_26608 || !byte_26586 && (!byte_26585 || (int)v28 != -1) )
        {
          if ( !v7 )
            __assert_fail("parent", "diff.c", 0x52Du, "compare_files");
          goto LABEL_68;
        }
LABEL_91:
        LODWORD(v7) = sub_A230(v85, compare_files, v85[19].m128i_u32[0], v50, a5, v49);
        goto LABEL_92;
      }
LABEL_118:
      if ( v85[19].m128i_i32[0] != -1 )
        goto LABEL_112;
      goto LABEL_119;
    }
    v47 = v85[21].m128i_i16[4];
    if ( v85[19].m128i_i32[0] == -1 )
    {
LABEL_85:
      v50 = 0;
      v48 = v85[2].m128i_i16[4] & 0xF000;
      goto LABEL_86;
    }
    if ( *(_OWORD *)&v85[1] == *(_OWORD *)&v85[20] )
    {
LABEL_108:
      if ( v85[2].m128i_i32[2] == v85[21].m128i_i32[2] )
      {
        if ( v85[2].m128i_i64[0] == v85[21].m128i_i64[0]
          && *(long long *)((char *)&v85[2].m128i_i64[1] + 4) == *(long long *)((char *)&v85[21].m128i_i64[1] + 4)
          && v85[4].m128i_i64[0] == v85[23].m128i_i64[0]
          && v85[6].m128i_i64[1] == v85[25].m128i_i64[1]
          && v85[7].m128i_i64[1] == v85[26].m128i_i64[1] )
        {
          if ( byte_26578 )
            goto LABEL_93;
          v50 = 1;
          v48 = v85[21].m128i_i16[4] & 0xF000;
        }
        else
        {
          v50 = 0;
          v48 = v85[21].m128i_i16[4] & 0xF000;
        }
        goto LABEL_86;
      }
      goto LABEL_85;
    }
    v48 = v85[2].m128i_i16[4] & 0xF000;
    v49 = v85[21].m128i_i16[4] & 0xF000;
    if ( v48 == 24576 )
    {
      if ( (int)v49 != 24576 )
        goto LABEL_62;
    }
    else
    {
      v50 = 0;
      if ( v48 != 0x2000 )
        goto LABEL_86;
      if ( (int)v49 != 0x2000 )
        goto LABEL_62;
    }
    if ( v85[22].m128i_i64[1] == v85[3].m128i_i64[1] )
      goto LABEL_108;
LABEL_62:
    v50 = 0;
    if ( (int)v49 != 0x4000 )
    {
LABEL_63:
      if ( v7 && ((v48 & 0xFFFFDFFF) != 0x8000 || (v49 & 0xFFFFDFFF) != 0x8000) )
      {
        if ( (int)v28 == -1 || v85[19].m128i_i32[0] == -1 )
        {
LABEL_68:
          v26 = 1;
          sub_E660("Only in %s: %s\n", *(const char **)(v7 + 304LL * ((int)v28 == -1) + 8), v6);
          goto LABEL_28;
        }
LABEL_112:
        v26 = 1;
        sub_10C90(&v85[20], v28, v46, v50);
        sub_10C90(&v85[1], v28, v57, v58);
        sub_E4C0("File %s is a %s while file %s is a %s\n");
        goto LABEL_28;
      }
      LOBYTE(v46) = (int)v49 == 40960;
      if ( v48 == 40960 || (int)v49 == 40960 )
      {
        if ( !byte_26550 )
          __assert_fail("no_dereference_symlinks", "diff.c", 0x549u, "compare_files");
        if ( v48 != 40960 || (int)v49 != 40960 )
          goto LABEL_112;
        v9 = (__m128i *)sub_16F20(v85[0].m128i_i64[1], v28, v46, v50);
        if ( !v9 )
          goto LABEL_181;
        v68 = (const char *)sub_16F20(v85[19].m128i_i64[1], v28, v66, v67);
        v69 = (char *)v68;
        if ( !v68 )
          goto LABEL_182;
        if ( strcmp(v9->m128i_i8, v68) )
        {
          v26 = 1;
          sub_E660(
            "Symbolic links %s and %s differ\n",
            (const char *)v85[0].m128i_i64[1],
            (const char *)v85[19].m128i_i64[1]);
          free(v9);
          free(v69);
          goto LABEL_28;
        }
        free(v9);
        free(v69);
LABEL_93:
        v26 = 0;
        if ( byte_26584 && (v85[2].m128i_i16[4] & 0xF000) != 0x4000 )
        {
          v54 = qword_26548;
          v55 = qword_26540;
          if ( !qword_26548 )
            v54 = v85[19].m128i_i64[1];
          if ( !qword_26540 )
            v55 = v85[0].m128i_i64[1];
          sub_E660("Files %s and %s are identical\n", v55, v54);
        }
        goto LABEL_29;
      }
      if ( ((unsigned char)byte_26553 & (v48 == 0x8000)) != 0
        && (int)v49 == 0x8000
        && v85[4].m128i_i64[0] > 0
        && v85[4].m128i_i64[0] != v85[23].m128i_i64[0]
        && v85[23].m128i_i64[0] > 0 )
      {
        v77 = (const char *)qword_26548;
        if ( !qword_26548 )
          v77 = (const char *)v85[19].m128i_i64[1];
        v78 = (const char *)qword_26540;
        if ( !qword_26540 )
          v78 = (const char *)v85[0].m128i_i64[1];
        v26 = 1;
        sub_E660("Files %s and %s differ\n", v78, v77);
        goto LABEL_28;
      }
      if ( (int)v28 == -2
        && (v28 = 0,
            filea = v50,
            v73 = open((const char *)v85[0].m128i_i64[1], 0, 0),
            LODWORD(v50) = filea,
            v85[0].m128i_i32[0] = v73,
            v73 < 0) )
      {
        sub_E490(v85[0].m128i_i64[1], 0, v74, filea);
        LODWORD(v50) = filea;
        if ( v85[19].m128i_i32[0] != -2 )
          goto LABEL_135;
        v59 = 2;
      }
      else
      {
        v59 = 0;
        if ( v85[19].m128i_i32[0] != -2 )
          goto LABEL_151;
      }
      if ( (int)v50 )
      {
        v85[19].m128i_i32[0] = v85[0].m128i_i32[0];
      }
      else
      {
        v28 = 0;
        v85[19].m128i_i32[0] = open((const char *)v85[19].m128i_i64[1], 0, 0);
        if ( v85[19].m128i_i32[0] < 0 )
        {
          sub_E490(v85[19].m128i_i64[1], 0, v75, v76);
          goto LABEL_135;
        }
      }
      if ( v59 )
      {
LABEL_135:
        v60 = v85[0].m128i_i32[0];
        if ( v85[0].m128i_i32[0] < 0 )
        {
          v61 = v85[19].m128i_i32[0];
          if ( v85[19].m128i_i32[0] < 0 )
            goto LABEL_28;
          goto LABEL_137;
        }
        LODWORD(v7) = 2;
        goto LABEL_157;
      }
LABEL_151:
      v70 = sub_6E20((long long)v85);
      v60 = v85[0].m128i_i32[0];
      LODWORD(v7) = v70;
      if ( v85[0].m128i_i32[0] < 0 )
      {
LABEL_152:
        if ( v85[19].m128i_i32[0] >= 0 && v85[19].m128i_i32[0] != v85[0].m128i_i32[0] && close(v85[19].m128i_i32[0]) )
          goto LABEL_138;
LABEL_92:
        if ( !(int)v7 )
          goto LABEL_93;
LABEL_185:
        v26 = v7;
        goto LABEL_28;
      }
LABEL_157:
      if ( close(v60) )
      {
        sub_E490(v85[0].m128i_i64[1], v28, v71, v72);
        v61 = v85[19].m128i_i32[0];
        if ( v85[19].m128i_i32[0] < 0 || v85[0].m128i_i32[0] == v85[19].m128i_i32[0] )
          goto LABEL_28;
LABEL_137:
        if ( !close(v61) )
          goto LABEL_28;
LABEL_138:
        sub_E490(v85[19].m128i_i64[1], v28, v62, v63);
        goto LABEL_28;
      }
      goto LABEL_152;
    }
    goto LABEL_118;
  }
  v29 = -3 - v85[0].m128i_i32[0];
  if ( v85[19].m128i_i32[0] == -1 )
  {
    memset(&v85[20], 0, 0x90u);
    v85[21].m128i_i32[2] = v85[2].m128i_i32[2];
    if ( v29 < 0 )
      goto LABEL_49;
LABEL_26:
    v30 = __errno_location();
    v31 = v85[0].m128i_i64[1];
    v32 = v30;
    *v30 = -3 - (int)v28;
    sub_E490(v31, v28, (unsigned int)(-3 - (int)v28), v33);
    v35 = (unsigned int)(-3 - v85[19].m128i_i32[0]);
    if ( -3 - v85[19].m128i_i32[0] >= 0 )
      goto LABEL_27;
  }
  else
  {
    if ( v29 >= 0 )
      goto LABEL_26;
LABEL_48:
    if ( -3 - v85[19].m128i_i32[0] < 0 )
      goto LABEL_49;
    fileb = -3 - v85[19].m128i_i32[0];
    v56 = __errno_location();
    v35 = fileb;
    v32 = v56;
LABEL_27:
    *v32 = v35;
    sub_E490(v85[19].m128i_i64[1], v28, v35, v34);
  }
LABEL_28:
  while ( fflush_unlocked(stdout) )
  {
    v28 = "standard output";
    v79 = dcgettext(0, "standard output", 5);
    sub_E770(v79);
LABEL_181:
    i = 0;
LABEL_182:
    sub_E490(v85[19 * i].m128i_i64[1], v28, v66, v67);
    free(v9);
    free(0);
  }
LABEL_29:
  free(v10);
  free(ptr);
  return v26;
}



// Function: dir_loop @ 0x9e40
long long dir_loop(long long a1, int a2)
{
  long long result; // rax
  long long v3; // rcx
  long long v4; // rsi
  long long v5; // rdi
  int v6; // edx

  result = *(long long *)(a1 + 608);
  if ( result )
  {
    v3 = 304LL * a2;
    v4 = a1 + v3;
    v5 = *(long long *)(a1 + v3 + 24);
    while ( 1 )
    {
      while ( 1 )
      {
        if ( *(long long *)(result + v3 + 24) == v5 && *(long long *)(result + v3 + 16) == *(long long *)(v4 + 16) )
          return 1;
        v6 = *(int *)(result + v3 + 40) & 0xF000;
        if ( v6 != 24576 )
          break;
        if ( (*(int *)(v4 + 40) & 0xF000) == 0x6000 )
        {
          if ( *(long long *)(result + v3 + 56) == *(long long *)(v4 + 56) )
            return 1;
          goto LABEL_11;
        }
LABEL_5:
        result = *(long long *)(result + 608);
        if ( !result )
          return result;
      }
      if ( v6 != 0x2000 || (*(int *)(v4 + 40) & 0xF000) != 0x2000 )
        goto LABEL_5;
      if ( *(long long *)(result + v3 + 56) == *(long long *)(v4 + 56) )
        return 1;
LABEL_11:
      result = *(long long *)(result + 608);
      if ( !result )
        return result;
    }
  }
  return 0;
}



// Function: compare_names @ 0x9f10
int compare_names(char *s1, char *s2)
{
  int result; // eax
  int *v3; // rax
  bool v4; // zf

  if ( !byte_266E8 )
    return strcmp(s1, s2);
  v3 = __errno_location();
  v4 = byte_26551 == 0;
  *v3 = 0;
  if ( !v4 )
    return strcasecmp(s1, s2);
  result = strcoll(s1, s2);
  if ( !result )
    return strcmp(s1, s2);
  return result;
}



// Function: dir_read @ 0x9f90
long long dir_read(long long a1, unsigned long long *a2)
{
  bool v2; // zf
  unsigned long long v3; // r14
  long long v4; // rax
  const char *v5; // r15
  struct dirent *v6; // rax
  struct dirent *v7; // rbx
  char *d_name; // r13
  char v9; // al
  unsigned long long v10; // rbx
  long long v11; // rax
  int v13; // ebx
  long long v14; // rax
  const char **v15; // rbx
  long long v16; // rbp
  const char **v17; // rbp
  long long v18; // rax
  size_t n; // [rsp+8h] [rbp-60h]
  size_t na; // [rsp+8h] [rbp-60h]
  DIR *dirp; // [rsp+10h] [rbp-58h]
  int *v22; // [rsp+18h] [rbp-50h]
  unsigned long long v23; // [rsp+20h] [rbp-48h]
  unsigned long long v24; // [rsp+28h] [rbp-40h]

  v2 = *(int *)a1 == -1;
  a2[1] = 0;
  a2[2] = 0;
  if ( !v2 )
  {
    dirp = opendir(*(const char **)(a1 + 8));
    if ( dirp )
    {
      v3 = 512;
      v4 = sub_16A40(512);
      a2[2] = v4;
      v5 = (const char *)v4;
      v23 = 0;
      v22 = __errno_location();
      v24 = 0;
      while ( 1 )
      {
        *v22 = 0;
        v6 = readdir(dirp);
        v7 = v6;
        if ( !v6 )
          break;
        d_name = v6->d_name;
        n = strlen(v6->d_name);
        if ( v7->d_name[0] == 46 )
        {
          v9 = v7->d_name[1];
          if ( !v9 || v9 == 46 && !v7->d_name[2] )
            continue;
        }
        if ( !(unsigned char)sub_104D0(qword_26410, v7->d_name) )
        {
          v10 = n + 1 + v23;
          na = n + 1;
          if ( v3 < v10 )
          {
            while ( v3 <= 0x3FFFFFFFFFFFFFFELL )
            {
              v3 *= 2LL;
              v11 = sub_16A90(v5, v3);
              a2[2] = v11;
              v5 = (const char *)v11;
              if ( v3 >= v10 )
                goto LABEL_12;
            }
LABEL_25:
            sub_16EC0();
          }
LABEL_12:
          memcpy((void *)&v5[v23], d_name, na);
          ++v24;
          v23 = v10;
        }
      }
      v13 = *v22;
      if ( *v22 )
      {
        closedir(dirp);
        *v22 = v13;
      }
      else if ( !closedir(dirp) )
      {
        if ( v24 > 0xFFFFFFFFFFFFFFDLL )
          goto LABEL_25;
        v14 = sub_16A40(8 * v24 + 8);
        *a2 = v24;
        a2[1] = v14;
        v15 = (const char **)v14;
        v16 = 8 * v24;
        if ( v24 )
        {
          v17 = (const char **)(v14 + v16);
          do
          {
            *v15++ = v5;
            v5 += strlen(v5) + 1;
          }
          while ( v15 != v17 );
        }
        else
        {
          v15 = (const char **)(v16 + v14);
        }
        goto LABEL_23;
      }
    }
    return 0;
  }
  v18 = sub_16A40(8);
  *a2 = 0;
  a2[1] = v18;
  v15 = (const char **)v18;
LABEL_23:
  *v15 = 0;
  return 1;
}



// Function: compare_names_for_qsort @ 0xa1c0
int compare_names_for_qsort(const char **a1, const char **a2)
{
  const char *v2; // rbp
  const char *v3; // r12
  int result; // eax
  int *v5; // rax
  bool v6; // zf

  v2 = *a1;
  v3 = *a2;
  if ( !byte_266E8 )
    return strcmp(v2, v3);
  v5 = __errno_location();
  v6 = byte_26551 == 0;
  *v5 = 0;
  result = v6 ? strcoll(v2, v3) : strcasecmp(v2, v3);
  if ( !result )
    return strcmp(v2, v3);
  return result;
}



// Function: diff_dirs @ 0xa230
long long diff_dirs(long long a1, long long (*a2)(long long, char *, long long))
{
  int v2; // r9d
  long long v3; // r12
  char *v4; // rax
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // rdx
  long long v9; // rcx
  char *v10; // rbx
  int v11; // eax
  unsigned int v12; // eax
  long long *v13; // rax
  long long v14; // rdx
  char *v15; // rsi
  int v16; // eax
  long long *v17; // rax
  long long *v18; // rax
  long long v19; // rax
  char **v20; // r14
  char *v21; // rbp
  char *v22; // r13
  char *v23; // r12
  int v24; // eax
  int v26; // [rsp+1Ch] [rbp-8Ch]
  void *base; // [rsp+20h] [rbp-88h]
  void *v28; // [rsp+28h] [rbp-80h]
  size_t nmemb; // [rsp+30h] [rbp-78h] BYREF
  void *ptr; // [rsp+38h] [rbp-70h]
  void *v31; // [rsp+40h] [rbp-68h]
  size_t v32; // [rsp+48h] [rbp-60h] BYREF
  void *v33; // [rsp+50h] [rbp-58h]
  void *v34; // [rsp+58h] [rbp-50h]
  unsigned long long v35; // [rsp+68h] [rbp-40h]

  v2 = *(int *)a1;
  v35 = __readfsqword(0x28u);
  v26 = 0;
  if ( (v2 == -1 || (unsigned char)sub_9E40(a1, 0))
    && (*(int *)(a1 + 304) == -1 || (unsigned char)sub_9E40(a1, 1)) )
  {
    v3 = *(long long *)(a1 + 304LL * (v2 == -1) + 8);
    v4 = dcgettext(0, "%s: recursive directory loop", 5);
    error(0, 0, v4, v3);
    return 2;
  }
  if ( !(unsigned char)sub_9F90(a1, &nmemb) )
  {
    sub_E490(*(long long *)(a1 + 8), &nmemb, v6, v7);
    v26 = 2;
  }
  if ( !(unsigned char)sub_9F90(a1 + 304, &v32) )
  {
    sub_E490(*(long long *)(a1 + 312), &v32, v8, v9);
    v26 = 2;
    goto LABEL_10;
  }
  if ( !v26 )
  {
    base = ptr;
    v28 = v33;
    byte_266E8 = _setjmp(env) == 0;
    qsort(base, nmemb, 8u, (__compar_fn_t)compar);
    qsort(v28, v32, 8u, (__compar_fn_t)compar);
    v10 = qword_26488;
    if ( qword_26488 && !*(long long *)(a1 + 608) )
    {
      while ( *(long long *)base && sub_9F10(*(char **)base, v10) < 0 )
        base = (char *)base + 8;
      while ( *(long long *)v28 && sub_9F10(*(char **)v28, v10) < 0 )
        v28 = (char *)v28 + 8;
    }
    while ( *(long long *)base || *(long long *)v28 )
    {
      v15 = *(char **)base;
      v17 = (long long *)v28;
      if ( !*(long long *)base )
      {
        v28 = (char *)v28 + 8;
        v14 = *v17;
        goto LABEL_19;
      }
      if ( *(long long *)v28 )
      {
        v11 = sub_9F10(*(char **)base, *(char **)v28);
        if ( !v11 )
        {
          if ( byte_26551 )
          {
            v12 = strcmp(*(const char **)base, *(const char **)v28);
            if ( v12 )
            {
              v19 = v12 >> 31;
              v20 = (char **)*(&base + 1 - (int)v19);
              v21 = v20[1];
              v22 = *(char **)*(&base + v19);
              v23 = (char *)(v20 + 1);
              if ( v21 )
              {
                while ( !sub_9F10(v21, v22) )
                {
                  v24 = strcmp(v21, v22);
                  if ( v24 >= 0 )
                  {
                    if ( !v24 )
                    {
                      memmove(v20 + 1, v20, v23 - (char *)v20);
                      *v20 = v22;
                    }
                    break;
                  }
                  v21 = (char *)*((long long *)v23 + 1);
                  v23 += 8;
                  if ( !v21 )
                    break;
                }
              }
            }
          }
          v13 = (long long *)v28;
          v28 = (char *)v28 + 8;
          v14 = *v13;
          goto LABEL_18;
        }
        if ( v11 >= 0 )
        {
          v18 = (long long *)v28;
          v15 = 0;
          v28 = (char *)v28 + 8;
          v14 = *v18;
          goto LABEL_19;
        }
      }
      v14 = 0;
LABEL_18:
      v15 = *(char **)base;
      base = (char *)base + 8;
LABEL_19:
      v16 = a2(a1, v15, v14);
      if ( v26 < v16 )
        v26 = v16;
    }
  }
LABEL_10:
  free(ptr);
  free(v31);
  free(v33);
  free(v34);
  return (unsigned int)v26;
}



// Function: find_dir_file_pathname @ 0xa660
long long find_dir_file_pathname(long long a1, char *a2)
{
  long long v2; // rax
  long long v3; // r12
  char *v5; // rbp
  char *v6; // rbx
  char *v7; // [rsp+8h] [rbp-190h]
  unsigned long long v8; // [rsp+20h] [rbp-178h] BYREF
  void *ptr; // [rsp+28h] [rbp-170h]
  void *v10; // [rsp+30h] [rbp-168h]
  int v11; // [rsp+40h] [rbp-158h] BYREF
  long long v12; // [rsp+48h] [rbp-150h]
  unsigned long long v13; // [rsp+178h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  v7 = a2;
  ptr = 0;
  v10 = 0;
  if ( byte_26551 )
  {
    v12 = a1;
    v11 = 0;
    if ( (unsigned char)sub_9F90((long long)&v11, &v8) )
    {
      byte_266E8 = 1;
      if ( _setjmp(env) )
      {
        v2 = sub_10D80(a1, a2, 0);
        goto LABEL_3;
      }
      v5 = (char *)ptr;
      v6 = *(char **)ptr;
      if ( *(long long *)ptr )
      {
        do
        {
          if ( !sub_9F10(v6, a2) )
          {
            if ( !strcmp(v6, a2) )
            {
              v2 = sub_10D80(a1, v6, 0);
              goto LABEL_3;
            }
            if ( a2 != v7 )
              v6 = v7;
            v7 = v6;
          }
          v6 = (char *)*((long long *)v5 + 1);
          v5 += 8;
        }
        while ( v6 );
      }
    }
  }
  v2 = sub_10D80(a1, v7, 0);
LABEL_3:
  v3 = v2;
  free(ptr);
  free(v10);
  return v3;
}



// Function: print_ed_hunk @ 0xa7c0
unsigned long long print_ed_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  int v5; // ebx
  int v6; // esi
  char *IO_write_ptr; // rax
  char *v8; // rax
  long long v9; // rbx
  long long v10; // rsi
  long long v11; // [rsp+8h] [rbp-50h] BYREF
  long long v12; // [rsp+10h] [rbp-48h] BYREF
  long long v13; // [rsp+18h] [rbp-40h] BYREF
  long long v14; // [rsp+20h] [rbp-38h] BYREF
  unsigned long long v15; // [rsp+28h] [rbp-30h]

  v15 = __readfsqword(0x28u);
  v1 = sub_F670(a1, &v11, &v12, &v13, &v14);
  if ( v1 )
  {
    v5 = v1;
    sub_E830(a1, &v11, v2, v3);
    sub_F610(44, &unk_261A0, v11, v12);
    v6 = byte_1E550[v5];
    IO_write_ptr = stream->_IO_write_ptr;
    if ( IO_write_ptr >= stream->_IO_write_end )
    {
      __overflow(stream, v6);
    }
    else
    {
      stream->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = v6;
    }
    v8 = stream->_IO_write_ptr;
    if ( v8 >= stream->_IO_write_end )
    {
      __overflow(stream, 10);
    }
    else
    {
      stream->_IO_write_ptr = v8 + 1;
      *v8 = 10;
    }
    if ( v5 != 1 )
    {
      v9 = v13;
      if ( v14 < v13 )
      {
LABEL_12:
        fputs_unlocked(".\n", stream);
      }
      else
      {
        while ( 1 )
        {
          while ( 1 )
          {
            v10 = qword_26388 + 8 * v9++;
            if ( **(char **)v10 == 46 && *(char *)(*(long long *)v10 + 1LL) == 10 )
              break;
            sub_F500("", v10);
            if ( v9 > v14 )
              goto LABEL_12;
          }
          fputs_unlocked("..\n.\ns/.//\n", stream);
          if ( v14 < v9 )
            break;
          fputs_unlocked("a\n", stream);
        }
      }
    }
  }
  return v15 - __readfsqword(0x28u);
}



// Function: pr_forward_ed_hunk @ 0xa960
unsigned long long pr_forward_ed_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  int v5; // ebx
  unsigned char v6; // dl
  char *IO_write_ptr; // rax
  char *v8; // rax
  long long i; // rbx
  long long v10; // rsi
  long long v11; // [rsp+8h] [rbp-40h] BYREF
  long long v12; // [rsp+10h] [rbp-38h] BYREF
  long long v13; // [rsp+18h] [rbp-30h] BYREF
  long long v14; // [rsp+20h] [rbp-28h] BYREF
  unsigned long long v15; // [rsp+28h] [rbp-20h]

  v15 = __readfsqword(0x28u);
  v1 = sub_F670(a1, &v11, &v12, &v13, &v14);
  if ( v1 )
  {
    v5 = v1;
    sub_E830(a1, &v11, v2, v3);
    v6 = byte_1E550[v5];
    IO_write_ptr = stream->_IO_write_ptr;
    if ( IO_write_ptr >= stream->_IO_write_end )
    {
      __overflow(stream, v6);
    }
    else
    {
      stream->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = v6;
    }
    sub_F610(32, &unk_261A0, v11, v12);
    v8 = stream->_IO_write_ptr;
    if ( v8 >= stream->_IO_write_end )
    {
      __overflow(stream, 10);
    }
    else
    {
      stream->_IO_write_ptr = v8 + 1;
      *v8 = 10;
    }
    if ( v5 != 1 )
    {
      for ( i = v13; v14 >= i; ++i )
      {
        v10 = qword_26388 + 8 * i;
        sub_F500("", v10);
      }
      fputs_unlocked(".\n", stream);
    }
  }
  return v15 - __readfsqword(0x28u);
}



// Function: print_rcs_hunk @ 0xaab0
unsigned long long print_rcs_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  char v5; // bl
  long long v6; // r8
  long long v7; // r8
  long long i; // rbx
  long long v9; // rsi
  long long v10; // [rsp+8h] [rbp-60h] BYREF
  long long v11; // [rsp+10h] [rbp-58h] BYREF
  long long v12; // [rsp+18h] [rbp-50h] BYREF
  long long v13; // [rsp+20h] [rbp-48h] BYREF
  long long v14; // [rsp+28h] [rbp-40h] BYREF
  long long v15; // [rsp+30h] [rbp-38h] BYREF
  long long v16; // [rsp+38h] [rbp-30h] BYREF
  long long v17; // [rsp+40h] [rbp-28h] BYREF
  unsigned long long v18; // [rsp+48h] [rbp-20h]

  v18 = __readfsqword(0x28u);
  v1 = sub_F670(a1, &v10, &v11, &v12, &v13);
  if ( v1 )
  {
    v5 = v1;
    sub_E830(a1, &v10, v2, v3);
    sub_F5E0(&unk_261A0, v10, v11, &v14, &v15);
    if ( (v5 & 1) != 0 )
    {
      v6 = 1;
      if ( v14 <= v15 )
        v6 = v15 - v14 + 1;
      __fprintf_chk(stream, 1, "d%ld %ld\n", v14, v6);
    }
    if ( (v5 & 2) != 0 )
    {
      sub_F5E0(&unk_262D0, v12, v13, &v16, &v17);
      v7 = 1;
      if ( v16 <= v17 )
        v7 = v17 - v16 + 1;
      __fprintf_chk(stream, 1, "a%ld %ld\n", v15, v7);
      for ( i = v12; v13 >= i; ++i )
      {
        v9 = qword_26388 + 8 * i;
        sub_F500("", v9);
      }
    }
  }
  return v18 - __readfsqword(0x28u);
}



// Function: print_ed_script @ 0xac20
long long print_ed_script(long long a1)
{
  return sub_F080(a1, sub_F070, sub_A7C0);
}



// Function: pr_forward_ed_script @ 0xac40
long long pr_forward_ed_script(long long a1)
{
  return sub_F080(a1, sub_F060, sub_A960);
}



// Function: print_rcs_script @ 0xac60
long long print_rcs_script(long long a1)
{
  return sub_F080(a1, sub_F060, sub_AAB0);
}



// Function: groups_letter_value @ 0xac80
long long groups_letter_value(long long *a1, char a2)
{
  long long *v2; // rdx
  long long result; // rax

  v2 = a1;
  switch ( a2 )
  {
    case 'E':
      v2 = a1 + 3;
LABEL_4:
      result = sub_F5D0(*v2, v2[1]) - 1;
      break;
    case 'F':
      v2 = a1 + 3;
LABEL_6:
      result = sub_F5D0(*v2, v2[1]);
      break;
    case 'L':
      v2 = a1 + 3;
LABEL_8:
      result = sub_F5D0(*v2, v2[2]) - 1;
      break;
    case 'M':
      v2 = a1 + 3;
LABEL_10:
      result = sub_F5D0(*v2, v2[2]);
      break;
    case 'N':
      v2 = a1 + 3;
LABEL_12:
      result = v2[2] - v2[1];
      break;
    default:
      switch ( a2 )
      {
        case 'e':
          goto LABEL_4;
        case 'f':
          goto LABEL_6;
        case 'l':
          goto LABEL_8;
        case 'm':
          goto LABEL_10;
        case 'n':
          goto LABEL_12;
        default:
          result = -1;
          break;
      }
      break;
  }
  return result;
}



// Function: do_printf_spec @ 0xad70
char *do_printf_spec(_IO_FILE *a1, char *src, long long a3, long long a4, long long *a5)
{
  char *v8; // rbx
  unsigned long long v9; // rax
  unsigned long long v10; // r12
  unsigned long long v11; // rax
  char v12; // dl
  long long v13; // rsi
  char *v14; // r15
  int v16; // eax
  int v17; // eax
  unsigned char v18; // dl
  char *IO_write_ptr; // rax
  long long v20; // rcx
  signed long long v21; // rax
  size_t v22; // rdx
  signed long long v23; // rsi
  long long *v24; // r8
  signed long long v25; // rdi
  void *v26; // rsp
  char *v27; // rbx
  char *v28; // r8
  long long v29; // rcx
  int v30; // eax
  char *v31; // rcx
  unsigned int v32; // eax
  long long v33; // rax
  long long v34; // [rsp+0h] [rbp-1068h]
  long long v35; // [rsp+8h] [rbp-1060h] BYREF
  char v36; // [rsp+2Fh] [rbp-1039h] BYREF
  long long v37; // [rsp+1008h] [rbp-60h] BYREF
  long long v38; // [rsp+1010h] [rbp-58h]
  char *v39; // [rsp+1018h] [rbp-50h]
  size_t v40; // [rsp+1020h] [rbp-48h]
  unsigned long long v41; // [rsp+1030h] [rbp-38h]

  v8 = src + 1;
  v41 = __readfsqword(0x28u);
  v9 = 0xFFFEDF7FFFFFFFFFLL;
  do
  {
    v10 = (unsigned char)*v8++;
    if ( (unsigned char)v10 > 0x30u )
    {
      if ( (unsigned int)((char)v10 - 48) > 9 )
        goto LABEL_6;
      goto LABEL_11;
    }
  }
  while ( !_bittest64((const long long *)&v9, v10) );
  if ( (unsigned int)((char)v10 - 48) <= 9 )
  {
    do
    {
LABEL_11:
      v16 = *v8++;
      LODWORD(v10) = v16;
    }
    while ( (unsigned int)(v16 - 48) <= 9 );
    if ( (char)v16 == 46 )
      goto LABEL_13;
LABEL_6:
    v11 = (unsigned int)(v10 - 88);
    v12 = *v8;
    if ( (unsigned char)(v10 - 88) > 0x20u )
      return 0;
    goto LABEL_7;
  }
  if ( (char)v10 != 46 )
    goto LABEL_6;
  do
  {
LABEL_13:
    v17 = *v8++;
    LOBYTE(v10) = v17;
  }
  while ( (unsigned int)(v17 - 48) <= 9 );
  v11 = (unsigned int)(v17 - 88);
  v12 = *v8;
  if ( (unsigned char)(v10 - 88) > 0x20u )
    return 0;
LABEL_7:
  v13 = 0x100801001LL;
  if ( _bittest64(&v13, v11) )
  {
    if ( a3 )
    {
      if ( v12 != 110 )
        return 0;
      v20 = sub_F5D0(a3, a4);
    }
    else
    {
      v20 = sub_AC80(a5, v12);
      if ( v20 < 0 )
        return 0;
    }
    v14 = v8 + 1;
    if ( a1 )
    {
      v21 = v14 - src;
      v22 = v14 - src - 2;
      v23 = v8 - src;
      if ( (unsigned long long)(v14 - src + 1) > 0xFA0 )
      {
        v39 = (char *)v20;
        v33 = sub_16F00();
        v22 = v14 - src - 2;
        v20 = (long long)v39;
        v23 = v8 - src;
        v27 = (char *)v33;
      }
      else
      {
        v24 = (long long *)((char *)&v37 - ((v21 + 55) & 0xFFFFFFFFFFFFF000LL));
        if ( &v37 != v24 )
        {
          while ( &v35 != v24 )
            ;
        }
        v25 = ((short)v21 + 55) & 0xFF0;
        v26 = alloca(v25);
        if ( (((short)v21 + 55) & 0xFF0) != 0 )
          *(long long *)((char *)&v34 + v25) = *(long long *)((char *)&v34 + v25);
        v27 = (char *)((unsigned long long)&v36 & 0xFFFFFFFFFFFFFFE0LL);
      }
      v38 = v20;
      v39 = &v27[v23];
      v40 = v22;
      memcpy(v27, src, v22);
      v28 = v39;
      v29 = v38;
      v27[v40] = 108;
      *v28 = v10;
      v28[1] = 0;
      __fprintf_chk(a1, 1, v27, v29);
      sub_121C0(v27);
    }
  }
  else
  {
    v14 = 0;
    if ( (char)v11 != 11 )
      return v14;
    if ( v12 != 39 )
      return 0;
    v18 = v8[1];
    if ( v18 != 39 )
    {
      if ( v18 == 92 )
      {
        v30 = v8[2];
        v31 = v8 + 3;
        if ( v8[2] != 39 )
        {
          v18 = 0;
          while ( 1 )
          {
            v32 = v30 - 48;
            if ( v32 > 7 )
              return 0;
            v14 = v31 + 1;
            v18 = v32 + 8 * v18;
            v30 = (char)*v31;
            if ( *v31 == 39 )
              goto LABEL_45;
            ++v31;
          }
        }
        v14 = v8 + 3;
        v18 = 0;
        v31 = v8 + 2;
LABEL_45:
        if ( (unsigned long long)(v31 - v8 - 3) > 2 )
          return 0;
        goto LABEL_21;
      }
      if ( v18 )
      {
        v14 = v8 + 3;
        if ( v8[2] != 39 )
          return 0;
LABEL_21:
        if ( a1 )
        {
          IO_write_ptr = a1->_IO_write_ptr;
          if ( IO_write_ptr >= a1->_IO_write_end )
          {
            __overflow(a1, v18);
          }
          else
          {
            a1->_IO_write_ptr = IO_write_ptr + 1;
            *IO_write_ptr = v18;
          }
        }
      }
    }
  }
  return v14;
}



// Function: print_ifdef_lines @ 0xb070
int print_ifdef_lines(_IO_FILE *a1, char *a2, long long *a3)
{
  long long v3; // r15
  long long v4; // r14
  char *IO_write_ptr; // rax
  char *v7; // rbp
  char *v8; // r9
  char v9; // bl
  unsigned char v10; // si
  char v11; // al
  const void *v12; // rdi
  size_t v13; // rdx
  long long v16; // [rsp+10h] [rbp-58h]
  char *v17; // [rsp+18h] [rbp-50h]
  char *v18; // [rsp+20h] [rbp-48h]
  long long v19; // [rsp+28h] [rbp-40h]

  v3 = a3[1];
  v19 = *a3;
  v4 = *(long long *)(*a3 + 184);
  IO_write_ptr = (char *)a3[2];
  v16 = (long long)IO_write_ptr;
  if ( a1 )
  {
    if ( !byte_264A0 )
    {
      LODWORD(IO_write_ptr) = (int)a2;
      if ( *a2 == 37 )
      {
        LODWORD(IO_write_ptr) = (unsigned char)a2[1];
        if ( (char)IO_write_ptr == 108 )
        {
          LODWORD(IO_write_ptr) = (int)a2;
          if ( a2[2] == 10 && !a2[3] )
          {
            if ( v3 >= v16 )
              return (int)IO_write_ptr;
            v12 = *(const void **)(v4 + 8 * v3);
            v13 = (*(char *)(*(long long *)(v4 + 8 * v16) - 1LL) != 10) + *(long long *)(v4 + 8 * v16) - (long long)v12;
            goto LABEL_28;
          }
        }
        else if ( (char)IO_write_ptr == 76 )
        {
          LODWORD(IO_write_ptr) = (int)a2;
          if ( !a2[2] )
          {
            v12 = *(const void **)(v4 + 8 * v3);
            v13 = *(long long *)(v4 + 8 * v16) - (long long)v12;
LABEL_28:
            LODWORD(IO_write_ptr) = fwrite_unlocked(v12, 1u, v13, a1);
            return (int)IO_write_ptr;
          }
        }
      }
    }
    if ( v3 >= v16 )
      return (int)IO_write_ptr;
    LODWORD(IO_write_ptr) = (int)a2 + 1;
    v17 = a2 + 1;
    while ( 1 )
    {
      v7 = a2;
      v8 = v17;
      v9 = *a2;
      if ( *a2 )
        break;
LABEL_18:
      if ( v16 == ++v3 )
        return (int)IO_write_ptr;
    }
    while ( v9 == 37 )
    {
      v11 = v7[1];
      if ( v11 == 76 || v11 == 108 )
      {
        v7 += 2;
        LODWORD(IO_write_ptr) = sub_F0E0(*(void **)(v4 + 8 * v3));
        goto LABEL_10;
      }
      if ( v11 == 37 )
      {
        v7 += 2;
        v10 = 37;
LABEL_8:
        IO_write_ptr = a1->_IO_write_ptr;
        if ( IO_write_ptr < a1->_IO_write_end )
          goto LABEL_9;
LABEL_17:
        LODWORD(IO_write_ptr) = __overflow(a1, v10);
        v9 = *v7;
        v8 = v7 + 1;
        if ( !*v7 )
          goto LABEL_18;
      }
      else
      {
        v18 = v8;
        IO_write_ptr = sub_AD70(a1, v7, v19, v3, 0);
        v7 = IO_write_ptr;
        if ( IO_write_ptr )
          goto LABEL_10;
        v7 = v18;
        IO_write_ptr = a1->_IO_write_ptr;
        v10 = 37;
        if ( IO_write_ptr >= a1->_IO_write_end )
          goto LABEL_17;
LABEL_9:
        a1->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = v9;
LABEL_10:
        v9 = *v7;
        v8 = v7 + 1;
        if ( !*v7 )
          goto LABEL_18;
      }
    }
    v10 = v9;
    v7 = v8;
    goto LABEL_8;
  }
  return (int)IO_write_ptr;
}



// Function: format_group @ 0xb2a0
char *format_group(_IO_FILE *a1, char *a2, char a3, long long *a4)
{
  char *result; // rax
  unsigned char v6; // bl
  unsigned char *v9; // r12
  char *IO_write_ptr; // rax
  char *v11; // r10
  long long *v12; // rdx
  char *v13; // rsi
  long long v14; // rcx
  int v15; // esi
  int *v16; // rax
  intmax_t v17; // rax
  long long v18; // rcx
  char *v19; // rax
  _IO_FILE *v20; // rdi
  _IO_FILE *v21; // r12
  long long v22; // rax
  long long *v23; // [rsp+8h] [rbp-90h]
  char *v24; // [rsp+10h] [rbp-88h]
  char *v25; // [rsp+18h] [rbp-80h]
  int *v26; // [rsp+18h] [rbp-80h]
  long long v27; // [rsp+18h] [rbp-80h]
  char *nptra; // [rsp+20h] [rbp-78h]
  char *nptr; // [rsp+20h] [rbp-78h]
  long long v30; // [rsp+28h] [rbp-70h]
  char *endptr; // [rsp+38h] [rbp-60h] BYREF
  long long v32[11]; // [rsp+40h] [rbp-58h]

  result = a2;
  v6 = *a2;
  v32[3] = __readfsqword(0x28u);
  if ( v6 != a3 && v6 )
  {
    v23 = a4 + 3;
    do
    {
      v9 = (unsigned char *)(result + 1);
      if ( v6 == 37 )
      {
        v11 = result + 2;
        switch ( result[1] )
        {
          case '%':
            v9 = (unsigned char *)(result + 2);
            v6 = result[1];
            goto LABEL_5;
          case '(':
            v14 = 0;
            break;
          case '<':
            v25 = result;
            v13 = (char *)qword_26448;
            v12 = a4;
            v24 = result + 2;
            goto LABEL_16;
          case '=':
            v25 = result;
            v13 = (char *)qword_26440[0];
            v12 = a4;
            v24 = result + 2;
            goto LABEL_16;
          case '>':
            v25 = result;
            v12 = v23;
            v24 = result + 2;
            v13 = (char *)qword_26450;
LABEL_16:
            sub_B070(a1, v13, v12);
            v6 = v25[2];
            result = v24;
            continue;
          default:
            result = sub_AD70(a1, result, 0, 0, a4);
            if ( !result )
              goto LABEL_5;
            v6 = *result;
            continue;
        }
        while ( 1 )
        {
          v15 = *v11;
          if ( (unsigned int)(v15 - 48) > 9 )
          {
            v27 = v14;
            nptr = v11;
            v22 = sub_AC80(a4, v15);
            v18 = v27;
            v32[v27] = v22;
            if ( v22 < 0 )
              goto LABEL_5;
            v19 = nptr + 1;
          }
          else
          {
            v30 = v14;
            nptra = v11;
            v16 = __errno_location();
            *v16 = 0;
            v26 = v16;
            v17 = strtoimax(nptra, &endptr, 10);
            v18 = v30;
            v32[v30] = v17;
            if ( *v26 )
              goto LABEL_5;
            v19 = endptr;
          }
          v11 = v19 + 1;
          if ( *v19 != byte_1E194[v18] )
            goto LABEL_5;
          if ( v18 == 1 )
            break;
          v14 = 1;
        }
        v20 = 0;
        v21 = 0;
        if ( v32[0] == v32[1] )
          v20 = a1;
        else
          v21 = a1;
        result = (char *)format_group(v20, v11, 58, a4);
        if ( !*result )
          return result;
        result = (char *)format_group(v21, result + 1, 41, a4);
        if ( !*result )
          return result;
        v6 = *++result;
      }
      else
      {
LABEL_5:
        if ( !a1 )
          goto LABEL_8;
        IO_write_ptr = a1->_IO_write_ptr;
        if ( IO_write_ptr < a1->_IO_write_end )
        {
          a1->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = v6;
LABEL_8:
          v6 = *v9;
          result = (char *)v9;
          continue;
        }
        __overflow(a1, v6);
        v6 = *v9;
        result = (char *)v9;
      }
    }
    while ( v6 && a3 != v6 );
  }
  return result;
}



// Function: print_ifdef_hunk @ 0xb560
unsigned long long print_ifdef_hunk(long long a1)
{
  unsigned int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  long long v4; // rbx
  long long v6; // rcx
  long long v7; // rsi
  char *v8; // r8
  long long v9; // [rsp+0h] [rbp-88h] BYREF
  long long v10; // [rsp+8h] [rbp-80h] BYREF
  long long v11; // [rsp+10h] [rbp-78h] BYREF
  long long v12; // [rsp+18h] [rbp-70h] BYREF
  void *v13; // [rsp+20h] [rbp-68h] BYREF
  long long v14; // [rsp+28h] [rbp-60h]
  long long v15; // [rsp+30h] [rbp-58h]
  void *v16; // [rsp+38h] [rbp-50h]
  long long v17; // [rsp+40h] [rbp-48h]
  long long v18; // [rsp+48h] [rbp-40h]
  unsigned long long v19; // [rsp+58h] [rbp-30h]

  v19 = __readfsqword(0x28u);
  v1 = sub_F670(a1, &v9, &v10, &v11, &v12);
  v4 = v1;
  if ( v1 )
  {
    sub_E830(a1, &v9, v2, v3);
    v6 = v9;
    v7 = v11;
    if ( qword_266F8 < v9 || qword_266F0 < v11 )
    {
      v18 = v11;
      v15 = v9;
      v17 = qword_266F0;
      v13 = &unk_261A0;
      v14 = qword_266F8;
      v16 = &unk_262D0;
      sub_B2A0(stream, s1, 0, (long long *)&v13);
      v7 = v11;
      v6 = v9;
    }
    v14 = v6;
    v8 = (&s1)[v4];
    v17 = v7;
    v13 = &unk_261A0;
    v15 = v10 + 1;
    qword_266F8 = v10 + 1;
    v16 = &unk_262D0;
    v18 = v12 + 1;
    qword_266F0 = v12 + 1;
    sub_B2A0(stream, v8, 0, (long long *)&v13);
  }
  return v19 - __readfsqword(0x28u);
}



// Function: print_ifdef_script @ 0xb6c0
unsigned long long print_ifdef_script(long long a1)
{
  long long v1; // rdx
  long long v2; // rcx
  long long v4[7]; // [rsp+0h] [rbp-48h] BYREF
  unsigned long long v5; // [rsp+38h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  qword_266F0 = -qword_26288;
  qword_266F8 = -qword_26288;
  sub_F080(a1, sub_F060, sub_B560);
  if ( qword_26270 > qword_266F8 || qword_263A0 > qword_266F0 )
  {
    sub_E830(a1, sub_F060, v1, v2);
    v4[0] = (long long)&unk_261A0;
    v4[1] = qword_266F8;
    v4[2] = qword_26270;
    v4[3] = (long long)&unk_262D0;
    v4[4] = qword_266F0;
    v4[5] = qword_263A0;
    sub_B2A0(stream, s1, 0, v4);
  }
  return v5 - __readfsqword(0x28u);
}



// Function: prepare_text @ 0xb7b0
void prepare_text(long long a1)
{
  long long *v1; // r13
  long long v2; // r12
  char *v4; // rbp
  const void *v5; // rdi
  char *v6; // rax
  char *v7; // rdx
  char v8; // cl

  v1 = *(long long **)(a1 + 160);
  if ( v1 )
  {
    v2 = *(long long *)(a1 + 176);
    if ( byte_26490 )
    {
      v4 = (char *)v1 + v2;
      v5 = *(const void **)(a1 + 160);
      *((char *)v1 + v2) = 13;
      v6 = rawmemchr(v5, 13);
      if ( (char *)v1 + v2 == v6 )
      {
        v7 = (char *)v1 + v2;
      }
      else
      {
        v7 = v6;
        do
        {
          while ( 1 )
          {
            v8 = *v6;
            if ( *v6 == 13 && v6[1] == 10 )
              break;
            ++v6;
            *v7++ = v8;
            if ( v4 == v6 )
              goto LABEL_9;
          }
          ++v7;
          v6 += 2;
          *(v7 - 1) = 10;
        }
        while ( v4 != v6 );
      }
LABEL_9:
      v2 = v7 - (char *)v1;
    }
    if ( v2 )
    {
      if ( *((char *)v1 + v2 - 1) != 10 )
      {
        *((char *)v1 + v2++) = 10;
        *(char *)(a1 + 288) = 1;
      }
      v1 = (long long *)((char *)v1 + v2);
    }
    *v1 = 0;
    *(long long *)(a1 + 176) = v2;
  }
}



// Function: sip @ 0xb920
bool sip(long long a1, char a2)
{
  long long v2; // rax

  if ( *(int *)a1 < 0 )
  {
    *(long long *)(a1 + 168) = 8;
    *(long long *)(a1 + 160) = sub_16A40(8);
    goto LABEL_3;
  }
  v2 = sub_17190(8, *(long long *)(a1 + 72), 0x7FFFFFFFFFFFFFEFLL);
  *(long long *)(a1 + 168) = v2;
  *(long long *)(a1 + 160) = sub_16A40(v2);
  if ( a2 )
  {
LABEL_3:
    *(long long *)(a1 + 176) = 0;
    *(char *)(a1 + 289) = 0;
    return 0;
  }
  sub_B8A0(a1, *(long long *)(a1 + 168));
  return memchr(*(const void **)(a1 + 160), 0, *(long long *)(a1 + 176)) != 0;
}



// Function: slurp @ 0xb9e0
unsigned long long slurp(unsigned int *a1)
{
  unsigned long long result; // rax
  unsigned long long v3; // rbp
  unsigned long long v4; // rsi
  unsigned long long v5; // rdx
  unsigned long long v6; // rsi
  long long v7; // rsi
  long long v8; // rdi
  long long v9; // rax
  unsigned long long v10; // r12
  long long v11; // rdi
  unsigned long long v12; // rsi
  unsigned long long v13; // rsi
  unsigned long long v14; // rax
  long long v15; // rdi

  result = *a1;
  if ( (result & 0x80000000) != 0LL )
    return result;
  if ( (a1[10] & 0xF000) == 0x8000 )
  {
    v3 = *((long long *)a1 + 8);
    v4 = (v3 & 0xFFFFFFFFFFFFFFF8LL) + 16;
    if ( v3 > v4 || v4 > 0x7FFFFFFFFFFFFFFELL )
LABEL_22:
      sub_16EC0();
    if ( *((long long *)a1 + 21) >= v4 )
    {
      result = *((long long *)a1 + 22);
      if ( result > v3 )
        goto LABEL_7;
LABEL_20:
      sub_B8A0(a1, v3 - result + 1);
      result = *((long long *)a1 + 22);
      if ( result <= v3 )
        return result;
      goto LABEL_7;
    }
    *((long long *)a1 + 21) = v4;
    *((long long *)a1 + 20) = sub_16A90(*((long long *)a1 + 20), v4);
    result = *((long long *)a1 + 22);
    if ( result <= v3 )
      goto LABEL_20;
  }
  else
  {
    result = *((long long *)a1 + 22);
  }
LABEL_7:
  result = sub_B8A0(a1, *((long long *)a1 + 21) - result);
  v5 = *((long long *)a1 + 22);
  if ( v5 )
  {
    v6 = *((long long *)a1 + 21);
    while ( v5 == v6 )
    {
      do
      {
        if ( v6 > 0x3FFFFFFFFFFFFFF7LL )
          goto LABEL_22;
        v7 = 2 * v6;
        v8 = *((long long *)a1 + 20);
        *((long long *)a1 + 21) = v7;
        v9 = sub_16A90(v8, v7);
        v6 = *((long long *)a1 + 21);
        v5 = *((long long *)a1 + 22);
        *((long long *)a1 + 20) = v9;
        v10 = v6 - v5;
      }
      while ( v6 == v5 );
      if ( !*((char *)a1 + 289) )
      {
        v13 = v9 + v5;
        v14 = sub_170E0(*a1, (void *)(v9 + v5));
        if ( v14 == -1 )
        {
          v15 = *((long long *)a1 + 1);
          sub_E770(v15);
          return sub_BBC0(v15, v13);
        }
        v6 = *((long long *)a1 + 21);
        v5 = v14 + *((long long *)a1 + 22);
        *((long long *)a1 + 22) = v5;
        *((char *)a1 + 289) = v10 > v14;
      }
    }
    v11 = *((long long *)a1 + 20);
    v12 = (v5 + 16) & 0xFFFFFFFFFFFFFFF8LL;
    *((long long *)a1 + 21) = v12;
    result = sub_16A90(v11, v12);
    *((long long *)a1 + 20) = result;
  }
  return result;
}



// Function: read_files @ 0xbbc0
long long read_files(long long a1, char a2)
{
  char v2; // bl
  bool v3; // al
  unsigned long long v5; // rsi
  char *v6; // r13
  char v7; // al
  unsigned long long v8; // r11
  char *i; // rax
  char *v10; // rdi
  char *v11; // r14
  char *v12; // r8
  char *v13; // rdx
  unsigned int v14; // r9d
  long long v15; // r10
  long long j; // rcx
  char *v17; // rcx
  char *v18; // rbx
  bool v19; // zf
  unsigned long long v20; // rsi
  long long v21; // r10
  long long v22; // r9
  unsigned long long v23; // rsi
  unsigned long long v24; // r12
  long long v25; // rax
  long long v26; // rdx
  long long v27; // rcx
  long long v28; // r9
  unsigned long long v29; // r15
  unsigned long long v30; // r8
  long long v31; // r10
  char *v32; // r11
  char *v33; // rbx
  unsigned long long v34; // rdi
  unsigned long long v35; // r13
  long long v36; // r15
  long long v37; // r12
  char *v38; // r14
  long long v39; // rbp
  long long v40; // rbp
  long long v41; // rax
  char *v42; // rdx
  char *v43; // r9
  long long v44; // r9
  bool v45; // r10
  long long v46; // rbp
  long long v47; // rbx
  long long v48; // rax
  unsigned long long v49; // rax
  signed long long v50; // rax
  unsigned long long v51; // rax
  unsigned long long v52; // rbx
  long long v53; // rax
  long long v54; // rax
  long long v55; // rsi
  long long k; // rax
  long long v57; // rax
  unsigned long long v58; // r12
  unsigned long long v59; // rbx
  unsigned long long v60; // rax
  long long v61; // rax
  size_t v62; // rdi
  long long v63; // r15
  unsigned char *v64; // r14
  long long v65; // rax
  unsigned int v66; // r15d
  long long v67; // rbp
  unsigned char *v68; // r13
  bool v69; // cc
  char v70; // r12
  unsigned long long v71; // r14
  unsigned char *v72; // r15
  long long v73; // rbx
  int v74; // r12d
  unsigned long long v75; // rbp
  const unsigned short **v76; // rax
  long long v77; // r13
  unsigned long long v78; // rbx
  size_t m; // r12
  long long *v80; // rbp
  unsigned long long v81; // r13
  const unsigned short *v82; // r14
  unsigned char *v83; // r15
  long long v84; // rax
  char *v85; // rax
  unsigned long long v86; // rbx
  const unsigned short *v87; // r14
  long long v88; // rax
  bool v89; // cf
  const int_t *v90; // rsi
  long long v91; // rax
  long long v92; // rbp
  long long v93; // rbx
  long long v94; // rax
  unsigned char **v95; // rdx
  unsigned long long v96; // r15
  long long v97; // rdx
  unsigned long long v98; // rax
  unsigned long long v99; // r12
  unsigned long long v100; // rax
  long long v101; // rax
  unsigned long long v102; // r12
  unsigned long long v103; // rax
  long long v104; // [rsp+8h] [rbp-E0h]
  char *v105; // [rsp+8h] [rbp-E0h]
  long long v106; // [rsp+8h] [rbp-E0h]
  unsigned long long v107; // [rsp+8h] [rbp-E0h]
  long long v108; // [rsp+8h] [rbp-E0h]
  unsigned char *v109; // [rsp+8h] [rbp-E0h]
  unsigned char *v110; // [rsp+8h] [rbp-E0h]
  long long v111; // [rsp+10h] [rbp-D8h]
  char *v112; // [rsp+10h] [rbp-D8h]
  long long v113; // [rsp+10h] [rbp-D8h]
  long long v114; // [rsp+10h] [rbp-D8h]
  long long v115; // [rsp+10h] [rbp-D8h]
  char *s2c; // [rsp+18h] [rbp-D0h]
  void *s2d; // [rsp+18h] [rbp-D0h]
  unsigned char *s2; // [rsp+18h] [rbp-D0h]
  long long *s2a; // [rsp+18h] [rbp-D0h]
  unsigned int s2b; // [rsp+18h] [rbp-D0h]
  long long v121; // [rsp+20h] [rbp-C8h]
  long long v122; // [rsp+20h] [rbp-C8h]
  char v123; // [rsp+20h] [rbp-C8h]
  unsigned long long v124; // [rsp+28h] [rbp-C0h]
  long long v125; // [rsp+28h] [rbp-C0h]
  char *ptr; // [rsp+30h] [rbp-B8h]
  unsigned int v127; // [rsp+38h] [rbp-B0h]
  long long v128; // [rsp+38h] [rbp-B0h]
  unsigned char *v129; // [rsp+40h] [rbp-A8h]
  long long v130; // [rsp+48h] [rbp-A0h]
  long long v131; // [rsp+48h] [rbp-A0h]
  long long v132; // [rsp+50h] [rbp-98h]
  long long v133; // [rsp+58h] [rbp-90h]
  long long v134; // [rsp+58h] [rbp-90h]
  unsigned long long v135; // [rsp+60h] [rbp-88h]
  unsigned char *v136; // [rsp+68h] [rbp-80h]
  unsigned long long v137; // [rsp+70h] [rbp-78h]
  long long v138; // [rsp+78h] [rbp-70h]
  long long v139; // [rsp+80h] [rbp-68h]
  long long v140; // [rsp+88h] [rbp-60h]
  long long v141; // [rsp+90h] [rbp-58h]
  int v143; // [rsp+A0h] [rbp-48h]
  char v144; // [rsp+A6h] [rbp-42h]
  unsigned char v145; // [rsp+A7h] [rbp-41h]

  v2 = a2 | byte_26568;
  v3 = sub_B920(a1, a2 | byte_26568);
  v145 = a2 | v3;
  if ( *(int *)a1 != *(int *)(a1 + 304) )
  {
    v145 = sub_B920(a1 + 304, a2 | v3 | v2) | a2 | v3;
    if ( v145 )
      return v145;
LABEL_5:
    sub_B9E0((unsigned int *)a1);
    sub_B7B0(a1);
    if ( *(int *)a1 == *(int *)(a1 + 304) )
    {
      v5 = *(long long *)(a1 + 176);
      v6 = *(char **)(a1 + 160);
      *(long long *)(a1 + 472) = *(long long *)(a1 + 168);
      v7 = *(char *)(a1 + 288);
      v8 = v5;
      *(long long *)(a1 + 464) = v6;
      *(long long *)(a1 + 480) = v5;
      *(char *)(a1 + 592) = v7;
    }
    else
    {
      sub_B9E0((unsigned int *)(a1 + 304));
      sub_B7B0(a1 + 304);
      v11 = *(char **)(a1 + 160);
      v6 = *(char **)(a1 + 464);
      v5 = *(long long *)(a1 + 176);
      v8 = *(long long *)(a1 + 480);
      if ( v11 != v6 )
      {
        v10 = &v11[v5];
        v12 = &v6[v8];
        if ( v5 < v8 )
          *v10 = ~v6[v5];
        else
          *v12 = ~v11[v8];
        v13 = v6;
        for ( i = v11; *(long long *)i == *(long long *)v13; v13 += 8 )
          i += 8;
        for ( ; *i == *v13; ++v13 )
          ++i;
        v14 = dword_26580 - 4;
        if ( (unsigned int)(dword_26580 - 4) > 1
          && i > &v11[v5 - *(unsigned char *)(a1 + 288)] != v13 > &v6[v8 - *(unsigned char *)(a1 + 592)] )
        {
          --i;
          --v13;
        }
LABEL_8:
        v15 = qword_26560;
        for ( j = qword_26560; i != v11; --v13 )
        {
          while ( *(i - 1) != 10 )
          {
            --i;
            --v13;
            if ( i == v11 )
              goto LABEL_14;
          }
          if ( !j )
            break;
          --i;
          --j;
        }
LABEL_14:
        *(long long *)(a1 + 224) = i;
        *(long long *)(a1 + 528) = v13;
        if ( v14 <= 1 || (v17 = v12, v18 = v10, *(char *)(a1 + 288) == *(char *)(a1 + 592)) )
        {
          v17 = v12;
          v42 = v10;
          v43 = &i[v5 - v8];
          if ( v8 > v5 )
            v43 = i;
          while ( v42 != v43 && *(v42 - 1) == *(v17 - 1) )
          {
            --v17;
            --v42;
          }
          if ( v42 == v11 || *(v42 - 1) == 10 )
          {
            if ( v17 != v6 )
              v15 += *(v17 - 1) != 10;
          }
          else
          {
            ++v15;
          }
          v44 = v15 - 1;
          v18 = v42;
          if ( v15 && v42 != v10 )
          {
            do
            {
              do
                ++v18;
              while ( *(v18 - 1) != 10 );
              v45 = v44-- != 0;
            }
            while ( v45 && v18 != v10 );
            v17 += v18 - v42;
          }
        }
        v19 = byte_26578 == 0;
        *(long long *)(a1 + 240) = v18;
        *(long long *)(a1 + 544) = v17;
        if ( v19 || buffer.fastmap || (v97 = qword_26570, qword_26570 > 0x1FFFFFFFFFFFFFFELL) || qword_26570 >= v5 )
        {
          v20 = v5 >> 5;
          v21 = -1;
          if ( !v20 )
            v20 = 1;
          if ( v20 > 0x787878787878782LL )
            v20 = 0x787878787878782LL;
          v22 = 0;
          v23 = v20 + 5;
          v24 = v23;
        }
        else
        {
          v22 = 1;
          v98 = (unsigned long long)(v18 - i) >> 5;
          if ( !v98 )
            v98 = 1;
          if ( v98 > 0x787878787878782LL )
            v98 = 0x787878787878782LL;
          v99 = v98 + 5;
          v100 = (unsigned long long)(v10 - v18) >> 5;
          if ( !v100 )
            v100 = 1;
          if ( v100 > 0x787878787878782LL )
            v100 = 0x787878787878782LL;
          v21 = 0;
          v101 = v100 + 5;
          if ( qword_26570 > 0 )
          {
            do
              v22 *= 2;
            while ( qword_26570 >= v22 );
            v21 = v22 - 1;
          }
          v102 = v22 + v99;
          if ( qword_26570 > v101 )
            v97 = v101;
          v24 = v97 + v102;
          v23 = v24;
        }
        v121 = v21;
        s2c = v12;
        v111 = v22;
        v104 = (long long)v17;
        v25 = sub_16A40(8 * v23);
        v27 = v104;
        v28 = v111;
        v29 = v25;
        v30 = (unsigned long long)s2c;
        v31 = v121;
        v32 = *(char **)(a1 + 224);
        if ( byte_26578 && v18 == v32 && (v48 = *(long long *)(a1 + 528), v104 == v48) )
        {
          v32 = v11;
        }
        else
        {
          v33 = v11;
          if ( v11 != v32 )
          {
            v105 = v6;
            v34 = v29;
            v35 = v24;
            v36 = v121;
            v112 = v11;
            v37 = 0;
            v38 = *(char **)(a1 + 224);
            do
            {
              v39 = v37++;
              v40 = v36 & v39;
              if ( v40 == v35 )
              {
                if ( v35 > 0x7FFFFFFFFFFFFFELL )
                  goto LABEL_218;
                v35 *= 2LL;
                v124 = v30;
                v23 = 8 * v35;
                v122 = v28;
                s2d = (void *)v27;
                v41 = sub_16A90(v34, 8 * v35);
                v30 = v124;
                v28 = v122;
                v27 = (long long)s2d;
                v34 = v41;
              }
              *(long long *)(v34 + 8 * v40) = v33;
              do
                ++v33;
              while ( *(v33 - 1) != 10 );
            }
            while ( v33 != v38 );
            v32 = v38;
            v46 = v37;
            v31 = v36;
            v24 = v35;
            v11 = v112;
            v29 = v34;
            v6 = v105;
            v47 = qword_26570;
            v48 = *(long long *)(a1 + 528);
            if ( !v28 )
            {
              v28 = v46;
LABEL_50:
              v34 = 32;
              v23 = v27 - v48;
              if ( v28 > 9 )
                v34 = (v32 - v11) / (unsigned long long)(v28 - 1);
LABEL_52:
              v113 = v31;
              v106 = v28;
              v49 = (v30 - v27) / v34;
              if ( !v49 )
                v49 = 1;
              if ( v49 > 0x787878787878782LL )
                v49 = 0x787878787878782LL;
              v50 = v49 + 5;
              if ( v50 <= v47 )
                v47 = v50;
              v51 = v23 / v34;
              if ( !(v23 / v34) )
                v51 = 1;
              v26 = 0x787878787878782LL;
              if ( v51 > 0x787878787878782LL )
                v51 = 0x787878787878782LL;
              v52 = v46 + v51 + 5 + v47;
              LOBYTE(v26) = v52 > 0xFFFFFFFFFFFFFFELL || v46 > (long long)v52;
              v145 = v26;
              if ( (char)v26 )
                goto LABEL_218;
              v34 = 8 * v52;
              v53 = sub_16A40(8 * v52);
              v28 = v106;
              v27 = v53;
              if ( v46 == v106 )
              {
                if ( !v46 )
                  goto LABEL_71;
              }
              else
              {
                if ( v46 <= 0 )
                  goto LABEL_71;
                v54 = v106 - qword_26570;
                v34 = v46 + v106 - qword_26570;
                v55 = v27 + 8 * (qword_26570 - v106);
                do
                {
                  *(long long *)(v55 + 8 * v54) = *(long long *)(v29 + 8 * (v54 & v113));
                  ++v54;
                }
                while ( v34 != v54 );
                for ( k = 0; k != v46; ++k )
                  *(long long *)(v29 + 8 * k) = *(long long *)(v27 + 8 * k);
              }
              v57 = 0;
              do
              {
                *(long long *)(v27 + 8 * v57) = &v6[*(long long *)(v29 + 8 * v57)] - v11;
                ++v57;
              }
              while ( v46 > v57 );
LABEL_71:
              v23 = a1;
              v58 = v24 - v46;
              v59 = v52 - v46;
              v26 = 0x3FFFFFFFFFFFFFELL;
              *(long long *)(a1 + 488) = v27 + 8 * v46;
              *(long long *)(a1 + 184) = 8 * v46 + v29;
              *(long long *)(a1 + 496) = -v46;
              *(long long *)(a1 + 192) = -v46;
              v60 = v58 + v59 + 1;
              *(long long *)(a1 + 216) = v58;
              *(long long *)(a1 + 520) = v59;
              *(long long *)(a1 + 536) = v106;
              *(long long *)(a1 + 232) = v106;
              qword_26700 = v60;
              if ( v60 > 0x3FFFFFFFFFFFFFELL )
                goto LABEL_218;
              v34 = 32 * v60;
              v61 = sub_16A40(32 * v60);
              qword_26708 = 1;
              qword_26710 = v61;
              if ( (unsigned long long)(qword_26700 / 3) > 0x200 )
              {
                LODWORD(v27) = 9;
                v23 = 1;
                do
                  LODWORD(v27) = v27 + 1;
                while ( 1LL << v27 < (unsigned long long)(qword_26700 / 3) );
                v27 = (int)v27;
                v103 = (1LL << v27) - byte_1E1A0[(int)v27];
                v26 = 0xFFFFFFFFFFFFFFELL;
                qword_26718 = v103;
                if ( v103 > 0xFFFFFFFFFFFFFFELL )
                  goto LABEL_218;
                v62 = 8 * v103 + 8;
              }
              else
              {
                qword_26718 = 509;
                v62 = 4080;
              }
              qword_26720 = sub_FAB0(v62) + 8;
              v140 = a1 + 160;
LABEL_75:
              v63 = *(long long *)(v140 + 32);
              v64 = *(unsigned char **)(v140 + 64);
              v130 = *(long long *)(v140 + 24);
              v141 = v63;
              v133 = *(long long *)(v140 + 56);
              v65 = sub_16A40(8 * v133);
              v34 = (unsigned int)dword_26558;
              v23 = (unsigned char)byte_26552;
              v132 = v65;
              v27 = *(long long *)(v140 + 80);
              v123 = byte_26552;
              ptr = (char *)qword_26710;
              v137 = v27;
              v125 = qword_26708;
              v135 = qword_26700;
              v136 = (unsigned char *)(*(long long *)v140 + *(long long *)(v140 + 16));
              v144 = byte_26552 | (dword_26558 != 0);
              v138 = 8 * v63;
              v139 = -8 * v63;
              if ( (unsigned long long)v64 >= v27 )
              {
                v114 = 0;
                v68 = v64;
LABEL_161:
                v92 = v114;
                v93 = 0;
                *(long long *)(v140 + 40) = v114;
                v27 = v133;
                v94 = v130;
                while ( 1 )
                {
                  if ( v27 == v92 )
                  {
                    v26 = 0x2AAAAAAAAAAAAAA9LL;
                    if ( v27 > 0x2AAAAAAAAAAAAAA9LL )
                      goto LABEL_218;
                    v26 = 0xFFFFFFFFFFFFFFELL;
                    v96 = 2 * v27 - v141;
                    if ( v96 > 0xFFFFFFFFFFFFFFELL )
                      goto LABEL_218;
                    v27 -= v141;
                    if ( (unsigned long long)v27 > 0xFFFFFFFFFFFFFFELL )
                      goto LABEL_218;
                    v34 = v94 + v138;
                    v23 = 8 * (v96 - v141);
                    v94 = v139 + sub_16A90(v94 + v138, v23);
                    v27 = v96;
                    v95 = (unsigned char **)(v94 + 8 * v92);
                    *v95 = v68;
                    if ( v136 == v68 )
                    {
LABEL_173:
                      v131 = v94;
                      v115 = v92;
                      v134 = v27;
                      if ( *(char *)(v140 + 128) && (unsigned int)(dword_26580 - 4) > 1 )
                        *v95 = v136 - 1;
LABEL_176:
                      *(long long *)(v140 + 24) = v131;
                      qword_26700 = v135;
                      *(long long *)(v140 + 48) = v115;
                      qword_26708 = v125;
                      *(long long *)(v140 + 56) = v134;
                      *(long long *)(v140 + 88) = v132;
                      v140 += 304;
                      qword_26710 = (long long)ptr;
                      if ( a1 + 768 == v140 )
                      {
                        *(long long *)(a1 + 600) = v125;
                        *(long long *)(a1 + 296) = v125;
                        free(ptr);
                        free((void *)(qword_26720 - 8));
                        return v145;
                      }
                      goto LABEL_75;
                    }
                  }
                  else
                  {
                    v95 = (unsigned char **)(v94 + 8 * v92);
                    *v95 = v68;
                    if ( v136 == v68 )
                      goto LABEL_173;
                  }
                  if ( qword_26570 <= v93 && byte_26578 )
                  {
                    v131 = v94;
                    v115 = v92;
                    v134 = v27;
                    goto LABEL_176;
                  }
                  ++v92;
                  do
                    ++v68;
                  while ( *(v68 - 1) != 10 );
                  ++v93;
                }
              }
              v66 = dword_26558;
              v34 = dword_26558 & 1;
              v67 = *v64;
              v68 = v64 + 1;
              v114 = 0;
              v143 = dword_26558 & 1;
              v69 = (unsigned int)dword_26558 <= 4;
              if ( dword_26558 == 4 )
                goto LABEL_104;
LABEL_77:
              if ( v69 )
              {
                if ( v66 - 1 <= 2 )
                {
                  if ( (char)v67 != 10 )
                  {
                    v70 = v66;
                    s2 = v64;
                    v127 = v66;
                    v71 = 0;
                    v72 = v68;
                    v73 = (unsigned char)v67;
                    v107 = qword_26498;
                    LOBYTE(v68) = v67;
                    v74 = v70 & 2;
                    v75 = 0;
                    while ( 1 )
                    {
                      if ( v74 )
                      {
                        v76 = __ctype_b_loc();
                        v27 = (long long)*v76;
                        if ( ((*v76)[(unsigned char)v73] & 0x2000) != 0 )
                        {
                          v34 = (unsigned long long)v72;
                          do
                          {
                            v88 = *(unsigned char *)v34++;
                            if ( (char)v88 == 10 )
                            {
                              v78 = v71;
                              v64 = s2;
                              v66 = v127;
                              v68 = (unsigned char *)v34;
                              m = v34 - (long long)s2 - 1;
                              goto LABEL_93;
                            }
                          }
                          while ( (*(char *)(v27 + 2 * v88 + 1) & 0x20) != 0 );
                        }
                      }
                      if ( !v143 )
                        goto LABEL_125;
                      if ( (char)v68 == 9 )
                      {
                        v73 = 32;
                        v27 = v107 - v75 % v107;
                        v89 = __CFADD__(v27, v75);
                        v75 += v27;
                        v77 = v27;
                        if ( v89 )
                          v75 = 0;
                        goto LABEL_88;
                      }
                      if ( (char)v68 == 13 )
                      {
                        v73 = 13;
                        v77 = 1;
                        v75 = 0;
                        goto LABEL_88;
                      }
                      if ( (char)v68 == 8 )
                        break;
                      ++v75;
                      v77 = 1;
LABEL_88:
                      if ( v123 )
                        v73 = LOBYTE((*__ctype_tolower_loc())[v73]);
                      do
                      {
                        v71 = v73 + __ROL8__(v71, 7);
                        --v77;
                      }
                      while ( v77 );
                      v68 = (unsigned char *)*v72++;
                      v73 = (unsigned char)v68;
                      if ( (char)v68 == 10 )
                      {
                        v68 = v72;
                        v78 = v71;
                        v64 = s2;
                        v66 = v127;
                        m = v68 - s2 - 1;
                        goto LABEL_93;
                      }
                    }
                    v73 = 8;
                    v75 = (v75 == 0) + v75 - 1;
LABEL_125:
                    v77 = 1;
                    goto LABEL_88;
                  }
LABEL_159:
                  m = 0;
                  v78 = 0;
                  goto LABEL_93;
                }
LABEL_147:
                if ( !v123 )
                {
                  v78 = 0;
                  m = 0;
                  if ( (char)v67 != 10 )
                  {
                    do
                    {
                      ++v68;
                      v78 = v67 + __ROL8__(v78, 7);
                      v67 = *(v68 - 1);
                    }
                    while ( (char)v67 != 10 );
                    m = v68 - v64 - 1;
                  }
                  goto LABEL_93;
                }
                if ( (char)v67 != 10 )
                {
                  v78 = 0;
                  v90 = *__ctype_tolower_loc();
                  do
                  {
                    ++v68;
                    v91 = v90[v67];
                    v67 = *(v68 - 1);
                    v78 = v91 + __ROL8__(v78, 7);
                  }
                  while ( (char)v67 != 10 );
                  m = v68 - v64 - 1;
                  goto LABEL_93;
                }
                goto LABEL_159;
              }
              if ( v66 != 5 )
                goto LABEL_147;
              if ( (char)v67 == 10 )
                goto LABEL_159;
              v110 = v64;
              v78 = 0;
              v87 = *__ctype_b_loc();
              do
              {
                if ( (v87[v67] & 0x2000) == 0 )
                {
                  if ( v123 )
                    v67 = (*__ctype_tolower_loc())[v67];
                  v78 = v67 + __ROL8__(v78, 7);
                }
                v67 = *v68++;
              }
              while ( (char)v67 != 10 );
              v64 = v110;
              for ( m = v68 - v110 - 1; ; m = v68 - v109 - 1 )
              {
LABEL_93:
                v23 = qword_26720;
                s2a = (long long *)(qword_26720 + 8 * (v78 % qword_26718));
                if ( v136 == v68 && *(char *)(v140 + 128) && (unsigned int)(dword_26580 - 4) > 1 )
                {
                  v23 = qword_26720 - 8;
                  if ( v66 > 1 )
                    v23 = qword_26720 + 8 * (v78 % qword_26718);
                  s2a = (long long *)v23;
                }
                v128 = *s2a;
                v108 = *s2a;
                if ( *s2a )
                  break;
LABEL_118:
                if ( v135 == v125 )
                {
                  v26 = 0x1FFFFFFFFFFFFFELL;
                  if ( v135 > 0x1FFFFFFFFFFFFFELL )
                    goto LABEL_218;
                  v34 = (unsigned long long)ptr;
                  v135 *= 2LL;
                  v23 = 32 * v135;
                  ptr = (char *)sub_16A90(ptr, 32 * v135);
                  v128 = *s2a;
                }
                v26 = v125++;
                v108 = v26;
                v85 = &ptr[32 * v26];
                *(long long *)v85 = v128;
                v27 = v133;
                *((long long *)v85 + 1) = v78;
                *((long long *)v85 + 2) = v64;
                *((long long *)v85 + 3) = m;
                *s2a = v26;
                if ( v114 == v133 )
                  goto LABEL_120;
LABEL_102:
                *(long long *)(v130 + 8 * v114) = v64;
                v27 = v132;
                *(long long *)(v132 + 8 * v114++) = v108;
                if ( v137 <= (unsigned long long)v68 )
                  goto LABEL_161;
                v64 = v68;
                v67 = *v68++;
                v69 = v66 <= 4;
                if ( v66 != 4 )
                  goto LABEL_77;
LABEL_104:
                if ( (char)v67 == 10 )
                  goto LABEL_159;
                v78 = 0;
                v109 = v64;
                s2b = v66;
                v82 = *__ctype_b_loc();
                do
                {
                  v83 = v68;
                  if ( (v82[v67] & 0x2000) != 0 )
                  {
                    v84 = *v68++;
                    if ( (char)v84 == 10 )
                      break;
                    while ( (v82[v84] & 0x2000) != 0 )
                    {
                      v84 = *v68++;
                      if ( (char)v84 == 10 )
                        goto LABEL_113;
                    }
                    v67 = v84;
                    v83 = v68;
                    v78 = __ROL8__(v78, 7) + 32LL;
                  }
                  if ( v123 )
                    v67 = (*__ctype_tolower_loc())[v67];
                  ++v68;
                  v78 = v67 + __ROL8__(v78, 7);
                  v67 = *v83;
                }
                while ( (char)v67 != 10 );
LABEL_113:
                v64 = v109;
                v66 = s2b;
              }
              v129 = v68;
              while ( 1 )
              {
                v80 = (long long *)&ptr[32 * v108];
                if ( v78 != v80[1] )
                  goto LABEL_96;
                v81 = v80[2];
                if ( v80[3] != m )
                  break;
                v23 = (unsigned long long)v64;
                v34 = v80[2];
                if ( !memcmp((const void *)v34, v64, m) )
                {
LABEL_101:
                  v68 = v129;
                  v27 = v133;
                  if ( v114 != v133 )
                    goto LABEL_102;
LABEL_120:
                  v23 = 0x2AAAAAAAAAAAAAA9LL;
                  if ( v114 <= 0x2AAAAAAAAAAAAAA9LL )
                  {
                    v34 = 0xFFFFFFFFFFFFFFELL;
                    v86 = 2 * v114 - v141;
                    if ( v86 <= 0xFFFFFFFFFFFFFFELL && (unsigned long long)(v114 - v141) <= 0xFFFFFFFFFFFFFFELL )
                    {
                      v34 = v138 + v130;
                      v132 = sub_16A90(v132, 8 * v86);
                      v23 = 8 * (v86 - v141);
                      v133 = 2 * v114 - v141;
                      v130 = v139 + sub_16A90(v138 + v130, v23);
                      goto LABEL_102;
                    }
                  }
LABEL_218:
                  sub_16EC0(v34, v23, v26, v27, v30, v28);
                }
                if ( v144 )
                {
LABEL_100:
                  v23 = (unsigned long long)v64;
                  v34 = v81;
                  if ( !(unsigned char)sub_EC00(v81, v64) )
                    goto LABEL_101;
LABEL_96:
                  v108 = *v80;
                  if ( !*v80 )
                    goto LABEL_117;
                }
                else
                {
                  v108 = *v80;
                  if ( !*v80 )
                  {
LABEL_117:
                    v68 = v129;
                    goto LABEL_118;
                  }
                }
              }
              if ( !v66 )
                goto LABEL_96;
              goto LABEL_100;
            }
LABEL_48:
            v28 = v46;
            if ( v47 <= v46 )
              v46 = v47;
            goto LABEL_50;
          }
          v48 = *(long long *)(a1 + 528);
        }
        v47 = qword_26570;
        if ( !v111 )
        {
          v46 = 0;
          v34 = 32;
          v23 = v104 - v48;
          goto LABEL_52;
        }
        v46 = 0;
        goto LABEL_48;
      }
    }
    i = &v6[v8];
    v10 = &v6[v5];
    v11 = v6;
    v12 = &v6[v8];
    v13 = &v6[v8];
    v14 = dword_26580 - 4;
    goto LABEL_8;
  }
  *(long long *)(a1 + 464) = *(long long *)(a1 + 160);
  *(long long *)(a1 + 472) = *(long long *)(a1 + 168);
  *(long long *)(a1 + 480) = *(long long *)(a1 + 176);
  if ( !v145 )
    goto LABEL_5;
  return v145;
}



// Function: print_normal_hunk @ 0xcd40
unsigned long long print_normal_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  int v5; // r12d
  int v6; // esi
  char *IO_write_ptr; // rax
  char *v8; // rax
  long long v9; // rbp
  long long v10; // rbx
  long long v11; // rsi
  char *v12; // rax
  long long v13; // rbp
  long long v14; // rbx
  long long v15; // rsi
  char *v16; // rax
  long long v17; // [rsp+8h] [rbp-50h] BYREF
  long long v18; // [rsp+10h] [rbp-48h] BYREF
  long long v19; // [rsp+18h] [rbp-40h] BYREF
  long long v20; // [rsp+20h] [rbp-38h] BYREF
  unsigned long long v21; // [rsp+28h] [rbp-30h]

  v21 = __readfsqword(0x28u);
  v1 = sub_F670(a1, &v17, &v18, &v19, &v20);
  if ( !v1 )
    return v21 - __readfsqword(0x28u);
  v5 = v1;
  sub_E830(a1, &v17, v2, v3);
  sub_F510(4);
  sub_F610(44, &unk_261A0, v17, v18);
  v6 = byte_1E550[v5];
  IO_write_ptr = stream->_IO_write_ptr;
  if ( IO_write_ptr >= stream->_IO_write_end )
  {
    __overflow(stream, v6);
  }
  else
  {
    stream->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = v6;
  }
  sub_F610(44, &unk_262D0, v19, v20);
  sub_F510(3);
  v8 = stream->_IO_write_ptr;
  if ( v8 >= stream->_IO_write_end )
  {
    __overflow(stream, 10);
  }
  else
  {
    stream->_IO_write_ptr = v8 + 1;
    *v8 = 10;
  }
  if ( (v5 & 1) != 0 )
  {
    v9 = v17;
    if ( v17 <= v18 )
    {
      v10 = 8 * v17;
      do
      {
        while ( 1 )
        {
          sub_F510(2);
          v11 = v10 + qword_26258;
          v10 += 8;
          sub_F330("<", v11, 1);
          sub_F510(3);
          if ( *(char *)(*(long long *)(qword_26258 + v10) - 1LL) == 10 )
            break;
LABEL_10:
          if ( v18 < ++v9 )
            goto LABEL_14;
        }
        v12 = stream->_IO_write_ptr;
        if ( v12 >= stream->_IO_write_end )
        {
          __overflow(stream, 10);
          goto LABEL_10;
        }
        ++v9;
        stream->_IO_write_ptr = v12 + 1;
        *v12 = 10;
      }
      while ( v18 >= v9 );
    }
  }
LABEL_14:
  if ( v5 == 3 )
  {
    fputs_unlocked("---\n", stream);
  }
  else if ( (v5 & 2) == 0 )
  {
    return v21 - __readfsqword(0x28u);
  }
  v13 = v19;
  if ( v19 <= v20 )
  {
    v14 = 8 * v19;
    do
    {
      sub_F510(1);
      v15 = v14 + qword_26388;
      v14 += 8;
      sub_F330(">", v15, 1);
      sub_F510(3);
      if ( *(char *)(*(long long *)(qword_26388 + v14) - 1LL) == 10 )
      {
        v16 = stream->_IO_write_ptr;
        if ( v16 >= stream->_IO_write_end )
        {
          __overflow(stream, 10);
        }
        else
        {
          stream->_IO_write_ptr = v16 + 1;
          *v16 = 10;
        }
      }
      ++v13;
    }
    while ( v20 >= v13 );
  }
  return v21 - __readfsqword(0x28u);
}



// Function: print_normal_script @ 0xcff0
long long print_normal_script(long long a1)
{
  return sub_F080(a1, sub_F060, sub_CD40);
}



// Function: tab_from_to @ 0xd010
unsigned long long tab_from_to(unsigned long long a1, unsigned long long a2)
{
  unsigned long long v2; // r13
  _IO_FILE *v3; // rbx
  long long v4; // rbp
  unsigned long long v5; // rax
  char *IO_write_ptr; // rdx
  unsigned long long v7; // rbp
  char *v8; // rax

  v2 = a1;
  v3 = stream;
  if ( !byte_264A0 )
  {
    v4 = qword_26498;
    v5 = qword_26498 + a1 - a1 % qword_26498;
    if ( v5 <= a2 )
    {
      do
      {
        IO_write_ptr = v3->_IO_write_ptr;
        v2 = v5;
        if ( IO_write_ptr >= v3->_IO_write_end )
        {
          __overflow(v3, 9);
        }
        else
        {
          v3->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = 9;
        }
        v5 = v4 + v2;
      }
      while ( a2 >= v4 + v2 );
    }
  }
  v7 = v2 + 1;
  if ( a2 > v2 )
  {
    while ( 1 )
    {
      v8 = v3->_IO_write_ptr;
      if ( v8 >= v3->_IO_write_end )
      {
        __overflow(v3, 32);
      }
      else
      {
        v3->_IO_write_ptr = v8 + 1;
        *v8 = 32;
      }
      if ( a2 == v7 )
        break;
      ++v7;
    }
  }
  return a2;
}



// Function: print_half_line @ 0xd0d0
size_t print_half_line(long long a1, unsigned long long a2, size_t a3)
{
  size_t v3; // r10
  unsigned char *v4; // rbp
  unsigned long long v5; // r14
  _IO_FILE *v6; // r15
  size_t v7; // r12
  unsigned char v8; // bl
  unsigned char *v9; // r9
  unsigned char *IO_write_ptr; // rax
  long long v12; // rax
  char *v13; // rax
  unsigned long long v14; // rcx
  char *v15; // rax
  char *v16; // rax
  unsigned long long v17; // rbx
  char *v18; // rax
  int v19; // eax
  size_t v20; // rdx
  char *v21; // rax
  char *v22; // rax
  size_t na; // [rsp+18h] [rbp-70h]
  size_t n; // [rsp+18h] [rbp-70h]
  size_t nb; // [rsp+18h] [rbp-70h]
  size_t nc; // [rsp+18h] [rbp-70h]
  size_t nd; // [rsp+18h] [rbp-70h]
  size_t v29; // [rsp+20h] [rbp-68h]
  unsigned long long v30; // [rsp+20h] [rbp-68h]
  size_t v31; // [rsp+20h] [rbp-68h]
  unsigned long long v32; // [rsp+20h] [rbp-68h]
  size_t v33; // [rsp+28h] [rbp-60h]
  wchar_t c; // [rsp+3Ch] [rbp-4Ch] BYREF
  long long v35[9]; // [rsp+40h] [rbp-48h] BYREF

  v3 = 0;
  v4 = *(unsigned char **)a1;
  v5 = *(long long *)(a1 + 8);
  v6 = stream;
  v35[1] = __readfsqword(0x28u);
  v35[0] = 0;
  if ( (unsigned long long)v4 >= v5 )
    return v3;
  v7 = 0;
  while ( 1 )
  {
    v8 = *v4;
    v9 = v4 + 1;
    if ( (char)*v4 <= 95 )
      break;
    if ( (unsigned char)(v8 - 97) > 0x1Du )
      goto LABEL_11;
LABEL_7:
    if ( a3 <= v7 )
    {
      ++v4;
      ++v7;
LABEL_15:
      if ( v5 <= (unsigned long long)v4 )
        return v3;
    }
    else
    {
      IO_write_ptr = (unsigned char *)v6->_IO_write_ptr;
      if ( (char *)IO_write_ptr >= v6->_IO_write_end )
      {
        __overflow(v6, v8);
        v3 = v7 + 1;
        ++v4;
        ++v7;
        goto LABEL_15;
      }
      ++v4;
      v3 = ++v7;
      v6->_IO_write_ptr = (char *)(IO_write_ptr + 1);
      *IO_write_ptr = v8;
      if ( v5 <= (unsigned long long)v9 )
        return v3;
    }
  }
  if ( (char)v8 <= 7 )
  {
LABEL_11:
    v29 = v3;
    v12 = sub_17200(&c, v4, v5 - (long long)v4, v35);
    v9 = v4 + 1;
    v3 = v29;
    if ( (unsigned long long)(v12 - 1) <= 0xFFFFFFFFFFFFFFFCLL )
    {
      n = v12;
      v19 = wcwidth(c);
      v20 = n;
      v3 = v29;
      if ( v19 > 0 )
        v7 += v19;
      if ( a3 >= v7 )
      {
        fwrite_unlocked(v4, 1u, n, stdout);
        v20 = n;
        v3 = v7;
      }
      v4 += v20;
    }
    else
    {
LABEL_12:
      v4 = v9;
      if ( a3 > v7 )
      {
        v13 = v6->_IO_write_ptr;
        if ( v13 >= v6->_IO_write_end )
        {
          nb = v3;
          __overflow(v6, v8);
          v3 = nb;
        }
        else
        {
          v6->_IO_write_ptr = v13 + 1;
          *v13 = v8;
        }
      }
    }
    goto LABEL_15;
  }
  switch ( v8 )
  {
    case 8u:
      ++v4;
      if ( v7 )
      {
        if ( a3 > --v7 )
        {
          if ( v7 >= v3 )
          {
            if ( v7 > v3 )
            {
              do
              {
                v22 = v6->_IO_write_ptr;
                if ( v22 >= v6->_IO_write_end )
                {
                  v31 = v3;
                  nd = (size_t)v9;
                  __overflow(v6, 32);
                  v9 = (unsigned char *)nd;
                  v3 = v31;
                }
                else
                {
                  v6->_IO_write_ptr = v22 + 1;
                  *v22 = 32;
                }
                ++v3;
              }
              while ( v7 != v3 );
              v4 = v9;
            }
          }
          else
          {
            v16 = v6->_IO_write_ptr;
            if ( v16 >= v6->_IO_write_end )
            {
              __overflow(v6, 8);
              v3 = v7;
            }
            else
            {
              v3 = v7;
              v6->_IO_write_ptr = v16 + 1;
              *v16 = 8;
            }
          }
        }
      }
      goto LABEL_15;
    case 9u:
      v14 = qword_26498 - v7 % qword_26498;
      if ( v3 == v7 )
      {
        v17 = v14 + v3;
        if ( byte_264A0 )
        {
          if ( a3 <= v17 )
            v17 = a3;
          if ( v17 > v3 )
          {
            do
            {
              v21 = v6->_IO_write_ptr;
              if ( v21 >= v6->_IO_write_end )
              {
                v33 = v3;
                v30 = v14;
                nc = (size_t)v9;
                __overflow(v6, 32);
                v9 = (unsigned char *)nc;
                v14 = v30;
                v3 = v33;
              }
              else
              {
                v6->_IO_write_ptr = v21 + 1;
                *v21 = 32;
              }
              ++v3;
            }
            while ( v3 != v17 );
          }
        }
        else if ( a3 > v17 )
        {
          v18 = v6->_IO_write_ptr;
          if ( v18 >= v6->_IO_write_end )
          {
            v32 = qword_26498 - v7 % qword_26498;
            __overflow(v6, 9);
            v9 = v4 + 1;
            v14 = v32;
            v3 = v17;
          }
          else
          {
            v3 += v14;
            v6->_IO_write_ptr = v18 + 1;
            *v18 = 9;
          }
        }
      }
      v7 += v14;
      v4 = v9;
      goto LABEL_15;
    case 0xAu:
      return v3;
    case 0xBu:
    case 0xCu:
      goto LABEL_12;
    case 0xDu:
      v15 = v6->_IO_write_ptr;
      if ( v15 >= v6->_IO_write_end )
      {
        __overflow(v6, 13);
        v9 = v4 + 1;
      }
      else
      {
        v6->_IO_write_ptr = v15 + 1;
        *v15 = 13;
      }
      na = (size_t)v9;
      v7 = 0;
      sub_D010(0, a2);
      v4 = (unsigned char *)na;
      v3 = 0;
      goto LABEL_15;
    case 0x20u:
    case 0x21u:
    case 0x22u:
    case 0x23u:
    case 0x25u:
    case 0x26u:
    case 0x27u:
    case 0x28u:
    case 0x29u:
    case 0x2Au:
    case 0x2Bu:
    case 0x2Cu:
    case 0x2Du:
    case 0x2Eu:
    case 0x2Fu:
    case 0x30u:
    case 0x31u:
    case 0x32u:
    case 0x33u:
    case 0x34u:
    case 0x35u:
    case 0x36u:
    case 0x37u:
    case 0x38u:
    case 0x39u:
    case 0x3Au:
    case 0x3Bu:
    case 0x3Cu:
    case 0x3Du:
    case 0x3Eu:
    case 0x3Fu:
    case 0x41u:
    case 0x42u:
    case 0x43u:
    case 0x44u:
    case 0x45u:
    case 0x46u:
    case 0x47u:
    case 0x48u:
    case 0x49u:
    case 0x4Au:
    case 0x4Bu:
    case 0x4Cu:
    case 0x4Du:
    case 0x4Eu:
    case 0x4Fu:
    case 0x50u:
    case 0x51u:
    case 0x52u:
    case 0x53u:
    case 0x54u:
    case 0x55u:
    case 0x56u:
    case 0x57u:
    case 0x58u:
    case 0x59u:
    case 0x5Au:
    case 0x5Bu:
    case 0x5Cu:
    case 0x5Du:
    case 0x5Eu:
    case 0x5Fu:
      goto LABEL_7;
    default:
      goto LABEL_11;
  }
  return v3;
}



// Function: print_1sdiff_line @ 0xd500
int print_1sdiff_line(long long a1, int a2, char **a3)
{
  char *IO_write_ptr; // rax
  _IO_FILE *v5; // r14
  size_t v6; // r12
  char v7; // r15
  unsigned long long v8; // rbp
  unsigned long long v9; // rax
  unsigned long long v11; // [rsp+0h] [rbp-48h]
  char v12; // [rsp+Eh] [rbp-3Ah]
  char v13; // [rsp+Fh] [rbp-39h]

  LODWORD(IO_write_ptr) = qword_26428;
  v5 = stream;
  v13 = a2;
  v6 = qword_26430;
  v11 = qword_26428;
  if ( (char)a2 == 60 )
  {
    sub_F510(2);
    goto LABEL_19;
  }
  if ( (char)a2 == 62 )
  {
    sub_F510(1);
LABEL_19:
    v12 = 1;
    if ( !a1 )
    {
      v7 = 0;
      v8 = sub_D010(0, (v6 + v11 - 1) >> 1) + 1;
      goto LABEL_21;
    }
    goto LABEL_4;
  }
  v12 = 0;
  if ( !a1 )
  {
    v7 = 0;
    v8 = 0;
    if ( (char)a2 == 32 )
      goto LABEL_5;
    goto LABEL_14;
  }
LABEL_4:
  v7 = *(char *)(*(long long *)(a1 + 8) - 1LL) == 10;
  IO_write_ptr = (char *)sub_D0D0(a1, 0, v6);
  v8 = (unsigned long long)IO_write_ptr;
  if ( (char)a2 == 32 )
    goto LABEL_5;
LABEL_14:
  v8 = sub_D010(v8, (v6 + v11 - 1) >> 1) + 1;
  if ( (char)a2 == 124 )
  {
    a2 = 124;
    if ( (*(a3[1] - 1) == 10) != v7 )
    {
      a2 = v7 == 0 ? 92 : 47;
      v13 = v7 == 0 ? 92 : 47;
    }
    goto LABEL_22;
  }
LABEL_21:
  a2 = (char)a2;
LABEL_22:
  IO_write_ptr = v5->_IO_write_ptr;
  if ( IO_write_ptr >= v5->_IO_write_end )
  {
    LODWORD(IO_write_ptr) = __overflow(v5, a2);
  }
  else
  {
    v5->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = v13;
  }
LABEL_5:
  if ( a3 )
  {
    v7 |= *(a3[1] - 1) == 10;
    IO_write_ptr = *a3;
    if ( **a3 != 10 )
    {
      v9 = sub_D010(v8, v11);
      LODWORD(IO_write_ptr) = sub_D0D0((long long)a3, v9, v6);
    }
  }
  if ( v7 )
  {
    IO_write_ptr = v5->_IO_write_ptr;
    if ( IO_write_ptr >= v5->_IO_write_end )
    {
      LODWORD(IO_write_ptr) = __overflow(v5, 10);
    }
    else
    {
      v5->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = 10;
    }
  }
  if ( v12 )
    LODWORD(IO_write_ptr) = sub_F510(3);
  return (int)IO_write_ptr;
}



// Function: print_sdiff_common_lines @ 0xd700
int print_sdiff_common_lines(long long a1, long long a2)
{
  long long v3; // rbp
  long long v4; // rbx
  bool v5; // r14
  bool v6; // r15
  char **v7; // rdx
  long long v8; // rdi
  int result; // eax

  if ( !byte_26438 )
  {
    v3 = qword_26730;
    v4 = qword_26728;
    v5 = qword_26730 != a1;
    v6 = qword_26728 != a2;
    if ( qword_26730 != a1 || qword_26728 != a2 )
    {
      if ( byte_2643A )
        result = __fprintf_chk(stream, 1, "i%ld,%ld\n", a1 - qword_26730, a2 - qword_26728);
      if ( byte_26439 )
        goto LABEL_17;
      if ( v5 && v6 )
      {
        while ( 1 )
        {
          ++v4;
          ++v3;
          result = sub_D500(qword_26258 + 8 * v3 - 8, 32, (char **)(qword_26388 + 8 * v4 - 8));
          if ( a1 == v3 )
            break;
          if ( a2 == v4 )
            goto LABEL_17;
        }
      }
      if ( a2 == v4 )
        goto LABEL_17;
      do
      {
        v7 = (char **)(qword_26388 + 8 * v4++);
        result = sub_D500(0, 41, v7);
      }
      while ( a2 != v4 );
      while ( a1 != v3 )
      {
        v8 = qword_26258 + 8 * v3++;
        result = sub_D500(v8, 40, 0);
LABEL_17:
        ;
      }
    }
  }
  qword_26730 = a1;
  qword_26728 = a2;
  return result;
}



// Function: print_sdiff_hunk @ 0xd860
unsigned long long print_sdiff_hunk(long long a1)
{
  int v1; // eax
  int v3; // r12d
  long long i; // rbx
  char **v5; // rdx
  long long j; // rbx
  long long v7; // rdi
  long long v8; // rcx
  long long v9; // rbx
  long long v10; // rdx
  long long v11; // rbp
  long long v12; // rax
  char **v13; // rdx
  long long v14; // rdi
  long long v15; // [rsp+8h] [rbp-40h] BYREF
  long long v16; // [rsp+10h] [rbp-38h] BYREF
  long long v17; // [rsp+18h] [rbp-30h] BYREF
  long long v18; // [rsp+20h] [rbp-28h] BYREF
  unsigned long long v19; // [rsp+28h] [rbp-20h]

  v19 = __readfsqword(0x28u);
  v1 = sub_F670(a1, &v15, &v16, &v17, &v18);
  if ( v1 )
  {
    v3 = v1;
    sub_D700(v15, v17);
    if ( byte_2643A )
      __fprintf_chk(stream, 1, "c%ld,%ld\n", v16 - v15 + 1, v18 - v17 + 1);
    if ( v3 == 3 )
    {
      v8 = v18;
      v9 = v15;
      v10 = v16;
      v11 = v17;
      v12 = v18;
      if ( v15 <= v16 )
      {
        while ( 1 )
        {
          v12 = v8;
          if ( v8 < v11 )
            break;
          v13 = (char **)(qword_26388 + 8 * v11++);
          v14 = qword_26258 + 8 * v9++;
          sub_D500(v14, 124, v13);
          v10 = v16;
          if ( v16 < v9 )
          {
            v12 = v18;
            break;
          }
          v8 = v18;
        }
      }
      v15 = v9;
      qword_26730 = v9;
      v17 = v11;
      qword_26728 = v11;
      LOBYTE(v3) = (v10 >= v9) + 2 * (v11 <= v12);
    }
    if ( (v3 & 2) != 0 )
    {
      for ( i = v17; v18 >= i; ++i )
      {
        v5 = (char **)(qword_26388 + 8 * i);
        sub_D500(0, 62, v5);
      }
      qword_26728 = i;
    }
    if ( (v3 & 1) != 0 )
    {
      for ( j = v15; v16 >= j; ++j )
      {
        v7 = qword_26258 + 8 * j;
        sub_D500(v7, 60, 0);
      }
      qword_26730 = j;
    }
  }
  return v19 - __readfsqword(0x28u);
}



// Function: print_sdiff_script @ 0xda50
int print_sdiff_script(long long a1, long long a2, long long a3, long long a4)
{
  sub_E830(a1, a2, a3, a4);
  qword_26728 = -qword_26288;
  qword_26730 = -qword_26288;
  sub_F080(a1, sub_F060, sub_D860);
  return sub_D700(qword_26270, qword_263A0);
}



// Function: sighandler @ 0xdaa0
long long sighandler(int a1)
{
  long long result; // rax

  result = (unsigned int)sig;
  if ( !sig )
    sig = a1;
  return result;
}



// Function: get_funky_string @ 0xdac0
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



// Function: check_color_output @ 0xdd30
unsigned long long check_color_output(char a1)
{
  int v1; // eax
  long long v2; // rbp
  unsigned char *v3; // rbx
  void *v4; // r14
  char v5; // al
  char v6; // al
  bool v7; // zf
  long long v8; // r15
  const char *v9; // rsi
  long long *v10; // rcx
  char *v11; // rax
  char *v12; // rax
  void *v13; // rbx
  void *v14; // rdi
  int v16; // r15d
  unsigned char *v17; // rbx
  unsigned char *i; // rbp
  int v19; // ebp
  void (*v20)(int); // rax
  int v21; // eax
  long long *v22; // r12
  unsigned char *v23; // [rsp+10h] [rbp-F8h] BYREF
  void *v24; // [rsp+18h] [rbp-F0h] BYREF
  struct sigaction oact; // [rsp+20h] [rbp-E8h] BYREF
  char s1[3]; // [rsp+C5h] [rbp-43h] BYREF
  unsigned long long v27; // [rsp+C8h] [rbp-40h]

  v27 = __readfsqword(0x28u);
  if ( !stream )
    return v27 - __readfsqword(0x28u);
  v1 = dword_2657C;
  if ( !dword_2657C )
    return v27 - __readfsqword(0x28u);
  if ( byte_26180 )
  {
    LODWORD(v2) = 1;
  }
  else
  {
    if ( a1 )
    {
      if ( dword_2657C != 2 )
      {
        byte_26758 = 0;
        return v27 - __readfsqword(0x28u);
      }
      LODWORD(v2) = 0;
      goto LABEL_9;
    }
    v21 = fileno(stream);
    v7 = isatty(v21) == 0;
    v1 = dword_2657C;
    v2 = !v7;
  }
  if ( v1 != 2 )
  {
    if ( v1 != 1 || !v2 )
    {
LABEL_24:
      byte_26758 = 0;
      goto LABEL_25;
    }
    LODWORD(v2) = 1;
  }
LABEL_9:
  v3 = (unsigned char *)qword_26748;
  byte_26758 = 1;
  v23 = (unsigned char *)qword_26748;
  if ( qword_26748 && *(char *)qword_26748 )
  {
    strcpy(s1, "??");
    v24 = (void *)sub_16EA0(qword_26748);
    v4 = v24;
    while ( 1 )
    {
      while ( 1 )
      {
        v5 = *v3;
        if ( *v3 != 42 )
          break;
        v22 = (long long *)sub_16A40(40);
        v22[4] = (long long)ptr;
        ptr = v22;
        v22[1] = (long long)v24;
        v23 = v3 + 1;
        if ( !(unsigned char)sub_DAC0((long long *)&v24, &v23, 1u, v22) )
          goto LABEL_22;
        v7 = *v23++ == 61;
        if ( !v7 )
          goto LABEL_22;
        v22[3] = (long long)v24;
        if ( !(unsigned char)sub_DAC0((long long *)&v24, &v23, 0, v22 + 2) )
          goto LABEL_22;
LABEL_47:
        v3 = v23;
      }
      if ( v5 != 58 )
        break;
      v23 = ++v3;
    }
    if ( !v5 )
      goto LABEL_25;
    s1[0] = *v3;
    v6 = v3[1];
    v23 = v3 + 1;
    if ( v6 )
    {
      s1[1] = v6;
      v7 = v3[2] == 61;
      v23 = v3 + 3;
      if ( v7 )
      {
        v8 = 0;
        v9 = "lc";
        while ( strcmp(s1, v9) )
        {
          v9 = off_25920[++v8];
          if ( !v9 )
            goto LABEL_21;
        }
        v10 = (long long *)((char *)&unk_26020 + 16 * (int)v8);
        v10[1] = (long long)v24;
        if ( (unsigned char)sub_DAC0((long long *)&v24, &v23, 0, v10) )
          goto LABEL_47;
LABEL_21:
        v11 = dcgettext(0, "unrecognized prefix: %s", 5);
        error(0, 0, v11, s1);
      }
    }
LABEL_22:
    v12 = dcgettext(0, "unparsable value for --palette", 5);
    error(0, 0, v12);
    free(v4);
    v13 = ptr;
    while ( v13 )
    {
      v14 = v13;
      v13 = (void *)*((long long *)v13 + 4);
      free(v14);
    }
    goto LABEL_24;
  }
LABEL_25:
  if ( v2 )
  {
    v16 = 20;
    v17 = (unsigned char *)&unk_1E524;
    sigemptyset(&set);
    for ( i = (unsigned char *)&unk_1E524; ; i += 4 )
    {
      sigaction(v16, 0, &oact);
      if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
        sigaddset(&set, v16);
      if ( i == byte_1E550 )
        break;
      v16 = *(int *)i;
    }
    oact.sa_flags = 0x10000000;
    v19 = 20;
    *(__m128i *)oact.sa_mask.__val = _mm_load_si128((const __m128i *)&set);
    *(__m128i *)&oact.sa_mask.__val[2] = _mm_load_si128((const __m128i *)&set.__val[2]);
    *(__m128i *)&oact.sa_mask.__val[4] = _mm_load_si128((const __m128i *)&set.__val[4]);
    *(__m128i *)&oact.sa_mask.__val[6] = _mm_load_si128((const __m128i *)&set.__val[6]);
    *(__m128i *)&oact.sa_mask.__val[8] = _mm_load_si128((const __m128i *)&set.__val[8]);
    *(__m128i *)&oact.sa_mask.__val[10] = _mm_load_si128((const __m128i *)&set.__val[10]);
    *(__m128i *)&oact.sa_mask.__val[12] = _mm_load_si128((const __m128i *)&set.__val[12]);
    *(__m128i *)&oact.sa_mask.__val[14] = _mm_load_si128((const __m128i *)&set.__val[14]);
    while ( 1 )
    {
      if ( sigismember(&set, v19) )
      {
        v20 = (void (*)(int))sub_DAA0;
        if ( v19 == 20 )
          v20 = (void (*)(int))sub_E1A0;
        oact.sa_handler = v20;
        sigaction(v19, &oact, 0);
      }
      if ( v17 == byte_1E550 )
        break;
      v19 = *(int *)v17;
      v17 += 4;
    }
  }
  return v27 - __readfsqword(0x28u);
}



// Function: put_indicator @ 0xe180
size_t put_indicator(long long a1)
{
  return fwrite_unlocked(*(const void **)(a1 + 8), *(long long *)a1, 1u, stream);
}



// Function: process_signals @ 0xe1c0
unsigned long long process_signals()
{
  int v0; // r15d
  sigset_t v2; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v3; // [rsp+88h] [rbp-40h]

  v3 = __readfsqword(0x28u);
  while ( sig || dword_26770 )
  {
    if ( byte_26758 && dword_26010 != 3 )
    {
      sub_E180((long long)&unk_26020);
      sub_E180((long long)&unk_26050);
      sub_E180((long long)&unk_26050 - 32);
      dword_26010 = 3;
    }
    fflush_unlocked(stdout);
    sigprocmask(0, &set, &v2);
    v0 = sig;
    if ( dword_26770 )
    {
      v0 = 19;
      --dword_26770;
    }
    else
    {
      signal(sig, 0);
    }
    raise(v0);
    sigprocmask(2, &v2, 0);
  }
  return v3 - __readfsqword(0x28u);
}



// Function: c_escape @ 0xe2e0
char *c_escape(char *a1)
{
  char v1; // al
  char *v2; // r12
  char *v3; // rdx
  long long v4; // rsi
  char v5; // di
  char *v7; // rax
  char *v8; // rdi
  char *v9; // rdx
  char v10; // al

  v1 = *a1;
  v2 = a1;
  if ( !*a1 )
    return v2;
  v3 = a1;
  v4 = 0;
  v5 = 0;
  do
  {
    while ( 1 )
    {
      if ( v1 == 32 )
      {
        v5 = 1;
        goto LABEL_5;
      }
      if ( v1 != 34 )
      {
        if ( v1 <= 34 )
        {
          if ( (unsigned char)(v1 - 7) > 6u )
          {
            if ( v1 <= 31 )
              v4 += 3;
            goto LABEL_5;
          }
          goto LABEL_4;
        }
        if ( v1 != 92 )
          break;
      }
LABEL_4:
      ++v4;
LABEL_5:
      v1 = *++v3;
      if ( !v1 )
        goto LABEL_11;
    }
    v1 = *++v3;
  }
  while ( v1 );
LABEL_11:
  if ( !v4 && !v5 )
    return v2;
  v7 = (char *)sub_16A40(v4 + v3 - v2 + 3);
  *v7 = 34;
  v8 = v7;
  v9 = v7 + 1;
  v10 = *v2;
  if ( *v2 )
  {
    while ( 2 )
    {
      if ( v10 > 34 )
      {
        if ( v10 == 92 )
        {
LABEL_26:
          *v9 = 92;
          v9 += 2;
          *(v9 - 1) = v10;
          goto LABEL_21;
        }
      }
      else
      {
        if ( v10 > 6 )
        {
          switch ( v10 )
          {
            case 7:
              *v9 = 92;
              v9 += 2;
              *(v9 - 1) = 97;
              break;
            case 8:
              v10 = 98;
              goto LABEL_26;
            case 9:
              v10 = 116;
              goto LABEL_26;
            case 10:
              *v9 = 92;
              v9 += 2;
              *(v9 - 1) = 110;
              break;
            case 11:
              *v9 = 92;
              v9 += 2;
              *(v9 - 1) = 118;
              break;
            case 12:
              *v9 = 92;
              v9 += 2;
              *(v9 - 1) = 102;
              break;
            case 13:
              v10 = 114;
              goto LABEL_26;
            case 34:
              goto LABEL_26;
            default:
              goto LABEL_19;
          }
LABEL_21:
          v10 = *++v2;
          if ( !v10 )
            goto LABEL_22;
          continue;
        }
LABEL_19:
        if ( v10 <= 31 )
        {
          *v9 = 92;
          v9 += 4;
          *(v9 - 3) = ((unsigned char)v10 >> 6) + 48;
          *(v9 - 1) = (v10 & 7) + 48;
          *(v9 - 2) = ((v10 >> 3) & 7) + 48;
          goto LABEL_21;
        }
      }
      break;
    }
    *v9++ = v10;
    goto LABEL_21;
  }
LABEL_22:
  *(short *)v9 = 34;
  return v8;
}



// Function: perror_with_name @ 0xe490
void perror_with_name(const char *a1)
{
  int *v1; // rax

  v1 = __errno_location();
  error(0, *v1, "%s", a1);
}



// Function: message5 @ 0xe4c0
unsigned long long message5(const char *src, long long a2, long long a3, char *a4, char *a5)
{
  char *v7; // r12
  const char *v8; // rbp
  char *v9; // rbx
  long long v10; // r13
  long long v11; // rbx
  size_t v12; // rax
  char *IO_write_ptr; // rax
  char *v14; // rsi
  long long v16; // rbx
  long long *v17; // r14
  char *v18; // rcx
  size_t v19; // r13
  long long *v20; // rax
  long long v21[11]; // [rsp+0h] [rbp-88h]
  unsigned long long v22; // [rsp+58h] [rbp-30h]

  v7 = a5;
  v8 = src;
  v9 = a4;
  v22 = __readfsqword(0x28u);
  if ( byte_26480 )
  {
    v21[1] = a2;
    v10 = 8;
    if ( !a4 )
      v9 = "";
    v21[2] = a3;
    if ( !a5 )
      v7 = "";
    v21[3] = v9;
    v11 = 0;
    v21[4] = v7;
    while ( 1 )
    {
      v12 = strlen(src) + 1;
      v21[v11 + 6] = v12;
      ++v11;
      v10 += v12;
      if ( v11 == 5 )
        break;
      src = (const char *)v21[v11];
    }
    v16 = 0;
    v17 = (long long *)sub_16A40(v10);
    v18 = (char *)(v17 + 1);
    while ( 1 )
    {
      v19 = v21[v16 + 6];
      ++v16;
      v18 = (char *)memcpy(v18, v8, v19) + v19;
      if ( v16 == 5 )
        break;
      v8 = (const char *)v21[v16];
    }
    v20 = (long long *)off_260A0;
    off_260A0 = (long long)v17;
    *v20 = v17;
    *v17 = 0;
    return v22 - __readfsqword(0x28u);
  }
  else
  {
    if ( byte_2643A )
    {
      IO_write_ptr = stdout->_IO_write_ptr;
      if ( IO_write_ptr >= stdout->_IO_write_end )
      {
        __overflow(stdout, 32);
      }
      else
      {
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = 32;
      }
    }
    v14 = dcgettext(0, src, 5);
    return __printf_chk(1, v14, a2, a3, v9, v7);
  }
}



// Function: message @ 0xe660
unsigned long long message(const char *a1, long long a2, long long a3)
{
  return sub_E4C0(a1, a2, a3, 0, 0);
}



// Function: print_message_queue @ 0xe670
unsigned long long print_message_queue()
{
  char *v0; // rbp
  char *v1; // r13
  const char **v2; // r15
  const char *v3; // rbx
  long long v4; // r15
  long long v5; // rbx
  char *v6; // rax
  long long v8; // [rsp+8h] [rbp-80h]
  long long v9; // [rsp+10h] [rbp-78h]
  char v10; // [rsp+20h] [rbp-68h] BYREF
  long long v11; // [rsp+28h] [rbp-60h]
  long long v12; // [rsp+30h] [rbp-58h]
  long long v13; // [rsp+38h] [rbp-50h]
  long long v14; // [rsp+40h] [rbp-48h] BYREF
  unsigned long long v15; // [rsp+48h] [rbp-40h]

  v0 = (char *)qword_26800;
  v15 = __readfsqword(0x28u);
  if ( qword_26800 )
  {
    do
    {
      v1 = v0;
      v2 = (const char **)&v10;
      v0 = *(char **)v0;
      v3 = v1 + 8;
      do
      {
        ++v2;
        v3 += strlen(v3) + 1;
        *v2 = v3;
      }
      while ( &v14 != (long long *)v2 );
      v4 = v12;
      v5 = v11;
      v9 = v14;
      v8 = v13;
      v6 = dcgettext(0, v1 + 8, 5);
      __printf_chk(1, v6, v5, v4, v8, v9);
      free(v1);
    }
    while ( v0 );
  }
  return v15 - __readfsqword(0x28u);
}



// Function: pfatal_with_name @ 0xe770
long long pfatal_with_name(const char *a1)
{
  int v1; // ebp

  v1 = *__errno_location();
  sub_E670();
  error(2, v1, "%s", a1);
  return sub_E7B0((char *)&dword_0 + 2);
}



// Function: fatal @ 0xe7b0
long long fatal(char *msgid)
{
  char *v1; // rax

  sub_E670();
  v1 = dcgettext(0, msgid, 5);
  error(2, 0, "%s", v1);
  return sub_E7F0(2);
}



// Function: set_color_palette @ 0xe7f0
void set_color_palette(void *a1)
{
  qword_26748 = a1;
}



// Function: setup_output @ 0xe800
void setup_output(long long a1, long long a2, char a3)
{
  qword_26768 = a1;
  qword_26760 = a2;
  byte_26759 = a3;
  stream = 0;
}



// Function: begin_output @ 0xe830
long long begin_output()
{
  int v1; // r8d
  int v2; // r9d
  void *v3; // rbp
  __pid_t v4; // eax
  int *v5; // rax
  char *v6; // rax
  int pipedes[2]; // [rsp+8h] [rbp-60h] BYREF
  void *ptr; // [rsp+10h] [rbp-58h] BYREF
  void *v9; // [rsp+18h] [rbp-50h]
  char *argv[5]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v11; // [rsp+48h] [rbp-20h]

  v11 = __readfsqword(0x28u);
  if ( stream )
    return v11 - __readfsqword(0x28u);
  ptr = sub_E2E0((char *)qword_26768);
  v9 = sub_E2E0((char *)qword_26760);
  v3 = (void *)sub_17020((unsigned int)"diff%s %s %s", qword_26420, (int)ptr, (int)v9, v1, v2);
  if ( !byte_26480 )
  {
    stream = stdout;
    sub_DD30(0);
    if ( !byte_26759 )
    {
LABEL_5:
      free(v3);
      if ( dword_26580 == 2 )
      {
        sub_8B80((long long)&unk_261A0, (const char **)&ptr, 0);
      }
      else if ( dword_26580 == 3 )
      {
        sub_8B80((long long)&unk_261A0, (const char **)&ptr, 1);
      }
      if ( ptr != (void *)qword_26768 )
        free(ptr);
      if ( v9 != (void *)qword_26760 )
        free(v9);
      return v11 - __readfsqword(0x28u);
    }
LABEL_24:
    __printf_chk(1, "%s\n", (const char *)v3);
    goto LABEL_5;
  }
  if ( fflush_unlocked(stdout) )
  {
    v6 = dcgettext(0, "write failed", 5);
    sub_E770(v6);
    goto LABEL_26;
  }
  argv[2] = (char *)v3;
  argv[0] = "/usr/bin/pr";
  argv[1] = "-h";
  argv[3] = 0;
  if ( pipe(pipedes) )
  {
LABEL_27:
    sub_E770("pipe");
    goto LABEL_28;
  }
  v4 = fork();
  pid = v4;
  if ( v4 < 0 )
  {
LABEL_26:
    sub_E770("fork");
    goto LABEL_27;
  }
  if ( !v4 )
  {
    close(pipedes[1]);
    if ( pipedes[0] )
    {
      if ( dup2(pipedes[0], 0) < 0 )
      {
        sub_E770("dup2");
        goto LABEL_24;
      }
      close(pipedes[0]);
    }
    execv("/usr/bin/pr", argv);
    v5 = __errno_location();
    _exit((*v5 == 2) + 126);
  }
  close(pipedes[0]);
  stream = fdopen(pipedes[1], "w");
  if ( stream )
  {
    sub_DD30(1);
    goto LABEL_5;
  }
LABEL_28:
  sub_E770("fdopen");
  return sub_EAD0();
}



// Function: finish_output @ 0xead0
unsigned long long finish_output()
{
  unsigned int v0; // ebx
  const char *v2; // rsi
  char *v3; // rax
  char *v4; // rax
  int stat_loc; // [rsp+4h] [rbp-14h] BYREF
  unsigned long long v6; // [rsp+8h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  if ( stream && stream != stdout )
  {
    if ( (stream->_flags & 0x20) == 0 )
    {
      if ( fclose(stream) )
      {
        v4 = dcgettext(0, "write failed", 5);
        sub_E770(v4);
      }
      else if ( waitpid(pid, &stat_loc, 0) >= 0 )
      {
        if ( (stat_loc & 0x7F) == 0 )
        {
          v0 = BYTE1(stat_loc);
          if ( !BYTE1(stat_loc) )
            goto LABEL_8;
          v2 = "subsidiary program '%s' could not be invoked";
          if ( BYTE1(stat_loc) != 126 )
          {
            v2 = "subsidiary program '%s' failed (exit status %d)";
            if ( BYTE1(stat_loc) == 127 )
              v2 = "subsidiary program '%s' not found";
          }
LABEL_10:
          v3 = dcgettext(0, v2, 5);
          error(2, 0, v3, "/usr/bin/pr", v0);
        }
LABEL_14:
        v0 = 0x7FFFFFFF;
        v2 = "subsidiary program '%s' failed";
        goto LABEL_10;
      }
      sub_E770("waitpid");
    }
    sub_E7B0("write failed");
    goto LABEL_14;
  }
LABEL_8:
  stream = 0;
  return v6 - __readfsqword(0x28u);
}



// Function: lines_differ @ 0xec00
long long lines_differ(unsigned char *a1, unsigned char *a2)
{
  unsigned char *v2; // r15
  unsigned char *v3; // r14
  unsigned long long v4; // r12
  unsigned long long v5; // r13
  long long v6; // rcx
  long long v7; // rbx
  unsigned char *v8; // rsi
  unsigned char *v9; // rbp
  const unsigned short *v10; // rax
  long long v11; // rdx
  char *v12; // r11
  const int_t *v13; // rax
  long long v14; // rax
  const unsigned short **v15; // rax
  const unsigned short *v16; // rax
  long long v17; // rdx
  unsigned long long v18; // r9
  const unsigned short *v19; // rax
  short v20; // r11
  bool v21; // dl
  long long v23; // rdx
  unsigned char *v24; // r9
  unsigned int v25; // [rsp+Ch] [rbp-5Ch]
  unsigned int v26; // [rsp+18h] [rbp-50h]
  unsigned char v27; // [rsp+1Eh] [rbp-4Ah]
  unsigned char v28; // [rsp+1Eh] [rbp-4Ah]
  unsigned char v29; // [rsp+1Eh] [rbp-4Ah]
  unsigned char v30; // [rsp+1Eh] [rbp-4Ah]
  char v31; // [rsp+1Fh] [rbp-49h]
  unsigned long long v32; // [rsp+28h] [rbp-40h]

  v2 = a1;
  v3 = a2;
  v4 = 0;
  v32 = (unsigned long long)(a2 + 1);
  v5 = qword_26498;
  v25 = dword_26558;
  v31 = byte_26552;
  v26 = dword_26558 - 2;
  while ( 1 )
  {
    while ( 1 )
    {
      LOBYTE(v6) = *v2;
      v7 = *v3;
      v8 = v2 + 1;
      v9 = v3 + 1;
      if ( *v2 == (char)v7 )
        goto LABEL_17;
      if ( v25 != 4 )
        break;
      v30 = *v2;
      v8 = v2 + 1;
      v19 = *__ctype_b_loc();
      v20 = v19[(unsigned char)v7] & 0x2000;
      LOBYTE(v6) = v30;
      if ( (v19[v30] & 0x2000) != 0 )
      {
        while ( (char)v6 != 10 )
        {
          v6 = *v8;
          if ( (v19[v6] & 0x2000) == 0 )
          {
            LOBYTE(v6) = 32;
            break;
          }
          ++v8;
        }
        if ( !v20 )
          goto LABEL_62;
      }
      else if ( !v20 )
      {
        goto LABEL_50;
      }
      while ( 1 )
      {
        if ( (char)v7 == 10 )
          goto LABEL_14;
        v7 = *v9;
        if ( (v19[v7] & 0x2000) == 0 )
          break;
        ++v9;
      }
      v7 = 32;
LABEL_62:
      if ( (char)v7 == (char)v6 )
        goto LABEL_14;
LABEL_50:
      v21 = (char)v6 != 10 && (char)v7 == 32;
      if ( v21 )
      {
        if ( v8 <= a1 + 1 )
        {
          v7 = 32;
        }
        else
        {
          v7 = 32;
          v2 = v8 - 1;
          v3 = v9;
          if ( (v19[*(v8 - 2)] & 0x2000) != 0 )
            continue;
        }
      }
      else
      {
        v21 = (char)v7 != 10;
      }
      if ( (char)v6 != 32 || !v21 )
        goto LABEL_14;
      if ( (unsigned long long)v9 <= v32 )
      {
        LOBYTE(v6) = 32;
        goto LABEL_14;
      }
      v3 = v9 - 1;
      v2 = v8;
      if ( (v19[*(v9 - 2)] & 0x2000) == 0 )
        goto LABEL_14;
    }
    if ( v25 > 4 )
    {
      if ( v25 == 5 )
      {
        v29 = *v2;
        v15 = __ctype_b_loc();
        v8 = v2 + 1;
        LOBYTE(v6) = v29;
        v16 = *v15;
        while ( (v16[(unsigned char)v6] & 0x2000) != 0 && (char)v6 != 10 )
          LOBYTE(v6) = *v8++;
        if ( (v16[(unsigned char)v7] & 0x2000) != 0 )
        {
          do
          {
            if ( (char)v7 == 10 )
              break;
            v17 = *v9++;
            v7 = v17;
          }
          while ( (v16[v17] & 0x2000) != 0 );
        }
      }
      goto LABEL_14;
    }
    if ( v25 == 1 )
      goto LABEL_32;
    if ( v26 > 1 )
      goto LABEL_14;
    v27 = *v2;
    v8 = v2 + 1;
    v10 = *__ctype_b_loc();
    LOBYTE(v6) = v27;
    if ( (v10[v27] & 0x2000) != 0 && (v10[(unsigned char)v7] & 0x2000) != 0 )
      break;
    if ( v25 == 2 )
      goto LABEL_14;
LABEL_32:
    if ( (char)v6 != 32 || (char)v7 != 9 )
    {
      if ( (char)v6 != 9 || (char)v7 != 32 )
        goto LABEL_14;
      LOBYTE(v6) = 9;
    }
    v18 = v4;
    while ( (char)v6 == 32 )
    {
      ++v18;
LABEL_40:
      LOBYTE(v6) = *v8++;
    }
    if ( (char)v6 == 9 )
    {
      v18 = v5 + v18 - v18 % v5;
      goto LABEL_40;
    }
    while ( (char)v7 == 32 )
    {
      ++v4;
LABEL_45:
      v7 = *v9++;
    }
    if ( (char)v7 == 9 )
    {
      v4 = v5 + v4 - v4 % v5;
      goto LABEL_45;
    }
    if ( v18 != v4 )
      return 1;
LABEL_14:
    if ( v31 )
    {
      v28 = v6;
      v13 = *__ctype_tolower_loc();
      LOBYTE(v6) = v13[v28];
      LOBYTE(v7) = v13[v7];
    }
    if ( (char)v6 != (char)v7 )
      return 1;
LABEL_17:
    if ( (char)v7 == 10 )
      return 0;
    v14 = 1;
    if ( (char)v7 == 9 )
      v14 = v5 - v4 % v5;
    v4 += v14;
    v3 = v9;
    v2 = v8;
  }
  if ( v27 != 10 )
  {
    v11 = v2[1];
    v12 = v2 + 1;
    if ( (char)v11 != 10 )
    {
      while ( (v10[v11] & 0x2000) != 0 )
      {
        v11 = (unsigned char)*++v12;
        if ( (char)v11 == 10 )
          goto LABEL_76;
      }
      goto LABEL_14;
    }
  }
LABEL_76:
  if ( (char)v7 != 10 )
  {
    v23 = v3[1];
    if ( (char)v23 != 10 )
    {
      v24 = v3 + 1;
      while ( (v10[v23] & 0x2000) != 0 )
      {
        v23 = *++v24;
        if ( (char)v23 == 10 )
          return 0;
      }
      goto LABEL_14;
    }
  }
  return 0;
}



// Function: print_script @ 0xf080
void print_script(long long a1, long long (*a2)(long long), void (*a3)(long long))
{
  long long v4; // rbp
  long long *v5; // rax
  long long v6; // rdi
  long long *v7; // rbx

  if ( a1 )
  {
    v4 = a1;
    do
    {
      v5 = (long long *)a2(v4);
      v6 = v4;
      v4 = *v5;
      v7 = v5;
      *v5 = 0;
      a3(v6);
      *v7 = v4;
    }
    while ( v4 );
  }
}



// Function: output_1_line @ 0xf0e0
void output_1_line(unsigned long long ptr, char *a2, const char *a3, long long a4)
{
  char *v4; // r15
  unsigned long long v6; // rbp
  size_t v7; // rbx
  size_t v8; // rax
  _IO_FILE *v9; // rbx
  unsigned long long v11; // r13
  long long v12; // r14
  int v13; // esi
  char *v14; // rax
  unsigned long long v15; // rcx
  char *v16; // rax
  char *v17; // rax
  char *IO_write_ptr; // rax
  unsigned long long v19; // [rsp+0h] [rbp-58h]
  unsigned long long v20; // [rsp+10h] [rbp-48h]

  v4 = (char *)ptr;
  if ( !byte_264A0 )
  {
    v6 = (unsigned long long)&a2[-ptr];
    if ( a2 != (char *)ptr )
    {
      do
      {
        v7 = 1024;
        if ( v6 <= 0x400 )
          v7 = v6;
        v8 = fwrite_unlocked(v4, 1u, v7, stream);
        if ( v7 > v8 )
          break;
        v6 -= v8;
        v4 += v8;
        sub_E1C0();
      }
      while ( v6 );
    }
    return;
  }
  v9 = stream;
  v11 = 0;
  v12 = 1;
  v20 = qword_26498;
  if ( ptr >= (unsigned long long)a2 )
    return;
LABEL_10:
  v13 = (unsigned char)*v4++;
  if ( (char)v13 != 9 )
  {
LABEL_11:
    if ( (char)v13 == 13 )
    {
      IO_write_ptr = v9->_IO_write_ptr;
      if ( IO_write_ptr >= v9->_IO_write_end )
      {
        __overflow(v9, v13);
      }
      else
      {
        v9->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = 13;
      }
      if ( a3 && a2 > v4 )
      {
        v11 = 0;
        if ( *v4 != 10 )
          __fprintf_chk(v9, 1, a3, a4);
        goto LABEL_16;
      }
    }
    else
    {
      if ( (char)v13 != 8 )
      {
        v11 += ((*__ctype_b_loc())[(unsigned char)v13] & 0x4000) != 0;
        v14 = v9->_IO_write_ptr;
        if ( v14 < v9->_IO_write_end )
        {
          v9->_IO_write_ptr = v14 + 1;
          *v14 = v13;
          goto LABEL_15;
        }
LABEL_33:
        __overflow(v9, v13);
        goto LABEL_15;
      }
      if ( v11 )
      {
        v17 = v9->_IO_write_ptr;
        --v11;
        if ( v17 < v9->_IO_write_end )
        {
          v9->_IO_write_ptr = v17 + 1;
          *v17 = 8;
          goto LABEL_15;
        }
        goto LABEL_33;
      }
    }
    v11 = 0;
    goto LABEL_15;
  }
  while ( 1 )
  {
    v15 = v20 - v11 % v20;
    v11 += v15;
    do
    {
      v16 = v9->_IO_write_ptr;
      if ( v16 < v9->_IO_write_end )
      {
        v9->_IO_write_ptr = v16 + 1;
        *v16 = 32;
      }
      else
      {
        v19 = v15;
        __overflow(v9, 32);
        v15 = v19;
      }
      --v15;
    }
    while ( v15 );
LABEL_15:
    if ( a2 == v4 )
      break;
LABEL_16:
    if ( ++v12 != 1024 )
      goto LABEL_10;
    sub_E1C0();
    v13 = (unsigned char)*v4++;
    v12 = 0;
    if ( (char)v13 != 9 )
      goto LABEL_11;
  }
}



// Function: print_1_line_nl @ 0xf330
void print_1_line_nl(char *a1, unsigned long long *a2, char a3)
{
  char *v4; // r14
  char *v5; // r12
  _IO_FILE *v6; // r15
  const char *v7; // r13
  char *v8; // rcx
  const char *v9; // rdx
  char *v10; // rax

  v4 = (char *)*a2;
  v5 = (char *)a2[1];
  v6 = stream;
  if ( a1 )
  {
    v7 = 0;
    if ( *a1 )
    {
      v7 = "%s\t";
      v8 = a1;
      if ( !byte_26492 )
        v7 = "%s ";
      v9 = v7;
      if ( byte_26491 && *v4 == 10 )
      {
        v9 = "%s";
        v8 = &a1[*a1 == 32];
      }
      __fprintf_chk(stream, 1, v9, v8);
    }
    if ( a3 )
    {
      sub_F0E0((unsigned long long)v4, &v5[-(*(v5 - 1) == 10)], v7, (long long)a1);
      if ( !*a1 )
        return;
    }
    else
    {
      sub_F0E0((unsigned long long)v4, v5, v7, (long long)a1);
      if ( !*a1 )
        return;
    }
  }
  else if ( a3 )
  {
    sub_F0E0(*a2, &v5[-(*(v5 - 1) == 10)], 0, 0);
  }
  else
  {
    sub_F0E0((unsigned long long)v4, (char *)a2[1], 0, 0);
  }
  if ( *(v5 - 1) != 10 )
  {
    if ( byte_26758 )
    {
      if ( dword_26010 != 3 )
      {
        sub_E180((long long)&unk_26020);
        sub_E180((long long)&unk_26050);
        sub_E180((long long)&unk_26030);
        dword_26010 = 3;
      }
    }
    v10 = dcgettext(0, "No newline at end of file", 5);
    __fprintf_chk(v6, 1, "\n\\ %s\n", v10);
  }
}



// Function: print_1_line @ 0xf500
void print_1_line(char *a1, unsigned long long *a2)
{
  sub_F330(a1, a2, 0);
}



// Function: print_number_range @ 0xf610
long long print_number_range(char a1, long long a2, long long a3, long long a4)
{
  long long v4; // rax
  long long v6; // rcx
  long long v7; // r9

  v4 = *(long long *)(a2 + 232);
  v6 = v4 + a3 + 1;
  v7 = v4 + a4 + 1;
  if ( v6 >= v7 )
    return __fprintf_chk(stream, 1, "%ld", v4 + a4 + 1);
  else
    return __fprintf_chk(stream, 1, "%ld%c%ld", v6, (unsigned int)a1, v7);
}



// Function: analyze_hunk @ 0xf670
long long analyze_hunk(long long *a1, long long *a2, long long *a3, long long *a4, long long *a5)
{
  char v5; // al
  int v6; // ebp
  long long v7; // r12
  long long v8; // r14
  long long *v9; // r12
  long long v10; // rbx
  long long v11; // rcx
  long long v12; // rdx
  long long v13; // r15
  long long *v14; // rax
  long long v15; // r12
  long long *v16; // r14
  const char *v17; // rsi
  long long v18; // r13
  int v19; // r8d
  long long v20; // rdi
  long long *v21; // rax
  unsigned int v22; // r8d
  const unsigned short **v24; // rax
  const unsigned short *v25; // rdi
  long long v26; // rdx
  const char *v27; // rax
  const char *v28; // rsi
  long long v29; // r13
  int v30; // r8d
  long long v31; // rdi
  const unsigned short **v32; // rax
  const unsigned short *v33; // rdi
  long long v34; // rdx
  const char *v35; // rax
  long long v36; // rt0
  long long v37; // [rsp+0h] [rbp-98h]
  long long v38; // [rsp+8h] [rbp-90h]
  char v39; // [rsp+20h] [rbp-78h]
  unsigned char v40; // [rsp+2Eh] [rbp-6Ah]
  unsigned char v41; // [rsp+2Eh] [rbp-6Ah]
  char v42; // [rsp+2Fh] [rbp-69h]
  int v43; // [rsp+30h] [rbp-68h]
  int v44; // [rsp+34h] [rbp-64h]
  long long v45; // [rsp+38h] [rbp-60h]
  long long v46; // [rsp+40h] [rbp-58h]
  long long v47; // [rsp+48h] [rbp-50h]

  v5 = byte_26554;
  if ( byte_26554 )
  {
    v44 = 0;
    if ( (unsigned int)dword_26558 <= 1 )
    {
      v6 = 0;
      v43 = 0;
    }
    else
    {
      v6 = 1;
      v43 = (unsigned int)dword_26558 > 3;
    }
  }
  else
  {
    v44 = -1;
    v43 = 0;
    v5 = stru_264C0.fastmap != 0;
    v6 = 0;
  }
  v7 = qword_26388;
  v46 = 0;
  v45 = 0;
  v8 = qword_26258;
  *a2 = a1[3];
  v47 = v7;
  v9 = a1;
  *a4 = a1[4];
  do
  {
    v10 = v9[3];
    v11 = v9[2];
    v12 = v9[1];
    v13 = v9[4];
    v46 += v12;
    v45 += v11;
    v38 = v11 + v10 - 1;
    v37 = v13 + v12 - 1;
    v39 = v5 & (v10 <= v38);
    if ( v39 )
    {
      v14 = v9;
      v15 = v8;
      v16 = v14;
      do
      {
        v17 = *(const char **)(v15 + 8 * v10);
        v18 = *(long long *)(v15 + 8 * v10 + 8) + (*(char *)(*(long long *)(v15 + 8 * v10 + 8) - 1LL) != 10) - 1LL;
        v19 = v18 - (int)v17;
        v20 = v18 - (long long)v17;
        if ( v6 )
        {
          v40 = *v17;
          if ( *v17 != 10 )
          {
            v24 = __ctype_b_loc();
            v19 = v18 - (int)v17;
            v25 = *v24;
            v26 = v40;
            v27 = v17;
            while ( (v25[v26] & 0x2000) != 0 )
            {
              v26 = *(unsigned char *)++v27;
              if ( (char)v26 == 10 )
              {
                v20 = v18 - (long long)v27;
                goto LABEL_11;
              }
            }
            v20 = v18 - (long long)v27;
            if ( !v43 )
              v20 = v18 - (long long)v17;
          }
        }
LABEL_11:
        if ( v44 == v20 )
        {
          v5 = v39;
        }
        else
        {
          if ( !stru_264C0.fastmap )
          {
            v21 = v16;
            v8 = v15;
            v9 = v21;
            v5 = 0;
            goto LABEL_14;
          }
          v5 = re_search(&stru_264C0, v17, v19, 0, v19, 0) >= 0;
        }
        ++v10;
      }
      while ( v38 >= v10 && v5 );
      v36 = v15;
      v9 = v16;
      v8 = v36;
      v13 = v9[4];
      v42 = v5 & (v37 >= v13);
    }
    else
    {
      v42 = v5 & (v13 <= v37);
    }
    if ( !v42 )
      goto LABEL_14;
    while ( 1 )
    {
      v28 = *(const char **)(v47 + 8 * v13);
      v29 = *(long long *)(v47 + 8 * v13 + 8) + (*(char *)(*(long long *)(v47 + 8 * v13 + 8) - 1LL) != 10) - 1LL;
      v30 = v29 - (int)v28;
      v31 = v29 - (long long)v28;
      if ( v6 )
      {
        v41 = *v28;
        if ( *v28 != 10 )
        {
          v32 = __ctype_b_loc();
          v30 = v29 - (int)v28;
          v33 = *v32;
          v34 = v41;
          v35 = v28;
          while ( (v33[v34] & 0x2000) != 0 )
          {
            v34 = *(unsigned char *)++v35;
            if ( (char)v34 == 10 )
            {
              v31 = v29 - (long long)v35;
              goto LABEL_34;
            }
          }
          v31 = v29 - (long long)v35;
          if ( !v43 )
            v31 = v29 - (long long)v28;
        }
      }
LABEL_34:
      if ( v44 != v31 )
        break;
      v5 = v42;
LABEL_31:
      if ( v37 < ++v13 || !v5 )
        goto LABEL_14;
    }
    if ( stru_264C0.fastmap )
    {
      v5 = re_search(&stru_264C0, v28, v30, 0, v30, 0) >= 0;
      goto LABEL_31;
    }
    v5 = 0;
LABEL_14:
    v9 = (long long *)*v9;
  }
  while ( v9 );
  v22 = 0;
  *a3 = v38;
  *a5 = v37;
  if ( !v5 )
    return (v45 != 0) | (2 * (unsigned int)(v46 != 0));
  return v22;
}



// Function: concat @ 0xfa30
long long concat(const char *a1, const char *a2, const char *a3)
{
  size_t v4; // rbx
  size_t v5; // rbx
  size_t v6; // rax
  long long v7; // r12

  v4 = strlen(a1);
  v5 = strlen(a2) + v4;
  v6 = strlen(a3);
  v7 = sub_16A40(v5 + v6 + 1);
  __sprintf_chk(v7, 1, -1, "%s%s%s", a1, a2, a3);
  return v7;
}



// Function: zalloc @ 0xfab0
void *zalloc(size_t n)
{
  void *v1; // rax

  v1 = (void *)sub_16A40(n);
  return memset(v1, 0, n);
}



// Function: debug_script @ 0xfad0
int debug_script(long long *a1)
{
  long long *v1; // rbx

  v1 = a1;
  fflush_unlocked(stdout);
  if ( a1 )
  {
    do
    {
      __fprintf_chk(stderr, 1, "%3ld %3ld delete %ld insert %ld\n", v1[3], v1[4], v1[2], v1[1]);
      v1 = (long long *)*v1;
    }
    while ( v1 );
  }
  return fflush_unlocked(stderr);
}


