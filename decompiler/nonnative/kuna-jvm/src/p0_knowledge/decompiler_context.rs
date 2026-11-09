//! Port of `org/jetbrains/java/decompiler/main/DecompilerContext.java`.
//!
//! The current context is a thread-local `Option<DecompilerContext>`; a
//! context is a shared (`Rc`) object, so installing the same value on several
//! tasks aliases one state, as in Java. Every static accessor takes a short
//! borrow and returns an owned or `Copy` value, so nested calls never collide.
//! The `Key`-indexed `staticProps` map becomes the typed [`ContextProperties`].

use std::cell::RefCell;
use std::rc::Rc;

use crate::bytecode_source_mapper::BytecodeSourceMapper;
use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassesProcessor};
use crate::counter_container::CounterContainer;
use crate::i_fernflower_logger::IFernflowerLogger;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::import_collector::ImportCollector;
use crate::method_wrapper::MethodWrapper;
use crate::pool_interceptor::PoolInterceptor;
use crate::rt::{jchar, JHashMap, JavaStr};
use crate::struct_class::StructClass;
use crate::struct_context::StructContext;
use crate::var_processor::VarProcessor;

/// `staticProps`: the values stored under `CURRENT_CLASS`, `CURRENT_CLASS_WRAPPER`,
/// `CURRENT_CLASS_NODE`, `CURRENT_METHOD_WRAPPER` and `CURRENT_VAR_PROCESSOR`.
#[derive(Clone, Default)]
pub struct ContextProperties {
    pub current_class: Option<StructClass>,
    pub current_class_wrapper: Option<ClassWrapper>,
    pub current_class_node: Option<ClassNode>,
    pub current_method_wrapper: Option<MethodWrapper>,
    pub current_var_processor: Option<VarProcessor>,
}

pub struct DecompilerContextData {
    pub static_props: ContextProperties,
    pub properties: JHashMap<String, String>,
    pub logger: Rc<dyn IFernflowerLogger>,
    pub struct_context: StructContext,
    pub class_processor: ClassesProcessor,
    pub pool_interceptor: Option<PoolInterceptor>,
    pub renamer_factory: Option<Rc<dyn IVariableNamingFactory>>,
    import_collector: Option<ImportCollector>,
    var_processor: Option<VarProcessor>,
    counter_container: CounterContainer,
    bytecode_source_mapper: Option<BytecodeSourceMapper>,
}

#[derive(Clone)]
pub struct DecompilerContext(Rc<RefCell<DecompilerContextData>>);

thread_local! {
    static CURRENT_CONTEXT: RefCell<Option<DecompilerContext>> = const { RefCell::new(None) };
    static KEY_BUFFER: RefCell<String> = const { RefCell::new(String::new()) };
}

impl DecompilerContext {
    pub fn new(
        properties: JHashMap<String, String>,
        logger: Rc<dyn IFernflowerLogger>,
        struct_context: StructContext,
        class_processor: ClassesProcessor,
        interceptor: Option<PoolInterceptor>,
    ) -> DecompilerContext {
        DecompilerContext(Rc::new(RefCell::new(DecompilerContextData {
            static_props: ContextProperties::default(),
            properties,
            logger,
            struct_context,
            class_processor,
            pool_interceptor: interceptor,
            renamer_factory: None,
            import_collector: None,
            var_processor: None,
            counter_container: CounterContainer::new(),
            bytecode_source_mapper: None,
        })))
    }

    /// Read this context's fields; the closure must not reach the context again.
    pub fn with<R>(&self, f: impl FnOnce(&DecompilerContextData) -> R) -> R {
        let Ok(data) = self.0.try_borrow() else {
            crate::rt::arena::borrow_conflict("DecompilerContext", 0, false)
        };
        f(&data)
    }

    /// Mutate this context's fields; the closure must not reach the context again.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut DecompilerContextData) -> R) -> R {
        let Ok(mut data) = self.0.try_borrow_mut() else {
            crate::rt::arena::borrow_conflict("DecompilerContext", 0, true)
        };
        f(&mut data)
    }

    /// Java reference `==`.
    pub fn ptr_eq(&self, other: &DecompilerContext) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn get_current_context() -> Option<DecompilerContext> {
        CURRENT_CONTEXT.with(|c| c.borrow().clone())
    }

    pub fn set_current_context(context: Option<DecompilerContext>) {
        let previous = CURRENT_CONTEXT.with(|c| std::mem::replace(&mut *c.borrow_mut(), context));
        drop(previous);
    }

    fn current<R>(f: impl FnOnce(&DecompilerContextData) -> R) -> R {
        CURRENT_CONTEXT.with(|c| {
            let current = c.borrow();
            let Some(context) = current.as_ref() else {
                crate::jthrow!(NullPointerException; "no current DecompilerContext");
            };
            context.with(f)
        })
    }

    fn current_mut<R>(f: impl FnOnce(&mut DecompilerContextData) -> R) -> R {
        let Some(context) = Self::get_current_context() else {
            crate::jthrow!(NullPointerException; "no current DecompilerContext");
        };
        context.with_mut(f)
    }

    /// Java: setProperty(String, Object)
    pub fn set_property(key: &str, value: impl Into<String>) {
        let value = value.into();
        Self::current_mut(|d| {
            d.properties.put(key.to_string(), value);
        })
    }

    pub fn start_class(import_collector: ImportCollector) {
        let context = Self::get_current_context().expect("getCurrentContext()");
        context.with_mut(|d| d.import_collector = Some(import_collector));
        let counter_container = CounterContainer::new();
        context.with_mut(|d| d.counter_container = counter_container);
        let bytecode_source_mapper = BytecodeSourceMapper::new();
        context.with_mut(|d| d.bytecode_source_mapper = Some(bytecode_source_mapper));
    }

    pub fn start_method(var_processor: VarProcessor) {
        let context = Self::get_current_context().expect("getCurrentContext()");
        context.with_mut(|d| d.var_processor = Some(var_processor));
        let counter_container = CounterContainer::new();
        context.with_mut(|d| d.counter_container = counter_container);
    }

    pub fn reset_method(wrapper: MethodWrapper) {
        let context = Self::get_current_context().expect("getCurrentContext()");
        let (varproc, counter) = wrapper.with(|w| (w.varproc, w.counter));
        context.with_mut(|d| {
            d.var_processor = Some(varproc);
            d.counter_container = counter;
        });
    }

    pub fn set_import_collector(import_collector: ImportCollector) {
        Self::current_mut(|d| d.import_collector = Some(import_collector))
    }

    pub fn get_property(key: &str) -> Option<String> {
        Self::current(|d| lookup(&d.properties, key, |v| v.map(str::to_string)))
    }

    pub fn get_option(key: &str) -> bool {
        Self::current(|d| lookup(&d.properties, key, |v| v == Some("1")))
    }

    pub fn get_int_option(key: &str) -> i32 {
        Self::current(|d| {
            lookup(&d.properties, key, |v| {
                v.and_then(java_parse_int).unwrap_or(0)
            })
        })
    }

    pub fn get_new_line_separator() -> &'static str {
        if Self::get_option(IFernflowerPreferences::NEW_LINE_SEPARATOR) {
            IFernflowerPreferences::LINE_SEPARATOR_UNX
        } else {
            IFernflowerPreferences::LINE_SEPARATOR_WIN
        }
    }

    pub fn get_logger() -> Rc<dyn IFernflowerLogger> {
        Self::current(|d| d.logger.clone())
    }

    pub fn get_struct_context() -> StructContext {
        Self::current(|d| d.struct_context)
    }

    pub fn get_class_processor() -> ClassesProcessor {
        Self::current(|d| d.class_processor)
    }

    pub fn get_pool_interceptor() -> Option<PoolInterceptor> {
        Self::current(|d| d.pool_interceptor)
    }

    pub fn get_naming_factory() -> Rc<dyn IVariableNamingFactory> {
        Self::current(|d| d.renamer_factory.clone()).expect("getNamingFactory()")
    }

    pub fn get_import_collector() -> ImportCollector {
        Self::current(|d| d.import_collector).expect("getImportCollector()")
    }

    pub fn get_var_processor() -> VarProcessor {
        Self::current(|d| d.var_processor).expect("getVarProcessor()")
    }

    pub fn get_counter_container() -> CounterContainer {
        Self::current(|d| d.counter_container)
    }

    pub fn get_bytecode_source_mapper() -> BytecodeSourceMapper {
        Self::current(|d| d.bytecode_source_mapper).expect("getBytecodeSourceMapper()")
    }

    pub fn get_current_class() -> Option<StructClass> {
        Self::current(|d| d.static_props.current_class)
    }

    pub fn set_current_class(value: Option<StructClass>) {
        Self::current_mut(|d| d.static_props.current_class = value)
    }

    pub fn get_current_class_wrapper() -> Option<ClassWrapper> {
        Self::current(|d| d.static_props.current_class_wrapper)
    }

    pub fn set_current_class_wrapper(value: Option<ClassWrapper>) {
        Self::current_mut(|d| d.static_props.current_class_wrapper = value)
    }

    pub fn get_current_class_node() -> Option<ClassNode> {
        Self::current(|d| d.static_props.current_class_node)
    }

    pub fn set_current_class_node(value: Option<ClassNode>) {
        Self::current_mut(|d| d.static_props.current_class_node = value)
    }

    pub fn get_current_method_wrapper() -> Option<MethodWrapper> {
        Self::current(|d| d.static_props.current_method_wrapper)
    }

    pub fn set_current_method_wrapper(value: Option<MethodWrapper>) {
        Self::current_mut(|d| d.static_props.current_method_wrapper = value)
    }

    pub fn get_current_var_processor() -> Option<VarProcessor> {
        Self::current(|d| d.static_props.current_var_processor)
    }

    pub fn set_current_var_processor(value: Option<VarProcessor>) {
        Self::current_mut(|d| d.static_props.current_var_processor = value)
    }

    /// This context's counters, as `getCounter` reads them.
    pub fn counter_values(&self) -> [i32; 4] {
        let counter = self.with(|d| d.counter_container);
        counter.with(|c| c.values)
    }

    /// Keeps this context usable after `rt::reset_class_scope`: class-scope
    /// handles that went stale are dropped (nothing reads them again) and the
    /// counter container is replaced by one holding `counters`.
    pub fn survive_class_scope_reset(&self, counters: [i32; 4]) {
        let counter_container = CounterContainer::new();
        counter_container.with_mut(|c| c.values = counters);
        self.with_mut(|d| {
            d.counter_container = counter_container;
            d.import_collector = d.import_collector.filter(|h| h.is_live());
            d.var_processor = d.var_processor.filter(|h| h.is_live());
            d.bytecode_source_mapper = d.bytecode_source_mapper.filter(|h| h.is_live());
            let props = &mut d.static_props;
            props.current_class_wrapper = props.current_class_wrapper.filter(|h| h.is_live());
            props.current_method_wrapper = props.current_method_wrapper.filter(|h| h.is_live());
            props.current_var_processor = props.current_var_processor.filter(|h| h.is_live());
        })
    }
}

/// A `properties.get(key)` without allocating a key string per call.
fn lookup<R>(
    properties: &JHashMap<String, String>,
    key: &str,
    f: impl FnOnce(Option<&str>) -> R,
) -> R {
    KEY_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        buffer.clear();
        buffer.push_str(key);
        f(properties.get(&*buffer).map(String::as_str))
    })
}

/// `Integer.parseInt(s)`; `None` where Java throws `NumberFormatException`.
pub(crate) fn java_parse_int(s: &str) -> Option<i32> {
    let units = s.utf16();
    let len = units.len();
    if len == 0 {
        return None;
    }
    let mut negative = false;
    let mut i = 0;
    let mut limit = -i32::MAX;
    let first_char = units[0];
    if first_char < b'0' as u16 {
        if first_char == b'-' as u16 {
            negative = true;
            limit = i32::MIN;
        } else if first_char != b'+' as u16 {
            return None;
        }
        if len == 1 {
            return None;
        }
        i += 1;
    }
    let multmin = limit / 10;
    let mut result: i32 = 0;
    while i < len {
        let digit = jchar::digit(units[i], 10);
        i += 1;
        if digit < 0 || result < multmin {
            return None;
        }
        result *= 10;
        if result < limit + digit {
            return None;
        }
        result -= digit;
    }
    Some(if negative { result } else { -result })
}
