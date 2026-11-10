//! The kuna option `jvm_source_patterns` (`kuna_options::SOURCE_PATTERNS`):
//! an `instanceof` and a cast of a class javac compiled stay a test and a
//! cast where the LineNumberTable shows the cast as a statement of its own.
//!
//! javac compiles `if (x instanceof T v)` and `if (x instanceof T) { T v =
//! (T) x; ...}` to the same instructions when `x` is a local (fix
//! `pattern_binding_shape` keeps the other test-and-cast shapes). Their line
//! tables differ: the cast is a statement and starts a line, followed by the
//! next statement on a line of its own, while a pattern's binding is part of
//! the condition's line. [`own_line`] takes the binding for the cast where it
//! starts a line after the `instanceof`'s and the code after it starts the
//! next one. A pattern whose condition ends on a later line than it starts,
//! with the binding first on that line, reads the same way. javac's own
//! temporary subject (a slot above the binding's, which javac allocates
//! first) shows a pattern and keeps it. Docs: `docs/spec/10-jvm.md` section
//! 10.10.

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionExprent;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_options::{self, SOURCE_PATTERNS};
use crate::rt::BitSet;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_method::StructMethod;

/// Whether the option is on and keeps the cast `binding`, the store into
/// `left` of the value `iof` tests in `source`, in method `mt` of a class
/// kotlinc did not compile.
pub fn keeps_cast(
    mt: StructMethod,
    iof: FunctionExprent,
    source: Exprent,
    left: Exprent,
    binding: Exprent,
) -> bool {
    if !kuna_options::form_enabled(SOURCE_PATTERNS)
        || DecompilerContext::get_current_class()
            .is_some_and(|cl| KotlinMetadataAnnotation::find(cl).is_some())
        || javac_temporary(source, left)
    {
        return false;
    }
    let mut range = BitSet::new();
    binding.get_bytecode_range(Some(&mut range));
    let (Some(test), Some(start)) = (
        first_offset(iof.0.get_bytecode()),
        first_offset(Some(range.clone())),
    ) else {
        return false;
    };
    own_line(mt, test, start, range.length() as i32)
}

/// Whether `source`, the tested variable, is the temporary javac stores a
/// pattern's subject in: its slot comes after that of `left`, the binding.
fn javac_temporary(source: Exprent, left: Exprent) -> bool {
    let slot = |e: Exprent| {
        e.try_as_var().map(|v| {
            v.get_processor()
                .and_then(|p| p.get_var_original_index(v.get_index()))
                .unwrap_or(v.get_index())
        })
    };
    matches!((slot(source), slot(left)), (Some(s), Some(b)) if s > b)
}

fn first_offset(bytecode: Option<BitSet>) -> Option<i32> {
    let start = bytecode?.next_set_bit(0);
    (start >= 0).then_some(start)
}

/// Whether the LineNumberTable of `mt` starts a line at `start`, the
/// binding of the `instanceof` at `test`, after the line of the test, and
/// another at `end`, the code after the binding.
fn own_line(mt: StructMethod, test: i32, start: i32, end: i32) -> bool {
    let Some(table) = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE) else {
        return false;
    };
    let lines = table.get_raw_data();
    let line_at = |pc: i32| {
        lines
            .as_chunks::<2>()
            .0
            .iter()
            .find(|e| e[0] == pc)
            .map(|e| e[1])
    };
    line_at(start).is_some_and(|line| line > table.find_line_number(test)) && line_at(end).is_some()
}
