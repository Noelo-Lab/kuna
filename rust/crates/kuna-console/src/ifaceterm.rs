//! Port of `decompiler/cpp/ifaceterm.{hh,cc}` (W9) — terminal line handling on
//! top of the generic [`IfaceStatus`] framework, plus the datatest's
//! command-stream feed.
//!
//! Two [`LineSource`] implementations live here:
//!
//! - [`IfaceTerm`] (C++ `ifaceterm.cc`): the `decomp_dbg` line reader.  It
//!   reads command lines a character at a time from a base input stream, with a
//!   stack of nested input streams pushed by `source`/init scripts.  The full
//!   character-handling switch of `IfaceTerm::readLine` is transcribed,
//!   including the echo-to-`optr` redraw logic that runs even on a piped
//!   (non-TTY) stdin — this is the batch path the Python harness
//!   (`kuna/decompile.py`) drives, so its stdout must stay byte-faithful.
//!   `doCompletion` (TAB completion) is ported faithfully against the command
//!   registry.
//!
//!   **Documented loss:** the C++ constructor's raw-terminal setup
//!   (`tcgetattr`/`tcsetattr`, `__TERMINAL__`) that makes a real TTY deliver
//!   single keystrokes is not reproduced — kuna's binaries are always driven
//!   by a pipe, where the C++ constructor itself takes the `ENOTTY ->
//!   is_terminal=false; return` branch.  The interactive-only consequence
//!   (live arrow-key / history editing on a true terminal) is therefore out of
//!   scope; the deterministic batch path is what is ported.
//!
//! - [`ConsoleCommands`] (C++ `testfunction.cc`): a non-interactive feed of a
//!   pre-collected command vector with no prompts and no line editing.  This is
//!   the source the XML datatest runner (`decomp_test_dbg`) uses, so its
//!   "return the next command, empty when exhausted" contract is exact.

use crate::interface::{words_to_string, CommandStream, IfaceStatus, LineSource};
use kuna_base::types::int4;

// ---------------------------------------------------------------------------
// CharStream — a byte buffer with cursor + latched eof, the input side of an
// IfaceTerm script-stack entry (C++ `istream *sptr`).
// ---------------------------------------------------------------------------

/// One input stream on `IfaceTerm`'s stack: raw bytes with a cursor and an eof
/// flag, exposing only `get()`/`eof()` (all `IfaceTerm::readLine` needs).
#[derive(Debug, Clone)]
struct CharStream {
    buf: Vec<u8>,
    pos: usize,
    eof: bool,
}

impl CharStream {
    fn new(bytes: Vec<u8>) -> Self {
        CharStream { buf: bytes, pos: 0, eof: false }
    }

    /// C++ `sptr->get()`: read one byte, latching eof at end-of-stream.  The
    /// returned value on eof is unspecified in C++ (the caller checks `eof()`
    /// right after); we return `0` and rely on the eof check, matching the
    /// `if (sptr->eof()) val='\n';` guard.
    fn get(&mut self) -> u8 {
        if self.pos >= self.buf.len() {
            self.eof = true;
            return 0;
        }
        let c = self.buf[self.pos];
        self.pos += 1;
        c
    }

    fn eof(&self) -> bool {
        self.eof
    }
}

// ---------------------------------------------------------------------------
// IfaceTerm.
// ---------------------------------------------------------------------------

/// C++ `IfaceTerm`: the command-line interface over a character input stream.
///
/// Held inside an [`IfaceStatus`] as its [`LineSource`].  Construct the status
/// with [`IfaceTerm::into_status`].
pub struct IfaceTerm {
    /// C++ `sptr`: the current base input stream.
    sptr: CharStream,
    /// C++ `inputstack`: stack of nested input streams (scripts).
    inputstack: Vec<CharStream>,
}

impl IfaceTerm {
    /// Construct over the given base input bytes (C++ `IfaceTerm(prmpt, is,
    /// os)`; the `__TERMINAL__` raw-mode setup is the documented loss).
    pub fn new(base_input: &[u8]) -> Self {
        IfaceTerm { sptr: CharStream::new(base_input.to_vec()), inputstack: Vec::new() }
    }

    /// Build an [`IfaceStatus`] driven by an `IfaceTerm` over `base_input`,
    /// with the given prompt (C++ `new IfaceTerm("[decomp]> ", cin, cout)`).
    pub fn into_status(prompt: &str, base_input: &[u8]) -> IfaceStatus {
        IfaceStatus::with_default_history(prompt, Box::new(IfaceTerm::new(base_input)))
    }

    /// C++ `IfaceTerm::doCompletion`: respond to TAB, completing tokens in
    /// `line` against the command registry, in place.
    ///
    /// Returns the new cursor position.  All `optr` writes go through `status`.
    fn do_completion(&self, status: &mut IfaceStatus, line: &mut String, cursor: int4) -> int4 {
        let mut s = CommandStream::new(line);
        let (mut mat, fullcommand, candidates) = status.completion_expand(&mut s);
        if mat == 0 {
            status.out("\nInvalid command\n");
            return cursor; // No change to command line
        }

        // At least one match.
        let oldsize = line.len();
        *line = words_to_string(&fullcommand);
        if mat < 0 {
            mat = -mat;
        } else {
            line.push(' '); // Extra space if command word is complete
        }
        // Read any additional parameters back to the command line.
        if !s.eof() {
            let tok = s.read_token();
            s.skip_ws();
            line.push_str(&tok); // First space assumed already present
        }
        while !s.eof() {
            line.push(' '); // space between extra parameters
            let tok = s.read_token();
            s.skip_ws();
            line.push_str(&tok);
        }
        if oldsize < line.len() {
            // Expanded at all: just display the expansion.
            return line.len() as int4;
        }

        if mat > 1 {
            // More than one possible command: display all completions.
            status.out("\n");
            for c in &candidates {
                status.out(c);
                status.out("\n");
            }
        } else {
            // Command is unique and expanded.
            status.out("\nCommand is complete\n");
        }
        line.len() as int4
    }
}

impl LineSource for IfaceTerm {
    /// C++ `IfaceTerm::readLine`: read one command line a character at a time,
    /// honoring the line-editing control keys and echoing to `optr`.
    ///
    /// The character switch is transcribed verbatim; on a piped stream the
    /// editing keys rarely appear, but the default-case insert + echo path —
    /// what produces `decomp_dbg`'s echoed stdout — is exact.
    fn read_line(&mut self, status: &mut IfaceStatus) -> String {
        let mut line = String::new();
        let mut cursor: int4 = 0;
        let mut hist: int4 = 0;
        let mut saveline = String::new();
        let mut val: u8;

        loop {
            let mut onecharecho = false;
            let lastlen = line.len();
            val = self.sptr.get();
            if self.sptr.eof() {
                val = b'\n';
            }
            match val {
                0x01 => {
                    // C-a: jump to beginning
                    cursor = 0;
                }
                0x02 => {
                    // C-b: back up one char
                    if cursor > 0 {
                        cursor -= 1;
                    }
                }
                0x03 => {
                    // C-c: clear line, act as return
                    line.clear();
                    cursor = 0;
                    val = 0x0a;
                    onecharecho = true;
                }
                0x04 => {
                    // C-d: delete char under cursor
                    erase_at(&mut line, cursor as usize, 1);
                }
                0x05 => {
                    // C-e: jump to end
                    cursor = line.len() as int4;
                }
                0x06 => {
                    // C-f: forward one char
                    if cursor < line.len() as int4 {
                        cursor += 1;
                    }
                }
                0x07 => {
                    // C-g: do nothing
                }
                0x09 => {
                    // TAB
                    cursor = self.do_completion(status, &mut line, cursor);
                }
                0x0a | 0x0d => {
                    // Newline / carriage return
                    cursor = line.len() as int4;
                    onecharecho = true;
                }
                0x0b => {
                    // C-k: erase from cursor to end
                    erase_from(&mut line, cursor as usize);
                }
                0x0c => {
                    // C-l: do nothing
                }
                0x0e => {
                    // C-n: more recent history
                    if hist > 0 {
                        hist -= 1;
                        if hist > 0 {
                            status.get_history(&mut line, hist - 1);
                        } else {
                            line = saveline.clone();
                        }
                        cursor = line.len() as int4;
                    }
                }
                0x10 => {
                    // C-p: more ancient history
                    if hist < status.history_size() {
                        hist += 1;
                        if hist == 1 {
                            saveline = line.clone();
                        }
                        status.get_history(&mut line, hist - 1);
                        cursor = line.len() as int4;
                    }
                }
                0x12 => {
                    // C-r: do nothing
                }
                0x15 => {
                    // C-u: erase up to cursor
                    erase_range(&mut line, 0, cursor as usize);
                    cursor = 0;
                }
                0x1b => {
                    // Escape: read two more bytes -> arrow keys
                    let mut escval: int4 = self.sptr.get() as int4;
                    escval <<= 8;
                    escval += self.sptr.get() as int4;
                    match escval {
                        0x4f44 => {
                            // left arrow
                            if cursor > 0 {
                                cursor -= 1;
                            }
                        }
                        0x4f43 => {
                            // right arrow
                            if cursor < line.len() as int4 {
                                cursor += 1;
                            }
                        }
                        _ => {}
                    }
                }
                0x08 | 0x7f => {
                    // backspace / delete
                    if cursor != 0 {
                        cursor -= 1;
                        erase_at(&mut line, cursor as usize, 1);
                    }
                }
                _ => {
                    // Insert single character at cursor.
                    insert_at(&mut line, cursor as usize, val);
                    cursor += 1;
                    if cursor == line.len() as int4 {
                        onecharecho = true;
                    }
                }
            }

            if onecharecho {
                status.out_byte(val); // Echo most characters
            } else {
                status.out_byte(b'\r'); // Ontop of old line
                status.write_prompt();
                status.out(&line); // print new line
                let mut i = line.len();
                while i < lastlen {
                    status.out_byte(b' '); // erase old characters
                    i += 1;
                }
                let mut k = (i as int4) - cursor;
                while k > 0 {
                    status.out_byte(0x08); // backspace: put cursor into place
                    k -= 1;
                }
            }

            if val == b'\n' {
                break;
            }
        }
        line
    }

    fn push_input(&mut self, lines: Vec<String>) {
        // C++ pushScript: inputstack.push_back(sptr); sptr = iptr;
        let joined = join_lines(&lines);
        let prev = std::mem::replace(&mut self.sptr, CharStream::new(joined));
        self.inputstack.push(prev);
    }

    fn pop_input(&mut self) {
        // C++ popScript: delete sptr; sptr = inputstack.back();
        self.sptr = self.inputstack.pop().expect("IfaceTerm::popScript: empty input stack");
    }

    fn current_stream_finished(&self) -> bool {
        // C++ isStreamFinished: (done||inerror handled by IfaceStatus) sptr->eof()
        self.sptr.eof()
    }
}

/// Rejoin pushed script lines into a byte buffer the `IfaceTerm` reader can
/// consume.  A script file's bytes are newline-separated lines; the framework
/// hands them in already split, so we re-add the separators.
fn join_lines(lines: &[String]) -> Vec<u8> {
    let mut out = String::new();
    for l in lines {
        out.push_str(l);
        out.push('\n');
    }
    out.into_bytes()
}

// ---- byte-indexed string edits mirroring std::string member ops ----------
//
// C++ operates on `std::string` (a byte sequence) with byte indices.  These
// helpers replicate `string::insert`, `string::erase(pos,n)`,
// `string::erase(pos)` over the UTF-8 byte buffer so command-line editing
// matches byte-for-byte.

fn insert_at(line: &mut String, pos: usize, b: u8) {
    let mut bytes = std::mem::take(line).into_bytes();
    let pos = pos.min(bytes.len());
    bytes.insert(pos, b);
    *line = String::from_utf8_lossy(&bytes).into_owned();
}

fn erase_at(line: &mut String, pos: usize, n: usize) {
    let mut bytes = std::mem::take(line).into_bytes();
    if pos < bytes.len() {
        let end = (pos + n).min(bytes.len());
        bytes.drain(pos..end);
    }
    *line = String::from_utf8_lossy(&bytes).into_owned();
}

fn erase_from(line: &mut String, pos: usize) {
    let mut bytes = std::mem::take(line).into_bytes();
    if pos < bytes.len() {
        bytes.truncate(pos);
    }
    *line = String::from_utf8_lossy(&bytes).into_owned();
}

fn erase_range(line: &mut String, start: usize, len: usize) {
    let mut bytes = std::mem::take(line).into_bytes();
    let start = start.min(bytes.len());
    let end = (start + len).min(bytes.len());
    bytes.drain(start..end);
    *line = String::from_utf8_lossy(&bytes).into_owned();
}

// ---------------------------------------------------------------------------
// ConsoleCommands — the datatest command feed (testfunction.cc).
// ---------------------------------------------------------------------------

/// C++ `ConsoleCommands`: a console whose command lines come from a fixed
/// vector, with no prompts and no line editing.  This is what the XML datatest
/// runner drives.
///
/// `readLine` returns commands in order and an empty string once exhausted;
/// `reset` rewinds for a new program (the rewind clears the index here, the
/// `inerror`/`done` reset is done by [`IfaceStatus::reset`]-style callers).
pub struct ConsoleCommands {
    commands: Vec<String>,
    /// C++ `pos`: index of the next command to execute.
    pos: usize,
}

impl ConsoleCommands {
    /// Construct over the command sequence (C++ `ConsoleCommands(s, comms)`).
    pub fn new(commands: Vec<String>) -> Self {
        ConsoleCommands { commands, pos: 0 }
    }

    /// Build an [`IfaceStatus`] driven by a `ConsoleCommands` feed with the
    /// C++ `"> "` prompt (C++ `IfaceStatus("> ", s)`).
    pub fn into_status(commands: Vec<String>) -> IfaceStatus {
        IfaceStatus::with_default_history("> ", Box::new(ConsoleCommands::new(commands)))
    }

    /// Replace the command list and rewind (the datatest collection rebuilds
    /// `commands` per program, then `reset`s).
    pub fn set_commands(&mut self, commands: Vec<String>) {
        self.commands = commands;
        self.pos = 0;
    }

    /// C++ `ConsoleCommands::reset` (the `pos` half): rewind to the first
    /// command.  `inerror`/`done` are reset on the owning [`IfaceStatus`].
    pub fn rewind(&mut self) {
        self.pos = 0;
    }
}

impl LineSource for ConsoleCommands {
    fn read_line(&mut self, _status: &mut IfaceStatus) -> String {
        if self.pos >= self.commands.len() {
            return String::new();
        }
        let line = self.commands[self.pos].clone();
        self.pos += 1;
        line
    }

    fn push_input(&mut self, _lines: Vec<String>) {
        // ConsoleCommands does not override pushScript: scripts are not pushed
        // in the datatest feed (commands are a flat list). Mirrors C++ where
        // ConsoleCommands uses the base IfaceStatus::pushScript, which throws
        // on a non-null stream — the datatest scripts never invoke `source`.
        panic!("ConsoleCommands does not support nested input streams");
    }

    fn pop_input(&mut self) {
        panic!("ConsoleCommands does not support nested input streams");
    }

    fn current_stream_finished(&self) -> bool {
        // C++ isStreamFinished: pos == commands.size()
        self.pos >= self.commands.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::{CommandStream, IfaceCommandAction, IfaceResult};

    struct NoopAction;
    impl IfaceCommandAction for NoopAction {
        fn execute(&self, _s: &mut IfaceStatus, _is: &mut CommandStream) -> IfaceResult<()> {
            Ok(())
        }
        fn module(&self) -> String {
            "base".into()
        }
    }

    fn term_status(input: &str) -> IfaceStatus {
        let mut st = IfaceTerm::into_status("[decomp]> ", input.as_bytes());
        st.register_com(Box::new(NoopAction), &["quit"]);
        st.register_com(Box::new(NoopAction), &["load", "file"]);
        st.register_com(Box::new(NoopAction), &["print", "C"]);
        st.register_com(Box::new(NoopAction), &["print", "raw"]);
        st
    }

    // ---- IfaceTerm batch readLine --------------------------------------

    #[test]
    fn term_reads_simple_line_and_echoes() {
        // Drive run_command through one line; assert the line executes and the
        // echo matches the C++ batch behavior (each char echoed, then newline).
        let mut st = term_status("quit\n");
        assert!(st.run_command().unwrap());
        // Echo: "quit" char-by-char, then '\n'.
        assert_eq!(st.optr, "quit\n");
    }

    #[test]
    fn term_reads_multichar_with_params() {
        let mut st = term_status("load file foo\n");
        assert!(st.run_command().unwrap());
        assert_eq!(st.optr, "load file foo\n");
    }

    #[test]
    fn term_eof_terminates_line() {
        // No trailing newline: eof supplies the '\n'.
        let mut st = term_status("quit");
        assert!(st.run_command().unwrap());
        assert_eq!(st.optr, "quit\n");
    }

    #[test]
    fn term_blank_line_no_command() {
        let mut st = term_status("\n");
        // Empty line -> readLine returns "" -> run_command returns false.
        assert!(!st.run_command().unwrap());
        // The lone '\n' is echoed.
        assert_eq!(st.optr, "\n");
    }

    #[test]
    fn term_stream_finished_after_input() {
        // Faithful to std::istream: reading "quit\n" in full does NOT set eof
        // (eof is latched only on the *next* get past the end), so the stream
        // is not yet finished after one command — mirroring C++ where the
        // mainloop makes one more (empty) readLine pass before eof shows.
        let mut st = term_status("quit\n");
        assert!(!st.is_stream_finished());
        st.run_command().unwrap();
        assert!(!st.is_stream_finished());
        // The trailing read sees eof, yields an empty line (no command).
        assert!(!st.run_command().unwrap());
        assert!(st.is_stream_finished());
    }

    // ---- doCompletion --------------------------------------------------

    #[test]
    fn term_tab_completes_unique() {
        // "q" + TAB should expand to "quit " and report complete.
        let mut st = term_status("q\t\n");
        // First run_command reads the "q<TAB>\n" line; TAB triggers completion
        // which writes to optr and rewrites the line, then '\n' ends it.
        st.run_command().unwrap();
        // The completion appended "Command is complete" once expanded uniquely.
        // After expansion the line became "quit " (oldsize<newsize) so it just
        // displays the expansion; assert the expanded command resolved.
        assert!(st.optr.contains("quit"));
    }

    #[test]
    fn term_tab_ambiguous_expands_to_space() {
        // "print" + TAB expands the common stem and appends a trailing space
        // (line grows), so doCompletion returns the expansion and does NOT yet
        // list candidates — matching C++ doCompletion's `oldsize < line.size()`
        // early return.
        let mut st = term_status("print\t\n");
        st.run_command().unwrap();
        assert!(!st.optr.contains("print C"));
        assert!(!st.optr.contains("Command is complete"));
    }

    #[test]
    fn term_tab_ambiguous_lists_candidates() {
        // "print " (trailing space) + TAB: the stem is already complete, the
        // line does not grow, so doCompletion lists both candidates.
        let mut st = term_status("print \t\n");
        st.run_command().unwrap();
        assert!(st.optr.contains("print C"));
        assert!(st.optr.contains("print raw"));
    }

    // ---- ConsoleCommands ----------------------------------------------

    #[test]
    fn console_commands_feed_in_order() {
        let mut cc = ConsoleCommands::new(vec!["a".into(), "b".into()]);
        let mut dummy = IfaceTerm::into_status("> ", b"");
        assert_eq!(cc.read_line(&mut dummy), "a");
        assert_eq!(cc.read_line(&mut dummy), "b");
        assert_eq!(cc.read_line(&mut dummy), ""); // exhausted
        assert!(cc.current_stream_finished());
    }

    #[test]
    fn console_commands_rewind() {
        let mut cc = ConsoleCommands::new(vec!["x".into()]);
        let mut dummy = IfaceTerm::into_status("> ", b"");
        assert_eq!(cc.read_line(&mut dummy), "x");
        assert!(cc.current_stream_finished());
        cc.rewind();
        assert!(!cc.current_stream_finished());
        assert_eq!(cc.read_line(&mut dummy), "x");
    }

    #[test]
    fn console_commands_drives_status() {
        let mut st = ConsoleCommands::into_status(vec!["quit".into()]);
        st.register_com(Box::new(NoopAction), &["quit"]);
        assert!(st.run_command().unwrap());
        // ConsoleCommands does not echo.
        assert_eq!(st.optr, "");
        assert!(st.is_stream_finished());
    }

    // ---- byte-edit helpers ---------------------------------------------

    #[test]
    fn string_edits() {
        let mut s = "abc".to_string();
        insert_at(&mut s, 1, b'X');
        assert_eq!(s, "aXbc");
        erase_at(&mut s, 1, 1);
        assert_eq!(s, "abc");
        erase_from(&mut s, 1);
        assert_eq!(s, "a");
        let mut s = "hello".to_string();
        erase_range(&mut s, 0, 2);
        assert_eq!(s, "llo");
    }

    // ---- VERIFIER adversarial tests (w9-con-interface) -----------------

    /// w9-con-interface: TAB completion on a unique prefix with trailing
    /// parameters must expand the command word AND carry the parameters back
    /// onto the line verbatim, with a single space before the first param and
    /// single spaces between params (doCompletion's `>> tok >> ws` readback).
    /// We drive it through expand_com + the readback by checking the resolved,
    /// executed line: "lo fi  bar   baz" + TAB completes to load file then runs
    /// (the command line gets rebuilt and re-fed).  Easiest faithful check:
    /// the doCompletion output for "lo<TAB>" expands and the param survives.
    #[test]
    fn verify_w9_tab_readback_preserves_params() {
        // "load file" registered; type "lo fil  extra\t" then newline.
        // After TAB, the line becomes "load file extra" (single-spaced); since
        // it grew, doCompletion returns the expansion (no candidate listing).
        let mut st = term_status("lo fil  extra\t\n");
        st.run_command().unwrap();
        // The redraw echoes the rebuilt line; it must contain the collapsed
        // "load file extra" with the multiple spaces between params collapsed
        // to one (C++ reads token, appends single space).
        assert!(st.optr.contains("load file extra"), "optr was {:?}", st.optr);
        // It must NOT have listed candidates or said "Command is complete"
        // (the line grew, so doCompletion early-returns the expansion).
        assert!(!st.optr.contains("Command is complete"));
    }

    /// w9-con-interface: a backspace mid-line on a piped stream must edit the
    /// buffer AND emit the C++ redraw sequence (\r, prompt, line, trailing
    /// blanks to erase the removed char, then backspaces to reposition).  Type
    /// "ab", backspace (0x7f), "c", newline -> final command line "ac".
    /// We assert the executed line resolves and the redraw emitted at least
    /// one literal '\r' and the erase blank for the shrunk line.
    #[test]
    fn verify_w9_backspace_redraw_bytes() {
        // Register "ac" so the edited line is a valid command.
        let mut st = IfaceTerm::into_status("[decomp]> ", b"ab\x7fc\n");
        st.register_com(Box::new(NoopAction), &["ac"]);
        let ran = st.run_command().unwrap();
        assert!(ran, "edited line 'ac' should execute");
        // The non-onecharecho redraw path emits a carriage return.
        assert!(st.optr.contains('\r'), "redraw must emit CR; optr={:?}", st.optr);
        // The prompt is rewritten on the redraw line.
        assert!(st.optr.contains("[decomp]> "), "redraw must rewrite prompt");
    }

    /// w9-con-interface: C-c (0x03) clears the line and behaves as a return,
    /// producing an empty command (no execution) and echoing a single newline.
    /// Pins the `val = 0x0a; onecharecho = true` branch.
    #[test]
    fn verify_w9_ctrl_c_clears_line() {
        let mut st = term_status("partial\x03");
        // C-c clears the typed "partial" and acts as newline -> empty line.
        let ran = st.run_command().unwrap();
        assert!(!ran, "C-c yields an empty line: no command");
        // The cleared line redraws (partial then erased) and the final newline
        // is echoed; the key invariant is that no command executed.
        assert!(st.optr.ends_with('\n'), "C-c emits a trailing newline");
    }
}
