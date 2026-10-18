//! Constant array bounds retain their language-specific origin.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn fortran_array_lengths_use_both_bounds() {
    if process::optional_output(Command::new("gfortran").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("array-bounds", "f90");
    let object = source.with_extension("so");
    std::fs::write(&source, "subroutine implicit_origin(a)\nreal :: a(3)\na(3)=1\nend\nsubroutine negative_origin(a)\nreal :: a(-2:2)\na(2)=1\nend\nsubroutine negative_upper(a)\nreal :: a(-8:-4)\na(-4)=1\nend\n").unwrap();
    let output = Command::new("gfortran").args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&source).arg("-o").arg(&object).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    for (name, count) in [("implicit_origin", 3), ("negative_origin", 5), ("negative_upper", 5)] {
        let (body, err, status) = common::run_kuna(&["decompile", object.to_str().unwrap(), name]);
        assert_eq!(status, 0, "{err}");
        assert!(body.lines().next().unwrap().contains(&format!("[{count}]")), "{body}");
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(object).unwrap();
}
