unsigned long build_spec_list(E_string *a0,Spec_list *a1)

{
  uint8 len; // rax
  int8 v1;
  uint8 v10;
  uint8 v11; // stack - 0x50
  uint8 v12; // stack - 0x48
  uint8 v12; // stack - 0x48
  uint8 v12; // stack - 0x48
  char v13; // stack - 0x51
  char v16;
  uint8 v17;
  int8 v18; // fs_offset
  int8 v19; // fs_offset
  bool v2; // al
  int8 v20; // stack - 0x40
  int4 v3; // eax
  unsigned long v4; // rax
  char *v6; // rax
  undefined7 v7; // rax
  uint8 *v8;
  uint8 v9; // rbx
  
  v10 = 0;
  v1 = *(int8 *)a0;
  v20 = *(int8 *)(v18 + 0x28);
  if (3 <= *(uint8 *)((int8)a0 + 0x10)) {
    v17 = 2;
    do {
      v10 = v9 + 1;
      v2 = es_match(a0,v9,'[');
      if (v2) {
        v2 = es_match(v15,v10,':');
        if (v2) {
label_40242e:
          v16 = *(char *)(v1 + v10);
          v8 = &v12;
          v14 = v12;
          v2 = find_closing_delim(a0,v17,v16,v8);
          if (!v2) goto label_4024b8;
          v5 = (char *)(v1 + v17);
          len = (v12 - 2) - v9;
          if (len == 0) {
            v5 = "missing character class name \'[::]\'";
            if (v16 != ':') {
              v5 = "missing equivalence class character \'[==]\'";
            }
            v4 = dcgettext(0,v5,5);
            error(0,0,v4);
            v4 = 0;
            goto label_40252c;
          }
          if (v16 != ':') {
            v2 = append_equiv_class(a1,v5,len);
            if (v2) goto label_4025f0;
            v2 = star_digits_closebracket(a0,v17);
            if (v2) goto label_4024b8;
            v6 = make_printable_str(v5,len);
            v4 = dcgettext(0,"%s: equivalence class operand must be a single character",5);
label_4025cc:
            error(0,0,v4,v6);
            free(v5);
            goto label_4025df;
          }
          v2 = append_char_class(a1,v5,len);
          if (!v2) {
            v2 = star_digits_closebracket(a0,v17);
            if (!v2) {
              v5 = make_printable_str(v5,len);
              v6 = (char *)quote(v5);
              v4 = dcgettext(0,"invalid character class %s",5);
              goto label_4025cc;
            }
            goto label_4024b8;
          }
label_4025f0:
          v14 = v14 + 2;
        }
        else {
          v2 = es_match(v15,v14,'=');
          v8 = &v12;
          if (v2) goto label_40242e;
label_4024b8:
          v3 = find_bracketed_repeat(a0,v10,&v13,v8,&v11);
          if (v3 != 0) {
            if (v3 == -1) goto label_4024da;
label_4025df:
            v4 = 0;
            goto label_40252c;
          }
          append_repeated_char(a1,v13);
          v10 = v11 + 1;
        }
      }
      else {
label_4024da:
        v2 = es_match(a0,v10,'-');
        if (v2) {
          v2 = append_range(a1,*(char *)(v1 + v9),*(char *)(v1 + v17));
          v4 = CONCAT71(v7,v2);
          if (!v2) goto label_40252c;
          v9 = v9 + 3;
        }
        else {
          append_normal_char(a1,*(char *)(v1 + v9));
        }
      }
      v17 = v10 + 2;
    } while (v17 < *(uint8 *)((int8)a0 + 0x10));
  }
  for (; v10 < *(uint8 *)((int8)a0 + 0x10); v10 = v10 + 1) {
    append_normal_char(a1,*(char *)(v1 + v10));
  }
  v4 = 1;
label_40252c:
  if (v20 == *(int8 *)(v19 + 0x28)) {
    return v4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}
