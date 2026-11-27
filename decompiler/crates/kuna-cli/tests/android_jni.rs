use crate::common;
use serde_json::Value;

fn image(triple: &str) -> String {
    common::repo_root()
        .join(format!("tests/fixtures/android-jni/{triple}.so"))
        .to_str()
        .unwrap()
        .to_owned()
}

fn output(args: &[&str]) -> String {
    let (text, stderr, status) = common::run_kuna(args);
    assert_eq!(status, 0, "{stderr}");
    text
}

#[test]
fn ndk_interfaces_and_registered_methods_use_the_target_abi() {
    for triple in [
        "aarch64-linux-android",
        "armv7a-linux-androideabi",
        "i686-linux-android",
        "x86_64-linux-android",
    ] {
        let image = image(triple);
        let hook = output(&["decompile", &image, "JNI_OnLoad"]);
        for expected in [
            "JNI_OnLoad(JavaVM",
            "GetEnv",
            "FindClass",
            "RegisterNatives",
        ] {
            assert!(
                hook.contains(expected),
                "{triple}: missing {expected}\n{hook}"
            );
        }

        let json = output(&["functions", &image, "--json"]);
        let inventory: Value = serde_json::from_str(&json).unwrap();
        let functions = inventory["functions"].as_array().unwrap();
        let addr = |name: &str| {
            functions
                .iter()
                .find_map(|entry| {
                    entry["name"]
                        .as_str()?
                        .contains(name)
                        .then(|| entry["address_hex"].as_str().unwrap())
                })
                .unwrap_or_else(|| panic!("{triple}: missing {name} in {json}"))
        };

        let counted = output(&["decompile", &image, addr("countBytes"), "--addr"]);
        for expected in ["countBytes", "JNIEnv", "jbyteArray", "GetArrayLength"] {
            assert!(
                counted.contains(expected),
                "{triple}: missing {expected}\n{counted}"
            );
        }
        let disabled = output(&[
            "decompile",
            &image,
            addr("countBytes"),
            "--addr",
            "--option",
            "jniregister",
            "off",
        ]);
        assert!(!disabled.contains("countBytes"), "{triple}: {disabled}");

        let exported = output(&["decompile", &image, "Java_org_example_Bridge_size___3B"]);
        assert!(exported.contains("jbyteArray"), "{triple}: {exported}");
        assert!(exported.contains("GetArrayLength"), "{triple}: {exported}");
        assert!(exported.contains("return "), "{triple}: {exported}");

        let critical = output(&["decompile", &image, addr("increment"), "--addr"]);
        assert!(!critical.contains("JNIEnv"), "{triple}: {critical}");
        let ordinary = output(&["decompile", &image, "ordinary_lookup"]);
        assert!(!ordinary.contains("FindClass"), "{triple}: {ordinary}");
    }
}

#[test]
fn a_table_a_helper_patches_through_an_index_is_not_registered() {
    for abi in ["aarch64", "x86_64"] {
        let json = output(&["functions", &image(&format!("patched-helper-{abi}")), "--json"]);
        assert!(json.contains("Java_x_Z_kept"), "{abi}: {json}");
        assert!(!json.contains("Java_x_Y_patched"), "{abi}: {json}");
    }
}

#[test]
fn a_linux_elf_can_export_the_same_jni_names() {
    let image = image("x86_64-linux-gnu");
    let hook = output(&["decompile", &image, "JNI_OnLoad"]);
    for android_type in ["JavaVM", "FindClass", "RegisterNatives"] {
        assert!(!hook.contains(android_type), "{hook}");
    }
}

#[test]
fn explicit_prototypes_can_use_the_loaded_types_and_override_a_hook() {
    let image = image("aarch64-linux-android");
    let typed = output(&[
        "decompile",
        &image,
        "ordinary_lookup",
        "--assert",
        "prototype ordinary_lookup jint ordinary_lookup(JNIEnv *env)",
    ]);
    assert!(typed.contains("ordinary_lookup(JNIEnv *env)"), "{typed}");
    assert!(typed.contains("FindClass"), "{typed}");
    let overridden = output(&[
        "decompile",
        &image,
        "JNI_OnLoad",
        "--assert",
        "prototype JNI_OnLoad unsigned int JNI_OnLoad(int *overridden)",
    ]);
    assert!(
        overridden.contains("unsigned int JNI_OnLoad(int *overridden)"),
        "{overridden}"
    );
}
