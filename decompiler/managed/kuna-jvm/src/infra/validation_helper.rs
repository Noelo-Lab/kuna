//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ValidationHelper.java`:
//! the two checks that always run. Vineflower's graph validators, which ran
//! only under `VALIDATE_DECOMPILED_CODE=true`, are not kept.

use crate::jthrow;

pub struct ValidationHelper;

impl ValidationHelper {
    /// Java: notNull(Object)
    pub fn not_null<T>(o: Option<&T>) {
        if o.is_none() {
            jthrow!(NullPointerException; "Null not expected here");
        }
    }

    pub fn assert_true(condition: bool, message: &str) {
        if !condition {
            jthrow!(IllegalStateException; "Assertion failed: {}", message);
        }
    }
}
