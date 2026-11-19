//! (kuna `objectview`) An `object::File` view of an image the loader maps
//! without one: a UEFI TE image or a headerless `--raw-image`.
//!
//! Every analyzer pass, and every CLI consumer that scans sections, starts from
//! an `object::File`. A TE image is a PE whose DOS, PE and optional headers were
//! stripped down to a 40-byte header, and a raw image has no header at all, so
//! both declined the whole tier: no strings, no prologue starts, no string or
//! crypto inventory. The loader keeps mapping them its own way; this module
//! only rebuilds a header the `object` crate can read, describing exactly the
//! mapping the loader already uses.
//!
//! - **TE → PE.** The TE header replaced `StrippedSize` bytes of headers, and
//!   every section's `PointerToRawData` still counts from the start of the
//!   original file. So the view is a freshly written DOS + PE + optional header
//!   of `StrippedSize` bytes followed by the TE file from its section table on:
//!   the section table and every raw pointer stay valid unchanged. Machine,
//!   subsystem, entry, `BaseOfCode`, `ImageBase` and the two data directories a
//!   TE keeps (base relocations, debug) are copied across.
//! - **Raw → ELF.** One `PT_LOAD` and one `.text` section (`SHF_ALLOC |
//!   SHF_WRITE | SHF_EXECINSTR`, the read-write-execute block Ghidra's raw
//!   importer makes) covering the file at the caller's base, with the machine,
//!   class and byte order of the selected SLEIGH language and the first entry
//!   as `e_entry`.

const TE_HEADER: usize = 40;
const SECTION_HEADER: usize = 40;

/// The PE view of TE image `te`, or `None` when its header or section table
/// does not fit the file.
pub fn te_as_pe(te: &[u8]) -> Option<Vec<u8>> {
    if te.get(..2)? != b"VZ" {
        return None;
    }
    let u16_at = |off: usize| Some(u16::from_le_bytes(te.get(off..off + 2)?.try_into().ok()?));
    let u32_at = |off: usize| Some(u32::from_le_bytes(te.get(off..off + 4)?.try_into().ok()?));
    let machine = u16_at(2)?;
    let nsections = usize::from(*te.get(4)?);
    let subsystem = u16::from(*te.get(5)?);
    let stripped = usize::from(u16_at(6)?);
    let entry = u32_at(8)?;
    let base_of_code = u32_at(12)?;
    let image_base = u64::from_le_bytes(te.get(16..24)?.try_into().ok()?);
    let directories = [(u32_at(24)?, u32_at(28)?), (u32_at(32)?, u32_at(36)?)];
    let table_end = TE_HEADER.checked_add(nsections.checked_mul(SECTION_HEADER)?)?;
    let table = te.get(TE_HEADER..table_end)?;

    let wide = matches!(
        machine,
        object::pe::IMAGE_FILE_MACHINE_AMD64
            | object::pe::IMAGE_FILE_MACHINE_ARM64
            | object::pe::IMAGE_FILE_MACHINE_IA64
            | object::pe::IMAGE_FILE_MACHINE_RISCV64
    );
    let optional_size: usize = if wide { 240 } else { 224 };
    let header_len = stripped.max(0x40 + 24 + optional_size);
    let shift = u32::try_from(header_len - stripped).ok()?;

    let mut sections = table.to_vec();
    let mut image_end: u64 = 0;
    for record in sections.chunks_exact_mut(SECTION_HEADER) {
        let field = |r: &[u8], off: usize| u32::from_le_bytes(r[off..off + 4].try_into().unwrap());
        let virtual_size = field(record, 8);
        let virtual_address = field(record, 12);
        let raw_size = field(record, 16);
        let raw_pointer = field(record, 20);
        image_end = image_end.max(u64::from(virtual_address) + u64::from(virtual_size.max(raw_size)));
        if raw_pointer != 0 {
            record[20..24].copy_from_slice(&raw_pointer.checked_add(shift)?.to_le_bytes());
        }
    }
    let size_of_image = u32::try_from((image_end + 0xfff) & !0xfff).ok()?;
    let size_of_headers = u32::try_from(header_len + sections.len()).ok()?;

    let mut out = vec![0u8; header_len];
    out[..2].copy_from_slice(b"MZ");
    let pe = header_len - 24 - optional_size;
    out[0x3c..0x40].copy_from_slice(&(pe as u32).to_le_bytes());
    let mut w = Writer { buf: &mut out, at: pe };
    w.bytes(b"PE\0\0");
    w.u16(machine);
    w.u16(nsections as u16);
    w.u32(0);
    w.u32(0);
    w.u32(0);
    w.u16(optional_size as u16);
    w.u16(if wide { 0x0022 } else { 0x0102 });
    w.u16(if wide { 0x20b } else { 0x10b });
    w.u16(0);
    w.u32(0);
    w.u32(0);
    w.u32(0);
    w.u32(entry);
    w.u32(base_of_code);
    if wide {
        w.u64(image_base);
    } else {
        w.u32(0);
        w.u32(image_base as u32);
    }
    w.u32(0x1000);
    w.u32(0x200);
    for _ in 0..6 {
        w.u16(0);
    }
    w.u32(0);
    w.u32(size_of_image);
    w.u32(size_of_headers);
    w.u32(0);
    w.u16(subsystem);
    w.u16(0);
    for _ in 0..4 {
        if wide {
            w.u64(0);
        } else {
            w.u32(0);
        }
    }
    w.u32(0);
    w.u32(16);
    for index in 0..16 {
        let (rva, size) = match index {
            5 => directories[0],
            6 => directories[1],
            _ => (0, 0),
        };
        w.u32(rva);
        w.u32(size);
    }
    debug_assert_eq!(w.at, header_len);
    out.extend_from_slice(&sections);
    out.extend_from_slice(&te[table_end..]);
    Some(out)
}

/// The ELF view of headerless `image` mapped at `base` for SLEIGH language
/// `language` (`processor:endian:size:variant[:compiler]`), entered at `entry`.
/// `None` for a language id that does not name its endianness and size.
pub fn raw_as_elf(image: &[u8], base: u64, entry: u64, language: &str) -> Option<Vec<u8>> {
    let mut fields = language.split(':');
    let processor = fields.next()?;
    let big = match fields.next()? {
        "BE" => true,
        "LE" => false,
        _ => return None,
    };
    let bits: u32 = fields.next()?.parse().ok()?;
    let wide = bits > 32;
    let machine = elf_machine(processor, wide);

    let ehsize: usize = if wide { 64 } else { 52 };
    let phsize: usize = if wide { 56 } else { 32 };
    let shsize: usize = if wide { 64 } else { 40 };
    let data_at = (ehsize + phsize + 15) & !15;
    let names = b"\0.text\0.shstrtab\0";
    let names_at = data_at + image.len();
    let sections_at = (names_at + names.len() + 7) & !7;

    let mut out = vec![0u8; sections_at + 3 * shsize];
    out[..4].copy_from_slice(b"\x7fELF");
    out[4] = if wide { 2 } else { 1 };
    out[5] = if big { 2 } else { 1 };
    out[6] = 1;
    let mut w = Endian { buf: &mut out, at: 16, big, wide };
    w.u16(2);
    w.u16(machine);
    w.u32(1);
    w.word(entry);
    w.word(ehsize as u64);
    w.word(sections_at as u64);
    w.u32(0);
    w.u16(ehsize as u16);
    w.u16(phsize as u16);
    w.u16(1);
    w.u16(shsize as u16);
    w.u16(3);
    w.u16(2);

    let len = image.len() as u64;
    w.u32(1);
    if wide {
        w.u32(7);
        w.word(data_at as u64);
        w.word(base);
        w.word(base);
        w.word(len);
        w.word(len);
        w.word(1);
    } else {
        w.word(data_at as u64);
        w.word(base);
        w.word(base);
        w.word(len);
        w.word(len);
        w.u32(7);
        w.word(1);
    }

    out[data_at..names_at].copy_from_slice(image);
    out[names_at..names_at + names.len()].copy_from_slice(names);
    let mut w = Endian { buf: &mut out, at: sections_at + shsize, big, wide };
    section(&mut w, 1, 1, 7, base, data_at as u64, len);
    section(&mut w, 7, 3, 0, 0, names_at as u64, names.len() as u64);
    Some(out)
}

fn section(w: &mut Endian, name: u32, kind: u32, flags: u64, addr: u64, offset: u64, size: u64) {
    w.u32(name);
    w.u32(kind);
    w.word(flags);
    w.word(addr);
    w.word(offset);
    w.word(size);
    w.u32(0);
    w.u32(0);
    w.word(1);
    w.word(0);
}

/// The ELF `e_machine` of a SLEIGH processor, `EM_NONE` for one ELF has no
/// number for (the view still carries its sections; only the architecture is
/// unknown to `object`).
fn elf_machine(processor: &str, wide: bool) -> u16 {
    match processor {
        "x86" if wide => 62,
        "x86" => 3,
        "ARM" => 40,
        "AARCH64" => 183,
        "MIPS" => 8,
        "PowerPC" if wide => 21,
        "PowerPC" => 20,
        "sparc" if wide => 43,
        "sparc" => 2,
        "RISCV" => 243,
        "68000" => 4,
        "avr8" => 83,
        "avr32" => 185,
        "TI_MSP430" | "TI_MSP430X" => 105,
        "8051" => 165,
        "SuperH" | "SuperH4" => 42,
        "V850" => 87,
        "Xtensa" => 94,
        "Hexagon" => 164,
        "Loongarch" => 258,
        "eBPF" => 247,
        "z80" => 220,
        "HC-12" | "HCS-12" | "HCS12" | "HCS-12X" => 53,
        "tricore" => 44,
        "CR16C" => 177,
        "NDS32" => 167,
        "pa-risc" => 15,
        "M16C/60" | "M16C/80" => 117,
        _ => 0,
    }
}

struct Writer<'a> {
    buf: &'a mut [u8],
    at: usize,
}

impl Writer<'_> {
    fn bytes(&mut self, b: &[u8]) {
        self.buf[self.at..self.at + b.len()].copy_from_slice(b);
        self.at += b.len();
    }
    fn u16(&mut self, v: u16) {
        self.bytes(&v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.bytes(&v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.bytes(&v.to_le_bytes());
    }
}

struct Endian<'a> {
    buf: &'a mut [u8],
    at: usize,
    big: bool,
    wide: bool,
}

impl Endian<'_> {
    fn put(&mut self, le: &[u8]) {
        let n = le.len();
        let slot = &mut self.buf[self.at..self.at + n];
        slot.copy_from_slice(le);
        if self.big {
            slot.reverse();
        }
        self.at += n;
    }
    fn u16(&mut self, v: u16) {
        self.put(&v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.put(&v.to_le_bytes());
    }
    fn word(&mut self, v: u64) {
        if self.wide {
            self.put(&v.to_le_bytes());
        } else {
            self.put(&(v as u32).to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loadimage_te::synthetic::{TeImage, CODE_RVA, IMAGE_BASE};
    use object::{Object, ObjectSection};

    #[test]
    fn te_view_maps_sections_where_the_te_loader_does() {
        let code = [0x31, 0xc0, 0xc3];
        let te = TeImage::arm(&code).machine(object::pe::IMAGE_FILE_MACHINE_AMD64).build();
        let pe = te_as_pe(&te).unwrap();
        let file = object::File::parse(pe.as_slice()).unwrap();
        assert_eq!(file.architecture(), object::Architecture::X86_64);
        assert_eq!(file.entry(), IMAGE_BASE + u64::from(CODE_RVA));
        let text = file.section_by_name(".text").unwrap();
        assert_eq!(text.address(), IMAGE_BASE + u64::from(CODE_RVA));
        assert_eq!(text.data().unwrap(), code);
    }

    #[test]
    fn te_view_shifts_raw_pointers_when_the_stripped_header_is_short() {
        let code = [0x70, 0x47];
        let mut te = TeImage::thumb(&code).build();
        let at = TeImage::thumb(&code).code_file_offset();
        let stripped = 0x60u16;
        let raw_pointer = (at + usize::from(stripped) - 40) as u32;
        te[6..8].copy_from_slice(&stripped.to_le_bytes());
        te[60..64].copy_from_slice(&raw_pointer.to_le_bytes());
        let pe = te_as_pe(&te).unwrap();
        let file = object::File::parse(pe.as_slice()).unwrap();
        assert_eq!(file.section_by_name(".text").unwrap().data().unwrap(), code);
    }

    #[test]
    fn raw_view_is_one_executable_section_at_the_base() {
        let image = [0x13, 0x00, 0x00, 0x00, 0x67, 0x80, 0x00, 0x00];
        for (language, arch) in [
            ("RISCV:LE:32:default", object::Architecture::Riscv32),
            ("PowerPC:BE:32:default", object::Architecture::PowerPc),
            ("AARCH64:LE:64:v8A", object::Architecture::Aarch64),
            ("6502:LE:16:default", object::Architecture::Unknown),
        ] {
            let elf = raw_as_elf(&image, 0x8000_0000, 0x8000_0004, language).unwrap();
            let file = object::File::parse(elf.as_slice()).unwrap();
            assert_eq!(file.architecture(), arch, "{language}");
            assert_eq!(file.entry(), 0x8000_0004, "{language}");
            let text = file.section_by_name(".text").unwrap();
            assert_eq!((text.address(), text.data().unwrap()), (0x8000_0000, image.as_slice()), "{language}");
        }
        assert!(raw_as_elf(&image, 0, 0, "x86").is_none());
    }
}
