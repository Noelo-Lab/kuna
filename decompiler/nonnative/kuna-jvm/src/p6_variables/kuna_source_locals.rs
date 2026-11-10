//! The kuna option `jvm_source_locals` (`kuna_options::SOURCE_LOCALS`): keep
//! the locals the class file records for the source.
//!
//! javac writes one LocalVariableTable entry per local of the source and none
//! for the temporaries it adds itself (a `finally`'s or `synchronized` block's
//! saved return value, a for-each's iterator, array, length and index), and it
//! gives each local the next free slot when its declaration runs. Vineflower
//! folds a variable assigned and then read once into the read
//! (`SimplifyExprentsHelper.isAssignmentReturn`, `isArrayInitializer`,
//! `MergeHelper.matchForEach`) and declares each variable at its first use
//! (`VarDefinitionHelper`), which loses a named local and reorders slots. With
//! the option on, [`is_source_local`] keeps a variable the table names, and
//! [`order_declarations`] moves (or splits off) declarations so that javac
//! numbers the slots as the class file does.

use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_options::{self, SOURCE_LOCALS};
use crate::method_descriptor::MethodDescriptor;
use crate::rt::JList;
use crate::statement::{Statement, StatementType};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;

/// Whether `jvm_source_locals` is on for the engine on this thread.
pub fn enabled() -> bool {
    kuna_options::form_enabled(SOURCE_LOCALS)
}

/// Whether `var`, a variable the method's code stores, is a local of the
/// source at the first instruction of `read` (or right after the store when
/// `read` has no offset): `Some` of whether the LocalVariableTable names its
/// slot there, `None` when the method has no table. A `$` marks a name a
/// compiler made up (javac before 8 listed its for-each temporaries `i$`,
/// `arr$` and `len$`), not a local of the source.
pub fn named_by_table(var: VarExprent, read: Option<VarExprent>) -> Option<bool> {
    let processor = var.get_processor()?;
    let table = processor.get_method().get_local_variable_attr()?;
    let slot = processor
        .get_var_original_index(var.get_index())
        .unwrap_or(var.get_index());
    let pc = read.and_then(first_offset).or_else(|| end_offset(var))?;
    Some(
        table
            .matching_vars(slot, pc)
            .iter()
            .any(|v| !v.get_name().contains('$')),
    )
}

/// Whether `store`, a variable an assignment writes, is a local of the source
/// that `read` reads next: named by the LocalVariableTable, or, in a class
/// without a table that kotlinc did not compile, read by the instruction right
/// after the store, which javac emits only for a local of the source (it
/// stores a value it returns only around a `finally` or a monitor exit, which
/// sit in between; kotlinc also stores array literals it builds).
pub fn is_source_local(store: VarExprent, read: VarExprent) -> bool {
    if store.is_stack() || !enabled() {
        return false;
    }
    named_by_table(store, Some(read)).unwrap_or_else(|| {
        !compiled_by_kotlinc()
            && matches!((end_offset(store), first_offset(read)), (Some(end), Some(start)) if end == start)
    })
}

/// Whether `var`, a variable a desugared loop would hide (the iterator or the
/// index of a loop Vineflower writes as a for-each), is named by the
/// LocalVariableTable: javac's own for-each variables are not; without a
/// table the two cannot be told apart.
pub fn is_named_loop_local(var: VarExprent) -> bool {
    enabled() && !var.is_stack() && named_by_table(var, None).unwrap_or(false)
}

fn first_offset(var: VarExprent) -> Option<i32> {
    let bytecode = var.0.get_bytecode()?;
    let start = bytecode.next_set_bit(0);
    (start >= 0).then_some(start)
}

fn end_offset(var: VarExprent) -> Option<i32> {
    let bytecode = var.0.get_bytecode()?;
    (!bytecode.is_empty()).then(|| bytecode.length() as i32)
}

/// One declaration printed at the top level of a block: `list` holds
/// `exprent`, a definition with (`bare` false) or without an initializer;
/// `slot` is its variable's original slot when the class file gives it to
/// this variable alone: a slot of the method's locals (not one of the indices
/// Vineflower gives the variables it makes, such as a `finally`'s semaphore),
/// not a parameter's, and not that of a variable still in scope (Vineflower
/// splits a reassigned parameter or local into versions of one slot, and
/// declares a reused slot's variables in one scope).
#[derive(Clone)]
struct Declaration {
    list: JList<Exprent>,
    exprent: Exprent,
    bare: bool,
    slot: Option<i32>,
    class_def: bool,
}

/// The method's blocks, each the declarations at its top level in printed
/// order, and the original slots of the variables in scope.
struct Gathered {
    varproc: VarProcessor,
    first_local: i32,
    max_locals: i32,
    blocks: Vec<Vec<Declaration>>,
    in_scope: Vec<i32>,
}

/// In every block of the method, moves each declaration without an
/// initializer before the first declaration printed earlier in that block
/// whose variable has a higher original slot (never past a local class), and
/// splits a declaration with one there into `T v;` and `v = e;`, so that
/// javac, which gives a local the next free slot where it is declared,
/// numbers them as the class file does. Classes kotlinc compiled are left
/// alone (it numbers its own temporaries among the locals).
pub fn order_declarations(root: Statement, varproc: VarProcessor) {
    if !enabled() || compiled_by_kotlinc() {
        return;
    }
    let mt = varproc.get_method();
    let descriptor = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
    let first_local = i32::from(!mt.has_modifier(ACC_STATIC))
        + descriptor.params.iter().map(|p| p.stack_size).sum::<i32>();
    let mut gathered = Gathered {
        varproc,
        first_local,
        max_locals: mt.get_local_variables(),
        blocks: Vec::new(),
        in_scope: Vec::new(),
    };
    let mut top = Vec::new();
    gather(root, &mut top, &mut gathered);
    gathered.blocks.push(top);
    for block in gathered.blocks {
        order_block(block);
    }
}

/// Whether the class being decompiled comes from kotlinc: it carries
/// `kotlin.Metadata` or names a `.kt` source file.
fn compiled_by_kotlinc() -> bool {
    DecompilerContext::get_current_class().is_some_and(|cl| {
        KotlinMetadataAnnotation::find(cl).is_some()
            || cl
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_SOURCE_FILE)
                .is_some_and(|a| a.get_source_file(cl.get_pool()).ends_with(".kt"))
    })
}

fn gather(stat: Statement, current: &mut Vec<Declaration>, gathered: &mut Gathered) {
    declarations(stat.get_var_definitions(), current, gathered);
    if let Some(exprents) = stat.get_exprents() {
        declarations(exprents, current, gathered);
        return;
    }
    let stats = stat.get_stats().borrow().to_vec();
    if stat.type_() == StatementType::Sequence && !stat.is_labeled() {
        for child in stats {
            gather(child, current, gathered);
        }
        return;
    }
    let head = match stat.type_() {
        StatementType::Root
        | StatementType::If
        | StatementType::Switch
        | StatementType::Synchronized => stat.get_first(),
        _ => None,
    };
    if let Some(head) = head {
        gather(head, current, gathered);
    }
    let scope = gathered.in_scope.len();
    if stat.type_() == StatementType::Do {
        for exprent in stat.get_stat_exprents() {
            for e in exprent
                .get_all_exprents_rec(true)
                .into_iter()
                .chain([exprent])
            {
                if let Some(var) = e.try_as_var().filter(|v| v.is_definition()) {
                    enter(var, gathered);
                }
            }
        }
    }
    let loop_scope = gathered.in_scope.len();
    if stat.type_() == StatementType::Sequence {
        let mut inner = Vec::new();
        for child in stats {
            gather(child, &mut inner, gathered);
        }
        gathered.blocks.push(inner);
    } else {
        for child in stats.into_iter().filter(|c| Some(*c) != head) {
            let mut inner = Vec::new();
            gather(child, &mut inner, gathered);
            gathered.blocks.push(inner);
            gathered.in_scope.truncate(loop_scope);
        }
    }
    gathered.in_scope.truncate(scope);
}

/// Brings `var`, a definition, into scope: its original slot when that is a
/// slot of its own (not a parameter's, not one a variable in scope holds).
fn enter(var: VarExprent, gathered: &mut Gathered) -> Option<i32> {
    if var.is_class_def() {
        return None;
    }
    let slot = gathered.varproc.get_var_original_index(var.get_index())?;
    let own = (gathered.first_local..gathered.max_locals).contains(&slot)
        && !gathered.in_scope.contains(&slot);
    gathered.in_scope.push(slot);
    own.then_some(slot)
}

fn declarations(list: JList<Exprent>, out: &mut Vec<Declaration>, gathered: &mut Gathered) {
    for exprent in list.iter() {
        let top = if let Some(var) = exprent.try_as_var() {
            Some((var, true))
        } else {
            exprent
                .try_as_assignment()
                .and_then(|a| a.get_left().try_as_var())
                .map(|var| (var, false))
        };
        let Some((var, bare)) = top.filter(|(var, _)| var.is_definition()) else {
            continue;
        };
        let slot = enter(var, gathered);
        out.push(Declaration {
            list: list.clone(),
            exprent,
            bare,
            slot,
            class_def: var.is_class_def(),
        });
    }
}

fn order_block(mut block: Vec<Declaration>) {
    let mut i = 0;
    while i < block.len() {
        let moved = block[i].clone();
        let Some(slot) = moved
            .slot
            .filter(|_| moved.bare || splittable(moved.exprent))
        else {
            i += 1;
            continue;
        };
        let from = block[..i]
            .iter()
            .rposition(|d| d.class_def)
            .map_or(0, |p| p + 1);
        if let Some(j) = (from..i).find(|&j| block[j].slot.is_some_and(|s| s > slot)) {
            let anchor = block[j].clone();
            let declaration = if moved.bare {
                remove(&moved.list, moved.exprent);
                moved.exprent
            } else {
                let var = moved.exprent.as_assignment().get_left().as_var();
                var.set_definition(false);
                let bare = var.copy();
                bare.as_var().set_definition(true);
                bare
            };
            let at = position(&anchor.list, anchor.exprent).expect("declaration in its list");
            anchor.list.add_at(at, declaration);
            block.remove(i);
            block.insert(
                j,
                Declaration {
                    list: anchor.list,
                    exprent: declaration,
                    bare: true,
                    ..moved
                },
            );
        }
        i += 1;
    }
}

/// Whether the definition `exprent` (`T v = e;`) can be written `T v;` and
/// `v = e;`: its type has a name (it is not printed `var`).
fn splittable(exprent: Exprent) -> bool {
    exprent
        .try_as_assignment()
        .and_then(|a| a.get_left().try_as_var())
        .is_some_and(|v| !v.is_intersection_type() && v.get_definition_type() != "var")
}

/// Sorts a run of declarations without an initializer as
/// `Exprent.sortIndexed` prints it: by original slot (then index) while
/// declarations are ordered for javac, else as Vineflower does, by index.
pub fn sort_declarations(defs: &mut [VarExprent]) {
    if !enabled() || compiled_by_kotlinc() {
        defs.sort_by(|o1, o2| o1.get_index().wrapping_sub(o2.get_index()).cmp(&0));
        return;
    }
    defs.sort_by_key(|var| {
        let index = var.get_index();
        let slot = var
            .get_processor()
            .and_then(|p| p.get_var_original_index(index))
            .unwrap_or(index);
        (slot, index)
    });
}

fn position(list: &JList<Exprent>, exprent: Exprent) -> Option<usize> {
    list.to_vec().iter().position(|e| *e == exprent)
}

fn remove(list: &JList<Exprent>, exprent: Exprent) {
    if let Some(at) = position(list, exprent) {
        list.remove_at(at);
    }
}
