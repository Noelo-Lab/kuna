//! The `Bootstrap` builder carries explicit options to each input it loads,
//! and one builder serves several threads.

use crate::common;

use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_analysis::loadimage_te::synthetic::TeImage;
use kuna_base::error::KunaResult;
use kuna_base::xml::{DocumentStorage, Element};
use kuna_console::engine::{language_database, Bootstrap, ConsoleProgram};
use kuna_console::project::{decompile_entry, DecompileOptions};
use kuna_decomp::kuna_loadoptions::{environment_fallbacks, LoadOptions};

const X86_64: &str = "x86:LE:64:default";
/// `lea rax, [rdi+rbx]; ret`
const X86_ADD: [u8; 5] = [0x48, 0x8d, 0x04, 0x1f, 0xc3];
/// Thumb `movs r0, #7; bx lr`, which decodes as something else in A32.
const THUMB_RETURN_7: [u8; 4] = [0x07, 0x20, 0x70, 0x47];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn specs() -> Vec<String> {
    vec![repo_root().join("specs").to_string_lossy().into_owned()]
}

fn fixture(name: &str) -> String {
    let path = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name);
    path.to_str().unwrap().to_string()
}

struct Scratch(PathBuf);

impl Scratch {
    fn new(stem: &str, extension: &str, bytes: &[u8]) -> Self {
        let path = common::scratch_file(stem, extension);
        std::fs::write(&path, bytes).unwrap();
        Self(path)
    }

    fn path(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// What a load built, as far as a caller can see it: the language, the
/// description, the sections, every function and the C of the first few, or
/// the error.
fn fingerprint(load: KunaResult<ConsoleProgram>) -> String {
    let mut prog = match load {
        Ok(prog) => prog,
        Err(err) => return format!("error: {err}"),
    };
    prog.commit_pending_analysis()
        .expect("analysis commit succeeds");
    let mut out = format!(
        "{}\n{}\n{:?}\nsymbols {}\n",
        prog.arch().archid,
        prog.description(),
        prog.sections(),
        prog.num_symbols()
    );
    let entries = prog.function_entries_canonical();
    for entry in &entries {
        out += &format!("{} @ {:#x}\n", entry.name, entry.addr.get_offset());
    }
    for entry in entries.iter().take(3) {
        let result = decompile_entry(&mut prog, entry.clone(), &DecompileOptions::default());
        match result.code {
            Some(code) => out += &code,
            None => out += &format!("decompile error: {:?}\n", result.error),
        }
    }
    out
}

/// Entry `0000:0000` far-calls `0002:0000`, which returns 9 with `RETF`.
fn dos_image() -> Vec<u8> {
    let mut module = vec![0x90u8; 0x24];
    module[..10].copy_from_slice(&[0x9a, 0x00, 0x00, 0x02, 0x00, 0xb8, 0x00, 0x4c, 0xcd, 0x21]);
    module[0x20..0x24].copy_from_slice(&[0xb8, 0x09, 0x00, 0xcb]);
    let mut header = vec![0u8; 0x20];
    header[..2].copy_from_slice(b"MZ");
    header[0x1c..0x20].copy_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    let total = header.len() + module.len();
    let words: [(usize, u16); 6] = [
        (0x02, (total % 512) as u16),
        (0x04, total.div_ceil(512) as u16),
        (0x06, 1),
        (0x08, (header.len() / 16) as u16),
        (0x10, 0x100),
        (0x18, 0x1c),
    ];
    for (at, value) in words {
        header[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    header.extend_from_slice(&module);
    header
}

const XML_IMAGE: &str = r#"<binaryimage arch="x86:LE:64:default"><bytechunk space="ram" offset="0x1000">488d041fc3</bytechunk><symbol space="ram" offset="0x1000" name="add"/></binaryimage>"#;

fn xml_root() -> Rc<Element> {
    DocumentStorage::new()
        .parse_document(XML_IMAGE.as_bytes())
        .unwrap()
        .get_root()
        .clone()
}

type Load<'a> = dyn Fn() -> KunaResult<ConsoleProgram> + 'a;

#[test]
fn explicit_options_reach_every_input_without_the_environment() {
    let specs = specs();
    let db = language_database(&specs).unwrap();
    let raw = Scratch::new("bootstrap-scoped-raw", "bin", &X86_ADD);
    let te_bytes = TeImage::thumb(&THUMB_RETURN_7).build();
    let te = Scratch::new("bootstrap-scoped-te", "te", &te_bytes);
    let mz_bytes = dos_image();
    let mz = Scratch::new("bootstrap-scoped-mz", "exe", &mz_bytes);
    let xml = Scratch::new("bootstrap-scoped-xml", "xml", XML_IMAGE.as_bytes());
    let fauxware = fixture("fauxware");
    let options = LoadOptions {
        goabi: false,
        ..LoadOptions::default()
    };
    let load = Bootstrap::new(&db).options(&options);

    let loads: Vec<(&str, Box<Load>)> = vec![
        ("object", Box::new(|| load.object(&fauxware))),
        ("image", Box::new(|| load.image(te.path()))),
        ("file", Box::new(|| load.file(xml.path()))),
        (
            "raw",
            Box::new(|| load.target(X86_64).raw(raw.path(), 0x1000, &[0x1000])),
        ),
        ("te", Box::new(|| load.te(te.path()))),
        ("te bytes", Box::new(|| load.te_bytes(te.path(), &te_bytes))),
        ("mz bytes", Box::new(|| load.mz_bytes(mz.path(), &mz_bytes))),
        ("xml root", Box::new(|| load.xml_root(&xml_root()))),
    ];
    for (input, bootstrap) in loads {
        let before = environment_fallbacks();
        let prog = bootstrap().unwrap_or_else(|err| panic!("{input}: {err}"));
        assert_eq!(
            environment_fallbacks(),
            before,
            "{input} read the environment"
        );
        assert!(
            !prog.arch().load_options().goabi,
            "{input} dropped its options"
        );
    }
}

#[test]
fn a_spec_root_array_or_vec_is_a_language_source() {
    let path: &Path = &repo_root().join("specs");
    let fauxware = fixture("fauxware");
    let from_array = fingerprint(Bootstrap::new(&[path]).object(&fauxware));
    let from_vec = fingerprint(Bootstrap::new(&vec![path]).object(&fauxware));
    let from_slice = fingerprint(Bootstrap::new(&specs()[..]).object(&fauxware));
    assert_eq!(from_array, from_vec);
    assert_eq!(from_array, from_slice);
}

#[test]
fn one_builder_loads_on_several_threads() {
    let specs = specs();
    let db = language_database(&specs).unwrap();
    let fauxware = fixture("fauxware");
    let options = LoadOptions::default();
    let load = Bootstrap::new(&db).options(&options);
    let expected = fingerprint(load.object(&fauxware));
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..2)
            .map(|_| scope.spawn(|| fingerprint(load.object(&fauxware))))
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), expected);
        }
    });
}
