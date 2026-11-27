//! ELF evidence for native calls into Java and for NDK Binder dispatch.

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JavaTarget {
    pub class: Option<String>,
    pub name: String,
    pub descriptor: String,
    pub lookup: u64,
}

impl JavaTarget {
    pub fn key(&self) -> Option<String> {
        let class = self.class.as_deref()?;
        let class = if class.starts_with('[') {
            class.to_owned()
        } else {
            format!("L{class};")
        };
        Some(format!("{class}->{}{}", self.name, self.descriptor))
    }

    pub fn display(&self) -> String {
        let class = self.class.as_deref().unwrap_or("?").replace('/', ".");
        format!("{class}.{}{}", self.name, self.descriptor)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct JavaCall {
    pub owner: u64,
    pub site: u64,
    pub dispatch: String,
    pub form: String,
    pub targets: Vec<JavaTarget>,
    pub incomplete: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinderTransaction {
    pub code: u32,
    pub name: Option<String>,
    pub site: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BinderService {
    pub descriptor: String,
    pub define_site: u64,
    pub create: u64,
    pub destroy: u64,
    pub transact: u64,
    pub transactions: Vec<BinderTransaction>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct AndroidCalls {
    pub java: Vec<JavaCall>,
    pub binder: Vec<BinderService>,
    #[serde(default)]
    pub java_limited: bool,
}

impl AndroidCalls {
    pub fn new(mut java: Vec<JavaCall>, mut binder: Vec<BinderService>) -> Self {
        java.sort_by_key(|call| (call.owner, call.site));
        binder.sort_by_key(|service| (service.transact, service.define_site));
        Self {
            java,
            binder,
            java_limited: false,
        }
    }

    pub fn java_in(&self, owner: u64) -> &[JavaCall] {
        let start = self.java.partition_point(|call| call.owner < owner);
        let len = self.java[start..].partition_point(|call| call.owner == owner);
        &self.java[start..start + len]
    }

    pub fn java_at(&self, owner: u64, site: u64) -> Option<&JavaCall> {
        let index = self
            .java
            .binary_search_by_key(&(owner, site), |call| (call.owner, call.site))
            .ok()?;
        self.java.get(index)
    }
}
