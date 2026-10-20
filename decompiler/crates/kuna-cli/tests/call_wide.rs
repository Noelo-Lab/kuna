//! Caller-proved sealed MIPS returns must agree across CLI surfaces and execute
//! like the same-byte native fixtures, including carry and high-piece callers.
use crate::common;
use common::process;
use object::{Object, ObjectSection, ObjectSymbol};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy)]
struct Case {
    stem: &'static str,
    dwarf: bool,
    narrowext: &'static str,
}

const CASES: [Case; 2] = [
    Case {
        stem: "mips32_be",
        dwarf: false,
        narrowext: "abi",
    },
    Case {
        stem: "mips32_be",
        dwarf: true,
        narrowext: "compiler",
    },
];

const CALLERS: [&str; 6] = [
    "caller64",
    "caller_high32",
    "caller_high16",
    "caller_high8",
    "caller_bool",
    "caller_ignored",
];

const ASSERTIONS: [&str; 9] = [
    "prototype consume64 unsigned long long consume64(unsigned long long value)",
    "prototype consume_two32 unsigned int consume_two32(unsigned int high,unsigned int low)",
    "prototype consume_slot unsigned long long consume_slot(unsigned int tag,unsigned long long value)",
    "prototype caller64 unsigned long long caller64(unsigned long long left,unsigned long long right)",
    "prototype caller_high32 unsigned int caller_high32(unsigned long long left,unsigned long long right)",
    "prototype caller_high16 unsigned short caller_high16(unsigned long long left,unsigned long long right)",
    "prototype caller_high8 unsigned char caller_high8(unsigned long long left,unsigned long long right)",
    "prototype caller_bool bool caller_bool(unsigned long long left,unsigned long long right)",
    "prototype caller_ignored unsigned int caller_ignored(unsigned long long left,unsigned long long right)",
];

const SUPPORT: &str = r#"
#include <stdbool.h>
_Static_assert(sizeof(unsigned int) == 4 && sizeof(unsigned long long) == 8, "fixture word widths");
#define CONCAT44(h,l) (((unsigned long long)(unsigned int)(h) << 32) | (unsigned int)(l))
#define CONCAT22(h,l) ((((unsigned int)(h) & 0xffffU) << 16) | ((unsigned int)(l) & 0xffffU))
#define CONCAT31(h,l) ((((unsigned int)(h) & 0xffffffU) << 8) | ((unsigned int)(l) & 0xffU))
#define CARRY4(a,b) ((unsigned int)((unsigned int)(a) + (unsigned int)(b)) < (unsigned int)(a))
#define LZCOUNT(a) ((unsigned int)(a) == 0 ? 32 : __builtin_clz((unsigned int)(a)))
"#;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callwide")
}

impl Case {
    fn name(self) -> String {
        format!(
            "{}_{}",
            self.stem,
            if self.dwarf { "dwarf" } else { "declared" }
        )
    }

    fn image(self) -> PathBuf {
        let path = fixtures().join(format!("{}.elf", self.name()));
        assert!(
            path.is_file(),
            "missing verified callwide fixture {}",
            path.display()
        );
        path
    }

    fn command(self, verb: &str, callwide: Option<&str>, bejoin: &str) -> Command {
        self.command_for(verb, &self.image(), callwide, bejoin)
    }

    fn command_for(
        self,
        verb: &str,
        image: &Path,
        callwide: Option<&str>,
        bejoin: &str,
    ) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command
            .arg(verb)
            .arg(image)
            .args(["--sleighpath"])
            .arg(
                std::env::var_os("KUNA_CALLWIDE_SPECS")
                    .map_or_else(|| common::repo_root().join("specs"), PathBuf::from),
            )
            .args([
                "--option",
                "bejoin",
                bejoin,
                "--option",
                "dwarf",
                if self.dwarf { "on" } else { "off" },
                "--option",
                "narrowext",
                self.narrowext,
            ]);
        if let Some(value) = callwide {
            command.args(["--option", "callwide", value]);
        }
        if !self.dwarf {
            for assertion in ASSERTIONS {
                command.args(["--assert", assertion]);
            }
            command.arg("--assert-strict");
        }
        command
    }
}

fn output(command: &mut Command) -> String {
    let result = process::required_output(command);
    String::from_utf8(result.stdout).expect("UTF-8 CLI output")
}

fn records(command: &mut Command) -> BTreeMap<String, Value> {
    let text = output(command.arg("--json"));
    let document: Value = serde_json::from_str(&text).expect("valid decompile JSON");
    assert!(document["error"].is_null(), "run failed: {text}");
    let functions = document["functions"].as_array().expect("function records");
    let mut result = BTreeMap::new();
    for function in functions {
        assert!(function["error"].is_null(), "function failed: {function}");
        assert!(
            function["code"].is_string(),
            "missing function body: {function}"
        );
        let name = function["name"].as_str().expect("function name").to_owned();
        assert!(
            result.insert(name, function.clone()).is_none(),
            "duplicate function record"
        );
    }
    assert!(!result.is_empty(), "empty decompilation: {text}");
    result
}

fn code<'a>(records: &'a BTreeMap<String, Value>, name: &str) -> &'a str {
    records
        .get(name)
        .unwrap_or_else(|| panic!("missing {name}: {records:?}"))["code"]
        .as_str()
        .expect("function C body")
}

fn signature<'a>(text: &'a str, name: &str) -> &'a str {
    text.lines()
        .find(|line| {
            !line.starts_with([' ', '\t', '/', '#']) && line.contains(&format!(" {name}("))
        })
        .unwrap_or_else(|| panic!("missing {name} signature:\n{text}"))
}

fn require_bodies(records: &BTreeMap<String, Value>, case: Case) {
    for name in ["produce_sum", "caller64"] {
        let body = code(records, name);
        signature(body, name);
    }
    for name in CALLERS {
        let body = code(records, name);
        assert!(
            body.contains("return "),
            "{}: {name} lost its result:\n{body}",
            case.name()
        );
    }
}

fn export_project(case: Case, callwide: &str, extra: &[&str]) -> (PathBuf, PathBuf) {
    let dir = common::scratch_file(&format!("callwide-{}-project", case.name()), "dir");
    let mut command = case.command("decompile-project", Some(callwide), "evidence");
    command.arg("-o").arg(&dir).args(extra);
    output(&mut command);
    let stem = format!("{}.elf", case.name());
    let c = dir.join(format!("{stem}.c"));
    let h = dir.join(format!("{stem}.h"));
    assert!(
        c.is_file() && h.is_file(),
        "missing project C/header in {}",
        dir.display()
    );
    (c, h)
}

fn project(case: Case, callwide: &str, extra: &[&str]) -> (PathBuf, PathBuf) {
    let (c, h) = export_project(case, callwide, extra);
    check_contract(case, &h, if callwide == "off" { 4 } else { 8 });
    (c, h)
}

fn check_contract(case: Case, header: &Path, producer_width: usize) {
    let names: Vec<_> = std::iter::once("produce_sum").chain(CALLERS).collect();
    check_contracts(case, header, producer_width, &names);
}

fn check_contracts(case: Case, header: &Path, producer_width: usize, names: &[&str]) {
    let source = common::scratch_file("callwide-header-widths", "c");
    let mut checks = format!("#include \"{}\"\n", header.display());
    let declarations = std::fs::read_to_string(header).expect("actual emitted declarations");
    let typed_callers = producer_width == 8 || !case.dwarf;
    let callers = if typed_callers { &CALLERS[..] } else { &[] };
    for (name, count, width) in std::iter::once(("produce_sum", 4, 4))
        .chain(callers.iter().map(|name| (*name, 2, 8)))
        .filter(|(name, _, _)| names.contains(name))
    {
        let declaration = signature(&declarations, name);
        let arguments = declaration
            .split_once(&format!(" {name}("))
            .unwrap()
            .1
            .rsplit_once(')')
            .unwrap()
            .0;
        let arguments: Vec<_> = arguments.split(',').collect();
        assert_eq!(arguments.len(), count, "{name}: actual header arity");
        let words = if width == 4 {
            ["int", "unsigned int"]
        } else {
            ["long long", "unsigned long long"]
        };
        checks.push_str(&format!(
            "_Static_assert(sizeof({}) == {width} && sizeof({}) == {width}, \"{name} native integer words\");\n",
            words[0], words[1],
        ));
        let combinations: Vec<_> = (0..1 << count)
            .map(|mask| {
                let types: Vec<_> = (0..count).map(|index| words[(mask >> index) & 1]).collect();
                format!(
                    "__builtin_types_compatible_p(__typeof__(&{name}), \
                     __typeof__({name}({})) (*)({}))",
                    vec!["0"; count].join(","),
                    types.join(","),
                )
            })
            .collect();
        checks.push_str(&format!(
            "_Static_assert({}, \"{name} exact integer formal widths and arity\");\n",
            combinations.join(" || "),
        ));
    }
    for (expression, width) in [
        ("produce_sum(0,0,0,0)", producer_width),
        ("caller64(0ULL,0ULL)", 8),
        ("caller_high32(0ULL,0ULL)", 4),
        ("caller_high16(0ULL,0ULL)", 2),
        ("caller_high8(0ULL,0ULL)", 1),
        ("caller_bool(0ULL,0ULL)", 1),
        ("caller_ignored(0ULL,0ULL)", 4),
    ] {
        if !names.contains(&expression.split_once('(').unwrap().0)
            || (!typed_callers && !expression.starts_with("produce_sum("))
        {
            continue;
        }
        checks.push_str(&format!(
            "_Static_assert(sizeof({expression}) == {width}, \"{expression} width\");\n\
             _Static_assert(__builtin_classify_type(({expression}) + 0) == 1, \"{expression} integer\");\n"
        ));
    }
    std::fs::write(&source, checks).unwrap();
    let mut compilers = 0;
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        compilers += 1;
        let result = Command::new(compiler)
            .args(["-std=gnu11", "-fsyntax-only"])
            .arg(&source)
            .output()
            .expect("check actual header integer widths");
        assert!(
            result.status.success(),
            "{} {compiler}: actual header violates return widths:\n{}",
            case.name(),
            String::from_utf8_lossy(&result.stderr)
        );
    }
    assert!(
        compilers > 0,
        "actual header width checks require a C compiler"
    );
}

fn require_abi(records: &BTreeMap<String, Value>) {
    for (name, count, width) in
        std::iter::once(("produce_sum", 4, 4)).chain(CALLERS.into_iter().map(|name| (name, 2, 8)))
    {
        let Some(record) = records.get(name) else {
            continue;
        };
        let mut arguments: Vec<_> = record["variables"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|variable| variable["kind"] == "arg")
            .map(|variable| {
                (
                    variable["arg_index"].as_u64().unwrap(),
                    variable["size"].as_u64().unwrap(),
                )
            })
            .collect();
        arguments.sort_unstable();
        assert_eq!(
            arguments,
            (0..count).map(|index| (index, width)).collect::<Vec<_>>(),
            "{name}: ABI argument grouping changed"
        );
    }
    if records.contains_key("produce_sum") {
        assert!(
            code(records, "produce_sum").contains("CONCAT44("),
            "producer lost the ordered pair join"
        );
    }
}

fn check_selected_text(source: &Path, header: &Path) {
    let support = common::scratch_file("callwide-selected-support", "h");
    std::fs::write(&support, SUPPORT).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        process::required_output(
            Command::new(compiler)
                .args([
                    "-std=gnu11",
                    "-fsyntax-only",
                    "-Werror=implicit-function-declaration",
                ])
                .arg("-include")
                .arg(&support)
                .arg("-include")
                .arg(header)
                .arg(source),
        );
    }
}

fn replay(case: Case, sources: &[&Path], header: &Path) {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| {
            process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect();
    assert!(
        !compilers.is_empty(),
        "callwide semantic replay requires a C compiler"
    );
    let support = common::scratch_file("callwide-support", "h");
    std::fs::write(&support, SUPPORT).unwrap();
    let oracle = fixtures().join("oracle.c");
    if case.stem == "mips32_be" {
        replay_mips(case, sources, header, &support);
    }
    for source in sources {
        for compiler in &compilers {
            for level in ["-O0", "-O2"] {
                let executable = common::scratch_file(
                    &format!("callwide-{}-{compiler}-{level}", case.name()),
                    "exe",
                );
                let mut command = Command::new(compiler);
                command
                    .args([
                        "-std=gnu11",
                        "-w",
                        "-Werror=implicit-function-declaration",
                        "-fwrapv",
                        level,
                    ])
                    .arg("-include")
                    .arg(&support)
                    .arg("-include")
                    .arg(header)
                    .arg("-DCALLWIDE_HOST_REPLAY")
                    .arg(format!(
                        "-DCALLWIDE_DWARF_CALLER_CONTRACTS={}",
                        u8::from(case.dwarf)
                    ))
                    .arg(format!(
                        "-DCALLWIDE_EMITTED_HEADER=\"{}\"",
                        header.display()
                    ))
                    .arg(source)
                    .arg(&oracle)
                    .arg("-o")
                    .arg(&executable);
                process::required_output(&mut command);
                let run = Command::new(&executable)
                    .output()
                    .expect("run actual emitted C/header");
                assert!(run.status.success(),
                    "{} {} {compiler} {level}: actual C/header disagreed with native oracle, exit {:?} \
                     (1=full64, 2=high32, 3=high16, 4=high8, 5=bool, 6=ignored, 7=wide helper, 8=narrow helper, 9=producer):\n{}\n{}",
                    case.name(), source.display(), run.status.code(),
                    String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr));
            }
        }
    }
}

fn cross_tool(variable: &str, defaults: &[&str]) -> Option<String> {
    if let Some(value) = std::env::var_os(variable) {
        let value = value.into_string().expect("UTF-8 tool path");
        process::required_output(Command::new(&value).arg("--version"));
        return Some(value);
    }
    defaults.iter().find_map(|tool| {
        process::optional_output(Command::new(tool).arg("--version")).map(|_| (*tool).to_owned())
    })
}

fn manifest_command(manifest: &Value, index: usize) -> Vec<String> {
    manifest["commands"][index]
        .as_array()
        .expect("generator command")
        .iter()
        .map(|value| value.as_str().expect("command argument").to_owned())
        .collect()
}

fn run_arguments(arguments: &[String]) {
    process::required_output(Command::new(&arguments[0]).args(&arguments[1..]));
}

fn replay_compile_arguments(
    template: &[String],
    source: &Path,
    object: &Path,
    header: &Path,
    support: &Path,
    level: &str,
) -> Vec<String> {
    let mut arguments = template.to_vec();
    let source_at = arguments
        .iter()
        .position(|argument| argument == "-c")
        .unwrap()
        + 1;
    let output_at = arguments
        .iter()
        .position(|argument| argument == "-o")
        .unwrap()
        + 1;
    arguments[source_at] = source.to_str().unwrap().to_owned();
    arguments[output_at] = object.to_str().unwrap().to_owned();
    for argument in &mut arguments {
        if argument == "-O2" {
            *argument = level.to_owned();
        }
    }
    arguments.extend([
        "-Werror=implicit-function-declaration".to_owned(),
        "-include".to_owned(),
        support.to_str().unwrap().to_owned(),
        "-include".to_owned(),
        header.to_str().unwrap().to_owned(),
        format!("-DCALLWIDE_EMITTED_HEADER=\"{}\"", header.display()),
    ]);
    arguments
}

fn compile_replay(
    template: &[String],
    source: &Path,
    object: &Path,
    header: &Path,
    support: &Path,
    level: &str,
) {
    run_arguments(&replay_compile_arguments(
        template, source, object, header, support, level,
    ));
}

fn native_target_mips(
    case: Case,
    directory: &Path,
    manifest: &Value,
    selected: Option<&str>,
) -> PathBuf {
    let bytes = std::fs::read(case.image()).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let section = file.section_by_name(".text.callwide").unwrap();
    let target = directory.join("native-target.bin");
    std::fs::write(&target, section.data().unwrap()).unwrap();
    let mut assembly = String::from(
        ".set noreorder\n.set noat\n.section .text.callwide,\"ax\",@progbits\n.balign 4\n.Ltarget:\n",
    );
    for symbol in file.symbols().filter(|symbol| {
        symbol.kind() == object::SymbolKind::Text
            && symbol.section_index() == Some(section.index())
            && symbol.size() != 0
    }) {
        let name = symbol.name().unwrap();
        assert!(name.chars().enumerate().all(|(index, ch)| {
            ch == '_' || ch.is_ascii_alphabetic() || (index != 0 && ch.is_ascii_digit())
        }));
        let native =
            selected.map_or_else(|| name.to_owned(), |_| format!("callwide_native_{name}"));
        for alias in std::iter::once(native.as_str())
            .chain((selected.is_some() && selected != Some(name)).then_some(name))
        {
            assembly.push_str(&format!(
                ".globl {alias}\n.hidden {alias}\n.type {alias},@function\n.set {alias},.Ltarget+{}\n.size {alias},{}\n",
                symbol.address() - section.address(), symbol.size()
            ));
        }
    }
    assembly.push_str(&format!(".incbin \"{}\"\n", target.display()));
    let source = directory.join("native-target.S");
    let object = directory.join("native-target.o");
    std::fs::write(&source, assembly).unwrap();
    let mut compile = manifest_command(manifest, 0);
    let source_at = compile
        .iter()
        .position(|argument| argument == "-c")
        .unwrap()
        + 1;
    let output_at = compile
        .iter()
        .position(|argument| argument == "-o")
        .unwrap()
        + 1;
    compile[source_at] = source.to_str().unwrap().to_owned();
    compile[output_at] = object.to_str().unwrap().to_owned();
    run_arguments(&compile);
    object
}

fn replay_native_mips(case: Case, directory: &Path, manifest: &Value, link: &[String]) {
    let object = native_target_mips(case, directory, manifest, None);
    for level in ["-O0", "-O2"] {
        let oracle = directory.join(format!("native-oracle-{level}.o"));
        let executable = directory.join(format!("native-{level}.elf"));
        let mut compile = manifest_command(manifest, 3);
        let output_at = compile
            .iter()
            .position(|argument| argument == "-o")
            .unwrap()
            + 1;
        compile[output_at] = oracle.to_str().unwrap().to_owned();
        for argument in &mut compile {
            if argument == "-O2" {
                *argument = level.to_owned();
            }
        }
        run_arguments(&compile);
        let mut arguments = Vec::new();
        for argument in link {
            if argument == directory.join("consumers.o").to_str().unwrap() {
                continue;
            }
            let replacement = if argument == directory.join("fixture.o").to_str().unwrap() {
                &object
            } else if argument == directory.join("oracle.o").to_str().unwrap() {
                &oracle
            } else if argument == directory.join("runtime.elf").to_str().unwrap() {
                &executable
            } else {
                arguments.push(argument.clone());
                continue;
            };
            arguments.push(replacement.to_str().unwrap().to_owned());
        }
        run_arguments(&arguments);
        process::required_output(
            Command::new("python3")
                .arg(fixtures().join("generate.py"))
                .arg("--verify")
                .arg(case.image())
                .arg(&executable),
        );
        let qemu = cross_tool(
            "KUNA_CALLWIDE_MIPS_QEMU",
            &["qemu-mips", "qemu-mips-static"],
        )
        .unwrap();
        process::required_output(Command::new(qemu).arg(&executable));
    }
}

fn mips_replay_environment(case: Case) -> Option<(PathBuf, Value, String)> {
    let compiler = cross_tool("KUNA_CALLWIDE_MIPS_CC", &["clang", "clang-14"]);
    let linker = cross_tool("KUNA_CALLWIDE_MIPS_LD", &["ld.lld", "ld.lld-14"]);
    let qemu = cross_tool(
        "KUNA_CALLWIDE_MIPS_QEMU",
        &["qemu-mips", "qemu-mips-static"],
    );
    let (Some(compiler), Some(linker), Some(qemu)) = (compiler, linker, qemu) else {
        eprintln!("callwide cross-MIPS replay: optional compiler/linker/QEMU is absent");
        return None;
    };
    let directory = common::scratch_file(&format!("callwide-{}-cross", case.name()), "dir");
    let emitted = process::required_output(
        Command::new("python3")
            .arg(fixtures().join("generate.py"))
            .arg("--out")
            .arg(&directory)
            .args([
                "--arch",
                "mips32-be",
                "--case",
                "positive",
                "--mode",
                if case.dwarf {
                    "consumer-dwarf"
                } else {
                    "declared"
                },
                "--cc",
            ])
            .arg(&compiler)
            .arg("--ld")
            .arg(&linker),
    );
    let manifest: Value = serde_json::from_slice(&emitted.stdout).expect("generator manifest");
    let start = manifest_command(&manifest, 4);
    run_arguments(&start);
    Some((directory, manifest, qemu))
}

fn replay_mips(case: Case, sources: &[&Path], header: &Path, support: &Path) {
    let Some((directory, manifest, qemu)) = mips_replay_environment(case) else {
        return;
    };
    let compile = manifest_command(&manifest, 3);
    let link = manifest_command(&manifest, 5);
    replay_native_mips(case, &directory, &manifest, &link);
    for (index, source) in sources.iter().enumerate() {
        for level in ["-O0", "-O2"] {
            let object = directory.join(format!("generated-{index}-{level}.o"));
            let oracle = directory.join(format!("oracle-{index}-{level}.o"));
            let executable = directory.join(format!("generated-{index}-{level}.elf"));
            compile_replay(&compile, source, &object, header, support, level);
            compile_replay(
                &compile,
                &fixtures().join("oracle.c"),
                &oracle,
                header,
                support,
                level,
            );
            let mut arguments = Vec::new();
            for argument in &link {
                if argument == directory.join("consumers.o").to_str().unwrap() {
                    continue;
                }
                let replacement = if argument == directory.join("fixture.o").to_str().unwrap() {
                    &object
                } else if argument == directory.join("oracle.o").to_str().unwrap() {
                    &oracle
                } else if argument == directory.join("runtime.elf").to_str().unwrap() {
                    &executable
                } else {
                    arguments.push(argument.clone());
                    continue;
                };
                arguments.push(replacement.to_str().unwrap().to_owned());
            }
            run_arguments(&arguments);
            let run = Command::new(&qemu)
                .arg(&executable)
                .output()
                .expect("run emitted C on MIPS");
            assert!(run.status.success(),
                "{} {} {level}: cross-MIPS actual C/header disagreed with native oracle, exit {:?} \
                 (1=full64, 2=high32, 3=high16, 4=high8, 5=bool, 6=ignored, 7=wide helper, 8=narrow helper, 9=producer):\n{}\n{}",
                case.name(), source.display(), run.status.code(),
                String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr));
        }
    }
}

fn replay_selected_mips(case: Case, source: &Path, header: &Path, selected: &str) {
    let Some((directory, manifest, qemu)) = mips_replay_environment(case) else {
        return;
    };
    let native = native_target_mips(case, &directory, &manifest, Some(selected));
    let support = directory.join("selected-support.h");
    std::fs::write(&support, SUPPORT).unwrap();
    let original_linker = std::fs::read_to_string(directory.join("runtime.ld")).unwrap();
    assert_eq!(original_linker.matches("    *(.text)\n").count(), 1);
    assert_eq!(
        original_linker
            .matches("  .text.harness : { *(.text.harness*) } :runtime\n")
            .count(),
        1
    );
    let linker = directory.join("selected-runtime.ld");
    std::fs::write(
        &linker,
        original_linker.replace("    *(.text)\n", "").replace(
            "  .text.harness : { *(.text.harness*) } :runtime\n",
            "  .text.harness : { *(.text.harness*) *(.text .text.*) } :runtime\n",
        ),
    )
    .unwrap();
    let declarations = std::fs::read_to_string(header).unwrap();
    let native_extras: Vec<_> = std::iter::once("produce_sum")
        .chain(CALLERS)
        .chain(case.dwarf.then_some("contract_wide"))
        .chain(case.dwarf.then_some("contract_narrow"))
        .filter(|name| {
            !declarations.lines().any(|line| {
                !line.starts_with([' ', '\t', '/', '#']) && line.contains(&format!(" {name}("))
            })
        })
        .map(|name| format!("-DCALLWIDE_NATIVE_{}", name.to_ascii_uppercase()))
        .collect();
    let compile = manifest_command(&manifest, 3);
    for level in ["-O0", "-O2"] {
        let object = directory.join(format!("selected-{selected}-{level}.o"));
        let oracle = directory.join(format!("selected-oracle-{selected}-{level}.o"));
        let executable = directory.join(format!("selected-{selected}-{level}.elf"));
        compile_replay(&compile, source, &object, header, &support, level);
        let mut oracle_compile = replay_compile_arguments(
            &compile,
            &fixtures().join("oracle.c"),
            &oracle,
            header,
            &support,
            level,
        );
        oracle_compile.extend(native_extras.iter().cloned());
        run_arguments(&oracle_compile);
        let mut arguments = Vec::new();
        for argument in manifest_command(&manifest, 5) {
            if argument == directory.join("consumers.o").to_str().unwrap() {
                continue;
            }
            let replacement = if argument == directory.join("fixture.o").to_str().unwrap() {
                &native
            } else if argument == directory.join("oracle.o").to_str().unwrap() {
                &oracle
            } else if argument == directory.join("runtime.elf").to_str().unwrap() {
                &executable
            } else if argument == directory.join("runtime.ld").to_str().unwrap() {
                &linker
            } else {
                arguments.push(argument);
                continue;
            };
            arguments.push(replacement.to_str().unwrap().to_owned());
        }
        arguments.push(object.to_str().unwrap().to_owned());
        run_arguments(&arguments);
        let verified = process::required_output(
            Command::new("python3")
                .arg(fixtures().join("generate.py"))
                .arg("--verify-selected")
                .arg(case.image())
                .arg(&executable)
                .arg(selected),
        );
        std::fs::write(
            directory.join(format!("selected-{selected}-{level}-identity.json")),
            verified.stdout,
        )
        .unwrap();
        let run = Command::new(&qemu)
            .arg(&executable)
            .output()
            .expect("run selected actual C against unchanged native callees");
        assert!(run.status.success(),
            "{} {selected} {level}: selected actual C/header disagreed with native oracle, exit {:?} \
             (1=full64, 2=high32, 3=high16, 4=high8, 5=bool, 6=ignored, 7=wide helper, 8=narrow helper, 9=producer):\n{}\n{}",
            case.name(), run.status.code(),
            String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr));
    }
}

#[test]
fn typed_forwarding_promotes_the_producer_and_actual_c_and_header_execute() {
    for case in CASES {
        let off = records(&mut case.command("decompile-all", Some("off"), "evidence"));
        let on = records(&mut case.command("decompile-all", Some("on"), "evidence"));
        require_bodies(&on, case);
        if case.stem == "mips32_be" {
            project(case, "off", &[]);
            assert_ne!(
                code(&off, "produce_sum"),
                code(&on, "produce_sum"),
                "{}: callwide did not change the original MIPS producer",
                case.name()
            );
        }
        let whole = output(&mut case.command("decompile-all", Some("on"), "evidence"));
        for name in ["produce_sum"].into_iter().chain(CALLERS) {
            assert!(
                whole.contains(code(&on, name).trim()),
                "{}: whole text and JSON disagree about {name}:\n{whole}",
                case.name()
            );
        }
        let whole_path = common::scratch_file(&format!("callwide-{}-whole", case.name()), "c");
        std::fs::write(&whole_path, whole).unwrap();
        let (project_c, project_h) = project(case, "on", &[]);
        replay(case, &[&whole_path, &project_c], &project_h);
        if !case.dwarf {
            for realtypes in ["on", "off"] {
                let (c, h) = project(
                    case,
                    "on",
                    &[
                        "--option",
                        "ctypes",
                        "off",
                        "--option",
                        "realtypes",
                        realtypes,
                    ],
                );
                if realtypes == "off" {
                    for path in [&c, &h] {
                        assert!(
                            std::fs::read_to_string(path)
                                .unwrap()
                                .contains("xunknown8 produce_sum("),
                            "raw unknown return projection was not exercised: {}",
                            path.display()
                        );
                    }
                }
                replay(case, &[&c], &h);
            }
        }
    }
}

#[test]
fn selected_text_json_and_address_order_share_the_full_image_verdict() {
    for case in CASES {
        let whole = records(&mut case.command("decompile-all", Some("on"), "evidence"));
        require_bodies(&whole, case);
        require_abi(&whole);
        project(case, "on", &[]);
        let address_order = records(case.command("decompile-all", Some("on"), "evidence").args([
            "--option",
            "protoorder",
            "off",
        ]));
        require_abi(&address_order);
        assert_eq!(
            code(&whole, "produce_sum"),
            code(&address_order, "produce_sum")
        );
        let (ordered_c, ordered_h) = project(case, "on", &["--option", "protoorder", "off"]);
        replay(case, &[&ordered_c], &ordered_h);
        for name in [
            "produce_sum",
            "caller64",
            "caller_high8",
            "caller_bool",
            "caller_ignored",
        ] {
            let selected = records(
                case.command("decompile-all", Some("on"), "evidence")
                    .args(["--functions", name]),
            );
            assert_eq!(selected.len(), 1, "selection expanded beyond {name}");
            require_abi(&selected);
            let (selected_c, selected_h) = export_project(case, "on", &["--functions", name]);
            check_contracts(case, &selected_h, 8, &["produce_sum", name]);
            check_selected_text(&selected_c, &selected_h);
            replay_selected_mips(case, &selected_c, &selected_h, name);
        }
        let single = output(
            case.command("decompile", Some("on"), "evidence")
                .arg("caller64"),
        );
        let single_c = common::scratch_file("callwide-single-caller", "c");
        std::fs::write(&single_c, single).unwrap();
        let (_, selected_h) = export_project(case, "on", &["--functions", "caller64"]);
        check_contracts(case, &selected_h, 8, &["produce_sum", "caller64"]);
        check_selected_text(&single_c, &selected_h);
        replay_selected_mips(case, &single_c, &selected_h, "caller64");
    }
}

#[test]
fn dwarf_without_assertions_supplies_identical_parallel_and_streamed_contracts() {
    for case in CASES.into_iter().filter(|case| case.dwarf) {
        let serial = records(&mut case.command("decompile-all", Some("on"), "evidence"));
        require_bodies(&serial, case);
        require_abi(&serial);
        let parallel = records(case.command("decompile-all", Some("on"), "evidence").args([
            "--jobs",
            "2",
            "--jobs-chunk",
            "1",
        ]));
        require_abi(&parallel);
        assert_eq!(code(&serial, "produce_sum"), code(&parallel, "produce_sum"));
        let (parallel_c, parallel_h) = project(case, "on", &["--jobs", "2", "--jobs-chunk", "1"]);
        replay(case, &[&parallel_c], &parallel_h);
        for extra in [
            &["--stream"][..],
            &["--stream", "--jobs", "2", "--jobs-chunk", "1"][..],
        ] {
            let (c, h) = project(case, "on", extra);
            let header = std::fs::read_to_string(&h).unwrap();
            for name in ["produce_sum"].into_iter().chain(CALLERS) {
                assert_eq!(
                    signature(code(&serial, name), name),
                    signature(&header, name).trim_end_matches(';'),
                    "{}: streamed header changed {name}",
                    case.name()
                );
            }
            replay(case, &[&c], &h);
        }
    }
}

#[test]
fn default_off_and_legacy_bejoin_modes_keep_complete_records() {
    for case in CASES.into_iter().filter(|case| !case.dwarf) {
        let default = records(&mut case.command("decompile-all", None, "evidence"));
        let off = records(&mut case.command("decompile-all", Some("off"), "evidence"));
        assert_eq!(
            default,
            off,
            "{}: callwide changed the default",
            case.name()
        );
        for mode in ["off", "on"] {
            let baseline = records(&mut case.command("decompile-all", Some("off"), mode));
            let enabled = records(&mut case.command("decompile-all", Some("on"), mode));
            assert_eq!(
                baseline,
                enabled,
                "{}: callwide changed legacy bejoin {mode}",
                case.name()
            );
        }
    }
}

#[test]
fn returnable_mips_entries_and_arm_images_keep_complete_records_and_exports() {
    for stem in ["mips32_be_missing_root", "arm32_be"] {
        for dwarf in [false, true] {
            let case = Case {
                stem,
                dwarf,
                narrowext: "abi",
            };
            let off = records(&mut case.command("decompile-all", Some("off"), "evidence"));
            let on = records(&mut case.command("decompile-all", Some("on"), "evidence"));
            assert_eq!(off, on, "{}: unsupported image changed", case.name());
            let off_text = output(&mut case.command("decompile-all", Some("off"), "evidence"));
            let on_text = output(&mut case.command("decompile-all", Some("on"), "evidence"));
            assert_eq!(
                off_text,
                on_text,
                "{}: unsupported text changed",
                case.name()
            );
            let (off_c, off_h) = export_project(case, "off", &[]);
            let (on_c, on_h) = export_project(case, "on", &[]);
            for (off, on) in [(off_c, on_c), (off_h, on_h)] {
                assert_eq!(
                    std::fs::read(&off).unwrap(),
                    std::fs::read(&on).unwrap(),
                    "{}: unsupported export changed: {}",
                    case.name(),
                    on.display()
                );
            }
        }
    }
    for narrowext in ["abi", "off"] {
        let case = Case {
            stem: "mips32_be",
            dwarf: true,
            narrowext,
        };
        let off = records(&mut case.command("decompile-all", Some("off"), "evidence"));
        let on = records(&mut case.command("decompile-all", Some("on"), "evidence"));
        assert_eq!(
            off, on,
            "narrowext {narrowext}: incompatible DWARF output changed records"
        );
        let (off_c, off_h) = export_project(case, "off", &[]);
        let (on_c, on_h) = export_project(case, "on", &[]);
        for (off, on) in [(off_c, on_c), (off_h, on_h)] {
            assert_eq!(
                std::fs::read(off).unwrap(),
                std::fs::read(on).unwrap(),
                "narrowext {narrowext}: incompatible DWARF export changed"
            );
        }
    }
}

fn patched_mips(case: Case, name: &str, edits: &[(&str, u64, u32, u32)]) -> PathBuf {
    let mut bytes = std::fs::read(case.image()).expect("verified MIPS fixture");
    let offsets = {
        let file = object::File::parse(bytes.as_slice()).expect("MIPS fixture ELF");
        assert_eq!(file.architecture(), object::Architecture::Mips);
        assert!(!file.is_little_endian());
        edits
            .iter()
            .map(|(function, delta, expected, replacement)| {
                let symbol = file
                    .symbols()
                    .find(|symbol| symbol.name() == Ok(*function))
                    .unwrap_or_else(|| panic!("missing mutation target {function}"));
                assert_eq!(delta % 4, 0);
                assert!(
                    delta + 4 <= symbol.size(),
                    "{function}: mutation outside body"
                );
                let section = file
                    .section_by_index(symbol.section_index().unwrap())
                    .unwrap();
                let (start, size) = section.file_range().expect("file-backed target code");
                let within = symbol.address() - section.address() + delta;
                assert!(within + 4 <= size);
                let offset = usize::try_from(start + within).unwrap();
                let word = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap());
                assert_eq!(word, *expected, "{name}: {function}+{delta:#x} changed");
                (offset, *replacement)
            })
            .collect::<Vec<_>>()
    };
    for (offset, replacement) in offsets {
        bytes[offset..offset + 4].copy_from_slice(&replacement.to_be_bytes());
    }
    let path = common::scratch_file(&format!("callwide-{}-{name}", case.name()), "elf");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn altered_forwarding_and_return_paths_refuse_for_the_same_producer() {
    let mutations: &[(&str, &[(&str, u64, u32, u32)])] = &[
        (
            "swapped",
            &[
                ("caller64", 16, 0x0040_2021, 0x0060_2021),
                ("caller64", 24, 0x0060_2821, 0x0040_2821),
            ],
        ),
        ("partial", &[("caller64", 16, 0x0040_2021, 0x3044_ffff)]),
        (
            "stale-low",
            &[
                ("produce_sum", 4, 0x00e5_1821, 0x00e5_4821),
                ("produce_sum", 8, 0x0067_102b, 0x0127_102b),
            ],
        ),
        (
            "unrelated",
            &[
                ("caller64", 16, 0x0040_2021, 0x2404_0001),
                ("caller64", 24, 0x0060_2821, 0x2405_0002),
            ],
        ),
        ("two-formals", &[("caller64", 20, 0x0c04_0039, 0x0c04_003c)]),
        ("wrong-slot", &[("caller64", 20, 0x0c04_0039, 0x0c04_003e)]),
        (
            "low-only-caller",
            &[("caller_high8", 16, 0x3042_00ff, 0x3062_00ff)],
        ),
        (
            "wrong-ra-cell",
            &[("caller64", 28, 0x8fbf_0014, 0x8fbf_0010)],
        ),
        (
            "bad-sp-delay",
            &[("caller64", 36, 0x27bd_0018, 0x27bd_0014)],
        ),
    ];
    for case in CASES {
        let positive_off = records(&mut case.command("decompile-all", Some("off"), "evidence"));
        let positive_on = records(&mut case.command("decompile-all", Some("on"), "evidence"));
        assert_ne!(
            code(&positive_off, "produce_sum"),
            code(&positive_on, "produce_sum")
        );
        project(case, "off", &[]);
        project(case, "on", &[]);
        for (name, edits) in mutations {
            let image = patched_mips(case, name, edits);
            eprintln!(
                "{}: refusal control {name}; OFF must remain decodable",
                case.name()
            );
            let off =
                records(&mut case.command_for("decompile-all", &image, Some("off"), "evidence"));
            let on =
                records(&mut case.command_for("decompile-all", &image, Some("on"), "evidence"));
            assert_eq!(off, on, "{}: {name} admitted caller evidence", case.name());
        }
    }
}

#[test]
fn mixed_verdict_replays_no_contract_from_the_refused_producer() {
    let case = Case {
        stem: "mips32_be_mixed",
        dwarf: true,
        narrowext: "compiler",
    };
    let off = records(&mut case.command("decompile-all", Some("off"), "evidence"));
    let on = records(&mut case.command("decompile-all", Some("on"), "evidence"));
    assert_ne!(code(&off, "produce_sum"), code(&on, "produce_sum"));
    let (_, off_h) = project(case, "off", &[]);
    let (_, on_h) = project(case, "on", &[]);
    let off_h = std::fs::read_to_string(off_h).unwrap();
    let on_h = std::fs::read_to_string(on_h).unwrap();
    for name in ["produce_rejected", "caller_rejected", "contract_rejected"] {
        assert_eq!(off.get(name), on.get(name), "{name}: refused model changed");
        assert_eq!(
            signature(&off_h, name),
            signature(&on_h, name),
            "{name}: refused header changed"
        );
    }
}
