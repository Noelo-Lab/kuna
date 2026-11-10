//! kuna fix `pattern_binding_shape` (`kuna_fixes`): javac compiles a type
//! pattern `e instanceof T v` (JDK 16 and later, `TransPatterns`) to one
//! shape: the subject is read once, from a local variable that is the
//! source's own (`x instanceof T v`) or a temporary it stores `e` in, and the
//! binding `aload; checkcast T; astore v` comes right after the
//! `instanceof T; ifeq` that skips it, also for a negated pattern
//! (`if (!(x instanceof T v)) return;`, whose binding a `goto` follows).
//! `IfPatternMatchProcessor` makes a pattern from any `instanceof` and a store
//! of the cast value at the head of the branch it guards, so the source's
//! test and cast became a pattern too: a field, array element or other
//! expression read once for the test and again for the cast
//! (`if (this.f instanceof T) { T v = (T) this.f;`), and a cast after a
//! negated test's exit (`if (!(x instanceof T)) return; T v = (T) x;`, the
//! cast at the `ifne`'s target). With the fix the pattern is made only from
//! javac's shape; otherwise the `instanceof` and the cast stay as the
//! bytecode has them. A captured variable of a local or anonymous class reads
//! the synthetic field javac gives it twice; it counts as a local.

use crate::code_constants::{OPC_IFEQ, OPC_INSTANCEOF};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionExprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::rt::BitSet;
use crate::struct_method::StructMethod;

/// Whether `iof`, an `instanceof` whose operand is `source`, and `binding`,
/// the store of its cast value that would become the pattern's variable,
/// are javac's compiled type pattern in method `mt`. `true` when the
/// offsets are unknown.
pub fn is_javac_pattern(
    mt: StructMethod,
    iof: FunctionExprent,
    source: Exprent,
    binding: Exprent,
) -> bool {
    if !read_once(source) {
        return false;
    }
    let Some(test) = iof.0.get_bytecode().filter(|b| !b.is_empty()) else {
        return true;
    };
    let mut range = BitSet::new();
    binding.get_bytecode_range(Some(&mut range));
    let start = range.next_set_bit(0);
    if start < 0 {
        return true;
    }
    let instanceof = test.next_set_bit(0);
    let jump = instanceof + 3;
    opcode_at(mt, instanceof) != Some(OPC_INSTANCEOF)
        || (opcode_at(mt, jump) == Some(OPC_IFEQ) && start == jump + 3)
}

/// Whether evaluating `source` reads it once: a variable, or the synthetic
/// field holding a variable a local or anonymous class captured.
fn read_once(source: Exprent) -> bool {
    if source.is_var() {
        return true;
    }
    let Some(field) = source.try_as_field() else {
        return false;
    };
    let descriptor = field.get_descriptor().descriptor_string.clone();
    DecompilerContext::get_struct_context()
        .get_class(&field.get_classname())
        .and_then(|cl| cl.get_field(&field.get_name(), &descriptor))
        .is_some_and(|fd| {
            fd.is_synthetic()
                || (DecompilerContext::get_option(IFernflowerPreferences::SYNTHETIC_NOT_SET)
                    && fd.get_name().contains('$'))
        })
}

/// The opcode of the instruction of `mt` at bytecode offset `offset`.
fn opcode_at(mt: StructMethod, offset: i32) -> Option<i32> {
    mt.get_instruction_sequence()?
        .instructions()
        .into_iter()
        .find(|i| i.start_offset() == offset)
        .map(|i| i.opcode())
}
