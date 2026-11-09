//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/SwitchExprent.java`.

use crate::basic_block_statement::BasicBlockStatement;
use crate::exprent::{Exprent, ExprentKind};
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::stat_edge::StatEdge;
use crate::switch_statement::SwitchStatement;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

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

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("SwitchExprent.toJava")
    }

    #[allow(dead_code)]
    fn is_empty_default(_block: BasicBlockStatement, _head: SwitchStatement) -> bool {
        unported!("SwitchExprent.isEmptyDefault")
    }

    #[allow(dead_code)]
    fn is_exhaustive(self) -> bool {
        unported!("SwitchExprent.isExhaustive")
    }

    #[allow(dead_code)]
    fn is_synthetic_throw_edge(_edge: StatEdge) -> bool {
        unported!("SwitchExprent.isSyntheticThrowEdge")
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
