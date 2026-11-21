//! Fix `kt_accessor_backing_field`: while an accessor's body is written, a read
//! or store of its property's backing field is written `field`.

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
