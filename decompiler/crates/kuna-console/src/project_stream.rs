//! The **streaming** half of the project export: the pieces
//! `kuna decompile-project --stream` needs that the one-shot builders in
//! [`crate::project`] do not expose.
//!
//! A streamed export writes its artifacts while it runs, so the two artifacts a
//! non-stream run builds in one call have to come apart:
//!
//! * [`AsmSweep`] — the linear disassembly as a resumable walk over a
//!   `std::io::Write` sink, so the `.asm` can be filled in bounded steps
//!   interleaved with decompilation.  [`crate::project::build_asm`] is the same
//!   walk with an unbounded budget, which is what keeps the two byte-identical.
//! * [`render_variables_section`] / [`render_data_tail`] — the two tails a
//!   streamed `.asm` appends once the decompile finishes, because the sweep
//!   itself completes long before the variables they describe exist.
//!
//! [`AsmSweep`] borrows the `ConsoleProgram` only for the duration of a step, so
//! a caller can hold a half-finished sweep across a `&mut ConsoleProgram`
//! decompile — which is what lets a serial streamed export interleave the two.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};

use kuna_sleigh::loadimage::section_flags;

use crate::engine::{ConsoleProgram, FunctionEntry};
use crate::project::{self, AssemblyScratch, FuncResult};

/// Flush the step buffer to the sink once it passes this, so a sweep of a large
/// image never holds the whole `.asm` in memory.  Only ever crossed between two
/// whole lines, so where it lands cannot change the bytes written.
const FLUSH_BYTES: usize = 64 * 1024;

/// One function label the sweep plants in the disassembly.
#[derive(Debug, Clone)]
pub struct AsmLabel {
    /// Engine byte address — where in the sweep the label lands.
    pub byte_address: u64,
    /// The address printed in the label comment (target units on a raw image).
    pub display_address: u64,
    pub name: String,
    /// The lines printed under the label, each already newline-terminated.  The
    /// variable comment block for a non-stream export; empty for a streamed one,
    /// which collects those lines into [`render_variables_section`] instead.
    pub header: String,
}

/// Label blocks for a finished result list, in the results' own order.
///
/// The sweep groups labels by address, keeping each group in the order given —
/// so passing the address-sorted result list every export builds reproduces the
/// non-stream `.asm` exactly.
pub fn asm_labels_from_results(results: &[FuncResult]) -> Vec<AsmLabel> {
    results
        .iter()
        .map(|r| AsmLabel {
            byte_address: r.byte_address,
            display_address: r.address,
            name: r.name.clone(),
            header: project::var_header_lines(r),
        })
        .collect()
}

/// Label blocks for a target list, before any of it has been decompiled —
/// address-ordered, then by name, with no variable comments (a streamed export
/// sweeps the image while the decompile is still running, so the variables do
/// not exist yet).
pub fn asm_labels_from_entries(prog: &ConsoleProgram, entries: &[FunctionEntry]) -> Vec<AsmLabel> {
    let arm = is_arm(prog);
    let mut labels: Vec<AsmLabel> = entries
        .iter()
        .map(|e| AsmLabel {
            byte_address: e.addr.get_offset(),
            display_address: prog.output_code_offset(e.addr.get_offset()),
            name: e.name.clone(),
            header: String::new(),
        })
        .collect();
    labels.sort_by(|a, b| {
        normalize(arm, a.byte_address)
            .cmp(&normalize(arm, b.byte_address))
            .then_with(|| a.name.cmp(&b.name))
    });
    labels
}

/// A resumable linear disassembly of every CODE section.
///
/// [`Self::step`] walks at most a byte budget of code and writes whole lines to
/// the sink; [`Self::is_done`] reports when every section has been swept.  The
/// scratch state (an in-progress `db` run above all) is carried across steps, so
/// the byte stream a bounded walk produces is the one an unbounded walk
/// produces.
pub struct AsmSweep {
    header: String,
    labels: BTreeMap<u64, Vec<AsmLabel>>,
    code_secs: Vec<(u64, u64)>,
    section: usize,
    addr: u64,
    in_section: bool,
    scratch: AssemblyScratch,
    pending: String,
    done: bool,
}

impl AsmSweep {
    pub fn new(prog: &ConsoleProgram, file_name: &str, labels: Vec<AsmLabel>) -> Self {
        let arm = is_arm(prog);
        let mut map: BTreeMap<u64, Vec<AsmLabel>> = BTreeMap::new();
        for label in labels {
            map.entry(normalize(arm, label.byte_address)).or_default().push(label);
        }
        let mut code_secs: Vec<(u64, u64)> = prog
            .sections()
            .iter()
            .filter(|(_, _, flags)| flags & section_flags::CODE != 0)
            .map(|&(vma, size, _)| (vma, size))
            .collect();
        code_secs.sort_unstable();
        let done = code_secs.is_empty();
        Self {
            header: format!(
                "; kuna decompile-project — {file_name}\n; arch: {}\n",
                prog.description()
            ),
            labels: map,
            code_secs,
            section: 0,
            addr: 0,
            in_section: false,
            scratch: AssemblyScratch::new(),
            pending: String::new(),
            done,
        }
    }

    /// The two lines that open a `.asm`.  Not written by [`Self::step`]: a
    /// streamed export writes them when it creates the file, before the sweep
    /// starts.
    pub fn header_lines(&self) -> &str {
        &self.header
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Sweep at most `byte_budget` bytes of code, writing whole lines to `out`,
    /// and report whether the sweep is now complete.
    ///
    /// Everything this step produced is written to `out` before it returns
    /// (large steps flush as they go); `out` itself is never flushed, so a
    /// buffered sink stays the caller's to drive.  A step that writes nothing
    /// still costs a decode, so a zero budget is a no-op rather than an error.
    ///
    /// `prog` must be the program the sweep was built from.
    pub fn step(
        &mut self,
        prog: &ConsoleProgram,
        byte_budget: u64,
        out: &mut dyn Write,
    ) -> io::Result<bool> {
        if self.done {
            return Ok(true);
        }
        let mut remaining = byte_budget;
        self.pending.clear();
        while self.section < self.code_secs.len() {
            let (vma, size) = self.code_secs[self.section];
            let end = vma.saturating_add(size);
            if !self.in_section {
                let display_vma = prog.output_code_offset(vma);
                let display_end = prog.output_code_end_offset(end);
                self.pending.push_str(&format!(
                    "\n; --- code section 0x{display_vma:x}..0x{display_end:x} ---\n"
                ));
                self.addr = vma;
                self.in_section = true;
            }
            while self.addr < end && remaining > 0 {
                let consumed = sweep_one(
                    prog,
                    &self.labels,
                    self.addr,
                    end,
                    &mut self.scratch,
                    &mut self.pending,
                );
                self.addr = self.addr.saturating_add(consumed);
                remaining = remaining.saturating_sub(consumed);
                if self.pending.len() >= FLUSH_BYTES {
                    out.write_all(self.pending.as_bytes())?;
                    self.pending.clear();
                }
            }
            if self.addr >= end {
                self.scratch.flush_db(&mut self.pending);
                self.section += 1;
                self.in_section = false;
            }
            if remaining == 0 {
                break;
            }
        }
        self.done = self.section >= self.code_secs.len();
        out.write_all(self.pending.as_bytes())?;
        self.pending.clear();
        Ok(self.done)
    }
}

/// Sweep one unit at `addr`: the labels anchored there, then either the
/// instruction that decodes there or one raw byte.  Returns the bytes consumed.
fn sweep_one(
    prog: &ConsoleProgram,
    labels: &BTreeMap<u64, Vec<AsmLabel>>,
    addr: u64,
    end: u64,
    scratch: &mut AssemblyScratch,
    out: &mut String,
) -> u64 {
    if let Some(anchored) = labels.get(&addr) {
        scratch.flush_db(out);
        for label in anchored {
            out.push('\n');
            out.push_str(&format!("{}:  ; 0x{:x}\n", label.name, label.display_address));
            out.push_str(&label.header);
        }
    }
    // Never decode across the next label (a mis-sync would otherwise swallow a
    // function label into an instruction's byte extent) or the section end.
    let next_stop = labels.range(addr + 1..).next().map(|(&a, _)| a.min(end)).unwrap_or(end);
    match prog.disassemble_at_into(addr, &mut scratch.mnem, &mut scratch.body) {
        Ok(len) if len > 0 && addr + len as u64 <= next_stop => {
            scratch.flush_db(out);
            prog.read_bytes_into(addr, len as usize, &mut scratch.raw);
            scratch.emit_instruction(prog.output_code_offset(addr), out);
            len as u64
        }
        _ => {
            // Decode failure (or a decode that would cross the next label): one
            // raw byte, coalesced into up-to-8-byte `db` lines.
            if prog.read_bytes_into(addr, 1, &mut scratch.raw) {
                scratch.push_db(prog.output_code_offset(addr), scratch.raw[0], out);
            } else {
                scratch.flush_db(out);
                scratch.emit_unreadable(prog.output_code_offset(addr), out);
            }
            1
        }
    }
}

/// `; --- variables ---`: the per-function variable comment blocks a non-stream
/// export prints under each `.asm` label, collected into one address-ordered
/// section.
///
/// A streamed `.asm` is swept before the decompile finishes, so the blocks
/// cannot go under the labels; appending them keeps every offset a reader
/// already took valid.  The section marker is unconditional — it is where a
/// reader splits the sweep from the tails.
pub fn render_variables_section(results: &[FuncResult]) -> String {
    let mut ordered: Vec<&FuncResult> =
        results.iter().filter(|r| !r.variables.is_empty()).collect();
    ordered.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));
    let mut out = String::from("\n; --- variables ---\n");
    for r in ordered {
        out.push('\n');
        out.push_str(&format!("; {}:  ; 0x{:x}\n", r.name, r.address));
        out.push_str(&project::var_header_lines(r));
    }
    out
}

/// `; --- data ---`: the `.asm` data tail, identical to the one
/// [`crate::project::build_asm`] appends.
pub fn render_data_tail(prog: &ConsoleProgram, dat_addrs: &BTreeSet<u64>) -> String {
    let mut out = String::new();
    project::emit_data_tail(prog, &prog.sections(), dat_addrs, &mut out);
    out
}

/// On ARM-family specs a Thumb function's entry VMA carries the mode bit; the
/// sweep walks even byte addresses, so labels are keyed on `vma & !1`.
fn is_arm(prog: &ConsoleProgram) -> bool {
    prog.description().starts_with("ARM")
}

fn normalize(arm: bool, vma: u64) -> u64 {
    if arm {
        vma & !1
    } else {
        vma
    }
}
