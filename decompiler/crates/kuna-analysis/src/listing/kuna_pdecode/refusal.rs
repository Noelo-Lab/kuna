//! Serial-fallback reasons shared by the decode gates and their diagnostics.

macro_rules! refusals {
    ($($(#[$doc:meta])* $variant:ident => $reason:literal),+ $(,)?) => {
        /// Why the walk ran serially, either at the gate or after a lane fault.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Refusal {
            $($(#[$doc])* $variant),+
        }

        impl Refusal {
            pub const COUNT: usize = [$(stringify!($variant)),+].len();
            pub const ALL: [Self; Self::COUNT] = [$(Self::$variant),+];

            /// The stderr spelling, stable across releases.
            pub fn reason(self) -> &'static str {
                match self {
                    $(Self::$variant => $reason),+
                }
            }
        }
    };
}

refusals! {
    /// The target has no threads (wasm).
    NoThreads => "no threads on this target",
    /// No rebuildable engine: not a standalone `Sleigh`, or no `.sla` bytes.
    NoEngine => "no rebuildable decode engine",
    /// A constructor's `globalset` can change how another address decodes.
    ContextCommits => "language commits context",
    /// A delay-slot decode fetches past its own instruction.
    DelaySlots => "language has delay slots",
    /// The loader cannot share its live, patched bytes.
    NoSharedBytes => "loader cannot share its bytes",
    /// A per-address decode mode is painted into the context database.
    ContextPaint => "per-address decode context",
    /// Nothing to walk.
    NoSeeds => "no seeds",
    /// An unmapped executable address makes the loader window history-dependent.
    UnmappedExec => "executable range not fully mapped",
    /// The walk is too small to pay for the engine builds.
    TooSmall => "executable image too small",
    /// A lane's engine could not be built.
    KitFailed => "engine rebuild failed",
    /// A rebuilt engine disagreed with the parent on a sampled decode.
    KitDisagrees => "rebuilt engine disagrees",
    /// The OS refused a lane thread.
    SpawnFailed => "thread spawn failed",
    /// A lane panicked.
    LaneFault => "lane fault",
    /// A lane fetched bytes at an unmapped address.
    UnmappedFetch => "unmapped fetch",
    /// Two shards claimed one address (a router bug).
    Collision => "merge collision",
    /// The rounds did not converge.
    RoundLimit => "round limit",
    /// The parent's context database moved during the walk.
    ContextMoved => "context moved",
}

#[cfg(test)]
mod tests {
    use super::Refusal;

    #[test]
    fn public_order_and_spellings_stay_compatible() {
        assert_eq!(
            Refusal::ALL.map(Refusal::reason),
            [
                "no threads on this target",
                "no rebuildable decode engine",
                "language commits context",
                "language has delay slots",
                "loader cannot share its bytes",
                "per-address decode context",
                "no seeds",
                "executable range not fully mapped",
                "executable image too small",
                "engine rebuild failed",
                "rebuilt engine disagrees",
                "thread spawn failed",
                "lane fault",
                "unmapped fetch",
                "merge collision",
                "round limit",
                "context moved",
            ]
        );
    }
}
