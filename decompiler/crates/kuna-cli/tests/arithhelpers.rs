//! `arithhelpers`: the C a 32-bit caller of a runtime arithmetic helper
//! decompiles to computes what the source computed, recompiled on the host.
use crate::common;
use std::process::Command;

const FUNCTIONS: [&str; 12] =
    ["udiv", "sdiv", "umod", "smod", "shl", "shr", "sar", "mul", "uidiv", "idiv", "imod", "both"];

fn emitted(fixture: &str, option: &str) -> String {
    let (output, error, status) = common::run_kuna(&[
        "decompile-all",
        &common::fixture(fixture),
        "--json",
        "--functions",
        &FUNCTIONS.join(","),
        "--option",
        "arithhelpers",
        option,
    ]);
    assert_eq!(status, 0, "{error}");
    let document: serde_json::Value = serde_json::from_str(&output).unwrap();
    document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            assert!(f["error"].is_null(), "{output}");
            f["code"].as_str().unwrap().to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn round_trip(fixture: &str) {
    let body = emitted(fixture, "on");
    for helper in ["__udivdi3", "__divdi3", "__moddi3", "__divmoddi4", "__aeabi_"] {
        assert!(!body.contains(helper), "{helper} survives in {fixture}:\n{body}");
    }
    assert!(emitted(fixture, "off").contains("__"), "off keeps the helper calls");
    let source = std::fs::read_to_string(common::fixture("arithhelpers.c")).unwrap();
    let mut reference = source.clone();
    for f in FUNCTIONS {
        reference = reference.replace(&format!(" {f}("), &format!(" ref_{f}("));
    }
    let program = format!(
        r#"
#define CONCAT44(h,l) (((unsigned long long)(unsigned)(h)<<32)|(unsigned)(l))
{reference}
{body}
int main(void) {{
  const long long v[] = {{1,-1,2,-3,7,-7,1000,123456789012LL,-98765432109LL,
    0x7fffffffffffffffLL,(long long)0x8000000000000001ULL}};
  const int n = sizeof v / sizeof *v;
  for (int i = 0; i < n; i++)
    for (int j = 0; j < n; j++) {{
      long long a = v[i], b = v[j], r1, r2;
      int s = (int)(b & 31) + (j & 1 ? 32 : 0);
      if (udiv(a,b) != ref_udiv(a,b) || sdiv(a,b) != ref_sdiv(a,b)) return 1;
      if (umod(a,b) != ref_umod(a,b) || smod(a,b) != ref_smod(a,b)) return 2;
      if (shl(a,s) != ref_shl(a,s) || shr(a,s) != ref_shr(a,s) || sar(a,s) != ref_sar(a,s)) return 3;
      if (mul(a,b) != ref_mul(a,b)) return 4;
      if (uidiv(a,b|1) != ref_uidiv(a,b|1)) return 5;
      if ((int)b != 0 && (int)b != -1 && (idiv(a,b) != ref_idiv(a,b) || imod(a,b) != ref_imod(a,b))) return 6;
      if (both(a,b,&r1) != ref_both(a,b,&r2) || r1 != r2) return 7;
    }}
  return 0;
}}
"#
    );
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("round_trip.c");
    std::fs::write(&path, &program).unwrap();
    let mut checked = 0;
    for cc in ["gcc", "clang"] {
        if Command::new(cc).arg("--version").output().is_err() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let executable = root.path().join(format!("{cc}{opt}"));
            let built = Command::new(cc)
                .args([opt, "-fwrapv", "-w"])
                .arg(&path)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(built.status.success(), "{}\n{body}", String::from_utf8_lossy(&built.stderr));
            let status = Command::new(&executable).status().unwrap();
            assert!(status.success(), "{fixture} {cc} {opt}: {status}\n{body}");
            checked += 1;
        }
    }
    assert!(checked > 0, "a C compiler is required");
}

#[test]
fn i386_libgcc_helpers_round_trip() {
    round_trip("arithhelpers_i386");
}

#[test]
fn thumb_aeabi_helpers_round_trip() {
    round_trip("arithhelpers_thumb");
}
