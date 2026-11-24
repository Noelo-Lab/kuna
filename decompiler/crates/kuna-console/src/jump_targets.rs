//! Image-scoped assertions about a single computed branch in the default code space.

use crate::engine::{ConsoleProgram, PendingFunction, RawPcodeOp, RawPcodeVarnode};
use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::jumptable::{StatedJump, StatedSelector};
use std::rc::Rc;

pub(crate) fn require_unstated(prog: &ConsoleProgram, site: &Address) -> Result<(), String> {
    if prog.arch().stated_jump_targets.contains_key(site) {
        return Err(format!(
            "computed branch at {:#x} already has stated destinations",
            site.get_offset()
        ));
    }
    Ok(())
}

/// Validate the complete destination set of the computed branch at `addr`
/// and install it for the image. Every check runs first, so a rejected set
/// leaves the site as it was.
pub(crate) fn install(prog: &mut ConsoleProgram, addr: u64, targets: &[u64]) -> Result<(), String> {
    let arch = prog.arch();
    let space = arch
        .manage()
        .get_default_code_space()
        .ok_or("no default code space")?
        .clone();
    let _load = arch.load_scope();
    let site = code_address(prog, &space, addr)?;
    if arch
        .kuna_callee_overrides
        .values()
        .any(|sites| sites.contains_key(&site))
    {
        return Err(format!(
            "an indirect-call destination is already stated at {addr:#x}"
        ));
    }
    let (selector, value_bytes) = branch_selector(prog, &site, addr)?;
    let mut dests = targets
        .iter()
        .map(|&value| Ok((code_address(prog, &space, value)?, value)))
        .collect::<Result<Vec<_>, String>>()?;
    dests.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    dests.dedup_by(|a, b| a.0 == b.0);
    if dests.is_empty() || dests.len() > arch.max_jumptable_size as usize {
        return Err(format!(
            "expected 1 to {} distinct jump destinations",
            arch.max_jumptable_size
        ));
    }
    let units = u64::from(space.get_word_size());
    if let Some((_, value)) = dests
        .iter()
        .find(|(at, _)| value_bytes < 8 && (at.get_offset() / units) >> (8 * value_bytes) != 0)
    {
        return Err(format!(
            "cannot label a switch at {addr:#x}: destination {value:#x} does not fit the {value_bytes}-byte value the jump takes"
        ));
    }
    let targets: Vec<Address> = dests.iter().map(|(at, _)| at.clone()).collect();
    if let Some(old) = arch.stated_jump_targets.get(&site) {
        if old.targets == targets && old.selector == selector {
            return Ok(());
        }
        return Err(format!(
            "conflicting destination sets for computed branch at {addr:#x}"
        ));
    }
    let entries: Vec<(u64, u64)> = dests.iter().map(|(at, value)| (at.get_offset(), *value)).collect();
    if let Some((entry, value)) = leaving_destination(prog, site.get_offset(), &entries, None) {
        let at = Address::new(space.clone(), entry);
        let name = arch
            .symboltab
            .function_display_name_across_scopes(&at)
            .unwrap_or_else(|| format!("{value:#x}"));
        return Err(format!(
            "destination {value:#x} is the entry of function {name}, so the jump leaves the function \
             (state a single tail-call destination with callee, or declare the extent of a function \
             that holds both)"
        ));
    }
    prog.arch_mut().stated_jump_targets.insert(site, StatedJump { targets, selector });
    prog.callwide_unavailable("computed branch destinations require a new complete image proof");
    Ok(())
}

/// The mapped code address `value` names, in the space's addressable units and
/// aligned for its decode mode. PowerPC VLE, ARM Thumb and MIPS16/microMIPS
/// make the translator's alignment describe only the least restrictive mode,
/// so an address whose context selects the standard mode must be 4-aligned.
fn code_address(prog: &ConsoleProgram, space: &Rc<AddrSpace>, value: u64) -> Result<Address, String> {
    let arch = prog.arch();
    let offset = prog
        .input_address_offset(value)
        .map_err(|e| e.explain().to_string())?;
    let at = Address::new(space.clone(), offset);
    let units = u64::from(space.get_word_size());
    if offset > space.get_highest() || offset % units != 0 || !prog.entry_bytes_mapped(&at) {
        return Err(format!(
            "address {value:#x} is unmapped or outside addressable code units"
        ));
    }
    let standard_mode = compressed_mode_variable(prog).is_some_and(|variable| {
        arch.with_context_db(|db| db.get_variable_value(variable, &at).is_ok_and(|v| v == 0))
    });
    let alignment = arch.translate().get_alignment().max(1) as u64;
    let alignment = if standard_mode { alignment.max(4) } else { alignment };
    if offset % alignment != 0 {
        return Err(format!(
            "address {value:#x} violates {alignment}-byte instruction alignment"
        ));
    }
    Ok(at)
}

/// The context variable that selects a language's compressed instruction set.
fn compressed_mode_variable(prog: &ConsoleProgram) -> Option<&'static [u8]> {
    let archid = &prog.arch().archid;
    if archid.starts_with("PowerPC:") {
        Some(b"vle")
    } else if prog.is_arm32() {
        Some(b"TMode")
    } else if archid.starts_with("MIPS:") {
        Some(b"ISA_MODE")
    } else {
        None
    }
}

/// The one computed branch at `site`, how its operand labels the cases, and
/// the fewest bytes that operand passed through in its instruction, which
/// bound every destination it can reach.
/// SLEIGH lifts a classic PPC32 branch through LR as RETURN, so a stated set
/// identifies that jump even when LR's provenance is unknown. A site whose
/// operand cannot label a case is rejected with the missing fact: an image
/// that may run elsewhere, a segmented destination, ARM interworking, or an
/// unresolved PowerPC or AArch64 target encoding. Every computed branch of a
/// 16-bit x86 language is segmented: a near target is an offset in CS and a
/// far target's segment is loaded with it, even where SLEIGH lifts the branch
/// without the segment operation.
fn branch_selector(prog: &ConsoleProgram, site: &Address, addr: u64) -> Result<(StatedSelector, u32), String> {
    let arch = prog.arch();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        prog.raw_pcode_at(site.get_offset())
    }));
    let Ok(Ok((len, ops))) = decoded else {
        return Err(format!("cannot decode branch site {addr:#x}"));
    };
    let powerpc = arch.archid.starts_with("PowerPC:");
    let classic_ppc = powerpc
        && arch.with_context_db(|db| db.get_variable_value(b"vle", site).unwrap_or(0) == 0);
    let link = (classic_ppc && arch.archid.contains(":32:"))
        .then(|| arch.get_register_varnode(b"LR").ok())
        .flatten()
        .as_ref()
        .map(RawPcodeVarnode::from);
    let mut branches = ops.iter().enumerate().filter(|(_, op)| {
        op.opcode == "BRANCHIND"
            || (op.opcode == "RETURN"
                && link
                    .as_ref()
                    .is_some_and(|lr| op.inputs.first() == Some(lr)))
    });
    let Some((index, branch)) = branches.next().filter(|_| len > 0) else {
        return Err(format!("expected one computed branch at {addr:#x}"));
    };
    if branches.next().is_some() {
        return Err(format!("expected one computed branch at {addr:#x}"));
    }
    let operand = branch
        .inputs
        .first()
        .ok_or("missing computed branch operand")?;
    let (producer, value_bytes) = operand_source(&ops[..index], operand);
    let producer = producer.and_then(|op| user_operation(prog, op));
    if producer.is_some_and(|userop| TRAPS.contains(&userop.get_name())) {
        return Err(format!(
            "exception transfer at {addr:#x} is not a computed jump"
        ));
    }
    let unlabelable = |reason: &str| Err(format!("cannot label a switch at {addr:#x}: {reason}"));
    if arch.runtime_code_relocation {
        return unlabelable("runtime load base is unknown for this image");
    }
    if arch.archid.starts_with("x86:LE:16:")
        || producer.is_some_and(|userop| is_segment_op(prog, userop.get_index()))
    {
        return unlabelable("a segmented destination is printed as an offset in its code segment");
    }
    if powerpc {
        let ctr_or_lr = [b"CTR".as_slice(), b"LR"].iter().any(|name| {
            arch.get_register_varnode(name)
                .is_ok_and(|reg| RawPcodeVarnode::from(&reg) == *operand)
        });
        if ctr_or_lr && classic_ppc {
            return Ok((StatedSelector::PowerPcAligned, value_bytes));
        }
        return unlabelable("PowerPC branch target encoding is unresolved");
    }
    if arch.archid.starts_with("ARM:") {
        return unlabelable("ARM interworking requires the branch target's ISA state");
    }
    if arch.archid.starts_with("AARCH64:")
        && !prog
            .disassemble_at(site.get_offset())
            .is_ok_and(|(_, mnemonic, _)| mnemonic.eq_ignore_ascii_case("br"))
    {
        return unlabelable("AArch64 branch target encoding is unresolved");
    }
    Ok((StatedSelector::Address, value_bytes))
}

/// User operations whose result is an exception handler, not a jump target.
const TRAPS: [&[u8]; 3] = [b"invalidInstructionException", b"SoftwareBreakpoint", b"IllegalInstructionTrap"];

/// The same-instruction operation that computes the branch input, seen
/// through the copies, extensions and truncations between them, and the
/// fewest bytes the input passed through on the way.
fn operand_source<'a>(before: &'a [RawPcodeOp], operand: &RawPcodeVarnode) -> (Option<&'a RawPcodeOp>, u32) {
    let mut input = operand;
    let mut bytes = operand.size;
    for op in before.iter().rev() {
        if op.output.as_ref() != Some(input) {
            continue;
        }
        let source = match (op.opcode, op.inputs.first()) {
            ("COPY" | "INT_SEXT" | "SUBPIECE", Some(source)) => source,
            ("INT_ZEXT", Some(source)) => {
                bytes = bytes.min(source.size);
                source
            }
            _ => return (Some(op), bytes),
        };
        input = source;
    }
    (None, bytes)
}

fn user_operation<'a>(prog: &'a ConsoleProgram, op: &RawPcodeOp) -> Option<&'a kuna_decomp::userop::UserPcodeOp> {
    if op.opcode != "CALLOTHER" {
        return None;
    }
    prog.arch().userops.get_op(op.inputs.first()?.offset as u32)
}

/// Is user operation `index` the segment operation of an address space? Its
/// result is printed as the near offset, which a linear label cannot match.
fn is_segment_op(prog: &ConsoleProgram, index: i32) -> bool {
    let arch = prog.arch();
    (0..arch.manage().num_spaces())
        .any(|space| arch.userops.get_segment_op(space).is_some_and(|op| op.get_index() == index))
}

/// The first `(entry, value)` destination that enters another function the
/// loader's symbols or the caller declare, counting `pending` as made: that
/// jump leaves the function, so it is no switch case. A destination inside
/// the known extent of a declared function that holds the site stays in it,
/// unless the caller declared the destination's function and only the
/// symbol table states that extent. The holder's own entry and a part split
/// off it (`f.cold`) always stay. An entry without a known extent holds no
/// site, and entries discovery alone found are not evidence, because
/// discovery also names address-taken case blocks as functions.
fn leaving_destination(
    prog: &ConsoleProgram,
    site: u64,
    dests: &[(u64, u64)],
    pending: Option<PendingFunction>,
) -> Option<(u64, u64)> {
    let holders = prog.functions_holding(site, pending);
    dests.iter().copied().find(|&(entry, _)| {
        let Some(declarer) = prog.function_declarer(entry, pending) else {
            return false;
        };
        !holders.iter().any(|&(start, size, extent)| {
            prog.same_function(entry, start)
                || (entry >= start && entry - start < size && extent >= declarer)
        })
    })
}

/// Refuse a function declaration under which a destination of an installed
/// computed branch would enter another function. The later fact conflicts
/// with the earlier one, as a second destination set for one site does.
pub(crate) fn require_declaration_keeps_cases(prog: &ConsoleProgram, pending: PendingFunction) -> Result<(), String> {
    for (site, jump) in &prog.arch().stated_jump_targets {
        let entries: Vec<(u64, u64)> = jump
            .targets
            .iter()
            .map(|at| (at.get_offset(), prog.output_code_offset(at.get_offset())))
            .collect();
        if let Some((_, value)) = leaving_destination(prog, site.get_offset(), &entries, Some(pending)) {
            return Err(format!(
                "destination {value:#x} of the computed branch stated at {:#x} would enter another function",
                prog.output_code_offset(site.get_offset())
            ));
        }
    }
    Ok(())
}
