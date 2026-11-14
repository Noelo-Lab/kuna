use super::*;

fn ftol2_block(sse_flag: u32) -> Vec<u8> {
    let mut v = FTOL2_SSE_HEAD.to_vec();
    for r in FTOL2_SSE_HEAD_WILD {
        v[r.clone()].copy_from_slice(&sse_flag.to_le_bytes());
    }
    v.extend_from_slice(FTOL2_X87);
    v
}

#[test]
fn full_ftol2_object_names_all_four_entries() {
    let mut code = vec![0xcc; 0x10];
    code.extend(ftol2_block(0x7d4ce0));
    let hits = scan_bytes(&code, 0x651000);
    assert_eq!(
        hits,
        vec![
            (0x651010, "__ftol2_sse"),
            (0x651019, "__ftol2_pentium4"),
            (0x65102c, "__ftol2_sse_excpt"),
            (0x651046, "__ftol2"),
        ]
    );
}

#[test]
fn x87_tail_alone_is_ftol2() {
    let mut code = vec![0x90; 0x40];
    code.extend_from_slice(FTOL2_X87);
    assert_eq!(scan_bytes(&code, 0x1000), vec![(0x1040, "__ftol2")]);
}

#[test]
fn ftol_matches() {
    let mut code = vec![0xc3];
    code.extend_from_slice(FTOL);
    assert_eq!(scan_bytes(&code, 0x400000), vec![(0x400001, "__ftol")]);
}

#[test]
fn one_changed_byte_rejects() {
    let mut code = FTOL.to_vec();
    code[0x11] = 0x08;
    assert!(scan_bytes(&code, 0).is_empty());
    let mut code = ftol2_block(1);
    code[0x15] = 0x90;
    assert_eq!(scan_bytes(&code, 0), vec![(0x36, "__ftol2")]);
}

#[test]
fn truncated_stream_is_ignored() {
    assert!(scan_bytes(&FTOL[..FTOL.len() - 1], 0).is_empty());
}

