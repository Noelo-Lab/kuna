//! Image-scoped assertions about a single computed branch in the default code space.

use crate::engine::{ConsoleProgram, RawPcodeOp};
use kuna_base::address::Address;
use kuna_decomp::jumptable::{StatedJump, StatedSelector};

pub(crate) fn require_unstated(prog: &ConsoleProgram, site: &Address) -> Result<(), String> {
    if prog.arch().stated_jump_targets.contains_key(site) {
        return Err(format!(
            "computed branch at {:#x} already has stated destinations",
            site.get_offset()
        ));
    }
    Ok(())
}

pub(crate) fn install(prog: &mut ConsoleProgram, addr: u64, targets: &[u64]) -> Result<(), String> {
    let arch = prog.arch();
    let space = arch
        .manage()
        .get_default_code_space()
        .ok_or("no default code space")?;
    let units = u64::from(space.get_word_size());
    let alignment = arch.translate().get_alignment().max(1) as u64;
    let powerpc = arch.archid.starts_with("PowerPC:");
    // These SLEIGH languages include a compressed instruction set, so their
    // translator alignment alone describes only the least restrictive mode.
    let compressed_mode = if powerpc {
        Some(b"vle".as_slice())
    } else if prog.is_arm32() {
        Some(b"TMode".as_slice())
    } else if arch.archid.starts_with("MIPS:") {
        Some(b"ISA_MODE".as_slice())
    } else {
        None
    };
    let code_address = |value| {
        let offset = prog
            .input_address_offset(value)
            .map_err(|e| e.explain().to_string())?;
        let at = Address::new(space.clone(), offset);
        if offset > space.get_highest() || offset % units != 0 || !prog.entry_bytes_mapped(&at) {
            return Err(format!(
                "address {value:#x} is unmapped or outside addressable code units"
            ));
        }
        let standard_mode = compressed_mode.is_some_and(|variable| {
            arch.with_context_db(|db| db.get_variable_value(variable, &at).is_ok_and(|v| v == 0))
        });
        let alignment = if standard_mode {
            alignment.max(4)
        } else {
            alignment
        };
        if offset % alignment != 0 {
            return Err(format!(
                "address {value:#x} violates {alignment}-byte instruction alignment"
            ));
        }
        Ok(at)
    };
    let _load = arch.load_scope();
    let site = code_address(addr)?;
    if arch
        .kuna_callee_overrides
        .values()
        .any(|sites| sites.contains_key(&site))
    {
        return Err(format!(
            "an indirect-call destination is already stated at {addr:#x}"
        ));
    }
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        prog.raw_pcode_at(site.get_offset())
    }));
    let Ok(Ok((len, ops))) = decoded else {
        return Err(format!("cannot decode branch site {addr:#x}"));
    };
    let classic_ppc = powerpc
        && arch.with_context_db(|db| db.get_variable_value(b"vle", &site).unwrap_or(0) == 0);
    // SLEIGH lifts a PPC32 branch through LR as RETURN. An explicit set of
    // destinations identifies a jump even if LR's provenance is unknown.
    let link = (classic_ppc && arch.archid.contains(":32:"))
        .then(|| arch.get_register_varnode(b"LR").ok())
        .flatten()
        .as_ref()
        .map(crate::engine::RawPcodeVarnode::from);
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
    // Trace the branch input back through same-instruction COPYs. An
    // exception handler supplied by a trap userop is not a computed jump.
    let mut input = operand;
    for op in ops[..index].iter().rev().filter(|op| op.output.is_some()) {
        if op.output.as_ref() != Some(input) {
            continue;
        }
        if op.opcode == "COPY" {
            if let Some(source) = op.inputs.first() {
                input = source;
                continue;
            }
        }
        if trap_operation(prog, op) {
            return Err(format!(
                "exception transfer at {addr:#x} is not a computed jump"
            ));
        }
        break;
    }
    let unlabelable = |reason: &str| Err(format!("cannot label a switch at {addr:#x}: {reason}"));
    let selector = if arch.runtime_code_relocation {
        return unlabelable("runtime load base is unknown for this image");
    } else if powerpc {
        let ctr_or_lr = [b"CTR".as_slice(), b"LR"].iter().any(|name| {
            arch.get_register_varnode(name)
                .is_ok_and(|reg| crate::engine::RawPcodeVarnode::from(&reg) == *operand)
        });
        if ctr_or_lr && classic_ppc {
            StatedSelector::PowerPcAligned
        } else {
            return unlabelable("PowerPC branch target encoding is unresolved");
        }
    } else if arch.archid.starts_with("ARM:") {
        return unlabelable("ARM interworking requires the branch target's ISA state");
    } else if arch.archid.starts_with("AARCH64:")
        && !prog
            .disassemble_at(site.get_offset())
            .is_ok_and(|(_, mnemonic, _)| mnemonic.eq_ignore_ascii_case("br"))
    {
        return unlabelable("AArch64 branch target encoding is unresolved");
    } else {
        StatedSelector::Address
    };
    let mut dests = targets
        .iter()
        .map(|&value| code_address(value))
        .collect::<Result<Vec<_>, _>>()?;
    dests.sort_unstable();
    dests.dedup();
    if dests.is_empty() || dests.len() > arch.max_jumptable_size as usize {
        return Err(format!(
            "expected 1 to {} distinct jump destinations",
            arch.max_jumptable_size
        ));
    }
    if let Some(old) = arch.stated_jump_targets.get(&site) {
        if old.targets == dests && old.selector == selector {
            return Ok(());
        }
        return Err(format!(
            "conflicting destination sets for computed branch at {addr:#x}"
        ));
    }
    prog.arch_mut().stated_jump_targets.insert(
        site,
        StatedJump {
            targets: dests,
            selector,
        },
    );
    prog.callwide_unavailable("computed branch destinations require a new complete image proof");
    Ok(())
}

fn trap_operation(prog: &ConsoleProgram, op: &RawPcodeOp) -> bool {
    if op.opcode != "CALLOTHER" {
        return false;
    }
    let Some(id) = op.inputs.first() else {
        return false;
    };
    let Some(userop) = prog.arch().userops.get_op(id.offset as u32) else {
        return false;
    };
    matches!(
        userop.get_name(),
        b"invalidInstructionException" | b"SoftwareBreakpoint" | b"IllegalInstructionTrap"
    )
}
