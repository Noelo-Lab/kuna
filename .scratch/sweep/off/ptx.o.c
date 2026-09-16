// Function: to_uchar @ 0x400000
char to_uchar(char ch)
{
  return ch;
}

// Function: compare_words @ 0x400010
int compare_words(void *void_first,void *void_second) // ternary
{
  long v1;
  long v10;
  long v11;
  long v12;
  unsigned char v2; // al
  int v3; // eax
  unsigned int v4; // eax
  long v5;
  unsigned int v6; // edx
  long v7; // rdx
  long v8; // rsi
  char *v9; // r8
  
  v11 = *(long *)((long)void_second + 8);
  v10 = *(long *)((long)void_first + 8);
  v5 = v10;
  if (v11 <= v10)
    v5 = v11;
  if (ignore_case) { // branch-flip
    if (1 <= v5) {
      v1 = *(long *)void_first;
      v12 = *(long *)void_second;
      v9 = folded_chars;
      v8 = 0;
      do {
        to_uchar(*(char *)(v1 + v8));
        v2 = to_uchar(*(char *)(v12 + v8));
        if ((unsigned int)(unsigned char)v9[v7] - (unsigned int)(unsigned char)v9[v2])
          return (unsigned int)(unsigned char)v9[v7] - (unsigned int)(unsigned char)v9[v2];
        v8 += 1;
      } while (v5 != v8);
    }
  }
  else if (1 <= v5) {
    v1 = *(long *)void_first;
    v12 = *(long *)void_second;
    v8 = 0;
    do {
      to_uchar(*(char *)(v1 + v8));
      v2 = to_uchar(*(char *)(v12 + v8));
      v3 = (v6 & 0xff) - (unsigned int)v2;
      if (v3)
        return v3;
      v8 += 1;
    } while (v5 != v8);
  }
  v4 = (v11 <= v10) ? (unsigned int)(v11 < v10) : 0xffffffff; // branch-flip
  return v4;
}

// Function: compare_occurs @ 0x4000e0
int compare_occurs(void *void_first,void *void_second) // ternary
{
  unsigned int v1; // eax
  
  v1 = compare_words(void_first,void_second);
  if (!v1) {
    v1 = (*(unsigned long *)void_second <= *(unsigned long *)void_first) ? (unsigned int)(*(unsigned long *)void_first != *(unsigned long *)void_second) : 0xffffffff; // branch-flip
  }
  return v1;
}

// Function: emit_stdin_note @ 0x400120
void emit_stdin_note(void)
{
  unsigned long v1;
  
  v1 = dat_407000;
  fputs_unlocked(dcgettext(0,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1); // tail-call
}

// Function: emit_mandatory_arg_note @ 0x400150
void emit_mandatory_arg_note(void)
{
  unsigned long v1;
  
  v1 = dat_407000;
  fputs_unlocked(dcgettext(0,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: matcher_error @ 0x400180
void matcher_error(void)
{
  unsigned long v1; // rax
  
  v1 = dcgettext(0,"error in regular expression matcher",5);
  error(1,*(unsigned int *)__errno_location(),v1); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: unescape_string @ 0x4001b0
void unescape_string(char *string)
{
  unsigned char v1; // al
  int v10; // r8d
  long v11;
  char v2; // cl
  int v3; // ecx
  int v4; // edx
  int v5;
  char *v6;
  char *v7;
  char *v8;
  char v9;
  
  v9 = *string;
  v7 = string;
label_4001bf:
  if (v9) {
    v6 = &string[1];
    if (v9 != '\\') { // branch-flip
      *v7 = v9;
      v7 = &v7[1];
      v9 = string[1];
      string = v6;
    }
    else {
      if (!string[1]) goto label_400219;
      switch(string[1]) {
        case 0x30:
          v8 = &string[2];
          v2 = '\0';
          string = &string[5];
          do {
            v9 = *v8;
            if (8 <= (unsigned char)(v9 - 0x30U)) goto label_40028d;
            v8 = &v8[1];
            v2 = (v9 - 0x30U) + v2 * '\b';
          } while (string != v8);
          *v7 = v2;
          v7 = &v7[1];
          v9 = *string;
          break;
        default:
          *v7 = '\\';
          v8 = &v7[2];
          v6 = &string[2];
          v7[1] = string[1];
          v9 = *v6;
          string = v6;
          v7 = v8;
          break;
        case 0x61:
          *v7 = '\a';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 0x62:
          *v7 = '\b';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 99:
          goto label_400210;
        case 0x66:
          *v7 = '\f';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 0x6e:
          *v7 = '\n';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 0x72:
          *v7 = '\r';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 0x74:
          *v7 = '\t';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 0x76:
          *v7 = '\v';
          string = &string[2];
          v9 = *string;
          v7 = &v7[1];
          break;
        case 0x78:
          v8 = &string[2];
          v10 = 0;
          v3 = 0;
          v11 = *(long *)__ctype_b_loc();
          do {
            v9 = *v8;
            v1 = to_uchar(v9);
            v2 = (char)v3;
            if (!(*(unsigned char *)(v11 + 1 + (unsigned long)v1 * 2) & 0x10)) {
              if (!v10) {
                v6 = &v7[2];
                v7[0] = '\\';
                v7[1] = 'x';
                v9 = *v8;
                string = v8;
                v7 = v6;
                goto label_4001bf;
              }
              break;
            }
            if (6 <= (unsigned char)(v9 + 0x9fU)) { // branch-flip
              v5 = v4 + -0x30;
              if ((unsigned char)(v9 + 0xbfU) <= 5)
                v5 = v4 + -0x37;
            }
            else {
              v5 = v4 + -0x57;
            }
            v10 += 1;
            v3 = v3 * 0x10 + v5;
            v2 = (char)v3;
            v8 = &v8[1];
          } while (v10 != 3);
label_40028d:
          *v7 = v2;
          v7 = &v7[1];
          v9 = *v8;
          string = v8;
        
      }
    }
    goto label_4001bf;
  }
label_400219:
  *v7 = '\0';
  return;
label_400210:
  do {
    v6 = &v6[1];
  } while (*v6);
  goto label_400219;
}

// Function: compile_regex @ 0x4003a0
void compile_regex(regex_data *regex)
{
  char *v1;
  char *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  
  v1 = regex->string;
  (regex->pattern).fastmap = regex->fastmap;
  v2 = folded_chars;
  if (!ignore_case)
    v2 = NULL;
  (regex->pattern).buffer = NULL;
  (regex->pattern).allocated = 0;
  (regex->pattern).translate = v2;
  v3 = re_compile_pattern(v1,strlen(v1),&regex->pattern);
  if (!v3) {
    re_compile_fastmap(&regex->pattern); // tail-call
    return;
  }
  v4 = quote(v1);
  error(1,0,dcgettext(0,"%s (for regexp %s)",5),v3,v4); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sort_found_occurs @ 0x400440
void sort_found_occurs(void)
{
  if (!number_of_occurs)
    return;
  qsort(occurs_table,number_of_occurs,0x30,compare_occurs); // tail-call
}

// Function: swallow_file_in_memory @ 0x400470
void swallow_file_in_memory(char *file_name,BLOCK *block)
{
  unsigned long used_length; // stack - 0x28
  char *v1;
  unsigned long v2; // rax
  
  if (((file_name) && (*file_name)) && ((*file_name != '-' || (file_name[1])))) {
    v1 = (char *)read_file(file_name,0,&used_length);
    block->start = v1;
    if (!v1) {
label_400517:
      v2 = quotearg_n_style_colon(0,3,file_name);
      error(1,*(unsigned int *)__errno_location(),"%s",v2);
      __stack_chk_fail(); // no-return
    }
  }
  else {
    v1 = (char *)fread_file(dat_407080,0,&used_length);
    block->start = v1;
    if (!v1) {
      file_name = "-";
      goto label_400517;
    }
    clearerr_unlocked(dat_407080);
    v1 = block->start;
  }
  block->end = &v1[used_length];
}

// Function: digest_word_file @ 0x400550
void digest_word_file(char *file_name,WORD_TABLE *table)
{
  BLOCK file_contents; // stack - 0x48
  WORD *v1;
  char *v2;
  unsigned long v3;
  unsigned long v4;
  
  swallow_file_in_memory(file_name,&file_contents);
  v1 = NULL;
  table->start = NULL;
  table->alloc = 0;
  table->length = 0;
  v3 = 0;
  while (v4 = v3, file_contents.start < file_contents.end) {
    v2 = file_contents.start;
    do {
      if (*v2 == '\n') break;
      v2 = &v2[1];
    } while (v2 < file_contents.end);
    if (file_contents.start < v2) {
      if (table->alloc == v3) {
        v1 = (WORD *)x2nrealloc(v1,&table->alloc,0x10);
        v3 = table->length;
        table->start = v1;
      }
      v4 = v3 + 1;
      v1[v3].start = file_contents.start;
      v1[v3].size = (long)v2 - (long)file_contents.start;
      table->length = v4;
    }
    if (file_contents.end <= v2) break;
    file_contents.start = &v2[1];
    v3 = v4;
  }
  qsort(v1,v4,0x10,compare_words); // tail-call
}

// Function: fix_output_parameters @ 0x400660
void fix_output_parameters(void) // return-dupe x2
{
  char ch;
  unsigned char v1; // al
  int v2; // eax
  char *v3; // rcx
  long v4; // rdx
  long v5;
  char v6 [24];
  long v7;
  unsigned long v8;
  unsigned long v9; // r14
  
  if (auto_reference) { // branch-flip
    reference_max_width = 0;
    if (number_input_files) {
      v5 = *file_line_count + 1;
      v8 = 0;
      while( true ) {
        v2 = __sprintf_chk(v6,1,0x15,"%ld",v5);
        v5 = (long)v2;
        if (*(long *)(input_file_name + v8 * 8))
          v5 = (long)v2 + strlen();
        if (reference_max_width < v5)
          reference_max_width = v5;
        v9 = v8 + 1;
        if ((unsigned long)(long)number_input_files <= v9) break;
        v5 = file_line_count[v8 + 1] + 1;
        if (v9)
          v5 -= file_line_count[v8];
        v8 = v9;
      }
    }
    v5 = reference_max_width + 1;
    v7 = reference_max_width + 2;
    reference_max_width = v5;
    reference._0_8_ = xmalloc(v7);
    if (!auto_reference) goto label_400689;
  }
  else {
label_400689:
    if (!input_reference) goto label_400696;
  }
  if (!right_reference)
    line_width -= gap_size + reference_max_width;
label_400696:
  if (line_width <= -1)
    line_width = 0;
  v5 = line_width / 2;
  v7 = v5 - gap_size;
  before_max_width = v7;
  half_line_width = v5;
  if ((truncation_string) && (*truncation_string)) {
    keyafter_max_width = v5;
    truncation_string_length = strlen();
  }
  else {
    truncation_string = NULL;
  }
  if (gnu_extensions) { // branch-flip
    before_max_width = v7 + truncation_string_length * -2;
    if (before_max_width <= -1)
      before_max_width = 0;
    keyafter_max_width = truncation_string_length * -2;
  }
  else {
    keyafter_max_width = -(truncation_string_length * 2 + 1);
  }
  keyafter_max_width = v5 + keyafter_max_width;
  v7 = 0;
  v3 = edited_flag;
  v5 = *(long *)__ctype_b_loc();
  do {
    edited_flag[v7] = (unsigned char)((unsigned short)*(unsigned short *)(v5 + v7 * 2) >> 0xd) & 1;
    v7 += 1;
  } while (v7 != 0x100);
  edited_flag[0xc] = 1;
  if (output_format != 2) {
    if (output_format != 3) {
      edited_flag[0xc] = 1;
      return;
    }
    ch = '$';
    do {
      v1 = to_uchar(ch);
      ch = *(char *)(v4 + 1);
      v3[v1] = 1;
    } while (ch);
    return;
  }
  edited_flag[0xc] = 1;
  edited_flag[0x22] = 1;
}

// Function: define_all_fields @ 0x400910
void define_all_fields(OCCURS *occurs)
{
  char ch;
  char *v1;
  char v10; // dl
  char v11; // dl
  char v12; // dl
  char v13; // dl
  char *v14;
  char *v15; // rdx
  char *v16; // rdx
  long v17; // rdx
  long v18;
  char *v19;
  int v2;
  char *v20; // rdx
  long v21; // rdx
  long v22; // rdx
  long v23; // rdx
  char *v24; // rdx
  char *v25; // rdx
  long v26; // rdx
  long v27; // rdx
  long v28; // rdx
  char *v29;
  unsigned long *v3; // rax
  char *v30;
  char v31;
  char *v32;
  long v33;
  char *v34;
  char *v35;
  char *v36;
  bool v37; // al
  char *v38; // stack - 0x98
  char *v39; // stack - 0x90
  char *v4;
  char *v40; // stack - 0x80
  char *v41; // stack - 0x60
  char *v42; // stack - 0x58
  char *v43; // stack - 0x50
  char *v44; // stack - 0x48
  char *v45; // stack - 0x40
  char v5;
  char *v6; // rcx
  char *v7; // rcx
  char *v8; // rcx
  char v9; // dl
  
  v30 = (occurs->key).start;
  v39 = (char *)occurs->left;
  v6 = &v39[(long)v30];
  v35 = &v30[(occurs->key).size];
  v3 = (unsigned long *)((long)occurs->file_index * 0x10 + text_buffers);
  v14 = &v30[keyafter_max_width];
  v1 = (char *)*v3;
  v4 = (char *)v3[1];
  v7 = &v30[occurs->right];
  keyafter._0_8_ = v30;
  keyafter._8_8_ = v35;
  if (v35 < v7) {
    v32 = word_fastmap;
    v19 = v14;
    do {
      v14 = v19;
      if (v14 < v35) {
        v39 = (char *)occurs->left;
        v35 = keyafter._8_8_;
        goto label_400a78;
      }
      keyafter._8_8_ = v35;
      if (word_regex._0_8_) { // branch-flip
        v2 = re_match(0x403268,v35,(int)v7 - (int)v35,0,0);
        if (v2 == -2) goto label_401578;
        if (v2 == -1)
          v2 = 1;
        v14 = &keyafter._0_8_[keyafter_max_width];
        v35 = &v35[v2];
        v19 = v14;
        v30 = keyafter._0_8_;
      }
      else {
        v5 = *v35;
        if (word_fastmap[(unsigned char)to_uchar(v5)]) { // branch-flip
          if (v7 <= v35) {
            v39 = (char *)occurs->left;
            goto label_400a71;
          }
          do {
            v35 = &v35[1];
            if (v7 == v35) goto label_400a54;
            v5 = *v35;
            v19 = v16;
          } while (word_fastmap[(unsigned char)to_uchar(v5)]);
        }
        else {
          v35 = &v35[1];
          v19 = v15;
        }
      }
    } while (v35 < v7);
label_400a54:
    v39 = (char *)occurs->left;
  }
  if (v35 <= v14) { // branch-flip
label_400a71:
    keyafter._8_8_ = v35;
  }
  else {
    v35 = keyafter._8_8_;
  }
label_400a78:
  keyafter_truncation = truncation_string && v35 < v7;
  if (v30 < v35) {
    __ctype_b_loc();
    v5 = '\0';
    do {
      v35 = &v35[-1];
      v31 = *v35;
      if (!(*(unsigned char *)(v17 + 1 + (unsigned long)(unsigned char)to_uchar(v31) * 2) & 0x20)) {
        if (v5)
          keyafter._8_8_ = v34;
        goto label_400ae9;
      }
      v5 = '\x01';
      v34 = v35;
    } while (v30 != v35);
    keyafter._8_8_ = v30;
  }
label_400ae9:
  v18 = maximum_word_length + half_line_width;
  if ((char *)-v18 != v39 && v18 <= -(long)v39) { // branch-flip
    v35 = &v30[-v18];
    if (word_regex._0_8_) { // branch-flip
      v2 = re_match(0x403268,v35,v18,0,0);
      v32 = v35;
      if (v2 == -2) goto label_401578;
      if (v2 == -1)
        v2 = 1;
      v35 = &v35[v2];
    }
    else {
      v5 = *v35;
      if (*(char *)(v27 + (unsigned long)(unsigned char)to_uchar(v5))) { // branch-flip
        if (v35 < v30) {
          do {
            v35 = &v35[1];
            if (v35 == v30) break;
            v5 = *v35;
          } while (*(char *)(v28 + (unsigned long)(unsigned char)to_uchar(v5)));
        }
      }
      else {
        v35 = &v35[1];
      }
    }
  }
  else {
    v35 = &v39[(long)v30];
  }
  v30 = keyafter._0_8_;
  before._8_8_ = keyafter._0_8_;
  before._0_8_ = v35;
  if (v35 < keyafter._0_8_) {
    v18 = *(long *)__ctype_b_loc();
    v14 = v30;
    do {
      v14 = &v14[-1];
      v5 = *v14;
      if (!(*(unsigned char *)(v18 + 1 + (unsigned long)(unsigned char)to_uchar(v5) * 2) & 0x20)) {
        if (v9) {
          v30 = v29;
          before._8_8_ = v29;
        }
        goto label_400bbb;
      }
      v29 = v14;
    } while (v35 != v14);
    v30 = v14;
    before._8_8_ = v35;
  }
label_400bbb:
  v18 = before_max_width;
  v14 = v35;
  if (&v35[before_max_width] < v30) {
    v19 = v40;
label_400c39:
    do {
      if (word_regex._0_8_) { // branch-flip
        v2 = re_match(0x403268);
        v32 = v35;
        if (v2 == -2) goto label_401578;
        if (v2 == -1)
          v2 = 1;
        before._0_8_ = &before._0_8_[v2];
        v14 = before._0_8_;
        v18 = before_max_width;
      }
      else {
        v5 = *v14;
        v8 = word_fastmap;
        if (!v8[(unsigned char)to_uchar(v5)]) {
          v14 = &v14[1];
          v30 = before._8_8_;
          before._0_8_ = v14;
          if (&v14[v18] >= before._8_8_) break;
          goto label_400c39;
        }
        v5 = '\0';
        v14 = before._0_8_;
        v32 = before._0_8_;
        if (before._0_8_ < v30) {
          while (v31 = *v14, word_fastmap[(unsigned char)to_uchar(v31)]) {
            v14 = (char *)(v21 + 1);
            v5 = '\x01';
            if (v14 == v30) {
              before._0_8_ = v14;
              goto label_400c29;
            }
            v19 = v14;
          }
          v14 = v32;
          if (v5) {
            v14 = v19;
            before._0_8_ = v19;
          }
        }
      }
label_400c29:
      v30 = before._8_8_;
    } while (before._8_8_ > &v14[v18]);
    v30 = before._8_8_;
  }
  v37 = 0;
  if (truncation_string) {
    if (v14 <= v1) // branch-flip
      v19 = v14;
    else {
      v33 = *(long *)__ctype_b_loc();
      v19 = v14;
      v32 = v1;
      do {
        v5 = v19[-1];
        v19 = v20;
        if (!(*(unsigned char *)(v33 + 1 + (unsigned long)(unsigned char)to_uchar(v5) * 2) & 0x20)) break;
        v19 = &v20[-1];
      } while (v32 != v19);
    }
    v37 = v6 < v19;
  }
  before_truncation = v37;
  if (v14 < v4) {
    v33 = *(long *)__ctype_b_loc();
    v19 = v4;
    do {
      v5 = *v14;
      if (!(*(unsigned char *)(v33 + 1 + (unsigned long)(unsigned char)to_uchar(v5) * 2) & 0x20)) {
        if (v10)
          before._0_8_ = v41;
        goto label_400d23;
      }
      v41 = &v14[1];
      v14 = v41;
    } while (v41 != v19);
    before._0_8_ = v41;
  }
label_400d23:
  v19 = keyafter._8_8_;
  v40 = gap_size;
  v18 = (v18 - ((long)v30 - (long)v14)) - (long)gap_size;
  v39 = keyafter._8_8_;
  if (v18 <= 0) {
    tail._0_8_ = NULL;
    tail._8_8_ = NULL;
    goto label_400ea6;
  }
  tail._0_8_ = keyafter._8_8_;
  if (v4 <= keyafter._8_8_) { // branch-flip
    v38 = keyafter._8_8_;
    v32 = keyafter._8_8_;
  }
  else {
    v33 = *(long *)__ctype_b_loc();
    v32 = v19;
    do {
      v5 = *v32;
      if (!(*(unsigned char *)(v33 + 1 + (unsigned long)(unsigned char)to_uchar(v5) * 2) & 0x20)) {
        if (v11) // branch-flip
          tail._0_8_ = v38;
        else {
          v38 = v19;
        }
        goto label_40127f;
      }
      v38 = &v32[1];
      v32 = v38;
    } while (v38 != v4);
    tail._0_8_ = v38;
  }
label_40127f:
  v30 = &v38[v18];
  tail._8_8_ = v32;
  if (v32 < v7) {
    v4 = v30;
    v14 = v38;
    do {
      v30 = v4;
      if (v30 <= v32) {
        v39 = keyafter._8_8_;
        v40 = gap_size;
        v32 = tail._8_8_;
        v38 = v14;
        goto label_4013a0;
      }
      tail._8_8_ = v32;
      if (word_regex._0_8_) { // branch-flip
        v2 = re_match(0x403268,v32,(int)v7 - (int)v32,0,0);
        v39 = v35;
        if (v2 == -2) goto label_401578;
        if (v2 == -1)
          v2 = 1;
        v30 = &tail._0_8_[v18];
        v32 = &v32[v2];
        v4 = v30;
        v14 = tail._0_8_;
      }
      else {
        v5 = *v32;
        if (word_fastmap[(unsigned char)to_uchar(v5)]) { // branch-flip
          if (v7 <= v32) {
            v39 = keyafter._8_8_;
            v38 = v14;
            v40 = gap_size;
            goto label_401399;
          }
          do {
            v32 = &v32[1];
            if (v7 == v32) goto label_40136b;
            v5 = *v32;
            v4 = v25;
          } while (word_fastmap[(unsigned char)to_uchar(v5)]);
        }
        else {
          v32 = &v32[1];
          v4 = v24;
        }
      }
    } while (v32 < v7);
label_40136b:
    v39 = keyafter._8_8_;
    v40 = gap_size;
    v38 = v14;
  }
  if (v30 <= v32) goto label_40157d;
label_401399:
  tail._8_8_ = v32;
label_4013a0:
  do {
    if (v32 <= v38) { // branch-flip
label_400ea6:
      tail_truncation = 0;
    }
    else {
      keyafter_truncation = 0;
      tail_truncation = truncation_string && v32 < v7;
      __ctype_b_loc();
      v5 = '\0';
      v30 = v44;
      v4 = v38;
      while( true ) {
        v32 = &v32[-1];
        v31 = *v32;
        v14 = v32;
        if (!(*(unsigned char *)(v26 + 1 + (unsigned long)(unsigned char)to_uchar(v31) * 2) & 0x20)) break;
        v5 = '\x01';
        if (v4 == v32) {
          tail._8_8_ = v38;
          goto label_400ead;
        }
        v30 = v14;
      }
      if (v5)
        tail._8_8_ = v30;
    }
label_400ead:
    v30 = before._0_8_;
    v18 = (keyafter_max_width - ((long)v39 - (long)keyafter._0_8_)) - (long)v40;
    if (v18 <= 0) {
      head._0_8_ = NULL;
      head._8_8_ = NULL;
      goto label_401003;
    }
    head._8_8_ = before._0_8_;
    if (v1 < before._0_8_) {
      v33 = *(long *)__ctype_b_loc();
      v14 = v1;
      v4 = v30;
      do {
        v4 = &v4[-1];
        v5 = *v4;
        if (!(*(unsigned char *)(v33 + 1 + (unsigned long)(unsigned char)to_uchar(v5) * 2) & 0x20)) {
          if (v12) {
            v30 = v36;
            head._8_8_ = v36;
          }
          goto label_400f43;
        }
        v36 = v4;
      } while (v14 != v4);
      v30 = v4;
      head._8_8_ = v4;
    }
label_400f43:
    head._0_8_ = v35;
    if (v30 <= &v35[v18]) goto label_400ffa;
    v32 = v42;
    v36 = v30;
label_400fbf:
    while (!word_regex._0_8_) {
      v5 = *v35;
      if (word_fastmap[(unsigned char)to_uchar(v5)]) {
        v5 = '\0';
        v35 = head._0_8_;
        v30 = head._0_8_;
        if (head._0_8_ < v36) {
          while (v31 = *v35, word_fastmap[(unsigned char)to_uchar(v31)]) {
            v35 = (char *)(v22 + 1);
            v5 = '\x01';
            if (v35 == v36) {
              head._0_8_ = v35;
              goto label_400faf;
            }
            v32 = v35;
          }
          v35 = v30;
          if (v5) {
            v35 = v32;
            head._0_8_ = v32;
          }
        }
        goto label_400faf;
      }
      v35 = &v35[1];
      v36 = head._8_8_;
      head._0_8_ = v35;
      if (head._8_8_ <= &v35[v18]) {
label_400ff7:
        v30 = head._8_8_;
        goto label_400ffa;
      }
    }
    v2 = re_match(0x403268);
    if (v2 != -2) {
      if (v2 == -1)
        v2 = 1;
      head._0_8_ = &head._0_8_[v2];
      v35 = head._0_8_;
label_400faf:
      v36 = head._8_8_;
      if (head._8_8_ <= &v35[v18]) goto label_400ff7;
      goto label_400fbf;
    }
label_401578:
    matcher_error();
    v35 = v32;
label_40157d:
    v32 = tail._8_8_;
  } while( true );
label_400ffa:
  if (v30 <= v35) { // branch-flip
label_401003:
    head_truncation = 0;
  }
  else {
    before_truncation = 0;
    head_truncation = truncation_string && v6 < v35;
    v18 = *(long *)__ctype_b_loc();
    while (v5 = *v35, *(unsigned char *)(v18 + 1 + (unsigned long)(unsigned char)to_uchar(v5) * 2) & 0x20) {
      v35 = &v35[1];
      if (v35 == v30) {
        head._0_8_ = v35;
        goto label_40100a;
      }
      v43 = v35;
    }
    v5 = input_reference;
    if (v13) goto label_401199;
  }
label_40100a:
  v43 = head._0_8_;
  v5 = input_reference;
label_401199:
  head._0_8_ = v43;
  input_reference = v5;
  if (auto_reference) { // branch-flip
    v2 = occurs->file_index;
    v18 = *(long *)(input_file_name + (long)v2 * 8);
    if (!v18)
      v18 = 0x404025;
    v33 = occurs->reference + 1;
    if (1 <= v2)
      v33 -= *(long *)(file_line_count + -8 + (long)v2 * 8);
    v18 = stpcpy(reference._0_8_,v18);
    reference._8_8_ = (char *)(v18 + (int)__sprintf_chk(v18,1,0xffffffffffffffff,":%ld",v33));
  }
  else if (v5) {
    v30 = &keyafter._0_8_[occurs->reference];
    reference._0_8_ = v30;
    reference._8_8_ = v30;
    if (v30 < v7) {
      __ctype_b_loc();
      v31 = '\0';
      while (ch = *v30, !(*(unsigned char *)(v23 + 1 + (unsigned long)(unsigned char)to_uchar(ch) * 2) & 0x20)) {
        v30 = &v30[1];
        if (v30 == v7) {
          reference._8_8_ = v30;
          return;
        }
        v45 = v30;
        v31 = v5;
      }
      if (v31)
        reference._8_8_ = v45;
    }
  }
}

// Function: print_spaces @ 0x401590
void print_spaces(long number)
{
  if (1 > number)
    return;
  do {
    putchar_unlocked(0x20);
    number -= 1;
  } while (number);
}

// Function: print_field @ 0x4015c0
void print_field(BLOCK field)
{
  unsigned char v1;
  
  field.start = (unsigned char *)field.start;
  if (field.end <= field.start)
    return;
  do {
    v1 = *field.start;
    if (edited_flag[v1]) { // branch-flip
      if (v1 != 0x5c) { // branch-flip
        if (']' <= (char)v1) { // branch-flip
          if (v1 + 0x85 & 0xfd) { // branch-flip
            if (v1 != 0x5f) goto label_401681;
label_401649:
            putchar_unlocked(0x5c);
            putchar_unlocked(v1);
          }
          else {
            __printf_chk(1,"$\\%c$",v1);
          }
        }
        else if (v1 != 0x22) { // branch-flip
          if (4 > (unsigned char)(v1 - 0x23)) goto label_401649;
label_401681:
          putchar_unlocked(0x20);
        }
        else {
          putchar_unlocked(0x22);
          putchar_unlocked(0x22);
        }
      }
      else {
        fwrite_unlocked("\\backslash{}",1,0xc,dat_407000);
      }
    }
    else {
      putchar_unlocked((int)(char)v1);
    }
    field.start = &field.start[1];
    if ((unsigned char *)field.end == field.start)
      return;
  } while( true );
}

// Function: output_one_dumb_line @ 0x4016f0
void output_one_dumb_line(void)
{
  BLOCK field;
  char *v1;
  char v2;
  char *v3;
  long v4; // rdi
  
  if (!right_reference) {
    if (auto_reference) { // branch-flip
      field.end = reference._8_8_;
      field.start = reference._0_8_;
      print_field(field);
      putchar_unlocked(0x3a);
      print_spaces(((gap_size + reference_max_width) - ((long)reference._8_8_ - (long)reference._0_8_)) + -1);
    }
    else {
      field.end = reference._8_8_;
      field.start = reference._0_8_;
      print_field(field);
      print_spaces((gap_size + reference_max_width) - ((long)reference._8_8_ - (long)reference._0_8_));
    }
  }
  if (tail._8_8_ <= tail._0_8_) { // branch-flip
    v4 = (half_line_width - gap_size) - ((long)before._8_8_ - (long)before._0_8_);
    if (!before_truncation) goto label_40178f;
label_4018de:
    print_spaces(v4 - truncation_string_length);
  }
  else {
    field.end = tail._8_8_;
    field.start = tail._0_8_;
    print_field(field);
    if (tail_truncation) { // branch-flip
      fputs_unlocked(truncation_string,dat_407000);
      v4 = (half_line_width - gap_size) - ((long)before._8_8_ - (long)before._0_8_);
      v1 = tail._8_8_;
      v2 = tail_truncation;
      if (before_truncation) goto label_4018c5;
    }
    else {
      v4 = (half_line_width - gap_size) - ((long)before._8_8_ - (long)before._0_8_);
      v2 = tail_truncation;
      if (!before_truncation) {
        v4 -= (long)tail._8_8_ - (long)tail._0_8_;
        goto label_40178f;
      }
label_4018c5:
      v4 -= truncation_string_length;
      v1 = tail._8_8_;
    }
    v4 -= (long)v1 - (long)tail._0_8_;
    if (v2) goto label_4018de;
label_40178f:
    print_spaces(v4);
  }
  if (before_truncation)
    fputs_unlocked(truncation_string,dat_407000);
  field.end = before._8_8_;
  field.start = before._0_8_;
  print_field(field);
  print_spaces(gap_size);
  field.end = keyafter._8_8_;
  field.start = keyafter._0_8_;
  print_field(field);
  if (keyafter_truncation) { // branch-flip
    fputs_unlocked(truncation_string,dat_407000);
    if (head._0_8_ < head._8_8_) {
      v4 = half_line_width - ((long)keyafter._8_8_ - (long)keyafter._0_8_);
      v1 = head._8_8_;
      v3 = head._0_8_;
      if (keyafter_truncation)
        v4 -= truncation_string_length;
      goto label_401948;
    }
label_4017f7:
    if (auto_reference) {
      if (!right_reference) {
        putchar_unlocked(10); // return-dupe, tail-call
        return;
      }
label_401811:
      v4 = half_line_width - ((long)keyafter._8_8_ - (long)keyafter._0_8_);
      if (keyafter_truncation)
        v4 -= truncation_string_length;
      print_spaces(v4);
      goto label_401845;
    }
    if (!input_reference) {
      putchar_unlocked(10);
      return;
    }
    if (right_reference) goto label_401811;
label_40184b:
    if (!input_reference) {
      putchar_unlocked(10); // tail-call
      return;
    }
  }
  else {
    if (head._8_8_ <= head._0_8_) goto label_4017f7;
    v4 = half_line_width - ((long)keyafter._8_8_ - (long)keyafter._0_8_);
    v1 = head._8_8_;
    v3 = head._0_8_;
label_401948:
    v4 -= (long)v1 - (long)v3;
    if (head_truncation)
      v4 -= truncation_string_length;
    print_spaces(v4);
    if (head_truncation)
      fputs_unlocked(truncation_string,dat_407000);
    field.end = head._8_8_;
    field.start = head._0_8_;
    print_field(field);
label_401845:
    if (!auto_reference) goto label_40184b;
  }
  if (right_reference) {
    print_spaces(gap_size);
    field.end = reference._8_8_;
    field.start = reference._0_8_;
    print_field(field);
  }
  putchar_unlocked(10);
  return;
}

// Function: output_one_roff_line @ 0x401af0
void output_one_roff_line(void)
{
  BLOCK field;
  
  __printf_chk(1,".%s \"",macro_name);
  field.end = tail._8_8_;
  field.start = tail._0_8_;
  print_field(field);
  if (tail_truncation)
    fputs_unlocked(truncation_string,dat_407000);
  putchar_unlocked(0x22);
  fwrite_unlocked(" \"",1,2,dat_407000);
  if (before_truncation)
    fputs_unlocked(truncation_string,dat_407000);
  field.end = before._8_8_;
  field.start = before._0_8_;
  print_field(field);
  putchar_unlocked(0x22);
  fwrite_unlocked(" \"",1,2,dat_407000);
  field.end = keyafter._8_8_;
  field.start = keyafter._0_8_;
  print_field(field);
  if (keyafter_truncation)
    fputs_unlocked(truncation_string,dat_407000);
  putchar_unlocked(0x22);
  fwrite_unlocked(" \"",1,2,dat_407000);
  if (head_truncation)
    fputs_unlocked(truncation_string,dat_407000);
  field.end = head._8_8_;
  field.start = head._0_8_;
  print_field(field);
  putchar_unlocked(0x22);
  if ((auto_reference) || (input_reference)) {
    fwrite_unlocked(" \"",1,2,dat_407000);
    field.end = reference._8_8_;
    field.start = reference._0_8_;
    print_field(field);
    putchar_unlocked(0x22);
  }
  putchar_unlocked(10); // tail-call
}

// Function: output_one_tex_line @ 0x401ce0
void output_one_tex_line(void)
{
  BLOCK field;
  char v1;
  char *v2;
  char *v3;
  int v4; // eax
  long v5; // rdx
  long v6; // rdx
  char *v7;
  
  __printf_chk(1,"\\%s ",macro_name);
  putchar_unlocked(0x7b);
  field.end = tail._8_8_;
  field.start = tail._0_8_;
  print_field(field);
  fwrite_unlocked("}{",1,2,dat_407000);
  field.end = before._8_8_;
  field.start = before._0_8_;
  print_field(field);
  fwrite_unlocked("}{",1,2,dat_407000);
  v3 = keyafter._8_8_;
  v2 = keyafter._0_8_;
  if (word_regex._0_8_) { // branch-flip
    v4 = re_match(0x403268,keyafter._0_8_,(int)keyafter._8_8_ - (int)keyafter._0_8_,0,0);
    if (v4 == -2)
      matcher_error();
    if (v4 == -1) // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
      v4 = 1;
    v7 = &v2[v4];
  }
  else {
    v1 = *keyafter._0_8_;
    if (*(char *)(v5 + (unsigned long)(unsigned char)to_uchar(v1))) { // branch-flip
      v7 = v2;
      if (v2 < v3) {
        do {
          v7 = &v7[1];
          if (v3 == v7) break;
          v1 = *v7;
        } while (*(char *)(v6 + (unsigned long)(unsigned char)to_uchar(v1)));
      }
    }
    else {
      v7 = &v2[1];
    }
  }
  field.end = v7;
  field.start = v2;
  print_field(field);
  fwrite_unlocked("}{",1,2,dat_407000);
  field.end = v3;
  field.start = v7;
  print_field(field);
  fwrite_unlocked("}{",1,2,dat_407000);
  field.end = head._8_8_;
  field.start = head._0_8_;
  print_field(field);
  putchar_unlocked(0x7d);
  if ((auto_reference) || (input_reference)) {
    putchar_unlocked(0x7b);
    field.end = reference._8_8_;
    field.start = reference._0_8_;
    print_field(field);
    putchar_unlocked(0x7d);
  }
  putchar_unlocked(10); // tail-call
}

// Function: generate_all_output @ 0x401ee0
void generate_all_output(void) // return-dupe
{
  OCCURS *occurs;
  long v1; // rbp
  
  tail._0_8_ = 0;
  tail_truncation = 0;
  tail._8_8_ = 0;
  head._0_8_ = 0;
  head._8_8_ = 0;
  head_truncation = 0;
  if (number_of_occurs <= 0) {
    tail._8_8_ = 0;
    tail._0_8_ = 0;
    tail_truncation = 0;
    head._8_8_ = 0;
    head._0_8_ = 0;
    head_truncation = 0;
    return;
  }
  v1 = 0;
  occurs = occurs_table;
  do {
    define_all_fields(occurs);
    if (output_format != 2) { // branch-flip
      if (3 <= output_format) { // branch-flip
        if (output_format == 3)
          output_one_tex_line();
      }
      else {
        output_one_dumb_line();
      }
    }
    else {
      output_one_roff_line();
    }
    occurs = &occurs[1];
    v1 += 1;
  } while (v1 < number_of_occurs);
}

// Function: initialize_regex @ 0x401f90
void initialize_regex(void) // return-dupe x2
{
  unsigned short *v1;
  unsigned long *v2;
  long *v3; // rax
  unsigned char *v4;
  unsigned char *v5; // rcx
  long v6; // rcx
  unsigned long v7; // rbx
  unsigned long *v8;
  
  if (ignore_case) {
    v7 = 0;
    do {
      folded_chars[v7] = toupper(v7 & 0xffffffff);
      v7 += 1;
    } while (v7 != 0x100);
  }
  if (context_regex._0_8_) { // branch-flip
    if (!*context_regex._0_8_) {
      context_regex._0_8_ = NULL;
      goto label_401fe5;
    }
  }
  else if ((gnu_extensions) && (!input_reference))
    context_regex._0_8_ = "[.?!][]\"\')}]*\\($\\|\t\\|  \\)[ \t\n]*";
  else {
    context_regex._0_8_ = "\n";
  }
  compile_regex((regex_data *)context_regex);
label_401fe5:
  if (word_regex._0_8_) {
    compile_regex((regex_data *)word_regex); // tail-call
    return;
  }
  if (break_file)
    return;
  if (gnu_extensions) {
    v3 = (long *)__ctype_b_loc();
    v6 = 0;
    v4 = word_fastmap;
    do {
      v5 = &v4[1];
      v1 = (unsigned short *)(*v3 + v6);
      v6 += 2;
      *v4 = (unsigned char)((unsigned short)*v1 >> 10) & 1;
      v4 = v5;
    } while (v6 != 0x200);
    return;
  }
  v6 = 0x20;
  v8 = (unsigned long *)word_fastmap;
  while (v6) {
    v2 = &v8[1];
    *v8 = 0x101010101010101;
    v6 -= 1;
    v8 = v2;
  }
  word_fastmap._9_2_ = 0;
  word_fastmap[0x20] = 0;
}

// Function: digest_break_file @ 0x4020c0
void digest_break_file(char *file_name)
{
  char ch;
  BLOCK file_contents; // stack - 0x28
  char *v1;
  char *v2; // rdx
  char *v3; // rsi
  char *v4;
  
  swallow_file_in_memory(file_name,&file_contents);
  word_fastmap._0_8_ = dat_404c50;
  word_fastmap._8_8_ = dat_404c58;
  word_fastmap._16_8_ = dat_404c50;
  word_fastmap._24_8_ = dat_404c58;
  word_fastmap._32_8_ = dat_404c50;
  word_fastmap._40_8_ = dat_404c58;
  word_fastmap._48_8_ = dat_404c50;
  word_fastmap._56_8_ = dat_404c58;
  word_fastmap._64_8_ = dat_404c50;
  word_fastmap._72_8_ = dat_404c58;
  word_fastmap._80_8_ = dat_404c50;
  word_fastmap._88_8_ = dat_404c58;
  word_fastmap._96_8_ = dat_404c50;
  word_fastmap._104_8_ = dat_404c58;
  word_fastmap._112_8_ = dat_404c50;
  word_fastmap._120_8_ = dat_404c58;
  word_fastmap._128_8_ = dat_404c50;
  word_fastmap._136_8_ = dat_404c58;
  word_fastmap._144_8_ = dat_404c50;
  word_fastmap._152_8_ = dat_404c58;
  word_fastmap._160_8_ = dat_404c50;
  word_fastmap._168_8_ = dat_404c58;
  word_fastmap._176_8_ = dat_404c50;
  word_fastmap._184_8_ = dat_404c58;
  word_fastmap._192_8_ = dat_404c50;
  word_fastmap._200_8_ = dat_404c58;
  word_fastmap._208_8_ = dat_404c50;
  word_fastmap._216_8_ = dat_404c58;
  word_fastmap._224_8_ = dat_404c50;
  word_fastmap._232_8_ = dat_404c58;
  word_fastmap._240_8_ = dat_404c50;
  word_fastmap._248_8_ = dat_404c58;
  v4 = file_contents.start;
  if (file_contents.start < file_contents.end) {
    v3 = word_fastmap;
    v1 = file_contents.end;
    do {
      ch = *file_contents.start;
      v3[(unsigned char)to_uchar(ch)] = 0;
      file_contents.start = v2;
    } while (v2 != v1);
  }
  if (!gnu_extensions) {
    word_fastmap._32_8_ &= 0xffffffffffffff00;
    word_fastmap._8_3_ = (undefined3)word_fastmap[8];
  }
  free(v4); // tail-call
}

// Function: emit_ancillary_info.constprop.0 @ 0x4021d0
void emit_ancillary_info.constprop.0(void)
{
  unsigned long v1;
  char *v10; // stack - 0x68
  char *v11; // stack - 0x60
  char *v12; // stack - 0x58
  char *v13; // stack - 0x50
  unsigned long v14; // stack - 0x48
  unsigned long v15; // stack - 0x40
  char *v16 [5]; // stack - 0x98
  char **v2;
  long v3; // rax
  char *v4; // rbx
  char *v5; // stack - 0xa8
  char *v6; // r12
  char *v7;
  char *v8; // stack - 0xa0
  char *v9; // stack - 0x70
  
  v16[4] = "sha256sum";
  v10 = "sha384sum";
  v12 = "sha512sum";
  v14 = 0;
  v15 = 0;
  v5 = "[";
  v8 = "test invocation";
  v16[0] = "coreutils";
  v16[1] = "Multi-call invocation";
  v16[2] = "sha224sum";
  v16[3] = "sha2 utilities";
  v9 = "sha2 utilities";
  v11 = "sha2 utilities";
  v13 = "sha2 utilities";
  v2 = &v5;
  do {
    v4 = (char *)v2;
    if (!*(long *)&v4[0x10]) break;
    v2 = (char **)&v4[0x10];
  } while (strcmp("ptx"));
  v7 = *(char **)&v4[0x18];
  if (v7) { // branch-flip
    __printf_chk(1,dcgettext(0,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,0);
    if ((v3) && (strncmp(v3,"en_",3))) goto label_4023e0;
  }
  else {
    __printf_chk(1,dcgettext(0,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,0);
    if ((!v3) || (!strncmp(v3,"en_",3))) {
      v7 = "ptx";
      __printf_chk(1,dcgettext(0,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/","ptx");
      v6 = " invocation";
      __printf_chk(1,dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v6); // return-dupe, tail-call
      return;
    }
    v7 = "ptx";
label_4023e0:
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  }
  __printf_chk(1,dcgettext(0,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/","ptx");
  v6 = " invocation";
  if (v7 != "ptx")
    v6 = "";
  __printf_chk(1,dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v6);
  return;
}

// Function: search_table.isra.0 @ 0x402450
unsigned long search_table.isra.0(void *a0,long a1,long a2)
{
  int v1; // eax
  long v2; // rbx
  long v3;
  
  a2 -= 1;
  if (a2 < 0)
    return 0;
  v3 = 0;
  do {
    while( true ) {
      v2 = v3 + a2 >> 1;
      v1 = compare_words(a0,(void *)(v2 * 0x10 + a1));
      if (0 <= v1) break;
      a2 = v2 + -1;
      if (a2 < v3)
        return 0;
    }
    if (!v1)
      return 1;
    v3 = v2 + 1;
  } while (v3 <= a2);
  return 0;
}

// Function: find_occurs_in_text @ 0x4024d0
void find_occurs_in_text(int file_index) // return-dupe
{
  char ch;
  WORD possible_key; // stack - 0x58
  unsigned long *v1;
  long v10;
  unsigned long v11; // rax
  char *v12; // rcx
  long v13; // rdx
  long v14; // rdx
  char *v15; // rdx
  long v16; // rdx
  long v17; // rdx
  char *v18;
  char *v19; // rdx
  unsigned short v2;
  char *v20;
  char *v21; // rdx
  long v22; // rdx
  long v23; // rdx
  char *v24;
  char *v25;
  char *v26;
  char *v27;
  char *v28;
  char *v29; // stack - 0xa8
  long v3;
  char *v30; // stack - 0xa0
  long v31; // stack - 0x98
  long v32; // stack - 0x90
  long v33; // stack - 0x80
  char v34; // stack - 0x75
  long v4;
  char *v5;
  char v6;
  char v7;
  int v8; // eax
  unsigned long *v9; // rax
  
  v9 = (unsigned long *)((long)file_index * 0x10 + text_buffers);
  v20 = (char *)*v9;
  v24 = (char *)v9[1];
  v29 = v20;
  if (!input_reference) {
    if (v24 > v20) {
      v33 = 0;
      v5 = v20;
      v28 = v20;
      goto label_402546;
    }
    return;
  }
  if (v24 <= v20)
    return;
  __ctype_b_loc();
  v28 = v20;
  do {
    v6 = *v28;
    v2 = *(unsigned short *)(v16 + (unsigned long)(unsigned char)to_uchar(v6) * 2);
    if (v2 & 0x2000) {
      v33 = (long)v28 - (long)v20;
      v5 = v20;
      if (v28 >= v24) goto label_402546;
      do {
        if (!(v2 & 0x2000)) goto label_402546;
        v28 = &v28[1];
        if (v28 == v24) goto label_402546;
        v6 = *v28;
        v2 = *(unsigned short *)(v17 + (unsigned long)(unsigned char)to_uchar(v6) * 2);
      } while( true );
    }
    v28 = &v28[1];
  } while (v28 != v24);
label_402b97:
  v33 = (long)v28 - (long)v20;
  v29 = v20;
  v5 = v20;
label_402546:
  v20 = v28;
  v28 = v5;
  v5 = v24;
  if ((context_regex._0_8_) && (v8 = re_search(0x4033c8,v28,(long)v24 - (long)v28 & 0xffffffff,0,(long)v24 - (long)v28,context_regs), v8 != -1)) {
    if (!v8) {
      v11 = quote(context_regex._0_8_);
      error(1,0,dcgettext(0,"error: regular expression has a match of length zero: %s",5),v11);
      __stack_chk_fail(); // no-return
    }
    if (v8 == -2) goto label_402b92;
    v5 = &v28[*context_regs._16_8_];
  }
  v10 = word_regex._0_8_;
  v30 = v28;
  v26 = v5;
  if (v28 < v5) {
    __ctype_b_loc();
    do {
      v6 = v26[-1];
      v10 = word_regex._0_8_;
      if (!(*(unsigned char *)(v13 + 1 + (unsigned long)(unsigned char)to_uchar(v6) * 2) & 0x20)) goto label_4025e3;
      v26 = &v26[-1];
    } while (v26 != v28);
  }
label_4025e3:
  do {
    if (v10) {
      v8 = re_search(0x403268,v28,(long)v26 - (long)v28 & 0xffffffff,0,(long)v26 - (long)v28,word_regs);
      if (v8 == -2) goto label_402b92;
      if (v8 == -1) goto label_402aa8;
      v24 = &v28[*word_regs._8_8_];
      v28 = &v28[*word_regs._16_8_];
      if (v24 == v28) {
label_402800:
        v10 = word_regex._0_8_;
        goto label_40280a;
      }
label_40263f:
      v6 = input_reference;
      possible_key.size = (long)v28 - (long)v24;
      if (maximum_word_length < possible_key.size)
        maximum_word_length = possible_key.size;
      v34 = input_reference;
      possible_key.start = v24;
      if (input_reference) {
        if (v20 < v24) {
          v31 = total_line_count;
          v7 = '\0';
          do {
            v25 = &v20[1];
            if (*v20 != '\n') // branch-flip
              v20 = v25;
            else {
              v31 += 1;
              v27 = (char *)v9[1];
              if (v27 <= v25) { // branch-flip
                v33 = 0;
                v27 = v25;
              }
              else {
                v10 = *(long *)__ctype_b_loc();
                v18 = v25;
                do {
                  v7 = *v18;
                  if (*(unsigned char *)(v10 + 1 + (unsigned long)(unsigned char)to_uchar(v7) * 2) & 0x20) {
                    v33 = (long)v19 - (long)v25;
                    v27 = v19;
                    goto label_40295c;
                  }
                  v18 = &v19[1];
                } while (v18 != v27);
                v33 = (long)v19 - (long)v20;
              }
label_40295c:
              v20 = v27;
              v7 = v6;
              v29 = v25;
            }
          } while (v20 < v24);
          if (v7)
            total_line_count = v31;
        }
        v10 = word_regex._0_8_;
        if (v24 < v20) goto label_4025e3;
      }
      if (ignore_file) {
        v6 = search_table.isra.0(&possible_key,ignore_table._0_8_,ignore_table._16_8_);
        v10 = word_regex._0_8_;
        if (v6) goto label_4025e3;
      }
      if (only_file) {
        v6 = search_table.isra.0(&possible_key,only_table._0_8_,only_table._16_8_);
        v10 = word_regex._0_8_;
        if (!v6) goto label_4025e3;
      }
      v10 = occurs_table;
      if (number_of_occurs == occurs_alloc) {
        v10 = x2nrealloc(occurs_table,&occurs_alloc,0x30);
        v34 = input_reference;
        occurs_table = v10;
      }
      v6 = auto_reference;
      v4 = total_line_count;
      v3 = number_of_occurs;
      v1 = (unsigned long *)(v10 + number_of_occurs * 0x30);
      if (auto_reference) { // branch-flip
        if (v24 <= v20) { // branch-flip
label_402a98:
          v32 = v4;
        }
        else {
          v32 = total_line_count;
          v7 = '\0';
          do {
            while( true ) {
              v25 = &v20[1];
              if (*v20 == '\n') break;
label_4029b0:
              v20 = v25;
              if (v25 >= v24) goto label_402a20;
            }
            v32 += 1;
            v27 = (char *)v9[1];
            v29 = v25;
            v7 = v6;
            if (v25 >= v27) goto label_4029b0;
            v10 = *(long *)__ctype_b_loc();
            v20 = v25;
            do {
              ch = *v20;
              v20 = v21;
              if (*(unsigned char *)(v10 + 1 + (unsigned long)(unsigned char)to_uchar(ch) * 2) & 0x20) break;
              v20 = &v21[1];
            } while (v20 != v27);
          } while (v24 > v20);
label_402a20:
          if (!v7) goto label_402a98;
          total_line_count = v32;
        }
        v1[4] = v32;
label_402727:
        if (((v29 == v30) && (v34)) && (v26 > v30)) {
          __ctype_b_loc();
          do {
            v6 = *v30;
            if (*(unsigned char *)(v22 + 1 + (unsigned long)(unsigned char)to_uchar(v6) * 2) & 0x20) {
              if (v30 < v26) goto label_402b40;
              break;
            }
            v30 = &v30[1];
          } while (v30 != v26);
        }
      }
      else if (v34) {
        v1[4] = (long)v29 - (long)v24;
        if (reference_max_width < v33)
          reference_max_width = v33;
        goto label_402727;
      }
      goto label_40273d;
    }
    v12 = word_fastmap;
    v24 = v28;
    if (v26 <= v28) { // branch-flip
      if (v28 == v26) goto label_402aa8;
    }
    else {
      while (v6 = *v24, !v12[(unsigned char)to_uchar(v6)]) {
        v24 = &v24[1];
        if (v24 == v26) goto label_402aa8;
      }
      if (v24 == v26) goto label_402aa8;
      v10 = v14;
      if (v24 < v26) {
        v28 = v24;
        do {
          v6 = *v28;
          if (!v12[(unsigned char)to_uchar(v6)]) {
            v28 = v15;
            if (v24 != v15) goto label_40263f;
            goto label_402800;
          }
          v28 = &v15[1];
        } while (v28 != v26);
        v28 = v26;
        goto label_40263f;
      }
    }
label_40280a:
    v28 = &v24[1];
  } while( true );
label_402b92:
  matcher_error();
  goto label_402b97;
  while (v6 = *v30, *(unsigned char *)(v23 + 1 + (unsigned long)(unsigned char)to_uchar(v6) * 2) & 0x20) {
label_402b40:
    v30 = &v30[1];
    if (v30 == v26) break;
  }
label_40273d:
  *v1 = possible_key.start;
  v1[1] = possible_key.size;
  v1[2] = (long)v30 - (long)v24;
  v1[3] = (long)v26 - (long)v24;
  *(int *)&v1[5] = file_index;
  number_of_occurs = v3 + 1;
  v10 = word_regex._0_8_;
  goto label_4025e3;
label_402aa8:
  v24 = (char *)v9[1];
  v28 = v20;
  if (v24 <= v5)
    return;
  goto label_402546;
}

// Function: usage @ 0x402bf0
void usage(int status)
{
  unsigned long v1;
  
  v1 = dat_4071c0;
  if (status) // branch-flip
    __fprintf_chk(dat_4071d0,1,dcgettext(0,"Try \'%s --help\' for more information.\n",5),v1);
  else {
    __printf_chk(1,dcgettext(0,"Usage: %s [OPTION]... [INPUT]...   (without -G)\n  or:  %s -G [OPTION]... [INPUT [OUTPUT]]\n",5),v1,v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"Output a permuted index, including context, of the words in the input files.\n",5),v1);
    emit_stdin_note();
    emit_mandatory_arg_note();
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"  -A, --auto-reference           output automatically generated references\n  -G, --traditional              behave more like System V \'ptx\'\n",5),v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"  -F, --flag-truncation=STRING   use STRING for flagging line truncations.\n                                 The default is \'/\'\n",5),v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"  -M, --macro-name=STRING        macro name to use instead of \'xx\'\n  -O, --format=roff              generate output as roff directives\n  -R, --right-side-refs          put references at right, not counted in -w\n  -S, --sentence-regexp=REGEXP   for end of lines or end of sentences\n  -T, --format=tex               generate output as TeX directives\n",5),v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"  -W, --word-regexp=REGEXP       use REGEXP to match each keyword\n  -b, --break-file=FILE          word break characters in this FILE\n  -f, --ignore-case              fold lower case to upper case for sorting\n  -g, --gap-size=NUMBER          gap size in columns between output fields\n  -i, --ignore-file=FILE         read ignore word list from FILE\n  -o, --only-file=FILE           read only word list from this FILE\n",5),v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"  -r, --references               first field of each line is a reference\n  -t, --typeset-mode               - not implemented -\n  -w, --width=NUMBER             output width in columns, reference excluded\n",5),v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"      --help        display this help and exit\n",5),v1);
    v1 = dat_407000;
    fputs_unlocked(dcgettext(0,"      --version     output version information and exit\n",5),v1);
    emit_ancillary_info.constprop.0();
  }
  exit(status); // no-return
}

// Function: main @ 0x404280
int main(int argc,char **argv)
{
  unsigned long *v1;
  int v2;
  unsigned int v3; // edx
  long v4; // rax
  long v5; // stack - 0x48
  long v6; // rbp
  unsigned long v7; // rax
  char *v8;
  char *v9;
  
  v9 = long_options;
  v8 = "AF:GM:ORS:TW:b:i:fg:o:trw:";
  set_program_name(*argv);
  setlocale(6,0x404025);
  bindtextdomain("coreutils","/usr/local/share/locale");
  v7 = 0x403e3c;
  textdomain("coreutils");
  atexit(0xf2e6600002f5ae8);
label_404310:
  v2 = getopt_long(argc,argv,"AF:GM:ORS:TW:b:i:fg:o:trw:",long_options,0);
  if (v2 == -1) {
label_40457a:
    if (dat_4072f0 != argc) { // branch-flip
      if (!gnu_extensions) {
        number_input_files = 1;
        input_file_name = (unsigned long *)xmalloc(8);
        file_line_count = xmalloc(8);
        text_buffers = xmalloc(0x10);
        v2 = dat_4072f0;
        v8 = argv[dat_4072f0];
        v9 = (char *)((long)dat_4072f0 * 8);
        if ((*v8) && (strcmp(v8,"-"))) goto label_4048fe;
        *input_file_name = 0;
        goto label_4046d9;
      }
      number_input_files = argc - dat_4072f0;
      input_file_name = (unsigned long *)xnmalloc((long)number_input_files,8);
      file_line_count = xnmalloc((long)number_input_files,8);
      text_buffers = xnmalloc((long)number_input_files,0x10);
      v1 = input_file_name;
      if (1 <= number_input_files) {
        v4 = (long)dat_4072f0;
        v3 = number_input_files - 1;
        v6 = (long)(dat_4072f0 + 1);
        do {
          v8 = argv[v6 + -1];
          if (*v8) { // branch-flip
            if (!strcmp(v8,"-"))
              v8 = NULL;
          }
          else {
            v8 = NULL;
          }
          v1[(v6 + -1) - v4] = v8;
          dat_4072f0 = (int)v6;
          v6 += 1;
        } while (v4 + 2 + (unsigned long)v3 != v6);
      }
    }
    else {
      input_file_name = (unsigned long *)xmalloc(8);
      file_line_count = xmalloc(8);
      text_buffers = xmalloc(0x10);
      number_input_files = 1;
      *input_file_name = 0;
    }
    goto label_4046ed;
  }
  if (0x77 < v2) goto label_4048f4;
  if (v2 <= 9) {
    if (v2 == -0x83) {
      version_etc(dat_407000,"ptx","GNU coreutils",dat_4072d0,proper_name_utf8("F. Pinard",0x40415c),0);
      exit(0); // no-return
    }
    if (v2 == -0x82) {
      usage(0);
      goto label_40457a;
    }
    goto label_4048f4;
  }
  switch(v2) {
    case 10:
      v4 = __xargmatch_internal("--format",dat_407280,format_args,&format_vals,4,dat_4072a0,1);
      output_format = *(int *)((long)&format_vals + v4 * 4);
      goto label_404310;
    default:
      goto label_4048f4;
    case 0x41:
      auto_reference = 1;
      goto label_404310;
    case 0x46:
      truncation_string = dat_407280;
      unescape_string(dat_407280);
      goto label_404310;
    case 0x47:
      gnu_extensions = 0;
      goto label_404310;
    case 0x4d:
      macro_name = dat_407280;
      goto label_404310;
    case 0x4f:
      output_format = 2;
      goto label_404310;
    case 0x52:
      right_reference = 1;
      goto label_404310;
    case 0x53:
      context_regex._0_8_ = dat_407280;
      unescape_string(dat_407280);
      goto label_404310;
    case 0x54:
      output_format = 3;
      goto label_404310;
    case 0x57:
      word_regex._0_8_ = dat_407280;
      unescape_string(dat_407280);
      if (!*word_regex._0_8_)
        word_regex._0_8_ = NULL;
      goto label_404310;
    case 0x62:
      break_file = dat_407280;
      goto label_404310;
    case 0x66:
      ignore_case = 1;
      goto label_404310;
    case 0x67:
      v2 = xstrtoimax(dat_407280,0,0,&v5,0x404025);
      if ((v2) || (v5 <= 0)) {
        v7 = quote(dat_407280);
        error(1,0,dcgettext(0,"invalid gap width: %s",5),v7);
        goto label_404943;
      }
      gap_size = v5;
      goto label_404310;
    case 0x69:
      ignore_file = dat_407280;
      goto label_404310;
    case 0x6f:
      only_file = dat_407280;
      goto label_404310;
    case 0x72:
      input_reference = 1;
      goto label_404310;
    case 0x74:
      goto label_404310;
    case 0x77:
      v2 = xstrtoimax(dat_407280,0,0,&v5,0x404025);
      if ((v2) || (v5 <= 0)) goto label_40497a;
      line_width = v5;
      goto label_404310;
    
  }
label_4048f4:
  v2 = (int)v7;
  usage(1);
label_4048fe:
  *input_file_name = v8;
label_4046d9:
  dat_4072f0 = v2 + 1;
  if (argc <= dat_4072f0) {
label_4046ed:
    if (!output_format)
      output_format = 2 - (unsigned int)gnu_extensions;
    initialize_regex();
    if (break_file)
      digest_break_file(break_file);
    if ((ignore_file) && (digest_word_file(ignore_file,(WORD_TABLE *)ignore_table), !ignore_table._16_8_))
      ignore_file = NULL;
    if ((only_file) && (digest_word_file(only_file,(WORD_TABLE *)only_table), !only_table._16_8_))
      only_file = NULL;
    number_of_occurs = 0;
    v4 = 0;
    total_line_count = 0;
    maximum_word_length = 0;
    reference_max_width = 0;
    if (1 <= number_input_files) {
      do {
        swallow_file_in_memory((char *)input_file_name[v4],(BLOCK *)(v4 * 0x10 + text_buffers));
        find_occurs_in_text((int)v4);
        total_line_count += 1;
        *(long *)(file_line_count + v4 * 8) = total_line_count;
        v4 += 1;
      } while ((int)v4 < number_input_files);
    }
    sort_found_occurs();
    fix_output_parameters();
    generate_all_output();
    return 0;
  }
  v4 = freopen_safer(*(unsigned long *)((long)argv + (long)&v9[8]),"w",dat_407000);
  if (!v4) {
label_404943:
    v7 = quotearg_n_style_colon(0,3,argv[dat_4072f0]);
    error(1,*(unsigned int *)__errno_location(),"%s",v7);
label_40497a:
    v7 = quote(dat_407280);
    error(1,0,dcgettext(0,"invalid line width: %s",5),v7);
    __stack_chk_fail(); // no-return
  }
  dat_4072f0 += 1;
  if (argc <= dat_4072f0) goto label_4046ed;
  v7 = quote(argv[dat_4072f0]);
  error(0,0,dcgettext(0,"extra operand %s",5),v7);
  goto label_4048f4;
}

