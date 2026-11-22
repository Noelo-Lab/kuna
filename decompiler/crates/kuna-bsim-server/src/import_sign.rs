//! Normalize and sign loaded function bodies for the server store.

use std::fs::Metadata;
use std::path::PathBuf;
use std::time::Instant;

use kuna_analysis::bsim::corpus::CorpusExecutable;
use kuna_analysis::bsim::lshweights::LshProfile;
use kuna_analysis::bsim::signature::{BsimSignature, sign_function_with_settings};
use kuna_analysis::ghidra_program::{ProgramEntry, ProgramFunction};
use kuna_analysis::loader::macho_fat::{fat_slice_range, is_fat, slice_pref};
use kuna_bsim_server::{MachOSlice, MAX_ARTIFACT_BYTES, MAX_NAME_BYTES, MAX_QUERY_FEATURES, PublishRequest};
use kuna_console::engine::{Bootstrap, ConsoleProgram, EntrySelector, FunctionEntry};
use kuna_decomp::kuna_memoryspecs::{MemorySpecs, SpecRoot};
use kuna_decomp::options::OptionDatabase;
use kuna_sleigh::loadimage::section_flags;
use md5::{Digest, Md5};

pub(super) fn sign_import(
    binary: &str,
    language_id: &str,
    functions: Vec<ProgramFunction>,
    requested_profile: Option<LshProfile>,
    display_name: String,
) -> Result<(LshProfile, CorpusExecutable, SigningCoverage), String> {
    if functions.is_empty() {
        return Err("project has no named saved function entries".into());
    }
    let mut image = PreparedImage::load(binary, requested_profile, Some(language_id))?;
    let sections = image.program.sections();
    let mut coverage = SigningCoverage { considered: functions.len(), first_failure: None };
    let mut signed = Vec::new();
    for stored in functions {
        let selector = match stored.location {
            ProgramEntry::LinkedAddress(address) => EntrySelector::Numeric(address),
            ProgramEntry::ElfSection { section_index, offset } =>
                EntrySelector::SectionIndexOffset { section_index, offset },
        };
        let result = image.program.resolve_entry(&selector)
            .map_err(|err| format!("saved entry {:x?}: {err}", stored.location))
            .and_then(|mut entry| {
                let prog = &image.program;
                let offset = entry.addr.get_offset();
                let exact = match stored.location {
                    ProgramEntry::LinkedAddress(address) => prog.output_code_offset(offset) == address,
                    ProgramEntry::ElfSection { section_index, offset } =>
                        entry.object_location.as_ref().is_some_and(|location| {
                            let arm_mode_bit = u64::from(prog.is_arm32());
                            location.section_index == section_index
                                && location.offset & !arm_mode_bit == offset
                        }),
                };
                if !exact
                    || !prog.is_body_entry(offset)
                    || (!sections.is_empty() && !in_executable_section(offset, &sections))
                {
                    return Err(format!(
                        "saved entry {:x?} does not resolve to the same executable location in the original image",
                        stored.location
                    ));
                }
                entry.name = stored.name;
                image.sign_entry(entry)
            });
        match result {
            Ok(signature) => signed.push(signature),
            Err(error) => { coverage.first_failure.get_or_insert(error); }
        }
    }
    if signed.is_empty() {
        return Err(coverage.first_failure.expect("nonempty saved entries all failed"));
    }
    // Signing every saved entry does not establish native discovery coverage.
    let (profile, executable) = image.finish(display_name, signed, false);
    Ok((profile, executable, coverage))
}

pub(super) fn sign_publication(
    bytes: Vec<u8>,
    request: &PublishRequest,
    roots: &[SpecRoot],
    deadline: Instant,
) -> Result<(LshProfile, CorpusExecutable), String> {
    let selector = match (request.address, request.symbol.as_deref()) {
        (Some(address), None) => EntrySelector::Numeric(address),
        (None, Some(symbol)) if !symbol.is_empty() => EntrySelector::Name(symbol.to_owned()),
        _ => return Err("provide exactly one function address or nonempty symbol".into()),
    };
    let profile = request.profile.as_deref().map(|name| {
        LshProfile::from_name(name).ok_or_else(|| format!("unknown BSim profile {name:?}"))
    }).transpose()?;
    let md5 = kuna_base::hex_lower(&Md5::digest(&bytes));
    let program = Bootstrap::new(roots).standalone_image(bytes)
        .map_err(|err| format!("load original executable: {}", err.explain()))?;
    let mut image = PreparedImage::prepare(program, profile, md5)?;
    let prog = &image.program;
    let entry = match &selector {
        // An explicit address can declare code Kuna has not discovered.
        // Numeric lookup may also fall back to a raw .o section offset; reject
        // that redirect so the address in the store is the one requested.
        EntrySelector::Numeric(requested) => {
            let entry = prog.resolve_entry(&selector).map_err(|e| e.to_string())?;
            if prog.output_code_offset(entry.addr.get_offset()) != *requested {
                return Err(format!("{requested:#x} is not an exact loaded executable address"));
            }
            entry
        }
        EntrySelector::Name(symbol) => {
            let name = kuna_decomp::kuna_symbolnamebound::bound_scope_path(symbol, "::");
            let name: &str = name.as_ref();
            let mut matches = prog.function_entries_executable().into_iter()
                .filter(|entry| entry.name == name || entry.aliases.iter().any(|alias| alias == name));
            let entry = matches.next().ok_or_else(|| format!("no executable entry named {symbol:?}"))?;
            if let Some(other) = matches.next() {
                return Err(format!(
                    "executable symbol {symbol:?} is ambiguous ({:#x}, {:#x}); select a loaded address",
                    prog.output_code_offset(entry.addr.get_offset()),
                    prog.output_code_offset(other.addr.get_offset()),
                ));
            }
            entry
        }
        _ => unreachable!("publication selector is an address or a symbol"),
    };
    let offset = entry.addr.get_offset();
    let address = prog.output_code_offset(offset);
    if !prog.is_body_entry(offset)
        || !in_executable_section(offset, &prog.sections())
        || !prog.entry_bytes_mapped(&entry.addr)
    {
        return Err(format!("{} at {address:#x} is not an exact loaded executable entry", entry.name));
    }
    image.program.arch_mut().kuna_fn_deadline = Some(deadline);
    let result = image.sign_entry(entry);
    if Instant::now() >= deadline {
        return Err("function signing exceeded the worker deadline".into());
    }
    let mut signature = result?;
    if let Some(rename) = &request.rename {
        signature.name.clone_from(rename);
    }
    Ok(image.finish(request.executable.clone(), vec![signature], false))
}

pub(super) fn check_artifact(metadata: &Metadata) -> Result<(), String> {
    if !metadata.is_file() {
        return Err("original binary must be a regular file".into());
    }
    if metadata.len() > MAX_ARTIFACT_BYTES {
        return Err(format!("original binary exceeds the {MAX_ARTIFACT_BYTES}-byte artifact limit"));
    }
    Ok(())
}

pub(super) struct SigningCoverage {
    pub(super) considered: usize,
    pub(super) first_failure: Option<String>,
}

pub(super) fn sign_binary(
    binary: &std::path::Path,
    name: String,
    profile: Option<LshProfile>,
) -> Result<(LshProfile, CorpusExecutable, SigningCoverage, Option<MachOSlice>), String> {
    let bytes = std::fs::read(binary).map_err(|err| format!("read original executable: {err}"))?;
    let macho_slice = if is_fat(&bytes) {
        let (offset, size) = fat_slice_range(&bytes, slice_pref(None, None))
            .ok_or("universal Mach-O has no usable selected slice")?;
        Some(MachOSlice { offset, size })
    } else {
        None
    };
    let mut image = PreparedImage::load_bytes(bytes, profile, None, macho_slice)?;
    // The loader inventory carries symbol names and synthetic VMAs for .o
    // definitions, and excludes import pointer slots and non-code sections.
    let entries = image.program.function_entries_executable();
    let (signed, coverage) = image.sign_entries(entries)?;
    let (profile, executable) = image.finish(name, signed, coverage.first_failure.is_none());
    Ok((profile, executable, coverage, macho_slice))
}

struct PreparedImage {
    program: ConsoleProgram,
    profile: LshProfile,
    settings: u32,
    architecture: String,
    compiler: String,
    md5: String,
}

impl PreparedImage {
    fn load(
        binary: &str, requested_profile: Option<LshProfile>, saved_language: Option<&str>,
    ) -> Result<Self, String> {
        let bytes = std::fs::read(binary).map_err(|err| format!("read original executable: {err}"))?;
        Self::load_bytes(bytes, requested_profile, saved_language, None)
    }

    fn load_bytes(
        bytes: Vec<u8>,
        requested_profile: Option<LshProfile>,
        saved_language: Option<&str>,
        macho_slice: Option<MachOSlice>,
    ) -> Result<Self, String> {
        let roots = spec_roots()?;
        let md5 = kuna_base::hex_lower(&Md5::digest(&bytes));
        let program = Bootstrap::new(&roots)
            .indexed_image(bytes, macho_slice.map(|slice| (slice.offset, slice.size)))
            .map_err(|err| format!("load original executable: {}", err.explain()))?;
        if let Some(saved) = saved_language {
            let loaded = kuna_decomp::kuna_langfamily::language_id(&program.arch().archid);
            if saved != loaded {
                return Err(format!(
                    "incompatible saved ProgramDB language {saved:?}: Kuna loaded {loaded:?}",
                ));
            }
        }
        Self::prepare(program, requested_profile, md5)
    }

    fn prepare(
        mut program: ConsoleProgram,
        requested_profile: Option<LshProfile>,
        md5: String,
    ) -> Result<Self, String> {
        let architecture = kuna_decomp::kuna_langfamily::language_id(&program.arch().archid).to_owned();
        let compiler = program.arch().compiler_spec_id()
            .ok_or("binary has no resolved compiler spec")?.to_owned();
        let profile = requested_profile.or_else(|| LshProfile::for_language(&architecture))
            .ok_or_else(|| format!("no BSim weights profile supports {architecture}"))?;
        if !profile.admits(&architecture) {
            return Err(format!("language {architecture} is incompatible with BSim profile {}", profile.name()));
        }
        let settings = profile.bundled_weights().map_err(|err| err.explain().to_owned())?.settings as u32;

        let id = program.registry().find_element("errortoomanyinstructions", 0);
        OptionDatabase::new().set(program.arch_mut(), id, "off", "", "")
            .map_err(|err| err.explain().to_owned())?;
        let gates: &[&str] = if architecture.starts_with("x86:LE:64:") {
            &["listing"]
        } else {
            &["listing", "funcstart_patterns", "aif"]
        };
        for gate in gates {
            program.arch_mut().set_kuna_option(gate, "on")
                .map_err(|err| err.explain().to_owned())?;
        }
        program.commit_pending_analysis().map_err(|err| err.explain().to_owned())?;
        kuna_console::kuna_callwide::prepare(&mut program);
        Ok(Self { program, profile, settings, architecture, compiler, md5 })
    }

    fn sign_entry(&mut self, entry: FunctionEntry) -> Result<BsimSignature, String> {
        let offset = entry.addr.get_offset();
        let extent = self.program.declared_extent(offset);
        let address = self.program.output_code_offset(offset);
        if entry.name.is_empty() || entry.name.len() > MAX_NAME_BYTES {
            return Err(format!("entry at {address:#x} has an empty or oversized name"));
        }
        let result = kuna_decomp::decompile_drive::normalize_function(
            self.program.arch_mut(), &entry.name, entry.addr, extent,
        ).map_err(|err| err.explain().to_owned())
            .and_then(|fd| sign_function_with_settings(&fd, &entry.name, address, self.settings)
                .map_err(|err| err.describe()));
        let signature = result.map_err(|err| format!("{} at {address:#x}: {err}", entry.name))?;
        if signature.features.is_empty() || signature.features.len() > MAX_QUERY_FEATURES {
            return Err(format!("entry at {address:#x} has an empty or oversized feature vector"));
        }
        Ok(signature)
    }

    fn sign_entries(
        &mut self,
        entries: Vec<FunctionEntry>,
    ) -> Result<(Vec<BsimSignature>, SigningCoverage), String> {
        let mut coverage = SigningCoverage { considered: entries.len(), first_failure: None };
        let mut signed = Vec::new();
        for entry in entries {
            match self.sign_entry(entry) {
                Ok(signature) => signed.push(signature),
                Err(err) => { coverage.first_failure.get_or_insert(err); }
            }
        }
        if signed.is_empty() {
            return Err(coverage.first_failure.unwrap_or_else(|| "original image has no signable functions".into()));
        }
        Ok((signed, coverage))
    }

    fn finish(
        self,
        name: String,
        functions: Vec<BsimSignature>,
        complete: bool,
    ) -> (LshProfile, CorpusExecutable) {
        (self.profile, CorpusExecutable {
            name,
            architecture: self.architecture,
            compiler: self.compiler,
            md5: self.md5,
            complete,
            functions,
        })
    }
}

fn in_executable_section(offset: u64, sections: &[(u64, u64, u32)]) -> bool {
    sections.iter().any(|&(start, size, flags)| {
        flags & section_flags::CODE != 0 && offset >= start && offset - start < size
    })
}

fn spec_roots() -> Result<Vec<SpecRoot>, String> {
    let (paths, embedded) = spec_locations()?;
    let mut roots = Vec::with_capacity(paths.len() + usize::from(embedded));
    roots.extend(paths.into_iter().map(SpecRoot::Dir));
    if embedded {
        roots.push(bundled_spec_root().expect("selected bundled specs are present"));
    }
    Ok(roots)
}

/// Resolve disk roots before the worker changes directories, without mounting
/// the embedded table in the HTTP process.
pub(super) fn spec_locations() -> Result<(Vec<PathBuf>, bool), String> {
    let env = |name| std::env::var_os(name).filter(|path| !path.is_empty()).map(PathBuf::from);
    let mut roots: Vec<PathBuf> = env("SLEIGHHOME").into_iter().collect();
    let explicit = env("KUNA_SPECS").or_else(|| env("KUNA_ROOT").map(|p| p.join("specs")));
    let embedded = explicit.is_none() && kuna_console::embedded_specs::files().is_some();
    if let Some(root) = explicit {
        roots.push(root);
    } else if !embedded {
        if let Ok(cwd) = std::env::current_dir() {
            roots.extend(cwd.ancestors().map(|dir| dir.join("specs")).filter(|p| p.join("Ghidra/Processors").is_dir()));
        }
    }
    if roots.is_empty() && !embedded {
        return Err("SLEIGH specs unavailable: use a bundled release or set KUNA_SPECS".into());
    }
    Ok((roots, embedded))
}

pub(super) fn bundled_spec_root() -> Option<SpecRoot> {
    use std::sync::LazyLock;

    static SPECS: LazyLock<Option<MemorySpecs>> =
        LazyLock::new(|| kuna_console::embedded_specs::files().map(MemorySpecs::new));
    SPECS.as_ref().cloned().map(SpecRoot::Memory)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn project_preserves_signed_entries_and_reports_incompatible_ones() {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/fid/callees/prog");
        let binary = fixture.to_str().unwrap();
        let entries = || vec![
            ProgramFunction { location: ProgramEntry::LinkedAddress(0x401000), name: "saved_scale".to_owned() },
            ProgramFunction { location: ProgramEntry::LinkedAddress(0), name: "outside_image".to_owned() },
            ProgramFunction { location: ProgramEntry::LinkedAddress(0x401020), name: "x".repeat(MAX_NAME_BYTES + 1) },
        ];
        let (_, executable, coverage) = sign_import(
            binary, "x86:LE:64:default", entries(), None, "project".to_owned(),
        ).unwrap();
        assert_eq!(executable.functions[0].name, "saved_scale");
        assert_eq!(executable.functions.len(), 1);
        assert_eq!(coverage.considered, 3);
        let failure = coverage.first_failure.as_deref().expect("skipped entries must be reported");
        assert!(failure.starts_with("saved entry LinkedAddress(0)"), "{failure}");
        assert!(!executable.complete);

        for saved in ["AARCH64:LE:64:v8A", "x86:LE:64:compat32"] {
            let mismatch = sign_import(binary, saved, entries(), None, "project".to_owned());
            assert!(matches!(mismatch, Err(error) if error.contains("incompatible saved ProgramDB language")));
        }
    }
}
