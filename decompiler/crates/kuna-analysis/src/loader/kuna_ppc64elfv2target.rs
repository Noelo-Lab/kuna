//! A big-endian PowerPC64 ELF whose `e_flags` declare ABI version 2 (ELFv2,
//! e.g. musl or `-mabi=elfv2`) takes the `elfv2` compiler spec: its frame
//! header is 32 bytes, not ELFv1's 48, so stack parameters start at SP+96.
//! Little-endian PowerPC64 is ELFv2 throughout and its default spec says so.
use object::{Architecture, FileFlags, Object};

const ID: &str = "PowerPC:BE:64:default:elfv2";

pub(crate) fn language_ids() -> impl Iterator<Item = String> {
    std::iter::once(ID.to_string())
}

pub(crate) fn language_id(file: &object::File<'_>) -> Option<String> {
    (file.format() == object::BinaryFormat::Elf
        && file.architecture() == Architecture::PowerPc64
        && !file.is_little_endian()
        && matches!(file.flags(), FileFlags::Elf { e_flags, .. } if e_flags & 3 == 2))
        .then(|| ID.to_string())
}

#[cfg(test)]
mod tests {
    use object::write::Object;
    use object::{Architecture, BinaryFormat, Endianness, FileFlags};

    fn selected(endian: Endianness, e_flags: u32) -> Option<String> {
        let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, endian);
        object.flags = FileFlags::Elf { os_abi: 0, abi_version: 0, e_flags };
        let bytes = object.write().unwrap();
        super::language_id(&object::File::parse(&*bytes).unwrap())
    }

    #[test]
    fn only_a_big_endian_abi_version_2_image_takes_the_elfv2_spec() {
        assert_eq!(selected(Endianness::Big, 2).as_deref(), Some(super::ID));
        assert_eq!(selected(Endianness::Big, 1), None, "ELFv1");
        assert_eq!(selected(Endianness::Big, 0), None, "unspecified stays ELFv1");
        assert_eq!(selected(Endianness::Little, 2), None, "LE's default spec is ELFv2");
    }
}
