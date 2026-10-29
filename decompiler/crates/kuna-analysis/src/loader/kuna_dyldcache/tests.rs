use super::xrefs::{read_note, Xref, XrefKind};
use super::*;
use object::{Object, ObjectSection, ObjectSymbol};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dyld_cache_arm64e/dyld_shared_cache_arm64e")
}

#[test]
fn opens_a_split_cache_with_its_sub_cache_and_symbols_file() {
    assert!(is_dyld_cache_file(&fixture()));
    let cache = DyldCache::open(&fixture()).unwrap();
    assert_eq!(cache.arch(), "arm64e");
    assert_eq!(cache.base(), 0x1_8000_0000);
    let paths: Vec<&str> = cache.images().iter().map(|i| i.path.as_str()).collect();
    assert_eq!(paths, ["/usr/lib/libcaller.dylib", "/usr/lib/libcallee.dylib"]);
    let stubs = cache.mappings().iter().find(|m| m.flags & MAPPING_TEXT_STUBS != 0).unwrap();
    assert_eq!((stubs.address, stubs.file), (0x1_8000_8000, 1));
}

#[test]
fn image_names_match_by_install_name_basename_then_stem() {
    let cache = DyldCache::open(&fixture()).unwrap();
    assert_eq!(cache.select(Some("/usr/lib/libcallee.dylib")), Ok(1));
    assert_eq!(cache.select(Some("libcaller.dylib")), Ok(0));
    assert_eq!(cache.select(Some("libcallee")), Ok(1));
    assert!(cache.select(Some("libcall")).unwrap_err().contains("close: /usr/lib/libcaller.dylib"));
    assert!(cache.select(None).unwrap_err().contains("holds 2 images"));
}

#[test]
fn rebuilt_image_is_a_mach_o_at_its_cache_addresses_with_slid_pointers() {
    let cache = DyldCache::open(&fixture()).unwrap();
    let bytes = cache.extract(0).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let text = file.section_by_name("__text").unwrap();
    assert_eq!(text.address(), 0x1_8000_1800);
    assert_eq!(&text.data().unwrap()[..4], &0xA9BF_7BFDu32.to_le_bytes());
    let got = file.section_by_name("__got").unwrap().data().unwrap();
    let slot = |i: usize| u64::from_le_bytes(got[i * 8..i * 8 + 8].try_into().unwrap());
    assert_eq!(slot(0), 0x1_8001_0100, "plain v3 pointer, chain link cleared");
    assert_eq!(slot(1), 0x1_8000_2800, "auth v3 pointer, offset from the cache base");
    let names: Vec<(String, u64)> =
        file.symbols().map(|s| (s.name().unwrap().to_string(), s.address())).collect();
    assert!(names.contains(&("_helper_local".into(), 0x1_8000_1828)), "{names:?}");
    assert!(!names.iter().any(|(n, _)| n == "<redacted>"));
    let exports: Vec<String> =
        file.exports().unwrap().iter().map(|e| String::from_utf8_lossy(e.name()).into_owned()).collect();
    assert_eq!(exports, ["_caller_entry", "_caller_indirect"]);
}

#[test]
fn cross_image_references_ride_in_the_note() {
    let cache = DyldCache::open(&fixture()).unwrap();
    let xrefs = read_note(&cache.extract(0).unwrap());
    let call = |addr, name: &[u8]| Xref { addr, kind: XrefKind::Call, name: name.to_vec() };
    let slot = |addr, name: &[u8]| Xref { addr, kind: XrefKind::Slot, name: name.to_vec() };
    assert_eq!(
        xrefs,
        [
            call(0x1_8000_8800, b"_callee_add"),
            slot(0x1_8001_0000, b"_callee_counter"),
            slot(0x1_8001_0008, b"_callee_add"),
        ]
    );
    assert!(read_note(&cache.extract(1).unwrap()).is_empty(), "the callee reaches no other image");
}

#[test]
fn a_file_that_is_not_a_cache_is_left_to_the_other_loaders() {
    let elf = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/armdiscseed_le32");
    assert!(read_selected_image(&elf).is_none());
    assert!(!is_dyld_cache(b"\x7fELF"));
}

#[test]
fn without_its_symbols_file_a_redacted_local_stays_nameless() {
    let dir = std::env::temp_dir().join(format!("kuna-dyld-nosymbols-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for suffix in ["", ".01"] {
        let name = format!("dyld_shared_cache_arm64e{suffix}");
        std::fs::copy(fixture().with_file_name(&name), dir.join(&name)).unwrap();
    }
    let cache = DyldCache::open(&dir.join("dyld_shared_cache_arm64e")).unwrap();
    let bytes = cache.extract(0).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let at_helper: Vec<String> = file
        .symbols()
        .filter(|s| s.address() == 0x1_8000_1828)
        .map(|s| s.name().unwrap().to_string())
        .collect();
    assert_eq!(at_helper, [""]);
    std::fs::remove_dir_all(dir).unwrap();
}
