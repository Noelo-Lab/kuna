//! Whether the image was built by clang, which at `-O0` keeps a value that
//! crosses a branch in a stack slot rather than a register.  An ELF image
//! names its compilers in `.comment` (the C runtime's objects add a GCC line
//! to every one, so only a clang line decides); a Mach-O image comes from
//! Apple's clang.
use object::{Object, ObjectSection};

pub fn built_by_clang(file: &object::File<'_>) -> bool {
    match file.format() {
        object::BinaryFormat::MachO => true,
        object::BinaryFormat::Elf => file
            .section_by_name(".comment")
            .and_then(|s| s.data().ok())
            .is_some_and(|d| d.windows(13).any(|w| w == b"clang version")),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_comment_section_names_clang() {
        let gcc = b"GCC: (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0\0";
        let clang = b"GCC: (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0\0Ubuntu clang version 14.0.0-1ubuntu1.1\0";
        assert!(!gcc.windows(13).any(|w| w == b"clang version"));
        assert!(clang.windows(13).any(|w| w == b"clang version"));
    }
}
