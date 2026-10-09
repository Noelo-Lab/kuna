//! A later same-width register local cannot replace the incoming parameter.
mod common;

use std::process::Command;

#[test]
fn independent_byte_hash_and_pointer_values_do_not_share_a_register_local() {
    let fixture = common::fixture("lifetimes_register_domains_x86_64");
    common::process::required_output(&mut Command::new(&fixture));
    let inferred = common::scratch_file("inferred-register-domains", "kuna");
    let declarations = std::fs::read_to_string(format!("{fixture}.kuna")).unwrap();
    std::fs::write(
        &inferred,
        declarations
            .lines()
            .filter(|line| !line.starts_with("prototype "))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    for (declared, knowledge) in [
        (true, format!("@{fixture}.kuna")),
        (false, format!("@{}", inferred.display())),
    ] {
        for stack_views in ["off", "on"] {
            let output = common::process::required_output(
                Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args([
                        "decompile",
                        &fixture,
                        "value_domains",
                        "--json",
                        "--mode",
                        "reliable",
                        "--assert-strict",
                        "--option",
                        "stackviews",
                        stack_views,
                        "--option",
                        "structdefs",
                        "on",
                        "--assert",
                        &knowledge,
                        "--sleighpath",
                    ])
                    .arg(common::repo_root().join("specs")),
            );
            let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert!(document["functions"][0]["error"].is_null(), "{document}");
            let code = document["functions"][0]["code"].as_str().unwrap();
            let declaration = if code.contains("struct Node {") {
                ""
            } else {
                "typedef struct Node { struct Node *next, *previous; uint4 identity; uint1 padding[4]; } Node;"
            };
            let selected_declaration = if declared { "Node *" } else { "unsigned long" };
            let selected_check = if declared {
                "selected!=(choice ? &last : &first)"
            } else {
                "selected!=(uintptr_t)(choice ? &last : &first)"
            };
            let source = common::scratch_file("register-value-domains", "c");
            std::fs::write(
                &source,
                format!(
                    "#include <stdint.h>\n\
             typedef uint8_t uint1; typedef uint32_t uint4;\n\
             typedef uint64_t uint8; typedef int64_t int8; typedef int32_t int4;\n\
             {declaration}\nuint8 observed_hash;\n{code}\n\
             int main(void) {{\n\
               Node first={{0}}, last={{0}}; {selected_declaration} selected=0;\n\
               first.identity=0x12345678;\n\
               for (uint4 choice=0; choice<2; ++choice) {{\n\
                 if (value_domains(&first,&last,&selected,choice)!=&last ||\n\
                     {selected_check} ||\n\
                     observed_hash!=((86ULL*0x100000001b3ULL)^52ULL^18ULL))\n\
                   return 1;\n\
               }}\n\
               return 0;\n\
             }}\n"
                ),
            )
            .unwrap();
            for compiler in ["clang", "gcc"] {
                if common::process::optional_output(Command::new(compiler).arg("--version"))
                    .is_none()
                {
                    continue;
                }
                for level in ["-O0", "-O2"] {
                    let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
                    common::process::required_output(
                        Command::new(compiler)
                            .args([
                                "-std=c11",
                                level,
                                "-fstrict-aliasing",
                                "-Werror=int-conversion",
                                "-Werror=incompatible-pointer-types",
                            ])
                            .arg(&source)
                            .arg("-o")
                            .arg(&binary),
                    );
                    common::process::required_output(&mut Command::new(binary));
                }
            }
        }
    }
}

#[test]
fn renaming_the_dispatch_key_keeps_the_incoming_property() {
    let fixture = common::fixture("lifetimes_x86_64");
    common::process::required_output(&mut Command::new(&fixture));
    for stack_views in ["off", "on"] {
        for assertions in [
            vec!["name register_parameter_reuse::property_00 cleared_property"],
            vec!["type register_parameter_reuse::property_00 unsigned int cleared_property"],
            vec![
                "name register_parameter_reuse::property incoming_property",
                "name register_parameter_reuse::property_00 cleared_property",
            ],
        ] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.args([
                "decompile",
                &fixture,
                "register_parameter_reuse",
                "--json",
                "--mode",
                "reliable",
                "--assert-strict",
                "--option",
                "stackviews",
                stack_views,
                "--assert",
                &format!("@{fixture}.kuna"),
            ]);
            for assertion in &assertions {
                command.args(["--assert", assertion]);
            }
            let output = common::process::required_output(&mut command);
            let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let code = document["functions"][0]["code"].as_str().unwrap();
            let input = if assertions.len() == 2 {
                "incoming_property"
            } else {
                "property"
            };
            assert!(code.contains(&format!("if ({input} != 200)")), "{code}");
            assert!(code.contains(&format!("return {input};")), "{code}");
            assert!(
                code.lines().any(|line| {
                    line.contains("consume_key(") && line.contains("cleared_property")
                }),
                "{code}"
            );
            let source = common::scratch_file("parameter-register-reuse", "c");
            std::fs::write(
                &source,
                format!(
                    "#include <stdint.h>\ntypedef int32_t int4; typedef uint32_t uint4;\n\
             int4 consume_key(int4 key) {{ return key*2; }}\n{code}\n\
             int main(void) {{ return register_parameter_reuse(200)!=200 || \
             register_parameter_reuse(142)!=116 || register_parameter_reuse(31)!=31 || \
             register_parameter_reuse(-3)!=-3; }}\n"
                ),
            )
            .unwrap();
            for compiler in ["clang", "gcc"] {
                if common::process::optional_output(Command::new(compiler).arg("--version"))
                    .is_none()
                {
                    continue;
                }
                for level in ["-O0", "-O2"] {
                    let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
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
    }
}
