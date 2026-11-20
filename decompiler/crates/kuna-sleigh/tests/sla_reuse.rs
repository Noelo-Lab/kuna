//! With reuse on, a translator built from a `.sla` this thread already decoded
//! is a copy of a kept template. The copy must equal a fresh decode and share no address
//! space with the template or with any other copy: every space a fresh decode
//! points at, the copy points at in its own space set, as many times.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::slaformat::FormatEncode;
use kuna_sleigh::sleigh::Sleigh;

struct NoImage;

impl LoadImage for NoImage {
    fn get_file_name(&self) -> &str {
        "none"
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

fn languages() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs/Ghidra/Processors")
}

fn build(sla: &[u8]) -> Sleigh {
    let mut sleigh = Sleigh::new(Box::new(NoImage), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(sla).expect("decode .sla");
    sleigh
}

fn encoded(sleigh: &Sleigh) -> Vec<u8> {
    let mut out = Vec::new();
    let mut encoder = FormatEncode::new(&mut out, -1);
    sleigh.base().encode(&mut encoder.packed()).expect("encode");
    encoder.flush().expect("flush");
    out
}

fn space_counts(sleigh: &Sleigh) -> Vec<(i32, usize)> {
    let manager = sleigh.base().manager();
    (0..manager.num_spaces())
        .filter_map(|i| manager.get_space(i).map(|spc| (i, Rc::strong_count(spc))))
        .collect()
}

fn registers(sleigh: &Sleigh) -> Vec<(i32, u64, u32, Vec<u8>)> {
    sleigh
        .base()
        .get_all_registers()
        .iter()
        .map(|(k, name)| {
            (k.space.as_ref().map_or(-1, |s| s.get_index()), k.offset, k.size, name.clone())
        })
        .collect()
}

#[test]
fn reused_sla_matches_a_fresh_decode() {
    kuna_sleigh::kuna_slacache::enable();
    for rel in [
        "x86/data/languages/x86-64.sla",
        "AARCH64/data/languages/AARCH64.sla",
        "ARM/data/languages/ARM8_le.sla",
        "MIPS/data/languages/mips32be.sla",
        "Atmel/data/languages/avr8.sla",
        "68000/data/languages/68040.sla",
    ] {
        let path = languages().join(rel);
        let sla = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let fresh = build(&sla);
        let first_copy = build(&sla);
        let second_copy = build(&sla);
        let want = encoded(&fresh);
        for copy in [&first_copy, &second_copy] {
            assert_eq!(encoded(copy), want, "{rel}: encode differs");
            assert_eq!(space_counts(copy), space_counts(&fresh), "{rel}: space references");
            assert_eq!(registers(copy), registers(&fresh), "{rel}: register xref");
            assert_eq!(copy.base().get_user_op_names(), fresh.base().get_user_op_names());
        }
        let a = first_copy.base().manager();
        let b = second_copy.base().manager();
        for i in 0..a.num_spaces() {
            if let (Some(x), Some(y)) = (a.get_space(i), b.get_space(i)) {
                assert!(!Rc::ptr_eq(x, y), "{rel}: copies share space {i}");
            }
        }
    }
}
