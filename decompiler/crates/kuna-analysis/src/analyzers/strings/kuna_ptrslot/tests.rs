use super::PointerSlots;
use crate::pass::StringFact;

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

/// `ptrslot_gcc_O1_x86_64`'s `tbl` at 0x405020 holds 0x404226, 0x404235,
/// 0x404244 and NULL; the first entry's bytes read as `"&B@"`.
#[test]
fn a_function_pointer_entry_holds_an_address() {
    let bytes = fixture("ptrslot_gcc_O1_x86_64");
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let slots = PointerSlots::new(&file);
    assert!(slots.holds_address(0x405020));
    assert!(slots.holds_address(0x405028));
    assert!(!slots.holds_address(0x405021), "an unaligned run start is not a slot");
    assert!(!slots.holds_address(0x405038), "a NULL entry is no address");
    assert!(!slots.holds_address(0x405000), "_IO_stdin_used holds 0x20001");
    assert!(!slots.holds_address(0x408000), "past every section");
}

/// The i386 build is linked at 0x21414140, so every byte of `tbl`'s entries is
/// printable and the string scan itself reads the table as `"@AA!KAA!VAA!"`.
#[test]
fn a_table_the_string_scan_finds_is_a_slot() {
    let bytes = fixture("ptrslot_gcc_O1_i386");
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let run = StringFact { addr: 0x21414300, len: 13 };
    assert!(crate::strings::scan_strings(&file, 4).contains(&run));
    assert!(PointerSlots::new(&file).holds_address(run.addr));
}

/// Big-endian slots: `plt_mips32`'s `.got` entry at 0x411028 is 0x00400700, an
/// address in `.text` only when read big-endian, and `plt_sparc64`'s at 0x202000
/// is `.dynamic`'s address.
#[test]
fn a_big_endian_slot_is_read_in_its_own_byte_order() {
    let bytes = fixture("plt_mips32");
    let file = object::File::parse(bytes.as_slice()).unwrap();
    assert!(PointerSlots::new(&file).holds_address(0x411028));
    let bytes = fixture("plt_sparc64");
    let file = object::File::parse(bytes.as_slice()).unwrap();
    assert!(PointerSlots::new(&file).holds_address(0x202000));
}

/// A relocatable object's sections all start at 0, so a small value is not an
/// address of anything yet.
#[test]
fn a_relocatable_object_has_no_slots() {
    use object::write::Object;
    use object::{Architecture, BinaryFormat, Endianness, SectionKind};
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let rodata = obj.add_section(Vec::new(), b".rodata".to_vec(), SectionKind::ReadOnlyData);
    obj.append_section_data(rodata, &[0x26, 0x42, 0, 0, 0, 0, 0, 0], 8);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &[0xc3; 0x5000], 16);
    let bytes = obj.write().unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    assert!(!PointerSlots::new(&file).holds_address(0));
}
