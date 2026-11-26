//! Native method bindings from a verified JNI registration or a self-describing export.
use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::architecture::Architecture;
use object::{Object, ObjectKind, ObjectSection, ObjectSymbol, SectionKind, SymbolKind};

use crate::loader::kuna_dataptrs::{data_pointers, DataPointers};

mod trace;

/// A JNI name and the C declaration justified at this implementation address.
#[derive(Debug)]
pub struct NativeMethod {
    pub addr: u64,
    pub name: String,
    pub declaration: Option<String>,
    pub return_known: bool,
}

struct Image<'a> {
    file: &'a object::File<'a>,
    bytes: &'a [u8],
    pointers: OnceCell<DataPointers>,
}

impl Image<'_> {
    fn data(&self, va: u64) -> Option<&[u8]> {
        self.file.sections().find_map(|s| {
            let off = usize::try_from(va.checked_sub(s.address())?).ok()?;
            if s.address() == 0 || off as u64 >= s.size() {
                return None;
            }
            s.data().ok()?.get(off..)
        })
    }

    fn string(&self, va: u64) -> Option<String> {
        let data = self.data(va)?;
        let end = memchr::memchr(0, &data[..data.len().min(65536)])?;
        modified_utf8(&data[..end])
    }

    fn pointer(&self, va: u64, width: u32) -> Option<u64> {
        let pointers = &self
            .pointers
            .get_or_init(|| data_pointers(self.file, self.bytes))
            .pointers;
        if let Ok(i) = pointers.binary_search_by_key(&va, |p| p.slot) {
            let p = &pointers[i];
            return (u32::from(p.width) == width).then_some(p.target);
        }
        if self.file.kind() != ObjectKind::Executable {
            return None;
        }
        let bytes = self.data(va)?.get(..width as usize)?;
        let mut word = [0; 8];
        word.get_mut(..bytes.len())?.copy_from_slice(bytes);
        Some(u64::from_le_bytes(word))
    }

    fn code(&self, va: u64) -> bool {
        self.file.sections().any(|s| {
            s.kind() == SectionKind::Text
                && va
                    .checked_sub(s.address())
                    .is_some_and(|off| off < s.size())
        })
    }

    fn pc(&self, raw: u64) -> u64 {
        if self.file.architecture() == object::Architecture::Arm {
            raw & !1
        } else {
            raw
        }
    }

    fn entry(&self, raw: u64) -> Option<u64> {
        match self.file.architecture() {
            object::Architecture::Aarch64 if raw & 3 != 0 => return None,
            object::Architecture::Arm if raw & 3 == 2 => return None,
            _ => {}
        }
        let pc = self.pc(raw);
        self.code(pc).then_some(pc)
    }
}

/// JNI strings encode UTF-16 units separately, including both halves of a surrogate pair.
fn modified_utf8(bytes: &[u8]) -> Option<String> {
    let mut units = Vec::with_capacity(bytes.len());
    let mut iter = bytes.iter().copied();
    while let Some(b) = iter.next() {
        let mut continuation = || {
            let n = iter.next()?;
            (n & 0xc0 == 0x80).then_some(u16::from(n & 0x3f))
        };
        let unit = match b {
            1..=0x7f => u16::from(b),
            0xc0 if continuation()? == 0 => 0,
            0xc2..=0xdf => (u16::from(b & 0x1f) << 6) | continuation()?,
            0xe0..=0xef => {
                let n = (u16::from(b & 15) << 12) | (continuation()? << 6) | continuation()?;
                if n < 0x800 {
                    return None;
                }
                n
            }
            _ => return None,
        };
        units.push(unit);
    }
    String::from_utf16(&units).ok()
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Binding {
    class: String,
    method: String,
    signature: String,
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && !name
            .chars()
            .any(|c| c.is_control() || ".;[/()<>".contains(c))
}

fn valid_class(name: &str) -> bool {
    name.split('/').all(valid_name)
}

fn descriptor_type(input: &mut &str, allow_void: bool) -> Option<&'static str> {
    let before = *input;
    let bare = before.trim_start_matches('[');
    let dimensions = before.len() - bare.len();
    if dimensions > 255 {
        return None;
    }
    let (first, rest) = bare.as_bytes().split_first()?;
    *input = std::str::from_utf8(rest).ok()?;
    let (scalar, array) = match first {
        b'B' => ("jbyte", "jbyteArray"),
        b'C' => ("jchar", "jcharArray"),
        b'D' => ("jdouble", "jdoubleArray"),
        b'F' => ("jfloat", "jfloatArray"),
        b'I' => ("jint", "jintArray"),
        b'J' => ("jlong", "jlongArray"),
        b'S' => ("jshort", "jshortArray"),
        b'Z' => ("jboolean", "jbooleanArray"),
        b'V' if allow_void && dimensions == 0 => ("void", ""),
        b'L' => {
            let (class, rest) = input.split_once(';')?;
            if !valid_class(class) {
                return None;
            }
            *input = rest;
            let ty = match class {
                "java/lang/String" => "jstring",
                "java/lang/Class" => "jclass",
                "java/lang/Throwable" => "jthrowable",
                _ => "jobject",
            };
            (ty, "jobjectArray")
        }
        _ => return None,
    };
    Some(match dimensions {
        0 => scalar,
        1 => array,
        _ => "jobjectArray",
    })
}

fn parameters(mut text: &str) -> Option<Vec<&'static str>> {
    let mut args = Vec::new();
    while !text.is_empty() && args.len() < 255 {
        args.push(descriptor_type(&mut text, false)?);
    }
    text.is_empty().then_some(args)
}

fn signature(text: &str) -> Option<(Vec<&'static str>, &'static str)> {
    let (params, mut result) = text.strip_prefix('(')?.split_once(')')?;
    let args = parameters(params)?;
    let ty = descriptor_type(&mut result, true)?;
    result.is_empty().then_some((args, ty))
}

fn reference(ty: &str) -> bool {
    matches!(ty, "jstring" | "jobject" | "jclass" | "jthrowable") || ty.ends_with("Array")
}

fn declaration(name: &str, args: &[&str], ret: &str) -> String {
    let mut params = String::from("JNIEnv *env, jobject receiver");
    for (i, ty) in args.iter().enumerate() {
        use std::fmt::Write;
        let _ = write!(params, ", {ty} arg{i}");
    }
    format!("{ret} {name}({params});")
}

fn escape(s: &str) -> String {
    let mut out = String::new();
    for unit in s.encode_utf16() {
        match unit {
            47 => out.push('_'),
            95 => out.push_str("_1"),
            59 => out.push_str("_2"),
            91 => out.push_str("_3"),
            48..=57 | 65..=90 | 97..=122 => out.push(char::from_u32(u32::from(unit)).unwrap()),
            _ => {
                use std::fmt::Write;
                let _ = write!(out, "_0{unit:04x}");
            }
        }
    }
    out
}

fn unescape(s: &str) -> Option<String> {
    let mut units = Vec::new();
    let mut bytes = s.as_bytes().iter().copied().peekable();
    while let Some(c) = bytes.next() {
        let unit = match c {
            b'_' => match bytes.peek().copied() {
                Some(d @ b'0'..=b'3') => {
                    bytes.next();
                    match d {
                        b'1' => 95,
                        b'2' => 59,
                        b'3' => 91,
                        _ => {
                            let mut u = 0;
                            for _ in 0..4 {
                                u = (u << 4) | char::from(bytes.next()?).to_digit(16)? as u16;
                            }
                            u
                        }
                    }
                }
                _ => 47,
            },
            c if c.is_ascii_alphanumeric() => u16::from(c),
            _ => return None,
        };
        units.push(unit);
    }
    String::from_utf16(&units).ok()
}

fn exported(addr: u64, name: &str) -> Option<NativeMethod> {
    let suffix = name.strip_prefix("Java_")?;
    // A separator may precede an escape, e.g. pkg/_Class or an array descriptor.
    for (i, _) in suffix.match_indices("__") {
        let Some(path) = unescape(&suffix[..i]) else {
            continue;
        };
        if !valid_class(&path) || !path.contains('/') {
            continue;
        }
        let Some(params) = unescape(&suffix[i + 2..]).and_then(|s| parameters(&s)) else {
            continue;
        };
        if params.iter().any(|t| reference(t)) {
            return Some(NativeMethod {
                addr,
                name: name.into(),
                declaration: Some(declaration(name, &params, "void")),
                return_known: false,
            });
        }
    }
    None
}

fn read_table(
    image: &Image,
    site: trace::Registration,
    width: u32,
    budget: &mut usize,
) -> Option<Vec<(u64, Binding)>> {
    let class = image.string(site.class)?;
    if !valid_class(&class) || site.count == 0 || site.count > 65535 {
        return None;
    }
    let stride = u64::from(width) * 3;
    let size = site.count.checked_mul(stride)?;
    if size > image.data(site.table)?.len() as u64 {
        return None;
    }
    let mut rows = Vec::new();
    for i in 0..site.count {
        let row = site.table.checked_add(i * stride)?;
        let name = image.string(image.pointer(row, width)?)?;
        let desc = image.string(image.pointer(row.checked_add(width.into())?, width)?)?;
        let addr = image.entry(image.pointer(row.checked_add(2 * u64::from(width))?, width)?)?;
        if !valid_name(&name) || signature(&desc).is_none() {
            return None;
        }
        let cost = class.len() + name.len() + desc.len();
        *budget = budget.saturating_sub(cost);
        if *budget == 0 {
            return None;
        }
        rows.push((
            addr,
            Binding {
                class: class.clone(),
                method: name,
                signature: desc,
            },
        ));
    }
    Some(rows)
}

/// Inspect Android JNI exports and registration tables reached from JNI_OnLoad.
pub fn registrations(
    file: &object::File,
    bytes: &[u8],
    arch: &Architecture,
    registered: bool,
) -> Vec<NativeMethod> {
    if !file.is_little_endian() {
        return Vec::new();
    }
    let image = Image {
        file,
        bytes,
        pointers: OnceCell::new(),
    };
    let mut exports: BTreeMap<u64, BTreeSet<&str>> = BTreeMap::new();
    let mut bindings: BTreeMap<u64, BTreeSet<Binding>> = BTreeMap::new();
    let width = if file.is_64() { 8 } else { 4 };
    let mut budget = 16 << 20;
    for sym in file
        .dynamic_symbols()
        .filter(|s| s.is_definition() && !s.is_local() && s.kind() == SymbolKind::Text)
    {
        let Ok(name) = sym.name() else { continue };
        if !(registered && name == "JNI_OnLoad" || name.starts_with("Java_")) {
            continue;
        }
        let Some(addr) = image.entry(sym.address()) else {
            continue;
        };
        if name == "JNI_OnLoad" {
            for site in trace::sites(&image, arch, sym.address(), sym.size()) {
                if let Some(rows) = read_table(&image, site, width, &mut budget) {
                    for (addr, binding) in rows {
                        bindings.entry(addr).or_default().insert(binding);
                    }
                }
            }
        } else {
            exports.entry(addr).or_default().insert(name);
        }
    }
    if budget == 0 {
        bindings.clear();
    }
    bind_methods(&bindings, &exports).into_values().collect()
}

fn bind_methods(
    bindings: &BTreeMap<u64, BTreeSet<Binding>>,
    exports: &BTreeMap<u64, BTreeSet<&str>>,
) -> BTreeMap<u64, NativeMethod> {
    let mut methods: BTreeMap<_, _> = exports
        .iter()
        .filter(|(_, names)| names.len() == 1)
        .filter_map(|(&addr, names)| Some((addr, exported(addr, names.first()?)?)))
        .collect();
    let mut targets = BTreeMap::new();
    let mut sources = BTreeMap::new();
    let mut overloads = BTreeMap::new();
    for group in bindings.values() {
        for b in group {
            *targets.entry(b).or_insert(0) += 1;
            let inputs = b.signature.split_once(')').unwrap().0;
            *sources.entry((&b.class, &b.method, inputs)).or_insert(0) += 1;
            overloads
                .entry((&b.class, &b.method))
                .or_insert_with(BTreeSet::new)
                .insert(&b.signature);
        }
    }
    for (&addr, group) in bindings {
        if group.len() != 1 || group.iter().any(|b| targets[b] != 1) {
            methods.remove(&addr);
            continue;
        }
        let b = group.first().unwrap();
        let inputs = b.signature.split_once(')').unwrap().0;
        if sources[&(&b.class, &b.method, inputs)] != 1 {
            methods.remove(&addr);
            continue;
        }
        let mut name = format!("Java_{}_{}", escape(&b.class), escape(&b.method));
        let long_name = || format!("{}__{}", name, escape(&inputs[1..]));
        if let Some(names) = exports.get(&addr) {
            let long = long_name();
            if names.iter().any(|&n| n != name && n != long) {
                methods.remove(&addr);
                continue;
            }
        }
        if overloads[&(&b.class, &b.method)].len() > 1 {
            name = long_name();
        }
        let (args, ret) = signature(&b.signature).unwrap();
        let ordinary = reference(ret) || args.iter().any(|ty| reference(ty));
        let decl = ordinary.then(|| declaration(&name, &args, ret));
        methods.insert(
            addr,
            NativeMethod {
                addr,
                name,
                declaration: decl,
                return_known: true,
            },
        );
    }
    methods
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_names_retain_modified_utf8_and_jni_escapes() {
        let s = modified_utf8(b"p/\xed\xa0\xbd\xed\xb1\x8d_A").unwrap();
        assert_eq!(escape(&s), "p__0d83d_0dc4d_1A");
        assert_eq!(unescape(&escape(&s)).as_deref(), Some(s.as_str()));
        assert_eq!(modified_utf8(b"\xed\xa0\xbd"), None);
        assert_eq!(modified_utf8(b"\xe0\x80\xaf"), None);
    }

    #[test]
    fn exact_descriptors_preserve_primitive_arrays_and_reject_trailing_or_void_arguments() {
        let (args, result) = signature("(J[Z[[DLjava/lang/String;)[I").unwrap();
        assert_eq!(args, ["jlong", "jbooleanArray", "jobjectArray", "jstring"]);
        assert_eq!(result, "jintArray");
        for s in ["([V)V", "()II", "(V)V", "(Lmissing)I", "(Lx//y;)I"] {
            assert!(signature(s).is_none(), "{s}");
        }
    }

    #[test]
    fn only_proven_ordinary_long_exports_seed_inputs() {
        assert!(exported(1, "Java_test_C_foo").is_none());
        assert!(exported(1, "Java_test_C_foo__J").is_none());
        let method = exported(1, "Java_test_C_foo___3B").unwrap();
        assert!(!method.return_known);
        assert!(method.declaration.unwrap().contains("jbyteArray"));
    }

    #[test]
    fn conflicting_export_aliases_cannot_choose_a_machine_prototype() {
        let ints = "Java_p_C_read___3I";
        let bytes = "Java_p_C_read___3B";
        let exports = BTreeMap::from([
            (1, BTreeSet::from([ints, bytes])),
            (2, BTreeSet::from([ints])),
            (3, BTreeSet::from([bytes])),
        ]);
        let registered = Binding {
            class: "p/C".into(),
            method: "read".into(),
            signature: "([B)I".into(),
        };
        let bindings = BTreeMap::from([(2, BTreeSet::from([registered.clone()]))]);
        let methods = bind_methods(&bindings, &exports);
        assert_eq!(methods.keys().copied().collect::<Vec<_>>(), [3]);
        let bindings = BTreeMap::from([(3, BTreeSet::from([registered]))]);
        assert!(bind_methods(&bindings, &exports)[&3].return_known);
    }

    #[test]
    fn native_entry_requires_a_valid_instruction_boundary() {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/android-jni/aarch64-linux-android.so"
        ));
        let file = object::File::parse(&bytes[..]).unwrap();
        let entry = file
            .dynamic_symbols()
            .find(|s| s.name() == Ok("JNI_OnLoad"))
            .unwrap()
            .address();
        let image = Image {
            file: &file,
            bytes,
            pointers: OnceCell::new(),
        };
        assert_eq!(image.entry(entry), Some(entry));
        assert_eq!(image.entry(entry + 1), None);
    }

    #[test]
    fn registration_conflicts_keep_the_implementation_ambiguous() {
        let binding = |method: &str, sig: &str| Binding {
            class: "p/C".into(),
            method: method.into(),
            signature: sig.into(),
        };
        let a = binding("a", "([I)I");
        let b = binding("a", "([D)I");
        let duplicate = binding("duplicate", "(I)I");
        let rows = BTreeMap::from([
            (1, BTreeSet::from([a, b])),
            (2, BTreeSet::from([duplicate.clone()])),
            (3, BTreeSet::from([duplicate])),
            (4, BTreeSet::from([binding("distinct", "([I)I")])),
            (5, BTreeSet::from([binding("distinct", "([D)I")])),
            (6, BTreeSet::from([binding("returns", "([I)I")])),
            (7, BTreeSet::from([binding("returns", "([I)D")])),
        ]);
        let methods = bind_methods(&rows, &BTreeMap::new());
        assert_eq!(methods.keys().copied().collect::<Vec<_>>(), [4, 5]);
        assert_eq!(methods[&4].name, "Java_p_C_distinct___3I");
        assert_eq!(methods[&5].name, "Java_p_C_distinct___3D");
    }
}
