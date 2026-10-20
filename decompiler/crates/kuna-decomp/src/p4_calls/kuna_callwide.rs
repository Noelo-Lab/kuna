//! Retain a big-endian register-pair return established by complete caller evidence.

use std::cell::Cell;
use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::architecture::Architecture;
use crate::context::OpId;
use crate::fspec::{FuncCallSpecs, FuncProto, ParamActive, ProtoModel};
use crate::funcdata::Funcdata;
use crate::kuna_bejoin::{BeJoinMode, WidePair};

pub mod mips;
mod proof;
pub use proof::{Declaration, DeclarationSource};

/// A complete effective executable byte range, after loader overlays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeRegion {
    pub address: u64,
    pub bytes: Vec<u8>,
}

/// An address/model proof containing no live IR identities. Only a completed
/// preflight may publish it; a restarted drive merely copies this same value.
pub struct Ledger {
    live: Cell<bool>,
    model: Rc<ProtoModel>,
    pair: WidePair,
    entries: BTreeSet<(int4, u64)>,
    declarations: BTreeSet<u64>,
}

/// Revokes existing Funcdata views when its architecture replaces or clears a proof.
#[derive(Default)]
pub struct ProofSlot(Option<Rc<Ledger>>);

/// A pre-render verdict retained while its explicit contracts are bound.
pub struct Verdict(Rc<Ledger>);

/// Project-only ABI metadata; no recovered parameter names or virtual storage escape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportPrototype {
    pub name: String,
    pub entry: u64,
    pub signed: bool,
    pub inputs: Vec<bool>,
    pub definition: bool,
}

impl ProofSlot {
    pub fn len(&self) -> usize {
        self.0
            .as_ref()
            .filter(|p| p.live.get())
            .map_or(0, |p| p.entries.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn needed_declaration(&self, entry: u64) -> bool {
        self.0
            .as_ref()
            .is_some_and(|proof| proof.live.get() && proof.declarations.contains(&entry))
    }

    pub fn verdict(&self) -> Option<Verdict> {
        self.0
            .as_ref()
            .filter(|proof| proof.live.get())
            .cloned()
            .map(Verdict)
    }

    pub fn same_verdict(&self, previous: &Verdict) -> bool {
        self.0.as_ref().is_some_and(|proof| {
            proof.live.get()
                && Rc::ptr_eq(&proof.model, &previous.0.model)
                && proof.entries == previous.0.entries
                && proof.declarations == previous.0.declarations
        })
    }

    pub fn clear(&mut self) {
        self.revoke();
        self.0 = None;
    }

    pub(crate) fn revoke(&self) {
        if let Some(proof) = &self.0 {
            proof.live.set(false);
        }
    }
}

impl Drop for ProofSlot {
    fn drop(&mut self) {
        self.clear();
    }
}

/// Copy previously frozen facts into a fresh drive without discovering a fact.
pub fn seed(arch: &Architecture, data: &mut Funcdata) {
    let proof = if arch.call_wide && arch.be_join == BeJoinMode::Evidence {
        arch.kuna_callwide
            .0
            .as_ref()
            .filter(|proof| {
                arch.default_fp()
                    .is_some_and(|model| Rc::ptr_eq(model, &proof.model))
            })
            .cloned()
    } else {
        None
    };
    data.kuna_set_callwide(proof);
}

/// The same verdict is read by the producer's own return and every direct call.
pub fn proven(data: &Funcdata, entry: &Address) -> bool {
    if !data.get_arch().call_wide || crate::kuna_bejoin::mode(data) != BeJoinMode::Evidence {
        return false;
    }
    let Some(space) = entry.get_space() else {
        return false;
    };
    data.kuna_callwide().is_some_and(|proof| {
        proof.live.get()
            && proof
                .entries
                .contains(&(space.get_index(), entry.get_offset()))
    })
}

/// Preserve actual finalized CALL operands without recovering or regrouping inputs.
pub fn call_prototype(
    data: &Funcdata,
    call: &FuncCallSpecs,
    name: &str,
) -> Result<Option<(FuncProto, ExportPrototype)>, String> {
    use crate::dtype::{type_metatype, Datatype};
    use crate::fspec::ParameterPieces;

    if !proven(data, call.get_entry_address()) {
        return Ok(None);
    }
    let invalid = || {
        format!(
            "callwide: cannot declare the exact call contract at {}",
            call.get_entry_address().get_offset()
        )
    };
    let proof = data.kuna_callwide().ok_or_else(invalid)?;
    let proto = call.proto();
    let op = data.obank().get(call.get_op()).ok_or_else(invalid)?;
    let count = op.num_input() - 1;
    if op.is_dead()
        || op.code() != kuna_num::opcodes::OpCode::CPUI_CALL
        || !proto.has_matching_model(&proof.model)
        || !proto.has_store()
        || proto.is_dotdotdot()
        || proto.is_paramshift_applied()
        || call.get_paramshift() != 0
        || call.format_arity().is_some()
        || !(0..=4).contains(&count)
        || call.final_input_storage().len() != count as usize
        || (proto.num_params() != 0 || proto.is_input_locked()) && proto.num_params() != count
    {
        return Err(invalid());
    }
    let mut snapshot = FuncProto::new();
    snapshot.copy(proto);
    snapshot.set_internal(
        proof.model.clone(),
        Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)),
    );
    for slot in 0..count {
        let value = op
            .get_in(slot + 1)
            .and_then(|id| data.vbank().get(id))
            .ok_or_else(invalid)?;
        let ty = value.get_type_read_facing(call.get_op());
        let (address, size) = &call.final_input_storage()[slot as usize];
        if value.get_size() != 4 || *size != 4 || export_integer(ty, 4).is_none() {
            return Err(invalid());
        }
        if proto.num_params() != 0 {
            let parameter = proto.get_param(slot).ok_or_else(invalid)?;
            if parameter.get_size() != 4
                || parameter.is_hidden_return()
                || parameter.is_indirect_storage()
                || parameter.is_this_pointer()
                || !parameter
                    .get_type()
                    .is_some_and(|ty| export_integer(ty, 4).is_some())
                || !export_same_address(&parameter.get_address(), address)
            {
                return Err(invalid());
            }
        }
        snapshot.set_param(
            slot,
            "",
            &ParameterPieces {
                addr: address.clone(),
                type_: Some(ty.clone()),
                flags: 0,
            },
        );
    }
    let output = if let Some(id) = op.get_out() {
        let value = data.vbank().get(id).ok_or_else(invalid)?;
        if value.get_size() != 8 || export_return_integer(value.get_type_def_facing()).is_none() {
            return Err(invalid());
        }
        ParameterPieces {
            addr: value.get_addr().clone(),
            type_: Some(value.get_type_def_facing().clone()),
            flags: 0,
        }
    } else {
        let address = proof.pair.hi.0.clone();
        let Some(space) = address.get_space() else {
            return Err(invalid());
        };
        if !space.is_big_endian()
            || proof.pair.hi.1 != 4
            || proof.pair.lo.1 != 4
            || address.get_offset().checked_add(4) != Some(proof.pair.lo.0.get_offset())
            || !proof
                .pair
                .lo
                .0
                .get_space()
                .is_some_and(|low| Rc::ptr_eq(space, low))
        {
            return Err(invalid());
        }
        let ty = if proto.get_output().get_size() == 0 {
            Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN))
        } else {
            proto.get_output_type().ok_or_else(invalid)?.clone()
        };
        ParameterPieces {
            addr: address,
            type_: Some(ty),
            flags: 0,
        }
    };
    let signed =
        export_return_integer(output.type_.as_ref().ok_or_else(invalid)?).ok_or_else(invalid)?;
    if proto.get_output().is_hidden_return()
        || proto.get_output().is_indirect_storage()
        || proto.get_output().get_size() != 0
            && (proto.get_output().get_size() != 8
                || proto
                    .get_output_type()
                    .and_then(|ty| export_return_integer(ty))
                    != Some(signed)
                || !export_storage(data, proto, proof))
    {
        return Err(invalid());
    }
    snapshot.set_output(&output);
    if !export_storage(data, &snapshot, proof) {
        return Err(invalid());
    }
    let contract = export_contract(
        &snapshot,
        name,
        call.get_entry_address().get_offset(),
        false,
    )
    .ok_or_else(invalid)?;
    Ok(Some((snapshot, contract)))
}

/// C already represents a canonical unknown eight-byte scalar as unsigned.
fn export_return_integer(ty: &crate::dtype::Datatype) -> Option<bool> {
    use crate::dtype::{type_metatype, DatatypeKind};
    export_integer(ty, 8).or_else(|| {
        (ty.get_size() == 8
            && ty.get_metatype() == type_metatype::TYPE_UNKNOWN
            && matches!(ty.kind, DatatypeKind::Unknown | DatatypeKind::Base)
            && matches!(ty.get_name(), "" | "xunknown8" | "undefined8")
            && ty.get_typedef().is_none()
            && !ty.is_char_print()
            && !ty.is_enum_type()
            && !ty.is_variable_length()
            && crate::kuna_lang::OutLang::C
                .speller()
                .unknown_base(8, false, false)
                .as_deref()
                == Some("unsigned long long"))
        .then_some(false)
    })
}

fn export_same_address(left: &Address, right: &Address) -> bool {
    left.get_offset() == right.get_offset()
        && left
            .get_space()
            .zip(right.get_space())
            .is_some_and(|(left, right)| Rc::ptr_eq(left, right))
}

fn export_storage(data: &Funcdata, proto: &FuncProto, proof: &Ledger) -> bool {
    use kuna_base::space::spacetype;
    let output = proto.get_output().get_address();
    let Some(space) = output.get_space() else {
        return false;
    };
    let output_matches = if space.get_type() == spacetype::IPTR_JOIN {
        let manager = data.get_arch().manage();
        manager
            .get_join_space()
            .is_some_and(|join| Rc::ptr_eq(space, join))
            && manager.find_join(output.get_offset()).is_ok_and(|join| {
                !join.is_float_extension()
                    && join.num_pieces() == 2
                    && [&proof.pair.hi, &proof.pair.lo].iter().enumerate().all(
                        |(i, (address, size))| {
                            let piece = join.get_piece(i as i32);
                            u32::try_from(*size) == Ok(piece.size)
                                && piece
                                    .space
                                    .as_ref()
                                    .zip(address.get_space())
                                    .is_some_and(|(left, right)| Rc::ptr_eq(left, right))
                                && piece.offset == address.get_offset()
                        },
                    )
            })
    } else {
        space.is_big_endian()
            && export_same_address(&output, &proof.pair.hi.0)
            && proof.pair.hi.1 == 4
            && proof.pair.lo.1 == 4
            && proof.pair.hi.0.get_offset().checked_add(4) == Some(proof.pair.lo.0.get_offset())
            && proof
                .pair
                .lo
                .0
                .get_space()
                .is_some_and(|low| Rc::ptr_eq(space, low))
    };
    if !output_matches || proof.model.input_opt().is_none() {
        return false;
    }
    let entries = proof.model.input().get_entry();
    if entries.len() > 64 {
        return false;
    }
    let words: Vec<_> = entries
        .iter()
        .filter(|entry| {
            entry.get_type() == crate::dtype::type_class::TYPECLASS_GENERAL
                && entry.get_join_record().is_none()
                && entry.is_exclusion()
                && entry.get_space().get_type() == spacetype::IPTR_PROCESSOR
                && entry.get_size() == 4
                && entry.get_min_size() <= 4
                && proof
                    .pair
                    .hi
                    .0
                    .get_space()
                    .is_some_and(|space| Rc::ptr_eq(entry.get_space(), space))
        })
        .collect();
    if words.len() > 4
        || words
            .windows(2)
            .any(|pair| pair[0].get_base().checked_add(4) != Some(pair[1].get_base()))
    {
        return false;
    }
    (0..proto.num_params()).all(|i| {
        let Some(parameter) = proto.get_param(i) else {
            return false;
        };
        let address = parameter.get_address();
        words.get(i as usize).is_some_and(|entry| {
            address.get_offset() == entry.get_base()
                && address
                    .get_space()
                    .is_some_and(|space| Rc::ptr_eq(space, entry.get_space()))
        })
    })
}

fn export_integer(ty: &crate::dtype::Datatype, size: int4) -> Option<bool> {
    use crate::dtype::type_metatype;
    if ty.get_size() != size {
        return None;
    }
    match ty.get_metatype() {
        type_metatype::TYPE_INT => Some(true),
        type_metatype::TYPE_UINT => Some(false),
        _ => None,
    }
}

fn export_contract(
    proto: &FuncProto,
    name: &str,
    entry: u64,
    definition: bool,
) -> Option<ExportPrototype> {
    if !proto.has_store()
        || !(0..=32).contains(&proto.num_params())
        || proto.is_dotdotdot()
        || proto.is_paramshift_applied()
        || proto.get_output().get_size() != 8
        || proto.get_output().is_hidden_return()
        || proto.get_output().is_indirect_storage()
    {
        return None;
    }
    let signed = export_return_integer(proto.get_output_type()?)?;
    let inputs = (0..proto.num_params())
        .map(|slot| {
            let parameter = proto.get_param(slot)?;
            if parameter.get_size() != 4
                || parameter.is_this_pointer()
                || parameter.is_hidden_return()
                || parameter.is_indirect_storage()
            {
                return None;
            }
            export_integer(parameter.get_type()?, 4)
        })
        .collect::<Option<Vec<_>>>()?;
    Some(ExportPrototype {
        name: name.into(),
        entry,
        signed,
        inputs,
        definition,
    })
}

/// Compare selected producer definitions with the declarations their callers need.
pub fn definition_prototype(
    data: &Funcdata,
    name: &str,
) -> Result<Option<ExportPrototype>, String> {
    if !proven(data, data.get_address()) {
        return Ok(None);
    }
    let valid = data.kuna_callwide().is_some_and(|proof| {
        data.get_func_proto().has_matching_model(&proof.model)
            && export_storage(data, data.get_func_proto(), proof)
    });
    let contract = valid
        .then(|| {
            export_contract(
                data.get_func_proto(),
                name,
                data.get_address().get_offset(),
                true,
            )
        })
        .flatten();
    contract
        .map(Some)
        .ok_or_else(|| format!("callwide: cannot export the exact producer contract for {name}"))
}

/// Retain only the exact, still-present output pair proved before recovery.
/// All checks finish before either trial is changed.
pub fn retain_pair(data: &Funcdata, active: &mut ParamActive, returns: &[OpId]) {
    if data.get_func_proto().is_output_locked() || !proven(data, data.get_address()) {
        return;
    }
    let Some(proof) = data.kuna_callwide() else {
        return;
    };
    if !data.get_func_proto().has_matching_model(&proof.model) {
        return;
    }
    let mut indices = Vec::new();
    for (address, size) in [&proof.pair.hi, &proof.pair.lo] {
        let matching: Vec<int4> = (0..active.get_num_trials())
            .filter(|&i| {
                let trial = active.get_trial(i);
                trial.get_address() == address
                    && trial.get_size() == *size
                    && trial.get_slot() > 0
                    && !trial.is_unref()
            })
            .collect();
        let [index] = matching[..] else { return };
        indices.push(index);
    }
    if indices[0] == indices[1] {
        return;
    }
    let mut live = 0;
    for &op in returns {
        let Some(ret) = data.obank().get(op) else {
            return;
        };
        if ret.is_dead() || ret.get_halt_type() != 0 || crate::kuna_bejoin::never_reached(data, op)
        {
            continue;
        }
        live += 1;
        for &index in &indices {
            let trial = active.get_trial(index);
            let Some(value) = ret
                .get_in(trial.get_slot())
                .and_then(|vn| data.vbank().get(vn))
            else {
                return;
            };
            if value.get_addr() != trial.get_address()
                || value.get_size() != trial.get_size()
                || !value.is_written()
            {
                return;
            }
        }
    }
    if live == 0 {
        return;
    }
    for index in indices {
        active.get_trial_mut(index).mark_active();
    }
}

/// Reconcile the entire sealed MIPS image before granting its return-mode proof.
pub fn publish_mips(
    arch: &mut Architecture,
    regions: &[CodeRegion],
    functions: &[(u64, u64)],
    entry: u64,
    observed_calls: &std::collections::BTreeSet<(u64, u64)>,
    closed_sites: &std::collections::BTreeMap<u64, Vec<u64>>,
    declarations: &[Declaration],
) -> usize {
    arch.kuna_callwide.clear();
    if !arch.call_wide
        || arch.be_join != BeJoinMode::Evidence
        || arch.archid != "MIPS:BE:32:default:default"
    {
        return 0;
    }
    let Some(completed) = mips::closed_image(entry, regions, functions, observed_calls) else {
        return 0;
    };
    if closed_sites
        .iter()
        .any(|(entry, sites)| completed.get(entry) != Some(sites))
    {
        return 0;
    }
    arch.kuna_callwide.0 = proof::prove(
        arch,
        regions,
        functions,
        closed_sites,
        declarations,
        proof::Authority::SealedMips,
    );
    arch.kuna_callwide.len()
}

#[cfg(test)]
#[path = "kuna_callwide/tests.rs"]
mod tests;
