//! Imported data names must denote identifiers rather than C expressions.
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

pub(crate) struct Names {
    reserved: HashSet<String>,
    assigned: HashMap<(String, u64), String>,
}

impl Names {
    pub(crate) fn new(names: impl IntoIterator<Item = String>) -> Self {
        Self {
            reserved: names
                .into_iter()
                .map(|name| basename(&name).to_string())
                .collect(),
            assigned: HashMap::new(),
        }
    }

    pub(crate) fn name<'a>(&mut self, original: &'a str, address: u64) -> Cow<'a, str> {
        let base = basename(original);
        if !invalid(base) {
            return Cow::Borrowed(original);
        }
        let key = (original.to_string(), address);
        if let Some(name) = self.assigned.get(&key) {
            return Cow::Owned(name.clone());
        }
        let stem = format!("kuna_global_{address:x}");
        let mut candidate = stem.clone();
        let mut suffix = 0;
        while !self.reserved.insert(candidate.clone()) {
            suffix += 1;
            candidate = format!("{stem}_{suffix}");
        }
        let qualified = match original.rsplit_once("::") {
            Some((scope, _)) => format!("{scope}::{candidate}"),
            None => candidate,
        };
        self.assigned.insert(key, qualified.clone());
        Cow::Owned(qualified)
    }
}

fn basename(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name)
}

fn invalid(name: &str) -> bool {
    name.is_empty()
        || !name
            .chars()
            .next()
            .is_some_and(|c| c == '_' || unicode_ident::is_xid_start(c))
        || name.chars().any(|c| !unicode_ident::is_xid_continue(c))
        || matches!(
            name,
            "auto"
                | "break"
                | "case"
                | "char"
                | "const"
                | "continue"
                | "default"
                | "do"
                | "double"
                | "else"
                | "enum"
                | "extern"
                | "float"
                | "for"
                | "goto"
                | "if"
                | "inline"
                | "int"
                | "long"
                | "register"
                | "restrict"
                | "return"
                | "short"
                | "signed"
                | "sizeof"
                | "static"
                | "struct"
                | "switch"
                | "typedef"
                | "union"
                | "unsigned"
                | "void"
                | "volatile"
                | "while"
                | "_Alignas"
                | "_Alignof"
                | "_Atomic"
                | "_Bool"
                | "_Complex"
                | "_Generic"
                | "_Imaginary"
                | "_Noreturn"
                | "_Static_assert"
                | "_Thread_local"
                | "bool"
                | "true"
                | "false"
                | "asm"
                | "__asm__"
                | "typeof"
                | "__typeof__"
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imported_spellings_remain_distinct_from_reserved_functions_and_data() {
        let mut names = Names::new(
            ["counter_3_1", "kuna_global_1000", "ns::kuna_global_1000_1"]
                .into_iter()
                .map(str::to_string),
        );
        assert_eq!(names.name("counter_3_1", 0x2000), "counter_3_1");
        let dotted = names.name("counter.3.1", 0x1000).into_owned();
        assert!(!invalid(&dotted));
        assert_ne!(dotted, "counter_3_1");
        assert_ne!(dotted, "kuna_global_1000");
        assert_ne!(dotted, "kuna_global_1000_1");
        assert_eq!(names.name("counter.3.1", 0x1000), dotted);
        assert_ne!(names.name("other.0", 0x1000), dotted);
        assert_eq!(names.name("ns::valid", 0x3000), "ns::valid");
        assert!(names
            .name("ns::other.0", 0x4000)
            .starts_with("ns::kuna_global_"));
        assert_eq!(names.name("ação", 0x5000), "ação");
        for raw in [
            "for",
            "1value",
            "a-b",
            "a b",
            "a.b",
            "bad\u{a0}name",
            "name—part",
            "١value",
            "",
        ] {
            assert!(!invalid(&names.name(raw, 0x6000)));
        }
    }
}
