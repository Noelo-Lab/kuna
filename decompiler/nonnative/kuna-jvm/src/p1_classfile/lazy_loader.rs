//! Port of `org/jetbrains/java/decompiler/struct/lazy/LazyLoader.java`.
//!
//! In Vineflower 1.12 the whole class body is commented out: method bodies
//! stay in `StructMethod` as raw bytes and are parsed by `expand_data`.

pub struct LazyLoader;
