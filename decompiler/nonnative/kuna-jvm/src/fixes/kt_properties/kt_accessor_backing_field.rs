//! kuna fix `kt_accessor_backing_field`: inside a property's own accessor,
//! the property's backing field is written `field`.
//!
//! A getter or setter reads and writes the backing field with `getfield` /
//! `putfield` (`getstatic` / `putstatic` for an `object`'s or a companion's
//! property). Vineflower writes such an access as `this.name` (or
//! `Owner.name`), which in Kotlin is a call of the property's accessor, so a
//! recompiled getter `get() { return this.limit }` calls itself without end
//! and a setter `this.limit = value` likewise. Kotlin names the backing field
//! `field` inside the accessors. While `KProperty` writes an accessor's body,
//! an access of exactly the backing field (the same field of the class file)
//! on the accessor's own receiver is written `field`: in the body, its `this`;
//! in an object or lambda written inside the body, the enclosing instance
//! (`this@Owner`), which there is the accessor's receiver (`object : Runnable {
//! override fun run() { field = v } }` came out `this@Acc.x = v`, a call of the
//! setter that recurses without end).

use std::cell::RefCell;

use crate::decompiler_context::DecompilerContext;
use crate::field_exprent::FieldExprent;
use crate::method_wrapper::MethodWrapper;
use crate::struct_field::StructField;
use crate::text_buffer::TextBuffer;
use crate::var_processor::VarProcessor;

#[derive(Clone)]
struct Accessor {
    field: StructField,
    varproc: VarProcessor,
    method_wrapper: Option<MethodWrapper>,
}

thread_local! {
    static ACCESSOR: RefCell<Vec<Accessor>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the accessor whose body is being written.
pub struct AccessorScope;

impl Drop for AccessorScope {
    fn drop(&mut self) {
        ACCESSOR.with(|a| {
            a.borrow_mut().pop();
        });
    }
}

/// Enters the body of the accessor `accessor` of the property backed by `field`.
pub fn enter(field: StructField, accessor: MethodWrapper) -> AccessorScope {
    let varproc = accessor.with(|d| d.varproc);
    ACCESSOR.with(|a| {
        a.borrow_mut().push(Accessor {
            field,
            varproc,
            method_wrapper: DecompilerContext::get_current_method_wrapper(),
        })
    });
    AccessorScope
}

/// `field`, when `exprent` is the backing field of the accessor being written.
pub fn stringify(exprent: FieldExprent) -> Option<TextBuffer> {
    let accessor = ACCESSOR.with(|a| a.borrow().last().cloned())?;
    let field = accessor.field;
    let static_field = field.has_modifier(crate::code_constants::ACC_STATIC);
    if exprent.is_static() != static_field
        || exprent.get_name() != field.get_name()
        || exprent.get_descriptor().descriptor_string != field.get_descriptor()
    {
        return None;
    }
    let owner = DecompilerContext::get_struct_context().get_class(&exprent.get_classname())?;
    if owner.get_field(&field.get_name(), &field.get_descriptor()) != Some(field) {
        return None;
    }
    if !static_field {
        let this = exprent.get_instance()?.try_as_var()?;
        let own = this.get_index() == 0
            && this.get_processor() == Some(accessor.varproc)
            && DecompilerContext::get_current_method_wrapper() == accessor.method_wrapper;
        let enclosing = || {
            this.get_processor().is_some_and(|p| {
                p.get_this_vars()
                    .borrow()
                    .get_cloned(&this.get_var_version_pair())
                    .is_some_and(|class| class == owner.qualified_name())
            })
        };
        if !own && !enclosing() {
            return None;
        }
    }
    let mut buf = TextBuffer::new();
    buf.add_bytecode_mapping(exprent.get_bytecode().as_ref());
    buf.append("field");
    Some(buf)
}
