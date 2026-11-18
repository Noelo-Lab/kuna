//! Embedded CVF metadata uses the normal PE analysis and comment commit paths.
use crate::common;
use object::{Object, ObjectSection, ObjectSymbol};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const SOURCE: &str = "__attribute__((section(\".trace\"), used)) const unsigned char metadata[1 << 18] = {1};\n\
int sample(void) { return 7; }\n\
__attribute__((noinline)) int helper(int x) { return x * 3 + 1; }\n\
__declspec(dllexport) int exported(int a) { return sample() + helper(a + 2); }\n";

struct Image {
    stripped: Vec<u8>,
    symbols: Vec<u8>,
    trace: usize,
    sample: u32,
    helper: u32,
    exported: u32,
}

fn link(root: &Path, extra: &[&str], out: &str) -> Result<Vec<u8>, String> {
    let binary = root.join(out);
    let output = Command::new("clang")
        .args(["--target=i686-w64-windows-gnu", "-shared", "-nostdlib", "-O1"])
        .args(["-Wl,--entry,_sample", "-Wl,--image-base,0x10000000"])
        .args(extra)
        .arg(root.join("sample.c"))
        .arg("-o")
        .arg(&binary)
        .output()
        .map_err(|e| format!("clang: {e}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    std::fs::read(binary).map_err(|e| e.to_string())
}

fn build(root: &Path) -> Result<Image, String> {
    std::fs::write(root.join("sample.c"), SOURCE).unwrap();
    let symbols = link(root, &[], "symbols.dll")?;
    let stripped = link(root, &["-Wl,-s"], "stripped.dll")?;
    let file = object::File::parse(symbols.as_slice()).map_err(|e| e.to_string())?;
    let at = |name: &str| {
        file.symbols()
            .find(|s| s.name() == Ok(name))
            .map(|s| s.address() as u32)
            .ok_or(format!("no {name}"))
    };
    let trace = file.section_by_name(".trace").ok_or(".trace not kept")?;
    Ok(Image {
        trace: trace.file_range().ok_or(".trace has no bytes")?.0 as usize,
        sample: at("_sample")?,
        helper: at("_helper")?,
        exported: at("_exported")?,
        stripped,
        symbols,
    })
}

fn record(low: u32, files: &[&str], routines: &[(u32, &str)]) -> Vec<u8> {
    let mut b = vec![1];
    b.extend(&(files.len() as u32).to_le_bytes()[..3]);
    b.extend(low.to_le_bytes());
    b.extend(0x100u32.to_le_bytes());
    b.extend([0; 4]);
    for file in files {
        b.extend((file.len() as u16).to_le_bytes());
        b.extend(file.as_bytes());
    }
    for (i, (address, name)) in routines.iter().enumerate() {
        b.extend([2, 0]);
        b.extend((name.len() as u16).to_le_bytes());
        b.extend(address.to_le_bytes());
        b.extend(name.as_bytes());
        if i == 0 {
            b.extend([3, 0, 0, 0, 0]);
        }
        b.extend([0xc0, 7]);
    }
    let size = b.len() as u32;
    b[12..16].copy_from_slice(&size.to_le_bytes());
    b
}

fn traced(root: &Path, image: &Image, base: &[u8], name: &str, trace: &[u8]) -> PathBuf {
    let mut bytes = base.to_vec();
    bytes[image.trace..image.trace + trace.len()].copy_from_slice(trace);
    let path = root.join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

fn kuna(args: &[&std::ffi::OsStr]) -> (String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .arg("--sleighpath")
        .arg(common::repo_root().join("specs"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

/// Function name by entry address, and the C of each, for one option value.
fn functions(binary: &Path, mode: &str) -> BTreeMap<u32, (String, String)> {
    let (json, _) = kuna(&[
        "decompile-all".as_ref(),
        binary.as_os_str(),
        "--json".as_ref(),
        "--option".as_ref(),
        "cvftrace".as_ref(),
        mode.as_ref(),
    ]);
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    json["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["address"].as_u64().unwrap() as u32,
                (
                    f["name"].as_str().unwrap().to_string(),
                    f["code"].as_str().unwrap_or_default().to_string(),
                ),
            )
        })
        .collect()
}

#[test]
fn pe_trace_names_only_free_placeholders() {
    let root = common::scratch_file("cvf-trace", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let image = match build(&root) {
        Ok(image) => image,
        Err(why) => {
            eprintln!("SKIP cvf_trace: cannot link an i686 mingw DLL with clang: {why}");
            return;
        }
    };
    let low = image.sample;
    let names = |path: &Path, mode: &str| functions(path, mode);
    let name = |map: &BTreeMap<u32, (String, String)>, at: u32| map[&at].0.clone();

    let plain = record(low, &["minimal.for"], &[(image.sample, "FROM_TRACE"), (image.helper, "HELPER_TRACE"), (image.exported, "EXPORTED_TRACE")]);
    let binary = traced(&root, &image, &image.stripped, "plain.dll", &plain);
    let on = names(&binary, "on");
    assert_eq!(name(&on, image.sample), "FROM_TRACE");
    assert_eq!(name(&on, image.helper), "HELPER_TRACE");
    assert_eq!(name(&on, image.exported), "exported");
    assert!(on.values().all(|(_, c)| !c.contains("minimal.for") && !c.contains("EXPORTED_TRACE")));
    let lines = names(&binary, "lines");
    assert!(lines[&image.sample].1.contains("minimal.for:7"), "{}", lines[&image.sample].1);
    let off = names(&binary, "off");
    assert!(off.values().all(|(n, c)| !n.ends_with("_TRACE") && !c.contains("_TRACE")));
    kuna(&["decompile".as_ref(), binary.as_os_str(), "FROM_TRACE".as_ref()]);

    let collide = record(low, &["a.for"], &[(image.helper, "exported"), (image.sample, "DUP"), (image.exported, "DUP")]);
    let binary = traced(&root, &image, &image.stripped, "collide.dll", &collide);
    let on = names(&binary, "on");
    assert_eq!(name(&on, image.exported), "exported");
    assert!(name(&on, image.helper).starts_with("sub_"), "{on:?}");
    assert!(name(&on, image.sample).starts_with("sub_"), "{on:?}");
    let (by_name, _) = kuna(&["decompile".as_ref(), binary.as_os_str(), "exported".as_ref()]);
    let helper_call = format!("sub_{:x}(", image.helper);
    assert!(by_name.contains("exported(") && by_name.contains(&helper_call), "{by_name}");

    let interior = record(low, &["a.for"], &[(image.sample + 1, "MID_FUNCTION"), (image.helper + 2, "MID_INSN")]);
    let binary = traced(&root, &image, &image.stripped, "interior.dll", &interior);
    let base = names(&binary, "off");
    let on = names(&binary, "on");
    assert_eq!(base, on, "a routine marker inside a function created or split one");

    let coff = record(low, &["a.for"], &[(image.sample, "FROM_TRACE"), (image.exported, "_helper")]);
    let binary = traced(&root, &image, &image.symbols, "coff.dll", &coff);
    let on = names(&binary, "on");
    assert_eq!(name(&on, image.sample), "_sample");
    assert_eq!(name(&on, image.helper), "_helper");
    assert!(on.values().all(|(n, c)| n != "FROM_TRACE" && !c.contains("FROM_TRACE")));

    let mut capped = record(low, &vec![""; 65537], &[(image.sample, "LOST")]);
    capped.extend(record(low, &["a.for"], &[(image.helper, "KEPT")]));
    let binary = traced(&root, &image, &image.stripped, "capped.dll", &capped);
    let (json, stderr) = kuna(&["decompile-all".as_ref(), binary.as_os_str(), "--json".as_ref()]);
    assert!(stderr.contains("[kuna cvftrace]"), "{stderr}");
    assert!(json.contains("\"KEPT\"") && !json.contains("LOST"), "{json}");

    let binary = traced(&root, &image, &image.stripped, "project.dll", &plain);
    let entry = format!("0x{:x}", image.sample);
    let only = root.join("only.c");
    std::fs::write(&only, "int sample(void) { return 7; }\n").unwrap();
    let text = |source: &Path, object: &Path| {
        let built = Command::new("clang")
            .args(["--target=i686-w64-windows-gnu", "-O1", "-c"])
            .arg(source)
            .arg("-o")
            .arg(object)
            .output()
            .unwrap();
        assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
        let bytes = std::fs::read(object).unwrap();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let text = file.section_by_name(".text").unwrap().data().unwrap().to_vec();
        text
    };
    let expected = text(&only, &root.join("original.obj"));
    for mode in ["on", "lines"] {
        let project = root.join(mode);
        kuna(&[
            "decompile-project".as_ref(),
            binary.as_os_str(),
            "--addr".as_ref(),
            entry.as_ref(),
            "--option".as_ref(),
            "cvftrace".as_ref(),
            mode.as_ref(),
            "--assert".as_ref(),
            format!("prototype {entry} int sample(void)").as_ref(),
            "-o".as_ref(),
            project.as_os_str(),
        ]);
        let regenerated = text(&project.join("project.dll.c"), &project.join("regenerated.obj"));
        assert_eq!(expected, regenerated, "{mode}");
    }

    let mut renamed = std::fs::read(&binary).unwrap();
    let at = renamed.windows(8).position(|b| b == b".trace\0\0").unwrap();
    renamed[at..at + 8].copy_from_slice(b".other\0\0");
    std::fs::write(&binary, renamed).unwrap();
    assert_eq!(names(&binary, "off"), names(&binary, "lines"));
}
