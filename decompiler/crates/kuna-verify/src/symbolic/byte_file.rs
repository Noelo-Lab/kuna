//! Little-endian register or temporary storage for the raw symbolic engine.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::concolic::{self, Expr};

use super::terms::{self, Term};

#[derive(Clone, Default)]
pub(super) struct ByteFile {
    bytes: BTreeMap<u64, Term>,
}

impl ByteFile {
    pub fn merge(self, other: Self, guard: &Term) -> Result<Self, String> {
        let mut bytes = BTreeMap::new();
        for (address, left) in self.bytes {
            if let Some(right) = other.bytes.get(&address) {
                let value = if Rc::ptr_eq(&left, right) {
                    left
                } else {
                    terms::select(Rc::clone(guard), left, Rc::clone(right))?
                };
                bytes.insert(address, value);
            }
        }
        Ok(Self { bytes })
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    pub fn read(&self, offset: u64, size: u32) -> Result<Term, String> {
        let end = end(offset, size)?;
        let mut parts = (offset..end)
            .rev()
            .map(|addr| {
                self.bytes
                    .get(&addr)
                    .cloned()
                    .ok_or_else(|| format!("uninitialized symbolic storage byte {addr:#x}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() == 1 {
            return Ok(parts.pop().expect("one byte was read"));
        }
        Ok(Rc::new(Expr::Concat(parts)))
    }

    pub fn write(&mut self, offset: u64, size: u32, value: &Term) -> Result<(), String> {
        let end = end(offset, size)?;
        if value.width() != size * 8 {
            return Err("symbolic storage write width does not match its value".to_owned());
        }
        for (index, addr) in (offset..end).enumerate() {
            let part = if size == 1 {
                Rc::clone(value)
            } else {
                concolic::byte(value, index as u32)
            };
            self.bytes.insert(addr, part);
        }
        Ok(())
    }
}

fn end(offset: u64, size: u32) -> Result<u64, String> {
    if !(1..=16).contains(&size) {
        return Err(format!("invalid symbolic storage width of {size} bytes"));
    }
    offset
        .checked_add(u64::from(size))
        .ok_or_else(|| "symbolic storage extent overflows".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_writes_replace_only_their_overlapping_storage() {
        let mut file = ByteFile::default();
        let word = Rc::new(Expr::Var(Rc::from("old")));
        let high = Rc::new(Expr::Var(Rc::from("high")));
        file.write(16, 1, &word).unwrap();
        file.write(17, 1, &high).unwrap();
        let newer = Rc::new(Expr::Var(Rc::from("replacement")));
        file.write(16, 1, &newer).unwrap();
        let joined = file.read(16, 2).unwrap();
        let Expr::Concat(parts) = &*joined else {
            panic!("expected a two-byte join")
        };
        assert!(Rc::ptr_eq(&parts[0], &high));
        assert!(Rc::ptr_eq(&parts[1], &newer));
        assert!(file.read(15, 2).is_err());
        assert!(file.write(u64::MAX, 1, &newer).is_err());
        assert!(file.write(20, 8, &newer).is_err());
        file.clear();
        assert!(file.read(16, 2).is_err());
    }
}
