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
fn a_stem_drops_only_the_last_extension() {
    let images: Vec<CacheImage> = ["/usr/lib/libobjc.A.dylib", "/usr/lib/libz.1.dylib", "/usr/lib/libz.1.2.dylib"]
        .into_iter()
        .map(|p| CacheImage { address: 0, path: p.into() })
        .collect();
    assert_eq!(select_image(&images, Some("libobjc.A")), Ok(0));
    assert_eq!(select_image(&images, Some("libz.1")), Ok(1));
    assert!(select_image(&images, Some("libobjc")).unwrap_err().contains("close: /usr/lib/libobjc.A.dylib"));
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

/// A copy of the fixture cache in a fresh directory, its main file patched.
fn patched_fixture(tag: &str, patch: impl FnOnce(&mut Vec<u8>)) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("kuna-dyld-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for suffix in ["", ".01", ".symbols"] {
        let name = format!("dyld_shared_cache_arm64e{suffix}");
        std::fs::copy(fixture().with_file_name(&name), dir.join(&name)).unwrap();
    }
    let main = dir.join("dyld_shared_cache_arm64e");
    let mut bytes = std::fs::read(&main).unwrap();
    patch(&mut bytes);
    std::fs::write(&main, bytes).unwrap();
    main
}

fn put(bytes: &mut [u8], at: usize, value: &[u8]) {
    bytes[at..at + value.len()].copy_from_slice(value);
}

/// The fixture's first image header, at file offset 0x1000; its sixth and last
/// load command is an 80-byte `LC_DYSYMTAB` at 0x11e8.
const IMAGE0: usize = 0x1000;
const IMAGE0_DYSYMTAB: usize = 0x11e8;

#[test]
fn a_truncated_note_is_read_as_no_note() {
    let mut seg = 0x19u32.to_le_bytes().to_vec();
    seg.extend(72u32.to_le_bytes());
    seg.resize(72, 0);
    let mut note = 0x31u32.to_le_bytes().to_vec();
    note.extend(40u32.to_le_bytes());
    note.extend(xrefs::NOTE_OWNER);
    let cmds = [seg, note].concat();
    let mut image = vec![0u8; 0x1000];
    for (i, v) in [0xfeed_facfu32, 0x0100_000c, 0, 2, 2, cmds.len() as u32].into_iter().enumerate() {
        put(&mut image, i * 4, &v.to_le_bytes());
    }
    put(&mut image, 32, &cmds);
    assert!(read_note(&image).is_empty(), "the note's offset and size lie past sizeofcmds");
    put(&mut image, 20, &(cmds.len() as u32 + 16).to_le_bytes());
    put(&mut image, 32 + cmds.len(), &u64::MAX.to_le_bytes());
    put(&mut image, 40 + cmds.len(), &u64::MAX.to_le_bytes());
    assert!(read_note(&image).is_empty(), "the note's blob lies past the file");
    put(&mut image, 20, &u32::MAX.to_le_bytes());
    assert!(read_note(&image).is_empty(), "sizeofcmds runs past the file");
}

#[test]
fn a_read_past_the_end_of_a_cache_file_is_refused_before_allocating() {
    let main = patched_fixture("bigslide", |b| put(b, 0x260 + 56 + 32, &(1u64 << 50).to_le_bytes()));
    let cache = DyldCache::open(&main).unwrap();
    let err = cache.extract(0).unwrap_err();
    assert!(err.contains("past the end"), "{err}");
    std::fs::remove_dir_all(main.parent().unwrap()).unwrap();
}

#[test]
fn a_sub_cache_suffix_cannot_name_another_directory() {
    let main = patched_fixture("traverse", |b| put(b, 0x320 + 24, b"/../../etc/hostname\0"));
    let err = DyldCache::open(&main).err().unwrap();
    assert!(err.contains("not a file-name suffix"), "{err}");
    std::fs::remove_dir_all(main.parent().unwrap()).unwrap();
}

#[test]
fn a_load_command_cut_by_sizeofcmds_is_skipped() {
    let main = patched_fixture("cutcmd", |b| put(b, IMAGE0 + 20, &(536u32 - 40).to_le_bytes()));
    let cache = DyldCache::open(&main).unwrap();
    let bytes = cache.extract(0).unwrap();
    assert!(object::File::parse(bytes.as_slice()).is_ok());
    std::fs::remove_dir_all(main.parent().unwrap()).unwrap();
}

#[test]
fn a_local_symbol_range_that_wraps_is_not_replaced() {
    let main = patched_fixture("wraplocals", |b| {
        put(b, IMAGE0_DYSYMTAB + 8, &u32::MAX.to_le_bytes());
        put(b, IMAGE0_DYSYMTAB + 12, &2u32.to_le_bytes());
    });
    let cache = DyldCache::open(&main).unwrap();
    assert!(cache.extract(0).is_ok());
    std::fs::remove_dir_all(main.parent().unwrap()).unwrap();
}

#[test]
fn image_entries_aliasing_one_header_share_its_layout() {
    let main = patched_fixture("alias", |b| put(b, 0x360 + 32, &0x1_8000_1000u64.to_le_bytes()));
    let cache = DyldCache::open(&main).unwrap();
    let (a, b) = (cache.image_layout(0).unwrap(), cache.image_layout(1).unwrap());
    assert!(std::rc::Rc::ptr_eq(&a, &b));
    assert_eq!(cache.layout_bytes.get(), 536, "the shared header's commands are counted once");
    std::fs::remove_dir_all(main.parent().unwrap()).unwrap();
}

#[test]
fn a_string_pool_without_a_terminator_is_scanned_once() {
    let size = 8192 * NAME_TAIL;
    let reads = std::cell::Cell::new(0u64);
    let read = |off: u64, len: usize| -> Result<Vec<u8>, String> {
        reads.set(reads.get() + len as u64);
        assert!(off + len as u64 <= size);
        Ok(vec![b'a'; len])
    };
    let start = std::time::Instant::now();
    let pool = StringPool::read_at(&read, size, [1u64].into_iter(), &|_| Ok(())).unwrap();
    let elapsed = start.elapsed();
    assert_eq!(pool.get_at(1).len() as u64, size - 1);
    assert_eq!(reads.get(), size - 1);
    assert!(elapsed < std::time::Duration::from_secs(5), "a 32 MiB pool took {elapsed:?}");
}

/// One 72-byte segment command claiming a million sections holds none: the
/// section headers are read only as far as its own `cmdsize` goes.
#[test]
fn sections_stop_at_the_segment_command() {
    let mut seg = 0x19u32.to_le_bytes().to_vec();
    seg.extend(72u32.to_le_bytes());
    seg.resize(72, 0);
    put(&mut seg, 64, &1_000_000u32.to_le_bytes());
    let mut cmds = seg;
    cmds.resize(72 + 80 * 1000, 0xAA);
    let layout = extract::tests_parse_layout(0x1000, vec![0; 32], cmds);
    assert_eq!(layout.segments.len(), 1);
    assert!(layout.segments[0].sections.is_empty());
}

/// Image entries that all name one long path copy it at most once per byte of
/// the file, however many entries there are.
#[test]
fn image_paths_copy_at_most_the_file() {
    let n = 1000u32;
    let header_len = 0x200usize;
    let pool_at = header_len + 32 * n as usize;
    let mut bytes = vec![0u8; pool_at + 1 << 16];
    bytes[..16].copy_from_slice(b"dyld_v1  arm64e\0");
    put(&mut bytes, HDR_IMAGES_OFFSET, &(header_len as u32).to_le_bytes());
    put(&mut bytes, HDR_IMAGES_OFFSET + 4, &n.to_le_bytes());
    for i in 0..n as usize {
        put(&mut bytes, header_len + i * 32, &(0x1000u64 + i as u64).to_le_bytes());
        put(&mut bytes, header_len + i * 32 + 24, &((pool_at + 1) as u32).to_le_bytes());
    }
    bytes[pool_at + 1..pool_at + (1 << 16) - 1].fill(b'a');
    let dir = std::env::temp_dir().join(format!("kuna-dyld-paths-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("cache");
    std::fs::write(&path, &bytes).unwrap();
    let file = CacheFile { file: File::open(&path).unwrap(), name: "cache".into(), len: bytes.len() as u64 };
    let header = Header(bytes[..header_len].to_vec());
    let images = DyldCache::read_images(&file, &header).unwrap();
    let total: usize = images.iter().map(|i| i.path.len()).sum();
    assert_eq!(images.len(), 1000);
    assert!(total <= bytes.len(), "{total} bytes of paths copied from a {}-byte file", bytes.len());
    std::fs::remove_dir_all(dir).unwrap();
}

/// A note whose every entry names one long string copies it at most once per
/// byte of the name pool.
#[test]
fn a_note_copies_at_most_its_name_pool() {
    let n = 1000usize;
    let mut blob = (n as u32).to_le_bytes().to_vec();
    for _ in 0..n {
        blob.extend(0u64.to_le_bytes());
        blob.extend(0u32.to_le_bytes());
        blob.extend(0u32.to_le_bytes());
    }
    blob.extend(std::iter::repeat_n(b'a', 1 << 16));
    blob.push(0);
    let names_len = blob.len() - (4 + n * 16);
    let xrefs = xrefs::decode(&blob);
    let copied: usize = xrefs.iter().map(|x| x.name.len()).sum();
    assert!(copied <= names_len, "{copied} name bytes from a {names_len}-byte pool");
    assert_eq!(xrefs.len(), 1, "the second entry would pass the allowance");
}

/// Once the retained-byte allowance is spent, slide info and symbol tables are
/// refused before their blobs are read or copied.
#[test]
fn a_spent_retained_allowance_stops_slide_and_symbol_reads() {
    let cache = DyldCache::open(&fixture()).unwrap();
    assert!(cache.extract(0).is_ok());
    let cache = DyldCache::open(&fixture()).unwrap();
    cache.retained.set(0);
    let err = cache.extract(0).unwrap_err();
    assert!(err.contains("exceed the cache's size"), "{err}");
    let pool = StringPool::read_at(&|_, len| Ok(vec![b'a'; len]), 8192, [1u64].into_iter(), &|_| Err("spent".into()));
    assert_eq!(pool.err().as_deref(), Some("spent"));
}
