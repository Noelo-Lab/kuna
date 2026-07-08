//! Raw p-code (SSA listing) rendering for a [`Funcdata`] — the `print raw`
//! console command (C++ `Funcdata::printRaw`, `funcdata.cc:207`).
//!
//! This is a faithful transcription of the upstream `printRaw` family, split
//! across three C++ files:
//!
//!   * `Funcdata::printRaw` (`funcdata.cc:207`)            → [`print_raw`]
//!   * `BlockGraph::printRaw` / `BlockBasic::printRaw` /
//!     `printHeader` / `printRawImpliedGoto` (`block.cc`)   → the block walk
//!   * `Varnode::printRaw` / `printRawNoMarkup`
//!     (`varnode.cc:730,760,1226`)                          → [`render_varnode`]
//!   * the per-op-code `TypeOp*::printRaw` overrides
//!     (`typeop.cc`)                                         → [`render_op`]
//!
//! Every accessor used here is read-only; the function takes the *real*
//! [`Architecture`] (for register-name resolution and the default address size)
//! and a `&Funcdata`, exactly as the C++ `IfcPrintRaw::execute` reaches
//! `dcp->fd->printRaw(*status->fileoptr)` against `dcp->conf`.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::typeop::{type_op_info, TypeOpClass};
use crate::varnode::varnode_flags;

/// C++ `Funcdata::printRaw(ostream &s)` (`funcdata.cc:207`).
///
/// If basic blocks have been generated (the post-`decompile` state the console
/// drives), defer to the block-graph walk; otherwise dump the raw op tree.
pub fn print_raw(arch: &Architecture, fd: &Funcdata) -> Result<String, String> {
    let mut s = String::new();
    if fd.bblocks_get_size() == 0 {
        // C++: if (obank.empty()) throw RecovError("No operations to print");
        let ops: Vec<OpId> = fd.obank().iter_all().map(|(_, id)| id).collect();
        if ops.is_empty() {
            return Err("No operations to print".to_string());
        }
        s.push_str("Raw operations: \n");
        for op in ops {
            render_seqnum(&mut s, fd, op)?;
            s.push_str(":\t");
            render_op(&mut s, arch, fd, op)?;
            s.push('\n');
        }
        return Ok(s);
    }
    print_block_graph_raw(&mut s, arch, fd)?;
    Ok(s)
}

/// C++ `BlockGraph::printRaw(ostream &s)` (`block.cc:1301`) over the top-level
/// `bblocks` list: `printHeader`, then each block's `printRaw`, with the
/// `printRawImpliedGoto` fall-through marker between consecutive blocks.
fn print_block_graph_raw(s: &mut String, arch: &Architecture, fd: &Funcdata) -> Result<(), String> {
    // C++ printHeader for the BlockGraph: `dec << index` (+ optional range). The
    // graph node's index is 0 for the top-level component graph; the range is
    // invalid for a non-basic graph block, so just the index is printed.
    let root = fd.bblocks_ref().root.ok_or_else(|| "no block graph root".to_string())?;
    let root_index = fd.bblocks_ref().block(root).get_index();
    s.push_str(&format!("{}", root_index));
    s.push('\n');

    let n = fd.bblocks_get_size();
    if n == 0 {
        return Ok(());
    }
    let mut last_bl = fd.bblocks_get_block(0);
    print_basic_block_raw(s, arch, fd, last_bl)?;
    for i in 1..n {
        let cur_bl = fd.bblocks_get_block(i);
        print_raw_implied_goto(s, fd, last_bl, cur_bl)?;
        print_basic_block_raw(s, arch, fd, cur_bl)?;
        last_bl = cur_bl;
    }
    Ok(())
}

/// C++ `BlockBasic::printRaw` (`block.cc:2720`): `printHeader` then each op.
fn print_basic_block_raw(
    s: &mut String,
    arch: &Architecture,
    fd: &Funcdata,
    bl: BlockId,
) -> Result<(), String> {
    // C++ `BlockBasic::printHeader`: "Basic Block " + FlowBlock::printHeader.
    s.push_str("Basic Block ");
    print_flow_header(s, fd, bl);
    s.push('\n');
    for op in fd.bb_ops(bl) {
        render_seqnum(s, fd, op)?;
        s.push_str(":\t");
        render_op(s, arch, fd, op)?;
        s.push('\n');
    }
    Ok(())
}

/// C++ `FlowBlock::printHeader` (`block.cc:603`): `dec << index` plus the
/// `start-stop` address range when both are valid.
fn print_flow_header(s: &mut String, fd: &Funcdata, bl: BlockId) {
    let index = fd.bblocks_ref().block(bl).get_index();
    s.push_str(&format!("{index}"));
    let start = crate::block::block_get_start(&fd.bblocks_ref().arena, bl);
    let stop = crate::block::block_get_stop(&fd.bblocks_ref().arena, bl);
    if !start.is_invalid() && !stop.is_invalid() {
        s.push(' ');
        let _ = start.print_raw(s);
        s.push('-');
        let _ = stop.print_raw(s);
    }
}

/// C++ `FlowBlock::printShortHeader` (`block.cc`): `Block_<index>` plus the
/// `:<start>` address when valid.  Used by the BRANCH/CBRANCH printRaw arms.
fn print_short_header(s: &mut String, fd: &Funcdata, bl: BlockId) {
    let index = fd.bblocks_ref().block(bl).get_index();
    s.push_str(&format!("Block_{index}"));
    let start = crate::block::block_get_start(&fd.bblocks_ref().arena, bl);
    if !start.is_invalid() {
        s.push(':');
        let _ = start.print_raw(s);
    }
}

/// C++ `BlockBasic::printRawImpliedGoto` (`block.cc:2736`): emit a
/// `[ goto Block_x ]` marker between two consecutive blocks when control falls
/// through to a block that is not the single successor.
fn print_raw_implied_goto(
    s: &mut String,
    fd: &Funcdata,
    bl: BlockId,
    next_block: BlockId,
) -> Result<(), String> {
    let block = fd.bblocks_ref().block(bl);
    if block.size_out() != 1 {
        return Ok(());
    }
    let out_block = block.get_out(0);
    // C++ resolves `nextBlock` to a basic block via getFrontLeaf/subBlock when it
    // is not already basic.  At the `bblocks` level every member is basic, so the
    // resolution is the identity.
    if out_block == next_block {
        return Ok(());
    }
    // if (!op.empty() && op.back()->isBranch()) return;
    let ops = fd.bb_ops(bl);
    if let Some(&last) = ops.last() {
        if fd.obank().get(last).map(|o| o.is_branch()).unwrap_or(false) {
            return Ok(());
        }
    }
    // C++ `getStop().printRaw(s)`: the block's stop address.
    let stop = crate::block::block_get_stop(&fd.bblocks_ref().arena, bl);
    let _ = stop.print_raw(s);
    s.push_str(":   \t[ goto ");
    print_short_header(s, fd, out_block);
    s.push_str(" ]\n");
    Ok(())
}

/// C++ `s << inst->getSeqNum()` (the `operator<<(ostream&,const SeqNum&)`):
/// `pc.printRaw(s); s << ':' << uniq;` with the trailing `uniq` in hex (the
/// preceding address leaves the stream base in hex).
fn render_seqnum(s: &mut String, fd: &Funcdata, op: OpId) -> Result<(), String> {
    let o = fd.obank().get(op).ok_or_else(|| "render_seqnum: stale op".to_string())?;
    o.get_seq_num().print_raw(s).map_err(|e| e.explain().to_string())
}

/// C++ `Varnode::printRaw(ostream &s,const Varnode *vn)` (`varnode.cc:1226`):
/// `<null>` for a missing varnode, else the markup form below.
fn render_varnode_opt(
    s: &mut String,
    arch: &Architecture,
    fd: &Funcdata,
    vn: Option<VarnodeId>,
) -> Result<(), String> {
    match vn {
        None => {
            s.push_str("<null>");
            Ok(())
        }
        Some(v) => render_varnode(s, arch, fd, v),
    }
}

/// C++ `Varnode::printRaw(ostream &s)` (`varnode.cc:760`): the markup form —
/// the base identifier (`printRawNoMarkup`) plus a `:size` suffix when the
/// register name does not cover the full size, the `(i)` input marker, the
/// `(<defining-seqnum>)` written marker, and the `(free)` trailing marker.
fn render_varnode(
    s: &mut String,
    arch: &Architecture,
    fd: &Funcdata,
    vn: VarnodeId,
) -> Result<(), String> {
    let v = fd.vbank().get(vn).ok_or_else(|| "render_varnode: stale vn".to_string())?;
    let size = v.get_size();
    let flags = v.get_flags();
    let addr = v.get_addr().clone();
    let is_written = v.is_written();
    let def = v.get_def();

    // C++ `printRawNoMarkup` dispatches `loc.printRaw(s)` to the space's
    // `printRaw` override.  For an \e iop-space Varnode (the second input of a
    // CPUI_INDIRECT, encoding the call/return op it guards around), that override
    // is `IopSpace::printRaw` (op.cc:41-59), which decodes the offset back to the
    // PcodeOp and prints `op->getSeqNum()` (the non-branch arm).  kuna-base's
    // `IopSpace::printRaw` leaves this to W3 (LOSS-012), so route it here where the
    // op arena is reachable: shortcut char `i` + the referenced op's seqnum.
    let expect = if addr.get_space().map(|sp| sp.get_type() == spacetype::IPTR_IOP) == Some(true) {
        // s << loc.getShortcut(); expect = trans->getDefaultSize(); loc.printRaw(s);
        let space = addr.get_space().expect("render_varnode: iop space present");
        s.push(space.get_shortcut());
        let referenced = crate::funcdata_varnode::op_iop_decode(addr.get_offset());
        // The non-branch (CPUI_INDIRECT) arm of IopSpace::printRaw needs only the
        // op's seqnum.  The branch arm's `block_info` closure is unreached for an
        // INDIRECT iop input (op.is_branch() == false); supply a panicking stub so
        // a future branch-form iop varnode here is loudly caught, not silently
        // mis-rendered.
        if let Some(referenced_op) = fd.obank().get(referenced) {
            crate::op::iop_space_print_raw(referenced_op, s, &|_op| {
                unreachable!("iop-space printRaw branch arm unreached for an INDIRECT iop input")
            })
            .map_err(|e| e.explain().to_string())?;
        }
        // C++ `expect = trans->getDefaultSize()`.
        arch.manage().get_default_size()
    } else {
        render_varnode_no_markup(s, arch, &addr, size)?
    };
    if expect != size {
        // C++ `s << ':' << setw(1) << size;` — a single decimal field.
        s.push(':');
        s.push_str(&format!("{size}"));
    }
    if (flags & varnode_flags::input) != 0 {
        s.push_str("(i)");
    }
    if is_written {
        if let Some(d) = def {
            s.push('(');
            render_seqnum(s, fd, d)?;
            s.push(')');
        }
    }
    if (flags & (varnode_flags::insert | varnode_flags::constant)) == 0 {
        s.push_str("(free)");
    }
    Ok(())
}

/// C++ `Varnode::printRawNoMarkup(ostream &s)` (`varnode.cc:730`): the bare
/// storage identifier — a register name (`+offset` when not register-aligned),
/// else the space shortcut char + the raw address.  Returns the `expect`-ed
/// size (the register's size, or the translator's default size) the caller
/// compares against the varnode size to decide on the `:size` suffix.
fn render_varnode_no_markup(
    s: &mut String,
    arch: &Architecture,
    addr: &Address,
    size: int4,
) -> Result<int4, String> {
    let space = match addr.get_space() {
        Some(sp) => sp,
        None => {
            // No space (a sentinel) — C++ would deref the space; this never
            // arises for a real varnode in the listing.  Fall back to the raw
            // address so the rendering never panics.
            let _ = addr.print_raw(s);
            return Ok(size);
        }
    };
    let off = addr.get_offset();
    // Register-name/storage resolution is a Sleigh-engine concern (the C++
    // `getTrans()` back-pointer); reach the concrete engine's `SleighBase`
    // through the seam downcast (only `Sleigh` implements it; this print-raw
    // path only ever runs on the standalone engine).
    let sleigh = arch.translate().as_sleigh().expect("print_raw: standalone Sleigh engine");
    let name = sleigh.base().get_register_name(space, off, size);
    if !name.is_empty() {
        // C++ reads the canonical register storage to know the base offset/size.
        let point = sleigh
            .base()
            .get_register(&name)
            .map_err(|e| e.explain().to_string())?;
        let point_off = point.offset;
        let point_size = point.size as int4;
        s.push_str(&String::from_utf8_lossy(&name));
        let off_delta = off.wrapping_sub(point_off);
        if off_delta != 0 {
            // C++ `s << '+' << dec << off;`
            s.push('+');
            s.push_str(&format!("{off_delta}"));
        }
        Ok(point_size)
    } else {
        // C++ `s << loc.getShortcut(); ... loc.printRaw(s);`
        s.push(space.get_shortcut());
        let expect = arch.manage().get_default_size();
        let _ = addr.print_raw(s);
        Ok(expect)
    }
}

/// The operator name with the per-op-code size suffix C++ appends in
/// `getOperatorName` (ZEXT/SEXT/SUBPIECE append `in0.size`+`out.size`,
/// CONCAT appends `in0.size`+`in1.size`, the CARRY/BORROW family appends
/// `in0.size`).  For every other op-code the bare [`type_op_info`] name.
fn operator_name(fd: &Funcdata, op: OpId, opc: OpCode) -> Result<String, String> {
    let base = type_op_info(opc).get_name();
    let o = fd.obank().get(op).ok_or_else(|| "operator_name: stale op".to_string())?;
    let in0_size = || -> Result<int4, String> {
        let v = o.get_in(0).ok_or_else(|| "operator_name: missing in0".to_string())?;
        Ok(fd.vbank().get(v).ok_or_else(|| "operator_name: stale in0".to_string())?.get_size())
    };
    let in1_size = || -> Result<int4, String> {
        let v = o.get_in(1).ok_or_else(|| "operator_name: missing in1".to_string())?;
        Ok(fd.vbank().get(v).ok_or_else(|| "operator_name: stale in1".to_string())?.get_size())
    };
    let out_size = || -> Result<int4, String> {
        let v = o.get_out().ok_or_else(|| "operator_name: missing out".to_string())?;
        Ok(fd.vbank().get(v).ok_or_else(|| "operator_name: stale out".to_string())?.get_size())
    };
    Ok(match opc {
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_SUBPIECE => {
            format!("{}{}{}", base, in0_size()?, out_size()?)
        }
        OpCode::CPUI_PIECE => {
            format!("{}{}{}", base, in0_size()?, in1_size()?)
        }
        OpCode::CPUI_INT_CARRY | OpCode::CPUI_INT_SCARRY | OpCode::CPUI_INT_SBORROW => {
            format!("{}{}", base, in0_size()?)
        }
        _ => base.to_string(),
    })
}

/// C++ `TypeOpCallother::getOperatorName(op)` (typeop.cc): resolve the userop
/// index in `op`'s input(0) against `glb->userops` and return its displayed
/// symbol; on an unknown index fall back to `<callother>[<in0>]`.
fn callother_operator_name(
    arch: &Architecture,
    fd: &Funcdata,
    op: OpId,
) -> Result<String, String> {
    let o = fd.obank().get(op).ok_or_else(|| "callother: stale op".to_string())?;
    let in0 = o.get_in(0);
    if let Some(in0) = in0 {
        let in0vn = fd.vbank().get(in0).ok_or_else(|| "callother: stale in0".to_string())?;
        let index = in0vn.get_offset() as uint4;
        if let Some(userop) = arch.userops.get_op(index) {
            // out.size for VolatileReadOp, in(2).size for VolatileWriteOp; the
            // base override ignores both.
            let out_size = o
                .get_out()
                .and_then(|v| fd.vbank().get(v))
                .map(|v| v.get_size());
            let in2_size = if o.num_input() > 2 {
                o.get_in(2).and_then(|v| fd.vbank().get(v)).map(|v| v.get_size())
            } else {
                None
            };
            let name = userop.get_operator_name(out_size, in2_size);
            return Ok(String::from_utf8_lossy(&name).into_owned());
        }
    }
    // Unknown index: `<callother-base-name>[<in0 raw>]`.
    let mut fallback = type_op_info(OpCode::CPUI_CALLOTHER).get_name().to_string();
    fallback.push('[');
    let mut in0buf = String::new();
    render_varnode_opt(&mut in0buf, arch, fd, in0)?;
    fallback.push_str(&in0buf);
    fallback.push(']');
    Ok(fallback)
}

/// C++ `PcodeOp::printRaw(s)` = `opcode->printRaw(s,this)` — the per-op-code
/// dispatch.  The special op-codes (LOAD/STORE/branches/calls/RETURN/marker)
/// match their `TypeOp*::printRaw` override; everything else falls to the
/// generic Binary/Unary/Func shape keyed by the [`TypeOpClass`].
fn render_op(s: &mut String, arch: &Architecture, fd: &Funcdata, op: OpId) -> Result<(), String> {
    let opc = fd.obank().get(op).ok_or_else(|| "render_op: stale op".to_string())?.code();
    let num_input = fd.obank().get(op).ok_or_else(|| "render_op: stale op".to_string())?.num_input();
    let in_at = |i: int4| -> Option<VarnodeId> { fd.obank().get(op).and_then(|o| o.get_in(i)) };
    let out = fd.obank().get(op).and_then(|o| o.get_out());

    match opc {
        OpCode::CPUI_COPY => {
            // TypeOpCopy: `out = in0` (typeop.cc:426).
            render_varnode_opt(s, arch, fd, out)?;
            s.push_str(" = ");
            render_varnode_opt(s, arch, fd, in_at(0))?;
        }
        OpCode::CPUI_LOAD => {
            // TypeOpLoad: `out = *(<spacename>,in1)` (typeop.cc:503).
            render_varnode_opt(s, arch, fd, out)?;
            s.push_str(" = *(");
            push_space_from_const(s, arch, fd, op, 0)?;
            s.push(',');
            render_varnode_opt(s, arch, fd, in_at(1))?;
            s.push(')');
        }
        OpCode::CPUI_STORE => {
            // TypeOpStore: `*(<spacename>,in1) = in2` (typeop.cc:574).
            s.push_str("*(");
            push_space_from_const(s, arch, fd, op, 0)?;
            s.push(',');
            render_varnode_opt(s, arch, fd, in_at(1))?;
            s.push_str(") = ");
            render_varnode_opt(s, arch, fd, in_at(2))?;
        }
        OpCode::CPUI_BRANCH => {
            // TypeOpBranch: `goto <short-header|in0>` (typeop.cc:592).
            s.push_str(type_op_info(opc).get_name());
            s.push(' ');
            if let Some((parent, 1)) = parent_size_out(fd, op) {
                print_short_header(s, fd, fd.bblocks_ref().block(parent).get_out(0));
            } else {
                render_varnode_opt(s, arch, fd, in_at(0))?;
            }
        }
        OpCode::CPUI_CBRANCH => {
            render_cbranch(s, arch, fd, op)?;
        }
        OpCode::CPUI_BRANCHIND => {
            // TypeOpBranchind: `switch in0` (typeop.cc:655).
            s.push_str(type_op_info(opc).get_name());
            s.push(' ');
            render_varnode_opt(s, arch, fd, in_at(0))?;
        }
        OpCode::CPUI_CALL => {
            render_call(s, arch, fd, op, type_op_info(opc).get_name(), true)?;
        }
        OpCode::CPUI_CALLIND => {
            render_call(s, arch, fd, op, type_op_info(opc).get_name(), false)?;
        }
        OpCode::CPUI_CALLOTHER => {
            // TypeOpCallother::printRaw (typeop.cc:820): `[out =] <name>(in1,...)`,
            // where `<name>` is `getOperatorName(op)` — resolve the userop index in
            // input(0) against `glb->userops` and ask it for the displayed symbol
            // (`read_volatile_1`, etc.).  When the index is unknown the C++ falls
            // back to `<baseopname>[<in0>]` (typeop.cc getOperatorName tail).
            if out.is_some() {
                render_varnode_opt(s, arch, fd, out)?;
                s.push_str(" = ");
            }
            let name = callother_operator_name(arch, fd, op)?;
            s.push_str(&name);
            if num_input > 1 {
                s.push('(');
                render_varnode_opt(s, arch, fd, in_at(1))?;
                for i in 2..num_input {
                    s.push(',');
                    render_varnode_opt(s, arch, fd, in_at(i))?;
                }
                s.push(')');
            }
        }
        OpCode::CPUI_RETURN => {
            // TypeOpReturn (typeop.cc:884): `return[(in0)] [in1[,in2...]]`.
            s.push_str(type_op_info(opc).get_name());
            if num_input >= 1 {
                s.push('(');
                render_varnode_opt(s, arch, fd, in_at(0))?;
                s.push(')');
            }
            if num_input > 1 {
                s.push(' ');
                render_varnode_opt(s, arch, fd, in_at(1))?;
                for i in 2..num_input {
                    s.push(',');
                    render_varnode_opt(s, arch, fd, in_at(i))?;
                }
            }
        }
        OpCode::CPUI_MULTIEQUAL => {
            // TypeOpMulti (typeop.cc:1969): `out = in0 <op> in1 <op> in2 ...`.
            render_varnode_opt(s, arch, fd, out)?;
            s.push_str(" = ");
            render_varnode_opt(s, arch, fd, in_at(0))?;
            let name = operator_name(fd, op, opc)?;
            if num_input == 1 {
                s.push(' ');
                s.push_str(&name);
            }
            for i in 1..num_input {
                s.push(' ');
                s.push_str(&name);
                s.push(' ');
                render_varnode_opt(s, arch, fd, in_at(i))?;
            }
        }
        OpCode::CPUI_INDIRECT => {
            // TypeOpIndirect (typeop.cc:2024): `out = [create]|in0 [] in1`.
            render_varnode_opt(s, arch, fd, out)?;
            s.push_str(" = ");
            let is_create =
                fd.obank().get(op).map(|o| o.is_indirect_creation()).unwrap_or(false);
            if is_create {
                s.push_str("[create] ");
            } else {
                render_varnode_opt(s, arch, fd, in_at(0))?;
                s.push(' ');
                s.push_str(&operator_name(fd, op, opc)?);
                s.push(' ');
            }
            render_varnode_opt(s, arch, fd, in_at(1))?;
        }
        OpCode::CPUI_INT_SRIGHT => {
            // TypeOpIntSright (typeop.cc:1577): `out = in0 s>> in1`.
            render_varnode_opt(s, arch, fd, out)?;
            s.push_str(" = ");
            render_varnode_opt(s, arch, fd, in_at(0))?;
            s.push_str(" s>> ");
            render_varnode_opt(s, arch, fd, in_at(1))?;
        }
        _ => {
            // Generic Binary / Unary / Func shape (TypeOpBinary/Unary/Func).
            let class = type_op_info(opc).class;
            match class {
                TypeOpClass::Binary => {
                    render_varnode_opt(s, arch, fd, out)?;
                    s.push_str(" = ");
                    render_varnode_opt(s, arch, fd, in_at(0))?;
                    s.push(' ');
                    s.push_str(&operator_name(fd, op, opc)?);
                    s.push(' ');
                    render_varnode_opt(s, arch, fd, in_at(1))?;
                }
                TypeOpClass::Unary => {
                    render_varnode_opt(s, arch, fd, out)?;
                    s.push_str(" = ");
                    s.push_str(&operator_name(fd, op, opc)?);
                    s.push(' ');
                    render_varnode_opt(s, arch, fd, in_at(0))?;
                }
                TypeOpClass::Func => {
                    render_varnode_opt(s, arch, fd, out)?;
                    s.push_str(" = ");
                    s.push_str(&operator_name(fd, op, opc)?);
                    s.push('(');
                    render_varnode_opt(s, arch, fd, in_at(0))?;
                    for i in 1..num_input {
                        s.push(',');
                        render_varnode_opt(s, arch, fd, in_at(i))?;
                    }
                    s.push(')');
                }
                TypeOpClass::Base => {
                    // A remaining special op-code with no dedicated arm above
                    // (e.g. SEGMENT/CPOOLREF/NEW/CAST/PTRADD/PTRSUB).  These are
                    // not exercised by the `print raw` corpus; render the bare
                    // `out = name(ins)` form so the listing never panics.
                    render_varnode_opt(s, arch, fd, out)?;
                    s.push_str(" = ");
                    s.push_str(type_op_info(opc).get_name());
                    if num_input > 0 {
                        s.push('(');
                        render_varnode_opt(s, arch, fd, in_at(0))?;
                        for i in 1..num_input {
                            s.push(',');
                            render_varnode_opt(s, arch, fd, in_at(i))?;
                        }
                        s.push(')');
                    }
                }
            }
        }
    }
    Ok(())
}

/// C++ `TypeOpCbranch::printRaw` (typeop.cc:623).
fn render_cbranch(
    s: &mut String,
    arch: &Architecture,
    fd: &Funcdata,
    op: OpId,
) -> Result<(), String> {
    let opc = OpCode::CPUI_CBRANCH;
    s.push_str(type_op_info(opc).get_name());
    s.push(' ');
    let mut false_out: Option<BlockId> = None;
    if let Some((parent, 2)) = parent_size_out(fd, op) {
        let parent_block = fd.bblocks_ref().block(parent);
        let true_out = parent_block.get_true_out();
        false_out = Some(parent_block.get_false_out());
        print_short_header(s, fd, true_out);
    } else {
        render_varnode_opt(s, arch, fd, fd.obank().get(op).and_then(|o| o.get_in(0)))?;
    }
    s.push_str(" if (");
    render_varnode_opt(s, arch, fd, fd.obank().get(op).and_then(|o| o.get_in(1)))?;
    let flip = fd.obank().get(op).map(|o| o.is_boolean_flip()).unwrap_or(false);
    if flip {
        s.push_str(" == 0)");
    } else {
        s.push_str(" != 0)");
    }
    if let Some(fo) = false_out {
        s.push_str(" else ");
        print_short_header(s, fd, fo);
    }
    Ok(())
}

/// C++ `TypeOpCall::printRaw` / `TypeOpCallind::printRaw` (typeop.cc:669,793).
/// `with_space_after_name` reproduces the one-space difference: `CALL` emits a
/// space after the name, `CALLIND` does not.
fn render_call(
    s: &mut String,
    arch: &Architecture,
    fd: &Funcdata,
    op: OpId,
    name: &str,
    with_space_after_name: bool,
) -> Result<(), String> {
    let out = fd.obank().get(op).and_then(|o| o.get_out());
    let num_input = fd.obank().get(op).ok_or_else(|| "render_call: stale op".to_string())?.num_input();
    if out.is_some() {
        render_varnode_opt(s, arch, fd, out)?;
        s.push_str(" = ");
    }
    s.push_str(name);
    if with_space_after_name {
        s.push(' ');
    }
    render_varnode_opt(s, arch, fd, fd.obank().get(op).and_then(|o| o.get_in(0)))?;
    if num_input > 1 {
        s.push('(');
        render_varnode_opt(s, arch, fd, fd.obank().get(op).and_then(|o| o.get_in(1)))?;
        for i in 2..num_input {
            s.push(',');
            render_varnode_opt(s, arch, fd, fd.obank().get(op).and_then(|o| o.get_in(i)))?;
        }
        s.push(')');
    }
    Ok(())
}

/// C++ `op->getIn(0)->getSpaceFromConst()` for a LOAD/STORE space operand: the
/// constant whose offset is the address-space manager index.  Emits the space
/// name into the stream.
fn push_space_from_const(
    s: &mut String,
    arch: &Architecture,
    fd: &Funcdata,
    op: OpId,
    slot: int4,
) -> Result<(), String> {
    let cvn = fd
        .obank()
        .get(op)
        .and_then(|o| o.get_in(slot))
        .ok_or_else(|| "space_from_const: missing operand".to_string())?;
    let idx = fd
        .vbank()
        .get(cvn)
        .ok_or_else(|| "space_from_const: stale operand".to_string())?
        .get_offset();
    let manage = arch.manage();
    if idx >= manage.num_spaces() as uintb {
        return Err("space_from_const: index out of range".to_string());
    }
    // cast: a validated space index (< num_spaces) — matches C++ `getSpace(int4)`.
    let space = manage
        .get_space(idx as int4)
        .ok_or_else(|| "space_from_const: no such space".to_string())?;
    s.push_str(space.get_name());
    Ok(())
}

/// `(parent_block, size_out)` for the op's parent, used by the BRANCH/CBRANCH
/// arms to decide between the structured short-header form and the raw
/// destination varnode form.
fn parent_size_out(fd: &Funcdata, op: OpId) -> Option<(BlockId, int4)> {
    let parent = fd.obank().get(op)?.get_parent()?;
    let size_out = fd.bblocks_ref().block(parent).size_out();
    Some((parent, size_out))
}
