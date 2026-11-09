//! TextBuffer replayed against traces of Vineflower's own TextBuffer
//! (`data/GenTextBufferGolden.java` on vineflower-1.12.0.jar).

use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::{try_catch, JavaStr};
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::text_token::TextTokenKind;

fn decode(tok: &str) -> String {
    let hex = tok.strip_prefix('x').expect("encoded string");
    let units: Vec<u16> = (0..hex.len() / 4)
        .map(|i| u16::from_str_radix(&hex[i * 4..i * 4 + 4], 16).unwrap())
        .collect();
    from_utf16(&units)
}

fn esc(s: &str) -> String {
    let mut b = String::new();
    for u in s.utf16() {
        match u {
            0x5C => b.push_str("\\\\"),
            0x0A => b.push_str("\\n"),
            0x0D => b.push_str("\\r"),
            0x09 => b.push_str("\\t"),
            0x20..=0x7E => b.push(u as u8 as char),
            _ => b.push_str(&format!("\\u{u:04x}")),
        }
    }
    b
}

fn tokens(b: &TextBuffer) -> String {
    let parts: Vec<String> = b
        .get_tokens()
        .iter()
        .map(|t| {
            let TextTokenKind::Class(c) = &t.kind else {
                panic!("only class tokens are recorded")
            };
            format!(
                "{}:{}:{}:{}",
                t.get_start(),
                t.get_length(),
                t.is_declaration() as i32,
                esc(&c.qualified_name)
            )
        })
        .collect();
    if parts.is_empty() {
        "-".to_string()
    } else {
        parts.join(";")
    }
}

struct Format {
    sep: &'static str,
    indent: String,
    pll: i32,
}

impl Format {
    fn buffer(&self) -> TextBuffer {
        TextBuffer::with_format(self.sep, &self.indent, self.pll)
    }
}

fn run_scenario(fmt: &Format, lines: &[(usize, &str)]) {
    let mut bufs: Vec<TextBuffer> = (0..5).map(|_| fmt.buffer()).collect();
    for &(at, line) in lines {
        let (head, want) = match line.split_once(" = ") {
            Some((h, w)) => (h, Some(w)),
            None => (line, None),
        };
        let a: Vec<&str> = head.split(' ').collect();
        let n = |i: usize| -> i32 { a[i].parse().unwrap() };
        let b = n(1) as usize;
        let check = |got: String| assert_eq!(got, want.unwrap(), "`{head}` on line {at}");
        match a[0] {
            "new" => bufs[b] = fmt.buffer(),
            "ap" => drop(bufs[b].append(&decode(a[2]))),
            "ac" => drop(bufs[b].append_char(decode(a[2]).chars().next().unwrap())),
            "ai" => drop(bufs[b].append_int(n(2))),
            "ls" => drop(bufs[b].append_line_separator()),
            "in" => drop(bufs[b].append_indent(n(2))),
            "push" => drop(bufs[b].push_newline_group(n(2), n(3))),
            "pn" => drop(bufs[b].append_possible_newline()),
            "pna" => drop(bufs[b].append_possible_newline_alt(&decode(a[2]))),
            "pnd" => drop(bufs[b].append_possible_newline_dedent(&decode(a[2]), true)),
            "pop" => drop(bufs[b].pop_newline_group()),
            "pre" => drop(bufs[b].prepend(&decode(a[2]))),
            "enc" => drop(bufs[b].enclose(&decode(a[2]), &decode(a[3]))),
            "ss" => bufs[b].set_start(n(2)),
            "sl" => bufs[b].set_length(n(2)),
            "cls" => drop(bufs[b].append_class(&decode(a[2]), n(3) == 1, &decode(a[4]))),
            "aac" => drop(bufs[b].append_all_classes(&decode(a[2]), Some(&decode(a[3])))),
            "act" => drop(bufs[b].add_class_token(n(2), n(3), &decode(a[4]))),
            "aat" => drop(bufs[b].add_all_class_tokens(n(2), &decode(a[3]), Some(&decode(a[4])))),
            "bsm" => bufs[b].add_start_bytecode_mapping_offset(n(2)),
            "bm" => bufs[b].add_bytecode_mapping_offset(n(2)),
            "apb" => {
                let other = bufs[n(2) as usize].clone();
                bufs[b].append_buffer(&other);
            }
            "apbm" => {
                let other = bufs[n(2) as usize].clone();
                bufs[b].append_buffer_mapped(&other, Some(&decode(a[3])), Some(&decode(a[4])));
            }
            "q" => {
                let t = &bufs[b];
                check(format!(
                    "{} {} {} {} {}",
                    t.length(),
                    t.count_lines(),
                    t.count(" ", 1),
                    t.contains_only_whitespaces() as i32,
                    t.count_chars('a')
                ));
            }
            "pos" => check(bufs[b].get_pos(n(2))),
            "str" => {
                let t = &bufs[b];
                check(format!(
                    "{} {}",
                    esc(&t.convert_to_string_and_allow_data_discard()),
                    tokens(t)
                ));
            }
            "ref" => {
                let buf = &mut bufs[b];
                let outcome = match try_catch(|| buf.reformat()) {
                    Ok(()) => "OK",
                    Err(_) => "EXC",
                };
                check(outcome.to_string());
            }
            "dol" => {
                let mapping: Vec<i32> = a[2].split(',').map(|x| x.parse().unwrap()).collect();
                bufs[b].dump_original_line_numbers(&mapping);
            }
            other => panic!("unknown op {other} on line {at}"),
        }
    }
}

#[test]
fn text_buffer_matches_vineflower() {
    let data = include_str!("data/text_buffer_golden.txt");
    let mut scenarios: Vec<(Format, Vec<(usize, &str)>)> = Vec::new();
    for (i, line) in data.lines().enumerate() {
        if let Some(rest) = line.strip_prefix("TB ") {
            let f: Vec<&str> = rest.split(' ').collect();
            let fmt = Format {
                sep: if f[0] == "1" { "\n" } else { "\r\n" },
                indent: decode(f[1]),
                pll: f[2].parse().unwrap(),
            };
            scenarios.push((fmt, Vec::new()));
        } else {
            scenarios.last_mut().unwrap().1.push((i + 1, line));
        }
    }
    assert_eq!(scenarios.len(), 60);
    for (fmt, lines) in &scenarios {
        run_scenario(fmt, lines);
    }
}

#[test]
fn appended_buffers_share_replacements_like_java() {
    let mut inner = TextBuffer::with_format("\n", "  ", 10);
    inner.push_newline_group(0, 1);
    inner.append("f(");
    inner.append_possible_newline();
    inner.append("aaaaaaaaaaaa");
    inner.append(")");
    inner.pop_newline_group();
    let mut outer = TextBuffer::with_format("\n", "  ", 10);
    outer.append("x = ");
    outer.append_buffer(&inner);
    let mut again = TextBuffer::with_format("\n", "  ", 10);
    again.append_buffer(&inner);
    again.reformat();
    assert_eq!(
        again.convert_to_string_and_allow_data_discard(),
        "f(aaaa\n  aaaaaaaa)"
    );
    outer.reformat();
    assert_eq!(
        outer.convert_to_string_and_allow_data_discard(),
        "x = f(\n  aaaaaaaaaaaa)"
    );
}
