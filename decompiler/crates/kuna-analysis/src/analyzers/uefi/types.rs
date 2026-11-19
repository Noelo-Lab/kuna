//! EFI table layouts (UEFI 2.10 / PI 1.8), built into kuna `Datatype`s.
//!
//! Each field is spelled in a small C-like vocabulary: a scalar (`UINTN`,
//! `UINT32`, `CHAR16`, ...), an EFI typedef (`EFI_STATUS`, `EFI_HANDLE`, ...), a
//! struct built earlier in [`TABLES`], `VOID`, any of them followed by `*`s, an
//! inline array `T[n]`, a function pointer `fn RET(ARG, ...)`, or `FUNC` for a
//! function pointer of unknown prototype. Layout follows natural alignment, which
//! is what the UEFI ABI uses on every supported CPU.
//!
//! [`TABLES`] is built for every EFI image. A protocol or PPI interface of
//! [`super::protocols`], or a struct the pass synthesized ([`EfiTypes::define`]),
//! is built the first time something names it.

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use kuna_decomp::fspec::PrototypePieces;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

pub(super) const TABLE_HEADER: &[(&str, &str)] = &[
    ("Signature", "UINT64"),
    ("Revision", "UINT32"),
    ("HeaderSize", "UINT32"),
    ("CRC32", "UINT32"),
    ("Reserved", "UINT32"),
];

pub(super) const TABLES: &[(&str, &[(&str, &str)])] = &[
    ("EFI_GUID", &[("Data1", "UINT32"), ("Data2", "UINT16"), ("Data3", "UINT16"), ("Data4", "UINT8[8]")]),
    ("EFI_TABLE_HEADER", TABLE_HEADER),
    ("EFI_CONFIGURATION_TABLE", &[("VendorGuid", "EFI_GUID"), ("VendorTable", "VOID*")]),
    ("EFI_SIMPLE_TEXT_INPUT_PROTOCOL", &[
        ("Reset", "fn EFI_STATUS(VOID*, BOOLEAN)"),
        ("ReadKeyStroke", "fn EFI_STATUS(VOID*, VOID*)"),
        ("WaitForKey", "EFI_EVENT"),
    ]),
    ("EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL", &[
        ("Reset", "fn EFI_STATUS(VOID*, BOOLEAN)"),
        ("OutputString", "fn EFI_STATUS(VOID*, CHAR16*)"),
        ("TestString", "fn EFI_STATUS(VOID*, CHAR16*)"),
        ("QueryMode", "fn EFI_STATUS(VOID*, UINTN, UINTN*, UINTN*)"),
        ("SetMode", "fn EFI_STATUS(VOID*, UINTN)"),
        ("SetAttribute", "fn EFI_STATUS(VOID*, UINTN)"),
        ("ClearScreen", "fn EFI_STATUS(VOID*)"),
        ("SetCursorPosition", "fn EFI_STATUS(VOID*, UINTN, UINTN)"),
        ("EnableCursor", "fn EFI_STATUS(VOID*, BOOLEAN)"),
        ("Mode", "VOID*"),
    ]),
    ("EFI_BOOT_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("RaiseTPL", "fn EFI_TPL(EFI_TPL)"),
        ("RestoreTPL", "fn VOID(EFI_TPL)"),
        ("AllocatePages", "fn EFI_STATUS(UINT32, UINT32, UINTN, EFI_PHYSICAL_ADDRESS*)"),
        ("FreePages", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINTN)"),
        ("GetMemoryMap", "fn EFI_STATUS(UINTN*, VOID*, UINTN*, UINTN*, UINT32*)"),
        ("AllocatePool", "fn EFI_STATUS(UINT32, UINTN, VOID**)"),
        ("FreePool", "fn EFI_STATUS(VOID*)"),
        ("CreateEvent", "fn EFI_STATUS(UINT32, EFI_TPL, VOID*, VOID*, EFI_EVENT*)"),
        ("SetTimer", "fn EFI_STATUS(EFI_EVENT, UINT32, UINT64)"),
        ("WaitForEvent", "fn EFI_STATUS(UINTN, EFI_EVENT*, UINTN*)"),
        ("SignalEvent", "fn EFI_STATUS(EFI_EVENT)"),
        ("CloseEvent", "fn EFI_STATUS(EFI_EVENT)"),
        ("CheckEvent", "fn EFI_STATUS(EFI_EVENT)"),
        ("InstallProtocolInterface", "fn EFI_STATUS(EFI_HANDLE*, EFI_GUID*, UINT32, VOID*)"),
        ("ReinstallProtocolInterface", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID*, VOID*)"),
        ("UninstallProtocolInterface", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID*)"),
        ("HandleProtocol", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID**)"),
        ("Reserved", "VOID*"),
        ("RegisterProtocolNotify", "fn EFI_STATUS(EFI_GUID*, EFI_EVENT, VOID**)"),
        ("LocateHandle", "fn EFI_STATUS(UINT32, EFI_GUID*, VOID*, UINTN*, EFI_HANDLE*)"),
        ("LocateDevicePath", "fn EFI_STATUS(EFI_GUID*, VOID**, EFI_HANDLE*)"),
        ("InstallConfigurationTable", "fn EFI_STATUS(EFI_GUID*, VOID*)"),
        ("LoadImage", "fn EFI_STATUS(BOOLEAN, EFI_HANDLE, VOID*, VOID*, UINTN, EFI_HANDLE*)"),
        ("StartImage", "fn EFI_STATUS(EFI_HANDLE, UINTN*, CHAR16**)"),
        ("Exit", "fn EFI_STATUS(EFI_HANDLE, EFI_STATUS, UINTN, CHAR16*)"),
        ("UnloadImage", "fn EFI_STATUS(EFI_HANDLE)"),
        ("ExitBootServices", "fn EFI_STATUS(EFI_HANDLE, UINTN)"),
        ("GetNextMonotonicCount", "fn EFI_STATUS(UINT64*)"),
        ("Stall", "fn EFI_STATUS(UINTN)"),
        ("SetWatchdogTimer", "fn EFI_STATUS(UINTN, UINT64, UINTN, CHAR16*)"),
        ("ConnectController", "fn EFI_STATUS(EFI_HANDLE, EFI_HANDLE*, VOID*, BOOLEAN)"),
        ("DisconnectController", "fn EFI_STATUS(EFI_HANDLE, EFI_HANDLE, EFI_HANDLE)"),
        ("OpenProtocol", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID**, EFI_HANDLE, EFI_HANDLE, UINT32)"),
        ("CloseProtocol", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, EFI_HANDLE, EFI_HANDLE)"),
        ("OpenProtocolInformation", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID**, UINTN*)"),
        ("ProtocolsPerHandle", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID***, UINTN*)"),
        ("LocateHandleBuffer", "fn EFI_STATUS(UINT32, EFI_GUID*, VOID*, UINTN*, EFI_HANDLE**)"),
        ("LocateProtocol", "fn EFI_STATUS(EFI_GUID*, VOID*, VOID**)"),
        ("InstallMultipleProtocolInterfaces", "fn EFI_STATUS(EFI_HANDLE*, ...)"),
        ("UninstallMultipleProtocolInterfaces", "fn EFI_STATUS(EFI_HANDLE, ...)"),
        ("CalculateCrc32", "fn EFI_STATUS(VOID*, UINTN, UINT32*)"),
        ("CopyMem", "fn VOID(VOID*, VOID*, UINTN)"),
        ("SetMem", "fn VOID(VOID*, UINTN, UINT8)"),
        ("CreateEventEx", "fn EFI_STATUS(UINT32, EFI_TPL, VOID*, VOID*, EFI_GUID*, EFI_EVENT*)"),
    ]),
    ("EFI_RUNTIME_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("GetTime", "fn EFI_STATUS(VOID*, VOID*)"),
        ("SetTime", "fn EFI_STATUS(VOID*)"),
        ("GetWakeupTime", "fn EFI_STATUS(BOOLEAN*, BOOLEAN*, VOID*)"),
        ("SetWakeupTime", "fn EFI_STATUS(BOOLEAN, VOID*)"),
        ("SetVirtualAddressMap", "fn EFI_STATUS(UINTN, UINTN, UINT32, VOID*)"),
        ("ConvertPointer", "fn EFI_STATUS(UINTN, VOID**)"),
        ("GetVariable", "fn EFI_STATUS(CHAR16*, EFI_GUID*, UINT32*, UINTN*, VOID*)"),
        ("GetNextVariableName", "fn EFI_STATUS(UINTN*, CHAR16*, EFI_GUID*)"),
        ("SetVariable", "fn EFI_STATUS(CHAR16*, EFI_GUID*, UINT32, UINTN, VOID*)"),
        ("GetNextHighMonotonicCount", "fn EFI_STATUS(UINT32*)"),
        ("ResetSystem", "fn VOID(UINT32, EFI_STATUS, UINTN, VOID*)"),
        ("UpdateCapsule", "fn EFI_STATUS(VOID**, UINTN, EFI_PHYSICAL_ADDRESS)"),
        ("QueryCapsuleCapabilities", "fn EFI_STATUS(VOID**, UINTN, UINT64*, UINT32*)"),
        ("QueryVariableInfo", "fn EFI_STATUS(UINT32, UINT64*, UINT64*, UINT64*)"),
    ]),
    ("EFI_SYSTEM_TABLE", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("FirmwareVendor", "CHAR16*"),
        ("FirmwareRevision", "UINT32"),
        ("ConsoleInHandle", "EFI_HANDLE"),
        ("ConIn", "EFI_SIMPLE_TEXT_INPUT_PROTOCOL*"),
        ("ConsoleOutHandle", "EFI_HANDLE"),
        ("ConOut", "EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL*"),
        ("StandardErrorHandle", "EFI_HANDLE"),
        ("StdErr", "EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL*"),
        ("RuntimeServices", "EFI_RUNTIME_SERVICES*"),
        ("BootServices", "EFI_BOOT_SERVICES*"),
        ("NumberOfTableEntries", "UINTN"),
        ("ConfigurationTable", "EFI_CONFIGURATION_TABLE*"),
    ]),
    ("EFI_DXE_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("AddMemorySpace", "fn EFI_STATUS(UINT32, EFI_PHYSICAL_ADDRESS, UINT64, UINT64)"),
        ("AllocateMemorySpace", "fn EFI_STATUS(UINT32, UINT32, UINTN, UINT64, EFI_PHYSICAL_ADDRESS*, EFI_HANDLE, EFI_HANDLE)"),
        ("FreeMemorySpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("RemoveMemorySpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("GetMemorySpaceDescriptor", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, VOID*)"),
        ("SetMemorySpaceAttributes", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64, UINT64)"),
        ("GetMemorySpaceMap", "fn EFI_STATUS(UINTN*, VOID**)"),
        ("AddIoSpace", "fn EFI_STATUS(UINT32, EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("AllocateIoSpace", "fn EFI_STATUS(UINT32, UINT32, UINTN, UINT64, EFI_PHYSICAL_ADDRESS*, EFI_HANDLE, EFI_HANDLE)"),
        ("FreeIoSpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("RemoveIoSpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("GetIoSpaceDescriptor", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, VOID*)"),
        ("GetIoSpaceMap", "fn EFI_STATUS(UINTN*, VOID**)"),
        ("Dispatch", "fn EFI_STATUS()"),
        ("Schedule", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*)"),
        ("Trust", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*)"),
        ("ProcessFirmwareVolume", "fn EFI_STATUS(VOID*, UINTN, EFI_HANDLE*)"),
        ("SetMemorySpaceCapabilities", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64, UINT64)"),
    ]),
    ("EFI_PEI_PPI_DESCRIPTOR", &[("Flags", "UINTN"), ("Guid", "EFI_GUID*"), ("Ppi", "VOID*")]),
    ("EFI_PEI_NOTIFY_DESCRIPTOR", &[
        ("Flags", "UINTN"),
        ("Guid", "EFI_GUID*"),
        ("Notify", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PEI_NOTIFY_DESCRIPTOR* NotifyDescriptor, VOID* Ppi)"),
    ]),
    ("EFI_PEI_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("InstallPpi", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PEI_PPI_DESCRIPTOR* PpiList)"),
        ("ReInstallPpi", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PEI_PPI_DESCRIPTOR* OldPpi, EFI_PEI_PPI_DESCRIPTOR* NewPpi)"),
        ("LocatePpi", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_GUID* Guid, UINTN Instance, EFI_PEI_PPI_DESCRIPTOR** PpiDescriptor, VOID** Ppi)"),
        ("NotifyPpi", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PEI_NOTIFY_DESCRIPTOR* NotifyList)"),
        ("GetBootMode", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT32* BootMode)"),
        ("SetBootMode", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT32 BootMode)"),
        ("GetHobList", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, VOID** HobList)"),
        ("CreateHob", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT16 Type, UINT16 Length, VOID** Hob)"),
        ("FfsFindNextVolume", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINTN Instance, VOID** VolumeHandle)"),
        ("FfsFindNextFile", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT8 SearchType, VOID* FvHandle, VOID** FileHandle)"),
        ("FfsFindSectionData", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT8 SectionType, VOID* FileHandle, VOID** SectionData)"),
        ("InstallPeiMemory", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PHYSICAL_ADDRESS MemoryBegin, UINT64 MemoryLength)"),
        ("AllocatePages", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT32 MemoryType, UINTN Pages, EFI_PHYSICAL_ADDRESS* Memory)"),
        ("AllocatePool", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINTN Size, VOID** Buffer)"),
        ("CopyMem", "fn VOID(VOID*, VOID*, UINTN)"),
        ("SetMem", "fn VOID(VOID*, UINTN, UINT8)"),
        ("ReportStatusCode", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT32 Type, UINT32 Value, UINT32 Instance, EFI_GUID* CallerId, VOID* Data)"),
        ("ResetSystem", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices)"),
        ("CpuIo", "VOID*"),
        ("PciCfg", "VOID*"),
        ("FfsFindFileByName", "fn EFI_STATUS(EFI_GUID* FileName, VOID* VolumeHandle, VOID** FileHandle)"),
        ("FfsGetFileInfo", "fn EFI_STATUS(VOID* FileHandle, VOID* FileInfo)"),
        ("FfsGetVolumeInfo", "fn EFI_STATUS(VOID* VolumeHandle, VOID* VolumeInfo)"),
        ("RegisterForShadow", "fn EFI_STATUS(VOID* FileHandle)"),
        ("FindSectionData3", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, UINT8 SectionType, UINTN SectionInstance, VOID* FileHandle, VOID** SectionData, UINT32* AuthenticationStatus)"),
        ("FfsGetFileInfo2", "fn EFI_STATUS(VOID* FileHandle, VOID* FileInfo)"),
        ("ResetSystem2", "fn VOID(UINT32 ResetType, EFI_STATUS ResetStatus, UINTN DataSize, VOID* ResetData)"),
        ("FreePages", "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PHYSICAL_ADDRESS Memory, UINTN Pages)"),
    ]),
];

/// One member of a built struct: where it is and how it was spelled.
#[derive(Clone, Debug)]
pub(super) struct Member {
    pub(super) offset: i64,
    pub(super) name: String,
    pub(super) spec: String,
}

/// Builds and caches the EFI types for one image.
pub(super) struct EfiTypes<'a> {
    pub(super) types: &'a dyn TypeFactory,
    pub(super) ptr: i32,
    built: RefCell<BTreeMap<String, Rc<Datatype>>>,
    open: RefCell<BTreeMap<String, Rc<Datatype>>>,
    defined: RefCell<BTreeMap<String, Vec<(String, String)>>>,
    layouts: RefCell<BTreeMap<String, Option<Rc<Vec<Member>>>>>,
}

impl<'a> EfiTypes<'a> {
    /// All of [`TABLES`] under `ptr`-byte pointers, or `None` when a name is
    /// already held by a type of another shape.
    pub(super) fn build(types: &'a dyn TypeFactory, ptr: i32) -> Option<Self> {
        let me = EfiTypes {
            types,
            ptr,
            built: RefCell::default(),
            open: RefCell::default(),
            defined: RefCell::default(),
            layouts: RefCell::default(),
        };
        for (name, _) in TABLES {
            me.get(name)?;
        }
        Some(me)
    }

    /// The members of `name` as written: a table of this file, a generated
    /// interface, or a struct given to [`Self::define`].
    pub(super) fn fields_of(&self, name: &str) -> Option<Vec<(String, String)>> {
        let own = |fields: &[(&str, &str)]| fields.iter().map(|(n, s)| (n.to_string(), s.to_string())).collect();
        if let Some((_, fields)) = TABLES.iter().find(|(n, _)| *n == name) {
            return Some(own(fields));
        }
        if let Some(fields) = super::protocols::table().structs.get(name) {
            return Some(own(fields));
        }
        self.defined.borrow().get(name).cloned()
    }

    /// The members of a struct given to [`Self::define`].
    pub(super) fn defined_fields(&self, name: &str) -> Option<Vec<(String, String)>> {
        self.defined.borrow().get(name).cloned()
    }

    /// Give `name` a layout the tables do not have. A name the tables hold, or
    /// one defined before, keeps its first layout.
    pub(super) fn define(&self, name: &str, fields: Vec<(String, String)>) {
        if self.fields_of(name).is_none() {
            self.defined.borrow_mut().insert(name.to_string(), fields);
        }
    }

    pub(super) fn get(&self, name: &str) -> Option<Rc<Datatype>> {
        if let Some(t) = self.built.borrow().get(name) {
            return Some(t.clone());
        }
        if let Some(t) = self.open.borrow().get(name) {
            return Some(t.clone());
        }
        let fields = self.fields_of(name)?;
        let st = self.build_struct(name, &fields)?;
        self.built.borrow_mut().insert(name.to_string(), st.clone());
        Some(st)
    }

    /// The members of `name` with their offsets, a struct held by value
    /// replaced by its own members.
    pub(super) fn layout(&self, name: &str) -> Option<Rc<Vec<Member>>> {
        if let Some(known) = self.layouts.borrow().get(name) {
            return known.clone();
        }
        let flat = self.flatten(name, 0).map(Rc::new);
        self.layouts.borrow_mut().insert(name.to_string(), flat.clone());
        flat
    }

    fn flatten(&self, name: &str, depth: u32) -> Option<Vec<Member>> {
        self.get(name)?;
        let specs = self.fields_of(name)?;
        let (fields, ..) = self.lay_out(&specs)?;
        if depth > 4 {
            return None;
        }
        let mut out = Vec::new();
        for (field, (fname, spec)) in fields.iter().zip(specs) {
            let offset = i64::from(field.offset);
            let nested = (!spec.ends_with('*') && self.fields_of(&spec).is_some())
                .then(|| self.flatten(&spec, depth + 1))
                .flatten();
            match nested {
                Some(inner) => out.extend(inner.into_iter().map(|m| Member {
                    offset: m.offset + offset,
                    name: format!("{fname}.{}", m.name),
                    ..m
                })),
                None => out.push(Member { offset, name: fname, spec }),
            }
        }
        Some(out)
    }

    fn build_struct(&self, name: &str, fields: &[(String, String)]) -> Option<Rc<Datatype>> {
        let shell = self.types.kuna_declare_record(name, false).ok()?;
        if shell.get_metatype() != type_metatype::TYPE_STRUCT {
            return None;
        }
        self.open.borrow_mut().insert(name.to_string(), shell.clone());
        let laid = self.lay_out(fields);
        self.open.borrow_mut().remove(name);
        let (out, size, align) = laid?;
        if !shell.is_incomplete() {
            return (shell.get_size() == size).then_some(shell);
        }
        self.types.set_fields_struct_raw(&shell, out, Vec::new(), size, align, 0).ok()
    }

    fn lay_out(&self, fields: &[(String, String)]) -> Option<(Vec<TypeField>, i32, i32)> {
        let mut out = Vec::with_capacity(fields.len());
        let mut off = 0i32;
        let mut align = 1i32;
        for (i, (fname, spec)) in fields.iter().enumerate() {
            let ty = self.parse(spec)?;
            if ty.is_incomplete() || ty.get_size() <= 0 {
                return None;
            }
            let a = natural_align(&ty, self.ptr);
            off = off.checked_add(a - 1)? / a * a;
            align = align.max(a);
            out.push(TypeField::new(i as i32, off, fname.as_str(), ty.clone()));
            off = off.checked_add(ty.get_size())?;
        }
        let size = off.checked_add(align - 1)? / align * align;
        Some((out, size, align))
    }

    /// One type in the field vocabulary.
    pub(super) fn parse(&self, spec: &str) -> Option<Rc<Datatype>> {
        let spec = spec.trim();
        if spec == "FUNC" {
            return self.types.get_type_pointer(self.ptr, self.types.get_type_code().ok()?, 1).ok();
        }
        if let Some(sig) = spec.strip_prefix("fn ") {
            let pieces = self.pieces("", sig)?;
            let code = self.types.get_type_code_proto(&pieces).or_else(|_| self.types.get_type_code()).ok()?;
            return self.types.get_type_pointer(self.ptr, code, 1).ok();
        }
        if let Some(open) = spec.find('[') {
            let n: i32 = spec.get(open + 1..spec.len() - 1)?.parse().ok().filter(|n| (1..=4096).contains(n))?;
            return self.types.get_type_array(n, self.parse(&spec[..open])?).ok();
        }
        if let Some(inner) = spec.strip_suffix('*') {
            let pointee = self.parse(inner)?;
            return self.types.get_type_pointer(self.ptr, pointee, 1).ok();
        }
        let t = self.types;
        let int = |n: i32| t.get_base(n, type_metatype::TYPE_UINT).ok();
        let sint = |n: i32| t.get_base(n, type_metatype::TYPE_INT).ok();
        let typedef = |base: Option<Rc<Datatype>>, name: &str| {
            let base = base?;
            t.get_typedef(&base, name, 0, 0).ok().or(Some(base))
        };
        match spec {
            "VOID" => t.get_type_void().ok(),
            "UINT8" => int(1),
            "UINT16" => int(2),
            "UINT32" => int(4),
            "UINT64" => int(8),
            "UINTN" => int(self.ptr),
            "INT8" => sint(1),
            "INT16" => sint(2),
            "INT32" => sint(4),
            "INT64" => sint(8),
            "INTN" => sint(self.ptr),
            "BOOLEAN" => t.get_base(1, type_metatype::TYPE_BOOL).ok(),
            "CHAR16" => t.get_type_char(2).or_else(|_| t.get_base(2, type_metatype::TYPE_UINT)).ok(),
            "CHAR8" => t.get_type_char(1).or_else(|_| t.get_base(1, type_metatype::TYPE_INT)).ok(),
            "EFI_STATUS" => typedef(int(self.ptr), "EFI_STATUS"),
            "EFI_TPL" => typedef(int(self.ptr), "EFI_TPL"),
            "EFI_PHYSICAL_ADDRESS" => typedef(int(8), "EFI_PHYSICAL_ADDRESS"),
            "EFI_HANDLE" | "EFI_EVENT" => {
                let void = t.get_type_pointer(self.ptr, t.get_type_void().ok()?, 1).ok();
                typedef(void, spec)
            }
            other => self.get(other),
        }
    }

    /// `RET(ARG, ...)` as prototype pieces named `name`.
    pub(super) fn pieces(&self, name: &str, sig: &str) -> Option<PrototypePieces> {
        let open = sig.find('(')?;
        let args = sig[open + 1..].strip_suffix(')')?;
        let mut pieces = PrototypePieces { name: name.to_string(), first_var_arg_slot: -1, ..Default::default() };
        pieces.outtype = Some(self.parse(&sig[..open])?);
        for arg in args.split(',').map(str::trim).filter(|a| !a.is_empty()) {
            if arg == "..." {
                pieces.first_var_arg_slot = pieces.intypes.len() as i32;
                continue;
            }
            let (ty, nm) = match arg.rsplit_once(' ') {
                Some((ty, nm)) if !nm.ends_with('*') => (ty, nm),
                _ => (arg, ""),
            };
            pieces.intypes.push(self.parse(ty)?);
            pieces.innames.push(nm.to_string());
        }
        Some(pieces)
    }
}

fn natural_align(ty: &Datatype, ptr: i32) -> i32 {
    match ty.get_metatype() {
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_ARRAY => ty.get_alignment().max(1),
        _ => ty.get_size().clamp(1, 8).min(ptr.max(8)),
    }
}
