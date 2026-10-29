//! `decompile-project --progress`: the non-stream export, reported on.
//!
//! [`decompile_project`] runs unchanged — the same serial or pooled decompile,
//! the same merge and structure naming, the same four artifacts written at the
//! end — so the folder it leaves is the non-stream export's plus `index.jsonl`,
//! and exactly as reproducible under `--jobs N` as that export is.  What this
//! module adds is the report while it runs: `.streaming` and the running README
//! on one clock, and `index.jsonl` as a per-function feed in completion order,
//! replaced at the end by the address-ordered index of the final `.c`.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use kuna_console::engine::ConsoleProgram;
use kuna_console::project::{render_c, AsmPhase, FuncResult, ReadmeFacts, StreamPhase};

use super::{
    create, index_line, write_atomic, Layout, StreamRun, CLOCK_GRANULARITY, INDEX_FILE,
    README_EVERY, STATUS_FILE, TICK,
};
use crate::decompile_all::{load_program, Args, DriverDefaults};
use crate::decompile_project::{decompile_project, ExportWatch, ProjectCompletion};
use crate::jobs::PoolWatch;

/// The export, watched; `.streaming` is removed once every artifact is final.
pub(super) fn export(
    args: &Args,
    output: Option<&str>,
    layout: &Layout,
    run: &StreamRun,
) -> Result<ProjectCompletion, String> {
    let watch = ProgressWatch { run, layout, readme_gate: Mutex::new(()), feed: Mutex::new(Feed::default()) };
    let completion = watch.with_clock(|| decompile_project(args, output, Some(&watch)))?;
    let status = run.dir.join(STATUS_FILE);
    std::fs::remove_file(&status).map_err(|e| format!("cannot remove {}: {e}", status.display()))?;
    Ok(completion)
}

/// The live `index.jsonl`: one line per first result, in the order they land.
#[derive(Default)]
struct Feed {
    file: Option<File>,
    seq: usize,
}

struct ProgressWatch<'a> {
    run: &'a StreamRun,
    layout: &'a Layout,
    /// Held across a running-README publish and across the move to
    /// `finalizing`, so no running README lands on top of the final one.
    readme_gate: Mutex<()>,
    feed: Mutex<Feed>,
}

impl ProgressWatch<'_> {
    /// Run `body` while `.streaming` ticks, and the README with it until the
    /// export starts finalizing.  The clock is stopped and joined before this
    /// returns, so nothing rewrites `.streaming` after the caller removes it.
    fn with_clock<T>(&self, body: impl FnOnce() -> T) -> T {
        struct Stopper<'a>(&'a AtomicBool);
        impl Drop for Stopper<'_> {
            fn drop(&mut self) {
                self.0.store(false, Ordering::SeqCst);
            }
        }

        let ticking = AtomicBool::new(true);
        std::thread::scope(|scope| {
            scope.spawn(|| {
                let mut last_status = Instant::now();
                let mut last_readme = Instant::now();
                let mut last_readme_done = None;
                while ticking.load(Ordering::SeqCst) {
                    std::thread::sleep(CLOCK_GRANULARITY);
                    if !ticking.load(Ordering::SeqCst) || last_status.elapsed() < TICK {
                        continue;
                    }
                    let _gate = self.readme_gate.lock().unwrap_or_else(|e| e.into_inner());
                    let (phase, done) = {
                        let shared = self.run.lock();
                        (shared.status.phase, shared.status.done)
                    };
                    let readme = matches!(phase, StreamPhase::Loading | StreamPhase::Decompiling)
                        && last_readme.elapsed() >= README_EVERY
                        && (phase == StreamPhase::Loading || last_readme_done != Some(done));
                    self.run.publish_progress(readme);
                    last_status = Instant::now();
                    if readme {
                        last_readme = Instant::now();
                        last_readme_done = Some(done);
                    }
                }
            });
            let _stop = Stopper(&ticking);
            body()
        })
    }

    /// Clear the previous export's artifacts and open the feed: from here the
    /// folder is this run's, as it is once a `--stream` run truncates.
    fn open_feed(&self) -> Result<(), String> {
        let name = &self.layout.file_name;
        self.run.artifacts.store(true, Ordering::SeqCst);
        for artifact in [format!("{name}.c"), format!("{name}.h"), format!("{name}.asm")] {
            let path = self.run.dir.join(artifact);
            match std::fs::remove_file(&path) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(format!("cannot remove {}: {e}", path.display())),
            }
        }
        let file = create(&self.run.dir.join(INDEX_FILE))?;
        *self.feed.lock().unwrap_or_else(|e| e.into_inner()) = Feed { file: Some(file), seq: 0 };
        Ok(())
    }

    /// Append the feed lines.  The feed reports on the export rather than
    /// being it — the final index is rewritten whole from the results — so a
    /// failed append warns once and ends the feed instead of the run.
    fn append_feed(&self, results: &[FuncResult]) {
        let mut feed = self.feed.lock().unwrap_or_else(|e| e.into_inner());
        let Feed { file, seq } = &mut *feed;
        let Some(out) = file.as_mut() else { return };
        let lines: String = results
            .iter()
            .enumerate()
            .map(|(k, r)| index_line(*seq + k, r, None))
            .collect();
        *seq += results.len();
        if let Err(e) = out.write_all(lines.as_bytes()) {
            eprintln!(
                "[kuna --stream] warning: cannot append to {INDEX_FILE}: {e}; the feed stops \
                 here and the final index is still written at the end"
            );
            *file = None;
        }
    }

    /// Every function is in.  The README stops here: the export writes the
    /// final one.
    fn enter_finalizing(&self) {
        let _gate = self.readme_gate.lock().unwrap_or_else(|e| e.into_inner());
        self.run.lock().status.phase = StreamPhase::Finalizing;
    }
}

impl PoolWatch for ProgressWatch<'_> {
    fn workers(&self, n: usize) {
        self.run.workers.store(n, Ordering::SeqCst);
    }

    fn landed(&self, results: &[FuncResult]) {
        if results.is_empty() {
            return;
        }
        self.append_feed(results);
        let all_in = {
            let mut shared = self.run.lock();
            let status = &mut shared.status;
            status.done += results.len();
            status.failed += results.iter().filter(|r| r.error.is_some()).count();
            status.last_result = Instant::now();
            status.total.is_some_and(|total| status.done >= total)
        };
        if all_in {
            self.enter_finalizing();
        }
    }
}

impl ExportWatch for ProgressWatch<'_> {
    fn load(&self, args: &Args) -> Result<ConsoleProgram, String> {
        load_program(args, DriverDefaults::Decompile)
    }

    fn decompiling(&self, prog: &ConsoleProgram, total: usize) -> Result<(), String> {
        let facts = ReadmeFacts::snapshot(
            &self.layout.binary_path,
            &self.layout.path_label,
            &self.layout.file_name,
            prog,
        );
        self.open_feed()?;
        let _gate = self.readme_gate.lock().unwrap_or_else(|e| e.into_inner());
        {
            let mut shared = self.run.lock();
            shared.facts = facts;
            shared.status.phase = StreamPhase::Decompiling;
            shared.status.total = Some(total);
            shared.status.last_result = Instant::now();
        }
        self.run.publish_progress(true);
        Ok(())
    }

    fn finalizing(&self) {
        self.enter_finalizing();
        self.run.set_asm_phase(AsmPhase::Sweeping);
        self.run.publish_progress(false);
    }

    fn written(
        &self,
        out_dir: &Path,
        file_name: &str,
        results: &[FuncResult],
    ) -> Result<Vec<(String, usize)>, String> {
        self.feed.lock().unwrap_or_else(|e| e.into_inner()).file = None;
        let mut offset = format!("#include \"{file_name}.h\"\n\n").len() as u64;
        let mut index = String::new();
        for (seq, r) in results.iter().enumerate() {
            let len = render_c(std::slice::from_ref(r)).len();
            index.push_str(&index_line(seq, r, Some((offset, len))));
            offset += len as u64;
        }
        write_atomic(&out_dir.join(INDEX_FILE), index.as_bytes())?;
        {
            let mut shared = self.run.lock();
            shared.status.c_bytes = offset;
            shared.status.asm = AsmPhase::Complete;
        }
        Ok(vec![(INDEX_FILE.to_string(), index.len())])
    }
}
