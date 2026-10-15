//! An unlocked SPARC entry argument survives a later double-width register read.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn unlocked_sparc_math_wrappers_preserve_entry_values() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        eprintln!("skipping SPARC compiler control: Clang unavailable");
        return;
    }
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    let dir = common::scratch_file("multireg-call-inputs", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("wrappers.c");
    let object = dir.join("wrappers.o");
    let operations = ["acos", "asin", "cos", "exp", "sin", "sqrt", "tan"];
    let mut wrappers = String::from("double number(void *context,int index); void push(void *context,double value);\n");
    for op in operations {
        wrappers.push_str(&format!("double {op}(double); int wrap_{op}(void *context) {{ push(context,{op}(number(context,1))); return 1; }}\n"));
    }
    std::fs::write(&source, wrappers).unwrap();
    process::required_output(Command::new("clang").args([
        "--target=sparc-linux-gnu", "-O2", "-fintegrated-as", "-fno-builtin", "-c",
    ]).arg(&source).arg("-o").arg(&object));
    for overlap in ["off", "in", "full"] {
        let mut args = vec!["decompile-all".to_owned(), object.to_str().unwrap().to_owned(),
            "--json".to_owned(), "--assert-strict".to_owned(),
            "--option".to_owned(), "multiregargs".to_owned(), "on".to_owned(),
            "--option".to_owned(), "calloverlap".to_owned(), overlap.to_owned()];
        for prototype in ["prototype number double number(void *context,int index)",
            "prototype push void push(void *context,double value)"] {
            args.extend(["--assert".to_owned(), prototype.to_owned()]);
        }
        for op in operations {
            args.extend(["--assert".to_owned(), format!("prototype {op} double {op}(double value)")]);
        }
        let borrowed: Vec<_> = args.iter().map(String::as_str).collect();
        let (out, err, rc) = common::run_kuna(&borrowed);
        assert_eq!(rc, 0, "{err}\n{out}");
        let json: serde_json::Value = serde_json::from_str(&out).unwrap();
        let functions = json["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 7);
        let mut printed = String::from("#include <math.h>\n#include <stdint.h>\nstatic void *expected=(void*)(uintptr_t)0x12345;\nstatic double supplied,observed; static int errors,calls;\ndouble number(void *context,int index) { if(context!=expected||index!=1) ++errors; ++calls; return supplied; }\nvoid push(void *context,double value) { if(context!=expected) ++errors; observed=value; ++calls; }\n");
        for function in functions {
            assert!(function["error"].is_null(), "{function}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.lines().next().unwrap().contains("(void)"), "{code}");
            printed.push_str(code);
            printed.push('\n');
        }
        printed.push_str("int main(void) {\n");
        for op in operations {
            printed.push_str(&format!("for(int i=1;i<=3;++i) {{ supplied=i*0.25; calls=0; if(wrap_{op}(expected)!=1) ++errors; if(calls!=2||fabs(observed-{op}(supplied))>1e-12) ++errors; }}\n"));
        }
        printed.push_str("return errors ? 1 : 0; }\n");
        let recovered = dir.join(format!("printed-{overlap}.c"));
        std::fs::write(&recovered, printed).unwrap();
        for cc in &compilers {
            for optimization in ["-O0", "-O2"] {
                let binary = dir.join(format!("{overlap}-{cc}-{optimization}"));
                process::required_output(Command::new(cc).args([optimization, "-Werror"])
                    .arg(&recovered).args(["-lm", "-o"]).arg(&binary));
                process::required_output(&mut Command::new(&binary));
            }
        }
    }
}
