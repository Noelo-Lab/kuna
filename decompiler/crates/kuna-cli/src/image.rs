//! Shared object-file view, including Mach-O slice selection and TE/MZ refusal.

use kuna_analysis::loader::macho_fat::SlicePref;

/// Read the loader's selected image slice for an object-file consumer.
/// TE inputs are loadable but cannot be represented by `object::File`.
pub(crate) fn image_bytes(binary: &str, pref: SlicePref) -> Result<Vec<u8>, String> {
    let bytes = kuna_analysis::loader::elf_shdr::read_image_sliced(binary, pref)
        .map_err(|e| format!("{binary}: {e}"))?;
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
