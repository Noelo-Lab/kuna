//! A runtime stack reservation on one path of a merge keeps its amount.
//!
//! Open Watcom's `alloca` (`watcom_dos_alloca.c`, Open Watcom 2.0,
//! `wcl -bt=dos -ms -s -ecc -od|-ox -fe=watcom_dos_alloca_od|ox.exe`) lowers
//! SP by `(n + 1) & ~1` only when `stackavail()` says there is room, and
//! returns NULL otherwise. The x86-16 default model
//! has unknown extrapop, so the stack-pointer solver runs; its merge equation
//! used to hand `sp - amount` the other path's constant and rewrite the
//! reservation away, so the pointer printed as a fixed frame local.

use std::process::Command;

#[test]
fn conditional_runtime_reservation_keeps_its_amount() {
    for (fixture, sample, avail) in [
        ("watcom_dos_alloca_od.exe", "0x10029", "0x100bd"),
        ("watcom_dos_alloca_ox.exe", "0x10017", "0x10060"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", crate::common::repo_root().join("specs"))
            .arg("decompile").arg(crate::common::fixture(fixture)).arg(sample)
            .args(["--define-function", "0x10008=observe",
                "--define-function", &format!("{avail}=stackavail"),
                "--define-function", &format!("{sample}=sample_one"),
                "--assert", "prototype observe unsigned short __cdecl16near observe(unsigned short p,unsigned short n)",
                "--assert", "prototype stackavail unsigned short __cdecl16near stackavail(void)",
                "--assert", "prototype sample_one unsigned short __cdecl16near sample_one(unsigned short n)"])
            .output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        let lines: Vec<&str> = code.lines().collect();
        let arm = lines.iter().position(|line| line.trim() == "else {")
            .unwrap_or_else(|| panic!("{fixture}: no reservation arm\n{code}"));
        // The arm's pointer must be computed from the amount `(n + 1) & ~1`,
        // directly or through the variable that holds it.
        let amount = lines.iter().find_map(|line| line.trim().strip_suffix(" = n + 1 & 0xfffe;"))
            .unwrap_or("n + 1");
        assert!(lines[arm + 1..].iter().take_while(|line| line.trim() != "}")
                .any(|line| line.contains(&format!("-{amount}")) || line.contains("-(n + 1")),
            "{fixture}: the reservation lost its runtime amount\n{code}");
        assert!(lines.iter().any(|line| line.trim().ends_with(" = 7;")),
            "{fixture}: the store into the reservation was dropped\n{code}");
    }
}
