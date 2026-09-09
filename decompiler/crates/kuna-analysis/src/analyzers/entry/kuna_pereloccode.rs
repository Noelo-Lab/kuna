//! (kuna) The PE **address-taken function** oracle: function starts recovered
//! from the image's own base-relocation table.
//!
//! ## Why the other PE oracles cannot see these
//!
//! [`super::pe_entry`]'s oracles answer for functions the image *describes* —
//! the entry point, an export, a TLS callback, a `.pdata` unwind record — and the
//! recursive-descent walk downstream adds every direct `CALL` target. A function
//! that is only ever reached through a pointer is described by none of them: a VM
//! interpreter's handler table, a C++ vtable slot, a callback registered with the
//! runtime. The pointer is in `.rdata`, the reference is a `LEA`, and if the
//! function is a leaf that MSVC left out of `.pdata` it is invisible to the whole
//! union. The witness is a 30-entry handler table whose entries 20/24/27/28 named
//! four functions the inventory did not have; the run-on bounder then folded each
//! into the function ahead of it, so `sub_140003c50` was reported at 32 bytes
//! when it is 16.
//!
//! ## What the relocation table says
//!
//! A PE's base-relocation directory is a list of the image words that hold an
//! absolute address, emitted so the loader can fix them up when the image lands
//! somewhere other than its preferred base. That makes it an exact answer to the
//! question the scan needs — *which data words are addresses?* — rather than a
//! guess from a byte pattern. A relocated word whose value lands in an executable
//! section is a stored address into executable memory.
//!
//! ## Which of those are function starts
//!
//! Two things that are not: a **label**, and a word that is not code at all.
//!
//! A non-PIC switch jump table is a run of relocated addresses pointing into the
//! middle of the function that switches on them. `.pdata` settles that one — a
//! `RUNTIME_FUNCTION` is the image's own statement that `[BeginAddress,
//! EndAddress)` is one function's body, so a candidate strictly inside such a
//! range is a label and is dropped. A `BeginAddress` itself is kept: it is a
//! start, and the `.pdata` oracle already has it.
//!
//! The second is what a packed or single-section image does to the exec-section
//! test. `jormungandr.exe` merges its payload into `.text`, so a UTF-16 locale
//! table ("hr-HR", "ko-KR", …) sits at an *executable* address and every
//! relocated pointer into it reads as a code pointer — 666 of them. The
//! discriminator is again `.pdata`: its records cover `0x1400091c0` upwards,
//! while every one of those strings is below that. So a candidate must also fall
//! inside the span of its own section that the exception table vouches for. On
//! the witness that costs nothing — `vm.exe`'s records span its `.text` end to
//! end — and it is what keeps the oracle from seeding a packer's data blob.
//!
//! The guard is therefore also the precondition. An image with no readable
//! `[begin, end)` table — a PE32 with no exception directory, or an ARM/ARM64 PE
//! whose 8-byte records carry no `EndAddress` — vouches for no code region at
//! all, so the oracle **abstains** rather than seed thousands of unvetted
//! addresses. That is a deliberate scope limit, not an oversight: 72 of the 152
//! PE images swept for this pass are PE32 without `.pdata`, and one of them
//! carries 3287 relocated code addresses.

use object::pe::{
    IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386, IMAGE_REL_BASED_DIR64,
    IMAGE_REL_BASED_HIGHLOW,
};
use object::read::pe::{ImageNtHeaders, PeFile};
use object::LittleEndian as LE;

/// Push every relocated absolute address the exception table vouches for as a
/// function start: inside an executable section's `.pdata`-covered span, and not
/// interior to one of the bodies that table describes.
///
/// `ptr` is the image's pointer width in bytes (8 for PE32+, 4 for PE32); only
/// the relocation kind of that width is honoured. `execs` is the caller's
/// executable-section table (`super::executable_sections`).
///
/// Pure & total, like the rest of the entry oracles: a missing directory, an
/// unreadable block or an RVA no section covers yields fewer candidates, never
/// an error.
pub(super) fn reloc_code_pointers<Pe: ImageNtHeaders>(
    pe: &PeFile<Pe>,
    bytes: &[u8],
    image_base: u64,
    ptr: usize,
    execs: &[(u64, u64, Vec<u8>)],
    out: &mut Vec<u64>,
) {
    let code = CodeRegions::describe(pe, bytes, image_base, execs);
    if code.hulls.is_empty() {
        return;
    }
    let sections = pe.section_table();
    let blocks = match pe.data_directories().relocation_blocks(bytes, &sections) {
        Ok(Some(b)) => b,
        _ => return,
    };
    for block in blocks.flatten() {
        for rel in block {
            let width = match rel.typ {
                IMAGE_REL_BASED_DIR64 => 8usize,
                IMAGE_REL_BASED_HIGHLOW => 4usize,
                _ => continue,
            };
            if width != ptr {
                continue;
            }
            let slot = match sections.pe_data_at(bytes, rel.virtual_address) {
                Some(s) => s,
                None => continue,
            };
            let va = match read_word(slot, width) {
                Some(v) if v != 0 => v,
                _ => continue,
            };
            if code.is_function_start(va) {
                out.push(va);
            }
        }
    }
}

/// What the exception directory says about where code is: the described function
/// bodies, and the span of each executable section those bodies fall in.
struct CodeRegions {
    /// `[begin, end)` per described body, disjoint and ascending.
    bodies: Vec<(u64, u64)>,
    /// Per executable section carrying at least one body, the span from its
    /// lowest `BeginAddress` to its highest `EndAddress`.
    hulls: Vec<(u64, u64)>,
}

impl CodeRegions {
    /// Read the exception directory. Both lists come back empty when the
    /// directory is absent, empty, or in the 8-byte ARM form (which carries no
    /// `EndAddress`) — the caller reads that as "abstain".
    ///
    /// Chained records are kept: a separated chunk is still one function's body,
    /// so an address inside it is still a label rather than a start.
    fn describe<Pe: ImageNtHeaders>(
        pe: &PeFile<Pe>,
        bytes: &[u8],
        image_base: u64,
        execs: &[(u64, u64, Vec<u8>)],
    ) -> Self {
        let mut me = CodeRegions { bodies: Vec::new(), hulls: Vec::new() };
        // Only the 12-byte x86/x64 `RUNTIME_FUNCTION` carries an `EndAddress`.
        match pe.nt_headers().file_header().machine.get(LE) {
            IMAGE_FILE_MACHINE_I386 | IMAGE_FILE_MACHINE_AMD64 => {}
            _ => return me,
        }
        let dir = match pe
            .data_directories()
            .get(object::pe::IMAGE_DIRECTORY_ENTRY_EXCEPTION)
        {
            Some(d) if d.size.get(LE) != 0 => d,
            _ => return me,
        };
        let data = match dir.data(bytes, &pe.section_table()) {
            Ok(d) => d,
            Err(_) => return me,
        };
        let mut hulls: Vec<Option<(u64, u64)>> = vec![None; execs.len()];
        for rec in data.chunks_exact(12) {
            let begin = u32::from_le_bytes([rec[0], rec[1], rec[2], rec[3]]) as u64;
            let end = u32::from_le_bytes([rec[4], rec[5], rec[6], rec[7]]) as u64;
            if begin == 0 || end <= begin {
                continue;
            }
            let (begin, end) = (image_base.wrapping_add(begin), image_base.wrapping_add(end));
            me.bodies.push((begin, end));
            if let Some(i) = execs.iter().position(|&(lo, hi, _)| begin >= lo && begin < hi) {
                let h = hulls[i].get_or_insert((begin, end));
                h.0 = h.0.min(begin);
                h.1 = h.1.max(end);
            }
        }
        me.hulls = hulls.into_iter().flatten().collect();
        me.bodies.sort_unstable();
        // Coalesce, so a malformed (nested or out-of-order) table still leaves a
        // disjoint ascending list and the interior test stays a binary search.
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(me.bodies.len());
        for (b, e) in me.bodies.drain(..) {
            match merged.last_mut() {
                Some(last) if b <= last.1 => last.1 = last.1.max(e),
                _ => merged.push((b, e)),
            }
        }
        me.bodies = merged;
        me
    }

    /// Whether a stored code address is a function start: inside a vouched-for
    /// span, and not strictly interior to a described body. `va == begin` is not
    /// interior — that is the function's own start.
    fn is_function_start(&self, va: u64) -> bool {
        if !self.hulls.iter().any(|&(lo, hi)| va >= lo && va < hi) {
            return false;
        }
        let i = self.bodies.partition_point(|&(b, _)| b <= va);
        !matches!(i.checked_sub(1).map(|j| self.bodies[j]), Some((b, e)) if b < va && va < e)
    }
}

/// Read the little-endian word of `width` bytes at the head of `slot`.
fn read_word(slot: &[u8], width: usize) -> Option<u64> {
    let b = slot.get(..width)?;
    let mut v = 0u64;
    for (i, byte) in b.iter().enumerate() {
        v |= (*byte as u64) << (8 * i);
    }
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::super::{collect_entries, executable_sections, in_executable_section};

    const HANDLER: u64 = 0x140001060; // address-taken, described by nothing else
    const LABEL: u64 = 0x140001040; // interior to dispatch's .pdata record
    const PAYLOAD: u64 = 0x1400010a0; // executable, but past everything .pdata vouches for
    const DISPATCH: u64 = 0x140001020;

    fn fixture() -> Vec<u8> {
        let path = format!(
            "{}/tests/fixtures/pe_reloctable_x86_64.exe",
            env!("CARGO_MANIFEST_DIR")
        );
        std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
    }

    /// The headline: a function only ever named by a relocated `.rdata` pointer
    /// is discovered, and the jump-table label sharing that table is not.
    #[test]
    fn relocated_pointer_finds_address_taken_function() {
        let bytes = fixture();
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture PE");
        let entries = collect_entries(&file, bytes.as_slice());
        assert!(
            entries.contains(&HANDLER),
            "address-taken {HANDLER:#x} missing from {entries:#x?}"
        );
        assert!(
            !entries.contains(&LABEL),
            "interior label {LABEL:#x} seeded as a function start"
        );
        assert!(entries.contains(&DISPATCH), "the .pdata oracle still answers");
        let execs = executable_sections(&file);
        for &e in &entries {
            assert!(in_executable_section(&execs, e), "entry {e:#x} outside exec");
        }
    }

    /// The `.rdata` table's third slot is relocated too, but points at the table
    /// itself — a data pointer, not a code pointer.
    #[test]
    fn relocated_data_pointer_is_not_a_function() {
        let bytes = fixture();
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture PE");
        assert!(!collect_entries(&file, bytes.as_slice()).contains(&0x140002000));
    }

    /// Data merged into the executable section, past everything the exception
    /// table vouches for, is not seeded however the section is flagged — the
    /// `jormungandr.exe` shape, where 666 relocated pointers name a UTF-16
    /// locale table living in `.text`.
    #[test]
    fn unvouched_executable_address_is_not_a_function() {
        let bytes = fixture();
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture PE");
        let entries = collect_entries(&file, bytes.as_slice());
        assert!(
            !entries.contains(&PAYLOAD),
            "unvouched {PAYLOAD:#x} seeded from {entries:#x?}"
        );
    }

    /// Without `[begin, end)` bounds there is no way to tell a handler pointer
    /// from a jump-table label, so the oracle abstains rather than guess: zeroing
    /// the exception directory takes the address-taken function back out.
    #[test]
    fn no_exception_directory_abstains() {
        let mut bytes = fixture();
        // IMAGE_DIRECTORY_ENTRY_EXCEPTION is directory 3; the array starts at
        // opt-header + 112 for PE32+, and the opt header follows the 24-byte
        // NT header at e_lfanew.
        let lfanew = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
        let dir3 = lfanew + 24 + 112 + 3 * 8;
        bytes[dir3..dir3 + 8].fill(0);
        let file = object::File::parse(bytes.as_slice()).expect("parse patched PE");
        let entries = collect_entries(&file, bytes.as_slice());
        assert!(
            !entries.contains(&HANDLER),
            "oracle fired without unwind bounds: {entries:#x?}"
        );
    }
}
