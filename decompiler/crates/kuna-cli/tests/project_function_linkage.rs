//! Project definitions and calls must use one C function identity.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(target_os = "linux")]
fn project_linkage_names_recompile_with_calls_and_function_addresses() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("project-function-linkage", "c");
    let binary = source.with_extension("so");
    std::fs::write(
        &source,
        r#"
__attribute__((noinline)) unsigned helper(unsigned value) __asm__("helper_linkage");
unsigned helper(unsigned value) { return value*3; }
unsigned sample(unsigned value) { return helper(value)+7; }
void *address(void) { return (void *)helper; }
void *address_shadow(unsigned helper_linkage) { return (void *)helper; }
unsigned named_local(unsigned helper_linkage) { return helper(helper_linkage)+11; }
"#,
    )
    .unwrap();
    let built = Command::new("gcc")
        .args(["-O0", "-g", "-shared", "-fPIC"])
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
    let inventory_args = ["functions", binary.to_str().unwrap(), "--json"];
    let (before_inventory, inventory_err, inventory_status) = common::run_kuna(&inventory_args);
    assert_eq!(inventory_status, 0, "{inventory_err}");
    let (standalone, err, status) =
        common::run_kuna(&["decompile", binary.to_str().unwrap(), "helper"]);
    assert_eq!(status, 0, "{err}");
    assert!(
        standalone.contains("helper(unsigned int value)"),
        "{standalone}"
    );
    for mode in 0..3 {
        let project = source.with_extension(format!("project-{mode}"));
        let mut args = vec![
            "decompile-project",
            binary.to_str().unwrap(),
            "--functions",
            "helper,sample,address,address_shadow,named_local",
            "--option",
            "protoorder",
            "off",
            "-o",
            project.to_str().unwrap(),
        ];
        if mode != 0 {
            args.extend(["--jobs", "2"]);
        }
        if mode == 2 {
            args.push("--stream");
        }
        let (out, err, status) = common::run_kuna(&args);
        assert_eq!(status, 0, "{out}\n{err}");
        let (after_inventory, inventory_err, inventory_status) = common::run_kuna(&inventory_args);
        assert_eq!(inventory_status, 0, "{inventory_err}");
        assert_eq!(before_inventory, after_inventory);
        let name = binary.file_name().unwrap().to_str().unwrap();
        let header_name = format!("{name}.h");
        if mode == 2 {
            let index = std::fs::read_to_string(project.join("index.jsonl")).unwrap();
            assert!(index.contains("\"name\":\"helper\""), "{index}");
            assert!(!index.contains("\"name\":\"helper_linkage\""), "{index}");
        }
        let header = std::fs::read_to_string(project.join(&header_name)).unwrap();
        assert!(
            header.contains("helper_linkage(unsigned int value)"),
            "{header}"
        );
        assert!(!header.contains(" helper(unsigned"), "{header}");
        let driver = source.with_extension("driver.c");
        let executable = source.with_extension("exe");
        std::fs::write(
            &driver,
            format!(
                r#"
#include "{header_name}"
#include <dlfcn.h>
int main(int argc, char **argv) {{
    if(argc!=2) return 2;
    void *module=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);
    if(!module) return 3;
    unsigned (*original)(unsigned)=(unsigned(*)(unsigned))dlsym(module,"sample");
    if(!original || address()!=(void *)helper_linkage) return 4;
    for(unsigned i=0;i<100;i++) {{
        if(sample(i)!=original(i) || named_local(i)!=i*3+11 || address_shadow(i)!=(void *)helper_linkage) return 1;
    }}
    dlclose(module); return 0;
}}
"#
            ),
        )
        .unwrap();
        for compiler in ["gcc", "clang"] {
            if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                continue;
            }
            for opt in ["-O0", "-O2"] {
                let built = Command::new(compiler)
                    .args(["-std=c11", "-Wall", "-Werror", opt])
                    .arg("-I")
                    .arg(&project)
                    .arg(project.join(format!("{name}.c")))
                    .arg(&driver)
                    .arg("-ldl")
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    built.status.success(),
                    "{}\n{header}",
                    String::from_utf8_lossy(&built.stderr)
                );
                assert!(
                    Command::new(&executable)
                        .arg(&binary)
                        .status()
                        .unwrap()
                        .success(),
                    "{compiler} {opt}"
                );
            }
        }
        std::fs::remove_file(driver).unwrap();
        std::fs::remove_file(executable).unwrap();
        std::fs::remove_dir_all(project).unwrap();
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(binary).unwrap();
}

#[test]
#[cfg(target_os = "linux")]
fn conflicting_emitted_function_names_are_reported_in_each_execution_mode() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let left = common::scratch_file("project-linkage-owner-left", "c");
    let right = left.with_extension("right.c");
    let binary = left.with_extension("so");
    for (path, name, constant) in [(&left, "hidden_left", 3), (&right, "hidden_right", 7)] {
        std::fs::write(path, format!(
            "static unsigned {name}(unsigned) __asm__(\"shared\");\nstatic __attribute__((used,noinline)) unsigned {name}(unsigned value){{return value+{constant};}}\n"
        )).unwrap();
    }
    let built = Command::new("gcc")
        .args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&left)
        .arg(&right)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    for mode in 0..3 {
        let project = left.with_extension(format!("conflict-{mode}"));
        let mut args = vec![
            "decompile-project",
            binary.to_str().unwrap(),
            "--functions",
            "hidden_left,hidden_right",
            "--option",
            "protoorder",
            "off",
            "-o",
            project.to_str().unwrap(),
        ];
        if mode != 0 {
            args.extend(["--jobs", "2"]);
        }
        if mode == 2 {
            args.push("--stream");
        }
        let (out, err, status) = common::run_kuna(&args);
        assert_eq!(status, 0, "{out}\n{err}");
        let header = std::fs::read_to_string(project.join(format!(
            "{}.h",
            binary.file_name().unwrap().to_str().unwrap()
        )))
        .unwrap();
        assert!(
            header.contains("/* C function name has multiple owning entries, including 0x")
                && !header.contains("#error"),
            "{out}\n{err}\n{header}"
        );
        std::fs::remove_dir_all(project).unwrap();
    }
    for path in [left, right, binary] {
        std::fs::remove_file(path).unwrap();
    }
}
