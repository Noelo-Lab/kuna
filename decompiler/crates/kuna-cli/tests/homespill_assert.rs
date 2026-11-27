//! A Windows x64 function keeps the parameters its prologue spills to their home
//! slots, so a `prototype` assertion that repeats a callee's own declaration
//! changes that call and nothing else.
use crate::common;

const FIXTURE: &str = "homespill_pe_x86_64.exe";
const CALLER: &str =
    "unsigned long long sub_140001100(unsigned int a0,unsigned int a1,unsigned int a2,char a3)";

fn decompile_all(extra: &[&str]) -> String {
    let image = common::fixture(FIXTURE);
    let specs = common::repo_root().join("specs");
    let mut args = vec!["decompile-all", image.as_str(), "--sleighpath", specs.to_str().unwrap()];
    args.extend_from_slice(extra);
    let (out, err, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{args:?}: {err}");
    assert!(!err.contains("rejected"), "{args:?}: {err}");
    out
}

fn header(extra: &[&str]) -> String {
    let image = common::fixture(FIXTURE);
    let specs = common::repo_root().join("specs");
    let dir = common::scratch_file("homespill-project", "kuna");
    let mut args = vec![
        "decompile-project",
        image.as_str(),
        "-o",
        dir.to_str().unwrap(),
        "--sleighpath",
        specs.to_str().unwrap(),
    ];
    args.extend_from_slice(extra);
    let (_, err, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{args:?}: {err}");
    std::fs::read_to_string(dir.join(format!("{FIXTURE}.h"))).unwrap()
}

/// Each function's text, keyed by the `// Function:` line that opens it.
fn functions(out: &str) -> Vec<(String, String)> {
    let mut all: Vec<(String, String)> = Vec::new();
    for line in out.lines() {
        if line.starts_with("// Function: ") {
            all.push((line.to_string(), String::new()));
        } else if let Some((_, body)) = all.last_mut() {
            body.push_str(line);
            body.push('\n');
        }
    }
    all
}

fn signature(out: &str, address: &str) -> String {
    let (_, body) = functions(out)
        .into_iter()
        .find(|(head, _)| head.contains(address))
        .unwrap_or_else(|| panic!("no function at {address}: {out}"));
    body.lines().next().unwrap().to_string()
}

fn declaration_of_callee(out: &str) -> String {
    let line = signature(out, "@ 0x140001200");
    format!("prototype sub_140001200 {line}")
}

#[test]
fn spilled_parameters_are_recovered() {
    let out = decompile_all(&[]);
    assert!(out.contains(CALLER), "{out}");
    assert!(out.contains("unsigned long long sub_140001300(unsigned int a0,unsigned int a1)"), "{out}");
    assert!(out.contains("sub_140001300(1,2);"), "{out}");
    assert!(out.contains("unsigned long long sub_140001380(void)"), "{out}");
    assert!(out.contains("unsigned long long sub_1400013c0(void)"), "{out}");
    let jobs = decompile_all(&["--jobs", "2"]);
    for address in ["@ 0x140001100", "@ 0x140001300", "@ 0x140001380", "@ 0x1400013c0"] {
        assert_eq!(signature(&jobs, address), signature(&out, address), "{jobs}");
    }
}

#[test]
fn restating_a_callee_declaration_changes_only_its_call() {
    let plain = decompile_all(&[]);
    let directive = declaration_of_callee(&plain);
    let asserted = decompile_all(&["--assert", &directive]);
    let (before, after) = (functions(&plain), functions(&asserted));
    assert_eq!(before.len(), after.len(), "{asserted}");
    for ((head, old), (_, new)) in before.iter().zip(&after) {
        if head.contains("@ 0x140001100") {
            assert!(new.starts_with(CALLER), "{new}");
            assert!(
                new.contains("sub_140001200(dat_140002000,dat_140002008,(unsigned char *)s_140002018,0x22) =="),
                "{new}"
            );
            continue;
        }
        assert_eq!(old, new, "{head}");
    }
    let (plain_h, asserted_h) = (header(&[]), header(&["--assert", &directive]));
    for name in ["sub_140001100(", "sub_140001200("] {
        let line = |h: &str| h.lines().find(|l| l.contains(name)).map(str::to_string);
        assert_eq!(line(&plain_h), line(&asserted_h), "{asserted_h}");
    }
}

#[test]
fn a_different_callee_declaration_still_reaches_the_call() {
    let five = decompile_all(&[
        "--assert",
        "prototype sub_140001200 long long f(long long *a,long long *b,unsigned char *c,long long d,unsigned char e)",
    ]);
    let typed = "unsigned long long sub_140001100(unsigned int a0,unsigned int a1,unsigned int a2,unsigned char a3)";
    assert_eq!(signature(&five, "@ 0x140001100"), format!("{typed} // return-dupe"), "{five}");
    assert!(five.contains(",0x22,a3) =="), "{five}");
    let two = decompile_all(&["--assert", "prototype sub_140001200 long long f(long long *a,long long *b)"]);
    assert_eq!(signature(&two, "@ 0x140001100"), format!("{CALLER} // return-dupe"), "{two}");
    assert!(two.contains("sub_140001200(dat_140002000,dat_140002008) =="), "{two}");
}
