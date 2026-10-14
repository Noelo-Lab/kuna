//! Generated frame unions describe storage, not standalone scalar snapshots.

use std::rc::Rc;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, TypeFactory};
use crate::funcdata::Funcdata;

pub(crate) fn prepare(fd: &mut Funcdata, factory: &dyn TypeFactory) {
    let mut snapshots = Vec::new();
    for (high_id, high) in fd.high_bank().iter() {
        if high.kuna_symbol_type().is_some() {
            continue;
        }
        for index in 0..high.num_instances() {
            let id = high.get_instance(index);
            let Some(value) = fd.vbank().get(id) else {
                continue;
            };
            if value.is_constant()
                || value.is_input()
                || value.is_addr_tied()
                || value.is_persist()
                || value.is_type_lock()
                || !matches!(value.get_size(), 1 | 2 | 4 | 8)
            {
                continue;
            }
            if value
                .get_def()
                .and_then(|id| fd.obank().get(id))
                .is_some_and(|op| op.not_printed())
            {
                continue;
            }
            let ty = value.get_type();
            let parent = ty.get_partial_base().unwrap_or_else(|| Rc::clone(ty));
            if parent.get_name().starts_with("stack_views_") {
                snapshots.push((
                    high_id,
                    id,
                    source_type(fd, id).or_else(|| {
                        factory
                            .get_base(value.get_size(), type_metatype::TYPE_UINT)
                            .ok()
                    }),
                ));
            }
        }
    }
    for (high_id, id, ty) in snapshots {
        if let Some(ty) = ty {
            let integer = matches!(
                ty.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT
            );
            let updated = fd.vn_update_type(id, ty);
            if integer && updated {
                if let Some(high) = fd.high_bank_mut().get_mut(high_id) {
                    high.set_kuna_frame_snapshot();
                }
            }
        }
    }
}

fn source_type(fd: &Funcdata, id: VarnodeId) -> Option<Rc<crate::dtype::Datatype>> {
    use crate::kuna_stackobjects::{ByteSource, SourcePoint};
    use kuna_num::opcodes::OpCode;

    let value = fd.vbank().get(id)?;
    let high = fd.high_bank().get(value.get_high()?)?;
    let mut result = None;
    for index in 0..high.num_instances() {
        let source = fd.vbank().get(high.get_instance(index))?;
        for copy in source.descend_iter() {
            let op = fd.obank().get(copy)?;
            if op.code() != OpCode::CPUI_COPY {
                continue;
            }
            let output = op.get_out().and_then(|v| fd.vbank().get(v))?;
            if !output.is_stack_store() || output.get_size() != value.get_size() {
                continue;
            }
            let point = SourcePoint::op(fd, copy)?;
            for object in &fd.stack_objects {
                if object.address != *output.get_addr()
                    || object.size != output.get_size()
                    || !object
                        .bytes
                        .iter()
                        .all(|bytes| bytes.len() == 1 && bytes.contains(&ByteSource::Write(point)))
                {
                    continue;
                }
                for use_ in &object.uses {
                    let ty = &use_.datatype;
                    if ty.get_metatype() != type_metatype::TYPE_PTR {
                        continue;
                    }
                    if result.as_ref().is_some_and(|prior| !Rc::ptr_eq(prior, ty)) {
                        return None;
                    }
                    result = Some(Rc::clone(ty));
                }
            }
        }
    }
    result
}
