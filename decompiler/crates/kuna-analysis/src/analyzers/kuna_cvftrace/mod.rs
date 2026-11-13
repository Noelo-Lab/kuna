//! Compaq Visual Fortran PE32 `.trace` routine names and source locations.
//!
//! Addresses in the section are preferred virtual addresses, not RVAs. The
//! runtime's line program describes intervals ending at each advanced PC,
//! inclusively. File selections advance through the string table; their index
//! is relative to the remaining table, not its beginning. Routine names are the
//! lowest-priority function names (`AnalysisOutput::fallback_names`); the
//! source-line comments are a separate pass so they stay opt-in.

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, CommentFact, FidMatch, Phase};
use object::{Object, ObjectSection};

mod decode;
pub use decode::{decode, Line, Routine, Trace};

pub struct CvfTracePass;
pub struct CvfTraceLinesPass;

/// The decoded routines and lines of an x86 PE32 image's `.trace` section,
/// keeping only routines that start in executable sections and lines that
/// belong to them.
fn facts(ctx: &AnalysisCtx) -> Option<(Vec<Routine>, Vec<Line>)> {
    if ctx.file.format() != object::BinaryFormat::Pe
        || ctx.file.architecture() != object::Architecture::I386
    {
        return None;
    }
    let bytes = ctx.file.section_by_name(".trace")?.data().ok()?;
    let trace = decode(bytes);
    let executable = |addr: u64| {
        ctx.file.sections().any(|s| {
            crate::entry::is_executable(&s)
                && addr >= s.address()
                && addr - s.address() < s.size()
        })
    };
    let routines: Vec<Routine> =
        trace.routines.into_iter().filter(|r| executable(r.addr)).collect();
    let valid: std::collections::BTreeSet<u64> = routines.iter().map(|r| r.addr).collect();
    let lines = trace
        .lines
        .into_iter()
        .filter(|l| valid.contains(&l.func_addr) && executable(l.start))
        .collect();
    Some((routines, lines))
}

impl AnalysisPass for CvfTracePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "cvftrace"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if let Some((routines, _)) = facts(ctx) {
            out.fallback_names = routines
                .into_iter()
                .map(|r| FidMatch {
                    addr: r.addr,
                    name: r.name,
                })
                .collect();
        }
        out
    }
}

impl AnalysisPass for CvfTraceLinesPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "cvftrace_lines"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if let Some((_, lines)) = facts(ctx) {
            for line in lines {
                let file = line.file.rsplit(['\\', '/']).next().unwrap_or(&line.file);
                out.comments.push(CommentFact {
                    func_addr: line.func_addr,
                    addr: line.start,
                    text: format!("{file}:{}", line.line),
                });
            }
        }
        out
    }
}
