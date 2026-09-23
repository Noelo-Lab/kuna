// Function: main @ 0x2860
long long main(unsigned int a1, char **a2, char **a3)
{
  struct option *v3; // r15
  const char *v4; // r14
  char *const *v5; // r13
  unsigned long long v6; // rbx
  char *v7; // rdi
  char *v8; // rax
  char *v9; // r12
  char *const *v10; // rsi
  int v11; // eax
  int v12; // eax
  FILE *v13; // rbp
  int v14; // eax
  int v15; // eax
  int *v16; // rdx
  unsigned int v17; // eax
  int v18; // eax
  char v19; // dl
  char *const *v20; // rax
  long long v21; // rdx
  int v22; // eax
  int *v23; // rax
  char *v24; // rax
  long long v25; // r13
  const char *v26; // rax
  char *v27; // rax
  int *v28; // rax
  char *v29; // rax
  const char *v31; // r13
  int *v32; // rax
  char *const *v33; // r15
  unsigned long long v34; // r13
  struct stat *v35; // r14
  int v36; // eax
  const char *v37; // rbp
  int *v38; // rax
  int v39; // esi
  unsigned long long v40; // rdx
  unsigned long long v41; // rcx
  int v42; // edi
  unsigned long long v43; // rcx
  double v44; // xmm1_8
  long long v45; // rdi
  long long v46; // r12
  char *v47; // rax
  char *v48; // rax
  long long v49; // r12
  char *v50; // rax
  long long v51; // [rsp+8h] [rbp-230h]
  FILE *v52; // [rsp+8h] [rbp-230h]
  double st_size_low; // [rsp+8h] [rbp-230h]
  char v54; // [rsp+10h] [rbp-228h]
  long long v55; // [rsp+10h] [rbp-228h]
  char *v56; // [rsp+10h] [rbp-228h]
  int *ptr; // [rsp+18h] [rbp-220h]
  int *v58; // [rsp+20h] [rbp-218h]
  char v59; // [rsp+20h] [rbp-218h]
  char *v60; // [rsp+20h] [rbp-218h]
  char v61; // [rsp+2Bh] [rbp-20Dh]
  int fd; // [rsp+2Ch] [rbp-20Ch]
  int v63; // [rsp+3Ch] [rbp-1FCh] BYREF
  unsigned long long v64; // [rsp+40h] [rbp-1F8h] BYREF
  char *const *v65; // [rsp+48h] [rbp-1F0h]
  struct stat buf; // [rsp+160h] [rbp-D8h] BYREF
  unsigned long long v67; // [rsp+1F8h] [rbp-40h]

  v3 = &longopts;
  v4 = "clLmw";
  v5 = a2;
  v6 = a1;
  v7 = *a2;
  v67 = __readfsqword(0x28u);
  sub_4D10(v7);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_88E0(sub_46F0);
  qword_D120 = getpagesize();
  setvbuf(stdout, 0, 1, 0);
  v8 = getenv("POSIXLY_CORRECT");
  byte_D131 = 0;
  byte_D132 = 0;
  byte_D133 = 0;
  byte_D134 = 0;
  byte_D130 = 0;
  qword_D138 = 0;
  qword_D140 = 0;
  qword_D148 = 0;
  qword_D150 = 0;
  qword_D158 = 0;
  byte_D118 = v8 != 0;
  v9 = 0;
  while ( 2 )
  {
    v10 = v5;
    v11 = getopt_long(v6, v5, "clLmw", &longopts, 0);
    if ( v11 != -1 )
    {
      if ( v11 <= 129 )
      {
        if ( v11 > 75 )
        {
          switch ( v11 )
          {
            case 76:
              byte_D130 = 1;
              continue;
            case 99:
              byte_D131 = 1;
              continue;
            case 108:
              byte_D134 = 1;
              continue;
            case 109:
              byte_D132 = 1;
              continue;
            case 119:
              byte_D133 = 1;
              continue;
            case 128:
              byte_D160 = 1;
              continue;
            case 129:
              v9 = (char *)optarg;
              continue;
            default:
              goto LABEL_112;
          }
        }
        if ( v11 == -131 )
        {
          sub_7A40(stdout, 0);
          exit(0);
        }
        if ( v11 == -130 )
          sub_4230(0);
      }
LABEL_112:
      sub_4230(1);
    }
    break;
  }
  if ( !byte_D134 && !byte_D133 && !byte_D132 && !byte_D131 && !byte_D130 )
  {
    byte_D131 = 1;
    byte_D133 = 1;
    byte_D134 = 1;
  }
  v12 = optind;
  if ( !v9 )
  {
LABEL_75:
    if ( (int)v6 > v12 )
    {
      v6 = (int)v6 - v12;
      v33 = &v5[v12];
    }
    else
    {
      v6 = 1;
      v33 = (char *const *)&unk_D110;
    }
    v13 = (FILE *)sub_4580(v33, v10);
    if ( !v13 )
      goto LABEL_115;
    v61 = 0;
    ptr = (int *)sub_7D50(v6, 152);
    goto LABEL_79;
  }
  if ( (int)v6 > optind )
  {
    v46 = sub_69F0(4, v5[optind]);
    v47 = dcgettext(0, "extra operand %s", 5);
    error(0, 0, v47, v46);
    v48 = dcgettext(0, "file operands cannot be combined with --files0-from", 5);
    __fprintf_chk(stderr, 1, "%s\n", v48);
    goto LABEL_112;
  }
  if ( !strcmp(v9, "-") )
  {
    v13 = stdin;
  }
  else
  {
    v13 = fopen(v9, "r");
    if ( !v13 )
    {
      v5 = (char *const *)sub_69F0(4, v9);
      v9 = dcgettext(0, "cannot open %s for reading", 5);
      v23 = __errno_location();
      error(1, *v23, v9, v5);
      goto LABEL_52;
    }
  }
  v14 = fileno(v13);
  if ( !fstat(v14, &buf) && (buf.st_mode & 0xF000) == 0x8000 )
  {
    st_size_low = (double)SLODWORD(buf.st_size);
    v44 = 10485760.0;
    if ( sub_4BD0() * 0.5 <= 10485760.0 )
      v44 = 0.5 * sub_4BD0();
    if ( v44 >= st_size_low )
    {
      sub_71F0(&v64);
      v61 = sub_72A0(v13);
      if ( !v61 || (unsigned int)sub_47E0(v13) )
      {
        v49 = sub_69F0(4, v9);
        v50 = dcgettext(0, "cannot read file names from %s", 5);
        error(1, 0, v50, v49);
        goto LABEL_115;
      }
      v6 = v64;
      v33 = v65;
      v13 = (FILE *)sub_4580(v65, &v64);
      if ( !v13 )
        goto LABEL_115;
      v45 = 1;
      if ( v6 )
        v45 = v6;
      ptr = (int *)sub_7D50(v45, 152);
      if ( !v6 )
        goto LABEL_25;
LABEL_79:
      if ( v6 == 1
        && (v15 = (unsigned char)byte_D130
                + (unsigned char)byte_D131
                + (unsigned char)byte_D132
                + (unsigned char)byte_D133
                + (unsigned char)byte_D134,
            v15 == 1) )
      {
        *ptr = 1;
      }
      else
      {
        v52 = v13;
        v34 = 0;
        v35 = (struct stat *)(ptr + 2);
        do
        {
          v37 = v33[v34];
          if ( !v37 || !strcmp(v33[v34], "-") )
            v36 = fstat(0, v35);
          else
            v36 = stat(v37, v35);
          ++v34;
          LODWORD(v35[-1].__unused[2]) = v36;
          v35 = (struct stat *)((char *)v35 + 152);
        }
        while ( v34 < v6 );
        v13 = v52;
        v15 = 1;
        if ( (int)*ptr <= 0 )
        {
          v38 = ptr;
          v39 = 1;
          v40 = 0;
          v41 = 0;
          do
          {
            if ( !*v38 )
            {
              if ( (v38[8] & 0xF000) == 0x8000 )
                v40 += *((long long *)v38 + 7);
              else
                v39 = 7;
            }
            ++v41;
            v38 += 38;
          }
          while ( v41 < v6 );
          v42 = 1;
          if ( v40 > 9 )
          {
            do
            {
              v43 = v40;
              ++v42;
              v40 /= 0xAu;
            }
            while ( v43 > 0x63 );
          }
          v15 = v42;
          if ( v39 >= v42 )
            v15 = v39;
        }
      }
      goto LABEL_26;
    }
  }
  v13 = (FILE *)sub_45B0(v13);
  if ( !v13 )
    goto LABEL_115;
  v61 = 0;
  ptr = (int *)sub_7D50(1, 152);
LABEL_25:
  v6 = 0;
  *ptr = 1;
  v15 = 1;
LABEL_26:
  dword_D12C = v15;
  v3 = (struct option *)&v63;
  v4 = "-";
  v51 = 0;
  v54 = 1;
  while ( 1 )
  {
    v20 = (char *const *)sub_45F0(v13, v3);
    v5 = v20;
    if ( !v20 )
      break;
    if ( v9 )
    {
      if ( strcmp(v9, v4) )
      {
        if ( *(char *)v5 )
          goto LABEL_41;
LABEL_53:
        v25 = sub_4690(v13);
        v56 = dcgettext(0, "invalid zero-length file name", 5);
        v26 = (const char *)sub_6D70(0, 3, v9);
        error(0, 0, "%s:%lu: %s", v26, v25, v56);
        v54 = 0;
        goto LABEL_34;
      }
      if ( strcmp((const char *)v5, v4) )
      {
        if ( *(char *)v5 )
        {
          if ( v6 )
          {
            v21 = v51;
            goto LABEL_43;
          }
          v16 = ptr;
          goto LABEL_31;
        }
        goto LABEL_53;
      }
LABEL_52:
      v55 = sub_69F0(4, v5);
      v24 = dcgettext(0, "when reading file names from stdin, no file name of %s allowed", 5);
      error(0, 0, v24, v55);
      if ( !*(char *)v5 )
        goto LABEL_53;
    }
    else
    {
      if ( *(char *)v20 )
      {
LABEL_41:
        v21 = 0;
        if ( v6 )
          v21 = v51;
LABEL_43:
        v60 = (char *)ptr + v21;
        v22 = strcmp((const char *)v5, v4);
        v16 = v60;
        if ( !v22 )
        {
          byte_D128 = 1;
          v19 = sub_3820(0, v5, v60, -1);
LABEL_33:
          v54 &= v19;
          goto LABEL_34;
        }
LABEL_31:
        v58 = v16;
        v17 = open((const char *)v5, 0);
        if ( v17 == -1 || (fd = v17, v59 = sub_3820(v17, v5, v58, 0), v18 = close(fd), v19 = v59, v18) )
        {
          v31 = (const char *)sub_6D70(0, 3, v5);
          v32 = __errno_location();
          error(0, *v32, "%s", v31);
          v19 = 0;
        }
        goto LABEL_33;
      }
      v27 = dcgettext(0, "invalid zero-length file name", 5);
      error(0, 0, "%s", v27);
    }
    v54 = 0;
LABEL_34:
    if ( !v6 )
      *ptr = 1;
    v51 += 152;
  }
  switch ( v63 )
  {
    case 3:
LABEL_115:
      sub_8110();
    case 4:
      v5 = (char *const *)sub_6D70(0, 3, v9);
      v9 = dcgettext(0, "%s: read error", 5);
      v28 = __errno_location();
      error(0, *v28, v9, v5);
      v54 = 0;
      break;
    case 2:
      LODWORD(v6) = (unsigned char)v54 & (v9 == 0);
      if ( ((unsigned char)v54 & (v9 == 0)) != 0 )
      {
        v54 &= v9 == 0;
        if ( !sub_4690(v13) )
        {
          byte_D128 = 1;
          v54 = sub_3820(0, 0, ptr, -1);
        }
      }
      break;
    default:
      __assert_fail("!\"unexpected error code from argv_iter\"", "src/wc.c", 0x3AAu, "main");
  }
  if ( v61 )
    sub_7270(&v64);
  if ( (unsigned long long)sub_4690(v13) > 1 )
  {
    v29 = dcgettext(0, "total", 5);
    sub_3430(qword_D158, qword_D150, qword_D148, qword_D140, qword_D138, v29);
  }
  sub_46B0(v13);
  free(ptr);
  if ( byte_D128 && close(0) )
  {
    v10 = (char *const *)(unsigned int)*__errno_location();
    error(1, (int)v10, "-");
    goto LABEL_75;
  }
  return (unsigned char)v54 ^ 1u;
}



// Function: write_counts @ 0x3430
unsigned long long write_counts(long long a1, long long a2, long long a3, long long a4, long long a5, const char *a6)
{
  const char *v10; // rbp
  char *IO_write_ptr; // rax
  long long v13; // rax
  long long v14; // rax
  const char *v15; // rsi
  long long v16; // rax
  const char *v17; // rsi
  long long v18; // rax
  const char *v19; // rsi
  const char *v20; // rax
  char v21[24]; // [rsp+0h] [rbp-58h] BYREF
  unsigned long long v22; // [rsp+18h] [rbp-40h]

  v10 = "%*s";
  v22 = __readfsqword(0x28u);
  if ( byte_D134 )
  {
    v20 = (const char *)sub_4940(a1, v21);
    v10 = " %*s";
    __printf_chk(1, "%*s", dword_D12C, v20);
  }
  if ( byte_D133 )
  {
    v18 = sub_4940(a2, v21);
    v19 = v10;
    v10 = " %*s";
    __printf_chk(1, v19, (unsigned int)dword_D12C, v18);
  }
  if ( byte_D132 )
  {
    v16 = sub_4940(a3, v21);
    v17 = v10;
    v10 = " %*s";
    __printf_chk(1, v17, (unsigned int)dword_D12C, v16);
  }
  if ( byte_D131 )
  {
    v14 = sub_4940(a4, v21);
    v15 = v10;
    v10 = " %*s";
    __printf_chk(1, v15, (unsigned int)dword_D12C, v14);
  }
  if ( byte_D130 )
  {
    v13 = sub_4940(a5, v21);
    __printf_chk(1, v10, (unsigned int)dword_D12C, v13);
  }
  if ( a6 )
  {
    if ( strchr(a6, 10) )
      a6 = (const char *)sub_6D70(0, 3, a6);
    __printf_chk(1, " %s", a6);
  }
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
  return v22 - __readfsqword(0x28u);
}



// Function: wc @ 0x3820
// positive sp value has been detected, the output may be wrong!
long long wc(unsigned int a1, const char *a2, long long a3, long long a4)
{
  size_t mb_cur_max; // rax
  char v7; // r12
  unsigned int v8; // r15d
  int v9; // eax
  long long v10; // r12
  unsigned long long v11; // r13
  long long v12; // rcx
  bool v13; // si
  __off_t v14; // rbp
  long long v15; // rax
  unsigned long long v16; // rcx
  unsigned long long v17; // rax
  unsigned long long v18; // rdx
  __off_t v19; // r12
  long long v20; // rax
  long long v21; // r12
  unsigned long long v22; // rbp
  long long v23; // rbx
  long long v25; // r14
  int v26; // r15d
  long long v27; // rbx
  long long v28; // rax
  unsigned char *v29; // r13
  int v30; // r9d
  long long v31; // r15
  unsigned char v32; // cl
  int v33; // r14d
  const unsigned short **v34; // rax
  unsigned short v35; // ax
  long long v36; // r14
  long long v37; // rax
  long long v38; // r8
  wint_t v39; // edi
  int v40; // eax
  int v41; // eax
  long long v42; // rbp
  unsigned long long v43; // r13
  unsigned long long v44; // rbx
  long long v45; // r12
  long long v46; // rax
  unsigned char *v47; // r14
  unsigned char *v48; // rdx
  unsigned char v49; // di
  const unsigned short **v50; // rax
  unsigned short v51; // ax
  wint_t v52; // eax
  int v53; // eax
  long long v54; // r15
  int *v55; // rax
  const char *v56; // rcx
  unsigned char v57; // al
  long long v58; // r12
  long long v59; // rbx
  unsigned char v60; // al
  const char *v61; // r14
  int *v62; // rax
  long long v63; // r12
  int *v64; // rax
  const char *v65; // rcx
  __off_t v66; // rax
  const char *v87; // rsi
  long long (*v88)(); // rax
  unsigned int v89; // eax
  char *v90; // rax
  char *v91; // rax
  unsigned char v92; // [rsp-78h] [rbp-40A8h]
  unsigned char v93; // [rsp-78h] [rbp-40A8h]
  unsigned char v94; // [rsp-78h] [rbp-40A8h]
  unsigned long long v95; // [rsp-78h] [rbp-40A8h]
  long long v96; // [rsp-70h] [rbp-40A0h]
  unsigned char *v97; // [rsp-70h] [rbp-40A0h]
  mbstate_t v98; // [rsp-68h] [rbp-4098h]
  unsigned long long v99; // [rsp-60h] [rbp-4090h]
  unsigned char v101; // [rsp-49h] [rbp-4079h]
  char *v103; // [rsp-40h] [rbp-4070h]
  wint_t v104; // [rsp-34h] [rbp-4064h] BYREF
  long long v105; // [rsp-30h] [rbp-4060h] BYREF
  long long v106; // [rsp-28h] [rbp-4058h] BYREF
  mbstate_t v107; // [rsp-20h] [rbp-4050h] BYREF
  char v108[24]; // [rsp-18h] [rbp-4048h] BYREF
  char v109; // [rsp+0h] [rbp-4030h] BYREF
  long long v110; // [rsp+1000h] [rbp-3030h] BYREF
  unsigned long long v111; // [rsp+3FF0h] [rbp-40h]

  while ( &v109 != (char *)(&v110 - 2048) )
    ;
  v111 = __readfsqword(0x28u);
  v103 = (char *)a2;
  if ( !a2 )
    v103 = dcgettext(0, "standard input", 5);
  v106 = 0;
  v105 = 0;
  mb_cur_max = __ctype_get_mb_cur_max();
  v7 = byte_D131;
  if ( mb_cur_max <= 1 )
  {
    v101 = 0;
    if ( !byte_D131 )
      v7 = byte_D132;
  }
  else
  {
    v101 = byte_D132;
  }
  if ( byte_D133 || (v8 = (unsigned char)byte_D130, byte_D130) )
  {
    j__posix_fadvise(a1, 0, 0, 2);
  }
  else
  {
    if ( v7 == 1 && !v101 )
    {
      if ( !byte_D134 )
        goto LABEL_11;
      j__posix_fadvise(a1, 0, 0, 2);
LABEL_130:
      if ( !byte_D134 )
      {
LABEL_11:
        v9 = *(int *)a3;
        if ( *(int *)a3 > 0 )
        {
          v9 = fstat(a1, (struct stat *)(a3 + 8));
          *(int *)a3 = v9;
        }
        if ( v9 || (*(int *)(a3 + 32) & 0xD000) != 0x8000 || (v10 = *(long long *)(a3 + 56), v10 < 0) )
        {
LABEL_25:
          j__posix_fadvise(a1, 0, 0, 2);
          while ( 1 )
          {
            v20 = sub_7490(a1, v108, 0x4000u);
            if ( !v20 )
            {
              v12 = v106;
              v21 = 0;
              v22 = 0;
              v23 = 0;
              v8 = 1;
              goto LABEL_30;
            }
            if ( v20 == -1 )
              break;
            v106 += v20;
          }
          v22 = 0;
          v23 = 0;
          v63 = sub_6D70(0, 3, v103);
          v64 = __errno_location();
          v65 = (const char *)v63;
          v21 = 0;
          error(0, *v64, "%s", v65);
          v12 = v106;
          goto LABEL_30;
        }
        v11 = qword_D120;
        v12 = *(long long *)(a3 + 56);
        if ( a4 == -1 )
        {
          v95 = *(long long *)(a3 + 56);
          v66 = lseek(a1, 0, 1);
          v14 = v66;
          if ( !(v95 % v11) )
          {
            v13 = v66 >= 0;
            goto LABEL_19;
          }
          if ( v66 <= v95 )
            v12 = v95 - v66;
          else
            v12 = 0;
        }
        else if ( !(v10 % (unsigned long long)qword_D120) )
        {
          v13 = 1;
          v14 = 0;
LABEL_19:
          v15 = *(long long *)(a3 + 64);
          v16 = v15 - 1;
          v17 = v15 + 1;
          v18 = 513;
          if ( v16 <= 0x1FFFFFFFFFFFFFFFLL )
            v18 = v17;
          v19 = v10 - v10 % v18;
          if ( v19 > v14 && v13 && lseek(a1, v19, 1) >= 0 )
            v106 = v19 - v14;
          goto LABEL_25;
        }
        v106 = v12;
        v22 = 0;
        v21 = 0;
        v23 = 0;
        v8 = 1;
        goto LABEL_30;
      }
LABEL_131:
      _RAX = 0;
      __asm { cpuid }
      if ( (int)_RAX )
      {
        _RAX = 1;
        __asm { cpuid }
        if ( (_RCX & 0x8000000) == 0 )
          goto LABEL_135;
        _RAX = 0;
        __asm { cpuid }
        if ( (unsigned int)_RAX > 6 )
        {
          _RAX = 7;
          __asm { cpuid }
          if ( (_RBX & 0x20) != 0 )
          {
            if ( byte_D160 )
            {
              v91 = dcgettext(0, "using avx2 hardware support", 5);
              error(0, 0, "%s", v91);
            }
            v88 = (long long (*)())&sub_86A0;
            off_D010 = (long long (*)())&sub_86A0;
            goto LABEL_137;
          }
LABEL_135:
          v87 = "avx2 support not detected";
          if ( !byte_D160 )
          {
LABEL_136:
            v88 = off_D010;
LABEL_137:
            v22 = 0;
            v21 = 0;
            v23 = 0;
            v89 = ((long long (*)(char *, long long, long long *, long long *))v88)(v103, a1, &v105, &v106);
            v12 = v106;
            v8 = v89;
            goto LABEL_30;
          }
LABEL_140:
          v90 = dcgettext(0, v87, 5);
          error(0, 0, "%s", v90);
          goto LABEL_136;
        }
      }
      if ( !byte_D160 )
        goto LABEL_136;
      v87 = "failed to get cpuid";
      goto LABEL_140;
    }
    j__posix_fadvise(a1, 0, 0, 2);
    if ( v101 != 1 )
    {
      if ( v7 )
        goto LABEL_130;
      if ( !v101 )
        goto LABEL_131;
    }
  }
  if ( __ctype_get_mb_cur_max() <= 1 )
  {
    v42 = 0;
    v43 = 0;
    v44 = 0;
    v45 = 0;
    while ( 2 )
    {
      v46 = sub_7490(a1, v108, 0x4000u);
      if ( v46 )
      {
        if ( v46 != -1 )
        {
          v106 += v46;
          v47 = v108;
          v48 = &v108[v46];
LABEL_90:
          v49 = *v47++;
          switch ( v49 )
          {
            case 9u:
              v43 = (v43 & 0xFFFFFFFFFFFFFFF8LL) + 8;
              goto LABEL_105;
            case 0xAu:
              ++v105;
              goto LABEL_102;
            case 0xBu:
              goto LABEL_105;
            case 0xCu:
            case 0xDu:
LABEL_102:
              if ( v44 < v43 )
                v44 = v43;
              v43 = 0;
              goto LABEL_105;
            case 0x20u:
              ++v43;
              goto LABEL_105;
            default:
              v97 = v48;
              v50 = __ctype_b_loc();
              v48 = v97;
              v51 = (*v50)[v49];
              if ( (v51 & 0x4000) == 0 )
                goto LABEL_99;
              ++v43;
              if ( (v51 & 0x2000) != 0 )
                goto LABEL_105;
              v52 = btowc(v49);
              v48 = v97;
              if ( byte_D118 )
              {
                v45 = (unsigned char)byte_D118;
              }
              else if ( v52 == 160 || v52 == 8199 || v52 == 8239 || v52 == 8288 )
              {
LABEL_105:
                v42 += v45;
                v45 = 0;
              }
              else
              {
                v45 = 1;
              }
LABEL_99:
              if ( v48 == v47 )
                continue;
              goto LABEL_90;
          }
        }
        v60 = v45;
        v58 = v42;
        v22 = v44;
        v59 = v60;
        v8 = 0;
        v61 = (const char *)sub_6D70(0, 3, v103);
        v62 = __errno_location();
        error(0, *v62, "%s", v61);
      }
      else
      {
        v57 = v45;
        v8 = 1;
        v58 = v42;
        v22 = v44;
        v59 = v57;
      }
      break;
    }
    v12 = v106;
    if ( v22 < v43 )
      v22 = v43;
    v23 = v58 + v59;
    v21 = 0;
    goto LABEL_30;
  }
  v25 = 0;
  v26 = 0;
  v22 = 0;
  v107 = 0;
  v27 = 0;
  v21 = 0;
  v99 = 0;
  v96 = 0;
LABEL_38:
  v28 = sub_7490(a1, &v108[v25], 0x4000 - v25);
  if ( !v28 )
  {
    v8 = 1;
    goto LABEL_114;
  }
  if ( v28 != -1 )
  {
    v106 += v28;
    v29 = v108;
    v30 = v26;
    v31 = v28 + v25;
    while ( 1 )
    {
      if ( !(char)v30 )
      {
        v32 = *v29;
        v33 = (dword_99E0[(*v29 >> 5) & 7] >> *v29) & 1;
        if ( v33 )
        {
          v104 = (char)v32;
          switch ( v32 )
          {
            case 9u:
              v36 = 1;
              goto LABEL_60;
            case 0xAu:
              v36 = 1;
              goto LABEL_58;
            case 0xBu:
              goto LABEL_54;
            case 0xCu:
            case 0xDu:
              v36 = 1;
              goto LABEL_50;
            case 0x20u:
              v36 = 1;
              goto LABEL_56;
            default:
              v92 = v32;
              v34 = __ctype_b_loc();
              v30 = 0;
              v35 = (*v34)[v92];
              if ( (v35 & 0x4000) == 0 )
              {
                v36 = 1;
                goto LABEL_46;
              }
              ++v22;
              if ( (v35 & 0x2000) != 0 )
              {
LABEL_54:
                v36 = 1;
                goto LABEL_53;
              }
              v27 = (unsigned char)v33;
              v36 = 1;
              break;
          }
          goto LABEL_46;
        }
      }
      v98 = v107;
      v37 = ((long long (*)(wint_t *, unsigned char *, long long, mbstate_t *))sub_4A50)(
              &v104,
              v29,
              v31,
              &v107);
      v36 = v37;
      if ( v37 == -2 )
      {
        v25 = v31;
        v107 = v98;
        if ( v31 )
        {
          if ( v31 == 0x4000 )
          {
            ++v29;
            v25 = 0x3FFF;
          }
          ((void (*)(char *, unsigned char *, long long, long long, long long, mbstate_t *))__memmove_chk)(
            v108,
            v29,
            v25,
            16385,
            v38,
            &v107);
        }
        v26 = 1;
        goto LABEL_38;
      }
      if ( v37 == -1 )
      {
        ++v29;
        --v31;
        v30 = 1;
        goto LABEL_47;
      }
      LOBYTE(v30) = mbsinit(&v107) == 0;
      if ( v36 )
      {
        switch ( v104 )
        {
          case 9u:
LABEL_60:
            v22 = (v22 & 0xFFFFFFFFFFFFFFF8LL) + 8;
            goto LABEL_53;
          case 0xAu:
LABEL_58:
            ++v105;
            break;
          case 0xBu:
            goto LABEL_53;
          case 0xCu:
          case 0xDu:
            break;
          case 0x20u:
LABEL_56:
            ++v22;
            goto LABEL_53;
          default:
            v39 = v104;
            goto LABEL_65;
        }
LABEL_50:
        if ( v99 >= v22 )
          v22 = v99;
        v99 = v22;
        v22 = 0;
      }
      else
      {
        v104 = 0;
        v39 = 0;
        v36 = 1;
LABEL_65:
        v93 = v30;
        v40 = iswprint(v39);
        v30 = v93;
        if ( !v40 )
          goto LABEL_46;
        if ( byte_D130 )
        {
          v53 = wcwidth(v104);
          LOBYTE(v30) = v93;
          if ( v53 > 0 )
            v22 += v53;
        }
        v94 = v30;
        v41 = iswspace(v104);
        v30 = v94;
        if ( !v41 )
        {
          if ( byte_D118 )
          {
            v27 = (unsigned char)byte_D118;
            goto LABEL_46;
          }
          if ( v104 != 160 && v104 != 8199 && v104 != 8239 && v104 != 8288 )
          {
            v27 = 1;
            goto LABEL_46;
          }
        }
      }
LABEL_53:
      v96 += v27;
      v27 = 0;
LABEL_46:
      v29 += v36;
      v31 -= v36;
      ++v21;
LABEL_47:
      if ( !v31 )
      {
        v25 = 0;
        v26 = v30;
        goto LABEL_38;
      }
    }
  }
  v54 = sub_6D70(0, 3, v103);
  v55 = __errno_location();
  v56 = (const char *)v54;
  v8 = 0;
  error(0, *v55, "%s", v56);
LABEL_114:
  v12 = v106;
  if ( v99 >= v22 )
    v22 = v99;
  v23 = v96 + v27;
LABEL_30:
  if ( (unsigned char)byte_D132 > v101 )
    v21 = v12;
  sub_3430(v105, v23, v21, v12, v22, a2);
  qword_D150 += v23;
  qword_D158 += v105;
  qword_D148 += v21;
  qword_D140 += v106;
  if ( qword_D138 < v22 )
    qword_D138 = v22;
  return v8;
}



// Function: usage @ 0x4230
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
  const char *v18; // rsi
  const char *v19; // r14
  char *v20; // rax
  long long v21; // r8
  long long v22; // r9
  char *v23; // rdi
  const char *v24; // r13
  char *v25; // rax
  const char *v26; // rcx
  char *v27; // r13
  char *v28; // rax
  FILE *v29; // rbx
  char *v30; // rax
  const char *v31; // [rsp+0h] [rbp-B8h] BYREF
  const char *v32; // [rsp+8h] [rbp-B0h]
  char *v33; // [rsp+10h] [rbp-A8h]
  const char *v34; // [rsp+18h] [rbp-A0h]
  const char *v35; // [rsp+20h] [rbp-98h]
  const char *v36; // [rsp+28h] [rbp-90h]
  const char *v37; // [rsp+30h] [rbp-88h]
  const char *v38; // [rsp+38h] [rbp-80h]
  const char *v39; // [rsp+40h] [rbp-78h]
  const char *v40; // [rsp+48h] [rbp-70h]
  const char *v41; // [rsp+50h] [rbp-68h]
  const char *v42; // [rsp+58h] [rbp-60h]
  long long v43; // [rsp+60h] [rbp-58h]
  long long v44; // [rsp+68h] [rbp-50h]
  unsigned long long v45; // [rsp+78h] [rbp-40h]

  v2 = qword_D178;
  v45 = __readfsqword(0x28u);
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = &v31;
    v5 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5);
    __printf_chk(1, v5, v2, v2);
    v6 = stdout;
    v7 = dcgettext(
           0,
           "Print newline, word, and byte counts for each FILE, and a total line if\n"
           "more than one FILE is specified.  A word is a non-zero-length sequence of\n"
           "printable characters delimited by white space.\n",
           5);
    fputs_unlocked(v7, v6);
    v8 = stdout;
    v9 = dcgettext(0, "\nWith no FILE, or when FILE is -, read standard input.\n", 5);
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = dcgettext(
            0,
            "\n"
            "The options below may be used to select which counts are printed, always in\n"
            "the following order: newline, word, character, byte, maximum line length.\n"
            "  -c, --bytes            print the byte counts\n"
            "  -m, --chars            print the character counts\n"
            "  -l, --lines            print the newline counts\n",
            5);
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = dcgettext(
            0,
            "      --files0-from=F    read input from the files specified by\n"
            "                           NUL-terminated names in file F;\n"
            "                           If F is - then read names from standard input\n"
            "  -L, --max-line-length  print the maximum display width\n"
            "  -w, --words            print the word counts\n",
            5);
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v17, v16);
    v31 = "[";
    v18 = "[";
    v32 = "test invocation";
    v33 = "coreutils";
    v34 = "Multi-call invocation";
    v37 = "sha256sum";
    v35 = "sha224sum";
    v39 = "sha384sum";
    v36 = "sha2 utilities";
    v38 = "sha2 utilities";
    v40 = "sha2 utilities";
    v41 = "sha512sum";
    v42 = "sha2 utilities";
    v43 = 0;
    v44 = 0;
    do
    {
      if ( !strcmp("wc", v18) )
        break;
      v18 = v4[2];
      v4 += 2;
    }
    while ( v18 );
    v19 = v4[1];
    if ( !v19 )
      v19 = "wc";
    v20 = dcgettext(0, "\n%s online help: <%s>\n", 5);
    __printf_chk(
      1,
      v20,
      "GNU coreutils",
      "https://www.gnu.org/software/coreutils/",
      v21,
      v22,
      v31,
      v32,
      v33,
      v34,
      v35,
      v36,
      v37,
      v38,
      v39,
      v40,
      v41,
      v42,
      v43,
      v44);
    v23 = setlocale(5, 0);
    if ( v23 && strncmp(v23, "en_", 3u) )
    {
      v29 = stdout;
      v30 = dcgettext(0, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
      fputs_unlocked(v30, v29);
    }
    v24 = "wc";
    if ( !strcmp("wc", "[") )
      v24 = "test";
    v25 = dcgettext(0, "Full documentation <%s%s>\n", 5);
    v26 = v24;
    v27 = " invocation";
    __printf_chk(1, v25, "https://www.gnu.org/software/coreutils/", v26);
    if ( v19 != "wc" )
      v27 = (char *)"";
    v28 = dcgettext(0, "or available locally via: info '(coreutils) %s%s'\n", 5);
    __printf_chk(1, v28, v19, v27);
  }
  exit(status);
}



// Function: wc_lines_avx2 @ 0x86a0
long long wc_lines_avx2(
        long long a1,
        int a2,
        long long *a3,
        long long *a4,
        __m128 _XMM0,
        __m128 _XMM1,
        __m128 _XMM2,
        __m128 _XMM3)
{
  unsigned int v8; // r13d
  long long v11; // rbx
  long long v12; // r14
  unsigned long long v13; // rax
  char *v15; // rdi
  char *v17; // rdx
  char *v19; // rax
  int v31; // r8d
  int v33; // r8d
  const char *v36; // r12
  int *v37; // rax
  char buf[4064]; // [rsp+20h] [rbp-4008h] BYREF
  unsigned long long v42; // [rsp+3FF8h] [rbp-30h]

  v42 = __readfsqword(0x28u);
  LOBYTE(v8) = a4 == 0 || a3 == 0;
  if ( (char)v8 )
  {
    return 0;
  }
  else
  {
    v11 = 0;
    v12 = 0;
    while ( 1 )
    {
      __asm { vzeroupper }
      v13 = sub_7490(a2, buf, 0x3FC0u);
      __asm { vmovdqa ymm4, cs:ymmword_A1C0 }
      if ( !v13 )
      {
        v8 = 1;
        *a3 = v12;
        *a4 = v11;
        __asm { vzeroupper }
        return v8;
      }
      if ( v13 == -1 )
        break;
      v11 += v13;
      v15 = &buf[v13];
      if ( v13 <= 0x3F )
      {
        __asm { vpxor   xmm0, xmm0, xmm0 }
        v19 = buf;
        __asm { vmovdqa ymm1, ymm0 }
      }
      else
      {
        __asm { vpxor   xmm3, xmm3, xmm3 }
        v17 = buf;
        __asm { vmovdqa ymm2, ymm3 }
        v19 = &buf[((v13 - 64) & 0xFFFFFFFFFFFFFFC0LL) + 64];
        do
        {
          __asm
          {
            vpcmpeqb ymm1, ymm4, ymmword ptr [rdx]
            vpcmpeqb ymm0, ymm4, ymmword ptr [rdx+20h]
          }
          v17 += 64;
          __asm
          {
            vpsubb  ymm1, ymm2, ymm1
            vpsubb  ymm0, ymm3, ymm0
            vmovdqa ymm2, ymm1
            vmovdqa ymm3, ymm0
          }
        }
        while ( v17 != v19 );
      }
      __asm
      {
        vpxor   xmm2, xmm2, xmm2
        vpsadbw ymm1, ymm1, ymm2
        vpsadbw ymm0, ymm0, ymm2
        vmovdqa xmm3, xmm1
        vextracti128 xmm1, ymm1, 1
        vmovdqa xmm2, xmm0
        vpextrw r8d, xmm3, 0
        vpextrw rdx, xmm3, 4
        vextracti128 xmm0, ymm0, 1
      }
      LODWORD(_RDX) = _R8D + _RDX;
      __asm
      {
        vpextrw r8d, xmm1, 0
        vpextrw r9d, xmm2, 0
      }
      LODWORD(_RDX) = _R8D + _RDX;
      __asm { vpextrw r8d, xmm1, 4 }
      LODWORD(_RDX) = _R8D + _RDX;
      __asm { vpextrw r8d, xmm2, 4 }
      v31 = _R9D + _R8D;
      __asm { vpextrw r9d, xmm0, 0 }
      v33 = _R9D + v31;
      __asm { vpextrw r9d, xmm0, 4 }
      for ( v12 += _R9D + v33 + (long long)(int)_RDX; v15 != v19; v12 += *(v19 - 1) == 10 )
        ++v19;
    }
    __asm { vzeroupper }
    v36 = (const char *)sub_6D70(0, 3, a1);
    v37 = __errno_location();
    error(0, *v37, "%s", v36);
  }
  return v8;
}


