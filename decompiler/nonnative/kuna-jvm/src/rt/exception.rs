//! Java unchecked exceptions as unwinding panics.
//!
//! Vineflower aborts a method by throwing and recovers at the method or class
//! boundary. The port keeps that shape: [`throw`] unwinds with a
//! [`JavaException`] payload and [`try_catch`] is the `try { } catch (Throwable)`
//! that receives it. An ordinary Rust panic inside the region (an index out of
//! bounds, an `unwrap` on `None`) is caught the same way, as the
//! `IndexOutOfBoundsException` or `NullPointerException` it stands for.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;

/// A thrown Java exception: its simple class name and message.
#[derive(Clone, Debug)]
pub struct JavaException {
    pub class: &'static str,
    pub message: String,
}

impl std::fmt::Display for JavaException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.message.is_empty() {
            f.write_str(self.class)
        } else {
            write!(f, "{}: {}", self.class, self.message)
        }
    }
}

/// What a [`try_catch`] region caught.
#[derive(Clone, Debug)]
pub struct Caught {
    /// The Java exception, or `None` for a Rust panic.
    pub exception: Option<JavaException>,
    /// `Throwable.toString()`-style text.
    pub message: String,
    /// Source location of the throw or panic.
    pub location: String,
}

impl Caught {
    /// Whether the caught throwable is of the named Java class.
    pub fn is(&self, class: &str) -> bool {
        self.exception.as_ref().is_some_and(|e| e.class == class)
    }
}

thread_local! {
    static CATCH_DEPTH: Cell<u32> = const { Cell::new(0) };
    static LAST_PANIC: RefCell<Option<(String, String)>> = const { RefCell::new(None) };
}

static HOOK: Once = Once::new();

fn payload_text(payload: &(dyn Any + Send)) -> String {
    if let Some(e) = payload.downcast_ref::<JavaException>() {
        e.to_string()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".to_string()
    }
}

fn install_hook() {
    HOOK.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if CATCH_DEPTH.with(Cell::get) == 0 {
                previous(info);
                return;
            }
            let location = info
                .location()
                .map(|l| format!("{}:{}", l.file(), l.line()))
                .unwrap_or_default();
            LAST_PANIC.with(|p| *p.borrow_mut() = Some((payload_text(info.payload()), location)));
        }));
    });
}

/// `throw new <class>(message)`.
#[track_caller]
pub fn throw(class: &'static str, message: impl Into<String>) -> ! {
    panic::panic_any(JavaException {
        class,
        message: message.into(),
    })
}

/// Rethrow something a [`try_catch`] caught.
pub fn rethrow(caught: Caught) -> ! {
    match caught.exception {
        Some(e) => panic::panic_any(e),
        None => panic::panic_any(caught.message),
    }
}

/// `try { body } catch (Throwable t) { ... }`: the error arm receives [`Caught`].
pub fn try_catch<R>(body: impl FnOnce() -> R) -> Result<R, Caught> {
    install_hook();
    CATCH_DEPTH.with(|d| d.set(d.get() + 1));
    let result = panic::catch_unwind(AssertUnwindSafe(body));
    CATCH_DEPTH.with(|d| d.set(d.get() - 1));
    result.map_err(|payload| {
        let (hook_text, location) = LAST_PANIC
            .with(|p| p.borrow_mut().take())
            .unwrap_or_default();
        let exception = payload.downcast_ref::<JavaException>().cloned();
        let message = match &exception {
            Some(e) => e.to_string(),
            None if hook_text.is_empty() => payload_text(payload.as_ref()),
            None => hook_text,
        };
        Caught {
            exception,
            message,
            location,
        }
    })
}

/// `throw new RuntimeException(format!(...))`, or another class when named first.
#[macro_export]
macro_rules! jthrow {
    ($class:ident; $($arg:tt)*) => {
        $crate::rt::exception::throw(stringify!($class), format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::rt::exception::throw("RuntimeException", format!($($arg)*))
    };
}

/// Marks a method whose body has not been ported yet; throws `UnportedError`.
#[macro_export]
macro_rules! unported {
    ($what:literal) => {
        $crate::rt::exception::throw("UnportedError", $what)
    };
}
