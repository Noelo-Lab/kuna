// Function: main @ 0x4fa0
long long main(int a1, char **a2, char **a3)
{
  int v3; // r14d
  const struct option *v4; // r13
  char *const *v6; // rbx
  char *v7; // rdi
  long long v8; // rax
  long long v9; // rdx
  long long v10; // rcx
  int v11; // r10d
  unsigned int v12; // eax
  int v13; // eax
  intmax_t v14; // rax
  long long v15; // rdx
  long long v16; // r14
  int v17; // r10d
  long long *v18; // rbx
  long long *v19; // rdi
  intmax_t v20; // rax
  long long v21; // rdx
  unsigned int v23; // eax
  size_t v24; // rax
  long long v25; // r14
  const char *v26; // rbx
  char *v27; // rbp
  char *v28; // rdi
  intmax_t v29; // rax
  intmax_t v30; // rax
  long long v31; // r13
  long long v32; // r12
  long long v33; // rbp
  int v34; // ebx
  int v35; // eax
  long long v36; // rdi
  long long v37; // rsi
  unsigned long long v38; // rdi
  unsigned long long v39; // rcx
  unsigned long long v40; // rdx
  unsigned long long v41; // rdi
  unsigned long long v42; // rax
  int v43; // eax
  char v44; // al
  bool v45; // dl
  long long v46; // rax
  long long v47; // r14
  const char *v48; // rax
  const char *v49; // rdi
  bool v50; // al
  int v51; // eax
  int i; // r12d
  int v53; // eax
  long long v54; // rax
  char *v55; // rdi
  int v56; // r8d
  const char *v57; // rdx
  const char *v58; // rdx
  int v59; // eax
  unsigned long long v60; // [rsp+0h] [rbp-88h]
  long long v61; // [rsp+8h] [rbp-80h]
  char v62; // [rsp+14h] [rbp-74h]
  int v63; // [rsp+14h] [rbp-74h]
  char v64; // [rsp+18h] [rbp-70h]
  int v65; // [rsp+18h] [rbp-70h]
  int v66; // [rsp+18h] [rbp-70h]
  int v67; // [rsp+1Ch] [rbp-6Ch]
  int v68; // [rsp+1Ch] [rbp-6Ch]
  int v69; // [rsp+1Ch] [rbp-6Ch]
  int v70; // [rsp+1Ch] [rbp-6Ch]
  int v71; // [rsp+1Ch] [rbp-6Ch]
  int v72; // [rsp+1Ch] [rbp-6Ch]
  int v73; // [rsp+1Ch] [rbp-6Ch]
  int v74; // [rsp+1Ch] [rbp-6Ch]
  int v75; // [rsp+1Ch] [rbp-6Ch]
  int v76; // [rsp+1Ch] [rbp-6Ch]
  int v77; // [rsp+1Ch] [rbp-6Ch]
  int v78; // [rsp+1Ch] [rbp-6Ch]
  int v79; // [rsp+1Ch] [rbp-6Ch]
  int v80; // [rsp+1Ch] [rbp-6Ch]
  int v81; // [rsp+1Ch] [rbp-6Ch]
  int v82; // [rsp+1Ch] [rbp-6Ch]
  int v83; // [rsp+1Ch] [rbp-6Ch]
  int v84; // [rsp+1Ch] [rbp-6Ch]
  int v85; // [rsp+1Ch] [rbp-6Ch]
  int v86; // [rsp+1Ch] [rbp-6Ch]
  int v87; // [rsp+1Ch] [rbp-6Ch]
  int v88; // [rsp+1Ch] [rbp-6Ch]
  int v89; // [rsp+1Ch] [rbp-6Ch]
  int v90; // [rsp+1Ch] [rbp-6Ch]
  char *const *v91; // [rsp+20h] [rbp-68h]
  int v92; // [rsp+20h] [rbp-68h]
  char *const *v93; // [rsp+28h] [rbp-60h]
  const struct option *v94; // [rsp+30h] [rbp-58h] BYREF
  long long v95; // [rsp+38h] [rbp-50h] BYREF
  char *endptr[9]; // [rsp+40h] [rbp-48h] BYREF

  v3 = -1;
  v4 = &longopts;
  v6 = a2;
  v7 = *a2;
  endptr[1] = (char *)__readfsqword(0x28u);
  v94 = 0;
  v95 = 0;
  status = 2;
  sub_13D30(v7);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_FA70(0);
  qword_26600 = (long long)&buffer;
  qword_265C0 = (long long)&stru_264C0;
  re_set_syntax(0x50A46u);
  v8 = sub_10050();
  byte_26180 = 0;
  qword_26410 = v8;
  sub_16390();
  v62 = 0;
  v60 = 0;
  v64 = 0;
  v61 = -1;
LABEL_2:
  while ( 1 )
  {
    v11 = getopt_long(a1, v6, "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ", v4, 0);
    if ( v11 == -1 )
      break;
    switch ( v11 )
    {
      case 0:
      case 104:
      case 128:
      case 133:
        goto LABEL_5;
      case 48:
      case 49:
      case 50:
      case 51:
      case 52:
      case 53:
      case 54:
      case 55:
      case 56:
      case 57:
        if ( (unsigned int)(v3 - 48) > 9 )
        {
          v3 = v11;
          v61 = v11 - 48;
        }
        else
        {
          v3 = v11;
          if ( v61 - (v11 <= 51) > 0x666666666666665LL )
            v61 = 0x3FFFFFFFFFFFFFFFLL;
          else
            v61 = 10 * v61 + v11 - 48;
        }
        continue;
      case 66:
        byte_26554 = 1;
        v3 = v11;
        continue;
      case 67:
      case 85:
        if ( optarg )
        {
          v65 = v11;
          v14 = strtoimax(optarg, endptr, 10);
          if ( *endptr[0] )
            goto LABEL_198;
          v11 = v65;
          if ( v14 < 0 )
            goto LABEL_198;
          v15 = 0x3FFFFFFFFFFFFFFFLL;
          if ( v14 <= 0x3FFFFFFFFFFFFFFFLL )
            v15 = v14;
          v16 = v15;
        }
        else
        {
          v16 = 3;
        }
        v66 = v11;
        sub_8D50((unsigned int)(v11 == 85) + 2);
        v17 = v66;
        if ( qword_26570 >= v16 )
        {
          v64 = 1;
          v3 = v17;
        }
        else
        {
          qword_26570 = v16;
          v3 = v66;
          v64 = 1;
        }
        break;
      case 68:
        v79 = v11;
        sub_8D50(7);
        v24 = strlen(optarg);
        v25 = sub_15F50(7 * v24 + 97);
        __sprintf_chk(
          v25,
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
        v93 = v6;
        v26 = (const char *)v25;
        v3 = v79;
        v92 = a1;
        v27 = (char *)&s1;
        do
        {
          v28 = v27;
          v27 += 8;
          sub_8C60(v28, v26, "-D");
          v26 += strlen(v26) + 1;
        }
        while ( v27 != &byte_26480 );
        a1 = v92;
        v6 = v93;
        continue;
      case 69:
        v3 = v11;
        if ( (unsigned int)dword_26558 <= 3 )
          dword_26558 |= 1u;
        continue;
      case 70:
        v71 = v11;
        sub_8930(&unk_265E0, optarg);
        v3 = v71;
        continue;
      case 72:
        byte_26418 = 1;
        v3 = v11;
        continue;
      case 73:
        v70 = v11;
        sub_8930(&qword_265A0, optarg);
        v3 = v70;
        continue;
      case 76:
        if ( !qword_26540 )
          goto LABEL_157;
        if ( qword_26548 )
          goto LABEL_199;
        v3 = v11;
        qword_26548 = (long long)optarg;
        continue;
      case 78:
        byte_26586 = 1;
        v3 = v11;
        continue;
      case 80:
        byte_26585 = 1;
        v3 = v11;
        continue;
      case 83:
        v85 = v11;
        sub_8C60(&qword_26488, optarg, "-S");
        v3 = v85;
        continue;
      case 84:
        byte_26492 = 1;
        v3 = v11;
        continue;
      case 87:
        v84 = v11;
        v30 = strtoimax(optarg, endptr, 10);
        if ( v30 <= 0 )
          goto LABEL_188;
        v11 = v84;
        if ( *endptr[0] )
          goto LABEL_188;
        if ( v30 == v60 )
          goto LABEL_5;
        if ( v60 )
        {
          sub_E230("conflicting width options");
LABEL_187:
          sub_E1F0(optarg);
LABEL_188:
          sub_8C00("invalid width '%s'");
        }
        v60 = v30;
        v3 = v84;
        continue;
      case 88:
        v67 = v11;
        v12 = sub_8910((unsigned int)a1, v6, v9, v10, 10);
        v13 = sub_10770(sub_102E0, qword_26410, optarg, v12);
        v11 = v67;
        if ( !v13 )
          goto LABEL_5;
        goto LABEL_187;
      case 90:
        v3 = v11;
        if ( (unsigned int)dword_26558 <= 3 )
          dword_26558 |= 2u;
        continue;
      case 97:
        byte_26568 = 1;
        v3 = v11;
        continue;
      case 98:
        v3 = v11;
        if ( (unsigned int)dword_26558 <= 3 )
          dword_26558 = 4;
        continue;
      case 99:
        v86 = v11;
        v36 = 2;
        goto LABEL_76;
      case 100:
        byte_26408 = 1;
        v3 = v11;
        continue;
      case 101:
        v90 = v11;
        sub_8D50(4);
        v3 = v90;
        continue;
      case 102:
        v89 = v11;
        sub_8D50(5);
        v3 = v89;
        continue;
      case 105:
        byte_26552 = 1;
        v3 = v11;
        continue;
      case 108:
        if ( !path[0] )
        {
          sub_8C00("pagination not supported on this host");
LABEL_197:
          sub_8C00("invalid horizon length '%s'");
LABEL_198:
          sub_8C00("invalid context length '%s'");
LABEL_199:
          sub_E230("too many file label options");
LABEL_200:
          sub_8C00("invalid tabsize '%s'");
LABEL_201:
          sub_E230("conflicting tabsize options");
          start();
        }
        v88 = v11;
        byte_26480 = 1;
        signal(17, 0);
        v3 = v88;
        break;
      case 110:
        v87 = v11;
        sub_8D50(6);
        v3 = v87;
        continue;
      case 112:
        v63 = v11;
        sub_8930(&unk_265E0, "^[[:alpha:]$_]");
        v3 = v63;
        v62 = 1;
        continue;
      case 113:
        byte_264A1 = 1;
        v3 = v11;
        continue;
      case 114:
        byte_26608 = 1;
        v3 = v11;
        continue;
      case 115:
        byte_26584 = 1;
        v3 = v11;
        continue;
      case 116:
        byte_264A0 = 1;
        v3 = v11;
        continue;
      case 117:
        v86 = v11;
        v36 = 3;
LABEL_76:
        sub_8D50(v36);
        v3 = v86;
        if ( qword_26570 <= 2 )
          qword_26570 = 3;
        continue;
      case 118:
        v31 = sub_14040("Len Tower");
        v32 = sub_14040("Richard Stallman");
        v33 = sub_14040("David Hayes");
        v34 = sub_14040("Mike Haertel");
        v35 = sub_14040("Paul Eggert");
        sub_15DE0(
          (int)stdout,
          (unsigned int)"diff",
          (unsigned int)"GNU diffutils",
          (int)off_260A8,
          v35,
          v34,
          v33,
          v32,
          v31,
          0);
        sub_8E20();
        return 0;
      case 119:
        dword_26558 = 5;
        v3 = v11;
        continue;
      case 120:
        v78 = v11;
        v23 = ((long long (*)(void))sub_8910)();
        sub_102E0(qword_26410, optarg, v23);
        v3 = v78;
        continue;
      case 121:
        v77 = v11;
        sub_8D50(8);
        v3 = v77;
        continue;
      case 129:
        v76 = v11;
        sub_8C60(&v94, optarg, "--from-file");
        v3 = v76;
        continue;
      case 130:
        sub_8A10();
        sub_8E20();
        return 0;
      case 131:
        v75 = v11;
        v20 = strtoimax(optarg, endptr, 10);
        if ( *endptr[0] || v20 < 0 )
          goto LABEL_197;
        v21 = qword_26560;
        v3 = v75;
        if ( qword_26560 < v20 )
          v21 = v20;
        qword_26560 = v21;
        continue;
      case 132:
        byte_26551 = 1;
        v3 = v11;
        continue;
      case 134:
        byte_26439 = 1;
        v3 = v11;
        continue;
      case 135:
        v74 = v11;
        sub_8D50(7);
        v91 = v6;
        v18 = qword_26440;
        v3 = v74;
        do
        {
          v19 = v18++;
          sub_8C60(v19, optarg, "--line-format");
        }
        while ( &unk_26458 != (_UNKNOWN *)v18 );
        v6 = v91;
        continue;
      case 136:
        byte_26550 = 1;
        v3 = v11;
        continue;
      case 137:
        byte_26551 = 0;
        v3 = v11;
        continue;
      case 138:
        v73 = v11;
        sub_8D50(1);
        v3 = v73;
        continue;
      case 139:
        v72 = v11;
        sub_8D50(8);
        byte_2643A = 1;
        v3 = v72;
        continue;
      case 140:
        byte_26490 = 1;
        v3 = v11;
        continue;
      case 141:
        byte_26491 = 1;
        v3 = v11;
        continue;
      case 142:
        byte_26438 = 1;
        v3 = v11;
        continue;
      case 143:
        v81 = v11;
        v29 = strtoimax(optarg, endptr, 10);
        if ( v29 <= 0 )
          goto LABEL_200;
        v11 = v81;
        if ( *endptr[0] )
          goto LABEL_200;
        if ( v29 == qword_26498 )
        {
LABEL_5:
          v3 = v11;
        }
        else
        {
          if ( qword_26498 )
            goto LABEL_201;
          qword_26498 = v29;
          v3 = v81;
        }
        continue;
      case 144:
        v80 = v11;
        sub_8C60(&v95, optarg, "--to-file");
        v3 = v80;
        continue;
      case 145:
      case 146:
      case 147:
        v69 = v11;
        sub_8D50(7);
        v3 = v69 - 145;
        sub_8C60(&qword_26440[v3], optarg, &aUnchangedLineF[24 * v3]);
        continue;
      case 148:
      case 149:
      case 150:
      case 151:
        v68 = v11;
        sub_8D50(7);
        v3 = v68 - 148;
        sub_8C60(&(&s1)[v3], optarg, &aUnchangedGroup[25 * v3]);
        continue;
      case 152:
        v83 = v11;
        sub_8CC0(optarg);
        v3 = v83;
        continue;
      case 153:
        v82 = v11;
        sub_E270(optarg);
        v3 = v82;
        continue;
      case 154:
        byte_26180 = 1;
        v3 = v11;
        continue;
      default:
        sub_8C00(0);
        goto LABEL_168;
    }
  }
  if ( dword_2657C == 1 )
  {
    v55 = getenv("TERM");
    if ( v55 )
    {
      if ( !strcmp(v55, "dumb") )
        dword_2657C = 0;
    }
  }
  if ( !dword_26580 )
  {
    if ( v62 )
    {
      sub_8D50(2);
      if ( v61 >= 0 )
      {
        if ( dword_26580 != 2 )
          goto LABEL_99;
LABEL_168:
        if ( (unsigned char)sub_10AB0(2) )
LABEL_99:
          qword_26400 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
        else
          qword_26400 = (long long)"%a %b %e %T %Y";
LABEL_100:
        v37 = qword_26570;
        if ( (unsigned int)(dword_26580 - 2) <= 1 && (v61 > qword_26570 || v61 != qword_26570 && !v64) )
        {
          qword_26570 = v61;
          v37 = v61;
        }
        goto LABEL_101;
      }
      qword_26570 = 3;
      if ( dword_26580 != 2 )
      {
        v37 = 3;
        qword_26400 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
        goto LABEL_101;
      }
      if ( (unsigned char)sub_10AB0(2) )
      {
        v37 = qword_26570;
        qword_26400 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
        goto LABEL_101;
      }
      qword_26400 = (long long)"%a %b %e %T %Y";
      goto LABEL_126;
    }
    goto LABEL_122;
  }
LABEL_123:
  if ( dword_26580 != 2 || (unsigned char)sub_10AB0(2) )
    v48 = "%Y-%m-%d %H:%M:%S.%N %z";
  else
    v48 = "%a %b %e %T %Y";
  qword_26400 = (long long)v48;
  if ( v61 >= 0 )
    goto LABEL_100;
LABEL_126:
  v37 = qword_26570;
LABEL_101:
  if ( !qword_26498 )
    qword_26498 = 8;
  if ( v60 )
  {
    v38 = 1;
    if ( byte_264A0 )
    {
LABEL_106:
      v39 = ((unsigned char)v60 & (unsigned char)(v38 + 3) & 1) + ((v38 + 3) >> 1) + (v60 >> 1);
      v40 = v39 % v38;
      v41 = v39 - v39 % v38;
      if ( v41 <= 3 || v41 >= v60 )
      {
        v42 = 0;
      }
      else
      {
        v42 = v40 - v39 + v60;
        if ( v42 > v41 - 3 )
          v42 = v41 - 3;
        if ( !v42 )
          v41 = v60;
        v60 = v41;
      }
LABEL_113:
      qword_26430 = v42;
      qword_26428 = v60;
      if ( qword_26560 < v37 )
        qword_26560 = v37;
      sub_8BA0(&unk_265E0);
      sub_8BA0(&qword_265A0);
      v43 = dword_26580;
      if ( dword_26580 != 7 )
      {
LABEL_116:
        v44 = byte_26438 | (v43 != 8);
        goto LABEL_117;
      }
      if ( !qword_26440[0] )
        goto LABEL_171;
      while ( 1 )
      {
        if ( !qword_26448 )
          qword_26448 = (long long)"%l\n";
        if ( !qword_26450 )
          qword_26450 = (long long)"%l\n";
        if ( !qword_26468 )
        {
          v58 = "%<";
          if ( qword_26478 )
            v58 = (const char *)qword_26478;
          qword_26468 = (long long)v58;
        }
        if ( !qword_26470 )
        {
          v57 = "%>";
          if ( qword_26478 )
            v57 = (const char *)qword_26478;
          qword_26470 = (long long)v57;
        }
        if ( s1 )
        {
          if ( qword_26478 )
            goto LABEL_146;
        }
        else
        {
          s1 = "%=";
          v49 = "%=";
          if ( qword_26478 )
          {
LABEL_163:
            v56 = strcmp(v49, "%=");
            v50 = 0;
            if ( !v56 )
              v50 = *(char *)qword_26440[0] == 0;
            goto LABEL_147;
          }
        }
        qword_26478 = sub_F560(qword_26468, qword_26470, "");
        v43 = dword_26580;
        if ( dword_26580 != 7 )
          goto LABEL_116;
LABEL_146:
        v49 = s1;
        v50 = 1;
        if ( *s1 )
          goto LABEL_163;
LABEL_147:
        v44 = v50;
LABEL_117:
        byte_26578 = v44;
        v45 = 1;
        if ( !qword_265A0 )
          v45 = dword_26558 != 0;
        byte_26553 = byte_264A1 & ~(v45 | byte_26490 | byte_26552 | byte_26554);
        byte_26553 &= 1u;
        v46 = sub_8D80(v6 + 1, (unsigned int)(optind - 1));
        v4 = v94;
        v47 = v95;
        qword_26420 = v46;
        if ( v94 )
        {
          if ( !v95 )
          {
            v51 = optind;
            for ( i = 0; v51 < a1; optind = v51 )
            {
              v53 = sub_8E70(0, v4, v6[v51]);
              if ( i < v53 )
                i = v53;
              v51 = optind + 1;
            }
            goto LABEL_152;
          }
          sub_E230("--from-file and --to-file both specified");
LABEL_122:
          sub_8D50(1);
          goto LABEL_123;
        }
        v54 = optind;
        if ( v95 )
        {
          i = 0;
          while ( (int)v54 < a1 )
          {
            v59 = sub_8E70(0, v6[v54], v47);
            if ( i < v59 )
              i = v59;
            v54 = ++optind;
          }
LABEL_152:
          sub_E0F0();
          sub_8E20();
          exit(i);
        }
        if ( a1 - optind == 2 )
        {
          i = sub_8E70(0, v6[optind], v6[optind + 1]);
          goto LABEL_152;
        }
        if ( a1 - optind > 1 )
        {
          sub_8C00("extra operand '%s'");
LABEL_157:
          v3 = v11;
          qword_26540 = (long long)optarg;
          goto LABEL_2;
        }
        sub_8C00("missing operand after '%s'");
LABEL_171:
        qword_26440[0] = (long long)"%l\n";
      }
    }
  }
  else
  {
    if ( byte_264A0 )
    {
      v60 = 67;
      v42 = 63;
      goto LABEL_113;
    }
    v60 = 130;
  }
  v38 = qword_26498;
  goto LABEL_106;
}



// Function: shift_boundaries @ 0x6a70
long long shift_boundaries(long long a1)
{
  long long i; // rbp
  long long v3; // rsi
  long long v4; // rcx
  long long v5; // r8
  long long result; // rax
  long long v7; // r9
  long long v8; // r11
  long long v9; // rdi
  bool v10; // zf
  long long v11; // rdx
  long long v12; // rbx
  long long v13; // r8
  long long v14; // r10
  char *v15; // r10

  for ( i = 0; i != 608; i += 304 )
  {
    v3 = *(long long *)(a1 + i + 280);
    v4 = 0;
    v5 = 0;
    result = -i;
    v7 = *(long long *)(a1 + i + 248);
    v8 = *(long long *)(a1 + i + 200);
    v9 = *(long long *)(a1 - i + 584);
    while ( 1 )
    {
LABEL_3:
      v10 = v5 == v8;
      if ( v5 >= v8 )
        goto LABEL_34;
LABEL_4:
      if ( *(char *)(v3 + v5) )
        break;
      do
        ++v4;
      while ( *(char *)(v9 + v4 - 1) );
      ++v5;
    }
    do
    {
      result = v5;
      do
        ++result;
      while ( *(char *)(v3 + result) );
      while ( *(char *)(v9 + v4) )
        ++v4;
      v11 = v5;
      do
      {
        v12 = result - v11;
        while ( v11 )
        {
          v13 = v11;
          if ( *(long long *)(v7 + 8 * v11 - 8) != *(long long *)(v7 + 8 * result - 8) )
            break;
          --v11;
          --result;
          *(char *)(v3 + v11) = 1;
          *(char *)(v3 + result) = 0;
          if ( *(char *)(v3 + v13 - 2) )
          {
            do
              --v11;
            while ( *(char *)(v3 + v11 - 1) );
          }
          do
            --v4;
          while ( *(char *)(v9 + v4) );
        }
        v5 = result;
        if ( !*(char *)(v9 + v4 - 1) )
          v5 = v8;
        if ( v8 != result )
        {
          while ( 1 )
          {
            v14 = result;
            if ( *(long long *)(v7 + 8 * v11) != *(long long *)(v7 + 8 * result) )
              goto LABEL_27;
            ++v11;
            ++result;
            *(char *)(v3 + v11 - 1) = 0;
            *(char *)(v3 + v14) = 1;
            if ( *(char *)(v3 + result) )
              break;
            while ( 1 )
            {
              ++v4;
              if ( !*(char *)(v9 + v4) )
                break;
LABEL_24:
              v5 = result;
            }
LABEL_26:
            if ( v8 == result )
              goto LABEL_27;
          }
          do
            ++result;
          while ( *(char *)(v3 + result) );
          ++v4;
          if ( !*(char *)(v9 + v4) )
            goto LABEL_26;
          goto LABEL_24;
        }
LABEL_27:
        ;
      }
      while ( result - v11 != v12 );
      if ( result <= v5 )
      {
        v5 = result;
        goto LABEL_3;
      }
      v15 = (char *)(v3 + result - 1);
      do
      {
        v15[v11 - result] = 1;
        *v15 = 0;
        do
          --v4;
        while ( *(char *)(v9 + v4) );
        --v15;
      }
      while ( (char *)(v3 - 1 + v5) != v15 );
      v10 = v5 == v8;
      if ( v5 < v8 )
        goto LABEL_4;
LABEL_34:
      ;
    }
    while ( !v10 );
  }
  return result;
}



// Function: add_change @ 0x6c30
long long *add_change(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long *result; // rax

  result = (long long *)sub_15F50(48);
  result[3] = a1;
  result[4] = a2;
  result[1] = a4;
  result[2] = a3;
  *result = a5;
  return result;
}



// Function: build_reverse_script @ 0x6c70
long long build_reverse_script(long long *a1)
{
  long long v1; // r14
  long long v2; // r12
  long long v3; // r13
  long long v4; // r15
  long long v5; // rsi
  long long v6; // rdi
  long long v7; // r8
  unsigned char v8; // dl
  long long v10; // rbx
  long long v11; // r9
  long long v12; // rbp
  long long v13; // rcx
  long long *v14; // rax

  v1 = a1[25];
  v2 = a1[35];
  v3 = a1[73];
  v4 = a1[63];
  if ( v1 <= 0 && v4 <= 0 )
    return 0;
  v5 = 0;
  v6 = 0;
  v7 = 0;
  do
  {
    v8 = *(char *)(v3 + v5);
    if ( v8 | *(char *)(v2 + v6) )
    {
      v10 = v6;
      if ( *(char *)(v2 + v6) )
      {
        do
          ++v10;
        while ( *(char *)(v2 + v10) );
        v11 = v10 - v6;
      }
      else
      {
        v11 = 0;
      }
      v12 = v5;
      if ( v8 )
      {
        do
          ++v12;
        while ( *(char *)(v3 + v12) );
        v13 = v12 - v5;
      }
      else
      {
        v13 = 0;
      }
      v14 = sub_6C30(v6, v5, v11, v13, v7);
      v5 = v12;
      v6 = v10;
      v7 = (long long)v14;
    }
    ++v6;
    ++v5;
  }
  while ( v1 > v6 || v4 > v5 );
  return v7;
}



// Function: build_script @ 0x6d50
long long *build_script(long long *a1)
{
  long long *v1; // r8
  long long v2; // rbx
  long long v3; // rbp
  long long v4; // r12
  long long v5; // r13
  unsigned char v6; // cl
  long long v8; // rdx
  long long v9; // r9
  long long v10; // rdx
  long long v11; // rcx
  long long v12; // rcx

  v1 = 0;
  v2 = a1[25];
  v3 = a1[63];
  v4 = a1[35];
  v5 = a1[73];
  if ( (v3 & v2) >= 0 )
  {
    do
    {
      v6 = *(char *)(v5 + v3 - 1);
      if ( !(v6 | *(char *)(v4 + v2 - 1)) )
        goto LABEL_3;
      if ( *(char *)(v4 + v2 - 1) )
      {
        v8 = v2;
        do
          --v8;
        while ( *(char *)(v4 + v8 - 1) );
        v9 = v2 - v8;
        v2 = v8;
        if ( v6 )
        {
LABEL_10:
          v10 = v3;
          do
            --v10;
          while ( *(char *)(v5 + v10 - 1) );
          v11 = v3;
          v3 = v10;
          v12 = v11 - v10;
          goto LABEL_13;
        }
      }
      else
      {
        v9 = 0;
        if ( v6 )
          goto LABEL_10;
      }
      v12 = 0;
LABEL_13:
      v1 = sub_6C30(v2, v3, v9, v12, (long long)v1);
LABEL_3:
      --v3;
      --v2;
    }
    while ( v2 >= 0 || v3 >= 0 );
  }
  return v1;
}



// Function: discard_confusing_lines @ 0x6e30
void discard_confusing_lines(long long *a1)
{
  long long v2; // rdi
  long long v3; // rax
  long long v4; // rsi
  long long v5; // rdi
  long long v6; // rax
  long long v7; // rax
  long long v8; // rdx
  long long v9; // rax
  long long *v10; // rbp
  long long v11; // r12
  long long v12; // rcx
  long long v13; // rax
  long long v14; // rdx
  long long v15; // rdi
  long long v16; // rcx
  long long i; // rax
  long long v18; // rdx
  long long v19; // rax
  unsigned long long v20; // rcx
  long long *v21; // r9
  long long *v22; // r8
  char *v23; // r15
  long long *v24; // r10
  long long v25; // rsi
  long long v26; // rdi
  unsigned long long v27; // rax
  unsigned long long v28; // rdx
  long long v29; // rax
  long long v30; // r13
  unsigned long long v31; // r13
  char *v32; // rdx
  long long v33; // r13
  char *v34; // rbp
  long long v35; // r8
  long long v36; // rax
  char *v37; // r9
  char v38; // cl
  long long *v39; // rcx
  long long *v40; // rbx
  char *v41; // r14
  char *k; // r10
  long long v43; // r9
  long long v44; // rdx
  long long v45; // rax
  long long v46; // rsi
  long long v47; // rcx
  long long j; // rsi
  char v49; // di
  long long v50; // rdi
  char *v51; // rcx
  long long v52; // rsi
  long long v53; // rcx
  long long v54; // r11
  long long v55; // r12
  long long v56; // rcx
  char *v57; // r10
  bool v58; // cc
  char *v59; // rax
  char *v60; // r12
  long long v61; // r11
  char v62; // cl
  char *v63; // r9
  char *v64; // rax
  long long v65; // rdi
  char *v66; // r9
  void *v67; // [rsp+10h] [rbp-78h]
  char v68; // [rsp+20h] [rbp-68h] BYREF
  unsigned long long v69; // [rsp+28h] [rbp-60h]
  long long v70[11]; // [rsp+30h] [rbp-58h] BYREF

  v2 = a1[25] + a1[63];
  v70[3] = __readfsqword(0x28u);
  v3 = sub_15F50(16 * v2);
  v4 = a1[25];
  v5 = a1[37];
  a1[32] = v3;
  v6 = 8 * v4 + v3;
  a1[33] = v6;
  v7 = 8 * v4 + v6;
  v8 = a1[63];
  a1[70] = v7;
  a1[71] = v7 + 8 * v8;
  v9 = sub_F5E0(16 * v5);
  v58 = a1[25] <= 0;
  v10 = (long long *)v9;
  v70[0] = v9;
  v11 = v9 + 8 * a1[37];
  v70[1] = v11;
  if ( !v58 )
  {
    v12 = a1[31];
    v13 = 0;
    do
    {
      v14 = *(long long *)(v12 + 8 * v13++);
      ++v10[v14];
    }
    while ( a1[25] > v13 );
  }
  v15 = a1[63];
  if ( v15 > 0 )
  {
    v16 = a1[69];
    for ( i = 0; i < v15; ++i )
    {
      v18 = *(long long *)(v16 + 8 * i);
      ++*(long long *)(v11 + 8 * v18);
      v15 = a1[63];
    }
  }
  v19 = sub_F5E0(a1[25] + v15);
  v20 = a1[25];
  v21 = (long long *)&v68;
  v22 = a1 + 31;
  v23 = (char *)v19;
  v24 = v70;
  v25 = a1[31];
  v26 = v19;
  v69 = v19 + v20;
  v27 = v20 >> 8;
  if ( !(v20 >> 8) )
    goto LABEL_87;
LABEL_8:
  v28 = 5;
  do
  {
    v28 *= 2LL;
    v27 >>= 2;
  }
  while ( v27 );
LABEL_10:
  v29 = 0;
  do
  {
    while ( 1 )
    {
      v30 = *(long long *)(v25 + 8 * v29);
      if ( !v30 )
        goto LABEL_12;
      v31 = *(long long *)(v11 + 8 * v30);
      if ( v31 )
        break;
      *(char *)(v26 + v29) = 1;
LABEL_12:
      if ( ++v29 == v20 )
        goto LABEL_17;
    }
    if ( v31 <= v28 )
      goto LABEL_12;
    *(char *)(v26 + v29++) = 2;
  }
  while ( v29 != v20 );
LABEL_17:
  while ( 1 )
  {
    v22 += 38;
    ++v21;
    if ( a1 + 107 == v22 )
      break;
    v20 = *(v22 - 6);
    v11 = *v24--;
    v26 = *v21;
    v25 = *v22;
    v27 = v20 >> 8;
    if ( v20 >> 8 )
      goto LABEL_8;
LABEL_87:
    if ( v20 )
    {
      v28 = 5;
      goto LABEL_10;
    }
  }
  v32 = v23;
  v33 = 0;
  v67 = v10;
  v34 = &v68;
  while ( 2 )
  {
    v35 = a1[v33 + 25];
    v36 = 0;
    if ( v35 > 0 )
    {
LABEL_23:
      while ( 1 )
      {
        v37 = &v32[v36];
        v38 = v32[v36];
        if ( v38 != 2 )
          break;
        ++v36;
        *v37 = 0;
        if ( v35 <= v36 )
          goto LABEL_25;
      }
      if ( !v38 )
        goto LABEL_22;
      v47 = v36;
      for ( j = 0; ; j += v49 == 2 )
      {
        if ( v35 > ++v47 )
        {
          v49 = v32[v47];
          if ( v49 )
            continue;
        }
        break;
      }
      if ( v47 > v36 )
      {
        while ( v32[v47 - 1] == 2 )
        {
          --v47;
          --j;
          v32[v47] = 0;
          if ( v47 == v36 )
          {
            v50 = 0;
            if ( j > 0 )
              goto LABEL_22;
            goto LABEL_51;
          }
        }
        v50 = v47 - v36;
        if ( 4 * j > v47 - v36 )
        {
          if ( v47 > v36 )
          {
            v51 = &v32[v47 - 1];
            do
            {
              if ( *v51 == 2 )
                *v51 = 0;
              --v51;
            }
            while ( &v32[v36 - 1] != v51 );
          }
          goto LABEL_22;
        }
      }
      else
      {
        v50 = v47 - v36;
        if ( v47 - v36 < 4 * j )
          goto LABEL_22;
      }
      v53 = v50 >> 4;
      if ( v50 >> 4 <= 0 )
      {
LABEL_51:
        v52 = v50 + v36 - 1;
        if ( v50 <= 0 )
          goto LABEL_52;
        v54 = 1;
      }
      else
      {
        v54 = 1;
        do
        {
          v54 *= 2;
          v53 >>= 2;
        }
        while ( v53 );
        v52 = v50 + v36 - 1;
      }
      v55 = 0;
      v56 = 0;
      while ( 1 )
      {
        v57 = &v32[v56 + v36];
        if ( *v57 != 2 )
          break;
        v58 = v55 <= v54;
        if ( v55 == v54 )
        {
          v56 -= ++v55;
          goto LABEL_60;
        }
        ++v55;
        if ( v58 )
        {
LABEL_60:
          if ( ++v56 >= v50 )
            goto LABEL_65;
        }
        else
        {
          ++v56;
          *v57 = 0;
          if ( v56 >= v50 )
          {
LABEL_65:
            v59 = &v32[v36];
            v60 = &v37[v50];
            v61 = 0;
            while ( 1 )
            {
              v62 = *v59;
              if ( v59 - v37 > 7 && v62 == 1 )
                break;
              if ( v62 == 2 )
              {
                *v59++ = 0;
                v61 = 0;
                if ( v60 == v59 )
                  break;
              }
              else
              {
                if ( v62 )
                {
                  if ( ++v61 == 3 )
                    break;
                }
                else
                {
                  v61 = 0;
                }
                if ( v60 == ++v59 )
                  break;
              }
            }
            v63 = &v32[-v50];
            v64 = &v32[v52];
            v65 = 0;
            v66 = &v63[v52];
            do
            {
              if ( &v32[v52] - v64 > 7 && *v64 == 1 )
                break;
              if ( *v64 == 2 )
              {
                *v64 = 0;
                v65 = 0;
              }
              else if ( *v64 )
              {
                if ( ++v65 == 3 )
                  break;
              }
              else
              {
                v65 = 0;
              }
              --v64;
            }
            while ( v66 != v64 );
LABEL_52:
            v36 = v52;
LABEL_22:
            if ( v35 <= ++v36 )
              goto LABEL_25;
            goto LABEL_23;
          }
        }
      }
      v55 = 0;
      goto LABEL_60;
    }
LABEL_25:
    v33 += 38;
    v34 += 8;
    if ( v33 != 76 )
    {
      v32 = *(char **)v34;
      continue;
    }
    break;
  }
  v39 = a1 + 25;
  v40 = a1 + 101;
  v41 = &v68;
  for ( k = v23; ; k = *(char **)v41 )
  {
    v43 = *v39;
    v44 = 0;
    if ( *v39 > 0 )
    {
      v45 = 0;
      do
      {
        while ( !byte_26408 && k[v45] )
        {
          *(char *)(v39[10] + v45++) = 1;
          if ( v43 == v45 )
            goto LABEL_33;
        }
        v46 = 8 * v44;
        *(long long *)(v39[7] + 8 * v44++) = *(long long *)(v39[6] + 8 * v45);
        *(long long *)(v39[8] + v46) = v45++;
      }
      while ( v43 != v45 );
    }
LABEL_33:
    v39[9] = v44;
    v39 += 38;
    v41 += 8;
    if ( v40 == v39 )
      break;
  }
  free(v23);
  free(v67);
}



// Function: briefly_report @ 0x73b0
void briefly_report(int a1, long long a2)
{
  long long v2; // r12
  long long v3; // rbp
  char *v4; // rdi

  if ( a1 )
  {
    v2 = qword_26548;
    if ( qword_26548 )
    {
      v3 = qword_26540;
      if ( qword_26540 )
        goto LABEL_4;
    }
    else
    {
      v3 = qword_26540;
      v2 = *(long long *)(a2 + 312);
      if ( qword_26540 )
      {
LABEL_4:
        if ( !byte_264A1 )
        {
LABEL_5:
          v4 = dcgettext(0, "Binary files %s and %s differ\n", 5);
LABEL_6:
          sub_E0E0(v4, v3, v2);
          return;
        }
LABEL_7:
        v4 = dcgettext(0, "Files %s and %s differ\n", 5);
        goto LABEL_6;
      }
    }
    v3 = *(long long *)(a2 + 8);
    if ( !byte_264A1 )
      goto LABEL_5;
    goto LABEL_7;
  }
}



// Function: diff_2_files @ 0x7460
long long diff_2_files(long long a1)
{
  long long v2; // rdx
  int v3; // ecx
  int v4; // esi
  long long v5; // rax
  long long v6; // rbp
  long long v7; // rax
  long long v8; // rdi
  long long v9; // rbx
  unsigned int v10; // r12d
  void *v11; // rdi
  long long v13; // rbx
  long long v14; // rax
  long long v15; // r11
  long long v16; // rdx
  long long v17; // rax
  long long *v18; // rbx
  long long *v19; // rbp
  long long *v20; // rax
  long long *v21; // rdi
  long long *v22; // r12
  int v23; // eax
  long long v24; // rsi
  long long v25; // rdi
  void *v26; // rdi
  char *v27; // rbx
  char *v28; // rbp
  char *v29; // rax
  const char *v30; // rcx
  char *ptr; // [rsp+18h] [rbp-A0h]
  char v32[8]; // [rsp+20h] [rbp-98h] BYREF
  char v33[8]; // [rsp+28h] [rbp-90h] BYREF
  char v34[8]; // [rsp+30h] [rbp-88h] BYREF
  char v35[8]; // [rsp+38h] [rbp-80h] BYREF
  long long v36[2]; // [rsp+40h] [rbp-78h] BYREF
  long long v37; // [rsp+50h] [rbp-68h]
  long long v38; // [rsp+58h] [rbp-60h]
  char v39; // [rsp+60h] [rbp-58h]
  long long v40; // [rsp+68h] [rbp-50h]
  unsigned long long v41; // [rsp+78h] [rbp-40h]

  v41 = __readfsqword(0x28u);
  if ( (unsigned char)sub_C8F0(a1, (unsigned char)byte_26553) )
  {
    v2 = *(long long *)(a1 + 368);
    v3 = *(int *)a1;
    v4 = *(int *)(a1 + 304);
    if ( *(long long *)(a1 + 64) == v2
      || *(long long *)(a1 + 64) <= 0LL
      || v2 <= 0
      || v3 >= 0 && (*(int *)(a1 + 40) & 0xF000) != 0x8000
      || v4 >= 0 && (*(int *)(a1 + 344) & 0xF000) != 0x8000 )
    {
      if ( v3 == v4 )
      {
LABEL_15:
        v10 = 0;
        goto LABEL_16;
      }
      v5 = sub_165D0(*(long long *)(a1 + 72), *(long long *)(a1 + 376), 0x7FFFFFFFFFFFFFFELL);
      v6 = sub_165D0(8, v5, 0x7FFFFFFFFFFFFFFELL);
      v7 = sub_15FA0(*(long long *)(a1 + 160), v6);
      v8 = *(long long *)(a1 + 464);
      *(long long *)(a1 + 160) = v7;
      *(long long *)(a1 + 464) = sub_15FA0(v8, v6);
      while ( 1 )
      {
        if ( *(int *)a1 >= 0 )
          sub_BF50(a1, v6 - *(long long *)(a1 + 176));
        if ( *(int *)(a1 + 304) >= 0 )
          sub_BF50(a1 + 304, v6 - *(long long *)(a1 + 480));
        v9 = *(long long *)(a1 + 176);
        if ( v9 != *(long long *)(a1 + 480)
          || memcmp(*(const void **)(a1 + 160), *(const void **)(a1 + 464), *(long long *)(a1 + 176)) )
        {
          break;
        }
        if ( v9 != v6 )
          goto LABEL_15;
        *(long long *)(a1 + 480) = 0;
        *(long long *)(a1 + 176) = 0;
      }
    }
    v10 = 1;
LABEL_16:
    sub_73B0(v10, a1);
    goto LABEL_17;
  }
  ptr = (char *)sub_F5E0(*(long long *)(a1 + 200) + *(long long *)(a1 + 504) + 4LL);
  *(long long *)(a1 + 280) = ptr + 1;
  *(long long *)(a1 + 584) = &ptr[*(long long *)(a1 + 200) + 3];
  sub_6E30((long long *)a1);
  v13 = *(long long *)(a1 + 272) + *(long long *)(a1 + 576) + 3LL;
  v36[0] = *(long long *)(a1 + 256);
  v36[1] = *(long long *)(a1 + 560);
  v14 = sub_15F50(16 * v13);
  v15 = *(long long *)(a1 + 576);
  v16 = 8 * v15 + 8;
  v37 = v14 + v16;
  v38 = v16 + 8 * v13 + v14;
  v39 = byte_26418;
  if ( v13 )
  {
    v17 = 1;
    do
    {
      v17 *= 2;
      v13 >>= 2;
    }
    while ( v13 );
    if ( v17 < 4096 )
      v17 = 4096;
  }
  else
  {
    v17 = 4096;
  }
  v40 = v17;
  qmemcpy(&unk_261A0, (const void *)a1, 0x130u);
  qmemcpy(&unk_262D0, (const void *)(a1 + 304), 0x130u);
  sub_6890(0, *(long long *)(a1 + 272), 0, v15, (unsigned char)byte_26408, v36);
  free((void *)(v37 + -8 - 8LL * *(long long *)(a1 + 576)));
  sub_6A70(a1);
  if ( dword_26580 == 4 )
    v18 = (long long *)sub_6C70((long long *)a1);
  else
    v18 = sub_6D50((long long *)a1);
  if ( !byte_26554 && !stru_264C0.fastmap )
  {
    v10 = v18 != 0;
    goto LABEL_33;
  }
  if ( v18 )
  {
    v19 = v18;
    do
    {
      v20 = (long long *)sub_EAF0(v19);
      v21 = v19;
      v19 = (long long *)*v20;
      *v20 = 0;
      v22 = v20;
      v23 = sub_F1A0(v21, v32, v33, v34, v35);
      *v22 = v19;
    }
    while ( v19 && !v23 );
    v10 = v23 != 0;
LABEL_33:
    if ( !byte_264A1 )
    {
      if ( v10 )
      {
LABEL_35:
        v24 = qword_26548;
        if ( !qword_26548 )
          v24 = *(long long *)(a1 + 312);
        v25 = qword_26540;
        if ( !qword_26540 )
          v25 = *(long long *)(a1 + 8);
        sub_E280(v25, v24, *(long long *)(a1 + 608) != 0);
        switch ( dword_26580 )
        {
          case 1:
            sub_CCF0(v18);
            break;
          case 2:
            sub_8870(v18, 0);
            break;
          case 3:
            sub_8870(v18, 1);
            break;
          case 4:
            sub_AA10(v18);
            break;
          case 5:
            sub_AA30(v18);
            break;
          case 6:
            sub_AA50(v18);
            break;
          case 7:
            sub_B430(v18);
            break;
          case 8:
            sub_D5B0(v18);
            break;
          default:
            abort();
        }
        sub_E550();
        goto LABEL_48;
      }
      goto LABEL_47;
    }
LABEL_58:
    sub_73B0(v10, a1);
    goto LABEL_48;
  }
  if ( byte_264A1 )
  {
    v10 = 0;
    goto LABEL_58;
  }
LABEL_47:
  v10 = 0;
  if ( !byte_26578 )
    goto LABEL_35;
LABEL_48:
  free(*(void **)(a1 + 256));
  free(ptr);
  free(*(void **)(a1 + 248));
  free((void *)(*(long long *)(a1 + 184) + 8LL * *(long long *)(a1 + 192)));
  free(*(void **)(a1 + 552));
  free((void *)(*(long long *)(a1 + 488) + 8LL * *(long long *)(a1 + 496)));
  while ( v18 )
  {
    v26 = v18;
    v18 = (long long *)*v18;
    free(v26);
  }
  if ( (unsigned int)(dword_26580 - 4) <= 1 )
  {
    v27 = (char *)&qword_26540;
    v28 = (char *)(a1 + 288);
    do
    {
      if ( *v28 )
      {
        v29 = dcgettext(0, "No newline at end of file", 5);
        v30 = *(const char **)v27;
        if ( !*(long long *)v27 )
          v30 = (const char *)*((long long *)v28 - 35);
        error(0, 0, "%s: %s\n", v30, v29);
        v10 = 2;
      }
      v27 += 8;
      v28 += 304;
    }
    while ( v27 != &byte_26550 );
  }
LABEL_17:
  v11 = *(void **)(a1 + 464);
  if ( *(void **)(a1 + 160) != v11 )
  {
    free(*(void **)(a1 + 160));
    v11 = *(void **)(a1 + 464);
  }
  free(v11);
  return v10;
}



// Function: print_context_label @ 0x7ab0
int print_context_label(const char *a1, long long a2, const char *a3, const char *a4)
{
  struct tm *v7; // r14
  unsigned int v8; // eax
  int v9; // ebx
  char v10[56]; // [rsp+0h] [rbp-78h] BYREF
  unsigned long long v11; // [rsp+38h] [rbp-40h]

  v11 = __readfsqword(0x28u);
  sub_EB70(0);
  if ( a4 )
  {
    __fprintf_chk(stream, 1, "%s %s", a1, a4);
  }
  else
  {
    v7 = localtime((const time_t *)(a2 + 104));
    v8 = sub_154D0(a2 + 16);
    v9 = v8;
    if ( !v7 || !sub_13D10(v10, 43, qword_26400, v7, 0, v8) )
      __sprintf_chk(v10, 1, 43, "%ld.%.9d", *(long long *)(a2 + 104), v9);
    __fprintf_chk(stream, 1, "%s %s\t%s", a1, a3, v10);
  }
  sub_EB70(3);
  return putc_unlocked(10, stream);
}



// Function: mark_ignorable @ 0x7be0
unsigned long long mark_ignorable(long long *a1)
{
  long long *v1; // rbx
  long long *v2; // rbp
  int v3; // eax
  char v5[8]; // [rsp+8h] [rbp-60h] BYREF
  char v6[8]; // [rsp+10h] [rbp-58h] BYREF
  char v7[8]; // [rsp+18h] [rbp-50h] BYREF
  char v8[8]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v9; // [rsp+28h] [rbp-40h]

  v9 = __readfsqword(0x28u);
  if ( a1 )
  {
    v1 = a1;
    do
    {
      v2 = v1;
      v1 = (long long *)*v1;
      *v2 = 0;
      v3 = sub_F1A0(v2, v5, v6, v7, v8);
      *v2 = v1;
      *((char *)v2 + 40) = v3 == 0;
    }
    while ( v1 );
  }
  return v9 - __readfsqword(0x28u);
}



// Function: find_function @ 0x7c80
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



// Function: print_context_function @ 0x7d30
size_t print_context_function(FILE *stream, char *ptr)
{
  int v2; // r14d
  char *v6; // rbx
  char v7; // di
  char *v8; // r13
  char v9; // al
  long long v10; // rdx
  int v11; // ebx
  char *v12; // rax
  char v13; // r15
  char *v14; // rbp

  v2 = 0;
  v6 = ptr;
  putc_unlocked(32, stream);
  while ( 1 )
  {
    v7 = *v6;
    v8 = v6++;
    v9 = ((long long (*)(void))sub_F850)();
    if ( v7 == 10 || !v9 )
      break;
    ++v2;
  }
  v10 = (unsigned int)(v2 + 39);
  v11 = v2;
  v12 = &ptr[v2 + 1];
  if ( v7 != 10 )
  {
    do
    {
      if ( ++v11 > (int)v10 )
        break;
      v13 = *v12++;
    }
    while ( v13 != 10 );
  }
  if ( v11 > v2 )
  {
    v14 = &ptr[v11 - 1];
    do
    {
      if ( !(unsigned char)sub_F850((unsigned char)*v14, stream, v10) )
        break;
      --v11;
      --v14;
    }
    while ( v2 != v11 );
  }
  return fwrite_unlocked(v8, 1u, v11 - v2, stream);
}



// Function: print_context_number_range @ 0x7e00
unsigned long long print_context_number_range(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+8h] [rbp-20h] BYREF
  long long v5; // [rsp+10h] [rbp-18h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  sub_F0C0(a1, a2, a3, &v4, &v5);
  if ( v5 > v4 )
    __fprintf_chk(stream, 1, "%ld,%ld", v4, v5);
  else
    __fprintf_chk(stream, 1, "%ld", v5);
  return v6 - __readfsqword(0x28u);
}



// Function: pr_context_hunk @ 0x7e90
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
  char *v10; // r13
  long long v11; // rdx
  long long v12; // rcx
  FILE *v13; // rbp
  long long v14; // r15
  long long *v15; // r14
  long long v16; // rax
  const char *v17; // rdi
  long long i; // r15
  long long v19; // rax
  const char *v20; // rdi
  long long v22; // [rsp+8h] [rbp-60h] BYREF
  long long v23; // [rsp+10h] [rbp-58h] BYREF
  long long v24; // [rsp+18h] [rbp-50h] BYREF
  long long v25; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v26; // [rsp+28h] [rbp-40h]

  v1 = a1;
  v26 = __readfsqword(0x28u);
  v2 = sub_F1A0(a1, &v22, &v23, &v24, &v25);
  if ( v2 )
  {
    v3 = v2;
    v4 = v22 - qword_26570;
    if ( v22 - qword_26570 < -qword_26288 )
      v4 = -qword_26288;
    v5 = v24 - qword_26570;
    v22 = v4;
    if ( v24 - qword_26570 < -qword_26288 )
      v5 = -qword_26288;
    v24 = v5;
    v6 = qword_26270 - 1;
    v7 = v25;
    if ( qword_26270 - qword_26570 > v23 )
      v6 = qword_26570 + v23;
    v23 = v6;
    v8 = qword_263A0 - 1;
    if ( qword_263A0 - qword_26570 > v25 )
      v8 = v25 + qword_26570;
    v25 = v8;
    if ( buffer.fastmap )
    {
      v9 = qword_26258;
      v10 = (char *)sub_7C80(qword_26258, v4);
      sub_E2B0(v9, v4, v11, v12);
      v13 = stream;
      fputs_unlocked("***************", stream);
      if ( v10 )
        sub_7D30(v13, v10);
    }
    else
    {
      sub_E2B0(qword_263A0 - qword_26570, v4, v8, v7);
      v13 = stream;
      fputs_unlocked("***************", stream);
    }
    putc_unlocked(10, v13);
    sub_EB70(4);
    fputs_unlocked("*** ", v13);
    sub_7E00((long long)&unk_261A0, v22, v23);
    fputs_unlocked(" ****", v13);
    sub_EB70(3);
    putc_unlocked(10, v13);
    if ( (v3 & 1) != 0 )
    {
      v14 = v22;
      if ( v22 <= v23 )
      {
        v15 = v1;
        do
        {
          sub_EB70(2);
          if ( !v15 )
            goto LABEL_41;
          while ( 1 )
          {
            v16 = v15[3];
            if ( v16 + v15[2] > v14 )
              break;
            v15 = (long long *)*v15;
            if ( !v15 )
              goto LABEL_41;
          }
          if ( v16 <= v14 )
          {
            v17 = "-";
            if ( (long long)v15[1] > 0 )
              v17 = "!";
          }
          else
          {
LABEL_41:
            v17 = " ";
          }
          sub_EF20(v17, 8 * v14 + qword_26258, 1);
          sub_EB70(3);
          if ( *(char *)(*(long long *)(qword_26258 + 8 * v14 + 8) - 1LL) == 10 )
            putc_unlocked(10, v13);
          ++v14;
        }
        while ( v23 >= v14 );
      }
    }
    sub_EB70(4);
    fputs_unlocked("--- ", v13);
    sub_7E00((long long)&unk_262D0, v24, v25);
    fputs_unlocked(" ----", v13);
    sub_EB70(3);
    putc_unlocked(10, v13);
    if ( (v3 & 2) != 0 )
    {
      for ( i = v24; v25 >= i; ++i )
      {
        sub_EB70(1);
        if ( !v1 )
          goto LABEL_40;
        while ( 1 )
        {
          v19 = v1[4];
          if ( v19 + v1[1] > i )
            break;
          v1 = (long long *)*v1;
          if ( !v1 )
            goto LABEL_40;
        }
        if ( v19 <= i )
        {
          v20 = "+";
          if ( (long long)v1[2] > 0 )
            v20 = "!";
        }
        else
        {
LABEL_40:
          v20 = " ";
        }
        sub_EF20(v20, 8 * i + qword_26388, 1);
        sub_EB70(3);
        if ( *(char *)(*(long long *)(qword_26388 + 8 * i + 8) - 1LL) == 10 )
          putc_unlocked(10, v13);
      }
    }
  }
  return v26 - __readfsqword(0x28u);
}



// Function: print_unidiff_number_range @ 0x8280
unsigned long long print_unidiff_number_range(long long a1, long long a2, long long a3)
{
  const char *v3; // rdx
  long long v5; // [rsp+8h] [rbp-20h] BYREF
  long long v6; // [rsp+10h] [rbp-18h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-10h]

  v7 = __readfsqword(0x28u);
  sub_F0C0(a1, a2, a3, &v5, &v6);
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



// Function: pr_unidiff_hunk @ 0x8320
unsigned long long pr_unidiff_hunk(long long *a1)
{
  long long *v1; // r14
  long long v2; // rdx
  long long v3; // rsi
  long long v4; // rdx
  long long v5; // rdx
  long long v6; // rcx
  long long v7; // rdx
  long long v8; // rdi
  char *v9; // rbp
  long long v10; // rdx
  long long v11; // rcx
  FILE *v12; // rbx
  long long v13; // r13
  long long v14; // rbp
  char **v15; // r12
  long long v17; // rbp
  long long v18; // r15
  long long v19; // r12
  long long v20; // r12
  long long v21; // rbp
  long long v22; // r15
  long long v23; // r12
  long long v24; // r12
  long long v25; // [rsp+0h] [rbp-78h]
  long long v26; // [rsp+8h] [rbp-70h]
  long long v27; // [rsp+8h] [rbp-70h]
  long long v28; // [rsp+18h] [rbp-60h] BYREF
  long long v29; // [rsp+20h] [rbp-58h] BYREF
  long long v30; // [rsp+28h] [rbp-50h] BYREF
  long long v31; // [rsp+30h] [rbp-48h] BYREF
  unsigned long long v32; // [rsp+38h] [rbp-40h]

  v1 = a1;
  v32 = __readfsqword(0x28u);
  if ( (unsigned int)sub_F1A0(a1, &v28, &v29, &v30, &v31) )
  {
    v2 = v28 - qword_26570;
    if ( v28 - qword_26570 < -qword_26288 )
      v2 = -qword_26288;
    v28 = v2;
    v3 = v2;
    v4 = v30 - qword_26570;
    if ( v30 - qword_26570 < -qword_26288 )
      v4 = -qword_26288;
    v30 = v4;
    v5 = qword_26270 - 1;
    v6 = v31;
    if ( qword_26270 - qword_26570 > v29 )
      v5 = qword_26570 + v29;
    v29 = v5;
    v7 = qword_263A0 - 1;
    if ( qword_263A0 - qword_26570 > v31 )
      v7 = v31 + qword_26570;
    v31 = v7;
    if ( buffer.fastmap )
    {
      v8 = qword_26258;
      v9 = (char *)sub_7C80(qword_26258, v3);
      sub_E2B0(v8, v3, v10, v11);
      v12 = stream;
      sub_EB70(4);
      fputs_unlocked("@@ -", v12);
      sub_8280((long long)&unk_261A0, v28, v29);
      fputs_unlocked(" +", v12);
      sub_8280((long long)&unk_262D0, v30, v31);
      fputs_unlocked(" @@", v12);
      sub_EB70(3);
      if ( v9 )
        sub_7D30(v12, v9);
    }
    else
    {
      sub_E2B0(qword_263A0 - qword_26570, v3, v7, v6);
      v12 = stream;
      sub_EB70(4);
      fputs_unlocked("@@ -", v12);
      sub_8280((long long)&unk_261A0, v28, v29);
      fputs_unlocked(" +", v12);
      sub_8280((long long)&unk_262D0, v30, v31);
      fputs_unlocked(" @@", v12);
      sub_EB70(3);
    }
    putc_unlocked(10, v12);
    v13 = v28;
    v25 = v30;
    while ( v29 >= v13 || v31 >= v25 )
    {
      while ( !v1 || v1[3] > v13 )
      {
        v14 = v13 + 1;
        v15 = (char **)(qword_26258 + 8 * v13);
        if ( !byte_26491 || **v15 != 10 )
          putc_unlocked(byte_26492 == 0 ? 32 : 9, v12);
        ++v13;
        sub_F0A0(0, v15);
        ++v25;
        if ( v29 < v14 && v31 < v25 )
          return v32 - __readfsqword(0x28u);
      }
      v17 = v1[2];
      v26 = v17 - 1;
      if ( v17 )
      {
        v18 = 8 * v13;
        do
        {
          while ( 1 )
          {
            v19 = qword_26258;
            sub_EB70(2);
            putc_unlocked(45, v12);
            v20 = v18 + v19;
            if ( byte_26492 && (!byte_26491 || **(char **)v20 != 10) )
              putc_unlocked(9, v12);
            sub_EF20(0, v20, 1);
            sub_EB70(3);
            if ( *(char *)(*(long long *)(v20 + 8) - 1LL) == 10 )
              break;
            v18 += 8;
            if ( !--v17 )
              goto LABEL_34;
          }
          v18 += 8;
          putc_unlocked(10, v12);
          --v17;
        }
        while ( v17 );
LABEL_34:
        v13 += v26 + 1;
      }
      v21 = v1[1];
      v27 = v21 - 1;
      if ( v21 )
      {
        v22 = 8 * v25;
        do
        {
          while ( 1 )
          {
            v23 = qword_26388;
            sub_EB70(1);
            putc_unlocked(43, v12);
            v24 = v22 + v23;
            if ( byte_26492 && (!byte_26491 || **(char **)v24 != 10) )
              putc_unlocked(9, v12);
            sub_EF20(0, v24, 1);
            sub_EB70(3);
            if ( *(char *)(*(long long *)(v24 + 8) - 1LL) == 10 )
              break;
            v22 += 8;
            if ( !--v21 )
              goto LABEL_44;
          }
          v22 += 8;
          putc_unlocked(10, v12);
          --v21;
        }
        while ( v21 );
LABEL_44:
        v25 += v27 + 1;
      }
      v1 = (long long *)*v1;
    }
  }
  return v32 - __readfsqword(0x28u);
}



// Function: find_hunk @ 0x8770
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



// Function: print_context_header @ 0x87e0
int print_context_header(long long a1, const char **a2, char a3)
{
  long long v4; // rbp
  const char *v5; // rdx

  v4 = a1 + 304;
  v5 = *a2;
  if ( a3 )
  {
    sub_7AB0("---", a1, v5, (const char *)qword_26540);
    return sub_7AB0("+++", v4, a2[1], (const char *)qword_26548);
  }
  else
  {
    sub_7AB0("***", a1, v5, (const char *)qword_26540);
    return sub_7AB0("---", v4, a2[1], (const char *)qword_26548);
  }
}



// Function: print_context_script @ 0x8870
long long print_context_script(long long *a1, char a2)
{
  unsigned long long (*v2)(long long *); // rdx
  long long *i; // rax
  long long v5; // rax

  if ( byte_26554 || stru_264C0.fastmap )
  {
    sub_7BE0(a1);
  }
  else
  {
    for ( i = a1; i; i = *(long long **)v5 )
    {
      *((char *)i + 40) = 0;
      v5 = *i;
      if ( !v5 )
        break;
      *(char *)(v5 + 40) = 0;
    }
  }
  v2 = sub_8320;
  qword_26178 = -qword_26288;
  qword_26170 = 0x7FFFFFFFFFFFFFFFLL;
  if ( !a2 )
    v2 = sub_7E90;
  return sub_EB10(a1, sub_8770, v2);
}



// Function: exclude_options @ 0x8910
long long exclude_options()
{
  return byte_26551 == 0 ? 0x10000000 : 268435472;
}



// Function: add_regexp @ 0x8930
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
      v10 = sub_15FA0(v5, v7);
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



// Function: usage @ 0x8a10
long long usage()
{
  long long v0; // r12
  char *v1; // rax
  long long v2; // rdx
  const char **v3; // r12
  char *v4; // rax
  FILE *v5; // rbp
  char *v6; // rax
  char v7; // al
  const char *v8; // rsi
  char *i; // r14
  char *v10; // rbx
  int v11; // edx
  char *v12; // rax

  v0 = qword_36860;
  v1 = dcgettext(0, "Usage: %s [OPTION]... FILES\n", 5);
  v2 = v0;
  v3 = (const char **)off_24F20;
  __printf_chk(1, v1, v2);
  v4 = dcgettext(0, "Compare FILES line by line.", 5);
  __printf_chk(1, "%s\n\n", v4);
  v5 = stdout;
  v6 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
  fputs_unlocked(v6, v5);
  v7 = 32;
  v8 = "    --normal                  output a normal diff (the default)";
  while ( !v7 )
  {
    ++v3;
    putchar_unlocked(10);
    v8 = *v3;
    if ( !*v3 )
      return sub_15E90();
LABEL_3:
    v7 = *v8;
  }
  for ( i = dcgettext(0, v8, 5); ; i = v10 )
  {
    v12 = strchr(i, 10);
    if ( !v12 )
      break;
    v10 = v12 + 1;
    v11 = (int)v12 + 1 - (int)i;
    if ( v11 > 4095 )
      __assert_fail("msglen < 4096", "diff.c", 0x3F8u, "usage");
    __printf_chk(1, "  %.*s", v11, i);
  }
  ++v3;
  __printf_chk(1, &aS[2 * (*i != 32 && *i != 45)], i);
  v8 = *v3;
  if ( *v3 )
    goto LABEL_3;
  return sub_15E90();
}



// Function: summarize_regexp_list @ 0x8ba0
void summarize_regexp_list(const char **a1)
{
  const char *v1; // rbp
  long long v2; // rax
  bool v3; // zf
  const char *v4; // rax

  if ( *a1 )
  {
    v1 = a1[4];
    v2 = sub_15F50(256);
    v3 = *((char *)a1 + 24) == 0;
    *((long long *)v1 + 4) = v2;
    if ( !v3 )
    {
      v4 = re_compile_pattern(*a1, (size_t)a1[1], (struct re_pattern_buffer *)a1[4]);
      if ( v4 )
      {
        error(2, 0, "%s: %s", *a1, v4);
        sub_8C00((char *)&dword_0 + 2);
      }
    }
  }
}



// Function: try_help @ 0x8c00
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
  v3 = qword_36860;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_8C60(2, 0, v5);
}



// Function: specify_value @ 0x8c60
void specify_value(const char **a1, const char *a2, long long a3)
{
  const char *v4; // rdi
  char *v6; // rax

  v4 = *a1;
  if ( v4 && strcmp(v4, a2) )
  {
    v6 = dcgettext(0, "conflicting %s option value '%s'", 5);
    error(0, 0, v6, a3, a2);
    sub_8C00(0, 0);
    sub_8CC0(0);
  }
  else
  {
    *a1 = a2;
  }
}



// Function: specify_colors_style @ 0x8cc0
long long specify_colors_style(char *s1)
{
  if ( s1 )
  {
    if ( !strcmp(s1, "auto") )
    {
      dword_2657C = 1;
      return 1;
    }
    else if ( !strcmp(s1, "always") )
    {
      dword_2657C = 2;
      return 2;
    }
    else if ( !strcmp(s1, "never") )
    {
      dword_2657C = 0;
      return 0;
    }
    else
    {
      sub_8C00("invalid color '%s'", (long long)s1);
      return sub_8D50("invalid color '%s'", s1);
    }
  }
  else
  {
    dword_2657C = 1;
    return 1;
  }
}



// Function: specify_style @ 0x8d50
long long specify_style(const char *a1)
{
  long long result; // rax

  result = (unsigned int)dword_26580;
  if ( dword_26580 != (int)a1 )
  {
    if ( dword_26580 )
    {
      sub_8C00("conflicting output style options", 0);
      return sub_8D80("conflicting output style options", 0);
    }
    else
    {
      dword_26580 = (int)a1;
    }
  }
  return result;
}



// Function: option_list @ 0x8d80
char *option_list(long long *a1, int a2)
{
  long long *v2; // rbx
  char **v3; // rbp
  long long v4; // r12
  long long v5; // r13
  char *v6; // rdi
  char *v7; // rax
  char *v8; // r12
  long long v9; // rsi

  if ( a2 <= 0 )
  {
    v7 = (char *)sub_15F50(1);
    v8 = v7;
  }
  else
  {
    v2 = a1;
    v3 = (char **)a1;
    v4 = 1;
    v5 = (long long)&a1[(unsigned int)(a2 - 1) + 1];
    do
    {
      v6 = *v3++;
      v4 += sub_144E0(v6) + 1;
    }
    while ( v3 != (char **)v5 );
    v7 = (char *)sub_15F50(v4);
    v8 = v7;
    do
    {
      *v7 = 32;
      v9 = *v2++;
      v7 = (char *)sub_14530(v7 + 1, v9);
    }
    while ( v2 != (long long *)v5 );
  }
  *v7 = 0;
  return v8;
}



// Function: set_mtime_to_now @ 0x8e10
long long set_mtime_to_now(long long a1)
{
  return sub_10A50((struct timespec *)(a1 + 88));
}



// Function: check_stdout @ 0x8e20
int check_stdout()
{
  int result; // eax
  char *v1; // rdi
  long long v2; // rdx

  if ( ferror_unlocked(stdout) )
  {
    sub_E230("write failed");
    goto LABEL_5;
  }
  result = fclose(stdout);
  if ( result )
  {
LABEL_5:
    v1 = dcgettext(0, "standard output", 5);
    sub_E1F0(v1);
    return sub_8E70(v1, "standard output", v2);
  }
  return result;
}



// Function: compare_files @ 0x8e70
long long compare_files(long long a1, const char *a2, const char *a3, long long a4, int *a5)
{
  const char *v5; // r13
  const char *v6; // r12
  long long v7; // rbx
  bool v8; // dl
  _OWORD *v9; // rbp
  char *v10; // r14
  long long v11; // rax
  long long v12; // rdi
  int v13; // r13d
  struct stat *v14; // r15
  __m128i si128; // xmm1
  __m128i v16; // xmm2
  __m128i v17; // xmm3
  __m128i v18; // xmm4
  __m128i v19; // xmm5
  __m128i v20; // xmm6
  __m128i v21; // xmm7
  __m128i v22; // xmm0
  char v23; // di
  _OWORD *v24; // rax
  _OWORD *v25; // rsi
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
  const char *v37; // rdi
  bool v38; // al
  long long v39; // r14
  long long v40; // rax
  struct stat *v41; // rsi
  bool v42; // al
  long long v43; // rdx
  long long v44; // rcx
  long long v45; // rdx
  short v46; // di
  int v47; // eax
  long long v48; // r9
  long long v49; // rcx
  int v50; // edx
  unsigned int v51; // edi
  bool v52; // di
  long long v53; // rdx
  long long v54; // rsi
  int *v55; // rax
  long long v56; // rdx
  long long v57; // rcx
  int v58; // ebx
  int v59; // edi
  int v60; // edi
  long long v61; // rdx
  long long v62; // rcx
  __off_t v63; // rax
  long long st_size; // rax
  long long v65; // rdx
  long long v66; // rcx
  const char *v67; // rax
  char *v68; // r12
  int v69; // eax
  long long v70; // rdx
  long long v71; // rcx
  int v72; // eax
  long long v73; // rdx
  long long v74; // rdx
  long long v75; // rcx
  const char *v76; // rdx
  const char *v77; // rsi
  char *v78; // rax
  char *ptr; // [rsp+0h] [rbp-2C8h]
  char *file; // [rsp+8h] [rbp-2C0h]
  unsigned int fileb; // [rsp+8h] [rbp-2C0h]
  unsigned int filea; // [rsp+8h] [rbp-2C0h]
  char *v83; // [rsp+18h] [rbp-2B0h]
  _OWORD v84[38]; // [rsp+20h] [rbp-2A8h] BYREF
  long long v85; // [rsp+280h] [rbp-48h]
  unsigned long long v86; // [rsp+288h] [rbp-40h]

  v5 = a3;
  v6 = a2;
  v7 = a1;
  v86 = __readfsqword(0x28u);
  v8 = a3 == 0;
  if ( a2 && !v8 )
  {
    v9 = v84;
    v85 = a1;
    memset(v84, 0, sizeof(v84));
    LODWORD(v84[0]) = -2;
    goto LABEL_10;
  }
  if ( byte_26585 == 1 && !v8 )
  {
    v9 = v84;
    v85 = a1;
    memset(v84, 0, sizeof(v84));
    if ( a2 )
    {
      LODWORD(v84[0]) = -2;
      LODWORD(v84[19]) = -2;
      goto LABEL_7;
    }
    LODWORD(v84[0]) = -1;
    goto LABEL_82;
  }
  if ( !byte_26586 )
  {
    v26 = 1;
    if ( !a2 )
      v6 = v5;
    sub_E0E0("Only in %s: %s\n", *(long long *)(a1 + 304LL * (a2 == 0) + 8), v6);
    return v26;
  }
  v9 = v84;
  v85 = a1;
  memset(v84, 0, sizeof(v84));
  if ( a2 )
  {
    LODWORD(v84[0]) = -2;
    if ( !v5 )
    {
      LODWORD(v84[19]) = -1;
      v5 = a2;
      goto LABEL_11;
    }
LABEL_10:
    LODWORD(v84[19]) = -2;
    goto LABEL_11;
  }
  LODWORD(v84[0]) = -1;
  if ( !v5 )
  {
    LODWORD(v84[19]) = -1;
LABEL_80:
    v6 = 0;
LABEL_11:
    if ( !a1 )
      goto LABEL_8;
LABEL_12:
    v11 = sub_10920(*(long long *)(a1 + 8), v6, 0);
    v12 = *(long long *)(a1 + 312);
    *((long long *)&v84[0] + 1) = v11;
    v10 = (char *)v11;
    ptr = (char *)sub_10920(v12, v5, 0);
    v5 = ptr;
    goto LABEL_13;
  }
LABEL_82:
  LODWORD(v84[19]) = -2;
  if ( !v5 )
    goto LABEL_80;
LABEL_7:
  v6 = v5;
  if ( a1 )
    goto LABEL_12;
LABEL_8:
  *((long long *)&v84[0] + 1) = v6;
  v10 = 0;
  ptr = 0;
LABEL_13:
  *((long long *)&v84[19] + 1) = v5;
  v13 = 0;
  v14 = (struct stat *)&v84[1];
  while ( 1 )
  {
    if ( LODWORD(v14[-1].__unused[1]) == -1 )
      goto LABEL_18;
    if ( v13 && !strcmp(*((const char **)&v84[19] + 1), *((const char **)&v84[0] + 1)) )
    {
      si128 = _mm_load_si128((const __m128i *)&v84[2]);
      v16 = _mm_load_si128((const __m128i *)&v84[3]);
      v17 = _mm_load_si128((const __m128i *)&v84[4]);
      v18 = _mm_load_si128((const __m128i *)&v84[5]);
      v84[20] = _mm_load_si128((const __m128i *)&v84[1]);
      v19 = _mm_load_si128((const __m128i *)&v84[6]);
      LODWORD(v84[19]) = v84[0];
      v20 = _mm_load_si128((const __m128i *)&v84[7]);
      v84[21] = si128;
      v21 = _mm_load_si128((const __m128i *)&v84[8]);
      v22 = _mm_load_si128((const __m128i *)&v84[9]);
      v84[22] = v16;
      v84[23] = v17;
      v84[24] = v18;
      v84[25] = v19;
      v84[26] = v20;
      v84[27] = v21;
      v84[28] = v22;
      goto LABEL_18;
    }
    v37 = (const char *)v14[-1].__unused[2];
    if ( !strcmp(v37, "-") )
    {
      LODWORD(v14[-1].__unused[1]) = 0;
      isatty(0);
      if ( fstat(0, v14) )
        goto LABEL_46;
      if ( (v14->st_mode & 0xF000) == 0x8000 )
      {
        v63 = lseek(0, 0, 1);
        if ( v63 < 0 )
        {
          LODWORD(v14[-1].__unused[1]) = -3 - *__errno_location();
        }
        else
        {
          v14->st_size -= v63;
          st_size = v14->st_size;
          if ( st_size < 0 )
            st_size = 0;
          v14->st_size = st_size;
        }
      }
      sub_8E10((long long)v14);
    }
    else
    {
      if ( byte_26550 )
      {
        if ( !lstat(v37, v14) )
          goto LABEL_18;
LABEL_46:
        a5 = __errno_location();
        LODWORD(v14[-1].__unused[1]) = -3 - *a5;
        goto LABEL_18;
      }
      if ( stat(v37, v14) )
        goto LABEL_46;
    }
LABEL_18:
    v14 = (struct stat *)((char *)v14 + 304);
    if ( v13 == 1 )
      break;
    v13 = 1;
  }
  v23 = byte_26586;
  v24 = v84;
  v25 = v84;
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
    if ( *(int *)v24 == -2 )
      break;
    if ( (*(int *)v24 == -12 || *(int *)v24 == -5) && !v7 && ((*((int *)v25 + 76) + 2) & 0xFFFFFFFD) == 0 )
      goto LABEL_40;
LABEL_22:
    if ( v26 == 2 )
      goto LABEL_23;
LABEL_41:
    v24 += 19;
    v25 -= 19;
  }
  v50 = *((int *)v24 + 10);
  if ( (v50 & 0xF000) != 0x8000 || (v50 & 0x1FF) != 0 || *((long long *)v24 + 8) )
    goto LABEL_22;
LABEL_40:
  *(int *)v24 = -1;
  if ( v26 != 2 )
    goto LABEL_41;
LABEL_23:
  v28 = (const char *)LODWORD(v84[0]);
  if ( LODWORD(v84[0]) == -1 )
  {
    memset(&v84[1], 0, 0x90u);
    DWORD2(v84[2]) = DWORD2(v84[21]);
    if ( LODWORD(v84[19]) != -1 )
      goto LABEL_48;
    memset(&v84[20], 0, 0x90u);
    DWORD2(v84[21]) = DWORD2(v84[2]);
LABEL_49:
    if ( !v7 )
    {
      v38 = (WORD4(v84[2]) & 0xF000) == 0x4000;
      if ( v38 != ((WORD4(v84[21]) & 0xF000) == 0x4000) )
      {
        v39 = (WORD4(v84[2]) & 0xF000) != 0x4000;
        v83 = (char *)*((long long *)&v84[19 * v38] + 1);
        v40 = sub_F670(v83);
        file = (char *)(304 * v39);
        v10 = (char *)sub_A4F0(*((long long *)&v84[19 * v39] + 1), v40);
        *(long long *)&file[(long long)v84 + 8] = v10;
        if ( !strcmp(v83, "-") )
        {
          sub_E230("cannot compare '-' to a directory");
          goto LABEL_184;
        }
        v41 = (struct stat *)((char *)&v84[1] + (long long)file);
        if ( byte_26550 )
          v42 = lstat(v10, v41) != 0;
        else
          v42 = stat(v10, v41) != 0;
        v28 = (const char *)LODWORD(v84[0]);
        if ( v42 )
        {
          sub_DF30(v10, LODWORD(v84[0]), v43, v44);
          goto LABEL_28;
        }
      }
    }
    v45 = LODWORD(v84[19]);
    if ( (int)v28 == -1 )
    {
      if ( LODWORD(v84[19]) == -1 )
        goto LABEL_93;
      v46 = WORD4(v84[21]);
      v49 = 0;
      v47 = WORD4(v84[2]) & 0xF000;
LABEL_86:
      v51 = v46 & 0xF000;
      v48 = v51;
      v52 = v51 == 0x4000;
      if ( v47 == 0x4000 )
      {
        if ( v52 )
        {
          if ( dword_26580 != 7 )
          {
            if ( v7 && !byte_26608 )
            {
              sub_E0E0(
                "Common subdirectories: %s and %s\n",
                *((const char **)&v84[0] + 1),
                *((const char **)&v84[19] + 1));
              goto LABEL_93;
            }
            goto LABEL_91;
          }
LABEL_184:
          sub_E230("-D option not supported with directories");
          goto LABEL_185;
        }
      }
      else if ( !v52 )
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
        LODWORD(v7) = sub_A0C0(v84, compare_files, LODWORD(v84[19]), v49, a5, v48, ptr);
        goto LABEL_92;
      }
LABEL_118:
      if ( LODWORD(v84[19]) != -1 )
        goto LABEL_112;
      goto LABEL_119;
    }
    v46 = WORD4(v84[21]);
    if ( LODWORD(v84[19]) == -1 )
    {
LABEL_85:
      v49 = 0;
      v47 = WORD4(v84[2]) & 0xF000;
      goto LABEL_86;
    }
    if ( v84[1] == v84[20] )
    {
LABEL_108:
      if ( DWORD2(v84[2]) == DWORD2(v84[21]) )
      {
        if ( *(long long *)&v84[2] == *(long long *)&v84[21]
          && *(long long *)((char *)&v84[2] + 12) == *(long long *)((char *)&v84[21] + 12)
          && *(long long *)&v84[4] == *(long long *)&v84[23]
          && *((long long *)&v84[6] + 1) == *((long long *)&v84[25] + 1)
          && *((long long *)&v84[7] + 1) == *((long long *)&v84[26] + 1) )
        {
          if ( byte_26578 )
            goto LABEL_93;
          v49 = 1;
          v47 = WORD4(v84[21]) & 0xF000;
        }
        else
        {
          v49 = 0;
          v47 = WORD4(v84[21]) & 0xF000;
        }
        goto LABEL_86;
      }
      goto LABEL_85;
    }
    v47 = WORD4(v84[2]) & 0xF000;
    v48 = WORD4(v84[21]) & 0xF000;
    if ( v47 == 24576 )
    {
      if ( (int)v48 != 24576 )
        goto LABEL_62;
    }
    else
    {
      v49 = 0;
      if ( v47 != 0x2000 )
        goto LABEL_86;
      if ( (int)v48 != 0x2000 )
        goto LABEL_62;
    }
    if ( *((long long *)&v84[22] + 1) == *((long long *)&v84[3] + 1) )
      goto LABEL_108;
LABEL_62:
    v49 = 0;
    if ( (int)v48 != 0x4000 )
    {
LABEL_63:
      if ( v7 && ((v47 & 0xFFFFDFFF) != 0x8000 || (v48 & 0xFFFFDFFF) != 0x8000) )
      {
        if ( (int)v28 == -1 || LODWORD(v84[19]) == -1 )
        {
LABEL_68:
          v26 = 1;
          sub_E0E0("Only in %s: %s\n", *(const char **)(v7 + 304LL * ((int)v28 == -1) + 8), v6);
          goto LABEL_28;
        }
LABEL_112:
        v26 = 1;
        sub_10830(&v84[20], v28, v45, v49);
        sub_10830(&v84[1], v28, v56, v57);
        sub_DF60("File %s is a %s while file %s is a %s\n");
        goto LABEL_28;
      }
      LOBYTE(v45) = (int)v48 == 40960;
      if ( v47 == 40960 || (int)v48 == 40960 )
      {
        if ( !byte_26550 )
          __assert_fail("no_dereference_symlinks", "diff.c", 0x549u, "compare_files");
        if ( v47 != 40960 || (int)v48 != 40960 )
          goto LABEL_112;
        v9 = (_OWORD *)sub_16360(*((long long *)&v84[0] + 1), v28, v45, v49);
        if ( !v9 )
          goto LABEL_181;
        v67 = (const char *)sub_16360(*((long long *)&v84[19] + 1), v28, v65, v66);
        v68 = (char *)v67;
        if ( !v67 )
          goto LABEL_182;
        if ( strcmp((const char *)v9, v67) )
        {
          v26 = 1;
          sub_E0E0("Symbolic links %s and %s differ\n", *((const char **)&v84[0] + 1), *((const char **)&v84[19] + 1));
          free(v9);
          free(v68);
          goto LABEL_28;
        }
        free(v9);
        free(v68);
LABEL_93:
        v26 = 0;
        if ( byte_26584 && (WORD4(v84[2]) & 0xF000) != 0x4000 )
        {
          v53 = qword_26548;
          v54 = qword_26540;
          if ( !qword_26548 )
            v53 = *((long long *)&v84[19] + 1);
          if ( !qword_26540 )
            v54 = *((long long *)&v84[0] + 1);
          sub_E0E0("Files %s and %s are identical\n", v54, v53);
        }
        goto LABEL_29;
      }
      if ( ((unsigned char)byte_26553 & (v47 == 0x8000)) != 0
        && (int)v48 == 0x8000
        && *(long long *)&v84[4] > 0LL
        && *(long long *)&v84[4] != *(long long *)&v84[23]
        && *(long long *)&v84[23] > 0 )
      {
        v76 = (const char *)qword_26548;
        if ( !qword_26548 )
          v76 = (const char *)*((long long *)&v84[19] + 1);
        v77 = (const char *)qword_26540;
        if ( !qword_26540 )
          v77 = (const char *)*((long long *)&v84[0] + 1);
        v26 = 1;
        sub_E0E0("Files %s and %s differ\n", v77, v76);
        goto LABEL_28;
      }
      if ( (int)v28 == -2
        && (v28 = 0,
            filea = v49,
            v72 = open(*((const char **)&v84[0] + 1), 0, 0),
            LODWORD(v49) = filea,
            LODWORD(v84[0]) = v72,
            v72 < 0) )
      {
        sub_DF30(*((long long *)&v84[0] + 1), 0, v73, filea);
        LODWORD(v49) = filea;
        if ( LODWORD(v84[19]) != -2 )
          goto LABEL_135;
        v58 = 2;
      }
      else
      {
        v58 = 0;
        if ( LODWORD(v84[19]) != -2 )
          goto LABEL_151;
      }
      if ( (int)v49 )
      {
        LODWORD(v84[19]) = v84[0];
      }
      else
      {
        v28 = 0;
        LODWORD(v84[19]) = open(*((const char **)&v84[19] + 1), 0, 0);
        if ( SLODWORD(v84[19]) < 0 )
        {
          sub_DF30(*((long long *)&v84[19] + 1), 0, v74, v75);
          goto LABEL_135;
        }
      }
      if ( v58 )
      {
LABEL_135:
        v59 = v84[0];
        if ( SLODWORD(v84[0]) < 0 )
        {
          v60 = v84[19];
          if ( SLODWORD(v84[19]) < 0 )
            goto LABEL_28;
          goto LABEL_137;
        }
        LODWORD(v7) = 2;
        goto LABEL_157;
      }
LABEL_151:
      v69 = sub_7460((long long)v84);
      v59 = v84[0];
      LODWORD(v7) = v69;
      if ( SLODWORD(v84[0]) < 0 )
      {
LABEL_152:
        if ( SLODWORD(v84[19]) >= 0 && LODWORD(v84[19]) != LODWORD(v84[0]) && close(v84[19]) )
          goto LABEL_138;
LABEL_92:
        if ( !(int)v7 )
          goto LABEL_93;
LABEL_185:
        v26 = v7;
        goto LABEL_28;
      }
LABEL_157:
      if ( close(v59) )
      {
        sub_DF30(*((long long *)&v84[0] + 1), v28, v70, v71);
        v60 = v84[19];
        if ( SLODWORD(v84[19]) < 0 || LODWORD(v84[19]) == LODWORD(v84[0]) )
          goto LABEL_28;
LABEL_137:
        if ( !close(v60) )
          goto LABEL_28;
LABEL_138:
        sub_DF30(*((long long *)&v84[19] + 1), v28, v61, v62);
        goto LABEL_28;
      }
      goto LABEL_152;
    }
    goto LABEL_118;
  }
  v29 = -3 - LODWORD(v84[0]);
  if ( LODWORD(v84[19]) == -1 )
  {
    memset(&v84[20], 0, 0x90u);
    DWORD2(v84[21]) = DWORD2(v84[2]);
    if ( v29 < 0 )
      goto LABEL_49;
LABEL_26:
    v30 = __errno_location();
    v31 = *((long long *)&v84[0] + 1);
    v32 = v30;
    *v30 = -3 - (int)v28;
    sub_DF30(v31, v28, (unsigned int)(-3 - (int)v28), v33);
    v35 = (unsigned int)(-3 - LODWORD(v84[19]));
    if ( -3 - LODWORD(v84[19]) >= 0 )
      goto LABEL_27;
  }
  else
  {
    if ( v29 >= 0 )
      goto LABEL_26;
LABEL_48:
    if ( -3 - LODWORD(v84[19]) < 0 )
      goto LABEL_49;
    fileb = -3 - LODWORD(v84[19]);
    v55 = __errno_location();
    v35 = fileb;
    v32 = v55;
LABEL_27:
    *v32 = v35;
    sub_DF30(*((long long *)&v84[19] + 1), v28, v35, v34);
  }
LABEL_28:
  while ( fflush_unlocked(stdout) )
  {
    v28 = "standard output";
    v78 = dcgettext(0, "standard output", 5);
    sub_E1F0(v78);
LABEL_181:
    v13 = 0;
LABEL_182:
    sub_DF30(*((long long *)&v84[19 * v13] + 1), v28, v65, v66);
    free(v9);
    free(0);
  }
LABEL_29:
  free(v10);
  free(ptr);
  return v26;
}



// Function: dir_loop @ 0x9cd0
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



// Function: compare_collated @ 0x9da0
int compare_collated(char *s1, char *s2)
{
  int *v2; // rax
  bool v3; // zf

  v2 = __errno_location();
  v3 = byte_26551 == 0;
  *v2 = 0;
  if ( v3 )
    return strcoll(s1, s2);
  else
    return strcasecmp(s1, s2);
}



// Function: compare_names_for_qsort @ 0x9df0
int compare_names_for_qsort(char **a1, char **a2)
{
  const char *v2; // rbp
  const char *v3; // r12
  int result; // eax

  v2 = *a1;
  v3 = *a2;
  if ( !byte_266E8 )
    return strcmp(v2, v3);
  result = sub_9DA0(*a1, *a2);
  if ( !result )
    return strcmp(v2, v3);
  return result;
}



// Function: compare_names @ 0x9e40
int compare_names(char *a1, char *a2)
{
  int result; // eax

  if ( !byte_266E8 )
    return strcmp(a1, a2);
  result = sub_9DA0(a1, a2);
  if ( !result && !byte_26551 )
    return strcmp(a1, a2);
  return result;
}



// Function: dir_read @ 0x9e90
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
      v4 = sub_15F50(512);
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
        if ( !(unsigned char)sub_10220(qword_26410, v7->d_name) )
        {
          v10 = n + 1 + v23;
          na = n + 1;
          if ( v3 < v10 )
          {
            while ( v3 <= 0x3FFFFFFFFFFFFFFELL )
            {
              v3 *= 2LL;
              v11 = sub_15FA0(v5, v3);
              a2[2] = v11;
              v5 = (const char *)v11;
              if ( v3 >= v10 )
                goto LABEL_12;
            }
LABEL_25:
            sub_16300();
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
        v14 = sub_15F50(8 * v24 + 8);
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
  v18 = sub_15F50(8);
  *a2 = 0;
  a2[1] = v18;
  v15 = (const char **)v18;
LABEL_23:
  *v15 = 0;
  return 1;
}



// Function: diff_dirs @ 0xa0c0
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
  if ( (v2 == -1 || (unsigned char)sub_9CD0(a1, 0))
    && (*(int *)(a1 + 304) == -1 || (unsigned char)sub_9CD0(a1, 1)) )
  {
    v3 = *(long long *)(a1 + 304LL * (v2 == -1) + 8);
    v4 = dcgettext(0, "%s: recursive directory loop", 5);
    error(0, 0, v4, v3);
    return 2;
  }
  if ( !(unsigned char)sub_9E90(a1, &nmemb) )
  {
    sub_DF30(*(long long *)(a1 + 8), &nmemb, v6, v7);
    v26 = 2;
  }
  if ( !(unsigned char)sub_9E90(a1 + 304, &v32) )
  {
    sub_DF30(*(long long *)(a1 + 312), &v32, v8, v9);
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
    v10 = (char *)qword_26488;
    if ( qword_26488 && !*(long long *)(a1 + 608) )
    {
      while ( *(long long *)base && sub_9E40(*(char **)base, v10) < 0 )
        base = (char *)base + 8;
      while ( *(long long *)v28 && sub_9E40(*(char **)v28, v10) < 0 )
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
        v11 = sub_9E40(*(char **)base, *(char **)v28);
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
                while ( !sub_9E40(v21, v22) )
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



// Function: find_dir_file_pathname @ 0xa4f0
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
    if ( (unsigned char)sub_9E90((long long)&v11, &v8) )
    {
      byte_266E8 = 1;
      if ( _setjmp(env) )
      {
        v2 = sub_10920(a1, a2, 0);
        goto LABEL_3;
      }
      v5 = (char *)ptr;
      v6 = *(char **)ptr;
      if ( *(long long *)ptr )
      {
        do
        {
          if ( !sub_9E40(v6, a2) )
          {
            if ( !strcmp(v6, a2) )
            {
              v2 = sub_10920(a1, v6, 0);
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
  v2 = sub_10920(a1, v7, 0);
LABEL_3:
  v3 = v2;
  free(ptr);
  free(v10);
  return v3;
}



// Function: print_ed_hunk @ 0xa650
unsigned long long print_ed_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  int v5; // ebx
  long long v6; // rbx
  long long v7; // rsi
  long long v8; // [rsp+8h] [rbp-50h] BYREF
  long long v9; // [rsp+10h] [rbp-48h] BYREF
  long long v10; // [rsp+18h] [rbp-40h] BYREF
  long long v11; // [rsp+20h] [rbp-38h] BYREF
  unsigned long long v12; // [rsp+28h] [rbp-30h]

  v12 = __readfsqword(0x28u);
  v1 = sub_F1A0(a1, &v8, &v9, &v10, &v11);
  if ( v1 )
  {
    v5 = v1;
    sub_E2B0(a1, &v8, v2, v3);
    sub_F0F0(44, &unk_261A0, v8, v9);
    fputc_unlocked(byte_1D550[v5], stream);
    fputc_unlocked(10, stream);
    if ( v5 != 1 )
    {
      v6 = v10;
      if ( v11 < v10 )
      {
LABEL_8:
        fputs_unlocked(".\n", stream);
      }
      else
      {
        while ( 1 )
        {
          while ( 1 )
          {
            v7 = qword_26388 + 8 * v6++;
            if ( **(char **)v7 == 46 && *(char *)(*(long long *)v7 + 1LL) == 10 )
              break;
            sub_F0A0("", v7);
            if ( v11 < v6 )
              goto LABEL_8;
          }
          fputs_unlocked("..\n.\ns/.//\n", stream);
          if ( v11 < v6 )
            break;
          fputs_unlocked("a\n", stream);
        }
      }
    }
  }
  return v12 - __readfsqword(0x28u);
}



// Function: pr_forward_ed_hunk @ 0xa7a0
unsigned long long pr_forward_ed_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  int v5; // ebx
  long long i; // rbx
  long long v7; // rsi
  long long v8; // [rsp+8h] [rbp-40h] BYREF
  long long v9; // [rsp+10h] [rbp-38h] BYREF
  long long v10; // [rsp+18h] [rbp-30h] BYREF
  long long v11; // [rsp+20h] [rbp-28h] BYREF
  unsigned long long v12; // [rsp+28h] [rbp-20h]

  v12 = __readfsqword(0x28u);
  v1 = sub_F1A0(a1, &v8, &v9, &v10, &v11);
  if ( v1 )
  {
    v5 = v1;
    sub_E2B0(a1, &v8, v2, v3);
    fputc_unlocked(byte_1D550[v5], stream);
    sub_F0F0(32, &unk_261A0, v8, v9);
    fputc_unlocked(10, stream);
    if ( v5 != 1 )
    {
      for ( i = v10; v11 >= i; ++i )
      {
        v7 = qword_26388 + 8 * i;
        sub_F0A0("", v7);
      }
      fputs_unlocked(".\n", stream);
    }
  }
  return v12 - __readfsqword(0x28u);
}



// Function: print_rcs_hunk @ 0xa8a0
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
  v1 = sub_F1A0(a1, &v10, &v11, &v12, &v13);
  if ( v1 )
  {
    v5 = v1;
    sub_E2B0(a1, &v10, v2, v3);
    sub_F0C0(&unk_261A0, v10, v11, &v14, &v15);
    if ( (v5 & 1) != 0 )
    {
      v6 = 1;
      if ( v14 <= v15 )
        v6 = v15 - v14 + 1;
      __fprintf_chk(stream, 1, "d%ld %ld\n", v14, v6);
    }
    if ( (v5 & 2) != 0 )
    {
      sub_F0C0(&unk_262D0, v12, v13, &v16, &v17);
      v7 = 1;
      if ( v16 <= v17 )
        v7 = v17 - v16 + 1;
      __fprintf_chk(stream, 1, "a%ld %ld\n", v15, v7);
      for ( i = v12; v13 >= i; ++i )
      {
        v9 = qword_26388 + 8 * i;
        sub_F0A0("", v9);
      }
    }
  }
  return v18 - __readfsqword(0x28u);
}



// Function: print_ed_script @ 0xaa10
long long print_ed_script(long long a1)
{
  return sub_EB10(a1, sub_EB00, sub_A650);
}



// Function: pr_forward_ed_script @ 0xaa30
long long pr_forward_ed_script(long long a1)
{
  return sub_EB10(a1, sub_EAF0, sub_A7A0);
}



// Function: print_rcs_script @ 0xaa50
long long print_rcs_script(long long a1)
{
  return sub_EB10(a1, sub_EAF0, sub_A8A0);
}



// Function: scan_char_literal @ 0xaa70
char *scan_char_literal(char *a1, char *a2)
{
  char v2; // dl
  char *v3; // r8
  int v5; // eax
  char v6; // dl
  unsigned int v7; // eax

  v2 = *a1;
  if ( *a1 != 39 )
  {
    if ( v2 == 92 )
    {
      v5 = a1[1];
      v3 = a1 + 2;
      v6 = 0;
      if ( a1[1] == 39 )
      {
LABEL_12:
        if ( (unsigned long long)(v3 - a1 - 3) <= 2 )
        {
          *a2 = v6;
          return v3;
        }
      }
      else
      {
        while ( 1 )
        {
          v7 = v5 - 48;
          if ( v7 > 7 )
            break;
          ++v3;
          v6 = v7 + 8 * v6;
          v5 = (char)*(v3 - 1);
          if ( *(v3 - 1) == 39 )
            goto LABEL_12;
        }
      }
    }
    else if ( v2 )
    {
      v3 = 0;
      if ( a1[1] == 39 )
      {
        *a2 = v2;
        return a1 + 2;
      }
      return v3;
    }
  }
  return 0;
}



// Function: groups_letter_value @ 0xaaf0
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
      result = sub_F0B0(*v2, v2[1]) - 1;
      break;
    case 'F':
      v2 = a1 + 3;
LABEL_6:
      result = sub_F0B0(*v2, v2[1]);
      break;
    case 'L':
      v2 = a1 + 3;
LABEL_8:
      result = sub_F0B0(*v2, v2[2]) - 1;
      break;
    case 'M':
      v2 = a1 + 3;
LABEL_10:
      result = sub_F0B0(*v2, v2[2]);
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



// Function: do_printf_spec @ 0xabe0
char *do_printf_spec(FILE *stream, char *src, long long a3, long long a4, long long *a5)
{
  unsigned long long v7; // rdx
  char *v10; // rax
  unsigned long long v11; // rbx
  char v12; // dl
  char *v13; // r13
  long long v14; // rcx
  int v16; // edx
  int v17; // edx
  long long v18; // rcx
  signed long long v19; // rax
  size_t v20; // rdx
  long long v21; // rsi
  long long *v22; // r8
  signed long long v23; // rdi
  void *v24; // rsp
  char *v25; // r15
  char *v26; // r8
  long long v27; // rcx
  long long v28; // rax
  long long v29; // [rsp+0h] [rbp-1068h]
  long long v30; // [rsp+8h] [rbp-1060h] BYREF
  char v31; // [rsp+2Fh] [rbp-1039h] BYREF
  long long v32; // [rsp+1008h] [rbp-60h] BYREF
  long long v33; // [rsp+1010h] [rbp-58h]
  char *v34; // [rsp+1018h] [rbp-50h]
  size_t v35; // [rsp+1020h] [rbp-48h]
  char v36; // [rsp+102Fh] [rbp-39h] BYREF
  unsigned long long v37; // [rsp+1030h] [rbp-38h]

  v7 = 0xFFFEDF7FFFFFFFFFLL;
  v37 = __readfsqword(0x28u);
  v10 = src + 1;
  do
  {
    v11 = (unsigned char)*v10++;
    if ( (unsigned char)v11 > 0x30u )
    {
      if ( (unsigned int)((char)v11 - 48) > 9 )
        goto LABEL_6;
      goto LABEL_12;
    }
  }
  while ( !_bittest64((const long long *)&v7, v11) );
  if ( (unsigned int)((char)v11 - 48) > 9 )
  {
    if ( (char)v11 != 46 )
      goto LABEL_6;
    goto LABEL_14;
  }
  do
  {
LABEL_12:
    v16 = *v10++;
    LODWORD(v11) = v16;
  }
  while ( (unsigned int)(v16 - 48) <= 9 );
  if ( (char)v16 == 46 )
  {
    do
    {
LABEL_14:
      v17 = *v10++;
      LODWORD(v11) = v17;
    }
    while ( (unsigned int)(v17 - 48) <= 9 );
  }
LABEL_6:
  v12 = *v10;
  v13 = v10 + 1;
  if ( (unsigned char)(v11 - 88) > 0x20u )
    return 0;
  v14 = 0x100801001LL;
  if ( !_bittest64(&v14, (unsigned int)(v11 - 88)) )
  {
    if ( (char)v11 == 99 && v12 == 39 )
    {
      v13 = sub_AA70(v13, &v36);
      if ( v13 && stream )
        putc_unlocked(v36, stream);
      return v13;
    }
    return 0;
  }
  if ( a3 )
  {
    if ( v12 != 110 )
      return 0;
    v18 = sub_F0B0(a3, a4);
  }
  else
  {
    v18 = sub_AAF0(a5, v12);
    if ( v18 < 0 )
      return 0;
  }
  if ( stream )
  {
    v19 = v13 - src;
    v20 = v13 - src - 2;
    v21 = v13 - src - 1;
    if ( (unsigned long long)(v13 - src + 1) > 0xFA0 )
    {
      v34 = (char *)v18;
      v28 = sub_16340();
      v20 = v13 - src - 2;
      v18 = (long long)v34;
      v21 = v13 - src - 1;
      v25 = (char *)v28;
    }
    else
    {
      v22 = (long long *)((char *)&v32 - ((v19 + 55) & 0xFFFFFFFFFFFFF000LL));
      if ( &v32 != v22 )
      {
        while ( &v30 != v22 )
          ;
      }
      v23 = ((short)v19 + 55) & 0xFF0;
      v24 = alloca(v23);
      if ( (((short)v19 + 55) & 0xFF0) != 0 )
        *(long long *)((char *)&v29 + v23) = *(long long *)((char *)&v29 + v23);
      v25 = (char *)((unsigned long long)&v31 & 0xFFFFFFFFFFFFFFE0LL);
    }
    v33 = v18;
    v34 = &v25[v21];
    v35 = v20;
    memcpy(v25, src, v20);
    v26 = v34;
    v27 = v33;
    v25[v35] = 108;
    *v26 = v11;
    v26[1] = 0;
    __fprintf_chk(stream, 1, v25, v27);
    sub_11CE0(v25);
  }
  return v13;
}



// Function: print_ifdef_lines @ 0xae40
int print_ifdef_lines(FILE *stream, char *a2, long long *a3)
{
  long long v3; // rbx
  long long v4; // rbp
  char *v5; // rax
  char *v7; // r15
  char *v8; // r14
  int v9; // edi
  char v10; // al
  char *v11; // r13
  const void *v12; // rdi
  size_t v13; // rdx
  long long v15; // [rsp+8h] [rbp-50h]
  long long v16; // [rsp+10h] [rbp-48h]

  v3 = a3[1];
  v16 = *a3;
  v4 = *(long long *)(*a3 + 184);
  v5 = (char *)a3[2];
  v15 = (long long)v5;
  if ( stream )
  {
    if ( !byte_264A0 )
    {
      LODWORD(v5) = (int)a2;
      if ( *a2 == 37 )
      {
        LODWORD(v5) = (unsigned char)a2[1];
        if ( (char)v5 == 108 )
        {
          LODWORD(v5) = (int)a2;
          if ( a2[2] == 10 && !a2[3] )
          {
            if ( v3 >= v15 )
              return (int)v5;
            v12 = *(const void **)(v4 + 8 * v3);
            v13 = (*(char *)(*(long long *)(v4 + 8 * v15) - 1LL) != 10) + *(long long *)(v4 + 8 * v15) - (long long)v12;
            goto LABEL_26;
          }
        }
        else if ( (char)v5 == 76 )
        {
          LODWORD(v5) = (int)a2;
          if ( !a2[2] )
          {
            v12 = *(const void **)(v4 + 8 * v3);
            v13 = *(long long *)(v4 + 8 * v15) - (long long)v12;
LABEL_26:
            LODWORD(v5) = fwrite_unlocked(v12, 1u, v13, stream);
            return (int)v5;
          }
        }
      }
    }
    if ( v3 >= v15 )
      return (int)v5;
    LODWORD(v5) = (int)a2 + 1;
    while ( 1 )
    {
      v7 = a2;
      v8 = a2 + 1;
      v9 = *a2;
      if ( *a2 )
        break;
LABEL_18:
      if ( v15 == ++v3 )
        return (int)v5;
    }
    while ( (char)v9 == 37 )
    {
      v10 = v7[1];
      v11 = v7 + 2;
      if ( v10 == 76 || v10 == 108 )
      {
        v7 += 2;
        v8 = v11 + 1;
        LODWORD(v5) = sub_ED50(*(void **)(v4 + 8 * v3));
        v9 = (char)*v11;
        if ( !*v11 )
          goto LABEL_18;
      }
      else
      {
        if ( v10 == 37 )
        {
          v7 += 2;
          v9 = 37;
        }
        else
        {
          v5 = sub_ABE0(stream, v7, v16, v3, 0);
          v7 = v5;
          if ( v5 )
            goto LABEL_9;
          v7 = v8;
          v9 = 37;
        }
LABEL_8:
        LODWORD(v5) = putc_unlocked(v9, stream);
LABEL_9:
        v9 = *v7;
        v8 = v7 + 1;
        if ( !*v7 )
          goto LABEL_18;
      }
    }
    v7 = v8;
    goto LABEL_8;
  }
  return (int)v5;
}



// Function: format_group @ 0xb040
char *format_group(FILE *a1, char *a2, char a3, long long *a4)
{
  char *result; // rax
  char v6; // bl
  char *v9; // r14
  char *v10; // r8
  long long *v11; // rdx
  char *v12; // rsi
  long long v13; // rcx
  int v14; // esi
  int *v15; // rax
  intmax_t v16; // rax
  long long v17; // rcx
  char *v18; // rax
  FILE *v19; // rdi
  FILE *v20; // r14
  long long v21; // rax
  long long *v22; // [rsp+8h] [rbp-90h]
  char *v23; // [rsp+10h] [rbp-88h]
  char *v24; // [rsp+18h] [rbp-80h]
  int *v25; // [rsp+18h] [rbp-80h]
  long long v26; // [rsp+18h] [rbp-80h]
  char *nptra; // [rsp+20h] [rbp-78h]
  char *nptr; // [rsp+20h] [rbp-78h]
  long long v29; // [rsp+28h] [rbp-70h]
  char *endptr; // [rsp+38h] [rbp-60h] BYREF
  long long v31[11]; // [rsp+40h] [rbp-58h]

  result = a2;
  v6 = *a2;
  v31[3] = __readfsqword(0x28u);
  if ( v6 != a3 && v6 )
  {
    v22 = a4 + 3;
    do
    {
      v9 = result + 1;
      if ( v6 == 37 )
      {
        v10 = result + 2;
        switch ( result[1] )
        {
          case '%':
            v9 = result + 2;
            v6 = result[1];
            goto LABEL_5;
          case '(':
            v13 = 0;
            break;
          case '<':
            v24 = result;
            v12 = (char *)qword_26448;
            v11 = a4;
            v23 = result + 2;
            goto LABEL_15;
          case '=':
            v24 = result;
            v12 = (char *)qword_26440[0];
            v11 = a4;
            v23 = result + 2;
            goto LABEL_15;
          case '>':
            v24 = result;
            v11 = v22;
            v23 = result + 2;
            v12 = (char *)qword_26450;
LABEL_15:
            sub_AE40(a1, v12, v11);
            v6 = v24[2];
            result = v23;
            continue;
          default:
            result = sub_ABE0(a1, result, 0, 0, a4);
            if ( !result )
              goto LABEL_5;
            v6 = *result;
            continue;
        }
        while ( 1 )
        {
          v14 = *v10;
          if ( (unsigned int)(v14 - 48) > 9 )
          {
            v26 = v13;
            nptr = v10;
            v21 = sub_AAF0(a4, v14);
            v17 = v26;
            v31[v26] = v21;
            if ( v21 < 0 )
              goto LABEL_5;
            v18 = nptr + 1;
          }
          else
          {
            v29 = v13;
            nptra = v10;
            v15 = __errno_location();
            *v15 = 0;
            v25 = v15;
            v16 = strtoimax(nptra, &endptr, 10);
            v17 = v29;
            v31[v29] = v16;
            if ( *v25 )
              goto LABEL_5;
            v18 = endptr;
          }
          v10 = v18 + 1;
          if ( *v18 != byte_1D194[v17] )
            goto LABEL_5;
          if ( v17 == 1 )
            break;
          v13 = 1;
        }
        v19 = 0;
        v20 = 0;
        if ( v31[0] == v31[1] )
          v19 = a1;
        else
          v20 = a1;
        result = (char *)format_group(v19, v10, 58, a4);
        if ( !*result )
          return result;
        result = (char *)format_group(v20, result + 1, 41, a4);
        if ( !*result )
          return result;
        v6 = *++result;
      }
      else
      {
LABEL_5:
        if ( a1 )
          putc_unlocked(v6, a1);
        v6 = *v9;
        result = v9;
      }
    }
    while ( v6 && a3 != v6 );
  }
  return result;
}



// Function: format_ifdef @ 0xb2e0
unsigned long long format_ifdef(char *a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v6[7]; // [rsp+0h] [rbp-48h] BYREF
  unsigned long long v7; // [rsp+38h] [rbp-10h]

  v7 = __readfsqword(0x28u);
  v6[1] = a2;
  v6[2] = a3;
  v6[4] = a4;
  v6[5] = a5;
  v6[0] = (long long)&unk_261A0;
  v6[3] = (long long)&unk_262D0;
  sub_B040(stream, a1, 0, v6);
  return v7 - __readfsqword(0x28u);
}



// Function: print_ifdef_hunk @ 0xb350
unsigned long long print_ifdef_hunk(long long a1)
{
  unsigned int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  long long v4; // rbx
  long long v6; // rsi
  long long v7; // rcx
  char *v8; // rdi
  long long v9; // [rsp+8h] [rbp-30h] BYREF
  long long v10; // [rsp+10h] [rbp-28h] BYREF
  long long v11; // [rsp+18h] [rbp-20h] BYREF
  long long v12; // [rsp+20h] [rbp-18h] BYREF
  unsigned long long v13; // [rsp+28h] [rbp-10h]

  v13 = __readfsqword(0x28u);
  v1 = sub_F1A0(a1, &v9, &v10, &v11, &v12);
  v4 = v1;
  if ( v1 )
  {
    sub_E2B0(a1, &v9, v2, v3);
    v6 = v9;
    v7 = v11;
    if ( qword_266F8 < v9 || qword_266F0 < v11 )
    {
      sub_B2E0(s1, qword_266F8, v9, qword_266F0, v11);
      v7 = v11;
      v6 = v9;
    }
    qword_266F8 = v10 + 1;
    v8 = (&s1)[v4];
    qword_266F0 = v12 + 1;
    sub_B2E0(v8, v6, v10 + 1, v7, v12 + 1);
  }
  return v13 - __readfsqword(0x28u);
}



// Function: print_ifdef_script @ 0xb430
unsigned long long print_ifdef_script(long long a1)
{
  long long v1; // rdx
  long long v2; // rcx
  unsigned long long result; // rax

  qword_266F0 = -qword_26288;
  qword_266F8 = -qword_26288;
  sub_EB10(a1, sub_EAF0, sub_B350);
  if ( qword_26270 > qword_266F8 || (result = qword_266F0, qword_263A0 > qword_266F0) )
  {
    sub_E2B0(a1, sub_EAF0, v1, v2);
    return sub_B2E0(s1, qword_266F8, qword_26270, qword_266F0, qword_263A0);
  }
  return result;
}



// Function: guess_lines @ 0xb4c0
unsigned long long guess_lines(long long a1, unsigned long long a2, unsigned long long a3)
{
  unsigned long long v4; // rsi
  unsigned long long v5; // rax

  v4 = 32;
  if ( a1 > 9 )
    v4 = a2 / (a1 - 1);
  v5 = a3 / v4;
  if ( !(a3 / v4) )
    v5 = 1;
  if ( v5 > 0x787878787878782LL )
    v5 = 0x787878787878782LL;
  return v5 + 5;
}



// Function: prepare_text @ 0xb510
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



// Function: find_and_hash_each_line @ 0xb600
long long find_and_hash_each_line(long long a1)
{
  long long v1; // r15
  unsigned char *v2; // rbx
  void *v3; // rbp
  char *v4; // r15
  unsigned char v5; // dl
  unsigned char *v6; // r14
  long long v7; // rbp
  unsigned long long v8; // r12
  unsigned long long v9; // rsi
  long long v10; // r13
  unsigned char *v11; // r15
  const unsigned short **v12; // rax
  const unsigned short *v13; // r8
  long long v14; // rbx
  unsigned long long v15; // r13
  unsigned long long v16; // r14
  char *v17; // r13
  long long v18; // r15
  void **v19; // r12
  const void *v20; // rdi
  long long v21; // rbp
  long long v22; // rdx
  long long v23; // rcx
  char *v24; // rax
  unsigned long long v25; // r12
  const unsigned short *v26; // rbx
  unsigned long long v27; // rcx
  unsigned char *v28; // rcx
  long long v29; // rax
  long long v30; // rax
  const unsigned short *v31; // rbx
  unsigned char *v32; // r15
  long long v33; // rax
  long long v34; // rcx
  bool v35; // cf
  const int_t *v36; // rdx
  long long v37; // rax
  long long v38; // r13
  long long v39; // rcx
  long long v40; // rsi
  long long v41; // rbp
  unsigned char *v42; // r11
  unsigned char **v43; // rax
  unsigned long long v44; // r9
  long long v45; // rax
  void *v46; // rbp
  unsigned int v48; // [rsp+8h] [rbp-D0h]
  long long v49; // [rsp+8h] [rbp-D0h]
  const void *v50; // [rsp+10h] [rbp-C8h]
  unsigned char *v51; // [rsp+10h] [rbp-C8h]
  char *v52; // [rsp+10h] [rbp-C8h]
  unsigned char *v53; // [rsp+10h] [rbp-C8h]
  long long v54; // [rsp+18h] [rbp-C0h]
  long long v55; // [rsp+18h] [rbp-C0h]
  unsigned long long v56; // [rsp+20h] [rbp-B8h]
  void **v57; // [rsp+20h] [rbp-B8h]
  unsigned char *v58; // [rsp+20h] [rbp-B8h]
  void *v59; // [rsp+20h] [rbp-B8h]
  size_t n; // [rsp+28h] [rbp-B0h]
  long long na; // [rsp+28h] [rbp-B0h]
  long long v62; // [rsp+30h] [rbp-A8h]
  unsigned char *s2; // [rsp+38h] [rbp-A0h]
  void *s2a; // [rsp+38h] [rbp-A0h]
  unsigned char *v65; // [rsp+40h] [rbp-98h]
  unsigned long long v66; // [rsp+48h] [rbp-90h]
  long long v67; // [rsp+50h] [rbp-88h]
  long long v68; // [rsp+50h] [rbp-88h]
  long long v69; // [rsp+58h] [rbp-80h]
  long long v70; // [rsp+60h] [rbp-78h]
  long long v71; // [rsp+60h] [rbp-78h]
  unsigned char *v72; // [rsp+68h] [rbp-70h]
  unsigned long long v73; // [rsp+70h] [rbp-68h]
  int v74; // [rsp+78h] [rbp-60h]
  char v75; // [rsp+7Eh] [rbp-5Ah]
  char v76; // [rsp+7Fh] [rbp-59h]
  long long v77; // [rsp+80h] [rbp-58h]
  long long v78; // [rsp+88h] [rbp-50h]
  long long v79; // [rsp+90h] [rbp-48h]

  v1 = *(long long *)(a1 + 192);
  v2 = *(unsigned char **)(a1 + 224);
  v67 = *(long long *)(a1 + 184);
  v79 = v1;
  v70 = *(long long *)(a1 + 216);
  v69 = sub_15F50(8 * v70);
  v76 = byte_26552;
  v62 = qword_26708;
  v73 = *(long long *)(a1 + 240);
  v3 = ptr;
  v66 = qword_26700;
  v48 = dword_26558;
  v72 = (unsigned char *)(*(long long *)(a1 + 160) + *(long long *)(a1 + 176));
  v75 = byte_26552 | (dword_26558 != 0);
  v77 = 8 * v1;
  v78 = -8 * v1;
  if ( (unsigned long long)v2 < v73 )
  {
    v54 = 0;
    v4 = (char *)ptr;
    v74 = dword_26558 & 1;
    while ( 1 )
    {
      v5 = *v2;
      v6 = v2 + 1;
      v7 = *v2;
      if ( v48 == 4 )
        break;
      if ( v48 > 4 )
      {
        if ( v48 == 5 )
        {
          if ( v5 != 10 )
          {
            v51 = v2;
            v15 = 0;
            v26 = *__ctype_b_loc();
            do
            {
              if ( (v26[v7] & 0x2000) == 0 )
              {
                if ( v76 )
                  v7 = (*__ctype_tolower_loc())[v7];
                v15 = v7 + __ROL8__(v15, 7);
              }
              v7 = *v6++;
            }
            while ( (char)v7 != 10 );
            v2 = v51;
            na = v6 - v51 - 1;
            goto LABEL_20;
          }
        }
        else
        {
LABEL_64:
          if ( !v76 )
          {
            v15 = 0;
            if ( v5 == 10 )
            {
              na = 0;
            }
            else
            {
              do
              {
                ++v6;
                v15 = v7 + __ROL8__(v15, 7);
                v7 = *(v6 - 1);
              }
              while ( (char)v7 != 10 );
              na = v6 - v2 - 1;
            }
            goto LABEL_20;
          }
          if ( v5 != 10 )
          {
            v15 = 0;
            v36 = *__ctype_tolower_loc();
            do
            {
              ++v6;
              v37 = v36[v7];
              v7 = *(v6 - 1);
              v15 = v37 + __ROL8__(v15, 7);
            }
            while ( (char)v7 != 10 );
            na = v6 - v2 - 1;
            goto LABEL_20;
          }
        }
LABEL_86:
        na = 0;
        v15 = 0;
        goto LABEL_20;
      }
      if ( v48 - 1 > 2 )
        goto LABEL_64;
      if ( v5 == 10 )
        goto LABEL_86;
      v8 = 0;
      v9 = 0;
      s2 = v2;
      v10 = v5;
      v56 = qword_26498;
      n = (size_t)v4;
      v11 = v2 + 1;
      while ( 1 )
      {
        if ( (v48 & 2) != 0 )
        {
          v12 = __ctype_b_loc();
          v13 = *v12;
          if ( ((*v12)[(unsigned char)v10] & 0x2000) != 0 )
            break;
        }
LABEL_10:
        if ( !v74 )
          goto LABEL_42;
        switch ( (char)v10 )
        {
          case 9:
            v10 = 32;
            v34 = v56 - v8 % v56;
            v35 = __CFADD__(v34, v8);
            v8 += v34;
            v14 = v34;
            if ( v35 )
              v8 = 0;
            goto LABEL_15;
          case 0xD:
            v10 = 13;
            v14 = 1;
            v8 = 0;
            goto LABEL_15;
          case 8:
            v10 = 8;
            v8 = (v8 == 0) + v8 - 1;
LABEL_42:
            v14 = 1;
            goto LABEL_15;
        }
        ++v8;
        v14 = 1;
LABEL_15:
        if ( v76 )
          v10 = LOBYTE((*__ctype_tolower_loc())[v10]);
        do
        {
          v9 = v10 + __ROL8__(v9, 7);
          --v14;
        }
        while ( v14 );
        v10 = *v11++;
        if ( (char)v10 == 10 )
        {
          v6 = v11;
          v2 = s2;
          v4 = (char *)n;
          v15 = v9;
          na = v6 - s2 - 1;
          goto LABEL_20;
        }
      }
      v28 = v11;
      while ( 1 )
      {
        v29 = *v28++;
        if ( (char)v29 == 10 )
          break;
        if ( (v13[v29] & 0x2000) == 0 )
          goto LABEL_10;
      }
      v2 = s2;
      v4 = (char *)n;
      v15 = v9;
      v6 = v28;
      na = v28 - s2 - 1;
LABEL_20:
      v57 = (void **)(qword_26720 + 8 * (v15 % qword_26718));
      if ( v6 == v72 && *(char *)(a1 + 288) && (unsigned int)(dword_26580 - 4) > 1 )
      {
        v27 = qword_26720 - 8;
        if ( v48 > 1 )
          v27 = qword_26720 + 8 * (v15 % qword_26718);
        v57 = (void **)v27;
      }
      s2a = *v57;
      if ( *v57 )
      {
        v65 = v6;
        v16 = v15;
        v17 = v4;
        v18 = (long long)*v57;
        while ( 1 )
        {
LABEL_24:
          v19 = (void **)&v17[32 * v18];
          if ( v19[1] != (void *)v16 )
            goto LABEL_23;
          v20 = v19[2];
          if ( v19[3] != (void *)na )
            break;
          v50 = v19[2];
          if ( !memcmp(v20, v2, na) )
          {
LABEL_28:
            v21 = v18;
            v6 = v65;
            v4 = v17;
            if ( v54 != v70 )
              goto LABEL_29;
LABEL_37:
            if ( v54 <= 0x2AAAAAAAAAAAAAA9LL )
            {
              v25 = 2 * v54 - v79;
              if ( v25 <= 0xFFFFFFFFFFFFFFELL && (unsigned long long)(v54 - v79) <= 0xFFFFFFFFFFFFFFELL )
              {
                v69 = sub_15FA0(v69, 8 * v25);
                v70 = 2 * v54 - v79;
                v67 = v78 + sub_15FA0(v77 + v67, 8 * (v25 - v79));
                goto LABEL_29;
              }
            }
LABEL_107:
            sub_16300();
          }
          v20 = v50;
          if ( v75 )
            goto LABEL_27;
          v18 = (long long)*v19;
          if ( !*v19 )
          {
LABEL_34:
            v4 = v17;
            v15 = v16;
            v6 = v65;
            goto LABEL_35;
          }
        }
        if ( !v48 )
          goto LABEL_23;
LABEL_27:
        if ( !(unsigned char)sub_E690(v20, v2) )
          goto LABEL_28;
LABEL_23:
        v18 = (long long)*v19;
        if ( !*v19 )
          goto LABEL_34;
        goto LABEL_24;
      }
LABEL_35:
      v22 = v62 + 1;
      if ( v66 == v62 )
      {
        if ( v66 > 0x1FFFFFFFFFFFFFELL )
          goto LABEL_107;
        v66 *= 2LL;
        v30 = sub_15FA0(v4, 32 * v66);
        v22 = v62 + 1;
        v4 = (char *)v30;
        s2a = *v57;
      }
      v23 = v62;
      v62 = v22;
      v21 = v23;
      v24 = &v4[32 * v23];
      *(long long *)v24 = s2a;
      *((long long *)v24 + 1) = v15;
      *((long long *)v24 + 2) = v2;
      *((long long *)v24 + 3) = na;
      *v57 = (void *)v23;
      if ( v54 == v70 )
        goto LABEL_37;
LABEL_29:
      *(long long *)(v67 + 8 * v54) = v2;
      *(long long *)(v69 + 8 * v54++) = v21;
      if ( (unsigned long long)v6 >= v73 )
      {
        v3 = v4;
        goto LABEL_88;
      }
      v2 = v6;
    }
    if ( v5 != 10 )
    {
      v15 = 0;
      v58 = v2;
      v52 = v4;
      v31 = *__ctype_b_loc();
      while ( 1 )
      {
        v32 = v6;
        if ( (v31[v7] & 0x2000) != 0 )
        {
          v33 = *v6++;
          if ( (char)v33 == 10 )
            goto LABEL_77;
          while ( (v31[v33] & 0x2000) != 0 )
          {
            v33 = *v6++;
            if ( (char)v33 == 10 )
              goto LABEL_77;
          }
          v7 = v33;
          v32 = v6;
          v15 = __ROL8__(v15, 7) + 32LL;
        }
        if ( v76 )
          v7 = (*__ctype_tolower_loc())[v7];
        ++v6;
        v15 = v7 + __ROL8__(v15, 7);
        v7 = *v32;
        if ( (char)v7 == 10 )
        {
LABEL_77:
          v2 = v58;
          v4 = v52;
          na = v6 - v58 - 1;
          goto LABEL_20;
        }
      }
    }
    goto LABEL_86;
  }
  v54 = 0;
  v6 = v2;
LABEL_88:
  v59 = v3;
  v38 = 0;
  v39 = v70;
  v40 = v67;
  *(long long *)(a1 + 200) = v54;
  v41 = v54;
  v42 = v72;
  while ( 1 )
  {
    if ( v39 != v41 )
    {
      v43 = (unsigned char **)(v40 + 8 * v41);
      *v43 = v6;
      if ( v6 == v42 )
        break;
      goto LABEL_90;
    }
    v53 = v42;
    if ( v39 > 0x2AAAAAAAAAAAAAA9LL )
      goto LABEL_107;
    v44 = 2 * v39 - v79;
    if ( v44 > 0xFFFFFFFFFFFFFFELL || (unsigned long long)(v39 - v79) > 0xFFFFFFFFFFFFFFELL )
      goto LABEL_107;
    v49 = 2 * v39 - v79;
    v45 = sub_15FA0(v40 + v77, 8 * (v44 - v79));
    v42 = v53;
    v39 = v49;
    v40 = v45 + v78;
    v43 = (unsigned char **)(v45 + v78 + 8 * v41);
    *v43 = v6;
    if ( v6 == v53 )
      break;
LABEL_90:
    if ( v38 >= qword_26570 && byte_26578 )
    {
      v55 = v41;
      v46 = v59;
      v71 = v39;
      v68 = v40;
      goto LABEL_103;
    }
    ++v41;
    do
      ++v6;
    while ( *(v6 - 1) != 10 );
    ++v38;
  }
  v55 = v41;
  v71 = v39;
  v46 = v59;
  v68 = v40;
  if ( *(char *)(a1 + 288) && (unsigned int)(dword_26580 - 4) > 1 )
    *v43 = v6 - 1;
LABEL_103:
  ptr = v46;
  *(long long *)(a1 + 184) = v68;
  *(long long *)(a1 + 208) = v55;
  *(long long *)(a1 + 216) = v71;
  *(long long *)(a1 + 248) = v69;
  qword_26700 = v66;
  qword_26708 = v62;
  return v62;
}



// Function: file_block_read @ 0xbf50
void file_block_read(long long a1, unsigned long long a2)
{
  unsigned long long v2; // rax

  if ( a2 && !*(char *)(a1 + 289) )
  {
    v2 = sub_16520(*(int *)a1, (void *)(*(long long *)(a1 + 160) + *(long long *)(a1 + 176)));
    if ( v2 == -1 )
    {
      sub_E1F0(*(long long *)(a1 + 8));
      sub_BFD0();
    }
    else
    {
      *(long long *)(a1 + 176) += v2;
      *(char *)(a1 + 289) = a2 > v2;
    }
  }
}



// Function: sip @ 0xbfd0
char sip(unsigned int *a1, char a2)
{
  long long v2; // rax
  int v4; // ebp
  size_t v5; // r12

  if ( (*a1 & 0x80000000) != 0 )
  {
    *((long long *)a1 + 21) = 8;
    *((long long *)a1 + 20) = sub_15F50(8);
    goto LABEL_3;
  }
  v2 = sub_165D0(8, *((long long *)a1 + 9), 0x7FFFFFFFFFFFFFEFLL);
  *((long long *)a1 + 21) = v2;
  *((long long *)a1 + 20) = sub_15F50(v2);
  if ( a2 )
  {
LABEL_3:
    *((long long *)a1 + 22) = 0;
    *((char *)a1 + 289) = 0;
    return 0;
  }
  v4 = sub_F710(*a1, 0);
  sub_BF50((long long)a1, *((long long *)a1 + 21));
  v5 = *((long long *)a1 + 22);
  if ( !v4 )
    return memchr(*((const void **)a1 + 20), 0, v5) != 0;
  if ( lseek(*a1, -(long long)v5, 1) >= 0 )
  {
    *((long long *)a1 + 22) = 0;
    *((char *)a1 + 289) = 0;
    return memchr(*((const void **)a1 + 20), 0, v5) != 0;
  }
  sub_E1F0(*((long long *)a1 + 1));
  return sub_C0D0();
}



// Function: slurp @ 0xc0d0
unsigned long long slurp(unsigned int *a1)
{
  unsigned long long result; // rax
  unsigned long long v3; // rbp
  unsigned long long v4; // rsi
  unsigned long long v5; // rsi
  long long v6; // rsi
  long long v7; // rdi
  long long v8; // rax
  unsigned long long v9; // rsi
  long long v10; // rdi
  unsigned long long v11; // rsi

  result = *a1;
  if ( (result & 0x80000000) != 0LL )
    return result;
  if ( (a1[10] & 0xF000) == 0x8000 )
  {
    v3 = *((long long *)a1 + 8);
    v4 = (v3 & 0xFFFFFFFFFFFFFFF8LL) + 16;
    if ( v3 > v4 || v4 > 0x7FFFFFFFFFFFFFFELL )
      goto LABEL_18;
    if ( *((long long *)a1 + 21) >= v4 )
    {
      result = *((long long *)a1 + 22);
      if ( result > v3 )
        goto LABEL_7;
LABEL_16:
      sub_BF50((long long)a1, v3 - result + 1);
      result = *((long long *)a1 + 22);
      if ( result <= v3 )
        return result;
      goto LABEL_7;
    }
    *((long long *)a1 + 21) = v4;
    *((long long *)a1 + 20) = sub_15FA0(*((long long *)a1 + 20), v4);
    result = *((long long *)a1 + 22);
    if ( result <= v3 )
      goto LABEL_16;
  }
  else
  {
    result = *((long long *)a1 + 22);
  }
LABEL_7:
  sub_BF50((long long)a1, *((long long *)a1 + 21) - result);
  result = *((long long *)a1 + 22);
  if ( result )
  {
    v5 = *((long long *)a1 + 21);
    if ( result == v5 )
    {
      while ( v5 <= 0x3FFFFFFFFFFFFFF7LL )
      {
        v6 = 2 * v5;
        v7 = *((long long *)a1 + 20);
        *((long long *)a1 + 21) = v6;
        v8 = sub_15FA0(v7, v6);
        v9 = *((long long *)a1 + 21) - *((long long *)a1 + 22);
        *((long long *)a1 + 20) = v8;
        sub_BF50((long long)a1, v9);
        result = *((long long *)a1 + 22);
        v5 = *((long long *)a1 + 21);
        if ( result != v5 )
          goto LABEL_11;
      }
LABEL_18:
      sub_16300();
    }
LABEL_11:
    v10 = *((long long *)a1 + 20);
    v11 = (result + 16) & 0xFFFFFFFFFFFFFFF8LL;
    *((long long *)a1 + 21) = v11;
    result = sub_15FA0(v10, v11);
    *((long long *)a1 + 20) = result;
  }
  return result;
}



// Function: find_identical_ends @ 0xc270
long long find_identical_ends(long long a1)
{
  unsigned long long v2; // r10
  char *v3; // r14
  char v4; // al
  unsigned long long v5; // rbp
  char *v6; // rax
  char *v7; // r11
  char *v8; // rdx
  unsigned int v9; // ebx
  long long v10; // rsi
  long long i; // rcx
  char *v12; // rbx
  bool v13; // zf
  unsigned long long v14; // rsi
  long long v15; // r11
  signed long long v16; // r13
  long long v17; // rdi
  long long v18; // rax
  long long v19; // r11
  char *v20; // rdx
  long long v21; // rbp
  long long *v22; // rbx
  long long v23; // rcx
  char *v24; // rax
  long long v25; // rax
  long long v26; // r12
  long long v27; // r12
  char *v28; // r13
  char *v29; // rdi
  long long v30; // rdx
  bool v31; // cl
  long long v32; // r12
  long long v33; // rbx
  long long v34; // r10
  signed long long v35; // rax
  long long v36; // r8
  unsigned long long v37; // r12
  long long v38; // rsi
  long long v39; // rax
  long long v40; // r8
  long long v41; // rdi
  long long j; // rax
  long long v43; // rdx
  long long v45; // rbp
  long long v46; // r11
  signed long long v47; // rax
  long long v48; // r10
  long long v49; // r10
  char *v50; // [rsp+8h] [rbp-70h]
  long long v51; // [rsp+10h] [rbp-68h]
  char *v52; // [rsp+10h] [rbp-68h]
  long long v53; // [rsp+10h] [rbp-68h]
  long long v54; // [rsp+18h] [rbp-60h]
  long long v55; // [rsp+20h] [rbp-58h]
  long long v56; // [rsp+20h] [rbp-58h]
  char *v57; // [rsp+28h] [rbp-50h]
  char *v58; // [rsp+30h] [rbp-48h]
  long long v59; // [rsp+38h] [rbp-40h]

  sub_C0D0((unsigned int *)a1);
  sub_B510(a1);
  if ( *(int *)a1 == *(int *)(a1 + 304) )
  {
    v2 = *(long long *)(a1 + 176);
    v3 = *(char **)(a1 + 160);
    *(long long *)(a1 + 472) = *(long long *)(a1 + 168);
    v4 = *(char *)(a1 + 288);
    v5 = v2;
    *(long long *)(a1 + 464) = v3;
    *(long long *)(a1 + 480) = v2;
    *(char *)(a1 + 592) = v4;
LABEL_3:
    v6 = &v3[v5];
    v50 = v3;
    v7 = &v3[v2];
    v57 = &v3[v5];
    v8 = &v3[v5];
    v9 = dword_26580 - 4;
    goto LABEL_4;
  }
  sub_C0D0((unsigned int *)(a1 + 304));
  sub_B510(a1 + 304);
  v3 = *(char **)(a1 + 160);
  v2 = *(long long *)(a1 + 176);
  v5 = *(long long *)(a1 + 480);
  v50 = *(char **)(a1 + 464);
  if ( v3 == v50 )
    goto LABEL_3;
  v7 = &v3[v2];
  v57 = &v50[v5];
  if ( v2 < v5 )
    *v7 = ~v50[v2];
  else
    *v57 = ~v3[v5];
  v8 = v50;
  if ( *(long long *)v3 == *(long long *)v50 )
  {
    v6 = v3;
    do
    {
      v6 += 8;
      v8 += 8;
    }
    while ( *(long long *)v6 == *(long long *)v8 );
  }
  else
  {
    v8 = v50;
    v6 = v3;
  }
  for ( ; *v6 == *v8; ++v8 )
    ++v6;
  v9 = dword_26580 - 4;
  if ( (unsigned int)(dword_26580 - 4) > 1
    && v6 > &v3[v2 - *(unsigned char *)(a1 + 288)] != v8 > &v50[v5 - *(unsigned char *)(a1 + 592)] )
  {
    --v6;
    --v8;
  }
LABEL_4:
  v10 = qword_26560;
  for ( i = qword_26560; v3 != v6; --v8 )
  {
    while ( *(v6 - 1) != 10 )
    {
      --v6;
      --v8;
      if ( v3 == v6 )
        goto LABEL_10;
    }
    if ( !i )
      break;
    --v6;
    --i;
  }
LABEL_10:
  *(long long *)(a1 + 224) = v6;
  *(long long *)(a1 + 528) = v8;
  if ( v9 <= 1 || (v12 = v7, v58 = v57, *(char *)(a1 + 288) == *(char *)(a1 + 592)) )
  {
    v28 = v57;
    v12 = v7;
    v29 = &v6[v2 - v5];
    if ( v2 < v5 )
      v29 = v6;
    while ( 1 )
    {
      if ( v12 == v29 )
      {
        if ( v3 == v12 )
          goto LABEL_52;
        goto LABEL_33;
      }
      if ( *(v12 - 1) != *(v28 - 1) )
        break;
      --v28;
      --v12;
    }
    v29 = v12;
    if ( v3 == v12 )
      goto LABEL_52;
LABEL_33:
    if ( *(v12 - 1) != 10 )
    {
      ++v10;
      goto LABEL_35;
    }
LABEL_52:
    if ( v50 != v28 )
      v10 += *(v28 - 1) != 10;
LABEL_35:
    v30 = v10 - 1;
    if ( v10 && v12 != v7 )
    {
      do
      {
        do
          ++v12;
        while ( *(v12 - 1) != 10 );
        v31 = v30-- != 0;
      }
      while ( v31 && v12 != v7 );
    }
    v58 = &v28[v12 - v29];
  }
  v13 = byte_26578 == 0;
  *(long long *)(a1 + 240) = v12;
  *(long long *)(a1 + 544) = v58;
  if ( v13 || buffer.fastmap || (v45 = qword_26570, qword_26570 > 0x1FFFFFFFFFFFFFFELL) || qword_26570 >= v2 )
  {
    v14 = 0;
    v59 = 0;
    v16 = sub_B4C0(0, 0, v2);
  }
  else
  {
    sub_B4C0(0, 0, v12 - v6);
    v14 = 0;
    v47 = sub_B4C0(0, 0, v46 - (long long)v12);
    v59 = 1;
    if ( v45 > 0 )
    {
      do
      {
        v59 *= 2;
        v14 = v59;
      }
      while ( v45 >= v59 );
      v15 = v59 - 1;
    }
    v49 = v59 + v48;
    if ( v45 > v47 )
      v45 = v47;
    v16 = v49 + v45;
  }
  v17 = 8 * v16;
  v51 = v15;
  v18 = sub_15F50(8 * v16);
  v19 = v51;
  v20 = *(char **)(a1 + 224);
  v21 = v18;
  if ( !byte_26578 || v20 != v12 )
  {
    v22 = v3;
    v23 = 0;
    if ( v3 != v20 )
      goto LABEL_24;
    goto LABEL_18;
  }
  v24 = v58;
  v14 = 0;
  v23 = 0;
  if ( *(char **)(a1 + 528) != v58 )
  {
    v22 = v3;
    v23 = 0;
    if ( v3 == v20 )
    {
LABEL_18:
      v24 = *(char **)(a1 + 528);
      v14 = 0;
      goto LABEL_55;
    }
    do
    {
LABEL_24:
      v26 = v23++;
      v27 = v19 & v26;
      if ( v27 == v16 )
      {
        if ( v16 > 0x7FFFFFFFFFFFFFELL )
          goto LABEL_88;
        v16 *= 2LL;
        v17 = v21;
        v55 = v19;
        v14 = 8 * v16;
        v54 = v23;
        v52 = v20;
        v25 = sub_15FA0(v21, 8 * v16);
        v19 = v55;
        v23 = v54;
        v20 = v52;
        v21 = v25;
      }
      *(long long *)(v21 + 8 * v27) = v22;
      do
        v22 = (long long *)((char *)v22 + 1);
      while ( *((char *)v22 - 1) != 10 );
    }
    while ( v22 != (long long *)v20 );
    v24 = *(char **)(a1 + 528);
    v14 = (char *)v22 - v3;
  }
LABEL_55:
  v32 = qword_26570;
  v53 = v23;
  v33 = qword_26570;
  v56 = v19;
  if ( v23 <= qword_26570 )
    v33 = v23;
  if ( !v59 )
    v33 = v23;
  sub_B4C0(v23, v14, v58 - v24);
  v17 = v53;
  v35 = sub_B4C0(v53, v14, (unsigned long long)&v57[-v34]);
  v20 = (char *)(v33 + v36);
  if ( v35 > v32 )
    v35 = v32;
  v37 = (unsigned long long)&v20[v35];
  if ( v33 > (long long)&v20[v35] || v37 > 0xFFFFFFFFFFFFFFELL )
LABEL_88:
    sub_16300(v17, v14, v20, v23);
  v38 = sub_15F50(8 * v37);
  if ( v53 == v33 )
  {
    if ( v53 <= 0 )
      goto LABEL_71;
  }
  else
  {
    if ( v33 <= 0 )
      goto LABEL_71;
    v39 = v53 - qword_26570;
    v40 = v33 + v53 - qword_26570;
    v41 = v38 + 8 * (qword_26570 - v53);
    do
    {
      *(long long *)(v41 + 8 * v39) = *(long long *)(v21 + 8 * (v39 & v56));
      ++v39;
    }
    while ( v40 != v39 );
    for ( j = 0; j != v33; ++j )
      *(long long *)(v21 + 8 * j) = *(long long *)(v38 + 8 * j);
  }
  v43 = 0;
  do
  {
    *(long long *)(v38 + 8 * v43) = &v50[*(long long *)(v21 + 8 * v43) - (long long)v3];
    ++v43;
  }
  while ( v33 > v43 );
LABEL_71:
  *(long long *)(a1 + 536) = v53;
  *(long long *)(a1 + 520) = v37 - v33;
  *(long long *)(a1 + 488) = v38 + 8 * v33;
  *(long long *)(a1 + 184) = 8 * v33 + v21;
  *(long long *)(a1 + 496) = -v33;
  *(long long *)(a1 + 192) = -v33;
  *(long long *)(a1 + 216) = v16 - v33;
  *(long long *)(a1 + 232) = v53;
  return -v33;
}



// Function: read_files @ 0xc8f0
long long read_files(long long a1, int a2)
{
  char v3; // r13
  long long v5; // rsi
  int v6; // eax
  unsigned int v7; // r12d
  int v9; // eax
  long long v10; // rdx
  long long v11; // rcx
  unsigned long long v12; // rdi
  void *v13; // rax
  size_t v14; // rdi
  long long v15; // rax
  void *v16; // rdi
  unsigned long long v17; // rax

  v3 = a2 | byte_26568;
  v5 = (unsigned char)(a2 | byte_26568);
  LOBYTE(v6) = sub_BFD0((unsigned int *)a1, v5);
  v7 = v6 | a2;
  if ( *(int *)a1 == *(int *)(a1 + 304) )
  {
    *(long long *)(a1 + 464) = *(long long *)(a1 + 160);
    *(long long *)(a1 + 472) = *(long long *)(a1 + 168);
    *(long long *)(a1 + 480) = *(long long *)(a1 + 176);
    if ( (char)v7 )
      return v7;
  }
  else
  {
    v5 = (unsigned char)(v7 | v3);
    LOBYTE(v9) = sub_BFD0((unsigned int *)(a1 + 304), v5);
    v7 |= v9;
    if ( (char)v7 )
      return v7;
  }
  sub_C270(a1);
  v12 = *(long long *)(a1 + 216) + *(long long *)(a1 + 520) + 1LL;
  qword_26700 = v12;
  if ( v12 > 0x3FFFFFFFFFFFFFELL )
    goto LABEL_13;
  v12 *= 32LL;
  v13 = (void *)sub_15F50(v12);
  qword_26708 = 1;
  ptr = v13;
  if ( (unsigned long long)(qword_26700 / 3) > 0x200 )
  {
    LODWORD(v11) = 9;
    v5 = 1;
    do
      LODWORD(v11) = v11 + 1;
    while ( 1LL << v11 < (unsigned long long)(qword_26700 / 3) );
    v11 = (int)v11;
    v17 = (1LL << v11) - byte_1D1A0[(int)v11];
    v10 = 0xFFFFFFFFFFFFFFELL;
    qword_26718 = v17;
    if ( v17 <= 0xFFFFFFFFFFFFFFELL )
    {
      v14 = 8 * v17 + 8;
      goto LABEL_8;
    }
LABEL_13:
    sub_16300(v12, v5, v10, v11);
  }
  qword_26718 = 509;
  v14 = 4080;
LABEL_8:
  qword_26720 = sub_F5E0(v14) + 8;
  sub_B600(a1);
  sub_B600(a1 + 304);
  v15 = qword_26708;
  v16 = ptr;
  *(long long *)(a1 + 600) = qword_26708;
  *(long long *)(a1 + 296) = v15;
  free(v16);
  free((void *)(qword_26720 - 8));
  return v7;
}



// Function: print_normal_hunk @ 0xcac0
unsigned long long print_normal_hunk(long long a1)
{
  int v1; // eax
  long long v2; // rdx
  long long v3; // rcx
  int v5; // r12d
  long long v6; // rbp
  long long v7; // rbx
  long long v8; // rsi
  long long v9; // rbp
  long long v10; // rbx
  long long v11; // rsi
  long long v12; // [rsp+8h] [rbp-50h] BYREF
  long long v13; // [rsp+10h] [rbp-48h] BYREF
  long long v14; // [rsp+18h] [rbp-40h] BYREF
  long long v15; // [rsp+20h] [rbp-38h] BYREF
  unsigned long long v16; // [rsp+28h] [rbp-30h]

  v16 = __readfsqword(0x28u);
  v1 = sub_F1A0(a1, &v12, &v13, &v14, &v15);
  if ( v1 )
  {
    v5 = v1;
    sub_E2B0(a1, &v12, v2, v3);
    sub_EB70(4);
    sub_F0F0(44, &unk_261A0, v12, v13);
    fputc_unlocked(byte_1D550[v5], stream);
    sub_F0F0(44, &unk_262D0, v14, v15);
    sub_EB70(3);
    fputc_unlocked(10, stream);
    if ( (v5 & 1) != 0 )
    {
      v6 = v12;
      if ( v12 <= v13 )
      {
        v7 = 8 * v12;
        do
        {
          while ( 1 )
          {
            sub_EB70(2);
            v8 = v7 + qword_26258;
            v7 += 8;
            sub_EF20("<", v8, 1);
            sub_EB70(3);
            if ( *(char *)(*(long long *)(qword_26258 + v7) - 1LL) == 10 )
              break;
            if ( v13 < ++v6 )
              goto LABEL_9;
          }
          ++v6;
          putc_unlocked(10, stream);
        }
        while ( v13 >= v6 );
      }
    }
LABEL_9:
    if ( v5 == 3 )
    {
      fputs_unlocked("---\n", stream);
    }
    else if ( (v5 & 2) == 0 )
    {
      return v16 - __readfsqword(0x28u);
    }
    v9 = v14;
    if ( v14 <= v15 )
    {
      v10 = 8 * v14;
      do
      {
        sub_EB70(1);
        v11 = v10 + qword_26388;
        v10 += 8;
        sub_EF20(">", v11, 1);
        sub_EB70(3);
        if ( *(char *)(*(long long *)(qword_26388 + v10) - 1LL) == 10 )
          putc_unlocked(10, stream);
        ++v9;
      }
      while ( v15 >= v9 );
    }
  }
  return v16 - __readfsqword(0x28u);
}



// Function: print_normal_script @ 0xccf0
long long print_normal_script(long long a1)
{
  return sub_EB10(a1, sub_EAF0, sub_CAC0);
}



// Function: tab_from_to @ 0xcd10
unsigned long long tab_from_to(unsigned long long a1, unsigned long long a2)
{
  FILE *v2; // rbp
  long long v3; // r13
  unsigned long long i; // rbx
  unsigned long long v5; // rbx
  unsigned long long v6; // rax

  v2 = stream;
  if ( !byte_264A0 )
  {
    v3 = qword_26498;
    for ( i = qword_26498 + a1 - a1 % qword_26498; a2 >= i; i += v3 )
    {
      putc_unlocked(9, v2);
      a1 = i;
    }
  }
  v5 = a1 + 1;
  if ( a2 > a1 )
  {
    do
    {
      putc_unlocked(32, v2);
      v6 = v5++;
    }
    while ( a2 != v6 );
  }
  return a2;
}



// Function: print_half_line @ 0xcda0
unsigned long long print_half_line(char **a1, unsigned long long a2, unsigned long long a3)
{
  unsigned long long v3; // r14
  char *v4; // rbx
  unsigned long long v5; // r12
  unsigned long long v6; // rbp
  int v7; // r11d
  char *v8; // r15
  char v9; // al
  long long v11; // rax
  unsigned long long v12; // rcx
  unsigned long long v13; // rbx
  int v14; // eax
  size_t v15; // rdx
  FILE *stream; // [rsp+8h] [rbp-70h]
  int na; // [rsp+18h] [rbp-60h]
  size_t nb; // [rsp+18h] [rbp-60h]
  size_t n; // [rsp+18h] [rbp-60h]
  size_t nc; // [rsp+18h] [rbp-60h]
  wchar_t c; // [rsp+2Ch] [rbp-4Ch] BYREF
  long long v23[9]; // [rsp+30h] [rbp-48h] BYREF

  v3 = 0;
  v4 = *a1;
  v5 = (unsigned long long)a1[1];
  v23[1] = __readfsqword(0x28u);
  v23[0] = 0;
  stream = ::stream;
  if ( (unsigned long long)v4 >= v5 )
    return v3;
  v6 = 0;
  while ( 1 )
  {
    v7 = *v4;
    v8 = v4 + 1;
    v9 = *v4;
    if ( *v4 <= 95 )
      break;
    if ( (unsigned char)(v9 - 97) > 0x1Du )
      goto LABEL_10;
LABEL_7:
    if ( a3 > v6 )
    {
      ++v4;
      putc_unlocked(v7, stream);
      v3 = ++v6;
LABEL_13:
      if ( (unsigned long long)v4 >= v5 )
        return v3;
    }
    else
    {
      ++v4;
      ++v6;
      if ( (unsigned long long)v8 >= v5 )
        return v3;
    }
  }
  if ( (char)v7 <= 7 )
  {
LABEL_10:
    na = *v4;
    v11 = sub_16640(&c, v4, v5 - (long long)v4, v23);
    v7 = na;
    if ( (unsigned long long)(v11 - 1) <= 0xFFFFFFFFFFFFFFFCLL )
    {
      n = v11;
      v14 = wcwidth(c);
      v15 = n;
      if ( v14 > 0 )
        v6 += v14;
      if ( v6 <= a3 )
      {
        v3 = v6;
        fwrite_unlocked(v4, 1u, n, stdout);
        v15 = n;
      }
      v4 += v15;
    }
    else
    {
LABEL_11:
      ++v4;
      if ( a3 > v6 )
        putc_unlocked(v7, stream);
    }
    goto LABEL_13;
  }
  switch ( v9 )
  {
    case 8:
      ++v4;
      if ( v6 )
      {
        if ( a3 > --v6 )
        {
          if ( v6 >= v3 )
          {
            if ( v6 > v3 )
            {
              do
              {
                ++v3;
                putc_unlocked(32, stream);
              }
              while ( v6 != v3 );
              v4 = v8;
            }
          }
          else
          {
            v3 = v6;
            putc_unlocked(8, stream);
          }
        }
      }
      goto LABEL_13;
    case 9:
      v12 = qword_26498 - v6 % qword_26498;
      if ( v3 == v6 )
      {
        v13 = v12 + v3;
        if ( byte_264A0 )
        {
          if ( a3 <= v13 )
            v13 = a3;
          if ( v13 > v3 )
          {
            do
            {
              ++v3;
              nc = v12;
              putc_unlocked(32, stream);
              v12 = nc;
            }
            while ( v3 != v13 );
          }
        }
        else if ( a3 > v13 )
        {
          nb = qword_26498 - v6 % qword_26498;
          v3 += v12;
          putc_unlocked(9, stream);
          v12 = nb;
        }
      }
      v6 += v12;
      v4 = v8;
      goto LABEL_13;
    case 10:
      return v3;
    case 11:
    case 12:
      goto LABEL_11;
    case 13:
      ++v4;
      v3 = 0;
      v6 = 0;
      putc_unlocked(13, stream);
      sub_CD10(0, a2);
      goto LABEL_13;
    case 32:
    case 33:
    case 34:
    case 35:
    case 37:
    case 38:
    case 39:
    case 40:
    case 41:
    case 42:
    case 43:
    case 44:
    case 45:
    case 46:
    case 47:
    case 48:
    case 49:
    case 50:
    case 51:
    case 52:
    case 53:
    case 54:
    case 55:
    case 56:
    case 57:
    case 58:
    case 59:
    case 60:
    case 61:
    case 62:
    case 63:
    case 65:
    case 66:
    case 67:
    case 68:
    case 69:
    case 70:
    case 71:
    case 72:
    case 73:
    case 74:
    case 75:
    case 76:
    case 77:
    case 78:
    case 79:
    case 80:
    case 81:
    case 82:
    case 83:
    case 84:
    case 85:
    case 86:
    case 87:
    case 88:
    case 89:
    case 90:
    case 91:
    case 92:
    case 93:
    case 94:
    case 95:
      goto LABEL_7;
    default:
      goto LABEL_10;
  }
  return v3;
}



// Function: print_1sdiff_line @ 0xd0a0
void print_1sdiff_line(long long a1, char a2, long long a3)
{
  long long v3; // r8
  FILE *v5; // r15
  unsigned long long v6; // r12
  unsigned long long v7; // r14
  char v8; // bp
  unsigned long long v9; // r8
  unsigned long long v10; // rax
  unsigned long long v11; // r8
  int v12; // edi
  char v13; // [rsp+0h] [rbp-48h]
  unsigned long long v14; // [rsp+8h] [rbp-40h]

  v3 = a1;
  v5 = stream;
  v6 = qword_26430;
  v7 = qword_26428;
  if ( a2 == 60 )
  {
    sub_EB70(2);
    v3 = a1;
    goto LABEL_18;
  }
  if ( a2 == 62 )
  {
    sub_EB70(1);
    v3 = a1;
LABEL_18:
    v13 = 1;
    if ( !v3 )
    {
      v8 = 0;
      v11 = sub_CD10(0, (v6 + v7 - 1) >> 1) + 1;
      goto LABEL_20;
    }
    goto LABEL_4;
  }
  v13 = 0;
  if ( !a1 )
  {
    v8 = 0;
    v9 = 0;
    if ( a2 == 32 )
      goto LABEL_5;
LABEL_14:
    v11 = sub_CD10(v9, (v6 + v7 - 1) >> 1) + 1;
    if ( a2 == 124 )
    {
      v12 = 124;
      if ( (*(char *)(*(long long *)(a3 + 8) - 1LL) == 10) != v8 )
        v12 = v8 == 0 ? 92 : 47;
      goto LABEL_21;
    }
LABEL_20:
    v12 = a2;
LABEL_21:
    v14 = v11;
    putc_unlocked(v12, v5);
    v9 = v14;
    goto LABEL_5;
  }
LABEL_4:
  v8 = *(char *)(*(long long *)(v3 + 8) - 1LL) == 10;
  v9 = sub_CDA0((char **)v3, 0, v6);
  if ( a2 != 32 )
    goto LABEL_14;
LABEL_5:
  if ( a3 && (v8 |= *(char *)(*(long long *)(a3 + 8) - 1LL) == 10, **(char **)a3 != 10) )
  {
    v10 = sub_CD10(v9, v7);
    sub_CDA0((char **)a3, v10, v6);
    if ( !v8 )
    {
LABEL_8:
      if ( !v13 )
        return;
LABEL_12:
      sub_EB70(3);
      return;
    }
  }
  else if ( !v8 )
  {
    goto LABEL_8;
  }
  putc_unlocked(10, v5);
  if ( v13 )
    goto LABEL_12;
}



// Function: print_sdiff_common_lines @ 0xd260
void print_sdiff_common_lines(long long a1, long long a2)
{
  long long v3; // rbp
  long long v4; // rbx
  bool v5; // r14
  bool v6; // r15
  long long v7; // rdx
  long long v8; // rdi

  if ( !byte_26438 )
  {
    v3 = qword_26730;
    v4 = qword_26728;
    v5 = qword_26730 != a1;
    v6 = qword_26728 != a2;
    if ( qword_26730 != a1 || qword_26728 != a2 )
    {
      if ( byte_2643A )
        __fprintf_chk(stream, 1, "i%ld,%ld\n", a1 - qword_26730, a2 - qword_26728);
      if ( byte_26439 )
        goto LABEL_17;
      if ( v5 && v6 )
      {
        while ( 1 )
        {
          ++v4;
          ++v3;
          sub_D0A0(qword_26258 + 8 * v3 - 8, 32, qword_26388 + 8 * v4 - 8);
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
        v7 = qword_26388 + 8 * v4++;
        sub_D0A0(0, 41, v7);
      }
      while ( a2 != v4 );
      while ( a1 != v3 )
      {
        v8 = qword_26258 + 8 * v3++;
        sub_D0A0(v8, 40, 0);
LABEL_17:
        ;
      }
    }
  }
  qword_26730 = a1;
  qword_26728 = a2;
}



// Function: print_sdiff_hunk @ 0xd3c0
unsigned long long print_sdiff_hunk(long long a1)
{
  int v1; // eax
  int v3; // r12d
  long long i; // rbx
  long long v5; // rdx
  long long j; // rbx
  long long v7; // rdi
  long long v8; // rcx
  long long v9; // rbx
  long long v10; // rdx
  long long v11; // rbp
  long long v12; // rax
  long long v13; // rdx
  long long v14; // rdi
  long long v15; // [rsp+8h] [rbp-40h] BYREF
  long long v16; // [rsp+10h] [rbp-38h] BYREF
  long long v17; // [rsp+18h] [rbp-30h] BYREF
  long long v18; // [rsp+20h] [rbp-28h] BYREF
  unsigned long long v19; // [rsp+28h] [rbp-20h]

  v19 = __readfsqword(0x28u);
  v1 = sub_F1A0(a1, &v15, &v16, &v17, &v18);
  if ( v1 )
  {
    v3 = v1;
    sub_D260(v15, v17);
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
          v13 = qword_26388 + 8 * v11++;
          v14 = qword_26258 + 8 * v9++;
          sub_D0A0(v14, 124, v13);
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
        v5 = qword_26388 + 8 * i;
        sub_D0A0(0, 62, v5);
      }
      qword_26728 = i;
    }
    if ( (v3 & 1) != 0 )
    {
      for ( j = v15; v16 >= j; ++j )
      {
        v7 = qword_26258 + 8 * j;
        sub_D0A0(v7, 60, 0);
      }
      qword_26730 = j;
    }
  }
  return v19 - __readfsqword(0x28u);
}



// Function: print_sdiff_script @ 0xd5b0
void print_sdiff_script(long long a1, long long a2, long long a3, long long a4)
{
  sub_E2B0(a1, a2, a3, a4);
  qword_26728 = -qword_26288;
  qword_26730 = -qword_26288;
  sub_EB10(a1, sub_EAF0, sub_D3C0);
  sub_D260(qword_26270, qword_263A0);
}



// Function: sighandler @ 0xd600
long long sighandler(int a1)
{
  long long result; // rax

  result = (unsigned int)sig;
  if ( !sig )
    sig = a1;
  return result;
}



// Function: stophandler @ 0xd620
long long stophandler()
{
  long long result; // rax

  result = (unsigned int)sig;
  if ( !sig )
    return (unsigned int)++dword_26770;
  return result;
}



// Function: get_funky_string @ 0xd640
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



// Function: c_escape_char @ 0xd8b0
long long c_escape_char(unsigned int a1)
{
  unsigned int v1; // r8d
  long long result; // rax

  v1 = a1;
  if ( (char)a1 > 34 )
  {
    if ( (char)a1 == 92 )
      return v1;
    goto LABEL_6;
  }
  if ( (char)a1 <= 6 )
  {
LABEL_6:
    LOBYTE(v1) = (char)a1 <= 31;
    return v1;
  }
  switch ( (char)a1 )
  {
    case 7:
      return 97;
    case 8:
      return 98;
    case 9:
      result = 116;
      break;
    case 10:
      result = 110;
      break;
    case 11:
      result = 118;
      break;
    case 12:
      result = 102;
      break;
    case 13:
      result = 114;
      break;
    case 34:
      return v1;
    default:
      goto LABEL_6;
  }
  return result;
}



// Function: c_escape @ 0xd970
char *c_escape(char *a1)
{
  char *v1; // r12
  unsigned int v2; // edi
  char *v3; // rsi
  long long v4; // rcx
  char v5; // al
  char v6; // r9
  long long v7; // rdx
  char *v9; // rax
  char v10; // cl
  char *v11; // r9
  char *i; // rsi
  char v13; // al
  char v14; // cl

  v1 = a1;
  v2 = *a1;
  if ( !(char)v2 )
    return v1;
  v3 = v1;
  v4 = 0;
  do
  {
    while ( (char)v2 == 32 )
    {
      v2 = (char)*++v3;
      v6 = 1;
      if ( !(char)v2 )
        goto LABEL_9;
    }
    v5 = sub_D8B0(v2);
    if ( v5 )
    {
      v7 = v4 + 3;
      ++v4;
      if ( v5 == 1 )
        v4 = v7;
    }
    v2 = (char)*++v3;
  }
  while ( (char)v2 );
LABEL_9:
  if ( !v4 && !v6 )
    return v1;
  v9 = (char *)sub_15F50(v4 + v3 - v1 + 3);
  *v9 = 34;
  v10 = *v1;
  v11 = v9;
  for ( i = v9 + 1; v10; ++v1 )
  {
    v13 = sub_D8B0(v10);
    if ( v13 )
    {
      *i = 92;
      if ( v13 == 1 )
      {
        i += 4;
        *(i - 3) = ((unsigned char)v14 >> 6) + 48;
        *(i - 1) = (v14 & 7) + 48;
        *(i - 2) = ((v14 >> 3) & 7) + 48;
      }
      else
      {
        i += 2;
        *(i - 1) = v13;
      }
    }
    else
    {
      *i++ = v14;
    }
    v10 = v1[1];
  }
  *(short *)i = 34;
  return v11;
}



// Function: parse_diff_color @ 0xda80
unsigned long long parse_diff_color()
{
  unsigned char *v0; // rbx
  void *v2; // r13
  char v3; // al
  char v4; // al
  bool v5; // zf
  long long v6; // r15
  const char *v7; // rsi
  long long *v8; // rcx
  char *v9; // rax
  char *v10; // rax
  void *v11; // rbx
  void *v12; // rdi
  long long *v13; // r12
  unsigned char *v14; // [rsp+10h] [rbp-58h] BYREF
  void *v15; // [rsp+18h] [rbp-50h] BYREF
  char s1[3]; // [rsp+25h] [rbp-43h] BYREF
  unsigned long long v17; // [rsp+28h] [rbp-40h]

  v0 = (unsigned char *)qword_26748;
  v17 = __readfsqword(0x28u);
  v14 = (unsigned char *)qword_26748;
  if ( qword_26748 && *(char *)qword_26748 )
  {
    strcpy(s1, "??");
    v15 = (void *)sub_162E0(qword_26748);
    v2 = v15;
    while ( 1 )
    {
      while ( 1 )
      {
        v3 = *v0;
        if ( *v0 != 42 )
          break;
        v13 = (long long *)sub_15F50(40);
        v13[4] = (long long)qword_26750;
        qword_26750 = v13;
        v13[1] = (long long)v15;
        v14 = v0 + 1;
        if ( !(unsigned char)sub_D640((long long *)&v15, &v14, 1u, v13) )
          goto LABEL_15;
        v5 = *v14++ == 61;
        if ( !v5 )
          goto LABEL_15;
        v13[3] = (long long)v15;
        if ( !(unsigned char)sub_D640((long long *)&v15, &v14, 0, v13 + 2) )
          goto LABEL_15;
LABEL_21:
        v0 = v14;
      }
      if ( v3 != 58 )
        break;
      v14 = ++v0;
    }
    if ( !v3 )
      return v17 - __readfsqword(0x28u);
    s1[0] = *v0;
    v4 = v0[1];
    v14 = v0 + 1;
    if ( v4 )
    {
      s1[1] = v4;
      v5 = v0[2] == 61;
      v14 = v0 + 3;
      if ( v5 )
      {
        v6 = 0;
        v7 = "lc";
        while ( strcmp(s1, v7) )
        {
          v7 = off_25900[++v6];
          if ( !v7 )
            goto LABEL_14;
        }
        v8 = (long long *)(&size + 2 * (int)v6);
        v8[1] = (long long)v15;
        if ( (unsigned char)sub_D640((long long *)&v15, &v14, 0, v8) )
          goto LABEL_21;
LABEL_14:
        v9 = dcgettext(0, "unrecognized prefix: %s", 5);
        error(0, 0, v9, s1);
      }
    }
LABEL_15:
    v10 = dcgettext(0, "unparsable value for --palette", 5);
    error(0, 0, v10);
    free(v2);
    v11 = qword_26750;
    while ( v11 )
    {
      v12 = v11;
      v11 = (void *)*((long long *)v11 + 4);
      free(v12);
    }
    byte_26758 = 0;
  }
  return v17 - __readfsqword(0x28u);
}



// Function: install_signal_handlers @ 0xdcf0
unsigned long long install_signal_handlers()
{
  int v0; // r15d
  char *v1; // rbx
  char *v2; // rbp
  int v3; // ebp
  void (*v4)(int); // rax
  struct sigaction v6; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v7; // [rsp+98h] [rbp-40h]

  v0 = 20;
  v1 = (char *)&unk_1D524;
  v2 = (char *)&unk_1D524;
  v7 = __readfsqword(0x28u);
  sigemptyset(&set);
  while ( 1 )
  {
    sigaction(v0, 0, &v6);
    if ( v6.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&set, v0);
    if ( v2 == byte_1D550 )
      break;
    v0 = *(int *)v2;
    v2 += 4;
  }
  v6.sa_flags = 0x10000000;
  v3 = 20;
  *(__m128i *)v6.sa_mask.__val = _mm_load_si128((const __m128i *)&set);
  *(__m128i *)&v6.sa_mask.__val[2] = _mm_load_si128((const __m128i *)&set.__val[2]);
  *(__m128i *)&v6.sa_mask.__val[4] = _mm_load_si128((const __m128i *)&set.__val[4]);
  *(__m128i *)&v6.sa_mask.__val[6] = _mm_load_si128((const __m128i *)&set.__val[6]);
  *(__m128i *)&v6.sa_mask.__val[8] = _mm_load_si128((const __m128i *)&set.__val[8]);
  *(__m128i *)&v6.sa_mask.__val[10] = _mm_load_si128((const __m128i *)&set.__val[10]);
  *(__m128i *)&v6.sa_mask.__val[12] = _mm_load_si128((const __m128i *)&set.__val[12]);
  *(__m128i *)&v6.sa_mask.__val[14] = _mm_load_si128((const __m128i *)&set.__val[14]);
  while ( 1 )
  {
    if ( sigismember(&set, v3) )
    {
      v4 = (void (*)(int))sub_D600;
      if ( v3 == 20 )
        v4 = (void (*)(int))sub_D620;
      v6.sa_handler = v4;
      sigaction(v3, &v6, 0);
    }
    if ( v1 == byte_1D550 )
      break;
    v3 = *(int *)v1;
    v1 += 4;
  }
  return v7 - __readfsqword(0x28u);
}



// Function: check_color_output @ 0xde60
void check_color_output(char a1)
{
  int v1; // eax
  int v2; // ebx
  int v3; // eax
  bool v4; // zf

  if ( stream )
  {
    v1 = dword_2657C;
    if ( dword_2657C )
    {
      if ( byte_26180 )
      {
        v2 = 1;
      }
      else
      {
        if ( a1 )
        {
          if ( dword_2657C == 2 )
          {
            byte_26758 = 1;
            sub_DA80();
          }
          else
          {
            byte_26758 = 0;
          }
          return;
        }
        v3 = fileno(stream);
        v4 = isatty(v3) == 0;
        v1 = dword_2657C;
        v2 = !v4;
      }
      if ( v1 == 2 || v1 == 1 && v2 )
      {
        byte_26758 = 1;
        sub_DA80();
      }
      else
      {
        byte_26758 = 0;
      }
      if ( v2 )
        sub_DCF0();
    }
  }
}



// Function: perror_with_name @ 0xdf30
void perror_with_name(const char *a1)
{
  int *v1; // rax

  v1 = __errno_location();
  error(0, *v1, "%s", a1);
}



// Function: message5 @ 0xdf60
unsigned long long message5(const char *src, long long a2, long long a3, char *a4, char *a5)
{
  char *v7; // r12
  const char *v8; // rbp
  char *v9; // rbx
  long long v10; // r13
  long long v11; // rbx
  size_t v12; // rax
  char *v13; // rsi
  long long v15; // rbx
  long long *v16; // r14
  char *v17; // rcx
  size_t v18; // r13
  long long *v19; // rax
  long long v20[11]; // [rsp+0h] [rbp-88h]
  unsigned long long v21; // [rsp+58h] [rbp-30h]

  v7 = a5;
  v8 = src;
  v9 = a4;
  v21 = __readfsqword(0x28u);
  if ( byte_26480 )
  {
    v20[1] = a2;
    v10 = 8;
    if ( !a4 )
      v9 = "";
    v20[2] = a3;
    if ( !a5 )
      v7 = "";
    v20[3] = v9;
    v11 = 0;
    v20[4] = v7;
    while ( 1 )
    {
      v12 = strlen(src) + 1;
      v20[v11 + 6] = v12;
      ++v11;
      v10 += v12;
      if ( v11 == 5 )
        break;
      src = (const char *)v20[v11];
    }
    v15 = 0;
    v16 = (long long *)sub_15F50(v10);
    v17 = (char *)(v16 + 1);
    while ( 1 )
    {
      v18 = v20[v15 + 6];
      ++v15;
      v17 = (char *)memcpy(v17, v8, v18) + v18;
      if ( v15 == 5 )
        break;
      v8 = (const char *)v20[v15];
    }
    v19 = (long long *)off_260A0;
    off_260A0 = (long long)v16;
    *v19 = v16;
    *v16 = 0;
    return v21 - __readfsqword(0x28u);
  }
  else
  {
    if ( byte_2643A )
      putchar_unlocked(32);
    v13 = dcgettext(0, src, 5);
    return __printf_chk(1, v13, a2, a3, v9, v7);
  }
}



// Function: message @ 0xe0e0
unsigned long long message(const char *a1, long long a2, long long a3)
{
  return sub_DF60(a1, a2, a3, 0, 0);
}



// Function: print_message_queue @ 0xe0f0
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



// Function: pfatal_with_name @ 0xe1f0
long long pfatal_with_name(const char *a1)
{
  int v1; // ebp

  v1 = *__errno_location();
  sub_E0F0();
  error(2, v1, "%s", a1);
  return sub_E230((char *)&dword_0 + 2);
}



// Function: fatal @ 0xe230
long long fatal(char *msgid)
{
  char *v1; // rax

  sub_E0F0();
  v1 = dcgettext(0, msgid, 5);
  error(2, 0, "%s", v1);
  return sub_E270(2);
}



// Function: set_color_palette @ 0xe270
void set_color_palette(void *a1)
{
  qword_26748 = a1;
}



// Function: setup_output @ 0xe280
void setup_output(long long a1, long long a2, char a3)
{
  qword_26768 = a1;
  qword_26760 = a2;
  byte_26759 = a3;
  stream = 0;
}



// Function: begin_output @ 0xe2b0
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
  ptr = sub_D970((char *)qword_26768);
  v9 = sub_D970((char *)qword_26760);
  v3 = (void *)sub_16460((unsigned int)"diff%s %s %s", qword_26420, (int)ptr, (int)v9, v1, v2);
  if ( !byte_26480 )
  {
    stream = stdout;
    sub_DE60(0);
    if ( !byte_26759 )
    {
LABEL_5:
      free(v3);
      if ( dword_26580 == 2 )
      {
        sub_87E0((long long)&unk_261A0, (const char **)&ptr, 0);
      }
      else if ( dword_26580 == 3 )
      {
        sub_87E0((long long)&unk_261A0, (const char **)&ptr, 1);
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
    sub_E1F0(v6);
    goto LABEL_26;
  }
  argv[2] = (char *)v3;
  argv[0] = "/usr/bin/pr";
  argv[1] = "-h";
  argv[3] = 0;
  if ( pipe(pipedes) )
  {
LABEL_27:
    sub_E1F0("pipe");
    goto LABEL_28;
  }
  v4 = fork();
  pid = v4;
  if ( v4 < 0 )
  {
LABEL_26:
    sub_E1F0("fork");
    goto LABEL_27;
  }
  if ( !v4 )
  {
    close(pipedes[1]);
    if ( pipedes[0] )
    {
      if ( dup2(pipedes[0], 0) < 0 )
      {
        sub_E1F0("dup2");
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
    sub_DE60(1);
    goto LABEL_5;
  }
LABEL_28:
  sub_E1F0("fdopen");
  return sub_E550();
}



// Function: finish_output @ 0xe550
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
    if ( !ferror_unlocked(stream) )
    {
      if ( fclose(stream) )
      {
        v4 = dcgettext(0, "write failed", 5);
        sub_E1F0(v4);
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
      sub_E1F0("waitpid");
    }
    sub_E230("write failed");
    goto LABEL_14;
  }
LABEL_8:
  stream = 0;
  return v6 - __readfsqword(0x28u);
}



// Function: lines_differ @ 0xe690
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



// Function: find_change @ 0xeaf0
long long find_change(long long a1)
{
  return a1;
}



// Function: print_script @ 0xeb10
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



// Function: set_color_context @ 0xeb70
void set_color_context(int a1)
{
  if ( a1 != 3 )
    sub_EC70();
  if ( byte_26758 && dword_26010 != a1 )
  {
    sub_DF10(size, off_26028);
    switch ( a1 )
    {
      case 0:
        sub_DF10(qword_26060, off_26068);
        break;
      case 1:
        sub_DF10(qword_26070, off_26078);
        break;
      case 2:
        sub_DF10(qword_26080, off_26088);
        break;
      case 3:
        sub_DF10(qword_26050, off_26058);
        break;
      case 4:
        sub_DF10(qword_26090, off_26098);
        break;
      default:
        abort();
    }
    sub_DF10(qword_26030, off_26038);
    dword_26010 = a1;
  }
}



// Function: process_signals @ 0xec70
unsigned long long process_signals()
{
  int v0; // r12d
  sigset_t v2; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v3; // [rsp+88h] [rbp-20h]

  v3 = __readfsqword(0x28u);
  while ( sig || dword_26770 )
  {
    sub_EB70(3);
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



// Function: output_1_line @ 0xed50
void output_1_line(unsigned long long ptr, char *a2, const char *a3, long long a4)
{
  char *v4; // rbx
  unsigned long long v5; // rbp
  size_t v6; // r12
  size_t v7; // rax
  FILE *v8; // r13
  unsigned long long v9; // r15
  long long v10; // r14
  long long v11; // rdi
  unsigned long long v12; // r12
  unsigned long long v14; // [rsp+10h] [rbp-48h]

  v4 = (char *)ptr;
  if ( byte_264A0 )
  {
    v8 = stream;
    v9 = 0;
    v10 = 1;
    v14 = qword_26498;
    if ( ptr >= (unsigned long long)a2 )
      return;
    while ( 1 )
    {
      v11 = (unsigned char)*v4++;
      if ( (char)v11 == 9 )
      {
        v12 = v14 - v9 % v14;
        v9 += v12;
        do
        {
          putc_unlocked(32, v8);
          --v12;
        }
        while ( v12 );
      }
      else
      {
        if ( (char)v11 == 13 )
        {
          putc_unlocked(v11, v8);
          if ( a3 && a2 > v4 )
          {
            v9 = 0;
            if ( *v4 != 10 )
              __fprintf_chk(v8, 1, a3, a4);
            goto LABEL_15;
          }
        }
        else
        {
          if ( (char)v11 != 8 )
          {
            v9 += ((*__ctype_b_loc())[v11] & 0x4000) != 0;
            putc_unlocked(v11, v8);
            goto LABEL_14;
          }
          if ( v9 )
          {
            --v9;
            putc_unlocked(v11, v8);
            goto LABEL_14;
          }
        }
        v9 = 0;
      }
LABEL_14:
      if ( a2 == v4 )
        return;
LABEL_15:
      if ( ++v10 == 1024 )
      {
        sub_EC70();
        v10 = 0;
      }
    }
  }
  v5 = (unsigned long long)&a2[-ptr];
  if ( a2 != (char *)ptr )
  {
    do
    {
      v6 = 1024;
      if ( v5 <= 0x400 )
        v6 = v5;
      v7 = fwrite_unlocked(v4, 1u, v6, stream);
      if ( v6 > v7 )
        break;
      v5 -= v7;
      v4 += v7;
      sub_EC70();
    }
    while ( v5 );
  }
}



// Function: print_1_line_nl @ 0xef20
void print_1_line_nl(char *a1, unsigned long long *a2, char a3)
{
  char *v4; // r14
  char *v5; // r12
  FILE *v6; // r15
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
      sub_ED50((unsigned long long)v4, &v5[-(*(v5 - 1) == 10)], v7, (long long)a1);
      if ( !*a1 )
        return;
    }
    else
    {
      sub_ED50((unsigned long long)v4, v5, v7, (long long)a1);
      if ( !*a1 )
        return;
    }
  }
  else if ( a3 )
  {
    sub_ED50(*a2, &v5[-(*(v5 - 1) == 10)], 0, 0);
  }
  else
  {
    sub_ED50((unsigned long long)v4, (char *)a2[1], 0, 0);
  }
  if ( *(v5 - 1) != 10 )
  {
    sub_EB70(3);
    v10 = dcgettext(0, "No newline at end of file", 5);
    __fprintf_chk(v6, 1, "\n\\ %s\n", v10);
  }
}



// Function: print_1_line @ 0xf0a0
void print_1_line(char *a1, unsigned long long *a2)
{
  sub_EF20(a1, a2, 0);
}



// Function: translate_line_number @ 0xf0b0
long long translate_line_number(long long a1, long long a2)
{
  return *(long long *)(a1 + 232) + a2 + 1;
}



// Function: translate_range @ 0xf0c0
long long translate_range(long long a1, long long a2)
{
  long long v2; // rax
  long long v3; // rdx
  long long *v4; // rcx
  long long result; // rax
  long long *v6; // r8

  v2 = sub_F0B0(a1, a2 - 1);
  *v4 = v2 + 1;
  result = sub_F0B0(a1, v3 + 1) - 1;
  *v6 = result;
  return result;
}



// Function: print_number_range @ 0xf0f0
long long print_number_range(long long a1, long long a2, long long a3)
{
  char v3; // r10
  long long v5; // [rsp+8h] [rbp-20h]
  long long v6; // [rsp+10h] [rbp-18h]

  sub_F0C0(a2, a3);
  if ( v6 <= v5 )
    return __fprintf_chk(stream, 1, "%ld", v6);
  else
    return __fprintf_chk(stream, 1, "%ld%c%ld", v5, (unsigned int)v3, v6);
}



// Function: analyze_hunk @ 0xf1a0
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



// Function: concat @ 0xf560
long long concat(const char *a1, const char *a2, const char *a3)
{
  size_t v4; // rbx
  size_t v5; // rbx
  size_t v6; // rax
  long long v7; // r12

  v4 = strlen(a1);
  v5 = strlen(a2) + v4;
  v6 = strlen(a3);
  v7 = sub_15F50(v5 + v6 + 1);
  __sprintf_chk(v7, 1, -1, "%s%s%s", a1, a2, a3);
  return v7;
}



// Function: zalloc @ 0xf5e0
void *zalloc(size_t n)
{
  void *v1; // rax

  v1 = (void *)sub_15F50(n);
  return memset(v1, 0, n);
}



// Function: debug_script @ 0xf600
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


