//! Whether an arm64 image's platform has the callee extend a return value
//! narrower than 32 bits to 32 bits.  Apple's arm64 ABI does, and its caller
//! reads `w0` as it is; the AAPCS64 that ELF and PE images follow leaves the
//! extension to the caller.  Any other container says nothing.
use object::Object;

pub fn callee_extends_returns(file: &object::File<'_>) -> Option<bool> {
    use object::{Architecture as A, BinaryFormat as F};
    if !matches!(file.architecture(), A::Aarch64 | A::Aarch64_Ilp32) {
        return None;
    }
    match file.format() {
        F::MachO => Some(true),
        F::Elf | F::Coff | F::Pe => Some(false),
        _ => None,
    }
}
