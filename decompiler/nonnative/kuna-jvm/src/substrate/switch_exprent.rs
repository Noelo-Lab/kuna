//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/SwitchExprent.java`.
//!
//! kuna fix `switch_yield_const_retype` (`kuna_fixes`): in `to_java`,
//! a constant yielded by a single-expression arm takes the switch's type only
//! when a constant can have that type, where Vineflower retypes it to any
//! common supertype of the arms and fails to write it.
//! kuna fixes (`kuna_fixes`): `type_switch_enum_label` qualifies an enum
//! constant label of a pattern switch on another type; `type_switch_label_lookup`
//! writes a case with several type patterns as one label of unnamed patterns;
//! `switch_synthetic_default` hides javac's default of any exhaustive switch
//! (`kuna_fix_switch_synthetic_default`), not only of an enum's whose class
//! is loaded.

use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::ACC_ENUM;
use crate::const_exprent::ConstExprent;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentKind};
use crate::field_exprent::FieldExprent;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::{kuna_fix_type_switch_enum_label, kuna_fix_type_switch_label_lookup};
use crate::function_exprent::FunctionType;
use crate::pattern::Pattern;
use crate::rt::{BitSet, JHashSet, JavaEq, JavaHash};
use crate::stat_edge::StatEdge;
use crate::substrate::kuna_fix_switch_synthetic_default;
use crate::switch_statement::SwitchStatement;
use crate::text_buffer::TextBuffer;
use crate::var_type::{get_class, VarType};
use crate::{java_subclass, jthrow};

pub struct SwitchExprentData {
    pub backing: SwitchStatement,
    pub type_: VarType,
    pub fallthrough: bool,
    pub standalone: bool,
}

java_subclass!(pub SwitchExprent: Exprent, ExprentKind::Switch(SwitchExprentData),
               is_switch, as_switch, try_as_switch);

impl JavaEq for SwitchExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for SwitchExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl SwitchExprent {
    pub fn new(
        backing: SwitchStatement,
        type_: VarType,
        fallthrough: bool,
        standalone: bool,
    ) -> SwitchExprent {
        let id = Exprent::next_id();
        SwitchExprent(Exprent::alloc_with_id(
            id,
            ExprentKind::Switch(SwitchExprentData {
                backing,
                type_,
                fallthrough,
                standalone,
            }),
        ))
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let (backing, type_, standalone) = self.d(|d| (d.backing, d.type_.clone(), d.standalone));
        if !backing.is_phantom() {
            jthrow!(IllegalStateException; "Switch expression backing statement isn't phantom!");
        }

        let mut buf = TextBuffer::new();

        let switch_type = backing.get_headexprent().get_expr_type();
        let is_exhaustive = self.is_exhaustive()
            || (kuna_fixes::enabled(KunaFix::SwitchSyntheticDefault)
                && kuna_fix_switch_synthetic_default::is_javac_default(backing));

        buf.append_buffer(&backing.get_headexprent().to_java(indent))
            .append(" {")
            .append_line_separator();
        for i in 0..backing.get_case_statements().len() {
            let stat = backing.get_case_statements().get(i);
            let edges = backing.get_case_edges().get(i);
            let values = backing.get_case_values().get(i);
            let guard = if backing.get_case_guards().len() > i {
                backing.get_case_guards().get(i)
            } else {
                None
            };

            let mut has_default = false;

            for edge in edges.iter() {
                if Some(edge) == backing.get_default_edge() {
                    if is_exhaustive && SwitchExprent::is_synthetic_throw_edge(edge) {
                        break;
                    }
                    has_default = true;
                    break;
                }
            }

            let unnamed = if kuna_fixes::enabled(KunaFix::TypeSwitchLabelLookup) {
                kuna_fix_type_switch_label_lookup::unnamed_pattern_types(&values)
            } else {
                None
            };
            let mut has_edge = false;
            for j in 0..edges.len() {
                if let Some(types) = &unnamed {
                    if j == 0 {
                        buf.append_indent(indent + 1).append("case ");
                        kuna_fix_type_switch_label_lookup::append_unnamed(&mut buf, types);
                        has_edge = true;
                    }
                    continue;
                }
                let value = if Some(edges.get(j)) == backing.get_default_edge() && values.len() <= j
                {
                    Some(ConstExprent::new(VarType::VARTYPE_NULL(), None, None).0)
                } else {
                    values.get(j)
                };
                let Some(mut value_ex) = value else {
                    continue;
                };

                if has_default && !value_ex.get_expr_type().ptr_eq(&VarType::VARTYPE_NULL()) {
                    continue;
                }

                if !has_edge {
                    buf.append_indent(indent + 1).append("case ");
                } else {
                    buf.append(", ");
                }

                if value_ex.is_const()
                    && !standalone
                    && !value_ex.get_expr_type().equals(&VarType::VARTYPE_NULL())
                {
                    value_ex = value_ex.copy();
                    value_ex.as_const().set_const_type(switch_type.clone());
                }

                let qualified = value_ex.try_as_field().filter(|f| {
                    f.is_static()
                        && kuna_fixes::enabled(KunaFix::TypeSwitchEnumLabel)
                        && kuna_fix_type_switch_enum_label::is_qualified_label(
                            *f,
                            &switch_type,
                            backing.0,
                        )
                });
                if let Some(field) = qualified {
                    buf.append_buffer(&kuna_fix_type_switch_enum_label::qualified_label(
                        field, indent,
                    ));
                } else if let Some(field) = value_ex.try_as_field().filter(|f| f.is_static()) {
                    let name = field.get_name();
                    buf.append_field_descriptor(
                        &name,
                        false,
                        &field.get_classname(),
                        &name,
                        &field.get_descriptor(),
                    );
                } else if value_ex
                    .try_as_function()
                    .is_some_and(|f| f.get_func_type() == FunctionType::Instanceof)
                {
                    let pattern = Pattern::cast(value_ex.get_all_exprents()[2]);
                    for var in pattern.get_pattern_vars() {
                        var.set_writing_pattern();
                    }

                    buf.append_buffer(&value_ex.get_all_exprents()[2].to_java(indent));
                } else {
                    buf.append_buffer(&value_ex.to_java(indent));
                }

                has_edge = true;
            }

            if let Some(guard) = guard {
                buf.append(" when ").append_buffer(&guard.to_java_default());
            }

            if has_default {
                if !has_edge {
                    if let Some(block) = stat.try_as_basic_block() {
                        if SwitchExprent::is_empty_default(block, backing) {
                            continue;
                        }
                    }

                    buf.append_indent(indent + 1).append("default");
                } else {
                    buf.append(", default");
                }
                has_edge = true;
            }

            if !has_edge {
                continue;
            }

            buf.append(" -> ");

            let mut simple = stat.is_basic_block();

            if stat.get_exprents().is_some_and(|e| e.len() != 1) {
                simple = false;
            }

            if simple {
                let exprent = stat.get_exprents().expect("NullPointerException").get(0);

                if let Some(yield_) = exprent.try_as_yield() {
                    let content = yield_.get_content();

                    if content.is_const()
                        && !content.get_expr_type().equals(&VarType::VARTYPE_NULL())
                    {
                        if kuna_fixes::enabled(KunaFix::SwitchYieldConstRetype) {
                            if content.as_const().can_be_written_as(&type_) {
                                content.as_const().set_const_type(type_.clone());
                            }
                        } else {
                            content.as_const().set_const_type(type_.clone());
                        }
                    }

                    buf.append_buffer(content.to_java(indent + 1).append(";"));
                } else if let Some(exit) = exprent.try_as_exit() {
                    if exit.get_exit_type() == ExitType::Throw {
                        buf.append_buffer(exit.0.to_java(indent + 1).append(";"));
                    } else {
                        jthrow!(IllegalStateException; "Can't have return in switch expression");
                    }
                } else {
                    buf.append_buffer(exprent.to_java(indent + 1).append(";"));
                }
            } else {
                buf.append("{");
                buf.append_line_separator();
                let stat_buf = stat.to_java(indent + 2);
                buf.append_buffer(&stat_buf);
                buf.append_indent(indent + 1).append("}");
            }

            buf.append_line_separator();
        }

        buf.append_indent(indent).append("}");

        buf
    }

    fn is_empty_default(block: BasicBlockStatement, _head: SwitchStatement) -> bool {
        if !block
            .get_exprents()
            .expect("NullPointerException")
            .is_empty()
        {
            return false;
        }

        let edges = block.get_successor_edge_view(StatEdge::TYPE_BREAK);

        edges.len() == 1
    }

    fn is_exhaustive(self) -> bool {
        let backing = self.get_backing();
        let mut enum_values_seen: JHashSet<FieldExprent> = JHashSet::new();
        for case_value in backing.get_case_values().iter() {
            for exprent in case_value.iter().flatten() {
                if let Some(field) = exprent.try_as_field().filter(|f| f.is_static()) {
                    enum_values_seen.add(field);
                }
            }
        }
        let enum_target_class = get_class(&backing.get_headexprent().get_expr_type().value);
        let Some(enum_target_class) = enum_target_class else {
            return false;
        };
        if !enum_target_class.has_modifier(ACC_ENUM) {
            return false;
        }
        let seen = enum_values_seen.to_vec();
        enum_target_class
            .get_fields()
            .to_vec()
            .into_iter()
            .filter(|x| x.has_modifier(ACC_ENUM))
            .map(|x| x.get_name())
            .all(|field_name| seen.iter().any(|found| found.get_name() == field_name))
    }

    fn is_synthetic_throw_edge(edge: StatEdge) -> bool {
        let target = edge.get_destination();
        let target_exprs = target.get_exprents();
        if let Some(target_exprs) = target_exprs.filter(|e| e.len() == 1) {
            let target_expr = target_exprs.get(0);
            return match target_expr.try_as_exit() {
                Some(exit) if exit.get_exit_type() == ExitType::Throw => {
                    let value = exit
                        .get_value()
                        .expect("NullPointerException")
                        .get_expr_type();
                    let name = value.value.as_deref().expect("NullPointerException");
                    name == "java/lang/IncompatibleClassChangeError"
                        || name == "java/lang/MatchException"
                }
                _ => false,
            };
        }
        false
    }

    pub fn set_type(self, type_: VarType) {
        self.d_mut(|d| d.type_ = type_)
    }

    pub fn get_precedence(self) -> i32 {
        1
    }

    pub fn get_expr_type(self) -> VarType {
        self.d(|d| d.type_.clone())
    }

    pub fn copy(self) -> Exprent {
        let (backing, type_, fallthrough, standalone) =
            self.d(|d| (d.backing, d.type_.clone(), d.fallthrough, d.standalone));
        SwitchExprent::new(backing, type_, fallthrough, standalone).0
    }

    pub fn get_backing(self) -> SwitchStatement {
        self.d(|d| d.backing)
    }

    pub fn get_all_exprents_list(self, _list: &mut Vec<Exprent>) {}

    pub fn get_bytecode_range(self, values: Option<&mut BitSet>) {
        self.measure_bytecode(values);
    }
}
