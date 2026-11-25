use super::super::tests::{code, tiny, tiny_bytes};
use super::*;

const OLD: Profile = Profile {
    version: (1, 88, 0),
    optimization: 3,
};
const NEW: Profile = Profile {
    version: (1, 89, 0),
    optimization: 3,
};
const SYMBOL: &str = "_ZN4core3ptr4read17h0123456789abcdefE";

fn signature(profile: Profile, bank: Bank, symbol: &str) -> ProfileSignature {
    let mut signature = tiny();
    signature.modules[0].publics[0].name = symbol.as_bytes().to_vec();
    ProfileSignature {
        source: Source {
            profile,
            bank,
            identity: format!("authored-{profile:?}-{bank:?}"),
        },
        signature,
    }
}

fn pair(profile: Profile, symbol: &str) -> Vec<ProfileSignature> {
    [Bank::Inline, Bank::NoInline]
        .into_iter()
        .map(|bank| signature(profile, bank, symbol))
        .collect()
}

fn run(bytes: &[u8], profiles: &[Profile], signatures: &[ProfileSignature]) -> Report {
    let image = Image {
        spans: vec![(0x1000, bytes)],
        x86_fields: true,
        pads: super::super::X86_PADS,
    };
    let listing = Listing::from_insns_for_test(Vec::new(), false);
    let known = HashMap::from([(0x1000, SYMBOL.into())]);
    score(
        &image,
        &listing,
        &[0x1000, 0x1000],
        &known,
        profiles,
        signatures,
    )
    .unwrap()
}

#[test]
fn scores_existing_symbols_and_retains_each_banks_provenance() {
    let report = run(&code(), &[OLD], &pair(OLD, SYMBOL));
    assert_eq!(report.selection, Selection::Unique(OLD));
    let candidate = &report.candidates[0];
    assert_eq!(candidate.score, Some(1));
    assert_eq!(candidate.banks.len(), 2);
    assert_ne!(
        candidate.banks[0].source.identity,
        candidate.banks[1].source.identity
    );
    for bank in &candidate.banks {
        assert_eq!(bank.signature_library, "testlib");
        assert_eq!(bank.matches.len(), 1);
        assert_eq!(bank.matches[0].address, 0x1000);
        assert_eq!(bank.matches[0].symbol, SYMBOL);
        assert_eq!(bank.matches[0].demangled, "core::ptr::read");
        assert_eq!(bank.matches[0].library, Library::Core);
    }
}

#[test]
fn ties_missing_banks_and_zero_matches_never_choose_a_profile() {
    let signatures: Vec<_> = pair(OLD, SYMBOL)
        .into_iter()
        .chain(pair(NEW, SYMBOL))
        .collect();
    let tied = run(&code(), &[NEW, OLD, OLD], &signatures);
    assert_eq!(tied.selection, Selection::Ambiguous(vec![OLD, NEW]));
    assert_eq!(
        run(&code(), &[], &signatures).selection,
        Selection::NoCandidates
    );
    assert_eq!(
        run(&code(), &[NEW, OLD], &signatures[..2]).selection,
        Selection::Incomplete(vec![NEW])
    );
    let missing = run(&code(), &[OLD], &signatures[..1]);
    assert_eq!(missing.candidates[0].missing_banks, vec![Bank::NoInline]);
    assert_eq!(missing.candidates[0].score, None);
    let mut bad = code();
    bad[33] ^= 1;
    assert_eq!(
        run(&bad, &[OLD, NEW], &signatures).selection,
        Selection::NoMatches
    );
}

#[test]
fn an_unrequested_bank_cannot_change_a_measured_winner() {
    let mut signatures = pair(OLD, SYMBOL);
    let mut newer = pair(NEW, SYMBOL);
    for signature in &mut newer {
        signature.signature.modules[0].crc ^= 1;
    }
    signatures.extend(newer);
    assert_eq!(
        run(&code(), &[OLD, NEW], &signatures).selection,
        Selection::Unique(OLD)
    );
    assert_eq!(
        run(&code(), &[NEW], &signatures).selection,
        Selection::NoMatches
    );
}

#[test]
fn ambiguous_module_names_and_unresolved_references_still_refuse() {
    let mut signatures = pair(OLD, SYMBOL);
    for signature in &mut signatures {
        let mut rival = signature.signature.modules[0].clone();
        rival.publics[0].name = b"std::other".to_vec();
        signature.signature.modules.push(rival);
    }
    assert_eq!(
        run(&code(), &[OLD], &signatures).selection,
        Selection::NoMatches
    );
    let mut signatures = pair(OLD, SYMBOL);
    for signature in &mut signatures {
        signature.signature.modules[0].reference = Some(flirt_sig::Reference {
            offset: 4,
            name: b"unresolved".to_vec(),
        });
    }
    assert_eq!(
        run(&code(), &[OLD], &signatures).selection,
        Selection::NoMatches
    );
}

#[test]
fn rust_library_filter_preserves_full_names_and_rejects_user_crates() {
    for (symbol, library) in [
        ("core::ptr::read", Library::Core),
        ("std::io::write", Library::Std),
        ("<&alloc::vec::Vec<u8>>::grow", Library::Alloc),
    ] {
        assert_eq!(
            library_match(0x1000, symbol.into()).unwrap().library,
            library
        );
    }
    for symbol in ["mycrate::core::read", "_Runtime", "malloc", "stdish::run"] {
        assert!(library_match(0x1000, symbol.into()).is_none());
    }
    assert_eq!(
        run(&code(), &[OLD], &pair(OLD, "user::function")).selection,
        Selection::NoMatches
    );
}

#[test]
fn duplicate_banks_are_rejected_before_scoring() {
    let bytes = code();
    let image = Image {
        spans: vec![(0x1000, &bytes)],
        x86_fields: true,
        pads: &[],
    };
    let listing = Listing::from_insns_for_test(Vec::new(), false);
    let signatures = [
        signature(OLD, Bank::Inline, SYMBOL),
        signature(OLD, Bank::Inline, SYMBOL),
    ];
    assert_eq!(
        score(
            &image,
            &listing,
            &[0x1000],
            &HashMap::new(),
            &[OLD],
            &signatures
        ),
        Err(ReportError::DuplicateBank(OLD, Bank::Inline))
    );
}

#[test]
fn signature_reader_rejects_malformed_foreign_and_startup_inputs() {
    let source = || Source {
        profile: OLD,
        bank: Bank::Inline,
        identity: "authored".into(),
    };
    let parsed = ProfileSignature::parse(source(), &tiny_bytes()).unwrap();
    assert_eq!(parsed.source, source());
    assert_eq!(parsed.signature, tiny());
    assert!(matches!(
        ProfileSignature::parse(source(), b"short"),
        Err(ReportError::InapplicableSignature(_))
    ));
    let mut bytes = vec![0; 41];
    bytes[..6].copy_from_slice(b"IDASGN");
    assert!(matches!(
        ProfileSignature::parse(source(), &bytes),
        Err(ReportError::MalformedSignature(_))
    ));
    bytes[7] = 12;
    assert!(matches!(
        ProfileSignature::parse(source(), &bytes),
        Err(ReportError::InapplicableSignature(_))
    ));
    bytes[7] = 0;
    bytes[8..12].copy_from_slice(&(1u32 << 11).to_le_bytes());
    assert!(matches!(
        ProfileSignature::parse(source(), &bytes),
        Err(ReportError::InapplicableSignature(_))
    ));
    bytes[8..12].fill(0);
    bytes[16] = 1;
    assert!(matches!(
        ProfileSignature::parse(source(), &bytes),
        Err(ReportError::InapplicableSignature(_))
    ));
}

#[test]
fn public_report_requires_a_linked_little_endian_x86_64_elf() {
    let listing = Listing::from_insns_for_test(Vec::new(), false);
    for (architecture, endian, linked, accepted) in [
        (
            object::Architecture::X86_64,
            object::Endianness::Little,
            true,
            true,
        ),
        (
            object::Architecture::X86_64,
            object::Endianness::Little,
            false,
            false,
        ),
        (
            object::Architecture::Aarch64,
            object::Endianness::Little,
            true,
            false,
        ),
        (
            object::Architecture::I386,
            object::Endianness::Little,
            true,
            false,
        ),
        (
            object::Architecture::X86_64,
            object::Endianness::Big,
            true,
            false,
        ),
    ] {
        let object = object::write::Object::new(object::BinaryFormat::Elf, architecture, endian);
        let mut bytes = object.write().unwrap();
        if linked {
            let kind = if endian == object::Endianness::Little {
                3u16.to_le_bytes()
            } else {
                3u16.to_be_bytes()
            };
            bytes[16..18].copy_from_slice(&kind);
        }
        let file = object::File::parse(&*bytes).unwrap();
        let result = report(&file, &listing, &[], &HashMap::new(), &[], &[]);
        if accepted {
            assert_eq!(result.unwrap().selection, Selection::NoCandidates);
        } else {
            assert_eq!(result, Err(ReportError::UnsupportedImage));
        }
    }
}
