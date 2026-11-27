//! Android NDK Binder services and the transaction codes compared by their registered callbacks.
mod image;
mod trace;

use std::collections::{BTreeMap, BTreeSet};

use crate::analyzers::nativeflow::Machine;
use image::Image;
use kuna_decomp::{
    architecture::Architecture,
    kuna_androidcalls::{BinderService, BinderTransaction},
};
use trace::Mode;

fn names(image: &Image, machine: &Machine, at: u64, len: u64) -> Option<BTreeMap<u32, String>> {
    let width = image.width as u64;
    if at % width != 0 || len == 0 || len > 16384 || len * width > image.data(at)?.len() as u64 {
        return None;
    }
    let mut out = BTreeMap::new();
    let mut budget = 1usize << 20;
    for i in 0..len {
        let ptr = image.pointer(at.checked_add(i * width)?, machine)?;
        if ptr == 0 {
            continue;
        }
        let name = image.text(ptr)?;
        budget = budget.checked_sub(name.len())?;
        out.insert(i as u32 + 1, name.to_owned());
    }
    Some(out)
}

fn case_sites(
    image: &Image,
    machine: &Machine,
    callback: u64,
    end: u64,
    budget: &mut usize,
) -> BTreeSet<(u32, u64)> {
    let mut sites = BTreeSet::new();
    let Some(facts) = trace::function(image, machine, callback, end, Mode::Dispatch(None), budget)
    else {
        return sites;
    };
    sites.extend(
        facts
            .cases
            .into_iter()
            .filter_map(|(pc, code)| Some((code?, pc))),
    );
    if !facts.indirect.is_empty() && facts.candidates.len() <= 256 {
        // The range comparison bounds the input probes. A case address is published only
        // when substituting distinct codes selects distinct local targets of the same jump.
        let mut jumps = BTreeMap::<u64, Vec<(u32, u64)>>::new();
        for &code in &facts.candidates {
            let Some(probe) = trace::function(
                image,
                machine,
                callback,
                end,
                Mode::Dispatch(Some(code)),
                budget,
            ) else {
                continue;
            };
            for (jump, site) in probe.indirect {
                if facts.indirect.get(&jump) == Some(&None) {
                    if let Some(site) = site {
                        jumps.entry(jump).or_default().push((code, site));
                    }
                }
            }
        }
        for probes in jumps.into_values() {
            if probes.iter().any(|&(_, site)| site != probes[0].1) {
                for (code, site) in probes {
                    sites.insert((code, site));
                }
            }
        }
    }
    sites
}

/// Start with proven call targets so ARM instruction-set evidence takes precedence over
/// an untagged EHABI range. Include hidden direct callees without their own unwind record.
fn private_table(
    image: &Image,
    machine: &Machine,
    bodies: &BTreeMap<u64, u64>,
    roots: &[u64],
    range: std::ops::Range<u64>,
    budget: &mut usize,
) -> bool {
    if image.has_alias(range.clone()) {
        return false;
    }
    let mut trusted = roots.to_vec();
    let mut metadata = bodies.keys().copied();
    let mut seen = BTreeSet::new();
    while let Some(raw) = trusted.pop().or_else(|| metadata.next()) {
        if !seen.insert(image.pc(raw)) {
            continue;
        }
        let Some(facts) = trace::function(
            image,
            machine,
            raw,
            image.end(bodies, raw),
            Mode::Watch(range.clone()),
            budget,
        ) else {
            return false;
        };
        if facts.exposed {
            return false;
        }
        trusted.extend(facts.callees);
    }
    true
}

/// Recover native callbacks passed to libbinder_ndk and the optional API 36 transaction names.
pub fn services(file: &object::File, bytes: &[u8], arch: &Architecture) -> Vec<BinderService> {
    let Some(mut image) = Image::new(file, bytes) else {
        return vec![];
    };
    let Some(machine) = Machine::new(file, bytes, arch) else {
        return vec![];
    };
    let bodies = image.bodies();
    let thunks: Vec<_> = bodies
        .iter()
        .filter(|(entry, end)| image.candidate(**entry, **end))
        .filter_map(|(&entry, _)| Some((entry, trace::forwarding_target(&image, &machine, entry)?)))
        .collect();
    image.stubs.extend(thunks);
    let mut defs = BTreeMap::new();
    let mut maps = BTreeMap::<u64, BTreeSet<Option<(u64, u64)>>>::new();
    let mut budget = 80000;
    let mut watch_budget = 80000;
    let mut case_budget = 80000;
    for (&entry, &end) in &bodies {
        if budget == 0 {
            break;
        }
        if !image.candidate(entry, end) {
            continue;
        }
        let Some(facts) = trace::function(&image, &machine, entry, end, Mode::Factory, &mut budget)
        else {
            continue;
        };
        for (site, words) in facts.classes {
            if let Some(words) = words {
                defs.insert(site, words);
            }
        }
        for (owner, tables) in facts.maps {
            maps.entry(owner).or_default().extend(tables);
        }
    }
    let mut result = Vec::new();
    for (site, [descriptor, create, destroy, callback]) in defs {
        let Some(descriptor) = image.text(descriptor) else {
            continue;
        };
        let pc = image.pc(callback);
        let end = image.end(&bodies, callback);
        let proven_cases = case_sites(&image, &machine, callback, end, &mut case_budget);
        let mut roots: Vec<u64> = proven_cases
            .iter()
            .map(|&(_, site)| site | (callback & 1))
            .collect();
        roots.extend([create, destroy, callback]);
        let tables = maps.get(&site);
        let method_names = tables
            .filter(|sets| sets.len() == 1)
            .and_then(BTreeSet::first)
            .and_then(|t| *t)
            .and_then(|(at, len)| {
                let table = names(&image, &machine, at, len)?;
                let size = len.checked_mul(image.width as u64)?;
                let end = at.checked_add(size)?;
                let stable = image.immutable(at, size)
                    || private_table(
                        &image,
                        &machine,
                        &bodies,
                        &roots,
                        at..end,
                        &mut watch_budget,
                    );
                stable.then_some(table)
            })
            .unwrap_or_default();
        let mut transactions = BTreeSet::new();
        let mut matched = BTreeSet::new();
        for (code, site) in proven_cases {
            matched.insert(code);
            transactions.insert(BinderTransaction {
                code,
                name: method_names.get(&code).cloned(),
                site: Some(site),
            });
        }
        for (code, name) in method_names {
            if !matched.contains(&code) {
                transactions.insert(BinderTransaction {
                    code,
                    name: Some(name),
                    site: None,
                });
            }
        }
        result.push(BinderService {
            descriptor: descriptor.into(),
            define_site: site,
            create: image.pc(create),
            destroy: image.pc(destroy),
            transact: pc,
            transactions: transactions.into_iter().collect(),
        });
    }
    result
}
