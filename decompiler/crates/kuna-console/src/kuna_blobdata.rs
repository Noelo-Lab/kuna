//! (kuna `blobs`) Commit the embedded images and archives the `blobs` pass
//! found: each becomes `<kind>_<addr>`, an `unsigned char[len]` data symbol,
//! where nothing else names the address. Every blob's first address is also
//! recorded on the architecture, so a constant equal to it is weighed as a
//! pointer whatever names it.

use std::rc::Rc;

use kuna_analysis::pass::BlobFact;
use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::varnode::varnode_flags;

use crate::engine::ConsoleProgram;
use crate::kuna_dataidentifiers::Names;

/// The address ranges of the blobs, sorted by start.
pub(crate) struct Extents(Vec<(u64, u64)>);

impl Extents {
    pub(crate) fn new(blobs: &[BlobFact]) -> Self {
        let mut ranges: Vec<(u64, u64)> = blobs.iter().map(|b| (b.addr, b.len)).collect();
        ranges.sort_unstable();
        Extents(ranges)
    }

    /// Is `addr` a byte of a blob?
    pub(crate) fn contains(&self, addr: u64) -> bool {
        let after = self.0.partition_point(|&(start, _)| start <= addr);
        after > 0 && addr - self.0[after - 1].0 < self.0[after - 1].1
    }
}

pub(crate) fn commit(
    prog: &mut ConsoleProgram,
    code_space: &Rc<AddrSpace>,
    blobs: &[BlobFact],
    names: &mut Names,
    num_spaces: i32,
) -> KunaResult<()> {
    for blob in blobs {
        let Ok(len) = i32::try_from(blob.len) else { continue };
        let addr = Address::new(Rc::clone(code_space), blob.addr);
        Rc::make_mut(&mut prog.arch_mut().blob_entries).insert(blob.addr);
        let occupied = {
            let arch = prog.arch();
            arch.is_lazy_entry(blob.addr)
                || arch.symboltab.get_global_scope().is_some_and(|global| {
                    arch.symboltab.find_function(global, &addr).is_some()
                        || arch.symboltab.find_container(global, &addr, 1, &Address::new_invalid()).is_some()
                })
        };
        if occupied {
            continue;
        }
        let byte = prog.arch().types().get_base(1, type_metatype::TYPE_UINT)?;
        let array = prog.arch().types().get_type_array(len, byte)?;
        let name = format!("{}_{:x}", blob.kind.as_str(), blob.addr);
        let name = names.name(&name, blob.addr);
        let arch = prog.arch_mut();
        let (scope, base) = arch.symboltab.find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
        let (sid, _) = arch.symboltab.add_symbol_mapped(scope, &base, array, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, varnode_flags::typelock | varnode_flags::namelock);
    }
    Ok(())
}
