//! A program loads with the load-time options its caller passes, not the
//! process environment's, so threads can load images at once with different
//! settings. These tests set environment variables, so they run in their own
//! binary.

mod common;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier, Mutex};

use kuna_console::engine::{
    bootstrap_from_object, ArmIsa, Bootstrap, ConsoleProgram, ARM_ISA_ENV,
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
    let mut prog = Bootstrap::new(&[specs()])
        .options(options)
        .object(path.to_str().unwrap())
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
        Bootstrap::new(&[specs()]).options(&LoadOptions::default()).object(path).unwrap()
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

/// A `Bootstrap` without options reads the environment: its load options,
/// and an unset ISA from `KUNA_ARM_ISA`. With options, it reads neither, so a bad `KUNA_ARM_ISA`
/// cannot fail it.
#[test]
fn a_bootstrap_without_options_reads_the_environment() {
    use kuna_analysis::loadimage_te::synthetic::TeImage;
    use kuna_console::project::{decompile_entry, DecompileOptions};

    fn first_function(load: kuna_base::error::KunaResult<ConsoleProgram>) -> String {
        let mut prog = load.expect("bootstrap");
        prog.commit_pending_analysis().unwrap();
        let entry = prog.function_entries_executable()[0].clone();
        decompile_entry(&mut prog, entry, &DecompileOptions::default()).code.unwrap()
    }

    struct ClearOnDrop;
    impl Drop for ClearOnDrop {
        fn drop(&mut self) {
            std::env::remove_var(ARM_ISA_ENV);
            for name in ENV_VARS {
                std::env::remove_var(name);
            }
        }
    }

    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    let _clear = ClearOnDrop;
    let specs = [specs()];
    let roots = Bootstrap::new(&specs);
    let te = common::scratch_file("loadoptions-te", "te");
    std::fs::write(&te, TeImage::arm(&[0x07, 0x20, 0x70, 0x47, 0x1e, 0xff, 0x2f, 0xe1]).build()).unwrap();
    let te = te.to_str().unwrap();
    let options = LoadOptions::default();

    std::env::set_var(ARM_ISA_ENV, "thumb");
    let from_env = first_function(roots.image(te));
    assert!(from_env.contains("return 7"), "{from_env}");
    assert_eq!(first_function(roots.arm_isa(Some(ArmIsa::Thumb)).image(te)), from_env);
    let explicit = first_function(roots.options(&options).image(te));
    assert_eq!(explicit, first_function(roots.arm_isa(None).options(&options).image(te)));
    assert_ne!(explicit, from_env, "an explicit load ignored KUNA_ARM_ISA");

    std::env::set_var(ARM_ISA_ENV, "bogus");
    let refused = |load: kuna_base::error::KunaResult<ConsoleProgram>| match load {
        Ok(_) => panic!("loaded with KUNA_ARM_ISA=bogus"),
        Err(err) => err.to_string(),
    };
    let err = refused(roots.image(te));
    assert!(err.contains("invalid --isa"), "{err}");
    assert_eq!(first_function(roots.options(&options).image(te)), explicit);
    std::env::remove_var(ARM_ISA_ENV);
    std::fs::remove_file(te).unwrap();

    let go = go_image();
    std::env::set_var("KUNA_GOABI", "off");
    let mut prog = roots.object(go.to_str().unwrap()).unwrap();
    prog.commit_pending_analysis().unwrap();
    assert_eq!(compiler(&prog), "gcc");
    let mut prog = roots.options(&options).object(go.to_str().unwrap()).unwrap();
    prog.commit_pending_analysis().unwrap();
    assert_eq!(compiler(&prog), "golang");
    std::fs::remove_file(go).unwrap();
}

/// A load without options reads the environment even while another
/// program's options are in force on the thread: every input builds a program
/// that keeps reading the environment, never the held program's options.
#[test]
fn a_load_without_options_ignores_options_held_on_the_thread() {
    use kuna_analysis::loadimage_te::synthetic::TeImage;
    use kuna_base::xml::DocumentStorage;
    use kuna_console::kuna_console::kuna_live_value;
    use kuna_decomp::kuna_kernelcache::KernelcacheChoice;

    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("KUNA_KERNELCACHE");
    std::env::remove_var(ARM_ISA_ENV);
    let held = load(
        &fauxware(),
        &LoadOptions {
            kernelcache: KernelcacheChoice::Off,
            ..LoadOptions::default()
        },
    );
    let te = common::scratch_file("loadoptions-held-te", "te");
    let te_bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build();
    std::fs::write(&te, &te_bytes).unwrap();
    let raw = common::scratch_file("loadoptions-held-raw", "bin");
    std::fs::write(&raw, [0x48, 0x8d, 0x04, 0x1f, 0xc3]).unwrap();
    let xml = DocumentStorage::new()
        .parse_document(
            br#"<binaryimage arch="x86:LE:64:default"><bytechunk space="ram" offset="0x1000">488d041fc3</bytechunk></binaryimage>"#,
        )
        .unwrap()
        .get_root()
        .clone();
    let specs = [specs()];
    let roots = Bootstrap::new(&specs);
    let (te_path, raw_path) = (te.to_str().unwrap(), raw.to_str().unwrap());

    let _held = held.arch().load_scope();
    let loads = [
        ("te", roots.te(te_path)),
        ("te bytes", roots.te_bytes(te_path, &te_bytes)),
        ("image", roots.image(te_path)),
        ("file", roots.file(te_path)),
        ("raw", roots.target("x86:LE:64:default").raw(raw_path, 0x1000, &[0x1000])),
        ("xml root", roots.xml_root(&xml)),
    ];
    for (input, prog) in loads {
        let prog = prog.unwrap_or_else(|err| panic!("{input}: {err}"));
        assert_eq!(
            prog.arch().load_options().kernelcache,
            KernelcacheChoice::default(),
            "{input} took the held program's options"
        );
        std::env::set_var("KUNA_KERNELCACHE", "off");
        let live = kuna_live_value(prog.arch(), "kernelcache").unwrap().into_owned();
        std::env::remove_var("KUNA_KERNELCACHE");
        assert_eq!(live, "off", "{input} reads the environment live");
    }
    std::fs::remove_file(te).unwrap();
    std::fs::remove_file(raw).unwrap();
}

/// The console loads a second image only after `clear architecture`, which
/// drops the first program, so `load file` reads the environment.
#[test]
fn the_console_loads_the_next_image_from_the_environment() {
    use kuna_analysis::loadimage_te::synthetic::TeImage;
    use kuna_console::ifacedecomp::{
        execute, register_console_commands, register_decomp_commands, IfaceDecompData,
        DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::ConsoleCommands;
    use kuna_decomp::kuna_kernelcache::KernelcacheChoice;

    let _env = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("KUNA_KERNELCACHE");
    std::env::remove_var(ARM_ISA_ENV);
    let te = common::scratch_file("loadoptions-console-te", "te");
    std::fs::write(&te, TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build()).unwrap();
    let held = load(
        &fauxware(),
        &LoadOptions {
            kernelcache: KernelcacheChoice::Off,
            ..LoadOptions::default()
        },
    );
    let cmds: Vec<String> = vec![
        format!("load file {}", te.display()),
        "clear architecture".into(),
        format!("load file {}", te.display()),
    ];
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    register_console_commands(&mut status);
    let dcp = |status: &mut kuna_console::interface::IfaceStatus| {
        status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap()
            .conf
            .take()
    };
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let decomp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    decomp.spec_roots = vec![specs()];
    decomp.conf = Some(held);
    for _ in 0..count {
        execute(&mut status);
    }
    assert!(status.optr.contains("Load image already present"), "{}", status.optr);
    let next = dcp(&mut status).expect("the second load file built a program");
    assert_eq!(next.arch().load_options().kernelcache, KernelcacheChoice::default());
    std::fs::remove_file(te).unwrap();
}
