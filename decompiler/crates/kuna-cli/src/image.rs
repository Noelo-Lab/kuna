//! Shared object-file view, including Mach-O slice selection and TE/MZ refusal.

use kuna_analysis::loader::macho_fat::SlicePref;

/// Read the loader's selected image slice for an object-file consumer.
/// TE inputs are loadable but cannot be represented by `object::File`.
pub(crate) fn image_bytes(binary: &str, pref: SlicePref) -> Result<Vec<u8>, String> {
    let bytes = kuna_analysis::loader::elf_shdr::read_image_sliced(binary, pref)
        .map_err(|e| format!("{binary}: {e}"))?;
    object_view(binary, bytes)
}

/// `[start, end)` offsets into an image slice whose bytes are not the file's.
pub(crate) type Repairs = Vec<(u64, u64)>;

/// [`image_bytes`], and the [`Repairs`]: the header fields the loader's repairs
/// rewrote so the image parses at all (a corrupt ELF section table, DOS magic
/// or PE data-directory count). The file is read a second time only when a
/// repair happened.
pub(crate) fn image_bytes_and_repairs(binary: &str, pref: SlicePref) -> Result<(Vec<u8>, Repairs), String> {
    let (bytes, repaired) = kuna_analysis::loader::elf_shdr::read_image_sliced_repaired(binary, pref)
        .map_err(|e| format!("{binary}: {e}"))?;
    let bytes = object_view(binary, bytes)?;
    if !repaired {
        return Ok((bytes, Vec::new()));
    }
    let file = std::fs::read(binary).map_err(|e| format!("{binary}: {e}"))?;
    let file = kuna_analysis::loader::macho_fat::peel_fat_image(file, pref);
    let mut spans = Repairs::new();
    for (i, &b) in bytes.iter().enumerate() {
        if file.get(i) == Some(&b) {
            continue;
        }
        let at = i as u64; // cast: a byte index
        match spans.last_mut() {
            Some(last) if last.1 == at => last.1 = at + 1,
            _ => spans.push((at, at + 1)),
        }
    }
    Ok((bytes, spans))
}

/// `bytes`, unless the image has no object-file view (TE, MZ).
fn object_view(binary: &str, bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    if kuna_analysis::loadimage_te::is_te_image(&bytes) {
        return Err(te_object_view_error(binary));
    }
    if kuna_analysis::loadimage_mz::is_mz_dos_image(&bytes) {
        return Err(mz_object_view_error(binary));
    }
    Ok(bytes)
}

/// Where [`image_bytes`]'s slice starts in `binary`: the selected arch's offset
/// in a fat Mach-O, and `0` for every thin input. Only a file that opens with a
/// fat magic is read whole a second time.
pub(crate) fn slice_offset(binary: &str, pref: SlicePref) -> u64 {
    use std::io::Read;
    let mut head = [0u8; 16];
    let opened = std::fs::File::open(binary).and_then(|mut f| f.read_exact(&mut head));
    if opened.is_err() || !kuna_analysis::loader::macho_fat::is_fat(&head) {
        return 0;
    }
    std::fs::read(binary)
        .ok()
        .and_then(|bytes| kuna_analysis::loader::macho_fat::fat_slice_offset(&bytes, pref))
        .unwrap_or(0)
}

/// The same capability error for an MS-DOS `MZ` executable, which loads as
/// one relocated real-mode image with no sections or symbols.
fn mz_object_view_error(binary: &str) -> String {
    format!(
        "{binary}: MS-DOS MZ input has no object-file view, which this operation needs; \
         MZ support covers decompile, decompile-all, functions, disassemble/read, \
         decompile-project, and the console"
    )
}

/// The capability error every object-view consumer reports for a UEFI TE
/// input: the loader maps it, but nothing can hand out the `object::File` the
/// call graph, string inventory, and reference index are built from.
fn te_object_view_error(binary: &str) -> String {
    format!(
        "{binary}: UEFI TE input has no object-file view, which this operation needs; \
         TE support covers decompile, decompile-all, functions, disassemble/read, \
         decompile-project, and the console"
    )
}
