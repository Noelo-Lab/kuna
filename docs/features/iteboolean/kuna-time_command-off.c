uint4 sub_4ea10(int4 *a0,unsigned int a1,unsigned int a2,unsigned int a3,unsigned long a4)
{
  unsigned long v1;
  char v10 [16];
  char v11 [16];
  char v12 [16];
  unsigned int v13; // stack - 0x3c8
  unsigned long v14; // stack - 0x3a0
  char v15 [16];
  unsigned int v16; // stack - 0x3c4
  unsigned long v17; // stack - 0x398
  char v18 [128];
  unsigned long v19; // stack - 0x1f8
  uint4 v2; // eax
  char v20 [128];
  unsigned long v21; // stack - 0x318
  char v22 [16];
  unsigned int v23; // stack - 0x3c0
  unsigned long v24; // stack - 0x390
  int8 v25; // fs_offset
  int4 v26; // stack - 0x3bc
  unsigned int v27; // stack - 0x3b8
  int4 v28; // stack - 0x3b4
  int4 v29; // stack - 0x3b0
  unsigned long v3; // rax
  int4 v30; // stack - 0x3ac
  int4 v31; // stack - 0x3a8
  unsigned int v32; // stack - 0x3a4
  char *v33; // stack - 0x388
  unsigned long v34; // stack - 0x340
  unsigned long v35; // stack - 0x320
  unsigned long v36; // stack - 0x310
  unsigned long v37; // stack - 0x200
  unsigned long v38; // stack - 0x1f0
  unsigned long v39; // stack - 0x20
  char v4 [8];
  unsigned long v5; // stack - 0x348
  unsigned long v6; // stack - 0x328
  unsigned long v7; // stack - 0x208
  char v8 [200];
  char v9 [16];
  
  v39 = *(unsigned long *)(v25 + 0x28);
  gettimeofday(&v5,v4);
  getrusage(0,&v6);
  getrusage(0xffffffff,&v7);
  v26 = subshell_environment;
  if ((a0) && (v2 = a0[1] & 0x100, v2))
    v2 = 1;
  else {
    v2 = 0;
  }
  v28 = v2;
  if ((a0) && (((*a0 != 4 || (*(int8 *)(*(int8 *)&a0[6] + 8))) || (*(int8 *)(*(int8 *)&a0[6] + 0x10)))))
    v2 = 0;
  else {
    v2 = 1;
  }
  if ((posixly_correct) && (v2)) {
    v19 = 0;
    v21 = 0;
    v7 = 0;
    v6 = 0;
    v38 = 0;
    v36 = 0;
    v37 = 0;
    v35 = 0;
    v5 = shellstart[0];
    v34 = shellstart[8];
  }
  v30 = a0[1];
  v29 = v2;
  xbcopy(top_level,v8,200);
  v2 = a0[1] & 0xfffffe7f;
  a0[1] = v2;
  v2 = __sigsetjmp(top_level,0);
  v31 = v2;
  if (!v2) {
    v2 = execute_command_internal(a0,a1,a2,a3,a4);
    v27 = v2;
  }
  xbcopy(v8,top_level,200);
  a0[1] = v30;
  if (((v31) && (subshell_environment)) && (subshell_environment != v26))
    siglongjmp(top_level,v31); // no-return
  v24 = 0;
  v17 = 0;
  v14 = 0;
  v32 = 0;
  v23 = 0;
  v16 = 0;
  v13 = 0;
  gettimeofday(v9,v4);
  getrusage(0,v10);
  getrusage(0xffffffff,v11);
  difftimeval(v12,&v5,v9);
  timeval_to_secs(v12,&v14,&v13);
  v3 = (unsigned long)difftimeval(&v5,&v7,v11);
  v1 = v3;
  v3 = (unsigned long)difftimeval(v9,&v6,v10);
  addtimeval(v15,v3,v1);
  timeval_to_secs(v15,&v17,&v16);
  v3 = (unsigned long)difftimeval(&v5,&v19,v18);
  v1 = v3;
  v3 = (unsigned long)difftimeval(v9,&v21,v20);
  addtimeval(v22,v3,v1);
  timeval_to_secs(v22,&v24,&v23);
  v2 = timeval_to_cpu(v12,v15,v22);
  v32 = v2;
  if (v28) // branch-flip
    v33 = "real %2R\nuser %2U\nsys %2S";
  else {
    v3 = (char *)get_string_value("TIMEFORMAT");
    v33 = v3;
    if (!v3) {
      if ((posixly_correct) && (v29))
        v33 = "user\t%2lU\nsys\t%2lS";
      else {
        v33 = "\nreal\t%3lR\nuser\t%3lU\nsys\t%3lS";
      }
    }
  }
  if ((v33) && (*v33))
    sub_4e553(stderr,v33,v14,v13,v17,v16,v24,v23,v32);
  if (!v31) {
    v2 = v27;
    return v2;
  }
  siglongjmp(top_level,v31); // no-return
}
