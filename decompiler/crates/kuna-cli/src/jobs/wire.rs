//! Versioned worker records, independent of process scheduling.
//!
//! Specs carry the parent's resolved inventory. Results use length-prefixed,
//! little-endian frames flushed per function; decoding retains the complete
//! prefix when a worker dies mid-write.

use std::io::{BufWriter, Write};

use kuna_console::engine::{EntryProvenance, ObjectLocation};
use kuna_console::project::FuncResult;
use kuna_decomp::decompile_drive::{GlobalInfo, LineMapping, TypeInfo, VarInfo};
use kuna_decomp::kuna_structsynth::shard::FunctionRecord;

use super::TargetSpec;

const SPEC_MAGIC: &[u8; 12] = b"KUNAJOBSPEC3";
pub(super) const RESULT_MAGIC: &[u8; 12] = b"KUNAJOBRES07";

/// Result-stream frame kind.  One kind today; the envelope is what lets a
/// truncated tail be dropped rather than guessed.
const FRAME_RESULT: u8 = 1;

// --- wire primitives ---------------------------------------------------------

fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_str(out: &mut Vec<u8>, s: &str) {
    put_u32(out, s.len() as u32);
    out.extend_from_slice(s.as_bytes());
}

fn put_opt_str(out: &mut Vec<u8>, s: Option<&str>) {
    match s {
        Some(s) => {
            out.push(1);
            put_str(out, s);
        }
        None => out.push(0),
    }
}

fn put_u64s(out: &mut Vec<u8>, values: &[u64]) {
    put_u32(out, values.len() as u32);
    for &v in values {
        put_u64(out, v);
    }
}

fn put_object_location(out: &mut Vec<u8>, location: Option<&ObjectLocation>) {
    match location {
        Some(location) => {
            out.push(1);
            put_u64(out, location.section_index as u64);
            put_str(out, &location.section);
            put_u64(out, location.offset);
        }
        None => out.push(0),
    }
}

fn provenance_code(p: EntryProvenance) -> u8 {
    match p {
        EntryProvenance::Mapped => 0,
        EntryProvenance::DefinedObject => 1,
        EntryProvenance::UndefinedExternal => 2,
    }
}

fn provenance_of(code: u8) -> Option<EntryProvenance> {
    match code {
        0 => Some(EntryProvenance::Mapped),
        1 => Some(EntryProvenance::DefinedObject),
        2 => Some(EntryProvenance::UndefinedExternal),
        _ => None,
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let out = self.bytes.get(self.pos..end)?;
        self.pos = end;
        Some(out)
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }

    fn i64(&mut self) -> Option<i64> {
        Some(i64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }

    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }

    /// A count read off the wire is untrusted: reserve for it only as far as the
    /// bytes actually left could possibly justify (the smallest element any
    /// count here governs is a 4-byte length prefix), so a corrupt frame cannot
    /// turn into a multi-hundred-gigabyte allocation.
    fn sized<T>(&self, count: usize) -> Vec<T> {
        Vec::with_capacity(count.min((self.bytes.len() - self.pos) / 4))
    }

    fn string(&mut self) -> Option<String> {
        let n = self.u32()? as usize;
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }

    fn opt_string(&mut self) -> Option<Option<String>> {
        match self.u8()? {
            0 => Some(None),
            1 => Some(Some(self.string()?)),
            _ => None,
        }
    }

    fn u64s(&mut self) -> Option<Vec<u64>> {
        let n = self.u32()? as usize;
        let mut out = self.sized(n);
        for _ in 0..n {
            out.push(self.u64()?);
        }
        Some(out)
    }

    fn object_location(&mut self) -> Option<Option<ObjectLocation>> {
        match self.u8()? {
            0 => Some(None),
            1 => Some(Some(ObjectLocation {
                section_index: self.u64()? as usize,
                section: self.string()?,
                offset: self.u64()?,
            })),
            _ => None,
        }
    }
}

// --- chunk spec (parent → worker) --------------------------------------------

pub(super) fn encode_spec(targets: &[TargetSpec]) -> Vec<u8> {
    let mut out = Vec::with_capacity(64 * targets.len() + SPEC_MAGIC.len() + 4);
    out.extend_from_slice(SPEC_MAGIC);
    put_u32(&mut out, targets.len() as u32);
    for t in targets {
        put_u64(&mut out, t.addr);
        put_str(&mut out, &t.space);
        put_str(&mut out, &t.name);
        put_u32(&mut out, t.aliases.len() as u32);
        for a in &t.aliases {
            put_str(&mut out, a);
        }
        put_u64(&mut out, t.size);
        put_object_location(&mut out, t.object_location.as_ref());
        out.push(provenance_code(t.provenance));
        put_opt_str(&mut out, t.binding.as_deref());
        match &t.synth {
            Some(answers) => {
                out.push(1);
                put_u32(&mut out, answers.len() as u32);
                for a in answers {
                    put_opt_str(&mut out, a.as_deref());
                }
            }
            None => out.push(0),
        }
    }
    out
}

/// Decode a worker's chunk spec (the worker side of [`encode_spec`]).
pub(crate) fn read_spec(path: &str) -> Result<Vec<TargetSpec>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read chunk spec {path}: {e}"))?;
    decode_spec(&bytes).ok_or_else(|| format!("malformed chunk spec {path}"))
}

pub(super) fn decode_spec(bytes: &[u8]) -> Option<Vec<TargetSpec>> {
    let mut r = Reader { bytes, pos: 0 };
    if r.take(SPEC_MAGIC.len())? != SPEC_MAGIC {
        return None;
    }
    let n = r.u32()? as usize;
    let mut out = r.sized(n);
    for _ in 0..n {
        let addr = r.u64()?;
        let space = r.string()?;
        let name = r.string()?;
        let na = r.u32()? as usize;
        let mut aliases = r.sized(na);
        for _ in 0..na {
            aliases.push(r.string()?);
        }
        let size = r.u64()?;
        let object_location = r.object_location()?;
        let provenance = provenance_of(r.u8()?)?;
        let binding = r.opt_string()?;
        let synth = match r.u8()? {
            0 => None,
            1 => {
                let n = r.u32()? as usize;
                let mut answers = r.sized(n);
                for _ in 0..n {
                    answers.push(r.opt_string()?);
                }
                Some(answers)
            }
            _ => return None,
        };
        out.push(TargetSpec {
            addr,
            space,
            name,
            aliases,
            size,
            object_location,
            provenance,
            binding,
            synth,
        });
    }
    Some(out)
}

// --- result stream (worker → parent) -----------------------------------------

/// A worker's incremental result writer: one length-prefixed frame per function,
/// flushed as it is produced so a worker killed mid-chunk still delivers every
/// function it had finished.
pub(crate) struct ResultWriter {
    out: BufWriter<std::fs::File>,
}

impl ResultWriter {
    pub(crate) fn create(path: &str) -> Result<Self, String> {
        let file = std::fs::File::create(path)
            .map_err(|e| format!("cannot create worker result file {path}: {e}"))?;
        let mut out = BufWriter::new(file);
        out.write_all(RESULT_MAGIC).map_err(|e| format!("worker result write failed: {e}"))?;
        Ok(Self { out })
    }

    pub(crate) fn push(&mut self, r: &FuncResult) -> Result<(), String> {
        let mut body = Vec::with_capacity(256);
        put_u64(&mut body, r.address);
        put_u64(&mut body, r.byte_address);
        body.extend_from_slice(&r.size.to_le_bytes());
        put_str(&mut body, &r.name);
        put_opt_str(&mut body, r.code.as_deref());
        put_opt_str(&mut body, r.error.as_deref());
        put_opt_str(&mut body, r.proto.as_deref());
        put_object_location(&mut body, r.object_location.as_ref());
        put_u32(&mut body, r.aliases.len() as u32);
        for a in &r.aliases {
            put_str(&mut body, a);
        }
        put_u32(&mut body, r.line_mappings.len() as u32);
        for m in &r.line_mappings {
            put_u64(&mut body, m.line_number as u64);
            put_u64s(&mut body, &m.addresses);
        }
        put_u32(&mut body, r.variables.len() as u32);
        for v in &r.variables {
            put_str(&mut body, &v.name);
            put_str(&mut body, &v.type_name);
            body.push(u8::from(v.is_param));
            match v.arg_index {
                Some(i) => {
                    body.push(1);
                    put_u64(&mut body, i as u64);
                }
                None => body.push(0),
            }
            match v.stack_offset {
                Some(o) => {
                    body.push(1);
                    body.extend_from_slice(&o.to_le_bytes());
                }
                None => body.push(0),
            }
            body.extend_from_slice(&v.size.to_le_bytes());
            put_u32(&mut body, v.line_numbers.len() as u32);
            for &n in &v.line_numbers {
                put_u64(&mut body, n as u64);
            }
            put_u64s(&mut body, &v.addresses);
        }
        // (kuna `structdefs`) The recovered type definitions travel with the
        // record: a pooled worker renders the C, so the parent has no `Funcdata`
        // left to re-derive them from.
        put_u32(&mut body, r.types.len() as u32);
        for t in &r.types {
            put_str(&mut body, &t.name);
            put_str(&mut body, &t.definition);
            body.extend_from_slice(&t.size.to_le_bytes());
        }
        put_u32(&mut body, r.stack_objects.len() as u32);
        for object in &r.stack_objects {
            put_str(&mut body, &object.id);
            put_str(&mut body, &object.name);
            body.extend_from_slice(&object.stack_offset.to_le_bytes());
            body.extend_from_slice(&object.size.to_le_bytes());
            body.push(u8::from(object.defined));
            put_u32(&mut body, object.uses.len() as u32);
            for use_ in &object.uses {
                put_u64(&mut body, use_.address);
                body.extend_from_slice(&use_.slot.to_le_bytes());
                put_str(&mut body, &use_.type_name);
            }
        }
        // (kuna `globalref`) The globals the body names by address, for the
        // project header the parent writes.
        put_u32(&mut body, r.globals.len() as u32);
        for g in &r.globals {
            put_u64(&mut body, g.address);
            put_str(&mut body, &g.name);
            put_str(&mut body, &g.declaration);
            body.extend_from_slice(&g.size.to_le_bytes());
            body.push(u8::from(g.unknown) | u8::from(g.direct) << 1 | u8::from(g.aggregate) << 2 | u8::from(g.elem) << 3);
        }
        put_u64s(&mut body, &r.callee_hints);
        match &r.synth {
            Some(record) => {
                body.push(1);
                record.encode(&mut body);
            }
            None => body.push(0),
        }
        put_pointerargs(&mut body, r.pointerargs.as_ref());
        self.frame(FRAME_RESULT, &body)
    }

    fn frame(&mut self, kind: u8, body: &[u8]) -> Result<(), String> {
        let mut frame = Vec::with_capacity(body.len() + 5);
        frame.push(kind);
        put_u32(&mut frame, body.len() as u32);
        frame.extend_from_slice(body);
        self.out.write_all(&frame).map_err(|e| format!("worker result write failed: {e}"))?;
        self.out.flush().map_err(|e| format!("worker result flush failed: {e}"))
    }
}

/// Decode every complete frame in a worker result file, ignoring a truncated
/// tail (a worker killed mid-write).  `None` only when the magic is absent.
pub(super) fn decode_results(bytes: &[u8]) -> Option<Vec<FuncResult>> {
    let mut r = Reader { bytes, pos: 0 };
    if r.take(RESULT_MAGIC.len())? != RESULT_MAGIC {
        return None;
    }
    let mut out = Vec::new();
    while let Some(kind) = r.u8() {
        let Some(len) = r.u32() else { break };
        let Some(body) = r.take(len as usize) else { break };
        match kind {
            FRAME_RESULT => match decode_one(body) {
                Some(rec) => out.push(rec),
                None => break,
            },
            _ => break,
        }
    }
    Some(out)
}

fn decode_one(body: &[u8]) -> Option<FuncResult> {
    let mut r = Reader { bytes: body, pos: 0 };
    let address = r.u64()?;
    let byte_address = r.u64()?;
    let size = r.i64()?;
    let name = r.string()?;
    let code = r.opt_string()?;
    let error = r.opt_string()?;
    let proto = r.opt_string()?;
    let object_location = r.object_location()?;
    let na = r.u32()? as usize;
    let mut aliases = r.sized(na);
    for _ in 0..na {
        aliases.push(r.string()?);
    }
    let nm = r.u32()? as usize;
    let mut line_mappings = r.sized(nm);
    for _ in 0..nm {
        line_mappings.push(LineMapping {
            line_number: r.u64()? as usize,
            addresses: r.u64s()?,
        });
    }
    let nv = r.u32()? as usize;
    let mut variables = r.sized(nv);
    for _ in 0..nv {
        let vname = r.string()?;
        let type_name = r.string()?;
        let is_param = r.u8()? != 0;
        let arg_index = match r.u8()? {
            0 => None,
            1 => Some(r.u64()? as usize),
            _ => return None,
        };
        let stack_offset = match r.u8()? {
            0 => None,
            1 => Some(r.i64()?),
            _ => return None,
        };
        let vsize = r.i64()?;
        let nl = r.u32()? as usize;
        let mut line_numbers = r.sized(nl);
        for _ in 0..nl {
            line_numbers.push(r.u64()? as usize);
        }
        let addresses = r.u64s()?;
        variables.push(VarInfo {
            name: vname,
            type_name,
            stack_offset,
            size: vsize,
            is_param,
            arg_index,
            line_numbers,
            addresses,
        });
    }
    let nt = r.u32()? as usize;
    let mut types = r.sized(nt);
    for _ in 0..nt {
        let tname = r.string()?;
        let definition = r.string()?;
        let tsize = r.i64()?;
        types.push(TypeInfo { name: tname, definition, size: tsize });
    }
    let no = r.u32()? as usize;
    let mut stack_objects = r.sized(no);
    for _ in 0..no {
        let id = r.string()?;
        let name = r.string()?;
        let stack_offset = r.i64()?;
        let size = r.i64()?;
        let defined = match r.u8()? {
            0 => false,
            1 => true,
            _ => return None,
        };
        let nu = r.u32()? as usize;
        let mut uses = r.sized(nu);
        for _ in 0..nu {
            uses.push(kuna_decomp::kuna_stackobjectinfo::StackObjectUseInfo {
                address: r.u64()?,
                slot: r.u32()? as i32,
                type_name: r.string()?,
            });
        }
        stack_objects.push(kuna_decomp::kuna_stackobjectinfo::StackObjectInfo {
            id, name, stack_offset, size, defined, uses,
        });
    }
    let ng = r.u32()? as usize;
    let mut globals = r.sized(ng);
    for _ in 0..ng {
        let address = r.u64()?;
        let gname = r.string()?;
        let declaration = r.string()?;
        let gsize = r.i64()?;
        let bits = r.u8()?;
        globals.push(GlobalInfo {
            address,
            name: gname,
            declaration,
            size: gsize,
            unknown: bits & 1 != 0,
            direct: bits & 2 != 0,
            aggregate: bits & 4 != 0,
            elem: bits & 8 != 0,
        });
    }
    let callee_hints = r.u64s()?;
    let synth = match r.u8()? {
        0 => None,
        1 => {
            let (record, consumed) = FunctionRecord::decode(&body[r.pos..])?;
            r.take(consumed)?;
            Some(record)
        }
        _ => return None,
    };
    let pointerargs = read_pointerargs(&mut r)?;
    if r.pos != body.len() { return None; }
    Some(FuncResult {
        name,
        address,
        byte_address,
        size,
        code,
        error,
        proto,
        variables,
        stack_objects,
        types,
        globals,
        line_mappings,
        aliases,
        object_location,
        callee_hints,
        synth,
        pointerargs,
        detail: None,
    })
}

fn put_entry(out: &mut Vec<u8>, entry: &kuna_decomp::kuna_pointerargs::Entry) {
    put_str(out, &entry.0);
    put_u64(out, entry.1);
}

fn put_storage(out: &mut Vec<u8>, storage: &kuna_decomp::kuna_pointerargs::Storage) {
    put_entry(out, &storage.0);
    out.extend_from_slice(&i64::from(storage.1).to_le_bytes());
}

fn put_pointerargs(out: &mut Vec<u8>, record: Option<&kuna_decomp::kuna_pointerargs::Record>) {
    let Some(record) = record else { out.push(0); return };
    out.push(1);
    put_entry(out, &record.entry);
    put_u32(out, record.parameters.len() as u32);
    for parameter in &record.parameters {
        match parameter {
            None => out.push(0),
            Some(parameter) => {
                out.push(1);
                put_storage(out, &parameter.storage);
                put_str(out, &parameter.spelling);
            }
        }
    }
    put_u32(out, record.calls.len() as u32);
    for call in &record.calls {
        put_entry(out, &call.callee);
        put_u64(out, call.index as u64);
        put_storage(out, &call.storage);
        put_str(out, &call.actual);
        put_u64(out, call.position.line as u64);
        put_u64(out, call.position.column as u64);
        put_str(out, &call.expression);
    }
}

fn read_entry(r: &mut Reader<'_>) -> Option<kuna_decomp::kuna_pointerargs::Entry> {
    Some((r.string()?, r.u64()?))
}

fn read_storage(r: &mut Reader<'_>) -> Option<kuna_decomp::kuna_pointerargs::Storage> {
    Some((read_entry(r)?, r.i64()?.try_into().ok()?))
}

fn read_pointerargs(r: &mut Reader<'_>) -> Option<Option<kuna_decomp::kuna_pointerargs::Record>> {
    use kuna_decomp::kuna_pointerargs::{Call, Parameter, Position, Record};
    match r.u8()? { 0 => return Some(None), 1 => {}, _ => return None }
    let entry = read_entry(r)?;
    let n = r.u32()? as usize;
    let mut parameters = r.sized(n);
    for _ in 0..n {
        parameters.push(match r.u8()? {
            0 => None,
            1 => Some(Parameter { storage: read_storage(r)?, spelling: r.string()? }),
            _ => return None,
        });
    }
    let n = r.u32()? as usize;
    let mut calls = r.sized(n);
    for _ in 0..n {
        calls.push(Call {
            callee: read_entry(r)?, index: r.u64()?.try_into().ok()?,
            storage: read_storage(r)?, actual: r.string()?,
            position: Position { line: r.u64()?.try_into().ok()?, column: r.u64()?.try_into().ok()? },
            expression: r.string()?,
        });
    }
    Some(Some(Record { entry, parameters, calls }))
}
