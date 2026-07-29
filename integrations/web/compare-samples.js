// compare-samples.js — the data behind the "compare" section of the landing page.
//
// Each sample is one function, decompiled by kuna, shown next to the same
// function as rendered by something else (another decompiler, or the original
// source). The page reads only this file — adding a sample is a data edit, no
// markup changes.
//
//   RIVALS   the right-hand pane's options, in dropdown order.
//   SAMPLES  the left dropdown. `kuna` is the left pane; `vs[<rival id>]` the
//            right one. A rival with no entry renders a "not recorded yet"
//            placeholder, so a half-filled sample is honest rather than empty.
//            `ged` (optional) is the measured DecBench score per decompiler;
//            the page prints the kuna/rival pair under the dropdowns.
//
// There is deliberately NO per-sample commentary field. The page shows the
// provenance line and the measured score, and lets the two panes speak; a
// caption telling the reader what to think would be the only thing on the page
// that is not machine-derived. Keep it that way.
//
// PROVENANCE. Every pane below is verbatim tool output. The stripped-binary
// samples come from one DecBench run (https://decbench.com) in which all five
// decompilers were given the SAME fully stripped, gcc -O2 binary; DecBench then
// renamed each function's own `sub_<addr>` placeholder to its DWARF name, for
// every decompiler equally. That rename is the only edit — callees stay
// `sub_`/`dat_` in every pane, which is what a stripped binary looks like.
// `ged` is DecBench's graph edit distance between the original source CFG and
// the decompiled one: lower is better, 0 = structurally exact.
//
// To add a comparison — mine it, do not hand-pick it:
//
//   python3 -m scripts.decbench.showcase --perfect --dump /tmp/cands
//   python3 -m scripts.decbench.showcase --dump /tmp/cands --verify   # still reproduces?
//   ... read all six panes of every candidate, reject the ones that flatter kuna ...
//   python3 -m scripts.decbench.showcase --dump /tmp/cands --emit picks.json
//
// The full procedure, including what disqualifies a sample, is
// `docs/decbench-loop.md` → *Finding good kuna examples*. `--verify` re-runs the
// current build over every pane below; keep it passing.
//
// Do not paste anything hand-written or reconstructed into a pane — the section
// is a claim about what these tools actually print.

export const RIVALS = [
  { id: 'source', name: 'Original C source' },
  { id: 'ghidra', name: 'Ghidra' },
  { id: 'ida', name: 'IDA (Hex-Rays)' },
  { id: 'binja', name: 'Binary Ninja' },
  { id: 'angr', name: 'angr' },
];

// The right-hand pane defaults to this rival (it should be one that has output
// recorded for most samples, so the section does not open on a placeholder).
export const DEFAULT_RIVAL = 'ida';

export const SAMPLES = [
  // This sample deviates from the one-DecBench-run provenance above: the kuna
  // and IDA panes were regenerated locally on 2026-07-29 (kuna 0.1.0 release
  // build @ HEAD; IDA Professional 9.2 via decbench's idalib backend, code
  // normalization included) against the classic SAILR `fmt` binary
  // (angr-dev binaries/tests/x86_64/decompiler/fmt, coreutils, gcc 8.4.0 -O2,
  // `strip --strip-all`), with decbench's one relabel (`main`) applied. The
  // ghidra/binja/angr panes are not recorded for THIS binary yet, and the GED
  // row is omitted: DecBench's scores measured a different fmt build.
  {
    id: 'O2-coreutils-fmt-main',
    name: "main() — coreutils fmt, x86-64",
    meta: "coreutils fmt · ELF x86-64 · gcc 8.4.0 -O2, stripped · 0x1900",
    kuna:
`long long main(uint4 a0,void *a1)
{
  uint1 *v1;
  unsigned long v10; // rax
  char v11 [16];
  int8 v12; // rcx
  uint8 v13; // rdx
  uint8 v14; // rdx
  uint8 v15; // rdx
  uint8 v16; // rdx
  uint8 v17;
  uint8 v18; // rdx
  unsigned long v19; // rdx
  bool v2;
  uint1 *v20; // rsi
  uint1 *v21;
  uint1 *v22; // rdi
  uint1 *v23;
  uint4 v24; // eax
  char *v25;
  char *v26;
  char *v27;
  bool v28; // cf
  bool v29;
  int4 v3; // eax
  uint1 v30; // df
  unsigned long v31; // stack - 0x38
  uint4 v4;
  char v5 [16];
  char *v6; // rax
  void *v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  v30 = 0;
  v25 = (char *)0x0;
  sub_3610(*a1);
  setlocale(6,"");
  bindtextdomain(0x68d8,"/usr/local/share/locale");
  textdomain(0x68d8);
  sub_67f0(sub_3400);
  dat_215130 = 0;
  dat_215131 = 0;
  dat_215132 = 0;
  dat_215133 = 0;
  dat_215120 = 0x4b;
  dat_215128 = "";
  dat_21511c = 0;
  dat_215118 = 0;
  dat_215114 = 0;
  if (((2 <= (int4)a0) && (v26 = (char *)a1[1], *v26 == '-')) && ((uint4)((int4)v26[1] - 0x30U) <= 9)) {
    v25 = &v26[1];
    v9 = *a1;
    a0 = a0 - 1;
    a1 = &a1[1];
    *a1 = v9;
  }
  v26 = (char *)0x0;
  while (v3 = getopt_long(a0,a1,"0123456789cstuw:p:g:",0x209aa0,0), v3 != -1) {
    switch(v3) {
      default:
        if ((uint4)(v3 - 0x30U) <= 9) {
          error(0,0,dcgettext(0,"invalid option -- %c; -WIDTH is recognized only when it is the first\\noption; use -w N instead",5),v3);
        }
                    /* WARNING: Subroutine does not return */
        sub_3000(1);
      case 99:
        dat_215133 = 1;
        break;
      case 0x67:
        v26 = dat_20a0a0;
        break;
      case 0x70:
        dat_215118 = 0;
        v27 = dat_20a0a0;
        if (*dat_20a0a0 == ' ') {
          do {
            dat_215118 = (1U - (int4)dat_20a0a0) + (int4)v27;
            v27 = &v27[1];
          } while (*v27 == ' ');
        }
        dat_215128 = v27;
        dat_21511c = (int4)strlen(v27);
        v6 = &v27[dat_21511c];
        if (v27 < v6) {
          do {
            if (v6[-1] != ' ') break;
            v6 = &v6[-1];
          } while (v27 != v6);
        }
        *v6 = '\\0';
        dat_215114 = (int4)v6 - (int4)v27;
        break;
      case 0x73:
        dat_215131 = 1;
        break;
      case 0x74:
        dat_215132 = 1;
        break;
      case 0x75:
        dat_215130 = 1;
        break;
      case 0x77:
        v25 = dat_20a0a0;
        break;
      case -0x83:
        sub_5920(dat_20a088,0x685a,"GNU coreutils",dat_20a010,"Ross Paterson",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      case -0x82:
                    /* WARNING: Subroutine does not return */
        sub_3000(0);
      
    }
  }
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (v25 != (char *)0x0) {
    v3 = sub_6010(v25,0,0x9c4,0x7661,dcgettext(0,"invalid width",5),0);
    dat_215120 = v3;
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
    if (v26 != (char *)0x0) {
      dat_215110 = sub_6010(v26,0,(int8)v3,0x7661,dcgettext(0,"invalid width",5),0);
      v17 = v13;
    }
    else {
label_1d19:
      dat_215110 = (dat_215120 * 0xbb) / 200;
      v17 = (int8)(dat_215120 * 0xbb) % 200 & 0xffffffff;
    }
  }
  else {
    if (v26 == (char *)0x0) goto label_1d19;
    dat_215110 = sub_6010(v26,0,0x4b,0x7661,dcgettext(0,"invalid width",5),0);
    dat_215120 = dat_215110 + 10;
    v17 = v18;
  }
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (dat_20a098 != a0) {
    v24 = 1;
    if ((int4)a0 <= (int4)dat_20a098) {
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
      v5[0] = (v24 ^ 1) & 0xff;
      v5[4] = 0;
      v5[8] = v31;
      return v5;
    }
    v28 = 0;
    v2 = 0;
    v29 = 1;
    v24 = 1;
    v4 = dat_20a098;
    do {
      v12 = 2;
      v1 = (uint1 *)a1[(int4)v4];
      v21 = v1;
      v23 = (uint1 *)0x6f52;
      do {
        if (v12 == 0) break;
        v12 = v12 + -1;
        v22 = &v23[(uint8)v30 * -2 + 1];
        v20 = &v21[(uint8)v30 * -2 + 1];
        v28 = *v21 < *v23;
        v29 = *v21 == *v23;
        v21 = v20;
        v23 = v22;
      } while (v29);
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
      if ((!v28 && !v29) != v28) {
        v7 = fopen((char *)v1,"r");
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
        if (v7 != (void *)0x0) {
          v24 = v24 & sub_29c0(v7);
          v17 = v15;
        }
        else {
          v9 = sub_5130(4,v1);
          v8 = dcgettext(0,"cannot open %s for reading",5);
          v24 = 0;
          error(0,*(void *)__errno_location(),v8,v9);
          v17 = v16;
        }
      }
      else {
        v2 = 1;
        v4 = sub_29c0(dat_20a090,v1,v17);
        v24 = v24 & v4;
        v17 = v14;
      }
      v4 = dat_20a098 + 1;
      v28 = v4 < a0;
      v29 = v4 == a0;
      dat_20a098 = v4;
    } while ((int4)v4 < (int4)a0);
    if (!v2) {
      v5[0] = (v24 ^ 1) & 0xff;
      v5[4] = 0;
      v5[8] = v31;
      return v5;
    }
  }
  else {
    v24 = sub_29c0(dat_20a090,0x6f52,v17);
  }
  v3 = sub_34f0(dat_20a090);
  if (v3 != 0) {
    v9 = dcgettext(0,"closing standard input",5);
    error(1,*(void *)__errno_location(),0x6f0c,v9);
    v11[8] = v19;
    v11[0] = v10;
    return v11;
  }
  v5[0] = (v24 ^ 1) & 0xff;
  v5[4] = 0;
  v5[8] = v31;
  return v5;
}`,
    vs: {
      source:
`int
main (int argc, char **argv)
{
  int optchar;
  bool ok = true;
  char const *max_width_option = NULL;
  char const *goal_width_option = NULL;

  initialize_main (&argc, &argv);
  set_program_name (argv[0]);
  setlocale (LC_ALL, "");
  bindtextdomain (PACKAGE, LOCALEDIR);
  textdomain (PACKAGE);

  atexit (close_stdout);

  crown = tagged = split = uniform = false;
  max_width = WIDTH;
  prefix = "";
  prefix_length = prefix_lead_space = prefix_full_length = 0;

  if (argc > 1 && argv[1][0] == '-' && ISDIGIT (argv[1][1]))
    {
      /* Old option syntax; a dash followed by one or more digits.  */
      max_width_option = argv[1] + 1;

      /* Make the option we just parsed invisible to getopt.  */
      argv[1] = argv[0];
      argv++;
      argc--;
    }

  while ((optchar = getopt_long (argc, argv, "0123456789cstuw:p:g:",
                                 long_options, NULL))
         != -1)
    switch (optchar)
      {
      default:
        if (ISDIGIT (optchar))
          error (0, 0, _("invalid option -- %c; -WIDTH is recognized\\
 only when it is the first\\noption; use -w N instead"),
                 optchar);
        usage (EXIT_FAILURE);

      case 'c':
        crown = true;
        break;

      case 's':
        split = true;
        break;

      case 't':
        tagged = true;
        break;

      case 'u':
        uniform = true;
        break;

      case 'w':
        max_width_option = optarg;
        break;

      case 'g':
        goal_width_option = optarg;
        break;

      case 'p':
        set_prefix (optarg);
        break;

      case_GETOPT_HELP_CHAR;

      case_GETOPT_VERSION_CHAR (PROGRAM_NAME, AUTHORS);

      }

  if (max_width_option)
    {
      /* Limit max_width to MAXCHARS / 2; otherwise, the resulting
         output can be quite ugly.  */
      max_width = xdectoumax (max_width_option, 0, MAXCHARS / 2, "",
                              _("invalid width"), 0);
    }

  if (goal_width_option)
    {
      /* Limit goal_width to max_width.  */
      goal_width = xdectoumax (goal_width_option, 0, max_width, "",
                               _("invalid width"), 0);
      if (max_width_option == NULL)
        max_width = goal_width + 10;
    }
  else
    {
      goal_width = max_width * (2 * (100 - LEEWAY) + 1) / 200;
    }

  bool have_read_stdin = false;

  if (optind == argc)
    {
      have_read_stdin = true;
      ok = fmt (stdin, "-");
    }
  else
    {
      for (; optind < argc; optind++)
        {
          char *file = argv[optind];
          if (STREQ (file, "-"))
            {
              ok &= fmt (stdin, file);
              have_read_stdin = true;
            }
          else
            {
              FILE *in_stream;
              in_stream = fopen (file, "r");
              if (in_stream != NULL)
                ok &= fmt (in_stream, file);
              else
                {
                  error (0, errno, _("cannot open %s for reading"),
                         quoteaf (file));
                  ok = false;
                }
            }
        }
    }

  if (have_read_stdin && fclose (stdin) != 0)
    die (EXIT_FAILURE, errno, "%s", _("closing standard input"));

  return ok ? EXIT_SUCCESS : EXIT_FAILURE;
}`,
      ida:
`long long main(int a1, char **a2, char **a3)
{
  char *v3; // r12
  int v4; // ebp
  char *const *v5; // rbx
  char *v6; // rax
  char *v7; // r13
  int v8; // r15d
  char *v9; // r15
  int v10; // eax
  char *v11; // rax
  char *v12; // rax
  int v13; // r12d
  char *v14; // rax
  int v15; // eax
  int v16; // r12d
  char v17; // r14
  const char *v18; // r15
  FILE *v19; // rax
  long long v21; // r15
  char *v22; // r12
  int *v23; // rax
  const char *v24; // rdx
  char *v25; // rax
  char *v26; // rax
  char *v27; // rbx
  int *v28; // rax

  v3 = 0;
  v4 = a1;
  v5 = a2;
  sub_3610(*a2);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_67F0(sub_3400);
  byte_215130 = 0;
  byte_215131 = 0;
  byte_215132 = 0;
  byte_215133 = 0;
  dword_215120 = 75;
  s = (char *)"";
  dword_21511C = 0;
  dword_215118 = 0;
  dword_215114 = 0;
  if ( a1 > 1 )
  {
    v6 = a2[1];
    if ( *v6 == 45 && (unsigned int)(v6[1] - 48) <= 9 )
    {
      v3 = v6 + 1;
      v4 = a1 - 1;
      v5 = a2 + 1;
      a2[1] = *a2;
    }
  }
  v7 = 0;
  while ( 1 )
  {
    v8 = getopt_long(v4, v5, "0123456789cstuw:p:g:", &longopts, 0);
    if ( v8 == -1 )
      break;
    if ( v8 == 112 )
    {
      v9 = optarg;
      dword_215118 = 0;
      if ( *optarg == 32 )
      {
        do
          v10 = 1 - (int)optarg + (int)v9++;
        while ( *v9 == 32 );
        dword_215118 = v10;
      }
      s = v9;
      dword_21511C = strlen(v9);
      v11 = &v9[dword_21511C];
      if ( v9 < v11 )
      {
        do
        {
          if ( *(v11 - 1) != 32 )
            break;
          --v11;
        }
        while ( v9 != v11 );
      }
      *v11 = 0;
      dword_215114 = (int)v11 - (int)v9;
    }
    else if ( v8 <= 112 )
    {
      if ( v8 == -130 )
        sub_3000(0);
      if ( v8 <= -130 )
      {
        if ( v8 == -131 )
        {
          sub_5920(
            (int)stdout,
            (unsigned int)"fmt",
            (unsigned int)"GNU coreutils",
            (int)off_20A010,
            (unsigned int)"Ross Paterson",
            0);
          exit(0);
        }
LABEL_53:
        if ( (unsigned int)(v8 - 48) <= 9 )
        {
          v26 = dcgettext(
                  0,
                  "invalid option -- %c; -WIDTH is recognized only when it is the first\\noption; use -w N instead",
                  5);
          error(0, 0, v26, (unsigned int)v8);
        }
        sub_3000(1);
      }
      if ( v8 == 99 )
      {
        byte_215133 = 1;
      }
      else
      {
        if ( v8 != 103 )
          goto LABEL_53;
        v7 = optarg;
      }
    }
    else if ( v8 == 116 )
    {
      byte_215132 = 1;
    }
    else if ( v8 <= 116 )
    {
      if ( v8 != 115 )
        goto LABEL_53;
      byte_215131 = 1;
    }
    else if ( v8 == 117 )
    {
      byte_215130 = 1;
    }
    else
    {
      if ( v8 != 119 )
        goto LABEL_53;
      v3 = optarg;
    }
  }
  if ( v3 )
  {
    v12 = dcgettext(0, "invalid width", 5);
    v13 = sub_6010(v3, 0, 2500, "", v12, 0);
    dword_215120 = v13;
    if ( v7 )
    {
      v14 = dcgettext(0, "invalid width", 5);
      dword_215110 = sub_6010(v7, 0, v13, "", v14, 0);
      goto LABEL_37;
    }
    goto LABEL_50;
  }
  if ( !v7 )
  {
LABEL_50:
    dword_215110 = 187 * dword_215120 / 200;
    goto LABEL_37;
  }
  v25 = dcgettext(0, "invalid width", 5);
  dword_215110 = sub_6010(v7, 0, 75, "", v25, 0);
  dword_215120 = dword_215110 + 10;
LABEL_37:
  v15 = optind;
  if ( optind == v4 )
  {
    LOBYTE(v16) = sub_29C0(stdin);
  }
  else
  {
    LOBYTE(v16) = 1;
    if ( optind >= v4 )
      return (unsigned char)v16 ^ 1u;
    v17 = 0;
    v16 = 1;
    do
    {
      v18 = v5[v15];
      if ( !strcmp(v18, "-") )
      {
        v17 = 1;
        v16 &= sub_29C0(stdin);
      }
      else
      {
        v19 = fopen(v18, "r");
        if ( v19 )
        {
          v16 &= sub_29C0(v19);
        }
        else
        {
          v21 = sub_5130(4, v18);
          v22 = dcgettext(0, "cannot open %s for reading", 5);
          v23 = __errno_location();
          v24 = v22;
          v16 = 0;
          error(0, *v23, v24, v21);
        }
      }
      v15 = optind + 1;
      optind = v15;
    }
    while ( v15 < v4 );
    if ( !v17 )
      return (unsigned char)v16 ^ 1u;
  }
  if ( (unsigned int)sub_34F0(stdin) )
  {
    v27 = dcgettext(0, "closing standard input", 5);
    v28 = __errno_location();
    error(1, *v28, "%s", v27);
    start();
  }
  return (unsigned char)v16 ^ 1u;
}`,
    },
  },

  {
    id: 'O2-openssh-portable-scp-sshbuf_b64tod',
    name: "sshbuf_b64tod() — openssh-portable scp, x86-64",
    meta: "openssh-portable scp · ELF x86-64 · gcc -O2, stripped · 0x2c500",
    ged: { kuna: 0, ida: 15, ghidra: 4, binja: 5, angr: 26 },
    kuna:
`/* WARNING: earlyreturn: hoisted 2 const-guard early-return(s) in sshbuf_b64tod */
int4 sshbuf_b64tod(unsigned long a0,char *a1)

{
  int4 v1; // eax
  uint8 v2; // rax
  void *v3; // rax
  
  v2 = strlen(a1);
  if (v2 == 0) {
    return 0;
  }
  v3 = malloc(v2);
  if (v3 == (void *)0x0) {
    return -2;
  }
  v1 = sub_2f400(a1,v3,v2);
  if (v1 <= -1) {
    sub_301c0(v3,v2);
    return -4;
  }
  v1 = sub_1c4b0(a0,v3,(int8)v1);
  if (v1 <= -1) {
    sub_301c0(v3,v2);
    return v1;
  }
  sub_301c0(v3,v2);
  return 0;
}`,
    vs: {
      source:
`int
sshbuf_b64tod(struct sshbuf *buf, const char *b64)
{
	size_t plen = strlen(b64);
	int nlen, r;
	u_char *p;

	if (plen == 0)
		return 0;
	if ((p = malloc(plen)) == NULL)
		return SSH_ERR_ALLOC_FAIL;
	if ((nlen = b64_pton(b64, p, plen)) < 0) {
		freezero(p, plen);
		return SSH_ERR_INVALID_FORMAT;
	}
	if ((r = sshbuf_put(buf, p, nlen)) < 0) {
		freezero(p, plen);
		return r;
	}
	freezero(p, plen);
	return 0;
}`,
      ghidra:
`int sshbuf_b64tod(undefined8 param_1,char *param_2)

{
  int iVar1;
  size_t __size;
  void *pvVar2;
  
  __size = strlen(param_2);
  iVar1 = 0;
  if (__size != 0) {
    pvVar2 = malloc(__size);
    if (pvVar2 == (void *)0x0) {
      iVar1 = -2;
    }
    else {
      iVar1 = FUN_0012f400(param_2,pvVar2,__size);
      if (-1 < iVar1) {
        iVar1 = FUN_0011c4b0(param_1,pvVar2,(long)iVar1);
        if (-1 < iVar1) {
          FUN_001301c0(pvVar2,__size);
          return 0;
        }
        FUN_001301c0(pvVar2,__size);
        return iVar1;
      }
      FUN_001301c0(pvVar2,__size);
      iVar1 = -4;
    }
  }
  return iVar1;
}`,
      ida:
`double sshbuf_b64tod(long long a1, const char *a2, double a3)
{
  size_t v3; // rbp
  void *v4; // rax
  void *v5; // r14
  int v6; // eax

  v3 = strlen(a2);
  if ( v3 )
  {
    v4 = malloc(v3);
    v5 = v4;
    if ( v4 )
    {
      v6 = sub_2F400(a2, v4, v3);
      if ( v6 >= 0 )
        sub_1C4B0(a1, v5, v6, a3);
      sub_301C0(v5);
    }
  }
  return 0.0;
}`,
      binja:
`int512_t sshbuf_b64tod(int64_t* arg1, char* arg2)

{
    uint64_t n;
    int512_t zmm0;
    int512_t zmm1;
    int512_t zmm2;
    int512_t zmm3;
    int512_t zmm4;
    int512_t zmm5;
    int512_t zmm6;
    int512_t zmm7;
    int512_t zmm8;
    int512_t zmm9;
    int512_t zmm10;
    int512_t zmm11;
    int512_t zmm12;
    int512_t zmm13;
    int512_t zmm14;
    int512_t zmm15;
    n = strlen(arg2);
    
    if (n)
    {
        int64_t rax_1;
        rax_1 = malloc(n);
        
        if (rax_1)
        {
            int32_t rax_2 = sub_42f400(arg2, rax_1, n);
            
            if (rax_2 >= 0)
            {
                int32_t rax_3 = sub_41c4b0(arg1, rax_1, rax_2);
                
                if (rax_3 >= 0)
                {
                    int512_t zmm0_1;
                    int512_t zmm1_1;
                    int512_t zmm2_1;
                    int512_t zmm3_1;
                    int512_t zmm4_1;
                    int512_t zmm5_1;
                    int512_t zmm6_1;
                    int512_t zmm7_1;
                    int512_t zmm8_1;
                    int512_t zmm9_1;
                    int512_t zmm10_1;
                    int512_t zmm11_1;
                    int512_t zmm12_1;
                    int512_t zmm13_1;
                    int512_t zmm14_1;
                    int512_t zmm15_1;
                    zmm0_1 = sub_4301c0(rax_1, n);
                    zmm0_1 = {0};
                    zmm1_1 = {0};
                    zmm2_1 = {0};
                    zmm3_1 = {0};
                    zmm4_1 = {0};
                    zmm5_1 = {0};
                    zmm6_1 = {0};
                    zmm7_1 = {0};
                    zmm8_1 = {0};
                    zmm9_1 = {0};
                    zmm10_1 = {0};
                    zmm11_1 = {0};
                    zmm12_1 = {0};
                    zmm13_1 = {0};
                    zmm14_1 = {0};
                    zmm15_1 = {0};
                    return zmm0_1;
                }
                
                int32_t var_2c = rax_3;
                int512_t zmm0_2;
                int512_t zmm1_2;
                int512_t zmm2_2;
                int512_t zmm3_2;
                int512_t zmm4_2;
                int512_t zmm5_2;
                int512_t zmm6_2;
                int512_t zmm7_2;
                int512_t zmm8_2;
                int512_t zmm9_2;
                int512_t zmm10_2;
                int512_t zmm11_2;
                int512_t zmm12_2;
                int512_t zmm13_2;
                int512_t zmm14_2;
                int512_t zmm15_2;
                zmm0_2 = sub_4301c0(rax_1, n);
                zmm0_2 = {0};
                zmm1_2 = {0};
                zmm2_2 = {0};
                zmm3_2 = {0};
                zmm4_2 = {0};
                zmm5_2 = {0};
                zmm6_2 = {0};
                zmm7_2 = {0};
                zmm8_2 = {0};
                zmm9_2 = {0};
                zmm10_2 = {0};
                zmm11_2 = {0};
                zmm12_2 = {0};
                zmm13_2 = {0};
                zmm14_2 = {0};
                zmm15_2 = {0};
                return zmm0_2;
            }
            
            zmm0 = sub_4301c0(rax_1, n);
        }
    }
    
    zmm0 = {0};
    zmm1 = {0};
    zmm2 = {0};
    zmm3 = {0};
    zmm4 = {0};
    zmm5 = {0};
    zmm6 = {0};
    zmm7 = {0};
    zmm8 = {0};
    zmm9 = {0};
    zmm10 = {0};
    zmm11 = {0};
    zmm12 = {0};
    zmm13 = {0};
    zmm14 = {0};
    zmm15 = {0};
    return zmm0;
}`,
      angr:
`void sshbuf_b64tod(unsigned long a0, unsigned long a1)
{
    unsigned long long v4;  // r14
    unsigned long long v5;  // r13
    unsigned long long v6;  // r12
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    sub_42c514();
    return;
}`,
    },
  },

  {
    id: 'O2-noinline-dpkg-dpkg-packages',
    name: "packages() — dpkg, x86-64",
    meta: "dpkg · ELF x86-64 · gcc -O2 -fno-inline, stripped · 0x14780",
    ged: { kuna: 0, ida: 5, ghidra: 3, binja: 11, angr: 5 },
    kuna:
`unsigned long packages(int8 *a0)

{
  unsigned long v1;
  char *v2; // rsi
  int4 v3; // edi
  
  sub_18fa0();
  v3 = 0;
  if (dat_513c4 == 0) {
    v3 = 4 - (uint4)(uint1)sub_118f0(0x10000);
  }
  sub_1ed10(v3);
  sub_12010();
  sub_201d0();
  sub_257e0("startup packages %s",*dat_254a78);
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (dat_513d0 != 0) {
    if (*a0 != 0) {
      v2 = "--%s --pending does not take any non-option arguments";
      v1 = *dat_254a78;
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
                    /* WARNING: Subroutine does not return */
      sub_26990(dcgettext(0,v2,5),v1);
    }
    sub_13970();
  }
  else {
    if (*a0 == 0) {
      v2 = "--%s needs at least one package name argument";
      v1 = *dat_254a78;
      sub_26990(dcgettext(0,v2,5),v1);
    }
    sub_140d0(a0);
  }
  sub_205a0();
  sub_14190();
  sub_18e90();
  sub_1f340();
  return 0;
}`,
    vs: {
      source:
`int
packages(const char *const *argv)
{
  trigproc_install_hooks();

  modstatdb_open(f_noact ?                  msdbrw_readonly :
                 in_force(FORCE_NON_ROOT) ? msdbrw_write :
                                            msdbrw_needsuperuser);
  checkpath();
  pkg_infodb_upgrade();

  log_message("startup packages %s", cipaction->olong);

  if (f_pending) {
    if (*argv)
      badusage(_("--%s --pending does not take any non-option arguments"),cipaction->olong);

    enqueue_pending();
  } else {
    if (!*argv)
      badusage(_("--%s needs at least one package name argument"), cipaction->olong);

    enqueue_specified(argv);
  }

  ensure_diversions();

  process_queue();
  trigproc_run_deferred();

  modstatdb_shutdown();

  return 0;
}`,
      ghidra:
`undefined8 packages(long *param_1)

{
  byte bVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  int iVar5;
  
  FUN_00118fa0();
  iVar5 = 0;
  if (DAT_001513c4 == 0) {
    bVar1 = FUN_001118f0(0x10000);
    iVar5 = 4 - (uint)bVar1;
  }
  FUN_0011ed10(iVar5);
  FUN_00112010();
  FUN_001201d0();
  FUN_001257e0("startup packages %s",*DAT_00354a78);
  if (DAT_001513d0 == 0) {
    if (*param_1 == 0) {
      pcVar4 = "--%s needs at least one package name argument";
      uVar3 = *DAT_00354a78;
      goto LAB_0011483f;
    }
    FUN_001140d0(param_1);
  }
  else {
    if (*param_1 != 0) {
      pcVar4 = "--%s --pending does not take any non-option arguments";
      uVar3 = *DAT_00354a78;
LAB_0011483f:
      uVar2 = dcgettext(0,pcVar4,5);
                    /* WARNING: Subroutine does not return */
      FUN_00126990(uVar2,uVar3);
    }
    FUN_00113970();
  }
  FUN_001205a0();
  FUN_00114190();
  FUN_00118e90();
  FUN_0011f340();
  return 0;
}`,
      ida:
`long long packages(long long *a1)
{
  long long v2; // rdi
  int v3; // edx
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  const char *v8; // rsi
  long long v9; // rbp
  unsigned int v10; // eax
  int v11; // edx
  int v12; // ecx
  int v13; // r8d
  int v14; // r9d

  sub_18FA0();
  v2 = 0;
  if ( !dword_513C4 )
    v2 = 4 - (unsigned int)sub_118F0(0x10000);
  sub_1ED10(v2);
  sub_12010();
  sub_201D0();
  sub_257E0((unsigned int)"startup packages %s", *(long long *)qword_254A78, v3, v4, v5, v6);
  if ( !dword_513D0 )
  {
    if ( *a1 )
    {
      sub_140D0(a1);
      goto LABEL_8;
    }
    v8 = "--%s needs at least one package name argument";
    v9 = *(long long *)qword_254A78;
LABEL_10:
    v10 = (unsigned int)dcgettext(0, v8, 5);
    sub_26990(v10, v9, v11, v12, v13, v14);
  }
  if ( *a1 )
  {
    v8 = "--%s --pending does not take any non-option arguments";
    v9 = *(long long *)qword_254A78;
    goto LABEL_10;
  }
  sub_13970();
LABEL_8:
  sub_205A0();
  sub_14190();
  sub_18E90();
  sub_1F340();
  return 0;
}`,
      binja:
`int64_t packages(int64_t* arg1)

{
    sub_418fa0();
    int32_t rdi = 0;
    
    if (!data_4513c4)
        rdi = 4 - sub_4118f0(0x10000);
    
    sub_41ed10(rdi);
    sub_412010();
    sub_4201d0();
    **&data_654a78;
    sub_4257e0("startup packages %s", 0);
    char const* const msgid;
    
    if (!data_4513d0)
    {
        if (*arg1)
        {
            sub_4140d0(arg1);
        label_4147ef:
            sub_4205a0();
            sub_414190();
            sub_418e90();
            sub_41f340();
            return 0;
        }
        
        msgid = "--%s needs at least one package name argument";
        **&data_654a78;
    }
    else
    {
        if (!*arg1)
        {
            sub_413970();
            goto label_4147ef;
        }
        
        msgid = "--%s --pending does not take any non-option arguments";
        **&data_654a78;
    }
    
    sub_426990(dcgettext(nullptr, msgid, 5), 0);
    /* no return */
}`,
      angr:
`typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_4513c4;
extern unsigned int g_4513d0;
extern struct_0 *g_654a78;

unsigned int packages(unsigned long long *a0)
{
    long long v1;  // rdi
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    char *v8;  // rsi
    unsigned long v9;  // rbp

    sub_418fa0();
    v1 = 0;
    if (!g_4513c4)
        v1 = 4 - (sub_4118f0(0x10000) & 0xff);
    sub_41ed10(v1);
    sub_412010();
    sub_4201d0(v2, v3, v4, v5, v6, v7);
    sub_4257e0("startup packages %s", g_654a78->field_0);
    if (g_4513d0)
    {
        if (*(a0))
            goto LABEL_414853;
        sub_413970();
    }
    else if (*(a0))
    {
        sub_4140d0(a0);
    }
    else
    {
        v8 = "--%s needs at least one package name argument";
        v9 = g_654a78->field_0;
        while (true)
        {
LABEL_414853:
            v8 = "--%s --pending does not take any non-option arguments";
            v9 = g_654a78->field_0;
            sub_426990(dcgettext(NULL, v8, 5), v9);
        }
    }
    sub_4205a0(v2, v3, v4, v5, v6, v7);
    sub_414190();
    sub_418e90(v2, v3, v4, v5, v6, v7);
    sub_41f340(v2, v3, v4, v5, v6, v7);
    return 0;
}`,
    },
  },

  {
    id: 'O2-noinline-zlib-libz.so.1.2-gz_look',
    name: "gz_look() — zlib libz.so.1.2, x86-64",
    meta: "zlib libz.so.1.2 · ELF x86-64 · gcc -O2 -fno-inline, stripped · 0xec60",
    ged: { kuna: 3, ida: 8, ghidra: 8, binja: 13, angr: 4 },
    kuna:
`/* WARNING: earlyreturn: hoisted 4 const-guard early-return(s) in gz_look */
unsigned long gz_look(void *a0)

{
  int4 v1;
  unsigned int v2;
  uint4 v3;
  void *v4; // rax
  unsigned long v5; // rax
  uint8 v6;
  
  if (a0[10] == 0) {
    v3 = a0[0xb];
    *(void **)&a0[0xc] = malloc((uint8)v3);
    v4 = malloc((uint8)(uint4)(a0[0xb] * 2));
    *(void **)&a0[0xe] = v4;
    if ((*(int8 *)&a0[0xc] == 0) || (v4 == (void *)0x0)) {
      free(v4);
      free(*(void **)&a0[0xc]);
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
      sub_e420(a0,0xfffffffc,"out of memory");
      return 0xffffffff;
    }
    *(void *)&a0[0x30] = 0;
    a0[10] = a0[0xb];
    *(void *)&a0[0x32] = 0;
    *(void *)&a0[0x34] = 0;
    a0[0x22] = 0;
    *(void *)&a0[0x20] = 0;
    if (inflateInit2_(&a0[0x20],0x1f,"1.2.13",0x70) != 0) {
      free(*(void **)&a0[0xe]);
      free(*(void **)&a0[0xc]);
      a0[10] = 0;
      sub_e420(a0,0xfffffffc,"out of memory");
      return 0xffffffff;
    }
  }
  v6 = (uint8)(uint4)a0[0x22];
  if ((uint4)a0[0x22] < 2) {
    v5 = sub_eb90(a0);
    if ((int4)v5 == -1) {
      return v5;
    }
    v3 = a0[0x22];
    v6 = (uint8)v3;
    if (v3 == 0) {
      return 0;
    }
    if (v3 == 1) {
      v1 = a0[0x10];
      goto label_ee07;
    }
  }
  if ((**(char **)&a0[0x20] == '\\x1f') && ((*(char **)&a0[0x20])[1] == '\\x8b')) {
    inflateReset(&a0[0x20]);
    *(void *)&a0[0x10] = 0x200000000;
    return 0;
  }
  v1 = a0[0x10];
label_ee07:
  if (v1 == 0) {
    a0[0x22] = 0;
    a0[0x14] = 1;
    *a0 = 0;
    return 0;
  }
  *(void **)&a0[2] = *(void **)&a0[0xe];
  memcpy(*(void **)&a0[0xe],*(void **)&a0[0x20],v6);
  v2 = a0[0x22];
  a0[0x22] = 0;
  *a0 = v2;
  *(void *)&a0[0x10] = 0x100000001;
  return 0;
}`,
    vs: {
      source:
`local int gz_look(state)
    gz_statep state;
{
    z_streamp strm = &(state->strm);

    /* allocate read buffers and inflate memory */
    if (state->size == 0) {
        /* allocate buffers */
        state->in = (unsigned char *)malloc(state->want);
        state->out = (unsigned char *)malloc(state->want << 1);
        if (state->in == NULL || state->out == NULL) {
            free(state->out);
            free(state->in);
            gz_error(state, Z_MEM_ERROR, "out of memory");
            return -1;
        }
        state->size = state->want;

        /* allocate inflate memory */
        state->strm.zalloc = Z_NULL;
        state->strm.zfree = Z_NULL;
        state->strm.opaque = Z_NULL;
        state->strm.avail_in = 0;
        state->strm.next_in = Z_NULL;
        if (inflateInit2(&(state->strm), 15 + 16) != Z_OK) {    /* gunzip */
            free(state->out);
            free(state->in);
            state->size = 0;
            gz_error(state, Z_MEM_ERROR, "out of memory");
            return -1;
        }
    }

    /* get at least the magic bytes in the input buffer */
    if (strm->avail_in < 2) {
        if (gz_avail(state) == -1)
            return -1;
        if (strm->avail_in == 0)
            return 0;
    }

    /* look for gzip magic bytes -- if there, do gzip decoding (note: there is
       a logical dilemma here when considering the case of a partially written
       gzip file, to wit, if a single 31 byte is written, then we cannot tell
       whether this is a single-byte file, or just a partially written gzip
       file -- for here we assume that if a gzip file is being written, then
       the header will be written in a single operation, so that reading a
       single byte is sufficient indication that it is not a gzip file) */
    if (strm->avail_in > 1 &&
            strm->next_in[0] == 31 && strm->next_in[1] == 139) {
        inflateReset(strm);
        state->how = GZIP;
        state->direct = 0;
        return 0;
    }

    /* no gzip header -- if we were decoding gzip before, then this is trailing
       garbage.  Ignore the trailing garbage and finish. */
    if (state->direct == 0) {
        strm->avail_in = 0;
        state->eof = 1;
        state->x.have = 0;
        return 0;
    }

    /* doing raw i/o, copy any leftover input to output -- this assumes that
       the output buffer is larger than the input buffer, which also assures
       space for gzungetc() */
    state->x.next = state->out;
    memcpy(state->x.next, strm->next_in, strm->avail_in);
    state->x.have = strm->avail_in;
    strm->avail_in = 0;
    state->how = COPY;
    state->direct = 1;
    return 0;
}`,
      ghidra:
`undefined8 gz_look(undefined4 *param_1)

{
  undefined4 uVar1;
  uint uVar2;
  int iVar3;
  void *pvVar4;
  undefined8 uVar5;
  size_t __n;
  
  if (param_1[10] == 0) {
    pvVar4 = malloc((ulong)(uint)param_1[0xb]);
    *(void **)(param_1 + 0xc) = pvVar4;
    pvVar4 = malloc((ulong)(uint)(param_1[0xb] * 2));
    *(void **)(param_1 + 0xe) = pvVar4;
    if ((*(long *)(param_1 + 0xc) == 0) || (pvVar4 == (void *)0x0)) {
      free(pvVar4);
      free(*(void **)(param_1 + 0xc));
LAB_0010edb5:
      FUN_0010e420(param_1,0xfffffffc,"out of memory");
      return 0xffffffff;
    }
    *(undefined8 *)(param_1 + 0x30) = 0;
    param_1[10] = param_1[0xb];
    *(undefined8 *)(param_1 + 0x32) = 0;
    *(undefined8 *)(param_1 + 0x34) = 0;
    param_1[0x22] = 0;
    *(undefined8 *)(param_1 + 0x20) = 0;
    iVar3 = inflateInit2_(param_1 + 0x20,0x1f,"1.2.13",0x70);
    if (iVar3 != 0) {
      free(*(void **)(param_1 + 0xe));
      free(*(void **)(param_1 + 0xc));
      param_1[10] = 0;
      goto LAB_0010edb5;
    }
  }
  __n = (size_t)(uint)param_1[0x22];
  if ((uint)param_1[0x22] < 2) {
    uVar5 = FUN_0010eb90(param_1);
    if ((int)uVar5 == -1) {
      return uVar5;
    }
    uVar2 = param_1[0x22];
    __n = (size_t)uVar2;
    if (uVar2 == 0) {
      return 0;
    }
    if (uVar2 == 1) {
      iVar3 = param_1[0x10];
      goto joined_r0x0010ee07;
    }
  }
  if ((**(char **)(param_1 + 0x20) == '\\x1f') && ((*(char **)(param_1 + 0x20))[1] == -0x75)) {
    inflateReset(param_1 + 0x20);
    *(undefined8 *)(param_1 + 0x10) = 0x200000000;
    return 0;
  }
  iVar3 = param_1[0x10];
joined_r0x0010ee07:
  if (iVar3 == 0) {
    param_1[0x22] = 0;
    param_1[0x14] = 1;
    *param_1 = 0;
    return 0;
  }
  *(void **)(param_1 + 2) = *(void **)(param_1 + 0xe);
  memcpy(*(void **)(param_1 + 0xe),*(void **)(param_1 + 0x20),__n);
  uVar1 = param_1[0x22];
  param_1[0x22] = 0;
  *param_1 = uVar1;
  *(undefined8 *)(param_1 + 0x10) = 0x100000001;
  return 0;
}`,
      ida:
`long long gz_look(long long a1)
{
  long long *v1; // r12
  size_t v3; // rdx
  char *v4; // rax
  long long result; // rax
  void *v6; // rdi
  const void *v7; // rsi
  int v8; // eax
  void *v9; // rax
  bool v10; // zf
  int v11; // eax

  v1 = (long long *)(a1 + 128);
  if ( !*(int *)(a1 + 40) )
  {
    *(long long *)(a1 + 48) = malloc(*(unsigned int *)(a1 + 44));
    v9 = malloc((unsigned int)(2 * *(int *)(a1 + 44)));
    v10 = *(long long *)(a1 + 48) == 0;
    *(long long *)(a1 + 56) = v9;
    if ( v10 || !v9 )
    {
      free(v9);
      free(*(void **)(a1 + 48));
    }
    else
    {
      v11 = *(int *)(a1 + 44);
      *(long long *)(a1 + 192) = 0;
      *(int *)(a1 + 40) = v11;
      *(long long *)(a1 + 200) = 0;
      *(long long *)(a1 + 208) = 0;
      *(int *)(a1 + 136) = 0;
      *(long long *)(a1 + 128) = 0;
      if ( !(unsigned int)inflateInit2_(v1, 31, "1.2.13", 112) )
        goto LABEL_2;
      free(*(void **)(a1 + 56));
      free(*(void **)(a1 + 48));
      *(int *)(a1 + 40) = 0;
    }
    sub_E420(a1, -4, "out of memory");
    return 0xFFFFFFFFLL;
  }
LABEL_2:
  v3 = *(unsigned int *)(a1 + 136);
  if ( (unsigned int)v3 > 1 )
    goto LABEL_3;
  result = sub_EB90(a1);
  if ( (int)result != -1 )
  {
    v3 = *(unsigned int *)(a1 + 136);
    result = 0;
    if ( (int)v3 )
    {
      if ( (int)v3 == 1 )
      {
        result = *(unsigned int *)(a1 + 64);
        if ( !(int)result )
        {
LABEL_5:
          *(int *)(a1 + 136) = 0;
          *(int *)(a1 + 80) = 1;
          *(int *)a1 = 0;
          return result;
        }
LABEL_7:
        v6 = *(void **)(a1 + 56);
        v7 = *(const void **)(a1 + 128);
        *(long long *)(a1 + 8) = v6;
        memcpy(v6, v7, v3);
        v8 = *(int *)(a1 + 136);
        *(int *)(a1 + 136) = 0;
        *(int *)a1 = v8;
        *(long long *)(a1 + 64) = 0x100000001LL;
        return 0;
      }
LABEL_3:
      v4 = *(char **)(a1 + 128);
      if ( *v4 == 31 && v4[1] == 0x8B )
      {
        inflateReset(v1);
        *(long long *)(a1 + 64) = 0x200000000LL;
        return 0;
      }
      result = *(unsigned int *)(a1 + 64);
      if ( !(int)result )
        goto LABEL_5;
      goto LABEL_7;
    }
  }
  return result;
}`,
      binja:
`int64_t gz_look(int32_t* arg1)

{
    if (!arg1[0xa])
    {
        *(arg1 + 0x30) = malloc(arg1[0xb]);
        int64_t rax_6 = malloc(arg1[0xb] * 2);
        bool cond:0_1 = !*(arg1 + 0x30);
        *(arg1 + 0x38) = rax_6;
        
        if (cond:0_1 || !rax_6)
        {
            free(rax_6);
            free(*(arg1 + 0x30));
            sub_40e420(arg1, 0xfffffffc, "out of memory");
            return -1;
        }
        
        int32_t rax_7 = arg1[0xb];
        *(arg1 + 0xc0) = 0;
        arg1[0xa] = rax_7;
        *(arg1 + 0xc8) = 0;
        *(arg1 + 0xd0) = 0;
        arg1[0x22] = 0;
        *(arg1 + 0x80) = 0;
        
        if (inflateInit2_(&arg1[0x20], 0x1f, "1.2.13", 0x70))
        {
            free(*(arg1 + 0x38));
            free(*(arg1 + 0x30));
            arg1[0xa] = 0;
            sub_40e420(arg1, 0xfffffffc, "out of memory");
            return -1;
        }
    }
    
    uint64_t rdx_1 = arg1[0x22];
    int32_t result;
    
    if (rdx_1 > 1)
    {
    label_40ec8b:
        char* rax_1 = *(arg1 + 0x80);
        
        if (*rax_1 == 0x1f && rax_1[1] == 0x8b)
        {
            inflateReset(&arg1[0x20]);
            *(arg1 + 0x40) = 0x200000000;
            return 0;
        }
        
        result = arg1[0x10];
        
        if (result)
            goto label_40ecc8;
        
        goto label_40eca2;
    }
    
    result = sub_40eb90(arg1);
    
    if (result != 0xffffffff)
    {
        rdx_1 = arg1[0x22];
        result = 0;
        
        if (rdx_1)
        {
            if (rdx_1 != 1)
                goto label_40ec8b;
            
            result = arg1[0x10];
            
            if (result)
            {
            label_40ecc8:
                int64_t rdi = *(arg1 + 0x38);
                int64_t rsi = *(arg1 + 0x80);
                *(arg1 + 8) = rdi;
                memcpy(rdi, rsi, rdx_1);
                int32_t rax_2 = arg1[0x22];
                arg1[0x22] = 0;
                *arg1 = rax_2;
                *(arg1 + 0x40) = 0x100000001;
                return 0;
            }
            
        label_40eca2:
            arg1[0x22] = 0;
            arg1[0x14] = 1;
            *arg1 = 0;
        }
    }
    
    return result;
}`,
      angr:
`typedef struct struct_0 {
    char padding_0[28];
    int field_1c;
    char padding_20[48];
    unsigned int field_50;
} struct_0;

unsigned long long gz_look(struct_0 *idx)
{
    struct_0 *v1;  // r12
    void* ptr;  // rax
    unsigned long v3;  // 4098
    unsigned int v4;  // eax
    unsigned long n;  // rdx
    unsigned long long v6;  // rax
    unsigned long long v7;  // rax
    void* v8;  // rdi
    char v9[2];  // rsi
    unsigned int v10;  // eax

    v1 = &idx[1].padding_20[12];
    if (*((int *)&idx->padding_20[8]))
    {
LABEL_40ec7c:
        n = *((int *)&idx[1].padding_20[20]);
        if ((unsigned int)n > 1)
        {
LABEL_40ec8b:
            if (*((char *)*((long long *)&idx[1].padding_20[12])) == 31 && *((char *)(*((long long *)&idx[1].padding_20[12]) + 1)) == 139)
            {
                ::0x4035e0::inflateReset(v1);
                *((unsigned long *)&idx->padding_20[32]) = 0x200000000;
                return 0;
            }
            v7 = *((int *)&idx->padding_20[32]);
            if ((unsigned int)v7)
            {
                v8 = *((long long *)&idx->padding_20[24]);
                v9 = *((long long *)&idx[1].padding_20[12]);
                *((void* *)&idx->padding_0[8]) = v8;
                memcpy(v8, v9, n);
                v10 = *((int *)&idx[1].padding_20[20]);
                *((unsigned int *)&idx[1].padding_20[20]) = 0;
                *((unsigned int *)&idx->padding_0[0]) = v10;
                *((unsigned long *)&idx->padding_20[32]) = 4294967297;
                return 0;
            }
        }
        else
        {
            v6 = sub_40eb90(idx);
            if ((unsigned int)v6 == 4294967295)
                return v6;
            n = *((int *)&idx[1].padding_20[20]);
            if (!(unsigned int)n)
                return 0;
            if ((unsigned int)n != 1)
                goto LABEL_40ec8b;
            v7 = *((int *)&idx->padding_20[32]);
            if ((unsigned int)v7)
            {
                v8 = *((long long *)&idx->padding_20[24]);
                v9 = *((long long *)&idx[1].padding_20[12]);
                *((void* *)&idx->padding_0[8]) = v8;
                memcpy(v8, v9, n);
                v10 = *((int *)&idx[1].padding_20[20]);
                *((unsigned int *)&idx[1].padding_20[20]) = 0;
                *((unsigned int *)&idx->padding_0[0]) = v10;
                *((unsigned long *)&idx->padding_20[32]) = 4294967297;
                return 0;
            }
        }
        *((unsigned int *)&idx[1].padding_20[20]) = 0;
        idx->field_50 = 1;
        *((unsigned int *)&idx->padding_0[0]) = 0;
        return v7;
    }
    else
    {
        *((void* *)&idx->padding_20[16]) = malloc(*((int *)&idx->padding_20[12]));
        ptr = malloc(*((int *)&idx->padding_20[12]) * 2);
        v3 = *((long long *)&idx->padding_20[16]);
        *((void* *)&idx->padding_20[24]) = ptr;
        if (v3 && ptr)
        {
            v4 = *((int *)&idx->padding_20[12]);
            *((unsigned long *)&idx[2].padding_0[24]) = 0;
            *((unsigned int *)&idx->padding_20[8]) = v4;
            *((unsigned long *)&idx[2].padding_20[0]) = 0;
            *((unsigned long *)&idx[2].padding_20[8]) = 0;
            *((unsigned int *)&idx[1].padding_20[20]) = 0;
            *((char *[2])&idx[1].padding_20[12]) = NULL;
            if (!::0x4035d0::inflateInit2_(v1, 31, "1.2.13", 112))
                goto LABEL_40ec7c;
            free(*((long long *)&idx->padding_20[24]));
            free(*((long long *)&idx->padding_20[16]));
            *((unsigned int *)&idx->padding_20[8]) = 0;
        }
        else
        {
            free(ptr);
            free(*((long long *)&idx->padding_20[16]));
        }
        sub_40e420(idx, 0xfffffffc, "out of memory");
        return 4294967295;
    }
}`,
    },
  },

  {
    id: 'O2-noinline-openssh-portable-sshd-should_drop_connection',
    name: "should_drop_connection() — openssh-portable sshd, x86-64",
    meta: "openssh-portable sshd · ELF x86-64 · gcc -O2 -fno-inline, stripped · 0x102d0",
    ged: { kuna: 0, ida: 12, ghidra: 11, binja: 12, angr: 11 },
    kuna:
`/* WARNING: earlyreturn: hoisted 3 const-guard early-return(s) in should_drop_connection */
bool should_drop_connection(int4 a0)

{
  int8 v1;
  int4 v2;
  unsigned int v3; // eax
  int4 v4; // eax
  int4 v5;
  int4 v6;
  
  v2 = dat_125298;
  v6 = dat_125294;
  v5 = dat_125290;
  if (a0 < dat_125290) {
    return 0;
  }
  if (dat_125298 <= a0) {
    return 1;
  }
  if (dat_125294 == 100) {
    return 1;
  }
  v3 = sub_cbf90(100,dat_125294);
  v4 = sub_cbfd0(sub_cbf90(a0,v5),v3);
  v1 = (int8)(int4)sub_cbf90(v2,v5);
  v5 = sub_cbf50(v6,(int8)v4 / v1 & 0xffffffff,(int8)v4 % v1 & 0xffffffff);
  v6 = sub_c2f00(100);
  sub_7ed20("sshd.c","should_drop_connection",0x344,1,5,0);
  return v6 < v5;
}`,
    vs: {
      source:
`static int
should_drop_connection(int startups)
{
	int p, r;

	if (startups < options.max_startups_begin)
		return 0;
	if (startups >= options.max_startups)
		return 1;
	if (options.max_startups_rate == 100)
		return 1;

	p  = 100 - options.max_startups_rate;
	p *= startups - options.max_startups_begin;
	p /= options.max_startups - options.max_startups_begin;
	p += options.max_startups_rate;
	r = arc4random_uniform(100);

	debug_f("p %d, r %d", p, r);
	return (r < p) ? 1 : 0;
}`,
      ghidra:
`bool should_drop_connection(int param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  undefined8 uVar8;
  
  iVar2 = DAT_00225298;
  iVar1 = DAT_00225294;
  iVar7 = DAT_00225290;
  bVar3 = false;
  if (DAT_00225290 <= param_1) {
    bVar3 = true;
    if ((param_1 < DAT_00225298) && (DAT_00225294 != 100)) {
      uVar4 = FUN_001cbf90(100,DAT_00225294);
      uVar5 = FUN_001cbf90(param_1,iVar7);
      iVar6 = FUN_001cbfd0(uVar5,uVar4);
      iVar7 = FUN_001cbf90(iVar2,iVar7);
      iVar7 = FUN_001cbf50(iVar1,(long)iVar6 / (long)iVar7 & 0xffffffff,
                           (long)iVar6 % (long)iVar7 & 0xffffffff);
      uVar8 = FUN_001c2f00(100);
      FUN_0017ed20("sshd.c","should_drop_connection",0x344,1,5,0,"p %d, r %d",iVar7,uVar8);
      bVar3 = (int)uVar8 < iVar7;
    }
  }
  return bVar3;
}`,
      ida:
`double should_drop_connection(int a1)
{
  unsigned int v1; // r12d
  unsigned int v2; // r13d
  unsigned int v3; // r14d
  unsigned int v4; // r15d
  unsigned int v5; // eax
  int v6; // ebp
  int v7; // eax
  int v8; // r12d
  int v9; // eax

  v1 = dword_125290;
  if ( dword_125290 <= a1 )
  {
    v2 = dword_125298;
    if ( dword_125298 > a1 )
    {
      v3 = dword_125294;
      if ( dword_125294 != 100 )
      {
        v4 = sub_CBF90(100, (unsigned int)dword_125294);
        v5 = sub_CBF90((unsigned int)a1, v1);
        v6 = sub_CBFD0(v5, v4);
        v7 = sub_CBF90(v2, v1);
        v8 = sub_CBF50(v3, (unsigned int)(v6 / v7));
        v9 = sub_C2F00(100);
        sub_7ED20((unsigned int)"sshd.c", (unsigned int)"should_drop_connection", 836, 1, 5, 0, "p %d, r %d", v8, v9);
      }
    }
  }
  return 0.0;
}`,
      binja:
`int512_t should_drop_connection(int32_t arg1)

{
    int32_t r12 = data_525290;
    int512_t result;
    int512_t zmm1;
    int512_t zmm2;
    int512_t zmm3;
    int512_t zmm4;
    int512_t zmm5;
    int512_t zmm6;
    int512_t zmm7;
    int512_t zmm8;
    int512_t zmm9;
    int512_t zmm10;
    int512_t zmm11;
    int512_t zmm12;
    int512_t zmm13;
    int512_t zmm14;
    int512_t zmm15;
    
    if (r12 <= arg1)
    {
        int32_t r13_1 = data_525298;
        
        if (r13_1 > arg1)
        {
            int32_t r14_1 = data_525294;
            
            if (r14_1 != 0x64)
            {
                int32_t rax_2 = __subvsi3(0x64, r14_1);
                uint64_t r12_1 = __addvsi3(r14_1, 
                    __mulvsi3(__subvsi3(arg1, r12), rax_2) / __subvsi3(r13_1, r12));
                int64_t rax_10 = sub_4c2f00(0x64);
                int64_t var_38_1 = rax_10;
                uint64_t var_40_1 = r12_1;
                result =
                    sub_47ed20("sshd.c", "should_drop_connection", 0x344, 1, 5, 0, 0, "p %d, r %d");
                int64_t rax_11;
                rax_11 = r12_1 > rax_10;
            }
        }
    }
    
    result = {0};
    zmm1 = {0};
    zmm2 = {0};
    zmm3 = {0};
    zmm4 = {0};
    zmm5 = {0};
    zmm6 = {0};
    zmm7 = {0};
    zmm8 = {0};
    zmm9 = {0};
    zmm10 = {0};
    zmm11 = {0};
    zmm12 = {0};
    zmm13 = {0};
    zmm14 = {0};
    zmm15 = {0};
    return result;
}`,
      angr:
`extern int g_525290;
extern unsigned int g_525294;
extern char g_525298;

unsigned int should_drop_connection(int a0)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    unsigned int v4;  // eax
    unsigned long long v5;  // r12
    long long v6;  // rax

    v1 = 0;
    if (g_525290 <= a0)
    {
        v1 = 1;
        if (*((int *)&g_525298) > a0 && g_525294 != 100)
        {
            v2 = sub_4cbf90(100, g_525294);
            v3 = sub_4cbfd0(sub_4cbf90(a0, g_525290) & 4294967295, v2);
            v4 = sub_4cbf90(*((int *)&g_525298), g_525290);
            v5 = sub_4cbf50(g_525294, v3 / v4, v3 % v4 & 4294967295) & 4294967295;
            v6 = sub_4c2f00(100);
            sub_47ed20("sshd.c", "should_drop_connection", 836, 1, 5, 0, "p %d, r %d", v5, v6);
            v1 = (int)v6 < (unsigned int)v5;
        }
    }
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    /* unsupported instruction */
    return v1;
}`,
    },
  },

  {
    id: 'O2-libacl-setfacl-seq_get_cmd',
    name: "seq_get_cmd() — libacl setfacl, x86-64",
    meta: "libacl setfacl · ELF x86-64 · gcc -O2, stripped · 0x5340",
    ged: { kuna: 2, ida: 11, ghidra: 7, binja: 6, angr: 3 },
    kuna:
`/* WARNING: earlyreturn: hoisted 3 const-guard early-return(s) in seq_get_cmd */
uint8 seq_get_cmd(int8 *a0,int4 a1,int8 *a2)

{
  int8 v1;
  
  if (a1 != 0) {
    if ((a2 != (int8 *)0x0) && (a1 == 1)) {
      if (*a2 == 0) {
        return 0;
      }
      v1 = *(int8 *)(*a2 + 0x18);
      *a2 = v1;
      return (uint8)(v1 != 0);
    }
    return 0xffffffff;
  }
  if (*a0 == 0) {
    return 0;
  }
  if (a2 == (int8 *)0x0) {
    return 1;
  }
  *a2 = *a0;
  return 1;
}`,
    vs: {
      source:
`int
seq_get_cmd(
	seq_t seq,
	int which,
	cmd_t *cmd)
{
	if (which == SEQ_FIRST_CMD) {
		if (seq->s_first == NULL)
			return 0;
		if (cmd)
			*cmd = seq->s_first;
		return 1;
	} else if (which == SEQ_NEXT_CMD) {
		if (cmd == NULL)
			return -1;
		if (*cmd) {
			*cmd = (*cmd)->c_next;
			return (*cmd == NULL) ? 0 : 1;
		}
		return 0;
	} else {
		return -1;
	}
}`,
      ghidra:
`ulong seq_get_cmd(long *param_1,int param_2,long *param_3)

{
  long lVar1;
  ulong uVar2;
  
  if (param_2 == 0) {
    uVar2 = 0;
    if ((*param_1 != 0) && (uVar2 = 1, param_3 != (long *)0x0)) {
      *param_3 = *param_1;
      return uVar2;
    }
  }
  else if ((param_3 == (long *)0x0) || (param_2 != 1)) {
    uVar2 = 0xffffffff;
  }
  else {
    uVar2 = 0;
    if (*param_3 != 0) {
      lVar1 = *(long *)(*param_3 + 0x18);
      *param_3 = lVar1;
      return (ulong)(lVar1 != 0);
    }
  }
  return uVar2;
}`,
      ida:
`long long seq_get_cmd(long long *a1, int a2, long long a3)
{
  long long result; // rax
  long long v4; // rax

  if ( a2 )
  {
    if ( a3 && a2 == 1 )
    {
      result = 0;
      if ( *(long long *)a3 )
      {
        v4 = *(long long *)(*(long long *)a3 + 24LL);
        *(long long *)a3 = v4;
        return v4 != 0;
      }
    }
    else
    {
      return 0xFFFFFFFFLL;
    }
  }
  else
  {
    result = 0;
    if ( *a1 )
    {
      result = 1;
      if ( a3 )
        *(long long *)a3 = *a1;
    }
  }
  return result;
}`,
      binja:
`uint64_t seq_get_cmd(int64_t* arg1, int32_t arg2, int64_t* arg3)

{
    int64_t result;
    
    if (arg2)
    {
        if (!arg3 || arg2 != 1)
            return 0xffffffff;
        
        void* rcx_2 = *arg3;
        result = 0;
        
        if (rcx_2)
        {
            int64_t rax = *(rcx_2 + 0x18);
            *arg3 = rax;
            rax = rax;
            return rax;
        }
    }
    else
    {
        int64_t rcx_1 = *arg1;
        result = 0;
        
        if (rcx_1)
        {
            result = 1;
            
            if (arg3)
            {
                *arg3 = rcx_1;
                return 1;
            }
        }
    }
    
    return result;
}`,
      angr:
`typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

unsigned int seq_get_cmd(struct_0 **a0, unsigned int a1, struct_0 **a2)
{
    unsigned long long v1;  // rax

    if (a1)
    {
        if (!a2 || a1 != 1)
            return 4294967295;
        if (!*(a2))
            return 0;
        v1 = *(a2)->field_18;
        *(a2) = v1;
        return v1;
        return 4294967295;
    }
    else if (*(a0))
    {
        if (!a2)
            return 1;
        *(a2) = *(a0);
        return 1;
    }
    else
    {
        return 0;
    }
}`,
    },
  },

  {
    id: 'O2-cronie-crontab-strcmp_until',
    name: "strcmp_until() — cronie crontab, x86-64",
    meta: "cronie crontab · ELF x86-64 · gcc -O2, stripped · 0x80f0",
    ged: { kuna: 18, ida: 26, ghidra: 19, binja: 26, angr: 28 },
    kuna:
`int4 strcmp_until(char *a0,char *a1,char a2)

{
  char *v1;
  char v2;
  char v3;
  
  v2 = *a0;
  while ((v2 != '\\0' && (v3 = *a1, v2 != a2))) {
    if (v2 != v3) {
      return (int4)v2 - (int4)v3;
    }
    v1 = &a0[1];
    a0 = &a0[1];
    a1 = &a1[1];
    v2 = *v1;
  }
  v3 = *a1;
  if ((v3 == a2) || (v3 == '\\0')) {
    return 0;
  }
  return (int4)v2 - (int4)v3;
}`,
    vs: {
      source:
`int strcmp_until(const char *left, const char *right, char until) {
	while (*left && *left != until && *left == *right) {
		left++;
		right++;
	}

	if ((*left == '\\0' || *left == until) && (*right == '\\0' ||
			*right == until)) {
		return (0);
	}
	return (*left - *right);
}`,
      ghidra:
`int strcmp_until(char *param_1,char *param_2,char param_3)

{
  char *pcVar1;
  char cVar2;
  char cVar3;
  
  cVar2 = *param_1;
  while ((cVar2 != '\\0' && (cVar3 = *param_2, cVar2 != param_3))) {
    if (cVar2 != cVar3) goto LAB_00108127;
    pcVar1 = param_1 + 1;
    param_1 = param_1 + 1;
    param_2 = param_2 + 1;
    cVar2 = *pcVar1;
  }
  cVar3 = *param_2;
  if ((cVar3 == param_3) || (cVar3 == '\\0')) {
    return 0;
  }
LAB_00108127:
  return (int)cVar2 - (int)cVar3;
}`,
      ida:
`long long strcmp_until(char *a1, char *a2, char a3)
{
  int v3; // eax
  int v4; // ecx

  v3 = (char)*a1;
  if ( *a1 )
  {
    do
    {
      v4 = (char)*a2;
      if ( (char)v3 == a3 )
        break;
      if ( (char)v3 != (char)v4 )
        return (unsigned int)(v3 - v4);
      v3 = (char)*++a1;
      ++a2;
    }
    while ( (char)v3 );
  }
  v4 = (char)*a2;
  if ( *a2 == a3 || !(char)v4 )
    return 0;
  else
    return (unsigned int)(v3 - v4);
}`,
      binja:
`uint64_t strcmp_until(char* arg1, char* arg2, char arg3)

{
    int32_t rax = *arg1;
    int32_t rcx_1;
    
    if (rax)
    {
        do
        {
            rcx_1 = *arg2;
            
            if (rax == arg3)
                break;
            
            if (rax != rcx_1)
                return rax - rcx_1;
            
            rax = arg1[1];
            arg1 = &arg1[1];
            arg2 = &arg2[1];
        } while (rax);
    }
    
    rcx_1 = *arg2;
    
    if (rcx_1 == arg3 || !rcx_1)
        return 0;
    
    return rax - rcx_1;
}`,
      angr:
`unsigned int strcmp_until(char *node, char *iter, char a2)
{
    unsigned int v1;  // eax
    unsigned int v2;  // ecx

    v1 = *(node);
    if ((char)v1)
    {
        do
        {
            v2 = *(iter);
            if ((char)v1 == a2)
                break;
            if ((char)v1 != (char)v2)
                return v1 - v2;
            v1 = node[1];
            node += 1;
            iter += 1;
        } while ((char)v1);
    }
    v2 = *(iter);
    if ((char)v2 == a2)
    {
        return 0;
    }
    else if ((char)v2)
    {
        return v1 - v2;
    }
    else
    {
        return 0;
    }
}`,
    },
  },

  {
    id: 'O2-noinline-tar-tar-sys_exec_command',
    name: "sys_exec_command() — tar, x86-64",
    meta: "tar · ELF x86-64 · gcc -O2 -fno-inline, stripped · 0x2b640",
    ged: { kuna: 0, ida: 6, ghidra: 8, binja: 8, angr: 6 },
    kuna:
`unsigned int sys_exec_command(unsigned long a0,char a1,unsigned long a2)

{
  unsigned int v1; // stack - 0x28
  int8 v2; // fs_offset
  unsigned int v3; // stack - 0x24
  unsigned long v4; // stack - 0x20
  
  v4 = *(void *)(v2 + 0x28);
  sub_24fc0(&v1);
  dat_84728 = signal(0xd,1);
  dat_84730 = sub_24f80();
  if (dat_84730 != 0) {
    sub_d2c0(v1);
    return v3;
  }
  sub_2a400(v1,0);
  sub_d2c0(v3);
  sub_2a8f0(a0,(int4)a1,a2);
                    /* WARNING: Subroutine does not return */
  sub_2a3b0(dat_84a60);
}`,
    vs: {
      source:
`int
sys_exec_command (char *file_name, int typechar, struct tar_stat_info *st)
{
  int p[2];

  xpipe (p);
  pipe_handler = signal (SIGPIPE, SIG_IGN);
  global_pid = xfork ();

  if (global_pid != 0)
    {
      xclose (p[PREAD]);
      return p[PWRITE];
    }

  /* Child */
  xdup2 (p[PREAD], STDIN_FILENO);
  xclose (p[PWRITE]);

  stat_to_env (file_name, typechar, st);

  priv_set_restore_linkdir ();
  xexec (to_command_option);
}`,
      ghidra:
`undefined4 sys_exec_command(undefined8 param_1,char param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined4 local_28;
  undefined4 local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00124fc0(&local_28);
  DAT_00184728 = signal(0xd,(__sighandler_t)0x1);
  DAT_00184730 = FUN_00124f80();
  if (DAT_00184730 == 0) {
    FUN_0012a400(local_28,0);
    FUN_0010d2c0(local_24);
    FUN_0012a8f0(param_1,(int)param_2,param_3);
    FUN_0012a3b0(DAT_00184a60);
  }
  else {
    FUN_0010d2c0(local_28);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_24;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}`,
      ida:
`long long sys_exec_command(const char *a1, char a2, long long a3)
{
  int fd; // [rsp+0h] [rbp-28h] BYREF
  unsigned int v6; // [rsp+4h] [rbp-24h]
  unsigned long long v7; // [rsp+8h] [rbp-20h]

  v7 = __readfsqword(0x28u);
  sub_24FC0(&fd);
  handler = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  pid = sub_24F80();
  if ( !pid )
  {
    sub_2A400(fd, 0);
    sub_D2C0(v6);
    sub_2A8F0(a1, a2, a3);
    sub_2A3B0((char *)qword_84A60);
  }
  sub_D2C0(fd);
  return v6;
}`,
      binja:
`uint64_t sys_exec_command(char* arg1, char arg2, void* arg3)

{
    void* fsbase;
    int64_t rax = *(fsbase + 0x28);
    sub_424fc0();
    data_484728 = signal(0xd, 1);
    pid_t rax_3 = sub_424f80();
    data_484730 = rax_3;
    int32_t var_28;
    int32_t var_24;
    
    if (!rax_3)
    {
        sub_42a400(var_28, 0);
        sub_40d2c0(var_24);
        sub_42a8f0(arg1, arg2, arg3);
        sub_42a3b0(data_484a60);
        /* no return */
    }
    
    sub_40d2c0(var_28);
    *(fsbase + 0x28);
    
    if (rax == *(fsbase + 0x28))
        return var_24;
    
    __stack_chk_fail();
    /* no return */
}`,
      angr:
`typedef struct struct_1 {
    char padding_0[8];
    char *field_8;
    char padding_10[8];
    char *field_18;
    char *field_20;
    char *field_28;
    char padding_30[64];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[4];
    long long field_80;
    long long field_88;
    char padding_90[88];
    long long field_e8;
    unsigned long long field_f0;
    long long field_f8;
    unsigned long long field_100;
    long long field_108;
    unsigned long long field_110;
} struct_1;

extern unsigned long long g_484728;
extern unsigned int g_484730;
extern long long g_484a60;

long long sys_exec_command(char *a0, char a1, struct_1 *a2)
{
    char v0;  // [bp-0x28]
    int v1;  // [bp-0x24]

    sub_424fc0(&v0);
    g_484728 = signal(13, 0x1);
    g_484730 = sub_424f80();
    if (!g_484730)
    {
        sub_42a400(v0, 0);
        sub_40d2c0(v1);
        sub_42a8f0(a0, a1, a2);
        sub_42a3b0(g_484a60); /* do not return */
    }
    sub_40d2c0(v0);
    return v1;
}`,
    },
  },

  {
    id: 'O2-iproute2-ip-xdp_parse',
    name: "xdp_parse() — iproute2 ip, x86-64",
    meta: "iproute2 ip · ELF x86-64 · gcc -O2, stripped · 0x427a0",
    ged: { kuna: 0, ida: 5, ghidra: 20, binja: 39, angr: 22 },
    kuna:
`/* WARNING: earlyreturn: hoisted 1 const-guard early-return(s) in xdp_parse */
unsigned long xdp_parse(int4 *a0,void *a1,unsigned long a2,unsigned long a3,char a4,char a5,unsigned int a6)

{
  char *v1;
  int4 v10; // stack - 0x8060
  int4 v11; // stack - 0x8058
  unsigned long v12; // stack - 0x8050
  unsigned long v13; // stack - 0x40
  unsigned int v2;
  int4 v3;
  unsigned long v4; // rax
  char v5 [32];
  unsigned long v6; // stack - 0x8098
  int8 v7; // fs_offset
  uint8 v8; // stack - 0x8090
  unsigned int v9; // stack - 0x8068
  
  v2 = a6;
  v13 = *(void *)(v7 + 0x28);
  memset(v5,0,0x8048);
  v3 = *a0;
  v12 = *a1;
  v9 = 6;
  v8 = 0;
  v6 = a2;
  v11 = v3;
  if ((char)v2 != '\\0') {
    v10 = ll_name_to_index(a3);
    if (v10 == 0) {
                    /* WARNING: Subroutine does not return */
      incomplete_command();
    }
    v3 = *a0;
  }
  if (dat_aa214 == 0) {
    v8 = v8 | 1;
  }
  if (a4 != '\\0') {
    v8 = v8 | 2;
  }
  if (a5 != '\\0') {
    v8 = v8 | 4;
  }
  if ((char)v2 != '\\0') {
    v8 = v8 | 8;
  }
  if (v3 == 1) {
    v1 = *(char **)*a1;
    if ((strcmp(v1,"none") == 0) || (strcmp(v1,"off") == 0)) {
      sub_42680(&v6,0xffffffff,0);
      return 0;
    }
  }
  v4 = bpf_parse_and_load_common(v5,0xa2390,&v6);
  if ((int4)v4 == 0) {
    *a0 = v11;
    *a1 = v12;
    return v4;
  }
  return 0xffffffff;
}`,
    vs: {
      source:
`int xdp_parse(int *argc, char ***argv, struct iplink_req *req,
	      const char *ifname, bool generic, bool drv, bool offload)
{
	struct bpf_cfg_in cfg = {
		.type = BPF_PROG_TYPE_XDP,
		.argc = *argc,
		.argv = *argv,
	};
	struct xdp_req xdp = {
		.req = req,
	};

	if (offload) {
		int ifindex = ll_name_to_index(ifname);

		if (!ifindex)
			incomplete_command();
		cfg.ifindex = ifindex;
	}

	if (!force)
		xdp.flags |= XDP_FLAGS_UPDATE_IF_NOEXIST;
	if (generic)
		xdp.flags |= XDP_FLAGS_SKB_MODE;
	if (drv)
		xdp.flags |= XDP_FLAGS_DRV_MODE;
	if (offload)
		xdp.flags |= XDP_FLAGS_HW_MODE;

	if (*argc == 1) {
		if (strcmp(**argv, "none") == 0 ||
		    strcmp(**argv, "off") == 0)
			return xdp_delete(&xdp);
	}

	if (bpf_parse_and_load_common(&cfg, &bpf_cb_ops, &xdp))
		return -1;

	*argc = cfg.argc;
	*argv = cfg.argv;
	return 0;
}`,
      ghidra:
`undefined8
xdp_parse(int *param_1,undefined8 *param_2,undefined8 param_3,undefined8 param_4,char param_5,
         undefined4 param_6)

{
  undefined4 uVar1;
  char *__s1;
  undefined1 *puVar2;
  int iVar3;
  undefined8 uVar4;
  undefined1 *puVar5;
  long in_FS_OFFSET;
  undefined1 local_8030 [32768];
  
  puVar2 = &stack0xffffffffffffffd0;
  do {
    puVar5 = puVar2;
    *(undefined8 *)(puVar5 + -0x1000) = *(undefined8 *)(puVar5 + -0x1000);
    puVar2 = puVar5 + -0x1000;
  } while (puVar5 + -0x1000 != local_8030);
  uVar1 = *(undefined4 *)(puVar5 + 0x7038);
  *(undefined8 *)(puVar5 + -0x1070) = param_4;
  *(undefined4 *)(puVar5 + -0x1074) = param_6;
  *(undefined8 *)(puVar5 + 0x6ff0) = *(undefined8 *)(in_FS_OFFSET + 0x28);
  *(undefined8 *)(puVar5 + -0x1080) = 0x142810;
  memset(puVar5 + -0x1058,0,0x8048);
  iVar3 = *param_1;
  uVar4 = *param_2;
  *(undefined8 *)(puVar5 + -0x1068) = param_3;
  *(undefined4 *)(puVar5 + -0x1038) = 6;
  *(int *)(puVar5 + -0x1028) = iVar3;
  *(undefined8 *)(puVar5 + -0x1020) = uVar4;
  *(undefined8 *)(puVar5 + -0x1060) = 0;
  if ((char)uVar1 != '\\0') {
    *(undefined8 *)(puVar5 + -0x1080) = 0x142922;
    iVar3 = ll_name_to_index(*(undefined8 *)(puVar5 + -0x1070));
    if (iVar3 == 0) {
                    /* WARNING: Subroutine does not return */
      *(undefined **)(puVar5 + -0x1080) = &UNK_00142943;
      incomplete_command();
    }
    *(int *)(puVar5 + -0x1030) = iVar3;
    iVar3 = *param_1;
  }
  if (force == 0) {
    *(uint *)(puVar5 + -0x1060) = *(uint *)(puVar5 + -0x1060) | 1;
  }
  if (param_5 != '\\0') {
    *(uint *)(puVar5 + -0x1060) = *(uint *)(puVar5 + -0x1060) | 2;
  }
  if (puVar5[-0x1074] != '\\0') {
    *(uint *)(puVar5 + -0x1060) = *(uint *)(puVar5 + -0x1060) | 4;
  }
  if ((char)uVar1 != '\\0') {
    *(uint *)(puVar5 + -0x1060) = *(uint *)(puVar5 + -0x1060) | 8;
  }
  if (iVar3 == 1) {
    __s1 = *(char **)*param_2;
    *(undefined8 *)(puVar5 + -0x1080) = 0x142886;
    iVar3 = strcmp(__s1,"none");
    if (iVar3 != 0) {
      *(undefined8 *)(puVar5 + -0x1080) = 0x142899;
      iVar3 = strcmp(__s1,"off");
      if (iVar3 != 0) goto LAB_0014289d;
    }
    *(undefined8 *)(puVar5 + -0x1080) = 0x142911;
    FUN_00142680(puVar5 + -0x1068,0xffffffff,0);
    uVar4 = 0;
  }
  else {
LAB_0014289d:
    *(undefined8 *)(puVar5 + -0x1080) = 0x1428b1;
    uVar4 = bpf_parse_and_load_common(puVar5 + -0x1058,&DAT_001a2390,puVar5 + -0x1068);
    if ((int)uVar4 == 0) {
      *param_1 = *(int *)(puVar5 + -0x1028);
      *param_2 = *(undefined8 *)(puVar5 + -0x1020);
    }
    else {
      uVar4 = 0xffffffff;
    }
  }
  if (*(long *)(puVar5 + 0x6ff0) == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar5 + -0x1080) = 0x14293e;
  __stack_chk_fail();
}`,
      ida:
`// positive sp value has been detected, the output may be wrong!
long long xdp_parse(int *a1, const char ***a2, long long a3, char *a4, char a5, char a6, char a7)
{
  int v9; // eax
  const char **v10; // rdx
  const char *v11; // r12
  long long result; // rax
  int v13; // eax
  long long v14; // rdx
  long long v15; // rcx
  char v16; // [rsp-74h] [rbp-80A4h]
  char *v17; // [rsp-70h] [rbp-80A0h]
  long long v18; // [rsp-68h] [rbp-8098h] BYREF
  long long v19; // [rsp-60h] [rbp-8090h]
  int v20[12]; // [rsp-58h] [rbp-8088h] BYREF
  int v21; // [rsp-28h] [rbp-8058h]
  const char **v22; // [rsp-20h] [rbp-8050h]
  char v23; // [rsp+0h] [rbp-8030h] BYREF
  long long v24; // [rsp+1000h] [rbp-7030h] BYREF
  unsigned long long v25; // [rsp+7FF0h] [rbp-40h]

  while ( &v23 != (char *)(&v24 - 4096) )
    ;
  v17 = a4;
  v16 = a6;
  v25 = __readfsqword(0x28u);
  memset(v20, 0, 0x8048u);
  v9 = *a1;
  v10 = *a2;
  v18 = a3;
  v20[8] = 6;
  v21 = v9;
  v22 = v10;
  v19 = 0;
  if ( a7 )
  {
    v13 = ll_name_to_index(v17);
    if ( !v13 )
      incomplete_command(v17, 0, v14, v15);
    v20[10] = v13;
    v9 = *a1;
  }
  if ( !force )
    LODWORD(v19) = v19 | 1;
  if ( a5 )
    LODWORD(v19) = v19 | 2;
  if ( v16 )
    LODWORD(v19) = v19 | 4;
  if ( a7 )
    LODWORD(v19) = v19 | 8;
  if ( v9 == 1 && ((v11 = **a2, !strcmp(v11, "none")) || !strcmp(v11, "off")) )
  {
    sub_42680(&v18, 0xFFFFFFFF);
    return 0;
  }
  else
  {
    result = ((long long (*)(int *, void *, long long *))bpf_parse_and_load_common)(v20, &unk_A2390, &v18);
    if ( (int)result )
    {
      return 0xFFFFFFFFLL;
    }
    else
    {
      *a1 = v21;
      *a2 = v22;
    }
  }
  return result;
}`,
      binja:
`int64_t xdp_parse(int32_t* arg1, int64_t* arg2, int64_t arg3, char* arg4, char arg5, char arg6, char arg7)

{
    int64_t __saved_rbx;
    int64_t* i = &__saved_rbx;
    void var_8030;
    
    do
    {
        i -= 0x1000;
        *i = *i;
    } while (i != &var_8030);
    
    void* fsbase;
    int64_t rax = *(fsbase + 0x28);
    char* var_8088;
    memset(&var_8088, 0, 0x8048);
    int32_t rax_2 = *arg1;
    int64_t rdx = *arg2;
    int64_t var_8098 = arg3;
    int32_t var_8068 = 6;
    int32_t var_8058 = rax_2;
    int32_t var_8090 = 0;
    
    if (arg7)
    {
        int32_t rax_6 = ll_name_to_index(arg4);
        
        if (!rax_6)
        {
            incomplete_command();
            /* no return */
        }
        
        int32_t var_8060_1 = rax_6;
        rax_2 = *arg1;
    }
    
    if (!force)
        var_8090 |= 1;
    
    if (arg5)
        var_8090 |= 2;
    
    if (arg6)
        var_8090 |= 4;
    
    if (arg7)
        var_8090 |= 8;
    
    int32_t rax_4;
    int32_t rax_5;
    
    if (rax_2 == 1)
    {
        char* r12_1 = **arg2;
        rax_4 = strcmp(r12_1, "none");
        
        if (rax_4)
            rax_5 = strcmp(r12_1, "off");
    }
    
    int32_t result;
    
    if (rax_2 == 1 && (!rax_4 || !rax_5))
    {
        sub_442680(&var_8098, 0xffffffff);
        result = 0;
    }
    else
    {
        result = bpf_parse_and_load_common(&var_8088, &data_4a2390, &var_8098);
        
        if (result)
            result = -1;
        else
        {
            *arg1 = var_8058;
            *arg2 = rdx;
        }
    }
    
    *(fsbase + 0x28);
    
    if (rax == *(fsbase + 0x28))
        return result;
    
    __stack_chk_fail();
    /* no return */
}`,
      angr:
`typedef struct struct_0 {
    char *field_0;
} struct_0;

extern unsigned int force;
extern long long g_4a2390;

int xdp_parse(void)
{
    char v10;  // r14b
    unsigned long v11;  // fs
    unsigned int *v12;  // rdi
    unsigned int v13;  // eax
    struct_0 **v14;  // rsi
    long long v15;  // rcx
    unsigned int v16;  // eax
    char v17;  // r8b
    char v18;  // r9b
    unsigned int v0;  // [bp-0x8090], Other Possible Types: unsigned long long, unsigned long
    char v1;  // [bp-0x8088]
    unsigned int v2;  // [bp-0x8068]
    unsigned int v3;  // [bp-0x8060]
    unsigned int v4;  // [bp-0x8058]
    char **v5;  // [bp-0x8050]
    char v6;  // [bp-0x8030]
    unsigned long v7;  // [bp-0x2030]
    unsigned long v8;  // [bp-0x40]
    unsigned int v9;  // [bp+0x8]

    do
    {
    } while (&v7 != &v6);
    v10 = v9;
    v8 = *((long long *)(40 + v11));
    memset(&v1, 0, 32840);
    v13 = *(v12);
    v2 = 6;
    v4 = *(v12);
    v5 = &*(v14)->field_0;
    v0 = 0;
    if (v10)
    {
        v16 = ll_name_to_index(v15);
        if (!v16)
            incomplete_command(); /* do not return */
        v3 = v16;
        v13 = *(v12);
    }
    if (!force)
        *((unsigned int *)&v0) = (unsigned int)v0 | 1;
    if (v17)
        *((unsigned int *)&v0) = (unsigned int)v0 | 2;
    if (v18)
        *((unsigned int *)&v0) = (unsigned int)v0 | 4;
    if (v10)
        v0 = (unsigned int)v0 | 8;
    if (!(v13 != 1 || strcmp(*(v14)->field_0, "none") && strcmp(*(v14)->field_0, "off")))
    {
        sub_442680(&/* unsupported instruction */, 4294967295);
    }
    else if (!bpf_parse_and_load_common(&v1, &g_4a2390, &/* unsupported instruction */))
    {
        *(v12) = v4;
        *(v14) = v5;
    }
    if (v8 != *((long long *)(40 + v11)))
        __stack_chk_fail(); /* do not return */
    return;
}`,
    },
  },

  {
    id: 'O2-noinline-cleanflight-cleanflight_DALRCF405-sendSatalliteSignalQualityAsTemperature2',
    name: "sendSatalliteSignalQualityAsTemperature2() — cleanflight_DALRCF405, ARM",
    meta: "cleanflight_DALRCF405 · ELF ARM · gcc -O2 -fno-inline, stripped · 0x80339d8",
    ged: { kuna: 0, ida: 3, ghidra: 3, binja: 8, angr: 3 },
    kuna:
`/* WARNING: iteregion: rewrote 1 if/else assignment diamond(s) to ?: ternary in sendSatalliteSignalQualityAsTemperature2 */
void sendSatalliteSignalQualityAsTemperature2(uint4 a0)

{
  uint2 v1;
  int2 v2;
  uint4 v3; // r4
  float4 v4; // s14
  float4 v5; // s15
  
  v3 = (uint4)*(uint1 *)(dat_8033a44 + 0x14);
  v1 = *(uint2 *)(dat_8033a44 + 0x12);
  if ((0x12d <= v1) && ((a0 & 8) == 0)) {
    v3 = sub_80337f4(v1,0,9999) & 0xffff;
  }
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (*(char *)(sub_8033810() + 8) != '\\0') {
    v4 = (float4)(int8)(int4)(v3 - 0x20) / dat_8033a48;
    v5 = (v4 < 0.0) ? -0.5 : 0.5;
    v2 = (int2)(int4)(v5 + v4);
  }
  else {
    v2 = (int2)v3;
  }
  sub_803384c(5,(int4)v2);
  return;
}`,
    vs: {
      source:
`static void sendSatalliteSignalQualityAsTemperature2(uint8_t cycleNum)
{
    uint16_t satellite = gpsSol.numSat;

    if (gpsSol.hdop > GPS_BAD_QUALITY && ( (cycleNum % 16 ) < 8)) { // Every 1s
        satellite = constrain(gpsSol.hdop, 0, GPS_MAX_HDOP_VAL);
    }
    int16_t data;
    if (telemetryConfig()->frsky_unit == FRSKY_UNIT_METRICS) {
        data = satellite;
    } else {
        float tmp = (satellite - 32) / 1.8f;
        // Round the value
        tmp += (tmp < 0) ? -0.5f : 0.5f;
        data = tmp;
    }
    frSkyHubWriteFrame(ID_TEMPRATURE2, data);
}`,
      ghidra:
`void sendSatalliteSignalQualityAsTemperature2(uint param_1)

{
  short sVar1;
  int iVar2;
  uint uVar3;
  uint in_fpscr;
  float fVar4;
  float fVar5;
  
  uVar3 = (uint)*(byte *)(DAT_08033a44 + 0x14);
  if ((300 < *(ushort *)(DAT_08033a44 + 0x12)) && ((param_1 & 8) == 0)) {
    uVar3 = FUN_080337f4(*(ushort *)(DAT_08033a44 + 0x12),0,9999);
    uVar3 = uVar3 & 0xffff;
  }
  iVar2 = FUN_08033810();
  if (*(char *)(iVar2 + 8) == '\\0') {
    sVar1 = (short)uVar3;
  }
  else {
    fVar4 = (float)VectorSignedToFloat(uVar3 - 0x20,(byte)(in_fpscr >> 0x16) & 3);
    if ((int)((uint)(fVar4 / DAT_08033a48 < 0.0) << 0x1f) < 0) {
      fVar5 = -0.5;
    }
    else {
      fVar5 = 0.5;
    }
    sVar1 = (short)(int)(fVar5 + fVar4 / DAT_08033a48);
  }
  FUN_0803384c(5,(int)sVar1);
  return;
}`,
      ida:
`int sendSatalliteSignalQualityAsTemperature2(char a1)
{
  int v1; // r4
  short v2; // r1
  float v4; // s14
  float v5; // s15

  v1 = (unsigned char)byte_2000CA0C;
  if ( (unsigned short)word_2000CA0A > 0x12Cu && (a1 & 8) == 0 )
    v1 = (unsigned short)sub_80337F4((unsigned short)word_2000CA0A, 0, 9999);
  if ( *(char *)(sub_8033810() + 8) )
  {
    v4 = (float)(v1 - 32) / 1.8;
    if ( v4 < 0.0 )
      v5 = -0.5;
    else
      v5 = 0.5;
    v2 = (int)(float)(v5 + v4);
  }
  else
  {
    v2 = v1;
  }
  return sub_803384C(5, v2);
}`,
      binja:
`int32_t sendSatalliteSignalQualityAsTemperature2(int32_t arg1)

{
    uint32_t r4 = data_2000ca0c;
    uint32_t r3 = data_2000ca0a;
    
    if (r3 > 0x12c && !(arg1 & 8))
        r4 = sub_80337f4(r3, 0, 0x270f);
    
    int32_t r1;
    
    if (data_2000cda0)
    {
        int128_t q3;
        *(&q3 + 0xc) = r4 - 0x20;
        *(&q3 + 0xc) = *(&q3 + 0xc);
        *(&q3 + 4) = 0x3fe66666;
        *(&q3 + 8) = *(&q3 + 0xc) / 1.79999995f;
🚫        /* unimplemented  {vcmpe.f32 s14, #0} */
🚫        /* unimplemented  {vmrs apsr_nzcv, fpscr} */
        
        if (r4 - 0x20 < 0)
            *(&q3 + 0xc) = 0xbf000000;
        else
            *(&q3 + 0xc) = 0x3f000000;
        
        *(&q3 + 0xc) = *(&q3 + 0xc) + *(&q3 + 8);
        *(&q3 + 0xc) = roundf(*(&q3 + 0xc));
        r1 = *(&q3 + 0xc);
    }
    else
        r1 = r4;
    
    return sub_803384c(5, r1);
}`,
      angr:
`extern char g_10000000;
extern unsigned short g_2000ca0a;
extern char g_2000ca0c;

int sendSatalliteSignalQualityAsTemperature2(unsigned int a0)
{
    unsigned int v0;  // r4
    unsigned int v1;  // r1
    unsigned int v2;  // s14
    unsigned int v3;  // fpscr

    v0 = g_2000ca0c;
    if (g_2000ca0a > 300 && !((char)a0 & 8))
        v0 = sub_80337f5(g_2000ca0a, 0, 9999) & 0xffff;
    if (!*((char *)(sub_8033811() + 8)))
    {
        v1 = v0 & 0xffff;
    }
    else
    {
        v2 = (v0 - 32) / 0x3fe66666;
        v1 = (((v3 & 0xfffffff | ((((CmpF(v2, 0.0) >> 5 & 3 | CmpF(v2, 0.0) & 1) ^ 1) * 0x40000000 - 1 >> 29) + 1 - ((CmpF(v2, 0.0) >> 5 & 3 | CmpF(v2, 0.0) & 1) & (CmpF(v2, 0.0) >> 5 & 3 | CmpF(v2, 0.0) & 1) >> 1 & 1)) * &g_10000000) & 0xf0000000) < 0 ? (unsigned short)-0.5 : (unsigned short)0.5) + v2 & 0xffff;
    }
    return sub_803384d(5, v1);
}`,
    },
  },
];
