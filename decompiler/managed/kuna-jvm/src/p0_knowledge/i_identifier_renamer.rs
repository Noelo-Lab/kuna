//! The element kinds of `org/jetbrains/java/decompiler/main/extern/IIdentifierRenamer.java`;
//! the one renamer is `ConverterHelper`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    ElementClass,
    ElementField,
    ElementMethod,
}
