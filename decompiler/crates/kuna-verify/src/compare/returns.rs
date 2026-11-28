//! Return observations shared by the main check and replay comparisons.

use kuna_console::engine::ConsoleProgram;

use crate::abi::{self, Claim};
use crate::contract::{CKind, Contract};
use crate::divergence::{self, ReturnMismatch};
use crate::state::State;
use crate::storage::{hex, Storage};
use crate::value::{mask, Halt};

/// The whole of x87's 80-bit `ST0` when `out` is it, where a `float` or
/// `double` return lives in the 80-bit format: the machine's value is rounded
/// to the claimed size before it is compared.
fn x87_register(prog: &ConsoleProgram, out: &Storage) -> Option<Storage> {
    if !(out.size == 4 || out.size == 8) || !prog.arch().archid.starts_with("x86") {
        return None;
    }
    let [piece] = out.pieces(prog).try_into().ok()?;
    let st0 = prog.arch().translate().get_register_data("ST0").ok()?;
    let space = st0.space.as_ref()?;
    (piece.space.get_index() == space.get_index() && piece.off == st0.offset && st0.size == 10)
        .then(|| Storage::new(space, st0.offset, st0.size))
}

pub(super) struct ReturnValue<'a> {
    logical: &'a Storage,
    x87: Option<Storage>,
    float: bool,
}

impl<'a> ReturnValue<'a> {
    pub(super) fn new(prog: &ConsoleProgram, logical: &'a Storage, float: bool) -> Self {
        Self { logical, x87: float.then(|| x87_register(prog, logical)).flatten(), float }
    }

    pub(super) fn physical(&self) -> &Storage {
        self.x87.as_ref().unwrap_or(self.logical)
    }

    pub(super) fn normalize(&self, raw: u128) -> u128 {
        if self.x87.is_some() { crate::x87::to_ieee(raw, self.logical.size) } else { raw & mask(self.logical.size) }
    }

    pub(super) fn agrees(&self, a: u128, b: u128) -> bool {
        a == b || self.float && divergence::nan_quieting(self.logical.size, a, b)
    }

    fn read(&self, machine: &mut State) -> Result<u128, Halt> {
        let s = self.physical();
        machine.read_storage(&s.space, s.off, s.size).map(|v| self.normalize(v))
    }
}

/// One run's comparison: the report lines when the two sides differ.
pub(super) struct Compare<'a> {
    pub(super) prog: &'a ConsoleProgram,
    pub(super) claim: &'a Claim,
    pub(super) contract: Option<(&'a Contract, &'a Storage)>,
}

pub(super) enum Outcome {
    Same,
    Differs(Vec<String>),
    Declared(&'static str),
}

impl Compare<'_> {
    pub(super) fn run(&self, ms: &mut State, ret: Option<u128>) -> Result<Outcome, Halt> {
        let mut lines = Vec::new();
        if let Some(out) = &self.claim.storage {
            let k = ret.ok_or_else(|| Halt::Harness("RETURN without the prototype's value".into()))?;
            let value = ReturnValue::new(self.prog, out, self.claim.float);
            let mv = value.read(ms)?;
            let k = k & mask(out.size);
            if mv != k {
                if value.agrees(mv, k) {
                    return Ok(Outcome::Declared(divergence::NAN_QUIETING.id));
                }
                return Ok(Outcome::Differs(vec![
                    format!("machine  {} = {}", out.name(self.prog), hex(mv, out.size)),
                    format!("kuna     {} = {}", self.claim.text, hex(k, out.size)),
                ]));
            }
        }
        let Some((c, cs)) = self.contract else { return Ok(Outcome::Same) };
        let mv = ms.read_storage(&cs.space, cs.off, cs.size)?;
        let kuna = match (&self.claim.storage, ret) {
            (Some(out), Some(k)) => {
                let value = if c.ret.kind == CKind::Boolean {
                    let value = k & mask(out.size);
                    let nonzero = if self.claim.float {
                        match out.size {
                            4 => f32::from_bits(value as u32) != 0.0,
                            8 => f64::from_bits(value as u64) != 0.0,
                            _ => return Err(Halt::Unsupported(format!("{}-byte float conversion to _Bool", out.size))),
                        }
                    } else {
                        value != 0
                    };
                    u128::from(nonzero)
                } else {
                    abi::convert(k, out.size, self.claim.signed, cs.size)
                };
                Some((out.size, value))
            }
            _ => None,
        };
        if kuna.is_some_and(|(_, kv)| kv == mv) {
            return Ok(Outcome::Same);
        }
        if let Some((ks, kv)) = kuna {
            let m = ReturnMismatch { arch: &self.prog.arch().archid, contract_size: cs.size, kuna_size: ks, machine: mv, kuna: kv };
            if let Some(d) = divergence::find(&m) {
                return Ok(Outcome::Declared(d.id));
            }
        }
        lines.push(format!("contract {}  ({})", c.declaration, c.source));
        lines.push(format!("machine  {} = {}", cs.name(self.prog), hex(mv, cs.size)));
        match (&self.claim.storage, ret, kuna) {
            (Some(out), Some(k), Some((_, kv))) => lines.push(format!(
                "kuna     {} = {}; as {} {}",
                self.claim.text,
                hex(k, out.size),
                c.ret.spelling,
                hex(kv, cs.size)
            )),
            _ => lines.push(format!("kuna     {} returns no value", self.claim.text)),
        }
        Ok(Outcome::Differs(lines))
    }

    pub(super) fn machine_return_diff(&self, a: &mut State, b: &mut State) -> Result<Option<String>, Halt> {
        let abi = self.claim.storage.as_ref().map(|out| ReturnValue::new(self.prog, out, self.claim.float));
        let contract = self.contract.map(|(_, out)| ReturnValue::new(self.prog, out, false));
        for value in abi.into_iter().chain(contract) {
            let (x, y) = (value.read(a)?, value.read(b)?);
            if !value.agrees(x, y) {
                let size = value.logical.size;
                return Ok(Some(format!("return is {} instead of {}", hex(y, size), hex(x, size))));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x87_return_uses_the_claimed_precision_on_replay() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let specs = vec![root.join("specs").to_string_lossy().into_owned()];
        let path = root.join("tests/datatests/retstruct.xml");
        let (prog, _) = crate::session::load_datatest(&path.to_string_lossy(), &specs).unwrap();
        let env = crate::state::Env::new(&prog, 0).unwrap();
        let st0 = prog.arch().translate().get_register_data("ST0").unwrap();
        let claim = Claim {
            storage: Some(Storage::new(st0.space.as_ref().unwrap(), st0.offset, 4)),
            signed: false,
            float: true,
            text: "float f()".into(),
            params: Vec::new(),
        };
        let compare = Compare { prog: &prog, claim: &claim, contract: None };
        let mut a = State::new(&env, 0);
        let mut b = State::new(&env, 0);
        let space = st0.space.as_ref().unwrap();
        let one = (0x3fffu128 << 64) | (1 << 63);
        a.write_storage(space, st0.offset, 10, one).unwrap();
        b.write_storage(space, st0.offset, 10, one + 1).unwrap();
        assert!(matches!(compare.run(&mut a, Some(u128::from(1.0f32.to_bits()))), Ok(Outcome::Same)));
        assert_eq!(compare.machine_return_diff(&mut a, &mut b).unwrap(), None);
        b.write_storage(space, st0.offset, 10, one + (1u128 << 64)).unwrap();
        assert_eq!(compare.machine_return_diff(&mut a, &mut b).unwrap().as_deref(),
            Some("return is 0x40000000 instead of 0x3f800000"));
    }
}
