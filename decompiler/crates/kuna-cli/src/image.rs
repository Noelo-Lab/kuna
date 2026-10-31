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
