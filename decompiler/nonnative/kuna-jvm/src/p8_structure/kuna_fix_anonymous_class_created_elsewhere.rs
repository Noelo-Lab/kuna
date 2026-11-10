//! kuna fix `anonymous_class_created_elsewhere`: an anonymous class that only
//! another class creates is nested in that class, so its captured variables
//! are mapped from the method that creates it.
//!
//! `ClassesProcessor` nests an anonymous class in the class whose
//! `InnerClasses` attribute lists it and whose method its `EnclosingMethod`
//! names, and `NestedClassProcessor` maps the values its constructor stores
//! (the captured variables) from the `new`s it finds in that class's
//! methods. kotlinc copies the body of an inline function's lambda into the
//! `$$inlined$` class it regenerates for each call site, `new` of the lambda's
//! objects included, but leaves those objects declared where the lambda was
//! written: the declaring method never creates them. Finding no `new`,
//! `NestedClassProcessor` takes the first captured value for the enclosing
//! instance, so the object's body reads `Outer.this` where the bytecode reads
//! a captured `Ref` or collector, and its other captured values stay fields
//! of the object. Vineflower prints that body, wrong, as Java and as a Kotlin
//! lambda, and fails a Kotlin object without Class metadata.
//!
//! With the fix, an anonymous class whose declaring class does not create it
//! and which exactly one other class creates (outside its own nested classes)
//! is nested in that class instead, before any class is processed; its
//! captured variables are then found at the `new`, as for any anonymous
//! class. The `$$inlined$` class holds the lambda's captured variables in
//! fields, so the `new` passes some of them as `this.f$inlined`, which
//! `NestedClassProcessor` does not take for a capture: the object would keep
//! its own copy in a field. For such an object (one constructor), a value
//! every `new` passes as the same field `f` of the creating instance is a
//! capture of that instance, and the object's reads of the field holding it
//! become `Outer.this.f`, when that reads the same value: `f` is a final field
//! of the creating class, every constructor of that class stores it once
//! before calling anything, nothing else stores it, and no `new` is in a
//! constructor. Like any anonymous class created at more than one `new`,
//! the captured variables take their names from the first creating method;
//! kotlinc's second site is the `$$forInline` copy of the first.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::classes_processor::{sort_nodes, ClassNode, ClassNodeType};
use crate::code_constants::{
    ACC_FINAL, ACC_STATIC, CLINIT_NAME, INIT_NAME, OPC_INVOKEDYNAMIC, OPC_INVOKEINTERFACE,
    OPC_INVOKESPECIAL, OPC_INVOKESTATIC, OPC_INVOKEVIRTUAL, OPC_NEW, OPC_PUTFIELD,
};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::interpreter_util::InterpreterUtil;
use crate::rt::{rethrow, try_catch, JHashMap, Shared};
use crate::struct_class::StructClass;
use crate::struct_context::is_io_exception;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

/// A field of the creating class: owner, name, descriptor.
type FieldRef = (String, String, String);

/// Per anonymous class and constructor descriptor, the field capture of each argument.
type Slots = BTreeMap<(String, String), Vec<Option<FieldRef>>>;

thread_local! {
    static SLOTS: RefCell<Slots> = const { RefCell::new(BTreeMap::new()) };
    static FIELDS: RefCell<BTreeMap<(String, String), FieldRef>> =
        const { RefCell::new(BTreeMap::new()) };
}

/// Nests every anonymous class of `classes` (all class nodes, by name) that
/// one class other than its declaring one creates in that class.
pub fn nest_at_creating_class(classes: &Shared<JHashMap<String, ClassNode>>) {
    SLOTS.with(|s| s.borrow_mut().clear());
    FIELDS.with(|f| f.borrow_mut().clear());
    let nodes: Vec<(String, ClassNode)> = classes
        .borrow()
        .iter()
        .map(|(name, node)| (name.clone(), *node))
        .collect();
    let anonymous: BTreeMap<String, ClassNode> = nodes
        .iter()
        .filter(|(_, node)| {
            node.with(|d| d.type_ == ClassNodeType::Anonymous && d.parent.is_some())
        })
        .map(|(name, node)| (name.clone(), *node))
        .collect();
    if anonymous.is_empty() {
        return;
    }

    let mut callers: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (name, node) in &nodes {
        let pool = node.with(|d| d.class_struct).get_pool();
        for owner in pool.constructor_owners() {
            if owner != *name && anonymous.contains_key(&owner) {
                callers.entry(owner).or_default().insert(name.clone());
            }
        }
    }

    for (name, callers) in callers {
        let node = anonymous[&name];
        let Some(parent) = node.with(|d| d.parent) else {
            continue;
        };
        let outside: Vec<ClassNode> = callers
            .iter()
            .filter_map(|caller| classes.borrow().get(caller).copied())
            .filter(|caller| *caller != parent && !is_within(*caller, node))
            .collect();
        if outside.is_empty() || creates(parent, &name) != Some(false) {
            continue;
        }
        let mut creators = Vec::new();
        for caller in outside {
            match creates(caller, &name) {
                Some(true) => creators.push(caller),
                Some(false) => {}
                None => creators.push(parent),
            }
        }
        let [creator] = creators[..] else {
            continue;
        };
        if creator == parent || is_within(creator, node) {
            continue;
        }
        parent.with(|d| d.nested.clone()).remove_item(&node);
        let nested = creator.with(|d| d.nested.clone());
        nested.add(node);
        sort_nodes(&nested);
        node.with_mut(|d| d.parent = Some(creator));
    }
}

/// Whether this fix nested `node` at its creator: an anonymous class whose
/// parent does not list it among its inner classes.
fn is_nested_at_creator(node: ClassNode) -> bool {
    let (type_, parent, listed_by) =
        node.with(|d| (d.type_, d.parent, d.enclosing_classes.clone()));
    type_ == ClassNodeType::Anonymous
        && parent.is_some_and(|p| {
            !listed_by
                .borrow()
                .contains(&p.with(|d| d.class_struct).qualified_name())
        })
}

/// Finds, for every class this fix nested in `node`, the constructor
/// arguments that every `new` in `node` passes as the same stable field of
/// the creating instance (`NestedClassProcessor.computeLocalVarsAndDefinitions`
/// asks [`field_capture`] for them next).
pub fn find_field_captures(node: ClassNode) {
    let (cl, nested) = node.with(|d| (d.class_struct, d.nested.to_vec()));
    let owner = cl.qualified_name();
    let moved: Vec<String> = nested
        .iter()
        .filter(|n| is_nested_at_creator(**n))
        .map(|n| n.with(|d| d.class_struct))
        .filter(|c| constructors(*c) == 1)
        .map(|c| c.qualified_name())
        .collect();
    if moved.is_empty() {
        return;
    }
    let Some(wrapper) = node.get_wrapper() else {
        return;
    };
    let mut sites: BTreeMap<(String, String), Option<Vec<Option<FieldRef>>>> = BTreeMap::new();
    for method in wrapper.get_methods().borrow().to_vec() {
        if method.with(|d| d.root.is_none()) {
            continue;
        }
        let mt = method.with(|d| d.method_struct);
        let plain = mt.get_name() != INIT_NAME
            && mt.get_name() != CLINIT_NAME
            && (mt.get_access_flags() & ACC_STATIC) == 0;
        let Some(graph) = method.get_or_build_graph() else {
            continue;
        };
        graph.iterate_exprents(|exprent: Exprent| {
            let mut all = exprent.get_all_exprents_rec(true);
            all.push(exprent);
            for expr in all {
                let Some(constructor) = expr.try_as_new().and_then(|n| n.get_constructor()) else {
                    continue;
                };
                let class = constructor.get_classname();
                if !moved.contains(&class) {
                    continue;
                }
                let fields: Vec<Option<FieldRef>> = constructor
                    .get_lst_parameters()
                    .to_vec()
                    .into_iter()
                    .map(|arg| this_field(arg, &owner).filter(|_| plain))
                    .collect();
                let key = (class, constructor.get_string_descriptor());
                let merged = match sites.remove(&key) {
                    None => Some(fields),
                    Some(None) => None,
                    Some(Some(seen)) if seen.len() == fields.len() => Some(
                        seen.into_iter()
                            .zip(fields)
                            .map(|(a, b)| a.filter(|a| Some(a) == b.as_ref()))
                            .collect(),
                    ),
                    Some(Some(_)) => None,
                };
                sites.insert(key, merged);
            }
            0
        });
    }
    for (key, fields) in sites {
        let Some(fields) = fields else {
            continue;
        };
        let fields: Vec<Option<FieldRef>> = fields
            .into_iter()
            .map(|f| f.filter(|f| is_stable(cl, f)))
            .collect();
        if fields.iter().any(Option::is_some) {
            SLOTS.with(|s| s.borrow_mut().insert(key, fields));
        }
    }
}

fn constructors(cl: StructClass) -> usize {
    cl.get_methods()
        .as_slice()
        .iter()
        .filter(|m| m.get_name() == INIT_NAME)
        .count()
}

/// `arg` as a read of a field of `owner` on the method's `this`.
fn this_field(arg: Exprent, owner: &str) -> Option<FieldRef> {
    let field = arg.try_as_field()?;
    let instance = field.get_instance()?.try_as_var()?;
    (!field.is_static() && field.get_classname() == owner && instance.get_index() == 0).then(|| {
        (
            owner.to_string(),
            field.get_name(),
            field.get_descriptor().descriptor_string.clone(),
        )
    })
}

/// Whether the field `f` of `cl` holds one value from the end of the
/// constructor's first store: it is final, every constructor stores it once
/// before any invocation, and no other method stores it.
fn is_stable(cl: StructClass, f: &FieldRef) -> bool {
    let Some(fd) = cl.get_field(&f.1, &f.2) else {
        return false;
    };
    if !fd.has_modifier(ACC_FINAL) || fd.has_modifier(ACC_STATIC) {
        return false;
    }
    let pool = cl.get_pool();
    let methods = cl.get_methods().to_vec();
    let scanned = try_catch(|| {
        let mut stable = true;
        for mt in methods {
            mt.expand_data(cl);
            let init = mt.get_name() == INIT_NAME;
            let mut stores = 0;
            let mut invoked = false;
            if let Some(seq) = mt.get_instruction_sequence() {
                for instr in seq.iter() {
                    match instr.opcode() {
                        OPC_PUTFIELD => {
                            let link = pool.get_link_constant(instr.operand(0));
                            if link.is_some_and(|l| {
                                l.classname.as_deref() == Some(f.0.as_str())
                                    && l.elementname == f.1
                                    && l.descriptor == f.2
                            }) {
                                stores += 1;
                                stable &= init && !invoked;
                            }
                        }
                        OPC_INVOKEVIRTUAL | OPC_INVOKESPECIAL | OPC_INVOKESTATIC
                        | OPC_INVOKEINTERFACE | OPC_INVOKEDYNAMIC => invoked = true,
                        _ => {}
                    }
                }
            }
            mt.release_resources();
            stable &= !init || stores == 1;
        }
        stable
    });
    match scanned {
        Ok(stable) => stable,
        Err(caught) if is_io_exception(&caught) => false,
        Err(caught) => rethrow(caught),
    }
}

/// For argument `slot` (`arg`) of a `new class(..)` with constructor
/// `descriptor`, stored by the constructor in the field `field_key`: the
/// creating method's `this` when the argument is a field capture
/// ([`find_field_captures`]), whose reads of `field_key` then read that field.
pub fn field_capture(
    class: &str,
    descriptor: &str,
    slot: usize,
    arg: Exprent,
    field_key: &str,
) -> Option<VarVersionPair> {
    let field = SLOTS.with(|s| {
        s.borrow()
            .get(&(class.to_string(), descriptor.to_string()))
            .and_then(|fields| fields.get(slot).cloned().flatten())
    })?;
    if field_key.is_empty() || this_field(arg, &field.0).as_ref() != Some(&field) {
        return None;
    }
    let instance = arg.as_field().get_instance()?.as_var();
    FIELDS.with(|f| {
        f.borrow_mut()
            .insert((class.to_string(), field_key.to_string()), field)
    });
    Some(VarVersionPair::from_var(instance))
}

/// `replacement` (the variable that stands for the captured creating
/// instance) as a read of the field the object's field `name` of `class`
/// captures, or `replacement` itself when that field is no field capture.
pub fn read_captured_field(
    class: &str,
    name: &str,
    descriptor: &str,
    replacement: Exprent,
) -> Exprent {
    let key = (
        class.to_string(),
        InterpreterUtil::make_unique_key(name, descriptor),
    );
    match FIELDS.with(|f| f.borrow().get(&key).cloned()) {
        None => replacement,
        Some((owner, name, descriptor)) => {
            FieldExprent::new(
                name,
                owner,
                false,
                Some(replacement),
                FieldDescriptor::parse_descriptor(&descriptor),
                replacement.get_bytecode().as_ref(),
            )
            .0
        }
    }
}

/// Whether `arg`, passed at the `new` of `node` being written where the
/// object reads the captured creating instance `pair`, is the field capture
/// the object reads.
pub fn field_capture_reads_at_site(node: ClassNode, pair: VarVersionPair, arg: Exprent) -> bool {
    let class = node.with(|d| d.class_struct).qualified_name();
    let Some(parent) = node.with(|d| d.parent) else {
        return false;
    };
    let owner = parent.with(|d| d.class_struct).qualified_name();
    let (Some(field), Some(instance)) = (
        this_field(arg, &owner),
        arg.try_as_field()
            .and_then(|f| f.get_instance())
            .and_then(|i| i.try_as_var()),
    ) else {
        return false;
    };
    pair.var == 0
        && DecompilerContext::get_current_class_node() == Some(parent)
        && VarExprent::get_name_base(instance) == "this"
        && FIELDS.with(|f| {
            f.borrow()
                .iter()
                .any(|((c, _), captured)| *c == class && *captured == field)
        })
}

/// Whether `node` is `ancestor` or nested in it.
fn is_within(node: ClassNode, ancestor: ClassNode) -> bool {
    let mut current = Some(node);
    while let Some(n) = current {
        if n == ancestor {
            return true;
        }
        current = n.with(|d| d.parent);
    }
    false
}

/// Whether a method of `node`'s class has a `new` of `name`; `None` when a
/// method's bytecode cannot be read.
fn creates(node: ClassNode, name: &str) -> Option<bool> {
    let cl = node.with(|d| d.class_struct);
    let pool = cl.get_pool();
    let methods = cl.get_methods().to_vec();
    let scanned = try_catch(|| {
        let mut found = false;
        for mt in methods {
            mt.expand_data(cl);
            if let Some(seq) = mt.get_instruction_sequence() {
                found |= seq.iter().any(|instr| {
                    instr.opcode() == OPC_NEW
                        && pool
                            .get_primitive_constant(instr.operand(0))
                            .is_some_and(|c| *c.get_string() == *name)
                });
            }
            mt.release_resources();
            if found {
                break;
            }
        }
        found
    });
    match scanned {
        Ok(found) => Some(found),
        Err(caught) if is_io_exception(&caught) => None,
        Err(caught) => rethrow(caught),
    }
}
