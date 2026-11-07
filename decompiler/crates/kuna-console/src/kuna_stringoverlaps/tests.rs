use super::*;

fn array(name: &str, address: u64, size: i64) -> GlobalInfo {
    GlobalInfo {
        address,
        name: name.into(),
        declaration: format!("char {name}[{size}]"),
        size,
        unknown: false,
        direct: true,
        aggregate: false,
        elem: false,
        integer: false,
    }
}

#[test]
fn complete_owner_keeps_overlapping_array_types_and_offsets() {
    let views = [
        array("whole", 0x5000, 16),
        array("part", 0x5004, 8),
        array("nested", 0x5006, 5),
    ];
    let declarations = declarations(views.iter(), &BTreeSet::new(), &[]);
    assert_eq!(declarations[&0x5000], "extern char whole[16];");
    assert_eq!(
        declarations[&0x5004],
        "#define part (*(char (*)[8])((unsigned char *)&whole + 0x4))"
    );
    assert_eq!(
        declarations[&0x5006],
        "#define nested (*(char (*)[5])((unsigned char *)&whole + 0x6))"
    );
}

#[test]
fn uncertain_or_incompatible_storage_remains_diagnostic() {
    let whole = array("whole", 0x5000, 16);
    let part = array("part", 0x5004, 8);
    let mut incompatible = part.clone();
    incompatible.declaration = "unsigned int part".into();
    incompatible.size = 4;
    incompatible.integer = true;
    let mut unknown = part.clone();
    unknown.unknown = true;
    let mut taken = part.clone();
    taken.direct = false;
    for views in [
        vec![whole.clone(), incompatible],
        vec![whole.clone(), unknown],
        vec![whole.clone(), taken],
        vec![whole.clone(), array("tail", 0x500c, 8)],
        vec![whole.clone(), part.clone(), array("alias", 0x5004, 8)],
        vec![whole.clone(), part.clone(), array("part", 0x6000, 8)],
        vec![whole.clone(), array("for", 0x5004, 8)],
        vec![whole.clone(), array("ns::part", 0x5004, 8)],
        vec![whole.clone(), array("part", 0x5010, 8)],
        vec![
            array("whole", u64::MAX - 15, 16),
            array("part", u64::MAX - 4, 4),
        ],
    ] {
        assert!(
            declarations(views.iter(), &BTreeSet::new(), &[]).is_empty(),
            "{views:?}"
        );
    }
}

#[test]
fn emitted_aliases_share_writes_and_relocate_with_the_owner() {
    use std::process::Command;
    let compiler = match Command::new("cc").arg("--version").output() {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("no C compiler for overlapping string alias observer");
            return;
        }
        Err(error) => panic!("run C compiler: {error}"),
    };
    assert!(compiler.status.success(), "C compiler version probe failed");
    let views = [
        array("whole", 0x5000, 16),
        array("part", 0x5004, 8),
        array("nested", 0x5006, 5),
    ];
    let header = declarations(views.iter(), &BTreeSet::new(), &[])
        .into_values()
        .collect::<Vec<_>>()
        .join("\n");
    let source = format!(
        r#"#include <assert.h>
#include <stdint.h>
{header}
char whole[16];
int main(void) {{
    _Static_assert(sizeof(whole) == 16, "owner extent");
    _Static_assert(sizeof(part) == 8, "part extent");
    _Static_assert(sizeof(nested) == 5, "nested extent");
    assert((uintptr_t)&whole != 0x5000);
    assert((char *)&part == whole + 4);
    assert((char *)&nested == whole + 6);
    char (*view)[8] = &part;
    for (unsigned value = 0; value < 256; ++value) {{
        whole[4] = (char)value;
        assert((unsigned char)(*view)[0] == value);
        part[2] = (char)(255 - value);
        assert((unsigned char)whole[6] == 255 - value);
        assert((unsigned char)nested[0] == 255 - value);
        nested[4] = (char)value;
        assert((unsigned char)part[6] == value);
        assert((unsigned char)whole[10] == value);
    }}
}}
"#,
    );
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "kuna-string-overlaps-{}-{stamp}",
        std::process::id()
    ));
    std::fs::create_dir(&dir).unwrap();
    let input = dir.join("observer.c");
    std::fs::write(&input, source).unwrap();
    for (index, flags) in [
        vec!["-O0"],
        vec!["-O3"],
        vec![
            "-O1",
            "-fsanitize=undefined",
            "-fno-sanitize-recover=undefined",
        ],
    ]
    .iter()
    .enumerate()
    {
        let binary = dir.join(format!("observer-{index}"));
        let compiled = Command::new("cc")
            .args(["-std=c11", "-Werror"])
            .args(flags)
            .arg(&input)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            compiled.status.success(),
            "{}",
            String::from_utf8_lossy(&compiled.stderr)
        );
        let executed = Command::new(&binary).output().unwrap();
        assert!(
            executed.status.success(),
            "{}",
            String::from_utf8_lossy(&executed.stderr)
        );
    }
    std::fs::remove_dir_all(dir).unwrap();
}
