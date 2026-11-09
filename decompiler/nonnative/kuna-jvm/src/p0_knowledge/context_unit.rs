//! Port of `org/jetbrains/java/decompiler/struct/ContextUnit.java`.
//!
//! `save` keeps Vineflower's two phases (pre-process every class, then emit
//! every class) but runs each class task on the calling thread; the per-class
//! context switch in [`ContextUnit::run_on_worker`] is the seam for a thread
//! pool. Directory and resource entries are not copied anywhere: finished
//! classes go to the callback that replaces `IOutputSink`.

use std::rc::Rc;

use crate::arena_object;
use crate::byte_cursor::ByteCursor;
use crate::cancelation_manager::CanceledException;
use crate::class_writer::ClassWriter;
use crate::context_source::{Entry, IContextSource};
use crate::decompiler_context::java_parse_int;
use crate::decompiler_context::DecompilerContext;
use crate::fernflower::DecompiledClass;
use crate::i_decompiled_data::IDecompiledData;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::rt::{rethrow, try_catch, Caught, JLinkedHashSet};
use crate::struct_class::StructClass;
use crate::struct_context::{is_io_exception, qualified_name_of};
use crate::text_buffer::TextBuffer;
use crate::{arena_identity, jthrow};

pub struct ContextUnitData {
    pub source: Rc<dyn IContextSource>,
    pub own: bool,
    pub root: bool,
    pub decompiled_data: Rc<dyn IDecompiledData>,
    pub entries_initialized: bool,
    pub class_entries: Vec<String>,
    pub dir_entries: Vec<String>,
    pub other_entries: Vec<Entry>,
    pub child_contexts: Vec<Rc<dyn IContextSource>>,
}

arena_object!(pub ContextUnit => ContextUnitData, Session);
arena_identity!(ContextUnit);

impl ContextUnit {
    /// Java: ContextUnit(IContextSource, boolean, boolean, IResultSaver, IDecompiledData)
    pub fn new(
        source: Rc<dyn IContextSource>,
        own: bool,
        root: bool,
        decompiled_data: Rc<dyn IDecompiledData>,
    ) -> ContextUnit {
        ContextUnit::alloc(ContextUnitData {
            source,
            own,
            root,
            decompiled_data,
            entries_initialized: false,
            class_entries: Vec::new(),
            dir_entries: Vec::new(),
            other_entries: Vec::new(),
            child_contexts: Vec::new(),
        })
    }

    fn source(self) -> Rc<dyn IContextSource> {
        self.with(|d| d.source.clone())
    }

    fn init_entries(self) {
        if self.is_lazy() {
            return;
        }

        if self.with(|d| d.entries_initialized) {
            return;
        }

        let source = self.source();
        let entries = source.get_entries();
        let class_entries: Vec<String> = entries
            .classes()
            .iter()
            .filter(|ent| ent.multirelease() == Entry::BASE_VERSION)
            .map(|ent| ent.base_path().to_string())
            .collect();
        let dir_entries: Vec<String> = entries.directories().to_vec();
        let include_extras =
            !DecompilerContext::get_option(IFernflowerPreferences::SKIP_EXTRA_FILES);
        let mut other_entries = Vec::new();
        for entry in entries.others().iter() {
            if entry.base_path() == "fernflower_abstract_parameter_names.txt" {
                let result = try_catch(|| source.get_input_stream(&entry.path()));
                match result {
                    Ok(Some(data)) => {
                        let text = String::from_utf8_lossy(&data).into_owned();
                        DecompilerContext::get_struct_context().load_abstract_metadata(&text);
                    }
                    Ok(None) => jthrow!(NullPointerException; "getInputStream returned null"),
                    Err(ex) if is_io_exception(&ex) => {
                        DecompilerContext::get_logger().write_message_ex(
                            "Failed to load abstract parameter names file",
                            Severity::Error,
                            &ex,
                        );
                    }
                    Err(ex) => rethrow(ex),
                }
            } else if include_extras {
                other_entries.push(entry.clone());
            }
        }
        let child_contexts: Vec<Rc<dyn IContextSource>> = entries.child_contexts().to_vec();
        self.with_mut(|d| {
            d.class_entries = class_entries;
            d.dir_entries = dir_entries;
            d.other_entries = other_entries;
            d.child_contexts = child_contexts;
            d.entries_initialized = true;
        });
    }

    pub fn get_class_names(self) -> Vec<String> {
        self.init_entries();
        self.with(|d| d.class_entries.clone())
    }

    pub fn has_class(self, class_name: &str) -> bool {
        self.source().has_class(class_name)
    }

    pub fn get_class_bytes(self, class_name: &str) -> Option<Vec<u8>> {
        self.source().get_class_bytes(class_name)
    }

    pub fn try_load_class(self, class_name: &str) -> Option<StructClass> {
        let data = self.get_class_bytes(class_name)?;

        Some(StructClass::create(
            &mut ByteCursor::new(data),
            self.is_own(),
        ))
    }

    pub fn get_directory_names(self) -> Vec<String> {
        self.init_entries();
        self.with(|d| d.dir_entries.clone())
    }

    pub fn get_other_entries(self) -> Vec<Entry> {
        self.init_entries();
        self.with(|d| d.other_entries.clone())
    }

    pub fn get_child_contexts(self) -> Vec<Rc<dyn IContextSource>> {
        self.init_entries();
        self.with(|d| d.child_contexts.clone())
    }

    pub fn get_name(self) -> String {
        self.source().get_name()
    }

    pub fn clear(self) {
        self.with_mut(|d| {
            d.entries_initialized = false;
            d.class_entries = Vec::new();
            d.dir_entries = Vec::new();
            d.other_entries = Vec::new();
        });
    }

    /// Decompiles the unit's classes and hands each result to `sink`.
    ///
    /// Java: save(Function<String, StructClass>)
    pub fn save(
        self,
        loader: &dyn Fn(&str) -> Option<StructClass>,
        sink: &mut dyn FnMut(DecompiledClass),
    ) {
        if !self.is_own() {
            jthrow!(IllegalStateException; "Trying to save non-own unit?");
        }

        self.init_entries();

        let root_context = DecompilerContext::get_current_context();

        let threads_property = DecompilerContext::get_property(IFernflowerPreferences::THREADS);
        let mut _threads = match threads_property.as_deref().and_then(java_parse_int) {
            Some(threads) => threads,
            None => {
                jthrow!(NumberFormatException; "For input string: \"{}\"", threads_property.unwrap_or_else(|| "null".to_string()))
            }
        };
        if _threads <= 0 {
            _threads = std::thread::available_parallelism().map_or(1, |n| n.get() as i32);
        }

        let (class_entries, decompiled_data) =
            self.with(|d| (d.class_entries.clone(), d.decompiled_data.clone()));
        let mut to_dump: Vec<ClassContext> = Vec::with_capacity(class_entries.len());
        let mut seen: JLinkedHashSet<String> = JLinkedHashSet::new();

        for class_entry in &class_entries {
            let cl = loader(class_entry).expect("loader.apply(classEntry)");
            let entry_name = decompiled_data.get_class_entry_name(cl, class_entry);
            if let Some(entry_name) = entry_name {
                let qualified_name = qualified_name_of(cl);
                if seen.add(qualified_name.clone()) {
                    to_dump.push(ClassContext::new(cl, entry_name));
                } else {
                    DecompilerContext::get_logger().write_message(
                        &format!("Skipping writing already existing class: {qualified_name}"),
                        Severity::Error,
                    );
                }
            }
        }

        let mut futures: Vec<Result<(), Caught>> = Vec::new();
        for class_ctx in to_dump.iter_mut() {
            futures.push(Self::run_on_worker(|| {
                self.set_context(root_context.as_ref());
                class_ctx.ctx = DecompilerContext::get_current_context();
                let processed = try_catch(|| decompiled_data.process_class(class_ctx.cl));
                if let Err(thr) = processed {
                    DecompilerContext::get_logger().write_message_throwable(
                        &format!(
                            "Class {} couldn't be fully decompiled.",
                            qualified_name_of(class_ctx.cl)
                        ),
                        &thr,
                    );
                    class_ctx.on_error(thr);
                }
                DecompilerContext::set_current_context(None);
            }));
        }

        Self::wait_for_all(futures);
        let mut futures: Vec<Result<(), Caught>> = Vec::new();

        for class_ctx in to_dump.iter_mut() {
            if let Some(pending_error) = &class_ctx.pending_error {
                let mut buf = TextBuffer::new();
                ClassWriter::write_exception(&mut buf, pending_error);
                class_ctx.class_content = Some(buf.convert_to_string_and_allow_data_discard());
                continue;
            }

            futures.push(Self::run_on_worker(|| {
                DecompilerContext::set_current_context(class_ctx.ctx.clone());
                class_ctx.class_content = decompiled_data.get_class_content(class_ctx.cl);
                if DecompilerContext::get_option(IFernflowerPreferences::BYTECODE_SOURCE_MAPPING) {
                    class_ctx.mapping = Some(
                        DecompilerContext::get_bytecode_source_mapper()
                            .get_original_lines_mapping(),
                    );
                }
            }));
        }

        Self::wait_for_all(futures);

        for cls in to_dump {
            if let Some(class_content) = cls.class_content {
                sink(DecompiledClass {
                    qualified_name: qualified_name_of(cls.cl),
                    path: cls.entry_name,
                    content: class_content,
                    mapping: cls.mapping,
                });
            }
        }
    }

    /// Runs one pool task as a worker thread would: with no current context of
    /// its own, restoring the calling thread's context afterwards.
    fn run_on_worker(task: impl FnOnce()) -> Result<(), Caught> {
        let caller_context = DecompilerContext::get_current_context();
        DecompilerContext::set_current_context(None);
        let result = try_catch(task);
        DecompilerContext::set_current_context(caller_context);
        result
    }

    /// `None` stands for a null root context.
    pub fn set_context(self, root_context: Option<&DecompilerContext>) {
        let current = DecompilerContext::get_current_context();
        if current.is_none() {
            let root_context = root_context.expect("rootContext");
            let (
                properties,
                logger,
                struct_context,
                class_processor,
                pool_interceptor,
                renamer_factory,
            ) = root_context.with(|d| {
                (
                    d.properties.clone(),
                    d.logger.clone(),
                    d.struct_context,
                    d.class_processor,
                    d.pool_interceptor,
                    d.renamer_factory.clone(),
                )
            });
            let current = DecompilerContext::new(
                properties,
                logger,
                struct_context,
                class_processor,
                pool_interceptor,
            );
            current.with_mut(|d| d.renamer_factory = renamer_factory);
            DecompilerContext::set_current_context(Some(current));
        }
    }

    fn wait_for_all(futures: Vec<Result<(), Caught>>) {
        for future in futures.into_iter().rev() {
            if let Err(cause) = future {
                if CanceledException::is(&cause) {
                    rethrow(cause);
                }
                jthrow!(RuntimeException; "java.util.concurrent.ExecutionException: {}", cause.message);
            }
        }
    }

    pub fn is_own(self) -> bool {
        self.with(|d| d.own)
    }

    pub fn is_root(self) -> bool {
        self.with(|d| d.root)
    }

    pub fn is_lazy(self) -> bool {
        let (own, source) = self.with(|d| (d.own, d.source.clone()));
        !own && source.is_lazy()
    }

    /// The source is released with its last reference; only the entries are reset here.
    pub(crate) fn close(self) {
        self.clear();
    }

    pub fn get_stream(self, entry: &Entry) -> Option<Vec<u8>> {
        self.source().get_input_stream(&entry.path())
    }
}

struct ClassContext {
    cl: StructClass,
    ctx: Option<DecompilerContext>,
    entry_name: String,
    class_content: Option<String>,
    mapping: Option<Vec<i32>>,
    pending_error: Option<Caught>,
}

impl ClassContext {
    fn new(cl: StructClass, entry_name: String) -> ClassContext {
        ClassContext {
            cl,
            ctx: None,
            entry_name,
            class_content: None,
            mapping: None,
            pending_error: None,
        }
    }

    /// Later errors would be suppressed exceptions of the first; only the first is kept.
    fn on_error(&mut self, thr: Caught) {
        if self.pending_error.is_none() {
            self.pending_error = Some(thr);
        }
    }
}
