//! Port of `org/jetbrains/java/decompiler/main/rels/ClassWrapper.java`.
//!
//! Methods are decompiled on the calling thread. With `max-time-per-method`
//! set, Vineflower runs each `MethodProcessor` on a watchdog thread and kills
//! it at the deadline (`killThread`, not ported); here that branch runs the
//! same schedule inline under a cooperative deadline that the schedule's
//! cancelation polls enforce, and a method that overruns it ends as the
//! killed one does (see [`ClassWrapper::code_to_java_on_watchdog`]).

use std::fmt;
use std::panic::{self, AssertUnwindSafe};
use std::time::{Duration, Instant};

use crate::cancelation_manager::{CancelationManager, CanceledException, MethodDeadlineExceeded};
use crate::classes_processor::ClassNode;
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::{java_parse_int, DecompilerContext};
use crate::exprent::Exprent;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::method_descriptor::MethodDescriptor;
use crate::method_processor::MethodProcessor;
use crate::method_properties::MethodProperties;
use crate::method_properties_processor::MethodPropertiesProcessor;
use crate::method_wrapper::MethodWrapper;
use crate::language_spec::LanguageSpec;
use crate::root_statement::RootStatement;
use crate::rt::{rethrow, try_catch, Caught, JHashMap, JHashSet, Shared};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_names_collector::VarNamesCollector;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;
use crate::vb_style_collection::VBStyleCollection;
use crate::{arena_identity, arena_object, jthrow};

pub struct ClassWrapperData {
    pub class_struct: StructClass,
    pub node: ClassNode,
    pub hidden_members: Shared<JHashSet<String>>,
    pub static_field_initializers: Shared<VBStyleCollection<Exprent, String>>,
    pub dynamic_field_initializers: Shared<VBStyleCollection<Exprent, String>>,
    pub methods: Shared<VBStyleCollection<MethodWrapper, String>>,
    pub method_properties: Shared<JHashMap<String, MethodProperties>>,
}

arena_object!(pub ClassWrapper => ClassWrapperData, Class);
arena_identity!(ClassWrapper);

impl ClassWrapper {
    const DEBUG_METHOD_FILTER: Option<&'static str> = None;

    pub fn new(node: ClassNode) -> ClassWrapper {
        let class_struct = node.with(|d| d.class_struct);
        ClassWrapper::alloc(ClassWrapperData {
            class_struct,
            node,
            hidden_members: Shared::new(JHashSet::new()),
            static_field_initializers: Shared::new(VBStyleCollection::new()),
            dynamic_field_initializers: Shared::new(VBStyleCollection::new()),
            methods: Shared::new(VBStyleCollection::new()),
            method_properties: Shared::new(JHashMap::new()),
        })
    }

    /// Decompiles every method of the class into a [`MethodWrapper`].
    pub fn init(self, spec: Option<&LanguageSpec>) {
        let (class_struct, node) = self.with(|d| (d.class_struct, d.node));
        DecompilerContext::set_current_class(Some(class_struct));
        DecompilerContext::set_current_class_wrapper(Some(self));
        DecompilerContext::set_current_class_node(Some(node));
        DecompilerContext::get_logger().start_class(&class_struct.qualified_name());

        let max_sec_property =
            DecompilerContext::get_property(IFernflowerPreferences::MAX_PROCESSING_METHOD)
                .expect("getProperty(MAX_PROCESSING_METHOD)");
        let max_sec = java_parse_int(&max_sec_property).unwrap_or_else(
            || jthrow!(NumberFormatException; "For input string: \"{}\"", max_sec_property),
        );
        let test_mode = DecompilerContext::get_option(IFernflowerPreferences::UNIT_TEST_MODE);

        let methods = class_struct.get_methods().to_vec();
        for &mt in &methods {
            let mut props = MethodProperties::new(mt);
            MethodPropertiesProcessor::parse(mt, class_struct, &mut props);
            let key = InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor());
            self.get_method_properties_map()
                .borrow_mut()
                .put(key, props);
        }

        for mt in methods {
            DecompilerContext::get_logger().start_method(&format!(
                "{} {}",
                mt.get_name(),
                mt.get_descriptor()
            ));

            let md = MethodDescriptor::parse_descriptor_struct(mt, None);
            let var_proc = VarProcessor::new(mt, &md);
            DecompilerContext::start_method(var_proc);

            let vc = var_proc.get_var_names_collector();
            let counter = DecompilerContext::get_counter_container();

            let mut root: Option<RootStatement> = None;

            let mut error: Option<Caught> = None;

            if let Some(filter) = Self::DEBUG_METHOD_FILTER {
                if filter != mt.get_name() {
                    let method_wrapper =
                        MethodWrapper::new(None, var_proc, mt, class_struct, counter);
                    self.get_methods().borrow_mut().add_with_key(
                        method_wrapper,
                        InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor()),
                    );
                    DecompilerContext::get_logger().end_method();

                    continue;
                }
            }

            let processed = try_catch(|| {
                if mt.contains_code() {
                    if max_sec == 0 || test_mode {
                        Some(MethodProcessor::code_to_java(
                            class_struct,
                            mt,
                            &md,
                            var_proc,
                            spec,
                        ))
                    } else {
                        match Self::code_to_java_on_watchdog(
                            class_struct,
                            mt,
                            &md,
                            var_proc,
                            spec,
                            max_sec,
                        ) {
                            Ok(root) => Some(root),
                            Err(timeout) => {
                                error = Some(timeout);
                                None
                            }
                        }
                    }
                } else {
                    Self::name_parameters(class_struct, mt, &md, var_proc, vc);
                    None
                }
            });
            match processed {
                Ok(result) => root = result,
                Err(t) => {
                    if CanceledException::is(&t) {
                        rethrow(t);
                    }
                    let message = format!(
                        "Method {} {} in class {} couldn't be decompiled.",
                        mt.get_name(),
                        mt.get_descriptor(),
                        class_struct.qualified_name()
                    );
                    DecompilerContext::get_logger().write_message_ex(&message, Severity::Warn, &t);
                    error = Some(t);
                }
            }

            let method_wrapper = MethodWrapper::new(root, var_proc, mt, class_struct, counter);
            method_wrapper.with_mut(|d| d.decompile_error = error.clone());

            self.get_methods().borrow_mut().add_with_key(
                method_wrapper,
                InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor()),
            );

            if error.is_none() {
                let names_collector = VarNamesCollector::new();
                for f in class_struct.get_fields().to_vec() {
                    names_collector.add_name(f.get_name());
                }
                var_proc.refresh_var_names(names_collector);

                if DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
                    let attr = mt.get_local_variable_attr();
                    if let Some(attr) = attr {
                        if let Some(root) = root {
                            var_proc.set_debug_var_names(root, &attr.get_map_names());
                        }
                    }
                }
            }

            DecompilerContext::get_logger().end_method();
        }

        DecompilerContext::get_logger().end_class();
    }

    /// The `max-time-per-method` branch. Java runs the schedule on a watchdog
    /// thread, waking every 200 ms; past `maxSec` seconds it logs "Processing
    /// time limit exceeded for method …", kills the thread and records a new
    /// `TimeoutException` as the method's error, leaving it without a body.
    /// Here the deadline is armed for the schedule's polls; `Err` is that
    /// `TimeoutException`. Anything else the schedule throws passes through
    /// untouched (with its original location), as the Java rethrows it.
    fn code_to_java_on_watchdog(
        cl: StructClass,
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
        spec: Option<&LanguageSpec>,
        max_sec: i32,
    ) -> Result<RootStatement, Caught> {
        let budget = Duration::from_millis((max_sec as i64 * 1000).max(200) as u64);
        let previous = CancelationManager::set_method_deadline(Some(Instant::now() + budget));
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            MethodProcessor::code_to_java(cl, mt, md, var_proc, spec)
        }));
        CancelationManager::set_method_deadline(previous);
        match result {
            Ok(root) => Ok(root),
            Err(payload) if MethodDeadlineExceeded::is_payload(payload.as_ref()) => {
                let message = format!(
                    "Processing time limit exceeded for method {}, execution interrupted.",
                    mt.get_name()
                );
                DecompilerContext::get_logger().write_message(&message, Severity::Error);
                Err(try_catch(|| crate::rt::throw("TimeoutException", "")).unwrap_err())
            }
            Err(payload) => panic::resume_unwind(payload),
        }
    }

    /// The no-code branch of `init`: `this` and the parameters get `var<index>` names.
    fn name_parameters(
        class_struct: StructClass,
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
        vc: VarNamesCollector,
    ) {
        let this_var = !mt.has_modifier(ACC_STATIC);

        let mut param_count = 0;
        if this_var {
            var_proc
                .get_this_vars()
                .borrow_mut()
                .put(VarVersionPair::new(0, 0), class_struct.qualified_name());
            param_count = 1;
        }
        param_count += md.params.len() as i32;

        let mut var_index = 0;
        for i in 0..param_count {
            var_proc.set_var_name(
                VarVersionPair::new(var_index, 0),
                vc.get_free_name_index(var_index),
            );
            var_proc.mark_param(VarVersionPair::new(var_index, 0));

            if this_var {
                if i == 0 {
                    var_index += 1;
                } else {
                    var_index += md.params[(i - 1) as usize].stack_size;
                }
            } else {
                var_index += md.params[i as usize].stack_size;
            }
        }
    }

    /// Java: getMethodWrapper(String, String)
    pub fn get_method_wrapper(self, name: &str, descriptor: &str) -> Option<MethodWrapper> {
        let key = InterpreterUtil::make_unique_key(name, descriptor);
        self.get_methods().borrow().get_with_key(&key)
    }

    /// Java: getMethodProperties(String, String)
    pub fn get_method_properties(self, name: &str, descriptor: &str) -> Option<MethodProperties> {
        let key = InterpreterUtil::make_unique_key(name, descriptor);
        self.with(|d| d.method_properties.borrow().get_cloned(&key))
    }

    /// Java: getMethodWrapper(int)
    pub fn get_method_wrapper_index(self, index: i32) -> MethodWrapper {
        self.get_methods().borrow().get(index as usize)
    }

    pub fn get_class_struct(self) -> StructClass {
        self.with(|d| d.class_struct)
    }

    pub fn get_methods(self) -> Shared<VBStyleCollection<MethodWrapper, String>> {
        self.with(|d| d.methods.clone())
    }

    /// Java: getMethodProperties()
    pub fn get_method_properties_map(self) -> Shared<JHashMap<String, MethodProperties>> {
        self.with(|d| d.method_properties.clone())
    }

    pub fn get_hidden_members(self) -> Shared<JHashSet<String>> {
        self.with(|d| d.hidden_members.clone())
    }

    pub fn get_static_field_initializers(self) -> Shared<VBStyleCollection<Exprent, String>> {
        self.with(|d| d.static_field_initializers.clone())
    }

    pub fn get_dynamic_field_initializers(self) -> Shared<VBStyleCollection<Exprent, String>> {
        self.with(|d| d.dynamic_field_initializers.clone())
    }
}

impl fmt::Display for ClassWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class_struct = self.get_class_struct();
        let qualified_name = class_struct.qualified_name();
        f.write_str(&qualified_name)
    }
}
