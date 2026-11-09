//! Port of `org/jetbrains/java/decompiler/main/decompiler/CancelationManager.java`.
//!
//! The checker is process-global like the Java static, so a host thread can
//! install one that observes its own cancel flag; the decompiling thread polls it.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

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
        if !HAS_CHECKER.load(Ordering::Acquire) {
            return;
        }
        let checker = CANCELATION_CHECKER
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        if let Some(checker) = checker {
            checker();
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
