// tar-xstrcat  tar @ 0x611a0  (kuna before the undefname fix)
// ScopeLocal symbol table (the --json `variables` array):
//   {"name": "param_1", "type": "int8", "kind": "arg", "arg_index": 0, "stack_offset": null, "size": 8}
//   {"name": "param_2", "type": "uint8 *", "kind": "arg", "arg_index": 1, "stack_offset": null, "size": 8}
//   {"name": "v12", "type": "uint8", "kind": "stack", "arg_index": null, "stack_offset": -88, "size": 8}
//   {"name": "v13", "type": "unsigned long *", "kind": "stack", "arg_index": null, "stack_offset": -80, "size": 8}

char * sub_611a0(int8 a0,uint8 *a1)
{
  uint4 $$undef00000000; // stack - 0x58
  unsigned long *v1;
  char *v10;
  bool v11; // cf
  uint8 v12; // stack - 0x58
  unsigned long *v13; // stack - 0x50
  uint4 v2;
  char *v3;
  uint8 v4; // rax
  char *v5;
  uint8 v6;
  unsigned long *v7;
  uint8 v8; // rbx
  int8 v9;
  
  v12 = *a1;
  v13 = (unsigned long *)a1[1];
  v6 = a1[2];
  if (a0) { // branch-flip
    v8 = 0;
    v9 = a0;
    do {
      v4 = v12 & 0xffffffff;
      if (0x30 <= (uint4)$$undef00000000) { // branch-flip
        v1 = &v13[1];
        v7 = v13;
        v13 = v1;
      }
      else {
        v12 = (uint8)((uint4)$$undef00000000 + 8);
        v7 = (unsigned long *)(v4 + v6);
      }
      v4 = strlen((char *)*v7);
      v11 = CARRY8(v8,v4);
      v8 += v4;
      if (v11)
        v8 = 0xffffffffffffffff;
      v9 -= 1;
    } while (v9);
    if (0x80000000 <= v8) {
      *(unsigned int *)__errno_location() = 0x4b;
      return NULL;
    }
    v5 = (char *)sub_4ecc0(v8 + 1);
    v10 = v5;
    do {
      v2 = (uint4)*a1;
      if (0x30 <= v2) { // branch-flip
        v7 = (unsigned long *)a1[1];
        a1[1] = (uint8)&v7[1];
      }
      else {
        v7 = (unsigned long *)((uint8)v2 + a1[2]);
        *(uint4 *)a1 = v2 + 8;
      }
      v3 = (char *)*v7;
      v6 = strlen(v3);
      memcpy(v10,v3,v6);
      v10 = &v10[v6];
      a0 -= 1;
    } while (a0);
  }
  else {
    v10 = (char *)sub_4ecc0(1);
    v5 = v10;
  }
  *v10 = 0;
  return v5;
}