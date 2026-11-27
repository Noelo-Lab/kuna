//! Run a proof obligation with a wall-clock deadline and bounded output.

use std::io::{Read, Seek, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const OUTPUT_LIMIT: u64 = 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Error {
    Timeout,
    Unavailable(String),
    Failed(String),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Status {
    Sat,
    Unsat,
    Unknown(String),
}

pub(super) fn run(
    process: &mut Command,
    script: &str,
    deadline: Instant,
) -> Result<(Status, String), Error> {
    parse_output(run_text(process, script, deadline)?)
}

pub(super) fn run_text(
    process: &mut Command,
    script: &str,
    deadline: Instant,
) -> Result<String, Error> {
    let io = |err: std::io::Error| Error::Failed(err.to_string());
    if Instant::now() >= deadline {
        return Err(Error::Timeout);
    }
    let mut stdin = tempfile::tempfile().map_err(io)?;
    stdin.write_all(script.as_bytes()).map_err(io)?;
    stdin.rewind().map_err(io)?;
    let mut stdout = tempfile::tempfile().map_err(io)?;
    let mut stderr = tempfile::tempfile().map_err(io)?;
    let mut child = process
        .stdin(Stdio::from(stdin))
        .stdout(Stdio::from(stdout.try_clone().map_err(io)?))
        .stderr(Stdio::from(stderr.try_clone().map_err(io)?))
        .spawn()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                Error::Unavailable(err.to_string())
            } else {
                io(err)
            }
        })?;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(
                    Duration::from_millis(4)
                        .min(deadline.saturating_duration_since(Instant::now())),
                );
            }
            outcome => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(outcome.err().map_or(Error::Timeout, io));
            }
        }
    };
    if !status.success() {
        let err = read_output(&mut stderr)?;
        let stdout = read_output(&mut stdout)?;
        let diagnosis = stdout
            .lines()
            .find(|line| line.trim_start().starts_with("(error "))
            .or_else(|| err.lines().next())
            .unwrap_or("no diagnostic");
        return Err(Error::Failed(format!(
            "solver exited with {status}: {diagnosis}",
        )));
    }
    read_output(&mut stdout)
}

fn read_output(file: &mut std::fs::File) -> Result<String, Error> {
    let io = |err: std::io::Error| Error::Failed(err.to_string());
    file.rewind().map_err(io)?;
    let mut output = String::new();
    let size = file
        .take(OUTPUT_LIMIT + 1)
        .read_to_string(&mut output)
        .map_err(io)?;
    if size as u64 > OUTPUT_LIMIT {
        return Err(Error::Failed(
            "solver output exceeds its byte limit".to_owned(),
        ));
    }
    Ok(output)
}

fn parse_output(output: String) -> Result<(Status, String), Error> {
    let mut lines = output.lines();
    let status = match lines.next().map(str::trim) {
        Some("sat") => Status::Sat,
        Some("unsat") => Status::Unsat,
        Some("unknown") => Status::Unknown(
            lines
                .find_map(|line| {
                    line.trim()
                        .strip_prefix("(:reason-unknown ")
                        .and_then(|reason| reason.strip_suffix(')'))
                        .map(|reason| reason.trim_matches('"').to_owned())
                })
                .filter(|reason| !reason.is_empty())
                .unwrap_or_else(|| "solver did not report a reason".to_owned()),
        ),
        _ => {
            return Err(Error::Failed(
                "solver did not return a satisfiability status".to_owned(),
            ))
        }
    };
    if output
        .lines()
        .any(|line| line.trim_start().starts_with("(error "))
    {
        return Err(Error::Failed("solver rejected the obligation".to_owned()));
    }
    Ok((status, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_only_explicit_solver_answers() {
        assert_eq!(parse_output("unsat\n".to_owned()).unwrap().0, Status::Unsat);
        assert_eq!(
            parse_output("unknown\n(:reason-unknown \"incomplete\")\n".to_owned())
                .unwrap()
                .0,
            Status::Unknown("incomplete".to_owned()),
        );
        assert!(parse_output("failed\n".to_owned()).is_err());
        assert!(parse_output("unsat\n(error \"bad input\")\n".to_owned()).is_err());
    }

    #[test]
    #[cfg(unix)]
    fn kills_and_reaps_an_unresponsive_solver() {
        let mut command = Command::new("sh");
        command.args(["-c", "while :; do :; done"]);
        let before = Instant::now();
        let result = run(&mut command, "", before + Duration::from_millis(50));
        assert_eq!(result, Err(Error::Timeout));
        assert!(before.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn preserves_sat_and_unsat_from_z3() {
        if Command::new("z3").arg("-version").output().is_err() {
            return;
        }
        let mut command = Command::new("z3");
        command.args(["-in", "-smt2"]);
        for (assertion, expected) in [("true", Status::Sat), ("false", Status::Unsat)] {
            let query = format!("(set-logic QF_BV)\n(assert {assertion})\n(check-sat)\n");
            let result = run(
                &mut command,
                &query,
                Instant::now() + Duration::from_secs(5),
            );
            assert_eq!(result.unwrap().0, expected);
        }
    }
}
