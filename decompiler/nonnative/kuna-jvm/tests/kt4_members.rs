//! KT4: the Kotlin plugin's member model (`KFunction`, `KProperty`,
//! `KConstructor`, `DefaultArgsMap`, `KotlinObjectProcessors`,
//! `KEnumProcessor`) against dumps made by the plugin's own classes
//! (`data/kt4/KtMembersDump.java`; regenerate with `data/kt4/gen.sh`) over the
//! kotlinc 2.0.0 fixture in `data/kt4/classes`. Every run decompiles with the
//! Java pipeline and no plugins on both sides.
//!
//! `members` needs `KType.from`, `KTypeParameter.from`, `KContract.from`,
//! `KTypes.getJavaSignature`, `KotlinWriter.searchForMethod` and
//! `KotlinChooser.parseMetadataFor` from other slices; while one of them is
//! still `unported!` the test reports which and passes.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::code_constants::ACC_ENUM;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::default_args_map::{DefaultArgsMap, DefaultArgsSupplier};
use kuna_jvm::i_fernflower_logger::{CollectingLogger, Severity};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::initializer_processor::InitializerProcessor;
use kuna_jvm::k_class::KClass;
use kuna_jvm::k_constructor::KConstructor;
use kuna_jvm::k_element::KElement;
use kuna_jvm::k_enum_processor::KEnumProcessor;
use kuna_jvm::k_file::KFile;
use kuna_jvm::k_function::{KFunction, MethodSupplier};
use kuna_jvm::k_function_reference::KFunctionReference;
use kuna_jvm::k_multifile_facade::KMultifileFacade;
use kuna_jvm::k_parameter::KParameter;
use kuna_jvm::k_property::{KProperty, KPropertyAnnotation, PropertyMethod};
use kuna_jvm::k_type::KType;
use kuna_jvm::k_type_parameter::KTypeParameter;
use kuna_jvm::kotlin_chooser::KotlinChooser;
use kuna_jvm::kotlin_object_processors::KotlinObjectProcessors;
use kuna_jvm::kt_attrs::KtAttrs;
use kuna_jvm::kt_flags::Flags;
use kuna_jvm::kt_metadata::{KotlinMetadataAnnotation, KotlinMetadataMessage, KtMetadata};
use kuna_jvm::method_wrapper::MethodWrapper;
use kuna_jvm::rt::{rethrow, try_catch, Caught};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_general_attribute::StructGeneralAttribute;
use kuna_jvm::struct_member::StructMember;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::text_buffer::TextBuffer;

use crate::common::{self, ContextGuard};

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt4")
}

fn golden(name: &str) -> Vec<String> {
    std::fs::read_to_string(data_dir().join(name))
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

fn install() -> ContextGuard {
    let guard = common::install_context_with(&[
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
        (IFernflowerPreferences::LOG_LEVEL, "warn"),
        ("kt-unknown-defaults", "..."),
        ("kt-show-public", "0"),
    ]);
    guard
        .struct_context()
        .add_space_file(&data_dir().join("classes"), true);
    guard
}

fn assert_lines(actual: &[String], expected: &[String]) {
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        assert_eq!(a, e, "line {}", i + 1);
    }
    assert_eq!(actual.len(), expected.len(), "line count");
}

/// The bodies of other slices this test runs through, when still `unported!`.
fn waiting_on(caught: &Caught) -> Option<String> {
    if !caught.is("UnportedError") {
        return None;
    }
    let body = caught.exception.as_ref().unwrap().message.clone();
    let own = [
        "KFunction.",
        "KProperty.",
        "KConstructor.",
        "DefaultArgsMap.",
        "KotlinObjectProcessors.",
        "KEnumProcessor.",
    ];
    assert!(
        !own.iter().any(|p| body.starts_with(p)),
        "KT4 body {body} is unported"
    );
    Some(body)
}

struct Dump {
    out: Vec<String>,
    logger: Rc<CollectingLogger>,
    logged: usize,
}

impl Dump {
    fn new(guard: &ContextGuard) -> Dump {
        Dump {
            out: Vec::new(),
            logger: guard.logger(),
            logged: 0,
        }
    }

    fn p(&mut self, line: String) {
        self.out.push(line);
    }

    fn flush_logs(&mut self) {
        let records = self.logger.records();
        let records = records.borrow();
        for record in &records[self.logged..] {
            let severity = match record.severity {
                Severity::Trace => "TRACE",
                Severity::Info => "INFO",
                Severity::Warn => "WARN",
                Severity::Error => "ERROR",
            };
            self.out.push(format!(
                "log\t{}",
                esc(&format!("{} {}", severity, record.message))
            ));
        }
        self.logged = records.len();
    }
}

fn esc(s: &str) -> String {
    let mut b = String::new();
    for c in s.encode_utf16() {
        if !(0x20..0x7f).contains(&c) || c == u16::from(b'\\') {
            b.push_str(&format!("\\u{:04x}", c));
        } else {
            b.push(c as u8 as char);
        }
    }
    b
}

/// `"!" + t.getClass().getSimpleName()`; an `unported!` callee propagates.
fn exc(caught: Caught) -> String {
    if caught.is("UnportedError") {
        rethrow(caught);
    }
    if let Some(e) = &caught.exception {
        return format!("!{}", e.class);
    }
    let class = caught.message.split(':').next().unwrap_or("");
    if class.ends_with("Exception") || class.ends_with("Error") {
        format!("!{}", class)
    } else {
        format!("!RustPanic {}", caught.message)
    }
}

fn text(buf: &TextBuffer) -> String {
    esc(&buf.convert_to_string_and_allow_data_discard())
}

fn type_(t: Option<&KType>) -> String {
    let Some(t) = t else {
        return "-".to_string();
    };
    let k = t.k();
    let mut s = format!(
        "{}{}|{}",
        k.kotlin_type,
        if k.is_nullable { "?" } else { "" },
        t.0
    );
    if let Some(name) = &k.type_parameter_name {
        s.push_str(&format!("|T={}", name));
    }
    if let Some(name) = &k.type_alias_name {
        s.push_str(&format!("|A={}", name));
    }
    s.push_str(&format!(
        "|args={}",
        k.type_arguments
            .as_ref()
            .map_or("-".to_string(), |a| a.len().to_string())
    ));
    s
}

fn param(p: &KParameter) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        p.name,
        p.flags,
        type_(Some(&p.type_)),
        type_(p.vararg_type.as_ref()),
        p.type_id
    )
}

fn params(ps: &[KParameter]) -> String {
    format!("[{}]", ps.iter().map(param).collect::<Vec<_>>().join(", "))
}

fn tparam(tp: &KTypeParameter) -> String {
    let bounds: Vec<String> = tp.upper_bounds.iter().map(|b| type_(Some(b))).collect();
    format!(
        "{}:{}:{}:{}:<{}>",
        tp.name,
        tp.reified,
        tp.variance.name(),
        tp.id,
        bounds.join(" & ")
    )
}

fn member(m: Option<StructMember>) -> String {
    let Some(m) = m else {
        return "-".to_string();
    };
    if let Some(mt) = m.try_as_method() {
        return format!("{} {}", mt.get_name(), mt.get_descriptor());
    }
    if let Some(fd) = m.try_as_field() {
        return format!("{} {}", fd.get_name(), fd.get_descriptor());
    }
    m.as_class().qualified_name()
}

fn ctor(c: &KConstructor) -> String {
    format!(
        "Constructor flags={} primary={} classFlags={} class={} method={} supplier={} params={}",
        c.flags,
        c.is_primary,
        c.class_flags,
        c.class_struct.qualified_name(),
        member(c.method_struct.map(Into::into)),
        c.method_supplier.is_some(),
        params(&c.parameters)
    )
}

fn annotations(p: &KProperty) -> String {
    let Some(annotations) = &p.annotations else {
        return "null".to_string();
    };
    let names: Vec<String> = annotations
        .iter()
        .map(|a| match a {
            KPropertyAnnotation::Deprecated => "DEPRECATED".to_string(),
            KPropertyAnnotation::Attribute(..) => a.get().get_class_name(),
        })
        .collect();
    format!("[{}]", names.join(","))
}

fn describe(e: Option<KElement>) -> Option<String> {
    Some(match e? {
        KElement::HiddenElement(h) => format!("Hidden {}", h.name()),
        KElement::Class(c) => {
            let companion = if c.proto.has_companion_object_name() {
                c.resolver
                    .as_ref()
                    .unwrap()
                    .resolve(c.proto.get_companion_object_name())
            } else {
                "-".to_string()
            };
            format!("Class flags={} companion={}", c.flags(), companion)
        }
        KElement::File(f) => format!(
            "File multifile={}",
            f.multifile_name.as_deref().unwrap_or("null")
        ),
        KElement::MultifileFacade(m) => format!("Facade {}", m.files.len()),
        KElement::FunctionReference(_) => "FunctionReference".to_string(),
        KElement::Function(f) => {
            if Rc::ptr_eq(&f, &KFunction::FAILED_LAMBDA()) {
                return Some("FailedLambda".to_string());
            }
            let ctx: Vec<String> = f
                .context_receiver_types
                .iter()
                .map(|t| type_(Some(t)))
                .collect();
            let tps: Vec<String> = f.type_parameters.iter().map(tparam).collect();
            format!(
                "Function name={} flags={} method={} class={} override={} receiver={} return={} ctx=[{}] tparams=[{}] contract={} params={}",
                f.name,
                f.flags,
                member(f.method_struct.map(Into::into)),
                f.class_struct.unwrap().qualified_name(),
                f.known_override,
                type_(f.receiver_type.as_ref()),
                type_(Some(&f.return_type)),
                ctx.join(", "),
                tps.join(", "),
                f.contract
                    .as_ref()
                    .map_or("-".to_string(), |c| c.effects.len().to_string()),
                params(&f.parameters)
            )
        }
        KElement::Property(p) => format!(
            "Property name={} flags={} type={} getter={} setter={} setterParam={} field={} initializer={} annotations={} class={}",
            p.name,
            p.flags,
            type_(p.type_.as_ref()),
            p.getter.as_ref().map_or("-".to_string(), |g| g.flags.to_string()),
            p.setter.as_ref().map_or("-".to_string(), |s| s.flags.to_string()),
            p.setter_param_name.as_deref().unwrap_or("null"),
            member(p.underlying_field.map(Into::into)),
            p.initializer.is_some(),
            annotations(&p),
            p.class_struct.qualified_name()
        ),
        KElement::Constructor(c) => ctor(&c),
    })
}

fn sorted_classes(guard: &ContextGuard) -> Vec<StructClass> {
    let mut classes = guard.struct_context().get_own_classes();
    classes.sort_by_key(|c| c.qualified_name());
    classes
}

fn dump_attrs(d: &mut Dump, cl: StructClass) {
    d.p(format!("class\t{}", cl.qualified_name()));
    if let Some(s) = describe(KtAttrs::get(cl, &KElement::KEY())) {
        d.p(format!("  self\t{}", s));
    }
    if let Some(primary) = KtAttrs::get(cl, &KConstructor::PRIMARY_CONSTRUCTOR()) {
        let same = primary.method_struct.is_some_and(|mt| {
            matches!(KtAttrs::get(mt, &KElement::KEY()),
                     Some(KElement::Constructor(c)) if Rc::ptr_eq(&c, &primary))
        });
        d.p(format!("  primary\t{} same={}", ctor(&primary), same));
    }
    for fd in cl.get_fields().to_vec() {
        if let Some(s) = describe(KtAttrs::get(fd, &KElement::KEY())) {
            d.p(format!("  field {}\t{}", member(Some(fd.into())), s));
        }
    }
    for mt in cl.get_methods().to_vec() {
        let s = describe(KtAttrs::get(mt, &KElement::KEY()));
        let pm = KtAttrs::get(mt, &PropertyMethod::KEY());
        if s.is_some() || pm.is_some() {
            d.p(format!(
                "  method {}\t{}{}",
                member(Some(mt.into())),
                s.as_deref().unwrap_or("null"),
                pm.map_or(String::new(), |pm| format!("\t{}", pm.name()))
            ));
        }
    }
}

fn tree(root: ClassNode) -> Vec<ClassNode> {
    fn collect(node: ClassNode, nodes: &mut Vec<ClassNode>) {
        if node.with(|d| d.type_) == ClassNodeType::Lambda {
            return;
        }
        nodes.push(node);
        for n in node.with(|d| d.nested.to_vec()) {
            collect(n, nodes);
        }
    }
    let mut nodes = Vec::new();
    collect(root, &mut nodes);
    nodes
}

fn enter_class(node: ClassNode) {
    DecompilerContext::set_current_class(Some(node.with(|d| d.class_struct)));
    DecompilerContext::set_current_class_wrapper(node.get_wrapper());
    DecompilerContext::set_current_class_node(Some(node));
}

fn wrapper_name(mw: Option<MethodWrapper>) -> String {
    match mw {
        None => "null".to_string(),
        Some(mw) => member(Some(mw.with(|d| d.method_struct).into())),
    }
}

/// `KotlinWriter.invokeProcessors` without the 1.4 class-reference pass.
fn invoke_processors(node: ClassNode, companion: bool) {
    let wrapper = node.get_wrapper().unwrap();
    let cl = wrapper.get_class_struct();
    InitializerProcessor::extract_initializers(wrapper);
    InitializerProcessor::hide_initalizers(wrapper);
    if cl.has_modifier(ACC_ENUM)
        && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM)
    {
        KEnumProcessor::clear_enum(wrapper);
    }
    if companion {
        KotlinObjectProcessors::parse_with_companion(node);
    }
    KotlinObjectProcessors::mark_object_instance(wrapper);
}

fn hidden(d: &mut Dump, wrapper: ClassWrapper) {
    let mut hidden = wrapper.get_hidden_members().borrow().to_vec();
    hidden.sort();
    if !hidden.is_empty() {
        d.p(format!("  hidden\t{}", esc(&hidden.join(","))));
    }
}

fn dump_defaults(
    d: &mut Dump,
    key: &str,
    wrapper: ClassWrapper,
    calling: Option<MethodWrapper>,
    defaults: impl FnOnce() -> DefaultArgsMap,
    ps: &[KParameter],
    flags: bool,
) {
    DecompilerContext::set_current_method_wrapper(calling);
    let label = if flags { "  defaults" } else { "  bridge" };
    match try_catch(defaults) {
        Ok(map) => {
            d.p(format!(
                "{} {}\t{}",
                label,
                key,
                member(map.get_default_method().map(Into::into))
            ));
            for p in ps {
                let v = match try_catch(|| map.to_java(p, 1)) {
                    Ok(buf) => text(&buf),
                    Err(e) => exc(e),
                };
                if flags {
                    d.p(format!(
                        "    {}\t{}\t{}",
                        p.name,
                        Flags::DECLARES_DEFAULT_VALUE.get(p.flags),
                        v
                    ));
                } else {
                    d.p(format!("    {}\t{}", p.name, v));
                }
            }
        }
        Err(e) => d.p(format!("{} {}\t{}", label, key, exc(e))),
    }
    let _ = wrapper;
    DecompilerContext::set_current_method_wrapper(None);
}

fn dump_wiring(d: &mut Dump, node: ClassNode) {
    let wrapper = node.get_wrapper().unwrap();
    let cl = node.with(|n| n.class_struct);
    d.p(format!("wiring\t{}", cl.qualified_name()));
    if let Some(primary) = KtAttrs::get(cl, &KConstructor::PRIMARY_CONSTRUCTOR()) {
        if primary.method_supplier.is_none() {
            let line = match try_catch(|| {
                let map = (primary.default_args_supplier)(wrapper);
                let flags: Vec<String> = primary
                    .parameters
                    .iter()
                    .map(|p| {
                        let shown = map.to_java(p, 1).convert_to_string_and_allow_data_discard();
                        format!("{}={}", p.name, shown != " = ...")
                    })
                    .collect();
                format!("[{}]", flags.join(","))
            }) {
                Ok(line) => line,
                Err(e) => exc(e),
            };
            d.p(format!("  annotation-defaults\t{}", line));
        }
    }
    for mt in cl.get_methods().to_vec() {
        let e = KtAttrs::get(mt, &KElement::KEY());
        let (ps, supplier, defaults): (
            Rc<[KParameter]>,
            Option<MethodSupplier>,
            DefaultArgsSupplier,
        ) = match &e {
            Some(KElement::Function(f)) => (
                f.parameters.clone(),
                Some(f.method_supplier.clone()),
                f.default_args_supplier.clone(),
            ),
            Some(KElement::Constructor(c)) => (
                c.parameters.clone(),
                c.method_supplier.clone(),
                c.default_args_supplier.clone(),
            ),
            Some(KElement::Property(prop)) => {
                if KtAttrs::get(mt, &PropertyMethod::KEY()) == Some(PropertyMethod::Getter) {
                    dump_property(d, wrapper, prop);
                }
                continue;
            }
            _ => continue,
        };
        let key = member(Some(mt.into()));
        let calling =
            match try_catch(|| (supplier.expect("NullPointerException: supplier"))(wrapper)) {
                Ok(calling) => {
                    d.p(format!("  supplier {}\t{}", key, wrapper_name(calling)));
                    calling
                }
                Err(e) => {
                    d.p(format!("  supplier {}\t{}", key, exc(e)));
                    continue;
                }
            };
        if !ps
            .iter()
            .any(|p| Flags::DECLARES_DEFAULT_VALUE.get(p.flags))
        {
            continue;
        }
        dump_defaults(d, &key, wrapper, calling, || defaults(wrapper), &ps, true);
    }
}

fn dump_property(d: &mut Dump, wrapper: ClassWrapper, prop: &KProperty) {
    let g = prop.getter.as_ref().map_or("-".to_string(), |g| {
        wrapper_name((g.method_supplier)(wrapper))
    });
    let s = prop.setter.as_ref().map_or("-".to_string(), |s| {
        wrapper_name((s.method_supplier)(wrapper))
    });
    let i = match &prop.initializer {
        None => "-".to_string(),
        Some(_)
            if prop.underlying_field.is_some_and(|f| {
                f.has_attribute(StructGeneralAttribute::ATTRIBUTE_CONSTANT_VALUE)
            }) && !Flags::IS_DELEGATED.get(prop.flags) =>
        {
            "const".to_string()
        }
        Some(initializer) => match try_catch(|| initializer(wrapper).map(|ex| ex.to_java(1))) {
            Ok(None) => "null".to_string(),
            Ok(Some(buf)) => text(&buf),
            Err(e) => exc(e),
        },
    };
    d.p(format!(
        "  property {}\tget={}\tset={}\tinit={}",
        prop.name, g, s, i
    ));
}

/// The decode half of `KotlinChooser.parseMetadataFor` plus the parsers it
/// runs, used while the chooser itself is `unported!`.
fn parse_metadata_for(cl: StructClass) {
    match try_catch(|| KotlinChooser::parse_metadata_for(cl)) {
        Ok(()) => return,
        Err(e)
            if e.is("UnportedError")
                && e.exception.as_ref().unwrap().message == "KotlinChooser.parseMetadataFor" => {}
        Err(e) => rethrow(e),
    }
    if KtAttrs::has(cl, &KElement::KEY()) {
        return;
    }
    let Some(anno) = KotlinMetadataAnnotation::find(cl) else {
        return;
    };
    let qualified_name = cl.qualified_name();
    let warn = |message: String| {
        DecompilerContext::get_logger().write_message(&message, Severity::Warn);
    };
    let result = try_catch(|| {
        let k_index = anno.index_of("k");
        let d1_index = anno.index_of("d1");
        let d2_index = anno.index_of("d2");
        if k_index == -1 {
            warn(format!("No k attribute (type) in class metadata for class {qualified_name}, cannot continue Kotlin parsing"));
            return;
        }
        let k = KotlinMetadataAnnotation::int_value(anno.get_par_value(k_index));
        if d1_index == -1 {
            let function_ref = cl
                .super_class()
                .is_some_and(|s| &*s.get_string() == "kotlin/jvm/internal/FunctionReferenceImpl");
            if function_ref {
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::FunctionReference(KFunctionReference),
                );
            }
            let resolution = if function_ref {
                "assuming function reference"
            } else {
                "cannot continue Kotlin parsing"
            };
            warn(format!(
                "No d1 attribute (data) in class metadata for class {qualified_name}, {resolution}"
            ));
            return;
        }
        if d2_index == -1 {
            warn(format!(
                "No d2 attribute (strings) in class metadata for class {qualified_name}"
            ));
        }
        let d1 = KotlinMetadataAnnotation::data_from_expr(anno.get_par_value(d1_index));
        let d2 = (d2_index != -1)
            .then(|| KotlinMetadataAnnotation::data_from_expr(anno.get_par_value(d2_index)));
        let xs_index = anno.index_of("xs");
        let xs = (k == 5 && xs_index != -1)
            .then(|| KotlinMetadataAnnotation::string_value(anno.get_par_value(xs_index)));
        let decoded = KtMetadata::decode(k, &d1, d2.as_deref(), xs.as_deref());
        let resolver = decoded.resolver;
        match decoded.message {
            KotlinMetadataMessage::Class(pcl) => {
                KProperty::parse(cl, &pcl.get_property_list(), resolver.as_ref(), None);
                KFunction::parse(
                    cl,
                    resolver.as_ref(),
                    &pcl.get_type_table(),
                    &pcl.get_function_list(),
                    false,
                    None,
                );
                KConstructor::parse(cl, &pcl, resolver.as_ref());
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::Class(Rc::new(KClass::new(pcl, resolver))),
                );
            }
            KotlinMetadataMessage::File(pcl) => {
                KProperty::parse(cl, &pcl.get_property_list(), resolver.as_ref(), None);
                KFunction::parse(
                    cl,
                    resolver.as_ref(),
                    &pcl.get_type_table(),
                    &pcl.get_function_list(),
                    false,
                    None,
                );
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::File(Rc::new(KFile::new(pcl, resolver, None))),
                );
            }
            KotlinMetadataMessage::SyntheticClass(func) => {
                KFunction::parse(
                    cl,
                    resolver.as_ref(),
                    &func.get_type_table(),
                    std::slice::from_ref(&func),
                    true,
                    None,
                );
                if !KtAttrs::has(cl, &KElement::KEY()) {
                    KtAttrs::put(
                        cl,
                        &KElement::KEY(),
                        KElement::Function(KFunction::FAILED_LAMBDA()),
                    );
                }
            }
            KotlinMetadataMessage::MultifileFacade(files) => {
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::MultifileFacade(Rc::new(KMultifileFacade::new(files))),
                );
            }
            KotlinMetadataMessage::MultifilePart { package, xs } => {
                KProperty::parse(cl, &package.get_property_list(), resolver.as_ref(), None);
                KFunction::parse(
                    cl,
                    resolver.as_ref(),
                    &package.get_type_table(),
                    &package.get_function_list(),
                    false,
                    None,
                );
                if xs.is_none() {
                    warn(format!("No xs attribute (extra string; multifile part name) in class metadata for class {qualified_name}"));
                }
                let name = KtMetadata::multifile_name(xs.as_deref(), &qualified_name);
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::File(Rc::new(KFile::new(package, resolver, Some(name)))),
                );
            }
            KotlinMetadataMessage::None => {}
        }
    });
    if let Err(e) = result {
        if e.is("UnportedError") {
            rethrow(e);
        }
        DecompilerContext::get_logger().write_message_ex(
            &format!("Failed to parse metadata for class {qualified_name}"),
            Severity::Warn,
            &e,
        );
    }
}

fn members(guard: &ContextGuard) -> Vec<String> {
    let mut d = Dump::new(guard);
    let classes = sorted_classes(guard);
    for &cl in &classes {
        parse_metadata_for(cl);
    }
    d.p("== parse".to_string());
    d.flush_logs();
    for &cl in &classes {
        dump_attrs(&mut d, cl);
    }

    let processor = guard.class_processor();
    processor.load_classes(None);
    for &cl in &classes {
        let root = processor
            .get_map_root_classes()
            .borrow()
            .get_cloned(&cl.qualified_name());
        let Some(root) = root.filter(|r| r.with(|n| n.type_) == ClassNodeType::Root) else {
            continue;
        };
        d.p(format!("== root {}", cl.qualified_name()));
        processor.process_class(cl);
        d.flush_logs();
        let nodes = tree(root);
        for &node in &nodes {
            enter_class(node);
            let wrapper = node.get_wrapper().unwrap();
            d.p(format!(
                "node\t{}",
                node.with(|n| n.class_struct).qualified_name()
            ));
            if let Err(e) = try_catch(|| invoke_processors(node, true)) {
                d.p(format!("  processors\t{}", exc(e)));
            }
            hidden(&mut d, wrapper);
            d.flush_logs();
        }
        for &node in &nodes {
            dump_attrs(&mut d, node.with(|n| n.class_struct));
        }
        for &node in &nodes {
            enter_class(node);
            dump_wiring(&mut d, node);
        }
        d.flush_logs();
    }
    d.out
}

#[test]
fn kt4_members_match_plugin() {
    let guard = install();
    match try_catch(|| members(&guard)) {
        Ok(out) => assert_lines(&out, &golden("members.golden")),
        Err(e) => match waiting_on(&e) {
            Some(body) => eprintln!("kt4_members_match_plugin: waiting on {body}"),
            None => rethrow(e),
        },
    }
}

/// Opt-in: `KT4_SWEEP=<input>=<dump>,...` compares `KtMembersDump members <input>`
/// dumps of other class directories or jars.
#[test]
fn kt4_members_sweep() {
    let Ok(sweep) = std::env::var("KT4_SWEEP") else {
        return;
    };
    for pair in sweep.split(',') {
        let (input, dump) = pair.split_once('=').unwrap();
        let guard = common::install_context_with(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            (IFernflowerPreferences::LOG_LEVEL, "warn"),
            ("kt-unknown-defaults", "..."),
            ("kt-show-public", "0"),
        ]);
        guard
            .struct_context()
            .add_space_file(Path::new(input), true);
        let expected: Vec<String> = std::fs::read_to_string(dump)
            .unwrap()
            .lines()
            .map(str::to_string)
            .collect();
        match try_catch(|| members(&guard)) {
            Ok(out) => match std::env::var("KT4_SWEEP_OUT") {
                Ok(dir) => {
                    let name = Path::new(dump).file_name().unwrap();
                    std::fs::write(Path::new(&dir).join(name), out.join("\n") + "\n").unwrap();
                }
                Err(_) => assert_lines(&out, &expected),
            },
            Err(e) => match waiting_on(&e) {
                Some(body) => eprintln!("kt4_members_sweep: waiting on {body}"),
                None => rethrow(e),
            },
        }
        drop(guard);
        eprintln!("kt4_members_sweep: {input} ok");
    }
}

fn class_metadata(
    cl: StructClass,
) -> (
    kuna_jvm::kt_proto::Class,
    Option<kuna_jvm::metadata_name_resolver::MetadataNameResolver>,
) {
    let anno = KotlinMetadataAnnotation::find(cl).expect("kotlin/Metadata");
    let d1 = anno.get_d1().expect("d1");
    let d2 = anno.get_d2();
    let decoded = KtMetadata::decode(anno.get_k().expect("k"), &d1, d2.as_deref(), None);
    match decoded.message {
        KotlinMetadataMessage::Class(pcl) => (pcl, decoded.resolver),
        _ => panic!("not a class"),
    }
}

/// `KConstructor.parse`, `writePrimaryConstructor` and `stringify` on classes
/// whose constructors take no parameters (no `KType` needed).
#[test]
fn kt4_constructors_match_plugin() {
    let guard = install();
    let mut d = Dump::new(&guard);
    let processor = guard.class_processor();
    processor.load_classes(None);
    for name in ["kt4/CtorOnly", "kt4/InitOnly", "kt4/Obj", "kt4/E2"] {
        let cl = guard.struct_context().get_class(name).unwrap();
        d.p(format!("== ctors {}", name));
        let (proto, resolver) = class_metadata(cl);
        KConstructor::parse(cl, &proto, resolver.as_ref());
        d.flush_logs();
        dump_attrs(&mut d, cl);
        let root = processor
            .get_map_root_classes()
            .borrow()
            .get_cloned(&name.to_string())
            .unwrap();
        processor.process_class(cl);
        d.flush_logs();
        enter_class(root);
        let wrapper = root.get_wrapper().unwrap();
        d.p(format!(
            "deprecated\t{}",
            text(&KProperty::DEPRECATED_ANNOTATION().0.to_java(0))
        ));
        for mt in cl.get_methods().to_vec() {
            let Some(KElement::Constructor(c)) = KtAttrs::get(mt, &KElement::KEY()) else {
                continue;
            };
            let mw = (c.method_supplier.as_ref().unwrap())(wrapper);
            DecompilerContext::set_current_method_wrapper(mw);
            if c.is_primary {
                let mut header = TextBuffer::new();
                let h = match try_catch(|| c.write_primary_constructor(wrapper, &mut header, 1)) {
                    Ok(h) => h.to_string(),
                    Err(e) => exc(e),
                };
                d.p(format!(
                    "primary {}\t{}\t{}",
                    member(Some(mt.into())),
                    h,
                    text(&header)
                ));
            }
            let mut buf = TextBuffer::new();
            let r = match try_catch(|| c.stringify(wrapper, &mut buf, 1)) {
                Ok(r) => r.to_string(),
                Err(e) => exc(e),
            };
            d.p(format!(
                "stringify {}\t{}\t{}",
                member(Some(mt.into())),
                r,
                text(&buf)
            ));
            d.flush_logs();
        }
    }
    assert_lines(&d.out, &golden("ctors.golden"));
}

/// The record `equals`/`hashCode` the writer's `visitedProperties` set relies on.
#[test]
fn kt4_property_record_identity() {
    let guard = install();
    let cl = guard.struct_context().get_class("kt4/Obj").unwrap();
    let plain = || {
        KProperty::new(
            "p".to_string(),
            Some(KType::NOTHING()),
            518,
            None,
            None,
            None,
            None,
            None,
            Some(vec![KPropertyAnnotation::Deprecated]),
            cl,
        )
    };
    let a = plain();
    let b = plain();
    assert!(a.equals(&b));
    assert_eq!(a.hash_code(), b.hash_code());

    let supplier: kuna_jvm::k_property::ExprentSupplier = Rc::new(|_: ClassWrapper| None);
    let mut c = plain();
    c.initializer = Some(supplier.clone());
    let d = c.clone();
    assert!(c.equals(&d));
    assert_eq!(c.hash_code(), d.hash_code());
    let mut e = plain();
    e.initializer = Some(Rc::new(|_: ClassWrapper| None));
    assert!(!c.equals(&e));
    assert!(!a.equals(&c));

    let mut f = plain();
    f.annotations = Some(Vec::new());
    assert!(!a.equals(&f));

    let mut set: kuna_jvm::rt::JHashSet<KProperty> = kuna_jvm::rt::JHashSet::new();
    assert!(set.add(c.clone()));
    assert!(!set.add(d));
    assert!(set.add(e));
}

fn is_default_bridge(mt: StructMethod) -> bool {
    let name = mt.get_name();
    if name.ends_with("$default") {
        return true;
    }
    name == "<init>"
        && mt
            .get_descriptor()
            .ends_with("ILkotlin/jvm/internal/DefaultConstructorMarker;)V")
}

fn bridges(cl: StructClass, bridge: StructMethod, mt: StructMethod) -> bool {
    let desc = mt.get_descriptor();
    let close = desc.find(')').unwrap();
    let (params, ret) = (&desc[1..close], &desc[close + 1..]);
    for k in 1..=3 {
        let masks = "I".repeat(k);
        let expected = if bridge.get_name() == "<init>" {
            if mt.get_name() != "<init>" {
                return false;
            }
            format!("({params}{masks}Lkotlin/jvm/internal/DefaultConstructorMarker;)V")
        } else {
            if bridge.get_name() != format!("{}$default", mt.get_name()) {
                return false;
            }
            let this = if mt.has_modifier(kuna_jvm::code_constants::ACC_STATIC) {
                String::new()
            } else {
                format!("L{};", cl.qualified_name())
            };
            format!("({this}{params}{masks}Ljava/lang/Object;){ret}")
        };
        if expected == bridge.get_descriptor() {
            return true;
        }
    }
    false
}

/// `KEnumProcessor`, `markObjectInstance` and `DefaultArgsMap.from` over every
/// default-argument bridge, with synthetic parameters: needs no other slice.
#[test]
fn kt4_isolated_processors_and_defaults_match_plugin() {
    let guard = install();
    let mut d = Dump::new(&guard);
    let classes = sorted_classes(&guard);
    for &cl in &classes {
        let Some(anno) = KotlinMetadataAnnotation::find(cl) else {
            continue;
        };
        if anno.get_k() != Some(1) || anno.get_d1().is_none() || anno.get_d2().is_none() {
            continue;
        }
        let (proto, resolver) = class_metadata(cl);
        KtAttrs::put(
            cl,
            &KElement::KEY(),
            KElement::Class(Rc::new(KClass::new(proto, resolver))),
        );
    }
    let processor = guard.class_processor();
    processor.load_classes(None);
    for &cl in &classes {
        let root = processor
            .get_map_root_classes()
            .borrow()
            .get_cloned(&cl.qualified_name());
        let Some(root) = root.filter(|r| r.with(|n| n.type_) == ClassNodeType::Root) else {
            continue;
        };
        d.p(format!("== root {}", cl.qualified_name()));
        processor.process_class(cl);
        d.flush_logs();
        for node in tree(root) {
            enter_class(node);
            let wrapper = node.get_wrapper().unwrap();
            let ncl = node.with(|n| n.class_struct);
            d.p(format!("node\t{}", ncl.qualified_name()));
            invoke_processors(node, false);
            hidden(&mut d, wrapper);
            for fd in ncl.get_fields().to_vec() {
                if let Some(s) = describe(KtAttrs::get(fd, &KElement::KEY())) {
                    d.p(format!("  field {}\t{}", member(Some(fd.into())), s));
                }
            }
            let methods = ncl.get_methods().to_vec();
            for &dm in &methods {
                if !is_default_bridge(dm) {
                    continue;
                }
                let caller = methods
                    .iter()
                    .copied()
                    .find(|&mt| mt != dm && bridges(ncl, dm, mt));
                let Some(caller) = caller else {
                    d.p(format!("  bridge {}\t-", member(Some(dm.into()))));
                    continue;
                };
                let count = caller.method_descriptor().params.len();
                let ps: Vec<KParameter> = (0..count)
                    .map(|i| KParameter::new(2, format!("p{i}"), KType::NOTHING(), None, 0))
                    .collect();
                let calling =
                    wrapper.get_method_wrapper(&caller.get_name(), &caller.get_descriptor());
                let key = format!(
                    "{}\t{}",
                    member(Some(dm.into())),
                    member(Some(caller.into()))
                );
                let defaults = wrapper.get_method_wrapper(&dm.get_name(), &dm.get_descriptor());
                dump_defaults(
                    &mut d,
                    &key,
                    wrapper,
                    calling,
                    || DefaultArgsMap::from(defaults, calling, &ps),
                    &ps,
                    false,
                );
            }
            d.flush_logs();
        }
    }
    assert_lines(&d.out, &golden("isolated.golden"));
}
