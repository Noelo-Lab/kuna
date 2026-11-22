//! Name proposals require complete profile selection and exact agreement of both banks.

use std::collections::BTreeSet;

use super::{select, Bank, Report, Selection};
use crate::pass::FidMatch;

impl Report {
    /// Produce signature proposals; the normal commit still owns name arbitration.
    pub fn agreed_names(&self) -> Result<Vec<FidMatch>, String> {
        if self.candidates.iter().any(|candidate| {
            !candidate.missing_banks.is_empty()
                || candidate.banks.len() != 2
                || [Bank::Inline, Bank::NoInline].iter().any(|bank| {
                    !candidate.banks.iter().any(|source| {
                        source.source.bank == *bank && source.source.profile == candidate.profile
                    })
                })
        }) {
            return Err(
                "incomplete Rust signature comparison: every profile requires both banks".into(),
            );
        }
        let Selection::Unique(profile) = select(&self.candidates) else {
            return Err(format!("Rust signature application requires a complete unique positive profile; found {:?}", self.selection));
        };
        if self.selection != Selection::Unique(profile) {
            return Err("Rust signature selection disagrees with its candidate scores".into());
        }
        let candidate = self
            .candidates
            .iter()
            .find(|candidate| candidate.profile == profile)
            .ok_or("selected Rust signature profile is absent")?;
        if !candidate.missing_banks.is_empty() || candidate.banks.len() != 2 {
            return Err("selected Rust signature profile requires both banks".into());
        }
        let names = |bank| -> Result<BTreeSet<(u64, String)>, String> {
            let source = candidate
                .banks
                .iter()
                .find(|source| source.source.bank == bank)
                .ok_or("selected Rust signature profile requires both banks")?;
            Ok(source
                .matches
                .iter()
                .map(|matched| (matched.address, matched.symbol.clone()))
                .collect())
        };
        let inline = names(Bank::Inline)?;
        let no_inline = names(Bank::NoInline)?;
        let agreed: Vec<_> = inline
            .intersection(&no_inline)
            .map(|(addr, name)| FidMatch {
                addr: *addr,
                name: name.clone(),
                by_signature: true,
            })
            .collect();
        if agreed.is_empty() {
            return Err("selected Rust signature banks agree on no symbol/address pairs".into());
        }
        Ok(agreed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flirt::kuna_rustprofiles::{
        BankScore, CandidateScore, Library, LibraryMatch, Profile, Source,
    };

    fn report() -> Report {
        let profile = Profile {
            version: (1, 88, 0),
            optimization: 3,
        };
        Report {
            selection: Selection::Unique(profile),
            candidates: vec![CandidateScore {
                profile,
                missing_banks: vec![],
                score: Some(1),
                banks: [Bank::Inline, Bank::NoInline]
                    .into_iter()
                    .map(|bank| BankScore {
                        source: Source {
                            profile,
                            bank,
                            identity: "authored".into(),
                        },
                        signature_library: "test".into(),
                        matches: vec![LibraryMatch {
                            address: 0x1000,
                            symbol: "core::read".into(),
                            demangled: "core::read".into(),
                            library: Library::Core,
                        }],
                    })
                    .collect(),
            }],
        }
    }

    #[test]
    fn exact_raw_identity_at_same_address_is_required() {
        let mut report = report();
        let proposals = report.agreed_names().unwrap();
        assert_eq!(proposals.len(), 1);
        assert_eq!(
            (
                proposals[0].addr,
                proposals[0].name.as_str(),
                proposals[0].by_signature
            ),
            (0x1000, "core::read", true)
        );
        report.candidates[0].banks[1].matches[0].symbol = "core::read_other_hash".into();
        assert!(report.agreed_names().is_err());
        report.candidates[0].banks[1].matches[0].symbol = "core::read".into();
        report.candidates[0].banks[1].matches[0].address = 0x2000;
        assert!(report.agreed_names().is_err());
    }

    #[test]
    fn best_bank_count_does_not_authorize_its_extra_names() {
        let mut report = report();
        let mut extra = report.candidates[0].banks[0].matches[0].clone();
        extra.address = 0x2000;
        report.candidates[0].banks[0].matches.push(extra);
        report.candidates[0].score = Some(2);
        assert_eq!(report.agreed_names().unwrap().len(), 1);
    }

    #[test]
    fn incomplete_or_tied_candidates_decline_even_with_stale_unique_selection() {
        let mut report = report();
        let mut other = report.candidates[0].clone();
        other.profile.version = (1, 71, 0);
        report.candidates.push(other);
        assert!(report.agreed_names().is_err());
        report.candidates[1].score = None;
        report.candidates[1].missing_banks.push(Bank::Inline);
        assert!(report.agreed_names().is_err());
        report.candidates.remove(1);
        report.candidates[0].score = Some(0);
        assert!(report.agreed_names().is_err());
    }

    #[test]
    fn agreeing_conflicts_remain_proposals_for_shared_arbitration() {
        let mut report = report();
        for bank in &mut report.candidates[0].banks {
            let mut conflict = bank.matches[0].clone();
            conflict.symbol = "core::write".into();
            bank.matches.push(conflict);
        }
        report.candidates[0].score = Some(2);
        assert_eq!(report.agreed_names().unwrap().len(), 2);
    }
}
