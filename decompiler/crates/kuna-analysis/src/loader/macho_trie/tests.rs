use super::*;

/// A trie node to lay out: its terminal bytes (empty for none) and its edges,
/// each a label and the index of the child node.
struct Node {
    terminal: Vec<u8>,
    edges: Vec<(Vec<u8>, usize)>,
}

fn node(terminal: Vec<u8>, edges: &[(&str, usize)]) -> Node {
    Node {
        terminal,
        edges: edges.iter().map(|(label, child)| (label.as_bytes().to_vec(), *child)).collect(),
    }
}

fn uleb_bytes(mut value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return out;
        }
        out.push(byte | 0x80);
    }
}

/// A child offset as a three-byte ULEB128 (padded, as a linker may write it),
/// so every node's size is known before the layout.
fn padded(value: usize) -> [u8; 3] {
    assert!(value < 1 << 21);
    [0x80 | (value & 0x7f) as u8, 0x80 | ((value >> 7) & 0x7f) as u8, (value >> 14) as u8]
}

/// Lay `nodes` out in order, node 0 the root.
fn lay_out(nodes: &[Node]) -> Vec<u8> {
    let size = |n: &Node| {
        uleb_bytes(n.terminal.len() as u64).len()
            + n.terminal.len()
            + 1
            + n.edges.iter().map(|(label, _)| label.len() + 4).sum::<usize>()
    };
    let mut offsets = Vec::new();
    let mut at = 0;
    for n in nodes {
        offsets.push(at);
        at += size(n);
    }
    let mut out = Vec::new();
    for n in nodes {
        out.extend(uleb_bytes(n.terminal.len() as u64));
        out.extend(&n.terminal);
        out.push(n.edges.len() as u8);
        for (label, child) in &n.edges {
            out.extend(label);
            out.push(0);
            out.extend(padded(offsets[*child]));
        }
    }
    out
}

fn export(flags: u8, values: &[u64]) -> Vec<u8> {
    let mut out = uleb_bytes(u64::from(flags));
    for &value in values {
        out.extend(uleb_bytes(value));
    }
    out
}

fn names(trie: &[u8]) -> Vec<(String, ExportKind, u64)> {
    walk(trie)
        .into_iter()
        .map(|(name, kind, value)| (String::from_utf8(name).unwrap(), kind, value))
        .collect()
}

/// Every export kind `mach-o/loader.h` defines: a re-export names another
/// image and is left out, a stub-and-resolver keeps its stub, a weak
/// definition is an ordinary address.
#[test]
fn every_export_kind() {
    let mut reexport = export(EXPORT_SYMBOL_FLAGS_REEXPORT, &[1]);
    reexport.extend(b"_elsewhere\0");
    let trie = lay_out(&[
        node(Vec::new(), &[("_audit_", 1)]),
        node(
            Vec::new(),
            &[
                ("function", 2),
                ("data", 3),
                ("weak", 4),
                ("tls", 5),
                ("abs", 6),
                ("reexport", 7),
                ("resolver", 8),
            ],
        ),
        node(export(0, &[0x328]), &[]),
        node(export(0, &[0x4000]), &[]),
        node(export(object::macho::EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION, &[0x340]), &[]),
        node(export(EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL, &[0x8000]), &[]),
        node(export(EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE, &[0x1234]), &[]),
        node(reexport, &[]),
        node(export(EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER, &[0x350, 0x360]), &[]),
    ]);
    assert_eq!(
        names(&trie),
        vec![
            ("_audit_function".into(), ExportKind::Regular, 0x328),
            ("_audit_data".into(), ExportKind::Regular, 0x4000),
            ("_audit_weak".into(), ExportKind::Regular, 0x340),
            ("_audit_tls".into(), ExportKind::ThreadLocal, 0x8000),
            ("_audit_abs".into(), ExportKind::Absolute, 0x1234),
            ("_audit_resolver".into(), ExportKind::Regular, 0x350),
        ]
    );
}

/// A node that is both a terminal and a prefix keeps both names, and an
/// unknown kind (3) is dropped while its children are still walked.
#[test]
fn terminal_prefixes_and_unknown_kinds() {
    let trie = lay_out(&[
        node(Vec::new(), &[("_a", 1)]),
        node(export(0, &[0x10]), &[("b", 2), ("c", 3)]),
        node(export(3, &[0x20]), &[("d", 4)]),
        node(export(0, &[0x30]), &[]),
        node(export(0, &[0x40]), &[]),
    ]);
    assert_eq!(
        names(&trie),
        vec![
            ("_a".into(), ExportKind::Regular, 0x10),
            ("_abd".into(), ExportKind::Regular, 0x40),
            ("_ac".into(), ExportKind::Regular, 0x30),
        ]
    );
}

/// An edge back to the root, to the node itself, or to a child already
/// entered ends there: each node is entered once.
#[test]
fn cycles_and_shared_children_end() {
    let trie = lay_out(&[
        node(Vec::new(), &[("_x", 1)]),
        node(export(0, &[0x10]), &[("loop", 0), ("self", 1), ("y", 2), ("z", 2)]),
        node(export(0, &[0x20]), &[("back", 1)]),
    ]);
    assert_eq!(
        names(&trie),
        vec![("_x".into(), ExportKind::Regular, 0x10), ("_xy".into(), ExportKind::Regular, 0x20)]
    );
}

/// A ULEB128 is at most 64 bits, and nothing is read past the end.
#[test]
fn uleb_overflow_and_truncation() {
    let mut max = vec![0xff; 9];
    max.push(0x01);
    assert_eq!(uleb(&max, &mut 0, max.len()), Some(u64::MAX));
    let mut wide = vec![0xff; 9];
    wide.push(0x02);
    assert_eq!(uleb(&wide, &mut 0, wide.len()), None);
    let mut long = vec![0x80; 10];
    long.push(0x00);
    assert_eq!(uleb(&long, &mut 0, long.len()), None);
    assert_eq!(uleb(&[0x80, 0x80], &mut 0, 2), None);
    assert_eq!(uleb(&[0x05, 0x06], &mut 0, 0), None);

    let mut root = vec![0xff; 11];
    root.push(0);
    assert!(walk(&root).is_empty(), "a root whose terminal size overflows has no names");

    let mut bad_value = lay_out(&[node(Vec::new(), &[("_v", 1)]), node(export(0, &[0x10]), &[("w", 2)]), node(export(0, &[0x20]), &[])]);
    let at = bad_value.windows(2).position(|w| w == [0x00, 0x10]).unwrap() + 1;
    bad_value[at] = 0x90;
    assert_eq!(
        names(&bad_value),
        vec![("_vw".into(), ExportKind::Regular, 0x20)],
        "a terminal whose value runs into its child count is dropped; its children are not"
    );
}

/// A terminal, a child offset or a label that runs past the end drops only
/// what it covers.
#[test]
fn offsets_past_the_end() {
    let mut trie = lay_out(&[
        node(Vec::new(), &[("_in", 1), ("_out", 2)]),
        node(export(0, &[0x10]), &[]),
        node(export(0, &[0x20]), &[]),
    ]);
    let out = trie.windows(5).position(|w| w == b"_out\0").unwrap() + 5;
    trie[out..out + 3].copy_from_slice(&padded(0x1f_ffff));
    assert_eq!(names(&trie), vec![("_in".into(), ExportKind::Regular, 0x10)]);

    let past = [0x7f, 0x00];
    assert!(walk(&past).is_empty(), "a terminal size past the end");

    let mut label = vec![0x00, 0x01];
    label.extend(b"_ok\0");
    label.extend(padded(9));
    label.extend([0x02, 0x00, 0x10, 0x01]);
    label.extend(b"_unterminated");
    assert_eq!(names(&label), vec![("_ok".into(), ExportKind::Regular, 0x10)]);
}

/// A label longer than [`MAX_NAME`] ends its node's edges; an empty one is
/// skipped.
#[test]
fn long_and_empty_labels() {
    let long = "x".repeat(MAX_NAME + 1);
    let trie = lay_out(&[
        node(Vec::new(), &[("", 1), ("_short", 1), (&long, 2)]),
        node(export(0, &[0x10]), &[]),
        node(export(0, &[0x20]), &[]),
    ]);
    assert_eq!(names(&trie), vec![("_short".into(), ExportKind::Regular, 0x10)]);
}

/// A chain of terminals, each one byte longer than the last, would hold names
/// quadratic in its size: the walk keeps at most [`NAMES_PER_BYTE`] name bytes
/// per trie byte.
#[test]
fn a_chain_cannot_allocate_quadratically() {
    let count = 60_000;
    let nodes: Vec<Node> = (0..count)
        .map(|i| {
            let edges: &[(&str, usize)] = if i + 1 < count { &[("a", i + 1)] } else { &[] };
            node(export(0, &[(i & 0x7f) as u64]), edges)
        })
        .collect();
    let trie = lay_out(&nodes);
    let out = walk(&trie);
    let kept: usize = out.iter().map(|(name, _, _)| name.len()).sum();
    assert!(!out.is_empty() && out.len() < count);
    assert!(kept <= trie.len() * NAMES_PER_BYTE, "{kept} name bytes from a {}-byte trie", trie.len());
}

/// Nodes whose edges all start in one long run without a NUL would make every
/// node rescan it: the reads are capped at [`WORK_PER_BYTE`] per trie byte, so
/// the walk stops after the first quarter of the 255 nodes (each rescans
/// [`MAX_NAME`] bytes of a 1 MiB run) instead of reading 16 MiB.
#[test]
fn overlapping_nodes_cannot_rescan() {
    let mut trie = vec![0u8, 255];
    let run_start = 2 + 255 * 7;
    for i in 0..255 {
        trie.extend(format!("{i:03}").as_bytes());
        trie.push(0);
        trie.extend(padded(run_start + i));
    }
    trie.extend(std::iter::repeat_n(b'A', 1 << 20));
    let entered = walk(&trie).len();
    assert!(entered > 0 && entered < 255 / 3, "{entered} of 255 nodes entered");
}

/// Random byte flips, truncations and garbage never panic and never keep more
/// than the name budget.
#[test]
fn mutated_tries_stay_bounded() {
    let seed = lay_out(&[
        node(Vec::new(), &[("_audit_", 1), ("_b", 2)]),
        node(export(0, &[0x328]), &[("f", 2), ("g", 3)]),
        node(export(EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER, &[0x350, 0x360]), &[("h", 3)]),
        node(export(EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL, &[0x8000]), &[]),
    ]);
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for round in 0..20_000 {
        let mut trie = seed.clone();
        for _ in 0..(next() % 4 + 1) {
            let at = (next() as usize) % trie.len();
            trie[at] = next() as u8;
        }
        if round % 5 == 0 {
            trie.truncate((next() as usize) % trie.len() + 1);
        }
        if round % 7 == 0 {
            trie = (0..(next() % 64 + 1)).map(|_| next() as u8).collect();
        }
        let kept: usize = walk(&trie).iter().map(|(name, _, _)| name.len()).sum();
        assert!(kept <= trie.len() * NAMES_PER_BYTE);
    }
}

/// A 32-bit ARM trie's odd address is a Thumb entry only where its even
/// neighbour is code: an exported byte at an odd data address keeps its
/// address, and nothing changes off ARM.
#[test]
fn the_thumb_bit_is_cleared_only_in_code() {
    let code = [(0x83a8, 0x1f48)];
    assert_eq!(thumb_entry(0x83ef, true, &code), 0x83ee);
    assert_eq!(thumb_entry(0x83ee, true, &code), 0x83ee);
    assert_eq!(thumb_entry(0xc9e5, true, &code), 0xc9e5);
    assert_eq!(thumb_entry(0x83ef, false, &code), 0x83ef);
}
