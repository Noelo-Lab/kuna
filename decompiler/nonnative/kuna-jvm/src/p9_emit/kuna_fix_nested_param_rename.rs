//! kuna fix `nested_param_rename` (`kuna_fixes`): `NestedClassProcessor.insertLocalVars`
//! renames the variables of a local or anonymous class's methods whose names
//! are also names the enclosing method's `VarProcessor` gives the variables
//! the class captures (`VarProcessor.refreshVarNames`). It changes only the
//! `VarProcessor` name. A use of a variable is written by its
//! local-variable-table name, which takes precedence over that name, while
//! `ClassWriter` writes a parameter's declaration by the `VarProcessor` name:
//! the method declares `Exception ex` and its body reads `e`, which javac
//! cannot resolve. (The captured variables themselves are written by their
//! own table names, so the table name is not the clash the rename avoids.)
//! With the fix a parameter whose uses are written by a table name is
//! declared under that name. A table name that is not a Java identifier
//! (kotlinc's setter parameter `<set-?>`, an inline-class receiver
//! `$v$c$kotlin-UIntArray$-$this$get$0`) is not: the uses are written by the
//! `VarProcessor` name then (`var1/* $VF was: <set-?> */`), and so is the
//! declaration.

use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// The name the uses of parameter `pair` are written by when it is a
/// local-variable-table name other than `name`.
pub fn table_name(varproc: VarProcessor, pair: VarVersionPair, name: Option<&str>) -> Option<String> {
    if !DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
        return None;
    }
    let lvt = varproc.get_var_lvt(pair)?.get_name();
    (Some(lvt.as_str()) != name && VarExprent::is_valid_name(&lvt)).then_some(lvt)
}
