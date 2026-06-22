//! DWARF debug-info names + types — the kuna analog of Ghidra's `DWARFAnalyzer`
//! ("DWARF").
//!
//! When a binary carries `.debug_*` sections, the compiler has already recorded
//! the source-level function names, parameter names, and *types*. Ghidra's
//! `DWARFAnalyzer` reads them and installs them onto the program, so a stripped
//! (no `.symtab`) but `-g` binary still decompiles to `add_values(int a,int b)`
//! instead of `FUN_00401136(undefined8,undefined8)`, and a typed parameter like
//! `char *binary` renders instead of `undefined8`/`long`.
//!
//! This pass reproduces the same two recoveries against kuna's symbol + type
//! tables:
//!   * **subtask 1 (names + globals)** — each *defined* `DW_TAG_subprogram`
//!     (one with `DW_AT_low_pc`) emits a [`SymFact`]`{Function}` at its entry
//!     VMA; each top-level `DW_TAG_variable` with a `DW_OP_addr` location emits
//!     a [`SymFact`]`{Data}`.
//!   * **subtask 2 (typed signatures)** — each defined subprogram also emits a
//!     [`PrototypePieces`] built from its return-type DIE + `DW_TAG_formal_parameter`
//!     children, mapping each DWARF type DIE to a kuna [`Datatype`] via the
//!     architecture's [`TypeFactory`].
//!
//! ## Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - Driver: `Ghidra/Features/Base/.../analysis/DWARFAnalyzer.java` (`added()`,
//!   builds DWARFProgram + runs `DWARFImporter.performImport`).
//! - `DWARFFunctionImporter.java` — the DIE iteration + commit loop:
//!   `importFunctions()` (switches on DIE tag), `processSubprogram()`
//!   (fn name/addr/params), `outputGlobal()` (global vars).
//! - `DWARFFunction.java` — `read(DIEAggregate)`: name, body ranges (the
//!   `getFuncBodyRanges` non-empty guard => skip declaration-only),
//!   `DW_AT_external`/`DW_AT_noreturn`, retval (`getDataTypeForVariable`), and
//!   the `getFunctionParamList()` loop building param `DWARFVariable`s. `address`
//!   = `getCodeAddress(dwarfBody.getFirstAddress())` (the entry VMA).
//! - `DWARFDataTypeImporter.java` — `getDataType(DIEAggregate)`, the recursive
//!   tag switch ([`build_datatype`] reproduces it): `makeDataTypeForPointer`,
//!   base_type, struct/union/array/typedef/const/volatile, plus the
//!   `trackRecursion` depth cap ([`MAX_TYPE_DEPTH`]) that survives type cycles.
//!
//! ## Dependency-substitution LOSS
//!
//! Ghidra hand-rolls a complete DWARF reader in `ghidra.app.util.bin.format.dwarf.*`
//! (DWARFProgram, DebugInfoEntry, DIEAggregate, DWARFAbbreviation, StringTable).
//! kuna substitutes [`gimli`], the de-facto Rust DWARF reader, for that parser
//! *wholesale* — the same dependency-substitution LOSS as BFD -> `object` (see
//! `loadimage_object.rs` / docs/rust-port/losses.md). We use gimli's high-level
//! `Dwarf::attr_string` / `attr_address` accessors (NOT raw form decode), so the
//! DWARF-5 `strx`/`addrx`/`.debug_str_offsets` indirections resolve correctly.
//!
//! ## Scope / faithful losses (DOC)
//!
//! - **subtask 3 (stack-local `ScopeLocal` map) is DEFERRED.** Per-local
//!   `DW_OP_fbreg` stack-variable naming+typing (`DWARFVariable.readLocalVariableStorage`)
//!   needs a new `locals` fact + an engine-side commit path mapping each into the
//!   function's `ScopeLocal` with a typelock — a wave-3 engine change. Left as a
//!   documented follow-up (parallel to the deferred `FindNoReturnFunctionsAnalyzer`).
//! - We skip `DW_TAG_label`, `DW_TAG_call_site`, inlined-subroutine, lexical-block
//!   comments, and source-info/plate comments — all listing cosmetics with zero
//!   decompiler-output payoff (the same scope as the strings/demangle losses).
//! - Type recovery is name/signature-level: a `DW_TAG_structure_type` maps to a
//!   *named opaque* struct (enough to render `struct foo *`), not a per-field
//!   layout reconstruction.
//!
//! ## PIE / load-bias limitation
//!
//! kuna's loader treats a DWARF `DW_AT_low_pc` as the runtime VMA verbatim (true
//! for the vendored PIE `cet_pie_x86_64`: its DWARF low_pc 0x1357 equals the
//! `.symtab` address). A target with a nonzero load bias would need a base
//! adjustment (Ghidra's `DWARFProgram.getCodeAddress` applies the program
//! image-base); that is a known limitation, asserted against the fixture below.
//!
//! ## Precedence vs `s1_protos` (libproto)
//!
//! cet_pie's DWARF also *declares* external fns (`fprintf`/`fopen`/`malloc`,
//! declaration-only, no low_pc) — this pass SKIPS those so it never fights
//! libproto for the same imports. The pass is registered AFTER `LibProtoPass` in
//! `passes.rs::default_passes()` so that for any name both emit, the DWARF
//! (real source) prototype wins (last-write in `set_function_prototype_pieces`).

use std::collections::BTreeMap;
use std::rc::Rc;

use object::{Object, ObjectSection};

use kuna_base::types::uint4;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage, SymFact, SymKind};

/// gimli's section reader: a byte slice tagged with the run-time endianness.
type Reader<'a> = gimli::EndianSlice<'a, gimli::RunTimeEndian>;

/// Recursion cap for the DIE -> [`Datatype`] mapper, the port of Ghidra's
/// `DWARFDataTypeImporter` `trackRecursion` guard (`DWARFDataTypeImporter.java`).
/// Struct -> pointer -> struct DWARF cycles would otherwise loop forever; at the
/// cap we fall back to a void pointer / void so the prototype still builds.
const MAX_TYPE_DEPTH: u32 = 3;

/// Port of `DWARFAnalyzer`: install DWARF function/global names and typed
/// function signatures from the program's `.debug_*` sections.
pub struct DwarfPass;

/// A flattened snapshot of one DIE's attributes (the subset this pass reads).
///
/// We snapshot the whole compilation unit into an offset-keyed map up front
/// because the type mapper resolves arbitrary `DW_AT_type` references by offset
/// while a separate DFS pass walks subprograms — gimli's cursor only streams
/// forward, so a snapshot is the clean way to random-access type DIEs.
#[derive(Clone)]
struct DieSnap {
    /// The DIE tag (`DW_TAG_*`).
    tag: gimli::DwTag,
    /// `DW_AT_name`, resolved through `.debug_str`/`.debug_line_str` (may be empty).
    name: String,
    /// `DW_AT_low_pc` resolved to a VMA, if present (marks a *defined* function).
    low_pc: Option<u64>,
    /// `DW_AT_type` reference (the offset of the referenced type DIE in this unit).
    type_ref: Option<usize>,
    /// `DW_AT_byte_size` (base_type/pointer/struct sizing).
    byte_size: Option<u64>,
    /// `DW_AT_encoding` (`DW_ATE_*`) for `DW_TAG_base_type`.
    encoding: Option<gimli::DwAte>,
    /// `DW_AT_count`/`DW_AT_upper_bound` (array subrange length).
    array_count: Option<u64>,
    /// True if `DW_AT_declaration` is set (a declaration-only DIE — skip).
    declaration: bool,
    /// True if the DIE carries a `DW_AT_location` (a global var has a real address).
    has_location: bool,
    /// `DW_OP_addr` operand of a simple `DW_AT_location`, if that is its form.
    addr_location: Option<u64>,
    /// Depth in the DIE tree (root unit DIE = 0).
    depth: isize,
    /// Offsets of this DIE's direct children, in order.
    children: Vec<usize>,
}

impl DieSnap {
    /// An empty snapshot for the given tag/depth (gimli's `DwTag` has no
    /// `Default`, so we cannot derive it).
    fn new(tag: gimli::DwTag, depth: isize) -> Self {
        DieSnap {
            tag,
            name: String::new(),
            low_pc: None,
            type_ref: None,
            byte_size: None,
            encoding: None,
            array_count: None,
            declaration: false,
            has_location: false,
            addr_location: None,
            depth,
            children: Vec::new(),
        }
    }
}

/// Snapshot every DIE in `unit` into an offset-keyed map (plus the ordered
/// top-level offsets). Mirrors building Ghidra's `DebugInfoEntry` tree before
/// `DWARFFunctionImporter.importFunctions()` walks it.
fn snapshot_unit(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
) -> BTreeMap<usize, DieSnap> {
    let mut map: BTreeMap<usize, DieSnap> = BTreeMap::new();
    // Parent stack of (offset, depth) to attach children to their parent.
    let mut stack: Vec<(usize, isize)> = Vec::new();

    let mut cursor = unit.entries();
    while let Ok(Some(entry)) = cursor.next_dfs() {
        let off = entry.offset().0;
        let depth = entry.depth();

        // Pop the parent stack back to this DIE's parent level.
        while let Some(&(_, d)) = stack.last() {
            if d >= depth {
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(&(parent_off, _)) = stack.last() {
            if let Some(p) = map.get_mut(&parent_off) {
                p.children.push(off);
            }
        }

        let mut snap = DieSnap::new(entry.tag(), depth);

        if let Some(val) = entry.attr_value(gimli::DW_AT_name) {
            if let Ok(s) = dwarf.attr_string(unit, val) {
                if let Ok(s) = std::str::from_utf8(s.slice()) {
                    snap.name = s.to_string();
                }
            }
        }
        if let Some(val) = entry.attr_value(gimli::DW_AT_low_pc) {
            if let Ok(Some(a)) = dwarf.attr_address(unit, val) {
                snap.low_pc = Some(a);
            }
        }
        if let Some(gimli::AttributeValue::UnitRef(o)) = entry.attr_value(gimli::DW_AT_type) {
            snap.type_ref = Some(o.0);
        }
        if let Some(v) = entry.attr_value(gimli::DW_AT_byte_size) {
            snap.byte_size = v.udata_value();
        }
        if let Some(gimli::AttributeValue::Encoding(e)) = entry.attr_value(gimli::DW_AT_encoding) {
            snap.encoding = Some(e);
        }
        // Array length: prefer DW_AT_count; else DW_AT_upper_bound + 1. The
        // subrange child of a DW_TAG_array_type carries it.
        if let Some(c) = entry.attr_value(gimli::DW_AT_count).and_then(|v| v.udata_value()) {
            snap.array_count = Some(c);
        } else if let Some(ub) =
            entry.attr_value(gimli::DW_AT_upper_bound).and_then(|v| v.udata_value())
        {
            snap.array_count = Some(ub + 1);
        }
        if matches!(entry.attr_value(gimli::DW_AT_declaration), Some(gimli::AttributeValue::Flag(true)))
        {
            snap.declaration = true;
        }
        if let Some(loc) = entry.attr_value(gimli::DW_AT_location) {
            snap.has_location = true;
            snap.addr_location = simple_addr_location(&loc);
        }

        map.insert(off, snap);
        if entry.has_children() {
            stack.push((off, depth));
        }
    }
    map
}

/// Decode a `DW_AT_location` that is a single `DW_OP_addr <vma>` expression (the
/// only global-variable location form this pass handles), returning the VMA.
/// Anything else (`DW_OP_fbreg`, register, multi-op) returns `None` — those are
/// stack/register locals, the deferred subtask-3 territory.
fn simple_addr_location(loc: &gimli::AttributeValue<Reader<'_>>) -> Option<u64> {
    let expr = match loc {
        gimli::AttributeValue::Exprloc(e) => e.clone(),
        gimli::AttributeValue::Block(b) => gimli::Expression(b.clone()),
        _ => return None,
    };
    // A DW_OP_addr expression is the opcode 0x03 followed by an address-sized
    // operand. Parse it with a temporary 64-bit encoding (address_size only
    // affects DW_OP_addr operand width); we only accept the single-op case.
    let bytes = expr.0.slice();
    if bytes.first() != Some(&0x03) {
        return None;
    }
    match bytes.len() {
        5 => Some(u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as u64),
        9 => Some(u64::from_le_bytes([
            bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
        ])),
        _ => None,
    }
}

/// Build the kuna [`Datatype`] for the type DIE at `off`, recursing through the
/// DWARF type chain (faithful reduction of `DWARFDataTypeImporter.getDataType`'s
/// tag switch). `None` for a missing/unbuildable type; the caller skips that one
/// piece rather than failing the analysis. `depth` enforces [`MAX_TYPE_DEPTH`].
fn build_datatype(
    off: Option<usize>,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
    depth: u32,
) -> Option<Rc<Datatype>> {
    // A null DW_AT_type means `void` (the C++ getDataTypeForVariable null case).
    let Some(off) = off else {
        return types.get_type_void().ok();
    };
    if depth >= MAX_TYPE_DEPTH {
        // Recursion cap (trackRecursion): a void pointer keeps a cyclic
        // struct->ptr->struct chain finite while still rendering as a pointer.
        return types.get_type_void().ok();
    }
    let die = dies.get(&off)?;
    let ptr = types.get_size_of_pointer();
    match die.tag {
        gimli::DW_TAG_base_type => {
            let size = die.byte_size.unwrap_or(0) as i32;
            if size <= 0 {
                return None;
            }
            match die.encoding {
                Some(gimli::DW_ATE_signed_char) | Some(gimli::DW_ATE_unsigned_char) => {
                    types.get_type_char(size).ok()
                }
                Some(gimli::DW_ATE_boolean) => types.get_base(size, type_metatype::TYPE_BOOL).ok(),
                Some(gimli::DW_ATE_float) => types.get_base(size, type_metatype::TYPE_FLOAT).ok(),
                Some(gimli::DW_ATE_unsigned) => types.get_base(size, type_metatype::TYPE_UINT).ok(),
                // DW_ATE_signed (and anything else) -> signed int.
                _ => types.get_base(size, type_metatype::TYPE_INT).ok(),
            }
        }
        gimli::DW_TAG_pointer_type => {
            // makeDataTypeForPointer: pointer to the (possibly null=void) pointee.
            let pointee = build_datatype(die.type_ref, dies, types, word_size, depth + 1)
                .or_else(|| types.get_type_void().ok())?;
            let psize = die.byte_size.map(|b| b as i32).unwrap_or(ptr);
            types.get_type_pointer(psize, pointee, word_size).ok()
        }
        // typedef/const/volatile/restrict: transparent — pass through to the
        // underlying DW_AT_type (a null underlying => void).
        gimli::DW_TAG_typedef
        | gimli::DW_TAG_const_type
        | gimli::DW_TAG_volatile_type
        | gimli::DW_TAG_restrict_type => {
            build_datatype(die.type_ref, dies, types, word_size, depth + 1)
        }
        gimli::DW_TAG_array_type => {
            let elem = build_datatype(die.type_ref, dies, types, word_size, depth + 1)?;
            // The length lives on a DW_TAG_subrange_type child (DW_AT_count or
            // upper_bound+1); fall back to 1 for a flexible/unknown array.
            let count = die
                .children
                .iter()
                .filter_map(|c| dies.get(c))
                .find(|c| c.tag == gimli::DW_TAG_subrange_type)
                .and_then(|c| c.array_count)
                .or(die.array_count)
                .unwrap_or(1) as i32;
            types.get_type_array(count.max(1), elem).ok()
        }
        gimli::DW_TAG_structure_type => {
            // A named opaque struct is enough for a pointer-to-struct to render.
            let n = if die.name.is_empty() { "anon_struct" } else { &die.name };
            types.get_type_struct(n).ok()
        }
        gimli::DW_TAG_union_type => {
            let n = if die.name.is_empty() { "anon_union" } else { &die.name };
            types.get_type_union(n).ok()
        }
        gimli::DW_TAG_enumeration_type => {
            // Render an enum as its underlying integer (size from byte_size).
            let size = die.byte_size.map(|b| b as i32).unwrap_or(4).max(1);
            types.get_base(size, type_metatype::TYPE_INT).ok()
        }
        // Any other tag (e.g. subroutine_type) -> give up on this type cleanly.
        _ => None,
    }
}

/// Build [`PrototypePieces`] for a defined subprogram DIE (`DWARFFunction.read` +
/// `getFunctionParamList`): return type from `DW_AT_type`, parameter types/names
/// from `DW_TAG_formal_parameter` children, `first_var_arg_slot` from a trailing
/// `DW_TAG_unspecified_parameters`. Returns `None` if any required type can't be
/// built (the whole prototype is then skipped — never a hard failure).
fn build_pieces(
    name: &str,
    sub: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> Option<PrototypePieces> {
    // Return type: a null DW_AT_type is `void` (build_datatype handles None).
    let outtype = build_datatype(sub.type_ref, dies, types, word_size, 0);

    let mut intypes = Vec::new();
    let mut innames = Vec::new();
    let mut first_var_arg_slot: i32 = -1;

    for &coff in &sub.children {
        let Some(child) = dies.get(&coff) else { continue };
        match child.tag {
            gimli::DW_TAG_formal_parameter => {
                let ty = build_datatype(child.type_ref, dies, types, word_size, 0)?;
                intypes.push(ty);
                innames.push(child.name.clone());
            }
            gimli::DW_TAG_unspecified_parameters => {
                // `...` — variadic from the current fixed-parameter count.
                first_var_arg_slot = intypes.len() as i32;
            }
            _ => {}
        }
    }

    Some(PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes,
        innames,
        first_var_arg_slot,
        output_storage: None,
    })
}

impl AnalysisPass for DwarfPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "dwarf"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if !matches!(ctx.file.format(), object::BinaryFormat::Elf) {
            return out;
        }
        // DWARFProgram.isDWARF: no .debug_info => not a DWARF program, empty out.
        if ctx.file.section_by_name(".debug_info").is_none() {
            return out;
        }

        let endian = if ctx.file.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        // Own every section's bytes so the gimli readers can borrow them. A
        // missing section reads as empty (gimli treats that as "section absent").
        let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
            match ctx.file.section_by_name(id.name()) {
                Some(sec) => Ok(sec.uncompressed_data().map(|d| d.into_owned()).unwrap_or_default()),
                None => Ok(Vec::new()),
            }
        };
        let Ok(sections) = gimli::DwarfSections::load(load) else {
            return out;
        };
        let dwarf = sections.borrow(|bytes| gimli::EndianSlice::new(bytes, endian));

        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();

        let mut units = dwarf.units();
        while let Ok(Some(header)) = units.next() {
            let Ok(unit) = dwarf.unit(header) else { continue };
            let dies = snapshot_unit(&dwarf, &unit);

            for snap in dies.values() {
                match snap.tag {
                    gimli::DW_TAG_subprogram => {
                        // Defined function only: DW_AT_low_pc present and not a
                        // declaration-only DIE (DWARFFunction.read body-ranges guard).
                        let Some(low_pc) = snap.low_pc else { continue };
                        if snap.declaration || snap.name.is_empty() {
                            continue;
                        }
                        out.symbols.push(SymFact {
                            addr: low_pc,
                            name: snap.name.clone(),
                            kind: SymKind::Function,
                        });
                        // subtask 2: typed signature. A prototype that can't be
                        // fully typed is skipped (never fails the analysis).
                        if let Some(pieces) =
                            build_pieces(&snap.name, snap, &dies, types, word_size)
                        {
                            out.prototypes.push(pieces);
                        }
                    }
                    gimli::DW_TAG_variable => {
                        // A CU top-level global with a DW_OP_addr location
                        // (DWARFFunctionImporter.outputGlobal). depth==1 == direct
                        // child of the CU root DIE (no subprogram ancestor).
                        if snap.depth != 1 || snap.name.is_empty() {
                            continue;
                        }
                        if let Some(addr) = snap.addr_location {
                            out.symbols.push(SymFact {
                                addr,
                                name: snap.name.clone(),
                                kind: SymKind::Data,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse the DWARF snapshot of a fixture and return the subprogram DIEs that
    /// are *defined* (low_pc + not a declaration), as (name, low_pc).
    fn defined_subprograms(path: &str) -> Vec<(String, u64)> {
        let bytes = std::fs::read(path).expect("read dwarf fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
        let endian = if file.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };
        let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
            Ok(file
                .section_by_name(id.name())
                .and_then(|s| s.uncompressed_data().ok())
                .map(|d| d.into_owned())
                .unwrap_or_default())
        };
        let sections = gimli::DwarfSections::load(load).expect("load dwarf");
        let dwarf = sections.borrow(|b| gimli::EndianSlice::new(b, endian));
        let mut found = Vec::new();
        let mut units = dwarf.units();
        while let Ok(Some(header)) = units.next() {
            let unit = dwarf.unit(header).expect("unit");
            let dies = snapshot_unit(&dwarf, &unit);
            for snap in dies.values() {
                if snap.tag == gimli::DW_TAG_subprogram
                    && !snap.declaration
                    && !snap.name.is_empty()
                {
                    if let Some(low_pc) = snap.low_pc {
                        found.push((snap.name.clone(), low_pc));
                    }
                }
            }
        }
        found
    }

    #[test]
    fn dwarf_stripped_recovers_function_names_and_addrs() {
        // dwarf_stripped_x86_64 has FUNC names ONLY in DWARF (.symtab stripped):
        // add_values@0x401136, compute@0x401153, main@0x401198. `printf` is
        // declaration-only (no low_pc) and must NOT appear.
        let path =
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/dwarf_stripped_x86_64");
        let defined = defined_subprograms(path);
        let by_name: BTreeMap<_, _> = defined.iter().cloned().collect();
        assert_eq!(by_name.get("add_values"), Some(&0x401136), "add_values low_pc");
        assert_eq!(by_name.get("compute"), Some(&0x401153), "compute low_pc");
        assert_eq!(by_name.get("main"), Some(&0x401198), "main low_pc");
        assert!(
            !by_name.contains_key("printf"),
            "declaration-only `printf` must be skipped, got: {defined:?}"
        );
    }

    #[test]
    fn cet_pie_recovers_elaborate_debug_symbol() {
        // cet_pie_x86_64 (not stripped, DWARF 5): elaborate_debug_symbol @ 0x1357
        // is a defined subprogram. (Names already come from .symtab here; the DWARF
        // value is the TYPED signature, asserted end-to-end in verify_s1_dwarf.rs.)
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cet_pie_x86_64");
        let defined = defined_subprograms(path);
        let by_name: BTreeMap<_, _> = defined.iter().cloned().collect();
        assert_eq!(
            by_name.get("elaborate_debug_symbol"),
            Some(&0x1357),
            "elaborate_debug_symbol low_pc (PIE: DWARF low_pc == runtime VMA)"
        );
        // Confirm the headline param type chain resolves to a char pointer
        // structurally (the engine-level render is the e2e test). We re-snapshot
        // and walk the formal_parameter -> pointer -> const -> char(signed_char).
        let bytes = std::fs::read(path).unwrap();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let endian = gimli::RunTimeEndian::Little;
        let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
            Ok(file
                .section_by_name(id.name())
                .and_then(|s| s.uncompressed_data().ok())
                .map(|d| d.into_owned())
                .unwrap_or_default())
        };
        let sections = gimli::DwarfSections::load(load).unwrap();
        let dwarf = sections.borrow(|b| gimli::EndianSlice::new(b, endian));
        let mut units = dwarf.units();
        let mut found_charptr = false;
        while let Ok(Some(header)) = units.next() {
            let unit = dwarf.unit(header).unwrap();
            let dies = snapshot_unit(&dwarf, &unit);
            for snap in dies.values() {
                if snap.tag == gimli::DW_TAG_subprogram && snap.name == "elaborate_debug_symbol" {
                    // First formal parameter -> follow type to a base char.
                    let pcoff = snap
                        .children
                        .iter()
                        .find(|c| dies.get(c).map(|d| d.tag) == Some(gimli::DW_TAG_formal_parameter))
                        .copied();
                    if let Some(pcoff) = pcoff {
                        let p = &dies[&pcoff];
                        // pointer
                        let pt = dies.get(&p.type_ref.unwrap()).unwrap();
                        assert_eq!(pt.tag, gimli::DW_TAG_pointer_type, "param is a pointer");
                        // -> const -> char base_type
                        let ct = dies.get(&pt.type_ref.unwrap()).unwrap();
                        let base = dies.get(&ct.type_ref.unwrap()).unwrap();
                        assert_eq!(base.tag, gimli::DW_TAG_base_type);
                        assert_eq!(base.name, "char");
                        found_charptr = true;
                    }
                }
            }
        }
        assert!(found_charptr, "elaborate_debug_symbol's first param should be char *");
    }
}
