//! Project headers retain explicit declarations for imported call targets.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn explicit_fixed_and_variadic_import_declarations_recompile() {
    run_case(false);
}

#[test]
fn dwarf_call_declarations_survive_parallel_and_streaming_exports() {
    run_case(true);
}

fn run_case(debug_types: bool) {
    if debug_types && !cfg!(target_os = "linux") {
        return;
    }
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let provider = common::scratch_file("project-call-provider", "s");
    let caller = common::scratch_file("project-call-caller", "s");
    let library = provider.with_extension("dll");
    let imports = provider.with_extension("a");
    let binary = caller.with_extension(if debug_types { "so" } else { "dll" });
    std::fs::write(
        &provider,
        r#".text
.globl __adjust
.def __adjust; .scl 2; .type 32; .endef
__adjust:
 mov 4(%esp),%eax
 add 8(%esp),%eax
 ret
"#,
    )
    .unwrap();
    std::fs::write(
        &caller,
        r#".text
.globl _sample
.def _sample; .scl 2; .type 32; .endef
_sample:
 mov 4(%esp),%eax
 push $7
 push %eax
 call *__imp___adjust
 add $8,%esp
 ret
"#,
    )
    .unwrap();
    let callee = "_adjust";
    if !debug_types {
        for (source, output, provider_build) in
            [(&provider, &library, true), (&caller, &binary, false)]
        {
            let mut command = Command::new("clang");
            command
                .args([
                    "--target=i686-w64-windows-gnu",
                    "-shared",
                    "-nostdlib",
                    "-Wl,--entry,0",
                    "-Wl,--export-all-symbols",
                ])
                .arg(source);
            if provider_build {
                command.arg(format!("-Wl,--out-implib,{}", imports.display()));
            } else {
                command.arg(&imports);
            }
            let built = command.arg("-o").arg(output).output().unwrap();
            assert!(
                built.status.success(),
                "{}",
                String::from_utf8_lossy(&built.stderr)
            );
        }
    } else {
        let source = caller.with_extension("c");
        std::fs::write(&source, "__attribute__((noinline)) int _adjust(int value,int increment) { return value+increment; }\nint _sample(int value) { return _adjust(value,7); }\n").unwrap();
        let built = Command::new("clang")
            .args(["-g", "-O0", "-fPIC", "-shared", "-Wl,-Bsymbolic"])
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
    }
    let (inventory, err, status) =
        common::run_kuna(&["functions", binary.to_str().unwrap(), "--json"]);
    assert_eq!(status, 0, "{err}");
    let inventory: serde_json::Value = serde_json::from_str(&inventory).unwrap();
    let slot = inventory["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| {
            f["name"].as_str() == Some(callee) && (debug_types || f["size"].as_u64() == Some(0))
        })
        .unwrap()["address_hex"]
        .as_str()
        .unwrap();
    for variadic in [false, true] {
        if debug_types && variadic {
            continue;
        }
        let parameters = if variadic {
            "int value,..."
        } else {
            "int value,int increment"
        };
        let assertion = format!("prototype {slot} int __cdecl _adjust({parameters})");
        let modes = if debug_types { vec![0, 1, 2] } else { vec![0] };
        for mode in modes {
            let directory = caller.with_extension(format!("project-{variadic}-{mode}"));
            let mut args = vec![
                "decompile-project",
                binary.to_str().unwrap(),
                "--functions",
                "_sample",
                "--option",
                "libproto",
                "on",
                "-o",
                directory.to_str().unwrap(),
            ];
            if !debug_types {
                args.extend([
                    "--assert",
                    "prototype _sample int __cdecl _sample(int value)",
                    "--assert",
                    &assertion,
                ]);
            }
            if mode > 0 {
                args.extend(["--jobs", "2"]);
            }
            if mode == 2 {
                args.push("--stream");
            }
            let (out, err, status) = common::run_kuna(&args);
            assert_eq!(status, 0, "{out}\n{err}");
            let stem = binary.file_name().unwrap().to_str().unwrap();
            let header = std::fs::read_to_string(directory.join(format!("{stem}.h"))).unwrap();
            assert!(header.contains(&format!("{callee}(")), "{header}");
            if variadic {
                assert!(header.contains("..."), "{header}");
            }
            let driver = caller.with_extension("driver.c");
            let definition = if variadic {
                "int _adjust(int value,...) { va_list ap; va_start(ap,value); int increment=va_arg(ap,int); va_end(ap); return value+increment; }"
            } else {
                "int _adjust(int value,int increment) { return value+increment; }"
            };
            let check = "for(int i=-100;i<=100;i++) if(_sample(i)!=i+7) return 1;";
            std::fs::write(&driver,format!("#include \"{stem}.h\"\n#include <stdarg.h>\n{definition}\nint main(void) {{ {check} return 0; }}\n")).unwrap();
            for compiler in ["gcc", "clang"] {
                if cfg!(target_os = "windows") && compiler == "gcc" {
                    continue;
                }
                if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                    continue;
                }
                for optimization in ["-O0", "-O2"] {
                    let executable = caller.with_extension("native");
                    let built = Command::new(compiler)
                        .args([
                            optimization,
                            "-std=c11",
                            "-fno-builtin",
                            "-Werror=implicit-function-declaration",
                        ])
                        .arg("-I")
                        .arg(&directory)
                        .arg(directory.join(format!("{stem}.c")))
                        .arg(&driver)
                        .arg("-o")
                        .arg(&executable)
                        .output()
                        .unwrap();
                    assert!(
                        built.status.success(),
                        "{}",
                        String::from_utf8_lossy(&built.stderr)
                    );
                    let result = Command::new(&executable).output().unwrap();
                    assert!(
                        result.status.success(),
                        "{compiler} {optimization}: {result:?}"
                    );
                }
            }
        }
    }
}

#[test]
#[cfg(target_os = "linux")]
fn format_specializations_do_not_become_global_function_declarations() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("project-format-declarations", "c");
    let binary = source.with_extension("so");
    std::fs::write(&source, "#include <stdio.h>\nint sample(int value) { printf(\"%d\",value); printf(\"%d%d\",value,value+1); return value; }\n").unwrap();
    let built = Command::new("clang")
        .args(["-g", "-O0", "-shared", "-fPIC"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let directory = source.with_extension("project");
    let (out, err, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "--functions",
        "sample",
        "-o",
        directory.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{out}\n{err}");
    let stem = binary.file_name().unwrap().to_str().unwrap();
    let driver = source.with_extension("driver.c");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "{stem}.h"
#include <stdarg.h>
static int calls,total;
int capture_printf(char *format,...) {{
    va_list args; va_start(args,format); total+=va_arg(args,int);
    if(format[2]=='%') total+=va_arg(args,int);
    va_end(args); ++calls; return 0;
}}
int main(void) {{
    for(int value=-10;value<=10;value++) {{
        calls=total=0;
        if(sample(value)!=value || calls!=2 || total!=value*3+1) return 1;
    }}
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = source.with_extension("native");
            let built = Command::new(compiler)
                .args([optimization, "-std=c11", "-Dprintf=capture_printf"])
                .arg("-I")
                .arg(&directory)
                .arg(directory.join(format!("{stem}.c")))
                .arg(&driver)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{}",
                String::from_utf8_lossy(&built.stderr)
            );
            let result = Command::new(&executable).output().unwrap();
            assert!(
                result.status.success(),
                "{compiler} {optimization}: {result:?}"
            );
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn selected_callees_and_array_returns_keep_the_header_valid() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("project-call-wide", "c");
    let binary = source.with_extension("so");
    std::fs::write(&source, "typedef unsigned __int128 wide;\nstruct prime { unsigned p; wide inverse; };\n__attribute__((noinline)) static wide invert(wide a) { wide x = a; for (int i = 0; i < 7; i++) x = 2 * x - x * x * a; return x; }\nvoid record(struct prime *info, unsigned p) { info->p = p; info->inverse = invert(p); }\n").unwrap();
    let built = Command::new("clang")
        .args(["-g", "-O0", "-fPIC", "-shared"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    let stem = binary.file_name().unwrap().to_str().unwrap();
    for selection in ["record", "record,invert"] {
        let directory = source.with_extension(format!("project-{}", selection.len()));
        let (out, err, status) = common::run_kuna(&[
            "decompile-project",
            binary.to_str().unwrap(),
            "--functions",
            selection,
            "-o",
            directory.to_str().unwrap(),
        ]);
        assert_eq!(status, 0, "{out}\n{err}");
        let header_path = directory.join(format!("{stem}.h"));
        let header = std::fs::read_to_string(&header_path).unwrap();
        assert!(header.matches("invert(").count() <= 1, "{header}");
        let checked = Command::new("clang")
            .args(["-std=c11", "-fsyntax-only", "-x", "c"])
            .arg(&header_path)
            .output()
            .unwrap();
        assert!(checked.status.success(), "{header}\n{}", String::from_utf8_lossy(&checked.stderr));
    }
}
