// Function: main @ 0x5090
long long main(int a1, char **a2, char **a3)
{
  int v5; // r12d
  int v6; // eax
  long long v7; // r12
  int v8; // r13d
  long long v9; // rdi
  unsigned int v10; // eax
  long long v11; // r8
  char *v12; // rax
  const char *v13; // r12
  char *v14; // rax
  char *v15; // rdi
  unsigned int v16; // eax
  long long v17; // r8
  long long v18; // rdi
  unsigned int v19; // eax
  long long v20; // r8
  char *v21; // r12
  const char *v22; // r12
  const char *v23; // r12
  int v24; // eax
  FILE *v25; // r13
  long long v26; // r14
  char *v27; // r12
  long long v28; // rdx
  long long v29; // rax
  size_t v30; // rax
  long long v31; // r12
  int v32; // eax
  char *v33; // r8
  int v34; // eax
  long long v35; // r13
  char *v36; // rsi
  size_t v37; // r12
  long long v38; // rdx
  long long v39; // rax
  int v40; // eax
  long long i; // r13
  long long v42; // rdi
  long long v43; // rsi
  unsigned int v44; // eax
  long long v45; // rbp
  char v46; // r12
  long long result; // rax
  long long v48; // r12
  long long v49; // r12
  int v50; // ebx
  int v51; // eax
  unsigned long long v52; // rcx
  unsigned long long v53; // rax
  unsigned char v54; // bp
  char **v55; // rbx
  char *v56; // rdi
  char v57; // al
  int v58; // eax
  char *v59; // rax
  long long v60; // rdx
  size_t v61; // rax
  char **v62; // rax
  char v63; // al
  int v64; // eax
  int v65; // eax
  int v66; // r9d
  char *v67; // rax
  char *v68; // rax
  char *v69; // r12
  char v70; // al
  char *v71; // r13
  char *v72; // rax
  long long *v73; // rdi
  long long v74; // rcx
  struct stat *p_buf; // rsi
  char v76; // r8
  char *v77; // rax
  int *v78; // rax
  int *v79; // rax
  char *v80; // rax
  char *v81; // [rsp+0h] [rbp-1C8h]
  int v82; // [rsp+8h] [rbp-1C0h]
  int v83; // [rsp+Ch] [rbp-1BCh]
  char *v84; // [rsp+10h] [rbp-1B8h]
  long long v85; // [rsp+18h] [rbp-1B0h]
  int v86; // [rsp+20h] [rbp-1A8h]
  int errnum; // [rsp+24h] [rbp-1A4h]
  char **v88; // [rsp+28h] [rbp-1A0h]
  int *v89; // [rsp+28h] [rbp-1A0h]
  long long v90; // [rsp+38h] [rbp-190h]
  long long v91; // [rsp+40h] [rbp-188h] BYREF
  long long v92; // [rsp+48h] [rbp-180h] BYREF
  long long v93; // [rsp+50h] [rbp-178h] BYREF
  char v94[8]; // [rsp+58h] [rbp-170h] BYREF
  struct stat buf; // [rsp+60h] [rbp-168h] BYREF
  struct stat v96; // [rsp+F0h] [rbp-D8h] BYREF
  char v97; // [rsp+185h] [rbp-43h]
  char v98[2]; // [rsp+186h] [rbp-42h] BYREF
  unsigned long long v99; // [rsp+188h] [rbp-40h]

  v99 = __readfsqword(0x28u);
  byte_2A684 = 10;
  v91 = 0;
  v92 = 0;
  status = 2;
  dword_2A804 = -1;
  qword_2A7E0 = 0x7FFFFFFFFFFFFFFFLL;
  qword_2A7F8 = -1;
  qword_2A7F0 = -1;
  v93 = -1;
  byte_2A8FD = 0;
  setlocale(6, "");
  bindtextdomain("grep", "/usr/local/share/locale");
  textdomain("grep");
  sub_145D0(&byte_2A180);
  sub_1E1D0(sub_7A80);
  sub_DD40(0);
  qword_2A8C8 = (void *)sub_18320(0, 0, sub_73D0, sub_7420, 0);
  if ( !qword_2A8C8 )
    goto LABEL_209;
  v86 = 0;
  v83 = 0;
  v82 = -1;
  v81 = 0;
  while ( 1 )
  {
    v5 = optind;
    v6 = sub_7AA0(a1, a2);
    if ( v6 == -1 )
      break;
    switch ( v6 )
    {
      case 0:
      case 85:
        continue;
      case 65:
        sub_7900(optarg, &qword_2A7F0);
        continue;
      case 66:
        sub_7900(optarg, &qword_2A7F8);
        continue;
      case 67:
        sub_7900(optarg, &v93);
        continue;
      case 68:
        v13 = optarg;
        if ( !strcmp(optarg, "read") )
        {
          dword_2A884 = 1;
        }
        else
        {
          if ( strcmp(v13, "skip") )
            goto LABEL_208;
          dword_2A884 = 2;
        }
        continue;
      case 69:
        v82 = sub_77B0("egrep");
        continue;
      case 70:
        v82 = sub_77B0("fgrep");
        continue;
      case 71:
        v82 = sub_77B0("grep");
        continue;
      case 72:
        v86 = 1;
        continue;
      case 73:
        goto LABEL_16;
      case 76:
        dword_2A80C = 2;
        continue;
      case 80:
        v82 = sub_77B0("perl");
        continue;
      case 82:
        dword_2A014 = 778;
        goto LABEL_12;
      case 84:
        byte_2A8FC = 1;
        continue;
      case 86:
        byte_2A905 = 1;
        continue;
      case 88:
        v82 = sub_77B0(optarg);
        continue;
      case 90:
        dword_2A804 = 0;
        continue;
      case 97:
        goto LABEL_14;
      case 98:
        byte_2A800 = 1;
        continue;
      case 99:
        goto LABEL_43;
      case 100:
        v40 = dword_20788[sub_D900("--directories", optarg, off_28F80, dword_20788, 4, off_2A078, 1)];
        if ( v40 != 3 )
          v5 = v83;
        dword_2A018 = v40;
        v83 = v5;
        continue;
      case 101:
        v35 = v91;
        v36 = optarg;
        v37 = strlen(optarg);
        v38 = v37 + v91 - v92 + 1;
        if ( v38 > 0 )
        {
          v39 = sub_1D2B0(v81, &v92, v38, -1, 1);
          v36 = optarg;
          v81 = (char *)v39;
          qword_2A8D8 = v39;
        }
        *(char *)mempcpy(&v81[v91], v36, v37) = 10;
        v91 = sub_7BD0(v81, v35, v37 + v35 + 1, "");
        continue;
      case 102:
        v23 = optarg;
        v24 = strcmp(optarg, "-");
        v25 = stdin;
        if ( v24 )
        {
          v25 = fopen(v23, "r");
          if ( !v25 )
            goto LABEL_206;
        }
        v88 = a2;
        v85 = v91;
        v26 = v91;
        v27 = v81;
        while ( 1 )
        {
          v28 = v92;
          if ( v26 - v92 + 2 > 0 )
          {
            v29 = sub_1D2B0(v27, &v92, v26 - v92 + 2, -1, 1);
            v28 = v92;
            qword_2A8D8 = v29;
            v27 = (char *)v29;
          }
          v30 = fread_unlocked(&v27[v26], 1u, v28 - (v26 + 1), v25);
          if ( !v30 )
            break;
          v26 += v30;
        }
        v90 = v26 + 1;
        v84 = &v27[v26];
        v81 = v27;
        v31 = v26;
        a2 = v88;
        v89 = __errno_location();
        errnum = *v89;
        v32 = ferror_unlocked(v25);
        v33 = v84;
        if ( v32 )
          goto LABEL_70;
        if ( stdin == v25 )
        {
          clearerr_unlocked(v25);
          v33 = v84;
        }
        else
        {
          v34 = fclose(v25);
          v33 = v84;
          if ( v34 )
          {
            errnum = *v89;
LABEL_70:
            if ( errnum )
            {
              error(2, errnum, "%s", optarg);
LABEL_208:
              v80 = dcgettext(0, "unknown devices method", 5);
              error(2, 0, v80);
LABEL_209:
              sub_1D4B0();
            }
          }
        }
        if ( v31 != v91 && v81[v31 - 1] != 10 )
        {
          *v33 = 10;
          v31 = v90;
        }
        v91 = sub_7BD0(v81, v85, v31, optarg);
        continue;
      case 104:
        v86 = -1;
        continue;
      case 105:
      case 121:
        byte_2A687 = 1;
        continue;
      case 108:
        dword_2A80C = 1;
        continue;
      case 109:
        if ( (unsigned int)sub_1D560(optarg) <= 1 )
          continue;
        v14 = dcgettext(0, "invalid max count", 5);
        error(2, 0, v14);
LABEL_43:
        byte_2A7E8 = 1;
        continue;
      case 110:
        byte_2A801 = 1;
        continue;
      case 111:
        byte_2A8FD = 1;
        continue;
      case 113:
        byte_2A7A1 = 1;
        status = 0;
        continue;
      case 114:
LABEL_12:
        dword_2A018 = 3;
        v83 = v5;
        continue;
      case 115:
        byte_2A904 = 1;
        continue;
      case 117:
        v12 = dcgettext(0, "warning: --unix-byte-offsets (-u) is obsolete", 5);
        error(0, 0, v12);
        continue;
      case 118:
        byte_2A802 = 1;
        continue;
      case 119:
        sub_CA30();
        byte_2A686 = 1;
        continue;
      case 120:
        byte_2A685 = 1;
        continue;
      case 122:
        byte_2A684 = 0;
        continue;
      case 128:
        v22 = optarg;
        if ( !strcmp(optarg, "binary") )
        {
          dword_2A810 = 0;
        }
        else if ( !strcmp(v22, "text") )
        {
LABEL_14:
          dword_2A810 = 1;
        }
        else
        {
          if ( strcmp(v22, "without-match") )
          {
            v77 = dcgettext(0, "unknown binary-files type", 5);
            error(2, 0, v77);
            goto LABEL_203;
          }
LABEL_16:
          dword_2A810 = 2;
        }
        break;
      case 129:
        v21 = optarg;
        if ( !optarg )
          goto LABEL_143;
        if ( (unsigned int)sub_DDF0(optarg, "always")
          && (unsigned int)sub_DDF0(v21, "yes")
          && (unsigned int)sub_DDF0(v21, "force") )
        {
          if ( (unsigned int)sub_DDF0(v21, "never")
            && (unsigned int)sub_DDF0(v21, "no")
            && (unsigned int)sub_DDF0(v21, "none") )
          {
            if ( (unsigned int)sub_DDF0(v21, "auto")
              && (unsigned int)sub_DDF0(v21, "tty")
              && (unsigned int)sub_DDF0(v21, "if-tty") )
            {
              dword_2A908 = 1;
            }
            else
            {
LABEL_143:
              dword_2A900 = 2;
            }
          }
          else
          {
            dword_2A900 = 0;
          }
        }
        else
        {
          dword_2A900 = 1;
        }
        continue;
      case 130:
        v15 = optarg;
        sub_147B0(optarg);
        if ( !qword_2A8A0 )
          qword_2A8A0 = sub_14D20(v15);
        v16 = sub_76D0(0);
        v18 = v17;
        sub_14FB0(v17, optarg, v16);
        if ( !qword_2A8A8 )
          qword_2A8A8 = sub_14D20(v18);
        v19 = sub_76D0(1);
        sub_14FB0(v20, optarg, v19);
        continue;
      case 131:
      case 134:
        v7 = 0;
        v8 = (v6 == 134) << 29;
        while ( 1 )
        {
          v9 = (unsigned int)v7;
          if ( !qword_2A8B0[v7] )
          {
            v9 = (unsigned int)v7;
            qword_2A8B0[v7] = sub_14D20((unsigned int)v7);
          }
          v10 = sub_76D0(v9);
          sub_14FB0(v11, optarg, v8 | v10);
          if ( v7 == 1 )
            break;
          v7 = 1;
        }
        continue;
      case 132:
        for ( i = 0; ; i = 1 )
        {
          v42 = (unsigned int)i;
          v43 = qword_2A8B0[i];
          if ( !v43 )
          {
            v42 = (unsigned int)i;
            v43 = sub_14D20((unsigned int)i);
            qword_2A8B0[i] = v43;
          }
          v44 = sub_76D0(v42);
          if ( (unsigned int)sub_15440(sub_14FB0, v43, optarg, v44, 10) )
          {
            v78 = __errno_location();
            error(2, *v78, "%s", optarg);
LABEL_206:
            v79 = __errno_location();
            error(2, *v79, "%s", optarg);
          }
          if ( i == 1 )
            break;
        }
        continue;
      case 133:
        off_2A070 = optarg;
        continue;
      case 135:
        byte_2A7D8 = 1;
        continue;
      case 136:
        qword_2A7D0 = (long long)optarg;
        continue;
      case 137:
        byte_2A687 = 0;
        continue;
      default:
        goto LABEL_188;
    }
  }
  if ( byte_2A905 )
  {
    v65 = sub_179E0();
    sub_1CF80((int)stdout, v65, (unsigned int)"GNU grep", (unsigned int)"3.8", 0, v66, (char)v81);
    v67 = dcgettext(
            0,
            "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",
            5);
    puts(v67);
    return 0;
  }
  if ( dword_2A908 )
    sub_A440(0);
  if ( v81 )
  {
    if ( !v91 )
    {
      byte_2A686 = 0;
      byte_2A685 = 0;
      byte_2A802 ^= 1u;
      v91 = 1;
      *v81 = 10;
    }
  }
  else
  {
    if ( optind >= a1 )
LABEL_188:
      sub_A440(2);
    v59 = a2[optind++];
    v60 = 0;
    if ( v82 != 2 && *v59 == 92 )
      v60 = v59[1] == 45;
    v81 = (char *)sub_1D490(&v59[v60]);
    qword_2A8D8 = (long long)v81;
    v61 = strlen(v81);
    v81[v61] = 10;
    v91 = sub_7BD0(v81, 0, v61 + 1, "");
  }
  v45 = --v91;
  sub_184E0(qword_2A8C8);
  v46 = byte_2A7A1;
  if ( byte_2A7A1 )
  {
    v46 = 0;
LABEL_95:
    dword_2A80C = 0;
LABEL_96:
    byte_2A7E8 = 0;
    byte_2A7A2 = 1;
    goto LABEL_97;
  }
  if ( !fstat(1, &buf) )
  {
    v58 = buf.st_mode & 0xF000;
    if ( v58 == 0x8000 )
    {
      v73 = &qword_2A920;
      v74 = 36;
      p_buf = &buf;
      while ( v74 )
      {
        *(int *)v73 = p_buf->st_dev;
        p_buf = (struct stat *)((char *)p_buf + 4);
        v73 = (long long *)((char *)v73 + 4);
        --v74;
      }
    }
    else if ( v58 == 0x2000 )
    {
      if ( !stat("/dev/null", &v96) && buf.st_ino == v96.st_ino && buf.st_dev == v96.st_dev )
        byte_2A7A0 = 1;
      else
        v46 = 1;
    }
  }
  if ( byte_2A7A0 )
    goto LABEL_95;
  if ( dword_2A80C )
    goto LABEL_96;
LABEL_97:
  byte_2A803 = byte_2A7A2 | byte_2A7E8;
  if ( qword_2A7F0 < 0 )
    qword_2A7F0 = v93;
  if ( qword_2A7F8 < 0 )
    qword_2A7F8 = v93;
  if ( (!qword_2A7E0 || !v45 && byte_2A802 && !byte_2A685 && !byte_2A686) && dword_2A80C != 2 )
    return 1;
  if ( dword_2A900 == 2 )
  {
    v64 = v46 && (unsigned int)sub_1D920() && isatty(1) != 0;
    dword_2A900 = v64;
  }
  sub_1D960();
  if ( dword_2A900 )
  {
    v68 = getenv("GREP_COLOR");
    v69 = v68;
    if ( v68 )
    {
      v70 = *v68;
      if ( v70 )
      {
        v71 = v69;
        while ( v70 == 59 || (unsigned char)sub_DA90((unsigned int)v70) )
        {
          v70 = v71[1];
          if ( !v70 )
          {
            off_2A060[0] = v69;
            off_2A068[0] = v69;
            break;
          }
          ++v71;
        }
      }
    }
    sub_7970();
    if ( off_2A068[0] == v69 || off_2A060[0] == v69 )
    {
      v72 = dcgettext(0, "warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'", 5);
      error(0, 0, v72, v69, v69);
    }
  }
  sub_7520();
  v48 = 0;
  if ( v82 >= 0 )
    v48 = v82;
  if ( v82 <= 2 )
  {
    if ( byte_2A687 )
      sub_7D30();
    if ( v82 == 2 )
    {
      if ( byte_2A180 )
      {
        v76 = sub_7730(v81, v45);
        v63 = 1;
        if ( !v76 )
        {
          v63 = 0;
          if ( byte_2A687 )
            v63 = sub_7F20(v81, v45) ^ 1;
        }
      }
      else
      {
        v63 = 0;
        if ( qword_2A8D0 == 1 )
          v63 = byte_2A686;
      }
      v48 = 2;
      if ( (v63 & 1) != 0 )
      {
        v48 = 0;
        sub_A6B0(&qword_2A8D8, &v91);
        v81 = (char *)qword_2A8D8;
      }
    }
    else if ( qword_2A8D0 > 1 )
    {
      v48 = (int)sub_7FB0((unsigned int)v48, v81, &v91);
    }
  }
  v49 = 32 * v48;
  qword_2A878 = *(long long (**)(long long, long long, long long, long long, long long))&aGrep_0[v49 + 24];
  qword_2A870 = (*(long long (**)(char *, long long, long long, bool))&aGrep_0[v49 + 16])(
                  v81,
                  v91,
                  *(int *)&aGrep_0[v49 + 12],
                  (dword_2A900 | (unsigned char)byte_2A8FD) != 0);
  v97 = 0;
  v98[0] = byte_2A684;
  v98[1] = 0;
  byte_2A820 = (qword_2A878(qword_2A870, v98, 1, v94, 0) == 0) == byte_2A802;
  v50 = a1 - optind;
  if ( v86 || v50 > 1 )
    v51 = v86 >= 0;
  else
    v51 = -(dword_2A018 == 3);
  dword_2A808 = v51;
  v52 = sysconf(30);
  if ( v52 - 1 > 0x3FFFFFFFFFFFFFFALL )
LABEL_203:
    abort();
  qword_2A838 = v52;
  v53 = 98304;
  if ( 0x18000 % v52 )
    v53 = v52 + 98304 - 0x18000 % v52;
  nbytes = v52 + v53 + 8;
  dest = (void *)sub_1D110();
  if ( (dword_2A014 & 2) != 0 && !dword_2A884 )
    dword_2A884 = 1;
  if ( v50 <= 0 )
  {
    if ( dword_2A018 == 3 && v83 > 0 )
    {
      byte_2A88A = 1;
      v62 = (char **)&off_28F70;
    }
    else
    {
      v62 = &off_28F60;
    }
    result = (unsigned char)sub_A3F0(*v62);
  }
  else
  {
    v54 = 1;
    v55 = &a2[optind];
    v56 = *v55;
    do
    {
      v57 = sub_A3F0(v56);
      v56 = v55[1];
      ++v55;
      v54 &= v57;
    }
    while ( v56 );
    result = v54;
  }
  if ( byte_2A889 )
    return 2;
  return result;
}



// Function: possible_backrefs_in_pattern @ 0x62e0
char *possible_backrefs_in_pattern(char *a1, long long a2, char a3)
{
  char *result; // rax
  int v4; // ebp
  long long v5; // rsi
  char *v6; // rbx

  result = 0;
  v4 = a3 == 0 ? 128 : 92;
  v5 = a2 - 1;
  v6 = &a1[v5];
  if ( v5 >= 0 )
  {
    while ( 1 )
    {
      result = (char *)memchr(a1, 92, v6 - a1);
      if ( !result )
        break;
      if ( (unsigned char)(result[1] - 49) <= 8u )
        return (char *)(&dword_0 + 1);
      if ( result[1] == v4 && v6 == ++result )
        return 0;
      a1 = result + 1;
    }
  }
  return result;
}



// Function: kwsmusts @ 0x6350
void kwsmusts(struct obstack **a1)
{
  long long v1; // rax
  char *v2; // rbp
  long long v3; // rax
  bool v4; // zf
  long long v5; // r12
  size_t v6; // rax
  size_t v7; // r15
  size_t v8; // r13
  char *v9; // rax
  char v10; // r8
  void *v11; // r12
  long long v12; // rcx
  char *v13; // rax
  char v14; // [rsp+Fh] [rbp-39h]

  v1 = sub_13730(a1[1]);
  if ( v1 )
  {
    v2 = (char *)v1;
    v3 = sub_CA70(0);
    v4 = *v2 == 0;
    *a1 = (struct obstack *)v3;
    v5 = v3;
    if ( v4 )
    {
      v6 = strlen(v2 + 3);
      sub_BAB0(v5, v2 + 3, v6);
    }
    else
    {
      a1[7] = (struct obstack *)((char *)a1[7] + 1);
      v7 = strlen(v2 + 3);
      v8 = (unsigned char)v2[2] + v7 + (unsigned char)v2[1];
      v9 = (char *)sub_1D110(v8);
      v10 = byte_2A684;
      v11 = v9;
      *v9 = byte_2A684;
      v12 = (unsigned char)v2[1];
      v14 = v10;
      *((char *)a1 + 64) |= v12;
      v13 = memcpy(&v9[v12], v2 + 3, v7);
      if ( v2[2] )
        v13[v7] = v14;
      sub_BAB0(*a1, v11, v8);
      free(v11);
    }
    sub_BFA0(*a1);
    j_free(v2);
  }
}



// Function: regex_compile @ 0x6460
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
  if ( !byte_2A687 && !(char)a7 )
  {
    buffer.translate = 0;
    buffer.fastmap = (char *)sub_1D110(256);
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
  if ( a5 == -1 || (v14 = (const char *)sub_89E0(a5, &v15), !*v14) )
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



// Function: dfaerror @ 0x6630
long long dfaerror(const char *a1)
{
  error(2, 0, "%s", a1);
  return sub_6660();
}



// Function: dfawarn @ 0x6660
void dfawarn(long long a1)
{
  char *v1; // rax

  v1 = dcgettext(0, "warning: %s", 5);
  error(0, 0, v1, a1);
}



// Function: GEAcompile @ 0x6690
long long GEAcompile(char *a1, size_t a2, reg_syntax_t a3, char a4)
{
  long long v5; // r12
  long long v6; // rax
  bool v7; // zf
  char v8; // r13
  long long v9; // rax
  unsigned char v10; // al
  long long v11; // rdx
  reg_syntax_t v12; // rcx
  unsigned char v13; // bl
  size_t v14; // r15
  long long v15; // r14
  long long v16; // rax
  char *v17; // r13
  reg_syntax_t v18; // rbp
  long long v19; // rdx
  char v20; // al
  char v21; // cl
  unsigned char v22; // al
  size_t v23; // r9
  long long v24; // rdx
  long long v25; // rax
  reg_syntax_t v26; // r15
  char *v27; // rbp
  char *v28; // rbp
  char *v30; // rax
  size_t v31; // rax
  size_t v32; // rbx
  const char *v33; // rsi
  char *v34; // rax
  size_t v35; // rax
  size_t v36; // rax
  size_t v37; // rax
  char v38; // [rsp+Fh] [rbp-A9h]
  char *v39; // [rsp+10h] [rbp-A8h]
  size_t v40; // [rsp+18h] [rbp-A0h]
  char *src; // [rsp+20h] [rbp-98h]
  char *ptr; // [rsp+28h] [rbp-90h]
  char *v43; // [rsp+30h] [rbp-88h]
  size_t v44; // [rsp+38h] [rbp-80h]
  char *s; // [rsp+40h] [rbp-78h]
  size_t n; // [rsp+48h] [rbp-70h]
  char v47; // [rsp+58h] [rbp-60h]
  long long v49; // [rsp+68h] [rbp-50h] BYREF
  long long v50[9]; // [rsp+70h] [rbp-48h] BYREF

  s = a1;
  v50[1] = __readfsqword(0x28u);
  v5 = sub_1D390(1, 72);
  v6 = sub_13F00();
  v7 = byte_2A687 == 0;
  *(long long *)(v5 + 8) = v6;
  if ( !v7 )
    a3 |= 0x400000uLL;
  sub_142E0(v6, &byte_2A180, a3, a3 & 0x10 | (2 * (unsigned int)(byte_2A684 == 0)) | 0x2C);
  v8 = byte_2A182 | byte_2A180 ^ 1;
  v47 = v8;
  v43 = &a1[a2];
  v9 = sub_1D0F0(64);
  *(long long *)(v5 + 24) = 0;
  v49 = 1;
  *(long long *)(v5 + 16) = v9 + 64;
  v50[0] = 0;
  v39 = (char *)rawmemchr(a1, 10);
  v10 = (unsigned char)sub_62E0(a1, v39 - a1, v8);
  src = a1;
  v11 = 0;
  v12 = a3;
  v13 = v10;
  v14 = v39 - a1;
  v38 = 0;
  v15 = 0;
  v16 = 1;
  v17 = a1;
  v40 = 0;
  v18 = v12;
  ptr = 0;
  while ( 1 )
  {
    v19 = v11 - v16 + 2;
    if ( v19 > 0 )
      *(long long *)(v5 + 16) = sub_1D2B0(*(long long *)(v5 + 16) - 64LL, &v49, v19, -1, 64) + 64;
    re_set_syntax(v18);
    v20 = sub_6460(v5, v17, v14, *(long long *)(v5 + 24), v15, v18, v13 ^ 1u);
    v21 = v38;
    if ( !v20 )
      v21 = 1;
    v17 = v39 + 1;
    ++v15;
    v38 = v21;
    if ( !v13 )
      break;
    ++*(long long *)(v5 + 24);
    if ( v43 < v17 )
    {
      v26 = v18;
      v27 = v39 + 1;
      goto LABEL_18;
    }
    v39 = (char *)rawmemchr(v17, 10);
    v14 = v39 - v17;
    src = v17;
    v13 = (unsigned char)sub_62E0(v17, v39 - v17, v47);
LABEL_6:
    v11 = *(long long *)(v5 + 24);
    v16 = v49;
  }
  if ( v43 >= v17 )
  {
    v39 = (char *)rawmemchr(v17, 10);
    v14 = v39 - v17;
    v22 = (unsigned char)sub_62E0(v17, v39 - v17, v47);
    v13 = v22 & (v17 > src);
    if ( v13 )
    {
      v23 = v17 - src;
      v24 = v17 - src + v40 - v50[0];
      if ( v24 > 0 )
      {
        v25 = sub_1D2B0(ptr, v50, v24, -1, 1);
        v23 = v17 - src;
        ptr = (char *)v25;
      }
      v44 = v23;
      memcpy(&ptr[v40], src, v23);
      v40 += v44;
    }
    else
    {
      v13 = v22;
    }
    goto LABEL_6;
  }
  v26 = v18;
  v27 = src;
LABEL_18:
  if ( v21 )
    exit(2);
  if ( v43 >= v27 )
  {
    if ( a1 < v27 )
    {
      ptr = (char *)sub_1D180(ptr, v40 + v43 - v27);
      memcpy(&ptr[v40], v27, v43 - v27);
      v40 += v43 - v27;
    }
    else
    {
      v40 = a2;
      ptr = a1;
    }
  }
  if ( byte_2A686 || (v28 = 0, byte_2A685) )
  {
    v30 = (char *)sub_1D110(a2 + 45);
    v28 = v30;
    if ( byte_2A685 )
    {
      if ( (v26 & 0x2000) != 0 )
      {
        strcpy(v30, "^(");
        v31 = strlen(v30);
        v32 = a2 + v31;
        memcpy(&v28[v31], a1, a2);
        v33 = ")$";
      }
      else
      {
        *(int *)v30 = 2645086;
        v36 = strlen(v30);
        v32 = a2 + v36;
        memcpy(&v28[v36], a1, a2);
        v33 = "\\)$";
      }
    }
    else if ( (v26 & 0x2000) != 0 )
    {
      *(__m128i *)v30 = _mm_load_si128((const __m128i *)&xmmword_1F060);
      strcpy(v30 + 16, ")(");
      v37 = strlen(v30);
      v32 = a2 + v37;
      memcpy(&v28[v37], a1, a2);
      v33 = ")([^[:alnum:]_]|$)";
    }
    else
    {
      *(__m128i *)v30 = _mm_load_si128((const __m128i *)&xmmword_1F080);
      strcpy(v30 + 16, "_]\\)\\(");
      v35 = strlen(v30);
      v32 = a2 + v35;
      memcpy(&v28[v35], a1, a2);
      v33 = "\\)\\([^[:alnum:]_]\\|$\\)";
    }
    v34 = strcpy(&v28[v32], v33);
    s = v28;
    n = v32 + strlen(v34);
    sub_13340(v28, n, *(long long *)(v5 + 8));
  }
  else
  {
    sub_13340(a1, a2, *(long long *)(v5 + 8));
  }
  sub_6350((struct obstack **)v5);
  sub_14250(0, 0, *(long long *)(v5 + 8), 1);
  if ( ptr )
  {
    if ( a4 || !(unsigned char)sub_13480(*(long long *)(v5 + 8)) )
    {
      *(long long *)(v5 + 16) -= 64LL;
      ++*(long long *)(v5 + 24);
      if ( !(unsigned char)sub_6460(v5, ptr, v40, 0, -1, v26, 0) )
        abort();
    }
    if ( s != ptr )
      free(ptr);
  }
  free(v28);
  return v5;
}



// Function: EGexecute @ 0x6c90
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
  v61 = byte_2A684;
  v60 = sub_13460(v7);
  v65 = a2;
  v62 = sub_13470(v7);
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
      v19 = sub_13450(v60, string, v9, 0, &v66, 0);
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
    v10 = sub_C870(v8, &string[-*((unsigned char *)v5 + 64)], *((unsigned char *)v5 + 64) + v57 - string, v67, 1);
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
    if ( byte_2A180 != 1 || byte_2A182 )
      goto LABEL_84;
    if ( v65 < v16 )
      v65 = v16;
    if ( !sub_CAF0(&v65, 0, v12, v57) )
    {
LABEL_84:
      v58 = string - v16;
      goto LABEL_64;
    }
    v22 = v65;
LABEL_21:
    v23 = sub_13450(v5[1], v22, string, 0, &v66, &v64);
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
      *(char *)(v38 + 56) = *(char *)(v38 + 56) & 0x3F | ((byte_2A684 == 10) << 7);
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
      if ( !byte_2A686 )
        goto LABEL_54;
      if ( !byte_2A685 )
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
    if ( !byte_2A685 )
    {
      if ( !byte_2A686 )
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
      while ( sub_CC90(&v32[v34], v52) || sub_CCA0(v33, v32, v52) )
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
              sub_1D4B0();
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



// Function: hash_pattern @ 0x73d0
unsigned long long hash_pattern(long long a1, unsigned long long a2)
{
  unsigned char *v2; // rcx
  unsigned long long v3; // rax
  long long i; // rdx

  v2 = (unsigned char *)(qword_2A8D8 + a1 - 1);
  v3 = 0x3938A0565D85CB03LL;
  for ( i = *v2; (char)i != 10; i = *v2 )
  {
    ++v2;
    v3 = i ^ (33 * v3);
  }
  return v3 % a2;
}



// Function: compare_patterns @ 0x7420
long long compare_patterns(long long a1, long long a2)
{
  char *v2; // rcx
  char *v3; // rsi
  char v4; // dl
  long long v5; // rax

  v2 = (char *)(qword_2A8D8 + a1 - 1);
  v3 = (char *)(qword_2A8D8 + a2 - 1);
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



// Function: color_cap_mt_fct @ 0x7470
char *color_cap_mt_fct()
{
  off_2A060[0] = off_2A068[0];
  return off_2A068[0];
}



// Function: color_cap_rv_fct @ 0x7490
void color_cap_rv_fct()
{
  dword_2A900 = -1;
}



// Function: color_cap_ne_fct @ 0x74a0
const char *color_cap_ne_fct()
{
  off_2A028 = "\x1B[%sm";
  off_2A020 = "\x1B[m";
  return "\x1B[m";
}



// Function: is_device_mode @ 0x74d0
long long is_device_mode(short a1, long long a2, long long a3, int a4)
{
  unsigned int v4; // eax
  int v5; // edx

  v5 = a1 & 0xF000;
  LOBYTE(v4) = (a1 & 0xB000) == 0x2000;
  LOBYTE(a4) = v5 == 49152;
  LOBYTE(v5) = v5 == 4096;
  return v5 | a4 | v4;
}



// Function: skip_devices @ 0x7500
long long skip_devices(int a1)
{
  long long result; // rax

  result = 1;
  if ( dword_2A884 != 2 )
  {
    LOBYTE(result) = dword_2A884 == 0;
    return (a1 ^ 1) & (unsigned int)result;
  }
  return result;
}



// Function: initialize_unibyte_mask @ 0x7520
long long initialize_unibyte_mask()
{
  long long v0; // rdx
  int v1; // eax
  unsigned char v2; // si
  int v3; // ecx

  v0 = 1;
  v1 = 1;
  v2 = 0;
  do
  {
    while ( byte_2A183[v0] == 1 || ((unsigned char)v0 & v2) != 0 )
    {
      if ( ++v0 == 256 )
        goto LABEL_7;
    }
    do
    {
      v3 = v1;
      v1 *= 2;
    }
    while ( v1 <= (int)v0 );
    ++v0;
    v2 |= v3;
    v1 = v3;
  }
  while ( v0 != 256 );
LABEL_7:
  qword_2A868 = 0x101010101010101LL * v2;
  return 0x101010101010101LL;
}



// Function: skip_easy_bytes @ 0x7590
long long *skip_easy_bytes(long long *a1)
{
  long long *v1; // rbx
  long long v2; // rbp
  long long v4; // rdi

  v1 = a1;
  v2 = qword_2A868;
  if ( ((unsigned char)a1 & 7) != 0 )
  {
    while ( ((unsigned char)v2 & (unsigned char)sub_C9D0((unsigned int)*(char *)v1)) == 0 )
    {
      v1 = (long long *)((char *)v1 + 1);
      if ( ((unsigned char)v1 & 7) == 0 )
        goto LABEL_6;
    }
  }
  else
  {
LABEL_6:
    if ( (*v1 & v2) != 0 )
    {
      if ( ((unsigned char)v2 & (unsigned char)sub_C9D0((unsigned int)*(char *)v1)) != 0 )
        return v1;
      goto LABEL_8;
    }
    do
      ++v1;
    while ( (*v1 & v2) == 0 );
    if ( ((unsigned char)v2 & (unsigned char)sub_C9D0((unsigned int)*(char *)v1)) == 0 )
    {
      do
      {
LABEL_8:
        v4 = (unsigned int)*((char *)v1 + 1);
        v1 = (long long *)((char *)v1 + 1);
      }
      while ( ((unsigned char)v2 & (unsigned char)sub_C9D0(v4)) == 0 );
    }
  }
  return v1;
}



// Function: buf_has_encoding_errors @ 0x7630
long long buf_has_encoding_errors(long long *a1, long long a2)
{
  char *v2; // rbp
  long long v3; // rax
  long long *v4; // rax
  long long *v5; // rbx
  long long v7[5]; // [rsp+0h] [rbp-28h] BYREF

  v7[1] = __readfsqword(0x28u);
  if ( qword_2A868 )
  {
    v2 = (char *)a1 + a2;
    v7[0] = 0;
    *((char *)a1 + a2) = -1;
    while ( 1 )
    {
      v4 = sub_7590(a1);
      v5 = v4;
      if ( v2 <= (char *)v4 )
        break;
      v3 = sub_C9E0(v4, v2 - (char *)v4, v7);
      if ( v3 < 0 )
        return 1;
      a1 = (long long *)((char *)v5 + v3);
    }
  }
  return 0;
}



// Function: exclude_options @ 0x76d0
long long exclude_options(char a1)
{
  return a1 == 0 ? 1342177280 : 0x10000000;
}



// Function: all_zeros @ 0x76f0
long long all_zeros(char *a1, long long a2)
{
  char *v2; // rsi

  v2 = &a1[a2];
  if ( a1 >= v2 )
    return 1;
  while ( !*a1 )
  {
    if ( ++a1 == v2 )
      return 1;
  }
  return 0;
}



// Function: open_symlink_nofollow_error @ 0x7720
long long open_symlink_nofollow_error(int a1, long long a2, int a3)
{
  unsigned int v3; // eax

  LOBYTE(v3) = a1 == 40;
  LOBYTE(a3) = a1 == 31;
  return a3 | v3;
}



// Function: contains_encoding_error @ 0x7730
long long contains_encoding_error(long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // rax
  long long v5[7]; // [rsp+0h] [rbp-38h] BYREF

  v5[1] = __readfsqword(0x28u);
  v5[0] = 0;
  if ( a2 <= 0 )
    return 0;
  v2 = 0;
  while ( 1 )
  {
    v3 = sub_C9F0(a1 + v2, a2 - v2, v5);
    if ( v3 < 0 )
      break;
    v2 += v3;
    if ( a2 <= v2 )
      return 0;
  }
  return 1;
}



// Function: setmatcher @ 0x77b0
long long setmatcher(char *s1, int a2)
{
  long long v2; // rbx
  unsigned int v3; // r12d
  char *v4; // rax
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
  return sub_7860();
}



// Function: add_count @ 0x7860
long long add_count(long long a1, long long a2)
{
  long long result; // rax
  char *v3; // rax

  result = a2 + a1;
  if ( __OFADD__(a2, a1) )
  {
    v3 = dcgettext(0, "input is too large to count", 5);
    error(2, 0, v3);
    return sub_7890();
  }
  return result;
}



// Function: buf_has_nulls @ 0x7890
bool buf_has_nulls(const char *a1, long long a2)
{
  a1[a2] = 0;
  return a2 != strlen(a1);
}



// Function: zap_nuls @ 0x78b0
void zap_nuls(char *s, char *a2, char a3)
{
  size_t v5; // rax

  if ( a3 )
  {
    while ( 1 )
    {
      *a2 = 0;
      v5 = strlen(s);
      *a2 = a3;
      s += v5;
      if ( a2 == s )
        break;
      do
        *s++ = a3;
      while ( !*s );
    }
  }
}



// Function: context_length_arg @ 0x7900
long long context_length_arg(char *a1, long long *a2)
{
  long long result; // rax
  char *v3; // rax

  result = sub_1D560(a1);
  if ( (unsigned int)result > 1 || *a2 < 0 )
  {
    v3 = dcgettext(0, "invalid context length argument", 5);
    error(2, 0, "%s: %s", a1, v3);
    return sub_7970();
  }
  return result;
}



// Function: parse_grep_colors @ 0x7970
char *parse_grep_colors()
{
  char *result; // rax
  const char *v1; // r14
  const char *v2; // r13
  const char *i; // r12
  char v4; // bp
  char **v5; // rbx
  const char *v6; // rdi
  char *v7; // rax

  result = getenv("GREP_COLORS");
  if ( result && *result )
  {
    v1 = 0;
    v2 = (const char *)sub_1D490(result);
    for ( i = v2 + 1; ; ++i )
    {
      v4 = *(i - 1);
      if ( v4 == 58 || !v4 )
      {
        *((char *)i - 1) = 0;
        v5 = off_29600;
        v6 = "mt";
        do
        {
          if ( !strcmp(v6, v2) )
            break;
          v6 = v5[3];
          v5 += 3;
        }
        while ( v6 );
        v7 = v5[1];
        if ( v1 && v7 )
          *(long long *)v7 = v1;
        result = v5[2];
        if ( result )
          result = (char *)((long long (*)(void))result)();
        if ( !v4 )
          return result;
        v2 = i;
        v1 = 0;
      }
      else if ( v4 == 61 )
      {
        result = (char *)(i - 1);
        if ( v2 == i - 1 || v1 )
          return result;
        *((char *)i - 1) = 0;
        v1 = i;
      }
      else if ( v1 )
      {
        if ( v4 != 59 )
        {
          result = (char *)sub_DA90((unsigned int)v4);
          if ( !(char)result )
            return result;
        }
      }
    }
  }
  return result;
}



// Function: clean_up_stdout @ 0x7a80
long long clean_up_stdout()
{
  long long result; // rax

  result = (unsigned int)errnum;
  if ( !errnum )
    return sub_DE70();
  return result;
}



// Function: get_nondigit_option @ 0x7aa0
long long get_nondigit_option(int argc, char **argv, long long *a3)
{
  int v3; // ebx
  char *v4; // rdx
  unsigned int v5; // ebp
  char v6; // al
  char v8; // [rsp+7h] [rbp-81h]
  char *v9; // [rsp+8h] [rbp-80h]
  char v11[21]; // [rsp+20h] [rbp-68h] BYREF
  char v12; // [rsp+35h] [rbp-53h] BYREF
  char v13; // [rsp+38h] [rbp-50h] BYREF
  unsigned long long v14; // [rsp+48h] [rbp-40h]

  v3 = optind;
  v14 = __readfsqword(0x28u);
  v8 = 0;
  v9 = v11;
  while ( 1 )
  {
    v5 = getopt_long(argc, argv, "0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz", &longopts, 0);
    v6 = sub_DA90(v5);
    if ( !v6 )
      break;
    if ( dword_2A010 == v3 && v8 )
    {
      v4 = &v9[-(v11[0] == 48)];
      if ( v4 == &v12 )
      {
        *(int *)v4 = 3026478;
        v9 = &v13;
LABEL_9:
        *v9 = 0;
        sub_7900(v11, a3);
        return v5;
      }
    }
    else
    {
      v4 = v11;
    }
    *v4 = v5;
    v9 = v4 + 1;
    v8 = v6;
    dword_2A010 = v3;
    v3 = optind;
  }
  if ( v9 != v11 )
    goto LABEL_9;
  return v5;
}



// Function: update_patterns @ 0x7bd0
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
  long long v13; // rdi
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
    v11 = sub_18750(qword_2A8C8, &v5[-a1 + 1], 0);
    v6 = v11;
    if ( v11 )
    {
      if ( v11 < 0 )
        sub_1D4B0();
      v5 += v4 - (char *)v10;
      if ( v17 )
      {
        v8 = qword_2A8D0;
      }
      else
      {
        v12 = qword_2A8E0;
        v13 = qword_2A8F0;
        if ( qword_2A8E0 == qword_2A8E8 )
        {
          qword_2A8F0 = sub_1D2B0(qword_2A8F0, &qword_2A8E8, 1, -1, 24);
          v13 = qword_2A8F0;
          v12 = qword_2A8E0;
        }
        qword_2A8E0 = v12 + 1;
        v14 = (long long *)(v13 + 24 * v12);
        v8 = qword_2A8D0;
        v14[1] = a4;
        *v14 = v8;
        v14[2] = v7;
      }
      v9 = &v5[-a1];
      qword_2A8D0 = v8 + 1;
    }
    ++v7;
  }
  while ( (unsigned long long)v4 < v18 );
  return v9;
}



// Function: setup_ok_fold @ 0x7d30
unsigned long long setup_ok_fold()
{
  char *v0; // r14
  char *v1; // r13
  long long v2; // rdi
  int v3; // eax
  long long v4; // rdx
  char *v5; // r15
  char *v6; // rbx
  wchar_t v7; // esi
  char v8; // al
  mbstate_t ps; // [rsp+18h] [rbp-E0h] BYREF
  char v11[128]; // [rsp+20h] [rbp-D8h] BYREF
  char s[24]; // [rsp+A0h] [rbp-58h] BYREF
  unsigned long long v13; // [rsp+B8h] [rbp-40h]

  v0 = (char *)&unk_2A6A0;
  v1 = (char *)&unk_2A284;
  v13 = __readfsqword(0x28u);
  do
  {
    v2 = *(unsigned int *)v1;
    if ( (int)v2 != -1 )
    {
      v3 = sub_146B0(v2, v11);
      if ( v3 <= 0 )
      {
LABEL_10:
        v8 = 1;
      }
      else
      {
        v4 = 4LL * v3;
        v5 = &v11[v4];
        v6 = (char *)&ps.__value + v4 + -4 * (unsigned int)(v3 - 1);
        while ( 1 )
        {
          v7 = *((int *)v5 - 1);
          ps = 0;
          if ( wcrtomb(s, v7, &ps) != 1 )
            break;
          v5 -= 4;
          if ( v5 == v6 )
            goto LABEL_10;
        }
        v8 = -1;
      }
      *v0 = v8;
    }
    v1 += 4;
    ++v0;
  }
  while ( v1 != &byte_2A684 );
  return v13 - __readfsqword(0x28u);
}



// Function: fgrep_icase_charlen @ 0x7e40
long long fgrep_icase_charlen(unsigned char *a1, long long a2, long long a3)
{
  long long v3; // rax
  unsigned long long v4; // r13
  unsigned long long v6; // rbp
  unsigned int v7; // [rsp+Ch] [rbp-BCh] BYREF
  char v8[136]; // [rsp+10h] [rbp-B8h] BYREF
  unsigned long long v9; // [rsp+98h] [rbp-30h]

  v9 = __readfsqword(0x28u);
  v3 = *a1;
  if ( *(int *)&byte_2A180[4 * v3 + 260] != -1 )
    return byte_2A6A0[v3];
  v4 = sub_18C40(&v7, a1, a2, a3);
  if ( v4 > 0x10 || (unsigned int)sub_146B0(v7, v8) )
    return -1;
  v6 = v4;
  while ( (long long)--v6 > 0 )
  {
    if ( a1[v6] != (*__ctype_toupper_loc())[a1[v6]] )
      return -1;
  }
  return v4;
}



// Function: fgrep_icase_available @ 0x7f20
long long fgrep_icase_available(long long a1, long long a2)
{
  long long v2; // r13
  int v3; // eax
  long long v5[7]; // [rsp+0h] [rbp-38h] BYREF

  v5[1] = __readfsqword(0x28u);
  v5[0] = 0;
  if ( a2 <= 0 )
    return 1;
  v2 = 0;
  while ( 1 )
  {
    v3 = sub_7E40((unsigned char *)(a1 + v2), a2 - v2, (long long)v5);
    if ( v3 < 0 )
      break;
    v2 += v3;
    if ( a2 <= v2 )
      return 1;
  }
  return 0;
}



// Function: try_fgrep_pattern @ 0x7fb0
long long try_fgrep_pattern(unsigned int a1, unsigned char *a2, long long *a3)
{
  unsigned char *v3; // r15
  long long v5; // r13
  long long v6; // rdi
  void *v7; // rbp
  void *v8; // r14
  char v9; // cl
  signed long long v10; // rbx
  unsigned char *v11; // rsi
  char v13; // cl
  unsigned char v14; // cl
  long long v17[9]; // [rsp+10h] [rbp-48h] BYREF

  v3 = a2;
  v5 = *a3;
  v6 = *a3 + 1;
  v17[1] = __readfsqword(0x28u);
  v17[0] = 0;
  v7 = (void *)sub_1D110(v6);
  v8 = v7;
  if ( v5 )
  {
    while ( 1 )
    {
      v9 = *v3;
      if ( *v3 == 91 )
        goto LABEL_14;
      if ( v9 <= 91 )
        break;
      if ( v9 == 92 )
      {
        if ( v5 <= 1 )
          goto LABEL_8;
        v13 = v3[1];
        if ( v13 <= 60 )
        {
          if ( v13 <= 9 )
            goto LABEL_24;
          if ( ((1LL << v13) & 0x13FE008000000400LL) != 0 )
            goto LABEL_14;
          if ( ((1LL << v13) & 0xB0000000000LL) == 0 )
            goto LABEL_24;
        }
        else
        {
          v14 = v13 - 62;
          if ( v14 > 0x3Eu )
            goto LABEL_24;
          if ( ((1LL << v14) & 0x220001402200011LL) != 0 )
            goto LABEL_14;
          if ( ((1LL << v14) & 0x6000000000000002LL) == 0 )
          {
LABEL_24:
            ++v3;
            --v5;
            goto LABEL_8;
          }
        }
        if ( !a1 )
          goto LABEL_14;
        goto LABEL_24;
      }
      if ( v9 == 94 )
        goto LABEL_14;
      if ( (unsigned char)(v9 - 123) <= 1u )
        goto LABEL_16;
LABEL_8:
      if ( byte_2A687 )
        v10 = sub_7E40(v3, v5, (long long)v17);
      else
        v10 = sub_C9F0(v3, v5, v17);
      if ( v10 < 0 )
        goto LABEL_14;
      v11 = v3;
      v3 += v10;
      v8 = mempcpy(v8, v11, v10);
      v5 -= v10;
      if ( !v5 )
        goto LABEL_12;
    }
    if ( v9 > 46 )
    {
      if ( v9 != 63 )
        goto LABEL_8;
    }
    else
    {
      if ( v9 <= 35 )
        goto LABEL_8;
      if ( ((1LL << v9) & 0x441000000000LL) != 0 )
        goto LABEL_14;
      if ( ((1LL << v9) & 0x90000000000LL) == 0 )
        goto LABEL_8;
    }
LABEL_16:
    if ( a1 )
      goto LABEL_14;
    goto LABEL_8;
  }
LABEL_12:
  a1 = 2;
  if ( *a3 != (char *)v8 - (char *)v7 )
  {
    *a3 = (char *)v8 - (char *)v7;
    *(char *)mempcpy(a2, v7, (char *)v8 - (char *)v7) = 10;
  }
LABEL_14:
  free(v7);
  return a1;
}



// Function: skipped_file @ 0x81e0
long long skipped_file(long long a1, unsigned char a2, unsigned int a3)
{
  long long v3; // r8

  if ( !(char)a3 )
  {
    v3 = qword_2A8B0[a2];
    if ( v3 )
      return sub_14EF0(v3, a1);
    return 0;
  }
  if ( dword_2A018 != 4 )
  {
    if ( !a2 || !byte_2A88A )
    {
      v3 = qword_2A8A0[a2];
      if ( v3 )
        return sub_14EF0(v3, a1);
    }
    return 0;
  }
  return a3;
}



// Function: nlscan @ 0x8250
long long nlscan(unsigned long long a1)
{
  const void *v2; // rdi
  int v3; // r12d
  long long v4; // rbp
  char *v5; // rax
  long long result; // rax

  v2 = (const void *)qword_2A7C0;
  if ( qword_2A7C0 >= a1 )
  {
    v4 = 0;
  }
  else
  {
    v3 = byte_2A684;
    v4 = 0;
    do
    {
      v5 = (char *)memchr(v2, v3, a1 - (long long)v2);
      if ( !v5 )
        break;
      v2 = v5 + 1;
      ++v4;
    }
    while ( a1 > (unsigned long long)(v5 + 1) );
  }
  result = sub_7860(qword_2A818, v4);
  qword_2A7C0 = a1;
  qword_2A818 = result;
  return result;
}



// Function: pr_sgr_start @ 0x82c0
long long pr_sgr_start(char *a1)
{
  long long result; // rax

  if ( *a1 )
    return __printf_chk(1, off_2A028, a1);
  return result;
}



// Function: pr_sgr_start_if @ 0x82e0
long long pr_sgr_start_if(char *a1)
{
  long long result; // rax

  result = (unsigned int)dword_2A900;
  if ( dword_2A900 )
    return sub_82C0(a1);
  return result;
}



// Function: fputs_errno @ 0x8300
int fputs_errno(const char *a1)
{
  int result; // eax

  result = fputs_unlocked(a1, stdout);
  if ( result < 0 )
  {
    result = *__errno_location();
    errnum = result;
  }
  return result;
}



// Function: printf_errno @ 0x8330
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



// Function: putchar_errno @ 0x8410
int putchar_errno(int a1)
{
  int result; // eax

  result = putchar_unlocked(a1);
  if ( result < 0 )
  {
    result = *__errno_location();
    errnum = result;
  }
  return result;
}



// Function: fflush_errno @ 0x8430
int fflush_errno()
{
  int result; // eax

  result = fflush_unlocked(stdout);
  if ( result )
  {
    result = *__errno_location();
    errnum = result;
  }
  return result;
}



// Function: fillbuf @ 0x8560
long long fillbuf(long long a1, long long a2)
{
  char *v3; // r14
  size_t v4; // rdx
  long long v5; // r13
  char *v6; // rbx
  size_t v7; // r15
  long long v8; // rdx
  long long v9; // rcx
  long long v10; // r9
  __off_t v11; // r11
  __off_t v12; // rax
  __off_t v13; // rsi
  __off_t v14; // rax
  char *v15; // rax
  unsigned long long v16; // rdx
  long long v17; // rbp
  size_t v18; // rbp
  long long result; // rax
  long long v20; // r8
  long long v21; // r13
  int v22; // r14d
  long long v23; // rax
  long long v24; // r8
  __off_t v25; // r15
  __off_t v26; // rdx
  long long v27; // rax
  unsigned long long v28; // rdx
  char *v29; // rsi
  long long v30; // [rsp+8h] [rbp-40h]
  __off_t v31; // [rsp+8h] [rbp-40h]
  char *v32; // [rsp+8h] [rbp-40h]

  v3 = (char *)dest;
  v4 = nbytes;
  v5 = qword_2A838;
  v6 = (char *)qword_2A840;
  if ( (char *)dest + nbytes - (char *)qword_2A840 < qword_2A838 + 8 )
  {
    v7 = a1 + 1;
    v30 = ~a1;
    if ( (long long)(qword_2A838 + 8 + qword_2A838 + a1 - nbytes) <= 0 )
    {
      v28 = ((unsigned long long)dest + v7) % qword_2A838;
      v29 = (char *)qword_2A840 + v30;
      if ( v28 )
      {
        v6 = (char *)dest + qword_2A838 + a1 + 1 - v28;
        memmove(&v6[v30], v29, a1 + 1);
        v4 = nbytes;
      }
      else
      {
        v32 = (char *)dest + v7;
        memmove(dest, v29, a1 + 1);
        v4 = nbytes;
        v6 = v32;
      }
    }
    else
    {
      if ( (unsigned char)sub_8550(*(unsigned int *)(a2 + 24)) )
      {
        v12 = *(long long *)(a2 + 48) - offset;
        v13 = v12;
        if ( v12 >= 0 )
        {
          v14 = v12 + v9 + a1;
          if ( !__OFADD__(v13, v9 + a1) )
          {
            v11 = v8 + v10;
            if ( v8 + v10 < v14 )
              v11 = v13 + v9 + a1;
          }
        }
      }
      v15 = (char *)sub_1D2B0(0, &nbytes, v10, v11, 1);
      v5 = qword_2A838;
      v6 = &v15[v7];
      v3 = v15;
      v16 = (unsigned long long)&v15[v7] % qword_2A838;
      if ( v16 )
      {
        v6 = &v15[qword_2A838 + a1 + 1 - v16];
        memmove(&v6[v30], (char *)qword_2A840 + v30, v7);
      }
      else
      {
        memmove(v15, (char *)qword_2A840 + v30, v7);
      }
      free(dest);
      dest = v3;
      v4 = nbytes;
    }
  }
  v17 = &v3[v4 - 8] - v6;
  qword_2A848 = (long long)&v6[-a1];
  v18 = v17 - v17 % v5;
LABEL_15:
  v22 = fd;
  while ( 1 )
  {
    v23 = sub_1BAF0(v22, v6, v18);
    v24 = v23;
    if ( v23 == -1 )
    {
      result = 0;
      goto LABEL_18;
    }
    offset += v23;
    result = (unsigned char)byte_2A821 ^ 1u;
    LOBYTE(result) = (v24 == 0) | byte_2A821 ^ 1;
    if ( (char)result )
      break;
    if ( !(unsigned char)sub_76F0(v6, v24) )
    {
      v6 += v20;
      result = 1;
      goto LABEL_18;
    }
    qword_2A818 = sub_7860(qword_2A818, v20);
    v21 = qword_2A818;
    if ( byte_2A880 )
      goto LABEL_15;
    v25 = offset;
    v22 = fd;
    v26 = lseek(fd, offset, 3);
    if ( v26 >= 0
      || *__errno_location() == 6
      && (unsigned char)sub_8550(*(unsigned int *)(a2 + 24))
      && v25 < *(long long *)(a2 + 48)
      && (v26 = lseek(v22, 0, 2), v26 >= 0) )
    {
      v31 = v26;
      v27 = sub_7860(v21, v26 - v25);
      v22 = fd;
      qword_2A818 = v27;
      offset = v31;
    }
    else
    {
      byte_2A880 = 1;
    }
  }
  v6 += v24;
LABEL_18:
  *(long long *)v6 = 0;
  qword_2A840 = v6;
  return result;
}



// Function: print_line_tail @ 0x88b0
unsigned long long print_line_tail(unsigned long long a1, unsigned long long a2, char *a3)
{
  unsigned long long v3; // r12
  unsigned long long v5; // rbx
  unsigned long long v6; // rdx
  long long v7; // rax

  v3 = a1;
  v5 = a2;
  if ( a2 > a1 )
  {
    v6 = a2;
    v7 = 0;
    if ( *(char *)(a2 - 1) == byte_2A684 )
    {
      v6 = a2 - 1;
      if ( a1 >= a2 - 1 )
        goto LABEL_4;
      v7 = 1;
      if ( *(char *)(a2 - 2) != 13 )
        goto LABEL_4;
    }
    else if ( *(char *)(a2 - 1) != 13 )
    {
LABEL_4:
      v5 = v6;
      goto LABEL_5;
    }
    v5 = a2 - (v7 + 1);
  }
LABEL_5:
  if ( (long long)(v5 - a1) > 0 )
  {
    sub_82C0(a3);
    v3 = v5;
    sub_8460(a1, v5 - a1);
    sub_8890((unsigned char)*a3);
  }
  return v3;
}



// Function: pr_sgr_end_if @ 0x8950
long long pr_sgr_end_if(unsigned char *a1)
{
  long long result; // rax

  result = (unsigned int)dword_2A900;
  if ( dword_2A900 )
    return sub_8890(*a1);
  return result;
}



// Function: print_offset @ 0x8970
long long print_offset(long long a1, unsigned char *a2)
{
  sub_82E0(a2);
  sub_8330((long long)"%*ld", dword_2A8F8, a1);
  return sub_8950(a2);
}



// Function: print_sep @ 0x89b0
long long print_sep(char a1)
{
  sub_82E0(off_2A040[0]);
  sub_8410(a1);
  return sub_8950((unsigned char *)off_2A040[0]);
}



// Function: pattern_file_name @ 0x89e0
long long pattern_file_name(long long a1, long long *a2)
{
  long long *v2; // rcx
  long long v3; // rax
  long long v4; // rdx
  long long *v6; // rcx

  v2 = (long long *)qword_2A8F0;
  if ( qword_2A8E0 <= 1 )
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
      if ( *(long long *)(qword_2A8F0 + v3) > a1 )
      {
        v2 = (long long *)(qword_2A8F0 + v3 - 24);
        goto LABEL_6;
      }
      ++v4;
      v3 += 24;
    }
    while ( v4 != qword_2A8E0 );
    v6 = (long long *)(qword_2A8F0 + 24 * v4 - 24);
    *a2 = v6[2] + a1 - *v6;
    return v6[1];
  }
}



// Function: input_filename @ 0x8a50
char *input_filename()
{
  char *result; // rax

  result = (char *)qword_2A890;
  if ( !qword_2A890 )
  {
    result = dcgettext(0, "(standard input)", 5);
    qword_2A890 = (long long)result;
  }
  return result;
}



// Function: print_filename @ 0x8a90
long long print_filename()
{
  char *v0; // rax

  sub_82E0(off_2A058[0]);
  v0 = sub_8A50();
  sub_8300(v0);
  return sub_8950((unsigned char *)off_2A058[0]);
}



// Function: print_line_head @ 0x8ac0
long long print_line_head(long long *a1, long long a2, unsigned long long a3, char a4)
{
  long long v7; // rdi
  long long result; // rax
  char v9; // r15
  char v10; // al
  long long v11; // rax
  long long v12; // rax
  bool v13; // [rsp+Fh] [rbp-39h]

  if ( dword_2A810 == 1 || (v9 = *((char *)a1 + a2), v10 = sub_7630(a1, a2), *((char *)a1 + a2) = v9, !v10) )
  {
    if ( dword_2A808 )
    {
      sub_8A90();
      if ( dword_2A804 )
        sub_89B0(a4);
      else
        sub_8410(0);
    }
    if ( byte_2A801 )
    {
      v7 = qword_2A818;
      if ( qword_2A7C0 < a3 )
      {
        sub_8250((unsigned long long)a1);
        v12 = sub_7860(qword_2A818, 1);
        qword_2A7C0 = a3;
        qword_2A818 = v12;
        v7 = v12;
      }
      sub_8970(v7, (unsigned char *)off_2A050[0]);
      sub_89B0(a4);
    }
    if ( byte_2A800 )
    {
      v11 = sub_7860(qword_2A7C8, (long long)a1 - qword_2A848);
      sub_8970(v11, (unsigned char *)off_2A048[0]);
      sub_89B0(a4);
    }
    result = (unsigned char)byte_2A8FC;
    if ( byte_2A8FC )
    {
      if ( a2 != 0 && (dword_2A808 | (unsigned char)(byte_2A800 | byte_2A801)) != 0 )
      {
        v13 = a2 != 0 && (dword_2A808 | (unsigned char)(byte_2A800 | byte_2A801)) != 0;
        sub_8410(9);
        return v13;
      }
    }
    else
    {
      return 1;
    }
  }
  else
  {
    byte_2A888 = 1;
    return 0;
  }
  return result;
}



// Function: print_line_middle @ 0x8c60
unsigned long long print_line_middle(unsigned long long a1, unsigned long long a2, char *a3, unsigned char *a4)
{
  unsigned long long v6; // r13
  unsigned long long v7; // rbp
  long long i; // rax
  unsigned long long v9; // r15
  unsigned long long v11; // rsi
  unsigned long long v12; // rdi
  unsigned long long v13; // [rsp+8h] [rbp-60h]
  long long v16[9]; // [rsp+20h] [rbp-48h] BYREF

  v16[1] = __readfsqword(0x28u);
  if ( a1 >= a2 )
  {
    if ( !byte_2A8FD )
      return a1;
    return a2;
  }
  else
  {
    v6 = a1;
    v7 = 0;
    v13 = a2 - a1;
    for ( i = qword_2A878(qword_2A870, a1, a2 - a1, v16, a1); i >= 0; i = qword_2A878(qword_2A870, a1, v13, v16, v6) )
    {
      v9 = a1 + i;
      if ( a2 == a1 + i )
        break;
      if ( v16[0] )
      {
        if ( byte_2A8FD )
        {
          if ( !(unsigned char)sub_8AC0((long long *)(a1 + i), v16[0], a2, byte_2A802 == 0 ? 58 : 45) )
            return 0;
        }
        else
        {
          sub_82C0(a3);
          if ( !v7 )
            v7 = v6;
          v11 = v9 - v7;
          v12 = v7;
          v7 = 0;
          sub_8460(v12, v11);
        }
        sub_82E0(a4);
        sub_8460(v9, v16[0]);
        sub_8950(a4);
        if ( byte_2A8FD )
          sub_8410(byte_2A684);
        v6 = v9 + v16[0];
        if ( a2 <= v9 + v16[0] )
          break;
      }
      else
      {
        v16[0] = 1;
        if ( !v7 )
          v7 = v6;
        v6 = v9 + 1;
        if ( a2 <= v9 + 1 )
          break;
      }
    }
    if ( byte_2A8FD )
      return a2;
    if ( !v7 )
      return v6;
  }
  return v7;
}



// Function: prline @ 0x8e20
int prline(long long *a1, char *a2, char a3)
{
  unsigned long long v4; // rbp
  unsigned long long v5; // rax
  char v6; // r9
  char v7; // di
  char v8; // cl
  int v9; // ebp
  char *v10; // r13
  char *v11; // r8
  char v12; // r8
  char *v13; // rax

  v4 = (unsigned long long)a1;
  LODWORD(v5) = (unsigned char)byte_2A8FD;
  v6 = byte_2A8FD;
  if ( !byte_2A8FD )
  {
    v12 = sub_8AC0(a1, a2 - (char *)a1 - 1, (unsigned long long)a2, a3);
    LODWORD(v5) = (unsigned char)byte_2A8FD;
    v6 = byte_2A8FD;
    if ( !v12 )
      return v5;
  }
  v7 = a3 == 58;
  v8 = v5 & (byte_2A802 ^ (a3 == 58));
  if ( !dword_2A900 )
  {
    if ( !v8 )
    {
LABEL_4:
      if ( v4 < (unsigned long long)a2 && v6 != 1 )
        LODWORD(v5) = sub_8460(v4, &a2[-v4]);
      goto LABEL_7;
    }
    v11 = 0;
    v10 = 0;
    goto LABEL_20;
  }
  v10 = off_2A038;
  v11 = off_2A068[0];
  if ( v7 == ((unsigned char)byte_2A802 & (dword_2A900 < 0)) )
    v10 = off_2A030;
  if ( a3 != 58 )
    v11 = off_2A060[0];
  if ( v8 )
    goto LABEL_20;
  if ( *v10 )
  {
    if ( v7 == byte_2A802 )
      goto LABEL_22;
    if ( !*v11 )
      goto LABEL_27;
    goto LABEL_20;
  }
  if ( !*v11 )
    goto LABEL_4;
  if ( v7 != byte_2A802 )
  {
LABEL_20:
    v5 = sub_8C60(v4, (unsigned long long)a2, v10, (unsigned char *)v11);
    v4 = v5;
    if ( !v5 )
      return v5;
    LODWORD(v5) = (unsigned char)byte_2A8FD;
  }
LABEL_22:
  if ( !(char)v5 )
  {
    if ( !*v10 )
    {
      v6 = 0;
      goto LABEL_4;
    }
LABEL_27:
    v5 = sub_88B0(v4, (unsigned long long)a2, v10);
    v6 = byte_2A8FD;
    v4 = v5;
    goto LABEL_4;
  }
LABEL_7:
  if ( byte_2A7D8 )
    LODWORD(v5) = sub_8430();
  v9 = errnum;
  if ( errnum )
  {
    v13 = dcgettext(0, "write error", 5);
    error(2, v9, v13);
    LODWORD(v5) = sub_8FF0();
  }
  else
  {
    s = a2;
  }
  return v5;
}



// Function: prpending @ 0x8ff0
void prpending(unsigned long long a1)
{
  char *v1; // rax
  long long *v2; // rbp

  if ( !s )
    s = (void *)qword_2A848;
  for ( ; qword_2A7A8 > 0; --qword_2A7A8 )
  {
    v2 = s;
    if ( (unsigned long long)s >= a1 )
      break;
    v1 = rawmemchr(s, byte_2A684);
    sub_8E20(v2, v1 + 1, 45);
  }
}



// Function: prtext @ 0x9070
long long prtext(char *s, char *a2)
{
  int v3; // r12d
  char *v4; // rbp
  char *v5; // rcx
  long long v6; // rax
  long long *v7; // rdi
  char v8; // r15
  long long v9; // r13
  long long result; // rax
  long long v11; // r14
  char *v12; // rax
  char *v13; // rbx

  v3 = byte_2A684;
  if ( byte_2A803 || qword_2A7A8 > 0 && (sub_8FF0((unsigned long long)s), byte_2A803) )
  {
    v8 = byte_2A802;
    v9 = qword_2A7B0;
    if ( !byte_2A802 )
    {
      v9 = qword_2A7B0 - 1;
      goto LABEL_32;
    }
    v4 = s;
    goto LABEL_35;
  }
  v4 = s;
  v5 = (char *)::s;
  if ( !::s )
    v5 = (char *)qword_2A848;
  v6 = 0;
  if ( qword_2A7F8 > 0 )
  {
    do
    {
      while ( v5 >= v4 )
      {
        if ( ++v6 == qword_2A7F8 )
          goto LABEL_25;
      }
      do
        --v4;
      while ( *(v4 - 1) != (char)v3 );
      ++v6;
    }
    while ( v6 != qword_2A7F8 );
LABEL_25:
    if ( !byte_2A688 || ::s == v4 || !off_2A070 )
      goto LABEL_12;
  }
  else
  {
    if ( (qword_2A7F0 & qword_2A7F8) < 0 || ::s == s || !byte_2A688 || !off_2A070 )
    {
      v4 = s;
      goto LABEL_14;
    }
    v4 = s;
  }
  sub_82E0(off_2A040[0]);
  sub_8300(off_2A070);
  sub_8950((unsigned char *)off_2A040[0]);
  sub_8410(10);
LABEL_12:
  while ( s > v4 )
  {
    v7 = v4;
    v4 = (char *)rawmemchr(v4, (char)v3) + 1;
    sub_8E20(v7, v4, 45);
  }
LABEL_14:
  v8 = byte_2A803;
  v9 = qword_2A7B0;
  if ( byte_2A802 )
  {
LABEL_35:
    v11 = 0;
    if ( a2 > v4 )
    {
      while ( v11 < v9 )
      {
        v12 = rawmemchr(v4, v3);
        v13 = v12 + 1;
        if ( v8 )
        {
          ++v11;
          if ( a2 <= v13 )
            goto LABEL_42;
        }
        else
        {
          ++v11;
          sub_8E20(v4, v12 + 1, 58);
          v8 = byte_2A803;
          v9 = qword_2A7B0;
          if ( a2 <= v13 )
          {
LABEL_42:
            v9 -= v11;
            v4 = v13;
            goto LABEL_17;
          }
        }
        v4 = v13;
      }
      v9 -= v11;
    }
    goto LABEL_17;
  }
  v9 = qword_2A7B0 - 1;
  if ( byte_2A803 )
  {
LABEL_32:
    result = 0;
    qword_2A828 = offset + a2 - (char *)qword_2A840;
    goto LABEL_33;
  }
  v4 = a2;
  sub_8E20(s, a2, 58);
  v8 = byte_2A803;
  v9 = qword_2A7B0 - 1;
LABEL_17:
  result = 0;
  qword_2A828 = offset + v4 - (char *)qword_2A840;
  if ( !v8 )
  {
    result = qword_2A7F0;
    if ( qword_2A7F0 < 0 )
      result = 0;
  }
LABEL_33:
  qword_2A7B0 = v9;
  qword_2A7A8 = result;
  byte_2A688 = 1;
  return result;
}



// Function: grepbuf @ 0x9350
long long grepbuf(char *a1, char *a2)
{
  long long v2; // r12
  char *v3; // r13
  char *v5; // rdi
  char *v6; // rsi
  long long v7; // rdx
  long long v8; // rax
  int v10; // edi
  long long v11[7]; // [rsp+0h] [rbp-38h] BYREF

  v2 = qword_2A7B0;
  v11[1] = __readfsqword(0x28u);
  if ( a1 < a2 )
  {
    v3 = a1;
    while ( 1 )
    {
      v8 = qword_2A878(qword_2A870, v3, a2 - v3, v11, 0);
      if ( v8 >= 0 )
      {
        v5 = &v3[v8];
        v6 = &v3[v8 + v11[0]];
        if ( byte_2A802 )
        {
          if ( v3 >= v5 )
          {
            v3 += v8 + v11[0];
            goto LABEL_8;
          }
          v6 = &v3[v8];
          v5 = v3;
          v3 += v8 + v11[0];
        }
        else
        {
          if ( a2 == v5 )
            goto LABEL_18;
          v3 += v8 + v11[0];
        }
      }
      else
      {
        if ( !byte_2A802 )
        {
LABEL_18:
          v7 = qword_2A7B0;
          return v2 - v7;
        }
        v11[0] = 0;
        if ( v3 >= a2 )
          return v2 - qword_2A7B0;
        v5 = v3;
        v6 = a2;
        v3 = a2;
      }
      sub_9070(v5, v6);
      v7 = qword_2A7B0;
      if ( !qword_2A7B0 || byte_2A7A2 )
      {
        if ( byte_2A7A1 )
        {
          v10 = 0;
          if ( byte_2A889 )
            v10 = status;
          exit(v10);
        }
        return v2 - v7;
      }
LABEL_8:
      if ( a2 <= v3 )
        return v2 - qword_2A7B0;
    }
  }
  return 0;
}



// Function: suppressible_error @ 0x94a0
void suppressible_error(int errnum)
{
  char *v1; // rax

  if ( !byte_2A904 )
  {
    v1 = sub_8A50();
    error(0, errnum, "%s", v1);
  }
  byte_2A889 = 1;
}



// Function: file_must_have_nulls @ 0x94e0
bool file_must_have_nulls(__off_t offset, int fd, long long a3)
{
  long long v3; // rbp
  long long v5; // rdx
  __off_t v7; // r13
  int *v8; // rax

  if ( byte_2A881 )
    return 0;
  v3 = offset;
  if ( !(unsigned char)sub_8550(*(unsigned int *)(a3 + 24)) )
    return 0;
  if ( *(long long *)(v5 + 48) <= offset )
    return 0;
  if ( !fd )
  {
    v3 = lseek(0, 0, 1);
    if ( v3 < 0 )
      return 0;
  }
  v7 = lseek(fd, v3, 4);
  if ( v7 < 0 )
    return 0;
  if ( lseek(fd, v3, 0) < 0 )
  {
    v8 = __errno_location();
    sub_94A0(*v8);
  }
  return *(long long *)(a3 + 48) > v7;
}



// Function: finalize_input @ 0x95a0
void finalize_input(int a1, long long a2, char a3)
{
  int *v3; // rax

  if ( a1 )
    return;
  if ( !qword_2A7B0 )
  {
    if ( offset == qword_2A828 || byte_2A881 || lseek(0, qword_2A828, 0) >= 0 )
      return;
LABEL_10:
    v3 = __errno_location();
    sub_94A0(*v3);
    return;
  }
  if ( !a3
    && (byte_2A881 || lseek(0, 0, 2) < 0 && *__errno_location() != 22)
    && !(unsigned char)sub_8490(*(unsigned int *)(a2 + 24)) )
  {
    goto LABEL_10;
  }
}



// Function: grep @ 0x9710
long long grep(int a1, long long a2, char *a3)
{
  char v4; // r13
  char v5; // r8
  long long v6; // rax
  char v7; // dl
  char v8; // r12
  signed long long v9; // r14
  long long v10; // r13
  char *v11; // r15
  char *v12; // rdi
  int v13; // ebx
  char *v14; // rbp
  char *v15; // rbx
  char v16; // r14
  char *v17; // rax
  char *v18; // rdi
  char *v19; // rbp
  long long v20; // rax
  long long v21; // rax
  char v22; // al
  int *v23; // rax
  char *v24; // r12
  char *v25; // rax
  int *v26; // rax
  long long v28; // rsi
  int v29; // edi
  bool v30; // zf
  signed long long v31; // [rsp+8h] [rbp-70h]
  long long v32; // [rsp+10h] [rbp-68h]
  char v33; // [rsp+1Fh] [rbp-59h]
  long long v34; // [rsp+20h] [rbp-58h]
  long long v35; // [rsp+28h] [rbp-50h]
  char v36; // [rsp+30h] [rbp-48h]
  bool v37; // [rsp+31h] [rbp-47h]
  char v38; // [rsp+32h] [rbp-46h]
  char v39; // [rsp+33h] [rbp-45h]

  v4 = byte_2A684;
  v38 = byte_2A7A2;
  v39 = byte_2A803;
  if ( !(unsigned char)sub_9630() )
    return 0;
  qword_2A7C8 = 0;
  s = 0;
  *((long long *)&xmmword_2A7A8 + 1) = qword_2A7E0;
  qword_2A818 = 0;
  qword_2A828 = 0;
  *(long long *)&xmmword_2A7A8 = 0;
  byte_2A821 = byte_2A820 & (v4 == 0);
  byte_2A888 = 0;
  v5 = sub_8560(0, a2);
  if ( !v5 )
  {
    v26 = __errno_location();
    sub_94A0(*v26);
    return 0;
  }
  dword_2A8F8 = 0;
  if ( byte_2A8FC )
  {
    v28 = 0x7FFFFFFFFFFFFFFFLL;
    if ( (unsigned char)sub_8550(*(unsigned int *)(a2 + 24)) )
    {
      v28 = *(long long *)(a2 + 48);
      if ( byte_2A801 )
        v28 += v28 != 0x7FFFFFFFFFFFFFFFLL;
    }
    v29 = 0;
    do
    {
      ++v29;
      v28 /= 10;
    }
    while ( v28 );
    dword_2A8F8 = v29;
  }
  v33 = 0;
  v6 = a2;
  v7 = v5;
  v37 = v4 != 0;
  v8 = v4;
  v32 = -1;
  v9 = 0;
  v10 = v6;
  v34 = 0;
  v31 = 0;
  while ( 1 )
  {
    v36 = v7;
    v11 = (char *)qword_2A840;
    v12 = (char *)qword_2A848;
    if ( v32 < 0 && v37 )
    {
      v13 = dword_2A810;
      if ( dword_2A810 != 1 )
      {
        v35 = qword_2A848;
        v14 = (char *)qword_2A840 - qword_2A848;
        if ( !sub_7890((const char *)qword_2A848, (long long)qword_2A840 - qword_2A848) )
        {
          v12 = (char *)v35;
          if ( !v36 )
            goto LABEL_10;
          if ( !sub_94E0((__off_t)v14, a1, v10) )
          {
            v12 = (char *)qword_2A848;
            v11 = (char *)qword_2A840;
            goto LABEL_10;
          }
          v13 = dword_2A810;
        }
        if ( v13 == 2 )
          return 0;
        if ( !byte_2A7E8 )
        {
          byte_2A803 = 1;
          byte_2A7A2 = 1;
        }
        v33 = v8;
        v12 = (char *)qword_2A848;
        v11 = (char *)qword_2A840;
        byte_2A821 = byte_2A820;
        v32 = v34;
      }
    }
LABEL_10:
    qword_2A7C0 = (long long)v12;
    if ( s )
      s = v12;
    v15 = &v12[v9];
    if ( &v12[v9] == v11 )
      break;
    sub_78B0(&v12[v9], v11, v33);
    v16 = *(v15 - 1);
    *(v15 - 1) = v8;
    v17 = (char *)memrchr(v15 - 1, v8, v11 - v15 + 1);
    *(v15 - 1) = v16;
    v18 = &v15[-v31];
    v19 = v17 + 1;
    if ( v15 == v17 + 1 )
    {
      v19 = &v15[-v31];
      v31 = v11 - v18;
      goto LABEL_22;
    }
    v31 = v11 - v19;
    if ( v19 <= v18 )
      goto LABEL_22;
    if ( !*((long long *)&xmmword_2A7A8 + 1) )
    {
      if ( !(long long)xmmword_2A7A8 )
        goto LABEL_35;
LABEL_17:
      sub_8FF0((unsigned long long)v19);
      if ( xmmword_2A7A8 == 0 )
        goto LABEL_35;
      goto LABEL_18;
    }
    v34 += sub_9350(v18, v19);
    if ( (long long)xmmword_2A7A8 )
      goto LABEL_17;
    if ( !*((long long *)&xmmword_2A7A8 + 1) )
      goto LABEL_35;
LABEL_18:
    if ( byte_2A7A2 )
    {
      v20 = 0;
      if ( v32 >= 0 )
        v20 = v32;
      if ( v20 < v34 )
        goto LABEL_35;
    }
LABEL_22:
    if ( qword_2A7F8 > 0 )
    {
      v21 = 0;
      do
      {
        if ( qword_2A848 >= (unsigned long long)v19 )
          break;
        if ( v19 == s )
          goto LABEL_31;
        ++v21;
        do
          --v19;
        while ( *(v19 - 1) != v8 );
      }
      while ( v21 != qword_2A7F8 );
    }
    if ( v19 != s )
      s = 0;
LABEL_31:
    v9 = v11 - v19;
    if ( byte_2A800 )
    {
      qword_2A7C8 = sub_7860(qword_2A7C8, (long long)qword_2A840 - qword_2A848 - v9);
      if ( byte_2A801 )
LABEL_42:
        sub_8250((unsigned long long)v19);
    }
    else if ( byte_2A801 )
    {
      goto LABEL_42;
    }
    v22 = sub_8560(v11 - v19, v10);
    v7 = 0;
    if ( !v22 )
    {
      v23 = __errno_location();
      sub_94A0(*v23);
      goto LABEL_35;
    }
  }
  *a3 = 1;
  if ( v31 )
  {
    v30 = *((long long *)&xmmword_2A7A8 + 1) == 0;
    *v11 = v8;
    qword_2A840 = v11 + 1;
    if ( !v30 )
      v34 += sub_9350(&v12[v9 - v31], v11 + 1);
    if ( (long long)xmmword_2A7A8 )
      sub_8FF0((unsigned long long)qword_2A840);
  }
LABEL_35:
  byte_2A7A2 = v38;
  byte_2A803 = v39;
  if ( !dword_2A810 && !v39 && (byte_2A888 || v32 >= 0 && v34 > v32) )
  {
    v24 = sub_8A50();
    v25 = dcgettext(0, "%s: binary file matches", 5);
    error(0, 0, v25, v24);
  }
  return v34;
}



// Function: grepdesc @ 0x9c70
long long grepdesc(int fd, unsigned char a2)
{
  unsigned int i; // r13d
  unsigned int v3; // r14d
  struct stat *p_buf; // rsi
  long long v6; // rbx
  long long v8; // rdx
  int v9; // ecx
  int v10; // eax
  int *v11; // rax
  int *v12; // rax
  unsigned int v13; // r12d
  void *v14; // rbp
  long long v15; // rsi
  int *v16; // rax
  int *v17; // rbx
  int *v18; // rax
  char *v19; // r12
  char *v20; // rax
  char v21; // [rsp+Fh] [rbp-D9h] BYREF
  long long v22[2]; // [rsp+10h] [rbp-D8h] BYREF
  struct stat buf; // [rsp+20h] [rbp-C8h] BYREF
  unsigned long long v24; // [rsp+B8h] [rbp-30h]

  v3 = a2;
  v24 = __readfsqword(0x28u);
  v21 = 0;
  p_buf = &buf;
  if ( fstat(fd, &buf) )
  {
    v12 = __errno_location();
    i = 1;
    sub_94A0(*v12);
    goto LABEL_15;
  }
  if ( !fd )
  {
    if ( dword_2A808 < 0 )
      dword_2A808 = (buf.st_mode & 0xF000) == 0x4000;
    goto LABEL_5;
  }
  if ( (unsigned char)sub_7500(a2) && (unsigned char)sub_74D0(buf.st_mode, (long long)&buf, v8, v9)
    || a2
    && (p_buf = (struct stat *)(&dword_0 + 1),
        (unsigned char)sub_81E0(qword_2A890, 1u, (buf.st_mode & 0xF000) == 0x4000)) )
  {
LABEL_27:
    i = 1;
LABEL_28:
    if ( close(fd) )
    {
      v11 = __errno_location();
      sub_94A0(*v11);
    }
    return i;
  }
  if ( dword_2A808 < 0 )
  {
    dword_2A808 = (buf.st_mode & 0xF000) == 0x4000;
    v10 = dword_2A018;
    if ( dword_2A018 != 3 )
      goto LABEL_22;
  }
  else
  {
    v10 = dword_2A018;
    if ( dword_2A018 != 3 )
    {
LABEL_22:
      if ( v10 == 4 && (buf.st_mode & 0xF000) == 0x4000 )
        goto LABEL_27;
      goto LABEL_23;
    }
  }
  if ( (buf.st_mode & 0xF000) != 0x4000 )
  {
LABEL_23:
    if ( dword_2A884 != 2 && (dword_2A884 || a2) || !(unsigned char)sub_74D0(buf.st_mode, (long long)p_buf, v8, v9) )
    {
LABEL_5:
      if ( !byte_2A803
        && !dword_2A80C
        && qword_2A7E0 > 1
        && (buf.st_mode & 0xF000) == 0x8000
        && buf.st_ino == qword_2A928
        && buf.st_dev == qword_2A920 )
      {
        if ( !byte_2A904 )
        {
          v19 = sub_8A50();
          v20 = dcgettext(0, "%s: input file is also the output", 5);
          error(0, 0, v20, v19);
        }
        byte_2A889 = 1;
        i = 1;
      }
      else
      {
        v6 = sub_9710(fd, (long long)&buf, &v21);
        if ( byte_2A7E8 )
        {
          if ( dword_2A808 )
          {
            sub_8A90();
            if ( dword_2A804 )
              sub_89B0(58);
            else
              sub_8410(0);
          }
          sub_8330((long long)"%ld\n", v6);
          if ( byte_2A7D8 )
            sub_8430();
        }
        LOBYTE(i) = v6 == 0;
        if ( dword_2A80C )
        {
          if ( dword_2A80C == (v6 == 0) + 1 )
          {
            sub_8A90();
            sub_8410(dword_2A804 & 0xA);
            if ( byte_2A7D8 )
              sub_8430();
          }
        }
        else
        {
          sub_95A0(fd, (long long)&buf, v21);
        }
      }
LABEL_15:
      if ( !fd )
        return i;
      goto LABEL_28;
    }
    goto LABEL_27;
  }
  v13 = dword_2A014 & ~(a2 ^ 1);
  if ( close(fd) )
  {
    v18 = __errno_location();
    sub_94A0(*v18);
  }
  v22[1] = 0;
  v22[0] = qword_2A890;
  v14 = (void *)sub_16C90(v22, v13, 0);
  if ( !v14 )
    sub_1D4B0();
  for ( i = 1; ; i &= sub_A180(v14, v15, v3) )
  {
    v15 = sub_171F0(v14);
    if ( !v15 )
      break;
  }
  v16 = __errno_location();
  v17 = v16;
  if ( *v16 )
    sub_94A0(*v16);
  if ( (unsigned int)sub_17000(v14) )
    sub_94A0(*v17);
  return i;
}



// Function: grepfile @ 0xa0f0
long long grepfile(long long a1, long long a2, long long a3, unsigned char a4)
{
  int v5; // eax
  int v6; // ecx
  int v7; // r8d
  int v8; // r9d
  int v9; // edi
  long long result; // rax
  int v11; // edi
  int v12; // edx
  int *v13; // rax

  v5 = (unsigned char)sub_7500(a4) << 11;
  if ( (char)v8 )
  {
    BYTE1(v5) |= 1u;
    v9 = sub_19570(v7, a2, v5, v6, v7, v8);
    if ( v9 >= 0 )
      return sub_9C70(v9, a4);
    v13 = __errno_location();
    sub_94A0(*v13);
    return 1;
  }
  else
  {
    v9 = sub_19570(v7, a2, v5 | 0x20100u, v6, v7, v8);
    if ( v9 >= 0 )
      return sub_9C70(v9, a4);
    v11 = *__errno_location();
    result = sub_7720(v11, a2, v12);
    if ( !(char)result )
    {
      sub_94A0(v11);
      return 1;
    }
  }
  return result;
}



// Function: grepdirent @ 0xa180
long long grepdirent(long long a1, long long a2, char a3)
{
  short v3; // ax
  unsigned char v4; // bl
  long long v6; // rbp
  long long v7; // rdx
  int v8; // edx
  unsigned int v9; // r14d
  unsigned int v10; // r13d
  long long v12; // rdi
  long long v13; // rdx
  int v14; // ecx
  int v15; // edi
  int v16; // ecx
  long long v17; // r12
  char *v18; // rax
  int *v19; // rax
  struct stat v20; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v21; // [rsp+98h] [rbp-40h]

  v21 = __readfsqword(0x28u);
  v3 = *(short *)(a2 + 104);
  v4 = a3 & (*(long long *)(a2 + 88) == 0);
  if ( v3 == 6 )
  {
    return 1;
  }
  else
  {
    v6 = a2;
    if ( !v4 )
    {
      v12 = a2 + 256;
      a2 = 0;
      v10 = sub_81E0(v12, 0, (v3 == 4) | (unsigned char)((unsigned short)(v3 - 1) <= 1u));
      if ( (char)v10 )
      {
        sub_17830(a1, v6, 4);
        return v10;
      }
      v3 = *(short *)(v6 + 104);
    }
    v7 = *(long long *)(v6 + 56);
    qword_2A890 = v7;
    if ( byte_2A88A && *(char *)(v7 + 1) )
      qword_2A890 = v7 + 2;
    v8 = *(int *)(a1 + 72);
    v9 = 1;
    if ( (v8 & 2) == 0 )
      v9 = (unsigned char)(v4 & v8);
    switch ( v3 )
    {
      case 1:
        if ( dword_2A018 == 3 )
          return 1;
        sub_17830(a1, v6, 4);
        return sub_A0F0(*(unsigned int *)(a1 + 44), *(long long *)(v6 + 48), v9, v4);
      case 2:
        if ( byte_2A904 )
          return 1;
        v17 = qword_2A890;
        v18 = dcgettext(0, "%s: warning: recursive directory loop", 5);
        v10 = 1;
        error(0, 0, v18, v17);
        return v10;
      case 3:
      case 11:
        v10 = sub_7500(v4);
        if ( !(char)v10 )
          return sub_A0F0(*(unsigned int *)(a1 + 44), *(long long *)(v6 + 48), v9, v4);
        v15 = *(int *)(v6 + 136);
        if ( v15 )
          goto LABEL_22;
        a2 = *(long long *)(v6 + 48);
        v16 = v9 << 8;
        BYTE1(v16) = v9 ^ 1;
        if ( fstatat(*(int *)(a1 + 44), (const char *)a2, &v20, v16) )
        {
          v19 = __errno_location();
          sub_94A0(*v19);
        }
        else
        {
          LOWORD(v15) = v20.st_mode;
LABEL_22:
          if ( !(unsigned char)sub_74D0(v15, a2, v13, v14) )
            return sub_A0F0(*(unsigned int *)(a1 + 44), *(long long *)(v6 + 48), v9, v4);
        }
        break;
      case 4:
      case 7:
      case 10:
        v10 = 1;
        sub_94A0(*(int *)(v6 + 64));
        return v10;
      case 8:
      case 13:
        return sub_A0F0(*(unsigned int *)(a1 + 44), *(long long *)(v6 + 48), v9, v4);
      case 12:
      case 14:
        return 1;
      default:
        abort();
    }
  }
  return v10;
}



// Function: grep_command_line_arg @ 0xa3f0
long long grep_command_line_arg(const char *a1)
{
  if ( !strcmp(a1, "-") )
  {
    qword_2A890 = qword_2A7D0;
    return sub_9C70(0, 1u);
  }
  else
  {
    qword_2A890 = (long long)a1;
    return sub_A0F0(4294967196LL, (long long)a1, 1, 1u);
  }
}



// Function: usage @ 0xa440
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
  char *v29; // rsi
  long long v30; // rdx

  v1 = sub_179E0();
  v2 = dcgettext(0, "Usage: %s [OPTION]... PATTERNS [FILE]...\n", 5);
  if ( status )
  {
    __fprintf_chk(stderr, 1, v2, v1);
    v3 = sub_179E0();
    v4 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v4, v3);
  }
  else
  {
    __printf_chk(1, v2, v1);
    v5 = dcgettext(0, "Search for PATTERNS in each FILE.\n", 5);
    __printf_chk(1, v5, v6);
    v7 = sub_179E0();
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
    sub_1D030();
  }
  exit(status);
}



// Function: fgrep_to_grep_pattern @ 0xa6b0
unsigned long long fgrep_to_grep_pattern(void **a1, size_t *a2)
{
  size_t v2; // r15
  char *v3; // r12
  long long v4; // rdi
  long long v5; // rax
  char *v6; // r13
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
  v5 = sub_1D210(v4, 2);
  v15 = v5;
  if ( !v2 )
  {
    v6 = (char *)v5;
    goto LABEL_13;
  }
  v6 = (char *)v5;
  do
  {
    while ( 1 )
    {
      v11 = sub_C9F0(v3, v2, &v17);
      if ( v11 != -1 )
        break;
      v17 = 0;
LABEL_8:
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
        goto LABEL_12;
    }
    if ( v11 == 1 )
      goto LABEL_8;
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
LABEL_12:
  v2 = (size_t)&v6[-v15];
LABEL_13:
  *v6 = 10;
  free(*a1);
  *a1 = (void *)v15;
  *a2 = v2;
  return v18 - __readfsqword(0x28u);
}



// Function: Fcompile @ 0xa820
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
  long long v15[9]; // [rsp+20h] [rbp-48h] BYREF

  v2 = s;
  v3 = (char *)s + (long long)a2;
  v4 = 0;
  v15[1] = __readfsqword(0x28u);
  v15[0] = 0;
  v5 = (struct obstack *)sub_CA70(1);
  do
  {
    v8 = (char *)rawmemchr(v2, 10);
    v6 = v8 - (char *)v2;
    if ( byte_2A685 )
    {
      v9 = byte_2A684;
      if ( v2 > s && byte_2A684 == 10 )
      {
        v2 = (struct obstack *)((char *)v2 - 1);
      }
      else
      {
        if ( (long long)(v6 + 1) >= v15[0] )
        {
          free(v4);
          v15[0] = v8 - (char *)v2;
          v10 = (char *)sub_1D2B0(0, v15, 2, -1, 1);
          v9 = byte_2A684;
          v4 = (struct obstack *)v10;
          *v10 = byte_2A684;
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
    sub_BAB0(v5, v7, v6);
  }
  while ( v8 + 1 <= v3 );
  free(v4);
  v11 = sub_BF90(v5);
  sub_BFA0(v5);
  result = (struct obstack **)sub_1D0F0(40);
  *result = v5;
  result[2] = s;
  result[1] = (struct obstack *)v11;
  result[3] = a2;
  result[4] = 0;
  return result;
}



// Function: Fexecute @ 0xa9c0
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
  v28 = byte_2A684;
  v7 = (unsigned char)byte_2A685;
  v27 = a5 != 0;
  v26 = ((byte_2A685 | byte_2A182) ^ 1) & byte_2A180[0];
  v29 = a5 != 0;
  v8 = a2;
  if ( a5 )
    v8 = a5;
  v24 = ((unsigned char)byte_2A685
       ^ 1)
      & (unsigned char)((a5 != 0) | byte_2A686 | ((byte_2A685 | byte_2A182) ^ 1) & byte_2A180[0]);
  s = v8;
  if ( v5 >= v8 )
  {
    while ( 1 )
    {
      v9 = sub_C870(v6, &v8[-v7], v7 + v5 - v8, v33, v24);
      if ( v9 < 0 )
        return -1;
      v10 = &v8[v9];
      v32 = 0;
      v11 = (char *)(v34 - 2 * (unsigned char)byte_2A685);
      if ( !v26 || !sub_CAF0(&s, &v32, v10, v5) )
        break;
      v8 = (char *)s;
      if ( v5 < s )
        return -1;
LABEL_28:
      v7 = (unsigned char)byte_2A685;
    }
    if ( byte_2A686 != 1 && v27 )
    {
      v19 = v11;
      goto LABEL_36;
    }
    if ( byte_2A685 )
    {
      v19 = &v11[a5 == 0];
      goto LABEL_36;
    }
    if ( !byte_2A686 )
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
        if ( !sub_CC90(&v10[-v32], v5) )
        {
LABEL_18:
          while ( 1 )
          {
            v14 = &v11[(long long)v10];
            if ( !sub_CC90(&v11[(long long)v10], v5) )
              break;
            if ( !a5 && !byte_2A180[0] )
            {
              if ( !a1[4] )
              {
                sub_A6B0((void **)a1 + 2, (size_t *)a1 + 3);
                a1[4] = sub_6690((char *)a1[2], a1[3], 0x10A46u, v29);
              }
              v15 = v5;
              if ( v5 > v14 )
                v15 = (char *)rawmemchr(v14, v28) + 1;
              v30 = v15;
              if ( sub_6C90((long long *)a1[4], v10, v15 - v10, a4, 0) < 0 )
              {
                v10 = v30 - 1;
                goto LABEL_27;
              }
              v20 = v28;
              v5 = v30;
              goto LABEL_41;
            }
            if ( !v11 || sub_C870(v6, v10, v11 - 1, v35, 1) )
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
    if ( !sub_CCA0(v12, v10, v5) )
      goto LABEL_18;
LABEL_27:
    s = &v10[sub_CC80(v10, v5)];
    v8 = (char *)s + 1;
    if ( v5 < (char *)s + 1 )
      return -1;
    goto LABEL_28;
  }
  return -1;
}



// Function: U @ 0xadc0
long long U(char a1)
{
  return sub_C9D0((unsigned int)a1);
}



// Function: tr @ 0xadd0
long long tr(long long a1, unsigned int a2)
{
  if ( a1 )
    return *(unsigned char *)(a1 + (unsigned char)sub_ADC0(a2));
  else
    return a2;
}



// Function: enqueue @ 0xae00
long long enqueue(long long *a1, long long a2)
{
  long long result; // rax

  if ( a1 )
  {
    enqueue(*a1);
    enqueue(a1[1]);
    result = a1[2];
    *(long long *)(*(long long *)a2 + 24LL) = result;
    *(long long *)a2 = result;
  }
  return result;
}



// Function: treefails @ 0xae50
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



// Function: treedelta @ 0xaf10
long long treedelta(long long a1, long long a2, long long a3)
{
  char *v4; // rbx
  long long result; // rax

  if ( a1 )
  {
    treedelta(*(long long *)a1);
    treedelta(*(long long *)(a1 + 8));
    v4 = (char *)(*(unsigned char *)(a1 + 24) + a3);
    result = (unsigned char)*v4;
    if ( result > a2 )
      *v4 = a2;
  }
  return result;
}



// Function: hasevery @ 0xaf60
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



// Function: treenext @ 0xafe0
long long treenext(long long a1, long long a2)
{
  long long result; // rax

  if ( a1 )
  {
    treenext(*(long long *)a1);
    treenext(*(long long *)(a1 + 8));
    result = *(unsigned char *)(a1 + 24);
    *(long long *)(a2 + 8 * result) = *(long long *)(a1 + 16);
  }
  return result;
}



// Function: bm_delta2_search @ 0xb030
long long bm_delta2_search(
        unsigned long long *a1,
        unsigned long long a2,
        long long a3,
        long long a4,
        long long a5,
        char a6,
        char a7,
        long long a8,
        long long a9)
{
  long long v9; // r13
  unsigned long long v11; // rbx
  long long v12; // rcx
  long long v13; // r14
  char v14; // r12
  char v15; // r12
  char v16; // r12
  long long result; // rax
  long long v19; // [rsp+10h] [rbp-58h]

  v9 = a4;
  v11 = *a1;
  v19 = 0;
  while ( (unsigned char)sub_ADD0(a5, *(char *)(v11 - 2)) != a7 )
  {
    v12 = 0;
    v13 = 2;
LABEL_3:
    v9 = *(long long *)(*(long long *)(a9 + 2424) + v12);
    v11 += v9;
    if ( v11 > a2 )
      goto LABEL_17;
    v14 = *(char *)(v11 - 1);
    if ( (unsigned char)sub_ADD0(a5, v14) != a6 )
    {
      if ( a8 )
        v11 += *(unsigned char *)(a8 + (unsigned char)sub_ADC0(v14));
LABEL_17:
      result = 0;
      goto LABEL_18;
    }
    v19 = v13 - 1;
  }
  v13 = 2;
  while ( v9 >= ++v13 )
  {
    v15 = sub_ADD0(a5, *(char *)(v11 - v13));
    if ( v15 != (unsigned char)sub_ADD0(a5, *(char *)(a3 - v13)) )
      goto LABEL_14;
  }
  v13 = v9 + v19 + 1;
  if ( a4 >= v13 )
  {
    while ( 1 )
    {
      v16 = sub_ADD0(a5, *(char *)(v11 - v13));
      if ( v16 != (unsigned char)sub_ADD0(a5, *(char *)(a3 - v13)) )
        break;
      if ( a4 < ++v13 )
        goto LABEL_19;
    }
LABEL_14:
    v12 = 8 * v13 - 16;
    goto LABEL_3;
  }
LABEL_19:
  v11 -= a4;
  result = 1;
LABEL_18:
  *a1 = v11;
  return result;
}



// Function: memchr_kwset @ 0xb1c0
long long memchr_kwset(char *a1, long long a2, long long a3)
{
  char *v3; // r12
  char *v4; // rbp
  int v6; // r13d
  long long result; // rax

  v3 = &a1[a2];
  v4 = a1;
  v6 = *(int *)(a3 + 2444);
  if ( v6 >= 0 )
  {
    while ( v4 < v3 )
    {
      if ( *(long long *)(a3 + 8LL * (unsigned char)sub_ADC0(*v4) + 368) )
        return (long long)v4;
      if ( ((unsigned char)++v4 & 0xF) == 0 )
        return sub_190F0(v4, *(unsigned int *)(a3 + 2440), (unsigned int)v6, v3 - v4);
    }
    return 0;
  }
  if ( a1 >= v3 )
    return 0;
  while ( 1 )
  {
    result = *(long long *)(a3 + 8LL * (unsigned char)sub_ADC0(*v4) + 368);
    if ( result )
      break;
    if ( v3 == ++v4 )
      return result;
  }
  return (long long)v4;
}



// Function: acexec_trans @ 0xb290
long long acexec_trans(long long a1, char *a2, long long a3, long long *a4, char a5)
{
  long long *v5; // r15
  char *v7; // r14
  long long v8; // r12
  long long v9; // rdx
  char *v10; // rbp
  char *v11; // rax
  char *v12; // rbx
  unsigned int v13; // esi
  unsigned char v14; // al
  long long i; // rdx
  long long v16; // rcx
  long long *v18; // r13
  long long v19; // rdx
  char *v20; // r14
  unsigned char v21; // al
  long long v22; // rdx
  long long v23; // rcx
  long long v24; // rax
  unsigned int v25; // esi
  long long *v26; // rax
  int v30; // [rsp+1Ch] [rbp-3Ch]

  if ( *(long long *)(a1 + 104) > a3 )
    return -1;
  v5 = *(long long **)(a1 + 96);
  v7 = a2;
  v8 = *(long long *)(a1 + 2432);
  v9 = *v5;
  v10 = &a2[a3];
  if ( *v5 )
  {
    v12 = a2;
    goto LABEL_19;
  }
  v30 = *(int *)(a1 + 2440);
  while ( 2 )
  {
    if ( v30 < 0 )
    {
      while ( 1 )
      {
        v25 = *v7++;
        v5 = *(long long **)(a1 + 8LL * (unsigned char)sub_ADD0(v8, v25) + 368);
        if ( v5 )
          break;
        if ( v10 <= v7 )
          return -1;
      }
      goto LABEL_7;
    }
    v11 = (char *)sub_B1C0(v7, v10 - v7, a1);
    if ( !v11 )
      return -1;
    v7 = v11 + 1;
    v5 = *(long long **)(a1 + 8LL * (unsigned char)sub_ADD0(v8, *v11) + 368);
LABEL_7:
    v9 = *v5;
    if ( !*v5 )
    {
      if ( v10 > v7 )
      {
LABEL_9:
        v12 = v7;
        v13 = *v7++;
        v14 = sub_ADD0(v8, v13);
        for ( i = v5[1]; ; i = v5[1] )
        {
          do
          {
            if ( *(char *)(i + 24) == v14 )
            {
              v5 = *(long long **)(i + 16);
              goto LABEL_15;
            }
            v16 = *(long long *)i;
            if ( *(char *)(i + 24) <= v14 )
              v16 = *(long long *)(i + 8);
            i = v16;
          }
          while ( v16 );
          v5 = (long long *)v5[4];
          if ( !v5 )
            break;
          v9 = *v5;
          if ( *v5 )
            goto LABEL_19;
        }
        v5 = *(long long **)(a1 + 8LL * v14 + 368);
        if ( !v5 )
        {
          if ( v10 > v7 )
            continue;
          return -1;
        }
LABEL_15:
        v9 = *v5;
        if ( *v5 )
          break;
        if ( v10 != v7 )
          goto LABEL_9;
      }
      return -1;
    }
    break;
  }
  v12 = v7;
LABEL_19:
  v18 = v5;
  if ( v9 < 0 )
  {
    do
      v18 = (long long *)v18[4];
    while ( *v18 < 0 );
  }
  v19 = v18[5];
  v20 = &v12[-v19];
  if ( a5 && v12 < v10 )
  {
    do
    {
      v21 = sub_ADD0(v8, *v12);
      while ( 1 )
      {
        v22 = v5[1];
        if ( v22 )
          break;
LABEL_37:
        v5 = (long long *)v5[4];
        v19 = v18[5];
        if ( !v5 || v5[5] < v19 )
          goto LABEL_33;
      }
      while ( *(char *)(v22 + 24) != v21 )
      {
        v23 = *(long long *)v22;
        if ( *(char *)(v22 + 24) <= v21 )
          v23 = *(long long *)(v22 + 8);
        v22 = v23;
        if ( !v23 )
          goto LABEL_37;
      }
      v5 = *(long long **)(v22 + 16);
      ++v12;
      if ( *v5 )
      {
        v26 = *(long long **)(v22 + 16);
        if ( *v5 < 0 )
        {
          do
            v26 = (long long *)v26[4];
          while ( *v26 < 0 );
        }
        if ( &v12[-v26[5]] <= v20 )
        {
          v20 = &v12[-v26[5]];
          v18 = v26;
        }
      }
    }
    while ( v10 != v12 );
    v19 = v18[5];
  }
LABEL_33:
  v24 = *v18 >> 1;
  a4[2] = v19;
  *a4 = v24;
  a4[1] = v20 - a2;
  return v20 - a2;
}



// Function: acexec @ 0xb530
long long acexec(long long a1, char *a2, long long a3, long long *a4, char a5)
{
  return sub_B290(a1, a2, a3, a4, a5);
}



// Function: bmexec_trans @ 0xb540
long long bmexec_trans(long long a1, char *a2, long long a3)
{
  signed long long v3; // r12
  long long v4; // r13
  long long v5; // rbx
  unsigned long long v6; // r15
  long long v7; // rax
  unsigned long long v8; // rbp
  signed long long v9; // r14
  char v10; // di
  unsigned char v11; // al
  unsigned long long v13; // r14
  long long v14; // rax
  unsigned long long v15; // rbp
  long long v16; // rcx
  long long v17; // rax
  char v18; // di
  long long v19; // rax
  char v20; // di
  long long v21; // rcx
  long long v22; // rdx
  long long v23; // rax
  char v24; // di
  long long v25; // rax
  char v26; // di
  long long v27; // rcx
  long long v28; // rdx
  long long v29; // rax
  unsigned long long v30; // rdx
  long long v31; // rax
  long long v32; // [rsp+0h] [rbp-88h]
  long long v33; // [rsp+8h] [rbp-80h]
  char v34; // [rsp+13h] [rbp-75h]
  char v35; // [rsp+14h] [rbp-74h]
  long long v36; // [rsp+18h] [rbp-70h]
  long long v37; // [rsp+18h] [rbp-70h]
  long long v38; // [rsp+18h] [rbp-70h]
  long long v39; // [rsp+18h] [rbp-70h]
  long long v40; // [rsp+18h] [rbp-70h]
  long long v41; // [rsp+18h] [rbp-70h]
  long long v42; // [rsp+18h] [rbp-70h]
  char *v45; // [rsp+38h] [rbp-50h]
  unsigned long long v46[9]; // [rsp+40h] [rbp-48h] BYREF

  v3 = *(long long *)(a1 + 104);
  v46[1] = __readfsqword(0x28u);
  if ( v3 )
  {
    v4 = a1;
    v32 = *(long long *)(a1 + 2432);
    if ( a3 < v3 )
      return -1;
    if ( v3 == 1 )
    {
      v31 = sub_B1C0(a2, a3, a1);
      if ( v31 )
        return v31 - (long long)a2;
      return -1;
    }
    v5 = a1 + 112;
    v6 = (unsigned long long)&a2[v3];
    v7 = v3 + *(long long *)(a1 + 2416);
    v46[0] = (unsigned long long)&a2[v3];
    v33 = v7;
    v35 = *(char *)(a1 + 2440);
    v34 = *(char *)(a1 + 2448);
    if ( 12 * v3 >= a3 || !is_mul_ok(0xCu, v3) )
    {
      v8 = (unsigned long long)&a2[a3];
      goto LABEL_7;
    }
    v13 = (unsigned long long)&a2[a3 - 11 * v3];
    v45 = &a2[a3];
    v8 = (unsigned long long)&a2[a3];
    if ( v6 > v13 )
    {
LABEL_7:
      v9 = *(unsigned char *)(v4 + (unsigned char)sub_ADC0(*(char *)(v6 - 1)) + 112);
      while ( v9 <= (long long)(v8 - v6) )
      {
        while ( 1 )
        {
          v6 += v9;
          v10 = *(char *)(v6 - 1);
          v46[0] = v6;
          v11 = sub_ADC0(v10);
          v9 = *(unsigned char *)(v5 + v11);
          if ( *(char *)(v5 + v11) )
            break;
          if ( (unsigned char)sub_B030(v46, v8, v33, v3, v32, v35, v34, 0, v4) )
            return v46[0] - (long long)a2;
          v6 = v46[0];
          if ( v9 > (long long)(v8 - v46[0]) )
            return -1;
        }
      }
      return -1;
    }
    while ( 1 )
    {
      v15 = v6 + *(unsigned char *)(v5 + (unsigned char)sub_ADC0(*(char *)(v6 - 1)));
      v16 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(*(char *)(v15 - 1)));
      v46[0] = v15 + v16;
      if ( !(char)v16 )
        goto LABEL_19;
      v36 = v15 + v16;
      v17 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(*(char *)(v15 + v16 - 1)));
      v18 = *(char *)(v17 + v36 - 1);
      v37 = v17 + v36;
      v19 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(v18));
      v20 = *(char *)(v19 + v37 - 1);
      v38 = v19 + v37;
      v21 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(v20));
      v22 = v21 + v38;
      v46[0] = v21 + v38;
      if ( !(char)v21 )
        goto LABEL_19;
      v39 = v21 + v38;
      v23 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(*(char *)(v22 - 1)));
      v24 = *(char *)(v23 + v39 - 1);
      v40 = v23 + v39;
      v25 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(v24));
      v26 = *(char *)(v25 + v40 - 1);
      v41 = v25 + v40;
      v27 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(v26));
      v28 = v27 + v41;
      v46[0] = v27 + v41;
      if ( !(char)v27 )
        goto LABEL_19;
      v42 = v27 + v41;
      v29 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(*(char *)(v28 - 1)));
      v30 = *(unsigned char *)(v5 + (unsigned char)sub_ADC0(*(char *)(v29 + v42 - 1))) + v29 + v42;
      v46[0] = v30;
      if ( (long long)(v30 - v6) <= 127 )
        break;
      v6 = v30;
LABEL_21:
      if ( v13 < v6 )
      {
        v4 = a1;
        v8 = (unsigned long long)&a2[a3];
        goto LABEL_7;
      }
    }
    v8 = (unsigned long long)v45;
    v14 = sub_B1C0((char *)(v30 - 1), (long long)&v45[-v30 + 1], a1);
    if ( !v14 )
      return -1;
    v6 = v14 + 1;
    v46[0] = v14 + 1;
    if ( v14 + 1 >= v13 )
    {
      v4 = a1;
      goto LABEL_7;
    }
LABEL_19:
    if ( (unsigned char)sub_B030(v46, v13, v33, v3, v32, v35, v34, v5, a1) )
      return v46[0] - (long long)a2;
    v6 = v46[0];
    goto LABEL_21;
  }
  return v3;
}



// Function: bmexec @ 0xb930
long long bmexec(long long a1, char *a2, long long a3, long long *a4)
{
  long long result; // rax
  long long v6; // rdx

  result = sub_B540(a1, a2, a3);
  v6 = *(long long *)(a1 + 104);
  *a4 = 0;
  a4[1] = result;
  a4[2] = v6;
  return result;
}



// Function: kwsalloc @ 0xb960
long long kwsalloc(long long a1)
{
  long long v1; // r12
  long long v2; // rcx
  long long v3; // rax
  long long *v4; // rdx
  long long v5; // rax
  long long v6; // rsi
  long long v7; // rax

  v1 = sub_1D0F0(2464);
  obstack_begin(v1, 0, 0, sub_1D0F0, &free);
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
  *(long long *)(v1 + 2456) = sub_B530;
  *(long long *)(v1 + 2416) = 0;
  return v1;
}



// Function: kwsincr @ 0xbab0
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
  if ( v5 == sub_B930 )
    v4 = &a2[a3];
  if ( a3 )
  {
    v7 = a3 - 1;
    v8 = a1[304];
    v9 = a1;
    while ( 1 )
    {
      if ( v5 == sub_B930 )
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



// Function: kwswords @ 0xbf90
long long kwswords(long long a1)
{
  return *(long long *)(a1 + 88);
}



// Function: kwsprep @ 0xbfa0
unsigned long long kwsprep(struct obstack *obstack)
{
  struct _obstack_chunk *(*chunkfun)(void *, long long); // rbx
  char *p_next_free; // r12
  char *v4; // rax
  long long v5; // rdi
  struct _obstack_chunk *v6; // r8
  struct _obstack_chunk **v7; // r11
  long long v8; // r8
  long long v9; // r9
  long long v10; // r8
  unsigned char v11; // cl
  long long *v12; // r8
  long long v13; // r10
  char *v14; // rax
  long long v15; // r9
  long long v16; // r14
  long long v17; // r8
  long long *v18; // r9
  long long v19; // rax
  long long v20; // rax
  char **p_object_base; // rax
  int v22; // r15d
  long long v23; // r14
  unsigned char v24; // al
  int v25; // edi
  char *v26; // rax
  int v27; // r13d
  long long v28; // r13
  struct _obstack_chunk *limit; // rax
  char *prev; // rdx
  struct _obstack_chunk *v32; // rcx
  char *v33; // rdx
  char *v34; // rax
  char *v35; // rsi
  char *v36; // r13
  long long v37; // rcx
  char *v38; // rax
  long long v39; // rdi
  char *v40; // rax
  struct _obstack_chunk *v41; // rdx
  char *v42; // rax
  char *v44; // rcx
  char *v45; // rdx
  char *v46; // rax
  unsigned long long v47; // r13
  char *v48; // rdi
  char *v49; // rax
  long long v50; // rsi
  char *v51; // rax
  struct _obstack_chunk *v52; // rax
  char *v53; // rdx
  long long v54; // rax
  long long v55; // rcx
  struct _obstack_chunk *chunk; // r8
  struct _obstack_chunk **v57; // r11
  struct _obstack_chunk *i; // rcx
  long long v59; // rcx
  char *chunk_limit; // rdx
  char *next_free; // rax
  char *object_base; // r9
  char *v63; // r14
  long long tempint; // rcx
  char *v65; // rax
  long long v66; // rsi
  char *v67; // rax
  long long v68; // rax
  long long *v69; // rax
  char *v70; // rdx
  unsigned char *v71; // rsi
  void *v72; // r14
  char *v73; // rax
  long long v74; // rdi
  int v75; // [rsp+Ch] [rbp-96Ch]
  char **v76; // [rsp+10h] [rbp-968h]
  long long chunk_size; // [rsp+18h] [rbp-960h]
  struct _obstack_chunk *v78; // [rsp+28h] [rbp-950h] BYREF
  char v79; // [rsp+30h] [rbp-948h] BYREF
  char v80; // [rsp+830h] [rbp-148h] BYREF
  unsigned long long v81; // [rsp+938h] [rbp-40h]

  chunkfun = obstack[27].chunkfun;
  v81 = __readfsqword(0x28u);
  p_next_free = &v80;
  if ( !chunkfun )
    p_next_free = (char *)&obstack[1].next_free;
  chunk_size = obstack[1].chunk_size;
  if ( chunk_size == 1 )
  {
    chunk = obstack[1].chunk;
    v57 = &v78;
    v78 = chunk;
    for ( i = chunk; i; i = *(struct _obstack_chunk **)(v59 + 24) )
      sub_AE00(&i->prev->limit, (long long)v57);
    chunk_limit = obstack->chunk_limit;
    next_free = obstack->next_free;
    object_base = obstack[1].object_base;
    v63 = object_base;
    if ( (unsigned long long)object_base > chunk_limit - next_free )
    {
      obstack_newchunk(obstack, obstack[1].object_base);
      tempint = (long long)obstack->object_base;
      chunk_limit = obstack->chunk_limit;
      chunk = obstack[1].chunk;
      v65 = &v63[(unsigned long long)obstack->next_free];
      object_base = obstack[1].object_base;
      if ( v65 != (char *)tempint )
        goto LABEL_73;
    }
    else
    {
      tempint = (long long)obstack->object_base;
      v65 = &next_free[(long long)object_base];
      if ( v65 != (char *)tempint )
        goto LABEL_73;
    }
    *((char *)obstack + 80) |= 2u;
LABEL_73:
    v66 = *(long long *)&obstack->alignment_mask;
    obstack[27].temp.tempint = tempint;
    v67 = (char *)(~v66 & (unsigned long long)&v65[v66]);
    if ( (char *)(v67 - (char *)obstack->chunk) <= (char *)(chunk_limit - (char *)obstack->chunk) )
      chunk_limit = v67;
    obstack->next_free = chunk_limit;
    obstack->object_base = chunk_limit;
    if ( (long long)object_base > 0 )
    {
      v68 = 0;
      while ( 1 )
      {
        *(char *)(tempint + v68++) = chunk->prev[1].limit;
        chunk = (struct _obstack_chunk *)chunk[1].limit;
        if ( (long long)obstack[1].object_base <= v68 )
          break;
        tempint = obstack[27].temp.tempint;
      }
    }
    v69 = (long long *)sub_B960((long long)obstack[27].chunkfun);
    v70 = obstack[1].object_base;
    v71 = (unsigned char *)obstack[27].temp.tempint;
    v72 = v69;
    v69[307] = sub_B930;
    sub_BAB0(v69, v71, (long long)v70);
    obstack_free(obstack, 0);
    qmemcpy(obstack, v72, 0x9A0u);
    free(v72);
    v73 = obstack[1].object_base;
    if ( (long long)v73 > 255 )
      LOBYTE(v73) = -1;
    *(long long *)p_next_free = 0x101010101010101LL * (unsigned char)v73;
    v74 = (__PAIR128__(0x101010101010101LL, 0x101010101010101LL) * (unsigned char)v73) >> 64;
    *((long long *)p_next_free + 1) = v74;
    *((long long *)p_next_free + 3) = v74;
    *((long long *)p_next_free + 2) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 5) = v74;
    *((long long *)p_next_free + 7) = v74;
    *((long long *)p_next_free + 9) = v74;
    *((long long *)p_next_free + 4) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 11) = v74;
    *((long long *)p_next_free + 13) = v74;
    *((long long *)p_next_free + 15) = v74;
    *((long long *)p_next_free + 6) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 17) = v74;
    *((long long *)p_next_free + 19) = v74;
    *((long long *)p_next_free + 21) = v74;
    *((long long *)p_next_free + 8) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 23) = v74;
    *((long long *)p_next_free + 25) = v74;
    *((long long *)p_next_free + 27) = v74;
    *((long long *)p_next_free + 10) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 29) = v74;
    *((long long *)p_next_free + 12) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 14) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 16) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 18) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 20) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 22) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 24) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 26) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 28) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 30) = 0x101010101010101LL * (unsigned char)v73;
    *((long long *)p_next_free + 31) = v74;
    limit = obstack[1].chunk;
    v78 = limit;
    v6 = limit;
    if ( !limit )
    {
LABEL_44:
      while ( 1 )
      {
        limit = (struct _obstack_chunk *)limit[1].limit;
        if ( !limit )
          goto LABEL_21;
        while ( 1 )
        {
          prev = (char *)limit[2].prev;
          v32 = *(struct _obstack_chunk **)(*(long long *)limit->contents + 56LL);
          if ( (long long)prev > (long long)v32 )
          {
            limit[2].prev = v32;
            prev = (char *)v32;
          }
          if ( (long long)limit[2].limit <= (long long)prev )
            break;
          limit[2].limit = prev;
          limit = (struct _obstack_chunk *)limit[1].limit;
          if ( !limit )
            goto LABEL_21;
        }
      }
    }
LABEL_7:
    v7 = &v78;
    do
    {
      while ( 1 )
      {
        sub_AE00(&v6->prev->limit, (long long)v7);
        sub_AF10(v9, *(long long *)(v8 + 40), (long long)p_next_free);
        sub_AE50(*(long long *)(v10 + 8), *(long long *)(v10 + 32), (long long)obstack[1].chunk, v11);
        if ( chunk_size == 1 )
        {
          v14 = obstack[1].object_base;
          v15 = v12[4];
          v12[6] = (long long)v14;
          v12[7] = (long long)v14;
          if ( v15 )
            break;
        }
        v6 = (struct _obstack_chunk *)v12[3];
        if ( !v6 )
          goto LABEL_20;
      }
      v16 = *v12;
      do
      {
        if ( !(unsigned char)sub_AF60(*(long long **)(v15 + 8), v13) )
        {
          v19 = *(long long *)(v17 + 40) - v18[5];
          if ( v19 < v18[6] )
            v18[6] = v19;
        }
        if ( v16 )
        {
          v20 = *(long long *)(v17 + 40) - v18[5];
          if ( v18[7] > v20 )
            v18[7] = v20;
        }
        v15 = v18[4];
      }
      while ( v15 );
      v6 = *(struct _obstack_chunk **)(v17 + 24);
    }
    while ( v6 );
LABEL_20:
    if ( chunk_size != 1 )
      goto LABEL_21;
    limit = obstack[1].chunk;
    goto LABEL_44;
  }
  v4 = obstack[1].object_base;
  if ( (long long)v4 > 255 )
    LOBYTE(v4) = -1;
  *(long long *)p_next_free = 0x101010101010101LL * (unsigned char)v4;
  v5 = (__PAIR128__(0x101010101010101LL, 0x101010101010101LL) * (unsigned char)v4) >> 64;
  *((long long *)p_next_free + 1) = v5;
  *((long long *)p_next_free + 3) = v5;
  *((long long *)p_next_free + 2) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 5) = v5;
  *((long long *)p_next_free + 7) = v5;
  *((long long *)p_next_free + 9) = v5;
  *((long long *)p_next_free + 4) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 11) = v5;
  *((long long *)p_next_free + 13) = v5;
  *((long long *)p_next_free + 15) = v5;
  *((long long *)p_next_free + 6) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 17) = v5;
  *((long long *)p_next_free + 19) = v5;
  *((long long *)p_next_free + 21) = v5;
  *((long long *)p_next_free + 8) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 23) = v5;
  *((long long *)p_next_free + 25) = v5;
  *((long long *)p_next_free + 27) = v5;
  *((long long *)p_next_free + 10) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 29) = v5;
  *((long long *)p_next_free + 12) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 14) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 16) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 18) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 20) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 22) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 24) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 26) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 28) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 30) = 0x101010101010101LL * (unsigned char)v4;
  *((long long *)p_next_free + 31) = v5;
  v6 = obstack[1].chunk;
  v78 = v6;
  if ( v6 )
    goto LABEL_7;
LABEL_21:
  p_object_base = &obstack[4].object_base;
  v22 = -2;
  if ( chunkfun )
    p_object_base = (char **)&v79;
  v23 = 0;
  v76 = p_object_base;
  *p_object_base = 0;
  p_object_base[255] = 0;
  memset(
    (void *)((unsigned long long)(p_object_base + 1) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)p_object_base - (((int)p_object_base + 8) & 0xFFFFFFF8) + 2048) >> 3));
  sub_AFE0((long long)obstack[1].chunk->prev, (long long)p_object_base);
  v75 = -1;
  do
  {
    while ( 1 )
    {
      v27 = v23;
      if ( chunkfun )
      {
        v24 = sub_ADC0(*((char *)chunkfun + v23));
        v25 = v24;
        v26 = v76[v24];
        *((long long *)&obstack[4].object_base + v23) = v26;
      }
      else
      {
        v26 = (char *)*((long long *)&obstack[4].object_base + v23);
        v25 = v23;
      }
      if ( v26 )
        break;
LABEL_31:
      if ( ++v23 == 256 )
        goto LABEL_35;
    }
    if ( v22 != -2 )
    {
      if ( v25 == v22 )
      {
        if ( v22 != v75 )
          v27 = -1;
        v75 = v27;
      }
      else if ( v25 == (int)v23 && v22 == v75 )
      {
        v75 = v23;
      }
      else
      {
        v22 = -1;
      }
      goto LABEL_31;
    }
    ++v23;
    v75 = v27;
    v22 = v25;
  }
  while ( v23 != 256 );
LABEL_35:
  LODWORD(obstack[27].freefun) = v22;
  HIDWORD(obstack[27].freefun) = v75;
  if ( chunk_size != 1 )
    goto LABEL_36;
  v33 = obstack->chunk_limit;
  v34 = obstack->next_free;
  v35 = obstack[1].object_base;
  v36 = v35;
  if ( (unsigned long long)v35 > v33 - v34 )
  {
    obstack_newchunk(obstack, v35);
    v37 = (long long)obstack->object_base;
    v33 = obstack->chunk_limit;
    v35 = obstack[1].object_base;
    v38 = &v36[(unsigned long long)obstack->next_free];
    if ( v38 != (char *)v37 )
      goto LABEL_52;
LABEL_84:
    *((char *)obstack + 80) |= 2u;
    goto LABEL_52;
  }
  v37 = (long long)obstack->object_base;
  v38 = &v34[(long long)v35];
  if ( v38 == (char *)v37 )
    goto LABEL_84;
LABEL_52:
  v39 = *(long long *)&obstack->alignment_mask;
  obstack[27].temp.tempint = v37;
  v40 = (char *)(~v39 & (unsigned long long)&v38[v39]);
  if ( (char *)(v40 - (char *)obstack->chunk) <= (char *)(v33 - (char *)obstack->chunk) )
    v33 = v40;
  obstack->next_free = v33;
  obstack->object_base = v33;
  v41 = obstack[1].chunk;
  if ( (long long)v35 > 0 )
  {
    v42 = v35 - 1;
    while ( 1 )
    {
      v42[v37] = (char)v41->prev[1].limit;
      v41 = (struct _obstack_chunk *)v41[1].limit;
      if ( v42-- == 0 )
        break;
      v37 = obstack[27].temp.tempint;
    }
    v44 = obstack[1].object_base;
    if ( (long long)v44 > 1 )
    {
      v45 = obstack->chunk_limit;
      v46 = obstack->next_free;
      v47 = 8LL * (long long)v44 - 8;
      if ( v47 > v45 - v46 )
      {
        obstack_newchunk(obstack, 8LL * (long long)v44 - 8);
        v46 = obstack->next_free;
        v45 = obstack->chunk_limit;
        v44 = obstack[1].object_base;
      }
      v48 = obstack->object_base;
      v49 = &v46[v47];
      if ( v49 == v48 )
        *((char *)obstack + 80) |= 2u;
      v50 = *(long long *)&obstack->alignment_mask;
      *(long long *)&obstack[27].alignment_mask = v48;
      v51 = (char *)(~v50 & (unsigned long long)&v49[v50]);
      if ( (char *)(v51 - (char *)obstack->chunk) <= (char *)(v45 - (char *)obstack->chunk) )
        v45 = v51;
      v52 = obstack[1].chunk;
      obstack->next_free = v45;
      obstack->object_base = v45;
      v53 = v52[1].limit;
      if ( (long long)v44 > 1 )
      {
        v54 = 0;
        do
        {
          v55 = *((long long *)v53 + 6);
          v53 = (char *)*((long long *)v53 + 3);
          *(long long *)&v48[8 * v54] = v55;
          v44 = obstack[1].object_base;
          ++v54;
        }
        while ( (long long)(v44 - 1) > v54 );
      }
      LOBYTE(obstack[27].extra_arg) = sub_ADD0((long long)chunkfun, v44[obstack[27].temp.tempint - 2]);
    }
  }
LABEL_36:
  v28 = 0;
  if ( chunkfun )
  {
    do
    {
      *((char *)&obstack[1].next_free + v28) = p_next_free[(unsigned char)sub_ADC0(*((char *)chunkfun + v28))];
      ++v28;
    }
    while ( v28 != 256 );
  }
  return v81 - __readfsqword(0x28u);
}



// Function: kwsexec @ 0xc870
long long kwsexec(long long a1, long long a2, long long a3, long long a4, unsigned char a5)
{
  return (*(long long (**)(long long, long long, long long, long long, long long))(a1 + 2456))(a1, a2, a3, a4, a5);
}



// Function: wordchar @ 0xc8b0
bool wordchar(wint_t a1)
{
  return a1 == 95 || iswalnum(a1) != 0;
}



// Function: wordchars_count @ 0xc8e0
long long wordchars_count(long long a1, long long a2, char a3)
{
  long long v3; // r14
  long long v4; // r12
  long long v6; // rax
  long long v7; // r13
  wint_t v9; // [rsp+1Ch] [rbp-4Ch] BYREF
  long long v10[9]; // [rsp+20h] [rbp-48h] BYREF

  v3 = a2 - a1;
  v4 = 0;
  v10[1] = __readfsqword(0x28u);
  v10[0] = 0;
  while ( v3 > v4 )
  {
    v6 = *(unsigned char *)(a1 + v4);
    if ( byte_2A9C0[v6] )
    {
      ++v4;
      if ( !a3 )
        return v4;
    }
    else
    {
      if ( byte_2A180[v6 + 3] != -2 )
        return v4;
      v9 = 0;
      v7 = sub_18C40(&v9, a1 + v4, v3 - v4, v10);
      if ( !sub_C8B0(v9) )
        return v4;
      v4 += v7 + (v7 == 0);
      if ( !a3 )
        return v4;
    }
  }
  return v4;
}



// Function: wordinit @ 0xca30
bool wordinit()
{
  char *v0; // rbp
  char *v1; // rbx
  wint_t v2; // edi
  bool result; // al

  v0 = byte_2A9C0;
  v1 = (char *)&unk_2A284;
  do
  {
    v2 = *(int *)v1;
    v1 += 4;
    ++v0;
    result = sub_C8B0(v2);
    *(v0 - 1) = result;
  }
  while ( v1 != &byte_2A684 );
  return result;
}



// Function: kwsinit @ 0xca70
long long kwsinit(char a1)
{
  long long v2; // rbx
  long long v3; // rbp

  if ( !byte_2A687 || __ctype_get_mb_cur_max() != 1 && !a1 )
    return sub_B960(0);
  v2 = 0;
  v3 = sub_1D110(256);
  do
  {
    *(char *)(v3 + v2) = toupper(v2);
    ++v2;
  }
  while ( v2 != 256 );
  return sub_B960(v3);
}



// Function: mb_goback @ 0xcaf0
char *mb_goback(unsigned long long *a1, long long *a2, char *a3, long long a4)
{
  unsigned long long v5; // r13
  char *v7; // rdx
  long long v8; // rax
  char v10; // al
  long long v11; // rdx
  char *v12; // r12
  long long v13; // rax
  long long v14[9]; // [rsp+0h] [rbp-48h] BYREF

  v5 = *a1;
  v14[1] = __readfsqword(0x28u);
  if ( v5 >= (unsigned long long)a3 )
    return &a3[-v5];
  if ( !byte_2A182 )
  {
    v14[0] = 0;
    while ( 1 )
    {
      v8 = sub_C9F0(v5, a4 - v5, v14);
      if ( v8 >= 0 )
      {
        v7 = (char *)(v5 + v8);
        if ( (unsigned long long)a3 <= v5 + v8 )
          goto LABEL_8;
      }
      else
      {
        v14[0] = 0;
        v8 = 1;
        v7 = (char *)(v5 + 1);
        if ( (unsigned long long)a3 <= v5 + 1 )
        {
LABEL_8:
          if ( a2 )
            *a2 = v8;
          goto LABEL_10;
        }
      }
      v5 = (unsigned long long)v7;
    }
  }
  if ( (*a3 & 0xC0) != 0x80 )
    goto LABEL_13;
  v10 = *(a3 - 1);
  if ( (v10 & 0xC0) == 0x80 )
  {
    v10 = *(a3 - 2);
    if ( (v10 & 0xC0) == 0x80 )
    {
      v10 = *(a3 - 3);
      if ( (v10 & 0xC0) == 0x80 )
        goto LABEL_13;
      v11 = 3;
    }
    else
    {
      v11 = 2;
    }
  }
  else
  {
    v11 = 1;
  }
  if ( !((int)(unsigned char)~v10 >> (7 - v11)) )
  {
    v14[0] = 0;
    v12 = &a3[-v11];
    v13 = sub_C9E0(v12, a4 - (long long)v12, v14);
    v7 = a3;
    if ( v13 >= 0 )
    {
      v7 = &v12[v13];
      v5 = (unsigned long long)v12;
    }
LABEL_10:
    *a1 = (unsigned long long)v7;
    if ( v7 == a3 )
      return 0;
    return &a3[-v5];
  }
LABEL_13:
  *a1 = (unsigned long long)a3;
  return 0;
}



// Function: wordchars_size @ 0xcc80
long long wordchars_size(long long a1, long long a2)
{
  return sub_C8E0(a1, a2, 1);
}



// Function: wordchar_next @ 0xcc90
long long wordchar_next(long long a1, long long a2)
{
  return sub_C8E0(a1, a2, 0);
}



// Function: wordchar_prev @ 0xcca0
long long wordchar_prev(unsigned long long a1, long long a2, long long a3)
{
  long long v4; // rdx
  char *v6; // rax
  unsigned long long v7[5]; // [rsp+0h] [rbp-28h] BYREF

  v7[1] = __readfsqword(0x28u);
  if ( a1 == a2 )
    return 0;
  v4 = *(unsigned char *)(a2 - 1);
  if ( !byte_2A180[0] || ((unsigned char)byte_2A182 & (unsigned char)~((unsigned char)v4 >> 7)) != 0 )
    return (unsigned char)byte_2A9C0[v4];
  v7[0] = a1;
  v6 = sub_CAF0(v7, 0, (char *)(a2 - 1), a3);
  return sub_CC90(a2 - 1 - (long long)v6, a3);
}



// Function: private_malloc @ 0xcd70
long long private_malloc(long long a1)
{
  if ( a1 < 0 )
    sub_1D4B0(a1);
  return sub_1D110(a1);
}



// Function: jit_exec @ 0xcd90
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
        sub_1D4B0(v19);
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



// Function: Pcompile @ 0xcf00
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
  v3 = byte_2A687 == 0 ? 0xFFFFFFF8 : 0;
  v4 = sub_1D110(56);
  v5 = v3 + 24;
  v6 = pcre2_general_context_create_8(sub_CD70, j_free_0, 0);
  *(long long *)v4 = v6;
  v7 = v6;
  v8 = pcre2_compile_context_create_8(v6);
  if ( byte_2A180[0] )
  {
    if ( !byte_2A182 )
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
  if ( byte_2A685 )
  {
    pcre2_set_compile_extra_options_8(v8, 8);
    v9 = 0;
  }
  else
  {
    v9 = 0;
    if ( byte_2A686 )
    {
      v23 = n + 17;
      ptra = (char *)sub_1D110(n + 17);
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
  *(int *)(v4 + 48) = sub_CD90((long long *)v4, (long long)"", 0, 0, 1u);
  *(int *)(v4 + 52) = sub_CD90((long long *)v4, (long long)"", 0, 0, 0);
  return v4;
}



// Function: Pexecute @ 0xd220
long long Pexecute(long long *a1, char *a2, long long a3, long long *a4, char *a5)
{
  char *v6; // rbx
  long long v7; // rdi
  bool v8; // r14
  long long v9; // r12
  unsigned int v10; // eax
  long long v11; // rdx
  long long v12; // rcx
  int v13; // r8d
  long long startchar_8; // rax
  long long v15; // rdx
  int v16; // eax
  long long v17; // rbp
  char *v18; // rax
  char *v20; // r12
  char *v21; // rax
  char *v22; // r12
  char *v23; // rax
  char *v24; // r12
  char *v25; // rax
  char *v26; // r12
  char *v27; // rax
  char *v28; // r12
  char *v29; // rax
  char *v30; // r12
  char *v31; // rax
  char *v32; // r12
  char *v33; // rax
  char *v34; // [rsp+0h] [rbp-78h]
  unsigned int v35; // [rsp+0h] [rbp-78h]
  long long *ovector_pointer_8; // [rsp+8h] [rbp-70h]
  long long v37; // [rsp+10h] [rbp-68h]
  char *v38; // [rsp+18h] [rbp-60h]

  v6 = a2;
  v7 = a1[3];
  if ( a5 )
    v6 = a5;
  v8 = *(v6 - 1) == byte_2A684;
  v38 = a2;
  ovector_pointer_8 = (long long *)pcre2_get_ovector_pointer_8(v7);
  while ( 2 )
  {
    v9 = (long long)v38;
    v34 = (char *)rawmemchr(v6, byte_2A684);
    while ( 1 )
    {
      while ( byte_2A180[(unsigned char)sub_C9D0((unsigned int)*v6) + 3] == -1 )
      {
        ++v6;
        v8 = 0;
        v9 = (long long)v6;
      }
      if ( v6 == v34 )
      {
        v17 = v9;
        v13 = *((int *)a1 + v8 + 12);
        ovector_pointer_8[1] = &v6[-v9];
        *ovector_pointer_8 = &v6[-v9];
        goto LABEL_21;
      }
      v10 = sub_CD90(a1, v9, (long long)&v34[-v9], (long long)&v6[-v9], !v8);
      if ( !(unsigned char)sub_CEF0(a1, v9, v11, v12, v10) )
        break;
      startchar_8 = pcre2_get_startchar_8(a1[3]);
      v15 = startchar_8;
      if ( (long long)&v6[-v9] <= startchar_8 )
      {
        if ( startchar_8 )
        {
          v37 = startchar_8;
          v16 = sub_CD90(a1, v9, startchar_8, (long long)&v6[-v9], !v8 | 0x40000002u);
          v15 = v37;
          v13 = v16;
          if ( v16 != -1 )
          {
LABEL_15:
            v17 = v9;
LABEL_16:
            if ( v13 <= 0 )
            {
              if ( v13 < -45 )
                goto LABEL_28;
              while ( 1 )
              {
                v35 = v13;
                v20 = sub_8A50();
                v21 = dcgettext(0, "%s: internal PCRE error: %d", 5);
                error(2, 0, v21, v20, v35);
LABEL_28:
                if ( v13 >= -63 )
                {
                  switch ( v13 )
                  {
                    case -63:
                      v22 = sub_8A50();
                      v23 = dcgettext(0, "%s: exceeded PCRE's heap limit", 5);
                      error(2, 0, v23, v22);
                      goto LABEL_31;
                    case -53:
                      goto LABEL_35;
                    case -52:
                      goto LABEL_34;
                    case -48:
                      goto LABEL_33;
                    case -47:
                      goto LABEL_32;
                    case -46:
LABEL_31:
                      v24 = sub_8A50();
                      v25 = dcgettext(0, "%s: exhausted PCRE JIT stack", 5);
                      error(2, 0, v25, v24);
LABEL_32:
                      v26 = sub_8A50();
                      v27 = dcgettext(0, "%s: exceeded PCRE's backtracking limit", 5);
                      error(2, 0, v27, v26);
LABEL_33:
                      v28 = sub_8A50();
                      v29 = dcgettext(0, "%s: memory exhausted", 5);
                      error(2, 0, v29, v28);
LABEL_34:
                      v30 = sub_8A50();
                      v31 = dcgettext(0, "%s: PCRE detected recurse loop", 5);
                      error(2, 0, v31, v30);
LABEL_35:
                      v32 = sub_8A50();
                      v33 = dcgettext(0, "%s: exceeded PCRE's nested backtracking limit", 5);
                      error(2, 0, v33, v32);
                      sub_A440(1);
                    default:
                      continue;
                  }
                }
              }
            }
            v18 = v34 + 1;
            if ( a5 )
            {
              v38 = (char *)(v17 + *ovector_pointer_8);
              v18 = (char *)(v17 + ovector_pointer_8[1]);
            }
            *a4 = v18 - v38;
            return v38 - a2;
          }
        }
        else
        {
          v13 = *((int *)a1 + v8 + 12);
          *ovector_pointer_8 = 0;
          ovector_pointer_8[1] = 0;
          if ( v13 != -1 )
            goto LABEL_15;
        }
        v6 = (char *)(v9 + v15 + 1);
        v8 = 0;
        v9 = (long long)v6;
      }
      else
      {
        v9 += startchar_8 + 1;
      }
    }
    v17 = v9;
LABEL_21:
    if ( v13 != -1 )
      goto LABEL_16;
    v8 = 1;
    v6 = v34 + 1;
    if ( v34 + 1 < &a2[a3] )
    {
      v38 = v34 + 1;
      continue;
    }
    return -1;
  }
}


