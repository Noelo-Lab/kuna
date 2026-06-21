//! Port of `decompiler/cpp/printjava.{cc,hh}` — the Java-language back-end.
//!
//! ## Deferred (LOSS)
//!
//! `PrintJava` derives from `PrintC` (`printjava.hh:57`, `class PrintJava :
//! public PrintC`): it reuses every operator token, the prototype/structuring
//! emission, and the RPN driver, overriding only the Java-specific
//! `printUnicode`/`pushTypeStart`/`pushTypeEnd`/`adjustTypeOperators`/
//! `opLoad`/`opStore`/`opCallind`/`opCpoolRefOp` and the `instanceof` keyword.
//! Because it is a thin specialization of the (separate W8 item) `PrintC`
//! emitter, and because **no oracle datatest selects the `java-language`
//! back-end** (the M2/M3 corpus is all `c-language`), `PrintJava` itself is
//! deferred per the W8 deferral plan — it is not on any byte-for-byte string
//! match path.
//!
//! What is **not** deferred is the Java *cast* surface: `cast.cc` defines
//! [`crate::cast::CastStrategyJava`] (`CastStrategyJava : public CastStrategyC`,
//! the `castStandard`/`isZextCast` overrides for Java's pointer-encoded object
//! references), and that lives in [`crate::cast`] alongside `CastStrategyC`.
//! `printjava.cc`'s `PrintJavaCapability::buildLanguage` would construct a
//! `PrintJava` whose `castStrategy` is a `CastStrategyJava`; the strategy is
//! ready, only the emitter wiring is deferred.
//!
//! When the W8 `PrintC` emitter lands, `PrintJava` is a small follow-on: the
//! token table is shared, and the eight overrides above plus
//! `PrintJavaCapability` registration are all that is left.

use kuna_base::error::{KunaError, KunaResult};

/// Placeholder for the (deferred) `PrintJava` Java-language emitter.
///
/// Constructing it is an explicit `Err` (`printjava deferred — LOSS`): the Java
/// back-end is not exercised by any oracle datatest and is intentionally not
/// ported in W8.  The Java cast strategy it would use is available now as
/// [`crate::cast::CastStrategyJava`].
pub struct PrintJava;

impl PrintJava {
    /// Build the Java-language emitter — deferred (LOSS).  See the module docs.
    pub fn new() -> KunaResult<PrintJava> {
        Err(KunaError::lowlevel(
            "printjava deferred — LOSS: PrintJava (PrintC specialization) is not \
             ported in W8; no oracle datatest selects the java-language back-end. \
             The Java cast strategy is available as cast::CastStrategyJava.",
        ))
    }
}
