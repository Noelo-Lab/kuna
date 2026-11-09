//! Port of `org/jetbrains/java/decompiler/struct/attr/StructModuleAttribute.java`.
//!
//! `asDescriptor()` is not ported: it builds a `java.lang.module.ModuleDescriptor`,
//! and its one caller (`JrtFinder`) is replaced by the `jdk_runtime` loader.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructModuleAttributeData {
    pub module_name: String,
    pub module_flags: i32,
    pub module_version: Option<String>,
    pub requires: Vec<RequiresEntry>,
    pub exports: Vec<ExportsEntry>,
    pub opens: Vec<OpensEntry>,
    pub uses: Vec<String>,
    pub provides: Vec<ProvidesEntry>,
}

attribute_view!(
    StructModuleAttribute,
    Module,
    StructModuleAttributeData,
    is_module,
    as_module,
    try_as_module
);

impl StructModuleAttribute {
    pub fn new() -> StructModuleAttribute {
        StructModuleAttribute::alloc_view(StructModuleAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let module_name_index = data.read_unsigned_short();
        let module_name = pool
            .get_primitive_constant(module_name_index)
            .expect("constant")
            .get_string();
        let module_flags = data.read_unsigned_short();
        self.d_mut(|d| {
            d.module_name = module_name;
            d.module_flags = module_flags;
        });

        let module_version_index = data.read_unsigned_short();
        if module_version_index != 0 {
            let module_version = pool
                .get_primitive_constant(module_version_index)
                .expect("constant")
                .get_string();
            self.d_mut(|d| d.module_version = Some(module_version));
        }

        let requires = self.read_requires(data, pool);
        self.d_mut(|d| d.requires = requires);
        let exports = StructModuleAttribute::read_exports(data, pool);
        self.d_mut(|d| d.exports = exports);
        let opens = StructModuleAttribute::read_opens(data, pool);
        self.d_mut(|d| d.opens = opens);
        let uses = StructModuleAttribute::read_uses(data, pool);
        self.d_mut(|d| d.uses = uses);
        let provides = StructModuleAttribute::read_provides(data, pool);
        self.d_mut(|d| d.provides = provides);
    }

    pub fn read_requires(self, data: &mut ByteCursor, pool: ConstantPool) -> Vec<RequiresEntry> {
        let requires_count = data.read_unsigned_short();
        if requires_count <= 0 {
            return Vec::new();
        }

        let mut requires = Vec::with_capacity(requires_count as usize);
        for _ in 0..requires_count {
            let module_name_index = data.read_unsigned_short();
            let requires_flags = data.read_unsigned_short();
            let version_index = data.read_unsigned_short();
            let module_name = pool
                .get_primitive_constant(module_name_index)
                .expect("constant")
                .get_string();
            let version = if version_index == 0 {
                None
            } else {
                Some(
                    pool.get_primitive_constant(version_index)
                        .expect("constant")
                        .get_string(),
                )
            };
            requires.push(RequiresEntry::new(module_name, requires_flags, version));
        }
        requires
    }

    fn read_exports(data: &mut ByteCursor, pool: ConstantPool) -> Vec<ExportsEntry> {
        let exports_count = data.read_unsigned_short();
        if exports_count <= 0 {
            return Vec::new();
        }

        let mut exports = Vec::with_capacity(exports_count as usize);
        for _ in 0..exports_count {
            let package_name_index = data.read_unsigned_short();
            let exports_flags = data.read_unsigned_short();
            let exports_to_modules = StructModuleAttribute::read_string_list(data, pool);
            let package_name = pool
                .get_primitive_constant(package_name_index)
                .expect("constant")
                .get_string();
            exports.push(ExportsEntry::new(
                package_name,
                exports_flags,
                exports_to_modules,
            ));
        }
        exports
    }

    fn read_opens(data: &mut ByteCursor, pool: ConstantPool) -> Vec<OpensEntry> {
        let opens_count = data.read_unsigned_short();
        if opens_count <= 0 {
            return Vec::new();
        }

        let mut opens = Vec::with_capacity(opens_count as usize);
        for _ in 0..opens_count {
            let package_name_index = data.read_unsigned_short();
            let opens_flags = data.read_unsigned_short();
            let opens_to_modules = StructModuleAttribute::read_string_list(data, pool);
            let package_name = pool
                .get_primitive_constant(package_name_index)
                .expect("constant")
                .get_string();
            opens.push(OpensEntry::new(package_name, opens_flags, opens_to_modules));
        }
        opens
    }

    fn read_uses(data: &mut ByteCursor, pool: ConstantPool) -> Vec<String> {
        StructModuleAttribute::read_string_list(data, pool)
    }

    fn read_provides(data: &mut ByteCursor, pool: ConstantPool) -> Vec<ProvidesEntry> {
        let provides_count = data.read_unsigned_short();
        if provides_count <= 0 {
            return Vec::new();
        }

        let mut provides = Vec::with_capacity(provides_count as usize);
        for _ in 0..provides_count {
            let interface_name_index = data.read_unsigned_short();
            let interface_name = pool
                .get_primitive_constant(interface_name_index)
                .expect("constant")
                .get_string();
            let implementation_names = StructModuleAttribute::read_string_list(data, pool);
            provides.push(ProvidesEntry::new(interface_name, implementation_names));
        }
        provides
    }

    fn read_string_list(data: &mut ByteCursor, pool: ConstantPool) -> Vec<String> {
        let count = data.read_unsigned_short();
        if count <= 0 {
            Vec::new()
        } else {
            let mut strings = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let index = data.read_unsigned_short();
                strings.push(
                    pool.get_primitive_constant(index)
                        .expect("constant")
                        .get_string(),
                );
            }
            strings
        }
    }

    /// The public field `moduleName`.
    pub fn module_name(self) -> String {
        self.d(|d| d.module_name.clone())
    }

    /// The public field `moduleFlags`.
    pub fn module_flags(self) -> i32 {
        self.d(|d| d.module_flags)
    }

    /// The public field `moduleVersion`.
    pub fn module_version(self) -> Option<String> {
        self.d(|d| d.module_version.clone())
    }

    /// The public field `requires`.
    pub fn requires(self) -> Vec<RequiresEntry> {
        self.d(|d| d.requires.clone())
    }

    /// The public field `exports`.
    pub fn exports(self) -> Vec<ExportsEntry> {
        self.d(|d| d.exports.clone())
    }

    /// The public field `opens`.
    pub fn opens(self) -> Vec<OpensEntry> {
        self.d(|d| d.opens.clone())
    }

    /// The public field `uses`.
    pub fn uses(self) -> Vec<String> {
        self.d(|d| d.uses.clone())
    }

    /// The public field `provides`.
    pub fn provides(self) -> Vec<ProvidesEntry> {
        self.d(|d| d.provides.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequiresEntry {
    pub module_name: String,
    pub flags: i32,
    pub module_version: Option<String>,
}

impl RequiresEntry {
    pub fn new(module_name: String, flags: i32, module_version: Option<String>) -> RequiresEntry {
        RequiresEntry {
            module_name,
            flags,
            module_version,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportsEntry {
    pub package_name: String,
    pub flags: i32,
    pub export_to_modules: Vec<String>,
}

impl ExportsEntry {
    pub fn new(package_name: String, flags: i32, export_to_modules: Vec<String>) -> ExportsEntry {
        ExportsEntry {
            package_name,
            flags,
            export_to_modules,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpensEntry {
    pub package_name: String,
    pub flags: i32,
    pub opens_to_modules: Vec<String>,
}

impl OpensEntry {
    pub fn new(package_name: String, flags: i32, export_to_modules: Vec<String>) -> OpensEntry {
        OpensEntry {
            package_name,
            flags,
            opens_to_modules: export_to_modules,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvidesEntry {
    pub interface_name: String,
    pub implementation_names: Vec<String>,
}

impl ProvidesEntry {
    pub fn new(interface_name: String, implementation_names: Vec<String>) -> ProvidesEntry {
        ProvidesEntry {
            interface_name,
            implementation_names,
        }
    }
}
