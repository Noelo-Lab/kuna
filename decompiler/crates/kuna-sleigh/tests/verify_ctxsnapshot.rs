//! `ContextValueSnapshot` round-trips a context database's read surface.
//!
//! A `ContextDatabase` cannot be cloned faithfully, so a second decoder's
//! database is rebuilt by value: default blob plus every split point, replayed
//! ascending. What must survive is exactly what a decode reads — the blob in
//! force at each address — and that is what these gates pin, on x86-64 (a
//! handful of split points) and on ARM after several `TMode` paints (many).
//!
//! ## `.sla` precondition
//!
//! The `.sla` files are build artifacts (gitignored; `make specs`). When one is
//! absent the test prints that and returns rather than failing green.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::kuna_ctxsnapshot::{restore_context, snapshot_context, ContextValueSnapshot};
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

struct DummyImg;

impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        ptr.fill(0);
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn engine(rel: &str) -> Option<(Sleigh, Rc<AddrSpace>)> {
    let path = repo_root().join("specs/Ghidra/Processors").join(rel);
    let bytes = std::fs::read(&path).ok()?;
    let mut sleigh = Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&bytes).ok()?;
    let space = Rc::clone(sleigh.base().manager().get_default_code_space()?);
    Some((sleigh, space))
}

/// The blob in force at each probe address, as the decoder would read it.
fn probe(sleigh: &Sleigh, space: &Rc<AddrSpace>, at: &[u64]) -> Vec<Vec<u32>> {
    at.iter()
        .map(|&off| {
            sleigh.with_context_db_mut(|db| {
                db.get_context(&Address::new(Rc::clone(space), off)).to_vec()
            })
        })
        .collect()
}

fn round_trip(sleigh: &Sleigh, space: &Rc<AddrSpace>, rel: &str, probes: &[u64]) -> ContextValueSnapshot {
    let snap = sleigh.with_context_db_mut(|db| snapshot_context(db, space));
    let want = probe(sleigh, space, probes);

    let (other, other_space) = engine(rel).expect("second engine");
    other.with_context_db_mut(|db| restore_context(db, &snap, &other_space));

    let again = other.with_context_db_mut(|db| snapshot_context(db, &other_space));
    assert_eq!(snap.default_blob, again.default_blob, "the default blob must survive the copy");
    assert_eq!(snap.points, again.points, "the split points must survive the copy");
    assert_eq!(want, probe(&other, &other_space, probes), "every probed address must read the same");
    snap
}

#[test]
fn x86_64_context_values_survive_a_copy() {
    let rel = "x86/data/languages/x86-64.sla";
    let Some((sleigh, space)) = engine(rel) else {
        eprintln!("verify_ctxsnapshot: skipping (no x86-64.sla; `make specs`)");
        return;
    };

    // A freshly decoded .sla has no splits at all: one region, the default.
    let bare = sleigh.with_context_db_mut(|db| snapshot_context(db, &space));
    assert_eq!(bare.points.len(), 1, "an unpainted database is one region");

    // Two paints, which is what a pspec `<context_data>` amounts to on x86-64.
    let invalid = Address::new_invalid();
    sleigh.with_context_db_mut(|db| {
        db.set_context_region(&Address::new(Rc::clone(&space), 0x1000), &invalid, 0, 0xf, 0x5);
        db.set_context_region(&Address::new(Rc::clone(&space), 0x2000), &invalid, 0, 0xf, 0xa);
    });

    let probes = [0x0, 0xfff, 0x1000, 0x1fff, 0x2000, 0x8000_0000];
    let snap = round_trip(&sleigh, &space, rel, &probes);
    assert_eq!(snap.points.len(), 3, "two paints split the map in three: {:x?}", snap.points);
    assert_eq!(snap.points[1].1[0] & 0xf, 0x5);
    assert_eq!(snap.points[2].1[0] & 0xf, 0xa);
}

#[test]
fn arm_tmode_paints_survive_a_copy() {
    let rel = "ARM/data/languages/ARM7_le.sla";
    let Some((sleigh, space)) = engine(rel) else {
        eprintln!("verify_ctxsnapshot: skipping (no ARM7_le.sla; `make specs`)");
        return;
    };

    // Alternating Thumb/A32 runs, the shape the ARM marker scan paints.
    let invalid = Address::new_invalid();
    let marks: Vec<u64> = (0..12).map(|i| 0x8000 + i * 0x40).collect();
    sleigh.with_context_db_mut(|db| {
        for (i, &off) in marks.iter().enumerate() {
            let value = (i % 2) as u32;
            db.set_variable_region(
                b"TMode",
                &Address::new(Rc::clone(&space), off),
                &invalid,
                value,
            )
            .expect("ARM registers TMode");
        }
    });

    let mut probes: Vec<u64> = vec![0, 0x7fff];
    for &m in &marks {
        probes.push(m);
        probes.push(m + 0x10);
    }
    let snap = round_trip(&sleigh, &space, rel, &probes);
    assert!(
        snap.points.len() >= marks.len(),
        "each paint is its own region: {} points for {} marks",
        snap.points.len(),
        marks.len()
    );
}
