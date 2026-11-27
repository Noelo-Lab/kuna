//! Prove an observation equality over a separately checked initial domain.

use std::collections::BTreeMap;
use std::process::Command;
use std::time::Instant;

use super::solver::{self, Status};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Verdict {
    Equivalent,
    Counterexample(BTreeMap<String, u8>),
    InfeasibleDomain,
    Unresolved { phase: &'static str, reason: String },
}

/// The common declarations and preconditions, plus one mismatch predicate.
/// Terms here are produced by the symbolic encoders.
pub(super) struct Obligation {
    pub domain: String,
    pub mismatch: String,
    pub input_bytes: Vec<String>,
}

impl Obligation {
    pub fn domain_query(&self) -> String {
        format!("{}\n(check-sat)\n(get-info :reason-unknown)\n", self.domain)
    }

    pub fn difference_query(&self) -> String {
        format!(
            "{}\n(assert {})\n(check-sat)\n(get-info :reason-unknown)\n",
            self.domain, self.mismatch,
        )
    }

    fn witness_query(&self) -> String {
        format!(
            "{}\n(assert {})\n(check-sat)\n(get-value ({}))\n",
            self.domain,
            self.mismatch,
            self.input_bytes.join(" "),
        )
    }

    pub fn solve(&self, command: &mut Command, deadline: Instant) -> Verdict {
        match solver::run(command, &self.domain_query(), deadline) {
            Ok((Status::Sat, _)) => {}
            Ok((Status::Unsat, _)) => return Verdict::InfeasibleDomain,
            other => return unresolved("initial-domain", other),
        }
        match solver::run(command, &self.difference_query(), deadline) {
            Ok((Status::Unsat, _)) => return Verdict::Equivalent,
            Ok((Status::Sat, _)) => {}
            other => return unresolved("observation-mismatch", other),
        }
        if self.input_bytes.is_empty() {
            return Verdict::Counterexample(BTreeMap::new());
        }
        match solver::run(command, &self.witness_query(), deadline) {
            Ok((Status::Sat, text)) => match witness(&text, &self.input_bytes) {
                Some(values) => Verdict::Counterexample(values),
                None => Verdict::Unresolved {
                    phase: "witness",
                    reason: "solver did not return all requested input bytes".to_owned(),
                },
            },
            other => unresolved("witness", other),
        }
    }
}

fn unresolved(phase: &'static str, answer: Result<(Status, String), solver::Error>) -> Verdict {
    let reason = match answer {
        Ok((Status::Unknown(reason), _)) => reason,
        Ok((status, _)) => format!("unexpected solver status {status:?}"),
        Err(solver::Error::Timeout) => "solver wall-clock deadline".to_owned(),
        Err(solver::Error::Unavailable(reason) | solver::Error::Failed(reason)) => reason,
    };
    Verdict::Unresolved { phase, reason }
}

fn witness(text: &str, expected: &[String]) -> Option<BTreeMap<String, u8>> {
    let (_, values) = text.split_once('\n')?;
    let tokens: Vec<_> = values
        .split(|c: char| c.is_ascii_whitespace() || matches!(c, '(' | ')'))
        .filter(|s| !s.is_empty())
        .collect();
    if tokens.len() != 2 * expected.len() {
        return None;
    }
    let mut inputs = BTreeMap::new();
    for pair in tokens.chunks_exact(2) {
        let (name, value) = (pair[0], pair[1]);
        let byte = match value.strip_prefix("#x") {
            Some(hex) if hex.len() == 2 => u8::from_str_radix(hex, 16).ok()?,
            None => {
                let binary = value.strip_prefix("#b").filter(|s| s.len() == 8)?;
                u8::from_str_radix(binary, 2).ok()?
            }
            _ => return None,
        };
        if inputs.insert(name.to_owned(), byte).is_some() {
            return None;
        }
    }
    expected
        .iter()
        .all(|name| inputs.contains_key(name))
        .then_some(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn rejects_partial_or_oversized_witnesses() {
        let names = vec!["b0".to_owned(), "b1".to_owned()];
        assert!(witness("sat\n((b0 #x12))", &names).is_none());
        assert!(witness("sat\n((b0 #x12) (b1 #x0001))", &names).is_none());
        assert!(witness("sat\n((b0 #x12) (b0 #x34))", &names).is_none());
    }

    #[test]
    fn real_z3_checks_domain_then_observation_and_returns_the_witness() {
        if Command::new("z3").arg("-version").output().is_err() {
            return;
        }
        let mut z3 = Command::new("z3");
        z3.args(["-in", "-smt2"]);
        let test = |z3: &mut Command, precondition: &str, mismatch: &str| {
            Obligation {
                domain: format!(
                    "(set-logic QF_BV)\n(declare-const b0 (_ BitVec 8))\n(assert {precondition})"
                ),
                mismatch: mismatch.to_owned(),
                input_bytes: vec!["b0".to_owned()],
            }
            .solve(z3, Instant::now() + Duration::from_secs(5))
        };
        let diff = "(distinct (bvadd b0 #x01) b0)";
        assert_eq!(test(&mut z3, "false", diff), Verdict::InfeasibleDomain);
        assert_eq!(
            test(&mut z3, "true", "(distinct (bvadd b0 #x00) b0)"),
            Verdict::Equivalent
        );
        assert_eq!(
            test(&mut z3, "(= b0 #xff)", diff),
            Verdict::Counterexample(BTreeMap::from([("b0".to_owned(), 255)])),
        );
    }
}
