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

/// [`stale_readers`] before [`Owed`]: every reader of every changed callee,
/// looked at on every call.
fn stale_readers_everywhere(
    arch: &Architecture,
    stamp_of: &BTreeMap<(int4, uintb), usize>,
    changed: &BTreeMap<(int4, uintb), usize>,
) -> BTreeSet<(int4, uintb)> {
    let vfp = crate::kuna_armfloatreturn::applies(arch);
    let mut out = BTreeSet::new();
    for (callee, &at) in changed {
        let ledger = &arch.kuna_voidret;
        let float = ledger.returns.get(callee) == Some(&Returns::Float);
        let withdrawn = ledger.withdrawn.contains(callee);
        let widened = ledger.wide.contains_key(callee);
        let states = float || withdrawn || arch.kuna_callret_types.contains_key(callee) || arch.kuna_narrowret.contains_key(callee);
        let scaled = arch.kuna_callret_types.get(callee).is_some_and(|s| {
            s.ct.get_metatype() == crate::dtype::type_metatype::TYPE_PTR
                && s.ct.get_ptr_to().is_some_and(|p| p.get_size() > 1 && p.get_metatype() != crate::dtype::type_metatype::TYPE_VOID)
        });
        if ledger.bits_withdrawn.contains(callee) {
            let callers = ledger.bits_callers.get(callee).into_iter().flatten();
            out.extend(callers.filter(|c| *c != callee && stamp_of.get(*c).is_some_and(|&s| s < at)));
        }
        for reader in ledger.readers.get(callee).into_iter().flatten() {
            if reader == callee || stamp_of.get(reader).is_none_or(|&s| s >= at) {
                continue;
            }
            let waiting = ledger.forced.contains_key(reader) && ledger.returns.get(reader) == Some(&Returns::Void);
            if !states && !waiting && !widened {
                continue;
            }
            let large = ledger.ops.get(reader).is_some_and(|&n| n > crate::kuna_callrettype::AUDIT_MAX_OPS);
            let wrong = ledger.held.get(&(*reader, *callee)).is_some_and(|h| {
                (scaled && !h.pointer && h.arithmetic)
                    || (float && !h.float && !(large && vfp))
                    || (withdrawn && h.float)
                    || (widened && h.signed)
            });
            if !large || wrong {
                out.insert(*reader);
            }
        }
    }
    out
}

/// [`stale_readers`] over what [`Owed`] keeps names what a look at every
/// reader of every changed callee names, as readers are decompiled again,
/// callees change again, new readers are filed and a callee's state moves.
#[test]
fn owed_readers_agree_with_a_look_at_every_reader() {
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
    let rax = (Address::new(std::rc::Rc::clone(&reg), 0), 8);
    let fns: Vec<(int4, uintb)> = (1..=9).map(|i| (1, 0x1000 * i)).collect();
    let (c1, c2, c3, r1, r2, r3, r4, r5, r6) = (fns[0], fns[1], fns[2], fns[3], fns[4], fns[5], fns[6], fns[7], fns[8]);
    let mut arch = arch();
    let mut stamp = 0usize;
    let mut stamp_of = BTreeMap::new();
    let mut changed = BTreeMap::new();
    let mut owed = Owed::default();
    let mut decompile = |stamp_of: &mut BTreeMap<(int4, uintb), usize>, k| {
        stamp += 1;
        stamp_of.insert(k, stamp);
        stamp
    };
    let ledger = &mut arch.kuna_voidret;
    for (callee, readers) in [(c1, vec![r1, r2, r3]), (c2, vec![r2, r4])] {
        ledger.readers.insert(callee, readers.into_iter().collect());
    }
    ledger.bits_callers.insert(c3, BTreeSet::from([r5, r6]));
    ledger.returns.insert(c1, Returns::Float);
    ledger.held.insert((r2, c1), Held { pointer: false, float: true, arithmetic: false, signed: false });
    ledger.ops.insert(r2, crate::kuna_callrettype::AUDIT_MAX_OPS + 1);
    for k in [r1, r2, r3, r4, r5] {
        decompile(&mut stamp_of, k);
    }
    let check = |arch: &Architecture, stamp_of: &BTreeMap<(int4, uintb), usize>, changed: &BTreeMap<(int4, uintb), usize>, owed: &mut Owed, expected: &[(int4, uintb)], step: &str| {
        let everywhere = stale_readers_everywhere(arch, stamp_of, changed);
        assert_eq!(stale_readers(arch, stamp_of, owed), everywhere, "{step}");
        assert_eq!(everywhere, expected.iter().copied().collect::<BTreeSet<_>>(), "{step}");
    };
    let at = decompile(&mut stamp_of, c1);
    changed.insert(c1, at);
    owed.changed(&arch, c1, at, &stamp_of);
    check(&arch, &stamp_of, &changed, &mut owed, &[r1, r3], "c1 changed");
    decompile(&mut stamp_of, r1);
    check(&arch, &stamp_of, &changed, &mut owed, &[r3], "r1 redone");
    arch.kuna_voidret.held.insert((r2, c1), Held { pointer: false, float: false, arithmetic: false, signed: false });
    check(&arch, &stamp_of, &changed, &mut owed, &[r2, r3], "r2 holds it as another type");
    let at = decompile(&mut stamp_of, c2);
    changed.insert(c2, at);
    owed.changed(&arch, c2, at, &stamp_of);
    check(&arch, &stamp_of, &changed, &mut owed, &[r2, r3], "c2 changed, states nothing");
    arch.kuna_voidret.forced.insert(r4, rax.clone());
    arch.kuna_voidret.returns.insert(r4, Returns::Void);
    check(&arch, &stamp_of, &changed, &mut owed, &[r2, r3, r4], "r4 waits on c2");
    arch.kuna_voidret.wide.insert(c2, rax.clone());
    decompile(&mut stamp_of, r3);
    check(&arch, &stamp_of, &changed, &mut owed, &[r2, r4], "c2 widened, r3 redone");
    let at = decompile(&mut stamp_of, c3);
    changed.insert(c3, at);
    owed.changed(&arch, c3, at, &stamp_of);
    check(&arch, &stamp_of, &changed, &mut owed, &[r2, r4], "c3 changed, its float not withdrawn");
    arch.kuna_voidret.bits_withdrawn.insert(c3);
    check(&arch, &stamp_of, &changed, &mut owed, &[r2, r4, r5], "c3 withdrawn");
    for k in [r2, r4, r5] {
        decompile(&mut stamp_of, k);
    }
    arch.kuna_voidret.readers.entry(c1).or_default().insert(r6);
    decompile(&mut stamp_of, r6);
    check(&arch, &stamp_of, &changed, &mut owed, &[], "every reader redone, r6 reads c1 since");
    let at = decompile(&mut stamp_of, c1);
    changed.insert(c1, at);
    owed.changed(&arch, c1, at, &stamp_of);
    check(&arch, &stamp_of, &changed, &mut owed, &[r1, r2, r3, r6], "c1 changed again");
}
