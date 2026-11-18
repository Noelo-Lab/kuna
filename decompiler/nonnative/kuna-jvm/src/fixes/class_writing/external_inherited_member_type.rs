//! kuna fix `external_inherited_member_type` (`kuna_fixes`), called from
//! `ImportCollector::get_super_class_inner_classes`.
//!
//! A member class a class inherits shadows a single-type import of the same
//! simple name in its body (JLS 6.4.1), and two different member classes of
//! one name inherited from two supertypes make the name ambiguous (JLS 8.5).
//! `ImportCollector.getSuperClassInnerClasses` learns the inherited member
//! classes by walking the supertypes in the class universe only, so a
//! supertype outside it contributes nothing: Elasticsearch's
//! `InternalFilters` implements `Filters` and extends a class implementing
//! `MultiBucketsAggregation`, both outside the input and both declaring a
//! `Bucket`, and its `implements Filters.Bucket` is written `Bucket`
//! under an import, which javac finds ambiguous. With the fix, the member
//! classes of a supertype outside the universe are read from the
//! `InnerClasses` entries of the classes the walk visited (javac records
//! every nested class a class file names there), and a simple name two
//! different inherited member classes share maps to no class, so every class
//! of that name is written qualified. Only members a subclass inherits count
//! (not private ones, nor package-private ones of another package), and
//! Kotlin output, where nested classes of supertypes are not in scope, is
//! left alone.

use crate::code_constants::{ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC};
use crate::rt::JHashMap;
use crate::struct_inner_classes_attribute::Entry;

/// Whether the member class of `entry` is inherited by a class of package
/// `package` (internal name prefix, `a/b/`): it is not private, and a
/// package-private one only within its package (JLS 8.5).
pub fn inherited_by(entry: &Entry, package: &str) -> bool {
    if (entry.access_flags & ACC_PRIVATE) != 0 {
        return false;
    }
    if (entry.access_flags & (ACC_PUBLIC | ACC_PROTECTED)) != 0 {
        return true;
    }
    let enclosing = entry.enclosing_name.as_deref().unwrap_or("");
    let own = enclosing.rfind('/').map_or("", |i| &enclosing[..=i]);
    own == package
}

/// Adds to `names` the member classes of each supertype in `external` (outside
/// the universe) found in `entries`, and marks a simple name with two different
/// inherited member classes (`inherited`, from the supertypes in the
/// universe, and these) as ambiguous: it maps to no class.
pub fn add_external_members(
    names: &mut JHashMap<String, String>,
    inherited: &mut Vec<(String, String)>,
    external: &[String],
    entries: &[Entry],
    package: &str,
) {
    for entry in entries {
        let (Some(enclosing), Some(simple)) = (&entry.enclosing_name, &entry.simple_name) else {
            continue;
        };
        if external.iter().any(|e| e == enclosing) && inherited_by(entry, package) {
            let inner = entry.inner_name.replace(['/', '$'], ".");
            if !inherited.iter().any(|(s, i)| s == simple && *i == inner) {
                inherited.push((simple.clone(), inner.clone()));
                names.put(simple.clone(), inner);
            }
        }
    }
    for (simple, inner) in inherited.iter() {
        if inherited.iter().any(|(s, i)| s == simple && i != inner) {
            names.put(simple.clone(), String::new());
        }
    }
}
