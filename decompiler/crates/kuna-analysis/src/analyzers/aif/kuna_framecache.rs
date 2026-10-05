//! Original-context frame validation, retained across provisional Listing rebuilds.

use super::*;

#[derive(Default)]
pub(crate) struct ArmFrames {
    pub roots: Vec<u64>,
    bodies: BTreeMap<u64, Option<Vec<(u64, u32)>>>,
    prefixes: Option<PrefixScan>,
    instructions: Rc<BTreeMap<u64, Rc<ProbedInsn>>>,
}

struct PrefixScan {
    roots: BTreeSet<u64>,
    histogram: BTreeMap<Fingerprint, usize>,
    replacements: BTreeMap<u64, u64>,
    instructions: BTreeMap<u64, Rc<ProbedInsn>>,
    bodies: BTreeMap<u64, BTreeSet<u64>>,
}

impl ArmFrames {
    pub(crate) fn replay(
        &self,
        at: u64,
        translate: &dyn Translate,
        space: &Rc<AddrSpace>,
        capture: &mut crate::listing::xrefs::FullCapture,
    ) -> Option<u32> {
        let insn = self.instructions.get(&at)?;
        insn.reusable
            .as_ref()?
            .replay(at, translate, space, capture)
            .then_some(insn.len)
    }

    /// Did `root`'s validated body claim the instruction at `at`?
    pub(crate) fn body_covers(&self, root: u64, at: u64) -> bool {
        self.bodies
            .get(&root)
            .and_then(Option::as_ref)
            .is_some_and(|body| body.iter().any(|&(start, _)| start == at))
    }

    pub(super) fn validated(&mut self, root: u64, body: Vec<(u64, u32)>, decoder: &mut GapDecoder) {
        let instructions = Rc::make_mut(&mut self.instructions);
        instructions.extend(body.iter().map(|&(at, _)| (at, decoder.probe(at).unwrap())));
        self.bodies.insert(root, Some(body));
    }

    pub(super) fn validated_body(&mut self, root: u64, body: Vec<(u64, u32)>) {
        self.bodies.insert(root, Some(body));
    }

    /// Every old instruction must still be reachable with identical decoding.
    /// A32 prefixes without interworking leave unrelated callees' decode modes intact.
    pub(crate) fn preserves_frame_bodies(&self, replacements: &BTreeMap<u64, u64>) -> bool {
        let Some(scan) = &self.prefixes else {
            return false;
        };
        replacements.iter().all(|(&root, &prefix)| {
            let Some(Some(body)) = self.bodies.get(&root) else {
                return false;
            };
            let Some(prefix_body) = scan.bodies.get(&prefix) else {
                return false;
            };
            prefix_body.iter().all(|at| {
                let insn = &scan.instructions[at];
                insn.len == 4 && !insn.mnemonic.starts_with("blx")
            }) && body.iter().all(|&(at, _)| {
                prefix_body.contains(&at)
                    && self
                        .instructions
                        .get(&at)
                        .zip(scan.instructions.get(&at))
                        .is_some_and(|(old, new)| old == new)
            })
        })
    }

    pub(crate) fn prefixes_in_arm_context(
        &self,
        replacements: &BTreeMap<u64, u64>,
        arch: &kuna_decomp::architecture::Architecture,
        space: &Rc<AddrSpace>,
    ) -> bool {
        let Some(scan) = &self.prefixes else {
            return false;
        };
        arch.with_context_db_mut(|db| {
            replacements.values().all(|prefix| {
                scan.bodies[prefix].iter().all(|&at| {
                    let addr = kuna_base::address::Address::new(Rc::clone(space), at);
                    db.get_variable_value(b"TMode", &addr).ok() == Some(0)
                })
            })
        })
    }

    pub(super) fn reconcile(
        &mut self,
        listing: &Listing,
        corpus: &Listing,
        decoder: &mut GapDecoder,
        roots: &BTreeSet<u64>,
        focused: &[u64],
        strict: bool,
        corroborate: bool,
    ) -> BTreeMap<u64, u64> {
        let histogram: BTreeMap<_, _> = build_fingerprint_histogram(corpus, decoder)
            .into_iter()
            .filter(|(_, count)| *count >= FINGERPRINT_THRESHOLD)
            .map(|(fp, count)| {
                (
                    fp,
                    if corroborate && count >= kuna_aifcorroborate::UNCORROBORATED_THRESHOLD {
                        kuna_aifcorroborate::UNCORROBORATED_THRESHOLD
                    } else {
                        FINGERPRINT_THRESHOLD
                    },
                )
            })
            .collect();
        if !histogram
            .values()
            .any(|&count| count >= FINGERPRINT_THRESHOLD)
        {
            return BTreeMap::new();
        }
        decoder.frame_instructions = Some(Rc::clone(&self.instructions));
        decoder.full_capture.get_or_insert_with(Default::default);
        decoder.cache.clear();
        decoder.linear_run = None;
        decoder.linear_events.clear();
        if self
            .prefixes
            .as_ref()
            .is_none_or(|scan| scan.roots != *roots || scan.histogram != histogram)
        {
            let mut claimed = BTreeMap::new();
            let mut validated = BTreeSet::new();
            for &root in roots {
                let body = self.bodies.entry(root).or_insert_with(|| {
                    let gap_hi = listing
                        .next_instruction_start_after(root)
                        .unwrap_or(u64::MAX);
                    check_valid_subroutine_strict(decoder, listing, root, root, gap_hi).map(
                        |body| {
                            body.into_iter()
                                .map(|at| (at, decoder.probe(at).unwrap().len))
                                .collect()
                        },
                    )
                });
                if let Some(body) = body {
                    validated.insert(root);
                    claimed.extend(body.iter().copied());
                }
            }
            let mut probed = BTreeMap::new();
            let replacements = reconcile_frame_prefixes(
                listing,
                corpus,
                decoder,
                &validated,
                claimed,
                strict,
                corroborate,
                &mut probed,
            );
            let mut instructions = BTreeMap::new();
            let mut bodies = BTreeMap::new();
            for prefix in replacements.values().copied().collect::<BTreeSet<_>>() {
                let body = probed.remove(&prefix).unwrap();
                bodies.insert(prefix, body.keys().copied().collect());
                instructions.extend(body);
            }
            self.prefixes = Some(PrefixScan {
                roots: roots.clone(),
                histogram,
                replacements,
                instructions,
                bodies,
            });
        }
        decoder.frame_instructions = None;
        let scan = self.prefixes.as_ref().unwrap();
        let mut replacements = scan.replacements.clone();
        if !replacements.is_empty() {
            let independent = listing
                .functions()
                .map(|(&entry, _)| entry)
                .chain(focused.iter().copied())
                .chain(
                    roots
                        .iter()
                        .copied()
                        .filter(|root| !replacements.contains_key(root)),
                )
                .chain(replacements.values().copied());
            let called =
                kuna_framecalls::independently_called(corpus, independent, &scan.instructions);
            replacements.retain(|root, _| !called.contains(root));
        }
        replacements
    }
}
