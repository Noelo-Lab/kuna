//! (kuna DIV-50) Reconcile a renaming input Varnode against the write-masked
//! input pieces a previous `Heritage::guardInput` left behind.
//!
//! `Heritage::guardInput` (`heritage.cc:1953`) tiles a partially-input range with
//! input Varnodes, marks every piece `writeMask`, and concatenates them into one
//! full-size value.  The pieces are then invisible to `Heritage::collect` (which
//! skips write-masked Varnodes) but remain in the def tree as inputs.  When the
//! simplification pools later fold the concatenating `PIECE` away and a *new* free
//! read of the full range shows up, `Heritage::renameRecurse` finds an empty
//! variable stack, asks `Funcdata::setInputVarnode` for a full-size input, and the
//! request collides with the leftover pieces — upstream Ghidra raises
//! `LowlevelError("Overlapping input varnodes")` and the whole function is lost.
//!
//! The pieces already *are* the value the read wants, so the collision is
//! recoverable: this module rebuilds the full-size input from them with
//! `Funcdata::combineInputVarnodes` (`funcdata_varnode.cc:383`), the upstream
//! primitive for exactly this shape — the pieces are destroyed, their non-`PIECE`
//! readers are repointed at `SUBPIECE`s of the new whole, and renaming continues
//! with a legal input.
//!
//! Only **write-masked** input pieces are combined.  That restriction is what makes
//! the rewrite safe to run in the middle of renaming: a write-masked Varnode is
//! never pushed onto a `VariableStack` (renaming pushes only `activeHeritage`
//! op outputs and the inputs it creates itself, neither of which is write-masked),
//! and each `Heritage::rename` builds a fresh stack, so no stack can be holding a
//! Varnode this rewrite destroys.  Any other overlap still raises the upstream error.

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

/// Create the input Varnode covering `size` bytes at `addr`, reconciling with any
/// write-masked input pieces already covering that storage.
///
/// Returns the upstream `LowlevelError` when the storage overlaps an input
/// Varnode that is not a reconcilable piece. The three `rename_recurse` call
/// sites currently turn this into a bare panic (`re-needs/ssa-renaming-panics-
/// protected`), so the message is built to be read standalone: it names the
/// requested range and, from [`combine_covered_pieces`]'s own scan, the exact
/// shape of the conflicting Varnode(s) — straddling vs. fully-contained,
/// write-masked vs. live — rather than the bare "Overlapping input varnodes"
/// the C++ `LowlevelError` carries.
pub fn new_tiled_input(fd: &mut Funcdata, size: int4, addr: &Address) -> KunaResult<VarnodeId> {
    let candidate = fd.new_varnode(size, addr, None);
    let overlap_err = match fd.set_input_varnode(candidate) {
        Ok(vn) => return Ok(vn),
        Err(e) => e,
    };
    // setInputVarnode raises before mutating, so the candidate is still a free
    // Varnode with no reads.
    let _ = fd.delete_varnode(candidate);
    combine_covered_pieces(fd, size, addr).map_err(|reason| {
        let spacename = addr.get_space().map(|s| s.get_name()).unwrap_or("?");
        let lo = addr.get_offset();
        let hi = lo.wrapping_add(size as u64);
        KunaError::lowlevel(format!(
            "{overlap_err} (requested input {spacename}[{lo:#x},{hi:#x}); {reason})"
        ))
    })
}

/// Fold the write-masked input pieces covering `[addr, addr+size)` into one
/// full-size input Varnode.
///
/// `Err` carries a human-readable reason instead of `None` so
/// [`new_tiled_input`] can fold it into a diagnostic message; every `?`/early
/// return below states the geometry it saw, not just that reconciliation
/// declined.
fn combine_covered_pieces(fd: &mut Funcdata, size: int4, addr: &Address) -> Result<VarnodeId, String> {
    let space = addr
        .get_space()
        .ok_or_else(|| "requested address has no space".to_string())?
        .clone();
    let lo = addr.get_offset();
    let hi = lo
        .checked_add(size as u64)
        .ok_or_else(|| format!("requested range [{lo:#x},+{size:#x}) overflows"))?;

    let mut pieces: Vec<(u64, int4, VarnodeId)> = Vec::new();
    for id in fd.vbank().iter_def_flag(varnode_flags::input) {
        let v = fd
            .vbank()
            .get(id)
            .ok_or_else(|| format!("input varnode {id:?} vanished mid-scan"))?;
        match v.get_addr().get_space() {
            Some(s) if std::rc::Rc::ptr_eq(s, &space) => {}
            _ => continue,
        }
        let voff = v.get_addr().get_offset();
        let vsize = v.get_size();
        let vend = voff
            .checked_add(vsize as u64)
            .ok_or_else(|| format!("existing input [{voff:#x},+{vsize:#x}) overflows"))?;
        if vend <= lo || voff >= hi {
            continue; // disjoint from the requested storage
        }
        if voff < lo || vend > hi {
            return Err(format!(
                "existing input [{voff:#x},{vend:#x}) straddles the request \
                 (write_mask={})",
                v.is_write_mask()
            ));
        }
        if !v.is_write_mask() {
            return Err(format!(
                "existing input [{voff:#x},{vend:#x}) is fully contained in the \
                 request but is a live (non-write-masked) input, not a \
                 guardInput leftover piece"
            ));
        }
        pieces.push((voff, vsize, id));
    }
    if pieces.is_empty() {
        return Err("no existing input Varnode overlaps the requested range".to_string());
    }
    pieces.sort_by_key(|p| p.0);

    // Plan the end-to-end tiling before touching the IR: every byte of the request
    // is either a piece or a hole that gets its own input, as guardInput does.
    let mut plan: Vec<(u64, int4, Option<VarnodeId>)> = Vec::new();
    let mut cur = lo;
    for (voff, vsize, id) in pieces {
        if voff < cur {
            return Err(format!(
                "pieces overlap each other at offset {voff:#x} (previous piece ends at {cur:#x})"
            ));
        }
        if voff > cur {
            plan.push((cur, (voff - cur) as int4, None));
        }
        plan.push((voff, vsize, Some(id)));
        cur = voff + vsize as u64;
    }
    if cur < hi {
        plan.push((cur, (hi - cur) as int4, None));
    }

    let mut tiling: Vec<(u64, int4, VarnodeId)> = Vec::new();
    for (off, sz, existing) in plan {
        let vn = match existing {
            Some(id) => id,
            None => {
                let holeaddr = Address::new(std::rc::Rc::clone(&space), off);
                let hole = fd.new_varnode(sz, &holeaddr, None);
                fd.set_input_varnode(hole)
                    .map_err(|e| format!("setInputVarnode on hole [{off:#x},+{sz:#x}): {e}"))?
            }
        };
        tiling.push((off, sz, vn));
    }

    let big_endian = addr.is_big_endian();
    while tiling.len() > 1 {
        let (aoff, asize, avn) = tiling.remove(0);
        let (_boff, bsize, bvn) = tiling.remove(0);
        // combineInputVarnodes takes (most significant, least significant).
        let (vn_hi, vn_lo) = if big_endian { (avn, bvn) } else { (bvn, avn) };
        fd.combine_input_varnodes(vn_hi, vn_lo)
            .map_err(|e| format!("combineInputVarnodes: {e}"))?;
        let joinaddr = Address::new(std::rc::Rc::clone(&space), aoff);
        let joined = fd
            .find_varnode_input(asize + bsize, &joinaddr)
            .ok_or_else(|| format!("combined input [{aoff:#x},+{:#x}) not found after join", asize + bsize))?;
        tiling.insert(0, (aoff, asize + bsize, joined));
    }
    let (_, joinedsize, joined) = tiling[0];
    if joinedsize != size {
        return Err(format!(
            "tiled result has size {joinedsize:#x}, expected {size:#x}"
        ));
    }
    Ok(joined)
}

// =====================================================================
// VERIFIER (ssa-renaming-panics-protected, docs/re-needs): the two decision
// points this module's overlap scan makes, pinned directly (no full
// heritage/rename walk needed — see the module doc for why "fully-contained
// but live" is a decline and not a silent combine).
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace,
        IopSpace, UniqueSpace,
    };

    use crate::context::ArchContext;

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<kuna_base::space::AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn no_replace(
    ) -> impl FnMut(&mut crate::varnode::VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
        |_, _, _| Ok(())
    }

    /// Make a *live* input varnode directly in the bank (bypassing
    /// `setInputVarnode`'s xref split-borrow, same shortcut `funcdata_varnode.rs`'s
    /// own w10dm tests use).
    fn make_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let r = ram(fd);
        let vn = fd.new_varnode(size, &Address::new(r, off), None);
        fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
    }

    /// A live 1-byte input fully inside the requested 4-byte range (the
    /// AH-vs-EAX / narrow-flag-vs-wide-register shape) is declined, not
    /// silently combined: nothing marks it a `guardInput` leftover, so it may
    /// still be the top of some *other* address key's `VariableStack` this
    /// same renaming round, and combining would destroy it out from under
    /// that stack. The diagnostic must name both the requested range and the
    /// conflicting Varnode's own range so `rename_recurse`'s panic is
    /// actionable without a debugger.
    #[test]
    fn new_tiled_input_declines_fully_contained_live_piece() {
        let mut fd = build_fd();
        let live = make_input(&mut fd, 0x101, 1);
        assert!(fd.vbank().get(live).unwrap().is_input());
        assert!(!fd.vbank().get(live).unwrap().is_write_mask());

        let r = ram(&fd);
        let err = new_tiled_input(&mut fd, 4, &Address::new(r, 0x100))
            .expect_err("a live, non-write-masked contained input must not be combined away");
        let msg = err.to_string();
        assert!(
            msg.contains("ram[0x100,0x104)"),
            "diagnostic names the requested range: {msg}"
        );
        assert!(
            msg.contains("0x101") && msg.contains("0x102"),
            "diagnostic names the conflicting Varnode's own range: {msg}"
        );
        assert!(
            msg.contains("live") || msg.contains("non-write-masked"),
            "diagnostic says *why* it declined (live, not a guardInput piece): {msg}"
        );
        // Declining must not have touched the live varnode.
        assert!(fd.vbank().get(live).unwrap().is_input(), "live input survives the decline");
    }

    /// The shape DIV-50 exists for: every byte of the request is covered by
    /// `guardInput` write-masked leftovers, so reconciliation may run and the
    /// pieces are folded into one full-size input.
    #[test]
    fn new_tiled_input_combines_write_masked_pieces() {
        let mut fd = build_fd();
        let lo = make_input(&mut fd, 0x100, 2);
        let hi = make_input(&mut fd, 0x102, 2);
        fd.vbank_mut().get_mut(lo).unwrap().set_write_mask();
        fd.vbank_mut().get_mut(hi).unwrap().set_write_mask();

        let r = ram(&fd);
        let whole = new_tiled_input(&mut fd, 4, &Address::new(Rc::clone(&r), 0x100))
            .expect("two contiguous write-masked pieces reconcile into one input");
        let wv = fd.vbank().get(whole).unwrap();
        assert!(wv.is_input());
        assert_eq!(wv.get_size(), 4);
        assert_eq!(wv.get_addr(), &Address::new(r, 0x100));
        assert!(fd.vbank().get(lo).is_none(), "piece destroyed by the combine");
        assert!(fd.vbank().get(hi).is_none(), "piece destroyed by the combine");
    }
}
