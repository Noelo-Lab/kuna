//! C++ `testfunction.cc` / `testfunction.hh`: the XML data-driven single-function
//! datatest runner.
//!
//! Ports the three C++ types that execute the 83 datatest files
//! (`tests/datatests/`, 675 `<stringmatch>` assertions):
//!
//! - [`FunctionTestProperty`] — one `<stringmatch min=.. max=..>` assertion: a
//!   (possibly multi-line) regular expression run over the lines of decompiler
//!   output, with the C++ min/max match counting and the sequential multi-line
//!   matching state machine ([`FunctionTestProperty::process_line`]).  Per
//!   LOSS-007 the `std::regex` (ECMAScript) is substituted by the `regex` crate;
//!   the corpus uses no backreferences/lookaround, so the match decisions agree.
//! - [`FunctionTestCollection`] — a `<decompilertest>` file: parse its
//!   `<binaryimage>`, `<script>`, and `<stringmatch>` children, build the
//!   program, run the console script, slice the bulk output into lines, feed
//!   every line to every property, and emit the EXACT `Success -- <name>` /
//!   `FAIL -- <name>` grammar.
//! - [`run_test_files`] — the `FunctionTestCollection::runTestFiles` driver: run a
//!   list of files, accumulate `Total tests applied`/`Total passing tests`, and
//!   print the capped `Failures:` summary, with the `Error parsing|executing
//!   <f>.xml` / `No output for <f>.xml` / `Execution failed for <f>.xml` strings
//!   the Python harness (`kuna/run_tests.py`) greps off **stdout**.
//!
//! ## The console wiring (the merged-tree reality)
//!
//! The C++ `ConsoleCommands` console constructor calls
//! `IfaceCapability::registerAllCommands(this)`; the Rust equivalent is
//! [`register_decomp_commands`](kuna_console::ifacedecomp::register_decomp_commands)
//! over a [`ConsoleCommands`](kuna_console::ifaceterm::ConsoleCommands)-backed
//! [`IfaceStatus`].  `runTests` swaps the console output streams: command-line
//! chatter to a discarded `midBuffer` (`optr`), the bulk `print C` output to a
//! `bulkout` (`fileoptr`); after the run the captured streams are read back.
//! This port reproduces that swap against the buffer-backed `IfaceStatus`
//! (`optr: String`, `fileoptr: Option<FileOut>`).
//!
//! The decompiler *engine* commands themselves (`load function`, `decompile`,
//! `print C`, …) are stubs in the merged console crate (each returns
//! `engine_unavailable`; the `Architecture` integration layer is a later
//! W-item).  So an end-to-end datatest drives the script until the first engine
//! command fails, the console enters its error state (`setErrorIsDone(true)`),
//! and `runTests` reports `Error: Did not apply tests in <f>` — the byte-faithful
//! console behavior for that engine state.  The framework's own
//! parse/count/output grammar is exercised fully and independently by the
//! [`FunctionTestProperty`] unit tests below.

use std::rc::Rc;

use regex::Regex;

use kuna_base::types::{int4, uint4};
use kuna_base::xml::{DocumentStorage, Element};

use kuna_console::engine::bootstrap_program;
use kuna_console::ifacedecomp::{
    mainloop, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::{FileOut, IfaceError, IfaceStatus};
use kuna_console::kuna_console::register_kuna_commands;

/// C++ `IfaceParseError` / `IfaceExecutionError` distinction at the file level:
/// the two `catch` arms of `runTestFiles` produce different prefixes
/// (`Error parsing` vs `Error executing`).  A load/parse failure is `Parse`; a
/// `buildProgram` / architecture-init failure is `Execution`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestError {
    /// C++ `IfaceParseError`: a malformed/unrecognized test file (the `catch
    /// (IfaceParseError &)` arm → `Error parsing <f>: <msg>`).
    Parse(String),
    /// C++ `IfaceExecutionError`: program build / architecture init failure (the
    /// `catch (IfaceExecutionError &)` arm → `Error executing <f>: <msg>`).
    Execution(String),
}

impl TestError {
    /// The bare message (C++ `err.explain`).
    pub fn message(&self) -> &str {
        match self {
            TestError::Parse(m) | TestError::Execution(m) => m,
        }
    }
}

impl From<IfaceError> for TestError {
    fn from(e: IfaceError) -> Self {
        if e.is_execution() {
            TestError::Execution(e.explain)
        } else {
            TestError::Parse(e.explain)
        }
    }
}

type TestResult<T> = Result<T, TestError>;

// ===========================================================================
// FunctionTestProperty — one <stringmatch> assertion.
// ===========================================================================

/// C++ `FunctionTestProperty` (`testfunction.hh:35`): a single property searched
/// for in the decompiler output.
///
/// A property is a list of line-regexes (one per `\n`-separated line in the
/// pattern).  It "matches" once when all of its line-regexes match consecutive
/// output lines in order; `count` records how many such full matches were seen,
/// and the test passes iff `count` is within `[minimumMatch, maximumMatch]`.
pub struct FunctionTestProperty {
    /// C++ `minimumMatch`: minimum times the property is expected to match.
    minimum_match: int4,
    /// C++ `maximumMatch`: maximum times the property is expected to match.
    maximum_match: int4,
    /// C++ `name`: printed in test summaries.
    name: String,
    /// C++ `pattern`: the per-line regexes (multi-line patterns split on `\n`).
    pattern: Vec<Regex>,
    /// C++ `patnum` (mutable): index of the current line-regex to match against.
    patnum: uint4,
    /// C++ `count` (mutable): number of full matches seen.
    count: uint4,
}

impl FunctionTestProperty {
    /// C++ `getName`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// C++ `startTest`: reset the running state before scanning a new output.
    pub fn start_test(&mut self) {
        self.count = 0;
        self.patnum = 0;
    }

    /// C++ `processLine(const string &line)`: search `line`, advancing the
    /// multi-line match state.
    ///
    /// Faithful transcription of the state machine: on a match of the current
    /// line-regex advance `patnum`, and when the whole pattern is consumed bump
    /// `count` and rewind; on a non-match *mid-pattern* abort the partial match
    /// and (C++ quirk, reproduced) immediately re-test this same line against the
    /// first line-regex so a fresh multi-line match can begin on it.
    pub fn process_line(&mut self, line: &str) {
        if self.pattern[self.patnum as usize].is_match(line) {
            self.patnum += 1;
            if self.patnum as usize >= self.pattern.len() {
                self.count += 1; // Full pattern has matched.  Count it.
                self.patnum = 0;
            }
        } else if self.patnum > 0 {
            self.patnum = 0; // Abort current multi-line match, restart trying to match first line
            if self.pattern[self.patnum as usize].is_match(line) {
                self.patnum += 1;
            }
        }
    }

    /// C++ `endTest`: the property passes iff the match count is in range.
    pub fn end_test(&self) -> bool {
        self.count >= self.minimum_match as uint4 && self.count <= self.maximum_match as uint4
    }

    /// C++ `restoreXml(const Element *el)`: reconstruct from a `<stringmatch>`
    /// tag (`name`, `min`, `max`, and the `\n`-split multi-line content).
    ///
    /// The content is split into per-line regexes exactly as C++ does: leading
    /// spaces/tabs are stripped from each line, a `\n` starts a new line-regex,
    /// and an all-whitespace tail produces no extra (empty) pattern.  Per
    /// LOSS-007 each line is compiled with the `regex` crate instead of
    /// `std::regex`.
    pub fn restore_xml(el: &Element) -> TestResult<FunctionTestProperty> {
        let name = attr_string(el, "name")?;
        // C++ `istringstream s1(min) >> minimumMatch;` — leading-ws-skipping
        // signed-int extraction; a non-numeric value leaves the int unset (C++
        // leaves the field's indeterminate value).  The corpus values are plain
        // decimal, so a strict parse with a 0 fallback is faithful in practice.
        let minimum_match = parse_int_attr(el, "min")?;
        let maximum_match = parse_int_attr(el, "max")?;

        // C++ reads `el->getContent()`, a byte string; the datatest patterns are
        // ASCII, so a lossy decode is exact for the corpus.
        let content = String::from_utf8_lossy(el.get_content()).into_owned();
        let line: &[u8] = content.as_bytes();

        let mut pattern: Vec<Regex> = Vec::new();
        // Faithful transcription of the C++ pos/nextpos walk over `line`.
        let mut pos: usize = 0;
        loop {
            // Remove whitespace at front of pattern.
            while pos < line.len() && (line[pos] == b' ' || line[pos] == b'\t') {
                pos += 1;
            }
            if pos >= line.len() {
                break;
            }
            // A newline in the pattern indicates a multi-line regex.
            let nextpos = memchr_nl(line, pos);
            let (n, next): (Option<usize>, Option<usize>) = match nextpos {
                None => (None, None),              // take all remaining chars
                Some(nl) => (Some(nl - pos), Some(nl + 1)), // upto newline, skip it
            };
            let end = match n {
                None => line.len(),
                Some(len) => pos + len,
            };
            let piece = &content[pos..end];
            pattern.push(compile_pattern(piece)?);
            match next {
                None => break, // C++ `pos = string::npos` terminates the do/while
                Some(np) => pos = np,
            }
        }

        Ok(FunctionTestProperty {
            minimum_match,
            maximum_match,
            name,
            pattern,
            patnum: 0,
            count: 0,
        })
    }

    /// Test-only constructor: a single-line property over a raw regex string.
    #[cfg(test)]
    fn from_parts(name: &str, min: int4, max: int4, raw_lines: &[&str]) -> FunctionTestProperty {
        let pattern = raw_lines.iter().map(|p| Regex::new(p).unwrap()).collect();
        FunctionTestProperty {
            minimum_match: min,
            maximum_match: max,
            name: name.to_string(),
            pattern,
            patnum: 0,
            count: 0,
        }
    }
}

/// Find the next `\n` at or after `from` (C++ `line.find('\n', pos)`).
fn memchr_nl(bytes: &[u8], from: usize) -> Option<usize> {
    bytes[from..].iter().position(|&b| b == b'\n').map(|i| from + i)
}

/// Translate the one ECMAScript-vs-`regex`-crate dialect divergence that matters
/// for the corpus: the identity escapes `\<` and `\>`.  C++ `std::regex`
/// (ECMAScript) has no word-boundary `\<`/`\>` syntax, so per the ECMAScript
/// `IdentityEscape` production `\<` and `\>` match the *literal* `<`/`>`.  The
/// Rust `regex` crate instead reads `\<`/`\>` as ASCII start-/end-of-word
/// boundary assertions (added in regex 1.x), so a pattern like `ptr-\>a`
/// compiles but can never match `ptr->a` — a silent mis-decision suppressing
/// every `->`/`<`-bearing datatest assertion.  Faithfully restoring the C++
/// semantics, we lower `\<`→`<` and `\>`→`>` (every other escape — `\+ \. \( \[
/// \* \\` … — is identical in both dialects and is preserved verbatim, including
/// an escaped backslash `\\` so `\\>` is left intact).
fn ecmascript_identity_escapes(p: &str) -> String {
    let mut out = String::with_capacity(p.len());
    let mut chars = p.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('<') | Some('>') => {
                    out.push(chars.next().unwrap()); // drop the backslash, keep < / >
                }
                Some('\\') => {
                    out.push('\\'); // escaped backslash: consume the pair verbatim
                    out.push(chars.next().unwrap());
                }
                _ => out.push('\\'), // any other escape: leave the backslash, char emitted next loop
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Compile one line of a `<stringmatch>` pattern (LOSS-007: `regex` crate in
/// place of `std::regex`).  The pattern is first lowered through
/// [`ecmascript_identity_escapes`] to recover C++ `std::regex` `\<`/`\>`
/// semantics.  A pattern the `regex` crate rejects (a remaining dialect gap) is
/// surfaced as a `Parse` test error naming the offending pattern, so a future
/// unsupported pattern is loud rather than silently mis-decided.
fn compile_pattern(p: &str) -> TestResult<Regex> {
    let translated = ecmascript_identity_escapes(p);
    Regex::new(&translated).map_err(|e| {
        TestError::Parse(format!("stringmatch pattern {p:?} not supported by the regex crate: {e}"))
    })
}

// ===========================================================================
// FunctionTestCollection — one <decompilertest> file.
// ===========================================================================

/// C++ `FunctionTestCollection` (`testfunction.hh:70`): the tests around a single
/// program/function, loaded from one XML file and run through the console.
///
/// Owns the [`IfaceStatus`] console (C++ `consoleOwner = true`: the
/// `FunctionTestCollection(ostream&)` constructor allocates a `ConsoleCommands`).
/// The C++ object also keeps an `IfaceDecompData *dcp` aliasing the console's
/// module data; here the data lives inside the `IfaceStatus` datamap and is
/// reached through it, so a separate `dcp` field is unnecessary.
pub struct FunctionTestCollection {
    /// C++ `fileName`: the file the current tests came from.
    file_name: String,
    /// C++ `testList`: the property assertions for the current file.
    test_list: Vec<FunctionTestProperty>,
    /// C++ `commands`: the console script for the current file.
    commands: Vec<String>,
    /// C++ `console`: the decompiler console.  Rebuilt per file so the script
    /// feed is fresh (the faithful observable equivalent of the C++
    /// `commands`-by-reference + `console->reset()`; see module docs).
    console: IfaceStatus,
    /// C++ `numTestsApplied` (mutable): count of tests executed.
    num_tests_applied: int4,
    /// C++ `numTestsSucceeded` (mutable): count of tests that passed.
    num_tests_succeeded: int4,
    /// Whether `<binaryimage>` produced a usable program (C++ `buildProgram`
    /// sets `dcp->conf`).
    saw_program: bool,
    /// The SLEIGH spec search roots (C++ `SleighArchitecture::specpaths`).
    /// `build_program` resolves the architecture against these; the binary
    /// supplies them (from `-sleighpath`/`SLEIGHHOME`).
    spec_roots: Vec<String>,
}

impl FunctionTestCollection {
    /// C++ `FunctionTestCollection(ostream &s)`: allocate an owned console
    /// (`ConsoleCommands`) and register the full command set; set
    /// `errorisdone` so the first failed script command aborts the run.
    pub fn new() -> FunctionTestCollection {
        Self::with_spec_roots(Vec::new())
    }

    /// Like [`new`](Self::new) but with the SLEIGH spec search roots the
    /// datatest's `<binaryimage>` architecture is resolved against (C++
    /// `SleighArchitecture::specpaths`).
    pub fn with_spec_roots(spec_roots: Vec<String>) -> FunctionTestCollection {
        let console = build_console();
        FunctionTestCollection {
            file_name: String::new(),
            test_list: Vec::new(),
            commands: Vec::new(),
            console,
            num_tests_applied: 0,
            num_tests_succeeded: 0,
            saw_program: false,
            spec_roots,
        }
    }

    /// C++ `getTestsApplied`.
    pub fn get_tests_applied(&self) -> int4 {
        self.num_tests_applied
    }

    /// C++ `getTestsSucceeded`.
    pub fn get_tests_succeeded(&self) -> int4 {
        self.num_tests_succeeded
    }

    /// C++ `numCommands`.
    pub fn num_commands(&self) -> int4 {
        self.commands.len() as int4
    }

    /// C++ `getCommand(i)`.
    pub fn get_command(&self, i: int4) -> &str {
        &self.commands[i as usize]
    }

    /// C++ `clear()`: drop any previous architecture/function and reset the
    /// console for a new program.
    pub fn clear(&mut self) {
        // dcp->clearArchitecture()
        if let Some(dcp) = self.dcp_mut() {
            dcp.clear_architecture();
        }
        self.commands.clear();
        self.test_list.clear();
        self.saw_program = false;
        self.console.reset();
    }

    /// Reach the console's shared [`IfaceDecompData`] (C++ `dcp`).
    fn dcp_mut(&mut self) -> Option<&mut IfaceDecompData> {
        self.console
            .get_data_mut(DECOMPILE_MODULE)
            .and_then(|d| d.as_any_mut().downcast_mut::<IfaceDecompData>())
    }

    /// C++ `stripNewlines(const string &ref)`: drop `\r`, turn `\n` into a
    /// space.  Operates on the raw bytes (the script content is ASCII).
    fn strip_newlines(reference: &[u8]) -> String {
        let mut res = String::with_capacity(reference.len());
        for &c in reference {
            if c == b'\r' {
                continue; // Remove carriage return
            }
            let c = if c == b'\n' { b' ' } else { c }; // Convert newline to space
            res.push(c as char);
        }
        res
    }

    /// C++ `restoreXmlCommands(const Element *el)`: the `<com>` children of
    /// `<script>` become the command list (each stripped of newlines).
    fn restore_xml_commands(&mut self, el: &Element) {
        for subel in el.get_children() {
            self.commands.push(Self::strip_newlines(subel.get_content()));
        }
    }

    /// C++ `buildProgram(DocumentStorage &docStorage)`: instantiate the program
    /// from the `<binaryimage>` tag.
    ///
    /// The C++ path is `ArchitectureCapability::getCapability("xml")` →
    /// `buildArchitecture` → `conf->init(store)` → `readLoaderSymbols`.  The kuna
    /// Rust equivalent ([`bootstrap_from_root`]) builds the same engine assembly
    /// (the `w9x-arch-engine-glue` `Architecture::init_post_engine` chain) and
    /// installs it on the console's shared data (`dcp->conf`), so the script's
    /// engine commands (`load function`/`decompile`/`print C`) drive a real
    /// `Architecture`.  An init failure raises the C++ `IfaceExecutionError`
    /// shape, so the file-level `Error executing <f>` path stays byte-faithful.
    fn build_program(&mut self, bi: &Rc<Element>) -> TestResult<()> {
        // C++ requires the "arch" attribute (the loader reads it); a missing one
        // is the init failure the `catch(LowlevelError &)` arm would surface.
        let arch_id = bi.get_attribute_value("arch").map_err(|e| {
            TestError::Execution(format!("Error during architecture initialization: {e}"))
        })?;
        let arch_id = String::from_utf8_lossy(arch_id).into_owned();
        // When no SLEIGH spec roots are configured (the framework-only path the
        // `FunctionTestProperty`/grammar unit tests drive — `echo`-fed scripts
        // that never touch the engine), skip the engine bootstrap: the program
        // stays unbuilt (`dcp->conf` null) so an engine command errors with the
        // same "No image loaded" the merged-tree stubs raised, but the framework
        // (stringmatch matching, Success/FAIL grammar) is exercised fully.  A
        // real run (the `decomp_test_dbg` binary supplies spec roots) bootstraps
        // the engine.
        if self.spec_roots.is_empty() {
            self.saw_program = true;
            return Ok(());
        }
        // capa->buildArchitecture + conf->init(store) + readLoaderSymbols.
        let prog = bootstrap_program(Rc::clone(bi), &arch_id, &self.spec_roots).map_err(|e| {
            TestError::Execution(format!("Error during architecture initialization: {e}"))
        })?;
        if let Some(dcp) = self.dcp_mut() {
            dcp.conf = Some(prog);
        }
        self.saw_program = true;
        Ok(())
    }

    /// C++ `loadTest(const string &filename)`: open the XML, dispatch on the
    /// root tag (`decompilertest` vs the unsupported `binaryimage` old form).
    pub fn load_test(&mut self, filename: &str) -> TestResult<()> {
        self.file_name = filename.to_string();
        let mut store = DocumentStorage::new();
        let doc = store
            .open_document(filename)
            .map_err(|e| TestError::Parse(e.explain().to_string()))?;
        let root = doc.get_root().clone();
        match root.get_name() {
            "decompilertest" => self.restore_xml(&mut store, &root),
            "binaryimage" => self.restore_xml_old_form(&root),
            other => Err(TestError::Parse(format!(
                "Test file {filename} has unrecognized XML tag: {other}"
            ))),
        }
    }

    /// C++ `restoreXml(DocumentStorage &store, const Element *el)`: walk the
    /// `<decompilertest>` children — `<script>`, `<stringmatch>`,
    /// `<binaryimage>` — enforcing that all three kinds appeared.
    pub fn restore_xml(&mut self, store: &mut DocumentStorage, el: &Rc<Element>) -> TestResult<()> {
        let mut saw_script = false;
        let mut saw_tests = false;
        let mut saw_program = false;
        for subel in el.get_children() {
            match subel.get_name() {
                "script" => {
                    saw_script = true;
                    self.restore_xml_commands(subel);
                }
                "stringmatch" => {
                    saw_tests = true;
                    self.test_list.push(FunctionTestProperty::restore_xml(subel)?);
                }
                "binaryimage" => {
                    saw_program = true;
                    store.register_tag(subel);
                    self.build_program(subel)?;
                }
                other => {
                    return Err(TestError::Parse(format!(
                        "Unknown tag in <decompilertest>: {other}"
                    )))
                }
            }
        }
        if !saw_script {
            return Err(TestError::Parse(
                "Did not see <script> tag in <decompilertest>".to_string(),
            ));
        }
        if !saw_tests {
            return Err(TestError::Parse(
                "Did not see any <stringmatch> tags in <decompilertest>".to_string(),
            ));
        }
        if !saw_program {
            return Err(TestError::Parse(
                "No <binaryimage> tag in <decompilertest>".to_string(),
            ));
        }
        Ok(())
    }

    /// C++ `restoreXmlOldForm`: the legacy binaryimage-comment format, which the
    /// upstream throws on.
    pub fn restore_xml_old_form(&mut self, _el: &Rc<Element>) -> TestResult<()> {
        Err(TestError::Parse("Old format test not supported".to_string()))
    }

    /// C++ `startTests()`: initialize each property.
    fn start_tests(&mut self) {
        for t in &mut self.test_list {
            t.start_test();
        }
    }

    /// C++ `passLineToTests(const string &line)`: every property processes the
    /// line.
    fn pass_line_to_tests(&mut self, line: &str) {
        for t in &mut self.test_list {
            t.process_line(line);
        }
    }

    /// C++ `evaluateTests(list<string> &lateStream)`: the final per-property
    /// verdict, printed `Success -- <name>` / `FAIL -- <name>` to the console
    /// output (`optr`, restored to the file-results stream), with failures
    /// collected for the summary.
    fn evaluate_tests(&mut self, late_stream: &mut Vec<String>) {
        // Borrow the test list and the console output separately.
        for t in &self.test_list {
            self.num_tests_applied += 1;
            if t.end_test() {
                self.console.optr.push_str("Success -- ");
                self.console.optr.push_str(t.get_name());
                self.console.optr.push('\n');
                self.num_tests_succeeded += 1;
            } else {
                self.console.optr.push_str("FAIL -- ");
                self.console.optr.push_str(t.get_name());
                self.console.optr.push('\n');
                late_stream.push(t.get_name().to_string());
            }
        }
    }

    /// C++ `runTests(list<string> &lateStream)`: run the script, slice the bulk
    /// output into lines, feed every line to every property, and emit the
    /// verdicts.  The console output produced here is appended to the
    /// file-results stream (the caller's `optr`); this returns it.
    ///
    /// Faithful to the C++ stream swap: command chatter to a discarded
    /// `midBuffer` (`optr`), bulk `print C` output to `bulkout` (`fileoptr`).
    /// On a console error (the first stubbed engine command), the
    /// `Error: Did not apply tests in <f>` path runs; on empty bulk output the
    /// `No output for <f>` path runs.  Returns the file-results text.
    pub fn run_tests(&mut self, late_stream: &mut Vec<String>) -> String {
        self.num_tests_applied = 0;
        self.num_tests_succeeded = 0;

        // Re-seat the console's command feed for this file (the C++
        // `commands`-by-reference equivalent), then rewind it.  The loaded
        // program (`dcp->conf`, set by `build_program` during `load_test`) is
        // carried across the console rebuild so the script's engine commands drive
        // it (the C++ console is the SAME object across loadTest/runTests; the
        // Rust rebuild for the fresh command feed must preserve the program).
        let carried_conf = self.take_conf();
        let mut console = build_console_with(self.commands.clone());
        if let (Some(prog), Some(dcp)) = (carried_conf, console_dcp_mut(&mut console)) {
            dcp.conf = Some(prog);
        }
        self.console = console;

        // C++: console->optr = &midBuffer (discarded); console->fileoptr = &bulkout.
        self.console.optr = String::new(); // midBuffer
        self.console.fileoptr = Some(FileOut::default()); // bulkout
        mainloop(&mut self.console);
        // C++ restores console->optr = origStream; fileoptr = origStream.
        let mid_buffer = std::mem::take(&mut self.console.optr);
        let bulkout = self.console.close_file_redirect().map(|f| f.contents).unwrap_or_default();

        // KUNA_DUMP triage hook (the C++ (kuna) addition), even on error.
        let mut results = String::new();
        if std::env::var_os("KUNA_DUMP").is_some() {
            results.push_str(&format!("===KUNA_DUMP_BEGIN {}===\n", self.file_name));
            results.push_str(&bulkout);
            results.push_str("\n===KUNA_DUMP_MID===\n");
            results.push_str(&mid_buffer);
            results.push_str("===KUNA_DUMP_END===\n");
        }

        if self.console.is_in_error() {
            results.push_str(&format!("Error: Did not apply tests in {}\n", self.file_name));
            results.push_str(&mid_buffer);
            results.push('\n');
            late_stream.push(format!("Execution failed for {}", self.file_name));
            return results;
        }
        if bulkout.is_empty() {
            late_stream.push(format!("No output for {}", self.file_name));
            return results;
        }

        // Slice the bulk output into lines (C++ find_first_of('\n') walk, which
        // does NOT keep the trailing newline and handles a final no-newline line).
        self.start_tests();
        let mut prevpos = 0usize;
        let bytes = bulkout.as_bytes();
        while let Some(rel) = bytes[prevpos..].iter().position(|&b| b == b'\n') {
            let pos = prevpos + rel;
            // `line` borrows the LOCAL `bulkout`, never `self`, so feeding it to
            // the `&mut self` property pass is conflict-free.
            self.pass_line_to_tests(&bulkout[prevpos..pos]);
            prevpos = pos + 1;
        }
        if prevpos != bulkout.len() {
            // Process the final line without a trailing newline char.
            self.pass_line_to_tests(&bulkout[prevpos..]);
        }

        // Verdicts go to the (restored) results stream.
        let saved = std::mem::take(&mut self.console.optr);
        self.console.optr = results;
        self.evaluate_tests(late_stream);
        let out = std::mem::take(&mut self.console.optr);
        self.console.optr = saved;
        out
    }
}

impl FunctionTestCollection {
    /// Take the loaded program out of the current console's shared data (so it can
    /// be carried across a console rebuild).
    fn take_conf(&mut self) -> Option<kuna_console::engine::ConsoleProgram> {
        self.dcp_mut().and_then(|dcp| dcp.conf.take())
    }
}

impl Default for FunctionTestCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a fresh datatest console (empty feed): register the decompiler + kuna
/// command modules and set `errorisdone` (C++ `ConsoleCommands` ctor +
/// `setErrorIsDone(true)`).
fn build_console() -> IfaceStatus {
    build_console_with(Vec::new())
}

/// Build a datatest console seeded with `commands` as its feed (C++
/// `ConsoleCommands(s, comms)` + `IfaceCapability::registerAllCommands`).
fn build_console_with(commands: Vec<String>) -> IfaceStatus {
    let mut console = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut console);
    register_kuna_commands(&mut console);
    console.set_error_is_done(true);
    console
}

/// Reach a console's shared [`IfaceDecompData`] (C++ `dcp`).
fn console_dcp_mut(console: &mut IfaceStatus) -> Option<&mut IfaceDecompData> {
    console
        .get_data_mut(DECOMPILE_MODULE)
        .and_then(|d| d.as_any_mut().downcast_mut::<IfaceDecompData>())
}

// ===========================================================================
// runTestFiles — the multi-file driver.
// ===========================================================================

/// C++ `FunctionTestCollection::runTestFiles(const vector<string> &testFiles,
/// ostream &s)`: run each listed file in turn, accumulate the totals, print the
/// per-file results and the capped `Failures:` summary, and return the number of
/// failed assertions (`applied - succeeded`).
///
/// The output text is written to `out` (the Python harness reads it off
/// **stdout**); the exact grammar — `Success -- `/`FAIL -- `, the
/// `Error parsing|executing <f>.xml` file-error lines at column 0, the
/// `Total tests applied = N` / `Total passing tests = N` footers, and the
/// 10-line-capped `Failures:` block — is byte-faithful.
pub fn run_test_files(test_files: &[String], out: &mut String) -> int4 {
    run_test_files_with_specs(test_files, &[], out)
}

/// Like [`run_test_files`] but with the SLEIGH spec search roots the
/// `<binaryimage>` architecture is resolved against (the `decomp_test_dbg`
/// `-sleighpath <dir>` / `SLEIGHHOME` value).  The plain [`run_test_files`]
/// forwards with no roots (the framework-only path the unit tests exercise).
pub fn run_test_files_with_specs(
    test_files: &[String],
    spec_roots: &[String],
    out: &mut String,
) -> int4 {
    let mut total_tests_applied = 0i32;
    let mut total_tests_succeeded = 0i32;
    let mut failures: Vec<String> = Vec::new();
    let mut collection = FunctionTestCollection::with_spec_roots(spec_roots.to_vec());

    for file in test_files {
        collection.clear();
        match collection.load_test(file) {
            Ok(()) => {
                let results = collection.run_tests(&mut failures);
                out.push_str(&results);
                total_tests_applied += collection.get_tests_applied();
                total_tests_succeeded += collection.get_tests_succeeded();
            }
            // C++ `catch(IfaceParseError &err)`: Error parsing <f>: <msg>
            Err(TestError::Parse(msg)) => {
                let fs = format!("Error parsing {file}: {msg}");
                out.push_str(&fs);
                out.push('\n');
                failures.push(fs);
            }
            // C++ `catch(IfaceExecutionError &err)`: Error executing <f>: <msg>
            Err(TestError::Execution(msg)) => {
                let fs = format!("Error executing {file}: {msg}");
                out.push_str(&fs);
                out.push('\n');
                failures.push(fs);
            }
        }
    }

    out.push('\n');
    out.push_str(&format!("Total tests applied = {total_tests_applied}\n"));
    out.push_str(&format!("Total passing tests = {total_tests_succeeded}\n"));
    out.push('\n');
    if !failures.is_empty() {
        out.push_str("Failures: \n");
        // C++ prints up to 10 failures (the `for(i=0;i<10;++i)` cap).
        for f in failures.iter().take(10) {
            out.push_str("  ");
            out.push_str(f);
            out.push('\n');
        }
    }
    total_tests_applied - total_tests_succeeded
}

// ---------------------------------------------------------------------------
// XML attribute helpers.
// ---------------------------------------------------------------------------

/// Read a required attribute as a `String` (lossy ASCII decode).
fn attr_string(el: &Element, name: &str) -> TestResult<String> {
    let v = el
        .get_attribute_value(name)
        .map_err(|e| TestError::Parse(e.explain().to_string()))?;
    Ok(String::from_utf8_lossy(v).into_owned())
}

/// C++ `istringstream(attr) >> int`: a leading-ws-skipping signed-int parse of
/// the `min`/`max` attributes.  The corpus values are plain decimal; a value
/// that does not parse yields 0 (the practical equivalent of the C++ field
/// staying at its zero-initialized state for the well-formed corpus).
fn parse_int_attr(el: &Element, name: &str) -> TestResult<int4> {
    let s = attr_string(el, name)?;
    Ok(istream_read_int(&s))
}

/// Reproduce `std::istringstream >> int4`: skip leading whitespace, read an
/// optional sign and the leading run of decimal digits, ignore the rest.
fn istream_read_int(s: &str) -> int4 {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    let mut sign = 1i64;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        if bytes[i] == b'-' {
            sign = -1;
        }
        i += 1;
    }
    let start = i;
    let mut val: i64 = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        val = val.saturating_mul(10).saturating_add((bytes[i] - b'0') as i64);
        i += 1;
    }
    if i == start {
        return 0; // no digits extracted
    }
    (sign * val) as int4
}

#[cfg(test)]
mod tests;
