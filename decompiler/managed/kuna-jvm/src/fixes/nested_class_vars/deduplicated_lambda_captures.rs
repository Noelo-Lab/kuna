//! Fix `deduplicated_lambda_captures`: javac compiles lambdas with equal
//! bodies to one synthetic method when it writes no local variable debug
//! information (its default), so one implementation method is the body of
//! lambdas in several methods, each capturing its own variables. The
//! implementation's captured parameters are named once, from the first
//! lambda that names them, and every other lambda's body was printed with
//! that lambda's names: `String var5 = var4.toString(); run(() ->
//! var4.hashCode())` for a lambda that captures `var5`, which compiles and
//! reads another variable. Each lambda's body is written with the names its
//! own captured arguments have where it is created.

use crate::classes_processor::ClassNode;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// Names the captured parameters of `lambda`'s implementation, the lambda
/// `node`, by `lambda`'s own captured arguments where another lambda named
/// them, until the returned guard drops.
pub fn name_at_site(lambda: NewExprent, node: ClassNode) -> Option<SiteNames> {
    let (info, parent) = node.with(|d| (d.lambda_information.clone(), d.parent));
    let info = info.filter(|i| !i.is_method_reference)?;
    let content = parent?.get_wrapper()?.get_methods().borrow().get_with_key(&info.content_method_key)?;
    let processor = content.with(|d| d.varproc);
    let md_content = MethodDescriptor::parse_descriptor(&info.content_method_descriptor);
    let md_lambda = MethodDescriptor::parse_descriptor(&info.method_descriptor);
    let captured = md_content.params.len().checked_sub(md_lambda.params.len())?;
    let arguments = lambda.get_constructor()?.get_lst_parameters();
    let first = if info.is_content_method_static { 0 } else { 1 };
    let mut slot = first as i32;
    let mut previous = Vec::new();
    for (i, param) in md_content.params.iter().take(captured).enumerate() {
        let pair = VarVersionPair::new(slot, 0);
        slot += param.stack_size;
        let Some(var) = arguments.to_vec().get(first + i).and_then(|a| a.try_as_var()) else {
            continue;
        };
        let site = var.get_processor().map(|p| (p, VarVersionPair::from_var(var)));
        let named = processor.get_captured_source(pair);
        if site.is_none() || named.is_none() || site == named {
            continue;
        }
        previous.push((pair, processor.get_clashing_name(pair)));
        processor.set_clashing_name(pair, var.get_name());
    }
    (!previous.is_empty()).then_some(SiteNames { processor, previous })
}

/// Restores the names [`name_at_site`] replaced when dropped.
pub struct SiteNames {
    processor: VarProcessor,
    previous: Vec<(VarVersionPair, Option<String>)>,
}

impl Drop for SiteNames {
    fn drop(&mut self) {
        for (pair, name) in self.previous.drain(..) {
            self.processor.restore_clashing_name(pair, name);
        }
    }
}
