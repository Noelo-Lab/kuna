//! Strip only storage diagnostics when comparing frozen x86 and toy C oracles.

#![allow(dead_code)]

use std::sync::OnceLock;

use regex::Regex;

fn is_storage(source: &str) -> bool {
    static HOME: OnceLock<Regex> = OnceLock::new();
    let register = r"(?i:acc|(?:r|e)?(?:ax|bx|cx|dx|sp|bp|di|si)|[abcd][lh]|(?:[sb]p|[sd]i)l|r[0-9]+[bwd]?|(?:[xyz]mm|st)[0-9]+(?:_[a-z]+)?|[cdefgs]s|[fg]s_offset|[csozapd]f)";
    let home = HOME.get_or_init(|| {
        Regex::new(&format!(
            r"^(?:tmp|{register}(?::{register})*|stack [+-] 0x[0-9a-fA-F]+|[A-Za-z_][A-Za-z0-9_]* 0x[0-9a-fA-F]+)$"
        ))
        .unwrap()
    });
    source.split(" | ").all(|part| home.is_match(part))
}

fn declaration_source(line: &str) -> Option<(&str, &str, &str)> {
    static DECLARATION: OnceLock<Regex> = OnceLock::new();
    let declaration = DECLARATION.get_or_init(|| {
        Regex::new(r"^(?P<decl>  (?:[A-Za-z_][A-Za-z0-9_]*[ \t]+|\*)+(?P<name>[A-Za-z_][A-Za-z0-9_]*)(?: \[[0-9]+\])?;) // (?P<source>.+)$").unwrap()
    });
    let captures = declaration.captures(line)?;
    let decl = captures.name("decl")?.as_str();
    if matches!(
        decl.trim_start().split_whitespace().next()?,
        "return" | "goto" | "throw"
    ) {
        return None;
    }
    let source = captures.name("source")?.as_str();
    is_storage(source).then_some((decl, captures.name("name")?.as_str(), source))
}

fn parameter_source(line: &str) -> Option<&str> {
    let (name, source) = line.strip_prefix("  // ")?.split_once(": ")?;
    if name.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        || name.as_bytes()[0].is_ascii_digit()
    {
        return None;
    }
    is_storage(source).then_some(source)
}

fn parameter_names(line: &str) -> Option<Vec<&str>> {
    static PROTOTYPE: OnceLock<Regex> = OnceLock::new();
    static NAME: OnceLock<Regex> = OnceLock::new();
    let prototype = PROTOTYPE
        .get_or_init(|| Regex::new(r"^[A-Za-z_][^{};\n]*\(([^()]*)\)(?: // [^\n]+)?$").unwrap());
    let name =
        NAME.get_or_init(|| Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\s*(?:\[[^\]]*\])?\s*$").unwrap());
    let arguments = prototype.captures(line)?.get(1)?.as_str();
    Some(
        arguments
            .split(',')
            .filter_map(|argument| name.captures(argument)?.get(1).map(|name| name.as_str()))
            .collect(),
    )
}

pub fn normalize_sources(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut parameters = Vec::new();
    for line in text.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if let Some(names) = parameter_names(content) {
            parameters = names;
        }
        if parameter_source(content).is_some() {
            let (name, _) = content
                .strip_prefix("  // ")
                .unwrap()
                .split_once(": ")
                .unwrap();
            if parameters.contains(&name) {
                continue;
            }
        }
        if let Some((declaration, _, _)) = declaration_source(content) {
            normalized.push_str(declaration);
            if line.ends_with('\n') {
                normalized.push('\n');
            }
        } else {
            normalized.push_str(line);
        }
    }
    normalized
}

pub fn has_decl_source(text: &str, name: &str, home: &str) -> bool {
    text.lines().any(|line| {
        declaration_source(line).is_some_and(|(_, variable, source)| {
            variable == name && source.split(" | ").any(|part| part == home)
        })
    })
}

pub fn has_source(text: &str, home: &str) -> bool {
    text.lines().any(|line| {
        declaration_source(line)
            .map(|(_, _, source)| source)
            .or_else(|| parameter_source(line))
            .is_some_and(|source| source.split(" | ").any(|part| part == home))
    })
}

#[test]
fn normalization_preserves_warnings_statements_and_unrelated_comments() {
    let text = "\nuint1 f(int4 x) // warn: bad data\n{\n  uint1 v1; // INTMEM 0x52 | acc\n  \n  // x: edi | stack - 0xc\n  // note: handle the warning\n  // note: rax\n  int4 v2; // warning\n  v1 = 1; // branch-flip\n  return v1; // acc\n}\n";
    let expected = "\nuint1 f(int4 x) // warn: bad data\n{\n  uint1 v1;\n  \n  // note: handle the warning\n  // note: rax\n  int4 v2; // warning\n  v1 = 1; // branch-flip\n  return v1; // acc\n}\n";
    assert_eq!(normalize_sources(text), expected);
    assert!(has_decl_source(text, "v1", "acc"));
    assert!(!has_decl_source(text, "v2", "acc"));
    assert!(!has_source(text, "ax"));
}
