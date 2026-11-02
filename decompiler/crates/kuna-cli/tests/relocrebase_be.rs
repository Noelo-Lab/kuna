//! A big-endian relocatable object's function inventory names each function
//! once, at the address the loader lays it out at.

use crate::common;

#[test]
fn a_big_endian_object_lists_each_function_once() {
    for name in ["relocrebase_be_armeb.o", "relocrebase_be_ppc32.o", "relocrebase_be_mips.o"] {
        let path = common::fixture(name);
        let (stdout, stderr, rc) = common::run_kuna(&["functions", &path, "--json"]);
        assert_eq!(rc, 0, "{name}: {stderr}");
        let result: serde_json::Value = serde_json::from_str(&stdout).expect("JSON inventory");
        let mut seen: Vec<(String, u64)> = result["functions"]
            .as_array()
            .expect("functions")
            .iter()
            .map(|f| (f["name"].as_str().unwrap_or("").to_owned(), f["address"].as_u64().unwrap()))
            .filter(|(n, _)| n == "scale" || n == "greet")
            .collect();
        seen.sort();
        assert_eq!(seen.len(), 2, "{name}: {seen:?}");
        assert!(seen.iter().all(|(_, a)| *a >= 0x40_0000), "{name}: a pre-link address in {seen:?}");
    }
}
