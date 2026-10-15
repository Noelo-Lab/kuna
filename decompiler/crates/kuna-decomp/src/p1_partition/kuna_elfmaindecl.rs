//! (kuna `elfmain`) The C runtime's declaration for an executable's own `main`,
//! applied as far as `main`'s own analysis recovers its parameters.
//!
//! An executable that keeps its symbol table names `main` itself, but the
//! declaration comes from its body alone: the `elfmainnamed_x86_64` fixture's
//! `main` prints `unsigned int main(int a0,struct_0 *a1)`, with
//! `score(a1->field_0x8)` where the source says `score(argv[1])`. The runtime's
//! `int main(int argc, char **argv, char **envp)` may only name and type what
//! the analysis finds, never add or drop a parameter: a declared parameter is a
//! locked input, which the analysis hands to the next call even where the body
//! never reads it, and a parameter left out turns the body's reads of it into
//! unassigned locals.
//!
//! So no input is locked. The drive locks only the `int` return and seeds the
//! function with the runtime's [`Runtime`]: the storage, name and type the
//! calling convention gives `argc`, `argv` and `envp`. Parameter recovery runs
//! as it always does; [`read_slots`] then asks whether the inputs the body
//! reads in parameter storage are exactly the first k of those storages, and
//! only then does type inference give each its runtime type ([`input_type`],
//! the same unlocked seam `calleevote` votes through) and the recovered
//! parameter its runtime name ([`param_name`]). A `main` that reads `argv` but
//! not `argc`, or reads the whole 64-bit register `argc` is the low half of,
//! is left as the analysis finds it. [`settled`] checks the finished function:
//! the parameters it declares must be exactly what was typed and named, or the
//! drive decompiles it again without the seed.

use std::cell::Cell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::context::VarnodeId;
use crate::dtype::Datatype;
use crate::fspec::{PrototypePieces, ProtoModel};
use crate::funcdata::Funcdata;

/// One runtime argument: where the calling convention puts it, its name and its
/// type.
#[derive(Debug, Clone)]
pub struct Slot {
    pub addr: Address,
    pub size: int4,
    pub name: String,
    pub ct: Rc<Datatype>,
}

/// The runtime's `main` declaration, laid out by the calling convention.
#[derive(Debug, Clone)]
pub struct Runtime {
    /// `argc`, `argv`, `envp`, in order.
    pub slots: Vec<Slot>,
    /// Every storage range the convention can pass a parameter in.
    pub entries: Vec<(Address, int4)>,
    /// The `int` return alone, applied as a locked output.
    pub output: PrototypePieces,
    /// Set once [`input_type`] has typed an input.
    typed: Cell<bool>,
}

impl Runtime {
    /// Lay `decl` out under `model`, or `None` when the convention cannot.
    pub fn new(
        decl: &PrototypePieces,
        model: &Rc<ProtoModel>,
        types: &dyn crate::dtype::TypeFactory,
        manager: &kuna_base::space::AddrSpaceManager,
    ) -> Option<Runtime> {
        let mut res = Vec::new();
        model.assign_parameter_storage(decl, &mut res, true, types, manager).ok()?;
        if res.len() != decl.intypes.len() + 1 {
            return None;
        }
        let slots = res
            .iter()
            .skip(1)
            .zip(decl.intypes.iter().zip(&decl.innames))
            .map(|(p, (ct, name))| Slot {
                addr: p.addr.clone(),
                size: ct.get_size(),
                name: name.clone(),
                ct: Rc::clone(ct),
            })
            .collect();
        let entries = model
            .input()
            .get_entry()
            .iter()
            .map(|e| (Address::new(Rc::clone(e.get_space()), e.get_base()), e.get_size()))
            .collect();
        let mut output = decl.clone();
        output.intypes.clear();
        output.innames.clear();
        output.input_storage.clear();
        Some(Runtime { slots, entries, output, typed: Cell::new(false) })
    }

    /// A copy for one drive, with nothing typed yet.
    pub fn fresh(&self) -> Rc<Runtime> {
        Rc::new(Runtime { typed: Cell::new(false), ..self.clone() })
    }
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    a.overlap(0, b, bsize) >= 0 || b.overlap(0, a, asize) >= 0
}

/// How many runtime arguments the body reads, when the inputs it reads in
/// parameter storage are exactly the first k runtime slots; `None` otherwise
/// (an unread slot before a read one, a read of a wider or narrower register
/// than the slot, a read past the third).
pub fn read_slots(data: &Funcdata) -> Option<usize> {
    let rt = data.kuna_elfmain()?;
    let mut read: Vec<(Address, int4)> = Vec::new();
    for id in data.vbank().iter_def_flag(crate::varnode::varnode_flags::input) {
        let Some(v) = data.vbank().get(id) else { continue };
        if v.is_input()
            && !v.has_no_descend()
            && rt.entries.iter().any(|(ea, es)| overlaps(v.get_addr(), v.get_size(), ea, *es))
        {
            read.push((v.get_addr().clone(), v.get_size()));
        }
    }
    if read.len() > rt.slots.len() {
        return None;
    }
    rt.slots[..read.len()]
        .iter()
        .all(|s| read.iter().any(|(a, size)| a == &s.addr && *size == s.size))
        .then_some(read.len())
}

fn slot_of<'a>(data: &'a Funcdata, addr: &Address, size: int4) -> Option<&'a Slot> {
    let rt = data.kuna_elfmain()?;
    let k = read_slots(data)?;
    rt.slots[..k].iter().find(|s| &s.addr == addr && s.size == size)
}

/// The runtime type of the function input `vn`, when it is one of the runtime
/// arguments the body reads.
pub fn input_type(data: &Funcdata, vn: VarnodeId) -> Option<Rc<Datatype>> {
    let v = data.vbank().get(vn)?;
    if !v.is_input() {
        return None;
    }
    let slot = slot_of(data, v.get_addr(), v.get_size())?;
    data.kuna_elfmain()?.typed.set(true);
    Some(Rc::clone(&slot.ct))
}

/// Is `ct`, the type the function input `vn` carries, its runtime type? Record
/// synthesis gives way to it as it does to the type every caller passes: an
/// `argv` read at `argv[0]` and `argv[1]` is the vector, not a two-field record.
pub fn took_runtime_type(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    v.is_input()
        && slot_of(data, v.get_addr(), v.get_size()).is_some_and(|s| crate::kuna_calleevote::same_type(&s.ct, ct))
}

/// The runtime name of the recovered parameter in `addr`/`size`.
pub fn param_name(data: &Funcdata, addr: &Address, size: int4) -> Option<String> {
    slot_of(data, addr, size).map(|s| s.name.clone())
}

/// Is the finished function's declaration exactly what the seed typed and named:
/// no parameter where the body reads no runtime argument, or the first k
/// runtime arguments, each named? A `main` whose reads are not such a prefix is
/// `false`: its locked `int` return was the only thing the seed changed, and
/// the drive decompiles it again without the seed, so it prints exactly what it
/// prints with the option off.
pub fn settled(fd: &Funcdata) -> bool {
    let Some(rt) = fd.kuna_elfmain() else { return true };
    let proto = fd.get_func_proto();
    let params: Vec<(Address, int4, String)> = (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .map(|p| (p.get_address(), p.get_size(), p.get_name().to_string()))
        .collect();
    let named = params.iter().any(|(_, _, n)| rt.slots.iter().any(|s| &s.name == n));
    match read_slots(fd) {
        Some(0) => params.is_empty() && !rt.typed.get() && !named,
        Some(k) => {
            params.len() == k
                && params.iter().zip(&rt.slots).all(|((a, size, n), s)| a == &s.addr && *size == s.size && n == &s.name)
        }
        None => false,
    }
}
