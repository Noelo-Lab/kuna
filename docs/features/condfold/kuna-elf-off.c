bool build_spec_list(E_string *es,Spec_list *result)

{
  E_string *es_00;
  uint8 i;
  char pre_bracket_char;
  uint8 *result_idx;
  uint8 start_idx;
  int8 v1;
  char v10; // stack - 0x51
  uint8 v11;
  int8 v12; // fs_offset
  unsigned long v13; // stack - 0x40
  bool v2; // al
  int4 v3; // eax
  uint8 v4;
  char *v5;
  char *v6; // rax
  unsigned long v7; // rax
  uint8 v8; // stack - 0x50
  uint8 v9; // stack - 0x48
  
  i = 0;
  v1 = *(int8 *)es;
  v13 = *(void *)(v12 + 0x28);
  if (3 <= *(uint8 *)((int8)es + 0x10)) {
    start_idx = 2;
    i = i;
    do {
      i = i + 1;
      es_00 = es;
      if (es_match(es,i,'[')) {
        v11 = i;
        v2 = es_match(es_00,i,':');
        if (v2) {
label_40242e:
          pre_bracket_char = *(char *)(v1 + i);
          result_idx = &v9;
          v4 = v9;
          v11 = v9;
          v2 = find_closing_delim(es,start_idx,pre_bracket_char,result_idx);
          v9 = v4;
          v9 = v9;
          if (!v2) goto label_4024b8;
          v5 = (char *)(v1 + start_idx);
          v4 = (v9 - 2) - i;
          if (v4 == 0) {
            v5 = "missing character class name \'[::]\'";
            if (pre_bracket_char != ':') {
              v5 = "missing equivalence class character \'[==]\'";
            }
            error(0,0,dcgettext(0,v5,5));
            return 0;
          }
          if (pre_bracket_char == ':') {
            v9 = v9;
            if (append_char_class(result,v5,v4)) goto label_4025f0;
            if (!star_digits_closebracket(es,start_idx)) {
              v5 = make_printable_str(v5,v4);
              v6 = (char *)quote(v5);
              v7 = dcgettext(0,"invalid character class %s",5);
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
              error(0,0,v7,v6);
              free(v5);
              return 0;
            }
            goto label_4024b8;
          }
          if (!append_equiv_class(result,v5,v4)) {
            if (!star_digits_closebracket(es,start_idx)) {
              v6 = make_printable_str(v5,v4);
              v7 = dcgettext(0,"%s: equivalence class operand must be a single character",5);
              v5 = v6;
              error(0,0,v7,v6);
              free(v5);
              return 0;
            }
            goto label_4024b8;
          }
label_4025f0:
          i = v11 + 2;
        }
        else {
          result_idx = &v9;
          if (es_match(es_00,v11,'=')) goto label_40242e;
label_4024b8:
          v3 = find_bracketed_repeat(es,i,&v10,result_idx,&v8);
          if (v3 != 0) {
            if (v3 != -1) {
              return 0;
            }
            goto label_4024da;
          }
          append_repeated_char(result,v10);
          i = v8 + 1;
        }
      }
      else {
label_4024da:
        if (es_match(es,i,'-')) {
          v2 = append_range(result,*(char *)(v1 + i),*(char *)(v1 + start_idx));
          if (!v2) {
            return v2;
          }
          i = i + 3;
        }
        else {
          append_normal_char(result,*(char *)(v1 + i));
        }
      }
      start_idx = i + 2;
      i = i;
    } while (start_idx < *(uint8 *)((int8)es + 0x10));
  }
  for (; i < *(uint8 *)((int8)es + 0x10); i = i + 1) {
    append_normal_char(result,*(char *)(v1 + i));
  }
  return 1;
}
