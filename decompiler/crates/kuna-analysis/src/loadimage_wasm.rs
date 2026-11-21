//! (kuna) A WebAssembly module as a load image.
//!
//! A module has no addresses of its own: code is a list of bodies, memory is a
//! separate index space, and globals and tables are index spaces too. The
//! loader gives each a fixed place in one 32-bit space, so the decompiler sees
//! an ordinary flat image:
//!
//! ```text
//! 0x00000000  linear memory 0, its minimum size, data segments placed in it
//! 0x80000000  code: every body in register form (`loader::kuna_wasmlower`)
//! 0xd0000000  globals, 16 bytes apart, holding their initial values
//! 0xe0100000  tables, as 4-byte code addresses filled from element segments
//! 0xf0000000  imported functions, 16 bytes apart, with no bytes behind them
//! ```
//!
//! A linear memory address is therefore the address the decompiler shows. A
//! function is named by the `name` section, then by its export or import name.
//! An immutable global with a constant initializer is read-only, and so is a
//! table that nothing can write: one that is neither imported nor exported in
//! a module whose every body was lowered (a lowered body holds no table write).

use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::fspec::PrototypePieces;
use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageFunc, LoadImageSection};

use crate::loader::format::wasm::{
    ConstExpr, ExportKind, FuncType, ImportKind, Module, ValType,
};
use crate::loader::kuna_wasmlower::{Fix, Lowered, Lowering};
use crate::loadimage_object::ObjectSectionMetadata;
use crate::pass::TypedDataFact;

pub use crate::loader::format::wasm::is_wasm_module;

/// The language a module decodes as.
pub const WASM_LANGUAGE: &str = "Wasm:LE:32:default";

pub const CODE_BASE: u64 = 0x8000_0000;
pub const GLOBAL_BASE: u64 = 0xd000_0000;
/// Not a single run of set bits, which the decompiler reads as a mask rather
/// than as an address.
pub const TABLE_BASE: u64 = 0xe010_0000;
pub const IMPORT_BASE: u64 = 0xf000_0000;
/// Bytes between two globals, and between two imported functions.
pub const SLOT_LEN: u64 = 16;
const PAGE_LEN: u64 = 0x1_0000;

/// One function of the module, imported functions first as in its index space.
#[derive(Clone, Debug)]
pub struct WasmFunction {
    pub addr: u64,
    pub name: Option<String>,
    pub imported: bool,
    /// Index into [`WasmImage::types`].
    pub type_index: u32,
}

/// A mapped range: zero-filled except where a piece supplies bytes.
#[derive(Clone, Debug)]
struct Region {
    name: String,
    base: u64,
    size: u64,
    flags: u32,
    /// Non-overlapping initialized spans, keyed by address.
    pieces: BTreeMap<u64, Vec<u8>>,
}

impl Region {
    fn new(name: &str, base: u64, size: u64, flags: u32) -> Region {
        Region { name: name.to_string(), base, size, flags, pieces: BTreeMap::new() }
    }

    fn end(&self) -> u64 {
        self.base + self.size
    }

    /// Place `bytes` at `addr`, replacing whatever earlier pieces held there.
    /// Returns `false`, placing nothing, when the span leaves the region.
    fn overlay(&mut self, addr: u64, bytes: &[u8]) -> bool {
        // cast: usize to u64 widens
        let Some(end) = addr.checked_add(bytes.len() as u64) else { return false };
        if addr < self.base || end > self.end() {
            return false;
        }
        if bytes.is_empty() {
            return true;
        }
        let first = self.pieces.range(..=addr).next_back().map_or(addr, |(&start, _)| start);
        let hit: Vec<u64> = self.pieces.range(first..end).map(|(&start, _)| start).collect();
        for start in hit {
            let old = self.pieces.remove(&start).expect("key came from the map");
            // cast: usize to u64 widens
            let old_end = start + old.len() as u64;
            if old_end <= addr {
                self.pieces.insert(start, old);
                continue;
            }
            // cast: both differences are below the piece's length
            if start < addr {
                self.pieces.insert(start, old[..(addr - start) as usize].to_vec());
            }
            if old_end > end {
                self.pieces.insert(end, old[(end - start) as usize..].to_vec());
            }
        }
        self.pieces.insert(addr, bytes.to_vec());
        true
    }

    /// Fill `dst` from `addr` up to the region's end; returns the bytes filled.
    fn fill(&self, dst: &mut [u8], addr: u64) -> usize {
        // cast: the count is at most `dst.len()`
        let count = (self.end() - addr).min(dst.len() as u64) as usize;
        let dst = &mut dst[..count];
        dst.fill(0);
        let first = self.pieces.range(..=addr).next_back().map_or(addr, |(&start, _)| start);
        // cast: usize to u64 widens
        for (&start, piece) in self.pieces.range(first..addr + count as u64) {
            let (skip, at) = if start < addr { ((addr - start) as usize, 0) } else { (0, (start - addr) as usize) };
            if skip < piece.len() {
                let len = (piece.len() - skip).min(count - at);
                dst[at..at + len].copy_from_slice(&piece[skip..skip + len]);
            }
        }
        count
    }
}

/// A parsed module laid out in the flat image.
#[derive(Clone, Debug)]
pub struct WasmImage {
    regions: Vec<Region>,
    readonly: Vec<(u64, u64)>,
    pub functions: Vec<WasmFunction>,
    pub types: Vec<Option<FuncType>>,
    /// Named globals as `(address, name, bytes)`.
    pub globals: Vec<(u64, String, u64)>,
    /// Every table as `(address, name, bytes)`, named by its export.
    pub tables: Vec<(u64, String, u64)>,
    /// The start function's address.
    pub entry: Option<u64>,
    /// Bodies left as an `unsupported` record: `(function index, reason)`.
    pub refused: Vec<(u32, String)>,
    /// Segments not placed because their offset is not a constant or is out
    /// of range.
    pub unplaced_segments: usize,
}

fn const_u32(expr: Option<ConstExpr>) -> Option<u64> {
    match expr? {
        // cast: an i32 offset is an unsigned address
        ConstExpr::I32(value) => Some(u64::from(value as u32)),
        _ => None,
    }
}

/// The export of `kind` that names index `index`.
fn export_name(module: &Module, kind: ExportKind, index: usize) -> Option<&String> {
    module
        .exports
        .iter()
        // cast: u32 to usize widens
        .find(|export| export.kind == kind && export.index as usize == index)
        .map(|export| &export.name)
}

/// The address of every function, imports first, and the bytes the lowered
/// bodies take; an error when the code, imports or globals outgrow their
/// place in the layout.
fn function_addresses(module: &Module, lowered: &[Lowered]) -> Result<(Vec<u64>, u64), String> {
    let imported = module.imported_functions();
    // cast: usize to u64 widens
    let mut addrs: Vec<u64> = (0..imported as u64).map(|index| IMPORT_BASE + index * SLOT_LEN).collect();
    let mut code_len = 0u64;
    for body in lowered {
        addrs.push(CODE_BASE + code_len);
        code_len += u64::from(body.len());
    }
    let import_room = (u64::from(u32::MAX) + 1 - IMPORT_BASE) / SLOT_LEN;
    let global_room = (TABLE_BASE - GLOBAL_BASE) / SLOT_LEN;
    // cast: usize to u64 widens
    if code_len > GLOBAL_BASE - CODE_BASE
        || imported as u64 > import_room
        || module.global_types().len() as u64 > global_room
    {
        return Err("module is too large for the 32-bit image layout".into());
    }
    Ok((addrs, code_len))
}

/// One region per table, sized by its minimum and packed from [`TABLE_BASE`],
/// with the active element segments placed. Returns the segments not placed.
fn table_regions(module: &Module, addrs: &[u64]) -> (Vec<Region>, usize) {
    let mut tables = Vec::new();
    let mut next = TABLE_BASE;
    for limits in module.table_limits() {
        let size = limits.min.min(IMPORT_BASE.saturating_sub(next) / 4) * 4;
        tables.push(Region::new(&format!("table{}", tables.len()), next, size, section_flags::DATA));
        next += size.next_multiple_of(SLOT_LEN);
    }
    let mut unplaced = 0;
    for segment in module.elements.iter().filter(|segment| segment.offset.is_some()) {
        let mut words = Vec::with_capacity(segment.funcs.len() * 4);
        for func in &segment.funcs {
            // cast: u32 to usize widens; a function address is below 2^32
            let addr = func.and_then(|index| addrs.get(index as usize)).map_or(0, |&addr| addr as u32);
            words.extend_from_slice(&addr.to_le_bytes());
        }
        // cast: u32 to usize widens
        let placed = const_u32(segment.offset)
            .zip(tables.get_mut(segment.table as usize))
            .is_some_and(|(offset, table)| table.overlay(table.base + offset * 4, &words));
        unplaced += usize::from(!placed);
    }
    (tables, unplaced)
}

/// Memory 0 at its minimum size with the active data segments placed.
/// Returns the segments not placed.
fn memory_region(module: &Module, bytes: &[u8]) -> Result<(Region, usize), String> {
    let memory = module.memory();
    if memory.is_some_and(|limits| limits.memory64) {
        return Err("64-bit linear memory (memory64) is not supported".into());
    }
    let len = memory.map_or(0, |limits| limits.min.saturating_mul(PAGE_LEN));
    if len > CODE_BASE {
        return Err(format!("linear memory of 0x{len:x} bytes does not fit below the code"));
    }
    let mut region = Region::new("memory", 0, len, section_flags::DATA);
    let mut unplaced = 0;
    for segment in module.data.iter().filter(|segment| segment.offset.is_some()) {
        let placed = segment.memory == 0
            && const_u32(segment.offset).is_some_and(|offset| region.overlay(offset, &bytes[segment.bytes.clone()]));
        unplaced += usize::from(!placed);
    }
    Ok((region, unplaced))
}

/// The globals at their initial values, the read-only ones as `(address,
/// bytes)`, and the named ones as `(address, name, bytes)`.
#[allow(clippy::type_complexity)]
fn global_region(module: &Module) -> (Region, Vec<(u64, u64)>, Vec<(u64, String, u64)>) {
    let types = module.global_types();
    let imported = types.len() - module.globals.len();
    // cast: usize to u64 widens
    let addr = |index: usize| GLOBAL_BASE + index as u64 * SLOT_LEN;
    let mut region = Region::new("globals", GLOBAL_BASE, addr(types.len()) - GLOBAL_BASE, section_flags::DATA);
    let mut readonly = Vec::new();
    for (index, global) in module.globals.iter().enumerate() {
        let value = match global.init {
            // cast: the initializer's bit pattern
            ConstExpr::I32(value) => u64::from(value as u32),
            ConstExpr::I64(value) => value as u64,
            ConstExpr::F32(bits) => u64::from(bits),
            ConstExpr::F64(bits) => bits,
            _ => continue,
        };
        region.overlay(addr(imported + index), &value.to_le_bytes());
        if !global.ty.mutable {
            readonly.push((addr(imported + index), SLOT_LEN));
        }
    }
    let mut named = Vec::new();
    for (index, ty) in types.iter().enumerate() {
        // cast: a global index is below the u32 count the module declared
        let name = module.global_names.get(&(index as u32)).or(export_name(module, ExportKind::Global, index));
        if let (Some(name), Some(size)) = (name, ty.ty.size()) {
            named.push((addr(index), name.clone(), u64::from(size)));
        }
    }
    (region, readonly, named)
}

/// Every function with its name, demangled as an object file's is: the `name`
/// section's, else its export's, else, for an import, its field name.
fn functions(module: &Module, addrs: &[u64]) -> Vec<WasmFunction> {
    let imported = module.imported_functions();
    let import_names: Vec<&String> = module
        .imports
        .iter()
        .filter_map(|import| matches!(import.kind, ImportKind::Func(_)).then_some(&import.field))
        .collect();
    addrs
        .iter()
        .enumerate()
        .map(|(at, &addr)| {
            // cast: a function index is below the u32 count the module declared
            let index = at as u32;
            let name = module
                .function_names
                .get(&index)
                .or(export_name(module, ExportKind::Func, at))
                .or(import_names.get(at).copied());
            WasmFunction {
                addr,
                name: name.map(|name| crate::demangle::demangle_name(name).unwrap_or_else(|| name.clone())),
                imported: at < imported,
                type_index: module.function_type_index(index).unwrap_or(u32::MAX),
            }
        })
        .collect()
}

impl WasmImage {
    /// Parse `bytes` as a module, lower its bodies and lay the image out.
    pub fn parse(bytes: &[u8]) -> Result<WasmImage, String> {
        let module = Module::parse(bytes)?;
        let (memory, unplaced_data) = memory_region(&module, bytes)?;
        let imported = module.imported_functions();
        let lowering = Lowering::new(&module, bytes);
        let lowered: Vec<Lowered> = (0..module.bodies.len()).map(|index| lowering.lower(index)).collect();
        let (addrs, code_len) = function_addresses(&module, &lowered)?;
        let (mut tables, unplaced_elements) = table_regions(&module, &addrs);

        // cast: `function_addresses` bounded the code below 2^32 bytes
        let mut code = Vec::with_capacity(code_len as usize);
        for (body, &addr) in lowered.iter().zip(&addrs[imported..]) {
            // cast: every address of the layout is below 2^32; u32 to usize widens
            let resolve = |fix: Fix| match fix {
                Fix::Function(index) => addrs.get(index as usize).map_or(0, |&addr| addr as u32),
                Fix::Table(index) => tables.get(index as usize).map_or(0, |table| table.base as u32),
                Fix::None | Fix::Label(_) => 0,
            };
            body.encode(addr as u32, resolve, &mut code);
        }
        let mut code_region = Region::new("code", CODE_BASE, code_len, section_flags::CODE | section_flags::READONLY);
        code_region.pieces.insert(CODE_BASE, code);

        let refused: Vec<(u32, String)> = lowered
            .iter()
            .enumerate()
            // cast: a function index is below the u32 count the module declared
            .filter_map(|(index, body)| Some(((imported + index) as u32, body.refused.clone()?)))
            .collect();
        let imported_tables = tables.len() - module.tables.len();
        let mut table_names = Vec::new();
        for (index, table) in tables.iter_mut().enumerate() {
            let exported = export_name(&module, ExportKind::Table, index);
            if refused.is_empty() && exported.is_none() && index >= imported_tables {
                table.flags |= section_flags::READONLY;
            }
            table_names.push((table.base, exported.unwrap_or(&table.name).clone(), table.size));
        }

        let (globals_region, mut readonly, globals) = global_region(&module);
        let functions = functions(&module, &addrs);
        let mut regions = vec![memory, code_region, globals_region];
        regions.extend(tables);
        regions.retain(|region| region.size != 0);
        regions.sort_by_key(|region| region.base);
        readonly.extend(
            regions
                .iter()
                .filter(|region| region.flags & section_flags::READONLY != 0)
                .map(|region| (region.base, region.size)),
        );
        // cast: u32 to usize widens
        let entry = module.start.and_then(|index| addrs.get(index as usize).copied());
        Ok(WasmImage {
            regions,
            readonly,
            functions,
            types: module.types,
            globals,
            tables: table_names,
            entry,
            refused,
            unplaced_segments: unplaced_data + unplaced_elements,
        })
    }

    /// The signature of `function`, when its type is a plain function type
    /// over the four number types.
    pub fn signature(&self, function: &WasmFunction) -> Option<&FuncType> {
        // cast: u32 to usize widens
        let ty = self.types.get(function.type_index as usize)?.as_ref()?;
        let numeric = |types: &[ValType]| types.iter().all(|ty| ty.size().is_some());
        (numeric(&ty.params) && numeric(&ty.results)).then_some(ty)
    }
}

impl WasmImage {
    /// Each imported function's prototype, by entry address, as its type
    /// states it; an integer is an undefined value of its width. An import has
    /// no body to recover a signature from. A defined function is left to the
    /// decompiler, which a locked signature would stop from typing a parameter
    /// as a pointer.
    pub fn prototypes(&self, arch: &Architecture) -> Vec<(u64, PrototypePieces)> {
        let types = arch.types();
        let datatype = |ty: ValType| match ty {
            ValType::I32 => types.get_base(4, type_metatype::TYPE_UNKNOWN).ok(),
            ValType::I64 => types.get_base(8, type_metatype::TYPE_UNKNOWN).ok(),
            ValType::F32 => types.get_base(4, type_metatype::TYPE_FLOAT).ok(),
            ValType::F64 => types.get_base(8, type_metatype::TYPE_FLOAT).ok(),
            ValType::V128 | ValType::Ref => None,
        };
        let mut out = Vec::new();
        for function in self.functions.iter().filter(|function| function.imported) {
            let Some(signature) = self.signature(function).filter(|ty| ty.results.len() <= 1) else {
                continue;
            };
            let intypes: Option<Vec<_>> = signature.params.iter().map(|&ty| datatype(ty)).collect();
            let outtype = match signature.results.first() {
                Some(&ty) => datatype(ty),
                None => types.get_type_void().ok(),
            };
            let (Some(intypes), Some(outtype)) = (intypes, outtype) else { continue };
            out.push((
                function.addr,
                PrototypePieces {
                    name: function.name.clone().unwrap_or_default(),
                    outtype: Some(outtype),
                    innames: vec![String::new(); intypes.len()],
                    intypes,
                    first_var_arg_slot: -1,
                    output_storage: None,
                    input_storage: Vec::new(),
                    kuna_library_names: None,
                },
            ));
        }
        out
    }

    /// Each table typed as an array of code pointers, so an indirect call
    /// reads as an index into the named table.
    pub fn table_types(&self, arch: &Architecture) -> Vec<TypedDataFact> {
        let types = arch.types();
        let Ok(pointer) = types.get_type_code().and_then(|code| types.get_type_pointer(4, code, 1)) else {
            return Vec::new();
        };
        self.tables
            .iter()
            .filter_map(|(addr, name, size)| {
                let count = i32::try_from(size / 4).ok().filter(|&count| count > 0)?;
                let type_ = types.get_type_array(count, Rc::clone(&pointer)).ok()?;
                Some(TypedDataFact { addr: *addr, name: name.clone(), type_ })
            })
            .collect()
    }
}

/// The [`LoadImage`] over a [`WasmImage`].
pub struct WasmLoadImage {
    filename: String,
    image: WasmImage,
    space: Option<Rc<AddrSpace>>,
    cursor: Cell<usize>,
}

impl WasmLoadImage {
    pub fn new(filename: &str, image: WasmImage) -> WasmLoadImage {
        WasmLoadImage { filename: filename.to_string(), image, space: None, cursor: Cell::new(0) }
    }

    pub fn attach_to_space(&mut self, space: Rc<AddrSpace>) {
        self.space = Some(space);
    }

    pub fn image(&self) -> &WasmImage {
        &self.image
    }

    /// The image's regions as named sections; none has a file range, because
    /// none is a verbatim span of the module file.
    pub fn section_metadata(&self) -> Vec<ObjectSectionMetadata> {
        self.image
            .regions
            .iter()
            .map(|region| {
                let kind = if region.flags & section_flags::CODE != 0 {
                    object::SectionKind::Text
                } else if region.flags & section_flags::READONLY != 0 {
                    object::SectionKind::ReadOnlyData
                } else {
                    object::SectionKind::Data
                };
                ObjectSectionMetadata::new(&region.name, region.base, region.size, kind, None, region.flags)
            })
            .collect()
    }

    fn region(&self, addr: u64) -> Option<&Region> {
        let index = self.image.regions.partition_point(|region| region.base <= addr).checked_sub(1)?;
        self.image.regions.get(index).filter(|region| addr < region.end())
    }
}

impl LoadImage for WasmLoadImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let attached = match (addr.get_space(), self.space.as_ref()) {
            (Some(space), Some(attached)) => Rc::ptr_eq(space, attached),
            _ => false,
        };
        let start = addr.get_offset();
        let Some(region) = self.region(start).filter(|_| attached) else {
            let mut message = format!("Unable to load {} bytes at ", ptr.len());
            addr.print_raw(&mut message)?;
            return Err(KunaError::data_unavail(message));
        };
        let filled = region.fill(ptr, start);
        ptr[filled..].fill(0);
        Ok(())
    }

    fn open_symbols(&self) {
        self.cursor.set(0);
    }

    fn get_next_symbol(&self, record: &mut LoadImageFunc) -> bool {
        let space = self.space.as_ref().expect("WasmLoadImage::getNextSymbol before attachToSpace");
        let mut index = self.cursor.get();
        while let Some(function) = self.image.functions.get(index) {
            index += 1;
            if let Some(name) = &function.name {
                record.address = Address::new(Rc::clone(space), function.addr);
                record.name = name.clone().into_bytes();
                self.cursor.set(index);
                return true;
            }
        }
        self.cursor.set(index);
        false
    }

    fn open_section_info(&self) {
        self.cursor.set(0);
    }

    fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
        let index = self.cursor.get();
        let Some(region) = self.image.regions.get(index) else {
            return false;
        };
        let space = self.space.as_ref().expect("WasmLoadImage::getNextSection before attachToSpace");
        record.address = Address::new(Rc::clone(space), region.base);
        record.size = region.size;
        record.flags = region.flags;
        self.cursor.set(index + 1);
        index + 1 < self.image.regions.len()
    }

    fn get_segments(&self) -> Vec<(u64, u64, u32)> {
        self.image.regions.iter().map(|region| (region.base, region.size, region.flags)).collect()
    }

    fn get_readonly(&self, list: &mut RangeList) {
        let Some(space) = self.space.as_ref() else {
            return;
        };
        for &(base, size) in &self.image.readonly {
            list.insert_range(Rc::clone(space), base, base + size - 1);
        }
    }

    fn get_arch_type(&self) -> Vec<u8> {
        WASM_LANGUAGE.as_bytes().to_vec()
    }

    fn adjust_vma(&mut self, _adjust: i64) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::format::wasm::synthetic::{i32_const, indexed, ModuleBuilder, I32};
    use kuna_base::space::{addrspace_flags, spacetype};

    fn loaded(builder: &ModuleBuilder) -> (WasmLoadImage, Rc<AddrSpace>) {
        let image = WasmImage::parse(&builder.build()).unwrap();
        let flags = addrspace_flags::hasphysical;
        let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 4, 1, 1, flags, 1, 1));
        let mut loader = WasmLoadImage::new("module.wasm", image);
        loader.attach_to_space(Rc::clone(&ram));
        (loader, ram)
    }

    fn read(loader: &mut WasmLoadImage, ram: &Rc<AddrSpace>, addr: u64, len: usize) -> KunaResult<Vec<u8>> {
        let mut bytes = vec![0xaa; len];
        loader.load_fill(&mut bytes, &Address::new(Rc::clone(ram), addr))?;
        Ok(bytes)
    }

    fn module() -> ModuleBuilder {
        let mut builder = ModuleBuilder::default();
        builder
            .import("ext", &[I32], &[])
            .function("first", &[], &[], &[], &[i32_const(1), indexed(0x10, 0)].concat())
            .function("second", &[], &[I32], &[], &indexed(0x23, 1))
            .memory(1)
            .global("__stack_pointer", I32, true, 0x1_0000)
            .global("limit", I32, false, 64)
            .table(&[2, 0], false)
            .data(8, b"abcdef")
            .data(10, b"XY");
        builder
    }

    #[test]
    fn each_index_space_has_its_place_in_the_flat_image() {
        let (loader, _) = loaded(&module());
        let image = loader.image();
        let addrs: Vec<(u64, Option<&str>, bool)> =
            image.functions.iter().map(|f| (f.addr, f.name.as_deref(), f.imported)).collect();
        assert_eq!(
            addrs,
            vec![
                (IMPORT_BASE, Some("ext"), true),
                (CODE_BASE, Some("first"), false),
                (CODE_BASE + 4 * 16, Some("second"), false),
            ]
        );
        assert_eq!(
            loader.get_segments(),
            vec![
                (0, 0x1_0000, section_flags::DATA),
                (CODE_BASE, 7 * 16, section_flags::CODE | section_flags::READONLY),
                (GLOBAL_BASE, 2 * 16, section_flags::DATA),
                (TABLE_BASE, 12, section_flags::DATA | section_flags::READONLY),
            ]
        );
        assert_eq!(
            image.globals,
            vec![(GLOBAL_BASE, "__stack_pointer".to_string(), 4), (GLOBAL_BASE + 16, "limit".to_string(), 4)]
        );
        assert_eq!(image.tables, vec![(TABLE_BASE, "table0".to_string(), 12)]);
    }

    #[test]
    fn reads_see_placed_bytes_zero_fill_and_nothing_outside_a_region() {
        let (mut loader, ram) = loaded(&module());
        assert_eq!(read(&mut loader, &ram, 6, 10).unwrap(), b"\0\0abXYef\0\0");
        assert_eq!(read(&mut loader, &ram, 0xfffe, 4).unwrap(), [0, 0, 0, 0], "a read may run off a region's end");
        assert!(read(&mut loader, &ram, 0x1_0000, 4).is_err());
        assert!(read(&mut loader, &ram, IMPORT_BASE, 4).is_err(), "an import has no bytes");
        // The call record of `first` holds the import's address.
        assert_eq!(read(&mut loader, &ram, CODE_BASE + 2 * 16 + 8, 4).unwrap(), (IMPORT_BASE as u32).to_le_bytes());
        // Slot 0 is null; slots 1 and 2 hold `second` and the import.
        let second = (CODE_BASE as u32 + 4 * 16).to_le_bytes();
        let table = [[0; 4], second, (IMPORT_BASE as u32).to_le_bytes()].concat();
        assert_eq!(read(&mut loader, &ram, TABLE_BASE, 12).unwrap(), table);
        assert_eq!(read(&mut loader, &ram, GLOBAL_BASE, 20).unwrap()[..20], [[0, 0, 1, 0].as_slice(), &[0; 12], &[64, 0, 0, 0]].concat());
    }

    #[test]
    fn only_what_nothing_can_write_is_read_only() {
        let readonly = |builder: &ModuleBuilder| {
            let (loader, _) = loaded(builder);
            let mut list = RangeList::new();
            loader.get_readonly(&mut list);
            list.iter().map(|range| (range.get_first(), range.get_last())).collect::<Vec<_>>()
        };
        let code = (CODE_BASE, CODE_BASE + 7 * 16 - 1);
        let limit = (GLOBAL_BASE + 16, GLOBAL_BASE + 31);
        let table = (TABLE_BASE, TABLE_BASE + 11);
        assert_eq!(readonly(&module()), vec![code, limit, table]);
        let mut exported = module();
        exported.table(&[2, 0], true);
        assert_eq!(readonly(&exported), vec![code, limit]);
        let mut refused = module();
        refused.function("simd", &[], &[], &[], &[0xfd, 0x0c]);
        assert_eq!(readonly(&refused), vec![(CODE_BASE, CODE_BASE + 8 * 16 - 1), limit]);
    }

    #[test]
    fn named_functions_are_the_symbol_stream_and_a_refused_body_is_reported() {
        let mut builder = module();
        builder.function("simd", &[], &[], &[], &[0xfd, 0x0c]);
        let (loader, _) = loaded(&builder);
        let mut names = Vec::new();
        loader.open_symbols();
        let mut record = LoadImageFunc::default();
        while loader.get_next_symbol(&mut record) {
            names.push((record.address.get_offset(), String::from_utf8(record.name.clone()).unwrap()));
        }
        assert_eq!(
            names,
            vec![
                (IMPORT_BASE, "ext".to_string()),
                (CODE_BASE, "first".to_string()),
                (CODE_BASE + 4 * 16, "second".to_string()),
                (CODE_BASE + 7 * 16, "simd".to_string()),
            ]
        );
        assert_eq!(loader.image().refused, vec![(3, "uses instruction 0xfd".to_string())]);
    }

    #[test]
    fn a_memory_the_layout_cannot_hold_is_an_error() {
        let mut builder = ModuleBuilder::default();
        builder.memory(0x8001);
        assert!(WasmImage::parse(&builder.build()).unwrap_err().contains("does not fit below the code"));
    }

    #[test]
    fn a_segment_outside_its_memory_is_counted_not_placed() {
        let mut builder = ModuleBuilder::default();
        builder.memory(1).data(0xfffe, b"abcd");
        assert_eq!(WasmImage::parse(&builder.build()).unwrap().unplaced_segments, 1);
    }
}
