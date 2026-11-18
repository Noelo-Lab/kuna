//! Fix family `patterns`: `instanceof` patterns and pattern switches. In this
//! file, `pattern_binding_shape` makes a type pattern only from javac's shape
//! (the subject read once from a local, the binding right after the
//! `instanceof; ifeq` that skips it); `pattern_variable_scope` makes it only
//! when every other use of its variable lies in the pattern's scope (JLS
//! 6.3.1).

pub(crate) mod type_switch_enum_label;
pub(crate) mod type_switch_label_lookup;
pub(crate) mod unchecked_type_pattern;

use crate::code_constants::{OPC_IFEQ, OPC_INSTANCEOF};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionExprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_statement::IfStatement;
use crate::rt::BitSet;
use crate::statement::{Statement, StatementType};
use crate::struct_method::StructMethod;
use crate::var_exprent::VarExprent;

/// Whether `iof`, an `instanceof` whose operand is `source`, and `binding`,
/// the store of its cast value that would become the pattern's variable,
/// are javac's compiled type pattern in method `mt`. `true` when the
/// offsets are unknown.
pub fn is_javac_pattern(
    mt: StructMethod,
    iof: FunctionExprent,
    source: Exprent,
    binding: Exprent,
) -> bool {
    if !read_once(source) {
        return false;
    }
    let Some(test) = iof.0.get_bytecode().filter(|b| !b.is_empty()) else {
        return true;
    };
    let mut range = BitSet::new();
    binding.get_bytecode_range(Some(&mut range));
    let start = range.next_set_bit(0);
    if start < 0 {
        return true;
    }
    let instanceof = test.next_set_bit(0);
    let jump = instanceof + 3;
    opcode_at(mt, instanceof) != Some(OPC_INSTANCEOF)
        || (opcode_at(mt, jump) == Some(OPC_IFEQ) && start == jump + 3)
}

/// Whether evaluating `source` reads it once: a variable, or the synthetic
/// field holding a variable a local or anonymous class captured.
fn read_once(source: Exprent) -> bool {
    if source.is_var() {
        return true;
    }
    let Some(field) = source.try_as_field() else {
        return false;
    };
    let descriptor = field.get_descriptor().descriptor_string.clone();
    DecompilerContext::get_struct_context()
        .get_class(&field.get_classname())
        .and_then(|cl| cl.get_field(&field.get_name(), &descriptor))
        .is_some_and(|fd| {
            fd.is_synthetic()
                || (DecompilerContext::get_option(IFernflowerPreferences::SYNTHETIC_NOT_SET)
                    && fd.get_name().contains('$'))
        })
}

/// The opcode of the instruction of `mt` at bytecode offset `offset`.
fn opcode_at(mt: StructMethod, offset: i32) -> Option<i32> {
    mt.get_instruction_sequence()?
        .instructions()
        .into_iter()
        .find(|i| i.start_offset() == offset)
        .map(|i| i.opcode())
}

/// Whether `index` is read or written by `exp` or one of its sub-exprents,
/// other than by `except`.
fn uses(exp: Exprent, index: i32, except: Exprent) -> bool {
    let mut all = exp.get_all_exprents_rec(true);
    all.push(exp);
    all.into_iter()
        .any(|e| e != except && e.try_as_var().is_some_and(|v| v.get_index() == index))
}

/// Whether a statement of `stat`'s subtree outside `scope` (and outside the
/// head condition of `if_stat`) uses `index`.
fn used_outside(stat: Statement, scope: &[Statement], if_stat: Statement, index: i32, except: Exprent) -> bool {
    if scope.contains(&stat) {
        return false;
    }
    if stat.get_var_definitions().to_vec().into_iter().any(|e| uses(e, index, except)) {
        return true;
    }
    match stat.get_exprents() {
        Some(exprents) => exprents.to_vec().into_iter().any(|e| uses(e, index, except)),
        None => {
            if stat != if_stat && stat.get_stat_exprents().into_iter().any(|e| uses(e, index, except)) {
                return true;
            }
            let stats = stat.get_stats().borrow().to_vec();
            stats.into_iter().any(|st| used_outside(st, scope, if_stat, index, except))
        }
    }
}

/// Whether the pattern variable `var`, defined by moving its assignment from
/// `branch` into `statement`'s condition, would be used only in its scope:
/// `branch` when it is the `if`'s own branch; the statements after the `if`
/// in its block when `branch` is the `if`'s successor.
pub fn in_scope(statement: IfStatement, branch: Statement, var: VarExprent, root: Statement) -> bool {
    let if_stat = statement.0;
    let scope: Vec<Statement> = if statement.get_ifstat() == Some(branch) || statement.get_elsestat() == Some(branch) {
        vec![branch]
    } else {
        match if_stat.get_parent() {
            Some(parent) if parent.type_() == StatementType::Sequence => {
                let siblings = parent.get_stats().borrow().to_vec();
                let at = siblings.iter().position(|&s| s == if_stat).expect("the if in its sequence");
                siblings[at + 1..].to_vec()
            }
            _ => Vec::new(),
        }
    };
    !used_outside(root, &scope, if_stat, var.get_index(), var.0)
}
