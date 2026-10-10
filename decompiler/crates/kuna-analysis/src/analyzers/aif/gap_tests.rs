use super::*;
use crate::listing::decode::decode_one;
use crate::listing::{DiscoveredFunction, Insn};
use kuna_base::{address::Address, error::KunaResult};
use kuna_sleigh::{globalcontext::ContextInternal, loadimage::LoadImage, sleigh::Sleigh};
use std::path::PathBuf;

const BASE: u64 = 0x1000;

struct Bytes(Vec<u8>);

impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-aif-gap"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = addr
                .get_offset()
                .checked_sub(BASE)
                .and_then(|off| self.0.get(off as usize + i))
                .copied()
                .unwrap_or(0);
        }
        Ok(())
    }
}

fn arm(bytes: Vec<u8>) -> (Sleigh, Rc<AddrSpace>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sla = std::fs::read(root.join("specs/Ghidra/Processors/ARM/data/languages/ARM7_le.sla"))
        .expect("AIF tests require built ARM specs");
    let mut engine = Sleigh::new(Box::new(Bytes(bytes)), Box::new(ContextInternal::new()));
    engine.initialize_from_sla(&sla).unwrap();
    engine.set_context_default("TMode", 0);
    engine.set_context_default("LRset", 0);
    let space = Rc::clone(engine.base().manager().get_space_by_name("ram").unwrap());
    (engine, space)
}

fn padding_fixture(size: usize, count: usize) -> (Sleigh, Rc<AddrSpace>, Listing) {
    let mut bytes = [0x00, 0xf0, 0x20, 0xe3].repeat(size / 4);
    let mut funcs = Vec::new();
    let mut insns = Vec::new();
    for i in 0..count {
        let offset = i * 16;
        bytes[offset..offset + 8].copy_from_slice(&[7, 0, 0xa0, 0xe3, 0x1e, 0xff, 0x2f, 0xe1]);
        funcs.push(DiscoveredFunction {
            entry: BASE + offset as u64,
            name: None,
            from_symbol: true,
            has_no_return: false,
            call_fixup: None,
        });
    }
    let (engine, space) = arm(bytes);
    for func in &funcs {
        for addr in [func.entry, func.entry + 4] {
            let decoded = decode_one(&engine, addr, &space, true, false).unwrap();
            let c = crate::listing::classify::classify(&decoded.ops, addr, decoded.len);
            insns.push(Insn {
                addr,
                len: decoded.len,
                fall_through: c.fall_through,
                flow: c.flow,
                flows: c.flows,
                mnemonic: decoded.mnemonic,
                operands: decoded.operands,
                pcode: None,
            });
        }
    }
    let listing =
        Listing::from_model_for_test(insns, funcs, Vec::new(), vec![(BASE, BASE + size as u64)]);
    (engine, space, listing)
}

#[test]
fn rejected_padding_does_not_retain_the_scanned_image() {
    for size in [4096, 65536] {
        let (engine, space, listing) = padding_fixture(size, 20);
        let mut decoder = GapDecoder::new(&engine, space, listing.exec_ranges());
        let entries = run_aif_with_decoder(&listing, &mut decoder, true, false, None);
        assert!(entries.is_empty(), "padding is not a function");
        assert!(
            decoder.decoded_addresses.len() > size / 8,
            "the gap scan must actually run"
        );
        assert!(
            decoder.peak_cache_len <= 8,
            "retained {} decodes for {size} bytes of rejected padding",
            decoder.peak_cache_len
        );
    }
}

#[test]
fn aif_activation_boundary_does_not_hide_the_gap_scan() {
    for count in [19, 20, 21] {
        let (engine, space, listing) = padding_fixture(4096, count);
        let mut decoder = GapDecoder::new(&engine, space, listing.exec_ranges());
        assert!(run_aif_with_decoder(&listing, &mut decoder, true, false, None).is_empty());
        assert_eq!(decoder.decoded_addresses.is_empty(), count < 20);
    }
}

fn legacy_walk(
    listing: &Listing,
    decoder: &mut GapDecoder,
    strict: bool,
    corroborate: bool,
) -> Vec<u64> {
    if listing.function_count() < MINIMUM_FUNCTION_COUNT || listing.num_instructions() == 0 {
        return Vec::new();
    }
    let hist = build_fingerprint_histogram(listing, decoder);
    if !hist.values().any(|&count| count >= FINGERPRINT_THRESHOLD) {
        return Vec::new();
    }
    let mut entries = BTreeSet::new();
    let mut claimed = BTreeSet::new();
    let mut cursor = listing.first_undefined_after(0);
    while let Some(start) = cursor {
        let hi = listing
            .next_instruction_start_after(start)
            .unwrap_or(u64::MAX);
        let mut advanced = if strict {
            kuna_aifstrict::next_probe_after(start, hi)
        } else {
            start.saturating_add(1)
        };
        if (!strict || kuna_aifstrict::probe_allowed(listing, start)) && !claimed.contains(&start) {
            decoder.linear_run = None;
            let body = match probe_gap_start(decoder, listing, &hist, start, hi, corroborate) {
                Probe::Accept(body) => {
                    entries.insert(start);
                    Some(body)
                }
                Probe::Uncorroborated(body) => Some(body),
                Probe::Reject => None,
            };
            if let Some(body) = body {
                advanced = body.last().copied().unwrap_or(start).saturating_add(1);
                claimed.extend(body);
            }
        }
        cursor = listing
            .first_undefined_after(advanced.saturating_sub(1))
            .filter(|&next| next > start);
    }
    entries.into_iter().collect()
}

fn hidden_fixture(kind: &str) -> (Sleigh, Rc<AddrSpace>, Listing) {
    let mut bytes = [0x00, 0xf0, 0x20, 0xe3].repeat(1024);
    let mut funcs = Vec::new();
    let prefix = [7, 0, 0xa0, 0xe3, 1, 0, 0x80, 0xe2];
    for i in 0..20 {
        let offset = i * 16;
        bytes[offset..offset + 8].copy_from_slice(&prefix);
        bytes[offset + 8..offset + 12].copy_from_slice(&[0x1e, 0xff, 0x2f, 0xe1]);
        funcs.push(DiscoveredFunction {
            entry: BASE + offset as u64,
            name: None,
            from_symbol: true,
            has_no_return: false,
            call_fixup: None,
        });
    }
    let entry = BASE + 0x400;
    bytes[0x400..0x408].copy_from_slice(&prefix);
    let branch = |from: u64, to: u64, call: bool| {
        let offset = ((to as i64 - from as i64 - 8) >> 2) as u32 & 0x00ff_ffff;
        ((if call { 0xeb00_0000 } else { 0xea00_0000 }) | offset).to_le_bytes()
    };
    let last = match kind {
        "call" => branch(entry + 8, BASE, true),
        "backward" => branch(entry + 8, BASE, false),
        "forward" | "sparse" => branch(entry + 8, BASE + 0x800, false),
        "loop" => branch(entry + 8, entry, false),
        "escape" => branch(entry + 8, BASE + 0x2000, false),
        "leaf" => [0x1e, 0xff, 0x2f, 0xe1],
        _ => panic!("unknown synthetic case"),
    };
    bytes[0x408..0x40c].copy_from_slice(&last);
    bytes[0x40c..0x410].copy_from_slice(&[0x1e, 0xff, 0x2f, 0xe1]);
    bytes[0x804..0x808].copy_from_slice(&[0x1e, 0xff, 0x2f, 0xe1]);
    let (engine, space) = arm(bytes);
    let insns = funcs
        .iter()
        .flat_map(|f| [f.entry, f.entry + 4, f.entry + 8])
        .map(|addr| {
            let d = decode_one(&engine, addr, &space, true, false).unwrap();
            let c = crate::listing::classify::classify(&d.ops, addr, d.len);
            Insn {
                addr,
                len: d.len,
                fall_through: c.fall_through,
                flow: c.flow,
                flows: c.flows,
                mnemonic: d.mnemonic,
                operands: d.operands,
                pcode: None,
            }
        })
        .collect();
    let ranges = if kind == "sparse" {
        vec![(BASE, BASE + 0x600), (BASE + 0x800, BASE + 0x1000)]
    } else {
        vec![(BASE, BASE + 0x1000)]
    };
    (
        engine,
        space,
        Listing::from_model_for_test(insns, funcs, Vec::new(), ranges),
    )
}

#[test]
fn retiring_and_span_scanning_preserve_entries_and_first_decode_order() {
    for kind in [
        "leaf", "call", "backward", "forward", "sparse", "loop", "escape",
    ] {
        for strict in [false, true] {
            for corroborate in [false, true] {
                let (old, old_space, old_listing) = hidden_fixture(kind);
                let (new, new_space, new_listing) = hidden_fixture(kind);
                let mut reference = GapDecoder::new(&old, old_space, old_listing.exec_ranges());
                let mut candidate = GapDecoder::new(&new, new_space, new_listing.exec_ranges());
                let want = legacy_walk(&old_listing, &mut reference, strict, corroborate);
                let got = run_aif_with_decoder(&new_listing, &mut candidate, strict, corroborate, None);
                assert_eq!(
                    got, want,
                    "{kind}, strict={strict}, corroborate={corroborate}"
                );
                let accepted = matches!(kind, "call" | "backward")
                    || (!corroborate && matches!(kind, "leaf" | "forward" | "sparse"));
                assert_eq!(got.contains(&(BASE + 0x400)), accepted, "{kind}");
                assert_eq!(candidate.decoded_addresses,reference.decoded_addresses,
                    "first-decode order changed: {kind}, strict={strict}, corroborate={corroborate}");
                assert!(candidate.peak_cache_len < reference.peak_cache_len / 2);
            }
        }
    }
}

#[test]
fn retirement_keeps_future_successes_and_failures_under_changed_context() {
    let (engine, space, listing) = padding_fixture(4096, 20);
    let mut decoder = GapDecoder::new(&engine, space, listing.exec_ranges());
    let future = BASE + 0x400;
    let first = decoder.probe(future).unwrap();
    assert!(decoder.probe(BASE + 0x2000).is_none());
    engine.with_context_db_mut(|db| db.set_variable_default(b"TMode", 1).unwrap());
    decoder.retire_before(future);
    let cached = decoder.probe(future).unwrap();
    assert!(Rc::ptr_eq(&first, &cached));
    assert!(decoder.probe(BASE + 0x2000).is_none());
    assert_eq!(decoder.decoded_addresses, vec![future, BASE + 0x2000]);
    decoder.retire_before(future + 1);
    assert!(!decoder.cache.contains_key(&future));
    assert!(decoder.cache.contains_key(&(BASE + 0x2000)));
}

#[test]
fn validation_instruction_limit_is_unchanged() {
    for count in [MAX_FOLLOW_INSNS, MAX_FOLLOW_INSNS + 1] {
        let mut bytes = [0x00, 0xf0, 0x20, 0xe3].repeat(count);
        bytes[(count - 1) * 4..].copy_from_slice(&[0x1e, 0xff, 0x2f, 0xe1]);
        let (engine, space) = arm(bytes);
        let end = BASE + (count * 4) as u64;
        let listing =
            Listing::from_model_for_test(Vec::new(), Vec::new(), Vec::new(), vec![(BASE, end)]);
        for strict in [false, true] {
            let mut decoder = GapDecoder::new(&engine, Rc::clone(&space), listing.exec_ranges());
            let body =
                check_valid_subroutine_with_policy(&mut decoder, &listing, BASE, BASE, end, strict);
            assert_eq!(body.is_some(), count == MAX_FOLLOW_INSNS);
            assert_eq!(decoder.decoded_addresses.len(), MAX_FOLLOW_INSNS);
        }
    }
}

#[test]
fn overflowing_fingerprint_end_is_rejected() {
    let (engine, space) = arm(Vec::new());
    let ranges = [(u64::MAX - 7, u64::MAX)];
    let mut decoder = GapDecoder::new(&engine, space, &ranges);
    assert!(decoder.fingerprint(u64::MAX - 7).is_none());
    assert_eq!(decoder.decoded_addresses, vec![u64::MAX - 7, u64::MAX - 3]);
}

#[test]
fn overlapping_linear_prefixes_preserve_validation_and_decode_order() {
    for backward in [false, true] {
        let count = 5000;
        let mut bytes = [0x00, 0xf0, 0x20, 0xe3].repeat(count);
        let tail = if backward {
            0xeafffffcu32.to_le_bytes()
        } else {
            [0x1e, 0xff, 0x2f, 0xe1]
        };
        bytes[(count - 1) * 4..].copy_from_slice(&tail);
        let (old, old_space) = arm(bytes.clone());
        let (new, new_space) = arm(bytes);
        let end = BASE + (count * 4) as u64;
        let listing =
            Listing::from_model_for_test(Vec::new(), Vec::new(), Vec::new(), vec![(BASE, end)]);
        for strict in [false, true] {
            let mut reference = GapDecoder::new(&old, Rc::clone(&old_space), listing.exec_ranges());
            let mut candidate = GapDecoder::new(&new, Rc::clone(&new_space), listing.exec_ranges());
            for i in (0..200).chain([1002, 1003, 4000, 4500]) {
                let entry = BASE + i * 4;
                reference.linear_run = None;
                candidate.retire_before(entry);
                let want = check_valid_subroutine_with_policy(
                    &mut reference,
                    &listing,
                    entry,
                    entry,
                    end,
                    strict,
                );
                let got = check_valid_subroutine_with_policy(
                    &mut candidate,
                    &listing,
                    entry,
                    entry,
                    end,
                    strict,
                );
                assert_eq!(got, want, "entry {i}, backward={backward}, strict={strict}");
                assert_eq!(got.is_some(), !backward && i >= 1002);
            }
            assert_eq!(candidate.decoded_addresses, reference.decoded_addresses);
            assert!(
                candidate.validation_steps < reference.validation_steps / 100,
                "{} versus {} visits",
                candidate.validation_steps,
                reference.validation_steps
            );
        }
    }
}

#[test]
fn cached_prefix_replays_deferred_branches_and_call_information() {
    for kind in ["forward", "backward", "call"] {
        let count = 5000;
        let mut bytes = [0x00, 0xf0, 0x20, 0xe3].repeat(count);
        for i in (0..count - 1).step_by(32) {
            let target = if kind == "forward" { count - 1 } else { 4 };
            let displacement = (target as i32 - i as i32 - 2) as u32 & 0x00ff_ffff;
            let opcode = if kind == "call" {
                0xeb00_0000
            } else {
                0x1a00_0000
            };
            bytes[i * 4..i * 4 + 4].copy_from_slice(&(opcode | displacement).to_le_bytes());
        }
        bytes[(count - 1) * 4..].copy_from_slice(&[0x1e, 0xff, 0x2f, 0xe1]);
        let (old, old_space) = arm(bytes.clone());
        let (new, new_space) = arm(bytes);
        let end = BASE + (count * 4) as u64;
        let listing =
            Listing::from_model_for_test(Vec::new(), Vec::new(), Vec::new(), vec![(BASE, end)]);
        for strict in [false, true] {
            let mut reference = GapDecoder::new(&old, Rc::clone(&old_space), listing.exec_ranges());
            let mut candidate = GapDecoder::new(&new, Rc::clone(&new_space), listing.exec_ranges());
            for i in (0..100).chain([1002, 1003, 4000, 4500]) {
                let entry = BASE + i * 4;
                reference.linear_run = None;
                candidate.retire_before(entry);
                let want = check_valid_subroutine_with_policy(
                    &mut reference,
                    &listing,
                    entry,
                    entry,
                    end,
                    strict,
                );
                let got = check_valid_subroutine_with_policy(
                    &mut candidate,
                    &listing,
                    entry,
                    entry,
                    end,
                    strict,
                );
                assert_eq!(got, want, "{kind}, entry {i}, strict={strict}");
                if i == 0 {
                    assert_eq!(got.is_some(), kind == "call" && !strict);
                }
            }
            assert_eq!(candidate.decoded_addresses, reference.decoded_addresses);
            assert!(candidate.validation_steps < reference.validation_steps / 30);
        }
    }
}

#[test]
fn unchanged_frame_evidence_does_not_decode_or_validate_the_gap_again() {
    let (engine, space, listing) = hidden_fixture("call");
    let root = BASE + 0x404;
    let mut decoder = GapDecoder::new(&engine, Rc::clone(&space), listing.exec_ranges());
    let body = check_valid_subroutine_strict(&mut decoder, &listing, root, root, u64::MAX).unwrap();
    let mut frames = ArmFrames::default();
    let spans = body
        .into_iter()
        .map(|at| (at, decoder.probe(at).unwrap().len))
        .collect();
    frames.validated(root, spans, &mut decoder);
    let roots = BTreeSet::from([root]);
    let replacements = frames.reconcile(&listing, &listing, &mut decoder, &roots, &[], true, false);
    assert_eq!(replacements, BTreeMap::from([(root, BASE + 0x400)]));
    assert!(!decoder.decoded_addresses.is_empty());

    let mut repeated = GapDecoder::new(&engine, space, listing.exec_ranges());
    assert_eq!(
        frames.reconcile(&listing, &listing, &mut repeated, &roots, &[], true, false),
        replacements
    );
    assert!(
        repeated.decoded_addresses.is_empty(),
        "unchanged evidence rescanned the gap"
    );
    assert_eq!(
        repeated.validation_steps, 0,
        "unchanged evidence revalidated frame bodies"
    );
}

#[test]
fn expanded_fingerprints_reuse_only_context_neutral_frame_instructions() {
    let (engine, space, listing) = hidden_fixture("call");
    let root = BASE + 0x404;
    let mut decoder = GapDecoder::new(&engine, Rc::clone(&space), listing.exec_ranges());
    decoder.full_capture = Some(Default::default());
    let body = check_valid_subroutine_strict(&mut decoder, &listing, root, root, u64::MAX).unwrap();
    let reusable: BTreeSet<_> = body
        .iter()
        .copied()
        .filter(|&at| decoder.probe(at).unwrap().reusable.is_some())
        .collect();
    assert!(
        !reusable.is_empty(),
        "the fixture must exercise certified reuse"
    );
    let spans = body
        .iter()
        .map(|&at| (at, decoder.probe(at).unwrap().len))
        .collect();
    let mut frames = ArmFrames::default();
    frames.validated(root, spans, &mut decoder);
    let unrelated = Listing::from_model_for_test(
        listing
            .instructions()
            .map(|(_, insn)| {
                let mut insn = insn.clone();
                insn.mnemonic = "unrelated".into();
                insn
            })
            .collect(),
        listing.functions().map(|(_, func)| func.clone()).collect(),
        Vec::new(),
        listing.exec_ranges().to_vec(),
    );
    let roots = BTreeSet::from([root]);
    assert!(frames
        .reconcile(&listing, &unrelated, &mut decoder, &roots, &[], true, false)
        .is_empty());
    let mut expanded = GapDecoder::new(&engine, space, listing.exec_ranges());
    assert_eq!(
        frames.reconcile(&listing, &listing, &mut expanded, &roots, &[], true, false),
        BTreeMap::from([(root, BASE + 0x400)])
    );
    assert!(
        expanded
            .decoded_addresses
            .iter()
            .all(|at| !reusable.contains(at)),
        "corpus growth redecoded a validated frame body"
    );
}
