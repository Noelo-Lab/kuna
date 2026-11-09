//! Port of `org/jetbrains/java/decompiler/main/collectors/ImportCollector.java`.

use std::collections::VecDeque;

use crate::classes_processor::{string_compare_to, ClassNode, ClassNodeType};
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::rt::{JHashMap, JHashSet};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::text_buffer::TextBuffer;
use crate::{arena_identity, arena_object};

pub struct ImportCollectorData {
    pub map_simple_names: JHashMap<String, String>,
    pub set_not_imported_names: JHashSet<String>,
    /// Field names of the class and all its superclasses.
    pub set_field_names: JHashSet<String>,
    pub map_inner_class_names: JHashMap<String, JHashMap<String, String>>,
    pub current_package_slash: String,
    pub current_package_point: String,
    pub write_locked: bool,
}

arena_object!(pub ImportCollector => ImportCollectorData, Class);
arena_identity!(ImportCollector);

impl ImportCollector {
    const JAVA_LANG_PACKAGE: &'static str = "java.lang";

    /// Java: ImportCollector(ClassNode)
    pub fn new(root: ClassNode) -> ImportCollector {
        let class_struct = root.with(|d| d.class_struct);
        let cl_name = class_struct.with(|d| d.qualified_name.clone());
        let (current_package_slash, current_package_point) = match cl_name.rfind('/') {
            Some(index) => {
                let package_name = &cl_name[..index];
                (format!("{package_name}/"), package_name.replace('/', "."))
            }
            None => (String::new(), String::new()),
        };

        let collector = ImportCollector::alloc(ImportCollectorData {
            map_simple_names: JHashMap::new(),
            set_not_imported_names: JHashSet::new(),
            set_field_names: JHashSet::new(),
            map_inner_class_names: JHashMap::new(),
            current_package_slash,
            current_package_point,
            write_locked: false,
        });

        let mut processed_classes: JHashSet<StructClass> = JHashSet::new();
        let ctx = DecompilerContext::get_struct_context();
        let mut current_class = Some(class_struct);
        while let Some(class) = current_class {
            processed_classes.add(class);
            let fields = class.get_fields().borrow().to_vec();
            for f in fields {
                let name = f.get_name();
                collector.with_mut(|d| d.set_field_names.add(name));
            }

            let super_class = class.with(|d| d.super_class.clone());
            current_class = match super_class {
                Some(super_class) => ctx.get_class(&super_class.get_string()),
                None => None,
            };

            if let Some(next) = current_class {
                if processed_classes.contains(&next) {
                    current_class = None;
                }
            }
        }

        collector.collect_conflicting_short_names(root, &JHashMap::new());
        collector
    }

    /// Java: ImportCollector(ImportCollector)
    pub fn new_copy(other: ImportCollector) -> ImportCollector {
        let data = other.with(|o| {
            let mut map_simple_names = JHashMap::new();
            map_simple_names.put_all(o.map_simple_names.entry_vec());
            let mut set_not_imported_names = JHashSet::new();
            set_not_imported_names.add_all(o.set_not_imported_names.to_vec());
            let mut set_field_names = JHashSet::new();
            set_field_names.add_all(o.set_field_names.to_vec());
            let mut map_inner_class_names = JHashMap::new();
            map_inner_class_names.put_all(o.map_inner_class_names.entry_vec());
            ImportCollectorData {
                map_simple_names,
                set_not_imported_names,
                set_field_names,
                map_inner_class_names,
                current_package_slash: o.current_package_slash.clone(),
                current_package_point: o.current_package_point.clone(),
                write_locked: o.write_locked,
            }
        });
        ImportCollector::alloc(data)
    }

    /// `ClassName`, or the full name when a field of the class shadows the short name.
    pub fn get_short_name_in_class_context(self, class_to_name: &str) -> String {
        let Some(short_name) = self.get_short_name(class_to_name) else {
            return "<unknownclass>".to_string();
        };

        if self.with(|d| d.set_field_names.contains(&short_name)) {
            class_to_name.to_string()
        } else {
            short_name
        }
    }

    /// Java: getShortName(String)
    pub fn get_short_name(self, full_name: &str) -> Option<String> {
        self.get_short_name_imported(full_name, true)
    }

    /// Java: getShortName(String, boolean). `None` where Java returns null (a
    /// nameless non-anonymous nested class).
    pub fn get_short_name_imported(self, full_name: &str, imported: bool) -> Option<String> {
        let mut full_name = full_name.to_string();
        let node = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get_cloned(&full_name.replace('.', "/"));

        let mut result: Option<String> = None;
        let own_node = node.filter(|n| n.with(|d| d.class_struct).is_own());
        if let Some(mut node) = own_node {
            result = node.with(|d| d.simple_name.clone());

            loop {
                let (parent, type_) = node.with(|d| (d.parent, d.type_));
                let Some(parent) = parent else {
                    break;
                };
                let Some(parent_simple_name) = parent.with(|d| d.simple_name.clone()) else {
                    break;
                };
                if type_ != ClassNodeType::Member {
                    break;
                }
                result = Some(format!(
                    "{}.{}",
                    parent_simple_name,
                    result.as_deref().unwrap_or("null")
                ));
                node = parent;
            }

            let type_ = node.with(|d| d.type_);
            if type_ == ClassNodeType::Root {
                full_name = node
                    .with(|d| d.class_struct)
                    .with(|d| d.qualified_name.clone());
                full_name = full_name.replace('/', ".");
            } else {
                if result.is_none() && type_ == ClassNodeType::Anonymous {
                    result = Some(ExprProcessor::UNREPRESENTABLE_TYPE_STRING.to_string());
                }
                return result;
            }
        } else {
            full_name = full_name.replace('$', ".");
        }

        let mut short_name = full_name.clone();
        let mut package_name = String::new();

        let mut last_dot = full_name.rfind('.').map_or(-1, |i| i as i32);
        if last_dot >= 0 {
            short_name = full_name[last_dot as usize + 1..].to_string();
            package_name = full_name[..last_dot as usize].to_string();
        }

        let context = DecompilerContext::get_struct_context();

        let (current_package_slash, current_package_point) = self.with(|d| {
            (
                d.current_package_slash.clone(),
                d.current_package_point.clone(),
            )
        });
        let mut exists_default_class = (context
            .get_class(&format!("{current_package_slash}{short_name}"))
            .is_some()
            && package_name != current_package_point)
            || (context.get_class(&short_name).is_some() && !current_package_point.is_empty());

        let map_key = match DecompilerContext::get_current_class_node() {
            None => String::new(),
            Some(curr_cls) => curr_cls
                .with(|d| d.class_struct)
                .with(|d| d.qualified_name.clone()),
        };
        let inner_class_name = |name: &str| -> Option<String> {
            self.with(|d| {
                d.map_inner_class_names
                    .get(&map_key)
                    .and_then(|names| names.get_cloned(&name.to_string()))
            })
        };

        let inner_short = inner_class_name(&short_name);
        if !exists_default_class
            && inner_short.is_some()
            && inner_short.as_deref() != Some(full_name.as_str())
        {
            if context
                .get_class(&format!(
                    "{}${}",
                    package_name.replace('.', "/"),
                    short_name
                ))
                .is_some()
            {
                last_dot = if last_dot <= 0 {
                    -1
                } else {
                    full_name[..last_dot as usize]
                        .rfind('.')
                        .map_or(-1, |i| i as i32)
                };
                if last_dot >= 0 {
                    let cut = last_dot as usize;
                    result = Some(full_name[cut + 1..].to_string());
                    short_name = package_name[cut + 1..].to_string();
                    package_name = package_name[..cut].to_string();

                    let inner_outer = inner_class_name(&short_name);
                    if inner_outer.is_some()
                        && inner_outer != Some(format!("{package_name}.{short_name}"))
                    {
                        exists_default_class = true;
                        result = None;
                    }
                }
            } else {
                exists_default_class = true;
            }
        }

        let simple_name_package = self.with(|d| d.map_simple_names.get_cloned(&short_name));
        if exists_default_class
            || simple_name_package
                .as_ref()
                .is_some_and(|package| &package_name != package)
        {
            return Some(match result {
                None => full_name,
                Some(result) => {
                    if package_name.is_empty() {
                        result
                    } else {
                        format!("{package_name}.{result}")
                    }
                }
            });
        } else if simple_name_package.is_none() {
            self.with_mut(|d| {
                if !d.write_locked {
                    d.map_simple_names
                        .put(short_name.clone(), package_name.clone());
                    if !imported {
                        d.set_not_imported_names.add(short_name.clone());
                    }
                }
            });
        }

        Some(result.unwrap_or(short_name))
    }

    pub fn write_imports(self, buffer: &mut TextBuffer, add_separator: bool) {
        if DecompilerContext::get_option(IFernflowerPreferences::REMOVE_IMPORTS) {
            return;
        }

        let imports = self.pack_imports();
        for line in &imports {
            buffer
                .append("import ")
                .append(line)
                .append(";")
                .append_line_separator();
        }
        if add_separator && !imports.is_empty() {
            buffer.append_line_separator();
        }
    }

    /// The import lines, ordered by package and then simple name.
    pub fn pack_imports(self) -> Vec<String> {
        let entries = self.with(|d| d.map_simple_names.entry_vec());
        let mut kept: Vec<(String, String)> = entries
            .into_iter()
            .filter(|(key, value)| self.keep_import((key, value)))
            .collect();
        kept.sort_by(|a, b| {
            string_compare_to(&a.1, &b.1).then_with(|| string_compare_to(&a.0, &b.0))
        });
        kept.into_iter()
            .map(|(key, value)| format!("{value}.{key}"))
            .collect()
    }

    /// Whether the `(simple name, package)` entry is written as an import.
    pub fn keep_import(self, ent: (&str, &str)) -> bool {
        let (key, value) = ent;
        self.with(|d| {
            !d.set_not_imported_names.contains(&key.to_string())
                && !value.is_empty()
                && ImportCollector::JAVA_LANG_PACKAGE != value
                && value != d.current_package_point
        })
    }

    fn collect_conflicting_short_names(
        self,
        root: ClassNode,
        root_names: &JHashMap<String, String>,
    ) {
        let mut names = root_names.clone();
        self.get_super_class_inner_classes(root, &mut names);
        let (class_struct, nested) = root.with(|d| (d.class_struct, d.nested.to_vec()));
        let qualified_name = class_struct.with(|d| d.qualified_name.clone());
        let stored = names.clone();
        self.with_mut(|d| d.map_inner_class_names.put(qualified_name, stored));

        for nested in nested {
            self.collect_conflicting_short_names(nested, &names);
        }
    }

    fn get_super_class_inner_classes(self, node: ClassNode, names: &mut JHashMap<String, String>) {
        let ctx = DecompilerContext::get_struct_context();
        let mut processed_classes: JHashSet<StructClass> = JHashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut current_class = Some(node.with(|d| d.class_struct));
        while let Some(class) = current_class {
            processed_classes.add(class);
            if let Some(super_class) = class.with(|d| d.super_class.clone()) {
                queue.push_back(super_class.get_string());
            }

            queue.extend(class.get_interface_names());

            let attribute = class.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES());
            if let Some(attribute) = attribute {
                let qualified_name = class.with(|d| d.qualified_name.clone());
                for entry in attribute.get_entries() {
                    if entry.enclosing_name.as_deref() == Some(qualified_name.as_str()) {
                        let inner_name = entry
                            .inner_name
                            .as_ref()
                            .unwrap()
                            .replace('/', ".")
                            .replace('$', ".");
                        if let Some(simple_name) = entry.simple_name.clone() {
                            names.put(simple_name, inner_name);
                        }
                    }
                }
            }

            loop {
                current_class = match queue.pop_front() {
                    None => None,
                    Some(name) => ctx.get_class(&name),
                };

                if let Some(next) = current_class {
                    if processed_classes.contains(&next) {
                        current_class = None;
                    }
                }

                if !(current_class.is_none() && !queue.is_empty()) {
                    break;
                }
            }
        }
    }

    fn set_write_locked(self, write_locked: bool) {
        self.with_mut(|d| d.write_locked = write_locked);
    }

    /// Blocks new imports until the returned guard is dropped (or closed).
    pub fn lock(self) -> Lock {
        Lock::new(self)
    }
}

/// `ImportCollector.Lock`: restores the previous write lock on close or drop.
pub struct Lock {
    outer: ImportCollector,
    target: bool,
}

impl Lock {
    fn new(outer: ImportCollector) -> Lock {
        let target = outer.with(|d| d.write_locked);
        outer.set_write_locked(true);
        Lock { outer, target }
    }

    pub fn close(self) {}
}

impl Drop for Lock {
    fn drop(&mut self) {
        if self.outer.is_live() {
            self.outer.set_write_locked(self.target);
        }
    }
}
