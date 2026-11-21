//! tar, zip and U-Boot FIT inputs through the built CLI: member listing,
//! selection, and a FIT image loaded at the address its node states.
//!
//! `fixtures/archive_inputs` holds the archives and the sources of the FIT.
//! `objs.tar`, `objs.tar.gz` and `objs.zip` store `sq.o`, `notes.txt` and
//! `lib/cube.o`, the members of `fixtures/archive_members/libm2.a` (clang
//! 22.1.8, x86-64 Linux; GNU tar 1.35 `--format=ustar --owner=0 --group=0
//! --numeric-owner --mtime=@0`, `gzip -9n`, Info-ZIP 3.0 `zip -X`).
//! `fit.itb` is `fit.its` compiled by dtc 1.7.2, as `mkimage -f` does:
//! `clang --target=aarch64-linux-gnu -O1 -fno-inline -ffreestanding -nostdlib
//! -fuse-ld=lld -static -Wl,-Ttext=0x80080000 -Wl,-e,kernel_entry kern.c -o
//! kern.elf && llvm-objcopy -O binary -j .text kern.elf kern.bin && gzip -9nc
//! kern.bin > kern.bin.gz && dtc -I dts -O dtb -o board.dtb board.dts && dtc
//! -I dts -O dtb -o fit.itb fit.its`.

use crate::common;

fn fixture(name: &str) -> String {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/archive_inputs");
    dir.join(name).to_string_lossy().into_owned()
}

#[test]
fn tar_and_zip_members_list_select_and_decompile() {
    for archive in ["objs.tar", "objs.tar.gz", "objs.zip"] {
        let path = fixture(archive);
        let (out, err, code) = common::run_kuna(&["functions", &path, "--member"]);
        assert_eq!(code, 0, "{archive}: {err}");
        assert_eq!(out, "sq.o\t1064\nnotes.txt\t14\nlib/cube.o\t1064\n", "{archive}");

        let (out, err, code) = common::run_kuna(&["functions", &path]);
        assert_eq!(code, 0, "{archive}: {err}");
        assert_eq!(out, "0x400000\tsq.o:sq\n0x400000\tlib/cube.o:cube\n", "{archive}");

        let (out, err, code) = common::run_kuna(&["decompile", &path, "--member", "lib/cube.o", "cube"]);
        assert_eq!(code, 0, "{archive}: {err}");
        assert!(out.contains("return a0 * a0 * a0;"), "{archive}: {out}");

        let (out, err, code) = common::run_kuna(&["decompile", &format!("{path}:sq.o"), "sq"]);
        assert_eq!(code, 0, "{archive}: {err}");
        assert!(out.contains("return a0 * a0;"), "{archive}: {out}");

        let (out, err, code) = common::run_kuna(&["decompile", &path, "cube"]);
        assert_eq!(code, 0, "{archive}: {err}");
        assert!(err.contains("using member lib/cube.o"), "{archive}: {err}");
        assert!(out.contains("int cube(int a0)"), "{archive}: {out}");

        let (_, err, code) = common::run_kuna(&["decompile", &path, "nosuch"]);
        assert_eq!(code, 2, "{archive}: {err}");
        assert!(err.contains("select one with --member NAME (sq.o, lib/cube.o)"), "{archive}: {err}");
    }
}

#[test]
fn zip_listing_json_marks_the_object_members() {
    let (out, err, code) = common::run_kuna(&["decompile", &fixture("objs.zip"), "--member", "--json"]);
    assert_eq!(code, 0, "{err}");
    let doc: serde_json::Value = serde_json::from_str(&out).expect("one JSON document");
    let rows: Vec<_> = doc["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| (m["name"].as_str().unwrap(), m["object"].as_bool().unwrap()))
        .collect();
    assert_eq!(rows, [("sq.o", true), ("notes.txt", false), ("lib/cube.o", true)]);
}

#[test]
fn an_archive_without_objects_passes_through_unless_a_member_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let tar = dir.path().join("notes.tar");
    let mut header = [0u8; 512];
    header[..9].copy_from_slice(b"notes.txt");
    header[124..135].copy_from_slice(b"00000000005");
    header[148..156].fill(b' ');
    header[156] = b'0';
    let sum: u32 = header.iter().map(|&b| u32::from(b)).sum();
    header[148..155].copy_from_slice(format!("{sum:06o}\0").as_bytes());
    let mut bytes = header.to_vec();
    bytes.extend(b"hello");
    bytes.resize(2560, 0);
    std::fs::write(&tar, bytes).unwrap();
    let tar = tar.to_str().unwrap();

    let (_, err, code) = common::run_kuna(&["functions", tar]);
    assert_eq!(code, 1, "{err}");
    assert!(err.contains("could not build an architecture"), "{err}");

    let (out, err, code) = common::run_kuna(&["functions", tar, "--member"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out, "notes.txt\t5\n");
}

#[test]
fn fit_lists_its_images_and_loads_the_default_kernel_at_its_address() {
    let path = fixture("fit.itb");
    let (out, err, code) = common::run_kuna(&["functions", &path, "--member"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(
        out,
        "kernel-1\t100\ttype=kernel\tarch=arm64\tos=linux\tcompression=gzip\tload=0x80080000\tentry=0x8008000c\n\
         fdt-1\t137\ttype=flat_dt\tarch=arm64\tcompression=none\n"
    );

    let (out, err, code) = common::run_kuna(&["functions", &path, "--member", "--json"]);
    assert_eq!(code, 0, "{err}");
    let doc: serde_json::Value = serde_json::from_str(&out).expect("one JSON document");
    assert_eq!(doc["members"][0]["load"], "0x80080000");
    assert_eq!(doc["members"][1]["load"], serde_json::Value::Null);

    let (out, err, code) = common::run_kuna(&["decompile-all", &path]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("using member kernel-1 of the default configuration"), "{err}");
    assert!(err.contains("U-Boot FIT image: 100 bytes at 0x80080000, target AARCH64:LE:64:v8A, entries [8008000c]"), "{err}");
    assert!(out.contains("// Function: sub_8008000c @ 0x8008000c"), "{out}");
    assert!(out.contains("v1 = sub_80080000(v3);"), "the call resolves at the load address: {out}");
    assert!(out.contains("return a0 * 3 + 7;"), "{out}");

    let (out, err, code) = common::run_kuna(&["decompile", &path, "--member", "kernel-1", "--addr", "0x80080000"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("return a0 * 3 + 7;"), "{out}");
}

#[test]
fn a_device_tree_that_is_no_fit_is_not_an_archive() {
    let dir = tempfile::tempdir().unwrap();
    let dtb = dir.path().join("board.dtb");
    let fit = std::fs::read(fixture("fit.itb")).unwrap();
    let at = fit.windows(4).rposition(|w| w == [0xd0, 0x0d, 0xfe, 0xed]).unwrap();
    std::fs::write(&dtb, &fit[at..at + 137]).unwrap();
    let (_, err, code) = common::run_kuna(&["functions", dtb.to_str().unwrap(), "--member"]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("--member applies to an archive input"), "{err}");
}
