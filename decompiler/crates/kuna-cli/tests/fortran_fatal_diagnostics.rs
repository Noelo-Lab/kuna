//! Fatal runtime calls end the path, while warning calls retain their continuation.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn fatal_runtime_diagnostics_have_no_fallthrough() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("fatal-diagnostics", "c");
    let binary = source.with_extension("so");
    let names = ["_gfortran_runtime_error", "_gfortran_runtime_error_at",
                 "_gfortran_os_error", "_gfortran_os_error_at",
                 "_gfortran_runtime_warning_at"];
    let mut code = String::new();
    for (i, name) in names.iter().enumerate() {
        let (parameters, arguments) = match i {
            0 => ("const char *, ...", "\"message\""),
            2 => ("const char *", "\"message\""),
            _ => ("const char *, const char *, ...", "\"probe\", \"message\""),
        };
        code.push_str(&format!("extern void {name}({parameters});\nint probe{i}(void) {{ {name}({arguments}); return 9187; }}\n"));
    }
    std::fs::write(&source, code).unwrap();
    let output = Command::new("gcc").args(["-O0", "-fPIC", "-shared"])
        .arg(&source).arg("-o").arg(&binary).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    for (i, name) in names.iter().enumerate() {
        let (body, err, status) = common::run_kuna(&["decompile", binary.to_str().unwrap(), &format!("probe{i}")]);
        assert_eq!(status, 0, "{err}");
        assert!(body.contains(name), "{body}");
        assert_eq!(body.contains("return 0x23e3;"), i == 4, "{body}");
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(binary).unwrap();
}
