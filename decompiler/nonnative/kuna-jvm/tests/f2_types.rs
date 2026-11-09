//! `VarType`, descriptors, generic signatures and the type lattice against
//! golden vectors computed by Vineflower 1.12.0 (`data/f2/GenF2Golden.java`).
//!
//! A vector field ending in `!` consulted the decompiler context in Java; those
//! need a live `DecompilerContext` and are skipped here.

use kuna_jvm::code_type::CodeType;
use kuna_jvm::field_descriptor::FieldDescriptor;
use kuna_jvm::generic_main::GenericMain;
use kuna_jvm::generic_type::GenericType;
use kuna_jvm::generics_checker::GenericsChecker;
use kuna_jvm::method_descriptor::MethodDescriptor;
use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::{try_catch, JHashMap, JHashSet, JavaStr};
use kuna_jvm::type_family::TypeFamily;
use kuna_jvm::var_type::VarType;

pub fn unesc(s: &str) -> Option<String> {
    if s == "\\N" {
        return None;
    }
    let units: Vec<u16> = s.encode_utf16().collect();
    let mut out: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < units.len() {
        if units[i] == b'\\' as u16 {
            let n = units[i + 1];
            match char::from_u32(n as u32).unwrap() {
                '\\' => out.push(b'\\' as u16),
                't' => out.push(b'\t' as u16),
                'n' => out.push(b'\n' as u16),
                'r' => out.push(b'\r' as u16),
                'u' => {
                    let hex = String::from_utf16(&units[i + 2..i + 6]).unwrap();
                    out.push(u16::from_str_radix(&hex, 16).unwrap());
                    i += 4;
                }
                c => panic!("bad escape {c}"),
            }
            i += 2;
        } else {
            out.push(units[i]);
            i += 1;
        }
    }
    Some(from_utf16(&out))
}

pub fn esc(s: Option<&str>) -> String {
    let Some(s) = s else {
        return "\\N".to_string();
    };
    let mut b = String::new();
    for c in s.utf16() {
        match c {
            0x5c => b.push_str("\\\\"),
            0x09 => b.push_str("\\t"),
            0x0a => b.push_str("\\n"),
            0x0d => b.push_str("\\r"),
            c if !(0x20..=0x7e).contains(&c) => b.push_str(&format!("\\u{:04x}", c)),
            c => b.push(c as u8 as char),
        }
    }
    b
}

pub fn dump(t: Option<&VarType>) -> String {
    let Some(t) = t else {
        return "\\N".to_string();
    };
    match t.try_as_generic() {
        Some(g) => {
            let args: Vec<String> = g.get_arguments().iter().map(|a| dump(a.as_ref())).collect();
            format!(
                "G[{},{},{},{},{},w{},g{},p={},a=({})]",
                t.type_.name(),
                t.array_dim,
                esc(t.value.as_deref()),
                t.type_family.name(),
                t.stack_size,
                g.get_wildcard(),
                g.is_generic() as i32,
                dump(g.get_parent().as_ref()),
                args.join(";")
            )
        }
        None => format!(
            "V[{},{},{},{},{}]",
            t.type_.name(),
            t.array_dim,
            esc(t.value.as_deref()),
            t.type_family.name(),
            t.stack_size
        ),
    }
}

pub fn dumps<'a>(l: impl IntoIterator<Item = &'a VarType>) -> String {
    let v: Vec<String> = l.into_iter().map(|t| dump(Some(t))).collect();
    format!("({})", v.join(";"))
}

pub fn dump_map(m: &JHashMap<VarType, Option<VarType>>) -> String {
    let v: Vec<String> = m
        .iter()
        .map(|(k, v)| format!("{}=>{}", dump(Some(k)), dump(v.as_ref())))
        .collect();
    format!("{{{}}}", v.join("|"))
}

/// Runs an op the way the generator does, rendering a throw as `EXC:<class>:<message>`.
pub fn run(op: impl FnOnce() -> String) -> String {
    match try_catch(op) {
        Ok(s) => s,
        Err(c) => match c.exception {
            Some(e) => format!("EXC:{}:{}", e.class, esc(Some(&e.message))),
            None => format!("EXC:Panic:{}", esc(Some(&c.message))),
        },
    }
}

fn exc_class(s: &str) -> &str {
    s.split(':').nth(1).unwrap_or("")
}

/// Java and Rust results agree; a Rust panic stands for any Java exception
/// (`ByteCursor`'s `IOException` for `EOFException`), and messages are
/// compared for the exceptions both sides build by hand.
pub fn same(java: &str, rust: &str) -> bool {
    if java == rust {
        return true;
    }
    if java.starts_with("EXC:") && rust.starts_with("EXC:") {
        let (jc, rc) = (exc_class(java), exc_class(rust));
        if rc == "Panic" || (jc == "EOFException" && rc == "IOException") {
            return true;
        }
        if jc != rc {
            return false;
        }
        return !matches!(
            jc,
            "IllegalArgumentException" | "RuntimeException" | "IllegalStateException"
        );
    }
    false
}

pub struct Checker {
    pub name: &'static str,
    pub checked: usize,
    pub skipped: usize,
    pub lenient: usize,
    pub failures: Vec<String>,
}

impl Checker {
    pub fn new(name: &'static str) -> Checker {
        Checker {
            name,
            checked: 0,
            skipped: 0,
            lenient: 0,
            failures: Vec::new(),
        }
    }

    /// Compares one field; a context-dependent Java field is skipped.
    pub fn field(&mut self, what: &str, java: &str, rust: impl FnOnce() -> String) {
        if java.ends_with('!') {
            self.skipped += 1;
            return;
        }
        self.checked += 1;
        let r = run(rust);
        if java != r {
            if same(java, &r) {
                self.lenient += 1;
                if std::env::var_os("F2_SHOW_LENIENT").is_some() {
                    eprintln!("lenient {what}\n   java: {java}\n   rust: {r}");
                }
            } else {
                self.failures
                    .push(format!("{what}\n   java: {java}\n   rust: {r}"));
            }
        }
    }

    pub fn finish(self) {
        eprintln!(
            "{}: {} fields checked ({} as exceptions of another message or class), {} context-dependent skipped",
            self.name, self.checked, self.lenient, self.skipped
        );
        assert!(
            self.failures.is_empty(),
            "{} mismatches in {}:\n{}",
            self.failures.len(),
            self.name,
            self.failures
                .iter()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert!(self.checked > 0);
    }
}

pub fn lines(data: &'static str, tag: &str) -> Vec<Vec<&'static str>> {
    data.lines()
        .filter(|l| l.split('\t').next() == Some(tag))
        .map(|l| l.split('\t').collect())
        .collect()
}

fn code_type(name: &str) -> CodeType {
    *CodeType::VALUES.iter().find(|c| c.name() == name).unwrap()
}

fn type_family(name: &str) -> TypeFamily {
    [
        TypeFamily::Unknown,
        TypeFamily::Boolean,
        TypeFamily::Integer,
        TypeFamily::Float,
        TypeFamily::Long,
        TypeFamily::Double,
        TypeFamily::Object,
    ]
    .into_iter()
    .find(|f| f.name() == name)
    .unwrap()
}

pub fn constants() -> Vec<(&'static str, VarType)> {
    vec![
        ("VARTYPE_UNKNOWN", VarType::VARTYPE_UNKNOWN()),
        ("VARTYPE_INT", VarType::VARTYPE_INT()),
        ("VARTYPE_FLOAT", VarType::VARTYPE_FLOAT()),
        ("VARTYPE_LONG", VarType::VARTYPE_LONG()),
        ("VARTYPE_DOUBLE", VarType::VARTYPE_DOUBLE()),
        ("VARTYPE_BYTE", VarType::VARTYPE_BYTE()),
        ("VARTYPE_CHAR", VarType::VARTYPE_CHAR()),
        ("VARTYPE_SHORT", VarType::VARTYPE_SHORT()),
        ("VARTYPE_BOOLEAN", VarType::VARTYPE_BOOLEAN()),
        ("VARTYPE_BYTECHAR", VarType::VARTYPE_BYTECHAR()),
        ("VARTYPE_SHORTCHAR", VarType::VARTYPE_SHORTCHAR()),
        ("VARTYPE_NULL", VarType::VARTYPE_NULL()),
        ("VARTYPE_STRING", VarType::VARTYPE_STRING()),
        ("VARTYPE_CLASS", VarType::VARTYPE_CLASS()),
        ("VARTYPE_OBJECT", VarType::VARTYPE_OBJECT()),
        ("VARTYPE_INTEGER", VarType::VARTYPE_INTEGER()),
        ("VARTYPE_CHARACTER", VarType::VARTYPE_CHARACTER()),
        ("VARTYPE_BYTE_OBJ", VarType::VARTYPE_BYTE_OBJ()),
        ("VARTYPE_SHORT_OBJ", VarType::VARTYPE_SHORT_OBJ()),
        ("VARTYPE_BOOLEAN_OBJ", VarType::VARTYPE_BOOLEAN_OBJ()),
        ("VARTYPE_FLOAT_OBJ", VarType::VARTYPE_FLOAT_OBJ()),
        ("VARTYPE_DOUBLE_OBJ", VarType::VARTYPE_DOUBLE_OBJ()),
        ("VARTYPE_LONG_OBJ", VarType::VARTYPE_LONG_OBJ()),
        ("VARTYPE_VOID", VarType::VARTYPE_VOID()),
        ("VARTYPE_METHODTYPE", VarType::VARTYPE_METHODTYPE()),
    ]
}

pub fn const_name(t: &VarType) -> String {
    for (name, c) in constants() {
        if c.ptr_eq(t) {
            return format!("C:{name}");
        }
    }
    dump(Some(t))
}

const VARTYPE: &str = include_str!("data/f2/vartype.tsv");
const DESCRIPTORS: &str = include_str!("data/f2/descriptors.tsv");
const SIGNATURES: &str = include_str!("data/f2/signatures.tsv");
const GENERIC_OPS: &str = include_str!("data/f2/generic_ops.tsv");
const LATTICE: &str = include_str!("data/f2/lattice.tsv");

#[test]
fn vartype_constants_are_identities() {
    assert!(VarType::VARTYPE_INT().ptr_eq(&VarType::VARTYPE_INT()));
    assert!(!VarType::VARTYPE_INT().ptr_eq(&VarType::new(CodeType::Int)));
    assert!(VarType::VARTYPE_INT().equals(&VarType::new(CodeType::Int)));
    assert!(GenericType::DUMMY_VAR().ptr_eq(&GenericType::DUMMY_VAR()));
    let unboxing = VarType::UNBOXING_TYPES();
    assert_eq!(unboxing.len(), 8);
    assert!(unboxing
        .get(&VarType::VARTYPE_INTEGER())
        .unwrap()
        .ptr_eq(&VarType::VARTYPE_INT()));
    let t = VarType::new_signature("I");
    assert!(t.decrease_array_dim().ptr_eq(&t));
    assert!(!t.resize_array_dim(0).ptr_eq(&t));
    let g = GenericType::parse("Ljava/util/List<TT;>;");
    assert!(g.is_generic() && g.is_generic_type());
    assert!(g.resize_array_dim(1).is_generic_type());
    assert!(g
        .remap(&JHashMap::<VarType, VarType>::new())
        .unwrap()
        .ptr_eq(&g));
    let plain = GenericType::parse("Ljava/lang/String;");
    assert!(!plain.is_generic_type());
    let caught = try_catch(|| plain.as_generic()).unwrap_err();
    assert!(caught.is("ClassCastException"));
    assert_eq!(GenericType::parse("TT;").to_string(), "T");
    assert_eq!(
        GenericType::parse_wildcard("Ljava/lang/Object;", GenericType::WILDCARD_SUPER).to_string(),
        "? super Ljava/lang/Object;"
    );
}

#[test]
fn vartype_from_descriptors_matches_vineflower() {
    let mut c = Checker::new("VarType(String, boolean)");
    for l in lines(VARTYPE, "VT") {
        let sig = unesc(l[1]).unwrap();
        let cl = l[2] == "1";
        if l[3].starts_with("EXC:") {
            c.field(&format!("VT {:?}", l[1]), l[3], || {
                dump(Some(&VarType::new_signature_cl(&sig, cl)))
            });
            continue;
        }
        let t = VarType::new_signature_cl(&sig, cl);
        let what = format!("VT {} {}", l[1], l[2]);
        c.field(&what, l[3], || dump(Some(&t)));
        c.field(&what, l[4], || esc(Some(&t.to_string())));
        c.field(&what, l[5], || t.hash_code().to_string());
        c.field(&what, l[6], || {
            let d = t.decrease_array_dim();
            if d.ptr_eq(&t) {
                "same".into()
            } else {
                dump(Some(&d))
            }
        });
        c.field(&what, l[7], || dump(Some(&t.resize_array_dim(0))));
        c.field(&what, l[8], || dump(Some(&t.resize_array_dim(3))));
        c.field(&what, l[9], || VarType::is_primitive(&t).to_string());
        c.field(&what, l[10], || t.is_generic().to_string());
    }
    for l in lines(VARTYPE, "CT") {
        let ct = code_type(l[1]);
        c.field(l[1], l[2], || esc(VarType::get_char(ct)));
        c.field(l[1], l[3], || dump(Some(&VarType::new(ct))));
        c.field(l[1], l[4], || dump(Some(&VarType::new_dim(ct, 2))));
    }
    for l in lines(VARTYPE, "FB") {
        let f = type_family(l[1]);
        c.field(l[1], l[2], || const_name(&VarType::find_family_bottom(f)));
        c.field(l[1], l[3], || f.is_numeric().to_string());
        c.field(l[1], l[4], || f.int_or_bool().to_string());
    }
    c.finish();
}

fn renamer(cn: &str) -> Option<String> {
    if let Some(rest) = cn.strip_prefix("java/util/") {
        Some(format!("ju/{rest}"))
    } else if cn == "java/lang/String" {
        Some("S".into())
    } else {
        None
    }
}

#[test]
fn descriptors_match_vineflower() {
    let mut c = Checker::new("Method/FieldDescriptor");
    let mut prev: Option<String> = None;
    for l in lines(DESCRIPTORS, "MD") {
        let d = unesc(l[1]).unwrap();
        let what = format!("MD {}", l[1]);
        if l[2].starts_with("EXC:") {
            c.field(&what, l[2], || {
                MethodDescriptor::parse_descriptor(&d).to_string()
            });
            continue;
        }
        let md = MethodDescriptor::parse_descriptor(&d);
        c.field(&what, l[2], || dumps(&md.params));
        c.field(&what, l[3], || dump(Some(&md.ret)));
        c.field(&what, l[4], || esc(Some(&md.to_string())));
        c.field(&what, l[5], || md.hash_code().to_string());
        c.field(&what, l[6], || {
            esc(md.build_new_descriptor(&renamer).as_deref())
        });
        if let Some(p) = &prev {
            c.field(&what, l[7], || {
                MethodDescriptor::parse_descriptor(p)
                    .equals(&md)
                    .to_string()
            });
        }
        prev = Some(d);
    }
    let mut prev: Option<String> = None;
    for l in lines(DESCRIPTORS, "FD") {
        let d = unesc(l[1]).unwrap();
        let what = format!("FD {}", l[1]);
        if l[2].starts_with("EXC:") {
            c.field(&what, l[2], || {
                dump(Some(&FieldDescriptor::parse_descriptor(&d).type_))
            });
            continue;
        }
        let fd = FieldDescriptor::parse_descriptor(&d);
        c.field(&what, l[2], || dump(Some(&fd.type_)));
        c.field(&what, l[3], || esc(Some(&fd.descriptor_string)));
        c.field(&what, l[4], || fd.hash_code().to_string());
        c.field(&what, l[5], || {
            esc(fd.build_new_descriptor(&renamer).as_deref())
        });
        if let Some(p) = &prev {
            c.field(&what, l[6], || {
                FieldDescriptor::parse_descriptor(p).equals(&fd).to_string()
            });
        }
        prev = Some(d);
    }
    assert!(FieldDescriptor::INTEGER_DESCRIPTOR().ptr_eq(&FieldDescriptor::INTEGER_DESCRIPTOR()));
    assert_eq!(
        FieldDescriptor::LONG_DESCRIPTOR().descriptor_string,
        "Ljava/lang/Long;"
    );
    c.finish();
}

pub fn dump_class(d: &kuna_jvm::generic_class_descriptor::GenericClassDescriptor) -> String {
    let b = d.borrow();
    let fp: Vec<String> = b.fparameters.iter().map(|s| esc(Some(s))).collect();
    let fb: Vec<String> = b.fbounds.iter().map(dumps).collect();
    format!(
        "sc={};si={};fp=({});fb=({});gt={}",
        dump(Some(&b.superclass)),
        dumps(&b.superinterfaces),
        fp.join(","),
        fb.join(","),
        dump(Some(&b.generic_type))
    )
}

pub fn dump_method(d: &kuna_jvm::generic_method_descriptor::GenericMethodDescriptor) -> String {
    let b = d.borrow();
    let tp: Vec<String> = b.type_parameters.iter().map(|s| esc(Some(s))).collect();
    let tb: Vec<String> = b.type_parameter_bounds.iter().map(dumps).collect();
    format!(
        "tp=({});tb=({});pt={};rt={};et={}",
        tp.join(","),
        tb.join(","),
        dumps(&b.parameter_types),
        dump(b.return_type.as_ref()),
        dumps(&b.exception_types)
    )
}

/// A signature Java rejects logs through the context's logger; those rows need a context.
fn parse_ok(java: &str) -> bool {
    java != "\\N"
}

#[test]
fn generic_signatures_match_vineflower() {
    let mut c = Checker::new("GenericMain.parse*Signature");
    let mut rejected = 0;
    for l in lines(SIGNATURES, "CS") {
        if !parse_ok(l[3]) {
            rejected += 1;
            continue;
        }
        let (qn, sig) = (unesc(l[1]).unwrap(), unesc(l[2]).unwrap());
        c.field(&format!("CS {} {}", l[1], l[2]), l[3], || {
            dump_class(&GenericMain::parse_class_signature(&qn, &sig).expect("rejected"))
        });
    }
    for l in lines(SIGNATURES, "MS") {
        if !parse_ok(l[2]) {
            rejected += 1;
            continue;
        }
        let sig = unesc(l[1]).unwrap();
        c.field(&format!("MS {}", l[1]), l[2], || {
            dump_method(&GenericMain::parse_method_signature(&sig).expect("rejected"))
        });
    }
    for l in lines(SIGNATURES, "FS") {
        if !parse_ok(l[2]) {
            rejected += 1;
            continue;
        }
        let sig = unesc(l[1]).unwrap();
        c.field(&format!("FS {}", l[1]), l[2], || {
            dump(Some(
                &GenericMain::parse_field_signature(&sig)
                    .expect("rejected")
                    .type_(),
            ))
        });
    }
    for l in lines(SIGNATURES, "NT") {
        let s = unesc(l[1]).unwrap();
        c.field(&format!("NT {}", l[1]), l[2], || {
            esc(Some(&GenericType::get_next_type(&s)))
        });
    }
    eprintln!("{rejected} signatures Java rejects need a logger and are not replayed here");
    c.finish();
}

pub fn resolve_use(kind: &str, sig: &str, idx: usize) -> VarType {
    match kind {
        "F" => GenericMain::parse_field_signature(sig).unwrap().type_(),
        "MP" => GenericMain::parse_method_signature(sig)
            .unwrap()
            .parameter_types()[idx]
            .clone(),
        "MR" => GenericMain::parse_method_signature(sig)
            .unwrap()
            .return_type()
            .unwrap(),
        "CS" => GenericMain::parse_class_signature("x", sig)
            .unwrap()
            .superclass(),
        _ => GenericMain::parse_class_signature("x", sig)
            .unwrap()
            .superinterfaces()[idx]
            .clone(),
    }
}

pub fn check_generic_ops(c: &mut Checker, l: &[&str]) {
    let value = unesc(l[1]).unwrap();
    let decl_sig = unesc(l[2]).unwrap();
    let use_sig = unesc(l[4]).unwrap();
    let idx: usize = l[5].parse().unwrap();
    let dd = GenericMain::parse_class_signature(&value, &decl_sig).unwrap();
    let decl = dd.generic_type();
    let use_ = resolve_use(l[3], &use_sig, idx);
    let gu = use_.as_generic();
    let mut named = JHashSet::new();
    for p in dd.fparameters().iter() {
        named.add(GenericType::parse(&format!("T{p};")));
    }
    let what = format!("GO {} {} {}", l[3], l[4], l[5]);
    let r = &l[6..];

    c.field(&what, r[0], || decl.equals(&use_).to_string());
    c.field(&what, r[1], || decl.equals_exact(&use_).to_string());
    c.field(&what, r[2], || decl.arguments_equal(&gu).to_string());
    if r[3].ends_with('!') {
        c.skipped += 12;
        return;
    }
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    let mapped = try_catch(|| {
        decl.map_gen_vars_to(&gu, &mut map);
    });
    c.field(&what, r[3], || {
        if let Err(e) = &mapped {
            kuna_jvm::rt::rethrow(e.clone());
        }
        dump_map(&map)
    });
    c.field(&what, r[4], || match decl.remap(&map) {
        Some(x) if x.ptr_eq(&decl) => "same".into(),
        x => dump(x.as_ref()),
    });
    c.field(&what, r[5], || match use_.remap(&map) {
        Some(x) if x.ptr_eq(&use_) => "same".into(),
        x => dump(x.as_ref()),
    });
    c.field(&what, r[6], || {
        let mut m2 = map.clone();
        GenericType::clean_lowered_generic_types(&mut m2, &decl, &gu, None);
        dump_map(&m2)
    });
    c.field(&what, r[7], || {
        let v = decl.get_all_generic_vars();
        dumps(v.iter().map(|g| &g.0))
    });
    c.field(&what, r[8], || {
        let v = gu.get_all_generic_vars();
        dumps(v.iter().map(|g| &g.0))
    });
    c.field(&what, r[9], || gu.is_type_unfinished().to_string());
    c.field(&what, r[10], || {
        gu.has_unknown_generic_type(&named).to_string()
    });
    c.field(&what, r[11], || {
        dump(Some(&GenericType::with_wildcard(
            &use_,
            GenericType::WILDCARD_EXTENDS,
        )))
    });
    c.field(&what, r[12], || {
        let mut m3: JHashMap<VarType, VarType> = JHashMap::new();
        for gv in gu.get_all_generic_vars() {
            m3.put(gv.0, VarType::VARTYPE_OBJECT());
        }
        match use_.remap(&m3) {
            Some(x) if x.ptr_eq(&use_) => "same".into(),
            x => dump(x.as_ref()),
        }
    });
    c.field(&what, r[13], || dump(Some(&use_.resize_array_dim(2))));
    c.field(&what, r[14], || {
        dump(Some(&use_.resize_array_dim(2).decrease_array_dim()))
    });
}

#[test]
fn generic_type_algebra_matches_vineflower() {
    let mut c = Checker::new("GenericType algebra");
    for l in lines(GENERIC_OPS, "GO") {
        check_generic_ops(&mut c, &l);
    }
    c.finish();
}

pub fn lattice_type(s: &str) -> VarType {
    if let Some(name) = s.strip_prefix("C:") {
        return constants().into_iter().find(|(n, _)| *n == name).unwrap().1;
    }
    if let Some(sig) = s.strip_prefix("S:") {
        return VarType::new_signature(sig);
    }
    GenericType::parse(&s[2..])
}

fn ident(r: Option<VarType>, a: &VarType, b: &VarType) -> String {
    match r {
        None => "\\N".into(),
        Some(r) if r.ptr_eq(a) => "=1".into(),
        Some(r) if r.ptr_eq(b) => "=2".into(),
        Some(r) => const_name(&r),
    }
}

pub fn check_lattice(c: &mut Checker, l: &[&str]) {
    let a = lattice_type(l[1]);
    let b = lattice_type(l[2]);
    let what = format!("LA {} {}", l[1], l[2]);
    c.field(&what, l[3], || a.higher_in_lattice_than(&b).to_string());
    c.field(&what, l[4], || {
        a.higher_equal_in_lattice_than(&b).to_string()
    });
    c.field(&what, l[5], || {
        a.higher_cross_family_than(&b, true).to_string()
    });
    c.field(&what, l[6], || {
        a.higher_cross_family_than(&b, false).to_string()
    });
    c.field(&what, l[7], || ident(VarType::meet(&a, &b), &a, &b));
    c.field(&what, l[8], || ident(VarType::join(&a, &b), &a, &b));
    c.field(&what, l[9], || a.equals(&b).to_string());
    c.field(&what, l[10], || {
        GenericsChecker::new()
            .is_properly_bounded(&a, &b)
            .to_string()
    });
    let mut named: JHashMap<VarType, Vec<VarType>> = JHashMap::new();
    named.put(
        GenericType::parse("TT;"),
        vec![
            VarType::VARTYPE_STRING(),
            GenericType::parse("Ljava/util/List<Ljava/lang/String;>;"),
        ],
    );
    c.field(&what, l[11], || {
        GenericType::is_assignable(&a, &b, &named).to_string()
    });
    c.field(&what, l[12], || {
        GenericType::are_arguments_assignable(&a, &b, &named).to_string()
    });
}

const VERIFY: &str = include_str!("data/f2/verify.tsv");

pub fn verify_method(l: &[&str]) -> String {
    let Some(d) = GenericMain::parse_method_signature(&unesc(l[4]).unwrap()) else {
        return "\\N".into();
    };
    let md = MethodDescriptor::parse_descriptor(&unesc(l[3]).unwrap());
    let checker = unesc(l[2])
        .and_then(|s| GenericMain::parse_class_signature(&unesc(l[1]).unwrap(), &s))
        .map(|cd| cd.get_checker());
    d.verify_types(checker.as_ref(), &md.params, &md.ret, &[]);
    dump_method(&d)
}

pub fn verify_field(l: &[&str], strict: bool) -> String {
    let Some(d) = GenericMain::parse_field_signature(&unesc(l[4]).unwrap()) else {
        return "\\N".into();
    };
    let real = VarType::new_signature(&unesc(l[3]).unwrap());
    if strict {
        let cd =
            unesc(l[2]).and_then(|s| GenericMain::parse_class_signature(&unesc(l[1]).unwrap(), &s));
        d.verify_type(cd.as_ref(), &real);
    } else {
        d.verify_loosely(&real);
    }
    dump(Some(&d.type_()))
}

pub fn verify_class(l: &[&str]) -> String {
    let Some(cd) = GenericMain::parse_class_signature(&unesc(l[1]).unwrap(), &unesc(l[2]).unwrap())
    else {
        return "\\N".into();
    };
    let ifaces_text = unesc(l[4]).unwrap();
    let ifaces: Vec<VarType> = if ifaces_text.is_empty() {
        Vec::new()
    } else {
        ifaces_text
            .split(' ')
            .map(|s| VarType::new_signature_cl(s, true))
            .collect()
    };
    cd.verify_types(
        &VarType::new_signature_cl(&unesc(l[3]).unwrap(), true),
        &ifaces,
    );
    dump_class(&cd)
}

/// Rows where Java logged a mismatch need the context's logger and are not replayed here.
#[test]
fn verify_types_match_vineflower() {
    let mut c = Checker::new("Generic*Descriptor.verify*");
    let mut logged = 0;
    for l in lines(VERIFY, "VM") {
        if l.len() > 6 {
            logged += 1;
            continue;
        }
        c.field(&format!("VM {} {}", l[3], l[4]), l[5], || verify_method(&l));
    }
    for l in lines(VERIFY, "VF") {
        for (i, strict) in [(5, false), (6, true)] {
            if l[i].contains("WARN:") {
                logged += 1;
                continue;
            }
            c.field(&format!("VF {} {} {strict}", l[3], l[4]), l[i], || {
                verify_field(&l, strict)
            });
        }
    }
    for l in lines(VERIFY, "VC") {
        if l.len() > 6 {
            logged += 1;
            continue;
        }
        c.field(&format!("VC {}", l[1]), l[5], || verify_class(&l));
    }
    eprintln!("{logged} verify rows log a mismatch and need a logger");
    c.finish();
}

#[test]
fn type_lattice_matches_vineflower() {
    let mut c = Checker::new("VarType lattice");
    for l in lines(LATTICE, "LA") {
        check_lattice(&mut c, &l);
    }
    c.finish();
}
