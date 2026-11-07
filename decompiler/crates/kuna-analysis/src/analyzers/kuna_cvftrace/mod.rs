//! Compaq Visual Fortran PE32 `.trace` names and source locations (opt-in).
//!
//! Addresses in the section are preferred virtual addresses, not RVAs. The
//! runtime's line program describes intervals ending at each advanced PC,
//! inclusively. File selections advance through the string table; their index
//! is relative to the remaining table, not its beginning.

use crate::pass::{
    AnalysisCtx, AnalysisOutput, AnalysisPass, CommentFact, FidMatch, Phase, SymFact, SymKind,
};
use object::{Object, ObjectSection};

mod decode;
pub use decode::{decode, Line, Routine, Trace};

pub struct CvfTracePass;

impl AnalysisPass for CvfTracePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "cvftrace"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if ctx.file.format() != object::BinaryFormat::Pe
            || ctx.file.architecture() != object::Architecture::I386
        {
            return out;
        }
        let Some(section) = ctx.file.section_by_name(".trace") else {
            return out;
        };
        let Ok(bytes) = section.data() else {
            return out;
        };
        let trace = decode(bytes);
        let executable = |addr: u64| {
            ctx.file.sections().any(|s| {
                crate::entry::is_executable(&s)
                    && addr >= s.address()
                    && addr - s.address() < s.size()
            })
        };
        let valid: std::collections::BTreeSet<_> = trace
            .routines
            .iter()
            .filter(|r| executable(r.addr))
            .map(|r| r.addr)
            .collect();
        for routine in trace.routines {
            if !valid.contains(&routine.addr) {
                continue;
            }
            out.symbols.push(SymFact {
                addr: routine.addr,
                name: routine.name.clone(),
                kind: SymKind::Function,
            });
            out.fid_names.push(FidMatch {
                addr: routine.addr,
                name: routine.name,
            });
        }
        for line in trace.lines {
            if valid.contains(&line.func_addr) && executable(line.start) {
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
