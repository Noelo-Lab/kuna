//! Declared vtable signatures must reach CALLIND, including a non-default ABI.
use std::path::PathBuf;
use std::process::Command;

fn image(msabi: bool) -> (Vec<u8>, u64) {
    let sysv = [
        0x53, 0x48, 0x8b, 0x07, 0xff, 0x10, 0x5b, 0xc3, 0x53, 0x48, 0x8b, 0x07, 0xff, 0x50, 0x08,
        0x5b, 0xc3,
    ];
    let win = [
        0x48, 0x83, 0xec, 0x28, 0x48, 0x8b, 0x01, 0xff, 0x10, 0x48, 0x83, 0xc4, 0x28, 0xc3, 0x48,
        0x83, 0xec, 0x28, 0x48, 0x8b, 0x01, 0xff, 0x50, 0x08, 0x48, 0x83, 0xc4, 0x28, 0xc3,
    ];
    let mut b = vec![0; 0x1000];
    b[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        b[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        b[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let code: &[u8] = if msabi { &win } else { &sysv };
    let size = (b.len() + code.len()) as u64;
    for (at, value) in [
        (24, 0x401000u64),
        (32, 64),
        (80, 0x400000),
        (88, 0x400000),
        (96, size),
        (104, size),
        (112, 0x1000),
    ] {
        b[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    b.extend_from_slice(code);
    (b, if msabi { 0x40100e } else { 0x401008 })
}

fn run(msabi: bool, typed: bool) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let dir = std::env::temp_dir().join(format!(
        "kuna-typed-vtable-{}-{msabi}-{typed}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let (bytes, setter) = image(msabi);
    let end = 0x400000 + bytes.len();
    let binary = dir.join("fixture.elf");
    std::fs::write(&binary, bytes).unwrap();
    let model = if msabi { "MSABI " } else { "" };
    let fields = if typed {
        format!("float ({model}*get_scale)(void *node); void ({model}*set_scale)(void *node, float scale);")
    } else {
        "void *get_scale; void *set_scale;".to_string()
    };
    let declarations = [
        format!("function 0x401000-{setter:#x}=read_scale"),
        format!("function {setter:#x}-{end:#x}=set_scale"),
        format!("typedef struct VirtualTable {{ {fields} }};"),
        "typedef struct VirtualNode { struct VirtualTable *vtable; };".to_string(),
        format!("prototype read_scale float {model}read_scale(struct VirtualNode *node)"),
        format!("prototype set_scale void {model}set_scale(struct VirtualNode *node, float scale)"),
    ];
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut cmd = Command::new(kuna);
    cmd.args(["decompile-all"])
        .arg(&binary)
        .args(["--addr", "0x401000", "--addr"])
        .arg(format!("{setter:#x}"))
        .args(["--mode", "reliable", "--assert-strict", "--sleighpath"])
        .arg(specs);
    for decl in declarations {
        cmd.args(["--assert", &decl]);
    }
    let out = cmd.output().expect("run kuna");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn typed_vtable_forwards_receiver_and_float() {
    let c = run(false, true);
    assert!(c.contains("->set_scale)(node,scale)"), "{c}");
    assert!(c.contains(")(node);"), "getter did not forward node: {c}");
    assert!(
        !c.contains("(float)(*"),
        "getter lost its float return type: {c}"
    );
}

#[test]
fn typed_vtable_preserves_explicit_msabi_on_sysv_image() {
    let c = run(true, true);
    assert!(c.contains("->set_scale)(node,scale)"), "{c}");
    assert!(c.contains(")(node);"), "getter used the default ABI: {c}");
}

#[test]
fn untyped_vtable_does_not_borrow_the_callers_signature() {
    let c = run(false, false);
    assert!(c.contains("->set_scale)()"), "{c}");
    assert!(!c.contains(")(node,scale)"), "invented a signature: {c}");
}
