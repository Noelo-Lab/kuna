//! A serial whole-program run's finished results, kept out of its heap.
//!
//! The callee-first driver holds every function's result until its last round
//! has run, and held as values they were the largest thing a one-worker batch
//! kept between functions: the per-function memory limit charged each function
//! for every result before it. Each result is written to an anonymous temporary
//! file as it lands, in the record a `--jobs` worker writes, and read back when
//! a round or the final document asks for it. A store whose file cannot be made
//! or written keeps the encoded record in memory instead, and a result carrying
//! a token map, which the record does not, stays as it is.

use std::io::{Read, Seek, SeekFrom, Write};

use kuna_console::project::FuncResult;

use crate::jobs::{decode_result, encode_result};

pub(crate) struct ResultStore {
    file: Option<std::fs::File>,
    end: u64,
    /// A write to `file` failed: the results already on it stay there, and
    /// every later one is kept in memory.
    full: bool,
    slots: Vec<Option<Slot>>,
}

struct Slot {
    body: Body,
    lines: usize,
}

enum Body {
    File { at: u64, len: usize },
    Memory(Box<[u8]>),
    Value(Box<FuncResult>),
}

impl ResultStore {
    /// An empty store with `len` slots.
    pub(crate) fn new(len: usize) -> Self {
        Self::over(tempfile::tempfile().ok(), len)
    }

    fn over(file: Option<std::fs::File>, len: usize) -> Self {
        Self { file, end: 0, full: false, slots: (0..len).map(|_| None).collect() }
    }

    /// Keep `r` as slot `index`'s result, replacing any earlier one.
    pub(crate) fn put(&mut self, index: usize, r: FuncResult) {
        let lines = super::callee_first::redo_charge(r.code.as_deref());
        if r.detail.is_some() {
            self.slots[index] = Some(Slot { body: Body::Value(Box::new(r)), lines });
            return;
        }
        let encoded = encode_result(&r);
        drop(r);
        let body = match self.write(&encoded) {
            Some(at) => Body::File { at, len: encoded.len() },
            None => Body::Memory(encoded.into_boxed_slice()),
        };
        self.slots[index] = Some(Slot { body, lines });
    }

    /// Slot `index`'s result, if it has one.
    pub(crate) fn get(&mut self, index: usize) -> Option<FuncResult> {
        Some(match &self.slots[index].as_ref()?.body {
            Body::Memory(bytes) => decode(bytes),
            Body::Value(r) => (**r).clone(),
            &Body::File { at, len } => decode(&read_at(self.file.as_mut(), at, len)),
        })
    }

    /// What redoing slot `index` is charged (`redo_charge`): the lines its C
    /// printed, 0 for an empty slot or an error.
    pub(crate) fn lines(&self, index: usize) -> usize {
        self.slots[index].as_ref().map_or(0, |s| s.lines)
    }

    /// How many lines every result printed.
    pub(crate) fn printed(&self) -> usize {
        self.slots.iter().flatten().map(|s| s.lines).sum()
    }

    /// Every result, in slot order, skipping empty slots.
    pub(crate) fn into_results(mut self) -> Vec<FuncResult> {
        let slots = std::mem::take(&mut self.slots);
        slots
            .into_iter()
            .flatten()
            .map(|slot| match slot.body {
                Body::Memory(bytes) => decode(&bytes),
                Body::Value(r) => *r,
                Body::File { at, len } => decode(&read_at(self.file.as_mut(), at, len)),
            })
            .collect()
    }

    /// Append `encoded` to the file, returning where it starts. After a failed
    /// write the file is only read, so every later result stays in memory.
    fn write(&mut self, encoded: &[u8]) -> Option<u64> {
        let file = self.file.as_mut().filter(|_| !self.full)?;
        let at = self.end;
        match file.seek(SeekFrom::Start(at)).and_then(|_| file.write_all(encoded)) {
            Ok(()) => {
                self.end += encoded.len() as u64;
                Some(at)
            }
            Err(_) => {
                self.full = true;
                None
            }
        }
    }
}

fn read_at(file: Option<&mut std::fs::File>, at: u64, len: usize) -> Vec<u8> {
    let file = file.expect("a slot on file implies the store's file");
    let mut bytes = vec![0; len];
    file.seek(SeekFrom::Start(at))
        .and_then(|_| file.read_exact(&mut bytes))
        .expect("re-reading a decompile result from this run's temporary file");
    bytes
}

fn decode(bytes: &[u8]) -> FuncResult {
    decode_result(bytes).expect("a decompile result record this run encoded")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(name: &str, code: Option<&str>) -> FuncResult {
        FuncResult {
            stack_objects: Vec::new(),
            name: name.into(),
            address: 0x1000,
            byte_address: 0x1000,
            size: 16,
            code: code.map(Into::into),
            error: code.is_none().then(|| "failed".into()),
            proto: None,
            emitted_c_name: None,
            call_declarations: Vec::new(),
            callwide_prototypes: Vec::new(),
            variables: Vec::new(),
            types: Vec::new(),
            globals: Vec::new(),
            field_calls: Vec::new(),
            line_mappings: Vec::new(),
            aliases: vec!["alias".into()],
            object_location: None,
            callee_hints: Vec::new(),
            synth: None,
            pointerargs: None,
            detail: None,
        }
    }

    fn check(mut store: ResultStore) {
        store.put(2, result("c", Some("int c(void)\n{\n}")));
        store.put(0, result("a", None));
        store.put(0, result("a2", Some("void a(void)")));
        let mut tokens = result("d", Some("void d(void)"));
        tokens.detail = Some(Box::default());
        store.put(3, tokens);
        assert_eq!(store.get(1).map(|r| r.name), None);
        assert_eq!(store.get(0).map(|r| r.name).as_deref(), Some("a2"));
        assert_eq!((store.lines(0), store.lines(1), store.lines(2), store.printed()), (1, 0, 3, 5));
        let all = store.into_results();
        assert_eq!(all.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["a2", "c", "d"]);
        assert_eq!(all[1].code.as_deref(), Some("int c(void)\n{\n}"));
        assert_eq!(all[1].aliases, ["alias"]);
        assert!(all[2].detail.is_some());
    }

    #[test]
    fn results_read_back_as_written_from_the_file_and_from_memory() {
        let store = ResultStore::new(4);
        assert!(store.file.is_some());
        check(store);
        check(ResultStore::over(None, 4));
    }

    #[test]
    fn a_failed_write_keeps_the_results_already_on_file() {
        let spool = tempfile::NamedTempFile::new().unwrap();
        let mut store = ResultStore::over(Some(spool.reopen().unwrap()), 3);
        store.put(0, result("a", Some("void a(void)")));
        assert!(matches!(store.slots[0].as_ref().map(|s| &s.body), Some(Body::File { .. })));
        store.file = Some(std::fs::File::open(spool.path()).unwrap());
        store.put(1, result("b", Some("void b(void)")));
        assert!(matches!(store.slots[1].as_ref().map(|s| &s.body), Some(Body::Memory(_))));
        store.put(2, result("c", None));
        assert_eq!(store.get(0).map(|r| r.name).as_deref(), Some("a"));
        let all = store.into_results();
        assert_eq!(all.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["a", "b", "c"]);
    }
}
