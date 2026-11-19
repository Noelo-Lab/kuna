//! arm64e virtual calls are reference edges: a call to the one method a slot
//! holds, a `vcall` to each of several, including slots of a kernel
//! collection's other entries.

use crate::common;

fn fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../kuna-analysis/tests/fixtures").join(name);
    path.to_str().unwrap().to_string()
}

#[test]
fn a_resolved_virtual_call_is_a_call_edge() {
    let shapes = fixture("macho_arm64e_vcall_shapes");
    let (on, err, status) = common::run_kuna(&["xrefs", &shapes, "--to", "_S_m2"]);
    assert_eq!(status, 0, "{err}");
    assert!(on.lines().any(|l| l == "0x1000004a0\tcall\t_call_swift+0x1c\tbraa x8, x17"), "{on}");
    let (off, err, status) =
        common::run_kuna(&["xrefs", &shapes, "--to", "_S_m2", "--option", "arm64evcall", "off"]);
    assert_eq!(status, 0, "{err}");
    assert!(off.starts_with("# 0 references"), "{off}");
}

#[test]
fn an_overridden_slot_is_a_vcall_edge_to_every_candidate() {
    let kc = fixture("kernelcache/fileset_vcall.macho");
    let (from, err, status) = common::run_kuna(&["xrefs", &kc, "--from", "kernel_vcall_m1", "--kind", "vcall"]);
    assert_eq!(status, 0, "{err}");
    let rows: Vec<&str> = from.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(
        rows,
        [
            "0xfffffff007007008\tvcall\tBase_m1\t@0xfffffff007007054\tbraa x8, x17",
            "0xfffffff007007800\tvcall\t-\t@0xfffffff007007054\tbraa x8, x17",
        ],
        "{from}"
    );
}
