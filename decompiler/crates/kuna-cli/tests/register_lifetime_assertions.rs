//! Register-local assertions select a definition identity, not a register slot (#809).
mod common;

fn run(extra: &[&str], json: bool) -> String {
    let (code, rejected) = run_function("register_names", extra, json);
    assert!(rejected.is_empty(), "{extra:?}: {rejected:?}\n{code}");
    code
}

fn run_function(function: &str, extra: &[&str], json: bool) -> (String, Vec<String>) {
    run_fixture(function, "lifetimes_x86_64", extra, json)
}

fn run_fixture(
    function: &str,
    fixture_name: &str,
    extra: &[&str],
    json: bool,
) -> (String, Vec<String>) {
    run_fixture_options(function, fixture_name, extra, json, &[])
}

fn run_fixture_options(
    function: &str,
    fixture_name: &str,
    extra: &[&str],
    json: bool,
    options: &[(&str, &str)],
) -> (String, Vec<String>) {
    let root = common::repo_root();
    let fixture = common::fixture(fixture_name);
    let baseline = format!("@{}.kuna", fixture);
    let specs = std::env::var_os("SLEIGHHOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile",
            &fixture,
            function,
            "--mode",
            "reliable",
            "--assert-strict",
            "--sleighpath",
        ])
        .arg(specs);
    if std::path::Path::new(&format!("{fixture}.kuna")).exists() {
        command.args(["--assert", &baseline]);
    }
    for directive in extra {
        command.args(["--assert", directive]);
    }
    for (option, value) in options {
        command.args(["--option", option, value]);
    }
    if json {
        command.arg("--json");
    }
    let output = command.output().expect("run kuna");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !json {
        let rejected = stderr
            .lines()
            .filter(|s| s.contains("rejected"))
            .map(str::to_owned)
            .collect();
        assert_eq!(
            output.status.success(),
            !stderr.contains("rejected"),
            "{stderr}"
        );
        return (stdout, rejected);
    }
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let rejected: Vec<String> = doc["assertions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["status"] != "applied")
        .map(|a| a["detail"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        output.status.success(),
        rejected.is_empty(),
        "{stderr}\n{stdout}"
    );
    (
        doc["functions"][0]["code"].as_str().unwrap().to_owned(),
        rejected,
    )
}

fn both_surfaces(extra: &[&str]) -> String {
    let text = run(extra, false);
    let json = run(extra, true);
    assert_eq!(text.trim(), json.trim(), "text and JSON surfaces disagree");
    json
}

#[test]
fn single_rename_keeps_the_other_lifetime_separate() {
    let number = both_surfaces(&["name register_names::number recovered_number"]);
    assert!(number.contains("uint8 recovered_number;"), "{number}");
    assert!(number.contains("int4 *pointer_00;"), "{number}");
    assert!(
        number.contains("observe_number(recovered_number)"),
        "{number}"
    );
    assert!(number.contains("observe_pointer(pointer_00)"), "{number}");
    let pointer = both_surfaces(&["name register_names::pointer_00 recovered_pointer"]);
    assert!(pointer.contains("uint8 number;"), "{pointer}");
    assert!(pointer.contains("int4 *recovered_pointer;"), "{pointer}");
    assert!(pointer.contains("observe_number(number)"), "{pointer}");
    assert!(
        pointer.contains("observe_pointer(recovered_pointer)"),
        "{pointer}"
    );
}

#[test]
fn both_renames_apply_in_either_order() {
    let number = "name register_names::number recovered_number";
    let pointer = "name register_names::pointer_00 recovered_pointer";
    for order in [[number, pointer], [pointer, number]] {
        let code = both_surfaces(&order);
        assert!(code.contains("uint8 recovered_number;"), "{code}");
        assert!(code.contains("int4 *recovered_pointer;"), "{code}");
        assert_eq!(
            code.matches("observe_number(recovered_number)").count(),
            2,
            "{code}"
        );
        assert_eq!(
            code.matches("observe_pointer(recovered_pointer)").count(),
            2,
            "{code}"
        );
    }
}

#[test]
fn retype_does_not_seed_the_other_register_lifetime() {
    let code = both_surfaces(&[
        "type register_names::number long long signed_number",
        "name register_names::pointer_00 recovered_pointer",
    ]);
    assert!(code.contains("int8 signed_number;"), "{code}");
    assert!(code.contains("int4 *recovered_pointer;"), "{code}");
    assert_eq!(
        code.matches("observe_pointer(recovered_pointer)").count(),
        2,
        "{code}"
    );
}

#[test]
fn copies_in_other_registers_do_not_block_storage_reuse() {
    for order in [
        [
            "name register_overlap::number recovered_number",
            "name register_overlap::pointer_00 recovered_pointer",
        ],
        [
            "name register_overlap::pointer_00 recovered_pointer",
            "name register_overlap::number recovered_number",
        ],
    ] {
        for json in [false, true] {
            let (code, rejected) = run_function("register_overlap", &order, json);
            assert!(rejected.is_empty(), "{rejected:?}\n{code}");
            assert_eq!(
                code.matches("observe_number(recovered_number)").count(),
                2,
                "{code}"
            );
            assert_eq!(
                code.matches("observe_pointer(recovered_pointer)").count(),
                2,
                "{code}"
            );
        }
    }
}

#[test]
fn branch_and_loop_replay_keep_definition_families() {
    for function in ["register_branch", "register_loop"] {
        let baseline = run_function(function, &[], true).0;
        assert!(baseline.contains("uint8 number;"), "{baseline}");
        assert!(baseline.contains("int4 *pointer_00;"), "{baseline}");
        let directives = [
            format!("name {function}::number recovered_number"),
            format!("name {function}::pointer_00 recovered_pointer"),
        ];
        for order in [[0, 1], [1, 0]] {
            for json in [false, true] {
                let (code, rejected) = run_function(
                    function,
                    &[&directives[order[0]], &directives[order[1]]],
                    json,
                );
                assert!(rejected.is_empty(), "{rejected:?}\n{code}");
                assert!(code.contains("observe_number(recovered_number)"), "{code}");
                assert!(
                    code.contains("observe_pointer(recovered_pointer)"),
                    "{code}"
                );
            }
        }
    }
}

/// Exercise the interactive boundary too: the second assertion must not rely on
/// transient batch IDs, which are discarded when the IR is rebuilt.
fn console(script: &str, fixture: &str) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let root = common::repo_root();
    let engine = std::env::var_os("KUNA_DECOMP_DBG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_BIN_EXE_kuna"))
                .parent()
                .unwrap()
                .join("decomp_dbg")
        });
    let specs = std::env::var_os("SLEIGHHOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let mut child = Command::new(engine)
        .arg("-s")
        .arg(specs)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run console");
    let script = format!(
        "load file \"{}\"\nread symbols\n{script}\nquit\n",
        common::fixture(fixture)
    );
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn formal_parameter_name_survives_successive_console_rebuilds() {
    let transcript = console(
        "map prototype param_scalar_escaped int param_scalar_escaped(int a,int b,int c,int d,int e,int f,bool prepare_only);\n\
         map prototype read_word int read_word(const int *word);\n\
         map prototype read_saved_word int read_saved_word(void);\n\
         map address r0x402000 void *escaped_pointer\n\
         option stackviews on\noption structdefs on\n\
         load function param_scalar_escaped\ndecompile\n\
         rename prepare_only source_flag\ndecompile\ndecompile\nprint C",
        "lifetimes_parameter_variants_x86_64",
    );
    assert!(!transcript.contains("error:"), "{transcript}");
    let start = transcript.rfind("\nint4 param_scalar_escaped(").unwrap();
    let code = &transcript[start..];
    assert!(code.contains("bool source_flag)"), "{transcript}");
    assert!(code.contains(".view_bool = source_flag;"), "{transcript}");
    assert!(!code.contains("bool prepare_only)"), "{transcript}");
    assert!(!code.contains(".view_bool = prepare_only;"), "{transcript}");
}

#[test]
fn mixed_width_identities_survive_a_decompile_boundary() {
    for (first, second) in [
        ("rename s text", "rename v1 flag"),
        ("rename v1 flag", "rename s text"),
    ] {
        let transcript = console(
            &format!(
                "map prototype lookup char *lookup(void);\n\
             map prototype probe int probe(char *s);\n\
             map prototype pick_flag int pick_flag(void);\n\
             load function pick_flag\ndecompile\n{first}\ndecompile\n{second}\ndecompile\nprint C"
            ),
            "localnamebatch_x86_64",
        );
        assert!(
            !transcript.contains("error:") && !transcript.contains("overlaps"),
            "{transcript}"
        );
        assert!(transcript.contains("char *text; // rax"), "{transcript}");
        assert!(transcript.contains("uint4 flag; // eax"), "{transcript}");
        assert!(
            transcript.contains("flag = probe(text) != 0;"),
            "{transcript}"
        );
        assert!(!transcript.contains("_0_4_"), "{transcript}");
    }
}

#[test]
fn mixed_width_renamed_body_matches_native_values() {
    use std::process::Command;
    if common::process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    for order in [
        ["name s text", "name v1 flag"],
        ["name v1 flag", "name s text"],
        ["type s char *text", "type v1 unsigned int flag"],
    ] {
        let extra = [
            "prototype lookup char *lookup(void)",
            "prototype probe int probe(char *s)",
            "prototype pick_flag int pick_flag(void)",
            order[0],
            order[1],
        ];
        let (code, rejected) = run_fixture("pick_flag", "localnamebatch_x86_64", &extra, true);
        assert!(rejected.is_empty(), "{rejected:?}\n{code}");
        let source = common::scratch_file("mixed-register-lifetimes", "c");
        std::fs::write(&source, format!("#include <stdint.h>\ntypedef int32_t int4; typedef uint32_t uint4;\nstatic int state;\nchar *lookup(void) {{ return state == 0 ? 0 : (char *)\"x\"; }}\nint4 probe(char *s) {{ return state == 1 ? 7 : 0; }}\n{code}\nint main(void) {{ state=0; if(pick_flag()!=0) return 1; state=1; if(pick_flag()!=1) return 2; state=2; return pick_flag()!=0; }}\n")).unwrap();
        for level in ["-O0", "-O2"] {
            let binary = source.with_extension(&level[1..]);
            common::process::required_output(
                Command::new("clang")
                    .args(["-std=c11", level])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
    }
}

#[test]
fn equal_width_renames_survive_separate_decompiles() {
    for function in [
        "register_names",
        "register_overlap",
        "register_branch",
        "register_loop",
    ] {
        for (first, second) in [
            (
                "rename number recovered_number",
                "rename pointer_00 recovered_pointer",
            ),
            (
                "rename pointer_00 recovered_pointer",
                "rename number recovered_number",
            ),
        ] {
            let baseline =
                std::fs::read_to_string(format!("{}.kuna", common::fixture("lifetimes_x86_64")))
                    .unwrap();
            let declarations: Vec<_> = baseline
                .lines()
                .filter_map(|line| {
                    line.strip_prefix("prototype ")
                        .map(|s| format!("map prototype {s};"))
                        .or_else(|| {
                            line.strip_prefix("data ")
                                .map(|s| format!("map address {s}"))
                        })
                })
                .collect();
            let transcript = console(&format!(
            "{}\nload function {function}\ndecompile\n{first}\ndecompile\n{second}\ndecompile\nprint C",
            declarations.join("\n")
        ), "lifetimes_x86_64");
            assert!(!transcript.contains("Execution error:"), "{transcript}");
            assert!(
                transcript.contains("uint8 recovered_number;"),
                "{transcript}"
            );
            assert!(
                transcript.contains("int4 *recovered_pointer;"),
                "{transcript}"
            );
            assert!(
                transcript.contains("observe_number(recovered_number)"),
                "{transcript}"
            );
            assert!(
                transcript.contains("observe_pointer(recovered_pointer)"),
                "{transcript}"
            );
            if matches!(function, "register_names" | "register_overlap") {
                assert_eq!(
                    transcript
                        .matches("observe_number(recovered_number)")
                        .count(),
                    2,
                    "{transcript}"
                );
                assert_eq!(
                    transcript
                        .matches("observe_pointer(recovered_pointer)")
                        .count(),
                    2,
                    "{transcript}"
                );
            }
        }
    }
}

#[test]
fn windows_x64_register_replay_uses_the_same_definition_identities() {
    for order in [
        [
            "name register_names::number recovered_number",
            "name register_names::pointer_00 recovered_pointer",
        ],
        [
            "name register_names::pointer_00 recovered_pointer",
            "name register_names::number recovered_number",
        ],
    ] {
        for json in [false, true] {
            let (code, rejected) =
                run_fixture("register_names", "lifetimes_windows_x64.exe", &order, json);
            assert!(rejected.is_empty(), "{rejected:?}\n{code}");
            assert_eq!(
                code.matches("observe_number(recovered_number)").count(),
                2,
                "{code}"
            );
            assert_eq!(
                code.matches("observe_pointer(recovered_pointer)").count(),
                2,
                "{code}"
            );
        }
    }
}

#[test]
fn joined_register_assertions_replay_names_and_types() {
    let name = "name wide_names::value recovered_wide";
    let retype = "type wide_names::value LifetimeWideAlias";
    for order in [[name, retype], [retype, name]] {
        let text = run_fixture("wide_names", "lifetimes_join_x86_64", &order, false);
        let json = run_fixture("wide_names", "lifetimes_join_x86_64", &order, true);
        assert!(text.1.is_empty() && json.1.is_empty(), "{text:?}\n{json:?}");
        assert_eq!(text.0.trim(), json.0.trim());
        assert!(
            json.0.contains("LifetimeWideAlias recovered_wide;"),
            "{}",
            json.0
        );
        assert_eq!(
            json.0.matches("observe_wide(recovered_wide)").count(),
            2,
            "{}",
            json.0
        );
    }
}

#[test]
fn joined_register_identity_survives_separate_decompiles() {
    let transcript = console(
        "parse line typedef struct LifetimeWide { unsigned long long x; unsigned long long y; } LifetimeWide;\n\
         parse line typedef struct LifetimeWide LifetimeWideAlias;\n\
         map prototype wide_names void wide_names(void);\n\
         map prototype wide_source struct LifetimeWide wide_source(void);\n\
         map prototype observe_wide void observe_wide(struct LifetimeWide value);\n\
         load function wide_names\ndecompile\nrename value recovered_wide\ndecompile\n\
         retype recovered_wide LifetimeWideAlias\ndecompile\nprint C",
        "lifetimes_join_x86_64",
    );
    assert!(!transcript.contains("error:"), "{transcript}");
    assert!(
        transcript.contains("LifetimeWideAlias recovered_wide;"),
        "{transcript}"
    );
    assert_eq!(
        transcript.matches("observe_wide(recovered_wide)").count(),
        2,
        "{transcript}"
    );
}

#[test]
fn joined_register_asserted_body_matches_native_aggregate() {
    use std::process::Command;
    common::process::required_output(&mut Command::new(common::fixture("lifetimes_join_x86_64")));
    let (code, rejected) = run_fixture(
        "wide_names",
        "lifetimes_join_x86_64",
        &[
            "name wide_names::value recovered_wide",
            "type wide_names::value LifetimeWideAlias",
        ],
        true,
    );
    assert!(rejected.is_empty(), "{rejected:?}\n{code}");
    let source = common::scratch_file("joined-register-lifetimes", "c");
    std::fs::write(
        &source,
        format!(
            "#include <stdint.h>\n\
        typedef uint64_t uint8;\n\
        typedef struct LifetimeWide {{ uint8 x,y; }} LifetimeWide;\n\
        typedef LifetimeWide LifetimeWideAlias;\n\
        static uint8 total;\n\
        LifetimeWide wide_source(void) {{ return (LifetimeWide){{13,55}}; }}\n\
        void observe_wide(LifetimeWide value) {{ total+=value.x+value.y; }}\n\
        {code}\n\
        int main(void) {{ wide_names(); return total!=136; }}\n"
        ),
    )
    .unwrap();
    for compiler in ["clang", "gcc"] {
        if common::process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let binary = source.with_extension(format!("{compiler}-{}", &level[1..]));
            common::process::required_output(
                Command::new(compiler)
                    .args(["-std=c11", level, "-fstrict-aliasing"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
    }
}

#[test]
fn same_instruction_register_and_temporary_names_survive_replay() {
    let baseline = "lifetimes_temporaries_x86_64";
    for order in [
        [
            "name v1 operand",
            "name v2 multiplier",
            "name input_00 low",
            "name input_01 high",
        ],
        [
            "name input_01 high",
            "name input_00 low",
            "name v2 multiplier",
            "name v1 operand",
        ],
    ] {
        for json in [false, true] {
            let (code, rejected) = run_fixture_options(
                "multiply_names",
                baseline,
                &order,
                json,
                &[("impliedrefs", "0")],
            );
            assert!(rejected.is_empty(), "{rejected:?}\n{code}");
            for name in ["operand", "multiplier"] {
                assert!(code.contains(&format!("char {name} [16];")), "{code}");
            }
            assert!(code.contains("operand = ZEXT816(input);"), "{code}");
            assert!(code.contains("multiplier = ZEXT816(7);"), "{code}");
            assert_eq!(code.matches("observe_number(low)").count(), 2, "{code}");
            assert_eq!(code.matches("observe_number(high)").count(), 2, "{code}");
            assert!(!code.contains("._0_"), "{code}");
        }
    }
}

#[test]
fn named_scalar_temporaries_execute_the_native_multiply() {
    use std::process::Command;
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_temporaries_x86_64",
    )));
    for order in [
        ["name v1 low", "name input_00 high"],
        ["name input_00 high", "name v1 low"],
    ] {
        let (code, rejected) = run_fixture_options(
            "multiply_names32",
            "lifetimes_temporaries_x86_64",
            &order,
            true,
            &[("impliedrefs", "0")],
        );
        assert!(rejected.is_empty(), "{rejected:?}\n{code}");
        assert_eq!(code.matches("observe_number(low)").count(), 2, "{code}");
        assert_eq!(code.matches("observe_number(high)").count(), 2, "{code}");
        let source = common::scratch_file("scalar-temporary-multiply", "c");
        std::fs::write(
            &source,
            format!(
                "#include <stdint.h>\ntypedef uint64_t uint8; typedef uint32_t uint4;\n\
             static uint8 total;\nvoid observe_number(uint8 value) {{ total += value; }}\n\
             {code}\nint main(void) {{\n\
               for (uint8 i = 0; i < 3; i++) {{\n\
                 uint4 input = i == 0 ? 0 : i == 1 ? 13 : UINT32_MAX;\n\
                 uint8 product = (uint8)input * 7;\n\
                 total = 0; multiply_names32(input);\n\
                 if (total != 2 * ((uint4)product + (product >> 32))) return 1;\n\
               }} return 0; }}\n"
            ),
        )
        .unwrap();
        for compiler in ["gcc", "clang"] {
            if common::process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                continue;
            }
            for level in ["-O0", "-O2"] {
                let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
                common::process::required_output(
                    Command::new(compiler)
                        .args(["-std=c11", level])
                        .arg(&source)
                        .arg("-o")
                        .arg(&binary),
                );
                common::process::required_output(&mut Command::new(binary));
            }
        }
    }
}
