//! Check the complete NDK interface tables in the type factory used by each Android ABI.
use kuna_console::engine::bootstrap_from_object;
use kuna_console::grammar::{parse_protopieces, DataOrg};
use kuna_decomp::architecture::Architecture;
use std::path::Path;
use std::rc::Rc;

const HEADER: &str = include_str!("../../kuna-analysis/data/android-jni/jni-types.h");

fn verify_table(arch: &Architecture, alias: &str, tag: &str, slots: i32, reserved: i32) {
    let factory = arch.types();
    let table = factory
        .find_by_name(alias)
        .unwrap()
        .unwrap()
        .get_ptr_to()
        .unwrap();
    let (addr_size, word_size) = arch.data_org();
    let org = DataOrg {
        addr_size,
        word_size,
    };
    assert_eq!(table.get_size(), slots * addr_size, "{tag}");
    assert!(table.get_field(slots).is_none(), "{tag}");
    let declaration = HEADER.split_once(&format!("struct {tag} {{")).unwrap().1;
    let fields = declaration.split_once("};").unwrap().0.trim();
    assert_eq!(fields.lines().count(), slots as usize, "{tag}");

    for (index, line) in fields.lines().enumerate() {
        let index = index as i32;
        let field = table.get_field(index).unwrap();
        assert_eq!(field.offset, index * addr_size, "{tag}[{index}]");
        let pointee = field.field_type.get_ptr_to().unwrap();
        if index < reserved {
            assert_eq!(field.name, format!("reserved{index}"));
            assert_eq!(pointee.get_name(), "void");
            continue;
        }
        let (ret, rest) = line.trim().split_once("(*").unwrap();
        let (name, args) = rest.split_once(")(").unwrap();
        assert_eq!(field.name, name, "{tag}[{index}]");
        let expected =
            parse_protopieces(&format!("extern {ret} oracle({args}"), factory, org).unwrap();
        let proto = pointee.get_code_prototype().unwrap();
        assert_eq!(
            proto.num_params() as usize,
            expected.intypes.len(),
            "{name}"
        );
        assert_eq!(
            proto.is_dotdotdot(),
            expected.first_var_arg_slot >= 0,
            "{name}"
        );
        assert!(
            Rc::ptr_eq(
                proto.get_output_type().unwrap(),
                expected.outtype.as_ref().unwrap()
            ),
            "{name} return"
        );
        for (i, declared) in expected.intypes.iter().enumerate() {
            let parameter = proto.get_param(i as i32).unwrap();
            let stored = parameter.get_type().unwrap();
            let actual = if parameter.is_indirect_storage() {
                stored.get_ptr_to().unwrap()
            } else {
                stored.clone()
            };
            assert!(Rc::ptr_eq(&actual, declared), "{name} parameter {i}");
        }
    }
}

#[test]
fn every_ndk_slot_preserves_its_offset_and_function_prototype() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = [root.join("specs").display().to_string()];
    for abi in [
        "aarch64-linux-android",
        "armv7a-linux-androideabi",
        "i686-linux-android",
        "x86_64-linux-android",
    ] {
        let file = root.join(format!("tests/fixtures/android-jni/{abi}.so"));
        let mut program = bootstrap_from_object(file.to_str().unwrap(), "", &specs).unwrap();
        program.commit_pending_analysis().unwrap();
        verify_table(program.arch(), "JNIEnv", "JNINativeInterface", 233, 4);
        verify_table(program.arch(), "JavaVM", "JNIInvokeInterface", 8, 3);
    }
}
