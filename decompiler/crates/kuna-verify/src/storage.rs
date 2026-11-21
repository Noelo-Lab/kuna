//! Storage locations that varnodes and prototypes name, and how to print them.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_console::engine::ConsoleProgram;

#[derive(Clone, Debug)]
pub struct Storage {
    pub space: Rc<AddrSpace>,
    pub off: u64,
    pub size: u32,
}

impl Storage {
    pub fn new(space: &Rc<AddrSpace>, off: u64, size: u32) -> Self {
        Storage { space: Rc::clone(space), off, size }
    }

    pub fn at(addr: &Address, size: u32) -> Option<Self> {
        addr.get_space().map(|s| Storage::new(s, addr.get_offset(), size))
    }

    pub fn overlaps(&self, o: &Storage) -> bool {
        self.space.get_index() == o.space.get_index()
            && self.off < o.off.wrapping_add(u64::from(o.size))
            && o.off < self.off.wrapping_add(u64::from(self.size))
    }

    pub fn covers(&self, inner: &Storage) -> bool {
        self.space.get_index() == inner.space.get_index()
            && inner.off >= self.off
            && inner.off + u64::from(inner.size) <= self.off + u64::from(self.size)
    }

    /// The physical pieces of a joined location, most significant first; the
    /// location itself otherwise.
    pub fn pieces(&self, prog: &ConsoleProgram) -> Vec<Storage> {
        if self.space.get_type() != spacetype::IPTR_JOIN {
            return vec![self.clone()];
        }
        match prog.arch().manage().find_join(self.off) {
            Ok(rec) => (0..rec.num_pieces())
                .filter_map(|i| {
                    let p = rec.get_piece(i);
                    p.space.as_ref().map(|s| Storage::new(s, p.offset, p.size))
                })
                .collect(),
            Err(_) => vec![self.clone()],
        }
    }

    pub fn name(&self, prog: &ConsoleProgram) -> String {
        let parts: Vec<String> = self
            .pieces(prog)
            .iter()
            .map(|p| {
                if p.space.get_type() == spacetype::IPTR_SPACEBASE {
                    return format!("stack[{:#x}]", p.off);
                }
                let name = prog
                    .arch()
                    .translate()
                    .as_sleigh()
                    .map(|s| s.base().get_register_name(&p.space, p.off, p.size as i32))
                    .unwrap_or_default();
                if name.is_empty() {
                    format!("{}:{:#x}", p.space.get_name(), p.off)
                } else {
                    String::from_utf8_lossy(&name).into_owned()
                }
            })
            .collect();
        parts.join(":")
    }
}

pub fn hex(v: u128, size: u32) -> String {
    format!("0x{:0width$x}", v, width = (size.max(1) * 2) as usize)
}
