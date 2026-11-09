//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/MonitorExprent.java`.

use crate::exprent::{Exprent, ExprentKind};
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::{java_subclass, unported};

/// Java `MonitorExprent.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    Enter,
    Exit,
}

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::Enter => "ENTER",
            Type::Exit => "EXIT",
        }
    }
}

impl JavaHash for Type {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for Type {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct MonitorExprentData {
    pub remove: bool,
    pub mon_type: Type,
    pub value: Exprent,
}

java_subclass!(pub MonitorExprent: Exprent, ExprentKind::Monitor(MonitorExprentData),
               is_monitor, as_monitor, try_as_monitor);

impl JavaEq for MonitorExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for MonitorExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl MonitorExprent {
    pub fn new(
        mon_type: Type,
        value: Exprent,
        bytecode_offsets: Option<&BitSet>,
    ) -> MonitorExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Monitor(MonitorExprentData {
                remove: false,
                mon_type,
                value,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        MonitorExprent(e)
    }

    pub fn copy(self) -> Exprent {
        let (mon_type, value) = self.d(|d| (d.mon_type, d.value));
        let bytecode = self.get_bytecode();
        MonitorExprent::new(mon_type, value.copy(), bytecode.as_ref()).0
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| lst.push(d.value));
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("MonitorExprent.toJava")
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.value {
                d.value = new_expr;
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(me) = o.try_as_monitor() else {
            return false;
        };
        let (mon_type, value) = self.d(|d| (d.mon_type, d.value));
        mon_type == me.get_mon_type() && value.equals(me.get_value())
    }

    pub fn get_mon_type(self) -> Type {
        self.d(|d| d.mon_type)
    }

    pub fn get_value(self) -> Exprent {
        self.d(|d| d.value)
    }

    pub fn is_removable(self) -> bool {
        self.d(|d| d.remove)
    }

    pub fn set_remove(self, remove: bool) {
        self.d_mut(|d| d.remove = remove)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let value = self.get_value();
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(value));
        self.measure_bytecode(values);
    }
}
