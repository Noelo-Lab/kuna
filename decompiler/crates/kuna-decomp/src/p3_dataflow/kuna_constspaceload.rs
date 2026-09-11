//! (kuna) `constspaceload` — resolve a SLEIGH dynamic constant-space export to
//! the value it exports instead of leaving it as a pointer-shaped LOAD.
//!
//! # The symptom
//!
//! A stripped ARM binary zeroes a six-word structure with one NEON immediate
//! and one vector store:
//!
//! ```text
//! 4e0:  vmov.i32 q8, #0
//! 4f4:  vst1.64  {d16-d17}, [r1]!
//! ```
//!
//! and kuna renders the zeroing as three reads through a null pointer:
//!
//! ```c
//! v2 = *(unsigned int *)(0 + 4);
//! v3 = *(unsigned int *)(0 + 8);
//! v4 = *(unsigned int *)(0 + 0xc);
//! v1 = 0;
//! ```
//!
//! # Why
//!
//! `vmov.i32 q8,#0` is `simdExpImm_16` (`ARMneon.sinc:549`), whose body is
//! `tmp:16 = 0; export *[const]:16 tmp;` — a **dynamic** export from the
//! constant space. A dynamic export lowers to `LOAD(const, tmp)`, where the
//! constant space's defining property is that an address *is* its own value.
//! Nothing folds it away here: `RuleCollapseConstants` declines any op wider
//! than a `uintb` (`PcodeOp::isCollapsible`, eight bytes), so the 16-byte
//! `tmp` never becomes a constant Varnode, and `RuleLoadVarnode` — which is the
//! rule that knows the address-is-value identity — only matches a LOAD whose
//! pointer already is one.
//!
//! `ActionLaneDivide` then splits the surviving 16-byte LOAD into four one-word
//! lane LOADs at `tmp`, `tmp+4`, `tmp+8`, `tmp+0xc`. That split is *invalid* in
//! the constant space: offsetting an address there changes the value, it does
//! not select a word of it. Lane 0 needs no offset and folds to the correct
//! `0`; the other three are left reading near-null memory, which is the
//! symptom, and folding them where they stand would be worse still — it would
//! print the literal constants `4`, `8` and `0xc`.
//!
//! # The rule
//!
//! For the constant space, `LOAD(const, p)` **is** `p`: that is the same
//! identity `RuleLoadVarnode` applies to a constant `p`, and it does not stop
//! holding when `p` is a temporary. So the rewrite is
//!
//! ```text
//! out:N = LOAD(const, p:N)   ->   out = COPY p
//! ```
//!
//! and only at equal widths. A dynamic `export *[const]:N tmp` gives the
//! exported operand and `tmp` the same size, so a width mismatch is some other
//! shape; resizing there would be inventing a truncation rather than applying
//! an identity, and it was measured to do harm (see *Bounds*).
//!
//! Applied in `oppool1` the rewrite lands **before** `ActionLaneDivide`, so the
//! vector immediate is an ordinary 16-byte value by the time the lanes are cut
//! and each lane is split off the value rather than off an address. The zero
//! case above becomes four zero lanes; a non-zero immediate
//! (`vmov.i32 q8,#1`, whose SLEIGH arm builds a genuinely non-representable
//! 16-byte `val` before exporting it the same way) becomes the lanes of `val`.
//!
//! # Bounds
//!
//! * Only a LOAD whose space operand decodes to the **constant** space is
//!   matched; every other space is a real memory read and is left alone.
//! * Only an **equal-width** LOAD is matched. Rewriting a narrower or wider
//!   read as a `SUBPIECE`/`INT_ZEXT` of the pointer was tried and withdrawn: on
//!   statically linked AVX-512 glibc it re-renders live `k` mask registers, and
//!   in `__strlen_evex`-shaped code one such mask lost its reaching definition
//!   at a shared label (the scan loop read a stale earlier compare instead of
//!   the live one). The NEON case this rule exists for is equal-width.
//! * A constant pointer is left to `RuleLoadVarnode`, which additionally
//!   resolves the spacebase-placeholder tail a plain COPY would lose.
//! * A free pointer Varnode is declined: it has no definition to copy from.
//! * A LOAD whose output is the spacebase placeholder a call set up is declined
//!   for the same reason: only `RuleLoadVarnode` clears that trigger.
//!
//! Gated by the arch flag `const_space_load_fold` (option `constspaceload
//! on|off`, shipped default `on`); with the option off the rule is inert and
//! the output is byte-identical to upstream.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshalling element id for the option (kuna 4000+ range).
pub const ELEM_CONSTSPACELOAD: ElementId = ElementId::new("constspaceload", 4162);

/// The `constspaceload on|off` option (`Architecture::const_space_load_fold`).
pub struct OptionConstSpaceLoad;

impl OptionConstSpaceLoad {
    /// The option name.
    pub const NAME: &'static str = "constspaceload";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::const_space_load_fold`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Constant-space dynamic export folding turned {prop}")))
    }
}

/// Rewrite `LOAD(const, p)` to the value of `p` (see the module docs).
pub struct RuleConstSpaceLoad {
    enabled: bool,
    group: String,
}

impl RuleConstSpaceLoad {
    /// Build the rule in the default `analysis` group.
    pub fn new(enabled: bool) -> RuleConstSpaceLoad {
        RuleConstSpaceLoad::with_group(enabled, "analysis")
    }

    /// Build the rule for `group`; `enabled` forces it on independently of the
    /// architecture flag (used by the unit tests).
    pub fn with_group(enabled: bool, group: &str) -> RuleConstSpaceLoad {
        RuleConstSpaceLoad { enabled, group: group.to_string() }
    }
}

impl Rule for RuleConstSpaceLoad {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_LOAD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleConstSpaceLoad { enabled: self.enabled, group: self.group.clone() }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !data.get_arch().const_space_load_fold {
            return 0;
        }
        let (spacevn, ptr, out) = {
            let o = match data.obank().get(op) {
                Some(o) => o,
                None => return 0,
            };
            match (o.get_in(0), o.get_in(1), o.get_out()) {
                (Some(s), Some(p), Some(out)) => (s, p, out),
                _ => return 0,
            }
        };
        if !is_constant_space(data, spacevn) {
            return 0;
        }
        let ptrv = match data.vbank().get(ptr) {
            Some(v) => v,
            None => return 0,
        };
        // A constant pointer is RuleLoadVarnode's case; it also resolves the
        // spacebase-placeholder tail, which a bare COPY would drop.
        if ptrv.is_constant() || ptrv.is_free() {
            return 0;
        }
        let is_placeholder = data
            .vbank()
            .get(out)
            .map(|v| v.is_spacebase_placeholder())
            .unwrap_or(false);
        if is_placeholder {
            return 0;
        }
        let ptr_size = ptrv.get_size();
        let out_size = match data.vbank().get(out) {
            Some(v) => v.get_size(),
            None => return 0,
        };
        // Only the same-width identity. A dynamic `export *[const]:N tmp` gives
        // the operand and `tmp` the same size, so N != S is some other shape and
        // resizing there would be inventing a truncation the rest of the
        // pipeline has not agreed to -- measured to re-render live AVX-512 mask
        // registers, one of which lost its reaching definition.  See the module
        // docs, "Bounds".
        if out_size != ptr_size {
            return 0;
        }

        // `RuleLoadVarnode`'s order: overwrite the space operand first, then drop
        // the pointer slot, so neither Varnode is ever left without a descendant.
        data.op_set_input(op, ptr, 0).expect("RuleConstSpaceLoad: opSetInput");
        data.op_remove_input(op, 1);
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        1
    }
}

/// Does the LOAD/STORE space operand `vn` name the constant space?
fn is_constant_space(data: &Funcdata, vn: VarnodeId) -> bool {
    let v = match data.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    if !v.is_constant() {
        return false;
    }
    // LOSS-015: the space operand carries the space manager *index*, not the
    // raw `AddrSpace *` the C++ stores.
    let idx = v.get_offset();
    let manage = data.get_arch().manage();
    if idx >= manage.num_spaces() as u64 {
        return false;
    }
    manage
        .get_space(idx as i32)
        .map(|s| s.get_type() == spacetype::IPTR_CONSTANT)
        .unwrap_or(false)
}

/// Per-file registration rows.
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        // Shipped default: `option constspaceload on`.
        ctor: || Box::new(RuleConstSpaceLoad::with_group(true, "analysis")),
    }]
}

#[cfg(test)]
#[path = "kuna_constspaceload/tests.rs"]
mod tests;
