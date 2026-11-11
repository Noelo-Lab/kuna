//! kuna fix `self_referencing_lambda` (`kuna_fixes`): lambdas whose bodies
//! instantiate themselves.
//!
//! `LambdaProcessor` writes a lambda whose implementation is a synthetic
//! method of its own class in place, as a lambda expression, and nests the
//! lambdas that method creates under it. When an implementation method
//! creates, directly or through the bodies of other lambdas, a lambda it
//! implements itself, that nesting has a cycle and the expansion has no end:
//! Vineflower throws `Lambda X is recursive?!` for one ordering of the
//! one-method cycle and overflows the stack for the others. Such a method is
//! kept as a method of the class instead, and every lambda it implements is
//! written as a method reference to it, which means the same call.

use std::collections::{BTreeMap, BTreeSet};

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::jthrow;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::{JHashMap, JList};

/// The implementation methods among `lambdas` (enclosing method,
/// implementation method) that reach themselves through the lambdas they
/// create.
fn recursive_content_methods(lambdas: &[(String, String)]) -> BTreeSet<String> {
    let mut edges: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (enclosing, content) in lambdas {
        edges.entry(enclosing).or_default().insert(content);
    }
    let mut recursive = BTreeSet::new();
    for start in edges.keys() {
        let mut seen = BTreeSet::new();
        let mut stack: Vec<&str> = edges[start].iter().copied().collect();
        while let Some(method) = stack.pop() {
            if method == *start {
                recursive.insert(start.to_string());
                break;
            }
            if seen.insert(method) {
                stack.extend(edges.get(method).into_iter().flatten().copied());
            }
        }
    }
    recursive
}

/// Turns the inlined lambdas of `nested` whose implementation method is
/// recursive into method references, and takes those methods out of
/// `map_methods_lambda` so that no lambda nests under them. A recursive
/// lambda that captures values besides its receiver has no method-reference
/// form and fails the class.
pub fn write_recursive_lambdas_as_method_references(
    nested: &JList<ClassNode>,
    map_methods_lambda: &mut JHashMap<String, String>,
) {
    let inlined: Vec<ClassNode> = nested
        .to_vec()
        .into_iter()
        .filter(|nd| {
            nd.with(|d| {
                d.type_ == ClassNodeType::Lambda
                    && d.lambda_information
                        .as_ref()
                        .is_some_and(|li| !li.is_method_reference)
            })
        })
        .collect();
    let edges: Vec<(String, String)> = inlined
        .iter()
        .filter_map(|nd| {
            nd.with(|d| {
                let content = d.lambda_information.as_ref()?.content_method_key.clone();
                Some((d.enclosing_method.clone()?, content))
            })
        })
        .collect();
    let recursive = recursive_content_methods(&edges);
    if recursive.is_empty() {
        return;
    }

    for nd in inlined {
        let info = nd.with(|d| d.lambda_information.clone());
        let info = info.expect("lambdaInformation");
        if !recursive.contains(&info.content_method_key) {
            continue;
        }
        let md_content = MethodDescriptor::parse_descriptor(&info.content_method_descriptor);
        let md_lambda = MethodDescriptor::parse_descriptor(&info.method_descriptor);
        if md_content.params.len() != md_lambda.params.len() {
            jthrow!(IllegalStateException;
                "Lambda {}.{}{} is recursive and captures variables: it cannot be written in place or as a method reference",
                info.content_class_name, info.content_method_name, info.content_method_descriptor);
        }
        nd.with_mut(|d| {
            if let Some(li) = d.lambda_information.as_mut() {
                li.is_method_reference = true;
            }
        });
    }
    for key in &recursive {
        map_methods_lambda.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::recursive_content_methods;

    fn edges(list: &[(&str, &str)]) -> Vec<(String, String)> {
        list.iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    }

    #[test]
    fn only_methods_on_a_cycle_are_recursive() {
        let none = recursive_content_methods(&edges(&[("main", "a"), ("a", "b"), ("main", "b")]));
        assert!(none.is_empty());
        let own = recursive_content_methods(&edges(&[("init", "body"), ("body", "body")]));
        assert_eq!(own.into_iter().collect::<Vec<_>>(), ["body"]);
        let mutual = recursive_content_methods(&edges(&[
            ("start", "ping"),
            ("ping", "pong"),
            ("pong", "ping"),
            ("pong", "leaf"),
            ("leaf", "x"),
        ]));
        assert_eq!(mutual.into_iter().collect::<Vec<_>>(), ["ping", "pong"]);
    }
}
