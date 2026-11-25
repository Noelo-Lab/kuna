//! #794 end to end on 32-bit ARM: `ins16` returns a `u64` whose high word the
//! machine zeroes. Narrowed to `int` (`--option returnpair single`, the pre-fix
//! behavior) it agrees under the physical ABI and differs against the source's
//! contract; with the fix it agrees with both.

use std::path::PathBuf;

use kuna_verify::{check_binary, Options, Report, Verdict};
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope};

const SOURCE: &str = "typedef unsigned long long u64;
u64 ins16(unsigned a, unsigned short b) { return ((a + 1) << 16) | b; }
u64 zadd(int a, int b) { return (unsigned)(a + b); }
u64 zsel(int a, unsigned b) { return a > 0 ? b : 7u; }
u64 twice(unsigned a, unsigned short b) { return ins16(a, b) * 2; }
";

/// `ins16` is `orr r0,r1,r0,lsl #16; mov r1,#0; add r0,r0,#65536; bx lr`;
/// `twice` calls it.
const FUNCS: &[(&str, &[u32])] = &[
    ("ins16", &[0xe1810800, 0xe3a01000, 0xe2800801, 0xe12fff1e]),
    ("zadd", &[0xe0810000, 0xe3a01000, 0xe12fff1e]),
    ("zsel", &[0xe3500000, 0xd3a01007, 0xe1a00001, 0xe3a01000, 0xe12fff1e]),
    ("twice", &[0xe92d4800, 0xebfffff9, 0xe1a01081, 0xe1811fa0, 0xe1a00080, 0xe8bd4800, 0xe12fff1e]),
];

fn arm_object() -> PathBuf {
    let mut text = Vec::new();
    let mut at = Vec::new();
    for (_, code) in FUNCS {
        at.push(text.len() as u64);
        text.extend(code.iter().flat_map(|w| w.to_le_bytes()));
    }
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    obj.flags = FileFlags::Elf { os_abi: 0, abi_version: 0, e_flags: 0x0500_0000 };
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, &text, 16);
    for ((name, code), &value) in FUNCS.iter().zip(&at) {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size: (code.len() * 4) as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let dir = std::env::temp_dir().join(format!("kuna-verify-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("zext_pair.o");
    write_whole(&path, &obj.write().unwrap());
    path
}

/// Write `bytes` to `path` through a rename, so a test reading the fixture
/// never sees another test thread's half-written copy of it.
pub fn write_whole(path: &std::path::Path, bytes: &[u8]) {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let tmp = path.with_extension(format!("{n}.tmp"));
    std::fs::write(&tmp, bytes).unwrap();
    std::fs::rename(&tmp, path).unwrap();
}

fn specs() -> Vec<String> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    vec![p.canonicalize().unwrap().to_string_lossy().into_owned()]
}

fn check(functions: &[&str], options: &[(&str, &str)], contract: bool) -> Vec<Report> {
    let path = arm_object();
    let functions: Vec<String> = functions.iter().map(|s| s.to_string()).collect();
    let options: Vec<(String, String)> = options.iter().map(|(n, v)| (n.to_string(), v.to_string())).collect();
    let opts = Options { runs: 64, ..Options::default() };
    let source = contract.then_some((SOURCE, "zext_pair.c"));
    check_binary(&path.to_string_lossy(), &functions, &options, &specs(), source, &opts).unwrap()
}

#[test]
fn narrowed_return_differs_only_against_the_contract() {
    let physical = check(&["ins16"], &[("returnpair", "single")], false);
    assert_eq!(physical[0].verdict, Verdict::Agrees(64), "{}", physical[0].render());
    assert!(
        physical[0].suspects.iter().any(|s| s.contains("r1") && s.contains("every run")),
        "{}",
        physical[0].render()
    );

    let typed = check(&["ins16"], &[("returnpair", "single")], true);
    let r = &typed[0];
    assert!(matches!(r.verdict, Verdict::Differs { .. }), "{}", r.render());
    let text = r.render();
    assert!(text.contains("contract u64 ins16(unsigned a, unsigned short b)"), "{text}");
    assert!(text.contains("r1:r0"), "{text}");
}

#[test]
fn the_fix_agrees_with_the_contract() {
    let reports = check(&["ins16", "zadd", "zsel"], &[], true);
    for r in &reports {
        assert_eq!(r.verdict, Verdict::Agrees(64), "{}", r.render());
    }
}

/// kuna prints `twice(void)` calling `sub_400020()`: it drops the two
/// arguments `twice` passes straight through to `ins16`. The machine runs the
/// callee for real, so the callee's reads show the drop.
#[test]
fn a_dropped_pass_through_argument_differs() {
    let reports = check(&["twice"], &[], true);
    let r = &reports[0];
    assert!(matches!(r.verdict, Verdict::Differs { .. }), "{}", r.render());
    assert!(r.render().contains("kuna's call passes nothing there"), "{}", r.render());
}

/// `localize` checks under the same contract as `check`: without it the
/// narrowed `ins16` does not differ, so there is nothing to localize.
#[test]
fn localize_uses_the_contract() {
    use kuna_verify::localize::{localize, Finding, Source};
    let path = arm_object();
    let path = path.to_string_lossy();
    let options = vec![("returnpair".to_string(), "single".to_string())];
    let opts = Options { runs: 64, ..Options::default() };
    let src = |contract| Source::Binary { path: &path, functions: vec!["ins16".to_string()], options: &options, contract };
    let without = localize(&src(None), &specs(), "ins16", None, &opts).unwrap();
    assert!(matches!(without.finding, Finding::NotDiffering(_)), "{}", without.render());
    let with = localize(&src(Some((SOURCE, "zext_pair.c"))), &specs(), "ins16", None, &opts).unwrap();
    assert!(!matches!(with.finding, Finding::NotDiffering(_)), "{}", with.render());
}
