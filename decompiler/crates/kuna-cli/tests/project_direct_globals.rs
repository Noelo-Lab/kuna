//! Direct global accesses need declarations even when nobody takes their address.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(target_os = "linux")]
fn exported_scalar_global_compiles_and_matches_the_original_shared_library() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("project-direct-globals", "c");
    let binary = source.with_extension("so");
    let project = source.with_extension("project");
    std::fs::write(&source,"static unsigned counter = 37;\nunsigned sample(unsigned amount) { counter += amount; return counter; }\n").unwrap();
    let built = Command::new("gcc")
        .args(["-O0", "-shared", "-fPIC", "-s"])
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
    let (out, err, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "--functions",
        "sample",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{out}\n{err}");
    let name = binary.file_name().unwrap().to_str().unwrap();
    let header_name = format!("{name}.h");
    let header = std::fs::read_to_string(project.join(&header_name)).unwrap();
    let globals: Vec<_> = header
        .lines()
        .filter_map(|line| line.strip_prefix("extern "))
        .collect();
    assert_eq!(globals.len(), 1, "{header}");
    let global = globals[0].strip_suffix(';').unwrap();
    assert!(global.contains("dat_"), "{header}");
    let driver = source.with_extension("driver.c");
    let executable = source.with_extension("exe");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "{header_name}"
#include <dlfcn.h>
{global} = 37;
int main(int argc, char **argv) {{
    if (argc != 2) return 2;
    void *module=dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!module) return 3;
    unsigned (*original)(unsigned)=(unsigned (*)(unsigned))dlsym(module,"sample");
    if (!original) return 4;
    for (int i=0;i<100;++i) {{
        int amount=i%16;
        if ((unsigned)sample(amount)!=original((unsigned)amount)) return 1;
    }}
    dlclose(module);
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let built = Command::new(cc)
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
                "{cc} {opt}"
            );
        }
    }
    for path in [source, binary, driver, executable] {
        std::fs::remove_file(path).unwrap();
    }
    std::fs::remove_dir_all(project).unwrap();
}
