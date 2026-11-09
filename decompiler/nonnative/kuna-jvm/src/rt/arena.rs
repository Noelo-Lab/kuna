//! Thread-local object arenas: the storage behind every ported Java class that
//! has identity or shared mutable state.
//!
//! A handle is a `Copy` index, so `==` on handles is Java's reference `==`. Each
//! object sits in its own cell, so a `with`/`with_mut` closure may freely touch
//! other objects and allocate; only re-entering the *same* object mutably is an
//! error. Class-scope arenas are emptied at the end of a decompilation run;
//! session-scope arenas live as long as the decompilation session. Objects
//! known to be garbage earlier can be released one by one: their slot (and
//! so their index, which is their identity hash) stays taken, only the
//! contents are dropped, and any later use of the handle panics.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scope {
    Class,
    Session,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct RawHandle {
    index: u32,
    epoch: u32,
}

impl RawHandle {
    pub fn index(self) -> u32 {
        self.index
    }
}

pub struct Arena<T> {
    slots: RefCell<Vec<Option<Rc<RefCell<T>>>>>,
    epoch: Cell<u32>,
    registered: Cell<bool>,
    scope: Scope,
    clear: fn(),
    name: &'static str,
}

thread_local! {
    static CLASS_CLEARERS: RefCell<Vec<fn()>> = const { RefCell::new(Vec::new()) };
    static SESSION_CLEARERS: RefCell<Vec<fn()>> = const { RefCell::new(Vec::new()) };
}

impl<T> Arena<T> {
    pub const fn new(scope: Scope, clear: fn(), name: &'static str) -> Self {
        Arena {
            slots: RefCell::new(Vec::new()),
            epoch: Cell::new(0),
            registered: Cell::new(false),
            scope,
            clear,
            name,
        }
    }

    pub fn alloc(&self, value: T) -> RawHandle {
        if !self.registered.replace(true) {
            let list = match self.scope {
                Scope::Class => &CLASS_CLEARERS,
                Scope::Session => &SESSION_CLEARERS,
            };
            list.with(|l| l.borrow_mut().push(self.clear));
        }
        let mut slots = self.slots.borrow_mut();
        let index = u32::try_from(slots.len()).expect("arena overflow");
        slots.push(Some(Rc::new(RefCell::new(value))));
        RawHandle {
            index,
            epoch: self.epoch.get(),
        }
    }

    pub fn get(&self, handle: RawHandle) -> Rc<RefCell<T>> {
        if handle.epoch != self.epoch.get() {
            panic!(
                "stale {} handle #{} used after its arena was reset",
                self.name, handle.index
            );
        }
        match &self.slots.borrow()[handle.index as usize] {
            Some(cell) => cell.clone(),
            None => released(self.name, handle.index),
        }
    }

    /// Drop the contents of every object allocated at or after index `mark`
    /// that `keep` (given the index; it must not touch this arena) rejects.
    pub fn release_since(&self, mark: u32, keep: impl Fn(u32) -> bool) {
        let len = self.slots.borrow().len() as u32;
        let mut dropped = Vec::new();
        for index in mark..len {
            if !keep(index) {
                if let Some(cell) = self.slots.borrow_mut()[index as usize].take() {
                    dropped.push(cell);
                }
            }
        }
        drop(dropped);
    }

    pub fn is_live(&self, handle: RawHandle) -> bool {
        handle.epoch == self.epoch.get()
    }

    pub fn clear(&self) {
        let dropped = std::mem::take(&mut *self.slots.borrow_mut());
        self.epoch.set(self.epoch.get().wrapping_add(1));
        drop(dropped);
    }

    pub fn len(&self) -> usize {
        self.slots.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// Drop every class-scope object on this thread; their handles become stale.
pub fn reset_class_scope() {
    let clearers = CLASS_CLEARERS.with(|l| l.borrow().clone());
    for clear in clearers {
        clear();
    }
}

/// Drop every arena object on this thread.
pub fn reset_session_scope() {
    reset_class_scope();
    let clearers = SESSION_CLEARERS.with(|l| l.borrow().clone());
    for clear in clearers {
        clear();
    }
}

#[cold]
fn released(name: &str, index: u32) -> ! {
    panic!("released {name} handle #{index} used after its owner declared it garbage");
}

#[cold]
pub fn borrow_conflict(name: &str, index: u32, mutable: bool) -> ! {
    let what = if mutable { "with_mut" } else { "with" };
    panic!(
        "{name}#{index}: `{what}` re-entered while the same object is mutably borrowed; \
         copy the fields out and call from outside the enclosing closure"
    );
}

/// Declare an arena-backed Java object: `arena_object!(pub Statement => StatementData, Class);`
///
/// Generates the `Copy` handle `Statement` with `alloc`, `with`, `with_mut` and
/// `index`. Equality, ordering and hashing on the handle are by identity.
#[macro_export]
macro_rules! arena_object {
    ($(#[$meta:meta])* $vis:vis $name:ident => $data:ty, $scope:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis struct $name($crate::rt::arena::RawHandle);

        impl $name {
            fn __arena<R>(f: impl FnOnce(&$crate::rt::arena::Arena<$data>) -> R) -> R {
                thread_local! {
                    static ARENA: $crate::rt::arena::Arena<$data> = const {
                        $crate::rt::arena::Arena::new(
                            $crate::rt::arena::Scope::$scope,
                            $name::__clear,
                            stringify!($name),
                        )
                    };
                }
                ARENA.with(f)
            }

            fn __clear() {
                Self::__arena(|a| a.clear())
            }

            pub fn alloc(data: $data) -> Self {
                $name(Self::__arena(|a| a.alloc(data)))
            }

            pub fn with<R>(self, f: impl FnOnce(&$data) -> R) -> R {
                let cell = Self::__arena(|a| a.get(self.0));
                let Ok(guard) = cell.try_borrow() else {
                    $crate::rt::arena::borrow_conflict(stringify!($name), self.0.index(), false)
                };
                f(&guard)
            }

            pub fn with_mut<R>(self, f: impl FnOnce(&mut $data) -> R) -> R {
                let cell = Self::__arena(|a| a.get(self.0));
                let Ok(mut guard) = cell.try_borrow_mut() else {
                    $crate::rt::arena::borrow_conflict(stringify!($name), self.0.index(), true)
                };
                f(&mut guard)
            }

            /// False once the arena this object lived in has been reset.
            pub fn is_live(self) -> bool {
                Self::__arena(|a| a.is_live(self.0))
            }

            /// Creation-order index within the arena; stable for the object's lifetime.
            pub fn index(self) -> u32 {
                self.0.index()
            }

            /// Number of objects allocated in this arena since its last reset: the
            /// index the next allocation gets.
            pub fn arena_len() -> u32 {
                Self::__arena(|a| a.len() as u32)
            }

            /// Release every object allocated at or after index `mark` whose
            /// index `keep` rejects; `keep` must not touch this arena.
            pub fn release_since(mark: u32, keep: impl Fn(u32) -> bool) {
                Self::__arena(|a| a.release_since(mark, keep))
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}#{}", stringify!($name), self.0.index())
            }
        }
    };
}

/// Give an arena handle Java's default `equals`/`hashCode` (identity).
#[macro_export]
macro_rules! arena_identity {
    ($name:ident) => {
        impl $crate::rt::JavaEq for $name {
            fn java_equals(&self, other: &Self) -> bool {
                self == other
            }
        }
        impl $crate::rt::JavaHash for $name {
            fn java_hash(&self) -> i32 {
                self.index() as i32
            }
        }
    };
}

/// Declare a Java subclass as a typed view over one variant of its base
/// object's `kind` enum:
///
/// `java_subclass!(pub IfStatement: Statement, StatementKind::If(IfStatementData), is_if, as_if, try_as_if);`
///
/// The view derefs to the base handle, `d`/`d_mut` reach the subclass fields,
/// and the base gains `is_if()` (`instanceof`), `as_if()` (a checked cast) and
/// `try_as_if()`.
#[macro_export]
macro_rules! java_subclass {
    ($(#[$meta:meta])* $vis:vis $view:ident : $base:ident, $kind:ident :: $variant:ident ( $data:ty ),
     $is:ident, $as_:ident, $try_as:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis struct $view(pub $base);

        impl ::std::ops::Deref for $view {
            type Target = $base;
            fn deref(&self) -> &$base {
                &self.0
            }
        }

        impl From<$view> for $base {
            fn from(view: $view) -> $base {
                view.0
            }
        }

        impl ::std::fmt::Debug for $view {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}#{}", stringify!($view), self.0.index())
            }
        }

        impl $view {
            /// Read the subclass fields.
            pub fn d<R>(self, f: impl FnOnce(&$data) -> R) -> R {
                self.0.with(|base| match &base.kind {
                    $kind::$variant(data) => f(data),
                    _ => unreachable!(concat!("not a ", stringify!($view))),
                })
            }

            /// Mutate the subclass fields.
            pub fn d_mut<R>(self, f: impl FnOnce(&mut $data) -> R) -> R {
                self.0.with_mut(|base| match &mut base.kind {
                    $kind::$variant(data) => f(data),
                    _ => unreachable!(concat!("not a ", stringify!($view))),
                })
            }
        }

        impl $base {
            pub fn $is(self) -> bool {
                self.with(|base| matches!(base.kind, $kind::$variant(_)))
            }

            pub fn $try_as(self) -> Option<$view> {
                self.$is().then_some($view(self))
            }

            #[track_caller]
            pub fn $as_(self) -> $view {
                if !self.$is() {
                    $crate::rt::exception::throw(
                        "ClassCastException",
                        concat!(stringify!($base), " is not a ", stringify!($view)),
                    );
                }
                $view(self)
            }
        }
    };
}
