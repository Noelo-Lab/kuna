//! Port of `org/jetbrains/java/decompiler/main/decompiler/CancelationManager.java`.
//!
//! The checker is process-global like the Java static, so a host thread can
//! install one that observes its own cancel flag; the decompiling thread polls it.
//!
//! kuna adds a per-thread method deadline and method guard, polled by the same
//! [`CancelationManager::check_canceled`] calls: the deadline replaces the
//! watchdog thread Vineflower kills when `max-time-per-method` runs out (see
//! `ClassWrapper::init`), the guard lets a host abort the method being
//! decompiled. Neither costs more than one thread-local read while unarmed.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;

type Checker = Arc<dyn Fn() + Send + Sync>;

static CANCELATION_CHECKER: RwLock<Option<Checker>> = RwLock::new(None);
static HAS_CHECKER: AtomicBool = AtomicBool::new(false);

pub struct CancelationManager;

impl CancelationManager {
    /// Throws `CanceledException`.
    pub fn cancel() -> ! {
        CanceledException::throw()
    }

    pub fn check_canceled() {
        if HAS_CHECKER.load(Ordering::Acquire) {
            let checker = CANCELATION_CHECKER
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            if let Some(checker) = checker {
                checker();
            }
        }
        let armed = METHOD_POLL.with(Cell::get);
        if armed.deadline.is_some() || armed.guarded {
            poll_method(armed);
        }
    }

    pub fn set_cancelation_checker(checker: impl Fn() + Send + Sync + 'static) {
        *CANCELATION_CHECKER
            .write()
            .unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(checker));
        HAS_CHECKER.store(true, Ordering::Release);
    }

    /// Restores the default no-op checker.
    pub fn clear_cancelation_checker() {
        HAS_CHECKER.store(false, Ordering::Release);
        *CANCELATION_CHECKER
            .write()
            .unwrap_or_else(|e| e.into_inner()) = None;
    }
}

#[derive(Clone, Copy)]
struct MethodPoll {
    deadline: Option<Instant>,
    guarded: bool,
}

type MethodGuard = Rc<dyn Fn() -> Option<String>>;

thread_local! {
    static METHOD_POLL: Cell<MethodPoll> = const {
        Cell::new(MethodPoll {
            deadline: None,
            guarded: false,
        })
    };
    static METHOD_GUARD: RefCell<Option<MethodGuard>> = const { RefCell::new(None) };
}

#[cold]
fn poll_method(armed: MethodPoll) {
    if armed.guarded {
        let guard = METHOD_GUARD.with(|g| g.borrow().clone());
        if let Some(reason) = guard.and_then(|g| g()) {
            crate::rt::throw(MethodAbortedException::CLASS, reason);
        }
    }
    if armed.deadline.is_some_and(|d| Instant::now() >= d) {
        crate::rt::throw(MethodDeadlineExceeded::CLASS, "");
    }
}

impl CancelationManager {
    /// Arms (or with `None` disarms) this thread's method deadline; returns the previous one.
    pub fn set_method_deadline(deadline: Option<Instant>) -> Option<Instant> {
        METHOD_POLL.with(|p| {
            let mut armed = p.get();
            let previous = armed.deadline;
            armed.deadline = deadline;
            p.set(armed);
            previous
        })
    }

    /// Installs (or with `None` removes) this thread's method guard: polled with
    /// the deadline, a returned reason aborts the method being decompiled with a
    /// `MethodAbortedException`, which fails that method like any other error.
    pub fn set_method_guard(guard: Option<Rc<dyn Fn() -> Option<String>>>) {
        let guarded = guard.is_some();
        METHOD_GUARD.with(|g| *g.borrow_mut() = guard);
        METHOD_POLL.with(|p| {
            let mut armed = p.get();
            armed.guarded = guarded;
            p.set(armed);
        });
    }
}

/// Thrown at a poll once the method deadline has passed; only the watchdog
/// branch of `ClassWrapper::init` arms the deadline and catches it.
pub struct MethodDeadlineExceeded;

impl MethodDeadlineExceeded {
    pub const CLASS: &'static str = "MethodDeadlineExceeded";

    /// Whether an unwinding payload is this exception.
    pub fn is_payload(payload: &(dyn std::any::Any + Send)) -> bool {
        payload
            .downcast_ref::<crate::rt::JavaException>()
            .is_some_and(|e| e.class == Self::CLASS)
    }
}

/// Thrown at a poll when the method guard returns a reason.
pub struct MethodAbortedException;

impl MethodAbortedException {
    pub const CLASS: &'static str = "MethodAbortedException";
}

pub struct CanceledException;

impl CanceledException {
    pub const CLASS: &'static str = "CanceledException";

    pub fn throw() -> ! {
        crate::rt::throw(Self::CLASS, "")
    }

    pub fn is(caught: &crate::rt::Caught) -> bool {
        caught.is(Self::CLASS)
    }
}
