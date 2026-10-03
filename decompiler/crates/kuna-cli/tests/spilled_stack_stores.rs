//! A pointer spilled to a stack slot keeps later reads linked to its writes.
mod common;
use common::process;
use std::process::Command;

const FUNCTIONS: &[&str] = &[
    "array_write",
    "array_constant_write",
    "array_constant_nonalias",
    "array_neighbors",
    "shifted_write",
    "array_snapshot",
    "array_nonalias",
    "pointer_move",
    "array_two_writes",
    "array_export",
    "array_escape",
    "array_choose_nonalias",
];

#[test]
fn spilled_stack_pointer_writes_round_trip_with_alias_and_escape_controls() {
    let source = std::fs::read_to_string(common::fixture("spillstoreguard.c")).unwrap();
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "the round trip requires a C compiler"
    );
    for target in [
        "gcc_x86_64",
        "clang_x86_64",
        "clang_aarch64",
        "clang_thumb",
        "clang_mipsel",
        "clang_ppc",
        "clang_x86",
    ] {
        for optimization in ["O0", "O2"] {
            let (compiler, architecture) = target.split_once('_').unwrap();
            let fixture = format!("spillstoreguard_{compiler}_{optimization}_{architecture}.o");
            let functions: Vec<_> = FUNCTIONS
                .iter()
                .copied()
                .filter(|name| !matches!(architecture, "mipsel" | "ppc") || *name != "array_escape")
                .collect();
            let mut args = vec![
                "decompile-all".to_owned(),
                common::fixture(&fixture),
                "--functions".into(),
                functions.join(","),
                "--assert-strict".into(),
                "--assert".into(),
                "prototype move_pointer void move_pointer(int **p,int *q)".into(),
            ];
            for name in &functions {
                let prototype = if *name == "array_export" {
                    format!("void {name}(int a,int x,int y,int *out)")
                } else {
                    format!("int {name}(int a,int x,int y)")
                };
                args.extend(["--assert".into(), format!("prototype {name} {prototype}")]);
            }
            let (printed, stderr, code) =
                common::run_kuna(&args.iter().map(String::as_str).collect::<Vec<_>>());
            assert_eq!(code, 0, "{fixture}: {stderr}");
            let mut reference = source.clone();
            for name in FUNCTIONS {
                reference = reference.replace(&format!("{name}("), &format!("source_{name}("));
            }
            let mut checks = String::new();
            for name in functions.iter().filter(|name| **name != "array_export") {
                checks.push_str(&format!("CHECK({name}(a,x,y),source_{name}(a,x,y));\n"));
            }
            let program = format!("#include <stdio.h>\nstatic int choose;\n\
                void move_pointer(int **p,int *q) {{ if(choose) *p=q; }}\n\
                {reference}\n{printed}\n\
                #define CHECK(got,want) do {{ if((got)!=(want)) {{ puts(#got); return 1; }} }} while(0)\n\
                int main(void) {{\n\
                for(choose=0;choose<2;choose++) for(int a=-3;a<4;a++)\n\
                for(int x=-3;x<4;x++) for(int y=-3;y<4;y++) {{\n{checks}\n\
                int got[4]={{101,-7,-8,202}},want[4]={{101,-7,-8,202}};\n\
                array_export(a,x,y,got+1); source_array_export(a,x,y,want+1);\n\
                for(int i=0;i<4;i++) CHECK(got[i],want[i]);\n\
                }} return 0; }}\n");
            let src = common::scratch_file("spillstoreguard", "c");
            let exe = common::scratch_file("spillstoreguard", "exe");
            std::fs::write(&src, program).unwrap();
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let compile = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fno-strict-aliasing", level])
                        .arg(&src)
                        .arg("-o")
                        .arg(&exe)
                        .output()
                        .expect("compile the printed C");
                    assert!(
                        compile.status.success(),
                        "{fixture} {cc} {level}:\n{}\n{printed}",
                        String::from_utf8_lossy(&compile.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the printed C");
                    assert!(
                        run.status.success(),
                        "{fixture} {cc} {level}:\n{}\n{printed}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
            let _ = std::fs::remove_file(src);
            let _ = std::fs::remove_file(exe);
        }
    }
}
