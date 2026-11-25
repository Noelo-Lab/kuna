//! TextUtil against Vineflower's own (`data/GenTextUtilGolden.java`) and the small value
//! classes of `util`.

use kuna_jvm::int_helper::IntHelper;
use kuna_jvm::interpreter_util::InterpreterUtil;
use kuna_jvm::key::Key;
use kuna_jvm::pair::Pair;
use kuna_jvm::rt::{try_catch, JavaEq, JavaHash};
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::text_util::TextUtil;

#[test]
fn text_util_matches_vineflower() {
    let data = include_str!("data/text_util_golden.txt");
    let mut chars = 0u32;
    for line in data.lines() {
        let f: Vec<&str> = line.splitn(4, ' ').collect();
        match f[0] {
            "P" => {
                let first = u32::from_str_radix(f[1], 16).unwrap();
                let last = u32::from_str_radix(f[2], 16).unwrap();
                for c in first..=last {
                    assert_eq!(
                        TextUtil::is_printable_unicode(c as u16),
                        f[3] == "1",
                        "U+{c:04X}"
                    );
                    chars += 1;
                }
            }
            "U" => assert_eq!(
                TextUtil::char_to_unicode_literal(f[1].parse().unwrap()),
                f[2]
            ),
            "I" => assert_eq!(TextUtil::get_instruction_name(f[1].parse().unwrap()), f[2]),
            other => panic!("unknown record {other}"),
        }
    }
    assert_eq!(chars, 0x10000);
    let mut s = String::from(">");
    TextUtil::append(&mut s, "[]", 3);
    assert_eq!(s, ">[][][]");
}

#[test]
fn small_values_follow_java() {
    let p = Pair::of(String::from("a"), 7);
    assert_eq!(p.java_hash(), 97 ^ 7);
    assert!(p.java_equals(&Pair::of("a".to_string(), 7)));
    assert_eq!(p.to_string(), "Pair{a,7}");

    assert_eq!(IntHelper::adjusted_int_representation(0xFF00), "0xFF00");
    assert_eq!(
        IntHelper::adjusted_int_representation(0xFF00_0000u32 as i32),
        "0xFF000000"
    );
    assert_eq!(IntHelper::adjusted_int_representation(-5), "-5");

    let k: Key<i32> = Key::of("CURRENT_CLASS");
    assert_eq!(k.java_hash(), "CURRENT_CLASS".java_hash());

    assert!(InterpreterUtil::equal_sets(
        Some(&[1, 2, 3][..]),
        Some(&[3, 1, 2][..])
    ));
    assert!(InterpreterUtil::equal_sets(
        Some(&[1, 1, 2][..]),
        Some(&[1, 2, 2][..])
    ));
    assert!(!InterpreterUtil::equal_sets(
        Some(&[1, 2, 3][..]),
        Some(&[1, 2, 4][..])
    ));
    assert!(InterpreterUtil::equal_sets::<i32>(None, None));
    assert!(InterpreterUtil::equal_objects::<str>(None, None));
    assert!(InterpreterUtil::equal_objects(Some("x"), Some("x")));
    assert_eq!(InterpreterUtil::make_unique_key("m", "()V"), "m ()V");
    assert_eq!(InterpreterUtil::make_unique_key_3("f", "a", "I"), "f a I");

    assert!(try_catch(|| {
        let mut b = TextBuffer::with_format("\n", " ", 10);
        b.pop_newline_group();
    })
    .unwrap_err()
    .is("IllegalStateException"));
}
