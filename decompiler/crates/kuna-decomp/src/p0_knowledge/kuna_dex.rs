//! (kuna `dexmodel`) What a DEX file tells the engine about its methods.
//!
//! The Dalvik SLEIGH spec leaves three things to the program that loaded the
//! file, which in Ghidra is its Java side:
//!
//! - the **constant pool** (`ConstantPoolDex`): every `cpool(ref, index, tag)`
//!   operator names a method, field, string or type by its index in the file;
//! - the **`uponentry` payload** (`InjectPayloadDexParameters`): a method's
//!   arguments arrive in the designated input registers `iv0..` and are copied to
//!   the top `ins_size` registers of its frame;
//! - the **`moveRangeToIV` payload** (`InjectPayloadDexRange`): `invoke-*/range`
//!   copies a run of frame registers into `iv0..`.
//!
//! [`DexModel`] answers all three from the parsed file. The architecture holds
//! one when it loaded a DEX image, and hands it to each function while option
//! `dexmodel` is on.

pub mod format;
pub mod links;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::cpool::{cpool_tag, CPoolRecord};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::fspec::PrototypePieces;
use crate::kuna_callmechanisms::Operation;

pub use format::{java_type_name, simple_class_name, CatchHandler, DexFile, EncodedMethod, TryItem};

/// Register-space offset of `iv0`, the first designated input register.
pub const INPUT_REGISTER_START: u64 = 0x100;
/// Register-space offset of `v0`, the bottom of a method's frame.
pub const REGISTER_START: u64 = 0x1000;

/// The `cpool` tags the Dalvik spec uses (`Dalvik_Base.sinc`, `CPOOL_*`).
mod tag {
    pub const METHOD: u64 = 0;
    pub const FIELD: u64 = 1;
    pub const STATIC_FIELD: u64 = 2;
    pub const STATIC_METHOD: u64 = 3;
    pub const STRING: u64 = 4;
    pub const CLASSREF: u64 = 5;
    pub const ARRAYLENGTH: u64 = 6;
    pub const SUPER: u64 = 7;
    pub const INSTANCEOF: u64 = 8;
}

/// A parsed DEX file, where it is mapped, and the caches the engine's queries
/// fill.
pub struct DexModel {
    pub file: DexFile,
    /// Address of the file's first byte. An APK's `classes*.dex` files are
    /// mapped one after another.
    pub base: u64,
    entries: HashMap<u64, EncodedMethod>,
    records: RefCell<HashMap<(u64, u64), Option<Rc<CPoolRecord>>>>,
    types: RefCell<HashMap<String, Rc<Datatype>>>,
}

impl std::fmt::Debug for DexModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DexModel").field("methods", &self.entries.len()).finish()
    }
}

/// A `try_item` in image addresses: the covered instructions `[start, end)` and
/// its catch clauses `(type_idx, handler)` in match order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryBlock {
    pub start: u64,
    pub end: u64,
    pub handlers: Vec<(Option<u32>, u64)>,
}

/// One method with code, as the loader names it.
#[derive(Clone, Debug)]
pub struct DexFunction {
    /// Byte address of the first instruction.
    pub addr: u64,
    /// `pkg::Class::method`, with `_<addr>` added to each member of an
    /// overload set.
    pub name: String,
    pub method_idx: u32,
    pub is_static: bool,
}

impl DexModel {
    pub fn new(file: DexFile, base: u64) -> DexModel {
        let mut entries = HashMap::new();
        for class in &file.classes {
            for method in &class.methods {
                if let Some(code) = method.code {
                    entries.insert(base + u64::from(code.insns_off), *method);
                }
            }
        }
        DexModel {
            file,
            base,
            entries,
            records: RefCell::new(HashMap::new()),
            types: RefCell::new(HashMap::new()),
        }
    }

    /// Parse `bytes` as a DEX file mapped at `base`.
    pub fn parse(bytes: Vec<u8>, base: u64) -> KunaResult<DexModel> {
        Ok(DexModel::new(DexFile::parse(bytes)?, base))
    }

    /// Does the file's mapping hold `addr`?
    pub fn contains(&self, addr: u64) -> bool {
        addr.checked_sub(self.base).is_some_and(|off| off < self.file.bytes().len() as u64)
    }

    fn string(&self, idx: u32) -> String {
        self.file.string(idx).unwrap_or_else(|| format!("string_{idx:x}"))
    }

    /// `(class descriptor, name, prototype descriptor)` of method `method_idx`.
    pub fn method_key(&self, method_idx: u32) -> Option<(String, String, String)> {
        let id = self.file.method_id(method_idx)?;
        let proto = self.file.proto_id(id.proto_idx)?;
        let mut desc = String::from("(");
        for param in &proto.parameters {
            desc.push_str(&self.descriptor(*param));
        }
        desc.push(')');
        desc.push_str(&self.descriptor(proto.return_type_idx));
        Some((self.descriptor(id.class_idx), self.string(id.name_idx), desc))
    }

    fn descriptor(&self, type_idx: u32) -> String {
        self.file.type_descriptor(type_idx).unwrap_or_else(|| "V".to_string())
    }

    fn class_scope(&self, class_idx: u32) -> String {
        java_type_name(&self.descriptor(class_idx)).replace('.', "::")
    }

    /// Every method with code, in file order, named for the symbol table.
    pub fn functions(&self) -> Vec<DexFunction> {
        let mut out = Vec::new();
        for class in &self.file.classes {
            for method in &class.methods {
                let Some(code) = method.code else { continue };
                let Some(id) = self.file.method_id(method.method_idx) else { continue };
                out.push(DexFunction {
                    addr: self.base + u64::from(code.insns_off),
                    name: format!("{}::{}", self.class_scope(id.class_idx), self.string(id.name_idx)),
                    method_idx: method.method_idx,
                    is_static: method.is_static(),
                });
            }
        }
        let mut counts: HashMap<String, usize> = HashMap::new();
        for function in &out {
            *counts.entry(function.name.clone()).or_default() += 1;
        }
        for function in &mut out {
            if counts[&function.name] > 1 {
                function.name = format!("{}_{:x}", function.name, function.addr);
            }
        }
        out
    }

    /// The register copies `InjectPayloadDexParameters` emits for the method
    /// whose code starts at `addr`: `(from, to, size)` in register space.
    pub fn entry_copies(&self, addr: u64) -> Option<Vec<(u64, u64, u32)>> {
        let entry = self.entries.get(&addr)?;
        let code = entry.code?;
        let id = self.file.method_id(entry.method_idx)?;
        let proto = self.file.proto_id(id.proto_idx)?;
        let mut from = INPUT_REGISTER_START;
        let mut to = REGISTER_START + 4 * u64::from(code.registers_size - code.ins_size);
        let mut copies = Vec::new();
        if !entry.is_static() {
            copies.push((from, to, 4));
            from += 4;
            to += 4;
        }
        for param in proto.parameters {
            let size = if format::is_wide(&self.descriptor(param)) { 8 } else { 4 };
            copies.push((from, to, size));
            from += u64::from(size);
            to += u64::from(size);
        }
        Some(copies)
    }

    /// The class of the method whose code starts at `addr`, and its superclass
    /// (`None` for `java.lang.Object`).
    pub fn method_class(&self, addr: u64) -> Option<(u32, Option<u32>)> {
        let id = self.file.method_id(self.entries.get(&addr)?.method_idx)?;
        let class = self.file.classes.iter().find(|c| c.class_idx == id.class_idx)?;
        let superclass = (class.superclass_idx != format::NO_INDEX).then_some(class.superclass_idx);
        Some((id.class_idx, superclass))
    }

    /// The class that declares method `method_idx`.
    pub fn method_owner(&self, method_idx: u32) -> Option<u32> {
        self.file.method_id(method_idx).map(|id| id.class_idx)
    }

    /// The descriptor of type `idx`.
    pub fn type_descriptor(&self, idx: u32) -> Option<String> {
        self.file.type_descriptor(idx)
    }

    /// The `try_item`s of the method whose code starts at `addr`.
    pub fn try_blocks(&self, addr: u64) -> Vec<TryBlock> {
        let Some(code) = self.entries.get(&addr).and_then(|m| m.code) else { return Vec::new() };
        let Ok(tries) = self.file.tries(&code) else { return Vec::new() };
        let unit = |u: u32| addr + 2 * u64::from(u);
        tries
            .iter()
            .map(|t| TryBlock {
                start: unit(t.start),
                end: unit(t.start + u32::from(t.count)),
                handlers: t.handlers.iter().map(|h| (h.type_idx, unit(h.addr))).collect(),
            })
            .collect()
    }

    /// The instruction bytes at `addr`, when the file holds them.
    pub fn bytes_at(&self, addr: u64, len: usize) -> Option<&[u8]> {
        let off = usize::try_from(addr.checked_sub(self.base)?).ok()?;
        self.file.bytes().get(off..off.checked_add(len)?)
    }

    /// The length in bytes of the instruction at `addr`, payloads included.
    pub fn instruction_size(&self, addr: u64) -> Option<u64> {
        let head = self.bytes_at(addr, 2)?;
        let units = match (head[0], head[1]) {
            (0x00, 0x01..=0x03) => {
                let word = |k: u64| -> Option<u64> {
                    let b = self.bytes_at(addr + 2 * k, 2)?;
                    Some(u64::from(u16::from_le_bytes([b[0], b[1]])))
                };
                match head[1] {
                    0x01 => word(1)? * 2 + 4,
                    0x02 => word(1)? * 4 + 2,
                    _ => {
                        let width = word(1)?;
                        let size = word(2)? | (word(3)? << 16);
                        (size * width + 1) / 2 + 4
                    }
                }
            }
            (op, _) => u64::from(format::opcode_units(op)),
        };
        Some(2 * units)
    }

    /// The simple name of a caught class; the catch-all catches `Throwable`.
    pub fn catch_type_name(&self, type_idx: Option<u32>) -> String {
        match type_idx {
            Some(idx) => simple_class_name(&self.descriptor(idx)),
            None => "Throwable".to_string(),
        }
    }

    /// The descriptor of a caught class; the catch-all catches `Throwable`.
    pub fn catch_descriptor(&self, type_idx: Option<u32>) -> String {
        match type_idx {
            Some(idx) => self.descriptor(idx),
            None => "Ljava/lang/Throwable;".to_string(),
        }
    }

    /// The Java type of a descriptor, as Ghidra's `DexUtil.toDataType` builds it:
    /// primitives by name, a class as a pointer to a structure of that name, an
    /// array as a pointer to its element type.
    pub fn java_type(&self, descriptor: &str, types: &dyn TypeFactory) -> KunaResult<Rc<Datatype>> {
        if let Some(found) = self.types.borrow().get(descriptor) {
            return Ok(Rc::clone(found));
        }
        use type_metatype::*;
        let built = match descriptor {
            "V" => types.get_type_void()?,
            "Z" => types.get_base_named(1, TYPE_BOOL, "boolean")?,
            "B" => types.get_base_named(1, TYPE_INT, "byte")?,
            "S" => types.get_base_named(2, TYPE_INT, "short")?,
            "C" => types.get_type_char(2)?,
            "I" => types.get_base(4, TYPE_INT)?,
            "J" => types.get_base_named(8, TYPE_INT, "long")?,
            "F" => types.get_base(4, TYPE_FLOAT)?,
            "D" => types.get_base(8, TYPE_FLOAT)?,
            _ if descriptor.starts_with('[') => {
                let element = self.java_type(&descriptor[1..], types)?;
                types.get_type_pointer(4, element, 1)?
            }
            _ if descriptor.starts_with('L') => {
                let class = types.get_type_struct(&simple_class_name(descriptor))?;
                types.get_type_pointer(4, class, 1)?
            }
            _ => return Err(KunaError::lowlevel(format!("bad DEX type descriptor: {descriptor}"))),
        };
        self.types.borrow_mut().insert(descriptor.to_string(), Rc::clone(&built));
        Ok(built)
    }

    /// The prototype of method `method_idx`, and the model it is called with
    /// (`__thiscall` for an instance method, whose first input is `this`).
    /// `call_site` builds the shape `ConstantPoolDex` gives a call: the object
    /// is an untyped `ref`, where the method's own signature has a typed `this`.
    pub fn method_prototype(
        &self,
        method_idx: u32,
        is_static: bool,
        call_site: bool,
        types: &dyn TypeFactory,
    ) -> KunaResult<(PrototypePieces, &'static str)> {
        let id = self
            .file
            .method_id(method_idx)
            .ok_or_else(|| KunaError::lowlevel("DEX method index out of range"))?;
        let proto = self
            .file
            .proto_id(id.proto_idx)
            .ok_or_else(|| KunaError::lowlevel("DEX proto index out of range"))?;
        let mut pieces = PrototypePieces { first_var_arg_slot: -1, ..Default::default() };
        pieces.name = self.string(id.name_idx);
        pieces.outtype = Some(self.java_type(&self.descriptor(proto.return_type_idx), types)?);
        if !is_static {
            if call_site {
                pieces.intypes.push(types.get_base(4, type_metatype::TYPE_UNKNOWN)?);
                pieces.innames.push("ref".to_string());
            } else {
                pieces.intypes.push(self.java_type(&self.descriptor(id.class_idx), types)?);
                pieces.innames.push("this".to_string());
            }
        }
        for param in proto.parameters {
            pieces.intypes.push(self.java_type(&self.descriptor(param), types)?);
            pieces.innames.push(String::new());
        }
        Ok((pieces, if is_static { "__stdcall" } else { "__thiscall" }))
    }

    /// The constant-pool record for `refs = [index, tag, ..]` (Ghidra's
    /// `ConstantPoolDex.getRecord`, which reads only the first two), built on
    /// first use.
    pub fn record(&self, refs: &[u64], types: &dyn TypeFactory) -> Option<Rc<CPoolRecord>> {
        let [index, tag, ..] = *refs else { return None };
        if let Some(found) = self.records.borrow().get(&(index, tag)) {
            return found.clone();
        }
        let built = self.build_record(index, tag, types).ok().flatten().map(Rc::new);
        self.records.borrow_mut().insert((index, tag), built.clone());
        built
    }

    fn build_record(&self, index: u64, tag: u64, types: &dyn TypeFactory) -> KunaResult<Option<CPoolRecord>> {
        let index32 = u32::try_from(index).map_err(|_| KunaError::lowlevel("DEX index out of range"))?;
        let record = match tag {
            tag::METHOD | tag::STATIC_METHOD => {
                let is_static = tag == tag::STATIC_METHOD;
                let Some(id) = self.file.method_id(index32) else { return Ok(None) };
                let mut token = self.string(id.name_idx);
                if is_static {
                    token = format!("{}.{token}", simple_class_name(&self.descriptor(id.class_idx)));
                }
                let (pieces, model) = self.method_prototype(index32, is_static, true, types)?;
                let code = types.get_type_code_proto_named(&pieces, model)?;
                let ptr = types.get_type_pointer(4, code, 1)?;
                CPoolRecord::from_parts(cpool_tag::POINTER_METHOD, token.into_bytes(), ptr, None)
            }
            tag::FIELD | tag::STATIC_FIELD => {
                let Some(id) = self.file.field_id(index32) else { return Ok(None) };
                let mut token = self.string(id.name_idx);
                if tag == tag::STATIC_FIELD {
                    token = format!("{}.{token}", simple_class_name(&self.descriptor(id.class_idx)));
                }
                let field = self.java_type(&self.descriptor(id.type_idx), types)?;
                let ptr = types.get_type_pointer(4, field, 1)?;
                CPoolRecord::from_parts(cpool_tag::POINTER_FIELD, token.into_bytes(), ptr, None)
            }
            tag::STRING => {
                let Some(text) = self.file.string(index32) else { return Ok(None) };
                let ty = self.java_type("Ljava/lang/String;", types)?;
                CPoolRecord::from_parts(cpool_tag::STRING_LITERAL, Vec::new(), ty, Some(text.into_bytes()))
            }
            tag::CLASSREF | tag::INSTANCEOF => {
                let (ty, token) = if index32 >= self.file.num_types() as u32 {
                    (types.get_base(4, type_metatype::TYPE_UNKNOWN)?, "<none>".to_string())
                } else {
                    let descriptor = self.descriptor(index32);
                    let token = if descriptor.starts_with('L') {
                        simple_class_name(&descriptor)
                    } else {
                        "<none>".to_string()
                    };
                    (self.java_type(&descriptor, types)?, token)
                };
                if tag == tag::INSTANCEOF {
                    CPoolRecord::from_parts(cpool_tag::INSTANCE_OF, b"instanceof".to_vec(), ty, None)
                } else {
                    CPoolRecord::from_parts(cpool_tag::CLASS_REFERENCE, token.into_bytes(), ty, None)
                }
            }
            tag::ARRAYLENGTH => CPoolRecord::from_parts(
                cpool_tag::ARRAY_LENGTH,
                b"length".to_vec(),
                types.get_base(4, type_metatype::TYPE_INT)?,
                None,
            ),
            // Upstream types `super` as a 1-byte `undefined`; a register-wide
            // one keeps the object reference it stands for free of a cast.
            tag::SUPER => CPoolRecord::from_parts(
                cpool_tag::POINTER_METHOD,
                b"super".to_vec(),
                types.get_base(4, type_metatype::TYPE_UNKNOWN)?,
                None,
            ),
            _ => return Ok(None),
        };
        Ok(Some(record))
    }
}

fn copy_operations(
    address: &Address,
    register: &Rc<AddrSpace>,
    copies: &[(u64, u64, u32)],
) -> Vec<Operation> {
    let storage = |offset, size| VarnodeData { space: Some(Rc::clone(register)), offset, size };
    copies
        .iter()
        .map(|&(from, to, size)| Operation {
            address: address.clone(),
            opcode: OpCode::CPUI_COPY,
            output: Some(storage(to, size)),
            inputs: vec![storage(from, size)],
        })
        .collect()
}

/// Insert the `uponentry` copies at the start of the function (the dynamic
/// payload `ActionConstbase` injects upstream).
pub fn apply_entry(data: &mut crate::funcdata::Funcdata) -> KunaResult<()> {
    if data.get_arch().dex.is_none() || data.kuna_call_mechanisms.dex_entry || data.bblocks_get_size() == 0 {
        return Ok(());
    }
    let arch = data.get_arch().clone();
    let Some(model) = arch.dex.as_ref() else { return Ok(()) };
    let Some(copies) = model.entry_copies(data.get_address().get_offset()) else { return Ok(()) };
    let register = arch
        .manage
        .get_space_by_name("register")
        .ok_or_else(|| KunaError::lowlevel("Dalvik spec has no register space"))?;
    let block = data.bblocks_get_block(0);
    let start = data.bblocks_block_start(block);
    let operations = copy_operations(&start, register, &copies);
    crate::kuna_callmechanisms::install_at_entry(data, block, &operations)?;
    data.kuna_call_mechanisms.dex_entry = true;
    Ok(())
}

/// Emit the `moveRangeToIV(count, first)` copies (`InjectPayloadDexRange`):
/// `count` registers starting at register-space offset `first` move to `iv0..`.
pub fn emit_range(
    context: &crate::pcodeinject::InjectContext,
    emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
) -> KunaResult<()> {
    let [count, first] = context.inputlist.as_slice() else {
        return Err(KunaError::lowlevel("moveRangeToIV takes two inputs"));
    };
    let register = first
        .space
        .as_ref()
        .ok_or_else(|| KunaError::lowlevel("moveRangeToIV register has no space"))?;
    let address = context
        .baseaddr
        .clone()
        .ok_or_else(|| KunaError::lowlevel("moveRangeToIV without an address"))?;
    let copies: Vec<(u64, u64, u32)> = (0..count.offset.min(256))
        .map(|i| (first.offset + 4 * i, INPUT_REGISTER_START + 4 * i, 4))
        .collect();
    for op in copy_operations(&address, register, &copies) {
        emit.dump(&op.address, op.opcode, op.output.as_ref(), &op.inputs);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
