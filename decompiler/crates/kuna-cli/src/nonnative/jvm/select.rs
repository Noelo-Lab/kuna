//! The selection grammar: which class or method of an input a target names.
//!
//! A class is `com.ex.Main`, `com/ex/Main`, `Main.class`, `com.ex.Main$Inner`
//! or `com.ex.Main.Inner`, or a bare simple name that only one class has. A
//! method is `<class>.<name>` or `<class>#<name>`, optionally followed by its
//! JVM descriptor; `<init>` and `<clinit>` are written as such. A whole target
//! that names a class is that class; otherwise it splits at its last `.`.

use std::collections::BTreeMap;

use kuna_jvm::kuna_inventory::{ClassInfo, MethodInfo};

/// Every class of an input, from the class files alone.
pub(crate) struct Universe {
    pub(crate) classes: Vec<ClassInfo>,
    index: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Target {
    /// A class by internal name.
    Class(String),
    /// The methods `name` of `class` (internal), one overload when `descriptor` is given.
    Methods {
        class: String,
        name: String,
        descriptor: Option<String>,
    },
}

impl Target {
    pub(crate) fn class(&self) -> &str {
        match self {
            Target::Class(class) | Target::Methods { class, .. } => class,
        }
    }

    pub(crate) fn matches(&self, class: &str, name: &str, descriptor: &str) -> bool {
        match self {
            Target::Class(_) => false,
            Target::Methods {
                class: c,
                name: n,
                descriptor: d,
            } => c == class && n == name && d.as_deref().is_none_or(|d| d == descriptor),
        }
    }
}

/// `com/ex/Main$Inner` as `com.ex.Main$Inner`.
pub(crate) fn java_name(internal: &str) -> String {
    internal.replace('/', ".")
}

impl Universe {
    pub(crate) fn new(classes: Vec<ClassInfo>) -> Universe {
        let index = classes
            .iter()
            .enumerate()
            .map(|(i, c)| (c.internal.clone(), i))
            .collect();
        Universe { classes, index }
    }

    pub(crate) fn get(&self, internal: &str) -> Option<&ClassInfo> {
        self.index.get(internal).map(|&i| &self.classes[i])
    }

    /// The top-level class `internal` is written in: its outer chain, as far as the input has it.
    pub(crate) fn root_of(&self, internal: &str) -> String {
        let mut current = internal.to_string();
        for _ in 0..256 {
            match self.get(&current).and_then(|c| c.outer.clone()) {
                Some(outer) if outer != current && self.index.contains_key(&outer) => current = outer,
                _ => break,
            }
        }
        current
    }

    pub(crate) fn method(&self, class: &str, name: &str, descriptor: &str) -> Option<&MethodInfo> {
        self.get(class)?
            .methods
            .iter()
            .find(|m| m.name == name && m.descriptor == descriptor)
    }

    /// The class `text` names, or why it names none (`Ok(None)`: no such class).
    fn class(&self, text: &str) -> Result<Option<String>, String> {
        let text = text.strip_suffix(".class").unwrap_or(text);
        if text.is_empty() {
            return Ok(None);
        }
        let mut candidate = text.replace('.', "/");
        if self.index.contains_key(&candidate) {
            return Ok(Some(candidate));
        }
        while let Some(slash) = candidate.rfind('/') {
            candidate.replace_range(slash..=slash, "$");
            if self.index.contains_key(&candidate) {
                return Ok(Some(candidate));
            }
        }
        if text.contains(['.', '/']) {
            return Ok(None);
        }
        let simple: Vec<&str> = self
            .classes
            .iter()
            .map(|c| c.internal.as_str())
            .filter(|internal| internal.rsplit('/').next() == Some(text))
            .collect();
        match simple.as_slice() {
            [] => Ok(None),
            [one] => Ok(Some(one.to_string())),
            many => Err(format!(
                "class name {text:?} is ambiguous: {}; give the qualified name",
                many.iter().map(|c| java_name(c)).collect::<Vec<_>>().join(", ")
            )),
        }
    }

    /// Resolves `text` against this input; `default_class` (a single class
    /// file's own class) also takes bare method names.
    pub(crate) fn resolve(&self, text: &str, default_class: Option<&str>, input: &str) -> Result<Target, String> {
        let text = text.trim();
        let (head, descriptor) = match text.find('(') {
            Some(open) => (&text[..open], Some(text[open..].to_string())),
            None => (text, None),
        };
        if descriptor.is_none() {
            if let Some(class) = self.class(head)? {
                return Ok(Target::Class(class));
            }
        }
        let split = head
            .rfind('#')
            .or_else(|| head.rfind('.'))
            .map(|at| (&head[..at], &head[at + 1..]));
        if let Some((class_text, name)) = split {
            if let Some(class) = self.class(class_text)? {
                return self.method_target(class, name, descriptor, text);
            }
        }
        if let (Some(class), false) = (default_class, head.contains(['.', '#', '/'])) {
            if self.get(class).is_some_and(|c| c.methods.iter().any(|m| m.name == head)) {
                return self.method_target(class.to_string(), head, descriptor, text);
            }
        }
        let mut message = format!("no class or method {text:?} in {input}");
        if let Some(near) = self.nearest(head) {
            message.push_str(&format!("; did you mean {near:?}?"));
        } else {
            message.push_str(&format!("; `kuna functions {input}` lists every method"));
        }
        Err(message)
    }

    fn method_target(
        &self,
        class: String,
        name: &str,
        descriptor: Option<String>,
        text: &str,
    ) -> Result<Target, String> {
        let info = self.get(&class).expect("resolved class");
        let found = info
            .methods
            .iter()
            .any(|m| m.name == name && descriptor.as_deref().is_none_or(|d| d == m.descriptor));
        if found {
            return Ok(Target::Methods {
                class,
                name: name.to_string(),
                descriptor,
            });
        }
        let owner = java_name(&class);
        let overloads: Vec<String> = info
            .methods
            .iter()
            .filter(|m| m.name == name)
            .map(|m| format!("{owner}.{}{}", m.name, m.descriptor))
            .collect();
        let mut message = format!("no method {text:?} in {owner}");
        if !overloads.is_empty() {
            message.push_str(&format!("; its overloads are {}", overloads.join(", ")));
        } else if let Some(near) = nearest(name, info.methods.iter().map(|m| m.name.as_str())) {
            message.push_str(&format!("; did you mean {:?}?", format!("{owner}.{near}")));
        }
        Err(message)
    }

    /// The class or `Class.method` name closest to `text`.
    fn nearest(&self, text: &str) -> Option<String> {
        let mut names: Vec<String> = Vec::new();
        for class in &self.classes {
            let owner = java_name(&class.internal);
            for method in &class.methods {
                names.push(format!("{owner}.{}", method.name));
            }
            names.push(owner);
        }
        let wanted = text.replace('/', ".");
        nearest(&wanted, names.iter().map(String::as_str)).map(str::to_string)
    }
}

/// The closest of `candidates` to `wanted`: equal ignoring case first, else
/// the nearest within an edit budget that grows with the name.
fn nearest<'a>(wanted: &str, candidates: impl Iterator<Item = &'a str> + Clone) -> Option<&'a str> {
    let lowered = wanted.to_ascii_lowercase();
    if let Some(hit) = candidates.clone().find(|c| c.to_ascii_lowercase() == lowered) {
        return Some(hit);
    }
    let budget = (wanted.chars().count() / 4).clamp(1, 3);
    candidates
        .map(|c| (crate::optname::distance(&lowered, &c.to_ascii_lowercase()), c))
        .filter(|(d, _)| *d <= budget)
        .min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(b.1)))
        .map(|(_, c)| c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class(internal: &str, outer: Option<&str>, methods: &[(&str, &str)]) -> ClassInfo {
        ClassInfo {
            internal: internal.into(),
            outer: outer.map(Into::into),
            methods: methods
                .iter()
                .map(|(n, d)| MethodInfo {
                    name: (*n).into(),
                    descriptor: (*d).into(),
                    ..MethodInfo::default()
                })
                .collect(),
            ..ClassInfo::default()
        }
    }

    fn universe() -> Universe {
        Universe::new(vec![
            class(
                "com/ex/Main",
                None,
                &[("<init>", "(I)V"), ("check", "(Ljava/lang/String;)Z"), ("add", "(II)I"), ("add", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")],
            ),
            class("com/ex/Main$Checker", Some("com/ex/Main"), &[("verify", "(Ljava/lang/String;)Z")]),
            class("com/ex/Main$1", Some("com/ex/Main"), &[("run", "()V")]),
            class("com/ex/util/Helper", None, &[("twice", "(I)I")]),
        ])
    }

    #[test]
    fn classes_resolve_in_every_spelling() {
        let u = universe();
        let main = Target::Class("com/ex/Main".into());
        for text in ["com.ex.Main", "com/ex/Main", "Main", "Main.class", "com/ex/Main.class"] {
            assert_eq!(u.resolve(text, None, "app.jar").unwrap(), main, "{text}");
        }
        let nested = Target::Class("com/ex/Main$Checker".into());
        for text in ["com.ex.Main$Checker", "com.ex.Main.Checker", "Main$Checker"] {
            assert_eq!(u.resolve(text, None, "app.jar").unwrap(), nested, "{text}");
        }
        assert_eq!(u.root_of("com/ex/Main$Checker"), "com/ex/Main");
        assert_eq!(u.root_of("com/ex/util/Helper"), "com/ex/util/Helper");
    }

    #[test]
    fn methods_resolve_by_name_and_descriptor() {
        let u = universe();
        let check = Target::Methods {
            class: "com/ex/Main".into(),
            name: "check".into(),
            descriptor: None,
        };
        for text in ["com.ex.Main.check", "com.ex.Main#check", "Main.check", "com/ex/Main.check"] {
            assert_eq!(u.resolve(text, None, "app.jar").unwrap(), check, "{text}");
        }
        let add = u.resolve("com.ex.Main.add(II)I", None, "app.jar").unwrap();
        assert!(add.matches("com/ex/Main", "add", "(II)I"));
        assert!(!add.matches("com/ex/Main", "add", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"));
        assert!(u.resolve("com.ex.Main.<init>", None, "app.jar").is_ok());
        assert!(u.resolve("com.ex.Main.Checker.verify", None, "app.jar").is_ok());
        assert_eq!(u.resolve("check", Some("com/ex/Main"), "Main.class").unwrap(), check);
    }

    #[test]
    fn misses_suggest_the_nearest_name() {
        let u = universe();
        let e = u.resolve("com.ex.Mian", None, "app.jar").unwrap_err();
        assert!(e.contains("did you mean \"com.ex.Main\"?"), "{e}");
        let e = u.resolve("com.ex.Main.chek", None, "app.jar").unwrap_err();
        assert!(e.contains("did you mean \"com.ex.Main.check\"?"), "{e}");
        let e = u.resolve("com.ex.Main.add(J)J", None, "app.jar").unwrap_err();
        assert!(e.contains("its overloads are com.ex.Main.add(II)I"), "{e}");
        let e = u.resolve("zzzz", None, "app.jar").unwrap_err();
        assert!(e.contains("kuna functions app.jar"), "{e}");
    }
}
