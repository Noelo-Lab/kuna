//! (kuna) The WebAssembly binary module: its sections read into plain tables.
//!
//! A module is the 8-byte preamble followed by `(id, size, payload)` sections.
//! Every count is checked against the bytes that remain before anything is
//! allocated for it, and a malformed module is an error, never a panic. The
//! function bodies are not decoded here: [`Body::code`] is the file range of
//! one body's instruction stream, which
//! [`crate::loader::kuna_wasmlower`] turns into register form.

use std::collections::BTreeMap;
use std::ops::Range;

/// `\0asm` and binary version 1.
pub const WASM_MAGIC: [u8; 8] = *b"\0asm\x01\0\0\0";

/// Whether `bytes` opens with the WebAssembly version-1 preamble.
pub fn is_wasm_module(bytes: &[u8]) -> bool {
    bytes.starts_with(&WASM_MAGIC)
}

pub type WasmResult<T> = Result<T, String>;

/// A value type. Reference and vector types are carried, not lowered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref,
}

impl ValType {
    fn decode(byte: u8) -> WasmResult<ValType> {
        Ok(match byte {
            0x7f => ValType::I32,
            0x7e => ValType::I64,
            0x7d => ValType::F32,
            0x7c => ValType::F64,
            0x7b => ValType::V128,
            0x63..=0x74 => ValType::Ref,
            _ => return Err(format!("unknown value type 0x{byte:02x}")),
        })
    }

    /// Bytes of storage, for the four number types.
    pub fn size(self) -> Option<u32> {
        match self {
            ValType::I32 | ValType::F32 => Some(4),
            ValType::I64 | ValType::F64 => Some(8),
            ValType::V128 | ValType::Ref => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Limits {
    pub min: u64,
    pub max: Option<u64>,
    pub memory64: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlobalType {
    pub ty: ValType,
    pub mutable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportKind {
    Func(u32),
    Table(Limits),
    Memory(Limits),
    Global(GlobalType),
    Tag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Import {
    pub module: String,
    pub field: String,
    pub kind: ImportKind,
}

/// A constant initializer expression, as far as the loader evaluates one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstExpr {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    GlobalGet(u32),
    RefFunc(u32),
    RefNull,
    /// An extended constant expression the loader does not fold.
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Global {
    pub ty: GlobalType,
    pub init: ConstExpr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    Func,
    Table,
    Memory,
    Global,
    Tag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: String,
    pub kind: ExportKind,
    pub index: u32,
}

/// An element segment. `offset` is `Some` only for an active segment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementSegment {
    pub table: u32,
    pub offset: Option<ConstExpr>,
    /// Function index per slot; `None` is a null or non-constant entry.
    pub funcs: Vec<Option<u32>>,
}

/// A data segment. `offset` is `Some` only for an active segment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataSegment {
    pub memory: u32,
    pub offset: Option<ConstExpr>,
    pub bytes: Range<usize>,
}

/// One entry of the code section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    /// Declared locals as `(count, type)` runs, parameters not included.
    pub locals: Vec<(u32, ValType)>,
    /// File range of the instruction stream, the final `end` included.
    pub code: Range<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub types: Vec<Option<FuncType>>,
    pub imports: Vec<Import>,
    /// Type index of each function the module defines.
    pub functions: Vec<u32>,
    pub tables: Vec<Limits>,
    pub memories: Vec<Limits>,
    pub globals: Vec<Global>,
    pub exports: Vec<Export>,
    pub start: Option<u32>,
    pub elements: Vec<ElementSegment>,
    pub data: Vec<DataSegment>,
    pub bodies: Vec<Body>,
    /// The `name` section's function and global names, by index.
    pub function_names: BTreeMap<u32, String>,
    pub global_names: BTreeMap<u32, String>,
}

/// A bounds-checked cursor over the module bytes.
pub struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
    end: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8], range: Range<usize>) -> Reader<'a> {
        let end = range.end.min(bytes.len());
        Reader { bytes, pos: range.start.min(end), end }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.end - self.pos
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.end
    }

    pub fn byte(&mut self) -> WasmResult<u8> {
        let byte = *self.bytes[..self.end].get(self.pos).ok_or("unexpected end of module")?;
        self.pos += 1;
        Ok(byte)
    }

    pub fn take(&mut self, len: usize) -> WasmResult<Range<usize>> {
        if len > self.remaining() {
            return Err("length runs past the end of its section".into());
        }
        let range = self.pos..self.pos + len;
        self.pos += len;
        Ok(range)
    }

    fn leb(&mut self, bits: u32) -> WasmResult<u64> {
        let mut value = 0u64;
        let mut shift = 0u32;
        loop {
            let byte = self.byte()?;
            if shift >= bits {
                return Err("LEB128 integer is too long".into());
            }
            value |= u64::from(byte & 0x7f) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                return Ok(if bits < 64 { value & ((1u64 << bits) - 1) } else { value });
            }
        }
    }

    pub fn u32(&mut self) -> WasmResult<u32> {
        // cast: masked to 32 bits by `leb`
        self.leb(32).map(|value| value as u32)
    }

    pub fn u64(&mut self) -> WasmResult<u64> {
        self.leb(64)
    }

    fn sleb(&mut self, bits: u32) -> WasmResult<i64> {
        let mut value = 0i64;
        let mut shift = 0u32;
        loop {
            let byte = self.byte()?;
            if shift >= bits {
                return Err("LEB128 integer is too long".into());
            }
            value |= i64::from(byte & 0x7f) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                if shift < 64 && byte & 0x40 != 0 {
                    value |= -1i64 << shift;
                }
                return Ok(value);
            }
        }
    }

    pub fn s32(&mut self) -> WasmResult<i32> {
        // cast: the low 32 bits of a sign-extended 32-bit LEB
        self.sleb(32).map(|value| value as i32)
    }

    pub fn s33(&mut self) -> WasmResult<i64> {
        self.sleb(33)
    }

    pub fn s64(&mut self) -> WasmResult<i64> {
        self.sleb(64)
    }

    pub fn fixed_u32(&mut self) -> WasmResult<u32> {
        let range = self.take(4)?;
        let mut word = [0u8; 4];
        word.copy_from_slice(&self.bytes[range]);
        Ok(u32::from_le_bytes(word))
    }

    pub fn fixed_u64(&mut self) -> WasmResult<u64> {
        let range = self.take(8)?;
        let mut word = [0u8; 8];
        word.copy_from_slice(&self.bytes[range]);
        Ok(u64::from_le_bytes(word))
    }

    /// A vector length that at least `min_item` bytes per item can still back.
    pub fn count(&mut self, min_item: usize) -> WasmResult<usize> {
        // cast: u32 to usize widens on every supported host
        let count = self.u32()? as usize;
        if count.saturating_mul(min_item.max(1)) > self.remaining() {
            return Err("vector length exceeds its section".into());
        }
        Ok(count)
    }

    /// A vector of items, each at least `min_item` bytes, read by `item`.
    fn vector<T>(&mut self, min_item: usize, item: fn(&mut Self) -> WasmResult<T>) -> WasmResult<Vec<T>> {
        (0..self.count(min_item)?).map(|_| item(self)).collect()
    }

    fn global(&mut self) -> WasmResult<Global> {
        let ty = self.global_type()?;
        Ok(Global { ty, init: self.const_expr()? })
    }

    fn name(&mut self) -> WasmResult<String> {
        // cast: u32 to usize widens
        let len = self.u32()? as usize;
        let range = self.take(len)?;
        Ok(String::from_utf8_lossy(&self.bytes[range]).into_owned())
    }

    fn val_type(&mut self) -> WasmResult<ValType> {
        let byte = self.byte()?;
        if matches!(byte, 0x63 | 0x64) {
            self.s33()?;
        }
        ValType::decode(byte)
    }

    fn limits(&mut self) -> WasmResult<Limits> {
        let flags = self.byte()?;
        let memory64 = flags & 4 != 0;
        let min = if memory64 { self.u64()? } else { u64::from(self.u32()?) };
        let max = match flags & 1 {
            0 => None,
            _ => Some(if memory64 { self.u64()? } else { u64::from(self.u32()?) }),
        };
        Ok(Limits { min, max, memory64 })
    }

    fn table_type(&mut self) -> WasmResult<Limits> {
        self.val_type()?;
        self.limits()
    }

    fn global_type(&mut self) -> WasmResult<GlobalType> {
        let ty = self.val_type()?;
        let mutable = self.byte()? & 1 != 0;
        Ok(GlobalType { ty, mutable })
    }

    /// One initializer: a single constant instruction and `end` folds; any
    /// longer expression is skipped to its `end` and reported as `Other`.
    fn const_expr(&mut self) -> WasmResult<ConstExpr> {
        let first = match self.byte()? {
            0x41 => ConstExpr::I32(self.s32()?),
            0x42 => ConstExpr::I64(self.s64()?),
            0x43 => ConstExpr::F32(self.fixed_u32()?),
            0x44 => ConstExpr::F64(self.fixed_u64()?),
            0x23 => ConstExpr::GlobalGet(self.u32()?),
            0xd2 => ConstExpr::RefFunc(self.u32()?),
            0xd0 => {
                self.s33()?;
                ConstExpr::RefNull
            }
            0x0b => return Ok(ConstExpr::Other),
            _ => return Err("unsupported initializer expression".into()),
        };
        if self.byte()? == 0x0b {
            return Ok(first);
        }
        self.pos -= 1;
        loop {
            match self.byte()? {
                0x0b => return Ok(ConstExpr::Other),
                0x41 => drop(self.s32()?),
                0x42 => drop(self.s64()?),
                0x23 => drop(self.u32()?),
                0x6a | 0x6b | 0x6c | 0x7c | 0x7d | 0x7e => {}
                _ => return Err("unsupported initializer expression".into()),
            }
        }
    }
}

impl Module {
    /// Number of imported functions; they take the first function indices.
    pub fn imported_functions(&self) -> usize {
        self.imports.iter().filter(|import| matches!(import.kind, ImportKind::Func(_))).count()
    }

    /// Every global's type, imports first, in index order.
    pub fn global_types(&self) -> Vec<GlobalType> {
        self.imports
            .iter()
            .filter_map(|import| match import.kind {
                ImportKind::Global(ty) => Some(ty),
                _ => None,
            })
            .chain(self.globals.iter().map(|global| global.ty))
            .collect()
    }

    /// Every table's limits, imports first, in index order.
    pub fn table_limits(&self) -> Vec<Limits> {
        self.imports
            .iter()
            .filter_map(|import| match import.kind {
                ImportKind::Table(limits) => Some(limits),
                _ => None,
            })
            .chain(self.tables.iter().copied())
            .collect()
    }

    /// The index of the global the toolchain keeps its linear-memory stack
    /// pointer in: the mutable `i32` global the `name` section or an export
    /// calls `__stack_pointer`.
    pub fn stack_pointer(&self) -> Option<u32> {
        const NAME: &str = "__stack_pointer";
        let named = self.global_names.iter().find(|(_, name)| *name == NAME).map(|(&index, _)| index);
        let exported = || {
            self.exports
                .iter()
                .find(|export| export.kind == ExportKind::Global && export.name == NAME)
                .map(|export| export.index)
        };
        let index = named.or_else(exported)?;
        // cast: u32 to usize widens
        let ty = self.global_types().get(index as usize).copied()?;
        (ty.mutable && ty.ty == ValType::I32).then_some(index)
    }

    /// Memory 0's limits, imported or defined.
    pub fn memory(&self) -> Option<Limits> {
        self.imports
            .iter()
            .find_map(|import| match import.kind {
                ImportKind::Memory(limits) => Some(limits),
                _ => None,
            })
            .or_else(|| self.memories.first().copied())
    }

    /// The type index of function `index`, imported or defined.
    pub fn function_type_index(&self, index: u32) -> Option<u32> {
        // cast: u32 to usize widens
        let index = index as usize;
        let mut imported = self.imports.iter().filter_map(|import| match import.kind {
            ImportKind::Func(ty) => Some(ty),
            _ => None,
        });
        let count = self.imported_functions();
        if index < count {
            imported.nth(index)
        } else {
            self.functions.get(index - count).copied()
        }
    }

    /// The signature behind type index `index`.
    pub fn func_type(&self, index: u32) -> Option<&FuncType> {
        // cast: u32 to usize widens
        self.types.get(index as usize)?.as_ref()
    }

    /// Read every section of `bytes`.
    pub fn parse(bytes: &[u8]) -> WasmResult<Module> {
        if !is_wasm_module(bytes) {
            return Err("not a WebAssembly version 1 module".into());
        }
        let mut module = Module::default();
        let mut reader = Reader::new(bytes, WASM_MAGIC.len()..bytes.len());
        while !reader.is_empty() {
            let id = reader.byte()?;
            // cast: u32 to usize widens
            let size = reader.u32()? as usize;
            let range = reader.take(size)?;
            let mut section = Reader::new(bytes, range);
            match id {
                0 => module.custom_section(bytes, &mut section),
                1 => module.type_section(&mut section)?,
                2 => module.import_section(&mut section)?,
                3 => module.functions = section.vector(1, Reader::u32)?,
                4 => module.tables = section.vector(2, Reader::table_type)?,
                5 => module.memories = section.vector(2, Reader::limits)?,
                6 => module.globals = section.vector(3, Reader::global)?,
                7 => module.export_section(&mut section)?,
                8 => module.start = Some(section.u32()?),
                9 => module.element_section(&mut section)?,
                10 => module.code_section(&mut section)?,
                11 => module.data_section(&mut section)?,
                // The data count and tag sections, and ids no version defines.
                _ => continue,
            }
        }
        if module.bodies.len() != module.functions.len() {
            return Err("function and code section lengths disagree".into());
        }
        Ok(module)
    }

    fn type_section(&mut self, section: &mut Reader) -> WasmResult<()> {
        for _ in 0..section.count(1)? {
            if section.byte()? != 0x60 {
                // A GC composite or recursive type group: its encoding is not
                // read, so no later type index of this section can be trusted.
                self.types.push(None);
                return Ok(());
            }
            let mut ty = FuncType::default();
            for _ in 0..section.count(1)? {
                ty.params.push(section.val_type()?);
            }
            for _ in 0..section.count(1)? {
                ty.results.push(section.val_type()?);
            }
            self.types.push(Some(ty));
        }
        Ok(())
    }

    fn import_section(&mut self, section: &mut Reader) -> WasmResult<()> {
        for _ in 0..section.count(4)? {
            let module = section.name()?;
            let field = section.name()?;
            let kind = match section.byte()? {
                0 => ImportKind::Func(section.u32()?),
                1 => ImportKind::Table(section.table_type()?),
                2 => ImportKind::Memory(section.limits()?),
                3 => ImportKind::Global(section.global_type()?),
                4 => {
                    section.byte()?;
                    section.u32()?;
                    ImportKind::Tag
                }
                kind => return Err(format!("unknown import kind {kind}")),
            };
            self.imports.push(Import { module, field, kind });
        }
        Ok(())
    }

    fn export_section(&mut self, section: &mut Reader) -> WasmResult<()> {
        for _ in 0..section.count(3)? {
            let name = section.name()?;
            let kind = match section.byte()? {
                0 => ExportKind::Func,
                1 => ExportKind::Table,
                2 => ExportKind::Memory,
                3 => ExportKind::Global,
                4 => ExportKind::Tag,
                kind => return Err(format!("unknown export kind {kind}")),
            };
            self.exports.push(Export { name, kind, index: section.u32()? });
        }
        Ok(())
    }

    fn element_section(&mut self, section: &mut Reader) -> WasmResult<()> {
        for _ in 0..section.count(2)? {
            let flags = section.u32()?;
            if flags > 7 {
                return Err(format!("unknown element segment flags {flags}"));
            }
            let passive_or_declared = flags & 1 != 0;
            let explicit_table = flags & 2 != 0;
            let expressions = flags & 4 != 0;
            let table = if !passive_or_declared && explicit_table { section.u32()? } else { 0 };
            let offset = if passive_or_declared { None } else { Some(section.const_expr()?) };
            if passive_or_declared || explicit_table {
                // elemkind (0 = funcref) or, with expressions, a reference type
                if expressions {
                    section.val_type()?;
                } else {
                    section.byte()?;
                }
            }
            let mut funcs = Vec::new();
            for _ in 0..section.count(1)? {
                funcs.push(if expressions {
                    match section.const_expr()? {
                        ConstExpr::RefFunc(index) => Some(index),
                        _ => None,
                    }
                } else {
                    Some(section.u32()?)
                });
            }
            self.elements.push(ElementSegment { table, offset, funcs });
        }
        Ok(())
    }

    fn code_section(&mut self, section: &mut Reader) -> WasmResult<()> {
        for _ in 0..section.count(2)? {
            // cast: u32 to usize widens
            let size = section.u32()? as usize;
            let range = section.take(size)?;
            let mut body = Reader { bytes: section.bytes, pos: range.start, end: range.end };
            let mut locals = Vec::new();
            for _ in 0..body.count(2)? {
                let count = body.u32()?;
                locals.push((count, body.val_type()?));
            }
            self.bodies.push(Body { locals, code: body.pos..range.end });
        }
        Ok(())
    }

    fn data_section(&mut self, section: &mut Reader) -> WasmResult<()> {
        for _ in 0..section.count(2)? {
            let flags = section.u32()?;
            let (memory, offset) = match flags {
                0 => (0, Some(section.const_expr()?)),
                1 => (0, None),
                2 => (section.u32()?, Some(section.const_expr()?)),
                _ => return Err(format!("unknown data segment flags {flags}")),
            };
            // cast: u32 to usize widens
            let len = section.u32()? as usize;
            self.data.push(DataSegment { memory, offset, bytes: section.take(len)? });
        }
        Ok(())
    }

    /// The `name` section, when it parses; a damaged one only loses names.
    fn custom_section(&mut self, bytes: &[u8], section: &mut Reader) {
        if section.name().as_deref() != Ok("name") {
            return;
        }
        while !section.is_empty() {
            let Ok(id) = section.byte() else { return };
            // cast: u32 to usize widens
            let Ok(range) = section.u32().and_then(|size| section.take(size as usize)) else {
                return;
            };
            let names = match id {
                1 => &mut self.function_names,
                7 => &mut self.global_names,
                _ => continue,
            };
            let mut sub = Reader::new(bytes, range);
            let Ok(count) = sub.count(2) else { continue };
            for _ in 0..count {
                let (Ok(index), Ok(name)) = (sub.u32(), sub.name()) else { break };
                names.insert(index, name);
            }
        }
    }
}

/// A small module writer for tests: enough of the binary format to state a
/// module's functions, memory, globals, table and segments by hand.
pub mod synthetic {
    use super::WASM_MAGIC;

    pub const I32: u8 = 0x7f;
    pub const I64: u8 = 0x7e;
    pub const F64: u8 = 0x7c;

    pub fn uleb(mut value: u64) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            // cast: the low 7 bits
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                out.push(byte);
                return out;
            }
            out.push(byte | 0x80);
        }
    }

    pub fn sleb(mut value: i64) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            // cast: the low 7 bits
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0) {
                out.push(byte);
                return out;
            }
            out.push(byte | 0x80);
        }
    }

    fn vector(items: &[Vec<u8>]) -> Vec<u8> {
        // cast: usize to u64 widens
        let mut out = uleb(items.len() as u64);
        items.iter().for_each(|item| out.extend_from_slice(item));
        out
    }

    fn name(text: &str) -> Vec<u8> {
        // cast: usize to u64 widens
        let mut out = uleb(text.len() as u64);
        out.extend_from_slice(text.as_bytes());
        out
    }

    fn section(id: u8, body: Vec<u8>) -> Vec<u8> {
        let mut out = vec![id];
        // cast: usize to u64 widens
        out.extend(uleb(body.len() as u64));
        out.extend(body);
        out
    }

    /// `i32.const value`.
    pub fn i32_const(value: i32) -> Vec<u8> {
        let mut out = vec![0x41];
        out.extend(sleb(i64::from(value)));
        out
    }

    /// One instruction with a single index immediate.
    pub fn indexed(op: u8, index: u32) -> Vec<u8> {
        let mut out = vec![op];
        out.extend(uleb(u64::from(index)));
        out
    }

    /// `block`, `loop` or `if` (`op`) of block type `ty` around `body`.
    pub fn block(op: u8, ty: u8, body: &[u8]) -> Vec<u8> {
        let mut out = vec![op, ty];
        out.extend_from_slice(body);
        out.push(0x0b);
        out
    }

    struct Function {
        name: String,
        ty: usize,
        locals: Vec<(u32, u8)>,
        code: Vec<u8>,
    }

    #[derive(Default)]
    pub struct ModuleBuilder {
        types: Vec<(Vec<u8>, Vec<u8>)>,
        imports: Vec<(String, usize)>,
        functions: Vec<Function>,
        memory_pages: Option<u32>,
        globals: Vec<(String, u8, bool, i32)>,
        table: Vec<u32>,
        export_table: bool,
        data: Vec<(u32, Vec<u8>)>,
    }

    impl ModuleBuilder {
        fn ty(&mut self, params: &[u8], results: &[u8]) -> usize {
            let ty = (params.to_vec(), results.to_vec());
            match self.types.iter().position(|have| *have == ty) {
                Some(index) => index,
                None => {
                    self.types.push(ty);
                    self.types.len() - 1
                }
            }
        }

        /// Import function `env.<field>`. Imports take the first indices, so
        /// declare them before any [`Self::function`].
        pub fn import(&mut self, field: &str, params: &[u8], results: &[u8]) -> &mut Self {
            let ty = self.ty(params, results);
            self.imports.push((field.to_string(), ty));
            self
        }

        /// Define and export a function; `code` omits the final `end`.
        pub fn function(
            &mut self,
            name: &str,
            params: &[u8],
            results: &[u8],
            locals: &[(u32, u8)],
            code: &[u8],
        ) -> &mut Self {
            let ty = self.ty(params, results);
            self.functions.push(Function { name: name.to_string(), ty, locals: locals.to_vec(), code: code.to_vec() });
            self
        }

        pub fn memory(&mut self, pages: u32) -> &mut Self {
            self.memory_pages = Some(pages);
            self
        }

        /// An `i32` or `i64` global named in the `name` section.
        pub fn global(&mut self, name: &str, ty: u8, mutable: bool, init: i32) -> &mut Self {
            self.globals.push((name.to_string(), ty, mutable, init));
            self
        }

        /// One table whose slots from 1 hold the given function indices.
        pub fn table(&mut self, functions: &[u32], exported: bool) -> &mut Self {
            self.table = functions.to_vec();
            self.export_table = exported;
            self
        }

        pub fn data(&mut self, offset: u32, bytes: &[u8]) -> &mut Self {
            self.data.push((offset, bytes.to_vec()));
            self
        }

        pub fn build(&self) -> Vec<u8> {
            // cast: every index below is a small test-sized count
            let index = |value: usize| uleb(value as u64);
            let mut out = WASM_MAGIC.to_vec();
            let types: Vec<Vec<u8>> = self
                .types
                .iter()
                .map(|(params, results)| {
                    let mut ty = vec![0x60];
                    ty.extend(index(params.len()));
                    ty.extend_from_slice(params);
                    ty.extend(index(results.len()));
                    ty.extend_from_slice(results);
                    ty
                })
                .collect();
            out.extend(section(1, vector(&types)));
            let imports: Vec<Vec<u8>> = self
                .imports
                .iter()
                .map(|(field, ty)| [name("env"), name(field), vec![0], index(*ty)].concat())
                .collect();
            out.extend(section(2, vector(&imports)));
            let functions: Vec<Vec<u8>> = self.functions.iter().map(|function| index(function.ty)).collect();
            out.extend(section(3, vector(&functions)));
            if !self.table.is_empty() {
                out.extend(section(4, vector(&[[vec![0x70, 0], index(self.table.len() + 1)].concat()])));
            }
            if let Some(pages) = self.memory_pages {
                out.extend(section(5, vector(&[[vec![0], uleb(u64::from(pages))].concat()])));
            }
            let globals: Vec<Vec<u8>> = self
                .globals
                .iter()
                .map(|&(_, ty, mutable, init)| {
                    let op = if ty == I64 { 0x42 } else { 0x41 };
                    [vec![ty, u8::from(mutable), op], sleb(i64::from(init)), vec![0x0b]].concat()
                })
                .collect();
            out.extend(section(6, vector(&globals)));
            let mut exports: Vec<Vec<u8>> = self
                .functions
                .iter()
                .enumerate()
                .map(|(at, function)| [name(&function.name), vec![0], index(self.imports.len() + at)].concat())
                .collect();
            if self.export_table {
                exports.push([name("table"), vec![1, 0]].concat());
            }
            out.extend(section(7, vector(&exports)));
            if !self.table.is_empty() {
                let slots: Vec<Vec<u8>> = self.table.iter().map(|&function| uleb(u64::from(function))).collect();
                out.extend(section(9, vector(&[[vec![0], i32_const(1), vec![0x0b], vector(&slots)].concat()])));
            }
            let bodies: Vec<Vec<u8>> = self
                .functions
                .iter()
                .map(|function| {
                    let locals: Vec<Vec<u8>> =
                        function.locals.iter().map(|&(count, ty)| [uleb(u64::from(count)), vec![ty]].concat()).collect();
                    let body = [vector(&locals), function.code.clone(), vec![0x0b]].concat();
                    [index(body.len()), body].concat()
                })
                .collect();
            out.extend(section(10, vector(&bodies)));
            let data: Vec<Vec<u8>> = self
                .data
                .iter()
                // cast: the offset's bit pattern as the i32 the format stores
                .map(|(offset, bytes)| [vec![0], i32_const(*offset as i32), vec![0x0b], index(bytes.len()), bytes.clone()].concat())
                .collect();
            out.extend(section(11, vector(&data)));
            let global_names: Vec<Vec<u8>> =
                self.globals.iter().enumerate().map(|(at, global)| [index(at), name(&global.0)].concat()).collect();
            let names = [name("name"), section(7, vector(&global_names))].concat();
            out.extend(section(0, names));
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::synthetic::{i32_const, indexed, ModuleBuilder, I32, I64};
    use super::*;

    #[test]
    fn leb128_reads_stop_at_the_integer_width() {
        let bytes = [0xe5, 0x8e, 0x26, 0x7f, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        let mut reader = Reader::new(&bytes, 0..bytes.len());
        assert_eq!(reader.u32(), Ok(624485));
        assert_eq!(reader.s32(), Ok(-1));
        assert!(reader.u32().is_err(), "six bytes is not a u32");
    }

    #[test]
    fn a_module_reads_back_what_the_builder_stated() {
        let mut builder = ModuleBuilder::default();
        builder
            .import("ext", &[I32], &[])
            .function("first", &[I32, I64], &[I32], &[(2, I64)], &indexed(0x20, 0))
            .memory(2)
            .global("__stack_pointer", I32, true, 0x1_0000)
            .global("limit", I32, false, 64)
            .table(&[1], true)
            .data(16, b"kuna");
        let bytes = builder.build();
        let module = Module::parse(&bytes).unwrap();
        assert_eq!(module.imported_functions(), 1);
        assert_eq!(module.functions.len(), 1);
        let ty = module.func_type(module.function_type_index(1).unwrap()).unwrap();
        assert_eq!((ty.params.as_slice(), ty.results.as_slice()), (&[ValType::I32, ValType::I64][..], &[ValType::I32][..]));
        assert_eq!(module.bodies[0].locals, vec![(2, ValType::I64)]);
        assert_eq!(&bytes[module.bodies[0].code.clone()], &[0x20, 0, 0x0b]);
        assert_eq!(module.memory().unwrap().min, 2);
        assert_eq!(module.stack_pointer(), Some(0));
        assert_eq!(module.globals[1].init, ConstExpr::I32(64));
        assert_eq!(module.elements[0].funcs, vec![Some(1)]);
        assert_eq!(&bytes[module.data[0].bytes.clone()], b"kuna");
        assert_eq!(module.exports.last().unwrap().kind, ExportKind::Table);
    }

    #[test]
    fn an_immutable_global_named_stack_pointer_is_not_one() {
        let mut builder = ModuleBuilder::default();
        builder.global("__stack_pointer", I32, false, 0);
        assert_eq!(Module::parse(&builder.build()).unwrap().stack_pointer(), None);
    }

    #[test]
    fn a_count_larger_than_its_section_is_refused_before_allocation() {
        let mut bytes = WASM_MAGIC.to_vec();
        bytes.extend([3, 5, 0xff, 0xff, 0xff, 0xff, 0x0f]);
        assert_eq!(Module::parse(&bytes).unwrap_err(), "vector length exceeds its section");
    }

    #[test]
    fn every_truncation_of_a_module_is_an_error_or_a_module() {
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[I32], &[I32], &[], &i32_const(7)).memory(1).data(0, b"abc");
        let bytes = builder.build();
        for len in 0..bytes.len() {
            let _ = Module::parse(&bytes[..len]);
        }
    }
}
