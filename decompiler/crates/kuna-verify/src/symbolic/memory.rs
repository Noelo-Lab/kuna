//! Finite symbolic byte-addressed memory with explicit access coverage.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::concolic::{self, Expr};

use super::terms::{self, Term};

#[derive(Clone, Copy)]
pub(super) struct Window {
    pub start: u64,
    pub size: u32,
    pub writable: bool,
    pub observed: bool,
    pub private_stack: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ReadMode {
    Machine,
    IrCurrent,
    IrInitial,
}

#[derive(Clone)]
struct Cell {
    value: Term,
    initialized: Term,
}

impl Cell {
    fn store(&mut self, guard: &Term, value: Term) -> Result<(), String> {
        self.value = choose(guard, value, Rc::clone(&self.value))?;
        self.initialized = disjunction(Rc::clone(guard), Rc::clone(&self.initialized));
        Ok(())
    }
}

#[derive(Clone)]
pub(super) struct Memory {
    windows: Rc<[Window]>,
    initial: Rc<BTreeMap<u64, Cell>>,
    bytes: BTreeMap<u64, Cell>,
    covered: Term,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(Vec::new(), BTreeMap::new())
    }
}

impl Memory {
    pub fn new(windows: Vec<Window>, bytes: BTreeMap<u64, Term>) -> Self {
        let bytes: BTreeMap<_, _> = bytes
            .into_iter()
            .map(|(address, value)| {
                let initialized =
                    truth(!containing(&windows, address).is_some_and(|w| w.private_stack));
                (address, Cell { value, initialized })
            })
            .collect();
        Self {
            windows: Rc::from(windows),
            initial: Rc::new(bytes.clone()),
            bytes,
            covered: truth(true),
        }
    }

    pub fn enabled(&self) -> bool {
        !self.windows.is_empty()
    }

    pub fn coverage(&self) -> Term {
        Rc::clone(&self.covered)
    }

    pub fn needs_coverage_check(&self) -> bool {
        concrete(&self.covered) != Some(1)
    }

    pub fn require(&mut self, condition: Term) {
        self.covered = conjunction(Rc::clone(&self.covered), condition);
    }

    pub fn observed_bytes(&self) -> impl Iterator<Item = (&u64, &Term)> {
        self.bytes
            .iter()
            .filter(|(address, _)| self.window_at(**address).is_some_and(|w| w.observed))
            .map(|(address, cell)| (address, &cell.value))
    }

    pub fn load(
        &mut self,
        address: Term,
        size: u32,
        big_endian: bool,
        active: &Term,
        mode: ReadMode,
    ) -> Result<Term, String> {
        self.access(&address, size, false, active)?;
        let source = if mode == ReadMode::IrInitial {
            self.initial.as_ref()
        } else {
            &self.bytes
        };
        let check_local = mode != ReadMode::Machine && self.windows.iter().any(|w| w.private_stack);
        let mut initialized = truth(true);
        let mut parts = Vec::with_capacity(size as usize);
        for byte in 0..size {
            let at = advance(&address, byte);
            let (value, present) = if let Some(known) = concrete(&at) {
                source.get(&known).map_or_else(
                    || (byte_const(0), truth(false)),
                    |cell| (Rc::clone(&cell.value), Rc::clone(&cell.initialized)),
                )
            } else {
                let mut value = byte_const(0);
                let mut present = truth(false);
                for (&address, item) in source {
                    let selected = equal_const(&at, address);
                    if check_local {
                        present = disjunction(
                            present,
                            conjunction(Rc::clone(&selected), Rc::clone(&item.initialized)),
                        );
                    }
                    value = terms::select(selected, Rc::clone(&item.value), value)?;
                }
                (value, present)
            };
            parts.push(value);
            if check_local {
                initialized = conjunction(initialized, present);
            }
        }
        if check_local {
            let inactive = Rc::new(Expr::Test("=", Rc::clone(active), truth(false)));
            self.require(disjunction(inactive, initialized));
        }
        if !big_endian {
            parts.reverse();
        }
        if size == 1 {
            return parts
                .pop()
                .ok_or_else(|| "symbolic byte load had no value".to_owned());
        }
        Ok(Rc::new(Expr::Concat(parts)))
    }

    pub fn store(
        &mut self,
        address: Term,
        value: &Term,
        big_endian: bool,
        active: &Term,
    ) -> Result<(), String> {
        let bits = value.width();
        if bits % 8 != 0 {
            return Err("symbolic store value is not byte-aligned".to_owned());
        }
        let size = bits / 8;
        let write = self.access(&address, size, true, active)?;
        for offset in 0..size {
            let index = if big_endian {
                size - 1 - offset
            } else {
                offset
            };
            let part = if size == 1 {
                Rc::clone(value)
            } else {
                concolic::byte(value, index)
            };
            let at = advance(&address, offset);
            if let Some(known) = concrete(&at) {
                if self.window_at(known).is_some_and(|w| w.writable) {
                    let entry = self
                        .bytes
                        .get_mut(&known)
                        .ok_or("symbolic memory byte is absent")?;
                    entry.store(&write, part)?;
                }
            } else {
                for (&key, entry) in &mut self.bytes {
                    if containing(&self.windows, key).is_some_and(|w| w.writable) {
                        let selected = conjunction(Rc::clone(&write), equal_const(&at, key));
                        entry.store(&selected, Rc::clone(&part))?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn join(self, other: Self, guard: &Term) -> Result<Self, String> {
        if !Rc::ptr_eq(&self.windows, &other.windows) {
            return Err("symbolic memory branches use different maps".to_owned());
        }
        let mut bytes = self.bytes;
        for (address, right) in other.bytes {
            let left = bytes
                .get_mut(&address)
                .ok_or("symbolic memory join lacks a byte")?;
            left.value = choose(guard, Rc::clone(&left.value), right.value)?;
            left.initialized = choose(guard, Rc::clone(&left.initialized), right.initialized)?;
        }
        let covered = choose(guard, self.covered, other.covered)?;
        Ok(Self {
            windows: self.windows,
            initial: self.initial,
            bytes,
            covered,
        })
    }

    fn access(
        &mut self,
        address: &Term,
        size: u32,
        write: bool,
        active: &Term,
    ) -> Result<Term, String> {
        if !self.enabled() {
            return Err("no explicit symbolic memory map".into());
        }
        if address.width() != 64 || !(1..=16).contains(&size) {
            return Err("symbolic memory requires a 64-bit address and 1..16 byte access".into());
        }
        let mut allowed = truth(false);
        for window in self
            .windows
            .iter()
            .filter(|w| w.size >= size && (!write || w.writable))
        {
            let last = window.start + u64::from(window.size - size);
            let in_window = match concrete(address) {
                Some(at) => truth((window.start..=last).contains(&at)),
                None => {
                    let lo = Rc::new(Expr::Test(
                        "bvule",
                        address_const(window.start),
                        Rc::clone(address),
                    ));
                    let hi = Rc::new(Expr::Test("bvule", Rc::clone(address), address_const(last)));
                    conjunction(lo, hi)
                }
            };
            allowed = disjunction(allowed, in_window);
        }
        let inactive = Rc::new(Expr::Test("=", Rc::clone(active), truth(false)));
        self.covered = conjunction(
            Rc::clone(&self.covered),
            disjunction(inactive, Rc::clone(&allowed)),
        );
        Ok(conjunction(Rc::clone(active), allowed))
    }

    fn window_at(&self, address: u64) -> Option<&Window> {
        containing(&self.windows, address)
    }
}

fn containing(windows: &[Window], address: u64) -> Option<&Window> {
    windows
        .iter()
        .find(|w| address >= w.start && address - w.start < u64::from(w.size))
}

fn choose(guard: &Term, yes: Term, no: Term) -> Result<Term, String> {
    match concrete(guard) {
        Some(0) => Ok(no),
        Some(_) => Ok(yes),
        None if Rc::ptr_eq(&yes, &no) => Ok(yes),
        None => terms::select(Rc::clone(guard), yes, no),
    }
}

fn conjunction(left: Term, right: Term) -> Term {
    match (concrete(&left), concrete(&right)) {
        (Some(0), _) | (_, Some(0)) => truth(false),
        (Some(1), _) => right,
        (_, Some(1)) => left,
        _ => Rc::new(Expr::Op("bvand", vec![left, right], 8)),
    }
}

fn disjunction(left: Term, right: Term) -> Term {
    match (concrete(&left), concrete(&right)) {
        (Some(1), _) | (_, Some(1)) => truth(true),
        (Some(0), _) => right,
        (_, Some(0)) => left,
        _ => Rc::new(Expr::Op("bvor", vec![left, right], 8)),
    }
}

fn advance(address: &Term, offset: u32) -> Term {
    if offset == 0 {
        return Rc::clone(address);
    }
    match concrete(address) {
        Some(base) => address_const(base.wrapping_add(u64::from(offset))),
        None => Rc::new(Expr::Op(
            "bvadd",
            vec![Rc::clone(address), address_const(u64::from(offset))],
            64,
        )),
    }
}

fn concrete(value: &Term) -> Option<u64> {
    match **value {
        Expr::Const(v, _) => u64::try_from(v).ok(),
        _ => None,
    }
}

fn equal_const(address: &Term, expected: u64) -> Term {
    Rc::new(Expr::Test("=", Rc::clone(address), address_const(expected)))
}

fn truth(value: bool) -> Term {
    byte_const(u8::from(value))
}

fn byte_const(value: u8) -> Term {
    Rc::new(Expr::Const(u128::from(value), 8))
}

fn address_const(value: u64) -> Term {
    Rc::new(Expr::Const(u128::from(value), 64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concolic::Emitter;
    use crate::symbolic::obligation::{Obligation, Verdict};
    use std::process::Command;
    use std::time::{Duration, Instant};

    #[test]
    fn whole_width_access_respects_endian_and_private_initialization() {
        if Command::new("z3").arg("-version").output().is_err() {
            return;
        }
        let window = Window {
            start: 0x1000,
            size: 2,
            writable: true,
            observed: false,
            private_stack: true,
        };
        let mut memory = Memory::new(
            vec![window],
            BTreeMap::from([(0x1000, byte_const(0)), (0x1001, byte_const(0))]),
        );
        let untouched = memory.clone();
        let address = address_const(0x1000);
        let active = truth(true);
        let x = Rc::new(Expr::Const(0x1234, 16));
        memory
            .store(Rc::clone(&address), &x, true, &active)
            .unwrap();
        let big = memory
            .load(Rc::clone(&address), 2, true, &active, ReadMode::IrCurrent)
            .unwrap();
        let little = memory
            .load(Rc::clone(&address), 2, false, &active, ReadMode::IrCurrent)
            .unwrap();
        let mut undef = untouched;
        undef
            .load(address, 2, true, &active, ReadMode::IrCurrent)
            .unwrap();
        let mut emitter = Emitter::default();
        let (big, little) = (emitter.name(&big), emitter.name(&little));
        let defined = emitter.name(&memory.coverage());
        let undefined = emitter.name(&undef.coverage());
        let proof = Obligation {
            domain: format!("(set-logic QF_BV)\n{}", emitter.definitions()),
            mismatch: format!("(or (distinct {big} #x1234) (distinct {little} #x3412) (distinct {defined} #x01) (distinct {undefined} #x00))"),
            coverage_gap: None,
            completion_gap: None,
            input_bytes: Vec::new(),
        };
        let mut z3 = Command::new("z3");
        z3.args(["-in", "-smt2"]);
        assert_eq!(
            proof.solve(&mut z3, Instant::now() + Duration::from_secs(5)),
            Verdict::Equivalent
        );
    }
}
