//! Classic Mach-O bind opcodes (`LC_DYLD_INFO` / `LC_DYLD_INFO_ONLY`): which
//! pointer slots the dynamic loader fills with which imported symbol.
//!
//! Before chained fixups a linked image kept its binds as a byte-coded program
//! (`BIND_OPCODE_*`, `mach-o/loader.h`): set a segment and offset, set a symbol,
//! bind, advance. The slot itself holds zero (or an addend) in the file, so a
//! reader that wants to know what `__objc_classrefs[3]` refers to has to replay
//! the program — the twin of the chained resolver's imports table
//! ([`super::chained`]). Only the names are wanted: ordinals, addends, weak
//! and lazy binds are decoded far enough to stay in step and otherwise ignored.
//! The run-time address of a bind is unknown statically, so nothing is written.

use std::collections::HashMap;

use object::read::macho::{LoadCommandVariant, MachHeader, Segment};
use object::FileKind;

const OPCODE_MASK: u8 = 0xF0;
const IMMEDIATE_MASK: u8 = 0x0F;
const BIND_OPCODE_DONE: u8 = 0x00;
const BIND_OPCODE_SET_DYLIB_ORDINAL_IMM: u8 = 0x10;
const BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB: u8 = 0x20;
const BIND_OPCODE_SET_DYLIB_SPECIAL_IMM: u8 = 0x30;
const BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM: u8 = 0x40;
const BIND_OPCODE_SET_TYPE_IMM: u8 = 0x50;
const BIND_OPCODE_SET_ADDEND_SLEB: u8 = 0x60;
const BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB: u8 = 0x70;
const BIND_OPCODE_ADD_ADDR_ULEB: u8 = 0x80;
const BIND_OPCODE_DO_BIND: u8 = 0x90;
const BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB: u8 = 0xA0;
const BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED: u8 = 0xB0;
const BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB: u8 = 0xC0;
const BIND_OPCODE_THREADED: u8 = 0xD0;

/// The most bind slots one image is followed for.
const MAX_BINDS: usize = 1 << 20;

/// `slot_vma → import symbol name` for every non-lazy bind of a classic
/// `LC_DYLD_INFO` Mach-O. Empty for a chained-fixup image (which has no bind
/// opcodes), a non-Mach-O, or an unparsable program.
pub fn resolve_dyld_info_binds(file: &object::File, bytes: &[u8]) -> HashMap<u64, String> {
    if file.format() != object::BinaryFormat::MachO {
        return HashMap::new();
    }
    match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => collect::<object::macho::MachHeader64<object::Endianness>>(bytes, 8),
        Ok(FileKind::MachO32) => collect::<object::macho::MachHeader32<object::Endianness>>(bytes, 4),
        _ => HashMap::new(),
    }
}

fn collect<Mach>(bytes: &[u8], ptr_size: u64) -> HashMap<u64, String>
where
    Mach: MachHeader<Endian = object::Endianness>,
{
    let mut out = HashMap::new();
    let Ok(header) = Mach::parse(bytes, 0) else { return out };
    let Ok(endian) = header.endian() else { return out };
    let Ok(mut commands) = header.load_commands(endian, bytes, 0) else { return out };
    let mut segments: Vec<u64> = Vec::new();
    let mut programs: Vec<(u32, u32)> = Vec::new();
    while let Ok(Some(command)) = commands.next() {
        if let Ok(Some((segment, _))) = Mach::Segment::from_command(command) {
            segments.push(segment.vmaddr(endian).into());
        } else if let Ok(LoadCommandVariant::DyldInfo(info)) = command.variant() {
            programs.push((info.bind_off.get(endian), info.bind_size.get(endian)));
            programs.push((info.weak_bind_off.get(endian), info.weak_bind_size.get(endian)));
        }
    }
    for (off, size) in programs {
        if let Some(program) = bytes.get(off as usize..(off as usize).saturating_add(size as usize)) {
            replay(program, &segments, ptr_size, &mut out);
        }
    }
    out
}

/// Replay one bind program, recording each `DO_BIND*` slot under the current
/// symbol. Decoding stops at the first malformed byte (never guesses a slot).
fn replay(program: &[u8], segments: &[u64], ptr_size: u64, out: &mut HashMap<u64, String>) {
    let mut p = 0usize;
    let mut addr: Option<u64> = None;
    let mut symbol = String::new();
    let bind = |addr: &mut Option<u64>, symbol: &str, out: &mut HashMap<u64, String>| {
        if let Some(a) = *addr {
            if !symbol.is_empty() {
                out.insert(a, symbol.to_string());
            }
            *addr = Some(a.wrapping_add(ptr_size));
        }
    };
    while let Some(&byte) = program.get(p) {
        // A real image binds some tens of thousands of slots; a program that
        // repeats past this is hostile and is not followed further.
        if out.len() >= MAX_BINDS {
            return;
        }
        p += 1;
        let imm = byte & IMMEDIATE_MASK;
        match byte & OPCODE_MASK {
            BIND_OPCODE_DONE => {}
            BIND_OPCODE_SET_DYLIB_ORDINAL_IMM | BIND_OPCODE_SET_DYLIB_SPECIAL_IMM | BIND_OPCODE_SET_TYPE_IMM => {}
            BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB => {
                let Some(_) = uleb(program, &mut p) else { return };
            }
            BIND_OPCODE_SET_ADDEND_SLEB => {
                let Some(_) = sleb(program, &mut p) else { return };
            }
            BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM => {
                let Some(end) = program[p..].iter().position(|&b| b == 0) else { return };
                symbol = String::from_utf8_lossy(&program[p..p + end]).into_owned();
                p += end + 1;
            }
            BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB => {
                let Some(offset) = uleb(program, &mut p) else { return };
                let Some(&base) = segments.get(imm as usize) else { return };
                addr = Some(base.wrapping_add(offset));
            }
            BIND_OPCODE_ADD_ADDR_ULEB => {
                let Some(delta) = uleb(program, &mut p) else { return };
                addr = addr.map(|a| a.wrapping_add(delta));
            }
            BIND_OPCODE_DO_BIND => bind(&mut addr, &symbol, out),
            BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB => {
                bind(&mut addr, &symbol, out);
                let Some(delta) = uleb(program, &mut p) else { return };
                addr = addr.map(|a| a.wrapping_add(delta));
            }
            BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED => {
                bind(&mut addr, &symbol, out);
                addr = addr.map(|a| a.wrapping_add(imm as u64 * ptr_size));
            }
            BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB => {
                let Some(count) = uleb(program, &mut p) else { return };
                let Some(skip) = uleb(program, &mut p) else { return };
                for _ in 0..count.min(0x10000) {
                    if out.len() >= MAX_BINDS {
                        return;
                    }
                    bind(&mut addr, &symbol, out);
                    addr = addr.map(|a| a.wrapping_add(skip));
                }
            }
            // arm64e threaded binds carry their ordinals in the slot words; out of
            // scope for the classic program, so the rest of it is not trusted.
            BIND_OPCODE_THREADED => return,
            _ => return,
        }
    }
}

fn uleb(program: &[u8], p: &mut usize) -> Option<u64> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        let byte = *program.get(*p)?;
        *p += 1;
        if shift >= 64 {
            return None;
        }
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            return Some(result);
        }
        shift += 7;
    }
}

fn sleb(program: &[u8], p: &mut usize) -> Option<i64> {
    let mut result = 0i64;
    let mut shift = 0u32;
    loop {
        let byte = *program.get(*p)?;
        *p += 1;
        if shift >= 64 {
            return None;
        }
        result |= ((byte & 0x7F) as i64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            if shift < 64 && byte & 0x40 != 0 {
                result |= -1i64 << shift;
            }
            return Some(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A hand-written program: segment 1 at 0x2000, symbol `_a` bound at +0x10 and
    /// again two slots later via IMM_SCALED, then `_b` bound once with ADD_ADDR.
    #[test]
    fn replays_the_common_opcodes() {
        let program = [
            BIND_OPCODE_SET_DYLIB_ORDINAL_IMM | 1,
            BIND_OPCODE_SET_TYPE_IMM | 1,
            BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM, b'_', b'a', 0,
            BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB | 1, 0x10,
            BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED | 1,
            BIND_OPCODE_DO_BIND,
            BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM, b'_', b'b', 0,
            BIND_OPCODE_ADD_ADDR_ULEB, 0x80, 0x01,
            BIND_OPCODE_DO_BIND,
            BIND_OPCODE_DONE,
        ];
        let mut out = HashMap::new();
        replay(&program, &[0x1000, 0x2000], 8, &mut out);
        assert_eq!(out.get(&0x2010).map(String::as_str), Some("_a"));
        assert_eq!(out.get(&0x2020).map(String::as_str), Some("_a"));
        assert_eq!(out.get(&0x20a8).map(String::as_str), Some("_b"));
        assert_eq!(out.len(), 3);
    }

    /// The classic x86-64 `macho_objc` fixture binds `__objc_empty_cache` (the
    /// `class_t.cache` word) and nothing is read from a chained image.
    #[test]
    fn classic_fixture_binds_resolve_and_chained_is_empty() {
        for (name, expect_some) in [("macho_objc", true), ("macho_objc_arm64", false)] {
            let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
            let bytes = std::fs::read(&path).expect("fixture");
            let file = object::File::parse(bytes.as_slice()).expect("parse");
            let binds = resolve_dyld_info_binds(&file, &bytes);
            assert_eq!(!binds.is_empty(), expect_some, "{name}");
            if expect_some {
                assert!(binds.values().any(|n| n == "__objc_empty_cache"), "{name}: {binds:?}");
            }
        }
    }

    /// A program that repeats a bind past the budget is cut off there.
    #[test]
    fn a_repeating_program_is_bounded() {
        let mut program = vec![BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM, b'_', b'a', 0, BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB, 0];
        for _ in 0..40 {
            program.extend_from_slice(&[BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB, 0xff, 0xff, 0x03, 0]);
        }
        let mut out = HashMap::new();
        replay(&program, &[0x1000], 8, &mut out);
        assert_eq!(out.len(), MAX_BINDS);
    }
}
