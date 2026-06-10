//! Verifier adversarial tests for item w1-base-util (crc32, compression,
//! filemanage). Written by the independent verifier, NOT the porter.
//!
//! Oracle data was produced by compiling small harnesses directly against the
//! vendored C++ sources (`decompiler/cpp/crc32.cc`, `compression.cc` linked
//! with C zlib, `filemanage.cc` run plain AND under
//! `-fsanitize=address,undefined` with identical output):
//!
//! - `tests/data/w1_util_compression_oracle.out`: chunked deflate/inflate
//!   return-value traces, exact zlib bytes for the edge streams, and the
//!   after-stream-end behaviors (`C:`/`E:` lines).
//! - `tests/data/w1_util_filemanage_oracle.out`: `splitPath` vectors (`S|`),
//!   discoverGhidraRoot component walks with skiplevel (`W|`), end-to-end
//!   discoverGhidraRoot layouts (`D|`), matchListDir with allowdot=true
//!   (`M|`, the known read_dir divergence), scanDirectoryRecursive depth
//!   sweeps (`R|`).
//!
//! Known, deliberate divergences these tests pin rather than paper over:
//! - LOSS-004/LOSS-010: the default-level deflate BYTE STREAM differs between
//!   C zlib and miniz_oxide (interop verified both directions instead).
//! - `std::fs::read_dir` never yields "." / "..", so `match_list_dir` with
//!   `allowdot=true` drops exactly those two entries relative to C++.

use kuna_base::compression::{Compress, Decompress};
use kuna_base::crc32::crc_update;
use kuna_base::filemanage::FileManage;

use std::fs;
use std::path::PathBuf;

fn data_file(name: &str) -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data").join(name);
    fs::read_to_string(p).unwrap()
}

fn oracle_line<'a>(oracle: &'a str, prefix: &str) -> &'a str {
    oracle
        .lines()
        .find(|l| l.starts_with(prefix))
        .unwrap_or_else(|| panic!("oracle line {} missing", prefix))
}

fn from_hex(s: &str) -> Vec<u8> {
    (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect()
}

fn to_hex(v: &[u8]) -> String {
    v.iter().map(|b| format!("{:02x}", b)).collect()
}

/// The 5000-byte payload used by the C++ compression oracle.
fn payload(n: usize) -> Vec<u8> {
    (0..n).map(|i| (((i * 7) & 0xff) ^ (i / 253)) as u8).collect()
}

// ---------------------------------------------------------------- crc32

/// Raw crc_update register vectors computed by a harness linked against the
/// vendored crc32.cc (no pre/post conditioning, as the in-tree callers use
/// it). Wrap-prone register values included.
#[test]
fn verify_crc32_raw_register_vectors_match_cpp_oracle() {
    // reg=0, feed bytes 0..=255 in order
    let mut reg: u32 = 0;
    for i in 0..=255u32 {
        reg = crc_update(reg, i);
    }
    assert_eq!(reg, 0x2493092b, "ALLBYTES oracle");

    // reg=0xffffffff, feed (i*37)&0xff for i in 0..1000
    let mut reg: u32 = 0xffffffff;
    for i in 0..1000u32 {
        reg = crc_update(reg, (i * 37) & 0xff);
    }
    assert_eq!(reg, 0x3861c1d2, "K1000 oracle");

    // val wider than 8 bits participates only via (reg^val)&0xff
    assert_eq!(crc_update(0x12345678, 0xdeadbeef), 0x6e7932b1, "VALWIDE oracle");
    // all-ones register
    assert_eq!(crc_update(0xffffffff, 0xff), 0x00ffffff, "WRAPREG oracle");
}

// ---------------------------------------------------------- compression

/// Replay the C++ oracle's inflate schedule on the EXACT bytes C zlib
/// produced (ABYTES line): 7-byte input chunks, 64-byte output buffer,
/// looping while the return value is 0 — the same consumption pattern as
/// slaformat.cc ingestStream. The per-call return values (avail_out), the
/// finish point, and the recovered payload must match C++ call-for-call.
#[test]
fn verify_compression_inflate_trace_matches_cpp_zlib_oracle() {
    let oracle = data_file("w1_util_compression_oracle.out");
    let abytes = from_hex(oracle_line(&oracle, "ABYTES:").trim_start_matches("ABYTES:"));
    let expect_btrace = oracle_line(&oracle, "B:");
    let data = payload(5000);

    let mut dec = Decompress::new();
    let mut plain = Vec::new();
    let mut buf = [0u8; 64];
    let mut btrace = String::from("B:");
    for chunk in abytes.chunks(7) {
        if dec.is_finished() {
            break;
        }
        dec.input(chunk);
        loop {
            let avail = dec.inflate(&mut buf).unwrap();
            plain.extend_from_slice(&buf[..64 - avail as usize]);
            btrace.push_str(&format!(" {}{}", avail, if dec.is_finished() { "F" } else { "" }));
            if avail != 0 || dec.is_finished() {
                break;
            }
        }
    }
    assert_eq!(btrace, expect_btrace, "per-call inflate return values diverge from C++ zlib");
    assert_eq!(plain, data, "decompressed payload diverges");
    assert!(dec.is_finished());

    // C: line — inflate called AGAIN after stream end: C++ zlib returns
    // Z_STREAM_END again (no error), avail_out untouched (=64), finished
    // stays true. flate2 must do the same, not Err.
    let again = dec.inflate(&mut buf).expect("inflate after stream end must not error (C++ C: line)");
    assert_eq!(again, 64, "C: oracle line says 64");
    assert!(dec.is_finished());
}

/// Byte-parity edges of the deflate path where C zlib and miniz_oxide DO
/// agree (pinned so a regression in either direction is caught):
/// empty stream at level -1 (D: line), and the 300-byte incompressible
/// payload at levels 1 and 9 through 1-byte output buffers (F lines: same
/// stored-block encoding, same number of deflate calls). Also the E: line —
/// deflate called again with finish=true after the stream completed does NOT
/// error in C++ (zlib returns Z_STREAM_END, the wrapper returns avail_out).
#[test]
fn verify_compression_deflate_byte_parity_edges_match_cpp_oracle() {
    let oracle = data_file("w1_util_compression_oracle.out");

    // D: empty input, finish immediately (== CompressBuffer flush with no
    // writes). C zlib: 789c030000000001 — must be byte-identical.
    let d_expect = oracle_line(&oracle, "D:").trim_start_matches("D:");
    let mut comp = Compress::new(-1).unwrap();
    comp.input(&[]);
    let mut buf = [0u8; 64];
    let mut out = Vec::new();
    loop {
        let avail = comp.deflate(&mut buf, true).unwrap();
        out.extend_from_slice(&buf[..64 - avail as usize]);
        if avail != 0 {
            break;
        }
    }
    assert_eq!(to_hex(&out), d_expect, "empty-stream deflate bytes diverge from C zlib");

    // E: deflate again after completion — C++ oracle prints "E:ret 64"
    // (no LowlevelError). The port must not error either.
    let e_line = oracle_line(&oracle, "E:");
    assert_eq!(e_line, "E:ret 64", "oracle drift");
    comp.input(&[]);
    let ret = comp
        .deflate(&mut buf, true)
        .expect("deflate after stream end must not error (C++ E: line)");
    assert_eq!(ret, 64);

    // F1/F9: 300-byte high-entropy payload, finish=true, 1-byte output
    // buffer per call; zlib emits a stored block — call count and bytes
    // matched miniz exactly when the oracle was generated; pin both.
    let data = payload(300);
    for level in [1i32, 9] {
        let f_line = oracle_line(&oracle, &format!("F{}:", level));
        let rest = f_line.split_once(":calls=").unwrap().1;
        let (calls_s, hex_s) = rest.split_once(' ').unwrap();
        let expect_calls: u32 = calls_s.parse().unwrap();
        let expect_bytes = from_hex(hex_s.trim());

        let mut comp = Compress::new(level).unwrap();
        comp.input(&data);
        let mut b = [0u8; 1];
        let mut out = Vec::new();
        let mut calls = 0u32;
        loop {
            let avail = comp.deflate(&mut b, true).unwrap();
            if avail == 0 {
                out.push(b[0]);
            }
            calls += 1;
            if avail != 0 {
                break;
            }
        }
        assert_eq!(calls, expect_calls, "level {} single-byte deflate call count", level);
        assert_eq!(out, expect_bytes, "level {} deflate bytes diverge from C zlib", level);
    }
}

/// LOSS-004/LOSS-010 canary: at the default level the compressed BYTE STREAM
/// is allowed to differ from C zlib (and did when this test was written:
/// 3902 bytes vs zlib's 3916 for this payload). What must hold instead:
/// the zlib header is identical (0x78 0x9C), the stream round-trips through
/// the port's own decoder, and — verified at review time with a C++ harness,
/// not assertable here — C zlib decodes the miniz stream (`REV: match=1`).
#[test]
fn verify_compression_default_level_stream_interop_loss010() {
    let oracle = data_file("w1_util_compression_oracle.out");
    let zlib_abytes = from_hex(oracle_line(&oracle, "ABYTES:").trim_start_matches("ABYTES:"));
    let data = payload(5000);

    // replay the A schedule: input chunks of 1000, 64-byte output buffer
    let mut comp = Compress::new(-1).unwrap();
    let mut abytes = Vec::new();
    let mut buf = [0u8; 64];
    for (k, chunk) in data.chunks(1000).enumerate() {
        let last = (k + 1) * 1000 >= data.len();
        comp.input(chunk);
        loop {
            let avail = comp.deflate(&mut buf, last).unwrap();
            abytes.extend_from_slice(&buf[..64 - avail as usize]);
            if avail != 0 {
                break;
            }
        }
    }
    assert_eq!(&abytes[..2], &[0x78, 0x9c], "default-level zlib header must match C zlib");
    if abytes != zlib_abytes {
        // the documented LOSS-010 state: streams differ; both must decode to
        // the same payload (zlib->rust direction proven in the trace test,
        // rust->rust here, rust->zlib at review time)
        let mut dec = Decompress::new();
        dec.input(&abytes);
        let mut plain = Vec::new();
        let mut big = [0u8; 8192];
        loop {
            let avail = dec.inflate(&mut big).unwrap();
            plain.extend_from_slice(&big[..8192 - avail as usize]);
            if avail != 0 || dec.is_finished() {
                break;
            }
        }
        assert!(dec.is_finished());
        assert_eq!(plain, data, "self round-trip of the diverged stream failed");
    }
    // if the streams ever become byte-identical, LOSS-010 can be retired;
    // nothing fails here in that case.
}

// ----------------------------------------------------------- filemanage

/// splitPath vectors against the C++ oracle (`S|input|path|base`), including
/// the wrapped-size_type edges: "/", "//", "a/", "a//", "", "./", and
/// multi-byte UTF-8 components.
#[test]
fn verify_filemanage_split_path_matches_cpp_oracle() {
    let oracle = data_file("w1_util_filemanage_oracle.out");
    let mut count = 0;
    for line in oracle.lines().filter(|l| l.starts_with("S|")) {
        let mut parts = line.splitn(4, '|');
        parts.next();
        let input = parts.next().unwrap();
        let expect_path = parts.next().unwrap();
        let expect_base = parts.next().unwrap();
        let mut path = String::from("stale-path");
        let mut base = String::from("stale-base");
        FileManage::split_path(input, &mut path, &mut base);
        assert_eq!(
            (path.as_str(), base.as_str()),
            (expect_path, expect_base),
            "splitPath({:?}) diverges from C++",
            input
        );
        count += 1;
    }
    assert!(count >= 20, "oracle file truncated? saw {} S-lines", count);
}

/// The discoverGhidraRoot component walk (filemanage.cc:378-390): repeated
/// aliased splitPath(cur,cur,base) calls with the "."/".." skiplevel
/// machinery. The C++ harness transcribed the loop verbatim around the real
/// splitPath; this transcribes the identical loop around the Rust
/// split_path, and the resulting path-element lists and leftover skiplevel
/// must match (`W|argv0|elems|skip=N`). This pins the aliasing-snapshot
/// behavior and the odd-but-faithful "a/" / "./" trailing elements.
#[test]
fn verify_filemanage_component_walk_matches_cpp_oracle() {
    fn walk(argv0: &str) -> (Vec<String>, i32) {
        let mut pathels: Vec<String> = Vec::new();
        let mut cur = argv0.to_string();
        let mut base = String::new();
        let mut skiplevel: i32 = 0;
        loop {
            let sizebefore = cur.len();
            let full = cur.clone(); // C++ aliases full/path; snapshot first
            FileManage::split_path(&full, &mut cur, &mut base);
            if cur.len() == sizebefore {
                break;
            }
            if base == "." {
                skiplevel += 1;
            } else if base == ".." {
                skiplevel += 2;
            }
            if skiplevel > 0 {
                skiplevel -= 1;
            } else {
                pathels.push(base.clone());
            }
        }
        (pathels, skiplevel)
    }

    let oracle = data_file("w1_util_filemanage_oracle.out");
    let mut count = 0;
    for line in oracle.lines().filter(|l| l.starts_with("W|")) {
        let mut parts = line.splitn(4, '|');
        parts.next();
        let argv0 = parts.next().unwrap();
        let elems_s = parts.next().unwrap();
        let skip_s = parts.next().unwrap().trim_start_matches("skip=");
        let expect_elems: Vec<String> = if elems_s.is_empty() {
            Vec::new()
        } else {
            elems_s.split(',').map(str::to_string).collect()
        };
        let expect_skip: i32 = skip_s.parse().unwrap();
        let (elems, skip) = walk(argv0);
        assert_eq!(elems, expect_elems, "component walk of {:?} diverges", argv0);
        assert_eq!(skip, expect_skip, "skiplevel after walk of {:?} diverges", argv0);
        count += 1;
    }
    assert!(count >= 15, "oracle file truncated? saw {} W-lines", count);
}

/// Build the exact fixture tree the C++ oracle ran against and replay the
/// filesystem-dependent lines: discoverGhidraRoot end-to-end (development,
/// dotted-argv0, install, and not-found layouts; `D|` lines),
/// scanDirectoryRecursive depth sweep with the nested-match no-descend rule
/// (`R|` lines), and the matchListDir allowdot divergence (`M|` line): the
/// C++ readdir-based listing includes "." and ".."; std::fs::read_dir never
/// yields them, so the Rust result must equal the C++ result MINUS exactly
/// those two entries (the documented loss).
#[test]
fn verify_filemanage_fs_layouts_match_cpp_oracle() {
    let root = std::env::temp_dir().join(format!("kuna_verify_w1util_{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    for d in [
        "devroot/ghidra.master.git/Ghidra/bin",
        "devroot/ghidra.git/Ghidra",
        "inst/Ghidra/bin",
        "inst/server/server.conf",
        "plain/Ghidra/bin",
        "mdir/sub",
        "scan/data/languages",
        "scan/other/data",
        "scan/data/data",
    ] {
        fs::create_dir_all(root.join(d)).unwrap();
    }
    fs::write(root.join("mdir/x86.sla"), b"x").unwrap();
    fs::write(root.join("mdir/.hidden"), b"x").unwrap();
    let r = root.to_str().unwrap().to_string();
    let subst = |s: &str| s.replace("$R", &r);

    let oracle = data_file("w1_util_filemanage_oracle.out");

    // D| lines: end-to-end discoverGhidraRoot
    for (tag, argv0) in [
        ("D|dev|", format!("{}/devroot/ghidra.master.git/Ghidra/bin/decomp", r)),
        ("D|dotted|", format!("{}/devroot/ghidra.master.git/./Ghidra/bin/decomp", r)),
        ("D|install|", format!("{}/inst/Ghidra/bin/decomp", r)),
        ("D|none|", format!("{}/plain/Ghidra/bin/decomp", r)),
    ] {
        let expect = subst(oracle_line(&oracle, tag).trim_start_matches(tag));
        assert_eq!(
            FileManage::discover_ghidra_root(&argv0),
            expect,
            "discoverGhidraRoot {} diverges",
            tag
        );
    }

    // R| lines: scanDirectoryRecursive depth sweep (results sorted, since
    // readdir/read_dir order is filesystem-defined on both sides)
    for depth in 0..=2i32 {
        let tag = format!("R|data|{}|", depth);
        let expect_s = subst(oracle_line(&oracle, &tag).trim_start_matches(tag.as_str()));
        let expect: Vec<String> = if expect_s.is_empty() {
            Vec::new()
        } else {
            expect_s.split(',').map(str::to_string).collect()
        };
        let mut res: Vec<String> = Vec::new();
        FileManage::scan_directory_recursive(&mut res, "data", &format!("{}/scan", r), depth);
        res.sort();
        assert_eq!(res, expect, "scanDirectoryRecursive depth {} diverges", depth);
    }

    // M| line: matchListDir(match="", suffix, allowdot=true). C++ saw
    // [".", "..", ".hidden", "sub", "x86.sla"]; Rust must see the same set
    // minus the "." and ".." entries — and nothing else may differ.
    let tag = "M|allowdot-empty-suffix|";
    let expect_s = subst(oracle_line(&oracle, tag).trim_start_matches(tag));
    let cpp: Vec<String> = expect_s.split(',').map(str::to_string).collect();
    let dot = format!("{}/mdir/.", r);
    let dotdot = format!("{}/mdir/..", r);
    assert!(cpp.contains(&dot) && cpp.contains(&dotdot), "C++ oracle must list . and ..");
    let cpp_minus: Vec<String> =
        cpp.into_iter().filter(|e| *e != dot && *e != dotdot).collect();
    let mut res: Vec<String> = Vec::new();
    FileManage::match_list_dir(&mut res, "", true, &format!("{}/mdir", r), true);
    res.sort();
    assert_eq!(
        res, cpp_minus,
        "allowdot=true listing must differ from C++ by exactly the dot entries"
    );

    let _ = fs::remove_dir_all(&root);
}
