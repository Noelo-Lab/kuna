//! Shared helpers of the driver-layer tests (inputs, options, spans, deadline, driver).

#![allow(dead_code)]

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, Severity};
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_input::InputSet;
use kuna_jvm::rt::Caught;

/// A corpus class file or directory (`tests/jvm/classes/<rel>`).
pub fn corpus(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tests/jvm/classes")
        .join(rel)
}

/// A scratch directory of this test binary, created if needed.
pub fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("kuna-driver")
        .join(name);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Copies corpus class files (`java8/pkg/X.class`, ...) into `dir/pkg/`.
pub fn copy_classes(dir: &Path, files: &[&str]) {
    for rel in files {
        let from = corpus(rel);
        let package = Path::new(rel).parent().unwrap().file_name().unwrap();
        let to_dir = dir.join(package);
        std::fs::create_dir_all(&to_dir).unwrap();
        std::fs::copy(&from, to_dir.join(from.file_name().unwrap())).unwrap();
    }
}

/// Options for machine-independent runs: no JDK runtime, quiet log.
pub fn options(extra: &[(&str, &str)]) -> Vec<(String, String)> {
    let mut options = vec![
        ("include-runtime".to_string(), "0".to_string()),
        ("log-level".to_string(), "error".to_string()),
    ];
    options.extend(extra.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    options
}

/// One logged message.
#[derive(Clone, Debug)]
pub struct Logged {
    pub severity: Severity,
    pub message: String,
    pub throwable: Option<String>,
}

/// A logger that keeps every message and the method being decompiled.
pub struct RecordingLogger {
    severity: Cell<Severity>,
    pub records: RefCell<Vec<Logged>>,
    pub method: Rc<RefCell<String>>,
}

impl RecordingLogger {
    pub fn new() -> RecordingLogger {
        RecordingLogger {
            severity: Cell::new(Severity::Trace),
            records: RefCell::new(Vec::new()),
            method: Rc::new(RefCell::new(String::new())),
        }
    }
}

impl IFernflowerLogger for RecordingLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, message: &str, severity: Severity) {
        self.records.borrow_mut().push(Logged {
            severity,
            message: message.to_string(),
            throwable: None,
        });
    }

    fn write_message_ex(&self, message: &str, severity: Severity, t: &Caught) {
        self.records.borrow_mut().push(Logged {
            severity,
            message: message.to_string(),
            throwable: Some(t.message.clone()),
        });
    }

    fn start_method(&self, method_name: &str) {
        *self.method.borrow_mut() = method_name.to_string();
    }

    fn end_method(&self) {
        self.method.borrow_mut().clear();
    }
}

/// A full single-instance run over `input` on an engine thread.
pub fn decompile(input: &InputSet, options: &[(String, String)]) -> Vec<DecompiledClass> {
    decompile_with(input, options, |_| {}).0
}

/// [`decompile`] with `setup` run on the engine thread first (it receives the
/// logger's current-method cell); returns the units and the log.
pub fn decompile_with(
    input: &InputSet,
    options: &[(String, String)],
    setup: impl FnOnce(Rc<RefCell<String>>) + Send,
) -> (Vec<DecompiledClass>, Vec<Logged>) {
    with_engine_stack(|| {
        reset_engine_thread();
        let logger = Rc::new(RecordingLogger::new());
        setup(logger.method.clone());
        let mut fernflower = Fernflower::new(options, logger.clone());
        for source in input.sources() {
            fernflower.add_source(source);
        }
        let mut units = Vec::new();
        fernflower.decompile_context(&mut |unit| units.push(unit));
        fernflower.clear_context();
        reset_engine_thread();
        kuna_jvm::cancelation_manager::CancelationManager::set_method_guard(None);
        let log = logger.records.borrow().clone();
        (units, log)
    })
}

/// A stored (uncompressed) zip archive of `entries`.
pub fn make_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    fn crc32(data: &[u8]) -> u32 {
        let mut crc = !0u32;
        for &b in data {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }
    let mut out = Vec::new();
    let mut central = Vec::new();
    for (name, data) in entries {
        let offset = out.len() as u32;
        let crc = crc32(data);
        let header = |sig: u32, out: &mut Vec<u8>| {
            out.extend(sig.to_le_bytes());
        };
        header(0x0403_4b50, &mut out);
        out.extend(20u16.to_le_bytes());
        out.extend([0u8; 4]);
        out.extend([0u8; 4]);
        out.extend(crc.to_le_bytes());
        out.extend((data.len() as u32).to_le_bytes());
        out.extend((data.len() as u32).to_le_bytes());
        out.extend((name.len() as u16).to_le_bytes());
        out.extend(0u16.to_le_bytes());
        out.extend(name.as_bytes());
        out.extend(*data);
        header(0x0201_4b50, &mut central);
        central.extend(20u16.to_le_bytes());
        central.extend(20u16.to_le_bytes());
        central.extend([0u8; 4]);
        central.extend([0u8; 4]);
        central.extend(crc.to_le_bytes());
        central.extend((data.len() as u32).to_le_bytes());
        central.extend((data.len() as u32).to_le_bytes());
        central.extend((name.len() as u16).to_le_bytes());
        central.extend([0u8; 12]);
        central.extend(offset.to_le_bytes());
        central.extend(name.as_bytes());
    }
    let cd_offset = out.len() as u32;
    let cd_len = central.len() as u32;
    out.extend(central);
    out.extend(0x0605_4b50u32.to_le_bytes());
    out.extend([0u8; 4]);
    out.extend((entries.len() as u16).to_le_bytes());
    out.extend((entries.len() as u16).to_le_bytes());
    out.extend(cd_len.to_le_bytes());
    out.extend(cd_offset.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out
}
