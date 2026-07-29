unsigned long build_spec_list(E_string *a0,Spec_list *a1)

{
  E_string *es;
  uint8 i;
  char pre_bracket_char;
  uint8 *result_idx;
  uint8 start_idx;
  int8 v1;
  uint8 v10; // stack - 0x48
  char v11; // stack - 0x51
  uint8 v12;
  int8 v13; // fs_offset
  unsigned long v14; // stack - 0x40
  bool v2; // al
  int4 v3; // eax
  uint8 v4;
  char *v5;
  char *v6; // rax
  unsigned long v7; // rax
  undefined7 v8; // rax
  uint8 v9; // stack - 0x50
  
  i = 0;
  v1 = *(int8 *)a0;
  v14 = *(void *)(v13 + 0x28);
  if (3 <= *(uint8 *)((int8)a0 + 0x10)) {
    start_idx = 2;
    i = i;
    do {
      i = i + 1;
      es = a0;
      if (es_match(a0,i,'[')) {
        v12 = i;
        v2 = es_match(es,i,':');
        if (((v2) || (result_idx = &v10, es_match(es,v12,'='))) && (pre_bracket_char = *(char *)(v1 + i), result_idx = &v10, v4 = v10, v12 = v10, v2 = find_closing_delim(a0,start_idx,pre_bracket_char,result_idx), v10 = v4, v10 = v10, v2)) {
          v5 = (char *)(v1 + start_idx);
          v4 = (v10 - 2) - i;
          if (v4 == 0) {
            v5 = "missing character class name \'[::]\'";
            if (pre_bracket_char != ':') {
              v5 = "missing equivalence class character \'[==]\'";
            }
            error(0,0,dcgettext(0,v5,5));
            return 0;
          }
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
          if (pre_bracket_char != ':') {
            if (!append_equiv_class(a1,v5,v4)) {
              if (!star_digits_closebracket(a0,start_idx)) {
                v6 = make_printable_str(v5,v4);
                v7 = dcgettext(0,"%s: equivalence class operand must be a single character",5);
                v5 = v6;
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
                error(0,0,v7,v6);
                free(v5);
                return 0;
              }
              goto label_4024b8;
            }
          }
          else {
            v10 = v10;
            if (!append_char_class(a1,v5,v4)) {
              if (!star_digits_closebracket(a0,start_idx)) {
                v5 = make_printable_str(v5,v4);
                v6 = (char *)quote(v5);
                v7 = dcgettext(0,"invalid character class %s",5);
                error(0,0,v7,v6);
                free(v5);
                return 0;
              }
              goto label_4024b8;
            }
          }
          i = v12 + 2;
        }
        else {
label_4024b8:
          v3 = find_bracketed_repeat(a0,i,&v11,result_idx,&v9);
          if (v3 != 0) {
            if (v3 != -1) {
              return 0;
            }
            goto label_4024da;
          }
          append_repeated_char(a1,v11);
          i = v9 + 1;
        }
      }
      else {
label_4024da:
        if (es_match(a0,i,'-')) {
          v2 = append_range(a1,*(char *)(v1 + i),*(char *)(v1 + start_idx));
          if (!v2) {
            return CONCAT71(v8,v2);
          }
          i = i + 3;
        }
        else {
          append_normal_char(a1,*(char *)(v1 + i));
        }
      }
      start_idx = i + 2;
      i = i;
    } while (start_idx < *(uint8 *)((int8)a0 + 0x10));
  }
  for (; i < *(uint8 *)((int8)a0 + 0x10); i = i + 1) {
    append_normal_char(a1,*(char *)(v1 + i));
  }
  return 1;
}
