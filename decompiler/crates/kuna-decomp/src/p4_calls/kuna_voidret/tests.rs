//! Tests for the `voidret` ledger.

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

use super::*;

struct NoImage;
impl LoadImage for NoImage {
    fn get_file_name(&self) -> &str {
        "none"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("none"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn arch() -> Architecture {
    Architecture::new("test:LE:32", Sleigh::new(Box::new(NoImage), Box::new(ContextInternal::new())))
}

#[test]
fn a_displaced_float_return_is_withdrawn_once_its_forced_decompile_is_discarded() {
    let mut arch = arch();
    let (put, main) = ((1, 0x1270), (1, 0x1080));
    arch.kuna_voidret.returns.insert(put, Returns::Float);
    arch.kuna_voidret.float_refused.entry(put).or_default().insert(main);
    arch.kuna_voidret.displacing.insert(put);
    assert!(withdrawals(&mut arch).is_empty(), "withdrawn before its forced decompile ran");
    restore(&mut arch, put, Some(Returns::Float), None);
    assert!(!arch.kuna_voidret.displacing.contains(&put));
    assert_eq!(withdrawals(&mut arch), BTreeSet::from([put]));
}
