//! Verifier adversarial tests for w1-base-xml — ROUND 2 (post-repair).
//!
//! Round 1 (`verify_w1_base_xml.rs`) pinned the parser/scanner against a
//! C++ oracle (485 byte-identical traces) and exposed F1: the recursive DOM
//! `Drop` aborted the process beyond the bison depth cap.  The repair
//! (commit "rport/w1-base-xml: repair round fixes") replaced it with a
//! worklist `Drop for Element` that steals grandchildren via
//! `Rc::try_unwrap` from sole-owner children and merely decrements shared
//! handles.  These tests attack the repair itself:
//!
//! - the steal must NOT be observable through a surviving shared handle
//!   (a live `tagmap`/clone subtree must stay intact after the parent
//!   document drops, and must itself tear down non-recursively later);
//! - shared-handle teardown must be stack-safe in BOTH extreme drop
//!   orders (shallow-first: a long sequence of O(1) root drops;
//!   deepest-first: one worklist drain over the whole chain);
//! - wide-and-deep trees (worklist holding many siblings per level);
//! - the module-doc claim at full strength: depth 200000 parse AND drop on
//!   a default 2 MiB cargo-test thread, plus the LOSS-010 pin that the
//!   first C++-rejected depth (4998, "memory exhausted"/RET 2 in the
//!   oracle) is accepted by the Rust port.

use std::rc::Rc;

use kuna_base::xml::{xml_tree, Document, DocumentStorage, Element};

/// `<a><a>...x...</a></a>` nested `depth` deep (same shape as round 1).
fn nested_doc(depth: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for _ in 0..depth {
        v.extend_from_slice(b"<a>");
    }
    v.push(b'x');
    for _ in 0..depth {
        v.extend_from_slice(b"</a>");
    }
    v.push(b'\n');
    v
}

/// Iteratively walk a chain from `start` to the single leaf; return
/// (levels, leaf content). No recursion in the test itself.
fn walk_chain(start: &Rc<Element>) -> (usize, Vec<u8>) {
    let mut el: &Element = start;
    let mut levels = 1usize;
    while let Some(child) = el.get_children().first() {
        el = child;
        levels += 1;
    }
    (levels, el.get_content().to_vec())
}

/// The worklist Drop steals grandchildren from sole-owner children.  That
/// optimization must be invisible through a live shared handle: keep an
/// `Rc` to an element 10000 levels down a 50000-deep chain, drop the
/// Document (everything above the kept node is freed through the worklist;
/// the kept node is shared, so its parent's Drop may only decrement), then
/// prove the surviving 40000-deep subtree is fully intact, then drop the
/// survivor — which must itself tear down without overflowing the stack.
#[test]
fn verify_xml_r2_shared_subtree_survives_parent_drop_then_drops_safely() {
    const DEPTH: usize = 50_000;
    const KEEP_AT: usize = 10_000; // 0-based level of the kept element
    let doc: Document = xml_tree(&nested_doc(DEPTH)).expect("parse");
    let mut kept: Rc<Element> = Rc::clone(doc.get_root());
    for _ in 0..KEEP_AT {
        let next = Rc::clone(kept.get_children().first().expect("chain child"));
        kept = next;
    }
    drop(doc); // must free levels 0..KEEP_AT without touching the survivor
    assert_eq!(Rc::strong_count(&kept), 1, "survivor must now be sole-owned");
    let (levels, content) = walk_chain(&kept);
    assert_eq!(levels, DEPTH - KEEP_AT, "shared subtree must be intact");
    assert_eq!(content, b"x");
    drop(kept); // 40000-deep teardown from a shared survivor: no overflow
}

/// Shared handles at EVERY level of a deep chain (the `DocumentStorage::
/// tagmap` pattern), torn down in both extreme orders.
///
/// shallow-first (root key sorts first): each tagmap entry drop frees
/// exactly one element whose child is still tagmap-shared — a 20000-long
/// sequence of O(1) drops.  deepest-first (root key sorts last): 19999
/// pure decrements, then the root drop drains the whole now-sole-owned
/// chain through one worklist.  Neither may recurse.
#[test]
fn verify_xml_r2_tagmap_shared_handles_both_drop_orders() {
    const DEPTH: usize = 20_000;
    for deepest_first in [false, true] {
        // distinct element names so every level gets its own tagmap key;
        // key text chooses the BTreeMap (lexicographic) drop order.
        let mut v = Vec::new();
        for i in 0..DEPTH {
            let k = if deepest_first { DEPTH - 1 - i } else { i };
            v.extend_from_slice(format!("<e{k:06}>").as_bytes());
        }
        v.push(b'x');
        for i in (0..DEPTH).rev() {
            let k = if deepest_first { DEPTH - 1 - i } else { i };
            v.extend_from_slice(format!("</e{k:06}>").as_bytes());
        }
        v.push(b'\n');

        let mut storage = DocumentStorage::new();
        {
            let doc = storage.parse_document(&v).expect("parse");
            let mut el: Rc<Element> = Rc::clone(doc.get_root());
            loop {
                let next = el.get_children().first().map(Rc::clone);
                storage.register_tag(&el);
                match next {
                    Some(n) => el = n,
                    None => break,
                }
            }
        }
        // spot-check registration survived the borrow dance
        assert!(storage.get_tag("e000000").is_some());
        assert!(storage.get_tag(&format!("e{:06}", DEPTH - 1)).is_some());
        drop(storage); // doclist then tagmap; both orders must be stack-safe
    }
}

/// Wide-and-deep comb: every spine level carries 3 leaf children before the
/// next spine element, so each unwrapped element appends 4 entries to the
/// Drop worklist (round-1 probes only exercised single-child chains).
#[test]
fn verify_xml_r2_wide_deep_comb_parse_and_drop() {
    const DEPTH: usize = 30_000;
    let mut v = Vec::new();
    for _ in 0..DEPTH {
        v.extend_from_slice(b"<a><b/><b/><b/>");
    }
    v.push(b'x');
    for _ in 0..DEPTH {
        v.extend_from_slice(b"</a>");
    }
    v.push(b'\n');
    let doc = xml_tree(&v).expect("parse");
    // iterative spot-walk: every spine node has 3 leading b-leaves, and all
    // but the innermost (which holds the "x" content) a 4th spine child
    let mut el: &Element = doc.get_root();
    let mut levels = 1usize;
    loop {
        let kids = el.get_children();
        for leaf in &kids[..3] {
            assert_eq!(leaf.get_name(), "b");
            assert!(leaf.get_children().is_empty());
        }
        if levels == DEPTH {
            assert_eq!(kids.len(), 3, "innermost level");
            break;
        }
        assert_eq!(kids.len(), 4, "level {levels}");
        el = &kids[3];
        levels += 1;
    }
    assert_eq!(el.get_content(), b"x");
    drop(doc); // 120000-element worklist teardown: no overflow
}

/// The repaired module-doc claim at full strength, plus the LOSS-010 pin:
/// depth 200000 parses, walks and drops on a 2 MiB test thread, and the
/// first depth the C++ oracle rejects ("memory exhausted", xml_parse RET 2
/// at depth 4998) is accepted by the port — the documented divergence.
#[test]
fn verify_xml_r2_depth_200k_and_loss010_boundary() {
    let doc = xml_tree(&nested_doc(200_000)).expect("depth 200000 parses");
    let (levels, content) = walk_chain(doc.get_root());
    assert_eq!(levels, 200_000);
    assert_eq!(content, b"x");
    drop(doc);

    // LOSS-010: C++ oracle rejects depth 4998; the Rust port accepts it.
    let doc = xml_tree(&nested_doc(4_998)).expect("LOSS-010: depth 4998 accepted");
    let (levels, _) = walk_chain(doc.get_root());
    assert_eq!(levels, 4_998);
}

/// Error-path teardown: fail the parse AFTER a deep subtree has closed, so
/// the partially built DOM (a 100000-deep closed chain hanging off a still
/// open element) is discarded through the parser's error return — that
/// internal-state drop must be exactly as stack-safe as the success path.
#[test]
fn verify_xml_r2_error_path_discards_deep_partial_dom_safely() {
    const DEPTH: usize = 100_000;
    let mut v = Vec::new();
    for _ in 0..DEPTH {
        v.extend_from_slice(b"<a>");
    }
    v.push(b'x');
    for _ in 0..DEPTH - 1 {
        v.extend_from_slice(b"</a>"); // outermost <a> never closes -> error
    }
    let err = xml_tree(&v).expect_err("truncated document must fail");
    assert_eq!(err.explain(), "syntax error"); // bison YYERROR_VERBOSE=0 text
}
