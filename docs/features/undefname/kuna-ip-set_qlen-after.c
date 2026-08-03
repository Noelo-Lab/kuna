// ip-set_qlen  ip @ 0x2b420  (kuna after the undefname fix)
// ScopeLocal symbol table (the --json `variables` array):
//   {"name": "param_1", "type": "unsigned long", "kind": "arg", "arg_index": 0, "stack_offset": null, "size": 8}
//   {"name": "param_2", "type": "unsigned int", "kind": "arg", "arg_index": 1, "stack_offset": null, "size": 4}
//   {"name": "v2", "type": "char[16]", "kind": "stack", "arg_index": null, "stack_offset": -72, "size": 16}
//   {"name": "v7", "type": "unsigned int", "kind": "stack", "arg_index": null, "stack_offset": -56, "size": 4}
//   {"name": "v4", "type": "char[12]", "kind": "stack", "arg_index": null, "stack_offset": -52, "size": 12}
//   {"name": "v5", "type": "unsigned long", "kind": "stack", "arg_index": null, "stack_offset": -40, "size": 8}
//   {"name": "v6", "type": "unsigned long", "kind": "stack", "arg_index": null, "stack_offset": -32, "size": 8}

unsigned long sub_2b420(unsigned long a0,unsigned int a1) // return-dupe x2
{
  int4 v1; // eax
  char v2 [16]; // stack - 0x48
  int8 v3; // fs_offset
  char v4 [12]; // stack - 0x34
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  char v7 [16]; // stack - 0x38
  
  v6 = *(unsigned long *)(v3 + 0x28);
  v5 = 0;
  v4[0] = SUB1612(0,4);
  v7 = CONCAT124(v4[0],a1);
  v2[0] = 0;
  v1 = sub_2b2f0();
  if (v1 < 0)
    return 0xffffffff;
  strlcpy(v2,a0,0x10);
  if (0 <= (int4)ioctl(v1,0x8943,v2)) {
    close(v1);
    return 0;
  }
  perror("SIOCSIFXQLEN");
  close(v1);
  return 0xffffffff;
}