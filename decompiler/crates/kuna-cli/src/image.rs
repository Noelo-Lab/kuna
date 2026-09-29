//! Shared object-file view, including Mach-O slice selection and TE refusal.

use kuna_analysis::loader::macho_fat::SlicePref;

/// Read the loader's selected image slice for an object-file consumer.
/// TE inputs are loadable but cannot be represented by `object::File`.
pub(crate) fn image_bytes(binary: &str, pref: SlicePref) -> Result<Vec<u8>, String> {
    let bytes = kuna_analysis::loader::elf_shdr::read_image_sliced(binary, pref)
        .map_err(|e| format!("{binary}: {e}"))?;
    if kuna_analysis::loadimage_te::is_te_image(&bytes) {
        return Err(te_object_view_error(binary));
    }
    Ok(bytes)
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
