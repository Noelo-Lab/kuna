unsigned long build_spec_list(int8 *a0,unsigned long a1)

{
  int8 v1;
  unsigned long v10; // rdx
  unsigned long v11; // rdx
  uint8 v12;
  uint8 v13;
  int8 v14; // stack - 0x48
  char v15; // stack - 0x51
  char v16;
  uint8 v17;
  int8 v18; // fs_offset
  int8 v19; // stack - 0x50
  int8 v2;
  unsigned long v20; // stack - 0x40
  int8 v3;
  int4 v4; // eax
  unsigned long v5;
  int8 v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  int8 *v9;
  
  v13 = 0;
  v2 = *a0;
  v20 = *(void *)(v18 + 0x28);
  if (3 <= (uint8)a0[2]) {
    v17 = 2;
    v12 = v13;
    do {
      v13 = v12 + 1;
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
      if (es_match(a0,v12,0x5b) != '\0') {
        if ((es_match() != '\0') || (es_match() != '\0')) {
          v16 = *(char *)(v2 + v13);
          v9 = &v14;
          v1 = v14;
          v3 = v14;
          v14 = v1;
          v14 = v14;
          if (find_closing_delim(a0,v17) == '\0') goto label_400d08;
          v1 = v2 + v17;
          v6 = (v14 + -2) - v12;
          if (v6 == 0) {
            v5 = 0x404858;
            if (v16 != ':') {
              v5 = 0x404880;
            }
            error(0,0,dcgettext(0,v5,5));
            return 0;
          }
          if (v16 == ':') {
            v14 = v14;
            if (append_char_class(a1,v1,v6) != '\0') goto label_400e40;
            if (star_digits_closebracket(a0,v17) == '\0') {
              v5 = make_printable_str(v1,v6,v11,v9);
              v7 = quote(v5);
              v8 = dcgettext(0,0x4044bf,5);
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
              error(0,0,v8,v7);
              free(v5);
              return 0;
            }
            goto label_400d08;
          }
          if (append_equiv_class() == '\0') {
            if (star_digits_closebracket(a0,v17) == '\0') {
              v7 = make_printable_str(v1,v6,v10,v9);
              v8 = dcgettext(0,0x4048b0,5);
              v5 = v7;
              error(0,0,v8,v7);
              free(v5);
              return 0;
            }
            goto label_400d08;
          }
label_400e40:
          v13 = v3 + 2;
        }
        else {
label_400d08:
          v4 = find_bracketed_repeat(a0,v13,&v15);
          if (v4 != 0) {
            if (v4 != -1) {
              return 0;
            }
            goto label_400d2a;
          }
          append_repeated_char(a1,v15,v14);
          v13 = v19 + 1;
        }
      }
      else {
label_400d2a:
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
        if (es_match(a0,v13,0x2d) != '\0') {
          v5 = append_range(a1,*(void *)(v2 + v12),*(void *)(v2 + v17));
          if ((char)v5 == '\0') {
            return v5;
          }
          v13 = v12 + 3;
        }
        else {
          append_normal_char(a1,*(void *)(v2 + v12));
        }
      }
      v17 = v13 + 2;
      v12 = v13;
    } while (v17 < (uint8)a0[2]);
  }
  for (; v13 < (uint8)a0[2]; v13 = v13 + 1) {
    append_normal_char(a1,*(void *)(v2 + v13));
  }
  return 1;
}
