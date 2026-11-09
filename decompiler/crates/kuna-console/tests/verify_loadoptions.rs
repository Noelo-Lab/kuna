//! A program loads with the load-time options its caller passes, not the
//! process environment's, so threads can load images at once with different
//! settings. These tests set environment variables, so they run in their own
//! binary.

mod common;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier, Mutex};

use kuna_console::engine::{
    bootstrap_from_object, bootstrap_from_object_with_options, ConsoleProgram,
};
use kuna_decomp::kuna_loadoptions::LoadOptions;
use kuna_decomp::kuna_sourcelang::SourceLang;
use object::write::{Object, StandardSection, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn specs() -> String {
    root().join("specs").to_str().unwrap().to_string()
}

fn fauxware() -> PathBuf {
    root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// An x86-64 relocatable object whose `.go.buildinfo` names `version`, which
/// is all Kuna needs to take it for a Go image of that release.
fn go_object(version: &str) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = obj.section_id(StandardSection::Text);
    let code = [0x48, 0x8d, 0x04, 0x1f, 0xc3]; // lea rax, [rdi+rbx]; ret
    let value = obj.append_section_data(text, &code, 16);
    obj.add_symbol(Symbol {
        name: b"main.add".to_vec(),
        value,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    let mut blob = b"\xff Go buildinf:".to_vec();
    blob.extend([8, 2]); // pointer size, inline strings
    blob.resize(32, 0);
    blob.push(u8::try_from(version.len()).unwrap());
    blob.extend(version.as_bytes());
    let info = obj.add_section(
        Vec::new(),
        b".go.buildinfo".to_vec(),
        SectionKind::ReadOnlyData,
    );
    obj.append_section_data(info, &blob, 16);
    obj.write().unwrap()
}

fn load(path: &Path, options: &LoadOptions) -> ConsoleProgram {
    let mut prog =
        bootstrap_from_object_with_options(path.to_str().unwrap(), "", &[specs()], None, options)
            .expect("bootstrap with built processor specs");
    prog.commit_pending_analysis()
        .expect("analysis commit succeeds");
    prog
}

/// The compiler field of the program's language id (`gcc`, `golang`, …).
fn compiler(prog: &ConsoleProgram) -> String {
    prog.arch().archid.rsplit(':').next().unwrap().to_string()
}

fn go_image() -> PathBuf {
    let path = common::scratch_file("loadoptions-go", "o");
    std::fs::write(&path, go_object("go1.25.3")).unwrap();
    path
}

/// What a load decided: its compiler spec, source language and Go release.
type Outcome = (String, SourceLang, Option<(u16, u16)>);

fn outcome(prog: &ConsoleProgram) -> Outcome {
    (
        compiler(prog),
        prog.arch().source_lang_detected,
        prog.arch().load_options().go_version,
    )
}

static ENVIRONMENT: Mutex<()> = Mutex::new(());

const ENV_VARS: [&str; 7] = [
    "KUNA_GOABI",
    "KUNA_SOURCELANG",
    "KUNA_KERNELCACHE",
    "KUNA_GOVERSION",
    "KUNA_LOAD_OBJC",
    "KUNA_LOAD_STRINGS",
    "KUNA_LOAD_MMAP",
];

fn snapshot() -> Vec<Option<std::ffi::OsString>> {
    ENV_VARS.iter().map(std::env::var_os).collect()
}

#[test]
fn concurrent_loads_keep_their_own_options() {
    let go = go_image();
    let fauxware = fauxware();
    let go_off = LoadOptions {
        goabi: false,
        ..LoadOptions::default()
    };
    let forced_c = LoadOptions {
        sourcelang: Some(SourceLang::C),
        ..LoadOptions::default()
    };
    let forced_go = LoadOptions {
        sourcelang: Some(SourceLang::Go),
        ..LoadOptions::default()
    };
    let stated_go = LoadOptions {
        go_version: Some((1, 22)),
        ..forced_go.clone()
    };
    let cases = [
        (
            go.clone(),
            LoadOptions::default(),
            ("golang".into(), SourceLang::Go, Some((1, 25))),
        ),
        (
            go.clone(),
            go_off,
            ("gcc".into(), SourceLang::Go, Some((1, 25))),
        ),
        (
            go.clone(),
            forced_c,
            ("gcc".into(), SourceLang::C, Some((1, 25))),
        ),
        (
            fauxware.clone(),
            LoadOptions::default(),
            ("gcc".into(), SourceLang::C, None),
        ),
        (
            fauxware.clone(),
            forced_go,
            ("golang".into(), SourceLang::Go, None),
        ),
        (
            fauxware,
            stated_go.clone(),
            ("golang".into(), SourceLang::Go, Some((1, 22))),
        ),
        (
            go.clone(),
            stated_go,
            ("golang".into(), SourceLang::Go, Some((1, 25))),
        ),
    ];
    for _ in 0..3 {
        let start = Arc::new(Barrier::new(cases.len()));
        let loads: Vec<_> = cases
            .iter()
            .cloned()
            .map(|(path, options, want)| {
                let start = Arc::clone(&start);
                std::thread::spawn(move || {
                    start.wait();
                    let prog = load(&path, &options);
                    assert_eq!(prog.arch().load_options().goabi, options.goabi);
                    assert_eq!(prog.arch().load_options().sourcelang, options.sourcelang);
                    (outcome(&prog), want)
                })
            })
            .collect();
        for load in loads {
            let (got, want) = load.join().unwrap();
            assert_eq!(got, want);
        }
    }
    std::fs::remove_file(go).unwrap();
}

/// What the `objc` and `strings` load passes left on two programs `bootstrap`
/// loads: the recovered `-[Greeter greet:]` and the `s_400915` string symbol.
fn load_pass_facts(bootstrap: impl Fn(&str) -> ConsoleProgram) -> (bool, bool) {
    let fixtures = root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let mut objc = bootstrap(fixtures.join("macho_objc_arm64").to_str().unwrap());
    objc.arch_mut().set_kuna_option("objc", "on").unwrap();
    objc.commit_pending_analysis().unwrap();
    let mut strings = bootstrap(fixtures.join("fauxware").to_str().unwrap());
    strings.commit_pending_analysis().unwrap();
    let arch = strings.arch();
    let global = arch.symboltab.get_global_scope().unwrap();
    (
        objc.has_symbol_named("-[Greeter greet:]"),
        !arch.symboltab.query_by_name(global, "s_400915").is_empty(),
    )
}

#[test]
fn explicit_options_neither_read_nor_write_the_environment() {
    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    let go = go_image();
    std::env::set_var("KUNA_GOABI", "off");
    std::env::set_var("KUNA_SOURCELANG", "rust");
    std::env::set_var("KUNA_KERNELCACHE", "off");
    std::env::set_var("KUNA_GOVERSION", "1.20");
    std::env::set_var("KUNA_LOAD_OBJC", "off");
    std::env::set_var("KUNA_LOAD_STRINGS", "off");
    std::env::set_var("KUNA_LOAD_MMAP", "off");
    let before = snapshot();
    let prog = load(&go, &LoadOptions::default());
    assert_eq!(
        outcome(&prog),
        ("golang".into(), SourceLang::Go, Some((1, 25)))
    );
    let explicit = load_pass_facts(|path| {
        bootstrap_from_object_with_options(path, "", &[specs()], None, &LoadOptions::default())
            .unwrap()
    });
    assert_eq!(explicit, (true, true), "an explicit load runs the passes the environment skips");
    assert_eq!(
        snapshot(),
        before,
        "an explicit load leaves the environment alone"
    );
    let from_env = load_pass_facts(|path| bootstrap_from_object(path, "", &[specs()]).unwrap());
    assert_eq!(from_env, (false, false), "an environment load skips them");
    for name in ENV_VARS {
        std::env::remove_var(name);
    }
    std::fs::remove_file(go).unwrap();
}

/// The language a load of the fat Mach-O fixture chose: its slice's processor.
fn fat_slice(options: &LoadOptions) -> String {
    let fat = root().join("decompiler/crates/kuna-analysis/tests/fixtures/macho_fat");
    let prog = load(&fat, options);
    let archid = prog.arch().archid.clone();
    archid.split(':').next().unwrap().to_string()
}

#[test]
fn an_explicit_load_picks_its_own_fat_slice() {
    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("KUNA_MACHO_SLICE", "arm64");
    let default = fat_slice(&LoadOptions::default());
    let x86 = fat_slice(&LoadOptions {
        macho_slice: Some("x86_64".into()),
        ..LoadOptions::default()
    });
    std::env::set_var("KUNA_MACHO_SLICE", "x86_64");
    let arm = fat_slice(&LoadOptions {
        macho_slice: Some("arm64".into()),
        ..LoadOptions::default()
    });
    std::env::remove_var("KUNA_MACHO_SLICE");
    assert_eq!(
        default, "x86",
        "no slice named: the default, not the environment's"
    );
    assert_eq!(x86, "x86");
    assert_eq!(arm, "AARCH64");
}

/// Decompile and print `name` through the console, as a session would.
fn console_decompile(prog: ConsoleProgram, name: &str) -> String {
    use kuna_console::ifacedecomp::{
        execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::ConsoleCommands;
    let cmds: Vec<String> = [
        format!("load function {name}"),
        "decompile".into(),
        "print C".into(),
    ]
    .into();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    dcp.conf = Some(prog);
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// Everything done with an explicitly loaded program -- the load, the commit,
/// listing, lookups, declarations, decompiles on both drivers, printing --
/// reads the program's own options, never a gate's environment fallback.
#[test]
fn an_explicit_program_never_reads_the_environment() {
    use kuna_console::project::{decompile_entry, DecompileOptions};
    use kuna_decomp::kuna_loadoptions::environment_fallbacks;
    let go = go_image();
    let fat = root().join("decompiler/crates/kuna-analysis/tests/fixtures/macho_fat");
    let before = environment_fallbacks();
    for path in [fauxware(), fat, go.clone()] {
        let mut prog = load(&path, &LoadOptions::default());
        let entries = prog.function_entries_executable();
        assert!(!entries.is_empty(), "{} lists functions", path.display());
        for entry in entries.iter().take(6) {
            assert!(prog.find_entry_at(entry.addr.get_offset()).is_some());
            prog.has_symbol_named(&entry.name);
            let result = decompile_entry(&mut prog, entry.clone(), &DecompileOptions::default());
            assert!(result.code.is_some_and(|c| !c.is_empty()), "{:?}", result.error);
        }
        let first = entries[0].clone();
        prog.ensure_function_symbol(first.addr.clone()).unwrap();
        prog.declare_function(first.addr.clone(), Some("outer::inner::renamed"), 0)
            .unwrap();
        assert!(prog.has_symbol_named("outer::inner::renamed"));
        assert!(prog.find_entry_by_name("outer::inner::renamed").is_some());
        let c = console_decompile(prog, "outer::inner::renamed");
        assert!(c.contains("renamed("), "the console decompiles it:\n{c}");
    }
    assert_eq!(
        environment_fallbacks(),
        before,
        "a gate read the environment on an explicit program's thread"
    );
    std::fs::remove_file(go).unwrap();
}

/// `set_kuna_option` changes the program's own load options, and the change
/// does not reach a later load with explicit options.
#[test]
fn set_kuna_option_changes_its_program_not_later_explicit_loads() {
    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    let go = go_image();
    std::env::remove_var("KUNA_GOABI");
    let mut prog = load(&go, &LoadOptions::default());
    prog.arch_mut().set_kuna_option("goabi", "off").unwrap();
    assert!(!prog.arch().load_options().goabi);
    assert_eq!(compiler(&load(&go, &LoadOptions::default())), "golang");
    for name in ENV_VARS {
        std::env::remove_var(name);
    }
    std::fs::remove_file(go).unwrap();
}

/// A program loaded with explicit options reports its own `kernelcache`
/// choice after the environment changes.
#[test]
fn an_explicit_program_ignores_a_later_environment_change() {
    use kuna_console::kuna_console::kuna_live_value;
    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("KUNA_KERNELCACHE");
    let explicit = load(&fauxware(), &LoadOptions::default());
    std::env::set_var("KUNA_KERNELCACHE", "off");
    let explicit_value = kuna_live_value(explicit.arch(), "kernelcache").unwrap().into_owned();
    std::env::remove_var("KUNA_KERNELCACHE");
    assert_eq!(explicit_value, "auto", "the explicit program kept its options");
}
