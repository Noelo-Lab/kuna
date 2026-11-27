//! Where each class and method lies in a decompiled root class's text.
//!
//! `ClassWriter` opens a span where it starts writing a class or a method and
//! closes it where it finishes, by putting zero-width marks into the text
//! buffer ([`TextBuffer::add_kuna_mark`]). Marks ride along with the buffer
//! through appends and line re-wrapping the way text tokens do and are never
//! printed, so recording changes no output. When the root class's text is
//! final, [`finish`] turns the marks into byte offsets and 1-based lines, and
//! groups the buffer's bytecode-offset mapping (what `BytecodeMappingTracer`
//! receives, whatever `bytecode-source-mapping` says) by method and line, as
//! the CLI contract's spans (`kuna_managed::{TypeSpan, MemberSpan}`).
//! Recording is active only between [`begin`] and [`take`].
//!
//! A body the writers print inside another one and cannot decompile (a
//! lambda, a method of an anonymous or local class, such a class itself) is
//! recorded by [`method_failed`] or [`class_failed`] against every method span
//! open at that moment, and against an open class span when no method of it
//! is open (a field initializer): the failure comment is part of their text.
//! A method's own `$VF:` comments are kept when the engine marks them as
//! errors (`MethodWrapper::add_error_comment`).

use std::cell::RefCell;
use std::collections::BTreeMap;

use kuna_managed::{Failure, MemberSpan, TypeSpan};

use crate::class_wrapper::ClassWrapper;
use crate::interpreter_util::InterpreterUtil;
use crate::kuna_engine::java_name;
use crate::kuna_inventory::method_access_names;
use crate::method_wrapper::MethodWrapper;
use crate::rt::Caught;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

/// A body that could not be decompiled: a method (or lambda), or a class when
/// `name` is empty.
#[derive(Clone, Default)]
struct BodyFailure {
    /// Internal name of the class.
    class: String,
    name: String,
    descriptor: String,
    error: String,
}

impl BodyFailure {
    /// Named like a method (`com.ex.Main.lambda$main$0(I)V`) or a class (`com.ex.Main$1`).
    fn failure(self) -> Failure {
        let class = java_name(&self.class);
        let body = if self.name.is_empty() { class } else { format!("{class}.{}{}", self.name, self.descriptor) };
        Failure { body, error: self.error }
    }
}

/// What one root class's writing recorded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Recorded {
    pub error: Option<String>,
    pub classes: Vec<TypeSpan>,
    pub methods: Vec<MemberSpan>,
}

/// An open span; `None` when nothing is being recorded.
pub type SpanId = Option<u32>;

enum Pending {
    Class {
        internal: String,
        failed: bool,
        reason: Option<String>,
        nested: Vec<BodyFailure>,
    },
    Method {
        class: String,
        name: String,
        descriptor: String,
        access: i32,
        wrapper: ClassWrapper,
        nested: Vec<BodyFailure>,
    },
}

#[derive(Default)]
struct Recorder {
    active: bool,
    pending: Vec<Pending>,
    /// Spans opened and not yet closed, innermost last.
    open: Vec<u32>,
    result: Recorded,
}

thread_local! {
    static RECORDER: RefCell<Recorder> = RefCell::new(Recorder::default());
}

/// Starts recording the next root class's spans.
pub fn begin() {
    RECORDER.with(|r| {
        *r.borrow_mut() = Recorder {
            active: true,
            ..Recorder::default()
        }
    });
}

/// Stops recording and returns what was recorded since [`begin`].
pub fn take() -> Recorded {
    RECORDER.with(|r| std::mem::take(&mut *r.borrow_mut()).result)
}

fn open(buffer: &mut TextBuffer, pending: Pending) -> SpanId {
    let id = RECORDER.with(|r| {
        let mut r = r.borrow_mut();
        if !r.active {
            return None;
        }
        r.pending.push(pending);
        let id = (r.pending.len() - 1) as u32;
        r.open.push(id);
        Some(id)
    })?;
    buffer.add_kuna_mark(id * 2);
    Some(id)
}

/// Opens the span of the class `cl` at the end of `buffer`.
pub fn open_class(buffer: &mut TextBuffer, cl: StructClass) -> SpanId {
    if !is_active() {
        return None;
    }
    open(
        buffer,
        Pending::Class {
            internal: cl.qualified_name(),
            failed: false,
            reason: None,
            nested: Vec::new(),
        },
    )
}

/// Opens the span of `mt` (of class `cl`) at the end of `buffer`; `wrapper`
/// holds the method's decompile error once writing is done.
pub fn open_method(
    buffer: &mut TextBuffer,
    cl: StructClass,
    mt: StructMethod,
    wrapper: ClassWrapper,
) -> SpanId {
    if !is_active() {
        return None;
    }
    open(
        buffer,
        Pending::Method {
            class: cl.qualified_name(),
            name: mt.get_name(),
            descriptor: mt.get_descriptor(),
            access: mt.get_access_flags(),
            wrapper,
            nested: Vec::new(),
        },
    )
}

/// Closes `span` at the end of `buffer`, and every span opened inside it
/// that a throw left open.
pub fn close(buffer: &mut TextBuffer, span: SpanId) {
    if let Some(id) = span {
        buffer.add_kuna_mark(id * 2 + 1);
        RECORDER.with(|r| {
            let mut guard = r.borrow_mut();
            if let Some(at) = guard.open.iter().rposition(|&o| o == id) {
                guard.open.truncate(at);
            }
        });
    }
}

/// How many spans are open, for a writer that catches a failure to
/// [`unwind`] to.
pub fn depth() -> usize {
    RECORDER.with(|r| r.borrow().open.len())
}

/// Drops the spans opened after `depth` that a caught failure left open (the
/// header of a method that failed to write), so that the failure is recorded
/// on the spans its comment is written in.
pub fn unwind(depth: usize) {
    RECORDER.with(|r| r.borrow_mut().open.truncate(depth));
}

/// `wrapper`'s method is written as an error comment: its own span keeps
/// the error, every other open span records it.
pub fn method_failed(wrapper: MethodWrapper) {
    if !is_active() {
        return;
    }
    let (cl, mt, error) = wrapper.with(|d| {
        (
            d.class_struct,
            d.method_struct,
            d.decompile_error.as_ref().map(|e| e.message.clone()),
        )
    });
    record(BodyFailure {
        class: cl.qualified_name(),
        name: mt.get_name(),
        descriptor: mt.get_descriptor(),
        error: error.unwrap_or_default(),
    });
}

/// The class `internal`, whose span is open, is written as an error comment
/// for `reason`.
pub fn class_failed(internal: &str, reason: &str) {
    if !is_active() {
        return;
    }
    RECORDER.with(|r| {
        let mut guard = r.borrow_mut();
        let Recorder { pending, open, .. } = &mut *guard;
        let span = open.iter().rev().find(|&&id| {
            matches!(pending.get(id as usize), Some(Pending::Class { internal: name, .. }) if name == internal)
        });
        if let Some(Pending::Class { failed, reason: slot, .. }) =
            span.and_then(|&id| pending.get_mut(id as usize))
        {
            *failed = true;
            *slot = Some(reason.to_string());
        }
    });
    record(BodyFailure {
        class: internal.to_string(),
        error: reason.to_string(),
        ..BodyFailure::default()
    });
}

fn record(failure: BodyFailure) {
    RECORDER.with(|r| {
        let mut guard = r.borrow_mut();
        let Recorder { pending, open, .. } = &mut *guard;
        let mut in_method = false;
        for &id in open.iter().rev() {
            let (own, nested) = match pending.get_mut(id as usize) {
                Some(Pending::Method {
                    class,
                    name,
                    descriptor,
                    nested,
                    ..
                }) => {
                    in_method = true;
                    let own = *class == failure.class
                        && *name == failure.name
                        && *descriptor == failure.descriptor;
                    (own, nested)
                }
                Some(Pending::Class {
                    internal, nested, ..
                }) if !in_method => (failure.name.is_empty() && *internal == failure.class, nested),
                _ => continue,
            };
            let seen = nested.iter().any(|f| {
                f.class == failure.class && f.name == failure.name && f.descriptor == failure.descriptor
            });
            if !own && !seen {
                nested.push(failure.clone());
            }
        }
    });
}

/// Closes the class span `span`, whose class could not be written.
pub fn close_failed(buffer: &mut TextBuffer, span: SpanId) {
    if let Some(id) = span {
        RECORDER.with(|r| {
            if let Some(Pending::Class { failed, .. }) = r.borrow_mut().pending.get_mut(id as usize)
            {
                *failed = true;
            }
        });
    }
    close(buffer, span);
}

/// The root class failed as a whole (`getClassContent`'s catch).
pub fn fail(class_name: &str, error: &Caught) {
    RECORDER.with(|r| {
        let mut r = r.borrow_mut();
        if r.active {
            r.result.error = Some(format!(
                "Class {class_name} couldn't be fully decompiled: {}",
                error.message
            ));
        }
    });
}

fn is_active() -> bool {
    RECORDER.with(|r| r.borrow().active)
}

/// Resolves the open spans against the final `buffer` and its text `content`.
pub fn finish(buffer: &TextBuffer, content: &str) {
    let pending = RECORDER.with(|r| {
        let mut r = r.borrow_mut();
        if !r.active {
            return None;
        }
        Some(std::mem::take(&mut r.pending))
    });
    let Some(pending) = pending else {
        return;
    };
    let text = Text::new(content);
    let mut edges: BTreeMap<u32, (Option<usize>, Option<usize>)> = BTreeMap::new();
    for (mark, at) in buffer.kuna_marks() {
        let Some(byte) = text.byte_offset(at) else {
            continue;
        };
        let slot = edges.entry(mark / 2).or_default();
        if mark % 2 == 0 {
            slot.0.get_or_insert(byte);
        } else {
            slot.1 = Some(byte);
        }
    }
    let lines = method_lines(buffer, &text);

    let mut classes = Vec::new();
    let mut methods = Vec::new();
    for (id, record) in pending.into_iter().enumerate() {
        let Some(&(Some(start), Some(end))) = edges.get(&(id as u32)) else {
            continue;
        };
        if end < start {
            continue;
        }
        let (start, end) = text.widen(start, end);
        let (start_line, end_line) = (text.line_of(start), text.line_of(end.max(start + 1) - 1));
        match record {
            Pending::Class {
                internal,
                failed,
                reason,
                nested,
            } => {
                let error = failed.then(|| {
                    reason.unwrap_or_else(|| format!("Class {internal} couldn't be written."))
                });
                classes.push(TypeSpan {
                    name: internal,
                    start,
                    end,
                    start_line,
                    error,
                    nested: nested.into_iter().map(BodyFailure::failure).collect(),
                });
            }
            Pending::Method {
                class,
                name,
                descriptor,
                access,
                wrapper,
                nested,
            } => {
                let method = Some(wrapper)
                    .filter(|w| w.is_live())
                    .and_then(|w| w.get_method_wrapper(&name, &descriptor));
                let (error, warnings) = method.map_or((None, Vec::new()), |m| {
                    m.with(|d| {
                        let warnings = match (&d.comment_lines, d.add_error_comment) {
                            (Some(lines), true) => lines
                                .iter()
                                .filter(|l| l.starts_with("$VF:"))
                                .cloned()
                                .collect(),
                            _ => Vec::new(),
                        };
                        (d.decompile_error.as_ref().map(|e| e.message.clone()), warnings)
                    })
                });
                let key = (
                    class.clone(),
                    InterpreterUtil::make_unique_key(&name, &descriptor),
                );
                let lines = lines.get(&key).cloned().unwrap_or_default();
                methods.push(MemberSpan {
                    owner: class,
                    name,
                    signature: descriptor,
                    access: method_access_names(access as u16),
                    start,
                    end,
                    start_line,
                    end_line,
                    error,
                    lines,
                    nested: nested.into_iter().map(BodyFailure::failure).collect(),
                    warnings,
                });
            }
        }
    }
    classes.sort_by_key(|c| c.start);
    methods.sort_by_key(|m| m.start);
    RECORDER.with(|r| {
        let mut r = r.borrow_mut();
        if r.active {
            r.result.classes = classes;
            r.result.methods = methods;
        }
    });
}

type MethodKey = (String, String);

fn method_lines(buffer: &TextBuffer, text: &Text) -> BTreeMap<MethodKey, Vec<(u32, Vec<u32>)>> {
    let mut per_method: BTreeMap<MethodKey, BTreeMap<u32, Vec<u32>>> = BTreeMap::new();
    for (class, method, offset, line) in buffer.kuna_bytecode_lines() {
        let (Some(class), Some(method)) = (class, method) else {
            continue;
        };
        if offset < 0 || line < 0 {
            continue;
        }
        let line = line as u32 + text.first_line();
        per_method
            .entry((class, method))
            .or_default()
            .entry(line)
            .or_default()
            .push(offset as u32);
    }
    per_method
        .into_iter()
        .map(|(key, lines)| {
            let lines = lines
                .into_iter()
                .map(|(line, mut offsets)| {
                    offsets.sort_unstable();
                    (line, offsets)
                })
                .collect();
            (key, lines)
        })
        .collect()
}

/// UTF-16 positions, byte offsets and lines of the final text.
struct Text<'a> {
    content: &'a str,
    /// Byte offset of each UTF-16 unit boundary; `units[i]` is where unit `i` starts.
    units: Vec<usize>,
    newlines: Vec<usize>,
}

impl<'a> Text<'a> {
    fn new(content: &'a str) -> Text<'a> {
        let mut units = Vec::with_capacity(content.len() + 1);
        let mut newlines = Vec::new();
        for (byte, c) in content.char_indices() {
            for _ in 0..c.len_utf16() {
                units.push(byte);
            }
            if c == '\n' {
                newlines.push(byte);
            }
        }
        units.push(content.len());
        Text {
            content,
            units,
            newlines,
        }
    }

    fn first_line(&self) -> u32 {
        1
    }

    fn byte_offset(&self, unit: i32) -> Option<usize> {
        usize::try_from(unit)
            .ok()
            .and_then(|u| self.units.get(u).copied())
    }

    /// 1-based line holding byte `at`.
    fn line_of(&self, at: usize) -> u32 {
        self.newlines.partition_point(|&n| n < at) as u32 + 1
    }

    /// Moves `start` back to the start of its line when only blanks precede it,
    /// and `end` back to the end of the previous line when only blanks follow that.
    fn widen(&self, start: usize, end: usize) -> (usize, usize) {
        let bytes = self.content.as_bytes();
        let blank = |b: u8| b == b' ' || b == b'\t';
        let mut s = start;
        while s > 0 && blank(bytes[s - 1]) {
            s -= 1;
        }
        if s > 0 && bytes[s - 1] != b'\n' {
            s = start;
        }
        let mut e = end;
        while e > s && blank(bytes[e - 1]) {
            e -= 1;
        }
        if !(e > s && bytes[e - 1] == b'\n') {
            e = end;
        }
        (s, e)
    }
}

#[cfg(test)]
mod tests {
    use super::BodyFailure;

    #[test]
    fn nested_bodies_are_named_like_methods_or_classes() {
        let failure = |class: &str, name: &str, descriptor: &str| {
            BodyFailure { class: class.into(), name: name.into(), descriptor: descriptor.into(), error: "boom".into() }
                .failure()
        };
        assert_eq!(failure("com/ex/Main", "lambda$main$0", "(I)V").body, "com.ex.Main.lambda$main$0(I)V");
        assert_eq!(failure("com/ex/Main$1", "", "").body, "com.ex.Main$1");
        assert_eq!(failure("com/ex/Main$1", "", "").error, "boom");
    }
}
