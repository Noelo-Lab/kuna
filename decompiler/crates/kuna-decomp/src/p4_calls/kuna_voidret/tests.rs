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

/// [`due`] looks only at the callees whose verdict inputs changed, and names
/// what a look at every callee names, leaving the same forced storage,
/// withdrawals and displaced returns behind.
#[test]
fn due_over_the_unsettled_callees_agrees_with_a_look_at_every_callee() {
    let reg = std::rc::Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        2,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let at = |off: u64, size: int4| (Address::new(std::rc::Rc::clone(&reg), off), size);
    let (rax, rdx, xmm0) = (0x0, 0x10, 0x1200);
    let (f, g, h, k) = ((1, 0x100), (1, 0x200), (1, 0x300), (1, 0x400));
    let (r1, r2, r3) = ((1, 0x1000), (1, 0x2000), (1, 0x3000));
    let mut incremental = arch();
    let mut everywhere = arch();
    let steps: Vec<(Box<dyn Fn(&mut Architecture) + '_>, BTreeSet<(int4, uintb)>)> = vec![
        (Box::new(|a| {
            restore(a, f, Some(Returns::Void), None);
            file_claim(a, r1, f, at(rax, 8));
        }), BTreeSet::from([f])),
        (Box::new(|_| {}), BTreeSet::new()),
        (Box::new(|a| file_claim(a, r2, f, at(rax, 4))), BTreeSet::new()),
        (Box::new(|a| {
            restore(a, g, Some(Returns::Void), None);
            file_claim(a, r1, g, at(rax, 4));
        }), BTreeSet::from([g])),
        (Box::new(|a| file_claim(a, r3, g, at(rax, 8))), BTreeSet::from([g])),
        (Box::new(|a| {
            restore(a, h, Some(Returns::Void), None);
            file_claim(a, r1, h, at(rax, 8));
            file_claim(a, r2, h, at(rdx, 8));
        }), BTreeSet::new()),
        (Box::new(|a| file_claim(a, r3, f, at(rdx, 8))), BTreeSet::from([f])),
        (Box::new(|a| {
            restore(a, k, Some(Returns::Float), Some(at(xmm0, 8)));
            file_claim(a, r1, k, at(rax, 8));
        }), BTreeSet::from([k])),
        (Box::new(|a| restore(a, f, Some(Returns::Other), Some(at(rax, 8)))), BTreeSet::new()),
        (Box::new(|_| {}), BTreeSet::new()),
    ];
    for (i, (step, expected)) in steps.iter().enumerate() {
        step(&mut incremental);
        step(&mut everywhere);
        let callees: Vec<(int4, uintb)> = everywhere.kuna_voidret.read.keys().copied().collect();
        everywhere.kuna_voidret.unsettled.extend(callees);
        let named = due(&mut incremental);
        assert_eq!(named, due(&mut everywhere), "step {i}");
        assert_eq!(&named, expected, "step {i}");
        let (a, b) = (&incremental.kuna_voidret, &everywhere.kuna_voidret);
        assert_eq!(format!("{:?}", a.forced), format!("{:?}", b.forced), "step {i}");
        assert_eq!(a.withdrawn, b.withdrawn, "step {i}");
        assert_eq!(a.displacing, b.displacing, "step {i}");
    }
}
