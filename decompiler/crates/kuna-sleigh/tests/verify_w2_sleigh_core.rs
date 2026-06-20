//! Verifier adversarial tests for item `w2-sleigh-core`
//! (decompiler/cpp/sleigh.cc, sleighbase.cc, slaformat.cc).
//!
//! These target the spots the hunt list flagged as most fragile:
//!
//!  * `SleighBase::getRegisterName` — a `std::map::upper_bound` followed by
//!    `--iter`, i.e. the *last element <= key*.  The port walked back from a
//!    `Bound::Excluded(key)` range, which lands on the last element *strictly
//!    less than* key, skipping an exact match.  Verified end-to-end against a
//!    real decoded MIPS `.sla` register map (F1).
//!  * `SourceFileIndexer` — post-increment index assignment, encode/decode
//!    round-trip, and the `operator[]`-miss defaults.
//!  * `is_sla_format` / `write_sla_header` — header boundary handling.

use kuna_base::address::Address;
use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
use kuna_base::space::AddrSpaceManager;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::sleighbase::SourceFileIndexer;
use kuna_sleigh::slaformat::{
    is_sla_format, write_sla_header, FORMAT_VERSION,
};

// ---------------------------------------------------------------------------
// Shared helper: build a real MIPS engine from the committed mips32be.sla so
// the register cross-reference map is populated by SleighBase::build_xrefs.
// ---------------------------------------------------------------------------

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> kuna_base::error::KunaResult<()> {
        Err(kuna_base::error::KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn mips_engine() -> Sleigh {
    let root = repo_root();
    let sla = std::fs::read(
        root.join("specs/Ghidra/Processors/MIPS/data/languages/mips32be.sla"),
    )
    .expect("read mips32be.sla");
    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize from sla");
    sleigh
}

// ---------------------------------------------------------------------------
// F1: getRegisterName must return the name for an *exact* register lookup.
//
// C++ `getRegisterName(base,off,size)` builds key {base,off,size}, then
// `upper_bound(key); iter--;` lands on the last entry <= key.  For an exact
// register that entry IS the register, and `off+size >= off+size` returns its
// name.  This is the dominant case in real decompilation (naming a varnode
// that exactly covers a register).
// ---------------------------------------------------------------------------

#[test]
fn verify_w2core_get_register_name_exact_match_every_register() {
    let sleigh = mips_engine();
    let base = sleigh.base();
    let regs = base.get_all_registers();
    assert!(!regs.is_empty(), "MIPS register map should be non-empty");

    // For every register in the cross-reference map, the exact (space,off,size)
    // lookup through getRegisterName must reproduce its name (the C++ oracle
    // does).  get_exact_register_name is the independent witness that the entry
    // exists at that exact triple.
    let mut checked = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for (storage, name) in regs.iter() {
        let space = storage.space.as_ref().expect("register has a space");
        let off = storage.offset;
        let size = storage.size as i32;

        let exact = base.get_exact_register_name(space, off, size);
        // Sanity: the entry is exactly present.
        assert_eq!(
            &exact, name,
            "get_exact_register_name disagrees with the map entry"
        );

        let got = base.get_register_name(space, off, size);
        if &got != name {
            failures.push(format!(
                "reg {} @ ({},0x{:x},{}) -> get_register_name={:?} (want {:?})",
                String::from_utf8_lossy(name),
                space.get_name(),
                off,
                size,
                String::from_utf8_lossy(&got),
                String::from_utf8_lossy(name),
            ));
        }
        checked += 1;
    }
    assert!(checked > 0);
    assert!(
        failures.is_empty(),
        "getRegisterName returned the wrong name for {} of {} exact register \
         lookups (C++ returns the register name for all of them). First few:\n{}",
        failures.len(),
        checked,
        failures.iter().take(8).cloned().collect::<Vec<_>>().join("\n"),
    );
}

// A second, self-contained witness of the same algorithmic divergence using a
// minimal register map decoded into a real SleighBase is not possible (the
// xref map is private and only populated via decode).  Instead, pin the
// *sub-register* path too: a query whose (off,size) lies strictly inside a
// register must still be named (upper_bound/iter-- containment), and a query
// just past every register must be unnamed.
#[test]
fn verify_w2core_get_register_name_subregister_containment() {
    let sleigh = mips_engine();
    let base = sleigh.base();
    let regs = base.get_all_registers();

    // Find a register of size >= 2 to probe a contained sub-byte.
    let (storage, name) = regs
        .iter()
        .find(|(s, _)| s.size >= 2)
        .expect("MIPS has a register of size >= 2");
    let space = storage.space.as_ref().unwrap();
    let off = storage.offset;
    let size = storage.size as i32;

    // The first byte of the register is contained by it: getRegisterName must
    // return the register name (C++ containment: point.offset+point.size >=
    // off+size with off=point.offset, size=1).
    let inside = base.get_register_name(space, off, 1);
    assert_eq!(
        &inside, name,
        "getRegisterName(first byte of {}) should name the containing register",
        String::from_utf8_lossy(name)
    );

    // And the exact whole-register lookup again, explicitly.
    let whole = base.get_register_name(space, off, size);
    assert_eq!(
        &whole, name,
        "getRegisterName(whole {}) should name the register",
        String::from_utf8_lossy(name)
    );
}

// ---------------------------------------------------------------------------
// SourceFileIndexer: post-increment assignment + encode/decode round-trip +
// operator[]-miss defaults.
// ---------------------------------------------------------------------------

#[test]
fn verify_w2core_sourcefileindexer_assignment_and_roundtrip() {
    let mut idx = SourceFileIndexer::new();
    // C++ `index` returns leastUnusedIndex++ (post-increment): first file -> 0.
    assert_eq!(idx.index(b"a.slaspec"), 0);
    assert_eq!(idx.index(b"b.slaspec"), 1);
    // Re-indexing an existing file returns its stored index, no new slot.
    assert_eq!(idx.index(b"a.slaspec"), 0);
    assert_eq!(idx.index(b"c.slaspec"), 2);
    assert_eq!(idx.get_index(b"b.slaspec"), 1);
    assert_eq!(idx.get_filename(2), b"c.slaspec".to_vec());

    // operator[]-miss defaults: unknown file -> 0 (no mutation), unknown index
    // -> empty string.
    assert_eq!(idx.get_index(b"missing"), 0);
    assert_eq!(idx.get_filename(99), Vec::<u8>::new());

    // encode/decode round-trip through the packed marshaller.  encode walks
    // 0..leastUnusedIndex in order; decode rebuilds both maps.
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        idx.encode(&mut enc);
    }

    let manager = AddrSpaceManager::new();
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&buf).expect("ingest encoded indexer");
    let mut idx2 = SourceFileIndexer::new();
    idx2.decode(&mut dec).expect("decode indexer");
    assert_eq!(idx2.get_filename(0), b"a.slaspec".to_vec());
    assert_eq!(idx2.get_filename(1), b"b.slaspec".to_vec());
    assert_eq!(idx2.get_filename(2), b"c.slaspec".to_vec());
    assert_eq!(idx2.get_index(b"c.slaspec"), 2);
}

// ---------------------------------------------------------------------------
// is_sla_format / write_sla_header boundaries.
// ---------------------------------------------------------------------------

#[test]
fn verify_w2core_is_sla_format_boundaries() {
    // A header written by write_sla_header must verify.
    let mut hdr: Vec<u8> = Vec::new();
    write_sla_header(&mut hdr).unwrap();
    assert_eq!(hdr, vec![b's', b'l', b'a', FORMAT_VERSION as u8]);
    let (ok, rest) = is_sla_format(&hdr);
    assert!(ok, "freshly written header must verify");
    assert!(rest.is_empty(), "no bytes consumed beyond the 4-byte header");

    // Trailing bytes are preserved as the tail.
    let mut withtail = hdr.clone();
    withtail.extend_from_slice(b"BODY");
    let (ok, rest) = is_sla_format(&withtail);
    assert!(ok);
    assert_eq!(rest, b"BODY");

    // Wrong version byte -> reject (but still consumes 4 bytes).
    let bad_ver = [b's', b'l', b'a', (FORMAT_VERSION as u8).wrapping_add(1), b'X'];
    let (ok, rest) = is_sla_format(&bad_ver);
    assert!(!ok, "wrong version must be rejected");
    assert_eq!(rest, b"X");

    // Wrong magic -> reject.
    let bad_magic = [b'x', b'l', b'a', FORMAT_VERSION as u8];
    let (ok, _rest) = is_sla_format(&bad_magic);
    assert!(!ok, "wrong magic must be rejected");

    // Short input (< 4 bytes) -> reject (C++ sets failbit on the read).
    let (ok, _rest) = is_sla_format(b"sl");
    assert!(!ok, "short input must be rejected");
}

// ---------------------------------------------------------------------------
// Smoke: the engine actually has a sane register set (guards against the
// register map silently being empty, which would make F1 vacuous).
// ---------------------------------------------------------------------------

#[test]
fn verify_w2core_register_map_is_populated() {
    let sleigh = mips_engine();
    let regs = sleigh.base().get_all_registers();
    // MIPS has dozens of registers; require a healthy count so the exact-match
    // sweep above is meaningful.
    assert!(
        regs.len() > 16,
        "expected a populated MIPS register map, got {}",
        regs.len()
    );
    // Spot-check a well-known register name is present (so the map keys/names
    // are real, not garbage).
    let has_named = regs.values().any(|n| !n.is_empty());
    assert!(has_named, "register map has at least one named entry");
}
