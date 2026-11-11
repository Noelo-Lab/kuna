//! kuna fix `kt_enum_when_mappings`: in Kotlin output, a `when` over an enum
//! is written over the enum value with the enum constants as its branches.
//!
//! kotlinc compiles `when (e) { A -> .. }` to a `tableswitch` over
//! `X$WhenMappings.$EnumSwitchMapping$N[e.ordinal()]`, a synthetic class whose
//! static initializer stores, for each constant the `when`s of a file name,
//! a label in a static final synthetic `int[]` (`map[A.ordinal()] = 1`); a
//! nullable subject is tested first (`e == null ? -1 : map[..]`, label -1 for
//! `null`). Vineflower's Java pipeline turns such switches back into
//! `switch (e) { case A: .. }` (`SwitchHelper.simplifySwitches`), but the
//! Kotlin plugin's pass list never calls it, so the `when` is written over
//! the array with numeric labels. Here, after the main loop (where the Java
//! pipeline simplifies switches), the mapping is read from the initializer's
//! bytecode, where the order in which classes are processed does not matter:
//! `getstatic F` or the `aload n` of a local later stored to `F` (`aload n;
//! putstatic F`), then `getstatic E.C`, `invokevirtual E.ordinal()`, the
//! label and `iastore`. Each label becomes its constant, `-1` of a nullable
//! subject `null`, and the subject becomes `e`. A label in the case that
//! holds the default (a `tableswitch` filler, or a mapped constant whose
//! branch kotlinc left to `else`) is dropped with its edge: the value reaches
//! the default either way. A switch with any other label not in the mapping
//! is left as it is.

use crate::array_exprent::ArrayExprent;
use crate::code_constants::{
    ACC_SYNTHETIC, CLINIT_NAME, OPC_ALOAD, OPC_ASTORE, OPC_BIPUSH, OPC_GETSTATIC, OPC_IASTORE,
    OPC_INVOKEVIRTUAL, OPC_PUTSTATIC, OPC_SIPUSH,
};
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::rt::{try_catch, JList};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::switch_helper::SwitchHelper;
use crate::switch_statement::SwitchStatement;
use crate::type_family::TypeFamily;
use crate::var_type::VarType;

/// A store `array[E.C.ordinal()] = label`: the label, `E`, `C`.
type Store = (i32, String, String);

/// The constants one mapping array labels: the enum class and, per label,
/// the constant's name.
struct Mapping {
    enum_class: String,
    labels: Vec<(i32, String)>,
}

impl Mapping {
    fn constant(&self, label: i32) -> Option<&str> {
        self.labels
            .iter()
            .find(|(l, _)| *l == label)
            .map(|(_, name)| name.as_str())
    }
}

/// Resugars every switch over a `$WhenMappings` array in `stat`; whether any
/// changed.
pub fn resugar(stat: Statement) -> bool {
    let mut ret = false;
    if let Some(switch) = stat.try_as_switch() {
        ret |= resugar_switch(switch);
    }
    let mut i = 0;
    while i < stat.get_stats().borrow().len() {
        let st = stat.get_stats().borrow().get(i);
        ret |= resugar(st);
        i += 1;
    }
    ret
}

fn resugar_switch(switch: SwitchStatement) -> bool {
    let Some(head) = switch.get_headexprent().try_as_switch_head() else {
        return false;
    };
    let value = head.get_value();
    let Some((subject, array, nullable)) = selector(value) else {
        return false;
    };
    let field = array.get_array().as_field();
    let Some(mapping) = mapping(&field.get_classname(), &field.get_name()) else {
        return false;
    };
    let ordinal = array.get_index().as_invocation();
    if ordinal.get_classname() != mapping.enum_class {
        return false;
    }

    let default_edge = switch.get_default_edge();
    let case_values = switch.get_case_values();
    let case_edges = switch.get_case_edges();
    let mut new_values: Vec<Vec<Option<Exprent>>> = Vec::new();
    let mut new_edges: Vec<Vec<StatEdge>> = Vec::new();
    let mut dropped: Vec<StatEdge> = Vec::new();
    for (values, edges) in case_values.iter().zip(case_edges.iter()) {
        let (values, edges) = (values.to_vec(), edges.to_vec());
        if values.len() != edges.len() {
            return false;
        }
        let holds_default = edges.iter().any(|e| Some(*e) == default_edge);
        let mut kept_values = Vec::new();
        let mut kept_edges = Vec::new();
        for (v, e) in values.into_iter().zip(edges) {
            let Some(v) = v else {
                kept_values.push(None);
                kept_edges.push(e);
                continue;
            };
            let Some(label) = v
                .try_as_const()
                .filter(|c| c.get_const_type().type_family == TypeFamily::Integer)
                .map(|c| c.get_int_value())
            else {
                return false;
            };
            if holds_default {
                dropped.push(e);
                continue;
            }
            let replacement = if nullable && label == -1 {
                ConstExprent::new(VarType::VARTYPE_NULL(), None, None).0
            } else if let Some(name) = mapping.constant(label) {
                FieldExprent::new(
                    name.to_string(),
                    mapping.enum_class.clone(),
                    true,
                    None,
                    FieldDescriptor::parse_descriptor(&format!("L{};", mapping.enum_class)),
                    None,
                )
                .0
            } else {
                return false;
            };
            kept_values.push(Some(replacement));
            kept_edges.push(e);
        }
        new_values.push(kept_values);
        new_edges.push(kept_edges);
    }

    if dropped
        .iter()
        .any(|e| Some(*e) == default_edge || new_edges.iter().flatten().any(|k| k == e))
    {
        return false;
    }
    case_values.set_all(new_values.into_iter().map(JList::from).collect());
    for (list, edges) in case_edges.iter().zip(new_edges) {
        list.set_all(edges);
    }
    for edge in dropped {
        edge.remove();
    }
    let new_expr = subject.copy();
    head.replace_exprent(value, new_expr);
    new_expr.add_bytecode_offsets(value.get_bytecode().as_ref());
    true
}

/// The subject, the mapping array read and whether the subject is
/// nullable, when `value` is `map[e.ordinal()]` or `e == null ? -1 :
/// map[e.ordinal()]` (`e != null ? map[..] : -1`).
fn selector(value: Exprent) -> Option<(Exprent, ArrayExprent, bool)> {
    if let Some(array) = mapping_read(value) {
        let subject = array.get_index().as_invocation().get_instance()?;
        return Some((subject, array, false));
    }
    let func = value.try_as_function()?;
    if func.get_func_type() != FunctionType::Ternary {
        return None;
    }
    let ops = func.get_lst_operands().to_vec();
    if ops.len() != 3 {
        return None;
    }
    let test = ops[0].try_as_function()?;
    let (array, minus_one) = match test.get_func_type() {
        FunctionType::Eq => (ops[2], ops[1]),
        FunctionType::Ne => (ops[1], ops[2]),
        _ => return None,
    };
    let array = mapping_read(array)?;
    let subject = array.get_index().as_invocation().get_instance()?;
    let tested = test.get_lst_operands().to_vec();
    if tested.len() != 2
        || !tested[1].try_as_const().is_some_and(|c| c.is_null())
        || !tested[0].equals(subject)
        || !minus_one.try_as_const().is_some_and(|c| {
            c.get_const_type().type_family == TypeFamily::Integer && c.get_int_value() == -1
        })
    {
        return None;
    }
    Some((subject, array, true))
}

/// `value` as `C.F[e.ordinal()]` of a static field `F`.
fn mapping_read(value: Exprent) -> Option<ArrayExprent> {
    let array = value.try_as_array()?;
    let field = array.get_array().try_as_field()?;
    let index = array.get_index().try_as_invocation()?;
    (field.is_static()
        && field.get_instance().is_none()
        && field.get_descriptor().descriptor_string == "[I"
        && index.get_name() == "ordinal"
        && index.get_string_descriptor() == "()I"
        && !index.is_static()
        && index.get_instance().is_some())
    .then_some(array)
}

/// The constants the static initializer of the `$WhenMappings` class
/// `classname` stores in its array `field`.
fn mapping(classname: &str, field: &str) -> Option<Mapping> {
    let cl = DecompilerContext::get_struct_context().get_class(classname)?;
    if cl.get_access_flags() & ACC_SYNTHETIC == 0 {
        return None;
    }
    let st_field = cl.get_field(field, "[I")?;
    if (st_field.get_access_flags() & SwitchHelper::STATIC_FINAL_SYNTHETIC)
        != SwitchHelper::STATIC_FINAL_SYNTHETIC
    {
        return None;
    }
    let clinit = cl.get_method(CLINIT_NAME, "()V")?;
    let expanded = clinit.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| clinit.expand_data(cl)).is_err() {
        return None;
    }
    let mapping = read_initializer(cl, field);
    if !expanded {
        clinit.release_resources();
    }
    mapping
}

fn read_initializer(cl: StructClass, field: &str) -> Option<Mapping> {
    let seq = cl.get_method(CLINIT_NAME, "()V")?.get_instruction_sequence()?;
    let pool = cl.get_pool();
    let name = cl.qualified_name();
    let own_field = |index: i32| {
        pool.get_link_constant(index).is_some_and(|link| {
            link.classname.as_deref() == Some(name.as_str())
                && link.elementname == field
                && link.descriptor == "[I"
        })
    };
    let n = seq.length();
    let mut stores: Vec<Store> = Vec::new();
    let mut pending: Vec<(i32, Vec<Store>)> = Vec::new();
    let mut i = 0;
    while i < n {
        let instr = seq.get_instr(i);
        let op = instr.opcode();
        if op == OPC_ASTORE {
            pending.retain(|(slot, _)| *slot != instr.operand(0));
        } else if op == OPC_ALOAD
            && i + 1 < n
            && seq.get_instr(i + 1).opcode() == OPC_PUTSTATIC
            && own_field(seq.get_instr(i + 1).operand(0))
        {
            let slot = instr.operand(0);
            if let Some(pos) = pending.iter().position(|(s, _)| *s == slot) {
                stores.extend(pending.remove(pos).1);
            }
        }
        if i + 4 < n && (op == OPC_GETSTATIC || op == OPC_ALOAD) {
            let constant = seq.get_instr(i + 1);
            let call = seq.get_instr(i + 2);
            let push = seq.get_instr(i + 3);
            let store = seq.get_instr(i + 4);
            if constant.opcode() == OPC_GETSTATIC
                && call.opcode() == OPC_INVOKEVIRTUAL
                && (push.opcode() == OPC_BIPUSH || push.opcode() == OPC_SIPUSH)
                && store.opcode() == OPC_IASTORE
            {
                let constant_link = pool.get_link_constant(constant.operand(0))?;
                let call_link = pool.get_link_constant(call.operand(0))?;
                let enum_class = constant_link.classname.clone()?;
                if call_link.elementname != "ordinal"
                    || call_link.descriptor != "()I"
                    || constant_link.descriptor != format!("L{enum_class};")
                {
                    return None;
                }
                let entry = (push.operand(0), enum_class, constant_link.elementname.clone());
                if op == OPC_GETSTATIC {
                    if own_field(instr.operand(0)) {
                        stores.push(entry);
                    }
                } else {
                    let slot = instr.operand(0);
                    match pending.iter_mut().find(|(s, _)| *s == slot) {
                        Some((_, list)) => list.push(entry),
                        None => pending.push((slot, vec![entry])),
                    }
                }
                i += 5;
                continue;
            }
        }
        i += 1;
    }

    let enum_class = stores.first()?.1.clone();
    let mut labels: Vec<(i32, String)> = Vec::new();
    for (label, class, constant) in stores {
        if class != enum_class || labels.iter().any(|(l, c)| *l == label || *c == constant) {
            return None;
        }
        labels.push((label, constant));
    }
    Some(Mapping { enum_class, labels })
}
