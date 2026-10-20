//! Freeze caller-return evidence from a complete object, before target selection.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use object::{Object, ObjectSection, ObjectSegment, ObjectSymbol};

use kuna_decomp::fspec::{ProtoModel, PrototypePieces};
use kuna_decomp::kuna_callwide::{CodeRegion, Declaration, DeclarationSource};

use crate::engine::ConsoleProgram;

const MAX_OBJECT_BYTES: usize = 8 * 1024 * 1024;
const MAX_CODE_BYTES: usize = 65_536;
const MAX_ENTRIES: usize = 16_384;
const MAX_NAMES: usize = 65_536;
const MAX_NAME_BYTES: usize = 1_048_576;

pub struct InventoryEntry {
    pub address: u64,
    pub size: u64,
    pub name: String,
    pub aliases: Vec<String>,
}

#[derive(Default)]
pub(crate) struct State {
    image: Option<Vec<u8>>,
    declarations: Vec<Record>,
    inventory: Option<Vec<InventoryEntry>>,
    frozen: bool,
    unavailable: Option<&'static str>,
    declaration_names: usize,
    declaration_bytes: usize,
}

enum Target {
    Entry(u64),
    Named(String),
}

struct Record {
    target: Target,
    pieces: PrototypePieces,
    model: Rc<ProtoModel>,
    source: DeclarationSource,
}

impl State {
    pub(crate) fn supported(&self) -> bool {
        self.image.is_some()
    }

    pub(crate) fn object(bytes: &[u8]) -> Self {
        let eligible = bytes.len() <= MAX_OBJECT_BYTES
            && kuna_analysis::loadimage_object::parse_object(bytes).is_ok_and(|file| {
                file.format() == object::BinaryFormat::Elf
                    && !file.is_64()
                    && !file.is_little_endian()
                    && file.kind() == object::ObjectKind::Executable
                    && file.architecture() == object::Architecture::Mips
            });
        Self {
            image: eligible.then(|| bytes.to_vec()),
            ..Self::default()
        }
    }

    pub(crate) fn changed(&mut self) {
        self.frozen = false;
    }

    pub(crate) fn unavailable(&mut self, reason: &'static str) {
        self.frozen = false;
        self.unavailable = Some(reason);
    }
}

/// External declarations are recorded at ingestion; recovered prototype locks do not enter here.
pub(crate) fn declaration(
    prog: &mut ConsoleProgram,
    entry: u64,
    pieces: &PrototypePieces,
    model: Option<Rc<ProtoModel>>,
    source: DeclarationSource,
) {
    record(prog, Target::Entry(entry), pieces, model, source);
}

fn record(
    prog: &mut ConsoleProgram,
    target: Target,
    pieces: &PrototypePieces,
    model: Option<Rc<ProtoModel>>,
    source: DeclarationSource,
) {
    if !prog.callwide.supported() {
        return;
    }
    let (name_count, name_bytes) = record_cost(&target, pieces);
    if pieces.intypes.len() > 256
        || pieces.input_storage.len() > 256
        || prog.callwide.declaration_names.saturating_add(name_count) > MAX_NAMES
        || prog.callwide.declaration_bytes.saturating_add(name_bytes) > MAX_NAME_BYTES
    {
        prog.callwide_unavailable("declaration provenance exceeds the image bound");
        return;
    }
    let model = model
        .or_else(|| kuna_decomp::kuna_typedcallabi::undeclared_model_for(prog.arch(), pieces))
        .or_else(|| prog.arch().default_fp().cloned());
    prog.callwide_changed();
    if let Some(model) = model {
        if prog.callwide.declarations.len() >= MAX_ENTRIES {
            prog.callwide_unavailable("declaration provenance exceeds the image bound");
            return;
        }
        prog.callwide.declaration_names += name_count;
        prog.callwide.declaration_bytes += name_bytes;
        prog.callwide.declarations.push(Record {
            target,
            pieces: pieces.clone(),
            model,
            source,
        });
    } else {
        prog.callwide_unavailable("declaration has no established ABI model");
    }
}

fn record_cost(target: &Target, pieces: &PrototypePieces) -> (usize, usize) {
    let name_count = pieces.innames.len().saturating_add(2);
    let name_bytes = pieces
        .name
        .len()
        .saturating_add(match target {
            Target::Entry(_) => 0,
            Target::Named(name) => name.len(),
        })
        .saturating_add(pieces.innames.iter().map(String::len).sum::<usize>());
    (name_count, name_bytes)
}

pub(crate) fn named_declaration(
    prog: &mut ConsoleProgram,
    name: &str,
    pieces: &PrototypePieces,
    model: Option<Rc<ProtoModel>>,
    source: DeclarationSource,
) {
    record(prog, Target::Named(name.to_owned()), pieces, model, source);
}

fn declarations(prog: &ConsoleProgram) -> Option<Vec<Declaration>> {
    resolve_declarations(&prog.callwide.declarations, declaration_inventory(prog))
}

fn declaration_inventory(prog: &ConsoleProgram) -> Vec<(u64, String, Vec<String>)> {
    match &prog.callwide.inventory {
        Some(inventory) => inventory
            .iter()
            .map(|entry| (entry.address, entry.name.clone(), entry.aliases.clone()))
            .collect(),
        None => prog
            .function_entries_canonical()
            .into_iter()
            .map(|entry| (entry.addr.get_offset(), entry.name, entry.aliases))
            .collect(),
    }
}

fn resolve_declarations(
    records: &[Record],
    entries: impl IntoIterator<Item = (u64, String, Vec<String>)>,
) -> Option<Vec<Declaration>> {
    let mut names = BTreeMap::new();
    let mut addresses = BTreeSet::new();
    let mut count = 0usize;
    let mut bytes = 0usize;
    for (address, name, aliases) in entries {
        if addresses.len() >= MAX_ENTRIES {
            return None;
        }
        addresses.insert(address);
        for name in std::iter::once(name).chain(aliases) {
            count = count.checked_add(1)?;
            bytes = bytes.checked_add(name.len())?;
            if count > MAX_NAMES || bytes > MAX_NAME_BYTES {
                return None;
            }
            names
                .entry(name)
                .and_modify(|known| {
                    if *known != Some(address) {
                        *known = None;
                    }
                })
                .or_insert(Some(address));
        }
    }
    let mut resolved = BTreeMap::new();
    for record in records
        .iter()
        .filter(|record| record.source == DeclarationSource::Dwarf)
        .chain(
            records
                .iter()
                .filter(|record| record.source != DeclarationSource::Dwarf),
        )
    {
        let entry = match &record.target {
            Target::Entry(entry) if addresses.contains(entry) => *entry,
            Target::Named(name) => names.get(name).copied().flatten()?,
            _ => return None,
        };
        resolved.insert(
            entry,
            Declaration {
                entry,
                pieces: record.pieces.clone(),
                model: record.model.clone(),
                source: record.source,
            },
        );
    }
    Some(resolved.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program() -> ConsoleProgram {
        image_program("mips32_be_declared.elf")
    }

    fn image_program(name: &str) -> ConsoleProgram {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = root.join("../kuna-cli/tests/fixtures/callwide").join(name);
        let specs = root.join("../../..").join("specs");
        crate::engine::bootstrap_from_image_with_isa(
            fixture.to_str().unwrap(),
            "",
            &[specs.to_str().unwrap().to_owned()],
            None,
        )
        .unwrap()
    }

    #[test]
    fn refused_producer_does_not_publish_other_strict_dwarf_contracts() {
        use crate::engine::EntrySelector;
        let mut prog = image_program("mips32_be_dwarf.elf");
        prog.arch_mut().call_wide = true;
        prog.arch_mut().analysis_dwarf = true;
        prog.arch_mut().narrow_ext = kuna_decomp::kuna_narrowext::NarrowExtMode::Compiler;
        prog.commit_pending_analysis().unwrap();
        let producer = prog
            .resolve_entry(&EntrySelector::Name("produce_sum".into()))
            .unwrap();
        let caller = prog
            .resolve_entry(&EntrySelector::Name("caller64".into()))
            .unwrap();
        assert!(prog
            .arch()
            .symboltab
            .function_proto_pieces_across_scopes(&caller.addr)
            .is_none());
        let bytes = prog.callwide.image.as_ref().unwrap();
        assert!(facts(&prog, bytes).is_some());
        let strict = kuna_analysis::dwarf::callwide_declarations(
            &kuna_analysis::loadimage_object::parse_object(bytes).unwrap(),
            prog.arch(),
            &declaration_inventory(&prog),
        )
        .unwrap();
        assert!(strict
            .iter()
            .any(|(entry, _)| *entry == caller.addr.get_offset()));
        let narrow = prog
            .arch()
            .types()
            .get_base(4, kuna_decomp::dtype::type_metatype::TYPE_UINT)
            .unwrap();
        declaration(
            &mut prog,
            producer.addr.get_offset(),
            &PrototypePieces {
                name: "produce_sum".into(),
                outtype: Some(narrow),
                first_var_arg_slot: -1,
                ..Default::default()
            },
            None,
            DeclarationSource::Assertion,
        );
        prepare(&mut prog);
        assert!(prog.arch().kuna_callwide.is_empty());
        assert!(prog
            .arch()
            .symboltab
            .function_proto_pieces_across_scopes(&caller.addr)
            .is_none());
        assert!(prog.arch().function_prototype_model("caller64").is_none());
    }

    #[test]
    fn positive_proof_publishes_only_its_used_strict_contracts() {
        use crate::engine::EntrySelector;
        let mut prog = image_program("mips32_be_dwarf.elf");
        prog.arch_mut().call_wide = true;
        prog.arch_mut().analysis_dwarf = true;
        prog.arch_mut().narrow_ext = kuna_decomp::kuna_narrowext::NarrowExtMode::Compiler;
        prog.commit_pending_analysis().unwrap();
        let caller = prog
            .resolve_entry(&EntrySelector::Name("caller64".into()))
            .unwrap();
        let ignored = prog
            .resolve_entry(&EntrySelector::Name("caller_ignored".into()))
            .unwrap();
        let unused = prog
            .resolve_entry(&EntrySelector::Name("consume_two32".into()))
            .unwrap();
        assert!(prog
            .arch()
            .symboltab
            .function_proto_pieces_across_scopes(&caller.addr)
            .is_none());
        assert!(prog
            .arch()
            .function_prototype_model("consume_two32")
            .is_none());
        prepare(&mut prog);
        assert!(!prog.arch().kuna_callwide.is_empty());
        assert!(prog
            .arch()
            .kuna_callwide
            .needed_declaration(caller.addr.get_offset()));
        assert!(prog
            .arch()
            .kuna_callwide
            .needed_declaration(ignored.addr.get_offset()));
        assert!(!prog
            .arch()
            .kuna_callwide
            .needed_declaration(unused.addr.get_offset()));
        let ignored_contract = prog
            .arch()
            .symboltab
            .function_proto_pieces_across_scopes(&ignored.addr)
            .unwrap();
        assert_eq!(
            ignored_contract
                .intypes
                .iter()
                .map(|ty| ty.get_size())
                .collect::<Vec<_>>(),
            [8, 8]
        );
        assert_eq!(ignored_contract.outtype.as_ref().unwrap().get_size(), 4);
        assert_eq!(
            prog.arch()
                .symboltab
                .function_proto_pieces_across_scopes(&caller.addr)
                .unwrap()
                .outtype
                .as_ref()
                .unwrap()
                .get_size(),
            8
        );
        assert!(prog
            .arch()
            .function_prototype_model("consume_two32")
            .is_none());
    }

    #[test]
    fn missing_narrow_extension_policy_declines_without_contract_publication() {
        use crate::engine::EntrySelector;
        use kuna_decomp::kuna_narrowext::NarrowExtMode;
        for mode in [NarrowExtMode::Abi, NarrowExtMode::Off] {
            let mut prog = image_program("mips32_be_dwarf.elf");
            prog.arch_mut().call_wide = true;
            prog.arch_mut().analysis_dwarf = true;
            prog.arch_mut().narrow_ext = mode;
            prog.commit_pending_analysis().unwrap();
            let caller = prog
                .resolve_entry(&EntrySelector::Name("caller64".into()))
                .unwrap();
            assert!(prog
                .arch()
                .symboltab
                .function_proto_pieces_across_scopes(&caller.addr)
                .is_none());
            let bytes = prog.callwide.image.as_ref().unwrap();
            assert!(facts(&prog, bytes).is_some());
            prepare(&mut prog);
            assert!(prog.arch().kuna_callwide.is_empty(), "{}", mode.as_str());
            assert!(prog
                .arch()
                .symboltab
                .function_proto_pieces_across_scopes(&caller.addr)
                .is_none());
            assert!(prog.arch().function_prototype_model("caller64").is_none());
        }
    }

    #[test]
    fn external_option_model_mode_and_kassert_commands_reopen_preflight() {
        use crate::ifacedecomp::{register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
        use crate::ifaceterm::ConsoleCommands;
        let commands = [
            "option callwide off",
            "option callwide on",
            "option protoeval __stdcall",
            "option bejoin off",
            "option bejoin evidence",
            "mode reliable",
            "option callwide on",
            "option bejoin evidence",
            "kassert P3 comparison-canonicalization original",
        ];
        let mut status = ConsoleCommands::into_status(
            commands
                .iter()
                .map(|command| (*command).to_owned())
                .collect(),
        );
        register_decomp_commands(&mut status);
        crate::kuna_console::register_kuna_commands(&mut status);
        let mut prog = program();
        prog.arch_mut().call_wide = true;
        prog.arch_mut().analysis_dwarf = false;
        status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap()
            .conf = Some(prog);
        for command in commands {
            let data = status
                .get_data_mut(DECOMPILE_MODULE)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<IfaceDecompData>()
                .unwrap();
            let prog = data.conf.as_mut().unwrap();
            if prog.arch().call_wide
                && prog.arch().be_join == kuna_decomp::kuna_bejoin::BeJoinMode::Evidence
            {
                prepare(prog);
                assert!(prog.callwide.frozen, "completed preflight before {command}");
            }
            status
                .run_command()
                .unwrap_or_else(|error| panic!("{command}: {error}"));
            let prog = status
                .get_data_mut(DECOMPILE_MODULE)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<IfaceDecompData>()
                .unwrap()
                .conf
                .as_ref()
                .unwrap();
            assert!(
                !prog.callwide.frozen,
                "external command left preflight frozen: {command}"
            );
        }
    }

    #[test]
    fn partial_storage_cannot_publish_an_unproven_recovered_sibling_type() {
        use crate::assertions::{Body, Directive};
        let mut prog = program();
        let wide = prog
            .arch()
            .types()
            .get_base(8, kuna_decomp::dtype::type_metatype::TYPE_UINT)
            .unwrap();
        let pieces = PrototypePieces {
            name: "consume64".into(),
            outtype: Some(wide.clone()),
            intypes: vec![wide],
            innames: vec!["recovered".into()],
            first_var_arg_slot: -1,
            ..Default::default()
        };
        prog.arch_mut()
            .set_function_prototype_pieces("consume64", pieces.clone());
        prog.set_pending_prototype("consume64", pieces);
        assert!(prog.callwide.declarations.is_empty());
        prog.set_assertions(vec![Directive {
            raw: "param consume64::0 %a0 unsigned int supplied".into(),
            body: Body::Param {
                func: Some("consume64".into()),
                index: 0,
                storage: "%a0".into(),
                decl: "unsigned int supplied".into(),
            },
        }]);
        crate::assertions::apply_program_scoped(&mut prog);
        assert_eq!(
            prog.assertion_outcomes()[0].status,
            "applied",
            "{:?}",
            prog.assertion_outcomes()[0].detail
        );
        assert_eq!(
            prog.pending_prototype("consume64")
                .unwrap()
                .outtype
                .as_ref()
                .unwrap()
                .get_size(),
            8
        );
        assert!(prog.callwide.declarations.is_empty());
        assert!(prog.callwide.unavailable.is_some());
        assert!(prog.arch().kuna_callwide.is_empty());
    }

    fn record(target: Target, name: &str, source: DeclarationSource) -> Record {
        Record {
            target,
            pieces: PrototypePieces {
                name: name.to_owned(),
                ..Default::default()
            },
            model: Rc::new(ProtoModel::new(&kuna_base::space::AddrSpaceManager::new())),
            source,
        }
    }

    #[test]
    fn arm_image_cannot_supply_sealed_mips_frontend_authority() {
        let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../kuna-cli/tests/fixtures/callwide/arm32_be_declared.elf");
        let bytes = std::fs::read(fixture).unwrap();
        assert!(!State::object(&bytes).supported());
    }

    #[test]
    fn mips_symbol_mode_markers_refuse_legacy_image_authority() {
        let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../kuna-cli/tests/fixtures/callwide/mips32_be_declared.elf");
        let original = std::fs::read(fixture).unwrap();
        let file = kuna_analysis::loadimage_object::parse_object(&original).unwrap();
        assert!(mips_container(&original, &file));
        let word = |at| u32::from_be_bytes(original[at..at + 4].try_into().unwrap()) as usize;
        let shoff = word(32);
        let shnum = u16::from_be_bytes(original[48..50].try_into().unwrap()) as usize;
        let mut function = None;
        for index in 0..shnum {
            let at = shoff + index * 40;
            if word(at + 4) != 2 {
                continue;
            }
            for symbol in (word(at + 16)..word(at + 16) + word(at + 20)).step_by(16) {
                if original[symbol + 12] & 15 == 2 {
                    function = Some(symbol);
                    break;
                }
            }
        }
        let function = function.unwrap();
        for mode in [0xf0, 0x80, 0x04] {
            let mut bytes = original.clone();
            bytes[function + 13] = (bytes[function + 13] & 3) | mode;
            let file = kuna_analysis::loadimage_object::parse_object(&bytes).unwrap();
            assert!(!mips_container(&bytes, &file), "st_other={mode:#x}");
        }
        let mut bytes = original;
        bytes[function + 7] |= 1;
        let file = kuna_analysis::loadimage_object::parse_object(&bytes).unwrap();
        assert!(!mips_container(&bytes, &file), "odd function address");
    }

    #[test]
    fn normalized_mips_language_keeps_only_the_legacy_default_compiler() {
        use kuna_decomp::sleigh_arch::normalize_architecture;
        let normalized = normalize_architecture("MIPS:BE:32:default").unwrap();
        assert!(supported_language(&normalized));
        for id in [
            "MIPS:BE:32:R6:default",
            "MIPS:BE:32:micro:default",
            "MIPS:BE:32:default:eabi",
            "MIPS:LE:32:default:default",
            "ARM:BE:32:v7:default",
        ] {
            assert!(!supported_language(id));
        }
    }

    #[test]
    fn named_origin_waits_for_complete_inventory_and_resolves_unique_alias() {
        let records = [record(
            Target::Named("sink".into()),
            "decl",
            DeclarationSource::Dwarf,
        )];
        assert!(resolve_declarations(&records, []).is_none());
        let resolved =
            resolve_declarations(&records, [(0x1000, "renamed".into(), vec!["sink".into()])])
                .unwrap();
        assert_eq!(resolved[0].entry, 0x1000);
        assert_eq!(resolved[0].source, DeclarationSource::Dwarf);
        assert!(resolve_declarations(
            &records,
            [
                (0x1000, "sink".into(), vec![]),
                (0x2000, "sink".into(), vec![])
            ]
        )
        .is_none());
    }

    #[test]
    fn explicit_origin_wins_when_strict_dwarf_is_extracted_later() {
        let records = [
            record(
                Target::Entry(0x1000),
                "explicit",
                DeclarationSource::Assertion,
            ),
            record(
                Target::Named("sink".into()),
                "dwarf",
                DeclarationSource::Dwarf,
            ),
        ];
        let resolved = resolve_declarations(&records, [(0x1000, "sink".into(), vec![])]).unwrap();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].pieces.name, "explicit");
        assert_eq!(resolved[0].source, DeclarationSource::Assertion);
        assert!(resolve_declarations(&records, [(0x2000, "sink".into(), vec![])]).is_none());
    }
}

pub(crate) fn target_declaration(
    prog: &mut ConsoleProgram,
    target: &crate::assertions::ProtoTarget,
    pieces: &PrototypePieces,
    model: Option<Rc<ProtoModel>>,
    source: DeclarationSource,
) {
    match target {
        crate::assertions::ProtoTarget::At(address, _) => {
            if !address
                .get_space()
                .zip(prog.arch().manage().get_default_code_space())
                .is_some_and(|(actual, code)| Rc::ptr_eq(actual, code))
            {
                prog.callwide_unavailable(
                    "declaration target is outside the established code space",
                );
                return;
            }
            declaration(prog, address.get_offset(), pieces, model, source)
        }
        crate::assertions::ProtoTarget::Named(name) => {
            named_declaration(prog, name, pieces, model, source)
        }
    }
}

/// A refusal is stable until an external mutation; no selected function can supply a positive fact.
pub fn prepare(prog: &mut ConsoleProgram) {
    if !prog.arch().call_wide
        || prog.arch().be_join != kuna_decomp::kuna_bejoin::BeJoinMode::Evidence
        || prog.callwide.frozen
    {
        return;
    }
    prog.arch_mut().kuna_callwide.clear();
    ingest_dwarf(prog);
    prog.callwide.frozen = true;
    if let Some(reason) = prog.callwide.unavailable {
        refuse(prog, reason);
        return;
    }
    if !prog.callwide.supported() {
        refuse(prog, "complete object facts are unavailable");
        return;
    }
    if prog.callwide.declarations.is_empty() {
        refuse(prog, "no declaration provenance was supplied");
        return;
    }
    let Some(bytes) = prog.callwide.image.take() else {
        refuse(prog, "complete object facts are unavailable");
        return;
    };
    let facts = facts(prog, &bytes);
    prog.callwide.image = Some(bytes);
    match facts {
        Some(Facts {
            regions,
            functions,
            sites,
            entry,
            observed,
        }) => {
            let Some(declarations) = declarations(prog) else {
                refuse(
                    prog,
                    "declaration has no unique entry in the completed inventory",
                );
                return;
            };
            if !prog.arch().default_fp().is_some_and(|model| {
                prog.arch()
                    .eval_fp_current()
                    .is_some_and(|current| Rc::ptr_eq(model, current))
                    && declarations
                        .iter()
                        .all(|declaration| Rc::ptr_eq(model, &declaration.model))
            }) {
                refuse(
                    prog,
                    "declaration ABI differs from the current default model",
                );
                return;
            }
            let Some(space) = prog.arch().manage().get_default_code_space().cloned() else {
                return;
            };
            if kuna_decomp::kuna_callwide::publish_mips(
                prog.arch_mut(),
                &regions,
                &functions,
                entry,
                &observed,
                &sites,
                &declarations,
            ) == 0
            {
                return;
            }
            let Some(verdict) = prog.arch().kuna_callwide.verdict() else {
                return;
            };
            let needed: Vec<_> = declarations
                .iter()
                .filter(|declaration| {
                    prog.arch()
                        .kuna_callwide
                        .needed_declaration(declaration.entry)
                })
                .cloned()
                .collect();
            for declaration in &needed {
                let address = kuna_base::address::Address::new(space.clone(), declaration.entry);
                let name = prog
                    .arch()
                    .symboltab
                    .function_display_name_across_scopes(&address)
                    .unwrap_or_else(|| declaration.pieces.name.clone());
                let target = crate::assertions::ProtoTarget::At(address, name);
                crate::assertions::park_prototype(
                    prog,
                    &target,
                    &mut declaration.pieces.clone(),
                    Some(&declaration.model),
                );
            }
            if prog.arch().kuna_callwide.is_empty() {
                kuna_decomp::kuna_callwide::publish_mips(
                    prog.arch_mut(),
                    &regions,
                    &functions,
                    entry,
                    &observed,
                    &sites,
                    &declarations,
                );
            }
            if !prog.arch().kuna_callwide.same_verdict(&verdict) {
                prog.arch_mut().kuna_callwide.clear();
                refuse(prog, "declaration replay changed the frozen proof");
            }
        }
        None => refuse(
            prog,
            "complete code, ownership, caller, or ABI evidence was refused",
        ),
    }
}

fn ingest_dwarf(prog: &mut ConsoleProgram) {
    prog.callwide
        .declarations
        .retain(|record| record.source != DeclarationSource::Dwarf);
    let (names, bytes) =
        prog.callwide
            .declarations
            .iter()
            .fold((0usize, 0usize), |(names, bytes), record| {
                let (count, size) = record_cost(&record.target, &record.pieces);
                (names.saturating_add(count), bytes.saturating_add(size))
            });
    prog.callwide.declaration_names = names;
    prog.callwide.declaration_bytes = bytes;
    if !prog.arch().analysis_dwarf
        || !prog.callwide.supported()
        || prog.callwide.unavailable.is_some()
    {
        return;
    }
    let entries = declaration_inventory(prog);
    let declarations = prog
        .callwide
        .image
        .as_deref()
        .and_then(|bytes| kuna_analysis::loadimage_object::parse_object(bytes).ok())
        .and_then(|file| kuna_analysis::dwarf::callwide_declarations(&file, prog.arch(), &entries));
    match declarations {
        Some(declarations) => {
            for (entry, pieces) in declarations {
                declaration(prog, entry, &pieces, None, DeclarationSource::Dwarf);
            }
        }
        None => prog
            .callwide_unavailable("strict DWARF declaration evidence was incomplete or ambiguous"),
    }
}

fn refuse(prog: &ConsoleProgram, reason: &str) {
    kuna_base::notes::say_once(
        prog.description(),
        &format!("[kuna callwide] no promotion: {reason}"),
    );
}

struct Facts {
    regions: Vec<CodeRegion>,
    functions: Vec<(u64, u64)>,
    sites: BTreeMap<u64, Vec<u64>>,
    entry: u64,
    observed: BTreeSet<(u64, u64)>,
}

pub fn replay_inventory(prog: &mut ConsoleProgram, inventory: Vec<InventoryEntry>) {
    prog.callwide_changed();
    let count: usize = inventory
        .iter()
        .map(|entry| entry.aliases.len().saturating_add(1))
        .sum();
    let bytes: usize = inventory
        .iter()
        .flat_map(|entry| std::iter::once(&entry.name).chain(&entry.aliases))
        .map(String::len)
        .sum();
    if inventory.len() > MAX_ENTRIES || count > MAX_NAMES || bytes > MAX_NAME_BYTES {
        prog.callwide_unavailable("complete worker inventory exceeds the image bound");
        return;
    }
    prog.callwide.inventory = Some(inventory);
}

fn facts(prog: &ConsoleProgram, bytes: &[u8]) -> Option<Facts> {
    let file = kuna_analysis::loadimage_object::parse_object(bytes).ok()?;
    if file.architecture() != object::Architecture::Mips
        || !supported_language(&prog.arch().archid)
        || !mips_container(bytes, &file)
    {
        return None;
    }
    let mut regions = Vec::new();
    let mut total = 0usize;
    for segment in file.segments() {
        let object::SegmentFlags::Elf { p_flags } = segment.flags() else {
            return None;
        };
        if p_flags & 1 == 0 {
            continue;
        }
        let original = segment.data().ok()?;
        total = total.checked_add(original.len())?;
        if original.is_empty()
            || original.len() as u64 != segment.size()
            || total > MAX_CODE_BYTES
            || p_flags & 2 != 0
        {
            return None;
        }
        let effective = prog.read_bytes(segment.address(), original.len())?;
        if effective != original {
            return None;
        }
        regions.push(CodeRegion {
            address: segment.address(),
            bytes: effective,
        });
    }
    if regions.is_empty() {
        return None;
    }
    for section in file.sections() {
        if let object::SectionFlags::Elf { sh_flags } = section.flags() {
            if sh_flags & 2 != 0 && section.kind() != object::SectionKind::UninitializedData {
                let data = section.data().ok()?;
                if data.len() as u64 != section.size() {
                    return None;
                }
            }
        }
    }
    let entries = prog.function_entries_canonical();
    if entries.is_empty() || entries.len() > MAX_ENTRIES {
        return None;
    }
    if !prog.arch().error_noreturn_callsites.is_empty()
        || entries.iter().any(|entry| {
            let db = &prog.arch().symboltab;
            (db.function_is_no_return_across_scopes(&entry.addr)
                && entry.addr.get_offset() != file.entry())
                || db.function_is_inline_across_scopes(&entry.addr)
                || db.function_inject_id_across_scopes(&entry.addr) >= 0
        })
    {
        return None;
    }
    let functions: Vec<_> = match &prog.callwide.inventory {
        Some(inventory) => {
            let actual: BTreeSet<_> = entries
                .iter()
                .map(|entry| entry.addr.get_offset())
                .collect();
            if actual != inventory.iter().map(|entry| entry.address).collect() {
                return None;
            }
            inventory
                .iter()
                .map(|entry| (entry.address, entry.size))
                .collect()
        }
        None => entries
            .iter()
            .map(|entry| (entry.addr.get_offset(), entry.size))
            .collect(),
    };
    if functions.iter().any(|(entry, size)| {
        *size == 0
            || !regions.iter().any(|region| {
                region.address <= *entry
                    && entry
                        .checked_add(*size)
                        .zip(region.address.checked_add(region.bytes.len() as u64))
                        .is_some_and(|(end, last)| end <= last)
            })
    }) {
        return None;
    }
    let seeds: Vec<_> = functions.iter().map(|(entry, _)| *entry).collect();
    let wanted: BTreeSet<_> = seeds.iter().copied().collect();
    let mut stored_addresses = BTreeSet::new();
    for segment in file.segments() {
        for window in segment.data().ok()?.windows(4) {
            let value = u64::from(u32::from_be_bytes(window.try_into().ok()?));
            if wanted.contains(&value) {
                stored_addresses.insert(value);
            }
        }
    }
    let owners: BTreeMap<_, _> = functions.iter().copied().collect();
    if owners.len() != functions.len() {
        return None;
    }
    let mut last = None;
    for (&entry, &size) in &owners {
        if last.is_some_and(|end| entry < end) {
            return None;
        }
        last = Some(entry.checked_add(size)?);
    }
    let index =
        kuna_analysis::listing::xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds);
    let owner = |at: u64| {
        let (&entry, &size) = owners.range(..=at).next_back()?;
        (at < entry.checked_add(size)?).then_some(entry)
    };
    if seeds.iter().any(|entry| index.has_indirect_calls(*entry)) {
        return None;
    }
    let mut observed = BTreeSet::new();
    let mut open = stored_addresses;
    open.insert(file.entry());
    for &entry in &seeds {
        for r in index.refs_to(entry) {
            if r.kind == kuna_analysis::listing::xrefs::XrefKind::Call && owner(r.from).is_some() {
                observed.insert((r.from, entry));
            } else {
                open.insert(entry);
            }
        }
    }
    for symbol in file.dynamic_symbols() {
        if symbol.is_definition() {
            open.insert(symbol.address());
        }
    }
    for export in file.exports().ok()? {
        open.insert(export.address());
    }
    let mut sites = kuna_decomp::kuna_callwide::mips::closed_image(
        file.entry(),
        &regions,
        &functions,
        &observed,
    )?;
    sites.retain(|entry, _| !open.contains(entry));
    Some(Facts {
        regions,
        functions,
        sites,
        entry: file.entry(),
        observed,
    })
}

fn supported_language(archid: &str) -> bool {
    archid == "MIPS:BE:32:default:default"
}

fn mips_container(bytes: &[u8], file: &object::File) -> bool {
    let Some(header) = bytes.get(..52) else {
        return false;
    };
    let word = |at| u32::from_be_bytes(header[at..at + 4].try_into().unwrap());
    let half = |at| u16::from_be_bytes(header[at..at + 2].try_into().unwrap());
    let flags = word(36);
    if !matches!(flags & 0xf000_0000, 0x5000_0000 | 0x7000_0000)
        || !matches!(flags & 0x0000_f000, 0 | 0x1000)
        || flags & !0xf000_1001 != 0
        || half(42) != 32
    {
        return false;
    }
    let phoff = word(28) as usize;
    for index in 0..usize::from(half(44)) {
        let Some(at) = index
            .checked_mul(32)
            .and_then(|offset| phoff.checked_add(offset))
        else {
            return false;
        };
        let Some(phdr) = at.checked_add(32).and_then(|end| bytes.get(at..end)) else {
            return false;
        };
        let kind = u32::from_be_bytes(phdr[..4].try_into().unwrap());
        let flags = u32::from_be_bytes(phdr[24..28].try_into().unwrap());
        if matches!(kind, 2 | 3 | 7) || (kind == 0x6474_e551 && flags & 1 != 0) {
            return false;
        }
    }
    if half(46) != 40 || half(48) == 0 {
        return false;
    }
    let shoff = word(32) as usize;
    for index in 0..usize::from(half(48)) {
        let Some(at) = index
            .checked_mul(40)
            .and_then(|offset| shoff.checked_add(offset))
        else {
            return false;
        };
        let Some(shdr) = at.checked_add(40).and_then(|end| bytes.get(at..end)) else {
            return false;
        };
        if matches!(
            u32::from_be_bytes(shdr[4..8].try_into().unwrap()),
            4 | 9 | 14..=16 | 19
        ) {
            return false;
        }
    }
    if !file.imports().is_ok_and(|imports| imports.is_empty()) {
        return false;
    }
    if !file.exports().is_ok_and(|exports| exports.is_empty()) {
        return false;
    }
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if let object::SymbolFlags::Elf { st_info, st_other } = symbol.flags() {
            if st_info & 15 == 10
                || st_other & !3 != 0
                || (st_info & 15 == 2 && symbol.address() & 1 != 0)
            {
                return false;
            }
            if symbol.is_definition()
                && matches!(st_info >> 4, 1 | 2)
                && matches!(st_other & 3, 0 | 3)
                && !(st_info & 15 == 2 && symbol.address() == file.entry())
            {
                return false;
            }
        } else {
            return false;
        }
    }
    for section in file.sections() {
        if section.name().is_ok_and(|name| {
            [
                ".init",
                ".fini",
                ".preinit_array",
                ".init_array",
                ".fini_array",
                ".ctors",
                ".dtors",
            ]
            .iter()
            .any(|prefix| name == *prefix || name.starts_with(&format!("{prefix}.")))
        }) {
            return false;
        }
        if matches!(
            section.name(),
            Ok(".dynamic" | ".interp" | ".got" | ".got.plt")
        ) && section.size() != 0
        {
            return false;
        }
        if matches!(section.flags(), object::SectionFlags::Elf { sh_flags } if sh_flags & 0x1000_0000 != 0)
        {
            return false;
        }
        if section.relocations().next().is_some() {
            return false;
        }
    }
    true
}
