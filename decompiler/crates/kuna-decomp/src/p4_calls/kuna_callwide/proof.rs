use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpaceManager, JoinRecord, JoinSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::architecture::Architecture;
use crate::dtype::type_metatype;
use crate::fspec::{ParameterPieces, ProtoModel, PrototypePieces};
use crate::kuna_bejoin::WidePair;

use super::{CodeRegion, Ledger};

const MAX_SITES: usize = 256;
const MAX_IMAGE_OPS: usize = 1_000_000;
const MAX_PRODUCER_INSNS: usize = 512;
const MAX_PRODUCER_OPS: usize = 20_000;
const MAX_FORWARD_INSNS: usize = 32;
const MAX_FORWARD_OPS: usize = 1_024;
const MAX_STORAGE: u32 = 32;
const MAX_DECLARATIONS: usize = 16_384;
const MAX_PARAMETERS: usize = 32;
const MAX_DECLARATION_BYTES: usize = 1_048_576;

type Cell = (i32, u64);

pub(super) enum Authority {
    SealedMips,
}

/// Positive provenance is supplied by declaration ingestion, never inferred locks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationSource {
    Assertion,
    Parsed,
    Dwarf,
}

#[derive(Clone)]
pub struct Declaration {
    pub entry: u64,
    pub pieces: PrototypePieces,
    pub model: Rc<ProtoModel>,
    pub source: DeclarationSource,
}

#[derive(Default)]
struct Collect(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Collect {
    fn dump(
        &mut self,
        _at: &Address,
        opcode: OpCode,
        out: Option<&VarnodeData>,
        inputs: &[VarnodeData],
    ) {
        self.0.push((opcode, out.cloned(), inputs.to_vec()));
    }
}

struct Budget(usize);

impl Budget {
    fn spend(&mut self, n: usize) -> Option<()> {
        match self.0.checked_sub(n) {
            Some(left) => {
                self.0 = left;
                Some(())
            }
            None => {
                self.0 = 0;
                None
            }
        }
    }
}

struct Image<'a> {
    arch: &'a Architecture,
    regions: &'a [CodeRegion],
    functions: BTreeMap<u64, u64>,
    declarations: BTreeMap<u64, &'a Declaration>,
    used_declarations: BTreeSet<u64>,
    pair: WidePair,
    model: Rc<ProtoModel>,
    budget: Budget,
    authority: Authority,
    manager: AddrSpaceManager,
}

/// Parameter assignment may allocate virtual joins. Keep these proof-local;
/// compiled model joins still refer to the original, read-only join space.
fn proof_manager(source: &AddrSpaceManager) -> Option<AddrSpaceManager> {
    if source.num_spaces() > 64 || source.get_join_space().is_none() {
        return None;
    }
    let mut manager = AddrSpaceManager::new();
    for index in 0..source.num_spaces() {
        if let Some(space) = source.get_space(index) {
            let space = if space.get_type() == spacetype::IPTR_JOIN {
                Rc::new(JoinSpace::new(space.get_index(), space.is_big_endian()))
            } else {
                space.clone()
            };
            manager.insert_space(space).ok()?;
        }
    }
    manager
        .set_default_code_space(source.get_default_code_space()?.get_index())
        .ok()?;
    manager
        .set_default_data_space(source.get_default_data_space()?.get_index())
        .ok()?;
    if let Some(lookup) = source.register_lookup() {
        manager.set_register_lookup(lookup.clone());
    }
    Some(manager)
}

fn join(
    original: &AddrSpaceManager,
    local: &AddrSpaceManager,
    address: &Address,
) -> Option<Rc<JoinRecord>> {
    let space = address.get_space()?;
    if Rc::ptr_eq(space, local.get_join_space()?) {
        local.find_join(address.get_offset()).ok()
    } else if Rc::ptr_eq(space, original.get_join_space()?) {
        original.find_join(address.get_offset()).ok()
    } else {
        None
    }
}

fn same(a: &VarnodeData, b: &VarnodeData) -> bool {
    a.size == b.size
        && a.offset == b.offset
        && a.space
            .as_ref()
            .zip(b.space.as_ref())
            .is_some_and(|(a, b)| Rc::ptr_eq(a, b))
}

fn constant(v: &VarnodeData, value: u64, size: u32) -> bool {
    v.size == size
        && v.offset == value
        && v.space
            .as_ref()
            .is_some_and(|space| space.get_type() == spacetype::IPTR_CONSTANT)
}

/// The sealed control proof makes every ordinary return address an aligned
/// internal JAL continuation. Only its exact JXWritePC mode operation is zero.
fn return_mode(
    ops: &Collect,
    ra: &VarnodeData,
    pc: &VarnodeData,
    mode: &VarnodeData,
    userop: u64,
) -> bool {
    let Some(
        [(OpCode::CPUI_INT_AND, Some(bit), mask), (OpCode::CPUI_INT_NOTEQUAL, Some(output), compare), (OpCode::CPUI_CALLOTHER, None, arguments), (OpCode::CPUI_INT_2COMP, Some(alignment), two), (OpCode::CPUI_INT_AND, Some(target), aligned), (OpCode::CPUI_COPY, Some(destination), copy), rest @ ..],
    ) = ops.0.get(..)
    else {
        return false;
    };
    ra.size == 4
        && pc.size == 4
        && mode.size == 1
        && bit.size == 4
        && bit
            .space
            .as_ref()
            .is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL)
        && mask.len() == 2
        && same(&mask[0], ra)
        && constant(&mask[1], 1, 4)
        && same(output, mode)
        && compare.len() == 2
        && same(&compare[0], bit)
        && constant(&compare[1], 0, 4)
        && arguments.len() == 2
        && constant(&arguments[0], userop, 4)
        && same(&arguments[1], mode)
        && alignment.size == 4
        && alignment
            .space
            .as_ref()
            .is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL)
        && two.len() == 1
        && constant(&two[0], 2, 4)
        && same(target, alignment)
        && aligned.len() == 2
        && same(&aligned[0], alignment)
        && same(&aligned[1], ra)
        && same(destination, pc)
        && copy.len() == 1
        && same(&copy[0], target)
        && rest.last().is_some_and(|(op, output, inputs)| {
            *op == OpCode::CPUI_RETURN
                && output.is_none()
                && inputs.len() == 1
                && same(&inputs[0], pc)
        })
        && rest.iter().enumerate().all(|(i, (op, _, _))| {
            (i + 1 == rest.len() && *op == OpCode::CPUI_RETURN)
                || *op == OpCode::CPUI_COPY
                || *op == OpCode::CPUI_PIECE
                || *op == OpCode::CPUI_SUBPIECE
                || *op == OpCode::CPUI_LOAD
                || *op == OpCode::CPUI_STORE
                || pure(*op)
        })
}

fn cell(v: &VarnodeData, physical: u32) -> Option<Cell> {
    if v.size == 0 || v.size > MAX_STORAGE || physical >= v.size {
        return None;
    }
    Some((
        v.space.as_ref()?.get_index(),
        v.offset.checked_add(physical as u64)?,
    ))
}

fn numeric_cell(v: &VarnodeData, numeric: u32) -> Option<Cell> {
    let physical = if v.space.as_ref()?.is_big_endian() {
        v.size.checked_sub(numeric + 1)?
    } else {
        numeric
    };
    cell(v, physical)
}

fn storage(address: &Address, size: i32) -> Option<VarnodeData> {
    let size = u32::try_from(size).ok()?;
    if size == 0 || size > MAX_STORAGE {
        return None;
    }
    Some(VarnodeData {
        space: address.get_space().cloned(),
        offset: address.get_offset(),
        size,
    })
}

fn pure(op: OpCode) -> bool {
    matches!(
        op,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_POPCOUNT
            | OpCode::CPUI_LZCOUNT
    )
}

fn integer(pieces: &ParameterPieces) -> Option<u32> {
    let datatype = pieces.type_.as_ref()?;
    integer_width(datatype.get_metatype(), datatype.get_size())
}

fn integer_width(kind: type_metatype, size: i32) -> Option<u32> {
    match (kind, size) {
        (type_metatype::TYPE_BOOL, 1) => Some(1),
        (type_metatype::TYPE_INT | type_metatype::TYPE_UINT, 1 | 2 | 4 | 8) => Some(size as u32),
        _ => None,
    }
}

fn integer_contract(pieces: &PrototypePieces) -> bool {
    pieces.outtype.as_ref().is_some_and(|output| {
        (output.get_metatype() == type_metatype::TYPE_VOID && output.get_size() == 0)
            || integer_width(output.get_metatype(), output.get_size()).is_some()
    }) && pieces
        .intypes
        .iter()
        .all(|ty| integer_width(ty.get_metatype(), ty.get_size()).is_some())
}

fn bounded(v: &VarnodeData) -> bool {
    v.space.is_some()
        && v.size != 0
        && v.size <= MAX_STORAGE
        && v.offset.checked_add(u64::from(v.size) - 1).is_some()
}

fn transport(
    opcode: OpCode,
    inputs: &[VarnodeData],
    bytes: &[Vec<u8>],
    size: u32,
) -> Option<Vec<u8>> {
    if size == 0
        || size > MAX_STORAGE
        || inputs.len() != bytes.len()
        || inputs
            .iter()
            .zip(bytes)
            .any(|(v, b)| !bounded(v) || v.size as usize != b.len())
    {
        return None;
    }
    let result: Vec<u8> = match opcode {
        OpCode::CPUI_COPY if inputs.len() == 1 => bytes[0].clone(),
        OpCode::CPUI_PIECE if inputs.len() == 2 => {
            bytes[1].iter().chain(bytes[0].iter()).copied().collect()
        }
        OpCode::CPUI_SUBPIECE
            if inputs.len() == 2
                && inputs[1].space.as_ref()?.get_type() == spacetype::IPTR_CONSTANT =>
        {
            let start = usize::try_from(inputs[1].offset).ok()?;
            bytes[0]
                .get(start..start.checked_add(size as usize)?)?
                .to_vec()
        }
        _ => return None,
    };
    (result.len() == size as usize).then_some(result)
}

impl Image<'_> {
    fn instruction(&mut self, at: u64, end: u64) -> Option<(u64, Collect)> {
        let space = self.arch.manage().get_default_code_space()?.clone();
        let address = Address::new(space, at);
        let mut collect = Collect::default();
        let len = u64::try_from(
            self.arch
                .translate()
                .one_instruction(&mut collect, &address)
                .ok()?,
        )
        .ok()?;
        let next = at.checked_add(len)?;
        if len == 0 || next > end || collect.0.is_empty() || collect.0.len() > MAX_FORWARD_OPS {
            return None;
        }
        if !self.regions.iter().any(|r| {
            r.address <= at
                && r.address
                    .checked_add(r.bytes.len() as u64)
                    .is_some_and(|last| next <= last)
        }) {
            return None;
        }
        self.budget.spend(collect.0.len())?;
        if matches!(self.authority, Authority::SealedMips)
            && self.arch.archid == "MIPS:BE:32:default:default"
            && len == 8
            && self.regions.iter().any(|region| {
                at.checked_sub(region.address)
                    .and_then(|offset| usize::try_from(offset).ok())
                    .and_then(|offset| region.bytes.get(offset..offset.checked_add(4)?))
                    == Some(&[0x03, 0xe0, 0, 8][..])
            })
        {
            let mut names = Vec::new();
            self.arch.translate().get_user_op_names(&mut names);
            if let (Some(userop), Ok(ra), Ok(pc), Ok(mode)) = (
                self.arch.userops.get_op_by_name(b"setISAMode"),
                self.arch.translate().get_register_data("ra"),
                self.arch.translate().get_register_data("pc"),
                self.arch.translate().get_register_data("ISAModeSwitch"),
            ) {
                if let Ok(index) = usize::try_from(userop.get_index()) {
                    if names.get(index).is_some_and(|name| name == "setISAMode")
                        && return_mode(&collect, &ra, &pc, &mode, index as u64)
                    {
                        collect.0.remove(2);
                    }
                }
            }
        }
        Some((next, collect))
    }

    fn owner(&self, at: u64) -> Option<(u64, u64)> {
        let (&entry, &size) = self.functions.range(..=at).next_back()?;
        let end = entry.checked_add(size)?;
        (size != 0 && at < end).then_some((entry, end))
    }

    fn locals_only(regs: &mut BTreeMap<Cell, u8>, spaces: &BTreeSet<i32>) {
        regs.retain(|(space, _), _| !spaces.contains(space));
    }

    fn producer(&mut self, entry: u64) -> Option<bool> {
        if !self.functions.contains_key(&entry) {
            return Some(false);
        }
        let (_, end) = self.owner(entry)?;
        let hi = storage(&self.pair.hi.0, self.pair.hi.1)?;
        let lo = storage(&self.pair.lo.0, self.pair.lo.1)?;
        let mut made = BTreeMap::<Cell, u8>::new();
        let mut uniques = BTreeSet::new();
        let mut at = entry;
        let mut ops = 0;
        for _ in 0..MAX_PRODUCER_INSNS {
            if at != entry && self.functions.contains_key(&at) {
                return Some(false);
            }
            let (next, instruction) = self.instruction(at, end)?;
            ops += instruction.0.len();
            if ops > MAX_PRODUCER_OPS {
                return Some(false);
            }
            for (opcode, output, inputs) in instruction.0 {
                if output.iter().chain(inputs.iter()).any(|v| !bounded(v)) {
                    return Some(false);
                }
                for v in output.iter().chain(inputs.iter()) {
                    if v.space.as_ref()?.get_type() == spacetype::IPTR_INTERNAL {
                        uniques.insert(v.space.as_ref()?.get_index());
                    }
                }
                if opcode == OpCode::CPUI_RETURN {
                    return Some([&hi, &lo].iter().all(|v| {
                        (0..v.size).all(|i| cell(v, i).is_some_and(|k| made.get(&k) == Some(&3)))
                    }));
                }
                if opcode != OpCode::CPUI_COPY
                    && opcode != OpCode::CPUI_PIECE
                    && opcode != OpCode::CPUI_SUBPIECE
                    && !pure(opcode)
                {
                    return Some(false);
                }
                let Some(output) = output else {
                    return Some(false);
                };
                if !matches!(
                    output.space.as_ref()?.get_type(),
                    spacetype::IPTR_PROCESSOR | spacetype::IPTR_INTERNAL
                ) {
                    return Some(false);
                }
                if output.space.as_ref()?.get_type() == spacetype::IPTR_PROCESSOR
                    && output.space.as_ref()?.get_index() != hi.space.as_ref()?.get_index()
                {
                    return Some(false);
                }
                if matches!(
                    opcode,
                    OpCode::CPUI_COPY | OpCode::CPUI_PIECE | OpCode::CPUI_SUBPIECE
                ) {
                    let bytes: Vec<_> = inputs.iter().map(|v| vec![0; v.size as usize]).collect();
                    if transport(opcode, &inputs, &bytes, output.size).is_none() {
                        return Some(false);
                    }
                }
                let valid = inputs.iter().all(|input| {
                    if input
                        .space
                        .as_ref()
                        .is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
                    {
                        return true;
                    }
                    (0..input.size).all(|i| {
                        cell(input, i).is_some_and(|k| {
                            if let Some(value) = made.get(&k) {
                                return value & 1 != 0;
                            }
                            self.model.input_opt().is_some_and(|list| {
                                list.get_entry().iter().any(|arg| {
                                    arg.get_space().get_index() == k.0
                                        && arg.get_base() <= k.1
                                        && arg
                                            .get_base()
                                            .checked_add(arg.get_size() as u64)
                                            .is_some_and(|end| k.1 < end)
                                })
                            })
                        })
                    })
                });
                let self_copy = opcode == OpCode::CPUI_COPY
                    && inputs.first().is_some_and(|input| {
                        input.space.as_ref().map(|s| s.get_index())
                            == output.space.as_ref().map(|s| s.get_index())
                            && input.offset == output.offset
                            && input.size == output.size
                    });
                if !self_copy {
                    for i in 0..output.size {
                        made.insert(cell(&output, i)?, if valid { 3 } else { 0 });
                    }
                }
            }
            Self::locals_only(&mut made, &uniques);
            at = next;
        }
        Some(false)
    }

    fn assigned(&mut self, entry: u64) -> Option<Vec<ParameterPieces>> {
        let declaration = self.declarations.get(&entry)?;
        if !Rc::ptr_eq(&declaration.model, &self.model)
            || !integer_contract(&declaration.pieces)
            || declaration.pieces.first_var_arg_slot >= 0
            || !declaration.pieces.input_storage.is_empty()
            || declaration.pieces.output_storage.is_some()
        {
            return None;
        }
        self.budget.spend(declaration.pieces.intypes.len() + 1)?;
        let mut pieces = Vec::new();
        self.model
            .assign_parameter_storage(
                &declaration.pieces,
                &mut pieces,
                true,
                self.arch.types(),
                &self.manager,
            )
            .ok()?;
        if pieces.len() != declaration.pieces.intypes.len() + 1 {
            return None;
        }
        self.used_declarations.insert(entry);
        Some(pieces)
    }

    fn declaration_storage_supported(&mut self) -> Option<bool> {
        for entry in self.used_declarations.clone() {
            let declaration = self.declarations.get(&entry)?;
            if declaration.source != DeclarationSource::Dwarf {
                continue;
            }
            if !integer_contract(&declaration.pieces) {
                return Some(false);
            }
            let assigned = self.assigned(entry)?;
            let rules = crate::kuna_narrowext::rules(self.arch);
            for (index, piece) in assigned.iter().enumerate() {
                let ty = piece.type_.as_ref()?;
                if !matches!(integer_width(ty.get_metatype(), ty.get_size()), Some(1 | 2)) {
                    continue;
                }
                let Some(widen) = (if index == 0 {
                    rules.output
                } else {
                    rules.input
                }) else {
                    return Some(false);
                };
                let list = if index == 0 {
                    self.model.output_list()?
                } else {
                    self.model.input_opt()?
                };
                let mut container = VarnodeData::default();
                if crate::kuna_narrowext::extension(
                    widen,
                    list,
                    &piece.addr,
                    ty.get_size(),
                    ty,
                    &mut container,
                )
                .is_none()
                    || container.size != 4
                    || (index == 0 && container.offset != self.pair.hi.0.get_offset())
                    || !container
                        .space
                        .as_ref()
                        .zip(self.pair.hi.0.get_space())
                        .is_some_and(|(actual, expected)| Rc::ptr_eq(actual, expected))
                {
                    return Some(false);
                }
            }
        }
        Some(true)
    }

    fn read(&self, values: &BTreeMap<Cell, u8>, location: &VarnodeData) -> Option<Vec<u8>> {
        if location.size == 0 || location.size > MAX_STORAGE {
            return None;
        }
        if location.space.as_ref()?.get_type() == spacetype::IPTR_CONSTANT {
            return Some(vec![0; location.size as usize]);
        }
        (0..location.size)
            .map(|i| {
                Some(
                    values
                        .get(&numeric_cell(location, i)?)
                        .copied()
                        .unwrap_or(0),
                )
            })
            .collect()
    }

    fn write(
        &self,
        values: &mut BTreeMap<Cell, u8>,
        location: &VarnodeData,
        bytes: &[u8],
    ) -> Option<()> {
        if bytes.len() != location.size as usize
            || location.size == 0
            || location.size > MAX_STORAGE
        {
            return None;
        }
        let kind = location.space.as_ref()?.get_type();
        if !matches!(kind, spacetype::IPTR_PROCESSOR | spacetype::IPTR_INTERNAL) {
            return None;
        }
        if kind == spacetype::IPTR_PROCESSOR
            && location.space.as_ref()?.get_index() != self.pair.hi.0.get_space()?.get_index()
        {
            return None;
        }
        for (i, &byte) in bytes.iter().enumerate() {
            let key = numeric_cell(location, i as u32)?;
            if byte == 0 {
                values.remove(&key);
            } else {
                values.insert(key, byte);
            }
        }
        Some(())
    }

    fn parameter(&self, values: &BTreeMap<Cell, u8>, piece: &ParameterPieces) -> Option<Vec<u8>> {
        let size = u32::try_from(piece.type_.as_ref()?.get_size()).ok()?;
        let address = &piece.addr;
        let space = address.get_space()?;
        if space.get_type() == spacetype::IPTR_JOIN {
            let join = join(self.arch.manage(), &self.manager, address)?;
            if join.is_float_extension() || join.num_pieces() != 2 {
                return None;
            }
            let mut bytes = Vec::new();
            for i in (0..join.num_pieces()).rev() {
                let part = join.get_piece(i);
                let location = VarnodeData {
                    space: part.space.clone(),
                    offset: part.offset,
                    size: u32::try_from(part.size).ok()?,
                };
                if location.size != 4
                    || location.space.as_ref()?.get_index()
                        != self.pair.hi.0.get_space()?.get_index()
                {
                    return None;
                }
                bytes.extend(self.read(values, &location)?);
            }
            return (bytes.len() == size as usize).then_some(bytes);
        }
        if space.get_index() != self.pair.hi.0.get_space()?.get_index() {
            return None;
        }
        self.read(values, &storage(address, i32::try_from(size).ok()?)?)
    }

    fn kill_output(&self, values: &mut BTreeMap<Cell, u8>, piece: &ParameterPieces) -> Option<()> {
        let datatype = piece.type_.as_ref()?;
        if datatype.get_metatype() == type_metatype::TYPE_VOID {
            return Some(());
        }
        integer(piece)?;
        let address = &piece.addr;
        if address.get_space()?.get_type() == spacetype::IPTR_JOIN {
            let join = join(self.arch.manage(), &self.manager, address)?;
            if join.num_pieces() != 2 || join.is_float_extension() {
                return None;
            }
            for i in 0..2 {
                let piece = join.get_piece(i);
                let location = VarnodeData {
                    space: piece.space.clone(),
                    offset: piece.offset,
                    size: u32::try_from(piece.size).ok()?,
                };
                self.write(values, &location, &vec![0; location.size as usize])?;
            }
            return Some(());
        }
        let location = storage(address, datatype.get_size())?;
        self.write(values, &location, &vec![0; location.size as usize])
    }

    fn caller(&mut self, producer: u64, site: u64) -> Option<Option<bool>> {
        let (caller, end) = self.owner(site)?;
        if caller == producer {
            return Some(None);
        }
        if self.declarations.contains_key(&caller) && self.assigned(caller).is_none() {
            return Some(None);
        }
        let hi = storage(&self.pair.hi.0, self.pair.hi.1)?;
        let lo = storage(&self.pair.lo.0, self.pair.lo.1)?;
        let mut values = BTreeMap::new();
        let mut uniques = BTreeSet::new();
        let mut at = site;
        let mut started = false;
        let mut positive = false;
        let mut high_read = false;
        let mut ops = 0;
        for _ in 0..MAX_FORWARD_INSNS {
            if at != site && self.functions.contains_key(&at) {
                return Some(None);
            }
            let (next, instruction) = self.instruction(at, end)?;
            ops += instruction.0.len();
            if ops > MAX_FORWARD_OPS {
                return Some(None);
            }
            for (opcode, output, inputs) in instruction.0 {
                if output.iter().chain(inputs.iter()).any(|v| !bounded(v)) {
                    return Some(None);
                }
                if !started {
                    if opcode == OpCode::CPUI_CALL && self.direct(&inputs) == Some(producer) {
                        self.write(&mut values, &lo, &[1, 2, 3, 4])?;
                        self.write(&mut values, &hi, &[5, 6, 7, 8])?;
                        started = true;
                    } else if !matches!(
                        opcode,
                        OpCode::CPUI_COPY
                            | OpCode::CPUI_PIECE
                            | OpCode::CPUI_SUBPIECE
                            | OpCode::CPUI_LOAD
                            | OpCode::CPUI_STORE
                    ) && !pure(opcode)
                    {
                        return Some(None);
                    }
                    continue;
                }
                for v in output.iter().chain(inputs.iter()) {
                    if v.space.as_ref()?.get_type() == spacetype::IPTR_INTERNAL {
                        uniques.insert(v.space.as_ref()?.get_index());
                    }
                }
                if opcode == OpCode::CPUI_CALL {
                    if positive {
                        return Some(None);
                    }
                    let Some(target) = self.direct(&inputs) else {
                        return Some(None);
                    };
                    let Some(pieces) = self.assigned(target) else {
                        return Some(None);
                    };
                    let mut seen = 0;
                    for piece in pieces.iter().skip(1) {
                        let bytes = self.parameter(&values, piece)?;
                        if bytes.iter().any(|&b| b != 0) {
                            if integer(piece) != Some(8) || bytes != [1, 2, 3, 4, 5, 6, 7, 8] {
                                return Some(None);
                            }
                            seen += 1;
                        }
                    }
                    if seen != 1 {
                        return Some(None);
                    }
                    self.kill_output(&mut values, &pieces[0])?;
                    positive = true;
                    continue;
                }
                if opcode == OpCode::CPUI_RETURN {
                    let Some(pieces) = self.assigned(caller) else {
                        return Some(None);
                    };
                    let out = &pieces[0];
                    if out.type_.as_ref()?.get_metatype() == type_metatype::TYPE_VOID {
                        return Some(Some(positive));
                    }
                    let Some(width) = integer(out) else {
                        return Some(None);
                    };
                    let bytes = self.parameter(&values, out)?;
                    if positive {
                        return Some((!bytes.iter().any(|&b| b != 0)).then_some(true));
                    }
                    if width <= 4
                        && !bytes.iter().any(|&b| (1..=4).contains(&b))
                        && (high_read || bytes.iter().any(|&b| b >= 5))
                    {
                        return Some(Some(false));
                    }
                    return Some(None);
                }
                let read: Vec<Vec<u8>> = inputs
                    .iter()
                    .map(|v| self.read(&values, v))
                    .collect::<Option<_>>()?;
                let tagged = read.iter().flatten().any(|&b| b != 0);
                if positive && tagged {
                    return Some(None);
                }
                let result = match opcode {
                    OpCode::CPUI_COPY | OpCode::CPUI_PIECE | OpCode::CPUI_SUBPIECE => {
                        transport(opcode, &inputs, &read, output.as_ref()?.size)?
                    }
                    OpCode::CPUI_LOAD if !tagged => vec![0; output.as_ref()?.size as usize],
                    OpCode::CPUI_STORE if !tagged => continue,
                    _ if pure(opcode) => {
                        if read.iter().flatten().any(|&b| (1..=4).contains(&b)) {
                            return Some(None);
                        }
                        high_read |= tagged;
                        vec![0; output.as_ref()?.size as usize]
                    }
                    _ => return Some(None),
                };
                self.write(&mut values, output.as_ref()?, &result)?;
            }
            if !started {
                return Some(None);
            }
            Self::locals_only(&mut values, &uniques);
            if values.is_empty() {
                return Some(Some(positive));
            }
            at = next;
        }
        Some(None)
    }

    fn direct(&self, inputs: &[VarnodeData]) -> Option<u64> {
        let [target] = inputs else { return None };
        Rc::ptr_eq(
            target.space.as_ref()?,
            self.arch.manage().get_default_code_space()?,
        )
        .then_some(target.offset)
    }
}

/// Complete all candidates before publishing any fact. Budget exhaustion keeps
/// the entire image ordinary rather than retaining a traversal-order prefix.
pub(super) fn prove(
    arch: &Architecture,
    regions: &[CodeRegion],
    functions: &[(u64, u64)],
    closed_sites: &BTreeMap<u64, Vec<u64>>,
    declarations: &[Declaration],
    authority: Authority,
) -> Option<Rc<Ledger>> {
    let model = arch.default_fp()?.clone();
    if !arch
        .eval_fp_current()
        .is_some_and(|current| Rc::ptr_eq(current, &model))
    {
        return None;
    }
    let pair = crate::kuna_bejoin::wide_pair(&model, arch.manage())?;
    if pair.hi.1 != 4 || pair.lo.1 != 4 || !pair.hi.0.get_space()?.is_big_endian() {
        return None;
    }
    if declarations.len() > MAX_DECLARATIONS {
        return None;
    }
    let mut declaration_bytes = 0usize;
    let mut declaration_parameters = 0usize;
    let mut declaration_map = BTreeMap::new();
    for declaration in declarations {
        let pieces = &declaration.pieces;
        if pieces.intypes.len() > MAX_PARAMETERS
            || pieces.innames.len() > MAX_PARAMETERS
            || pieces.input_storage.len() > MAX_PARAMETERS
            || pieces
                .outtype
                .iter()
                .chain(pieces.intypes.iter())
                .any(|ty| ty.get_size() < 0 || ty.get_size() > MAX_STORAGE as i32)
        {
            return None;
        }
        declaration_parameters = declaration_parameters.checked_add(pieces.intypes.len() + 1)?;
        declaration_bytes = declaration_bytes.checked_add(pieces.name.len())?;
        for name in &pieces.innames {
            declaration_bytes = declaration_bytes.checked_add(name.len())?;
        }
        if declaration_parameters > 65_536 || declaration_bytes > MAX_DECLARATION_BYTES {
            return None;
        }
        if declaration_map
            .insert(declaration.entry, declaration)
            .is_some()
        {
            return None;
        }
    }
    let mut image = Image {
        arch,
        regions,
        functions: functions.iter().copied().collect(),
        declarations: declaration_map,
        used_declarations: BTreeSet::new(),
        pair: pair.clone(),
        model: model.clone(),
        budget: Budget(MAX_IMAGE_OPS),
        authority,
        manager: proof_manager(arch.manage())?,
    };
    if image.functions.len() != functions.len()
        || functions.len() > 16_384
        || closed_sites
            .values()
            .try_fold(0usize, |sum, sites| sum.checked_add(sites.len()))?
            > 65_536
    {
        return None;
    }
    let mut last = None;
    for (&entry, &size) in &image.functions {
        let end = entry.checked_add(size)?;
        if size == 0
            || last.is_some_and(|last| entry < last)
            || !regions.iter().any(|r| {
                r.address <= entry
                    && r.address
                        .checked_add(r.bytes.len() as u64)
                        .is_some_and(|last| end <= last)
            })
        {
            return None;
        }
        last = Some(end);
    }
    let mut entries = BTreeSet::new();
    let mut needed = BTreeSet::new();
    let space = arch.manage().get_default_code_space()?.get_index();
    for (&entry, sites) in closed_sites {
        image.used_declarations.clear();
        if sites.is_empty()
            || sites.len() > MAX_SITES
            || sites.iter().copied().collect::<BTreeSet<_>>().len() != sites.len()
        {
            continue;
        }
        if image
            .declarations
            .get(&entry)
            .is_some_and(|d| d.pieces.outtype.is_some() || !d.pieces.input_storage.is_empty())
        {
            continue;
        }
        if !image.producer(entry)? {
            continue;
        }
        let mut positive = false;
        let mut valid = true;
        for &site in sites {
            match image.caller(entry, site)? {
                Some(typed) => positive |= typed,
                None => {
                    valid = false;
                    break;
                }
            }
        }
        if valid && positive && image.declaration_storage_supported()? {
            entries.insert((space, entry));
            needed.extend(image.used_declarations.iter().copied());
        }
    }
    if image.budget.0 == 0 {
        return None;
    }
    Some(Rc::new(Ledger {
        live: std::cell::Cell::new(true),
        model,
        pair,
        entries,
        declarations: needed,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_join_allocation_does_not_change_the_original_pool() {
        use kuna_base::space::{addrspace_flags, AddrSpace, ConstantSpace, VarnodeStorage};
        let mut original = AddrSpaceManager::new();
        original
            .insert_space(Rc::new(ConstantSpace::new()))
            .unwrap();
        let register = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            true,
            4,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        original.insert_space(register.clone()).unwrap();
        original
            .insert_space(Rc::new(JoinSpace::new(5, true)))
            .unwrap();
        original.set_default_code_space(4).unwrap();
        original.set_default_data_space(4).unwrap();
        let pieces = |offset| {
            [
                VarnodeStorage {
                    space: Some(register.clone()),
                    offset,
                    size: 4,
                },
                VarnodeStorage {
                    space: Some(register.clone()),
                    offset: offset + 4,
                    size: 4,
                },
            ]
        };
        let old = original.find_add_join(&pieces(0), 0).unwrap();
        let local = proof_manager(&original).unwrap();
        assert!(Rc::ptr_eq(
            original.get_space_by_name("register").unwrap(),
            local.get_space_by_name("register").unwrap(),
        ));
        let fresh = local.find_add_join(&pieces(8), 0).unwrap();
        let address = |record: &JoinRecord| {
            Address::new(
                record.get_unified().space.as_ref().unwrap().clone(),
                record.get_unified().offset,
            )
        };
        assert_eq!(address(&old).get_offset(), address(&fresh).get_offset());
        assert!(!Rc::ptr_eq(
            address(&old).get_space().unwrap(),
            address(&fresh).get_space().unwrap(),
        ));
        assert_eq!(
            join(&original, &local, &address(&old))
                .unwrap()
                .get_piece(0)
                .offset,
            0
        );
        assert_eq!(
            join(&original, &local, &address(&fresh))
                .unwrap()
                .get_piece(0)
                .offset,
            8
        );
        local.find_add_join(&pieces(16), 0).unwrap();
        assert!(original.find_join(16).is_err());
        drop(local);
        assert!(Rc::ptr_eq(&old, &original.find_join(0).unwrap()));
        let next = original.find_add_join(&pieces(24), 0).unwrap();
        assert_eq!(next.get_unified().offset, 16);
    }

    fn mode_bundle() -> (Collect, VarnodeData, VarnodeData, VarnodeData) {
        use kuna_base::space::{addrspace_flags, AddrSpace, ConstantSpace, UniqueSpace};
        let register = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            true,
            4,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        let unique: Rc<kuna_base::space::AddrSpace> = Rc::new(UniqueSpace::new(2, 0, true)).into();
        let constants: Rc<kuna_base::space::AddrSpace> = Rc::new(ConstantSpace::new()).into();
        let vn = |space: &Rc<kuna_base::space::AddrSpace>, offset, size| VarnodeData {
            space: Some(space.clone()),
            offset,
            size,
        };
        let ra = vn(&register, 124, 4);
        let pc = vn(&register, 128, 4);
        let mode = vn(&register, 0x3f00, 1);
        let bit = vn(&unique, 0x1900, 4);
        let target = vn(&unique, 0x1c00, 4);
        let ops = Collect(vec![
            (
                OpCode::CPUI_INT_AND,
                Some(bit.clone()),
                vec![ra.clone(), vn(&constants, 1, 4)],
            ),
            (
                OpCode::CPUI_INT_NOTEQUAL,
                Some(mode.clone()),
                vec![bit, vn(&constants, 0, 4)],
            ),
            (
                OpCode::CPUI_CALLOTHER,
                None,
                vec![vn(&constants, 0, 4), mode.clone()],
            ),
            (
                OpCode::CPUI_INT_2COMP,
                Some(target.clone()),
                vec![vn(&constants, 2, 4)],
            ),
            (
                OpCode::CPUI_INT_AND,
                Some(target.clone()),
                vec![target.clone(), ra.clone()],
            ),
            (OpCode::CPUI_COPY, Some(pc.clone()), vec![target]),
            (
                OpCode::CPUI_INT_ADD,
                Some(vn(&register, 8, 4)),
                vec![vn(&register, 32, 4), vn(&register, 8, 4)],
            ),
            (OpCode::CPUI_RETURN, None, vec![pc.clone()]),
        ]);
        (ops, ra, pc, mode)
    }

    #[test]
    fn return_mode_requires_the_exact_switch_dependency_and_only_one_userop() {
        let (ops, ra, pc, mode) = mode_bundle();
        assert!(return_mode(&ops, &ra, &pc, &mode, 0));
        assert!(!return_mode(&ops, &ra, &pc, &mode, 1));
        for mutation in 0..13 {
            let (mut ops, ra, pc, mut mode) = mode_bundle();
            match mutation {
                0 => ops.0[2].2[1].offset += 1,
                1 => ops.0[2].2[1].size = 4,
                2 => ops.0[2].2.push(ra.clone()),
                3 => ops.0[2].1 = Some(mode.clone()),
                4 => ops.0[1].2[0].offset += 4,
                5 => ops.0[0].2[1].offset = 3,
                6 => ops.0.swap(0, 1),
                7 => ops.0.insert(3, (OpCode::CPUI_CALLOTHER, None, vec![])),
                8 => mode.offset += 1,
                9 => ops.0[3].2[0].offset = 1,
                10 => ops.0[4].2[1].offset = 16,
                11 => ops.0[5].1.as_mut().unwrap().offset = 16,
                _ => ops.0.last_mut().unwrap().2[0].offset = 16,
            }
            assert!(
                !return_mode(&ops, &ra, &pc, &mode, 0),
                "mutation {mutation}"
            );
        }
    }

    #[test]
    fn a_wide_bool_is_not_an_eight_byte_integer_formal() {
        assert_eq!(integer_width(type_metatype::TYPE_BOOL, 8), None);
        assert_eq!(integer_width(type_metatype::TYPE_BOOL, 4), None);
        assert_eq!(integer_width(type_metatype::TYPE_BOOL, 1), Some(1));
        assert_eq!(integer_width(type_metatype::TYPE_INT, 8), Some(8));
        assert_eq!(integer_width(type_metatype::TYPE_UINT, 8), Some(8));
    }

    #[test]
    fn exhausted_budget_cannot_resume_from_a_partial_prefix() {
        let mut budget = Budget(7);
        assert_eq!(budget.spend(4), Some(()));
        assert_eq!(budget.spend(4), None);
        assert_eq!(budget.0, 0);
        assert_eq!(budget.spend(1), None);
    }
}
