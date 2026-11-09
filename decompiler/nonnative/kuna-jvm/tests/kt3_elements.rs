//! KT3: the Kotlin plugin's type model, contracts, type mapping, import
//! collector and language chooser against what Vineflower's own plugin
//! classes print for the same input (`data/kt3/KtElementDump.java`;
//! regenerate with `data/kt3/gen.sh`). Every case is compared in full: an
//! `unported!` callee (`?pending`) fails the comparison. Opt-in sweep over full dumps:
//! `KT3_SWEEP_DIR=<dir> cargo test -p kuna-jvm --test integration kt3_sweep -- --ignored`.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::generic_type::GenericType;
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, Severity};
use kuna_jvm::import_collector::{ImportCollector, ImportCollectorData};
use kuna_jvm::invocation_exprent::InvocationExprent;
use kuna_jvm::k_contract::KContract;
use kuna_jvm::k_element::KElement;
use kuna_jvm::k_function::KFunction;
use kuna_jvm::k_parameter::KParameter;
use kuna_jvm::k_type::KType;
use kuna_jvm::k_type_parameter::KTypeParameter;
use kuna_jvm::k_types::KTypes;
use kuna_jvm::k_utils::KUtils;
use kuna_jvm::kotlin_chooser::KotlinChooser;
use kuna_jvm::kotlin_import_collector::KotlinImportCollector;
use kuna_jvm::kt_attrs::KtAttrs;
use kuna_jvm::kt_metadata::KtMetadata;
use kuna_jvm::kt_proto::{self, Visibility};
use kuna_jvm::metadata_name_resolver::MetadataNameResolver;
use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::{rethrow, try_catch, Caught, JHashMap, JHashSet};
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::var_type::VarType;

use crate::common;

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt3")
}

/// The driver's `esc`: UTF-16 units up to 0x20, from 0x7f, and `\` as `\uXXXX`.
fn esc(s: &str) -> String {
    let mut b = String::new();
    for u in s.encode_utf16() {
        if u <= 0x20 || u >= 0x7f || u == b'\\' as u16 {
            b.push_str(&format!("\\u{:04x}", u));
        } else {
            b.push(u as u8 as char);
        }
    }
    b
}

fn esc_opt(s: Option<&str>) -> String {
    match s {
        Some(s) => esc(s),
        None => "\\null".to_string(),
    }
}

fn unesc(s: &str) -> String {
    let units: Vec<u16> = s.encode_utf16().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < units.len() {
        if units[i] == b'\\' as u16 {
            let hex = from_utf16(&units[i + 2..i + 6]);
            out.push(u16::from_str_radix(&hex, 16).unwrap());
            i += 6;
        } else {
            out.push(units[i]);
            i += 1;
        }
    }
    from_utf16(&out)
}

/// A d1 line: `h <hex bytes>` or `s <escaped>`.
fn d1_string(line: &str) -> String {
    if let Some(hex) = line.strip_prefix("h ") {
        let units: Vec<u16> = (0..hex.len() / 2)
            .map(|i| u16::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
            .collect();
        return from_utf16(&units);
    }
    unesc(line.strip_prefix("s ").unwrap())
}

fn exc(c: &Caught) -> String {
    match &c.exception {
        Some(e) => format!("!{}", e.class),
        None => format!("!panic {}", c.message),
    }
}

/// The lines a walk prints; `pending` is set by the first `unported!` callee.
#[derive(Clone, Default)]
struct Out(Rc<RefCell<Vec<String>>>);

impl Out {
    fn p(&self, line: impl Into<String>) {
        self.0.borrow_mut().push(line.into());
    }

    /// The driver's `item`: run, and print the exception instead of failing;
    /// a still-`unported!` callee ends the whole walk.
    fn item(&self, f: impl FnOnce()) {
        if let Err(c) = try_catch(f) {
            if self.is_pending() {
                rethrow(c);
            }
            if c.is("UnportedError") {
                self.p(format!(
                    "?pending {}",
                    c.exception.as_ref().unwrap().message
                ));
                rethrow(c);
            }
            self.p(exc(&c));
        }
    }

    fn is_pending(&self) -> bool {
        self.0
            .borrow()
            .last()
            .is_some_and(|l| l.starts_with("?pending "))
    }

    fn lines(&self) -> Vec<String> {
        self.0.borrow().clone()
    }
}

#[derive(Default)]
struct Tally {
    cases: usize,
    full: usize,
    pending: usize,
    pending_names: std::collections::BTreeMap<String, usize>,
    mismatches: Vec<String>,
}

impl Tally {
    /// Compares a case's lines; `actual` may end early at a `?pending` line.
    fn compare(&mut self, id: &str, expected: &[String], actual: &[String]) {
        self.cases += 1;
        for (i, e) in expected.iter().enumerate() {
            let Some(a) = actual.get(i) else {
                self.mismatch(id, i, e, "<missing>");
                return;
            };
            if let Some(what) = a.strip_prefix("?pending ") {
                self.pending += 1;
                *self.pending_names.entry(what.to_string()).or_default() += 1;
                return;
            }
            if a != e {
                self.mismatch(id, i, e, a);
                return;
            }
        }
        if actual.len() > expected.len() {
            if let Some(what) = actual[expected.len()].strip_prefix("?pending ") {
                self.pending += 1;
                *self.pending_names.entry(what.to_string()).or_default() += 1;
                return;
            }
            self.mismatch(id, expected.len(), "<end>", &actual[expected.len()]);
            return;
        }
        self.full += 1;
    }

    fn mismatch(&mut self, id: &str, line: usize, expected: &str, actual: &str) {
        self.mismatches.push(format!(
            "{} line {}:\n  expected: {}\n  actual:   {}",
            id, line, expected, actual
        ));
    }

    fn check(&self, what: &str) {
        eprintln!(
            "{}: {} cases, {} fully equal, {} pending {:?}, {} mismatches",
            what,
            self.cases,
            self.full,
            self.pending,
            self.pending_names,
            self.mismatches.len()
        );
        assert_eq!(
            self.pending, 0,
            "{what}: no Kotlin body is unported any more: {:?}",
            self.pending_names
        );
        if !self.mismatches.is_empty() {
            let shown: Vec<&String> = self.mismatches.iter().take(20).collect();
            panic!(
                "{}: {} mismatches, first:\n{}",
                what,
                self.mismatches.len(),
                shown
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }
}

/// Runs `f`, which prints into `out`, and returns the lines (ending with a
/// `?pending` line when a callee is still `unported!`).
fn run_walk(f: impl FnOnce(&Out)) -> Vec<String> {
    let out = Out::default();
    let o = out.clone();
    if let Err(c) = try_catch(move || f(&o)) {
        if !out.is_pending() {
            if c.is("UnportedError") {
                out.p(format!(
                    "?pending {}",
                    c.exception.as_ref().unwrap().message
                ));
            } else {
                out.p(exc(&c));
            }
        }
    }
    out.lines()
}

// ---- the empty class the driver's ImportCollector stands for

fn fresh_collector(pkg: &str) -> KotlinImportCollector {
    let base = ImportCollector::alloc(ImportCollectorData {
        map_simple_names: JHashMap::new(),
        set_not_imported_names: JHashSet::new(),
        set_field_names: JHashSet::new(),
        map_inner_class_names: JHashMap::new(),
        current_package_slash: if pkg.is_empty() {
            String::new()
        } else {
            format!("{}/", pkg)
        },
        current_package_point: pkg.replace('/', "."),
        write_locked: false,
        kotlin: false,
    });
    let collector = KotlinImportCollector::new(base);
    DecompilerContext::set_import_collector(collector.0);
    collector
}

// ---- meta

struct MetaCase {
    id: String,
    pkg: String,
    k: i32,
    d1: Vec<String>,
    d2: Option<Vec<String>>,
    xs: Option<String>,
    expected: Vec<String>,
}

fn read_meta(path: &Path) -> Vec<MetaCase> {
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();
    let mut cases = Vec::new();
    while let Some(header) = lines.next() {
        let id = header
            .strip_prefix("=== ")
            .expect("case header")
            .to_string();
        let pkg = unesc(lines.next().unwrap().strip_prefix("pkg ").unwrap());
        let k: i32 = lines.next().unwrap()[2..].parse().unwrap();
        let n: usize = lines.next().unwrap()[3..].parse().unwrap();
        let d1 = (0..n).map(|_| d1_string(lines.next().unwrap())).collect();
        let d2_line = lines.next().unwrap();
        let d2 = if d2_line == "d2 -" {
            None
        } else {
            let n: usize = d2_line[3..].parse().unwrap();
            Some(
                (0..n)
                    .map(|_| unesc(lines.next().unwrap().strip_prefix("s ").unwrap()))
                    .collect(),
            )
        };
        let mut xs = None;
        let mut line = lines.next().unwrap();
        if let Some(x) = line.strip_prefix("xs ") {
            xs = Some(unesc(x));
            line = lines.next().unwrap();
        }
        assert_eq!(line, "--");
        let mut expected = Vec::new();
        for line in lines.by_ref() {
            if line == "end" {
                break;
            }
            expected.push(line.to_string());
        }
        cases.push(MetaCase {
            id,
            pkg,
            k,
            d1,
            d2,
            xs,
            expected,
        });
    }
    cases
}

enum Msg {
    Class(kt_proto::Class),
    Package(kt_proto::Package),
    Function(kt_proto::Function),
    None,
}

fn walk(c: &MetaCase, out: &Out) {
    let collector = fresh_collector(&c.pkg);
    let _ = &c.xs;
    let decoded = try_catch(|| {
        let mut input = KtMetadata::open(&c.d1);
        let resolver =
            c.d2.as_ref()
                .map(|d2| KtMetadata::read_resolver(&mut input, d2));
        let msg = match c.k {
            1 => Msg::Class(kt_proto::Class::parse_from(&mut input)),
            2 | 5 => Msg::Package(kt_proto::Package::parse_from(&mut input)),
            3 => Msg::Function(kt_proto::Function::parse_from(&mut input)),
            _ => Msg::None,
        };
        (resolver, msg)
    });
    let (resolver, msg) = match decoded {
        Ok(d) => d,
        Err(e) => {
            out.p(format!("decode {}", exc(&e)));
            return;
        }
    };
    let Some(r) = resolver else {
        out.p("noresolver");
        return;
    };
    let mut tail: Vec<Box<dyn FnOnce()>> = Vec::new();
    match msg {
        Msg::Class(pc) => walk_class(out, &pc, &r, &mut tail),
        Msg::Package(pp) => walk_package(out, &pp, &r, &mut tail),
        Msg::Function(f) => {
            out.p("lambda");
            walk_function(out, &f, &f.get_type_table(), &r, &mut tail);
        }
        Msg::None => out.p("nomsg"),
    }
    let mut ib = TextBuffer::new();
    collector.write_imports(&mut ib, true);
    out.p(format!(
        "imports {}",
        esc(&ib.convert_to_string_and_allow_data_discard())
    ));
    for t in tail {
        out.item(t);
    }
}

fn walk_class(
    out: &Out,
    pc: &kt_proto::Class,
    r: &MetadataNameResolver,
    tail: &mut Vec<Box<dyn FnOnce()>>,
) {
    out.p(format!("class {}", pc.get_flags()));
    out.item(|| out.p(format!("fq {}", esc(&r.resolve(pc.get_fq_name())))));
    if pc.has_companion_object_name() {
        out.item(|| {
            out.p(format!(
                "companion {}",
                esc(&r.resolve(pc.get_companion_object_name()))
            ))
        });
    }
    for t in pc.get_supertype_list() {
        out.item(|| {
            out.p("super");
            top(out, &KType::from(&t, r), " ");
        });
    }
    for f in pc.get_function_list() {
        walk_function(out, &f, &pc.get_type_table(), r, tail);
    }
    for pr in pc.get_property_list() {
        walk_property(out, &pr, r);
    }
    for ct in pc.get_constructor_list() {
        walk_constructor(out, &ct, r, tail);
    }
    walk_aliases(out, &pc.get_type_alias_list(), r);
    walk_type_table(out, &pc.get_type_table(), r);
}

fn walk_package(
    out: &Out,
    pp: &kt_proto::Package,
    r: &MetadataNameResolver,
    tail: &mut Vec<Box<dyn FnOnce()>>,
) {
    out.p("package");
    for f in pp.get_function_list() {
        walk_function(out, &f, &pp.get_type_table(), r, tail);
    }
    for pr in pp.get_property_list() {
        walk_property(out, &pr, r);
    }
    walk_aliases(out, &pp.get_type_alias_list(), r);
    walk_type_table(out, &pp.get_type_table(), r);
}

fn walk_aliases(out: &Out, aliases: &[kt_proto::TypeAlias], r: &MetadataNameResolver) {
    for ta in aliases {
        out.item(|| {
            out.p(format!("alias {}", esc(&r.resolve(ta.get_name()))));
            top(out, &KType::from(&ta.get_underlying_type(), r), " ");
        });
    }
}

fn walk_type_table(out: &Out, tt: &kt_proto::TypeTable, r: &MetadataNameResolver) {
    for i in 0..tt.get_type_count() {
        out.item(|| {
            out.p(format!("tt {}", i));
            top(out, &KType::from_table(i, r, tt), " ");
        });
    }
}

fn dump_params(out: &Out, params: &[KParameter]) {
    for kp in params {
        out.p(format!(
            " param {} {} {}",
            kp.flags,
            esc(&kp.name),
            kp.type_id
        ));
        top(out, &kp.type_, "  ");
        if let Some(vararg) = &kp.vararg_type {
            out.p("  vararg");
            top(out, vararg, "   ");
        }
    }
}

fn param_strings(
    out: &Out,
    name: String,
    params: Rc<[KParameter]>,
    tail: &mut Vec<Box<dyn FnOnce()>>,
) {
    let out = out.clone();
    tail.push(Box::new(move || {
        out.p(format!("pstr {}", esc(&name)));
        for kp in params.iter() {
            out.item(|| {
                let mut b = TextBuffer::new();
                kp.stringify(1, &mut b);
                out.p(format!(
                    " {}",
                    esc(&b.convert_to_string_and_allow_data_discard())
                ));
            });
        }
    }));
}

fn walk_function(
    out: &Out,
    f: &kt_proto::Function,
    tt: &kt_proto::TypeTable,
    r: &MetadataNameResolver,
    tail: &mut Vec<Box<dyn FnOnce()>>,
) {
    out.item(|| {
        let name = r.resolve(f.get_name());
        out.p(format!("fun {} {}", esc(&name), f.get_flags()));
        let mut params = Vec::new();
        for i in 0..f.get_value_parameter_count() {
            let vp = f.get_value_parameter(i);
            let flags = vp.get_flags();
            let pname = r.resolve(vp.get_name());
            let type_ = KType::from(&vp.get_type(), r);
            let vararg = if vp.has_vararg_element_type() {
                Some(KType::from(&vp.get_vararg_element_type(), r))
            } else {
                None
            };
            params.push(KParameter::new(
                flags,
                pname,
                type_,
                vararg,
                vp.get_type_id(),
            ));
        }
        let params: Rc<[KParameter]> = Rc::from(params);
        dump_params(out, &params);
        for tp in f.get_type_parameter_list() {
            type_parameter(out, &KTypeParameter::from(&tp, r), " ");
        }
        if f.has_receiver_type() {
            out.p(" receiver");
            top(out, &KType::from(&f.get_receiver_type(), r), "  ");
        }
        out.p(" returns");
        top(out, &KType::from(&f.get_return_type(), r), "  ");
        for ctx in f.get_context_receiver_type_list() {
            out.p(" context");
            top(out, &KType::from(&ctx, r), "  ");
        }
        param_strings(out, name.clone(), params.clone(), tail);
        if f.has_contract() {
            let out = out.clone();
            let f = f.clone();
            let tt = tt.clone();
            let r = r.clone();
            tail.push(Box::new(move || {
                out.p(format!("contract {}", esc(&name)));
                let c = KContract::from(&f.get_contract(), &params, &r, &tt);
                out.p(format!(
                    " {}",
                    esc(&c.stringify(1).convert_to_string_and_allow_data_discard())
                ));
            }));
        }
    });
}

fn walk_property(out: &Out, pr: &kt_proto::Property, r: &MetadataNameResolver) {
    out.item(|| {
        out.p(format!(
            "prop {} {}",
            esc(&r.resolve(pr.get_name())),
            pr.get_flags()
        ));
        let t = KType::from(&pr.get_return_type(), r);
        top(out, &t, " ");
        out.p(format!(
            " jsig {}",
            esc(&KTypes::get_java_signature(
                &t.k().kotlin_type,
                pr.get_return_type().get_nullable()
            ))
        ));
        if pr.has_setter_value_parameter() {
            out.p(" setter");
            top(
                out,
                &KType::from(&pr.get_setter_value_parameter().get_type(), r),
                "  ",
            );
        }
    });
}

fn walk_constructor(
    out: &Out,
    ct: &kt_proto::Constructor,
    r: &MetadataNameResolver,
    tail: &mut Vec<Box<dyn FnOnce()>>,
) {
    out.item(|| {
        out.p(format!("ctor {}", ct.get_flags()));
        let mut params = Vec::new();
        for i in 0..ct.get_value_parameter_count() {
            let vp = ct.get_value_parameter(i);
            params.push(KParameter::new(
                vp.get_flags(),
                r.resolve(vp.get_name()),
                KType::from(&vp.get_type(), r),
                Some(KType::from(&vp.get_vararg_element_type(), r)),
                vp.get_type_id(),
            ));
        }
        let params: Rc<[KParameter]> = Rc::from(params);
        dump_params(out, &params);
        param_strings(out, "<init>".to_string(), params, tail);
    });
}

fn type_parameter(out: &Out, tp: &KTypeParameter, ind: &str) {
    out.p(format!(
        "{}tparam {} {} {} {}",
        ind,
        tp.reified,
        tp.variance.name(),
        esc(&tp.name),
        tp.id
    ));
    for b in &tp.upper_bounds {
        top(out, b, &format!("{} ", ind));
    }
}

fn ident(t: &KType) -> &'static str {
    if t.ptr_eq(&KType::UNIT()) {
        " =UNIT"
    } else if t.ptr_eq(&KType::NOTHING()) {
        " =NOTHING"
    } else if t.ptr_eq(&KType::UBYTE()) {
        " =UBYTE"
    } else if t.ptr_eq(&KType::USHORT()) {
        " =USHORT"
    } else if t.ptr_eq(&KType::UINT()) {
        " =UINT"
    } else if t.ptr_eq(&KType::ULONG()) {
        " =ULONG"
    } else {
        ""
    }
}

fn type_(out: &Out, t: &KType, ind: &str) {
    let k = t.k();
    out.p(format!(
        "{}type {} {} {} {} {} {} {} {} {} {}{}",
        ind,
        esc(&k.kotlin_type),
        k.is_nullable,
        k.is_unsigned_type,
        esc_opt(k.type_parameter_name.as_deref()),
        esc_opt(k.type_alias_name.as_deref()),
        esc(&t.0.to_string()),
        t.type_.name(),
        t.array_dim,
        t.type_family.name(),
        t.stack_size,
        ident(t)
    ));
    if let Some(arguments) = &k.type_arguments {
        for a in arguments.iter() {
            out.p(format!("{} arg {}", ind, a.type_projection.name()));
            type_(out, &a.type_, &format!("{}  ", ind));
        }
    }
}

fn top(out: &Out, t: &KType, ind: &str) {
    type_(out, t, ind);
    out.p(format!(
        "{}jsig {}",
        ind,
        esc(&KTypes::get_java_signature(
            &t.k().kotlin_type,
            t.k().is_nullable
        ))
    ));
    out.item(|| {
        out.p(format!(
            "{}kt {}",
            ind,
            esc(&KTypes::get_kotlin_type(Some(&t.0)))
        ))
    });
    let str_ = RefCell::new(None::<String>);
    out.item(|| {
        let s = t.stringify(1).convert_to_string_and_allow_data_discard();
        out.p(format!("{}str {}", ind, esc(&s)));
        *str_.borrow_mut() = Some(s);
    });
    out.item(|| {
        let mut b = TextBuffer::new();
        b.append(&"#".repeat(150));
        b.append_buffer(&t.stringify(1));
        b.reformat();
        let fmt = b.convert_to_string_and_allow_data_discard()[150..].to_string();
        if str_.borrow().as_deref() != Some(fmt.as_str()) {
            out.p(format!("{}fmt {}", ind, esc(&fmt)));
        }
    });
}

fn check_meta(cases: &[MetaCase], tally: &mut Tally) {
    for c in cases {
        let actual = run_walk(|out| walk(c, out));
        tally.compare(&c.id, &c.expected, &actual);
    }
}

#[test]
fn kt3_elements_match_plugin() {
    let _guard = common::install_default_context();
    let cases = read_meta(&data_dir().join("meta.golden"));
    let mut tally = Tally::default();
    check_meta(&cases, &mut tally);
    tally.check("meta.golden");
    assert!(tally.cases >= 100);
}

#[test]
fn kt3_crafted_elements_match_plugin() {
    let _guard = common::install_default_context();
    let cases = read_meta(&data_dir().join("crafted.golden"));
    let mut tally = Tally::default();
    check_meta(&cases, &mut tally);
    tally.check("crafted.golden");
    assert!(tally.cases >= 16);
}

// ---- KTypes.getKotlinType

fn check_kotlin_types(path: &Path, tally: &mut Tally) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines().peekable();
    while let Some(header) = lines.next() {
        let rest = header.strip_prefix("=== ").expect("case header");
        let (_, sig) = rest.split_once(' ').unwrap();
        let sig = unesc(sig);
        let mut expected = Vec::new();
        for line in lines.by_ref() {
            if line == "end" {
                break;
            }
            expected.push(line.to_string());
        }
        let actual = run_walk(|out| {
            fresh_collector("");
            let t = match try_catch(|| GenericType::parse(&sig)) {
                Ok(t) => t,
                Err(e) => {
                    out.p(format!("parse {}", exc(&e)));
                    return;
                }
            };
            out.item(|| out.p(format!("kt {}", esc(&KTypes::get_kotlin_type(Some(&t))))));
            out.item(|| {
                out.p(format!(
                    "ko {}",
                    esc(&KTypes::get_kotlin_type_outer(Some(&t), false))
                ))
            });
        });
        tally.compare(rest, &expected, &actual);
    }
}

#[test]
fn kt3_kotlin_types_match_plugin() {
    let _guard = common::install_default_context();
    let mut tally = Tally::default();
    check_kotlin_types(&data_dir().join("kotlintypes.golden"), &mut tally);
    tally.check("kotlintypes.golden");
    assert_eq!(tally.pending, 0);
    assert!(tally.cases >= 500);
    assert_eq!(KTypes::get_kotlin_type(None), "*");
}

// ---- crafted getJavaSignature / appendVisibility

#[test]
fn kt3_java_signatures_and_visibility_match_plugin() {
    let text = std::fs::read_to_string(data_dir().join("misc.golden")).unwrap();
    let mut checked = 0;
    let mut guard = common::install_default_context();
    let mut show: Option<String> = None;
    for line in text.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "jsig" => {
                let input = unesc(parts[1]);
                let nullable = parts[2] == "true";
                let got = match try_catch(|| KTypes::get_java_signature(&input, nullable)) {
                    Ok(s) => esc(&s),
                    Err(e) => exc(&e),
                };
                assert_eq!(got, parts[3], "getJavaSignature({:?}, {})", input, nullable);
            }
            "vis" => {
                if show.as_deref() != Some(parts[1]) {
                    drop(guard);
                    guard = common::install_context_with(&[("kt-show-public", parts[1])]);
                    show = Some(parts[1].to_string());
                }
                let visibility = match parts[2] {
                    "null" => None,
                    name => Some(
                        *Visibility::VALUES
                            .iter()
                            .find(|v| v.name() == name)
                            .unwrap(),
                    ),
                };
                let mut b = TextBuffer::new();
                let got = match try_catch(|| KUtils::append_visibility(&mut b, visibility)) {
                    Ok(()) if visibility.is_none() => "ok".to_string(),
                    Ok(()) => esc(&b.convert_to_string_and_allow_data_discard()),
                    Err(e) => exc(&e),
                };
                assert_eq!(
                    got, parts[3],
                    "appendVisibility({}) show={}",
                    parts[2], parts[1]
                );
            }
            other => panic!("unknown line {}", other),
        }
        checked += 1;
    }
    drop(guard);
    assert!(checked >= 100);
}

// ---- the chooser

fn class_names(dir: &Path) -> Vec<String> {
    fn rec(base: &Path, dir: &Path, out: &mut Vec<String>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                rec(base, &p, out);
            } else if p.extension().is_some_and(|x| x == "class") {
                let rel = p
                    .strip_prefix(base)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");
                out.push(rel[..rel.len() - 6].to_string());
            }
        }
    }
    let mut out = Vec::new();
    rec(dir, dir, &mut out);
    out.sort_by(|a, b| {
        let a: Vec<u16> = a.encode_utf16().collect();
        let b: Vec<u16> = b.encode_utf16().collect();
        a.cmp(&b)
    });
    out
}

fn describe(e: Option<KElement>) -> String {
    let Some(e) = e else {
        return "-".to_string();
    };
    match e {
        KElement::Class(c) => format!("class {} {}", c.flags(), c.resolver.is_some()),
        KElement::File(f) => format!(
            "file {} {}",
            f.resolver.is_some(),
            esc_opt(f.multifile_name.as_deref())
        ),
        KElement::MultifileFacade(m) => format!(
            "facade {}",
            m.files.iter().map(|s| esc(s)).collect::<Vec<_>>().join(" ")
        ),
        KElement::FunctionReference(_) => "funref".to_string(),
        KElement::Function(f) if Rc::ptr_eq(&f, &KFunction::FAILED_LAMBDA()) => {
            "failedlambda".to_string()
        }
        KElement::Function(_) => "function".to_string(),
        KElement::HiddenElement(h) => format!("hidden {}", h.name()),
        KElement::Property(_) => "KProperty".to_string(),
        KElement::Constructor(_) => "KConstructor".to_string(),
    }
}

fn log_lines(logger: &common::ContextGuard, from: usize) -> Vec<String> {
    let records = logger.logger().records();
    let records = records.borrow();
    records[from..]
        .iter()
        .map(|r| {
            let mut line = format!("log {} {}", r.severity.name(), esc(&r.message));
            if let Some(t) = &r.throwable {
                let class = t.split(':').next().unwrap();
                line.push_str(&format!(" !{}", class));
            }
            line
        })
        .collect()
}

struct ChooserCase {
    id: String,
    expected: Vec<String>,
}

fn read_cases(path: &Path) -> Vec<ChooserCase> {
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();
    let mut cases = Vec::new();
    while let Some(header) = lines.next() {
        let id = header
            .strip_prefix("=== ")
            .expect("case header")
            .to_string();
        let mut expected = Vec::new();
        for line in lines.by_ref() {
            if line == "end" {
                break;
            }
            expected.push(line.to_string());
        }
        cases.push(ChooserCase { id, expected });
    }
    cases
}

fn check_chooser(label: &str, dir: &Path, cases: &[ChooserCase], tally: &mut Tally) {
    for (enable, export) in [("1", "0"), ("0", "0"), ("0", "1")] {
        let scenario = format!("e{}x{}", enable, export);
        let guard =
            common::install_context_with(&[("kt-enable", enable), ("kt-export-metadata", export)]);
        guard.logger().set_severity(Severity::Warn);
        KtAttrs::clear();
        let sc = guard.struct_context();
        sc.add_space_file(dir, true);
        for name in class_names(dir) {
            let id = format!("{}!{}!{}", label, scenario, name);
            let case = cases
                .iter()
                .find(|c| c.id == id)
                .unwrap_or_else(|| panic!("{} not in chooser.golden", id));
            let cl = sc.get_class(&name).unwrap();
            let out = Out::default();
            let mut stop = false;
            for _ in 0..2 {
                if stop {
                    break;
                }
                let from = guard.logger().records().borrow().len();
                let res = match try_catch(|| KotlinChooser::new().is_language(cl)) {
                    Ok(b) => b.to_string(),
                    Err(e) if e.is("UnportedError") => {
                        out.p(format!(
                            "?pending {}",
                            e.exception.as_ref().unwrap().message
                        ));
                        stop = true;
                        continue;
                    }
                    Err(e) => exc(&e),
                };
                out.p(format!("call {}", res));
                for l in log_lines(&guard, from) {
                    out.p(l);
                }
                out.p(format!(
                    "elem {}",
                    describe(KtAttrs::get(cl, &KElement::KEY()))
                ));
            }
            tally.compare(&id, &case.expected, &out.lines());
        }
        KtAttrs::clear();
    }
}

#[test]
fn kt3_chooser_matches_plugin() {
    let cases = read_cases(&data_dir().join("chooser.golden"));
    let mut tally = Tally::default();
    check_chooser("kt3", &data_dir().join("classes"), &cases, &mut tally);
    check_chooser(
        "kt1fx",
        &data_dir().join("../kt1/classes"),
        &cases,
        &mut tally,
    );
    tally.check("chooser.golden");
    assert_eq!(tally.cases, cases.len());
}

// ---- KUtils

fn invocation(desc: &str, params: Vec<Exprent>) -> InvocationExprent {
    let inv = InvocationExprent::new();
    inv.set_string_descriptor(desc.to_string());
    inv.set_descriptor(kuna_jvm::method_descriptor::MethodDescriptor::parse_descriptor(desc));
    inv.set_lst_parameters(params);
    inv
}

fn int(value: i32) -> Exprent {
    kuna_jvm::const_exprent::ConstExprent::new_int(value, false, None).0
}

#[test]
fn kt3_remove_arguments() {
    let _guard = common::install_default_context();
    let marker = VarType::new_signature("Lkotlin/jvm/internal/DefaultConstructorMarker;");
    let int_type = VarType::VARTYPE_INT();

    let none = invocation("()V", vec![]);
    KUtils::remove_arguments(none, &int_type);
    assert_eq!(none.get_string_descriptor(), "()V");

    let single = invocation("(I)V", vec![int(1)]);
    KUtils::remove_arguments(single, &int_type);
    assert!(single.get_lst_parameters().is_empty());
    assert_eq!(
        single.get_string_descriptor(),
        "(I)V",
        "one argument keeps the descriptor"
    );

    let a = int(1);
    let c = int(3);
    let several = invocation(
        "(ILkotlin/jvm/internal/DefaultConstructorMarker;I)V",
        vec![a, int(2), c],
    );
    KUtils::remove_arguments(several, &marker);
    assert_eq!(several.get_lst_parameters().to_vec(), vec![a, c]);
    assert_eq!(several.get_string_descriptor(), "(II)V");

    let to_return = invocation("(IJ)J", vec![int(1), int(2)]);
    KUtils::remove_arguments_replace(to_return, &VarType::VARTYPE_LONG(), Some(&int_type));
    assert_eq!(to_return.get_lst_parameters().len(), 1);
    assert_eq!(to_return.get_string_descriptor(), "(I)I");

    let invalid = invocation("(IJ)J", vec![int(1), int(2)]);
    let e = try_catch(|| KUtils::remove_arguments(invalid, &VarType::VARTYPE_LONG())).unwrap_err();
    assert!(e.is("IllegalStateException"), "{}", e.message);
    assert_eq!(e.exception.unwrap().message, "Invalid descriptor: (I)");
    assert_eq!(
        invalid.get_lst_parameters().len(),
        1,
        "arguments go before the check"
    );
}

#[test]
fn kt3_replace_exprent() {
    let _guard = common::install_default_context();
    let assignment = kuna_jvm::assignment_exprent::AssignmentExprent::new(int(1), int(2), None).0;
    assert_eq!(KUtils::replace_exprent(assignment), None);
    let constant = int(4);
    constant.set_kt(Some(kuna_jvm::k_dispatch::KExprentExt::Const));
    assert_eq!(
        KUtils::replace_exprent(constant),
        Some(constant),
        "a Kotlin object is kept"
    );
    assert_eq!(
        KUtils::replace_exprents(&[assignment, constant]),
        vec![assignment, constant]
    );
    let k = KUtils::replace_exprent(int(5)).unwrap();
    assert!(k.is_k_const() && k.is_const());
}

// ---- the opt-in sweep

#[test]
#[ignore]
fn kt3_sweep() {
    let Ok(dir) = std::env::var("KT3_SWEEP_DIR") else {
        return;
    };
    let _guard = common::install_default_context();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    let mut meta = Tally::default();
    let mut types = Tally::default();
    for f in files {
        let name = f.file_name().unwrap().to_str().unwrap().to_string();
        if name.starts_with("meta-") {
            check_meta(&read_meta(&f), &mut meta);
        } else if name.starts_with("kotlintypes-") {
            check_kotlin_types(&f, &mut types);
        }
    }
    types.check("sweep kotlintypes");
    meta.check("sweep meta");
}
