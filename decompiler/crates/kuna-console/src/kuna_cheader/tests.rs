use super::*;

fn bodies(text: &str) -> Vec<(usize, Body)> {
    let mut known = BTreeMap::new();
    split_declarations("t.h", text)
        .into_iter()
        .filter_map(|c| classify(&c.text, &mut known).map(|b| (c.line, b)))
        .collect()
}

fn typedef(decl: &str) -> Body {
    Body::Typedef { decl: decl.into() }
}

fn prototype(func: &str, decl: &str) -> Body {
    Body::Prototype { func: func.into(), decl: decl.into() }
}

#[test]
fn declarations_keep_their_line_and_kind() {
    let got = bodies(
        "/* handle */\ntypedef void CURL;\n\nCURL *curl_easy_init(void);\n\
         int curl_easy_setopt(CURL *h,\n    int opt, ...);\nextern int errno;\n\
         struct pt { int x; int y; };\nstruct opaque;\n",
    );
    assert_eq!(
        got,
        vec![
            (2, typedef("typedef void CURL")),
            (4, prototype("curl_easy_init", "CURL *curl_easy_init(void)")),
            (5, prototype("curl_easy_setopt", "int curl_easy_setopt(CURL *h, int opt, ...)")),
            (8, typedef("struct pt{int x;int y;}")),
        ]
    );
}

#[test]
fn bodies_linkage_blocks_and_decorations_are_dropped() {
    let got = bodies(
        "#ifdef __cplusplus\nextern \"C\" {\n#endif\n\
         static inline int twice(int v) { if (v) { return v * 2; } return 0; }\n\
         extern __attribute__((nonnull(1))) char *dup(const char *__restrict s) __asm__(\"dup2\");\n\
         int run(char *const argv[], __builtin_va_list ap);\n\
         struct __attribute__((packed)) hdr { char tag; int len; };\n\
         #ifdef __cplusplus\n}\n#endif\n",
    );
    assert_eq!(
        got,
        vec![
            (4, prototype("twice", "int twice(int v)")),
            (5, prototype("dup", "char *dup(const char *s)")),
            (6, prototype("run", "int run(char *const *argv, void *ap)")),
            (7, typedef("struct hdr{char tag;int len;}")),
        ]
    );
}

#[test]
fn line_markers_reanchor_the_position() {
    let chunks = split_declarations(
        "t.h",
        "# 1 \"t.h\"\n# 40 \"/inc/dep.h\" 1 3 4\ntypedef int dep_t;\n# 7 \"t.h\" 2\nstruct s {\n# 9 \"t.h\"\n int a; };\n",
    );
    let at: Vec<(&str, usize)> = chunks.iter().map(|c| (c.file.as_str(), c.line)).collect();
    assert_eq!(at, vec![("/inc/dep.h", 40), ("t.h", 7)]);
    assert_eq!(chunks[1].text, "struct s { int a; }");
}

#[test]
fn constant_expressions_fold_to_values() {
    let got = bodies(
        "typedef enum { A, B = (1 << 4) | 3, C, D = B + 0x10, E = -(2), F = sizeof(int), G } e_t;\n\
         struct buf { char data[4 * 8]; char raw[16]; };\n",
    );
    assert_eq!(
        got,
        vec![
            (1, typedef("typedef enum{A, B = 19, C, D = 35, E = -2, F = sizeof(int), G}e_t")),
            (2, typedef("struct buf{char data[32];char raw[16];}")),
        ]
    );
}

#[test]
fn variables_and_function_pointers_state_nothing() {
    assert!(bodies("int counter = 3;\nvoid (*hook)(int);\nextern const char version[];\n").is_empty());
}

#[test]
fn constants_take_the_value_c_gives_them() {
    let got = bodies(
        "enum f { A = 0xFFFFFFFFu + 1, B = 1 << 31, C = ~0U, D = ~0u >> 1, E = 1ULL << 36,\n\
         F = 0b101, G = E2, H = (unsigned)-1, I = (char)1, J = ~0UL, K = 1ULL << 63, L = 1UL << 4 };\n\
         enum g { E2 = 7 };\nstruct s { char a[E2]; char b[0b100]; };\n",
    );
    assert_eq!(
        got,
        vec![
            (
                1,
                typedef(
                    "enum f{A = 0, B = -2147483648, C = 4294967295, D = 2147483647, E = 68719476736, \
                     F = 5, G = E2, H = 4294967295, I = (char)1, J = ~0UL, K = 1ULL<<63, L = 16}"
                )
            ),
            (3, typedef("enum g{E2 = 7}")),
            (4, typedef("struct s{char a[7];char b[4];}")),
        ]
    );
}

#[test]
fn a_body_is_found_after_a_record_in_the_same_declaration() {
    let got = bodies(
        "struct s __attribute__((packed)) { int a; } make(void) __attribute__((cold)) { return s; }\n\
         struct __attribute__((packed)) t { char c; } __attribute__((aligned(2)));\n",
    );
    assert_eq!(
        got,
        vec![
            (1, prototype("make", "struct s{int a;}make(void)")),
            (2, typedef("struct t{char c;}")),
        ]
    );
}

#[test]
fn a_linkage_block_after_a_long_gap_is_still_transparent() {
    let gap = "\n".repeat(300);
    let got = bodies(&format!("extern{gap}\"C\" {{\ntypedef int id_t;\n}}\n"));
    assert_eq!(got, vec![(302, typedef("typedef int id_t"))]);
}

#[test]
fn constants_compilers_disagree_on_are_not_folded() {
    let got = bodies(
        "enum h { A = 0xFFFFFFFFFFFFFFFFLL >> 32, B = (-2147483647 - 1) % -1, C = 7 % -2 };\n",
    );
    assert_eq!(
        got,
        vec![(1, typedef("enum h{A = 0xFFFFFFFFFFFFFFFFLL>>32, B = (-2147483647-1)%-1, C = 1}"))]
    );
}

#[test]
fn a_line_marker_at_the_top_of_the_range_does_not_overflow() {
    let chunks = split_declarations("t.h", "# 18446744073709551615 \"x.h\"\ntypedef int a;\ntypedef int b;\n");
    assert_eq!(chunks.len(), 2);
}
