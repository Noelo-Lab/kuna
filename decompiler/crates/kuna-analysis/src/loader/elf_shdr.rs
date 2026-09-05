//! (kuna) ELF section-table tolerance: keep an image whose section headers are
//! unreadable, because its program headers already describe the loadable image.
//!
//! An ELF's section table is link-time metadata. Everything the decompiler needs
//! to map memory -- the entry point and the `PT_LOAD` segments -- lives in the
//! ELF header and the program headers, which is why `readelf -l` still prints a
//! full segment map for an image whose `e_shoff` is garbage. `object`, though,
//! validates the section table eagerly in `File::parse`, so a single corrupt
//! half-word rejects the whole file and every kuna surface (`functions`,
//! `decompile-all`, `strings`, `disassemble`) exits with
//! "not in recognized object file format: Invalid ELF section header
//! offset/size/alignment".
//!
//! Packers, `sstrip`, and CTF authors all produce such images deliberately. This
//! module recovers them: when the section-table fields cannot describe a table
//! inside the file, clear `e_shoff`/`e_shnum`/`e_shstrndx` in a copy of the
//! bytes -- the encoding of "this ELF has no section table", which `object`
//! accepts -- and hand that copy downstream. A file whose section table is
//! usable is returned verbatim, byte for byte, with no parse performed.

use object::read::Object;

/// Read an image file with [`tolerate_unusable_section_table`] applied, so a
/// surface that parses the bytes itself sees the same recovered view the loader
/// does instead of rejecting the file outright.
///
/// Silent by design: the surfaces that call this already report their own errors,
/// and the interactive load path (`bootstrap_from_object`) prints the diagnostic
/// once for the whole run.
pub fn read_image(path: &str) -> std::io::Result<Vec<u8>> {
    let bytes = std::fs::read(path)?;
    Ok(tolerate_unusable_section_table(bytes).0)
}

/// ELF header field offsets, by class. `object` reads these through typed
/// structs; the repair needs them as raw offsets because the file it is looking
/// at is by definition one `object` refuses to parse.
struct HeaderLayout {
    /// `e_shoff` offset and width (4 for ELF32, 8 for ELF64).
    shoff: usize,
    shoff_width: usize,
    /// `e_shentsize`, `e_shnum`, `e_shstrndx` -- `u16` each.
    shentsize: usize,
    shnum: usize,
    shstrndx: usize,
    /// The size one section header must have for `object` to accept the table.
    entsize: u64,
    /// Offset of `sh_size` inside a section header (the extended `e_shnum == 0`
    /// encoding stores the real count there, in section 0) and of `sh_link` (the
    /// extended `e_shstrndx == SHN_XINDEX` encoding stores the real index there).
    sh_size: usize,
    sh_size_width: usize,
    sh_link: usize,
}

const ELF32: HeaderLayout = HeaderLayout {
    shoff: 32,
    shoff_width: 4,
    shentsize: 46,
    shnum: 48,
    shstrndx: 50,
    entsize: 40,
    sh_size: 20,
    sh_size_width: 4,
    sh_link: 24,
};

const ELF64: HeaderLayout = HeaderLayout {
    shoff: 40,
    shoff_width: 8,
    shentsize: 58,
    shnum: 60,
    shstrndx: 62,
    entsize: 64,
    sh_size: 32,
    sh_size_width: 8,
    sh_link: 40,
};

/// `SHN_XINDEX`: `e_shstrndx` too large for a `u16`, real index in `sh_link`.
const SHN_XINDEX: u64 = 0xffff;

/// Return `bytes` with an unusable ELF section table cleared, plus the one-line
/// diagnostic describing what was dropped (`None` when nothing was).
///
/// The check is pure header arithmetic and runs before any parse, so a
/// well-formed image costs one bounds test and is returned untouched. The repair
/// itself is only kept if the rewritten copy actually parses -- otherwise the
/// original bytes are returned so the caller reports `object`'s own error rather
/// than a misleading one about a table that was not the problem.
pub fn tolerate_unusable_section_table(bytes: Vec<u8>) -> (Vec<u8>, Option<String>) {
    let Some((layout, le)) = elf_layout(&bytes) else {
        return (bytes, None);
    };
    let Some(reason) = unusable_reason(&bytes, layout, le) else {
        return (bytes, None);
    };

    let mut repaired = bytes.clone();
    write_uint(&mut repaired, layout.shoff, layout.shoff_width, le, 0);
    write_uint(&mut repaired, layout.shnum, 2, le, 0);
    write_uint(&mut repaired, layout.shstrndx, 2, le, 0);
    let Ok(file) = object::File::parse(repaired.as_slice()) else {
        return (bytes, None);
    };
    let note = format!(
        "ELF section table unusable ({reason}); continuing from the program headers \
         (entry {:#x}, {} load segment(s))",
        file.entry(),
        file.segments().count()
    );
    drop(file);
    (repaired, Some(note))
}

/// `(layout, little_endian)` for a file whose `e_ident` says it is an ELF whose
/// header is fully present. `None` for anything else (PE, Mach-O, a truncated
/// stub), which is returned untouched.
fn elf_layout(bytes: &[u8]) -> Option<(&'static HeaderLayout, bool)> {
    if bytes.len() < 64 || &bytes[..4] != b"\x7fELF" {
        return None;
    }
    let layout = match bytes[4] {
        1 => &ELF32,
        2 => &ELF64,
        _ => return None,
    };
    let le = match bytes[5] {
        1 => true,
        2 => false,
        _ => return None,
    };
    Some((layout, le))
}

/// Why `object`'s `section_headers`/`section_strings` will reject this table, or
/// `None` if they will accept it. Mirrors `object::read::elf::FileHeader`: a zero
/// `e_shoff` or a zero section count is "no sections", which is fine; a wrong
/// entry size, a table running past EOF, or an out-of-range `e_shstrndx` is not.
fn unusable_reason(bytes: &[u8], layout: &HeaderLayout, le: bool) -> Option<String> {
    let shoff = read_uint(bytes, layout.shoff, layout.shoff_width, le);
    if shoff == 0 {
        return None;
    }
    let len = bytes.len() as u64;
    let shentsize = read_uint(bytes, layout.shentsize, 2, le);
    if shentsize != layout.entsize {
        return Some(format!(
            "e_shentsize is {shentsize}, not {}",
            layout.entsize
        ));
    }
    // The extended count: `e_shnum == 0` with a nonzero `e_shoff` means the real
    // count is section 0's `sh_size`, which itself must be inside the file.
    let mut shnum = read_uint(bytes, layout.shnum, 2, le);
    if shnum == 0 {
        if shoff.saturating_add(layout.entsize) > len {
            return Some(format!(
                "e_shoff {shoff:#x} leaves no room for section 0 in a {len}-byte file"
            ));
        }
        shnum = read_uint(
            bytes,
            (shoff as usize) + layout.sh_size,
            layout.sh_size_width,
            le,
        );
        if shnum == 0 {
            return None;
        }
    }
    let span = shnum.saturating_mul(layout.entsize);
    if shoff.saturating_add(span) > len {
        return Some(format!(
            "{shnum} section headers at e_shoff {shoff:#x} run {} bytes past the end of a \
             {len}-byte file",
            shoff.saturating_add(span) - len
        ));
    }
    let mut shstrndx = read_uint(bytes, layout.shstrndx, 2, le);
    if shstrndx == SHN_XINDEX {
        // The real string-table index lives in section 0's `sh_link`.
        shstrndx = read_uint(bytes, (shoff as usize) + layout.sh_link, 4, le);
    }
    // `object` rejects a zero `e_shstrndx` outright once a section table exists
    // ("Missing ELF e_shstrndx"), which is what a section-name table stripped in
    // place looks like -- the whole file is unloadable over a name table nothing
    // in the decompiler needs.
    if shstrndx == 0 {
        return Some("e_shstrndx is 0, so the section names cannot be read".to_string());
    }
    if shstrndx >= shnum {
        return Some(format!(
            "e_shstrndx is {shstrndx} but there are only {shnum} section headers"
        ));
    }
    None
}

fn read_uint(bytes: &[u8], off: usize, width: usize, le: bool) -> u64 {
    let Some(slice) = bytes.get(off..off + width) else {
        return 0;
    };
    let mut v: u64 = 0;
    if le {
        for (i, b) in slice.iter().enumerate() {
            v |= (*b as u64) << (8 * i);
        }
    } else {
        for b in slice {
            v = (v << 8) | *b as u64;
        }
    }
    v
}

fn write_uint(bytes: &mut [u8], off: usize, width: usize, le: bool, value: u64) {
    let Some(slice) = bytes.get_mut(off..off + width) else {
        return;
    };
    for i in 0..width {
        let shift = if le { 8 * i } else { 8 * (width - 1 - i) };
        slice[i] = ((value >> shift) & 0xff) as u8;
    }
}

#[cfg(test)]
mod tests;
