// Function: main @ 0x4050
long long main(int a1, char **a2, char **a3)
{
  char *v5; // rdi
  char *v6; // rax
  char **v7; // rsi
  int v8; // eax
  long long v9; // rax
  char *v10; // rax
  int v11; // eax
  long long v12; // rdi
  long long v13; // rdx
  long long v14; // rcx
  char *v16; // rax
  const char *v17; // rsi
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  int *v21; // rax
  char *v22; // rax
  long long v23; // r13
  char *v24; // r12
  int *v25; // rax
  long long v26; // [rsp+0h] [rbp-C8h]
  char **v27; // [rsp+8h] [rbp-C0h]
  long long v28; // [rsp+10h] [rbp-B8h]
  long long v29; // [rsp+18h] [rbp-B0h]
  char v30; // [rsp+25h] [rbp-A3h]
  unsigned char v31; // [rsp+26h] [rbp-A2h]
  char v32; // [rsp+27h] [rbp-A1h]
  const char *format; // [rsp+28h] [rbp-A0h]
  int v34; // [rsp+30h] [rbp-98h] BYREF
  int v35; // [rsp+34h] [rbp-94h]
  long long v36; // [rsp+38h] [rbp-90h]
  long long v37; // [rsp+40h] [rbp-88h]
  short v38; // [rsp+48h] [rbp-80h]
  int v39; // [rsp+4Ch] [rbp-7Ch]
  char v40; // [rsp+50h] [rbp-78h]
  long long v41; // [rsp+58h] [rbp-70h]
  char v42[12]; // [rsp+60h] [rbp-68h]
  short v43; // [rsp+6Ch] [rbp-5Ch]
  bool v44; // [rsp+6Eh] [rbp-5Ah]
  int v45; // [rsp+74h] [rbp-54h]
  long long v46; // [rsp+78h] [rbp-50h]
  long long v47; // [rsp+80h] [rbp-48h]
  unsigned long long v48; // [rsp+88h] [rbp-40h]

  v5 = *a2;
  v48 = __readfsqword(0x28u);
  sub_12680(v5);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_19D70(sub_E2B0);
  byte_2315A = (int)is_selinux_enabled() > 0;
  sub_7670(&v34);
  v40 = 0;
  v45 = 1;
  v36 = 0x200000004LL;
  v39 = 0;
  v35 = 1;
  v37 = 0x100000000LL;
  v38 = 0;
  v41 = 0;
  *(long long *)v42 = 256;
  *(int *)&v42[8] = 0;
  v43 = 0;
  v6 = getenv("POSIXLY_CORRECT");
  v31 = 0;
  v46 = 0;
  v47 = 0;
  v29 = 0;
  v26 = 0;
  v32 = 0;
  v27 = 0;
  v28 = 0;
  v30 = 0;
  v44 = v6 != 0;
  while ( 1 )
  {
    v7 = a2;
    v8 = getopt_long(a1, a2, "abdfHilLnprst:uvxPRS:TZ", &longopts, 0);
    if ( v8 == -1 )
      break;
    if ( v8 > 136 )
      goto LABEL_84;
    if ( v8 <= 71 )
    {
      if ( v8 == -131 )
      {
        sub_16A20(stdout, (char)"Jim Meyering");
        exit(0);
      }
      if ( v8 == -130 )
        sub_54F0(0);
LABEL_84:
      sub_54F0(1);
    }
    switch ( v8 )
    {
      case 72:
        v35 = 3;
        continue;
      case 76:
        v35 = 4;
        continue;
      case 80:
        v35 = 2;
        continue;
      case 82:
      case 114:
        v42[8] = 1;
        continue;
      case 83:
        v30 = 1;
        v28 = optarg;
        continue;
      case 84:
        v31 = 1;
        continue;
      case 90:
        if ( byte_2315A )
        {
          if ( optarg )
          {
            v29 = optarg;
          }
          else
          {
            v41 = selabel_open(0, 0, 0);
            if ( !v41 )
            {
              format = dcgettext(0, "warning: ignoring --context", 5);
              v21 = __errno_location();
              error(0, *v21, format);
            }
          }
        }
        else if ( optarg )
        {
          v16 = dcgettext(0, "warning: ignoring --context; it requires an SELinux-enabled kernel", 5);
          error(0, 0, v16);
        }
        continue;
      case 97:
        v42[0] = 1;
        v35 = 2;
        BYTE1(v39) = 1;
        HIWORD(v39) = 257;
        v42[2] = 1;
        if ( byte_2315A )
          v42[3] = 1;
        v42[5] = 1;
        *(short *)&v42[7] = 257;
        continue;
      case 98:
        v9 = optarg;
        v30 = 1;
        if ( !optarg )
          v9 = (long long)v27;
        v27 = (char **)v9;
        continue;
      case 100:
        v42[0] = 1;
        v35 = 2;
        continue;
      case 102:
        BYTE6(v37) = 1;
        continue;
      case 105:
        LODWORD(v36) = 3;
        continue;
      case 108:
        HIBYTE(v37) = 1;
        continue;
      case 110:
        LODWORD(v36) = 2;
        continue;
      case 112:
        goto LABEL_7;
      case 115:
        v42[10] = 1;
        continue;
      case 116:
        if ( v26 )
        {
          v22 = dcgettext(0, "multiple target directories specified", 5);
          error(1, 0, v22);
LABEL_89:
          v17 = "options --backup and --no-clobber are mutually exclusive";
LABEL_83:
          v20 = dcgettext(0, v17, 5);
          error(0, 0, v20);
          goto LABEL_84;
        }
        v26 = optarg;
        break;
      case 117:
        v42[11] = 1;
        continue;
      case 118:
        LOBYTE(v43) = 1;
        continue;
      case 120:
        LOBYTE(v39) = 1;
        continue;
      case 128:
        v42[1] = 0;
        continue;
      case 129:
        v32 = 1;
        continue;
      case 130:
        sub_49A0(optarg, &v34, 0);
        continue;
      case 131:
        byte_23159 = 1;
        continue;
      case 132:
        if ( optarg )
        {
          sub_49A0(optarg, &v34, 1);
          v42[2] = 1;
        }
        else
        {
LABEL_7:
          BYTE1(v39) = 1;
          HIWORD(v39) = 257;
          v42[2] = 1;
        }
        continue;
      case 133:
        if ( optarg )
          v45 = dword_1A460[sub_CFC0("--reflink", optarg, &off_22860, dword_1A460, 4, off_23028, 1)];
        else
          v45 = 2;
        continue;
      case 134:
        HIDWORD(v36) = dword_1A470[sub_CFC0("--sparse", optarg, off_22880, dword_1A470, 4, off_23028, 1)];
        continue;
      case 135:
        byte_23158 = 1;
        continue;
      case 136:
        BYTE5(v37) = 1;
        continue;
      default:
        goto LABEL_84;
    }
  }
  if ( HIBYTE(v37) && v42[10] )
  {
    v17 = "cannot make both hard and symbolic links";
    goto LABEL_83;
  }
  if ( (int)v36 == 2 )
  {
    v42[11] = 0;
    if ( v30 )
      goto LABEL_89;
LABEL_66:
    v11 = 0;
    if ( v45 == 2 && HIDWORD(v36) != 2 )
      goto LABEL_82;
    goto LABEL_52;
  }
  if ( !v30 )
    goto LABEL_66;
  if ( v45 == 2 && HIDWORD(v36) != 2 )
  {
LABEL_82:
    v17 = "--reflink can be used only with --sparse=auto";
    goto LABEL_83;
  }
  v10 = dcgettext(0, "backup type", 5);
  v7 = v27;
  v11 = sub_D8D0(v10, v27);
LABEL_52:
  v12 = v28;
  v34 = v11;
  sub_D0F0(v28);
  if ( v35 == 1 )
    goto LABEL_72;
LABEL_53:
  if ( v42[8] )
    goto LABEL_75;
  while ( 1 )
  {
    v14 = v41 | v29;
    if ( !(v41 | v29) )
    {
      if ( !v42[4] || byte_2315A )
        goto LABEL_59;
      goto LABEL_71;
    }
    if ( !v42[4] )
      break;
    if ( v42[3] )
    {
      v19 = dcgettext(0, "cannot set target context and preserve it", 5);
      v7 = 0;
      v12 = 1;
      error(1, 0, v19);
      goto LABEL_78;
    }
    if ( byte_2315A )
      goto LABEL_57;
LABEL_71:
    v18 = dcgettext(0, "cannot preserve security context without an SELinux-enabled kernel", 5);
    v7 = 0;
    v12 = 1;
    error(1, 0, v18);
LABEL_72:
    if ( !v42[8] || HIBYTE(v37) )
    {
LABEL_78:
      v35 = 4;
      goto LABEL_53;
    }
    v35 = 2;
LABEL_75:
    BYTE4(v37) = v32;
  }
  v42[3] = 0;
LABEL_57:
  if ( v29 )
  {
    v12 = v29;
    if ( (int)setfscreatecon(v29, v7, v13, v14) < 0 )
    {
      v23 = sub_14B30(v29);
      v24 = dcgettext(0, "failed to set default file creation context to %s", 5);
      v25 = __errno_location();
      error(1, *v25, v24, v23);
    }
  }
LABEL_59:
  sub_BDB0(v12, v7, v13, v14);
  return (unsigned char)sub_5A50((unsigned int)(a1 - optind), &a2[optind], v26, v31, &v34) ^ 1u;
}



// Function: decode_preserve_arg @ 0x49a0
void decode_preserve_arg(void *a1, char *a2, char a3)
{
  const char *v5; // r15
  const char *v6; // rax
  const char *v7; // r14
  char *v8; // rax
  bool v9; // zf
  const char *v10; // [rsp+0h] [rbp-48h]
  char *ptr; // [rsp+8h] [rbp-40h]

  ptr = (char *)sub_17130(a1);
  v5 = ptr;
  v6 = "--no-preserve";
  if ( a3 )
    v6 = "--preserve";
  v10 = v6;
  while ( 2 )
  {
    v7 = 0;
    v8 = strchr(v5, 44);
    if ( v8 )
    {
      *v8 = 0;
      v7 = v8 + 1;
    }
    switch ( sub_CFC0(v10, v5, off_22460, &unk_1A440, 4, off_23028, 1) )
    {
      case 0LL:
      case 7LL:
      case 10LL:
      case 11LL:
      case 15LL:
        a2[30] = a3;
        a2[32] = a3 ^ 1;
        goto LABEL_8;
      case 1LL:
      case 8LL:
      case 12LL:
        a2[31] = a3;
        goto LABEL_8;
      case 2LL:
      case 9LL:
      case 13LL:
        a2[29] = a3;
        goto LABEL_8;
      case 3LL:
      case 14LL:
        a2[48] = a3;
        goto LABEL_8;
      case 4LL:
        a2[52] = a3;
        a2[51] = a3;
        if ( !v7 )
          goto LABEL_11;
        goto LABEL_9;
      case 5LL:
        a2[53] = a3;
        a2[54] = a3;
        goto LABEL_8;
      case 6LL:
        a2[30] = a3;
        v9 = byte_2315A == 0;
        a2[31] = a3;
        a2[29] = a3;
        a2[48] = a3;
        a2[32] = a3 ^ 1;
        if ( !v9 )
          a2[51] = a3;
        a2[53] = a3;
LABEL_8:
        if ( v7 )
        {
LABEL_9:
          v5 = v7;
          continue;
        }
LABEL_11:
        free(ptr);
        return;
      default:
        abort();
    }
  }
}



// Function: make_dir_parents_private @ 0x4b10
long long make_dir_parents_private(const char *src, size_t a2, int a3, long long a4, long long *a5, char *a6, long long a7)
{
  size_t v9; // r14
  size_t v11; // rax
  struct stat **v12; // rcx
  signed long long v13; // rdx
  void *v14; // rsp
  long long *v15; // rdx
  signed long long v16; // rcx
  void *v17; // rsp
  const char *v18; // rsi
  char *v19; // rbx
  bool v20; // zf
  char *v21; // r14
  const char *v22; // rdi
  char *v23; // rax
  char *v24; // rbx
  int v25; // r13d
  int v26; // r8d
  long long v27; // rax
  __m128i v28; // xmm1
  __m128i v29; // xmm2
  __m128i v30; // xmm3
  long long v31; // r9
  __m128i v32; // xmm5
  __m128i v33; // xmm6
  __m128i v34; // xmm7
  __m128i v35; // xmm0
  __m128i v36; // xmm4
  long long *v37; // rcx
  long long v38; // rax
  long long v39; // rdx
  long long v40; // r9
  int v41; // edx
  int v42; // r13d
  short v43; // cx
  int v44; // eax
  long long v45; // r9
  int v46; // eax
  long long v47; // r9
  __mode_t st_mode; // eax
  long long v49; // r12
  char *v50; // rax
  int v51; // eax
  char *v52; // rsi
  long long v53; // r12
  char *v54; // rax
  const char *v55; // rsi
  long long v56; // r13
  char *v57; // r12
  int *v58; // rax
  long long v59; // [rsp+0h] [rbp-21A8h]
  long long v60; // [rsp+8h] [rbp-21A0h] BYREF
  char v61[4088]; // [rsp+10h] [rbp-2198h] BYREF
  long long v62; // [rsp+1008h] [rbp-11A0h] BYREF
  long long v63; // [rsp+1010h] [rbp-1198h] BYREF
  struct stat *v64; // [rsp+2008h] [rbp-1A0h] BYREF
  long long v65; // [rsp+2010h] [rbp-198h]
  long long v66; // [rsp+2018h] [rbp-190h]
  long long *v67; // [rsp+2020h] [rbp-188h]
  struct stat *p_buf; // [rsp+2028h] [rbp-180h]
  char *v69; // [rsp+2030h] [rbp-178h]
  char *file; // [rsp+2038h] [rbp-170h]
  void *srca; // [rsp+2040h] [rbp-168h]
  struct stat buf; // [rsp+2048h] [rbp-160h] BYREF
  __m128i v73; // [rsp+20D8h] [rbp-D0h] BYREF
  __m128i v74; // [rsp+20E8h] [rbp-C0h] BYREF
  __m128i v75; // [rsp+20F8h] [rbp-B0h] BYREF
  __m128i v76; // [rsp+2108h] [rbp-A0h] BYREF
  __m128i v77; // [rsp+2118h] [rbp-90h] BYREF
  __m128i v78; // [rsp+2128h] [rbp-80h] BYREF
  __m128i v79; // [rsp+2138h] [rbp-70h] BYREF
  __m128i v80; // [rsp+2148h] [rbp-60h] BYREF
  __m128i v81; // [rsp+2158h] [rbp-50h] BYREF
  unsigned long long v82; // [rsp+2170h] [rbp-38h]

  v65 = a4;
  v67 = a5;
  v69 = a6;
  v82 = __readfsqword(0x28u);
  v9 = sub_E4A0();
  *v67 = 0;
  if ( v9 <= a2 )
    return 1;
  v11 = strlen(src);
  v12 = (struct stat **)((char *)&v64 - ((v11 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v64 != v12 )
  {
    while ( &v62 != (long long *)v12 )
      ;
  }
  v13 = ((short)v11 + 24) & 0xFF0;
  v14 = alloca(v13);
  if ( (((short)v11 + 24) & 0xFF0) != 0 )
    *(long long *)&v61[v13 + 4080] = *(long long *)&v61[v13 + 4080];
  srca = memcpy(&v63, src, v11 + 1);
  v15 = (long long *)((char *)&v62 - ((v9 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v62 != v15 )
  {
    while ( &v60 != v15 )
      ;
  }
  v16 = ((short)v9 + 24) & 0xFF0;
  v17 = alloca(v16);
  if ( (((short)v9 + 24) & 0xFF0) != 0 )
    *(long long *)((char *)&v59 + v16) = *(long long *)((char *)&v59 + v16);
  memcpy(v61, srca, v9);
  v18 = &v61[a2];
  for ( v61[v9] = 0; *v18 == 47; ++v18 )
    ;
  p_buf = &buf;
  if ( fstatat(a3, v18, &buf, 0) )
  {
    v19 = (char *)srca + a2;
    v20 = *v19 == 47;
    file = v19;
    v21 = v19;
    if ( v20 )
    {
      do
        ++v21;
      while ( *v21 == 47 );
    }
    else
    {
      v21 = file;
    }
    v22 = v21;
    v64 = (struct stat *)&v73;
    while ( 1 )
    {
      v23 = strchr(v22, 47);
      v24 = v23;
      if ( !v23 )
        return 1;
      *v23 = 0;
      v25 = fstatat(a3, v21, p_buf, 0);
      if ( !v25 && (*(int *)(a7 + 28) & 0xFFFFFF00) == 0 )
        break;
      if ( stat(file, v64) )
      {
        v26 = *__errno_location();
        if ( v26 )
          goto LABEL_51;
      }
      else if ( (v74.m128i_i16[4] & 0xF000) != 0x4000 )
      {
        v26 = 20;
LABEL_51:
        LODWORD(srca) = v26;
        v49 = sub_14440(4, file);
        v50 = dcgettext(0, "failed to get attributes of %s", 5);
        error(0, (int)srca, v50, v49);
        return 0;
      }
      v27 = sub_16C50(168);
      v28 = _mm_load_si128(&v74);
      v29 = _mm_load_si128(&v75);
      v30 = _mm_load_si128(&v76);
      v31 = v27;
      *(__m128i *)v27 = _mm_load_si128(&v73);
      v32 = _mm_load_si128(&v78);
      v33 = _mm_load_si128(&v79);
      v34 = _mm_load_si128(&v80);
      v35 = _mm_load_si128(&v81);
      *(__m128i *)(v27 + 16) = v28;
      v36 = _mm_load_si128(&v77);
      v37 = v67;
      *(__m128i *)(v27 + 32) = v29;
      *(__m128i *)(v27 + 48) = v30;
      *(__m128i *)(v27 + 64) = v36;
      *(__m128i *)(v27 + 80) = v32;
      *(__m128i *)(v27 + 96) = v33;
      *(__m128i *)(v27 + 112) = v34;
      *(__m128i *)(v27 + 128) = v35;
      *(long long *)(v27 + 152) = v24 - (char *)srca;
      v38 = *v37;
      *(char *)(v31 + 144) = 0;
      *(long long *)(v31 + 160) = v38;
      *v37 = v31;
      if ( !v25 )
        break;
      v39 = *(unsigned int *)(v31 + 24);
      v66 = v31;
      if ( !(unsigned char)sub_72A0(file, srca, v39, 1, a7) )
        return 0;
      v40 = v66;
      *v69 = 1;
      v41 = *(int *)(v40 + 24);
      if ( *(char *)(a7 + 29) )
      {
        v42 = *(int *)(v40 + 24) & 0x3F;
        v43 = ~(v41 & 0x3F);
      }
      else if ( *(char *)(a7 + 30) )
      {
        v42 = *(int *)(v40 + 24) & 0x12;
        v43 = ~(v41 & 0x12);
      }
      else
      {
        v43 = -1;
        v42 = 0;
      }
      if ( *(char *)(a7 + 32) )
        LOWORD(v41) = 511;
      v66 = v40;
      v44 = mkdirat(a3, v21, (unsigned short)v43 & (unsigned short)v41 & 0xFFF);
      v45 = v66;
      if ( v44 )
      {
        v55 = "cannot make directory %s";
        v56 = sub_14440(4, srca);
        goto LABEL_66;
      }
      if ( v65 )
      {
        __printf_chk(1, v65, file, srca);
        v45 = v66;
      }
      v66 = v45;
      v46 = fstatat(a3, v21, p_buf, 256);
      v47 = v66;
      if ( v46 )
      {
        v55 = "failed to get attributes of %s";
        v56 = sub_14440(4, srca);
LABEL_66:
        v57 = dcgettext(0, v55, 5);
        v58 = __errno_location();
        error(0, *v58, v57, v56);
        return 0;
      }
      st_mode = buf.st_mode;
      if ( !*(char *)(a7 + 30) )
      {
        if ( (v42 & ~buf.st_mode) != 0
          && (v51 = sub_BBD0(), v47 = v66, v42 &= ~v51, st_mode = buf.st_mode, (v42 & ~buf.st_mode) != 0)
          || (st_mode & 0x1C0) != 0x1C0 )
        {
          *(char *)(v47 + 144) = 1;
          *(int *)(v47 + 24) = st_mode | v42;
        }
      }
      if ( (st_mode | 0x1C0) != st_mode && fchmodat(a3, v21, st_mode | 0x1C0, 256) )
      {
        v55 = "setting permissions for %s";
        v56 = sub_14440(4, srca);
        goto LABEL_66;
      }
      if ( !*v69 )
        goto LABEL_23;
LABEL_26:
      v20 = v24[1] == 47;
      *v24 = 47;
      v22 = v24 + 1;
      if ( v20 )
      {
        do
          ++v22;
        while ( *v22 == 47 );
      }
    }
    if ( !(unsigned char)sub_72A0(file, srca, 0, 0, a7) )
      return 0;
    if ( (buf.st_mode & 0xF000) != 0x4000 )
    {
      v52 = srca;
      goto LABEL_62;
    }
    *v69 = 0;
LABEL_23:
    if ( (*(long long *)(a7 + 40) || *(char *)(a7 + 51)) && !(unsigned char)sub_7510(srca, 0, a7) && *(char *)(a7 + 52) )
      return 0;
    goto LABEL_26;
  }
  if ( (buf.st_mode & 0xF000) == 0x4000 )
  {
    *v69 = 0;
    return 1;
  }
  v52 = v61;
LABEL_62:
  v53 = sub_14440(4, v52);
  v54 = dcgettext(0, "%s exists but is not a directory", 5);
  error(0, 0, v54, v53);
  return 0;
}



// Function: re_protect @ 0x5210
long long re_protect(const char *src, int fd, long long a3, long long a4, char *a5)
{
  size_t v7; // rax
  struct timespec **v8; // rcx
  signed long long v9; // rdx
  void *v10; // rsp
  char *v11; // r12
  const char *v12; // r13
  char v13; // r8
  bool v14; // zf
  long long v15; // rax
  long long v16; // rdx
  long long v17; // rax
  long long v18; // r13
  char *v19; // r12
  int *v20; // rax
  __mode_t v22; // edx
  const char *v23; // rsi
  long long v24; // r13
  char *v25; // r12
  int *v26; // rax
  long long v28; // [rsp+0h] [rbp-1078h]
  long long v29; // [rsp+8h] [rbp-1070h] BYREF
  long long v30; // [rsp+10h] [rbp-1068h] BYREF
  struct timespec *times; // [rsp+1008h] [rbp-70h] BYREF
  long long v32; // [rsp+1010h] [rbp-68h]
  long long v33[12]; // [rsp+1018h] [rbp-60h] BYREF

  v32 = a3;
  v33[5] = __readfsqword(0x28u);
  v7 = strlen(src);
  v8 = (struct timespec **)((char *)&times - ((v7 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &times != v8 )
  {
    while ( &v29 != (long long *)v8 )
      ;
  }
  v9 = ((short)v7 + 24) & 0xFF0;
  v10 = alloca(v9);
  if ( (((short)v7 + 24) & 0xFF0) != 0 )
    *(long long *)((char *)&v28 + v9) = *(long long *)((char *)&v28 + v9);
  v11 = memcpy(&v30, src, v7 + 1);
  v12 = &v11[v32 - (long long)src];
  if ( !a4 )
    return 1;
  times = (struct timespec *)v33;
  while ( 1 )
  {
    v14 = a5[31] == 0;
    v11[*(long long *)(a4 + 152)] = 0;
    if ( !v14 )
    {
      v15 = *(long long *)(a4 + 80);
      v33[0] = *(long long *)(a4 + 72);
      v16 = *(long long *)(a4 + 88);
      v33[1] = v15;
      v17 = *(long long *)(a4 + 96);
      v33[2] = v16;
      v33[3] = v17;
      if ( utimensat(fd, v12, times, 0) )
      {
        v18 = sub_14440(4, v11);
        v19 = dcgettext(0, "failed to preserve times for %s", 5);
        v20 = __errno_location();
        error(0, *v20, v19, v18);
        return 0;
      }
    }
    if ( !a5[29] || !fchownat(fd, v12, *(int *)(a4 + 28), *(int *)(a4 + 32), 256) )
    {
      v13 = a5[30];
      if ( v13 )
        goto LABEL_17;
      goto LABEL_9;
    }
    if ( !(unsigned char)sub_76C0(a5) )
      break;
    fchownat(fd, v12, 0xFFFFFFFF, *(int *)(a4 + 32), 256);
    v13 = a5[30];
    if ( v13 )
    {
LABEL_17:
      if ( (unsigned int)sub_C830(v12, 0xFFFFFFFFLL, v11, 0xFFFFFFFFLL, *(unsigned int *)(a4 + 24)) )
        return 0;
      goto LABEL_10;
    }
LABEL_9:
    if ( *(char *)(a4 + 144) )
    {
      v22 = *(int *)(a4 + 24);
      LOBYTE(v32) = v13;
      if ( fchmodat(fd, v12, v22, 256) )
      {
        v23 = "failed to preserve permissions for %s";
        v24 = sub_14440(4, v11);
        goto LABEL_21;
      }
    }
LABEL_10:
    v11[*(long long *)(a4 + 152)] = 47;
    a4 = *(long long *)(a4 + 160);
    if ( !a4 )
      return 1;
  }
  LOBYTE(v32) = 0;
  v23 = "failed to preserve ownership for %s";
  v24 = sub_14440(4, v11);
LABEL_21:
  v25 = dcgettext(0, v23, 5);
  v26 = __errno_location();
  error(0, *v26, v25, v24);
  return (unsigned char)v32;
}



// Function: usage @ 0x54f0
void usage(int status)
{
  long long v2; // r13
  char *v3; // rax
  long long *v4; // rbx
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
  FILE *v28; // rbp
  char *v29; // rax
  FILE *v30; // rbp
  char *v31; // rax
  FILE *v32; // rbp
  char *v33; // rax
  FILE *v34; // rbp
  char *v35; // rax
  FILE *v36; // rbp
  char *v37; // rax
  FILE *v38; // rbp
  char *v39; // rax
  FILE *v40; // rbp
  char *v41; // rax
  FILE *v42; // rbp
  char *v43; // rax
  FILE *v44; // rbp
  char *v45; // rax
  const char *v46; // rsi
  const char *v47; // r14
  char *v48; // rax
  char *v49; // rdi
  const char *v50; // r13
  char *v51; // rax
  const char *v52; // rcx
  char *v53; // r13
  char *v54; // rax
  FILE *v55; // rbx
  char *v56; // rax
  long long v57[23]; // [rsp+0h] [rbp-B8h] BYREF

  v2 = qword_235A0;
  v57[15] = __readfsqword(0x28u);
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = v57;
    v5 = dcgettext(
           0,
           "Usage: %s [OPTION]... [-T] SOURCE DEST\n"
           "  or:  %s [OPTION]... SOURCE... DIRECTORY\n"
           "  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",
           5);
    __printf_chk(1, v5, v2, v2);
    v6 = stdout;
    v7 = dcgettext(0, "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n", 5);
    fputs_unlocked(v7, v6);
    v8 = stdout;
    v9 = dcgettext(0, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = dcgettext(
            0,
            "  -a, --archive                same as -dR --preserve=all\n"
            "      --attributes-only        don't copy the file data, just the attributes\n"
            "      --backup[=CONTROL]       make a backup of each existing destination file\n"
            "  -b                           like --backup but does not accept an argument\n"
            "      --copy-contents          copy contents of special files when recursive\n"
            "  -d                           same as --no-dereference --preserve=links\n",
            5);
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = dcgettext(
            0,
            "  -f, --force                  if an existing destination file cannot be\n"
            "                                 opened, remove it and try again (this option\n"
            "                                 is ignored when the -n option is also used)\n"
            "  -i, --interactive            prompt before overwrite (overrides a previous -n\n"
            "                                  option)\n"
            "  -H                           follow command-line symbolic links in SOURCE\n",
            5);
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = dcgettext(
            0,
            "  -l, --link                   hard link files instead of copying\n"
            "  -L, --dereference            always follow symbolic links in SOURCE\n",
            5);
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = dcgettext(
            0,
            "  -n, --no-clobber             do not overwrite an existing file (overrides\n"
            "                                 a previous -i option)\n"
            "  -P, --no-dereference         never follow symbolic links in SOURCE\n",
            5);
    fputs_unlocked(v17, v16);
    v18 = stdout;
    v19 = dcgettext(
            0,
            "  -p                           same as --preserve=mode,ownership,timestamps\n"
            "      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n"
            "                                 mode,ownership,timestamps), if possible\n"
            "                                 additional attributes: context, links, xattr,\n"
            "                                 all\n",
            5);
    fputs_unlocked(v19, v18);
    v20 = stdout;
    v21 = dcgettext(
            0,
            "      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n"
            "      --parents                use full source file name under DIRECTORY\n",
            5);
    fputs_unlocked(v21, v20);
    v22 = stdout;
    v23 = dcgettext(
            0,
            "  -R, -r, --recursive          copy directories recursively\n"
            "      --reflink[=WHEN]         control clone/CoW copies. See below\n"
            "      --remove-destination     remove each existing destination file before\n"
            "                                 attempting to open it (contrast with --force)\n",
            5);
    fputs_unlocked(v23, v22);
    v24 = stdout;
    v25 = dcgettext(
            0,
            "      --sparse=WHEN            control creation of sparse files. See below\n"
            "      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n"
            "                                 argument\n",
            5);
    fputs_unlocked(v25, v24);
    v26 = stdout;
    v27 = dcgettext(
            0,
            "  -s, --symbolic-link          make symbolic links instead of copying\n"
            "  -S, --suffix=SUFFIX          override the usual backup suffix\n"
            "  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n"
            "  -T, --no-target-directory    treat DEST as a normal file\n",
            5);
    fputs_unlocked(v27, v26);
    v28 = stdout;
    v29 = dcgettext(
            0,
            "  -u, --update                 copy only when the SOURCE file is newer\n"
            "                                 than the destination file or when the\n"
            "                                 destination file is missing\n"
            "  -v, --verbose                explain what is being done\n"
            "  -x, --one-file-system        stay on this file system\n",
            5);
    fputs_unlocked(v29, v28);
    v30 = stdout;
    v31 = dcgettext(
            0,
            "  -Z                           set SELinux security context of destination\n"
            "                                 file to default type\n"
            "      --context[=CTX]          like -Z, or if CTX is specified then set the\n"
            "                                 SELinux or SMACK security context to CTX\n",
            5);
    fputs_unlocked(v31, v30);
    v32 = stdout;
    v33 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v33, v32);
    v34 = stdout;
    v35 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v35, v34);
    v36 = stdout;
    v37 = dcgettext(
            0,
            "\n"
            "By default, sparse SOURCE files are detected by a crude heuristic and the\n"
            "corresponding DEST file is made sparse as well.  That is the behavior\n"
            "selected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\n"
            "file whenever the SOURCE file contains a long enough sequence of zero bytes.\n"
            "Use --sparse=never to inhibit creation of sparse files.\n",
            5);
    fputs_unlocked(v37, v36);
    v38 = stdout;
    v39 = dcgettext(
            0,
            "\n"
            "When --reflink[=always] is specified, perform a lightweight copy, where the\n"
            "data blocks are copied only when modified.  If this is not possible the copy\n"
            "fails, or if --reflink=auto is specified, fall back to a standard copy.\n"
            "Use --reflink=never to ensure a standard copy is performed.\n",
            5);
    fputs_unlocked(v39, v38);
    v40 = stdout;
    v41 = dcgettext(
            0,
            "\n"
            "The backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\n"
            "The version control method may be selected via the --backup option or through\n"
            "the VERSION_CONTROL environment variable.  Here are the values:\n"
            "\n",
            5);
    fputs_unlocked(v41, v40);
    v42 = stdout;
    v43 = dcgettext(
            0,
            "  none, off       never make backups (even if --backup is given)\n"
            "  numbered, t     make numbered backups\n"
            "  existing, nil   numbered if numbered backups exist, simple otherwise\n"
            "  simple, never   always make simple backups\n",
            5);
    fputs_unlocked(v43, v42);
    v44 = stdout;
    v45 = dcgettext(
            0,
            "\n"
            "As a special case, cp makes a backup of SOURCE when the force and backup\n"
            "options are given and SOURCE and DEST are the same name for an existing,\n"
            "regular file.\n",
            5);
    fputs_unlocked(v45, v44);
    v57[0] = "[";
    v46 = "[";
    v57[1] = "test invocation";
    v57[2] = "coreutils";
    v57[3] = "Multi-call invocation";
    v57[6] = "sha256sum";
    v57[4] = "sha224sum";
    v57[8] = "sha384sum";
    v57[5] = "sha2 utilities";
    v57[7] = "sha2 utilities";
    v57[9] = "sha2 utilities";
    v57[10] = "sha512sum";
    v57[11] = "sha2 utilities";
    v57[12] = 0;
    v57[13] = 0;
    do
    {
      if ( !strcmp("cp", v46) )
        break;
      v46 = (const char *)v4[2];
      v4 += 2;
    }
    while ( v46 );
    v47 = (const char *)v4[1];
    if ( !v47 )
      v47 = "cp";
    v48 = dcgettext(0, "\n%s online help: <%s>\n", 5);
    __printf_chk(1, v48, "GNU coreutils", "https://www.gnu.org/software/coreutils/");
    v49 = setlocale(5, 0);
    if ( v49 && strncmp(v49, "en_", 3u) )
    {
      v55 = stdout;
      v56 = dcgettext(0, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
      fputs_unlocked(v56, v55);
    }
    v50 = "cp";
    if ( !strcmp("cp", "[") )
      v50 = "test";
    v51 = dcgettext(0, "Full documentation <%s%s>\n", 5);
    v52 = v50;
    v53 = " invocation";
    __printf_chk(1, v51, "https://www.gnu.org/software/coreutils/", v52);
    if ( v47 != "cp" )
      v53 = (char *)"";
    v54 = dcgettext(0, "or available locally via: info '(coreutils) %s%s'\n", 5);
    __printf_chk(1, v54, v47, v53);
  }
  exit(status);
}



// Function: do_copy @ 0x5a50
long long do_copy(int a1, const char **a2, const char *a3, char a4, long long a5)
{
  const char **v5; // r13
  long long v6; // rbx
  unsigned int v7; // r14d
  const char *v8; // r15
  const char *v9; // rdi
  const char *v10; // r12
  size_t v11; // rax
  unsigned char **v12; // rcx
  signed long long v13; // rdx
  void *v14; // rsp
  const char *v15; // r12
  int v16; // eax
  char *v17; // r12
  const char *v19; // r14
  const char *v20; // r12
  unsigned char v21; // bl
  int v22; // eax
  int *v23; // rdi
  long long v24; // rcx
  int *v25; // rsi
  size_t v26; // rax
  unsigned char **v27; // rcx
  signed long long v28; // rdx
  void *v29; // rsp
  void *v30; // r12
  const char *v31; // rdi
  const char *v32; // rax
  const char *v33; // rcx
  bool v34; // zf
  char v35; // al
  char *v36; // rdx
  char v37; // al
  long long *v38; // r15
  void *v39; // rdi
  const char *v40; // rbx
  int v41; // r12d
  char *v42; // rax
  char *v43; // rax
  long long v44; // rax
  const char *v45; // rsi
  long long v46; // r12
  char *v47; // rax
  long long v48; // r13
  char *v49; // r12
  int *v50; // rax
  const char *v51; // rsi
  char *v52; // rax
  long long v53; // rax
  const char *v54; // [rsp+0h] [rbp-1128h]
  char v55[15]; // [rsp+8h] [rbp-1120h] BYREF
  unsigned char *v56; // [rsp+1008h] [rbp-120h] BYREF
  char **v57; // [rsp+1010h] [rbp-118h]
  char *v58; // [rsp+1018h] [rbp-110h]
  const char *v59; // [rsp+1020h] [rbp-108h]
  long long v60; // [rsp+1028h] [rbp-100h]
  int fd; // [rsp+1030h] [rbp-F8h]
  int v62; // [rsp+1034h] [rbp-F4h]
  unsigned char v63; // [rsp+1046h] [rbp-E2h] BYREF
  char v64; // [rsp+1047h] [rbp-E1h] BYREF
  void *ptr; // [rsp+1048h] [rbp-E0h] BYREF
  char *v66; // [rsp+1050h] [rbp-D8h] BYREF
  struct stat buf; // [rsp+1058h] [rbp-D0h] BYREF
  unsigned long long v68; // [rsp+10F0h] [rbp-38h]

  v5 = a2;
  v62 = a1;
  v59 = a3;
  v60 = a5;
  v68 = __readfsqword(0x28u);
  v63 = 0;
  if ( (a3 == 0) >= a1 )
  {
    if ( a1 != 1 )
    {
      v51 = "missing file operand";
LABEL_67:
      v52 = dcgettext(0, v51, 5);
      error(0, 0, v52);
LABEL_63:
      sub_54F0(1);
    }
    v53 = sub_14440(4, *a2);
    v45 = "missing destination file operand after %s";
    v46 = v53;
    goto LABEL_62;
  }
  buf.st_mode = 0;
  if ( a4 )
  {
    if ( v59 )
    {
      v43 = dcgettext(0, "cannot combine --target-directory (-t) and --no-target-directory (-T)", 5);
      a2 = 0;
      error(1, 0, v43);
    }
    else if ( v62 <= 2 )
    {
      goto LABEL_19;
    }
    v44 = sub_14440(4, a2[2]);
    v45 = "extra operand %s";
    v46 = v44;
LABEL_62:
    v47 = dcgettext(0, v45, 5);
    error(0, 0, v47, v46);
    goto LABEL_63;
  }
  if ( v59 )
  {
    fd = sub_156A0(v59, &buf);
    if ( fd == -1 )
    {
      v48 = sub_14440(4, v59);
      v49 = dcgettext(0, "target directory %s", 5);
      v50 = __errno_location();
      error(1, *v50, v49, v48);
    }
    if ( v62 == 1 )
      goto LABEL_6;
    goto LABEL_48;
  }
  v40 = a2[v62 - 1];
  v59 = v40;
  fd = sub_156A0(v40, &buf);
  if ( fd == -1 )
  {
    v41 = *__errno_location();
    if ( v41 == 2 )
      goto LABEL_57;
    while ( v62 > 2 )
    {
      v5 = (const char **)sub_14440(4, v59);
      v42 = dcgettext(0, "target %s", 5);
      error(1, v41, v42, v5);
LABEL_57:
      v63 = 1;
    }
    goto LABEL_19;
  }
  --v62;
  if ( !v40 )
  {
LABEL_19:
    v19 = *v5;
    v20 = v5[1];
    if ( !byte_23159 )
    {
      v21 = v63;
      if ( *(char *)(v60 + 22)
        && *(int *)v60
        && !strcmp(*v5, v5[1])
        && !v63
        && (buf.st_mode || !stat(v20, &buf))
        && (buf.st_mode & 0xF000) == 0x8000 )
      {
        v22 = sub_D850(4294967196LL, v20, *(unsigned int *)v60);
        v23 = &dword_23100;
        v24 = 22;
        v25 = (int *)v60;
        while ( v24 )
        {
          *v23++ = *v25++;
          --v24;
        }
        LODWORD(v20) = v22;
        dword_23100 = 0;
        v60 = (long long)&dword_23100;
      }
      return (unsigned int)sub_BA80((int)v19, (int)v20, -100, (int)v20, -v21, v60, (long long)&v66, 0);
    }
    v51 = "with --parents, the destination must be a directory";
    goto LABEL_67;
  }
  if ( v62 > 1 )
  {
LABEL_48:
    sub_75F0(v60);
    sub_7630(v60);
LABEL_6:
    v6 = 0;
    v7 = 1;
    v57 = &v66;
    v58 = &v64;
    v56 = &v63;
    while ( 1 )
    {
      v8 = a2[v6];
      v66 = 0;
      if ( byte_23158 )
      {
        sub_E5A0(v8);
        v9 = v8;
        if ( !byte_23159 )
        {
LABEL_9:
          v10 = (const char *)sub_D970(v9);
          v11 = strlen(v10);
          v12 = (unsigned char **)((char *)&v56 - ((v11 + 24) & 0xFFFFFFFFFFFFF000LL));
          if ( &v56 != v12 )
          {
            while ( v55 != (char *)v12 )
              ;
          }
          v13 = ((short)v11 + 24) & 0xFF0;
          v14 = alloca(v13);
          if ( (((short)v11 + 24) & 0xFF0) != 0 )
            *(long long *)&v55[v13 - 8] = *(long long *)&v55[v13 - 8];
          v15 = (const char *)memcpy(v55, v10, v11 + 1);
          sub_E5A0(v15);
          v16 = strcmp(v15, "..");
          v17 = (char *)sub_EA10(v59, &v15[v16 == 0], v57);
          goto LABEL_14;
        }
      }
      else
      {
        v9 = v8;
        if ( !byte_23159 )
          goto LABEL_9;
      }
      v26 = strlen(v9);
      v27 = (unsigned char **)((char *)&v56 - ((v26 + 24) & 0xFFFFFFFFFFFFF000LL));
      if ( &v56 != v27 )
      {
        while ( v55 != (char *)v27 )
          ;
      }
      v28 = ((short)v26 + 24) & 0xFF0;
      v29 = alloca(v28);
      if ( (((short)v26 + 24) & 0xFF0) != 0 )
        *(long long *)&v55[v28 - 8] = *(long long *)&v55[v28 - 8];
      v30 = memcpy(v55, v8, v26 + 1);
      sub_E5A0(v30);
      v31 = v59;
      v32 = (const char *)sub_EA10(v59, v30, v57);
      v33 = 0;
      v17 = (char *)v32;
      v34 = *(char *)(v60 + 60) == 0;
      v54 = v31;
      if ( !v34 )
        v33 = "%s -> %s\n";
      v35 = sub_4B10(v32, v66 - v32, fd, (long long)v33, (long long *)&ptr, v56, v60);
      if ( *v66 == 47 )
      {
        v36 = v66 + 1;
        do
          v66 = v36++;
        while ( *(v36 - 1) == 47 );
      }
      if ( !v35 )
      {
        v37 = byte_23159;
        v7 = 0;
LABEL_43:
        if ( v37 )
        {
          v38 = ptr;
          if ( ptr )
          {
            do
            {
              v39 = v38;
              v38 = (long long *)v38[20];
              ptr = v38;
              free(v39);
            }
            while ( v38 );
          }
        }
        goto LABEL_15;
      }
LABEL_14:
      v7 &= sub_BA80((int)v8, (int)v17, fd, (int)v66, v63, v60, (long long)v58, 0);
      if ( byte_23159 )
      {
        v7 &= sub_5210(v17, fd, (long long)v66, (long long)ptr, (char *)v60);
        v37 = byte_23159;
        goto LABEL_43;
      }
LABEL_15:
      ++v6;
      free(v17);
      if ( v62 <= (int)v6 )
        return v7;
    }
  }
  v7 = 1;
  if ( v62 == 1 )
    goto LABEL_6;
  return v7;
}



// Function: copy_attr_free @ 0x60f0
void copy_attr_free()
{
  ;
}



// Function: copy_attr_quote @ 0x6100
long long copy_attr_quote(long long a1, long long a2)
{
  return sub_14440(4, a2);
}



// Function: subst_suffix @ 0x6110
void *subst_suffix(void *src, long long a2, const char *a3)
{
  size_t v4; // r15
  char *v5; // rbp

  v4 = strlen(a3) + 1;
  v5 = (char *)sub_16C70(v4 + a2 - (long long)src);
  memcpy(&v5[a2 - (long long)src], a3, v4);
  return memcpy(v5, src, a2 - (long long)src);
}



// Function: write_zeros @ 0x6170
long long write_zeros(int fd, size_t n)
{
  size_t v2; // rbp
  size_t v3; // rbx
  long long v4; // rax

  v2 = n;
  if ( !buf )
  {
    buf = calloc(nmemb, 1u);
    if ( !buf )
    {
      nmemb = 1024;
      buf = &unk_23160;
    }
  }
  if ( !n )
    return 1;
  while ( 1 )
  {
    v3 = nmemb;
    if ( v2 <= nmemb )
      v3 = v2;
    v4 = sub_111C0(fd, buf, v3);
    if ( v4 != v3 )
      break;
    v2 -= v4;
    if ( !v2 )
      return 1;
  }
  return 0;
}



// Function: copy_attr @ 0x6220
bool copy_attr(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  char v7; // dl
  long long (*v8)(); // r8
  long long (*v9)(int, int, int, int, int, int, char); // r9
  long long (**v10)(int, int, int, int, int, int, char); // r9
  long long (*v12)(int, int, int, int, int, int, char); // [rsp+0h] [rbp-28h] BYREF
  long long (*v13)(long long, long long); // [rsp+8h] [rbp-20h]
  void (*v14)(); // [rsp+10h] [rbp-18h]
  unsigned long long v15; // [rsp+18h] [rbp-10h]

  v15 = __readfsqword(0x28u);
  v7 = *(char *)(a5 + 51);
  if ( !*(char *)(a5 + 49) || *(char *)(a5 + 54) )
  {
    v8 = sub_6720;
    v9 = sub_6440;
    if ( !v7 )
    {
      v8 = sub_6720;
      if ( !*(long long *)(a5 + 40) )
        v8 = 0;
      v9 = sub_6440;
    }
LABEL_4:
    v12 = v9;
    v10 = &v12;
    v13 = sub_6100;
    v14 = sub_60F0;
    goto LABEL_5;
  }
  if ( !*(char *)(a5 + 55) )
  {
    v8 = sub_6720;
    v9 = sub_6360;
    if ( !v7 && !*(long long *)(a5 + 40) )
      v8 = 0;
    goto LABEL_4;
  }
  v8 = sub_6720;
  v10 = 0;
  if ( !v7 )
  {
    v10 = *(long long (***)(int, int, int, int, int, int, char))(a5 + 40);
    v8 = 0;
    if ( v10 )
    {
      v8 = sub_6720;
      v10 = 0;
    }
  }
LABEL_5:
  if ( (int)(a4 | a2) < 0 )
    return (unsigned int)attr_copy_file(a1, a3, v8, v10, v8, v10, v12, v13, v14, v15) == 0;
  else
    return (unsigned int)attr_copy_fd(a1, a2, a3, a4, v8, v10, v12, v13, v14, v15) == 0;
}



// Function: copy_attr_error @ 0x6360
unsigned long long copy_attr_error()
{
  int v0; // esi
  unsigned long long v2; // [rsp+18h] [rbp-C0h]

  v2 = __readfsqword(0x28u);
  v0 = *__errno_location();
  if ( v0 != 95 && v0 != 61 )
    sub_163B0(0, v0);
  return v2 - __readfsqword(0x28u);
}



// Function: copy_attr_allerror @ 0x6440
unsigned long long copy_attr_allerror()
{
  int *v0; // rax
  unsigned long long v2; // [rsp+18h] [rbp-C0h]

  v2 = __readfsqword(0x28u);
  v0 = __errno_location();
  sub_163B0(0, *v0);
  return v2 - __readfsqword(0x28u);
}



// Function: emit_verbose @ 0x6510
int emit_verbose(long long a1, long long a2, long long a3)
{
  const char *v4; // r12
  const char *v5; // rax
  long long v6; // r12
  char *v7; // rax
  char *IO_write_ptr; // rax

  v4 = (const char *)sub_14310(1, 4, a2);
  v5 = (const char *)sub_14310(0, 4, a1);
  __printf_chk(1, "%s -> %s", v5, v4);
  if ( a3 )
  {
    v6 = sub_14440(4, a3);
    v7 = dcgettext(0, " (backup: %s)", 5);
    __printf_chk(1, v7, v6);
  }
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
  return (int)IO_write_ptr;
}



// Function: create_hard_link @ 0x65d0
long long create_hard_link(
        void *a1,
        unsigned int a2,
        const char *a3,
        void *a4,
        unsigned int a5,
        long long a6,
        unsigned char a7,
        unsigned int a8,
        unsigned char a9)
{
  void *v10; // r14
  unsigned int v13; // r12d
  int v14; // ebp
  void *v16; // r9
  long long v17; // rbx
  long long v18; // r12
  char *v19; // rax
  long long v20; // rcx
  long long v21; // r13
  char *v22; // rax
  void *ptr; // [rsp+10h] [rbp-40h]

  v10 = a1;
  v13 = a8;
  v14 = sub_BE90(a2, a3, a5, a6, a9 << 10, a7, -1);
  if ( v14 > 0 )
  {
    v16 = 0;
    if ( !a1 )
    {
      v10 = sub_6110(a4, a6, a3);
      v16 = v10;
    }
    ptr = v16;
    v17 = sub_14310(1, 4, v10);
    v18 = sub_14310(0, 4, a4);
    v19 = dcgettext(0, "cannot create hard link %s to %s", 5);
    v20 = v18;
    v13 = 0;
    error(0, v14, v19, v20, v17);
    free(ptr);
  }
  else
  {
    LOBYTE(v13) = (v14 < 0) & a8;
    if ( (char)v13 )
    {
      v21 = sub_14440(4, a4);
      v22 = dcgettext(0, "removed %s\n", 5);
      __printf_chk(1, v22, v21);
    }
    else
    {
      return 1;
    }
  }
  return v13;
}



// Function: overwrite_ok @ 0x67a0
long long overwrite_ok(long long a1, long long a2, int a3, const char *a4, long long a5)
{
  long long v5; // r12
  char *v6; // rax
  long long v11; // r13
  char *v12; // rdx
  char v13[12]; // [rsp+Ch] [rbp-3Ch] BYREF
  unsigned long long v14; // [rsp+18h] [rbp-30h]

  v14 = __readfsqword(0x28u);
  if ( (*(int *)(a5 + 24) & 0xF000) == 0xA000 || (unsigned char)sub_16BB0() || !faccessat(a3, a4, 2, 512) )
  {
    sub_14440(4, a2);
    v5 = qword_235A0;
    v6 = dcgettext(0, "%s: overwrite %s? ", 5);
    __fprintf_chk(stderr, 1, v6, v5);
  }
  else
  {
    sub_E880(*(unsigned int *)(a5 + 24), v13);
    v13[10] = 0;
    sub_14440(4, a2);
    v11 = qword_235A0;
    if ( *(char *)(a1 + 24) || (*(int *)(a1 + 20) & 0xFFFF00) != 0 )
      v12 = dcgettext(0, "%s: replace %s, overriding mode %04lo (%s)? ", 5);
    else
      v12 = dcgettext(0, "%s: unwritable %s (mode %04lo, %s); try anyway? ", 5);
    __fprintf_chk(stderr, 1, v12, v11);
  }
  return sub_17440();
}



// Function: create_hole @ 0x6920
long long create_hole(int fd, long long a2, char a3, __off_t a4)
{
  __off_t v4; // r12
  __off_t v6; // rax
  int *v8; // rbx
  long long v9; // r13
  char *v10; // rax
  long long v11; // r13
  char *v12; // r12
  int *v13; // rax

  v4 = a4;
  v6 = lseek(fd, a4, 1);
  if ( v6 < 0 )
  {
    v11 = sub_14440(4, a2);
    v12 = dcgettext(0, "cannot lseek %s", 5);
    v13 = __errno_location();
    error(0, *v13, v12, v11);
    return 0;
  }
  else
  {
    if ( !a3
      || fallocate(fd, 3, v6 - v4, v4) >= 0
      || (v8 = __errno_location(), LOBYTE(v4) = *v8 == 38 || *v8 == 95, (char)v4) )
    {
      LODWORD(v4) = 1;
    }
    else
    {
      v9 = sub_14440(4, a2);
      v10 = dcgettext(0, "error deallocating %s", 5);
      error(0, *v8, v10, v9);
    }
    return (unsigned int)v4;
  }
}



// Function: sparse_copy @ 0x6a30
long long sparse_copy(
        unsigned int a1,
        unsigned int a2,
        void **a3,
        size_t a4,
        size_t a5,
        char a6,
        char a7,
        long long a8,
        long long a9,
        size_t a10,
        long long *a11,
        char *a12)
{
  unsigned int v12; // r13d
  unsigned long long v13; // r12
  long long v14; // r8
  long long v15; // rax
  unsigned int v16; // r15d
  size_t v17; // rdi
  unsigned int v18; // r14d
  size_t v19; // r15
  char *v20; // r11
  size_t v21; // rdx
  ssize_t v22; // rax
  void *v23; // r11
  int *v24; // rbx
  long long v25; // r12
  char *v26; // rax
  unsigned long long v28; // rbp
  unsigned int v29; // ecx
  char *v30; // r14
  size_t v31; // rbx
  unsigned long long v32; // r12
  char *v33; // rsi
  size_t v34; // rdx
  int v35; // eax
  char v36; // dl
  bool v37; // al
  char v38; // r9
  long long v39; // rax
  char v40; // r9
  unsigned int v41; // eax
  bool v42; // zf
  int v43; // eax
  int *v44; // rax
  int v45; // ecx
  int *v46; // rbp
  long long v47; // rbx
  long long v48; // r12
  char *v49; // rax
  long long v50; // r12
  char *v51; // rax
  long long v52; // r13
  char *v53; // r12
  int *v54; // rax
  int v55; // r14d
  char v56; // r8
  __off_t v57; // r12
  char v58; // r15
  char *s1; // [rsp+8h] [rbp-B0h]
  bool s1a; // [rsp+8h] [rbp-B0h]
  char v61; // [rsp+14h] [rbp-A4h]
  int v62; // [rsp+14h] [rbp-A4h]
  void *bufa; // [rsp+18h] [rbp-A0h]
  unsigned char buf; // [rsp+18h] [rbp-A0h]
  char bufb; // [rsp+18h] [rbp-A0h]
  size_t nbytes; // [rsp+20h] [rbp-98h]
  char v69; // [rsp+3Eh] [rbp-7Ah]
  ssize_t v72; // [rsp+50h] [rbp-68h]

  *a12 = 0;
  nbytes = a10;
  *a11 = 0;
  if ( a5 || !a7 )
  {
    if ( !a10 )
      return 1;
  }
  else
  {
    if ( !a10 )
      return 1;
    v12 = a1;
    v13 = a10;
    while ( 1 )
    {
      while ( 1 )
      {
        v14 = 0x7FFFFFFFC0000000LL;
        if ( v13 <= 0x7FFFFFFFC0000000LL )
          v14 = v13;
        v15 = copy_file_range(a1, 0, a2, 0, v14, 0);
        if ( !v15 )
        {
          nbytes = v13;
          if ( *a11 )
            return 1;
          goto LABEL_12;
        }
        if ( v15 < 0 )
          break;
        *a11 += v15;
        v13 -= v15;
        if ( !v13 )
          return 1;
      }
      v44 = __errno_location();
      v45 = *v44;
      v46 = v44;
      if ( *v44 == 38 )
        goto LABEL_71;
      if ( v45 > 26 )
        break;
      if ( v45 > 0 )
      {
        if ( ((1LL << v45) & 0x4440200) != 0 )
        {
LABEL_71:
          nbytes = v13;
          goto LABEL_12;
        }
        if ( v45 == 1 )
        {
          nbytes = v13;
          if ( *a11 )
            goto LABEL_69;
          goto LABEL_12;
        }
      }
      if ( v45 != 4 )
        goto LABEL_69;
    }
    nbytes = v13;
    if ( v45 != 95 )
    {
LABEL_69:
      v16 = 0;
      v47 = sub_14310(1, 4, a9);
      v48 = sub_14310(0, 4, a8);
      v49 = dcgettext(0, "error copying %s to %s", 5);
      error(0, *v46, v49, v48, v47);
      return v16;
    }
  }
LABEL_12:
  v17 = a4;
  if ( a5 )
    v17 = a5;
  v18 = 0;
  v19 = 0;
  while ( 1 )
  {
    v20 = (char *)*a3;
    if ( !*a3 )
      goto LABEL_54;
LABEL_16:
    v21 = a4;
    s1 = v20;
    if ( nbytes <= a4 )
      v21 = nbytes;
    v22 = read(a1, v20, v21);
    v23 = s1;
    v72 = v22;
    if ( v22 >= 0 )
      break;
    v24 = __errno_location();
    if ( *v24 != 4 )
    {
      v16 = 0;
      v25 = sub_14440(4, a8);
      v26 = dcgettext(0, "error reading %s", 5);
      error(0, *v24, v26, v25);
      return v16;
    }
  }
  if ( !v22 )
  {
    v57 = v19;
    v58 = v18;
    v55 = a2;
    v56 = v58;
    goto LABEL_80;
  }
  v28 = v22;
  v29 = v18;
  v30 = s1;
  v31 = v17;
  *a11 += v22;
  s1a = a5 != 0;
  while ( 1 )
  {
    v32 = v28;
    if ( v31 > v28 )
      v31 = v28;
    LOBYTE(v12) = s1a && v31 != 0;
    if ( (char)v12 )
      break;
    if ( v31 != v28 || (char)v29 == 1 )
    {
      v12 = v29;
      if ( v31 )
      {
LABEL_49:
        v19 += v31;
        if ( v19 + 0x8000000000000000LL < v31 )
        {
          v16 = 0;
          v50 = sub_14440(4, a8);
          v51 = dcgettext(0, "overflow reading %s", 5);
          error(0, 0, v51, v50);
          return v16;
        }
        v28 -= v31;
        v30 += v31;
        v29 = v12;
        goto LABEL_51;
      }
    }
LABEL_44:
    v19 += v31;
    v38 = 0;
    buf = v29;
    v62 = 1;
    if ( !(char)v29 )
      goto LABEL_37;
LABEL_45:
    bufb = v38;
    v41 = sub_6920(a2, a9, a6, v19);
    v40 = bufb;
    if ( !(char)v41 )
      return v41;
LABEL_38:
    if ( v62 )
    {
      if ( !v31 )
      {
        if ( v40 )
        {
          v19 = 0;
LABEL_52:
          v42 = nbytes == v72;
          nbytes -= v72;
          *a12 = v12;
          if ( v42 )
          {
            v55 = a2;
            v56 = v12;
            v57 = v19;
LABEL_80:
            if ( v56 )
              return sub_6920(v55, a9, a6, v57);
            return 1;
          }
          v18 = v12;
          v20 = (char *)*a3;
          if ( !*a3 )
          {
LABEL_54:
            v43 = getpagesize();
            v20 = (char *)sub_16BF0(v43, a4);
            *a3 = v20;
          }
          goto LABEL_16;
        }
        v32 = 0;
LABEL_60:
        v23 = v30;
        v29 = v12;
        v30 += v31;
        v28 = v32 - v31;
        v19 = 0;
        goto LABEL_51;
      }
      if ( !v40 )
        goto LABEL_60;
      v19 = v31;
      v29 = v12;
      v23 = v30;
      v31 = 0;
    }
    else
    {
      v23 = v30;
      v28 -= v31;
      v29 = v12;
      v30 += v31;
      v19 = v31;
LABEL_51:
      if ( !v28 )
        goto LABEL_52;
    }
  }
  v33 = v30;
  v34 = v31;
  do
  {
    if ( *v33 )
    {
      v37 = s1a && v31 != 0;
      v36 = v29;
      v12 = 0;
      goto LABEL_32;
    }
    ++v33;
    if ( !--v34 )
    {
      v38 = (v19 != 0) & (v29 ^ 1);
      goto LABEL_48;
    }
  }
  while ( (v34 & 0xF) != 0 );
  bufa = v23;
  v61 = v29;
  v35 = memcmp(v30, v33, v34);
  LOBYTE(v29) = v61;
  v23 = bufa;
  LOBYTE(v12) = v35 == 0;
  v36 = v61 ^ (v35 == 0);
  v37 = v35 != 0;
LABEL_32:
  v38 = v36 & (v19 != 0);
  if ( v31 != v28 || !v37 )
  {
LABEL_48:
    if ( !v38 )
      goto LABEL_49;
    v62 = 0;
    goto LABEL_36;
  }
  if ( !v38 )
  {
    v12 = 0;
    goto LABEL_44;
  }
  v62 = 1;
  v12 = 0;
LABEL_36:
  buf = v29;
  if ( (char)v29 )
    goto LABEL_45;
LABEL_37:
  v69 = v38;
  v39 = sub_111C0(a2, v23, v19);
  v40 = v69;
  if ( v19 == v39 )
    goto LABEL_38;
  v16 = buf;
  v52 = sub_14440(4, a9);
  v53 = dcgettext(0, "error writing %s", 5);
  v54 = __errno_location();
  error(0, *v54, v53, v52);
  return v16;
}



// Function: set_process_security_ctx @ 0x72a0
long long set_process_security_ctx(long long a1, long long a2, long long a3, char a4, long long a5)
{
  unsigned int v6; // r12d
  long long v7; // rdx
  long long v8; // rcx
  int *v9; // r13
  long long v10; // r14
  char *v11; // rax
  long long v13; // rdx
  long long v14; // rcx
  long long v15; // rdx
  long long v16; // rcx
  long long v17; // rdi
  int *v18; // rbx
  long long v19; // r13
  char *v20; // rax
  int v21; // eax
  int *v22; // rbp
  long long v23; // r13
  char *v24; // rax
  long long v25[7]; // [rsp+10h] [rbp-38h] BYREF

  v6 = *(unsigned char *)(a5 + 51);
  v25[1] = __readfsqword(0x28u);
  if ( !(char)v6 )
  {
    LOBYTE(v6) = a4 & (*(long long *)(a5 + 40) != 0);
    if ( !(char)v6 )
      return 1;
    if ( (int)sub_C450() >= 0 )
      return 1;
    v18 = __errno_location();
    if ( *v18 == 95 || *v18 == 61 )
    {
      return 1;
    }
    else
    {
      v19 = sub_14440(4, a2);
      v20 = dcgettext(0, "failed to set default file creation context for %s", 5);
      error(0, *v18, v20, v19);
    }
    return v6;
  }
  if ( !*(char *)(a5 + 49) || *(char *)(a5 + 52) )
  {
    if ( (int)sub_15590(a1, v25) < 0 )
    {
      v9 = __errno_location();
LABEL_5:
      v10 = sub_14440(4, a1);
      v11 = dcgettext(0, "failed to get security context of %s", 5);
      error(0, *v9, v11, v10);
      goto LABEL_6;
    }
    v21 = setfscreatecon(v25[0], v25, v7, v8);
    v17 = v25[0];
    if ( v21 >= 0 )
    {
LABEL_26:
      freecon(v17);
      return v6;
    }
    v22 = __errno_location();
    goto LABEL_30;
  }
  if ( *(char *)(a5 + 55) )
  {
    if ( (int)sub_15590(a1, v25) < 0 )
      goto LABEL_6;
    if ( (int)setfscreatecon(v25[0], v25, v15, v16) < 0 )
      goto LABEL_19;
    goto LABEL_31;
  }
  if ( (int)sub_15590(a1, v25) >= 0 )
  {
    if ( (int)setfscreatecon(v25[0], v25, v13, v14) < 0 )
    {
      v22 = __errno_location();
      if ( *v22 == 95 || *v22 == 61 )
      {
LABEL_19:
        v17 = v25[0];
        if ( *(char *)(a5 + 52) )
        {
          freecon(v25[0]);
          return 0;
        }
        goto LABEL_26;
      }
LABEL_30:
      v23 = sub_14B30(v25[0]);
      v24 = dcgettext(0, "failed to set default file creation context to %s", 5);
      error(0, *v22, v24, v23);
      goto LABEL_19;
    }
LABEL_31:
    v17 = v25[0];
    goto LABEL_26;
  }
  v9 = __errno_location();
  if ( *v9 != 95 && *v9 != 61 )
    goto LABEL_5;
LABEL_6:
  if ( *(char *)(a5 + 52) )
    return 0;
  return v6;
}



// Function: set_file_security_ctx @ 0x7510
long long set_file_security_ctx(long long a1, unsigned char a2, long long a3)
{
  long long v4; // rdi
  unsigned int v5; // r13d
  int *v7; // rbx
  long long v8; // r12
  char *v9; // rax

  v4 = *(long long *)(a3 + 40);
  if ( *(char *)(a3 + 49) && !*(char *)(a3 + 52) )
  {
    if ( *(char *)(a3 + 55) )
      return sub_C6F0(v4, a1, a2);
    v5 = sub_C6F0(v4, a1, a2);
    if ( (char)v5 )
      return 1;
    v7 = __errno_location();
    if ( *v7 == 95 || *v7 == 61 )
      return v5;
  }
  else
  {
    if ( (unsigned char)sub_C6F0(v4, a1, a2) )
      return 1;
    v7 = __errno_location();
  }
  v8 = sub_14310(0, 4, a1);
  v9 = dcgettext(0, "failed to set the security context of %s", 5);
  error(0, *v7, v9, v8);
  return 0;
}



// Function: dest_info_init @ 0x75f0
long long dest_info_init(long long a1)
{
  long long result; // rax

  result = sub_11BB0(61, 0, sub_12460, sub_12430, sub_124D0);
  *(long long *)(a1 + 72) = result;
  if ( !result )
    sub_17150();
  return result;
}



// Function: src_info_init @ 0x7630
long long src_info_init(long long a1)
{
  long long result; // rax

  result = sub_11BB0(61, 0, sub_12410, sub_12430, sub_124D0);
  *(long long *)(a1 + 80) = result;
  if ( !result )
    sub_17150();
  return result;
}



// Function: cp_options_default @ 0x7670
bool cp_options_default(long long a1)
{
  long long v2; // rdi
  __uid_t v3; // eax
  bool result; // al

  v2 = a1 + 8;
  *(long long *)(v2 - 8) = 0;
  *(long long *)(v2 + 72) = 0;
  memset(
    (void *)(v2 & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8 * ((unsigned long long)((unsigned int)a1 - (v2 & 0xFFFFFFF8) + 88) >> 3));
  v3 = geteuid();
  *(int *)(a1 + 64) = -1;
  result = v3 == 0;
  *(char *)(a1 + 27) = result;
  *(char *)(a1 + 26) = result;
  return result;
}



// Function: copy_internal @ 0x76f0
long long copy_internal(
        long long a1,
        void *a2,
        int a3,
        char *a4,
        int a5,
        __off_t a6,
        long long a7,
        const __m128i *a8,
        int a9,
        char *a10,
        struct stat *a11,
        __off_t a12)
{
  char *v12; // r15
  __off_t v13; // rsi
  const __m128i *v14; // r14
  char *v15; // r13
  void *v16; // r12
  char *v17; // rsi
  bool v18; // zf
  int v19; // eax
  size_t p_buf; // r9
  unsigned int v21; // eax
  long long v22; // rbx
  const char *v23; // r12
  int v24; // eax
  char v25; // al
  int v26; // ebx
  int *v27; // r13
  struct stat *v28; // rdx
  long long v29; // rbx
  long long v30; // r13
  char *v31; // rax
  int v32; // eax
  bool v33; // dl
  int v34; // r8d
  const __m128i *v35; // r12
  const __m128i *v36; // r9
  long long v37; // rbx
  const char *v38; // rsi
  long long v39; // r12
  char *v40; // rax
  char v42; // al
  int v43; // edx
  int v44; // ecx
  int v45; // r8d
  int v46; // r9d
  const char *v47; // rax
  int v48; // eax
  char v49; // r9
  char v50; // al
  int v51; // eax
  struct stat *v52; // rdx
  long long v53; // rdi
  char *v54; // rbx
  int *v55; // rax
  int v56; // eax
  long long v57; // r15
  char *v58; // r13
  int *v59; // rax
  int v60; // edx
  long long *v61; // rax
  const char *v62; // rsi
  char *v63; // rax
  long long v64; // rdx
  long long v65; // rcx
  int *v66; // rbx
  char *v67; // rax
  const char *v68; // rcx
  long long v69; // r13
  char *v70; // rdx
  long long v71; // rax
  const char *v72; // rsi
  long long v73; // r13
  char *v74; // r12
  int *v75; // rax
  int v76; // eax
  int v77; // eax
  const char *v78; // rsi
  long long v79; // r13
  long long v80; // r12
  char *v81; // rax
  int v82; // eax
  char *v83; // rax
  __m128i v84; // xmm1
  __m128i v85; // xmm2
  __m128i v86; // xmm3
  __m128i v87; // xmm4
  long long v88; // rax
  char *v89; // rbx
  char *v90; // r14
  void *v91; // r12
  void *v92; // r15
  char v93; // al
  char v94; // r13
  long long v95; // r8
  const char *v96; // rdx
  long long v97; // rdx
  long long v98; // rcx
  int v99; // eax
  short v100; // ax
  int v101; // r13d
  char v102; // al
  int *v103; // r13
  long long v104; // rbx
  long long v105; // r12
  char *v106; // rax
  unsigned char v107; // al
  char v108; // al
  long long v109; // rdx
  long long v110; // rcx
  char v111; // al
  int v112; // eax
  long long v113; // rbx
  long long v114; // r12
  char *v115; // rax
  char *v116; // rax
  char *v117; // rax
  long long v118; // rbx
  char *v119; // rax
  const char *v120; // rcx
  int v121; // eax
  unsigned int v122; // r13d
  int v123; // ecx
  long long v124; // r8
  __off_t v125; // r9
  unsigned int v126; // ebx
  int *v127; // rax
  long long v128; // rcx
  unsigned int v129; // eax
  int v130; // eax
  int v131; // edx
  size_t v132; // rax
  int v133; // r8d
  int *v134; // r8
  int v135; // eax
  char *v136; // rax
  char *v137; // rax
  long long v138; // r13
  char v139; // dl
  size_t v140; // rax
  const char *v141; // r12
  size_t v142; // rax
  char *v143; // r13
  size_t v144; // rax
  char *v145; // r13
  int v146; // r12d
  char *v147; // r13
  const char *v148; // rax
  char *v149; // r12
  size_t v150; // rax
  long long v151; // rcx
  char *v152; // rsi
  short v153; // cx
  signed long long v154; // rcx
  void *v155; // rsp
  size_t v156; // rdx
  void *v157; // rax
  const char *v158; // rsi
  char *v159; // rdx
  char v160; // cl
  long long v161; // r12
  char *v162; // rax
  long long v163; // rdx
  long long v164; // rax
  char *v165; // rax
  long long v166; // rbx
  char *v167; // rax
  char *v168; // rcx
  const char *v169; // rdx
  int v170; // r12d
  void *v171; // r12
  char *v172; // rax
  char *v173; // rax
  const char *v174; // rcx
  int v175; // eax
  __mode_t v176; // edx
  int v177; // eax
  __mode_t v178; // ebx
  int v179; // r13d
  __mode_t v180; // ebx
  long long v181; // r12
  char *v182; // rax
  char *v183; // rbx
  int *v184; // rax
  unsigned int v185; // r10d
  int *v186; // rax
  int v187; // r10d
  char v188; // al
  long long v189; // rbx
  char *v190; // rax
  int v191; // eax
  int v192; // eax
  int v193; // eax
  char v194; // al
  long long v195; // rdx
  long long v196; // rcx
  char v197; // al
  long long v198; // r12
  char *v199; // rax
  ssize_t v200; // rax
  int v201; // r9d
  unsigned int v202; // eax
  char *v203; // rbx
  void *v204; // rdi
  unsigned long long v205; // rax
  char *v206; // rcx
  unsigned long long v207; // rdx
  int v208; // eax
  char v209; // r9
  int v210; // r8d
  char *v211; // rax
  long long v212; // r14
  char *v213; // rax
  int v214; // eax
  char *v215; // rbx
  int v216; // eax
  char *v217; // rax
  char *v218; // rax
  long long v219; // rdx
  int v220; // eax
  int v221; // eax
  const char *v222; // rsi
  __off_t v223; // rax
  __off_t v224; // r12
  signed long long v225; // r14
  long long v226; // r15
  __off_t v227; // rcx
  size_t v228; // r8
  char v229; // di
  __off_t v230; // rax
  signed long long v231; // r15
  __off_t v232; // rax
  int *v233; // rax
  bool v234; // dl
  char v235; // r8
  long long v236; // rax
  __off_t v237; // r8
  const char *v238; // rsi
  char *v239; // rax
  long long v240; // rbx
  char *v241; // rax
  long long v242; // rbx
  long long v243; // rax
  const char *v244; // rdx
  int v245; // eax
  long long v246; // rbx
  char *v247; // rax
  long long v248; // rbx
  char *v249; // rax
  long long v250; // rcx
  long long v251; // r13
  char *v252; // rax
  int v253; // esi
  long long v254; // r13
  int *v255; // rax
  const char *v256; // rdx
  long long v257[2]; // [rsp+1010h] [rbp-13F8h] BYREF
  __dev_t st_dev; // [rsp+1020h] [rbp-13E8h]
  long long v259; // [rsp+1028h] [rbp-13E0h] BYREF
  char v260[7]; // [rsp+1030h] [rbp-13D8h] BYREF
  char v261[8]; // [rsp+2028h] [rbp-3E0h] BYREF
  __off_t v262; // [rsp+2030h] [rbp-3D8h]
  int v263; // [rsp+2038h] [rbp-3D0h]
  int v264; // [rsp+203Ch] [rbp-3CCh]
  size_t st_blksize; // [rsp+2040h] [rbp-3C8h]
  int v266; // [rsp+2048h] [rbp-3C0h]
  __mode_t v267; // [rsp+204Ch] [rbp-3BCh]
  const __m128i *v268; // [rsp+2050h] [rbp-3B8h]
  __off_t v269; // [rsp+2058h] [rbp-3B0h]
  char *s; // [rsp+2060h] [rbp-3A8h]
  long long *v271; // [rsp+2068h] [rbp-3A0h]
  const __m128i *v272; // [rsp+2070h] [rbp-398h]
  __off_t offset; // [rsp+2078h] [rbp-390h]
  char *p_ptr; // [rsp+2080h] [rbp-388h]
  void *s2; // [rsp+2088h] [rbp-380h]
  char *v276; // [rsp+2090h] [rbp-378h]
  __off_t st_size; // [rsp+2098h] [rbp-370h]
  struct stat *v278; // [rsp+20A0h] [rbp-368h]
  signed long long v279; // [rsp+20A8h] [rbp-360h]
  long long v280; // [rsp+20B0h] [rbp-358h]
  char *format; // [rsp+20B8h] [rbp-350h]
  size_t n; // [rsp+20C0h] [rbp-348h]
  long long v283; // [rsp+20C8h] [rbp-340h]
  char *name; // [rsp+20D0h] [rbp-338h]
  void *src; // [rsp+20D8h] [rbp-330h]
  int fd; // [rsp+20E0h] [rbp-328h]
  __mode_t st_mode; // [rsp+20E4h] [rbp-324h]
  struct stat *v288; // [rsp+20E8h] [rbp-320h]
  size_t v289; // [rsp+20F0h] [rbp-318h]
  char v290; // [rsp+20FEh] [rbp-30Ah] BYREF
  char v291; // [rsp+20FFh] [rbp-309h] BYREF
  void *ptr; // [rsp+2100h] [rbp-308h] BYREF
  __off_t length[2]; // [rsp+2108h] [rbp-300h] BYREF
  __m128i v294; // [rsp+2118h] [rbp-2F0h]
  __m128i v295; // [rsp+2128h] [rbp-2E0h]
  __m128i v296; // [rsp+2138h] [rbp-2D0h]
  __m128i v297; // [rsp+2148h] [rbp-2C0h]
  long long v298; // [rsp+2158h] [rbp-2B0h]
  struct timespec v299[2]; // [rsp+2168h] [rbp-2A0h] BYREF
  struct stat buf; // [rsp+2188h] [rbp-280h] BYREF
  struct stat v301; // [rsp+2218h] [rbp-1F0h] BYREF
  struct stat v302; // [rsp+22A8h] [rbp-160h] BYREF
  struct stat v303; // [rsp+2338h] [rbp-D0h] BYREF
  char v304; // [rsp+23CFh] [rbp-39h] BYREF
  unsigned long long v305; // [rsp+23D0h] [rbp-38h]

  v12 = (char *)a1;
  src = a2;
  v13 = a12;
  name = a4;
  v280 = a7;
  fd = a3;
  st_size = a6;
  v14 = a8;
  LODWORD(v288) = a9;
  format = a10;
  LODWORD(v15) = a8[4].m128i_i32[0];
  v278 = a11;
  v283 = a12;
  LOBYTE(v279) = a9;
  v305 = __readfsqword(0x28u);
  LOBYTE(a11->st_dev) = 0;
  LOBYTE(v289) = a8[1].m128i_i8[8];
  if ( (char)v289 )
  {
    if ( (int)v15 < 0 )
    {
      v13 = a1;
      a1 = 4294967196LL;
      if ( !(unsigned int)sub_14B50(-100, (char *)v13, fd, name) )
      {
        a5 = 1;
        *(char *)v283 = 1;
        goto LABEL_34;
      }
      LODWORD(v15) = *__errno_location();
    }
    LOBYTE(v289) = (int)v15 == 0;
    a5 = (int)v15 == 0;
    *(char *)v283 = a5;
    if ( (int)v15 )
      goto LABEL_3;
  }
  else
  {
    LOBYTE(v289) = a5 > 0;
    if ( (int)v15 )
    {
LABEL_3:
      if ( (int)v15 != 17 || a8->m128i_i32[2] != 2 )
      {
        v16 = v12;
        v17 = v12;
        a1 = 4294967196LL;
LABEL_5:
        v18 = a8->m128i_i32[1] == 2;
        st_mode = a5;
        v19 = fstatat(a1, v17, &buf, v18 << 8);
        a5 = st_mode;
        v13 = (__off_t)v16;
        if ( v19 )
        {
LABEL_181:
          v71 = sub_14440(4, v13);
          v72 = "cannot stat %s";
          v73 = v71;
          goto LABEL_182;
        }
        st_mode = buf.st_mode;
        if ( (buf.st_mode & 0xF000) == 0x4000 )
        {
          LODWORD(v23) = a8[3].m128i_u8[8];
          if ( !(char)v23 )
          {
            v69 = sub_14440(4, v12);
            if ( !a8[1].m128i_i8[9] )
            {
              v70 = dcgettext(0, "-r not specified; omitting directory %s", 5);
              error(0, 0, v70, v69);
              return (unsigned int)v23;
            }
            v158 = "omitting directory %s";
LABEL_390:
            v159 = dcgettext(0, v158, 5);
            error(0, 0, v159, v69);
            return (unsigned int)v23;
          }
        }
        goto LABEL_7;
      }
      goto LABEL_7;
    }
  }
LABEL_34:
  if ( !a8[3].m128i_i8[15] )
  {
    a1 = (unsigned int)fd;
    v16 = src;
    LODWORD(v15) = 0;
    v17 = name;
    goto LABEL_5;
  }
  LODWORD(v15) = 0;
LABEL_7:
  if ( (char)v288 )
  {
    a1 = a8[5].m128i_i64[0];
    if ( a1 )
    {
      p_buf = (size_t)&buf;
      if ( (st_mode & 0xF000) != 0x4000 && !a8->m128i_i32[0] )
      {
        LODWORD(v276) = a5;
        n = (size_t)&buf;
        LODWORD(v23) = sub_E820(a1, v12, &buf);
        if ( (char)v23 )
        {
          v158 = "warning: source file %s specified more than once";
          v69 = sub_14440(4, v12);
          goto LABEL_390;
        }
        a1 = a8[5].m128i_i64[0];
        p_buf = n;
        a5 = (int)v276;
      }
      v13 = (__off_t)v12;
      LODWORD(n) = a5;
      sub_E790(a1, v12, p_buf);
      a5 = n;
    }
  }
  v21 = a8->m128i_u32[1];
  if ( v21 == 4 )
  {
    LODWORD(v276) = 1;
LABEL_16:
    n = 0;
    LOBYTE(v22) = 0;
    if ( a5 > 0 )
      goto LABEL_17;
LABEL_60:
    if ( (int)v15 == 17 && a8->m128i_i32[2] == 2 )
    {
      LOBYTE(v22) = 0;
      LOBYTE(v15) = 0;
LABEL_223:
      if ( !a8[3].m128i_i8[11] )
        goto LABEL_236;
      if ( (st_mode & 0xF000) == 0x4000 )
      {
        if ( !a8[1].m128i_i8[8] )
          goto LABEL_240;
      }
      else
      {
        v95 = 0;
        if ( a8[1].m128i_i8[15] )
        {
          v95 = 1;
          if ( a8[1].m128i_i8[8] )
            v95 = v301.st_dev != buf.st_dev;
        }
        v13 = (__off_t)name;
        a1 = (unsigned int)fd;
        if ( (int)sub_15B50((unsigned int)fd, name, &v301, &buf, v95) >= 0 )
        {
          if ( v283 )
            *(char *)v283 = 1;
          v96 = (const char *)sub_BD30(name);
          if ( v96 )
          {
            v13 = (unsigned int)fd;
            if ( !(unsigned char)sub_65D0(
                                     0,
                                     fd,
                                     v96,
                                     src,
                                     fd,
                                     (long long)name,
                                     1u,
                                     a8[3].m128i_u8[12],
                                     (unsigned char)v276) )
            {
              if ( a8[3].m128i_i8[3] )
              {
                a1 = 0;
                if ( (unsigned int)setfscreatecon(0, (unsigned int)fd, v97, v98) )
                  goto LABEL_235;
              }
              goto LABEL_78;
            }
          }
LABEL_89:
          LODWORD(v23) = 1;
          return (unsigned int)v23;
        }
        if ( !a8[1].m128i_i8[8] )
          goto LABEL_238;
      }
LABEL_300:
      v112 = v14->m128i_i32[2];
      if ( v112 != 2 )
      {
        if ( v112 != 3 )
        {
          if ( v112 != 4 || !v14[3].m128i_i8[13] )
            goto LABEL_240;
          v100 = v301.st_mode;
          if ( (v301.st_mode & 0xF000) == 0xA000 )
          {
            if ( (char)v15 )
              goto LABEL_89;
            goto LABEL_242;
          }
          if ( (unsigned char)sub_16BB0(a1, v13) || !faccessat(fd, name, 2, 512) )
            goto LABEL_240;
        }
        if ( (unsigned char)sub_67A0((long long)v14, (long long)src, fd, name, (long long)&v301) )
          goto LABEL_240;
      }
LABEL_308:
      if ( v283 )
        *(char *)v283 = 1;
      goto LABEL_89;
    }
    v32 = st_mode & 0xF000;
    if ( v32 == 0x8000 || (LOBYTE(v22) = a8[1].m128i_i8[4] ^ 1 | (v32 == 0x4000 || v32 == 40960), !(char)v22) )
    {
      LOBYTE(v22) = a8[1].m128i_i8[8];
      if ( !(char)v22 )
      {
        LOBYTE(v22) = a8[3].m128i_i8[10];
        if ( !(char)v22 )
        {
          LOBYTE(v22) = a8[1].m128i_i8[7];
          if ( !(char)v22 )
          {
            a1 = a8->m128i_u32[0];
            if ( (int)a1 )
            {
              LOBYTE(v22) = 1;
            }
            else
            {
              LOBYTE(v22) = a8[1].m128i_i8[5];
              if ( !(char)v22 )
              {
                if ( a5 )
                  goto LABEL_171;
LABEL_64:
                v13 = (__off_t)name;
                a1 = (unsigned int)fd;
                if ( !fstatat(fd, name, &v301, a5) )
                  goto LABEL_65;
                v66 = __errno_location();
                if ( *v66 == 40 )
                {
                  if ( a8[1].m128i_i8[6] )
                  {
LABEL_172:
                    n = 0;
                    LOBYTE(v22) = 0;
                    if ( (int)v15 != 17 )
                      goto LABEL_17;
LABEL_65:
                    if ( a8->m128i_i32[2] == 2 )
                      goto LABEL_222;
                    if ( buf.st_ino == v301.st_ino && buf.st_dev == v301.st_dev )
                    {
                      LOBYTE(v15) = a8[1].m128i_i8[7];
                      if ( (char)v15 )
                        goto LABEL_223;
                      if ( a8->m128i_i32[1] != 2 )
                      {
                        v35 = (const __m128i *)&v302;
                        a1 = (unsigned int)fd;
                        v13 = (__off_t)name;
                        if ( fstatat(fd, name, &v302, 256) )
                          goto LABEL_223;
                        a1 = (long long)v12;
                        v13 = (__off_t)&v303;
                        n = (size_t)&v303;
                        if ( lstat(v12, &v303) )
                          goto LABEL_223;
                        v33 = 0;
                        if ( v303.st_ino == v302.st_ino )
                          v33 = v303.st_dev == v302.st_dev;
                        v36 = (const __m128i *)n;
                        if ( (v303.st_mode & 0xF000) == 0xA000 && (v302.st_mode & 0xF000) == 0xA000 && a8[1].m128i_i8[5] )
                          goto LABEL_223;
                        goto LABEL_70;
                      }
                      v33 = 1;
                      v34 = 1;
                    }
                    else
                    {
                      if ( a8->m128i_i32[1] != 2 )
                        goto LABEL_222;
                      v33 = 0;
                      v34 = 0;
                    }
                    v35 = (const __m128i *)&v301;
                    v36 = (const __m128i *)&buf;
                    if ( (buf.st_mode & 0xF000) == 0xA000 && (v301.st_mode & 0xF000) == 0xA000 )
                    {
                      v13 = (__off_t)v12;
                      a1 = 4294967196LL;
                      LODWORD(n) = v34;
                      LOBYTE(v15) = sub_14E20(-100);
                      if ( (char)v15 )
                        goto LABEL_76;
                      if ( a8->m128i_i32[0] || !(int)n )
                        goto LABEL_223;
                      LOBYTE(v15) = 1;
                      LOBYTE(v170) = a8[1].m128i_i8[8] ^ 1;
                      goto LABEL_465;
                    }
LABEL_70:
                    v13 = a8->m128i_u32[0];
                    if ( (int)v13 )
                    {
                      if ( !v33 )
                      {
                        if ( !a8[1].m128i_i8[8]
                          && a8->m128i_i32[1] != 2
                          && (v36[1].m128i_i32[2] & 0xF000) == 0xA000
                          && (v35[1].m128i_i32[2] & 0xF000) != 0xA000 )
                        {
                          goto LABEL_76;
                        }
LABEL_222:
                        LOBYTE(v15) = 0;
                        goto LABEL_223;
                      }
                      v13 = (__off_t)v12;
                      a1 = 4294967196LL;
                      LOBYTE(v15) = 0;
                      v170 = sub_14E20(-100) ^ 1;
                      goto LABEL_465;
                    }
                    v160 = a8[1].m128i_i8[8];
                    if ( v160 || a8[1].m128i_i8[5] )
                    {
                      if ( (v35[1].m128i_i32[2] & 0xF000) == 0xA000 )
                        goto LABEL_222;
                      if ( !v33 || v35[1].m128i_i64[0] <= 1uLL )
                      {
                        if ( (v36[1].m128i_i32[2] & 0xF000) == 0xA000 )
                          goto LABEL_402;
                        goto LABEL_432;
                      }
                      v13 = (__off_t)v12;
                      a1 = 4294967196LL;
                      n = (size_t)v36;
                      v194 = sub_14E20(-100);
                      v36 = (const __m128i *)n;
                      LOBYTE(v15) = v194;
                      if ( !v194 )
                      {
                        LOBYTE(v170) = a8[1].m128i_i8[8] ^ 1;
LABEL_465:
                        if ( (char)v170 )
                          goto LABEL_223;
                        goto LABEL_76;
                      }
                      if ( (*(int *)(n + 24) & 0xF000) == 0xA000 )
                        goto LABEL_401;
                    }
                    else if ( (v36[1].m128i_i32[2] & 0xF000) == 0xA000 )
                    {
                      goto LABEL_404;
                    }
                    if ( (v35[1].m128i_i32[2] & 0xF000) == 0xA000 )
                    {
LABEL_401:
                      v160 = a8[1].m128i_i8[8];
LABEL_402:
                      if ( v160 )
                      {
                        if ( (buf.st_mode & 0xF000) == 0xA000 && v35[1].m128i_i64[0] > 1uLL )
                        {
                          a1 = (long long)v12;
                          n = (size_t)v36;
                          v211 = canonicalize_file_name(v12);
                          v36 = (const __m128i *)n;
                          if ( v211 )
                          {
                            v13 = (__off_t)v211;
                            a1 = (long long)v211;
                            LOBYTE(v15) = 0;
                            v170 = sub_14E20(-100) ^ 1;
                            free((void *)a1);
                            goto LABEL_465;
                          }
                        }
                      }
LABEL_404:
                      if ( a8[3].m128i_i8[10] && (v35[1].m128i_i32[2] & 0xF000) == 0xA000 )
                        goto LABEL_222;
                      if ( a8->m128i_i32[1] == 2 )
                      {
                        if ( (v36[1].m128i_i32[2] & 0xF000) == 0xA000 )
                        {
                          v13 = (__off_t)&v303;
                          a1 = (long long)v12;
                          if ( stat(v12, &v303) )
                            goto LABEL_222;
                        }
                        else
                        {
                          *(__m128i *)&v303.st_dev = _mm_load_si128(v36);
                          *(__m128i *)&v303.st_nlink = _mm_load_si128(v36 + 1);
                          *(__m128i *)&v303.st_gid = _mm_load_si128(v36 + 2);
                          *(__m128i *)&v303.st_size = _mm_load_si128(v36 + 3);
                          *(__m128i *)&v303.st_blocks = _mm_load_si128(v36 + 4);
                          *(__m128i *)&v303.st_atim.tv_nsec = _mm_load_si128(v36 + 5);
                          *(__m128i *)&v303.st_mtim.tv_nsec = _mm_load_si128(v36 + 6);
                          *(__m128i *)&v303.st_ctim.tv_nsec = _mm_load_si128(v36 + 7);
                          *(__m128i *)&v303.__unused[1] = _mm_load_si128(v36 + 8);
                        }
                        if ( (v35[1].m128i_i32[2] & 0xF000) == 0xA000 )
                        {
                          v13 = (__off_t)name;
                          a1 = (unsigned int)fd;
                          if ( fstatat(fd, name, &v302, 0) )
                            goto LABEL_222;
                        }
                        else
                        {
                          *(__m128i *)&v302.st_dev = _mm_load_si128(v35);
                          *(__m128i *)&v302.st_nlink = _mm_load_si128(v35 + 1);
                          *(__m128i *)&v302.st_gid = _mm_load_si128(v35 + 2);
                          *(__m128i *)&v302.st_size = _mm_load_si128(v35 + 3);
                          *(__m128i *)&v302.st_blocks = _mm_load_si128(v35 + 4);
                          *(__m128i *)&v302.st_atim.tv_nsec = _mm_load_si128(v35 + 5);
                          *(__m128i *)&v302.st_mtim.tv_nsec = _mm_load_si128(v35 + 6);
                          *(__m128i *)&v302.st_ctim.tv_nsec = _mm_load_si128(v35 + 7);
                          *(__m128i *)&v302.__unused[1] = _mm_load_si128(v35 + 8);
                        }
                        if ( v303.st_ino != v302.st_ino || v303.st_dev != v302.st_dev )
                          goto LABEL_222;
                        if ( a8[1].m128i_i8[7] )
                        {
                          LOBYTE(v15) = (v35[1].m128i_i32[2] & 0xF000) != 40960;
                          goto LABEL_223;
                        }
                      }
LABEL_76:
                      v37 = sub_14310(1, 4, src);
                      v38 = "%s and %s are the same file";
                      v39 = sub_14310(0, 4, v12);
LABEL_77:
                      v40 = dcgettext(0, v38, 5);
                      error(0, 0, v40, v39, v37);
                      goto LABEL_78;
                    }
LABEL_432:
                    if ( v36->m128i_i64[1] != v35->m128i_i64[1] || v36->m128i_i64[0] != v35->m128i_i64[0] )
                      goto LABEL_222;
                    LOBYTE(v15) = a8[1].m128i_i8[7];
                    if ( (char)v15 )
                      goto LABEL_223;
                    goto LABEL_401;
                  }
                }
                else if ( *v66 == 2 )
                {
LABEL_171:
                  LOBYTE(v289) = 1;
                  goto LABEL_172;
                }
                v23 = (const char *)sub_14440(4, src);
                v67 = dcgettext(0, "cannot stat %s", 5);
                v68 = v23;
                LODWORD(v23) = 0;
                error(0, *v66, v67, v68);
                return (unsigned int)v23;
              }
            }
          }
        }
      }
    }
    a5 = 256;
    goto LABEL_64;
  }
  if ( v21 != 3 || !(char)v288 )
  {
    LODWORD(v276) = 0;
    goto LABEL_16;
  }
  LODWORD(v276) = 1;
  if ( a5 <= 0 )
    goto LABEL_60;
  n = 0;
  LOBYTE(v22) = 0;
LABEL_53:
  if ( v14[4].m128i_i64[1] )
  {
    LODWORD(v23) = v14[1].m128i_u8[8];
    if ( !(char)v23 )
    {
      if ( !v14->m128i_i32[0] )
      {
        v28 = &v301;
        if ( !(char)v22 )
        {
          v22 = (long long)&v303;
          if ( fstatat(fd, name, &v303, 256) )
            goto LABEL_18;
          v28 = &v303;
        }
        if ( (v28->st_mode & 0xF000) == 0xA000 && (unsigned char)sub_E820(v14[4].m128i_i64[1], name, v28) )
        {
          v29 = sub_14310(1, 4, src);
          v30 = sub_14310(0, 4, v12);
          v31 = dcgettext(0, "will not copy %s through just-created symlink %s", 5);
          error(0, 0, v31, v30, v29);
          return (unsigned int)v23;
        }
        goto LABEL_18;
      }
      if ( v14[3].m128i_i8[12] )
      {
LABEL_147:
        if ( (st_mode & 0xF000) != 0x4000 )
          sub_6510((long long)v12, (long long)src, n);
        goto LABEL_20;
      }
      if ( (int)v15 )
        goto LABEL_21;
      goto LABEL_92;
    }
  }
  while ( 1 )
  {
LABEL_18:
    if ( v14[3].m128i_i8[12] && !v14[1].m128i_i8[8] )
      goto LABEL_147;
LABEL_20:
    if ( !(int)v15 )
    {
      if ( v14[1].m128i_i8[8] )
      {
LABEL_81:
        if ( v14[3].m128i_i8[12] )
        {
          v116 = dcgettext(0, "renamed ", 5);
          __printf_chk(1, v116);
          sub_6510((long long)v12, (long long)src, n);
        }
        if ( v14[2].m128i_i64[1] )
          sub_7510((long long)src, 1u, (long long)v14);
        if ( v283 )
          *(char *)v283 = 1;
        if ( (char)v288 && !v14[3].m128i_i8[15] )
        {
          sub_E790(v14[4].m128i_i64[1], name, &buf);
          LODWORD(v23) = (unsigned char)v288;
          return (unsigned int)v23;
        }
        goto LABEL_89;
      }
LABEL_92:
      LODWORD(v15) = (unsigned char)v289;
      v23 = 0;
      goto LABEL_93;
    }
LABEL_21:
    if ( v14[3].m128i_i8[8] && (st_mode & 0xF000) == 0x4000 )
    {
      if ( (char)v288 )
        v23 = (const char *)sub_BD30(name);
      else
        v23 = (const char *)sub_BCD0(buf.st_ino, buf.st_dev);
      if ( v23 )
      {
LABEL_26:
        if ( (unsigned char)sub_14E20(-100) )
        {
          v22 = sub_14310(1, 4, qword_23568);
          v161 = sub_14310(0, 4, qword_23570);
          v162 = dcgettext(0, "cannot copy a directory, %s, into itself, %s", 5);
          v13 = 0;
          error(0, 0, v162, v161, v22);
          LOBYTE(v278->st_dev) = 1;
LABEL_281:
          if ( v14[3].m128i_i8[3] )
          {
            a1 = 0;
            if ( (unsigned int)setfscreatecon(0, v13, v109, v110) )
              goto LABEL_235;
          }
LABEL_194:
          if ( !n )
            goto LABEL_78;
          if ( !renameat(fd, (const char *)(n + name - (char *)src), fd, name) )
          {
            if ( v14[3].m128i_i8[12] )
            {
              v79 = sub_14310(1, 4, src);
              v80 = sub_14310(0, 4, n);
              v81 = dcgettext(0, "%s -> %s (unbackup)\n", 5);
              __printf_chk(1, v81, v80, v79);
            }
            goto LABEL_78;
          }
          v72 = "cannot un-backup %s";
          v73 = sub_14440(4, src);
LABEL_182:
          v74 = dcgettext(0, v72, 5);
          v75 = __errno_location();
          error(0, *v75, v74, v73);
          goto LABEL_78;
        }
        if ( (unsigned char)sub_14E20(fd) )
        {
          v181 = sub_14440(4, qword_23570);
          v182 = dcgettext(0, "warning: source directory %s specified more than once", 5);
          error(0, 0, v182, v181);
          if ( !v14[1].m128i_i8[8] )
            goto LABEL_89;
          goto LABEL_308;
        }
        v24 = v14->m128i_i32[1];
        if ( v24 != 4 && (v24 != 3 || !(char)v288) )
        {
          v12 = (char *)src;
          v171 = sub_6110(src, (long long)name, v23);
          v22 = sub_14310(1, 4, v171);
          v15 = (char *)sub_14310(0, 4, v12);
          v172 = dcgettext(0, "will not create hard link %s to directory %s", 5);
          v13 = 0;
          error(0, 0, v172, v15, v22);
          free(v171);
          goto LABEL_281;
        }
        goto LABEL_31;
      }
      goto LABEL_31;
    }
    v25 = v14[1].m128i_i8[8];
    if ( v25 )
    {
      if ( buf.st_nlink == 1 )
      {
        v23 = (const char *)sub_BCD0(buf.st_ino, buf.st_dev);
LABEL_278:
        if ( v23 )
        {
          if ( (st_mode & 0xF000) != 0x4000 )
          {
            v13 = (unsigned int)fd;
            if ( (unsigned char)sub_65D0(
                                    0,
                                    fd,
                                    v23,
                                    src,
                                    fd,
                                    (long long)name,
                                    1u,
                                    v14[3].m128i_u8[12],
                                    (unsigned char)v276) )
              goto LABEL_89;
            goto LABEL_281;
          }
          goto LABEL_26;
        }
LABEL_31:
        v25 = v14[1].m128i_i8[8];
        goto LABEL_145;
      }
      if ( !v14[3].m128i_i8[0] || v14[1].m128i_i8[7] )
      {
        v23 = 0;
        goto LABEL_44;
      }
    }
    else
    {
      if ( !v14[3].m128i_i8[0] )
        goto LABEL_92;
      if ( v14[1].m128i_i8[7] )
      {
        v23 = 0;
LABEL_146:
        LODWORD(v15) = (unsigned char)v289;
        goto LABEL_93;
      }
    }
    if ( buf.st_nlink > 1 || (v60 = v14->m128i_i32[1], (char)v288) && v60 == 3 || (v23 = 0, v60 == 4) )
    {
      v23 = (const char *)sub_BD30(name);
      goto LABEL_278;
    }
LABEL_145:
    if ( !v25 )
      goto LABEL_146;
LABEL_44:
    if ( (int)v15 == 17 )
    {
      if ( !renameat(-100, v12, fd, name) )
        goto LABEL_81;
      LODWORD(v15) = *__errno_location();
      if ( !(int)v15 )
        goto LABEL_81;
    }
    if ( (int)v15 == 22 )
    {
      v118 = sub_14310(1, 4, qword_23568);
      v23 = (const char *)sub_14310(0, 4, qword_23570);
      v119 = dcgettext(0, "cannot move %s to a subdirectory of itself, %s", 5);
      v120 = v23;
      LODWORD(v23) = 1;
      error(0, 0, v119, v120, v118);
      LOBYTE(v278->st_dev) = 1;
      return (unsigned int)v23;
    }
    if ( (int)v15 != 18 )
    {
      v104 = sub_14310(1, 4, src);
      v105 = sub_14310(0, 4, v12);
      v106 = dcgettext(0, "cannot move %s to %s", 5);
      error(0, (int)v15, v106, v105, v104);
      goto LABEL_266;
    }
    v26 = st_mode & 0xF000;
    if ( unlinkat(fd, name, (v26 == 0x4000) << 9) )
    {
      v27 = __errno_location();
      if ( *v27 != 2 )
      {
        v113 = sub_14310(1, 4, src);
        v114 = sub_14310(0, 4, v12);
        v115 = dcgettext(0, "inter-device move failed: %s to %s; unable to remove target", 5);
        error(0, *v27, v115, v114, v113);
LABEL_266:
        sub_BC60(buf.st_ino, buf.st_dev);
LABEL_78:
        LODWORD(v23) = 0;
        return (unsigned int)v23;
      }
    }
    LOBYTE(v289) = v14[3].m128i_i8[12] & (v26 != 0x4000);
    if ( (char)v289 )
    {
      LODWORD(v15) = 1;
      v117 = dcgettext(0, "copied ", 5);
      __printf_chk(1, v117);
      sub_6510((long long)v12, (long long)src, n);
    }
    else
    {
      LOBYTE(v289) = 1;
      LODWORD(v15) = 1;
    }
LABEL_93:
    LODWORD(v22) = st_mode & 0xFFF;
    if ( v14[3].m128i_i8[9] )
      LODWORD(v22) = v14[1].m128i_i32[0] & 0xFFF;
    if ( !v14[1].m128i_i8[13] )
    {
      LODWORD(v283) = st_mode & 0xF000;
      if ( (int)v283 != 0x4000 )
      {
        v42 = sub_72A0((long long)v12, (long long)src, st_mode, (char)v15, (long long)v14);
        LODWORD(s2) = 0;
        if ( !v42 )
          goto LABEL_78;
        goto LABEL_102;
      }
      if ( !(unsigned char)sub_72A0((long long)v12, (long long)src, st_mode, (char)v15, (long long)v14) )
        goto LABEL_78;
      LODWORD(s2) = v22 & 0x12;
LABEL_155:
      v61 = (long long *)v280;
      if ( v280 )
      {
        while ( v61[1] != buf.st_ino || v61[2] != buf.st_dev )
        {
          v61 = (long long *)*v61;
          if ( !v61 )
            goto LABEL_198;
        }
        v62 = "cannot copy cyclic symbolic link %s";
        v15 = (char *)sub_14440(4, v12);
        goto LABEL_161;
      }
LABEL_198:
      v271 = v257;
      v257[0] = v280;
      v257[1] = buf.st_ino;
      st_dev = buf.st_dev;
      if ( !(char)v289 && (v301.st_mode & 0xF000) == 0x4000 )
      {
        if ( v14[2].m128i_i64[1] || v14[3].m128i_i8[3] )
        {
          v13 = 0;
          if ( !(unsigned char)sub_7510((long long)src, 0, (long long)v14) )
          {
            if ( v14[3].m128i_i8[4] )
              goto LABEL_191;
          }
        }
        LOBYTE(v269) = 0;
        LODWORD(s2) = 0;
      }
      else
      {
        if ( mkdirat(fd, name, v22 & ~(int)s2) )
        {
          v78 = "cannot create directory %s";
          v12 = (char *)sub_14440(4, src);
          goto LABEL_190;
        }
        if ( fstatat(fd, name, &v301, 256) )
        {
          v78 = "cannot stat %s";
          v12 = (char *)sub_14440(4, src);
          goto LABEL_190;
        }
        LODWORD(v22) = v301.st_mode;
        LOBYTE(v269) = 0;
        if ( (v301.st_mode & 0x1C0) != 0x1C0 )
        {
          v82 = fchmodat(fd, name, v301.st_mode | 0x1C0, 256);
          v267 = v22;
          LOBYTE(v269) = 1;
          if ( v82 )
          {
            v78 = "setting permissions for %s";
            v12 = (char *)sub_14440(4, src);
            goto LABEL_190;
          }
        }
        if ( !*format )
        {
          sub_BD30(name);
          *format = 1;
        }
        if ( v14[3].m128i_i8[12] )
        {
          if ( v14[1].m128i_i8[8] )
          {
            v23 = (const char *)sub_14440(4, src);
            v83 = dcgettext(0, "created directory %s\n", 5);
            __printf_chk(1, v83, v23);
          }
          else
          {
            sub_6510((long long)v12, (long long)src, 0);
          }
        }
      }
      LOBYTE(v23) = v14[1].m128i_i8[12] & (st_size != 0);
      if ( !(char)v23 || *(long long *)st_size == buf.st_dev )
      {
        v84 = _mm_loadu_si128(v14 + 1);
        v85 = _mm_loadu_si128(v14 + 2);
        v86 = _mm_loadu_si128(v14 + 3);
        v87 = _mm_loadu_si128(v14 + 4);
        v88 = v14[5].m128i_i64[0];
        *(__m128i *)length = _mm_loadu_si128(v14);
        v294 = v84;
        v298 = v88;
        v295 = v85;
        v296 = v86;
        v297 = v87;
        s = (char *)sub_15300(v12, 2);
        if ( s )
        {
          if ( v14->m128i_i32[1] == 3 )
            HIDWORD(length[0]) = 2;
          if ( *s )
          {
            v279 = name - (char *)src;
            v89 = s;
            p_ptr = (char *)&ptr;
            st_size = (__off_t)&v290;
            v276 = &v291;
            v268 = v14;
            v90 = v278;
            LOBYTE(v280) = 0;
            offset = (__off_t)length;
            LODWORD(v272) = (int)v15;
            LOBYTE(v283) = 1;
            n = (size_t)v12;
            v278 = &buf;
            do
            {
              v91 = (void *)sub_EA10(n, v89, 0);
              v92 = (void *)sub_EA10(src, v89, 0);
              v291 = *format;
              v93 = copy_internal(
                      (int)v91,
                      (int)v92,
                      fd,
                      (int)v92 + (int)v279,
                      (int)v272,
                      (int)v278,
                      (long long)v271,
                      offset,
                      0,
                      (long long)v276,
                      st_size,
                      (long long)p_ptr);
              v94 = v290;
              *v90 |= v290;
              LOBYTE(v283) = v93 & v283;
              free(v92);
              free(v91);
              if ( v94 )
                break;
              LOBYTE(v280) = v291 | v280;
              v89 += strlen(v89) + 1;
            }
            while ( *v89 );
            LODWORD(v23) = (unsigned char)v283;
            v12 = (char *)n;
            v14 = v268;
          }
          else
          {
            LOBYTE(v280) = 0;
            LODWORD(v23) = 1;
          }
          free(s);
          *format = v280;
        }
        else
        {
          v254 = sub_14440(4, v12);
          v23 = dcgettext(0, "cannot access %s", 5);
          v255 = __errno_location();
          v256 = v23;
          LODWORD(v23) = 0;
          error(0, *v255, v256, v254);
        }
      }
      if ( (char)v288 )
      {
        v49 = 0;
        LOBYTE(v15) = 0;
        LOBYTE(v22) = 0;
        LOBYTE(v279) = v269;
LABEL_113:
        if ( v14[4].m128i_i64[1] )
        {
          LOBYTE(v283) = v49;
          v288 = &v303;
          v51 = fstatat(fd, name, &v303, 256);
          v52 = v288;
          v49 = v283;
          if ( !v51 )
          {
            v53 = v14[4].m128i_i64[1];
            LOBYTE(v288) = v283;
            sub_E790(v53, name, v52);
            v49 = (char)v288;
          }
        }
LABEL_116:
        if ( v14[1].m128i_i8[7] && (char)v22 || v49 )
          return (unsigned int)v23;
      }
      else
      {
        LOBYTE(v15) = 0;
        LOBYTE(v279) = v269;
      }
      if ( v14[1].m128i_i8[15] )
      {
        v303.st_dev = buf.st_atim.tv_sec;
        v303.st_ino = buf.st_atim.tv_nsec;
        *(struct timespec *)&v303.st_nlink = buf.st_mtim;
        if ( utimensat(fd, name, (const struct timespec *)&v303, (unsigned char)v15 << 8) )
        {
          v288 = (struct stat *)sub_14440(4, src);
          v54 = dcgettext(0, "preserving times for %s", 5);
          v55 = __errno_location();
          error(0, *v55, v54, v288);
          if ( v14[3].m128i_i8[2] )
            goto LABEL_78;
        }
      }
      if ( (char)v15 )
      {
        if ( !v14[3].m128i_i8[5]
          || sub_6220((long long)v12, 0xFFFFFFFFLL, (long long)src, 0xFFFFFFFFLL, (long long)v14)
          || !v14[3].m128i_i8[6] )
        {
          return (unsigned int)v23;
        }
        goto LABEL_78;
      }
      if ( v14[1].m128i_i8[13] && ((char)v289 || v301.st_uid != buf.st_uid || v301.st_gid != buf.st_gid) )
      {
        v56 = sub_7080(
                v14,
                src,
                (unsigned int)fd,
                name,
                0xFFFFFFFFLL,
                buf.st_mode,
                buf.st_uid,
                buf.st_gid,
                (unsigned char)v289,
                &v301);
        if ( v56 == -1 )
          goto LABEL_78;
        if ( !v56 )
          st_mode &= 0xFFFFF1FF;
      }
      if ( v14[3].m128i_i8[5]
        && !sub_6220((long long)v12, 0xFFFFFFFFLL, (long long)src, 0xFFFFFFFFLL, (long long)v14)
        && v14[3].m128i_i8[6] )
      {
        goto LABEL_78;
      }
      if ( (v14[1].m128i_i64[1] & 0xFF0000000000FFLL) == 0 )
      {
        if ( v14[3].m128i_i8[9] )
        {
          v163 = v14[1].m128i_u32[0];
        }
        else
        {
          if ( (v14[2].m128i_i8[0] & (unsigned char)v289) == 0 )
          {
            if ( !(int)s2 )
              goto LABEL_134;
            v180 = dword_23010;
            if ( dword_23010 == -1 )
            {
              v180 = umask(0);
              dword_23010 = v180;
              umask(v180);
            }
            LODWORD(s2) = ~v180 & (unsigned int)s2;
            if ( (int)s2 )
            {
              if ( (char)v279 != 1 )
              {
                if ( (char)v289 && fstatat(fd, name, &v301, 256) )
                {
                  v13 = (__off_t)src;
                  goto LABEL_181;
                }
                v267 = v301.st_mode;
                if ( (~v301.st_mode & (unsigned int)s2) == 0 )
                  return (unsigned int)v23;
              }
            }
            else
            {
LABEL_134:
              if ( !(char)v279 )
                return (unsigned int)v23;
            }
            if ( !fchmodat(fd, name, (unsigned int)s2 | v267, 256) )
              return (unsigned int)v23;
            v57 = sub_14440(4, src);
            v58 = dcgettext(0, "preserving permissions for %s", 5);
            v59 = __errno_location();
            error(0, *v59, v58, v57);
LABEL_137:
            if ( !v14[3].m128i_i8[2] )
              return (unsigned int)v23;
            goto LABEL_78;
          }
          v178 = dword_23010;
          v179 = 511;
          if ( (st_mode & 0x7000) != 0x4000 )
            v179 = 438;
          if ( dword_23010 == -1 )
          {
            v178 = umask(0);
            dword_23010 = v178;
            umask(v178);
          }
          v163 = v179 & ~v178;
        }
        if ( !(unsigned int)sub_C8E0(src, 0xFFFFFFFFLL, v163) )
          return (unsigned int)v23;
        goto LABEL_78;
      }
      if ( !(unsigned int)sub_C830(v12, 0xFFFFFFFFLL, src, 0xFFFFFFFFLL, st_mode) )
        return (unsigned int)v23;
      goto LABEL_137;
    }
    if ( !(unsigned char)sub_72A0((long long)v12, (long long)src, st_mode, (char)v15, (long long)v14) )
      goto LABEL_78;
    LODWORD(s2) = v22 & 0x3F;
    LODWORD(v283) = st_mode & 0xF000;
    if ( (int)v283 == 0x4000 )
      goto LABEL_155;
LABEL_102:
    LOBYTE(v15) = v14[3].m128i_i8[10];
    if ( (char)v15 )
    {
      if ( *v12 != 47 )
      {
        v47 = (const char *)sub_E480(name);
        v22 = (long long)v47;
        if ( fd != -100 || (format = (char *)".", strcmp(".", v47)) )
        {
          if ( !stat(".", &v302)
            && !fstatat(fd, (const char *)v22, &v303, 0)
            && (v302.st_ino != v303.st_ino || v302.st_dev != v303.st_dev) )
          {
            free((void *)v22);
            v62 = "%s: can make relative symbolic links only in current directory";
            v15 = (char *)sub_147C0(0, 3, src);
            goto LABEL_161;
          }
        }
        free((void *)v22);
      }
      v48 = sub_C000(v12, (unsigned int)fd, name, v14[1].m128i_u8[6], 0xFFFFFFFFLL);
      v49 = 0;
      LODWORD(v22) = v48;
      if ( v48 <= 0 )
        goto LABEL_109;
      v12 = (char *)sub_14310(1, 4, v12);
      v15 = (char *)sub_14310(0, 4, src);
      v137 = dcgettext(0, "cannot create symbolic link %s to %s", 5);
      v13 = (unsigned int)v22;
      error(0, v22, v137, v15, v12);
      goto LABEL_191;
    }
    if ( v14[1].m128i_i8[7] )
    {
      v107 = 1;
      if ( !v14[1].m128i_i8[6] )
        v107 = v14->m128i_i32[2] == 3;
      v13 = 4294967196LL;
      v108 = sub_65D0(v12, 0xFFFFFF9C, v12, src, fd, (long long)name, v107, 0, (unsigned char)v276);
      v49 = 0;
      if ( v108 )
        goto LABEL_109;
      goto LABEL_191;
    }
    if ( (int)v283 != 0x8000 )
    {
      LOBYTE(v46) = v14[1].m128i_i8[4] & ((int)v283 != 40960);
      if ( !(char)v46 )
      {
        LOBYTE(format) = 0;
        if ( (int)v283 == 4096 )
        {
          LODWORD(v22) = st_mode & ~(int)s2;
          v175 = mknodat(fd, name, v22, 0);
          v49 = (char)format;
          if ( !v175 )
            goto LABEL_109;
          v176 = v22;
          BYTE1(v176) = BYTE1(v22) & 0xEF;
          v177 = mkfifoat(fd, name, v176);
          v49 = (char)format;
          if ( !v177 )
            goto LABEL_109;
          v78 = "cannot create fifo %s";
          v12 = (char *)sub_14440(4, src);
          goto LABEL_190;
        }
        v76 = v283;
        BYTE1(v76) = BYTE1(v283) & 0xBF;
        if ( (int)v283 == 49152 || v76 == 0x2000 )
        {
          v77 = mknodat(fd, name, st_mode & ~(int)s2, buf.st_rdev);
          v49 = (char)format;
          if ( !v77 )
            goto LABEL_109;
          v78 = "cannot create special file %s";
          v12 = (char *)sub_14440(4, src);
          goto LABEL_190;
        }
        if ( (int)v283 == 40960 )
        {
          v164 = sub_C960(v12);
          v22 = v164;
          if ( v164 )
          {
            v13 = (unsigned int)fd;
            LODWORD(v15) = sub_C000(v164, (unsigned int)fd, name, v14[1].m128i_u8[6], 0xFFFFFFFFLL);
            if ( (int)v15 > 0 )
            {
              if ( v14[3].m128i_i8[11] == 1 && !(char)v289 && (v301.st_mode & 0xF000) == 0xA000 )
              {
                format = (char *)v301.st_size;
                v217 = (char *)strlen((const char *)v22);
                if ( format == v217 )
                {
                  v218 = (char *)sub_CAF0(fd, name);
                  if ( v218 )
                  {
                    v13 = v22;
                    format = v218;
                    if ( !strcmp(v218, (const char *)v22) )
                    {
                      free(format);
                      free((void *)v22);
                      goto LABEL_536;
                    }
                    free(format);
                  }
                }
              }
              free((void *)v22);
              v12 = (char *)sub_14440(4, src);
              v165 = dcgettext(0, "cannot create symbolic link %s", 5);
              v13 = (unsigned int)v15;
              error(0, (int)v15, v165, v12);
              goto LABEL_191;
            }
            free((void *)v22);
LABEL_536:
            if ( v14[3].m128i_i8[3] )
            {
              a1 = 0;
              if ( (unsigned int)setfscreatecon(0, v13, v195, v196) )
                goto LABEL_235;
            }
            v49 = v14[1].m128i_i8[13];
            LOBYTE(v15) = 1;
            if ( v49 )
            {
              LOBYTE(format) = v14[1].m128i_i8[13];
              if ( !fchownat(fd, name, buf.st_uid, buf.st_gid, 256) )
              {
                LOBYTE(v15) = (char)format;
                v49 = 0;
                goto LABEL_109;
              }
              v197 = sub_76C0(v14);
              v49 = 0;
              LOBYTE(v15) = v197;
              if ( !v197 )
              {
                v15 = dcgettext(0, "failed to preserve ownership for %s", 5);
                v13 = (unsigned int)*__errno_location();
                error(0, v13, v15, src);
                if ( !v14[3].m128i_i8[2] )
                {
                  LOBYTE(v15) = (char)format;
                  v49 = 0;
                  goto LABEL_109;
                }
                goto LABEL_191;
              }
            }
LABEL_109:
            LOBYTE(v22) = (int)v283 != 0x4000;
            if ( !(char)v289 )
            {
              v50 = v22 & (v14[1].m128i_i8[4] ^ 1);
              if ( v50 )
              {
                if ( v14[2].m128i_i64[1] || v14[3].m128i_i8[3] )
                {
                  v13 = 0;
                  LOBYTE(v283) = v49;
                  LOBYTE(format) = v50;
                  v111 = sub_7510((long long)src, 0, (long long)v14);
                  v49 = v283;
                  LOBYTE(v22) = v111;
                  if ( !v111 )
                  {
                    LOBYTE(v289) = v14[3].m128i_i8[4];
                    if ( (char)v289 )
                      goto LABEL_191;
                    LOBYTE(v22) = (char)format;
                  }
                }
                else
                {
                  LOBYTE(v22) = v22 & (v14[1].m128i_i8[4] ^ 1);
                }
              }
            }
            LODWORD(v23) = 1;
            if ( (char)v288 )
            {
              LOBYTE(v279) = 0;
              LODWORD(v23) = (int)v288;
              goto LABEL_113;
            }
            goto LABEL_116;
          }
          v78 = "cannot read symbolic link %s";
          v12 = (char *)sub_14440(4, v12);
LABEL_190:
          v15 = dcgettext(0, v78, 5);
          v13 = (unsigned int)*__errno_location();
          error(0, v13, v15, v12);
          goto LABEL_191;
        }
        v62 = "%s has unknown file type";
        v15 = (char *)sub_14440(4, v12);
LABEL_161:
        v63 = dcgettext(0, v62, 5);
        v13 = 0;
        error(0, 0, v63, v15);
        goto LABEL_191;
      }
    }
    v18 = v14->m128i_i32[1] == 2;
    ptr = 0;
    LODWORD(v278) = buf.st_mode;
    LOBYTE(format) = v14[3].m128i_i8[1];
    LOBYTE(v280) = v14[3].m128i_i8[5];
    v121 = sub_E620((int)v12, (unsigned char)v18 << 17, v43, v44, v45, v46, v261[0]);
    v122 = v121;
    if ( v121 < 0 )
    {
      v78 = "cannot open %s for reading";
      v12 = (char *)sub_14440(4, v12);
      goto LABEL_190;
    }
    if ( fstat(v121, &v303) )
    {
      format = (char *)sub_14440(4, v12);
      v183 = dcgettext(0, "cannot fstat %s", 5);
      v184 = __errno_location();
      v168 = format;
      v169 = v183;
      v13 = (unsigned int)*v184;
      goto LABEL_447;
    }
    if ( buf.st_ino != v303.st_ino || buf.st_dev != v303.st_dev )
    {
      v166 = sub_14440(4, v12);
      v167 = dcgettext(0, "skipping file %s, as it was replaced while being copied", 5);
      v168 = (char *)v166;
      v13 = 0;
      v169 = v167;
      goto LABEL_447;
    }
    LODWORD(s) = v22 & 0x1FF;
    if ( !(char)v289 )
    {
      v13 = (__off_t)name;
      v126 = sub_125A0(fd, (int)name, (char)format == 0 ? 1 : 513, v123, v124, v125, v261[0]);
      v127 = __errno_location();
      v124 = (unsigned int)*v127;
      v125 = (__off_t)v127;
      if ( (v126 & 0x80000000) == 0 )
      {
        if ( v14[2].m128i_i64[1] || v14[3].m128i_i8[3] )
        {
          v13 = 0;
          v129 = sub_7510((long long)src, 0, (long long)v14);
          v125 = v129;
          if ( !(char)v129 )
          {
            if ( v14[3].m128i_i8[4] )
            {
              LOBYTE(v289) = 0;
              goto LABEL_353;
            }
          }
        }
        v263 = 0;
        LODWORD(v268) = 0;
        goto LABEL_333;
      }
      if ( (int)v124 != 2 )
      {
        v188 = v14[1].m128i_i8[6];
        if ( !v188 )
        {
LABEL_501:
          LOBYTE(v289) = v188;
          goto LABEL_502;
        }
        st_size = v125;
        v214 = unlinkat(fd, name, 0);
        LODWORD(v125) = st_size;
        if ( v214 )
        {
          if ( *(int *)st_size != 2 )
          {
            format = (char *)st_size;
            v240 = sub_14440(4, src);
            v241 = dcgettext(0, "cannot remove %s", 5);
            v168 = (char *)v240;
            v169 = v241;
            v13 = *(unsigned int *)format;
            goto LABEL_447;
          }
        }
        else if ( v14[3].m128i_i8[12] )
        {
          v246 = sub_14440(4, src);
          v247 = dcgettext(0, "removed %s\n", 5);
          __printf_chk(1, v247, v246);
        }
      }
      if ( v14[2].m128i_i64[1] )
      {
        v13 = (__off_t)src;
        LODWORD(v125) = sub_72A0((long long)v12, (long long)src, (unsigned int)s, 1, (long long)v14);
        if ( !(char)v125 )
        {
          LOBYTE(v289) = 0;
          v22 = 0;
          goto LABEL_355;
        }
      }
    }
    v185 = (unsigned int)s & ~(int)s2;
    if ( (char)v280 && !v14[1].m128i_i8[11] )
      LOBYTE(v185) = v185 | 0x80;
    v13 = (__off_t)name;
    LODWORD(v276) = v185;
    v126 = sub_125A0(fd, (int)name, 193, v185, v124, v125, v261[0]);
    v186 = __errno_location();
    v187 = (int)v276;
    v124 = (unsigned int)*v186;
    st_size = (__off_t)v186;
    LOBYTE(v289) = (v126 & 0x80000000) != 0 && (int)v124 == 17;
    if ( (char)v289 )
    {
      v188 = v14[1].m128i_i8[8];
      LODWORD(v124) = 17;
      if ( v188 )
        goto LABEL_501;
      v13 = (__off_t)name;
      LODWORD(p_ptr) = 17;
      v200 = readlinkat(fd, name, &v304, 1u);
      v187 = (int)v276;
      v124 = (unsigned int)p_ptr;
      if ( v200 >= 0 )
      {
        v201 = v14[3].m128i_u8[14];
        if ( !(char)v201 )
        {
          LOBYTE(format) = 0;
          v248 = sub_14440(4, src);
          v249 = dcgettext(0, "not writing through dangling symlink %s", 5);
          v250 = v248;
          v13 = 0;
          v22 = 0;
          error(0, 0, v249, v250);
          LOBYTE(v125) = (char)format;
          goto LABEL_355;
        }
        v13 = (__off_t)name;
        LODWORD(v289) = (int)v276;
        v202 = sub_125A0(fd, (int)name, 65, (int)v276, (int)p_ptr, v201, v261[0]);
        v187 = v289;
        v126 = v202;
        v124 = *(unsigned int *)st_size;
      }
    }
    LOBYTE(v289) = (v126 & 0x80000000) != 0 && (int)v124 == 21;
    if ( (char)v289 )
    {
      v203 = (char *)src;
      LODWORD(v124) = 21;
      if ( *(char *)src )
        LODWORD(v124) = (v203[strlen((const char *)src) - 1] != 47) + 20;
LABEL_502:
      LODWORD(format) = v124;
      v189 = sub_14440(4, src);
      v190 = dcgettext(0, "cannot create regular file %s", 5);
      v13 = (unsigned int)format;
      v168 = (char *)v189;
      v169 = v190;
LABEL_447:
      v22 = 0;
      error(0, v13, v169, v168);
      LOBYTE(v125) = 0;
      goto LABEL_355;
    }
    LODWORD(v268) = v187 & ~(int)s;
    if ( (v126 & 0x80000000) != 0 )
    {
      LOBYTE(v289) = 1;
      goto LABEL_502;
    }
    LOBYTE(v289) = 1;
    v263 = (int)s2;
LABEL_333:
    if ( !(char)format )
      goto LABEL_509;
    if ( v14[4].m128i_i32[1] )
    {
      v13 = 1074041865;
      if ( ioctl(v126, 0x40049409u, v122, v128, v124, v125) )
      {
        if ( v14[4].m128i_i32[1] == 2 )
        {
          v278 = (struct stat *)sub_14310(1, 4, v12);
          v280 = sub_14310(0, 4, src);
          format = dcgettext(0, "failed to clone %s from %s", 5);
          v13 = (unsigned int)*__errno_location();
          error(0, v13, format, v280, v278);
          LOBYTE(v125) = 0;
          goto LABEL_353;
        }
        goto LABEL_335;
      }
LABEL_509:
      LODWORD(format) = (unsigned int)v268 | v14[1].m128i_u8[13];
      if ( !(int)format )
      {
        v302.st_mode = 0;
        if ( v14[1].m128i_i8[15] )
          goto LABEL_511;
        goto LABEL_518;
      }
      LOBYTE(format) = 0;
    }
LABEL_335:
    if ( fstat(v126, &v302) )
    {
      v222 = "cannot fstat %s";
      v280 = sub_14440(4, src);
LABEL_613:
      format = dcgettext(0, v222, 5);
      v13 = (unsigned int)*__errno_location();
      error(0, v13, format, v280);
LABEL_352:
      LOBYTE(v125) = 0;
      goto LABEL_353;
    }
    LODWORD(st_size) = 0;
    v13 = v302.st_mode | (unsigned int)v268;
    if ( v302.st_mode != (int)v13 )
    {
      v130 = fchmod(v126, v13);
      v131 = st_size;
      if ( !v130 )
        v131 = (int)v268;
      LODWORD(v268) = v131;
    }
    if ( !(char)format )
      goto LABEL_563;
    st_blksize = v302.st_blksize;
    p_ptr = (char *)v302.st_blksize;
    if ( (unsigned long long)(v302.st_blksize - 0x20000) > 0x1FFFFFFFFFFE0000LL )
    {
      p_ptr = (char *)&unk_20000;
      v132 = 512;
      if ( st_blksize - 1 <= 0x1FFFFFFFFFFFFFFFLL )
        v132 = st_blksize;
      st_blksize = v132;
    }
    if ( (v303.st_mode & 0xF000) == 0x8000 )
    {
      v133 = 1;
      if ( v303.st_blocks < v303.st_size / 512 )
      {
        offset = lseek(v122, 0, 3);
        if ( offset >= 0 || (v134 = __errno_location(), v135 = *v134, st_size = (__off_t)v134, v135 == 6) )
        {
          v133 = 3;
        }
        else
        {
          if ( v135 != 22 && v135 != 95 )
          {
            format = (char *)sub_14440(4, v12);
            v136 = dcgettext(0, "cannot lseek %s", 5);
            v13 = *(unsigned int *)st_size;
            error(0, v13, v136, format);
            goto LABEL_352;
          }
          v133 = 2;
        }
      }
    }
    else
    {
      v133 = 1;
    }
    if ( (v302.st_mode & 0xF000) == 0x8000 && ((v221 = v14->m128i_i32[3], v221 == 3) || v133 != 1 && v221 == 2) )
    {
      LODWORD(st_size) = v133;
      j__posix_fadvise(v122, 0, 0, 2);
      v290 = 0;
      if ( (int)st_size != 3 )
      {
        v18 = v14->m128i_i32[3] == 3;
        v264 = v14[4].m128i_i32[1] != 0;
        v209 = v18;
        goto LABEL_560;
      }
      v264 = v14[4].m128i_i32[1] != 0;
      v266 = v14->m128i_i32[3];
    }
    else
    {
      LODWORD(st_size) = v133;
      j__posix_fadvise(v122, 0, 0, 2);
      v204 = (void *)v303.st_blksize;
      if ( (unsigned long long)(v303.st_blksize - 0x20000) > 0x1FFFFFFFFFFE0000LL )
        v204 = &unk_20000;
      v205 = sub_DA00(v204, p_ptr, 0x7FFFFFFFFFFFFFFFLL);
      v206 = (char *)v205;
      if ( (v303.st_mode & 0xF000) == 0x8000 && v303.st_size < (unsigned long long)p_ptr )
        p_ptr = (char *)(v303.st_size + 1);
      v290 = 0;
      v207 = (unsigned long long)&p_ptr[v205 - 1] % v205;
      if ( (long long)&p_ptr[v205 - 1 - v207] > 0 )
        v206 = &p_ptr[v205 - 1 - v207];
      v208 = v14[4].m128i_i32[1] != 0;
      p_ptr = v206;
      v264 = v208;
      if ( (int)st_size != 3 )
      {
        st_blksize = 0;
        v209 = v14->m128i_i32[3] == 3;
LABEL_560:
        v13 = v126;
        v210 = sub_6A30(
                 v122,
                 v126,
                 &ptr,
                 (size_t)p_ptr,
                 st_blksize,
                 v209,
                 v264,
                 (long long)v12,
                 (long long)src,
                 0xFFFFFFFFFFFFFFFFLL,
                 length,
                 &v290)
             ^ 1;
        goto LABEL_561;
      }
      v266 = 1;
    }
    st_size = v303.st_size;
    v223 = offset;
    if ( offset < 0 )
    {
      v269 = 0;
      v234 = st_size > 0;
      goto LABEL_663;
    }
    v276 = v12;
    v269 = 0;
    v262 = 0;
    LOBYTE(v271) = (char)format;
    offset = (__off_t)v23;
    v224 = v223;
    v272 = v14;
    v225 = 0;
    while ( 1 )
    {
      v226 = lseek(v122, v224, 4);
      if ( v226 >= 0 )
      {
        v232 = st_size;
        if ( st_size < v226 )
          v232 = v226;
        st_size = v232;
        goto LABEL_623;
      }
      if ( *__errno_location() != 6 )
        goto LABEL_657;
      v226 = st_size;
      if ( st_size <= v224 )
        break;
LABEL_623:
      if ( lseek(v122, v224, 0) < 0 )
        goto LABEL_657;
      v227 = v224 - v262 - v225;
      if ( v224 - v262 == v225 )
      {
        v231 = v226 - v224;
        if ( v266 != 1 )
          v227 = st_blksize;
        LOBYTE(v271) = 0;
        v225 = v231;
        v228 = v227;
      }
      else if ( v266 == 1 )
      {
        if ( !(unsigned char)sub_6170(v126, v224 - v262 - v225) )
        {
          v23 = (const char *)offset;
          v12 = v276;
          v14 = v272;
          v238 = "%s: write failed";
          st_size = sub_147C0(0, 3, src);
          goto LABEL_659;
        }
        LOBYTE(v271) = 0;
        v228 = 0;
        v225 = v226 - v224;
      }
      else
      {
        v13 = (__off_t)src;
        LOBYTE(v271) = sub_6920(v126, (long long)src, v266 == 3, v227);
        if ( !(char)v271 )
          goto LABEL_661;
        v228 = st_blksize;
        v225 = v226 - v224;
      }
      v12 = v276;
      v13 = v126;
      if ( !(unsigned char)sub_6A30(
                               v122,
                               v126,
                               &ptr,
                               (size_t)p_ptr,
                               v228,
                               1,
                               v264,
                               (long long)v276,
                               (long long)src,
                               v225,
                               length,
                               &v291) )
      {
LABEL_661:
        v23 = (const char *)offset;
        v12 = v276;
        v14 = v272;
        goto LABEL_660;
      }
      v229 = (char)v271;
      v13 = length[0] + v224;
      if ( length[0] )
        v229 = v291;
      v269 = length[0] + v224;
      LOBYTE(v271) = v229;
      if ( length[0] < v225 )
      {
        v23 = (const char *)offset;
        v12 = v276;
        v14 = v272;
        v237 = v269;
        goto LABEL_656;
      }
      v230 = lseek(v122, v13, 3);
      if ( v230 < 0 )
      {
        v233 = __errno_location();
        v23 = (const char *)offset;
        v14 = v272;
        if ( *v233 == 6 )
          goto LABEL_641;
LABEL_658:
        v238 = "cannot lseek %s";
        st_size = sub_14440(4, v12);
        goto LABEL_659;
      }
      v262 = v224;
      v224 = v230;
    }
    v13 = 0;
    v236 = lseek(v122, 0, 2);
    v226 = v236;
    if ( v236 < 0 )
    {
LABEL_657:
      v23 = (const char *)offset;
      v12 = v276;
      v14 = v272;
      goto LABEL_658;
    }
    if ( v236 > v224 )
    {
      st_size = v236;
      goto LABEL_623;
    }
    v23 = (const char *)offset;
    v12 = v276;
    v237 = v236;
    v14 = v272;
LABEL_656:
    st_size = v237;
LABEL_641:
    v234 = st_size > v269;
    if ( st_size <= v269 && !(char)v271 )
    {
LABEL_643:
      v235 = (char)format;
      goto LABEL_644;
    }
LABEL_663:
    if ( v266 == 1 )
    {
      v13 = st_size - v269;
      if ( (unsigned char)sub_6170(v126, st_size - v269) )
        goto LABEL_643;
LABEL_688:
      v238 = "failed to extend %s";
      st_size = sub_14440(4, src);
LABEL_659:
      format = dcgettext(0, v238, 5);
      v13 = (unsigned int)*__errno_location();
      error(0, v13, format, st_size);
LABEL_660:
      v235 = 0;
      goto LABEL_644;
    }
    v13 = st_size;
    LOBYTE(v276) = v234;
    if ( ftruncate(v126, st_size) )
      goto LABEL_688;
    if ( v266 != 3 )
      goto LABEL_643;
    if ( !(char)v276 )
      goto LABEL_643;
    v13 = 3;
    if ( fallocate(v126, 3, v269, st_size - v269) >= 0 )
      goto LABEL_643;
    v125 = (__off_t)__errno_location();
    if ( *(int *)v125 == 38 || *(int *)v125 == 95 )
      goto LABEL_643;
    LOBYTE(st_size) = 0;
    v276 = (char *)v125;
    format = (char *)sub_14440(4, src);
    v239 = dcgettext(0, "error deallocating %s", 5);
    v13 = *(unsigned int *)v276;
    error(0, v13, v239, format);
    v235 = st_size;
LABEL_644:
    LOBYTE(v210) = v235 ^ 1;
LABEL_561:
    if ( (char)v210 )
      goto LABEL_352;
    if ( v290 )
    {
      v13 = length[0];
      if ( ftruncate(v126, length[0]) < 0 )
      {
        v222 = "failed to extend %s";
        v280 = sub_14440(4, src);
        goto LABEL_613;
      }
    }
LABEL_563:
    v18 = v14[1].m128i_i8[15] == 0;
    LODWORD(format) = (int)v268;
    if ( !v18 )
    {
LABEL_511:
      v13 = (unsigned int)fd;
      v299[0] = buf.st_atim;
      v299[1] = buf.st_mtim;
      if ( (unsigned int)sub_E6A0(v126, (unsigned int)fd, name, v299, 0, v125) )
      {
        v276 = (char *)sub_14440(4, src);
        st_size = (__off_t)dcgettext(0, "preserving times for %s", 5);
        v13 = (unsigned int)*__errno_location();
        error(0, v13, (const char *)st_size, v276);
        if ( v14[3].m128i_i8[2] )
          goto LABEL_352;
      }
    }
    if ( v14[1].m128i_i8[13] && (buf.st_uid != v302.st_uid || buf.st_gid != v302.st_gid) )
    {
      v13 = (__off_t)src;
      v191 = sub_7080(
               v14,
               src,
               (unsigned int)fd,
               name,
               v126,
               buf.st_mode,
               buf.st_uid,
               buf.st_gid,
               (unsigned char)v289,
               &v302);
      if ( v191 == -1 )
        goto LABEL_352;
      if ( !v191 )
        LODWORD(v278) = (unsigned int)v278 & 0xFFFFF1FF;
    }
LABEL_518:
    if ( !(char)v280 || (v13 = v122, sub_6220((long long)v12, v122, (long long)src, v126, (long long)v14)) )
      LOBYTE(v125) = 1;
    else
      LOBYTE(v125) = v14[3].m128i_i8[6] ^ 1;
    if ( (v14[1].m128i_i64[1] & 0xFF0000000000FFLL) != 0 )
    {
      v13 = v122;
      LOBYTE(format) = v125;
      v216 = sub_C830(v12, v122, src, v126, (unsigned int)v278);
      LOBYTE(v125) = (char)format;
      if ( v216 && v14[3].m128i_i8[2] )
        LOBYTE(v125) = 0;
    }
    else if ( v14[3].m128i_i8[9] )
    {
      v219 = v14[1].m128i_u32[0];
      v13 = v126;
      LOBYTE(format) = v125;
      v220 = sub_C8E0(src, v126, v219);
      LOBYTE(v125) = (char)format;
      if ( v220 )
        LOBYTE(v125) = 0;
    }
    else
    {
      LOBYTE(v280) = v14[2].m128i_i8[0] & v289;
      if ( (char)v280 )
      {
        if ( dword_23010 == -1 )
        {
          LOBYTE(v289) = v125;
          dword_23010 = umask(0);
          umask(dword_23010);
          LOBYTE(v125) = v289;
        }
        v13 = v126;
        LOBYTE(format) = v125;
        v245 = sub_C8E0(src, v126, ~(short)dword_23010 & 0x1B6);
        LOBYTE(v125) = (char)format;
        LOBYTE(v289) = v280;
        if ( v245 )
          LOBYTE(v125) = 0;
      }
      else if ( v263 | (unsigned int)format )
      {
        v192 = dword_23010;
        if ( dword_23010 == -1 )
        {
          LOBYTE(v278) = v125;
          dword_23010 = umask(0);
          LODWORD(v280) = dword_23010;
          umask(dword_23010);
          LOBYTE(v125) = (char)v278;
          if ( (unsigned int)format | v263 & ~(int)v280 )
          {
            v192 = dword_23010;
            if ( (int)v280 == -1 )
            {
              LOBYTE(format) = (char)v278;
              dword_23010 = umask(0);
              umask(dword_23010);
              v192 = dword_23010;
              LOBYTE(v125) = (char)format;
            }
LABEL_526:
            LOBYTE(format) = v125;
            v13 = ~v192 & (unsigned int)s;
            v193 = fchmod(v126, v13);
            LOBYTE(v125) = (char)format;
            if ( v193 )
            {
              LOBYTE(v278) = (char)format;
              v280 = sub_14440(4, src);
              format = dcgettext(0, "preserving permissions for %s", 5);
              v13 = (unsigned int)*__errno_location();
              error(0, v13, format, v280);
              LOBYTE(v125) = (char)v278;
              if ( v14[3].m128i_i8[2] )
                LOBYTE(v125) = 0;
            }
          }
        }
        else if ( (unsigned int)format | v263 & ~dword_23010 )
        {
          goto LABEL_526;
        }
      }
    }
LABEL_353:
    LOBYTE(format) = v125;
    if ( close(v126) < 0 )
    {
      format = (char *)sub_14440(4, src);
      v215 = dcgettext(0, "failed to close %s", 5);
      v13 = (unsigned int)*__errno_location();
      error(0, v13, v215, format);
      v22 = (long long)ptr;
      LOBYTE(v125) = 0;
    }
    else
    {
      v22 = (long long)ptr;
      LOBYTE(v125) = (char)format;
    }
LABEL_355:
    LOBYTE(format) = v125;
    if ( close(v122) >= 0 )
    {
      LOBYTE(v15) = 0;
      free((void *)v22);
      v49 = (char)format;
      if ( !(char)format )
        goto LABEL_191;
      goto LABEL_109;
    }
    v12 = (char *)sub_14440(4, v12);
    v15 = dcgettext(0, "failed to close %s", 5);
    v13 = (unsigned int)*__errno_location();
    error(0, v13, v15, v12);
    free((void *)v22);
LABEL_191:
    if ( !v14[3].m128i_i8[3] || (a1 = 0, !(unsigned int)setfscreatecon(0, v13, v64, v65)) )
    {
      if ( !v23 )
        sub_BC60(buf.st_ino, buf.st_dev);
      goto LABEL_194;
    }
LABEL_235:
    sub_6770();
LABEL_236:
    if ( v14[1].m128i_i8[8] )
      goto LABEL_300;
    if ( (st_mode & 0xF000) != 0x4000 )
    {
LABEL_238:
      v99 = v14->m128i_i32[2];
      if ( v99 == 2 || v99 == 3 && !(unsigned char)sub_67A0((long long)v14, (long long)src, fd, name, (long long)&v301) )
        goto LABEL_89;
    }
LABEL_240:
    if ( (char)v15 )
      goto LABEL_89;
    v100 = v301.st_mode;
    if ( (v301.st_mode & 0xF000) == 0x4000 )
    {
      if ( (st_mode & 0xF000) == 0x4000 )
        goto LABEL_248;
LABEL_363:
      if ( !v14[1].m128i_i8[8] || (LODWORD(v23) = v14->m128i_i32[0]) == 0 )
      {
        v23 = (const char *)sub_14440(4, src);
        v173 = dcgettext(0, "cannot overwrite directory %s with non-directory", 5);
        v174 = v23;
        LODWORD(v23) = 0;
        error(0, 0, v173, v174);
        return (unsigned int)v23;
      }
LABEL_365:
      if ( (buf.st_mode & 0xF000) != 0x4000 )
      {
LABEL_366:
        v138 = sub_D970(v12);
        v102 = 1;
        if ( *(char *)v138 != 46 )
          goto LABEL_371;
        goto LABEL_367;
      }
LABEL_457:
      if ( (v100 & 0xF000) != 0x4000 )
      {
        if ( !(int)v23 )
        {
          v37 = sub_147C0(0, 3, src);
          v38 = "cannot move directory onto non-directory: %s -> %s";
          v39 = sub_147C0(0, 3, v12);
          goto LABEL_77;
        }
        goto LABEL_366;
      }
LABEL_426:
      if ( !(int)v23 )
        goto LABEL_260;
      goto LABEL_366;
    }
LABEL_242:
    v101 = st_mode & 0xF000;
    if ( v101 == 0x4000 )
    {
      if ( !v14[1].m128i_i8[8] || (LODWORD(v23) = v14->m128i_i32[0]) == 0 )
      {
        v37 = sub_14310(1, 4, v12);
        v38 = "cannot overwrite non-directory %s with directory %s";
        v39 = sub_14310(0, 4, src);
        goto LABEL_77;
      }
      if ( !(char)v288 )
        goto LABEL_365;
LABEL_244:
      if ( (int)v23 != 3 )
      {
        if ( (unsigned char)sub_E820(v14[4].m128i_i64[1], name, &v301) )
        {
          v37 = sub_14310(1, 4, v12);
          v38 = "will not overwrite just-created %s with %s";
          v39 = sub_14310(0, 4, src);
          goto LABEL_77;
        }
        if ( v101 == 0x4000 || (v100 = v301.st_mode, (v301.st_mode & 0xF000) != 0x4000) )
        {
LABEL_248:
          LODWORD(v23) = v14->m128i_i32[0];
          goto LABEL_249;
        }
        goto LABEL_363;
      }
      if ( v14[1].m128i_i8[8] )
        goto LABEL_365;
      goto LABEL_423;
    }
    LODWORD(v23) = v14->m128i_i32[0];
    if ( (char)v288 )
      goto LABEL_244;
LABEL_249:
    v102 = v14[1].m128i_i8[8];
    if ( v102 )
    {
      if ( (buf.st_mode & 0xF000) == 0x4000 )
      {
        v100 = v301.st_mode;
        goto LABEL_457;
      }
      goto LABEL_426;
    }
    if ( !(int)v23 )
      break;
LABEL_423:
    v138 = sub_D970(v12);
    if ( *(char *)v138 != 46 )
      goto LABEL_370;
    v102 = 0;
LABEL_367:
    v139 = *(char *)(v138 + (*(char *)(v138 + 1) == 46) + 1);
    if ( !v139 || v139 == 47 )
      break;
    if ( !v102 )
    {
LABEL_370:
      if ( (v301.st_mode & 0xF000) == 0x4000 )
        goto LABEL_260;
    }
LABEL_371:
    if ( (int)v23 != 3 )
    {
      v289 = strlen((const char *)v138);
      s2 = (void *)sub_D970(name);
      v140 = strlen((const char *)s2);
      v141 = ::src;
      n = v140;
      v142 = strlen(::src);
      if ( v289 == n + v142 )
      {
        v289 = n;
        if ( !memcmp((const void *)v138, s2, n) && !strcmp((const char *)(v138 + v289), v141) )
        {
          v143 = name;
          v144 = strlen(name);
          v145 = (char *)sub_6110(v143, (long long)&v143[v144], v141);
          v146 = fstatat(fd, v145, &v303, 0);
          free(v145);
          if ( !v146 && buf.st_ino == v303.st_ino && buf.st_dev == v303.st_dev )
          {
            if ( v14[1].m128i_i8[8] )
              v23 = dcgettext(0, "backing up %s might destroy source;  %s not moved", 5);
            else
              v23 = dcgettext(0, "backing up %s might destroy source;  %s not copied", 5);
            v242 = sub_14310(1, 4, v12);
            v243 = sub_14310(0, 4, src);
            v244 = v23;
            LODWORD(v23) = 0;
            error(0, 0, v244, v243, v242);
            return (unsigned int)v23;
          }
        }
      }
    }
    v147 = name;
    v148 = (const char *)sub_D840((unsigned int)fd, name, v14->m128i_u32[0]);
    v149 = (char *)v148;
    if ( !v148 )
    {
      v23 = (const char *)__errno_location();
      if ( *(int *)v23 == 2 )
      {
        LOBYTE(v289) = 1;
        LODWORD(v15) = 17;
        n = 0;
        goto LABEL_17;
      }
      v251 = sub_14440(4, src);
      v252 = dcgettext(0, "cannot backup %s", 5);
      v253 = *(int *)v23;
      LODWORD(v23) = 0;
      error(0, v253, v252, v251);
      return (unsigned int)v23;
    }
    v15 = (char *)(v147 - (char *)src);
    v150 = strlen(v148);
    v151 = (long long)&v15[v150 + 24];
    v152 = &v261[-(v151 & 0xFFFFFFFFFFFFF000LL)];
    v153 = v151 & 0xFFF0;
    if ( v261 != v152 )
    {
      while ( &v259 != (long long *)v152 )
        ;
    }
    v154 = v153 & 0xFFF;
    v155 = alloca(v154);
    if ( v154 )
      *(__dev_t *)((char *)&st_dev + v154) = *(__dev_t *)((char *)&st_dev + v154);
    v156 = (size_t)v15;
    v289 = v150 + 1;
    LODWORD(v15) = 17;
    n = (size_t)v260;
    v157 = mempcpy(v260, src, v156);
    memcpy(v157, v149, v289);
    free(v149);
    LOBYTE(v289) = 1;
LABEL_17:
    if ( (char)v288 )
      goto LABEL_53;
  }
  LODWORD(v15) = 17;
  n = 0;
  LOBYTE(v23) = v102 | ((v301.st_mode & 0xF000) == 0x4000);
  if ( (char)v23 )
    goto LABEL_17;
  if ( !v14[1].m128i_i8[5] )
  {
    if ( !v14[3].m128i_i8[1] )
      goto LABEL_17;
    if ( !v14[3].m128i_i8[0] || v301.st_nlink <= 1 )
    {
      v18 = v14->m128i_i32[1] == 2;
      LODWORD(v15) = 17;
      n = 0;
      if ( !v18 || (buf.st_mode & 0xF000) == 0x8000 )
        goto LABEL_17;
    }
  }
  if ( !unlinkat(fd, name, 0) || (v103 = __errno_location(), *v103 == 2) )
  {
    LOBYTE(v289) = v14[3].m128i_i8[12];
    if ( (char)v289 )
    {
      LODWORD(v15) = 17;
      v198 = sub_14440(4, src);
      v199 = dcgettext(0, "removed %s\n", 5);
      __printf_chk(1, v199, v198);
      n = 0;
      goto LABEL_17;
    }
    LOBYTE(v289) = 1;
LABEL_260:
    n = 0;
    LODWORD(v15) = 17;
    goto LABEL_17;
  }
  v212 = sub_14440(4, src);
  v213 = dcgettext(0, "cannot remove %s", 5);
  error(0, *v103, v213, v212);
  return (unsigned int)v23;
}



// Function: copy @ 0xba80
long long copy(
        long long a1,
        void *a2,
        int a3,
        char *a4,
        int a5,
        const __m128i *a6,
        struct stat *a7,
        __off_t a8)
{
  int v8; // eax
  unsigned int v10; // edx
  char v12; // [rsp+7h] [rbp-11h] BYREF
  unsigned long long v13; // [rsp+8h] [rbp-10h]

  v13 = __readfsqword(0x28u);
  if ( a6->m128i_i32[0] > 3u )
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)", "src/copy.c", 0xC0Fu, "valid_options");
  v8 = a6->m128i_i32[3];
  if ( (unsigned int)(v8 - 1) > 2 )
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)", "src/copy.c", 0xC10u, "valid_options");
  v10 = a6[4].m128i_u32[1];
  if ( v10 > 2 )
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)", "src/copy.c", 0xC11u, "valid_options");
  if ( a6[1].m128i_i8[7] && a6[3].m128i_i8[10] )
    __assert_fail("!(co->hard_link && co->symbolic_link)", "src/copy.c", 0xC12u, "valid_options");
  if ( v8 != 2 && v10 == 2 )
    __assert_fail(
      "! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)",
      "src/copy.c",
      0xC13u,
      "valid_options");
  v12 = 0;
  qword_23570 = a1;
  qword_23568 = (long long)a2;
  return sub_76F0(a1, a2, a3, a4, a5, 0, 0, a6, 1, &v12, a7, a8);
}



// Function: src_to_dest_hash @ 0xbc10
unsigned long long src_to_dest_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: src_to_dest_compare @ 0xbc20
bool src_to_dest_compare(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( *a1 == *a2 )
    return a1[1] == a2[1];
  return result;
}



// Function: forget_created @ 0xbc60
unsigned long long forget_created(long long a1, long long a2)
{
  long long v2; // rax
  void *v3; // rbp
  long long v5[3]; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  v5[0] = a1;
  v5[1] = a2;
  v5[2] = 0;
  v2 = ((long long (*)(long long, long long *))sub_12270)(qword_23578, v5);
  if ( v2 )
  {
    v3 = (void *)v2;
    free(*(void **)(v2 + 16));
    free(v3);
  }
  return v6 - __readfsqword(0x28u);
}



// Function: src_to_dest_lookup @ 0xbcd0
long long src_to_dest_lookup(long long a1, long long a2)
{
  long long result; // rax
  long long v3[5]; // [rsp+0h] [rbp-28h] BYREF

  v3[3] = __readfsqword(0x28u);
  v3[0] = a1;
  v3[1] = a2;
  result = sub_11920(qword_23578, v3);
  if ( result )
    return *(long long *)(result + 16);
  return result;
}



// Function: remember_copied @ 0xbd30
long long remember_copied(void *src, long long a2, long long a3)
{
  long long *v4; // rbp
  long long v5; // rax
  long long v6; // rdi
  long long v7; // rax
  long long *v8; // rbx
  long long result; // rax

  v4 = (long long *)sub_16C50(24);
  v5 = sub_17130(src);
  v6 = qword_23578;
  *v4 = a2;
  v4[2] = v5;
  v4[1] = a3;
  v7 = sub_12210(v6, v4);
  if ( !v7 )
    sub_17150();
  v8 = (long long *)v7;
  result = 0;
  if ( v4 != v8 )
  {
    free((void *)v4[2]);
    free(v4);
    return v8[2];
  }
  return result;
}



// Function: hash_init @ 0xbdb0
long long hash_init()
{
  long long result; // rax

  result = sub_11BB0(103, 0, sub_BC10, sub_BC20, sub_BC40);
  qword_23578 = result;
  if ( !result )
    sub_17150();
  return result;
}



// Function: try_link @ 0xbdf0
int try_link(char *to, long long a2)
{
  return linkat(*(int *)a2, *(const char **)(a2 + 8), *(int *)(a2 + 16), to, *(int *)(a2 + 20));
}



// Function: samedir_template @ 0xbe10
void *samedir_template(void *src, void *dest)
{
  long long v3; // rax
  size_t v4; // r12

  v3 = sub_D970(src) - (long long)src;
  v4 = v3;
  if ( (unsigned long long)(v3 + 9) > 0x100 )
  {
    dest = malloc(v3 + 9);
    if ( !dest )
      return 0;
  }
  strcpy((char *)mempcpy(dest, src, v4), "CuXXXXXX");
  return dest;
}



// Function: try_symlink @ 0xbe70
int try_symlink(char *to, long long a2)
{
  return symlinkat(*(const char **)a2, *(int *)(a2 + 8), to);
}



// Function: force_linkat @ 0xbe90
long long force_linkat(int a1, const char *a2, int a3, char *a4, int a5, char a6, int a7)
{
  unsigned int v10; // eax
  char *v11; // rax
  char *v12; // r15
  unsigned int v13; // r12d
  char dest[264]; // [rsp+30h] [rbp-148h] BYREF
  unsigned long long v16; // [rsp+138h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  v10 = a7;
  if ( a7 < 0 )
  {
    v13 = linkat(a1, a2, a3, a4, a5);
    if ( !v13 )
      return v13;
    v10 = *__errno_location();
  }
  if ( a6 == 1 && v10 == 17 )
  {
    v11 = (char *)sub_BE10(a4, dest);
    v12 = v11;
    if ( v11 )
    {
      if ( (unsigned int)sub_157B0(v11) )
      {
        v13 = *__errno_location();
      }
      else
      {
        v13 = -1;
        if ( renameat(a3, v12, a3, a4) )
          v13 = *__errno_location();
        unlinkat(a3, v12, 0);
      }
      if ( v12 != dest )
        free(v12);
    }
    else
    {
      return (unsigned int)*__errno_location();
    }
  }
  else
  {
    return v10;
  }
  return v13;
}



// Function: force_symlinkat @ 0xc000
long long force_symlinkat(const char *a1, int a2, char *a3, char a4, int a5)
{
  char *v7; // rax
  char *v8; // r15
  unsigned int v9; // r12d
  char dest[264]; // [rsp+10h] [rbp-148h] BYREF
  unsigned long long v12; // [rsp+118h] [rbp-40h]

  v12 = __readfsqword(0x28u);
  if ( a5 < 0 )
  {
    v9 = symlinkat(a1, a2, a3);
    if ( !v9 )
      return v9;
    a5 = *__errno_location();
  }
  if ( a4 == 1 && a5 == 17 )
  {
    v7 = (char *)sub_BE10(a3, dest);
    v8 = v7;
    if ( v7 )
    {
      if ( (unsigned int)sub_157B0(v7) )
      {
        v9 = *__errno_location();
      }
      else
      {
        v9 = -1;
        if ( renameat(a2, v8, a2, a3) )
        {
          v9 = *__errno_location();
          unlinkat(a2, v8, 0);
        }
      }
      if ( v8 != dest )
        free(v8);
    }
    else
    {
      return (unsigned int)*__errno_location();
    }
  }
  else
  {
    return (unsigned int)a5;
  }
  return v9;
}



// Function: restorecon_private @ 0xc140
long long restorecon_private(long long a1, const char *a2)
{
  struct stat *p_buf; // rsi
  unsigned int v4; // r12d
  int *v5; // rax
  int *v6; // rbx
  long long v7; // r13
  long long v8; // r15
  int v9; // r14d
  unsigned int v10; // ebp
  int *v11; // rax
  int *v12; // rbx
  unsigned int v13; // eax
  int v14; // r12d
  unsigned int v16; // eax
  long long v17; // [rsp+0h] [rbp-E8h] BYREF
  long long v18; // [rsp+8h] [rbp-E0h] BYREF
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v20; // [rsp+A8h] [rbp-40h]

  v20 = __readfsqword(0x28u);
  v17 = 0;
  v18 = 0;
  if ( a1 )
  {
    p_buf = (struct stat *)&unk_20000;
    v4 = open(a2, (int)&unk_20000);
    v5 = __errno_location();
    v6 = v5;
    if ( v4 == -1 )
    {
      v9 = *v5;
      if ( *v5 != 40 )
      {
        v8 = 0;
        v7 = 0;
        goto LABEL_17;
      }
      p_buf = &buf;
      if ( lstat(a2, &buf) < 0 )
      {
        v9 = *v6;
        v8 = 0;
        v7 = 0;
        goto LABEL_17;
      }
    }
    else
    {
      p_buf = &buf;
      if ( fstat(v4, &buf) < 0 )
      {
        v8 = 0;
        v7 = 0;
        v10 = -1;
LABEL_30:
        v9 = *v6;
        goto LABEL_13;
      }
    }
    p_buf = (struct stat *)&v17;
    if ( (int)selabel_lookup(a1, &v17, a2, buf.st_mode) < 0 )
    {
      v9 = *v6;
      v8 = 0;
      v7 = 0;
      if ( *v6 == 2 )
      {
        *v6 = 61;
        v9 = 61;
      }
    }
    else
    {
      v7 = context_new(v17);
      if ( v7 )
      {
        p_buf = (struct stat *)&v18;
        if ( v4 == -1 )
        {
          if ( (int)sub_15590(a2, &v18) < 0 )
          {
            v9 = *v6;
            v8 = 0;
            goto LABEL_17;
          }
          v8 = context_new(v18);
          if ( !v8 )
          {
            v9 = *v6;
            v10 = -1;
            goto LABEL_18;
          }
        }
        else
        {
          if ( (int)sub_15610(v4, &v18) < 0 )
          {
            v8 = 0;
            v10 = -1;
            goto LABEL_30;
          }
          v8 = context_new(v18);
          if ( !v8 )
          {
            v9 = *v6;
            v10 = -1;
            goto LABEL_13;
          }
        }
        p_buf = (struct stat *)context_type_get(v7);
        if ( p_buf )
        {
          if ( !(unsigned int)context_type_set(v8, p_buf) )
          {
            p_buf = (struct stat *)context_str(v8);
            if ( p_buf )
            {
              if ( v4 != -1 )
              {
                v10 = fsetfilecon(v4, p_buf);
                goto LABEL_30;
              }
              v16 = lsetfilecon(a2, p_buf);
              v9 = *v6;
              v4 = v16;
LABEL_17:
              v10 = v4;
              goto LABEL_18;
            }
          }
        }
        v9 = *v6;
      }
      else
      {
        v9 = *v6;
        v8 = 0;
      }
    }
    v10 = -1;
    if ( v4 == -1 )
    {
LABEL_18:
      context_free(v7, p_buf);
      context_free(v8, p_buf);
      freecon(v17);
      freecon(v18);
      *v6 = v9;
      return v10;
    }
LABEL_13:
    close(v4);
    goto LABEL_18;
  }
  if ( (int)getfscreatecon(&v18) < 0 )
  {
    return (unsigned int)-1;
  }
  else
  {
    v11 = __errno_location();
    v12 = v11;
    if ( v18 )
    {
      v13 = lsetfilecon(a2, v18);
      v14 = *v12;
      v10 = v13;
      freecon(v18);
      *v12 = v14;
    }
    else
    {
      *v11 = 61;
      return (unsigned int)-1;
    }
  }
  return v10;
}



// Function: defaultcon @ 0xc450
long long defaultcon(long long a1, char *a2, unsigned int a3)
{
  long long v5; // rbp
  int *v6; // rbx
  long long *v8; // rsi
  long long v9; // rax
  void *v10; // r13
  void *v11; // rbp
  unsigned short v12; // ax
  int v13; // eax
  int v14; // r14d
  int v15; // r12d
  long long v16; // r12
  long long v17; // rdx
  long long v18; // rcx
  long long v19; // rdi
  unsigned int v20; // eax
  unsigned int v21; // r15d
  void *v23; // rdi
  long long v24; // [rsp+8h] [rbp-60h] BYREF
  long long v25; // [rsp+10h] [rbp-58h] BYREF
  long long v26; // [rsp+18h] [rbp-50h] BYREF
  long long v27[9]; // [rsp+20h] [rbp-48h] BYREF

  v5 = (long long)a2;
  v27[1] = __readfsqword(0x28u);
  v24 = 0;
  v25 = 0;
  v6 = __errno_location();
  if ( *a2 == 47 )
  {
    v10 = 0;
  }
  else
  {
    v8 = (long long *)(&dword_0 + 2);
    v9 = sub_DA80(a2, 2);
    v5 = v9;
    if ( !v9 )
    {
      v14 = *v6;
      v10 = 0;
      v16 = 0;
      v21 = -1;
      goto LABEL_15;
    }
    v10 = (void *)v9;
  }
  v8 = &v24;
  if ( (int)selabel_lookup(a1, &v24, v5, a3) < 0 )
  {
    v14 = *v6;
    if ( *v6 == 2 )
    {
      *v6 = 61;
      v14 = 61;
      v16 = 0;
      v5 = 0;
      v21 = -1;
      goto LABEL_15;
    }
    goto LABEL_20;
  }
  v26 = 0;
  v27[0] = 0;
  v11 = (void *)sub_E480(v5);
  if ( (int)getcon(&v26) < 0 || (v8 = v27, (int)sub_15510(v11, v27) < 0) || (v12 = mode_to_security_class(a3)) == 0 )
  {
    v23 = v11;
    v14 = *v6;
    v16 = 0;
    v5 = 0;
    free(v23);
    v21 = -1;
    freecon(v26);
    freecon(v27[0]);
    *v6 = v14;
    goto LABEL_15;
  }
  v8 = (long long *)v27[0];
  v13 = security_compute_create(v26, v27[0], v12, &v25);
  v14 = *v6;
  v15 = v13;
  free(v11);
  freecon(v26);
  freecon(v27[0]);
  *v6 = v14;
  if ( v15 < 0 )
  {
LABEL_20:
    v16 = 0;
    v5 = 0;
    v21 = -1;
    goto LABEL_15;
  }
  v5 = context_new(v24);
  if ( v5 )
  {
    v16 = context_new(v25);
    if ( v16
      && (v8 = (long long *)context_type_get(v5)) != 0
      && !(unsigned int)context_type_set(v16, v8)
      && (v19 = context_str(v16)) != 0 )
    {
      v20 = setfscreatecon(v19, v8, v17, v18);
      v14 = *v6;
      v21 = v20;
    }
    else
    {
      v14 = *v6;
      v21 = -1;
    }
  }
  else
  {
    v14 = *v6;
    v16 = 0;
    v21 = -1;
  }
LABEL_15:
  context_free(v5, v8);
  context_free(v16, v8);
  freecon(v24);
  freecon(v25);
  free(v10);
  *v6 = v14;
  return v21;
}



// Function: restorecon @ 0xc6f0
long long restorecon(long long a1, const char *a2, char a3)
{
  void *v4; // r13
  unsigned int v5; // r12d
  const char *v6; // rbp
  int *v7; // rbx
  int v8; // r14d
  const char **v9; // rbp
  long long v11; // rax
  int v12; // eax
  int v13; // ebp
  long long v14[9]; // [rsp+0h] [rbp-48h] BYREF

  v4 = 0;
  v5 = a1;
  v6 = a2;
  v14[3] = __readfsqword(0x28u);
  if ( *a2 != 47 )
  {
    v11 = sub_DA80(a2, 2);
    v6 = (const char *)v11;
    if ( !v11 )
      return 0;
    v4 = (void *)v11;
    v7 = __errno_location();
    if ( a3 )
      goto LABEL_3;
LABEL_15:
    v12 = sub_C140(a1, v6);
    v13 = *v7;
    LOBYTE(v5) = v12 != -1;
    free(v4);
    *v7 = v13;
    return v5;
  }
  v7 = __errno_location();
  if ( !a3 )
    goto LABEL_15;
LABEL_3:
  v14[0] = v6;
  v14[1] = 0;
  v8 = 0;
  v9 = (const char **)sub_17190(v14, 16, 0);
  while ( sub_10840(v9) )
  {
    while ( (int)sub_C140(a1, v9[4]) < 0 )
    {
      v8 = *v7;
      if ( !sub_10840(v9) )
        goto LABEL_7;
    }
  }
LABEL_7:
  if ( *v7 )
    v8 = *v7;
  if ( (unsigned int)sub_10650(v9) )
    v8 = *v7;
  free(v4);
  LOBYTE(v5) = v8 == 0;
  return v5;
}



// Function: dir_name @ 0xe480
long long dir_name(void *a1)
{
  long long result; // rax

  result = sub_E4E0(a1);
  if ( !result )
    sub_17150();
  return result;
}



// Function: raw_hasher @ 0x11240
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return __ROR8__(a1, 3) % a2;
}



// Function: raw_comparator @ 0x11260
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0x11270
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_1C620 )
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
    *(long long *)(a1 + 40) = &unk_1C620;
    return 0;
  }
  return result;
}



// Function: hash_find_entry @ 0x11300
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



// Function: transfer_entries @ 0x11550
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



// Function: hash_print_statistics @ 0x117c0
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



// Function: hash_lookup @ 0x11920
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



// Function: hash_get_next @ 0x119e0
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



// Function: hash_do_for_each @ 0x11ac0
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



// Function: hash_initialize @ 0x11bb0
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
    v7 = sub_11240;
  if ( !a4 )
    v8 = sub_11260;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_1C620;
    v9[5] = v5;
    if ( (unsigned char)sub_11270((long long)v9)
      && (v10 = sub_11410(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
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



// Function: hash_clear @ 0x11cc0
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



// Function: hash_free @ 0x11d70
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



// Function: hash_rehash @ 0x11e60
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
  v2 = sub_11410(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
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
  v6 = sub_11550((long long)&ptr, a1, 0);
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
    if ( !(unsigned char)sub_11550(a1, (long long)&ptr, 1) || !(unsigned char)sub_11550(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0x11fe0
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
  v5 = sub_11300((long long)a1, a2, v22, 0);
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
          sub_11270((long long)a1);
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
            if ( !(unsigned char)sub_11E60((long long)a1, v21) )
              return (unsigned int)-1;
            if ( sub_11300((long long)a1, a2, v22, 0) )
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



// Function: hash_insert @ 0x12210
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_11FE0(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0x12270
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
  v3 = sub_11300((long long)a1, a2, v18, 1);
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
    sub_11270((long long)a1);
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
      if ( !(unsigned char)sub_11E60((long long)a1, v14) )
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


