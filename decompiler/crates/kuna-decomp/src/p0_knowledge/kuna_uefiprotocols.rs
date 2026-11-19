//! (kuna `uefi`) The protocol and PPI interface instances an EFI module
//! installs, as the `uefi` analysis pass found them.
//!
//! Parked on the [`Architecture`](crate::architecture::Architecture) so a
//! front-end can list them: `kuna unpack --protocols` writes them into the
//! index that links a lookup in one module of a firmware volume to the module
//! that installs the interface.

/// One installed interface instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstalledInterface {
    /// The protocol or PPI GUID, in memory order.
    pub guid: [u8; 16],
    /// Where the instance is.
    pub addr: u64,
    /// The name of its struct.
    pub interface: String,
    /// The struct's members as `(name, type)`, when the pass made the struct
    /// up from the instance; empty for a struct of the built-in tables.
    pub fields: Vec<(String, String)>,
    /// Each function the instance points at: member name and entry.
    pub members: Vec<(String, u64)>,
}
