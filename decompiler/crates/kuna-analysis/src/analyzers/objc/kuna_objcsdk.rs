//! (kuna `objcsdk`) Objective-C method signatures from SDK headers, for the
//! selectors an image only sends.
//!
//! A stub `objc_msgSend$setAlpha:` has a prototype only when something in the
//! image declares the selector (§1.4): a UIKit setter never does, so its calls
//! kept the untyped call-site inference and lost arguments
//! (`objc_msgSend$setTransitioningDelegate:(v3)`). The declarations exist — in
//! the SDK the app was built against — and Apple's headers may not be shipped
//! with kuna, so this reads them at load from a directory the user names
//! (`--option objcsdk <SDK root or header dir>`, `KUNA_OBJCSDK`): an SDK root
//! contributes the `Headers/` of every framework the image links
//! (`LC_LOAD_DYLIB`), any other directory every `.h` under it. Nothing derived
//! from the headers is written anywhere.
//!
//! The grammar read is the declaration subset: `@interface` / `@protocol`
//! (categories included) bodies, `- (ret)sel:(T)a …;` methods and `@property
//! (attrs) T name;` with `getter=`/`setter=`/`readonly` honoured, plus `typedef`
//! statements anywhere so `NSInteger`, `CGFloat`, `NS_ENUM` names and
//! `NSString *`-backed typed enums resolve. Each declaration becomes the same
//! type-encoding string the runtime metadata would carry (`v@:d` for
//! `setAlpha:(CGFloat)`), so it rides the one selector → prototype path. A type
//! this reader cannot encode (an unknown struct, a function pointer) makes the
//! selector unusable rather than guessed, and one selector declared with two
//! different shapes across classes is dropped; a class-pointer argument keeps
//! its class (`@"UIView"`) but classes do not count as a disagreement.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use object::read::macho::{LoadCommandVariant, MachHeader};
use object::FileKind;

/// `selector → type encoding` for every declaration under `root` that this
/// image's frameworks reach; `None` marks a selector declared two ways.
pub fn signatures(root: &Path, frameworks: &[String]) -> HashMap<String, Option<String>> {
    let mut texts = Vec::new();
    for path in headers(root, frameworks) {
        if let Ok(text) = std::fs::read_to_string(&path) {
            texts.push(strip_comments(&text));
        }
    }
    let mut types = Types::default();
    for text in &texts {
        types.scan_typedefs(text);
    }
    let mut out: HashMap<String, Option<String>> = HashMap::new();
    for text in &texts {
        scan_declarations(text, &types, &mut out);
    }
    out
}

/// The frameworks a Mach-O links (`UIKit` for
/// `/System/Library/Frameworks/UIKit.framework/UIKit`), in load-command order.
pub fn linked_frameworks(file: &object::File, bytes: &[u8]) -> Vec<String> {
    fn collect<Mach: MachHeader<Endian = object::Endianness>>(bytes: &[u8]) -> Vec<String> {
        let mut out = Vec::new();
        let Ok(header) = Mach::parse(bytes, 0) else { return out };
        let Ok(endian) = header.endian() else { return out };
        let Ok(mut commands) = header.load_commands(endian, bytes, 0) else { return out };
        while let Ok(Some(command)) = commands.next() {
            let Ok(LoadCommandVariant::Dylib(dylib)) = command.variant() else { continue };
            let Ok(name) = command.string(endian, dylib.dylib.name) else { continue };
            let name = String::from_utf8_lossy(name);
            if let Some(fw) = name.rsplit('/').nth(1).and_then(|d| d.strip_suffix(".framework")) {
                out.push(fw.to_string());
            }
        }
        out
    }
    if file.format() != object::BinaryFormat::MachO {
        return Vec::new();
    }
    match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => collect::<object::macho::MachHeader64<object::Endianness>>(bytes),
        Ok(FileKind::MachO32) => collect::<object::macho::MachHeader32<object::Endianness>>(bytes),
        _ => Vec::new(),
    }
}

/// The header files to read: an SDK root's linked frameworks (every framework
/// when none is linked), or every `.h` under any other directory.
fn headers(root: &Path, frameworks: &[String]) -> Vec<PathBuf> {
    let fw_root = root.join("System/Library/Frameworks");
    let mut out = Vec::new();
    if fw_root.is_dir() {
        let dirs: Vec<PathBuf> = if frameworks.is_empty() {
            std::fs::read_dir(&fw_root).map(|d| d.flatten().map(|e| e.path()).collect()).unwrap_or_default()
        } else {
            frameworks.iter().map(|f| fw_root.join(format!("{f}.framework"))).collect()
        };
        for dir in dirs {
            walk(&dir.join("Headers"), &mut out, 0);
        }
    } else {
        walk(root, &mut out, 0);
    }
    out.sort();
    out
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > 8 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out, depth + 1);
        } else if path.extension().is_some_and(|x| x == "h") {
            out.push(path);
        }
    }
}

/// Drop `//` and `/* */` comments and preprocessor lines; newlines are kept so
/// `@interface` lines stay line-delimited.
fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let b = text.as_bytes();
    let mut i = 0;
    let mut line_start = true;
    while i < b.len() {
        if b[i] == b'/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
        } else if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            i += 2;
            while i < b.len() && !(b[i] == b'*' && b.get(i + 1) == Some(&b'/')) {
                if b[i] == b'\n' {
                    out.push('\n');
                }
                i += 1;
            }
            i += 2;
        } else if line_start && b[i] == b'#' {
            // A preprocessor line, with its backslash continuations.
            while i < b.len() && b[i] != b'\n' {
                if b[i] == b'\\' && b.get(i + 1) == Some(&b'\n') {
                    i += 1;
                    out.push('\n');
                }
                i += 1;
            }
        } else if b[i] == b'"' {
            // A string literal: copied through so a quote never starts a comment.
            let start = i;
            i += 1;
            while i < b.len() && b[i] != b'"' && b[i] != b'\n' {
                if b[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            i = (i + 1).min(b.len());
            out.push_str(&text[start..i]);
            line_start = false;
            continue;
        } else {
            line_start = b[i] == b'\n' || (line_start && b[i].is_ascii_whitespace());
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        line_start = true;
    }
    out
}

/// The type names the headers define, resolved lazily to encodings.
#[derive(Default)]
struct Types {
    /// `typedef` name → its raw declared type text (`double`, `NSString *`,
    /// `enum:NSInteger`), resolved on demand.
    typedefs: HashMap<String, String>,
}

impl Types {
    /// Record every `typedef` statement of `text`.
    fn scan_typedefs(&mut self, text: &str) {
        for stmt in statements(text) {
            let Some(rest) = stmt.strip_prefix("typedef") else { continue };
            let rest = rest.trim();
            // typedef NS_ENUM(NSInteger, Name) { … }  /  NS_OPTIONS / NS_CLOSED_ENUM / NS_ERROR_ENUM(domain, Name)
            if let Some((macro_name, inner)) = rest.split_once('(') {
                let m = macro_name.trim();
                if m.starts_with("NS_") && (m.ends_with("ENUM") || m.ends_with("OPTIONS")) {
                    let args: Vec<&str> = inner.split(')').next().unwrap_or("").split(',').map(str::trim).collect();
                    if let [base, name] = args[..] {
                        let base = if m == "NS_ERROR_ENUM" { "NSInteger" } else { base };
                        self.typedefs.insert(name.to_string(), base.to_string());
                    }
                    continue;
                }
            }
            // typedef enum Name : Base { … } Name;  /  typedef enum : Base { … } Name;
            if let Some(rest) = rest.strip_prefix("enum") {
                let base = rest.split('{').next().and_then(|h| h.split_once(':')).map(|(_, b)| b.trim().to_string());
                let name = last_ident(after_braces(rest));
                if let (Some(base), Some(name)) = (base, name) {
                    self.typedefs.insert(name, base);
                }
                continue;
            }
            // typedef struct Tag { … } Name;  /  typedef struct Tag Name;  /  typedef struct Tag *Ref;
            if let Some(rest) = rest.strip_prefix("struct") {
                let tail = after_braces(rest);
                let name = last_ident(tail);
                if let Some(name) = name {
                    let tag = rest.split(|c: char| c == '{' || c == '*').next().unwrap_or("").trim();
                    let tag = tag.split_whitespace().find(|t| !t.starts_with("__attribute__") && !t.starts_with('(')).unwrap_or("");
                    let value = if tail.contains('*') {
                        "void *".to_string()
                    } else if tag.is_empty() || tag == name {
                        format!("struct {name}")
                    } else {
                        format!("struct {tag}")
                    };
                    self.typedefs.insert(name, value);
                }
                continue;
            }
            if rest.contains("(^") {
                if let Some(name) = rest.split("(^").nth(1).and_then(|s| s.split(')').next()).map(str::trim) {
                    if !name.is_empty() {
                        self.typedefs.insert(name.to_string(), "@?".to_string());
                    }
                }
                continue;
            }
            // typedef <type> <Name> [MACROS];
            let toks = tokens(rest);
            let toks = drop_macros(&toks);
            if let Some((name, ty)) = toks.split_last() {
                if is_ident(name) && !ty.is_empty() {
                    self.typedefs.insert(name.to_string(), ty.join(" "));
                }
            }
        }
    }

    /// The encoding of a declared type (`NSInteger` → `q`, `UIView *` →
    /// `@"UIView"`, `CGRect` → `{CGRect=dddd}`), or `None` when it is not known.
    fn encode(&self, text: &str, depth: usize) -> Option<String> {
        if depth > 16 {
            return None;
        }
        let toks = tokens(text);
        let toks = drop_macros(&toks);
        let toks: Vec<&str> = toks
            .iter()
            .copied()
            .filter(|t| !matches!(*t, "nullable" | "nonnull" | "null_unspecified" | "null_resettable" | "_Nullable" | "_Nonnull" | "_Null_unspecified" | "__kindof" | "const" | "volatile" | "__unsafe_unretained" | "__weak" | "__strong" | "__autoreleasing" | "in" | "out" | "inout" | "bycopy" | "byref" | "oneway" | "struct" | "enum" | "union" | "NS_NOESCAPE"))
            .filter(|t| !t.starts_with('<') && !t.starts_with("__attribute__"))
            .collect();
        if text.contains("(^") || text.contains("^)") || toks.iter().any(|t| t.starts_with('(')) {
            return Some("@?".to_string());
        }
        let stars = toks.iter().rev().take_while(|t| **t == "*").count();
        let base: Vec<&str> = toks[..toks.len() - stars].to_vec();
        if base.is_empty() {
            return None;
        }
        let base_text = base.join(" ");
        if stars > 0 {
            return Some(match (base_text.as_str(), stars) {
                ("char" | "unsigned char" | "signed char", 1) => "*".to_string(),
                ("void", 1) => "^v".to_string(),
                (b, 1) if b == "id" || b == "instancetype" => "^@".to_string(),
                (b, 1) if is_class_name(b) => format!("@\"{b}\""),
                (b, 1) if self.typedefs.get(b).is_some_and(|t| is_class_name(t)) => format!("@\"{}\"", self.typedefs[b]),
                _ => "^v".to_string(),
            });
        }
        if let Some(enc) = scalar(&base_text) {
            return Some(enc.to_string());
        }
        if let Some(enc) = by_value_struct(&base_text) {
            return Some(enc.to_string());
        }
        if let Some(def) = self.typedefs.get(&base_text) {
            return self.encode(def, depth + 1);
        }
        None
    }
}

/// `@interface`/`@protocol` bodies: every method and property declaration,
/// recorded by selector.
fn scan_declarations(text: &str, types: &Types, out: &mut HashMap<String, Option<String>>) {
    let mut in_body = false;
    let mut depth = 0i32;
    let mut stmt = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("@interface") || t.starts_with("@protocol") && !t.contains(';') {
            in_body = true;
            depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
            stmt.clear();
            continue;
        }
        if t.starts_with("@end") {
            in_body = false;
            continue;
        }
        if !in_body {
            continue;
        }
        if depth > 0 {
            depth += t.matches('{').count() as i32 - t.matches('}').count() as i32;
            continue;
        }
        if t.starts_with('{') {
            depth += t.matches('{').count() as i32 - t.matches('}').count() as i32;
            continue;
        }
        if t.starts_with("@optional") || t.starts_with("@required") || t.starts_with("@class") || t.is_empty() {
            continue;
        }
        stmt.push_str(t);
        stmt.push(' ');
        if !t.ends_with(';') {
            continue;
        }
        let s = std::mem::take(&mut stmt);
        let s = s.trim().trim_end_matches(';').trim();
        if s.starts_with('-') || s.starts_with('+') {
            if let Some((sel, enc)) = method(s, types) {
                record(out, sel, enc);
            }
        } else if let Some(rest) = s.strip_prefix("@property") {
            for (sel, enc) in property(rest.trim(), types) {
                record(out, sel, enc);
            }
        }
    }
}

/// Agreement is judged with class names erased; the first full spelling is kept.
fn record(out: &mut HashMap<String, Option<String>>, sel: String, enc: Option<String>) {
    let Some(enc) = enc else {
        out.insert(sel, None);
        return;
    };
    match out.get(&sel) {
        None => {
            out.insert(sel, Some(enc));
        }
        Some(Some(have)) if erase_classes(have) == erase_classes(&enc) => {}
        Some(_) => {
            out.insert(sel, None);
        }
    }
}

fn erase_classes(enc: &str) -> String {
    let mut out = String::new();
    let mut in_quote = false;
    for c in enc.chars() {
        if c == '"' {
            in_quote = !in_quote;
        } else if !in_quote {
            out.push(c);
        }
    }
    out
}

/// `- (ret)part:(T)name part2:(T2)name2 …` → the selector and its encoding
/// (`None` when a type does not encode).
fn method(s: &str, types: &Types) -> Option<(String, Option<String>)> {
    let b = s.as_bytes();
    let mut i = 1;
    skip_ws(b, &mut i);
    let ret = paren_group(b, &mut i)?;
    let mut sel = String::new();
    let mut encs: Vec<Option<String>> = vec![Some("@".to_string()), Some(":".to_string())];
    loop {
        skip_ws(b, &mut i);
        let start = i;
        while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
            i += 1;
        }
        if i == start {
            break;
        }
        let part = &s[start..i];
        let mut j = i;
        skip_ws(b, &mut j);
        if j < b.len() && b[j] == b':' {
            i = j + 1;
            skip_ws(b, &mut i);
            let ty = paren_group(b, &mut i)?;
            encs.push(types.encode(&ty, 0));
            sel.push_str(part);
            sel.push(':');
            skip_ws(b, &mut i);
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            continue;
        }
        if sel.is_empty() {
            sel = part.to_string();
        }
        break;
    }
    if sel.is_empty() {
        return None;
    }
    let mut enc = types.encode(&ret, 0)?;
    for e in encs {
        enc.push_str(&e?);
    }
    Some((sel, Some(enc)))
}

/// `(attrs) T name` → the getter (and, unless `readonly`, the setter).
fn property(rest: &str, types: &Types) -> Vec<(String, Option<String>)> {
    let b = rest.as_bytes();
    let mut i = 0;
    let attrs = if b.first() == Some(&b'(') { paren_group(b, &mut i).unwrap_or_default() } else { String::new() };
    let mut readonly = false;
    let mut getter = None;
    let mut setter = None;
    for a in attrs.split(',').map(str::trim) {
        if a == "readonly" {
            readonly = true;
        } else if let Some(g) = a.strip_prefix("getter=") {
            getter = Some(g.trim().to_string());
        } else if let Some(s) = a.strip_prefix("setter=") {
            setter = Some(s.trim().trim_end_matches(':').to_string());
        }
    }
    let decl = &rest[i..];
    let (ty, name) = if let Some(k) = decl.find("(^") {
        let name = decl[k + 2..].split(')').next().unwrap_or("").trim().to_string();
        (decl.to_string(), name)
    } else {
        let toks = tokens(decl);
        let toks = drop_macros(&toks);
        match toks.split_last() {
            Some((name, ty)) if is_ident(name) => (ty.join(" "), name.to_string()),
            _ => return Vec::new(),
        }
    };
    if name.is_empty() {
        return Vec::new();
    }
    let enc = types.encode(&ty, 0);
    let mut out = vec![(getter.unwrap_or_else(|| name.clone()), enc.as_ref().map(|e| format!("{e}@:")))];
    if !readonly {
        let setter = setter.unwrap_or_else(|| {
            let mut c = name.chars();
            format!("set{}{}", c.next().map(|f| f.to_ascii_uppercase()).unwrap_or_default(), c.as_str())
        });
        out.push((format!("{setter}:"), enc.map(|e| format!("v@:{e}"))));
    }
    out
}

/// Statements of `text` split at `;` outside braces and parentheses.
fn statements(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let (mut braces, mut parens) = (0i32, 0i32);
    for c in text.chars() {
        match c {
            '{' => braces += 1,
            '}' => braces -= 1,
            '(' => parens += 1,
            ')' => parens -= 1,
            ';' if braces <= 0 && parens <= 0 => {
                out.push(cur.split_whitespace().collect::<Vec<_>>().join(" "));
                cur.clear();
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    out
}

/// Tokens of a type text: identifiers, `*`, and balanced `<…>` / `(…)` groups
/// kept whole.
fn tokens(text: &str) -> Vec<&str> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_whitespace() || c == b',' {
            i += 1;
        } else if c.is_ascii_alphanumeric() || c == b'_' {
            let s = i;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            out.push(&text[s..i]);
        } else if c == b'<' || c == b'(' {
            let close = if c == b'<' { b'>' } else { b')' };
            let s = i;
            let mut depth = 0;
            while i < b.len() {
                if b[i] == c {
                    depth += 1;
                } else if b[i] == close {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            out.push(&text[s..i.min(b.len())]);
        } else {
            out.push(&text[i..i + 1]);
            i += 1;
        }
    }
    out
}

/// Everything from the first attribute macro (`API_AVAILABLE`, `NS_SWIFT_NAME`,
/// `UI_APPEARANCE_SELECTOR`, `__attribute__`) on is dropped.
fn drop_macros<'a>(toks: &[&'a str]) -> Vec<&'a str> {
    let end = toks
        .iter()
        .position(|t| {
            t.starts_with("__attribute__")
                || (t.len() > 2 && t.contains('_') && t.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_') && !t.starts_with("__"))
        })
        .unwrap_or(toks.len());
    toks[..end].to_vec()
}

fn is_ident(t: &str) -> bool {
    let mut cs = t.chars();
    cs.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') && cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// A class-like name: an identifier starting with an uppercase letter that is
/// not a C scalar or a by-value struct.
fn is_class_name(t: &str) -> bool {
    is_ident(t) && t.starts_with(|c: char| c.is_ascii_uppercase()) && scalar(t).is_none() && by_value_struct(t).is_none()
}

fn last_ident(text: &str) -> Option<String> {
    let toks = tokens(text);
    let toks = drop_macros(&toks);
    toks.iter().rev().find(|t| is_ident(t)).map(|t| t.to_string())
}

/// The text after a `{ … }` block, or the whole text without one.
fn after_braces(text: &str) -> &str {
    match text.rfind('}') {
        Some(k) => &text[k + 1..],
        None => text,
    }
}

fn skip_ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && b[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

/// The text inside a balanced `( … )` starting at `b[*i]`, `*i` left past it.
fn paren_group(b: &[u8], i: &mut usize) -> Option<String> {
    if b.get(*i) != Some(&b'(') {
        return None;
    }
    let start = *i + 1;
    let mut depth = 0;
    while *i < b.len() {
        match b[*i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    let inner = std::str::from_utf8(&b[start..*i]).ok()?.trim().to_string();
                    *i += 1;
                    return Some(inner);
                }
            }
            _ => {}
        }
        *i += 1;
    }
    None
}

/// The C and Foundation scalars with their 64-bit Apple encodings.
fn scalar(t: &str) -> Option<&'static str> {
    Some(match t {
        "void" => "v",
        "id" | "instancetype" => "@",
        "Class" => "#",
        "SEL" => ":",
        "BOOL" | "bool" | "_Bool" | "Boolean" => "B",
        "char" | "signed char" | "int8_t" => "c",
        "unsigned char" | "uint8_t" | "UInt8" => "C",
        "short" | "signed short" | "int16_t" | "SInt16" => "s",
        "unsigned short" | "uint16_t" | "UInt16" | "unichar" => "S",
        "int" | "signed int" | "signed" | "int32_t" | "SInt32" | "OSStatus" => "i",
        "unsigned int" | "unsigned" | "uint32_t" | "UInt32" | "FourCharCode" | "OSType" => "I",
        "long" | "signed long" | "long long" | "int64_t" | "SInt64" | "intptr_t" | "ssize_t" | "NSInteger" | "CFIndex" | "ptrdiff_t" => "q",
        "unsigned long" | "unsigned long long" | "uint64_t" | "UInt64" | "uintptr_t" | "size_t" | "NSUInteger" | "CFTypeID" | "CFOptionFlags" | "CFHashCode" => "Q",
        "float" | "Float32" | "UILayoutPriority" => "f",
        "double" | "Float64" | "CGFloat" | "NSTimeInterval" | "CFTimeInterval" | "CFAbsoluteTime" => "d",
        _ => return None,
    })
}

/// The by-value structs whose members are all one register class: the CoreGraphics /
/// Foundation / UIKit geometry an AAPCS64 call passes in `d` (an HFA of at most
/// four doubles) or `x` registers. `CGAffineTransform` (six doubles) and
/// `CATransform3D` are passed indirectly and are encoded for that.
fn by_value_struct(t: &str) -> Option<&'static str> {
    Some(match t {
        "CGPoint" => "{CGPoint=dd}",
        "CGSize" => "{CGSize=dd}",
        "CGVector" => "{CGVector=dd}",
        "CGRect" => "{CGRect={CGPoint=dd}{CGSize=dd}}",
        "NSRange" | "_NSRange" | "CFRange" => "{_NSRange=QQ}",
        "UIOffset" => "{UIOffset=dd}",
        "UIEdgeInsets" => "{UIEdgeInsets=dddd}",
        "NSDirectionalEdgeInsets" => "{NSDirectionalEdgeInsets=dddd}",
        "CGAffineTransform" => "{CGAffineTransform=dddddd}",
        "CATransform3D" => "{CATransform3D=dddddddddddddddd}",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/objc_sdk")
    }

    /// The hand-written fixture header: properties yield getter/setter pairs
    /// (`getter=` honoured, `readonly` no setter), methods encode by-value
    /// structs and class pointers, and an unknown type leaves the selector out.
    #[test]
    fn reads_the_fixture_header() {
        let sigs = signatures(&fixture_dir(), &[]);
        let get = |s: &str| sigs.get(s).cloned().flatten();
        assert_eq!(get("setAlpha:").as_deref(), Some("v@:d"));
        assert_eq!(get("alpha").as_deref(), Some("d@:"));
        assert_eq!(get("setFrame:").as_deref(), Some("v@:{CGRect={CGPoint=dd}{CGSize=dd}}"));
        assert_eq!(get("setTag:").as_deref(), Some("v@:q"));
        assert_eq!(get("isHidden").as_deref(), Some("B@:"));
        assert_eq!(get("setHidden:").as_deref(), Some("v@:B"));
        assert!(!sigs.contains_key("hidden"), "getter=isHidden replaces the plain getter");
        assert_eq!(get("superview").as_deref(), Some("@\"Widget\"@:"));
        assert!(!sigs.contains_key("setSuperview:"), "readonly has no setter");
        assert_eq!(get("insertSubview:atIndex:").as_deref(), Some("v@:@\"Widget\"q"));
        assert_eq!(get("setContentMode:").as_deref(), Some("v@:q"), "NS_ENUM resolves to its base");
        assert_eq!(get("animateWithDuration:animations:").as_deref(), Some("v@:d@?"));
        assert_eq!(get("setTitle:").as_deref(), Some("v@:@\"NSString\""), "a typed-enum string");
        assert_eq!(sigs.get("setOpaqueThing:"), Some(&None), "an unknown struct is not guessed");
        assert_eq!(get("convertRect:").as_deref(), Some("{CGRect={CGPoint=dd}{CGSize=dd}}@:{CGRect={CGPoint=dd}{CGSize=dd}}"));
    }

    #[test]
    fn disagreeing_declarations_are_dropped_but_classes_are_not_a_disagreement() {
        let mut out = HashMap::new();
        record(&mut out, "setView:".into(), Some("v@:@\"UIView\"".into()));
        record(&mut out, "setView:".into(), Some("v@:@\"NSView\"".into()));
        assert_eq!(out["setView:"].as_deref(), Some("v@:@\"UIView\""));
        record(&mut out, "setView:".into(), Some("v@:q".into()));
        assert_eq!(out["setView:"], None);
    }

    #[test]
    fn comments_and_preprocessor_lines_are_dropped() {
        let t = strip_comments("#import <X.h>\n- (void)a; // c\n/* multi\nline */ - (void)b;\n#define Y \\\n  1\n@property (copy) NSString *s; // \"quoted\"\n");
        assert!(!t.contains("import") && !t.contains("multi") && !t.contains("define"));
        assert!(t.contains("- (void)a;") && t.contains("- (void)b;") && t.contains("NSString *s;"));
    }
}
