use super::*;

fn hello() -> DexModel {
    let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/stages/fixtures/dex/hello.dex"));
    DexModel::parse(bytes.to_vec(), 0).unwrap()
}

fn function<'a>(functions: &'a [DexFunction], name: &str) -> &'a DexFunction {
    functions.iter().find(|f| f.name == name).unwrap()
}

#[test]
fn parses_the_class_and_names_every_method_with_code() {
    let model = hello();
    assert_eq!(model.file.version, 35);
    assert_eq!(model.file.classes.len(), 1);
    let functions = model.functions();
    assert_eq!(functions.len(), 16);
    let add = function(&functions, "demo::Hello::add");
    assert!(add.is_static);
    assert_eq!(model.file.bytes()[add.addr as usize..add.addr as usize + 2], [0xb0, 0x10]);
    assert!(!function(&functions, "demo::Hello::getValue").is_static);
}

#[test]
fn entry_copies_follow_the_frame_layout() {
    let model = hello();
    let functions = model.functions();
    // static long mulWide(long, int): five registers, three of them inputs.
    let wide = function(&functions, "demo::Hello::mulWide");
    assert_eq!(model.entry_copies(wide.addr).unwrap(), vec![(0x100, 0x1008, 8), (0x108, 0x1010, 4)]);
    // int getValue(): `this` alone, in the top register of three.
    let get = function(&functions, "demo::Hello::getValue");
    let code = model.entries[&get.addr].code.unwrap();
    let top = REGISTER_START + 4 * u64::from(code.registers_size - 1);
    assert_eq!(model.entry_copies(get.addr).unwrap(), vec![(0x100, top, 4)]);
    assert!(model.entry_copies(get.addr + 2).is_none());
}

#[test]
fn descriptors_spell_as_java() {
    assert_eq!(java_type_name("[[I"), "int[][]");
    assert_eq!(java_type_name("Ljava/lang/String;"), "java.lang.String");
    assert_eq!(simple_class_name("Ljava/lang/String;"), "String");
    assert_eq!(java_type_name("Z"), "boolean");
}

#[test]
fn rejects_truncated_files() {
    let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/stages/fixtures/dex/hello.dex"));
    assert!(DexModel::parse(bytes[..0x80].to_vec(), 0).is_err());
    assert!(DexModel::parse(b"dex\n035\0".to_vec(), 0).is_err());
}

#[test]
fn a_compact_dex_file_reads_like_its_dex_file() {
    let fixture = |name: &str| {
        std::fs::read(format!("{}/../../../tests/stages/fixtures/dex/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    };
    let dex = DexModel::parse(fixture("tries.dex"), 0).unwrap();
    let cdex = DexModel::parse(fixture("tries.cdex"), 0).unwrap();
    let names = |m: &DexModel| m.functions().into_iter().map(|f| f.method_idx).collect::<Vec<_>>();
    assert_eq!(names(&dex), names(&cdex));
    for (a, b) in dex.functions().iter().zip(cdex.functions().iter()) {
        let (x, y) = (dex.entries[&a.addr].code.unwrap(), cdex.entries[&b.addr].code.unwrap());
        assert_eq!((x.registers_size, x.ins_size, x.outs_size, x.tries_size, x.insns_units),
                   (y.registers_size, y.ins_size, y.outs_size, y.tries_size, y.insns_units), "{}", a.name);
        let shift = |blocks: Vec<TryBlock>, base: u64| -> Vec<(u64, u64)> {
            blocks.iter().map(|t| (t.start - base, t.end - base)).collect()
        };
        assert_eq!(shift(dex.try_blocks(a.addr), a.addr), shift(cdex.try_blocks(b.addr), b.addr));
    }
}
