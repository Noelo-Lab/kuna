//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/PatternExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::dec_helper::DecHelper;
use crate::exprent::{Exprent, ExprentKind};
use crate::pattern::Pattern;
use crate::rt::{BitSet, JList, JavaEq, JavaHash};
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

/// Java `PatternExprent.PatternData.RecordPatternData`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RecordPatternData {
    pub cl: StructClass,
}

/// Java sealed interface `PatternExprent.PatternData`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternData {
    RecordPatternData(RecordPatternData),
}

pub struct PatternExprentData {
    pub data: PatternData,
    pub var_type: VarType,
    /// Each element is a `Pattern` (a `VarExprent` or a nested `PatternExprent`).
    pub exprents: JList<Exprent>,
    pub bound_types: JList<Option<VarType>>,
}

java_subclass!(pub PatternExprent: Exprent, ExprentKind::Pattern(PatternExprentData),
               is_pattern, as_pattern, try_as_pattern);

impl JavaEq for PatternExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for PatternExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl PatternExprent {
    pub fn new(
        data: PatternData,
        type_: VarType,
        exprents: impl Into<JList<Exprent>>,
    ) -> PatternExprent {
        let id = Exprent::next_id();
        let exprents = exprents.into();
        let bound_types: JList<Option<VarType>> = JList::new();

        for exprent in exprents.iter() {
            if !Pattern::instance_of(exprent) {
                ValidationHelper::assert_true(false, "Illegal input for PatternExprent");
            }
            bound_types.add(None);
        }

        PatternExprent(Exprent::alloc_with_id(
            id,
            ExprentKind::Pattern(PatternExprentData {
                data,
                var_type: type_,
                exprents,
                bound_types,
            }),
        ))
    }

    pub fn get_all_exprents_list(self, list: &mut Vec<Exprent>) {
        self.d(|d| list.extend(d.exprents.iter()));
    }

    pub fn get_expr_type(self) -> VarType {
        self.d(|d| d.var_type.clone())
    }

    pub fn copy(self) -> Exprent {
        let (data, var_type, exprents) =
            self.d(|d| (d.data, d.var_type.clone(), d.exprents.to_vec()));
        PatternExprent::new(data, var_type, DecHelper::copy_exprent_list(&exprents)).0
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("PatternExprent.toJava")
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("PatternExprent.checkExprTypeBounds")
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let exprents = self.d(|d| d.exprents.to_vec());
        Exprent::measure_bytecode_list(values.as_deref_mut(), &exprents);
        self.measure_bytecode(values);
    }

    pub fn get_pattern_vars(self) -> Vec<VarExprent> {
        let mut vars = Vec::new();
        for e in self.get_exprents().iter() {
            match Pattern::try_cast(e) {
                Some(p) => vars.extend(p.get_pattern_vars()),
                None => ValidationHelper::assert_true(false, "Illegal input in PatternExprent"),
            }
        }
        vars
    }

    pub fn get_data(self) -> PatternData {
        self.d(|d| d.data)
    }

    /// The sub-pattern list itself (aliased, like the Java reference).
    pub fn get_exprents(self) -> JList<Exprent> {
        self.d(|d| d.exprents.clone())
    }

    /// The bound-type list itself (aliased; callers `set` entries).
    pub fn get_bound_types(self) -> JList<Option<VarType>> {
        self.d(|d| d.bound_types.clone())
    }

    pub fn record_data(cl: StructClass) -> PatternData {
        PatternData::RecordPatternData(RecordPatternData { cl })
    }
}
