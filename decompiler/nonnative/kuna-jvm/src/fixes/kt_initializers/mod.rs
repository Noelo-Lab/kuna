//! Fix family `kt_initializers` (Kotlin output): where a property's or class's
//! initializer comes from: a companion's field, a type's default value, an
//! object's static initializer written `init { .. }` (in its companion's body
//! when it has one), the Kotlin 1.3 instance local read as `INSTANCE`, and a
//! file's static initializer (`kt_file_static_initializer`).

pub(crate) mod kt_file_static_initializer;

use std::cell::RefCell;
use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::code_constants::{
    ACC_STATIC, ACC_SYNTHETIC, OPC_INVOKESPECIAL, OPC_PUTFIELD, OPC_PUTSTATIC,
};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::k_element::KElement;
use crate::k_property::ExprentSupplier;
use crate::k_type::KType;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::ClassKind;
use crate::root_statement::RootStatement;
use crate::rt::{try_catch, JList};
use crate::statement::Statement;
use crate::statements::Statements;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

thread_local! {
    static MOVED: RefCell<Vec<(String, TextBuffer)>> = const { RefCell::new(Vec::new()) };
}

/// Whether `cl` is an `object` or a companion object, whose static
/// initializer is its own `init` block.
pub fn is_object(cl: StructClass) -> bool {
    let Some(KElement::Class(cls)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return false;
    };
    matches!(
        Flags::CLASS_KIND.get(cls.flags()),
        Some(ClassKind::Object | ClassKind::CompanionObject)
    )
}

/// Whether the static initializer of `cl` is written as an `init` block:
/// `cl` is an object, or `companion` (its companion object's node) takes it.
pub fn writes_init_block(cl: StructClass, companion: Option<ClassNode>) -> bool {
    is_object(cl) || companion.is_some()
}

/// Whether the member class `companion` of the class `outer` is written (as
/// the class body writes its member classes), so a block moved to it is not lost.
pub fn is_written(companion: ClassNode, outer: ClassWrapper) -> bool {
    let (cl, access) = companion.with(|d| (d.class_struct, d.access));
    let synthetic = (access & ACC_SYNTHETIC) != 0 || cl.is_synthetic();
    !(synthetic && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC))
        && !outer.get_hidden_members().borrow().contains(&cl.qualified_name())
}

/// While alive, the `init` blocks moved during one class's write; dropping
/// it forgets those its companion did not take (not written, or the write
/// threw), so none reaches a later class of the same name on the thread.
pub struct MovedScope(usize);

pub fn scope() -> MovedScope {
    MovedScope(MOVED.with(|m| m.borrow().len()))
}

impl Drop for MovedScope {
    fn drop(&mut self) {
        MOVED.with(|m| m.borrow_mut().truncate(self.0));
    }
}

/// Keeps the `init` block `text` of an outer class for its companion `companion`.
pub fn move_to(companion: ClassNode, text: TextBuffer) {
    let name = companion.with(|d| d.class_struct).qualified_name();
    MOVED.with(|m| m.borrow_mut().push((name, text)));
}

/// The `init` block moved to the companion object `cl`, if any.
pub fn take(cl: StructClass) -> Option<TextBuffer> {
    let name = cl.qualified_name();
    MOVED.with(|m| {
        let mut moved = m.borrow_mut();
        let pos = moved.iter().position(|(n, _)| *n == name)?;
        Some(moved.remove(pos).1)
    })
}

/// Rewrites the instance local of the static initializer `root` of `cl`.
pub fn replace_instance_local(root: RootStatement, cl: StructClass) -> bool {
    if !is_object(cl) {
        return false;
    }
    let Some(first) = Statements::find_first_data(root.0) else {
        return false;
    };
    let Some(exprents) = first.get_exprents() else {
        return false;
    };
    if exprents.len() < 2 {
        return false;
    }
    let (Some(create), Some(store)) =
        (exprents.get(0).try_as_assignment(), exprents.get(1).try_as_assignment())
    else {
        return false;
    };
    let (Some(local), Some(new)) = (create.get_left().try_as_var(), create.get_right().try_as_new())
    else {
        return false;
    };
    let own_type = format!("L{};", cl.qualified_name());
    let instance = store.get_left().try_as_field().filter(|f| {
        f.is_static()
            && f.get_classname() == cl.qualified_name()
            && f.get_descriptor().descriptor_string == own_type
    });
    let stored = store.get_right().try_as_var().is_some_and(|v| {
        v.get_index() == local.get_index() && v.get_version() == local.get_version()
    });
    let Some(instance) = instance else {
        return false;
    };
    if !stored
        || new.get_new_type().value.as_deref() != Some(cl.qualified_name().as_str())
        || writes(root.0, local.get_index(), create.0)
    {
        return false;
    }
    store.set_right(create.get_right());
    exprents.remove_at(0);
    replace_reads(root.0, local.get_index(), instance.0);
    true
}

fn lists(stat: Statement) -> Vec<JList<Exprent>> {
    let mut lists: Vec<JList<Exprent>> = stat.get_exprents().into_iter().collect();
    if let Some(ifstat) = stat.try_as_if() {
        lists.push(ifstat.get_headexprent_list());
    } else if let Some(switch) = stat.try_as_switch() {
        lists.push(switch.get_headexprent_list());
    } else if let Some(sync) = stat.try_as_synchronized() {
        lists.push(sync.get_headexprent_list());
    } else if let Some(dostat) = stat.try_as_do() {
        lists.push(dostat.get_init_exprent_list());
        lists.push(dostat.get_condition_exprent_list());
        lists.push(dostat.get_inc_exprent_list());
    }
    lists
}

/// Whether local `index` is written anywhere under `stat` but by `except`.
fn writes(stat: Statement, index: i32, except: Exprent) -> bool {
    let mut stats = vec![stat];
    while let Some(st) = stats.pop() {
        for list in lists(st) {
            for e in list.iter() {
                for sub in e.get_all_exprents_rec_self(true, true) {
                    if sub == except {
                        continue;
                    }
                    let target = match sub.get_type() {
                        ExprentType::Assignment => Some(sub.as_assignment().get_left()),
                        ExprentType::Function => {
                            let f = sub.as_function();
                            f.get_func_type().is_ppmm().then(|| f.get_lst_operands().get(0))
                        }
                        _ => None,
                    };
                    if target.and_then(|t| t.try_as_var()).is_some_and(|v| v.get_index() == index)
                    {
                        return true;
                    }
                }
            }
        }
        stats.extend(st.get_stats().borrow().to_vec());
    }
    false
}

fn replace_reads(stat: Statement, index: i32, field: Exprent) {
    let mut stats = vec![stat];
    while let Some(st) = stats.pop() {
        for list in lists(st) {
            for i in 0..list.len() {
                let e = list.get(i);
                if e.try_as_var().is_some_and(|v| v.get_index() == index) {
                    list.set(i, field.copy());
                } else {
                    replace_in(e, index, field);
                }
            }
        }
        stats.extend(st.get_stats().borrow().to_vec());
    }
}

fn replace_in(expr: Exprent, index: i32, field: Exprent) {
    for sub in expr.get_all_exprents() {
        if sub.try_as_var().is_some_and(|v| v.get_index() == index) {
            expr.replace_exprent(sub, field.copy());
        } else {
            replace_in(sub, index, field);
        }
    }
}

/// The initializer of the static field `key` of the class `owner`.
pub fn static_initializer(owner: String, key: String) -> ExprentSupplier {
    Rc::new(move |_writing: ClassWrapper| {
        let node = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get(&owner)
            .copied()?;
        node.get_wrapper()?
            .get_static_field_initializers()
            .borrow()
            .get_with_key(&key)
    })
}

/// The default value of `type_` written as a Kotlin literal, for the types
/// that have one.
pub fn default_literal(type_: &KType) -> Option<&'static str> {
    if type_.k().is_nullable {
        return Some("null");
    }
    Some(match type_.k().kotlin_type.as_str() {
        "kotlin/Int" | "kotlin/Short" | "kotlin/Byte" => "0",
        "kotlin/Long" => "0L",
        "kotlin/Float" => "0.0F",
        "kotlin/Double" => "0.0",
        "kotlin/Boolean" => "false",
        "kotlin/Char" => "'\\u0000'",
        _ => return None,
    })
}

/// Whether no constructor of `class_struct` (or, for a static field, of the
/// classes that may hold it) stores `field`.
pub fn never_stored(class_struct: StructClass, field: StructField) -> bool {
    let is_static = field.has_modifier(ACC_STATIC);
    let mut owners = vec![class_struct];
    if is_static {
        let name = class_struct.qualified_name();
        if let Some(outer) = name
            .rfind('$')
            .and_then(|i| DecompilerContext::get_struct_context().get_class(&name[..i]))
        {
            owners.push(outer);
        }
    }
    let constructor = if is_static { "<clinit>" } else { "<init>" };
    owners.into_iter().all(|owner| {
        owner
            .get_methods()
            .as_slice()
            .iter()
            .filter(|mt| mt.get_name() == constructor)
            .all(|mt| !stores(owner, *mt, field, is_static))
    })
}

/// Whether `mt`'s code stores `field` (conservatively true when it cannot be read).
fn stores(cl: StructClass, mt: StructMethod, field: StructField, is_static: bool) -> bool {
    if !mt.contains_code() {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return true;
    }
    let opcode = if is_static {
        OPC_PUTSTATIC
    } else {
        OPC_PUTFIELD
    };
    let found = mt.get_instruction_sequence().is_none_or(|seq| {
        let delegating = !is_static
            && (0..seq.length())
                .map(|i| seq.get_instr(i))
                .find(|instr| {
                    instr.opcode() == OPC_INVOKESPECIAL
                        && cl
                            .get_pool()
                            .get_link_constant(instr.operand(0))
                            .is_some_and(|link| link.elementname == "<init>")
                })
                .and_then(|instr| cl.get_pool().get_link_constant(instr.operand(0)))
                .is_some_and(|link| {
                    link.classname.as_deref() == Some(cl.qualified_name().as_str())
                });
        !delegating
            && (0..seq.length()).any(|i| {
                let instr = seq.get_instr(i);
                instr.opcode() == opcode
                    && cl
                        .get_pool()
                        .get_link_constant(instr.operand(0))
                        .is_none_or(|link| {
                            link.elementname == field.get_name()
                                && link.descriptor == field.get_descriptor()
                        })
            })
    });
    if !expanded {
        mt.release_resources();
    }
    found
}
