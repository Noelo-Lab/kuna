//! (kuna `lanefuse`) Re-fuse the per-lane p-code of a vector instruction.
//!
//! The x86 SLEIGH spec writes many SSE/AVX instructions lane by lane:
//! `VADDPS ymm0, ymm0, ymm1` is eight `FLOAT_ADD`s on 4-byte slices of the
//! registers. Heritage then sees eight 4-byte reads of `ymm0` and `ymm1`, never
//! the whole register, so neither the vector argument nor the vector result
//! exists as one value, and the body prints as eight float statements. This
//! rebuilds such a run into one op on the whole registers before heritage: a
//! user-op named for the Intel intrinsic it is (`_mm256_add_ps`), reading the
//! two source vectors and writing the destination vector.
//!
//! A run is two or more ops of one opcode in one instruction whose output and
//! inputs are consecutive equal-width slices of the same storage (or
//! `SUBPIECE`s of one varnode at consecutive offsets), covering 16, 32 or 64
//! bytes.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;

/// (kuna `lanefuse`) Which lane runs fuse: none, the VEX/EVEX and four-lane
/// float ones, or the legacy-SSE integer and two-lane double ones too.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LaneFuseMode {
    #[default]
    Off,
    On,
    All,
}

impl LaneFuseMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "on" => Some(Self::On),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
            Self::All => "all",
        }
    }
}

/// First built-in id of the fused-lane user-ops.
pub const BUILTIN_LANE_BASE: uint4 = 0x1000_0100;

/// The fusable opcodes and the intrinsic operation each one names. Lane-wise
/// AND/OR/XOR are deliberately absent: compilers use them on whole registers for
/// scalar idioms (`xorps x,x` to zero, `andps` with a sign mask for `fabs`),
/// which the scalar rules read only while the lanes stay separate. Their table
/// slots are kept so the built-in ids stay stable.
const OPS: [(OpCode, &str); 10] = [
    (OpCode::CPUI_FLOAT_ADD, "add"),
    (OpCode::CPUI_FLOAT_SUB, "sub"),
    (OpCode::CPUI_FLOAT_MULT, "mul"),
    (OpCode::CPUI_FLOAT_DIV, "div"),
    (OpCode::CPUI_INT_ADD, "add"),
    (OpCode::CPUI_INT_SUB, "sub"),
    (OpCode::CPUI_INT_MULT, "mullo"),
    (OpCode::CPUI_INT_AND, "and"),
    (OpCode::CPUI_INT_OR, "or"),
    (OpCode::CPUI_INT_XOR, "xor"),
];
const LANES: [int4; 4] = [1, 2, 4, 8];
const TOTALS: [int4; 3] = [16, 32, 64];

/// Every fused-lane built-in as `(id, intrinsic name)`.
pub fn builtins() -> Vec<(uint4, String)> {
    let mut out = Vec::new();
    for (oi, (code, op)) in OPS.iter().enumerate() {
        for (li, lane) in LANES.iter().enumerate() {
            for (ti, total) in TOTALS.iter().enumerate() {
                if let Some(name) = name(*code, op, *lane, *total) {
                    out.push((id(oi, li, ti), name));
                }
            }
        }
    }
    out
}

fn id(oi: usize, li: usize, ti: usize) -> uint4 {
    BUILTIN_LANE_BASE + (oi * 16 + li * 4 + ti) as uint4
}

fn name(code: OpCode, op: &str, lane: int4, total: int4) -> Option<String> {
    let w = match total {
        16 => "",
        32 => "256",
        _ => "512",
    };
    let float = matches!(code, OpCode::CPUI_FLOAT_ADD | OpCode::CPUI_FLOAT_SUB | OpCode::CPUI_FLOAT_MULT | OpCode::CPUI_FLOAT_DIV);
    let ty = if float {
        match lane {
            4 => "ps".to_string(),
            8 => "pd".to_string(),
            _ => return None,
        }
    } else if matches!(code, OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR) {
        format!("si{}", total * 8)
    } else {
        format!("epi{}", lane * 8)
    };
    if code == OpCode::CPUI_INT_MULT && lane == 1 {
        return None;
    }
    Some(format!("_mm{w}_{op}_{ty}"))
}

/// First built-in id of the masked forms of the SLEIGH vector user-ops: user-op
/// `i` has its merge-masked form at `BUILTIN_MASK_BASE + 2*i` and its
/// zero-masked form at `+ 1`.
pub const BUILTIN_MASK_BASE: uint4 = 0x1001_0000;

/// The masked built-ins for the SLEIGH vector user-ops `(index, name)`:
/// `vaddps_avx512f_mask` and `vaddps_avx512f_maskz` beside `vaddps_avx512f`.
pub fn mask_builtins<'a>(ops: impl Iterator<Item = (uint4, &'a [u8])>) -> Vec<(uint4, String)> {
    let mut out = Vec::new();
    for (i, nm) in ops {
        let Ok(name) = std::str::from_utf8(nm) else { continue };
        if crate::kuna_avxintrinsics::intrinsic(name, 64).is_none() {
            continue;
        }
        out.push((BUILTIN_MASK_BASE + 2 * i, format!("{name}_mask")));
        out.push((BUILTIN_MASK_BASE + 2 * i + 1, format!("{name}_maskz")));
    }
    out
}

/// One operand of a lane op: the storage it slices and the slice offset.
#[derive(Clone, PartialEq)]
enum Slice {
    /// A direct varnode at `(space index, offset)`.
    Direct(usize, u64, Address),
    /// `SUBPIECE(source, offset)`.
    Piece(VarnodeId, u64),
}

fn slice_of(data: &Funcdata, vn: VarnodeId) -> Option<Slice> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return None;
    }
    if let Some(def) = v.get_def().filter(|_| v.is_written()).and_then(|d| data.obank().get(d)) {
        if def.code() == OpCode::CPUI_SUBPIECE {
            let off = data.vbank().get(def.get_in(1)?)?.get_offset();
            return Some(Slice::Piece(def.get_in(0)?, off));
        }
    }
    let addr = v.get_addr().clone();
    Some(Slice::Direct(addr.get_space()?.get_index() as usize, addr.get_offset(), addr))
}

fn advance(s: &Slice, by: u64) -> Slice {
    match s {
        Slice::Direct(sp, off, a) => {
            Slice::Direct(*sp, off + by, Address::new(a.get_space().expect("slice space").clone(), off + by))
        }
        Slice::Piece(v, off) => Slice::Piece(*v, off + by),
    }
}

/// The whole operand a run of slices starting at `s` covers: the source of a
/// `SUBPIECE` run, or a new varnode over `total` bytes of direct storage.
fn whole(data: &mut Funcdata, s: &Slice, total: int4) -> Option<VarnodeId> {
    match s {
        Slice::Piece(v, 0) => (data.vbank().get(*v)?.get_size() == total).then_some(*v),
        Slice::Piece(..) => None,
        Slice::Direct(_, _, addr) => Some(data.new_varnode(total, addr, None)),
    }
}

/// Fuse every lane run among the ops of one instruction (`ops`, in order).
/// `vex` also stands for `lanefuse all`, which fuses legacy-SSE runs as if
/// they were VEX-encoded.
pub fn fuse(data: &mut Funcdata, ops: &[OpId], vex: bool) -> KunaResult<bool> {
    let mut changed = false;
    let mut i = 0;
    while i < ops.len() {
        let Some((oi, run, end)) = run_at(data, ops, i, vex) else {
            i += 1;
            continue;
        };
        let first = run[0];
        let lane = data.obank().get(first).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)).map(|v| v.get_size()).unwrap_or(0);
        let total = lane * run.len() as int4;
        let (Some(li), Some(ti)) = (LANES.iter().position(|&l| l == lane), TOTALS.iter().position(|&t| t == total)) else {
            i = end;
            continue;
        };
        let (code, op) = OPS[oi];
        if name(code, op, lane, total).is_none() {
            i = end;
            continue;
        }
        let (Some(out), Some(a), Some(b)) = operands(data, first) else {
            i = end;
            continue;
        };
        let (Some(va), Some(vb)) = (whole(data, &a, total), whole(data, &b, total)) else {
            i = end;
            continue;
        };
        let Slice::Direct(_, _, out_addr) = out else {
            i = end;
            continue;
        };
        // The first lane op becomes the fused op, so the instruction keeps its
        // first op (flow has already marked it as the instruction start).
        let idc = data.new_constant(4, id(oi, li, ti) as u64);
        data.op_set_opcode_code(first, OpCode::CPUI_CALLOTHER);
        if let Some(o) = data.obank_mut().get_mut(first) {
            o.clear_flag(pcodeop_flags::call);
        }
        data.op_set_all_input(first, &[idc, va, vb])?;
        data.op_unset_output(first);
        data.new_varnode_out(total, &out_addr, first)?;
        for &dead in &run[1..] {
            data.op_destroy_raw(dead)?;
        }
        changed = true;
        i = end;
    }
    Ok(changed)
}

fn operands(data: &Funcdata, op: OpId) -> (Option<Slice>, Option<Slice>, Option<Slice>) {
    let Some(o) = data.obank().get(op) else { return (None, None, None) };
    let out = o.get_out().and_then(|v| {
        let v = data.vbank().get(v)?;
        let a = v.get_addr().clone();
        Some(Slice::Direct(a.get_space()?.get_index() as usize, a.get_offset(), a))
    });
    (out, o.get_in(0).and_then(|v| slice_of(data, v)), o.get_in(1).and_then(|v| slice_of(data, v)))
}

/// The lane run that starts at `ops[i]`: its opcode index, its ops, and the
/// position in `ops` just past the last of them.
fn run_at(data: &Funcdata, ops: &[OpId], i: usize, vex: bool) -> Option<(usize, Vec<OpId>, usize)> {
    let o = data.obank().get(ops[i])?;
    let oi = OPS.iter().position(|(c, _)| *c == o.code())?;
    if matches!(o.code(), OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR) {
        return None;
    }
    // Legacy-encoded SSE integer lanes are left to the scalar lane rules
    // (`devectorize`, `wideslice`, `simdlane`): compilers emit them for
    // auto-vectorised scalar loops, which those rules turn back into the source.
    if !vex && !matches!(o.code(), OpCode::CPUI_FLOAT_ADD | OpCode::CPUI_FLOAT_SUB | OpCode::CPUI_FLOAT_MULT | OpCode::CPUI_FLOAT_DIV) {
        return None;
    }
    // Legacy-encoded two-lane double runs are the SLP vectoriser pairing two
    // scalar computations far more often than an `__m128d` in the source, so
    // they also stay with the scalar rules.
    if !vex && data.vbank().get(o.get_out()?)?.get_size() == 8 {
        return None;
    }
    let lane = data.vbank().get(o.get_out()?)?.get_size() as u64;
    let (Some(out0), Some(a0), Some(b0)) = operands(data, ops[i]) else { return None };
    let mut run = vec![ops[i]];
    let mut end = i + 1;
    // Lane ops of one run may be separated by the SUBPIECEs that feed them.
    let mut j = i + 1;
    while j < ops.len() {
        let oj = data.obank().get(ops[j])?;
        if oj.code() == OpCode::CPUI_SUBPIECE || oj.code() == OpCode::CPUI_COPY && !oj.get_out().and_then(|v| data.vbank().get(v)).is_some_and(|v| v.get_addr().get_space().is_some_and(|s| s.get_name() == "register")) {
            j += 1;
            continue;
        }
        if oj.code() != o.code() {
            break;
        }
        let step = lane * run.len() as u64;
        let (Some(out), Some(a), Some(b)) = operands(data, ops[j]) else { break };
        if out != advance(&out0, step) || a != advance(&a0, step) || b != advance(&b0, step) {
            break;
        }
        run.push(ops[j]);
        j += 1;
        end = j;
    }
    (run.len() >= 2).then_some((oi, run, end))
}

/// The storage address `vn` names, if it is a register or temporary.
fn addr_of(data: &Funcdata, vn: Option<VarnodeId>) -> Option<Address> {
    let v = data.vbank().get(vn?)?;
    (!v.is_constant()).then(|| v.get_addr().clone())
}

/// Fold an AVX-512 write mask back into its instruction's user-op.
///
/// SLEIGH applies `{k}` lane by lane after the user-op: for each lane `j`,
/// `r[j] = zext(k_j) * r[j] | zext(!k_j) * src[j]` (merge) or
/// `r[j] = zext(k_j) * r[j]` (zero). This finds the user-op writing `r`, checks
/// that every lane of `r` is rewritten so, and turns the user-op into its masked
/// built-in reading `(src, k, operands...)` or `(k, operands...)`; the lane ops
/// and the mask extraction are removed.
pub fn fold_mask(data: &mut Funcdata, ops: &[OpId], evex_z: bool) -> KunaResult<bool> {
    let Some(pos) = ops.iter().position(|&op| {
        data.obank().get(op).is_some_and(|o| {
            o.code() == OpCode::CPUI_CALLOTHER
                && o.get_in(0).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.get_offset() < 0x1000_0000)
        })
    }) else {
        return Ok(false);
    };
    let call = ops[pos];
    let Some(r_addr) = addr_of(data, data.obank().get(call).and_then(|o| o.get_out())) else { return Ok(false) };
    let r_size = data.obank().get(call).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)).map(|v| v.get_size()).unwrap_or(0);
    // The lane writes: ORs (merge) or MULTs (zero) into consecutive slices of r.
    let lane_writes: Vec<(usize, OpId)> = ops[pos + 1..]
        .iter()
        .enumerate()
        .filter(|(_, &op)| {
            addr_of(data, data.obank().get(op).and_then(|o| o.get_out())).is_some_and(|a| {
                a.get_space().map(|s| s.get_index()) == r_addr.get_space().map(|s| s.get_index()) && a.get_offset() >= r_addr.get_offset() && a.get_offset() < r_addr.get_offset() + r_size as u64
            })
        })
        .map(|(k, &op)| (pos + 1 + k, op))
        .collect();
    let Some(&(_, first_write)) = lane_writes.first() else { return Ok(false) };
    let lane = data.obank().get(first_write).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)).map(|v| v.get_size()).unwrap_or(0);
    if !matches!(lane, 4 | 8) || lane_writes.len() as int4 * lane != r_size {
        return Ok(false);
    }
    let merge = data.obank().get(first_write).is_some_and(|o| o.code() == OpCode::CPUI_INT_OR);
    for (j, &(_, op)) in lane_writes.iter().enumerate() {
        let Some(o) = data.obank().get(op) else { return Ok(false) };
        let want = if merge { OpCode::CPUI_INT_OR } else { OpCode::CPUI_INT_MULT };
        let at = addr_of(data, o.get_out());
        if o.code() != want || at.map(|a| a.get_offset()) != Some(r_addr.get_offset() + j as u64 * lane as u64) {
            return Ok(false);
        }
    }
    // The merge source: the storage the second product of lane 0 reads.
    let range_end = lane_writes.last().map(|&(i, _)| i).expect("lanes");
    let unique_writes = |data: &Funcdata, op: OpId| {
        addr_of(data, data.obank().get(op).and_then(|o| o.get_out()))
            .is_some_and(|a| a.get_space().is_some_and(|s| s.get_name() == "unique"))
    };
    let mut src: Option<Address> = None;
    if merge {
        for &op in &ops[pos + 1..range_end] {
            let Some(o) = data.obank().get(op) else { continue };
            if o.code() == OpCode::CPUI_INT_MULT {
                if let Some(a) = addr_of(data, o.get_in(1)) {
                    if a.get_space().map(|s| s.get_index()) == r_addr.get_space().map(|s| s.get_index()) && a.get_offset() != r_addr.get_offset() {
                        src = Some(a);
                        break;
                    }
                }
            }
        }
    }
    // The merge source is a copy the instruction makes after the user-op (of the
    // destination, or of zero under `{z}`): read what that copy reads.
    let mut src_zero = false;
    if let Some(s) = src.clone() {
        let writer = ops.iter().copied().find(|&op| {
            data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_COPY)
                && addr_of(data, data.obank().get(op).and_then(|o| o.get_out())).is_some_and(|a| a == s)
        });
        if let Some(w) = writer {
            let input = data.obank().get(w).and_then(|o| o.get_in(0)).and_then(|v| data.vbank().get(v));
            match input {
                Some(v) if v.is_constant() => src_zero = v.get_offset() == 0,
                Some(v) => src = Some(v.get_addr().clone()),
                None => {}
            }
        }
    }
    // A merge whose other product is by zero is zero-masking spelled as a merge.
    let zeroing = evex_z
        || src_zero
        || !merge
        || src.is_none()
            && ops[pos + 1..range_end].iter().any(|&op| {
                data.obank().get(op).is_some_and(|o| {
                    o.code() == OpCode::CPUI_INT_MULT
                        && (0..2).any(|k| o.get_in(k).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_constant() && v.get_offset() == 0))
                })
            });
    if merge && src.is_none() && !zeroing {
        return Ok(false);
    }
    // The mask: the register copied into the temporary the lane bits come from.
    let mut mask: Option<(Address, int4)> = None;
    let mut doomed = Vec::new();
    for &op in &ops[pos + 1..=range_end] {
        let Some(o) = data.obank().get(op) else { continue };
        if lane_writes.iter().any(|&(_, w)| w == op) {
            doomed.push(op);
            continue;
        }
        if !unique_writes(data, op) {
            continue;
        }
        if o.code() == OpCode::CPUI_COPY && mask.is_none() {
            if let Some(v) = o.get_in(0).and_then(|v| data.vbank().get(v)) {
                if v.get_addr().get_space().is_some_and(|s| s.get_name() == "register") {
                    mask = Some((v.get_addr().clone(), v.get_size()));
                }
            }
        }
        doomed.push(op);
    }
    let Some((mask_addr, mask_size)) = mask else { return Ok(false) };
    let id = data.obank().get(call).and_then(|o| o.get_in(0)).and_then(|v| data.vbank().get(v)).map(|v| v.get_offset()).unwrap_or(0);
    let masked = BUILTIN_MASK_BASE as u64 + 2 * id + u64::from(zeroing);
    let mut inputs: Vec<VarnodeId> = Vec::new();
    inputs.push(data.new_constant(4, masked));
    if let Some(src) = src.as_ref().filter(|_| !zeroing) {
        inputs.push(data.new_varnode(r_size, src, None));
    }
    // The mask register holds one bit per lane; pass it at the width the
    // intrinsic takes (`__mmask16` for sixteen lanes).
    let lanes = lane_writes.len() as int4;
    let mask_bytes = ((lanes + 7) / 8).max(1).min(mask_size);
    inputs.push(data.new_varnode(mask_bytes, &mask_addr, None));
    let n = data.obank().get(call).map(|o| o.num_input()).unwrap_or(1);
    for k in 1..n {
        if let Some(v) = data.obank().get(call).and_then(|o| o.get_in(k)) {
            inputs.push(v);
        }
    }
    data.op_set_all_input(call, &inputs)?;
    for op in doomed {
        data.op_destroy_raw(op)?;
    }
    // `dst = COPY r` after the lanes: the masked op writes `dst` itself.
    let copy = ops[range_end + 1..].iter().copied().find(|&op| {
        data.obank().get(op).is_some_and(|o| {
            o.code() == OpCode::CPUI_COPY && addr_of(data, o.get_in(0)).is_some_and(|a| a == r_addr)
                && o.get_in(0).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.get_size() == r_size)
        })
    });
    if let Some(copy) = copy {
        let dst = addr_of(data, data.obank().get(copy).and_then(|o| o.get_out()));
        if let Some(dst) = dst {
            data.op_destroy_raw(copy)?;
            data.op_unset_output(call);
            data.new_varnode_out(r_size, &dst, call)?;
        }
    }
    Ok(true)
}
