//! Shared test support: a decompilation context installed on the current
//! thread the way `Fernflower::new` sets one up, over an empty class
//! universe (no JDK runtime is added; add sources through
//! `guard.struct_context().add_space(..)`). Whole runs belong to `Fernflower`.

#![allow(dead_code)]

use std::cell::OnceCell;
use std::rc::Rc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::classes_processor::ClassesProcessor;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_decompiled_data::IDecompiledData;
use kuna_jvm::i_fernflower_logger::{CollectingLogger, IFernflowerLogger, Severity};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::pool_interceptor::PoolInterceptor;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_context::StructContext;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_processor::VarProcessor;

/// The installed context; dropping it uninstalls the context and drops every
/// arena object of this thread (`rt::reset_session_scope`).
pub struct ContextGuard {
    context: DecompilerContext,
    logger: Rc<CollectingLogger>,
}

impl ContextGuard {
    pub fn logger(&self) -> Rc<CollectingLogger> {
        self.logger.clone()
    }

    pub fn context(&self) -> &DecompilerContext {
        &self.context
    }

    pub fn struct_context(&self) -> StructContext {
        self.context.with(|d| d.struct_context)
    }

    pub fn class_processor(&self) -> ClassesProcessor {
        self.context.with(|d| d.class_processor)
    }
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        DecompilerContext::set_current_context(None);
        kuna_jvm::rt::reset_session_scope();
    }
}

/// Vineflower's default options.
pub fn install_default_context() -> ContextGuard {
    install_context_with(&[])
}

/// The defaults overlaid, in order, with `options` (long option keys).
pub fn install_context_with(options: &[(&str, &str)]) -> ContextGuard {
    let mut properties = IFernflowerPreferences::DEFAULTS();
    for (key, value) in options {
        properties.put(key.to_string(), value.to_string());
    }

    let logger = Rc::new(CollectingLogger::new());
    if let Some(level) = properties.get_cloned(&IFernflowerPreferences::LOG_LEVEL.to_string()) {
        if let Some(severity) = Severity::value_of(&level.to_uppercase()) {
            logger.set_severity(severity);
        }
    }

    let data = Rc::new(ProcessorData(OnceCell::new()));
    let struct_context = StructContext::new(data.clone());
    let class_processor = ClassesProcessor::new(struct_context);
    let _ = data.0.set(class_processor);

    let rename = properties
        .get(&IFernflowerPreferences::RENAME_ENTITIES.to_string())
        .map(String::as_str)
        == Some("1");
    let interceptor = rename.then(PoolInterceptor::new);

    let context = DecompilerContext::new(
        properties,
        logger.clone(),
        struct_context,
        class_processor,
        interceptor,
    );
    DecompilerContext::set_current_context(Some(context.clone()));

    let plugins = struct_context.get_plugin_context();
    plugins.find_plugins();
    plugins.initialize();

    ContextGuard { context, logger }
}

/// The `IDecompiledData` the struct context holds: classes are processed by
/// the context's `ClassesProcessor`, nothing is written.
struct ProcessorData(OnceCell<ClassesProcessor>);

impl IDecompiledData for ProcessorData {
    fn get_class_entry_name(&self, _cl: StructClass, _entry_name: &str) -> Option<String> {
        None
    }

    fn process_class(&self, cl: StructClass) {
        self.0.get().expect("classProcessor").process_class(cl);
    }

    fn get_class_content(&self, _cl: StructClass) -> Option<String> {
        None
    }
}

/// `straight(II)I` of the committed fixture class `K2Shapes` (`data/k2_graph_oracle`).
pub fn fixture_method() -> StructMethod {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/k2_graph_oracle/classes/K2Shapes.class"
    );
    let bytes = std::fs::read(path).unwrap();
    StructClass::create(&mut ByteCursor::new(bytes), true)
        .get_method("straight", "(II)I")
        .expect("fixture method")
}

/// `ClassWrapper.init`'s per-method setup: a fresh var processor and counter container.
pub fn start_method(mt: StructMethod) {
    DecompilerContext::start_method(VarProcessor::new(mt, &mt.method_descriptor()));
}

/// Vineflower's options with `TRY_LOOP_FIX`, the one these hand-built graphs reach, off.
pub fn oracle_context() -> ContextGuard {
    install_context_with(&[(IFernflowerPreferences::TRY_LOOP_FIX, "0")])
}
